//! S2 tests (save.md "Tests"): the canonical round trip in a synthetic test level, quick and
//! auto saves, the gate, corruption, dormancy, migration and background jobs. Shapes, the golden
//! save and budgets are in `tests_formats.rs` and `tests_budget.rs`.
// Exact: saved floats round-trip bit for bit.
#![allow(clippy::float_cmp)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use bevy_time::{Time, Virtual};
use glam::{IVec3, Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_action::{ActionState, RuleKey};
use sf_controller::Locomotion;
use sf_coords::{WorldPos, WorldTransform};
use sf_level::Level;
use sf_records::{PatchOp, RecordId, Value};
use sf_rpg::{
    AttrValue, Attrs, CharacterRpg, EntryDraft, EquipPlacement, EquipRules, GameTime, HeardLine, Holder, ItemStack, PlayerId,
    PlayerKey, RpgCommand, SlotId, StageFlags, WorldRpg,
};
use sf_save::{FileKind, SaveFile, SaveReason, SaveWriter, SectionFlags, SectionKey, encode};
use sf_sky::{Calendar, SkyClock};
use sf_weather::WeatherState;
use sf_wind::{Wind, WindSource, WindState};
use sf_world::{Location, Placement, PlacementClass, PlacementFlags, PlacementId, PlacementOverlay, RecordRef, WorldspaceId};

use crate::sections::{self, BlobVersions, CharacterBody, CharacterSave, RecordOverlay, VersionedBlob, WeatherSave, order};
use crate::*;

// ---- the test game: resources a content pack would own ---------------------------------------

#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Sky(pub SkyClock);
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Weather(pub WeatherSave);
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Overlay(pub PlacementOverlay);
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Rpg(pub WorldRpg);
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Records(pub RecordOverlay);
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Party(pub Vec<CharacterSave>);
/// The adapter's state (`test/globals`), like `MWScript` globals.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(crate) struct Globals(pub BTreeMap<String, f64>);

pub(crate) const WS: &str = "save_yard";
pub(crate) const HERO: PlayerKey = PlayerKey(0x00c0_ffee);

pub(crate) fn globals_key() -> SectionKey {
    SectionKey::new("test", "globals").unwrap()
}

/// The level as baked content: its props and actors as placements, by name.
pub(crate) struct Yard {
    pub level: Level,
    pub placements: BTreeMap<String, Placement>,
}

pub(crate) fn yard() -> Yard {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/levels/save_yard.ron");
    let level = Level::parse(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert!(level.problems().is_empty(), "{:?}", level.problems());
    let mut placements = BTreeMap::new();
    let place = |name: &str, kind: &str, at: Vec3, yaw: f32, class| Placement {
        id: PlacementId::from_key(WS, name.as_bytes()),
        base: RecordRef::new(kind, name),
        model: None,
        transform: WorldTransform { pos: WorldPos::from_f64(at.as_dvec3()), rotation: Quat::from_rotation_y(yaw.to_radians()), scale: Vec3::ONE },
        class,
        flags: PlacementFlags::empty(),
    };
    for o in level.objects.iter().filter(|o| o.dynamic.is_some() || o.interact.is_some()) {
        let name = o.name.clone().unwrap();
        let class = if o.dynamic.is_some() { PlacementClass::Dynamic } else { PlacementClass::Static };
        placements.insert(name.clone(), place(&name, "prop", o.at, o.yaw, class));
    }
    for a in &level.actors {
        let spawn = level.spawn(&a.spawn).unwrap();
        placements.insert(a.id.clone(), place(&a.id, "npc", spawn.at, spawn.yaw, PlacementClass::Actor));
    }
    Yard { level, placements }
}

/// A fresh character at the level's start.
pub(crate) fn new_character(y: &Yard) -> CharacterSave {
    let start = y.level.spawn("start").unwrap();
    let pos = WorldPos::from_f64(start.at.as_dvec3());
    let mut rpg = CharacterRpg::default();
    rpg.loadout.add(RecordId::new("gold"), 25, Attrs::default()).unwrap();
    CharacterSave {
        player: HERO,
        name: "Hero".into(),
        location: Location::new(WorldspaceId::new(WS), pos),
        facing: start.yaw.to_radians(),
        body: Some(CharacterBody { action: ActionState::new(RuleKey::of("locomotion")), locomotion: Locomotion::new(pos, start.yaw.to_radians()) }),
        rpg,
        ext: BTreeMap::new(),
    }
}

/// Every section the framework has today plus one adapter section.
pub(crate) fn wire(app: &mut App) {
    app.add_save_section(sections::clock(read(|s: &Sky| s.0), write(|s: &mut Sky, v| s.0 = v)))
        .add_save_section(sections::weather(read(|w: &Weather| w.0.clone()), write(|w: &mut Weather, v| w.0 = v)))
        .add_save_section(sections::records(read(|r: &Records| r.0.clone()), write(|r: &mut Records, v| r.0 = v)))
        .add_save_section(sections::overlay(read(|o: &Overlay| o.0.clone()), write(|o: &mut Overlay, v| o.0 = v)))
        .add_save_section(sections::rpg(read(|r: &Rpg| r.0.clone()), write(|r: &mut Rpg, v| r.0 = v)))
        .add_save_section(sections::characters(
            |w: &World| w.get_resource::<Party>().map(|p| p.0.clone()).unwrap_or_default(),
            |w: &mut World, v| {
                w.insert_resource(Party(v));
            },
        ))
        .add_save_section(
            SaveSection::value(globals_key(), 1, read(|g: &Globals| g.0.clone()), write(|g: &mut Globals, v| g.0 = v)).order(order::ADAPTER),
        );
}

/// A headless app over `root`, jobs inline unless `background`.
pub(crate) fn app(root: &Path, background: bool) -> App {
    let mut app = App::new();
    let config = SaveConfig { root: root.to_owned(), background, ..SaveConfig::default() };
    app.add_plugins(SfSavePlugin { config })
        .init_resource::<Sky>()
        .init_resource::<Weather>()
        .init_resource::<Overlay>()
        .init_resource::<Rpg>()
        .init_resource::<Records>()
        .init_resource::<Party>()
        .init_resource::<Globals>();
    app.world_mut().resource_mut::<Autosave>().interval = None;
    app
}

/// A new game in the yard: one character at the start.
pub(crate) fn new_game(root: &Path) -> (App, Yard) {
    let y = yard();
    let mut a = app(root, false);
    wire(&mut a);
    a.world_mut().insert_resource(Party(vec![new_character(&y)]));
    (a, y)
}

pub(crate) fn drain<M: Message>(app: &mut App) -> Vec<M> {
    app.world_mut().resource_mut::<Messages<M>>().drain().collect()
}

pub(crate) fn save(app: &mut App, req: SaveRequest) -> SaveReport {
    app.world_mut().write_message(req);
    app.update();
    let mut done = drain::<SaveDone>(app);
    assert_eq!(done.len(), 1, "{done:?}");
    done.pop().unwrap().0.unwrap()
}

pub(crate) fn load(app: &mut App, req: LoadRequest) -> Result<LoadReport, LoadError> {
    app.world_mut().write_message(req);
    app.update();
    let mut done = drain::<LoadDone>(app);
    assert_eq!(done.len(), 1, "{done:?}");
    done.pop().unwrap().0
}

struct OneHand(SlotId);
impl EquipRules for OneHand {
    fn placement(&self, stack: &ItemStack) -> Option<EquipPlacement> {
        (stack.item.as_str() == "sword").then(|| EquipPlacement { slots: [self.0].into_iter().collect(), whole_stack: false })
    }
}

#[derive(Serialize)]
struct Potion {
    name: String,
    magnitude: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ActorBlob {
    health: f32,
    dead: bool,
}

/// Gameplay: pick up, move, kill, spawn, quest, items, clock, weather, wind, records, globals.
pub(crate) fn play(app: &mut App, y: &Yard) {
    let w = app.world_mut();
    let id = |name: &str| y.placements[name].id;
    {
        let mut o = w.resource_mut::<Overlay>();
        o.0.remove(id("crate-a"));
        let to = Location::new(WorldspaceId::new(WS), WorldPos::new(IVec3::new(1, 0, -1), Vec3::new(3.5, 0.25, 250.0)));
        o.0.move_to(&y.placements["crate-b"], &to, Quat::from_rotation_y(0.7));
        o.0.set_state(id("guard"), "actor", encode(&ActorBlob { health: 0.0, dead: true }));
        o.0.set_state(id("chest"), "container", vec![1, 2, 3]);
        let mut potion = y.placements["crate-c"].clone();
        potion.base = RecordRef::new("item", "save:potion.0");
        potion.transform.pos = WorldPos::from_f64(glam::DVec3::new(0.5, 0.0, 5.0));
        o.0.spawn(WorldspaceId::new(WS), potion);
    }
    {
        let mut party = w.resource_mut::<Party>();
        let hero = &mut party.0[0];
        let holder = Holder::Player(PlayerId::HOST);
        hero.rpg.apply(RpgCommand::AddItem { holder, item: RecordId::new("crate"), count: 1, attrs: Attrs::default() }).unwrap();
        let worn = Attrs::new().with("condition", AttrValue::Int(70));
        hero.rpg.apply(RpgCommand::AddItem { holder, item: RecordId::new("sword"), count: 1, attrs: worn }).unwrap();
        let sword = hero.rpg.loadout.inventory().stacks().iter().find(|s| s.item.as_str() == "sword").unwrap().id;
        hero.rpg.loadout.equip(sword, &OneHand(SlotId(0))).unwrap();
        hero.location.pos = WorldPos::new(IVec3::new(0, 0, 0), Vec3::new(-1.5, 0.0, 2.0));
        hero.facing = 1.25;
        hero.ext.insert("test".into(), VersionedBlob { version: 2, bytes: vec![9, 9] });
    }
    {
        let mut rpg = w.resource_mut::<Rpg>();
        let draft = EntryDraft { source: Some(RecordId::new("info.find.10")), text: "I found the crate.".into(), time: GameTime { day: 1, minute: 600 } };
        rpg.0.apply(RpgCommand::SetStage { quest: RecordId::new("find_crate"), stage: 10, flags: StageFlags::default(), entry: Some(draft) }).unwrap();
        rpg.0.apply(RpgCommand::SetStage { quest: RecordId::new("guard_duty"), stage: 100, flags: StageFlags { finish: true, restart: false }, entry: None }).unwrap();
        rpg.0.apply(RpgCommand::AddTopic(RecordId::new("little secret"))).unwrap();
        let line = HeardLine { source: RecordId::new("info.secret.1"), speaker: "Guard".into(), text: "Don't tell.".into() };
        rpg.0.apply(RpgCommand::HearLine { topic: RecordId::new("little secret"), line }).unwrap();
    }
    w.resource_mut::<Sky>().0.advance(3600.0 * 7.0, &Calendar::default());
    {
        let mut weather = w.resource_mut::<Weather>();
        weather.0.weather = WeatherState::new(42);
        weather.0.weather.force(&RecordId::new("yard"), &RecordId::new("storm"), 30.5, Some(0.5), Some(6.0));
        let mut wind = WindState::new(Wind { velocity: Vec2::new(3.0, -1.0), ..Wind::default() });
        wind.add_source(WindSource::radial(WorldPos::from_f64(glam::DVec3::new(2.0, 1.0, 0.0)), 8.0, 5.0).with_lifetime(0.1, 2.0, 0.5));
        wind.advance(0.25, &sf_wind::WindSettings::default());
        weather.0.wind = Some(wind);
    }
    {
        let mut records = w.resource_mut::<Records>();
        records.0.create("item", "potion", &Potion { name: "Yard Tonic".into(), magnitude: 12.5 }).unwrap();
        records.0.patch("npc", &RecordId::new("guard"), &[PatchOp::Set { path: "health".into(), value: Value::F64(0.0) }]);
    }
    w.resource_mut::<Globals>().0.extend([("guard_dead".to_owned(), 1.0), ("crate_count".to_owned(), 2.0)]);
    w.resource_mut::<SaveDisplay>().character_name = "Hero".into();
}

/// Every saved resource of the app, for equality checks.
#[derive(Debug, PartialEq)]
pub(crate) struct State {
    sky: Sky,
    weather: Weather,
    overlay: Overlay,
    rpg: Rpg,
    records: Records,
    party: Party,
    globals: Globals,
}

pub(crate) fn state(app: &App) -> State {
    let w = app.world();
    State {
        sky: w.resource::<Sky>().clone(),
        weather: w.resource::<Weather>().clone(),
        overlay: w.resource::<Overlay>().clone(),
        rpg: w.resource::<Rpg>().clone(),
        records: w.resource::<Records>().clone(),
        party: w.resource::<Party>().clone(),
        globals: w.resource::<Globals>().clone(),
    }
}

/// (key, version, codec, hash) of every section but `sf/meta`.
pub(crate) fn sections_of(path: &Path) -> Vec<(String, u32, sf_save::Codec, [u8; 32])> {
    let f = SaveFile::open(std::fs::read(path).unwrap()).unwrap();
    f.entries().iter().skip(1).map(|e| (e.key.to_string(), e.version, e.codec, e.hash)).collect()
}

#[cfg(feature = "script")]
mod script_game {
    use std::collections::HashMap;

    use sf_script_bevy::sf_script::{ContextConfig, ScriptRuntime, ScriptSource, StateValue, storage_value};
    use sf_script_bevy::Scripts;

    use super::*;

    struct MemSource(HashMap<String, String>);
    impl ScriptSource for MemSource {
        fn read(&self, path: &str) -> Option<Vec<u8>> {
            self.0.get(path).map(|s| s.as_bytes().to_vec())
        }
    }

    const YARD: &str = "local timers = require('@sf/timers')\n\
        local s = sf.storage() s.fired = s.fired or 0 s.visits = s.visits or 0\n\
        timers.register('ring', function(p) sf.storage().fired += p.n end)\n\
        require('@sf/events').on('arm', function() timers.after(2, 'ring', { n = 7 }) end)\n\
        require('@sf/events').on('visit', function() sf.storage().visits += 1 end)";

    pub fn add_scripts(app: &mut App) {
        let mut scripts = Scripts::new(ScriptRuntime::default());
        let source = MemSource(HashMap::from([("scripts/yard/main.luau".to_owned(), YARD.to_owned())]));
        scripts.start(vec![ContextConfig::for_mod("yard")], Arc::new(source));
        app.insert_resource(scripts).add_save_section(sections::script::scripts());
    }

    /// Visits twice, arms the timer, runs half its delay.
    pub fn play_scripts(app: &mut App) {
        let mut s = app.world_mut().resource_mut::<Scripts>();
        s.runtime.dispatch("visit", StateValue::Nil);
        s.runtime.dispatch("visit", StateValue::Nil);
        s.runtime.dispatch("arm", StateValue::Nil);
        s.runtime.update(1.0);
    }

    pub fn number(app: &App, key: &str) -> Option<f64> {
        match storage_value(app.world().resource::<Scripts>().runtime.context("yard")?, key)? {
            StateValue::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn advance(app: &mut App, dt: f64) {
        app.world_mut().resource_mut::<Scripts>().runtime.update(dt);
    }

    pub fn snapshot(app: &App) -> Vec<u8> {
        app.world().resource::<Scripts>().runtime.snapshot("yard").unwrap()
    }
}

// ---- the canonical round trip -----------------------------------------------------------------

#[test]
fn canonical_round_trip_in_the_test_level() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    #[cfg(feature = "script")]
    script_game::add_scripts(&mut a);
    play(&mut a, &y);
    #[cfg(feature = "script")]
    script_game::play_scripts(&mut a);
    let first = save(&mut a, SaveRequest::slot(SaveReason::Manual));
    let played = state(&a);
    assert_eq!(first.sequence, 1);
    assert!(first.per_owner.contains_key("sf") && first.per_owner.contains_key("test"));

    // A fresh game loads it: everything the sections cover comes back.
    let (mut b, _) = new_game(dir.path());
    #[cfg(feature = "script")]
    script_game::add_scripts(&mut b);
    assert_ne!(state(&b), played);
    let report = load(&mut b, LoadRequest::path(&first.path)).unwrap();
    assert!(report.migrated.is_empty() && report.dormant.is_empty() && report.skipped.is_empty());
    assert_eq!(state(&b), played);
    let session = b.world().resource::<SaveSession>().clone();
    assert_eq!((session.world_id, session.last_save, session.sequence), (report.meta.world_id, Some(first.save_id), 1));

    // Save again: identical sections apart from sf/meta (save.md "Saving": equal state, equal bytes).
    let second = save(&mut b, SaveRequest::slot(SaveReason::Manual));
    assert_eq!(second.sequence, 2);
    assert_eq!(sections_of(&first.path), sections_of(&second.path));
    let meta = SaveFile::open(std::fs::read(&second.path).unwrap()).unwrap().meta().unwrap();
    assert_eq!((meta.parent, meta.world_id, meta.sequence), (Some(first.save_id), report.meta.world_id, 2));
    assert_eq!(report.meta.character_name, "Hero");

    // The restored world keeps running like the original: the half-elapsed script timer fires
    // after the rest of its delay, once.
    #[cfg(feature = "script")]
    {
        assert_eq!(script_game::snapshot(&a), script_game::snapshot(&b));
        assert_eq!(script_game::number(&b, "visits"), Some(2.0));
        script_game::advance(&mut b, 0.5);
        assert_eq!(script_game::number(&b, "fired"), Some(0.0));
        script_game::advance(&mut b, 0.6);
        assert_eq!(script_game::number(&b, "fired"), Some(7.0));
        script_game::advance(&mut b, 5.0);
        assert_eq!(script_game::number(&b, "fired"), Some(7.0));
    }

    // Loading in the same app after more play rewinds to the save.
    play(&mut a, &y);
    assert_ne!(state(&a), played);
    load(&mut a, LoadRequest::path(&first.path)).unwrap();
    assert_eq!(state(&a), played);
}

#[test]
fn load_is_a_full_replace() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    // A save of a new game: no globals, nothing in the overlay.
    let empty = save(&mut a, SaveRequest::slot(SaveReason::Manual));
    play(&mut a, &y);
    a.world_mut().resource_mut::<Party>().0.push(CharacterSave { player: PlayerKey(7), ..new_character(&y) });
    let resets = Arc::new(AtomicUsize::new(0));
    let r = resets.clone();
    a.on_load_reset(move |_| {
        r.fetch_add(1, Ordering::Relaxed);
    });
    load(&mut a, LoadRequest::path(&empty.path)).unwrap();
    assert_eq!(resets.load(Ordering::Relaxed), 1);
    let w = a.world();
    assert!(w.resource::<Globals>().0.is_empty());
    assert_eq!(w.resource::<Overlay>().0, PlacementOverlay::default());
    assert_eq!(w.resource::<Party>().0, vec![new_character(&y)]);
    assert_eq!(w.resource::<Rpg>().0, WorldRpg::default());
}

#[test]
fn quicksave_quickload_and_rotation() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let mut paths = Vec::new();
    for i in 0..4 {
        a.world_mut().resource_mut::<Globals>().0.insert("i".into(), f64::from(i));
        paths.push(save(&mut a, SaveRequest::quick()).path);
    }
    let r = save(&mut a, SaveRequest::quick());
    assert_eq!(r.rotated.len(), 1);
    let world = a.world().resource::<SaveSession>().world_id;
    let slot = SlotDir::world(dir.path(), SaveMode::Single, world);
    let quick: Vec<u64> = slot.list(&StdFs).iter().map(|e| e.sequence).collect();
    assert_eq!(quick, [4, 5]);
    a.world_mut().resource_mut::<Globals>().0.insert("i".into(), 99.0);
    let loaded = load(&mut a, LoadRequest::latest(Some(SaveReason::Quick))).unwrap();
    assert_eq!(loaded.meta.sequence, 5);
    assert_eq!(a.world().resource::<Globals>().0["i"], 3.0);
    assert!(!paths[0].exists());
    // Nothing of another reason: not found.
    assert_eq!(load(&mut a, LoadRequest::latest(Some(SaveReason::Exit))), Err(LoadError::NotFound));
}

#[test]
fn autosave_fires_on_play_time() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    a.insert_resource(Time::<Virtual>::default());
    a.world_mut().resource_mut::<Autosave>().interval = Some(Duration::from_secs(600));
    let mut done = Vec::new();
    for _ in 0..25 {
        a.world_mut().resource_mut::<Time<Virtual>>().advance_by(Duration::from_secs(60));
        a.update();
        done.extend(drain::<SaveDone>(&mut a));
    }
    assert_eq!(done.len(), 2, "{done:?}");
    assert!(done.iter().all(|d| d.0.as_ref().is_ok_and(|r| r.reason == SaveReason::Auto)));
    let session = a.world().resource::<SaveSession>();
    assert_eq!(session.play_time, Duration::from_secs(25 * 60));
    let meta = SaveFile::open(std::fs::read(session.last_path.as_ref().unwrap()).unwrap()).unwrap().meta().unwrap();
    assert_eq!(meta.play_time_ms, 20 * 60 * 1000);
}

#[test]
fn the_gate_delays_or_refuses() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    a.world_mut().resource_mut::<SaveGate>().block("streaming burst");
    a.world_mut().write_message(SaveRequest::quick());
    for _ in 0..3 {
        a.update();
        assert!(drain::<SaveDone>(&mut a).is_empty());
    }
    a.world_mut().resource_mut::<SaveGate>().unblock("streaming burst");
    a.update();
    assert!(drain::<SaveDone>(&mut a)[0].0.is_ok());
    a.world_mut().resource_mut::<SaveGate>().can_save =
        Some(Arc::new(|w: &World, reason| if w.resource::<Globals>().0.contains_key("enemies") && reason != SaveReason::Exit { Err("enemies nearby".into()) } else { Ok(()) }));
    a.world_mut().resource_mut::<Globals>().0.insert("enemies".into(), 1.0);
    a.world_mut().write_message(SaveRequest::quick());
    a.update();
    assert_eq!(drain::<SaveDone>(&mut a)[0].0, Err(SaveError::Refused("enemies nearby".into())));
    assert!(save(&mut a, SaveRequest::slot(SaveReason::Exit)).durable);
}

#[test]
fn a_load_cancels_saves_queued_before_it() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let first = save(&mut a, SaveRequest::quick());
    a.world_mut().resource_mut::<SaveGate>().block("door");
    a.world_mut().write_message(SaveRequest::quick());
    a.update();
    a.world_mut().write_message(LoadRequest::path(&first.path));
    a.update();
    assert_eq!(drain::<SaveDone>(&mut a), [SaveDone(Err(SaveError::Cancelled("a load was requested")))]);
    assert!(drain::<LoadDone>(&mut a)[0].0.is_ok());
}

#[test]
fn corrupt_saves_are_refused_and_change_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    play(&mut a, &y);
    let good = save(&mut a, SaveRequest::quick());
    let mut bytes = std::fs::read(&good.path).unwrap();
    let f = SaveFile::open(bytes.clone()).unwrap();
    let rpg = f.entry(&sections::key(sections::RPG)).unwrap();
    let at = usize::try_from(f.preamble().area_start() + rpg.offset).unwrap();
    bytes[at] ^= 0x40;
    let bad = dir.path().join("bad.sfsave");
    std::fs::write(&bad, &bytes).unwrap();
    let (mut b, _) = new_game(dir.path());
    let before = state(&b);
    let err = load(&mut b, LoadRequest::path(&bad)).unwrap_err();
    assert_eq!(err, LoadError::Container(ContainerError::SectionHash { key: sections::key(sections::RPG) }));
    assert_eq!(state(&b), before);
    std::fs::write(&bad, &bytes[..bytes.len() - 3]).unwrap();
    assert!(matches!(load(&mut b, LoadRequest::path(&bad)), Err(LoadError::Container(ContainerError::Length { .. }))));
    assert!(matches!(load(&mut b, LoadRequest::path(dir.path().join("missing.sfsave"))), Err(LoadError::Read(_))));
    assert_eq!(state(&b), before);
}

/// Re-writes a save with extra sections (as another build or a removed mod would have).
fn with_extra(path: &Path, extra: &[(SectionKey, u32, SectionFlags, Vec<u8>)]) -> PathBuf {
    let f = SaveFile::open(std::fs::read(path).unwrap()).unwrap();
    let mut w = SaveWriter::new(FileKind::World, &f.meta().unwrap());
    for e in f.entries().iter().skip(1) {
        w.add_stored(e, f.stored(&e.key).unwrap(), e.flags).unwrap();
    }
    for (k, v, flags, raw) in extra {
        w.add(k.clone(), *v, *flags, raw).unwrap();
    }
    let out = path.with_extension("extra.sfsave");
    std::fs::write(&out, w.finish()).unwrap();
    out
}

#[test]
fn unknown_sections_stay_dormant_byte_for_byte() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let base = save(&mut a, SaveRequest::quick());
    let lantern = SectionKey::new("better_lanterns", "script").unwrap();
    let extra = with_extra(&base.path, &[(lantern.clone(), 3, SectionFlags::NONE, vec![7; 300])]);
    let report = load(&mut a, LoadRequest::path(&extra)).unwrap();
    assert_eq!(report.dormant, std::slice::from_ref(&lantern));
    let again = save(&mut a, SaveRequest::quick());
    let src = SaveFile::open(std::fs::read(&extra).unwrap()).unwrap();
    let out = SaveFile::open(std::fs::read(&again.path).unwrap()).unwrap();
    let (e0, e1) = (src.entry(&lantern).unwrap(), out.entry(&lantern).unwrap());
    assert_eq!((e0.hash, e0.version, e0.codec), (e1.hash, e1.version, e1.codec));
    assert!(e1.flags.contains(SectionFlags::DORMANT));
    assert_eq!(out.raw(&lantern).unwrap(), vec![7; 300]);
    assert!(again.per_owner.contains_key("better_lanterns"));
}

#[test]
fn a_corrupt_optional_section_is_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let base = save(&mut a, SaveRequest::quick());
    let thumb = SectionKey::new("sf", "thumbnail").unwrap();
    let extra = with_extra(&base.path, &[(thumb.clone(), 1, SectionFlags::OPTIONAL, vec![1, 2, 3])]);
    let mut bytes = std::fs::read(&extra).unwrap();
    let f = SaveFile::open(bytes.clone()).unwrap();
    let at = usize::try_from(f.preamble().area_start() + f.entry(&thumb).unwrap().offset).unwrap();
    bytes[at] ^= 1;
    std::fs::write(&extra, bytes).unwrap();
    let report = load(&mut a, LoadRequest::path(&extra)).unwrap();
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].0, thumb);
    assert!(report.dormant.is_empty());
}

/// `test/globals` v1: f32 values (the "older build").
#[derive(Serialize, Deserialize, Default)]
struct GlobalsV1(BTreeMap<String, f32>);

/// The older build's resource behind `test/globals` v1.
#[derive(Resource, Default)]
struct OldGlobals(BTreeMap<String, f32>);

#[test]
fn sections_migrate_from_an_older_version() {
    let dir = tempfile::tempdir().unwrap();
    // An older build: `test/globals` v1 held f32s.
    let mut old = app(dir.path(), false);
    old.init_resource::<OldGlobals>()
        .add_save_section(SaveSection::value(globals_key(), 1, read(|g: &OldGlobals| GlobalsV1(g.0.clone())), write(|g: &mut OldGlobals, v: GlobalsV1| g.0 = v.0)));
    old.world_mut().resource_mut::<OldGlobals>().0.insert("gold_mult".into(), 1.5);
    let v1 = save(&mut old, SaveRequest::quick());

    // This build: v2 (f64) with a typed step.
    // A build that changed the shape without a version bump fails to decode it (strict decoding:
    // the drift guard's runtime half), naming the section, instead of loading garbage.
    let (mut b, _) = new_game(dir.path());
    let err = load(&mut b, LoadRequest::path(&v1.path)).unwrap_err();
    assert!(matches!(&err, LoadError::Decode { key, .. } if *key == globals_key()), "{err}");
    let mut b = app(dir.path(), false);
    b.add_save_section(
        SaveSection::value(globals_key(), 2, read(|g: &Globals| g.0.clone()), write(|g: &mut Globals, v| g.0 = v))
            .typed_step(1, |old: GlobalsV1| old.0.into_iter().map(|(k, v)| (k, f64::from(v))).collect::<BTreeMap<String, f64>>()),
    );
    let report = load(&mut b, LoadRequest::path(&v1.path)).unwrap();
    assert_eq!(report.migrated, [(globals_key(), 1, 2)]);
    assert_eq!(b.world().resource::<Globals>().0["gold_mult"], 1.5);
    // Saved again at the current version only (one-way).
    let v2 = save(&mut b, SaveRequest::quick());
    let f = SaveFile::open(std::fs::read(&v2.path).unwrap()).unwrap();
    assert_eq!(f.entry(&globals_key()).unwrap().version, 2);

    // The older build refuses the newer file, naming the section, and changes nothing.
    old.world_mut().resource_mut::<OldGlobals>().0.insert("x".into(), 2.0);
    let err = load(&mut old, LoadRequest::path(&v2.path)).unwrap_err();
    assert_eq!(err, LoadError::Migrate(MigrateError::TooNew { key: globals_key(), version: 2, current: 1 }));
    assert_eq!(old.world().resource::<OldGlobals>().0.len(), 2);
}

#[test]
fn checks_refuse_before_anything_applies() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    a.world_mut().resource_mut::<BlobVersions>().declare("actor", 3);
    play(&mut a, &y);
    let newer = save(&mut a, SaveRequest::quick());
    let (mut b, _) = new_game(dir.path());
    b.world_mut().resource_mut::<Globals>().0.insert("untouched".into(), 1.0);
    let before = state(&b);
    let err = load(&mut b, LoadRequest::path(&newer.path)).unwrap_err();
    assert!(matches!(&err, LoadError::Check { key, message } if *key == sections::key(sections::OVERLAY) && message.contains("\"actor\"")), "{err}");
    assert_eq!(state(&b), before);
    b.world_mut().resource_mut::<BlobVersions>().declare("actor", 3);
    assert!(load(&mut b, LoadRequest::path(&newer.path)).is_ok());
}

#[test]
fn background_jobs_finish_on_the_task_pool() {
    let dir = tempfile::tempdir().unwrap();
    let y = yard();
    let mut a = app(dir.path(), true);
    a.add_plugins(TaskPoolPlugin::default());
    wire(&mut a);
    a.world_mut().insert_resource(Party(vec![new_character(&y)]));
    play(&mut a, &y);
    let played = state(&a);
    a.world_mut().write_message(SaveRequest::quick());
    let mut done = Vec::new();
    for _ in 0..2000 {
        a.update();
        done.extend(drain::<SaveDone>(&mut a));
        if !done.is_empty() {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let path = done[0].0.as_ref().unwrap().path.clone();
    a.world_mut().insert_resource(Globals::default());
    a.world_mut().write_message(LoadRequest::path(&path));
    let mut loaded = Vec::new();
    for _ in 0..2000 {
        a.update();
        loaded.extend(drain::<LoadDone>(&mut a));
        if !loaded.is_empty() {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(loaded[0].0.is_ok());
    assert_eq!(state(&a), played);
}

#[test]
fn character_sections_are_keyed_by_player() {
    let y = yard();
    let c = new_character(&y);
    let key = sections::character_key(c.player);
    assert_eq!(key.to_string(), "sf/character/00000000000000000000000000c0ffee");
    assert_eq!(sections::player_of(&key), Some(HERO));
    assert_eq!(sections::player_of(&SectionKey::new("sf", "character/xyz").unwrap()), None);
}

#[test]
fn record_overlay_reapplies_and_reports_what_it_skips() {
    use sf_records::RecordDb;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        name: String,
        magnitude: f32,
    }
    let mut db = RecordDb::new();
    db.register_serde::<Item>("item").unwrap();
    let base = db.add_source("base", &[]);
    db.define(base, RecordId::new("bread"), Item { name: "Bread".into(), magnitude: 1.0 }).unwrap();
    let mut overlay = RecordOverlay::default();
    let id = overlay.create("item", "potion", &Potion { name: "Tonic".into(), magnitude: 3.0 }).unwrap();
    assert_eq!(id.as_str(), "save:potion.0");
    overlay.patch("item", &RecordId::new("bread"), &[PatchOp::Set { path: "magnitude".into(), value: Value::F64(2.0) }]);
    overlay.patch("item", &RecordId::new("gone"), &[PatchOp::Set { path: "magnitude".into(), value: Value::F64(2.0) }]);
    overlay.patch("spell", &RecordId::new("x"), &[PatchOp::Remove { path: "a".into() }]);
    let bytes = encode(&overlay);
    let back: RecordOverlay = sf_save::decode(&bytes).unwrap();
    assert_eq!(back, overlay);
    let save_src = db.add_source("save", &[base]);
    let skipped = back.apply(&mut db, save_src);
    assert_eq!(skipped.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(), ["gone", "x"]);
    assert_eq!(db.get::<Item>(&id), Some(&Item { name: "Tonic".into(), magnitude: 3.0 }));
    assert_eq!(db.get::<Item>(&RecordId::new("bread")).unwrap().magnitude, 2.0);
}
