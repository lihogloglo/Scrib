//! Live view distance (tunables slice T5c): a change of the streaming [`LodScreen`] (`lod.bias`,
//! `lod.size_cull.*`) re-evaluates the LOD chains already spawned, in place, instead of waiting for
//! their cells to stream out and back.
//!
//! The industry form is per-frame or on-change LOD selection from data kept with the instance: UE's
//! `r.ViewDistanceScale` / `r.StaticMeshLODDistanceScale` and Unity's `LODGroup` with
//! `QualitySettings.lodBias` re-select every frame from the screen size; nothing respawns. Bevy draws
//! our LOD levels as entities with a `VisibilityRange` (a fixed range per entity, crossfaded by the
//! GPU), so the equivalent here is: every spawned LOD part carries its chain's inputs ([`LodPart`]:
//! the template, the scaled errors, the placement's end before size culling and its size-cull
//! radius); on a screen change [`apply_lod_screen`] recomputes each chain's ranges with the exact
//! functions a fresh spawn uses, writes the `VisibilityRange`s that changed, despawns the parts of
//! levels that collapsed and spawns the parts of levels that opened (a fresh spawn skips empty
//! levels, so a lower bias can need a coarser level that was never spawned). Re-extraction is
//! Bevy's: a changed `VisibilityRange` re-extracts its mesh instance.
//!
//! Spread over frames: at most [`StreamingConfig::lod_live_budget`](crate::StreamingConfig) chains
//! (placements or precombine buckets) per frame, so a preset switch never re-extracts every resident
//! mesh in one frame. Only LOD parts are touched: no placement, collider, script, actor or item state.
//!
//! Not reached: runtime-imported (uncooked) models keep their spawn-time end ([`crate::SubtreeRange`],
//! applied once per mesh); the static scene (`--scene`) has no LOD or size cull yet; far-field
//! proxies don't read this screen (their handoff is `far.handoff_scale`, a restart).

use std::collections::HashMap;
use std::sync::Arc;

use bevy_camera::visibility::{NoAutoAabb, NoCpuCulling, RenderLayers, VisibilityRange};
use bevy_ecs::bundle::{DynamicBundle, NoBundleEffect};
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh3d;
use bevy_pbr::MeshMaterial3d;
use bevy_platform::time::Instant;
use bevy_transform::components::Transform;
use sf_diag_bevy::DiagCounters;
use sf_model::{LodRange, LodScreen};
use sf_model_bevy::{NoRayTracing, RayTracingMesh};
use sf_world::{IndexedPlacement, PlacementClass, PlacementFlags};

use crate::precombine::bucket_ranges;
use crate::spawn::{ModelTemplate, SpawnCx, chain_ranges, visibility_range};
use crate::stream::Streaming;

/// Where a placement stops being drawn before size culling, and whether size culling may shorten it:
/// everything [`Self::resolve`] needs to redo the end under another [`LodScreen`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EndInputs {
    /// The ring's unload radius or the far-field handoff, meters.
    pub end: f32,
    /// The fade band past `end` (a far-field lead band), meters; 0: abrupt.
    pub band: f32,
    /// The bounds radius size culling projects; `None`: exempt (actors, doors, `NO_SIZE_CULL`, unknown
    /// bounds, kept by a far-field proxy).
    pub cull_radius: Option<f32>,
    /// It ends at a far-field handoff unless size culling ends it first (a precombine bucket then adds
    /// no reach).
    pub at_handoff: bool,
}

/// [`EndInputs`] under one screen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedEnd {
    pub distance: f32,
    pub band: f32,
    /// Size culling ended it before its ring or handoff.
    pub size_culled: bool,
}

impl EndInputs {
    /// An end no screen shortens.
    pub fn fixed(end: f32, band: f32) -> Self {
        Self { end, band, cull_radius: None, at_handoff: false }
    }

    /// A placement's end: `end`/`band` as streaming gives them (ring, or handoff and lead band),
    /// `kept_radius` `Some` for a placement that hands over to a far-field proxy (the smallest radius
    /// its cell's proxy keeps). Exempt from size culling: actors and doors, `NO_SIZE_CULL`, unknown
    /// bounds (radius 0) and what the proxy keeps (it must reach the handoff or a hole opens).
    pub fn of(placement: &IndexedPlacement, end: f32, band: f32, kept_radius: Option<f32>) -> Self {
        let p = &placement.placement;
        let exempt = matches!(p.class, PlacementClass::Actor | PlacementClass::Door)
            || p.flags.contains(PlacementFlags::NO_SIZE_CULL)
            || placement.radius <= 0.0
            || kept_radius.is_some_and(|k| placement.radius >= k);
        Self { end, band, cull_radius: (!exempt).then_some(placement.radius), at_handoff: kept_radius.is_some() }
    }

    /// The end under `lod`: [`LodScreen::draw_distance`] of the radius with a dithered fade
    /// `[d · (1 − c), d]` (`c` the crossfade fraction) when it comes before `end` (industry-audit.md J1).
    pub fn resolve(&self, lod: &LodScreen) -> ResolvedEnd {
        let unchanged = ResolvedEnd { distance: self.end, band: self.band, size_culled: false };
        let Some(radius) = self.cull_radius else { return unchanged };
        let d = lod.draw_distance(radius);
        if d >= self.end {
            return unchanged;
        }
        let band = d * lod.crossfade;
        ResolvedEnd { distance: d - band, band, size_culled: true }
    }
}

/// A precombine bucket's end (precombine.md): its farthest-reaching member's, measured from the
/// bucket origin (members ending at the handoff end there, the proxy's origin; the others reach
/// `reach` past their own end).
pub fn bucket_end(members: &[EndInputs], reach: f32, lod: &LodScreen) -> (f32, f32) {
    let (mut end, mut band) = (0.0f32, 0.0f32);
    for m in members {
        let e = m.resolve(lod);
        let reach = if m.at_handoff && !e.size_culled { 0.0 } else { reach };
        if e.distance + reach > end {
            (end, band) = (e.distance + reach, e.band);
        }
    }
    (end, band)
}

/// How a chain's end follows the screen.
#[derive(Clone, Debug)]
enum ChainEnd {
    /// One placement ([`chain_ranges`]).
    Placement(EndInputs),
    /// A precombine bucket ([`bucket_ranges`]): its members' ends and its reach.
    Bucket { reach: f32, members: Arc<[EndInputs]> },
}

/// One spawned LOD chain: what its parts were spawned from, so a screen change can redo their ranges
/// and spawn levels a fresh spawn would have.
#[derive(Clone, Debug)]
pub struct LodChain {
    template: Arc<ModelTemplate>,
    /// Each level's error in the parent's space (scaled with the placement).
    errors: Box<[f32]>,
    end: ChainEnd,
    parent: Entity,
    local: Transform,
    layers: RenderLayers,
    gpu_culled: bool,
    /// The screen the parts' ranges come from.
    screen: LodScreen,
}

impl LodChain {
    /// A placement's chain as `cx` spawns it.
    pub(crate) fn placement(cx: &SpawnCx<'_>, template: Arc<ModelTemplate>, gpu_culled: bool) -> Self {
        let scale = cx.local.scale.abs().max_element();
        let errors = template.lods.iter().map(|l| l.error * scale).collect();
        Self {
            template,
            errors,
            end: ChainEnd::Placement(cx.end),
            parent: cx.parent,
            local: cx.local,
            layers: cx.layers.clone(),
            gpu_culled,
            screen: *cx.lod,
        }
    }

    /// A precombine bucket's chain (errors in cell space, ranges from the bucket origin `local`).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn bucket(
        template: Arc<ModelTemplate>,
        reach: f32,
        members: Arc<[EndInputs]>,
        parent: Entity,
        local: Transform,
        layers: RenderLayers,
        gpu_culled: bool,
        screen: LodScreen,
    ) -> Self {
        let errors = template.lods.iter().map(|l| l.error).collect();
        Self { template, errors, end: ChainEnd::Bucket { reach, members }, parent, local, layers, gpu_culled, screen }
    }

    /// Every level's range under `lod` (what a fresh spawn under `lod` computes).
    pub fn ranges(&self, lod: &LodScreen) -> Vec<LodRange> {
        match &self.end {
            ChainEnd::Placement(e) => {
                let r = e.resolve(lod);
                chain_ranges(&self.errors, lod, r.distance, r.band)
            }
            ChainEnd::Bucket { reach, members } => {
                let (end, band) = bucket_end(members, *reach, lod);
                bucket_ranges(&self.errors, lod, end, band, *reach)
            }
        }
    }

    /// The screen its parts' ranges come from.
    pub fn screen(&self) -> &LodScreen {
        &self.screen
    }
}

/// On every LOD part a chain spawns: its chain and level.
#[derive(Component, Clone, Debug)]
pub struct LodPart {
    pub chain: Arc<LodChain>,
    pub level: u8,
}

/// Spawns the parts of `chain`'s levels that `levels` admits and whose range is not empty, each with
/// its [`LodPart`]. Returns the entities spawned.
///
/// With ray-tracing variants ([`ModelTemplate::ray_tracing`]), the traced level's parts carry their
/// [`RayTracingMesh`] and the other levels [`NoRayTracing`]: the ray tracer ignores visibility ranges
/// and sees exactly one level. `gpu_culled` chains' parts carry `NoCpuCulling` (statics only, see
/// [`crate::StaticGpuCulling`]): every LOD level stays alive and the GPU preprocess range-culls it per
/// view (vendored `mesh_preprocess.wesl`), as Bevy's CPU range check did.
pub(crate) fn spawn_levels(world: &mut World, chain: &Arc<LodChain>, ranges: &[LodRange], levels: impl Fn(usize) -> bool) -> u32 {
    let template = &chain.template;
    let mut plain = Vec::new();
    let mut traced = Vec::new();
    let mut untraced = Vec::new();
    for (level, (lod, range)) in template.lods.iter().zip(ranges).enumerate() {
        if !levels(level) {
            continue;
        }
        let Some(visibility) = visibility_range(range) else { continue };
        let tag = LodPart { chain: chain.clone(), level: u8::try_from(level).unwrap_or(u8::MAX) };
        for (part, (mesh, material)) in lod.parts.iter().enumerate() {
            let bundle = (
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                chain.local,
                visibility.clone(),
                chain.layers.clone(),
                template.aabb,
                // The model's bounds are exact already; skip Bevy's AABB scan.
                NoAutoAabb,
                // What the material-class gauges count (static-scene.md S2).
                crate::matclass::StaticPart,
                tag.clone(),
                ChildOf(chain.parent),
            );
            match &template.ray_tracing {
                None => plain.push(bundle),
                Some(rt) => match (rt.level == level).then(|| rt.meshes.get(part)).flatten() {
                    Some(variant) => traced.push((bundle, RayTracingMesh(variant.clone()))),
                    None => untraced.push((bundle, NoRayTracing)),
                },
            }
        }
    }
    let n = u32::try_from(plain.len() + traced.len() + untraced.len()).unwrap_or(u32::MAX);
    if chain.gpu_culled {
        spawn_parts::<NoCpuCulling, _, _, _>(world, plain, traced, untraced);
    } else {
        spawn_parts::<(), _, _, _>(world, plain, traced, untraced);
    }
    n
}

/// Spawns the three part lists, each part with an extra `T` (a marker or `()`).
fn spawn_parts<T: Bundle + Default, A: Bundle, B: Bundle, C: Bundle>(world: &mut World, plain: Vec<A>, traced: Vec<B>, untraced: Vec<C>)
where
    <T as DynamicBundle>::Effect: NoBundleEffect,
    <A as DynamicBundle>::Effect: NoBundleEffect,
    <B as DynamicBundle>::Effect: NoBundleEffect,
    <C as DynamicBundle>::Effect: NoBundleEffect,
{
    world.spawn_batch(plain.into_iter().map(|b| (b, T::default())));
    world.spawn_batch(traced.into_iter().map(|b| (b, T::default())));
    world.spawn_batch(untraced.into_iter().map(|b| (b, T::default())));
}

/// A stale chain and its parts (entity, level).
type StaleChain = (Arc<LodChain>, Vec<(Entity, u8)>);

/// What one [`apply_lod_screen`] frame did (summed into [`crate::StreamStats`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct LiveLodFrame {
    pub chains: u64,
    pub ranges_changed: u64,
    pub parts_spawned: u64,
    pub parts_despawned: u64,
}

/// `Update`, after streaming's drive: brings the LOD chains spawned under an older [`LodScreen`] to
/// the current one, at most `lod_live_budget` chains a frame. Idle (one compare) once every chain
/// is current. Not behind the selection gates (CW-2): no gated selection reads the screen (rings,
/// far-field residency), and this pass runs on the screen's change itself.
pub(crate) fn apply_lod_screen(world: &mut World, parts: &mut QueryState<(Entity, &'static LodPart)>, mut applied: Local<Option<LodScreen>>) {
    let Some(streaming) = world.get_resource::<Streaming>() else { return };
    let screen = streaming.config().lod;
    let budget = streaming.config().lod_live_budget.max(1);
    if *applied == Some(screen) {
        return;
    }
    let started = Instant::now();
    // Stale chains (by identity) with their parts, at most `budget` of them.
    let mut stale: HashMap<*const LodChain, StaleChain> = HashMap::new();
    let mut more = false;
    for (entity, part) in parts.iter(world) {
        if part.chain.screen == screen {
            continue;
        }
        let key = Arc::as_ptr(&part.chain);
        if let Some((_, members)) = stale.get_mut(&key) {
            members.push((entity, part.level));
        } else if stale.len() < budget {
            stale.insert(key, (part.chain.clone(), vec![(entity, part.level)]));
        } else {
            more = true;
        }
    }
    let mut frame = LiveLodFrame::default();
    for (chain, members) in stale.into_values() {
        apply_chain(world, &chain, &members, screen, &mut frame);
    }
    if !more {
        *applied = Some(screen);
    }
    if let Some(mut streaming) = world.get_resource_mut::<Streaming>() {
        streaming.add_lod_live(frame);
    }
    if frame.chains > 0
        && let Some(c) = world.get_resource::<DiagCounters>()
    {
        #[allow(clippy::cast_precision_loss)]
        c.0.add_named("stream/lod_live/chains", "count", frame.chains as f64);
        c.0.add_duration_named("stream/lod_live/ms", started.elapsed());
    }
}

/// Re-ranges one chain's parts under `screen`: changed ranges written, collapsed levels despawned,
/// opened levels spawned (under the chain's parent, with a part's current layers).
fn apply_chain(world: &mut World, old: &Arc<LodChain>, members: &[(Entity, u8)], screen: LodScreen, frame: &mut LiveLodFrame) {
    let mut chain = LodChain::clone(old);
    chain.screen = screen;
    if let Some(layers) = members.first().and_then(|(e, _)| world.get::<RenderLayers>(*e)) {
        chain.layers = layers.clone();
    }
    let chain = Arc::new(chain);
    let ranges = chain.ranges(&screen);
    let mut present = vec![false; chain.template.lods.len()];
    for &(entity, level) in members {
        let level = usize::from(level);
        if let Some(p) = present.get_mut(level) {
            *p = true;
        }
        match ranges.get(level).and_then(visibility_range) {
            Some(range) => {
                let Ok(mut e) = world.get_entity_mut(entity) else { continue };
                if let Some(mut current) = e.get_mut::<VisibilityRange>()
                    && current.set_if_neq(range)
                {
                    frame.ranges_changed += 1;
                }
                if let Some(mut part) = e.get_mut::<LodPart>() {
                    part.chain = chain.clone();
                }
            }
            None => {
                if world.get_entity(entity).is_ok() {
                    world.despawn(entity);
                    frame.parts_despawned += 1;
                }
            }
        }
    }
    let opened = |level: usize| !present.get(level).copied().unwrap_or(true);
    if (0..present.len()).any(opened) && world.get_entity(chain.parent).is_ok() {
        frame.parts_spawned += u64::from(spawn_levels(world, &chain, &ranges, opened));
    }
    frame.chains += 1;
}
