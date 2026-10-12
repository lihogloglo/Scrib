//! Frame pacing: work time vs delivery time (diag.md "Frame pacing").
//!
//! A frame's presented interval (`frame_ms`, present to present) is the
//! player's experience, but it mixes the engine's work with waiting on the
//! swap chain and the compositor. `PresentMon` splits a frame the same way
//! (CPU busy / CPU wait, GPU busy / GPU wait). The work time of a frame is
//! the longest busy stage: main-thread update, render-thread work without
//! its swap-chain waits, and GPU time. An app measures the stages (the
//! `sf_diag_bevy` `FramePhasePlugin` does); [`derive_work`] combines them.

use crate::counters::{CounterDef, CounterKind, unit};
use crate::recorder::Recorder;

/// Main-thread busy time: the main schedule, first system to last.
pub const MAIN_BUSY: &str = "main/busy/ms";
/// Render-thread busy time: the render schedule minus swap-chain acquire
/// and present (the two calls that wait on the GPU or the compositor).
pub const RENDER_BUSY: &str = "render/busy/ms";
/// Sum of the timed GPU passes (`*/elapsed_gpu`), each carried forward to
/// frames without a new measurement. Partial: untimed passes are missing.
pub const GPU_TIMED: &str = "gpu/timed/ms";
/// The frame's work time: max of [`MAIN_BUSY`], [`RENDER_BUSY`], [`GPU_TIMED`].
pub const WORK: &str = "frame/work/ms";
/// Suffix of Bevy's per-pass GPU timings.
const GPU_SUFFIX: &str = "/elapsed_gpu";

/// Adds the [`GPU_TIMED`] and [`WORK`] columns. Does nothing without a
/// busy-time column (an app that doesn't measure phases), so a work gate
/// then fails for missing data instead of passing on nothing.
pub fn derive_work(recorder: &mut Recorder) {
    let frames = recorder.frame_ms().len();
    let gpu: Vec<&[f64]> = recorder.series().iter().filter(|s| s.def().name.ends_with(GPU_SUFFIX)).map(crate::recorder::Series::values).collect();
    let gpu_timed = (!gpu.is_empty()).then(|| {
        let mut last = vec![f64::NAN; gpu.len()];
        (0..frames)
            .map(|row| {
                for (pass, values) in gpu.iter().enumerate() {
                    if values[row].is_finite() {
                        last[pass] = values[row];
                    }
                }
                let known: Vec<f64> = last.iter().copied().filter(|v| v.is_finite()).collect();
                if known.is_empty() { f64::NAN } else { known.iter().sum() }
            })
            .collect::<Vec<f64>>()
    });
    let column = |name: &str| recorder.get(name).map(|s| s.values().to_vec());
    let (main, render) = (column(MAIN_BUSY), column(RENDER_BUSY));
    if main.is_none() && render.is_none() {
        return;
    }
    let work: Vec<f64> = (0..frames)
        .map(|row| {
            [main.as_ref(), render.as_ref(), gpu_timed.as_ref()]
                .into_iter()
                .flatten()
                .map(|c| c[row])
                .filter(|v| v.is_finite())
                .fold(f64::NAN, f64::max)
        })
        .collect();
    let def = |name: &str| CounterDef { name: name.to_owned(), kind: CounterKind::Gauge, unit: unit::MS.to_owned() };
    if let Some(gpu_timed) = gpu_timed {
        recorder.set_derived(def(GPU_TIMED), gpu_timed);
    }
    recorder.set_derived(def(WORK), work);
}

/// The derived columns, left out of spike culprit lists (they are totals,
/// not causes).
pub const DERIVED: [&str; 2] = [WORK, GPU_TIMED];

/// Frames the stutter gate compares against: the previous 60 recorded frames
/// (0.5 s at 120 fps), so a slow change of scene (ocean to town) moves the
/// reference instead of counting as a spike.
pub const STUTTER_WINDOW: usize = 60;

/// Fewest earlier frames a stutter needs as reference; the first frames of a
/// run are never stutters.
const STUTTER_MIN_REFERENCE: usize = 8;

/// The stutter frames of a per-frame series: frames whose value is over
/// `factor` × the median of the previous `window` finite values (a moving
/// median, as stutter counters in `CapFrameX` and hitch detection in Unreal
/// compare a frame with its recent past, not with the whole run). NaN frames
/// are skipped. Returns the frame indices in order.
pub fn stutters(values: &[f64], factor: f64, window: usize) -> Vec<usize> {
    let window = window.max(1);
    let mut recent: std::collections::VecDeque<f64> = std::collections::VecDeque::with_capacity(window);
    let mut sorted = Vec::with_capacity(window);
    let mut found = Vec::new();
    for (frame, &value) in values.iter().enumerate() {
        if !value.is_finite() {
            continue;
        }
        if recent.len() >= STUTTER_MIN_REFERENCE.min(window) {
            sorted.clear();
            sorted.extend(recent.iter().copied());
            sorted.sort_by(f64::total_cmp);
            if value > factor * crate::stats::percentile_sorted(&sorted, 50.0) {
                found.push(frame);
            }
        }
        if recent.len() == window {
            recent.pop_front();
        }
        recent.push_back(value);
    }
    found
}
