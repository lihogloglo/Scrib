//! Player movement feel ([`crate::feel`], visual check PL-4): sanity of the player's motion
//! springs, and the measurement (ignored, prints) of starts, stops and turns before
//! (`MoverParams::default()`, NPC springs) and after (the player's).
#![allow(clippy::cast_precision_loss)]

use std::f32::consts::PI;

use bevy_math::Vec3;
use sf_action::CharacterParams;
use sf_controller::{Gait, MotionState, MoverParams, SpringParams, forward, wrap_angle};

use crate::feel;

const DT: f32 = 1.0 / 60.0;
const RUN: f32 = 4.5;

/// The stick's goal velocity toward `dir_yaw` at `gait` for a body facing `yaw`: before the
/// feel change `sf_action` scaled it by the gait's direction table against the current
/// facing (`legacy`), now it asks for the forward speed.
fn goal(gait: Gait, dir_yaw: f32, yaw: f32, legacy: bool) -> Vec3 {
    let speeds = *CharacterParams::default().gaits.get(gait);
    speeds.velocity(forward(dir_yaw), if legacy { yaw } else { dir_yaw })
}

fn running(speed: f32) -> MotionState {
    MotionState { velocity: Vec3::Z * speed, ..MotionState::facing(0.0) }
}

/// Seconds until `done`, stepping toward the stick direction `dir_yaw` (None: released).
fn until(
    m: &mut MotionState,
    springs: &SpringParams,
    gait: Gait,
    dir_yaw: Option<f32>,
    legacy: bool,
    mut done: impl FnMut(&MotionState) -> bool,
) -> (f32, f32, f32) {
    let (mut t, mut dist, mut min_speed) = (0.0, 0.0, f32::MAX);
    for _ in 0..600 {
        if done(m) {
            break;
        }
        let (v, yaw_goal) = match dir_yaw {
            Some(d) => (goal(gait, d, m.yaw, legacy), d),
            None => (Vec3::ZERO, m.facing_goal),
        };
        let d = m.advance(v, yaw_goal, gait, springs, DT);
        assert!(d.is_finite() && m.velocity.is_finite() && m.yaw.is_finite(), "NaN");
        dist += d.length();
        min_speed = min_speed.min(m.velocity.length());
        t += DT;
    }
    (t, dist, min_speed)
}

struct Feel {
    start_90: [f32; 3],
    stop_t: f32,
    stop_d: f32,
    reverse_t: f32,
    reverse_min_speed: f32,
    turn90_t: f32,
}

fn measure(springs: &SpringParams, legacy: bool) -> Feel {
    let gaits = CharacterParams::default().gaits;
    let start_90 = [Gait::Walk, Gait::Run, Gait::Sprint].map(|g| {
        let target = gaits.get(g).forward;
        until(&mut MotionState::facing(0.0), springs, g, Some(0.0), legacy, |m| m.velocity.length() >= 0.9 * target).0
    });
    // Stopped: under 0.1 m/s.
    let (stop_t, stop_d, _) = until(&mut running(RUN), springs, Gait::Run, None, legacy, |m| m.velocity.length() < 0.1);
    // Arrived: facing within 5 deg and 90 % of run speed along the new direction.
    let arrived = |dir: f32| {
        move |m: &MotionState| {
            wrap_angle(m.yaw - dir).abs() < 5f32.to_radians() && m.velocity.dot(forward(dir)) >= 0.9 * RUN
        }
    };
    let (reverse_t, _, reverse_min_speed) = until(&mut running(RUN), springs, Gait::Run, Some(PI), legacy, arrived(PI));
    let (turn90_t, _, _) = until(&mut running(RUN), springs, Gait::Run, Some(PI / 2.0), legacy, arrived(PI / 2.0));
    Feel { start_90, stop_t, stop_d, reverse_t, reverse_min_speed, turn90_t }
}

#[test]
fn player_motion_is_sane() {
    let f = measure(&feel::springs(), false);
    for t in f.start_90.into_iter().chain([f.stop_t, f.reverse_t, f.turn90_t]) {
        assert!(t > 0.0 && t < 2.0, "never arrived: {t}");
    }
    assert!(f.stop_d > 0.0 && f.stop_d < 3.0);
    // Only the springs and gravity differ from the generic shape.
    let generic = MoverParams::default();
    assert_eq!(MoverParams { springs: generic.springs, gravity: generic.gravity, ..feel::mover_params() }, generic);
}

#[test]
#[ignore = "measurement: player movement feel before/after (PL-4)"]
fn measure_player_feel() {
    for (name, springs, legacy) in [("before (NPC springs)", MoverParams::default().springs, true), ("after (player feel)", feel::springs(), false)]
    {
        let f = measure(&springs, legacy);
        println!(
            "{name}: 90% speed walk {:.2} s, run {:.2} s, sprint {:.2} s | stop from run {:.2} s over {:.2} m | \
             180 reversal at run {:.2} s (min speed {:.2} m/s) | 90 turn at run {:.2} s",
            f.start_90[0], f.start_90[1], f.start_90[2], f.stop_t, f.stop_d, f.reverse_t, f.reverse_min_speed, f.turn90_t
        );
    }
}
