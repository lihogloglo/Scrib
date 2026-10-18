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

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_time::Time;
use bevy_transform::components::Transform;
use sf_controller_bevy::{ControllerSystems, Dormant, Mover, MoverMode};
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
    pub direction: bevy_math::Vec3,
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

/// Each character's capsule: its world, body and shape.
#[derive(Resource, Default)]
struct Capsules(HashMap<Entity, (WorldspaceId, BodyId, CharacterShape)>);

pub(crate) fn add(app: &mut App) {
    app.init_resource::<CharacterPhysics>()
        .init_resource::<CharacterStats>()
        .init_resource::<Capsules>()
        .init_resource::<SafeGround>()
        .add_message::<ImpactOnCharacter>()
        .add_message::<CharacterFellOut>()
        .add_systems(Update, (fell_out, capsules, impacts).chain().after(ControllerSystems::Step).before(PhysicsSystems::WriteBack));
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
        loco.motion.velocity = bevy_math::Vec3::ZERO;
        loco.motion.acceleration = bevy_math::Vec3::ZERO;
        loco.slide = bevy_math::Vec3::ZERO;
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

/// Inserts, moves and removes capsules; pushes what they touch.
#[allow(clippy::needless_pass_by_value)]
fn capsules(
    time: Res<Time>,
    settings: Res<CharacterPhysics>,
    mut physics: ResMut<Physics>,
    mut map: ResMut<Capsules>,
    mut stats: ResMut<CharacterStats>,
    movers: Query<(Entity, &Mover), Without<Dormant>>,
) {
    let dt = time.delta_secs();
    let active = physics.worlds.active().cloned();
    // Gone, dormant, or in another worldspace: out.
    map.0.retain(|e, (ws, id, _)| {
        let keep = movers.contains(*e) && Some(&*ws) == active.as_ref();
        if !keep && let Some(w) = physics.worlds.get_mut(ws) {
            w.remove(*id);
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
            match map.0.get(&e) {
                Some((_, id, s)) if *s == shape => {
                    w.move_character(*id, feet, shape);
                }
                other => {
                    if let Some((_, id, _)) = other {
                        w.remove(*id);
                    }
                    let id = w.insert(&BodyDesc::character(feet, shape, &settings.push));
                    map.0.insert(e, (ws.clone(), id, shape));
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
    let by_body: HashMap<BodyId, Entity> = map.0.iter().map(|(e, (_, id, _))| (*id, *e)).collect();
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
