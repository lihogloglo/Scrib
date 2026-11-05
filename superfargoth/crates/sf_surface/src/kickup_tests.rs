//! Kick-up (W7): the burst ring (allocation, wrap, ageing), the pace, the strength and the
//! closed-form arc the shader mirrors.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::many_single_char_names, reason = "exact twins and small test indices")]

use glam::{DVec3, Vec2, Vec3};

use crate::kickup::*;

const LOOKS: [KickLook; 2] = [KickLook::SNOW, KickLook::ASH];

fn burst(x: f64, born_s: f64, kind: KickKind) -> Burst {
    Burst { at: DVec3::new(x, 0.0, 0.0), forward: Vec2::X, kind, strength: 1.0, born_s, seed: 0.25 }
}

#[test]
fn ring_allocates_in_order_and_wraps_over_the_oldest() {
    let mut ring = BurstRing::new(4);
    for i in 0..4 {
        assert_eq!(ring.spawn(burst(f64::from(i), 0.0, KickKind::Snow)), i as usize);
    }
    assert_eq!(ring.live_count(), 4);
    // Full: the fifth overwrites slot 0 (the oldest), the sixth slot 1.
    assert_eq!(ring.spawn(burst(10.0, 0.1, KickKind::Snow)), 0);
    assert_eq!(ring.spawn(burst(11.0, 0.2, KickKind::Snow)), 1);
    assert_eq!(ring.get(0).unwrap().at.x, 10.0);
    assert_eq!(ring.get(1).unwrap().at.x, 11.0);
    assert_eq!(ring.get(2).unwrap().at.x, 2.0);
    assert_eq!(ring.live_count(), 4);
    assert_eq!(ring.spawned(), 6);
    assert_eq!(BurstRing::default().capacity(), BURSTS);
}

#[test]
fn ring_ages_out_each_kind_at_its_own_life() {
    let mut ring = BurstRing::new(8);
    ring.spawn(burst(0.0, 0.0, KickKind::Snow));
    ring.spawn(burst(1.0, 0.0, KickKind::Ash));
    let snow = f64::from(KickLook::SNOW.max_age_s());
    let ash = f64::from(KickLook::ASH.max_age_s());
    assert!(snow < ash);
    assert_eq!(ring.expire(snow - 0.01, &LOOKS), 2);
    assert_eq!(ring.expire(snow + 0.01, &LOOKS), 1);
    assert_eq!(ring.live().map(|(i, _)| i).collect::<Vec<_>>(), vec![1]);
    assert_eq!(ring.expire(ash + 0.01, &LOOKS), 0);
    // Freed slots are reused in ring order, not first-free order.
    assert_eq!(ring.spawn(burst(2.0, 5.0, KickKind::Snow)), 2);
    ring.clear();
    assert_eq!(ring.live_count(), 0);
}

#[test]
fn every_sprite_is_dead_by_max_age() {
    for look in LOOKS {
        for i in 0..64u8 {
            let s = Vec3::new(f32::from(i) / 64.0, 0.999, 0.999);
            let (delay, life) = timing(&look, s);
            assert!(delay + life <= look.max_age_s() + 1e-6);
        }
    }
}

#[test]
fn pace_pairs_a_print_with_the_last_one_a_step_away() {
    let mut clock = StepClock::default();
    // First print: unknown.
    assert_eq!(clock.step(DVec3::ZERO, 0.0), 0.0);
    // A run: 1.4 m in 0.35 s.
    let pace = clock.step(DVec3::new(1.4, 0.0, 0.0), 0.35);
    assert!((pace - 4.0).abs() < 1e-4, "{pace}");
    // A walk: 0.7 m in 0.5 s.
    let pace = clock.step(DVec3::new(2.1, 0.0, 0.0), 0.85);
    assert!((pace - 1.4).abs() < 1e-4, "{pace}");
    // Too long after: a fresh start.
    assert_eq!(clock.step(DVec3::new(2.8, 0.0, 0.0), 3.0), 0.0);
    // Too far (another walker): unknown.
    assert_eq!(clock.step(DVec3::new(20.0, 0.0, 0.0), 3.3), 0.0);
    // Never more than KEEP prints held.
    for i in 0..40 {
        clock.step(DVec3::new(f64::from(i) * 0.7, 0.0, 0.0), 4.0 + f64::from(i) * 0.01);
    }
    assert!(clock.recent_len() <= StepClock::KEEP);
}

#[test]
fn strength_follows_cover_pace_and_foot() {
    assert_eq!(strength(0.0, 5.0, FOOT_REF_M), 0.0);
    assert_eq!(strength(MIN_COVER, 5.0, FOOT_REF_M), 0.0);
    assert!((strength(1.0, 6.0, FOOT_REF_M) - 1.0).abs() < 1e-6);
    let stand = strength(1.0, 0.0, FOOT_REF_M);
    assert!((stand - 0.35).abs() < 1e-6);
    assert!(strength(1.0, 3.0, FOOT_REF_M) > stand);
    assert!((strength(1.0, 6.0, 0.5 * FOOT_REF_M) - 0.5).abs() < 1e-6);
    assert!((strength(1.0, 6.0, 3.0 * FOOT_REF_M) - 1.0).abs() < 1e-6);
}

#[test]
fn kind_is_the_larger_cover_above_the_threshold() {
    assert_eq!(kind_at(0.8, 0.2), Some((KickKind::Snow, 0.8)));
    assert_eq!(kind_at(0.1, 0.5), Some((KickKind::Ash, 0.5)));
    assert_eq!(kind_at(0.1, 0.05), None);
}

#[test]
fn arc_starts_at_launch_and_tends_to_the_terminal_velocity() {
    let (p0, v0, wind) = (Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, -0.5), Vec3::new(2.0, 0.0, 0.0));
    let (k, g) = (4.0, 7.0);
    assert!(arc(p0, v0, wind, k, g, 0.0).abs_diff_eq(p0, 1e-6));
    // Initial velocity by finite difference.
    let h = 1e-3;
    let v = (arc(p0, v0, wind, k, g, h) - p0) / h;
    assert!(v.abs_diff_eq(v0, 0.02), "{v}");
    // Late velocity: wind + terminal fall.
    let late = (arc(p0, v0, wind, k, g, 10.0 + h) - arc(p0, v0, wind, k, g, 10.0)) / h;
    assert!(late.abs_diff_eq(Vec3::new(2.0, -g / k, 0.0), 0.02), "{late}");
    // The ODE: a = k (wind - v) - g ŷ, at t = 0.2.
    let t = 0.2;
    let (a, b, c) = (arc(p0, v0, wind, k, g, t - h), arc(p0, v0, wind, k, g, t), arc(p0, v0, wind, k, g, t + h));
    let vel = (c - a) / (2.0 * h);
    let acc = (c - 2.0 * b + a) / (h * h);
    let want = k * (wind - vel) - Vec3::Y * g;
    assert!(acc.abs_diff_eq(want, 0.3), "{acc} vs {want}");
}

#[test]
fn snow_arcs_are_short_and_ash_drifts_downwind() {
    let wind = Vec3::new(3.0, 0.0, 0.0);
    let s = Vec3::new(0.0, 1.0, 1.0);
    let snow = KickLook::SNOW;
    let v0 = launch(&snow, Vec2::ZERO, 1.0, s);
    let (_, life) = timing(&snow, s);
    let end = arc(Vec3::ZERO, v0, wind * snow.wind_follow, snow.drag, snow.gravity, life);
    let top = (0..=50).map(|i| arc(Vec3::ZERO, v0, Vec3::ZERO, snow.drag, snow.gravity, life * i as f32 / 50.0).y).fold(0.0, f32::max);
    assert!(top > 0.1 && top < 0.6, "snow apex {top} m");
    assert!(end.length() < 1.5, "snow lands near the foot: {end}");
    let ash = KickLook::ASH;
    let still = arc(Vec3::ZERO, launch(&ash, Vec2::ZERO, 1.0, s), Vec3::ZERO, ash.drag, ash.gravity, 1.0);
    let windy = arc(Vec3::ZERO, launch(&ash, Vec2::ZERO, 1.0, s), wind * ash.wind_follow, ash.drag, ash.gravity, 1.0);
    assert!(windy.x - still.x > 1.0, "ash drifts with the wind: {still} -> {windy}");
}

#[test]
fn launch_spray_is_biased_forward_and_scaled_by_strength() {
    let look = KickLook::SNOW;
    let mean = (0..360)
        .map(|i| launch(&look, Vec2::Y, 1.0, Vec3::new(i as f32 / 360.0, 0.5, 0.5)))
        .fold(Vec3::ZERO, |a, v| a + v)
        / 360.0;
    assert!(mean.z > 0.3 && mean.x.abs() < 1e-3, "{mean}");
    assert!(mean.y > 0.0);
    let half = launch(&look, Vec2::Y, 0.5, Vec3::splat(0.3));
    let full = launch(&look, Vec2::Y, 1.0, Vec3::splat(0.3));
    assert!(half.abs_diff_eq(full * 0.5, 1e-6));
    assert_eq!(launch(&look, Vec2::Y, 0.0, Vec3::splat(0.3)), Vec3::ZERO);
}

#[test]
fn sprites_never_sink_through_the_floor_and_stay_finite() {
    let wind = Vec3::new(-4.0, 0.0, 2.0);
    for kind in KickKind::ALL {
        let look = KickLook::of(kind);
        let b = Burst { at: DVec3::new(1000.0, 50.0, -200.0), forward: Vec2::new(0.6, 0.8), kind, strength: 1.0, born_s: 0.0, seed: 0.7 };
        let origin = DVec3::new(990.0, 40.0, -190.0);
        let floor = 10.0 - FLOOR_BELOW_M;
        let mut shown = 0;
        for i in 0..SPRITES {
            let seed = Vec3::new((i as f32 * 0.37).fract(), (i as f32 * 0.71).fract(), (i as f32 * 0.13).fract());
            for step in 0..=60 {
                let age = look.max_age_s() * step as f32 / 60.0;
                if let Some((p, r, a)) = sprite_at(&look, &b, origin, wind, seed, age) {
                    shown += 1;
                    assert!(p.is_finite() && r.is_finite() && a.is_finite());
                    assert!(p.y >= floor + 0.5 * r - 1e-4, "{kind:?} sank: {p}");
                    assert!((0.0..=1.0).contains(&a) && r > 0.0);
                }
            }
            assert!(sprite_at(&look, &b, origin, wind, seed, look.max_age_s() + 0.01).is_none());
        }
        assert!(shown > 0);
    }
}

#[test]
fn fade_and_size_over_life() {
    assert_eq!(fade(-0.1), 0.0);
    assert_eq!(fade(0.0), 0.0);
    assert_eq!(fade(1.0), 0.0);
    assert_eq!(fade(1.1), 0.0);
    assert!((fade(0.2) - 1.0).abs() < 1e-6);
    let look = KickLook::SNOW;
    let s = Vec3::splat(0.5);
    assert!(size(&look, 0.0, s) < size(&look, 0.5, s) && size(&look, 0.5, s) < size(&look, 1.0, s));
    assert!((size(&look, 1.0, s) - look.size[1]).abs() < 1e-6);
}

#[test]
fn sprite_seeds_vary_between_bursts_in_one_slot() {
    let m = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(sprite_seeds(m, 0.0), m);
    let a = sprite_seeds(m, 0.25);
    assert!(a.cmpge(Vec3::ZERO).all() && a.cmplt(Vec3::ONE).all());
    assert!(!a.abs_diff_eq(m, 1e-3));
}
