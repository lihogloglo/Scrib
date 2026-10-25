//! Named timers as plain data (scripting.md "State, timers and saves"; OpenMW-style registered
//! callbacks). A pending timer is a callback *name*, a due time and a plain-data payload, so
//! timers survive saves and hot reloads; the callback is looked up by name when it fires.

use serde::{Deserialize, Serialize};

use crate::state::StateValue;

/// One pending timer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    /// Returned to the script by `after`/`every`; `cancel(id)` removes it.
    pub id: u64,
    /// Registered callback name (`timers.register(name, fn)`).
    pub name: String,
    /// Simulation time (seconds) at which it fires.
    pub due: f64,
    /// Repeat interval for `every`.
    pub every: Option<f64>,
    pub payload: StateValue,
}

/// Pending timers of one context, fired in (due, id) order: deterministic.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimerQueue {
    pub timers: Vec<Timer>,
    pub next_id: u64,
}

impl TimerQueue {
    /// Schedules a timer and returns its id.
    pub fn schedule(
        &mut self,
        name: String,
        due: f64,
        every: Option<f64>,
        payload: StateValue,
    ) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.timers.push(Timer {
            id,
            name,
            due,
            every,
            payload,
        });
        id
    }

    pub fn cancel(&mut self, id: u64) -> bool {
        let before = self.timers.len();
        self.timers.retain(|t| t.id != id);
        before != self.timers.len()
    }

    /// Removes and returns the timers due at `now`, in firing order. Repeating timers are
    /// rescheduled (at most one firing per call, so a long frame doesn't cause a burst).
    pub fn take_due(&mut self, now: f64) -> Vec<Timer> {
        let mut due: Vec<Timer> = Vec::new();
        let mut i = 0;
        while i < self.timers.len() {
            if self.timers[i].due <= now {
                let t = self.timers.swap_remove(i);
                if let Some(every) = t.every {
                    let mut next = t.clone();
                    next.due = (t.due + every.max(1e-3)).max(now);
                    self.timers.push(next);
                }
                due.push(t);
            } else {
                i += 1;
            }
        }
        due.sort_by(|a, b| a.due.total_cmp(&b.due).then(a.id.cmp(&b.id)));
        due
    }
}

/// PCG32 (O'Neill 2014, XSH-RR variant): `math.random` and `sf.random` per context, seeded and
/// saved with the context. A 20-line generator instead of `rand_pcg` because its state must be
/// plain data in our snapshot and the stream must never change under a dependency update.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    const MUL: u64 = 6_364_136_223_846_793_005;

    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();
        rng
    }

    #[allow(clippy::cast_possible_truncation)] // PCG output permutation truncates by design
    pub fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old.wrapping_mul(Self::MUL).wrapping_add(self.inc);
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    /// Uniform in [0, 1).
    #[allow(clippy::cast_precision_loss)] // 53 bits: exact
    pub fn next_f64(&mut self) -> f64 {
        let hi = u64::from(self.next_u32()) << 21;
        let lo = u64::from(self.next_u32() >> 11);
        let x = (hi | lo) as f64;
        x / (1u64 << 53) as f64
    }

    /// Uniform integer in [lo, hi] (Lua `math.random(m, n)` semantics).
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        let span = hi.wrapping_sub(lo) as u64 + 1;
        lo + (self.next_f64() * span as f64) as i64
    }
}
