//! ai.md "Build plan" AI5 (Bevy-free half): the `sf/ai` shape round-trips byte-identically and
//! carries on identically; a loaded section that breaks a stack's invariants is refused, naming
//! the actor. The section codec, the shape golden and old saves are tested in `sf_save_bevy`.

use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_rpg::GameTime;
use sf_world::{Location, PlacementId, WorldspaceId};

use crate::*;

const GUARD: ActorKey = ActorKey(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10);
const TRADER: ActorKey = ActorKey(0xA1);
const IDLE: ActorKey = ActorKey(u128::MAX);

fn t(minutes: u32) -> GameTime {
    GameTime { day: minutes / 1440, minute: u16::try_from(minutes % 1440).unwrap() }
}

fn place(ws: &str) -> Location {
    Location::new(WorldspaceId::new(ws), WorldPos::ORIGIN)
}

/// Three actors: one with every kind and both limits, a named served directive and gaps in its
/// ids (a finish, an expiry, a cancel); one suspended; one with an empty stack whose ids moved.
fn world() -> Vec<DirectiveStack> {
    let policy = ModernPolicy;
    let mut events = Vec::new();
    let mut guard = DirectiveStack::new(GUARD, t(60));
    let kinds = [
        DirectiveKind::StayNear { home: place("town"), radius: 4.25 },
        DirectiveKind::GoTo(place("cave")),
        DirectiveKind::Follow(TRADER),
        DirectiveKind::Escort { who: TRADER, to: place("vault") },
        DirectiveKind::Use(PlacementId(0xdead_beef)),
        DirectiveKind::Fight(IDLE),
        DirectiveKind::Custom(RecordId::new("patrol")),
    ];
    for (i, kind) in kinds.into_iter().enumerate() {
        let source = [DirectiveSource::Record, DirectiveSource::Schedule, DirectiveSource::Quest][i % 3];
        let limit = if i % 2 == 0 { Limit::For { minutes: 90 + u32::try_from(i).unwrap() } } else { Limit::Until(t(3000)) };
        let mut spec = DirectiveSpec::new(kind, source).priority(u8::try_from(i).unwrap()).limit(limit);
        if i == 3 {
            spec = spec.repeating().origin("quest_escort");
        }
        guard.push(spec, t(60), &policy, &mut events);
    }
    // dir7 on top for 5 served minutes: it expires at 66.
    let short = DirectiveSpec::new(DirectiveKind::Custom(RecordId::new("bark")), DirectiveSource::Quest).priority(9).limit(Limit::For { minutes: 5 });
    guard.push(short, t(61), &policy, &mut events);
    guard.finish(DirectiveId(1), t(62), &policy, &mut events);
    guard.cancel(DirectiveId(5), t(63), &policy, &mut events);
    guard.advance(t(67), &policy, &mut events);
    guard.serve(Serving::Directive(DirectiveId(4)), t(67), &policy, &mut events);
    guard.advance(t(70), &policy, &mut events);

    let mut trader = DirectiveStack::new(TRADER, t(10));
    let wander = DirectiveSpec::new(DirectiveKind::Custom(RecordId::new("wander")), DirectiveSource::Record).repeating().limit(Limit::For { minutes: 30 });
    trader.push(wander, t(10), &policy, &mut events);
    trader.advance(t(75), &policy, &mut events);
    trader.serve(Serving::Suspended, t(75), &policy, &mut events);

    let mut idle = DirectiveStack::new(IDLE, t(0));
    let id = idle.push(DirectiveSpec::new(DirectiveKind::GoTo(place("x")), DirectiveSource::Script), t(0), &policy, &mut events);
    idle.finish(id, t(1), &policy, &mut events);
    vec![guard, trader, idle]
}

#[test]
fn the_section_key_and_version() {
    assert_eq!(AI_SECTION, "sf/ai");
    assert_eq!(AI_VERSION, 1);
}

#[test]
fn every_actors_stack_round_trips_byte_identically_and_carries_on_identically() {
    let mut live = world();
    // The fixture covers what it claims: gaps, a named served directive, a suspended actor.
    assert_eq!(live[0].iter().map(|d| d.id.0).collect::<Vec<_>>(), [2, 4, 6, 3, 0]);
    assert_eq!(live[0].next_id(), DirectiveId(8));
    assert_eq!(live[0].get(DirectiveId(4)).unwrap().served, 3);
    assert_eq!(live[0].serving().unwrap().id, DirectiveId(4));
    assert_eq!(live[1].serving_mode(), Serving::Suspended);
    assert!(live[2].is_empty());

    let save = AiSave::from_stacks(&live);
    let bytes = postcard::to_stdvec(&save).unwrap();
    let decoded: AiSave = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(decoded, save);
    decoded.check().unwrap();
    let mut loaded = decoded.into_stacks().unwrap();
    // Key order (the section is a sorted map); the same stacks.
    live.sort_by_key(DirectiveStack::actor);
    assert_eq!(loaded, live);
    assert_eq!(postcard::to_stdvec(&AiSave::from_stacks(&loaded)).unwrap(), bytes);

    // The same future: next ids, expiries and `done(id)` answers match.
    let policy = ModernPolicy;
    let mut futures = [Vec::new(), Vec::new()];
    for (stacks, events) in [&mut live, &mut loaded].into_iter().zip(&mut futures) {
        for s in stacks.iter_mut() {
            let next = s.next_id();
            let id = s.push(DirectiveSpec::new(DirectiveKind::Follow(GUARD), DirectiveSource::Memory), t(80), &policy, events);
            assert_eq!(id, next);
            s.serve(Serving::Top, t(90), &policy, events);
            s.advance(t(4000), &policy, events);
        }
    }
    assert_eq!(futures[0], futures[1]);
    assert!(!futures[0].is_empty());
    assert_eq!(loaded, live);
    let guard = loaded.iter().find(|s| s.actor() == GUARD).unwrap();
    assert_eq!(guard.status(DirectiveId(1)), DirectiveStatus::Ended);
    assert_eq!(guard.status(DirectiveId(99)), DirectiveStatus::Unknown);
}

#[test]
fn a_stack_that_breaks_its_invariants_is_refused_naming_the_actor() {
    let good = AiSave::from_stacks(&world());
    let edit = |f: &dyn Fn(&mut SavedStack)| {
        let mut save = good.clone();
        f(&mut save.actors.get_mut(&GUARD).unwrap().directives);
        save
    };
    let cases: [(AiSave, StackError); 3] = [
        (edit(&|s| s.next_id = 4), StackError::NotIssued { id: DirectiveId(4), next: 4 }),
        (edit(&|s| s.entries[1].id = DirectiveId(2)), StackError::Duplicate { id: DirectiveId(2) }),
        (edit(&|s| s.serving = Serving::Directive(DirectiveId(50))), StackError::ServingNotIssued { id: DirectiveId(50), next: 8 }),
    ];
    for (save, error) in cases {
        let expected = AiSaveError { actor: GUARD, error };
        assert_eq!(save.check(), Err(expected.clone()));
        assert_eq!(save.clone().into_stacks(), Err(expected.clone()));
        assert!(expected.to_string().starts_with("actor 0102030405060708090a0b0c0d0e0f10: "), "{expected}");
    }
    // A served directive that already left is legitimate (serving falls back to the top).
    let left = edit(&|s| s.serving = Serving::Directive(DirectiveId(1)));
    assert_eq!(left.check(), Ok(()));
    assert_eq!(AiSave::default().into_stacks(), Ok(Vec::new()));
}
