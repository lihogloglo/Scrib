//! Bevy plugin for [`sf_diag`]. Spec: `docs/specs/diag.md`.
//!
//! - [`DiagCounters`]: the counter API for every plugin (always present,
//!   reporting takes `Res`, never `ResMut`, so it adds no system conflicts).
//! - With a scenario: at each frame boundary (`First`, after
//!   `TimeSystems`) the frame time from `Time<Real>` and every new Bevy
//!   diagnostic (`DiagnosticsStore`) are recorded, the scenario advances,
//!   and the fly-through pose is applied to [`FlyThroughCamera`] entities.
//!   At the end the JSON and CSV reports are written, compared against the
//!   baseline, and the app optionally exits (code 1 on failure).

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy_app::{App, AppExit, First, Plugin};
use bevy_diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy_ecs::prelude::*;
use bevy_platform::collections::HashMap;
use bevy_platform::time::Instant;
use bevy_time::{Real, Time, TimeSystems};
use bevy_transform::components::Transform;

pub use sf_diag;
use sf_diag::{Counters, DiagError, Phase, Pose, Recorder, Report, Scenario, ScenarioRun};

mod churn;

#[cfg(feature = "assets")]
pub use churn::AssetChurnPlugin;
#[cfg(feature = "render")]
pub use churn::PipelineChurnPlugin;
pub use churn::{ChurnPlugin, ENTITIES, ENTITIES_DELTA, short_type_name};

mod phases;
pub use phases::FramePhasePlugin;

#[cfg(test)]
mod tests;

/// Which Bevy diagnostics are recorded as gauge counters (under their own
/// path, e.g. `render/opaque_pass/elapsed_gpu`). Bevy's own frame time,
/// fps and frame count are always skipped: the frame column covers them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DiagnosticFilter {
    #[default]
    All,
    None,
    /// Only paths starting with one of these.
    Prefixes(Vec<String>),
}

impl DiagnosticFilter {
    fn accepts(&self, path: &str) -> bool {
        match self {
            Self::All => true,
            Self::None => false,
            Self::Prefixes(prefixes) => prefixes.iter().any(|p| path.starts_with(p.as_str())),
        }
    }
}

/// Where reports go and what happens at the end of a scenario.
#[derive(Resource, Clone, Debug)]
#[allow(clippy::struct_excessive_bools, reason = "independent settings")]
pub struct DiagSettings {
    /// `<scenario>.json` and `<scenario>.csv` are written here.
    pub output_dir: PathBuf,
    /// Baseline report to compare against. Missing file: a warning, no
    /// comparison.
    pub baseline: Option<PathBuf>,
    /// Write this run's report as the new baseline instead of comparing.
    pub update_baseline: bool,
    /// Send `AppExit` when the scenario ends: success, or code 1 if a gate
    /// failed, something regressed, or a report couldn't be written.
    pub exit_on_finish: bool,
    /// Write the fly-through pose into `Transform` of [`FlyThroughCamera`]
    /// entities. Apps with their own coordinate system (`big_space`) turn
    /// this off and convert [`FlyThroughPose`] themselves.
    pub apply_transform: bool,
    pub bevy_diagnostics: DiagnosticFilter,
    /// Copied into the report: machine, build, quality preset, resolution.
    /// Apps may add to it until the scenario ends (e.g. the GPU adapter).
    pub meta: BTreeMap<String, String>,
    /// Frames longer than this are hitches (default: 2.5 × the median
    /// frame, at least 25 ms; `sf_diag::default_hitch_ms`).
    pub hitch_ms: Option<f64>,
    /// Also write `<scenario>.md`, the human-readable report.
    pub markdown: bool,
}

impl Default for DiagSettings {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("diag"),
            baseline: None,
            update_baseline: false,
            exit_on_finish: false,
            apply_transform: true,
            bevy_diagnostics: DiagnosticFilter::All,
            meta: BTreeMap::new(),
            hitch_ms: None,
            markdown: true,
        }
    }
}

/// The shared counter registry. Systems take `Res<DiagCounters>`; async
/// tasks clone the inner `Arc`.
#[derive(Resource, Clone, Debug, Default)]
pub struct DiagCounters(pub Arc<Counters>);

impl Deref for DiagCounters {
    type Target = Counters;
    fn deref(&self) -> &Counters {
        &self.0
    }
}

/// The running scenario and its recording.
#[derive(Resource, Debug)]
pub struct DiagScenario {
    run: ScenarioRun,
    recorder: Recorder,
    /// Warm-up frames (loading, settling), for the hitch table.
    warmup: Recorder,
    /// Wall clock (Unix seconds) when measurement began.
    wall_start: Option<f64>,
    /// Keeps the warm-up going while true (e.g. until streaming is idle).
    pub hold: bool,
}

impl DiagScenario {
    pub fn run(&self) -> &ScenarioRun {
        &self.run
    }

    pub fn phase(&self) -> Phase {
        self.run.phase()
    }

    pub fn recorder(&self) -> &Recorder {
        &self.recorder
    }

    /// The warm-up frames recorded so far.
    pub fn warmup_recorder(&self) -> &Recorder {
        &self.warmup
    }
}

/// Camera pose for this frame, from the scenario's path (f64 world units).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FlyThroughPose(pub Option<Pose>);

/// Marks the camera a fly-through drives.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FlyThroughCamera;

/// The finished scenario's report, inserted when measurement ends.
#[derive(Resource, Clone, Debug)]
pub struct DiagReport {
    pub report: Report,
    pub json: Option<PathBuf>,
    pub csv: Option<PathBuf>,
    /// The Markdown report, when [`DiagSettings::markdown`] is on.
    pub markdown: Option<PathBuf>,
    /// Report or baseline I/O problems. Any error fails the run.
    pub errors: Vec<String>,
}

impl DiagReport {
    /// Gates passed, nothing regressed, and the files were written.
    pub fn passed(&self) -> bool {
        self.report.passed && self.errors.is_empty()
    }
}

/// System sets in `First`, after `TimeSystems`, in this order.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagSystems {
    /// Records the frame that just ended and advances the scenario.
    Boundary,
    /// Moves [`FlyThroughCamera`] entities to the new pose.
    ApplyPose,
}

/// Adds the counter API and, with a scenario, runs it.
pub struct SfDiagPlugin {
    settings: DiagSettings,
    scenario: Option<ScenarioRun>,
}

impl SfDiagPlugin {
    pub fn new(settings: DiagSettings) -> Self {
        Self {
            settings,
            scenario: None,
        }
    }

    /// Runs `scenario` from app start. Fails if it is invalid.
    pub fn with_scenario(mut self, scenario: Scenario) -> Result<Self, DiagError> {
        self.scenario = Some(ScenarioRun::new(scenario)?);
        Ok(self)
    }
}

impl Plugin for SfDiagPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<DiagCounters>() {
            app.insert_resource(DiagCounters::default());
        }
        app.insert_resource(self.settings.clone())
            .insert_resource(FlyThroughPose(
                self.scenario.as_ref().and_then(ScenarioRun::start_pose),
            ))
            .configure_sets(
                First,
                (DiagSystems::Boundary, DiagSystems::ApplyPose)
                    .chain()
                    .after(TimeSystems),
            )
            .add_systems(
                First,
                (
                    frame_boundary.in_set(DiagSystems::Boundary),
                    apply_pose.in_set(DiagSystems::ApplyPose),
                ),
            );
        if let Some(run) = &self.scenario {
            app.insert_resource(DiagScenario {
                run: run.clone(),
                recorder: Recorder::new(),
                warmup: Recorder::new(),
                wall_start: None,
                hold: false,
            });
        }
    }
}

#[allow(clippy::too_many_arguments)] // a system's parameters
fn frame_boundary(
    time: Res<Time<Real>>,
    counters: Res<DiagCounters>,
    settings: Res<DiagSettings>,
    store: Option<Res<DiagnosticsStore>>,
    scenario: Option<ResMut<DiagScenario>>,
    mut pose: ResMut<FlyThroughPose>,
    mut seen: Local<HashMap<String, Instant>>,
    entities: Query<()>,
    mut last_entities: Local<Option<usize>>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(mut scenario) = scenario else {
        counters.clear();
        return;
    };
    let DiagScenario {
        run,
        recorder,
        warmup,
        wall_start,
        hold,
    } = &mut *scenario;
    if run.phase() == Phase::Done {
        counters.clear();
        return;
    }
    if let Some(store) = store {
        sample_diagnostics(&store, &settings.bevy_diagnostics, &counters, &mut seen);
    }
    // Net entity change over the frame that just ended. `Query::count` on
    // an archetypal query costs O(archetypes), not O(entities).
    #[allow(clippy::cast_precision_loss)] // entity counts are far below 2^52
    {
        let count = entities.count();
        counters.set_named(ENTITIES, sf_diag::unit::COUNT, count as f64);
        if let Some(last) = last_entities.replace(count) {
            counters.add_named(
                ENTITIES_DELTA,
                sf_diag::unit::COUNT,
                count as f64 - last as f64,
            );
        }
    }
    let frame_ms = time.delta_secs_f64() * 1000.0;
    if frame_ms <= 0.0 {
        // The first frame has no duration yet.
        counters.clear();
        return;
    }

    let was = run.phase();
    let step = run.advance(frame_ms, *hold);
    if step.record {
        recorder.commit_frame(frame_ms, &counters);
    } else if was == Phase::Warmup {
        warmup.commit_frame(frame_ms, &counters);
    } else {
        Recorder::discard(&counters);
    }
    if was == Phase::Warmup && step.phase == Phase::Measure {
        *wall_start = Some(sf_diag::suite::unix_now());
    }
    pose.0 = step.pose;

    if step.just_finished {
        sf_diag::pacing::derive_work(recorder);
        sf_diag::pacing::derive_work(warmup);
        let wall = (wall_start.unwrap_or_default(), sf_diag::suite::unix_now());
        let finished = finish(run.scenario(), recorder, warmup, wall, &settings);
        if finished.passed() {
            log::info!("{}", finished.report.summary());
        } else {
            log::warn!("{}", finished.report.summary());
        }
        for error in &finished.errors {
            log::error!("sf_diag: {error}");
        }
        if settings.exit_on_finish {
            exit.write(if finished.passed() {
                AppExit::Success
            } else {
                AppExit::error()
            });
        }
        commands.insert_resource(finished);
    }
}

/// Records each Bevy diagnostic's newest measurement, once, as a gauge.
fn sample_diagnostics(
    store: &DiagnosticsStore,
    filter: &DiagnosticFilter,
    counters: &Counters,
    seen: &mut HashMap<String, Instant>,
) {
    let skip = [
        FrameTimeDiagnosticsPlugin::FRAME_TIME,
        FrameTimeDiagnosticsPlugin::FPS,
        FrameTimeDiagnosticsPlugin::FRAME_COUNT,
    ];
    for diagnostic in store.iter() {
        let path = diagnostic.path().as_str();
        if !diagnostic.is_enabled
            || !filter.accepts(path)
            || skip.iter().any(|s| s.as_str() == path)
        {
            continue;
        }
        let Some(m) = diagnostic.measurement() else {
            continue;
        };
        match seen.get_mut(path) {
            Some(last) if *last >= m.time => continue,
            Some(last) => *last = m.time,
            None => {
                seen.insert(path.to_owned(), m.time);
            }
        }
        counters.set_named(path, &diagnostic.suffix, m.value);
    }
}

/// Builds the report, compares it with the baseline, and writes the files.
fn finish(
    scenario: &Scenario,
    recorder: &Recorder,
    warmup: &Recorder,
    (wall_start, wall_end): (f64, f64),
    settings: &DiagSettings,
) -> DiagReport {
    let mut report = Report::build(
        &scenario.name,
        recorder,
        &scenario.gates,
        scenario.spike_factor,
        settings.meta.clone(),
    );
    report.add_hitches(recorder, Some(warmup), settings.hitch_ms);
    report.wall_start_unix_s = Some(wall_start);
    report.wall_end_unix_s = Some(wall_end);
    let mut errors = Vec::new();

    if let Some(path) = settings
        .baseline
        .as_ref()
        .filter(|_| !settings.update_baseline)
    {
        if path.exists() {
            let label = path.display().to_string();
            if let Err(e) = Report::load(path)
                .and_then(|baseline| report.compare(&baseline, &label, &scenario.tolerance))
            {
                errors.push(format!("baseline {label}: {e}"));
            }
        } else {
            log::warn!(
                "sf_diag: no baseline at {}; set `update_baseline` to record one",
                path.display()
            );
        }
    }

    let stem = file_stem(&scenario.name);
    let json = settings.output_dir.join(format!("{stem}.json"));
    let csv = settings.output_dir.join(format!("{stem}.csv"));
    let json = match report.save(&json) {
        Ok(()) => Some(json),
        Err(e) => {
            errors.push(format!("writing {}: {e}", json.display()));
            None
        }
    };
    let csv = match write_csv(recorder, &csv) {
        Ok(()) => Some(csv),
        Err(e) => {
            errors.push(format!("writing {}: {e}", csv.display()));
            None
        }
    };
    // Warm-up frames too (loading hitches), next to the measured ones.
    if !warmup.frame_ms().is_empty() {
        let path = settings.output_dir.join(format!("{stem}.warmup.csv"));
        if let Err(e) = write_csv(warmup, &path) {
            errors.push(format!("writing {}: {e}", path.display()));
        }
    }
    let markdown = if settings.markdown {
        let path = settings.output_dir.join(format!("{stem}.md"));
        match std::fs::write(&path, report.to_markdown()) {
            Ok(()) => Some(path),
            Err(e) => {
                errors.push(format!("writing {}: {e}", path.display()));
                None
            }
        }
    } else {
        None
    };
    if let Some(path) = settings
        .baseline
        .as_ref()
        .filter(|_| settings.update_baseline)
    {
        match report.save(path) {
            Ok(()) => log::info!("sf_diag: baseline updated at {}", path.display()),
            Err(e) => errors.push(format!("writing baseline {}: {e}", path.display())),
        }
    }

    DiagReport {
        report,
        json,
        csv,
        markdown,
        errors,
    }
}

fn write_csv(recorder: &Recorder, path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut w = BufWriter::new(File::create(path)?);
    recorder.write_csv(&mut w)?;
    w.flush()
}

/// A scenario name as a file name: anything but `[A-Za-z0-9._-]` becomes `_`.
fn file_stem(name: &str) -> String {
    let stem: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect();
    if stem.is_empty() {
        "scenario".to_owned()
    } else {
        stem
    }
}

fn apply_pose(
    settings: Res<DiagSettings>,
    pose: Res<FlyThroughPose>,
    mut cameras: Query<&mut Transform, With<FlyThroughCamera>>,
) {
    if !settings.apply_transform {
        return;
    }
    let Some(pose) = pose.0 else {
        return;
    };
    for mut transform in &mut cameras {
        transform.translation = pose.position.as_vec3();
        transform.rotation = pose.rotation.as_quat();
    }
}
