//! The modern gait set on the real install and the user's library (ROADMAP 3.4c A3): lane
//! per speed band, and a walk <-> run state switch keeping the foot phase (A1 measured a
//! restart at the clip start, up to half a cycle off). Exact checks on blend weights and
//! playheads; skipped without the install or the library's gait takes.
//! `cargo test -p mw_graphs --test gait_set`.

use std::sync::Arc;

use mw_anim::{Animation, rig};
use mw_bake::sources::AnimSources;
use mw_graphs::build::{gaits, params as p};
use mw_graphs::sim::Sim;
use mw_graphs::sources::SourcesManifest;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

const DT: f32 = 1.0 / 60.0;

/// The gait clips by reference, cooked once per test binary (`None`: not in the library).
type Cooked = Option<Vec<(String, Arc<sf_anim::Clip>)>>;
static COOKED: std::sync::OnceLock<Cooked> = std::sync::OnceLock::new();

/// The shipped humanoid graph on `base_anim` with the gait slots cooked the bake's way;
/// `None` (skip) without the install or when a gait take is missing.
fn gait_sim() -> Option<Sim> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let set = &HUMANOID_SETS[0];
    let skeleton = rig::skeleton(&mw_nif::load(&read(set.skeleton).unwrap()).unwrap()).unwrap();
    let anims = set
        .animations
        .iter()
        .filter_map(|a| Some(((*a).to_owned(), Animation::from_file(a, &read(a)?).unwrap())))
        .filter(|(_, a)| !a.text_keys.is_empty())
        .collect();
    let mut set = AnimSet::new(set.name, skeleton, anims).with_humanoid_rig();
    let cooked = COOKED.get_or_init(|| cook_gait_set(&set.skeleton)).as_ref()?;
    for (reference, clip) in cooked {
        set.insert_cooked(reference.clone(), clip.clone());
    }
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let clips = set.binding_clips(&v.graph).unwrap();
    Some(Sim::new(v.graph, set.skeleton.clone(), clips).unwrap())
}

/// The gait slots' first candidates cooked onto `skeleton` (the bake's path), by reference;
/// `None` when one is missing from the library.
fn cook_gait_set(skeleton: &sf_anim::Skeleton) -> Cooked {
    let manifest = SourcesManifest::builtin();
    let library = AnimSources::local();
    let target = mw_bake::anim::HumanoidTarget::new(skeleton.clone()).unwrap();
    let mut out = Vec::new();
    for (gait, _) in gaits::SET {
        let name = manifest.gait_slot("humanoid", gaits::STYLE, gait, "cycle").expect("shipped gait slot");
        let slot = &manifest.slots[&name];
        let c = &slot.candidates[0];
        let file = c.file.clone().expect("generated takes name their file");
        let Some(path) = library.real_path(&AnimSources::virtual_path(&c.source, &file)) else {
            eprintln!("{name}: {file} not in the library; skipping");
            return None;
        };
        let mut clip = mw_bake::anim::cook_library_clip(&c.source, &path, Some(&c.clip), &slot.rules(c), &target).unwrap();
        assert!(clip.loop_range.is_some(), "{name}: the loop cut found a loop");
        clip.name = c.reference();
        out.push((c.reference(), Arc::new(clip)));
    }
    Some(out)
}

fn node(sim: &Sim, name: &str) -> u32 {
    sim.graph.node(name).unwrap_or_else(|| panic!("no node {name}"))
}

/// Moving straight ahead at `speed` m/s (running from the jog's band on, as the player).
fn feed(sim: &mut Sim, speed: f32, running: bool) {
    sim.set(p::MOVING, 1.0);
    sim.set(p::RUNNING, if running { 1.0 } else { 0.0 });
    sim.set(p::SPEED, speed);
    sim.set_vec2(p::MOVE, glam::Vec2::Y);
    sim.velocity = glam::Vec3::new(0.0, 0.0, speed);
}

/// Lane weights of the walk state's gait set at speeds across the bands: one lane at each
/// gait speed (and beyond the ends), two in between, linear in speed.
#[test]
fn the_lane_follows_the_speed_band() {
    let Some(mut sim) = gait_sim() else { return };
    let set = node(&sim, "walk.none.gait");
    for (speed, want) in [
        (0.8, [1.0, 0.0, 0.0]),
        (1.6, [1.0, 0.0, 0.0]),
        (3.05, [0.5, 0.5, 0.0]),
        (4.5, [0.0, 1.0, 0.0]),
        (5.75, [0.0, 0.5, 0.5]),
        (7.0, [0.0, 0.0, 1.0]),
        (9.0, [0.0, 0.0, 1.0]),
    ] {
        feed(&mut sim, speed, false);
        sim.run(1.0, DT);
        let (lanes, w) = sim.instance.player(set).expect("the walk's gait set plays");
        assert!((w - 1.0).abs() < 1e-3, "{speed} m/s: player weight {w}");
        let got: Vec<f32> = lanes.iter().map(|l| l.1).collect();
        for (g, x) in got.iter().zip(want) {
            assert!((g - x).abs() < 1e-3, "{speed} m/s: lanes {got:?}, want {want:?}");
        }
    }
}

/// Foot phase of a lane time: the fraction of the loop since leg 0's contact rises (as A1's
/// `foot_phase`), so walk and run phases compare.
fn phase(sim: &Sim, clip: u16, time: f32) -> f32 {
    const STEPS: u16 = 240;
    let cooked = &sim.clips[usize::from(clip)];
    let (start, end) = cooked.loop_range.expect("a loop");
    let len = end - start;
    let leg0 = &sim.graph.curves[usize::from(sim.graph.legs[0].contact.expect("contact"))];
    let curve = cooked.curves.iter().position(|x| &x.name == leg0).expect("contact curve");
    let at = |step: u16| start + len * f32::from(step % STEPS) / f32::from(STEPS);
    let contact = |step: u16| cooked.curve(curve, at(step));
    let plant = (0..STEPS).find(|&s| contact(s + STEPS - 1) < 0.5 && contact(s) >= 0.5).map_or(start, at);
    ((time - plant) / len).rem_euclid(1.0)
}

/// The heaviest lane of a gait set: (clip slot, time).
fn top(sim: &Sim, set: u32) -> (u16, f32) {
    let sf_anim_graph::NodeKind::Player(pl) = &sim.graph.nodes[set as usize].kind else { panic!("not a player") };
    let (lanes, _) = sim.instance.player(set).expect("playing");
    let i = (0..lanes.len()).max_by(|&x, &y| lanes[x].1.total_cmp(&lanes[y].1)).unwrap();
    (pl.lanes[i], lanes[i].0)
}

/// Walk -> run -> walk at the walk speed (the state switches, the lane does not) and at
/// the gait speeds (1.6 -> 4.5 m/s: the lane changes too), at five points of the cycle each
/// way: the entering state's gait set joins the leaving one's foot phase (jump ~0, was
/// -0.46..+0.49 in A1), printed as A1's switch table.
#[test]
fn a_gait_switch_keeps_the_foot_phase() {
    let Some(probe) = gait_sim() else { return };
    let (walk, run) = (node(&probe, "walk.none.gait"), node(&probe, "run.none.gait"));
    eprintln!("switch     speed m/s  | before ph | after ph | jump (cycles)");
    for (from_speed, to_speed) in [(1.6, 1.6), (1.6, 4.5)] {
        let mut worst = 0.0f32;
        for k in 0..5u8 {
            let mut sim = gait_sim().expect("as above");
            feed(&mut sim, from_speed, false);
            sim.run(2.0 + f32::from(k) * 0.2, DT);
            let (c0, t0) = top(&sim, walk);
            let before = phase(&sim, c0, t0);
            // Walk -> run: one frame later the run's gait set leads.
            feed(&mut sim, to_speed, true);
            sim.run(DT, DT);
            let (c1, t1) = top(&sim, run);
            let after = phase(&sim, c1, t1);
            let jump = (after - before + 0.5).rem_euclid(1.0) - 0.5;
            eprintln!("walk>run   {from_speed:.1} > {to_speed:.1} | {before:9.3} | {after:8.3} | {jump:+.3}");
            worst = worst.max(jump.abs());
            // Run -> walk after a while.
            sim.run(0.37 + f32::from(k) * 0.11, DT);
            let (c0, t0) = top(&sim, run);
            let before = phase(&sim, c0, t0);
            feed(&mut sim, from_speed, false);
            sim.run(DT, DT);
            let (c1, t1) = top(&sim, walk);
            let after = phase(&sim, c1, t1);
            let jump = (after - before + 0.5).rem_euclid(1.0) - 0.5;
            eprintln!("run>walk   {to_speed:.1} > {from_speed:.1} | {before:9.3} | {after:8.3} | {jump:+.3}");
            worst = worst.max(jump.abs());
        }
        // The state switch alone: exactly the frame the joined set then plays (1/60 s is 0.016
        // of the 1.06 s walk loop). With the speed jumping too, the heaviest lane may change
        // (walk and jog are aligned by their foot-down markers, the phase here by the contact
        // curve) and the frame plays at the new rate: under 0.12 cycle (measured 0.09).
        let limit = if (from_speed - to_speed).abs() < 1e-3 { 0.03 } else { 0.12 };
        assert!(worst < limit, "{from_speed} > {to_speed} m/s: largest phase jump {worst:.3} cycles");
    }
}

/// The A0 ladder's `speed_match` gate (WALK-0) works on the gait lanes: 0 plays the jog at
/// its own rate, 1 at the matched rate (4.5 m/s on the ~5.3 m/s jog: ~0.92, inside the clamp).
#[test]
fn the_speed_match_gate_reaches_the_gait_lanes() {
    let Some(mut sim) = gait_sim() else { return };
    let set = node(&sim, "run.none.gait");
    let rate = |sim: &mut Sim, gate: f32| {
        sim.set(p::SPEED_MATCH, gate);
        feed(sim, 4.5, true);
        sim.run(1.0, DT);
        let (_, t0) = top(sim, set);
        sim.run(DT, DT);
        let (_, t1) = top(sim, set);
        (t1 - t0) / DT
    };
    let off = rate(&mut sim, 0.0);
    let on = rate(&mut sim, 1.0);
    eprintln!("jog rate at 4.5 m/s: gate off {off:.3}, on {on:.3}");
    assert!((off - 1.0).abs() < 1e-3, "gate off: {off}");
    assert!(on > gaits::RATE_CLAMP.0 && on < 0.99, "gate on: {on}");
}
