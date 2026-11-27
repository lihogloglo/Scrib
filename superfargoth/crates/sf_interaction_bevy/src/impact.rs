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
//!   DAMAGE_IMPACT, point, dir, culprit: None }`, `dir` the normal into that side (`normal` for `b`,
//!   `-normal` for `a`). Both sides take the same raw amount (one impulse); [`DestructionRules`]'
//!   `(impact, material)` factor and each def's health split it. The damage step's sort makes the
//!   result independent of event order.
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
use sf_physics_bevy::{Physics, PhysicsBody, PhysicsEvents, PhysicsStreaming, PhysicsSystems};
use sf_world::PlacementId;

use crate::{DebrisPiece, DestroySystems, Destructibles, InteractionSettings, PropDamage};

/// How many of this frame's [`PhysicsEvents`] the impact step has read (reset in `First`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImpactCursor(pub usize);

/// The impact step, `FixedUpdate` (module docs).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImpactSystems;

/// Layers whose contacts are never impacts (module docs).
const NOT_IMPACTS: Layer = Layer::PROJECTILE.union(Layer::HELD);

pub(crate) fn build(app: &mut bevy_app::App) {
    use bevy_app::{First, FixedUpdate};
    app.init_resource::<ImpactCursor>()
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
        .add_systems(FixedUpdate, impacts.in_set(ImpactSystems));
}

fn reset_cursor(mut cursor: ResMut<ImpactCursor>) {
    cursor.0 = 0;
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
        for (body, dir) in [(h.a, -h.normal), (h.b, h.normal)] {
            let Some(&placement) = placements.get(&body) else { continue };
            out.write(PropDamage { placement, amount: h.amount, kind: DAMAGE_IMPACT, point: h.point, dir, culprit: None });
        }
    }
}
