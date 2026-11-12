//! Loop cutter on the generated loco1 cycles (measurement only).
//!
//! Every `keep`/`maybe` take of `loco.{walk,run,sprint}.cycle.*` (incl. the
//! `.free` groups) in `<library>/generated/loco1/verdicts.toml`, cooked with
//! `ClipRules::loop_cut` through [`mw_bake::anim::cook_library_clip_report`]
//! (the bake's cook + retarget onto the Morrowind humanoid): per take the
//! window (source s), strides, the seam before the fade (hard cut) and after
//! it (pose RMS and contact-foot jump in cm, root ground-speed jump in m/s),
//! the loop's speed against the steady part's and the prompt's root-path
//! speed (`.prov.json`), the turn per loop, and the retargeted loop's speed.
//! Run with `-- --ignored --nocapture`; skips without a Morrowind install or
//! the files.
#![allow(clippy::cast_precision_loss)]

use std::path::Path;

use mw_bake::anim::{HumanoidTarget, cook_library_clip_report};
use mw_bake::sources::AnimSources;
use mw_graphs::HUMANOID_SETS;
use mw_install::MorrowindInstall;
use sf_anim::{ClipRules, LoopCutRules, Looping, RootMotionMode, RootMotionRules};
use sf_vfs::{Vfs, VfsPath};

/// The prompt's root-path speed: the path constraint's length over its time.
fn target_speed(prov: &Path) -> Option<f32> {
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(prov).ok()?).ok()?;
    let root = json.get("constraints")?.get("root")?;
    let frames = root.get("frames")?.as_array()?;
    let points = root.get("points")?.as_array()?;
    let fps = json.get("fps")?.as_f64()?;
    let point = |v: &serde_json::Value| -> Option<(f64, f64)> { Some((v.get(0)?.as_f64()?, v.get(1)?.as_f64()?)) };
    let (mut length, mut last) = (0.0, point(points.first()?)?);
    for p in points.iter().skip(1) {
        let q = point(p)?;
        length += ((q.0 - last.0).powi(2) + (q.1 - last.1).powi(2)).sqrt();
        last = q;
    }
    let span = (frames.last()?.as_f64()? - frames.first()?.as_f64()?) / fps;
    #[allow(clippy::cast_possible_truncation)]
    (span > 0.0).then(|| (length / span) as f32)
}

#[test]
#[ignore = "measurement: loop cutter window and seam errors on the loco1 keep/maybe cycles"]
fn measure_loop_cut_loco1() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let loco = AnimSources::local().dir.join("generated").join("loco1");
    let Ok(verdicts) = std::fs::read_to_string(loco.join("verdicts.toml")) else {
        eprintln!("{} missing; skipping", loco.join("verdicts.toml").display());
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    let bytes = vfs.read(&VfsPath::new(HUMANOID_SETS[0].skeleton).unwrap()).unwrap();
    let target = HumanoidTarget::new(mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap()).unwrap();
    let rules = ClipRules {
        looping: Looping::Whole,
        root_motion: RootMotionRules { mode: RootMotionMode::Match, ..RootMotionRules::default() },
        loop_cut: Some(LoopCutRules::default()),
        ..ClipRules::default()
    };
    println!(
        "{:<38} {:<5} {:>11} {:>3} {:>4} | cut: {:>5} {:>5} {:>5} | seam: {:>5} {:>5} {:>5} | {:>5} {:>5} {:>5} {:>5} | {:>5}",
        "take", "verd", "window s", "str", "ft", "pose", "foot", "dv", "pose", "foot", "dv", "speed", "steady", "target", "turn", "MW v"
    );
    println!("{:<38} {:<5} {:>11} {:>3} {:>4} | cut: {:>5} {:>5} {:>5} | seam: {:>5} {:>5} {:>5} | {:>5} {:>5} {:>5} {:>5} | {:>5}", "", "", "", "", "", "cm", "cm", "m/s", "cm", "cm/f", "m/s", "m/s", "m/s", "m/s", "deg", "m/s");
    let (mut cut, mut failed) = (0, 0);
    for line in verdicts.lines() {
        let Some((id, verdict)) = line.split_once(" = ") else { continue };
        let (id, verdict) = (id.trim_matches('"'), verdict.trim_matches('"'));
        if !matches!(verdict, "keep" | "maybe") || !["loco.walk.cycle.", "loco.run.cycle.", "loco.sprint.cycle."].iter().any(|p| id.starts_with(p)) {
            continue;
        }
        let Some((activity, _)) = id.split_once(".act.") else { continue };
        let dir = loco.join(activity).join("act");
        let path = dir.join(format!("{id}.bvh"));
        if !path.exists() {
            eprintln!("{} missing", path.display());
            continue;
        }
        let target_v = target_speed(&dir.join(format!("{id}.prov.json")));
        let (clip, report) = cook_library_clip_report("generated", &path, None, &rules, &target).unwrap();
        // Sanity: a finite clip.
        assert!(clip.duration.is_finite() && clip.meta.average_velocity.is_finite(), "{id}");
        let tv = target_v.map_or_else(|| "-".to_owned(), |v| format!("{v:.2}"));
        match report.loop_cut.expect("the rule was given") {
            Ok(c) => {
                cut += 1;
                assert!(clip.loop_range.is_some(), "{id}: a cut loop loops");
                for x in [c.cut_pose_error, c.seam_pose_error, c.cut_foot_jump, c.seam_foot_jump, c.seam_speed_jump, c.speed, c.turn] {
                    assert!(x.is_finite(), "{id}: {c:?}");
                }
                println!(
                    "{:<38} {:<5} {:>5.2}-{:<5.2} {:>3} {:>4} | cut: {:>5.1} {:>5.1} {:>5.2} | seam: {:>5.2} {:>5.2} {:>5.2} | {:>5.2} {:>5.2} {:>5} {:>5.1} | {:>5.2}",
                    id.trim_start_matches("loco."),
                    verdict,
                    c.start,
                    c.end,
                    c.strides,
                    if c.foot.to_ascii_lowercase().contains("left") || c.foot.ends_with('L') || c.foot.contains("_l") { "L" } else { "R" },
                    c.cut_pose_error * 100.0,
                    c.cut_foot_jump * 100.0,
                    c.cut_speed_jump,
                    c.seam_pose_error * 100.0,
                    c.seam_foot_jump * 100.0,
                    c.seam_speed_jump,
                    c.speed,
                    c.plateau_speed,
                    tv,
                    c.turn.to_degrees(),
                    clip.meta.average_velocity.length(),
                );
            }
            Err(e) => {
                failed += 1;
                assert!(clip.loop_range.is_none(), "{id}: an uncut take stays unlooped");
                println!("{:<38} {:<5} no loop: {e}", id.trim_start_matches("loco."), verdict);
            }
        }
    }
    println!("{cut} cut, {failed} without a loop");
}
