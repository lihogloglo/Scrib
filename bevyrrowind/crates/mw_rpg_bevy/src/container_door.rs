//! A container's window as a door event (IN33d): when a container window opens for the host
//! player the game writes one [`DoorChanged`] with `open: true` for the container's placement, and
//! one with `open: false` when that window closes. A container has no leaf: nothing moves and
//! nothing is registered as a door; readers (IN14 `WorldChange` `Opened` / `Closed`, nav, scripts)
//! filter by the placement's class, so a searched corpse sends it too.
//!
//! The source is the window's lifecycle, not `ContainerOpened`: the window root's
//! [`OpenContainer`] is added when the window opens and removed when it closes (a despawn included),
//! so open and close always pair, and an approved open that shows no window sends nothing. This is
//! the host player's window (single host today); a co-op host will send the same message from its
//! own record of which player has which container open.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use mw_game_core::player::HOST_KEY;
use sf_interaction_bevy::sf_interaction::{self, UserKey};
use sf_interaction_bevy::{DoorChanged, InteractionTick};
use sf_world::PlacementId;

use crate::inventory_ui::OpenContainer;

#[cfg(test)]
#[path = "container_door_tests.rs"]
mod container_door_tests;

/// The [`DoorChanged`] message (this crate also runs without the interaction plugin) and the two
/// window observers.
pub fn add(app: &mut App) {
    app.add_message::<DoorChanged>().add_observer(window_opened).add_observer(window_closed);
}

/// The host player's change of one container window.
fn change(open: &OpenContainer, opened: bool, tick: Option<&InteractionTick>) -> DoorChanged {
    DoorChanged(sf_interaction::DoorChanged {
        placement: PlacementId(open.key.0),
        open: opened,
        user: Some(UserKey::Player(HOST_KEY)),
        tick: tick.map(|t| t.0).unwrap_or_default(),
    })
}

/// A container window opened: `open: true` for its placement.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn window_opened(
    add: On<Add<OpenContainer>>,
    windows: Query<&OpenContainer>,
    tick: Option<Res<InteractionTick>>,
    mut changed: MessageWriter<DoorChanged>,
) {
    let Ok(open) = windows.get(add.entity) else { return };
    changed.write(change(open, true, tick.as_deref()));
}

/// A container window closed (its root lost [`OpenContainer`] or despawned): `open: false`.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn window_closed(
    remove: On<Remove<OpenContainer>>,
    windows: Query<&OpenContainer>,
    tick: Option<Res<InteractionTick>>,
    mut changed: MessageWriter<DoorChanged>,
) {
    let Ok(open) = windows.get(remove.entity) else { return };
    changed.write(change(open, false, tick.as_deref()));
}
