//! In-process A/B (`docs/reviews/bench-method-2026-11-23.md` §3): one launch, one live setting flipped
//! A B A B ... every `phase_s` seconds of measured time. The first A+B cycle is discarded (pipelines
//! compile on the first flip), and so is a settle at the start of every phase; each kept phase gives
//! frame median, p99, hitches and the main / render busy and GPU timed medians. Δ per metric is the
//! median over the pairs (A phase, the B phase right after it) of B - A: ABAB pairing cancels slow
//! drift (heat, clocks). |Δ| under the noise floor (0.5 ms, `docs/ledger/perf.md`) is noise.
//!
//! Pure: the schedule ([`AbSchedule::slot`]), the per-phase stats ([`phase_stats`]), the deltas
//! ([`deltas`]) and the report text. The app flips the value ([`AbSchedule::value_at`]) and writes
//! the files.

use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::ab::median;
use crate::stats::Stats;

/// The scenario's `ab` field, or `--ab NAME=A,B`: which setting, its two values (text, in the
/// setting's own syntax), and optionally the phase length, the measured phases per value and the
/// settle (unset: the app's defaults, [`AbDefaults`]).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbSpec {
    pub tunable: String,
    pub a: String,
    pub b: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phases: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settle_s: Option<f64>,
}

impl AbSpec {
    /// `NAME=A,B`; when a value itself holds commas (`1,2` of a vector), separate the two with `|`
    /// instead: `NAME=1,2|3,4`.
    pub fn parse(text: &str) -> Result<Self, String> {
        let bad = |why: &str| format!("--ab {text:?}: {why} (expected NAME=A,B, or NAME=A|B when a value holds commas)");
        let (name, values) = text.split_once('=').ok_or_else(|| bad("no `=`"))?;
        let name = name.trim();
        if name.is_empty() {
            return Err(bad("no tunable name"));
        }
        let sep = if values.contains('|') { '|' } else { ',' };
        let parts: Vec<&str> = values.split(sep).map(str::trim).collect();
        let [a, b] = parts.as_slice() else { return Err(bad("needs exactly two values")) };
        if a.is_empty() || b.is_empty() {
            return Err(bad("a value is empty"));
        }
        Ok(Self { tunable: name.to_owned(), a: (*a).to_owned(), b: (*b).to_owned(), phase_s: None, phases: None, settle_s: None })
    }
}

/// What an [`AbSpec`] leaves unset (the app's tunables) and the noise floor of the verdict.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbDefaults {
    pub phase_s: f64,
    pub phases: u32,
    pub settle_s: f64,
    pub noise_ms: f64,
}

/// One of the two values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arm {
    A,
    B,
}

impl Arm {
    /// A in even phases, B in odd ones.
    pub fn of_phase(phase: u32) -> Self {
        if phase.is_multiple_of(2) { Self::A } else { Self::B }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
        }
    }
}

/// What a frame starting at measured time `t` counts for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    /// The first A+B cycle: discarded (first-flip compiles, uploads).
    FirstCycle { phase: u32 },
    /// The settle at the start of a later phase: discarded.
    Settle { phase: u32 },
    /// Measured, for `phase` (0-based; phases 0 and 1 are the first cycle).
    Keep { phase: u32, arm: Arm },
    /// After the last phase.
    After,
}

/// A resolved A/B: every field set and checked.
#[derive(Clone, Debug, PartialEq)]
pub struct AbSchedule {
    pub tunable: String,
    pub a: String,
    pub b: String,
    /// Seconds per phase.
    pub phase_s: f64,
    /// Measured phases per value (pairs); one more cycle runs first and is discarded.
    pub phases: u32,
    /// Seconds left out at the start of every phase.
    pub settle_s: f64,
    /// A |Δ| under this (ms) is noise.
    pub noise_ms: f64,
}

impl AbSchedule {
    /// `spec` with `defaults` for what it leaves unset; refuses a bad phase length, count or settle.
    pub fn new(spec: &AbSpec, defaults: AbDefaults) -> Result<Self, String> {
        let s = Self {
            tunable: spec.tunable.clone(),
            a: spec.a.clone(),
            b: spec.b.clone(),
            phase_s: spec.phase_s.unwrap_or(defaults.phase_s),
            phases: spec.phases.unwrap_or(defaults.phases),
            settle_s: spec.settle_s.unwrap_or(defaults.settle_s),
            noise_ms: defaults.noise_ms,
        };
        let name = &s.tunable;
        if name.trim().is_empty() {
            return Err("ab: no tunable name".to_owned());
        }
        if s.a == s.b {
            return Err(format!("ab {name}: A and B are the same value {:?}", s.a));
        }
        if !(s.phase_s.is_finite() && s.phase_s > 0.0) {
            return Err(format!("ab {name}: phase_s must be > 0, got {}", s.phase_s));
        }
        if s.phases == 0 {
            return Err(format!("ab {name}: phases must be >= 1"));
        }
        if !(s.settle_s.is_finite() && s.settle_s >= 0.0 && s.settle_s < s.phase_s) {
            return Err(format!("ab {name}: settle_s must be in [0, phase_s = {}), got {}", s.phase_s, s.settle_s));
        }
        Ok(s)
    }

    /// Phases run, the discarded first cycle included: `2 × (phases + 1)`.
    pub fn phase_count(&self) -> u32 {
        2 * (self.phases + 1)
    }

    /// Measured seconds the whole schedule takes.
    pub fn total_s(&self) -> f64 {
        self.phase_s * f64::from(self.phase_count())
    }

    /// The phase running at measured time `t` (clamped to the first and last phase).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "t > 0, clamped to the phase count")]
    pub fn phase_at(&self, t: f64) -> u32 {
        let last = self.phase_count() - 1;
        if t.is_nan() || t <= 0.0 {
            return 0;
        }
        (t / self.phase_s).floor().min(f64::from(last)) as u32
    }

    /// The value active at measured time `t` (A in even phases, B in odd ones; A before the start).
    pub fn arm_at(&self, t: f64) -> Arm {
        Arm::of_phase(self.phase_at(t))
    }

    /// The value text of `arm`.
    pub fn value(&self, arm: Arm) -> &str {
        match arm {
            Arm::A => &self.a,
            Arm::B => &self.b,
        }
    }

    /// The value text active at measured time `t`.
    pub fn value_at(&self, t: f64) -> &str {
        self.value(self.arm_at(t))
    }

    /// What a frame that starts at measured time `t` counts for.
    pub fn slot(&self, t: f64) -> Slot {
        if t >= self.total_s() {
            return Slot::After;
        }
        let phase = self.phase_at(t);
        if phase < 2 {
            return Slot::FirstCycle { phase };
        }
        if t - f64::from(phase) * self.phase_s < self.settle_s {
            return Slot::Settle { phase };
        }
        Slot::Keep { phase, arm: Arm::of_phase(phase) }
    }

    /// One line: what flips, how often, what is kept.
    pub fn describe(&self) -> String {
        format!(
            "{} A={} B={}: {} x {:.1} s phases (ABAB), first A+B cycle and {:.2} s per phase discarded, {} pair(s) kept",
            self.tunable,
            self.a,
            self.b,
            self.phase_count(),
            self.phase_s,
            self.settle_s,
            self.phases
        )
    }
}

/// The per-frame columns of a measured run (index = measured frame). The optional ones may hold NaN
/// (no measurement that frame).
#[derive(Clone, Copy, Debug, Default)]
pub struct Columns<'a> {
    pub frame_ms: &'a [f64],
    pub main_ms: Option<&'a [f64]>,
    pub render_ms: Option<&'a [f64]>,
    pub gpu_ms: Option<&'a [f64]>,
}

/// One kept phase.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseStats {
    pub phase: u32,
    pub arm: Arm,
    pub frames: usize,
    pub median_ms: f64,
    pub p99_ms: f64,
    /// Frames over the run's hitch threshold.
    pub hitches: usize,
    pub main_ms: Option<f64>,
    pub render_ms: Option<f64>,
    pub gpu_ms: Option<f64>,
}

/// Stats of every kept phase that has frames, in phase order. A frame belongs to the phase of its
/// start time (the sum of the frames before it): the value set for it. `hitch_ms`: the run's
/// threshold (the same for every phase).
pub fn phase_stats(schedule: &AbSchedule, cols: Columns<'_>, hitch_ms: f64) -> Vec<PhaseStats> {
    let count = schedule.phase_count() as usize;
    let mut rows: Vec<Vec<usize>> = vec![Vec::new(); count];
    let mut t = 0.0;
    for (i, &ms) in cols.frame_ms.iter().enumerate() {
        if let Slot::Keep { phase, .. } = schedule.slot(t) {
            rows[phase as usize].push(i);
        }
        t += ms / 1000.0;
    }
    let pick = |col: Option<&[f64]>, idx: &[usize]| -> Option<f64> {
        let col = col?;
        let values: Vec<f64> = idx.iter().filter_map(|&i| col.get(i).copied()).filter(|v| v.is_finite()).collect();
        median(&values)
    };
    rows.iter()
        .enumerate()
        .filter(|(_, idx)| !idx.is_empty())
        .filter_map(|(phase, idx)| {
            let frames: Vec<f64> = idx.iter().map(|&i| cols.frame_ms[i]).collect();
            let stats = Stats::from_samples(&frames)?;
            #[allow(clippy::cast_possible_truncation, reason = "phase < phase_count, a u32")]
            let phase = phase as u32;
            Some(PhaseStats {
                phase,
                arm: Arm::of_phase(phase),
                frames: idx.len(),
                median_ms: stats.median,
                p99_ms: stats.p99,
                hitches: frames.iter().filter(|&&ms| ms > hitch_ms).count(),
                main_ms: pick(cols.main_ms, idx),
                render_ms: pick(cols.render_ms, idx),
                gpu_ms: pick(cols.gpu_ms, idx),
            })
        })
        .collect()
}

/// A compared quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metric {
    Median,
    P99,
    Hitches,
    Main,
    Render,
    Gpu,
}

impl Metric {
    pub const ALL: [Self; 6] = [Self::Median, Self::P99, Self::Hitches, Self::Main, Self::Render, Self::Gpu];

    pub fn label(self) -> &'static str {
        match self {
            Self::Median => "frame median ms",
            Self::P99 => "frame p99 ms",
            Self::Hitches => "hitches",
            Self::Main => "main busy ms",
            Self::Render => "render busy ms",
            Self::Gpu => "GPU timed ms",
        }
    }

    /// In milliseconds (the noise floor applies), or a count.
    pub fn is_ms(self) -> bool {
        self != Self::Hitches
    }

    #[allow(clippy::cast_precision_loss, reason = "hitch counts are tiny")]
    pub fn of(self, p: &PhaseStats) -> Option<f64> {
        match self {
            Self::Median => Some(p.median_ms),
            Self::P99 => Some(p.p99_ms),
            Self::Hitches => Some(p.hitches as f64),
            Self::Main => p.main_ms,
            Self::Render => p.render_ms,
            Self::Gpu => p.gpu_ms,
        }
    }
}

/// B - A of one metric over the pairs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Delta {
    pub metric: Metric,
    /// Pairs with both phases measured.
    pub pairs: usize,
    /// Median over the pairs of B - A.
    pub median: f64,
    pub min: f64,
    pub max: f64,
}

impl Delta {
    /// The verdict: for ms, |Δ| < `noise_ms` is noise, else B is slower or faster; for hitches, more,
    /// fewer or the same. `(pairs disagree)` when the pair deltas straddle zero.
    pub fn verdict(&self, noise_ms: f64) -> String {
        let base = if self.metric.is_ms() {
            if self.median.abs() < noise_ms {
                "noise"
            } else if self.median > 0.0 {
                "B slower"
            } else {
                "B faster"
            }
        } else if self.median > 0.0 {
            "B more hitches"
        } else if self.median < 0.0 {
            "B fewer hitches"
        } else {
            "same"
        };
        if self.min < 0.0 && self.max > 0.0 { format!("{base} (pairs disagree)") } else { base.to_owned() }
    }
}

/// Δ per metric over the pairs (A phase `2k`, B phase `2k + 1`); a metric no pair has is left out.
pub fn deltas(phases: &[PhaseStats]) -> Vec<Delta> {
    let find = |n: u32| phases.iter().find(|p| p.phase == n);
    let pairs: Vec<(&PhaseStats, &PhaseStats)> =
        phases.iter().filter(|p| p.arm == Arm::A).filter_map(|a| find(a.phase + 1).map(|b| (a, b))).collect();
    Metric::ALL
        .iter()
        .filter_map(|&metric| {
            let d: Vec<f64> = pairs.iter().filter_map(|(a, b)| Some(metric.of(b)? - metric.of(a)?)).collect();
            Some(Delta {
                metric,
                pairs: d.len(),
                median: median(&d)?,
                min: d.iter().copied().fold(f64::INFINITY, f64::min),
                max: d.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            })
        })
        .collect()
}

fn cell(v: Option<f64>) -> String {
    v.map_or_else(|| "?".to_owned(), |v| format!("{v:.2}"))
}

/// `ab.md`: the schedule, a row per kept phase, and the Δ line per metric with its pair range and
/// verdict. `scenario`: the scenario name; `hitch_ms`: the threshold used.
pub fn markdown(scenario: &str, schedule: &AbSchedule, phases: &[PhaseStats], deltas: &[Delta], hitch_ms: f64) -> String {
    let mut md = format!("# In-process A/B: `{}` on {scenario}\n\n", schedule.tunable);
    let _ = writeln!(md, "- {}", schedule.describe());
    let _ = writeln!(md, "- Δ = median over pairs of (B - A); |Δ| < {:.2} ms is noise; hitch = frame > {hitch_ms:.1} ms", schedule.noise_ms);
    md.push_str("\n| metric | Δ (B - A) | pair min..max | pairs | verdict |\n|---|---:|---:|---:|---|\n");
    for d in deltas {
        let _ = writeln!(md, "| {} | {:+.2} | {:+.2}..{:+.2} | {} | {} |", d.metric.label(), d.median, d.min, d.max, d.pairs, d.verdict(schedule.noise_ms));
    }
    if deltas.is_empty() {
        md.push_str("| (no complete pair) | | | 0 | |\n");
    }
    md.push_str("\n| phase | arm | value | frames | median ms | p99 ms | hitches | main ms | render ms | GPU ms |\n|---:|---|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for p in phases {
        let _ = writeln!(
            md,
            "| {} | {} | `{}` | {} | {:.2} | {:.2} | {} | {} | {} | {} |",
            p.phase,
            p.arm.label(),
            schedule.value(p.arm),
            p.frames,
            p.median_ms,
            p.p99_ms,
            p.hitches,
            cell(p.main_ms),
            cell(p.render_ms),
            cell(p.gpu_ms)
        );
    }
    md
}

/// `ab.csv`: one row per kept phase.
pub fn csv(schedule: &AbSchedule, phases: &[PhaseStats]) -> String {
    let mut out = String::from("phase,arm,value,frames,median_ms,p99_ms,hitches,main_ms,render_ms,gpu_ms\n");
    let opt = |v: Option<f64>| v.map_or_else(String::new, |v| format!("{v:.4}"));
    for p in phases {
        let _ = writeln!(
            out,
            "{},{},\"{}\",{},{:.4},{:.4},{},{},{},{}",
            p.phase,
            p.arm.label(),
            schedule.value(p.arm).replace('"', "\"\""),
            p.frames,
            p.median_ms,
            p.p99_ms,
            p.hitches,
            opt(p.main_ms),
            opt(p.render_ms),
            opt(p.gpu_ms)
        );
    }
    out
}

/// The headline: the frame-median Δ and its verdict (for a log line).
pub fn headline(schedule: &AbSchedule, deltas: &[Delta]) -> String {
    match deltas.iter().find(|d| d.metric == Metric::Median) {
        Some(d) => format!(
            "ab {} {}→{}: frame median Δ {:+.2} ms ({:+.2}..{:+.2} over {} pair(s)): {}",
            schedule.tunable,
            schedule.a,
            schedule.b,
            d.median,
            d.min,
            d.max,
            d.pairs,
            d.verdict(schedule.noise_ms)
        ),
        None => format!("ab {}: no complete pair measured", schedule.tunable),
    }
}

#[cfg(test)]
#[path = "flip_ab_tests.rs"]
mod tests;
