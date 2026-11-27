//! Interaction noise shell (`docs/specs/interaction-world-rules.md` "Noise", slice `IN18b`). Host
//! only: the tick's door toggles, breaks and (with the `physics` feature) physics contacts become
//! core [`Noise`]s, thinned by the core's [`NoiseMerge`], written as [`InteractionNoise`] on
//! `FixedUpdate` in [`NoiseSystems::Emit`]. The host maps them to its perception's vocabulary
//! (`sf_ai_bevy::NoiseEvent`, `IN18c`); this crate never depends on the AI.
//!
//! - **Materials**: [`InteractionMaterials`] (the host fills it from `sf.interact.material`) gives
//!   a placement's def material its `impact_reach_m`. The door, bash and lock pick reaches are set
//!   for a reference material (`interact.noise.material_ref_m`, wood): a material's ratio is its
//!   reach over that one; no material, or one the table lacks, is the reference (ratio 1).
//! - **Door** ([`DoorChanged`] of a registered door): at the doorway (the door slot's centre in the
//!   world), `interact.noise.door_reach_m` x the door's material ratio, kind [`NOISE_DOOR`], the
//!   toggling user as maker, revealed (a close-behind has no maker).
//! - **Break** ([`Destroyed`] of a registered destructible): at the object, the material's reach x
//!   `interact.noise.speed_mul_max` (a break is as loud as the hardest impact), kind
//!   [`NOISE_BREAK`], the culprit as maker, not revealed. Not yet: a melee break reveals its maker
//!   (`Destroyed` does not carry the hit's damage kind).
//! - **Impact** (`physics` feature): each `PhysicsEvent::Contact` of this step (its own
//!   [`NoiseCursor`], reset in `First`) between bodies of indexed placements with a known material,
//!   at the contact point, [`sf_interaction::loudness_m`]`(impact_reach_m, speed)` of the louder side (one contact,
//!   one noise), kind = that material's `noise`. Skipped as in the impact shell: debris-shell
//!   pieces, `PROJECTILE` and `HELD` bodies. A body in flight from a user's throw
//!   ([`crate::Thrown`]) makes a [`NoiseCase::ThrownLanding`] naming the thrower, not revealed (a
//!   distraction, not blame); others are [`NoiseCase::Impact`] with no maker. Not yet: the `Drop`
//!   case (a released prop revealing a dropper within `interact.noise.reveal_m`).
//! - **Thinning**: [`NoiseInbox`] gathers the tick's noises (a later slice's bash or lock pick
//!   pushes its own), [`NoiseMerge::step`] keeps one per object a tick and window and the
//!   `interact.noise.per_tick_max` loudest. A changed active physics world clears the windows.
//! - **Out**: `InteractionNoise { tick, at, space: None, .. }`, loudest first; `space` `None` is the
//!   active worldspace (every noise the shell makes today is in it).
//!
//! Order: `FixedUpdate`, [`NoiseSystems::Collect`] after `DoorSystems::Commit`,
//! `DestroySystems::Release` (and `ImpactSystems`), then [`NoiseSystems::Emit`]. Hosts read
//! [`InteractionNoise`] `.after(NoiseSystems::Emit)`, before their perception step.

use std::collections::BTreeMap;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::Vec3;
use sf_interaction::{
    InteractParams, MaterialDef, Noise, NoiseCase, NoiseMerge, TagId, Tick, UserKey, splitmix64,
};
use sf_world::{PlacementId, WorldspaceId};

use crate::{
    DestroySystems, Destroyed, Destructibles, DoorChanged, DoorSystems, Doors, InteractionSettings, InteractionTick,
    effective_params,
};

/// The kind of a door opening or closing.
pub const NOISE_DOOR: TagId = TagId::of("noise.door");
/// The kind of an object breaking.
pub const NOISE_BREAK: TagId = TagId::of("noise.break");

/// The material records by id (`sf.interact.material`), filled by the host.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionMaterials(BTreeMap<TagId, MaterialDef>);

impl InteractionMaterials {
    /// Adds or replaces `def` (by its id).
    pub fn insert(&mut self, def: MaterialDef) {
        self.0.insert(def.id, def);
    }

    #[must_use]
    pub fn get(&self, id: TagId) -> Option<&MaterialDef> {
        self.0.get(&id)
    }

    /// `material`'s impact reach, m; the reference material's for none or an unknown one.
    #[must_use]
    pub fn reach_m(&self, material: Option<TagId>, params: &InteractParams) -> f32 {
        material.and_then(|m| self.get(m)).map_or(params.noise_material_ref_m, |m| m.impact_reach_m)
    }

    /// `material`'s reach over the reference material's (1 for none or an unknown one; a
    /// non-finite ratio is 1, a negative one 0).
    #[must_use]
    pub fn ratio(&self, material: Option<TagId>, params: &InteractParams) -> f32 {
        let r = self.reach_m(material, params) / params.noise_material_ref_m;
        if r.is_finite() { r.max(0.0) } else { 1.0 }
    }
}

/// One heard interaction noise (spec "The message"); hosts map it to their perception.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct InteractionNoise {
    pub tick: Tick,
    /// Where it is heard from, world metres (interaction frame).
    pub at: Vec3,
    /// The space it was made in; `None`: the active worldspace.
    pub space: Option<WorldspaceId>,
    pub kind: TagId,
    /// How far it is heard in open air, m.
    pub loudness_m: f32,
    /// Who made it (an actor never hears its own noise).
    pub maker: Option<UserKey>,
    /// Listeners learn who made it, not only where.
    pub reveals_maker: bool,
    pub case: NoiseCase,
}

/// The tick's noises before thinning, and the merge windows (module docs).
#[derive(Resource, Clone, Debug, Default)]
pub struct NoiseInbox {
    pending: Vec<Noise>,
    merge: NoiseMerge,
    /// The active physics world the windows belong to.
    #[cfg(feature = "physics")]
    world: Option<WorldspaceId>,
}

impl NoiseInbox {
    /// Queues `noise` for this tick's [`NoiseSystems::Emit`].
    pub fn push(&mut self, noise: Noise) {
        self.pending.push(noise);
    }

    /// Objects with an open merge window.
    #[must_use]
    pub fn open_windows(&self) -> usize {
        self.merge.open_windows()
    }
}

/// How many of this frame's `PhysicsEvents` the noise step has read (reset in `First`).
#[cfg(feature = "physics")]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoiseCursor(pub usize);

/// Where noises are gathered and written, in `FixedUpdate`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoiseSystems {
    /// The tick's doors, breaks and contacts into [`NoiseInbox`].
    Collect,
    /// [`NoiseInbox`] thinned into [`InteractionNoise`].
    Emit,
}

/// The merge key of a placement (the core keys objects by `u64`).
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn source_key(placement: PlacementId) -> u64 {
    splitmix64((placement.0 as u64) ^ splitmix64((placement.0 >> 64) as u64))
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<InteractionMaterials>()
        .init_resource::<NoiseInbox>()
        .add_message::<InteractionNoise>()
        .configure_sets(
            FixedUpdate,
            (NoiseSystems::Collect, NoiseSystems::Emit)
                .chain()
                .after(DoorSystems::Commit)
                .after(DestroySystems::Release),
        )
        .add_systems(FixedUpdate, (doors, breaks).chain().in_set(NoiseSystems::Collect))
        .add_systems(FixedUpdate, emit.in_set(NoiseSystems::Emit));
    #[cfg(feature = "physics")]
    {
        use sf_physics_bevy::{Physics, PhysicsEvents, PhysicsStreaming};
        app.init_resource::<NoiseCursor>()
            .configure_sets(FixedUpdate, NoiseSystems::Collect.after(crate::ImpactSystems))
            .add_systems(bevy_app::First, |mut c: ResMut<NoiseCursor>| c.0 = 0)
            .add_systems(
                FixedUpdate,
                impacts
                    .after(breaks)
                    .in_set(NoiseSystems::Collect)
                    .run_if(resource_exists::<Physics>)
                    .run_if(resource_exists::<PhysicsEvents>)
                    .run_if(resource_exists::<PhysicsStreaming>),
            );
    }
}

/// The tick's door toggles of registered doors (module docs, "Door").
#[allow(clippy::needless_pass_by_value)]
fn doors(
    settings: Res<InteractionSettings>,
    doors: Res<Doors>,
    materials: Res<InteractionMaterials>,
    mut changed: MessageReader<DoorChanged>,
    mut inbox: ResMut<NoiseInbox>,
) {
    let p = settings.0;
    for c in changed.read() {
        let Some(door) = doors.get(c.0.placement) else { continue };
        let at = door.object.translation + door.object.rotation * door.doorway;
        inbox.push(Noise {
            source: source_key(c.0.placement),
            at,
            kind: NOISE_DOOR,
            loudness_m: p.noise_door_reach_m * materials.ratio(door.def.material, &p),
            maker: c.0.user,
            reveals_maker: true,
            case: NoiseCase::Door,
        });
    }
}

/// The tick's released breaks of registered destructibles (module docs, "Break").
#[allow(clippy::needless_pass_by_value)]
fn breaks(
    settings: Res<InteractionSettings>,
    registry: Res<Destructibles>,
    materials: Res<InteractionMaterials>,
    mut destroyed: MessageReader<Destroyed>,
    mut inbox: ResMut<NoiseInbox>,
) {
    let p = settings.0;
    for d in destroyed.read() {
        let Some(object) = registry.get(d.0.placement) else { continue };
        inbox.push(Noise {
            source: source_key(d.0.placement),
            at: object.object.translation,
            kind: NOISE_BREAK,
            loudness_m: materials.reach_m(object.def.material, &p) * p.noise_speed_mul_max,
            maker: d.0.culprit,
            reveals_maker: false,
            case: NoiseCase::Break,
        });
    }
}

/// This step's physics contacts (module docs, "Impact").
#[cfg(feature = "physics")]
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn impacts(
    settings: Res<InteractionSettings>,
    physics: Res<sf_physics_bevy::Physics>,
    events: Res<sf_physics_bevy::PhysicsEvents>,
    streaming: Res<sf_physics_bevy::PhysicsStreaming>,
    defs: Res<crate::InteractionDefs>,
    materials: Res<InteractionMaterials>,
    thrown: Res<crate::Thrown>,
    pieces: Query<&sf_physics_bevy::PhysicsBody, With<crate::DebrisPiece>>,
    mut cursor: ResMut<NoiseCursor>,
    mut inbox: ResMut<NoiseInbox>,
) {
    use std::collections::BTreeSet;

    use sf_interaction::loudness_m;
    use sf_physics_bevy::sf_physics::{BodyId, Layer, PhysicsEvent};

    let active = physics.worlds.active();
    if inbox.world.as_ref() != active {
        inbox.world = active.cloned();
        inbox.merge.clear();
    }
    let from = cursor.0.min(events.0.len());
    cursor.0 = events.0.len();
    let p = settings.0;
    let contacts: Vec<(BodyId, BodyId, f32, Vec3)> = events.0[from..]
        .iter()
        .filter_map(|e| match *e {
            PhysicsEvent::Contact { a, b, speed, point, .. } if speed >= p.noise_min_speed_mps => {
                // `as_vec3`: interaction frames are f32 world metres (as the impact shell).
                Some((a, b, speed, point.to_f64().as_vec3()))
            }
            _ => None,
        })
        .collect();
    if contacts.is_empty() {
        return;
    }
    let Some(ws) = active else { return };
    let Some(world) = physics.world(ws) else { return };
    // Bodies of indexed placements with a known material: (placement, reach, noise kind).
    let sounding: BTreeMap<BodyId, (PlacementId, f32, TagId)> = defs
        .0
        .iter()
        .filter_map(|(&placement, def)| {
            let m = materials.get(def.material?)?;
            Some((streaming.body(ws, placement)?, (placement, m.impact_reach_m, m.noise)))
        })
        .collect();
    let debris: BTreeSet<BodyId> = pieces.iter().filter(|b| &b.worldspace == ws).map(|b| b.id).collect();
    let skip = |id: BodyId| {
        debris.contains(&id) || world.layer(id).is_some_and(|l| l.intersects(Layer::PROJECTILE.union(Layer::HELD)))
    };
    for (a, b, speed, at) in contacts {
        if skip(a) || skip(b) {
            continue;
        }
        // The louder side (ties: the lower placement), with the body that made it.
        let side = |id: BodyId| {
            let &(placement, reach, kind) = sounding.get(&id)?;
            Some((loudness_m(reach, speed, &p)?, placement, kind))
        };
        let Some((loud, placement, kind)) = [side(a), side(b)]
            .into_iter()
            .flatten()
            .max_by(|x, y| x.0.total_cmp(&y.0).then(y.1.cmp(&x.1)))
        else {
            continue;
        };
        let thrower = thrown.get(ws, a).or_else(|| thrown.get(ws, b)).map(|t| t.user);
        inbox.push(Noise {
            source: source_key(placement),
            at,
            kind,
            loudness_m: loud,
            maker: thrower,
            reveals_maker: false,
            case: if thrower.is_some() { NoiseCase::ThrownLanding } else { NoiseCase::Impact },
        });
    }
}

/// The tick's noises thinned and written (module docs, "Thinning", "Out").
#[allow(clippy::needless_pass_by_value)]
fn emit(
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    tick: Res<InteractionTick>,
    mut inbox: ResMut<NoiseInbox>,
    mut out: MessageWriter<InteractionNoise>,
) {
    let p = effective_params(&settings, fixed.as_deref());
    let NoiseInbox { pending, merge, .. } = &mut *inbox;
    // Windows expire by tick inside the next step: an empty tick has nothing to do.
    if pending.is_empty() {
        return;
    }
    for n in merge.step(tick.0, pending.drain(..), &p) {
        out.write(InteractionNoise {
            tick: tick.0,
            at: n.at,
            space: None,
            kind: n.kind,
            loudness_m: n.loudness_m,
            maker: n.maker,
            reveals_maker: n.reveals_maker,
            case: n.case,
        });
    }
}
