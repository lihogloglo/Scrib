//! Tests of [`super`]: one `DoorChanged` per container window open and close.

use bevy_app::App;
use bevy_ecs::message::Messages;
use mw_game_core::player::HOST_KEY;
use sf_interaction_bevy::sf_interaction::{Tick, UserKey};
use sf_interaction_bevy::{DoorChanged, InteractionTick};
use sf_records::RecordId;
use sf_rpg::ActorKey;
use sf_world::PlacementId;

use crate::inventory_ui::OpenContainer;

fn app() -> App {
    let mut app = App::new();
    super::add(&mut app);
    app
}

fn window(key: u128, record: &str) -> OpenContainer {
    OpenContainer { key: ActorKey(key), record: RecordId::new(record), organic: false }
}

/// Every message written so far, as `(placement, open, user, tick)`.
fn sent(app: &App) -> Vec<(PlacementId, bool, Option<UserKey>, Tick)> {
    let messages = app.world().resource::<Messages<DoorChanged>>();
    messages.iter_current_update_messages().map(|m| (m.0.placement, m.0.open, m.0.user, m.0.tick)).collect()
}

const HOST: Option<UserKey> = Some(UserKey::Player(HOST_KEY));

#[test]
fn open_then_close_sends_one_each() {
    let mut app = app();
    let e = app.world_mut().spawn(window(42, "barrel_01")).id();
    assert_eq!(sent(&app), vec![(PlacementId(42), true, HOST, Tick::default())]);
    app.world_mut().despawn(e);
    assert_eq!(
        sent(&app),
        vec![(PlacementId(42), true, HOST, Tick::default()), (PlacementId(42), false, HOST, Tick::default())]
    );
}

#[test]
fn carries_the_interaction_tick() {
    let mut app = app();
    app.insert_resource(InteractionTick(Tick(7)));
    app.world_mut().spawn(window(42, "barrel_01"));
    assert_eq!(sent(&app), vec![(PlacementId(42), true, HOST, Tick(7))]);
}

#[test]
fn two_windows_keep_their_placements() {
    let mut app = app();
    let a = app.world_mut().spawn(window(1, "barrel_01")).id();
    app.world_mut().spawn(window(2, "chest_small_01"));
    app.world_mut().despawn(a);
    assert_eq!(
        sent(&app),
        vec![(PlacementId(1), true, HOST, Tick::default()), (PlacementId(2), true, HOST, Tick::default()), (PlacementId(1), false, HOST, Tick::default())]
    );
}
