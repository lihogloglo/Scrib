//! The trail ring's CPU twin (W6a): re-centering, stamp shape, Min/Max combine, decay, packing.
#![allow(clippy::float_cmp)]

use glam::{DVec2, IVec2, Vec2};

use crate::trail::*;

const D: f32 = 0.15;

fn foot(center: Vec2, axis: Vec2) -> Stamp {
    Stamp { center, axis, half: FOOT_HALF_M, depth: 1.0 }
}

#[test]
fn window_is_centred_on_whole_texels() {
    assert_eq!(window_min(DVec2::new(0.0, 0.0)), IVec2::splat(-RING_SIZE / 2));
    // 10.03 m = texel 160 (0.0625 m); -0.01 m = texel -1.
    assert_eq!(window_min(DVec2::new(10.03, -0.01)), IVec2::new(160, -1) - RING_SIZE / 2);
}

#[test]
fn world_of_covers_the_window_once() {
    let n = 8;
    let min = IVec2::new(-13, 5);
    let mut seen: Vec<IVec2> = (0..n).flat_map(|z| (0..n).map(move |x| world_of(IVec2::new(x, z), min, n))).collect();
    seen.sort_by_key(|v| (v.y, v.x));
    let want: Vec<IVec2> = (0..n).flat_map(|z| (0..n).map(move |x| min + IVec2::new(x, z))).collect();
    assert_eq!(seen, want);
}

#[test]
fn recenter_clears_exactly_the_entering_rows() {
    let n = 16;
    let old = IVec2::new(100, -40);
    let new = old + IVec2::new(3, -2);
    let cleared: Vec<IVec2> = (0..n)
        .flat_map(|z| (0..n).map(move |x| IVec2::new(x, z)))
        .filter(|&a| !kept(a, Some(old), new, n))
        .collect();
    // 3 columns + 2 rows enter: 3·16 + 2·16 − 3·2 texels.
    assert_eq!(cleared.len(), 3 * 16 + 2 * 16 - 6);
    for a in cleared {
        let w = world_of(a, new, n);
        let in_old = (w - old).cmpge(IVec2::ZERO).all() && (w - old).cmplt(IVec2::splat(n)).all();
        assert!(!in_old, "{a} held {w}, which the old window had");
    }
    // Nothing valid: everything clears; standing still: nothing does.
    assert!(!kept(IVec2::ZERO, None, new, n));
    assert!((0..n).all(|x| kept(IVec2::new(x, 7), Some(new), new, n)));
}

#[test]
fn stamp_carves_to_the_ground_with_a_lip_around_it() {
    let s = foot(Vec2::new(2.0, 3.0), Vec2::X);
    assert_eq!(s.at(s.center, D), [-D, 0.0]);
    // Inside the flat bottom (r < 1 - WALL): full depth.
    assert_eq!(s.at(s.center + Vec2::new(0.5 * FOOT_HALF_M.x, 0.0), D), [-D, 0.0]);
    // On the rim: no depression, no lip yet.
    let rim = s.at(s.center + Vec2::new(FOOT_HALF_M.x, 0.0), D);
    assert!(rim[0].abs() < 1e-6 && rim[1].abs() < 1e-6, "{rim:?}");
    // Mid-lip (r = 1 + LIP_WIDTH / 2): the lip's top.
    let top = s.at(s.center + Vec2::new(0.0, FOOT_HALF_M.y * (1.0 + LIP_WIDTH / 2.0)), D);
    assert_eq!(top[0], 0.0);
    assert!((top[1] - LIP_HEIGHT * D).abs() < 1e-6, "{top:?}");
    // Past the reach: nothing.
    assert_eq!(s.at(s.center + Vec2::new(s.reach() + 0.01, 0.0), D), [0.0, 0.0]);
    // Oriented: along Z, the long axis is Z.
    let z = foot(Vec2::ZERO, Vec2::Y);
    assert_eq!(z.at(Vec2::new(0.0, 0.10), D), [-D, 0.0]);
    assert_eq!(z.at(Vec2::new(0.12, 0.0), D)[0], 0.0);
    // Half depth stamps half as deep.
    assert_eq!(Stamp { depth: 0.5, ..s }.at(s.center, D), [-0.5 * D, 0.0]);
}

#[test]
fn profile_is_the_print_and_its_quadratic_lip() {
    assert_eq!(profile(0.0), (1.0, 0.0));
    assert_eq!(profile(1.0 - WALL), (1.0, 0.0));
    assert_eq!(profile(1.0), (0.0, 0.0));
    let (dep, lip) = profile(1.0 + LIP_WIDTH * 0.25);
    assert_eq!(dep, 0.0);
    assert!((lip - 0.75).abs() < 1e-6);
    let (dep, lip) = profile(1.0 + LIP_WIDTH);
    assert!(dep == 0.0 && lip < 1e-6, "{lip}");
}

#[test]
fn stamps_combine_min_on_depth_max_on_lip() {
    assert_eq!(combine([-0.05, 0.01], [-0.1, 0.0]), [-0.1, 0.01]);
    assert_eq!(combine([-0.1, 0.0], [-0.05, 0.03]), [-0.1, 0.03]);
    // Order doesn't matter.
    let (a, b) = ([-0.02, 0.004], [-0.07, 0.001]);
    assert_eq!(combine(a, b), combine(b, a));
}

#[test]
fn lip_never_stands_in_a_print() {
    // Full depth: no lip. Half depth: half the lip height at most. Bare: the full lip.
    assert_eq!(lip_cap([-D, 0.03], D), [-D, 0.0]);
    assert_eq!(lip_cap([-0.5 * D, 0.03], D), [-0.5 * D, LIP_HEIGHT * D * 0.5]);
    assert_eq!(lip_cap([0.0, 0.01], D), [0.0, 0.01]);
    // A new print over an older print's lip: flat bottom.
    let older = foot(Vec2::ZERO, Vec2::X);
    let q = Vec2::new(0.0, FOOT_HALF_M.y * (1.0 + LIP_WIDTH / 2.0));
    let newer = foot(q, Vec2::X);
    let v = texel_update(older.at(q, D), q, &[newer], 0.0, D);
    assert_eq!(v, [-D, 0.0]);
}

#[test]
fn decay_refills_prints_and_flattens_lips() {
    let v = decay([-0.1, 0.02], 0.004);
    assert!((v[0] + 0.096).abs() < 1e-7 && (v[1] - (0.02 - 0.004 * LIP_HEIGHT)).abs() < 1e-7, "{v:?}");
    assert_eq!(decay([-0.001, 0.0005], 0.004), [0.0, 0.0]);
    // Nothing falling: prints stay.
    assert_eq!(decay([-0.1, 0.02], 0.0), [-0.1, 0.02]);
    // Refilling a full print takes depth / step passes.
    let mut v = [-D, 0.0];
    let mut passes = 0;
    while v[0] < -1e-6 {
        v = decay(v, REFILL_QUANTUM_M);
        passes += 1;
    }
    assert_eq!(passes, 75);
}

#[test]
fn refill_is_released_in_quanta() {
    let mut r = Refill::default();
    // Morrowind snow (0.5 / h) on 0.15 m: 0.45 m per game hour.
    let rate = refill_rate_m_per_h(0.5, 0.0, D, 0.03);
    assert!((rate - 0.45).abs() < 1e-6);
    // 1/1000 h: 0.45 mm, held; four more release 2.25 mm.
    assert_eq!(r.add(rate, 0.001), 0.0);
    for _ in 0..3 {
        assert_eq!(r.add(rate, 0.001), 0.0);
    }
    let out = r.add(rate, 0.001);
    assert!((out - 0.00225).abs() < 1e-6, "{out}");
    assert_eq!(r.pending_m, 0.0);
    // Time going back adds nothing; no fall adds nothing.
    assert_eq!(r.add(rate, -1.0), 0.0);
    assert_eq!(r.add(0.0, 1.0), 0.0);
    assert_eq!(r.pending_m, 0.0);
}

#[test]
fn pass_keeps_moves_and_stamps() {
    let n = RING_SIZE;
    let idx = |a: IVec2| usize::try_from(a.y * n + a.x).unwrap();
    let min = window_min(DVec2::new(0.0, 0.0));
    // Render space = world here (origin 0).
    #[allow(clippy::cast_possible_truncation)]
    let corner = min.as_vec2() * RING_TEXEL_M as f32;
    let first = TrailPass { corner, depth_m: D, new_min: min, old_min: None, refill_m: 0.0, stamps: vec![foot(Vec2::new(0.03, 0.03), Vec2::X)] };
    let ring = first.run(&vec![[1.0, 1.0]; idx(IVec2::new(0, n))]);
    // The texel under the stamp's centre: world texel (0, 0), at address world mod n.
    let under = IVec2::ZERO.rem_euclid(IVec2::splat(n));
    assert_eq!(ring[idx(under)], [-D, 0.0]);
    // Everything else was cleared (the old window was invalid), far from the stamp.
    assert_eq!(ring[idx(IVec2::new(3, 900))], [0.0, 0.0]);
    // Move one texel in +X with refill: the print refills one step, the column that entered is 0.
    let second = TrailPass { corner: corner + Vec2::new(0.0625, 0.0), new_min: min + IVec2::X, old_min: Some(min), refill_m: 0.002, stamps: vec![], ..first.clone() };
    let mut prev = ring.clone();
    let entering = (min.x + n).rem_euclid(n); // the address of world column min.x + n
    prev[idx(IVec2::new(entering, 10))] = [-0.05, 0.0];
    let ring2 = second.run(&prev);
    assert!((ring2[idx(under)][0] + D - 0.002).abs() < 1e-7);
    assert_eq!(ring2[idx(IVec2::new(entering, 10))], [0.0, 0.0]);
    // The packed uniform: four header slots, count capped.
    let many = TrailPass { stamps: vec![foot(Vec2::ZERO, Vec2::X); MAX_STAMPS + 5], ..first };
    let s = many.slots();
    assert_eq!(s.len(), PARAM_SLOTS);
    assert_eq!(s[2], [u32::try_from(MAX_STAMPS).unwrap(), 0, 1024, 0]);
    assert_eq!(f32::from_bits(s[0][3]), D);
}

/// W6b: the raised cover is the full depth on open snow, carves exactly to the ground in a full
/// print, stands higher on a lip, scales with partial cover, and ash in a snow-deep ring prints
/// only its own depth.
#[test]
fn raise_composes_cover_and_prints() {
    let knee = 0.45;
    assert_eq!(cover_raise(1.0, 1.0, knee, [0.0, 0.0], knee), knee);
    assert_eq!(cover_raise(1.0, 1.0, knee, [-knee, 0.0], knee), 0.0);
    let lip = cover_raise(1.0, 1.0, knee, [0.0, LIP_HEIGHT * knee], knee);
    assert!((lip - knee * (1.0 + LIP_HEIGHT)).abs() < 1e-6);
    assert!((cover_raise(0.5, 1.0, knee, [-0.1, 0.0], knee) - 0.5 * (knee - 0.1)).abs() < 1e-6);
    // Ash (3 cm) under a 0.45 m print: cut to the ground, never below it.
    assert_eq!(cover_raise(1.0, 1.0, 0.03, [-knee, 0.0], knee), 0.0);
    assert_eq!(cover_raise(0.0, 1.0, knee, [0.0, 0.1], knee), 0.0);
    // No ring: the cover's depth.
    assert_eq!(cover_raise(1.0, 1.0, knee, [-0.2, 0.3], 0.0), knee);
}

/// SW-6 (2026-11-06, paving): the raise follows the shading's coverage, not the cell amount.
/// Where the height blend leaves the ground bare (a cobble top at a thin cover, a drift gap) the
/// ground is never lifted, with or without prints and lips; full cover stands the full depth.
#[test]
fn raise_is_zero_where_the_coverage_is() {
    use crate::layer::cover_coverage;
    let knee = 0.45;
    // A thin cover (0.4): a high cobble top in a drift gap is bare, the grout in a drift is covered.
    let bare = cover_coverage(0.4, 0.9, 0.2, 0.9, 1.0);
    let covered = cover_coverage(0.4, 0.1, 0.8, 0.2, 1.0);
    assert_eq!(bare, 0.0);
    assert_eq!(covered, 1.0);
    assert_eq!(cover_raise(0.4, bare, knee, [0.0, 0.0], 0.0), 0.0);
    assert_eq!(cover_raise(0.4, bare, knee, [0.0, LIP_HEIGHT * knee], knee), 0.0);
    assert!((cover_raise(0.4, covered, knee, [0.0, 0.0], 0.0) - 0.4 * knee).abs() < 1e-6);
    // Full cover covers every height, so the whole ground stands the full depth.
    for h in [0.0, 0.3, 0.7, 1.0] {
        let c = cover_coverage(1.0, h, 1.0 - h, h, 1.0);
        assert_eq!(c, 1.0);
        assert_eq!(cover_raise(1.0, c, knee, [0.0, 0.0], 0.0), knee);
    }
    // No cover: no coverage, no raise.
    assert_eq!(cover_coverage(0.0, 0.0, 1.0, 0.0, 1.0), 0.0);
    // Monotone in the coverage.
    let mut last = 0.0;
    for i in 0..=20u8 {
        let r = cover_raise(0.4, f32::from(i) / 20.0, knee, [0.0, 0.0], 0.0);
        assert!(r >= last);
        last = r;
    }
}

/// Rounds to the nearest half float (normal range): what the `Rgba16Float` ring stores.
#[allow(clippy::cast_possible_truncation)]
fn half(x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }
    let ulp = 2f32.powi(x.abs().log2().floor() as i32 - 10);
    (x / ulp).round() * ulp
}

/// W6b knee-deep snow: a 2 mm refill quantum still moves a 0.45 m print (half-float step there
/// is 0.24 mm), and a print refills to level in whole quanta.
#[test]
fn refill_quantum_survives_half_float_at_knee_depth() {
    let knee = 0.45;
    let mut v = [half(-knee), 0.0];
    let mut steps = 0;
    while v[0] < 0.0 && steps < 1000 {
        let next = [half(decay(v, REFILL_QUANTUM_M)[0]), 0.0];
        assert!(next[0] > v[0], "a quantum must move the print at {}", v[0]);
        if next[0] < 0.0 {
            assert!((next[0] - v[0] - REFILL_QUANTUM_M).abs() < 3e-4);
        }
        v = next;
        steps += 1;
    }
    assert_eq!(v[0], 0.0);
    // 225 quanta, a few more where the half-float steps round down.
    assert!((225..=240).contains(&steps), "{steps}");
}
