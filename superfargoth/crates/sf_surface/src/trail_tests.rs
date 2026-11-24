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
    let first = TrailPass { corner, depth_m: D, new_min: min, old_min: None, refill_m: 0.0, stamps: vec![foot(Vec2::new(0.03, 0.03), Vec2::X)], capture: None, press: Press::default() };
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

/// W6b: on a flat surface (top 0) the raised cover is the full depth on open snow, carves exactly
/// to the ground in a full print, stands higher on a lip, scales with partial cover, and ash in a
/// snow-deep ring prints only its own depth.
#[test]
fn raise_composes_cover_and_prints() {
    let knee = 0.45;
    assert_eq!(cover_raise(1.0, 0.0, knee, [0.0, 0.0], knee), knee);
    assert_eq!(cover_raise(1.0, 0.0, knee, [-knee, 0.0], knee), 0.0);
    let lip = cover_raise(1.0, 0.0, knee, [0.0, LIP_HEIGHT * knee], knee);
    assert!((lip - knee * (1.0 + LIP_HEIGHT)).abs() < 1e-6);
    assert!((cover_raise(0.5, 0.0, knee, [-0.1, 0.0], knee) - 0.5 * (knee - 0.1)).abs() < 1e-6);
    // Ash (3 cm) under a 0.45 m print: cut to the ground, never below it.
    assert_eq!(cover_raise(1.0, 0.0, 0.03, [-knee, 0.0], knee), 0.0);
    assert_eq!(cover_raise(0.0, 0.0, knee, [0.0, 0.1], knee), 0.0);
    // No ring: the cover's depth.
    assert_eq!(cover_raise(1.0, 0.0, knee, [-0.2, 0.3], 0.0), knee);
}

/// SW-6 "B" (2026-11-15, paving): the raise follows the bury depth, not where the cover shows. A
/// cover below the relief's high point (snow 0.4 in a road's grout, cobble tops at 0.8) raises
/// nothing, with or without prints and lips; past it the raise ramps (SW-6 2026-11-16: a
/// smoothstep over a ramp no narrower than `1 - COVER_TOP_MAX`) to the full depth at amount 1 over
/// tops up to `COVER_TOP_MAX`, part of it over higher tops, monotone; a flat surface (top 0) keeps
/// smoothstep(amount) x depth. `raise_tests.rs` holds the slope bound.
#[test]
fn raise_follows_the_bury_depth() {
    use crate::layer::{COVER_TOP_MAX, cover_bury};
    let knee = 0.45;
    let top = 0.8;
    let ss = |x: f32| x * x * (3.0 - 2.0 * x);
    assert_eq!(cover_raise(0.4, top, knee, [0.0, 0.0], 0.0), 0.0);
    assert_eq!(cover_raise(0.4, top, knee, [0.0, LIP_HEIGHT * knee], knee), 0.0);
    assert_eq!(cover_raise(top, top, knee, [0.0, 0.0], 0.0), 0.0);
    let ramp = 1.0 - COVER_TOP_MAX;
    assert!((cover_raise(0.9, top, knee, [0.0, 0.0], 0.0) - ss(0.1 / ramp) * knee).abs() < 1e-6);
    assert!((cover_raise(1.0, top, knee, [0.0, 0.0], 0.0) - ss(0.2 / ramp) * knee).abs() < 1e-6);
    assert_eq!(cover_raise(1.0, 0.5, knee, [0.0, 0.0], 0.0), knee);
    // Amount 1 buries any relief up to COVER_TOP_MAX; a top at 1 never rises.
    assert_eq!(cover_bury(1.0, COVER_TOP_MAX), 1.0);
    assert_eq!(cover_bury(1.0, 1.0), 0.0);
    // Flat: smoothstep(amount) x depth.
    for i in 0..=10u8 {
        let a = f32::from(i) / 10.0;
        assert!((cover_raise(a, 0.0, knee, [0.0, 0.0], 0.0) - ss(a) * knee).abs() < 1e-6);
    }
    // Monotone in the amount, for any top; full at amount 1 up to COVER_TOP_MAX.
    for t in [0.0, 0.3, COVER_TOP_MAX, 0.8, 1.0] {
        let mut last = 0.0;
        for i in 0..=40u8 {
            let r = cover_raise(f32::from(i) / 40.0, t, knee, [0.0, 0.0], 0.0);
            assert!(r >= last, "top {t}, step {i}: {r} < {last}");
            last = r;
        }
        if t <= COVER_TOP_MAX {
            assert_eq!(last, knee, "top {t}");
        }
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

// -- the press channel (A, GC-T1) ---------------------------------------------------------------

const RECOVERY: f32 = 2.0;
const RADIUS: f32 = 0.35;

fn press(dt_s: f32) -> Press {
    Press { dt_s, recovery_s: RECOVERY, radius_m: RADIUS }
}

/// A stamp presses 1 at its centre, 0.5 at half the radius (the smoothstep's midpoint) and
/// nothing from the radius on, round whatever the foot's facing.
#[test]
fn stamp_presses_its_centre_and_fades_to_the_radius() {
    let s = [foot(Vec2::ZERO, Vec2::Y)];
    let at = |d: Vec2| press_update(0.0, d, &s, &press(0.0), 0.0);
    assert_eq!(at(Vec2::ZERO), 1.0);
    assert_eq!(at(Vec2::new(RADIUS / 2.0, 0.0)), 0.5);
    assert_eq!(at(Vec2::new(0.0, -RADIUS / 2.0)), 0.5);
    assert_eq!(at(Vec2::new(RADIUS, 0.0)), 0.0);
    assert_eq!(at(Vec2::new(0.0, RADIUS * 1.5)), 0.0);
    assert_eq!(press_profile(0.1, 0.0), 0.0, "no radius: no press");
}

/// Two stamps Max-combine (a texel between two feet keeps the stronger press), and a stamp never
/// lowers an older, stronger press.
#[test]
fn presses_combine_max() {
    let a = foot(Vec2::ZERO, Vec2::X);
    let b = foot(Vec2::new(RADIUS, 0.0), Vec2::X);
    let q = Vec2::new(RADIUS / 4.0, 0.0);
    let pa = press_profile(RADIUS / 4.0, RADIUS);
    let pb = press_profile(RADIUS * 0.75, RADIUS);
    assert!(pa > pb && pb > 0.0);
    assert_eq!(press_update(0.0, q, &[a, b], &press(0.0), 0.0), pa);
    assert_eq!(press_update(0.0, q, &[b, a], &press(0.0), 0.0), pa);
    assert_eq!(press_update(0.9, q, &[b], &press(0.0), 0.0), 0.9);
}

/// N passes of `dt` give `exp(-N dt / recovery)` within a half float's rounding, and the press
/// is exactly 0 from the pass that takes it under the floor (`ln(256)` recovery times). Stored
/// in the half-float ring, each pass rounds once at most (2⁻¹¹), and the press still reaches 0.
#[test]
#[allow(clippy::cast_precision_loss)]
fn press_springs_back_exponentially_and_snaps_to_zero() {
    let dt = 1.0 / 60.0;
    let far = Vec2::splat(50.0);
    let (mut a, mut h) = (1.0f32, 1.0f32);
    let (mut pass, mut zero_at, mut half_zero_at) = (0, None, None);
    while (a > 0.0 || h > 0.0) && pass < 10_000 {
        a = press_update(a, far, &[], &press(dt), 0.0);
        h = half(press_update(h, far, &[], &press(dt), 0.0));
        pass += 1;
        let want = (-(pass as f32) * dt / RECOVERY).exp();
        if a > 0.0 {
            assert!((a - want).abs() <= want * HALF_ROUNDING, "pass {pass}: {a} vs {want}");
        } else if zero_at.is_none() {
            zero_at = Some(pass);
            assert!(want < PRESS_FLOOR * (1.0 + HALF_ROUNDING), "snapped at {want}");
            let before = (-((pass - 1) as f32) * dt / RECOVERY).exp();
            assert!(before >= PRESS_FLOOR * (1.0 - HALF_ROUNDING), "snapped late: {before}");
        }
        if h > 0.0 {
            assert!(h >= PRESS_FLOOR);
            assert!((h / want).ln().abs() <= pass as f32 * HALF_ROUNDING * 1.01, "half pass {pass}: {h} vs {want}");
        } else if half_zero_at.is_none() {
            half_zero_at = Some(pass);
        }
    }
    assert_eq!((a, h), (0.0, 0.0));
    // ln(256) · 2 s ≈ 11.1 s of 60 Hz passes; the half ring within the bound.
    let t = zero_at.unwrap() as f32 * dt;
    assert!((t - RECOVERY * (1.0 / PRESS_FLOOR).ln()).abs() < 2.0 * dt, "{t} s");
    assert!(half_zero_at.unwrap() as f32 * dt <= RECOVERY * press_bound() + dt);
    // No decay released: the press stays; no recovery time: it is gone at once.
    assert_eq!(press_update(0.5, Vec2::ZERO, &[], &press(0.0), 0.0), 0.5);
    assert_eq!(press_update(0.5, Vec2::ZERO, &[], &Press { dt_s: dt, recovery_s: 0.0, radius_m: RADIUS }, 0.0), 0.0);
}

/// A slow spring back at a high frame rate still moves a half-float press: decay is released in
/// steps of at least `PRESS_STEP` recovery times, each above the half float's rounding.
#[test]
#[allow(clippy::cast_precision_loss)]
fn press_steps_survive_half_float() {
    let (dt, recovery) = (1.0 / 360.0, 10.0);
    let mut clock = PressClock::default();
    clock.advance(dt, recovery, true);
    let mut a = 1.0f32;
    let mut frames = 0;
    while a > 0.0 && frames < 100_000 {
        let p = Press { dt_s: clock.advance(dt, recovery, false), recovery_s: recovery, radius_m: RADIUS };
        if p.dt_s > 0.0 {
            let next = half(press_update(a, Vec2::ZERO, &[], &p, 0.0));
            assert!(next < a, "a released step must move the press at {a}");
            a = next;
        }
        frames += 1;
    }
    assert_eq!(a, 0.0);
    let t = frames as f32 * dt;
    assert!(t <= recovery * (press_bound() + PRESS_STEP) + dt, "{t} s");
}

/// The clock keeps releasing decay for `press_bound` recovery times after the last press stamp
/// and then stops (the pass has no press work left); a new press restarts it.
#[test]
#[allow(clippy::cast_precision_loss)]
fn press_clock_runs_for_the_bound_after_the_last_press() {
    let dt = 1.0 / 120.0;
    let mut clock = PressClock::default();
    assert_eq!(clock.advance(dt, RECOVERY, false), 0.0, "nothing pressed: no work");
    assert!(!clock.live());
    clock.advance(dt, RECOVERY, true);
    let (mut released, mut last_release, mut frame) = (0.0, 0, 0);
    while frame < 100_000 {
        frame += 1;
        let r = clock.advance(dt, RECOVERY, false);
        if r > 0.0 {
            // Each release is at least one step of recovery times.
            assert!(r >= PRESS_STEP * RECOVERY - 1e-6, "{r}");
            released += r;
            last_release = frame;
        }
        if !clock.live() {
            break;
        }
    }
    let bound_s = press_bound() * RECOVERY;
    assert!(released >= bound_s && released - bound_s <= PRESS_STEP * RECOVERY + dt, "{released} vs {bound_s}");
    assert!((last_release as f32 * dt - bound_s).abs() <= PRESS_STEP * RECOVERY + dt, "stopped at frame {last_release}");
    // Past the bound: no more work, nothing pending.
    for _ in 0..1000 {
        assert_eq!(clock.advance(dt, RECOVERY, false), 0.0);
    }
    assert_eq!(clock.pending_s, 0.0);
    // The bound covers every rounding up: 1 decayed by it is under the floor.
    assert!((-press_bound()).exp() * (1.0 + HALF_ROUNDING).powf(press_bound() / PRESS_STEP) < PRESS_FLOOR * 1.0001);
    // A new press restarts the count.
    clock.advance(dt, RECOVERY, true);
    assert!(clock.live());
    assert_eq!(clock.decayed, Some(0.0));
}

/// Snow-only behaviour is untouched by the press: R/G of the pass and the header slots before
/// the press are the same with and without it, and the press slot carries dt, recovery, radius
/// and the floor.
#[test]
fn press_leaves_the_snow_channels_alone() {
    let min = window_min(DVec2::new(0.0, 0.0));
    #[allow(clippy::cast_possible_truncation)]
    let corner = min.as_vec2() * RING_TEXEL_M as f32;
    let base = TrailPass { corner, depth_m: D, new_min: min, old_min: Some(min), refill_m: 0.002, stamps: vec![foot(Vec2::new(0.03, 0.03), Vec2::X)], capture: None, press: Press::default() };
    let pressed = TrailPass { press: press(0.25), ..base.clone() };
    let n = usize::try_from(RING_SIZE * RING_SIZE).unwrap();
    let prev: Vec<[f32; 2]> = (0..n).map(|i| if i % 7 == 0 { [-0.05, 0.01] } else { [0.0, 0.0] }).collect();
    assert_eq!(base.run(&prev), pressed.run(&prev));
    let (s0, s1) = (base.slots(), pressed.slots());
    assert_eq!(s0[..HEADER_SLOTS - 1], s1[..HEADER_SLOTS - 1]);
    assert_eq!(s0[HEADER_SLOTS..], s1[HEADER_SLOTS..]);
    assert_eq!(s1[HEADER_SLOTS - 1], [0.25, RECOVERY, RADIUS, PRESS_FLOOR].map(f32::to_bits));
    // The press pass: the stamp's centre texel at 1, the entering texels and the far ring at 0.
    let a = pressed.run_press(&vec![0.0; n], &[]);
    let under = usize::try_from(IVec2::ZERO.rem_euclid(IVec2::splat(RING_SIZE)).x).unwrap();
    assert!(a[under] > 0.9);
    assert_eq!(a[n / 2 + 300], 0.0);
}

/// A press-only pass (GC-T2a: no cover look, ring depth 0): a stamp leaves R/G at 0 (finite, no
/// NaN from the zero depth) and still presses A.
#[test]
fn a_stamp_at_depth_zero_prints_nothing_and_still_presses() {
    let s = foot(Vec2::new(0.03, 0.03), Vec2::X);
    for q in [s.center, s.center + Vec2::new(0.1, 0.05), s.center + Vec2::new(0.2, 0.0)] {
        let v = lip_cap(s.at(q, 0.0), 0.0);
        assert!(v.iter().all(|x| x.is_finite()), "{v:?}");
        assert_eq!(v, [0.0, 0.0]);
    }
    let min = window_min(DVec2::new(0.0, 0.0));
    #[allow(clippy::cast_possible_truncation)]
    let corner = min.as_vec2() * RING_TEXEL_M as f32;
    let pass = TrailPass { corner, depth_m: 0.0, new_min: min, old_min: None, refill_m: 0.0, stamps: vec![s], capture: None, press: press(0.0) };
    let n = usize::try_from(RING_SIZE * RING_SIZE).unwrap();
    let ring = pass.run(&vec![[0.0, 0.0]; n]);
    assert!(ring.iter().all(|v| *v == [0.0, 0.0] && v.iter().all(|x| x.is_finite())));
    let a = pass.run_press(&vec![0.0; n], &[]);
    assert!(a.iter().all(|x| x.is_finite()));
    let under = usize::try_from(IVec2::ZERO.rem_euclid(IVec2::splat(RING_SIZE)).x).unwrap();
    assert!(a[under] > 0.9, "{}", a[under]);
}
