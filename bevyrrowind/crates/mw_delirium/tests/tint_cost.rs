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
use mw_delirium::sim::{Sim, SpawnPose, Spawned, spawn};
use mw_delirium::{Options, character_from_skeleton};
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

/// The `setStiffness` message's own damping default (`Stiffness::default`, delirium `msg.rs:516`), sent explicitly so
/// the printed value is the one used.
const SET_STIFFNESS_DAMPING: f32 = 1.0;
const WARM_UP: usize = 60;
const TIMED: usize = 600;
/// Ticks after setup (warm-up included) at which `fallen` is counted: 2 s at 60 Hz.
const FALLEN_AT: usize = 120;

fn fixture() -> Option<&'static Spawned> {
    static F: OnceLock<Option<Spawned>> = OnceLock::new();
    F.get_or_init(|| {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return None;
        };
        let vfs: Vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
        let bytes = vfs.read(&VfsPath::new("meshes/base_anim.nif").unwrap()).unwrap();
        let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
        let body = character_from_skeleton(&skeleton, &Options::default()).unwrap();
        Some(spawn(&body, SpawnPose::Bind, None))
    })
    .as_ref()
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
        tick: Some(|s, target| {
            s.agent.incoming.clear();
            s.agent.incoming.extend_from_slice(target);
            s.send("activePose", &[("mask", Value::Str("fb".into()))]);
        }),
    },
];

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
    println!("tint cost: setStiffness damping {SET_STIFFNESS_DAMPING}, {WARM_UP} warm-up + {TIMED} timed ticks");
    for state in STATES {
        for n in [1usize, 4, 8] {
            let mut sims: Vec<Sim> = (0..n).map(|i| Sim::new(spawned, 0.3 + 0.1 * i as f32, false)).collect();
            for s in &mut sims {
                (state.setup)(s);
            }
            let mut fallen = None;
            let mut done = 0;
            for _ in 0..WARM_UP {
                tick_all(&mut sims, state, &spawned.frames);
                done += 1;
                if done == FALLEN_AT {
                    fallen = Some(sims.iter().filter(|s| s.agent.fallen).count());
                }
            }
            let mut t: Vec<f64> = Vec::with_capacity(TIMED);
            for _ in 0..TIMED {
                let t0 = Instant::now();
                tick_all(&mut sims, state, &spawned.frames);
                t.push(t0.elapsed().as_secs_f64() * 1000.0);
                done += 1;
                if done == FALLEN_AT {
                    fallen = Some(sims.iter().filter(|s| s.agent.fallen).count());
                }
            }
            assert!(sims.iter().all(Sim::finite), "{} N={n}: non-finite frames", state.name);
            t.sort_by(f64::total_cmp);
            let mean = t.iter().sum::<f64>() / t.len() as f64;
            let median = t[t.len() / 2];
            let pelvis = sims.iter().map(|s| f64::from(s.pelvis_z())).sum::<f64>() / n as f64 / pelvis0 * 100.0;
            println!(
                "tint cost {} N={n}: {mean:.3} ms/tick mean, {:.3} min, {median:.3} median, {:.3} p99, {:.3} max, {:.3} ms/body; fallen {}/{n} at 2 s; pelvis {pelvis:.0} %",
                state.name,
                t[0],
                t[t.len() * 99 / 100],
                t[t.len() - 1],
                median / n as f64,
                fallen.unwrap_or(0),
            );
        }
    }
}
