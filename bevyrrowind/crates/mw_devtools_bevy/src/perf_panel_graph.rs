//! The perf panel's graphs, pure half (ROADMAP 1.25 CW-17b): the per-frame history ring, the
//! per-frame bound-by pick, the column mapping (spikes kept: a column shows the worst frame it
//! covers), bar scaling, and the RGBA rasterizer the Bevy half copies into an `Image`. No Bevy, so
//! every rule has an exact test (`perf_panel_graph_tests.rs`).
//!
//! Industry: UE `stat unitgraph` (rolling frame / game / draw / GPU lines, a budget line), RTSS and
//! `PresentMon` / `FrameView` frame-time graphs (one sample per frame, hitches stand out), `OpenMW`'s F3
//! bars. Bevy 0.20's `bevy_dev_tools` frame-time graph draws a storage buffer of frame times in a
//! `UiMaterial` shader; ours is a CPU-written image (one `write_texture` of a few hundred KB per
//! redraw: Bevy reuses the texture when the size stays), so it needs no shader, colours each frame
//! by its bound and draws the thread lines, and is testable to the pixel.

use std::collections::VecDeque;

use crate::perf_panel_model::{Bound, bound_by};

/// One frame's times (ms) as the panel saw them. The thread and GPU numbers come from the gauges
/// (`main/busy/ms` of the last frame, `render/busy/ms` one frame behind, the GPU a few frames
/// behind), so a spike can sit a column or two apart between lines.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FrameSample {
    pub frame_ms: f32,
    pub main_ms: Option<f32>,
    pub render_ms: Option<f32>,
    pub gpu_ms: Option<f32>,
    /// `encode/submit` (CW-5b), for the full panel's sparkline.
    pub submit_ms: Option<f32>,
    /// Views drawn (camera + cascades + lamp faces), for the full panel's sparkline.
    pub views: Option<f32>,
}

impl FrameSample {
    /// What bound this frame ([`bound_by`] on its own numbers).
    pub fn bound(&self, ratio: f32) -> Bound {
        let f = |v: Option<f32>| v.map(f64::from);
        bound_by(Some(f64::from(self.frame_ms)), f(self.main_ms), f(self.render_ms), f(self.gpu_ms), f64::from(ratio))
    }
}

/// The last `capacity` frames, oldest first.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct History {
    samples: VecDeque<FrameSample>,
    capacity: usize,
}

impl History {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { samples: VecDeque::with_capacity(capacity), capacity }
    }

    /// Adds the newest frame, dropping the oldest once full.
    pub fn push(&mut self, sample: FrameSample) {
        if self.capacity == 0 {
            return;
        }
        while self.samples.len() >= self.capacity {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }

    /// A new capacity (live tunable): the oldest frames go when it shrinks.
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
        while self.samples.len() > capacity {
            self.samples.pop_front();
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &FrameSample> {
        self.samples.iter()
    }

    /// Wall time the history spans (s): the sum of its frame times.
    pub fn span_s(&self) -> f32 {
        self.samples.iter().map(|s| s.frame_ms).sum::<f32>() / 1000.0
    }

    /// The frames of slots `start..end` of the graph's `capacity` slots (slot 0 the oldest; the
    /// history fills the newest, right-hand slots).
    fn slots(&self, start: usize, end: usize) -> impl Iterator<Item = &FrameSample> {
        let first = self.capacity - self.samples.len();
        let (a, b) = (start.max(first) - first, end.max(first) - first);
        self.samples.range(a.min(self.samples.len())..b.min(self.samples.len()))
    }
}

/// The slots `[start, end)` column `column` of `width` covers, out of `capacity` (oldest left).
/// A history wider than the graph puts several frames in a column; a narrower one repeats a frame
/// over several columns. Never empty while `column < width` and `capacity > 0`.
pub fn column_slots(column: usize, width: usize, capacity: usize) -> (usize, usize) {
    if width == 0 || capacity == 0 {
        return (0, 0);
    }
    let start = column * capacity / width;
    let end = ((column + 1) * capacity / width).max(start + 1).min(capacity);
    (start, end)
}

/// One graph column: the worst frame it covers (so a one-frame hitch never disappears) with what
/// bound that frame, and each line's worst value over the column.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Column {
    pub frame_ms: f32,
    pub bound: Bound,
    pub main_ms: Option<f32>,
    pub render_ms: Option<f32>,
    pub gpu_ms: Option<f32>,
}

fn max_opt(a: Option<f32>, b: Option<f32>) -> Option<f32> {
    match (a.filter(|v| v.is_finite()), b.filter(|v| v.is_finite())) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (a, b) => a.or(b),
    }
}

/// The graph's `width` columns over `history` (`None`: no frame there yet).
pub fn columns(history: &History, width: usize, ratio: f32) -> Vec<Option<Column>> {
    (0..width)
        .map(|c| {
            let (start, end) = column_slots(c, width, history.capacity());
            let mut out: Option<(Column, f32)> = None;
            for s in history.slots(start, end) {
                let frame = if s.frame_ms.is_finite() { s.frame_ms } else { 0.0 };
                let col = out.get_or_insert((
                    Column { frame_ms: frame, bound: s.bound(ratio), main_ms: None, render_ms: None, gpu_ms: None },
                    frame,
                ));
                if frame > col.1 {
                    *col = (Column { frame_ms: frame, bound: s.bound(ratio), ..col.0 }, frame);
                }
                col.0.main_ms = max_opt(col.0.main_ms, s.main_ms);
                col.0.render_ms = max_opt(col.0.render_ms, s.render_ms);
                col.0.gpu_ms = max_opt(col.0.gpu_ms, s.gpu_ms);
            }
            out.map(|(c, _)| c)
        })
        .collect()
}

/// One value per column (`pick` of each frame, the column's largest), for a sparkline.
pub fn series_columns(history: &History, width: usize, pick: impl Fn(&FrameSample) -> Option<f32>) -> Vec<Option<f32>> {
    (0..width)
        .map(|c| {
            let (start, end) = column_slots(c, width, history.capacity());
            history.slots(start, end).fold(None, |acc, s| max_opt(acc, pick(s)))
        })
        .collect()
}

/// Rows a value fills from the bottom of a `height`-row graph whose top is `top` (same unit), and
/// whether it was clipped (above the top: a spike off the scale). Non-finite or <= 0: nothing.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "clamped to 0..=height first")]
pub fn bar_rows(value: f32, top: f32, height: u32) -> (u32, bool) {
    if !value.is_finite() || value <= 0.0 || !top.is_finite() || top <= 0.0 {
        return (0, false);
    }
    let rows = (value / top * height as f32).round().clamp(0.0, height as f32) as u32;
    (rows, value > top)
}

/// A bar's filled share of its track: `value / full`, clamped to 0..=1 (0 for nothing to show).
pub fn bar_fraction(value: Option<f64>, full: Option<f64>) -> f32 {
    match (value, full) {
        #[allow(clippy::cast_possible_truncation, reason = "a share in 0..=1")]
        (Some(v), Some(f)) if v.is_finite() && f.is_finite() && f > 0.0 => (v / f).clamp(0.0, 1.0) as f32,
        _ => 0.0,
    }
}

/// An 8-bit sRGB colour with alpha.
pub type Rgba = [u8; 4];

/// sRGB 0..=1 floats (a `float3` tunable) to bytes.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 0..=255 first")]
pub fn rgba(rgb: [f32; 3], alpha: f32) -> Rgba {
    let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    [b(rgb[0]), b(rgb[1]), b(rgb[2]), b(alpha)]
}

/// `c` at `k` brightness (the frame columns sit under the lines).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a byte times 0..=1")]
pub fn dim(c: Rgba, k: f32) -> Rgba {
    let m = |v: u8| (f32::from(v) * k.clamp(0.0, 1.0)).round() as u8;
    [m(c[0]), m(c[1]), m(c[2]), c[3]]
}

/// The graph's colours.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub background: Rgba,
    pub main: Rgba,
    pub render: Rgba,
    pub gpu: Rgba,
    /// Frames bound by none of them (vsync, a cap, untimed GPU) or with nothing measured.
    pub wait: Rgba,
    /// The median target line (8.3 ms) and the p99 one (11 ms).
    pub target: Rgba,
    pub target_p99: Rgba,
    /// The mark on a spike above the scale.
    pub clip: Rgba,
}

impl Palette {
    pub const fn bound(&self, b: Bound) -> Rgba {
        match b {
            Bound::Main => self.main,
            Bound::Render => self.render,
            Bound::Gpu => self.gpu,
            Bound::Wait | Bound::Unknown => self.wait,
        }
    }
}

/// The frame graph's shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphSpec {
    pub width: u32,
    pub height: u32,
    /// The scale's top (ms); frames above it are clipped and marked.
    pub top_ms: f32,
    /// The median and p99 target lines (ms).
    pub targets: [f32; 2],
    /// [`crate::perf_panel_model::bound_by`]'s ratio.
    pub ratio: f32,
}

/// Brightness of a frame column under the lines.
// tunables: const (a drawing style: the column is the frame, the lines stay readable over it)
const COLUMN_DIM: f32 = 0.55;
/// Rows of the clip mark on a spike above the scale.
// tunables: const (a drawing style)
const CLIP_ROWS: u32 = 2;
/// Target lines: `on` pixels out of every `period` (dashed, so the columns show through).
// tunables: const (a drawing style)
const DASH: (u32, u32) = (4, 6);

struct Canvas<'a> {
    w: u32,
    h: u32,
    px: &'a mut Vec<u8>,
}

impl Canvas<'_> {
    fn set(&mut self, x: u32, y: u32, c: Rgba) {
        if x < self.w && y < self.h {
            let i = ((y * self.w + x) * 4) as usize;
            self.px[i..i + 4].copy_from_slice(&c);
        }
    }

    /// Row (from the top) of the top of a value's bar; `None` for nothing.
    fn row(&self, value: f32, top: f32) -> Option<u32> {
        let (rows, _) = bar_rows(value, top, self.h);
        (value.is_finite() && value > 0.0).then(|| self.h - rows.max(1))
    }
}

fn reset(out: &mut Vec<u8>, w: u32, h: u32, bg: Rgba) {
    out.clear();
    out.reserve((w * h * 4) as usize);
    for _ in 0..w * h {
        out.extend_from_slice(&bg);
    }
}

/// Picks one line's value from a column.
type LinePick = fn(&Column) -> Option<f32>;

/// Draws the frame graph into `out` (RGBA8 sRGB, `width * height * 4` bytes, row 0 at the top):
/// one column per [`columns`] entry filled to its frame time in its bound's colour (dimmed), the
/// clip mark on spikes above the scale, the two dashed target lines, then the main, render and GPU
/// lines (each column joined to the last, so a spike reads as a vertical stroke).
pub fn draw_graph(history: &History, spec: &GraphSpec, palette: &Palette, out: &mut Vec<u8>) {
    let (width, height) = (spec.width, spec.height);
    reset(out, width, height, palette.background);
    if width == 0 || height == 0 {
        return;
    }
    let cols = columns(history, width as usize, spec.ratio);
    let mut canvas = Canvas { w: width, h: height, px: out };
    for (x, col) in (0..width).zip(&cols) {
        let Some(col) = col else { continue };
        let (rows, clipped) = bar_rows(col.frame_ms, spec.top_ms, height);
        let fill = dim(palette.bound(col.bound), COLUMN_DIM);
        for y in height - rows..height {
            canvas.set(x, y, fill);
        }
        if clipped {
            for y in 0..CLIP_ROWS.min(height) {
                canvas.set(x, y, palette.clip);
            }
        }
    }
    for (target, colour) in spec.targets.iter().zip([palette.target, palette.target_p99]) {
        if let Some(y) = canvas.row(*target, spec.top_ms) {
            for x in (0..width).filter(|x| x % DASH.1 < DASH.0) {
                canvas.set(x, y, colour);
            }
        }
    }
    let lines: [(LinePick, Rgba); 3] =
        [(|col| col.gpu_ms, palette.gpu), (|col| col.render_ms, palette.render), (|col| col.main_ms, palette.main)];
    for (pick, colour) in lines {
        let mut last: Option<u32> = None;
        for (x, col) in (0..width).zip(&cols) {
            let row = col.as_ref().and_then(pick).and_then(|v| canvas.row(v, spec.top_ms));
            if let Some(row) = row {
                let (from, to) = last.map_or((row, row), |l| (l.min(row), l.max(row)));
                for y in from..=to {
                    canvas.set(x, y, colour);
                }
            }
            last = row;
        }
    }
}

/// Draws a sparkline into `out` (as [`draw_graph`]): each column's value as a dimmed area with a
/// bright top, scaled to the largest value shown. Returns that largest value (`None`: no data).
pub fn draw_sparkline(values: &[Option<f32>], width: u32, height: u32, colour: Rgba, background: Rgba, out: &mut Vec<u8>) -> Option<f32> {
    reset(out, width, height, background);
    let max = values.iter().flatten().copied().filter(|v| v.is_finite()).reduce(f32::max)?;
    if width == 0 || height == 0 || max <= 0.0 {
        return Some(max);
    }
    let mut c = Canvas { w: width, h: height, px: out };
    for (x, v) in (0..width).zip(values) {
        let Some(v) = v.filter(|v| v.is_finite()) else { continue };
        let (rows, _) = bar_rows(v, max, height);
        for y in height - rows..height {
            c.set(x, y, dim(colour, COLUMN_DIM));
        }
        if let Some(y) = c.row(v, max) {
            c.set(x, y, colour);
        }
    }
    Some(max)
}

/// The window height the panel's sizes are authored for.
// tunables: const (a layout reference: sizes are authored at 1080p, `diag.panel.scale` tunes them)
pub const AUTHORED_HEIGHT_PX: f32 = 1080.0;

/// The panel's scale: `(logical px, physical px)` per authored px. `setting` > 0 is the physical
/// size multiplier; 0 is auto: the window's physical height over 1080p, at least `min`, so the panel
/// covers the same share of a 720p or a 1440p screen yet stays readable on small ones. Bevy
/// multiplies logical px by the window's scale factor and `UiScale`: the first is divided out,
/// the second kept (the panel follows the game's UI scale).
#[allow(clippy::cast_precision_loss, reason = "a window height")]
pub fn panel_scale(setting: f32, min: f32, physical_height: u32, scale_factor: f32, ui_scale: f32) -> (f32, f32) {
    let overall = if setting > 0.0 { setting } else { (physical_height as f32 / AUTHORED_HEIGHT_PX).max(min) };
    let sf = if scale_factor > 0.0 { scale_factor } else { 1.0 };
    let ui = if ui_scale > 0.0 { ui_scale } else { 1.0 };
    (overall / sf, overall * ui)
}

#[cfg(test)]
#[path = "perf_panel_graph_tests.rs"]
mod tests;
