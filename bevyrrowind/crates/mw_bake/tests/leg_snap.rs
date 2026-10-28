//! AN-22 (leg snap) diagnosis: sudden leg-bone rotations in library clips
//! retargeted onto `base_anim`, each attributed to the source clip, our
//! cook, our retarget (FK pairs or leg IK) or the loop seam. Measurement
//! only (`-- --ignored --nocapture`); no fix.
//!
//! Every take is sampled at 30 Hz in four layers: the raw source clip on its
//! own skeleton ([`BoundSource`]), the cooked source clip, and the clip
//! retargeted onto `base_anim` with leg IK (the bake's and `clip-preview`'s
//! path, [`cook_library_clip`]) and without it. Takes: every BVH under
//! `<library>/generated/tier1/` and every clip of the Quaternius
//! `UAL1_RM.glb` / `UAL2_RM.glb`, cooked with `clip-preview`'s rules.
//! Skips (passes) without a Morrowind install or the files.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use glam::{Quat, Vec3};
use mw_bake::anim::{self, HumanoidTarget, MwAnimRules, cook_library_clip, library_rig};
use mw_bake::sources::AnimSources;
use mw_graphs::HUMANOID_SETS;
use mw_install::MorrowindInstall;
use sf_anim::{
    BoundSource, Clip, ClipRules, CookSettings, LocalPose, Looping, RigHints, RootMotionMode, RootMotionRules, Skeleton, SourceClip, Trs,
};
use sf_character::{HumanoidBone as B, RetargetOptions, RigMap};
use sf_vfs::{Vfs, VfsPath};

const RATE: f32 = 30.0;
/// A step over this in one 1/30 s frame is a snap.
const ABS_DEG: f32 = 25.0;
/// Or over this many times the bone's median step, and over [`REL_MIN_DEG`].
const REL_FACTOR: f32 = 3.0;
const REL_MIN_DEG: f32 = 10.0;
/// A layer upstream "spikes too" when it meets the snap rule or moves at
/// least this share of the target's step (within one frame).
const SHARE: f32 = 0.5;
const LEGS: [B; 8] =
    [B::LeftUpperLeg, B::LeftLowerLeg, B::LeftFoot, B::LeftToes, B::RightUpperLeg, B::RightLowerLeg, B::RightFoot, B::RightToes];

struct Take {
    label: String,
    source: &'static str,
    path: PathBuf,
    clip: Option<String>,
    looping: bool,
}

/// One layer's leg-bone local rotations: `[bone][frame]` (`None`: unmapped).
type Rots = Vec<Option<Vec<Quat>>>;

struct Layers {
    raw: Rots,
    cooked: Rots,
    ik: Rots,
    fk: Rots,
    /// Each target leg bone's axis in its own frame (towards its child).
    axis: Vec<Vec3>,
}

struct Flag {
    take: String,
    bone: B,
    /// The step `frame - 1 → frame`; `None`: the loop seam (last → first).
    frame: Option<usize>,
    deg: f32,
    twist: f32,
    cause: &'static str,
    upstream_deg: f32,
}

fn rules() -> ClipRules {
    // `clip-preview`'s (`mw_viewers_bevy::clip_preview::cook::preview_rules`).
    ClipRules { looping: Looping::Once, root_motion: RootMotionRules { mode: RootMotionMode::Drive, ..RootMotionRules::default() }, ..ClipRules::default() }
}

fn install_vfs() -> Option<Vfs> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    Some(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs)
}

fn bvh_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            bvh_files(&p, out);
        } else if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bvh")) {
            out.push(p);
        }
    }
}

fn takes() -> Vec<Take> {
    let mut out = Vec::new();
    let tier1 = AnimSources::local().dir.join("generated").join("tier1");
    let mut files = Vec::new();
    bvh_files(&tier1, &mut files);
    if files.is_empty() {
        eprintln!("{}: no BVH files; skipping the generated takes", tier1.display());
    }
    for path in files {
        let label = path.file_stem().unwrap().to_string_lossy().into_owned();
        let looping = path.parent().and_then(Path::file_name).is_some_and(|d| d == "loop");
        out.push(Take { label, source: "generated", path, clip: None, looping });
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/animation/quaternius");
    let importers = anim::anim_importers();
    for (source, file) in [("quaternius_ual1", "UAL1_RM.glb"), ("quaternius_ual2", "UAL2_RM.glb")] {
        let path = dir.join(file);
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("{} missing; skipping", path.display());
            continue;
        };
        let virtual_path = AnimSources::virtual_path(source, file);
        let imported = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).unwrap();
        for c in &imported.clips {
            let looping = c.name.to_ascii_lowercase().contains("loop");
            out.push(Take { label: format!("{file}:{}", c.name), source, path: path.clone(), clip: Some(c.name.clone()), looping });
        }
    }
    out
}

/// Frame times at 30 Hz from 0, the last on the clip's end.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn frame_times(duration: f32) -> Vec<f32> {
    let n = (duration * RATE).ceil().max(1.0) as usize;
    (0..=n).map(|i| (i as f32 / RATE).min(duration)).collect()
}

fn joints(skeleton: &Skeleton, map: &RigMap) -> Vec<Option<usize>> {
    LEGS.iter().map(|b| map.joint(*b).and_then(|j| skeleton.find(j))).collect()
}

fn gather(joints: &[Option<usize>], frames: &[Vec<Trs>]) -> Rots {
    joints.iter().map(|j| j.map(|j| frames.iter().map(|f| f[j].rotation).collect())).collect()
}

fn sample_clip(skeleton: &Skeleton, clip: &Clip, times: &[f32]) -> Vec<Vec<Trs>> {
    times
        .iter()
        .map(|&t| {
            let mut pose = LocalPose::rest(skeleton);
            clip.sample(t, &mut pose, None);
            pose.to_vec()
        })
        .collect()
}

fn sample_source(skeleton: &Skeleton, clip: &SourceClip, times: &[f32]) -> Vec<Vec<Trs>> {
    let bound = BoundSource::new(clip, skeleton);
    times
        .iter()
        .map(|&t| {
            let mut out = skeleton.rest().to_vec();
            bound.sample(t, skeleton.rest(), &mut out);
            out
        })
        .collect()
}

/// The bone's axis in its own frame: towards its farthest direct child.
fn bone_axis(skeleton: &Skeleton, joint: Option<usize>) -> Vec3 {
    let Some(j) = joint else { return Vec3::Y };
    (0..skeleton.len())
        .filter(|&k| skeleton.parent(k) == Some(j))
        .map(|k| skeleton.rest()[k].translation)
        .max_by(|a, b| a.length_squared().total_cmp(&b.length_squared()))
        .filter(|v| v.length_squared() > 1e-10)
        .map_or_else(|| skeleton.rest()[j].translation.normalize_or(Vec3::Y), Vec3::normalize)
}

/// The whole take cooked as `cook_library_clip` cooks it, keeping the
/// intermediate layers, plus the retarget without leg IK.
fn layers(take: &Take, target: &HumanoidTarget, fk_rig: &sf_anim::RetargetRig, check: bool) -> Result<Layers, String> {
    let manifest = mw_graphs::sources::SourcesManifest::builtin();
    let library = &manifest.sources[take.source];
    let name = take.path.file_name().unwrap().to_string_lossy().into_owned();
    let virtual_path = AnimSources::virtual_path(take.source, &name);
    let importers = anim::anim_importers();
    let bytes = std::fs::read(&take.path).map_err(|e| e.to_string())?;
    let imported = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).map_err(|e| e.to_string())?;
    let joints_in = imported.skeleton.as_ref().ok_or("no skeleton")?;
    let mut skeleton_rules = MwAnimRules::skeleton_rules();
    skeleton_rules.root_motion_joint.clone_from(&imported.root_motion);
    let (skeleton, _) = Skeleton::with_rules(joints_in, &skeleton_rules).map_err(|e| e.to_string())?;
    let (map, source_rig) = library_rig(&skeleton).ok_or("no humanoid rig")?;
    let hints = RigHints { profile: map.profile(&skeleton).ok(), ..RigHints::default() };
    let profile = sf_anim::detect_profile(&skeleton, &hints).map_err(|e| e.to_string())?;
    let rules = rules();
    let retarget_root = library.root_from_pelvis.then(|| rules.root_motion.clone());
    let mut cook_rules = rules.clone();
    if retarget_root.is_some() {
        cook_rules.root_motion.extract = false;
        cook_rules.root_motion.anchor_end = false;
    }
    let src = match &take.clip {
        Some(c) => imported.clips.iter().find(|x| &x.name == c),
        None => imported.clips.first(),
    }
    .ok_or("no clip")?;
    let settings = CookSettings::default();
    let cooked = sf_anim::cook_clip_for(&skeleton, Some(&profile), src, &cook_rules, &settings).map_err(|e| e.to_string())?;
    let retarget = |rig: &sf_anim::RetargetRig| -> Result<Clip, String> {
        let r = sf_anim::Retargeter::new(&skeleton, &source_rig, &target.skeleton, rig).map_err(|e| e.to_string())?;
        match &retarget_root {
            Some(root) => r.clip_with_root(&cooked, root, &settings),
            None => r.clip(&cooked, &settings),
        }
        .map_err(|e| e.to_string())
    };
    let ik = retarget(&target.rig)?;
    let fk = retarget(fk_rig)?;
    if check {
        // This file's IK layer is the bake's clip (`cook_library_clip`).
        let mut ours = ik.clone();
        sf_anim::transfer_contacts(&cooked.meta.contacts, &target.skeleton, &target.profile, &mut ours);
        let mut baked = cook_library_clip(take.source, &take.path, take.clip.as_deref(), &rules, target)?;
        baked.name.clone_from(&ours.name);
        assert!(baked.to_bytes() == ours.to_bytes(), "{}: the replicated cook differs from cook_library_clip", take.label);
    }
    let times = frame_times(ik.duration);
    let src_joints = joints(&skeleton, &map);
    let tgt_joints = joints(&target.skeleton, &target.map);
    Ok(Layers {
        raw: gather(&src_joints, &sample_source(&skeleton, src, &times)),
        cooked: gather(&src_joints, &sample_clip(&skeleton, &cooked, &times)),
        ik: gather(&tgt_joints, &sample_clip(&target.skeleton, &ik, &times)),
        fk: gather(&tgt_joints, &sample_clip(&target.skeleton, &fk, &times)),
        axis: tgt_joints.iter().map(|j| bone_axis(&target.skeleton, *j)).collect(),
    })
}

fn angle_deg(a: Quat, b: Quat) -> f32 {
    (a.inverse() * b).normalize().w.abs().min(1.0).acos().to_degrees() * 2.0
}

fn steps(q: &[Quat]) -> Vec<f32> {
    q.windows(2).map(|w| angle_deg(w[0], w[1])).collect()
}

fn median(v: &[f32]) -> f32 {
    let mut s = v.to_vec();
    s.sort_by(f32::total_cmp);
    s.get(s.len() / 2).copied().unwrap_or(0.0)
}

fn snaps(step: f32, median: f32) -> bool {
    step > ABS_DEG || (step > REL_FACTOR * median && step > REL_MIN_DEG)
}

/// The twist share (about `axis`) of the rotation `a → b`, in `[0, 1]`.
fn twist_share(from: Quat, to: Quat, axis: Vec3) -> f32 {
    let mut delta = (from.inverse() * to).normalize();
    if delta.w < 0.0 {
        delta = -delta;
    }
    let along = axis * Vec3::new(delta.x, delta.y, delta.z).dot(axis);
    let twist = Quat::from_xyzw(along.x, along.y, along.z, delta.w);
    let twist = if twist.length_squared() < 1e-12 { Quat::IDENTITY } else { twist.normalize() };
    let swing = delta * twist.inverse();
    let (twist_deg, swing_deg) = (angle_deg(Quat::IDENTITY, twist), angle_deg(Quat::IDENTITY, swing));
    if twist_deg + swing_deg < 1e-6 { 0.0 } else { twist_deg / (twist_deg + swing_deg) }
}

/// Whether `layer` spikes too near step `i` (steps `i - 1 ..= i + 1`):
/// meets the snap rule, or moves at least [`SHARE`] of `deg`. Returns its
/// largest step there.
fn upstream(layer: &[f32], i: usize, deg: f32) -> (bool, f32) {
    let m = median(layer);
    let lo = i.saturating_sub(1);
    let hi = (i + 1).min(layer.len().saturating_sub(1));
    let s = layer.get(lo..=hi).map_or(0.0, |w| w.iter().copied().fold(0.0, f32::max));
    (snaps(s, m) || s >= SHARE * deg, s)
}

fn flags(take: &Take, l: &Layers) -> Vec<Flag> {
    let mut out = Vec::new();
    for (b, bone) in LEGS.iter().enumerate() {
        let Some(ik) = &l.ik[b] else { continue };
        let ik_steps = steps(ik);
        let ik_median = median(&ik_steps);
        let layer = |r: &Rots| r[b].as_ref().map(|q| steps(q));
        let (raw, cooked, fk) = (layer(&l.raw), layer(&l.cooked), layer(&l.fk));
        for (i, &deg) in ik_steps.iter().enumerate() {
            if !snaps(deg, ik_median) {
                continue;
            }
            let check = |s: &Option<Vec<f32>>| s.as_ref().map_or((false, 0.0), |s| upstream(s, i, deg));
            let (r, c, f) = (check(&raw), check(&cooked), check(&fk));
            let (cause, up) = if r.0 {
                ("source", r.1)
            } else if c.0 {
                ("cook", c.1)
            } else if f.0 {
                ("retarget-fk", f.1)
            } else {
                ("retarget-ik", f.1)
            };
            let twist = twist_share(ik[i], ik[i + 1], l.axis[b]);
            out.push(Flag { take: take.label.clone(), bone: *bone, frame: Some(i + 1), deg, twist, cause, upstream_deg: up });
        }
        if take.looping && ik.len() > 2 {
            let (first, last) = (ik[0], ik[ik.len() - 1]);
            let deg = angle_deg(last, first);
            if snaps(deg, ik_median) {
                let raw_seam = l.raw[b].as_ref().map_or(0.0, |q| angle_deg(q[q.len() - 1], q[0]));
                let cause = if raw_seam >= SHARE * deg { "seam (source)" } else { "seam (ours)" };
                let twist = twist_share(last, first, l.axis[b]);
                out.push(Flag { take: take.label.clone(), bone: *bone, frame: None, deg, twist, cause, upstream_deg: raw_seam });
            }
        }
    }
    out
}

#[test]
#[ignore = "measurement: AN-22 leg snaps in generated and Quaternius clips on base_anim, attributed to source, cook, retarget or loop seam"]
#[allow(clippy::cast_precision_loss)]
fn measure_leg_snaps() {
    let Some(vfs) = install_vfs() else { return };
    let takes = takes();
    if takes.is_empty() {
        eprintln!("no takes; skipping");
        return;
    }
    let set = &HUMANOID_SETS[0];
    let bytes = vfs.read(&VfsPath::new(set.skeleton).unwrap()).unwrap();
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
    let target = HumanoidTarget::new(skeleton).unwrap();
    let fk_rig = target.map.retarget_rig(&target.skeleton, RetargetOptions { ik_legs: false }).unwrap();

    let mut all = Vec::new();
    let (mut cooked, mut failed, mut loops) = (0usize, Vec::new(), 0usize);
    let mut affected: BTreeMap<&'static str, std::collections::BTreeSet<String>> = BTreeMap::new();
    let mut any = std::collections::BTreeSet::new();
    for (i, take) in takes.iter().enumerate() {
        match layers(take, &target, &fk_rig, i == 0) {
            Ok(l) => {
                cooked += 1;
                loops += usize::from(take.looping);
                for f in flags(take, &l) {
                    affected.entry(f.cause).or_default().insert(f.take.clone());
                    any.insert(f.take.clone());
                    all.push(f);
                }
            }
            Err(e) => failed.push(format!("{}: {e}", take.label)),
        }
    }
    all.sort_by(|a, b| b.deg.total_cmp(&a.deg));
    println!("\nAN-22 leg snaps on {} (30 Hz; > {ABS_DEG}° per frame, or > {REL_FACTOR}x median and > {REL_MIN_DEG}°)", set.skeleton);
    println!("{:<52} {:<14} {:>6} {:>7} {:>6} {:>8}  cause", "take", "bone", "frame", "deg", "twist", "upstr°");
    let row = |f: &Flag| {
        let frame = f.frame.map_or_else(|| "seam".to_owned(), |i| i.to_string());
        println!(
            "{:<52} {:<14} {:>6} {:>7.1} {:>5.0}% {:>8.1}  {}",
            f.take,
            format!("{:?}", f.bone),
            frame,
            f.deg,
            f.twist * 100.0,
            f.upstream_deg,
            f.cause
        );
    };
    all.iter().take(30).for_each(row);
    // Ours only (the source is calm there): what a fix would remove.
    println!("\nTop 15 where the source is calm (cook, retarget, seam (ours)):");
    all.iter().filter(|f| !matches!(f.cause, "source" | "seam (source)")).take(15).for_each(row);
    println!("\nSummary: {cooked} takes cooked ({loops} looping), {} failed", failed.len());
    for (cause, takes) in &affected {
        let n = all.iter().filter(|f| f.cause == *cause).count();
        let twist = all.iter().filter(|f| f.cause == *cause).map(|f| f.twist).sum::<f32>() / n.max(1) as f32;
        println!("  {cause:<14} {n:>5} flags in {:>4} takes, mean twist share {:.0}%", takes.len(), twist * 100.0);
    }
    for (library, prefix) in [("generated", false), ("quaternius", true)] {
        let of = |f: &&Flag| f.take.starts_with("UAL") == prefix;
        let per: BTreeMap<&str, usize> = all.iter().filter(of).fold(BTreeMap::new(), |mut m, f| {
            *m.entry(f.cause).or_default() += 1;
            m
        });
        let takes: std::collections::BTreeSet<&str> = all.iter().filter(of).map(|f| f.take.as_str()).collect();
        println!("  {library}: {per:?} in {} takes", takes.len());
    }
    println!("  any cause      {:>5} flags in {:>4} of {cooked} takes ({:.0}%)", all.len(), any.len(), 100.0 * any.len() as f32 / cooked.max(1) as f32);
    for e in failed.iter().take(10) {
        println!("  failed: {e}");
    }
}
