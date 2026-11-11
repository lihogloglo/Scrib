//! Pipeline warm-up measurement (pipeline-warmup.md §6, slice W0): what a reveal shows late.
//!
//! A *reveal* is the moment new content becomes visible after a wait: a door fade opening, or
//! `Loading` → `InGame`. Its emitter writes a [`Reveal`] (how long it held, the terms it waited
//! on); [`RevealWatchPlugin`] logs one `reveal:` line for it and then, for
//! [`AFTER_REVEAL_SECS`], counts what still arrives late (UE's PSO precache "too late" stats,
//! Unity's pipeline trace): render pipelines created (`pipelines/after_reveal`, target 0) and
//! visible draws skipped for a pipeline (`draws/skipped_late`). The window ends with one summary
//! line. The GPU side is sampled in the render world into [`PendingDraws`] (feature `pipelines`,
//! `PipelineWatchPlugin`); this module is the arithmetic and the main-world half, GPU-free.
//!
//! [`CompileStats`] is the per-pipeline compile-time histogram (front end vs driver, timed in the
//! vendored `bevy_render` `pipeline_cache.rs`, marker `own: pipeline compile timing`), and
//! [`describe_key_change`] names the bits of a view-key flip (every material on that view then
//! re-specializes: a burst of new pipelines).

use std::fmt::Write as _;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Real, Time};

use crate::{DiagCounters, GpuPending};

/// How long after a reveal new pipelines and skipped draws count as late, seconds.
pub const AFTER_REVEAL_SECS: f64 = 10.0;

/// One frame's GPU readiness, as the render world saw it (one frame behind the main world).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PendingSample {
    /// Visible main-pass draws skipped: their pipeline is queued or compiling (cause A/B).
    pub skipped: usize,
    /// Visible main-pass draws that have a pipeline (drawn or skipped): the share's denominator.
    pub visible: usize,
    /// Visible meshes with a material but no pipeline yet: mesh or material not prepared (cause C).
    pub unprepared: usize,
    /// Render assets waiting for a later frame (meshes, images and the counted material types'
    /// retry queues: upload budget, dependencies). Global, for the log; the gate uses `near_unprepared`.
    pub assets: usize,
    /// Pipelines queued or compiling.
    pub waiting: usize,
    /// Pipelines requested since start (monotonic).
    pub created: usize,
    /// Shadow-view and prepass draws whose pipeline is queued or compiling (W1; only while a
    /// [`RevealProbe`] is set).
    pub skipped_shadow: usize,
    /// Resident, shown meshes within the probe's radius whose mesh or material (with its
    /// textures) is not prepared in the render world (W1; only while a [`RevealProbe`] is set).
    pub near_unprepared: usize,
}

/// Where a reveal looks for unprepared assets: resident meshes within `radius` of `center`
/// (world space). Set by the reveal gate while it holds; the render world reads it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RevealProbe {
    pub center: [f32; 3],
    pub radius: f32,
}

const SLOTS: usize = 8;

#[derive(Default)]
struct Shared {
    slots: [AtomicUsize; SLOTS],
    probe: Mutex<Option<RevealProbe>>,
    examples: Mutex<Vec<Entity>>,
}

/// The render world's latest [`PendingSample`], shared with the main world (one `Arc`, a frame
/// late, which a fade's hold absorbs), and the main world's [`RevealProbe`] for the render
/// world. Filled by `PendingDrawsPlugin` (feature `pipelines`); read by the reveal gate.
#[derive(Resource, Clone, Default)]
pub struct PendingDraws(Arc<Shared>);

impl PendingDraws {
    /// Visible draws skipped for a pipeline still compiling.
    pub fn get(&self) -> usize {
        self.0.slots[0].load(Ordering::Relaxed)
    }

    /// Pipelines still compiling and the visible draws they hold back.
    pub fn gpu(&self) -> GpuPending {
        let s = self.sample();
        GpuPending { pipelines: s.waiting, draws: s.skipped }
    }

    pub fn sample(&self) -> PendingSample {
        let [skipped, visible, unprepared, assets, waiting, created, skipped_shadow, near_unprepared] =
            self.0.slots.each_ref().map(|a| a.load(Ordering::Relaxed));
        PendingSample { skipped, visible, unprepared, assets, waiting, created, skipped_shadow, near_unprepared }
    }

    pub fn store(&self, s: PendingSample) {
        let values =
            [s.skipped, s.visible, s.unprepared, s.assets, s.waiting, s.created, s.skipped_shadow, s.near_unprepared];
        for (slot, v) in self.0.slots.iter().zip(values) {
            slot.store(v, Ordering::Relaxed);
        }
    }

    /// Sets (or clears) where the render world counts unprepared assets and shadow draws.
    pub fn set_probe(&self, probe: Option<RevealProbe>) {
        *self.0.probe.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = probe;
    }

    pub fn probe(&self) -> Option<RevealProbe> {
        *self.0.probe.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// A few of the main-world meshes the last sample counted in `near_unprepared` (for a warning).
    pub fn examples(&self) -> Vec<Entity> {
        self.0.examples.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn set_examples(&self, examples: Vec<Entity>) {
        *self.0.examples.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = examples;
    }
}

/// A wait the reveal measured (`secs: None`: not measured by this emitter, logged as `n/a`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RevealTerm {
    pub name: &'static str,
    pub secs: Option<f32>,
}

/// New content became visible after a wait (see the module docs). Written by the emitter (the
/// door fade, the app's `Loading` → `InGame`), read by [`RevealWatchPlugin`].
#[derive(Message, Clone, Debug, PartialEq)]
pub struct Reveal {
    /// `door`, `loading`, ...
    pub kind: &'static str,
    /// Where (worldspace or cell name), for the log.
    pub place: String,
    /// Seconds the screen stayed covered.
    pub held_secs: f32,
    /// What the hold waited on, in order.
    pub terms: Vec<RevealTerm>,
    /// The cap that ended the hold, if one did (`None`: the content was ready).
    pub capped: Option<&'static str>,
}

impl Reveal {
    /// `reveal: door held 1.20 s (cells 0.40 s, assets 0.10 s), capped: none, "Place"; at reveal: ...`
    pub fn line(&self, at: &PendingSample) -> String {
        let mut s = format!("reveal: {} held {:.2} s (", self.kind, self.held_secs);
        for (i, t) in self.terms.iter().enumerate() {
            let sep = if i == 0 { "" } else { ", " };
            match t.secs {
                Some(secs) => {
                    let _ = write!(s, "{sep}{} {secs:.2} s", t.name);
                }
                None => {
                    let _ = write!(s, "{sep}{} n/a", t.name);
                }
            }
        }
        let _ = write!(
            s,
            "), capped: {}, \"{}\"; at reveal: {} pipelines compiling, {} visible draws skipped ({} shadow/prepass), {} meshes unprepared ({} near), {} assets pending",
            self.capped.unwrap_or("none"),
            self.place,
            at.waiting,
            at.skipped,
            at.skipped_shadow,
            at.unprepared,
            at.near_unprepared,
            at.assets,
        );
        s
    }
}

/// What arrived late after one reveal ([`AfterReveal`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RevealSummary {
    pub kind: &'static str,
    pub place: String,
    /// Seconds the window covered (short when the next reveal cut it).
    pub secs: f64,
    /// Pipelines created in the window (`pipelines/after_reveal`, target 0).
    pub pipelines: usize,
    /// Visible draws skipped in the window, summed over frames (`draws/skipped_late`).
    pub skipped: u64,
    /// Frames with at least one skipped draw, and the most in one frame.
    pub skipped_frames: u32,
    pub skipped_peak: usize,
    /// Visible draws summed over the window's frames (the share's denominator).
    pub visible: u64,
    /// Most meshes unprepared (cause C) in one frame.
    pub unprepared_peak: usize,
}

impl RevealSummary {
    /// Skipped draws as a share of visible draws, percent (exterior target ≤ 1 %).
    #[allow(clippy::cast_precision_loss)] // draw counts
    pub fn skipped_percent(&self) -> f64 {
        if self.visible == 0 { 0.0 } else { 100.0 * self.skipped as f64 / self.visible as f64 }
    }

    pub fn line(&self) -> String {
        format!(
            "reveal: {} +{:.1} s: after_reveal {} pipelines, skipped_late {} draws in {} frames (peak {}/frame, {:.2} % of visible), unprepared peak {}, \"{}\"",
            self.kind,
            self.secs,
            self.pipelines,
            self.skipped,
            self.skipped_frames,
            self.skipped_peak,
            self.skipped_percent(),
            self.unprepared_peak,
            self.place,
        )
    }
}

/// This frame's late counts ([`AfterReveal::frame`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LateFrame {
    /// Pipelines created this frame inside a window.
    pub pipelines: usize,
    /// Visible draws skipped this frame inside a window.
    pub skipped: usize,
    /// The window that ended this frame.
    pub closed: Option<RevealSummary>,
}

/// The after-reveal window (see the module docs). Pure: the caller passes the clock.
#[derive(Debug, Default)]
pub struct AfterReveal {
    open: Option<(f64, RevealSummary)>,
    last_created: Option<usize>,
}

impl AfterReveal {
    /// Opens a window at `now`; returns the previous one if it was still open (cut short).
    pub fn reveal(&mut self, now: f64, kind: &'static str, place: &str) -> Option<RevealSummary> {
        let previous = self.open.take().map(|(at, mut s)| {
            s.secs = now - at;
            s
        });
        self.open = Some((now, RevealSummary { kind, place: place.to_owned(), ..RevealSummary::default() }));
        previous
    }

    pub fn is_open(&self) -> bool {
        self.open.is_some()
    }

    /// Feeds one frame's sample; counts it if a window is open and not past
    /// [`AFTER_REVEAL_SECS`], else closes the window.
    pub fn frame(&mut self, now: f64, sample: &PendingSample) -> LateFrame {
        let new = sample.created.saturating_sub(self.last_created.unwrap_or(sample.created));
        self.last_created = Some(sample.created);
        let mut late = LateFrame::default();
        let Some((at, s)) = self.open.as_mut() else { return late };
        if now - *at > AFTER_REVEAL_SECS {
            let (at, mut s) = self.open.take().expect("open");
            s.secs = now - at;
            late.closed = Some(s);
            return late;
        }
        s.pipelines += new;
        s.skipped += sample.skipped as u64;
        if sample.skipped > 0 {
            s.skipped_frames += 1;
        }
        s.skipped_peak = s.skipped_peak.max(sample.skipped);
        s.visible += sample.visible as u64;
        s.unprepared_peak = s.unprepared_peak.max(sample.unprepared);
        late.pipelines = new;
        late.skipped = sample.skipped;
        late
    }
}

/// Upper edges of the compile-time histogram's buckets, ms (one more bucket past the last).
pub const COMPILE_BUCKETS_MS: [f64; 10] = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0];

/// Per-pipeline compile times (front end + driver), bucketed, with the totals of each part.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompileStats {
    pub count: usize,
    /// Pipelines per [`COMPILE_BUCKETS_MS`] bucket (`< edge`), the last one `≥ 512 ms`.
    pub buckets: [usize; 11],
    /// Sums, ms: queue + lock wait, front end (shader composition), driver (pipeline creation).
    pub wait_ms: f64,
    pub front_ms: f64,
    pub driver_ms: f64,
    /// The slowest pipeline (front + driver): label, ms.
    pub slowest: Option<(String, f64)>,
}

impl CompileStats {
    pub fn record(&mut self, label: &str, wait_ms: f64, front_ms: f64, driver_ms: f64) {
        let ms = front_ms + driver_ms;
        let bucket = COMPILE_BUCKETS_MS.iter().position(|edge| ms < *edge).unwrap_or(COMPILE_BUCKETS_MS.len());
        self.buckets[bucket] += 1;
        self.count += 1;
        self.wait_ms += wait_ms;
        self.front_ms += front_ms;
        self.driver_ms += driver_ms;
        if self.slowest.as_ref().is_none_or(|(_, max)| ms > *max) {
            self.slowest = Some((label.to_owned(), ms));
        }
    }

    /// The bucket edge under which `p` (0..=1) of the pipelines compiled, ms (the slowest
    /// pipeline's time for the open-ended bucket; 0 when empty).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)] // counts
    pub fn percentile_ms(&self, p: f64) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        let want = ((p * self.count as f64).ceil() as usize).clamp(1, self.count);
        let mut seen = 0;
        for (i, n) in self.buckets.iter().enumerate() {
            seen += n;
            if seen >= want {
                return COMPILE_BUCKETS_MS.get(i).copied().unwrap_or_else(|| self.slowest.as_ref().map_or(0.0, |s| s.1));
            }
        }
        0.0
    }

    /// `compiled 3 (front 0.01 s, driver 0.05 s, waiting 0.00 s; p50 <16 ms, p95 <64 ms, slowest 40 ms "x"; ms <16:2 <64:1)`
    pub fn line(&self) -> String {
        let mut s = format!(
            "compiled {} (front {:.2} s, driver {:.2} s, waiting {:.2} s; p50 <{} ms, p95 <{} ms",
            self.count,
            self.front_ms / 1000.0,
            self.driver_ms / 1000.0,
            self.wait_ms / 1000.0,
            self.percentile_ms(0.5),
            self.percentile_ms(0.95),
        );
        if let Some((label, ms)) = &self.slowest {
            let _ = write!(s, ", slowest {ms:.0} ms \"{label}\"");
        }
        s.push_str("; ms");
        for (i, n) in self.buckets.iter().enumerate().filter(|(_, n)| **n > 0) {
            match COMPILE_BUCKETS_MS.get(i) {
                Some(edge) => {
                    let _ = write!(s, " <{edge}:{n}");
                }
                None => {
                    let _ = write!(s, " >=512:{n}");
                }
            }
        }
        s.push(')');
        s
    }
}

/// Names a view-key flip: `+FLAG` / `-FLAG` for single-bit flags, `FIELD a->b` for multi-bit
/// fields (`fields`: name and mask), `bits 0x..` for changed bits neither names. `flags` holds
/// single-bit names (bits inside a field are skipped).
pub fn describe_key_change(old: u64, new: u64, flags: &[(&str, u64)], fields: &[(&str, u64)]) -> String {
    let changed = old ^ new;
    let mut parts: Vec<String> = Vec::new();
    let mut named = 0u64;
    for (name, mask) in fields {
        named |= mask;
        if changed & mask != 0 {
            let shift = mask.trailing_zeros();
            parts.push(format!("{name} {}->{}", (old & mask) >> shift, (new & mask) >> shift));
        }
    }
    for (name, bit) in flags {
        if bit & named != 0 || changed & bit == 0 {
            continue;
        }
        named |= bit;
        parts.push(format!("{}{name}", if new & bit != 0 { '+' } else { '-' }));
    }
    let rest = changed & !named;
    if rest != 0 {
        parts.push(format!("bits {rest:#x}"));
    }
    if parts.is_empty() { "none".to_owned() } else { parts.join(" ") }
}

/// The main-world half (see the module docs): logs each [`Reveal`] with the GPU state at that
/// moment, counts the late work in the window after it (counters `pipelines/after_reveal` and
/// `draws/skipped_late` when [`DiagCounters`] exist) and logs the window's summary. Reads the
/// [`PendingDraws`] that `PipelineWatchPlugin` (feature `pipelines`) fills; without it every
/// sample is zero and only the `reveal:` lines remain.
#[derive(Default)]
pub struct RevealWatchPlugin;

impl Plugin for RevealWatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Reveal>().init_resource::<PendingDraws>().add_systems(PostUpdate, watch_reveals);
    }
}

#[allow(clippy::needless_pass_by_value, clippy::cast_precision_loss)] // system parameters; counts
fn watch_reveals(
    mut reveals: MessageReader<Reveal>,
    pending: Res<PendingDraws>,
    time: Res<Time<Real>>,
    counters: Option<Res<DiagCounters>>,
    mut state: Local<AfterReveal>,
) {
    let now = time.elapsed_secs_f64();
    let sample = pending.sample();
    for reveal in reveals.read() {
        log::info!("{}", reveal.line(&sample));
        if let Some(cut) = state.reveal(now, reveal.kind, &reveal.place) {
            log::info!("{} (cut short by the next reveal)", cut.line());
        }
    }
    let late = state.frame(now, &sample);
    if let Some(closed) = &late.closed {
        log::info!("{}", closed.line());
    }
    if let Some(counters) = counters
        && state.is_open()
    {
        counters.add_named("pipelines/after_reveal", sf_diag::unit::COUNT, late.pipelines as f64);
        counters.add_named("draws/skipped_late", sf_diag::unit::COUNT, late.skipped as f64);
    }
}
