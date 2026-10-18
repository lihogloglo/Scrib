//! Cloth solver gates (physics.md "Verification", cloth): hem stretch ≤ 5 %,
//! no particle inside a capsule at run speed, teleports stable, cost.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::f32::consts::{PI, TAU};

use super::*;

const ROWS: usize = 5;
const COLS: usize = 10;

/// A full robe: a 10 × 5 ring from the waist (y 1.0, radius 0.17) flaring
/// to the hem (y 0.15, radius 0.44), every particle skinned to the pelvis.
/// A robe much narrower than the running stride can't hold the legs
/// without stretching: the collisions run last and win (as in Chaos), so
/// its hem would stretch rather than let the legs through.
fn robe() -> ClothLattice {
    let mut rest = Vec::new();
    for r in 0..ROWS {
        let f = r as f32 / (ROWS - 1) as f32;
        let (y, radius) = (1.0 - 0.85 * f, 0.17 + 0.27 * f);
        for c in 0..COLS {
            let a = TAU * c as f32 / COLS as f32;
            rest.push(Vec3::new(radius * a.sin(), y, radius * a.cos()));
        }
    }
    ClothLattice::grid(&GridSpec::default(), &rest, &[0; ROWS * COLS])
}

/// A synthetic runner in character space (+Z forward): pelvis bob and legs
/// swinging about the hips at `speed`.
struct Runner {
    lattice: ClothLattice,
    state: ClothState,
    params: ClothParams,
    animated: Vec<Vec3>,
    capsules: Vec<Capsule>,
    phase: f32,
}

impl Runner {
    fn new() -> Self {
        let lattice = robe();
        let animated = lattice.particles.iter().map(|p| p.rest).collect();
        Self { lattice, state: ClothState::new(), params: ClothParams::default(), animated, capsules: Vec::new(), phase: 0.0 }
    }

    /// Poses the body for `speed` (m/s) after `dt` and steps the cloth.
    fn update(&mut self, speed: f32, yaw_rate: f32, dt: f32) -> StepReport {
        self.update_moved(RootDelta::new(Vec3::Z * speed * dt, yaw_rate * dt), speed, dt)
    }

    fn update_moved(&mut self, moved: RootDelta, speed: f32, dt: f32) -> StepReport {
        let run = (speed / 6.0).clamp(0.0, 1.0);
        // Stride cadence 0.8 Hz walking to 1.4 Hz running; the hip flexes up
        // to 45° and extends up to 15° (a running stride).
        self.phase += TAU * (0.8 + 0.6 * run) * dt * f32::from(u8::from(speed > 0.05));
        let bob = 0.03 * run * (2.0 * self.phase).sin();
        for (a, p) in self.animated.iter_mut().zip(&self.lattice.particles) {
            *a = p.rest + Vec3::Y * bob;
        }
        let swing = 0.78 * run;
        let leg = |side: f32, phase: f32| {
            let hip = Vec3::new(0.09 * side, 0.92 + bob, 0.0);
            let thigh = swing * phase.sin() * if phase.sin() < 0.0 { 1.0 / 3.0 } else { 1.0 };
            let shin = thigh - 0.9 * run * (0.5 - 0.5 * phase.cos());
            let knee = hip + 0.45 * Vec3::new(0.0, -thigh.cos(), thigh.sin());
            let foot = knee + 0.45 * Vec3::new(0.0, -shin.cos(), shin.sin());
            [Capsule { a: hip, b: knee, radius: 0.065 }, Capsule { a: knee, b: foot, radius: 0.05 }]
        };
        self.capsules.clear();
        self.capsules.push(Capsule { a: Vec3::new(-0.1, 0.95 + bob, 0.0), b: Vec3::new(0.1, 0.95 + bob, 0.0), radius: 0.09 });
        self.capsules.extend(leg(1.0, self.phase));
        self.capsules.extend(leg(-1.0, self.phase + PI));
        let inputs = ClothInputs { animated: &self.animated, capsules: &self.capsules, moved, dt, acceleration: Vec3::ZERO };
        self.state.step(&self.lattice, &self.params, &inputs)
    }

    fn free(&self) -> impl Iterator<Item = (usize, Vec3)> + '_ {
        self.state.positions().iter().copied().enumerate().filter(|&(i, _)| self.lattice.particles[i].inv_mass > 0.0)
    }

    /// Deepest particle penetration into a capsule (the bare body, without
    /// the cloth thickness), m; ≤ 0 means nothing is inside.
    fn penetration(&self) -> f32 {
        self.free().flat_map(|(_, p)| self.capsules.iter().map(move |c| -c.distance(p))).fold(f32::NEG_INFINITY, f32::max)
    }

    /// Fastest particle relative to the character, m/s.
    fn max_speed(&self) -> f32 {
        self.state.pos.iter().zip(&self.state.prev).map(|(p, q)| p.distance(*q) / CLOTH_SUBSTEP).fold(0.0, f32::max)
    }
}

#[derive(Default, Debug)]
struct Peaks {
    hem: f32,
    all: f32,
    /// Update of the worst structural stretch.
    worst: u32,
    penetration: f32,
    speed: f32,
}

impl Peaks {
    fn take(&mut self, r: &Runner, frame: u32) {
        let pos = r.state.positions();
        assert!(pos.iter().all(|p| p.is_finite()), "cloth went non-finite");
        self.hem = self.hem.max(r.lattice.max_stretch(pos, true));
        let all = r.lattice.max_stretch(pos, false);
        if all > self.all {
            (self.all, self.worst) = (all, frame);
        }
        self.penetration = self.penetration.max(r.penetration());
        self.speed = self.speed.max(r.max_speed());
    }
}

/// Stand, speed up to a 6 m/s run, run, turn 180° at speed, run, stop dead.
fn run_course(fps: f32) -> Peaks {
    let dt = 1.0 / fps;
    let mut r = Runner::new();
    let mut peaks = Peaks { penetration: f32::NEG_INFINITY, ..Peaks::default() };
    let frames = (5.0 * fps) as u32;
    for f in 0..frames {
        let t = f as f32 * dt;
        let speed = match t {
            t if t < 0.5 => 0.0,
            t if t < 1.0 => 12.0 * (t - 0.5),
            t if t < 4.0 => 6.0,
            _ => 0.0,
        };
        let yaw_rate = if (2.5..3.0).contains(&t) { PI / 0.5 } else { 0.0 };
        let report = r.update(speed, yaw_rate, dt);
        assert!(!report.teleported, "a run is not a teleport (t {t})");
        if f > 0 {
            peaks.take(&r, f);
        }
    }
    peaks
}

#[test]
fn run_turn_stop_holds_stretch_and_capsules() {
    for fps in [30.0, 60.0, 144.0] {
        let p = run_course(fps);
        eprintln!(
            "cloth run @ {fps} fps: hem stretch {:.2} %, structural {:.2} % (at {:.2} s), penetration {:.2} mm, peak speed {:.2} m/s",
            p.hem * 100.0,
            p.all * 100.0,
            p.worst as f32 / fps,
            p.penetration * 1000.0,
            p.speed
        );
        assert!(p.hem <= 0.05, "hem stretch {:.2} % at {fps} fps", p.hem * 100.0);
        assert!(p.penetration <= 1e-4, "a particle {:.2} mm inside a capsule at {fps} fps", p.penetration * 1000.0);
    }
}

#[test]
fn inertia_trails_on_acceleration_only() {
    // At a steady run the robe hangs as when standing (Galilean); speeding
    // up swings the hem back (−Z), stopping swings it forward.
    let mut r = Runner::new();
    r.params.inertia_linear = 1.0;
    let hem_z = |r: &Runner| r.state.positions()[(ROWS - 1) * COLS..].iter().map(|p| p.z).sum::<f32>() / COLS as f32;
    let dt = 1.0 / 60.0;
    for _ in 0..120 {
        r.update(0.0, 0.0, dt);
    }
    let rest = hem_z(&r);
    r.update(6.0, 0.0, dt);
    r.update(6.0, 0.0, dt);
    let accel = hem_z(&r) - rest;
    for _ in 0..300 {
        r.update(6.0, 0.0, dt);
    }
    let steady = hem_z(&r) - rest;
    r.update(0.0, 0.0, dt);
    r.update(0.0, 0.0, dt);
    let stop = hem_z(&r) - rest;
    eprintln!("hem z offset: speed-up {accel:.3} m, steady run {steady:.3} m, stop {stop:.3} m");
    assert!(accel < -0.01, "speeding up should swing the hem back ({accel})");
    assert!(stop > steady + 0.01, "stopping should swing the hem forward ({stop} vs {steady})");
}

#[test]
fn teleport_is_stable() {
    let dt = 1.0 / 60.0;
    let mut r = Runner::new();
    for _ in 0..90 {
        r.update(6.0, 0.0, dt);
    }
    let before = r.state.positions().to_vec();
    let report = r.update_moved(RootDelta::new(Vec3::new(30.0, -2.0, 40.0), 2.5), 6.0, dt);
    assert!(report.teleported);
    // The shape is kept and the velocity dropped.
    let drift = before.iter().zip(r.state.positions()).map(|(a, b)| a.distance(*b)).fold(0.0, f32::max);
    assert!(drift < 0.05, "teleport moved the cloth {drift} m in character space");
    let mut peaks = Peaks { penetration: f32::NEG_INFINITY, ..Peaks::default() };
    for f in 0..60 {
        r.update(6.0, 0.0, dt);
        peaks.take(&r, f);
        for (i, p) in r.free() {
            let limit = r.lattice.particles[i].max_distance + 1e-4;
            assert!(p.distance(r.animated[i]) <= limit, "particle {i} past its max distance");
        }
    }
    eprintln!("after teleport: {peaks:?}");
    assert!(peaks.speed < 6.0, "cloth whipped at {} m/s after a teleport", peaks.speed);
    assert!(peaks.hem <= 0.05 && peaks.penetration <= 1e-4);
}

#[test]
fn seeded_lattice_is_at_rest_frame() {
    let mut r = Runner::new();
    let report = r.update(0.0, 0.0, 1.0 / 60.0);
    assert!(report.seeded);
    let mut q = Vec::new();
    r.state.rotations(&r.lattice, &mut q);
    assert_eq!(q.len(), ROWS * COLS);
    // The bob moves the particles, not their orientation.
    assert!(q.iter().all(|q| q.angle_between(Quat::IDENTITY) < 1e-3));
    assert_eq!(r.lattice.particles.iter().filter(|p| p.inv_mass == 0.0).count(), COLS);
    assert_eq!(r.lattice.tethers.len(), (ROWS - 1) * COLS);
}

/// Cost per robe update at 60 fps (4 substeps), release only:
/// `cargo test -p sf_anim --release --lib cloth -- --ignored --nocapture`.
#[test]
#[ignore = "timing; run in release"]
fn cost_per_robe() {
    let mut r = Runner::new();
    for fps in [60.0f32, 30.0] {
        let dt = 1.0 / fps;
        for _ in 0..60 {
            r.update(6.0, 0.0, dt);
        }
        let frames = 20_000;
        let start = std::time::Instant::now();
        for i in 0..frames {
            r.update(6.0, if i % 200 < 30 { PI } else { 0.0 }, dt);
        }
        let ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(frames);
        eprintln!("cloth robe ({} particles, {} edges): {:.4} ms per update at {fps} fps", r.lattice.particles.len(), r.lattice.edges.len(), ms);
        if !cfg!(debug_assertions) && fps >= 60.0 {
            assert!(ms <= 0.03, "{ms} ms per robe");
        }
    }
}
