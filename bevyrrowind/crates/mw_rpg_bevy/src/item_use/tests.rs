//! Drinking potions headless (no install, `effects::test_records`): one fixed tick per update
//! (60 Hz), from the inventory screen's `Use` intent through the host to the player's effects.

#![allow(clippy::float_cmp)]

use std::time::Duration;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use mw_game_core::player::{HOST_KEY, PlayerCharacter};
use sf_effects_bevy::Effects;
use sf_records::RecordId;
use sf_rpg::{Attrs, Holder, PlayerId, RpgCommand, StackId};
use sf_stats_bevy::sf_stats::{CauseKind, StatOp, StatOwner};
use sf_stats_bevy::{StatCommand, StatTableRes, Stats};
use sf_ui_inventory::InventoryView;
use sf_ui_screens::{InventoryEvent, InventoryIntent, InventoryScreen};

use super::{DRINK_SOUND, ItemUsed, UseItemRequest};
use crate::dialogue::Rpg;
use crate::effects::test_records::{
    self, FORTIFY_STRENGTH_POTION, MISC_ITEM, RESTORE_HEALTH_POTION,
};
use crate::inventory::ItemRecords;

const PLAYER: StatOwner = StatOwner::Player(HOST_KEY);
/// Fixed ticks per second.
const HZ: u32 = 60;

/// Collected [`ItemUsed`]s.
#[derive(Resource, Default)]
struct Used(Vec<ItemUsed>);

fn collect(mut used: MessageReader<ItemUsed>, mut out: ResMut<Used>) {
    out.0.extend(used.read().cloned());
}

/// The player's health at 70 of 100, `items` in the inventory.
fn app(items: &[(&str, u32)]) -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / f64::from(HZ),
        )))
        .insert_resource(Time::<Fixed>::from_hz(f64::from(HZ)));
    let db = test_records::db(&[]);
    crate::stats::add(&mut app, Some(db.clone()));
    let mut rpg = Rpg::new(db.clone()).expect("rpg host");
    for &(item, count) in items {
        let holder = Holder::Player(PlayerId::HOST);
        let add = RpgCommand::AddItem {
            holder,
            item: RecordId::new(item),
            count,
            attrs: Attrs::new(),
        };
        rpg.state.pc.apply(add).expect("item added");
    }
    app.insert_resource(rpg)
        .insert_resource(ItemRecords(db))
        .init_resource::<Used>();
    super::add(&mut app);
    app.add_systems(Update, collect.after(super::apply_item_use));
    app.update();
    let health = idx(&app, "health");
    let ops = vec![
        StatOp::SetBase {
            stat: health,
            value: 100.0,
        },
        StatOp::PoolSetCurrent {
            pool: health,
            value: 70.0,
        },
    ];
    app.world_mut()
        .write_message(StatCommand::new(PLAYER, CauseKind::Debug, ops));
    app.update();
    assert_eq!(health_of(&mut app), (70.0, 100.0));
    app
}

fn idx(app: &App, key: &str) -> sf_stats_bevy::sf_stats::StatIdx {
    app.world().resource::<StatTableRes>().0.find(key).unwrap()
}

fn player(app: &mut App) -> (&Stats, &Effects) {
    let mut q = app
        .world_mut()
        .query_filtered::<(&Stats, &Effects), With<PlayerCharacter>>();
    q.single(app.world()).unwrap()
}

/// Current and max health.
fn health_of(app: &mut App) -> (f32, f32) {
    let table = app.world().resource::<StatTableRes>().0.clone();
    let health = idx(app, "health");
    let view = player(app).0.view(&table);
    (view.current(health), view.max(health))
}

fn strength(app: &mut App) -> f32 {
    let table = app.world().resource::<StatTableRes>().0.clone();
    let strength = idx(app, "strength");
    player(app).0.view(&table).modified(strength)
}

/// The player's stacks: `(item, count)`.
fn inventory(app: &App) -> Vec<(String, u32)> {
    let pc = &app.world().resource::<Rpg>().state.pc;
    pc.loadout
        .inventory()
        .stacks()
        .iter()
        .map(|s| (s.item.as_str().to_owned(), s.count))
        .collect()
}

fn stack_of(app: &App, item: &str) -> StackId {
    let pc = &app.world().resource::<Rpg>().state.pc;
    pc.loadout
        .inventory()
        .stacks()
        .iter()
        .find(|s| s.item.as_str() == item)
        .expect("stack")
        .id
}

/// The player's inventory screen root, as `inventory_ui` spawns it (no UI plugins here).
fn screen(app: &mut App) -> Entity {
    let view = InventoryView::new(Holder::Player(PlayerId::HOST), Vec::new(), 1.0, 1.0);
    app.world_mut().spawn(InventoryScreen::new(view)).id()
}

/// The screen's Use on `item`'s stack.
fn use_from_screen(app: &mut App, screen: Entity, item: &str) {
    let stack = stack_of(app, item);
    app.world_mut().write_message(InventoryEvent {
        screen,
        intent: InventoryIntent::Use(stack),
    });
}

fn run(app: &mut App, ticks: u32) {
    for _ in 0..ticks {
        app.update();
    }
}

#[test]
fn a_restore_health_potion_from_the_inventory_heals_over_its_duration() {
    let mut app = app(&[(RESTORE_HEALTH_POTION, 2)]);
    let screen = screen(&mut app);
    use_from_screen(&mut app, screen, RESTORE_HEALTH_POTION);
    // The frame's `Update` applies the request (the potion is gone, `ItemUsed` out); the effect
    // lands on the next fixed tick (the next frame here).
    app.update();
    assert_eq!(
        inventory(&app),
        vec![(RESTORE_HEALTH_POTION.to_owned(), 1)],
        "exactly one drunk"
    );
    let used = &app.world().resource::<Used>().0;
    assert_eq!(used.len(), 1);
    assert_eq!(used[0].item.as_str(), RESTORE_HEALTH_POTION);
    assert_eq!(used[0].sound, Some(RecordId::new(DRINK_SOUND)));
    assert!(player(&mut app).1.active.is_empty(), "not landed yet");
    app.update();
    assert_eq!(player(&mut app).1.active.sources().len(), 1, "landed");
    let start = health_of(&mut app).0;
    // The landing tick delivers its first tick of the rate (10 / 60).
    assert!(
        (start - (70.0 + 10.0 / 60.0)).abs() < 1e-4,
        "at t = 0: {start}"
    );
    // 10 a second for 5 s from 70 would be 120: 2 s in it is 90, then it stops at the cap.
    run(&mut app, 2 * HZ);
    let mid = health_of(&mut app).0;
    assert!(
        (mid - (start + 20.0)).abs() < 1e-3,
        "2 s in: {mid} (from {start})"
    );
    run(&mut app, 3 * HZ);
    assert_eq!(health_of(&mut app), (100.0, 100.0), "ends at the cap");
    assert!(player(&mut app).1.active.is_empty(), "expired");
}

#[test]
fn a_fortify_strength_potion_raises_strength_for_its_duration() {
    let mut app = app(&[(FORTIFY_STRENGTH_POTION, 1)]);
    let base = strength(&mut app);
    let stack = stack_of(&app, FORTIFY_STRENGTH_POTION);
    app.world_mut().write_message(UseItemRequest {
        player: PlayerId::HOST,
        stack,
    });
    run(&mut app, 2);
    assert_eq!(strength(&mut app), base + 10.0, "fortified");
    assert!(
        inventory(&app).is_empty(),
        "the last potion removes the stack"
    );
    run(&mut app, HZ - 3);
    assert_eq!(strength(&mut app), base + 10.0, "still within 1 s");
    run(&mut app, 3);
    assert_eq!(strength(&mut app), base, "ended after 1 s");
}

#[test]
fn two_drinks_of_one_potion_stack_as_two_instances() {
    let mut app = app(&[(FORTIFY_STRENGTH_POTION, 2)]);
    let base = strength(&mut app);
    let screen = screen(&mut app);
    use_from_screen(&mut app, screen, FORTIFY_STRENGTH_POTION);
    run(&mut app, 10);
    use_from_screen(&mut app, screen, FORTIFY_STRENGTH_POTION);
    run(&mut app, 2);
    assert_eq!(strength(&mut app), base + 20.0, "both count");
    assert_eq!(
        player(&mut app).1.active.sources().len(),
        2,
        "a new instance, not a refresh"
    );
    assert!(inventory(&app).is_empty());
    // The first runs out on its own, then the second.
    run(&mut app, HZ - 10);
    assert_eq!(strength(&mut app), base + 10.0, "the first ended");
    run(&mut app, 10);
    assert_eq!(strength(&mut app), base, "both ended");
}

#[test]
fn using_something_else_does_nothing() {
    let mut app = app(&[(MISC_ITEM, 1), (RESTORE_HEALTH_POTION, 1)]);
    let screen = screen(&mut app);
    use_from_screen(&mut app, screen, MISC_ITEM);
    // An unknown stack and another player's inventory are refused too.
    let stack = stack_of(&app, RESTORE_HEALTH_POTION);
    app.world_mut().write_message(UseItemRequest {
        player: PlayerId::new(1).unwrap(),
        stack,
    });
    app.world_mut().write_message(UseItemRequest {
        player: PlayerId::HOST,
        stack: StackId(9999),
    });
    run(&mut app, 3);
    let mut have = inventory(&app);
    have.sort();
    assert_eq!(
        have,
        vec![
            (MISC_ITEM.to_owned(), 1),
            (RESTORE_HEALTH_POTION.to_owned(), 1)
        ]
    );
    assert!(app.world().resource::<Used>().0.is_empty());
    assert!(player(&mut app).1.active.is_empty());
    assert_eq!(health_of(&mut app), (70.0, 100.0));
}
