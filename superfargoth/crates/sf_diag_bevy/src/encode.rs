//! Untraced encode costs (`docs/research/cpu-frame-audit-2026-11-15.md` M3), added by
//! [`crate::FramePhasePlugin`] with feature `render`. The vendored `bevy_render` times each
//! pending encoder's `finish()` and each `queue.submit` behind an opt-in hook
//! (`bevy_render::renderer::encode_timing`, marker `own, M3 encode timers`); this module installs
//! it and folds one render frame into counters, all `ms` unless noted:
//!
//! - `encode/<system>/{sum,max,count}`: per recording system (short name, generics and closures
//!   stripped): total and longest `finish()` this frame, and how many encoders (`count`).
//! - `encode/finish_critical`: the longest single `finish()` this frame (the critical path of the
//!   parallel finish); `encode/finish_wall`: the parallel finishes' wall time.
//! - `encode/submit`: every timed `queue.submit` this frame (the main pending-buffer submit and
//!   `render_system`'s readback submit).
//! - `encode/poll` (submit-cost CW-5b): the device poll just before the main submit, only when
//!   `set_poll_before_submit` is on (`docs/reviews/submit-cost-diagnosis-2026-11-16.md` cause 2:
//!   wgpu's deferred destruction then leaves `encode/submit`).
//! - `encode/queue/...` (`count` / `bytes`, CW-5b): every call through `RenderQueue` this frame,
//!   timed or not: `submits`, `command_buffers` (one per command encoder), and
//!   `write_buffer`, `write_buffer_with`, `write_texture` as `<op>/calls` + `<op>/bytes` (each
//!   call is one wgpu staging buffer). Calls on a bare `&wgpu::Queue` are not seen.
//!
//! The hook is process-wide (one renderer per process); not installed = the vendored code does one
//! atomic load per finish/submit and times nothing.

use std::collections::HashMap;
#[cfg(feature = "render")]
use std::sync::{LazyLock, Mutex, PoisonError};

use sf_diag::{Counters, unit};

/// One recording system's names and this frame's totals.
struct PerName {
    keys: [String; 3],
    sum: f64,
    max: f64,
    count: u32,
}

/// One render frame's encode timings, folded into counters by [`EncodeFrame::report`].
#[derive(Default)]
pub(crate) struct EncodeFrame {
    by_name: HashMap<String, PerName>,
    critical: f64,
    wall: f64,
    submit: f64,
    poll: Option<f64>,
    queue: QueueTraffic,
    seen: bool,
}

/// One frame's queue calls (a mirror of the vendored `encode_timing::QueueCounts`, so the fold is
/// testable without feature `render`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct QueueTraffic {
    pub(crate) submits: u64,
    pub(crate) command_buffers: u64,
    /// `(calls, bytes)` of `write_buffer`, `write_buffer_with`, `write_texture`.
    pub(crate) writes: [(u64, u64); 3],
}

/// `encode/queue/<op>/{calls,bytes}` keys, in [`QueueTraffic::writes`] order.
const WRITE_KEYS: [[&str; 2]; 3] = [
    ["encode/queue/write_buffer/calls", "encode/queue/write_buffer/bytes"],
    ["encode/queue/write_buffer_with/calls", "encode/queue/write_buffer_with/bytes"],
    ["encode/queue/write_texture/calls", "encode/queue/write_texture/bytes"],
];

#[allow(clippy::cast_precision_loss)] // counts and bytes per frame, far below 2^53
fn as_f64(v: u64) -> f64 {
    v as f64
}

/// `bevy_core_pipeline::core_3d::main_opaque_pass_3d<X>::{{closure}}` -> `main_opaque_pass_3d`.
pub(crate) fn encoder_name(system: &str) -> &str {
    let base = system.split('<').next().unwrap_or(system);
    base.rsplit("::").find(|s| !s.is_empty() && !s.starts_with('{')).unwrap_or(base)
}

impl EncodeFrame {
    pub(crate) fn finish(&mut self, system: &str, ms: f64) {
        let name = encoder_name(system);
        if !self.by_name.contains_key(name) {
            let keys = ["sum", "max", "count"].map(|m| format!("encode/{name}/{m}"));
            self.by_name.insert(name.to_owned(), PerName { keys, sum: 0.0, max: 0.0, count: 0 });
        }
        if let Some(n) = self.by_name.get_mut(name) {
            n.sum += ms;
            n.max = n.max.max(ms);
            n.count += 1;
        }
        self.critical = self.critical.max(ms);
        self.seen = true;
    }

    pub(crate) fn finish_wall(&mut self, ms: f64) {
        self.wall += ms;
        self.seen = true;
    }

    pub(crate) fn submit(&mut self, ms: f64) {
        self.submit += ms;
        self.seen = true;
    }

    pub(crate) fn poll(&mut self, ms: f64) {
        *self.poll.get_or_insert(0.0) += ms;
        self.seen = true;
    }

    /// This frame's queue calls (replaces, the source is already per frame).
    pub(crate) fn queue(&mut self, traffic: QueueTraffic) {
        if traffic != QueueTraffic::default() {
            self.queue = traffic;
            self.seen = true;
        }
    }

    /// Writes this frame's values (only systems that finished an encoder) and resets.
    pub(crate) fn report(&mut self, counters: &Counters) {
        if !std::mem::take(&mut self.seen) {
            return;
        }
        for n in self.by_name.values_mut().filter(|n| n.count > 0) {
            counters.add_named(&n.keys[0], unit::MS, n.sum);
            counters.set_named(&n.keys[1], unit::MS, n.max);
            counters.add_named(&n.keys[2], unit::COUNT, f64::from(n.count));
            (n.sum, n.max, n.count) = (0.0, 0.0, 0);
        }
        counters.set_named("encode/finish_critical", unit::MS, self.critical);
        counters.add_named("encode/finish_wall", unit::MS, self.wall);
        counters.add_named("encode/submit", unit::MS, self.submit);
        (self.critical, self.wall, self.submit) = (0.0, 0.0, 0.0);
        if let Some(poll) = self.poll.take() {
            counters.add_named("encode/poll", unit::MS, poll);
        }
        let q = std::mem::take(&mut self.queue);
        counters.add_named("encode/queue/submits", unit::COUNT, as_f64(q.submits));
        counters.add_named("encode/queue/command_buffers", unit::COUNT, as_f64(q.command_buffers));
        for ([calls, bytes], (c, b)) in WRITE_KEYS.iter().zip(q.writes) {
            counters.add_named(calls, unit::COUNT, as_f64(c));
            counters.add_named(bytes, unit::BYTES, as_f64(b));
        }
    }
}

/// Process-wide (one renderer per process): poll the device (`PollType::Poll`) just before the main
/// pending-command-buffer submit, timed as `encode/poll`, so wgpu's deferred housekeeping (staging
/// buffers and dropped resources destroyed, command-buffer trackers dropped, map callbacks) is paid
/// there instead of inside `encode/submit`. Default off = stock. An A/B switch for submit-cost
/// CW-5b; the work moves, it is not removed.
#[cfg(feature = "render")]
pub fn set_poll_before_submit(on: bool) {
    bevy_render::renderer::encode_timing::set_poll_before_submit(on);
}

#[cfg(feature = "render")]
static FRAME: LazyLock<Mutex<EncodeFrame>> = LazyLock::new(Mutex::default);

/// Folds the render frame's encode timings into `counters` (render thread, after `Cleanup`).
#[cfg(feature = "render")]
pub(crate) fn report(counters: &Counters) {
    let q = bevy_render::renderer::encode_timing::take_queue_counts();
    let mut frame = FRAME.lock().unwrap_or_else(PoisonError::into_inner);
    frame.queue(QueueTraffic {
        submits: q.submits,
        command_buffers: q.command_buffers,
        writes: [
            (q.write_buffer_calls, q.write_buffer_bytes),
            (q.write_buffer_with_calls, q.write_buffer_with_bytes),
            (q.write_texture_calls, q.write_texture_bytes),
        ],
    });
    frame.report(counters);
}

#[cfg(feature = "render")]
pub(crate) fn install() {
    use bevy_render::renderer::encode_timing::{self, EncodeEvent};
    fn hook(event: EncodeEvent<'_>) {
        if !crate::gauges_live() {
            return;
        }
        let ms = |d: core::time::Duration| d.as_secs_f64() * 1000.0;
        let mut frame = FRAME.lock().unwrap_or_else(PoisonError::into_inner);
        match event {
            EncodeEvent::Finish { name, elapsed } => frame.finish(name, ms(elapsed)),
            EncodeEvent::FinishWall(elapsed) => frame.finish_wall(ms(elapsed)),
            EncodeEvent::Submit(elapsed) => frame.submit(ms(elapsed)),
            EncodeEvent::Poll(elapsed) => frame.poll(ms(elapsed)),
        }
    }
    // Already installed (a second app in this process) is fine: same hook.
    let _ = encode_timing::install(hook);
}
