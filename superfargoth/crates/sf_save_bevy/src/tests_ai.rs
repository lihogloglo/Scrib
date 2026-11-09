//! `sf/ai` (ai.md "Build plan" AI5: "round trip; old saves load"), through the real section
//! codec: several actors' directive stacks save and load byte-identically and carry on
//! identically; a save from before AI5 loads with no AI state; a newer `sf/ai` and a stack that
//! breaks its invariants are refused, naming the section, before anything applies. The shape
//! golden is `tests/shapes/sf.ai.v1.json` (`tests_formats.rs`).

use sf_ai::{
    ActorKey, AiSave, DirectiveId, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, GameTime, Limit,
    ModernPolicy, Serving,
};
use sf_records::RecordId;
use sf_save::{SaveFile, SectionFlags};
use sf_world::{Location, PlacementId, WorldspaceId};

use crate::sections;
use crate::tests::{Ai, app, load, new_game, play, save, state, wire, with_extra};
use crate::{LoadError, LoadRequest, MigrateError, SaveRequest};

const GUARD: ActorKey = ActorKey(0x0bad_cafe_0000_0000_0000_0000_0000_0001);
const TRADER: ActorKey = ActorKey(0x42);
/// A spawned actor: its key is a spawn id from the saved overlay counter.
const SPAWNED: ActorKey = ActorKey(u128::MAX - 7);

fn t(minutes: u32) -> GameTime {
    GameTime { day: minutes / 1440, minute: u16::try_from(minutes % 1440).unwrap() }
}

fn at(ws: &str) -> Location {
    Location::new(WorldspaceId::new(ws), sf_coords::WorldPos::new(glam::IVec3::new(2, 0, -1), glam::Vec3::new(1.5, 0.25, -30.0)))
}

/// Three actors: every kind, both limits, a named served directive and id gaps; a suspended
/// repeat; an emptied stack whose id sequence moved on.
fn stacks() -> Vec<DirectiveStack> {
    let p = ModernPolicy;
    let mut ev = Vec::new();
    let mut guard = DirectiveStack::new(GUARD, t(600));
    let kinds = [
        DirectiveKind::StayNear { home: at("town"), radius: 6.5 },
        DirectiveKind::GoTo(at("mine")),
        DirectiveKind::Follow(TRADER),
        DirectiveKind::Escort { who: TRADER, to: at("fort") },
        DirectiveKind::Use(PlacementId(0x1234_5678_9abc)),
        DirectiveKind::Fight(SPAWNED),
        DirectiveKind::Custom(RecordId::new("sweep_floor")),
    ];
    for (i, kind) in kinds.into_iter().enumerate() {
        let n = u32::try_from(i).unwrap();
        let limit = if i % 2 == 0 { Limit::For { minutes: 120 + n } } else { Limit::Until(t(2000 + n)) };
        let source = [DirectiveSource::Record, DirectiveSource::Script, DirectiveSource::Quest][i % 3];
        guard.push(DirectiveSpec::new(kind, source).priority(u8::try_from(i).unwrap()).limit(limit).origin("q_guard"), t(600), &p, &mut ev);
    }
    guard.finish(DirectiveId(2), t(605), &p, &mut ev);
    guard.cancel(DirectiveId(4), t(606), &p, &mut ev);
    guard.serve(Serving::Directive(DirectiveId(3)), t(610), &p, &mut ev);
    guard.advance(t(640), &p, &mut ev);

    let mut trader = DirectiveStack::new(TRADER, t(0));
    let wander = DirectiveSpec::new(DirectiveKind::Custom(RecordId::new("wander")), DirectiveSource::Record).repeating();
    trader.push(wander.limit(Limit::For { minutes: 45 }), t(0), &p, &mut ev);
    trader.advance(t(100), &p, &mut ev);
    trader.serve(Serving::Suspended, t(100), &p, &mut ev);

    let mut spawned = DirectiveStack::new(SPAWNED, t(500));
    let id = spawned.push(DirectiveSpec::new(DirectiveKind::Follow(GUARD), DirectiveSource::Script), t(500), &p, &mut ev);
    spawned.finish(id, t(501), &p, &mut ev);
    vec![guard, trader, spawned]
}

/// Pushes, serves and advances every stack the same way; returns the events.
fn future(stacks: &mut [DirectiveStack]) -> Vec<sf_ai::DirectiveDone> {
    let mut ev = Vec::new();
    for s in stacks {
        let next = s.next_id();
        let id = s.push(DirectiveSpec::new(DirectiveKind::GoTo(at("inn")), DirectiveSource::Schedule).limit(Limit::Until(t(900))), t(700), &ModernPolicy, &mut ev);
        assert_eq!(id, next);
        s.serve(Serving::Top, t(710), &ModernPolicy, &mut ev);
        s.advance(t(5000), &ModernPolicy, &mut ev);
    }
    ev
}

#[test]
fn ai_stacks_round_trip_through_the_section_codec_and_carry_on_identically() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    play(&mut a, &y);
    let live = stacks();
    a.world_mut().resource_mut::<Ai>().0 = AiSave::from_stacks(&live);
    let first = save(&mut a, SaveRequest::quick());
    let f = SaveFile::open(std::fs::read(&first.path).unwrap()).unwrap();
    assert_eq!(f.entry(&sections::key(sections::AI)).unwrap().version, sections::AI_VERSION);
    assert_eq!(sections::key(sections::AI).to_string(), sf_ai::AI_SECTION);

    // A fresh game loads the same state, and re-saves the same section bytes.
    let (mut b, _) = new_game(dir.path());
    load(&mut b, LoadRequest::path(&first.path)).unwrap();
    assert_eq!(state(&b), state(&a));
    let again = save(&mut b, SaveRequest::quick());
    let raw = |p: &std::path::Path| SaveFile::open(std::fs::read(p).unwrap()).unwrap().raw(&sections::key(sections::AI)).unwrap();
    assert_eq!(raw(&again.path), raw(&first.path));

    // The loaded stacks behave like the saved ones: next ids, expiries, `done(id)`.
    let mut loaded = b.world().resource::<Ai>().0.clone().into_stacks().unwrap();
    let mut original = live;
    original.sort_by_key(DirectiveStack::actor);
    assert_eq!(loaded, original);
    let events = future(&mut original);
    assert_eq!(future(&mut loaded), events);
    assert!(events.len() > 3, "{events:?}");
    assert_eq!(loaded, original);
}

#[test]
fn a_save_from_before_ai5_loads_with_no_ai_state() {
    let dir = tempfile::tempdir().unwrap();
    // The released S2 golden (written before `sf/ai` existed).
    let s2 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/s2.sfsave");
    let f = SaveFile::open(std::fs::read(&s2).unwrap()).unwrap();
    assert!(f.entry(&sections::key(sections::AI)).is_none());
    let mut b = app(dir.path(), false);
    wire(&mut b);
    b.world_mut().resource_mut::<Ai>().0 = AiSave::from_stacks(&stacks());
    let report = load(&mut b, LoadRequest::path(&s2)).unwrap();
    assert!(report.dormant.is_empty(), "{report:?}");
    // Full replace: the actors start from their record defaults (no saved stacks).
    assert_eq!(b.world().resource::<Ai>().0, AiSave::default());
    // And a world without AI state writes no `sf/ai`.
    let again = save(&mut b, SaveRequest::quick());
    assert!(SaveFile::open(std::fs::read(&again.path).unwrap()).unwrap().entry(&sections::key(sections::AI)).is_none());
}

#[test]
fn a_newer_or_broken_sf_ai_is_refused_before_anything_applies() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let base = save(&mut a, SaveRequest::quick());
    let (mut b, _) = new_game(dir.path());
    b.world_mut().resource_mut::<Ai>().0 = AiSave::from_stacks(&stacks());
    let before = state(&b);
    let key = sections::key(sections::AI);

    let newer = with_extra(&base.path, &[(key.clone(), sections::AI_VERSION + 1, SectionFlags::NONE, vec![0])]);
    let err = load(&mut b, LoadRequest::path(&newer)).unwrap_err();
    assert_eq!(err, LoadError::Migrate(MigrateError::TooNew { key: key.clone(), version: sections::AI_VERSION + 1, current: sections::AI_VERSION }));
    assert!(err.to_string().contains("sf/ai"), "{err}");
    assert_eq!(state(&b), before);

    // A stack whose entries the id sequence never issued (corrupt or forged).
    let mut bad = AiSave::from_stacks(&stacks());
    bad.actors.get_mut(&GUARD).unwrap().directives.next_id = 1;
    let broken = with_extra(&base.path, &[(key.clone(), sections::AI_VERSION, SectionFlags::NONE, sf_save::encode(&bad))]);
    let err = load(&mut b, LoadRequest::path(&broken)).unwrap_err();
    assert!(
        matches!(&err, LoadError::Check { key: k, message } if *k == key && message.contains(&format!("actor {:032x}", GUARD.0))),
        "{err}"
    );
    assert_eq!(state(&b), before);
}
