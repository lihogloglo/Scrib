//! Characters on physics (physics.md "Controller", P5; feature `controller`).
//!
//! Every [`Mover`] that isn't [`Dormant`] gets a kinematic capsule in the
//! active worldspace's physics world ([`sf_physics::BodyDesc::character`]),
//! targeted at the mover's feet after each mover step, so falling and thrown
//! bodies hit characters. After the step, the bodies the capsule touches get
//! capped push impulses ([`sf_physics::PhysicsWorld::push`]) from the intent
//! the mover ran with. Contact forces on a capsule over
//! [`CharacterPhysics::impact_threshold`] become [`ImpactOnCharacter`]
//! messages for gameplay (hit reactions, knockdowns). The mover itself keeps
//! running in `Update` against the physics world's queries: the host makes
//! [`crate::Physics`] its `SfControllerPlugin` ground.

use std::collections::HashMap;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_time::Time;
use sf_controller_bevy::{ControllerSystems, Dormant, Mover};
use sf_coords::WorldPos;
use sf_physics::{BodyDesc, BodyId, CharacterShape, PhysicsEvent, PushParams};
use sf_world::WorldspaceId;

use crate::{Physics, PhysicsEvents, PhysicsSystems};

/// Push and impact tuning (physics.md defaults).
#[derive(Resource, Clone, Debug)]
pub struct CharacterPhysics {
    pub push: PushParams,
    /// Contact impulse on a capsule (N·s) from which it counts as an impact.
    pub impact_threshold: f32,
}

impl Default for CharacterPhysics {
    fn default() -> Self {
        Self { push: PushParams::default(), impact_threshold: 5.0 }
    }
}

/// Something hit a character hard enough to react (physics.md "Impacts").
#[derive(Message, Clone, Debug)]
pub struct ImpactOnCharacter {
    pub entity: Entity,
    /// Contact impulse, N·s.
    pub impulse: f32,
    pub point: WorldPos,
}

/// Counters for the log and benches.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CharacterStats {
    pub capsules: usize,
    /// Bodies pushed this frame and the largest push, N·s.
    pub pushed: u32,
    pub max_push: f32,
    pub impacts: u32,
}

/// Each character's capsule: its world, body and shape.
#[derive(Resource, Default)]
struct Capsules(HashMap<Entity, (WorldspaceId, BodyId, CharacterShape)>);

pub(crate) fn add(app: &mut App) {
    app.init_resource::<CharacterPhysics>()
        .init_resource::<CharacterStats>()
        .init_resource::<Capsules>()
        .add_message::<ImpactOnCharacter>()
        .add_systems(Update, (capsules, impacts).chain().after(ControllerSystems::Step).before(PhysicsSystems::WriteBack));
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
    *stats = CharacterStats { capsules: map.0.len(), pushed, max_push, impacts: 0 };
}

/// Contact forces on capsules → [`ImpactOnCharacter`].
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
        let PhysicsEvent::ContactForce { a, b, impulse, point } = *e else { continue };
        if impulse < settings.impact_threshold {
            continue;
        }
        if let Some(&entity) = by_body.get(&a).or_else(|| by_body.get(&b)) {
            out.write(ImpactOnCharacter { entity, impulse, point });
            stats.impacts += 1;
        }
    }
}
