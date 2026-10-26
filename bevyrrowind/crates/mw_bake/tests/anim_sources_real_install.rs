//! Library clips through the bake (animation-sources.md): Quaternius UAL1's
//! `ClimbLedge_RM` cooked on the library's skeleton and retargeted onto the
//! humanoid skeletons, the humanoid graph's `traverse.mantle.waist` slot
//! resolving to it with CC0 provenance. Sanity only (rule 5); the numbers
//! come from the ignored measurement test (`-- --ignored --nocapture`).
//! Skips (passes) without a Morrowind install or the library file
//! (`BEVYRROWIND_ANIM_SOURCES`, default `<repo>/animation packs`).

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
use sf_anim::{Clip, LocalPose, Skeleton};
use sf_anim_graph::CompiledGraph;
use sf_anim_graph::steps::{ANIMGRAPH_COMPILE, graph_subject};
use sf_bake::{Baker, ProductKey, Store};
use sf_character::{HumanoidBone as B, RigMap};
use sf_vfs::Vfs;

const SLOT: &str = "traverse.mantle.waist";
const REFERENCE: &str = "quaternius_ual1:ClimbLedge_RM";
/// `base_anim`, `base_anim_female`, `base_animkna`.
const TARGETS: usize = 3;

struct Setup {
    install: MorrowindInstall,
    vfs: Arc<Vfs>,
    sources: Arc<AnimSources>,
    file: String,
}

fn setup() -> Option<Setup> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let sources = Arc::new(AnimSources::local());
    let (file, _) = sources.references().into_iter().find(|(_, r)| r.reference == REFERENCE).expect("the manifest names the mantle clip");
    if sources.real_path(&file).is_none() {
        eprintln!("{file} not in {}; skipping", sources.dir.display());
        return None;
    }
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    Some(Setup { install, vfs: Arc::new(vfs), sources, file })
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
    ProductKey::new(ANIM_RETARGET, retarget_subject(target, &anim_subject(&s.file, &s.file, REFERENCE)))
}

#[test]
fn ual1_mantle_retargets_onto_the_humanoid_skeletons() {
    let Some(s) = setup() else { return };
    let dir = tempfile::tempdir().unwrap();
    let (report, baker) = bake(&s, dir.path(), &s.sources);
    for (key, e) in &report.failures {
        eprintln!("failed: {key}: {e}");
    }
    assert!(report.failures.is_empty());
    println!("slots: {:?}; {} library clips", report.slots, report.library_clips);
    assert_eq!(report.slots, [format!("{SLOT}: {REFERENCE}")]);
    assert_eq!(report.library_clips, TARGETS);

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
        assert_eq!((used.provenance.source.as_str(), used.provenance.license.as_str(), used.provenance.redistributable), ("quaternius_ual1", "CC0-1.0", true));
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
    assert!(report.slots[0].starts_with(&format!("{SLOT}: fallback Drop")), "{:?}", report.slots);
    let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject(Template::Humanoid.path(), HUMANOID_SETS[0].skeleton));
    let graph = CompiledGraph::from_bytes(&bytes(&baker, &key)).unwrap();
    assert!(!graph.slot_clips.iter().any(|(n, _)| n == SLOT));
    assert!(graph.resolutions.iter().any(|r| r.name == SLOT && r.slot.is_none()));
}

#[test]
#[ignore = "measurement: mantle clip numbers, source vs retargeted"]
fn measure_ual1_mantle() {
    let Some(s) = setup() else { return };
    let dir = tempfile::tempdir().unwrap();
    let (_, baker) = bake(&s, dir.path(), &s.sources);
    let source = skeleton(&baker, &s.file);
    let (mut source_map, map_report) = sf_character::auto_map(&source);
    source_map.auto_align(&source);
    println!("source rig: {} bones mapped, clean {}", source_map.bones.len(), map_report.is_clean());
    let source_clip = Clip::from_bytes(&bytes(&baker, &ProductKey::new(ANIM_COOK, anim_subject(&s.file, &s.file, REFERENCE)))).unwrap();
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
    describe("source (UAL1 rig)", &source_clip);
    let limbs: [(&str, &[B]); 3] = [
        ("leg", &LEG),
        ("arm", &[B::LeftUpperArm, B::LeftLowerArm, B::LeftHand]),
        ("spine", &[B::Hips, B::Spine, B::Chest, B::UpperChest, B::Neck, B::Head]),
    ];
    for set in &HUMANOID_SETS[..TARGETS] {
        let target = skeleton(&baker, set.skeleton);
        let map = target_map(&target);
        let clip = Clip::from_bytes(&bytes(&baker, &retargeted(&s, set.skeleton))).unwrap();
        describe(set.skeleton, &clip);
        for (name, bones) in limbs {
            let present = |m: &RigMap| bones.iter().filter(|b| m.joint(**b).is_some()).copied().collect::<Vec<_>>();
            let (sl, tl) = (chain(&source, &source_map, &present(&source_map)), chain(&target, &map, &present(&map)));
            println!("  {name}: source {sl:.3} m, target {tl:.3} m, ratio {:.3}", tl / sl);
        }
        let model = model_at(&target, &clip, 0.0);
        let start = clip.root_at(0.0).0;
        let feet: Vec<f32> = [B::LeftFoot, B::RightFoot].iter().map(|f| pos(&target, &model, &map, *f).y + start.y).collect();
        println!("  frame 0 ankle heights {:.3} / {:.3} m", feet[0], feet[1]);
    }
}
