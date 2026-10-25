//! Named, thread-safe counters.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{PoisonError, RwLock};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Common unit suffixes. Any string works; these keep reports consistent.
pub mod unit {
    /// Milliseconds. Baseline comparison only looks at counters in this unit.
    pub const MS: &str = "ms";
    /// A number of things (items, requests, entities).
    pub const COUNT: &str = "count";
    /// Bytes.
    pub const BYTES: &str = "bytes";
    /// Seconds (ages, latencies too coarse for milliseconds).
    pub const SECONDS: &str = "s";
}

/// How a counter combines the values reported within one frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterKind {
    /// Values add up within a frame (stage time, items processed). A frame
    /// with no report counts as 0.
    Sum,
    /// The last value in a frame wins (queue depth, request age). A frame
    /// with no report is missing, not 0.
    Gauge,
}

/// A counter's name, kind, and unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CounterDef {
    /// Slash-separated `area/stage/metric`, e.g. `stream/spawn/ms`.
    pub name: String,
    pub kind: CounterKind,
    /// Unit suffix, see [`unit`].
    pub unit: String,
}

/// Handle to a registered counter: an index, so reporting skips the name
/// lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CounterId(pub(crate) u32);

impl CounterId {
    /// Position in registration order (the recorder's column index).
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// f64 bits of NaN: "nothing reported this frame".
const UNSET: u64 = 0x7ff8_0000_0000_0000;

struct Slot {
    def: CounterDef,
    bits: AtomicU64,
}

#[derive(Default)]
struct Inner {
    by_name: HashMap<String, CounterId>,
    slots: Vec<Slot>,
}

/// Registry of named counters, shared by everything that reports.
///
/// Reporting takes `&self` (each slot is one atomic holding f64 bits), so
/// systems that report never conflict over an ECS resource, and task-pool
/// jobs can report through an `Arc<Counters>`. Registration takes a short
/// write lock; do it up front where possible.
#[derive(Default)]
pub struct Counters {
    inner: RwLock<Inner>,
}

impl std::fmt::Debug for Counters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Counters")
            .field("len", &self.len())
            .finish()
    }
}

impl Counters {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a counter, or returns the existing one with that name (its
    /// kind and unit stay as first registered).
    pub fn register(&self, name: &str, kind: CounterKind, unit: &str) -> CounterId {
        if let Some(id) = self.id(name) {
            return id;
        }
        let mut inner = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(&id) = inner.by_name.get(name) {
            return id;
        }
        let id = CounterId(u32::try_from(inner.slots.len()).expect("too many counters"));
        inner.slots.push(Slot {
            def: CounterDef {
                name: name.to_owned(),
                kind,
                unit: unit.to_owned(),
            },
            bits: AtomicU64::new(UNSET),
        });
        inner.by_name.insert(name.to_owned(), id);
        id
    }

    /// The id of a registered counter.
    pub fn id(&self, name: &str) -> Option<CounterId> {
        let inner = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        inner.by_name.get(name).copied()
    }

    /// The definition of a registered counter.
    pub fn def(&self, id: CounterId) -> Option<CounterDef> {
        let inner = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        inner.slots.get(id.index()).map(|slot| slot.def.clone())
    }

    /// Number of registered counters.
    pub fn len(&self) -> usize {
        self.inner
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .slots
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds to this frame's value (the value starts from 0 each frame).
    pub fn add(&self, id: CounterId, value: f64) {
        let inner = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(slot) = inner.slots.get(id.index()) {
            // The closure always returns `Some`, so this never fails.
            let _ = slot
                .bits
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |bits| {
                    let current = f64::from_bits(bits);
                    let base = if current.is_nan() { 0.0 } else { current };
                    Some((base + value).to_bits())
                });
        }
    }

    /// Sets this frame's value (last write wins).
    pub fn set(&self, id: CounterId, value: f64) {
        let inner = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(slot) = inner.slots.get(id.index()) {
            slot.bits.store(value.to_bits(), Ordering::Relaxed);
        }
    }

    /// Adds a duration in milliseconds.
    pub fn add_duration(&self, id: CounterId, duration: Duration) {
        self.add(id, duration.as_secs_f64() * 1000.0);
    }

    /// [`Counters::add`] by name, registering a [`CounterKind::Sum`] counter
    /// on first use.
    pub fn add_named(&self, name: &str, unit: &str, value: f64) {
        let id = self.register(name, CounterKind::Sum, unit);
        self.add(id, value);
    }

    /// [`Counters::add_duration`] by name, registering a millisecond
    /// [`CounterKind::Sum`] counter on first use.
    pub fn add_duration_named(&self, name: &str, duration: Duration) {
        let id = self.register(name, CounterKind::Sum, unit::MS);
        self.add_duration(id, duration);
    }

    /// [`Counters::set`] by name, registering a [`CounterKind::Gauge`] on
    /// first use.
    pub fn set_named(&self, name: &str, unit: &str, value: f64) {
        let id = self.register(name, CounterKind::Gauge, unit);
        self.set(id, value);
    }

    /// Runs `f` and adds its wall time to `id`.
    pub fn time<R>(&self, id: CounterId, f: impl FnOnce() -> R) -> R {
        let start = std::time::Instant::now();
        let result = f();
        self.add_duration(id, start.elapsed());
        result
    }

    /// Takes every counter's value for the frame and resets it. `visit`
    /// sees every registered counter, with `None` when nothing was reported.
    pub fn drain(&self, mut visit: impl FnMut(CounterId, &CounterDef, Option<f64>)) {
        let inner = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        for (index, slot) in inner.slots.iter().enumerate() {
            let value = f64::from_bits(slot.bits.swap(UNSET, Ordering::Relaxed));
            let id = CounterId(u32::try_from(index).expect("too many counters"));
            visit(id, &slot.def, (!value.is_nan()).then_some(value));
        }
    }

    /// Resets every counter without reading it.
    pub fn clear(&self) {
        self.drain(|_, _, _| {});
    }
}
