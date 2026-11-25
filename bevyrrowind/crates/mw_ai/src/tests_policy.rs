//! `MA2b`: `OpenMW`'s `AiSequence::stack` / `execute` orders under [`MorrowindPolicy`] (`policy.rs`).

use sf_ai::{
    ActorKey, DirectiveDone, DirectiveEnd, DirectiveId, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack,
    GameTime, Limit, ModernPolicy, StackPolicy,
};
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_world::{Location, PlacementId, WorldspaceId};

use crate::policy::{MorrowindPolicy, Options, is_actual_package, options};

const NPC: ActorKey = ActorKey(0xA1);
const PLAYER: ActorKey = ActorKey(7);
const GUARD: ActorKey = ActorKey(8);

fn t(minutes: u64) -> GameTime {
    GameTime::from_minutes(minutes)
}

fn place() -> Location {
    Location::new(WorldspaceId::new("test"), WorldPos::ORIGIN)
}

fn wander() -> DirectiveKind {
    DirectiveKind::StayNear { home: place(), radius: 5.0 }
}

fn travel() -> DirectiveKind {
    DirectiveKind::GoTo(place())
}

fn ids(stack: &DirectiveStack) -> Vec<u32> {
    stack.iter().map(|d| d.id.0).collect()
}

fn done(id: u32, end: DirectiveEnd, at: u64) -> DirectiveDone {
    DirectiveDone { actor: NPC, id: DirectiveId(id), end, at: t(at) }
}

/// Pushes `specs` in order at time 0; returns the stack and the events so far.
fn stacked<P: StackPolicy>(policy: &P, specs: Vec<DirectiveSpec>) -> (DirectiveStack, Vec<DirectiveDone>) {
    let mut stack = DirectiveStack::new(NPC, t(0));
    let mut events = Vec::new();
    for spec in specs {
        stack.push(spec, t(0), policy, &mut events);
    }
    (stack, events)
}

#[test]
fn options_follow_openmw_package_defaults() {
    let d = Options::DEFAULT;
    assert_eq!(options(&wander()), Options { use_variable_speed: true, ..d });
    assert_eq!(options(&travel()), Options { use_variable_speed: true, always_active: true, ..d });
    assert_eq!(
        options(&DirectiveKind::Escort { who: PLAYER, to: place() }),
        Options { use_variable_speed: true, side_with_target: true, ..d }
    );
    assert_eq!(
        options(&DirectiveKind::Follow(PLAYER)),
        Options { use_variable_speed: true, side_with_target: true, follow_target_through_doors: true, ..d }
    );
    assert_eq!(options(&DirectiveKind::Use(PlacementId(42))), d);
    assert_eq!(
        options(&DirectiveKind::Fight(PLAYER)),
        Options { priority: 1, can_cancel: false, should_cancel_previous_ai: false, ..d }
    );
    assert_eq!(options(&DirectiveKind::Custom(RecordId::new("x"))), d);
    assert!(is_actual_package(&wander()) && is_actual_package(&DirectiveKind::Use(PlacementId(1))));
    assert!(!is_actual_package(&DirectiveKind::Fight(PLAYER)));
    assert!(!is_actual_package(&DirectiveKind::Custom(RecordId::new("x"))));
}

#[test]
fn record_packages_keep_record_order() {
    // AiSequence::fill appends; the modern policy puts the newest first among equals.
    let specs = || {
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record),
            DirectiveSpec::new(travel(), DirectiveSource::Record),
            DirectiveSpec::new(DirectiveKind::Follow(PLAYER), DirectiveSource::Record),
        ]
    };
    let (mw, events) = stacked(&MorrowindPolicy, specs());
    assert_eq!(ids(&mw), [0, 1, 2]);
    assert!(events.is_empty());
    let (modern, _) = stacked(&ModernPolicy, specs());
    assert_eq!(ids(&modern), [2, 1, 0]);
}

#[test]
fn script_travel_cancels_the_record_wander() {
    let (stack, events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record).repeating(),
            DirectiveSpec::new(DirectiveKind::Use(PlacementId(42)), DirectiveSource::Record),
            DirectiveSpec::new(travel(), DirectiveSource::Script),
        ],
    );
    // shouldCancelPreviousAi: both cancellable packages go (the repeating wander does not come back).
    assert_eq!(ids(&stack), [2]);
    assert_eq!(events, [done(0, DirectiveEnd::Cancelled, 0), done(1, DirectiveEnd::Cancelled, 0)]);
}

#[test]
fn fight_goes_above_without_cancelling() {
    let (stack, events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record),
            DirectiveSpec::new(travel(), DirectiveSource::Record),
            DirectiveSpec::new(DirectiveKind::Fight(PLAYER), DirectiveSource::Script),
        ],
    );
    // Combat: priority 1, shouldCancelPreviousAi false.
    assert_eq!(ids(&stack), [2, 0, 1]);
    assert!(events.is_empty());
}

#[test]
fn a_new_package_stops_combat() {
    let (stack, events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record),
            DirectiveSpec::new(DirectiveKind::Fight(PLAYER), DirectiveSource::Script),
            DirectiveSpec::new(DirectiveKind::Fight(GUARD), DirectiveSource::Script),
            DirectiveSpec::new(DirectiveKind::Follow(PLAYER), DirectiveSource::Script),
        ],
    );
    // stopCombat first (every fight, top first), then the cancellable wander.
    assert_eq!(ids(&stack), [3]);
    assert_eq!(
        events,
        [
            done(2, DirectiveEnd::Cancelled, 0),
            done(1, DirectiveEnd::Cancelled, 0),
            done(0, DirectiveEnd::Cancelled, 0),
        ]
    );
}

#[test]
fn equal_priority_goes_above_whatever_the_source() {
    let specs = || {
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record),
            DirectiveSpec::new(DirectiveKind::Fight(PLAYER), DirectiveSource::Quest),
            DirectiveSpec::new(DirectiveKind::Fight(GUARD), DirectiveSource::Script),
        ]
    };
    // OpenMW: before the first package of lower or equal priority, so the newest fight is on top.
    let (mw, _) = stacked(&MorrowindPolicy, specs());
    assert_eq!(ids(&mw), [2, 1, 0]);
    // Modern: the quest outranks the script.
    let (modern, _) = stacked(&ModernPolicy, specs());
    assert_eq!(ids(&modern), [1, 2, 0]);
}

#[test]
fn a_finished_repeating_package_goes_to_the_end() {
    let (mut stack, mut events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(travel(), DirectiveSource::Record).repeating(),
            DirectiveSpec::new(wander(), DirectiveSource::Record),
        ],
    );
    assert!(stack.finish(DirectiveId(0), t(5), &MorrowindPolicy, &mut events));
    assert_eq!(ids(&stack), [1, 2]);
    assert_eq!(stack.get(DirectiveId(2)).unwrap().spec.kind, travel());
    assert_eq!(events, [done(0, DirectiveEnd::Completed, 5)]);
}

#[test]
fn a_finished_package_without_repeat_leaves() {
    let (mut stack, mut events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(travel(), DirectiveSource::Record),
            DirectiveSpec::new(wander(), DirectiveSource::Record),
        ],
    );
    assert!(stack.finish(DirectiveId(0), t(5), &MorrowindPolicy, &mut events));
    assert_eq!(ids(&stack), [1]);
    assert_eq!(stack.next_id(), DirectiveId(2));
    assert_eq!(events, [done(0, DirectiveEnd::Completed, 5)]);
}

#[test]
fn a_repeating_package_whose_duration_runs_out_goes_to_the_end() {
    let (mut stack, mut events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record).repeating().limit(Limit::For { minutes: 60 }),
            DirectiveSpec::new(travel(), DirectiveSource::Record),
        ],
    );
    stack.advance(t(60), &MorrowindPolicy, &mut events);
    assert_eq!(ids(&stack), [1, 2]);
    assert_eq!(events, [done(0, DirectiveEnd::Expired, 60)]);
}

#[test]
fn fights_and_cancelled_packages_never_repeat() {
    let (mut stack, mut events) = stacked(
        &MorrowindPolicy,
        vec![
            DirectiveSpec::new(wander(), DirectiveSource::Record).repeating(),
            DirectiveSpec::new(DirectiveKind::Fight(PLAYER), DirectiveSource::Script).repeating(),
        ],
    );
    assert_eq!(ids(&stack), [1, 0]);
    // Combat is not an actual package (execute only re-queues those).
    assert!(stack.finish(DirectiveId(1), t(1), &MorrowindPolicy, &mut events));
    assert!(stack.cancel(DirectiveId(0), t(2), &MorrowindPolicy, &mut events));
    assert!(stack.is_empty());
    assert_eq!(events, [done(1, DirectiveEnd::Completed, 1), done(0, DirectiveEnd::Cancelled, 2)]);
}
