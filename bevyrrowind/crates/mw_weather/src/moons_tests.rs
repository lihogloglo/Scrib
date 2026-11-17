//! `MoonModel` against `OpenMW`'s (NS-3). The expected cycles and points were computed with an
//! independent transliteration of `weather.cpp` (JavaScript, f64) and checked by hand where noted.
#![allow(clippy::float_cmp, reason = "exact values: OpenMW's piecewise results and clamped opacities")]

use super::*;

fn masser() -> MoonModel {
    MoonModel::from_ini(MoonName::Masser, None)
}

fn secunda() -> MoonModel {
    MoonModel::from_ini(MoonName::Secunda, None)
}

fn close(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn vanilla_ini_values_and_day_length() {
    let s = MoonSettings::from_ini(MoonName::Secunda, None);
    assert_eq!((s.size, s.axis_offset, s.speed, s.daily_increment), (40.0, 50.0, 0.6, 1.2));
    assert_eq!((s.fade_start_angle, s.fade_end_angle, s.moon_shadow_early_fade_angle), (50.0, 30.0, 0.5));
    assert_eq!((s.fade_in_start, s.fade_in_finish, s.fade_out_start, s.fade_out_finish), (14.0, 15.0, 7.0, 10.0));
    let m = MoonSettings::from_ini(MoonName::Masser, None);
    assert_eq!((m.size, m.axis_offset, m.speed, m.daily_increment, m.fade_end_angle), (94.0, 35.0, 0.5, 1.0, 40.0));
    assert_eq!(day_hours(None), (6.0, 18.0));
    // calculateSettings: 0.5 * 180 / 12 = 7.5 is under the 180/23 minimum; 0.6 * 180 / 12 = 9.
    assert!(close(masser().degrees_per_hour(), 180.0 / 23.0, 1e-6));
    assert!(close(secunda().degrees_per_hour(), 9.0, 1e-6));
}

#[test]
fn player_ini_overrides_a_key() {
    let ini = MorrowindIni::parse(b"[Moons]\nSecunda Axis Offset=10\n");
    let s = MoonSettings::from_ini(MoonName::Secunda, Some(&ini));
    assert_eq!((s.axis_offset, s.size), (10.0, 40.0));
}

/// `buildCycle`: (rise hour, handover hour) per cycle day, from the JavaScript twin.
#[test]
fn cycles_match_openmw() {
    let masser_cycle: [(f32, f32); 24] = [
        (16.0, 10.9528), (17.0, 11.9528), (18.0, 12.9528), (19.0, 0.0), (20.0, 0.0), (21.0, 0.0), (22.0, 0.0), (23.0, 0.0),
        (24.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (4.0, 0.0), (5.0, 0.0), (6.0, 0.9528), (7.0, 1.9528),
        (8.0, 2.9528), (9.0, 3.9528), (10.0, 4.9528), (11.0, 5.9528), (12.0, 6.9528), (13.0, 7.9528), (14.0, 8.9528),
        (15.0, 9.9528),
    ];
    let secunda_cycle: [(f32, f32); 24] = [
        (19.2, 13.1222), (20.4, 14.3222), (21.6, 0.0), (22.8, 0.0), (24.0, 0.0), (1.2, 0.0), (2.4, 0.0), (3.6, 0.0),
        (24.0, 0.0), (1.2, 0.0), (2.4, 0.0), (3.6, 0.0), (4.8, 0.0), (6.0, 0.0), (7.2, 1.1222), (8.4, 2.3222),
        (9.6, 3.5222), (10.8, 4.7222), (12.0, 5.9222), (13.2, 7.1222), (14.4, 8.3222), (15.6, 9.5222), (16.8, 10.7222),
        (18.0, 11.9222),
    ];
    for (model, want) in [(masser(), masser_cycle), (secunda(), secunda_cycle)] {
        for (i, (got, (rise, handover))) in model.cycle().iter().zip(want).enumerate() {
            assert!(close(got.rise_hour, rise, 1e-3) && close(got.handover_hour, handover, 1e-3), "day {i}: {got:?}");
        }
    }
    // riseHour's 24-day skip: Secunda's 1.2 h a day would drift 28.8 h a cycle; 4 increments are
    // skipped each cycle so day 25 rises like day 1 (17 increments from the start date: 20.4 h).
    let s = secunda();
    assert!(close(s.rise_hour(1), 20.4, 1e-4));
    assert!(close(s.rise_hour(25), s.rise_hour(1), 1e-3));
    assert!(close(masser().rise_hour(1), 17.0, 1e-5));
}

/// Hand-checked points. A new game (day 1, 16 Last Seed): both full; Masser rose at 17:00 and
/// shows from about 22:03 (angle 39.5 at 7.826 deg/h), Secunda rose at 20:24 and shows from about
/// 23:40 (angle 29.5 at 9 deg/h); both fade out 7:00-10:00 and are hidden 10:00-14:00.
#[test]
fn new_game_first_night() {
    let (m, s) = (masser(), secunda());
    let at = |model: &MoonModel, day, hour| model.state(day, hour);
    // 21:30: risen, still under the fade end angle.
    let (m21, s21) = (at(&m, 1, 21.5), at(&s, 1, 21.5));
    assert!(close(m21.rotation_from_horizon, 35.2174, 1e-3) && m21.alpha == 0.0);
    assert!(close(s21.rotation_from_horizon, 9.9, 1e-3) && s21.alpha == 0.0);
    assert_eq!((m21.phase, s21.phase), (Phase::Full, Phase::Full));
    // 23:00: Masser up, its texture fading in (40..50 deg); Secunda still hidden.
    let (m23, s23) = (at(&m, 1, 23.0), at(&s, 1, 23.0));
    assert!(close(m23.rotation_from_horizon, 46.9565, 1e-3) && m23.alpha == 1.0);
    assert!(close(m23.shadow_blend, 0.69565, 1e-4));
    assert!(close(s23.rotation_from_horizon, 23.4, 1e-3) && s23.alpha == 0.0);
    // 23:45: Secunda in.
    let s2345 = at(&s, 1, 23.75);
    assert!(close(s2345.rotation_from_horizon, 30.15, 1e-3) && s2345.alpha == 1.0 && close(s2345.shadow_blend, 0.0075, 1e-3));
    // Day 2, 03:00: both high and textured.
    let (m3, s3) = (at(&m, 2, 3.0), at(&s, 2, 3.0));
    assert!(close(m3.rotation_from_horizon, 78.2609, 1e-3) && close(s3.rotation_from_horizon, 59.4, 1e-3));
    assert!(m3.shadow_blend == 1.0 && s3.shadow_blend == 1.0 && m3.alpha == 1.0 && s3.alpha == 1.0);
    // 07:30 on day 1 (before the handover): fading out, one-minute early cull (10 - 0.0167).
    let m7 = at(&m, 1, 7.5);
    assert!(close(m7.alpha, (9.9833 - 7.5) / (9.9833 - 7.0), 1e-4), "{m7:?}");
    assert!(close(m.hourly_alpha(9.99), 0.0, 0.0));
    // Noon: hidden.
    assert_eq!((at(&m, 1, 12.0).alpha, at(&s, 1, 12.0).alpha), (0.0, 0.0));
}

#[test]
fn phases_follow_the_calendar() {
    // Full on days 1-2, waning gibbous from day 3; one step every 3 days, 24-day cycle.
    assert_eq!(MoonModel::phase(1, false), Phase::Full);
    assert_eq!(MoonModel::phase(2, true), Phase::WaningGibbous);
    assert_eq!(MoonModel::phase(3, false), Phase::WaningGibbous);
    assert_eq!(MoonModel::phase(13, false), Phase::New);
    // The NS-3 scenes: clock day 168 (game day 169) full, 162 first quarter, 159 crescent.
    assert_eq!(MoonModel::phase(169, false), Phase::Full);
    assert_eq!(MoonModel::phase(163, false), Phase::FirstQuarter);
    assert_eq!(MoonModel::phase(160, false), Phase::WaxingCrescent);
    assert_eq!(Phase::FirstQuarter.cycle_fraction(), 0.25);
    assert_eq!(Phase::Full.texture_suffix(), "full");
    assert!(Phase::FROM_NEW.iter().zip(0_u8..).all(|(p, k)| p.cycle_fraction() * 8.0 == f32::from(k)));
}

#[test]
fn clock_days_map_to_openmw_days() {
    assert_eq!(game_day_hour(0.0), (1, 0.0));
    assert_eq!(game_day_hour(168.0 + 21.5 / 24.0), (169, 21.5));
    let (d, h) = game_day_hour(1.999_999_999);
    assert!(d == 2 && h < 24.0);
}

/// The twin sweep: every quarter hour of a 24-day cycle, both moons, each state field against its
/// `weather.cpp` definition evaluated from the cycle.
#[test]
fn sweep_matches_the_formulas() {
    for model in [masser(), secunda()] {
        let s = *model.settings();
        let dph = model.degrees_per_hour();
        for day in 1..=24 {
            for q in 0..96_u8 {
                let hour = f32::from(q) / 4.0;
                let st = model.state(day, hour);
                let today = model.cycle()[MoonModel::day_of_cycle(day)];
                let post = hour >= today.handover_hour;
                let rise = if post { model.cycle()[MoonModel::day_of_cycle(day + 1)].rise_hour } else { today.rise_hour };
                let travelled = if hour - rise < 0.0 { hour - rise + 24.0 } else { hour - rise };
                let angle = if (dph * travelled).abs() < 180.0 { dph * travelled } else { 0.0 };
                assert!(close(st.rotation_from_horizon, angle, 1e-4), "{day} {hour}: {st:?}");
                assert_eq!(st.phase, Phase::from_index(((day + i32::from(post)) / 3) % 8));
                assert_eq!(st.rotation_from_north, s.axis_offset);
                let visible_arc = angle > s.fade_end_angle - s.moon_shadow_early_fade_angle
                    && angle < 180.0 - s.fade_end_angle + s.moon_shadow_early_fade_angle;
                let daylight_hidden = (10.0 - ONE_MINUTE_H..=s.fade_in_start).contains(&hour);
                assert_eq!(st.alpha > 0.0, visible_arc && !daylight_hidden && angle > 0.0, "{day} {hour}: {st:?}");
                let textured = angle >= s.fade_start_angle && angle < 180.0 - s.fade_start_angle;
                assert_eq!(st.shadow_blend == 1.0, textured, "{day} {hour}: {st:?}");
                assert!((0.0..=1.0).contains(&st.alpha) && (0.0..=1.0).contains(&st.shadow_blend));
            }
        }
    }
}

/// Secunda is visible (alpha > 0) for some hours on most nights; printed against Masser.
#[test]
fn secunda_shows_most_nights() {
    let night = |model: &MoonModel, day: i32| -> u32 {
        // 18:00 to 06:00 the next morning, quarter hours.
        (72..144_u16)
            .filter(|&q| {
                let h = f32::from(q) / 4.0;
                let (d, h) = if h >= 24.0 { (day + 1, h - 24.0) } else { (day, h) };
                model.state(d, h).alpha > 0.0
            })
            .count()
            .try_into()
            .unwrap_or(u32::MAX)
    };
    let (m, s) = (masser(), secunda());
    let (mut s_nights, mut m_nights) = (0, 0);
    for day in 1..=24 {
        let (sq, mq) = (night(&s, day), night(&m, day));
        println!("day {day:2}: Secunda {:5.2} h, Masser {:5.2} h", f64::from(sq) / 4.0, f64::from(mq) / 4.0);
        s_nights += u32::from(sq >= 4);
        m_nights += u32::from(mq >= 4);
    }
    println!("nights with >= 1 h visible: Secunda {s_nights}/24, Masser {m_nights}/24");
    assert!(s_nights >= 16, "Secunda visible on only {s_nights} of 24 nights");
}

#[test]
fn direction_follows_skyutil() {
    // Rising (angle 0) on the northern horizon turned by the axis offset toward the west, overhead
    // at 90, setting on the opposite side.
    let st = |angle: f32| MoonState { rotation_from_horizon: angle, rotation_from_north: 50.0, phase: Phase::Full, shadow_blend: 1.0, alpha: 1.0 };
    let rise = st(0.0).direction();
    assert!(close(rise.x, -(50.0_f32.to_radians().sin()), 1e-6) && close(rise.y, 50.0_f32.to_radians().cos(), 1e-6) && rise.z == 0.0);
    assert!(close(st(90.0).direction().z, 1.0, 1e-6));
    let set = st(179.0).direction();
    assert!(set.x > 0.0 && set.y < 0.0 && set.z > 0.0);
    // Elevation along the path equals the angle from the horizon (a great circle through the zenith).
    assert!(close(st(30.0).direction().z.asin().to_degrees(), 30.0, 1e-4));
}

#[test]
fn quad_sizes_follow_sky_cpp() {
    // 450 * 94 / 115 = 367.8 units at 1000: 20.8 deg; Secunda 156.5 units: 8.95 deg.
    assert!(close(quad_angular_size(94.0).to_degrees(), 20.84, 0.01));
    assert!(close(quad_angular_size(40.0).to_degrees(), 8.95, 0.01));
}

fn rising(moon: MoonName) -> MoonModel {
    MoonModel::from_ini_with(moon, None, MoonHorizon::Rise)
}

/// NS-3 owner 2026-11-17 (modern rule): a moon shows, textured, from just above the horizon on
/// both sides of its arc; the classic rule hides it below `Fade End Angle` (Masser 40, Secunda
/// 30). The cycles differ (the handovers move), so the rules are compared at equal angles, then
/// on a real night.
#[test]
fn rise_shows_the_moon_from_the_horizon() {
    for moon in MoonName::ALL {
        let (modern, classic) = (rising(moon), MoonModel::from_ini(moon, None));
        for angle in [1.0, 10.0, 25.0, 155.0, 179.0] {
            assert_eq!((modern.early_moon_shadow_alpha(angle), modern.shadow_blend(angle)), (1.0, 1.0), "{moon:?} {angle}");
            assert_eq!(classic.early_moon_shadow_alpha(angle), 0.0, "{moon:?} {angle}");
        }
        // Some night of the first cycle has it opaque and textured below 15 deg.
        let low = (1..=24).any(|day| {
            (0..96).any(|q| {
                let st = modern.state(day, f32::from(u8::try_from(q).unwrap_or(0)) / 4.0);
                st.rotation_from_horizon > 0.5 && st.rotation_from_horizon < 15.0 && st.alpha == 1.0 && st.shadow_blend == 1.0
            })
        });
        assert!(low, "{moon:?} never seen low");
    }
    let s = rising(MoonName::Secunda);
    // A quarter degree up: half way through the 0.5 deg edge ramp.
    assert!(close(s.early_moon_shadow_alpha(0.25), 0.5, 1e-5));
    assert_eq!((s.early_moon_shadow_alpha(0.0), s.early_moon_shadow_alpha(180.0)), (0.0, 0.0));
    // Noon: the daytime fade still hides it.
    assert_eq!(s.state(1, 12.0).alpha, 0.0);
}

/// The modern rule must not show the rise-hour handover (`handoverHour`, the moon switching to
/// the next day's place on its path): over a whole 24-day cycle, minute by minute, a moon seen in
/// two consecutive minutes moves at most its path speed, both rules.
#[test]
fn a_visible_moon_never_jumps() {
    for horizon in [MoonHorizon::Rise, MoonHorizon::OpenMwFade] {
        for moon in MoonName::ALL {
            let model = MoonModel::from_ini_with(moon, None, horizon);
            let step_deg = model.degrees_per_hour() / 60.0;
            let mut last: Option<(glam::Vec3, f32)> = None;
            let mut seen = 0_u32;
            for minute in 0..24 * 24 * 60 {
                let (day, hour) = (1 + minute / (24 * 60), f32::from(u16::try_from(minute % (24 * 60)).unwrap_or(0)) / 60.0);
                let state = model.state(day, hour);
                let now = (state.direction(), state.alpha);
                if let Some((dir, alpha)) = last
                    && alpha > 0.0
                    && now.1 > 0.0
                {
                    seen += 1;
                    let moved = dir.angle_between(now.0).to_degrees();
                    assert!(moved <= step_deg * 1.5 + 1e-3, "{horizon:?} {moon:?} day {day} {hour:.3} h: jumped {moved:.2} deg");
                }
                last = Some(now);
            }
            assert!(seen > 0, "{horizon:?} {moon:?} never visible");
        }
    }
}

#[test]
#[ignore = "measurement: NS-3 both moons over the owner's 2026-11-17 night (clock day 172 from 22:00), modern vs classic"]
fn owner_night_both_moons() {
    for moon in MoonName::ALL {
        let (modern, classic) = (rising(moon), MoonModel::from_ini(moon, None));
        for q in 0..((31 - 22) * 4) {
            let (day, hour) = game_day_hour(172.0 + (22.0 + f64::from(q) * 0.25) / 24.0);
            let (m, c) = (modern.state(day, hour), classic.state(day, hour));
            println!(
                "{moon:?} day {day} {hour:5.2} h: modern up {:6.2} deg {:?} alpha {:.2} blend {:.2} | classic up {:6.2} deg {:?} alpha {:.2} blend {:.2}",
                m.direction().z.asin().to_degrees(),
                m.phase,
                m.alpha,
                m.shadow_blend,
                c.direction().z.asin().to_degrees(),
                c.phase,
                c.alpha,
                c.shadow_blend
            );
        }
    }
}
