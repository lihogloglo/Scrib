//! Generated clips (motion-generation.md MG3): NVIDIA Kimodo SOMA-77 BVH,
//! source `generated`, cooked on their own skeleton and retargeted onto
//! `base_anim`, `base_anim_female` and `base_animkna` in process
//! ([`mw_bake::anim::cook_library_clip`]), and that path equals the bake's
//! `anim.retarget` product.
//! Sanity only (rule 5); the numbers come from the ignored measurement
//! (`-- --ignored --nocapture`).
//! Skips (passes) without a Morrowind install or the sample files, written
//! with `kimodo_gen ... --bvh --bvh_standard_tpose` into
//! `<library>/generated/samples/` (`BEVYRROWIND_ANIM_SOURCES`, default
//! `<repo>/animation packs`): `stand_still.bvh` ("A person stands still.",
//! seed 1, 3 s), `sweep.bvh`, `lean_wall.bvh`, `fold_cloth.bvh` (6 s); the
//! `*_seedrest.bvh` twins are Kimodo's default (BONES-SEED rest) export.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;

use glam::{Affine3A, Quat, Vec3};
use mw_bake::VfsInputs;
use mw_bake::anim::{self, ActorAnimSet, ActorSets, HumanoidTarget, cook_library_clip};
use mw_bake::sources::AnimSources;
use mw_graphs::sources::{Candidate, Slot, SourceFile};
use mw_graphs::{HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::steps::{ANIM_RETARGET, anim_subject, retarget_subject};
use sf_anim::{Clip, ClipRules, LocalPose, RootMotionMode, Skeleton};
use sf_anim_graph::asset::ClipFallback;
use sf_bake::{Baker, ProductKey, Store};
use sf_character::HumanoidBone as B;
use sf_vfs::{Vfs, VfsPath};

const SOURCE: &str = "generated";
/// Kimodo T-pose-rest exports (the shipped choice).
const SAMPLES: [&str; 4] = ["stand_still", "sweep", "lean_wall", "fold_cloth"];
/// Kimodo's default export of the same motions (rest = BONES-SEED's pose).
const SEED_REST: [&str; 2] = ["stand_still_seedrest", "sweep_seedrest"];
/// `base_anim`, `base_anim_female`, `base_animkna`.
const TARGETS: usize = 3;

fn install() -> Option<(MorrowindInstall, Vfs)> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    Some((install, vfs))
}

fn sample_path(name: &str) -> Option<PathBuf> {
    let path = AnimSources::local().dir.join(SOURCE).join("samples").join(format!("{name}.bvh"));
    if path.is_file() {
        return Some(path);
    }
    eprintln!("{} missing; skipping", path.display());
    None
}

fn targets(vfs: &Vfs) -> Vec<(&'static str, HumanoidTarget)> {
    HUMANOID_SETS[..TARGETS]
        .iter()
        .map(|set| {
            let bytes = vfs.read(&VfsPath::new(set.skeleton).unwrap()).unwrap();
            let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
            (set.skeleton, HumanoidTarget::new(skeleton).unwrap())
        })
        .collect()
}

/// An ambient activity slot: plays once, root motion drives the character.
fn slot(file: &str) -> Slot {
    let clip = file.rsplit('/').next().unwrap().trim_end_matches(".bvh").to_owned();
    Slot {
        graph: "humanoid".to_owned(),
        candidates: vec![Candidate { source: SOURCE.to_owned(), clip, file: Some(file.to_owned()), range: None }],
        looping: sf_anim::Looping::default(),
        root: RootMotionMode::Drive,
        vertical: false,
        in_place: false,
        fallback: ClipFallback::Drop,
    }
}

fn rules() -> ClipRules {
    let s = slot("samples/x.bvh");
    s.rules(&s.candidates[0])
}

fn model_of(skeleton: &Skeleton, local: &[sf_anim::Trs]) -> Vec<Affine3A> {
    let mut out: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        out.push(skeleton.parent(j).map_or(m, |p| out[p] * m));
    }
    out
}

/// World model pose at `t`: the sampled pose carried by the root motion.
fn world_at(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Affine3A> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    let (pos, yaw) = clip.root_at(t);
    let root = Affine3A::from_rotation_translation(Quat::from_rotation_y(yaw), pos);
    model_of(skeleton, &pose.to_vec()).into_iter().map(|m| root * m).collect()
}

fn joint_pos(target: &HumanoidTarget, model: &[Affine3A], bone: B) -> Vec3 {
    Vec3::from(model[target.skeleton.find(target.map.joint(bone).expect("mapped")).unwrap()].translation)
}

#[allow(clippy::cast_precision_loss)]
fn times(clip: &Clip, n: usize) -> Vec<f32> {
    (0..=n).map(|i| clip.duration * i as f32 / n as f32).collect()
}

/// The real SOMA-77 skeleton auto-maps cleanly (thigh = `LeftLeg`, fingers
/// past the metacarpal), with the T-pose rest needing no facing turn.
#[test]
fn soma77_skeleton_auto_maps_cleanly() {
    let Some(path) = sample_path("stand_still") else { return };
    let virtual_path = AnimSources::virtual_path(SOURCE, "stand_still.bvh");
    let importers = anim::anim_importers();
    let src = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &std::fs::read(&path).unwrap()).unwrap();
    let (skeleton, _) = Skeleton::new(src.skeleton.as_ref().unwrap()).unwrap();
    assert_eq!(skeleton.len(), 78, "SOMA-77 plus Kimodo's Root wrapper");
    let (mut map, report) = sf_character::auto_map(&skeleton);
    assert!(report.is_clean(), "{report:#?}");
    for (bone, joint) in [(B::LeftUpperLeg, "LeftLeg"), (B::LeftLowerLeg, "LeftShin"), (B::UpperChest, "Chest"), (B::LeftIndexProximal, "LeftHandIndex2")] {
        assert_eq!(map.joint(bone), Some(joint), "{bone:?}");
    }
    assert_eq!(map.bones.iter().filter(|(_, b)| b.finger_of().is_some()).count(), 30);
    let align = map.auto_align(&skeleton);
    println!("auto-align: facing {:.1}°, max residual {:.2}°, {} joints rotated", align.facing_correction_deg, align.max_residual_deg, align.rotated.len());
    assert!(align.facing_correction_deg.abs() < 1.0, "{align:?}");
}

#[test]
fn generated_clips_retarget_onto_the_humanoid_skeletons() {
    let Some((_, vfs)) = install() else { return };
    let samples: Vec<(&str, PathBuf)> = SAMPLES.iter().filter_map(|s| sample_path(s).map(|p| (*s, p))).collect();
    if samples.is_empty() {
        return;
    }
    for (skeleton, target) in targets(&vfs) {
        for (name, path) in &samples {
            let clip = cook_library_clip(SOURCE, path, None, &rules(), &target).unwrap_or_else(|e| panic!("{name} on {skeleton}: {e}"));
            assert!(clip.duration > 0.0, "{name} on {skeleton}: empty clip");
            assert!(clip.root_motion.is_some(), "{name} on {skeleton}: no root motion");
            for t in times(&clip, 60) {
                let (pos, yaw) = clip.root_at(t);
                assert!(pos.is_finite() && yaw.is_finite(), "{name} on {skeleton}: root {pos} {yaw} at {t}");
                let model = world_at(&target.skeleton, &clip, t);
                assert!(model.iter().all(Affine3A::is_finite), "{name} on {skeleton}: non-finite pose at {t}");
            }
            // Root motion from the target pelvis: the clip starts over its root.
            assert!(clip.root_at(0.0).0.length() < 1e-3, "{name} on {skeleton}: starts at {}", clip.root_at(0.0).0);
        }
    }
}

/// `cook_library_clip` is the bake: a manifest listing the sample under a
/// test slot bakes the same `anim.retarget` bytes.
#[test]
fn in_process_clip_equals_the_baked_product() {
    let Some((install, vfs)) = install() else { return };
    let vfs = Arc::new(vfs);
    let Some(path) = sample_path("stand_still") else { return };
    let file = "samples/stand_still.bvh";
    let local = AnimSources::local();
    let mut manifest = local.manifest.clone();
    manifest.sources.get_mut(SOURCE).unwrap().files = vec![SourceFile { name: file.to_owned(), sha256: String::new() }];
    manifest.slots = BTreeMap::from([("test.generated".to_owned(), slot(file))]);
    let reference = manifest.slots["test.generated"].candidates[0].reference();
    let sources = Arc::new(AnimSources::new(manifest, local.dir.clone()));
    let virtual_path = AnimSources::virtual_path(SOURCE, file);
    assert_eq!(sources.real_path(&virtual_path).as_deref(), Some(path.as_path()));

    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path().join("store")).unwrap();
    let inputs = Arc::new(VfsInputs::new(vfs.clone(), &install, dir.path().join("filehash.memo")).with_anim_sources(sources.clone()));
    let mut baker = Baker::new(store, inputs);
    // The humanoid sets without their Morrowind files: only the library clip
    // cooks (the graph compile then lacks its required clips; not checked).
    let mut sets = ActorSets::default();
    for set in &HUMANOID_SETS[..TARGETS] {
        let s = ActorAnimSet {
            skeleton: set.skeleton.to_owned(),
            humanoid: true,
            graphs: BTreeSet::from([Template::Humanoid.path().to_owned()]),
            actors: 1,
            ..ActorAnimSet::default()
        };
        sets.sets.insert(s.skeleton.clone(), s);
    }
    let report = anim::bake_animation(&mut baker, &sets, &sources);
    assert_eq!(report.library_clips, TARGETS, "slots: {:?}; failures: {:?}", report.slots, report.failures);

    for (skeleton, target) in targets(&vfs) {
        let key = ProductKey::new(ANIM_RETARGET, retarget_subject(skeleton, &anim_subject(&virtual_path, &virtual_path, &reference)));
        let digest = baker.product(&key).unwrap_or_else(|e| panic!("{key}: {e}"));
        let mut from_bake = Clip::from_bytes(&baker.store().get(&digest).unwrap()).unwrap();
        let ours = cook_library_clip(SOURCE, &path, None, &rules(), &target).unwrap();
        from_bake.name.clone_from(&ours.name);
        assert!(from_bake.to_bytes() == ours.to_bytes(), "{skeleton}: the in-process clip differs from the baked one");
    }
}

/// Per clip and target: length, root travel, and how far the feet sit from
/// the floor. A standing foot's ankle should stay at its rest height above
/// the floor (y = 0 at the root); printed per foot as mean / max |error|
/// over the frames where that foot is the lower one, plus the lowest the
/// foot goes. The `*_seedrest` rows show Kimodo's default export.
#[test]
#[ignore = "measurement: generated clip length, root travel and standing-foot height error on the three skeletons"]
fn measure_generated_feet() {
    let Some((_, vfs)) = install() else { return };
    let names: Vec<&str> = SAMPLES.iter().chain(&SEED_REST).copied().collect();
    for (skeleton, target) in targets(&vfs) {
        let rest = model_of(&target.skeleton, target.skeleton.rest());
        // The model origin is the floor (Morrowind actors stand on it).
        let rest_y = [B::LeftFoot, B::RightFoot].map(|b| joint_pos(&target, &rest, b).y);
        let lowest = rest.iter().map(|m| m.translation.y).fold(f32::MAX, f32::min);
        println!("{skeleton}: rest ankle height L {:.3} R {:.3} m (lowest rest joint {lowest:+.3})", rest_y[0], rest_y[1]);
        for name in &names {
            let Some(path) = sample_path(name) else { continue };
            let clip = match cook_library_clip(SOURCE, &path, None, &rules(), &target) {
                Ok(c) => c,
                Err(e) => {
                    println!("  {name}: {e}");
                    continue;
                }
            };
            let ts = times(&clip, 300);
            let mut err = [(0.0f32, 0.0f32, 0usize); 2];
            let mut low = [f32::MAX; 2];
            for &t in &ts {
                let model = world_at(&target.skeleton, &clip, t);
                let y = [B::LeftFoot, B::RightFoot].map(|b| joint_pos(&target, &model, b).y);
                let lower = usize::from(y[1] < y[0]);
                let e = (y[lower] - rest_y[lower]).abs();
                err[lower] = (err[lower].0 + e, err[lower].1.max(e), err[lower].2 + 1);
                low = [low[0].min(y[0]), low[1].min(y[1])];
            }
            let (end, yaw) = clip.root_at(clip.duration);
            #[allow(clippy::cast_precision_loss)]
            let mean = |i: usize| if err[i].2 == 0 { 0.0 } else { err[i].0 / err[i].2 as f32 };
            println!(
                "  {name:22} {:.2} s, root end ({:+.2}, {:+.2}, {:+.2}) yaw {:+.1}°; standing ankle error L mean {:.3} max {:.3} m ({} frames), R mean {:.3} max {:.3} m ({}); lowest ankle L {:+.3} R {:+.3}",
                clip.duration,
                end.x,
                end.y,
                end.z,
                yaw.to_degrees(),
                mean(0),
                err[0].1,
                err[0].2,
                mean(1),
                err[1].1,
                err[1].2,
                low[0],
                low[1],
            );
        }
    }
}
