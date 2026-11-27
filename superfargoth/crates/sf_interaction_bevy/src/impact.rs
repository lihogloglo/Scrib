//! Impact damage shell (`docs/specs/interaction-destruction.md` "Impacts (IN4)", slice `IN4b`;
//! `physics` feature). Gameplay (clients get `Destroyed`): a hard physics contact damages the
//! registered placements it touches, through the same [`PropDamage`] path as a blow. The maths is
//! the core's ([`impact_damage`]); damage only, no noise (IN18 reads the same contacts itself).
//!
//! - **This step's events only**: [`PhysicsEvents`] holds the frame's steps (cleared in `First`);
//!   [`ImpactCursor`] (reset in `First`) marks how far this frame's steps were read, so two steps
//!   in one frame never read the same event.
//! - **Trigger**: each `PhysicsEvent::Contact` (solver normal impulse + the speed change it caused,
//!   each pair once) with a nonzero [`impact_damage`]. Only on a step with one does the shell map
//!   bodies to placements: [`Destructibles`] (not `gone`) through `PhysicsStreaming::body` in the
//!   active world.
//! - **Filters**: a pair with a debris-shell piece ([`DebrisPiece`], cosmetic), a `PROJECTILE` body
//!   (combat's hit path) or a `HELD` one (the carry spring pushing) gives nothing. Not the `DEBRIS`
//!   layer: light loose items stream in on it, and they are what this is for.
//! - **Out**: per registered side, `PropDamage { amount: impact_damage(impulse, speed), kind:
//!   DAMAGE_IMPACT, point, dir, culprit }`, `dir` the normal into that side (`normal` for `b`,
//!   `-normal` for `a`). Both sides take the same raw amount (one impulse); [`DestructionRules`]'
//!   `(impact, material)` factor and each def's health split it. The damage step's sort makes the
//!   result independent of event order.
//! - **Culprit** (`IN4c`): [`Thrown`] maps a body thrown by a holder with an [`InteractionUser`]
//!   (`CarryFeedback { outcome: Thrown { body, .. } }`) to that user and the throw's tick, until the
//!   body sleeps, is held again (`HELD`), leaves its world or the active world changes. A side's
//!   culprit is the other body's thrower, else its own (a thrown jar breaking on a wall); a body
//!   dropped, knocked or never thrown has none.
//!
//! Order: `FixedUpdate`, [`ImpactSystems`] after `PhysicsSystems::Step`, before
//! `DestroySystems::Damage`; only with `Physics`, `PhysicsEvents` and `PhysicsStreaming`. Without
//! `PhysicsParams::contact_report_layers` there are no contacts and nothing to do.
//!
//! [`DestructionRules`]: crate::DestructionRules

use std::collections::{BTreeMap, BTreeSet};

use bevy_ecs::prelude::*;
use sf_interaction::{DAMAGE_IMPACT, impact_damage};
use sf_physics_bevy::sf_physics::{BodyId, Layer, PhysicsEvent};
use sf_interaction::{Tick, UserKey};
use sf_physics_bevy::{CarryFeedback, CarryOutcome, Physics, PhysicsBody, PhysicsEvents, PhysicsStreaming, PhysicsSystems};
use sf_world::{PlacementId, WorldspaceId};

use crate::{
    DebrisPiece, DestroySystems, Destructibles, InteractionSettings, InteractionTick, InteractionUser, PropDamage,
};

/// How many of this frame's [`PhysicsEvents`] the impact step has read (reset in `First`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImpactCursor(pub usize);

/// Who threw a body still in flight (module docs, "Culprit").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThrownBy {
    pub user: UserKey,
    /// The interaction tick the shell saw the throw on.
    pub tick: Tick,
}

/// Bodies thrown by a user, in the world they were thrown in, until they rest (module docs).
#[derive(Resource, Clone, Debug, Default)]
pub struct Thrown {
    world: Option<WorldspaceId>,
    bodies: BTreeMap<BodyId, ThrownBy>,
}

impl Thrown {
    /// The thrower of `body` in `ws`, while it flies.
    pub fn get(&self, ws: &WorldspaceId, body: BodyId) -> Option<ThrownBy> {
        if self.world.as_ref() != Some(ws) {
            return None;
        }
        self.bodies.get(&body).copied()
    }

    pub fn len(&self) -> usize {
        self.bodies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bodies.is_empty()
    }
}

/// The impact step, `FixedUpdate` (module docs).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImpactSystems;

/// Layers whose contacts are never impacts (module docs).
const NOT_IMPACTS: Layer = Layer::PROJECTILE.union(Layer::HELD);

pub(crate) fn build(app: &mut bevy_app::App) {
    use bevy_app::{First, FixedUpdate};
    // Idempotent: the carry plugin registers it too (an app without carry reads none).
    app.add_message::<CarryFeedback>()
        .init_resource::<ImpactCursor>()
        .init_resource::<Thrown>()
        .configure_sets(
            FixedUpdate,
            ImpactSystems
                .after(PhysicsSystems::Step)
                .before(DestroySystems::Damage)
                // An app without physics (tests, tools) runs the shell without impacts.
                .run_if(resource_exists::<Physics>)
                .run_if(resource_exists::<PhysicsEvents>)
                .run_if(resource_exists::<PhysicsStreaming>),
        )
        .add_systems(First, reset_cursor)
        .add_systems(FixedUpdate, (record_throws, impacts, forget_landed).chain().in_set(ImpactSystems));
}

fn reset_cursor(mut cursor: ResMut<ImpactCursor>) {
    cursor.0 = 0;
}

/// This step's throws by a user into [`Thrown`] (a body thrown again takes the new thrower).
#[allow(clippy::needless_pass_by_value)]
fn record_throws(
    mut feedback: MessageReader<CarryFeedback>,
    users: Query<&InteractionUser>,
    physics: Res<Physics>,
    tick: Res<InteractionTick>,
    mut thrown: ResMut<Thrown>,
) {
    for f in feedback.read() {
        let CarryOutcome::Thrown { body, .. } = f.outcome else { continue };
        let Some(ws) = physics.worlds.active() else { continue };
        if thrown.world.as_ref() != Some(ws) {
            thrown.world = Some(ws.clone());
            thrown.bodies.clear();
        }
        if let Ok(u) = users.get(f.holder) {
            thrown.bodies.insert(body, ThrownBy { user: u.user, tick: tick.0 });
        } else {
            // A holder that is no user (a dev camera): the body flies without a culprit.
            thrown.bodies.remove(&body);
        }
    }
}

/// Bodies at rest, held again or gone leave [`Thrown`]; a changed active world clears it.
#[allow(clippy::needless_pass_by_value)]
fn forget_landed(physics: Res<Physics>, mut thrown: ResMut<Thrown>) {
    if thrown.bodies.is_empty() {
        return;
    }
    let ws = physics.worlds.active().filter(|ws| thrown.world.as_ref() == Some(*ws));
    let Some(world) = ws.and_then(|ws| physics.world(ws)) else {
        thrown.bodies.clear();
        return;
    };
    thrown
        .bodies
        .retain(|&id, _| world.is_sleeping(id) == Some(false) && world.layer(id).is_some_and(|l| !l.contains(Layer::HELD)));
}

/// One gated contact: both bodies, the raw damage, where and the normal from `a` to `b`.
struct Hit {
    a: BodyId,
    b: BodyId,
    amount: f32,
    point: glam::Vec3,
    normal: glam::Vec3,
}

/// This step's contacts into [`PropDamage`] (module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn impacts(
    settings: Res<InteractionSettings>,
    physics: Res<Physics>,
    events: Res<PhysicsEvents>,
    streaming: Res<PhysicsStreaming>,
    registry: Res<Destructibles>,
    thrown: Res<Thrown>,
    pieces: Query<&PhysicsBody, With<DebrisPiece>>,
    mut cursor: ResMut<ImpactCursor>,
    mut out: MessageWriter<PropDamage>,
) {
    let from = cursor.0.min(events.0.len());
    cursor.0 = events.0.len();
    let p = settings.0;
    let hits: Vec<Hit> = events.0[from..]
        .iter()
        .filter_map(|e| match *e {
            PhysicsEvent::Contact { a, b, impulse, speed, point, normal, .. } => {
                let amount = impact_damage(impulse, speed, &p);
                // `as_vec3`: interaction frames are f32 world metres (the inverse of debris' `world_pos`).
                (amount > 0.0).then(|| Hit { a, b, amount, point: point.to_f64().as_vec3(), normal })
            }
            _ => None,
        })
        .collect();
    if hits.is_empty() {
        return;
    }
    let Some(ws) = physics.worlds.active() else { return };
    let Some(world) = physics.world(ws) else { return };
    let placements: BTreeMap<BodyId, PlacementId> = registry
        .iter()
        .filter(|(_, d)| !d.gone)
        .filter_map(|(placement, _)| Some((streaming.body(ws, placement)?, placement)))
        .collect();
    let debris: BTreeSet<BodyId> = pieces.iter().filter(|b| &b.worldspace == ws).map(|b| b.id).collect();
    let skip = |id: BodyId| debris.contains(&id) || world.layer(id).is_some_and(|l| l.intersects(NOT_IMPACTS));
    for h in hits {
        if skip(h.a) || skip(h.b) {
            continue;
        }
        let by = |id: BodyId| thrown.get(ws, id).map(|t| t.user);
        for (body, other, dir) in [(h.a, h.b, -h.normal), (h.b, h.a, h.normal)] {
            let Some(&placement) = placements.get(&body) else { continue };
            let culprit = by(other).or_else(|| by(body));
            out.write(PropDamage { placement, amount: h.amount, kind: DAMAGE_IMPACT, point: h.point, dir, culprit });
        }
    }
}
