use glam::{IVec3, Vec2, Vec3};
use sf_coords::WorldPos;

use crate::{MAX_SOURCES, PERIOD, Wind, WindSettings, WindSource, WindState};

fn points(n: usize, spread: f32) -> impl Iterator<Item = Vec3> {
    // Deterministic scatter (golden-ratio sequence), no rand dependency.
    (0..n).map(move |i| {
        #[expect(clippy::cast_precision_loss)]
        let i = i as f32;
        let x = (i * 0.618_034).fract() * 2.0 - 1.0;
        let z = (i * 0.754_878).fract() * 2.0 - 1.0;
        Vec3::new(x * spread, (i * 0.569_840).fract() * 30.0, z * spread)
    })
}

fn stormy() -> Wind {
    Wind { velocity: Vec2::new(-7.0, 9.5), gustiness: 0.8, gust_length: 45.0 }
}

#[test]
fn wind_is_deterministic_and_gusts_around_the_mean() {
    let wind = Wind::default();
    let a = wind.velocity_at(Vec3::new(10.0, 0.0, 3.0), 4.0);
    let b = wind.velocity_at(Vec3::new(10.0, 0.0, 3.0), 4.0);
    assert_eq!(a, b);
    let mean = wind.speed();
    let mut lo = f32::MAX;
    let mut hi = 0.0f32;
    for i in 0..400 {
        #[expect(clippy::cast_precision_loss)]
        let v = wind.velocity_at(Vec3::new(i as f32 * 7.3, 0.0, i as f32 * -3.1), 2.0);
        assert!(v.y.abs() < f32::EPSILON);
        assert!(v.length() <= mean * (1.0 + wind.gustiness) + 1e-4);
        assert!(v.length() >= mean * (1.0 - wind.gustiness) - 1e-4);
        lo = lo.min(v.length());
        hi = hi.max(v.length());
    }
    // Both lulls and gusts occur.
    assert!(lo < mean * 0.8 && hi > mean * 1.1, "{lo} {hi} around {mean}");
    let calm = Wind { velocity: Vec2::ZERO, ..wind };
    assert_eq!(calm.velocity_at(Vec3::ONE, 1.0), Vec3::ZERO);
}

#[test]
fn gusts_never_blow_upwind() {
    let settings = WindSettings { swirl: 0.99, ..WindSettings::default() };
    for (velocity, gust_length) in [(Vec2::X, 20.0), (Vec2::new(-3.0, 4.0), 50.0), (Vec2::new(0.2, -25.0), 300.0)] {
        let mut state = WindState::new(Wind { velocity, gustiness: 1.0, gust_length });
        let axis = velocity.normalize();
        let max_turn = settings.swirl.asin() + 1e-3;
        for _ in 0..20 {
            state.advance(0.37, &settings);
            let frame = state.frame(&settings, WorldPos::new(IVec3::new(-901, 0, 1733), Vec3::ZERO));
            for p in points(500, 2000.0) {
                let v = frame.global(p, 10.0);
                let along = Vec2::new(v.x, v.z).dot(axis);
                assert!(along >= 0.0, "upwind {v} for axis {axis}");
                let g = frame.gust(Vec2::new(p.x, p.z));
                assert!(g.dir.dot(axis).clamp(-1.0, 1.0).acos() <= max_turn);
            }
        }
    }
}

#[test]
fn phase_exact_across_an_origin_shift() {
    let settings = WindSettings::default();
    let mut state = WindState::new(stormy());
    state.add_source(WindSource::radial(WorldPos::from_f64(glam::DVec3::new(312_345.5, 40.0, -98_760.25)), 12.0, 80.0));
    for _ in 0..1000 {
        state.advance(1.0 / 60.0, &settings);
    }
    // A world point ~330 km out; the origin recenters by ~390 m (not whole
    // cells, not a period). Points stay within a few hundred meters of each
    // origin, as near the camera; farther out f32 position rounding alone
    // (2.4e-4 m at 4 km) moves the wind by ~1e-4 m/s, with or without a shift.
    let world = WorldPos::from_f64(glam::DVec3::new(312_400.125, 41.5, -98_700.75));
    let o1 = world.offset(Vec3::new(-150.0, -40.0, 120.0));
    let o2 = world.offset(Vec3::new(210.5, -12.0, -95.25));
    let mut worst = 0.0f32;
    for d in points(200, 60.0) {
        let p = world.offset(d);
        let v1 = state.frame(&settings, o1).sample(o1.delta_f32(p), 5.0);
        let v2 = state.frame(&settings, o2).sample(o2.delta_f32(p), 5.0);
        let v3 = state.velocity_at(&settings, p, 5.0);
        worst = worst.max((v1 - v2).length()).max((v1 - v3).length());
    }
    assert!(worst < 5e-5, "origin shift changed the wind by {worst} m/s");
}

#[test]
fn field_tiles_the_period() {
    let settings = WindSettings::default();
    let frame = WindState::new(stormy()).frame(&settings, WorldPos::ORIGIN);
    #[expect(clippy::cast_possible_truncation)]
    let period = PERIOD as f32;
    for p in points(100, 400.0) {
        let a = frame.global(p, 10.0);
        let b = frame.global(p + Vec3::new(period, 0.0, -period), 10.0);
        assert!((a - b).length() < 1e-4, "{a} vs {b}");
    }
}

#[test]
fn speed_change_keeps_the_gust_pattern() {
    let settings = WindSettings::default();
    let mut state = WindState::new(stormy());
    state.advance(13.0, &settings);
    let before = state.frame(&settings, WorldPos::ORIGIN);
    state.wind.velocity *= 2.0;
    let after = state.frame(&settings, WorldPos::ORIGIN);
    for p in points(100, 300.0) {
        let (a, b) = (before.global(p, 10.0), after.global(p, 10.0));
        assert!((a * 2.0 - b).length() < 1e-4);
    }
    // And the field moves: some point changes after more travel.
    state.advance(5.0, &settings);
    let later = state.frame(&settings, WorldPos::ORIGIN);
    assert!(points(100, 300.0).any(|p| (later.global(p, 10.0) - after.global(p, 10.0)).length() > 0.5));
}

#[test]
fn altitude_profile_is_logarithmic() {
    let settings = WindSettings::default();
    let frame = WindState::new(stormy()).frame(&settings, WorldPos::ORIGIN);
    assert!(frame.profile(0.0).abs() < 1e-6);
    assert!(frame.profile(-3.0).abs() < 1e-6);
    assert!((frame.profile(settings.reference_height) - 1.0).abs() < 1e-5);
    assert!(frame.profile(2.0) < frame.profile(5.0));
    assert!((frame.profile(1e5) - settings.max_profile).abs() < 1e-6);
    let p = Vec3::new(10.0, 0.0, 20.0);
    let ratio = frame.global(p, 2.0).length() / frame.global(p, 10.0).length();
    let want = (2.0f32 / 0.1 + 1.0).ln() / (10.0f32 / 0.1 + 1.0).ln();
    assert!((ratio - want).abs() < 1e-4);
}

#[test]
fn sources_push_swirl_and_expire() {
    let settings = WindSettings::default();
    let mut state = WindState::new(Wind { velocity: Vec2::ZERO, ..Wind::default() });
    let c = WorldPos::new(IVec3::new(5, 0, -2), Vec3::new(10.0, 1.0, -4.0));
    let burst = state.add_source(WindSource::radial(c, 20.0, 50.0).with_inner_radius(10.0).with_lifetime(0.1, 2.0, 1.0));
    let frame = state.frame(&settings, c);
    // Attack: zero at age 0.
    assert!(frame.sample(Vec3::new(5.0, 0.0, 0.0), 1.0).length() < 1e-6);
    state.advance(0.5, &settings);
    let frame = state.frame(&settings, c);
    let v = frame.sample(Vec3::new(5.0, 0.0, 0.0), 1.0);
    assert!((v - Vec3::new(20.0, 0.0, 0.0)).length() < 1e-4, "{v}");
    assert!(frame.sample(Vec3::new(0.0, 0.0, -60.0), 1.0).length() < 1e-6);
    // Release fades, then it goes.
    state.advance(1.0, &settings);
    let half = state.frame(&settings, c).sample(Vec3::new(5.0, 0.0, 0.0), 1.0).x;
    assert!((half - 10.0).abs() < 1e-3, "{half}");
    state.advance(0.6, &settings);
    assert_eq!(state.sources().count(), 0);
    assert!(!state.remove_source(burst.unwrap()));

    let v_id = state.add_source(WindSource::vortex(c, Vec3::Y, 8.0, 30.0).with_inner_radius(10.0)).unwrap();
    let v = state.frame(&settings, c).sample(Vec3::new(4.0, 0.0, 0.0), 1.0);
    // Right-handed around +Y: +X turns toward -Z.
    assert!((v - Vec3::new(0.0, 0.0, -8.0)).length() < 1e-5, "{v}");
    assert!(state.remove_source(v_id));

    state.add_source(WindSource::directional(c, Vec3::new(0.0, 0.0, 3.0), 6.0, 30.0));
    let v = state.frame(&settings, c).sample(Vec3::new(0.0, 0.0, 0.0), 1.0);
    assert!((v - Vec3::new(0.0, 0.0, 6.0)).length() < 1e-5);

    for _ in 1..MAX_SOURCES {
        assert!(state.add_source(WindSource::radial(c, 1.0, 1.0)).is_some());
    }
    assert!(state.add_source(WindSource::radial(c, 1.0, 1.0)).is_none());
    #[expect(clippy::cast_precision_loss)]
    let count = MAX_SOURCES as f32;
    assert!((state.frame(&settings, c).params.shape.w - count).abs() < f32::EPSILON);
}

#[test]
fn lulls_and_gusts_are_patchy() {
    let frame = WindState::new(stormy()).frame(&WindSettings::default(), WorldPos::ORIGIN);
    let gusty = points(2000, 1000.0).filter(|p| frame.gust(Vec2::new(p.x, p.z)).gust > 0.5).count();
    assert!((200..1800).contains(&gusty), "{gusty} / 2000 gusty");
}
