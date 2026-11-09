//! ai.md "Build plan" AI4: exact stack orders, a policy double, completion events, expiry on the
//! logical clock, the save round trip.

use std::sync::Mutex;

use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_rpg::GameTime;
use sf_world::{Location, PlacementId, WorldspaceId};

use crate::*;

const NPC: ActorKey = ActorKey(0xA1);
const OTHER: ActorKey = ActorKey(0xB2);

/// Game time from minutes since the start.
fn t(minutes: u32) -> GameTime {
    GameTime { day: minutes / 1440, minute: u16::try_from(minutes % 1440).unwrap() }
}

fn custom(name: &str, source: DirectiveSource) -> DirectiveSpec {
    DirectiveSpec::new(DirectiveKind::Custom(RecordId::new(name)), source)
}

fn ids(stack: &DirectiveStack) -> Vec<u32> {
    stack.iter().map(|d| d.id.0).collect()
}

fn done(id: u32, end: DirectiveEnd, at: u32) -> DirectiveDone {
    DirectiveDone { actor: NPC, id: DirectiveId(id), end, at: t(at) }
}

fn place(ws: &str) -> Location {
    Location::new(WorldspaceId::new(ws), WorldPos::ORIGIN)
}

#[test]
fn mixed_sources_give_the_exact_order_under_the_modern_policy() {
    use DirectiveSource::{Memory, Quest, Record, Schedule, Script};
    let mut stack = DirectiveStack::new(NPC, t(0));
    let mut events = Vec::new();
    // (source, priority) pushed in this order; ids 0..=8.
    let pushes = [
        (Quest, 0),
        (Record, 5),
        (Schedule, 1),
        (Script, 3),
        (Record, 5),
        (Memory, 0),
        (Quest, 2),
        (Record, 0),
        (Schedule, 1),
    ];
    for (i, (source, priority)) in pushes.into_iter().enumerate() {
        let id = stack.push(custom(&format!("c{i}"), source).priority(priority), t(0), &ModernPolicy, &mut events);
        assert_eq!(id, DirectiveId(u32::try_from(i).unwrap()));
    }
    // Source rank first (quest > script > memory > schedule > record), then priority, newest
    // first among equals (8 above 2, 4 above 1).
    assert_eq!(ids(&stack), [6, 0, 3, 5, 8, 2, 4, 1, 7]);
    assert!(events.is_empty());
    assert_eq!(stack.top().unwrap().id, DirectiveId(6));
    assert_eq!(stack.serving().unwrap().id, DirectiveId(6));
    assert_eq!(stack.status(DirectiveId(6)), DirectiveStatus::Serving);
    assert_eq!(stack.status(DirectiveId(7)), DirectiveStatus::Waiting);
    assert_eq!(stack.status(DirectiveId(9)), DirectiveStatus::Unknown);
}

/// Test double: OpenMW-shaped rules (priority only, newest first among equals, repeats to the
/// bottom, a "takeover" origin cancels every record directive), every call recorded.
#[derive(Default)]
struct Double {
    calls: Mutex<Vec<String>>,
}

impl Double {
    fn log(&self, line: String) {
        self.calls.lock().unwrap().push(line);
    }

    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}

impl StackPolicy for Double {
    fn push(&self, stack: &mut StackEdit<'_>, d: Directive) {
        self.log(format!("push {}", d.id));
        if d.spec.origin.as_ref().is_some_and(|o| o.as_str() == "refuse") {
            return;
        }
        if d.spec.origin.as_ref().is_some_and(|o| o.as_str() == "takeover") {
            stack.cancel_where(|e| e.spec.source == DirectiveSource::Record);
        }
        let at = stack.iter().position(|e| e.spec.priority <= d.spec.priority).unwrap_or(stack.len());
        stack.insert(at, d);
    }

    fn on_end(&self, stack: &mut StackEdit<'_>, ended: &Directive, end: DirectiveEnd) {
        self.log(format!("end {} {end:?}", ended.id));
        if ended.spec.repeat && end != DirectiveEnd::Cancelled {
            let again = stack.renew(ended);
            self.log(format!("requeue {} as {}", ended.id, again.id));
            stack.push_bottom(again);
        }
    }
}

#[test]
fn a_policy_double_is_honoured_exactly() {
    use DirectiveSource::{Quest, Record, Script};
    let policy = Double::default();
    let mut stack = DirectiveStack::new(NPC, t(0));
    let mut events = Vec::new();
    stack.push(custom("wander", Record).priority(1).repeating(), t(0), &policy, &mut events); // 0
    stack.push(custom("guard", Record).priority(4), t(0), &policy, &mut events); // 1
    stack.push(custom("talk", Quest).priority(2), t(0), &policy, &mut events); // 2
    // Priority only: the record's 4 beats the quest's 2 (the default would rank the quest first).
    assert_eq!(ids(&stack), [1, 2, 0]);

    // The repeating wander completes: back to the bottom with a fresh id.
    assert!(stack.finish(DirectiveId(0), t(5), &policy, &mut events));
    stack.push(custom("script", Script).priority(2), t(6), &policy, &mut events); // 4
    assert_eq!(ids(&stack), [1, 4, 2, 3]);

    // A takeover cancels both record directives, then sits by priority; a refused push ends at once.
    stack.push(custom("escort", Script).priority(0).origin("takeover"), t(7), &policy, &mut events); // 5
    stack.push(custom("nope", Script).origin("refuse"), t(8), &policy, &mut events); // 6
    assert_eq!(ids(&stack), [4, 2, 5]);
    assert_eq!(
        events,
        [
            done(0, DirectiveEnd::Completed, 5),
            done(1, DirectiveEnd::Cancelled, 7),
            done(3, DirectiveEnd::Cancelled, 7),
            done(6, DirectiveEnd::Cancelled, 8),
        ]
    );
    assert_eq!(
        policy.calls(),
        [
            "push dir0",
            "push dir1",
            "push dir2",
            "end dir0 Completed",
            "requeue dir0 as dir3",
            "push dir4",
            "push dir5",
            "end dir1 Cancelled",
            "end dir3 Cancelled",
            "push dir6",
        ]
    );
    for id in [0, 1, 3, 6] {
        assert_eq!(stack.status(DirectiveId(id)), DirectiveStatus::Ended);
    }
}

#[test]
fn completion_pops_or_requeues_and_emits_the_exact_event() {
    use DirectiveSource::{Record, Script};
    let mut stack = DirectiveStack::new(NPC, t(100));
    let mut events = Vec::new();
    let go = stack.push(
        DirectiveSpec::new(DirectiveKind::GoTo(place("town")), Script).priority(3),
        t(100),
        &ModernPolicy,
        &mut events,
    );
    let wander = stack.push(
        DirectiveSpec::new(DirectiveKind::StayNear { home: place("town"), radius: 5.0 }, Record).repeating(),
        t(100),
        &ModernPolicy,
        &mut events,
    );
    let usage = stack.push(
        DirectiveSpec::new(DirectiveKind::Use(PlacementId(7)), Record).priority(2).repeating(),
        t(100),
        &ModernPolicy,
        &mut events,
    );
    assert_eq!([go, wander, usage].map(|d| d.0), [0, 1, 2]);
    assert_eq!(ids(&stack), [0, 2, 1]);

    // Non-repeating: popped.
    assert!(stack.finish(go, t(110), &ModernPolicy, &mut events));
    assert_eq!(ids(&stack), [2, 1]);
    // Repeating: a fresh instance re-pushed by the same rule (record p2, above the wander).
    assert!(stack.finish(usage, t(120), &ModernPolicy, &mut events));
    assert_eq!(ids(&stack), [3, 1]);
    assert_eq!(stack.get(DirectiveId(3)).unwrap().spec.kind, DirectiveKind::Use(PlacementId(7)));
    // Already gone, or never issued: nothing happens.
    assert!(!stack.finish(go, t(130), &ModernPolicy, &mut events));
    assert!(!stack.finish(DirectiveId(99), t(130), &ModernPolicy, &mut events));
    // Cancelling never repeats.
    assert!(stack.cancel(wander, t(140), &ModernPolicy, &mut events));
    assert_eq!(ids(&stack), [3]);
    assert_eq!(
        events,
        [
            done(0, DirectiveEnd::Completed, 110),
            done(2, DirectiveEnd::Completed, 120),
            done(1, DirectiveEnd::Cancelled, 140),
        ]
    );
    assert_eq!(stack.status(usage), DirectiveStatus::Ended);
    assert_eq!(stack.status(DirectiveId(3)), DirectiveStatus::Serving);

    // A quest stopping cancels everything it issued.
    stack.push(custom("q1", DirectiveSource::Quest).origin("quest_a"), t(150), &ModernPolicy, &mut events);
    stack.push(custom("q2", DirectiveSource::Quest).origin("quest_a"), t(150), &ModernPolicy, &mut events);
    events.clear();
    let n = stack.cancel_where(t(160), &ModernPolicy, &mut events, |d| {
        d.spec.origin.as_ref().is_some_and(|o| o.as_str() == "quest_a")
    });
    assert_eq!(n, 2);
    assert_eq!(events, [done(5, DirectiveEnd::Cancelled, 160), done(4, DirectiveEnd::Cancelled, 160)]);
    assert_eq!(ids(&stack), [3]);
}

/// One scripted run: a timed wander under a quest interrupt, a combat suspension and a
/// schedule window; returns the stack and every event.
fn timed_run(step: u32) -> (DirectiveStack, Vec<DirectiveDone>) {
    use DirectiveSource::{Quest, Record, Schedule};
    let policy = ModernPolicy;
    let mut stack = DirectiveStack::new(NPC, t(0));
    let mut events = Vec::new();
    // Steps the clock from `from` to `to` in `step`-minute increments (the tier's rate).
    let run = |stack: &mut DirectiveStack, events: &mut Vec<DirectiveDone>, from: u32, to: u32| {
        let mut m = from;
        while m < to {
            m = (m + step).min(to);
            stack.advance(t(m), &policy, events);
        }
    };
    stack.push(custom("wander", Record).limit(Limit::For { minutes: 60 }).repeating(), t(0), &policy, &mut events); // 0
    stack.push(custom("shop", Schedule).limit(Limit::Until(t(500))), t(0), &policy, &mut events); // 1
    stack.serve(Serving::Directive(DirectiveId(0)), t(0), &policy, &mut events);
    run(&mut stack, &mut events, 0, 30); // wander served 30
    stack.push(custom("talk", Quest), t(30), &policy, &mut events); // 2
    stack.serve(Serving::Top, t(30), &policy, &mut events);
    run(&mut stack, &mut events, 30, 90); // the quest is served; the wander waits
    stack.finish(DirectiveId(2), t(90), &policy, &mut events);
    stack.serve(Serving::Directive(DirectiveId(0)), t(90), &policy, &mut events);
    run(&mut stack, &mut events, 90, 100); // wander served 40
    stack.serve(Serving::Suspended, t(100), &policy, &mut events);
    run(&mut stack, &mut events, 100, 200); // combat: nothing counts
    stack.serve(Serving::Top, t(200), &policy, &mut events);
    // The schedule's window (top) is served now; the wander still has 20 minutes left and
    // counts again once the window closes at 500: it expires at 520, repeats as dir3, which
    // expires at 580 (dir4) and 640 (dir5), which has 50 minutes at 690.
    run(&mut stack, &mut events, 200, 690);
    (stack, events)
}

#[test]
fn limits_expire_on_the_logical_clock_at_their_exact_time() {
    let (stack, events) = timed_run(1);
    assert_eq!(
        events,
        [
            done(2, DirectiveEnd::Completed, 90),
            done(1, DirectiveEnd::Expired, 500),
            done(0, DirectiveEnd::Expired, 520),
            done(3, DirectiveEnd::Expired, 580),
            done(4, DirectiveEnd::Expired, 640),
        ]
    );
    assert_eq!(ids(&stack), [5]);
    assert_eq!(stack.top().unwrap().served, 50);
    assert_eq!(stack.clock(), t(690));
}

#[test]
fn coarse_steps_give_the_same_stack_and_events_as_fine_ones() {
    // Tier invariance: the same run at 1-, 7- and 1000-minute steps.
    let fine = timed_run(1);
    assert_eq!(timed_run(7), fine);
    assert_eq!(timed_run(1000), fine);
}

#[test]
fn an_until_in_the_past_expires_at_the_clock_and_ties_end_top_first() {
    let mut stack = DirectiveStack::new(NPC, t(50));
    let mut events = Vec::new();
    stack.push(custom("a", DirectiveSource::Script).limit(Limit::Until(t(80))), t(50), &ModernPolicy, &mut events);
    stack.push(custom("b", DirectiveSource::Quest).limit(Limit::Until(t(80))), t(50), &ModernPolicy, &mut events);
    stack.advance(t(60), &ModernPolicy, &mut events);
    assert!(events.is_empty());
    // Pushed with its deadline already behind the clock: ends at the push's time.
    stack.push(custom("late", DirectiveSource::Record).limit(Limit::Until(t(10))), t(70), &ModernPolicy, &mut events);
    // A clock reading from the past is ignored.
    stack.advance(t(65), &ModernPolicy, &mut events);
    assert_eq!(stack.clock(), t(70));
    stack.advance(t(80), &ModernPolicy, &mut events);
    assert_eq!(
        events,
        [
            done(2, DirectiveEnd::Expired, 70),
            done(1, DirectiveEnd::Expired, 80),
            done(0, DirectiveEnd::Expired, 80),
        ]
    );
    assert!(stack.is_empty());
    // A repeating Until never repeats (its copy would expire at once).
    stack.push(custom("r", DirectiveSource::Record).limit(Limit::Until(t(90))).repeating(), t(80), &ModernPolicy, &mut events);
    stack.advance(t(100), &ModernPolicy, &mut events);
    assert!(stack.is_empty());
    assert_eq!(events.last(), Some(&done(3, DirectiveEnd::Expired, 90)));
}

#[test]
fn a_finish_after_the_deadline_finds_it_already_expired() {
    let mut stack = DirectiveStack::new(NPC, t(0));
    let mut events = Vec::new();
    let id =
        stack.push(custom("x", DirectiveSource::Script).limit(Limit::For { minutes: 10 }), t(0), &ModernPolicy, &mut events);
    assert!(!stack.finish(id, t(15), &ModernPolicy, &mut events));
    assert_eq!(events, [done(0, DirectiveEnd::Expired, 10)]);
}

#[test]
fn the_stack_round_trips_through_the_save_codec_and_carries_on_identically() {
    let policy = ModernPolicy;
    let mut stack = DirectiveStack::new(NPC, t(10));
    let mut events = Vec::new();
    let kinds = [
        DirectiveKind::StayNear { home: place("a"), radius: 2.5 },
        DirectiveKind::GoTo(place("b")),
        DirectiveKind::Follow(OTHER),
        DirectiveKind::Escort { who: OTHER, to: place("c") },
        DirectiveKind::Use(PlacementId(u128::MAX)),
        DirectiveKind::Fight(OTHER),
        DirectiveKind::Custom(RecordId::new("dance")),
    ];
    for (i, kind) in kinds.into_iter().enumerate() {
        let spec = DirectiveSpec::new(kind, DirectiveSource::Script)
            .priority(u8::try_from(i).unwrap())
            .limit(if i % 2 == 0 { Limit::For { minutes: 30 } } else { Limit::Until(t(2000)) })
            .origin("script_x");
        stack.push(spec, t(10), &policy, &mut events);
    }
    stack.finish(DirectiveId(1), t(12), &policy, &mut events);
    stack.serve(Serving::Directive(DirectiveId(4)), t(15), &policy, &mut events);
    stack.advance(t(20), &policy, &mut events);

    // The saved form (AI5: `sf/ai` keys it by actor).
    let bytes = postcard::to_stdvec(&stack.to_saved()).unwrap();
    let mut loaded = DirectiveStack::from_saved(NPC, postcard::from_bytes(&bytes).unwrap()).unwrap();
    assert_eq!(loaded, stack);
    let json = serde_json::to_string(&stack.to_saved()).unwrap();
    assert_eq!(DirectiveStack::from_saved(NPC, serde_json::from_str(&json).unwrap()).unwrap(), stack);

    // The same future from the original and the loaded copy: ids continue, timers resume.
    let mut a = Vec::new();
    let mut b = Vec::new();
    for s in [(&mut stack, &mut a), (&mut loaded, &mut b)] {
        let (st, ev) = s;
        let id = st.push(custom("after_load", DirectiveSource::Quest), t(25), &policy, ev);
        assert_eq!(id, DirectiveId(7));
        st.advance(t(100), &policy, ev);
    }
    assert_eq!(a, b);
    assert_eq!(loaded, stack);
    // dir4 stays served (named) under the quest: its 30 minutes, counted from 15, end at 45;
    // dir6 kept the 5 minutes it was served at the top before that.
    assert_eq!(a, [done(4, DirectiveEnd::Expired, 45)]);
    assert_eq!(loaded.get(DirectiveId(6)).unwrap().served, 5);
    assert_eq!(loaded.serving().unwrap().id, DirectiveId(7));
}

#[test]
fn every_issued_id_ends_exactly_once() {
    let policy = Double::default();
    let mut stack = DirectiveStack::new(NPC, t(0));
    let mut events = Vec::new();
    for i in 0..6u8 {
        let spec = custom("c", DirectiveSource::Record).priority(i % 3).repeating();
        let spec = if i == 4 { spec.origin("refuse") } else { spec };
        stack.push(spec, t(u32::from(i)), &policy, &mut events);
    }
    stack.finish(DirectiveId(2), t(10), &policy, &mut events);
    stack.push(custom("t", DirectiveSource::Script).origin("takeover"), t(11), &policy, &mut events);
    stack.cancel(DirectiveId(7), t(12), &policy, &mut events);
    let mut ended: Vec<u32> = events.iter().map(|e| e.id.0).collect();
    ended.sort_unstable();
    assert_eq!(ended, [0, 1, 2, 3, 4, 5, 6, 7]);
    assert!(stack.is_empty());
}

#[test]
fn the_console_listing_is_exact() {
    let mut stack = DirectiveStack::new(NPC, t(1440 + 8 * 60));
    let mut events = Vec::new();
    let now = t(1440 + 8 * 60);
    stack.push(custom("wander", DirectiveSource::Record).limit(Limit::For { minutes: 120 }).repeating(), now, &ModernPolicy, &mut events);
    stack.push(
        DirectiveSpec::new(DirectiveKind::GoTo(place("town")), DirectiveSource::Quest)
            .priority(10)
            .limit(Limit::Until(t(2 * 1440 + 6 * 60 + 5)))
            .origin("main_quest"),
        now,
        &ModernPolicy,
        &mut events,
    );
    stack.serve(Serving::Directive(DirectiveId(0)), now, &ModernPolicy, &mut events);
    stack.advance(t(1440 + 8 * 60 + 30), &ModernPolicy, &mut events);
    assert_eq!(
        format_directives(&stack),
        "dir1 quest p10 go_to until d2 06:05 from main_quest\n\
         dir0 record p0 custom wander for 30/120 min repeat [serving]\n\
         clock d1 08:30, serving dir0, 2 directive(s)\n"
    );
}
