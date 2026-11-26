//! AN1 (owner 2026-11-26: the shipped Kimodo walk / jog / sprint still look bad): which stage
//! introduces each defect, diagnosis only.
//!
//! Per shipped gait take (`sources.ron` `loco.{walk,jog,sprint}.cycle`, root `Match`, loop
//! cut), over its cut loop:
//! - (a) `src`: the source cooked on its own skeleton (Kimodo SOMA, root left in the hips);
//! - (b) `tgt`: retargeted onto the Morrowind humanoid and cooked the bake's way
//!   ([`mw_bake::anim::cook_library_clip_report`] with the slot's rules), root track applied
//!   for the world-space columns;
//! - (c) `graph`: the shipped humanoid graph (`mw_graphs::sim::Sim`, free movement, strafing 0,
//!   flat ground probes, capsule moving +Z) at the gait speed and at the band edges, 2 s after
//!   3 s of settling (4 s measured);
//! - (d) `MW`: Morrowind's native `WalkForward` / `RunForward`, and the graph locked on
//!   (strafing 1: the Morrowind directional sets) at their own stride speed.
//!
//! Columns: upper-arm sagittal swing vs the trunk (deg, back - .. fwd +; trunk up = hips ->
//! neck, forward from the thighs), elbow and wrist min clearance to the skinned Morrowind
//! chest mesh (cm, `generated_arms.rs`'s nearest-vertex method; target rig only), elbow and
//! wrist min lateral offset from the chest joint's mid-sagittal plane (cm, < 0 = crosses the
//! midline), segment pitch vs bind/rest (`Chest -> UpperChest`, `UpperChest -> Neck`, deg forward
//! +) and hips -> neck vs world vertical, pelvis (hips joint) vertical bob (cm, max - min),
//! stance-foot skate (the lower foot while within 2 cm of its lowest: mean horizontal world
//! speed, cm/s), cadence (clips: 2 steps per one-stride loop; graph: stance switches with 1 cm hysteresis per s); for (c) the heaviest
//! clip's playback rate (clip s per s) and the stride ratio left to `StrideWarp`
//! (speed / (natural stride speed x rate), and clamped to the op's range).
//!
//! `AN5b` (`measure_an5b_loco2_takes`): the same (a) / (b) columns for every looping loco2 gait
//! take (`generated/loco2/loco2.<gait>.{cycle,back,strafe}.*`, cooked with the gait's
//! `loco.<gait>.cycle` slot rules; starts, stops, pivots and turns skipped), with its
//! `grades.json` gates, then the takes ranked per (gait, direction) against Morrowind's native
//! group of that direction (`Walk*` for walk, `Run*` for jog and sprint; sprint has no MW
//! group). Here cadence = stance switches counted over the cut loop / loop s (`n` = steps in
//! the loop), not the 2-steps-per-loop assumption. Ranking (a printed measurement, not a
//! gate), on the retargeted (b) row, lower = closer: score = mean over both arms of
//! (|swing min - MW min| + |swing max - MW max|) deg + 5 x |bob - MW bob| cm + 0.5 x stance
//! skate cm/s (1 cm of bob ~ 5 deg of swing ~ 10 cm/s of skate); top 2 per group among the
//! ungated takes the loop cut could cut (`UNCUT` = no loop found, the whole take would play), and
//! for forward gaits also the top 2 within that gait's graph speed band (`GAITS`). Speed and
//! cadence are printed, not scored.
//! Run with `cargo test -p mw_bake --test it gait_diagnosis:: -- --ignored --nocapture`; skips
//! without a Morrowind install or the library.
#![allow(clippy::cast_precision_loss, clippy::similar_names, clippy::too_many_lines, clippy::many_single_char_names)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use glam::{Affine3A, Quat, Vec3};
use mw_anim::{Animation, rig};
use mw_bake::anim::{HumanoidTarget, MwAnimRules, cook_library_clip_report, library_rig};
use mw_bake::sources::AnimSources;
use mw_graphs::sources::SourcesManifest;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::{Clip, ClipRules, CookSettings, LocalPose, RigHints, Skeleton, Trs};
use sf_character::{HumanoidBone as B, RigMap};
use sf_vfs::{DirEntry, Vfs, VfsPath};

/// Sampling rate of the clip stages, Hz (the graph runs at 60 too).
const RATE: f32 = 60.0;

/// (gait, slot, take file under loco1, band edges m/s, running).
type Gait = (&'static str, &'static str, &'static str, (f32, f32), bool);
const GAITS: [Gait; 3] = [
    ("walk", "loco.walk.cycle", "loco.walk.cycle.1/act/loco.walk.cycle.1.act.ground0.05.bvh", (1.2, 2.2), false),
    ("jog", "loco.jog.cycle", "loco.run.cycle.3/act/loco.run.cycle.3.act.ground0.04.bvh", (3.5, 5.5), true),
    ("sprint", "loco.sprint.cycle", "loco.sprint.cycle.1/act/loco.sprint.cycle.1.act.ground0.05.bvh", (6.0, 7.5), true),
];

fn model_aff(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Affine3A> {
    let mut m: Vec<Affine3A> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(Affine3A::IDENTITY, |p| m[p]);
        m.push(parent * t.to_affine());
    }
    m
}

fn sample(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Affine3A> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    model_aff(skeleton, &pose.to_vec())
}

fn pos(m: &[Affine3A]) -> Vec<Vec3> {
    m.iter().map(|a| Vec3::from(a.translation)).collect()
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn times(start: f32, end: f32) -> Vec<f32> {
    let n = ((end - start) * RATE).round().max(1.0) as usize;
    (0..=n).map(|i| (start + (end - start) * i as f32 / n as f32).min(end)).collect()
}

fn aff(a: mw_nif::glam::Affine3A) -> Affine3A {
    Affine3A::from_cols_array(&a.to_cols_array())
}

/// Bind position, normal, `(target joint, inverse bind, weight)` influences.
type SkinVert = (Vec3, Vec3, Vec<(usize, Affine3A, f32)>);

/// The skinned Morrowind chest mesh (`generated_arms.rs`'s `Torso`).
struct Torso {
    verts: Vec<SkinVert>,
}

impl Torso {
    fn posed(&self, model: &[Affine3A]) -> Vec<(Vec3, Vec3)> {
        self.verts
            .iter()
            .map(|(p, n, inf)| {
                let (mut pos, mut nor, mut w) = (Vec3::ZERO, Vec3::ZERO, 0.0);
                for (j, ib, wt) in inf {
                    let m = model[*j] * *ib;
                    pos += m.transform_point3(*p) * *wt;
                    nor += m.transform_vector3(*n) * *wt;
                    w += wt;
                }
                (pos / w.max(1e-6), nor.normalize_or_zero())
            })
            .collect()
    }

    /// Signed distance from `p` to the nearest vertex (outside +).
    fn clearance(posed: &[(Vec3, Vec3)], p: Vec3) -> f32 {
        let (v, n) = posed.iter().min_by(|a, b| a.0.distance_squared(p).total_cmp(&b.0.distance_squared(p))).unwrap();
        let d = v.distance(p);
        if (p - *v).dot(*n) < 0.0 { -d } else { d }
    }
}

/// The vanilla male body's chest shape skinned on `skeleton` (`generated_arms.rs` `torso_mesh`).
fn torso_mesh(vfs: &Vfs, skeleton: &Skeleton) -> Option<Torso> {
    let files: Vec<String> = vfs
        .read_dir("meshes/b")
        .into_iter()
        .filter_map(|e| if let DirEntry::File(p) = e { Some(p.as_str().to_owned()) } else { None })
        .collect();
    let skins = ["imperial", "nord", "dark elf"]
        .iter()
        .find_map(|race| files.iter().find(|f| f.contains(&format!("b_n_{race}_m_skins"))).cloned())
        .or_else(|| files.iter().find(|f| f.contains("_m_skins")).cloned())?;
    let find = |n: &str| skeleton.find(n).or_else(|| (0..skeleton.len()).find(|&j| skeleton.name(j).eq_ignore_ascii_case(n)));
    let model = mw_nif::load(&vfs.read(&VfsPath::new(&skins).ok()?).ok()?).ok()?;
    let shape_name = |m: usize| model.nodes.iter().find(|n| n.mesh == Some(m)).map_or(String::new(), |n| n.name.to_ascii_lowercase());
    let mut verts = Vec::new();
    for (mi, mesh) in model.meshes.iter().enumerate() {
        if !shape_name(mi).contains("chest") {
            continue;
        }
        let Some(skin) = &mesh.skin else { continue };
        let bones: Vec<Option<usize>> = skin.bones.iter().map(|b| find(&model.nodes[b.node].name)).collect();
        for (i, p) in skin.positions.iter().enumerate() {
            let n = skin.normals.get(i).map_or(Vec3::ZERO, |n| Vec3::from(*n));
            let inf: Vec<(usize, Affine3A, f32)> =
                skin.influences[i].iter().filter_map(|&(b, w)| bones[b as usize].map(|j| (j, aff(skin.bones[b as usize].inverse_bind), w))).collect();
            if !inf.is_empty() {
                verts.push((Vec3::from(*p), n, inf));
            }
        }
    }
    println!("chest mesh: {skins}, {} verts", verts.len());
    (!verts.is_empty()).then_some(Torso { verts })
}

#[derive(Clone, Copy)]
struct Arm {
    upper: usize,
    lower: usize,
    hand: usize,
    /// +1 left, -1 right.
    side: f32,
}

/// The measured joints of a humanoid rig.
struct Body {
    arms: [Arm; 2],
    hips: usize,
    chest: usize,
    upper_chest: usize,
    neck: usize,
    thighs: (usize, usize),
    feet: (usize, usize),
    /// Segment pitches at rest/bind (`Chest -> UpperChest`, `UpperChest -> Neck`).
    rest_segs: [f32; 2],
}

/// Per frame.
#[derive(Clone, Copy)]
struct Frame {
    swing: [f32; 2],
    el_clear: [f32; 2],
    wr_clear: [f32; 2],
    el_lat: [f32; 2],
    wr_lat: [f32; 2],
    segs: [f32; 2],
    trunk: f32,
}

impl Body {
    fn new(skeleton: &Skeleton, map: &RigMap) -> Self {
        let j = |b: B| map.joint(b).and_then(|n| skeleton.find(n));
        let need = |b: B| j(b).unwrap_or_else(|| panic!("{b:?} unmapped"));
        let arm = |u, l, h, side| Arm { upper: need(u), lower: need(l), hand: need(h), side };
        let chest = j(B::Chest).unwrap_or_else(|| need(B::Spine));
        let upper_chest = j(B::UpperChest).unwrap_or(chest);
        let mut b = Self {
            arms: [arm(B::LeftUpperArm, B::LeftLowerArm, B::LeftHand, 1.0), arm(B::RightUpperArm, B::RightLowerArm, B::RightHand, -1.0)],
            hips: need(B::Hips),
            chest,
            upper_chest,
            neck: need(B::Neck),
            thighs: (need(B::LeftUpperLeg), need(B::RightUpperLeg)),
            feet: (need(B::LeftFoot), need(B::RightFoot)),
            rest_segs: [0.0; 2],
        };
        let rest = pos(&model_aff(skeleton, skeleton.rest()));
        b.rest_segs = b.segs(&rest);
        b
    }

    fn names(&self, skeleton: &Skeleton) -> String {
        let n = |j: usize| skeleton.name(j).to_owned();
        format!("segs {}>{} | {}>{}; midline at {}", n(self.chest), n(self.upper_chest), n(self.upper_chest), n(self.neck), n(self.upper_chest))
    }

    /// (left, forward), horizontal, from the thighs.
    fn frame(&self, p: &[Vec3]) -> (Vec3, Vec3) {
        let mut left = p[self.thighs.0] - p[self.thighs.1];
        left.y = 0.0;
        let left = left.normalize_or(Vec3::X);
        (left, left.cross(Vec3::Y))
    }

    fn segs(&self, p: &[Vec3]) -> [f32; 2] {
        let (_, fwd) = self.frame(p);
        let pitch = |v: Vec3| v.dot(fwd).atan2(v.dot(Vec3::Y)).to_degrees();
        [pitch(p[self.upper_chest] - p[self.chest]), pitch(p[self.neck] - p[self.upper_chest])]
    }

    fn measure(&self, p: &[Vec3], torso: Option<&[(Vec3, Vec3)]>) -> Frame {
        let (left, fwd) = self.frame(p);
        let up_t = (p[self.neck] - p[self.hips]).normalize_or(Vec3::Y);
        let fwd_t = left.cross(up_t).normalize_or(fwd);
        let c = p[self.upper_chest];
        let segs = self.segs(p);
        let mut f = Frame {
            swing: [0.0; 2],
            el_clear: [f32::NAN; 2],
            wr_clear: [f32::NAN; 2],
            el_lat: [0.0; 2],
            wr_lat: [0.0; 2],
            segs: [segs[0] - self.rest_segs[0], segs[1] - self.rest_segs[1]],
            trunk: (p[self.neck] - p[self.hips]).dot(fwd).atan2((p[self.neck] - p[self.hips]).y).to_degrees(),
        };
        for (i, a) in self.arms.iter().enumerate() {
            let out = left * a.side;
            let (s, e, w) = (p[a.upper], p[a.lower], p[a.hand]);
            let u = (e - s).normalize_or_zero();
            f.swing[i] = u.dot(fwd_t).atan2(-u.dot(up_t)).to_degrees();
            f.el_lat[i] = (e - c).dot(out);
            f.wr_lat[i] = (w - c).dot(out);
            if let Some(t) = torso {
                f.el_clear[i] = Torso::clearance(t, e);
                f.wr_clear[i] = Torso::clearance(t, w);
            }
        }
        f
    }
}

/// One stage's row.
struct Row {
    label: String,
    frames: Vec<Frame>,
    /// World joint positions per frame (for bob, skate, cadence).
    world: Vec<Vec<Vec3>>,
    dt: f32,
    /// Graph only: (rate, stride ratio raw, stride clamped, playing).
    graph: Option<(f32, f32, f32, String)>,
}

fn min(v: impl Iterator<Item = f32>) -> f32 {
    v.fold(f32::INFINITY, f32::min)
}

fn max(v: impl Iterator<Item = f32>) -> f32 {
    v.fold(f32::NEG_INFINITY, f32::max)
}

fn mean(v: impl Iterator<Item = f32>) -> f32 {
    let (s, n) = v.fold((0.0, 0usize), |(s, n), x| (s + x, n + 1));
    s / n.max(1) as f32
}

impl Row {
    /// (bob cm, per-step bob min..max cm, skate cm/s, cadence steps/s).
    fn legs(&self, body: &Body) -> (f32, (f32, f32), f32, f32) {
        let w = &self.world;
        let hy = |p: &Vec<Vec3>| p[body.hips].y;
        let bob = (max(w.iter().map(hy)) - min(w.iter().map(hy))) * 100.0;
        let (l, r) = body.feet;
        let fj = [l, r];
        let low = [min(w.iter().map(|p| p[l].y)), min(w.iter().map(|p| p[r].y))];
        let mut speeds = Vec::new();
        let mut stance = usize::from(w[0][r].y < w[0][l].y);
        // Frame indices of stance switches (lower foot, 1 cm hysteresis).
        let mut switches: Vec<usize> = Vec::new();
        for (i, pair) in w.windows(2).enumerate() {
            let (a, b) = (&pair[0], &pair[1]);
            let other = 1 - stance;
            if a[fj[other]].y + 0.01 < a[fj[stance]].y {
                stance = other;
                switches.push(i);
            }
            let f = fj[stance];
            if a[f].y <= low[stance] + 0.02 {
                let d = b[f] - a[f];
                speeds.push(Vec3::new(d.x, 0.0, d.z).length() / self.dt);
            }
        }
        // Cadence between the first and last switch; per-step bob between switches.
        let secs = self.dt * (w.len() - 1) as f32;
        let (cad, steps) = if self.graph.is_none() {
            // The cut loops are one stride (`loop_cut.rs`; MW groups too): 2 steps per loop.
            (2.0 / secs.max(1e-3), (bob, bob))
        } else if switches.len() >= 2 {
            let span = (switches[switches.len() - 1] - switches[0]) as f32 * self.dt;
            let steps: Vec<f32> = switches.windows(2).map(|s| (max(w[s[0]..=s[1]].iter().map(hy)) - min(w[s[0]..=s[1]].iter().map(hy))) * 100.0).collect();
            ((switches.len() - 1) as f32 / span, (min(steps.iter().copied()), max(steps.iter().copied())))
        } else {
            // A one-stride loop may show a single switch: 2 steps per loop.
            (f32::NAN, (bob, bob))
        };
        (bob, steps, mean(speeds.into_iter()) * 100.0, cad)
    }

    fn print(&self, body: &Body) {
        let fr = &self.frames;
        let sw = |i: usize| (min(fr.iter().map(|f| f.swing[i])), max(fr.iter().map(|f| f.swing[i])));
        let cm = |v: f32| if v.is_finite() { format!("{:>4.1}", v * 100.0) } else { "   -".to_owned() };
        let mn = |g: &dyn Fn(&Frame) -> f32| min(fr.iter().map(g));
        let (sl, sr) = (sw(0), sw(1));
        let (bob, steps, skate, cad) = self.legs(body);
        let graph = self.graph.as_ref().map_or(String::new(), |(rate, raw, cl, playing)| format!(" | rate {rate:>4.2} stride {raw:>4.2}->{cl:>4.2} | {playing}"));
        println!(
            "{:<26} | L {:>5.1}..{:>5.1} R {:>5.1}..{:>5.1} | el {}/{} wr {}/{} | lat el {}/{} wr {}/{} | seg {:>5.1} {:>5.1} trunk {:>5.1} | bob {:>4.1} (step {:>4.1}..{:>4.1}) skate {:>5.1} cad {:>4.2}{graph}",
            self.label,
            sl.0,
            sl.1,
            sr.0,
            sr.1,
            cm(mn(&|f| f.el_clear[0])),
            cm(mn(&|f| f.el_clear[1])),
            cm(mn(&|f| f.wr_clear[0])),
            cm(mn(&|f| f.wr_clear[1])),
            cm(mn(&|f| f.el_lat[0])),
            cm(mn(&|f| f.el_lat[1])),
            cm(mn(&|f| f.wr_lat[0])),
            cm(mn(&|f| f.wr_lat[1])),
            mean(fr.iter().map(|f| f.segs[0])),
            mean(fr.iter().map(|f| f.segs[1])),
            mean(fr.iter().map(|f| f.trunk)),
            bob,
            steps.0,
            steps.1,
            skate,
            cad,
        );
    }
}

struct Lib {
    install: MorrowindInstall,
    vfs: Vfs,
    loco: PathBuf,
    target: HumanoidTarget,
}

fn lib() -> Option<Lib> {
    lib_at("loco1")
}

/// The library's `generated/<batch>` folder (`loco1`, `loco2`, ...) with the Morrowind target.
fn lib_at(batch: &str) -> Option<Lib> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let loco = AnimSources::local().dir.join("generated").join(batch);
    if !loco.exists() {
        eprintln!("{} missing; skipping", loco.display());
        return None;
    }
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    let bytes = vfs.read(&VfsPath::new(HUMANOID_SETS[0].skeleton).unwrap()).unwrap();
    let target = HumanoidTarget::new(rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap()).unwrap();
    Some(Lib { install, vfs, loco, target })
}

fn slot_rules(slot: &str) -> ClipRules {
    let manifest = SourcesManifest::builtin();
    let s = &manifest.slots[slot];
    s.rules(&s.candidates[0])
}

/// The source take cooked as the bake does before its retarget (loop cut, root left in the
/// hips): (skeleton, map, clip). From `gait_heading.rs`.
fn source_cook(path: &Path, rules: &ClipRules) -> (Skeleton, RigMap, Clip) {
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

/// A cooked clip over its loop: model-space frames measured, world frames with the root track
/// applied (identity when the root stays in the hips).
fn clip_row(label: String, skeleton: &Skeleton, body: &Body, clip: &Clip, torso: Option<&Torso>) -> Row {
    let (start, end) = clip.loop_range.unwrap_or((0.0, clip.duration));
    let ts = times(start, end);
    let (mut frames, mut world) = (Vec::new(), Vec::new());
    for &t in &ts {
        let m = sample(skeleton, clip, t);
        let p = pos(&m);
        let posed = torso.map(|t| t.posed(&m));
        frames.push(body.measure(&p, posed.as_deref()));
        let (rp, ry) = clip.root_at(t);
        world.push(p.iter().map(|v| Quat::from_rotation_y(ry) * *v + rp).collect());
    }
    Row { label, frames, world, dt: (end - start) / (ts.len() - 1).max(1) as f32, graph: None }
}

/// Builds the shipped humanoid graph's sim with the gait slots cooked the bake's way.
fn graph_sim(lib: &Lib) -> Option<(AnimSet, mw_graphs::sim::Sim)> {
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
    for (gait, _) in mw_graphs::build::gaits::SET {
        let name = manifest.gait_slot("humanoid", mw_graphs::build::gaits::STYLE, gait, "cycle").expect("shipped gait slot");
        let slot = &manifest.slots[&name];
        let c = &slot.candidates[0];
        let file = c.file.clone().expect("generated takes name their file");
        let Some(path) = library.real_path(&AnimSources::virtual_path(&c.source, &file)) else {
            eprintln!("{name}: not in the library; skipping the graph part");
            return None;
        };
        let mut clip = mw_bake::anim::cook_library_clip(&c.source, &path, Some(&c.clip), &slot.rules(c), &target).unwrap();
        clip.name = c.reference();
        set.insert_cooked(c.reference(), Arc::new(clip));
    }
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let clips = set.binding_clips(&v.graph).unwrap();
    let sim = mw_graphs::sim::Sim::new(v.graph, set.skeleton.clone(), clips).unwrap();
    Some((set, sim))
}

/// The graph straight ahead at `speed` on flat ground: 3 s settle, 4 s measured. `off`: op
/// gate params set to 0, and `"probes"` = no ground samples (foot placement off).
fn graph_row(label: String, base: &mw_graphs::sim::Sim, body: &Body, torso: Option<&Torso>, (speed, running, strafing): (f32, bool, bool), off: &[&str]) -> Row {
    use mw_graphs::build::params as p;
    let mut sim = mw_graphs::sim::Sim::new(base.graph.clone(), base.skeleton.clone(), base.clips.clone()).unwrap();
    sim.set(p::MOVING, 1.0);
    sim.set(p::RUNNING, if running { 1.0 } else { 0.0 });
    sim.set(p::STRAFING, if strafing { 1.0 } else { 0.0 });
    sim.set(p::SPEED, speed);
    sim.set_vec2(p::MOVE, glam::Vec2::Y);
    sim.velocity = Vec3::new(0.0, 0.0, speed);
    for gate in off.iter().filter(|g| **g != "probes") {
        sim.set(gate, 0.0);
    }
    let probes = !off.contains(&"probes");
    let dt = 1.0 / RATE;
    let (mut frames, mut world) = (Vec::new(), Vec::new());
    let (mut rates, mut natural) = (Vec::new(), Vec::new());
    let mut last: Option<(String, f32)> = None;
    let mut playing = Vec::new();
    for frame in 0..420 {
        sim.moved = sf_anim::RootDelta::new(Vec3::new(0.0, 0.0, speed * dt), 0.0);
        if probes {
            let origins = sim.out.probe_origins.clone();
            for (slot, o) in sim.ground.iter_mut().zip(origins.iter().chain(std::iter::repeat(&Vec3::ZERO))) {
                *slot = Some(sf_anim_graph::GroundSample { point: Vec3::new(o.x, 0.0, o.z), normal: Vec3::Y });
            }
        }
        sim.run(dt, dt);
        if frame < 180 {
            continue;
        }
        let m = model_aff(&sim.skeleton, &sim.out.pose.to_vec());
        let ps = pos(&m);
        let posed = torso.map(|t| t.posed(&m));
        frames.push(body.measure(&ps, posed.as_deref()));
        let z = speed * dt * (frame + 1) as f32;
        world.push(ps.iter().map(|v| *v + Vec3::new(0.0, 0.0, z)).collect());
        playing = sim.playing();
        if let Some((top, _)) = playing.first() {
            let t = sim.clip_time(top).unwrap_or(0.0);
            if let Some((c, t0)) = &last
                && c == top
                && t >= *t0
            {
                rates.push((t - t0) / dt);
            }
            last = Some((top.clone(), t));
            // Natural stride speed of the blend: lane-weighted stride speeds.
            let (mut s, mut w) = (0.0, 0.0);
            for (c, wt) in &playing {
                if let Some(i) = sim.graph.clips.iter().position(|x| &x.reference == c) {
                    let v = sim.clips[i].meta.stride_velocity();
                    s += glam::Vec2::new(v.x, v.z).length() * wt;
                    w += wt;
                }
            }
            natural.push(s / w.max(1e-6));
        }
    }
    rates.sort_by(f32::total_cmp);
    let rate = rates.get(rates.len() / 2).copied().unwrap_or(f32::NAN);
    let nat = mean(natural.into_iter());
    let raw = speed / (nat * rate).max(1e-3);
    let warp = sim.graph.ops.iter().find_map(|o| match o.op {
        sf_anim_graph::Op::StrideWarp { min, max, .. } => Some((min, max)),
        _ => None,
    });
    let clamped = warp.map_or(1.0, |(lo, hi)| raw.clamp(lo, hi));
    let playing: Vec<String> = playing.iter().map(|(c, w)| format!("{} {w:.2}", c.trim_start_matches("loco.").replace(".act.ground0", ""))).collect();
    Row { label, frames, world, dt, graph: Some((rate, raw, clamped, format!("nat {nat:.2} m/s | {}", playing.join(", ")))) }
}

/// Morrowind's own group cooked natively on the target (`generated_arms.rs` `native`).
fn native_clip(vfs: &Vfs, target: &HumanoidTarget, group: &str) -> Option<Clip> {
    let file = HUMANOID_SETS[0].animations[0];
    let anim = Animation::from_file(file, &vfs.read(&VfsPath::new(file).unwrap()).unwrap()).unwrap();
    let g = anim.groups().into_iter().find(|g| g.name.eq_ignore_ascii_case(group))?;
    let src = anim.source_clip(&g);
    Some(sf_anim::cook_clip_for(&target.skeleton, None, &src, &mw_anim::clip_rules(&g), &CookSettings::default()).unwrap())
}

#[test]
#[ignore = "measurement: AN1 shipped gait cycles: arms, trunk, bob, skate at source / retarget / graph"]
fn measure_gait_diagnosis() {
    let Some(lib) = lib() else { return };
    let tb = Body::new(&lib.target.skeleton, &lib.target.map);
    let torso = torso_mesh(&lib.vfs, &lib.target.skeleton);
    println!("target {}", tb.names(&lib.target.skeleton));
    println!(
        "columns: upper-arm swing vs trunk min..max deg (L, R) | clearance to chest mesh min cm el L/R wr L/R | lateral from mid-plane min cm (< 0 crosses) el L/R wr L/R | seg pitch vs bind (Chest>UpperChest, UpperChest>Neck) and hips>neck vs vertical deg (fwd +) | pelvis bob cm, stance skate cm/s, cadence steps/s"
    );
    let graph = graph_sim(&lib);
    if let Some((set, _)) = &graph {
        assert_eq!(set.skeleton.len(), lib.target.skeleton.len(), "graph skeleton = target skeleton");
    }
    for (gait, slot, file, (lo, hi), running) in GAITS {
        let path = lib.loco.join(file);
        if !path.exists() {
            eprintln!("{} missing", path.display());
            continue;
        }
        let rules = slot_rules(slot);
        println!("\n== {gait} ({})", path.file_stem().unwrap().to_string_lossy());
        let (sk, map, sclip) = source_cook(&path, &rules);
        let sb = Body::new(&sk, &map);
        if gait == "walk" {
            println!("source {}", sb.names(&sk));
        }
        let (start, end) = sclip.loop_range.unwrap_or((0.0, sclip.duration));
        let hips = sb.hips;
        let (h0, h1) = (sample(&sk, &sclip, start)[hips].translation, sample(&sk, &sclip, end)[hips].translation);
        let sspeed = Vec3::from(h1 - h0).with_y(0.0).length() / (end - start);
        let a = clip_row(format!("(a) src {sspeed:.2} m/s {:.2}s", end - start), &sk, &sb, &sclip, None);
        a.print(&sb);
        let (tclip, _) = cook_library_clip_report("generated", &path, None, &rules, &lib.target).unwrap();
        let (start, end) = tclip.loop_range.unwrap_or((0.0, tclip.duration));
        let (p0, _) = tclip.root_at(start);
        let (p1, _) = tclip.root_at(end);
        let b = clip_row(format!("(b) tgt {:.2} m/s {:.2}s", (p1 - p0).with_y(0.0).length() / (end - start), end - start), &lib.target.skeleton, &tb, &tclip, torso.as_ref());
        b.print(&tb);
        for f in &b.frames {
            assert!(f.trunk.is_finite() && f.swing.iter().all(|s| s.is_finite()), "{gait}: not finite");
        }
        if let Some((_, base)) = &graph {
            let speed = mw_graphs::build::gaits::SET.iter().find(|(g, _)| *g == gait).unwrap().1;
            for v in [lo, speed, hi] {
                graph_row(format!("(c) graph {v:.1}"), base, &tb, torso.as_ref(), (v, running, false), &[]).print(&tb);
            }
            // Ablation at the upper band edge (two lanes blend): which op moves the numbers.
            for off in [mw_graphs::build::params::STRIDE_WARP, mw_graphs::build::params::SPEED_MATCH, mw_graphs::build::params::LEAN, "probes"] {
                graph_row(format!("    {hi:.1} {off} off"), base, &tb, torso.as_ref(), (hi, running, false), &[off]).print(&tb);
            }
        }
    }
    for (group, running) in [("WalkForward", false), ("RunForward", true)] {
        let Some(clip) = native_clip(&lib.vfs, &lib.target, group) else {
            println!("\n== MW {group}: not found");
            continue;
        };
        let nat = clip.meta.stride_velocity().with_y(0.0).length();
        println!("\n== MW native {group} (stride speed {nat:.2} m/s)");
        clip_row("(d) MW native".into(), &lib.target.skeleton, &tb, &clip, torso.as_ref()).print(&tb);
        if let Some((_, base)) = &graph {
            graph_row(format!("(d) graph lock {nat:.1}"), base, &tb, torso.as_ref(), (nat, running, true), &[]).print(&tb);
        }
    }
}

// --- AN1b: which FootPlacement mechanism dips the pelvis at the walk/jog blend ------------

/// Reach of a leg's solved span (`procedural.rs` `reach`): thigh plus shin-to-toe for a
/// `TwoBoneAnkle` leg, else the chain length; times 0.999.
fn an1b_reach(p: &[Vec3], leg: &sf_anim_graph::Leg) -> f32 {
    let q = |j: u16| p[usize::from(j)];
    let s = leg.solved();
    let length = match (leg.solver, s) {
        (sf_anim::LimbSolver::TwoBoneAnkle, &[a, b, _, d]) => q(a).distance(q(b)) + q(b).distance(q(d)),
        _ => s.windows(2).map(|w| q(w[0]).distance(q(w[1]))).sum::<f32>(),
    };
    length * 0.999
}

/// Per foot of one graph update.
#[derive(Clone, Copy, Default)]
struct An1bFoot {
    /// Blended contact curve (graph output).
    contact: f32,
    /// Output foot still in the world (< 0.1 m/s horizontally): held by a lock.
    planted: bool,
    /// Output foot minus the FootPlacement-off twin's foot, horizontal (z +: ahead), m.
    off: Vec3,
    /// Output |hip -> foot| over the leg's reach (>= ~1: straight, reach-bound).
    straight: f32,
    /// `rise_cap` of the twin's hip for the output foot's plane spot: < 0 = drop reach asks, m.
    cap: f32,
    /// The twin's (animated) foot horizontal world speed, m/s (0 = no skate).
    anim_speed: f32,
    /// Each lane's (walk, jog) own contact curve at its clip time.
    lanes: [f32; 2],
    /// Blended heel-roll curve (graph output) and each lane's own.
    roll: f32,
    roll_lanes: [f32; 2],
    /// Output toe still in the world (< 0.1 m/s horizontally): pinned (lock or heel roll).
    toe_planted: bool,
}

struct An1bFrame {
    hip_ref: f32,
    hip_out: f32,
    feet: [An1bFoot; 2],
    weights: [f32; 2],
    /// Each lane's gait phase from its own left-contact onset, cycles.
    phases: [f32; 2],
    stats: sf_anim_graph::FootStats,
}

/// A lane clip's left/right contact onsets (first upward 0.5 crossing in the loop, s), its
/// duty factors and loop span.
fn an1b_onsets(clip: &Clip, curves: [Option<usize>; 2]) -> ([f32; 2], [f32; 2], (f32, f32)) {
    let (a, b) = clip.loop_range.unwrap_or((0.0, clip.duration));
    let ts = times(a, b);
    let mut on = [a; 2];
    let mut duty = [0.0; 2];
    for (k, c) in curves.iter().enumerate() {
        let Some(c) = *c else { continue };
        let v: Vec<f32> = ts.iter().map(|&t| clip.curve(c, t)).collect();
        #[allow(clippy::cast_precision_loss)]
        {
            duty[k] = v.iter().filter(|x| **x > 0.5).count() as f32 / v.len() as f32;
        }
        if let Some(i) = (1..v.len()).find(|&i| v[i - 1] <= 0.5 && v[i] > 0.5) {
            on[k] = ts[i];
        }
    }
    (on, duty, (a, b))
}

/// The graph straight ahead at `speed` (walk band) on flat ground, in lockstep with a twin
/// that gets no ground samples (`FootPlacement` weight 0: the animated + `StrideWarp` pose).
/// 3 s settle, 4 s traced. Returns the frames and the lane header line.
#[allow(clippy::too_many_lines, clippy::needless_range_loop, clippy::format_push_string)]
fn an1b_trace(base: &mw_graphs::sim::Sim, speed: f32) -> (Vec<An1bFrame>, String) {
    use mw_graphs::build::params as p;
    let mk = || {
        let mut sim = mw_graphs::sim::Sim::new(base.graph.clone(), base.skeleton.clone(), base.clips.clone()).unwrap();
        sim.set(p::MOVING, 1.0);
        sim.set(p::RUNNING, 0.0);
        sim.set(p::STRAFING, 0.0);
        sim.set(p::SPEED, speed);
        sim.set_vec2(p::MOVE, glam::Vec2::Y);
        sim.velocity = Vec3::new(0.0, 0.0, speed);
        sim
    };
    let (mut out, mut twin) = (mk(), mk());
    let g = base.graph.clone();
    let legs: Vec<sf_anim_graph::Leg> = {
        let mut l: Vec<_> = g.legs.iter().filter(|l| l.support).cloned().collect();
        // Left (+x in model space) first.
        let rest = pos(&model_aff(&base.skeleton, &LocalPose::rest(&base.skeleton).to_vec()));
        l.sort_by(|a, b| rest[usize::from(b.effector())].x.total_cmp(&rest[usize::from(a.effector())].x));
        l
    };
    assert_eq!(legs.len(), 2, "two support legs");
    let contact_names: Vec<Option<String>> = legs.iter().map(|l| l.contact.map(|c| g.curves[usize::from(c)].clone())).collect();
    let roll_names: Vec<Option<String>> = legs.iter().map(|l| l.roll.map(|c| g.curves[usize::from(c)].clone())).collect();
    let toe_of = |leg: &sf_anim_graph::Leg| usize::from(leg.toe.map_or(leg.effector(), |(j, _, _)| j));
    let hips = usize::from(legs[0].girdle);
    // The gait blend spaces (players with 3 lanes).
    let gait_nodes: Vec<(usize, Vec<u16>)> = g
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match &n.kind {
            sf_anim_graph::NodeKind::Player(pl) if pl.lanes.len() == 3 => Some((i, pl.lanes.clone())),
            _ => None,
        })
        .collect();
    let dt = 1.0 / RATE;
    let mut frames = Vec::new();
    let mut header = String::new();
    let mut prev: Option<([Vec3; 2], [Vec3; 2], [Vec3; 2])> = None;
    for frame in 0..420 {
        for (sim, probes) in [(&mut out, true), (&mut twin, false)] {
            sim.moved = sf_anim::RootDelta::new(Vec3::new(0.0, 0.0, speed * dt), 0.0);
            if probes {
                let origins = sim.out.probe_origins.clone();
                for (slot, o) in sim.ground.iter_mut().zip(origins.iter().chain(std::iter::repeat(&Vec3::ZERO))) {
                    *slot = Some(sf_anim_graph::GroundSample { point: Vec3::new(o.x, 0.0, o.z), normal: Vec3::Y });
                }
            }
            sim.run(dt, dt);
        }
        let po = pos(&model_aff(&out.skeleton, &out.out.pose.to_vec()));
        let pr = pos(&model_aff(&twin.skeleton, &twin.out.pose.to_vec()));
        #[allow(clippy::cast_precision_loss)]
        let z = Vec3::new(0.0, 0.0, speed * dt * (frame + 1) as f32);
        let wo = [po[usize::from(legs[0].effector())] + z, po[usize::from(legs[1].effector())] + z];
        let wr = [pr[usize::from(legs[0].effector())] + z, pr[usize::from(legs[1].effector())] + z];
        let wt = [po[toe_of(&legs[0])] + z, po[toe_of(&legs[1])] + z];
        let last = prev.replace((wo, wr, wt));
        if frame < 180 {
            continue;
        }
        // The active gait player and its lanes.
        let Some((lanes, clips)) = gait_nodes
            .iter()
            .filter_map(|(n, c)| out.instance.player(u32::try_from(*n).unwrap()).map(|(l, w)| (l, w, c)))
            .find(|(_, w, _)| *w > 0.5)
            .map(|(l, _, c)| (l, c.clone()))
        else {
            continue;
        };
        let lane_clip = |k: usize| &out.clips[usize::from(clips[k])];
        let lane_curve = |k: usize, f: usize| contact_names[f].as_ref().and_then(|n| lane_clip(k).curves.iter().position(|c| &c.name == n));
        if header.is_empty() {
            for k in 0..2 {
                let c = lane_clip(k);
                let (on, duty, (a, b)) = an1b_onsets(c, [lane_curve(k, 0), lane_curve(k, 1)]);
                let marks: Vec<String> = c.markers.iter().map(|m| format!("{}@{:.2}", m.name, (m.time - a) / (b - a))).collect();
                let rc = |ff: usize| roll_names[ff].as_ref().and_then(|n| c.curves.iter().position(|x| &x.name == n));
                let (ron, rduty, _) = an1b_onsets(c, [rc(0), rc(1)]);
                header += &format!(
                    "  lane {k} roll on L {:.2} R {:.2} (phase), roll duty L {:.2} R {:.2}\n",
                    (ron[0] - a) / (b - a),
                    (ron[1] - a) / (b - a),
                    rduty[0],
                    rduty[1]
                );
                header += &format!(
                    "  lane {k} {}: loop {:.2}s, contact on L {:.2} R {:.2} (phase), duty L {:.2} R {:.2}, markers [{}]\n",
                    g.clips[usize::from(clips[k])].reference,
                    b - a,
                    (on[0] - a) / (b - a),
                    (on[1] - a) / (b - a),
                    duty[0],
                    duty[1],
                    marks.join(" ")
                );
            }
        }
        let mut phases = [0.0; 2];
        let mut lane_contacts = [[0.0f32; 2]; 2];
        let mut lane_rolls = [[0.0f32; 2]; 2];
        for k in 0..2 {
            let c = lane_clip(k);
            let (on, _, (a, b)) = an1b_onsets(c, [lane_curve(k, 0), lane_curve(k, 1)]);
            phases[k] = ((lanes[k].0 - on[0]) / (b - a)).rem_euclid(1.0);
            for f in 0..2 {
                lane_contacts[f][k] = lane_curve(k, f).map_or(f32::NAN, |ci| c.curve(ci, lanes[k].0));
                lane_rolls[f][k] = roll_names[f].as_ref().and_then(|n| c.curves.iter().position(|x| &x.name == n)).map_or(f32::NAN, |ci| c.curve(ci, lanes[k].0));
            }
        }
        let mut feet = [An1bFoot::default(); 2];
        for (f, leg) in legs.iter().enumerate() {
            let e = usize::from(leg.effector());
            let hip_j = usize::from(leg.solved()[0]);
            let reach = an1b_reach(&po, leg);
            let off = (po[e] - pr[e]).with_y(0.0);
            let target = Vec3::new(po[e].x, pr[e].y, po[e].z);
            let hip = pr[hip_j];
            let r = an1b_reach(&pr, leg);
            let d = (target - hip).with_y(0.0).length();
            let cap = target.y - hip.y + if d < r { (r * r - d * d).sqrt() } else { 0.0 };
            let (vo, vr, vt) = last.map_or((0.0, 0.0, 0.0), |(lo, lr, lt)| {
                ((wo[f] - lo[f]).with_y(0.0).length() / dt, (wr[f] - lr[f]).with_y(0.0).length() / dt, (wt[f] - lt[f]).with_y(0.0).length() / dt)
            });
            feet[f] = An1bFoot {
                contact: leg.contact.map_or(f32::NAN, |c| out.out.curves[usize::from(c)]),
                planted: vo < 0.1,
                off,
                straight: po[e].distance(po[hip_j]) / reach,
                cap,
                anim_speed: vr,
                lanes: lane_contacts[f],
                roll: leg.roll.map_or(f32::NAN, |c| out.out.curves[usize::from(c)]),
                roll_lanes: lane_rolls[f],
                toe_planted: vt < 0.1,
            };
        }
        frames.push(An1bFrame { hip_ref: pr[hips].y, hip_out: po[hips].y, feet, weights: [lanes[0].1, lanes[1].1], phases, stats: out.out.feet });
    }
    (frames, header)
}

fn an1b_state(f: &An1bFoot) -> &'static str {
    if f.planted {
        "planted"
    } else if f.toe_planted {
        "toe-pin"
    } else if f.off.length() > 5e-3 {
        "releasing"
    } else {
        "free"
    }
}

#[test]
#[ignore = "measurement: AN1b graph pelvis dip at the walk/jog blend"]
#[allow(clippy::too_many_lines)]
fn measure_an1b_blend_dip() {
    let Some(lib) = lib() else { return };
    let Some((_, base)) = graph_sim(&lib) else { return };
    let cm = |m: f32| m * 100.0;
    let (frames, header) = an1b_trace(&base, 2.2);
    println!("AN1b @2.2 m/s, flat, per update (4 s). drop = twin hips - output hips (FootPlacement's pelvis drop)");
    print!("{header}");
    println!(
        "cols: t | hipRef hipOut drop cm | per foot L/R: blended contact [walk jog lane contacts] r blended roll [walk jog lane rolls] state (planted = ankle still, toe-pin = only toe still) off(x,z) cm straight cap cm animSpeed | w walk/jog | phase walk/jog, dphase | reachClamp capBound locks unplanted driftMax cm"
    );
    for (i, f) in frames.iter().enumerate() {
        let foot = |x: &An1bFoot| {
            format!(
                "{:.2} [{:.2} {:.2}] r {:.2} [{:.2} {:.2}] {:9} ({:+5.1},{:+5.1}) {:.3} {:+5.1} {:.2}",
                x.contact,
                x.lanes[0],
                x.lanes[1],
                x.roll,
                x.roll_lanes[0],
                x.roll_lanes[1],
                an1b_state(x),
                cm(x.off.x),
                cm(x.off.z),
                x.straight,
                cm(x.cap),
                x.anim_speed
            )
        };
        let dph = (f.phases[0] - f.phases[1] + 0.5).rem_euclid(1.0) - 0.5;
        #[allow(clippy::cast_precision_loss)]
        let t = i as f32 / RATE;
        println!(
            "{t:5.3} | {:6.1} {:6.1} {:+5.1} | L {} | R {} | w {:.2}/{:.2} | ph {:.3}/{:.3} {:+.3} | {} {} {} {} {:.1}",
            cm(f.hip_ref),
            cm(f.hip_out),
            cm(f.hip_ref - f.hip_out),
            foot(&f.feet[0]),
            foot(&f.feet[1]),
            f.weights[0],
            f.weights[1],
            f.phases[0],
            f.phases[1],
            dph,
            f.stats.reach_clamped,
            f.stats.cap_bound,
            f.stats.locks,
            f.stats.unplanted,
            cm(f.stats.drift_max)
        );
    }
    println!("\nAN1b sweep summary (walk band, flat): per speed");
    println!(
        "speed | bob out/ref cm | max drop cm @foot state off(x,z) cm straight cap cm | per-step bob min-max cm | reachClamp frames capBound frames unplants | |dphase| mean max | lane-contact disagree while blended>0.5 (L,R) | stance anim skate m/s mean (L,R) | w walk/jog"
    );
    for i in 0..8u8 {
        let speed = 1.6 + 0.2 * f32::from(i);
        let (fr, _) = an1b_trace(&base, speed);
        if fr.is_empty() {
            continue;
        }
        let span = |v: &mut dyn Iterator<Item = f32>| {
            let (lo, hi) = v.fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), x| (a.min(x), b.max(x)));
            hi - lo
        };
        let bob_out = span(&mut fr.iter().map(|f| f.hip_out));
        let bob_ref = span(&mut fr.iter().map(|f| f.hip_ref));
        let (mi, md) = fr.iter().enumerate().map(|(i, f)| (i, f.hip_ref - f.hip_out)).fold((0, f32::NEG_INFINITY), |a, b| if b.1 > a.1 { b } else { a });
        let at = &fr[mi];
        // The foot whose cap is lowest at the dip.
        let fi = usize::from(at.feet[1].cap < at.feet[0].cap);
        let foot = &at.feet[fi];
        // Steps: cut at each new plant of either foot.
        let mut cuts = vec![0];
        for k in 1..fr.len() {
            if (0..2).any(|f| fr[k].feet[f].planted && !fr[k - 1].feet[f].planted) {
                cuts.push(k);
            }
        }
        cuts.push(fr.len());
        let steps: Vec<f32> = cuts.windows(2).filter(|w| w[1] - w[0] > 3).map(|w| span(&mut fr[w[0]..w[1]].iter().map(|f| f.hip_out))).collect();
        let smin = steps.iter().copied().fold(f32::INFINITY, f32::min);
        let smax = steps.iter().copied().fold(0.0, f32::max);
        let rc = fr.iter().filter(|f| f.stats.reach_clamped > 0).count();
        let cb = fr.iter().filter(|f| f.stats.cap_bound > 0).count();
        let un: u32 = fr.iter().map(|f| f.stats.unplanted).sum();
        let dph: Vec<f32> = fr.iter().map(|f| ((f.phases[0] - f.phases[1] + 0.5).rem_euclid(1.0) - 0.5).abs()).collect();
        let dmean = mean(dph.iter().copied());
        let dmax = dph.iter().copied().fold(0.0, f32::max);
        let disagree = |f: usize| {
            let held: Vec<&An1bFrame> = fr.iter().filter(|x| x.feet[f].contact > 0.5).collect();
            let n = held.iter().filter(|x| (x.feet[f].lanes[0] > 0.5) != (x.feet[f].lanes[1] > 0.5)).count();
            #[allow(clippy::cast_precision_loss)]
            {
                n as f32 / held.len().max(1) as f32
            }
        };
        let skate = |f: usize| mean(fr.iter().filter(|x| x.feet[f].contact > 0.5).map(|x| x.feet[f].anim_speed));
        // Toe pin after the contact (heel roll): frames per step, largest offset, and the
        // share of those frames whose jog lane already has both curves off (its toe lifted).
        let pin = |f: usize| {
            let p: Vec<&An1bFoot> = fr.iter().map(|x| &x.feet[f]).filter(|x| x.contact <= 0.5 && x.toe_planted && !x.planted).collect();
            let jog_off = p.iter().filter(|x| x.lanes[1] <= 0.5 && x.roll_lanes[1] <= 0.5).count();
            let off = p.iter().map(|x| x.off.length()).fold(0.0, f32::max);
            #[allow(clippy::cast_precision_loss)]
            (p.len() as f32 / (steps.len().max(2) as f32 / 2.0), off, jog_off as f32 / p.len().max(1) as f32)
        };
        let (pl, pr) = (pin(0), pin(1));
        println!(
            "{speed:.1} | {:.1}/{:.1} | {:.1} @{} {} ({:+.1},{:+.1}) {:.3} {:+.1} | {:.1}-{:.1} | {rc} {cb} {un} | {:.3} {:.3} | {:.0}% {:.0}% | {:.2} {:.2} | {:.2}/{:.2}",
            cm(bob_out),
            cm(bob_ref),
            cm(md),
            ["L", "R"][fi],
            an1b_state(foot),
            cm(foot.off.x),
            cm(foot.off.z),
            foot.straight,
            cm(foot.cap),
            cm(smin),
            cm(smax),
            dmean,
            dmax,
            100.0 * disagree(0),
            100.0 * disagree(1),
            skate(0),
            skate(1),
            at.weights[0],
            at.weights[1]
        );
        println!(
            "      toe-pin after contact: frames/step L {:.1} R {:.1}, max offset cm L {:.1} R {:.1}, with jog lane toe already off L {:.0}% R {:.0}%",
            pl.0,
            pr.0,
            cm(pl.1),
            cm(pr.1),
            100.0 * pl.2,
            100.0 * pr.2
        );
        assert!(fr.iter().all(|f| f.hip_out.is_finite()), "{speed}: non-finite hips");
    }
}

// --- AN5b: every looping loco2 take, (a) / (b) columns, ranked against Morrowind ----------

/// One stage's summary: per-arm swing (min, max) deg, bob cm, stance skate cm/s, steps in the
/// cut loop, cadence steps/s, root speed m/s, loop s.
#[derive(Clone, Copy)]
struct An5bStats {
    swing: [(f32, f32); 2],
    bob: f32,
    skate: f32,
    steps: usize,
    cad: f32,
    speed: f32,
    loop_s: f32,
    /// The loop cut found a loop (else the whole take plays).
    cut: bool,
}

impl An5bStats {
    fn of(row: &Row, body: &Body, speed: f32) -> Self {
        let fr = &row.frames;
        let sw = |i: usize| (min(fr.iter().map(|f| f.swing[i])), max(fr.iter().map(|f| f.swing[i])));
        let (bob, _, skate, _) = row.legs(body);
        // Stance switches over the loop (lower foot, 1 cm hysteresis, as `Row::legs`).
        let w = &row.world;
        let (l, r) = body.feet;
        let fj = [l, r];
        let mut stance = usize::from(w[0][r].y < w[0][l].y);
        let mut steps = 0;
        for p in w {
            let other = 1 - stance;
            if p[fj[other]].y + 0.01 < p[fj[stance]].y {
                stance = other;
                steps += 1;
            }
        }
        let loop_s = row.dt * (w.len() - 1) as f32;
        Self { swing: [sw(0), sw(1)], bob, skate, steps, cad: steps as f32 / loop_s.max(1e-3), speed, loop_s, cut: true }
    }

    fn cols(&self) -> String {
        let [l, r] = self.swing;
        format!(
            "L {:>5.1}..{:>5.1} R {:>5.1}..{:>5.1} bob {:>4.1} skate {:>5.1} n{} cad {:>4.2} {:>4.2} m/s {:>4.2}s{}",
            l.0,
            l.1,
            r.0,
            r.1,
            self.bob,
            self.skate,
            self.steps,
            self.cad,
            self.speed,
            self.loop_s,
            if self.cut { "" } else { " UNCUT" }
        )
    }

    /// The ranking score vs a Morrowind reference row (module docs).
    fn score(&self, mw: &Self) -> f32 {
        let arm = |i: usize| (self.swing[i].0 - mw.swing[i].0).abs() + (self.swing[i].1 - mw.swing[i].1).abs();
        f32::midpoint(arm(0), arm(1)) + 5.0 * (self.bob - mw.bob).abs() + 0.5 * self.skate
    }
}

/// One measured take.
struct An5bTake {
    /// `loco2.<gait>.<kind>.<variant>`.
    group: String,
    /// Take number (`.NN` of `... .act.ground0.NN`).
    take: String,
    gait: &'static str,
    dir: &'static str,
    gates: Vec<String>,
    grade_skate: f32,
    stages: Result<(An5bStats, An5bStats), String>,
}

/// (gait, direction) of a looping loco2 group folder, `None` for starts / stops / pivots / turns.
fn an5b_kind(folder: &str) -> Option<(&'static str, &'static str)> {
    let mut it = folder.strip_prefix("loco2.")?.split('.');
    let gait = match it.next()? {
        "walk" => "walk",
        "jog" => "jog",
        "sprint" => "sprint",
        _ => return None,
    };
    let dir = match (it.next()?, it.next()) {
        ("cycle", _) => "forward",
        ("back", _) => "back",
        ("strafe", Some("l")) => "left",
        _ => return None,
    };
    Some((gait, dir))
}

/// Morrowind's native group for a (gait, direction).
fn an5b_mw_group(gait: &str, dir: &str) -> String {
    let pace = if gait == "walk" { "Walk" } else { "Run" };
    let d = match dir {
        "forward" => "Forward",
        "back" => "Back",
        _ => "Left",
    };
    format!("{pace}{d}")
}

/// The take cooked at source and onto the target, the bake's way (`measure_gait_diagnosis`).
fn an5b_stages(lib: &Lib, tb: &Body, path: &Path, rules: &ClipRules) -> Result<(An5bStats, An5bStats), String> {
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (sk, map, sclip) = source_cook(path, rules);
        let sb = Body::new(&sk, &map);
        let (start, end) = sclip.loop_range.unwrap_or((0.0, sclip.duration));
        let (h0, h1) = (sample(&sk, &sclip, start)[sb.hips].translation, sample(&sk, &sclip, end)[sb.hips].translation);
        let sspeed = Vec3::from(h1 - h0).with_y(0.0).length() / (end - start);
        let a = An5bStats { cut: sclip.loop_range.is_some(), ..An5bStats::of(&clip_row(String::new(), &sk, &sb, &sclip, None), &sb, sspeed) };
        let (tclip, _) = cook_library_clip_report("generated", path, None, rules, &lib.target)?;
        let (start, end) = tclip.loop_range.unwrap_or((0.0, tclip.duration));
        let ((p0, _), (p1, _)) = (tclip.root_at(start), tclip.root_at(end));
        let tspeed = (p1 - p0).with_y(0.0).length() / (end - start);
        let b = An5bStats { cut: tclip.loop_range.is_some(), ..An5bStats::of(&clip_row(String::new(), &lib.target.skeleton, tb, &tclip, None), tb, tspeed) };
        Ok((a, b))
    }));
    caught.unwrap_or_else(|e| Err(e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_owned())).unwrap_or_else(|| "panic".into())))
}

#[test]
#[ignore = "measurement: AN5b per-take loco2 cycles (arms, bob, skate, cadence, speed) ranked vs Morrowind"]
fn measure_an5b_loco2_takes() {
    let Some(lib) = lib_at("loco2") else { return };
    let tb = Body::new(&lib.target.skeleton, &lib.target.map);
    let grades: serde_json::Value =
        std::fs::read_to_string(lib.loco.join("grades.json")).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    let mut folders: Vec<String> = std::fs::read_dir(&lib.loco)
        .unwrap()
        .filter_map(|e| Some(e.ok()?.file_name().to_string_lossy().into_owned()))
        .filter(|f| an5b_kind(f).is_some())
        .collect();
    folders.sort();
    println!(
        "columns: take | (a) src, (b) tgt: upper-arm swing vs trunk min..max deg (L, R) | pelvis bob cm | stance skate cm/s | n steps in the cut loop, cadence steps/s | root speed m/s | loop s || grade: skate cm/s, gates"
    );

    // Morrowind references per (gait, direction).
    let mut refs = std::collections::BTreeMap::new();
    println!("\n== MW native references (target rig)");
    for g in ["WalkForward", "WalkBack", "WalkLeft", "RunForward", "RunBack", "RunLeft"] {
        let stats = native_clip(&lib.vfs, &lib.target, g).map(|clip| {
            let speed = clip.meta.stride_velocity().with_y(0.0).length();
            An5bStats::of(&clip_row(String::new(), &lib.target.skeleton, &tb, &clip, None), &tb, speed)
        });
        println!("{g:<12} | {}", stats.map_or_else(|| "not found".into(), |s| s.cols()));
        refs.insert(g.to_owned(), stats);
    }

    let mut takes = Vec::new();
    for folder in &folders {
        let (gait, dir) = an5b_kind(folder).unwrap();
        let slot = format!("loco.{gait}.cycle");
        let rules = slot_rules(&slot);
        let mut files: Vec<PathBuf> = std::fs::read_dir(lib.loco.join(folder).join("act"))
            .unwrap()
            .filter_map(|e| Some(e.ok()?.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "bvh"))
            .collect();
        files.sort();
        println!("\n== {folder} ({gait} {dir}, {slot} rules)");
        for path in files {
            let id = path.file_stem().unwrap().to_string_lossy().into_owned();
            let entry = &grades[&id];
            let gates: Vec<String> =
                entry["gates"].as_array().map(|a| a.iter().filter_map(|g| g.as_str().map(str::to_owned)).collect()).unwrap_or_default();
            #[allow(clippy::cast_possible_truncation, reason = "a grade in cm/s")]
            let grade_skate = entry["metrics"]["skate_cm_s"].as_f64().map_or(f32::NAN, |v| v as f32);
            let stages = an5b_stages(&lib, &tb, &path, &rules);
            let take = id.rsplit('.').next().unwrap_or("?").to_owned();
            let grade = format!("{grade_skate:>5.1} {}", if gates.is_empty() { "-".to_owned() } else { format!("GATED {}", gates.join(",")) });
            match &stages {
                Ok((a, b)) => {
                    for (i, s) in b.swing.iter().enumerate() {
                        assert!(s.0.is_finite() && s.1.is_finite(), "{id}: arm {i} swing not finite");
                    }
                    println!("{take} (a) src | {} || {grade}", a.cols());
                    println!("{take} (b) tgt | {}", b.cols());
                }
                Err(e) => println!("{take} cook failed: {e} || {grade}"),
            }
            takes.push(An5bTake { group: folder.clone(), take, gait, dir, gates, grade_skate, stages });
        }
    }

    // Per variant: means of the (b) row over its cooked takes.
    println!("\n== per variant, (b) tgt means: swing L/R deg (min..max) | bob | skate | cad | speed | gated/total");
    for folder in &folders {
        let of: Vec<&An5bTake> = takes.iter().filter(|t| &t.group == folder).collect();
        let ok: Vec<&An5bStats> = of.iter().filter_map(|t| t.stages.as_ref().ok().map(|s| &s.1)).collect();
        let gated = of.iter().filter(|t| !t.gates.is_empty()).count();
        let m = |f: &dyn Fn(&An5bStats) -> f32| mean(ok.iter().map(|s| f(s)));
        let failed = if ok.len() < of.len() { format!(" ({} cook failed)", of.len() - ok.len()) } else { String::new() };
        println!(
            "{folder:<28} | L {:>5.1}..{:>5.1} R {:>5.1}..{:>5.1} | bob {:>4.1} | skate {:>5.1} | cad {:>4.2} | {:>4.2} m/s | {gated}/{}{failed}",
            m(&|s| s.swing[0].0),
            m(&|s| s.swing[0].1),
            m(&|s| s.swing[1].0),
            m(&|s| s.swing[1].1),
            m(&|s| s.bob),
            m(&|s| s.skate),
            m(&|s| s.cad),
            m(&|s| s.speed),
            of.len(),
        );
    }

    // Ranked short list per (gait, direction): ungated, loop-cut takes, (b) row vs the MW group;
    // forward gaits also within the graph's speed band for that gait (`GAITS`).
    println!("\n== ranked (b) tgt vs MW, top 2 ungated + cut per (gait, direction); score = arm swing err deg + 5 x bob err cm + 0.5 x skate cm/s");
    for gait in ["walk", "jog", "sprint"] {
        for dir in ["forward", "back", "left"] {
            let group = an5b_mw_group(gait, dir);
            let Some(Some(mw)) = refs.get(&group) else { continue };
            let mut ranked: Vec<(f32, &An5bTake, &An5bStats)> = takes
                .iter()
                .filter(|t| t.gait == gait && t.dir == dir && t.gates.is_empty())
                .filter_map(|t| t.stages.as_ref().ok().filter(|s| s.1.cut).map(|s| (s.1.score(mw), t, &s.1)))
                .collect();
            if ranked.is_empty() {
                continue;
            }
            ranked.sort_by(|a, b| a.0.total_cmp(&b.0));
            println!("-- {gait} {dir} (vs MW {group}, {} ungated cut takes)", ranked.len());
            let line = |(score, t, s): &(f32, &An5bTake, &An5bStats)| {
                let name = format!("{}.{}", t.group.trim_start_matches("loco2."), t.take);
                println!("{score:>6.1} {name:<26} | {} | grade skate {:>5.1}", s.cols(), t.grade_skate);
            };
            ranked.iter().take(2).for_each(line);
            if dir == "forward"
                && let Some((_, _, _, (lo, hi), _)) = GAITS.iter().find(|g| g.0 == gait)
            {
                let band: Vec<_> = ranked.iter().filter(|(_, _, s)| (*lo..=*hi).contains(&s.speed)).collect();
                println!("   within the {gait} band {lo:.1}..{hi:.1} m/s: {} takes", band.len());
                band.into_iter().take(2).for_each(line);
            }
        }
    }
}
