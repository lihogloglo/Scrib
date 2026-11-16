//! NS-3: the moons the framework receives from the `OpenMW` moon model.
#![allow(clippy::float_cmp, reason = "exact values: OpenMW's piecewise results and clamped opacities")]

use super::*;
use sf_sky_bevy::sf_sky::{MoonState, SkyState, Weather};

fn settings() -> SkySettings {
    let mut s = SkySettings::default();
    apply(&mut s);
    s
}

fn at(s: &SkySettings, day: f64, hour: f64) -> SkyState {
    s.evaluate(&SkyClock::at(day, hour, &s.calendar), &Weather::default())
}

/// Radiance of the full disk per unit tint and transmittance, nits (`sf_sky_bevy` night pass:
/// full illuminance over the drawn solid angle).
fn disk_radiance(m: &MoonState) -> f32 {
    let r = m.angular_size * 0.5;
    m.full_illuminance / (std::f32::consts::PI * r * r)
}

#[test]
fn masser_is_bigger_and_both_have_eight_phases() {
    let s = settings();
    assert_eq!(s.moons.len(), 2);
    assert!(s.moons[0].angular_size > 2.0 * s.moons[1].angular_size);
    assert!((s.moons[0].angular_size.to_degrees() - 20.84).abs() < 0.01, "{}", s.moons[0].angular_size.to_degrees());
    assert!((s.moons[1].angular_size.to_degrees() - 8.95).abs() < 0.01);
    assert!(s.moons.iter().all(|m| m.textures.len() == 8 && m.phase_steps == 8 && matches!(m.path, MoonPath::Driven(_))));
    assert_eq!(s.moons[0].textures[4], "textures/tx_masser_full.dds");
    assert_eq!(s.moons[1].textures[2], "textures/tx_secunda_half_wax.dds");
}

/// The framework gets a visible Secunda: clock day 162 (game day 163, first quarter) at 21:30
/// (`ns-moons-quarter`'s hour), and the first night of a new game at 01:00.
#[test]
fn framework_receives_a_visible_secunda() {
    let s = settings();
    let sky = at(&s, 162.0, 21.5);
    let secunda = sky.night.moons()[1];
    assert!(secunda.position.elevation.to_degrees() > 60.0, "{secunda:?}");
    assert!(secunda.alpha == 1.0 && secunda.texture_blend == 1.0 && secunda.angular_size > 0.0);
    assert_eq!(secunda.frame, Some(2));
    assert_eq!(s.moons[1].textures[2], "textures/tx_secunda_half_wax.dds");
    assert!(secunda.full_illuminance > 0.0 && secunda.transmittance.min_element() > 0.0);
    // New game (16 Last Seed, clock day 0): both full, Secunda up by 01:00 of the next day.
    let first = at(&s, 1.0, 1.0).night.moons()[1];
    assert!(first.alpha == 1.0 && first.position.elevation.to_degrees() > 40.0 && first.frame == Some(4), "{first:?}");
}

/// The NS-3 cause, as a test that fails on the earlier circuit (highest 40 deg, mostly low in the
/// east at the checked hours): Secunda climbs clear of Balmora's valley sides (45 deg), opaque and
/// textured, between 18:00 and 06:00 on at least half the nights of the cycle (the rest: its high
/// arc falls in the day, `OpenMW`'s 24-day rise-hour cycle).
#[test]
fn secunda_climbs_high_on_most_nights() {
    let s = settings();
    let high_nights = (0..24)
        .filter(|&day| {
            (0..48).any(|q| {
                let m = at(&s, f64::from(day), 18.0 + f64::from(q) * 0.25).night.moons()[1];
                m.alpha == 1.0 && m.texture_blend == 1.0 && m.position.elevation.to_degrees() >= 45.0
            })
        })
        .count();
    println!("Secunda opaque at >= 45 deg on {high_nights} of 24 nights");
    assert!(high_nights >= 12, "Secunda high on only {high_nights} of 24 nights");
}

/// The disks keep the radiance they had (larger disks, same nits per texel), and Secunda's is not
/// below Masser's.
#[test]
fn secunda_disk_is_as_bright_as_massers() {
    let sky = at(&settings(), 1.0, 2.0);
    let (masser, secunda) = (sky.night.moons()[0], sky.night.moons()[1]);
    let (rm, rs) = (disk_radiance(&masser), disk_radiance(&secunda));
    // The earlier disks: 0.8 lx over 9.4 deg, 0.2 lx over 4.0 deg.
    let earlier = |lux: f32, deg: f32| lux / (std::f32::consts::PI * (deg.to_radians() * 0.5).powi(2));
    assert!((rm / earlier(0.8, 9.4) - 1.0).abs() < 1e-3 && (rs / earlier(0.2, 4.0) - 1.0).abs() < 1e-3, "{rm} {rs}");
    assert!(rs >= rm);
}

/// The earlier circuit (`MoonPath::Circuit`, removed for NS-3) at the hours the owner checked
/// against the port.
#[test]
#[ignore = "measurement: NS-3 Secunda's elevation at the checked scenes, earlier circuit vs OpenMW model"]
fn old_circuit_put_secunda_at_the_horizon() {
    use sf_sky_bevy::sf_sky::{ExposureRange, KeyCurve, luminance};
    // The game's night calibration (`mw_assets::LightCalibration`: floor 4 nits, EV floor 3.5).
    let mut new = settings();
    new.exposure_range = ExposureRange { min_ev100: 3.5, ..ExposureRange::default() };
    new.ambient = KeyCurve::new(vec![(-12.0, 4.0), (-4.0, 0.0)]);
    let mut old = new.clone();
    for (moon, (inc, ha, tilt)) in old.moons.iter_mut().zip([(1.0_f32, 0.0_f32, 35.0_f32), (1.2, -30.0, 50.0)]) {
        moon.path = MoonPath::Circuit { circuit_days: 1.0 + inc / 24.0, hour_angle_at_epoch_deg: ha, tilt_deg: tilt, declination_deg: 0.0 };
    }
    // Exposed full-disk value per unit texel (1 = white before tonemapping); the brightest texels
    // are about 0.14 (Masser) and 0.29 (Secunda) linear, the means 0.012 and 0.03.
    let shown = |m: &MoonState, ev100: f32| disk_radiance(m) * luminance(m.transmittance) * m.alpha * m.texture_blend / (1.2 * 2f32.powf(ev100));
    println!("scene                     day  hour  sun_deg  ev100 | old Masser  old Secunda | new Masser (alpha, shown)  new Secunda (alpha, shown)");
    for (scene, day, hour) in [
        ("ns-dusk-stars", 0.0, 20.5),
        ("ns-night-clear", 0.0, 21.5),
        ("ro16-terrain-veil --hour 23", 0.0, 23.0),
        ("ns-moons-full", 168.0, 21.5),
        ("ns-moons-quarter", 162.0, 21.5),
        ("first night 01:00", 1.0, 1.0),
    ] {
        let (o, n) = (at(&old, day, hour), at(&new, day, hour));
        let deg = |m: &MoonState| m.position.elevation.to_degrees();
        let (om, os, nm, ns) = (o.night.moons()[0], o.night.moons()[1], n.night.moons()[0], n.night.moons()[1]);
        println!(
            "{scene:26}{day:4} {hour:5} {:7.1} {:6.2} | {:8.1}deg {:9.1}deg | {:8.1}deg ({:.2}, {:6.2}) {:9.1}deg ({:.2}, {:6.2})",
            n.celestial.sun.elevation_deg(), n.ev100, deg(&om), deg(&os), deg(&nm), nm.alpha, shown(&nm, n.ev100), deg(&ns), ns.alpha, shown(&ns, n.ev100)
        );
    }
}
