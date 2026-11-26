//! Swing doors (`docs/specs/interaction.md`, IN33): the resident doors, the default for their
//! `open` / `close` use, and their swing on the fixed tick. The state is the core's [`DoorState`].
//!
//! - **Registry** ([`Doors`], `FixedUpdate`, in `InteractionSystems::Apply` after the claim step):
//!   every [`IndexChange::Insert`] whose def has a [`sf_interaction::SlotKind::Door`] slot registers
//!   the placement with its def, world frame and doorway (the door slot's `local.translation`, the
//!   closed leaf's centre, in the object's frame). A re-insert keeps the door's state; a re-insert
//!   without a door slot, or a removal, drops it and clears its [`DOOR_OPEN`] tag. A move refreshes
//!   its frame. Changes apply in send order.
//! - **Default** (`Update`, [`DoorSystems::Default`], after `ActivationSystems::Default`): a
//!   [`UseApproved`] on a registered door toggles it: verb `open` only when its target is closed,
//!   `close` only when open, no verb always, any other verb never. The user is the
//!   [`InteractionUser`] whose actor made the activation; its position (the body's feet, else the
//!   doorway) picks the swing side in the core. No such user (a script) toggles with no user. Each
//!   toggle writes one [`DoorChanged`] and sets or clears [`DOOR_OPEN`] in
//!   [`InteractionStateTags`].
//! - **Plan** (`FixedUpdate`, [`DoorSystems::Plan`], after `InteractionSystems::Apply`), over the
//!   doors in placement order:
//!   1. close-behind arm: a [`ClaimChanged`] `Released` or `Invalidated` on a door's slot hands the
//!      user to [`DoorState::released`] (the core schedules only for the opener of an open
//!      `close_behind` door);
//!   2. close-behind fire: [`DoorState::poll`]; a due close writes [`DoorChanged`] (no user) and
//!      clears [`DOOR_OPEN`]; the swing starts the same tick;
//!   3. sweep: [`DoorSweeps`] is rebuilt with one [`DoorSweep`] per moving door: the leaf's world
//!      frame after this tick's step ([`DoorState::stepped`]), `blocked` false.
//! - **Host test** (between [`DoorSystems::Plan`] and [`DoorSystems::Commit`]): the host tests each
//!   sweep's `next` leaf against its bodies and calls [`DoorSweeps::block`] on a hit. This crate runs
//!   no physics.
//! - **Commit** (`FixedUpdate`, [`DoorSystems::Commit`]): a blocked door does not move this tick (it
//!   keeps `t` and its target); any other swept door takes its step and writes one [`DoorMoved`]
//!   with the leaf's new world frame (the arriving tick included; none at rest). A door gone since
//!   Plan is skipped. A swinging leaf never sends an [`IndexChange::Move`]: the object's frame (and
//!   its claims) stay put.
//!
//! Plan and Commit step by one fixed tick (`1 / tick_hz` s, `interact.door.swing_s` closed to open).

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::Vec3;
use sf_action_bevy::ActionBody;
use sf_interaction::{
    DOOR_OPEN, DoorState, EventKind, Frame, InteractParams, InteractableDef, VERB_CLOSE, VERB_OPEN, to_local,
};
use sf_world::PlacementId;

use crate::seat::feet_of;
use crate::{
    ClaimChanged, IndexChange, InteractionSettings, InteractionStateTags, InteractionTick, InteractionUser,
    UseApproved, effective_params,
};

/// One resident swing door.
#[derive(Clone, Debug)]
pub struct Door {
    pub def: Arc<InteractableDef>,
    /// The object's world frame, as last inserted or moved.
    pub object: Frame,
    /// The door slot's `local.translation`: the closed leaf's centre, in the object's frame.
    pub doorway: Vec3,
    pub state: DoorState,
}

/// The resident swing doors, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct Doors(BTreeMap<PlacementId, Door>);

impl Doors {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&Door> {
        self.0.get(&placement)
    }

    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &Door)> {
        self.0.iter().map(|(p, d)| (*p, d))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn get_mut(&mut self, placement: PlacementId) -> Option<&mut Door> {
        self.0.get_mut(&placement)
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (PlacementId, &mut Door)> {
        self.0.iter_mut().map(|(p, d)| (*p, d))
    }
}

/// One moving door's planned step, for the host's blocker test.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoorSweep {
    /// The leaf's world frame after this tick's step.
    pub next: Frame,
    /// Set by the host: the step is not taken this tick (a pause).
    pub blocked: bool,
}

/// This tick's planned steps of the moving doors, by placement (rebuilt by Plan each tick).
#[derive(Resource, Clone, Debug, Default)]
pub struct DoorSweeps(BTreeMap<PlacementId, DoorSweep>);

impl DoorSweeps {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&DoorSweep> {
        self.0.get(&placement)
    }

    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &DoorSweep)> {
        self.0.iter().map(|(p, s)| (*p, s))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PlacementId, &mut DoorSweep)> {
        self.0.iter_mut().map(|(p, s)| (*p, s))
    }

    /// Holds the door at `placement` this tick (nothing if it has no sweep).
    pub fn block(&mut self, placement: PlacementId) {
        if let Some(sweep) = self.0.get_mut(&placement) {
            sweep.blocked = true;
        }
    }
}

/// One toggle of one door (the core's event): sound, nav links and scripts read it.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DoorChanged(pub sf_interaction::DoorChanged);

/// One committed step of one door's leaf: its new world frame (the host moves the leaf's body and
/// mesh). One per door per moving tick, none at rest or while blocked.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct DoorMoved {
    pub placement: PlacementId,
    pub leaf: Frame,
}

/// Where the door systems run.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DoorSystems {
    /// `Update`: approved uses on doors into toggles.
    Default,
    /// `FixedUpdate`, after `InteractionSystems::Apply`: close-behind, then [`DoorSweeps`].
    Plan,
    /// `FixedUpdate`, after [`Self::Plan`]: the unblocked steps, [`DoorMoved`] out. The host's
    /// blocker test runs between the two.
    Commit,
}

/// The registry follows the tick's index changes (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn register(
    mut changes: MessageReader<IndexChange>,
    mut doors: ResMut<Doors>,
    mut tags: ResMut<InteractionStateTags>,
) {
    for change in changes.read() {
        match change {
            IndexChange::Insert { placement, def, object, .. } => {
                let Some((_, slot, _)) = def.door() else {
                    if doors.0.remove(placement).is_some() {
                        tags.remove(*placement, DOOR_OPEN);
                    }
                    continue;
                };
                let doorway = slot.local.translation;
                let state = doors.0.get(placement).map_or_else(DoorState::default, |d| d.state);
                doors.0.insert(*placement, Door { def: Arc::clone(def), object: *object, doorway, state });
            }
            IndexChange::Move { placement, object } => {
                if let Some(door) = doors.0.get_mut(placement) {
                    door.object = *object;
                }
            }
            IndexChange::Remove { placement, .. } => {
                if doors.0.remove(placement).is_some() {
                    tags.remove(*placement, DOOR_OPEN);
                }
            }
        }
    }
}

/// Whether an approved `verb` acts on a door whose target is `open`.
fn verb_acts(verb: Option<sf_interaction::TagId>, open: bool) -> bool {
    match verb {
        None => true,
        Some(v) if v == VERB_OPEN => !open,
        Some(v) if v == VERB_CLOSE => open,
        Some(_) => false,
    }
}

/// Approved uses on registered doors into toggles (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn door_default(
    mut approved: MessageReader<UseApproved>,
    users: Query<(&InteractionUser, Option<&ActionBody>)>,
    tick: Res<InteractionTick>,
    mut doors: ResMut<Doors>,
    mut tags: ResMut<InteractionStateTags>,
    mut out: MessageWriter<DoorChanged>,
) {
    for a in approved.read() {
        let placement = PlacementId(a.activation.target.0);
        let Some(door) = doors.0.get_mut(&placement) else { continue };
        if !verb_acts(a.verb, door.state.open) {
            continue;
        }
        let Some((_, _, door_def)) = door.def.door() else { continue };
        let user = users.iter().find(|(u, _)| u.actor == a.activation.actor).map(|(u, body)| {
            let at = body.map_or(door.doorway, |b| to_local(&door.object, feet_of(b)));
            (u.user, at)
        });
        let changed = door.state.toggle(placement, door_def, door.doorway, user, tick.0);
        if changed.open {
            tags.insert(placement, DOOR_OPEN);
        } else {
            tags.remove(placement, DOOR_OPEN);
        }
        out.write(DoorChanged(changed));
    }
}

/// One fixed tick's step in seconds and the swing time, from the effective params.
fn swing_step(params: &InteractParams) -> (f32, f32) {
    (1.0 / params.tick_hz, params.door_swing_s)
}

/// Close-behind, then this tick's sweeps (module docs, Plan).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub(crate) fn plan(
    mut claims: MessageReader<ClaimChanged>,
    tick: Res<InteractionTick>,
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    mut doors: ResMut<Doors>,
    mut sweeps: ResMut<DoorSweeps>,
    mut tags: ResMut<InteractionStateTags>,
    mut out: MessageWriter<DoorChanged>,
) {
    let params = effective_params(&settings, fixed.as_deref());
    let (dt, swing_s) = swing_step(&params);
    let close_behind_ticks = params.door_close_behind_ticks();

    for ClaimChanged(event) in claims.read() {
        if !matches!(event.kind, EventKind::Released | EventKind::Invalidated(_)) {
            continue;
        }
        let Some(door) = doors.get_mut(event.slot.placement) else { continue };
        let Some((slot, _, door_def)) = door.def.door() else { continue };
        if slot == event.slot.slot {
            door.state.released(event.user, door_def, tick.0, close_behind_ticks);
        }
    }

    sweeps.0.clear();
    for (placement, door) in doors.iter_mut() {
        let Some((_, _, door_def)) = door.def.door() else { continue };
        if let Some(changed) = door.state.poll(placement, door_def, door.doorway, tick.0) {
            tags.remove(placement, DOOR_OPEN);
            out.write(DoorChanged(changed));
        }
        if door.state.is_moving() {
            let next = door.state.stepped(dt, swing_s).leaf(door_def, &door.object);
            sweeps.0.insert(placement, DoorSweep { next, blocked: false });
        }
    }
}

/// The unblocked sweeps into steps (module docs, Commit).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn commit(
    sweeps: Res<DoorSweeps>,
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    mut doors: ResMut<Doors>,
    mut out: MessageWriter<DoorMoved>,
) {
    let (dt, swing_s) = swing_step(&effective_params(&settings, fixed.as_deref()));
    for (placement, sweep) in sweeps.iter() {
        if sweep.blocked {
            continue;
        }
        let Some(door) = doors.get_mut(placement) else { continue };
        let Some((_, _, door_def)) = door.def.door() else { continue };
        door.state = door.state.stepped(dt, swing_s);
        out.write(DoorMoved { placement, leaf: door.state.leaf(door_def, &door.object) });
    }
}
