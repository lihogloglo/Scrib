//! Cached point-light shadow maps (superfargoth `docs/specs/shadows.md`, "Caching").
//!
//! Bevy 0.20 clears and redraws all six cube faces of every shadowed point light
//! every frame, and the cost is mostly render-thread work (six views queued,
//! specialized and encoded per light). This crate keeps, per shadowed light, the
//! depth of its **static** casters in a persistent cube array of our own and hands
//! it back to Bevy each frame:
//!
//! - *Capture* (a light gains a slot, stops moving, or a static caster in its
//!   range changes): Bevy's views for that light see static casters only; after
//!   Bevy's shadow pass a depth-copy draw stores the six faces in our cache.
//! - *Cached*: Bevy's views see **moving** casters only. Before Bevy's shadow pass
//!   a depth-copy draw restores the cached faces into Bevy's cube layer (which
//!   turns Bevy's clear into a load), so Bevy draws the movers over the copy and
//!   its sampling (filters, PCSS, clusters) is unchanged. A cached face with no
//!   mover in it loses its Bevy view altogether (shadows v2 V1): each Bevy face
//!   view runs the whole `Core3d` schedule (~0.35 ms of render thread), so
//!   those faces are restored by one pass in the camera's schedule instead.
//! - A moving light is left to Bevy (full redraw) until it stops.
//!
//! Casters are classified automatically: skinned meshes and anything marked
//! [`ShadowMobility::Movable`] always move; any other mesh whose transform or
//! visibility changed counts as moving until it has been still for
//! [`ShadowCacheSettings::settle_time`] after it appeared, then it is baked
//! into the lights it touches; a baked caster that moves stays a mover from
//! then on (at most one bake and one unbake per caster). A torch's flicker
//! changes intensity only: no recapture.


mod render;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::embedded_asset;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, VisibilitySystems};
use bevy_ecs::prelude::*;
use bevy_light::{NotShadowCaster, PointLight, PointLightShadowMap, SimulationLightSystems};
use bevy_math::Vec3;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::Mesh3d;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;

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
}

impl Default for ShadowCacheSettings {
    fn default() -> Self {
        Self { enabled: true, max_lights: 24, settle_time: 1.0, move_threshold: 0.01 }
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
    /// Captures completed since start (each is six faces).
    pub captures: u64,
}

/// The plugin.
pub struct ShadowCachePlugin;

impl Plugin for ShadowCachePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "copy_depth.wesl");
        let shared = SharedCaptures::default();
        app.init_resource::<ShadowCacheSettings>()
            .init_resource::<ShadowCacheStats>()
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
                    .chain(),
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
}

#[derive(Resource, Default)]
struct Casters {
    map: HashMap<Entity, Caster>,
    moving: HashSet<Entity>,
    /// Spheres whose baked depth changed this frame.
    dirty: Vec<Sphere>,
}

type CasterItem<'a> = (Entity, &'a GlobalTransform, Option<&'a Aabb>, Option<&'a ShadowMobility>, Has<SkinnedMesh>);
type CasterChanged = (With<Mesh3d>, Without<NotShadowCaster>, Or<(Changed<GlobalTransform>, Changed<InheritedVisibility>, Changed<Aabb>)>);

/// Classifies casters (moving or baked) and collects the spheres whose baked
/// depth changed: a baked caster that moved (old and new place), a caster
/// that settled, a baked caster that went away.
fn track_casters(
    time: Res<Time>,
    settings: Res<ShadowCacheSettings>,
    mut casters: ResMut<Casters>,
    changed: Query<CasterItem, CasterChanged>,
    mut removed: RemovedComponents<Mesh3d>,
) {
    let now = time.elapsed_secs();
    let c = &mut *casters;
    c.dirty.clear();
    for entity in removed.read() {
        if let Some(old) = c.map.remove(&entity) {
            c.moving.remove(&entity);
            if !old.moving {
                c.dirty.push(old.sphere);
            }
        }
    }
    for (entity, transform, aabb, mobility, skinned) in &changed {
        let sphere = Sphere::of(transform, aabb);
        let mobility = mobility.copied().unwrap_or_default();
        let was_baked = c.map.get(&entity).filter(|k| !k.moving).map(|k| k.sphere);
        if let Some(old) = was_baked {
            c.dirty.push(old);
        }
        // A baked caster that moves stays a mover (doors, items, animated parts that
        // start and stop): each caster costs at most one bake and one unbake, so
        // intermittent movers can't keep the lights recapturing.
        let always = mobility == ShadowMobility::Movable || (mobility == ShadowMobility::Auto && (skinned || was_baked.is_some()));
        let caster = if mobility == ShadowMobility::Static {
            c.dirty.push(sphere);
            c.moving.remove(&entity);
            Caster { sphere, until: now, moving: false }
        } else {
            c.moving.insert(entity);
            Caster { sphere, until: if always { f32::INFINITY } else { now + settings.settle_time }, moving: true }
        };
        c.map.insert(entity, caster);
    }
    let Casters { map, moving, dirty } = c;

    moving.retain(|entity| {
        let Some(k) = map.get_mut(entity) else { return false };
        if k.until <= now {
            k.moving = false;
            dirty.push(k.sphere);
            false
        } else {
            true
        }
    });
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

    fn invalidate_all(&mut self) {
        for light in self.map.values_mut() {
            light.state = SlotState::Capture;
            light.generation = light.generation.wrapping_add(1);
        }
    }
}

/// Keeps a light's face lists to static casters (capturing) or movers
/// (restoring); returns the faces left with something to draw (bit per face).
fn filter_faces(faces: &mut CubemapVisibleEntities, moving: &HashSet<Entity>, capture: bool) -> u8 {
    let mut drawn = 0u8;
    for (i, face) in faces.iter_mut().enumerate() {
        face.entities.retain(|e| moving.contains(e) != capture);
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
#[allow(clippy::too_many_arguments)]
fn classify_lights(
    settings: Res<ShadowCacheSettings>,
    map_size: Option<Res<PointLightShadowMap>>,
    casters: Res<Casters>,
    shared: Res<SharedCaptures>,
    mut lights: ResMut<Lights>,
    mut frame: ResMut<render::CacheFrame>,
    mut stats: ResMut<ShadowCacheStats>,
    mut query: Query<(Entity, &PointLight, &GlobalTransform, &mut CubemapVisibleEntities)>,
    mut removed: RemovedComponents<PointLight>,
) {
    frame.lights.clear();
    let l = &mut *lights;
    l.frame += 1;
    for entity in removed.read() {
        if let Some(gone) = l.map.remove(&entity) {
            l.free.push(gone.slot);
        }
    }
    let size = map_size.map_or(1024, |m| u32::try_from(m.size).unwrap_or(1024));
    if !settings.enabled || size != l.map_size {
        // Off, or a new face size: every cached face is stale.
        l.map_size = size;
        l.invalidate_all();
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
    let mut counts = ShadowCacheStats { captures: stats.captures + confirmed, ..ShadowCacheStats::default() };
    for (entity, light, transform, mut faces) in &mut query {
        if !light.shadow_maps_enabled {
            continue;
        }
        let position = transform.translation();
        if !l.map.contains_key(&entity) {
            let Some(slot) = l.allocate(max) else {
                counts.uncached += 1;
                continue;
            };
            l.map.insert(entity, LightSlot { slot, position, range: light.range, state: SlotState::Capture, generation: 0, used: 0 });
        }
        let frame_no = l.frame;
        let threshold = settings.move_threshold;
        let Some(cached) = l.map.get_mut(&entity) else { continue };
        cached.used = frame_no;
        let moved = cached.position.distance(position) > threshold || (cached.range - light.range).abs() > threshold;
        let touched = casters.dirty.iter().any(|s| s.touches(position, light.range));
        if moved || touched {
            cached.generation = cached.generation.wrapping_add(1);
            cached.position = position;
            cached.range = light.range;
            cached.state = if moved { SlotState::Moving } else { SlotState::Capture };
        } else if cached.state == SlotState::Moving {
            cached.state = SlotState::Capture;
        }
        let capture = match cached.state {
            SlotState::Moving => {
                counts.uncached += 1;
                continue;
            }
            SlotState::Capture => true,
            SlotState::Valid => false,
        };
        let movers = filter_faces(&mut faces, &casters.moving, capture);
        if capture {
            counts.capturing += 1;
        } else {
            counts.cached += 1;
        }
        frame.lights.push(render::CachedLight { light: entity, slot: cached.slot, capture, generation: cached.generation, movers });
    }
    let capacity = capacity_for(l.next, max);
    if capacity != l.capacity {
        // A new texture: everything recaptures (the frame already sent
        // restores for this frame's lights, so turn them into captures).
        l.capacity = capacity;
        l.invalidate_all();
        frame.lights.clear();
    }
    frame.map_size = size;
    frame.slots = l.capacity;
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
        let slot = |slot, used| LightSlot { slot, position: Vec3::ZERO, range: 1.0, state: SlotState::Valid, generation: 0, used };
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
