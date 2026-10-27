//! Takeovers (slice 2a) on Morrowind's humanoid skeleton (`meshes/base_anim.nif`), headless: from a pose, stand, hand
//! back; the click ray's part pick. Skips (passes) without a Morrowind install. Sanity asserts only; numbers from the
//! measurement: `cargo test -p mw_delirium --test takeover_real_install -- --ignored --nocapture`.
//!
//! One process per test binary: Delirium's asset is process-global, all tests use one character.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::sync::OnceLock;
use std::time::Instant;

use mw_delirium::delirium::prelude::*;
use mw_delirium::sim::DT;
use mw_delirium::takeover::pick;
use mw_delirium::{Body, HandBack, Options, Rules, Takeover, authoring_from_model, bones_to_parts, character_from_skeleton};
use mw_install::MorrowindInstall;
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
