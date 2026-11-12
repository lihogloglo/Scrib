//! Cached point-light shadow maps (superfargoth `docs/specs/shadows.md`, "Shadows v2", V1).
//!
//! Bevy 0.20 clears and redraws all six cube faces of every shadowed point light
//! every frame, and the cost is mostly render-thread work (each face is a view
//! run through a schedule, queued, batched and encoded). This crate keeps, per
//! shadowed light, the depth of its **static** casters in a persistent cube
//! array of our own and hands it back to Bevy:
//!
//! - *Capture* (a light gains a slot, stops moving, or static casters in its
//!   range changed, at most every [`ShadowCacheSettings::recapture_interval`]):
//!   Bevy draws the light whole that frame, and a capture view per face (our
//!   subview of the light, Bevy's own shadow pipelines) draws its static casters
//!   into our cache. Throttled to [`ShadowCacheSettings::captures_per_frame`].
//! - *Cached*: Bevy's views see **moving** casters only (and casters that settled
//!   after the capture). Our restore copies the cached faces into Bevy's cube layer
//!   first, so Bevy draws the movers over the copy and its sampling (filters,
//!   PCSS, clusters) is unchanged. A face with no mover loses its Bevy view, and is
//!   restored only when Bevy's layer went stale ([`ShadowCacheSettings::reuse_layers`]).
//! - A moving light is left to Bevy (full redraw) until it stops.
//!
//! Every lamp face view runs a slim schedule (`render::LampShadowView`: GPU
//! preprocessing and the shadow passes) instead of the whole `Core3d`.
//!
//! Casters are classified automatically: skinned meshes and anything marked
//! [`ShadowMobility::Movable`] (or under an entity so marked) always move; any
//! other mesh whose transform, bounds or visibility really changed counts as
//! moving until it has been still for [`ShadowCacheSettings::settle_time`], then
//! it is baked into the lights it touches; a baked caster that moves stays a
//! mover. Meshes that cast nothing must carry `NotShadowCaster` (a material with
//! shadows off isn't enough: the cache would track it). Hosts on a floating
//! origin report shifts with [`ShadowOriginShift`]. A torch's flicker changes
//! intensity only: no recapture.
//!
//! **GPU instance batches** (`bevy_pbr::gpu_instance_batch::GpuBatchedMesh3d`,
//! e.g. the static scene's retained batches) are static casters: one batch
//! entity draws many instances whose placement lives in GPU slots, so the cache
//! has no transform to watch. It captures them with the other statics (they
//! join the light face lists in `GpuBatchedLightVisibility`, which
//! [`classify_lights`] runs after) and keeps them out of cached faces. Their
//! producer reports where their instances changed (slots written or released)
//! with [`ShadowCastersChanged`], which recaptures the lights that sphere
//! touches; a batch whose instances move every frame must be `NotShadowCaster`.
//!
//! **GPU-culled casters** (`NoCpuCulling`, the streamed statics): Bevy's
//! point-light scan skips them, and with the vendored
//! `bevy_pbr::CpuListedPointShadowCasters` (inserted here) its GPU-culled path
//! leaves them out of point faces too. The cache lists them itself: each
//! shadowed point light keeps the GPU-culled casters within its range plus a
//! margin ([`GpuCulledLists`], rebuilt when the light moves past the margin,
//! updated from the casters that changed), and [`add_gpu_culled_casters`]
//! adds the ones in each face's frustum and in LOD range of Bevy's shadow LOD
//! origin to the face lists before [`classify_lights`], so every rule above
//! (captures take the statics, cached faces keep the movers, uncached lights
//! draw everything) holds unchanged. A GPU-culled caster is a known static: it
//! is baked when first seen, with no settle time
//! (`docs/research/statics-cpu-cost-2026-11-12.md`, slice 1).

mod render;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::embedded_asset;
use bevy_camera::primitives::{Aabb, CubemapFrusta, Sphere as BevySphere};
use bevy_camera::{Camera, RenderTarget, ShadowLodOrigin};
use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, NoCpuCulling, RenderLayers, ViewVisibility, VisibilityRange, VisibilitySystems};
use bevy_ecs::entity::{EntityHashMap, EntityHashSet};
use bevy_ecs::prelude::*;
use bevy_light::{NotShadowCaster, PointLight, PointLightShadowMap, SimulationLightSystems, SpotLight};
use bevy_math::{Affine3A, Vec3, Vec3A};
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::Mesh3d;
use bevy_pbr::gpu_instance_batch::GpuBatchedLightVisibility;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;
use bevy_render::sync_world::{MainEntity, RenderEntity};

/// Most cube maps a 2D array can hold (wgpu's default 256 layers / 6).
pub const MAX_CACHED_LIGHTS: usize = 42;

/// Settings; insert before or after the plugin (it inits the default).
#[derive(Resource, Clone, Debug)]
pub struct ShadowCacheSettings {
    /// Off: Bevy draws every shadowed light every frame (A/B).
    pub enabled: bool,
    /// Cache slots (cube maps kept, also for lights that lost their shadow
    /// recently: re-entering a room costs nothing until evicted, LRU). The
    /// texture grows in steps of 4 as slots are used, up to this.
    pub max_lights: usize,
    /// Seconds a moved caster must stay still before it is baked again.
    pub settle_time: f32,
    /// Light movement or range change (world units) that invalidates its cache.
    pub move_threshold: f32,
    /// Restore a detached face into Bevy's cube texture only when its layer
    /// went stale (new texture, new layer, recapture, movers drawn over it),
    /// not every frame. Off: every detached face is restored every frame (A/B).
    pub reuse_layers: bool,
    /// Lights captured per frame at most (a capture draws a light's static
    /// casters into the cache: one extra view per face). Lights waiting for
    /// their turn are drawn whole by Bevy, so nothing is missing meanwhile.
    pub captures_per_frame: usize,
    /// Seconds between a light's recaptures when static casters in its range change.
    pub recapture_interval: f32,
    /// Set by the host while the screen is hidden (a loading fade): captures are
    /// unlimited, recaptures immediate, and casters that appear settle after
    /// [`Self::warmup_settle_time`]; [`ShadowCacheStats::pending`] then also counts
    /// lights a settling caster touches, so the host can wait for
    /// [`ShadowCacheStats::settled`] before showing the frame.
    pub warmup: bool,
    /// Settle time of casters that appear during a warmup, seconds.
    pub warmup_settle_time: f32,
}

impl Default for ShadowCacheSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_lights: 24,
            settle_time: 1.0,
            move_threshold: 0.01,
            reuse_layers: true,
            captures_per_frame: 2,
            recapture_interval: 2.0,
            warmup: false,
            warmup_settle_time: 0.2,
        }
    }
}

/// How a shadow caster is treated (default: [`ShadowMobility::Auto`]).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShadowMobility {
    /// Baked once it has been still for the settle time after appearing;
    /// skinned meshes, and baked casters that move, are movers.
    #[default]
    Auto,
    /// Always drawn per frame over the cached copy (actors, carried items).
    Movable,
    /// Always baked; a change recaptures the lights it touches.
    Static,
}

/// What the cache did (main world, for logs and benches).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ShadowCacheStats {
    /// Lights drawn from the cache this frame.
    pub cached: u32,
    /// Lights capturing (static casters only, this frame).
    pub capturing: u32,
    /// Shadowed lights Bevy draws in full (moving, or no free slot).
    pub uncached: u32,
    /// Casters currently treated as moving.
    pub moving_casters: u32,
    /// Faces of cached lights that hold a mover (Bevy keeps their views).
    pub mover_faces: u32,
    /// Recaptures since start because the light moved, or a static caster in its range changed.
    pub moved: u64,
    pub touched: u64,
    /// Captures completed since start (each is six faces).
    pub captures: u64,
    /// Shadowed lights in view with a slot whose cached faces aren't final yet
    /// (waiting for or in a capture, due a recapture; in a warmup also
    /// touched by a settling caster). A moving light (a swaying lantern) isn't
    /// pending: Bevy draws it whole every frame.
    pub pending: u32,
}

impl ShadowCacheStats {
    /// Every shadowed light in view draws from final cached faces (or has no slot).
    pub fn settled(&self) -> bool {
        self.pending == 0
    }
}

/// A floating-origin shift reported by the host: every render-space position
/// moved by this (world units) this frame. Write it in the shift's frame,
/// after transform propagation and before [`ShadowCacheSystems`]; the cache
/// moves what it remembers along, so a shift isn't taken for every caster and
/// light moving (a recapture of everything). The cache resets it.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShadowOriginShift(pub Vec3);

/// Static casters the cache cannot track as entities changed inside this
/// sphere (render space, world units): a GPU instance batch's slots were written
/// or released (a cell streamed its statics in or out). Every cached light the
/// sphere reaches recaptures on its schedule, as when a baked mesh in its range
/// changes. Write it in the frame of this frame's origin shift, before
/// [`ShadowCacheSystems`] (a later one is read the next frame).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct ShadowCastersChanged {
    pub center: Vec3,
    pub radius: f32,
}

impl ShadowCastersChanged {
    /// The sphere around a box.
    pub fn from_aabb(min: Vec3, max: Vec3) -> Self {
        Self { center: (min + max) * 0.5, radius: (max - min).length() * 0.5 }
    }
}

/// The cache's main-world systems (`PostUpdate`).
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShadowCacheSystems;

/// The plugin.
pub struct ShadowCachePlugin;

impl Plugin for ShadowCachePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "copy_depth.wesl");
        let shared = SharedCaptures::default();
        add_main_world(app, shared.clone());
        render::build(app, shared);
    }

    fn finish(&self, app: &mut App) {
        render::finish(app);
    }
}

/// The plugin's main-world half (resources, messages, [`ShadowCacheSystems`]).
fn add_main_world(app: &mut App, shared: SharedCaptures) {
    app.init_resource::<ShadowCacheSettings>()
        .init_resource::<ShadowCacheStats>()
        .init_resource::<ShadowOriginShift>()
        .add_message::<ShadowCastersChanged>()
        .init_resource::<Casters>()
        .init_resource::<GpuCulledLists>()
        .init_resource::<Lights>()
        .init_resource::<render::CacheFrame>()
        .insert_resource(shared)
        .add_systems(
            PostUpdate,
            (
                track_casters
                    .after(TransformSystems::Propagate)
                    .after(VisibilitySystems::VisibilityPropagate)
                    .after(VisibilitySystems::CalculateBounds),
                // After Bevy's lists and the GPU instance batches joined them (both statics).
                add_gpu_culled_casters.after(SimulationLightSystems::CheckLightVisibility).after(GpuBatchedLightVisibility),
                classify_lights,
            )
                .chain()
                .in_set(ShadowCacheSystems),
        );
}

/// Faces captured by the render world: light (main entity) → (generation,
/// face mask). Written by the capture pass, read by [`classify_lights`].
#[derive(Resource, Clone, Default)]
pub(crate) struct SharedCaptures(pub(crate) Arc<Mutex<HashMap<Entity, (u32, u8)>>>);

/// A caster's world bounding sphere.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn of(transform: &GlobalTransform, aabb: Option<&Aabb>) -> Self {
        match aabb {
            Some(b) => Self {
                center: transform.transform_point(Vec3::from(b.center)),
                radius: Vec3::from(b.half_extents).length() * transform.scale().abs().max_element(),
            },
            None => Self { center: transform.translation(), radius: 0.0 },
        }
    }

    /// Whether this sphere reaches into a light's range.
    fn touches(self, light: Vec3, range: f32) -> bool {
        self.center.distance_squared(light) < (range + self.radius).powi(2)
    }
}

struct Caster {
    sphere: Sphere,
    /// Moving until this time (`INFINITY`: always).
    until: f32,
    moving: bool,
    /// Where it was last seen, and whether it was shown.
    at: Affine3A,
    visible: bool,
    /// Why it moves (report): 0 settling, 1 skinned, 2 moved after its bake, 3 marked movable.
    why: u8,
    /// When it last settled (baked); a light captured before that still draws it as a mover.
    settled_at: f32,
    /// `NoCpuCulling`: Bevy's light scan never lists it, [`GpuCulledLists`] does.
    gpu_culled: bool,
    /// Its `VisibilityRange` as `(start, end, use_aabb)`: drawn from `start` to
    /// `end` meters of the LOD origin (`is_visible_at_all`).
    lod: Option<(f32, f32, bool)>,
    /// Its render layers (GPU-culled casters only; `None`: layer 0): a light lists it only if they meet.
    layers: Option<RenderLayers>,
}

impl Caster {
    /// Whether its visibility range shows it (at all) from `origin`.
    fn in_lod_range(&self, origin: Option<Vec3>) -> bool {
        let (Some((start, end, use_aabb)), Some(origin)) = (self.lod, origin) else { return true };
        let at = if use_aabb { self.sphere.center } else { Vec3::from(self.at.translation) };
        let d = at.distance(origin);
        d >= start && d < end
    }
}

#[derive(Resource, Default)]
struct Casters {
    map: EntityHashMap<Caster>,
    moving: EntityHashSet,
    /// Spheres whose baked depth changed this frame.
    dirty: Vec<(Sphere, Entity)>,
    /// Dirty events since start: removed, moved after bake, static changed, settled,
    /// reported ([`ShadowCastersChanged`]).
    kinds: [u64; 5],
    /// GPU-culled casters added, changed or removed this frame ([`GpuCulledLists`]).
    gpu_changed: Vec<Entity>,
}

type CasterItem<'a> = (
    Entity,
    &'a GlobalTransform,
    Option<Ref<'a, Aabb>>,
    Option<&'a ShadowMobility>,
    Has<SkinnedMesh>,
    &'a InheritedVisibility,
    (Has<NoCpuCulling>, Option<&'a VisibilityRange>, Option<&'a RenderLayers>),
);
type CasterChanged = (
    With<Mesh3d>,
    Without<NotShadowCaster>,
    Or<(Changed<GlobalTransform>, Changed<InheritedVisibility>, Changed<Aabb>, Added<NoCpuCulling>, Changed<VisibilityRange>, Changed<RenderLayers>)>,
);

/// Classifies casters (moving or baked) and collects the spheres whose baked
/// depth changed: a baked caster that moved (old and new place), a caster
/// that settled, a baked caster that went away, and what producers of GPU
/// instance batches report ([`ShadowCastersChanged`], no entity:
/// `Entity::PLACEHOLDER`).
#[allow(clippy::too_many_arguments)]
fn track_casters(
    time: Res<Time>,
    settings: Res<ShadowCacheSettings>,
    shift: Res<ShadowOriginShift>,
    mut casters: ResMut<Casters>,
    changed: Query<CasterItem, CasterChanged>,
    mut removed: RemovedComponents<Mesh3d>,
    ancestors: Query<(Option<&ShadowMobility>, Option<&ChildOf>)>,
    mut reported: MessageReader<ShadowCastersChanged>,
) {
    let now = time.elapsed_secs();
    let c = &mut *casters;
    c.dirty.clear();
    c.gpu_changed.clear();
    if shift.0 != Vec3::ZERO {
        for k in c.map.values_mut() {
            k.sphere.center += shift.0;
            k.at.translation += Vec3A::from(shift.0);
        }
    }
    for change in reported.read() {
        c.dirty.push((Sphere { center: change.center, radius: change.radius }, Entity::PLACEHOLDER));
        c.kinds[4] += 1;
    }
    for entity in removed.read() {
        if let Some(old) = c.map.remove(&entity) {
            c.moving.remove(&entity);
            if old.gpu_culled {
                c.gpu_changed.push(entity);
            }
            if !old.moving {
                c.dirty.push((old.sphere, entity));
                c.kinds[0] += 1;
            }
        }
    }
    for (entity, transform, aabb, mobility, skinned, visibility, (gpu_culled, range, render_layers)) in &changed {
        let (at, visible) = (transform.affine(), visibility.get());
        let lod = range.map(|r| (r.start_margin.start, r.end_margin.end, r.use_aabb));
        let layers = render_layers.filter(|_| gpu_culled).cloned();
        let same = |k: &Caster| k.visible == visible && k.gpu_culled == gpu_culled && k.lod == lod && k.layers == layers && k.at.abs_diff_eq(at, 1e-3);
        let aabb_changed = aabb.as_ref().is_some_and(DetectChanges::is_changed);
        if !aabb_changed && c.map.get(&entity).is_some_and(same) {
            continue;
        }
        let sphere = Sphere::of(transform, aabb.as_deref());
        // Change detection fires on writes, not moves (animation and propagation rewrite
        // unchanged transforms every frame): only a real change counts.
        if c.map.get(&entity).is_some_and(|k| same(k) && (k.sphere.radius - sphere.radius).abs() < 1e-4) {
            continue;
        }
        // Own marker, else the nearest ancestor's (hosts mark an actor's root: its held
        // items and attachments move with it even when they aren't skinned).
        let mobility = mobility.copied().or_else(|| ancestor_mobility(entity, &ancestors)).unwrap_or_default();
        // A GPU-culled mesh is a known static (the host tags statics only): baked at once, and a
        // change (a cell's reveal, a LOD range) recaptures the lights it touches.
        let mobility = if gpu_culled && !skinned && mobility == ShadowMobility::Auto { ShadowMobility::Static } else { mobility };
        if gpu_culled || c.map.get(&entity).is_some_and(|k| k.gpu_culled) {
            c.gpu_changed.push(entity);
        }
        let was_baked = c.map.get(&entity).filter(|k| !k.moving).map(|k| k.sphere);
        // A GPU-culled static that stays hidden (spawned under a level not yet revealed)
        // changes no light's depth: no recapture for it.
        let unseen = gpu_culled && !visible && c.map.get(&entity).is_none_or(|k| !k.visible);
        if let Some(old) = was_baked.filter(|_| !unseen) {
            c.dirty.push((old, entity));
            c.kinds[1] += 1;
        }
        // A baked caster that moves stays a mover (doors, items, animated parts that
        // start and stop): each caster costs at most one bake and one unbake, so
        // intermittent movers can't keep the lights recapturing.
        let always = mobility == ShadowMobility::Movable || (mobility == ShadowMobility::Auto && (skinned || was_baked.is_some()));
        let caster = if mobility == ShadowMobility::Static {
            if !unseen {
                c.dirty.push((sphere, entity));
                c.kinds[2] += 1;
            }
            c.moving.remove(&entity);
            Caster { sphere, until: now, moving: false, at, visible, why: 0, settled_at: now, gpu_culled, lod, layers }
        } else {
            c.moving.insert(entity);
            let why = if mobility == ShadowMobility::Movable { 3 } else if skinned { 1 } else if was_baked.is_some() { 2 } else { 0 };
            let settle = if settings.warmup { settings.warmup_settle_time } else { settings.settle_time };
            Caster { sphere, until: if always { f32::INFINITY } else { now + settle }, moving: true, at, visible, why, settled_at: f32::INFINITY, gpu_culled, lod, layers }
        };
        c.map.insert(entity, caster);
    }
    let Casters { map, moving, dirty, kinds, .. } = c;

    moving.retain(|entity| {
        let Some(k) = map.get_mut(entity) else { return false };
        if k.until <= now {
            k.moving = false;
            k.settled_at = now;
            dirty.push((k.sphere, *entity));
            kinds[3] += 1;
            false
        } else {
            true
        }
    });
}

/// The nearest ancestor's [`ShadowMobility`] (at most 16 levels up).
fn ancestor_mobility(entity: Entity, ancestors: &Query<(Option<&ShadowMobility>, Option<&ChildOf>)>) -> Option<ShadowMobility> {
    let mut at = ancestors.get(entity).ok().and_then(|(_, p)| p.map(ChildOf::parent));
    for _ in 0..16 {
        let (mobility, parent) = ancestors.get(at?).ok()?;
        if let Some(m) = mobility {
            return Some(*m);
        }
        at = parent.map(ChildOf::parent);
    }
    None
}

/// A shadowed point light's GPU-culled casters: those whose sphere reaches
/// `reach` meters around `center` (the light's range plus a margin, so a light
/// that moves a little keeps its list).
struct GpuCulledList {
    center: Vec3,
    range: f32,
    reach: f32,
    members: EntityHashSet,
}

impl GpuCulledList {
    /// Margin around a light's range (world units): a light moving less keeps its list.
    fn margin(range: f32) -> f32 {
        (range * 0.25).max(1.0)
    }

    /// Whether it still covers a light at `position` with `range`.
    fn covers(&self, position: Vec3, range: f32) -> bool {
        (self.range - range).abs() <= f32::EPSILON && self.center.distance(position) + range <= self.reach
    }

    fn admits(&self, k: &Caster) -> bool {
        k.gpu_culled && k.sphere.touches(self.center, self.reach)
    }
}

/// Each shadowed point light's [`GpuCulledList`] (by light entity).
#[derive(Resource, Default)]
struct GpuCulledLists(EntityHashMap<GpuCulledList>);

/// Bevy's point-light scan never sees GPU-culled (`NoCpuCulling`) casters: adds
/// those each shadowed point light reaches to its face lists (per face: sphere
/// in the face's frustum, in the light's range, shown by its visibility range
/// from Bevy's shadow LOD origin, visible), so [`classify_lights`] treats them
/// as Bevy's own. The lists are kept per light and updated from this frame's
/// changed casters; a light rebuilds its list (one pass over the casters) when
/// it first gains shadows or moves past the margin.
#[allow(clippy::type_complexity)]
fn add_gpu_culled_casters(
    casters: Res<Casters>,
    mut lists: ResMut<GpuCulledLists>,
    mut lights: Query<(Entity, &PointLight, &GlobalTransform, &CubemapFrusta, &ViewVisibility, &mut CubemapVisibleEntities, Option<&RenderLayers>)>,
    mut removed: RemovedComponents<PointLight>,
    (mut cameras, mut origins, mut lamps, transforms): (
        Query<(Entity, &RenderTarget), With<Camera>>,
        Query<Entity, With<ShadowLodOrigin>>,
        Query<Entity, Or<(With<PointLight>, With<SpotLight>)>>,
        Query<&GlobalTransform>,
    ),
) {
    for entity in removed.read() {
        lists.0.remove(&entity);
    }
    // Bevy's rule for point and spot shadow LODs (`bevy_light::get_shadow_lod_origin`).
    let origin = bevy_light::get_shadow_lod_origin(cameras.transmute_lens_filtered(), origins.transmute_lens_filtered(), lamps.transmute_lens_filtered())
        .and_then(|e| transforms.get(e).ok())
        .map(GlobalTransform::translation);
    let layer_zero = RenderLayers::default();
    for (entity, light, transform, frusta, view_visibility, mut faces, light_layers) in &mut lights {
        let light_layers = light_layers.unwrap_or(&layer_zero);
        if !light.shadow_maps_enabled {
            lists.0.remove(&entity);
            continue;
        }
        let position = transform.translation();
        let list = lists.0.entry(entity).or_insert_with(|| GpuCulledList { center: position, range: -1.0, reach: 0.0, members: EntityHashSet::default() });
        if list.covers(position, light.range) {
            for e in &casters.gpu_changed {
                if casters.map.get(e).is_some_and(|k| list.admits(k)) {
                    list.members.insert(*e);
                } else {
                    list.members.remove(e);
                }
            }
        } else {
            list.center = position;
            list.range = light.range;
            list.reach = light.range + GpuCulledList::margin(light.range);
            let members = casters.map.iter().filter(|(_, k)| list.admits(k)).map(|(e, _)| *e).collect();
            list.members = members;
        }
        // Bevy fills the faces of lights in view only.
        if !view_visibility.get() || list.members.is_empty() {
            continue;
        }
        let shown: Vec<(Entity, BevySphere)> = list
            .members
            .iter()
            .filter_map(|e| casters.map.get(e).map(|k| (*e, k)))
            .filter(|(_, k)| k.visible && k.sphere.touches(position, light.range) && k.in_lod_range(origin))
            // Bevy's scan: a caster casts only into lights it shares a render layer with.
            .filter(|(_, k)| light_layers.intersects(k.layers.as_ref().unwrap_or(&layer_zero)))
            .map(|(e, k)| (e, BevySphere { center: Vec3A::from(k.sphere.center), radius: k.sphere.radius }))
            .collect();
        if shown.is_empty() {
            continue;
        }
        for (frustum, face) in frusta.iter().zip(faces.iter_mut()) {
            let before = face.entities.len();
            face.entities.extend(shown.iter().filter(|(_, s)| frustum.intersects_sphere(s, true)).map(|(e, _)| *e));
            if face.entities.len() != before {
                // Sorted like Bevy's lists; a list Bevy left as it was (it skipped the light) holds ours.
                face.entities.sort_unstable();
                face.entities.dedup();
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SlotState {
    /// Moving: Bevy draws it in full.
    Moving,
    /// Capturing generation `gen` (static casters only).
    Capture,
    /// Cached faces valid: Bevy draws moving casters only.
    Valid,
}

struct LightSlot {
    slot: u32,
    position: Vec3,
    range: f32,
    state: SlotState,
    generation: u32,
    used: u64,
    /// When its current capture began (casters settled later are drawn as movers).
    captured_at: f32,
    /// A static caster in range changed since that capture (recaptured on schedule).
    stale: bool,
}

#[derive(Resource, Default)]
struct Lights {
    map: HashMap<Entity, LightSlot>,
    free: Vec<u32>,
    /// Slots handed out so far (the texture holds `capacity` of them).
    next: u32,
    capacity: u32,
    map_size: u32,
    frame: u64,
}

impl Lights {
    /// A slot for a light: a free one, a new one, or the least recently used
    /// one not used this frame. `None` when all are in use.
    fn allocate(&mut self, max: u32) -> Option<u32> {
        if let Some(slot) = self.free.pop() {
            return Some(slot);
        }
        if self.next < max {
            self.next += 1;
            return Some(self.next - 1);
        }
        let frame = self.frame;
        let (&victim, _) = self.map.iter().filter(|(_, l)| l.used < frame).min_by_key(|(_, l)| l.used)?;
        self.map.remove(&victim).map(|l| l.slot)
    }

    fn invalidate_all(&mut self, now: f32) {
        for light in self.map.values_mut() {
            light.state = SlotState::Capture;
            light.generation = light.generation.wrapping_add(1);
            light.captured_at = now;
            light.stale = false;
        }
    }
}

/// Keeps a cached light's face lists to its movers (drawn over the restored
/// static faces); returns the faces left with something to draw (bit per face).
fn filter_faces(faces: &mut CubemapVisibleEntities, mover: impl Fn(&Entity) -> bool) -> u8 {
    let mut drawn = 0u8;
    for (i, face) in faces.iter_mut().enumerate() {
        face.entities.retain(|e| mover(e));
        if !face.entities.is_empty() {
            drawn |= 1 << i;
        }
    }
    drawn
}

/// Texture capacity for `used` slots: steps of 4, at most `max`.
fn capacity_for(used: u32, max: u32) -> u32 {
    used.div_ceil(4).max(1).saturating_mul(4).min(max)
}

/// Picks each shadowed light's mode, filters its cube faces' visible casters
/// to match (static only while capturing, moving only when cached) and hands
/// the frame to the render world.
#[allow(clippy::too_many_arguments, clippy::too_many_lines, clippy::type_complexity)]
fn classify_lights(
    settings: Res<ShadowCacheSettings>,
    map_size: Option<Res<PointLightShadowMap>>,
    casters: Res<Casters>,
    shared: Res<SharedCaptures>,
    mut lights: ResMut<Lights>,
    mut frame: ResMut<render::CacheFrame>,
    mut stats: ResMut<ShadowCacheStats>,
    mut query: Query<(Entity, &PointLight, &GlobalTransform, &ViewVisibility, &mut CubemapVisibleEntities)>,
    mut removed: RemovedComponents<PointLight>,
    (time, names, parents, mut next_report, mut touchers): (Res<Time>, Query<&Name>, Query<&ChildOf>, Local<f32>, Local<Vec<Entity>>),
    (render_entities, mut shift, mut why_pending): (Query<&RenderEntity>, ResMut<ShadowOriginShift>, Local<[u32; 3]>),
) {
    frame.captures.clear();
    frame.lights.clear();
    // A warmup that ended unsettled (the host's timeout): why its last frame wasn't.
    if settings.warmup || why_pending.iter().all(|n| *n == 0) {
        *why_pending = [0; 3];
    } else {
        let [capture, stale, settling] = std::mem::take(&mut *why_pending);
        log::debug!("shadow cache: warmup ended unsettled: {capture} light(s) not captured, {stale} due a recapture, {settling} touched by a settling caster");
    }
    // Movers seen in cached lights' faces this frame (a few, for the report).
    let mut sample: Vec<Entity> = Vec::new();
    let now = time.elapsed_secs();
    let l = &mut *lights;
    l.frame += 1;
    if shift.0 != Vec3::ZERO {
        for light in l.map.values_mut() {
            light.position += shift.0;
        }
        shift.0 = Vec3::ZERO;
    }
    for entity in removed.read() {
        if let Some(gone) = l.map.remove(&entity) {
            l.free.push(gone.slot);
        }
    }
    let size = map_size.map_or(1024, |m| u32::try_from(m.size).unwrap_or(1024));
    if !settings.enabled || size != l.map_size {
        // Off, or a new face size: every cached face is stale.
        l.map_size = size;
        l.invalidate_all(now);
        if !settings.enabled {
            shared.0.lock().map(|mut m| m.clear()).ok();
            *stats = ShadowCacheStats { captures: stats.captures, ..ShadowCacheStats::default() };
            return;
        }
    }
    // Confirmed captures: all six faces of the current generation.
    let mut confirmed = 0;
    if let Ok(mut faces) = shared.0.lock() {
        faces.retain(|entity, (generation, mask)| {
            let Some(light) = l.map.get_mut(entity) else { return false };
            if *generation != light.generation {
                return false;
            }
            if *mask == 0x3F {
                if light.state == SlotState::Capture {
                    light.state = SlotState::Valid;
                    confirmed += 1;
                }
                return false;
            }
            true
        });
    }
    let max = u32::try_from(settings.max_lights.clamp(1, MAX_CACHED_LIGHTS)).unwrap_or(1);
    let mut counts = ShadowCacheStats { captures: stats.captures + confirmed, moved: stats.moved, touched: stats.touched, ..ShadowCacheStats::default() };
    for (entity, light, transform, view_visibility, mut faces) in &mut query {
        if !light.shadow_maps_enabled {
            continue;
        }
        let position = transform.translation();
        if !l.map.contains_key(&entity) {
            let Some(slot) = l.allocate(max) else {
                counts.uncached += 1;
                continue;
            };
            l.map.insert(entity, LightSlot { slot, position, range: light.range, state: SlotState::Capture, generation: 0, used: 0, captured_at: now, stale: false });
        }
        let frame_no = l.frame;
        let threshold = settings.move_threshold;
        let Some(cached) = l.map.get_mut(&entity) else { continue };
        cached.used = frame_no;
        let moved = cached.position.distance(position) > threshold || (cached.range - light.range).abs() > threshold;
        let dirtied_by = casters.dirty.iter().find(|(s, _)| s.touches(position, light.range)).map(|(_, e)| *e);
        let touched = dirtied_by.is_some();
        if let Some(e) = dirtied_by
            && touchers.len() < 6
        {
            touchers.push(e);
        }
        if moved {
            counts.moved += 1;
            cached.generation = cached.generation.wrapping_add(1);
            cached.position = position;
            cached.range = light.range;
            cached.state = SlotState::Moving;
        } else if cached.state == SlotState::Moving {
            cached.state = SlotState::Capture;
            cached.captured_at = now;
            cached.stale = false;
        }
        // A static caster in range changed (streaming, LOD swaps): recapture on a
        // schedule, not per event (HDRP on-demand / Doom budgeted updates). Until
        // then a caster settled after the capture is drawn as a mover; one removed
        // since leaves its shadow for at most the interval.
        cached.stale |= touched && !moved;
        let interval = if settings.warmup { 0.0 } else { settings.recapture_interval };
        if cached.stale && cached.state == SlotState::Valid && now - cached.captured_at >= interval {
            counts.touched += 1;
            cached.generation = cached.generation.wrapping_add(1);
            cached.state = SlotState::Capture;
            cached.captured_at = now;
            cached.stale = false;
        }
        let captured_at = cached.captured_at;
        let settling = |k: &Caster| k.why == 0 && k.until.is_finite() && k.sphere.touches(position, light.range);
        if view_visibility.get() {
            // Why it isn't final (warmup report): not captured yet, due a recapture, a caster settling.
            let why = if cached.state == SlotState::Capture {
                Some(0)
            } else if cached.stale {
                Some(1)
            } else if settings.warmup && casters.moving.iter().any(|e| casters.map.get(e).is_some_and(settling)) {
                Some(2)
            } else {
                None
            };
            if let Some(i) = why {
                counts.pending += 1;
                why_pending[i] += u32::from(settings.warmup);
            }
        }
        let per_frame = if settings.warmup { u32::MAX } else { u32::try_from(settings.captures_per_frame).unwrap_or(u32::MAX) };
        match cached.state {
            // Bevy draws the light whole (its faces keep every caster), and on its
            // turn a capture view per face draws the static casters into the cache:
            // what is on screen never lacks the movers.
            // (Only lights Bevy extracts this frame: one out of every view has no faces.)
            SlotState::Capture if view_visibility.get() && counts.capturing < per_frame => {
                counts.capturing += 1;
                let statics = |i: usize| {
                    let mut list: Vec<(Entity, MainEntity)> = faces
                        .get(i)
                        .entities
                        .iter()
                        .filter(|e| !casters.moving.contains(*e))
                        .map(|e| (render_entities.get(*e).map_or(Entity::PLACEHOLDER, |r| **r), MainEntity::from(*e)))
                        .collect();
                    list.sort_unstable();
                    list
                };
                frame.captures.push(render::CaptureRequest { light: entity, slot: cached.slot, faces: std::array::from_fn(statics) });
                frame.lights.push(render::CachedLight { light: entity, slot: cached.slot, capture: true, generation: cached.generation, movers: 0x3F });
            }
            // Moving, or waiting for a capture turn: Bevy draws it whole.
            SlotState::Moving | SlotState::Capture => {
                counts.uncached += 1;
            }
            SlotState::Valid => {
                let movers = filter_faces(&mut faces, |e| casters.moving.contains(e) || casters.map.get(e).is_some_and(|k| k.settled_at > captured_at));
                counts.cached += 1;
                counts.mover_faces += movers.count_ones();
                if sample.len() < 8 {
                    sample.extend(faces.iter().flat_map(|f| f.entities.iter().copied()).filter(|e| casters.map.get(e).is_some_and(|k| k.why == 0)).take(8 - sample.len()));
                }
                frame.lights.push(render::CachedLight { light: entity, slot: cached.slot, capture: false, generation: cached.generation, movers });
            }
        }
    }
    let capacity = capacity_for(l.next, max);
    if capacity != l.capacity {
        // A new texture: everything recaptures (the frame already sent
        // restores for this frame's lights, so turn them into captures).
        l.capacity = capacity;
        l.invalidate_all(now);
        frame.lights.clear();
        frame.captures.clear();
    }
    frame.map_size = size;
    frame.slots = l.capacity;
    frame.reuse = settings.reuse_layers;
    counts.moving_casters = u32::try_from(casters.moving.len()).unwrap_or(u32::MAX);
    if confirmed > 0 {
        log::debug!(
            "shadow cache: {confirmed} light(s) captured (cached {}, capturing {}, uncached {}, moving casters {}, total captures {})",
            counts.cached,
            counts.capturing,
            counts.uncached,
            counts.moving_casters,
            counts.captures
        );
    }

    if now >= *next_report && counts.cached + counts.capturing + counts.uncached > 0 {
        *next_report = now + 5.0;
        // A mover's own name, else its nearest named ancestor's.
        let name = |e: Entity| {
            if e == Entity::PLACEHOLDER {
                return "reported change".to_owned();
            }
            let mut at = Some(e);
            for _ in 0..6 {
                let Some(x) = at else { break };
                if let Ok(n) = names.get(x) {
                    return n.as_str().to_owned();
                }
                at = parents.get(x).ok().map(ChildOf::parent);
            }
            format!("{e}")
        };
        let movers: Vec<String> = sample.iter().map(|e| name(*e)).collect();
        let dirtied: Vec<String> = touchers.drain(..).map(name).collect();
        let mut why = [0u32; 4];
        for k in casters.moving.iter().filter_map(|e| casters.map.get(e)) {
            why[usize::from(k.why.min(3))] += 1;
        }
        log::debug!(
            "shadow cache: {} cached ({} of {} faces hold a mover), {} capturing, {} uncached, {} captures so far (recaptures: {} moved, {} touched); {} moving casters \
             (settling {}, skinned {}, moved after bake {}, movable {}), e.g. {movers:?}; recaptured by {dirtied:?}; dirty events (removed, moved, static, settled, reported) {:?}",
            counts.cached,
            counts.mover_faces,
            counts.cached * 6,
            counts.capturing,
            counts.uncached,
            counts.captures,
            counts.moved,
            counts.touched,
            counts.moving_casters,
            why[0],
            why[1],
            why[2],
            why[3],
            casters.kinds
        );
    }
    *stats = counts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_grows_in_steps_of_four() {
        assert_eq!(capacity_for(0, 24), 4);
        assert_eq!(capacity_for(5, 24), 8);
        assert_eq!(capacity_for(30, 24), 24);
    }

    #[test]
    fn slots_reuse_free_then_evict_least_recent() {
        let mut lights = Lights { frame: 5, ..Lights::default() };
        let a = lights.allocate(2).unwrap();
        let b = lights.allocate(2).unwrap();
        assert_ne!(a, b);
        let slot = |slot, used| LightSlot { slot, position: Vec3::ZERO, range: 1.0, state: SlotState::Valid, generation: 0, used, captured_at: 0.0, stale: false };
        lights.map.insert(Entity::from_raw_u32(1).unwrap(), slot(a, 3));
        lights.map.insert(Entity::from_raw_u32(2).unwrap(), slot(b, 5));
        // Full: the light unused since frame 3 gives up its slot; the one used
        // this frame keeps it.
        assert_eq!(lights.allocate(2), Some(a));
        assert_eq!(lights.allocate(2), None);
    }

    use bevy_pbr::gpu_instance_batch::GpuBatchedMesh3d;

    /// Stands in for the vendored `add_gpu_batched_meshes_to_light_visibility`
    /// (crate-private there), in its public set: every batch joins every face.
    fn batches_join_faces(batches: Query<Entity, With<GpuBatchedMesh3d>>, mut lights: Query<&mut CubemapVisibleEntities>) {
        for mut faces in &mut lights {
            for face in faces.iter_mut() {
                for batch in &batches {
                    if let Err(i) = face.entities.binary_search(&batch) {
                        face.entities.insert(i, batch);
                    }
                }
            }
        }
    }

    /// The main-world half with a lamp at the origin (range 10) and a batch.
    fn lamp_and_batch() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.init_resource::<Time>();
        add_main_world(&mut app, SharedCaptures::default());
        app.add_systems(PostUpdate, batches_join_faces.in_set(GpuBatchedLightVisibility));
        // Both write the face lists: an unordered pair fails the schedule build instead of
        // passing on whichever order the executor happens to pick.
        app.edit_schedule(PostUpdate, |schedule| {
            schedule.set_build_settings(bevy_ecs::schedule::ScheduleBuildSettings {
                ambiguity_detection: bevy_ecs::schedule::LogLevel::Error,
                ..Default::default()
            });
        });
        // Time stands still here: recapture as soon as a change is seen.
        app.world_mut().resource_mut::<ShadowCacheSettings>().recapture_interval = 0.0;
        let light = app
            .world_mut()
            .spawn((PointLight { shadow_maps_enabled: true, range: 10.0, ..Default::default() }, GlobalTransform::default(), ViewVisibility::VISIBLE))
            .id();
        let batch = app.world_mut().spawn(GpuBatchedMesh3d { mesh: bevy_asset::Handle::default(), max_capacity: 16, ranges: Vec::new() }).id();
        (app, light, batch)
    }

    fn frame(app: &App) -> &render::CacheFrame {
        app.world().resource::<render::CacheFrame>()
    }

    /// The render world confirms all six faces of the light's current capture.
    fn confirm(app: &mut App, light: Entity) {
        let generation = app.world().resource::<Lights>().map[&light].generation;
        let shared = app.world().resource::<SharedCaptures>().clone();
        shared.0.lock().unwrap().insert(light, (generation, 0x3F));
    }

    fn is_valid(app: &App, light: Entity) -> bool {
        frame(app).lights.iter().any(|l| l.light == light && !l.capture)
    }

    #[test]
    fn batches_are_captured_as_statics_and_kept_out_of_cached_faces() {
        let (mut app, light, batch) = lamp_and_batch();
        // Frame 1 sizes the texture (everything recaptures); frame 2 captures.
        app.update();
        app.update();
        let request = frame(&app).captures.iter().find(|c| c.light == light).expect("the lamp captures");
        for face in &request.faces {
            assert!(face.iter().any(|(_, main)| main.id() == batch), "the batch is in every capture face");
        }
        confirm(&mut app, light);
        app.update();
        assert!(is_valid(&app, light), "cached after the confirmed capture");
        let cached = frame(&app).lights.iter().find(|l| l.light == light).unwrap();
        assert_eq!(cached.movers, 0, "no face keeps a Bevy view for the batch");
        let faces = app.world().get::<CubemapVisibleEntities>(light).unwrap();
        assert!(faces.iter().all(|f| !f.entities.contains(&batch)), "the batch is drawn from the cache, not live");
    }

    #[test]
    fn reported_change_recaptures_only_the_lights_it_reaches() {
        let (mut app, light, _) = lamp_and_batch();
        app.update();
        app.update();
        confirm(&mut app, light);
        app.update();
        assert!(is_valid(&app, light));
        // Out of range (12 > 10 + 1): stays cached.
        app.world_mut().write_message(ShadowCastersChanged { center: Vec3::new(12.0, 0.0, 0.0), radius: 1.0 });
        app.update();
        assert!(is_valid(&app, light));
        assert_eq!(app.world().resource::<ShadowCacheStats>().touched, 0);
        // A cell streaming in next to it: back to capture.
        app.world_mut().write_message(ShadowCastersChanged::from_aabb(Vec3::new(8.0, -1.0, -1.0), Vec3::new(14.0, 1.0, 1.0)));
        app.update();
        assert_eq!(app.world().resource::<ShadowCacheStats>().touched, 1);
        assert!(frame(&app).captures.iter().any(|c| c.light == light), "recapturing");
        assert!(!is_valid(&app, light));
    }


    /// A GPU-culled (`NoCpuCulling`) static at `x` on the lamp's +X axis.
    fn gpu_static(app: &mut App, x: f32) -> Entity {
        app.world_mut()
            .spawn((
                Mesh3d::default(),
                GlobalTransform::from_xyz(x, 0.0, 0.0),
                Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5)),
                InheritedVisibility::VISIBLE,
                NoCpuCulling,
            ))
            .id()
    }

    /// Statics-cpu-cost slice 1: Bevy's light scan never lists a `NoCpuCulling` caster; the
    /// cache's own list for the lamp holds the ones in range, its capture draws them, and its
    /// cached faces leave them out (they are in the cached depth).
    #[test]
    fn gpu_culled_statics_are_listed_captured_and_kept_out_of_cached_faces() {
        let (mut app, light, _) = lamp_and_batch();
        // Bevy's face frusta (the scan itself isn't run here: it would skip these casters).
        app.add_systems(PostUpdate, bevy_light::update_point_light_frusta.before(ShadowCacheSystems));
        let near = gpu_static(&mut app, 3.0);
        let far = gpu_static(&mut app, 30.0);
        app.update();
        app.update();
        let lists = app.world().resource::<GpuCulledLists>();
        let members = &lists.0[&light].members;
        assert!(members.contains(&near), "the static in range is on the lamp's list");
        assert!(!members.contains(&far), "the one 30 m out (range 10) is not");
        assert!(app.world().resource::<Casters>().map[&near].gpu_culled);
        assert!(!app.world().resource::<Casters>().moving.contains(&near), "a GPU-culled static is baked at once");
        let request = frame(&app).captures.iter().find(|c| c.light == light).expect("the lamp captures");
        let in_capture = |e: Entity| request.faces.iter().filter(|f| f.iter().any(|(_, m)| m.id() == e)).count();
        assert!(in_capture(near) >= 1, "the capture draws the static (in the +X face at least)");
        assert!(in_capture(near) < 6, "only in the faces whose frustum holds it");
        assert_eq!(in_capture(far), 0);
        confirm(&mut app, light);
        app.update();
        assert!(is_valid(&app, light), "cached after the confirmed capture");
        let faces = app.world().get::<CubemapVisibleEntities>(light).unwrap();
        assert!(faces.iter().all(|f| !f.entities.contains(&near)), "drawn from the cache, not live");
        assert_eq!(frame(&app).lights.iter().find(|l| l.light == light).unwrap().movers, 0);
    }

    /// With the cache off, Bevy draws the lamp whole: its faces must hold the GPU-culled statics.
    #[test]
    fn gpu_culled_statics_join_the_faces_of_lamps_bevy_draws_whole() {
        let (mut app, light, _) = lamp_and_batch();
        app.add_systems(PostUpdate, bevy_light::update_point_light_frusta.before(ShadowCacheSystems));
        app.world_mut().resource_mut::<ShadowCacheSettings>().enabled = false;
        let near = gpu_static(&mut app, 3.0);
        app.update();
        let faces = app.world().get::<CubemapVisibleEntities>(light).unwrap();
        assert!(faces.iter().any(|f| f.entities.contains(&near)));
        assert!(faces.iter().all(|f| f.entities.windows(2).all(|w| w[0] < w[1])), "lists stay sorted, no duplicates");
        app.update();
        let faces = app.world().get::<CubemapVisibleEntities>(light).unwrap();
        assert_eq!(faces.iter().map(|f| f.entities.iter().filter(|e| **e == near).count()).max(), Some(1), "added once per face");
    }

    #[test]
    fn lod_range_is_measured_from_the_shadow_lod_origin() {
        let k = Caster {
            sphere: Sphere { center: Vec3::new(0.0, 5.0, 0.0), radius: 1.0 },
            until: 0.0,
            moving: false,
            at: Affine3A::from_translation(Vec3::new(10.0, 0.0, 0.0)),
            visible: true,
            why: 0,
            settled_at: 0.0,
            gpu_culled: true,
            lod: Some((5.0, 20.0, false)),
            layers: None,
        };
        assert!(k.in_lod_range(None), "no origin: no LOD test (the GPU still range-culls)");
        assert!(k.in_lod_range(Some(Vec3::ZERO)), "10 m: shown");
        assert!(!k.in_lod_range(Some(Vec3::new(8.0, 0.0, 0.0))), "2 m: before its start");
        assert!(!k.in_lod_range(Some(Vec3::new(-15.0, 0.0, 0.0))), "25 m: past its end");
    }

    #[test]
    fn sphere_touches_light_range() {
        let s = Sphere { center: Vec3::new(5.0, 0.0, 0.0), radius: 1.0 };
        assert!(s.touches(Vec3::ZERO, 4.5));
        assert!(!s.touches(Vec3::ZERO, 3.5));
    }
}
