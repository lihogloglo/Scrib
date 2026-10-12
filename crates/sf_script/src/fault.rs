//! Fault policy (scripting.md "Error isolation and limits"): one handler's error never stops the
//! game; a repeat offender is suspended for the session.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// What went wrong in one call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultKind {
    /// A Lua error (raised, or from an API function).
    Error,
    /// The watchdog aborted the call (per-call time limit).
    Timeout,
    /// The context hit its memory limit: its state may be inconsistent, so it is suspended.
    Memory,
    /// A Rust panic escaped an API function: the context is failed.
    Panic,
}

/// Thresholds. Defaults: 3 aborts or 30 errors within 60 s suspend the context (owner decision 5).
#[derive(Clone, Copy, Debug)]
pub struct FaultPolicy {
    pub max_aborts: usize,
    pub max_errors: usize,
    pub window: Duration,
}

impl Default for FaultPolicy {
    fn default() -> Self {
        Self {
            max_aborts: 3,
            max_errors: 30,
            window: Duration::from_secs(60),
        }
    }
}

/// Recent faults of one context, as sliding windows.
#[derive(Debug, Default, Clone)]
pub struct FaultLog {
    aborts: VecDeque<Instant>,
    errors: VecDeque<Instant>,
}

/// The policy's verdict for a fault.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Logged and counted; the context keeps running.
    Continue,
    /// Suspend the context for the session (handlers skipped, state kept).
    Suspend(String),
    /// Fail the context (panic).
    Fail(String),
}

impl FaultLog {
    /// Records a fault at `now` and returns what to do with the context.
    pub fn record(&mut self, kind: FaultKind, now: Instant, policy: &FaultPolicy) -> Verdict {
        let prune = |q: &mut VecDeque<Instant>| {
            while q
                .front()
                .is_some_and(|t| now.saturating_duration_since(*t) > policy.window)
            {
                q.pop_front();
            }
        };
        match kind {
            FaultKind::Panic => Verdict::Fail("a Rust API function panicked".into()),
            FaultKind::Memory => Verdict::Suspend("memory limit reached".into()),
            FaultKind::Timeout => {
                self.aborts.push_back(now);
                prune(&mut self.aborts);
                if self.aborts.len() >= policy.max_aborts {
                    Verdict::Suspend(format!(
                        "{} handler timeouts within {:?}",
                        self.aborts.len(),
                        policy.window
                    ))
                } else {
                    Verdict::Continue
                }
            }
            FaultKind::Error => {
                self.errors.push_back(now);
                prune(&mut self.errors);
                if self.errors.len() >= policy.max_errors {
                    Verdict::Suspend(format!(
                        "{} errors within {:?}",
                        self.errors.len(),
                        policy.window
                    ))
                } else {
                    Verdict::Continue
                }
            }
        }
    }
}
