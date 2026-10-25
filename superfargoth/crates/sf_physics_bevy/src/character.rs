//! Characters on physics (physics.md "Controller", P5; feature `controller`).
//!
//! Every [`Mover`] that isn't [`Dormant`] gets a kinematic capsule in the
//! active worldspace's physics world ([`sf_physics::BodyDesc::character`]),
//! targeted at the mover's feet after each mover step, so falling and thrown
//! bodies hit characters. After the step, the bodies the capsule touches get
//! capped push impulses ([`sf_physics::PhysicsWorld::push`]) from the intent
//! the mover ran with. Hits on a capsule ([`PhysicsEvent::Hit`]: the
//! incoming body's momentum at first contact) of at least
//! [`CharacterPhysics::impact_threshold`] become [`ImpactOnCharacter`]
//! messages for gameplay (hit reactions, knockdowns). The mover itself keeps
//! running in `Update` against the physics world's queries: the host makes
//! [`crate::Physics`] its `SfControllerPlugin` ground.

use std::collections::HashMap;

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec3};
use bevy_time::Time;
use bevy_transform::components::Transform;
use sf_controller_bevy::{ControllerSettings, ControllerSystems, Dormant, Mover, MoverMode};
use sf_coords::WorldPos;
use sf_physics::{BodyDesc, BodyId, CharacterShape, PhysicsEvent, PushParams};
use sf_world::WorldspaceId;

use crate::{Physics, PhysicsEvents, PhysicsSystems};

/// Push and impact tuning (physics.md defaults).
#[derive(Resource, Clone, Debug)]
pub struct CharacterPhysics {
    pub push: PushParams,
    /// Impact momentum on a capsule (N·s) from which it counts as an impact
    /// (a 3 kg ball at 1/3 m/s; soft hits drive hit reactions).
    pub impact_threshold: f32,
}

impl Default for CharacterPhysics {
    fn default() -> Self {
        Self { push: PushParams::default(), impact_threshold: 1.0 }
    }
}

/// Something hit a character hard enough to react (physics.md "Impacts").
#[derive(Message, Clone, Debug)]
pub struct ImpactOnCharacter {
    pub entity: Entity,
    /// Impact momentum, N·s ([`sf_physics::impact_impulse`]).
    pub impulse: f32,
    pub point: WorldPos,
    /// The impulse's direction on the character (unit, world; zero if unknown).
    pub direction: Vec3,
}

/// Counters for the log and benches.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CharacterStats {
    pub capsules: usize,
    /// Bodies pushed this frame and the largest push, N·s.
    pub pushed: u32,
    pub max_push: f32,
    pub impacts: u32,
    /// Characters returned from under the kill height since start ([`CharacterFellOut`]).
    pub fell_out: u32,
}

/// Each character's capsule: its world, body and shape, and the mover's feet
/// at the last mover step with their world velocity and the virtual time then,
/// from which each fixed step targets the feet at its own time.
#[derive(Resource, Default)]
pub(crate) struct Capsules(pub(crate) HashMap<Entity, Capsule>);

pub(crate) struct Capsule {
    pub(crate) worldspace: WorldspaceId,
    pub(crate) id: BodyId,
    shape: CharacterShape,
    feet: WorldPos,
    velocity: Vec3,
    time: f64,
}

pub(crate) fn add(app: &mut App) {
    app.init_resource::<CharacterPhysics>()
        .init_resource::<CharacterStats>()
        .init_resource::<Capsules>()
        .init_resource::<SafeGround>()
        .add_message::<ImpactOnCharacter>()
        .add_message::<CharacterFellOut>()
        .add_systems(Update, (fell_out, capsules, impacts).chain().after(ControllerSystems::Step).before(PhysicsSystems::WriteBack))
        .add_systems(FixedUpdate, target_capsules.before(PhysicsSystems::Step));
}

/// A walking character fell below the active world's kill height and was
/// returned to its last walkable ground (physics.md "Fell out"): the
/// out-of-world safety net (Unreal's `KillZ` / `FellOutOfWorld`). The host
/// logs it; it means collision was missing under the character.
#[derive(Message, Clone, Debug)]
pub struct CharacterFellOut {
    pub entity: Entity,
    /// Where it was caught.
    pub from: WorldPos,
    /// Where it was put back: its last walkable ground, else where it was first seen.
    pub to: WorldPos,
}

/// How far under resident terrain a character counts as fallen out, m
/// (physics.md "Fell out": `TerrainHeights` − 2 m).
const TERRAIN_KILL_DEPTH: f32 = 2.0;

/// Each simulating character's last walkable ground (first: where it was first seen).
#[derive(Resource, Default)]
struct SafeGround(HashMap<Entity, WorldPos>);

/// Fell-out recovery for walking characters, after the mover step: in the
/// air more than [`TERRAIN_KILL_DEPTH`] under the resident terrain, or where
/// there is none under the active world's
/// [`sf_physics::PhysicsWorld::kill_height`] (10 m under its lowest fixed
/// body), a `Ground` mover goes back to its last walkable
/// ground with no fall speed and re-grounds there with the next step.
/// Flying and swimming movers have no gravity and are left alone.
#[allow(clippy::needless_pass_by_value)]
fn fell_out(
    physics: Res<Physics>,
    mut safe: ResMut<SafeGround>,
    mut stats: ResMut<CharacterStats>,
    mut out: MessageWriter<CharacterFellOut>,
    mut movers: Query<(Entity, &mut Mover, &mut Transform), Without<Dormant>>,
) {
    safe.0.retain(|e, _| movers.contains(*e));
    let kill = physics.worlds.active_world().and_then(sf_physics::PhysicsWorld::kill_height);
    for (entity, mut m, mut t) in &mut movers {
        let pos = m.loco.position;
        let walkable = m.params.min_normal_y();
        let last = *safe.0.entry(entity).or_insert(pos);
        match m.loco.grounded {
            Some(g) if g.walkable(walkable) => {
                safe.0.insert(entity, pos);
                continue;
            }
            // On ground, however steep or deep: not out of the world.
            Some(_) => continue,
            None => {}
        }
        // Under resident terrain by more than `TERRAIN_KILL_DEPTH`, else under the
        // kill height (sea floors can lie deeper than any fixed body in yet).
        let y = pos.to_f64().y;
        let below = match physics.terrain.as_ref().and_then(|t| t.height(pos)) {
            Some(h) => y < f64::from(h - TERRAIN_KILL_DEPTH),
            None => kill.is_some_and(|k| y < k),
        };
        if !below || !matches!(m.loco.mode, MoverMode::Ground) {
            continue;
        }
        let loco = &mut m.loco;
        loco.position = last;
        loco.vertical_speed = 0.0;
        loco.motion.velocity = Vec3::ZERO;
        loco.motion.acceleration = Vec3::ZERO;
        loco.slide = Vec3::ZERO;
        loco.height_offset = 0.0;
        loco.height_offset_velocity = 0.0;
        loco.grounded = None;
        let (translation, rotation) = m.root_transform();
        t.translation = translation;
        t.rotation = rotation;
        stats.fell_out += 1;
        out.write(CharacterFellOut { entity, from: pos, to: last });
    }
}

/// Inserts and removes capsules, records where each mover's feet are and how
/// fast they move (for [`target_capsules`]); pushes what the capsules touch.
#[allow(clippy::needless_pass_by_value)]
fn capsules(
    time: Res<Time>,
    settings: Res<CharacterPhysics>,
    controller: Option<Res<ControllerSettings>>,
    mut physics: ResMut<Physics>,
    mut map: ResMut<Capsules>,
    mut stats: ResMut<CharacterStats>,
    movers: Query<(Entity, &Mover), Without<Dormant>>,
) {
    let dt = time.delta_secs();
    // The mover step's time (`sf_controller_bevy` clamps a long frame).
    let step_dt = controller.map_or(dt, |c| dt.min(c.max_dt));
    let now = time.elapsed_secs_f64();
    let active = physics.worlds.active().cloned();
    // Gone, dormant, or in another worldspace: out.
    map.0.retain(|e, c| {
        let keep = movers.contains(*e) && Some(&c.worldspace) == active.as_ref();
        if !keep && let Some(w) = physics.worlds.get_mut(&c.worldspace) {
            w.remove(c.id);
        }
        keep
    });
    let (mut pushed, mut max_push) = (0, 0.0_f32);
    if let Some(ws) = active
        && let Some(w) = physics.worlds.get_mut(&ws)
    {
        for (e, m) in &movers {
            let shape = CharacterShape { radius: m.params.radius, height: m.params.height };
            let feet = m.loco.position;
            // The last step's displacement (in the frame before it) in world space;
            // teleports (fell out, got up) set the position and aren't in it.
            let moved = m.loco.moved;
            let world = Quat::from_rotation_y(m.loco.motion.yaw - moved.yaw) * moved.translation;
            let velocity = if step_dt > 0.0 && world.is_finite() { world / step_dt } else { Vec3::ZERO };
            match map.0.get_mut(&e) {
                Some(c) if c.shape == shape => {
                    c.feet = feet;
                    c.velocity = velocity;
                    c.time = now;
                }
                other => {
                    if let Some(c) = other {
                        w.remove(c.id);
                    }
                    let id = w.insert(&BodyDesc::character(feet, shape, &settings.push));
                    map.0.insert(e, Capsule { worldspace: ws.clone(), id, shape, feet, velocity, time: now });
                }
            }
            let p = w.push(feet, shape, m.applied.desired_velocity, &settings.push, dt);
            pushed += p.bodies;
            max_push = max_push.max(p.max_impulse);
        }
    }
    *stats = CharacterStats { capsules: map.0.len(), pushed, max_push, impacts: 0, fell_out: stats.fell_out };
}

/// `FixedUpdate`, before each step: every capsule targets its mover's feet *at
/// the step's time*, extrapolated from the last mover step (the mover runs at
/// frame rate, the step at 60 Hz). Targeting the last frame's feet instead
/// moved the capsule by 0, 1 or 2 frames of walking per step as the two
/// clocks beat: the capsule vibrated against the body (AN-7).
#[allow(clippy::needless_pass_by_value)]
fn target_capsules(time: Res<Time>, mut physics: ResMut<Physics>, map: Res<Capsules>) {
    // In `FixedUpdate` the generic clock is the fixed one: this step's end.
    let now = time.elapsed_secs_f64();
    let Some(w) = physics.worlds.active_world_mut() else { return };
    for c in map.0.values() {
        #[allow(clippy::cast_possible_truncation)]
        let lead = (now - c.time).max(0.0) as f32;
        w.move_character(c.id, c.feet.offset(c.velocity * lead), c.shape);
    }
}

/// Hits on capsules ([`PhysicsEvent::Hit`]: impact momentum at first
/// contact, [`sf_physics::impact_impulse`]) → [`ImpactOnCharacter`]. Not the
/// solver's contact force: that one is spread over steps for soft hits and
/// unbounded for bodies a capsule walks into or pins (PH-10).
#[allow(clippy::needless_pass_by_value)]
fn impacts(
    events: Res<PhysicsEvents>,
    settings: Res<CharacterPhysics>,
    map: Res<Capsules>,
    mut stats: ResMut<CharacterStats>,
    mut out: MessageWriter<ImpactOnCharacter>,
) {
    if events.0.is_empty() || map.0.is_empty() {
        return;
    }
    let by_body: HashMap<BodyId, Entity> = map.0.iter().map(|(e, c)| (c.id, *e)).collect();
    for e in &events.0 {
        let PhysicsEvent::Hit { target, impulse, point, direction, .. } = *e else { continue };
        if impulse < settings.impact_threshold {
            continue;
        }
        if let Some(&entity) = by_body.get(&target) {
            out.write(ImpactOnCharacter { entity, impulse, point, direction });
            stats.impacts += 1;
        }
    }
}
