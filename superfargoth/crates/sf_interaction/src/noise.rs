//! Interaction noise core (interaction-world-rules.md "Noise", IN18a): how far an impact is heard
//! and how one tick's noises are thinned before the shell writes them. Host only, pure: the same
//! noises in any order give the same output.
//!
//! - **Loudness.** [`loudness_m`]`(reach, v)` = `reach x clamp(v / interact.noise.ref_speed_mps, 0,
//!   interact.noise.speed_mul_max)`, none under `interact.noise.min_speed_mps` (reach ∝ sound
//!   pressure ∝ impact speed, inverse-distance law).
//! - **Merge window** ([`NoiseMerge`]). One object's noises in one tick merge into the loudest. The
//!   first noise an object makes opens a window of `interact.noise.merge_s` (at least one tick): inside
//!   it a later noise is heard only if it is louder than every noise heard from that object in the
//!   window (a rattle of decaying bounces is one noise; a louder blow right after a tap is not lost).
//!   Listeners never miss the loudest noise an object made in a window; nothing waits for the window.
//! - **Cap.** At most `interact.noise.per_tick_max` noises a tick, loudest first; a noise the cap cut
//!   opens no window. Ties break on a total order of every field, so arrival order never matters.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::claims::{Tick, UserKey};
use crate::def::TagId;
use crate::params::InteractParams;

/// How far an impact at normal speed `speed_mps` on a material of `reach_m`
/// (`MaterialDef::impact_reach_m`, the reach at `interact.noise.ref_speed_mps`) is heard, m; `None`
/// under `interact.noise.min_speed_mps`, for a silent material or a non-finite input.
#[must_use]
pub fn loudness_m(reach_m: f32, speed_mps: f32, params: &InteractParams) -> Option<f32> {
    if !reach_m.is_finite() || !speed_mps.is_finite() || speed_mps < params.noise_min_speed_mps {
        return None;
    }
    // `max`/`min`, not `clamp`: a NaN tunable gives 0 (silent), never a panic.
    let mul = (speed_mps / params.noise_ref_speed_mps).max(0.0).min(params.noise_speed_mul_max);
    let loud = reach_m * mul;
    // NaN fails the test (silent); an infinite tunable saturates.
    (loud > 0.0).then(|| loud.min(f32::MAX))
}

/// Which row of the spec's noise table a noise comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NoiseCase {
    Impact,
    ThrownLanding,
    Drop,
    Break,
    Door,
    Bash,
    LockPick,
    Fire,
}

/// One noise before the shell adds its tick and space (`InteractionNoise`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Noise {
    /// The shell's stable key of the object that made it (placement or body): the merge key.
    pub source: u64,
    /// Where it is heard from, world metres.
    pub at: Vec3,
    /// The noise kind (the material's `noise`, or the case's tag).
    pub kind: TagId,
    /// How far it is heard, m.
    pub loudness_m: f32,
    /// Who made it (an actor never hears its own noise).
    pub maker: Option<UserKey>,
    /// Listeners learn who made it (a door, a pick), not only where.
    pub reveals_maker: bool,
    pub case: NoiseCase,
}

/// The total order noises are kept in: loudest first, then every other field.
fn order(a: &Noise, b: &Noise) -> Ordering {
    b.loudness_m
        .total_cmp(&a.loudness_m)
        .then(a.source.cmp(&b.source))
        .then(a.case.cmp(&b.case))
        .then(a.kind.cmp(&b.kind))
        .then(a.maker.cmp(&b.maker))
        .then(a.reveals_maker.cmp(&b.reveals_maker))
        .then(a.at.x.total_cmp(&b.at.x))
        .then(a.at.y.total_cmp(&b.at.y))
        .then(a.at.z.total_cmp(&b.at.z))
}

/// One object's open merge window.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Window {
    opened: Tick,
    /// The loudest noise heard from the object in the window, m.
    loudest_m: f32,
}

/// The merge windows by object (ordered, so every replay iterates alike). Cleared on a world change.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NoiseMerge {
    windows: BTreeMap<u64, Window>,
}

impl NoiseMerge {
    /// The noises of tick `tick` that are heard, loudest first (module docs: merge window, cap).
    /// Noises with a loudness that is not a positive finite number are dropped.
    pub fn step(&mut self, tick: Tick, noises: impl IntoIterator<Item = Noise>, params: &InteractParams) -> Vec<Noise> {
        let window_ticks = params.ticks(params.noise_merge_s);
        self.windows.retain(|_, w| tick >= w.opened && tick < w.opened.plus(window_ticks));

        let mut heard: Vec<Noise> =
            noises.into_iter().filter(|n| n.loudness_m.is_finite() && n.loudness_m > 0.0).collect();
        // One per object a tick: its loudest (the total order breaks ties).
        heard.sort_by(|a, b| a.source.cmp(&b.source).then_with(|| order(a, b)));
        heard.dedup_by_key(|n| n.source);
        // Inside an open window only a louder noise is heard.
        heard.retain(|n| self.windows.get(&n.source).is_none_or(|w| n.loudness_m > w.loudest_m));
        heard.sort_by(order);
        heard.truncate(usize::from(params.noise_per_tick_max));

        for n in &heard {
            self.windows
                .entry(n.source)
                .and_modify(|w| w.loudest_m = n.loudness_m)
                .or_insert(Window { opened: tick, loudest_m: n.loudness_m });
        }
        heard
    }

    /// Objects with an open window.
    #[must_use]
    pub fn open_windows(&self) -> usize {
        self.windows.len()
    }

    /// Forget every window (a world change: the noises belonged to the old world).
    pub fn clear(&mut self) {
        self.windows.clear();
    }
}
