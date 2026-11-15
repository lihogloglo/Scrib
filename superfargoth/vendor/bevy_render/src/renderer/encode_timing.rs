//! superfargoth patch (own, M3 encode timers): an opt-in wall-clock hook around command-encoder `finish()` and
//! `queue.submit`, for untraced encode costs (`sf_diag_bevy` installs it; superfargoth
//! `vendor/README.md`). Not installed = one atomic load per finish/submit, nothing timed.

use std::sync::OnceLock;

use bevy_platform::time::Instant;
use core::time::Duration;

/// One timed event, reported on the thread that did the work.
#[derive(Clone, Copy, Debug)]
pub enum EncodeEvent<'a> {
    /// One pending encoder's `finish()`, named by the system that recorded it.
    Finish { name: &'a str, elapsed: Duration },
    /// One whole parallel finish of the pending encoders (wall).
    FinishWall(Duration),
    /// One `queue.submit`.
    Submit(Duration),
}

/// The hook: called from task-pool threads too, so it must be thread-safe.
pub type EncodeTimingHook = fn(EncodeEvent<'_>);

static HOOK: OnceLock<EncodeTimingHook> = OnceLock::new();

/// Installs the process-wide hook; `false` if one is already installed.
pub fn install(hook: EncodeTimingHook) -> bool {
    HOOK.set(hook).is_ok()
}

/// The installed hook, if any.
#[inline]
pub fn hook() -> Option<EncodeTimingHook> {
    HOOK.get().copied()
}

/// Starts a timer when a hook is installed.
#[inline]
pub fn start() -> Option<(EncodeTimingHook, Instant)> {
    hook().map(|hook| (hook, Instant::now()))
}
