//! In-process A/B of a benchmark run (`docs/reviews/bench-method-2026-11-23.md` §3, slice 1): `--ab
//! NAME=A,B` (or the scenario's `ab` field) flips one tunable A B A B ... every `phase_s` seconds of
//! the measurement, at the `CommandLine` layer through the normal tunable store ([`SetTunable`]).
//! The schedule, the per-phase stats and `ab.md` are [`sf_diag::flip_ab`]; `sf_diag_bevy` writes
//! the files at the end. The warm-up runs with A.
//!
//! Live only: a tunable whose lifetime is `restart` or `bake`, a startup-only feature switch, or a
//! switch not built this run is refused before the first frame (exit 2, as a refused `--set`);
//! those take one run per arm (`perf suite --arm`). Standing scenarios only: a path scenario is
//! refused (a phase would have to span whole laps of the path to compare the same views).
//!
//! The flip itself mirrors the switch probe's (`crate::switch_probe` quick A/B: one value away and
//! back at a layer above the defaults), but for any live tunable, on a timed schedule, no panel.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::{error, info};
use sf_diag_bevy::sf_diag::flip_ab::{AbDefaults, AbSchedule, AbSpec, Arm};
use sf_diag_bevy::sf_diag::{Measure, Phase, Scenario};
use sf_diag_bevy::{DiagScenario, DiagSystems};
use sf_tunables_bevy::overrides::parse_set;
use sf_tunables_bevy::sf_tunables::{self, AnyDef, Availability, Def, Lifetime, SetBy, Source, Unit, def};
use sf_tunables_bevy::{FeatureSwitches, SetTunable, Tunables, TunablesAppExt, TunablesPlugin};

/// `bench.ab.phase_s`: seconds per A or B phase.
pub const PHASE_S: Def<f32> = def!(float "bench.ab.phase_s" = 5.0)
    .range(0.5, 60.0)
    .unit(Unit::Seconds)
    .help("In-process A/B (--ab): seconds of measurement per A or B phase")
    .source(Source::Guess)
    .lifetime(Lifetime::Restart);

/// `bench.ab.phases`: measured phases per value (one more A+B cycle runs first and is discarded).
pub const PHASES: Def<u32> = def!(count "bench.ab.phases" = 4)
    .range(1, 20)
    .unit(Unit::Count)
    .help("In-process A/B (--ab): measured phases per value; one more A+B cycle runs first and is discarded")
    .source(Source::Guess)
    .lifetime(Lifetime::Restart);

/// `bench.ab.settle_s`: seconds left out at the start of every phase (the flip's compiles, uploads).
pub const SETTLE_S: Def<f32> = def!(float "bench.ab.settle_s" = 0.5)
    .range(0.0, 10.0)
    .unit(Unit::Seconds)
    .help("In-process A/B (--ab): seconds left out at the start of every phase, after the flip")
    .source(Source::Guess)
    .lifetime(Lifetime::Restart);

/// `bench.ab.noise_ms`: a |Δ| under this is noise in `ab.md`.
pub const NOISE_MS: Def<f32> = def!(float "bench.ab.noise_ms" = 0.5)
    .range(0.0, 5.0)
    .unit(Unit::Millis)
    .help("In-process A/B (--ab): a paired frame-time delta smaller than this is noise")
    .source(Source::Measured("docs/ledger: A/B noise floor ~0.5 ms"))
    .lifetime(Lifetime::Restart);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[&PHASE_S, &PHASES, &SETTLE_S, &NOISE_MS];

/// The run's A/B spec: `--ab` (replaces the scenario's `ab`), else the scenario's; then the
/// `--ab-phase-s` / `--ab-phases` / `--ab-settle-s` flags on top. `None`: no A/B.
pub fn spec(scenario: Option<&AbSpec>, ab: Option<&str>, phase_len_s: Option<f64>, per_value: Option<u32>, settle_s: Option<f64>) -> Result<Option<AbSpec>, String> {
    let spec = match ab {
        Some(text) => Some(AbSpec::parse(text)?),
        None => scenario.cloned(),
    };
    let Some(mut spec) = spec else {
        if phase_len_s.is_some() || per_value.is_some() || settle_s.is_some() {
            return Err("--ab-phase-s / --ab-phases / --ab-settle-s need --ab NAME=A,B or a scenario `ab`".to_owned());
        }
        return Ok(None);
    };
    spec.phase_s = phase_len_s.or(spec.phase_s);
    spec.phases = per_value.or(spec.phases);
    spec.settle_s = settle_s.or(spec.settle_s);
    Ok(Some(spec))
}

/// The defaults of what `spec` leaves unset: this module's tunables with the run's `--set` values
/// (checked like the game checks them, in a scratch store), then the schedule.
pub fn schedule(spec: &AbSpec, sets: &[String]) -> Result<AbSchedule, String> {
    let mut store = sf_tunables::Tunables::default();
    store.register_all(TUNABLES).map_err(|e| e.to_string())?;
    for arg in sets {
        let o = parse_set(arg).map_err(|e| e.to_string())?;
        if let Some(id) = store.id(&o.name) {
            store.set_text_by(id, &o.value, SetBy::CommandLine).map_err(|e| format!("--set {arg}: {e}"))?;
        }
    }
    let read_f = |def: &Def<f32>| store.id_of(def).map(|id| f64::from(store.read(id))).map_err(|e| e.to_string());
    let phases = store.id_of(&PHASES).map(|id| store.read(id)).map_err(|e| e.to_string())?;
    let defaults = AbDefaults { phase_s: read_f(&PHASE_S)?, phases, settle_s: read_f(&SETTLE_S)?, noise_ms: read_f(&NOISE_MS)? };
    AbSchedule::new(spec, defaults)
}

/// Fits `scenario` to the A/B: the measured duration becomes the whole schedule. Refuses a path
/// scenario and `--bench-seconds` (both would cut the schedule).
pub fn fit_scenario(scenario: &mut Scenario, spec: AbSpec, schedule: &AbSchedule, bench_seconds: bool) -> Result<(), String> {
    if matches!(scenario.measure, Measure::Path { .. }) {
        return Err(format!(
            "--ab: `{}` is a path scenario; path scenarios are refused for now (a phase would have to span whole laps of the path): use a standing scenario",
            scenario.name
        ));
    }
    if bench_seconds {
        return Err("--ab sets the measured duration (the whole schedule): drop --bench-seconds, or set --ab-phase-s / --ab-phases".to_owned());
    }
    scenario.measure = Measure::Duration { seconds: schedule.total_s() };
    scenario.ab = Some(spec);
    Ok(())
}

/// Why tunable `name` can't be A/B'd in one launch, or `None` when it can: a `restart` or `bake`
/// lifetime, a startup-only feature switch (`startup_hint`), a switch not built this run
/// (`not_built`). `live`, `respawn` (new spawns: a blade field toggle respawns its fields) and
/// `recompile` (the first flip compiles: the discarded cycle) take effect in a running game.
pub fn live_refusal(name: &str, lifetime: Lifetime, startup_hint: Option<&str>, not_built: Option<&str>) -> Option<String> {
    if matches!(lifetime, Lifetime::Restart | Lifetime::Bake) {
        return Some(format!(
            "--ab {name}: not live: its lifetime is `{}` ({}); A/B it with one run per value (`perf suite --arm`)",
            lifetime.name(),
            lifetime.notice().unwrap_or_default()
        ));
    }
    if let Some(hint) = startup_hint {
        return Some(format!("--ab {name}: its feature switch is startup-only ({hint}); A/B it with one run per value (`perf suite --arm`)"));
    }
    not_built.map(|reason| format!("--ab {name}: its feature switch is not built this run ({reason}): flipping it changes nothing"))
}

/// The flip schedule and the value last asked for.
#[derive(Resource, Debug)]
struct FlipAb {
    schedule: AbSchedule,
    applied: Arm,
}

/// Flips the A/B's tunable along its schedule. Add after the run's tunable overrides and the part
/// switches (its `finish` checks the tunable once every plugin registered and marked).
pub struct FlipAbPlugin(pub AbSchedule);

impl Plugin for FlipAbPlugin {
    fn build(&self, app: &mut App) {
        // The store's request messages and their `PreUpdate` apply: a benchmark is not interactive,
        // so the console (which brings them in a played run) is not there, and without them
        // `drive`'s `SetTunable` writer fails validation every frame and no flip happens (first
        // real run, 2026-11-23: A vs A).
        if !app.is_plugin_added::<TunablesPlugin>() {
            app.add_plugins(TunablesPlugin);
        }
        app.register_tunables(TUNABLES)
            .insert_resource(FlipAb { schedule: self.0.clone(), applied: Arm::A })
            .add_systems(First, drive.after(DiagSystems::Boundary));
    }

    fn finish(&self, app: &mut App) {
        if let Err(why) = start(app.world_mut(), &self.0) {
            error!("{why}");
            eprintln!("error: {why}");
            std::process::exit(2);
        }
        info!("perf ab: {}", self.0.describe());
    }
}

/// Checks the tunable and both values, then sets A for the warm-up.
fn start(world: &mut World, ab: &AbSchedule) -> Result<(), String> {
    let name = ab.tunable.as_str();
    let switch = world.get_resource::<FeatureSwitches>().and_then(|switches| {
        let tunables = world.get_resource::<Tunables>()?;
        let id = tunables.id(name)?;
        let switch = switches.iter().find(|s| switches.tunable(s.name()) == Some(id))?;
        let not_built = match switches.state(tunables, switch.name()).map(|s| s.availability) {
            Some(Availability::NotBuilt { reason }) => Some(reason.to_string()),
            _ => None,
        };
        Some((switch.startup_hint(), not_built))
    });
    let (hint, not_built) = switch.unwrap_or_default();
    let mut tunables = world.get_resource_mut::<Tunables>().ok_or("--ab: no tunable store this run")?;
    let id = tunables.id(name).ok_or_else(|| format!("--ab {name}: no such tunable (`bevyrrowind tunables --md` lists them)"))?;
    let var = tunables.var(id);
    if let Some(why) = live_refusal(name, var.meta().lifetime, hint, not_built.as_deref()) {
        return Err(why);
    }
    let kind = var.kind().clone();
    let [a, b] = [&ab.a, &ab.b].map(|v| kind.parse(v).map_err(|e| format!("--ab {name}={v}: {e}")));
    let (a, _) = (a?, b?);
    tunables.set_noted(id, a, SetBy::CommandLine, Some("perf ab: A".to_owned())).map_err(|e| format!("--ab {name}: {e}"))?;
    let set_by = tunables.var(id).set_by();
    if set_by != SetBy::CommandLine {
        return Err(format!("--ab {name}: set by `{}`, above the command line: the flip would not show", set_by.describe()));
    }
    Ok(())
}

/// `First`, after the scenario advanced: asks for the value of the frame that starts now (A during
/// the warm-up), on a change only; it applies in this frame's `PreUpdate`.
#[allow(clippy::needless_pass_by_value, reason = "system parameters")]
fn drive(scenario: Option<Res<DiagScenario>>, mut flip: ResMut<FlipAb>, mut sets: MessageWriter<SetTunable>) {
    let Some(scenario) = scenario else { return };
    let t = match scenario.phase() {
        Phase::Warmup => 0.0,
        Phase::Measure => scenario.measured_s(),
        Phase::Done => return,
    };
    let arm = flip.schedule.arm_at(t);
    if arm == flip.applied {
        return;
    }
    flip.applied = arm;
    let s = &flip.schedule;
    let phase = s.phase_at(t);
    info!("perf ab: phase {phase} at {t:.2} s: {}={} ({})", s.tunable, s.value(arm), arm.label());
    sets.write(SetTunable::new(s.tunable.clone(), s.value(arm), SetBy::CommandLine).note(format!("perf ab: phase {phase} {}", arm.label())));
}

#[cfg(test)]
#[path = "flip_ab_tests.rs"]
mod tests;
