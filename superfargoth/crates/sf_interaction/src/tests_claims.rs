//! `IN0a` claims core: transitions, leases, holder-only occupy, priority and preemption, exclusion
//! groups, queues, `UserBusy`, hand claims, the sorted batch, every invalidation reason; defs serde.
#![allow(clippy::many_single_char_names)] // a, b, s, w: users and slots in short scenarios

use std::collections::BTreeMap;

use glam::{Quat, Vec3};
use sf_ids::{ActorKey, PlayerKey};
use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::smallvec;

use crate::*;

/// 10 Hz keeps the numbers small: lease 10 s = 100 ticks, ticket lease 30 s = 300 ticks.
fn params() -> InteractParams {
    InteractParams { tick_hz: 10.0, ..InteractParams::default() }
}

fn claims() -> Claims {
    Claims::new(params())
}

const T0: Tick = Tick(0);
const LEASE: u64 = 100;
const TICKET_LEASE: u64 = 300;

fn actor(n: u128) -> UserKey {
    UserKey::Actor(ActorKey(n))
}

fn player(n: u128) -> UserKey {
    UserKey::Player(PlayerKey(n))
}

fn slot(p: u128, i: u8) -> SlotRef {
    SlotRef { placement: PlacementId(p), slot: i }
}

fn terms() -> SlotTerms {
    SlotTerms { group: None, queue: None, reach: 0.0, users: TagFilter::default(), enabled: true }
}

fn queued(cap: u8) -> SlotTerms {
    SlotTerms { queue: Some(cap), ..terms() }
}

fn grouped(mask: u8) -> SlotTerms {
    SlotTerms { group: Some(mask), ..terms() }
}

fn kinds(c: &Claims, user: UserKey) -> Vec<EventKind> {
    c.events().iter().filter(|e| e.user == user).map(|e| e.kind).collect()
}

fn phase(c: &Claims, s: SlotRef) -> Option<Phase> {
    c.claim_on(s).map(|c| c.phase)
}

#[test]
fn free_reserved_occupied_free() {
    let mut c = claims();
    let (s, a) = (slot(1, 0), actor(1));
    assert_eq!(c.claim(s, &terms(), a, &[], 10, T0), Ok(()));
    assert_eq!(phase(&c, s), Some(Phase::Reserved));
    assert_eq!(c.claim_on(s).unwrap().lease_until, Some(Tick(LEASE)));
    assert_eq!(c.hold(a), Some(Hold::Claim(s)));
    assert_eq!(c.occupy(s, a, Tick(5)), Ok(()));
    assert_eq!(phase(&c, s), Some(Phase::Occupied));
    assert_eq!(c.claim_on(s).unwrap().lease_until, None);
    assert!(c.release(a, Tick(9)));
    assert_eq!(c.claim_on(s), None);
    assert_eq!(c.hold(a), None);
    assert_eq!(kinds(&c, a), [EventKind::Claimed, EventKind::Entered, EventKind::Released]);
    assert!(!c.release(a, Tick(10)), "a second release has nothing to end");
}

#[test]
fn reserved_release_frees() {
    let mut c = claims();
    let (s, a) = (slot(1, 0), actor(1));
    c.claim(s, &terms(), a, &[], 10, T0).unwrap();
    assert!(c.release(a, Tick(1)));
    assert_eq!(c.claim_on(s), None);
    assert_eq!(kinds(&c, a), [EventKind::Claimed, EventKind::Released]);
}

#[test]
fn lease_expires_and_renew_extends() {
    let mut c = claims();
    let (s, a) = (slot(1, 0), actor(1));
    c.claim(s, &terms(), a, &[], 10, T0).unwrap();
    c.tick(Tick(LEASE - 1));
    assert_eq!(phase(&c, s), Some(Phase::Reserved));
    assert!(c.renew(a, Tick(50)));
    assert_eq!(c.claim_on(s).unwrap().lease_until, Some(Tick(50 + LEASE)));
    c.tick(Tick(LEASE));
    assert_eq!(phase(&c, s), Some(Phase::Reserved), "renewed past the first deadline");
    c.tick(Tick(50 + LEASE));
    assert_eq!(c.claim_on(s), None);
    assert_eq!(c.hold(a), None);
    assert_eq!(kinds(&c, a), [EventKind::Claimed, EventKind::Invalidated(InvalidReason::LeaseExpired)]);
    assert!(!c.renew(a, Tick(200)));
}

#[test]
fn occupied_has_no_lease() {
    let mut c = claims();
    let (s, a) = (slot(1, 0), actor(1));
    c.claim(s, &terms(), a, &[], 10, T0).unwrap();
    c.occupy(s, a, Tick(1)).unwrap();
    c.tick(Tick(100_000));
    assert_eq!(phase(&c, s), Some(Phase::Occupied));
}

#[test]
fn only_the_holder_occupies() {
    let mut c = claims();
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    assert_eq!(c.occupy(s, a, T0), Err(Refusal::NotHolder), "free slot");
    c.claim(s, &terms(), a, &[], 10, T0).unwrap();
    assert_eq!(c.occupy(s, b, T0), Err(Refusal::NotHolder));
    assert_eq!(phase(&c, s), Some(Phase::Reserved));
    assert_eq!(c.occupy(s, a, T0), Ok(()));
    assert_eq!(c.occupy(s, a, T0), Ok(()), "occupying again is a no-op");
    assert_eq!(kinds(&c, a).iter().filter(|k| **k == EventKind::Entered).count(), 1);
}

#[test]
fn higher_priority_preempts_reserved_without_queue() {
    let mut c = claims();
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    c.claim(s, &terms(), a, &[], 10, T0).unwrap();
    assert_eq!(c.claim(s, &terms(), b, &[], 20, Tick(1)), Ok(()));
    assert_eq!(c.claim_on(s).unwrap().user, b);
    assert_eq!(c.hold(a), None);
    assert_eq!(kinds(&c, a), [EventKind::Claimed, EventKind::Invalidated(InvalidReason::Preempted)]);
    assert_eq!(kinds(&c, b), [EventKind::Claimed]);
}

#[test]
fn preempted_user_requeued_at_the_head() {
    let mut c = claims();
    let (s, a, b, w) = (slot(1, 0), actor(1), actor(2), actor(3));
    c.claim(s, &queued(3), a, &[], 10, T0).unwrap();
    assert_eq!(c.enqueue(s, &queued(3), w, &[], 10, Tick(1)), Ok(Granted::Queued { position: 0 }));
    c.claim(s, &queued(3), b, &[], 20, Tick(2)).unwrap();
    // A later, higher waiter (below b, so it queues instead of preempting) still sorts behind the head.
    let x = actor(4);
    assert_eq!(c.enqueue(s, &queued(3), x, &[], 15, Tick(3)), Ok(Granted::Queued { position: 1 }));
    let q = c.queue(s);
    assert_eq!(q.iter().map(|t| t.user).collect::<Vec<_>>(), [a, x, w], "preempted head ahead of a higher waiter");
    assert!(q[0].head);
    assert_eq!(q[0].lease_until, Tick(2 + TICKET_LEASE));
    assert_eq!(c.hold(a), Some(Hold::Ticket(s)));
    assert_eq!(kinds(&c, a), [EventKind::Claimed, EventKind::Queued]);
}

#[test]
fn equal_or_lower_priority_refused() {
    let mut c = claims();
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    c.claim(s, &terms(), a, &[], 20, T0).unwrap();
    assert_eq!(c.claim(s, &terms(), b, &[], 20, T0), Err(Refusal::Taken));
    assert_eq!(c.claim(s, &terms(), b, &[], 10, T0), Err(Refusal::Taken));
    assert_eq!(c.claim_on(s).unwrap().user, a);
    assert_eq!(c.hold(b), None);
}

#[test]
fn occupied_never_preempted() {
    let mut c = claims();
    let (s, a, p) = (slot(1, 0), actor(1), player(1));
    c.claim(s, &terms(), a, &[], 10, T0).unwrap();
    c.occupy(s, a, T0).unwrap();
    assert_eq!(c.claim(s, &terms(), p, &[], u8::MAX, T0), Err(Refusal::Taken));
    assert_eq!(phase(&c, s), Some(Phase::Occupied));
}

/// A bench: left end `0b01`, right end `0b10`, middle `0b11`.
const LEFT: u8 = 0b01;
const RIGHT: u8 = 0b10;
const MIDDLE: u8 = 0b11;

#[test]
fn group_conflicts() {
    let mut c = claims();
    let (l, r, m, free) = (slot(1, 0), slot(1, 1), slot(1, 2), slot(1, 3));
    c.claim(l, &grouped(LEFT), actor(1), &[], 10, T0).unwrap();
    assert_eq!(c.claim(r, &grouped(RIGHT), actor(2), &[], 10, T0), Ok(()), "the ends don't exclude each other");
    assert_eq!(c.claim(m, &grouped(MIDDLE), actor(3), &[], 10, T0), Err(Refusal::GroupTaken));
    assert_eq!(c.claim(free, &terms(), actor(3), &[], 10, T0), Ok(()), "an ungrouped slot is free");
    assert_eq!(c.claim(slot(2, 2), &grouped(MIDDLE), actor(4), &[], 10, T0), Ok(()), "groups are per object");
}

#[test]
fn group_preempt_only_when_all_reserved_and_lower() {
    let (l, r, m) = (slot(1, 0), slot(1, 1), slot(1, 2));
    let (a, b, z) = (actor(1), actor(2), actor(9));
    let base = |right_priority: u8, right_occupied: bool| {
        let mut c = claims();
        c.claim(l, &grouped(LEFT), a, &[], 10, T0).unwrap();
        c.claim(r, &grouped(RIGHT), b, &[], right_priority, T0).unwrap();
        if right_occupied {
            c.occupy(r, b, T0).unwrap();
        }
        c
    };
    // Both ends Reserved and lower: both preempted.
    let mut c = base(10, false);
    assert_eq!(c.claim(m, &grouped(MIDDLE), z, &[], 20, Tick(1)), Ok(()));
    assert_eq!((c.claim_on(l), c.claim_on(r)), (None, None));
    assert_eq!(c.hold(a), None);
    assert_eq!(c.hold(b), None);
    assert_eq!(kinds(&c, a).last(), Some(&EventKind::Invalidated(InvalidReason::Preempted)));
    // One end Occupied: refused, nothing preempted.
    let mut c = base(10, true);
    assert_eq!(c.claim(m, &grouped(MIDDLE), z, &[], 20, Tick(1)), Err(Refusal::GroupTaken));
    assert_eq!(c.claim_on(l).unwrap().user, a);
    // One end Reserved at an equal priority: refused, nothing preempted.
    let mut c = base(20, false);
    assert_eq!(c.claim(m, &grouped(MIDDLE), z, &[], 20, Tick(1)), Err(Refusal::GroupTaken));
    assert_eq!(c.claim_on(l).unwrap().user, a);
    assert_eq!(c.claim_on(r).unwrap().user, b);
}

#[test]
fn group_blocked_queue_promotes_when_the_group_frees() {
    let mut c = claims();
    let (l, m) = (slot(1, 0), slot(1, 2));
    let (a, w) = (actor(1), actor(2));
    c.claim(l, &grouped(LEFT), a, &[], 10, T0).unwrap();
    c.occupy(l, a, T0).unwrap();
    let mid = SlotTerms { queue: Some(2), ..grouped(MIDDLE) };
    assert_eq!(c.enqueue(m, &mid, w, &[], 10, Tick(1)), Ok(Granted::Queued { position: 0 }));
    assert!(c.release(a, Tick(5)));
    assert_eq!(c.claim_on(m).map(|c| (c.user, c.phase)), Some((w, Phase::Reserved)));
    assert_eq!(kinds(&c, w), [EventKind::Queued, EventKind::Promoted]);
}

#[test]
fn queue_order_priority_since_user() {
    let mut c = claims();
    let s = slot(1, 0);
    let t = queued(5);
    c.claim(s, &t, actor(100), &[], 30, T0).unwrap();
    c.enqueue(s, &t, actor(10), &[], 10, Tick(1)).unwrap();
    c.enqueue(s, &t, actor(20), &[], 20, Tick(2)).unwrap();
    c.enqueue(s, &t, actor(5), &[], 10, Tick(2)).unwrap();
    assert_eq!(c.enqueue(s, &t, actor(4), &[], 10, Tick(2)), Ok(Granted::Queued { position: 2 }));
    let order: Vec<UserKey> = c.queue(s).iter().map(|t| t.user).collect();
    assert_eq!(order, [actor(20), actor(10), actor(4), actor(5)]);
}

#[test]
fn queue_full_no_queue_and_players() {
    let mut c = claims();
    let s = slot(1, 0);
    c.claim(s, &queued(2), actor(1), &[], 10, T0).unwrap();
    c.occupy(s, actor(1), T0).unwrap();
    c.enqueue(s, &queued(2), actor(2), &[], 10, T0).unwrap();
    c.enqueue(s, &queued(2), actor(3), &[], 10, T0).unwrap();
    assert_eq!(c.enqueue(s, &queued(2), actor(4), &[], 10, T0), Err(Refusal::QueueFull));
    assert_eq!(c.hold(actor(4)), None);
    let s0 = slot(2, 0);
    c.claim(s0, &queued(0), actor(5), &[], 10, T0).unwrap();
    assert_eq!(c.enqueue(s0, &queued(0), actor(6), &[], 10, T0), Err(Refusal::QueueFull));
    let none = slot(3, 0);
    c.claim(none, &terms(), actor(7), &[], 10, T0).unwrap();
    assert_eq!(c.enqueue(none, &terms(), actor(8), &[], 10, T0), Err(Refusal::NoQueue));
    assert_eq!(c.enqueue(s, &queued(5), player(1), &[], 30, T0), Err(Refusal::NoQueue), "players never queue");
    assert_eq!(c.enqueue(slot(4, 0), &queued(5), player(1), &[], 30, T0), Ok(Granted::Claimed), "a free slot claims");
}

#[test]
fn head_promoted_on_release_same_tick() {
    let mut c = claims();
    let (s, a, b, d) = (slot(1, 0), actor(1), actor(2), actor(3));
    c.claim(s, &queued(3), a, &[], 10, T0).unwrap();
    c.occupy(s, a, T0).unwrap();
    c.enqueue(s, &queued(3), b, &[], 20, Tick(1)).unwrap();
    c.enqueue(s, &queued(3), d, &[], 10, Tick(1)).unwrap();
    assert!(c.release(a, Tick(40)));
    let claim = c.claim_on(s).unwrap();
    assert_eq!((claim.user, claim.phase, claim.lease_until), (b, Phase::Reserved, Some(Tick(40 + LEASE))));
    assert_eq!(c.hold(b), Some(Hold::Claim(s)));
    assert_eq!(c.queue(s).iter().map(|t| t.user).collect::<Vec<_>>(), [d]);
    let promoted = c.events().iter().find(|e| e.user == b && e.kind == EventKind::Promoted).unwrap();
    assert_eq!(promoted.tick, Tick(40));
}

#[test]
fn head_promoted_on_each_user_invalidation() {
    for reason in [InvalidReason::UserDied, InvalidReason::UserKnockedDown, InvalidReason::UserLeftWorldspace] {
        let mut c = claims();
        let (s, a, b) = (slot(1, 0), actor(1), actor(2));
        c.claim(s, &queued(3), a, &[], 10, T0).unwrap();
        c.occupy(s, a, T0).unwrap();
        c.enqueue(s, &queued(3), b, &[], 10, Tick(1)).unwrap();
        c.invalidate(Subject::User(a), reason, Tick(7));
        assert_eq!(c.claim_on(s).map(|c| c.user), Some(b), "{reason:?}");
        assert_eq!(kinds(&c, b), [EventKind::Queued, EventKind::Promoted], "{reason:?}");
    }
    // Lease expiry promotes too.
    let mut c = claims();
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    c.claim(s, &queued(3), a, &[], 10, T0).unwrap();
    c.enqueue(s, &queued(3), b, &[], 10, Tick(1)).unwrap();
    c.tick(Tick(LEASE));
    assert_eq!(c.claim_on(s).map(|c| (c.user, c.lease_until)), Some((b, Some(Tick(2 * LEASE)))));
}

#[test]
fn ticket_lease_expires_and_renews() {
    let mut c = claims();
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    c.claim(s, &queued(3), a, &[], 10, T0).unwrap();
    c.occupy(s, a, T0).unwrap();
    c.enqueue(s, &queued(3), b, &[], 10, T0).unwrap();
    c.tick(Tick(TICKET_LEASE - 1));
    assert_eq!(c.queue(s).len(), 1);
    assert!(c.renew(b, Tick(100)));
    assert_eq!(c.queue(s)[0].lease_until, Tick(100 + TICKET_LEASE));
    c.tick(Tick(TICKET_LEASE));
    assert_eq!(c.queue(s).len(), 1, "renewed");
    c.tick(Tick(100 + TICKET_LEASE));
    assert!(c.queue(s).is_empty());
    assert_eq!(c.hold(b), None);
    assert_eq!(kinds(&c, b), [EventKind::Queued, EventKind::Invalidated(InvalidReason::LeaseExpired)]);
    assert_eq!(c.claim_on(s).map(|c| c.user), Some(a), "the holder is untouched");
}

#[test]
fn one_body_hold_per_user() {
    let mut c = claims();
    let (s1, s2, s3, a) = (slot(1, 0), slot(2, 0), slot(3, 0), actor(1));
    c.claim(s1, &queued(3), a, &[], 10, T0).unwrap();
    assert_eq!(c.claim(s2, &terms(), a, &[], 10, T0), Err(Refusal::UserBusy));
    assert_eq!(c.claim(s1, &terms(), a, &[], 50, T0), Err(Refusal::UserBusy), "no re-claim of one's own slot");
    // Holding a ticket is busy too.
    c.claim(s3, &queued(3), actor(2), &[], 10, T0).unwrap();
    c.release(a, T0);
    c.enqueue(s3, &queued(3), a, &[], 10, T0).unwrap();
    assert_eq!(c.claim(s2, &terms(), a, &[], 10, T0), Err(Refusal::UserBusy));
    assert_eq!(c.enqueue(s1, &queued(3), a, &[], 10, T0), Err(Refusal::UserBusy));
}

#[test]
fn filter_and_disabled_refusals() {
    let mut c = claims();
    let small = TagId::of("small");
    let t = SlotTerms { users: TagFilter { all: smallvec![small], ..TagFilter::default() }, ..terms() };
    assert_eq!(c.claim(slot(1, 0), &t, actor(1), &[], 10, T0), Err(Refusal::Filter));
    assert_eq!(c.claim(slot(1, 0), &t, actor(1), &[small], 10, T0), Ok(()));
    let off = SlotTerms { enabled: false, ..terms() };
    assert_eq!(c.claim(slot(2, 0), &off, actor(2), &[], 10, T0), Err(Refusal::Disabled));
}

#[test]
fn hand_claims_within_reach_while_occupied() {
    let mut c = claims();
    let seat = SlotTerms { reach: 1.0, ..terms() };
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    let (cup, plate, bowl) = (PlacementId(50), PlacementId(51), PlacementId(52));
    assert_eq!(c.hand_claim(a, cup, 0.5), Err(Refusal::NotOccupied), "no body claim");
    c.claim(s, &seat, a, &[], 10, T0).unwrap();
    assert_eq!(c.hand_claim(a, cup, 0.5), Err(Refusal::NotOccupied), "only Reserved");
    c.occupy(s, a, T0).unwrap();
    assert_eq!(c.hand_claim(a, cup, 1.5), Err(Refusal::OutOfReach));
    assert_eq!(c.hand_claim(a, cup, 1.0), Ok(()));
    assert_eq!(c.hand_claim(a, cup, 1.0), Ok(()), "holding it already");
    assert_eq!(c.hand_claim(a, plate, 0.2), Ok(()));
    assert_eq!(c.hand_claim(a, bowl, 0.2), Err(Refusal::HandsFull), "hand_max 2");
    // Another seated user can't take a held prop; a seat without reach takes none.
    let s2 = slot(2, 0);
    c.claim(s2, &terms(), b, &[], 10, T0).unwrap();
    c.occupy(s2, b, T0).unwrap();
    assert_eq!(c.hand_claim(b, cup, 0.1), Err(Refusal::Taken));
    assert_eq!(c.hand_claim(b, bowl, 0.0), Err(Refusal::OutOfReach));
    // Put-back, then the body claim ends every hand claim.
    assert!(c.hand_release(a, cup));
    assert!(!c.hand_release(a, cup));
    assert_eq!(c.hands(a), [plate]);
    assert!(c.release(a, Tick(3)));
    assert!(c.hands(a).is_empty());
    assert_eq!(c.hand_holder(plate), None);
}

#[test]
fn owner_destroyed_drops_hand_claims_on_it() {
    let mut c = claims();
    let (s, a, cup) = (slot(1, 0), actor(1), PlacementId(50));
    c.claim(s, &SlotTerms { reach: 1.0, ..terms() }, a, &[], 10, T0).unwrap();
    c.occupy(s, a, T0).unwrap();
    c.hand_claim(a, cup, 0.5).unwrap();
    c.invalidate(Subject::Owner(cup), InvalidReason::OwnerDestroyed, Tick(1));
    assert!(c.hands(a).is_empty());
    assert_eq!(phase(&c, s), Some(Phase::Occupied), "the seat stays");
}

fn batch() -> Vec<Request> {
    let t = queued(2);
    let claim = |slot, t: &SlotTerms| Op::Claim { slot, terms: t.clone(), tags: smallvec![] };
    let enqueue = |slot, t: &SlotTerms| Op::Enqueue { slot, terms: t.clone(), tags: smallvec![] };
    vec![
        Request { user: actor(3), priority: 10, op: claim(slot(1, 0), &t) },
        Request { user: actor(1), priority: 10, op: enqueue(slot(1, 0), &t) },
        Request { user: player(1), priority: 30, op: claim(slot(1, 0), &t) },
        Request { user: actor(2), priority: 20, op: enqueue(slot(1, 0), &t) },
        Request { user: actor(4), priority: 10, op: claim(slot(1, 1), &grouped(LEFT)) },
        Request { user: actor(5), priority: 20, op: claim(slot(1, 2), &grouped(MIDDLE)) },
        Request { user: actor(6), priority: 10, op: Op::Release },
        Request { user: actor(7), priority: 15, op: enqueue(slot(1, 0), &t) },
    ]
}

fn run(order: &[usize]) -> (Claims, BTreeMap<UserKey, Result<Granted, Refusal>>) {
    let reqs = batch();
    let permuted: Vec<Request> = order.iter().map(|&i| reqs[i].clone()).collect();
    let mut c = claims();
    let out = c.apply_batch(&permuted, Tick(3));
    let by_user = permuted.iter().zip(out).map(|(r, o)| (r.user, o)).collect();
    (c, by_user)
}

#[test]
fn permuted_batch_gives_the_same_result() {
    let base = run(&[0, 1, 2, 3, 4, 5, 6, 7]);
    for order in [[7, 6, 5, 4, 3, 2, 1, 0], [3, 0, 6, 2, 7, 1, 5, 4], [5, 2, 7, 0, 4, 6, 1, 3]] {
        let other = run(&order);
        assert_eq!(other.1, base.1, "{order:?}");
        assert_eq!(other.0, base.0, "{order:?}");
    }
    // The player (30) wins the seat; then 20s, then 15, then 10s by UserKey.
    let r = &base.1;
    assert_eq!(r[&player(1)], Ok(Granted::Claimed));
    assert_eq!(r[&actor(5)], Ok(Granted::Claimed));
    assert_eq!(r[&actor(2)], Ok(Granted::Queued { position: 0 }));
    assert_eq!(r[&actor(7)], Ok(Granted::Queued { position: 1 }));
    assert_eq!(r[&actor(1)], Err(Refusal::QueueFull));
    assert_eq!(r[&actor(3)], Err(Refusal::Taken));
    assert_eq!(r[&actor(4)], Err(Refusal::GroupTaken));
    assert_eq!(r[&actor(6)], Err(Refusal::NoHold));
}

/// A Reserved holder with a queued waiter; returns (claims, slot, holder, waiter).
fn held_with_waiter() -> (Claims, SlotRef, UserKey, UserKey) {
    let mut c = claims();
    let (s, a, b) = (slot(1, 0), actor(1), actor(2));
    c.claim(s, &queued(3), a, &[], 10, T0).unwrap();
    c.enqueue(s, &queued(3), b, &[], 10, Tick(1)).unwrap();
    (c, s, a, b)
}

fn count(c: &Claims, user: UserKey, reason: InvalidReason) -> usize {
    c.events().iter().filter(|e| e.user == user && e.kind == EventKind::Invalidated(reason)).count()
}

#[test]
fn each_invalidation_reason_releases_once() {
    use InvalidReason as R;
    let all = [
        R::Preempted,
        R::LeaseExpired,
        R::OwnerDestroyed,
        R::OwnerMoved,
        R::OwnerUnloaded,
        R::SlotDisabled,
        R::UserDied,
        R::UserKnockedDown,
        R::UserLeftWorldspace,
    ];
    for reason in all {
        let (mut c, s, a, b) = if reason == R::Preempted {
            let mut c = claims();
            let (s, a) = (slot(1, 0), actor(1));
            c.claim(s, &terms(), a, &[], 10, T0).unwrap();
            (c, s, a, actor(2))
        } else {
            held_with_waiter()
        };
        let fire = |c: &mut Claims, now: Tick| match reason {
            R::Preempted => {
                let _ = c.claim(s, &terms(), actor(9), &[], 50, now);
            }
            R::LeaseExpired => c.tick(now.plus(LEASE)),
            R::OwnerDestroyed | R::OwnerUnloaded => c.invalidate(Subject::Owner(s.placement), reason, now),
            R::OwnerMoved => {
                c.owner_moved(s.placement, 1.0, 0.0, now);
            }
            R::SlotDisabled => c.invalidate(Subject::Slot(s), reason, now),
            R::UserDied | R::UserKnockedDown | R::UserLeftWorldspace => c.invalidate(Subject::User(a), reason, now),
        };
        fire(&mut c, Tick(2));
        assert_eq!(count(&c, a, reason), 1, "{reason:?}");
        assert_ne!(c.claim_on(s).map(|c| c.user), Some(a), "{reason:?}");
        assert_eq!(c.hold(a), None, "{reason:?}");
        let events = c.events().len();
        fire(&mut c, Tick(3));
        assert_eq!(count(&c, a, reason), 1, "{reason:?}: a second invalidation ends nothing of a's");
        let slot_scoped = matches!(reason, R::OwnerDestroyed | R::OwnerMoved | R::OwnerUnloaded | R::SlotDisabled);
        if slot_scoped {
            assert_eq!(count(&c, b, reason), 1, "{reason:?}: the slot's tickets end too");
            assert!(c.queue(s).is_empty() && c.claim_on(s).is_none(), "{reason:?}");
            assert_eq!(c.events().len(), events, "{reason:?}: nothing left to invalidate");
        } else if reason != R::Preempted {
            assert_eq!(kinds(&c, b)[..2], [EventKind::Queued, EventKind::Promoted], "{reason:?}");
        }
    }
}

#[test]
fn move_within_tolerance_keeps_the_claim() {
    let (mut c, s, a, _) = held_with_waiter();
    assert!(!c.owner_moved(s.placement, 0.25, 20.0, Tick(2)), "at the tolerance");
    assert_eq!(c.claim_on(s).map(|c| c.user), Some(a));
    assert!(c.events().iter().all(|e| !matches!(e.kind, EventKind::Invalidated(_))));
    assert!(c.owner_moved(s.placement, 0.0, 21.0, Tick(3)), "turned too far");
    assert_eq!(count(&c, a, InvalidReason::OwnerMoved), 1);
}

#[test]
fn defs_serde_round_trip_every_slot_kind() {
    let verb = |v: &str| VerbDef { verb: TagId::of(v), priority: 1, requires: TagFilter::default() };
    let pose_slot = |kind: SlotKind, v: &str| SlotDef {
        kind,
        local: Frame { translation: Vec3::new(0.0, 0.45, 0.1), rotation: Quat::from_rotation_y(0.5) },
        band: 0.45,
        verb: verb(v),
        entrances: smallvec![
            Entrance { side: Side::Front, approach: Vec3::new(0.0, 0.0, 0.6) },
            Entrance { side: Side::Left, approach: Vec3::new(-0.6, 0.0, 0.0) },
        ],
        clips: ClipFamily("seat.chair".into()),
        ik: IkTargets { hands: smallvec![Vec3::X, Vec3::NEG_X], feet: smallvec![Vec3::ZERO], look: Some(Vec3::Z) },
        reach: 0.8,
        occupancy: Occupancy::Pose,
        control: Some(Control::Passenger),
        users: TagFilter { all: smallvec![TagId::of("biped")], any: smallvec![], none: smallvec![TagId::of("huge")] },
        group: Some(MIDDLE),
        queue: Some(2),
    };
    let def = InteractableDef {
        id: RecordId::new("bench_tavern"),
        slots: vec![
            pose_slot(SlotKind::Pose { family: TagId::of("seat") }, "sit"),
            pose_slot(
                SlotKind::Door(DoorDef {
                    hinge: Vec3::new(0.5, 0.0, 0.0),
                    axis: Vec3::Y,
                    open_deg: 95.0,
                    both_ways: true,
                    close_behind: true,
                }),
                "open",
            ),
            pose_slot(
                SlotKind::Lever(LeverDef {
                    positions: 2,
                    links: smallvec![LeverLink { target: PlacementId(0xabc), signal: TagId::of("toggle") }],
                }),
                "pull",
            ),
            pose_slot(
                SlotKind::Ladder(LadderDef { chain: smallvec![Vec3::ZERO, Vec3::new(0.0, 4.0, 0.0)], rung_spacing: 0.3 }),
                "climb",
            ),
            pose_slot(
                SlotKind::Workstation(WorkstationDef {
                    jobs: TagFilter { any: smallvec![TagId::of("smith")], ..TagFilter::default() },
                    player_verb: Some(TagId::of("repair")),
                }),
                "work",
            ),
            pose_slot(SlotKind::Light(LightDef::default()), "snuff"),
        ],
        verbs: smallvec![verb("carry"), verb("break")],
        material: Some(TagId::of("wood")),
        destructible: Some(DestructibleDef {
            health: 40.0,
            stages: smallvec![
                Stage {
                    at: 0.5,
                    model: Some(RecordId::new("bench_cracked")),
                    debris: smallvec![RecordId::new("planks")],
                    chunks: None,
                    effect: Some(RecordId::new("dust")),
                    on_break: BreakAction::None,
                },
                Stage {
                    at: 0.0,
                    model: None,
                    debris: smallvec![],
                    chunks: Some(RecordId::new("bench_chunks")),
                    effect: None,
                    on_break: BreakAction::SpillContents,
                },
            ],
            ignore_external: false,
            loot: Some(RecordId::new("lvl_junk")),
            salvage: smallvec![(RecordId::new("plank"), 3)],
        }),
        carry: Some(CarryDef {
            grips: smallvec![Grip::default(), Grip { local: Frame { translation: Vec3::X, rotation: Quat::IDENTITY } }],
            class: RecordId::new("two_hand"),
            relocatable: true,
            pairs_with: TagFilter { all: smallvec![TagId::of("table_edge")], ..TagFilter::default() },
        }),
        adverts: vec![(TagId::of("rest"), 0.6), (TagId::of("social"), 0.3)],
    };
    let text = ron::to_string(&def).unwrap();
    let back: InteractableDef = ron::from_str(&text).unwrap();
    assert_eq!(back, def);
    let class = CarryClass {
        id: RecordId::new("two_hand"),
        clips: ClipFamily("carry.two_hand".into()),
        hands: 2,
        drag: false,
        speed_mul: 0.6,
        can_sprint: false,
        can_roll: false,
    };
    assert_eq!(ron::from_str::<CarryClass>(&ron::to_string(&class).unwrap()).unwrap(), class);
}

#[test]
fn terms_fill_the_kind_default_queue() {
    let p = params();
    let mut def = SlotDef {
        kind: SlotKind::Door(DoorDef {
            hinge: Vec3::ZERO,
            axis: Vec3::Y,
            open_deg: 90.0,
            both_ways: false,
            close_behind: false,
        }),
        local: Frame::default(),
        band: 0.0,
        verb: VerbDef { verb: TagId::of("open"), priority: 0, requires: TagFilter::default() },
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
    assert_eq!(SlotTerms::of(&def, &p).queue, Some(p.queue_capacity));
    def.queue = Some(0);
    assert_eq!(SlotTerms::of(&def, &p).queue, Some(0), "the def overrides");
    def.queue = None;
    def.kind = SlotKind::Light(LightDef::default());
    assert_eq!(SlotTerms::of(&def, &p).queue, None, "lights don't queue");
}
