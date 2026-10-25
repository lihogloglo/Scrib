//! Walkers on slopes and stairs (owner check WK-4): the real humanoid graph
//! on the male skeleton, driven like `sf_controller_bevy` drives it (the
//! kinematic mover over static ramps and stairs, the measured feed with its
//! moving hysteresis, per-leg ground probes at the graph's probe origins).
//! Both feet must keep cycling, planted feet must stay put in the world,
//! and the stride must track the distance travelled. Skips (passes) when no
//! Morrowind install is detected. `-- --nocapture` prints the reports.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::fmt::Write as _;
use std::sync::Arc;

use glam::{Affine3A, Quat, Vec2, Vec3};
use mw_anim::Animation;
use mw_anim::rig;
use mw_graphs::build::params as p;
use mw_graphs::sim::Sim;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::foot_metrics::{FootFrame, FootMeter, FootMetrics, MeterRules};
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
    #[allow(unused_mut)]
    let mut clips = set.binding_clips(&v.graph).unwrap();
    if std::env::var_os("GAIT_BASELINE").is_some() {
        // Before the stride speed: speed matching by the roots' velocity.
        for c in &mut clips {
            Arc::make_mut(c).meta.stance_velocity = Vec3::ZERO;
        }
    }
    if std::env::var_os("GAIT_NOROLL").is_some() {
        // Before AN-9: the lock releases at heel-off, with the toe still down.
        for c in &mut clips {
            for t in Arc::make_mut(c).curves.iter_mut().filter(|t| t.name.starts_with("roll.")) {
                t.values.fill(0.0);
            }
        }
    }
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
    /// Per leg: simulation times of those plants.
    plant_times: Vec<Vec<f32>>,
    slide: Vec<f32>,
    foot_travel: Vec<f32>,
    /// Frames the feed said "not moving" while the body moved.
    idle_frames: u32,
    /// Lowest walk-cycle playback rate seen, as a fraction of the stride.
    min_measured: f32,
    stats: sf_anim_graph::FootStats,
    /// Per leg: `sf_anim::foot_metrics` over the measured stretch, for the
    /// foot joint and for its toe (the rig profile's, if any).
    feet: Vec<FootMetrics>,
    toes: Vec<Option<FootMetrics>>,
    /// Per leg: the sole's lowest clearance (heel or toe) as a metric with
    /// the ground at 0: penetration = deepest point, float = gap to the ground.
    soles: Vec<FootMetrics>,
    /// Per leg: the toe's horizontal world travel while it is down (within 2 cm of the
    /// ground) after the heel let go (the weight transfer: AN-9), and the number of such
    /// episodes (heel-offs with the toe still down).
    roll_slide: Vec<f32>,
    roll_episodes: Vec<u32>,
    /// Per leg: the largest hip-to-ankle distance over the leg's length while the
    /// contact is up (1 = straight), and the foot's fastest horizontal world speed, m/s.
    extension: Vec<f32>,
    foot_peak: Vec<f32>,
    /// Per measured frame: the graph's filtered ground grade and the trunk's
    /// pitch (chest to head, forward positive, radians) in character space (AN-21).
    grades: Vec<f32>,
    trunk: Vec<f32>,
}

/// Per leg (graph order): the foot joint's height over flat ground with
/// the sole down (the rig profile's `foot.sole`).
fn soles(sim: &Sim) -> Vec<f32> {
    let profile = rig::humanoid(&sim.skeleton).ok().and_then(|r| r.map.profile(&sim.skeleton).ok());
    sim.graph
        .legs
        .iter()
        .map(|l| {
            let e = usize::from(l.effector());
            profile
                .as_ref()
                .and_then(|p| p.limbs.iter().find(|m| usize::from(m.effector) == e))
                .and_then(|m| m.foot.as_ref())
                .map_or(0.0, |f| f.sole)
        })
        .collect()
}

/// Per leg (graph order): the toe joint (the rig profile's, else the
/// foot's first child) and its height over flat ground in the rest pose
/// (where the rig profile's `foot.sole` is measured).
fn toes(sim: &Sim) -> Vec<Option<(usize, f32)>> {
    let profile = rig::humanoid(&sim.skeleton).ok().and_then(|r| r.map.profile(&sim.skeleton).ok());
    let rest = sim.skeleton.rest_model();
    sim.graph
        .legs
        .iter()
        .map(|l| {
            let e = usize::from(l.effector());
            let foot = profile.as_ref()?.limbs.iter().find(|m| usize::from(m.effector) == e)?.foot.clone()?;
            let child = (0..rest.len()).find(|&j| sim.skeleton.parent(j) == Some(e));
            let toe = foot.toe.map(usize::from).or(child)?;
            Some((toe, rest[toe].translation.y - (rest[e].translation.y - foot.sole)))
        })
        .collect()
}

/// Ground height straight under `p` (a thin probe from half a meter above).
fn ground_at(world: &World, p: Vec3) -> Option<f32> {
    let q = sf_controller::ProbeQuery { origin: WorldPos::ORIGIN.offset(p + Vec3::Y * 0.5), dir: -Vec3::Y, max: 1.5, radius: 0.01, half_height: 0.0 };
    let mut hit = [None];
    world.probe(&[q], &mut hit);
    hit[0].as_ref().map(|h| WorldPos::ORIGIN.delta_f32(h.point).y)
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
    let mut r = Report { plants: vec![0; legs.len()], plant_times: vec![Vec::new(); legs.len()], slide: vec![0.0; legs.len()], foot_travel: vec![0.0; legs.len()], min_measured: f32::MAX, ..Report::default() };
    let mut stance = vec![0.0f32; legs.len()];
    let mut toe_last: Vec<Option<Vec3>> = vec![None; legs.len()];
    r.roll_slide = vec![0.0; legs.len()];
    r.roll_episodes = vec![0; legs.len()];
    r.extension = vec![0.0; legs.len()];
    r.foot_peak = vec![0.0; legs.len()];
    let chains: Vec<Vec<usize>> = sim.graph.legs.iter().map(|l| l.joints.iter().map(|&j| usize::from(j)).collect()).collect();
    let sole_heights = soles(sim);
    let mut meters: Vec<FootMeter> = sole_heights.iter().map(|&sole| FootMeter::new(MeterRules { sole, ..MeterRules::default() })).collect();
    let mut sole_meters: Vec<FootMeter> = sole_heights.iter().map(|_| FootMeter::new(MeterRules::default())).collect();
    let toe_joints = toes(sim);
    let mut toe_meters: Vec<Option<FootMeter>> = toe_joints.iter().map(|t| t.map(|(_, sole)| FootMeter::new(MeterRules { sole, ..MeterRules::default() }))).collect();
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
        r.grades.push(sim.out.grade);
        if let (Some(pelvis), Some(neck)) = (sim.skeleton.find("Bip01 Spine1"), sim.skeleton.find("Bip01 Head")) {
            let t = sim.out.model.translation(neck) - sim.out.model.translation(pelvis);
            r.trunk.push(t.z.atan2(t.y));
        }
        for (k, &(j, c)) in legs.iter().enumerate() {
            let w = WorldPos::ORIGIN.delta_f32(f.to_world(sim.out.model.translation(j)));
            let contact = c.map_or(0.0, |c| sim.out.curves[c]);
            let heel_ground = ground_at(world, w);
            meters[k].push(FootFrame { position: w, contact, ground: heel_ground }, DT);
            // The sole (heel and toe as a rigid foot): its lowest clearance,
            // so a foot on a stair nosing with the heel hanging counts as touching.
            let mut gap = heel_ground.map(|g| w.y - g - sole_heights[k]);
            if let (Some((toe, toe_sole)), Some(m)) = (toe_joints[k], &mut toe_meters[k]) {
                let t = WorldPos::ORIGIN.delta_f32(f.to_world(sim.out.model.translation(toe)));
                let toe_ground = ground_at(world, t);
                m.push(FootFrame { position: t, contact, ground: toe_ground }, DT);
                // Heel-off until the toe first lifts: the toe is the pivot, so it must not move
                // (the weight transfer).
                let heel_off = last[k].is_some_and(|(_, pc)| pc > 0.5) && contact <= 0.5;
                let down = toe_ground.is_some_and(|g| t.y - g - toe_sole < 0.02);
                if heel_off && down {
                    toe_last[k] = Some(t);
                    r.roll_episodes[k] += 1;
                } else if let (Some(pt), true) = (toe_last[k], contact <= 0.5) {
                    if down {
                        r.roll_slide[k] += Vec2::new(t.x - pt.x, t.z - pt.z).length();
                        toe_last[k] = Some(t);
                    } else {
                        toe_last[k] = None;
                    }
                } else {
                    toe_last[k] = None;
                }
                if let (Some(h), Some(g)) = (gap, toe_ground) {
                    gap = Some(h.min(t.y - g - toe_sole));
                }
            }
            sole_meters[k].push(FootFrame { position: w.with_y(gap.unwrap_or(0.0)), contact, ground: gap.map(|_| 0.0) }, DT);
            let chain = &chains[k];
            let at = |i: usize| sim.out.model.translation(chain[i]);
            let length: f32 = (1..chain.len()).map(|i| at(i).distance(at(i - 1))).sum();
            if contact > 0.5 {
                r.extension[k] = r.extension[k].max(at(0).distance(at(chain.len() - 1)) / length.max(1e-3));
            }
            if let Some((pw, pc)) = last[k] {
                let d = Vec2::new(w.x - pw.x, w.z - pw.z).length();
                r.foot_peak[k] = r.foot_peak[k].max(d / DT);
                r.foot_travel[k] += d;
                if contact > 0.5 && pc <= 0.5 {
                    r.plants[k] += 1;
                    r.plant_times[k].push(sim.time);
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
    r.feet = meters.iter().map(FootMeter::metrics).collect();
    r.toes = toe_meters.iter().map(|m| m.as_ref().map(FootMeter::metrics)).collect();
    r.soles = sole_meters.iter().map(FootMeter::metrics).collect();
    r
}

fn check(name: &str, r: &Report) {
    eprintln!("{name}: {r:?}");
    let expect = r.travel / 2.2;
    for k in 0..r.plants.len() {
        assert!(r.plants[k] as f32 >= expect * 0.7, "{name}: leg {k} planted {} times over {:.1} m (stride 2.2 m): {r:?}", r.plants[k], r.travel);
        assert!(r.slide[k] <= 0.02, "{name}: leg {k} slides {:.3} m/s while planted: {r:?}", r.slide[k]);
        // Per leg, a short window (stairs: 1.4 cycles) holds one swing more or less than the other leg's.
        assert!((r.foot_travel[k] - r.travel).abs() < 0.4 * r.travel, "{name}: leg {k} moved {:.2} m, body {:.2} m: {r:?}", r.foot_travel[k], r.travel);
    }
    let mean = r.foot_travel.iter().sum::<f32>() / r.foot_travel.len().max(1) as f32;
    assert!((mean - r.travel).abs() < 0.25 * r.travel, "{name}: legs moved {mean:.2} m on average, body {:.2} m: {r:?}", r.travel);
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

/// Feet on the surface at contact frames (owner checks WK-4, SB-5, AL-3;
/// animation-plan.md slice 5): walkers up and down the lab's 10/20/30°
/// ramps (and flat ground). Every planted frame's sole (heel and toe, a rigid foot) must
/// touch the ground within 2 cm: no point deeper, lowest point no higher.
/// `-- --nocapture` prints the table (locomotion.md "Foot metrics").
#[test]
fn feet_meet_the_ground_on_ramps() {
    feet_meet_the_ground(true, &[]);
}

/// The lab staircases (rise/run 0.15/0.30 x 12; 0.25/0.22 x 16 so each leg
/// plants twice). Comfortable, up: a toe sinks 5 cm (the lock keeps the
/// heel-strike height while the toe rolls down onto the next tread). Steep:
/// a 1.1 m walk step spans five treads (1.25 m of rise), beyond the leg, so
/// samples are rejected and feet sink or float 25-68 cm; needs shorter steps
/// on steep grades (animation-plan.md slice 5).
#[test]
#[ignore = "stairs: toe sinks 5 cm going up (lock vs heel-strike), steep flight needs shorter steps (animation-plan.md slice 5)"]
fn feet_meet_the_ground_on_stairs() {
    feet_meet_the_ground(false, &[(12, 0.15, 0.30), (16, 0.25, 0.22)]);
}

/// A named walk: world, start, direction, measured x window.
type Walk = (String, World, Vec3, Vec3, (f32, f32));

fn feet_meet_the_ground(ramps: bool, stairs: &[(usize, f32, f32)]) {
    if humanoid_sim().is_none() {
        return;
    }
    let mut walks: Vec<Walk> = Vec::new();
    if ramps {
        walks.push(("flat".to_owned(), World::new(), Vec3::new(-4.0, 0.0, 0.0), Vec3::X, (-1.0, 5.0)));
        for deg in [10.0, 20.0, 30.0] {
            let (w, top) = World::ramp(deg, 8.0);
            walks.push((format!("up {deg}°"), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (1.0, top.x - 1.0)));
            let (w, top) = World::ramp(deg, 8.0);
            walks.push((format!("down {deg}°"), w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, (1.0, top.x - 1.0)));
        }
    }
    for &(n, rise, run) in stairs {
        let (w, top) = World::stairs(n, rise, run);
        walks.push((format!("up {rise}/{run}"), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (0.3, top.x - 0.3)));
        let (w, top) = World::stairs(n, rise, run);
        walks.push((format!("down {rise}/{run}"), w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, (0.3, top.x - 0.3)));
    }
    let mut failures = Vec::new();
    for (name, world, start, dir, window) in &walks {
        let mut sim = humanoid_sim().unwrap();
        let trace = std::env::var("SLOPE_TRACE").is_ok_and(|g| g == *name);
        let r = walk(&mut sim, world, *start, *dir, 6.0, *window, trace);
        let legs: Vec<String> = r
            .soles
            .iter()
            .zip(r.feet.iter().zip(&r.toes))
            .map(|(s, (m, t))| {
                let toe = t.map_or(f32::NAN, |t| t.max_penetration * 100.0);
                format!(
                    "{} st sole pen {:4.1} float {:4.1} (heel pen {:4.1} toe pen {toe:4.1}) slide {:3.1}",
                    s.stances,
                    s.max_penetration * 100.0,
                    s.max_float * 100.0,
                    m.max_penetration * 100.0,
                    m.slide_per_stance() * 100.0
                )
            })
            .collect();
        eprintln!("{name:14} {:3.1} m | {} | rej {} reach {} pelvis {}", r.travel, legs.join(" | "), r.stats.rejected, r.stats.reach_clamped, r.stats.pelvis_clamped);
        for (k, m) in r.soles.iter().enumerate() {
            if m.stances < 1 || m.max_penetration > 0.02 || m.max_float > 0.02 {
                failures.push(format!("{name}: leg {k} {m:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
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

/// Upper-body and arm overlays (the torch arm, the crossbow and spell
/// stance idles over the walk) must not replace the legs' contact curves:
/// a masked override layer used to copy every curve of its clip, so the
/// overlay idle's constant contacts (one foot 0, the other 1) froze the
/// walk's contacts while the body travelled (walkers3: frozen contacts on
/// flat ground).
#[test]
fn overlays_keep_the_walk_contacts() {
    if humanoid_sim().is_none() {
        return;
    }
    let flat = World::new();
    let spell = mw_graphs::build::Stance::Spell.value();
    let crossbow = mw_graphs::build::Stance::Crossbow.value();
    for (name, param, value) in [("torch", p::TORCH, 1.0), ("spell stance", p::STANCE, spell), ("crossbow stance", p::STANCE, crossbow)] {
        let mut sim = humanoid_sim().unwrap();
        sim.set(param, value);
        let r = walk(&mut sim, &flat, Vec3::new(-4.0, 0.0, 0.0), Vec3::X, 5.0, (-1.0, 5.0), false);
        check(name, &r);
    }
}

/// Foot-slide and penetration per gait on flat ground (owner checks SB-5,
/// WK-3; animation.md "Verification"): walk and run forward, back and
/// sideways (`sf_anim::foot_metrics` on the measured stretch). Every leg
/// must keep planting, and planted feet must stay put and on the ground.
/// `-- --nocapture` prints the table (locomotion.md "Foot metrics").
#[test]
fn gaits_plant_their_feet_on_flat_ground() {
    use std::f32::consts::{FRAC_PI_2, PI};
    const RUN: f32 = 3.7;
    if humanoid_sim().is_none() {
        return;
    }
    let flat = World::new();
    let mut failures = Vec::new();
    // Body moves +X (yaw π/2 faces it); the facing offset picks the gait. Every gait is
    // gated: strafes and backpedals play by their planted feet's speed (locomotion.md "Foot metrics").
    for (gait, speed, offset) in [
        ("walk fwd", SPEED, 0.0),
        ("walk back", SPEED, PI),
        ("walk left", SPEED, -FRAC_PI_2),
        ("walk right", SPEED, FRAC_PI_2),
        ("run fwd", RUN, 0.0),
        ("run back", RUN, PI),
        ("run left", RUN, -FRAC_PI_2),
        ("run right", RUN, FRAC_PI_2),
    ] {
        let mut sim = humanoid_sim().unwrap();
        sim.set(p::RUNNING, if speed > SPEED { 1.0 } else { 0.0 });
        let trace = std::env::var("GAIT_TRACE").is_ok_and(|g| g == gait);
        let r = walk_at(&mut sim, &flat, (Vec3::new(-4.0, 0.0, 0.0), Vec3::X * speed, Some(FRAC_PI_2 + offset)), 5.0, (-9.0, 99.0), trace);
        if trace {
            eprintln!("{gait}: playing {:?}, {:?}", sim.playing(), r.stats);
        }
        let state = sim.state(mw_graphs::build::LOCOMOTION).unwrap_or("?").to_owned();
        let legs: Vec<String> = r
            .feet
            .iter()
            .map(|m| {
                format!(
                    "{:2} st {:4.1} cm/st {:4.1} cm/s pen {:4.1} float {:4.1} cm, swing {:3.1} stance {:3.1} s",
                    m.stances,
                    m.slide_per_stance() * 100.0,
                    m.slide_speed() * 100.0,
                    m.max_penetration * 100.0,
                    m.max_float * 100.0,
                    m.longest_swing,
                    m.longest_stance
                )
            })
            .collect();
        // The top clip's root and toe-aware stance speed, and the foot lock's drift.
        let top = sim.playing().first().map(|(n, _)| n.clone()).unwrap_or_default();
        let (root, stance) = sim.graph.clips.iter().position(|c| c.reference == top).map_or((0.0, 0.0), |i| {
            let m = &sim.clips[i].meta;
            (m.average_velocity.length(), m.stance_velocity.length())
        });
        let roll_cm: Vec<f32> = r.roll_slide.iter().zip(&r.roll_episodes).map(|(s, &n)| s * 100.0 / n.max(1) as f32).collect();
        let mean_drift = r.stats.drift_sum / r.stats.drift_count.max(1) as f32;
        eprintln!(
            "{gait:10} [{state}] {:4.1} m | {top} root {root:.2} stance {stance:.2} m/s | worst slide {:4.1} cm/st, lock drift max {:4.1} mean {:4.1} cm, unplanted {} | toe slide after heel-off {:.1}/{:.1} cm per roll ({:?} rolls) | {}",
            r.travel,
            r.feet.iter().map(FootMetrics::slide_per_stance).fold(0.0, f32::max) * 100.0,
            r.stats.drift_max * 100.0,
            mean_drift * 100.0,
            r.stats.unplanted,
            roll_cm[0],
            roll_cm[1],
            r.roll_episodes,
            legs.join(" | ")
        );
        // AN-9: the toe stays put from heel-off until it lifts (cook `roll.*` curves + the foot
        // lock's toe pin); released at heel-off it slid 14-16 cm per step. Strafes skim the toe
        // close to the ground in swing, so only the forward walk is gated (a lead for the rest).
        if gait == "walk fwd" && roll_cm.iter().any(|&c| c > 4.0) {
            failures.push(format!("{gait}: toe slides {roll_cm:?} cm per heel-off"));
        }
        for (k, m) in r.feet.iter().enumerate() {
            if m.stances < 2 || m.stuck(1.5) || m.slide_per_stance() > 0.03 || m.max_penetration > 0.03 {
                failures.push(format!("{gait}: leg {k} {m:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// AN-9: the cooked walk and run clips of every humanoid skeleton carry a
/// `roll.<foot>` curve beside each `contact.<foot>` (heel-off to toe-off, the
/// toe still planted), or the foot lock's toe pin never engages. The humanoid
/// skeletons have no toe bone (only the beast races have `Toe0`): the cook
/// uses the foot's first child (the body-part socket) as the toe.
#[test]
fn walk_clips_carry_heel_roll_curves() {
    for (index, name) in [(0, "male"), (1, "female"), (2, "beast")] {
        let Some(sim) = humanoid_sim_of(index) else { return };
        let mut walkers = 0;
        for clip in sim.clips.iter().filter(|c| c.curves.iter().any(|t| t.name.starts_with("contact."))) {
            let rolls: Vec<&sf_anim::CurveTrack> = clip.curves.iter().filter(|t| t.name.starts_with("roll.")).collect();
            let rolling = rolls.iter().filter(|t| t.values.iter().any(|&v| v > 0.5)).count();
            eprintln!("{name}: {:<16} rolls {} ({} rolling)", clip.name, rolls.len(), rolling);
            if clip.name.starts_with("Walk") || clip.name.starts_with("Run") {
                walkers += 1;
                // The human walk rolls both heels; elsewhere a toe may leave with its heel, so one foot may not.
                let want = if clip.name == "WalkForward" && index < 2 { 2 } else { 1 };
                assert!(rolling >= want, "{name}: {} has {rolling} rolling curves of {}", clip.name, rolls.len());
            }
        }
        assert!(walkers >= 4, "{name}: {walkers} walk/run clips with contact curves");
    }
}

/// AN-3: footstep sounds come from the clips' `SoundGen: Left/Right` text keys
/// (`mw_anim::events::FOOTSTEP`, the graph's leading clip fires them), so they
/// must land on the matching foot's plant at every speed-matched rate. The
/// app's old distance timer (one step per 0.7 m) knew nothing of the feet.
#[test]
fn footstep_keys_land_on_their_foot_plants() {
    use std::f32::consts::FRAC_PI_2;
    if humanoid_sim().is_none() {
        return;
    }
    let flat = World::new();
    let mut failures = Vec::new();
    for (gait, speed) in [("walk slow", 1.4), ("walk", SPEED), ("run", 3.7)] {
        let mut sim = humanoid_sim().unwrap();
        sim.set(p::RUNNING, if speed > SPEED { 1.0 } else { 0.0 });
        let r = walk_at(&mut sim, &flat, (Vec3::new(-4.0, 0.0, 0.0), Vec3::X * speed, Some(FRAC_PI_2)), 5.0, (-9.0, 99.0), false);
        let side = |k: usize| {
            let name = sim.skeleton.name(sim.graph.legs[k].effector().into()).to_ascii_lowercase();
            if name.contains(" l ") { "left" } else { "right" }
        };
        let mut offsets = Vec::new();
        for e in sim.fired(mw_anim::events::FOOTSTEP, 2.5) {
            if e.weight < 0.5 {
                continue;
            }
            let Some(k) = (0..r.plant_times.len()).find(|&k| side(k) == e.payload) else { continue };
            let near = r.plant_times[k].iter().map(|t| e.time - t).min_by(|a, b| a.abs().total_cmp(&b.abs()));
            match near {
                // Vanilla keys sit 50-130 ms after the cooked contact rises (heel strike vs
                // foot flat); a sound may trail its sight by ~125 ms unnoticed (ITU-R BT.1359), lead by far less.
                // (Plants land on 60 Hz frames: 0.15 s is 9 of them, summed in f32.)
                Some(d) if (-0.05..=0.15 + 1e-4).contains(&d) => offsets.push(d),
                _ => failures.push(format!("{gait}: {} step at {:.2} s, nearest plant {near:?}", e.payload, e.time)),
            }
        }
        eprintln!("{gait:9}: {} steps, key - plant {:?} s", offsets.len(), offsets.iter().map(|d| (d * 100.0).round() / 100.0).collect::<Vec<_>>());
        if offsets.len() < 4 {
            failures.push(format!("{gait}: only {} steps on plants", offsets.len()));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
/// AN-16/AN-17: each lab walker at its path speed (1.4 m/s in the world,
/// `1.4 / scale` in its unscaled skeleton space) walks with a human cadence
/// and step: a full cycle of 0.9-1.35 s covering 1.2-1.85 leg lengths (the
/// legacy male walk, made for 2.1 m/s, played its 2.2-leg cycle at 0.64:
/// 1.58 s), and every foot plants once per cycle (the female walk's contact
/// dropped for a frame at its loop seam: two plants per left step).
/// `-- --nocapture` prints the numbers.
#[test]
fn walkers_keep_a_human_stride_and_plant_once_per_step() {
    use std::f32::consts::FRAC_PI_2;
    if humanoid_sim().is_none() {
        return;
    }
    let flat = World::new();
    let mut failures = Vec::new();
    for (who, set, scale) in [("fargoth (male x0.9)", 0, 0.9), ("male", 0, 1.0), ("elone (female)", 1, 1.0), ("ajira (beast)", 2, 1.0)] {
        let mut sim = humanoid_sim_of(set).unwrap();
        let speed = 1.4 / scale;
        let r = walk_at(&mut sim, &flat, (Vec3::new(-4.0, 0.0, 0.0), Vec3::X * speed, Some(FRAC_PI_2)), 6.0, (-9.0, 99.0), false);
        let leg = &sim.graph.legs[0];
        let at = |j: u16| sim.out.model.translation(usize::from(j));
        let length: f32 = leg.joints.windows(2).map(|w| at(w[0]).distance(at(w[1]))).sum();
        let mut ok = true;
        let mut cycles = Vec::new();
        for (k, t) in r.plant_times.iter().enumerate() {
            let gaps: Vec<f32> = t.windows(2).map(|w| w[1] - w[0]).collect();
            let mut sorted = gaps.clone();
            sorted.sort_by(f32::total_cmp);
            let median = sorted.get(sorted.len() / 2).copied().unwrap_or(0.0);
            // Once per cycle: every interval near the cycle (a mid-stance re-plant halves one).
            if gaps.len() < 2 || gaps.iter().any(|&g| (g - median).abs() > 0.25 * median) {
                failures.push(format!("{who} leg {k}: plant intervals {gaps:.2?}"));
                ok = false;
            }
            cycles.push(median);
        }
        let cycle = cycles.iter().sum::<f32>() / cycles.len().max(1) as f32;
        let stride = speed * cycle / length;
        eprintln!(
            "{who:20} v {speed:4.2} | cycle {cycle:4.2} s, stride {:4.2} m = {stride:4.2} legs | lock drift max {:4.1} mean {:4.1} cm | stance extension {:.2?} | foot peak {:.2?} m/s",
            speed * cycle,
            r.stats.drift_max * 100.0,
            r.stats.drift_sum / r.stats.drift_count.max(1) as f32 * 100.0,
            r.extension,
            r.foot_peak,
        );
        if ok && !((0.9..=1.35).contains(&cycle) && (1.2..=1.85).contains(&stride)) {
            failures.push(format!("{who}: cycle {cycle:.2} s, stride {stride:.2} leg lengths"));
        }
        // AN-19: the lock holds each stance within a few cm, and its release adds no whip: the
        // fastest foot stays near the clip's own fastest (body at the clip's stride speed).
        let drift = r.stats.drift_sum / r.stats.drift_count.max(1) as f32;
        let (_, authored, _) = walk_clip_feet(&sim);
        let fastest = r.foot_peak.iter().fold(0.0f32, |m, &p| m.max(p)) / speed;
        eprintln!("{who:20} foot peak {fastest:.2}x the walk speed, authored {authored:.2}x");
        if drift > 0.03 || fastest > authored * 1.15 {
            failures.push(format!("{who}: lock drift mean {:.1} cm, foot peak {fastest:.2}x (authored {authored:.2}x)", drift * 100.0));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// AN-19: in a walk clip's own time, a planted foot must keep pace with the
/// body: with the body moving at the clip's stride speed (what speed
/// matching plays it by), the foot lock's offset from the animation at
/// release is the foot's drift over its stance. The legacy male walk's
/// stance feet moved unevenly against its root and its root carried the
/// hips' ±17° yaw swing (12-19 cm per stance): the lock let go of that much
/// each step, a jolt. Per stance, heel while `contact` is up, then the toe
/// while `roll` is (the pin). Returns the worst drift per stance, the
/// fastest foot over the body's speed as authored (body at the stride
/// speed), and a report.
fn walk_clip_feet(sim: &Sim) -> (f32, f32, String) {
    let clip = sim.clips.iter().find(|c| c.name == "WalkForward").expect("WalkForward");
    let v = clip.meta.stride_velocity().with_y(0.0);
    let (a, b) = clip.loop_range.unwrap_or((0.0, clip.duration));
    let dt = 1.0 / clip.rate;
    let frames = ((b - a) / dt).round() as usize;
    let mut local = sf_anim::LocalPose::new(sim.skeleton.len());
    let mut model = sf_anim::ModelPose::new(sim.skeleton.len());
    let toe_joints = toes(sim);
    let (mut worst, mut peak, mut out) = (0.0f32, 0.0f32, format!("v {:.2} m/s, cycle {frames}f", v.length()));
    for (k, leg) in sim.graph.legs.iter().enumerate() {
        let foot = usize::from(leg.effector());
        let name = sim.skeleton.name(foot);
        let curve = |prefix: &str| clip.curves.iter().position(|c| c.name == format!("{prefix}.{name}"));
        let (contact, roll) = (curve("contact").expect("contact curve"), curve("roll"));
        let toe = toe_joints[k].map(|(t, _)| t);
        // Two cycles from the first stance start, so every stance is whole.
        let (mut pinned, mut last): (Vec<Option<Vec3>>, Option<Vec3>) = (Vec::new(), None);
        for f in 0..2 * frames {
            let t = a + (f % frames) as f32 * dt;
            clip.sample(t, &mut local, None);
            sf_anim::local_to_model(&sim.skeleton, &local, &mut model, &Affine3A::IDENTITY);
            let at = |j: usize| model.translation(j).with_y(0.0) + v * (f as f32 * dt);
            if let Some(p) = last.filter(|_| f % frames != 0) {
                peak = peak.max((at(foot) - p).length() / dt / v.length());
            }
            last = Some(at(foot));
            let heel = clip.curve(contact, t) > 0.5;
            let rolling = roll.is_some_and(|r| clip.curve(r, t) > 0.5);
            pinned.push(if heel { Some(at(foot)) } else if rolling { toe.map(at) } else { None });
        }
        let start = pinned.iter().position(Option::is_none).unwrap_or(0);
        let mut f = start;
        let _ = write!(out, " | {name}:");
        while f < 2 * frames {
            let Some(p0) = pinned[f] else {
                f += 1;
                continue;
            };
            let (mut drift, mut len, mut max, mut base, mut prev) = (0.0, 0, 0.0f32, p0, p0);
            while let Some(&Some(p)) = pinned.get(f) {
                // The pin point switches heel -> toe at heel-off: the toe is held from there.
                if (p - prev).length() > 0.05 {
                    base += p - prev;
                }
                prev = p;
                drift = (p - base).length();
                max = max.max(drift);
                len += 1;
                f += 1;
            }
            if f - len < start + frames {
                let _ = write!(out, " {len}f {:.1}/{:.1} cm", drift * 100.0, max * 100.0);
                worst = worst.max(max);
            }
        }
    }
    (worst, peak, out)
}

/// AN-19 in clip time (see [`walk_clip_feet`]): no stance foot drifts more
/// than 5 cm from its lock (was 12-19 cm on the male and beast walks).
/// `-- --nocapture` prints per stance: frames, drift at release / max.
#[test]
fn walk_clip_stance_feet_keep_pace_with_the_body() {
    let mut failures = Vec::new();
    for (index, who) in [(0, "male"), (1, "female"), (2, "beast")] {
        let Some(sim) = humanoid_sim_of(index) else { return };
        let (worst, peak, report) = walk_clip_feet(&sim);
        eprintln!("{who:6} {report} | foot peak {peak:.2}x");
        if worst > 0.05 {
            failures.push(format!("{who}: a stance foot drifts {:.1} cm from its lock", worst * 100.0));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Grade-aware gait (AN-21): on the lab ramps (10/20/30°) walkers take shorter steps uphill and
/// longer ones downhill at the same speed (the cadence takes the rest), and the trunk leans a
/// little into the slope: forward uphill, slightly back downhill, within its cap and without
/// jolts. Flat ground keeps grade 0 (stride and lean as authored). Feet on the ramps stay gated
/// by `feet_meet_the_ground_on_ramps`.
#[test]
fn walkers_shorten_their_steps_uphill_and_lean_into_the_slope() {
    if humanoid_sim().is_none() {
        return;
    }
    // Step length = speed x the mean interval between plants of one leg / 2.
    let step = |r: &Report| {
        let cycles: Vec<f32> = r.plant_times.iter().flat_map(|t| t.windows(2).map(|w| w[1] - w[0])).collect();
        let cycle = cycles.iter().sum::<f32>() / cycles.len().max(1) as f32;
        r.travel / r.seconds.max(1e-3) * cycle / 2.0
    };
    let mean = |v: &[f32]| v.iter().sum::<f32>() / v.len().max(1) as f32;
    let jolt = |v: &[f32]| v.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0f32, f32::max);
    let mut sim = humanoid_sim().unwrap();
    let flat = walk(&mut sim, &World::new(), Vec3::new(-4.0, 0.0, 0.0), Vec3::X, 6.0, (-1.0, 5.0), false);
    let (flat_step, flat_trunk, flat_jolt) = (step(&flat), mean(&flat.trunk), jolt(&flat.trunk));
    let flat_grade = flat.grades.iter().fold(0.0f32, |m, g| m.max(g.abs()));
    eprintln!("flat      step {flat_step:.3} m trunk {:5.2}° jolt {:.2}° grade {flat_grade:.4}", flat_trunk.to_degrees(), flat_jolt.to_degrees());
    let mut failures = Vec::new();
    if flat_grade > 1e-3 {
        failures.push(format!("flat: grade {flat_grade}"));
    }
    let (mut last_up, mut last_down) = (flat_step, flat_step);
    for deg in [10.0f32, 20.0, 30.0] {
        let (w, top) = World::ramp(deg, 8.0);
        let up = walk(&mut humanoid_sim().unwrap(), &w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, 6.0, (1.0, top.x - 1.0), false);
        let (w, top) = World::ramp(deg, 8.0);
        let down = walk(&mut humanoid_sim().unwrap(), &w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, 6.0, (1.0, top.x - 1.0), false);
        let tan = deg.to_radians().tan();
        for (name, r, sign) in [("up", &up, 1.0f32), ("down", &down, -1.0)] {
            let (s, lean, j) = (step(r), mean(&r.trunk) - flat_trunk, jolt(&r.trunk));
            let g = mean(&r.grades);
            eprintln!("{name:4} {deg:2}° step {s:.3} m ({:+4.0}%) lean {:+5.2}° jolt {:.2}° grade {g:+.3} (tan {:+.3})", (s / flat_step - 1.0) * 100.0, lean.to_degrees(), j.to_degrees(), sign * tan);
            if (g - sign * tan).abs() > 0.25 * tan + 0.02 {
                failures.push(format!("{name} {deg}°: grade {g} vs {}", sign * tan));
            }
            // Owner 2026-10-22: "stronger", then "more pronounced" uphill (up 0.4, cap 12°): forward up (at least 6° at 30°), back down (at least 2°).
            let least = if deg >= 30.0 { if sign > 0.0 { 6.0 } else { 2.0 } } else { 0.0 };
            if lean.abs() > 13f32.to_radians() || lean * sign < f32::to_radians(least) {
                failures.push(format!("{name} {deg}°: lean {:.2}° (forward up, back down, within 13°)", lean.to_degrees()));
            }
            if j > flat_jolt + 0.5f32.to_radians() {
                failures.push(format!("{name} {deg}°: trunk jolt {:.2}° vs flat {:.2}°", j.to_degrees(), flat_jolt.to_degrees()));
            }
        }
        let (su, sd) = (step(&up), step(&down));
        // Shorter uphill, longer downhill, by a sane margin at 30° (and monotonic in the grade).
        if su >= last_up || sd <= last_down * 0.99 {
            failures.push(format!("{deg}°: up {su:.3} (prev {last_up:.3}) down {sd:.3} (prev {last_down:.3}) not monotonic"));
        }
        (last_up, last_down) = (su, sd);
        if deg >= 30.0 && (su > flat_step * 0.88 || sd < flat_step * 1.05) {
            failures.push(format!("30°: up {su:.3} down {sd:.3} vs flat {flat_step:.3}: under the margin"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
