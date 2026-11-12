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
    let mut v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    if let Some(deg) = std::env::var("IK_UNPLANT_ANGLE").ok().and_then(|s| s.parse::<f32>().ok()) {
        // Measurement sweep (P-IK5): the shipped graph with another turn unplant angle (degrees;
        // 1000 = off, the drift limit alone).
        let mut asset = sf_anim_graph::asset::parse_graph(mw_graphs::builtin(Template::Humanoid)).unwrap();
        for op in &mut asset.procedural {
            if let sf_anim_graph::asset::OpSpec::FootPlacement { guards, .. } = op {
                guards.unplant_angle = deg.to_radians();
            }
        }
        v.graph = set.compile(&format!("{}:{}", set.name, Template::Humanoid.path()), &asset).unwrap();
    }
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
    /// Per measured frame: the heaviest clip and its playback rate (clip seconds
    /// per second, from its clip time; wraps skipped).
    rates: Vec<(String, f32)>,
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
    let mut prev_clip: Option<(String, f32)> = None;
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
        if let Some((top, _)) = sim.playing().into_iter().next()
            && let Some(t) = sim.clip_time(&top)
        {
            if let Some((c, pt)) = &prev_clip
                && *c == top
                && t > *pt
            {
                r.rates.push((top.clone(), (t - pt) / DT));
            }
            prev_clip = Some((top, t));
        }
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
            // Sanity (AN-21): forward up, back down at 30° (owner 2026-11-05 tune: about +16/-1.5°, `slope_lean_table`), within the op's 26° cap.
            let least = if deg >= 30.0 { if sign > 0.0 { 6.0 } else { 0.5 } } else { 0.0 };
            if lean.abs() > 26f32.to_radians() || lean * sign < f32::to_radians(least) {
                failures.push(format!("{name} {deg}°: lean {:.2}° (forward up, back down, within 26°)", lean.to_degrees()));
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

/// Slope lean table (AN-21, owner 2026-11-05: "not enough forward when going up", down
/// "maybe sliiiightly too much"): the trunk pitch (`Bip01 Spine1` to `Bip01 Head`, forward
/// positive, degrees) on the lab ramps at 0/10/20/30° up and down, walk (2.1 m/s) and run
/// (4.5 m/s), relative to the gait's own flat pitch and absolute (from vertical). Rows:
/// "before" (the 2026-10-22 values: chest 0.7 + neck 0.3, up 0.4 / down 0.12 / max 0.21),
/// "shipped" (the builder's) and candidates. Sanity only: finite, within the op's cap.
#[test]
#[ignore = "measurement: slope lean table"]
fn slope_lean_table() {
    use sf_anim_graph::Op;
    use sf_anim_graph::asset::GradeLean;
    type Override = Option<(Vec<(u16, f32)>, GradeLean)>;
    let Some(probe) = humanoid_sim() else { return };
    let bone = |n: &str| u16::try_from(probe.skeleton.find(n).unwrap()).unwrap();
    let (spine1, neck) = (bone("Bip01 Spine1"), bone("Bip01 Neck"));
    let old_joints = vec![(spine1, 0.7), (neck, 0.3)];
    // Candidates (owner retune): `(vec![(spine1, 0.45), (bone("Bip01 Spine2"), 0.4), (neck, 0.15)], g(up, down, max))`.
    let g = |up, down, max| GradeLean { up, down, max };
    let variants: Vec<(&str, Override)> = vec![
        ("before", Some((old_joints, g(0.4, 0.12, 0.21)))),
        ("shipped", None),
    ];
    let mean = |v: &[f32]| v.iter().sum::<f32>() / v.len().max(1) as f32;
    for (label, data) in variants {
        let mut cap = 0.0f32;
        let make = || {
            let mut sim = humanoid_sim().unwrap();
            for o in &mut sim.graph.ops {
                if let (Op::Lean { joints, grade, .. }, Some((j, gl))) = (&mut o.op, &data) {
                    joints.clone_from(j);
                    *grade = *gl;
                }
            }
            sim
        };
        for o in &make().graph.ops {
            if let Op::Lean { grade, .. } = &o.op {
                cap = grade.max;
                eprintln!("--- {label}: up {} down {} max {} ({:.1}°)", grade.up, grade.down, grade.max, grade.max.to_degrees());
            }
        }
        for (gait, speed, running) in [("walk", SPEED, false), ("run", 4.5f32, true)] {
            let run = |world: &World, start: Vec3, dir: Vec3, window: (f32, f32)| {
                let mut sim = make();
                sim.set(p::RUNNING, if running { 1.0 } else { 0.0 });
                let r = walk_at(&mut sim, world, (start, dir * speed, None), 1.0 + 14.0 / speed, window, false);
                assert!(!r.trunk.is_empty() && r.trunk.iter().all(|t| t.is_finite()), "{label} {gait}: trunk samples");
                mean(&r.trunk)
            };
            let flat = run(&World::new(), Vec3::new(-6.0, 0.0, 0.0), Vec3::X, (-99.0, 999.0));
            let mut line = format!("{gait:4} flat {:+5.1}° |", flat.to_degrees());
            for deg in [10.0f32, 20.0, 30.0] {
                let (w, top) = World::ramp(deg, 12.0);
                let up = run(&w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (1.0, top.x - 1.0));
                let down = run(&w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, (1.0, top.x - 1.0));
                for (name, v) in [("up", up), ("dn", down)] {
                    let rel = v - flat;
                    // Sanity: the op's cap plus the gait's own sway.
                    assert!(rel.abs() < cap + 3f32.to_radians(), "{label} {gait} {name} {deg}°: {:.1}° over the cap", rel.to_degrees());
                    write!(line, " {name}{deg:2.0} {:+5.1} ({:+5.1})", rel.to_degrees(), v.to_degrees()).unwrap();
                }
            }
            eprintln!("{line}");
        }
    }
}

/// Pose audit S3 / feed audit F4: the gait bands. The player moves at 1.6 (walk),
/// 4.5 (run) and 7.0 m/s (sprint, the run cycle) against cycles authored at about
/// 2.2 and 3.2 m/s; NPCs walk at about 2.2. Per gait and op data: the state, top
/// clip, playback rate, cadence, the stride ratio left to `StrideWarp` (after its
/// clamp), its largest pelvis drop, and the feet's slide and lock drift. Data rows:
/// "shipped" (the builder's) and candidate overrides of the `StrideWarp` max,
/// the `SpeedMatch` rate share and rate max.
#[test]
#[ignore = "measurement: gait bands"]
fn gait_bands() {
    use std::f32::consts::FRAC_PI_2;
    use sf_anim_graph::Op;
    if humanoid_sim().is_none() {
        return;
    }
    let flat = World::new();
    let variants = [
        ("shipped", None),
        ("old 1.6/0.3/2.2", Some((1.6, 0.3, 2.2))),
        ("1.15/0.5/2.2", Some((1.15, 0.5, 2.2))),
        ("1.15/0.3/2.2", Some((1.15, 0.3, 2.2))),
        ("1.25/0.5/2.2", Some((1.25, 0.5, 2.2))),
    ];
    for (label, data) in variants {
        eprintln!("--- {label} (StrideWarp max / rate_share / rate max)");
        // Body moves +X; the facing offset picks the gait (strafes: the run clips plant at a quarter of the speed).
        for (gait, speed, running, offset) in [
            ("player walk", 1.6, false, 0.0),
            ("npc walk", 2.2, false, 0.0),
            ("player run", 4.5, true, 0.0),
            ("player sprint", 7.0, true, 0.0),
            ("npc run left", 3.7, true, -FRAC_PI_2),
            ("npc run right", 3.7, true, FRAC_PI_2),
            ("player run lt", 4.5, true, -FRAC_PI_2),
        ] {
            let mut sim = humanoid_sim().unwrap();
            let mut warp = (0.0, 0.0);
            for o in &mut sim.graph.ops {
                match &mut o.op {
                    Op::StrideWarp { min, max, .. } => {
                        if let Some((m, _, _)) = data {
                            *max = m;
                        }
                        warp = (*min, *max);
                    }
                    Op::SpeedMatch { max, rate_share, .. } => {
                        if let Some((_, s, r)) = data {
                            *rate_share = s;
                            *max = r;
                        }
                    }
                    _ => {}
                }
            }
            sim.set(p::RUNNING, if running { 1.0 } else { 0.0 });
            let r = walk_at(&mut sim, &flat, (Vec3::new(-4.0, 0.0, 0.0), Vec3::X * speed, Some(FRAC_PI_2 + offset)), 5.0, (-99.0, 999.0), false);
            let state = sim.state(mw_graphs::build::LOCOMOTION).unwrap_or("?").to_owned();
            let mut rates: Vec<f32> = r.rates.iter().map(|(_, x)| *x).collect();
            rates.sort_by(f32::total_cmp);
            let rate = rates.get(rates.len() / 2).copied().unwrap_or(0.0);
            let top = r.rates.last().map(|(c, _)| c.clone()).unwrap_or_default();
            let natural = sim.graph.clips.iter().position(|c| c.reference == top).map_or(0.0, |i| {
                let v = sim.clips[i].meta.stride_velocity();
                Vec2::new(v.x, v.z).length()
            });
            let actual = r.travel / r.seconds.max(1e-3);
            let ratio = actual / (natural * rate).max(1e-3);
            let cadence = r.plants.iter().sum::<u32>() as f32 / r.seconds.max(1e-3);
            let slide = r.feet.iter().map(FootMetrics::slide_per_stance).fold(0.0, f32::max) * 100.0;
            eprintln!(
                "{gait:13} {actual:4.2} m/s [{state}] {top} @ {natural:4.2} m/s | rate {rate:4.2} | cadence {cadence:4.2} steps/s | stride {ratio:4.2} (warp {:4.2}) | pelvis drop {:5.1} mm, clamped {} | slide {slide:4.1} cm/st, lock drift max {:4.1} cm, unplanted {}",
                ratio.clamp(warp.0, warp.1),
                r.stats.stride_drop_max * 1000.0,
                r.stats.pelvis_clamped,
                r.stats.drift_max * 100.0,
                r.stats.unplanted,
            );
        }
    }
}

// --- IK/feel diagnosis 2026-11-11 (superfargoth docs/research/ik-feel-diagnosis-2026-11-11.md) ----------

/// One frame of [`drive`]: world pelvis (`Bip01 Pelvis`) and, per leg, the sole's gap to the
/// ground under it (m, positive floats) with the leg's contact curve, and the foot's position.
struct Sample {
    pelvis: Vec3,
    soles: Vec<(f32, f32)>,
    feet: Vec<Vec3>,
    /// The graph's filtered ground grade (`out.grade`).
    grade: f32,
}

/// Which character's feed [`drive`] stands in for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Feed {
    /// The walkers' (`sf_controller_bevy` `feed_animators`): ground probes, measured speed
    /// half-life 0.05 s.
    Npc,
    /// The player's before P-IK1 as the diagnosis measured it: no probes (half-life 0.05 s).
    PlayerBefore,
    /// The player's since P-IK1 (`mw_player_bevy::avatar::feed`, the walkers' `FootProbes` at
    /// the drawn root, which is the tick's root at 60 fps): probes, the player's measured speed
    /// half-life `feel::MEASURE_HALFLIFE` (0.025 s).
    Player,
}

impl Feed {
    fn probes(self) -> bool {
        self != Self::PlayerBefore
    }

    fn halflife(self) -> f32 {
        if self == Self::Player { 0.025 } else { 0.05 }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Npc => "npc",
            Self::PlayerBefore => "player before",
            Self::Player => "player",
        }
    }
}

/// Walks the humanoid sim like `walk_at` (NPC mover, measured feed, ground probes per `feed`),
/// then holds `paused` updates at dt = 0 with the probes refreshed each update (what
/// `sf_controller_bevy`'s feed and `sf_anim_bevy::evaluate` do while `Time<Virtual>` is paused).
/// Returns the walking frames (after the first second) and the paused frames.
fn drive(sim: &mut Sim, world: &World, start: Vec3, velocity: Vec3, seconds: f32, feed: Feed, paused: usize) -> (Vec<Sample>, Vec<Sample>) {
    let probes = feed.probes();
    let dir = velocity.normalize();
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(start), dir.x.atan2(dir.z));
    let params = MoverParams::default();
    let mut stepper = KinematicMover::new();
    let intent = LocomotionIntent { desired_velocity: velocity, ..LocomotionIntent::IDLE };
    let legs: Vec<(usize, Option<usize>)> = sim.graph.legs.iter().map(|l| (usize::from(l.effector()), l.contact.map(usize::from))).collect();
    let sole = soles(sim);
    let pelvis = sim.skeleton.find("Bip01 Pelvis").expect("pelvis");
    let mut measured = Vec3::ZERO;
    let (mut walking, mut held) = (Vec::new(), Vec::new());
    let frames = (seconds / DT).round() as usize;
    for frame in 0..frames + 60 + paused {
        let pause = frame >= frames + 60;
        if !pause {
            let it = if frame >= 60 { intent } else { LocomotionIntent::IDLE };
            stepper.step(MoveInput { loco: &mut loco, intent: it, params: &params, root: RootMotion::default() }, world, DT);
            let moved = loco.moved;
            let raw = (Quat::from_rotation_y(-moved.yaw) * moved.translation / DT).with_y(0.0);
            measured = raw.lerp(measured, 0.5f32.powf(DT / feed.halflife()));
            let flat = Vec2::new(-measured.x, measured.z);
            sim.set(p::MOVING, if flat.length() > 0.5 { 1.0 } else { 0.0 });
            sim.set_vec2(p::MOVE, if flat.length() > 1e-3 { flat.normalize() } else { Vec2::Y });
            sim.velocity = measured;
            sim.moved = moved;
        }
        let f = loco.frame();
        let queries: Vec<_> = f.foot_queries(&sim.out.probe_origins, 0.5, 0.75).collect();
        let mut hits = vec![None; queries.len()];
        world.probe(&queries, &mut hits);
        for (slot, hit) in sim.ground.iter_mut().zip(&hits) {
            *slot = if probes {
                hit.as_ref().map(|h| {
                    let (point, normal) = f.ground_sample(h);
                    GroundSample { point, normal }
                })
            } else {
                None
            };
        }
        if pause {
            // One update at dt = 0 (`Sim::run` rounds the step count up to 1).
            sim.run(0.0, 0.0);
        } else {
            sim.run(DT, DT);
        }
        if frame < 120 {
            continue;
        }
        let w = |j: usize| WorldPos::ORIGIN.delta_f32(f.to_world(sim.out.model.translation(j)));
        let s = Sample {
            pelvis: w(pelvis),
            soles: legs
                .iter()
                .enumerate()
                .map(|(k, &(j, c))| {
                    let at = w(j);
                    (ground_at(world, at).map_or(f32::NAN, |g| at.y - g - sole[k]), c.map_or(0.0, |c| sim.out.curves[c]))
                })
                .collect(),
            feet: legs.iter().map(|&(j, _)| w(j)).collect(),
            grade: sim.out.grade,
        };
        if pause {
            held.push(s);
        } else {
            walking.push(s);
        }
    }
    (walking, held)
}

/// The player's foot feed: the shipped humanoid graph on the lab ramps with the walkers' probes
/// ("npc"), with none ("player before": `mw_player_bevy::avatar::feed` set `drive.ground` to
/// `None` every frame until P-IK1) and with the player's feed since P-IK1 ("player": the same
/// probes, the player's speed half-life; the real path's data is asserted by `mw_player_bevy`'s
/// `the_player_probes_the_ground_under_its_feet`). Per walk: planted soles' worst sink and float (cm, contact > 0.5), the
/// planted feet's slide (cm per second of stance), the filtered grade the graph reads
/// (`out.grade`, drives `Lean` and `StrideWarp`), and the pelvis' vertical jerk (RMS of the
/// second difference per frame, mm). Sanity only: finite.
#[test]
#[ignore = "measurement: player foot feed vs walkers"]
fn player_without_foot_probes() {
    if humanoid_sim().is_none() {
        return;
    }
    let mut walks: Vec<Walk> = vec![("flat".into(), World::new(), Vec3::new(-4.0, 0.0, 0.0), Vec3::X, (-99.0, 99.0))];
    for deg in [10.0, 20.0, 30.0] {
        let (w, top) = World::ramp(deg, 8.0);
        walks.push((format!("up {deg}°"), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (1.0, top.x - 1.0)));
        let (w, top) = World::ramp(deg, 8.0);
        walks.push((format!("down {deg}°"), w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, (1.0, top.x - 1.0)));
    }
    let (w, top) = World::stairs(12, 0.15, 0.30);
    walks.push(("stairs up 0.15".into(), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (0.5, top.x - 0.5)));
    for (speed, gait) in [(1.6f32, "walk 1.6"), (4.5, "run 4.5")] {
        eprintln!("--- {gait} m/s");
        for (name, world, start, dir, window) in &walks {
            for feed in [Feed::Npc, Feed::PlayerBefore, Feed::Player] {
                let mut sim = humanoid_sim().unwrap();
                sim.set(p::RUNNING, if speed > 3.0 { 1.0 } else { 0.0 });
                let secs = (12.0 / speed).max(3.0);
                let (frames, _) = drive(&mut sim, world, *start, *dir * speed, secs, feed, 0);
                let on: Vec<&Sample> = frames.iter().filter(|s| s.pelvis.x >= window.0 && s.pelvis.x <= window.1).collect();
                let (mut sink, mut float, mut slide, mut stance) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
                let grade = on.iter().map(|s| s.grade).sum::<f32>() / on.len().max(1) as f32;
                for pair in on.windows(2) {
                    for k in 0..pair[1].soles.len() {
                        let (gap, c) = pair[1].soles[k];
                        if c > 0.5 && gap.is_finite() {
                            sink = sink.max(-gap);
                            float = float.max(gap);
                            stance += DT;
                            slide += Vec2::new(pair[1].feet[k].x - pair[0].feet[k].x, pair[1].feet[k].z - pair[0].feet[k].z).length();
                        }
                    }
                }
                let ys: Vec<f32> = on.iter().map(|s| s.pelvis.y).collect();
                let jerk = (ys.windows(3).map(|w| (w[2] - 2.0 * w[1] + w[0]).powi(2)).sum::<f32>() / ys.len().saturating_sub(2).max(1) as f32).sqrt();
                eprintln!(
                    "{name:15} {:13} | {:3} frames | planted sole sink {:5.1} float {:5.1} cm | slide {:5.2} cm/s stance | mean grade {:5.2} | pelvis d2y rms {:6.3} mm",
                    feed.label(),
                    on.len(),
                    sink * 100.0,
                    float * 100.0,
                    slide / stance.max(1e-3) * 100.0,
                    grade,
                    jerk * 1000.0
                );
                assert!(sink.is_finite() && float.is_finite() && jerk.is_finite());
            }
        }
    }
}

/// ROADMAP 1.18 (IK runs while paused): after a walk (20° ramp, stairs, flat), the graph keeps
/// updating at dt = 0 with fresh probes (no pause gate on `AnimSystems`). Prints the largest
/// pelvis and foot motion (mm) from the last walking frame to the first paused one, and over 60
/// paused updates from the first paused frame. Sanity only: finite.
#[test]
#[ignore = "measurement: paused updates"]
fn paused_updates_move_the_pose() {
    if humanoid_sim().is_none() {
        return;
    }
    let (ramp, _) = World::ramp(20.0, 8.0);
    let (stairs, _) = World::stairs(12, 0.15, 0.30);
    let flat = World::new();
    for (name, world, secs) in [("ramp 20° mid-walk", &ramp, 2.4), ("stairs mid-walk", &stairs, 2.0), ("flat", &flat, 2.0)] {
        let mut sim = humanoid_sim().unwrap();
        let (walking, held) = drive(&mut sim, world, Vec3::new(-3.0, 0.0, 0.0), Vec3::X * SPEED, secs, Feed::Npc, 60);
        let first = &held[0];
        let last = walking.last().unwrap();
        let dist = |a: &Sample, b: &Sample| {
            let feet = a.feet.iter().zip(&b.feet).map(|(x, y)| x.distance(*y)).fold(0.0, f32::max);
            (a.pelvis.distance(b.pelvis), feet)
        };
        let (enter_p, enter_f) = dist(last, first);
        let (mut drift_p, mut drift_f) = (0.0f32, 0.0f32);
        let mut moving = 0;
        let mut steps = Vec::new();
        let mut prev = last;
        for s in &held {
            let (dp, df) = dist(s, first);
            drift_p = drift_p.max(dp);
            drift_f = drift_f.max(df);
            let (_, step) = dist(s, prev);
            moving += usize::from(step > 1e-3);
            if steps.len() < 8 {
                steps.push(format!("{:.0}", step * 1000.0));
            }
            prev = s;
        }
        let heights: Vec<String> = held.iter().take(6).map(|s| s.feet.iter().map(|f| format!("{:.3}", f.y)).collect::<Vec<_>>().join("/")).collect();
        eprintln!(
            "{name:18} pause entry: pelvis {:6.2} mm, feet {:6.2} mm | over 60 paused updates: pelvis {:6.3} mm, feet {:6.3} mm | {moving} updates moved a foot > 1 mm, first steps [{}] mm, foot y {}",
            enter_p * 1000.0,
            enter_f * 1000.0,
            drift_p * 1000.0,
            drift_f * 1000.0,
            steps.join(" "),
            heights.join(" ")
        );
        assert!(drift_p.is_finite() && drift_f.is_finite());
    }
}

// --- P-IK1b (owner PL-7 failed 2026-11-11): the player's kinematics vs the walkers' -----------

/// The player's mover (`mw_player_bevy::feel::mover_params`, copied: velocity half-life
/// 0.07 s, brake 0.05, facing 0.1, turn walk 720 / run 1080 / sprint 480 / sneak 540 deg/s,
/// gravity 18), or the walkers' `MoverParams::default()`.
fn mover_params(player: bool) -> MoverParams {
    if !player {
        return MoverParams::default();
    }
    let turn = sf_controller::TurnRates { walk: 720f32.to_radians(), run: 1080f32.to_radians(), sprint: 480f32.to_radians(), sneak: 540f32.to_radians() };
    let springs = sf_controller::SpringParams { halflife_velocity: 0.07, halflife_facing: 0.1, halflife_brake: Some(0.05), turn: Some(turn) };
    MoverParams { springs, gravity: 18.0, ..MoverParams::default() }
}

/// Per frame of [`ik_walk`] (after the first second).
#[derive(Default)]
struct IkFrame {
    root: Vec3,
    /// Worst |sample height - ground under the sample| this frame, m.
    sample_err: f32,
    grade: f32,
    /// Trunk pitch (`Bip01 Spine1` to `Bip01 Head`, forward positive, rad) minus the twin's.
    lean: f32,
    /// Head pitch (`Bip01 Head`'s +Y axis, forward positive, rad) minus the twin's.
    head: f32,
    /// Pelvis height minus the twin's (no IK), m.
    pelvis: f32,
    /// Per leg: foot (effector) minus the twin's animated foot, character space (x lateral, z fore-aft), m.
    foot: Vec<Vec3>,
    /// Knee spread (`|x_l - x_r|`) minus the twin's, m.
    knees: f32,
    stride: f32,
    drift: f32,
    unplanted: u32,
    unplanted_turn: u32,
    weight: f32,
    /// Seconds since the intent started (L1).
    t: f32,
    /// Locomotion state (L1).
    state: String,
    /// Per leg: the twin's animated foot, character space (L1).
    anim_feet: Vec<Vec3>,
    /// Per leg: the placed foot in the world (L1).
    world_feet: Vec<Vec3>,
    /// Pelvis height, character space: placed and the twin's (L1).
    pelvis_abs: f32,
    twin_pelvis_abs: f32,
}

/// Walks the humanoid graph with the player's (`player`) or the walkers' mover and feed on
/// `world` at `gait`, with ground probes, next to a **twin** graph fed the same motion and no
/// ground (the animated pose: no foot placement, no lean, no stride-warp grade term).
/// `turn`: after that many seconds the intent velocity becomes the second value.
#[allow(clippy::many_single_char_names, reason = "pose math")]
fn ik_walk(world: &World, start: Vec3, velocity: Vec3, gait: sf_controller::Gait, seconds: f32, player: bool, turn: Option<(f32, Vec3)>) -> Vec<IkFrame> {
    let mut sim = humanoid_sim().unwrap();
    let mut twin = humanoid_sim().unwrap();
    let running = velocity.length() > 3.0;
    for s in [&mut sim, &mut twin] {
        s.set(p::RUNNING, if running { 1.0 } else { 0.0 });
    }
    let dir = velocity.normalize();
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(start), dir.x.atan2(dir.z));
    let params = mover_params(player);
    let mut stepper = KinematicMover::new();
    let halflife = if player { 0.025 } else { 0.05 };
    let bone = |n: &str| sim.skeleton.find(n).unwrap_or_else(|| panic!("{n}"));
    let (spine1, head, pelvis) = (bone("Bip01 Spine1"), bone("Bip01 Head"), bone("Bip01 Pelvis"));
    let legs: Vec<(usize, usize)> = sim.graph.legs.iter().map(|l| (usize::from(l.effector()), usize::from(l.joints[1]))).collect();
    let stride_grade = sim.graph.ops.iter().find_map(|o| match o.op {
        sf_anim_graph::Op::StrideWarp { grade, .. } => Some(grade),
        _ => None,
    });
    let mut measured = Vec3::ZERO;
    let mut out = Vec::new();
    let frames = (seconds / DT).round() as usize;
    for frame in 0..frames + 60 {
        let t = frame as f32 * DT - 1.0;
        let v = match turn {
            Some((at, to)) if t >= at => to,
            _ => velocity,
        };
        let it = if frame >= 60 { LocomotionIntent { desired_velocity: v, gait, ..LocomotionIntent::IDLE } } else { LocomotionIntent::IDLE };
        stepper.step(MoveInput { loco: &mut loco, intent: it, params: &params, root: RootMotion::default() }, world, DT);
        let moved = loco.moved;
        let raw = (Quat::from_rotation_y(-moved.yaw) * moved.translation / DT).with_y(0.0);
        measured = raw.lerp(measured, 0.5f32.powf(DT / halflife));
        let flat = Vec2::new(-measured.x, measured.z);
        for s in [&mut sim, &mut twin] {
            s.set(p::MOVING, if flat.length() > 0.5 { 1.0 } else { 0.0 });
            s.set_vec2(p::MOVE, if flat.length() > 1e-3 { flat.normalize() } else { Vec2::Y });
            s.velocity = measured;
            s.moved = moved;
            s.turn_rate = moved.yaw / DT;
        }
        let f = loco.frame();
        let queries: Vec<_> = f.foot_queries(&sim.out.probe_origins, 0.5, 0.75).collect();
        let mut hits = vec![None; queries.len()];
        world.probe(&queries, &mut hits);
        let mut sample_err = 0.0f32;
        for (slot, hit) in sim.ground.iter_mut().zip(&hits) {
            *slot = hit.as_ref().map(|h| {
                let w = WorldPos::ORIGIN.delta_f32(h.point);
                if let Some(g) = ground_at(world, w) {
                    sample_err = sample_err.max((w.y - g).abs());
                }
                let (point, normal) = f.ground_sample(h);
                GroundSample { point, normal }
            });
        }
        sim.run(DT, DT);
        twin.run(DT, DT);
        if frame < 120 {
            continue;
        }
        let (m, n) = (&sim.out.model, &twin.out.model);
        let pitch = |a: Vec3, b: Vec3| {
            let d = b - a;
            d.z.atan2(d.y)
        };
        let fwd = |q: Quat| {
            let d = q * Vec3::Y;
            d.z.atan2(d.y)
        };
        let foot: Vec<Vec3> = legs.iter().map(|&(e, _)| m.translation(e) - n.translation(e)).collect();
        let kx_m = (m.translation(legs[0].1).x - m.translation(legs[1].1).x).abs();
        let kx_n = (n.translation(legs[0].1).x - n.translation(legs[1].1).x).abs();
        out.push(IkFrame {
            root: WorldPos::ORIGIN.delta_f32(f.position),
            sample_err,
            grade: sim.out.grade,
            lean: pitch(m.translation(spine1), m.translation(head)) - pitch(n.translation(spine1), n.translation(head)),
            head: fwd(m.rotation(head)) - fwd(n.rotation(head)),
            pelvis: m.translation(pelvis).y - n.translation(pelvis).y,
            foot,
            knees: kx_m - kx_n,
            stride: stride_grade.map_or(1.0, |g| g.scale(sim.out.grade)),
            drift: sim.out.feet.drift_max,
            unplanted: sim.out.feet.unplanted,
            unplanted_turn: sim.out.feet.unplanted_turn,
            weight: sim.out.feet.weight_max,
            t,
            state: sim.state(mw_graphs::build::LOCOMOTION).unwrap_or("?").to_owned(),
            anim_feet: legs.iter().map(|&(e, _)| n.translation(e)).collect(),
            world_feet: legs.iter().map(|&(e, _)| WorldPos::ORIGIN.delta_f32(f.to_world(m.translation(e)))).collect(),
            pelvis_abs: m.translation(pelvis).y,
            twin_pelvis_abs: n.translation(pelvis).y,
        });
    }
    out
}

fn ik_row(label: &str, frames: &[&IkFrame]) -> String {
    let n = frames.len().max(1) as f32;
    let max = |f: &dyn Fn(&IkFrame) -> f32| frames.iter().map(|x| f(x)).fold(0.0f32, f32::max);
    let mean = |f: &dyn Fn(&IkFrame) -> f32| frames.iter().map(|x| f(x)).sum::<f32>() / n;
    let lat = |x: &IkFrame| x.foot.iter().map(|d| d.x.abs()).fold(0.0f32, f32::max);
    let fa = |x: &IkFrame| x.foot.iter().map(|d| d.z.abs()).fold(0.0f32, f32::max);
    let gmax = frames.iter().map(|x| x.grade).fold(f32::NEG_INFINITY, f32::max);
    let gmin = frames.iter().map(|x| x.grade).fold(f32::INFINITY, f32::min);
    let peak = frames.iter().map(|x| x.lean).fold(0.0f32, |a, b| if b.abs() > a.abs() { b } else { a });
    format!(
        "{label:22} {:4} fr | smp err {:4.1} cm | grade {:+5.2} [{:+5.2},{:+5.2}] | lean {:+5.1}° peak {:+5.1}° head {:+5.1}° | pelvis {:+5.1} cm | foot lat max {:4.1} mean {:4.1} cm, fore-aft max {:5.1} | knees {:+5.1} cm | stride x{:4.2} | drift max {:5.1} cm, unplants {:3} (turn {}) | w {:.2}",
        frames.len(),
        max(&|x| x.sample_err) * 100.0,
        mean(&|x| x.grade),
        gmin,
        gmax,
        mean(&|x| x.lean).to_degrees(),
        peak.to_degrees(),
        mean(&|x| x.head).to_degrees(),
        mean(&|x| x.pelvis) * 100.0,
        max(&lat) * 100.0,
        mean(&lat) * 100.0,
        max(&fa) * 100.0,
        mean(&|x| x.knees) * 100.0,
        mean(&|x| x.stride),
        max(&|x| x.drift) * 100.0,
        frames.iter().map(|x| x.unplanted).sum::<u32>(),
        frames.iter().map(|x| x.unplanted_turn).sum::<u32>(),
        max(&|x| x.weight),
    )
}

/// P-IK1b (owner PL-7 failed): the player's mover and feed (player springs and turn rates,
/// measured half-life 0.025 s) next to the walkers' (default mover, 0.05 s), the shipped
/// humanoid graph with ground probes, at walk 1.6, run 4.5 and sprint 7 m/s on the flat,
/// 10/20/30° ramps up and down and 0.15/0.30 stairs, plus sharp turns (90°, 180°) on the 20°
/// ramp. Each value is against a twin graph fed the same motion without ground (the animated
/// pose). Sanity only: finite.
#[test]
#[ignore = "measurement: player IK vs walkers (P-IK1b)"]
fn player_ik_vs_walkers() {
    use sf_controller::Gait;
    if humanoid_sim().is_none() {
        return;
    }
    let mut walks: Vec<Walk> = vec![("flat".into(), World::new(), Vec3::new(-4.0, 0.0, 0.0), Vec3::X, (-99.0, 99.0))];
    for deg in [10.0, 20.0, 30.0, 40.0] {
        let (w, top) = World::ramp(deg, 12.0);
        walks.push((format!("up {deg}°"), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (1.0, top.x - 1.0)));
        let (w, top) = World::ramp(deg, 12.0);
        walks.push((format!("down {deg}°"), w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, (1.0, top.x - 1.0)));
    }
    let (w, top) = World::stairs(16, 0.15, 0.30);
    walks.push(("stairs up".into(), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (0.5, top.x - 0.5)));
    let (w, top) = World::stairs(16, 0.15, 0.30);
    walks.push(("stairs down".into(), w, Vec3::new(top.x + 3.0, top.y, 0.0), -Vec3::X, (0.5, top.x - 0.5)));
    let (w, top) = World::stairs(8, 0.25, 0.22);
    walks.push(("steep stairs up".into(), w, Vec3::new(-3.0, 0.0, 0.0), Vec3::X, (0.3, top.x - 0.3)));
    let check = |frames: &[IkFrame]| {
        assert!(frames.iter().all(|f| f.lean.is_finite() && f.pelvis.is_finite() && f.foot.iter().all(|d| d.is_finite())), "finite");
    };
    for (speed, gait, label) in [(1.6f32, Gait::Walk, "walk 1.6"), (4.5, Gait::Run, "run 4.5"), (7.0, Gait::Sprint, "sprint 7")] {
        eprintln!("--- {label} m/s");
        for (name, world, start, dir, window) in &walks {
            for player in [false, true] {
                let frames = ik_walk(world, *start, *dir * speed, gait, 1.0 + 16.0 / speed, player, None);
                check(&frames);
                let on: Vec<&IkFrame> = frames.iter().filter(|f| f.root.x >= window.0 && f.root.x <= window.1).collect();
                eprintln!("{}", ik_row(&format!("{name} {}", if player { "player" } else { "npc" }), &on));
            }
        }
    }
    // Sharp turns half way up the 20° ramp: the 0.6 s after the stick turns.
    eprintln!("--- turns on the 20° ramp (0.6 s after the turn)");
    let (w, _) = World::ramp(20.0, 12.0);
    for (speed, gait, label) in [(1.6f32, Gait::Walk, "walk"), (4.5, Gait::Run, "run")] {
        for (turn, to) in [("90°", Vec3::Z), ("180°", -Vec3::X)] {
            for player in [false, true] {
                let at = 4.0 / speed;
                let frames = ik_walk(&w, Vec3::new(-1.0, 0.0, 0.0), Vec3::X * speed, gait, at + 1.0, player, Some((at, to * speed)));
                check(&frames);
                let after: Vec<&IkFrame> = frames
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| {
                        let t = 1.0 + *i as f32 * DT;
                        t >= at && t < at + 0.6
                    })
                    .map(|(_, f)| f)
                    .collect();
                eprintln!("{}", ik_row(&format!("{label} {turn} {}", if player { "player" } else { "npc" }), &after));
            }
        }
    }
}

// --- L1 (owner: a leg pops back on stopping, hips rise): the player's stop ---------------------

/// L1: the player's mover walks (1.6 m/s) or runs (4.5) on the flat, then the stick is released
/// (velocity goal 0) at 8 evenly spaced phases of the gait cycle (phase 0: leg 0's animated foot
/// at its most forward, from a calibration walk). Per row, over the 2.5 s after the release: per
/// foot the largest lock drift (placed minus animated foot, horizontal), the `unplanted` releases
/// (each given to the foot with the larger drift that update), the fastest world slide while the
/// foot is within 3 cm of its rest height, and its world path; the IK pelvis offset (placed minus
/// twin) peak and rise after it, the placed and twin pelvis rise after their lowest point; the
/// time to the idle state and to rest (feet and pelvis under 2 cm/s); the states visited.
/// Sanity only: finite.
#[test]
#[ignore = "measurement: player stop"]
fn player_stop() {
    use sf_controller::Gait;
    let Some(sim) = humanoid_sim() else { return };
    let names: Vec<String> = sim.graph.legs.iter().map(|l| sim.skeleton.name(usize::from(l.effector())).to_owned()).collect();
    eprintln!("legs: 0 = {}, 1 = {}", names[0], names[1]);
    let world = World::new();
    let start = Vec3::new(-4.0, 0.0, 0.0);
    let hdist = |v: Vec3| Vec2::new(v.x, v.z).length();
    for (speed, gait, label) in [(1.6f32, Gait::Walk, "walk"), (4.5, Gait::Run, "run")] {
        // Calibration: the gait period from leg 0's animated fore-aft peaks.
        let cal = ik_walk(&world, start, Vec3::X * speed, gait, 6.0, true, None);
        let z: Vec<f32> = cal.iter().map(|f| f.anim_feet[0].z).collect();
        let peaks: Vec<f32> = (1..z.len() - 1).filter(|&i| cal[i].t >= 2.0 && z[i] > z[i - 1] && z[i] >= z[i + 1]).map(|i| cal[i].t).collect();
        assert!(peaks.len() >= 3, "{label}: gait peaks {peaks:?}");
        let period = (peaks[peaks.len() - 1] - peaks[0]) / (peaks.len() - 1) as f32;
        let t_ref = peaks.iter().copied().find(|&t| t >= 2.5).unwrap_or(peaks[0]);
        eprintln!("--- {label} {speed} m/s: gait period {period:.3} s, phase 0 at t {t_ref:.3} s");
        for k in 0..8u8 {
            let at = t_ref + f32::from(k) * period / 8.0;
            let frames = ik_walk(&world, start, Vec3::X * speed, gait, at + 2.5, true, Some((at, Vec3::ZERO)));
            assert!(frames.iter().all(|f| f.pelvis.is_finite() && f.world_feet.iter().all(|d| d.is_finite())), "finite");
            let i0 = frames.iter().position(|f| f.t >= at).unwrap();
            let w = &frames[i0..];
            let last = w.last().unwrap();
            let mut feet = String::new();
            let mut unpl = [0u32; 2];
            for f in w {
                if f.unplanted > 0 {
                    let j = usize::from(hdist(f.foot[1]) > hdist(f.foot[0]));
                    unpl[j] += f.unplanted;
                }
            }
            let mut rest_at = 0usize;
            for j in 0..2 {
                let drift = w.iter().map(|f| hdist(f.foot[j])).fold(0.0f32, f32::max);
                let rest_y = last.world_feet[j].y;
                let (mut slide, mut path) = (0.0f32, 0.0f32);
                for (i, p) in w.windows(2).enumerate() {
                    let d = hdist(p[1].world_feet[j] - p[0].world_feet[j]);
                    path += d;
                    if p[1].world_feet[j].y - rest_y < 0.03 {
                        slide = slide.max(d / DT);
                    }
                    if d / DT > 0.02 {
                        rest_at = rest_at.max(i + 1);
                    }
                }
                let side = if j == 0 { "L0" } else { "R1" };
                let _ = write!(feet, "{side} drift {:5.1} unpl {} slide {:4.0} cm/s path {:5.1} | ", drift * 100.0, unpl[j], slide * 100.0, path * 100.0);
            }
            for (i, p) in w.windows(2).enumerate() {
                if (p[1].pelvis_abs - p[0].pelvis_abs).abs() / DT > 0.02 {
                    rest_at = rest_at.max(i + 1);
                }
            }
            // Lowest point, then the highest after it.
            let rise = |v: &dyn Fn(&IkFrame) -> f32| {
                let (lo, min) = w.iter().enumerate().map(|(i, f)| (i, v(f))).fold((0, f32::INFINITY), |a, b| if b.1 < a.1 { b } else { a });
                let max = w[lo..].iter().map(v).fold(f32::NEG_INFINITY, f32::max);
                (min, max - min, w[lo].t - at)
            };
            let (ik_min, ik_rise, ik_t) = rise(&|f| f.pelvis);
            let (_, abs_rise, abs_t) = rise(&|f| f.pelvis_abs);
            let (_, twin_rise, _) = rise(&|f| f.twin_pelvis_abs);
            let idle = w.iter().find(|f| f.state != w[0].state).map_or(f32::NAN, |f| f.t - at);
            let mut states: Vec<&str> = Vec::new();
            for f in w {
                if states.last() != Some(&f.state.as_str()) {
                    states.push(&f.state);
                }
            }
            let turned = states.iter().any(|s| s.starts_with("turn"));
            eprintln!(
                "{label:4} ph {k}/8 | swing {} | leave +{idle:.2} s | {feet}pelvis IK min {:+5.1} cm at +{ik_t:.2} rise {:+4.1} | placed rise {:+4.1} at +{abs_t:.2} (twin {:+4.1}) | rest +{:.2} s | turn {} | {}",
                if w[0].anim_feet[0].y > w[0].anim_feet[1].y { "L0" } else { "R1" },
                ik_min * 100.0,
                ik_rise * 100.0,
                abs_rise * 100.0,
                twin_rise * 100.0,
                w[rest_at.min(w.len() - 1)].t - at,
                if turned { "YES" } else { "no" },
                states.join(">"),
            );
        }
    }
}
