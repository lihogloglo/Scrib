//! Pass/fail gates over a run.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::recorder::Recorder;
use crate::report::CounterSummary;
use crate::stats::{Stat, Stats};

/// Presented stutters per minute the streaming gate allows (see
/// [`Gate::wishlist_streaming_stutters`]): about one OS or driver hiccup a
/// minute; WISHLIST §0's "worst < 1.5× median" read strictly would be 0.
pub const STREAMING_STUTTERS_PER_MIN: f64 = 1.0;

/// Which number of a counter a [`Gate::MaxCounter`] checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterStat {
    /// Sum over the whole run ([`crate::CounterKind::Sum`] counters only),
    /// e.g. a bake's total time.
    Total,
    Mean,
    Median,
    P95,
    P99,
    /// Worst single frame, e.g. a stage's per-frame budget.
    Max,
}

impl CounterStat {
    fn per_frame(self) -> Option<Stat> {
        match self {
            Self::Total => None,
            Self::Mean => Some(Stat::Mean),
            Self::Median => Some(Stat::Median),
            Self::P95 => Some(Stat::P95),
            Self::P99 => Some(Stat::P99),
            Self::Max => Some(Stat::Max),
        }
    }
}

/// A condition a run must meet. Frame times are in milliseconds. A gate
/// without the data it needs fails.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "gate", rename_all = "snake_case")]
pub enum Gate {
    /// No frame longer than `ms`.
    MaxFrameMs { ms: f64 },
    /// Worst frame shorter than `factor` × the median frame.
    WorstOverMedian { factor: f64 },
    /// A per-frame counter's worst value shorter than `factor` × its median:
    /// the streaming gate on work time (`frame/work/ms`, diag.md "Frame pacing").
    CounterWorstOverMedian { counter: String, factor: f64 },
    /// At most `per_minute` stutters per minute of recorded time: frames whose
    /// value is over `factor` × the moving median of the previous `window`
    /// frames ([`crate::pacing::stutters`]). `counter` names a per-frame ms
    /// counter (e.g. `frame/work/ms`); without it the presented frame time is
    /// used. diag.md "Frame pacing".
    MaxStutters {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        counter: Option<String>,
        factor: f64,
        #[serde(default = "default_stutter_window")]
        window: usize,
        per_minute: f64,
    },
    /// At least `fps`, computed from a frame-time stat: `median` gives the
    /// median fps, `p99` the 1st-percentile ("1 % low") fps.
    MinFps { stat: Stat, fps: f64 },
    /// A frame-time stat at most `ms`.
    MaxFrameStat { stat: Stat, ms: f64 },
    /// At most `count` frames longer than `factor` × the median frame.
    MaxSpikes { factor: f64, count: usize },
    /// A counter's stat at most `max` (in the counter's unit).
    MaxCounter {
        counter: String,
        stat: CounterStat,
        max: f64,
    },
}

/// Outcome of one gate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GateResult {
    pub gate: Gate,
    /// The measured value, if the data existed.
    pub value: Option<f64>,
    pub passed: bool,
    /// Human-readable outcome, e.g. `worst 9.10 ms / median 6.90 ms = 1.32 < 1.5`.
    pub detail: String,
}

impl Gate {
    /// WISHLIST §0, worst frame while streaming: below 1.5× the median frame,
    /// and no frame over 33 ms.
    pub fn wishlist_streaming() -> Vec<Gate> {
        vec![
            Gate::WorstOverMedian { factor: 1.5 },
            Gate::MaxFrameMs { ms: 33.0 },
        ]
    }

    /// The streaming gate as measured (diag.md "Frame pacing", ARCHITECTURE
    /// §10 2026-10-12): presented stutters (frames over 1.5× the moving
    /// median of the previous [`crate::pacing::STUTTER_WINDOW`] frames) at
    /// most [`STREAMING_STUTTERS_PER_MIN`] per minute, and no frame over
    /// 33 ms. Measure it uncapped (no vsync) in a window that fits the
    /// display; the work counters attribute the stutters, they don't gate.
    pub fn wishlist_streaming_stutters() -> Vec<Gate> {
        vec![
            Gate::MaxStutters {
                counter: None,
                factor: 1.5,
                window: crate::pacing::STUTTER_WINDOW,
                per_minute: STREAMING_STUTTERS_PER_MIN,
            },
            Gate::MaxFrameMs { ms: 33.0 },
        ]
    }

    /// Median frame rate of at least `fps` (WISHLIST §0 scenario targets).
    pub fn target_fps(fps: f64) -> Gate {
        Gate::MinFps {
            stat: Stat::Median,
            fps,
        }
    }

    /// Evaluates the gate against a run's frame times (with their stats)
    /// and counter summaries.
    pub fn evaluate(
        &self,
        recorder: &Recorder,
        frames: Option<&Stats>,
        counters: &BTreeMap<String, CounterSummary>,
    ) -> GateResult {
        let frame_ms = recorder.frame_ms();
        let (value, passed, detail) = match self {
            Gate::MaxFrameMs { ms } => match frames {
                Some(s) => (
                    Some(s.max),
                    s.max <= *ms,
                    format!("worst frame {:.2} ms, limit {ms} ms", s.max),
                ),
                None => no_frames(),
            },
            Gate::WorstOverMedian { factor } => match frames {
                Some(s) if s.median > 0.0 => {
                    let ratio = s.max / s.median;
                    (
                        Some(ratio),
                        ratio < *factor,
                        format!(
                            "worst {:.2} ms / median {:.2} ms = {ratio:.2}, limit {factor}",
                            s.max, s.median
                        ),
                    )
                }
                _ => no_frames(),
            },
            Gate::CounterWorstOverMedian { counter, factor } => {
                counter_worst_over_median(counters, counter, *factor)
            }
            Gate::MaxStutters { counter, factor, window, per_minute } => {
                max_stutters(recorder, counter.as_deref(), *factor, *window, *per_minute)
            }
            Gate::MinFps { stat, fps } => match frames {
                Some(s) if s.get(*stat) > 0.0 => {
                    let measured = 1000.0 / s.get(*stat);
                    (
                        Some(measured),
                        measured >= *fps,
                        format!("{stat:?} fps {measured:.1}, target {fps}"),
                    )
                }
                _ => no_frames(),
            },
            Gate::MaxFrameStat { stat, ms } => match frames {
                Some(s) => {
                    let v = s.get(*stat);
                    (
                        Some(v),
                        v <= *ms,
                        format!("{stat:?} frame {v:.2} ms, limit {ms} ms"),
                    )
                }
                None => no_frames(),
            },
            Gate::MaxSpikes { factor, count } => match frames {
                Some(s) => {
                    let threshold = s.median * factor;
                    let spikes = frame_ms.iter().filter(|&&ms| ms > threshold).count();
                    #[allow(clippy::cast_precision_loss)]
                    let v = spikes as f64;
                    (
                        Some(v),
                        spikes <= *count,
                        format!(
                            "{spikes} frames over {factor}x median ({threshold:.2} ms), limit {count}"
                        ),
                    )
                }
                None => no_frames(),
            },
            Gate::MaxCounter { counter, stat, max } => {
                let summary = counters.get(counter);
                let v = summary.and_then(|c| match stat.per_frame() {
                    None => c.total,
                    Some(stat) => c.per_frame.map(|s| s.get(stat)),
                });
                match (summary, v) {
                    (Some(c), Some(v)) => (
                        Some(v),
                        v <= *max,
                        format!("{counter} {stat:?} {v:.3} {}, limit {max}", c.unit),
                    ),
                    (None, _) => (
                        None,
                        false,
                        format!("counter `{counter}` was never reported"),
                    ),
                    (Some(_), None) => (
                        None,
                        false,
                        format!("counter `{counter}` has no {stat:?} (gauge total, or no frames)"),
                    ),
                }
            }
        };
        GateResult {
            gate: self.clone(),
            value,
            passed,
            detail,
        }
    }
}

fn counter_worst_over_median(
    counters: &BTreeMap<String, CounterSummary>,
    counter: &str,
    factor: f64,
) -> (Option<f64>, bool, String) {
    match counters.get(counter).and_then(|c| c.per_frame) {
        Some(s) if s.median > 0.0 => {
            let ratio = s.max / s.median;
            (
                Some(ratio),
                ratio < factor,
                format!("{counter}: worst {:.2} / median {:.2} = {ratio:.2}, limit {factor}", s.max, s.median),
            )
        }
        _ => (None, false, format!("counter `{counter}` has no per-frame values")),
    }
}

fn max_stutters(
    recorder: &Recorder,
    counter: Option<&str>,
    factor: f64,
    window: usize,
    per_minute: f64,
) -> (Option<f64>, bool, String) {
    let frame_ms = recorder.frame_ms();
    let values = match counter {
        Some(name) => recorder.get(name).map(crate::recorder::Series::values),
        None => (!frame_ms.is_empty()).then_some(frame_ms),
    };
    let label = counter.unwrap_or("frame_ms");
    let Some(values) = values else {
        return (None, false, format!("counter `{label}` has no per-frame values"));
    };
    let count = crate::pacing::stutters(values, factor, window).len();
    let minutes = frame_ms.iter().sum::<f64>() / 60_000.0;
    #[allow(clippy::cast_precision_loss)]
    let rate = if minutes > 0.0 { count as f64 / minutes } else { 0.0 };
    (
        Some(rate),
        rate <= per_minute,
        format!(
            "{label}: {count} stutters (> {factor}x moving median of {window}) in {minutes:.2} min = {rate:.2}/min, limit {per_minute}/min"
        ),
    )
}

fn default_stutter_window() -> usize {
    crate::pacing::STUTTER_WINDOW
}

fn no_frames() -> (Option<f64>, bool, String) {
    (None, false, "no frames recorded".to_owned())
}
