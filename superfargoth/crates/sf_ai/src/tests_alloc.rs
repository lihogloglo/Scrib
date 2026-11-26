//! `AI9a`: the tier allocator (hard caps, ranking, hysteresis, pins, in-view holds, determinism, stagger).

use crate::alloc::{AllocConfig, AllocInput, Allocator, CpuTier, Pin, Tier, TierCaps, decide_due, first_decide_after};
use crate::sight::stagger_phase;
use sf_ids::ActorKey;

const TICK_S: f32 = 1.0 / 60.0;

fn a(k: u128) -> ActorKey {
    ActorKey(k)
}

fn input(k: u128, d: f32) -> AllocInput {
    AllocInput { actor: a(k), nearest_player_m: d, visible: false, pin: None }
}

fn seen(k: u128, d: f32) -> AllocInput {
    AllocInput { visible: true, ..input(k, d) }
}

fn pinned(k: u128, d: f32, pin: Pin) -> AllocInput {
    AllocInput { pin: Some(pin), ..input(k, d) }
}

fn caps(full: usize, reduced: usize) -> TierCaps {
    TierCaps { full, reduced }
}

fn tier_of(alloc: &crate::alloc::Allocation, k: u128) -> Tier {
    alloc.tiers.iter().find(|(key, _)| *key == a(k)).map(|&(_, t)| t).unwrap()
}

fn margins(full: f32, reduced: f32) -> AllocConfig {
    AllocConfig { full_margin_m: full, reduced_margin_m: reduced, ..AllocConfig::default() }
}

#[test]
fn cpu_tier_caps_match_the_spec_table() {
    let want = [(48, 160), (80, 260), (128, 400), (192, 600), (256, 1000)];
    for (tier, (full, reduced)) in CpuTier::ALL.into_iter().zip(want) {
        assert_eq!(tier.caps(), caps(full, reduced), "{tier:?}");
    }
}

#[test]
fn caps_are_respected_on_every_cpu_tier() {
    let inputs: Vec<AllocInput> = (0..2000u16).map(|i| input(u128::from(i), f32::from(i))).collect();
    for cpu in CpuTier::ALL {
        let c = cpu.caps();
        let out = Allocator::new().allocate(&inputs, c, &AllocConfig::default(), 0, TICK_S);
        assert_eq!(out.stats.full, c.full, "{cpu:?}");
        assert_eq!(out.stats.reduced, c.reduced, "{cpu:?}");
        assert_eq!(out.stats.logical, 2000 - c.full - c.reduced, "{cpu:?}");
        assert_eq!((out.stats.over_cap_full, out.stats.over_cap_reduced), (0, 0), "{cpu:?}");
        // Nearest first: the first `full` keys are tier 2, the next `reduced` tier 1.
        for (i, &(_, t)) in out.tiers.iter().enumerate() {
            let want = if i < c.full {
                Tier::Full
            } else if i < c.full + c.reduced {
                Tier::Reduced
            } else {
                Tier::Logical
            };
            assert_eq!(t, want, "{cpu:?} actor {i}");
        }
    }
}

#[test]
fn ranking_is_pins_then_visible_then_distance_then_key() {
    // 1 tier-2 slot, 2 tier-1 slots, 5 actors.
    let inputs = [
        input(1, 1.0),                 // nearest but unseen
        seen(2, 50.0),                 // visible, far
        pinned(3, 500.0, Pin::Dialogue), // pinned, farthest
        seen(4, 50.0),                 // visible, same distance as 2: key breaks the tie
        input(5, 2.0),
    ];
    let out = Allocator::new().allocate(&inputs, caps(1, 2), &AllocConfig::default(), 0, TICK_S);
    assert_eq!(tier_of(&out, 3), Tier::Full);
    assert_eq!(tier_of(&out, 2), Tier::Reduced);
    assert_eq!(tier_of(&out, 4), Tier::Reduced);
    assert_eq!(tier_of(&out, 1), Tier::Logical);
    assert_eq!(tier_of(&out, 5), Tier::Logical);

    // Without the pin: visible beats nearer, key breaks the distance tie.
    let inputs = [input(1, 1.0), seen(2, 50.0), seen(4, 50.0), input(5, 2.0)];
    let out = Allocator::new().allocate(&inputs, caps(1, 2), &AllocConfig::default(), 0, TICK_S);
    assert_eq!(tier_of(&out, 2), Tier::Full);
    assert_eq!(tier_of(&out, 4), Tier::Reduced);
    assert_eq!(tier_of(&out, 1), Tier::Reduced);
    assert_eq!(tier_of(&out, 5), Tier::Logical);
}

#[test]
fn hysteresis_keeps_a_band_within_the_margin() {
    let cfg = margins(2.0, 4.0);
    let mut alloc = Allocator::new();
    // Tick 0: actor 1 at 10 m wins the single tier-2 slot over actor 2 at 12 m.
    let out = alloc.allocate(&[input(1, 10.0), input(2, 12.0)], caps(1, 1), &cfg, 0, TICK_S);
    assert_eq!((tier_of(&out, 1), tier_of(&out, 2)), (Tier::Full, Tier::Reduced));
    // Actor 1 drifts to 11 m, 2 comes to 10.5 m: 1 ranks as 9 m, keeps tier 2.
    let out = alloc.allocate(&[input(1, 11.0), input(2, 10.5)], caps(1, 1), &cfg, 1, TICK_S);
    assert_eq!((tier_of(&out, 1), tier_of(&out, 2)), (Tier::Full, Tier::Reduced));
    assert!(out.changes.is_empty());
    // Just inside the margin (1 at 12.4 m ranks 10.4 < 10.5): still kept.
    let out = alloc.allocate(&[input(1, 12.4), input(2, 10.5)], caps(1, 1), &cfg, 2, TICK_S);
    assert_eq!(tier_of(&out, 1), Tier::Full);
    // Past the margin (12.6 m ranks 10.6 > 10.5): they swap.
    let out = alloc.allocate(&[input(1, 12.6), input(2, 10.5)], caps(1, 1), &cfg, 3, TICK_S);
    assert_eq!((tier_of(&out, 1), tier_of(&out, 2)), (Tier::Reduced, Tier::Full));
    assert_eq!(out.changes.len(), 2);
    // And the new incumbent 2 now holds the margin the other way.
    let out = alloc.allocate(&[input(1, 11.0), input(2, 12.5)], caps(1, 1), &cfg, 4, TICK_S);
    assert_eq!((tier_of(&out, 1), tier_of(&out, 2)), (Tier::Reduced, Tier::Full));

    // Tier-1 band: margin 4 m. Actor 3 in tier 1 at 20 m, actor 4 challenges.
    let mut alloc = Allocator::new();
    let out = alloc.allocate(&[input(3, 20.0), input(4, 30.0)], caps(0, 1), &cfg, 0, TICK_S);
    assert_eq!((tier_of(&out, 3), tier_of(&out, 4)), (Tier::Reduced, Tier::Logical));
    let out = alloc.allocate(&[input(3, 23.0), input(4, 19.5)], caps(0, 1), &cfg, 1, TICK_S);
    assert_eq!(tier_of(&out, 3), Tier::Reduced, "23 - 4 = 19 < 19.5");
    let out = alloc.allocate(&[input(3, 24.0), input(4, 19.5)], caps(0, 1), &cfg, 2, TICK_S);
    assert_eq!((tier_of(&out, 3), tier_of(&out, 4)), (Tier::Logical, Tier::Reduced), "24 - 4 = 20 > 19.5");
}

#[test]
fn pins_exceed_the_cap_and_are_reported() {
    let inputs: Vec<AllocInput> = (0..5u8)
        .map(|i| pinned(u128::from(i), 100.0 + f32::from(i), if i % 2 == 0 { Pin::Combat } else { Pin::Scripted }))
        .chain([seen(10, 1.0), input(11, 1.0)])
        .collect();
    let out = Allocator::new().allocate(&inputs, caps(3, 1), &AllocConfig::default(), 0, TICK_S);
    for i in 0..5 {
        assert_eq!(tier_of(&out, i), Tier::Full, "pin {i}");
    }
    assert_eq!(out.stats.pinned, 5);
    assert_eq!(out.stats.full, 5);
    assert_eq!(out.stats.over_cap_full, 2);
    // No tier-2 slot left: the visible actor takes tier 1, the unseen one tier 0.
    assert_eq!(tier_of(&out, 10), Tier::Reduced);
    assert_eq!(tier_of(&out, 11), Tier::Logical);
}

#[test]
fn visible_incumbents_hold_tier_1_and_block_promotion_to_it() {
    let cfg = margins(0.0, 0.0);
    let mut alloc = Allocator::new();
    // Tick 0: 1 and 2 (visible, far) take the two tier-1 slots.
    alloc.allocate(&[seen(1, 50.0), seen(2, 60.0)], caps(0, 2), &cfg, 0, TICK_S);
    // Tick 1: a nearer visible newcomer and a pin arrive. The pin exceeds the tier-2 cap, the holds
    // stay in tier 1, the newcomer waits in tier 0 (no promotion past the cap).
    let out = alloc.allocate(
        &[seen(1, 50.0), seen(2, 60.0), seen(3, 5.0), pinned(4, 70.0, Pin::Combat)],
        caps(0, 2),
        &cfg,
        1,
        TICK_S,
    );
    assert_eq!(tier_of(&out, 4), Tier::Full);
    assert_eq!(tier_of(&out, 1), Tier::Reduced);
    assert_eq!(tier_of(&out, 2), Tier::Reduced);
    assert_eq!(tier_of(&out, 3), Tier::Logical);
    assert_eq!((out.stats.over_cap_full, out.stats.over_cap_reduced), (1, 0));
    assert_eq!(out.stats.held_in_view, 1, "the plain ranking keeps 3 and 1, drops 2");
    // Tick 2: actor 2 leaves view: it is demoted and the nearer newcomer takes its slot.
    let out = alloc.allocate(
        &[seen(1, 50.0), input(2, 60.0), seen(3, 5.0), pinned(4, 70.0, Pin::Combat)],
        caps(0, 2),
        &cfg,
        2,
        TICK_S,
    );
    assert_eq!(tier_of(&out, 2), Tier::Logical);
    assert_eq!(tier_of(&out, 1), Tier::Reduced);
    assert_eq!(tier_of(&out, 3), Tier::Reduced);
    assert_eq!(out.stats.held_in_view, 0);
    // Tick 3: actor 2 leaves the set.
    let out = alloc.allocate(&[seen(1, 50.0), seen(3, 5.0), input(4, 70.0)], caps(0, 2), &cfg, 3, TICK_S);
    assert_eq!(tier_of(&out, 4), Tier::Logical, "the ended pin is unseen: tier 1 is full of holds");
    assert_eq!(out.removed, vec![a(2)]);
}

#[test]
fn full_never_exceeds_its_cap_without_pins() {
    let cfg = AllocConfig::default();
    let mut alloc = Allocator::new();
    // Tick 0: six visible actors, all tier 2.
    let inputs: Vec<AllocInput> = (1..=6u8).map(|i| seen(u128::from(i), 10.0 * f32::from(i))).collect();
    let out = alloc.allocate(&inputs, caps(6, 10), &cfg, 0, TICK_S);
    assert_eq!(out.stats.full, 6);
    // Tick 1: the cap falls to 3: the three farthest, visible and incumbent all, drop to tier 1.
    let out = alloc.allocate(&inputs, caps(3, 10), &cfg, 1, TICK_S);
    for k in 1..=3 {
        assert_eq!(tier_of(&out, k), Tier::Full, "actor {k}");
    }
    for k in 4..=6 {
        assert_eq!(tier_of(&out, k), Tier::Reduced, "actor {k}");
    }
    assert_eq!(out.stats.full, 3);
    assert_eq!((out.stats.over_cap_full, out.stats.held_in_view), (0, 0));
    assert_eq!(out.changes.len(), 3);
    assert!(out.changes.iter().all(|c| c.from == Tier::Full && c.to == Tier::Reduced));
    // Tick 2: four pins far away: they all take tier 2 past the cap, every unpinned actor drops.
    let with_pins: Vec<AllocInput> =
        inputs.iter().copied().chain((10..14u8).map(|i| pinned(u128::from(i), 500.0, Pin::Combat))).collect();
    let out = alloc.allocate(&with_pins, caps(3, 10), &cfg, 2, TICK_S);
    assert_eq!((out.stats.full, out.stats.pinned, out.stats.over_cap_full), (4, 4, 1));
    for k in 1..=6 {
        assert_eq!(tier_of(&out, k), Tier::Reduced, "actor {k}");
    }
}

#[test]
fn a_visible_actor_past_the_reduced_cap_stays_reduced_and_is_counted() {
    let cfg = margins(0.0, 0.0);
    let mut alloc = Allocator::new();
    // Tick 0: 1 takes tier 2, 2 tier 1 (both visible).
    let out = alloc.allocate(&[seen(1, 10.0), seen(2, 20.0)], caps(1, 1), &cfg, 0, TICK_S);
    assert_eq!((tier_of(&out, 1), tier_of(&out, 2)), (Tier::Full, Tier::Reduced));
    // Tick 1: a nearer visible newcomer takes tier 2; 1 drops to tier 1, where 2 holds too: tier 1
    // runs one past its cap, nobody visible goes to tier 0, the unseen newcomer waits.
    let out = alloc.allocate(&[seen(1, 10.0), seen(2, 20.0), seen(3, 5.0), input(4, 1.0)], caps(1, 1), &cfg, 1, TICK_S);
    assert_eq!(tier_of(&out, 3), Tier::Full);
    assert_eq!(tier_of(&out, 1), Tier::Reduced);
    assert_eq!(tier_of(&out, 2), Tier::Reduced);
    assert_eq!(tier_of(&out, 4), Tier::Logical);
    assert_eq!((out.stats.full, out.stats.reduced, out.stats.logical), (1, 2, 1));
    assert_eq!((out.stats.over_cap_full, out.stats.over_cap_reduced), (0, 1));
    assert_eq!(out.stats.held_in_view, 1);
}

#[test]
fn same_inputs_in_any_order_give_the_same_allocation() {
    let base: Vec<AllocInput> = (0..60u8)
        .map(|i| {
            let d = f32::from(i % 7) * 3.0; // many distance ties
            match i % 5 {
                0 => seen(u128::from(i) * 7919, d),
                1 if i % 3 == 0 => pinned(u128::from(i) * 7919, d, Pin::Combat),
                _ => input(u128::from(i) * 7919, d),
            }
        })
        .collect();
    let run = |inputs: &[AllocInput]| {
        let mut alloc = Allocator::new();
        let first = alloc.allocate(inputs, caps(10, 20), &AllocConfig::default(), 0, TICK_S);
        let second = alloc.allocate(inputs, caps(8, 15), &AllocConfig::default(), 1, TICK_S);
        (first, second)
    };
    let want = run(&base);
    let mut shuffled = base.clone();
    shuffled.reverse();
    assert_eq!(run(&shuffled), want);
    // A fixed interleave (stride 7 over 60 is a permutation).
    let interleaved: Vec<AllocInput> = (0..60).map(|i| base[(i * 7) % 60]).collect();
    assert_eq!(run(&interleaved), want);
    // A duplicated actor counts once.
    let mut dup = base.clone();
    dup.push(base[3]);
    assert_eq!(run(&dup), want);
}

#[test]
fn bad_distances_rank_farthest() {
    let inputs = [input(1, f32::NAN), input(2, 1e9), input(3, -5.0)];
    let out = Allocator::new().allocate(&inputs, caps(1, 1), &AllocConfig::default(), 0, TICK_S);
    assert_eq!(tier_of(&out, 3), Tier::Full);
    assert_eq!(tier_of(&out, 2), Tier::Reduced);
    assert_eq!(tier_of(&out, 1), Tier::Logical);
}

#[test]
fn promotions_get_stable_staggered_first_decides() {
    let cfg = AllocConfig::default();
    let full_period = cfg.decide_period_ticks(Tier::Full, TICK_S).unwrap();
    let reduced_period = cfg.decide_period_ticks(Tier::Reduced, TICK_S).unwrap();
    assert_eq!((full_period, reduced_period), (9, 60));
    assert_eq!(cfg.decide_period_ticks(Tier::Logical, TICK_S), None);

    let inputs: Vec<AllocInput> = (1..=64u8).map(|i| input((u128::from(i) << 40) | 0xABCD, f32::from(i))).collect();
    let tick = 1000;
    let out = Allocator::new().allocate(&inputs, caps(32, 32), &cfg, tick, TICK_S);
    assert_eq!(out.changes.len(), 64);
    let mut phases = std::collections::BTreeSet::new();
    for c in &out.changes {
        let period = cfg.decide_period_ticks(c.to, TICK_S).unwrap();
        let first = c.first_decide_tick.unwrap();
        assert!(first > tick && first <= tick + u64::from(period), "{c:?}");
        assert_eq!(first % u64::from(period), u64::from(stagger_phase(c.actor, period)));
        // Stable: the same key, tick and period give the same first decide.
        assert_eq!(first_decide_after(c.actor, tick, period), first);
        if c.to == Tier::Reduced {
            phases.insert(first);
        }
    }
    // Spread: 32 tier-1 promotions over a 60-tick period land on many different ticks.
    assert!(phases.len() > 16, "{phases:?}");

    // Demotion to tier 0 carries no decide tick; a phase that falls on `tick` itself moves a period on.
    let mut alloc = Allocator::new();
    alloc.allocate(&[input(7, 1.0)], caps(1, 0), &cfg, 0, TICK_S);
    let out = alloc.allocate(&[input(7, 1.0)], caps(0, 0), &cfg, 1, TICK_S);
    assert_eq!(out.changes[0].first_decide_tick, None);
    let p = u64::from(stagger_phase(a(7), 9));
    assert_eq!(first_decide_after(a(7), 9 + p, 9), 18 + p);
}

#[test]
fn decide_due_is_first_tick_then_every_period() {
    // The first tick, then its multiples; nothing before it.
    assert!(decide_due(12, 9, 12));
    assert!(decide_due(12, 9, 21));
    assert!(decide_due(12, 9, 12 + 9 * 100));
    assert!(!decide_due(12, 9, 13));
    assert!(!decide_due(12, 9, 20));
    assert!(!decide_due(12, 9, 3), "before the first tick (12 - 9: no wrap-around)");
    assert!(!decide_due(12, 9, 11));
    // Period 1 (and 0, clamped): every tick from the first.
    assert!((12..40).all(|t| decide_due(12, 1, t)));
    assert!((12..40).all(|t| decide_due(12, 0, t)));
    assert!(!decide_due(12, 1, 11));
    // 600 ticks at a 60-tick period: 10 decides.
    assert_eq!((1..=600).filter(|&t| decide_due(31, 60, t)).count(), 10);
}
