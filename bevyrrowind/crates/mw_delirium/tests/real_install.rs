//! The Delirium character on Morrowind's humanoid skeleton (`meshes/base_anim.nif`), headless. Skips (passes) when no
//! Morrowind install is detected. Sanity asserts only; quality numbers come from the `measurement` tests:
//! `cargo test -p mw_delirium --test real_install -- --ignored --nocapture`.
//!
//! Delirium's asset is process-global, so every test here uses one character. Variants for comparisons, read at
//! start: `MW_DELIRIUM_ARM_OUT=<deg>`, `MW_DELIRIUM_MASS=even`, `MW_DELIRIUM_SPAWN=zero`, `MW_DELIRIUM_SET_ZERO=1`.

#![allow(clippy::cast_precision_loss, clippy::many_single_char_names, clippy::format_push_string)]

use std::sync::OnceLock;
use std::time::Instant;

use delirium::prelude::*;
use mw_delirium::body::{JOINTS, MASS_PCT_EVEN, NAMES};
use mw_delirium::sim::{DT, Outcome, Sim, SpawnPose, Spawned, push_case, spawn};
use mw_delirium::{Body, Options, authoring_from_model, bones_to_parts, character_from_skeleton, pose_to_bones};
use mw_delirium::{delirium, sdk};
use mw_install::MorrowindInstall;
use sf_anim::{Skeleton, Trs};
use sf_vfs::{Vfs, VfsPath};

struct Fixture {
    skeleton: Skeleton,
    body: Body,
    spawned: Spawned,
    set_zero: bool,
}

fn env_f(k: &str) -> Option<f32> {
    std::env::var(k).ok().and_then(|v| v.parse().ok())
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
        let mut opt = Options::default();
        if let Some(a) = env_f("MW_DELIRIUM_ARM_OUT") {
            opt.arm_out_deg = a;
        }
        if std::env::var("MW_DELIRIUM_MASS").is_ok_and(|v| v == "even") {
            opt.mass_pct = MASS_PCT_EVEN;
        }
        let pose = if std::env::var("MW_DELIRIUM_SPAWN").is_ok_and(|v| v == "zero") { SpawnPose::Zero } else { SpawnPose::Bind };
        let set_zero = std::env::var("MW_DELIRIUM_SET_ZERO").is_ok_and(|v| v == "1");
        let body = character_from_skeleton(&skeleton, &opt).unwrap();
        let spawned = spawn(&body, pose, None);
        println!("character: arm out {} deg, mass {:?}, spawn {pose:?}, set_zero {set_zero}", opt.arm_out_deg, std::env::var("MW_DELIRIUM_MASS").ok());
        Some(Fixture { skeleton, body, spawned, set_zero })
    })
    .as_ref()
}

fn ang(a: Quat, b: Quat) -> f32 {
    a.angle_between(b).to_degrees()
}

#[test]
fn asset_round_trips_through_the_sdk() {
    let Some(f) = fixture() else { return };
    let text = f.body.json();
    let parsed = sdk::character::Character::parse(&text).unwrap();
    let (asset, parts, masses, pairs) = parsed.asset();
    assert_eq!((asset.len(), parts.len(), masses.len()), (20, 21, 21));
    assert!(!pairs.is_empty());
    for (a, b) in asset.iter().zip(&f.body.character.joints) {
        assert_eq!(a.name, b.name);
        assert!(a.limits.iter().all(|&(lo, hi)| lo <= hi && lo.is_finite() && hi.is_finite()), "{}", a.name);
    }
    let (build, frames) = parsed.build().unwrap();
    let (mut dp, mut dr) = (0.0f32, 0.0f32);
    for (a, b) in frames.iter().zip(&f.body.bind) {
        dp = dp.max((a.translation - b.translation).length());
        dr = dr.max(ang(a.rotation, b.rotation));
    }
    let kg: f32 = build.parts.iter().map(|p| p.mass).sum();
    let top = f.spawned.frames[12].translation.z + build.parts[12].radius + build.parts[12].half;
    println!(
        "round trip: build frames vs authored bind {:.3} mm / {dr:.3} deg; {kg:.1} kg, head top {top:.3} m, pelvis {:.3} m; bind bends knee {:.1} elbow {:.1} hip out {:.1} deg",
        dp * 1000.0,
        f.spawned.frames[0].translation.z,
        f.body.bind_bends[0],
        f.body.bind_bends[1],
        f.body.bind_bends[2]
    );
    assert!(dp < 0.001 && dr < 0.1, "build layout moved the bind pose: {dp} m, {dr} deg");
    assert!((1.4..2.1).contains(&top), "a human is 1.4-2.1 m tall, got {top}");
}

/// Each joint frame axis turns Delirium's angle it should (lean1 about y, lean2 about -x, twist about z), and every
/// angle is zero at the zero pose (skyrim-delirium's own check). Prints the bind-pose angles out of range.
#[test]
fn joint_axes_turn_the_right_way() {
    let Some(f) = fixture() else { return };
    let parsed = sdk::character::Character::parse(&f.body.json()).unwrap();
    let (mut asset, ..) = parsed.asset();
    let (b, frames) = parsed.build().unwrap();
    let real = delirium::nmrs::joints(&b, &asset).expect("joints");
    for jt in &mut asset {
        jt.limits = [(0.0, 0.0); 3];
        jt.flip = [false; 3];
    }
    let raw = delirium::nmrs::Character::new(delirium::nmrs::joints(&b, &asset).expect("joints"), &frames);
    let zf = &f.body.zero;
    let mut bad = vec![];
    for (i, name) in JOINTS.iter().enumerate() {
        let jd = &raw.joints[i];
        let w = zf[jd.parent].rotation * jd.in_parent;
        let base0 = raw.angle(i, zf);
        assert!(base0.iter().all(|v| v.abs() < 1e-3), "{name} not zero at the zero pose: {base0:?}");
        for (e, want) in [(Vec3::X, [0.0, -1.0, 0.0]), (Vec3::Y, [1.0, 0.0, 0.0]), (Vec3::Z, [0.0, 0.0, 1.0])] {
            if !jd.three && e != Vec3::Z {
                continue;
            }
            let want = if jd.three { want } else { [1.0, 0.0, 0.0] };
            let mut f2 = zf.clone();
            f2[jd.child].rotation = (Quat::from_axis_angle(w * e, 0.3) * zf[jd.child].rotation).normalize();
            let d = raw.angle(i, &f2);
            let got: Vec<f32> = (0..3).map(|c| d[c] / 0.3).collect();
            if (0..3).any(|c| (got[c] - want[c]).abs() > 0.05) {
                bad.push(format!("{name} {e:?}: got {got:.2?}, want {want:?}"));
            }
        }
        let base = raw.angle(i, &frames);
        let l = &real[i].limits;
        let n = if real[i].three { 3 } else { 1 };
        for c in 0..n {
            if base[c] < l[c].0 - 1e-3 || base[c] > l[c].1 + 1e-3 {
                println!("bind pose: {name} {} {:.0} not in {:.0}..{:.0}", ["lean1", "lean2", "twist"][c], base[c].to_degrees(), l[c].0.to_degrees(), l[c].1.to_degrees());
            }
        }
    }
    assert!(bad.is_empty(), "axes: {bad:#?}");
}

/// bodyBalance for 60 s: no NaN, nothing through the floor, still standing.
#[test]
fn balance_stands_60_s() {
    let Some(f) = fixture() else { return };
    let mut s = Sim::new(&f.spawned, 0.3, f.set_zero);
    let z0 = s.pelvis_z();
    s.start("bodyBalance");
    let (mut min, mut last_move, mut vmax) = (f32::MAX, 0.0f32, 0.0f32);
    let t0 = Instant::now();
    for k in 0..3600 {
        s.step();
        assert!(s.finite(), "NaN at step {k}");
        let z = s.pelvis_z();
        min = min.min(z);
        vmax = vmax.max(s.max_speed());
        assert!((0..21).all(|p| s.clearance(p) > -0.05), "through the floor at step {k}");
        if s.com().1.length() > 0.05 {
            last_move = (k + 1) as f32 * DT;
        }
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0 / 3600.0;
    let z = s.pelvis_z();
    println!(
        "bodyBalance 60 s: pelvis {:.0} % at the end (min {:.0} %), centre of mass last above 0.05 m/s at {last_move:.1} s, max part speed {vmax:.2} m/s, {ms:.3} ms/step",
        100.0 * z / z0,
        100.0 * min / z0
    );
    assert!(z > 0.7 * z0 && !s.agent.fallen, "fell: pelvis {z:.2} of {z0:.2}");
}

#[test]
#[ignore = "measurement: push sweep (8 directions x 20/45/80 N s at the chest), outcome and time to rest"]
fn push_sweep() {
    let Some(f) = fixture() else { return };
    println!("push sweep: 1 s bodyBalance, chest impulse, 15 s after (cells: outcome, rest [s], foot moves, min pelvis ratio)");
    println!("{:>5} | {:^26} | {:^26} | {:^26}", "dir", "20 Ns", "45 Ns", "80 Ns");
    let (mut counts, mut rests) = ([0u32; 4], vec![]);
    for d in 0..8 {
        let deg = d as f32 * 45.0;
        let mut row = format!("{deg:>5}");
        for mag in [20.0, 45.0, 80.0] {
            let r = push_case(&f.spawned, f.set_zero, deg, mag, 15.0);
            assert_ne!(r.outcome, Outcome::Exploded, "{deg} deg {mag} Ns");
            counts[r.outcome as usize] += 1;
            if let Some(t) = r.rest {
                rests.push(t);
            }
            let rest = r.rest.map_or("never".into(), |t| format!("{t:.1}"));
            row += &format!(" | {:<8} {rest:>5} {:>3} {:>5.2}", format!("{:?}", r.outcome), r.steps, r.min_ratio);
        }
        println!("{row}");
    }
    rests.sort_by(f32::total_cmp);
    println!(
        "stayed {} stepped {} fell {} exploded {}; came to rest {}/24 (median {:.1} s, max {:.1} s)",
        counts[0],
        counts[1],
        counts[2],
        counts[3],
        rests.len(),
        rests.get(rests.len() / 2).copied().unwrap_or(f32::NAN),
        rests.last().copied().unwrap_or(f32::NAN)
    );
}

#[test]
#[ignore = "measurement: forward falls with catchFall, hands before head"]
fn forward_fall_catches_with_the_hands() {
    let Some(f) = fixture() else { return };
    let mut first = 0;
    for v in 0..10 {
        let mut s = Sim::new(&f.spawned, 0.1 + 0.08 * v as f32, f.set_zero);
        s.start("bodyBalance");
        for _ in 0..30 {
            s.step();
        }
        let (fwd, right) = s.facing();
        let mag = 100.0 + 10.0 * v as f32;
        let a = ((v as f32 - 4.5) * 4.0).to_radians();
        s.push(10, (fwd * a.cos() + right * a.sin()) * mag);
        s.start("catchFall");
        let (mut hand, mut head, mut vmax) = (None, None, 0.0f32);
        for k in 0..240 {
            s.step();
            assert!(s.finite(), "NaN");
            vmax = vmax.max(s.max_speed());
            let t = k as f32 * DT;
            if hand.is_none() && (s.clearance(16) < 0.01 || s.clearance(20) < 0.01) {
                hand = Some(t);
            }
            if head.is_none() && s.clearance(12) < 0.01 {
                head = Some(t);
            }
        }
        let ok = match (hand, head) {
            (Some(a), Some(b)) => a < b,
            (Some(_), None) => true,
            _ => false,
        };
        first += u32::from(ok);
        let t = |o: Option<f32>| o.map_or("-".into(), |t| format!("{t:.2}"));
        println!("push {mag:3.0} Ns: hands {:>5} head {:>5} {} (max speed {vmax:.1} m/s)", t(hand), t(head), if ok { "hands first" } else { "HEAD FIRST/none" });
    }
    println!("forward fall: hands before head {first}/10");
}

fn local_rest(s: &Skeleton) -> Vec<Trs> {
    s.rest().to_vec()
}

fn bone_errors(sk: &Skeleton, a: &[Trs], b: &[Trs], joints: impl Iterator<Item = usize>) -> (f32, f32) {
    let (ma, mb) = (mw_delirium::pose::model_pose(sk, a), mw_delirium::pose::model_pose(sk, b));
    joints.fold((0.0f32, 0.0f32), |(p, r), j| {
        let (_, ra, ta) = ma[j].to_scale_rotation_translation();
        let (_, rb, tb) = mb[j].to_scale_rotation_translation();
        (p.max((ta - tb).length()), r.max(ang(ra, rb)))
    })
}

fn part_errors(a: &[Transform], b: &[Transform], skip: &[usize]) -> (f32, f32) {
    (0..21).filter(|k| !skip.contains(k)).fold((0.0f32, 0.0f32), |(p, r), k| {
        (p.max((a[k].translation - b[k].translation).length()), r.max(ang(a[k].rotation, b[k].rotation)))
    })
}

/// Bones -> parts -> bones (rest pose and a bent pose) is exact on every bone; parts from a pushed, stepping body ->
/// bones -> parts within 1 cm / 2 deg on every written part. At the rest pose the parts land on the bind frames.
#[test]
fn pose_copies_round_trip() {
    let Some(f) = fixture() else { return };
    let sk = &f.skeleton;
    let table = &f.body.table;
    let sim_from_model = f.spawned.sim_from_authoring.compute_affine() * authoring_from_model();
    let model_from_sim = sim_from_model.inverse();

    // rest pose: the parts are the spawned bind frames
    let rest = local_rest(sk);
    let parts = bones_to_parts(table, sk, &rest, &sim_from_model);
    let mut bind = f.body.character.build().unwrap().1;
    for p in &mut bind {
        p.translation += f.spawned.sim_from_authoring.translation;
    }
    let (dp, dr) = part_errors(&parts, &bind, &[]);
    println!("rest bones -> parts vs bind frames: {:.2} mm / {dr:.3} deg", dp * 1000.0);
    assert!(dp < 0.01 && dr < 2.0);

    // a bent pose (every owned bone turned a little) -> parts -> bones
    let mut bent = rest.clone();
    for (n, p) in table.parts.iter().enumerate() {
        bent[p.joint].rotation = (Quat::from_euler(EulerRot::XYZ, 0.2, -0.1 * n as f32 / 21.0, 0.15) * bent[p.joint].rotation).normalize();
    }
    let parts = bones_to_parts(table, sk, &bent, &sim_from_model);
    let mut back = rest.clone();
    pose_to_bones(table, sk, &parts, &model_from_sim, &mut back);
    let (dp, dr) = bone_errors(sk, &bent, &back, 0..sk.len());
    println!("bent bones -> parts -> bones: {:.3} mm / {dr:.3} deg (all {} joints)", dp * 1000.0, sk.len());
    assert!(dp < 0.01 && dr < 2.0);

    // a simulated pose (pushed, mid-step) -> bones -> parts
    let mut s = Sim::new(&f.spawned, 0.3, f.set_zero);
    s.start("bodyBalance");
    for _ in 0..60 {
        s.step();
    }
    let (fwd, _) = s.facing();
    s.push(10, fwd * 45.0);
    for _ in 0..20 {
        s.step();
    }
    let sim = s.frames();
    let mut bones = rest.clone();
    pose_to_bones(table, sk, &sim, &model_from_sim, &mut bones);
    let again = bones_to_parts(table, sk, &bones, &sim_from_model);
    let (dp, dr) = part_errors(&sim, &again, &[9]);
    let (dp9, dr9) = part_errors(&sim, &again, &(0..21).filter(|&k| k != 9).collect::<Vec<_>>());
    println!(
        "simulated parts -> bones -> parts: {:.2} mm / {dr:.3} deg (written parts); Spine2 (read only) {:.1} mm / {dr9:.2} deg",
        dp * 1000.0,
        dp9 * 1000.0
    );
    let worst = (0..21).filter(|&k| k != 9).max_by(|&a, &b| {
        (sim[a].translation - again[a].translation).length().total_cmp(&(sim[b].translation - again[b].translation).length())
    });
    if let Some(k) = worst {
        println!("  worst part {}", NAMES[k]);
    }
    assert!(dp < 0.01 && dr < 2.0, "{dp} m, {dr} deg");
}

#[test]
#[ignore = "measurement: ms per character step (bodyBalance)"]
fn ms_per_step() {
    let Some(f) = fixture() else { return };
    let mut s = Sim::new(&f.spawned, 0.3, f.set_zero);
    s.start("bodyBalance");
    let mut t: Vec<f64> = (0..1200)
        .map(|_| {
            let t0 = Instant::now();
            s.step();
            t0.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    t.sort_by(f64::total_cmp);
    let mean = t.iter().sum::<f64>() / t.len() as f64;
    println!("bodyBalance: {mean:.3} ms/step mean, {:.3} median, {:.3} p99", t[t.len() / 2], t[t.len() * 99 / 100]);
}

/// Per-actor size through the build (`Build::scaled`): a tall, heavy actor still stands 10 s.
#[test]
fn scaled_body_stands() {
    let Some(f) = fixture() else { return };
    let spawned = spawn(&f.body, SpawnPose::Bind, Some((1.1, 1.2)));
    let mut s = Sim::new(&spawned, 0.3, f.set_zero);
    let z0 = s.pelvis_z();
    s.start("bodyBalance");
    for k in 0..600 {
        s.step();
        assert!(s.finite(), "NaN at step {k}");
    }
    println!("scaled x1.1 (mass factor 1.2): pelvis {:.0} % after 10 s (standing pelvis {z0:.3} m)", 100.0 * s.pelvis_z() / z0);
    assert!(s.pelvis_z() > 0.7 * z0 && !s.agent.fallen);
}
