//! Task P1's backend spike: an R0-sized world (dense town: 2,400 statics
//! over 120 shared meshes, 64 terrain chunks, 600 sleeping items) idling,
//! then a 50-item pile collapsing plus 2 ragdolls (impulse joints vs
//! multibody, limp vs powered), and a rebase. Numbers go to stdout and
//! ROADMAP M4 P1. Run in release on a quiet machine.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]

use std::time::Instant;

use glam::{IVec3, Quat, Vec3};
use sf_controller::{CollisionBuilder, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::{at, ragdoll};
use crate::{BodyDesc, BodyKind, Drive, HeightShape, JointSolver, Layer, MeshShape, PhysicsParams, PhysicsWorld, ShapeRef};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 40) as f32) / (1u64 << 24) as f32
    }
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.next()
    }
}

fn terrain_height(x: f32, z: f32) -> f32 {
    3.0 * (x / 37.0).sin() + 2.0 * (z / 23.0).cos() + 0.5 * ((x + z) / 7.0).sin()
}

/// A box of half extents `h` with each face split k × k (12·k² triangles).
fn box_mesh(h: Vec3, k: usize) -> sf_controller::CollisionMesh {
    let mut b = CollisionBuilder::new(1e-4);
    for axis in 0..3 {
        for sign in [-1.0f32, 1.0] {
            let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
            let mut positions = Vec::new();
            for i in 0..=k {
                for j in 0..=k {
                    let mut p = [0.0f32; 3];
                    p[axis] = sign * h[axis];
                    p[u] = (i as f32 / k as f32 * 2.0 - 1.0) * h[u];
                    p[v] = (j as f32 / k as f32 * 2.0 - 1.0) * h[v];
                    positions.push(p);
                }
            }
            let mut idx = Vec::new();
            let at = |i: usize, j: usize| (i * (k + 1) + j) as u32;
            for i in 0..k {
                for j in 0..k {
                    idx.extend([at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j), at(i + 1, j + 1), at(i, j + 1)]);
                }
            }
            b.add(&positions, &idx, glam::Affine3A::IDENTITY, SurfaceId(1));
        }
    }
    b.finish()
}

struct R0 {
    world: PhysicsWorld,
    insert_ms: f64,
    bodies: usize,
    triangles: usize,
}

fn build_r0(anchor: WorldPos) -> R0 {
    let mut rng = Rng(0x5eed);
    let shapes: Vec<_> = (0..120)
        .map(|_| {
            let h = Vec3::new(rng.range(0.3, 6.0), rng.range(0.3, 5.0), rng.range(0.3, 6.0));
            let k = 2 + (rng.next() * 9.0) as usize;
            (MeshShape::new(&box_mesh(h, k)).unwrap(), h)
        })
        .collect();
    let mut descs = Vec::new();
    // Terrain: 8 × 8 chunks of 64 m, 129² samples at 0.5 m.
    let n = 129;
    for cz in 0..8 {
        for cx in 0..8 {
            let min = Vec3::new(-256.0 + 64.0 * cx as f32, 0.0, -256.0 + 64.0 * cz as f32);
            let heights: Vec<f32> =
                (0..n * n).map(|i| terrain_height(min.x + (i % n) as f32 * 0.5, min.z + (i / n) as f32 * 0.5)).collect();
            let hs = HeightShape::new(n, 0.5, &heights).unwrap();
            descs.push(BodyDesc::fixed(at(anchor.offset(min + Vec3::new(32.0, 0.0, 32.0))), ShapeRef::Heightfield(hs), Layer::TERRAIN));
        }
    }
    let mut triangles = 0;
    for _ in 0..2400 {
        let (shape, h) = &shapes[(rng.next() * shapes.len() as f32) as usize % shapes.len()];
        let r = 190.0 * rng.next().sqrt();
        let a = rng.range(0.0, std::f32::consts::TAU);
        let (x, z) = (r * a.cos(), r * a.sin());
        let pos = anchor.offset(Vec3::new(x, terrain_height(x, z) + h.y - 0.2, z));
        let pose = WorldTransform { pos, rotation: Quat::from_rotation_y(rng.range(0.0, std::f32::consts::TAU)), scale: Vec3::ONE };
        triangles += shape.triangle_count();
        descs.push(BodyDesc::fixed(pose, ShapeRef::Mesh(shape.clone()), Layer::STATIC));
    }
    for _ in 0..600 {
        let (x, z) = (rng.range(-180.0, 180.0), rng.range(-180.0, 180.0));
        let e = rng.range(0.05, 0.3);
        let (shape, layer) = match (rng.next() * 3.0) as u32 {
            0 => (ShapeRef::Box(Vec3::splat(e)), Layer::DEBRIS),
            1 => (ShapeRef::Sphere(e), Layer::DEBRIS),
            _ => (ShapeRef::Capsule { r: e * 0.5, half: e }, Layer::PROP),
        };
        let pos = anchor.offset(Vec3::new(x, terrain_height(x, z) + 1.6 * e, z));
        descs.push(BodyDesc::item(at(pos), shape, layer, Some(rng.range(0.3, 20.0))));
    }
    let mut world = PhysicsWorld::new(anchor, PhysicsParams::default());
    let t = Instant::now();
    for d in &descs {
        world.insert(d);
    }
    let insert_ms = t.elapsed().as_secs_f64() * 1e3;
    R0 { world, insert_ms, bodies: descs.len(), triangles }
}

struct Stats {
    mean: f64,
    p95: f64,
    max: f64,
}

fn stats(mut ms: Vec<f64>) -> Stats {
    let mean = ms.iter().sum::<f64>() / ms.len() as f64;
    ms.sort_by(f64::total_cmp);
    Stats { mean, p95: ms[(ms.len() * 95 / 100).min(ms.len() - 1)], max: *ms.last().unwrap() }
}

fn time_steps(w: &mut PhysicsWorld, n: usize) -> Stats {
    let ms = (0..n)
        .map(|_| {
            let t = Instant::now();
            w.step();
            w.drain_events().for_each(drop);
            t.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    stats(ms)
}

pub(crate) fn run() {
    let anchor = WorldPos::new(IVec3::new(-3, 0, 12), Vec3::ZERO);
    println!("sf_physics P1 spike (rapier3d 0.36.1, serial solver, 60 Hz). Gates: idle <= 0.3 ms, active <= 2 ms, rebase <= 2 ms.");

    let mut r0 = build_r0(anchor);
    let t = Instant::now();
    r0.world.step();
    let first = t.elapsed().as_secs_f64() * 1e3;
    println!(
        "R0 build: {} bodies (2,400 statics / {} triangles placed, 64 terrain chunks, 600 sleeping items): insert {:.2} ms ({:.2} us/body), first step (broad-phase build) {:.2} ms",
        r0.bodies,
        r0.triangles,
        r0.insert_ms,
        r0.insert_ms * 1e3 / r0.bodies as f64,
        first
    );
    time_steps(&mut r0.world, 60);
    let idle = time_steps(&mut r0.world, 600);
    println!("idle step: mean {:.4} ms, p95 {:.4}, max {:.4}; awake {}", idle.mean, idle.p95, idle.max, r0.world.awake_count());

    // Rebase by (1, 0, -1) cells and back: the call, then the following step.
    let mut rebase = Vec::new();
    let mut after = Vec::new();
    for k in 0..10 {
        let cell = anchor.cell + if k % 2 == 0 { IVec3::new(1, 0, -1) } else { IVec3::ZERO };
        let t = Instant::now();
        r0.world.rebase(cell);
        rebase.push(t.elapsed().as_secs_f64() * 1e3);
        let t = Instant::now();
        r0.world.step();
        after.push(t.elapsed().as_secs_f64() * 1e3);
        r0.world.drain_events().for_each(drop);
        time_steps(&mut r0.world, 10);
    }
    let (rb, af) = (stats(rebase), stats(after));
    println!(
        "rebase: call mean {:.3} ms max {:.3}; next step mean {:.3} ms max {:.3}; awake after {}",
        rb.mean,
        rb.max,
        af.mean,
        af.max,
        r0.world.awake_count()
    );

    for (solver, drive) in [
        (JointSolver::Impulse, None),
        (JointSolver::Multibody, None),
        (JointSolver::Impulse, Some(Drive::critical(20.0))),
        (JointSolver::Multibody, Some(Drive::critical(20.0))),
    ] {
        let mut r0 = build_r0(anchor);
        r0.world.step();
        let w = &mut r0.world;
        let mut rng = Rng(7);
        let ground = |x: f32, z: f32| terrain_height(x, z);
        for i in 0..50 {
            let (x, z) = (rng.range(-0.6, 0.6), rng.range(-0.6, 0.6));
            let pos = anchor.offset(Vec3::new(x, ground(x, z) + 0.3 + i as f32 * 0.12, z));
            let rot = Quat::from_euler(glam::EulerRot::YXZ, rng.range(0.0, 6.0), rng.range(0.0, 1.0), 0.0);
            let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, WorldTransform { rotation: rot, ..at(pos) }, ShapeRef::Box(Vec3::new(0.15, 0.1, 0.12)), Layer::PROP);
            w.insert(&BodyDesc { mass: Some(2.0), ..desc });
        }
        let mut joints = 0;
        for k in 0..2 {
            let x = 3.0 + 2.0 * k as f32;
            let rd = ragdoll(w, anchor.offset(Vec3::new(x, ground(x, 0.0) + 0.6, 0.0)), solver, drive);
            w.set_velocity(rd.bodies[2], Vec3::new(0.5, 0.0, 1.5), Vec3::ZERO);
            joints += rd.joints.len();
        }
        let active = time_steps(w, 180);
        let label = format!("{solver:?}/{}", if drive.is_some() { "powered" } else { "limp" });
        println!(
            "active (50-item pile + 2 x 15-body ragdolls, {joints} joints, {label}): first 3 s mean {:.3} ms, p95 {:.3}, max {:.3}; awake at end {}",
            active.mean,
            active.p95,
            active.max,
            w.awake_count()
        );
    }
}
