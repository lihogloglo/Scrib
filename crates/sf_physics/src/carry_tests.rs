//! P6: carry, drag and throw (physics.md "Carry and throw", "Verification").

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]

use glam::{IVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::{at, ground};
use crate::{BodyDesc, BodyId, BodyKind, CarryEnd, Carrier, GrabMode, GrabResult, HolderLimits, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

const DT: f64 = 1.0 / 60.0;
const EYE: f32 = 1.6;

fn world(params: PhysicsParams) -> (PhysicsWorld, WorldPos) {
    let base = WorldPos::new(IVec3::new(-1, 0, 4), Vec3::new(3.0, 12.0, -7.0));
    let mut w = PhysicsWorld::new(base, params);
    ground(&mut w, base);
    (w, base)
}

fn view(pos: WorldPos, yaw: f32, pitch: f32) -> WorldTransform {
    WorldTransform { pos, rotation: Quat::from_rotation_y(yaw) * Quat::from_rotation_x(pitch), scale: Vec3::ONE }
}

/// A box item resting on the ground at `p` (horizontal), awake.
fn item(w: &mut PhysicsWorld, base: WorldPos, p: Vec3, half: f32, mass: f32) -> BodyId {
    let layer = if mass < 15.0 { Layer::DEBRIS } else { Layer::PROP };
    let mut d = BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(base.offset(Vec3::new(p.x, half, p.z))), ShapeRef::Box(Vec3::splat(half)), layer);
    d.mass = Some(mass);
    d.damping = (0.0, 0.0);
    w.insert(&d)
}

/// Bevy's frame order at `fps`: virtual time advances, the fixed steps due
/// run (hand → step → record), then the view moves and the frame renders.
struct Clock {
    fps: f64,
    virt: f64,
    fixed: f64,
}

impl Clock {
    fn new(fps: f64) -> Self {
        Self { fps, virt: 0.0, fixed: 0.0 }
    }

    /// One frame; `view_at(t)` is the holder's eye at time t. Returns alpha.
    fn frame(&mut self, w: &mut PhysicsWorld, c: &mut Carrier, view_at: &dyn Fn(f64) -> WorldTransform, ends: &mut Vec<CarryEnd>) -> f32 {
        self.virt += 1.0 / self.fps;
        while self.fixed + DT <= self.virt + 1e-9 {
            self.fixed += DT;
            ends.extend(c.before_step(w, self.fixed));
            w.step();
            ends.extend(c.after_step(w));
        }
        c.record_view(self.virt, view_at(self.virt));
        ((self.virt - self.fixed) / DT) as f32
    }
}

/// PH-4 (automated): the carried item's view-space position varies < 1 mm
/// while the holder walks and turns, at 30, 60 and 144 fps.
#[test]
fn carried_item_is_steady_in_view_space() {
    for fps in [30.0, 60.0, 144.0] {
        let (mut w, base) = world(PhysicsParams::default());
        let body = item(&mut w, base, Vec3::new(0.0, 0.0, -1.0), 0.12, 2.0);
        // Walk along the view's facing at 1.4 m/s while turning at 1 rad/s (a 1.4 m circle), looking 20° down.
        let eye = base.offset(Vec3::Y * EYE);
        let path = move |t: f64| {
            let (yaw, r) = (t as f32, 1.4);
            let center = eye.offset(Vec3::new(-r, 0.0, 0.0));
            view(center.offset(Vec3::new(r * yaw.cos(), 0.0, -r * yaw.sin())), yaw, -0.35)
        };
        let mut c = Carrier::new(HolderLimits::default());
        let mut clock = Clock::new(fps);
        let mut ends = Vec::new();
        c.record_view(0.0, path(0.0));
        let grabbed = { let p = w.pose(body).unwrap().pos; c.grab_body(&mut w, body, p) };
        assert!(matches!(grabbed, GrabResult::Grabbed { mode: GrabMode::Carry, .. }), "{grabbed:?}");
        let (mut lo, mut hi, mut naive_lo, mut naive_hi) = (Vec3::MAX, Vec3::MIN, Vec3::MAX, Vec3::MIN);
        while clock.virt < 4.0 {
            let alpha = clock.frame(&mut w, &mut c, &path, &mut ends);
            if clock.virt < 1.5 {
                continue;
            }
            let v = path(clock.virt);
            let inv = v.rotation.inverse();
            let (_, pose) = c.render_pose(&v, alpha).expect("carrying");
            let p = inv * v.pos.delta(pose.pos).as_vec3();
            (lo, hi) = (lo.min(p), hi.max(p));
            let naive = inv * v.pos.delta(w.pose_at(body, alpha).unwrap().pos).as_vec3();
            (naive_lo, naive_hi) = (naive_lo.min(naive), naive_hi.max(naive));
        }
        let spread = (hi - lo).max_element();
        println!("{fps} fps: view-space spread {:.3} mm (world-space interpolation: {:.3} mm)", spread * 1e3, (naive_hi - naive_lo).max_element() * 1e3);
        assert!(ends.is_empty(), "{fps} fps: {ends:?}");
        assert!(spread < 1e-3, "{fps} fps: view-space spread {spread} m");
        // It is actually held up, in front of the view.
        assert!(lo.z < -0.5 && hi.z > -1.2, "{lo} {hi}");
    }
}

/// Carry the item standing still, looking level, for a second.
fn carry_still(w: &mut PhysicsWorld, base: WorldPos, body: BodyId, limits: HolderLimits) -> (Carrier, WorldTransform) {
    let eye = view(base.offset(Vec3::Y * EYE), 0.0, -0.5);
    let level = view(eye.pos, 0.0, 0.0);
    let mut c = Carrier::new(limits);
    c.record_view(0.0, eye);
    assert!(matches!({ let p = w.pose(body).unwrap().pos; c.grab_body(w, body, p) }, GrabResult::Grabbed { mode: GrabMode::Carry, .. }));
    let mut clock = Clock::new(60.0);
    let mut ends = Vec::new();
    while clock.virt < 1.5 {
        clock.frame(w, &mut c, &|_| level, &mut ends);
    }
    assert!(ends.is_empty(), "{ends:?}");
    (c, level)
}

/// Throws leave at the capped speed and land where a ballistic arc says.
#[test]
fn throws_are_capped_and_land_on_the_arc() {
    // A 0.3 kg cup: 6 N·s / 0.3 kg = 20 m/s, capped at 15.
    let (mut w, base) = world(PhysicsParams::default());
    let cup = item(&mut w, base, Vec3::new(0.0, 0.0, -0.8), 0.04, 0.3);
    let (mut c, _) = carry_still(&mut w, base, cup, HolderLimits::default());
    let (_, speed) = c.throw(&mut w).unwrap();
    assert!((speed - 15.0).abs() < 0.15, "launch speed {speed}");
    w.step();
    let v = w.velocity(cup).unwrap().0.length();
    assert!((v - 15.0).abs() / 15.0 < 0.01, "speed after a step {v}");

    // A 1 kg box at 6 m/s from about eye height: lands at v·√(2h/g).
    let (mut w, base) = world(PhysicsParams::default());
    let half = 0.08;
    let b = item(&mut w, base, Vec3::new(0.0, 0.0, -0.8), half, 1.0);
    let (mut c, _) = carry_still(&mut w, base, b, HolderLimits::default());
    let start = w.pose(b).unwrap().pos;
    let (_, speed) = c.throw(&mut w).unwrap();
    assert!((speed - 6.0).abs() < 0.06, "launch speed {speed}");
    let h = base.delta(start).y as f32 - half;
    let expected = speed * (2.0 * h / 9.81).sqrt();
    // The impact step: the fall stops (vertical speed jumps by more than gravity's
    // 0.16 m/s per step). The landing point is the free flight from the step
    // before it, solved to the floor.
    let mut prev = (start, w.velocity(b).unwrap().0);
    let mut landed = None;
    for _ in 0..120 {
        w.step();
        let v = w.velocity(b).unwrap().0;
        if v.y - prev.1.y > 1.0 {
            let (y, vy, g) = (base.delta(prev.0).y as f32 - half, prev.1.y, 9.81f32);
            let t = (vy + (vy * vy + 2.0 * g * y).sqrt()) / g;
            landed = Some(-(start.delta(prev.0).z as f32 + prev.1.z * t));
            break;
        }
        prev = (w.pose(b).unwrap().pos, v);
    }
    let landed = landed.expect("the box landed");
    println!("throw: {speed:.2} m/s from {h:.2} m lands at {landed:.3} m (ballistic {expected:.3} m)");
    assert!((landed - expected).abs() / expected < 0.02, "landed {landed} vs {expected}");
}

/// Carry, drag and too heavy switch at the holder's limits; a stronger
/// holder carries what a weaker one drags.
#[test]
fn modes_follow_the_holder_limits() {
    let weak = HolderLimits { carry_force: 160.0, carry_mass: 11.0, drag_mass: 88.0, throw_impulse: 4.4 };
    let strong = HolderLimits { carry_force: 440.0, carry_mass: 25.0, drag_mass: 200.0, throw_impulse: 10.0 };
    let params = PhysicsParams { mass_range: (0.2, 300.0), ..PhysicsParams::default() };
    for (mass, weak_mode, strong_mode) in [
        (5.0, Some(GrabMode::Carry), Some(GrabMode::Carry)),
        (11.0, Some(GrabMode::Carry), Some(GrabMode::Carry)),
        (12.0, Some(GrabMode::Drag), Some(GrabMode::Carry)),
        (60.0, Some(GrabMode::Drag), Some(GrabMode::Drag)),
        (100.0, None, Some(GrabMode::Drag)),
        (250.0, None, None),
    ] {
        for (limits, want) in [(weak, weak_mode), (strong, strong_mode)] {
            let (mut w, base) = world(params.clone());
            let body = item(&mut w, base, Vec3::new(0.0, 0.0, -1.2), 0.3, mass);
            w.step();
            let mut c = Carrier::new(limits);
            // Look at the box's top from the eye: the pick ray finds it.
            c.record_view(0.0, view(base.offset(Vec3::Y * EYE), 0.0, -(1.0f32.atan2(1.2))));
            let got = c.grab(&mut w);
            match (want, got) {
                (Some(m), GrabResult::Grabbed { body: b, mode, .. }) => assert!(b == body && mode == m, "{mass} kg: {got:?}"),
                (None, GrabResult::TooHeavy { body: b, .. }) => assert_eq!(b, body),
                _ => panic!("{mass} kg with {limits:?}: {got:?}, want {want:?}"),
            }
            if want == Some(GrabMode::Drag) {
                let s = c.speed_scale();
                assert!((s - (1.0 - mass / (2.0 * limits.drag_mass))).abs() < 1e-5, "speed scale {s}");
            }
        }
    }
}

/// A dragged crate stays on the floor and trails the holder; an item the
/// force cap can't lift lags and drops after `break_time`.
#[test]
fn drag_trails_on_the_floor_and_overloads_break() {
    let (mut w, base) = world(PhysicsParams::default());
    let crate_ = item(&mut w, base, Vec3::new(0.0, 0.0, -1.2), 0.3, 40.0);
    w.step();
    let limits = HolderLimits { carry_force: 240.0, carry_mass: 15.0, drag_mass: 120.0, throw_impulse: 6.0 };
    let mut c = Carrier::new(limits);
    let pitch = -(1.0f32.atan2(1.2));
    c.record_view(0.0, view(base.offset(Vec3::Y * EYE), 0.0, pitch));
    assert!(matches!(c.grab(&mut w), GrabResult::Grabbed { mode: GrabMode::Drag, .. }));
    let start = w.pose(crate_).unwrap().pos;
    // Walk forward (−Z) at 1 m/s × the drag speed scale for 3 s.
    let speed = f64::from(c.speed_scale());
    let path = move |t: f64| view(base.offset(Vec3::new(0.0, EYE, -(t * speed) as f32)), 0.0, pitch);
    let mut clock = Clock::new(60.0);
    let mut ends = Vec::new();
    let mut max_lift: f32 = 0.0;
    while clock.virt < 3.0 {
        clock.frame(&mut w, &mut c, &path, &mut ends);
        max_lift = max_lift.max(base.delta(w.pose(crate_).unwrap().pos).y as f32 - 0.3);
    }
    let moved = -start.delta(w.pose(crate_).unwrap().pos).z as f32;
    println!("drag: 40 kg crate moved {moved:.2} m in 3 s at scale {speed:.2}, lifted at most {max_lift:.3} m");
    assert!(ends.is_empty(), "{ends:?}");
    assert!(moved > 0.8 * 3.0 * speed as f32, "moved {moved}");
    assert!(max_lift < 0.15, "lifted {max_lift}");
    assert_eq!(c.throw(&mut w).map(|(b, _)| b), Some(crate_), "a dragged object is released");
    assert!(c.held().is_none());

    // Overload: 11 kg on a 40 N grip never lifts; the grab breaks.
    let (mut w, base) = world(PhysicsParams::default());
    let anvil = item(&mut w, base, Vec3::new(0.0, 0.0, -0.8), 0.1, 11.0);
    w.step();
    let mut c = Carrier::new(HolderLimits { carry_force: 40.0, ..HolderLimits::default() });
    let up = view(base.offset(Vec3::Y * EYE), 0.0, 0.3);
    c.record_view(0.0, view(base.offset(Vec3::Y * EYE), 0.0, -0.5));
    assert!(matches!({ let p = w.pose(anvil).unwrap().pos; c.grab_body(&mut w, anvil, p) }, GrabResult::Grabbed { mode: GrabMode::Carry, .. }));
    let mut clock = Clock::new(60.0);
    let mut ends = Vec::new();
    while clock.virt < 2.0 && ends.is_empty() {
        clock.frame(&mut w, &mut c, &|_| up, &mut ends);
    }
    assert_eq!(ends, vec![CarryEnd::Broke(anvil)]);
    assert!(c.held().is_none());
    assert_eq!(w.layer(anvil), Some(Layer::DEBRIS), "layer restored");
}

/// The holder's own capsule never touches what it carries.
#[test]
fn held_item_ignores_its_holder() {
    let (mut w, base) = world(PhysicsParams::default());
    let body = item(&mut w, base, Vec3::new(0.0, 0.0, -0.6), 0.1, 2.0);
    let capsule = w.insert(&BodyDesc::new(BodyKind::Kinematic, at(base.offset(Vec3::Y * 0.9)), ShapeRef::Capsule { r: 0.3, half: 0.6 }, Layer::CHARACTER));
    w.step();
    let mut c = Carrier::new(HolderLimits::default());
    c.holder_body = Some(capsule);
    // Look straight down: the hold point is inside the capsule.
    c.record_view(0.0, view(base.offset(Vec3::Y * EYE), 0.0, -0.9));
    assert!(matches!({ let p = w.pose(body).unwrap().pos; c.grab_body(&mut w, body, p) }, GrabResult::Grabbed { .. }));
    let down = view(base.offset(Vec3::Y * EYE), 0.0, -1.5);
    let mut clock = Clock::new(60.0);
    let mut ends = Vec::new();
    while clock.virt < 1.5 {
        clock.frame(&mut w, &mut c, &|_| down, &mut ends);
    }
    let (_, pose) = c.render_pose(&down, 0.0).unwrap();
    let lag = pose.pos.delta(w.pose(body).unwrap().pos).length();
    assert!(ends.is_empty(), "{ends:?}");
    // At rest in the hand, inside the capsule (not shoved out of it).
    let hand = down.pos.offset(down.rotation * Vec3::NEG_Z * (c.params.hold_distance + 0.1 * 3f32.sqrt()));
    let d = hand.delta(w.pose(body).unwrap().pos).length();
    assert!(d < 0.05 && lag < 0.05, "{d} {lag}");
    c.drop(&mut w);
    assert_eq!(w.layer(body), Some(Layer::DEBRIS));
}
