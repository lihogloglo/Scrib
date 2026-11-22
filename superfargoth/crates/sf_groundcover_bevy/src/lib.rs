//! The groundcover mesh renderer (spec `superfargoth/docs/specs/groundcover.md` "Renderers", decided GC-R, slice
//! GC-2): world tiles around the viewer are scattered on the async compute pool by the Bevy-free core
//! (`sf_groundcover`) and written into retained GPU instance batches (`GpuBatchedMesh3d`, the vendored bevy#25978
//! port the static scene uses), one per (model, shadow casting), drawn with [`GroundcoverMaterial`].
//!
//! What the game provides (nothing here names a game):
//! - [`GroundcoverSource`] (resource): the species set and rules (`Groundcover`), the `Overrides`, and the ground
//!   ([`GroundSource`], e.g. [`FnGround`] over a terrain height query). Replacing it rebuilds every tile.
//! - [`GroundcoverModels`] (resource): each mesh species' model key → [`GroundcoverModel`] (mesh handle, base
//!   `StandardMaterial`, optional bend height). A model may arrive late; its species draw from then on.
//! - [`GroundcoverViewer`] (component) on the camera whose `GlobalTransform` the rings follow.
//! - Positions: the ground is sampled in absolute world metres (`sf_coords::WorldPos::to_f64` frame, Y up); the
//!   floating origin is `sf_world_bevy::WorldOrigin` (absent = origin at zero), as for the static scene.
//!
//! Per frame ([`GroundcoverSystems::Drive`], `PostUpdate` after `WorldOriginSystems`): move the rings, poll finished
//! jobs, start new ones (nearest first, budgeted), open the batches whose model resolved, reconcile placements
//! ([`residency`]) and queue the slot runs ([`render`]). Gauges `groundcover/*` in `Last` ([`publish_gauges`]).
//!
//! ```no_run
//! use std::sync::Arc;
//! use bevy::prelude::*;
//! use sf_groundcover::{GroundSample, Groundcover, Overrides};
//! use sf_groundcover_bevy::*;
//!
//! fn setup(mut commands: Commands, cover: Res<MyCover>) {
//!     commands.insert_resource(GroundcoverSource {
//!         cover: Arc::new(cover.0.clone()),
//!         overrides: Arc::new(Overrides::none()),
//!         ground: Arc::new(FnGround(|_x: f64, _z: f64, out: &mut GroundSample| *out = GroundSample { channels: vec![1.0], ..GroundSample::blank() })),
//!     });
//! }
//! # #[derive(Resource)] struct MyCover(Groundcover);
//! ```

mod ground;
pub mod material;
mod pool;
pub mod render;
pub mod residency;
mod ring;
pub mod terrain_ground;
pub mod tunables;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use bevy_app::{App, Last, Plugin, PostUpdate};
use bevy_asset::{AssetApp, Assets, Handle, RenderAssetUsages, embedded_asset};
use bevy_camera::primitives::MeshAabb;
use bevy_ecs::prelude::*;
use bevy_light::NotShadowCaster;
use bevy_material::{AlphaMode, OpaqueRendererMethod};
use bevy_math::{DVec3, Vec3, Vec4};
use bevy_mesh::Mesh;
use bevy_pbr::gpu_instance_batch::{GpuBatchedMesh3d, GpuInstanceBatchReservations, GpuInstanceBatchSystems};
use bevy_pbr::{MaterialPlugin, MeshMaterial3d, StandardMaterial};
use bevy_platform::collections::HashMap;
use bevy_platform::time::Instant;
use bevy_render::storage::ShaderBuffer;
use bevy_render::{ExtractSchedule, RenderApp};
use bevy_render::renderer::RenderGraph;
use bevy_tasks::{AsyncComputeTaskPool, Task, futures::check_ready};
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;
use sf_diag_bevy::{DiagCounters, sf_diag::unit};
use sf_groundcover::{Groundcover, Overrides, ScatterSettings, TileKey};
use sf_tunables::{Def, Type};
use sf_tunables_bevy::{Tunables, TunablesAppExt};
use sf_wind_bevy::WindBuffer;
use sf_world_bevy::{WorldOrigin, WorldOriginSystems};

pub use ground::{FnGround, GroundSource, LatticeGround};
pub use material::{GroundcoverMaterial, GroundcoverParams, GroundcoverSpeciesGpu, GroundcoverWind, WindSettings};
pub use pool::SlotPool;
pub use render::{GroundcoverUploads, GroundcoverWrite};
pub use residency::{JobInputs, Plan, RenderSettings, Residency, ResidencyStats, TileScatter, scatter_tile, slot_bytes, slot_words};
pub use ring::{Measure, RingDelta, RingSettings, TileRing, farthest_distance, max_ring_tiles, nearest_distance, tiles_within};
pub use terrain_ground::{LayerChannels, TerrainGround, TerrainGroundPlugin, TerrainGroundRef};

use render::{PendingGroundcoverWrites, extract_groundcover_uploads, publish_groundcover};
use residency::batch_index;

/// The species set, overrides and ground the renderer scatters. Insert or replace it to (re)build.
#[derive(Resource, Clone)]
pub struct GroundcoverSource {
    pub cover: Arc<Groundcover>,
    pub overrides: Arc<Overrides>,
    pub ground: Arc<dyn GroundSource>,
}

/// A resolved mesh species model.
#[derive(Clone, Debug)]
pub struct GroundcoverModel {
    pub mesh: Handle<Mesh>,
    /// The base material (texture, alpha cutoff, double-sidedness). The renderer forces forward rendering and
    /// turns `AlphaMode::Blend` into a mask (instanced plants are not sorted).
    pub material: StandardMaterial,
    /// Height of the model (metres, unscaled) the bend is weighted by; `None` = the mesh AABB's top (the renderer
    /// waits for the mesh to load).
    pub bend_height_m: Option<f32>,
}

/// Model key (`RenderKind::Mesh { model }`) → model, filled by the game.
#[derive(Resource, Clone, Debug, Default)]
pub struct GroundcoverModels(pub HashMap<String, GroundcoverModel>);

/// The camera the tile rings follow (its `GlobalTransform`, relative to the floating origin). One per app; with
/// several, the first in query order.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GroundcoverViewer;

/// On a batch entity.
#[derive(Component, Clone, Debug)]
pub struct GroundcoverBatch {
    pub model: String,
    pub casting: bool,
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GroundcoverSystems {
    /// `PostUpdate`, after `WorldOriginSystems`: rings, jobs, batches, slot writes.
    Drive,
}

/// What the renderer did, for tests and tools (the `groundcover/*` gauges carry the same).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct GroundcoverStats {
    pub residency: ResidencyStats,
    pub pending_jobs: u32,
    pub batches: u32,
    pub live_slots: u64,
    pub capacity_slots: u64,
    /// Mean scatter time per tile (µs, exponential mean over finished jobs).
    pub scatter_us: f32,
    /// Bytes queued for the GPU this frame.
    pub upload_bytes: u64,
}

/// The renderer: registers the tunables (`groundcover.*` core and `groundcover.render.*`), the material and the
/// slot writer, and drives residency (module docs).
pub struct GroundcoverPlugin;

impl Plugin for GroundcoverPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/groundcover.wesl");
        embedded_asset!(app, "shaders/groundcover_prepass.wesl");
        embedded_asset!(app, "shaders/groundcover_common.wesl");
        if app.world().contains_resource::<Assets<bevy_shader::Shader>>() {
            // The import library must stay loaded (as `load_shader_library!`).
            std::mem::forget(bevy_asset::load_embedded_asset!(app, "shaders/groundcover_common.wesl") as Handle<bevy_shader::Shader>);
        }
        if app.get_sub_app(RenderApp).is_some() {
            if !app.is_plugin_added::<MaterialPlugin<GroundcoverMaterial>>() {
                app.add_plugins(MaterialPlugin::<GroundcoverMaterial>::default());
            }
        } else {
            app.init_asset::<GroundcoverMaterial>();
        }
        app.register_tunables(sf_groundcover::tunables::TUNABLES)
            .register_tunables(tunables::TUNABLES)
            .init_resource::<GroundcoverModels>()
            .init_resource::<GroundcoverUploads>()
            .init_resource::<GroundcoverStats>()
            .init_resource::<Runtime>()
            .add_systems(PostUpdate, drive.in_set(GroundcoverSystems::Drive).after(WorldOriginSystems))
            .add_systems(Last, publish_gauges.run_if(resource_exists::<DiagCounters>));
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app
            .init_resource::<PendingGroundcoverWrites>()
            .add_systems(ExtractSchedule, extract_groundcover_uploads)
            .add_systems(
                RenderGraph,
                publish_groundcover
                    .in_set(GpuInstanceBatchSystems::Publish)
                    .run_if(resource_exists::<GpuInstanceBatchReservations>),
            );
    }
}

/// A def's value in `store`, or its default when nobody registered it.
fn read<T: Type + Sync + Copy>(store: &sf_tunables::Tunables, def: &Def<T>) -> T
where
    for<'a> T::Read<'a>: Into<T>,
{
    store.id_of(def).map_or_else(|_| def.default_value(), |id| store.read(id).into())
}

impl RenderSettings {
    /// The store's values (defaults for anything unregistered).
    pub fn from_tunables(store: &sf_tunables::Tunables) -> Self {
        use sf_groundcover::tunables as c;
        use tunables as r;
        Self {
            scatter: ScatterSettings {
                tile_size_m: read(store, &c::TILE_SIZE),
                max_instances_per_tile: read(store, &c::MAX_INSTANCES_PER_TILE),
                density_scale: read(store, &c::DENSITY_SCALE),
            },
            mesh_range_m: read(store, &r::MESH_RANGE),
            hysteresis_m: read(store, &r::HYSTERESIS),
            shadow_range_m: read(store, &r::SHADOW_RANGE),
            fade_width_m: read(store, &r::FADE_WIDTH),
            fade_shrink_m: read(store, &r::FADE_SHRINK),
            max_blocks_per_batch: read(store, &r::MAX_BLOCKS_PER_BATCH),
            jobs_per_frame: read(store, &r::JOBS_PER_FRAME),
            max_jobs_in_flight: read(store, &r::MAX_JOBS_IN_FLIGHT),
            ground_cache_m: read(store, &r::GROUND_CACHE_SPACING),
        }
    }
}

impl WindSettings {
    pub fn from_tunables(store: &sf_tunables::Tunables) -> Self {
        Self {
            scale: read(store, &tunables::WIND_SCALE),
            bend_per_mps: read(store, &tunables::BEND_PER_MPS),
            phase_offset_m: read(store, &tunables::PHASE_OFFSET),
        }
    }
}

/// A scatter task's result and its time.
type JobOutput = (TileScatter, f32);

/// The live state behind [`drive`].
#[derive(Resource, Default)]
struct Runtime {
    settings: Option<RenderSettings>,
    wind: WindSettings,
    residency: Option<Residency>,
    inputs: Option<JobInputs>,
    jobs: BTreeMap<TileKey, Task<JobOutput>>,
    /// By batch index.
    batches: Vec<Option<(Entity, Handle<GroundcoverMaterial>)>>,
    /// Zeroed stand-in wind buffer when no `WindBuffer` exists.
    still_air: Option<Handle<ShaderBuffer>>,
    scatter_us: f32,
}

impl Runtime {
    fn clear(&mut self, commands: &mut Commands) {
        for (entity, _) in self.batches.drain(..).flatten() {
            commands.entity(entity).despawn();
        }
        self.jobs.clear();
        self.residency = None;
        self.inputs = None;
    }
}

/// The renderer's per-frame work (module docs).
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn drive(
    mut runtime: ResMut<Runtime>,
    source: Option<Res<GroundcoverSource>>,
    tunables: Option<Res<Tunables>>,
    models: Res<GroundcoverModels>,
    meshes: Option<Res<Assets<Mesh>>>,
    mut materials: ResMut<Assets<GroundcoverMaterial>>,
    wind_buffer: Option<Res<WindBuffer>>,
    mut shader_buffers: Option<ResMut<Assets<ShaderBuffer>>>,
    origin: Option<Res<WorldOrigin>>,
    viewers: Query<&GlobalTransform, With<GroundcoverViewer>>,
    mut uploads: ResMut<GroundcoverUploads>,
    mut stats: ResMut<GroundcoverStats>,
    mut commands: Commands,
) {
    let rt = &mut *runtime;
    let settings = tunables.as_ref().map_or_else(RenderSettings::default, |t| RenderSettings::from_tunables(&t.0));
    let wind = tunables.as_ref().map_or_else(WindSettings::default, |t| WindSettings::from_tunables(&t.0));
    let source_changed = source.as_ref().is_some_and(Res::is_changed);
    if source.is_none() || source_changed || rt.settings != Some(settings) {
        rt.clear(&mut commands);
        rt.settings = Some(settings);
    }
    let Some(source) = source else {
        *stats = GroundcoverStats::default();
        return;
    };
    if rt.residency.is_none() {
        let plan = Plan::new(&source.cover, &settings);
        let residency = Residency::new(plan, settings);
        rt.inputs = Some(JobInputs {
            cover: source.cover.clone(),
            overrides: source.overrides.clone(),
            ground: source.ground.clone(),
            settings,
            species: residency.job_species(),
        });
        rt.batches = vec![None; residency.plan.batch_count()];
        rt.residency = Some(residency);
    }
    // Live wind values: every batch material.
    if rt.wind != wind {
        rt.wind = wind;
        for (_, handle) in rt.batches.iter().flatten() {
            if let Some(mut m) = materials.get_mut(handle) {
                m.extension.params.wind = wind.to_gpu();
            }
        }
    }
    let (Some(residency), Some(inputs)) = (rt.residency.as_mut(), rt.inputs.as_ref()) else { return };
    let Some(viewer) = viewers.iter().next() else { return };

    // Floating origin: render frame = world − the origin cell's frame (as `sf_scene::cell_offset`).
    let origin = origin.map_or(DVec3::ZERO, |o| WorldPos::new(o.cell, Vec3::ZERO).to_f64());
    let eye = origin + viewer.translation().as_dvec3();
    let p = [eye.x, eye.z];

    // Rings; jobs of tiles that left are dropped (cancelled).
    for k in residency.update_rings(p) {
        rt.jobs.remove(&k);
    }

    // Finished jobs, in key order.
    let mut done = Vec::new();
    for (&k, task) in &mut rt.jobs {
        if let Some(out) = check_ready(task) {
            done.push((k, out));
        }
    }
    for (k, (scatter, us)) in done {
        rt.jobs.remove(&k);
        rt.scatter_us += (us - rt.scatter_us) / 16.0;
        residency.insert(scatter);
    }

    // New jobs, nearest first.
    let room = settings.max_jobs_in_flight.saturating_sub(u32::try_from(rt.jobs.len()).unwrap_or(u32::MAX));
    let n = settings.jobs_per_frame.min(room) as usize;
    if n > 0 {
        let in_flight: BTreeSet<TileKey> = rt.jobs.keys().copied().collect();
        let size = settings.scatter.tile_size_m;
        let ground = inputs.ground.clone();
        let pool = AsyncComputeTaskPool::get();
        for k in residency.next_jobs(p, &in_flight, |k| ground.ready(k.rect(size)), n) {
            let inputs = inputs.clone();
            let task = pool.spawn(async move {
                let start = Instant::now();
                let out = scatter_tile(&inputs, k);
                (out, start.elapsed().as_secs_f32() * 1.0e6)
            });
            rt.jobs.insert(k, task);
        }
    }

    // Batches whose model resolved.
    let wind_handle = match (&wind_buffer, shader_buffers.as_mut()) {
        (Some(w), _) => w.0.clone(),
        (None, Some(buffers)) => rt
            .still_air
            .get_or_insert_with(|| {
                let mut b = ShaderBuffer::new(vec![[0.0_f32; 4]; sf_wind_bevy::DATA_VEC4S], RenderAssetUsages::default());
                b.label = "sf_groundcover still air".into();
                buffers.add(b)
            })
            .clone(),
        (None, None) => Handle::default(),
    };
    for (gi, group) in residency.plan.groups.clone().iter().enumerate() {
        let Some(model) = models.0.get(&group.model) else { continue };
        for casting in [false, true] {
            let b = batch_index(gi, casting);
            if residency.is_open(b) || residency.plan.batch_layout(b).0 == 0 {
                continue;
            }
            let height = model.bend_height_m.or_else(|| {
                let aabb = meshes.as_ref()?.get(&model.mesh)?.get_aabb()?;
                Some(aabb.max().y)
            });
            let Some(height) = height else { continue };
            let Some((blocks, slots)) = residency.open_batch(b) else { continue };
            let capacity = blocks * slots;
            let material = batch_material(model, group, &inputs.cover, &settings, wind, height, casting, wind_handle.clone());
            let handle = materials.add(material);
            let mut entity = commands.spawn((
                GpuBatchedMesh3d { mesh: model.mesh.clone(), max_capacity: capacity, ranges: Vec::new() },
                MeshMaterial3d(handle.clone()),
                GroundcoverBatch { model: group.model.clone(), casting },
            ));
            if !casting {
                entity.insert(NotShadowCaster);
            }
            let entity = entity.id();
            // A new batch's whole output zeroed before anything draws (as the static scene's batches).
            uploads.writes.push(GroundcoverWrite { entity, first_slot: 0, bytes: vec![0; capacity as usize * 64] });
            rt.batches[b] = Some((entity, handle));
        }
    }

    // Placements → slot runs.
    let mut upload_bytes = 0u64;
    for w in residency.reconcile(origin) {
        let Some(entity) = rt.batches.get(w.batch).and_then(|b| b.as_ref().map(|(e, _)| *e)) else { continue };
        let bytes = slot_bytes(&w.slots);
        upload_bytes += bytes.len() as u64;
        uploads.writes.push(GroundcoverWrite { entity, first_slot: w.first_slot, bytes });
    }

    let mut live = 0u64;
    let mut capacity = 0u64;
    let mut batches = 0u32;
    for b in 0..residency.plan.batch_count() {
        if let Some(pool) = residency.pool(b) {
            live += pool.live_slots();
            capacity += u64::from(pool.capacity());
            batches += 1;
        }
    }
    *stats = GroundcoverStats {
        residency: residency.stats,
        pending_jobs: u32::try_from(rt.jobs.len()).unwrap_or(u32::MAX),
        batches,
        live_slots: live,
        capacity_slots: capacity,
        scatter_us: rt.scatter_us,
        upload_bytes,
    };
}

/// The material of batch (group, casting): the model's base forced forward (and masked, not blended), the species
/// rows (tints, wind response, bend height), the fade (off for the casting batch) and the wind.
#[allow(clippy::too_many_arguments)]
fn batch_material(
    model: &GroundcoverModel,
    group: &residency::Group,
    cover: &Groundcover,
    settings: &RenderSettings,
    wind: WindSettings,
    height: f32,
    casting: bool,
    wind_buffer: Handle<ShaderBuffer>,
) -> GroundcoverMaterial {
    let mut base = model.material.clone();
    base.opaque_render_method = OpaqueRendererMethod::Forward;
    if matches!(base.alpha_mode, AlphaMode::Blend | AlphaMode::Premultiplied | AlphaMode::Add) {
        // Bevy's default mask cutoff (`AlphaMode::Mask` docs).
        base.alpha_mode = AlphaMode::Mask(0.5);
    }
    let mut params = GroundcoverParams {
        fade: if casting {
            Vec4::ZERO
        } else {
            Vec4::new(settings.mesh_range_m, settings.fade_width_m, settings.fade_shrink_m, 1.0)
        },
        wind: wind.to_gpu(),
        ..Default::default()
    };
    for (row, &s) in params.species.iter_mut().zip(&group.species) {
        let sp = cover.species(s);
        let [a, b] = sp.tint;
        *row = GroundcoverSpeciesGpu {
            tint_a: Vec4::new(a[0], a[1], a[2], sp.wind),
            tint_b: Vec4::new(b[0], b[1], b[2], height),
        };
    }
    GroundcoverMaterial { base, extension: GroundcoverWind { params, wind: wind_buffer } }
}

/// `Last`: the `groundcover/*` gauges (zeros included, so a gauge never reads stale).
pub fn publish_gauges(stats: Res<GroundcoverStats>, counters: Res<DiagCounters>) {
    let s = &*stats;
    #[expect(clippy::cast_precision_loss, reason = "gauges; exact below 2^53")]
    let set = |name: &str, u: &str, v: u64| counters.set_named(name, u, v as f64);
    set("groundcover/tiles/resident", unit::COUNT, u64::from(s.residency.resident));
    set("groundcover/tiles/scattered", unit::COUNT, u64::from(s.residency.scattered));
    set("groundcover/tiles/casting", unit::COUNT, u64::from(s.residency.casting));
    set("groundcover/tiles/capped", unit::COUNT, u64::from(s.residency.capped));
    set("groundcover/tiles/unplaced", unit::COUNT, u64::from(s.residency.unplaced));
    set("groundcover/jobs/pending", unit::COUNT, u64::from(s.pending_jobs));
    set("groundcover/batches/count", unit::COUNT, u64::from(s.batches));
    set("groundcover/slots/live", unit::COUNT, s.live_slots);
    set("groundcover/slots/capacity", unit::COUNT, s.capacity_slots);
    set("groundcover/slots/capacity_bytes", unit::BYTES, s.capacity_slots * render::SLOT_BYTES);
    set("groundcover/upload/bytes", unit::BYTES, s.upload_bytes);
    counters.set_named("groundcover/scatter/us", "us", f64::from(s.scatter_us));
}

#[cfg(test)]
mod shader_tests;
#[cfg(test)]
mod tests;
