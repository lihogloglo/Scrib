//! The lamp shadow cache's Bevy side (ROADMAP 1.25 CW-4, slice S3; spec
//! `docs/specs/lamp-shadow-cache.md` "Design", "S3 as built"). The policy is
//! [`sf_sky::lamp_cache`] (S1), the render mechanism the vendored `bevy_pbr` patch "own, CW-4:
//! lamp shadow cache" (S2). This module joins them:
//!
//! - **Who is cached.** With [`LampShadowCache::enabled`] (`shadow.lamp.cache.enabled`, live), a
//!   [`BudgetedShadow`] lamp that holds a budget slot and is still is admitted: it gets
//!   `bevy_pbr::CachedPointShadowMap` and [`ShadowMapCached`], and its
//!   `PointLight::shadow_maps_enabled` goes false, so Bevy's caster scan and hashing skip it.
//!   The budget keeps choosing holders and fading them; a cached lamp's fade does not rise until
//!   all its faces are captured. Off (or a lamp that moved): the stock path, byte for byte.
//! - **Casters per lamp.** At admission one scan of the shadow casters in the lamp's shadowed
//!   reach (`ShadowReach::shadowed`, else its range) lists them per cube face, with Bevy's own
//!   tests (sphere, face frusta, `RenderLayers`, `InheritedVisibility`, `NotShadowCaster`,
//!   `VisibilityRange` from the lamp). After that only casters whose transform, bounds,
//!   visibility or layers changed are looked at (and removed ones). Comparisons are light
//!   relative at 1 mm, never change ticks, so a floating-origin shift changes nothing.
//! - **Classification** ([`LampCacheConfig::classify`], S1): skinned, morphed and
//!   [`LampShadowMobility::Movable`] casters (on the mesh or an ancestor) always move;
//!   `NoCpuCulling` casters and GPU instance batches are static at once; other rigid casters are
//!   movers until still for `settle_time_s`. A caster that settles, moves, spawns, despawns, hides
//!   or shows dirties the faces it touches (`Invalidation::StaticsChanged`); a static not yet in a
//!   capture is drawn as a mover meanwhile. A lamp that moves against its statics leaves the
//!   cache (`Invalidation::Moved`, stock path) until it has been still for `settle_time_s`.
//! - **Each frame.** Mover faces are the faces a mover is in, kept only while the lamp's sphere
//!   is in a camera frustum (SH-21b) and the face is seen (`bevy_pbr::point_shadow_faces_seen`,
//!   CW-3b's test). S1 plans; the lamp's `CachedPointShadowMap` is written from its `LampPlan`
//!   (`view_faces = views`, `draw_faces = capture | movers`, `load_faces = movers`, its copies,
//!   `plan = frame`) only when there is something to do, and its `CubemapVisibleEntities` hold
//!   the static list on capture faces and the mover list on mover faces (written on change).
//!   Listed casters are marked view-visible so off-screen ones are extracted.
//! - **Feedback.** `bevy_pbr::PointShadowCacheFeedback` (and [`LampCacheReports`], for hosts and
//!   tests without a render world): confirmed faces of a plan's captures go to
//!   `confirm_capture`; a lost slot recaptures the lamp (`Invalidation::SlotLost`, admitted
//!   again, its fade back to 0).
//! - **Gauges** `lamp_cache/*` ([`LampCacheStats`]; `lamp_cache/render/*` from the render world).
//!
//! Not handled yet (spec "S3 as built"): instances changing inside a GPU instance batch (only the
//! batch entity is watched).

use bevy_app::{App, PostUpdate};
use bevy_camera::primitives::{Aabb, CubemapFrusta, CUBE_MAP_FACES, CubeMapFace, Frustum, Sphere};
use bevy_camera::visibility::{
    CubemapVisibleEntities, InheritedVisibility, NoCpuCulling, NoFrustumCulling, RenderLayers, SetViewVisibility,
    ViewVisibility, VisibilityRange, VisibilitySystems,
};
use bevy_camera::{Camera, Camera3d};
use bevy_ecs::entity::{EntityHashMap, EntityHashSet};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_light::{NotShadowCaster, PointLight, PointLightShadowMap, SimulationLightSystems};
use bevy_math::{proj, Affine3A, Vec3, Vec3A};
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::Mesh3d;
use bevy_pbr::gpu_instance_batch::{GpuBatchedLightVisibility, GpuBatchedMesh3d};
use bevy_pbr::{
    point_shadow_faces_seen, AtlasPointShadowMap, AtlasPointShadowReport, CachedPointShadowCopy,
    CachedPointShadowMap, CachedPointShadowReport, PointLightShadowFade, PointShadowAtlasFeedback,
    PointShadowCacheFeedback, PointShadowCacheStats, PointShadowCacheStore, PointShadowFaceCull,
};
use bevy_render::{Render, RenderApp, RenderSystems};
use bevy_shape::ViewFrustum;
use bevy_time::Time;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_diag_bevy::DiagCounters;
use sf_sky::lamp_cache::{
    self as policy, CasterClass, CasterMobility, CopyOp, FramePlan, Invalidation, LampCache, LampCacheConfig,
    LampKey, LampTier, MoverInput, TileLayout, TileMask,
};
use sf_sky::shadow_atlas::{
    guard_band_fov_y, pick_tile_size, ShadowAtlasManager,
};
use sf_tunables_bevy::TunablesAppExt;

use crate::shadow_atlas::{
    publish_atlas_gauges, LampShadowAtlas, LampShadowAtlasReports, LampShadowAtlasStats,
};
use crate::shadow_budget::{budget_shadows, BudgetedShadow, PointShadowBudget, ShadowMapCached, ShadowReach, ShadowSlot};

/// All six cube faces.
const ALL_FACES: u8 = 0b11_1111; // tunables: const (layout: the six cube faces)
/// Light-relative change below which a caster or lamp has not moved (the spec's 1 mm rule).
const STILL_EPSILON: f32 = 1e-3; // tunables: const (fixture: the spec's 1 mm comparison)
/// Capture plans remembered per lamp to match render-world confirmations (a report arrives a
/// frame after its plan with pipelined rendering).
const CAPTURE_HISTORY: usize = 8; // tunables: const (capacity: plans in flight)
/// Ancestors searched for a [`LampShadowMobility`] mark.
const MOBILITY_DEPTH: usize = 16; // tunables: const (capacity: hierarchy depth searched)
/// Minimum static capture faces per frame under 2D atlas mode (smaller 256²/128² tiles capture
/// inside one encoder so entering a 30+ lamp interior populates quickly).
const ATLAS_MIN_CAPTURES_PER_FRAME: u32 = 6; // tunables: const (capacity: atlas initial capture floor)
/// Default tile size requested for moving lights (carried torches) in the Live Atlas.
const ATLAS_MOVING_TILE_PX: u16 = 512; // tunables: const (layout: hero torch live tile size)
/// All six faces have a store slot in the policy's in-memory pool when the 2D atlas owns VRAM.
const ATLAS_POLICY_STORE_FACES: u32 = 6; // tunables: const (layout: 6 faces per cube in policy pool)

/// The cache's switch and policy values, bound to the `shadow.lamp.cache.*` tunables.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LampShadowCache {
    /// `shadow.lamp.cache.enabled`: budgeted still lamps go through the cache; off = stock path.
    pub enabled: bool,
    pub config: LampCacheConfig,
}

/// How a shadow caster moves for the lamp cache, on the mesh or any ancestor (the nearest mark
/// wins). Without one: skinned and morphed meshes move, other meshes settle.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LampShadowMobility {
    /// Decided by the mesh (no override).
    #[default]
    Auto,
    /// Always drawn over the cached depth (characters and what they carry).
    Movable,
    /// Never moves: in the cached depth at once.
    Static,
    /// Placed still, animated in the vertex shader (wind): frozen in the cache unless
    /// `shadow.lamp.cache.animated_casters_move`.
    AnimatedStatic,
}

/// Render-world reports from elsewhere (a host's own renderer, tests), read with
/// `bevy_pbr::PointShadowCacheFeedback`.
#[derive(Resource, Default)]
pub struct LampCacheReports(pub Vec<(Entity, CachedPointShadowReport)>);

/// What the cache did this frame (main world; also the `lamp_cache/*` gauges).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LampCacheStats {
    /// The policy's frame (S1 [`policy::FrameStats`]).
    pub frame: policy::FrameStats,
    /// Face views the cached lamps keep (the components' `view_faces`).
    pub views: u32,
    /// Slot holders on the stock path (moving, refused or the cache off).
    pub stock_lamps: u32,
    /// Casters drawn as movers (in some cached lamp's reach).
    pub movers: u32,
    /// Full caster scans since startup (admissions and moved lamps without statics).
    pub scans: u64,
    /// Store layers in use / in the pool.
    pub store_used: u32,
    pub store_layers: u32,
}

/// One caster of one cached lamp.
#[derive(Clone, Copy, Debug)]
struct Caster {
    /// World affine with the lamp's translation taken out.
    rel: Affine3A,
    aabb: Option<Aabb>,
    /// Faces whose frustum it meets.
    faces: u8,
    /// World centre (mover distances).
    center: Vec3A,
    mobility: CasterMobility,
    class: CasterClass,
    last_move_s: f32,
    /// Faces whose current capture holds it.
    baked: u8,
    /// A GPU instance batch: no transform of its own to watch.
    batch: bool,
    no_cpu_culling: bool,
}

impl Caster {
    /// Faces it is drawn on as a mover: all its faces if it moves, the faces whose capture does
    /// not hold it yet if it is static.
    fn mover_faces(&self) -> u8 {
        match self.class {
            CasterClass::Mover => self.faces,
            CasterClass::Static => self.faces & !self.baked,
        }
    }
}

struct Lamp {
    key: LampKey,
    pos: Vec3A,
    reach: f32,
    frusta: CubemapFrusta,
    layers: RenderLayers,
    casters: EntityHashMap<Caster>,
    /// Moved against its statics this frame: leaves the cache next frame.
    moved: bool,
    /// `(plan, capture faces)` of recent plans.
    captures: Vec<(u64, u8)>,
    /// Current atlas tile resolution (unused on CW-4 cube path).
    tile_size: u16,
    /// Priority from [`BudgetedShadow`] (for atlas tier selection).
    priority: f32,
    /// Active visible mover faces this frame (keeps Live Atlas tiles resident across Half/Far skip frames).
    active_mover_faces: u8,
    /// Which atlas rects hold this lamp's own depth (SH-33 flashing, 2026-11-20).
    atlas_written: AtlasWritten,
}

/// Per-face atlas rects (`[x, y, size]`, `None` = no tile), as in `AtlasPointShadowMap`.
type FaceRects = [Option<[u16; 3]>; 6];

/// Tracks, per face, whether the lamp's current Static / Live atlas rect has been drawn since it
/// was (re)allocated. A rect fresh from the buddy allocator still holds another lamp's old depth,
/// so it is published for sampling only once a draw went into it (SH-33: flashing stray shadows
/// when Live tiles were sampled on the mover scheduler's skip frames, or Static rects after a tier
/// resize before their recapture).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AtlasWritten {
    static_rects: FaceRects,
    live_rects: FaceRects,
    static_mask: u8,
    live_mask: u8,
}

/// Masks out rects not yet drawn: `drawn` faces get a draw into `rects` this frame. A face whose
/// rect changed since the last call loses its written bit until drawn again. Returns the rects to
/// publish (drawn this frame or earlier since allocation).
fn gate_written(rects: FaceRects, last: &mut FaceRects, written: &mut u8, drawn: u8) -> FaceRects {
    let mut out = [None; 6];
    for (face, (rect, prev)) in rects.iter().zip(last.iter()).enumerate() {
        let bit = 1u8 << face;
        if rect != prev || rect.is_none() {
            *written &= !bit;
        }
        if rect.is_some() && drawn & bit != 0 {
            *written |= bit;
        }
        if *written & bit != 0 {
            out[face] = *rect;
        }
    }
    *last = rects;
    out
}

impl AtlasWritten {
    pub(crate) fn gate(&mut self, static_rects: FaceRects, live_rects: FaceRects, capture: u8, movers: u8) -> (FaceRects, FaceRects) {
        let s = gate_written(static_rects, &mut self.static_rects, &mut self.static_mask, capture);
        let l = gate_written(live_rects, &mut self.live_rects, &mut self.live_mask, movers);
        (s, l)
    }
}

impl Lamp {
    fn statics(&self) -> TileMask {
        TileMask(self.casters.values().filter(|c| c.class == CasterClass::Static).fold(0, |m, c| m | c.faces))
    }

    /// Every static in its faces' next capture (an admission, a lost slot): none is drawn as a
    /// mover meanwhile.
    fn bake_statics(&mut self) {
        for c in self.casters.values_mut() {
            c.baked = if c.class == CasterClass::Static { c.faces } else { 0 };
        }
    }

    /// A static to compare the lamp's position against later.
    fn reference(&self) -> Option<(Entity, Vec3A)> {
        self.casters
            .iter()
            .find(|(_, c)| c.class == CasterClass::Static && !c.batch)
            .map(|(e, c)| (*e, c.rel.translation))
    }
}

/// A lamp out of the cache because it moved: back once still for `settle_time_s`.
struct MovingLamp {
    reference: Option<(Entity, Vec3A)>,
    still_since: f32,
}

/// A moving lamp (e.g. carried torch) rendered directly into the 2D Live Atlas.
struct MovingAtlasLamp {
    key: LampKey,
    tile_size: u16,
    layers: RenderLayers,
}

#[derive(Resource, Default)]
struct LampCacheState {
    cache: Option<LampCache>,
    atlas: Option<ShadowAtlasManager>,
    atlas_mode: bool,
    lamps: EntityHashMap<Lamp>,
    moving: EntityHashMap<MovingLamp>,
    /// Moving lamps currently drawing directly into the 2D Live Atlas (carried torches).
    moving_atlas: EntityHashMap<MovingAtlasLamp>,
    /// Lamps whose stock cube map was drawn last frame: their live layers hold valid depth.
    stock_drawn: EntityHashSet,
    map_size: usize,
    max_lamps: u32,
    scans: u64,
    /// Last elapsed second a `debug` mover-breakdown line was logged (Slice A0 audit).
    last_mover_log_s: f32,
    /// Monotonic plan counter for moving atlas lamps.
    moving_plan: u64,
    /// Static atlas generation at which a non-handover (or handover) admission failed,
    /// avoiding repeated full-world caster scans until static atlas space is freed.
    static_full_at_gen: Option<(u64, bool)>,
}

type CasterData<'a> = (
    Entity,
    &'a GlobalTransform,
    Option<&'a Aabb>,
    &'a InheritedVisibility,
    Option<&'a RenderLayers>,
    (Has<NoFrustumCulling>, Has<NoCpuCulling>, Has<SkinnedMesh>, Has<MeshMorphWeights>),
    Option<&'a VisibilityRange>,
);
type IsCaster = (With<Mesh3d>, Without<NotShadowCaster>, Without<PointLight>);
type IsBatch = (With<GpuBatchedMesh3d>, Without<Mesh3d>, Without<NotShadowCaster>, Without<NoCpuCulling>);
type CasterChanged = Or<(Changed<GlobalTransform>, Changed<Aabb>, Changed<InheritedVisibility>, Changed<RenderLayers>)>;

/// The world's shadow casters, as the lamp cache sees them.
#[derive(SystemParam)]
struct Casters<'w, 's> {
    meshes: Query<'w, 's, CasterData<'static>, IsCaster>,
    batches: Query<'w, 's, (Entity, Ref<'static, InheritedVisibility>, Option<&'static RenderLayers>), IsBatch>,
    parents: Query<'w, 's, &'static ChildOf>,
    marks: Query<'w, 's, &'static LampShadowMobility>,
}

impl Casters<'_, '_> {
    /// How `entity` moves: its own or its nearest ancestor's mark, else by its mesh.
    fn mobility(&self, entity: Entity, skinned: bool, morphed: bool, no_cpu_culling: bool) -> CasterMobility {
        if no_cpu_culling {
            return CasterMobility::Static;
        }
        let mut at = Some(entity);
        for _ in 0..MOBILITY_DEPTH {
            let Some(e) = at else { break };
            match self.marks.get(e) {
                Ok(LampShadowMobility::Movable) => return CasterMobility::Movable,
                Ok(LampShadowMobility::Static) => return CasterMobility::Static,
                Ok(LampShadowMobility::AnimatedStatic) => return CasterMobility::AnimatedStatic,
                Ok(LampShadowMobility::Auto) | Err(_) => {}
            }
            at = self.parents.get(e).ok().map(ChildOf::parent);
        }
        if skinned || morphed { CasterMobility::Movable } else { CasterMobility::Rigid }
    }

    /// `entity` as a caster of `lamp`, or `None` (not a caster, hidden, other layers, out of
    /// reach). `settled`: a caster that was already there (an admission scan) is still at once.
    fn caster(&self, entity: Entity, lamp: &Lamp, now_s: f32, settled: bool, config: &LampCacheConfig) -> Option<Caster> {
        let (_, transform, aabb, shown, layers, (unculled, no_cpu, skinned, morphed), range) = self.meshes.get(entity).ok()?;
        if !shown.get() || !lamp.layers.intersects(layers.unwrap_or_default()) {
            return None;
        }
        let affine = transform.affine();
        let center = aabb.map_or(affine.translation, |a| affine.transform_point3a(a.center));
        if let Some(range) = range {
            let at = if range.use_aabb { center } else { affine.translation };
            if !range.is_visible_at_all((at - lamp.pos).length()) {
                return None;
            }
        }
        let sphere = Sphere { center: lamp.pos, radius: lamp.reach };
        let faces = if unculled { ALL_FACES } else { crate::gpu_culled_casters::caster_faces(&sphere, &lamp.frusta, aabb, transform) };
        if faces == 0 {
            return None;
        }
        let mobility = self.mobility(entity, skinned, morphed, no_cpu);
        let last_move_s = if settled { f32::NEG_INFINITY } else { now_s };
        let class = config.classify(mobility, now_s - last_move_s);
        Some(Caster {
            rel: relative(&affine, lamp.pos),
            aabb: aabb.copied(),
            faces,
            center,
            mobility,
            class,
            last_move_s,
            baked: 0,
            batch: false,
            no_cpu_culling: no_cpu,
        })
    }

    /// Every caster of `lamp` (a full scan): meshes, then GPU instance batches (static, every face).
    fn scan(&self, lamp: &Lamp, now_s: f32, config: &LampCacheConfig) -> EntityHashMap<Caster> {
        let mut out = EntityHashMap::default();
        let sphere = Sphere { center: lamp.pos, radius: lamp.reach };
        for (entity, transform, aabb, shown, layers, (unculled, ..), _) in &self.meshes {
            // Cheap rejections first; `caster` repeats them with the rest of the tests.
            if !shown.get() || !lamp.layers.intersects(layers.unwrap_or_default()) {
                continue;
            }
            if !unculled && aabb.is_some_and(|a| !sphere.intersects_obb(a, &transform.affine())) {
                continue;
            }
            if let Some(c) = self.caster(entity, lamp, now_s, true, config) {
                out.insert(entity, c);
            }
        }
        for (entity, shown, layers) in &self.batches {
            if shown.get() && lamp.layers.intersects(layers.unwrap_or_default()) {
                out.insert(entity, batch_caster(lamp.pos));
            }
        }
        out
    }
}

fn batch_caster(lamp: Vec3A) -> Caster {
    Caster {
        rel: Affine3A::IDENTITY,
        aabb: None,
        faces: ALL_FACES,
        center: lamp,
        mobility: CasterMobility::Static,
        class: CasterClass::Static,
        last_move_s: f32::NEG_INFINITY,
        baked: 0,
        batch: true,
        no_cpu_culling: true,
    }
}

fn relative(affine: &Affine3A, lamp: Vec3A) -> Affine3A {
    let mut rel = *affine;
    rel.translation -= lamp;
    rel
}

/// A lamp's six face frusta at vertical/horizontal `fov_y` (PI/2 for cube array, widened by
/// guard band for the 2D shadow atlas).
fn cube_frusta_fov(transform: &GlobalTransform, near_z: f32, range: f32, fov_y: f32) -> CubemapFrusta {
    let clip_from_view = proj::perspective_infinite_reverse(fov_y, 1.0, near_z);
    let view_translation = Transform::from_translation(transform.translation());
    let mut frusta = CubemapFrusta::default();
    for (CubeMapFace { target, up }, frustum) in CUBE_MAP_FACES.iter().zip(frusta.frusta.iter_mut()) {
        let world_from_view = view_translation * Transform::IDENTITY.looking_at(*target, *up);
        let clip_from_world = clip_from_view * world_from_view.compute_affine().inverse();
        *frustum = Frustum(ViewFrustum::from_clip_from_world_custom_far(
            &clip_from_world,
            &transform.translation(),
            &transform.back(),
            range,
        ));
    }
    frusta
}

/// The reach a cached lamp's lists cover: the shadowed one (the budget blends the range).
fn shadowed_reach(light: &PointLight, reach: Option<&ShadowReach>) -> f32 {
    reach.map_or(light.range, |r| r.shadowed)
}

pub(crate) fn add(app: &mut App) {
    app.register_tunables(policy::TUNABLES);
    bind(app, &policy::ENABLED, |c, v| c.enabled = v);
    bind(app, &policy::CAPTURE_FACES_PER_FRAME, |c, v| c.config.capture_faces_per_frame = v);
    bind(app, &policy::RECAPTURE_INTERVAL, |c, v| c.config.recapture_interval_s = v);
    bind(app, &policy::SETTLE_TIME, |c, v| c.config.settle_time_s = v);
    bind(app, &policy::STORE_FACES_PER_LAMP, |c, v| c.config.store_faces_per_lamp = v);
    bind(app, &policy::MOVER_FULL_RATE, |c, v| c.config.mover_full_rate_m = v);
    bind(app, &policy::MOVER_HALF_RATE, |c, v| c.config.mover_half_rate_m = v);
    bind(app, &policy::MOVER_FAR_INTERVAL, |c, v| c.config.mover_far_interval_frames = v);
    bind(app, &policy::MAX_MOVER_FACES_PER_FRAME, |c, v| c.config.max_mover_faces_per_frame = v);
    bind(app, &policy::MOVER_LINGER, |c, v| c.config.mover_linger_s = v);
    bind(app, &policy::STATIC_TIER_SLOTS, |c, v| c.config.static_tier_slots = v);
    bind(app, &policy::ANIMATED_CASTERS_MOVE, |c, v| c.config.animated_casters_move = v);
    app.init_resource::<LampShadowCache>()
        .init_resource::<LampCacheState>()
        .init_resource::<LampCacheReports>()
        .init_resource::<LampCacheStats>()
        .init_resource::<PointShadowCacheStore>()
        .init_resource::<PointShadowCacheFeedback>()
        .add_systems(
            PostUpdate,
            (
                admit_lamps.after(budget_shadows).before(VisibilitySystems::VisibilityPropagate),
                track_lamps
                    .after(admit_lamps)
                    .after(SimulationLightSystems::CheckLightVisibility)
                    .after(GpuBatchedLightVisibility)
                    .after(crate::gpu_culled_casters::list_gpu_culled_casters)
                    .before(VisibilitySystems::MarkNewlyHiddenEntitiesInvisible),
            ),
        );
}

/// The render world's counts as gauges (`lamp_cache/render/*`), when the app has counters.
pub(crate) fn finish(app: &mut App) {
    let Some(counters) = app.world().get_resource::<DiagCounters>().cloned() else {
        return;
    };
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        if !render_app.world().contains_resource::<DiagCounters>() {
            render_app.insert_resource(counters);
        }
        render_app.add_systems(Render, render_gauges.in_set(RenderSystems::Cleanup));
    }
}

fn bind<T: sf_tunables::Type + Sync + Send>(
    app: &mut App,
    def: &sf_tunables::Def<T>,
    apply: impl for<'v> Fn(&mut LampShadowCache, T::Read<'v>) + Send + Sync + 'static,
) {
    let _bound = app.add_tunable(def).bind_resource(apply);
}

fn render_gauges(stats: Option<Res<PointShadowCacheStats>>, counters: Res<DiagCounters>, mut active: Local<bool>) {
    let Some(s) = stats else { return };
    let idle = *s == PointShadowCacheStats::default();
    // Nothing cached: the counters stay silent on the stock path (zeroed once when it goes idle).
    if idle && !*active {
        return;
    }
    *active = !idle;
    for (name, value) in [
        ("lamps", s.lamps),
        ("views", s.views),
        ("drawn", s.drawn),
        ("loaded", s.loaded),
        ("copies", s.copies),
        ("dropped_copies", s.dropped_copies),
        ("lost", s.lost),
    ] {
        counters.set_named(&format!("lamp_cache/render/{name}"), sf_diag_bevy::sf_diag::unit::COUNT, f64::from(value));
    }
}

type LampItem<'a> = (
    Entity,
    &'a mut PointLight,
    &'a GlobalTransform,
    &'a BudgetedShadow,
    Option<&'a RenderLayers>,
    Option<&'a ShadowReach>,
    Option<&'a mut ShadowSlot>,
    Option<&'a mut PointLightShadowFade>,
);

/// Who is cached: admits still slot holders, releases lamps that lost their slot or moved, and
/// tears everything down when the switch goes off. Runs after the budget, before Bevy's caster
/// scan, so a lamp changes path within the frame. Supports both CW-4 (`CachedPointShadowMap`) and
/// CW-13b (`AtlasPointShadowMap`, taking precedence when `shadow.lamp.atlas.enabled` is on).
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value, clippy::type_complexity)]
#[expect(clippy::too_many_lines, reason = "one pass over the budgeted lamps: mode switch, policy sizes, moving lamps, admissions and releases")]
fn admit_lamps(
    mut commands: Commands,
    (settings, atlas_settings, budget, map): (
        Res<LampShadowCache>,
        Option<Res<LampShadowAtlas>>,
        Res<PointShadowBudget>,
        Option<Res<PointLightShadowMap>>,
    ),
    time: Res<Time>,
    (mut state, mut store): (ResMut<LampCacheState>, ResMut<PointShadowCacheStore>),
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut lamps: Query<LampItem>,
    casters: Casters,
) {
    let state = &mut *state;
    let now = time.elapsed_secs();
    let atlas_cfg = atlas_settings.as_deref().filter(|a| a.enabled).map(|a| a.config);
    let atlas_on = atlas_cfg.is_some();
    let active = atlas_on || settings.enabled;

    // Tear down if disabled or switching live between CW-4 cube cache and CW-13b 2D atlas.
    if !active || state.atlas_mode != atlas_on {
        if state.cache.is_some() || !state.lamps.is_empty() || !state.moving_atlas.is_empty() {
            for (entity, mut light, .., slot, _) in &mut lamps {
                if state.lamps.remove(&entity).is_some() || state.moving_atlas.remove(&entity).is_some() {
                    release(&mut commands, entity, &mut light, slot.as_deref());
                }
            }
            *state = LampCacheState { scans: state.scans, atlas_mode: atlas_on, ..LampCacheState::default() };
        } else {
            state.atlas_mode = atlas_on;
        }
        if !active {
            store.set_if_neq(PointShadowCacheStore { layers: 0 });
            return;
        }
    }

    let eyes: Vec<Vec3A> = cameras.iter().filter(|(c, _)| c.is_active).map(|(_, t)| t.translation_vec3a()).collect();
    let dist_to_eye = |pos: Vec3A| {
        eyes.iter().map(|eye| (*eye - pos).length()).fold(f32::INFINITY, f32::min).min(1e6)
    };

    // Configure policy & allocator for either Atlas or CW-4 mode.
    let (max, policy_cfg) = if let Some(acfg) = atlas_cfg {
        let m = acfg.max_lamps.saturating_add(acfg.handover_spares);
        let pcfg = LampCacheConfig {
            store_faces_per_lamp: ATLAS_POLICY_STORE_FACES,
            capture_faces_per_frame: settings.config.capture_faces_per_frame.max(ATLAS_MIN_CAPTURES_PER_FRAME),
            ..settings.config
        };
        (m, pcfg)
    } else {
        (u32::try_from(budget.max).unwrap_or(u32::MAX), settings.config)
    };

    let switched_on = state.cache.is_none();
    let cache = state.cache.get_or_insert_with(|| LampCache::new(TileLayout::CUBE, max, policy_cfg));
    if cache.config() != &policy_cfg || state.max_lamps != max {
        cache.reconfigure(max, policy_cfg);
        state.max_lamps = max;
    }

    if let Some(acfg) = atlas_cfg {
        let mgr = state.atlas.get_or_insert_with(|| ShadowAtlasManager::new(acfg));
        if mgr.reconfigure(acfg) {
            // Atlas dimensions changed: release all lamps so they re-admit cleanly into new textures.
            for (entity, mut light, .., slot, _) in &mut lamps {
                if state.lamps.remove(&entity).is_some() || state.moving_atlas.remove(&entity).is_some() {
                    release(&mut commands, entity, &mut light, slot.as_deref());
                }
            }
            *cache = LampCache::new(TileLayout::CUBE, max, policy_cfg);
            state.static_full_at_gen = None;
        }
        store.set_if_neq(PointShadowCacheStore { layers: 0 });
    } else {
        let map_size = map.map_or(0, |m| m.size);
        if state.map_size != map_size {
            if state.map_size != 0 {
                cache.recapture_all();
            }
            state.map_size = map_size;
        }
        let layers = u32::try_from(cache.pool().len()).unwrap_or(u32::MAX);
        store.set_if_neq(PointShadowCacheStore { layers });
    }

    // Moving lamps: still again once nothing moved against their reference for the settle time.
    let settle = settings.config.settle_time_s;
    state.moving.retain(|entity, moving| {
        let Ok((_, _, at, ..)) = lamps.get(*entity) else { return false };
        if let Some((reference, rel)) = moving.reference {
            match casters.meshes.get(reference) {
                Ok((_, t, ..)) => {
                    let now_rel = t.translation_vec3a() - at.translation_vec3a();
                    if (now_rel - rel).length() > STILL_EPSILON {
                        moving.still_since = now;
                    }
                    moving.reference = Some((reference, now_rel));
                }
                Err(_) => moving.reference = None,
            }
        }
        let still_moving = now - moving.still_since < settle;
        if !still_moving
            && let Some(m) = state.moving_atlas.remove(entity)
            && let Some(mgr) = state.atlas.as_mut()
        {
            mgr.remove(m.key);
        }
        still_moving
    });

    let mut seen = EntityHashSet::default();
    let mut seen_moving = EntityHashSet::default();
    let mut stock_drawn = EntityHashSet::default();
    for (entity, mut light, at, prio, light_layers, reach, mut slot, fade) in &mut lamps {
        let held = slot.as_ref().is_some_and(|s| s.held);
        if let Some(lamp) = state.lamps.get(&entity) {
            seen.insert(entity);
            if held && !lamp.moved {
                if light.shadow_maps_enabled {
                    light.shadow_maps_enabled = false;
                }
                continue;
            }
            let lamp = state.lamps.remove(&entity).expect("checked");
            let cache = state.cache.as_mut().expect("enabled");
            if let Some(mgr) = state.atlas.as_mut() {
                mgr.remove(lamp.key);
            }
            if lamp.moved {
                let _ = cache.invalidate(lamp.key, Invalidation::Moved);
                state.moving.insert(entity, MovingLamp { reference: lamp.reference(), still_since: now });
                // Under 2D atlas, immediately promote a held moving lamp (carried torch) into the Live Atlas.
                if held && let (Some(acfg), Some(mgr)) = (atlas_cfg, state.atlas.as_mut()) {
                    let key = LampKey(entity.to_bits());
                    let desired = pick_tile_size(dist_to_eye(at.translation_vec3a()), prio.priority, None, f32::INFINITY, &acfg)
                        .max(ATLAS_MOVING_TILE_PX.min(512));
                    if let Some(tile_size) = mgr.admit_moving(key, desired, now) {
                        if light.shadow_maps_enabled {
                            light.shadow_maps_enabled = false;
                        }
                        commands.entity(entity).try_insert((AtlasPointShadowMap::default(), ShadowMapCached { ready: true }));
                        state.moving_atlas.insert(entity, MovingAtlasLamp {
                            key,
                            tile_size,
                            layers: light_layers.cloned().unwrap_or_default(),
                        });
                        seen_moving.insert(entity);
                        continue;
                    }
                }
            } else {
                let _ = cache.invalidate(lamp.key, Invalidation::SlotLost);
            }
            release(&mut commands, entity, &mut light, slot.as_deref());
        } else if let Some(moving_lamp) = state.moving_atlas.get(&entity) {
            if held && state.moving.contains_key(&entity) {
                seen_moving.insert(entity);
                if light.shadow_maps_enabled {
                    light.shadow_maps_enabled = false;
                }
                continue;
            }
            let key = moving_lamp.key;
            state.moving_atlas.remove(&entity);
            if let Some(mgr) = state.atlas.as_mut() {
                mgr.remove(key);
            }
            release(&mut commands, entity, &mut light, slot.as_deref());
        } else if held && state.moving.contains_key(&entity) {
            // Still moving under 2D atlas: keep drawing in the Live Atlas.
            if let (Some(acfg), Some(mgr)) = (atlas_cfg, state.atlas.as_mut()) {
                let key = LampKey(entity.to_bits());
                let desired = pick_tile_size(dist_to_eye(at.translation_vec3a()), prio.priority, None, f32::INFINITY, &acfg)
                    .max(ATLAS_MOVING_TILE_PX.min(512));
                if let Some(tile_size) = mgr.admit_moving(key, desired, now) {
                    if light.shadow_maps_enabled {
                        light.shadow_maps_enabled = false;
                    }
                    commands.entity(entity).try_insert((AtlasPointShadowMap::default(), ShadowMapCached { ready: true }));
                    state.moving_atlas.insert(entity, MovingAtlasLamp {
                        key,
                        tile_size,
                        layers: light_layers.cloned().unwrap_or_default(),
                    });
                    seen_moving.insert(entity);
                    continue;
                }
            }
        } else if held {
            let handover = atlas_cfg.is_some_and(|acfg| {
                state.lamps.len() + state.moving_atlas.len() >= acfg.max_lamps as usize
            });
            if let Some(mgr) = state.atlas.as_ref()
                && let Some((full_gen, was_handover)) = state.static_full_at_gen
                && full_gen == mgr.static_free_gen()
                && (!handover || was_handover)
            {
                continue;
            }
            let reach = shadowed_reach(&light, reach);
            let pos = at.translation_vec3a();
            let (desired_tile, fov_y) = if let Some(acfg) = atlas_cfg {
                let t = pick_tile_size(dist_to_eye(pos), prio.priority, None, f32::INFINITY, &acfg);
                (t, guard_band_fov_y(u32::from(t), acfg.guard_texels))
            } else {
                (0, core::f32::consts::FRAC_PI_2)
            };
            let mut lamp = Lamp {
                key: LampKey(entity.to_bits()),
                pos,
                reach,
                frusta: cube_frusta_fov(at, light.shadow_map_near_z, reach, fov_y),
                layers: light_layers.cloned().unwrap_or_default(),
                casters: EntityHashMap::default(),
                moved: false,
                captures: Vec::new(),
                tile_size: desired_tile,
                priority: prio.priority,
                active_mover_faces: 0,
                atlas_written: AtlasWritten::default(),
            };
            lamp.casters = casters.scan(&lamp, now, &settings.config);
            lamp.bake_statics();
            state.scans += 1;

            if let (Some(acfg), Some(mgr)) = (atlas_cfg, state.atlas.as_mut()) {
                let Some(granted) = mgr.admit_static(lamp.key, lamp.statics().0, desired_tile, now, handover) else {
                    state.static_full_at_gen = Some((mgr.static_free_gen(), handover));
                    continue;
                };
                if granted != desired_tile {
                    lamp.tile_size = granted;
                    let new_fov = guard_band_fov_y(u32::from(granted), acfg.guard_texels);
                    lamp.frusta = cube_frusta_fov(at, light.shadow_map_near_z, reach, new_fov);
                }
            }

            let cache = state.cache.as_mut().expect("enabled");
            if cache.admit(lamp.key, LampTier::Full, lamp.statics()).is_err() {
                if let Some(mgr) = state.atlas.as_mut() {
                    mgr.remove(lamp.key);
                }
                if !atlas_on && light.shadow_maps_enabled {
                    stock_drawn.insert(entity);
                }
                continue;
            }
            seen.insert(entity);
            let gained = slot.as_ref().is_none_or(DetectChanges::is_added);
            let drawn = !atlas_on
                && (state.stock_drawn.contains(&entity) || (switched_on && light.shadow_maps_enabled && !gained));
            if light.shadow_maps_enabled {
                light.shadow_maps_enabled = false;
            }
            if !drawn
                && let Some(slot) = slot.as_deref_mut()
            {
                slot.fade = 0.0;
                if let Some(mut fade) = fade {
                    fade.0 = 0.0;
                }
            }
            if atlas_on {
                commands.entity(entity).try_insert((AtlasPointShadowMap::default(), ShadowMapCached::default()));
            } else {
                commands.entity(entity).try_insert((CachedPointShadowMap::default(), ShadowMapCached::default()));
            }
            state.lamps.insert(entity, lamp);
            continue;
        }
        if !atlas_on && light.shadow_maps_enabled {
            stock_drawn.insert(entity);
        }
    }
    // Despawned lamps.
    let cache = state.cache.as_mut().expect("enabled");
    let mut atlas_mgr = state.atlas.as_mut();
    state.lamps.retain(|entity, lamp| {
        let keep = seen.contains(entity);
        if !keep {
            let _ = cache.invalidate(lamp.key, Invalidation::Despawned);
            if let Some(mgr) = atlas_mgr.as_deref_mut() {
                mgr.remove(lamp.key);
            }
        }
        keep
    });
    if let Some(mgr) = state.atlas.as_mut() {
        state.moving_atlas.retain(|entity, moving| {
            let keep = seen_moving.contains(entity);
            if !keep {
                mgr.remove(moving.key);
            }
            keep
        });
    }
    state.stock_drawn = stock_drawn;
}

/// Hands a lamp back to the stock path: its components go, its shadow flag is the budget's.
fn release(commands: &mut Commands, entity: Entity, light: &mut Mut<PointLight>, slot: Option<&ShadowSlot>) {
    commands.entity(entity).try_remove::<(CachedPointShadowMap, AtlasPointShadowMap, ShadowMapCached)>();
    let stock = slot.is_some_and(|s| s.held && s.fade > 0.0);
    if light.shadow_maps_enabled != stock {
        light.shadow_maps_enabled = stock;
    }
}

type CachedLampItem<'a> = (
    &'a GlobalTransform,
    &'a PointLight,
    Option<&'a ShadowReach>,
    Option<&'a mut CachedPointShadowMap>,
    Option<&'a mut AtlasPointShadowMap>,
    &'a mut CubemapVisibleEntities,
    &'a mut ShadowMapCached,
    (Option<&'a mut ShadowSlot>, Option<&'a mut PointLightShadowFade>),
);

/// The cached lamps' frame: feedback, lamp and caster changes, movers, the plan, the components.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value, clippy::type_complexity)]
#[expect(clippy::too_many_lines, reason = "one pass per cached lamp: feedback, lamp and caster changes, movers, then the plan")]
fn track_lamps(
    (settings, atlas_settings): (Res<LampShadowCache>, Option<Res<LampShadowAtlas>>),
    time: Res<Time>,
    (mut state, mut gauges, mut atlas_gauges): (
        ResMut<LampCacheState>,
        ResMut<LampCacheStats>,
        Option<ResMut<LampShadowAtlasStats>>,
    ),
    (feedback, mut reports, atlas_feedback, mut atlas_reports, face_cull, counters): (
        Res<PointShadowCacheFeedback>,
        ResMut<LampCacheReports>,
        Option<Res<PointShadowAtlasFeedback>>,
        Option<ResMut<LampShadowAtlasReports>>,
        Option<Res<PointShadowFaceCull>>,
        Option<Res<DiagCounters>>,
    ),
    cameras: Query<(&Camera, &GlobalTransform, Option<&Frustum>), With<Camera3d>>,
    mut lamps: Query<CachedLampItem>,
    casters: Casters,
    (changed, mut removed, mut removed_batches, shadowless): (
        Query<Entity, (IsCaster, CasterChanged)>,
        RemovedComponents<Mesh3d>,
        RemovedComponents<GpuBatchedMesh3d>,
        Query<Entity, (With<Mesh3d>, Added<NotShadowCaster>)>,
    ),
    mut visibility: Query<&mut ViewVisibility>,
) {
    let state = &mut *state;
    let removed: EntityHashSet = removed.read().chain(removed_batches.read()).chain(shadowless.iter()).collect();
    let reports: Vec<(Entity, CachedPointShadowReport)> = feedback.drain().into_iter().chain(reports.0.drain(..)).collect();
    let atlas_reports_vec: Vec<(Entity, AtlasPointShadowReport)> = atlas_feedback
        .as_deref()
        .map(PointShadowAtlasFeedback::drain)
        .unwrap_or_default()
        .into_iter()
        .chain(atlas_reports.as_deref_mut().map(|r| std::mem::take(&mut r.0)).unwrap_or_default())
        .collect();

    let atlas_cfg = atlas_settings.as_deref().filter(|a| a.enabled).map(|a| a.config);
    let atlas_on = atlas_cfg.is_some();
    let active = atlas_on || settings.enabled;
    let Some(cache) = state.cache.as_mut().filter(|_| active) else {
        let idle = LampCacheStats { scans: state.scans, ..LampCacheStats::default() };
        if *gauges != idle {
            if let Some(counters) = counters.as_deref() {
                publish(counters, &idle);
            }
            *gauges = idle;
        }
        if let Some(ag) = atlas_gauges.as_deref_mut() {
            let idle_atlas = LampShadowAtlasStats { scans: state.scans, ..LampShadowAtlasStats::default() };
            if *ag != idle_atlas {
                if let Some(counters) = counters.as_deref() {
                    publish_atlas_gauges(counters, &idle_atlas);
                }
                *ag = idle_atlas;
            }
        }
        return;
    };
    let now = time.elapsed_secs();
    let config = settings.config;

    // Render-world feedback: confirmed captures, lost slots / atlas reallocations.
    if atlas_on {
        for (entity, report) in atlas_reports_vec {
            let (Some(lamp), Ok((.., (slot, fade)))) = (state.lamps.get_mut(&entity), lamps.get_mut(entity)) else {
                continue;
            };
            if report.lost {
                let _ = cache.invalidate(lamp.key, Invalidation::SlotLost);
                let _ = cache.admit(lamp.key, LampTier::Full, lamp.statics());
                lamp.captures.clear();
                lamp.bake_statics();
                // New atlas textures hold nothing of it: every rect unwritten until drawn again.
                lamp.atlas_written = AtlasWritten::default();
                if let Some(mut slot) = slot {
                    slot.fade = 0.0;
                }
                if let Some(mut fade) = fade {
                    fade.0 = 0.0;
                }
                continue;
            }
            let captured = lamp.captures.iter().find(|(plan, _)| *plan == report.plan).map_or(0, |(_, mask)| *mask);
            for face in TileMask(report.confirmed & captured).iter() {
                let _ = cache.confirm_capture(lamp.key, face);
            }
        }
    } else {
        for (entity, report) in reports {
            let (Some(lamp), Ok((.., (slot, fade)))) = (state.lamps.get_mut(&entity), lamps.get_mut(entity)) else {
                continue;
            };
            if report.lost {
                let _ = cache.invalidate(lamp.key, Invalidation::SlotLost);
                let _ = cache.admit(lamp.key, LampTier::Full, lamp.statics());
                lamp.captures.clear();
                lamp.bake_statics();
                // Its live layers hold nothing of it any more: no shadow until captured again.
                if let Some(mut slot) = slot {
                    slot.fade = 0.0;
                }
                if let Some(mut fade) = fade {
                    fade.0 = 0.0;
                }
                continue;
            }
            let captured = lamp.captures.iter().find(|(plan, _)| *plan == report.plan).map_or(0, |(_, mask)| *mask);
            for face in TileMask(report.confirmed & captured).iter() {
                let _ = cache.confirm_capture(lamp.key, face);
            }
        }
    }

    // Camera frusta (SH-21b, CW-3b) and positions (mover distances & atlas tier selection).
    let mut frusta: Vec<Frustum> = Vec::new();
    let mut eyes: Vec<Vec3A> = Vec::new();
    for (camera, at, frustum) in &cameras {
        if camera.is_active {
            eyes.push(at.translation_vec3a());
            frusta.extend(frustum.copied());
        }
    }
    let view_frusta: Vec<ViewFrustum> = frusta.iter().map(|f| f.0).collect();
    let margin = face_cull.map_or(PointShadowFaceCull::default().margin_deg, |c| c.margin_deg);

    // Precompute bounding spheres for changed casters ONCE outside the lamp loop so scaling
    // from 8 -> 48+ lamps avoids O(lamps x changed_casters) ECS lookups.
    let changed_spheres: Vec<(Entity, Vec3A, f32, bool)> = changed
        .iter()
        .map(|e| {
            if let Ok((_, transform, aabb, _, _, (unculled, ..), _)) = casters.meshes.get(e) {
                if unculled {
                    (e, Vec3A::ZERO, f32::INFINITY, true)
                } else {
                    let affine = transform.affine();
                    let center = aabb.map_or(affine.translation, |a| affine.transform_point3a(a.center));
                    let radius = aabb.map_or(f32::INFINITY, |a| {
                        let s = affine.matrix3.x_axis.length_squared()
                            .max(affine.matrix3.y_axis.length_squared())
                            .max(affine.matrix3.z_axis.length_squared())
                            .sqrt();
                        a.half_extents.length() * s
                    });
                    (e, center, radius, false)
                }
            } else {
                (e, Vec3A::ZERO, f32::INFINITY, true)
            }
        })
        .collect();

    let log_movers = log::log_enabled!(log::Level::Debug) && now - state.last_mover_log_s >= 1.0;
    let mut unique_movers = EntityHashSet::default();
    let (mut movable_pairs, mut rigid_pairs, mut animated_pairs, mut unbaked_pairs) = (0u32, 0u32, 0u32, 0u32);
    let mut movers_total = 0;
    for (&entity, lamp) in &mut state.lamps {
        let Ok((at, light, reach, ..)) = lamps.get(entity) else { continue };
        if lamp.moved {
            continue;
        }
        // The lamp: position (origin shifts move everything alike), reach, frusta.
        let pos = at.translation_vec3a();
        let reach = shadowed_reach(light, reach);
        let fov_y = if let Some(acfg) = atlas_cfg {
            guard_band_fov_y(u32::from(lamp.tile_size.max(128)), acfg.guard_texels)
        } else {
            core::f32::consts::FRAC_PI_2
        };
        let mut rescan = false;
        if (pos - lamp.pos).length() > STILL_EPSILON * 0.01 || (reach - lamp.reach).abs() > STILL_EPSILON {
            if lamp_moved(lamp, pos, &casters) {
                lamp.moved = true;
                let _ = cache.invalidate(lamp.key, Invalidation::Moved);
                continue;
            }
            rescan = lamp.casters.values().all(|c| c.class != CasterClass::Static) || (reach - lamp.reach).abs() > STILL_EPSILON;
            lamp.pos = pos;
            lamp.reach = reach;
            lamp.frusta = cube_frusta_fov(at, light.shadow_map_near_z, reach, fov_y);
        }

        let mut touched = 0u8;
        let statics_before = lamp.statics();
        if rescan {
            let fresh = casters.scan(lamp, now, &config);
            state.scans += 1;
            for (e, c) in fresh {
                lamp.casters.entry(e).or_insert_with(|| {
                    touched |= c.faces;
                    c
                });
            }
        }
        for e in &removed {
            if let Some(c) = lamp.casters.remove(e) {
                touched |= c.baked;
            }
        }
        for &(e, center, radius, unculled) in &changed_spheres {
            if !unculled && !lamp.casters.contains_key(&e) && radius.is_finite() {
                let max_d = lamp.reach + radius;
                if (center - lamp.pos).length_squared() > max_d * max_d {
                    continue;
                }
            }
            touched |= update_caster(lamp, e, now, &config, &casters);
        }
        for (e, shown, _) in &casters.batches {
            if shown.is_changed() && !shown.is_added() {
                touched |= ALL_FACES;
                if !shown.get() {
                    lamp.casters.remove(&e);
                }
            } else if shown.is_added() && shown.get() {
                lamp.casters.insert(e, batch_caster(lamp.pos));
                touched |= ALL_FACES;
            }
        }
        // Rigid movers that have been still long enough settle into the statics.
        for c in lamp.casters.values_mut() {
            if c.class == CasterClass::Mover && config.classify(c.mobility, now - c.last_move_s) == CasterClass::Static {
                c.class = CasterClass::Static;
                touched |= c.faces;
            }
        }
        let statics = lamp.statics();

        // Under 2D atlas: sync empty/newly populated static faces and check dynamic tier resize.
        if let (Some(acfg), Some(mgr)) = (atlas_cfg, state.atlas.as_mut()) {
            if statics != statics_before {
                touched |= mgr.sync_static_faces(lamp.key, statics.0);
            }
            let dist = eyes.iter().map(|eye| (*eye - lamp.pos).length()).fold(f32::INFINITY, f32::min);
            if dist.is_finite()
                && let Some((new_tile, alloc_mask)) = mgr.maybe_resize(lamp.key, dist, lamp.priority, now)
            {
                lamp.tile_size = new_tile;
                let new_fov = guard_band_fov_y(u32::from(new_tile), acfg.guard_texels);
                lamp.frusta = cube_frusta_fov(at, light.shadow_map_near_z, reach, new_fov);
                for c in lamp.casters.values_mut() {
                    c.baked &= !alloc_mask;
                }
                touched |= alloc_mask;
            }
        }

        if touched != 0 || statics != statics_before {
            let _ = cache.invalidate(lamp.key, Invalidation::StaticsChanged { touched: TileMask(touched), statics });
        }

        // Movers, kept to what a camera can see.
        let mut faces = 0u8;
        let mut distance = f32::INFINITY;
        for (&caster_entity, c) in &lamp.casters {
            let f = c.mover_faces();
            if f != 0 {
                faces |= f;
                movers_total += 1;
                if log_movers {
                    unique_movers.insert(caster_entity);
                    match (c.class, c.mobility) {
                        (CasterClass::Mover, CasterMobility::Movable) => movable_pairs += 1,
                        (CasterClass::Mover, CasterMobility::Rigid) => rigid_pairs += 1,
                        (CasterClass::Mover, CasterMobility::AnimatedStatic) => animated_pairs += 1,
                        (CasterClass::Static, _) | (CasterClass::Mover, CasterMobility::Static) => unbaked_pairs += 1,
                    }
                }
                distance = distance.min(eyes.iter().map(|eye| (*eye - c.center).length()).fold(f32::INFINITY, f32::min));
            }
        }
        let sphere = Sphere { center: lamp.pos, radius: lamp.reach };
        let in_view = frusta.is_empty() || frusta.iter().any(|f| f.intersects_sphere(&sphere, true));
        if faces != 0 && in_view {
            faces &= point_shadow_faces_seen(Vec3::from(lamp.pos), lamp.reach, margin, &view_frusta);
        }
        lamp.active_mover_faces = if in_view { faces } else { 0 };
        if faces != 0 && in_view {
            let _ = cache.set_movers(lamp.key, MoverInput { tiles: TileMask(faces), distance_m: distance });
        }
    }

    let plan = cache.plan(now).clone();
    if log_movers {
        state.last_mover_log_s = now;
        log::debug!(
            "lamp_cache movers: unique={} pairs={} (movable={} rigid={} animated={} unbaked={}) drawn_mover_faces={} clean_faces={} lamps={}",
            unique_movers.len(),
            movers_total,
            movable_pairs,
            rigid_pairs,
            animated_pairs,
            unbaked_pairs,
            plan.stats.mover_draws,
            plan.stats.clean_faces,
            plan.stats.lamps,
        );
    }

    if let Some(acfg) = atlas_cfg {
        let mut views = write_atlas_plan(&plan, state, &mut lamps, &mut visibility);
        views += update_moving_atlas_lamps(
            state,
            acfg.guard_texels,
            margin,
            &frusta,
            &view_frusta,
            &casters,
            &mut lamps,
            &mut visibility,
        );
        if let (Some(mgr), Some(ag)) = (state.atlas.as_ref(), atlas_gauges.as_deref_mut()) {
            *ag = LampShadowAtlasStats {
                atlas: mgr.stats(),
                views,
                moving_lamps: u32::try_from(state.moving_atlas.len()).unwrap_or(u32::MAX),
                movers: movers_total,
                captures: plan.stats.captures,
                mover_draws: plan.stats.mover_draws,
                scans: state.scans,
            };
            if let Some(counters) = counters.as_deref() {
                publish_atlas_gauges(counters, ag);
            }
        }
    } else {
        let views = write_plan(&plan, state, &mut lamps, &mut visibility);
        let cache = state.cache.as_ref().expect("enabled");
        *gauges = LampCacheStats {
            frame: plan.stats,
            views,
            stock_lamps: u32::try_from(state.stock_drawn.len()).unwrap_or(u32::MAX),
            movers: movers_total,
            scans: state.scans,
            store_used: u32::try_from(cache.pool().in_use()).unwrap_or(u32::MAX),
            store_layers: u32::try_from(cache.pool().len()).unwrap_or(u32::MAX),
        };
        if let Some(counters) = counters.as_deref() {
            publish(counters, &gauges);
        }
    }
}

/// Whether a lamp whose position changed moved against its statics (more than half of them
/// shifted relative to it; an origin shift moves them all alike). A lamp without statics has
/// nothing to move against.
fn lamp_moved(lamp: &Lamp, pos: Vec3A, casters: &Casters) -> bool {
    let (mut checked, mut shifted) = (0usize, 0usize);
    for (e, c) in &lamp.casters {
        if c.class != CasterClass::Static || c.batch {
            continue;
        }
        let Ok((_, t, ..)) = casters.meshes.get(*e) else { continue };
        checked += 1;
        let rel = t.translation_vec3a() - pos;
        if (rel - c.rel.translation).length() > STILL_EPSILON {
            shifted += 1;
        }
    }
    checked > 0 && shifted * 2 > checked
}

/// A caster `entity` of the world changed: its record in `lamp` is created, updated or dropped.
/// Returns the faces whose capture it invalidates.
fn update_caster(lamp: &mut Lamp, entity: Entity, now: f32, config: &LampCacheConfig, casters: &Casters) -> u8 {
    let current = casters.caster(entity, lamp, now, false, config);
    match (lamp.casters.get_mut(&entity), current) {
        (Some(old), None) => {
            let baked = old.baked;
            lamp.casters.remove(&entity);
            baked
        }
        (Some(old), Some(new)) => {
            let moved = !old.rel.abs_diff_eq(new.rel, STILL_EPSILON) || old.aabb != new.aabb;
            old.center = new.center;
            if !moved {
                // An origin shift or a no-op write: nothing moved relative to the lamp.
                return 0;
            }
            let mut touched = old.baked;
            old.rel = new.rel;
            old.aabb = new.aabb;
            old.faces = new.faces;
            old.last_move_s = now;
            old.baked = 0;
            old.class = config.classify(old.mobility, 0.0);
            if old.class == CasterClass::Static {
                // A caster that never moves was placed again: its new faces recapture.
                touched |= old.faces;
            }
            touched
        }
        (None, Some(new)) => {
            let touched = if new.class == CasterClass::Static { new.faces } else { 0 };
            lamp.casters.insert(entity, new);
            touched
        }
        (None, None) => 0,
    }
}

/// Writes each cached lamp's component, lists and readiness from `plan`; returns the face views
/// kept.
fn write_plan(
    plan: &FramePlan,
    state: &mut LampCacheState,
    lamps: &mut Query<CachedLampItem>,
    visibility: &mut Query<&mut ViewVisibility>,
) -> u32 {
    let mut views = 0;
    for (&entity, lamp) in &mut state.lamps {
        let Some(lamp_plan) = plan.lamp(lamp.key) else { continue };
        let Ok((_, _, _, Some(mut cached), _, mut lists, mut marker, _)) = lamps.get_mut(entity) else { continue };
        let capture = lamp_plan.capture.0;
        let movers = lamp_plan.movers.0;
        if capture != 0 {
            lamp.captures.push((plan.frame, capture));
            if lamp.captures.len() > CAPTURE_HISTORY {
                lamp.captures.remove(0);
            }
        }
        // Per drawn face: the statics (a capture) or the movers, sorted like Bevy's lists.
        for face in 0..6u8 {
            let bit = 1 << face;
            let list: Vec<Entity> = if capture & bit != 0 {
                let mut list: Vec<Entity> = lamp
                    .casters
                    .iter_mut()
                    .filter(|(_, c)| c.class == CasterClass::Static && c.faces & bit != 0)
                    .map(|(e, c)| {
                        c.baked |= bit;
                        *e
                    })
                    .collect();
                list.sort_unstable();
                list
            } else if movers & bit != 0 {
                let mut list: Vec<Entity> =
                    lamp.casters.iter().filter(|(_, c)| c.mover_faces() & bit != 0).map(|(e, _)| *e).collect();
                list.sort_unstable();
                list
            } else {
                continue;
            };
            // Listed casters are extracted even when no camera sees them.
            for e in &list {
                if lamp.casters.get(e).is_some_and(|c| !c.no_cpu_culling)
                    && let Ok(mut v) = visibility.get_mut(*e)
                {
                    v.set_visible();
                }
            }
            if lists.bypass_change_detection().get(usize::from(face)).entities != list {
                lists.get_mut(usize::from(face)).entities = list;
            }
        }

        let copies: Vec<CachedPointShadowCopy> = plan
            .copies
            .iter()
            .filter(|c| c.lamp == lamp.key)
            .map(|c| match c.op {
                CopyOp::Save(layer) => CachedPointShadowCopy::Save { face: c.tile, layer: layer.0 },
                CopyOp::Restore(layer) => CachedPointShadowCopy::Restore { face: c.tile, layer: layer.0 },
                CopyOp::Clear => CachedPointShadowCopy::Clear { face: c.tile },
            })
            .collect();
        let view_faces = lamp_plan.views.0;
        views += view_faces.count_ones();
        let draw_faces = capture | movers;
        // Written only when there is something to do: an untouched component draws nothing.
        if draw_faces != 0 || !copies.is_empty() || cached.view_faces != view_faces {
            *cached = CachedPointShadowMap { view_faces, draw_faces, load_faces: movers, copies, plan: plan.frame };
        }
        marker.set_if_neq(ShadowMapCached { ready: lamp_plan.ready });
    }
    views
}

/// Writes each static atlas lamp's [`AtlasPointShadowMap`], face caster lists and readiness.
fn write_atlas_plan(
    plan: &FramePlan,
    state: &mut LampCacheState,
    lamps: &mut Query<CachedLampItem>,
    visibility: &mut Query<&mut ViewVisibility>,
) -> u32 {
    let Some(mgr) = state.atlas.as_mut() else { return 0 };
    let mut views = 0;
    for (&entity, lamp) in &mut state.lamps {
        let Some(lamp_plan) = plan.lamp(lamp.key) else { continue };
        let Ok((_, _, _, _, Some(mut atlas_comp), mut lists, mut marker, _)) = lamps.get_mut(entity) else { continue };

        let live_valid = mgr.sync_live_faces(lamp.key, lamp.active_mover_faces);
        let static_rects_opt = mgr.lamp_static_rects(lamp.key).unwrap_or([None; 6]);
        let live_rects_opt = mgr.lamp_live_rects(lamp.key).unwrap_or([None; 6]);
        let raw_static_rects = static_rects_opt.map(|r| r.map(sf_sky::shadow_atlas::AtlasRect::to_array));
        let raw_live_rects = live_rects_opt.map(|r| r.map(sf_sky::shadow_atlas::AtlasRect::to_array));
        let static_mask = raw_static_rects.iter().enumerate().fold(0u8, |m, (i, r)| if r.is_some() { m | (1 << i) } else { m });

        let capture = lamp_plan.capture.0 & static_mask;
        let movers = lamp_plan.movers.0 & live_valid;
        if capture != 0 {
            lamp.captures.push((plan.frame, capture));
            if lamp.captures.len() > CAPTURE_HISTORY {
                lamp.captures.remove(0);
            }
        }

        for face in 0..6u8 {
            let bit = 1 << face;
            let list: Vec<Entity> = if capture & bit != 0 {
                let mut list: Vec<Entity> = lamp
                    .casters
                    .iter_mut()
                    .filter(|(_, c)| c.class == CasterClass::Static && c.faces & bit != 0)
                    .map(|(e, c)| {
                        c.baked |= bit;
                        *e
                    })
                    .collect();
                list.sort_unstable();
                list
            } else if movers & bit != 0 {
                let mut list: Vec<Entity> =
                    lamp.casters.iter().filter(|(_, c)| c.mover_faces() & bit != 0).map(|(e, _)| *e).collect();
                list.sort_unstable();
                list
            } else {
                continue;
            };
            for e in &list {
                if lamp.casters.get(e).is_some_and(|c| !c.no_cpu_culling)
                    && let Ok(mut v) = visibility.get_mut(*e)
                {
                    v.set_visible();
                }
            }
            if lists.bypass_change_detection().get(usize::from(face)).entities != list {
                lists.get_mut(usize::from(face)).entities = list;
            }
        }

        // Publish only rects that have been drawn since allocation (SH-33). On a tier resize, keep
        // sampling the retiring static rect until the new one is drawn.
        let (mut static_rects, live_rects) = lamp.atlas_written.gate(raw_static_rects, raw_live_rects, capture, movers);
        if capture != 0 {
            mgr.release_retiring(lamp.key, capture);
        }
        if let Some(retiring) = mgr.lamp_retiring_static(lamp.key) {
            for (published, old) in static_rects.iter_mut().zip(retiring) {
                if published.is_none() {
                    *published = old.map(sf_sky::shadow_atlas::AtlasRect::to_array);
                }
            }
        }
        let mask_of = |rects: &[Option<[u16; 3]>; 6]| {
            rects.iter().enumerate().fold(0u8, |m, (i, r)| if r.is_some() { m | (1 << i) } else { m })
        };
        let written_static = mask_of(&static_rects);
        let written_live = mask_of(&live_rects);

        let draw_faces = capture | movers;
        let draw_to_live = movers;
        let blit_from_static = movers & written_static;
        let mover_only_faces = movers;
        let view_faces = (lamp_plan.views.0 & (written_static | written_live)) | draw_faces;
        views += view_faces.count_ones();

        if draw_faces != 0
            || atlas_comp.view_faces != view_faces
            || atlas_comp.static_rects != static_rects
            || atlas_comp.live_rects != live_rects
        {
            *atlas_comp = AtlasPointShadowMap {
                static_rects,
                live_rects,
                view_faces,
                draw_faces,
                draw_to_live,
                blit_from_static,
                mover_only_faces,
                plan: plan.frame,
            };
        }
        marker.set_if_neq(ShadowMapCached { ready: lamp_plan.ready });
    }
    views
}

struct MovingTorchWork {
    entity: Entity,
    pos: Vec3A,
    reach: f32,
    layers: RenderLayers,
    frusta: CubemapFrusta,
    live_rects: [Option<[u16; 3]>; 6],
    draw_faces: u8,
    face_lists: [Vec<Entity>; 6],
}

/// One pass over `casters.meshes` + `casters.batches` for all active moving torches together,
/// using squared-distance sphere pre-rejection and no mobility walks or temporary hash maps.
fn scan_moving_atlas_torches(
    active: &mut [MovingTorchWork],
    casters: &Casters,
    visibility: &mut Query<&mut ViewVisibility>,
) {
    for (caster_entity, transform, aabb, shown, layers, (unculled, no_cpu, ..), range) in &casters.meshes {
        if !shown.get() {
            continue;
        }
        let affine = transform.affine();
        let center = aabb.map_or(affine.translation, |a| affine.transform_point3a(a.center));
        let radius = if unculled {
            f32::INFINITY
        } else {
            aabb.map_or(f32::INFINITY, |a| {
                let s = affine
                    .matrix3
                    .x_axis
                    .length_squared()
                    .max(affine.matrix3.y_axis.length_squared())
                    .max(affine.matrix3.z_axis.length_squared())
                    .sqrt();
                a.half_extents.length() * s
            })
        };
        let caster_layers = layers.unwrap_or_default();
        let mut hit_any = false;
        for torch in active.iter_mut() {
            if !unculled && radius.is_finite() {
                let max_d = torch.reach + radius;
                if (center - torch.pos).length_squared() > max_d * max_d {
                    continue;
                }
            }
            if !torch.layers.intersects(caster_layers) {
                continue;
            }
            if let Some(range) = range {
                let at = if range.use_aabb { center } else { affine.translation };
                if !range.is_visible_at_all((at - torch.pos).length()) {
                    continue;
                }
            }
            let sphere = Sphere { center: torch.pos, radius: torch.reach };
            let faces = if unculled {
                torch.draw_faces
            } else {
                crate::gpu_culled_casters::caster_faces(&sphere, &torch.frusta, aabb, transform) & torch.draw_faces
            };
            if faces == 0 {
                continue;
            }
            hit_any = true;
            for face in TileMask(faces).iter() {
                torch.face_lists[usize::from(face)].push(caster_entity);
            }
        }
        if hit_any && !no_cpu && let Ok(mut v) = visibility.get_mut(caster_entity) {
            v.set_visible();
        }
    }
    for (batch_entity, shown, layers) in &casters.batches {
        if !shown.get() {
            continue;
        }
        let batch_layers = layers.unwrap_or_default();
        for torch in active.iter_mut() {
            if !torch.layers.intersects(batch_layers) {
                continue;
            }
            for face in TileMask(torch.draw_faces).iter() {
                torch.face_lists[usize::from(face)].push(batch_entity);
            }
        }
    }
}

/// Updates moving atlas lamps (carried torches) drawing directly into their Live Atlas tiles.
#[allow(clippy::too_many_arguments)]
fn update_moving_atlas_lamps(
    state: &mut LampCacheState,
    guard_texels: f32,
    margin: f32,
    frusta: &[Frustum],
    view_frusta: &[ViewFrustum],
    casters: &Casters,
    lamps: &mut Query<CachedLampItem>,
    visibility: &mut Query<&mut ViewVisibility>,
) -> u32 {
    let Some(mgr) = state.atlas.as_ref() else { return 0 };
    if state.moving_atlas.is_empty() {
        return 0;
    }
    state.moving_plan = state.moving_plan.wrapping_add(1);
    let plan_id = state.moving_plan;
    let mut views = 0u32;
    let mut active: Vec<MovingTorchWork> = Vec::with_capacity(state.moving_atlas.len());

    for (&entity, moving) in &state.moving_atlas {
        let Ok((at, light, reach, _, Some(mut atlas_comp), _, mut marker, _)) = lamps.get_mut(entity) else {
            continue;
        };
        let pos = at.translation_vec3a();
        let reach = shadowed_reach(light, reach);
        let sphere = Sphere { center: pos, radius: reach };
        let in_view = frusta.is_empty() || frusta.iter().any(|f| f.intersects_sphere(&sphere, true));
        let seen = if !in_view {
            0
        } else if view_frusta.is_empty() {
            ALL_FACES
        } else {
            point_shadow_faces_seen(Vec3::from(pos), reach, margin, view_frusta)
        };

        let live_rects_opt = mgr.lamp_live_rects(moving.key).unwrap_or([None; 6]);
        let live_rects = live_rects_opt.map(|r| r.map(sf_sky::shadow_atlas::AtlasRect::to_array));
        let live_mask = live_rects.iter().enumerate().fold(0u8, |m, (i, r)| if r.is_some() { m | (1 << i) } else { m });
        let draw_faces = seen & live_mask;

        if draw_faces == 0 {
            if atlas_comp.draw_faces != 0 || atlas_comp.view_faces != 0 || atlas_comp.live_rects != live_rects {
                *atlas_comp = AtlasPointShadowMap {
                    static_rects: [None; 6],
                    live_rects,
                    view_faces: 0,
                    draw_faces: 0,
                    draw_to_live: 0,
                    blit_from_static: 0,
                    mover_only_faces: 0,
                    plan: plan_id,
                };
            }
            marker.set_if_neq(ShadowMapCached { ready: true });
            continue;
        }

        let fov_y = guard_band_fov_y(u32::from(moving.tile_size.max(128)), guard_texels);
        views += draw_faces.count_ones();
        active.push(MovingTorchWork {
            entity,
            pos,
            reach,
            layers: moving.layers.clone(),
            frusta: cube_frusta_fov(at, light.shadow_map_near_z, reach, fov_y),
            live_rects,
            draw_faces,
            face_lists: core::array::from_fn(|_| Vec::new()),
        });
    }

    if !active.is_empty() {
        state.scans += 1;
        scan_moving_atlas_torches(&mut active, casters, visibility);
    }

    for mut torch in active {
        let Ok((_, _, _, _, Some(mut atlas_comp), mut lists, mut marker, _)) = lamps.get_mut(torch.entity) else {
            continue;
        };
        for face in TileMask(torch.draw_faces).iter() {
            let list = &mut torch.face_lists[usize::from(face)];
            list.sort_unstable();
            if lists.bypass_change_detection().get(usize::from(face)).entities != *list {
                lists.get_mut(usize::from(face)).entities = core::mem::take(list);
            }
        }
        *atlas_comp = AtlasPointShadowMap {
            static_rects: [None; 6],
            live_rects: torch.live_rects,
            view_faces: torch.draw_faces,
            draw_faces: torch.draw_faces,
            draw_to_live: torch.draw_faces,
            blit_from_static: 0,
            mover_only_faces: 0,
            plan: plan_id,
        };
        marker.set_if_neq(ShadowMapCached { ready: true });
    }
    views
}

fn publish(counters: &DiagCounters, s: &LampCacheStats) {
    let f = &s.frame;
    for (name, value) in [
        ("lamps", f.lamps),
        ("ready", f.ready_lamps),
        ("views", s.views),
        ("captures", f.captures),
        ("mover_faces", f.mover_draws),
        ("deferred_faces", f.deferred_faces),
        ("store_starved", f.store_starved),
        ("copies", f.saves + f.restores + f.clears),
        ("queued", f.queued),
        ("clean_faces", f.clean_faces),
        ("movers", s.movers),
        ("stock_lamps", s.stock_lamps),
        ("store_used", s.store_used),
    ] {
        counters.set_named(&format!("lamp_cache/{name}"), sf_diag_bevy::sf_diag::unit::COUNT, f64::from(value));
    }
}
