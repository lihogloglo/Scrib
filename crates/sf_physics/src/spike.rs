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

/// P5 follow-up: the walkers' probe mix (per mover: a horizontal capsule
/// sweep, a downward sphere cast, two feet rays) over the same R0 statics as
/// a `StaticWorld` + analytic terrain (the physics-off path) and through
/// `PhysicsProbe` with and without the terrain heightfields.
/// `cargo test -p sf_physics --release -- --ignored --nocapture probe_bench`.
#[allow(clippy::too_many_lines)]
pub(crate) fn probe_bench() {
    use sf_controller::{GroundProbe, HeightSurface, Nearest, ProbeQuery, StaticShape, StaticWorld, TerrainProbe};

    struct Analytic(WorldPos);
    impl HeightSurface for Analytic {
        fn height(&self, p: WorldPos) -> Option<f32> {
            let d = self.0.delta_f32(p);
            Some(terrain_height(d.x, d.z))
        }
        fn normal(&self, p: WorldPos) -> Option<Vec3> {
            let d = self.0.delta_f32(p);
            let e = 0.25;
            let dx = terrain_height(d.x + e, d.z) - terrain_height(d.x - e, d.z);
            let dz = terrain_height(d.x, d.z + e) - terrain_height(d.x, d.z - e);
            Some(Vec3::new(-dx, 2.0 * e, -dz).normalize())
        }
        fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
            let mut t = 0.0;
            while t <= max {
                let p = origin.offset(dir * t);
                if p.to_f64().y < f64::from(self.height(p)?) {
                    return Some((t, self.normal(p)?));
                }
                t += 0.25;
            }
            None
        }
    }

    let anchor = WorldPos::new(IVec3::new(-3, 0, 12), Vec3::ZERO);
    let mut rng = Rng(0x5eed);
    let shapes: Vec<_> = (0..120)
        .map(|_| {
            let h = Vec3::new(rng.range(0.3, 6.0), rng.range(0.3, 5.0), rng.range(0.3, 6.0));
            let k = 2 + (rng.next() * 9.0) as usize;
            (std::sync::Arc::new(StaticShape::new(&box_mesh(h, k)).unwrap()), h)
        })
        .collect();
    let mesh_shapes: Vec<_> = shapes.iter().map(|(s, _)| MeshShape::from_static(s)).collect();
    let mut world = PhysicsWorld::new(anchor, PhysicsParams::default());
    let mut statics = StaticWorld::new(anchor);
    let n = 129;
    for cz in 0..8 {
        for cx in 0..8 {
            let min = Vec3::new(-256.0 + 64.0 * cx as f32, 0.0, -256.0 + 64.0 * cz as f32);
            let heights: Vec<f32> =
                (0..n * n).map(|i| terrain_height(min.x + (i % n) as f32 * 0.5, min.z + (i / n) as f32 * 0.5)).collect();
            let hs = HeightShape::new(n, 0.5, &heights).unwrap();
            world.insert(&BodyDesc::fixed(at(anchor.offset(min + Vec3::new(32.0, 0.0, 32.0))), ShapeRef::Heightfield(hs), Layer::TERRAIN));
        }
    }
    for _ in 0..2400 {
        let i = (rng.next() * shapes.len() as f32) as usize % shapes.len();
        let h = shapes[i].1;
        let r = 190.0 * rng.next().sqrt();
        let a = rng.range(0.0, std::f32::consts::TAU);
        let (x, z) = (r * a.cos(), r * a.sin());
        let pos = anchor.offset(Vec3::new(x, terrain_height(x, z) + h.y - 0.2, z));
        let pose = WorldTransform { pos, rotation: Quat::from_rotation_y(rng.range(0.0, std::f32::consts::TAU)), scale: Vec3::ONE };
        world.insert(&BodyDesc::fixed(pose, ShapeRef::Mesh(mesh_shapes[i].clone()), Layer::STATIC));
        statics.insert(&shapes[i].0, pose);
    }
    statics.rebuild();
    world.step();

    // 150 movers on open ground and near statics.
    let mut queries = Vec::new();
    for _ in 0..150 {
        let r = 190.0 * rng.next().sqrt();
        let a = rng.range(0.0, std::f32::consts::TAU);
        let (x, z) = (r * a.cos(), r * a.sin());
        let feet = anchor.offset(Vec3::new(x, terrain_height(x, z), z));
        let dir = Vec3::new(a.cos(), 0.0, a.sin());
        queries.push(ProbeQuery::capsule(feet.offset(Vec3::Y * 0.95), dir, 0.06, 0.3, 0.6));
        queries.push(ProbeQuery::sphere(feet.offset(Vec3::Y * 0.65), Vec3::NEG_Y, 0.9, 0.25));
        queries.push(ProbeQuery::down(feet.offset(Vec3::new(0.1, 0.5, 0.0)), 1.0));
        queries.push(ProbeQuery::down(feet.offset(Vec3::new(-0.1, 0.5, 0.0)), 1.0));
    }
    let mut out = vec![None; queries.len()];
    let terrain = TerrainProbe(Analytic(anchor));
    let time = |name: &str, p: &dyn GroundProbe, out: &mut Vec<_>| {
        let mut ms = Vec::new();
        for _ in 0..40 {
            let t = Instant::now();
            p.probe(&queries, out);
            ms.push(t.elapsed().as_secs_f64() * 1e3);
        }
        let s = stats(ms);
        let hits = out.iter().filter(|h| h.is_some()).count();
        println!("{name:<44} mean {:.3} ms  p95 {:.3}  max {:.3}  ({hits} hits / {})", s.mean, s.p95, s.max, out.len());
    };
    // Per kind: which query shape costs what.
    for (kind, k) in [("capsule sweeps", 0), ("sphere down", 1), ("rays down", 2)] {
        let sub: Vec<_> = queries.iter().enumerate().filter(|(i, _)| i % 4 == k || (k == 2 && i % 4 == 3)).map(|(_, q)| *q).collect();
        let mut o = vec![None; sub.len()];
        for (name, p) in [
            ("statics", &statics as &dyn GroundProbe),
            ("physics STATIC", &world.probe(Layer::STATIC)),
            ("physics TERRAIN", &world.probe(Layer::TERRAIN)),
        ] {
            let t = Instant::now();
            for _ in 0..40 {
                p.probe(&sub, &mut o);
            }
            println!("  {kind:<16} {name:<16} {:.2} us/query", t.elapsed().as_secs_f64() * 1e6 / (40 * sub.len()) as f64);
        }
    }
    time("old: Nearest(terrain, StaticWorld)", &Nearest(&[&terrain, &statics]), &mut out);
    time("P5: physics STATIC|TERRAIN|PROP", &world.probe(Layer::STATIC | Layer::TERRAIN | Layer::PROP), &mut out);
    time("new: Nearest(terrain, physics STATIC|PROP)", &Nearest(&[&terrain, &world.probe(Layer::STATIC | Layer::PROP)]), &mut out);
}
