//! Walkers on slopes and stairs (owner check WK-4): the real humanoid graph
//! on the male skeleton, driven like `sf_controller_bevy` drives it (the
//! kinematic mover over static ramps and stairs, the measured feed with its
//! moving hysteresis, per-leg ground probes at the graph's probe origins).
//! Both feet must keep cycling, planted feet must stay put in the world,
//! and the stride must track the distance travelled. Skips (passes) when no
//! Morrowind install is detected. `-- --nocapture` prints the reports.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::sync::Arc;

use glam::{Affine3A, Quat, Vec2, Vec3};
use mw_anim::Animation;
use mw_anim::rig;
use mw_graphs::build::params as p;
use mw_graphs::sim::Sim;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim_graph::GroundSample;
use sf_controller::{
    CollisionBuilder, CollisionMesh, GroundProbe, HeightSurface, KinematicMover, Locomotion, LocomotionIntent, MoveInput, MoverParams, Nearest,
    RootMotion, StaticShape, StaticWorld, SurfaceId, TerrainProbe,
};
use sf_coords::{WorldPos, WorldTransform};
use sf_vfs::{Vfs, VfsPath};

const DT: f32 = 1.0 / 60.0;
/// NPC walk speed at Speed 50 (`fMinWalkSpeed`..`fMaxWalkSpeed`).
const SPEED: f32 = 2.1;

// --- The game data ------------------------------------------------------------------------

fn humanoid_sim() -> Option<Sim> {
    humanoid_sim_of(0)
}

fn humanoid_sim_of(index: usize) -> Option<Sim> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let set = &HUMANOID_SETS[index];
    let model = mw_nif::load(&read(set.skeleton).unwrap()).unwrap();
    let skeleton = rig::skeleton(&model).unwrap();
    let mut anims = Vec::new();
    for a in set.animations {
        let Some(bytes) = read(a) else { continue };
        let anim = Animation::from_file(a, &bytes).unwrap();
        if !anim.text_keys.is_empty() {
            anims.push(((*a).to_owned(), anim));
        }
    }
    let mut set = AnimSet::new(set.name, skeleton, anims).with_humanoid_rig();
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let clips = set.binding_clips(&v.graph).unwrap();
    Some(Sim::new(v.graph, set.skeleton.clone(), clips).unwrap())
}

// --- The world ----------------------------------------------------------------------------

struct Flat;

impl HeightSurface for Flat {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        let oy = WorldPos::ORIGIN.delta_f32(origin).y;
        if dir.y.abs() < 1e-6 {
            return None;
        }
        let t = -oy / dir.y;
        (0.0..=max).contains(&t).then_some((t, Vec3::Y))
    }
}

fn box_mesh(h: Vec3) -> CollisionMesh {
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
    b.add(&corners, &idx, Affine3A::IDENTITY, SurfaceId(3));
    b.finish()
}

struct World {
    terrain: TerrainProbe<Flat>,
    statics: StaticWorld,
}

impl World {
    fn new() -> Self {
        Self { terrain: TerrainProbe(Flat), statics: StaticWorld::new(WorldPos::ORIGIN) }
    }

    fn add(&mut self, half: Vec3, center: Vec3, rotation: Quat) {
        let s = Arc::new(StaticShape::new(&box_mesh(half)).unwrap());
        self.statics.insert(&s, WorldTransform { pos: WorldPos::ORIGIN.offset(center), rotation, scale: Vec3::ONE });
    }

    /// A ramp rising along +X from x = 0 at `deg`, `len` m along the slope,
    /// with a landing at the top (z in [-3, 3]). Returns the top's x and y.
    fn ramp(deg: f32, len: f32) -> (Self, Vec2) {
        let mut w = Self::new();
        let a = deg.to_radians();
        let r = Quat::from_rotation_z(a);
        let mid_top = Vec3::new(len / 2.0 * a.cos(), len / 2.0 * a.sin(), 0.0);
        w.add(Vec3::new(len / 2.0, 0.5, 3.0), mid_top - r * Vec3::Y * 0.5, r);
        let top = Vec2::new(len * a.cos(), len * a.sin());
        w.add(Vec3::new(4.0, top.y / 2.0, 3.0), Vec3::new(top.x + 4.0, top.y / 2.0, 0.0), Quat::IDENTITY);
        (w, top)
    }

    /// `n` steps up along +X from x = 0 (`rise` and `tread`), then a landing.
    fn stairs(n: usize, rise: f32, tread: f32) -> (Self, Vec2) {
        let mut w = Self::new();
        let end = n as f32 * tread + 4.0;
        for i in 0..n {
            let (x0, h) = (i as f32 * tread, (i + 1) as f32 * rise);
            w.add(Vec3::new((end - x0) / 2.0, h / 2.0, 3.0), Vec3::new(x0.midpoint(end), h / 2.0, 0.0), Quat::IDENTITY);
        }
        (w, Vec2::new(n as f32 * tread, n as f32 * rise))
    }
}

impl GroundProbe for World {
    fn probe(&self, queries: &[sf_controller::ProbeQuery], out: &mut [Option<sf_controller::GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

// --- The walk -----------------------------------------------------------------------------

/// What one walk did on the measured stretch.
#[derive(Debug, Default)]
struct Report {
    /// Body travel (horizontal), m.
    travel: f32,
    seconds: f32,
    /// Per leg: plants (contact rising above 0.5), the planted feet's
    /// horizontal world slide, m/s of stance, and how far the foot moved.
    plants: Vec<u32>,
    slide: Vec<f32>,
    foot_travel: Vec<f32>,
    /// Frames the feed said "not moving" while the body moved.
    idle_frames: u32,
    /// Lowest walk-cycle playback rate seen, as a fraction of the stride.
    min_measured: f32,
    stats: sf_anim_graph::FootStats,
}

/// Walks from `start` toward `dir` at `SPEED` for `seconds`, measuring
/// while the body's x is inside `window`. Drives the graph exactly like
/// `sf_controller_bevy`'s step and feed systems do.
#[allow(clippy::too_many_lines)]
fn walk(sim: &mut Sim, world: &World, start: Vec3, dir: Vec3, seconds: f32, window: (f32, f32), trace: bool) -> Report {
    walk_at(sim, world, (start, dir * SPEED, None), seconds, window, trace)
}

#[allow(clippy::too_many_lines)]
fn walk_at(sim: &mut Sim, world: &World, (start, velocity, facing): (Vec3, Vec3, Option<f32>), seconds: f32, window: (f32, f32), trace: bool) -> Report {
    let dir = velocity.normalize();
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(start), facing.unwrap_or(dir.x.atan2(dir.z)));
    let params = MoverParams::default();
    let mut stepper = KinematicMover::new();
    let intent = LocomotionIntent { desired_velocity: velocity, desired_facing: facing, ..LocomotionIntent::IDLE };
    let legs: Vec<(usize, Option<usize>)> = sim.graph.legs.iter().map(|l| (usize::from(l.effector()), l.contact.map(usize::from))).collect();
    // Settle standing on the start, probes included.
    sim.set(p::MOVING, 0.0);
    sim.velocity = Vec3::ZERO;
    let (mut measured, mut moving) = (Vec3::ZERO, false);
    let mut r = Report { plants: vec![0; legs.len()], slide: vec![0.0; legs.len()], foot_travel: vec![0.0; legs.len()], min_measured: f32::MAX, ..Report::default() };
    let mut stance = vec![0.0f32; legs.len()];
    let mut last: Vec<Option<(Vec3, f32)>> = vec![None; legs.len()];
    let mut prev_body: Option<Vec3> = None;
    let frames = (seconds / DT).round() as usize;
    for frame in 0..frames + 60 {
        let walking = frame >= 60;
        let it = if walking { intent } else { LocomotionIntent::IDLE };
        stepper.step(MoveInput { loco: &mut loco, intent: it, params: &params, root: RootMotion::default() }, world, DT);
        // sf_controller_bevy: measured motion and the moving hysteresis (walkers: 0.5 / 0.25 m/s).
        let moved = loco.moved;
        let raw = (Quat::from_rotation_y(-moved.yaw) * moved.translation / DT).with_y(0.0);
        measured = raw.lerp(measured, 0.5f32.powf(DT / 0.05));
        let flat = Vec2::new(-measured.x, measured.z);
        moving = flat.length() > if moving { 0.25 } else { 0.5 };
        sim.set(p::MOVING, if moving { 1.0 } else { 0.0 });
        sim.set_vec2(p::MOVE, if flat.length() > 1e-3 { flat.normalize() } else { Vec2::Y });
        sim.velocity = measured;
        sim.moved = moved;
        // Foot probes at the last update's origins, in the current frame.
        let f = loco.frame();
        let queries: Vec<_> = f.foot_queries(&sim.out.probe_origins, 0.5, 0.75).collect();
        let mut hits = vec![None; queries.len()];
        world.probe(&queries, &mut hits);
        for (slot, hit) in sim.ground.iter_mut().zip(&hits) {
            *slot = hit.as_ref().map(|h| {
                let (point, normal) = f.ground_sample(h);
                GroundSample { point, normal }
            });
        }
        sim.run(DT, DT);
        if !walking {
            continue;
        }
        let body = WorldPos::ORIGIN.delta_f32(loco.position);
        // The first second of walking is the start (idle to walk), not measured.
        let inside = frame >= 120 && body.x >= window.0 && body.x <= window.1;
        if trace && frame % 4 == 0 {
            let feet: Vec<String> = legs
                .iter()
                .map(|&(j, c)| {
                    let w = WorldPos::ORIGIN.delta_f32(f.to_world(sim.out.model.translation(j)));
                    format!("({:5.2},{:5.2},{:5.2} c{:.1})", w.x, w.y, w.z, c.map_or(0.0, |c| sim.out.curves[c]))
                })
                .collect();
            eprintln!(
                "t{:5.2} body ({:5.2},{:5.2},{:5.2}) v {:4.2} mov {} {} feet {} {:?}",
                frame as f32 * DT,
                body.x,
                body.y,
                body.z,
                flat.length(),
                u8::from(moving),
                sim.state(mw_graphs::build::LOCOMOTION).unwrap_or("?"),
                feet.join(" "),
                sim.out.feet
            );
        }
        if !inside {
            last.fill(None);
            prev_body = None;
            continue;
        }
        if let Some(b) = prev_body {
            r.travel += Vec2::new(body.x - b.x, body.z - b.z).length();
        }
        prev_body = Some(body);
        r.seconds += DT;
        r.idle_frames += u32::from(!moving);
        r.min_measured = r.min_measured.min(flat.length());
        r.stats += sim.out.feet;
        for (k, &(j, c)) in legs.iter().enumerate() {
            let w = WorldPos::ORIGIN.delta_f32(f.to_world(sim.out.model.translation(j)));
            let contact = c.map_or(0.0, |c| sim.out.curves[c]);
            if let Some((pw, pc)) = last[k] {
                let d = Vec2::new(w.x - pw.x, w.z - pw.z).length();
                r.foot_travel[k] += d;
                if contact > 0.5 && pc <= 0.5 {
                    r.plants[k] += 1;
                    stance[k] = 0.0;
                } else if contact > 0.5 {
                    stance[k] += DT;
                    // Mid-stance only: the lock blends in and out over 0.1 s.
                    if stance[k] > 0.1 {
                        r.slide[k] += d;
                    }
                }
            }
            last[k] = Some((w, contact));
        }
    }
    for s in &mut r.slide {
        *s /= r.seconds.max(1e-3);
    }
    r
}

fn check(name: &str, r: &Report) {
    eprintln!("{name}: {r:?}");
    let expect = r.travel / 2.2;
    for k in 0..r.plants.len() {
        assert!(r.plants[k] as f32 >= expect * 0.7, "{name}: leg {k} planted {} times over {:.1} m (stride 2.2 m): {r:?}", r.plants[k], r.travel);
        assert!(r.slide[k] <= 0.02, "{name}: leg {k} slides {:.3} m/s while planted: {r:?}", r.slide[k]);
        assert!((r.foot_travel[k] - r.travel).abs() < 0.25 * r.travel, "{name}: leg {k} moved {:.2} m, body {:.2} m: {r:?}", r.foot_travel[k], r.travel);
    }
    assert_eq!(r.idle_frames, 0, "{name}: fed as standing while walking: {r:?}");
}

#[test]
fn walkers_keep_their_stride_on_ramps_and_stairs() {
    let Some(mut sim) = humanoid_sim() else { return };
    let trace = std::env::var_os("SLOPE_TRACE").is_some();
    let flat = World::new();
    let r = walk(&mut sim, &flat, Vec3::new(-4.0, 0.0, 0.0), Vec3::X, 5.0, (-1.0, 5.0), trace);
    check("flat", &r);
    for (index, name) in [(1, "flat female"), (2, "flat beast")] {
        let mut s = humanoid_sim_of(index).unwrap();
        let r = walk(&mut s, &flat, Vec3::new(-4.0, 0.0, 0.0), Vec3::X, 5.0, (-1.0, 5.0), trace);
        check(name, &r);
    }
    for deg in [15.0, 25.0, 30.0] {
        let (w, top) = World::ramp(deg, 8.0);
        let mut s = humanoid_sim().unwrap();
        let r = walk(&mut s, &w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, 6.0, (1.0, top.x - 1.0), trace);
        check(&format!("up {deg}°"), &r);
        let mut s = humanoid_sim().unwrap();
        let r = walk(&mut s, &w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, 6.0, (1.0, top.x - 1.0), trace);
        check(&format!("down {deg}°"), &r);
    }
    let (w, top) = World::stairs(12, 0.18, 0.32);
    let r = walk(&mut sim, &w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, 6.0, (0.5, top.x - 0.5), trace);
    check("stairs up", &r);
}

/// A path into ground too steep to climb, at a shallow angle: the mover
/// slides along the slope's contour at a quarter of the asked speed. The
/// walker must face that motion and keep a forward walk (it used to face
/// into the slope: the graph got a slow strafe, one leg stopped planting
/// and the body slid; owner check WK-4).
#[test]
fn walkers_turned_aside_by_steep_ground_keep_walking() {
    let Some(mut sim) = humanoid_sim() else { return };
    let (steep, _) = World::ramp(60.0, 4.0);
    let asked = Vec3::new(75f32.to_radians().sin(), 0.0, 75f32.to_radians().cos()) * SPEED;
    let r = walk_at(&mut sim, &steep, (Vec3::new(-0.5, 0.0, -3.0), asked, None), 6.0, (-9.0, 9.0), false);
    eprintln!("contour: {r:?}");
    assert!(r.min_measured > 0.4, "slides along the contour: {r:?}");
    for k in 0..r.plants.len() {
        assert!(r.plants[k] >= 2, "contour: leg {k} planted {} times over {:.1} m: {r:?}", r.plants[k], r.travel);
        assert!(r.slide[k] <= 0.02, "contour: leg {k} slides {:.3} m/s while planted: {r:?}", r.slide[k]);
    }
    assert_eq!(r.idle_frames, 0, "contour: fed as standing while walking: {r:?}");
}
