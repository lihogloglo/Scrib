//! Heading against travel on the generated gait cycles (owner 2026-11-12, WALK-1 "the walk
//! goes diagonal" in `scene level-flat-slopes`): diagnosis only.
//!
//! Per loco1 take (the three gait slot takes, then the other walk keep/maybe cycles), cooked
//! the bake's way ([`mw_bake::anim::cook_library_clip_report`] with the slot's rules: loop
//! cut, root `Match`) onto the Morrowind humanoid, over the cut loop:
//! - `travel`: the root track's ground displacement first -> last loop frame, in the root
//!   frame at the loop start (yaw from +Z, degrees, + toward +X = the actor's left);
//! - the root heading (the extracted yaw: the target hips' twist) mean and swing;
//! - the pelvis facing (from the thigh line) and the chest facing (from the upper arms) in
//!   the root frame, mean and swing: what `Match` draws on a capsule moving along +Z;
//! - `Drive` view (root track applied, `clip-preview`): pelvis and chest world facing
//!   against `travel`, and the feet midpoint's lateral drift per loop against the pelvis;
//! - `Match` view: the stance (lower) foot's sweep in the root frame (the direction the body
//!   travels over its planted foot; 0 = straight along the capsule's +Z) and its lateral
//!   travel per loop, cm;
//! - the clip meta's average and stride velocity direction (what speed matching reads).
//!
//! Then the same on the source take (Kimodo SOMA skeleton, cut the same way, root left in the
//! hips): hips ground travel against the pelvis and chest facing. Then the shipped humanoid
//! graph (`mw_graphs::sim`) in free movement at 1.6 m/s: the drawn pose's pelvis / chest
//! facing and stance-foot sweep against the capsule's +Z travel, and the gait set's and
//! directional set's weights. Run with
//! `cargo test --release -p mw_bake --test gait_heading -- --ignored --nocapture`; skips
//! without a Morrowind install or the files.
#![allow(clippy::cast_precision_loss, clippy::similar_names, clippy::too_many_lines)]

use std::path::PathBuf;
use std::sync::Arc;

use glam::{Quat, Vec3};
use mw_anim::{Animation, rig};
use mw_bake::anim::{HumanoidTarget, MwAnimRules, cook_library_clip_report, library_rig};
use mw_bake::sources::AnimSources;
use mw_graphs::sources::SourcesManifest;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::{Clip, ClipRules, CookSettings, LocalPose, RigHints, Skeleton, Trs};
use sf_character::{HumanoidBone as B, RigMap};
use sf_vfs::{Vfs, VfsPath};

const WALK: &str = "loco.walk.cycle";

/// (take id, slot whose rules cook it, is the slot's own take).
const TAKES: [(&str, &str, bool); 9] = [
    ("loco.walk.cycle.free.act.ground0.05", WALK, true),
    ("loco.run.cycle.3.act.ground0.04", "loco.jog.cycle", true),
    ("loco.sprint.cycle.1.act.ground0.05", "loco.sprint.cycle", true),
    ("loco.walk.cycle.free.act.ground0.01", WALK, false),
    ("loco.walk.cycle.free.act.ground0.02", WALK, false),
    ("loco.walk.cycle.free.act.ground0.03", WALK, false),
    ("loco.walk.cycle.free.act.ground0.04", WALK, false),
    ("loco.walk.cycle.1.act.ground0.05", WALK, false),
    ("loco.walk.cycle.2.act.ground0.04", WALK, false),
];

fn model_pos(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Vec3> {
    let mut m: Vec<glam::Affine3A> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(glam::Affine3A::IDENTITY, |p| m[p]);
        m.push(parent * t.to_affine());
    }
    m.iter().map(|a| Vec3::from(a.translation)).collect()
}

fn sample(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Vec3> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    model_pos(skeleton, &pose.to_vec())
}

/// Yaw of a ground vector from +Z, degrees (+ toward +X).
fn yaw(v: Vec3) -> f32 {
    v.x.atan2(v.z).to_degrees()
}

fn wrap(a: f32) -> f32 {
    (a + 180.0).rem_euclid(360.0) - 180.0
}

/// Circular mean and swing (max - min about the mean), degrees.
fn circ(angles: &[f32]) -> (f32, f32) {
    let (s, c) = angles.iter().fold((0.0f32, 0.0f32), |(s, c), a| (s + a.to_radians().sin(), c + a.to_radians().cos()));
    let mean = s.atan2(c).to_degrees();
    let (lo, hi) = angles.iter().fold((f32::MAX, f32::MIN), |(lo, hi), a| {
        let d = wrap(a - mean);
        (lo.min(d), hi.max(d))
    });
    (mean, hi - lo)
}

/// The facing joints of a humanoid rig.
struct Body {
    thighs: (usize, usize),
    arms: (usize, usize),
    feet: (usize, usize),
    toes: (usize, usize),
}

impl Body {
    fn new(skeleton: &Skeleton, map: &RigMap) -> Self {
        let j = |b: B| map.joint(b).and_then(|n| skeleton.find(n)).unwrap_or_else(|| panic!("{b:?} unmapped"));
        // Unmapped toes: the foot's first child.
        let toe = |b: B, foot: usize| map.joint(b).and_then(|n| skeleton.find(n)).or_else(|| (0..skeleton.len()).find(|&c| skeleton.parent(c) == Some(foot))).unwrap_or_else(|| panic!("{b:?}: no toe"));
        Self {
            thighs: (j(B::LeftUpperLeg), j(B::RightUpperLeg)),
            arms: (j(B::LeftUpperArm), j(B::RightUpperArm)),
            feet: (j(B::LeftFoot), j(B::RightFoot)),
            toes: (toe(B::LeftToes, j(B::LeftFoot)), toe(B::RightToes, j(B::RightFoot))),
        }
    }

    /// Facing yaw (deg) of a left-right pair: forward = left x up.
    fn facing(pos: &[Vec3], (l, r): (usize, usize)) -> f32 {
        let mut left = pos[l] - pos[r];
        left.y = 0.0;
        yaw(left.normalize_or(Vec3::X).cross(Vec3::Y))
    }

    fn pelvis(&self, pos: &[Vec3]) -> f32 {
        Self::facing(pos, self.thighs)
    }

    fn chest(&self, pos: &[Vec3]) -> f32 {
        Self::facing(pos, self.arms)
    }

    fn feet_mid(&self, pos: &[Vec3]) -> Vec3 {
        (pos[self.feet.0] + pos[self.feet.1]) * 0.5
    }

    /// Per foot (left, right): the mean toe-out yaw (foot -> toe, deg, against `facing`)
    /// over the frames where that foot is the stance (lower) one; + = toward the actor's left.
    fn toe_yaw(&self, frames: &[Vec<Vec3>], facing: f32) -> (f32, f32) {
        let side = |foot: usize, toe: usize, other: usize| {
            let v: Vec<f32> = frames.iter().filter(|p| p[foot].y <= p[other].y).map(|p| wrap(yaw(p[toe] - p[foot]) - facing)).collect();
            if v.is_empty() { f32::NAN } else { circ(&v).0 }
        };
        (side(self.feet.0, self.toes.0, self.feet.1), side(self.feet.1, self.toes.1, self.feet.0))
    }
}

/// Sum of the stance (lower) foot's frame-to-frame ground motion, negated: how the body
/// moves over its planted feet in the frame the positions are in.
fn stance_sweep(body: &Body, frames: &[Vec<Vec3>]) -> Vec3 {
    let mut sum = Vec3::ZERO;
    for w in frames.windows(2) {
        let foot = |pos: &[Vec3]| if pos[body.feet.0].y <= pos[body.feet.1].y { body.feet.0 } else { body.feet.1 };
        let f = foot(&w[0]);
        let d = w[1][f] - w[0][f];
        sum -= Vec3::new(d.x, 0.0, d.z);
    }
    sum
}

/// [`stance_sweep`]'s direction (deg) per stance foot: (over the left, over the right).
fn stance_sweep_sides(body: &Body, frames: &[Vec<Vec3>]) -> (f32, f32) {
    let (mut l, mut r) = (Vec3::ZERO, Vec3::ZERO);
    for w in frames.windows(2) {
        let left = w[0][body.feet.0].y <= w[0][body.feet.1].y;
        let f = if left { body.feet.0 } else { body.feet.1 };
        let d = w[1][f] - w[0][f];
        *(if left { &mut l } else { &mut r }) -= Vec3::new(d.x, 0.0, d.z);
    }
    (yaw(l), yaw(r))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn times(start: f32, end: f32, rate: f32) -> Vec<f32> {
    let n = ((end - start) * rate).round().max(1.0) as usize;
    (0..=n).map(|i| (start + (end - start) * i as f32 / n as f32).min(end)).collect()
}

struct Lib {
    install: MorrowindInstall,
    loco: PathBuf,
    target: HumanoidTarget,
}

fn lib() -> Option<Lib> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let loco = AnimSources::local().dir.join("generated").join("loco1");
    if !loco.exists() {
        eprintln!("{} missing; skipping", loco.display());
        return None;
    }
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    let bytes = vfs.read(&VfsPath::new(HUMANOID_SETS[0].skeleton).unwrap()).unwrap();
    let target = HumanoidTarget::new(rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap()).unwrap();
    Some(Lib { install, loco, target })
}

fn take_path(loco: &std::path::Path, id: &str) -> PathBuf {
    let (activity, _) = id.split_once(".act.").unwrap();
    loco.join(activity).join("act").join(format!("{id}.bvh"))
}

fn slot_rules(slot: &str) -> ClipRules {
    let manifest = SourcesManifest::builtin();
    let s = &manifest.slots[slot];
    s.rules(&s.candidates[0])
}

/// The source take cooked as the bake does before its retarget (loop cut, root left in the
/// hips): (skeleton, map, clip).
fn source_cook(path: &std::path::Path, rules: &ClipRules) -> (Skeleton, RigMap, Clip) {
    let name = path.file_name().unwrap().to_string_lossy().into_owned();
    let virtual_path = AnimSources::virtual_path("generated", &name);
    let importers = mw_bake::anim::anim_importers();
    let bytes = std::fs::read(path).unwrap();
    let imported = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).unwrap();
    let mut skeleton_rules = MwAnimRules::skeleton_rules();
    skeleton_rules.root_motion_joint.clone_from(&imported.root_motion);
    let (skeleton, _) = Skeleton::with_rules(imported.skeleton.as_ref().unwrap(), &skeleton_rules).unwrap();
    let (map, _) = library_rig(&skeleton, "generated").unwrap();
    let hints = RigHints { profile: map.profile(&skeleton).ok(), ..RigHints::default() };
    let profile = sf_anim::detect_profile(&skeleton, &hints).unwrap();
    let mut cook_rules = rules.clone();
    cook_rules.root_motion.extract = false;
    cook_rules.root_motion.anchor_end = false;
    let (clip, _) = sf_anim::cook_clip_report(&skeleton, Some(&profile), imported.clips.first().unwrap(), &cook_rules, &CookSettings::default()).unwrap();
    (skeleton, map, clip)
}

#[test]
#[ignore = "measurement: gait cycle heading against travel (WALK-1 diagonal walk)"]
fn measure_gait_heading() {
    let Some(lib) = lib() else { return };
    let tb = Body::new(&lib.target.skeleton, &lib.target.map);
    {
        let s = &lib.target.skeleton;
        let rest = model_pos(s, s.rest());
        let f = |a: usize, b: usize| wrap(yaw(rest[b] - rest[a]) - tb.pelvis(&rest));
        println!("target bind: pelvis {:.1}, toe L {:.1} R {:.1}", tb.pelvis(&rest), f(tb.feet.0, tb.toes.0), f(tb.feet.1, tb.toes.1));
    }
    println!("angles in degrees from +Z, + toward +X (the actor's left); lat in cm per loop, + = left");
    println!("\n== cooked (Morrowind humanoid, slot rules: loop cut, root Match), over the cut loop");
    println!(
        "{:<30} {:>5} {:>5} | {:>6} | root yaw {:>5} {:>5} | root frame: pelvis {:>5} {:>5} chest {:>5} {:>5} | Drive: pel-trv {:>6} chest-trv {:>6} feet lat {:>6} | Match: sweep {:>6} lat {:>6} | meta avg {:>6} stride {:>6}",
        "take", "loop", "m/s", "travel", "mean", "swing", "mean", "swing", "mean", "swing", "", "", "", "", "", "", ""
    );
    let mut rows = Vec::new();
    for (id, slot, own) in TAKES {
        let path = take_path(&lib.loco, id);
        if !path.exists() {
            eprintln!("{} missing", path.display());
            continue;
        }
        let rules = slot_rules(slot);
        let (clip, _) = cook_library_clip_report("generated", &path, None, &rules, &lib.target).unwrap();
        let (start, end) = clip.loop_range.unwrap_or((0.0, clip.duration));
        let ts = times(start, end, clip.rate);
        let (p0, y0) = clip.root_at(start);
        let to_start = Quat::from_rotation_y(-y0);
        let root: Vec<(Vec3, f32)> = ts
            .iter()
            .map(|&t| {
                let (p, y) = clip.root_at(t);
                (to_start * (p - p0), y - y0)
            })
            .collect();
        let local: Vec<Vec<Vec3>> = ts.iter().map(|&t| sample(&lib.target.skeleton, &clip, t)).collect();
        let world: Vec<Vec<Vec3>> =
            local.iter().zip(&root).map(|(pos, (p, y))| pos.iter().map(|v| Quat::from_rotation_y(*y) * *v + *p).collect()).collect();
        let disp = root.last().unwrap().0;
        let travel = yaw(disp);
        let speed = Vec3::new(disp.x, 0.0, disp.z).length() / (end - start);
        let ry: Vec<f32> = root.iter().map(|r| r.1.to_degrees()).collect();
        let (ry_mean, ry_swing) = circ(&ry);
        let pel_l: Vec<f32> = local.iter().map(|p| tb.pelvis(p)).collect();
        let che_l: Vec<f32> = local.iter().map(|p| tb.chest(p)).collect();
        let (pl, pls) = circ(&pel_l);
        let (cl, cls) = circ(&che_l);
        let (pw, _) = circ(&world.iter().map(|p| tb.pelvis(p)).collect::<Vec<_>>());
        let (cw, _) = circ(&world.iter().map(|p| tb.chest(p)).collect::<Vec<_>>());
        let fd = tb.feet_mid(world.last().unwrap()) - tb.feet_mid(&world[0]);
        let pw_left = Quat::from_rotation_y(pw.to_radians()) * Vec3::X;
        let feet_lat = fd.dot(pw_left) * 100.0;
        let sweep = stance_sweep(&tb, &local);
        let (avg, stride) = (clip.meta.average_velocity, clip.meta.stride_velocity());
        let toes = tb.toe_yaw(&local, 0.0);
        let sides = stance_sweep_sides(&tb, &local);
        let width = local.iter().map(|p| (p[tb.feet.0].x - p[tb.feet.1].x).abs()).sum::<f32>() / local.len() as f32 * 100.0;
        for x in [travel, ry_mean, pl, cl, pw, cw, feet_lat, sweep.x, speed] {
            assert!(x.is_finite(), "{id}: not finite");
        }
        let label = format!("{}{}", if own { "*" } else { " " }, id.trim_start_matches("loco.").replace(".act.ground0", ""));
        println!(
            "{:<30} {:>5.2} {:>5.2} | {:>6.1} | root yaw {:>5.1} {:>5.1} | root frame: pelvis {:>5.1} {:>5.1} chest {:>5.1} {:>5.1} | Drive: pel-trv {:>6.1} chest-trv {:>6.1} feet lat {:>6.1} | Match: sweep {:>6.1} lat {:>6.1} | meta avg {:>6.1} stride {:>6.1} | toe L {:>5.1} R {:>5.1} width {:>4.1} | sweep over L {:>5.1} R {:>5.1}",
            label,
            end - start,
            speed,
            travel,
            ry_mean,
            ry_swing,
            pl,
            pls,
            cl,
            cls,
            wrap(pw - travel),
            wrap(cw - travel),
            feet_lat,
            yaw(sweep),
            sweep.x * 100.0,
            yaw(avg),
            yaw(stride),
            toes.0,
            toes.1,
            width,
            sides.0,
            sides.1,
        );
        rows.push((id, slot, rules));
    }

    println!("\n== source (Kimodo SOMA, cut the same way, root in the hips), over the cut loop");
    println!(
        "{:<30} {:>5} | {:>6} | pelvis-trv {:>6} swing {:>5} | chest-trv {:>6} swing {:>5} | feet lat {:>6}",
        "take", "loop", "travel", "", "", "", "", ""
    );
    for (id, _, rules) in &rows {
        let path = take_path(&lib.loco, id);
        let (skeleton, map, clip) = source_cook(&path, rules);
        let sb = Body::new(&skeleton, &map);
        if id == &rows[0].0 {
            let rest = model_pos(&skeleton, skeleton.rest());
            let (pf, f) = (sb.pelvis(&rest), |a: usize, b: usize| wrap(yaw(rest[b] - rest[a]) - sb.pelvis(&rest)));
            println!("source rest: pelvis {pf:.1}, toe L {:.1} R {:.1}", f(sb.feet.0, sb.toes.0), f(sb.feet.1, sb.toes.1));
        }
        let hips = map.joint(B::Hips).and_then(|n| skeleton.find(n)).unwrap();
        let (start, end) = clip.loop_range.unwrap_or((0.0, clip.duration));
        let frames: Vec<Vec<Vec3>> = times(start, end, clip.rate).iter().map(|&t| sample(&skeleton, &clip, t)).collect();
        let disp = frames.last().unwrap()[hips] - frames[0][hips];
        let travel = yaw(disp);
        let (pm, ps) = circ(&frames.iter().map(|p| sb.pelvis(p)).collect::<Vec<_>>());
        let (cm, cs) = circ(&frames.iter().map(|p| sb.chest(p)).collect::<Vec<_>>());
        let fd = sb.feet_mid(frames.last().unwrap()) - sb.feet_mid(&frames[0]);
        let feet_lat = fd.dot(Quat::from_rotation_y(pm.to_radians()) * Vec3::X) * 100.0;
        let stoes = sb.toe_yaw(&frames, pm);
        assert!(travel.is_finite() && pm.is_finite() && cm.is_finite(), "{id}");
        println!(
            "{:<30} {:>5.2} | {:>6.1} | pelvis-trv {:>6.1} swing {:>5.1} | chest-trv {:>6.1} swing {:>5.1} | feet lat {:>6.1} | toe L {:>5.1} R {:>5.1} (vs pelvis)",
            id.trim_start_matches("loco.").replace(".act.ground0", ""),
            end - start,
            travel,
            wrap(pm - travel),
            ps,
            wrap(cm - travel),
            cs,
            feet_lat,
            stoes.0,
            stoes.1,
        );
    }

    graph_walk(&lib);
}

/// The shipped humanoid graph with the gait slots cooked the bake's way, in free movement at
/// 1.6 m/s along +Z (`strafing` off, as `mw_player_bevy` feeds it without a lock-on).
fn graph_walk(lib: &Lib) {
    use mw_graphs::build::{gaits, params as p};
    let vfs = mw_vfs::mount_install(&lib.install, Vfs::builder()).unwrap().vfs;
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let hs = &HUMANOID_SETS[0];
    let skeleton = rig::skeleton(&mw_nif::load(&read(hs.skeleton).unwrap()).unwrap()).unwrap();
    let anims = hs
        .animations
        .iter()
        .filter_map(|a| Some(((*a).to_owned(), Animation::from_file(a, &read(a)?).unwrap())))
        .filter(|(_, a)| !a.text_keys.is_empty())
        .collect();
    let mut set = AnimSet::new(hs.name, skeleton, anims).with_humanoid_rig();
    let manifest = SourcesManifest::builtin();
    let library = AnimSources::local();
    let target = HumanoidTarget::new(set.skeleton.clone()).unwrap();
    for (gait, _) in gaits::SET {
        let name = manifest.gait_slot("humanoid", gaits::STYLE, gait, "cycle").expect("shipped gait slot");
        let slot = &manifest.slots[&name];
        let c = &slot.candidates[0];
        let file = c.file.clone().expect("generated takes name their file");
        let Some(path) = library.real_path(&AnimSources::virtual_path(&c.source, &file)) else {
            eprintln!("{name}: not in the library; skipping the graph part");
            return;
        };
        let mut clip = mw_bake::anim::cook_library_clip(&c.source, &path, Some(&c.clip), &slot.rules(c), &target).unwrap();
        clip.name = c.reference();
        set.insert_cooked(c.reference(), Arc::new(clip));
    }
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let clips = set.binding_clips(&v.graph).unwrap();
    let graph = v.graph;
    let body = Body::new(&set.skeleton, &target.map);
    let dt = 1.0f32 / 60.0;
    for (speed, ground) in [(1.6, false), (0.9, true), (1.6, true), (2.3, true), (3.05, true)] {
        let mut sim = mw_graphs::sim::Sim::new(graph.clone(), set.skeleton.clone(), clips.clone()).unwrap();
        sim.set(p::MOVING, 1.0);
        sim.set(p::RUNNING, 0.0);
        sim.set(p::STRAFING, 0.0);
        sim.set(p::SPEED, speed);
        sim.set_vec2(p::MOVE, glam::Vec2::Y);
        sim.velocity = Vec3::new(0.0, 0.0, speed);
        let mut frames = Vec::new();
        for frame in 0..300 {
            if ground {
                // Flat ground at the character's feet level (y = 0), the capsule moving +Z.
                sim.moved = sf_anim::RootDelta::new(Vec3::new(0.0, 0.0, speed * dt), 0.0);
                let origins = sim.out.probe_origins.clone();
                for (slot, o) in sim.ground.iter_mut().zip(origins.iter().chain(std::iter::repeat(&Vec3::ZERO))) {
                    *slot = Some(sf_anim_graph::GroundSample { point: Vec3::new(o.x, 0.0, o.z), normal: Vec3::Y });
                }
            }
            sim.run(dt, dt);
            if frame >= 180 {
                frames.push(model_pos(&sim.skeleton, &sim.out.pose.to_vec()));
            }
        }
        let (pm, ps) = circ(&frames.iter().map(|p| body.pelvis(p)).collect::<Vec<_>>());
        let (cm, cs) = circ(&frames.iter().map(|p| body.chest(p)).collect::<Vec<_>>());
        let sweep = stance_sweep(&body, &frames);
        let toes = body.toe_yaw(&frames, 0.0);
        let sides = stance_sweep_sides(&body, &frames);
        assert!(pm.is_finite() && cm.is_finite() && sweep.is_finite());
        println!(
            "\n== graph (shipped humanoid, free movement, {speed} m/s along +Z, strafing 0, {}), 2 s",
            if ground { "flat ground probes + moved" } else { "no ground" }
        );
        println!(
            "pelvis {pm:.1} (swing {ps:.1}), chest {cm:.1} (swing {cs:.1}) vs capsule travel 0.0; stance-foot sweep {:.1} deg, lat {:.1} cm over {:.2} m; toe L {:.1} R {:.1}; sweep over L {:.1} R {:.1}",
            yaw(sweep),
            sweep.x * 100.0,
            sweep.z,
            toes.0,
            toes.1,
            sides.0,
            sides.1
        );
        for (i, n) in sim.graph.nodes.iter().enumerate() {
            let Some((lanes, w)) = sim.instance.player(u32::try_from(i).unwrap()) else { continue };
            if w > 1e-3 {
                let lanes: Vec<(f32, f32)> = lanes.iter().map(|l| ((l.0 * 100.0).round() / 100.0, (l.1 * 1000.0).round() / 1000.0)).collect();
                println!("  player {} weight {w:.3} lanes (time, weight) {lanes:?}", n.name);
            }
        }
    }
}
