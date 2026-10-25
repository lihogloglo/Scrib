//! The graph templates on the real game data: every template compiles on
//! every relevant skeleton, a headless simulation drives each state, and
//! every CREA model gets a coverage row. Skips (passes) when no Morrowind
//! install is detected. `-- --nocapture` prints the reports.

use std::collections::BTreeMap;

use glam::Vec2;
use mw_anim::Animation;
use mw_anim::rig::{self, CreatureMover};
use mw_graphs::build::{params as p, slots, states};
use mw_graphs::script::{GroupRequest, SCRIPT_SLOT};
use mw_graphs::sim::Sim;
use mw_graphs::{AnimSet, Coverage, HUMANOID_SETS, Stance, Template};
use mw_install::MorrowindInstall;
use sf_anim::EventEdge;
use sf_anim_graph::{MontageClip, NoticeKind};
use sf_vfs::{Vfs, VfsPath};

const DT: f32 = 1.0 / 30.0;

fn install() -> Option<(MorrowindInstall, Vfs)> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    Some((install, vfs))
}

fn read(vfs: &Vfs, path: &str) -> Option<Vec<u8>> {
    vfs.read(&VfsPath::new(path).ok()?).ok()
}

fn exists(vfs: &Vfs, path: &str) -> bool {
    VfsPath::new(path).is_ok_and(|p| vfs.contains(&p))
}

fn load_set(vfs: &Vfs, name: &str, skeleton: &str, animations: &[String]) -> Result<AnimSet, String> {
    let bytes = read(vfs, skeleton).ok_or_else(|| format!("no {skeleton}"))?;
    let model = mw_nif::load(&bytes).map_err(|e| format!("{skeleton}: {e}"))?;
    let skeleton = rig::skeleton(&model).map_err(|e| format!("{skeleton}: {e:?}"))?;
    let mut anims = Vec::new();
    for a in animations {
        let Some(bytes) = read(vfs, a) else { continue };
        let anim = Animation::from_file(a, &bytes).map_err(|e| format!("{a}: {e}"))?;
        if !anim.text_keys.is_empty() {
            anims.push((a.clone(), anim));
        }
    }
    Ok(AnimSet::new(name, skeleton, anims))
}

fn humanoid(vfs: &Vfs, set: &mw_graphs::HumanoidSet) -> AnimSet {
    let anims: Vec<String> = set.animations.iter().map(|s| (*s).to_owned()).collect();
    load_set(vfs, set.name, set.skeleton, &anims).unwrap().with_humanoid_rig()
}

fn sim(set: &mut AnimSet, t: Template) -> Sim {
    let v = mw_graphs::validate(set, t).unwrap_or_else(|e| panic!("{}: {e}", set.name));
    let clips = set.binding_clips(&v.graph).unwrap();
    Sim::new(v.graph, set.skeleton.clone(), clips).unwrap()
}

#[test]
fn templates_compile_on_every_humanoid_skeleton() {
    let Some((_, vfs)) = install() else { return };
    for set in &HUMANOID_SETS {
        let mut s = humanoid(&vfs, set);
        let t = if set.first_person { Template::FirstPerson } else { Template::Humanoid };
        let v = mw_graphs::validate(&mut s, t).unwrap_or_else(|e| panic!("{}: {e}", set.name));
        eprintln!(
            "{} ({} groups): {} clips, {} states, {} nodes; fell back: {:?}; dropped: {:?}",
            set.name,
            s.group_names().count(),
            v.graph.clips.len(),
            v.states.len(),
            v.graph.nodes.len(),
            v.fell_back.iter().map(|(n, _, to)| format!("{n}->{}", to.as_deref().unwrap_or("-"))).collect::<Vec<_>>(),
            v.dropped
        );
        let clips = s.binding_clips(&v.graph).unwrap();
        Sim::new(v.graph, s.skeleton.clone(), clips).unwrap();
    }
}

/// Runs `seconds` and returns the `locomotion` state.
fn settle(s: &mut Sim, seconds: f32) -> String {
    s.run(seconds, DT);
    s.state(mw_graphs::build::LOCOMOTION).unwrap_or("?").to_owned()
}

fn expect(s: &mut Sim, seconds: f32, state: &str, clip: &str) {
    let got_state = settle(s, seconds);
    let playing = s.playing();
    assert_eq!(got_state, state, "t={:.2}: playing {playing:?}", s.time);
    assert!(s.plays(clip, 0.5), "t={:.2} {state}: expected {clip}, playing {playing:?}", s.time);
}

fn count(s: &Sim, name: &str, payload: &str, since: f32) -> usize {
    s.fired(name, since).iter().filter(|e| e.payload == payload && e.edge != EventEdge::Exit).count()
}

/// A scripted parameter sequence drives every state of the humanoid graph
/// on the male skeleton; clips, footsteps, attack windows and the scripted
/// slot are checked.
#[test]
#[allow(clippy::too_many_lines)]
fn humanoid_simulation_drives_every_state() {
    let Some((_, vfs)) = install() else { return };
    let mut set = humanoid(&vfs, &HUMANOID_SETS[0]);
    let mut s = sim(&mut set, Template::Humanoid);
    let fwd = Vec2::new(0.0, 1.0);

    expect(&mut s, 0.5, states::IDLE, "Idle");
    // Walk, four directions, footsteps from the walk cycle.
    s.set(p::MOVING, 1.0);
    s.set_vec2(p::MOVE, fwd);
    // At the walk cycle's authored speed (the feed gives the measured one).
    s.velocity = glam::Vec3::new(0.0, 0.0, 2.2);
    let t0 = s.time;
    expect(&mut s, 3.0, states::WALK, "WalkForward");
    assert!(count(&s, "footstep", "left", t0) >= 2 && count(&s, "footstep", "right", t0) >= 2, "{:?}", s.fired("footstep", t0));
    for (dir, clip) in [(Vec2::X, "WalkRight"), (-Vec2::X, "WalkLeft"), (-Vec2::Y, "WalkBack")] {
        s.set_vec2(p::MOVE, dir);
        expect(&mut s, 0.6, states::WALK, clip);
    }
    s.set_vec2(p::MOVE, fwd);
    s.set(p::RUNNING, 1.0);
    expect(&mut s, 1.0, states::RUN, "RunForward");
    // Weapon stances: their own movement sets, or an upper-body overlay.
    for (stance, clip) in [
        (Stance::HandToHand, "RunForwardhh"),
        (Stance::OneHand, "RunForward1h"),
        (Stance::TwoClose, "RunForward2c"),
        (Stance::TwoWide, "RunForward2w"),
    ] {
        s.set(p::STANCE, stance.value());
        expect(&mut s, 0.6, states::RUN, clip);
    }
    s.set(p::RUNNING, 0.0);
    s.set(p::STANCE, Stance::Crossbow.value());
    expect(&mut s, 0.6, states::WALK, "WalkForward1h");
    assert!(s.plays("IdleCrossbow", 0.5), "crossbow overlay: {:?}", s.playing());
    s.set(p::MOVING, 0.0);
    s.set(p::STANCE, Stance::Spell.value());
    expect(&mut s, 0.6, states::IDLE, "IdleSpell");
    s.set(p::TURN, 2.0);
    expect(&mut s, 0.6, states::TURN_LEFT, "SpellTurnLeft");
    s.set(p::STANCE, Stance::TwoWide.value());
    s.set(p::TURN, -2.0);
    expect(&mut s, 0.6, states::TURN_RIGHT, "TurnRight2w");
    s.set(p::TURN, 0.0);
    s.set(p::STANCE, Stance::None.value());
    // Sneak.
    s.set(p::SNEAKING, 1.0);
    expect(&mut s, 0.6, states::SNEAK_IDLE, "IdleSneak");
    s.set(p::MOVING, 1.0);
    expect(&mut s, 0.6, states::SNEAK, "SneakForward");
    s.set(p::SNEAKING, 0.0);
    s.set(p::MOVING, 0.0);
    // Jump: airborne plays Jump; its land key fires.
    s.set(p::ON_GROUND, 0.0);
    let t0 = s.time;
    expect(&mut s, 0.3, states::JUMP, "Jump");
    s.run(0.7, DT);
    assert_eq!(count(&s, "land", "", t0), 1, "{:?}", s.fired("land", t0));
    s.set(p::ON_GROUND, 1.0);
    expect(&mut s, 0.5, states::IDLE, "Idle");
    // Swim.
    s.set(p::SWIMMING, 1.0);
    expect(&mut s, 0.6, states::SWIM_IDLE, "IdleSwim");
    s.set(p::TURN, 2.0);
    expect(&mut s, 0.6, states::SWIM_TURN_LEFT, "SwimTurnLeft");
    s.set(p::TURN, 0.0);
    s.set(p::MOVING, 1.0);
    expect(&mut s, 0.6, states::SWIM_WALK, "SwimWalkForward");
    s.set(p::RUNNING, 1.0);
    let t0 = s.time;
    expect(&mut s, 2.0, states::SWIM_RUN, "SwimRunForward");
    assert!(count(&s, "footstep", "left", t0) >= 1, "{:?}", s.fired("footstep", t0));
    for v in [p::SWIMMING, p::RUNNING, p::MOVING] {
        s.set(v, 0.0);
    }
    expect(&mut s, 0.5, states::IDLE, "Idle");
    // Idle variants: the AI roll picks the number; the graph plays it once.
    for n in 2..=9u8 {
        s.set(p::IDLE_VARIANT, f32::from(n));
        s.set(p::FIDGET, 1.0);
        let clip = format!("Idle{n}");
        let duration = set.group(&clip).unwrap().duration();
        expect(&mut s, 0.3, states::IDLE, &clip);
        s.run(duration, DT);
        expect(&mut s, 0.5, states::IDLE, "Idle");
    }
    // Hits (by variant), knockdown, knockout, death; each returns to idle.
    for n in 1..=5u8 {
        s.set(p::HIT_VARIANT, f32::from(n));
        s.set(p::HIT, 1.0);
        expect(&mut s, 0.2, &format!("hit{n}"), &format!("Hit{n}"));
        s.run(1.3, DT);
        expect(&mut s, 0.3, states::IDLE, "Idle");
    }
    let t0 = s.time;
    s.set(p::HIT_VARIANT, 2.0);
    s.set(p::HIT, 1.0);
    s.run(1.0, DT);
    assert_eq!(count(&s, "sound_gen", "roar", t0), 1, "Hit2 roars");
    s.set(p::KNOCKDOWN, 1.0);
    expect(&mut s, 0.3, states::KNOCKDOWN, "Knockdown");
    s.run(2.6, DT);
    expect(&mut s, 0.3, states::IDLE, "Idle");
    s.set(p::KNOCKED_OUT, 1.0);
    expect(&mut s, 6.0, states::KNOCKOUT, "Knockout");
    s.set(p::KNOCKED_OUT, 0.0);
    expect(&mut s, 0.3, states::KNOCKOUT_END, "Knockout@tail");
    s.run(1.7, DT);
    expect(&mut s, 0.3, states::IDLE, "Idle");
    for n in 1..=5u8 {
        s.set(p::DEATH_VARIANT, f32::from(n));
        s.set(p::DEAD, 1.0);
        expect(&mut s, 4.0, &format!("death{n}"), &format!("Death{n}"));
        s.set(p::DEAD, 0.0);
        expect(&mut s, 0.5, states::IDLE, "Idle");
    }

    // Upper-body slot: a one-hand chop while walking; the legs keep walking.
    s.set(p::STANCE, Stance::OneHand.value());
    s.set(p::MOVING, 1.0);
    expect(&mut s, 0.6, states::WALK, "WalkForward1h");
    let chop = s.request("WeaponOneHand@chop").expect("chop is a slot clip");
    let t0 = s.time;
    let id = s.play_slot(slots::UPPER, chop).unwrap();
    s.run(2.2, DT);
    assert!(s.plays("WalkForward1h", 0.5), "{:?}", s.playing());
    let attack: Vec<_> =
        s.events.iter().filter(|e| e.time > t0 && e.montage == Some(id)).map(|e| (e.name.as_str(), e.payload.as_str(), e.edge)).collect();
    for want in [
        ("attack.charge", "chop", EventEdge::Enter),
        ("attack.charge", "chop", EventEdge::Exit),
        ("attack.min_hit", "chop", EventEdge::Point),
        ("attack.hit", "chop", EventEdge::Point),
    ] {
        assert!(attack.contains(&want), "{want:?} in {attack:?}");
    }
    assert!(s.notices.iter().any(|(_, n)| n.montage == id && n.kind == NoticeKind::Ended { interrupted: false }));
    s.set(p::MOVING, 0.0);
    s.set(p::STANCE, 0.0);

    // Scripted: `LoopGroup Idle3 2 1` plays Idle3, then its loop once more, in the full-body slot.
    let group = "Idle3";
    let slot = mw_graphs::script::bound_group(&s.graph, group).unwrap();
    let cooked = s.clips[usize::from(slot)].clone();
    let req = GroupRequest::looped(group, 2, 1).slot_request(MontageClip::Bound(slot), cooked.loop_range);
    let t0 = s.time;
    let id = s.play_slot(SCRIPT_SLOT, req).unwrap();
    s.run(cooked.duration * 2.0 + 0.5, DT);
    let ended = s.notices.iter().find(|(_, n)| n.montage == id && n.kind == NoticeKind::Ended { interrupted: false }).map(|(t, _)| t - t0);
    let (a, b) = cooked.loop_range.unwrap();
    let expected = cooked.duration + (b - a);
    assert!(ended.is_some_and(|t| (t - expected).abs() < 0.15), "ended after {ended:?}, expected {expected}");
    // A gameplay montage can't interrupt a scripted one (priority).
    let id = s.play_slot(SCRIPT_SLOT, GroupRequest::play(group, 1).slot_request(MontageClip::Bound(slot), None)).unwrap();
    s.run(0.2, DT);
    let gameplay = s.request("Idle4").unwrap();
    assert!(s.play_slot(SCRIPT_SLOT, gameplay).is_err(), "busy");
    assert!(s.instance.montage(s.graph.slot(SCRIPT_SLOT).unwrap()).is_some_and(|m| m.id == id));
}

/// CREA `(id, model, flags)` from the base game and expansions.
fn creatures(install: &MorrowindInstall) -> Vec<(String, String, u32)> {
    let mut out = Vec::new();
    for file in ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm"] {
        let Some(path) = install.data_file(file).into_iter().next() else { continue };
        let bytes = std::fs::read(path).unwrap();
        let mut plugin = tes3::esp::Plugin::new();
        plugin.load_bytes(&bytes).unwrap();
        out.extend(
            plugin.objects_of_type::<tes3::esp::Creature>().map(|c| (c.id.to_ascii_lowercase(), c.mesh.clone(), u32::from(c.creature_flags.bits()))),
        );
    }
    out
}

fn creature_set(vfs: &Vfs, model: &str, flags: u32) -> Result<AnimSet, String> {
    let biped = mw_graphs::creature_template(flags) == Template::Humanoid;
    let (path, anims) = mw_graphs::creature_animations(model, biped, |p| exists(vfs, p));
    let mut set = load_set(vfs, &rig::model_key(model), &path, &anims)?;
    if biped {
        return Ok(set.with_humanoid_rig());
    }
    set.profile = rig::creature_profile(&set.skeleton, model).ok();
    Ok(set)
}

/// Every CREA model compiles its template (biped creatures the humanoid
/// one); the report lists, per model, the mover, the groups and what dropped.
#[test]
fn creature_coverage() {
    let Some((install, vfs)) = install() else { return };
    let mut models: BTreeMap<String, (String, Vec<String>, u32)> = BTreeMap::new();
    for (id, mesh, flags) in creatures(&install) {
        let e = models.entry(mesh.to_ascii_lowercase()).or_insert_with(|| (mesh.clone(), Vec::new(), flags));
        e.1.push(id);
    }
    let mut rows = Vec::new();
    for (mesh, ids, flags) in models.into_values() {
        let mover = CreatureMover::from_crea_flags(flags);
        let template = mw_graphs::creature_template(flags);
        let (groups, result) = match creature_set(&vfs, &mesh, flags) {
            Ok(mut set) => (set.group_names().count(), mw_graphs::validate(&mut set, template)),
            Err(e) => (0, Err(e)),
        };
        rows.push(Coverage { model: mesh, creatures: ids, mover, template, groups, result });
    }
    eprintln!("{}", mw_graphs::coverage(&rows));
    let failed: Vec<_> = rows.iter().filter_map(|r| r.result.as_ref().err().map(|e| (&r.model, e))).collect();
    assert!(rows.len() > 80, "{} models", rows.len());
    assert!(rows.iter().filter(|r| r.template == Template::Humanoid).count() >= 6, "biped creatures");
    assert!(failed.is_empty(), "{failed:#?}");
    // Every walker can walk, every swimmer swim (directly or by fallback).
    for r in &rows {
        let v = r.result.as_ref().unwrap();
        let need = match r.mover {
            CreatureMover::Swim => states::SWIM_WALK,
            _ => states::WALK,
        };
        if !v.states.iter().any(|s| s == need) {
            eprintln!("{} ({:?}): no {need} state", r.model, r.mover);
        }
    }
}

/// Guar (ground), cliff racer (fly) and slaughterfish (swim) on the
/// creature template: movement, footsteps, attacks in the full-body slot.
#[test]
fn creature_simulation() {
    let Some((_, vfs)) = install() else { return };
    let mut guar = creature_set(&vfs, "r/Guar.NIF", 0).unwrap();
    let mut s = sim(&mut guar, Template::Creature);
    expect(&mut s, 0.5, states::IDLE, "Idle");
    s.set(p::MOVING, 1.0);
    let t0 = s.time;
    expect(&mut s, 3.0, states::WALK, "WalkForward");
    assert!(!s.fired("footstep", t0).is_empty(), "guar footsteps");
    s.set(p::RUNNING, 1.0);
    expect(&mut s, 1.0, states::RUN, "RunForward");
    s.set(p::MOVING, 0.0);
    s.set(p::RUNNING, 0.0);
    let attack = s.request("Attack1").expect("guar Attack1");
    let t0 = s.time;
    let id = s.play_slot(slots::FULL, attack).unwrap();
    s.run(2.0, DT);
    assert!(s.events.iter().any(|e| e.time > t0 && e.montage == Some(id) && e.name == "attack.hit"), "guar attack hit");
    // AN-13: the guar has no `Knockout` / `Knockout@tail`; clearing the flag
    // (the ragdoll's get-up) must still leave the knockout state.
    s.set(p::KNOCKED_OUT, 1.0);
    assert_eq!(settle(&mut s, 1.0), states::KNOCKOUT);
    s.set(p::KNOCKED_OUT, 0.0);
    expect(&mut s, 1.0, states::IDLE, "Idle");
    s.set(p::DEAD, 1.0);
    expect(&mut s, 3.0, "death1", "Death1");

    let mut racer = creature_set(&vfs, "r/CliffRacer.NIF", 0).unwrap();
    let mut s = sim(&mut racer, Template::Creature);
    s.set(p::MOVING, 1.0);
    let (state, top) = (settle(&mut s, 1.0), s.playing());
    eprintln!("cliff racer flying: {state} {top:?}");
    assert!(state == states::WALK || state == states::RUN, "{state}");

    let mut fish = creature_set(&vfs, "r/Slaughterfish.NIF", 0).unwrap();
    let mut s = sim(&mut fish, Template::Creature);
    s.set(p::SWIMMING, 1.0);
    s.set(p::MOVING, 1.0);
    let (state, top) = (settle(&mut s, 1.0), s.playing());
    eprintln!("slaughterfish swimming: {state} {top:?}");
    assert_eq!(state, states::SWIM_WALK, "{top:?}");
}

/// Planted fraction per foot of every locomotion clip (walk, run, sneak,
/// swim; humanoid sets and every CREA model), from the cook's contact
/// curves: the census behind the contact detection (walkers3). Forward
/// cycles plant each foot for a good part of the cycle; sidesteps too
/// (they used to plant on 0–6 of 81 frames, so every strafe skated).
#[test]
fn contact_census() {
    let Some((install, vfs)) = install() else { return };
    let locomotion = |g: &str| {
        ["Walk", "Run", "Sneak", "SwimWalk", "SwimRun"].iter().any(|p| g.starts_with(p)) && !g.contains("Turn")
    };
    let mut sets: Vec<AnimSet> = HUMANOID_SETS.iter().filter(|s| !s.first_person).map(|s| humanoid(&vfs, s)).collect();
    let mut seen = std::collections::BTreeSet::new();
    for (_, mesh, flags) in creatures(&install) {
        if mw_graphs::creature_template(flags) != Template::Humanoid
            && seen.insert(mesh.to_ascii_lowercase())
            && let Ok(s) = creature_set(&vfs, &mesh, flags)
        {
            sets.push(s);
        }
    }
    let (mut rows, mut low) = (0, Vec::new());
    for set in &mut sets {
        let groups: Vec<String> = set.group_names().filter(|g| locomotion(g)).map(str::to_owned).collect();
        for g in groups {
            let Ok(clip) = set.cooked(&g) else { continue };
            // The cycle that repeats (a group's intro and outro play once).
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let cycle = clip.loop_range.map(|(a, b)| ((a * clip.rate).round() as usize, (b * clip.rate).round() as usize));
            let fractions: Vec<f32> = clip
                .curves
                .iter()
                .filter(|c| c.name.starts_with("contact."))
                .map(|c| {
                    let (a, b) = cycle.filter(|(a, b)| a < b).map_or((0, c.values.len()), |(a, b)| (a, (b + 1).min(c.values.len())));
                    let v = &c.values[a..b];
                    #[allow(clippy::cast_precision_loss)]
                    let f = v.iter().filter(|&&v| v > 0.5).count() as f32 / v.len().max(1) as f32;
                    f
                })
                .collect();
            if fractions.is_empty() {
                continue;
            }
            rows += 1;
            let line = format!("{} {g}: {:?}", set.name, fractions.iter().map(|f| format!("{:.0}%", f * 100.0)).collect::<Vec<_>>());
            eprintln!("census {line}");
            let walking = !g.starts_with("Swim");
            if walking && fractions.iter().any(|&f| f < 0.15) {
                low.push(line);
            }
        }
    }
    eprintln!("census: {rows} clips, {} with a foot planted under 15% of the cycle", low.len());
    for l in &low {
        eprintln!("census low: {l}");
    }
    assert!(rows > 100, "{rows} clips");
    let humanoid_low: Vec<_> = low.iter().filter(|l| l.starts_with("base_anim")).collect();
    assert!(humanoid_low.is_empty(), "humanoid locomotion with a foot that never plants: {humanoid_low:#?}");
}

/// AN-6: the get-up clip (`Knockout@tail`) is authored in place: the actor
/// doesn't move while knocked out, so the clip starts lying beside its origin
/// and ends standing over it. Cooked with root-motion extraction, the Biped
/// root kept its lying frame-0 ground position the whole clip (the extracted
/// track went to a mover nobody fed), so the character stood up beside its
/// capsule and slid onto it when the clip ended. The knock clips cook in
/// place (`mw_anim::clip_rules`): the clip must end with the root over the
/// origin, and carry no root track.
#[test]
fn get_up_clip_ends_standing_over_its_origin() {
    let Some((_, vfs)) = install() else { return };
    let mut set = humanoid(&vfs, &HUMANOID_SETS[0]);
    let clip = set.cooked("Knockout@tail").unwrap();
    let skeleton = set.skeleton.clone();
    let root = skeleton.root_motion_joint().expect("Bip01");
    let at = |t: f32| {
        let mut local = sf_anim::LocalPose::rest(&skeleton);
        clip.sample(t, &mut local, None);
        let mut model = sf_anim::ModelPose::new(skeleton.len());
        sf_anim::local_to_model(&skeleton, &local, &mut model, &glam::Affine3A::IDENTITY);
        glam::Vec3::from(model.mats[root].translation)
    };
    let (start, end) = (at(0.0), at(clip.duration));
    let track = clip.root_motion.as_ref().and_then(|r| r.frames.last().copied());
    eprintln!("Knockout@tail {:.2} s: root at start {start:.2}, at end {end:.2}; root track end {track:?}", clip.duration);
    let ground = Vec2::new(end.x, end.z).length();
    assert!(ground < 0.1, "the get-up ends {ground:.2} m from its origin (root at {end:.2})");
    assert!(track.is_none_or(|f| glam::Vec3::new(f[0], f[1], f[2]).length() < 1e-3 && f[3].abs() < 1e-3), "root track {track:?}");
}
