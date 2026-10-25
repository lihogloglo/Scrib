//! Human-readable Markdown rendering of a [`Report`].

use std::fmt::Write as _;

use crate::counters::{CounterKind, unit};
use crate::report::{Report, RunPhase};

/// GPU pass timings recorded from Bevy's `RenderDiagnosticsPlugin`:
/// `render/<span path>/elapsed_gpu` (and `elapsed_cpu`).
const RENDER_PREFIX: &str = "render/";
const GPU_SUFFIX: &str = "/elapsed_gpu";
const CPU_SUFFIX: &str = "/elapsed_cpu";

/// One timed render pass.
#[derive(Clone, Debug, PartialEq)]
pub struct PassTiming {
    /// The span path below `render/`; nested spans contain `/`.
    pub span: String,
    /// Mean GPU milliseconds per sample.
    pub gpu_ms: Option<f64>,
    pub gpu_p95_ms: Option<f64>,
    /// Mean CPU (encoding) milliseconds per sample.
    pub cpu_ms: Option<f64>,
}

impl PassTiming {
    /// Top-level spans only: their sum doesn't count nested spans twice.
    pub fn is_top_level(&self) -> bool {
        !self.span.contains('/')
    }
}

impl Report {
    /// Per-pass GPU and CPU timings, largest GPU time first.
    pub fn pass_timings(&self) -> Vec<PassTiming> {
        let mut spans: Vec<&str> = self
            .counters
            .keys()
            .filter_map(|k| k.strip_prefix(RENDER_PREFIX))
            .filter_map(|k| {
                k.strip_suffix(GPU_SUFFIX)
                    .or_else(|| k.strip_suffix(CPU_SUFFIX))
            })
            .collect();
        spans.sort_unstable();
        spans.dedup();
        let stat = |span: &str, suffix: &str| {
            self.counters
                .get(&format!("{RENDER_PREFIX}{span}{suffix}"))
                .and_then(|c| c.per_frame)
        };
        let mut rows: Vec<PassTiming> = spans
            .into_iter()
            .map(|span| PassTiming {
                span: span.to_owned(),
                gpu_ms: stat(span, GPU_SUFFIX).map(|s| s.mean),
                gpu_p95_ms: stat(span, GPU_SUFFIX).map(|s| s.p95),
                cpu_ms: stat(span, CPU_SUFFIX).map(|s| s.mean),
            })
            .collect();
        rows.sort_by(|a, b| b.gpu_ms.unwrap_or(0.0).total_cmp(&a.gpu_ms.unwrap_or(0.0)));
        rows
    }

    /// Sum of the top-level passes' mean GPU time, if any were timed.
    pub fn timed_gpu_ms(&self) -> Option<f64> {
        let rows = self.pass_timings();
        let top: Vec<f64> = rows
            .iter()
            .filter(|r| r.is_top_level())
            .filter_map(|r| r.gpu_ms)
            .collect();
        (!top.is_empty()).then(|| top.iter().sum())
    }

    /// The report as Markdown: frame stats, gates, GPU passes, per-frame
    /// costs, gauges, churn totals, and the hitch table.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "# `{}`\n", self.scenario);
        if !self.meta.is_empty() {
            let meta: Vec<String> = self.meta.iter().map(|(k, v)| format!("{k}: {v}")).collect();
            let _ = writeln!(out, "- {}", meta.join(" · "));
        }
        self.frames_md(&mut out);
        self.gates_md(&mut out);
        self.passes_md(&mut out);
        self.costs_md(&mut out);
        self.gauges_md(&mut out);
        self.churn_md(&mut out);
        self.hitches_md(&mut out);
        out
    }

    fn frames_md(&self, out: &mut String) {
        let Some(f) = &self.frames else {
            let _ = writeln!(out, "- no frames recorded");
            return;
        };
        let s = &f.stats;
        let _ = writeln!(
            out,
            "- {} frames over {:.1} s · **median {:.2} ms ({:.0} fps)** · mean {:.2} · p95 {:.2} · p99 {:.2} \
             (1% low {:.0} fps) · worst {:.2} ms · {} spikes over {}× median · {:.1} stutters/min{}",
            s.count,
            self.duration_s,
            s.median,
            f.fps_median,
            s.mean,
            s.p95,
            s.p99,
            f.fps_p1,
            s.max,
            f.spikes,
            f.spike_factor,
            f.stutters_per_min,
            f.work_stutters_per_min.map_or(String::new(), |w| format!(" (work {w:.1}/min)"))
        );
        if let Some(w) = &self.warmup {
            let _ = writeln!(
                out,
                "- warm-up: {} frames over {:.1} s, worst {:.1} ms",
                w.frames, w.duration_s, w.worst_ms
            );
        }
        if let Some(h) = &self.hitches {
            let _ = writeln!(
                out,
                "- hitches (> {:.1} ms): **{}** measured, {} in warm-up",
                h.threshold_ms, h.measured, h.warmup
            );
        }
        if let (Some(timed), Some(f)) = (self.timed_gpu_ms(), &self.frames) {
            let _ = writeln!(
                out,
                "- timed GPU passes: {timed:.2} ms of a {:.2} ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)",
                f.stats.mean
            );
        }
        if let Some(base) = &self.baseline {
            let _ = writeln!(
                out,
                "- baseline `{base}`: {} regressions, {} improvements",
                self.regressions.len(),
                self.improvements.len()
            );
            for r in self.regressions.iter().chain(&self.improvements) {
                let _ = writeln!(
                    out,
                    "  - {} {:.3} → {:.3} ({:+.1}%)",
                    r.metric,
                    r.baseline,
                    r.current,
                    r.change * 100.0
                );
            }
        }
        let _ = writeln!(
            out,
            "- verdict: {}",
            if self.passed { "PASS" } else { "FAIL" }
        );
    }

    fn gates_md(&self, out: &mut String) {
        if self.gates.is_empty() {
            return;
        }
        let _ = writeln!(out, "\n## Gates\n");
        for g in &self.gates {
            let _ = writeln!(
                out,
                "- {} {}",
                if g.passed { "pass" } else { "**FAIL**" },
                g.detail
            );
        }
    }

    fn passes_md(&self, out: &mut String) {
        let rows = self.pass_timings();
        if rows.is_empty() {
            return;
        }
        let _ = writeln!(out, "\n## GPU passes (Bevy-timed, mean per frame)\n");
        let _ = writeln!(out, "| pass | GPU ms | GPU p95 | CPU ms |");
        let _ = writeln!(out, "|---|---:|---:|---:|");
        for r in rows {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} |",
                r.span,
                opt_ms(r.gpu_ms),
                opt_ms(r.gpu_p95_ms),
                opt_ms(r.cpu_ms)
            );
        }
    }

    /// Millisecond `Sum` counters: per-frame stage costs.
    fn costs_md(&self, out: &mut String) {
        let mut rows: Vec<_> = self
            .counters
            .iter()
            .filter(|(_, c)| c.kind == CounterKind::Sum && c.unit == unit::MS)
            .filter_map(|(name, c)| Some((name, c.per_frame?, c.total.unwrap_or(0.0))))
            .collect();
        if rows.is_empty() {
            return;
        }
        rows.sort_by(|a, b| b.1.mean.total_cmp(&a.1.mean));
        let _ = writeln!(out, "\n## Per-frame costs (ms)\n");
        let _ = writeln!(out, "| counter | mean | p95 | max | total |");
        let _ = writeln!(out, "|---|---:|---:|---:|---:|");
        for (name, s, total) in rows {
            let _ = writeln!(
                out,
                "| {name} | {:.3} | {:.3} | {:.2} | {:.1} |",
                s.mean, s.p95, s.max, total
            );
        }
    }

    /// Gauges other than render timings: counts, queue depths, sizes.
    fn gauges_md(&self, out: &mut String) {
        let rows: Vec<_> = self
            .counters
            .iter()
            .filter(|(name, c)| c.kind == CounterKind::Gauge && !name.starts_with(RENDER_PREFIX))
            .filter_map(|(name, c)| Some((name, c.per_frame?, c.last, &c.unit)))
            .collect();
        if rows.is_empty() {
            return;
        }
        let _ = writeln!(out, "\n## Gauges (counts, sizes)\n");
        let _ = writeln!(out, "| gauge | unit | median | max | last |");
        let _ = writeln!(out, "|---|---|---:|---:|---:|");
        for (name, s, last, unit) in rows {
            let _ = writeln!(
                out,
                "| {name} | {unit} | {} | {} | {} |",
                big(s.median),
                big(s.max),
                last.map_or("—".to_owned(), big)
            );
        }
    }

    /// Non-millisecond `Sum` counters: items, bytes, events.
    fn churn_md(&self, out: &mut String) {
        let rows: Vec<_> = self
            .counters
            .iter()
            .filter(|(_, c)| c.kind == CounterKind::Sum && c.unit != unit::MS)
            .filter_map(|(name, c)| Some((name, c.per_frame?, c.total.unwrap_or(0.0), &c.unit)))
            .collect();
        if rows.is_empty() {
            return;
        }
        let _ = writeln!(out, "\n## Churn (per frame)\n");
        let _ = writeln!(out, "| counter | unit | mean | max | total |");
        let _ = writeln!(out, "|---|---|---:|---:|---:|");
        for (name, s, total, unit) in rows {
            let _ = writeln!(
                out,
                "| {name} | {unit} | {} | {} | {} |",
                big(s.mean),
                big(s.max),
                big(total)
            );
        }
    }

    fn hitches_md(&self, out: &mut String) {
        let Some(h) = &self.hitches else { return };
        let _ = writeln!(
            out,
            "\n## Hitches (> {:.1} ms; slowest {}, time order; churn = this frame + the one before)\n",
            h.threshold_ms,
            h.rows.len()
        );
        if h.rows.is_empty() {
            let _ = writeln!(out, "(none)");
            return;
        }
        let _ = writeln!(out, "| phase | t s | frame ms | churn |");
        let _ = writeln!(out, "|---|---:|---:|---|");
        for row in &h.rows {
            let churn: Vec<String> = row
                .churn
                .iter()
                .map(|c| format!("{} {}", c.counter, big(c.value)))
                .collect();
            let _ = writeln!(
                out,
                "| {} | {:.2} | {:.1} | {} |",
                match row.phase {
                    RunPhase::Warmup => "warm-up",
                    RunPhase::Measure => "measure",
                },
                row.t_s,
                row.frame_ms,
                churn.join(", ")
            );
        }
    }
}

fn opt_ms(v: Option<f64>) -> String {
    v.map_or("—".to_owned(), |v| format!("{v:.3}"))
}

/// Compact number: `1.23 M`, `45.6 k`, `789`, `0.25`.
pub(crate) fn big(v: f64) -> String {
    let a = v.abs();
    if a >= 1.0e9 {
        format!("{:.2} G", v / 1.0e9)
    } else if a >= 1.0e6 {
        format!("{:.2} M", v / 1.0e6)
    } else if a >= 1.0e4 {
        format!("{:.1} k", v / 1.0e3)
    } else if v.fract() == 0.0 || a >= 100.0 {
        format!("{v:.0}")
    } else {
        format!("{v:.2}")
    }
}
