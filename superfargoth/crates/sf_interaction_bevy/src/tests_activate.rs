//! `IN31b1` headless tests: the activation pipeline `ActivateCommand` → `Activated` → scripts →
//! gates → `UseApproved` / `UseRefused` (interaction.md "Activation contract").

use std::collections::HashMap;

use sf_ids::ActorKey;
use sf_interaction::{
    ClipFamily, IkTargets, Occupancy, SlotDef, SlotKind, SlotTerms, TagFilter, TagId, UserKey, VerbDef,
};
use sf_records::RecordId;
use sf_script_bevy::sf_script::{ContextConfig, Handle, ScriptRuntime, ScriptSource, StateValue, storage_value};
use sf_script_bevy::{PLAYER, Placed, ScriptPlacements, Scripts, ScriptsPlugin};
use sf_tunables_bevy::TunablesPlugin;
use smallvec::{SmallVec, smallvec};

use super::*;

struct MemSource(HashMap<String, String>);

impl ScriptSource for MemSource {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.0.get(path).map(|s| s.as_bytes().to_vec())
    }
}

/// Logs every activation it sees; handles the chest.
const WATCHER: &str = r#"
local core = require("@sf/core")
local events = require("@sf/events")
local world = require("@sf/world")
local s = core.storage()
s.seen = s.seen or ""
events.on("activated", function(e)
    s.seen ..= world.record(e.target) .. ";"
    if world.record(e.target) == "chest" then
        return "handled"
    end
    return nil
end)
"#;

const CHAIR: PlacementId = PlacementId(7);
const LAMP: PlacementId = PlacementId(8);
const CHEST: PlacementId = PlacementId(9);
const DOOR: PlacementId = PlacementId(10);

fn sit() -> TagId {
    TagId::of("sit")
}

fn snuff() -> TagId {
    TagId::of("snuff")
}

fn lit() -> TagId {
    TagId::of("lit")
}

fn locked() -> TagId {
    TagId::of("locked")
}

fn handle(p: PlacementId) -> Handle {
    Handle(p.0)
}

fn actor(n: u128) -> UserKey {
    UserKey::Actor(ActorKey(n))
}

/// Targets the test gates refuse: the first gives `locked`, the second (after it) `trapped`.
#[derive(Resource, Default)]
struct Gated(Vec<Handle>);

#[derive(Resource, Default)]
struct Seen {
    activated: Vec<Activated>,
    approved: Vec<UseApproved>,
    refused: Vec<UseRefused>,
}

fn lock_gate(gated: Res<Gated>, mut pending: ResMut<PendingUses>) {
    for u in pending.uses_mut().iter_mut().filter(|u| gated.0.contains(&u.activation.target)) {
        u.refuse(locked());
    }
}

fn trap_gate(gated: Res<Gated>, mut pending: ResMut<PendingUses>) {
    for u in pending.uses_mut().iter_mut().filter(|u| gated.0.contains(&u.activation.target)) {
        u.refuse(TagId::of("trapped"));
    }
}

fn gather(
    mut seen: ResMut<Seen>,
    mut activated: MessageReader<Activated>,
    mut approved: MessageReader<UseApproved>,
    mut refused: MessageReader<UseRefused>,
) {
    seen.activated.extend(activated.read().copied());
    seen.approved.extend(approved.read().copied());
    seen.refused.extend(refused.read().copied());
}

fn slot(verb: TagId) -> SlotDef {
    SlotDef {
        kind: SlotKind::Pose { family: TagId::of("seat") },
        local: Frame::default(),
        band: 0.45,
        verb: VerbDef { verb, priority: 0, requires: TagFilter::default() },
        entrances: smallvec![],
        clips: ClipFamily("seat.chair".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    }
}

fn def(id: &str, slots: Vec<SlotDef>, verbs: SmallVec<[VerbDef; 2]>) -> Arc<InteractableDef> {
    Arc::new(InteractableDef {
        id: RecordId::new(id),
        slots,
        verbs,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

fn chair() -> Arc<InteractableDef> {
    def("chair", vec![slot(sit())], SmallVec::new())
}

/// A slotless `snuff` that needs the `lit` state tag.
fn lamp() -> Arc<InteractableDef> {
    let requires = TagFilter { all: smallvec![lit()], ..TagFilter::default() };
    def("lamp", Vec::new(), smallvec![VerbDef { verb: snuff(), priority: 0, requires }])
}

fn app() -> App {
    let source = MemSource(HashMap::from([("scripts/watcher/main.luau".to_owned(), WATCHER.to_owned())]));
    let mut scripts = Scripts::new(ScriptRuntime::default());
    scripts.start(vec![ContextConfig::for_mod("watcher")], Arc::new(source));
    let mut app = App::new();
    app.insert_resource(scripts)
        .add_plugins((TunablesPlugin, ScriptsPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Gated>()
        .init_resource::<Seen>()
        .add_systems(Update, (lock_gate, trap_gate.after(lock_gate)).in_set(ActivationSystems::Gates))
        .add_systems(Update, gather.after(ActivationSystems::Default));
    let placed = |record: &str| Placed { record: record.into(), worldspace: "test".into(), position: [0.0; 3], radius: 0.5 };
    let mut p = app.world_mut().resource_mut::<ScriptPlacements>();
    for (h, record) in [(CHAIR, "chair"), (LAMP, "lamp"), (CHEST, "chest"), (DOOR, "door")] {
        p.loaded.insert(handle(h), placed(record));
    }
    p.player = Some(placed("player"));
    app.update();
    let owner = app.world_mut().spawn_empty().id();
    for (placement, def) in [(CHAIR, chair()), (LAMP, lamp())] {
        app.world_mut().write_message(IndexChange::Insert {
            placement,
            def,
            object: Frame::default(),
            owner,
            disabled: SmallVec::new(),
        });
    }
    app.world_mut().run_schedule(FixedUpdate);
    app
}

/// A player command (priority 30, no tags).
fn cmd(target: PlacementId, verb: Option<TagId>) -> ActivateCommand {
    ActivateCommand {
        user: UserKey::Player(sf_ids::PlayerKey(1)),
        actor: PLAYER,
        target,
        verb,
        slot: None,
        priority: 30,
        tags: SmallVec::new(),
    }
}

/// Sends `commands`, runs one fixed tick (validation), then one frame (scripts, gates, default).
fn send(app: &mut App, commands: Vec<ActivateCommand>) {
    for c in commands {
        app.world_mut().write_message(c);
    }
    app.world_mut().run_schedule(FixedUpdate);
    app.update();
}

fn seen_by_scripts(app: &App) -> String {
    let ctx = app.world().resource::<Scripts>().runtime.context("watcher").unwrap();
    match storage_value(ctx, "seen") {
        Some(StateValue::Str(b)) => String::from_utf8_lossy(&b).into_owned(),
        _ => String::new(),
    }
}

#[test]
fn a_use_handled_by_a_script_is_not_approved() {
    let mut app = app();
    send(&mut app, vec![cmd(CHEST, None)]);
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.activated.len(), 1);
    assert!(seen.approved.is_empty());
    assert!(seen.refused.is_empty());
    assert_eq!(seen_by_scripts(&app), "chest;");
}

#[test]
fn an_unhandled_use_is_approved_once_with_its_verb() {
    let mut app = app();
    send(&mut app, vec![cmd(CHAIR, Some(sit()))]);
    let a = Activated { target: handle(CHAIR), actor: PLAYER, verb: Some(sit().0) };
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.activated, vec![a]);
    assert_eq!(seen.approved, vec![UseApproved { activation: a, verb: Some(sit()), slot: Some(0) }]);
    assert!(seen.refused.is_empty());
    // The primary use of a target with a def names the best offered verb.
    send(&mut app, vec![cmd(CHAIR, None)]);
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.approved.len(), 2);
    assert_eq!(seen.approved[1].verb, Some(sit()));
    assert_eq!(seen_by_scripts(&app), "chair;chair;");
}

#[test]
fn a_gate_refusal_reports_its_reason_and_approves_nothing() {
    let mut app = app();
    app.world_mut().resource_mut::<Gated>().0.push(handle(DOOR));
    send(&mut app, vec![cmd(DOOR, None), cmd(CHAIR, Some(sit()))]);
    let door = Activated { target: handle(DOOR), actor: PLAYER, verb: None };
    let seen = app.world().resource::<Seen>();
    // The first gate's reason wins; the chair passes.
    assert_eq!(seen.refused, vec![UseRefused { activation: door, verb: None, reason: locked() }]);
    assert_eq!(seen.approved.len(), 1);
    assert_eq!(seen.approved[0].activation.target, handle(CHAIR));
}

#[test]
fn a_script_activate_passes_gates_and_default_once_and_skips_the_scripts() {
    let mut app = app();
    app.world_mut().resource_mut::<Gated>().0.push(handle(DOOR));
    app.world_mut().write_message(ScriptActivate { target: handle(CHEST), actor: PLAYER, verb: None });
    app.world_mut().write_message(ScriptActivate { target: handle(DOOR), actor: PLAYER, verb: Some(TagId::of("open")) });
    app.update();
    app.update();
    let seen = app.world().resource::<Seen>();
    assert!(seen.activated.is_empty());
    let chest = Activated { target: handle(CHEST), actor: PLAYER, verb: None };
    assert_eq!(seen.approved, vec![UseApproved { activation: chest, verb: None, slot: None }]);
    assert_eq!(seen.refused.len(), 1);
    assert_eq!(seen.refused[0].reason, locked());
    assert_eq!(seen.refused[0].verb, Some(TagId::of("open")));
    // The chest's script would have handled it: scripts never saw either.
    assert_eq!(seen_by_scripts(&app), "");
}

#[test]
fn a_verb_not_offered_writes_nothing() {
    let mut app = app();
    // Actor 1 claims and occupies the chair.
    let terms = SlotTerms::of(&chair().slots[0], &InteractParams::default());
    let seat = SlotRef { placement: CHAIR, slot: 0 };
    for op in [Op::Claim { slot: seat, terms, tags: SmallVec::new() }, Op::Occupy { slot: seat }] {
        app.world_mut().write_message(ClaimRequest(Request { user: actor(1), priority: 20, op }));
        app.world_mut().run_schedule(FixedUpdate);
    }
    let other = ActivateCommand { user: actor(2), actor: Handle(2), ..cmd(CHAIR, Some(sit())) };
    let wrong_slot = ActivateCommand { slot: Some(1), ..cmd(CHAIR, Some(sit())) };
    send(
        &mut app,
        vec![
            cmd(CHAIR, Some(sit())),       // taken: Occupied by actor 1
            other,                         // same
            wrong_slot,                    // no slot 1
            cmd(LAMP, Some(snuff())),      // `requires: lit` unmet
            cmd(LAMP, Some(sit())),        // not a lamp verb
            cmd(CHEST, Some(sit())),       // no def: only the primary use
        ],
    );
    let seen = app.world().resource::<Seen>();
    assert!(seen.activated.is_empty(), "{:?}", seen.activated);
    assert!(seen.approved.is_empty() && seen.refused.is_empty());
    assert_eq!(seen_by_scripts(&app), "");

    // The holder's own slot is offered to it; a lit lamp offers snuff.
    app.world_mut().resource_mut::<InteractionStateTags>().insert(LAMP, lit());
    let holder = ActivateCommand { user: actor(1), actor: Handle(1), ..cmd(CHAIR, Some(sit())) };
    send(&mut app, vec![holder, cmd(LAMP, Some(snuff()))]);
    let seen = app.world().resource::<Seen>();
    let targets: Vec<Handle> = seen.approved.iter().map(|u| u.activation.target).collect();
    // Sorted by actor then target: the player's handle (0) before actor 1's.
    assert_eq!(targets, vec![handle(LAMP), handle(CHAIR)]);

    // A removed placement loses its def: its verbs are refused again.
    app.world_mut().write_message(IndexChange::unload(LAMP));
    send(&mut app, vec![cmd(LAMP, Some(snuff()))]);
    assert_eq!(app.world().resource::<Seen>().approved.len(), 2);
}

#[test]
fn a_target_without_a_def_is_activated_for_its_primary_use() {
    let mut app = app();
    send(&mut app, vec![cmd(DOOR, None)]);
    let a = Activated { target: handle(DOOR), actor: PLAYER, verb: None };
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.activated, vec![a]);
    assert_eq!(seen.approved, vec![UseApproved { activation: a, verb: None, slot: None }]);
    assert_eq!(seen_by_scripts(&app), "door;");
}
