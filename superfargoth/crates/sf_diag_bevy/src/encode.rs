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
//! - `encode/submit`: every `queue.submit` this frame.
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
    seen: bool,
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
    }
}

#[cfg(feature = "render")]
static FRAME: LazyLock<Mutex<EncodeFrame>> = LazyLock::new(Mutex::default);

/// Folds the render frame's encode timings into `counters` (render thread, after `Cleanup`).
#[cfg(feature = "render")]
pub(crate) fn report(counters: &Counters) {
    FRAME.lock().unwrap_or_else(PoisonError::into_inner).report(counters);
}

#[cfg(feature = "render")]
pub(crate) fn install() {
    use bevy_render::renderer::encode_timing::{self, EncodeEvent};
    fn hook(event: EncodeEvent<'_>) {
        let ms = |d: core::time::Duration| d.as_secs_f64() * 1000.0;
        let mut frame = FRAME.lock().unwrap_or_else(PoisonError::into_inner);
        match event {
            EncodeEvent::Finish { name, elapsed } => frame.finish(name, ms(elapsed)),
            EncodeEvent::FinishWall(elapsed) => frame.finish_wall(ms(elapsed)),
            EncodeEvent::Submit(elapsed) => frame.submit(ms(elapsed)),
        }
    }
    // Already installed (a second app in this process) is fine: same hook.
    let _ = encode_timing::install(hook);
}
