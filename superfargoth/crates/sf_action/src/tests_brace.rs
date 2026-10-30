//! Wall brace (B2c): sanity only (hand on the face, never behind it, weights gated and finite);
//! tuned numbers print from the `measurement` tests.

use glam::Vec3;
use sf_controller::{CharacterFrame, GroundHit, GroundProbe, ProbeQuery, SurfaceId};
use sf_coords::WorldPos;

use crate::{BRACE_PALM_GAP, BraceBody, HandBrace, LEFT, RIGHT, WallBrace};

const HEIGHT: f32 = 1.8;
const DT: f32 = 1.0 / 60.0;

/// Axis-aligned boxes (min, max) around the world origin; rays only.
struct Boxes(Vec<(Vec3, Vec3)>);

impl GroundProbe for Boxes {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            let origin = WorldPos::ORIGIN.delta_f32(q.origin);
            *o = self
                .0
                .iter()
                .filter_map(|&(lo, hi)| slab(origin, q.dir, lo, hi))
                .filter(|(t, _)| *t <= q.max)
                .min_by(|a, b| a.0.total_cmp(&b.0))
                .map(|(t, normal)| GroundHit { point: q.origin.offset(q.dir * t), normal, distance: t, surface: SurfaceId::DEFAULT });
        }
    }
}

/// Entry distance and face normal of a ray into a box (origin outside).
#[allow(clippy::many_single_char_names)]
fn slab(o: Vec3, d: Vec3, lo: Vec3, hi: Vec3) -> Option<(f32, Vec3)> {
    let (mut t0, mut t1, mut n) = (0.0f32, f32::INFINITY, Vec3::ZERO);
    for k in 0..3 {
        if d[k].abs() < 1e-9 {
            if o[k] < lo[k] || o[k] > hi[k] {
                return None;
            }
            continue;
        }
        let (a, b) = ((lo[k] - o[k]) / d[k], (hi[k] - o[k]) / d[k]);
        let (near, far) = if a < b { (a, b) } else { (b, a) };
        if near > t0 {
            t0 = near;
            n = Vec3::ZERO;
            n[k] = -d[k].signum();
        }
        t1 = t1.min(far);
    }
    (t0 <= t1 && t0 > 0.0).then_some((t0, n))
}

/// A wall whose near face is the plane z = `z`, 6 m wide, 3.5 m high (the ledges course's).
fn wall_ahead(z: f32) -> Boxes {
    Boxes(vec![(Vec3::new(-3.0, 0.0, z), Vec3::new(3.0, 3.5, z + 0.5))])
}

/// A wall on the character's left (+X), its near face the plane x = `x`.
fn wall_left(x: f32) -> Boxes {
    Boxes(vec![(Vec3::new(x, 0.0, -5.0), Vec3::new(x + 0.5, 3.5, 5.0))])
}

fn body(desired: Vec3, velocity: Vec3) -> BraceBody {
    BraceBody { desired, velocity, ..BraceBody::estimate(HEIGHT) }
}

/// Runs the brace `frames` frames at the origin facing +Z.
fn run(probe: &Boxes, body: &BraceBody, frames: usize) -> WallBrace {
    let frame = CharacterFrame::new(WorldPos::ORIGIN, 0.0);
    let mut brace = WallBrace::default();
    for _ in 0..frames {
        brace.update(&frame, probe, body, DT);
    }
    brace
}

fn assert_sane(brace: &WallBrace) {
    for h in brace.hands {
        assert!(h.weight.is_finite() && (0.0..=1.0).contains(&h.weight), "weight {}", h.weight);
        assert!(h.target.is_none_or(Vec3::is_finite), "target {:?}", h.target);
        assert_eq!(h.target.is_some(), h.weight > 0.0);
    }
}

#[test]
fn running_into_a_wall_plants_both_palms_on_the_face() {
    let wall = 0.45;
    let b = body(Vec3::Z * 4.5, Vec3::ZERO);
    let brace = run(&wall_ahead(wall), &b, 60);
    assert_sane(&brace);
    for i in [LEFT, RIGHT] {
        let h = brace.hands[i];
        assert!(h.weight > 0.95, "hand {i}: firm brace, weight {}", h.weight);
        let t = h.target.expect("a target");
        // The palm on the face (wrist the palm's gap off it), never behind the plane.
        assert!((wall - BRACE_PALM_GAP - t.z).abs() <= 0.02, "hand {i}: wrist z {} vs face {wall}", t.z);
        assert!(t.z < wall, "hand {i}: behind the face");
        // Within the arm: IK reaches it without the arm through the wall.
        assert!(t.distance(b.shoulders[i]) <= b.arms[i] + 1e-4, "hand {i}: out of reach");
        // Each hand on its own side.
        assert_eq!(t.x > 0.0, i == LEFT, "hand {i}: x {}", t.x);
        assert!((h.normal - Vec3::NEG_Z).length() < 1e-3);
    }
}

#[test]
fn walking_past_a_side_wall_is_a_light_trail_on_that_hand() {
    let b = body(Vec3::Z * 1.5, Vec3::Z * 1.5);
    let brace = run(&wall_left(0.5), &b, 60);
    assert_sane(&brace);
    let left = brace.hands[LEFT];
    assert!(left.weight > 0.0 && left.weight < 0.9, "light, weight {}", left.weight);
    let t = left.target.expect("a target");
    assert!(t.x < 0.5 && (0.5 - BRACE_PALM_GAP - t.x).abs() <= 0.02, "wrist x {}", t.x);
    // Trailing: behind the shoulder along the walk.
    assert!(t.z < b.shoulders[LEFT].z, "trails, z {}", t.z);
    assert_eq!(brace.hands[RIGHT], HandBrace::default(), "no wall on the right");
}

#[test]
fn a_busy_hand_never_braces() {
    // Sword drawn: the weapon hand (right) is busy, the off hand braces.
    let b = BraceBody { free: [true, false], ..body(Vec3::Z * 4.5, Vec3::ZERO) };
    let brace = run(&wall_ahead(0.45), &b, 60);
    assert_sane(&brace);
    assert!(brace.hands[LEFT].weight > 0.95);
    assert_eq!(brace.hands[RIGHT], HandBrace::default());
}

#[test]
fn no_wall_still_or_inactive_means_no_brace() {
    let cases = [
        (Boxes(vec![]), body(Vec3::Z * 4.5, Vec3::Z * 4.5)),
        (wall_ahead(0.45), body(Vec3::ZERO, Vec3::ZERO)),
        (wall_ahead(0.45), BraceBody { active: false, ..body(Vec3::Z * 4.5, Vec3::ZERO) }),
        // Out of reach: 3 m away while walking (no look-ahead past the band).
        (wall_ahead(3.0), body(Vec3::X * 1.5, Vec3::X * 1.5)),
        // A low box (waist high) is not a wall at the shoulders.
        (Boxes(vec![(Vec3::new(-3.0, 0.0, 0.45), Vec3::new(3.0, 0.9, 1.0))]), body(Vec3::Z * 4.5, Vec3::ZERO)),
    ];
    for (k, (probe, b)) in cases.iter().enumerate() {
        let brace = run(probe, b, 30);
        assert_sane(&brace);
        assert_eq!(brace, WallBrace::default(), "case {k}");
    }
}

#[test]
fn a_wall_that_ends_drops_the_hand_past_it() {
    // Running into a wall whose end is at x = -0.25: the right shoulder (x -0.19) faces it, its
    // palm (out to -X) lands past the end.
    let probe = Boxes(vec![(Vec3::new(-0.25, 0.0, 0.45), Vec3::new(3.0, 3.5, 1.0))]);
    let brace = run(&probe, &body(Vec3::Z * 4.5, Vec3::ZERO), 60);
    assert_sane(&brace);
    assert!(brace.hands[LEFT].weight > 0.95);
    assert_eq!(brace.hands[RIGHT], HandBrace::default());
}

#[test]
fn released_weight_fades_out_and_bad_input_stays_finite() {
    let probe = wall_ahead(0.45);
    let frame = CharacterFrame::new(WorldPos::ORIGIN, 0.0);
    let mut brace = run(&probe, &body(Vec3::Z * 4.5, Vec3::ZERO), 60);
    // Stick released: the weight falls over frames, never jumps to 0 in one.
    let still = body(Vec3::ZERO, Vec3::ZERO);
    let before = brace.hands[LEFT].weight;
    brace.update(&frame, &probe, &still, DT);
    assert!(brace.hands[LEFT].weight > 0.0 && brace.hands[LEFT].weight < before);
    for _ in 0..60 {
        brace.update(&frame, &probe, &still, DT);
    }
    assert_eq!(brace, WallBrace::default());
    // NaN shoulders, velocities and dt.
    let bad = BraceBody { shoulders: [Vec3::NAN; 2], desired: Vec3::NAN, ..body(Vec3::Z * 4.5, Vec3::ZERO) };
    for dt in [DT, f32::NAN, -1.0] {
        brace.update(&frame, &probe, &bad, dt);
        assert_sane(&brace);
    }
    let bad = BraceBody { arms: [f32::NAN, 0.0], ..body(Vec3::Z * 4.5, Vec3::ZERO) };
    brace.update(&frame, &probe, &bad, DT);
    assert_sane(&brace);
}

#[test]
fn rotated_frames_give_the_same_local_brace() {
    // The same wall ahead of a character facing +X, away from the origin: same local targets.
    let yaw = std::f32::consts::FRAC_PI_2;
    let at = Vec3::new(10.0, 2.0, -4.0);
    let frame = CharacterFrame::new(WorldPos::ORIGIN.offset(at), yaw);
    // Facing +X: the face is the plane x = at.x + 0.45.
    let probe = Boxes(vec![(Vec3::new(at.x + 0.45, at.y, at.z - 3.0), Vec3::new(at.x + 0.95, at.y + 3.5, at.z + 3.0))]);
    // Velocities are character space.
    let b = body(Vec3::Z * 4.5, Vec3::ZERO);
    let mut rotated = WallBrace::default();
    for _ in 0..60 {
        rotated.update(&frame, &probe, &b, DT);
    }
    let straight = run(&wall_ahead(0.45), &b, 60);
    for i in [LEFT, RIGHT] {
        let (a, s) = (rotated.hands[i].target.expect("rotated"), straight.hands[i].target.expect("straight"));
        assert!(a.distance(s) < 1e-3, "hand {i}: {a} vs {s}");
    }
}

#[test]
#[ignore = "measurement: approach run into the wall, palm gap and weight per frame"]
fn measure_approach() {
    let wall = 0.45;
    let probe = wall_ahead(wall + 2.0);
    let mut brace = WallBrace::default();
    let mut z = 0.0f32;
    let speed = 4.5;
    println!("frame  feet_z  w_L    gap_L_cm  reach_L");
    for f in 0..50 {
        let moving = z < 2.0;
        let b = body(Vec3::Z * speed, if moving { Vec3::Z * speed } else { Vec3::ZERO });
        let frame = CharacterFrame::new(WorldPos::ORIGIN.offset(Vec3::Z * z), 0.0);
        let [l, _] = brace.update(&frame, &probe, &b, DT);
        let (gap, reach) = l.target.map_or((f32::NAN, f32::NAN), |t| ((wall + 2.0 - z - t.z - BRACE_PALM_GAP) * 100.0, t.distance(b.shoulders[LEFT]) / b.arms[LEFT]));
        println!("{f:5}  {z:6.3}  {:.3}  {gap:8.2}  {reach:.3}", l.weight);
        z = (z + speed * DT).min(2.0);
    }
}

#[test]
#[ignore = "measurement: weight level by speed into and along the wall"]
fn measure_levels() {
    println!("into_mps  past_mps  w_frontal  w_side_L");
    for (into, past) in [(0.0f32, 0.5f32), (0.0, 1.5), (0.0, 4.5), (1.0, 0.0), (2.5, 0.0), (4.5, 0.0)] {
        let frontal = run(&wall_ahead(0.45), &body(Vec3::new(0.0, 0.0, into.max(past)), Vec3::ZERO), 60);
        let side = run(&wall_left(0.5), &body(Vec3::new(into, 0.0, past), Vec3::new(0.0, 0.0, past)), 60);
        println!("{into:8.1}  {past:8.1}  {:.3}      {:.3}", frontal.hands[LEFT].weight, side.hands[LEFT].weight);
    }
}

#[test]
fn running_up_to_a_wall_never_puts_the_wrist_through_it() {
    let face = 2.45;
    let probe = wall_ahead(face);
    let mut brace = WallBrace::default();
    let mut z = 0.0f32;
    for f in 0..60 {
        let b = body(Vec3::Z * 4.5, if z < 2.0 { Vec3::Z * 4.5 } else { Vec3::ZERO });
        let frame = CharacterFrame::new(WorldPos::ORIGIN.offset(Vec3::Z * z), 0.0);
        let hands = brace.update(&frame, &probe, &b, DT);
        for (i, h) in hands.iter().enumerate() {
            if let Some(t) = h.target {
                assert!(z + t.z <= face - BRACE_PALM_GAP + 1e-3, "frame {f} hand {i}: wrist {} past the face {face}", z + t.z);
            }
        }
        assert_sane(&brace);
        z = (z + 4.5 * DT).min(2.0);
    }
    assert!(brace.hands.iter().all(|h| h.weight > 0.95), "braced at the end");
}

/// Closest the elbow of a two-bone arm (equal bones of `arm / 2`) from `shoulder` to `wrist` can
/// come to the plane through `face` with normal `n` (toward the character), whatever its pole:
/// the elbow circles the segment's midpoint at radius `h`, perpendicular to the segment.
fn elbow_clearance(shoulder: Vec3, wrist: Vec3, arm: f32, face: Vec3, n: Vec3) -> f32 {
    let axis = (wrist - shoulder).normalize();
    let half = shoulder.distance(wrist) / 2.0;
    let h = ((arm / 2.0).powi(2) - half * half).max(0.0).sqrt();
    let mid = (shoulder + wrist) / 2.0;
    (mid - face).dot(n) - h * (1.0 - axis.dot(n).powi(2)).max(0.0).sqrt()
}

#[test]
fn no_elbow_pole_puts_the_arm_through_the_face() {
    for (probe, face, n, b) in [
        (wall_ahead(0.45), Vec3::Z * 0.45, Vec3::NEG_Z, body(Vec3::Z * 4.5, Vec3::ZERO)),
        (wall_ahead(0.32), Vec3::Z * 0.32, Vec3::NEG_Z, body(Vec3::Z * 4.5, Vec3::ZERO)),
        (wall_left(0.5), Vec3::X * 0.5, Vec3::NEG_X, body(Vec3::Z * 1.5, Vec3::Z * 1.5)),
        (wall_left(0.35), Vec3::X * 0.35, Vec3::NEG_X, body(Vec3::new(1.0, 0.0, 1.5), Vec3::Z * 1.5)),
    ] {
        let brace = run(&probe, &b, 60);
        for (i, h) in brace.hands.iter().enumerate() {
            if let Some(t) = h.target {
                let c = elbow_clearance(b.shoulders[i], t, b.arms[i], face, n);
                assert!(c > 0.0, "hand {i} at face {face}: elbow can reach {c} m past it");
            }
        }
    }
}
