//! Seat clip pelvis fit survey (AN26): measurement only, no limits.
//!
//! Every `seat.*` slot of the builtin sources manifest, its first candidate cooked the bake's
//! way ([`mw_bake::anim::cook_library_clip_report`] with the slot's rules: pelvis fit and
//! mirror included) onto the Morrowind humanoid. One row per slot: the fit target, the hub
//! lift, the largest foot error after the leg IK, the frames lifted, the clip length, whether
//! the candidate is mirrored and the fit's status. Then a summary (largest lift and foot
//! error, min/max lift per family). Run with
//! `cargo test -p mw_bake --test it seat_fit_survey:: -- --ignored --nocapture`; skips without a
//! Morrowind install or the generated `sit1` takes.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]

use std::collections::BTreeMap;

use mw_anim::rig;
use mw_bake::anim::{HumanoidTarget, cook_library_clip_report};
use mw_bake::sources::AnimSources;
use mw_graphs::HUMANOID_SETS;
use mw_graphs::sources::SourcesManifest;
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

/// Families, most specific first (`seat.chair.old` before `seat.chair`).
const FAMILIES: [&str; 5] = ["seat.chair.old", "seat.chair", "seat.stool", "seat.bench", "seat.floor"];

fn family(slot: &str) -> &'static str {
    FAMILIES.iter().find(|f| slot.starts_with(&format!("{f}."))).copied().unwrap_or("other")
}

fn target() -> Option<HumanoidTarget> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    let bytes = vfs.read(&VfsPath::new(HUMANOID_SETS[0].skeleton).unwrap()).unwrap();
    Some(HumanoidTarget::new(rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap()).unwrap())
}

#[test]
#[ignore = "measurement: seat clip pelvis fit (lift, foot error) over the baked seat slots (AN26)"]
fn measure_seat_fit() {
    let generated = AnimSources::local().dir.join("generated");
    let sit = generated.join("sit1");
    if !sit.exists() {
        eprintln!("{} missing; skipping", sit.display());
        return;
    }
    let Some(target) = target() else { return };
    let manifest = SourcesManifest::builtin();
    let seats: Vec<_> = manifest.slots.iter().filter(|(name, _)| name.starts_with("seat.")).collect();
    assert!(!seats.is_empty(), "no seat.* slot in the builtin sources manifest");

    println!(
        "{:<36} | {:>8} | {:>8} | {:>8} | {:>9} | {:>6} | {:<6} | status",
        "slot", "target m", "lift cm", "foot cm", "lifted/n", "sec", "mirror"
    );
    let (mut cooked, mut failed, mut fit_errors, mut missing) = (0u32, 0u32, 0u32, 0u32);
    let mut max_lift: Option<(f32, &str)> = None;
    let mut max_foot: Option<(f32, &str)> = None;
    let mut per_family: BTreeMap<&str, (f32, f32)> = BTreeMap::new();
    for (name, slot) in &seats {
        let Some(candidate) = slot.candidates.first() else {
            println!("{name:<36} | no candidate");
            failed += 1;
            continue;
        };
        let rules = slot.rules(candidate);
        let fit_target = rules.pelvis_fit.as_ref().map_or_else(|| "-".to_owned(), |f| format!("{:.3}", f.target_m));
        let mirror = if candidate.mirror.is_some() { "mirror" } else { "" };
        let Some(file) = candidate.file.as_deref() else {
            println!("{name:<36} | {fit_target:>8} | candidate names no file ({})", candidate.reference());
            missing += 1;
            continue;
        };
        let path = generated.join(file);
        if !path.exists() {
            println!("{name:<36} | {fit_target:>8} | file missing: {}", path.display());
            missing += 1;
            continue;
        }
        let (clip, report) = match cook_library_clip_report(&candidate.source, &path, None, &rules, &target) {
            Ok(r) => r,
            Err(e) => {
                println!("{name:<36} | {fit_target:>8} | {:>8} | {:>8} | {:>9} | {:>6} | {mirror:<6} | cook: {e}", "-", "-", "-", "-");
                failed += 1;
                continue;
            }
        };
        cooked += 1;
        let frames = (clip.duration * clip.rate).round() as u32 + 1;
        let (lift, foot, lifted, status) = match &report.pelvis_fit {
            Some(Ok(s)) => {
                let lift_cm = s.lift_m * 100.0;
                let foot_cm = s.max_foot_err_m * 100.0;
                if max_lift.is_none_or(|(m, _)| lift_cm.abs() > m.abs()) {
                    max_lift = Some((lift_cm, name.as_str()));
                }
                if max_foot.is_none_or(|(m, _)| foot_cm > m) {
                    max_foot = Some((foot_cm, name.as_str()));
                }
                let e = per_family.entry(family(name)).or_insert((f32::INFINITY, f32::NEG_INFINITY));
                e.0 = e.0.min(lift_cm);
                e.1 = e.1.max(lift_cm);
                (format!("{lift_cm:.1}"), format!("{foot_cm:.2}"), format!("{}/{frames}", s.frames_lifted), "ok".to_owned())
            }
            Some(Err(e)) => {
                fit_errors += 1;
                ("-".to_owned(), "-".to_owned(), format!("-/{frames}"), format!("fit: {e}"))
            }
            None => ("-".to_owned(), "-".to_owned(), format!("-/{frames}"), "no fit asked".to_owned()),
        };
        println!(
            "{name:<36} | {fit_target:>8} | {lift:>8} | {foot:>8} | {lifted:>9} | {:>6.2} | {mirror:<6} | {status}",
            clip.duration
        );
    }

    println!("\n== summary over {} seat slots", seats.len());
    println!("cooked {cooked}, cook failures {failed}, fit errors {fit_errors}, files missing {missing}");
    match max_lift {
        Some((l, s)) => println!("largest |lift|: {l:.1} cm ({s})"),
        None => println!("largest |lift|: - (no fit applied)"),
    }
    match max_foot {
        Some((f, s)) => println!("largest foot error: {f:.2} cm ({s})"),
        None => println!("largest foot error: - (no fit applied)"),
    }
    for f in FAMILIES {
        match per_family.get(f) {
            Some((lo, hi)) => println!("{f:<15} lift min {lo:.1} cm, max {hi:.1} cm"),
            None => println!("{f:<15} no fitted slot"),
        }
    }
}
