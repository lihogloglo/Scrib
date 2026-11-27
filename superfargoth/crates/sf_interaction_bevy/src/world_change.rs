//! `WorldChange` shell (`docs/specs/world-reactions.md` "Pipeline" 1 "Emit", slice `IN14b`): the tick's
//! door toggles, light changes and released breaks become the core's [`WorldChange`]s, one per
//! transition ([`fold_tick`]), written as [`WorldChanged`] on `FixedUpdate` in [`WorldChangeSystems`]. The
//! readers (the AI concern board AI19, crime witnesses) are the host's; this crate names neither.
//!
//! - **Door** ([`DoorChanged`] of a registered door): `Opened` / `Closed` at the object
//!   ([`from_door`]); the toggling user is the culprit and the verb the cause (a close-behind or a
//!   script: neither). A door that left the registry before the step (its cell unloaded) is dropped.
//! - **Light** ([`LightChanged`]): `Snuffed` / `Relit` at the flame ([`from_light`]); no rule cause yet
//!   (water and fire snuffing are later rows).
//! - **Break** ([`Destroyed`]): `Destroyed` on the def's final stage or a removing one
//!   ([`from_destroyed`], the cause = the hit's damage kind), at the object. One whose placement left
//!   meanwhile counts only if it removes, at the hit point (no def to count stages by).
//! - **Owner and home** come from [`ChangeOwners`], filled by the host from its ownership data (none:
//!   unowned). The core never names crime; the host maps [`sf_interaction::ChangeOwner`] and the
//!   culprit's `UserKey`.
//!
//! The tick's changes are gathered in send order (doors, lights, breaks), stably sorted by tick (a light
//! changed in `Update` is read by the next fixed step), folded, then written. Order: `FixedUpdate`, after
//! `DoorSystems::Commit` and `DestroySystems::Release`; hosts read [`WorldChanged`]
//! `.after(WorldChangeSystems)`.

use std::collections::BTreeMap;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use sf_interaction::{ChangeOwner, ChangeSite, WorldChange, fold_tick, from_destroyed, from_door, from_light};
use sf_world::PlacementId;

use crate::{DestroySystems, Destroyed, Destructibles, DoorChanged, DoorSystems, Doors, LightChanged};

/// One lasting change of one object (the core's [`WorldChange`]).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct WorldChanged(pub WorldChange);

/// Who owns a placement and the placement it belongs to, filled by the host (crime.md "Ownership").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChangeOwnership {
    pub owner: Option<ChangeOwner>,
    /// The container's or building's placement.
    pub home: Option<PlacementId>,
}

/// The host's ownership of the resident placements (module docs); a placement not in it is unowned.
#[derive(Resource, Clone, Debug, Default)]
pub struct ChangeOwners(BTreeMap<PlacementId, ChangeOwnership>);

impl ChangeOwners {
    /// Sets (or replaces) `placement`'s ownership.
    pub fn insert(&mut self, placement: PlacementId, ownership: ChangeOwnership) {
        self.0.insert(placement, ownership);
    }

    /// Forgets `placement` (it left).
    pub fn remove(&mut self, placement: PlacementId) {
        self.0.remove(&placement);
    }

    #[must_use]
    pub fn get(&self, placement: PlacementId) -> ChangeOwnership {
        self.0.get(&placement).copied().unwrap_or_default()
    }

    fn site(&self, placement: PlacementId, at: glam::Vec3) -> ChangeSite {
        let o = self.get(placement);
        ChangeSite { at, owner: o.owner, home: o.home }
    }
}

/// Where the tick's changes are gathered and written: `FixedUpdate`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldChangeSystems;

pub(crate) fn build(app: &mut App) {
    app.init_resource::<ChangeOwners>()
        .add_message::<WorldChanged>()
        .configure_sets(
            FixedUpdate,
            WorldChangeSystems.after(DoorSystems::Commit).after(DestroySystems::Release),
        )
        .add_systems(FixedUpdate, emit.in_set(WorldChangeSystems));
}

/// The tick's door, light and break events into folded [`WorldChanged`]s (module docs).
#[allow(clippy::needless_pass_by_value)]
fn emit(
    doors: Res<Doors>,
    registry: Res<Destructibles>,
    owners: Res<ChangeOwners>,
    mut door_changed: MessageReader<DoorChanged>,
    mut light_changed: MessageReader<LightChanged>,
    mut destroyed: MessageReader<Destroyed>,
    mut out: MessageWriter<WorldChanged>,
) {
    let mut changes: Vec<WorldChange> = Vec::new();
    for c in door_changed.read() {
        let Some(door) = doors.get(c.0.placement) else { continue };
        changes.push(from_door(&c.0, &owners.site(c.0.placement, door.object.translation)));
    }
    for c in light_changed.read() {
        changes.push(from_light(&c.0, &owners.site(c.0.placement, c.0.at), None));
    }
    for d in destroyed.read() {
        let e = &d.0;
        let (at, stages) = registry
            .get(e.placement)
            .map_or((e.point, usize::MAX), |o| (o.object.translation, o.destructible().map_or(0, |def| def.stages.len())));
        changes.extend(from_destroyed(e, stages, &owners.site(e.placement, at)));
    }
    if changes.is_empty() {
        return;
    }
    changes.sort_by_key(|c| c.tick);
    fold_tick(&mut changes);
    out.write_batch(changes.into_iter().map(WorldChanged));
}
