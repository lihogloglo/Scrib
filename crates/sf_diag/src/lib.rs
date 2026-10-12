//! Benchmark and diagnostics core (Bevy-free). Spec: `docs/specs/diag.md`.
//!
//! - [`Counters`]: named, thread-safe per-frame counters that any system or
//!   task reports into (stage times, item counts, queue depths).
//! - [`Recorder`]: the frame-time column plus one column per counter.
//! - [`Stats`], [`Gate`], [`Report`]: percentiles, pass/fail gates (the
//!   WISHLIST §0 targets), JSON/CSV/Markdown reports, hitch tables, and
//!   baseline comparison.
//! - [`CameraPath`] and [`ScenarioRun`]: scripted fly-throughs with a
//!   warm-up, measured at constant speed along a centripetal Catmull-Rom
//!   spline.
//! - [`suite`]: driving runs from another process (quiet-machine wait,
//!   `nvidia-smi` sampling, summaries across repeats); [`nsys`]: Nsight
//!   Systems per-pass GPU tables; [`trace`]: Chrome trace summaries.
//!
//! The Bevy glue lives in `sf_diag_bevy`.

mod counters;
mod gate;
mod markdown;
pub mod nsys;
pub mod pacing;
mod path;
mod recorder;
mod report;
mod scenario;
mod stats;
pub mod suite;
pub mod trace;

#[cfg(test)]
mod tests;

/// The math types of [`Keyframe`] and [`Pose`].
pub use glam;

pub use counters::{CounterDef, CounterId, CounterKind, Counters, unit};
pub use gate::{CounterStat, Gate, GateResult, STREAMING_STUTTERS_PER_MIN};
pub use markdown::PassTiming;
pub use path::{CameraPath, Keyframe, Pose};
pub use recorder::{Recorder, Series};
pub use report::{
    CounterSummary, Culprit, FrameSummary, Hitch, HitchTable, MAX_HITCHES, MAX_SPIKES,
    REPORT_FORMAT, Regression, Report, RunPhase, Spike, Tolerance, WarmupSummary, default_hitch_ms,
};
pub use scenario::{Measure, Phase, Scenario, ScenarioRun, Step, Warmup};
pub use stats::{Stat, Stats, percentile_sorted};

/// Errors from loading, validating, or writing diagnostics data.
#[derive(Debug, thiserror::Error)]
pub enum DiagError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid camera path: {0}")]
    Path(String),
    #[error("invalid scenario: {0}")]
    Scenario(String),
    #[error("baseline is report format {found}, expected {expected}")]
    FormatMismatch { found: u32, expected: u32 },
    #[error("baseline is for scenario `{baseline}`, not `{current}`")]
    ScenarioMismatch { baseline: String, current: String },
}
