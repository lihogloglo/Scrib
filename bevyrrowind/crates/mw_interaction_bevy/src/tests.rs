//! `IN32c1` headless tests: furniture into the index on R0 activation, the press latch, the two
//! activation paths (def-less primary use, furniture verb with its slot), the defs product read.

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate, Update};
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use glam::{DVec3, Quat, Vec3};
use mw_bake::interact::{INTERACT_DEFS, InteractDefs, SUBJECT};
use mw_player_bevy::{Focus, FocusHit, FocusTarget, UsePresses};
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_interaction::{
    ClipFamily, Frame, IkTargets, InteractableDef, Occupancy, SlotDef, SlotKind, TagFilter, TagId, VerbDef,
};
use sf_interaction_bevy::{
    ActivateCommand, ActivationSystems, IndexChange, InteractionDefs, InteractionFrames, UseApproved,
};
use sf_records::RecordId;
use sf_script_bevy::sf_script::{Handle, ScriptRuntime};
use sf_script_bevy::{Activated, ActivationOutcome, PLAYER, Placed, ScriptPlacements, Scripts, ScriptsPlugin};
use sf_tunables_bevy::{SetBy, Tunables, TunablesPlugin};
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, LoadedModel, PlacementSpawner, SpawnCx};
use smallvec::{SmallVec, smallvec};

use crate::index::FurnitureHook;
use crate::{FurnitureDefs, SitSwitch, add_with_defs, furniture_prompt, load_defs};

const CHAIR: PlacementId = PlacementId(7);
const DOOR: PlacementId = PlacementId(10);
const ROCK: PlacementId = PlacementId(11);
const CHEST: PlacementId = PlacementId(12);

fn sit() -> TagId {
    TagId::of(mw_interaction::activation::verb::SIT)
}

/// A spawner that does nothing (the wrapped class's own).
pub(crate) struct Nothing;

impl PlacementSpawner for Nothing {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
    fn prepare(&self, _: &AssetServer, _: &IndexedPlacement, _: Option<&LoadedModel>) -> Option<UntypedHandle> {
        None
    }
}

#[derive(Resource, Default)]
struct Seen {
    index: Vec<IndexChange>,
    commands: Vec<ActivateCommand>,
    outcomes: Vec<ActivationOutcome>,
    approved: Vec<UseApproved>,
}

fn gather_fixed(mut seen: ResMut<Seen>, mut index: MessageReader<IndexChange>, mut commands: MessageReader<ActivateCommand>) {
    seen.index.extend(index.read().cloned());
    seen.commands.extend(commands.read().cloned());
}

fn gather(mut seen: ResMut<Seen>, mut outcomes: MessageReader<ActivationOutcome>, mut approved: MessageReader<UseApproved>) {
    seen.outcomes.extend(outcomes.read().copied());
    seen.approved.extend(approved.read().copied());
}

fn chair_def() -> Arc<InteractableDef> {
    let seat = SlotDef {
        kind: SlotKind::Pose { family: TagId::of("seat") },
        local: Frame { translation: Vec3::new(0.0, 0.0, 0.3), rotation: Quat::IDENTITY },
        band: 0.45,
        verb: VerbDef { verb: sit(), priority: 0, requires: TagFilter::default() },
        entrances: smallvec![],
        clips: ClipFamily("seat.chair".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    Arc::new(InteractableDef {
        id: RecordId::new("furn_chair_01"),
        slots: vec![seat],
        verbs: SmallVec::new(),
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

/// A census container that is also a seat, as the bake gives it: its seat slot plus the container's
/// own open verb, slotless and ranked first (`mw_interaction::class_verb`).
fn chest_def() -> Arc<InteractableDef> {
    let mut def = (*chair_def()).clone();
    def.id = RecordId::new("com_chest_seat");
    def.verbs.push(mw_interaction::class_verb(mw_interaction::RecordClass::Container).expect("a container opens"));
    Arc::new(def)
}

fn open() -> TagId {
    TagId::of(mw_interaction::activation::verb::OPEN)
}

fn defs() -> FurnitureDefs {
    FurnitureDefs(BTreeMap::from([
        (RecordId::new("furn_chair_01"), chair_def()),
        (RecordId::new("com_chest_seat"), chest_def()),
    ]))
}

pub(crate) fn app() -> App {
    let mut app = App::new();
    app.insert_resource(Scripts::new(ScriptRuntime::default()))
        .add_plugins((TunablesPlugin, ScriptsPlugin))
        .init_resource::<Seen>();
    add_with_defs(&mut app, defs());
    app.add_systems(FixedUpdate, gather_fixed.after(crate::input::commands))
        .add_systems(Update, gather.after(ActivationSystems::Default));
    let placed = |record: &str| Placed { record: record.into(), worldspace: "test".into(), position: [0.0; 3], radius: 0.5 };
    let mut p = app.world_mut().resource_mut::<ScriptPlacements>();
    for (h, record) in [(CHAIR, "furn_chair_01"), (DOOR, "ex_door_01"), (ROCK, "rock_01"), (CHEST, "com_chest_seat")] {
        p.loaded.insert(Handle(h.0), placed(record));
    }
    app.update();
    app
}

pub(crate) fn placement(id: PlacementId, record: &str, pos: DVec3, rotation: Quat) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id,
            base: RecordRef::new("STAT", record),
            model: None,
            transform: WorldTransform { pos: WorldPos::from_f64(pos), rotation, scale: Vec3::ONE },
            class: PlacementClass::Static,
            flags: PlacementFlags::empty(),
        },
        radius: 0.5,
    }
}

/// `placement` enters R0 (`on`) or leaves it, through the hook.
fn residency(app: &mut App, placement: &IndexedPlacement, on: bool) {
    let hook = FurnitureHook(Arc::new(Nothing));
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let ws = WorldspaceId::new("test");
    let cx = ActivateCx { worldspace: &ws, cell: cell(), cell_entity, discard: false };
    if on {
        hook.activate(world, &cx, placement);
    } else {
        hook.deactivate(world, &cx, placement);
    }
}

pub(crate) fn cell() -> glam::IVec3 {
    glam::IVec3::new(1, -2, 0)
}

pub(crate) fn tick(app: &mut App, n: usize) {
    for _ in 0..n {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

fn set_sit(app: &mut App, on: bool) {
    let id = app.world().resource::<SitSwitch>().0;
    app.world_mut().resource_mut::<Tunables>().write(id, on, SetBy::Console).expect("sit switch is a bool");
}

#[test]
fn furniture_with_a_def_is_indexed_on_activation_and_removed_on_deactivation() {
    let mut app = app();
    let pos = DVec3::new(10.0, 1.5, -4.0);
    let rot = Quat::from_rotation_y(0.5);
    let chair = placement(CHAIR, "furn_chair_01", pos, rot);
    residency(&mut app, &chair, true);
    tick(&mut app, 1);
    let seen = &app.world().resource::<Seen>().index;
    assert_eq!(seen.len(), 1, "one Insert: {seen:?}");
    let IndexChange::Insert { placement, def, object, disabled, .. } = &seen[0] else { panic!("an Insert: {seen:?}") };
    assert_eq!(*placement, CHAIR);
    assert!(Arc::ptr_eq(def, &app.world().resource::<FurnitureDefs>().0[&RecordId::new("furn_chair_01")]));
    assert_eq!(*object, Frame { translation: pos.as_vec3(), rotation: rot });
    assert!(disabled.is_empty());
    assert!(app.world().resource::<InteractionDefs>().get(CHAIR).is_some(), "the shell holds the def");
    assert_eq!(app.world().resource::<InteractionFrames>().current(CHAIR), Some(&Frame { translation: pos.as_vec3(), rotation: rot }));

    residency(&mut app, &chair, false);
    tick(&mut app, 1);
    let seen = &app.world().resource::<Seen>().index;
    assert_eq!(seen.len(), 2);
    assert_eq!(seen[1], IndexChange::unload(CHAIR));
    assert!(app.world().resource::<InteractionDefs>().get(CHAIR).is_none());
}

#[test]
fn a_placement_without_a_def_is_not_indexed() {
    let mut app = app();
    let rock = placement(ROCK, "rock_01", DVec3::ZERO, Quat::IDENTITY);
    residency(&mut app, &rock, true);
    residency(&mut app, &rock, false);
    tick(&mut app, 1);
    assert!(app.world().resource::<Seen>().index.is_empty());
}

#[test]
fn sit_switch_off_indexes_no_furniture_and_toggling_follows_live() {
    let mut app = app();
    set_sit(&mut app, false);
    app.update();
    let chair = placement(CHAIR, "furn_chair_01", DVec3::ZERO, Quat::IDENTITY);
    residency(&mut app, &chair, true);
    tick(&mut app, 1);
    assert!(app.world().resource::<Seen>().index.is_empty(), "Classic: no Insert");
    // On again: the resident chair comes in; off: it leaves.
    set_sit(&mut app, true);
    app.update();
    tick(&mut app, 1);
    assert!(matches!(app.world().resource::<Seen>().index.as_slice(), [IndexChange::Insert { placement: CHAIR, .. }]));
    set_sit(&mut app, false);
    app.update();
    tick(&mut app, 1);
    let seen = &app.world().resource::<Seen>().index;
    assert!(matches!(seen.as_slice(), [_, IndexChange::Remove { placement: CHAIR, .. }]), "{seen:?}");
    // Leaving R0 while off writes nothing more.
    residency(&mut app, &chair, false);
    tick(&mut app, 1);
    assert_eq!(app.world().resource::<Seen>().index.len(), 2);
}

fn aim(app: &mut App, target: PlacementId) {
    app.world_mut().resource_mut::<Focus>().hit = Some(FocusHit { handle: Handle(target.0), distance: 1.0 });
}

fn press(app: &mut App, target: PlacementId) {
    app.world_mut().resource_mut::<UsePresses>().0.push_back(Handle(target.0));
}

#[test]
fn one_press_gives_exactly_one_command_across_zero_one_and_two_ticks_per_frame() {
    let mut app = app();
    aim(&mut app, DOOR);
    press(&mut app, DOOR);
    // A frame with no fixed tick: the press waits.
    tick(&mut app, 0);
    assert!(app.world().resource::<Seen>().commands.is_empty());
    // A frame with two: one command, the second tick none.
    tick(&mut app, 2);
    assert_eq!(app.world().resource::<Seen>().commands.len(), 1);
    // The next press, a frame with one tick: one more.
    press(&mut app, DOOR);
    tick(&mut app, 1);
    tick(&mut app, 0);
    tick(&mut app, 2);
    let commands = &app.world().resource::<Seen>().commands;
    assert_eq!(commands.len(), 2, "{commands:?}");
    assert!(app.world().resource::<UsePresses>().0.is_empty());
}

#[test]
fn a_def_less_target_activates_as_before_and_furniture_approves_its_seat_slot() {
    let mut app = app();
    residency(&mut app, &placement(CHAIR, "furn_chair_01", DVec3::ZERO, Quat::IDENTITY), true);
    tick(&mut app, 1);

    // The door: no def, the primary use, an outcome exactly as the direct write gave.
    aim(&mut app, DOOR);
    press(&mut app, DOOR);
    tick(&mut app, 1);
    app.update();
    let door = Activated { target: Handle(DOOR.0), actor: PLAYER, verb: None };
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.commands.last().map(|c| (c.target, c.verb, c.slot)), Some((DOOR, None, None)));
    assert_eq!(seen.outcomes, vec![ActivationOutcome { activation: door, handled: false }]);

    // The chair: its sit verb through slot 0 (what the seat default claims).
    aim(&mut app, CHAIR);
    press(&mut app, CHAIR);
    tick(&mut app, 1);
    app.update();
    let seen = app.world().resource::<Seen>();
    let chair = Activated { target: Handle(CHAIR.0), actor: PLAYER, verb: Some(sit().0) };
    assert_eq!(seen.outcomes.last(), Some(&ActivationOutcome { activation: chair, handled: false }));
    assert_eq!(seen.approved.last(), Some(&UseApproved { activation: chair, verb: Some(sit()), slot: Some(0) }));
}

#[test]
fn furniture_prompts_its_verb_only() {
    let mut app = app();
    residency(&mut app, &placement(CHAIR, "furn_chair_01", DVec3::ZERO, Quat::IDENTITY), true);
    tick(&mut app, 1);
    aim(&mut app, CHAIR);
    tick(&mut app, 1);
    app.update();
    let focus = app.world().resource::<Focus>();
    let target = focus.target.as_ref().expect("the chair prompts");
    assert_eq!((target.handle, target.record.as_str(), target.name.as_str(), target.verb), (Handle(CHAIR.0), "furn_chair_01", "", Some("Sit")));
    assert_eq!(furniture_prompt(TagId::of("lie")), Some("Lie down"));
    assert_eq!(furniture_prompt(TagId::of("lean")), Some("Lean"));
    assert_eq!(furniture_prompt(TagId::of("browse")), None, "not built");
}

#[test]
fn a_defd_container_opens_first_with_activatables_prompt() {
    let mut app = app();
    residency(&mut app, &placement(CHEST, "com_chest_seat", DVec3::ZERO, Quat::IDENTITY), true);
    tick(&mut app, 1);
    aim(&mut app, CHEST);
    // What `activatable` shows for it (name + the class's verb).
    let shown = FocusTarget { handle: Handle(CHEST.0), record: "com_chest_seat".into(), name: "Chest".into(), verb: Some("Open"), distance: 1.0 };
    app.world_mut().resource_mut::<Focus>().target = Some(shown);
    tick(&mut app, 1);
    app.update();
    let target = app.world().resource::<Focus>().target.clone().expect("the class's prompt stays");
    assert_eq!((target.handle, target.record.as_str(), target.name.as_str(), target.verb), (Handle(CHEST.0), "com_chest_seat", "Chest", Some("Open")));

    press(&mut app, CHEST);
    tick(&mut app, 1);
    app.update();
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.commands.last().map(|c| (c.target, c.verb, c.slot)), Some((CHEST, Some(open()), None)));
    let opened = Activated { target: Handle(CHEST.0), actor: PLAYER, verb: Some(open().0) };
    // Unhandled: the container default (inventory) still reads it, as before the def.
    assert_eq!(seen.outcomes.last(), Some(&ActivationOutcome { activation: opened, handled: false }));
    assert_eq!(seen.approved.last(), Some(&UseApproved { activation: opened, verb: Some(open()), slot: None }));
}

/// A resident swing door (`tests_door`'s), at rest open when `open`, aimed at, `Focus::target`
/// what `activatable` shows for every door ("Door", "Open"); one tick for the rest and the
/// state tag, one for the focus.
fn aimed_swing_door(open: bool) -> (App, FocusTarget) {
    use crate::tests_door::{DOOR as SWING, door, residency as door_residency, spawn};
    let mut app = app();
    let hook = crate::door::DoorHook(Arc::new(Nothing));
    let p = door();
    spawn(&mut app, &hook, &p);
    door_residency(&mut app, &hook, &p, true);
    if open {
        app.world_mut().write_message(sf_interaction_bevy::DoorRest { placement: SWING, open: true, sign: 1 });
    }
    tick(&mut app, 1);
    aim(&mut app, SWING);
    let shown = FocusTarget { handle: Handle(SWING.0), record: "in_door_01".into(), name: "Door".into(), verb: Some("Open"), distance: 1.0 };
    app.world_mut().resource_mut::<Focus>().target = Some(shown.clone());
    tick(&mut app, 1);
    (app, shown)
}

/// A prompt's fields, comparable (`FocusTarget` has no `PartialEq`).
fn fields(t: &FocusTarget) -> (Handle, &str, &str, Option<&'static str>, f32) {
    (t.handle, t.record.as_str(), t.name.as_str(), t.verb, t.distance)
}

#[test]
fn an_open_swing_door_prompts_its_close_row() {
    let (mut app, shown) = aimed_swing_door(true);
    app.update();
    let target = app.world().resource::<Focus>().target.clone().expect("the door keeps its prompt");
    assert_eq!(fields(&target), fields(&FocusTarget { verb: Some("Close"), ..shown }));
}

#[test]
fn a_closed_swing_door_keeps_activatables_open_and_focus_unwritten() {
    let (mut app, shown) = aimed_swing_door(false);
    let before = app.world().resource_ref::<Focus>().last_changed();
    app.update();
    let focus = app.world().resource_ref::<Focus>();
    assert_eq!(focus.target.as_ref().map(fields), Some(fields(&shown)));
    assert_eq!(focus.last_changed(), before, "prompt left Focus unchanged");
}

fn scratch_store(name: &str) -> (Store, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("mw_interaction_bevy_{name}_{}", std::process::id()));
    (Store::open(&dir).expect("temp store"), dir)
}

#[test]
fn defs_product_missing_or_unreadable_gives_no_defs_and_a_good_one_loads() {
    let (store, dir) = scratch_store("defs");
    assert!(load_defs(&store, &Manifest::default()).0.is_empty(), "missing: no defs, no panic");
    let mut manifest = Manifest::default();
    let junk = store.put(b"not a product", false).expect("put");
    manifest.products.insert(ProductKey::new(INTERACT_DEFS, SUBJECT), junk);
    assert!(load_defs(&store, &manifest).0.is_empty(), "unreadable: no defs, no panic");
    let product = InteractDefs { defs: vec![(*chair_def()).clone()], ..InteractDefs::default() };
    let good = store.put(&postcard::to_stdvec(&product).expect("encode"), false).expect("put");
    manifest.products.insert(ProductKey::new(INTERACT_DEFS, SUBJECT), good);
    let loaded = load_defs(&store, &manifest);
    assert_eq!(loaded.0.len(), 1);
    assert_eq!(loaded.get(&RecordId::new("FURN_Chair_01")).map(|d| d.slots.len()), Some(1));
    let _ = std::fs::remove_dir_all(dir);
}
