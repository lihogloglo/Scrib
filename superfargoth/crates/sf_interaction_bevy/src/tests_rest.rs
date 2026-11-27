//! `IN3a` headless tests: the rest hook through the whole activation pipeline (`ActivateCommand` →
//! scripts → gates → `UseApproved` → [`RestRequested`]): one press on a bed writes exactly one
//! request, a handled (script) use, a gate's refusal, another verb or an actor with no
//! [`InteractionUser`] write none.

use std::collections::HashMap;

use sf_interaction::{
    ClipFamily, IkTargets, Occupancy, SlotDef, SlotKind, TagFilter, TagId, Tick, UserKey, VERB_SLEEP, VerbDef,
};
use sf_records::RecordId;
use sf_script_bevy::sf_script::{ContextConfig, Handle, ScriptRuntime, ScriptSource};
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

/// Handles every use of the scripted bed (its script owns the sleep).
const BED_SCRIPT: &str = r#"
local events = require("@sf/events")
local world = require("@sf/world")
events.on("activated", function(e)
    if world.record(e.target) == "scriptbed" then
        return "handled"
    end
    return nil
end)
"#;

const BED: PlacementId = PlacementId(20);
const SCRIPT_BED: PlacementId = PlacementId(21);
const GATED_BED: PlacementId = PlacementId(22);

fn lie() -> TagId {
    TagId::of("lie")
}

fn player() -> UserKey {
    UserKey::Player(sf_ids::PlayerKey(1))
}

fn handle(p: PlacementId) -> Handle {
    Handle(p.0)
}

/// Refuses every use of [`GATED_BED`] (an owned bed's gate).
fn owned_gate(mut pending: ResMut<PendingUses>) {
    for u in pending.uses_mut().iter_mut().filter(|u| u.activation.target == handle(GATED_BED)) {
        u.refuse(TagId::of("owned"));
    }
}

#[derive(Resource, Default)]
struct Requests(Vec<RestRequested>);

fn gather(mut requests: ResMut<Requests>, mut read: MessageReader<RestRequested>) {
    requests.0.extend(read.read().copied());
}

/// A bed: one `lie` pose slot (priority 0) and the slotless `sleep` above it.
fn bed(id: &str) -> Arc<InteractableDef> {
    let lie = SlotDef {
        kind: SlotKind::Pose { family: TagId::of("bed") },
        local: Frame::default(),
        band: 0.5,
        verb: VerbDef { verb: lie(), priority: 0, requires: TagFilter::default() },
        entrances: smallvec![],
        clips: ClipFamily("bed.lie".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    Arc::new(InteractableDef {
        id: RecordId::new(id),
        slots: vec![lie],
        verbs: smallvec![VerbDef { verb: VERB_SLEEP, priority: 1, requires: TagFilter::default() }],
        material: None,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

/// The pipeline with three bed placements and, when `with_user`, the player's [`InteractionUser`].
fn app(with_user: bool) -> App {
    let source = MemSource(HashMap::from([("scripts/beds/main.luau".to_owned(), BED_SCRIPT.to_owned())]));
    let mut scripts = Scripts::new(ScriptRuntime::default());
    scripts.start(vec![ContextConfig::for_mod("beds")], Arc::new(source));
    let mut app = App::new();
    app.insert_resource(scripts)
        .add_plugins((TunablesPlugin, ScriptsPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Requests>()
        .add_systems(Update, owned_gate.in_set(ActivationSystems::Gates))
        .add_systems(Update, gather.after(RestSystems));
    let placed = |record: &str| Placed { record: record.into(), worldspace: "test".into(), position: [0.0; 3], radius: 1.0 };
    let beds = [(BED, "bed"), (SCRIPT_BED, "scriptbed"), (GATED_BED, "ownedbed")];
    let mut p = app.world_mut().resource_mut::<ScriptPlacements>();
    for (h, record) in beds {
        p.loaded.insert(handle(h), placed(record));
    }
    p.player = Some(placed("player"));
    app.update();
    if with_user {
        app.world_mut().spawn(InteractionUser { user: player(), actor: PLAYER, priority: 30, tags: SmallVec::new() });
    }
    let owner = app.world_mut().spawn_empty().id();
    for (placement, record) in beds {
        app.world_mut().write_message(IndexChange::Insert {
            placement,
            def: bed(record),
            object: Frame::default(),
            owner,
            disabled: SmallVec::new(),
        });
    }
    app.world_mut().run_schedule(FixedUpdate);
    app
}

/// One player press on `target` (`verb` `None` = the primary use), then one fixed tick and one frame.
fn press(app: &mut App, target: PlacementId, verb: Option<TagId>) {
    app.world_mut().write_message(ActivateCommand {
        user: player(),
        actor: PLAYER,
        target,
        verb,
        slot: None,
        priority: 30,
        tags: SmallVec::new(),
    });
    app.world_mut().run_schedule(FixedUpdate);
    app.update();
}

fn requests(app: &App) -> &[RestRequested] {
    &app.world().resource::<Requests>().0
}

#[test]
fn one_press_on_a_bed_writes_exactly_one_rest_request() {
    let mut app = app(true);
    press(&mut app, BED, None);
    let tick: Tick = app.world().resource::<InteractionTick>().0;
    let want = RestRequested { tick, user: player(), actor: PLAYER, placement: BED, slot: None };
    assert_eq!(requests(&app), [want]);
    // Later frames write nothing more; the named verb is the same use.
    app.update();
    app.world_mut().run_schedule(FixedUpdate);
    app.update();
    assert_eq!(requests(&app).len(), 1);
    press(&mut app, BED, Some(VERB_SLEEP));
    assert_eq!(requests(&app).len(), 2);
    assert_eq!(requests(&app)[1].slot, None);
}

#[test]
fn a_handled_use_writes_no_rest_request() {
    let mut app = app(true);
    press(&mut app, SCRIPT_BED, None);
    press(&mut app, SCRIPT_BED, Some(VERB_SLEEP));
    assert!(requests(&app).is_empty());
}

#[test]
fn a_gate_refusal_writes_no_rest_request() {
    let mut app = app(true);
    press(&mut app, GATED_BED, Some(VERB_SLEEP));
    assert!(requests(&app).is_empty());
}

#[test]
fn another_verb_on_the_bed_writes_no_rest_request() {
    let mut app = app(true);
    press(&mut app, BED, Some(lie()));
    assert!(requests(&app).is_empty());
}

#[test]
fn an_actor_with_no_interaction_user_writes_no_rest_request() {
    let mut app = app(false);
    press(&mut app, BED, None);
    assert!(requests(&app).is_empty());
}
