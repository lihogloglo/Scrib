//! `IN19b` headless tests: the light shell through the whole activation pipeline
//! (`ActivateCommand` → scripts → gates → `UseApproved` → [`LightChanged`]): snuff and relight
//! each write one event and keep the registry, the `Lit` tag and [`GameplayLights`]; a snuff on an
//! unlit light is refused (by the `requires` gate for a player, by the core for a script); a
//! removal or a re-insert without a light slot drops the entry; a rest is no use; a move follows.

use std::collections::HashMap;

use glam::{Quat, Vec3};
use sf_interaction::{
    ClipFamily, IkTargets, LIT, LightDef, Occupancy, SlotDef, SlotKind, TagFilter, TagId, UserKey, VERB_RELIGHT,
    VERB_SNUFF, VerbDef,
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

/// Answers nothing: every use reaches the default.
const NO_SCRIPT: &str = r#"
local events = require("@sf/events")
events.on("activated", function(e)
    return nil
end)
"#;

/// Lit when first loaded.
const LAMP: PlacementId = PlacementId(30);
/// Unlit when first loaded.
const CANDLE: PlacementId = PlacementId(31);

/// A script actor with no [`InteractionUser`].
const SCRIPT_ACTOR: Handle = Handle(99);

/// The flame, in the object's frame.
const FLAME: Vec3 = Vec3::new(0.0, 0.5, 0.0);

/// The objects' world frame: 2 m up +X, a quarter turn about +Y.
fn object() -> Frame {
    Frame { translation: Vec3::new(2.0, 0.0, 0.0), rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2) }
}

fn player() -> UserKey {
    UserKey::Player(sf_ids::PlayerKey(1))
}

fn handle(p: PlacementId) -> Handle {
    Handle(p.0)
}

#[derive(Resource, Default)]
struct Changes(Vec<sf_interaction::LightChanged>);

fn gather(mut changes: ResMut<Changes>, mut read: MessageReader<LightChanged>) {
    changes.0.extend(read.read().map(|c| c.0));
}

fn light_def(lit_default: bool) -> LightDef {
    LightDef { radius_m: 4.5, intensity: 0.8, lit_default }
}

/// A light: its slot's verb is snuff (needs `Lit`), the slotless relight needs no `Lit`.
fn lamp(id: &str, lit_default: bool) -> Arc<InteractableDef> {
    let slot = SlotDef {
        kind: SlotKind::Light(light_def(lit_default)),
        local: Frame { translation: FLAME, rotation: Quat::IDENTITY },
        band: 0.5,
        verb: VerbDef { verb: VERB_SNUFF, priority: 1, requires: TagFilter { all: smallvec![LIT], ..Default::default() } },
        entrances: smallvec![],
        clips: ClipFamily::default(),
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
        slots: vec![slot],
        verbs: smallvec![VerbDef {
            verb: VERB_RELIGHT,
            priority: 1,
            requires: TagFilter { none: smallvec![LIT], ..Default::default() },
        }],
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

/// The same object with no light slot (a re-insert after it became something else).
fn plain(id: &str) -> Arc<InteractableDef> {
    Arc::new(InteractableDef {
        id: RecordId::new(id),
        slots: Vec::new(),
        verbs: smallvec![VerbDef { verb: TagId::of("take"), priority: 0, requires: TagFilter::default() }],
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

fn insert(app: &mut App, placement: PlacementId, def: Arc<InteractableDef>, object: Frame) {
    let owner = app.world_mut().spawn_empty().id();
    app.world_mut().write_message(IndexChange::Insert { placement, def, object, owner, disabled: SmallVec::new() });
    app.world_mut().run_schedule(FixedUpdate);
}

/// The pipeline with [`LAMP`] and [`CANDLE`] indexed at [`object`] and the player's user.
fn app() -> App {
    let source = MemSource(HashMap::from([("scripts/lights/main.luau".to_owned(), NO_SCRIPT.to_owned())]));
    let mut scripts = Scripts::new(ScriptRuntime::default());
    scripts.start(vec![ContextConfig::for_mod("lights")], Arc::new(source));
    let mut app = App::new();
    app.insert_resource(scripts)
        .add_plugins((TunablesPlugin, ScriptsPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Changes>()
        .add_systems(Update, gather.after(LightSystems));
    let placed = |record: &str| Placed { record: record.into(), worldspace: "test".into(), position: [0.0; 3], radius: 1.0 };
    let mut p = app.world_mut().resource_mut::<ScriptPlacements>();
    p.loaded.insert(handle(LAMP), placed("lamp"));
    p.loaded.insert(handle(CANDLE), placed("candle"));
    p.player = Some(placed("player"));
    app.update();
    app.world_mut().spawn(InteractionUser { user: player(), actor: PLAYER, priority: 30, tags: SmallVec::new() });
    insert(&mut app, LAMP, lamp("lamp", true), object());
    insert(&mut app, CANDLE, lamp("candle", false), object());
    app
}

/// One player press on `target` with `verb`, then one fixed tick and one frame.
fn press(app: &mut App, target: PlacementId, verb: TagId) {
    app.world_mut().write_message(ActivateCommand {
        user: player(),
        actor: PLAYER,
        target,
        verb: Some(verb),
        slot: None,
        priority: 30,
        tags: SmallVec::new(),
    });
    app.world_mut().run_schedule(FixedUpdate);
    app.update();
}

/// One script `Activate` on `target` by an actor with no user, then one frame.
fn script(app: &mut App, target: PlacementId, verb: Option<TagId>) {
    app.world_mut().write_message(ScriptActivate { target: handle(target), actor: SCRIPT_ACTOR, verb });
    app.update();
}

fn changes(app: &App) -> &[sf_interaction::LightChanged] {
    &app.world().resource::<Changes>().0
}

fn lit(app: &App, placement: PlacementId) -> bool {
    app.world().resource::<Lights>().get(placement).expect("registered").state.lit
}

fn tagged(app: &App, placement: PlacementId) -> bool {
    app.world().resource::<InteractionStateTags>().tags(placement).contains(&LIT)
}

fn gameplay(app: &App, placement: PlacementId) -> Option<GameplayLight> {
    app.world().resource::<GameplayLights>().get(placement).copied()
}

fn tick(app: &App) -> Tick {
    app.world().resource::<InteractionTick>().0
}

#[test]
fn registration_starts_from_lit_default_and_fills_the_ai_lights() {
    let app = app();
    let flame = object().translation + object().rotation * FLAME;
    assert_eq!(app.world().resource::<Lights>().len(), 2);
    assert_eq!(
        gameplay(&app, LAMP),
        Some(GameplayLight { placement: LAMP, pos: flame, radius_m: 4.5, intensity: 0.8, lit: true })
    );
    assert_eq!(gameplay(&app, CANDLE).map(|g| g.lit), Some(false));
    assert!(tagged(&app, LAMP) && !tagged(&app, CANDLE));
    let order: Vec<_> = app.world().resource::<GameplayLights>().iter().map(|g| g.placement).collect();
    assert_eq!(order, [LAMP, CANDLE]);
    assert!(changes(&app).is_empty(), "registration is no use");
}

#[test]
fn snuff_then_relight_write_one_event_each_and_keep_the_state() {
    let mut app = app();
    let at = object().translation + object().rotation * FLAME;
    press(&mut app, LAMP, VERB_SNUFF);
    let snuffed = tick(&app);
    assert_eq!(
        changes(&app),
        [sf_interaction::LightChanged { tick: snuffed, placement: LAMP, lit: false, at, culprit: Some(player()) }]
    );
    assert!(!lit(&app, LAMP) && !tagged(&app, LAMP));
    assert_eq!(gameplay(&app, LAMP).map(|g| g.lit), Some(false));
    // Later frames write nothing more.
    app.world_mut().run_schedule(FixedUpdate);
    app.update();
    assert_eq!(changes(&app).len(), 1);

    press(&mut app, LAMP, VERB_RELIGHT);
    let relit = tick(&app);
    assert_eq!(changes(&app).len(), 2);
    assert_eq!(
        changes(&app)[1],
        sf_interaction::LightChanged { tick: relit, placement: LAMP, lit: true, at, culprit: Some(player()) }
    );
    assert!(lit(&app, LAMP) && tagged(&app, LAMP));
    assert_eq!(gameplay(&app, LAMP).map(|g| g.lit), Some(true));
}

#[test]
fn snuff_on_an_unlit_light_is_refused() {
    let mut app = app();
    // The player's snuff fails the `requires` gate in validation.
    press(&mut app, CANDLE, VERB_SNUFF);
    // A script's snuff passes no `requires` gate; the core refuses it.
    script(&mut app, CANDLE, Some(VERB_SNUFF));
    assert!(changes(&app).is_empty());
    assert!(!lit(&app, CANDLE) && !tagged(&app, CANDLE));
    // A relight on the lit lamp is refused the same way.
    press(&mut app, LAMP, VERB_RELIGHT);
    script(&mut app, LAMP, Some(VERB_RELIGHT));
    assert!(changes(&app).is_empty());
    assert!(lit(&app, LAMP));
}

#[test]
fn a_script_use_without_a_verb_toggles_with_no_culprit_and_other_verbs_do_nothing() {
    let mut app = app();
    script(&mut app, CANDLE, Some(TagId::of("take")));
    assert!(changes(&app).is_empty());
    script(&mut app, CANDLE, None);
    assert_eq!(changes(&app).len(), 1);
    assert_eq!((changes(&app)[0].lit, changes(&app)[0].culprit), (true, None));
    assert!(lit(&app, CANDLE) && tagged(&app, CANDLE));
}

#[test]
fn unregister_drops_the_entry_its_tag_and_its_ai_light() {
    let mut app = app();
    app.world_mut().write_message(IndexChange::unload(LAMP));
    app.world_mut().run_schedule(FixedUpdate);
    assert!(app.world().resource::<Lights>().get(LAMP).is_none());
    assert_eq!(gameplay(&app, LAMP), None);
    assert!(!tagged(&app, LAMP));
    // A re-insert without a light slot drops it too.
    press(&mut app, CANDLE, VERB_RELIGHT);
    assert!(tagged(&app, CANDLE));
    insert(&mut app, CANDLE, plain("candle"), object());
    assert!(app.world().resource::<Lights>().is_empty());
    assert!(app.world().resource::<GameplayLights>().is_empty());
    assert!(!tagged(&app, CANDLE));
}

#[test]
fn rest_sets_the_state_without_an_event_reinsert_keeps_it_and_a_move_follows() {
    let mut app = app();
    app.world_mut().write_message(LightRest { placement: LAMP, lit: false });
    app.world_mut().write_message(LightRest { placement: PlacementId(777), lit: true });
    app.world_mut().run_schedule(FixedUpdate);
    assert!(!lit(&app, LAMP) && !tagged(&app, LAMP));
    assert_eq!(app.world().resource::<Lights>().len(), 2, "a rest for no light is dropped");
    // A re-insert keeps the restored state (not `lit_default`).
    insert(&mut app, LAMP, lamp("lamp", true), object());
    assert!(!lit(&app, LAMP));
    // A move takes the flame along; the state stays.
    let moved = Frame { translation: Vec3::new(0.0, 0.0, -3.0), rotation: Quat::IDENTITY };
    app.world_mut().write_message(IndexChange::Move { placement: LAMP, object: moved });
    app.world_mut().run_schedule(FixedUpdate);
    let g = gameplay(&app, LAMP).expect("still registered");
    assert_eq!((g.pos, g.lit), (Vec3::new(0.0, 0.5, -3.0), false));
    app.update();
    assert!(changes(&app).is_empty());
}
