//! The perf panel's pure half ([`crate::perf_panel`], ROADMAP 1.25 CW-17): rolling windows over
//! the sampled values, the bound-by pick, top-N, and the text. No Bevy, so every rule has an exact
//! test (`perf_panel_tests.rs`).

use std::collections::{HashMap, VecDeque};
use std::fmt::Write as _;

/// One value's samples over the last `window` seconds: `(time s, value)`, oldest first.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Series {
    samples: VecDeque<(f64, f64)>,
}

impl Series {
    /// Adds a sample at `t` and drops the ones older than `t - window`.
    pub fn push(&mut self, t: f64, value: f64, window: f64) {
        self.samples.push_back((t, value));
        while self.samples.front().is_some_and(|(at, _)| *at < t - window) {
            self.samples.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// The smoothed value: the mean over the window.
    #[allow(clippy::cast_precision_loss, reason = "a window holds far fewer than 2^52 samples")]
    pub fn mean(&self) -> Option<f64> {
        (!self.samples.is_empty()).then(|| self.samples.iter().map(|(_, v)| v).sum::<f64>() / self.samples.len() as f64)
    }

    pub fn max(&self) -> Option<f64> {
        self.samples.iter().map(|(_, v)| *v).reduce(f64::max)
    }

    /// Nearest-rank percentile, `p` in 0..=100: the smallest sample with at least `p` % of the
    /// samples at or below it (p99 of 120 frames is the second-largest).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "sample counts far below 2^52; the rank is clamped to 1..=len")]
    pub fn percentile(&self, p: f64) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted: Vec<f64> = self.samples.iter().map(|(_, v)| *v).collect();
        sorted.sort_by(f64::total_cmp);
        let n = sorted.len();
        let rank = ((p.clamp(0.0, 100.0) / 100.0) * n as f64).ceil() as usize;
        Some(sorted[rank.clamp(1, n) - 1])
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

/// Every sampled value by name (counter name or diagnostic path).
#[derive(Clone, Debug, Default)]
pub struct Windows {
    series: HashMap<String, Series>,
}

impl Windows {
    /// Adds a sample (allocates the name only the first time it is seen).
    pub fn push(&mut self, name: &str, t: f64, value: f64, window: f64) {
        if let Some(s) = self.series.get_mut(name) {
            s.push(t, value, window);
        } else {
            let mut s = Series::default();
            s.push(t, value, window);
            self.series.insert(name.to_owned(), s);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Series> {
        self.series.get(name).filter(|s| !s.is_empty())
    }

    pub fn mean(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(Series::mean)
    }

    /// `(name, mean)` of every non-empty series whose name `keep` accepts, in no order.
    pub fn means<'a>(&'a self, keep: impl Fn(&str) -> bool + 'a) -> impl Iterator<Item = (&'a str, f64)> + 'a {
        self.series.iter().filter(move |(n, _)| keep(n)).filter_map(|(n, s)| s.mean().map(|m| (n.as_str(), m)))
    }

    pub fn clear(&mut self) {
        self.series.clear();
    }
}

/// What the frame waits on: the busiest of the main thread, the render thread and the GPU, or
/// none of them when the busiest is well under the frame (vsync, a frame cap, the compositor,
/// or untimed GPU work).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bound {
    Main,
    Render,
    Gpu,
    /// No measured busy time comes close to the frame.
    Wait,
    /// Nothing measured.
    Unknown,
}

/// The bound-by pick (UE `stat unit`: the thread whose time matches the frame). The busiest of
/// `gpu`, `render`, `main` (ties in that order) is the bound when it is at least `ratio` of
/// `frame`; else [`Bound::Wait`]. [`Bound::Unknown`] with no frame or no busy time.
pub fn bound_by(frame: Option<f64>, main: Option<f64>, render: Option<f64>, gpu: Option<f64>, ratio: f64) -> Bound {
    let Some(frame) = frame.filter(|f| *f > 0.0) else { return Bound::Unknown };
    let mut best: Option<(Bound, f64)> = None;
    for (bound, value) in [(Bound::Gpu, gpu), (Bound::Render, render), (Bound::Main, main)] {
        if let Some(v) = value.filter(|v| v.is_finite())
            && best.is_none_or(|(_, b)| v > b)
        {
            best = Some((bound, v));
        }
    }
    match best {
        None => Bound::Unknown,
        Some((bound, v)) if v >= ratio * frame => bound,
        Some(_) => Bound::Wait,
    }
}

/// The `n` largest `(name, value)`, largest first, ties by name.
pub fn top_n<'a>(items: impl IntoIterator<Item = (&'a str, f64)>, n: usize) -> Vec<(&'a str, f64)> {
    let mut all: Vec<(&str, f64)> = items.into_iter().filter(|(_, v)| v.is_finite()).collect();
    all.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    all.truncate(n);
    all
}

/// Bevy's GPU pass timings: `render/<span path>/elapsed_gpu`.
pub const GPU_PREFIX: &str = "render/";
pub const GPU_SUFFIX: &str = "/elapsed_gpu";

/// A GPU span's path without `render/` and `/elapsed_gpu`, if it is one.
pub fn gpu_span(path: &str) -> Option<&str> {
    path.strip_prefix(GPU_PREFIX)?.strip_suffix(GPU_SUFFIX)
}

/// The GPU frame: the sum of the top-level timed spans (nested spans are inside their parent).
/// Partial: untimed passes are missing (as `sf_diag::pacing::GPU_TIMED`).
pub fn gpu_frame<'a>(spans: impl IntoIterator<Item = (&'a str, f64)>) -> Option<f64> {
    spans.into_iter().filter(|(span, _)| !span.contains('/')).map(|(_, v)| v).reduce(|a, b| a + b)
}

/// How the panel is shown (F3 cycles).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelMode {
    #[default]
    Off,
    /// Frame, threads, GPU and the bound.
    Compact,
    /// Plus GPU passes, views, lights, encoders, entities, preset and tunables.
    Full,
}

impl PanelMode {
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Off => Self::Compact,
            Self::Compact => Self::Full,
            Self::Full => Self::Off,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Compact => "compact",
            Self::Full => "full",
        }
    }
}

/// Views drawn this window (means): `sf_diag_bevy::RenderViewsPlugin`'s `views/*` gauges.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ViewsLine {
    pub main: f64,
    pub cascades: f64,
    pub lamp_faces: f64,
    /// Point/spot shadow faces culled for no visible receiver (`views/lamp_face/culled`).
    pub lamp_culled: Option<f64>,
    /// Queued phase items and batch sets over every view.
    pub items: f64,
    pub batch_sets: f64,
}

/// Encode and submit costs (means per frame): `sf_diag_bevy`'s `encode/*` counters (CW-5b).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EncodeLine {
    pub encoders: f64,
    pub command_buffers: Option<f64>,
    pub submits: Option<f64>,
    pub submit_ms: Option<f64>,
    pub poll_ms: Option<f64>,
}

/// One refresh's numbers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub frame_ms: Option<f64>,
    pub frame_p99: Option<f64>,
    pub frame_max: Option<f64>,
    pub main_ms: Option<f64>,
    pub render_ms: Option<f64>,
    pub gpu_ms: Option<f64>,
    pub bound: Option<Bound>,
    /// Why main/render are missing (gauges off by tunable, or another owner), if they are.
    pub cpu_note: Option<&'static str>,
    /// Why GPU is missing, if it is.
    pub gpu_note: Option<&'static str>,
    pub main_stages: Vec<(String, f64)>,
    pub gpu_passes: Vec<(String, f64)>,
    pub views: Option<ViewsLine>,
    /// Point and spot lights: total, with shadows on.
    pub lights: (usize, usize),
    pub encode: Option<EncodeLine>,
    pub entities: usize,
    pub preset: Option<String>,
    /// Every tunable set above the preset (user, command line, code, console): name, value, layer.
    pub tunables: Vec<(String, String, &'static str)>,
    /// Non-default tunables left out (over the cap).
    pub tunables_more: usize,
}

fn ms(v: Option<f64>) -> String {
    v.map_or_else(|| "n/a".to_owned(), |v| format!("{v:.2}"))
}

fn num(v: f64) -> String {
    format!("{v:.0}")
}

impl Bound {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Main => "MAIN thread",
            Self::Render => "RENDER thread",
            Self::Gpu => "GPU",
            Self::Wait => "none (vsync / cap / untimed GPU)",
            Self::Unknown => "n/a",
        }
    }
}

/// The panel's text (`full`: every line; else the compact lines).
pub fn format(s: &Snapshot, full: bool) -> String {
    let mut out = String::new();
    let fps = s.frame_ms.filter(|f| *f > 0.0).map_or_else(|| "--".to_owned(), |f| format!("{:.0}", 1000.0 / f));
    let _ = writeln!(out, "frame {} ms ({fps} fps)  p99 {}  max {}", ms(s.frame_ms), ms(s.frame_p99), ms(s.frame_max));
    let _ = write!(out, "main {}  render {}  GPU {}", ms(s.main_ms), ms(s.render_ms), ms(s.gpu_ms));
    for note in [s.cpu_note, s.gpu_note].into_iter().flatten() {
        let _ = write!(out, "  ({note})");
    }
    let _ = write!(out, "\nbound by: {}", s.bound.map_or("n/a", Bound::label));
    if !full {
        return out;
    }
    if !s.main_stages.is_empty() {
        let stages: Vec<String> = s.main_stages.iter().map(|(n, v)| format!("{n} {v:.2}")).collect();
        let _ = write!(out, "\nmain stages: {}", stages.join("  "));
    }
    if s.gpu_passes.is_empty() {
        let _ = write!(out, "\nGPU passes: n/a");
    } else {
        let _ = write!(out, "\nGPU passes (ms):");
        for (name, v) in &s.gpu_passes {
            let _ = write!(out, "\n  {v:6.2}  {name}");
        }
    }
    match &s.views {
        Some(v) => {
            let culled = v.lamp_culled.map_or_else(String::new, |c| format!(" (+{} culled)", num(c)));
            let _ = write!(
                out,
                "\nviews: camera {}  cascades {}  lamp faces {}{culled}  items {}  batch sets {}",
                num(v.main),
                num(v.cascades),
                num(v.lamp_faces),
                num(v.items),
                num(v.batch_sets)
            );
        }
        None => {
            let _ = write!(out, "\nviews: n/a");
        }
    }
    let _ = write!(out, "\nlights (point+spot): {} total, {} shadowed", s.lights.0, s.lights.1);
    match &s.encode {
        Some(e) => {
            let opt = |v: Option<f64>| v.map_or_else(|| "n/a".to_owned(), num);
            let poll = e.poll_ms.map_or_else(String::new, |p| format!("  poll {p:.2} ms"));
            let _ = write!(
                out,
                "\nencoders {}  command buffers {}  submits {} ({} ms){poll}",
                num(e.encoders),
                opt(e.command_buffers),
                opt(e.submits),
                ms(e.submit_ms)
            );
        }
        None => {
            let _ = write!(out, "\nencoders: n/a");
        }
    }
    let _ = write!(out, "\nentities {}  streaming: n/a", s.entities);
    let _ = write!(out, "\npreset: {}", s.preset.as_deref().unwrap_or("n/a (no sf.graphics_preset)"));
    if s.tunables.is_empty() {
        let _ = write!(out, "\ntunables: all default");
    } else {
        let _ = write!(out, "\ntunables (above the preset):");
        for (name, value, layer) in &s.tunables {
            let _ = write!(out, "\n  {name} = {value} [{layer}]");
        }
        if s.tunables_more > 0 {
            let _ = write!(out, "\n  ... {} more", s.tunables_more);
        }
    }
    out
}
