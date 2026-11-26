//! `AN32a` survey of Morrowind's humanoid attack, block and hit-reaction groups on the real game data: for each
//! humanoid set (`mw_graphs::HUMANOID_SETS`) and each of its animation files, every group with its duration, then
//! for each weapon (`chop`, `slash`, `thrust`, `shoot`, `block` sections) and reaction group (`Hit*`, `KnockDown`,
//! `Knockout`, `Shield*`, `SwimHit*`, `SwimKnock*`) its raw text keys, its sections' phase times and spans, and its
//! planar root travel, cooked as the bake cooks it (`mw_anim::rig` skeleton, the group's `clip_rules`). Last, the
//! female and beast groups whose duration differs from the male group of the same name. The data the combat sheets
//! (AN33) and reaction tiers (AN35) are written from. Every printed line starts with `attack clips`.
//! `creature_attack_clips` (`AN32b`) does the same for every creature skeleton's `Attack1`-`Attack3` groups (keys,
//! start-hit-stop spans, planar root travel, the closing travel table the lunge threshold is set from); its lines start
//! with `creature attack`.
//! Skips (passes) without a Morrowind install. Run with `-- --ignored --nocapture`.
#![allow(clippy::too_many_lines)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::sync::Arc;

use glam::Vec3;
use mw_anim::{Animation, Group, Section};
use mw_graphs::HUMANOID_SETS;
use mw_install::MorrowindInstall;
use sf_anim::{Clip, CookSettings, cook_clip};
use sf_vfs::{Vfs, VfsPath};

/// Sections that make a group a weapon (or block) group.
const WEAPON_SECTIONS: &[&str] = &["chop", "slash", "thrust", "shoot", "block"];
/// Group-name prefixes (lowercase) of the hit-reaction groups.
const REACTION_PREFIXES: &[&str] = &["hit", "knockdown", "knockout", "shield", "swimhit", "swimknock"];
/// Follow-through sizes; `""` is the unsized `Follow Start` / `Stop` pair (`Shoot`).
const FOLLOWS: &[&str] = &["small", "medium", "large", ""];
/// Root sampling rate of the excursion scan (Hz); a measurement fixture.
const SAMPLE_HZ: f32 = 60.0;
/// Key-time slack (s) when matching a key to a group's span or a section start.
const EPS: f32 = 1e-4;

/// One raw text key of a group: seconds from the group start, the event part lowercased (trailing `.` dropped),
/// and the line as written.
struct RawKey {
    t: f32,
    what: String,
    line: String,
}

fn group_keys(animation: &Animation, group: &Group) -> Vec<RawKey> {
    let mut keys: Vec<RawKey> = animation
        .text_keys
        .iter()
        .filter_map(|(time, line)| {
            let (prefix, what) = line.split_once(':')?;
            let inside = (group.start - EPS..=group.stop + EPS).contains(time);
            (inside && prefix.trim().eq_ignore_ascii_case(&group.name)).then(|| RawKey {
                t: time - group.start,
                what: what.trim().trim_end_matches('.').trim().to_ascii_lowercase(),
                line: line.clone(),
            })
        })
        .collect();
    keys.sort_by(|a, b| a.t.total_cmp(&b.t));
    keys
}

fn is_surveyed(group: &Group) -> bool {
    let lower = group.name.to_ascii_lowercase();
    group.sections.iter().any(|s| WEAPON_SECTIONS.contains(&s.name.as_str()))
        || REACTION_PREFIXES.iter().any(|p| lower.starts_with(p))
}

/// `"-"` for a missing value, else three decimals.
fn opt(v: Option<f32>) -> String {
    v.map_or_else(|| "-".to_owned(), |v| format!("{v:.3}"))
}

/// The first key `<action> <verb>` at or after the section start, in seconds from the section start.
fn key_at(keys: &[RawKey], section: &Section, verb: &str) -> Option<f32> {
    let want = if verb.is_empty() { section.name.clone() } else { format!("{} {verb}", section.name) };
    keys.iter().find(|k| k.t >= section.start - EPS && k.what == want).map(|k| k.t - section.start)
}

fn follow_verb(size: &str, edge: &str) -> String {
    if size.is_empty() { format!("follow {edge}") } else { format!("{size} follow {edge}") }
}

fn follow_label(size: &str) -> &str {
    if size.is_empty() { "follow" } else { size }
}

/// Planar (x/z) root offset of the cooked clip from time `a` to time `b` (group seconds).
fn planar(clip: &Clip, a: f32, b: f32) -> Vec3 {
    let d = clip.root_at(b).0 - clip.root_at(a).0;
    Vec3::new(d.x, 0.0, d.z)
}

/// Prints one weapon section: key times, derived spans, root travel.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "printed measurements"
)]
fn print_section(tag: &str, keys: &[RawKey], section: &Section, clip: Option<&Clip>, forward: Option<Vec3>) {
    let strike_verb = if section.name == "shoot" { "release" } else { "hit" };
    let start = key_at(keys, section, "start");
    let min = key_at(keys, section, "min attack");
    let max = key_at(keys, section, "max attack");
    let hit = key_at(keys, section, strike_verb);
    let stop = key_at(keys, section, "stop");
    let follows: Vec<(&str, Option<f32>, Option<f32>)> = FOLLOWS
        .iter()
        .map(|size| {
            let edge = |e: &str| key_at(keys, section, &follow_verb(size, e));
            (*size, edge("start"), edge("stop"))
        })
        // The unsized pair only when the data has it.
        .filter(|(size, a, b)| !size.is_empty() || a.is_some() || b.is_some())
        .collect();
    let s = &section.name;
    println!(
        "attack clips {tag} section {s}: span {:.3}..{:.3} s (group); \
         start {} min attack {} max attack {} {strike_verb} {} stop {}",
        section.start,
        section.stop,
        opt(start),
        opt(min),
        opt(max),
        opt(hit),
        opt(stop)
    );
    for (size, a, b) in &follows {
        println!("attack clips {tag} section {s}: {} follow start {} stop {}", follow_label(size), opt(*a), opt(*b));
    }
    let span = |a: Option<f32>, b: Option<f32>| a.zip(b).map(|(a, b)| b - a);
    let mut spans = format!(
        "wind-up {} charge {} strike {}",
        opt(span(start, min)),
        opt(span(min, max)),
        opt(span(max, hit))
    );
    // The follow windows lie one after another (large, medium, small), so `hit` to a later window's stop spans the
    // earlier windows too: the window's own length is printed beside it.
    for (size, a, b) in &follows {
        let label = follow_label(size);
        let _ = write!(spans, " follow-through {label} {} (window {})", opt(span(hit, *b)), opt(span(*a, *b)));
    }
    println!("attack clips {tag} section {s}: spans (s) {spans}");
    let Some(clip) = clip else {
        println!("attack clips {tag} section {s}: root travel - (no cooked clip)");
        return;
    };
    let s0 = section.start;
    let travel = |rel: Option<f32>| rel.map(|r| planar(clip, s0, s0 + r).length());
    let mut line = format!("to {strike_verb} {}", opt(travel(hit)));
    // The root at a follow start too: a jump from `hit` to that start moves the root by the difference.
    for (size, a, b) in &follows {
        let label = follow_label(size);
        let _ = write!(line, " to {label} follow start {} stop {}", opt(travel(*a)), opt(travel(*b)));
    }
    let end = section.stop.max(s0);
    let steps = ((end - s0) * SAMPLE_HZ).ceil() as usize;
    let (mut max_fwd, mut max_planar) = (f32::NEG_INFINITY, 0.0_f32);
    for i in 0..=steps {
        let t = (s0 + i as f32 / SAMPLE_HZ).min(end);
        let d = planar(clip, s0, t);
        max_planar = max_planar.max(d.length());
        if let Some(f) = forward {
            max_fwd = max_fwd.max(d.dot(f));
        }
    }
    let fwd = forward.map(|_| max_fwd);
    println!(
        "attack clips {tag} section {s}: root travel (m) {line}; max forward {} max planar {max_planar:.3}",
        opt(fwd)
    );
}

#[test]
#[ignore = "measurement: humanoid attack, block and hit-reaction text keys, phase durations and root travel (AN32a)"]
fn humanoid_attack_clips_survey() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs);
    let read = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.read(&p).ok());

    // Group name (lowercase) -> (name, duration) of the male set, for the cross-set comparison.
    let mut male: BTreeMap<String, (String, f32)> = BTreeMap::new();
    let mut surveyed_files: BTreeSet<String> = BTreeSet::new();
    let mut forward: Option<Vec3> = None;
    let mut male_weapon_groups = 0_usize;
    for set in &HUMANOID_SETS {
        let Some(model) = read(set.skeleton).and_then(|b| mw_nif::load(&b).ok()) else {
            println!("attack clips {}: skeleton {} not readable", set.name, set.skeleton);
            continue;
        };
        let skeleton = match mw_anim::rig::skeleton(&model) {
            Ok(s) => s,
            Err(e) => {
                println!("attack clips {}: skeleton {} failed: {e:?}", set.name, set.skeleton);
                continue;
            }
        };
        for file in set.animations {
            let tag_file = format!("{} {file}", set.name);
            if !surveyed_files.insert((*file).to_owned()) {
                println!("attack clips {tag_file}: already surveyed above (same file)");
                continue;
            }
            let Some(animation) = read(file).and_then(|b| Animation::from_file(file, &b).ok()) else {
                println!("attack clips {tag_file}: not readable");
                continue;
            };
            let groups = animation.groups();
            let cook = |g: &Group| {
                cook_clip(&skeleton, &animation.source_clip(g), &mw_anim::clip_rules(g), &CookSettings::default())
            };
            // The forward axis: the male WalkForward travel direction (one coordinate convention for all sets).
            if forward.is_none()
                && let Some(walk) = groups.iter().find(|g| g.name.eq_ignore_ascii_case("walkforward"))
                && let Ok(clip) = cook(walk)
            {
                let d = planar(&clip, 0.0, clip.duration);
                forward = (d.length() > EPS).then(|| d.normalize());
                let axis = forward.map_or_else(|| "-".to_owned(), |f| format!("{f:.3}"));
                println!("attack clips forward axis (WalkForward of {tag_file}): {axis}");
            }
            let list: Vec<String> = groups.iter().map(|g| format!("{} {:.3}", g.name, g.duration())).collect();
            println!("attack clips {tag_file}: {} groups: {}", groups.len(), list.join(", "));

            for group in groups.iter().filter(|g| is_surveyed(g)) {
                let tag = format!("{} {}", set.name, group.name);
                let weapon = group.sections.iter().any(|s| WEAPON_SECTIONS.contains(&s.name.as_str()));
                if set.name == "male" && weapon {
                    male_weapon_groups += 1;
                }
                let clip = match cook(group) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        println!("attack clips {tag}: cook failed: {e:?}");
                        None
                    }
                };
                let whole = clip.as_ref().map(|c| planar(c, 0.0, c.duration).length());
                let rm = match &clip {
                    Some(c) if c.root_motion.is_none() => " (no root motion)",
                    _ => "",
                };
                let sections: Vec<&str> = group.sections.iter().map(|s| s.name.as_str()).collect();
                println!(
                    "attack clips {tag}: group {:.3} s, planar root travel {} m{rm}, sections [{}]",
                    group.duration(),
                    opt(whole),
                    sections.join(" ")
                );
                let keys = group_keys(&animation, group);
                for k in &keys {
                    println!("attack clips {tag}: key {:.3} {}", k.t, k.line);
                }
                for section in group.sections.iter().filter(|s| WEAPON_SECTIONS.contains(&s.name.as_str())) {
                    print_section(&tag, &keys, section, clip.as_ref(), forward);
                }
            }

            if set.name == "male" {
                for g in &groups {
                    male.entry(g.name.to_ascii_lowercase()).or_insert_with(|| (g.name.clone(), g.duration()));
                }
            } else if set.name == "female" || set.name == "beast" {
                let (mut same, mut differ) = (0, 0);
                for g in &groups {
                    let Some((name, d)) = male.get(&g.name.to_ascii_lowercase()) else { continue };
                    if (g.duration() - d).abs() > 1e-3 {
                        differ += 1;
                        println!(
                            "attack clips {tag_file}: duration differs {name}: \
                             male {d:.3} s, this {:.3} s, delta {:+.3} s",
                            g.duration(),
                            g.duration() - d
                        );
                    } else {
                        same += 1;
                    }
                }
                println!("attack clips {tag_file}: vs male: {same} groups same duration, {differ} differ");
            }
        }
    }
    assert!(male_weapon_groups > 0, "a detected install yields no weapon group in the male set");
}

/// The creature attack groups (lowercase), one move each in the combat spec.
const CREATURE_ATTACKS: &[&str] = &["attack1", "attack2", "attack3"];
/// Sections that make another creature group an attack (`block` is not one).
const CREATURE_STRIKE_SECTIONS: &[&str] = &["chop", "slash", "thrust", "shoot"];
/// Groups whose cooked travel gives a creature skeleton its forward axis (first found wins), as slice a derives the
/// humanoid one from `WalkForward`.
const FORWARD_GROUPS: &[&str] = &["walkforward", "runforward", "swimwalkforward", "swimrunforward"];
/// Start-to-hit travel (m) the closing table counts moves at or over; a measurement fixture.
const TRAVEL_BINS: &[f32] = &[0.5, 1.0, 1.5, 2.0];

/// Planar root travel of one creature attack, in metres.
#[derive(Default)]
struct Travel {
    to_hit: Option<f32>,
    hit_fwd: Option<f32>,
    to_stop: Option<f32>,
    max_fwd: Option<f32>,
    max_planar: Option<f32>,
    end_fwd: Option<f32>,
    end_side: Option<f32>,
}

/// Root travel of `clip` from `t0` (the start key) to `hit` and to `t1` (the stop key), the largest forward and
/// planar excursion between them, and where the root ends (forward and sideways) relative to `t0`.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "printed measurements"
)]
fn root_travel(clip: &Clip, t0: f32, t1: f32, hit: Option<f32>, forward: Option<Vec3>) -> Travel {
    let steps = ((t1 - t0).max(0.0) * SAMPLE_HZ).ceil() as usize;
    let (mut max_fwd, mut max_planar) = (f32::NEG_INFINITY, 0.0_f32);
    for i in 0..=steps {
        let t = (t0 + i as f32 / SAMPLE_HZ).min(t1);
        let d = planar(clip, t0, t);
        max_planar = max_planar.max(d.length());
        if let Some(f) = forward {
            max_fwd = max_fwd.max(d.dot(f));
        }
    }
    let end = planar(clip, t0, t1);
    let at_hit = hit.map(|h| planar(clip, t0, h));
    Travel {
        to_hit: at_hit.map(Vec3::length),
        hit_fwd: at_hit.zip(forward).map(|(d, f)| d.dot(f)),
        to_stop: Some(end.length()),
        max_fwd: forward.map(|_| max_fwd),
        max_planar: Some(max_planar),
        end_fwd: forward.map(|f| end.dot(f)),
        end_side: forward.map(|f| (end - f * end.dot(f)).length()),
    }
}

/// One surveyed (skeleton, group) for the closing table.
struct Row {
    skeleton: String,
    group: String,
    travel: Travel,
}

#[test]
#[ignore = "measurement: creature Attack1-3 text keys, phase durations and root travel (AN32b)"]
fn creature_attack_clips() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs);
    let db = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
    let sets = mw_bake::anim::actor_sets(&db, &vfs);
    let read = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.read(&p).ok());

    let mut rows: Vec<Row> = Vec::new();
    let mut surveyed: BTreeSet<(String, Vec<String>)> = BTreeSet::new();
    let (mut skeletons, mut without) = (0_usize, 0_usize);
    for set in sets.sets.values().filter(|s| sets.bodies.contains(&s.skeleton)) {
        let skel = &set.skeleton;
        if !surveyed.insert((skel.to_ascii_lowercase(), set.files.clone())) {
            continue;
        }
        let Some(model) = read(skel).and_then(|b| mw_nif::load(&b).ok()) else {
            println!("creature attack {skel}: skeleton not readable");
            continue;
        };
        let skeleton = match mw_anim::rig::skeleton(&model) {
            Ok(s) => s,
            Err(e) => {
                println!("creature attack {skel}: skeleton failed: {e:?}");
                continue;
            }
        };
        skeletons += 1;
        // Highest-priority file first: a group comes from the first file that has it.
        let anims: Vec<(&str, Animation)> = set
            .files
            .iter()
            .filter_map(|file| {
                let animation = read(file).and_then(|b| Animation::from_file(file, &b).ok());
                if animation.is_none() {
                    println!("creature attack {skel}: {file} not readable");
                }
                animation.map(|a| (file.as_str(), a))
            })
            .collect();
        let groups: Vec<_> = anims.iter().map(|(_, a)| a.groups()).collect();
        // Group name (lowercase) -> (file index, group index).
        let mut chosen: BTreeMap<String, (usize, usize)> = BTreeMap::new();
        for (fi, file_groups) in groups.iter().enumerate() {
            for (gi, g) in file_groups.iter().enumerate() {
                chosen.entry(g.name.to_ascii_lowercase()).or_insert((fi, gi));
            }
        }
        let cook = |fi: usize, g: &Group| {
            cook_clip(&skeleton, &anims[fi].1.source_clip(g), &mw_anim::clip_rules(g), &CookSettings::default())
        };
        let forward = FORWARD_GROUPS.iter().find_map(|name| {
            let &(fi, gi) = chosen.get(*name)?;
            let clip = cook(fi, &groups[fi][gi]).ok()?;
            let d = planar(&clip, 0.0, clip.duration);
            (d.length() > EPS).then(|| (groups[fi][gi].name.clone(), d.normalize()))
        });
        let rm = skeleton.root_motion_joint().map_or_else(|| "-".to_owned(), |j| skeleton.name(j).to_owned());
        let files: Vec<&str> = anims.iter().map(|(f, _)| *f).collect();
        let axis = forward.as_ref().map_or_else(|| "-".to_owned(), |(n, f)| format!("{f:.3} ({n})"));
        println!(
            "creature attack {skel}: files [{}], root-motion joint {rm}, forward axis {axis}",
            files.join(" ")
        );
        let forward = forward.map(|(_, f)| f);

        let mut found = 0_usize;
        for name in CREATURE_ATTACKS {
            let Some(&(fi, gi)) = chosen.get(*name) else { continue };
            found += 1;
            let (file, animation) = (anims[fi].0, &anims[fi].1);
            let group = &groups[fi][gi];
            let tag = format!("{skel} {}", group.name);
            let keys = group_keys(animation, group);
            let sections: Vec<&str> = group.sections.iter().map(|s| s.name.as_str()).collect();
            println!(
                "creature attack {tag}: from {file}, {:.3} s, {} keys, sections [{}]",
                group.duration(),
                keys.len(),
                sections.join(" ")
            );
            for k in &keys {
                println!("creature attack {tag}: key {:.3} {}", k.t, k.line);
            }
            let key = |what: &str| keys.iter().find(|k| k.what == what).map(|k| k.t);
            let (start, hit, stop) = (key("start"), key("hit"), key("stop"));
            let span = |a: Option<f32>, b: Option<f32>| a.zip(b).map(|(a, b)| b - a);
            println!(
                "creature attack {tag}: start {} hit {} stop {}; spans (s) start-hit {} hit-stop {}",
                opt(start),
                opt(hit),
                opt(stop),
                opt(span(start, hit)),
                opt(span(hit, stop))
            );
            let travel = match cook(fi, group) {
                Ok(clip) => {
                    let t0 = start.unwrap_or(0.0).clamp(0.0, clip.duration);
                    let t1 = stop.unwrap_or(clip.duration).clamp(t0, clip.duration);
                    let travel = root_travel(&clip, t0, t1, hit, forward);
                    let no_rm = if clip.root_motion.is_none() { " (no root motion)" } else { "" };
                    println!(
                        "creature attack {tag}: root travel (m) start-hit {} (forward {}) start-stop {}; \
                         max forward {} max planar {}; ends forward {} side {}{no_rm}",
                        opt(travel.to_hit),
                        opt(travel.hit_fwd),
                        opt(travel.to_stop),
                        opt(travel.max_fwd),
                        opt(travel.max_planar),
                        opt(travel.end_fwd),
                        opt(travel.end_side)
                    );
                    travel
                }
                Err(e) => {
                    println!("creature attack {tag}: cook failed: {e:?}");
                    Travel::default()
                }
            };
            rows.push(Row { skeleton: skel.clone(), group: group.name.clone(), travel });
        }
        if found == 0 {
            without += 1;
            println!("creature attack {skel}: no Attack1-3 group");
        }

        // Other attack-like groups, by source file.
        let mut others: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
        for (lower, &(fi, gi)) in &chosen {
            let g = &groups[fi][gi];
            let strike = g.sections.iter().any(|s| CREATURE_STRIKE_SECTIONS.contains(&s.name.as_str()));
            if !CREATURE_ATTACKS.contains(&lower.as_str()) && (lower.starts_with("attack") || strike) {
                others.entry(fi).or_default().push(g.name.as_str());
            }
        }
        if !others.is_empty() {
            let list: Vec<String> =
                others.iter().map(|(fi, names)| format!("{}: {}", anims[*fi].0, names.join(" "))).collect();
            println!("creature attack {skel}: other attack groups: {}", list.join("; "));
        }
    }

    println!(
        "creature attack summary: {skeletons} skeletons, {} attack groups, {without} skeletons without Attack1-3",
        rows.len()
    );
    // Moves without a hit key (or a failed cook) sort last.
    rows.sort_by(|a, b| {
        let k = |r: &Row| (r.travel.to_hit.is_none(), r.travel.to_hit.unwrap_or(0.0));
        let (ka, kb) = (k(a), k(b));
        ka.0.cmp(&kb.0).then(ka.1.total_cmp(&kb.1))
    });
    println!("creature attack table: start-hit (m), hit forward, start-stop, max forward, ends forward, skeleton group");
    for r in &rows {
        let t = &r.travel;
        println!(
            "creature attack table: {:>7} {:>7} {:>7} {:>7} {:>7}  {} {}",
            opt(t.to_hit),
            opt(t.hit_fwd),
            opt(t.to_stop),
            opt(t.max_fwd),
            opt(t.end_fwd),
            r.skeleton,
            r.group
        );
    }
    let counts: Vec<String> = TRAVEL_BINS
        .iter()
        .map(|&m| {
            let n = rows.iter().filter(|r| r.travel.to_hit.is_some_and(|d| d >= m)).count();
            format!(">= {m:.1} m: {n}")
        })
        .collect();
    let no_hit = rows.iter().filter(|r| r.travel.to_hit.is_none()).count();
    println!(
        "creature attack counts (start-hit travel, of {} moves): {}; no hit key or cook: {no_hit}",
        rows.len(),
        counts.join(", ")
    );
    assert!(!rows.is_empty(), "a detected install yields no creature attack group");
}
