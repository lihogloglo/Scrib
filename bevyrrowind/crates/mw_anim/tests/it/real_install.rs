//! KF import on the real game data. Skips (passes) when no Morrowind
//! install is detected. Per-file listing:
//! `cargo run -p mw_anim --example anim_report -- meshes/xbase_anim.kf --events`.

use mw_anim::{Animation, Group, KfAnimImporter, events};
use mw_install::MorrowindInstall;
use sf_anim::{AnimImporter, Event, Looping};
use sf_vfs::{Vfs, VfsPath};

fn vfs() -> Option<Vfs> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    Some(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs)
}

fn read(vfs: &Vfs, path: &str) -> Vec<u8> {
    vfs.read(&VfsPath::new(path).unwrap()).unwrap()
}

fn group<'a>(groups: &'a [Group], name: &str) -> &'a Group {
    groups.iter().find(|g| g.name.eq_ignore_ascii_case(name)).unwrap_or_else(|| panic!("group {name}"))
}

fn count(events: &[Event], name: &str, payload: &str) -> usize {
    events.iter().filter(|e| e.name == name && e.payload == payload).count()
}

fn windows(events: &[Event], name: &str, payload: &str) -> usize {
    events.iter().filter(|e| e.name == name && e.payload == payload && e.end.is_some_and(|end| end > e.time)).count()
}

/// The ROADMAP's table test: groups and events of `xbase_anim.kf`.
#[test]
fn xbase_anim_groups_and_events() {
    let Some(vfs) = vfs() else { return };
    let animation = Animation::from_kf(&read(&vfs, "meshes/xbase_anim.kf")).unwrap();
    let groups = animation.groups();
    assert_eq!(animation.tracks.len(), 34, "animated Bip01 bones");
    assert_eq!(groups.len(), 135);
    assert!(groups.iter().all(|g| g.duration() > 0.0));
    assert!(animation.tracks.iter().any(|(bone, _)| bone == "Bip01"));

    // Idle groups loop; Idle2 and walks carry explicit loop ranges.
    assert_eq!(group(&groups, "Idle").looping(), Looping::Whole);
    assert!(matches!(group(&groups, "Idle2").looping(), Looping::Range(..)));
    for name in ["Idle3", "Idle9", "IdleSwim", "IdleSneak"] {
        assert_ne!(group(&groups, name).looping(), Looping::Once, "{name}");
    }

    // Table: (group, event, payload, count).
    let table: &[(&str, &str, &str, usize)] = &[
        ("WalkForward", events::FOOTSTEP, "left", 1),
        ("WalkForward", events::FOOTSTEP, "right", 1),
        ("RunForward", events::FOOTSTEP, "left", 1),
        ("SneakBack", events::FOOTSTEP, "right", 1),
        ("Jump", events::LAND, "", 1),
        ("Hit1", events::SOUND_GEN, "moan", 1),
        ("Hit2", events::SOUND_GEN, "roar", 1),
        ("Hit3", events::SOUND_GEN, "scream", 1),
        ("WeaponOneHand", events::EQUIP_ATTACH, "", 1),
        ("WeaponOneHand", events::UNEQUIP_DETACH, "", 1),
        ("WeaponOneHand", events::ATTACK_HIT, "chop", 1),
        ("WeaponOneHand", events::ATTACK_HIT, "slash", 1),
        ("WeaponOneHand", events::ATTACK_HIT, "thrust", 1),
        ("WeaponOneHand", events::ATTACK_MIN_HIT, "chop", 1),
        ("WeaponTwoHand", events::ATTACK_HIT, "slash", 1),
        ("HandToHand", events::ATTACK_HIT, "thrust", 1),
        ("BowAndArrow", events::ATTACK_ATTACH, "shoot", 1),
        ("BowAndArrow", events::ATTACK_RELEASE, "shoot", 1),
        ("BowAndArrow", events::SOUND, "BowPull", 1),
        ("BowAndArrow", events::SOUND, "BowShoot", 1),
        ("Crossbow", events::ATTACK_FOLLOW_ATTACH, "shoot", 1),
        ("SpellCast", events::CAST_RELEASE, "self", 1),
        ("SpellCast", events::CAST_RELEASE, "touch", 1),
        ("SpellCast", events::CAST_RELEASE, "target", 1),
        ("Shield", events::BLOCK_HIT, "", 1),
    ];
    for &(name, event, payload, n) in table {
        assert_eq!(count(&group(&groups, name).events, event, payload), n, "{name}: {event} {payload}");
    }
    // Windows: attack wind-ups and follow-throughs.
    let weapon = group(&groups, "WeaponOneHand");
    for attack in ["chop", "slash", "thrust"] {
        assert_eq!(windows(&weapon.events, events::ATTACK_CHARGE, attack), 1, "{attack} charge");
        for size in ["small", "medium", "large"] {
            assert_eq!(windows(&weapon.events, events::ATTACK_FOLLOW, &format!("{attack} {size}")), 1, "{attack} {size} follow");
        }
    }
    assert_eq!(windows(&group(&groups, "BowAndArrow").events, events::ATTACK_CHARGE, "shoot"), 1);

    // Sections: what a graph plays for each sub-action.
    let names = |g: &Group| g.sections.iter().map(|s| s.name.clone()).collect::<Vec<_>>();
    assert_eq!(names(weapon), ["equip", "slash", "chop", "thrust", "unequip"]);
    assert_eq!(names(group(&groups, "SpellCast")), ["equip", "unequip", "self", "target", "touch"]);
    assert_eq!(names(group(&groups, "BowAndArrow")), ["equip", "shoot", "unequip"]);
    assert_eq!(names(group(&groups, "Shield")), ["block"]);
    for g in &groups {
        assert!(g.sections.iter().all(|s| s.start <= s.stop && s.stop <= g.duration() + 1e-4), "{}", g.name);
        assert!(g.events.iter().all(|e| e.time >= 0.0 && e.time <= g.duration() + 1e-4), "{}", g.name);
    }
    // Every key is understood.
    let unknown: Vec<&str> = groups.iter().flat_map(|g| &g.events).filter(|e| e.name == events::TEXT_KEY).map(|e| e.payload.as_str()).collect();
    assert!(unknown.is_empty(), "unmapped text keys: {unknown:?}");

    // Root motion: forward travel is on Bip01 (engine -Z is forward).
    let walk = animation.source_clip(group(&groups, "WalkForward"));
    let bip01 = walk.tracks.iter().find(|t| t.joint == "Bip01").and_then(|t| t.translation.as_ref()).unwrap();
    let travel = bip01.sample(walk.duration) - bip01.sample(0.0);
    assert!(travel.z < -2.0 && travel.x.abs() < 0.2, "{travel}");
    let non_accum = walk.tracks.iter().find(|t| t.joint == "Bip01 NonAccum").and_then(|t| t.translation.as_ref());
    assert!(non_accum.is_none_or(|c| (c.sample(walk.duration) - c.sample(0.0)).length() < 0.05));
    let source = KfAnimImporter.import("meshes/xbase_anim.kf", &read(&vfs, "meshes/xbase_anim.kf")).unwrap();
    assert_eq!(source.root_motion.as_deref(), Some("Bip01"));
    assert_eq!(source.clips.len(), groups.len());
}

/// Every `.kf` in the install imports, with groups and valid curves.
#[test]
fn every_kf_imports() {
    let Some(vfs) = vfs() else { return };
    let is_kf = |p: &&VfsPath| std::path::Path::new(p.as_str()).extension().is_some_and(|e| e.eq_ignore_ascii_case("kf"));
    let kfs: Vec<VfsPath> = vfs.paths().filter(is_kf).cloned().collect();
    assert!(kfs.len() > 50, "{} kf files", kfs.len());
    let (mut clips, mut failures, mut empty) = (0, Vec::new(), Vec::new());
    let mut unknown = std::collections::BTreeSet::new();
    for path in &kfs {
        match KfAnimImporter.import(path.as_str(), &vfs.read(path).unwrap()) {
            Ok(source) if source.clips.is_empty() => empty.push(path.as_str().to_owned()),
            Ok(source) => {
                for clip in &source.clips {
                    for track in &clip.tracks {
                        let ok = track.translation.as_ref().is_none_or(sf_anim::Curve::is_valid)
                            && track.rotation.as_ref().is_none_or(sf_anim::Curve::is_valid)
                            && track.scale.as_ref().is_none_or(sf_anim::Curve::is_valid);
                        assert!(ok, "{path}#{}: {} has an invalid curve", clip.name, track.joint);
                    }
                }
                clips += source.clips.len();
                let generic = source.clips.iter().flat_map(|c| &c.events).filter(|e| e.name == events::TEXT_KEY);
                unknown.extend(generic.map(|e| e.payload.split_once(':').map_or(e.payload.as_str(), |(_, w)| w.trim()).to_ascii_lowercase()));
            }
            Err(e) => failures.push(e.to_string()),
        }
    }
    eprintln!("{} kf files, {clips} clips, {} without groups: {empty:?}", kfs.len(), empty.len());
    assert!(unknown.is_empty(), "unmapped text-key kinds: {unknown:?}");
    assert!(failures.is_empty(), "{failures:?}");
    assert!(empty.len() * 20 < kfs.len(), "most KFs have groups: {empty:?}");
}
