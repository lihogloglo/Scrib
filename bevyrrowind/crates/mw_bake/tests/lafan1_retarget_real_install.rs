//! LAFAN1 get-ups retargeted onto `base_anim` (animation-sources.md "LAFAN1"):
//! the bones must turn as the source's do, twist (roll about the bone) included.
//!
//! The reference is a pose both sides physically share: every LAFAN1 take
//! starts in the actor's T-pose (frame 0), and the target's retarget pose is
//! its auto-aligned T-pose. Per mapped bone and frame, the source's rotation
//! away from its T-pose (`qS(f) · qS(0)⁻¹`) is compared with the target's
//! (`qT(f) · Rt⁻¹`): the angle between them, and its twist about the bone.
//! Independent of that reference: knees must bend away from the kneecap and
//! elbows toward the inner elbow (the target T-pose's +Z, carried by the
//! thigh / upper arm), and feet stand sole down at the get-up's end.
//!
//! Sanity asserts only (rule 5); the numbers come from the ignored
//! measurement (`-- --ignored --nocapture`). `LAFAN1_RETARGET_DUMP=<dir>`
//! also writes per-bone boxes (source and target) at the start, middle and
//! end of each clip as JSON for a Blender render.
//! Skips (passes) without a Morrowind install or the LAFAN1 files.

use std::fmt::Write as _;

use glam::{Affine3A, Quat, Vec3};
use mw_bake::sources::AnimSources;
use mw_install::MorrowindInstall;
use sf_anim::{AnimSource, BoundSource, Retargeter, Skeleton, SkeletonRules, Trs};
use sf_character::{HumanoidBone as B, RetargetOptions, RigMap};
use sf_vfs::{Vfs, VfsPath};

const TARGET: &str = "meshes/base_anim.nif";
const GET_UPS: [&str; 4] = ["getup.supine", "getup.prone", "getup.side_left", "getup.side_right"];
/// UAL2 (rest = a real T-pose): the regression reference.
const UAL2: (&str, &str) = ("quaternius_ual2", "ClimbUp_1m_RM");

/// A bone shown and measured: its humanoid bone, the next bone (its
/// direction), its canonical T-pose direction and "up" (flat face / kneecap).
struct Seg {
    bone: B,
    child: Option<B>,
    dir: Vec3,
    up: Vec3,
}

fn segs() -> Vec<Seg> {
    let s = |bone, child, dir, up| Seg { bone, child, dir, up };
    let (x, y, z) = (Vec3::X, Vec3::Y, Vec3::Z);
    vec![
        s(B::Hips, Some(B::Spine), y, z),
        s(B::Spine, Some(B::Chest), y, z),
        s(B::Chest, Some(B::Neck), y, z),
        s(B::Neck, Some(B::Head), y, z),
        s(B::Head, None, y, z),
        s(B::LeftUpperLeg, Some(B::LeftLowerLeg), -y, z),
        s(B::LeftLowerLeg, Some(B::LeftFoot), -y, z),
        s(B::LeftFoot, Some(B::LeftToes), z, y),
        s(B::RightUpperLeg, Some(B::RightLowerLeg), -y, z),
        s(B::RightLowerLeg, Some(B::RightFoot), -y, z),
        s(B::RightFoot, Some(B::RightToes), z, y),
        s(B::LeftUpperArm, Some(B::LeftLowerArm), x, y),
        s(B::LeftLowerArm, Some(B::LeftHand), x, y),
        s(B::LeftHand, None, x, y),
        s(B::RightUpperArm, Some(B::RightLowerArm), -x, y),
        s(B::RightLowerArm, Some(B::RightHand), -x, y),
        s(B::RightHand, None, -x, y),
    ]
}

struct Side {
    skeleton: Skeleton,
    map: RigMap,
}

impl Side {
    fn joint(&self, b: B) -> Option<usize> {
        self.map.joint(b).and_then(|n| self.skeleton.find(n))
    }

    /// The joint after `b` toward `child` (the foot's first child when the
    /// toes are unmapped, as Morrowind's `Bip01 L Toe0`).
    fn child_joint(&self, b: B, child: Option<B>) -> Option<usize> {
        child.and_then(|c| self.joint(c)).or_else(|| {
            let j = self.joint(b)?;
            matches!(b, B::LeftFoot | B::RightFoot).then(|| (0..self.skeleton.len()).find(|&k| self.skeleton.parent(k) == Some(j)))?
        })
    }
}

fn model_of(skeleton: &Skeleton, local: &[Trs]) -> Vec<Affine3A> {
    let mut out: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        out.push(skeleton.parent(j).map_or(m, |p| out[p] * m));
    }
    out
}

fn rot(m: &Affine3A) -> Quat {
    m.to_scale_rotation_translation().1.normalize()
}

fn pos(m: &Affine3A) -> Vec3 {
    Vec3::from(m.translation)
}

/// Twist of `q` about `axis`, radians in [0, π].
fn twist(q: Quat, axis: Vec3) -> f32 {
    let p = Vec3::new(q.x, q.y, q.z).dot(axis);
    let t = Quat::from_xyzw(axis.x * p, axis.y * p, axis.z * p, q.w);
    if t.length_squared() < 1e-12 { std::f32::consts::PI } else { t.normalize().angle_between(Quat::IDENTITY) }
}

struct Rig {
    source: Side,
    target: Side,
    retargeter: Retargeter,
    /// Target retarget-pose model rotations.
    target_ref: Vec<Quat>,
    report: String,
}

fn install() -> Option<Vfs> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    Some(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs)
}

fn target_side(vfs: &Vfs) -> (Side, sf_anim::RetargetRig) {
    let bytes = vfs.read(&VfsPath::new(TARGET).unwrap()).unwrap();
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
    let map = mw_anim::rig::humanoid(&skeleton).unwrap().map;
    let rig = map.retarget_rig(&skeleton, RetargetOptions::default()).unwrap();
    (Side { skeleton, map }, rig)
}

/// Imports a library file the way the bake does (its source's space and
/// reference frame) and builds the bake's source rig.
fn import(sources: &AnimSources, source: &str, file: &str) -> Option<AnimSource> {
    let path = AnimSources::virtual_path(source, file);
    let real = sources.real_path(&path)?;
    let importers = mw_bake::anim::anim_importers();
    let importer = importers.for_path(&path).expect("an importer for the library file");
    Some(importer.import(&path, &std::fs::read(real).unwrap()).unwrap())
}

fn rig(src: &AnimSource, target: (Side, sf_anim::RetargetRig)) -> Rig {
    let rules = SkeletonRules { root_motion_joint: src.root_motion.clone(), ..SkeletonRules::default() };
    let (skeleton, _) = Skeleton::with_rules(src.skeleton.as_ref().unwrap(), &rules).unwrap();
    let (map, source_rig) = mw_bake::anim::library_rig(&skeleton).expect("a humanoid source rig");
    let (target, target_rig) = target;
    let retargeter = Retargeter::new(&skeleton, &source_rig, &target.skeleton, &target_rig).unwrap();
    let target_ref = model_of(&target.skeleton, retargeter.target_retarget_pose()).iter().map(rot).collect();
    let (_, align) = sf_character::auto_align(&skeleton, &map);
    let report = format!("source auto-align: facing {:.1}°, {} joints rotated", align.facing_correction_deg, align.rotated.len());
    Rig { source: Side { skeleton, map }, target, retargeter, target_ref, report }
}

/// Per-bone stats over a clip.
#[derive(Default)]
struct Stats {
    /// Per segment: (sum, max) of the rotation error and of its twist, degrees.
    rot: Vec<(f32, f32)>,
    tw: Vec<(f32, f32)>,
    frames: usize,
    /// (bent frames, wrong side) for left knee, right knee, left elbow, right elbow.
    bends: [(usize, usize); 4],
    /// Foot sole "up" · world up at the last frame (left, right).
    sole_end: [f32; 2],
}

struct Sampled {
    s_model: Vec<Affine3A>,
    t_model: Vec<Affine3A>,
}

fn sample(rig: &Rig, bound: &BoundSource, t: f32) -> Sampled {
    let (src, tgt) = (&rig.source.skeleton, &rig.target.skeleton);
    let mut s_local = vec![Trs::IDENTITY; src.len()];
    let mut t_local = vec![Trs::IDENTITY; tgt.len()];
    bound.sample(t, src.rest(), &mut s_local);
    rig.retargeter.pose(&s_local, &[], &mut t_local);
    Sampled { s_model: model_of(src, &s_local), t_model: model_of(tgt, &t_local) }
}

/// The source's and target's rotation away from the shared reference pose.
fn deltas(rig: &Rig, s_ref: &[Quat], seg: &Seg, at: &Sampled) -> Option<(Quat, Quat)> {
    let (s, t) = (rig.source.joint(seg.bone)?, rig.target.joint(seg.bone)?);
    Some((rot(&at.s_model[s]) * s_ref[s].inverse(), rot(&at.t_model[t]) * rig.target_ref[t].inverse()))
}

#[allow(clippy::many_single_char_names)]
fn measure(rig: &Rig, s_ref: &[Quat], bound: &BoundSource, range: (f32, f32)) -> Stats {
    let segs = segs();
    let mut st = Stats { rot: vec![(0.0, 0.0); segs.len()], tw: vec![(0.0, 0.0); segs.len()], ..Stats::default() };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((range.1 - range.0) * 30.0).round().max(1.0) as usize;
    for f in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = range.0 + (range.1 - range.0) * f as f32 / n as f32;
        let at = sample(rig, bound, t);
        assert!(at.t_model.iter().all(Affine3A::is_finite), "non-finite target pose at {t}");
        for (i, seg) in segs.iter().enumerate() {
            let Some((a, b)) = deltas(rig, s_ref, seg, &at) else { continue };
            let e = b * a.inverse();
            let axis = (b * seg.dir).normalize();
            let (deg, tw) = (e.angle_between(Quat::IDENTITY).to_degrees(), twist(e, axis).to_degrees());
            st.rot[i] = (st.rot[i].0 + deg, st.rot[i].1.max(deg));
            st.tw[i] = (st.tw[i].0 + tw, st.tw[i].1.max(tw));
        }
        // Anatomy of the target alone: the kneecap / inner elbow is its T-pose +Z.
        let tgt = &rig.target;
        for (k, (upper, lower, end, sign)) in [
            (B::LeftUpperLeg, B::LeftLowerLeg, B::LeftFoot, -1.0),
            (B::RightUpperLeg, B::RightLowerLeg, B::RightFoot, -1.0),
            (B::LeftUpperArm, B::LeftLowerArm, B::LeftHand, 1.0),
            (B::RightUpperArm, B::RightLowerArm, B::RightHand, 1.0),
        ]
        .into_iter()
        .enumerate()
        {
            let (Some(u), Some(l), Some(e)) = (tgt.joint(upper), tgt.joint(lower), tgt.joint(end)) else { continue };
            let m = &at.t_model;
            let d1 = (pos(&m[l]) - pos(&m[u])).normalize();
            let d2 = (pos(&m[e]) - pos(&m[l])).normalize();
            if d1.angle_between(d2) < 30f32.to_radians() {
                continue;
            }
            let front = rot(&m[u]) * rig.target_ref[u].inverse() * Vec3::Z;
            let bend = d2 - d1 * d1.dot(d2);
            st.bends[k].0 += 1;
            if bend.dot(front) * sign < 0.0 {
                st.bends[k].1 += 1;
            }
        }
        if f == n {
            for (k, foot) in [B::LeftFoot, B::RightFoot].into_iter().enumerate() {
                if let Some(j) = tgt.joint(foot) {
                    st.sole_end[k] = (rot(&at.t_model[j]) * rig.target_ref[j].inverse() * Vec3::Y).y;
                }
            }
        }
        st.frames += 1;
    }
    st
}

#[allow(clippy::cast_precision_loss)]
fn print_stats(name: &str, st: &Stats) {
    let segs = segs();
    let n = st.frames.max(1) as f32;
    let mut line = format!("{name}: {} frames\n  bone            rot mean/max   twist mean/max (deg)\n", st.frames);
    let (mut worst_tw, mut sum_tw) = (0.0f32, 0.0f32);
    for (i, seg) in segs.iter().enumerate() {
        let (r, t) = (st.rot[i], st.tw[i]);
        worst_tw = worst_tw.max(t.1);
        sum_tw += t.0 / n;
        let _ = writeln!(line, "  {:<15} {:6.1} {:6.1}    {:6.1} {:6.1}", seg.bone.name(), r.0 / n, r.1, t.0 / n, t.1);
    }
    let bend = |k: usize| format!("{}/{}", st.bends[k].1, st.bends[k].0);
    let _ = writeln!(
        line,
        "  twist: mean over bones {:.1}, worst {:.1}; wrong-side bends knees L {} R {}, elbows L {} R {}; end sole·up L {:.2} R {:.2}",
        sum_tw / segs.len() as f32,
        worst_tw,
        bend(0),
        bend(1),
        bend(2),
        bend(3),
        st.sole_end[0],
        st.sole_end[1]
    );
    println!("{line}");
}

/// Boxes for Blender: per segment, start, direction, up and length, for the
/// source (recentred on its hips, shifted −X) and the target (+X).
fn dump_pose(rig: &Rig, s_ref: &[Quat], at: &Sampled, column: f32, out: &mut String) {
    let segs = segs();
    let side = |which: &Side, model: &[Affine3A], x: f32, is_source: bool, out: &mut String| {
        let hips = which.joint(B::Hips).map_or(Vec3::ZERO, |h| pos(&model[h]));
        let shift = Vec3::new(column + x - hips.x, 0.0, -hips.z);
        for seg in &segs {
            let Some(j) = which.joint(seg.bone) else { continue };
            let q = if is_source { rot(&model[j]) * s_ref[j].inverse() } else { rot(&model[j]) * rig.target_ref[j].inverse() };
            let start = pos(&model[j]);
            let (dir, len) = match which.child_joint(seg.bone, seg.child).map(|c| pos(&model[c]) - start) {
                Some(v) if v.length() > 0.02 => (v.normalize(), v.length()),
                _ => (q * seg.dir, 0.12),
            };
            let up = q * seg.up;
            let up = (up - dir * dir.dot(up)).normalize_or_zero();
            let p = start + shift;
            let _ = write!(
                out,
                "{{\"src\":{is_source},\"bone\":\"{}\",\"p\":[{},{},{}],\"d\":[{},{},{}],\"u\":[{},{},{}],\"l\":{}}},",
                seg.bone.name(),
                p.x,
                p.y,
                p.z,
                dir.x,
                dir.y,
                dir.z,
                up.x,
                up.y,
                up.z,
                len
            );
        }
    };
    side(&rig.source, &at.s_model, -0.9, true, out);
    side(&rig.target, &at.t_model, 0.9, false, out);
}

fn dump(rig: &Rig, s_ref: &[Quat], bound: &BoundSource, range: (f32, f32), name: &str) {
    let Ok(dir) = std::env::var("LAFAN1_RETARGET_DUMP") else { return };
    let mut out = String::from("[");
    for (k, u) in [0.0f32, 0.5, 1.0].into_iter().enumerate() {
        let at = sample(rig, bound, range.0 + (range.1 - range.0) * u);
        #[allow(clippy::cast_precision_loss)]
        dump_pose(rig, s_ref, &at, k as f32 * 3.8, &mut out);
    }
    out.pop();
    out.push(']');
    std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")), out).unwrap();
}

/// The LAFAN1 get-up candidates of the shipped manifest: (slot, file, clip, range).
fn get_ups() -> Vec<(&'static str, String, String, (f32, f32))> {
    let manifest = mw_graphs::sources::SourcesManifest::builtin();
    GET_UPS
        .iter()
        .filter_map(|&slot| {
            let c = manifest.slots.get(slot)?.candidates.iter().find(|c| c.source == "lafan1")?;
            Some((slot, c.file.clone()?, c.clip.clone(), c.range?))
        })
        .collect()
}

/// Runs `f` on each LAFAN1 get-up retargeted onto `base_anim`, with the
/// take's frame-0 T-pose as the source reference.
fn each_get_up(mut f: impl FnMut(&str, &Rig, &[Quat], &BoundSource, (f32, f32))) {
    let Some(vfs) = install() else { return };
    let sources = AnimSources::local();
    for (slot, file, clip, range) in get_ups() {
        let Some(src) = import(&sources, "lafan1", &file) else {
            eprintln!("{file} not in {}; skipping", sources.dir.display());
            return;
        };
        let rig = rig(&src, target_side(&vfs));
        let clip = src.clips.iter().find(|c| c.name == clip).expect("the take's clip");
        let bound = BoundSource::new(clip, &rig.source.skeleton);
        let at0 = sample(&rig, &bound, 0.0);
        let s_ref: Vec<Quat> = at0.s_model.iter().map(rot).collect();
        f(slot, &rig, &s_ref, &bound, range);
    }
}

/// Sanity: every get-up retargets to finite poses.
#[test]
fn lafan1_get_ups_retarget_to_finite_poses() {
    each_get_up(|slot, rig, s_ref, bound, range| {
        let st = measure(rig, s_ref, bound, range);
        assert!(st.frames > 10, "{slot}: {} frames", st.frames);
    });
}

#[test]
#[ignore = "measurement: LAFAN1 get-ups on base_anim, per-bone rotation and twist error vs the source, bend sides, soles"]
fn measure_lafan1_retarget_twist() {
    let mut first = true;
    each_get_up(|slot, rig, s_ref, bound, range| {
        if first {
            println!("{}", rig.report);
            first = false;
        }
        print_stats(slot, &measure(rig, s_ref, bound, range));
        dump(rig, s_ref, bound, range, slot);
    });
}

/// UAL2 (a T-pose rest): the same numbers with the source's rest as its
/// reference, so a change to the BVH path can be checked against it.
#[test]
#[ignore = "measurement: UAL2 ClimbUp_1m_RM on base_anim, same metrics (regression reference)"]
fn measure_ual2_retarget_twist() {
    let Some(vfs) = install() else { return };
    let sources = AnimSources::local();
    let Some((file, _)) = sources.references().into_iter().find(|(_, r)| r.source == UAL2.0 && r.clip == UAL2.1) else { return };
    let Some(real) = sources.real_path(&file) else { return };
    let importers = mw_bake::anim::anim_importers();
    let src = importers.for_path(&file).unwrap().import(&file, &std::fs::read(real).unwrap()).unwrap();
    let rig = rig(&src, target_side(&vfs));
    let clip = src.clips.iter().find(|c| c.name == UAL2.1).expect("the clip");
    let bound = BoundSource::new(clip, &rig.source.skeleton);
    let s_ref: Vec<Quat> = model_of(&rig.source.skeleton, rig.source.skeleton.rest()).iter().map(rot).collect();
    let range = (0.0, clip.duration);
    println!("{}", rig.report);
    print_stats("ual2 ClimbUp_1m_RM", &measure(&rig, &s_ref, &bound, range));
    dump(&rig, &s_ref, &bound, range, "ual2_climb");
}
