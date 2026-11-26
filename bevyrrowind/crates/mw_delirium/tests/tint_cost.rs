//! Cost of a "physics tint" (partial physics on an animated character): per-tick cost of N Delirium agents in each
//! drive state, on Morrowind's humanoid skeleton (`meshes/base_anim.nif`), headless. Skips (passes) when no Morrowind
//! install is detected. Measurement only, no timing limits:
//! `cargo test -p mw_delirium --release --test tint_cost -- --ignored --nocapture`.
//!
//! Each `Sim` owns its own floor world and agent, so N agents are N independent sims (they do not collide).
//! States are a table (`STATES`): a name, a setup run once and an optional per-tick drive run before each step.

#![allow(clippy::cast_precision_loss)]

use std::sync::OnceLock;
use std::time::Instant;

use delirium::prelude::*;
use mw_delirium::delirium;
use mw_delirium::delirium::msg::Value;
use mw_delirium::sim::{DT, Sim, SpawnPose, Spawned, spawn};
use mw_delirium::{Options, character_from_skeleton};
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

/// The `setStiffness` message's own damping default (`Stiffness::default`, delirium `msg.rs:516`), sent explicitly so
/// the printed value is the one used.
const SET_STIFFNESS_DAMPING: f32 = 1.0;
const WARM_UP: usize = 60;
const TIMED: usize = 600;
/// Ticks after setup (warm-up included) at which `fallen` is counted, and the span of the "first 2 s" median: 2 s at
/// 60 Hz (a tint lives 0.5-2 s).
const FALLEN_AT: usize = 120;
/// The disturbance D and E react to, as `real_install.rs` `parts_round_trip` uses: the chest (part 10), forward.
const PUSH_PART: usize = 10;
const PUSH_NS: f32 = 45.0;

/// Ticks after setup at which `tint_pose_error` samples (0.5, 1 and 2 s at 60 Hz: a tint lives 0.5-2 s).
const POSE_SAMPLES: [usize; 3] = [30, 60, 120];

/// The same build laid out twice: `bind` (where every sim starts) and `zero` (the asset's zero pose, the pose target
/// `tint_pose_error` feeds, so the target differs from the natural stand).
struct Layouts {
    bind: Spawned,
    zero: Spawned,
}

fn layouts() -> Option<&'static Layouts> {
    static F: OnceLock<Option<Layouts>> = OnceLock::new();
    F.get_or_init(|| {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return None;
        };
        let vfs: Vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
        let bytes = vfs.read(&VfsPath::new("meshes/base_anim.nif").unwrap()).unwrap();
        let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
        let body = character_from_skeleton(&skeleton, &Options::default()).unwrap();
        Some(Layouts { bind: spawn(&body, SpawnPose::Bind, None), zero: spawn(&body, SpawnPose::Zero, None) })
    })
    .as_ref()
}

fn fixture() -> Option<&'static Spawned> {
    layouts().map(|l| &l.bind)
}

/// A drive state: `setup` runs once on a fresh sim, `tick` (if any) before every step, inside the timed region.
struct State {
    name: &'static str,
    setup: fn(&mut Sim),
    tick: Option<fn(&mut Sim, &[Transform])>,
}

fn soft_stiffness(s: &mut Sim, stiffness: f32, mask: &str) {
    s.send(
        "setStiffness",
        &[("bodyStiffness", Value::Float(stiffness)), ("mask", Value::Str(mask.into())), ("damping", Value::Float(SET_STIFFNESS_DAMPING))],
    );
}

/// Feeds the target pose and drives the `mask` joints to it (`activePose`).
fn pose_follow(s: &mut Sim, target: &[Transform], mask: &str) {
    s.agent.incoming.clear();
    s.agent.incoming.extend_from_slice(target);
    s.send("activePose", &[("mask", Value::Str(mask.into()))]);
}

/// A reaction behaviour on top of the balancer, then the disturbance it reacts to.
fn reaction(s: &mut Sim, name: &str) {
    s.start("bodyBalance");
    s.start(name);
    let (fwd, _) = s.facing();
    s.push(PUSH_PART, fwd * PUSH_NS);
}

const STATES: &[State] = &[
    // A: the balancer alone.
    State { name: "A bodyBalance", setup: |s| s.start("bodyBalance"), tick: None },
    // B: balancer plus a soft upper body.
    State {
        name: "B balance+soft ub",
        setup: |s| {
            s.start("bodyBalance");
            soft_stiffness(s, 4.0, "ub");
        },
        tick: None,
    },
    // F: no balancer, whole body driven to a pose (the bind pose; a fixed target is fine for cost).
    State {
        name: "F pose fb",
        setup: |s| soft_stiffness(s, 12.0, "fb"),
        tick: Some(|s, target| pose_follow(s, target, "fb")),
    },
    // C: balancer legs, upper body pose-followed.
    State { name: "C balance+pose ub", setup: |s| s.start("bodyBalance"), tick: Some(|s, target| pose_follow(s, target, "ub")) },
    // D: stumble after a chest push.
    State { name: "D stumble", setup: |s| reaction(s, "stumble"), tick: None },
    // E: staggerFall after a chest push.
    State { name: "E staggerFall", setup: |s| reaction(s, "staggerFall"), tick: None },
    // G: F plus the pelvis held on its target (`hardConstraint` reads `agent.incoming`, which F's tick feeds).
    State {
        name: "G pose fb+pinned pelvis",
        setup: |s| {
            soft_stiffness(s, 12.0, "fb");
            s.send(
                "hardConstraint",
                &[
                    ("partIndex", Value::Str("0".into())),
                    ("constrainOrientation", Value::Str("1".into())),
                    ("constrainTranslation", Value::Str("1".into())),
                    ("start", Value::Bool(true)),
                ],
            );
        },
        tick: Some(|s, target| pose_follow(s, target, "fb")),
    },
];

fn sorted(t: &[f64]) -> Vec<f64> {
    let mut t = t.to_vec();
    t.sort_by(f64::total_cmp);
    t
}

fn tick_all(sims: &mut [Sim], state: &State, target: &[Transform]) {
    for s in sims {
        if let Some(t) = state.tick {
            t(s, target);
        }
        s.step();
    }
}

#[test]
#[ignore = "measurement: ms per tick of N Delirium agents per tint drive state"]
fn tint_step_cost() {
    let Some(spawned) = fixture() else { return };
    let pelvis0 = f64::from(spawned.frames[0].translation.z);
    println!(
        "tint cost: setStiffness damping {SET_STIFFNESS_DAMPING}, push part {PUSH_PART}, {PUSH_NS} N s, {WARM_UP} warm-up + {TIMED} timed ticks"
    );
    for state in STATES {
        for n in [1usize, 2, 4, 8] {
            let mut sims: Vec<Sim> = (0..n).map(|i| Sim::new(spawned, 0.3 + 0.1 * i as f32, false)).collect();
            for s in &mut sims {
                (state.setup)(s);
            }
            // Every tick is timed from tick 0: ticks WARM_UP.. are the steady-state numbers, ticks ..FALLEN_AT the
            // "first 2 s" a tint lives in.
            let mut all: Vec<f64> = Vec::with_capacity(WARM_UP + TIMED);
            let mut fallen = None;
            for done in 1..=WARM_UP + TIMED {
                let t0 = Instant::now();
                tick_all(&mut sims, state, &spawned.frames);
                all.push(t0.elapsed().as_secs_f64() * 1000.0);
                if done == FALLEN_AT {
                    fallen = Some(sims.iter().filter(|s| s.agent.fallen).count());
                }
            }
            assert!(sims.iter().all(Sim::finite), "{} N={n}: non-finite frames", state.name);
            let t = sorted(&all[WARM_UP..]);
            let first = sorted(&all[..FALLEN_AT]);
            let mean = t.iter().sum::<f64>() / t.len() as f64;
            let median = t[t.len() / 2];
            let pelvis = sims.iter().map(|s| f64::from(s.pelvis_z())).sum::<f64>() / n as f64 / pelvis0 * 100.0;
            println!(
                "tint cost {} N={n}: {mean:.3} ms/tick mean, {:.3} min, {median:.3} median, {:.3} p99, {:.3} max, {:.3} ms/body; fallen {}/{n} at 2 s; pelvis {pelvis:.0} %; first 2 s {:.3} median, {:.3} p99 ms/tick",
                state.name,
                t[0],
                t[t.len() * 99 / 100],
                t[t.len() - 1],
                median / n as f64,
                fallen.unwrap_or(0),
                first[first.len() / 2],
                first[first.len() * 99 / 100],
            );
        }
    }
}

/// The parts a Delirium mask drives: its joints (`nmrs::Character::mask`), each moving its child part, `joint + 1`
/// (`nmrs::joints`: `child = i + 1`).
fn mask_parts(mask: &str) -> Vec<usize> {
    delirium::nmrs::Character::mask(mask).into_iter().map(|j| j + 1).collect()
}

/// Mean and max, degrees, over `parts` of the angle between each part's rotation relative to the pelvis (part 0) in
/// `frames` and in `target`, so drift and turning of the whole body do not count.
fn pose_error(frames: &[Transform], target: &[Transform], parts: &[usize]) -> (f64, f64) {
    let rel = |f: &[Transform], k: usize| f[0].rotation.inverse() * f[k].rotation;
    let e: Vec<f64> = parts.iter().map(|&k| f64::from(rel(frames, k).angle_between(rel(target, k)).to_degrees())).collect();
    (e.iter().sum::<f64>() / e.len() as f64, e.iter().copied().fold(0.0, f64::max))
}

#[test]
#[ignore = "measurement: per drive state, how far the simulated body is from the pose it is told to follow"]
fn tint_pose_error() {
    let Some(Layouts { bind, zero }) = layouts() else { return };
    let (ub, legs) = (mask_parts("ub"), mask_parts("lb"));
    let target = &zero.frames;
    let pelvis0 = f64::from(bind.frames[0].translation.z);
    let ((ub_mean, ub_max), (lb_mean, lb_max)) = (pose_error(&bind.frames, target, &ub), pose_error(&bind.frames, target, &legs));
    println!(
        "tint pose error header: target = zero pose, start = bind pose, N=1; ub parts {ub:?}, leg parts {legs:?}; bind to target (no motion): ub mean {ub_mean:.1} max {ub_max:.1}; legs mean {lb_mean:.1} max {lb_max:.1}"
    );
    for prefix in ["A ", "C ", "F ", "G "] {
        let state = STATES.iter().find(|s| s.name.starts_with(prefix)).expect("state in STATES");
        let mut sim = Sim::new(bind, 0.3, false);
        (state.setup)(&mut sim);
        for done in 1..=POSE_SAMPLES[POSE_SAMPLES.len() - 1] {
            tick_all(std::slice::from_mut(&mut sim), state, target);
            if POSE_SAMPLES.contains(&done) {
                assert!(sim.finite(), "{} t={done}: non-finite frames", state.name);
                let frames = sim.frames();
                let ((ub_mean, ub_max), (lb_mean, lb_max)) = (pose_error(&frames, target, &ub), pose_error(&frames, target, &legs));
                let pelvis = f64::from(sim.pelvis_z()) / pelvis0 * 100.0;
                println!(
                    "tint pose error {} t={:.1} s: ub mean {ub_mean:.1} max {ub_max:.1}; legs mean {lb_mean:.1} max {lb_max:.1}; pelvis {pelvis:.0} %",
                    state.name,
                    done as f64 * f64::from(DT),
                );
            }
        }
    }
}
