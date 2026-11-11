//! The render-world half of the warm-up measurement ([`crate::warmup`], pipeline-warmup.md §6)
//! and of the reveal gate's GPU terms (§5.4, W1).
//!
//! [`PendingDrawsPlugin`] (the reveal gate adds it in every build) fills [`PendingDraws`] each
//! frame:
//! - **Pending draws:** the cameras' visible meshes whose specialized main-pass pipeline is
//!   queued or compiling (Bevy skips their draws), as `queue_material_meshes` looks them up;
//!   meshes with a material instance but no pipeline yet (mesh or material not prepared: cause
//!   C); render assets retried next frame (`PrepareNextFrameAssets` of meshes, images and each
//!   material type a [`PendingMaterialPlugin`] counts: read-only counts in the vendored
//!   `bevy_render`).
//! - **While a [`crate::RevealProbe`] is set** (a reveal holds): shadow-view and prepass entries
//!   of the specialized pipeline caches whose pipeline is queued or compiling, and the resident,
//!   shown meshes within the probe's radius (extracted from the main world, on camera or not)
//!   whose mesh or material is not prepared (a material prepares once its textures are uploaded,
//!   so this is the texture term too). Public render-world resources only, read after `Queue`.
//!
//! [`PipelineWatchPlugin`] (feature `pipelines`, dev builds) adds the logging:
//! - **Compile times:** `PipelineCache::take_compile_timings` (vendored `bevy_render`, marker
//!   `own: pipeline compile timing`): counters `pipelines/compile_ms`, `/front_ms`, `/driver_ms`,
//!   `/wait_ms` (per frame, ms) and a `pipelines: compiled N (...)` line per burst with the
//!   histogram ([`CompileStats`]).
//! - **View keys:** `bevy_pbr::ViewKeyCache` (public, read-only) diffed per view each frame: one
//!   `view key changed: <bits> (...)` line per flip (each flip re-specializes every material on
//!   that view).

use std::marker::PhantomData;
use std::time::Instant;

use bevy_app::{App, Plugin};
use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{
    Material, MeshMaterial3d, MeshPipelineKey, PreparedMaterial, RenderMaterialInstances, SpecializedMaterialPipelineCache,
    SpecializedPrepassMaterialPipelineCache, SpecializedShadowMaterialPipelineCache, ViewKeyCache,
};
use bevy_platform::collections::HashMap;
use bevy_render::camera::ExtractedCamera;
use bevy_render::erased_render_asset::{ErasedRenderAssets, PrepareNextFrameAssets as ErasedNextFrame};
use bevy_render::mesh::RenderMesh;
use bevy_render::render_asset::{PrepareNextFrameAssets, RenderAssets};
use bevy_render::render_resource::{CachedPipelineState, CachedRenderPipelineId, PipelineCache};
use bevy_render::sync_world::MainEntity;
use bevy_render::texture::GpuImage;
use bevy_render::view::{ExtractedView, RenderVisibleEntities, RetainedViewEntity};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_transform::components::GlobalTransform;

use crate::DiagCounters;
use crate::warmup::{CompileStats, PendingDraws, PendingSample, RevealWatchPlugin, describe_key_change};

/// A compile burst is logged once nothing is compiling, or after this long.
const BURST_SECS: f32 = 2.0;

/// Near meshes named in [`PendingDraws::examples`] (the gate's cap warning names them).
const EXAMPLES: usize = 4;

/// Fills [`PendingDraws`] in the render world (see the module docs). Added once: the reveal gate
/// and [`PipelineWatchPlugin`] both check [`App::is_plugin_added`].
#[derive(Default)]
pub struct PendingDrawsPlugin;

impl Plugin for PendingDrawsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingDraws>();
    }

    fn finish(&self, app: &mut App) {
        let pending = app.world().resource::<PendingDraws>().clone();
        let Some(render) = app.get_sub_app_mut(RenderApp) else { return };
        render
            .insert_resource(pending)
            .init_resource::<NearMeshes>()
            .init_resource::<MaterialQueues>()
            .add_systems(ExtractSchedule, extract_near_meshes)
            .add_systems(Render, count_pending.in_set(RenderSystems::Cleanup));
    }
}

/// Counts material type `M`'s retry queue into [`PendingSample::assets`] (with
/// [`PendingDrawsPlugin`]). The gate's near term covers every material type without it; this is
/// the log's global count.
pub struct PendingMaterialPlugin<M>(PhantomData<fn() -> M>);

impl<M> Default for PendingMaterialPlugin<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M: Material> Plugin for PendingMaterialPlugin<M>
where
    M::Data: PartialEq + Eq + std::hash::Hash + Clone,
{
    fn build(&self, _: &mut App) {}

    fn finish(&self, app: &mut App) {
        let Some(render) = app.get_sub_app_mut(RenderApp) else { return };
        render
            .init_resource::<MaterialQueues>()
            .add_systems(Render, count_material_queue::<M>.in_set(RenderSystems::Cleanup).before(count_pending));
    }
}

/// See the module docs. Adds [`PendingDrawsPlugin`] and [`RevealWatchPlugin`] (the main-world
/// half). Reports into [`DiagCounters`] when `SfDiagPlugin` was added first.
#[derive(Default)]
pub struct PipelineWatchPlugin;

impl Plugin for PipelineWatchPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<RevealWatchPlugin>() {
            app.add_plugins(RevealWatchPlugin);
        }
        if !app.is_plugin_added::<PendingDrawsPlugin>() {
            app.add_plugins(PendingDrawsPlugin);
        }
    }

    fn finish(&self, app: &mut App) {
        let counters = app.world().get_resource::<DiagCounters>().cloned();
        let Some(render) = app.get_sub_app_mut(RenderApp) else { return };
        if let Some(counters) = counters {
            render.insert_resource(counters);
        }
        render.add_systems(Render, (compile_times, view_keys).in_set(RenderSystems::Cleanup));
    }
}

/// Resident, shown meshes within the probe's radius this frame (main entity, mesh asset).
#[derive(Resource, Default)]
struct NearMeshes(Vec<(MainEntity, AssetId<Mesh>)>);

/// The counted material types' retry queues this frame (summed, then taken by `count_pending`).
#[derive(Resource, Default)]
struct MaterialQueues(usize);

#[allow(clippy::needless_pass_by_value)] // system parameters
fn extract_near_meshes(
    pending: Res<PendingDraws>,
    mut near: ResMut<NearMeshes>,
    meshes: Extract<Query<(Entity, &GlobalTransform, &Mesh3d, &InheritedVisibility)>>,
) {
    near.0.clear();
    let Some(probe) = pending.probe() else { return };
    let r2 = probe.radius * probe.radius;
    for (entity, at, mesh, shown) in &meshes {
        if shown.get() && at.translation().distance_squared(probe.center.into()) <= r2 {
            near.0.push((MainEntity::from(entity), mesh.id()));
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn count_material_queue<M: Material>(queue: Option<Res<ErasedNextFrame<MeshMaterial3d<M>>>>, mut queues: ResMut<MaterialQueues>)
where
    M::Data: PartialEq + Eq + std::hash::Hash + Clone,
{
    queues.0 += queue.map_or(0, |q| q.len());
}

/// What the probe terms read (only while a reveal holds).
#[derive(SystemParam)]
struct Probed<'w> {
    near: Res<'w, NearMeshes>,
    shadow: Option<Res<'w, SpecializedShadowMaterialPipelineCache>>,
    prepass: Option<Res<'w, SpecializedPrepassMaterialPipelineCache>>,
    meshes: Option<Res<'w, RenderAssets<RenderMesh>>>,
    prepared: Option<Res<'w, ErasedRenderAssets<PreparedMaterial>>>,
}

/// Fills [`PendingDraws`] (see the module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn count_pending(
    pending: Res<PendingDraws>,
    cache: Res<PipelineCache>,
    specialized: Option<Res<SpecializedMaterialPipelineCache>>,
    materials: Option<Res<RenderMaterialInstances>>,
    meshes_next: Option<Res<PrepareNextFrameAssets<RenderMesh>>>,
    images_next: Option<Res<PrepareNextFrameAssets<GpuImage>>>,
    mut material_queues: ResMut<MaterialQueues>,
    views: Query<(&ExtractedView, &RenderVisibleEntities), With<ExtractedCamera>>,
    probed: Probed,
) {
    let compiling = |id: CachedRenderPipelineId| {
        matches!(cache.get_render_pipeline_state(id), CachedPipelineState::Queued | CachedPipelineState::Creating(_))
    };
    let mut s = PendingSample::default();
    if let Some(specialized) = specialized {
        for (view, visible) in &views {
            let Some(meshes) = visible.get::<Mesh3d>() else { continue };
            let pipelines = specialized.get(&view.retained_view_entity);
            let main = meshes.entities_cpu_culling.iter().map(|(_, m)| m).chain(meshes.entities_gpu_culling.keys());
            for m in main {
                match pipelines.and_then(|p| p.get(m)) {
                    Some(id) => {
                        s.visible += 1;
                        if compiling(*id) {
                            s.skipped += 1;
                        }
                    }
                    None if materials.as_ref().is_some_and(|mi| mi.instances.contains_key(m)) => s.unprepared += 1,
                    None => {}
                }
            }
        }
    }
    s.assets =
        meshes_next.map_or(0, |q| q.len()) + images_next.map_or(0, |q| q.len()) + std::mem::take(&mut material_queues.0);
    s.waiting = cache.waiting_pipelines().count();
    s.created = cache.pipelines().count();
    let mut examples = Vec::new();
    if pending.probe().is_some() {
        let shadow =
            probed.shadow.as_ref().map_or(0, |c| c.values().flat_map(|v| v.values()).filter(|(id, _, _)| compiling(*id)).count());
        let prepass =
            probed.prepass.as_ref().map_or(0, |c| c.values().flat_map(|v| v.values()).filter(|(_, id, _)| compiling(*id)).count());
        s.skipped_shadow = shadow + prepass;
        if let Some(materials) = &materials {
            for (main, mesh) in &probed.near.0 {
                // A mesh with no material instance is not drawn by a material: not ours to wait for.
                let Some(material) = materials.instances.get(main) else { continue };
                let mesh_ready = probed.meshes.as_ref().is_none_or(|m| m.get(*mesh).is_some());
                let material_ready = probed.prepared.as_ref().is_none_or(|p| p.get(material.asset_id).is_some());
                if !(mesh_ready && material_ready) {
                    s.near_unprepared += 1;
                    if examples.len() < EXAMPLES {
                        examples.push(main.id());
                    }
                }
            }
        }
    }
    pending.store(s);
    pending.set_examples(examples);
}

#[derive(Default)]
struct Bursts {
    started: bool,
    burst: CompileStats,
    since: Option<Instant>,
    session: CompileStats,
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn compile_times(cache: Res<PipelineCache>, counters: Option<Res<DiagCounters>>, mut b: Local<Bursts>) {
    if !b.started {
        cache.record_compile_timings();
        b.started = true;
    }
    let timings = cache.take_compile_timings();
    let (mut wait, mut front, mut driver) = (0.0, 0.0, 0.0);
    for t in &timings {
        let ms = |d: std::time::Duration| d.as_secs_f64() * 1000.0;
        let (w, f, d) = (ms(t.queued + t.lock_wait), ms(t.front), ms(t.driver));
        let label = t.label.as_deref().unwrap_or("unlabelled");
        b.burst.record(label, w, f, d);
        b.session.record(label, w, f, d);
        (wait, front, driver) = (wait + w, front + f, driver + d);
    }
    if !timings.is_empty() {
        b.since.get_or_insert_with(Instant::now);
        if let Some(c) = &counters {
            let ms = sf_diag::unit::MS;
            c.add_named("pipelines/compile_ms", ms, front + driver);
            c.add_named("pipelines/front_ms", ms, front);
            c.add_named("pipelines/driver_ms", ms, driver);
            c.add_named("pipelines/wait_ms", ms, wait);
        }
    }
    let due = b.since.is_some_and(|t| t.elapsed().as_secs_f32() >= BURST_SECS);
    if b.burst.count > 0 && (due || cache.waiting_pipelines().next().is_none()) {
        let session = &b.session;
        log::info!(
            "pipelines: {}; session {} (front {:.2} s, driver {:.2} s)",
            b.burst.line(),
            session.count,
            session.front_ms / 1000.0,
            session.driver_ms / 1000.0,
        );
        b.burst = CompileStats::default();
        b.since = None;
    }
}

/// Single-bit flag names and multi-bit fields of [`MeshPipelineKey`].
/// Bit names: (name, mask) pairs.
type Names = Vec<(&'static str, u64)>;

fn key_names() -> (Names, Names) {
    let fields = vec![
        ("MSAA", MeshPipelineKey::MSAA_RESERVED_BITS.bits()),
        ("BLEND", MeshPipelineKey::BLEND_RESERVED_BITS.bits()),
        ("TONEMAP_METHOD", MeshPipelineKey::TONEMAP_METHOD_RESERVED_BITS.bits()),
        ("SHADOW_FILTER_METHOD", MeshPipelineKey::SHADOW_FILTER_METHOD_RESERVED_BITS.bits()),
        ("VIEW_PROJECTION", MeshPipelineKey::VIEW_PROJECTION_RESERVED_BITS.bits()),
        ("SSS_TRANSMISSION", MeshPipelineKey::SCREEN_SPACE_SPECULAR_TRANSMISSION_RESERVED_BITS.bits()),
        ("COLOR_TARGET_FORMAT", MeshPipelineKey::COLOR_TARGET_FORMAT_RESERVED_BITS.bits()),
    ];
    let reserved = MeshPipelineKey::ALL_RESERVED_BITS.bits();
    let flags = (0..64)
        .map(|b| 1u64 << b)
        .filter(|bit| bit & reserved == 0)
        .filter_map(|bit| MeshPipelineKey::from_bits_retain(bit).iter_names().next().map(|(name, _)| (name, bit)))
        .collect();
    (flags, fields)
}

#[derive(Default)]
struct Keys {
    names: Option<(Names, Names)>,
    last: HashMap<RetainedViewEntity, u64>,
}

fn view_keys(cache: Option<Res<ViewKeyCache>>, mut k: Local<Keys>) {
    let Some(cache) = cache else { return };
    let Keys { names, last } = &mut *k;
    let (flags, fields) = names.get_or_insert_with(key_names);
    for (view, key) in cache.iter() {
        let new = key.bits();
        let old = last.insert(*view, new);
        if old == Some(new) {
            continue;
        }
        let (bits, cause) = match old {
            Some(old) => (describe_key_change(old, new, flags, fields), "settings changed: its materials re-specialize"),
            None => (format!("{new:#x}"), "new view"),
        };
        log::info!("view key changed: {bits} ({cause}; view {view:?})");
    }
    last.retain(|view, _| cache.contains_key(view));
}
