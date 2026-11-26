//! Root-motion census over every creature skeleton on the real game data:
//! each walk/run/swim clip cooked as the bake cooks it (`mw_anim::rig`
//! skeleton, the group's `clip_rules`, `anim.cook` default root-motion
//! rules), with the distance extracted as root motion over one loop and
//! the travel left in the bones. A clip whose joints drift over the loop
//! while less was extracted keeps its travel in the bones: the root-motion
//! joint is wrong, and the creature walks off and snaps back at the loop
//! instead of playing in place (owner note on NC-1, 2026-10-12).
//! Skips (passes) without a Morrowind install. `-- --nocapture` prints the
//! census.
#![allow(clippy::too_many_lines)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use glam::{Affine3A, Vec3};
use mw_anim::Animation;
use mw_install::MorrowindInstall;
use sf_anim::{Clip, CookSettings, LocalPose, ModelPose, Skeleton, cook_clip, local_to_model};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};

/// Travel left in the bones over one loop above this (shared by nine joints
/// in ten, ground plane) means the extraction missed it. Measured: 0.00-0.08 m on
/// every clip once extracted (loop seams of secondary chains), 1.1-82 m
/// for the `Root Bone` creatures before.
const BONE_TRAVEL_LIMIT: f32 = 0.10;

fn is_locomotion(group: &str) -> bool {
    let g = group.to_ascii_lowercase();
    ["walk", "run", "swimwalk", "swimrun"].iter().any(|p| g.starts_with(p))
}

/// Model poses (root motion removed) at `t0` and `t1`.
fn model_poses(skeleton: &Skeleton, clip: &Clip, t0: f32, t1: f32) -> [ModelPose; 2] {
    let n = skeleton.len();
    let mut model = [ModelPose::new(n), ModelPose::new(n)];
    for (m, t) in model.iter_mut().zip([t0, t1]) {
        let mut local = LocalPose::rest(skeleton);
        clip.sample(t, &mut local, None);
        local_to_model(skeleton, &local, m, &Affine3A::IDENTITY);
    }
    model
}

/// Ground-plane drift over `[t0, t1]` that nine joints in ten of the body
/// share, the body's median drift, and the shallowest joint that drifts by
/// the shared amount (the joint carrying the travel). The body is the
/// largest branch under the model root (rigid side branches, such as the
/// storm atronach's rocks, don't count); the 10th percentile, because
/// travel left in the bones moves the whole body and a loop seam only some
/// limbs.
fn bone_travel(skeleton: &Skeleton, clip: &Clip, t0: f32, t1: f32) -> (f32, f32, String) {
    let n = skeleton.len();
    let model = model_poses(skeleton, clip, t0, t1);
    let drift: Vec<f32> = (0..n)
        .map(|j| {
            let d = model[1].translation(j) - model[0].translation(j);
            Vec3::new(d.x, 0.0, d.z).length()
        })
        .collect();
    // Each joint's branch: its ancestor just under the model root.
    let branch = |mut j: usize| {
        while let Some(p) = skeleton.parent(j).filter(|&p| skeleton.parent(p).is_some()) {
            j = p;
        }
        j
    };
    let branches: Vec<usize> = (0..n).map(branch).collect();
    let body = (1..n).max_by_key(|&b| branches.iter().filter(|&&x| x == b).count()).unwrap_or(0);
    let mut sorted: Vec<f32> = (0..n).filter(|&j| branches[j] == body).map(|j| drift[j]).collect();
    sorted.sort_by(f32::total_cmp);
    let (shared, median) = (sorted[sorted.len() / 10], sorted[sorted.len() / 2]);
    let depth = |mut j: usize| {
        let mut d = 0;
        while let Some(p) = skeleton.parent(j) {
            d += 1;
            j = p;
        }
        d
    };
    let carrier = (0..n).filter(|&j| drift[j] >= shared * 0.9 && shared > 0.0).min_by_key(|&j| depth(j));
    (shared, median, carrier.map_or_else(String::new, |j| skeleton.name(j).to_owned()))
}

#[test]
fn creature_locomotion_travel_is_extracted_as_root_motion() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs);
    let db: Arc<RecordDb> = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
    let sets = mw_bake::anim::actor_sets(&db, &vfs);
    let read = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.read(&p).ok());

    let mut rows = Vec::new();
    let mut flagged = Vec::new();
    let mut by_joint: BTreeMap<String, usize> = BTreeMap::new();
    let mut in_place = Vec::new();
    let mut controls = BTreeSet::new();
    for set in sets.sets.values().filter(|s| sets.bodies.contains(&s.skeleton)) {
        let Some(model) = read(&set.skeleton).and_then(|b| mw_nif::load(&b).ok()) else { continue };
        let skeleton = mw_anim::rig::skeleton(&model).unwrap();
        let rm = skeleton.root_motion_joint().map_or("-".to_owned(), |j| skeleton.name(j).to_owned());
        *by_joint.entry(rm.clone()).or_default() += 1;
        let mut seen = BTreeSet::new();
        // Highest-priority file first: a group comes from the first file that has it.
        for file in &set.files {
            let Some(animation) = read(file).and_then(|b| Animation::from_file(file, &b).ok()) else { continue };
            for group in animation.groups().iter().filter(|g| is_locomotion(&g.name)) {
                if !seen.insert(group.name.to_ascii_lowercase()) {
                    continue;
                }
                let source = animation.source_clip(group);
                let Ok(clip) = cook_clip(&skeleton, &source, &mw_anim::clip_rules(group), &CookSettings::default()) else {
                    flagged.push(format!("{} {}: cook failed", set.skeleton, group.name));
                    continue;
                };
                let (t0, t1) = clip.loop_range.unwrap_or((0.0, clip.duration));
                let extracted = Vec3::new(clip.root_delta(t0, t1).translation.x, 0.0, clip.root_delta(t0, t1).translation.z).length();
                let speed = if t1 > t0 { extracted / (t1 - t0) } else { 0.0 };
                let (bones, median, carrier) = bone_travel(&skeleton, &clip, t0, t1);
                let bad = bones > BONE_TRAVEL_LIMIT;
                rows.push(format!(
                    "{:<40} {:<16} {:<10} loop {:>5.2} s  extracted {extracted:>6.2} m ({speed:>5.2} m/s)  bones {bones:>5.2} m (median {median:.2}){}",
                    set.skeleton,
                    group.name,
                    rm,
                    t1 - t0,
                    if bad { format!("  <-- TRAVEL IN BONES (on {carrier})") } else { String::new() }
                ));
                if bad {
                    flagged.push(format!("{} {}: {bones:.2} m left in the bones (root-motion joint {rm}, travel on {carrier})", set.skeleton, group.name));
                }
                // Control, once per skeleton: the same clip cooked without
                // extraction must flag (the metric sees travel in the bones).
                if extracted > 0.5 && controls.insert(set.skeleton.clone()) {
                    let mut rules = mw_anim::clip_rules(group);
                    rules.root_motion.extract = false;
                    let raw = cook_clip(&skeleton, &source, &rules, &CookSettings::default()).unwrap();
                    let (kept, ..) = bone_travel(&skeleton, &raw, t0, t1);
                    assert!(kept > 0.8 * extracted, "{} {}: control kept {kept:.2} m of {extracted:.2} m in the bones", set.skeleton, group.name);
                }
                if extracted < 0.05 && !bad {
                    in_place.push(format!("{} {}", set.skeleton, group.name));
                }
            }
        }
    }
    eprintln!("creature locomotion clips: {} (skeletons by root-motion joint: {by_joint:?})", rows.len());
    for r in &rows {
        eprintln!("{r}");
    }
    eprintln!("authored in place (no travel anywhere; speed comes from planted feet): {}", in_place.len());
    for c in &in_place {
        eprintln!("  {c}");
    }
    eprintln!("flagged: {}", flagged.len());
    for f in &flagged {
        eprintln!("  {f}");
    }
    assert!(rows.len() > 400, "walk/run clips found: {}", rows.len());
    assert!(!by_joint.contains_key("-"), "every creature skeleton has a root-motion joint: {by_joint:?}");
    assert!(flagged.is_empty(), "{} clips keep their travel in the bones:\n{}", flagged.len(), flagged.join("\n"));
}
