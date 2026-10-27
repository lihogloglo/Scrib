use std::f32::consts::{FRAC_PI_2, PI};
use std::sync::Arc;

use glam::{Affine3A, Vec3};
use sf_controller::{CollisionBuilder, StaticShape, StaticWorld, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};

use crate::*;

fn close(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

// ---------- Look ----------

#[test]
fn look_rotation_matches_forward_and_right() {
    for (yaw, pitch) in [(0.0, 0.0), (1.0, 0.3), (-2.5, -0.7), (PI, 0.5)] {
        let l = Look::new(yaw, pitch);
        let f = l.rotation() * Vec3::NEG_Z;
        assert!(f.distance(l.forward()) < 1e-5, "{l:?}: {f} vs {}", l.forward());
        let r = l.rotation() * Vec3::X;
        assert!(r.distance(l.right()) < 1e-5, "{l:?}");
        let back = Look::from_dir(l.forward());
        assert!(close(wrap_angle(back.yaw - yaw), 0.0, 1e-5) && close(back.pitch, pitch, 1e-5));
    }
}

#[test]
fn yaw_wraps_and_pitch_clamps() {
    let l = Look::new(PI - 0.1, 0.0).turned(Look::new(0.3, 2.0), -1.0, 1.0);
    assert!(close(l.yaw, -PI + 0.2, 1e-5) && close(l.pitch, 1.0, 0.0), "{l:?}");
    // Easing across the ±π seam takes the short way.
    let e = Look::new(PI - 0.1, 0.0).eased(Look::new(-PI + 0.1, 0.0), 0.0, 1.0 / 60.0);
    assert!(close(wrap_angle(e.yaw - (-PI + 0.1)), 0.0, 1e-5));
    let half = Look::new(PI - 0.1, 0.0).eased(Look::new(-PI + 0.1, 0.0), 0.1, 0.1);
    assert!(close(half.yaw.abs(), PI, 1e-4), "{half:?}");
}

// ---------- Orbit ----------

fn run_orbit(fps: f32, secs: f32, snap_at: f32) -> Vec3 {
    let p = OrbitParams::default();
    let mut rig = OrbitRig::new(Shoulder::Right);
    let dt = 1.0 / fps;
    let vel = Vec3::new(4.0, 0.0, 1.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let frames = (secs * fps).round() as u32;
    let mut last = Vec3::ZERO;
    for i in 0..=frames {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f32 / fps;
        // A 0.3 m step-up snap at `snap_at`, otherwise steady travel.
        let y = if t >= snap_at - 1e-4 { 0.3 } else { 0.0 };
        let body = WorldPos::ORIGIN.offset(vel * t + Vec3::Y * y);
        let pose = rig.update(&p, body, vel, Look::default(), RigMode::Normal, dt);
        last = body.offset(Vec3::Y * p.pivot_height).delta_f32(pose.pivot);
    }
    last
}

#[test]
fn pivot_tracks_steady_motion_without_lag() {
    for fps in [30.0, 60.0, 144.0] {
        let lag = run_orbit(fps, 1.0, 10.0);
        assert!(lag.length() < 1e-3, "{fps} fps: {lag}");
    }
}

#[test]
fn pivot_spring_filters_snaps_and_is_frame_rate_independent() {
    // Just after the snap the pivot is still below the body; it converges later.
    // 2/3 s is a whole number of frames at every rate.
    let early: Vec<f32> = [30.0, 60.0, 144.0].iter().map(|&f| run_orbit(f, 2.0 / 3.0, 0.5).y).collect();
    for y in &early {
        assert!(*y < -0.05, "{early:?}");
    }
    // Same lag at the same time whatever the frame rate (snap tick aligned to every rate).
    assert!(early.iter().all(|y| close(*y, early[0], 0.01)), "{early:?}");
    let late = run_orbit(60.0, 2.0, 0.5);
    assert!(late.length() < 5e-3, "{late}");
}

#[test]
fn teleport_snaps_instead_of_flying() {
    let p = OrbitParams::default();
    let mut rig = OrbitRig::new(Shoulder::Right);
    rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Normal, 1.0 / 60.0);
    let far = WorldPos::ORIGIN.offset(Vec3::new(500.0, 0.0, 0.0));
    let pose = rig.update(&p, far, Vec3::ZERO, Look::default(), RigMode::Normal, 1.0 / 60.0);
    assert!(far.offset(Vec3::Y * p.pivot_height).delta_f32(pose.pivot).length() < 1e-3);
}

#[test]
fn a_traversal_lead_eases_in_and_never_teleports() {
    let p = OrbitParams::default();
    let dt = 1.0 / 60.0;
    let mut rig = OrbitRig::new(Shoulder::Right);
    rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Normal, dt);
    // A climb's destination 2.5 m up (past `max_lag`): the pivot rises toward it, no cut.
    rig.set_lead(2.5);
    let first = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Normal, dt).pivot;
    let rise = WorldPos::ORIGIN.delta_f32(first).y - p.pivot_height;
    assert!(rise > -1e-4 && rise < 0.1, "first frame rises a little, not at once: {rise}");
    let mut last = first;
    for _ in 0..60 {
        last = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Normal, dt).pivot;
    }
    let settled = WorldPos::ORIGIN.delta_f32(last).y - p.pivot_height;
    assert!(close(settled, 2.5, 0.01), "{settled}");
    // A real teleport still cuts.
    let far = WorldPos::ORIGIN.offset(Vec3::new(500.0, 0.0, 0.0));
    rig.set_lead(0.0);
    let pose = rig.update(&p, far, Vec3::ZERO, Look::default(), RigMode::Normal, dt);
    assert!(far.offset(Vec3::Y * p.pivot_height).delta_f32(pose.pivot).length() < 1e-3);
}

#[test]
fn orbit_pitch_limits_mode_distance_and_shoulder_swap() {
    let p = OrbitParams::default();
    let mut rig = OrbitRig::new(Shoulder::Right);
    let dt = 1.0 / 60.0;
    let pose = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::new(0.0, -1.5), RigMode::Normal, dt);
    assert!(close(pose.look.pitch, p.pitch_min, 0.0));
    let pose = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::new(0.0, 1.5), RigMode::Normal, dt);
    assert!(close(pose.look.pitch, p.pitch_max, 0.0));
    // Arm: behind the look, right shoulder at yaw 0 is -X.
    let pose = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Normal, dt);
    assert!(pose.arm.z < -3.0 && close(pose.arm.x, -p.shoulder_offset, 1e-4), "{:?}", pose.arm);
    // Sneak eases closer, not at once.
    let first = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Sneak, dt);
    assert!(-first.arm.z > p.distance.sneak + 0.5);
    let mut pose = first;
    for _ in 0..120 {
        pose = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Sneak, dt);
    }
    assert!(close(-pose.arm.z, p.distance.sneak, 0.01), "{:?}", pose.arm);
    rig.swap_shoulder();
    for _ in 0..60 {
        pose = rig.update(&p, WorldPos::ORIGIN, Vec3::ZERO, Look::default(), RigMode::Sneak, dt);
    }
    assert!(close(pose.arm.x, p.shoulder_offset, 0.01), "{:?}", pose.arm);
}

#[test]
fn recentre_waits_for_idle_look_and_travel() {
    let p = RecentreParams::default();
    let look = Look::new(0.0, 0.2);
    let travel = Vec3::new(3.0, 0.0, 0.0); // yaw +π/2
    assert_eq!(recentre_behind(look, travel, 0.5, &p, 0.1), look);
    assert_eq!(recentre_behind(look, Vec3::new(0.1, 0.0, 0.0), 5.0, &p, 0.1), look);
    let mut l = look;
    for _ in 0..600 {
        l = recentre_behind(l, travel, 5.0, &p, 1.0 / 60.0);
    }
    assert!(close(l.yaw, FRAC_PI_2, 1e-3) && close(l.pitch, 0.2, 0.0), "{l:?}");
}

// ---------- Collision ----------

fn box_shape(half: Vec3) -> Arc<StaticShape> {
    let c = |x: f32, y: f32, z: f32| [x * half.x, y * half.y, z * half.z];
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
    b.add(&corners, &idx, Affine3A::IDENTITY, SurfaceId(1));
    Arc::new(StaticShape::new(&b.finish()).unwrap())
}

/// A wall slab from `min` to `max`.
fn world_with_box(min: Vec3, max: Vec3) -> StaticWorld {
    let mut w = StaticWorld::new(WorldPos::ORIGIN);
    w.insert(&box_shape((max - min) / 2.0), WorldTransform::from_pos(WorldPos::ORIGIN.offset((min + max) / 2.0)));
    w
}

fn box_distance(p: Vec3, min: Vec3, max: Vec3) -> f32 {
    (p.clamp(min, max) - p).length()
}

const WALL_MIN: Vec3 = Vec3::new(-6.0, 0.0, -3.0);
const WALL_MAX: Vec3 = Vec3::new(6.0, 4.0, -2.5);

/// Body walking backwards into the wall, then stepping clear.
#[test]
fn push_in_is_instant_never_inside_and_eases_out_slower() {
    let world = world_with_box(WALL_MIN, WALL_MAX);
    let (op, cp) = (OrbitParams::default(), CollisionParams::default());
    let mut rig = OrbitRig::new(Shoulder::Right);
    let mut arm = ArmCollision::new();
    let dt = 1.0 / 60.0;
    let mut z = 3.0;
    let mut min_len = f32::MAX;
    // Walk back toward the wall at 3 m/s, yaw jittering so the arm sweeps.
    for i in 0..90 {
        z -= 3.0 * dt;
        #[allow(clippy::cast_precision_loss)]
        let yaw = 0.3 * (i as f32 * 0.2).sin();
        let body = WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, z));
        let pose = rig.update(&op, body, Vec3::new(0.0, 0.0, -3.0), Look::new(yaw, -0.2), RigMode::Normal, dt);
        let r = arm.update(&world, pose.pivot, pose.arm, &cp, dt);
        // Never inside: the eye sphere stays clear of the wall.
        let eye = WorldPos::ORIGIN.delta_f32(r.eye);
        assert!(box_distance(eye, WALL_MIN, WALL_MAX) >= cp.radius - 1e-3, "frame {i}: eye {eye} len {}", r.length);
        // Push-in is immediate: never longer than this frame's free length.
        assert!(r.length <= r.free + 1e-6);
        min_len = min_len.min(r.length);
    }
    assert!(min_len < 1.0, "the wall should have pushed the eye in: {min_len}");
    // Fade near the character.
    rig.reset();
    let pose = rig.update(&op, WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, -2.2)), Vec3::ZERO, Look::new(0.0, 0.0), RigMode::Normal, dt);
    let r = arm.update(&world, pose.pivot, pose.arm, &cp, dt);
    assert!(r.blocked && r.fade > 0.99, "{r:?}");

    // Step clear at once: the arm eases out over several frames (slower than in).
    rig.reset();
    let far = WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, 20.0));
    let mut frames = 0;
    loop {
        let pose = rig.update(&op, far, Vec3::ZERO, Look::default(), RigMode::Normal, dt);
        let r = arm.update(&world, pose.pivot, pose.arm, &cp, dt);
        frames += 1;
        assert!(r.fade <= 1.0 && r.length <= r.free + 1e-6);
        if r.length > 0.95 * op.distance.normal {
            break;
        }
        assert!(frames < 600);
    }
    assert!(frames > 10, "ease-out took {frames} frames");
}

#[test]
fn ease_out_is_frame_rate_independent() {
    let cp = CollisionParams::default();
    let world = StaticWorld::new(WorldPos::ORIGIN);
    let lens: Vec<f32> = [30.0f32, 60.0, 144.0]
        .iter()
        .map(|&fps| {
            let mut arm = ArmCollision::new();
            // Start pushed in by a box, then free.
            let blocker = world_with_box(Vec3::new(-1.0, 0.0, -1.5), Vec3::new(1.0, 3.0, -1.0));
            arm.update(&blocker, WorldPos::ORIGIN.offset(Vec3::Y), Vec3::new(0.0, 0.0, -3.0), &cp, 1.0 / fps);
            let mut len = 0.0;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            for _ in 0..(0.3 * fps).round() as usize {
                len = arm.update(&world, WorldPos::ORIGIN.offset(Vec3::Y), Vec3::new(0.0, 0.0, -3.0), &cp, 1.0 / fps).length;
            }
            len
        })
        .collect();
    assert!(lens[0] < 2.9 && lens.iter().all(|l| close(*l, lens[0], 0.01)), "{lens:?}");
}

/// A desired arm that grows by itself (zoom out, a longer mode distance: the rig eases those) is
/// followed exactly when nothing blocks it; only recovery from an obstruction eases.
#[test]
fn an_unblocked_arm_follows_its_desired_length_exactly() {
    let cp = CollisionParams::default();
    let world = StaticWorld::new(WorldPos::ORIGIN);
    let pivot = WorldPos::ORIGIN.offset(Vec3::Y);
    let mut arm = ArmCollision::new();
    arm.update(&world, pivot, Vec3::new(0.0, 0.0, -3.0), &cp, 1.0 / 60.0);
    let r = arm.update(&world, pivot, Vec3::new(0.0, 0.0, -3.6), &cp, 1.0 / 60.0);
    assert!(close(r.length, 3.6, 1e-5) && !r.blocked, "{r:?}");
    // Blocked at 1 m, then the arm both clears and grows: still eased, still never past free.
    let blocker = world_with_box(Vec3::new(-1.0, 0.0, -1.5), Vec3::new(1.0, 3.0, -1.0));
    let pushed = arm.update(&blocker, pivot, Vec3::new(0.0, 0.0, -3.6), &cp, 1.0 / 60.0).length;
    let r = arm.update(&world, pivot, Vec3::new(0.0, 0.0, -4.0), &cp, 1.0 / 60.0);
    assert!(r.length > pushed && r.length < 2.0, "{pushed} -> {r:?}");
}

// ---------- Lock-on ----------

fn cand(id: u32, x: f32, z: f32, los: bool) -> Candidate<u32> {
    Candidate { id, pos: WorldPos::ORIGIN.offset(Vec3::new(x, 1.0, z)), los }
}

#[test]
fn acquire_scores_cone_range_and_line_of_sight() {
    let p = LockParams::default();
    let o = WorldPos::ORIGIN.offset(Vec3::Y);
    // Centred and near wins; off-cone, too far, and hidden never.
    let list = [cand(1, 3.0, 8.0, true), cand(2, 0.2, 6.0, true), cand(3, 20.0, 1.0, true), cand(4, 0.0, 16.0, true), cand(5, 0.0, 3.0, false)];
    assert_eq!(acquire(o, 0.0, &list, &p), Some(2));
    assert_eq!(acquire(o, PI, &list, &p), None);
    // Switching: from 2, screen right (toward -X at yaw 0) has nothing; left has 1.
    let wide = [cand(1, 3.0, 8.0, true), cand(2, 0.2, 6.0, true), cand(6, -4.0, 6.0, true)];
    assert_eq!(switch_target(o, 2, &wide, false, &p), Some(1));
    assert_eq!(switch_target(o, 2, &wide, true, &p), Some(6));
    assert_eq!(switch_target(o, 1, &wide, false, &p), None);
}

#[test]
fn lock_breaks_with_range_hysteresis_and_los_timeout() {
    let p = LockParams::default();
    let o = WorldPos::ORIGIN.offset(Vec3::Y);
    let mut t = LockTimers::default();
    // 18 m: can't acquire, but a held lock keeps it.
    assert_eq!(acquire(o, 0.0, &[cand(1, 0.0, 18.0, true)], &p), None);
    assert_eq!(t.check(o, Some(&cand(1, 0.0, 18.0, true)), &p, 0.1), Ok(()));
    assert_eq!(t.check(o, Some(&cand(1, 0.0, 21.0, true)), &p, 0.1), Err(BreakReason::Range));
    assert_eq!(t.check::<u32>(o, None, &p, 0.1), Err(BreakReason::Lost));
    // Line of sight: lost for 1.4 s keeps, regaining resets, 1.6 s breaks.
    let hidden = cand(1, 0.0, 8.0, false);
    for _ in 0..14 {
        assert_eq!(t.check(o, Some(&hidden), &p, 0.1), Ok(()));
    }
    assert_eq!(t.check(o, Some(&cand(1, 0.0, 8.0, true)), &p, 0.1), Ok(()));
    let broke = (1..=20).find(|_| t.check(o, Some(&hidden), &p, 0.1).is_err());
    assert!(matches!(broke, Some(15 | 16)), "{broke:?}");
}

#[test]
fn reacquire_after_traversal_only_after_touchdown_and_within_window() {
    let p = LockParams::default();
    let o = WorldPos::ORIGIN.offset(Vec3::Y);
    let list = [cand(7, 0.0, 12.0, true), cand(8, 0.0, 5.0, true)];
    let mut t = LockTimers::default();
    assert_eq!(t.break_for_traversal(7), BreakReason::Traversal);
    // Mid-air: nothing offered back yet.
    assert_eq!(t.reacquire(o, &list, |id| id, &p, 0.1), None);
    t.touchdown(&p);
    // Same target back (not the nearer 8), even if out of the acquire cone.
    assert_eq!(t.reacquire(WorldPos::ORIGIN.offset(Vec3::new(0.0, 1.0, 20.0)), &list, |id| id, &p, 0.1), Some(7));
    assert_eq!(t.reacquire(o, &list, |id| id, &p, 0.1), None, "offer is spent");
    // Expiry: hidden target during the whole window, then visible too late.
    t.break_for_traversal(7);
    t.touchdown(&p);
    let hidden = [cand(7, 0.0, 12.0, false)];
    for _ in 0..11 {
        assert_eq!(t.reacquire(o, &hidden, |id| id, &p, 0.1), None);
    }
    assert_eq!(t.reacquire(o, &list, |id| id, &p, 0.1), None);
}

#[test]
fn locked_framing_pitch_follows_height_gap_within_limits() {
    let p = LockParams::default();
    let pivot = WorldPos::ORIGIN.offset(Vec3::Y * 1.5);
    let level = locked_look(pivot, pivot.offset(Vec3::new(5.0, 0.0, 0.0)), &p);
    assert!(close(level.yaw, FRAC_PI_2, 1e-5) && close(level.pitch, p.pitch_bias, 1e-5));
    let above = locked_look(pivot, pivot.offset(Vec3::new(0.0, 30.0, 1.0)), &p);
    assert!(close(above.pitch, p.pitch_max, 0.0));
    let below = locked_look(pivot, pivot.offset(Vec3::new(0.0, -30.0, 1.0)), &p);
    assert!(close(below.pitch, p.pitch_min, 0.0));
}

// ---------- First person ----------

#[test]
fn first_person_limits_ease_in_hold_and_release() {
    let lim = LookLimits::from_degrees(70.0, (-45.0, 35.0), 0.15);
    let free = (-1.5, 1.5);
    let dt = 1.0 / 60.0;
    let mut fp = FirstPersonLook::new();
    let mut look = Look::new(2.0, -1.2); // past both limits around entry yaw 0
    fp.set_limits(Some(lim), 0.0);
    let first = fp.update(look, Look::default(), false, free, dt);
    // Eased: the first frame moves only a little toward the limits.
    assert!(first.yaw > lim.yaw + 0.5 && first.pitch < lim.pitch_min - 0.2, "{first:?}");
    look = first;
    for _ in 0..12 {
        look = fp.update(look, Look::default(), false, free, dt);
    }
    assert!(close(look.yaw, lim.yaw, 1e-4) && close(look.pitch, lim.pitch_min, 1e-4), "{look:?}");
    // Held: pushing further does nothing.
    let pushed = fp.update(look, Look::new(0.5, -0.5), false, free, dt);
    assert!(close(pushed.yaw, lim.yaw, 1e-4) && close(pushed.pitch, lim.pitch_min, 1e-4));
    // ignore_look drops input inside the limits too.
    let mid = Look::new(0.2, 0.0);
    let kept = fp.update(mid, Look::new(0.3, 0.1), true, free, dt);
    assert!(close(kept.yaw, mid.yaw, 1e-5) && close(kept.pitch, mid.pitch, 1e-5), "{kept:?}");
    // Release: free again once eased out.
    fp.set_limits(None, 0.0);
    for _ in 0..12 {
        fp.update(mid, Look::default(), false, free, dt);
    }
    assert!(fp.weight() < 1e-6);
    let after = fp.update(mid, Look::new(1.5, -1.0), false, free, dt);
    assert!(close(after.yaw, 1.7, 1e-5) && close(after.pitch, -1.0, 1e-5), "{after:?}");
}

#[test]
fn first_person_limits_wrap_at_seam() {
    let lim = LookLimits::from_degrees(30.0, (-45.0, 35.0), 0.0);
    let mut fp = FirstPersonLook::new();
    fp.set_limits(Some(lim), PI - 0.1);
    let l = fp.update(Look::new(-PI + 0.1, 0.0), Look::new(-1.0, 0.0), false, (-1.5, 1.5), 1.0 / 60.0);
    // Starting just across the ±π seam, turning 1 rad back clamps at entry - 30°.
    assert!(close(wrap_angle(l.yaw - (PI - 0.1)), -lim.yaw, 1e-4), "{l:?}");
}

// ---------- Comfort ----------

#[test]
fn trauma_shake_is_rotation_only_bounded_and_decays() {
    let p = ShakeParams::default();
    let on = ComfortSettings::default();
    let off = ComfortSettings { shake: false, ..on };
    assert!(!on.head_bob && !on.auto_camera, "comfort defaults");
    let mut s = TraumaShake::new(42);
    s.add(0.5);
    s.add(0.8);
    assert!(close(s.trauma(), 1.0, 0.0));
    let mut moved = false;
    for _ in 0..120 {
        let q = s.update(&p, &on, 1.0 / 60.0);
        assert!(q.is_normalized());
        let (yaw, pitch, roll) = q.to_euler(glam::EulerRot::YXZ);
        assert!(yaw.abs() <= p.max_yaw + 1e-5 && pitch.abs() <= p.max_pitch + 1e-5 && roll.abs() <= p.max_roll + 1e-5);
        moved |= yaw.abs() > 1e-4;
    }
    assert!(moved && s.trauma() <= 0.0);
    assert_eq!(s.update(&p, &on, 1.0 / 60.0), Quat::IDENTITY);
    s.add(1.0);
    assert_eq!(s.update(&p, &off, 1.0 / 60.0), Quat::IDENTITY);
    for t in 0..200 {
        #[allow(clippy::cast_precision_loss)]
        let n = comfort::noise(7, t as f32 * 0.37);
        assert!((-1.0..=1.0).contains(&n));
    }
}
