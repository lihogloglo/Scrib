//! `IN17a` object states: add / has / refresh / remove, TTL expiry at the exact tick, the same table
//! whatever order the operations ran in, and the def's `material` field (moved up from
//! `DestructibleDef`) through serde.

use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::smallvec;

use crate::*;

const BURNING: TagId = TagId::of("burning");
const WET: TagId = TagId::of("wet");
const CHARRED: TagId = TagId::of("charred");
const P1: PlacementId = PlacementId(1);
const P2: PlacementId = PlacementId(2);

/// 60 Hz, so 1 s is 60 ticks.
fn params() -> InteractParams {
    InteractParams { tick_hz: 60.0, ..InteractParams::default() }
}

#[test]
fn add_has_refresh_remove() {
    let p = params();
    let mut s = ObjectStates::new();
    assert!(!s.has(P1, BURNING));
    assert!(s.add(P1, BURNING, Ttl::Secs(1.0), Tick(10), &p), "new state is a change");
    assert!(s.has(P1, BURNING));
    assert!(!s.has(P2, BURNING), "states are per placement");
    assert_eq!(s.states(P1), &[StateEntry { tag: BURNING, expires: Some(Tick(70)) }]);

    // Refresh extends: added again at tick 40 for 1 s -> expires 100; not a change.
    assert!(!s.add(P1, BURNING, Ttl::Secs(1.0), Tick(40), &p));
    assert_eq!(s.states(P1)[0].expires, Some(Tick(100)));
    // A shorter refresh never shortens it.
    assert!(!s.add(P1, BURNING, Ttl::Secs(0.5), Tick(41), &p));
    assert_eq!(s.states(P1)[0].expires, Some(Tick(100)));
    // UntilRemoved beats any Secs, both ways.
    assert!(!s.add(P1, BURNING, Ttl::UntilRemoved, Tick(42), &p));
    assert_eq!(s.states(P1)[0].expires, None);
    assert!(!s.add(P1, BURNING, Ttl::Secs(9.0), Tick(43), &p));
    assert_eq!(s.states(P1)[0].expires, None);
    assert!(s.expire(Tick(10_000)).is_empty(), "the old deadlines went with the refresh");
    assert!(s.has(P1, BURNING));

    // Tags stay sorted; `tags` feeds TagFilter.
    s.add(P1, WET, Ttl::Secs(2.0), Tick(0), &p);
    let mut want = [BURNING, WET];
    want.sort();
    assert_eq!(s.tags(P1).as_slice(), want);
    assert!(TagFilter { all: smallvec![WET], none: smallvec![CHARRED], ..TagFilter::default() }.matches(&s.tags(P1)));

    assert!(s.remove(P1, BURNING));
    assert!(!s.remove(P1, BURNING), "already gone");
    assert!(!s.remove(P2, WET), "never there");
    assert!(s.remove(P1, WET));
    assert!(s.is_empty(), "a placement without states leaves the table");
    assert!(s.expire(Tick(10_000)).is_empty(), "removing a timed state drops its deadline");
}

#[test]
fn ttl_expires_at_the_exact_tick() {
    let p = params();
    let mut s = ObjectStates::new();
    s.add(P2, WET, Ttl::Secs(1.0), Tick(10), &p); // expires 70
    s.add(P1, WET, Ttl::Secs(1.0), Tick(10), &p); // expires 70
    s.add(P1, BURNING, Ttl::Secs(0.5), Tick(10), &p); // expires 40
    s.add(P1, CHARRED, Ttl::UntilRemoved, Tick(10), &p);
    // A zero or negative TTL still lasts one tick.
    s.add(P2, BURNING, Ttl::Secs(0.0), Tick(10), &p); // expires 11

    assert!(s.expire(Tick(10)).is_empty());
    assert_eq!(s.expire(Tick(11)), vec![(P2, BURNING)]);
    assert!(s.expire(Tick(39)).is_empty());
    assert_eq!(s.expire(Tick(40)), vec![(P1, BURNING)]);
    assert!(s.has(P1, WET));
    assert!(s.expire(Tick(69)).is_empty());
    // Same tick: placement order.
    assert_eq!(s.expire(Tick(70)), vec![(P1, WET), (P2, WET)]);
    assert!(!s.has(P1, WET));
    assert!(!s.has(P2, WET));
    assert!(s.has(P1, CHARRED), "UntilRemoved never expires");
    assert_eq!(s.len(), 1);

    // A late call drops everything due, in (tick, placement, tag) order.
    let mut late = ObjectStates::new();
    late.add(P2, WET, Ttl::Secs(1.0), Tick(0), &p); // 60
    late.add(P1, WET, Ttl::Secs(2.0), Tick(0), &p); // 120
    late.add(P1, BURNING, Ttl::Secs(2.0), Tick(0), &p); // 120
    let mut same_tick = [(P1, BURNING), (P1, WET)];
    same_tick.sort_by_key(|&(_, t)| t);
    let mut want = vec![(P2, WET)];
    want.extend(same_tick);
    assert_eq!(late.expire(Tick(500)), want);
    assert!(late.is_empty());
}

#[test]
fn permuted_op_order_gives_the_same_table() {
    let p = params();
    // Commuting operations (adds and refreshes, then the same removes) in every order of six.
    let ops: [(PlacementId, TagId, Ttl, u64); 6] = [
        (P1, BURNING, Ttl::Secs(1.0), 5),
        (P1, WET, Ttl::Secs(2.0), 3),
        (P2, WET, Ttl::Secs(1.0), 7),
        (P1, BURNING, Ttl::Secs(1.0), 20), // refresh: later expiry wins
        (P2, CHARRED, Ttl::UntilRemoved, 1),
        (P2, WET, Ttl::Secs(0.25), 8), // shorter refresh: no effect
    ];
    let build = |order: &[usize]| {
        let mut s = ObjectStates::new();
        for &i in order {
            let (pl, tag, ttl, now) = ops[i];
            s.add(pl, tag, ttl, Tick(now), &p);
        }
        s.remove(P1, WET);
        s
    };
    let reference = build(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(reference.states(P1), &[StateEntry { tag: BURNING, expires: Some(Tick(80)) }]);
    assert!(reference.has(P2, WET) && reference.has(P2, CHARRED));
    let mut order = [0, 1, 2, 3, 4, 5];
    let mut count = 0;
    permute(&mut order, 0, &mut |o| {
        assert_eq!(build(o), reference, "order {o:?}");
        count += 1;
    });
    assert_eq!(count, 720);
    // Same expiry lists too.
    let (mut a, mut b) = (build(&[0, 1, 2, 3, 4, 5]), build(&[5, 4, 3, 2, 1, 0]));
    assert_eq!(a.expire(Tick(1_000)), b.expire(Tick(1_000)));
}

fn permute(v: &mut [usize; 6], k: usize, f: &mut impl FnMut(&[usize])) {
    if k == v.len() {
        f(v);
        return;
    }
    for i in k..v.len() {
        v.swap(k, i);
        permute(v, k + 1, f);
        v.swap(k, i);
    }
}

#[test]
fn material_lives_on_the_def_and_round_trips() {
    let def = InteractableDef {
        id: RecordId::new("banner"),
        slots: vec![],
        verbs: smallvec![],
        material: Some(TagId::of("cloth")),
        destructible: None,
        carry: None,
        adverts: vec![],
    };
    let text = ron::to_string(&def).unwrap();
    assert_eq!(ron::from_str::<InteractableDef>(&text).unwrap(), def);
    // Data written before the move (no `material` key) loads as `None`.
    let (start, end) = (text.find("material:").unwrap(), text.find("destructible:").unwrap());
    let old = format!("{}{}", &text[..start], &text[end..]);
    let back: InteractableDef = ron::from_str(&old).unwrap();
    assert_eq!(back, InteractableDef { material: None, ..def });
    assert_eq!(ron::from_str::<Ttl>(&ron::to_string(&Ttl::Secs(2.5)).unwrap()).unwrap(), Ttl::Secs(2.5));
}
