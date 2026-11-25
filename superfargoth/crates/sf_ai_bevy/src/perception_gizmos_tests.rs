//! Slice AI26: the perception gizmos' pure builders (cone edges, arcs, band colours, the noise
//! ring buffer, entry text, the render anchor) and a headless app that runs the collector without
//! a gizmo plugin (nothing drawn, nothing panics).

// Exact assertions on purpose where the maths is exact; trigonometry within 1e-5.
#![allow(clippy::float_cmp)]

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, SQRT_2};

use bevy::prelude::MinimalPlugins;
use bevy::time::{Fixed, Time};
use bevy_color::Color;
use bevy_gizmos::config::GizmoConfigStore;
use bevy_transform::components::GlobalTransform;
use glam::Vec3;
use sf_ai::{AwarenessBand, AwarenessConfig, AwarenessMeter, NoiseKindId, PerceptClock, Stimulus};
use sf_console_bevy::{ConsolePlugin, ConsoleSubmit};

use super::*;
use crate::perception_gizmos::{
    Anchor, NOISE_CAPACITY, RecentNoise, RecentNoises, arc_points, band_color, cone_edges, entry_lines, light_line,
};
use crate::sight_tests::{PLAYER, Walls, mover, observer, step, target, wpos};

fn close(a: Vec3, b: Vec3) -> bool {
    (a - b).length() < 1e-5
}

#[test]
fn cone_edges_are_the_two_half_angle_rays() {
    let eye = Vec3::new(1.0, 2.0, 3.0);
    // Yaw 0 faces +Z; ±90° points along ±X.
    let [left, right] = cone_edges(eye, 0.0, FRAC_PI_2, 10.0);
    assert!(close(left, Vec3::new(11.0, 2.0, 3.0)), "{left}");
    assert!(close(right, Vec3::new(-9.0, 2.0, 3.0)), "{right}");
    // Facing +X, 45° each side, √2 m: the corners (1, 0, ∓1) from the eye.
    let [left, right] = cone_edges(Vec3::ZERO, FRAC_PI_2, FRAC_PI_4, SQRT_2);
    assert!(close(left, Vec3::new(1.0, 0.0, -1.0)), "{left}");
    assert!(close(right, Vec3::new(1.0, 0.0, 1.0)), "{right}");
}

#[test]
fn arc_spans_the_cone_and_closes_at_180_degrees() {
    let c = Vec3::new(5.0, 1.0, -2.0);
    let arc = arc_points(c, 0.3, FRAC_PI_4, 7.0);
    assert_eq!(arc.len(), 13, "a quarter turn: 12 of the 48 full-circle segments");
    let [left, right] = cone_edges(c, 0.3, FRAC_PI_4, 7.0);
    assert!(close(arc[0], right) && close(arc[12], left));
    for p in &arc {
        assert_eq!(p.y, c.y);
        assert!(((*p - c).length() - 7.0).abs() < 1e-4);
    }
    let full = arc_points(c, 0.3, PI, 7.0);
    assert_eq!(full.len(), 49);
    assert_eq!(full[0], full[48]);
    assert_eq!(arc_points(c, 0.0, 4.0, 7.0).len(), 49, "wider than 180° clamps to the circle");
    let line = arc_points(c, 0.0, 0.0, 7.0);
    assert_eq!(line.len(), 3, "a zero cone still draws (two segments on one point)");
}

#[test]
fn every_band_has_its_own_colour() {
    let bands = [
        (AwarenessBand::Unaware, Color::srgb(0.6, 0.6, 0.6)),
        (AwarenessBand::Curious, Color::srgb(1.0, 0.9, 0.2)),
        (AwarenessBand::Suspicious, Color::srgb(1.0, 0.6, 0.1)),
        (AwarenessBand::Searching, Color::srgb(1.0, 0.35, 0.1)),
        (AwarenessBand::Alerted, Color::srgb(1.0, 0.1, 0.1)),
    ];
    for (band, color) in bands {
        assert_eq!(band_color(band), color, "{band:?}");
    }
    for (i, (a, _)) in bands.iter().enumerate() {
        for (b, _) in &bands[i + 1..] {
            assert_ne!(band_color(*a), band_color(*b), "{a:?} vs {b:?}");
        }
    }
}

fn noise(tick: u64) -> RecentNoise {
    RecentNoise { tick, at: wpos(Vec3::ZERO), loudness_m: 10.0, reveals_source: true }
}

#[test]
fn recent_noises_keep_only_the_window_and_cap_at_capacity() {
    let mut n = RecentNoises::default();
    for t in 0..6 {
        n.push(noise(t));
    }
    // 0.5 s ticks, 2 s window: at tick 6 only ticks 3, 4 and 5 are younger than 2 s.
    n.prune(6, 0.5);
    assert_eq!(n.iter().map(|x| x.tick).collect::<Vec<_>>(), [3, 4, 5]);

    let mut n = RecentNoises::default();
    for t in 0..40 {
        n.push(noise(t));
    }
    assert_eq!(n.iter().len(), NOISE_CAPACITY);
    assert_eq!(n.iter().next().unwrap().tick, 40 - NOISE_CAPACITY as u64, "the oldest go first");
}

#[test]
fn entry_lines_show_band_level_lkp_and_hold() {
    let cfg = AwarenessConfig::default();
    let mut meter = AwarenessMeter::new();
    let seen = PerceptClock::new(10, 0.5);
    meter.apply(&cfg, &Stimulus::Sight { target: PLAYER, gain: 0.3, pos: wpos(Vec3::new(0.0, 0.0, 12.0)), velocity: Vec3::ZERO }, seen);
    let e = *meter.get(PLAYER).unwrap();
    let pct = (e.level * 100.0).round();
    let feet = wpos(Vec3::new(0.0, 0.0, 2.0));
    // Two ticks of 0.5 s later.
    assert_eq!(
        entry_lines(&e, feet, 12, 0.5),
        [format!("{:?} {pct}%", e.band), "LKP 10.0 m, conf 1.00, 1.0 s ago".to_string()]
    );
    assert!(meter.hold_searching(&cfg, PLAYER, PerceptClock::new(12, 0.5)));
    let held = entry_lines(meter.get(PLAYER).unwrap(), feet, 12, 0.5);
    assert_eq!(held.len(), 3);
    assert_eq!(held[2], "search held");
    assert_eq!(light_line(0.424), "light 42%");
    assert_eq!(light_line(3.0), "light 100%");
}

#[test]
fn anchor_maps_world_positions_through_the_movers_render_position() {
    let m = mover(Vec3::new(300.0, 0.0, 0.0), 0.0);
    let a = Anchor::of(&m, &GlobalTransform::from_translation(Vec3::new(5.0, 0.0, 0.0)));
    let off = m.loco.render_position().delta_f32(m.loco.position);
    assert!(close(a.to_render(wpos(Vec3::new(300.0, 0.0, 10.0))), Vec3::new(5.0, 0.0, 10.0) + off));
}

#[test]
fn headless_app_collects_noises_and_draws_nothing() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin, AiPlugin { timings: false }));
    app.insert_resource(Time::<Fixed>::from_hz(30.0));
    app.init_resource::<Walls>();
    app.add_plugins(AiSightPlugin::<Walls>::default());
    observer(&mut app, 1, Vec3::ZERO);
    target(&mut app, Vec3::new(0.0, 0.0, 5.0));
    let ev = NoiseEvent {
        tick: 0,
        at: wpos(Vec3::new(0.0, 0.0, 3.0)),
        space: None,
        kind: NoiseKindId(0),
        loudness_m: 12.0,
        source: Some(PLAYER),
        reveals_source: true,
        notify: None,
    };

    // Off by default: nothing collected.
    app.world_mut().write_message(ev.clone());
    step(&mut app);
    assert_eq!(app.world().resource::<RecentNoises>().iter().len(), 0);

    app.world_mut().write_message(ConsoleSubmit::exec("ai.perception on"));
    app.update();
    app.world_mut().write_message(ev);
    step(&mut app);
    let kept: Vec<_> = app.world().resource::<RecentNoises>().iter().copied().collect();
    assert_eq!(kept.len(), 1);
    assert_eq!((kept[0].loudness_m, kept[0].reveals_source), (12.0, true));

    // PostUpdate runs every frame; without Bevy's gizmo plugin the draw system never does.
    for _ in 0..3 {
        app.update();
    }
    assert!(!app.world().contains_resource::<GizmoConfigStore>());
}
