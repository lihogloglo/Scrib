//! The owner's A/B ladder on the player's walk (WALK-0, ROADMAP 3.4c A0): each switch zeroes
//! one locomotion op's output through the humanoid graph's gate params, and at 1 the graph
//! plays exactly as with no gates. Each op is checked against a graph without it, the other
//! ops fixed so the base animation is the same (`SpeedMatch` gated to 0 plays every clip at
//! its own rate whether or not a stride warp runs). Skips without an install.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use glam::{Vec2, Vec3, Vec4};
use mw_anim::Animation;
use mw_anim::rig;
use mw_graphs::build::{lane_direction, params as p};
use mw_graphs::sim::Sim;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim_graph::GroundSample;
use sf_anim_graph::NodeKind;
use sf_anim_graph::asset::{GraphAsset, OpSpec};
use sf_vfs::{Vfs, VfsPath};

const DT: f32 = 1.0 / 60.0;
/// The player's walk speed, m/s (the clip is authored at ~2.2).
const WALK: f32 = 1.6;

fn humanoid_set() -> Option<AnimSet> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let set = &HUMANOID_SETS[0];
    let model = mw_nif::load(&read(set.skeleton).unwrap()).unwrap();
    let skeleton = rig::skeleton(&model).unwrap();
    let mut anims = Vec::new();
    for a in set.animations {
        let Some(bytes) = read(a) else { continue };
        let anim = Animation::from_file(a, &bytes).unwrap();
        if !anim.text_keys.is_empty() {
            anims.push(((*a).to_owned(), anim));
        }
    }
    Some(AnimSet::new(set.name, skeleton, anims).with_humanoid_rig())
}

/// A sim of `asset` (a variant of the humanoid graph).
fn sim_of(set: &mut AnimSet, asset: &GraphAsset) -> Sim {
    let graph = set.compile("walk_ladder", asset).unwrap();
    let clips = set.binding_clips(&graph).unwrap();
    Sim::new(graph, set.skeleton.clone(), clips).unwrap()
}

/// The humanoid graph with only the ops `keep` picks.
fn with_ops(keep: impl Fn(&OpSpec) -> bool) -> GraphAsset {
    let mut a = mw_graphs::asset(Template::Humanoid);
    a.procedural.retain(|o| keep(o));
    a
}

fn is_speed_match(o: &OpSpec) -> bool {
    matches!(o, OpSpec::SpeedMatch { .. })
}

/// The humanoid graph as it was before the gates (every op, none gated by the ladder).
fn ungated() -> GraphAsset {
    let mut a = mw_graphs::asset(Template::Humanoid);
    for o in &mut a.procedural {
        match o {
            OpSpec::SpeedMatch { gate, .. } | OpSpec::StrideWarp { gate, .. } | OpSpec::Lean { gate, .. } => *gate = None,
            _ => {}
        }
    }
    a
}

/// Walking (running past 3 m/s) straight ahead at `speed` on a ramp rising `grade` (rise over run) along the travel,
/// the ground sampled under each probe origin, for `seconds`.
fn walk(sim: &mut Sim, speed: f32, grade: f32, seconds: f32) {
    sim.set(p::MOVING, 1.0);
    // The player's gaits: walk 1.6, run 4.5 m/s.
    sim.set(p::RUNNING, if speed > 3.0 { 1.0 } else { 0.0 });
    sim.set_vec2(p::MOVE, Vec2::Y);
    sim.velocity = Vec3::new(0.0, 0.0, speed);
    let normal = Vec3::new(0.0, 1.0, -grade).normalize();
    let steps = (seconds / DT).round() as usize;
    for _ in 0..steps {
        if grade != 0.0 {
            let origins = sim.out.probe_origins.clone();
            for (i, g) in sim.ground.iter_mut().enumerate() {
                let o = origins.get(i).copied().unwrap_or(Vec3::ZERO);
                *g = Some(GroundSample { point: Vec3::new(o.x, o.z * grade, o.z), normal });
            }
        }
        sim.run(DT, DT);
    }
}

/// The largest joint translation (m) or rotation (quaternion components, either sign)
/// difference between two sims' last poses. Not `Quat::angle_between`: its `acos` near 1 reads
/// a one-ulp norm error as 1e-3 rad.
fn pose_gap(a: &Sim, b: &Sim) -> f32 {
    let (pa, pb) = (&a.out.model, &b.out.model);
    (0..a.skeleton.len())
        .map(|j| {
            let (qa, qb) = (Vec4::from(pa.rotation(j)), Vec4::from(pb.rotation(j)));
            (pa.translation(j) - pb.translation(j)).length().max((qa - qb).length().min((qa + qb).length()))
        })
        .fold(0.0, f32::max)
}

/// The walk blend's lanes (clip reference, weight) in the last update.
fn walk_lanes(sim: &Sim) -> Vec<(String, f32)> {
    let n = sim.graph.nodes.iter().position(|n| n.name.ends_with("walk.none")).expect("walk.none");
    let NodeKind::Player(def) = &sim.graph.nodes[n].kind else { panic!("walk.none: not a player") };
    let (lanes, _) = sim.instance.player(u32::try_from(n).unwrap()).unwrap();
    def.lanes.iter().zip(lanes).map(|(&c, (_, w))| (sim.graph.clips[usize::from(c)].reference.clone(), w)).collect()
}

/// The forward walk lane's playback rate over the next update (clip seconds per second).
fn walk_rate(sim: &mut Sim) -> f32 {
    let forward = walk_lanes(sim).into_iter().find(|(c, _)| c.contains("Forward")).expect("forward lane").0;
    for _ in 0..30 {
        let t0 = sim.clip_time(&forward).expect("forward walk plays");
        sim.run(DT, DT);
        let t1 = sim.clip_time(&forward).expect("forward walk plays");
        if t1 > t0 {
            return (t1 - t0) / DT;
        }
    }
    panic!("the forward walk never advanced");
}

#[test]
fn speed_match_gate_plays_the_clip_at_its_own_rate() {
    let Some(mut set) = humanoid_set() else { return };
    let mut gated = sim_of(&mut set, &mw_graphs::asset(Template::Humanoid));
    gated.set(p::SPEED_MATCH, 0.0);
    walk(&mut gated, WALK, 0.0, 1.0);
    let off = walk_rate(&mut gated);
    let mut on = sim_of(&mut set, &mw_graphs::asset(Template::Humanoid));
    walk(&mut on, WALK, 0.0, 1.0);
    let on = walk_rate(&mut on);
    eprintln!("walk at {WALK} m/s: rate {on:.3} with speed match, {off:.3} without");
    assert!((off - 1.0).abs() < 1e-3, "speed match off plays at rate 1, got {off}");
    assert!((on - 1.0).abs() > 0.05, "speed match on changes the rate at {WALK} m/s, got {on}");
}

#[test]
fn stride_warp_gate_leaves_the_pose_as_animated() {
    let Some(mut set) = humanoid_set() else { return };
    let warp = |o: &OpSpec| is_speed_match(o) || matches!(o, OpSpec::StrideWarp { .. });
    let mut none = sim_of(&mut set, &with_ops(is_speed_match));
    none.set(p::SPEED_MATCH, 0.0);
    walk(&mut none, WALK, 0.0, 2.0);
    for (gate, zeroed) in [(0.0, true), (1.0, false)] {
        let mut s = sim_of(&mut set, &with_ops(warp));
        s.set(p::SPEED_MATCH, 0.0);
        s.set(p::STRIDE_WARP, gate);
        walk(&mut s, WALK, 0.0, 2.0);
        let gap = pose_gap(&s, &none);
        eprintln!("stride warp gate {gate}: pose gap {gap:.5} vs no warp");
        if zeroed {
            assert!(gap < 1e-5, "stride warp off: the pose as animated, gap {gap}");
            assert_eq!(s.out.probe_origins, none.out.probe_origins, "stride warp off: probes not warped");
        } else {
            assert!(gap > 1e-3, "stride warp on warps the {WALK} m/s walk, gap {gap}");
        }
    }
}

#[test]
fn lean_gate_leaves_the_trunk_as_animated() {
    let Some(mut set) = humanoid_set() else { return };
    let lean = |o: &OpSpec| is_speed_match(o) || matches!(o, OpSpec::Lean { .. });
    // A 20° climb (the Lean op pitches by the ground grade only: gain 0).
    let grade = 20f32.to_radians().tan();
    let mut none = sim_of(&mut set, &with_ops(is_speed_match));
    none.set(p::SPEED_MATCH, 0.0);
    walk(&mut none, WALK, grade, 2.0);
    for (gate, zeroed) in [(0.0, true), (1.0, false)] {
        let mut s = sim_of(&mut set, &with_ops(lean));
        s.set(p::SPEED_MATCH, 0.0);
        s.set(p::LEAN, gate);
        walk(&mut s, WALK, grade, 2.0);
        let gap = pose_gap(&s, &none);
        eprintln!("lean gate {gate}: pose gap {gap:.5} vs no lean (grade {:.3})", s.out.grade);
        if zeroed {
            assert!(gap < 1e-5, "lean off: the trunk as animated, gap {gap}");
        } else {
            assert!(gap > 1e-3, "lean on pitches the trunk uphill, gap {gap}");
        }
    }
}

#[test]
fn forward_lane_only_drops_the_side_and_back_lanes() {
    let Some(mut set) = humanoid_set() else { return };
    for forward_only in [true, false] {
        let mut s = sim_of(&mut set, &mw_graphs::asset(Template::Humanoid));
        walk(&mut s, WALK, 0.0, 0.5);
        let mut bleed = 0.0f32;
        let mut forward = 1.0f32;
        // A turn through every direction, as the body's measured travel swings during a fast turn.
        for step in 0..120 {
            let a = step as f32 * std::f32::consts::TAU / 120.0;
            let d = Vec2::new(a.sin(), a.cos());
            assert_eq!(lane_direction(d, false), d, "the switch on feeds the direction unchanged");
            s.set_vec2(p::MOVE, lane_direction(d, forward_only));
            s.velocity = Vec3::new(d.x, 0.0, d.y) * WALK;
            s.run(DT, DT);
            for (clip, w) in walk_lanes(&s) {
                if clip.contains("Forward") {
                    forward = forward.min(w);
                } else {
                    bleed = bleed.max(w);
                }
            }
        }
        eprintln!("forward lane only {forward_only}: side/back lanes up to {bleed:.3}, forward down to {forward:.3}");
        if forward_only {
            assert!(bleed < 1e-4, "forward lane only: no side or back lane, got {bleed}");
            assert!(forward > 1.0 - 1e-4, "forward lane only: the forward lane takes it all, got {forward}");
        } else {
            assert!(bleed > 0.5, "with the side lanes a full turn plays them, got {bleed}");
        }
    }
    assert_eq!(lane_direction(Vec2::ZERO, true), Vec2::ZERO, "standing stays still");
}

#[test]
fn gates_at_one_play_as_the_ungated_graph() {
    let Some(mut set) = humanoid_set() else { return };
    let grade = 15f32.to_radians().tan();
    let mut gated = sim_of(&mut set, &mw_graphs::asset(Template::Humanoid));
    let mut plain = sim_of(&mut set, &ungated());
    for s in [&mut gated, &mut plain] {
        walk(s, WALK, grade, 2.0);
        walk(s, 4.5, grade, 1.0);
    }
    let gap = pose_gap(&gated, &plain);
    assert!(gap == 0.0, "gates at 1: the shipped behaviour, gap {gap}");
    assert_eq!(gated.out.probe_origins, plain.out.probe_origins);
}
