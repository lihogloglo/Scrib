//! Launches (jumps, knock-ups): the body leaves the ground, flies a ballistic arc, bumps
//! its head on ceilings and lands only on the way down (the A2 prerequisite found in
//! `sf_action` A1: a rising body used to re-snap to the ground within `step_up`).

use super::*;

/// What `sf_action`'s `ActionOutput::launch` becomes in the shell. (These tests first ran
/// against the raw fields, `vertical_speed = speed; grounded = None`: every one re-snapped.)
fn jump(loco: &mut Locomotion, speed: f32) {
    loco.launch(speed);
}

fn standing(world: &World, at: Vec3, params: &MoverParams) -> Locomotion {
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(at), 0.0);
    walk(world, &mut loco, LocomotionIntent::IDLE, params, 2, |_| {});
    assert!(loco.grounded.is_some());
    loco
}

/// Steps until grounded (or `frames` run out); the airborne frames' feet heights and
/// vertical speeds, and the landing frame.
fn fly(world: &World, loco: &mut Locomotion, intent: LocomotionIntent, params: &MoverParams, frames: usize) -> (Vec<(f32, f32)>, Option<usize>) {
    let mut mover = KinematicMover::new();
    let mut arc = Vec::new();
    for i in 1..=frames {
        mover.step(MoveInput { loco: &mut *loco, intent, params, root: RootMotion::default() }, world, DT);
        if loco.grounded.is_some() {
            return (arc, Some(i));
        }
        arc.push((xyz(loco.position).y, loco.vertical_speed));
    }
    (arc, None)
}

#[test]
fn a_launch_leaves_the_ground_on_a_ballistic_arc_and_lands() {
    let world = World::new();
    let params = MoverParams::default();
    let mut loco = standing(&world, Vec3::ZERO, &params);
    let (v0, g) = (5.0, params.gravity);
    jump(&mut loco, v0);
    let (arc, landed) = fly(&world, &mut loco, LocomotionIntent::IDLE, &params, 120);
    assert!(arc.len() > 30, "re-snapped after {} airborne frames", arc.len());
    for (i, &(y, vy)) in arc.iter().enumerate() {
        // Constant gravity integrates exactly (trapezoid rule).
        let t = (i + 1) as f32 * DT;
        assert!((y - (v0 * t - 0.5 * g * t * t)).abs() < 1e-3, "frame {i}: y {y}");
        assert!((vy - (v0 - g * t)).abs() < 1e-3, "frame {i}: vy {vy}");
    }
    let apex = arc.iter().map(|a| a.0).fold(0.0, f32::max);
    assert!((apex - v0 * v0 / (2.0 * g)).abs() < 0.02, "apex {apex}");
    let t = landed.expect("lands") as f32 * DT;
    assert!((t - 2.0 * v0 / g).abs() < 2.0 * DT, "air time {t}");
    assert!(xyz(loco.position).y.abs() < 1e-4 && loco.vertical_speed == 0.0);
}

#[test]
fn a_running_jump_keeps_its_horizontal_speed() {
    let world = World::new();
    let params = MoverParams::default();
    let mut loco = standing(&world, Vec3::ZERO, &params);
    let run = toward(Vec3::new(0.0, 0.0, 4.0));
    walk(&world, &mut loco, run, &params, 120, |_| {});
    let z0 = xyz(loco.position).z;
    jump(&mut loco, 4.0);
    let (arc, landed) = fly(&world, &mut loco, run, &params, 120);
    let frames = landed.expect("lands");
    assert!(arc.len() > 30, "re-snapped after {} airborne frames", arc.len());
    let dz = xyz(loco.position).z - z0;
    let expect = 4.0 * frames as f32 * DT;
    assert!((dz - expect).abs() < 0.05, "{dz} vs {expect}");
}

#[test]
fn a_launch_under_a_ceiling_bumps_its_head_and_falls_back() {
    let mut world = World::new();
    world.add_box(Vec3::new(-2.0, 2.5, -2.0), Vec3::new(2.0, 3.0, 2.0));
    let params = MoverParams::default();
    let mut loco = standing(&world, Vec3::ZERO, &params);
    let v0 = 6.0;
    jump(&mut loco, v0);
    let (arc, landed) = fly(&world, &mut loco, LocomotionIntent::IDLE, &params, 120);
    let apex = arc.iter().map(|a| a.0).fold(0.0, f32::max);
    // Head (feet + 1.8 m) stops at the 2.5 m ceiling, less the skin.
    assert!(apex > 0.6 && apex <= 0.7 + 1e-4, "apex {apex}");
    let bump = arc.iter().position(|a| a.0 >= apex - 1e-6).unwrap();
    assert!(arc[bump].1 <= 0.0, "upward speed kept after the bump: {}", arc[bump].1);
    // Falls from the ceiling, well before a free arc would end.
    assert!((landed.expect("lands") as f32 * DT) < 2.0 * v0 / params.gravity - 0.3);
    assert!(xyz(loco.position).y.abs() < 1e-4);
}

#[test]
fn a_running_jump_lands_on_a_box_taller_than_step_up() {
    let mut world = World::new();
    world.add_box(Vec3::new(-2.0, 0.0, 1.5), Vec3::new(2.0, 0.8, 6.0));
    let params = MoverParams::default();
    let mut loco = standing(&world, Vec3::new(0.0, 0.0, -4.0), &params);
    let run = toward(Vec3::new(0.0, 0.0, 4.0));
    let mut mover = KinematicMover::new();
    while xyz(loco.position).z < 0.0 {
        mover.step(MoveInput { loco: &mut loco, intent: run, params: &params, root: RootMotion::default() }, &world, DT);
    }
    jump(&mut loco, 5.0);
    let (arc, landed) = fly(&world, &mut loco, run, &params, 120);
    assert!(landed.is_some() && arc.len() > 20, "{} airborne frames", arc.len());
    let at = xyz(loco.position);
    assert!((at.y - 0.8).abs() < 1e-3 && at.z > 1.5, "{at}");
}
