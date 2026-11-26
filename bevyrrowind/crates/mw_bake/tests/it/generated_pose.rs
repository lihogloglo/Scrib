//! Slouch and jaw on retargeted library clips (owner 2026-11-12,
//! `clip-preview --browse generated/loco1`): diagnosis only.
//!
//! Per take (Kimodo SOMA-77 loco1 BVH, and two UAL1 loops), the trunk's
//! sagittal posture in the source and on the Morrowind humanoid it is
//! retargeted onto ([`sf_anim::Retargeter`] with `HumanoidTarget::rig`, the
//! bake's and `clip-preview`'s path): per segment of the hips + spine + neck
//! chain, the pitch from vertical (forward positive, facing from the thighs),
//! the head's look pitch (down positive) and the clavicle's elevation and
//! protraction, as the mean over the middle half of the clip, each next to
//! the rig's own rest (bind) value. A rest-frame check retargets the source
//! rest pose (Kimodo: the BVH T-pose, zero rotations) and prints, per target
//! joint, how far its segment ends up from the Morrowind bind pose: the
//! posture offset every frame of that source carries. Then the jaw: which
//! target joint the humanoid map gives `Jaw`, `Head` and `Neck`, and the
//! source Jaw's rotation amplitude. Run with `-- --ignored --nocapture`;
//! skips without a Morrowind install or the files.
#![allow(clippy::similar_names, clippy::items_after_statements, clippy::cast_precision_loss)]

use std::path::{Path, PathBuf};

use glam::{Quat, Vec3};
use mw_bake::anim::{HumanoidTarget, MwAnimRules, library_rig};
use mw_bake::sources::AnimSources;
use mw_graphs::HUMANOID_SETS;
use mw_install::MorrowindInstall;
use sf_anim::{Clip, ClipRules, CookSettings, LocalPose, Looping, RetargetRig, RigHints, RootMotionMode, RootMotionRules, Skeleton, Trs};
use sf_character::{HumanoidBone as B, RigMap};
use sf_vfs::{Vfs, VfsPath};

const RATE: f32 = 30.0;

struct Take {
    label: String,
    source: &'static str,
    path: PathBuf,
    clip: Option<String>,
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

fn takes() -> Vec<Take> {
    let mut out = Vec::new();
    let loco = AnimSources::local().dir.join("generated").join("loco1");
    for (dir, file) in [
        ("loco.walk.cycle.1", "loco.walk.cycle.1.act.ground0.00.bvh"),
        ("loco.walk.cycle.free", "loco.walk.cycle.free.act.ground0.00.bvh"),
        ("loco.run.cycle.3", "loco.run.cycle.3.act.ground0.04.bvh"),
        ("loco.walk.stop.lf.1", "loco.walk.stop.lf.1.act.ground0.00.bvh"),
        ("loco.sprint.cycle.free", "loco.sprint.cycle.free.act.ground0.00.bvh"),
    ] {
        let path = loco.join(dir).join("act").join(file);
        if path.exists() {
            out.push(Take { label: path.file_stem().unwrap().to_string_lossy().into_owned(), source: "generated", path, clip: None });
        } else {
            eprintln!("{} missing", path.display());
        }
    }
    let ual = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/animation/quaternius/UAL1_RM.glb");
    for clip in ["Walk_Loop", "Jog_Fwd_Loop", "Idle_Loop"] {
        out.push(Take { label: format!("UAL1:{clip}"), source: "quaternius_ual1", path: ual.clone(), clip: Some(clip.into()) });
    }
    out
}

fn model_rots(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Quat> {
    let mut out: Vec<Quat> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(Quat::IDENTITY, |p| out[p]);
        out.push((parent * t.rotation).normalize());
    }
    out
}

fn model_pos(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Vec3> {
    let mut m: Vec<glam::Affine3A> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(glam::Affine3A::IDENTITY, |p| m[p]);
        m.push(parent * t.to_affine());
    }
    m.iter().map(|a| Vec3::from(a.translation)).collect()
}

fn sample(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Trs> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    pose.to_vec()
}

fn angle_deg(a: Quat, b: Quat) -> f32 {
    (a.inverse() * b).normalize().w.abs().min(1.0).acos().to_degrees() * 2.0
}

fn joint(skeleton: &Skeleton, map: &RigMap, b: B) -> Option<usize> {
    map.joint(b).and_then(|j| skeleton.find(j))
}

/// Segment pitches, head look pitch, clavicle (elevation, protraction).
type Meas = (Vec<f32>, f32, Vec<(f32, f32)>);

/// A rig's posture joints: hips, the `spine` and `neck` retarget chains,
/// head; the clavicles and upper arms.
struct Posture {
    /// Hips, spine.., neck.., head (each segment runs to the next one).
    trunk: Vec<usize>,
    head: usize,
    /// (left, right) upper legs: the facing.
    thighs: (usize, usize),
    /// (shoulder, upper arm) per side, when the shoulder is mapped.
    clav: Vec<(usize, usize)>,
    /// Forward in the head joint's local frame (from the rest pose).
    head_fwd_local: Vec3,
}

impl Posture {
    fn new(skeleton: &Skeleton, map: &RigMap, rig: &RetargetRig) -> Self {
        let need = |b: B| joint(skeleton, map, b).unwrap_or_else(|| panic!("{b:?} unmapped"));
        let chain = |name: &str| -> Vec<usize> {
            rig.chains.iter().find(|c| c.name == name).map(|c| c.joints.iter().filter_map(|n| skeleton.find(n)).collect()).unwrap_or_default()
        };
        let head = need(B::Head);
        let mut trunk = vec![need(B::Hips)];
        trunk.extend(chain("spine"));
        trunk.extend(chain("neck"));
        trunk.push(head);
        trunk.dedup();
        let thighs = (need(B::LeftUpperLeg), need(B::RightUpperLeg));
        let mut clav = Vec::new();
        for (s, a) in [(B::LeftShoulder, B::LeftUpperArm), (B::RightShoulder, B::RightUpperArm)] {
            if let (Some(s), Some(a)) = (joint(skeleton, map, s), joint(skeleton, map, a)) {
                clav.push((s, a));
            }
        }
        let rest = skeleton.rest().to_vec();
        let (pos, rot) = (model_pos(skeleton, &rest), model_rots(skeleton, &rest));
        let mut p = Self { trunk, head, thighs, clav, head_fwd_local: Vec3::Z };
        let (_, fwd) = p.frame(&pos);
        p.head_fwd_local = rot[head].inverse() * fwd;
        p
    }

    /// (left, forward), horizontal, from the thighs; up is +Y.
    fn frame(&self, pos: &[Vec3]) -> (Vec3, Vec3) {
        let mut left = pos[self.thighs.0] - pos[self.thighs.1];
        left.y = 0.0;
        let left = left.normalize_or(Vec3::X);
        (left, left.cross(Vec3::Y))
    }

    /// Segment pitches (deg, forward +), head look pitch (down +), and per
    /// clavicle (elevation, protraction) in degrees.
    fn measure(&self, skeleton: &Skeleton, pose: &[Trs]) -> Meas {
        let pos = model_pos(skeleton, pose);
        let rot = model_rots(skeleton, pose);
        let (left, fwd) = self.frame(&pos);
        let pitch = |v: Vec3| v.dot(fwd).atan2(v.dot(Vec3::Y)).to_degrees();
        let segs = self.trunk.windows(2).map(|w| pitch(pos[w[1]] - pos[w[0]])).collect();
        let hf = (rot[self.head] * self.head_fwd_local).normalize_or_zero();
        let look = (-hf.y).clamp(-1.0, 1.0).asin().to_degrees();
        let clav = self
            .clav
            .iter()
            .map(|&(s, a)| {
                let v = (pos[a] - pos[s]).normalize_or_zero();
                (v.y.clamp(-1.0, 1.0).asin().to_degrees(), v.dot(fwd).atan2(v.dot(left).abs()).to_degrees())
            })
            .collect();
        (segs, look, clav)
    }

    fn names(&self, skeleton: &Skeleton) -> String {
        self.trunk.windows(2).map(|w| format!("{}>{}", short(skeleton.name(w[0])), short(skeleton.name(w[1])))).collect::<Vec<_>>().join(" | ")
    }
}

fn short(n: &str) -> String {
    n.trim_start_matches("Bip01 ").replace(' ', "")
}

fn fmt(v: &[f32]) -> String {
    v.iter().map(|x| format!("{x:>6.1}")).collect::<Vec<_>>().join(" ")
}

fn fmt_clav(v: &[(f32, f32)]) -> String {
    v.iter().map(|(e, p)| format!("elev {e:>5.1} prot {p:>5.1}")).collect::<Vec<_>>().join(" / ")
}

/// Mean of per-frame measurements over the middle half of the clip.
fn mean_mid(rows: &[Meas]) -> Meas {
    let n = rows.len();
    let mid = &rows[n / 4..(3 * n / 4).max(n / 4 + 1)];
    let k = mid.len() as f32;
    let segs = (0..mid[0].0.len()).map(|i| mid.iter().map(|r| r.0[i]).sum::<f32>() / k).collect();
    let look = mid.iter().map(|r| r.1).sum::<f32>() / k;
    let clav = (0..mid[0].2.len()).map(|i| (mid.iter().map(|r| r.2[i].0).sum::<f32>() / k, mid.iter().map(|r| r.2[i].1).sum::<f32>() / k)).collect();
    (segs, look, clav)
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn frame_times(duration: f32) -> Vec<f32> {
    let n = (duration * RATE).ceil().max(1.0) as usize;
    (0..=n).map(|i| (i as f32 / RATE).min(duration)).collect()
}

/// The retarget pose (local) of a skeleton (`sf_anim::retarget`'s private one).
fn retarget_pose(skeleton: &Skeleton, rig: &RetargetRig) -> Vec<Trs> {
    let mut pose = skeleton.rest().to_vec();
    for (name, q) in &rig.retarget_pose {
        pose[skeleton.find(name).unwrap()].rotation = q.normalize();
    }
    pose
}

fn print_map(label: &str, skeleton: &Skeleton, map: &RigMap, rig: &RetargetRig) {
    let bones = [B::Hips, B::Spine, B::Chest, B::UpperChest, B::Neck, B::Head, B::Jaw, B::LeftEye, B::LeftShoulder];
    let row: Vec<String> = bones.iter().map(|b| format!("{b:?}={}", map.joint(*b).unwrap_or("-"))).collect();
    println!("  {label} map: {}", row.join(", "));
    for c in rig.chains.iter().filter(|c| ["hips", "spine", "neck", "head", "jaw", "left_shoulder"].contains(&c.name.as_str())) {
        println!("    chain {:<14} {:?}", c.name, c.joints);
    }
    // Retarget pose vs rest per posture joint (model rotation, deg).
    let rest = skeleton.rest().to_vec();
    let rp = retarget_pose(skeleton, rig);
    let (mr, mp) = (model_rots(skeleton, &rest), model_rots(skeleton, &rp));
    let mut top = joint(skeleton, map, B::Hips).unwrap();
    while let Some(p) = skeleton.parent(top) {
        top = p;
    }
    let facing = angle_deg(mr[top], mp[top]);
    let row: Vec<String> = bones
        .iter()
        .filter_map(|b| joint(skeleton, map, *b).map(|j| format!("{}:{:.1}", short(skeleton.name(j)), angle_deg(mp[top] * mr[top].inverse() * mr[j], mp[j]))))
        .collect();
    println!("    align (retarget pose vs facing-corrected rest, deg; facing {facing:.1}): {}", row.join(" "));
}

#[allow(clippy::too_many_lines)]
fn measure(take: &Take, target: &HumanoidTarget, tpost: &Posture, first: &mut bool) -> Result<(), String> {
    let manifest = mw_graphs::sources::SourcesManifest::builtin();
    let library = &manifest.sources[take.source];
    let name = take.path.file_name().unwrap().to_string_lossy().into_owned();
    let virtual_path = AnimSources::virtual_path(take.source, &name);
    let importers = mw_bake::anim::anim_importers();
    let bytes = std::fs::read(&take.path).map_err(|e| e.to_string())?;
    let imported = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).map_err(|e| e.to_string())?;
    let joints_in = imported.skeleton.as_ref().ok_or("no skeleton")?;
    let mut skeleton_rules = MwAnimRules::skeleton_rules();
    skeleton_rules.root_motion_joint.clone_from(&imported.root_motion);
    let (skeleton, _) = Skeleton::with_rules(joints_in, &skeleton_rules).map_err(|e| e.to_string())?;
    let (map, source_rig) = library_rig(&skeleton, take.source).ok_or("no humanoid rig")?;
    let hints = RigHints { profile: map.profile(&skeleton).ok(), ..RigHints::default() };
    let profile = sf_anim::detect_profile(&skeleton, &hints).map_err(|e| e.to_string())?;
    let rules = rules();
    let mut cook_rules = rules.clone();
    if library.root_from_pelvis {
        cook_rules.root_motion.extract = false;
        cook_rules.root_motion.anchor_end = false;
    }
    let src = match &take.clip {
        Some(c) => imported.clips.iter().find(|x| &x.name == c),
        None => imported.clips.first(),
    }
    .ok_or("no clip")?;
    let cooked = sf_anim::cook_clip_for(&skeleton, Some(&profile), src, &cook_rules, &CookSettings::default()).map_err(|e| e.to_string())?;
    let r = sf_anim::Retargeter::new(&skeleton, &source_rig, &target.skeleton, &target.rig).map_err(|e| e.to_string())?;
    let spost = Posture::new(&skeleton, &map, &source_rig);

    let first_of_source = take.label.starts_with("UAL1:Walk") || *first;
    println!("\n== {} ({:.2} s)", take.label, cooked.duration);
    if first_of_source {
        *first = false;
        print_map("source", &skeleton, &map, &source_rig);
        println!("  source trunk segments: {}", spost.names(&skeleton));
        // Pairs of the trunk chains.
        for p in r.pairs() {
            if tpost.trunk.contains(&p.target) {
                println!(
                    "    pair {:<14} <- {} / {} alpha {:.2}",
                    short(target.skeleton.name(p.target)),
                    skeleton.name(p.source.0),
                    skeleton.name(p.source.1),
                    p.alpha
                );
            }
        }
        // Rest-frame check: the source rest pose retargeted, against the
        // target bind (rest) and retarget pose.
        let mut out = vec![Trs::IDENTITY; target.skeleton.len()];
        let weights = vec![0.0; 2];
        r.pose(skeleton.rest(), &weights, &mut out);
        let s_rest = spost.measure(&skeleton, skeleton.rest());
        let s_rp = spost.measure(&skeleton, &retarget_pose(&skeleton, &source_rig));
        let t_rest = tpost.measure(&target.skeleton, target.skeleton.rest());
        let t_rp = tpost.measure(&target.skeleton, r.target_retarget_pose());
        let t_of_rest = tpost.measure(&target.skeleton, &out);
        println!("  rest-frame check (segment pitch deg, forward +; look down +; clavicle elev/prot):");
        println!("    source rest           {} | look {:>5.1} | {}", fmt(&s_rest.0), s_rest.1, fmt_clav(&s_rest.2));
        println!("    source retarget pose  {} | look {:>5.1} | {}", fmt(&s_rp.0), s_rp.1, fmt_clav(&s_rp.2));
        println!("    target bind (rest)    {} | look {:>5.1} | {}", fmt(&t_rest.0), t_rest.1, fmt_clav(&t_rest.2));
        println!("    target retarget pose  {} | look {:>5.1} | {}", fmt(&t_rp.0), t_rp.1, fmt_clav(&t_rp.2));
        println!("    source rest -> target {} | look {:>5.1} | {}", fmt(&t_of_rest.0), t_of_rest.1, fmt_clav(&t_of_rest.2));
        let off: Vec<f32> = t_of_rest.0.iter().zip(&t_rest.0).map(|(a, b)| a - b).collect();
        println!("    offset vs target bind {} | look {:>5.1}", fmt(&off), t_of_rest.1 - t_rest.1);
        let rot_out = model_rots(&target.skeleton, &out);
        let rot_rp = model_rots(&target.skeleton, r.target_retarget_pose());
        let row: Vec<String> =
            tpost.trunk.iter().map(|&j| format!("{}:{:.2}", short(target.skeleton.name(j)), angle_deg(rot_out[j], rot_rp[j]))).collect();
        println!("    (source rest -> target) vs target retarget pose, model deg: {}", row.join(" "));
        // Source retarget pose -> target: lands exactly on the target retarget pose?
        let mut out2 = vec![Trs::IDENTITY; target.skeleton.len()];
        r.pose(&retarget_pose(&skeleton, &source_rig), &weights, &mut out2);
        let rot2 = model_rots(&target.skeleton, &out2);
        let worst = tpost.trunk.iter().map(|&j| angle_deg(rot2[j], rot_rp[j])).fold(0.0f32, f32::max);
        println!("    source retarget pose -> target: max trunk residual vs target retarget pose {worst:.3} deg");
    }

    // Per frame, source and target.
    let times = frame_times(cooked.duration);
    let mut s_rows = Vec::new();
    let mut t_rows = Vec::new();
    let mut out = vec![Trs::IDENTITY; target.skeleton.len()];
    for &t in &times {
        let sp = sample(&skeleton, &cooked, t);
        s_rows.push(spost.measure(&skeleton, &sp));
        r.pose(&sp, &[1.0, 1.0], &mut out);
        t_rows.push(tpost.measure(&target.skeleton, &out));
    }
    let (s, t) = (mean_mid(&s_rows), mean_mid(&t_rows));
    let s_rest = spost.measure(&skeleton, skeleton.rest());
    let t_rest = tpost.measure(&target.skeleton, target.skeleton.rest());
    let ds: Vec<f32> = s.0.iter().zip(&s_rest.0).map(|(a, b)| a - b).collect();
    let dt: Vec<f32> = t.0.iter().zip(&t_rest.0).map(|(a, b)| a - b).collect();
    let total = |post: &Posture, sk: &Skeleton, pose: &[Trs]| {
        let pos = model_pos(sk, pose);
        let (_, fwd) = post.frame(&pos);
        let v = pos[*post.trunk.last().unwrap()] - pos[post.trunk[0]];
        v.dot(fwd).atan2(v.y).to_degrees()
    };
    let mid_t = times[times.len() / 2];
    let sp = sample(&skeleton, &cooked, mid_t);
    r.pose(&sp, &[1.0, 1.0], &mut out);
    println!("  mid-half mean (segment pitch deg, forward +):");
    println!("    source  {} | look {:>5.1} | {}", fmt(&s.0), s.1, fmt_clav(&s.2));
    println!("    - rest  {} | look {:>5.1}", fmt(&ds), s.1 - s_rest.1);
    println!("    target  {} | look {:>5.1} | {}", fmt(&t.0), t.1, fmt_clav(&t.2));
    println!("    - bind  {} | look {:>5.1}", fmt(&dt), t.1 - t_rest.1);
    println!(
        "    hips->head overall at mid frame: source {:.1}, target {:.1}",
        total(&spost, &skeleton, &sp),
        total(tpost, &target.skeleton, &out)
    );

    // Jaw.
    if let Some(jaw) = joint(&skeleton, &map, B::Jaw) {
        let rest = skeleton.rest()[jaw].rotation;
        let amp: Vec<f32> = times.iter().map(|&t| angle_deg(rest, sample(&skeleton, &cooked, t)[jaw].rotation)).collect();
        let steps: Vec<f32> = times
            .windows(2)
            .map(|w| angle_deg(sample(&skeleton, &cooked, w[0])[jaw].rotation, sample(&skeleton, &cooked, w[1])[jaw].rotation))
            .collect();
        let max = amp.iter().copied().fold(0.0f32, f32::max);
        let mean = amp.iter().sum::<f32>() / amp.len() as f32;
        let smax = steps.iter().copied().fold(0.0f32, f32::max);
        println!("  source jaw {}: angle from rest mean {mean:.2} max {max:.2} deg; largest step {smax:.2} deg/frame", skeleton.name(jaw));
    }
    let head_steps: Vec<f32> = times
        .windows(2)
        .map(|w| {
            r.pose(&sample(&skeleton, &cooked, w[0]), &[1.0, 1.0], &mut out);
            let a = out[tpost.head].rotation;
            r.pose(&sample(&skeleton, &cooked, w[1]), &[1.0, 1.0], &mut out);
            angle_deg(a, out[tpost.head].rotation)
        })
        .collect();
    let mut hs = head_steps.clone();
    hs.sort_by(f32::total_cmp);
    println!(
        "  target head local rotation step: median {:.2} max {:.2} deg/frame",
        hs.get(hs.len() / 2).copied().unwrap_or(0.0),
        hs.last().copied().unwrap_or(0.0)
    );
    Ok(())
}

#[test]
#[ignore = "measurement: trunk posture (per spine segment pitch, head look, clavicles) source vs retargeted, rest-frame offset, jaw mapping"]
fn measure_generated_pose() {
    let Some(vfs) = install_vfs() else { return };
    let set = &HUMANOID_SETS[0];
    let bytes = vfs.read(&VfsPath::new(set.skeleton).unwrap()).unwrap();
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
    let target = HumanoidTarget::new(skeleton).unwrap();
    let tpost = Posture::new(&target.skeleton, &target.map, &target.rig);
    println!("== target {}", set.skeleton);
    print_map("target", &target.skeleton, &target.map, &target.rig);
    println!("  target trunk segments: {}", tpost.names(&target.skeleton));
    let jawish: Vec<&str> = (0..target.skeleton.len())
        .map(|j| target.skeleton.name(j))
        .filter(|n| {
            let l = n.to_ascii_lowercase();
            l.contains("jaw") || l.contains("mouth") || l.contains("head") || l.contains("neck")
        })
        .collect();
    println!("  target joints named jaw/mouth/head/neck: {jawish:?}");
    let mut first = true;
    for take in takes() {
        if let Err(e) = measure(&take, &target, &tpost, &mut first) {
            println!("\n== {}: failed: {e}", take.label);
        }
    }
}
