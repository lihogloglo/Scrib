//! superfargoth patch (own, M3 encode timers): an opt-in wall-clock hook around command-encoder `finish()` and
//! `queue.submit`, for untraced encode costs (`sf_diag_bevy` installs it; superfargoth
//! `vendor/README.md`). Not installed = one atomic load per finish/submit, nothing timed.
//!
//! superfargoth patch (own, CW-5b submit counters): queue traffic counters ([`QueueCounts`], counted by the
//! [`RenderQueue`](super::RenderQueue) methods that shadow `wgpu::Queue`'s while a hook is installed) and an
//! opt-in device poll before the main submit ([`set_poll_before_submit`], timed as [`EncodeEvent::Poll`]).

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
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
    /// superfargoth patch (own, CW-5b submit counters): one `device.poll(Poll)` just before the main submit
    /// (only with [`set_poll_before_submit`]).
    Poll(Duration),
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

// superfargoth patch (own, CW-5b submit counters)

/// Queue traffic since the last [`take_queue_counts`]: every call through [`RenderQueue`](super::RenderQueue)
/// (timed or not), counted only while a hook is installed. Each `write_*` creates one wgpu staging buffer,
/// destroyed in a later `maintain` (inside a submit unless the device is polled).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueueCounts {
    /// `queue.submit` calls.
    pub submits: u64,
    /// Command buffers in those submits (one per command encoder).
    pub command_buffers: u64,
    /// `queue.write_buffer` calls.
    pub write_buffer_calls: u64,
    /// Their bytes.
    pub write_buffer_bytes: u64,
    /// `queue.write_buffer_with` calls.
    pub write_buffer_with_calls: u64,
    /// Their bytes (the view sizes).
    pub write_buffer_with_bytes: u64,
    /// `queue.write_texture` calls.
    pub write_texture_calls: u64,
    /// Their source bytes (`data.len()`).
    pub write_texture_bytes: u64,
}

/// First slot of an operation's (calls, amount) pair in `COUNTS`.
#[derive(Clone, Copy)]
pub(crate) enum QueueOp {
    /// Amount = command buffers.
    Submit = 0,
    WriteBuffer = 2,
    WriteBufferWith = 4,
    WriteTexture = 6,
}

static COUNTS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];

/// Counts one call of `op` with `amount` (bytes or command buffers) while a hook is installed.
#[inline]
pub(crate) fn count(op: QueueOp, amount: u64) {
    if HOOK.get().is_some() {
        let i = op as usize;
        COUNTS[i].fetch_add(1, Ordering::Relaxed);
        COUNTS[i + 1].fetch_add(amount, Ordering::Relaxed);
    }
}

/// The counts since the last call, reset to zero.
pub fn take_queue_counts() -> QueueCounts {
    let t = |i: usize| COUNTS[i].swap(0, Ordering::Relaxed);
    QueueCounts {
        submits: t(0),
        command_buffers: t(1),
        write_buffer_calls: t(2),
        write_buffer_bytes: t(3),
        write_buffer_with_calls: t(4),
        write_buffer_with_bytes: t(5),
        write_texture_calls: t(6),
        write_texture_bytes: t(7),
    }
}

static POLL_BEFORE_SUBMIT: AtomicBool = AtomicBool::new(false);

/// Process-wide: poll the device (`PollType::Poll`, non-blocking) just before the main pending-command-buffer
/// submit, so wgpu's deferred housekeeping (`maintain`: staging-buffer and resource destruction, tracker drops,
/// map callbacks) runs there instead of inside the submit. Default `false` = stock.
pub fn set_poll_before_submit(on: bool) {
    POLL_BEFORE_SUBMIT.store(on, Ordering::Relaxed);
}

/// Whether [`set_poll_before_submit`] is on.
#[inline]
pub fn poll_before_submit() -> bool {
    POLL_BEFORE_SUBMIT.load(Ordering::Relaxed)
}

/// With [`set_poll_before_submit`] on: polls `device` once, timed as [`EncodeEvent::Poll`] when a hook is
/// installed. Off: nothing.
pub fn poll_if_enabled(device: &super::RenderDevice) {
    if !poll_before_submit() {
        return;
    }
    let timer = start();
    // A poll error (device lost) surfaces again at the submit that follows.
    let _ = device.poll(wgpu::PollType::Poll);
    if let Some((hook, start)) = timer {
        hook(EncodeEvent::Poll(start.elapsed()));
    }
}

// superfargoth patch (own: bind group walk once): forwards to the vendored wgpu-core
// (`wgpu_core::command::bind_group_walk`, superfargoth `vendor/README.md`), so callers need no wgpu-core
// dependency of their own. On wasm (no wgpu-core in reach) the switch does nothing and the counts are zero.

/// Render-pass bind group walks since the last [`take_bind_group_walk_counts`] (wgpu-core's
/// `bind_group_walk::WalkCounts`: `SetBindGroup` merges and pre-draw init registrations, done and skipped,
/// with the texture entries each walked or would have walked).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BindGroupWalkCounts {
    pub merges: u64,
    pub merge_textures: u64,
    pub merges_skipped: u64,
    pub merge_textures_skipped: u64,
    pub inits: u64,
    pub init_textures: u64,
    pub inits_skipped: u64,
    pub init_textures_skipped: u64,
}

/// Process-wide: a render pass walks each bind group's textures once (repeat merges and repeat init
/// registrations of a bind group within one pass are skipped). Default `false` = stock wgpu-core.
pub fn set_bind_group_walk_once(on: bool) {
    #[cfg(not(target_arch = "wasm32"))]
    wgpu::wgc::command::bind_group_walk::set_walk_once(on);
    #[cfg(target_arch = "wasm32")]
    let _ = on;
}

/// The walk counts since the last call, reset to zero.
pub fn take_bind_group_walk_counts() -> BindGroupWalkCounts {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let c = wgpu::wgc::command::bind_group_walk::take_counts();
        BindGroupWalkCounts {
            merges: c.merges,
            merge_textures: c.merge_textures,
            merges_skipped: c.merges_skipped,
            merge_textures_skipped: c.merge_textures_skipped,
            inits: c.inits,
            init_textures: c.init_textures,
            inits_skipped: c.inits_skipped,
            init_textures_skipped: c.init_textures_skipped,
        }
    }
    #[cfg(target_arch = "wasm32")]
    BindGroupWalkCounts::default()
}
