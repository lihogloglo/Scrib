//! Per-call CPU limit through Luau's `interrupt` callback (scripting.md "Error isolation and
//! limits"; Roblox script timeout).
//!
//! Luau calls the interrupt at function calls and loop back-edges. The callback compares a
//! monotonic clock with a deadline armed for the duration of one host → script call. A script
//! that catches the timeout error with `pcall` gets it again at its next safepoint (the tripped
//! flag latches until the host disarms), so it can't run on past the limit.
//!
//! Spike L1 measured a clock read per interrupt against a read every N interrupts; see
//! [`Watchdog::CHECK_EVERY`].

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use mlua::{Lua, VmState};

/// Shared between the context (arms, disarms) and the interrupt callback (checks).
#[derive(Debug)]
pub struct Watchdog {
    epoch: Instant,
    /// Nanoseconds since `epoch`; 0 = disarmed.
    deadline: AtomicU64,
    tripped: AtomicBool,
    ticks: AtomicU32,
}

impl Watchdog {
    /// The clock is read on every N-th interrupt (power of two). Measured in the spike:
    /// see scripting.md "Spike results" (watchdog overhead).
    pub const CHECK_EVERY: u32 = 64;

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            epoch: Instant::now(),
            deadline: AtomicU64::new(0),
            tripped: AtomicBool::new(false),
            ticks: AtomicU32::new(0),
        })
    }

    #[allow(clippy::cast_possible_truncation)] // u64 nanoseconds cover 584 years of uptime
    fn now_ns(&self) -> u64 {
        self.epoch.elapsed().as_nanos() as u64
    }

    /// Arms the deadline `limit` from now and clears the tripped flag.
    #[allow(clippy::cast_possible_truncation)]
    pub fn arm(&self, limit: Duration) {
        self.arm_from(Instant::now(), limit);
    }

    /// Arms the deadline `limit` after `start` (saves a clock read when the caller has one).
    #[allow(clippy::cast_possible_truncation)]
    pub fn arm_from(&self, start: Instant, limit: Duration) {
        let start_ns = start.saturating_duration_since(self.epoch).as_nanos() as u64;
        self.tripped.store(false, Ordering::Relaxed);
        self.deadline.store(
            start_ns.saturating_add(limit.as_nanos() as u64).max(1),
            Ordering::Relaxed,
        );
    }

    /// Disarms and returns whether the deadline fired during the call.
    pub fn disarm(&self) -> bool {
        self.deadline.store(0, Ordering::Relaxed);
        self.tripped.load(Ordering::Relaxed)
    }

    /// Installs the interrupt callback on `lua`.
    pub fn install(self: &Arc<Self>, lua: &Lua) {
        let wd = Arc::clone(self);
        lua.set_interrupt(move |_| {
            let deadline = wd.deadline.load(Ordering::Relaxed);
            if deadline == 0 {
                return Ok(VmState::Continue);
            }
            // Once tripped, every safepoint raises: a `pcall` loop can't swallow the timeout,
            // because the error resurfaces at the first safepoint outside the `pcall`.
            if wd.tripped.load(Ordering::Relaxed) {
                return Err(mlua::Error::runtime(
                    "script exceeded its time limit (watchdog)",
                ));
            }
            if !wd
                .ticks
                .fetch_add(1, Ordering::Relaxed)
                .is_multiple_of(Self::CHECK_EVERY)
            {
                return Ok(VmState::Continue);
            }
            if wd.now_ns() > deadline {
                wd.tripped.store(true, Ordering::Relaxed);
                return Err(mlua::Error::runtime(
                    "script exceeded its time limit (watchdog)",
                ));
            }
            Ok(VmState::Continue)
        });
    }

    /// The tripped flag stays set until the next arm, so a `pcall`ed timeout still counts.
    pub fn tripped(&self) -> bool {
        self.tripped.load(Ordering::Relaxed)
    }
}

/// Checks the clock on every interrupt (the spike's baseline variant).
pub fn install_every_tick(lua: &Lua, wd: &Arc<Watchdog>) {
    let wd = Arc::clone(wd);
    lua.set_interrupt(move |_| {
        let deadline = wd.deadline.load(Ordering::Relaxed);
        if deadline != 0 && wd.now_ns() > deadline {
            wd.tripped.store(true, Ordering::Relaxed);
            return Err(mlua::Error::runtime(
                "script exceeded its time limit (watchdog)",
            ));
        }
        Ok(VmState::Continue)
    });
}
