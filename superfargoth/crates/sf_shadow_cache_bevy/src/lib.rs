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


mod render;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::embedded_asset;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, ViewVisibility, VisibilitySystems};
use bevy_ecs::entity::{EntityHashMap, EntityHashSet};
use bevy_ecs::prelude::*;
use bevy_light::{NotShadowCaster, PointLight, PointLightShadowMap, SimulationLightSystems};
use bevy_math::{Affine3A, Vec3, Vec3A};
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::Mesh3d;
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

/// The cache's main-world systems (`PostUpdate`).
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShadowCacheSystems;

/// The plugin.
pub struct ShadowCachePlugin;

impl Plugin for ShadowCachePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "copy_depth.wesl");
        let shared = SharedCaptures::default();
        app.init_resource::<ShadowCacheSettings>()
            .init_resource::<ShadowCacheStats>()
            .init_resource::<ShadowOriginShift>()
            .init_resource::<Casters>()
            .init_resource::<Lights>()
            .init_resource::<render::CacheFrame>()
            .insert_resource(shared.clone())
            .add_systems(
                PostUpdate,
                (
                    track_casters
                        .after(TransformSystems::Propagate)
                        .after(VisibilitySystems::VisibilityPropagate)
                        .after(VisibilitySystems::CalculateBounds),
                    classify_lights.after(SimulationLightSystems::CheckLightVisibility),
                )
                    .chain()
                    .in_set(ShadowCacheSystems),
            );
        render::build(app, shared);
    }

    fn finish(&self, app: &mut App) {
        render::finish(app);
    }
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
}

#[derive(Resource, Default)]
struct Casters {
    map: EntityHashMap<Caster>,
    moving: EntityHashSet,
    /// Spheres whose baked depth changed this frame.
    dirty: Vec<(Sphere, Entity)>,
    /// Dirty events since start: removed, moved after bake, static changed, settled.
    kinds: [u64; 4],
}

type CasterItem<'a> = (Entity, &'a GlobalTransform, Option<Ref<'a, Aabb>>, Option<&'a ShadowMobility>, Has<SkinnedMesh>, &'a InheritedVisibility);
type CasterChanged = (With<Mesh3d>, Without<NotShadowCaster>, Or<(Changed<GlobalTransform>, Changed<InheritedVisibility>, Changed<Aabb>)>);

/// Classifies casters (moving or baked) and collects the spheres whose baked
/// depth changed: a baked caster that moved (old and new place), a caster
/// that settled, a baked caster that went away.
fn track_casters(
    time: Res<Time>,
    settings: Res<ShadowCacheSettings>,
    shift: Res<ShadowOriginShift>,
    mut casters: ResMut<Casters>,
    changed: Query<CasterItem, CasterChanged>,
    mut removed: RemovedComponents<Mesh3d>,
    ancestors: Query<(Option<&ShadowMobility>, Option<&ChildOf>)>,
) {
    let now = time.elapsed_secs();
    let c = &mut *casters;
    c.dirty.clear();
    if shift.0 != Vec3::ZERO {
        for k in c.map.values_mut() {
            k.sphere.center += shift.0;
            k.at.translation += Vec3A::from(shift.0);
        }
    }
    for entity in removed.read() {
        if let Some(old) = c.map.remove(&entity) {
            c.moving.remove(&entity);
            if !old.moving {
                c.dirty.push((old.sphere, entity));
                c.kinds[0] += 1;
            }
        }
    }
    for (entity, transform, aabb, mobility, skinned, visibility) in &changed {
        let (at, visible) = (transform.affine(), visibility.get());
        let aabb_changed = aabb.as_ref().is_some_and(DetectChanges::is_changed);
        if !aabb_changed
            && c.map
                .get(&entity)
                .is_some_and(|k| k.visible == visible && k.at.abs_diff_eq(at, 1e-3))
        {
            continue;
        }
        let sphere = Sphere::of(transform, aabb.as_deref());
        // Change detection fires on writes, not moves (animation and propagation rewrite
        // unchanged transforms every frame): only a real change counts.
        if c.map.get(&entity).is_some_and(|k| k.visible == visible && (k.sphere.radius - sphere.radius).abs() < 1e-4 && k.at.abs_diff_eq(at, 1e-3)) {
            continue;
        }
        // Own marker, else the nearest ancestor's (hosts mark an actor's root: its held
        // items and attachments move with it even when they aren't skinned).
        let mobility = mobility.copied().or_else(|| ancestor_mobility(entity, &ancestors)).unwrap_or_default();
        let was_baked = c.map.get(&entity).filter(|k| !k.moving).map(|k| k.sphere);
        if let Some(old) = was_baked {
            c.dirty.push((old, entity));
            c.kinds[1] += 1;
        }
        // A baked caster that moves stays a mover (doors, items, animated parts that
        // start and stop): each caster costs at most one bake and one unbake, so
        // intermittent movers can't keep the lights recapturing.
        let always = mobility == ShadowMobility::Movable || (mobility == ShadowMobility::Auto && (skinned || was_baked.is_some()));
        let caster = if mobility == ShadowMobility::Static {
            c.dirty.push((sphere, entity));
            c.kinds[2] += 1;
            c.moving.remove(&entity);
            Caster { sphere, until: now, moving: false, at, visible, why: 0, settled_at: now }
        } else {
            c.moving.insert(entity);
            let why = if mobility == ShadowMobility::Movable { 3 } else if skinned { 1 } else if was_baked.is_some() { 2 } else { 0 };
            let settle = if settings.warmup { settings.warmup_settle_time } else { settings.settle_time };
            Caster { sphere, until: if always { f32::INFINITY } else { now + settle }, moving: true, at, visible, why, settled_at: f32::INFINITY }
        };
        c.map.insert(entity, caster);
    }
    let Casters { map, moving, dirty, kinds } = c;

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
             (settling {}, skinned {}, moved after bake {}, movable {}), e.g. {movers:?}; recaptured by {dirtied:?}; dirty events (removed, moved, static, settled) {:?}",
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

    #[test]
    fn sphere_touches_light_range() {
        let s = Sphere { center: Vec3::new(5.0, 0.0, 0.0), radius: 1.0 };
        assert!(s.touches(Vec3::ZERO, 4.5));
        assert!(!s.touches(Vec3::ZERO, 3.5));
    }
}
