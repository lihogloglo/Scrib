//! Run reports (JSON), baseline comparison, and spike attribution.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::DiagError;
use crate::counters::{CounterKind, unit};
use crate::gate::{Gate, GateResult};
use crate::recorder::Recorder;
use crate::stats::Stats;

/// Version of the JSON report layout. Baselines must match it.
pub const REPORT_FORMAT: u32 = 1;

/// How many of the worst spike frames a report lists.
pub const MAX_SPIKES: usize = 32;

/// How many counters a spike lists as likely culprits.
const MAX_CULPRITS: usize = 5;

/// Frame-time summary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameSummary {
    /// Frame-time stats in milliseconds.
    pub stats: Stats,
    /// 1000 / median frame time.
    pub fps_median: f64,
    /// 1000 / p99 frame time: the 1st-percentile ("1 % low") frame rate.
    pub fps_p1: f64,
    /// A spike is a frame longer than `spike_factor` × the median.
    pub spike_factor: f64,
    /// Number of spike frames.
    pub spikes: usize,
    /// Stutters per minute: frames over `spike_factor` × the moving median of
    /// the previous [`crate::pacing::STUTTER_WINDOW`] frames (diag.md "Frame pacing").
    #[serde(default)]
    pub stutters_per_min: f64,
    /// The same on the work time (`frame/work/ms`), when measured: diagnosis, not a gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_stutters_per_min: Option<f64>,
}

/// One counter over the run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CounterSummary {
    pub unit: String,
    pub kind: CounterKind,
    /// Sum over the run, for [`CounterKind::Sum`] counters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// The last reported value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// Stats over the recorded frames (Sum: untouched frames count as 0;
    /// Gauge: only frames where it was reported).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_frame: Option<Stats>,
}

/// A counter's value in a spike frame.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Culprit {
    pub counter: String,
    pub value: f64,
}

/// One of the worst frames, with the millisecond counters that were
/// largest in it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spike {
    /// Index of the frame among the recorded frames (the CSV's `frame`).
    pub frame: usize,
    /// End of the frame, seconds since recording began.
    pub t_s: f64,
    pub frame_ms: f64,
    pub culprits: Vec<Culprit>,
    /// The frame's work time (`frame/work/ms`, diag.md "Frame pacing"), when measured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_ms: Option<f64>,
}

/// Allowed drift against a baseline before a change counts.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tolerance {
    /// Relative change, e.g. 0.10 = 10 %.
    pub relative: f64,
    /// Absolute change in milliseconds that must also be exceeded, so
    /// sub-millisecond noise on small numbers never fails a run.
    pub absolute_ms: f64,
}

impl Default for Tolerance {
    fn default() -> Self {
        Self {
            relative: 0.10,
            absolute_ms: 0.25,
        }
    }
}

/// A metric that moved past the tolerance against the baseline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Regression {
    /// `frame_ms.median`, `counter.<name>.total`, ...
    pub metric: String,
    pub baseline: f64,
    pub current: f64,
    /// Relative change, `current / baseline - 1`.
    pub change: f64,
}

/// The result of a run: the JSON report and baseline format.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub format: u32,
    pub scenario: String,
    /// App-supplied context: machine, build, quality preset, resolution.
    #[serde(default)]
    pub meta: BTreeMap<String, String>,
    /// Sum of recorded frame times, in seconds.
    pub duration_s: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<FrameSummary>,
    #[serde(default)]
    pub counters: BTreeMap<String, CounterSummary>,
    /// The worst spike frames, worst first, at most [`MAX_SPIKES`].
    #[serde(default)]
    pub spikes: Vec<Spike>,
    /// The worst frames by work time (`frame/work/ms` over `spike_factor` x its
    /// median), worst first: real work spikes, whatever the presentation did.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub work_spikes: Vec<Spike>,
    #[serde(default)]
    pub gates: Vec<GateResult>,
    /// Set when compared against a baseline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<String>,
    #[serde(default)]
    pub regressions: Vec<Regression>,
    #[serde(default)]
    pub improvements: Vec<Regression>,
    /// Every gate passed and nothing regressed.
    pub passed: bool,
    /// Frames over [`Report::hitch_ms`], warm-up included, with what
    /// changed in them. Set by [`Report::add_hitches`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hitches: Option<HitchTable>,
    /// The warm-up (loading, settling) before measurement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmup: Option<WarmupSummary>,
    /// Wall clock (Unix seconds) when measurement began and ended, to line
    /// up outside samplers (`nvidia-smi`, traces) with the window.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wall_start_unix_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wall_end_unix_s: Option<f64>,
}

/// How many hitches a report lists (the slowest, in time order).
pub const MAX_HITCHES: usize = 64;

/// How many changes a hitch lists.
const MAX_CHURN: usize = 12;

/// Which part of a run a frame belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunPhase {
    Warmup,
    Measure,
}

/// One slow frame and what changed in it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hitch {
    pub phase: RunPhase,
    /// Index among the phase's recorded frames.
    pub frame: usize,
    /// End of the frame, seconds since the phase began.
    pub t_s: f64,
    pub frame_ms: f64,
    /// Every [`CounterKind::Sum`] counter that was non-zero in this frame
    /// or the one before (work is often reported a frame early or late:
    /// the render world runs a frame behind), summed over both; millisecond
    /// counters first, largest first.
    pub churn: Vec<Culprit>,
}

/// Every frame over a threshold.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HitchTable {
    pub threshold_ms: f64,
    /// Hitches in the measured window.
    pub measured: usize,
    /// Hitches during warm-up (loading).
    pub warmup: usize,
    /// The slowest, at most [`MAX_HITCHES`], in time order.
    pub rows: Vec<Hitch>,
}

/// Warm-up frames: loading and settling before measurement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarmupSummary {
    pub frames: usize,
    pub duration_s: f64,
    pub worst_ms: f64,
}

/// `count` events over `minutes` as a rate (0 for an empty run).
#[allow(clippy::cast_precision_loss)]
fn per_minute(count: usize, minutes: f64) -> f64 {
    if minutes > 0.0 { count as f64 / minutes } else { 0.0 }
}

/// The hitch threshold when none is given: 2.5 × the median frame, at
/// least 25 ms.
pub fn default_hitch_ms(median_ms: f64) -> f64 {
    (2.5 * median_ms).max(25.0)
}

impl Report {
    /// Summarizes a recorder and evaluates `gates`.
    pub fn build(
        scenario: &str,
        recorder: &Recorder,
        gates: &[Gate],
        spike_factor: f64,
        meta: BTreeMap<String, String>,
    ) -> Self {
        let frame_ms = recorder.frame_ms();
        let stats = Stats::from_samples(frame_ms);
        let minutes = frame_ms.iter().sum::<f64>() / 60_000.0;
        let frames = stats.map(|stats| {
            let threshold = stats.median * spike_factor;
            FrameSummary {
                stats,
                fps_median: fps(stats.median),
                fps_p1: fps(stats.p99),
                spike_factor,
                spikes: frame_ms.iter().filter(|&&ms| ms > threshold).count(),
                stutters_per_min: per_minute(crate::pacing::stutters(frame_ms, spike_factor, crate::pacing::STUTTER_WINDOW).len(), minutes),
                work_stutters_per_min: recorder.get(crate::pacing::WORK).map(|w| {
                    per_minute(crate::pacing::stutters(w.values(), spike_factor, crate::pacing::STUTTER_WINDOW).len(), minutes)
                }),
            }
        });

        let counters: BTreeMap<String, CounterSummary> = recorder
            .series()
            .iter()
            .map(|series| {
                let def = series.def();
                let summary = CounterSummary {
                    unit: def.unit.clone(),
                    kind: def.kind,
                    total: (def.kind == CounterKind::Sum).then_some(series.total()),
                    last: series.last(),
                    per_frame: Stats::from_samples(series.values()),
                };
                (def.name.clone(), summary)
            })
            .collect();

        let frame_spikes = frames
            .as_ref()
            .map(|f| spikes(recorder, frame_ms, f.stats.median * spike_factor))
            .unwrap_or_default();
        let work_spikes = recorder
            .get(crate::pacing::WORK)
            .and_then(|s| Some((s.values(), Stats::from_samples(s.values())?)))
            .map(|(values, s)| spikes(recorder, values, s.median * spike_factor))
            .unwrap_or_default();

        let gates: Vec<GateResult> = gates
            .iter()
            .map(|gate| gate.evaluate(recorder, stats.as_ref(), &counters))
            .collect();
        let passed = gates.iter().all(|g| g.passed);

        Self {
            format: REPORT_FORMAT,
            scenario: scenario.to_owned(),
            meta,
            duration_s: frame_ms.iter().sum::<f64>() / 1000.0,
            frames,
            counters,
            spikes: frame_spikes,
            work_spikes,
            gates,
            baseline: None,
            regressions: Vec::new(),
            improvements: Vec::new(),
            passed,
            hitches: None,
            warmup: None,
            wall_start_unix_s: None,
            wall_end_unix_s: None,
        }
    }

    /// Fills [`Report::hitches`] (and [`Report::warmup`]) from the
    /// measured recorder and, optionally, the warm-up's. `threshold_ms`
    /// defaults to [`default_hitch_ms`] of the measured median.
    pub fn add_hitches(
        &mut self,
        measure: &Recorder,
        warmup: Option<&Recorder>,
        threshold_ms: Option<f64>,
    ) {
        let median = self.frames.as_ref().map_or(0.0, |f| f.stats.median);
        let threshold = threshold_ms.unwrap_or_else(|| default_hitch_ms(median));
        let mut rows = Vec::new();
        let mut counts = [0usize; 2];
        for (phase, recorder) in [
            (RunPhase::Warmup, warmup),
            (RunPhase::Measure, Some(measure)),
        ] {
            let Some(recorder) = recorder else { continue };
            let found = hitches(recorder, threshold, phase);
            counts[usize::from(phase == RunPhase::Measure)] = found.len();
            rows.extend(found);
        }
        // Keep the slowest, then show them in time order.
        rows.sort_by(|a, b| b.frame_ms.total_cmp(&a.frame_ms));
        rows.truncate(MAX_HITCHES);
        rows.sort_by(|a, b| {
            (a.phase == RunPhase::Measure, a.frame).cmp(&(b.phase == RunPhase::Measure, b.frame))
        });
        self.hitches = Some(HitchTable {
            threshold_ms: threshold,
            measured: counts[1],
            warmup: counts[0],
            rows,
        });
        if let Some(w) = warmup {
            let ms = w.frame_ms();
            self.warmup = Some(WarmupSummary {
                frames: ms.len(),
                duration_s: ms.iter().sum::<f64>() / 1000.0,
                worst_ms: ms.iter().copied().fold(0.0, f64::max),
            });
        }
    }

    /// Compares against a baseline report of the same scenario: median,
    /// p95 and p99 frame time, and the total and p95 of every millisecond
    /// counter. A metric regresses when it exceeds both the relative and
    /// the absolute tolerance. Updates `regressions`, `improvements`, and
    /// `passed`. `label` names the baseline in the report (e.g. its path).
    pub fn compare(
        &mut self,
        baseline: &Report,
        label: &str,
        tolerance: &Tolerance,
    ) -> Result<(), DiagError> {
        if baseline.format != REPORT_FORMAT {
            return Err(DiagError::FormatMismatch {
                found: baseline.format,
                expected: REPORT_FORMAT,
            });
        }
        if baseline.scenario != self.scenario {
            return Err(DiagError::ScenarioMismatch {
                baseline: baseline.scenario.clone(),
                current: self.scenario.clone(),
            });
        }
        let old: BTreeMap<String, f64> = baseline.comparable_metrics().into_iter().collect();
        self.regressions.clear();
        self.improvements.clear();
        for (metric, current) in self.comparable_metrics() {
            let Some(&base) = old.get(&metric) else {
                continue;
            };
            let delta = current - base;
            let change = if base > 0.0 {
                current / base - 1.0
            } else {
                0.0
            };
            let entry = Regression {
                metric,
                baseline: base,
                current,
                change,
            };
            let beyond_absolute = delta.abs() > tolerance.absolute_ms;
            if beyond_absolute && current > base * (1.0 + tolerance.relative) {
                self.regressions.push(entry);
            } else if beyond_absolute && current < base * (1.0 - tolerance.relative) {
                self.improvements.push(entry);
            }
        }
        self.baseline = Some(label.to_owned());
        self.passed = self.gates.iter().all(|g| g.passed) && self.regressions.is_empty();
        Ok(())
    }

    /// The metrics [`Report::compare`] looks at, all in milliseconds.
    pub fn comparable_metrics(&self) -> Vec<(String, f64)> {
        let mut out = Vec::new();
        if let Some(f) = &self.frames {
            out.push(("frame_ms.median".to_owned(), f.stats.median));
            out.push(("frame_ms.p95".to_owned(), f.stats.p95));
            out.push(("frame_ms.p99".to_owned(), f.stats.p99));
        }
        for (name, c) in &self.counters {
            if c.unit != unit::MS {
                continue;
            }
            if let Some(total) = c.total {
                out.push((format!("counter.{name}.total"), total));
            }
            if let Some(s) = &c.per_frame {
                out.push((format!("counter.{name}.p95"), s.p95));
            }
        }
        out
    }

    /// One-paragraph summary for logs.
    pub fn summary(&self) -> String {
        let mut text = format!("sf_diag `{}`: ", self.scenario);
        match &self.frames {
            Some(f) => {
                let _ = write!(
                    text,
                    "{} frames in {:.1} s, median {:.2} ms ({:.0} fps), p99 {:.2} ms ({:.0} fps), worst {:.2} ms, {} spikes over {}x median, {:.1} stutters/min",
                    f.stats.count,
                    self.duration_s,
                    f.stats.median,
                    f.fps_median,
                    f.stats.p99,
                    f.fps_p1,
                    f.stats.max,
                    f.spikes,
                    f.spike_factor,
                    f.stutters_per_min
                );
            }
            None => text.push_str("no frames"),
        }
        let passed_gates = self.gates.iter().filter(|g| g.passed).count();
        let _ = write!(text, "; gates {passed_gates}/{}", self.gates.len());
        for gate in self.gates.iter().filter(|g| !g.passed) {
            let _ = write!(text, "; FAILED {}", gate.detail);
        }
        if self.baseline.is_some() {
            let _ = write!(
                text,
                "; {} regressions, {} improvements",
                self.regressions.len(),
                self.improvements.len()
            );
            for r in &self.regressions {
                let _ = write!(
                    text,
                    "; REGRESSED {} {:.3} -> {:.3} ({:+.1}%)",
                    r.metric,
                    r.baseline,
                    r.current,
                    r.change * 100.0
                );
            }
        }
        text.push_str(if self.passed { "; PASS" } else { "; FAIL" });
        text
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("reports always serialize")
    }

    pub fn from_json(text: &str) -> Result<Self, DiagError> {
        Ok(serde_json::from_str(text)?)
    }

    pub fn write_json(&self, w: impl Write) -> Result<(), DiagError> {
        serde_json::to_writer_pretty(w, self)?;
        Ok(())
    }

    /// Writes the report as JSON to `path`, creating parent directories.
    pub fn save(&self, path: &Path) -> Result<(), DiagError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut w = BufWriter::new(std::fs::File::create(path)?);
        self.write_json(&mut w)?;
        w.flush()?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, DiagError> {
        let r = BufReader::new(std::fs::File::open(path)?);
        Ok(serde_json::from_reader(r)?)
    }
}

fn fps(frame_ms: f64) -> f64 {
    if frame_ms > 0.0 {
        1000.0 / frame_ms
    } else {
        0.0
    }
}

/// Every frame of `recorder` over `threshold`, with its churn.
fn hitches(recorder: &Recorder, threshold: f64, phase: RunPhase) -> Vec<Hitch> {
    let frame_ms = recorder.frame_ms();
    let sums: Vec<_> = recorder
        .series()
        .iter()
        .filter(|s| s.def().kind == CounterKind::Sum)
        .collect();
    let mut t = 0.0;
    let mut out = Vec::new();
    for (frame, &ms) in frame_ms.iter().enumerate() {
        t += ms;
        if ms <= threshold {
            continue;
        }
        let mut churn: Vec<(bool, Culprit)> = sums
            .iter()
            .filter_map(|s| {
                let v = s.values();
                let value = v[frame] + if frame > 0 { v[frame - 1] } else { 0.0 };
                (value != 0.0).then(|| {
                    (
                        s.def().unit == unit::MS,
                        Culprit {
                            counter: s.def().name.clone(),
                            value,
                        },
                    )
                })
            })
            .collect();
        churn.sort_by(|(a_ms, a), (b_ms, b)| {
            b_ms.cmp(a_ms).then(b.value.abs().total_cmp(&a.value.abs()))
        });
        churn.truncate(MAX_CHURN);
        out.push(Hitch {
            phase,
            frame,
            t_s: t / 1000.0,
            frame_ms: ms,
            churn: churn.into_iter().map(|(_, c)| c).collect(),
        });
    }
    out
}

/// The worst frames over `threshold`, with their largest millisecond
/// counters.
fn spikes(recorder: &Recorder, values: &[f64], threshold: f64) -> Vec<Spike> {
    let frame_ms = recorder.frame_ms();
    let work = recorder.get(crate::pacing::WORK).map(crate::recorder::Series::values);
    let mut end_s = Vec::with_capacity(frame_ms.len());
    let mut t = 0.0;
    for ms in frame_ms {
        t += ms;
        end_s.push(t / 1000.0);
    }
    let mut worst: Vec<usize> = (0..frame_ms.len())
        .filter(|&i| values[i] > threshold)
        .collect();
    worst.sort_by(|&a, &b| values[b].total_cmp(&values[a]).then(a.cmp(&b)));
    worst.truncate(MAX_SPIKES);
    worst
        .into_iter()
        .map(|frame| {
            let mut culprits: Vec<Culprit> = recorder
                .series()
                .iter()
                .filter(|s| s.def().unit == unit::MS && !crate::pacing::DERIVED.contains(&s.def().name.as_str()))
                .filter_map(|s| {
                    let value = s.values()[frame];
                    (value > 0.0).then(|| Culprit {
                        counter: s.def().name.clone(),
                        value,
                    })
                })
                .collect();
            culprits.sort_by(|a, b| b.value.total_cmp(&a.value));
            culprits.truncate(MAX_CULPRITS);
            Spike {
                frame,
                t_s: end_s[frame],
                frame_ms: frame_ms[frame],
                culprits,
                work_ms: work.map(|w| w[frame]).filter(|v| v.is_finite()),
            }
        })
        .collect()
}
