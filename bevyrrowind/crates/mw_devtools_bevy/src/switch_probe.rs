//! The flip delta probe (spec `superfargoth/docs/specs/feature-switches.md` "Cost beside the
//! switch", slice FS-C1a): on [`FeatureToggled`], the mean frame, main, render and GPU ms of
//! `diag.switch.window_s` before the flip against the same window after `diag.switch.settle_s`,
//! stored per switch in [`SwitchProbe`] ([`SwitchDelta`]: deltas, frame counts, `noise`, `moved`,
//! `superseded`). The statistics are [`crate::switch_probe_model`].
//!
//! Samples come from the F3 panel's per-frame read ([`PerfFrameFeed`]: the frame time, the gauges'
//! `main/busy/ms` and `render/busy/ms`, the GPU frame when GPU timers run). The probe records while
//! the Features panel is open (it sets [`PerfGaugeDemand`], so the gauges run with F3 hidden), while
//! F3 is shown, or while a flip waits for its after window; it keeps a ring of the last window of
//! frames for the next flip's before window. Otherwise (both panels closed, nothing waiting) the
//! gauges are off, nothing is recorded and the ring is empty: the cost is one system that reads the
//! toggle queue and returns. A flip then (a console `toggle` with both panels closed) has no before
//! window: its result has no delta and nothing waits. A frame without a feed sample (F3 warm-up, or
//! the counters owned by a benchmark) is the bare `Time<Real>` frame time.
//!
//! The camera is the streaming camera's `GlobalTransform` ([`ExteriorCamera`]); none, no `moved`.

use std::collections::BTreeMap;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::{Real, Time};
use bevy_transform::components::GlobalTransform;
use mw_game_core::exterior::ExteriorCamera;
use bevy_log::info;
use sf_tunables_bevy::sf_tunables::{Availability, Def, Lifetime, Source, Unit, def};
use sf_tunables_bevy::{
    AnyDef, FeatureSwitches, FeatureToggled, Id, PeelTunable, SetBy, SetTunable, TunableSystems, Tunables,
    TunablesAppExt,
};

use crate::features_panel::FeaturesPanel;
use crate::perf_panel::{PanelMode, PerfFrameFeed, PerfGaugeDemand, PerfPanel};
pub use crate::switch_probe_model::{AbAbort, AbCost, AbOutcome, Counts, SwitchDelta};
use crate::switch_probe_model::{AbAction, AbConfig, AbInput, AbRun, Outcome, Pose, Probe, ProbeConfig, Sample};

/// `diag.switch.window_s`: the before and after windows.
pub const WINDOW_S: Def<f32> = def!(float "diag.switch.window_s" = 2.0)
    .range(0.25, 10.0)
    .unit(Unit::Seconds)
    .help("Flip delta: seconds of frames averaged before a feature switch flips, and again after the settle time")
    .source(Source::Guess);

/// `diag.switch.settle_s`: frames left out after a flip (pipeline compiles, uploads, re-converging effects).
pub const SETTLE_S: Def<f32> = def!(float "diag.switch.settle_s" = 0.5)
    .range(0.0, 5.0)
    .unit(Unit::Seconds)
    .help("Flip delta: seconds after a flip left out before the after window opens")
    .source(Source::Guess);

/// `diag.switch.noise_ms`: a frame delta under this is noise.
pub const NOISE_MS: Def<f32> = def!(float "diag.switch.noise_ms" = 0.5)
    .range(0.0, 5.0)
    .unit(Unit::Millis)
    .help("Flip delta: a frame-time change smaller than this is marked noise (greyed)")
    .source(Source::Measured("docs/ledger: A/B noise floor ~0.5 ms"));

/// `diag.switch.still_m`: a camera move past this during the windows marks the result `moved`.
pub const STILL_M: Def<f32> = def!(float "diag.switch.still_m" = 0.5)
    .range(0.0, 100.0)
    .unit(Unit::Meters)
    .help("Flip delta: camera travel during the windows that marks the result moved")
    .source(Source::Guess);

/// `diag.switch.still_deg`: a camera turn past this during the windows marks the result `moved`.
pub const STILL_DEG: Def<f32> = def!(float "diag.switch.still_deg" = 2.0)
    .range(0.0, 180.0)
    .unit(Unit::Degrees)
    .help("Flip delta: camera turn during the windows that marks the result moved")
    .source(Source::Guess);

/// `diag.switch.ab_rounds`: off/on pairs of a quick A/B.
pub const AB_ROUNDS: Def<u32> = def!(count "diag.switch.ab_rounds" = 3)
    .range(1, 10)
    .unit(Unit::Count)
    .help("Quick A/B: pairs of flips (away and back) measured; the cost is the median over their flips")
    .source(Source::Guess);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[&WINDOW_S, &SETTLE_S, &NOISE_MS, &STILL_M, &STILL_DEG, &AB_ROUNDS];

/// The last measured flip of each switch (by switch name).
#[derive(Resource, Default, Debug)]
pub struct SwitchProbe {
    results: BTreeMap<&'static str, SwitchDelta>,
    generation: u64,
    measuring: bool,
    waiting: Vec<&'static str>,
    ready: bool,
}

impl SwitchProbe {
    /// The last result for switch `name`.
    pub fn get(&self, name: &str) -> Option<&SwitchDelta> {
        self.results.get(name)
    }

    /// Every switch with a result, by name.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &SwitchDelta)> {
        self.results.iter().map(|(k, v)| (*k, v))
    }

    /// Bumped on every stored result (a view redraws when it moves).
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// A flip waits for its after window.
    pub fn measuring(&self) -> bool {
        self.measuring
    }

    /// The waiting flip measures switch `name`.
    pub fn waiting(&self, name: &str) -> bool {
        self.waiting.contains(&name)
    }

    /// Recording, with a full before window: a flip now gets a whole one.
    pub fn ready(&self) -> bool {
        self.ready
    }

    fn store(&mut self, outcomes: Vec<Outcome>) {
        if outcomes.is_empty() {
            return;
        }
        self.generation += 1;
        self.results.extend(outcomes);
    }
}

#[derive(Clone, Copy)]
struct Ids {
    window: Id<f32>,
    settle: Id<f32>,
    noise: Id<f32>,
    still_m: Id<f32>,
    still_deg: Id<f32>,
}

#[derive(Resource)]
struct ProbeState {
    ids: Ids,
    probe: Probe,
    recording: bool,
}

/// Adds the probe ([`SwitchProbe`]); next to the Features panel ([`crate::live_ab::add`]).
pub struct SwitchProbePlugin;

impl Plugin for SwitchProbePlugin {
    fn build(&self, app: &mut App) {
        app.register_tunables(TUNABLES);
        let ids = Ids {
            window: app.add_tunable(&WINDOW_S).id(),
            settle: app.add_tunable(&SETTLE_S).id(),
            noise: app.add_tunable(&NOISE_MS).id(),
            still_m: app.add_tunable(&STILL_M).id(),
            still_deg: app.add_tunable(&STILL_DEG).id(),
        };
        let rounds = app.add_tunable(&AB_ROUNDS).id();
        app.init_resource::<SwitchProbe>()
            .insert_resource(ProbeState { ids, probe: Probe::default(), recording: false })
            .insert_resource(QuickAb { rounds, noise: ids.noise, run: None, results: BTreeMap::new(), generation: 0 })
            .add_message::<FeatureToggled>()
            .add_message::<SetTunable>()
            .add_message::<PeelTunable>()
            .add_systems(Update, drive_ab)
            // After the flips of this frame are reported; the frame sample (`First`) is the last
            // frame's, before them.
            .add_systems(PreUpdate, probe.after(TunableSystems::Apply));
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value, reason = "system parameters")]
fn probe(
    mut toggles: MessageReader<FeatureToggled>,
    features: Option<Res<FeaturesPanel>>,
    perf: Option<Res<PerfPanel>>,
    feed: Option<ResMut<PerfFrameFeed>>,
    demand: Option<ResMut<PerfGaugeDemand>>,
    real: Res<Time<Real>>,
    camera: Query<&GlobalTransform, With<ExteriorCamera>>,
    tunables: Res<Tunables>,
    mut state: ResMut<ProbeState>,
    mut results: ResMut<SwitchProbe>,
) {
    let state = &mut *state;
    let open = features.is_some_and(|f| f.open);
    let shown = perf.is_some_and(|p| p.mode != PanelMode::Off);
    let recording = open || shown || state.probe.is_pending();
    let ids = state.ids;
    let cfg = ProbeConfig {
        window_s: tunables.read(ids.window),
        settle_s: tunables.read(ids.settle),
        noise_ms: tunables.read(ids.noise),
        still_m: tunables.read(ids.still_m),
        still_deg: tunables.read(ids.still_deg),
    };
    let fresh = feed.and_then(|mut f| f.0.take());
    if recording {
        #[allow(clippy::cast_possible_truncation, reason = "ms to f32")]
        let mut sample = Sample { frame_ms: (real.delta_secs_f64() * 1000.0) as f32, ..Sample::default() };
        if let Some(f) = fresh {
            sample.frame_ms = f.frame_ms;
            sample.main_ms = f.main_ms;
            sample.render_ms = f.render_ms;
            sample.gpu_ms = f.gpu_ms;
        }
        sample.pose = camera.iter().next().map(|g| {
            let (_, rotation, translation) = g.to_scale_rotation_translation();
            Pose { translation, rotation }
        });
        let out = state.probe.push(sample, &cfg);
        if !out.is_empty() {
            results.store(out);
        }
    } else if state.recording {
        state.probe.clear_ring();
    }
    state.recording = recording;
    let mut flipped: Vec<(&'static str, bool)> = Vec::new();
    for t in toggles.read() {
        match flipped.iter_mut().find(|(name, _)| *name == t.name) {
            Some(seen) => seen.1 = t.running,
            None => flipped.push((t.name, t.running)),
        }
    }
    if !flipped.is_empty() {
        let out = state.probe.flip(&flipped, &cfg);
        if !out.is_empty() {
            results.store(out);
        }
    }
    let measuring = state.probe.is_pending();
    if results.measuring != measuring {
        results.measuring = measuring;
    }
    if !results.waiting.iter().copied().eq(state.probe.waiting()) {
        results.waiting = state.probe.waiting().collect();
    }
    let ready = recording && state.probe.ring_full(&cfg);
    if results.ready != ready {
        results.ready = ready;
    }
    if let Some(mut demand) = demand {
        demand.set_if_neq(PerfGaugeDemand(open || measuring));
    }
}

// ---- Quick A/B (slice FS-C1b) ----

/// How the switch comes back after a quick A/B: its value came from below the console layer
/// (peel the console layer) or from it (write it back).
#[derive(Clone, Debug, PartialEq, Eq)]
enum Restore {
    Peel,
    Set(String),
}

#[derive(Debug)]
struct ActiveAb {
    name: &'static str,
    run: AbRun,
    restore: Restore,
    /// A flip's refusal, set by the queued flip; read by the next poll.
    refused: Option<String>,
}

/// The quick A/B (spec "Cost beside the switch", slice FS-C1b): one switch at a time flipped away
/// from its state and back `diag.switch.ab_rounds` times at the console layer, each flip measured
/// by [`SwitchProbe`]; the result is the median cost of the feature ([`AbCost`], positive = it
/// costs that much). Rules: [`AbRun`]. Started by [`start_quick_ab`] (the Features panel's `A/B`
/// button); needs the panel open (closing it aborts). It ends with the switch as it was: its
/// value back, and the console layer peeled when the value came from below it.
#[derive(Resource, Debug)]
pub struct QuickAb {
    rounds: Id<u32>,
    noise: Id<f32>,
    run: Option<ActiveAb>,
    results: BTreeMap<&'static str, AbOutcome>,
    generation: u64,
}

impl QuickAb {
    /// The switch being A/B'd and its progress (flips measured, flips planned).
    pub fn running(&self) -> Option<(&'static str, (u32, u32))> {
        self.run.as_ref().map(|a| (a.name, a.run.progress()))
    }

    /// Switch `name`'s last finished A/B.
    pub fn get(&self, name: &str) -> Option<&AbOutcome> {
        self.results.get(name)
    }

    /// Bumped on a start, a measured flip and an end (a view redraws when it moves).
    pub fn generation(&self) -> u64 {
        self.generation
    }

    fn finish(&mut self, outcome: AbOutcome) -> Option<(&'static str, Restore)> {
        let active = self.run.take()?;
        match &outcome {
            AbOutcome::Done(c) => info!(
                "[features] A/B {}: cost {:.2} ms (median over {} round(s), {:.2}..{:.2})",
                active.name, c.frame_ms, c.rounds, c.min_ms, c.max_ms
            ),
            AbOutcome::Aborted(why) => info!("[features] A/B {}: stopped: {}", active.name, why.text()),
        }
        self.results.insert(active.name, outcome);
        self.generation += 1;
        Some((active.name, active.restore))
    }
}

/// Starts a quick A/B of switch `name`: the answer line, or why not (one runs already, not a live
/// switch, not built, blocked).
pub fn start_quick_ab(world: &mut World, name: &'static str) -> Result<String, String> {
    let Some(ab) = world.get_resource::<QuickAb>() else {
        return Err("no switch probe this run".to_owned());
    };
    if let Some((busy, _)) = ab.running() {
        return Err(format!("{name}: the A/B of {busy} still runs"));
    }
    let (rounds, noise) = (ab.rounds, ab.noise);
    let (Some(switches), Some(tunables)) = (world.get_resource::<FeatureSwitches>(), world.get_resource::<Tunables>())
    else {
        return Err(format!("{name}: no feature switches"));
    };
    let (Some(switch), Some(id), Some(state)) =
        (switches.get(name), switches.tunable(name), switches.state(tunables, name))
    else {
        return Err(format!("{name}: no such feature switch"));
    };
    let var = tunables.var(id);
    if switch.startup_hint().is_some() || matches!(var.meta().lifetime, Lifetime::Restart | Lifetime::Bake) {
        return Err(format!("{name}: not live (applies on a restart or a bake): no A/B"));
    }
    match state.availability {
        Availability::Available => {}
        Availability::NotBuilt { reason } => return Err(format!("{name}: not built this run ({reason}): no A/B")),
        Availability::Blocked { by } => return Err(format!("{name}: blocked by {by}: no A/B")),
    }
    let restore = if state.layer == SetBy::Console { Restore::Set(var.value().to_string()) } else { Restore::Peel };
    let cfg = AbConfig { rounds: tunables.read(rounds), noise_ms: tunables.read(noise) };
    let running = state.running();
    let mut ab = world.resource_mut::<QuickAb>();
    ab.run = Some(ActiveAb { name, run: AbRun::new(running, cfg), restore, refused: None });
    ab.generation += 1;
    Ok(format!("{name}: A/B started ({} round(s) of off/on)", cfg.rounds.max(1)))
}

/// Polls the running A/B once a frame; flips, finishes and restores through queued commands.
#[allow(clippy::needless_pass_by_value, reason = "system parameters")]
fn drive_ab(
    mut commands: Commands,
    mut toggles: MessageReader<FeatureToggled>,
    panel: Option<Res<FeaturesPanel>>,
    probe: Res<SwitchProbe>,
    mut ab: ResMut<QuickAb>,
) {
    let flips: Vec<(&'static str, bool)> = toggles.read().map(|t| (t.name, t.running)).collect();
    let ab = &mut *ab;
    let Some(active) = ab.run.as_mut() else { return };
    let name = active.name;
    let action = if let Some(why) = active.refused.take() {
        AbAction::Finish(AbOutcome::Aborted(AbAbort::Refused(why)))
    } else {
        let own = flips.iter().rev().find(|(n, _)| *n == name).map(|&(_, r)| r);
        let input = AbInput {
            open: panel.is_some_and(|p| p.open),
            ready: probe.ready() && !probe.measuring(),
            own,
            others: own.is_none() && flips.iter().any(|(n, _)| *n != name),
            generation: probe.generation(),
            waiting: probe.waiting(name),
            result: probe.get(name).copied(),
        };
        let before = active.run.progress();
        let action = active.run.poll(&input);
        if active.run.progress() != before {
            ab.generation += 1;
        }
        action
    };
    match action {
        AbAction::Wait => {}
        AbAction::Flip(on) => commands.queue(move |world: &mut World| flip(world, name, on)),
        AbAction::Finish(outcome) => {
            if let Some((name, restore)) = ab.finish(outcome) {
                commands.queue(move |world: &mut World| match restore {
                    Restore::Peel => {
                        world.write_message(PeelTunable { name: name.to_owned(), layer: Some(SetBy::Console) });
                    }
                    Restore::Set(value) => {
                        world.write_message(SetTunable::new(name, value, SetBy::Console).note("A/B restore"));
                    }
                });
            }
        }
    }
}

/// One A/B flip, as the console's `toggle` (the Features panel's path).
fn flip(world: &mut World, name: &'static str, on: bool) {
    let answer = if world.contains_resource::<Tunables>() {
        world.resource_scope(|world, tunables: Mut<Tunables>| {
            sf_console_bevy::features::set_feature(world, &tunables.0, name, Some(on))
        })
    } else {
        None
    };
    let refused = match answer {
        Some(Ok(_)) => return,
        Some(Err(line)) => line,
        None => format!("{name}: no such feature switch"),
    };
    if let Some(active) = world.resource_mut::<QuickAb>().run.as_mut() {
        active.refused = Some(refused);
    }
}

#[cfg(test)]
#[path = "switch_probe_tests.rs"]
mod tests;
