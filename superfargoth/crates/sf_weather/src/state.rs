//! The per-climate state machine as a pure function of time.
//!
//! Each climate's timeline is a sequence of **slots**. Slot `k` starts at
//! `t_k = phase + k·mean + jitter·(2u_k − 1)` (game hours; `u_k` hashed,
//! `jitter` a quarter of the interval range, so consecutive starts stay
//! inside `change_interval_hours`) and picks its preset by a weighted draw
//! hashed from `(seed, climate, k)`. Nothing is stepped or stored: the
//! weather at any time, past or future, is computed from the seed and the
//! explicit overrides ([`WeatherState`]), which is all a save keeps.
//!
//! A **segment** is one transition: from whatever the weather was just
//! before its start (a [`Blend`], possibly mid-transition) to one preset.
//! Segments start at slot starts, at forced changes, and when a force ends.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use sf_records::RecordId;

use crate::book::{BookClimate, ClimateIx, PresetIx, WeatherBook};
use crate::hash::{self, salt};

/// Blend channels: each has its own lead/lag window.
pub const CHANNELS: usize = 6;
/// Continuous channels (weights sum to 1; values interpolate).
pub const CLOUDS: usize = 0;
pub const SKY: usize = 1;
pub const WIND: usize = 2;
pub const FOG: usize = 3;
/// Discrete channels (each preset's contribution fades on its own:
/// crossfade, never morph): precipitation, lightning, surface rates, tags.
pub const PRECIP: usize = 4;
/// Discrete: sound cues.
pub const CUES: usize = 5;

/// Presets with a weight per channel.
pub type Blend = SmallVec<[(PresetIx, [f32; CHANNELS]); 4]>;

/// "Just before" a segment start, game hours.
const EPS: f64 = 1e-6;
/// Nested transitions (a change interrupting a change) deeper than this
/// start from the target preset.
const MAX_DEPTH: u32 = 6;

/// An explicit change, kept in saves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Override {
    /// The climate changes to `preset` at `start` (scripted weather change).
    Force {
        climate: RecordId,
        preset: RecordId,
        start: f64,
        /// Transition duration, game hours (`None`: the climate's).
        transition_hours: Option<f32>,
        /// When the natural forecast resumes (`None`: at the first slot start
        /// at least one minimum interval after `start`).
        until: Option<f64>,
    },
    /// The climate's weights change for slots starting at or after `from`
    /// (e.g. a quest calms a region).
    Weights { climate: RecordId, from: f64, weights: Vec<(RecordId, f32)> },
}

/// Everything the weather needs besides the records: a seed and the
/// overrides. Serializable; the save format.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WeatherState {
    pub seed: u64,
    pub overrides: Vec<Override>,
}

/// One forecast entry: from `start`, the weather turns into `preset`.
#[derive(Clone, Debug, PartialEq)]
pub struct ForecastEntry {
    pub start: f64,
    pub preset: PresetIx,
    pub transition_hours: f64,
    pub forced: bool,
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    to: PresetIx,
    start: f64,
    duration: f64,
    /// Forces starting at or after this don't exist for the `from` blend.
    bound: f64,
    forced: bool,
}

#[derive(Clone, Copy)]
struct Force {
    preset: PresetIx,
    start: f64,
    transition: Option<f64>,
    until: Option<f64>,
}

impl WeatherState {
    pub fn new(seed: u64) -> Self {
        Self { seed, overrides: Vec::new() }
    }

    /// Forces `climate` to `preset` from `start` (game hours). `hold_hours`
    /// keeps it at least that long (then the next slot start resumes the
    /// forecast); `None` holds until the first slot start one minimum
    /// interval later.
    pub fn force(&mut self, climate: &RecordId, preset: &RecordId, start: f64, transition_hours: Option<f32>, hold_hours: Option<f32>) {
        self.overrides.push(Override::Force {
            climate: climate.clone(),
            preset: preset.clone(),
            start,
            transition_hours,
            until: hold_hours.map(|h| start + f64::from(h.max(0.0))),
        });
    }

    /// Replaces `climate`'s weights for slots starting at or after `from`.
    pub fn set_weights(&mut self, climate: &RecordId, from: f64, weights: Vec<(RecordId, f32)>) {
        self.overrides.push(Override::Weights { climate: climate.clone(), from, weights });
    }

    fn phase(&self, c: &BookClimate) -> f64 {
        hash::unit(hash::hash(self.seed, c.key, salt::PHASE, 0)) * c.mean
    }

    /// Start of slot `k`, game hours.
    pub fn slot_start(&self, book: &WeatherBook, climate: ClimateIx, k: i64) -> f64 {
        self.start_in(book.c(climate), k)
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn start_in(&self, c: &BookClimate, k: i64) -> f64 {
        let u = hash::unit(hash::hash(self.seed, c.key, salt::JITTER, k as u64));
        self.phase(c) + k as f64 * c.mean + c.jitter * (2.0 * u - 1.0)
    }

    /// The slot containing `t`.
    pub fn slot_at(&self, book: &WeatherBook, climate: ClimateIx, t: f64) -> i64 {
        self.slot_in(book.c(climate), t)
    }

    /// The preset slot `k` draws: a pure function of (seed, climate, k) and
    /// the weight overrides in effect at its start.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn slot_preset(&self, book: &WeatherBook, climate: ClimateIx, k: i64) -> Option<PresetIx> {
        let c = book.c(climate);
        let t = self.start_in(c, k);
        let u = hash::unit(hash::hash(self.seed, c.key, salt::PICK, k as u64)) as f32;
        // The latest weight override for this climate in effect at `t`.
        let mut edit: Option<(f64, &[(RecordId, f32)])> = None;
        for o in &self.overrides {
            if let Override::Weights { climate: id, from, weights } = o
                && *id == c.id
                && *from <= t
                && edit.is_none_or(|(f, _)| *from >= f)
            {
                edit = Some((*from, weights.as_slice()));
            }
        }
        match edit {
            Some((_, weights)) => {
                let resolved = weights.iter().filter_map(|(id, w)| Some((book.preset(id)?, *w))).filter(|(_, w)| *w > 0.0);
                pick(resolved, u)
            }
            None => pick(c.weights.iter().copied(), u),
        }
    }

    fn forces(&self, book: &WeatherBook, c: &BookClimate) -> impl Iterator<Item = Force> {
        self.overrides.iter().filter_map(move |o| match o {
            Override::Force { climate, preset, start, transition_hours, until } if *climate == c.id => Some(Force {
                preset: book.preset(preset)?,
                start: *start,
                transition: transition_hours.map(|h| f64::from(h.max(0.0))),
                until: *until,
            }),
            _ => None,
        })
    }

    fn force_end(&self, c: &BookClimate, f: &Force) -> f64 {
        if let Some(u) = f.until {
            return u.max(f.start);
        }
        let earliest = f.start + f.transition.unwrap_or(c.transition).max(c.min_interval);
        let mut k = self.slot_in(c, earliest);
        if self.start_in(c, k) < earliest {
            k += 1;
        }
        self.start_in(c, k)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn slot_in(&self, c: &BookClimate, t: f64) -> i64 {
        let mut k = ((t - self.phase(c)) / c.mean).floor() as i64;
        while self.start_in(c, k) > t {
            k -= 1;
        }
        while self.start_in(c, k + 1) <= t {
            k += 1;
        }
        k
    }

    fn segment(&self, book: &WeatherBook, climate: ClimateIx, t: f64, bound: f64) -> Option<Segment> {
        let c = book.c(climate);
        let k = self.slot_in(c, t);
        let natural_start = self.start_in(c, k);
        // The latest force started at or before `t` (and before `bound`).
        let mut latest: Option<Force> = None;
        for f in self.forces(book, c) {
            if f.start <= t && f.start < bound && latest.is_none_or(|l| f.start >= l.start) {
                latest = Some(f);
            }
        }
        if let Some(f) = latest {
            let end = self.force_end(c, &f);
            if t < end {
                return Some(Segment {
                    to: f.preset,
                    start: f.start,
                    duration: f.transition.unwrap_or(c.transition),
                    bound: f.start,
                    forced: true,
                });
            }
            if end >= natural_start {
                // The force ended inside this slot: back to the slot's draw.
                let to = self.slot_preset(book, climate, k)?;
                return Some(Segment { to, start: end, duration: c.transition, bound, forced: false });
            }
        }
        let to = self.slot_preset(book, climate, k)?;
        Some(Segment { to, start: natural_start, duration: c.transition, bound, forced: false })
    }

    /// The climate's weather at `t`: presets with per-channel weights.
    /// Empty when the climate has no presets.
    pub fn blend(&self, book: &WeatherBook, climate: ClimateIx, t: f64) -> Blend {
        self.blend_in(book, climate, t, f64::INFINITY, 0)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn blend_in(&self, book: &WeatherBook, climate: ClimateIx, t: f64, bound: f64, depth: u32) -> Blend {
        let Some(seg) = self.segment(book, climate, t, bound) else {
            return Blend::new();
        };
        let s = if seg.duration > 0.0 { ((t - seg.start) / seg.duration).clamp(0.0, 1.0) as f32 } else { 1.0 };
        let target = [(seg.to, [1.0; CHANNELS])];
        if s >= 1.0 || depth >= MAX_DEPTH {
            return Blend::from_slice(&target);
        }
        let mut from = self.blend_in(book, climate, seg.start - EPS, seg.bound, depth + 1);
        if from.is_empty() {
            return Blend::from_slice(&target);
        }
        let timing = &book.p(seg.to).record.transition;
        let incoming = [
            timing.clouds.ramp(s),
            timing.sky.ramp(s),
            timing.wind.ramp(s),
            timing.fog.ramp(s),
            timing.precipitation.ramp(s),
            timing.cues.ramp(s),
        ];
        let mut has_target = false;
        for (p, w) in &mut from {
            if *p == seg.to {
                has_target = true;
                for ch in 0..CHANNELS {
                    w[ch] += (1.0 - w[ch]) * incoming[ch];
                }
                continue;
            }
            let leaving = &book.p(*p).record.transition;
            for ch in [CLOUDS, SKY, WIND, FOG] {
                w[ch] *= 1.0 - incoming[ch];
            }
            w[PRECIP] *= 1.0 - leaving.precipitation_out.ramp(s);
            w[CUES] *= 1.0 - leaving.cues_out.ramp(s);
        }
        if !has_target {
            from.push((seg.to, incoming));
        }
        from
    }

    /// The preset the climate is in, or turning into, at `t`.
    pub fn current(&self, book: &WeatherBook, climate: ClimateIx, t: f64) -> Option<PresetIx> {
        self.segment(book, climate, t, f64::INFINITY).map(|s| s.to)
    }

    /// The next `count` changes from `from` on (the one in progress first).
    pub fn forecast(&self, book: &WeatherBook, climate: ClimateIx, from: f64, count: usize) -> Vec<ForecastEntry> {
        let c = book.c(climate);
        let mut out: Vec<ForecastEntry> = Vec::with_capacity(count);
        let mut t = from;
        // Each step moves to the next event, so this ends; the cap guards
        // against degenerate data (zero-length intervals).
        for _ in 0..count.saturating_mul(4).max(8) {
            if out.len() >= count {
                break;
            }
            let Some(seg) = self.segment(book, climate, t, f64::INFINITY) else { break };
            if out.last().is_none_or(|l| (l.start - seg.start).abs() > EPS) {
                out.push(ForecastEntry { start: seg.start, preset: seg.to, transition_hours: seg.duration, forced: seg.forced });
            }
            let mut next = self.start_in(c, self.slot_in(c, t) + 1);
            for f in self.forces(book, c) {
                if f.start > t {
                    next = next.min(f.start);
                }
                let end = self.force_end(c, &f);
                if f.start <= t && end > t {
                    next = next.min(end);
                }
            }
            t = next.max(t + EPS);
        }
        out
    }
}

/// Weighted draw with `u` in `[0, 1)`.
fn pick(weights: impl Iterator<Item = (PresetIx, f32)> + Clone, u: f32) -> Option<PresetIx> {
    let total: f32 = weights.clone().map(|(_, w)| w).sum();
    if total <= 0.0 {
        return None;
    }
    let mut x = u * total;
    let mut last = None;
    for (p, w) in weights {
        last = Some(p);
        if x < w {
            return Some(p);
        }
        x -= w;
    }
    last
}
