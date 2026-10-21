//! Unit tests (locomotion.md "Verification").
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::too_many_lines)]

use std::f32::consts::{FRAC_PI_2, FRAC_PI_6, PI};
use std::sync::Arc;

use glam::{Affine3A, IVec3, Quat, Vec3};
use sf_anim::RootDelta;
use sf_coords::{WorldPos, WorldTransform};

use crate::*;

// --- Fixtures ----------------------------------------------------------------------------

fn p(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::new(IVec3::ZERO, Vec3::new(x, y, z))
}

fn xyz(w: WorldPos) -> Vec3 {
    WorldPos::ORIGIN.delta_f32(w)
}

/// Flat ground at height `0`.
struct Flat(f32);

impl HeightSurface for Flat {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(self.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        let oy = xyz(origin).y - self.0;
        if dir.y.abs() < 1e-6 {
            return None;
        }
        let t = -oy / dir.y;
        (0.0..=max).contains(&t).then_some((t, Vec3::Y))
    }
}

fn box_mesh(h: Vec3, surface: SurfaceId) -> CollisionMesh {
    let c = |x: f32, y: f32, z: f32| [x * h.x, y * h.y, z * h.z];
    let corners = [
        c(-1.0, -1.0, -1.0),
        c(1.0, -1.0, -1.0),
        c(1.0, 1.0, -1.0),
        c(-1.0, 1.0, -1.0),
        c(-1.0, -1.0, 1.0),
        c(1.0, -1.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(-1.0, 1.0, 1.0),
    ];
    let idx = [0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5];
    let mut b = CollisionBuilder::new(1e-3);
    b.add(&corners, &idx, Affine3A::IDENTITY, surface);
    b.finish()
}

fn shape(mesh: &CollisionMesh) -> Arc<StaticShape> {
    Arc::new(StaticShape::new(mesh).unwrap())
}

/// Terrain plus statics.
struct World {
    terrain: TerrainProbe<Flat>,
    statics: StaticWorld,
}

impl World {
    fn new() -> Self {
        Self { terrain: TerrainProbe(Flat(0.0)), statics: StaticWorld::new(WorldPos::ORIGIN) }
    }

    /// An axis-aligned box from `min` to `max`.
    fn add_box(&mut self, min: Vec3, max: Vec3) {
        let s = shape(&box_mesh((max - min) / 2.0, SurfaceId(3)));
        self.statics.insert(&s, WorldTransform::from_pos(WorldPos::ORIGIN.offset((min + max) * 0.5)));
    }

    /// A ramp rising along +X from (x0, 0) at `angle`, 8 m long, z in [-3, 3].
    fn add_ramp(&mut self, x0: f32, angle: f32) {
        self.add_ramp_len(x0, angle, 8.0);
    }

    /// A ramp rising along +X from (x0, 0) at `angle`, `len` m long, z in [-3, 3].
    fn add_ramp_len(&mut self, x0: f32, angle: f32, len: f32) {
        let (half_len, half_thick) = (len / 2.0, 0.5);
        let r = Quat::from_rotation_z(angle);
        let mid_top = Vec3::new(x0 + half_len * angle.cos(), half_len * angle.sin(), 0.0);
        let center = mid_top - r * Vec3::new(0.0, half_thick, 0.0);
        let s = shape(&box_mesh(Vec3::new(half_len, half_thick, 3.0), SurfaceId(4)));
        let pos = WorldPos::ORIGIN.offset(center);
        self.statics.insert(&s, WorldTransform { pos, rotation: r, scale: Vec3::ONE });
    }
}

impl GroundProbe for World {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

const DT: f32 = 1.0 / 60.0;

fn walk(world: &World, loco: &mut Locomotion, intent: LocomotionIntent, params: &MoverParams, frames: usize, mut each: impl FnMut(&Locomotion)) {
    let mut mover = KinematicMover::new();
    for _ in 0..frames {
        mover.step(MoveInput { loco, intent, params, root: RootMotion::default() }, world, DT);
        each(loco);
    }
}

fn toward(v: Vec3) -> LocomotionIntent {
    LocomotionIntent { desired_velocity: v, ..LocomotionIntent::IDLE }
}

// --- Springs and trajectory --------------------------------------------------------------

#[test]
fn springs_converge_without_overshoot() {
    let springs = SpringParams::default();
    let mut m = MotionState::facing(0.0);
    let goal = Vec3::new(2.0, 0.0, 0.0);
    let mut last = 0.0;
    for _ in 0..180 {
        m.advance(goal, 0.0, &springs, DT);
        let s = m.velocity.x;
        assert!(s >= last - 1e-6 && s <= 2.0 + 1e-5, "speed {s} after {last}");
        last = s;
    }
    assert!((m.velocity - goal).length() < 1e-3);

    // A 170° turn approaches the goal monotonically and never passes it.
    let mut m = MotionState::facing(0.0);
    let target = 170f32.to_radians();
    let mut last = 0.0;
    for _ in 0..180 {
        m.advance(Vec3::ZERO, target, &springs, DT);
        assert!(m.yaw >= last - 1e-6 && m.yaw <= target + 1e-5, "yaw {} after {last}", m.yaw);
        last = m.yaw;
    }
    assert!((m.yaw - target).abs() < 1e-3);

    // The short way round: from 170° to -170° is a 20° turn through 180°.
    let mut m = MotionState::facing(target);
    for _ in 0..180 {
        m.advance(Vec3::ZERO, -target, &springs, DT);
        assert!(wrap_angle(m.yaw - target) >= -1e-5, "turned the long way: {}", m.yaw);
    }
    assert!(wrap_angle(m.yaw + target).abs() < 1e-3);
}

#[test]
fn springs_are_frame_rate_independent() {
    let springs = SpringParams::default();
    let run = |hz: u32| {
        let mut m = MotionState::facing(0.3);
        let mut x = Vec3::ZERO;
        for _ in 0..hz {
            x += m.advance(Vec3::new(-1.0, 0.0, 3.0), 2.5, &springs, 1.0 / hz as f32);
        }
        (x, m)
    };
    let (x60, m60) = run(60);
    for hz in [30, 144] {
        let (x, m) = run(hz);
        assert!((x - x60).length() < 1e-4, "{hz} Hz: {x} vs {x60}");
        assert!((m.velocity - m60.velocity).length() < 1e-4 && (m.yaw - m60.yaw).abs() < 1e-4);
    }
}

#[test]
fn trajectory_matches_simulation_within_1cm_at_1s() {
    let world = World::new();
    let params = MoverParams::default();
    // Mid-acceleration and mid-turn: a run toward -X after walking toward +Z.
    for jitter in [false, true] {
        let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
        walk(&world, &mut loco, toward(Vec3::new(0.0, 0.0, 1.4)), &params, 60, |_| {});
        let intent = LocomotionIntent { desired_velocity: Vec3::new(-3.5, 0.0, 0.5), desired_facing: Some(-FRAC_PI_2), ..LocomotionIntent::IDLE };
        let mut mover = KinematicMover::new();
        mover.step(MoveInput { loco: &mut loco, intent, params: &params, root: RootMotion::default() }, &world, DT);
        let (start, prediction) = (loco.position, loco.trajectory);
        // Simulate one second; with jitter, frame times vary from 4 ms to 33 ms.
        let mut t = 0.0;
        let mut k = 0;
        let mut checked = 0;
        while checked < TRAJECTORY_TIMES.len() {
            let target = TRAJECTORY_TIMES[checked];
            let dt = if jitter { [0.004, 0.033, 0.0167, 0.009, 0.021][k % 5] } else { DT };
            let dt = dt.min(target - t);
            k += 1;
            mover.step(MoveInput { loco: &mut loco, intent, params: &params, root: RootMotion::default() }, &world, dt);
            t += dt;
            if (t - target).abs() < 1e-6 {
                let actual = start.delta_f32(loco.position);
                let predicted = prediction[checked];
                let err = (actual - predicted.offset).length();
                assert!(err < 0.01, "jitter {jitter}: +{target} s: error {err} m ({actual} vs {})", predicted.offset);
                assert!(wrap_angle(loco.motion.yaw - predicted.yaw).abs() < 1e-3);
                assert!((loco.motion.velocity - predicted.velocity).length() < 1e-3);
                checked += 1;
            }
        }
    }
}

#[test]
fn graph_parameters_read_the_motion_and_trajectory() {
    let world = World::new();
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    // Strafing toward +X while facing +Z.
    let intent = LocomotionIntent { desired_velocity: Vec3::new(1.5, 0.0, 0.0), desired_facing: Some(0.0), ..LocomotionIntent::IDLE };
    walk(&world, &mut loco, intent, &params, 120, |_| {});
    let g = loco.params();
    assert!((g.speed - 1.5).abs() < 0.01 && g.grounded);
    assert!((g.direction - FRAC_PI_2).abs() < 0.01, "{}", g.direction);
    assert!((g.local_velocity.x - 1.5).abs() < 0.01);
    // A turn request shows up in the trajectory before the body turns.
    let turn = LocomotionIntent { desired_velocity: Vec3::ZERO, desired_facing: Some(FRAC_PI_2), ..LocomotionIntent::IDLE };
    walk(&world, &mut loco, turn, &params, 1, |_| {});
    let g = loco.params();
    assert!(g.future_heading_change > 1.3, "{}", g.future_heading_change);
    // Velocity decays as (1 + yt)·e^(-yt): 1.5 m/s leaves ~0.05 m/s after 1 s.
    assert!(g.turn_rate > 0.0 && g.future_speed < 0.1, "{g:?}");
}

#[test]
fn gait_speeds_depend_on_direction() {
    let g = GaitSpeeds { forward: 2.0, side: 1.5, back: 1.0 };
    let yaw = FRAC_PI_2; // facing +X
    assert!((g.velocity(Vec3::X, yaw) - Vec3::new(2.0, 0.0, 0.0)).length() < 1e-5);
    assert!((g.velocity(-Vec3::X, yaw) - Vec3::new(-1.0, 0.0, 0.0)).length() < 1e-5);
    assert!((g.velocity(Vec3::Z, yaw).length() - 1.5).abs() < 1e-5);
    assert!((g.velocity(Vec3::new(5.0, 3.0, 0.0), yaw).length() - 2.0).abs() < 1e-5, "clamped to unit input, Y ignored");
}

// --- Ground following --------------------------------------------------------------------

#[test]
fn steps_up_a_staircase_with_smoothed_height() {
    let mut world = World::new();
    // Five 0.3 m steps, 0.4 m deep, from x = 1; the top one is a landing.
    for k in 0..5 {
        let x = 1.0 + 0.4 * k as f32;
        let end = if k == 4 { 8.0 } else { x + 0.4 };
        world.add_box(Vec3::new(x, 0.0, -2.0), Vec3::new(end, 0.3 * (k + 1) as f32, 2.0));
    }
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
    let mut max_drawn_jump = 0.0f32;
    let mut last_drawn = 0.0;
    walk(&world, &mut loco, toward(Vec3::new(1.2, 0.0, 0.0)), &params, 240, |l| {
        let drawn = xyz(l.render_position()).y;
        max_drawn_jump = max_drawn_jump.max(drawn - last_drawn);
        last_drawn = drawn;
        assert!(l.grounded.is_some(), "fell at {}", xyz(l.position));
    });
    let at = xyz(loco.position);
    assert!(at.x > 3.2, "stuck at {at}");
    assert!((at.y - 1.5).abs() < 1e-3, "on the top step: {at}");
    assert_eq!(loco.grounded.unwrap().surface, SurfaceId(3));
    // 0.3 m steps, but the drawn height never jumps more than a fraction of one.
    assert!(max_drawn_jump < 0.12, "drawn height jumped {max_drawn_jump}");
    assert!(loco.height_offset.abs() < 1e-3);
}

#[test]
fn ledges_taller_than_step_up_block_and_walls_slide() {
    let params = MoverParams::default();
    // A 0.6 m ledge: the sweep's rounded bottom rides its edge (the capsule's
    // horizontal radius there is ~0.26 m), but nothing climbs it.
    let mut world = World::new();
    world.add_box(Vec3::new(1.0, 0.0, -5.0), Vec3::new(2.0, 0.6, 5.0));
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
    walk(&world, &mut loco, toward(Vec3::new(1.5, 0.0, 0.0)), &params, 120, |_| {});
    let at = xyz(loco.position);
    assert!(at.x < 1.0 - 0.25 && at.x > 0.6 && at.y.abs() < 1e-4, "{at}");

    // Diagonally into a wall: blocked along X, keeps going along Z, and the
    // capsule never gets closer than its radius.
    let mut world = World::new();
    world.add_box(Vec3::new(1.0, 0.0, -5.0), Vec3::new(2.0, 3.0, 5.0));
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    walk(&world, &mut loco, toward(Vec3::new(1.0, 0.0, 1.0)), &params, 180, |l| {
        assert!(xyz(l.position).x <= 1.0 - params.radius + 1e-3, "penetrated: {}", xyz(l.position));
    });
    let at = xyz(loco.position);
    assert!(at.x > 1.0 - params.radius - 0.02, "stopped short: {at}");
    assert!(at.z > 2.0, "didn't slide along the wall: {at}");
    assert!(at.y.abs() < 1e-4);
    assert!(loco.motion.velocity.x.abs() < 1e-3, "velocity into the wall was kept");
}

#[test]
fn steps_down_small_drops_and_falls_off_high_ones() {
    let params = MoverParams::default();
    for (height, falls) in [(0.3, false), (2.0, true)] {
        let mut world = World::new();
        world.add_box(Vec3::new(-5.0, 0.0, -2.0), Vec3::new(0.5, height, 2.0));
        let mut loco = Locomotion::new(p(0.0, height, 0.0), FRAC_PI_2);
        let mut airborne = 0;
        walk(&world, &mut loco, toward(Vec3::new(1.5, 0.0, 0.0)), &params, 120, |l| airborne += usize::from(l.grounded.is_none()));
        let at = xyz(loco.position);
        assert!(at.x > 1.5 && at.y.abs() < 1e-4, "height {height}: ended at {at}");
        if falls {
            // A 2 m fall takes about 0.64 s.
            assert!((30..50).contains(&airborne), "airborne {airborne} frames");
        } else {
            assert_eq!(airborne, 0, "a {height} m drop is a step");
        }
    }
}

/// Standing on ground steeper than the slope limit slides down it (gravity
/// along the slope) and stops on the flat; a walkable slope holds.
#[test]
fn unwalkable_slopes_slide() {
    let params = MoverParams::default();
    for (deg, slides) in [(30.0f32, false), (60.0, true)] {
        let mut world = World::new();
        world.add_ramp(1.0, deg.to_radians());
        let start = p(2.0, deg.to_radians().tan(), 0.0);
        let mut loco = Locomotion::new(start, FRAC_PI_2);
        let mut max_slide = 0.0f32;
        walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 180, |l| max_slide = max_slide.max(l.slide.length()));
        let at = xyz(loco.position);
        if slides {
            assert!(at.x < 1.0 && at.y.abs() < 0.02, "{deg}°: ended at {at}");
            assert!(max_slide > 1.0 && loco.slide == Vec3::ZERO, "{deg}°: slide peak {max_slide}, end {}", loco.slide);
        } else {
            assert!((at - xyz(start)).length() < 0.01 && max_slide == 0.0, "{deg}°: moved to {at}");
        }
    }
}

#[test]
fn slopes_steeper_than_the_limit_block() {
    let params = MoverParams::default();
    for (deg, climbs) in [(30.0f32, true), (60.0, false)] {
        let mut world = World::new();
        world.add_ramp(1.0, deg.to_radians());
        let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
        walk(&world, &mut loco, toward(Vec3::new(1.5, 0.0, 0.0)), &params, 180, |_| {});
        let at = xyz(loco.position);
        if climbs {
            let surface = (at.x - 1.0) * deg.to_radians().tan();
            assert!(at.x > 3.0 && (at.y - surface).abs() < 0.02, "{deg}°: at {at}, ramp height {surface}");
            assert_eq!(loco.grounded.unwrap().surface, SurfaceId(4));
        } else {
            assert!(at.x < 1.05 && at.y < 0.05, "{deg}°: climbed to {at}");
        }
    }
    // Into the steep ramp at an angle: blocked uphill, still moving along its contour.
    let mut world = World::new();
    world.add_ramp(1.0, 60f32.to_radians());
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    walk(&world, &mut loco, toward(Vec3::new(1.0, 0.0, 1.0)), &params, 120, |_| {});
    let at = xyz(loco.position);
    assert!(at.x < 1.05 && at.z > 1.0, "{at}");
}

/// A short unwalkable bit (a steep bevel up onto a ledge) whose top is
/// within `step_up` is stepped over like a ledge (UE `StepUp`; `OpenMW`'s
/// stepper skips tiny unwalkable slopes); a taller one still blocks.
#[test]
fn short_steep_bevels_onto_ledges_are_stepped_over() {
    let params = MoverParams::default();
    let angle = 60f32.to_radians();
    for (h, crosses) in [(0.1f32, true), (0.4, true), (0.6, false)] {
        let mut world = World::new();
        world.add_ramp_len(1.0, angle, h / angle.sin());
        let top = 1.0 + h / angle.tan();
        world.add_box(Vec3::new(top, h - 1.0, -3.0), Vec3::new(top + 6.0, h, 3.0));
        let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
        walk(&world, &mut loco, toward(Vec3::new(1.5, 0.0, 0.0)), &params, 180, |_| {});
        let at = xyz(loco.position);
        if crosses {
            assert!(at.x > 3.0 && (at.y - h).abs() < 0.02, "{h} m bevel: at {at}");
        } else {
            assert!(at.x < 1.05, "{h} m bevel: climbed to {at}");
        }
    }
}

/// Orient to movement around obstacles (owner check WK-4): a walker whose
/// path runs into ground too steep to climb, or a wall, at a shallow angle
/// slides along it; with no facing in its intent it turns to face that
/// motion, so its graph is fed a forward walk (not a slow strafe while
/// facing the slope). Head-on it keeps facing its goal.
#[test]
fn deflected_walkers_face_where_they_go() {
    let params = MoverParams::default();
    let asked = |deg: f32| Vec3::new(deg.to_radians().sin(), 0.0, deg.to_radians().cos()) * 2.1;
    let mut slope = World::new();
    slope.add_ramp(1.0, 60f32.to_radians());
    let mut wall = World::new();
    wall.add_box(Vec3::new(1.0, 0.0, -3.0), Vec3::new(1.5, 2.5, 6.0));
    for (name, world) in [("slope", &slope), ("wall", &wall)] {
        // 75° from the contour (+Z): the surface leaves a quarter of the speed.
        let mut loco = Locomotion::new(p(0.6, 0.0, -2.0), 75f32.to_radians());
        walk(world, &mut loco, toward(asked(75.0)), &params, 120, |_| {});
        let local = loco.motion.local_velocity();
        assert!(xyz(loco.position).x < 1.05, "{name}: went through, at {}", xyz(loco.position));
        assert!(local.z > 0.4 && local.x.abs() < 0.1 * local.z, "{name}: local velocity {local} (yaw {})", loco.motion.yaw);
        // Head-on: stands facing the goal.
        let mut loco = Locomotion::new(p(0.6, 0.0, 0.0), FRAC_PI_2);
        walk(world, &mut loco, toward(asked(89.0)), &params, 120, |_| {});
        assert!((loco.motion.yaw - 89f32.to_radians()).abs() < 0.05, "{name}: head-on yaw {}", loco.motion.yaw);
    }
    // An explicit facing (a strafing player) is kept.
    let mut loco = Locomotion::new(p(0.6, 0.0, -2.0), 0.0);
    let intent = LocomotionIntent { desired_facing: Some(FRAC_PI_2), ..toward(asked(75.0)) };
    walk(&slope, &mut loco, intent, &params, 120, |_| {});
    assert!((loco.motion.yaw - FRAC_PI_2).abs() < 0.01, "strafe yaw {}", loco.motion.yaw);
}

#[test]
fn drive_root_motion_moves_the_character() {
    let world = World::new();
    let params = MoverParams::default();
    let mut mover = KinematicMover::new();
    // Facing +X; the clip pushes 2 cm forward and turns 0.01 rad per frame.
    let root = RootMotion { delta: RootDelta::new(Vec3::new(0.0, 0.0, 0.02), 0.01), drive_weight: 1.0, traversal: false };
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
    for _ in 0..30 {
        mover.step(MoveInput { loco: &mut loco, intent: LocomotionIntent::IDLE, params: &params, root }, &world, DT);
    }
    let at = xyz(loco.position);
    assert!((at.length() - 0.6).abs() < 0.01 && at.x > 0.55, "{at}");
    assert!((loco.motion.yaw - (FRAC_PI_2 + 0.3)).abs() < 1e-4);
    // Half weight: half the root motion, half the (zero) spring motion.
    let mut half = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    let root = RootMotion { drive_weight: 0.5, ..root };
    mover.step(MoveInput { loco: &mut half, intent: LocomotionIntent::IDLE, params: &params, root }, &world, DT);
    assert!((xyz(half.position).z - 0.01).abs() < 1e-5);
    // `moved` reports the step in the character's frame (forward = +Z).
    assert!((half.moved.translation - Vec3::new(0.0, 0.0, 0.01)).length() < 1e-5);
}

#[test]
fn separation_pushes_overlapping_characters_apart() {
    let positions = [p(0.0, 0.0, 0.0), p(0.4, 0.0, 0.0), p(5.0, 0.0, 0.0), p(0.0, 3.0, 0.1)];
    let radii = [0.3; 4];
    let mut out = [Vec3::ZERO; 4];
    separate(&positions, &radii, &mut out);
    assert!((out[0] - Vec3::new(-0.1, 0.0, 0.0)).length() < 1e-5 && (out[1] - Vec3::new(0.1, 0.0, 0.0)).length() < 1e-5);
    assert_eq!(out[2], Vec3::ZERO, "far away");
    assert_eq!(out[3], Vec3::ZERO, "a floor above");
}

// --- Probes and the static world ---------------------------------------------------------

#[test]
fn batched_probes_match_single_queries() {
    let mut world = World::new();
    world.add_box(Vec3::new(1.0, 0.0, -1.0), Vec3::new(2.0, 1.0, 1.0));
    world.add_ramp(-6.0, FRAC_PI_6);
    let s = shape(&box_mesh(Vec3::new(0.5, 0.5, 0.5), SurfaceId(9)));
    world.statics.insert(&s, WorldTransform { pos: p(0.0, 0.5, 4.0), rotation: Quat::from_rotation_y(0.7), scale: Vec3::splat(2.0) });
    let mut queries = Vec::new();
    for i in 0..40 {
        let x = -8.0 + i as f32 * 0.4;
        queries.push(ProbeQuery::down(p(x, 3.0, 0.0), 5.0));
        queries.push(ProbeQuery::sphere(p(x, 3.0, 0.3), Vec3::NEG_Y, 5.0, 0.2));
        queries.push(ProbeQuery::capsule(p(x, 0.9, 4.0 - x * 0.1), Vec3::new(1.0, 0.0, 0.3).normalize(), 3.0, 0.3, 0.4));
        queries.push(ProbeQuery::ray(p(x, 0.5, -3.0), Vec3::new(0.0, -0.2, 1.0).normalize(), 10.0));
    }
    let mut batched = vec![None; queries.len()];
    world.probe(&queries, &mut batched);
    let hits = batched.iter().filter(|h| h.is_some()).count();
    assert!(hits > queries.len() / 2, "only {hits} hits");
    for (q, b) in queries.iter().zip(&batched) {
        assert_eq!(world.probe_one(*q), *b);
    }
}

#[test]
fn static_world_hits_placed_scaled_rotated_shapes() {
    let s = shape(&box_mesh(Vec3::new(0.5, 0.5, 0.5), SurfaceId(9)));
    // Far from the origin, in another grid cell.
    let base = WorldPos::new(IVec3::new(40, 0, -12), Vec3::new(10.0, 0.0, 10.0));
    let mut w = StaticWorld::new(base.offset(Vec3::new(50.0, 0.0, 0.0)));
    let id = w.insert(&s, WorldTransform { pos: base, rotation: Quat::from_rotation_y(0.4), scale: Vec3::splat(2.0) });
    let tall = w.insert(&s, WorldTransform { pos: base.offset(Vec3::new(5.0, 0.0, 0.0)), rotation: Quat::IDENTITY, scale: Vec3::new(1.0, 4.0, 1.0) });
    assert_eq!(w.len(), 2);
    // Down onto the scaled cube's top (1 m above its center).
    let hit = w.probe_one(ProbeQuery::down(base.offset(Vec3::new(0.2, 5.0, 0.1)), 10.0)).unwrap();
    assert!((hit.distance - 4.0).abs() < 1e-4 && hit.normal.y > 0.999 && hit.surface == SurfaceId(9), "{hit:?}");
    // The non-uniformly scaled one is 4 m tall.
    let hit = w.probe_one(ProbeQuery::down(base.offset(Vec3::new(5.0, 5.0, 0.0)), 10.0)).unwrap();
    assert!((hit.distance - 3.0).abs() < 1e-4, "{hit:?}");
    // A sphere swept sideways stops a radius short of the face.
    let hit = w.probe_one(ProbeQuery::sphere(base.offset(Vec3::new(5.0, 0.0, -3.0)), Vec3::Z, 5.0, 0.25)).unwrap();
    assert!((hit.distance - 2.25).abs() < 1e-3 && hit.normal.z < -0.999, "{hit:?}");
    assert!((base.delta_f32(hit.point) - Vec3::new(5.0, 0.0, -0.5)).length() < 1e-3, "{:?}", base.delta_f32(hit.point));
    // Rebase keeps answers; removal takes the shape out.
    let before = w.probe_one(ProbeQuery::down(base.offset(Vec3::new(0.2, 5.0, 0.1)), 10.0));
    w.rebase(base);
    assert_eq!(w.probe_one(ProbeQuery::down(base.offset(Vec3::new(0.2, 5.0, 0.1)), 10.0)).map(|h| h.surface), before.map(|h| h.surface));
    assert!(w.remove(id) && !w.remove(id));
    assert!(w.probe_one(ProbeQuery::down(base.offset(Vec3::new(0.2, 5.0, 0.1)), 10.0)).is_none());
    assert!(w.remove(tall) && w.is_empty());
}

#[test]
fn terrain_probe_rests_spheres_on_slopes() {
    struct Plane(Vec3);
    impl HeightSurface for Plane {
        fn height(&self, p: WorldPos) -> Option<f32> {
            let q = xyz(p);
            Some(-(self.0.x * q.x + self.0.z * q.z) / self.0.y)
        }
        fn normal(&self, _p: WorldPos) -> Option<Vec3> {
            Some(self.0)
        }
        fn raycast(&self, _o: WorldPos, _d: Vec3, _m: f32) -> Option<(f32, Vec3)> {
            None
        }
    }
    let n = Vec3::new(-0.5, 1.0, 0.0).normalize();
    let t = TerrainProbe(Plane(n));
    let hit = t.probe_one(ProbeQuery::sphere(p(2.0, 5.0, 0.0), Vec3::NEG_Y, 10.0, 0.5)).unwrap();
    // The sphere's center rests exactly one radius from the plane.
    let center = xyz(p(2.0, 5.0, 0.0)) - Vec3::Y * hit.distance;
    let plane_dist = center.dot(n);
    assert!((plane_dist - 0.5).abs() < 1e-4, "{plane_dist}");
    assert!((xyz(hit.point).dot(n)).abs() < 1e-4);
    assert!(t.probe_one(ProbeQuery::down(p(2.0, 5.0, 0.0), 3.0)).is_none(), "out of reach");
}

// --- Collision meshes --------------------------------------------------------------------

#[test]
fn collision_builder_welds_and_drops_bad_triangles() {
    // Two quads sharing an edge, split per face like a render mesh, plus a
    // degenerate and a duplicate (reversed) triangle.
    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0002], // welds to [1, 0, 0]
        [2.0, 0.0, 0.0],
        [2.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [5.0, 0.0, 5.0],
    ];
    let indices = [0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 0, 8, 8, 2, 1, 0];
    let mut b = CollisionBuilder::new(1e-3);
    b.add(&positions, &indices, Affine3A::from_translation(Vec3::Y), SurfaceId(2));
    let mesh = b.finish();
    assert_eq!(mesh.triangles.len(), 4);
    assert_eq!(mesh.positions.len(), 6, "{:?}", mesh.positions);
    assert!(mesh.positions.iter().all(|p| p[1] == 1.0));
    assert_eq!(mesh.surface(3), SurfaceId(2));
    let bytes = mesh.to_bytes();
    assert_eq!(CollisionMesh::from_bytes(&bytes).unwrap(), mesh);
    assert_eq!(CollisionMesh::from_bytes(b"nope0000"), Err(FormatError::Magic));
    let mut wrong = bytes.clone();
    wrong[4] = 99;
    assert!(matches!(CollisionMesh::from_bytes(&wrong), Err(FormatError::Version { .. })));
    assert!(StaticShape::new(&CollisionMesh::default()).is_none());
}

// --- Path following ----------------------------------------------------------------------

#[test]
fn path_follower_cuts_corners_waits_and_finishes() {
    let world = World::new();
    let params = MoverParams::default();
    let mut nodes = vec![PathNode::at(p(4.0, 0.0, 0.0)), PathNode::at(p(4.0, 0.0, 4.0)), PathNode::at(p(0.0, 0.0, 4.0))];
    nodes[1].wait = 1.0;
    nodes[1].idle_variant = 3;
    nodes[1].facing = Some(PI);
    let mut path = PathFollower::new(nodes, PathMode::Once, 1.4);
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    let mut mover = KinematicMover::new();
    let mut waited = 0.0;
    let mut closest_to_wait = f32::MAX;
    let mut finished_at = None;
    for frame in 0..1200 {
        let out = path.update(loco.position, DT);
        if out.idle_variant == Some(3) {
            waited += DT;
            closest_to_wait = closest_to_wait.min(xyz(loco.position).distance(Vec3::new(4.0, 0.0, 4.0)));
        }
        if out.finished && finished_at.is_none() {
            finished_at = Some(frame);
        }
        mover.step(MoveInput { loco: &mut loco, intent: out.intent, params: &params, root: RootMotion::default() }, &world, DT);
    }
    assert!((waited - 1.0).abs() < 0.05, "waited {waited} s");
    assert!(closest_to_wait < 0.3, "stood {closest_to_wait} m from the wait node");
    assert!(finished_at.is_some());
    let end = xyz(loco.position);
    assert!(end.distance(Vec3::new(0.0, 0.0, 4.0)) < 0.3, "ended at {end}");
    assert!(loco.motion.velocity.length() < 0.01);

    // Loops cut corners within the arrival radius and never stop.
    let square = [p(3.0, 0.0, 0.0), p(3.0, 0.0, 3.0), p(0.0, 0.0, 3.0), p(0.0, 0.0, 0.0)].map(PathNode::at).to_vec();
    let mut path = PathFollower::new(square, PathMode::Loop, 1.4);
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
    let mut laps_nodes = 0;
    let mut last = 0;
    for _ in 0..1200 {
        let out = path.update(loco.position, DT);
        assert!(!out.finished && out.idle_variant.is_none());
        if out.node != last {
            laps_nodes += 1;
            last = out.node;
        }
        mover.step(MoveInput { loco: &mut loco, intent: out.intent, params: &params, root: RootMotion::default() }, &world, DT);
        assert!(loco.motion.velocity.length() < 1.41);
    }
    assert!(laps_nodes >= 8, "visited {laps_nodes} nodes in 20 s");

    // Ping-pong turns around at the ends.
    let line = [p(2.0, 0.0, 0.0), p(0.0, 0.0, 0.0)].map(PathNode::at).to_vec();
    let mut path = PathFollower::new(line, PathMode::PingPong, 1.0);
    let mut targets = Vec::new();
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    for _ in 0..900 {
        let out = path.update(loco.position, DT);
        if targets.last() != Some(&out.node) {
            targets.push(out.node);
        }
        mover.step(MoveInput { loco: &mut loco, intent: out.intent, params: &params, root: RootMotion::default() }, &world, DT);
    }
    assert!(targets.len() >= 4 && targets.windows(2).all(|w| w[0] != w[1]), "{targets:?}");
}

// --- Bake step ---------------------------------------------------------------------------

#[cfg(feature = "bake")]
mod bake {
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, RunCx, Step, Store};
    use sf_model::{CookSettings, LodPolicy, MeshData, Model, Node};

    use crate::steps::{COLLISION_COOK, CollisionCookStep, CollisionRules, CollisionSettings};
    use crate::*;

    #[derive(Default)]
    struct Files(RwLock<HashMap<String, Vec<u8>>>);

    impl Inputs for Files {
        fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
            Ok(self.0.read().unwrap().get(path).map(|b| Digest::of(b)))
        }
        fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
            self.0.read().unwrap().get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
        }
        fn record_fingerprint(&self, _kind: &str, _id: &str) -> Result<Option<Digest>, BakeError> {
            Ok(None)
        }
    }

    /// Stands in for `mesh.cook`: products from a table.
    struct MeshStub(Arc<RwLock<HashMap<String, Vec<u8>>>>);

    impl Step for MeshStub {
        fn id(&self) -> &'static str {
            sf_model::steps::MeshCookStep::ID
        }
        fn version(&self) -> u32 {
            1
        }
        fn plan(&self, subject: &str, cx: &mut sf_bake::PlanCx) -> Result<(), BakeError> {
            let bytes = self.0.read().unwrap().get(subject).cloned().unwrap_or_default();
            cx.param("bytes", Digest::of(&bytes).0.to_vec());
            Ok(())
        }
        fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
            self.0.read().unwrap().get(subject).cloned().ok_or_else(|| BakeError::step(&ProductKey::new(self.id(), subject), "missing"))
        }
    }

    /// A subdivided slab: 2 m × 2 m, 0.5 m thick, enough triangles for a LOD chain.
    fn slab() -> Model {
        let n = 16;
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for y in [0.0, 0.5] {
            let base = u32::try_from(positions.len()).unwrap();
            for j in 0..=n {
                for i in 0..=n {
                    positions.push([i as f32 * 2.0 / n as f32 - 1.0, y, j as f32 * 2.0 / n as f32 - 1.0]);
                }
            }
            for j in 0..n {
                for i in 0..n {
                    let a = base + j * (n + 1) + i;
                    let (b, c, d) = (a + 1, a + n + 1, a + n + 2);
                    if y > 0.0 {
                        indices.extend([a, c, b, b, c, d]);
                    } else {
                        indices.extend([a, b, c, b, d, c]);
                    }
                }
            }
        }
        let mesh = MeshData { positions, normals: None, uvs: None, colors: None, skinned: false, indices, material: 0, skin: None, morph_targets: Vec::new() };
        let mut node = Node::new("slab");
        node.mesh = Some(0);
        node.lod = LodPolicy::Simplify;
        let material = sf_model::Material { name: "stone".into(), ..sf_model::Material::default() };
        Model { nodes: vec![node], meshes: vec![mesh], materials: vec![material] }
    }

    struct Stone;

    impl CollisionRules for Stone {
        fn surface(&self, m: &sf_model::Material) -> SurfaceId {
            if m.name == "stone" { SurfaceId(5) } else { SurfaceId::DEFAULT }
        }
    }

    #[test]
    fn collision_cook_step_bakes_caches_and_honors_overrides() {
        let meshes: Arc<RwLock<HashMap<String, Vec<u8>>>> = Arc::default();
        let cooked = sf_model::cook(&slab(), &CookSettings::default()).unwrap();
        meshes.write().unwrap().insert("props/slab.glb".into(), cooked.to_bytes());
        meshes.write().unwrap().insert("props/anim.glb".into(), sf_model::Cooked::Skipped { reason: "animated".into() }.to_bytes());
        let files = Arc::new(Files::default());
        let dir = tempfile::tempdir().unwrap();
        let session = || {
            let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files.clone());
            baker.register(MeshStub(meshes.clone()));
            CollisionCookStep::register(Arc::new(Stone), CollisionSettings::default(), &mut baker);
            baker
        };
        let key = ProductKey::new(COLLISION_COOK, "props/slab.glb");
        let baker = session();
        let report = baker.bake(std::slice::from_ref(&key));
        assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
        let mesh = CollisionMesh::from_bytes(&baker.store().get(&baker.product(&key).unwrap()).unwrap()).unwrap();
        assert!(!mesh.is_empty());
        assert!(mesh.surfaces.iter().all(|&s| s == SurfaceId(5)));
        // The collision works: a ray lands on the slab's top.
        let mut w = StaticWorld::new(sf_coords::WorldPos::ORIGIN);
        w.insert(&Arc::new(StaticShape::new(&mesh).unwrap()), sf_coords::WorldTransform::IDENTITY);
        let hit = w.probe_one(ProbeQuery::down(sf_coords::WorldPos::ORIGIN.offset(glam::Vec3::new(0.3, 2.0, -0.2)), 5.0)).unwrap();
        assert!((hit.distance - 1.5).abs() < 0.01, "{hit:?}");

        // A warm session builds nothing; skipped models give empty collision.
        let again = session();
        assert_eq!(again.bake(std::slice::from_ref(&key)).failures().count(), 0);
        assert_eq!(again.step_stats()[COLLISION_COOK].built, 0);
        let anim = ProductKey::new(COLLISION_COOK, "props/anim.glb");
        let empty = CollisionMesh::from_bytes(&again.store().get(&again.product(&anim).unwrap()).unwrap()).unwrap();
        assert!(empty.is_empty());

        // An authored `collision/<model>.sfcol` wins.
        let mut b = CollisionBuilder::new(1e-3);
        b.add(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]], &[0, 1, 2], glam::Affine3A::IDENTITY, SurfaceId(8));
        let authored = b.finish();
        files.0.write().unwrap().insert("collision/props/slab.glb.sfcol".into(), authored.to_bytes());
        let third = session();
        let got = CollisionMesh::from_bytes(&third.store().get(&third.product(&key).unwrap()).unwrap()).unwrap();
        assert_eq!(got, authored);
    }
}

// --- Timing ------------------------------------------------------------------------------

/// The crowd budget (locomotion.md): 150 characters on terrain plus a town
/// of statics, one mover step each.
/// `cargo test --release -p sf_controller -- --ignored --nocapture crowd_step`
#[test]
#[ignore = "timing; run in release"]
fn crowd_step_timing() {
    let mut world = World::new();
    // A town: 400 houses (boxes) and 400 stair sets on a 20 × 20 grid, 12 m apart.
    for i in 0..20 {
        for j in 0..20 {
            let (x, z) = (i as f32 * 12.0, j as f32 * 12.0);
            world.add_box(Vec3::new(x, 0.0, z), Vec3::new(x + 6.0, 5.0, z + 6.0));
            world.add_box(Vec3::new(x + 7.0, 0.0, z), Vec3::new(x + 8.0, 0.3, z + 3.0));
        }
    }
    world.statics.rebuild();
    let params = MoverParams::default();
    let mut locos: Vec<Locomotion> =
        (0..150).map(|k| Locomotion::new(p(6.5 + (k % 15) as f32 * 12.0, 0.0, 7.0 + (k / 15) as f32 * 12.0), 0.0)).collect();
    let mut mover = KinematicMover::new();
    let intent = toward(Vec3::new(1.0, 0.0, 0.7));
    let mut times = Vec::new();
    for _ in 0..300 {
        let t0 = std::time::Instant::now();
        let mut items: Vec<MoveInput<'_>> =
            locos.iter_mut().map(|loco| MoveInput { loco, intent, params: &params, root: RootMotion::default() }).collect();
        mover.step_batch(&mut items, &world, DT);
        times.push(t0.elapsed().as_secs_f64() * 1e3);
    }
    times.sort_by(f64::total_cmp);
    println!("crowd of 150, mover step: median {:.3} ms, p95 {:.3} ms (battery)", times[150], times[285]);
}

#[cfg(feature = "terrain")]
#[test]
fn terrain_probe_runs_on_sf_terrain_heights() {
    use sf_terrain::{HeightTile, ResidentTile, TerrainHeights, TileCoord};
    let mut heights = TerrainHeights::new();
    heights.insert(Arc::new(ResidentTile::new(HeightTile::flat(TileCoord::new(0, 0, 0), 12.5))));
    let terrain = TerrainProbe(&heights);
    let hit = terrain.probe_one(ProbeQuery::down(p(10.0, 20.0, 30.0), 10.0)).unwrap();
    assert!((hit.distance - 7.5).abs() < 1e-4 && hit.normal.y > 0.999, "{hit:?}");
    let slanted = terrain.probe_one(ProbeQuery::ray(p(10.0, 20.0, 30.0), Vec3::new(0.6, -0.8, 0.0), 20.0)).unwrap();
    assert!((xyz(slanted.point).y - 12.5).abs() < 1e-3, "{slanted:?}");
    // A character dropped above the terrain lands on it.
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(10.0, 13.0, 30.0), 0.0);
    let mut mover = KinematicMover::new();
    for _ in 0..60 {
        mover.step(MoveInput { loco: &mut loco, intent: LocomotionIntent::IDLE, params: &params, root: RootMotion::default() }, &terrain, DT);
    }
    assert!(loco.grounded.is_some() && (xyz(loco.position).y - 12.5).abs() < 1e-4, "{:?}", xyz(loco.position));
}

// --- Traversal, flying, swimming, rig-profile shapes (audit items 13-14) ------------------

/// Steps with a constant root delta (character frame) at full `Drive` weight.
fn drive(world: &World, mover: &mut KinematicMover, loco: &mut Locomotion, params: &MoverParams, delta: Vec3, traversal: bool, frames: usize) {
    let root = RootMotion { delta: RootDelta::new(delta, 0.0), drive_weight: 1.0, traversal };
    for _ in 0..frames {
        mover.step(MoveInput { loco, intent: LocomotionIntent::IDLE, params, root }, world, DT);
    }
}

#[test]
fn a_traversal_applies_vertical_root_motion_and_regrounds() {
    let mut world = World::new();
    world.add_box(Vec3::new(1.0, 0.0, -1.0), Vec3::new(2.0, 1.0, 1.0));
    let params = MoverParams::default();
    let mut mover = KinematicMover::new();
    // A mantle onto a 1 m box, taller than `step_up`: rise 1.05 m, then 1.5 m forward.
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 2, |_| {});
    drive(&world, &mut mover, &mut loco, &params, Vec3::new(0.0, 1.05 / 30.0, 0.0), true, 30);
    assert!(loco.grounded.is_none() && loco.traversing && (xyz(loco.position).y - 1.05).abs() < 1e-3, "{:?}", xyz(loco.position));
    drive(&world, &mut mover, &mut loco, &params, Vec3::new(0.0, 0.0, 0.05), true, 30);
    // The window closes: the step-down probe lands the feet on the top.
    drive(&world, &mut mover, &mut loco, &params, Vec3::ZERO, false, 2);
    let at = xyz(loco.position);
    assert!(loco.grounded.is_some() && !loco.traversing && (at.y - 1.0).abs() < 0.02 && (at.x - 1.5).abs() < 0.02, "{at}");
    assert_eq!(loco.vertical_speed, 0.0);

    // Without the flag the vertical part is dropped and the box blocks.
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), FRAC_PI_2);
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 2, |_| {});
    drive(&world, &mut mover, &mut loco, &params, Vec3::new(0.0, 1.05 / 30.0, 0.0), false, 30);
    drive(&world, &mut mover, &mut loco, &params, Vec3::new(0.0, 0.0, 0.05), false, 30);
    let at = xyz(loco.position);
    assert!(at.y.abs() < 1e-4 && at.x < 0.75, "{at}");

    // A traversal downward stops on the floor, never through it.
    let mut loco = Locomotion::new(p(-3.0, 0.0, 0.0), 0.0);
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 2, |_| {});
    drive(&world, &mut mover, &mut loco, &params, Vec3::new(0.0, -0.05, 0.0), true, 30);
    assert!(xyz(loco.position).y.abs() < 1e-3, "{:?}", xyz(loco.position));
}

#[test]
fn flyers_move_in_3d_between_floor_and_ceiling() {
    let mut world = World::new();
    world.add_box(Vec3::new(-5.0, 5.0, -5.0), Vec3::new(5.0, 5.5, 5.0));
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(0.0, 1.0, 0.0), 0.0);
    loco.mode = MoverMode::Fly { hover: None };
    walk(&world, &mut loco, toward(Vec3::new(0.0, 2.0, 1.0)), &params, 60, |_| {});
    let at = xyz(loco.position);
    assert!(at.y > 2.0 && at.z > 0.5 && loco.grounded.is_none(), "{at}");
    assert!(loco.params().pitch > 0.5 && loco.params().vertical_speed > 1.0, "{:?}", loco.params());
    // The ceiling stops the capsule top (height 1.8) under it.
    walk(&world, &mut loco, toward(Vec3::new(0.0, 3.0, 0.0)), &params, 180, |_| {});
    let top = xyz(loco.position).y + params.height;
    assert!(top <= 5.0 && top > 4.95, "capsule top {top}");
    // Diving stops on the floor.
    walk(&world, &mut loco, toward(Vec3::new(0.0, -4.0, 0.0)), &params, 180, |_| {});
    assert!(xyz(loco.position).y.abs() < 1e-3, "{:?}", xyz(loco.position));
    // Back to walking in the air: it falls with its vertical speed and lands.
    let mut loco = Locomotion::new(p(0.0, 2.0, 0.0), 0.0);
    loco.mode = MoverMode::Fly { hover: None };
    walk(&world, &mut loco, toward(Vec3::new(0.0, -1.0, 0.0)), &params, 30, |_| {});
    loco.mode = MoverMode::Ground;
    let mut first = None;
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 90, |l| {
        first.get_or_insert(l.vertical_speed);
    });
    assert!(first.unwrap() < -0.5, "kept the dive speed: {first:?}");
    assert!(loco.grounded.is_some() && xyz(loco.position).y.abs() < 1e-4);
}

#[test]
fn hovering_holds_its_height_over_flat_ground_and_ramps_at_6_m_per_s() {
    let mut world = World::new();
    world.add_ramp(0.0, 10f32.to_radians());
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(-12.0, 1.5, 0.0), FRAC_PI_2);
    loco.mode = MoverMode::Fly { hover: Some(1.5) };
    loco.motion.velocity = Vec3::new(6.0, 0.0, 0.0);
    let (mut flat, mut ramp) = (0.0f32, 0.0f32);
    walk(&world, &mut loco, toward(Vec3::new(6.0, 0.0, 0.0)), &params, 198, |l| {
        let at = xyz(l.position);
        if (-6.0..0.0).contains(&at.x) {
            flat = flat.max((at.y - 1.5).abs());
        }
        if (4.0..7.5).contains(&at.x) {
            ramp = ramp.max((at.y - 1.5 - at.x * 10f32.to_radians().tan()).abs());
        }
    });
    println!("hover error: flat {flat:.4} m, 10° ramp {ramp:.4} m");
    assert!(xyz(loco.position).x > 7.5);
    assert!(flat <= 0.05 && ramp <= 0.05, "flat {flat} m, ramp {ramp} m");
}

#[test]
fn swimmers_stop_at_the_surface_and_the_bottom() {
    let world = World::new();
    let params = MoverParams::default();
    let water = 2.0;
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    let mut mover = KinematicMover::new();
    let mut swim = |loco: &mut Locomotion, vy: f32, frames: usize| {
        for _ in 0..frames {
            loco.mode = MoverMode::Swim { surface: water - xyz(loco.position).y };
            mover.step(MoveInput { loco, intent: toward(Vec3::new(0.0, vy, 0.5)), params: &params, root: RootMotion::default() }, &world, DT);
        }
    };
    swim(&mut loco, 2.0, 180);
    let y = xyz(loco.position).y;
    assert!((y - (water - params.swim_depth)).abs() < 1e-3, "floating feet at {y}");
    swim(&mut loco, -2.0, 180);
    assert!(xyz(loco.position).y.abs() < 1e-3 && xyz(loco.position).z > 2.0);
}

#[test]
fn attitude_follows_driven_tilt_and_levels_on_the_ground() {
    let world = World::new();
    let params = MoverParams::default();
    let mut mover = KinematicMover::new();
    let mut loco = Locomotion::new(p(0.0, 3.0, 0.0), 0.0);
    loco.mode = MoverMode::Fly { hover: None };
    let tilt = RootDelta { tilt: glam::Vec2::new(0.01, -0.005), ..RootDelta::new(Vec3::new(0.0, 0.0, 0.1), 0.0) };
    for _ in 0..30 {
        let root = RootMotion { delta: tilt, drive_weight: 1.0, traversal: false };
        mover.step(MoveInput { loco: &mut loco, intent: LocomotionIntent::IDLE, params: &params, root }, &world, DT);
    }
    assert!((loco.attitude - glam::Vec2::new(0.3, -0.15)).length() < 1e-4, "{}", loco.attitude);
    let nose = loco.render_rotation() * Vec3::Z;
    assert!(nose.y < -0.25, "pitch 0.3 about +X tips the nose down: {nose}");
    // Held while flying undriven; levelled on the ground.
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 30, |_| {});
    assert!((loco.attitude.x - 0.3).abs() < 1e-4);
    loco.mode = MoverMode::Ground;
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 180, |_| {});
    assert!(loco.attitude.length() < 0.01, "{}", loco.attitude);
}

#[test]
fn mover_shapes_come_from_rig_profiles() {
    use sf_anim::fixtures::{biped, hexapod, quadruped};
    use sf_anim::{RigHints, detect_profile};
    let profile = |s| detect_profile(&s, &RigHints::default()).unwrap();
    let b = profile(biped());
    let pb = MoverParams::from_profile(&b);
    assert!((pb.radius - b.measures.height / 6.0).abs() < 1e-5, "{pb:?}");
    assert!((pb.step_up - 0.5 * b.measures.leg_length).abs() < 1e-5 && pb.height == b.measures.height);
    let q = profile(quadruped(1.0));
    let pq = MoverParams::from_profile(&q);
    assert!((pq.radius - (q.measures.body_length / 4.0).max(q.measures.body_width / 2.0)).abs() < 1e-5, "{pq:?} {:?}", q.measures);
    assert!(pq.radius > pb.radius && pq.height >= 2.0 * pq.radius);
    let h = MoverParams::from_profile(&profile(hexapod()));
    assert!(h.step_up < pb.step_up && h.radius > 0.05, "{h:?}");
    // Without measures: the defaults.
    let mut none = q.clone();
    none.measures = sf_anim::Measures::default();
    assert_eq!(MoverParams::from_profile(&none), MoverParams::default());
}

#[test]
fn locomotion_round_trips_through_postcard() {
    // Net rule 3: the controller state is plain serde data (saves, co-op snapshots).
    let world = World::new();
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(0.5, 0.0, -2.0), 0.3);
    walk(&world, &mut loco, toward(Vec3::new(1.0, 0.0, 3.0)), &params, 20, |_| {});
    loco.launch(2.0);
    loco.attitude = glam::Vec2::new(0.1, -0.2);
    loco.deflect = Vec3::X;
    loco.slide = Vec3::new(0.0, 0.0, 0.5);
    assert!(loco.moved != RootDelta::ZERO && loco.trajectory[4].offset != Vec3::ZERO);
    let grounded = Locomotion { grounded: Some(GroundHit { point: p(1.0, 0.0, 2.0), normal: Vec3::Y, distance: 0.45, surface: SurfaceId(3) }), ..loco.clone() };
    for (i, mode) in [MoverMode::Ground, MoverMode::Fly { hover: Some(0.5) }, MoverMode::Swim { surface: 1.25 }].into_iter().enumerate() {
        let l = Locomotion { mode, ..if i == 0 { grounded.clone() } else { loco.clone() } };
        let bytes = postcard::to_allocvec(&l).unwrap();
        assert_eq!(postcard::from_bytes::<Locomotion>(&bytes).unwrap(), l);
    }
}

mod context;

mod avoid;
mod launch;
