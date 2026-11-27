//! `IN38b1` headless tests of the breakables wiring: the records table (scripted and script-named
//! records refused, decorative statics counted apart), a breakable placement registered by the
//! shell with the rules filled (and kept indexed with sitting off), its state through the overlay
//! and back on reactivation, and the dev key breaking the focused placement.

#![allow(clippy::float_cmp)] // exact by construction: tunable defaults and their sums

use std::collections::BTreeMap;
use std::sync::{Arc, PoisonError};

use bevy_app::{App, Update};
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use glam::{DVec3, Quat};
use mw_interaction::breakable::{kind, material};
use mw_interaction::{BreakClass, Breakable, Refusal};
use mw_player_bevy::{Focus, FocusHit};
use sf_interaction::{DESTRUCT_STATE_OWNER, DestructState, InvalidReason};
use sf_interaction_bevy::{Destructibles, DestructionRules, IndexChange, PropDamage};
use sf_records::{RecordDb, RecordId};
use sf_script_bevy::sf_script::Handle;
use sf_tunables_bevy::{SetBy, Tunables};
use sf_world::{IndexedPlacement, PlacementClass, PlacementId};
use sf_world_bevy::{ActivateCx, PlacementSpawner, WorldOverlay};
use tes3::esp::{Container, ContainerFlags, DialogueInfo, MiscItem, Script, Static};

use crate::breakable::{BreakCounts, BreakablesTable, DEV_BREAK_KEY, dev_break};
use crate::index::FurnitureHook;
use crate::tests::{Nothing, app, cell, placement, tick};
use crate::tests_door::ws;
use crate::{ResidentFurniture, SitSwitch};

const CRATE: PlacementId = PlacementId(80);
const POT: PlacementId = PlacementId(81);
const CRATE_BASE: &str = "contain_crate_01";
const POT_BASE: &str = "misc_com_redware_pot_01";

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

#[test]
fn table_lists_breakables_and_refuses_scripted_ones() {
    let mut db = RecordDb::new();
    db.register::<Container>("CONT").unwrap();
    db.register::<MiscItem>("MISC").unwrap();
    db.register::<Static>("STAT").unwrap();
    db.register::<Script>("SCPT").unwrap();
    db.register::<DialogueInfo>("INFO").unwrap();
    let src = db.add_source("test", &[]);
    let crate_ = |name: &str, script: &str, flags| Container { id: name.into(), script: script.into(), container_flags: flags, ..Default::default() };
    db.define(src, id("contain_crate_01"), crate_("contain_crate_01", "", ContainerFlags::empty())).unwrap();
    db.define(src, id("contain_crate_quest"), crate_("contain_crate_quest", "crate_trap", ContainerFlags::empty())).unwrap();
    db.define(src, id("contain_crate_tidy"), crate_("contain_crate_tidy", "crate_tidy", ContainerFlags::empty())).unwrap();
    db.define(src, id("contain_crate_named"), crate_("contain_crate_named", "", ContainerFlags::empty())).unwrap();
    db.define(src, id("contain_sack_plant"), crate_("contain_sack_plant", "", ContainerFlags::ORGANIC)).unwrap();
    db.define(src, id("com_chest_01"), crate_("com_chest_01", "", ContainerFlags::empty())).unwrap();
    db.define(src, id(POT_BASE), MiscItem { id: POT_BASE.into(), ..Default::default() }).unwrap();
    db.define(src, id("misc_silverware_cup"), MiscItem { id: "misc_silverware_cup".into(), ..Default::default() }).unwrap();
    db.define(src, id("furn_urn_01"), Static { id: "furn_urn_01".into(), ..Default::default() }).unwrap();
    let script = |name: &str, text: &str| Script { id: name.into(), text: text.into(), ..Default::default() };
    db.define(src, id("crate_trap"), script("crate_trap", "begin crate_trap\nif ( OnActivate == 1 )\nActivate\nendif\nend")).unwrap();
    // A script without an activation block keeps its record breakable.
    db.define(src, id("crate_tidy"), script("crate_tidy", "begin crate_tidy\nshort done\nend")).unwrap();
    db.define(src, id("quest"), script("quest", "begin quest\n\"contain_crate_named\"->Disable\nend")).unwrap();

    let (table, counts) = BreakablesTable::build(&db);
    let listed: BTreeMap<&str, Breakable> = table.0.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    assert_eq!(
        listed,
        BTreeMap::from([
            ("contain_crate_01", Breakable { class: BreakClass::Crate, spills: true }),
            ("contain_crate_tidy", Breakable { class: BreakClass::Crate, spills: true }),
            (POT_BASE, Breakable { class: BreakClass::Pottery, spills: false }),
            ("furn_urn_01", Breakable { class: BreakClass::Urn, spills: false }),
        ])
    );
    assert_eq!(table.static_ids(&db), std::collections::HashSet::from(["furn_urn_01".to_owned()]));
    assert_eq!(
        counts,
        BreakCounts {
            breakable: BTreeMap::from([(BreakClass::Crate, 2), (BreakClass::Pottery, 1)]),
            refused: BTreeMap::from([(Refusal::Organic, 1), (Refusal::ActivateScript, 1), (Refusal::NamedByScript, 1)]),
            statics: 1,
        }
    );
}

/// An app with an empty overlay and two breakables: a crate (no furniture def) and a pot.
fn break_app() -> App {
    let mut app = app();
    app.init_resource::<WorldOverlay>();
    let table = BTreeMap::from([
        (id(CRATE_BASE), Breakable { class: BreakClass::Crate, spills: true }),
        (id(POT_BASE), Breakable { class: BreakClass::Pottery, spills: false }),
    ]);
    app.insert_resource(BreakablesTable(Arc::new(table)));
    app
}

fn crate_placement() -> IndexedPlacement {
    let mut p = placement(CRATE, CRATE_BASE, DVec3::new(2.0, 0.0, 3.0), Quat::IDENTITY);
    p.placement.class = mw_world::source::CONTAINER;
    p
}

fn residency(app: &mut App, p: &IndexedPlacement, on: bool) {
    let hook = FurnitureHook(Arc::new(Nothing));
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let worldspace = ws();
    let cx = ActivateCx { worldspace: &worldspace, cell: cell(), cell_entity, discard: false };
    if on {
        hook.activate(world, &cx, p);
    } else {
        hook.deactivate(world, &cx, p);
    }
}

fn health(app: &App, p: PlacementId) -> Option<f32> {
    app.world().resource::<Destructibles>().get(p).map(|d| d.state.health)
}

fn hit(app: &mut App, placement: PlacementId, amount: f32) {
    let point = glam::Vec3::ZERO;
    app.world_mut().write_message(PropDamage { placement, amount, kind: kind::BLUNT, point, dir: glam::Vec3::NEG_Y, culprit: None });
}

#[test]
fn breakables_register_with_rules_and_stay_indexed_with_sitting_off() {
    let mut app = break_app();
    residency(&mut app, &crate_placement(), true);
    let mut pot = placement(POT, POT_BASE, DVec3::new(1.0, 1.0, 0.0), Quat::IDENTITY);
    pot.placement.class = PlacementClass::Dynamic;
    residency(&mut app, &pot, true);
    // A placement with neither a def nor a breakable record registers nothing.
    residency(&mut app, &placement(PlacementId(82), "rock_01", DVec3::ZERO, Quat::IDENTITY), true);
    tick(&mut app, 1);
    let registry = app.world().resource::<Destructibles>();
    assert_eq!(registry.len(), 2);
    let crate_ = registry.get(CRATE).expect("the crate registers");
    assert_eq!(crate_.def.material, Some(material::WOOD));
    assert!(crate_.def.slots.is_empty() && crate_.def.verbs.is_empty(), "no furniture: a slotless def");
    assert_eq!(health(&app, CRATE), Some(15.0));
    assert_eq!(health(&app, POT), Some(2.0));
    let rules = app.world().resource::<DestructionRules>();
    assert_eq!(rules.factor(kind::BLADE, Some(material::WOOD)), 0.5);
    assert_eq!(rules.factor(kind::FIRE, Some(material::WOOD)), 1.5);
    assert_eq!(rules.factor(kind::FIRE, Some(material::CLAY)), 1.0, "absent row: 1");

    // Sitting off removes furniture, never a breakable.
    let sit = app.world().resource::<SitSwitch>().0;
    app.world_mut().resource_mut::<Tunables>().write(sit, false, SetBy::Console).expect("bool");
    app.update();
    tick(&mut app, 1);
    let messages = app.world().resource::<bevy_ecs::message::Messages<IndexChange>>();
    let removed = messages.get_cursor().read(messages).any(|c| matches!(c, IndexChange::Remove { reason: InvalidReason::SlotDisabled, .. }));
    assert!(!removed, "no breakable left the index");
    assert!(app.world().resource::<ResidentFurniture>().indexed(CRATE));
    assert_eq!(app.world().resource::<Destructibles>().len(), 2);

    // The switch off (Classic): a newly resident breakable gets no def.
    let off = app.world().resource::<Tunables>().id_of(&mw_interaction::tunables::BREAK).expect("registered");
    app.world_mut().resource_mut::<Tunables>().write(off, false, SetBy::Console).expect("bool");
    app.update();
    residency(&mut app, &crate_placement(), false);
    tick(&mut app, 1);
    residency(&mut app, &crate_placement(), true);
    tick(&mut app, 1);
    assert!(health(&app, CRATE).is_none(), "Classic: the crate does not register");
    assert_eq!(app.world().resource::<DestructionRules>(), &DestructionRules::default(), "Classic: no rows");
}

fn saved(app: &App, p: PlacementId) -> Option<Vec<u8>> {
    let overlay = app.world().resource::<WorldOverlay>();
    let overlay = overlay.0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.state(p, DESTRUCT_STATE_OWNER).map(<[u8]>::to_vec)
}

#[test]
fn damage_is_saved_to_the_overlay_and_restored_on_reactivation() {
    let mut app = break_app();
    residency(&mut app, &crate_placement(), true);
    tick(&mut app, 1);
    hit(&mut app, CRATE, 6.0);
    tick(&mut app, 1);
    assert_eq!(health(&app, CRATE), Some(9.0));
    assert_eq!(saved(&app, CRATE), Some(DestructState { health: 9.0, stage: 0 }.rest_bytes().to_vec()));
    // The cell unloads and loads again: the shell starts from the saved state, not intact.
    residency(&mut app, &crate_placement(), false);
    tick(&mut app, 1);
    assert!(health(&app, CRATE).is_none());
    residency(&mut app, &crate_placement(), true);
    tick(&mut app, 1);
    assert_eq!(health(&app, CRATE), Some(9.0));
    // The breaking hit (a removing stage) writes no bytes: the last standing state stays.
    hit(&mut app, CRATE, 20.0);
    tick(&mut app, 1);
    assert!(app.world().resource::<Destructibles>().get(CRATE).is_some_and(|d| d.gone));
    assert_eq!(saved(&app, CRATE), Some(DestructState { health: 9.0, stage: 0 }.rest_bytes().to_vec()));
}

#[test]
fn dev_key_breaks_the_focused_placement() {
    let mut app = break_app();
    app.init_resource::<ButtonInput<KeyCode>>().add_systems(Update, dev_break);
    residency(&mut app, &crate_placement(), true);
    app.world_mut().resource_mut::<Focus>().hit = Some(FocusHit { handle: Handle(CRATE.0), distance: 1.0 });
    tick(&mut app, 3);
    let focused = app.world().resource::<sf_interaction_bevy::PlayerFocus>().get(crate::input::PLAYER_USER).and_then(sf_interaction_bevy::FocusState::target);
    assert_eq!(focused, Some(CRATE));
    // No press: nothing.
    app.world_mut().run_schedule(Update);
    tick(&mut app, 1);
    assert_eq!(health(&app, CRATE), Some(15.0));
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(DEV_BREAK_KEY);
    app.world_mut().run_schedule(Update);
    tick(&mut app, 1);
    let crate_ = app.world().resource::<Destructibles>().get(CRATE).expect("still registered until released");
    assert_eq!(crate_.state, DestructState { health: 0.0, stage: 1 }, "one press runs every stage");
    assert!(crate_.gone);
}
