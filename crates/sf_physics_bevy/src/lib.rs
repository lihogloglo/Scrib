//! Bevy glue for `sf_physics` (physics.md "Stepping and the floating origin",
//! "Streaming").
//!
//! - [`PhysicsPlugin`] steps the **active worldspace's** world in
//!   `FixedUpdate` at 60 Hz, at most [`PhysicsSettings::max_steps_per_frame`]
//!   steps per frame (beyond that physics time dilates instead of
//!   spiralling). Before each step the world follows [`PhysicsFocus`] (the
//!   streaming anchor): whole-cell rebases, independent of the render
//!   floating origin.
//! - Entities with a [`PhysicsBody`] get their interpolated pose written back
//!   as `CellCoord + Transform` each frame while awake; despawning one
//!   removes its body.
//! - Streaming ([`stream`]): [`PhysicsHook`] wraps placement spawners, so R0
//!   activation queues `Fixed` colliders (one shared mesh per model, built on
//!   the task pool) and deactivation removes them; terrain heightfield chunks
//!   (64 m, 0.5 m) follow the active worldspace's R0 cells. Insertion runs
//!   under a per-frame time budget.
//! - Counters (`physics/...`) when the app runs `SfDiagPlugin`; collider
//!   wireframes with the `debug` feature ([`PhysicsDebug`]).

use std::time::Instant;

use bevy_app::{App, First, FixedUpdate, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use bevy_transform::components::Transform;
use sf_coords::WorldPos;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;
use sf_controller::{GroundHit, GroundProbe, HeightSurface, Nearest, ProbeQuery, TerrainProbe};
use sf_physics::{BodyDesc, BodyId, Layer, PhysicsEvent, PhysicsParams, PhysicsWorld, PhysicsWorlds};
use sf_terrain_bevy::TerrainHeightsMap;
use sf_terrain_bevy::sf_terrain::TerrainHeights;
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, StreamingAnchor, StreamingSystems, from_grid, transform_to_grid};

pub mod carry;
#[cfg(feature = "controller")]
pub mod character;
#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "ragdoll")]
pub mod ragdoll;
pub mod stream;
#[cfg(test)]
mod tests;

#[cfg(feature = "controller")]
pub use character::{CharacterFellOut, CharacterPhysics, CharacterStats, ImpactOnCharacter};
#[cfg(feature = "debug")]
pub use debug::PhysicsDebug;
pub use carry::{CarryFeedback, CarryInput, CarryOutcome, HeldPoses, Holder};
pub use sf_physics;
#[cfg(feature = "ragdoll")]
pub use ragdoll::{LYING_REST_GET_UP, Ragdoll, RagdollConfig, RagdollFailed, RagdollGotUp,RagdollLoader, RagdollMode, RagdollSource, RagdollStats, Ragdolls, clip_root_body};
#[cfg(feature = "ragdoll")]
pub use sf_ragdoll;
pub use stream::{
    BodyLoader, ItemBody, ItemContent, ItemPivot, PhysicsBodySource, PhysicsHook, PhysicsShapeSource, PhysicsStreaming, STATE_OWNER,
    STATE_VERSION, ShapeLoader, hook_items, hook_physics,
};

/// Plugin settings (physics.md defaults).
#[derive(Resource, Clone, Debug)]
pub struct PhysicsSettings {
    /// Fixed steps per second.
    pub hz: f64,
    /// More fixed steps than this in one frame are dropped (time dilates).
    pub max_steps_per_frame: u32,
    /// Main-thread time per frame for inserting bodies (shapes are built on the
    /// task pool), milliseconds.
    pub insert_budget_ms: f32,
    /// Inserts between query refreshes: each batch's broad-phase work lands in
    /// its own frame instead of piling up on the next step.
    pub insert_batch: usize,
    /// Items inserted per frame at most (after the statics' refresh; their
    /// broad-phase work lands in the next step).
    pub item_batch: usize,
    /// Items lighter than `debris_mass` (kg) and smaller than `debris_extent`
    /// (meters, largest scaled extent) are `DEBRIS`, the rest `PROP`.
    pub debris_mass: f32,
    pub debris_extent: f32,
    /// Terrain heightfield chunk edge (meters) and sample spacing.
    pub terrain_chunk: f32,
    pub terrain_spacing: f32,
    /// Off: no terrain chunks (worlds without heights, A/B runs).
    pub terrain: bool,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            hz: 60.0,
            max_steps_per_frame: 3,
            insert_budget_ms: 0.5,
            insert_batch: 16,
            item_batch: 32,
            debris_mass: 15.0,
            debris_extent: 0.6,
            terrain_chunk: 64.0,
            terrain_spacing: 0.5,
            terrain: true,
        }
    }
}

/// The physics worlds, one per worldspace; only the active one steps.
#[derive(Resource, Default)]
pub struct Physics {
    pub worlds: PhysicsWorlds<WorldspaceId>,
    pub params: PhysicsParams,
    /// The active worldspace's terrain heights (from `TerrainHeightsMap`):
    /// the controller's ground reads them analytically instead of casting
    /// against the heightfield chunks (see `impl GroundProbe for Physics`).
    pub terrain: Option<TerrainHeights>,
}

/// Layers the characters' ground and wall casts see (physics.md
/// "Controller"): statics, terrain and props; debris never blocks, and
/// characters avoid each other through ORCA, not collision.
pub const GROUND_LAYERS: Layer = Layer::STATIC.union(Layer::TERRAIN).union(Layer::PROP);

/// The controller's ground on physics ([`GROUND_LAYERS`] of the active
/// world): the host's `SfControllerPlugin::<Physics>` probes it.
///
/// With the active worldspace's heights known, terrain is read through
/// `TerrainProbe` (the exact heights the renderer uses, no marching) and
/// composed with the bodies' `STATIC | PROP` by `Nearest`; the heightfield
/// chunks stay for bodies. A sphere or capsule cast against a heightfield
/// costs about 13 µs, as much as all of a walker's other casts together
/// (P5 follow-up, `sf_physics` `probe_bench`).
impl GroundProbe for Physics {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        match (self.worlds.active_world(), &self.terrain) {
            (Some(w), Some(t)) => {
                let terrain = TerrainProbe(t);
                Nearest(&[&terrain, &w.probe(GROUND_LAYERS - Layer::TERRAIN)]).probe(queries, out);
            }
            (Some(w), None) => w.probe(GROUND_LAYERS).probe(queries, out),
            (None, _) => out.fill(None),
        }
    }
}

/// The active world's events of this frame's steps (cleared in `First`).
#[derive(Resource, Default)]
pub struct PhysicsEvents(pub Vec<PhysicsEvent>);

impl Physics {
    /// Inserts a body into `ws`'s world (created anchored at the body's cell).
    pub fn insert(&mut self, ws: &WorldspaceId, desc: &BodyDesc) -> BodyId {
        self.worlds.get_or_create(ws, desc.pose.pos, &self.params).insert(desc)
    }

    pub fn world(&self, ws: &WorldspaceId) -> Option<&PhysicsWorld> {
        self.worlds.get(ws)
    }

    pub fn world_mut(&mut self, ws: &WorldspaceId) -> Option<&mut PhysicsWorld> {
        self.worlds.get_mut(ws)
    }
}

/// Where the gameplay anchor is: worlds rebase around it. Filled from the
/// first [`StreamingAnchor`] each frame when there is one; set it yourself
/// otherwise.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PhysicsFocus(pub Option<WorldPos>);

/// An entity simulated by a body: its pose is written back while awake.
/// Removing the component (or despawning) removes the body.
#[derive(Component, Clone, Debug)]
pub struct PhysicsBody {
    pub worldspace: WorldspaceId,
    pub id: BodyId,
}

/// Per-frame numbers (also published as `physics/...` counters).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PhysicsStats {
    /// Fixed steps run and dropped this frame.
    pub steps: u32,
    pub dropped_steps: u32,
    /// Time in `PhysicsWorld::step` (and rebases) this frame, ms.
    pub step_ms: f32,
    pub max_step_ms: f32,
    pub rebases: u32,
    /// Bodies inserted this frame and the main-thread time it took (inserts +
    /// query refreshes), ms.
    pub inserts: u32,
    pub insert_ms: f32,
    pub removals: u32,
    /// Waiting for their shape or their turn.
    pub pending: usize,
    pub bodies: usize,
    pub awake: usize,
    pub terrain_chunks: usize,
    /// Item bodies (asleep, awake or held).
    pub items: usize,
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysicsSystems {
    /// `Update`, after `StreamingSystems::Drive`: activation inserts.
    Stream,
    /// `FixedUpdate`: the step.
    Step,
    /// `Update`, between `Stream` and `WriteBack`: holders' views, carry input,
    /// held items' holder-space poses. Order the holder's view (camera)
    /// before it.
    Carry,
    /// `RunFixedMainLoop`, before the fixed steps: holders' views (the steps
    /// move the hands toward them). Order camera controllers that run there
    /// before it.
    Views,
    /// `Update`, after `Stream`: interpolated poses to transforms.
    WriteBack,
}

/// Fixed-step physics for the active worldspace (see the crate docs).
#[derive(Default)]
pub struct PhysicsPlugin {
    pub params: PhysicsParams,
    pub settings: PhysicsSettings,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Physics { worlds: PhysicsWorlds::default(), params: self.params.clone(), terrain: None })
            .insert_resource(self.settings.clone())
            .insert_resource(Time::<Fixed>::from_hz(self.settings.hz))
            .init_resource::<PhysicsFocus>()
            .init_resource::<PhysicsStats>()
            .init_resource::<PhysicsStreaming>()
            .init_resource::<BodyEntities>()
            .init_resource::<PhysicsEvents>()
            .add_systems(First, begin_frame)
            .add_systems(FixedUpdate, step.in_set(PhysicsSystems::Step))
            .add_systems(
                Update,
                (
                    (sync_active, stream::apply_inserts, stream::terrain_chunks)
                        .chain()
                        .in_set(PhysicsSystems::Stream)
                        .after(StreamingSystems::Drive),
                    (track_bodies, write_back, stream::follow_items, publish, stream::coverage)
                        .chain()
                        .in_set(PhysicsSystems::WriteBack)
                        .after(PhysicsSystems::Stream),
                ),
            );
        carry::add(app);
        #[cfg(feature = "debug")]
        debug::add(app);
        #[cfg(feature = "controller")]
        character::add(app);
        #[cfg(feature = "ragdoll")]
        ragdoll::add(app);
    }
}

fn begin_frame(mut stats: ResMut<PhysicsStats>, mut events: ResMut<PhysicsEvents>) {
    events.0.clear();
    let s = &mut *stats;
    (s.steps, s.dropped_steps, s.step_ms, s.max_step_ms, s.rebases, s.inserts, s.insert_ms, s.removals) = (0, 0, 0.0, 0.0, 0, 0, 0.0, 0);
}

/// The active worldspace, its terrain heights and the focus.
#[allow(clippy::needless_pass_by_value)]
fn sync_active(
    active: Res<ActiveWorldspace>,
    anchors: Query<(&CellCoord, &Transform), With<StreamingAnchor>>,
    heights: Option<Res<TerrainHeightsMap>>,
    mut physics: ResMut<Physics>,
    mut focus: ResMut<PhysicsFocus>,
) {
    let switched = physics.worlds.active() != active.0.as_ref();
    if switched {
        physics.worlds.set_active(active.0.clone());
    }
    if switched || heights.as_ref().is_some_and(Res::is_changed) {
        physics.terrain = heights.as_ref().zip(active.0.as_ref()).and_then(|(h, ws)| h.0.get(ws).cloned());
    }
    if let Some((cell, t)) = anchors.iter().next() {
        focus.0 = Some(from_grid(cell, t));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn step(
    mut physics: ResMut<Physics>,
    focus: Res<PhysicsFocus>,
    settings: Res<PhysicsSettings>,
    mut stats: ResMut<PhysicsStats>,
    mut events: ResMut<PhysicsEvents>,
) {
    if stats.steps >= settings.max_steps_per_frame {
        stats.dropped_steps += 1;
        return;
    }
    let Some(world) = physics.worlds.active_world_mut() else { return };
    let started = Instant::now();
    if let Some(f) = focus.0
        && world.follow(f)
    {
        stats.rebases += 1;
    }
    world.step();
    // This frame's events (P5 impacts, contact reports; cleared in `First`).
    let drained: Vec<PhysicsEvent> = world.drain_events().collect();
    let world = &*world;
    events.0.extend(drained.into_iter().map(|e| terrain_surfaces(world, physics.terrain.as_ref(), e)));
    let ms = started.elapsed().as_secs_f32() * 1e3;
    stats.steps += 1;
    stats.step_ms += ms;
    stats.max_step_ms = stats.max_step_ms.max(ms);
}

/// A contact report against a terrain chunk carries the terrain's surface under the contact
/// (`HeightSurface::surface`, the ground probe's: footsteps and impacts agree) instead of the
/// chunk body's untagged one.
fn terrain_surfaces(world: &PhysicsWorld, terrain: Option<&TerrainHeights>, event: PhysicsEvent) -> PhysicsEvent {
    let (PhysicsEvent::Contact { a, b, impulse, speed, slide, point, normal, mut surfaces }, Some(t)) = (event, terrain) else {
        return event;
    };
    for (side, body) in [a, b].into_iter().enumerate() {
        if world.layer(body).is_some_and(|l| l.contains(Layer::TERRAIN)) {
            surfaces[side] = HeightSurface::surface(t, point);
        }
    }
    PhysicsEvent::Contact { a, b, impulse, speed, slide, point, normal, surfaces }
}

/// Which body each entity owns, so despawns remove bodies.
#[derive(Resource, Default)]
struct BodyEntities(std::collections::HashMap<Entity, PhysicsBody>);

fn track_bodies(
    added: Query<(Entity, &PhysicsBody), Changed<PhysicsBody>>,
    mut removed: RemovedComponents<PhysicsBody>,
    mut map: ResMut<BodyEntities>,
    mut physics: ResMut<Physics>,
) {
    for e in removed.read() {
        if let Some(b) = map.0.remove(&e)
            && let Some(w) = physics.worlds.get_mut(&b.worldspace)
        {
            w.remove(b.id);
        }
    }
    for (e, b) in &added {
        map.0.insert(e, b.clone());
    }
}

/// Interpolated poses of awake bodies → grid transforms.
#[allow(clippy::needless_pass_by_value)]
fn write_back(
    physics: Res<Physics>,
    time: Res<Time<Fixed>>,
    held: Res<HeldPoses>,
    mut bodies: Query<(&PhysicsBody, &mut CellCoord, &mut Transform)>,
) {
    let alpha = time.overstep_fraction();
    for (b, mut cell, mut t) in &mut bodies {
        let Some(w) = physics.worlds.get(&b.worldspace) else { continue };
        if w.is_sleeping(b.id) != Some(false) {
            continue;
        }
        let carried = held.0.get(&b.id).filter(|_| physics.worlds.active() == Some(&b.worldspace));
        let Some(pose) = carried.copied().or_else(|| w.pose_at(b.id, alpha)) else { continue };
        let (c, nt) = transform_to_grid(&pose);
        if *cell != c {
            *cell = c;
        }
        t.translation = nt.translation;
        t.rotation = nt.rotation;
    }
}

#[allow(clippy::needless_pass_by_value, clippy::cast_precision_loss)]
fn publish(
    physics: Res<Physics>,
    streaming: Res<PhysicsStreaming>,
    mut stats: ResMut<PhysicsStats>,
    counters: Option<Res<DiagCounters>>,
) {
    stats.bodies = physics.worlds.body_count();
    stats.awake = physics.worlds.active_world().map_or(0, PhysicsWorld::awake_count);
    stats.pending = streaming.pending_count();
    stats.terrain_chunks = streaming.terrain_count();
    stats.items = streaming.item_count();
    let Some(c) = counters else { return };
    let s = *stats;
    c.add_named("physics/step/ms", unit::MS, f64::from(s.step_ms));
    c.add_named("physics/steps", unit::COUNT, f64::from(s.steps));
    c.add_named("physics/steps_dropped", unit::COUNT, f64::from(s.dropped_steps));
    c.add_named("physics/rebases", unit::COUNT, f64::from(s.rebases));
    c.add_named("physics/inserts", unit::COUNT, f64::from(s.inserts));
    c.add_named("physics/insert/ms", unit::MS, f64::from(s.insert_ms));
    c.add_named("physics/removals", unit::COUNT, f64::from(s.removals));
    c.set_named("physics/bodies", unit::COUNT, s.bodies as f64);
    c.set_named("physics/awake", unit::COUNT, s.awake as f64);
    c.set_named("physics/pending", unit::COUNT, s.pending as f64);
    c.set_named("physics/terrain/chunks", unit::COUNT, s.terrain_chunks as f64);
    c.set_named("physics/items", unit::COUNT, s.items as f64);
}
