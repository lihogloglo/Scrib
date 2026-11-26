//! Swing doors (`docs/specs/interaction.md`, IN33): the resident doors and the default for their
//! `open` / `close` use. The swing itself is the core's [`DoorState`].
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

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_action_bevy::ActionBody;
use sf_interaction::{DOOR_OPEN, DoorState, Frame, InteractableDef, VERB_CLOSE, VERB_OPEN, to_local};
use sf_world::PlacementId;

use crate::seat::feet_of;
use crate::{IndexChange, InteractionStateTags, InteractionTick, InteractionUser, UseApproved};

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

    #[allow(dead_code)] // the swing step uses it
    pub(crate) fn get_mut(&mut self, placement: PlacementId) -> Option<&mut Door> {
        self.0.get_mut(&placement)
    }

    #[allow(dead_code)] // the swing step uses it
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (PlacementId, &mut Door)> {
        self.0.iter_mut().map(|(p, d)| (*p, d))
    }
}

/// One toggle of one door (the core's event): sound, nav links and scripts read it.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DoorChanged(pub sf_interaction::DoorChanged);

/// Where the door default runs, in `Update`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DoorSystems {
    /// Approved uses on doors into toggles.
    Default,
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
