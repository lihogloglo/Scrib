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
use sf_physics::{BodyDesc, BodyId, PhysicsParams, PhysicsWorld, PhysicsWorlds};
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, StreamingAnchor, StreamingSystems, from_grid, transform_to_grid};

#[cfg(feature = "debug")]
mod debug;
pub mod stream;
#[cfg(test)]
mod tests;

#[cfg(feature = "debug")]
pub use debug::PhysicsDebug;
pub use sf_physics;
pub use stream::{BodyLoader, PhysicsBodySource, PhysicsHook, PhysicsShapeSource, ShapeLoader, hook_physics};

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
            insert_batch: 32,
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
}

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
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysicsSystems {
    /// `Update`, after `StreamingSystems::Drive`: activation inserts.
    Stream,
    /// `FixedUpdate`: the step.
    Step,
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
        app.insert_resource(Physics { worlds: PhysicsWorlds::default(), params: self.params.clone() })
            .insert_resource(self.settings.clone())
            .insert_resource(Time::<Fixed>::from_hz(self.settings.hz))
            .init_resource::<PhysicsFocus>()
            .init_resource::<PhysicsStats>()
            .init_resource::<stream::PhysicsStreaming>()
            .init_resource::<BodyEntities>()
            .add_systems(First, begin_frame)
            .add_systems(FixedUpdate, step.in_set(PhysicsSystems::Step))
            .add_systems(
                Update,
                (
                    (sync_active, stream::apply_inserts, stream::terrain_chunks)
                        .chain()
                        .in_set(PhysicsSystems::Stream)
                        .after(StreamingSystems::Drive),
                    (track_bodies, write_back, publish).chain().in_set(PhysicsSystems::WriteBack).after(PhysicsSystems::Stream),
                ),
            );
        #[cfg(feature = "debug")]
        debug::add(app);
    }
}

fn begin_frame(mut stats: ResMut<PhysicsStats>) {
    let s = &mut *stats;
    (s.steps, s.dropped_steps, s.step_ms, s.max_step_ms, s.rebases, s.inserts, s.insert_ms, s.removals) = (0, 0, 0.0, 0.0, 0, 0, 0.0, 0);
}

/// The active worldspace and the focus.
#[allow(clippy::needless_pass_by_value)]
fn sync_active(
    active: Res<ActiveWorldspace>,
    anchors: Query<(&CellCoord, &Transform), With<StreamingAnchor>>,
    mut physics: ResMut<Physics>,
    mut focus: ResMut<PhysicsFocus>,
) {
    if physics.worlds.active() != active.0.as_ref() {
        physics.worlds.set_active(active.0.clone());
    }
    if let Some((cell, t)) = anchors.iter().next() {
        focus.0 = Some(from_grid(cell, t));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn step(mut physics: ResMut<Physics>, focus: Res<PhysicsFocus>, settings: Res<PhysicsSettings>, mut stats: ResMut<PhysicsStats>) {
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
    // No consumers yet (impact sounds, P5 impacts): keep the queue bounded.
    world.drain_events().for_each(drop);
    let ms = started.elapsed().as_secs_f32() * 1e3;
    stats.steps += 1;
    stats.step_ms += ms;
    stats.max_step_ms = stats.max_step_ms.max(ms);
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
fn write_back(physics: Res<Physics>, time: Res<Time<Fixed>>, mut bodies: Query<(&PhysicsBody, &mut CellCoord, &mut Transform)>) {
    let alpha = time.overstep_fraction();
    for (b, mut cell, mut t) in &mut bodies {
        let Some(w) = physics.worlds.get(&b.worldspace) else { continue };
        if w.is_sleeping(b.id) != Some(false) {
            continue;
        }
        let Some(pose) = w.pose_at(b.id, alpha) else { continue };
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
    streaming: Res<stream::PhysicsStreaming>,
    mut stats: ResMut<PhysicsStats>,
    counters: Option<Res<DiagCounters>>,
) {
    stats.bodies = physics.worlds.body_count();
    stats.awake = physics.worlds.active_world().map_or(0, PhysicsWorld::awake_count);
    stats.pending = streaming.pending_count();
    stats.terrain_chunks = streaming.terrain_count();
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
}
