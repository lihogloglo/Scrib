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
//! messages for gameplay (hit reactions, knockdowns). The host makes [`crate::Physics`] its
//! `SfControllerPlugin` ground.
//!
//! Tick order (`IN11b`): **the physics step first, then the movers**, so a character reads the
//! world its base has just moved to. [`PhysicsSystems::Step`] runs before
//! [`ControllerSystems::Drive`] here; a host with action-driven bodies (`sf_action_bevy`) orders
//! it before `ActionSystems::Sample` too. This is the industry's based movement: UE's
//! `CharacterMovementComponent` (and Mover 2.0) tick after their movement base (a tick
//! prerequisite), Jolt's `CharacterVirtual` updates after the physics step. The mover rides the
//! base with the velocity its last ground cast saw (`Locomotion::ride_base`, `sf_controller`);
//! run before the step, it casts against the base's pose of the tick before, so a lift would
//! sink into the character by one tick of travel and every start or stop of a base would shift
//! the character on it. Between the two, [`refresh_base_velocity`] re-reads each based mover's
//! ground velocity from its body as this step left it (Jolt's
//! `CharacterVirtual::UpdateGroundVelocity`), so the ride is this step's base motion, not the
//! last cast's (one tick late when a base starts, stops or turns). A host with action-driven
//! bodies calls it for them in the same place. The capsules (below) and the carry hands
//! ([`crate::carry`]) are targeted after the movers, for the next tick's step.

use std::collections::HashMap;

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::Time;
use sf_controller_bevy::{ControllerSystems, Dormant, Locomotion, Mover, MoverMode};
use sf_coords::WorldPos;
use sf_physics::{BodyDesc, BodyId, CharacterShape, PhysicsEvent, PhysicsWorld, PushParams};
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
    /// The struck shape of the character's [`crate::BodyProxies`] (character-shapes.md "Impacts"); `None`: its
    /// mover capsule.
    pub shape: Option<u16>,
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

/// Each character's capsule: its world, body and shape. Targeted at the
/// mover's feet on every tick (the mover and the physics step share it).
#[derive(Resource, Default)]
pub(crate) struct Capsules(pub(crate) HashMap<Entity, Capsule>);

pub(crate) struct Capsule {
    pub(crate) worldspace: WorldspaceId,
    pub(crate) id: BodyId,
    shape: CharacterShape,
}

pub(crate) fn add(app: &mut App) {
    app.init_resource::<CharacterPhysics>()
        .init_resource::<CharacterStats>()
        .init_resource::<Capsules>()
        .init_resource::<SafeGround>()
        .add_message::<ImpactOnCharacter>()
        .add_message::<CharacterFellOut>()
        // The physics step, then the movers (the module docs: based movement). The fell-out net
        // and the capsule targets follow each tick's mover state, for the next tick's step.
        .configure_sets(FixedUpdate, PhysicsSystems::Step.before(ControllerSystems::Drive))
        .add_systems(FixedUpdate, (fell_out, capsules).chain().after(ControllerSystems::Step))
        .add_systems(FixedUpdate, base_velocities.after(PhysicsSystems::Step).before(ControllerSystems::Step))
        .add_systems(Update, impacts.before(PhysicsSystems::WriteBack));
}

/// Re-reads the ground velocity of a character standing on a body (`grounded.body`) from the
/// body as the last physics step left it: its point velocity at the ground point and its
/// angular velocity, the motion `Locomotion::ride_base` applies in the mover step that follows
/// (Jolt `CharacterVirtual::UpdateGroundVelocity`). Call after the physics step, before the
/// mover. The ground point was cast before this step, on the body's pose at the step's start,
/// so the velocity field is taken about the centre of mass there. Fixed ground, or a body gone
/// from `world`: left as it is.
pub fn refresh_base_velocity(world: &PhysicsWorld, loco: &mut Locomotion) {
    let Some(g) = loco.grounded.as_mut() else { return };
    let Some(id) = g.body.map(BodyId::from) else { return };
    let (Some((v, w)), Some(pose), Some((com, _))) = (world.velocity(id), world.pose(id), world.mass_center(id)) else { return };
    let start = pose.pos.offset(pose.rotation * com - v * world.params().dt);
    g.velocity = v + w.cross(start.delta_f32(g.point));
    g.angular_velocity = w;
}

/// [`refresh_base_velocity`] for the movers on a body, after the physics step.
#[allow(clippy::needless_pass_by_value)]
fn base_velocities(physics: Res<Physics>, mut movers: Query<&mut Mover, Without<Dormant>>) {
    let Some(w) = physics.worlds.active_world() else { return };
    for mut m in &mut movers {
        if m.loco.grounded.is_some_and(|g| g.body.is_some()) {
            refresh_base_velocity(w, &mut m.loco);
        }
    }
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
    mut movers: Query<(Entity, &mut Mover), Without<Dormant>>,
) {
    safe.0.retain(|e, _| movers.contains(*e));
    let kill = physics.worlds.active_world().and_then(PhysicsWorld::kill_height);
    for (entity, mut m) in &mut movers {
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
        // Drawn there at once, not slid to over a tick.
        m.snap();
        stats.fell_out += 1;
        out.write(CharacterFellOut { entity, from: pos, to: last });
    }
}

/// `FixedUpdate`, after the mover's tick (the next tick's physics step moves
/// the capsules): inserts and removes capsules, targets each at its mover's
/// feet led by the mover's motion of this tick, and pushes what the capsules
/// touch over the tick. A kinematic body reaches its target at the end of the
/// next step, which simulates the tick in which the mover moves on from these
/// feet: led by one tick of the same motion (constant velocity), the capsule
/// moves with the mover, not a tick behind it (Jolt's `CharacterVirtual`
/// inner body: the character's pose and velocity after its update). A mover
/// snapped since its step (a fell-out return) has no motion to lead by.
#[allow(clippy::needless_pass_by_value)]
fn capsules(
    time: Res<Time>,
    settings: Res<CharacterPhysics>,
    mut physics: ResMut<Physics>,
    mut map: ResMut<Capsules>,
    mut stats: ResMut<CharacterStats>,
    movers: Query<(Entity, &Mover), Without<Dormant>>,
) {
    // In `FixedUpdate` the generic clock is the fixed one: the tick.
    let dt = time.delta_secs();
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
            let (prev, curr) = m.poses();
            let lead = prev.position.delta_f32(curr.position);
            match map.0.get(&e) {
                Some(c) if c.shape == shape => {
                    w.move_character(c.id, feet.offset(lead), shape);
                }
                other => {
                    if let Some(c) = other {
                        w.remove(c.id);
                    }
                    let id = w.insert(&BodyDesc::character(feet, shape, &settings.push));
                    map.0.insert(e, Capsule { worldspace: ws.clone(), id, shape });
                }
            }
            let p = w.push(feet, shape, m.applied.desired_velocity, &settings.push, dt);
            pushed += p.bodies;
            max_push = max_push.max(p.max_impulse);
        }
    }
    *stats = CharacterStats { capsules: map.0.len(), pushed, max_push, impacts: 0, fell_out: stats.fell_out };
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
            out.write(ImpactOnCharacter { entity, impulse, point, direction, shape: None });
            stats.impacts += 1;
        }
    }
}
