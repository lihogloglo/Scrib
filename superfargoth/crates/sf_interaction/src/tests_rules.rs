//! `IN17b` rules evaluation: water on `Burning` gives `Wet` (and `Wet` expires), the required /
//! forbidden state and material gate, record order wins, the per-tick budget carries causes over oldest
//! first, and every arrival order of a tick's causes gives the same events and states.

use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::{SmallVec, smallvec};

use crate::*;

const BURNING: TagId = TagId::of("burning");
const WET: TagId = TagId::of("wet");
const WATER: TagId = TagId::of("water");
const FIRE: TagId = TagId::of("fire");
const WOOD: TagId = TagId::of("wood");
const STONE: TagId = TagId::of("stone");
const MARK: TagId = TagId::of("mark");
const SPLASH: TagId = TagId::of("splash");
const HEAT: TagId = TagId::of("heat");
const P0: PlacementId = PlacementId(0);
const P1: PlacementId = PlacementId(1);
const P2: PlacementId = PlacementId(2);
const P3: PlacementId = PlacementId(3);

fn params() -> InteractParams {
    InteractParams { tick_hz: 60.0, ..InteractParams::default() }
}

fn filter(all: &[TagId], any: &[TagId], none: &[TagId]) -> TagFilter {
    TagFilter { all: all.into(), any: any.into(), none: none.into() }
}

fn rule(id: &str, source: TagFilter, target: TagFilter, result: RuleResult) -> InteractionRule {
    InteractionRule { id: RecordId::new(id), source, target, result }
}

/// Water on `Burning`: remove `Burning`, add `Wet` for 2 s, splash effect.
fn douse() -> InteractionRule {
    rule(
        "douse",
        filter(&[], &[WATER], &[]),
        filter(&[BURNING], &[], &[]),
        RuleResult { remove: smallvec![BURNING], add: smallvec![(WET, Ttl::Secs(2.0))], effect: Some(SPLASH), ..RuleResult::default() },
    )
}

/// Fire on dry wood: `Burning` until removed, heat damage 5.
fn ignite() -> InteractionRule {
    rule(
        "ignite",
        filter(&[FIRE], &[], &[]),
        filter(&[WOOD], &[], &[WET]),
        RuleResult { add: smallvec![(BURNING, Ttl::UntilRemoved)], damage: Some((HEAT, 5.0)), ..RuleResult::default() },
    )
}

fn cause(placement: PlacementId, material: Option<TagId>, source: &[TagId]) -> Cause {
    Cause { placement, material, source: SmallVec::from_slice(source) }
}

fn ev(placement: PlacementId, rule: Option<&str>, output: RuleOutput) -> RuleEvent {
    RuleEvent { placement, rule: rule.map(RecordId::new), output }
}

#[test]
fn water_on_burning_turns_wet_then_wet_expires() {
    let p = params();
    let rules = Rules::new(vec![douse()]);
    let mut s = ObjectStates::new();
    s.add(P1, BURNING, Ttl::UntilRemoved, Tick(0), &p);
    let mut q = RuleQueue::new();
    q.push(Tick(10), cause(P1, Some(WOOD), &[WATER]));
    let out = step_rules(&mut s, &mut q, &rules, Tick(10), &p);
    assert_eq!(
        out,
        vec![
            ev(P1, Some("douse"), RuleOutput::Removed(BURNING)),
            ev(P1, Some("douse"), RuleOutput::Added(WET)),
            ev(P1, Some("douse"), RuleOutput::Effect(SPLASH)),
        ]
    );
    assert!(q.is_empty());
    assert_eq!(s.states(P1), &[StateEntry { tag: WET, expires: Some(Tick(130)) }]);

    // Water again on a target no longer burning: no rule matches, nothing happens.
    q.push(Tick(11), cause(P1, Some(WOOD), &[WATER]));
    assert!(step_rules(&mut s, &mut q, &rules, Tick(11), &p).is_empty());
    // 2 s at 60 Hz after tick 10: `Wet` expires exactly at tick 130, reported before any cause.
    assert!(step_rules(&mut s, &mut q, &rules, Tick(129), &p).is_empty());
    assert_eq!(step_rules(&mut s, &mut q, &rules, Tick(130), &p), vec![ev(P1, None, RuleOutput::Expired(WET))]);
    assert!(s.is_empty());

    // The record through serde, and a sparse record (mod data) with every result field defaulted.
    let r = douse();
    assert_eq!(ron::from_str::<InteractionRule>(&ron::to_string(&r).unwrap()).unwrap(), r);
    let sparse: InteractionRule = ron::from_str(r#"(id: "x", result: (stage: Some(2)))"#).unwrap();
    assert_eq!(sparse.result, RuleResult { stage: Some(2), ..RuleResult::default() });
    assert_eq!(sparse.source, TagFilter::default());
}

#[test]
fn required_and_forbidden_states_and_material_gate() {
    let p = params();
    let rules = Rules::new(vec![ignite()]);
    let mut s = ObjectStates::new();
    s.add(P2, WET, Ttl::UntilRemoved, Tick(0), &p);
    let mut q = RuleQueue::new();
    q.push(Tick(1), cause(P1, Some(WOOD), &[FIRE])); // dry wood: ignites
    q.push(Tick(1), cause(P2, Some(WOOD), &[FIRE])); // wet wood: forbidden state
    q.push(Tick(1), cause(P3, Some(STONE), &[FIRE])); // stone: not the required material
    q.push(Tick(1), cause(P0, None, &[FIRE])); // no material
    q.push(Tick(1), cause(P1, Some(WOOD), &[WATER])); // wrong source
    let out = step_rules(&mut s, &mut q, &rules, Tick(1), &p);
    assert_eq!(
        out,
        vec![
            ev(P1, Some("ignite"), RuleOutput::Added(BURNING)),
            ev(P1, Some("ignite"), RuleOutput::Damage { kind: HEAT, amount: 5.0 }),
        ]
    );
    assert!(s.has(P1, BURNING));
    assert!(!s.has(P2, BURNING) && s.has(P2, WET));
    assert!(!s.has(P3, BURNING) && !s.has(P0, BURNING));

    // Fire again on a burning target: the add is a refresh (no state event), the damage repeats.
    q.push(Tick(2), cause(P1, Some(WOOD), &[FIRE]));
    assert_eq!(
        step_rules(&mut s, &mut q, &rules, Tick(2), &p),
        vec![ev(P1, Some("ignite"), RuleOutput::Damage { kind: HEAT, amount: 5.0 })]
    );
}

#[test]
fn record_order_wins() {
    let params = params();
    let mark = |id: &str, tag: TagId| {
        rule(id, filter(&[FIRE], &[], &[]), TagFilter::default(), RuleResult { add: smallvec![(tag, Ttl::UntilRemoved)], ..RuleResult::default() })
    };
    let first = mark("a", MARK);
    let second = mark("b", BURNING);
    for (rules, won, lost) in [(vec![first.clone(), second.clone()], MARK, BURNING), (vec![second, first], BURNING, MARK)] {
        let rules = Rules::new(rules);
        let mut s = ObjectStates::new();
        let mut q = RuleQueue::new();
        q.push(Tick(0), cause(P1, Some(WOOD), &[FIRE]));
        let out = step_rules(&mut s, &mut q, &rules, Tick(0), &params);
        assert_eq!(out.len(), 1, "one match per cause");
        assert!(s.has(P1, won) && !s.has(P1, lost));
    }
}

#[test]
fn budget_carries_causes_over_oldest_first() {
    let p = InteractParams { rules_evals_per_tick: 2, ..params() };
    let tag = rule("tag", filter(&[], &[FIRE], &[]), TagFilter::default(), RuleResult { change: Some(MARK), ..RuleResult::default() });
    let rules = Rules::new(vec![tag]);
    let mut s = ObjectStates::new();
    let mut q = RuleQueue::new();
    for pl in [P3, P1, P2] {
        q.push(Tick(1), cause(pl, None, &[FIRE]));
    }
    let placements = |out: &[RuleEvent]| out.iter().map(|e| e.placement).collect::<Vec<_>>();
    // Same tick: canonical order by placement, two evaluated, P3 waits.
    assert_eq!(placements(&step_rules(&mut s, &mut q, &rules, Tick(1), &p)), vec![P1, P2]);
    assert_eq!(q.len(), 1);
    // A newer cause on a lower placement goes after the older one that waited.
    q.push(Tick(2), cause(P0, None, &[FIRE]));
    assert_eq!(placements(&step_rules(&mut s, &mut q, &rules, Tick(2), &p)), vec![P3, P0]);
    assert!(q.is_empty());

    // A cause that matches nothing still uses the budget.
    q.push(Tick(3), cause(P0, None, &[WATER]));
    q.push(Tick(3), cause(P1, None, &[WATER]));
    q.push(Tick(3), cause(P2, None, &[FIRE]));
    assert!(step_rules(&mut s, &mut q, &rules, Tick(3), &p).is_empty());
    assert_eq!(placements(&step_rules(&mut s, &mut q, &rules, Tick(4), &p)), vec![P2]);
}

/// Every permutation of `items` (Heap's algorithm).
fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    fn go<T: Clone>(k: usize, a: &mut Vec<T>, out: &mut Vec<Vec<T>>) {
        if k <= 1 {
            out.push(a.clone());
            return;
        }
        for i in 0..k {
            go(k - 1, a, out);
            let j = if k.is_multiple_of(2) { i } else { 0 };
            a.swap(j, k - 1);
        }
    }
    let mut a = items.to_vec();
    let mut out = Vec::new();
    go(a.len(), &mut a, &mut out);
    out
}

#[test]
fn permuted_causes_give_the_same_result() {
    let p = InteractParams { rules_evals_per_tick: 4, ..params() };
    let rules = Rules::new(vec![douse(), ignite()]);
    // Order-sensitive on P1 (fire then water vs water then fire), duplicate source tags, a cause that
    // waits for the next tick under the budget of 4.
    let causes = [
        cause(P1, Some(WOOD), &[FIRE]),
        cause(P1, Some(WOOD), &[WATER]),
        cause(P2, Some(WOOD), &[FIRE, FIRE]),
        cause(P2, Some(WOOD), &[WATER, FIRE]),
        cause(P0, Some(STONE), &[FIRE]),
    ];
    let run = |order: &[Cause]| {
        let mut s = ObjectStates::new();
        s.add(P2, BURNING, Ttl::UntilRemoved, Tick(0), &p);
        let mut q = RuleQueue::new();
        for c in order {
            q.push(Tick(5), c.clone());
        }
        let mut out = step_rules(&mut s, &mut q, &rules, Tick(5), &p);
        out.extend(step_rules(&mut s, &mut q, &rules, Tick(6), &p));
        (out, s)
    };
    let all = permutations(&causes);
    assert_eq!(all.len(), 120);
    let first = run(&all[0]);
    assert!(!first.0.is_empty());
    for order in &all[1..] {
        assert_eq!(run(order), first);
    }
}
