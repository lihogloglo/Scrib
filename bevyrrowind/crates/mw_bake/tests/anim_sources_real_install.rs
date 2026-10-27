//! Library clips through the bake (animation-sources.md): Quaternius UAL2's
//! `ClimbUp_1m` cooked on the library's skeleton and retargeted onto the
//! humanoid skeletons, the humanoid graph's `traverse.mantle.waist` slot
//! resolving to it with CC0 provenance; LAFAN1's mocap mantle (the shipped
//! first candidate) with root motion from the target pelvis, private only.
//! Sanity only (rule 5); the numbers come from the ignored measurement tests
//! (`-- --ignored --nocapture`).
//! Skips (passes) without a Morrowind install or the library file
//! (`BEVYRROWIND_ANIM_SOURCES`, default `<repo>/animation packs`, then the bundled
//! `bevyrrowind/assets/animation/quaternius`).

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use glam::{Affine3A, Vec3};
use mw_bake::VfsInputs;
use mw_bake::anim::{self, ActorAnimSet, ActorSets, AnimBakeReport};
use mw_bake::sources::AnimSources;
use mw_graphs::{HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::steps::{ANIM_COOK, ANIM_RETARGET, SKELETON_COOK, anim_subject, retarget_subject};
use sf_anim::{Clip, ContactKind, LocalPose, Skeleton};
use sf_anim_graph::CompiledGraph;
use sf_anim_graph::steps::{ANIMGRAPH_COMPILE, graph_subject};
use sf_bake::{Baker, ProductKey, Store};
use sf_character::{HumanoidBone as B, RigMap};
use sf_vfs::Vfs;

const SLOT: &str = "traverse.mantle.waist";
const REFERENCE: &str = "quaternius_ual2:ClimbUp_1m";
/// The high mantle: UAL2's 2 m climb-up.
const HIGH_SLOT: &str = "traverse.mantle.high";
const HIGH: &str = "quaternius_ual2:ClimbUp_2m";
/// The shipped LAFAN1 candidate (second, after UAL2; private only): a run-in
/// step-up onto a 1.05 m box. Its tests resolve it explicitly.
const LAFAN1: &str = "lafan1:bvh/obstacles4_subject3.bvh:obstacles4_subject3@199.4-201.2";
/// Every slot in the shipped manifest with a non-LAFAN1 candidate: the 3 traversal slots (`traverse.climb.up` = UAL1
/// `ClimbLedge`, for B3; the two mantles), the 16 combat slots and the 2 Quaternius get-ups (`getup.supine`, `getup.prone`).
const SLOTS: usize = 21;
/// `base_anim`, `base_anim_female`, `base_animkna`.
const TARGETS: usize = 3;

struct Setup {
    install: MorrowindInstall,
    vfs: Arc<Vfs>,
    sources: Arc<AnimSources>,
    file: String,
    reference: &'static str,
}

/// UAL2's mantle: the shipped manifest less its LAFAN1 candidates, so the
/// slot resolves to UAL2 whether or not LAFAN1 is in the folder.
fn setup() -> Option<Setup> {
    setup_with(REFERENCE, false)
}

/// `only_lafan1`: keep only the mantle slot's LAFAN1 candidates (it resolves
/// to LAFAN1 whatever the slot order); otherwise drop LAFAN1 from every slot.
fn setup_with(reference: &'static str, only_lafan1: bool) -> Option<Setup> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let local = AnimSources::local();
    let mut manifest = local.manifest.clone();
    for (name, slot) in &mut manifest.slots {
        if !only_lafan1 || name == SLOT {
            slot.candidates.retain(|c| (c.source == "lafan1") == only_lafan1);
        }
    }
    let sources = Arc::new(AnimSources { manifest, ..local });
    let (file, _) = sources.references().into_iter().find(|(_, r)| r.reference == reference).expect("the manifest names the mantle clip");
    if sources.real_path(&file).is_none() {
        eprintln!("{file} not in {}; skipping", sources.dir.display());
        return None;
    }
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    Some(Setup { install, vfs: Arc::new(vfs), sources, file, reference })
}

fn sets() -> ActorSets {
    let mut sets = ActorSets::default();
    for set in &HUMANOID_SETS[..TARGETS] {
        let s = ActorAnimSet {
            skeleton: set.skeleton.to_owned(),
            files: set.animations.iter().map(|f| (*f).to_owned()).collect(),
            humanoid: true,
            graphs: BTreeSet::from([Template::Humanoid.path().to_owned()]),
            actors: 1,
            ..ActorAnimSet::default()
        };
        sets.sets.insert(s.skeleton.clone(), s);
    }
    sets
}

fn bake(s: &Setup, dir: &Path, sources: &Arc<AnimSources>) -> (AnimBakeReport, Baker) {
    let store = Store::open(dir.join("store")).unwrap();
    let inputs = Arc::new(VfsInputs::new(s.vfs.clone(), &s.install, dir.join("filehash.memo")).with_anim_sources(sources.clone()));
    let mut baker = Baker::new(store, inputs);
    let report = anim::bake_animation(&mut baker, &sets(), sources);
    (report, baker)
}

fn bytes(baker: &Baker, key: &ProductKey) -> Vec<u8> {
    let digest = baker.product(key).unwrap_or_else(|e| panic!("{key}: {e}"));
    baker.store().get(&digest).unwrap()
}

fn skeleton(baker: &Baker, path: &str) -> Skeleton {
    Skeleton::from_bytes(&bytes(baker, &ProductKey::new(SKELETON_COOK, path))).unwrap()
}

fn model_at(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Affine3A> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    model_of(skeleton, &pose.to_vec())
}

fn model_of(skeleton: &Skeleton, local: &[sf_anim::Trs]) -> Vec<Affine3A> {
    let mut out: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        out.push(skeleton.parent(j).map_or(m, |p| out[p] * m));
    }
    out
}

fn pos(skeleton: &Skeleton, model: &[Affine3A], map: &RigMap, bone: B) -> Vec3 {
    Vec3::from(model[skeleton.find(map.joint(bone).expect("mapped")).unwrap()].translation)
}

/// Rest length of a joint chain (sum of segment lengths).
fn chain(skeleton: &Skeleton, map: &RigMap, bones: &[B]) -> f32 {
    let rest = model_of(skeleton, skeleton.rest());
    bones.windows(2).map(|w| pos(skeleton, &rest, map, w[0]).distance(pos(skeleton, &rest, map, w[1]))).sum()
}

const LEG: [B; 3] = [B::LeftUpperLeg, B::LeftLowerLeg, B::LeftFoot];

fn target_map(skeleton: &Skeleton) -> RigMap {
    mw_anim::rig::humanoid(skeleton).unwrap().map
}

fn retargeted(s: &Setup, target: &str) -> ProductKey {
    ProductKey::new(ANIM_RETARGET, retarget_subject(target, &anim_subject(&s.file, &s.file, s.reference)))
}

#[test]
fn mantle_clip_retargets_onto_the_humanoid_skeletons() {
    let Some(s) = setup() else { return };
    let dir = tempfile::tempdir().unwrap();
    let (report, baker) = bake(&s, dir.path(), &s.sources);
    for (key, e) in &report.failures {
        eprintln!("failed: {key}: {e}");
    }
    assert!(report.failures.is_empty());
    println!("slots: {:?}; {} library clips", report.slots, report.library_clips);
    assert!(report.slots.contains(&format!("{SLOT}: {REFERENCE}")), "{:?}", report.slots);
    assert!(report.slots.contains(&format!("{HIGH_SLOT}: {HIGH}")), "{:?}", report.slots);
    assert_eq!(report.library_clips, SLOTS * TARGETS);

    for set in &HUMANOID_SETS[..TARGETS] {
        let target = skeleton(&baker, set.skeleton);
        let clip = Clip::from_bytes(&bytes(&baker, &retargeted(&s, set.skeleton))).unwrap();
        let map = target_map(&target);
        let tolerance = 0.05 * chain(&target, &map, &LEG);
        // Finite everywhere, root motion present, the root rises.
        #[allow(clippy::cast_precision_loss)]
        let times: Vec<f32> = (0..=30).map(|i| clip.duration * i as f32 / 30.0).collect();
        for &t in &times {
            let model = model_at(&target, &clip, t);
            assert!(model.iter().all(Affine3A::is_finite), "{}: non-finite pose at {t}", set.skeleton);
        }
        assert!(clip.root_motion.is_some(), "{}: no root motion", set.skeleton);
        let (start, end) = (clip.root_at(0.0).0, clip.root_at(clip.duration).0);
        assert!(start.is_finite() && end.is_finite());
        assert!(end.y > start.y, "{}: root doesn't rise ({start} → {end})", set.skeleton);
        // Frame 0 stands on the start floor (root frame: y = 0 is the floor).
        let model = model_at(&target, &clip, 0.0);
        for foot in [B::LeftFoot, B::RightFoot] {
            let y = pos(&target, &model, &map, foot).y + start.y;
            assert!(y > -tolerance, "{}: {foot:?} {y} below the floor", set.skeleton);
        }

        // The graph's slot resolves to it, with the library's provenance.
        let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject(Template::Humanoid.path(), set.skeleton));
        let graph = CompiledGraph::from_bytes(&bytes(&baker, &key)).unwrap();
        let (_, slot) = graph.slot_clips.iter().find(|(n, _)| n == SLOT).unwrap_or_else(|| panic!("{}: no {SLOT} slot clip", set.skeleton));
        let used = &graph.clips[usize::from(*slot)];
        assert_eq!(used.product, retargeted(&s, set.skeleton));
        assert_eq!((used.provenance.source.as_str(), used.provenance.license.as_str(), used.provenance.redistributable), ("quaternius_ual2", "CC0-1.0", true));
        assert_eq!(graph.non_redistributable().count(), 0);
    }
}

/// Without the library the slot drops: no error, no retarget, the graph
/// compiles without it.
#[test]
fn missing_library_drops_the_slot() {
    let Some(s) = setup() else { return };
    let dir = tempfile::tempdir().unwrap();
    let empty = tempfile::tempdir().unwrap();
    let sources = Arc::new(AnimSources::new(s.sources.manifest.clone(), empty.path()));
    let (report, baker) = bake(&s, dir.path(), &sources);
    assert!(report.failures.is_empty(), "{:?}", report.failures);
    assert_eq!(report.library_clips, 0);
    assert!(report.slots.iter().any(|s| s.starts_with(&format!("{SLOT}: fallback Drop"))), "{:?}", report.slots);
    let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject(Template::Humanoid.path(), HUMANOID_SETS[0].skeleton));
    let graph = CompiledGraph::from_bytes(&bytes(&baker, &key)).unwrap();
    assert!(!graph.slot_clips.iter().any(|(n, _)| n == SLOT));
    assert!(graph.resolutions.iter().any(|r| r.name == SLOT && r.slot.is_none()));
}

#[test]
#[ignore = "measurement: waist mantle clip numbers, source vs retargeted, contact timing"]
fn measure_mantle_waist() {
    let Some(s) = setup() else { return };
    measure_mantle(&s);
}

/// The high mantle's `ClimbUp_2m`: its rise sets the rule's height band (`traverse.mantle.high`).
#[test]
#[ignore = "measurement: high mantle clip numbers, source vs retargeted, contact timing"]
fn measure_mantle_high() {
    let Some(s) = setup_with(HIGH, false) else { return };
    measure_mantle(&s);
}

/// Root rise, forward travel and a hand/foot timeline of `s.reference`, source and retargeted.
fn measure_mantle(s: &Setup) {
    let dir = tempfile::tempdir().unwrap();
    let (_, baker) = bake(s, dir.path(), &s.sources);
    let source = skeleton(&baker, &s.file);
    let (mut source_map, map_report) = sf_character::auto_map(&source);
    source_map.auto_align(&source);
    println!("source rig: {} bones mapped, clean {}", source_map.bones.len(), map_report.is_clean());
    let source_clip = Clip::from_bytes(&bytes(&baker, &ProductKey::new(ANIM_COOK, anim_subject(&s.file, &s.file, s.reference)))).unwrap();
    let describe = |name: &str, clip: &Clip| {
        let (a, b) = (clip.root_at(0.0), clip.root_at(clip.duration));
        let d = b.0 - a.0;
        println!(
            "{name}: {:.3} s, {} frames at {:.0} Hz; root rise {:.3} m, forward {:.3} m, side {:.3} m, yaw {:.1}°",
            clip.duration,
            clip.frame_count,
            clip.rate,
            d.y,
            d.z,
            d.x,
            (b.1 - a.1).to_degrees()
        );
    };
    describe("source (UAL2 rig)", &source_clip);
    hands(&source_clip);
    hand_frames(&source, &source_map, &source_clip);
    let limbs: [(&str, &[B]); 3] = [
        ("leg", &LEG),
        ("arm", &[B::LeftUpperArm, B::LeftLowerArm, B::LeftHand]),
        ("spine", &[B::Hips, B::Spine, B::Chest, B::UpperChest, B::Neck, B::Head]),
    ];
    for set in &HUMANOID_SETS[..TARGETS] {
        let target = skeleton(&baker, set.skeleton);
        let map = target_map(&target);
        let clip = Clip::from_bytes(&bytes(&baker, &retargeted(s, set.skeleton))).unwrap();
        describe(set.skeleton, &clip);
        hands(&clip);
        hand_frames(&target, &map, &clip);
        for (name, bones) in limbs {
            let present = |m: &RigMap| bones.iter().filter(|b| m.joint(**b).is_some()).copied().collect::<Vec<_>>();
            let (sl, tl) = (chain(&source, &source_map, &present(&source_map)), chain(&target, &map, &present(&map)));
            println!("  {name}: source {sl:.3} m, target {tl:.3} m, ratio {:.3}", tl / sl);
        }
        let model = model_at(&target, &clip, 0.0);
        let start = clip.root_at(0.0).0;
        let feet: Vec<f32> = [B::LeftFoot, B::RightFoot].iter().map(|f| pos(&target, &model, &map, *f).y + start.y).collect();
        println!("  frame 0 ankle heights {:.3} / {:.3} m", feet[0], feet[1]);
        // Timeline in the clip's start frame (root at t added; yaw ignored):
        // where the hands and feet are, to read contact timing by eye.
        println!("  t s | root y z | hand L y z | hand R y z | ankle L y | ankle R y");
        for i in 0..=12u8 {
            let t = clip.duration * f32::from(i) / 12.0;
            let model = model_at(&target, &clip, t);
            let root = clip.root_at(t).0;
            let at = |b: B| pos(&target, &model, &map, b) + root;
            let (hl, hr, fl, fr) = (at(B::LeftHand), at(B::RightHand), at(B::LeftFoot), at(B::RightFoot));
            println!(
                "  {t:.2} | {:.2} {:.2} | {:.2} {:.2} | {:.2} {:.2} | {:.2} | {:.2}",
                root.y, root.z, hl.y, hl.z, hr.y, hr.z, fl.y, fr.y
            );
        }
    }
}

// --- LAFAN1: root motion from the target pelvis ---------------------------------------------

/// Heading of a left-right pair (`left - right` on the ground, turned to
/// forward: left is +X, forward +Z), radians from +Z.
fn heading_of(left: Vec3, right: Vec3) -> f32 {
    let f = Vec3::new(left.x - right.x, 0.0, left.z - right.z).cross(Vec3::Y);
    f.x.atan2(f.z)
}

/// The toe joint's position: the mapped toe bone, else the foot's first
/// child (Morrowind's `Bip01 L Toe0` stays unmapped).
fn toe_pos(target: &Skeleton, model: &[Affine3A], map: &RigMap, foot: B, toe: B) -> Vec3 {
    let j = map.joint(toe).and_then(|n| target.find(n)).unwrap_or_else(|| {
        let f = target.find(map.joint(foot).expect("mapped foot")).unwrap();
        (0..target.len()).find(|&j| target.parent(j) == Some(f)).expect("a joint under the foot")
    });
    Vec3::from(model[j].translation)
}

/// Angle of `toe - ankle` on the ground from +Z, degrees.
fn toe_angle(model: &[Affine3A], target: &Skeleton, map: &RigMap, ankle: B, toe: B) -> f32 {
    let d = toe_pos(target, model, map, ankle, toe) - pos(target, model, map, ankle);
    d.x.atan2(d.z).to_degrees()
}

fn lafan1_bake() -> Option<(Setup, tempfile::TempDir, Baker)> {
    let s = setup_with(LAFAN1, true)?;
    let dir = tempfile::tempdir().unwrap();
    let (report, baker) = bake(&s, dir.path(), &s.sources);
    for (key, e) in &report.failures {
        eprintln!("failed: {key}: {e}");
    }
    assert!(report.failures.is_empty());
    assert!(report.slots.contains(&format!("{SLOT}: {LAFAN1}")), "{:?}", report.slots);
    Some((s, dir, baker))
}

/// LAFAN1's mocap take (no root bone, hips wandering the capture volume)
/// retargets onto `base_anim` with root motion from the target pelvis: the
/// clip starts over its root facing +Z, rises onto the obstacle, feet
/// pointing forward and never under the start floor; the compiled graph
/// marks it non-redistributable.
#[test]
fn lafan1_mantle_starts_over_its_root_facing_forward() {
    let Some((s, _dir, baker)) = lafan1_bake() else { return };
    let set = &HUMANOID_SETS[0];
    let target = skeleton(&baker, set.skeleton);
    let clip = Clip::from_bytes(&bytes(&baker, &retargeted(&s, set.skeleton))).unwrap();
    let map = target_map(&target);
    let tolerance = 0.05 * chain(&target, &map, &LEG);
    #[allow(clippy::cast_precision_loss)]
    let times: Vec<f32> = (0..=30).map(|i| clip.duration * i as f32 / 30.0).collect();
    let start = clip.root_at(0.0).0;
    for &t in &times {
        let model = model_at(&target, &clip, t);
        assert!(model.iter().all(Affine3A::is_finite), "non-finite pose at {t}");
        let root = clip.root_at(t).0;
        for foot in [B::LeftFoot, B::RightFoot] {
            let y = pos(&target, &model, &map, foot).y + root.y - start.y;
            assert!(y > -tolerance, "{foot:?} {y} m under the start floor at {t}");
        }
    }
    assert!(clip.root_motion.is_some(), "no root motion");
    let model = model_at(&target, &clip, 0.0);
    let hips = pos(&target, &model, &map, B::Hips);
    assert!(Vec3::new(hips.x, 0.0, hips.z).length() < 0.05, "frame-0 hips {hips} off the root");
    let heading = heading_of(pos(&target, &model, &map, B::LeftUpperLeg), pos(&target, &model, &map, B::RightUpperLeg));
    assert!(heading.abs() < 10f32.to_radians(), "frame-0 hips face {}° off +Z", heading.to_degrees());
    let rise = clip.root_at(clip.duration).0.y - start.y;
    assert!((0.6..1.4).contains(&rise), "root rises {rise} m");
    for (ankle, toe) in [(B::LeftFoot, B::LeftToes), (B::RightFoot, B::RightToes)] {
        let ahead = (toe_pos(&target, &model, &map, ankle, toe) - pos(&target, &model, &map, ankle)).z;
        // A flipped foot (the degenerate-rest bug) puts the toe a foot length (~0.15 m) behind the ankle. Since the
        // T-pose rest frame (2026-10-27) the foot keeps the actor's own direction: mid-step at frame 0, the toe can sit
        // a few cm behind the ankle, which is no flip.
        assert!(ahead > -0.06, "{toe:?} behind its ankle at frame 0 ({ahead} m): flipped foot?");
    }

    let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject(Template::Humanoid.path(), set.skeleton));
    let graph = CompiledGraph::from_bytes(&bytes(&baker, &key)).unwrap();
    let (_, slot) = graph.slot_clips.iter().find(|(n, _)| n == SLOT).expect("the mantle slot");
    let used = &graph.clips[usize::from(*slot)];
    assert_eq!(used.product, retargeted(&s, set.skeleton));
    assert_eq!((used.provenance.source.as_str(), used.provenance.license.as_str(), used.provenance.redistributable), ("lafan1", "CC-BY-NC-ND-4.0", false));
    assert!(graph.non_redistributable().count() > 0);
}

#[test]
#[ignore = "measurement: LAFAN1 waist mantle on base_anim (root from the target pelvis)"]
fn measure_mantle_waist_lafan1() {
    let Some((s, _dir, baker)) = lafan1_bake() else { return };
    for set in &HUMANOID_SETS[..TARGETS] {
        let target = skeleton(&baker, set.skeleton);
        let map = target_map(&target);
        let clip = Clip::from_bytes(&bytes(&baker, &retargeted(&s, set.skeleton))).unwrap();
        let (first, last) = (clip.root_at(0.0), clip.root_at(clip.duration));
        let travel = last.0 - first.0;
        println!(
            "{}: {:.3} s, {} frames at {:.0} Hz; root rise {:.3} m, forward {:.3} m, side {:.3} m, yaw {:.1}°; max error {:.5} m",
            set.skeleton,
            clip.duration,
            clip.frame_count,
            clip.rate,
            travel.y,
            travel.z,
            travel.x,
            (last.1 - first.1).to_degrees(),
            clip.meta.max_error
        );
        let model = model_at(&target, &clip, 0.0);
        let hips_heading = heading_of(pos(&target, &model, &map, B::LeftUpperLeg), pos(&target, &model, &map, B::RightUpperLeg));
        let shoulders = heading_of(pos(&target, &model, &map, B::LeftUpperArm), pos(&target, &model, &map, B::RightUpperArm));
        println!(
            "  frame 0: thighs face {:.1}°, shoulders {:.1}°; toe angle L {:.1}° R {:.1}°",
            hips_heading.to_degrees(),
            shoulders.to_degrees(),
            toe_angle(&model, &target, &map, B::LeftFoot, B::LeftToes),
            toe_angle(&model, &target, &map, B::RightFoot, B::RightToes)
        );
        println!("  t s | root x y z yaw° | hips-root x z | ankle L y | ankle R y | toe L° R°");
        let mut min_yaw = f32::INFINITY;
        let mut max_yaw = f32::NEG_INFINITY;
        for i in 0..=12u8 {
            let t = clip.duration * f32::from(i) / 12.0;
            let model = model_at(&target, &clip, t);
            let (root, yaw) = clip.root_at(t);
            min_yaw = min_yaw.min(yaw);
            max_yaw = max_yaw.max(yaw);
            let hips = pos(&target, &model, &map, B::Hips);
            let ankle = |foot: B| pos(&target, &model, &map, foot).y + root.y - first.0.y;
            println!(
                "  {t:.2} | {:.2} {:.2} {:.2} {:.1} | {:.3} {:.3} | {:.2} | {:.2} | {:.0} {:.0}",
                root.x,
                root.y,
                root.z,
                yaw.to_degrees(),
                hips.x,
                hips.z,
                ankle(B::LeftFoot),
                ankle(B::RightFoot),
                toe_angle(&model, &target, &map, B::LeftFoot, B::LeftToes),
                toe_angle(&model, &target, &map, B::RightFoot, B::RightToes)
            );
        }
        println!("  root yaw range {:.1}° .. {:.1}°", min_yaw.to_degrees(), max_yaw.to_degrees());
    }
}

/// The clip's baked hand contacts (B2b: the hands on the lip), for the hand IK slice.
fn hands(clip: &Clip) {
    let span = clip.meta.contact_span(ContactKind::Hand);
    let feet = clip.meta.contacts_of(ContactKind::Foot).count();
    println!("  hand contacts (span {span:?}; {feet} foot contacts):");
    for c in clip.meta.contacts_of(ContactKind::Hand) {
        println!(
            "    {:?} {} ({}): {:.3}-{:.3} s, at {:.3} {:.3} {:.3} (clip), {:.3} {:.3} {:.3} (root frame)",
            c.side, c.limb, c.joint, c.start, c.end, c.position.x, c.position.y, c.position.z, c.local.x, c.local.y, c.local.z
        );
    }
}

/// Per frame, in clip space: each hand's height and forward position, its speed, and the shoulder's speed against it
/// (m/s): the hand-contact rule's inputs, to read grab and release by eye.
fn hand_frames(skeleton: &Skeleton, map: &RigMap, clip: &Clip) {
    use std::fmt::Write as _;
    let n = clip.frame_count as usize;
    #[allow(clippy::cast_precision_loss)]
    let time = |f: usize| clip.duration * f as f32 / (n - 1).max(1) as f32;
    let at = |f: usize, b: B| {
        let t = time(f);
        let (root, yaw) = clip.root_at(t);
        let model = model_at(skeleton, clip, t);
        glam::Quat::from_rotation_y(yaw) * pos(skeleton, &model, map, b) + root
    };
    let pairs = [(B::LeftHand, B::LeftUpperArm), (B::RightHand, B::RightUpperArm)];
    let tracks: Vec<(Vec<Vec3>, Vec<Vec3>)> =
        pairs.iter().map(|&(h, s)| ((0..n).map(|f| at(f, h)).collect(), (0..n).map(|f| at(f, s)).collect())).collect();
    println!("  frame t | hand L y z v | shoulder-L v | hand R y z v | shoulder-R v");
    for f in 0..n {
        let (a, b) = (f.saturating_sub(1), (f + 1).min(n - 1));
        let dt = time(b) - time(a);
        let mut line = format!("  {f:3} {:.3}", time(f));
        for (hand, shoulder) in &tracks {
            let v = hand[b].distance(hand[a]) / dt;
            let rel = (shoulder[b] - hand[b]).distance(shoulder[a] - hand[a]) / dt;
            let _ = write!(line, " | {:.2} {:.2} {:.2} | {:.2}", hand[f].y, hand[f].z, v, rel);
        }
        println!("{line}");
    }
}
