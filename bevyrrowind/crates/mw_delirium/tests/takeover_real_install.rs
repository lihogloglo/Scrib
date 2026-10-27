//! Takeovers (slice 2a) on Morrowind's humanoid skeleton (`meshes/base_anim.nif`), headless: from a pose, stand, hand
//! back; the click ray's part pick. Skips (passes) without a Morrowind install. Sanity asserts only; numbers from the
//! measurement: `cargo test -p mw_delirium --test takeover_real_install -- --ignored --nocapture`.
//!
//! One process per test binary: Delirium's asset is process-global, all tests use one character.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::BTreeSet;
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use glam::{Affine3A, Vec2};

use mw_delirium::delirium::prelude::*;
use mw_delirium::sim::DT;
use mw_delirium::takeover::pick;
use mw_delirium::{Body, HandBack, Options, Rules, Takeover, authoring_from_model, bones_to_parts, character_from_skeleton};
use mw_bake::VfsInputs;
use mw_bake::anim::{ActorAnimSet, ActorSets};
use mw_bake::sources::AnimSources;
use mw_graphs::build::GET_UPS;
use mw_graphs::{HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::steps::{ANIM_RETARGET, anim_subject, retarget_subject};
use sf_bake::{Baker, ProductKey, Store};
use sf_anim::Skeleton;
use sf_vfs::{Vfs, VfsPath};

struct Fixture {
    skeleton: Skeleton,
    body: Body,
}

fn fixture() -> Option<&'static Fixture> {
    static F: OnceLock<Option<Fixture>> = OnceLock::new();
    F.get_or_init(|| {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return None;
        };
        let vfs: Vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
        let bytes = vfs.read(&VfsPath::new("meshes/base_anim.nif").unwrap()).unwrap();
        let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
        let body = character_from_skeleton(&skeleton, &Options::default()).unwrap();
        Some(Fixture { skeleton, body })
    })
    .as_ref()
}

/// The rest pose's parts (sim = authoring space, the model's feet at z = 0), and the same a frame earlier moving
/// at `v` (sim space, m/s).
fn rest_parts(f: &Fixture, v: Vec3) -> (Vec<Transform>, Vec<Transform>) {
    let parts = bones_to_parts(&f.body.table, &f.skeleton, f.skeleton.rest(), &authoring_from_model());
    let prev = parts.iter().map(|p| Transform::from_translation(p.translation - v * DT).with_rotation(p.rotation)).collect();
    (parts, prev)
}

fn run(t: &mut Takeover, secs: f32) -> Option<(HandBack, f32)> {
    for _ in 0..(secs / DT) as u32 {
        if let Some(h) = t.step(DT) {
            return Some((h, t.time));
        }
    }
    None
}

#[test]
fn takeover_from_a_pose_stands_and_hands_back() {
    let Some(f) = fixture() else { return };
    let (parts, prev) = rest_parts(f, Vec3::new(0.05, 0.0, 0.0));
    let mut t = Takeover::start(&f.body, &parts, Some((&prev, DT)), None, 0.0, 0.3, Rules::default());
    assert_eq!(t.behaviour, "bodyBalance");
    assert!(t.floor <= 0.0 && t.floor > -0.1, "floor {}", t.floor);
    let out = run(&mut t, 20.0);
    println!("rest pose takeover: {out:?}, pelvis {:.2} m over the floor (stood {:.2})", t.frames()[0].translation.z - t.floor, t.stand);
    assert_eq!(out.map(|o| o.0), Some(HandBack::Balanced));
}

#[test]
fn pushed_takeover_ends_in_a_hand_back() {
    let Some(f) = fixture() else { return };
    let (parts, prev) = rest_parts(f, Vec3::ZERO);
    let mut t = Takeover::start(&f.body, &parts, Some((&prev, DT)), Some((1.05, 1.05f32.powi(3))), 0.0, 0.3, Rules::default());
    let chest = t.frames()[10].translation;
    t.push(10, chest, Vec3::new(0.0, -80.0, 0.0));
    let out = run(&mut t, 25.0);
    println!("80 N s from the front, scale 1.05: {out:?}, fell {}", t.fell());
    let (h, _) = out.expect("a hand-back within 25 s");
    assert_ne!(h, HandBack::Exploded);
}

#[test]
fn falling_pose_starts_catch_fall() {
    let Some(f) = fixture() else { return };
    let (mut parts, prev) = rest_parts(f, Vec3::new(0.0, 0.0, -3.0));
    for p in &mut parts {
        p.translation.z += 0.5;
    }
    let prev: Vec<Transform> = prev.iter().map(|p| Transform::from_translation(p.translation + Vec3::Z * 0.5).with_rotation(p.rotation)).collect();
    let mut t = Takeover::start(&f.body, &parts, Some((&prev, DT)), None, 0.0, 0.3, Rules::default());
    assert_eq!(t.behaviour, "catchFall");
    let out = run(&mut t, 10.0);
    println!("dropped from 0.5 m at 3 m/s: {out:?}, fell {}", t.fell());
    assert!(out.is_some_and(|o| o.0 != HandBack::Exploded));
}

#[test]
fn click_ray_picks_the_part_it_points_at() {
    let Some(f) = fixture() else { return };
    let (parts, _) = rest_parts(f, Vec3::ZERO);
    let (build, _) = f.body.character.build().unwrap();
    // from 3 m in front (the body faces +Y), straight at a part's centre
    for k in [0usize, 2, 5, 10, 12, 15] {
        let target = parts[k].translation;
        let origin = target + Vec3::Y * 3.0;
        let hit = pick(&build, &parts, origin, -Vec3::Y, 50.0);
        assert_eq!(hit.map(|h| h.0), Some(k), "part {k}");
        let (_, d, p) = hit.unwrap();
        assert!(d > 2.5 && d < 3.0 && (p - target).length() < 0.25, "part {k}: {d} {p}");
    }
    // past the body
    assert!(pick(&build, &parts, Vec3::new(2.0, 3.0, 1.0), -Vec3::Y, 50.0).is_none());
}

#[test]
#[ignore = "measurement: ms per takeover step (bodyBalance from the rest pose)"]
fn takeover_step_time() {
    let Some(f) = fixture() else { return };
    let (parts, prev) = rest_parts(f, Vec3::ZERO);
    let mut t = Takeover::start(&f.body, &parts, Some((&prev, DT)), None, 0.0, 0.3, Rules { max_secs: 1e9, ..Rules::default() });
    let n = 600;
    let t0 = Instant::now();
    for _ in 0..n {
        t.step(DT);
    }
    println!("takeover step: {:.3} ms/step over {n} steps", t0.elapsed().as_secs_f64() * 1000.0 / f64::from(n));
}

/// One pushed takeover's recovery: `(hand-back, s; shown at the idle, s; stance dwell, s; stance exposure, m s;
/// largest part off the idle at the blend start, m)`. The idle is the rest pose (arms down, as Morrowind's idle); "off the
/// idle" is the largest part distance from it, pelvis-relative. Dwell: from the last catch step's end (or the push)
/// to the blend start. Exposure: the shown share of that distance (`weight` x distance) integrated after the last step.
/// `inertialization`: the host's hand-back blend after an instant hand-back (counted as half the distance over it).
fn recovery(f: &Fixture, rules: Rules, inertialization: f32, dir: Vec3, impulse: f32) -> (Option<HandBack>, [f32; 5]) {
    let (parts, prev) = rest_parts(f, Vec3::ZERO);
    let rel = |p: &[Transform], k: usize| p[0].rotation.inverse() * (p[k].translation - p[0].translation);
    let off = |p: &[Transform]| (1..p.len()).map(|k| (rel(p, k) - rel(&parts, k)).length()).fold(0.0, f32::max);
    let mut t = Takeover::start(&f.body, &parts, Some((&prev, DT)), None, 0.0, 0.3, rules);
    let chest = t.frames()[10].translation;
    t.push(10, chest, dir * impulse);
    let (mut last_step, mut blend, mut exposure, mut tail, mut off_at_blend) = (0.0, None, 0.0, Vec::new(), 0.0);
    let mut how = None;
    for _ in 0..(25.0 / DT) as u32 {
        how = t.step(DT);
        let d = off(&t.frames());
        if t.stepping() == Some(true) {
            last_step = t.time;
            tail.clear();
        }
        tail.push(t.weight.min(1.0) * d * DT);
        if blend.is_none() && t.weight < 1.0 {
            blend = Some(t.time);
            off_at_blend = d;
        }
        if how.is_some() {
            exposure = tail.iter().sum::<f32>() + 0.5 * inertialization * d;
            break;
        }
    }
    let end = t.time;
    let dwell = blend.unwrap_or(end) - last_step;
    (how, [end, end + inertialization, dwell, exposure, off_at_blend])
}

#[test]
#[ignore = "measurement: recovery to the idle after a push, before (still, instant + 0.3 s) vs after (blend-out)"]
fn recovery_to_idle() {
    let Some(f) = fixture() else { return };
    let before = Rules { recover_secs: f32::INFINITY, blend_out: 0.0, ..Rules::default() };
    let after = Rules::default();
    println!("recovery after a chest push, 8 directions; Balanced runs: median / max of [hand-back s, at the idle s, stance dwell s, exposure m s, largest part off the idle at the blend start m]");
    for (name, rules, inert) in [("before", before, 0.3), ("after", after, 0.0)] {
        for impulse in [20.0, 45.0, 80.0] {
            let mut rows: Vec<[f32; 5]> = Vec::new();
            let mut other = Vec::new();
            for k in 0..8 {
                let a = k as f32 * std::f32::consts::FRAC_PI_4;
                let (how, row) = recovery(f, rules, inert, Vec3::new(a.sin(), a.cos(), 0.0), impulse);
                assert!(how.is_some_and(|h| h != HandBack::Exploded), "{name} {impulse} N s dir {k}: {how:?}");
                assert!(row.iter().all(|v| v.is_finite()));
                if how == Some(HandBack::Balanced) {
                    rows.push(row);
                } else {
                    other.push(how);
                }
            }
            let stat = |i: usize| {
                let mut v: Vec<f32> = rows.iter().map(|r| r[i]).collect();
                v.sort_by(f32::total_cmp);
                v.get(v.len() / 2).map_or(String::from("-"), |m| format!("{m:.2}/{:.2}", v[v.len() - 1]))
            };
            println!(
                "  {name:6} {impulse:>2.0} N s: {} balanced, other {other:?}; hand-back {}, idle {}, dwell {}, exposure {}, off {}",
                rows.len(),
                stat(0),
                stat(1),
                stat(2),
                stat(3),
                stat(4)
            );
        }
    }
}

/// Audit F5: on a 15 deg ramp (rising across the stance, +X), taken over 0.1 m above the ground under the pelvis (the
/// mover's render height lagging a step down). The host's floor is the ground under the pelvis (one ray), lowered to
/// the lowest part; the frame sits on it, so at the hand-back the root takes the floor height and the pelvis only
/// blends from its settled height in the frame to the graph's (the rest pose's).
#[test]
fn ramp_takeover_floor_at_the_ground_root_takes_the_height() {
    let Some(f) = fixture() else { return };
    let (parts, prev) = rest_parts(f, Vec3::ZERO);
    let (slope, lag) = (15f32.to_radians().tan(), 0.1);
    let ground = |p: Vec3| slope * p.x - lag;
    let pelvis = parts[0].translation;
    let probed = ground(pelvis);
    let mut t = Takeover::start(&f.body, &parts, Some((&prev, DT)), None, probed, 0.3, Rules::default());
    let out = run(&mut t, 20.0);
    let settled = t.frames()[0].translation;
    // Foot_Left, Foot_Right: their height and the ramp's under them.
    let feet: Vec<(f32, f32)> = [3usize, 6].iter().map(|&k| t.frames()[k].translation).map(|p| (p.z, ground(p))).collect();
    println!(
        "15 deg ramp, 0.1 m lag: {out:?}; ground under the pelvis {probed:+.3}, floor {:+.3}; pelvis in the frame (on the \
         floor): {:.3} at the takeover, {:.3} settled, {:.3} 0.3 s after the hand-back (graph); in the takeover's render \
         frame (before F5): {:.3} settled; root moves {:+.3}; feet z (ramp under them) {:+.3} ({:+.3}), {:+.3} ({:+.3})",
        t.floor,
        pelvis.z - t.floor,
        settled.z - t.floor,
        pelvis.z,
        settled.z,
        t.floor,
        feet[0].0,
        feet[0].1,
        feet[1].0,
        feet[1].1,
    );
    assert!(out.is_some_and(|o| o.0 != HandBack::Exploded), "{out:?}");
    assert!(t.floor <= probed && t.floor.is_finite());
    assert!(t.frames().iter().all(|p| p.translation.is_finite() && p.translation.z > t.floor - 0.05), "through the floor");
    assert!(settled.z > t.floor, "pelvis under the floor");
}

/// The get-up clip's first frame, local pose: `Knockout@tail` (the `Knockout` group at its `Loop Stop`), cooked in
/// place like the graph's (no root-motion extraction, anchored on the last frame: `mw_anim::root_motion_rules`).
fn get_up_pose(f: &Fixture) -> (Vec<sf_anim::Trs>, f32, sf_anim::Clip, f32) {
    let (install, _) = MorrowindInstall::detect(None).unwrap();
    let vfs: Vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let anim = mw_anim::Animation::from_kf(&vfs.read(&VfsPath::new("meshes/xbase_anim.kf").unwrap()).unwrap()).unwrap();
    let group = anim.groups().into_iter().find(|g| g.name.eq_ignore_ascii_case("Knockout")).expect("Knockout");
    let (_, tail) = group.loop_range.expect("Knockout loops");
    let rules = sf_anim::ClipRules {
        looping: sf_anim::Looping::Once,
        root_motion: sf_anim::RootMotionRules { extract: false, anchor_end: true, ..Default::default() },
        ..Default::default()
    };
    let clip = sf_anim::cook_clip(&f.skeleton, &anim.source_clip(&group), &rules, &sf_anim::CookSettings::default()).unwrap();
    let mut local = sf_anim::LocalPose::rest(&f.skeleton);
    clip.sample(tail, &mut local, None);
    (local.to_vec(), clip.duration - tail, clip, tail)
}

/// A pack get-up: slot, first frame, duration s, clip.
type PackGetUp = (&'static str, Vec<sf_anim::Trs>, f32, sf_anim::Clip);

/// The packs' get-ups on `base_anim` (the sources manifest's `getup.*` slots, retargeted by the animation bake as the
/// game's graph gets them): (slot, first frame, duration s, clip); slots the manifest does not list are skipped.
/// Each slot takes its first candidate in the library (Quaternius, then LAFAN1). `None` without any.
fn pack_get_ups(f: &Fixture) -> Option<Vec<PackGetUp>> {
    let (install, _) = MorrowindInstall::detect(None).ok()?;
    let sources = Arc::new(AnimSources::local());
    let refs = sources.references();
    let mut wanted = Vec::new();
    for (_, slot) in GET_UPS {
        // The slot's first candidate whose file is in the library, as the bake picks it.
        let candidates = sources.manifest.slots.get(slot).map(|s| s.candidates.iter().map(mw_graphs::sources::Candidate::reference).collect::<Vec<_>>());
        let found = candidates.unwrap_or_default().into_iter().find_map(|reference| {
            let (file, _) = refs.iter().find(|(_, r)| r.reference == reference)?.clone();
            sources.real_path(&file).map(|_| (file, reference))
        });
        let Some((file, reference)) = found else {
            eprintln!("{slot}: no candidate file in {}", sources.dir.display());
            continue;
        };
        println!("{slot}: {reference}");
        wanted.push((slot, file, reference));
    }
    if wanted.is_empty() {
        return None;
    }
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs);
    let dir = tempfile::tempdir().unwrap();
    let inputs = Arc::new(VfsInputs::new(vfs, &install, dir.path().join("filehash.memo")).with_anim_sources(sources.clone()));
    let mut baker = Baker::new(Store::open(dir.path().join("store")).unwrap(), inputs);
    let set = &HUMANOID_SETS[0];
    let mut sets = ActorSets::default();
    let actor = ActorAnimSet {
        skeleton: set.skeleton.to_owned(),
        files: set.animations.iter().map(|f| (*f).to_owned()).collect(),
        humanoid: true,
        graphs: BTreeSet::from([Template::Humanoid.path().to_owned()]),
        actors: 1,
        ..ActorAnimSet::default()
    };
    sets.sets.insert(actor.skeleton.clone(), actor);
    let report = mw_bake::anim::bake_animation(&mut baker, &sets, &sources);
    assert!(report.failures.is_empty(), "{:?}", report.failures);
    let out = wanted
        .into_iter()
        .map(|(slot, file, reference)| {
            let key = ProductKey::new(ANIM_RETARGET, retarget_subject(set.skeleton, &anim_subject(&file, &file, &reference)));
            let digest = baker.product(&key).unwrap_or_else(|e| panic!("{key}: {e}"));
            let clip = sf_anim::Clip::from_bytes(&baker.store().get(&digest).unwrap()).unwrap();
            let mut local = sf_anim::LocalPose::rest(&f.skeleton);
            clip.sample(0.0, &mut local, None);
            (slot, local.to_vec(), clip.duration, clip)
        })
        .collect();
    Some(out)
}

/// One fall: 200 N s on `part` along `dir`, then the takeover runs to its hand-back: (hand-back, seconds after the
/// push, down s, rolled s, the picked get-up, the blend's whole-body sweep in deg, each clip's tilt angle at the pick
/// in deg).
type Fall = (Option<HandBack>, f32, f32, f32, Option<usize>, f32, Vec<f32>);

fn fall(f: &Fixture, rules: Rules, tilts: &[Vec3], part: usize, dir: Vec3) -> Fall {
    let (rest, prev) = rest_parts(f, Vec3::ZERO);
    let mut t = Takeover::start(&f.body, &rest, Some((&prev, DT)), None, 0.0, 0.3, rules);
    t.set_get_ups(tilts);
    let point = t.frames()[part].translation;
    t.push(part, point, dir * 200.0);
    let out = run(&mut t, 30.0);
    let frames = t.frames();
    assert!(out.is_some_and(|o| o.0 != HandBack::Exploded), "{out:?}");
    assert!(frames.iter().all(|p| p.translation.is_finite() && p.translation.z > t.floor - 0.05), "through the floor");
    let sweep = t.get_up_left().unwrap_or(0.0).to_degrees();
    assert!(sweep.is_finite());
    let angles = t.get_up_angles().iter().map(|a| a.to_degrees()).collect();
    (out.map(|o| o.0), out.map_or(0.0, |o| o.1), t.down.unwrap_or(0.0), t.rolling.unwrap_or(0.0), t.get_up(), sweep, angles)
}

/// The rules before DE-1's get-up pass (2026-10-27): down 1.5..4 s, the balancer running on the floor.
fn rules_before() -> Rules {
    Rules { down_secs: (1.5, 4.0), rest: (0.05, 0.5), down_relax: None, ..Rules::default() }
}

/// The pelvis (Delirium's pelvis part joint) of `clip` at `t`, model space, m.
fn pelvis_at(f: &Fixture, clip: &sf_anim::Clip, t: f32) -> Vec3 {
    let mut local = sf_anim::LocalPose::rest(&f.skeleton);
    clip.sample(t, &mut local, None);
    let mut model = sf_anim::ModelPose::new(f.skeleton.len());
    sf_anim::local_to_model(&f.skeleton, &local, &mut model, &Affine3A::IDENTITY);
    Vec3::from(model.mats[f.body.table.parts[0].joint].translation)
}

/// DE-1 "starts walking after standing up": the get-up after the hand-back, played from `start` to its end in place
/// (the root moves only by the clip's root motion), then the idle at the frame's origin. Once up (the pelvis first at
/// 90% of its final height): how long the clip runs on, the pelvis' horizontal travel and peak speed then (steps after
/// standing read as walking); how far it ends off the frame (the idle's pop or slide) and the root's drift over the
/// clip: (up s before the end, travel m, peak m/s, off m, root m).
fn after_get_up(f: &Fixture, clip: &sf_anim::Clip, start: f32) -> [f32; 5] {
    const RATE: f32 = 30.0;
    let end = clip.duration;
    let n = ((end - start) * RATE).ceil().max(1.0) as u32;
    let dt = (end - start) / n as f32;
    let pelvis: Vec<Vec3> = (0..=n).map(|i| pelvis_at(f, clip, start + dt * i as f32)).collect();
    let height = pelvis[n as usize].y;
    let up = pelvis.iter().position(|p| p.y >= 0.9 * height).unwrap_or(n as usize);
    let ground = |p: Vec3| Vec2::new(p.x, p.z);
    let steps = pelvis[up..].windows(2).map(|w| ground(w[1]).distance(ground(w[0])));
    let (travel, peak) = steps.fold((0.0f32, 0.0f32), |(t, p), d| (t + d, p.max(d / dt.max(1e-4))));
    let out = [(n as usize - up) as f32 * dt, travel, peak, ground(pelvis[n as usize]).length(), clip.root_delta(start, end).translation.length()];
    assert!(out.iter().all(|v| v.is_finite() && *v < 20.0), "get-up blew up: {out:?}");
    out
}

/// DE-1 "magic flip", "lies too long", "Morrowind's get-up unused" and "walks after standing up": the whole-body
/// (pelvis) rotation the hand-back's 0.3 s blend sweeps at a fallen get-up (the tilt left between the lying pelvis and
/// the picked clip's first pose after the yaw-only get-up frame), and push-to-standing (the hand-back plus the clip).
/// Morrowind's `Knockout@tail` alone (one clip on its side: the body rolls over to it) before (down 1.5..4 s, no relax)
/// and after (down 0.5..2.5 s, relaxed); then, with the LAFAN1 pack, the game's set (`Knockout@tail` first, the
/// baseline within `Rules::baseline_margin`, then LAFAN1's get-ups) with the pick and its gaps, and LAFAN1 alone.
/// Each clip's tail: what the body does in its last 2 s once up.
#[test]
#[ignore = "measurement: get-up blend rotation, push-to-standing, the pick, and drift after the get-up"]
fn get_up_blend_rotation() {
    let Some(f) = fixture() else { return };
    let (clip_pose, tail, tail_clip, tail_start) = get_up_pose(f);
    let mw = [mw_delirium::clip_tilt(&f.body.table, &f.skeleton, &clip_pose)];
    let lafan = pack_get_ups(f);
    let lafan_tilts: Vec<Vec3> = lafan.iter().flatten().map(|(_, pose, _, _)| mw_delirium::clip_tilt(&f.body.table, &f.skeleton, pose)).collect();
    let both: Vec<Vec3> = mw.iter().chain(&lafan_tilts).copied().collect();
    let names: Vec<&str> = std::iter::once("Knockout@tail").chain(lafan.iter().flatten().map(|l| l.0)).collect();
    let drift = |[up, travel, peak, off, root]: [f32; 5]| {
        format!("up {up:.2} s before the end, then pelvis travel {travel:.2} m, peak {peak:.2} m/s; ends {off:.2} m off the frame; root drift {root:.2} m")
    };
    println!("Knockout@tail: {tail:.2} s, pelvis tilt {:.2}; {}", mw[0], drift(after_get_up(f, &tail_clip, tail_start)));
    for ((slot, _, secs, clip), tilt) in lafan.iter().flatten().zip(&lafan_tilts) {
        let from_tail = tilt.angle_between(mw[0]).to_degrees();
        println!("{slot}: {secs:.2} s, pelvis tilt {tilt:.2}, {from_tail:.0} deg from Knockout@tail; {}", drift(after_get_up(f, clip, 0.0)));
    }
    println!("per push: sweep deg | push-to-standing s (hand-back + clip; down s, rolled s; the pick at its tilt deg, the others' gaps deg)");
    // The body faces sim +Y: a push toward -Y is from the front.
    let dirs = [("front", -Vec3::Y), ("behind", Vec3::Y), ("left", Vec3::X), ("right", -Vec3::X)];
    let mut totals = [0.0f32; 4];
    let mut n = [0.0f32; 4];
    let mut wins = vec![0u32; names.len()];
    for (part, at) in [(10usize, "chest"), (12, "head"), (0, "pelvis")] {
        for (name, dir) in dirs {
            let mut row = Vec::new();
            let cases: [(&str, Rules, &[Vec3]); 4] = [
                ("MW before", rules_before(), &mw),
                ("MW after", Rules::default(), &mw),
                ("MW+packs", Rules::default(), if lafan.is_some() { &both } else { &[] }),
                ("packs", Rules::default(), &lafan_tilts),
            ];
            for (k, (label, rules, tilts)) in cases.into_iter().enumerate() {
                if tilts.is_empty() {
                    continue;
                }
                let (how, secs, down, rolled, pick, sweep, angles) = fall(f, rules, tilts, part, dir);
                if how != Some(HandBack::Fallen) {
                    row.push(format!("{label}: {how:?}"));
                    continue;
                }
                let i = pick.unwrap_or(0);
                // The pick's name and duration: `names` indexes the game's set; LAFAN1 alone has no Knockout@tail.
                let (clip, clip_secs) = match (k, &lafan) {
                    (2, Some(l)) if i > 0 => (names[i], l[i - 1].2),
                    (3, Some(l)) => (l[i].0, l[i].2),
                    _ => ("Knockout@tail", tail),
                };
                totals[k] += secs + clip_secs;
                n[k] += 1.0;
                let picked = match k {
                    2 => {
                        wins[i] += 1;
                        let won = angles.get(i).copied().unwrap_or(0.0);
                        let gaps: Vec<String> =
                            angles.iter().enumerate().filter(|&(j, _)| j != i).map(|(j, a)| format!("{} {:+.0}", names[j], a - won)).collect();
                        format!("; {clip} at {won:.0}: {}", gaps.join(" "))
                    }
                    3 => format!("; {clip}"),
                    _ => String::new(),
                };
                row.push(format!("{label}: {sweep:5.1} | {:4.1} (down {down:.1}, rolled {rolled:.2}{picked})", secs + clip_secs));
            }
            println!("  {at:6} {name:6}: {}", row.join("; "));
        }
    }
    let mean = |k: usize| if n[k] > 0.0 { totals[k] / n[k] } else { f32::NAN };
    println!("mean push-to-standing s: MW before {:.1}, MW after {:.1}, MW+packs {:.1}, packs {:.1}", mean(0), mean(1), mean(2), mean(3));
    let tally: Vec<String> = names.iter().zip(&wins).map(|(c, w)| format!("{c} {w}")).collect();
    println!("MW+packs picks (of {}): {}", n[2], tally.join(", "));
}

/// DE-1 "a bit too much vibration": the fastest part (m/s) and the fastest-turning part (rad/s) while the body lies
/// still (down 1.5..3 s, centre of mass under 0.1 m/s, the get-up held off), the takeover's balancer running on the floor (before) vs relaxed
/// (`Rules::down_relax`, after).
#[test]
#[ignore = "measurement: part speed while lying still, balancer running vs relaxed"]
fn lying_vibration() {
    let Some(f) = fixture() else { return };
    let dirs = [("front", -Vec3::Y), ("behind", Vec3::Y), ("left", Vec3::X), ("right", -Vec3::X)];
    for (label, relax) in [("before (balancer)", None), ("after (relaxed)", Rules::default().down_relax)] {
        let mut worst = (0.0f32, 0.0f32);
        let mut row = Vec::new();
        for (name, dir) in dirs {
            let rules = Rules { down_secs: (10.0, 10.0), down_relax: relax, ..Rules::default() };
            let (rest, prev) = rest_parts(f, Vec3::ZERO);
            let mut t = Takeover::start(&f.body, &rest, Some((&prev, DT)), None, 0.0, 0.3, rules);
            let point = t.frames()[10].translation;
            t.push(10, point, dir * 200.0);
            let (mut v, mut w) = (0.0f32, 0.0f32);
            let mut last = t.frames();
            for _ in 0..(8.0 / DT) as u32 {
                assert_ne!(t.step(DT), Some(HandBack::Exploded));
                let now = t.frames();
                if t.down.is_some_and(|d| (1.5..3.0).contains(&d)) && t.com().1.length() < 0.1 {
                    for (a, b) in last.iter().zip(&now) {
                        v = v.max((b.translation - a.translation).length() / DT);
                        w = w.max(b.rotation.angle_between(a.rotation) / DT);
                    }
                }
                last = now;
            }
            assert!(last.iter().all(|p| p.translation.is_finite()));
            if t.down.is_none() {
                row.push(format!("{name}: stood"));
                continue;
            }
            worst = (worst.0.max(v), worst.1.max(w));
            row.push(format!("{name}: {v:.3} m/s {w:.2} rad/s"));
        }
        println!("{label}: {}; worst {:.3} m/s {:.2} rad/s", row.join(", "), worst.0, worst.1);
    }
}
