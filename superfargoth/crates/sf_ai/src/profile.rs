//! `AI8g`: the per-individual perception profile (`ai.md` "Perception and stimuli" → "Perception
//! profile (per individual)"). Pure, Bevy-free, host-only.
//!
//! - [`PerceptionProfile`]: nine ratio fields on the global tunables, default 1.0 each (an actor
//!   without one behaves exactly as the globals), range checked at load ([`PerceptionProfile::validate`]).
//! - [`PerceptionState`]: a temporary state record (game-named: drunk, tired, on duty) with the same
//!   fields, each optional (missing = 1), and an optional `duration_s`.
//! - [`PerceptionStates`]: an actor's active states, an inline array of [`PERCEPTION_STATES_CAPACITY`]
//!   (no heap): timed entries expire on the host tick, conditions (`until = None`) stay until the
//!   adapter clears them. The same state refreshes (later `until` wins), never stacks.
//! - [`resolve`] builds the cached [`EffectivePerception`] (profile × Π states, clamped), rebuilt
//!   only when the inputs change or [`EffectivePerception::needs_rebuild`] says a state expired.
//!
//! The framework knows only these fields and record ids, never a stat or a race (the adapter fills
//! the profile, `AI0g`).

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError, RecordId};

use crate::perception::{AwarenessConfig, PerceptClock};

/// `sf_records` kind of perception profile presets (a race, creature type or role; the adapter
/// picks one and writes the actor's profile).
pub const PERCEPTION_PROFILE_KIND: &str = "sf.ai.perception_profile";

/// `sf_records` kind of perception states (drunk, tired, on duty: game-named records).
pub const PERCEPTION_STATE_KIND: &str = "sf.ai.perception_state";

/// Registers [`PERCEPTION_PROFILE_KIND`] and [`PERCEPTION_STATE_KIND`]. Call once before loading mods
/// (here in the core so a Bevy-free mod loader can register them).
///
/// # Errors
/// A kind already registered with another type.
pub fn register_perception_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<PerceptionProfile>(PERCEPTION_PROFILE_KIND)?;
    db.register_serde::<PerceptionState>(PERCEPTION_STATE_KIND)?;
    Ok(())
}

// tunables: const (identity ratio: 1.0 leaves the global tunable unchanged, ai.md)
const IDENTITY: f32 = 1.0;
// tunables: const (maths: a half-angle never exceeds a full circle's half, ai.md "≤ 180°")
const MAX_HALF_DEG: f32 = 180.0;

/// Active states per actor (ai.md: "inline array of 4, a capacity const").
pub const PERCEPTION_STATES_CAPACITY: usize = 4;

/// One field of a [`PerceptionProfile`] / [`PerceptionState`], with its load-time range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PerceptionField {
    /// × `sight_range_m` and `lose_sight_range_m` (`near_m` unscaled).
    SightRange,
    /// × both sight half-angles, capped at 180°.
    SightCone,
    /// × the sight `rate`.
    SightGain,
    /// How much darkness lowers sight: 0 sees in the dark, 1 normal, 2 night-blind.
    Darkness,
    /// The per-actor `hearing_mul` (× noise reach).
    Hearing,
    /// × sight gain and `noise_gain`.
    Alertness,
    /// × the three band decays (< 1 holds suspicion longer).
    Decay,
    /// × `search_ttl_s` (persistence).
    SearchTtl,
    /// × `search_error_m` and `search_widen_mps`.
    SearchError,
}

impl PerceptionField {
    /// Every field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SightRange,
        Self::SightCone,
        Self::SightGain,
        Self::Darkness,
        Self::Hearing,
        Self::Alertness,
        Self::Decay,
        Self::SearchTtl,
        Self::SearchError,
    ];

    /// The field's record name.
    pub fn name(self) -> &'static str {
        match self {
            Self::SightRange => "sight_range",
            Self::SightCone => "sight_cone",
            Self::SightGain => "sight_gain",
            Self::Darkness => "darkness",
            Self::Hearing => "hearing",
            Self::Alertness => "alertness",
            Self::Decay => "decay",
            Self::SearchTtl => "search_ttl",
            Self::SearchError => "search_error",
        }
    }

    /// Inclusive `(min, max)` the field is checked against at load and clamped to after
    /// multiplying (ai.md; a record format limit, not a tunable).
    pub fn range(self) -> (f32, f32) {
        match self {
            Self::SightCone | Self::Darkness => (0.0, 2.0),
            Self::SightRange
            | Self::SightGain
            | Self::Hearing
            | Self::Alertness
            | Self::Decay
            | Self::SearchTtl
            | Self::SearchError => (0.0, 4.0),
        }
    }

    fn contains(self, v: f32) -> bool {
        let (lo, hi) = self.range();
        (lo..=hi).contains(&v)
    }

    fn clamp(self, v: f32) -> f32 {
        let (lo, hi) = self.range();
        // A NaN product (never from validated records) falls back to the neutral ratio.
        if v.is_nan() { IDENTITY } else { v.clamp(lo, hi) }
    }
}

/// A field (or `duration_s`) outside its range, found by `validate`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutOfRange {
    /// The field's record name.
    pub field: &'static str,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

/// The out-of-range fields of a profile or state record, by name.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[error("perception fields out of range: {}", list(.fields))]
pub struct PerceptionRangeError {
    pub fields: Vec<OutOfRange>,
}

impl PerceptionRangeError {
    /// The offending field names, in field order.
    pub fn names(&self) -> Vec<&'static str> {
        self.fields.iter().map(|f| f.field).collect()
    }
}

fn list(fields: &[OutOfRange]) -> String {
    fields.iter().map(|f| format!("{} = {} ({}-{})", f.field, f.value, f.min, f.max)).collect::<Vec<_>>().join(", ")
}

fn check(fields: Vec<OutOfRange>) -> Result<(), PerceptionRangeError> {
    if fields.is_empty() { Ok(()) } else { Err(PerceptionRangeError { fields }) }
}

fn out_of_range(field: PerceptionField, value: f32) -> OutOfRange {
    let (min, max) = field.range();
    OutOfRange { field: field.name(), value, min, max }
}

/// An actor's perception multipliers on the global tunables (ai.md "Perception profile"). A record
/// type (presets: a race, creature type, role) and the actor's resolved input; missing fields read
/// 1.0, unknown fields are a load error.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PerceptionProfile {
    pub sight_range: f32,
    pub sight_cone: f32,
    pub sight_gain: f32,
    pub darkness: f32,
    pub hearing: f32,
    pub alertness: f32,
    pub decay: f32,
    pub search_ttl: f32,
    pub search_error: f32,
}

impl Default for PerceptionProfile {
    fn default() -> Self {
        Self {
            sight_range: IDENTITY,
            sight_cone: IDENTITY,
            sight_gain: IDENTITY,
            darkness: IDENTITY,
            hearing: IDENTITY,
            alertness: IDENTITY,
            decay: IDENTITY,
            search_ttl: IDENTITY,
            search_error: IDENTITY,
        }
    }
}

impl PerceptionProfile {
    /// One field's value.
    pub fn get(&self, field: PerceptionField) -> f32 {
        match field {
            PerceptionField::SightRange => self.sight_range,
            PerceptionField::SightCone => self.sight_cone,
            PerceptionField::SightGain => self.sight_gain,
            PerceptionField::Darkness => self.darkness,
            PerceptionField::Hearing => self.hearing,
            PerceptionField::Alertness => self.alertness,
            PerceptionField::Decay => self.decay,
            PerceptionField::SearchTtl => self.search_ttl,
            PerceptionField::SearchError => self.search_error,
        }
    }

    fn set(&mut self, field: PerceptionField, v: f32) {
        let slot = match field {
            PerceptionField::SightRange => &mut self.sight_range,
            PerceptionField::SightCone => &mut self.sight_cone,
            PerceptionField::SightGain => &mut self.sight_gain,
            PerceptionField::Darkness => &mut self.darkness,
            PerceptionField::Hearing => &mut self.hearing,
            PerceptionField::Alertness => &mut self.alertness,
            PerceptionField::Decay => &mut self.decay,
            PerceptionField::SearchTtl => &mut self.search_ttl,
            PerceptionField::SearchError => &mut self.search_error,
        };
        *slot = v;
    }

    /// The load check: every field inside its range (NaN is out of range).
    ///
    /// # Errors
    /// Lists every out-of-range field by name.
    pub fn validate(&self) -> Result<(), PerceptionRangeError> {
        check(
            PerceptionField::ALL
                .into_iter()
                .filter(|&f| !f.contains(self.get(f)))
                .map(|f| out_of_range(f, self.get(f)))
                .collect(),
        )
    }
}

/// A temporary perception state record (ai.md "Temporary states"): the profile's fields, each
/// optional (missing = 1), and how long a push lasts when the pusher gives no duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PerceptionState {
    pub sight_range: Option<f32>,
    pub sight_cone: Option<f32>,
    pub sight_gain: Option<f32>,
    pub darkness: Option<f32>,
    pub hearing: Option<f32>,
    pub alertness: Option<f32>,
    pub decay: Option<f32>,
    pub search_ttl: Option<f32>,
    pub search_error: Option<f32>,
    /// Seconds a push lasts; `None` = a condition the adapter holds and clears.
    pub duration_s: Option<f32>,
}

impl PerceptionState {
    /// One field's value, if the record sets it.
    pub fn get(&self, field: PerceptionField) -> Option<f32> {
        match field {
            PerceptionField::SightRange => self.sight_range,
            PerceptionField::SightCone => self.sight_cone,
            PerceptionField::SightGain => self.sight_gain,
            PerceptionField::Darkness => self.darkness,
            PerceptionField::Hearing => self.hearing,
            PerceptionField::Alertness => self.alertness,
            PerceptionField::Decay => self.decay,
            PerceptionField::SearchTtl => self.search_ttl,
            PerceptionField::SearchError => self.search_error,
        }
    }

    /// The tick a push of this state at `now` expires on: `now + ceil(duration_s / tick_s)`, or
    /// `None` (a condition) when the record has no valid duration.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // finite, ≥ 0, ceil'd
    pub fn until(&self, now: PerceptClock) -> Option<u64> {
        let d = self.duration_s.filter(|d| d.is_finite() && *d >= 0.0)?;
        if !(now.tick_s.is_finite() && now.tick_s > 0.0) {
            return None;
        }
        Some(now.tick.saturating_add((d / now.tick_s).ceil() as u64))
    }

    /// The load check: every set field inside its range, `duration_s` finite and ≥ 0.
    ///
    /// # Errors
    /// Lists every out-of-range field by name.
    pub fn validate(&self) -> Result<(), PerceptionRangeError> {
        let mut bad: Vec<OutOfRange> = PerceptionField::ALL
            .into_iter()
            .filter_map(|f| self.get(f).filter(|&v| !f.contains(v)).map(|v| out_of_range(f, v)))
            .collect();
        if let Some(d) = self.duration_s.filter(|d| !(d.is_finite() && *d >= 0.0)) {
            bad.push(OutOfRange { field: "duration_s", value: d, min: 0.0, max: f32::INFINITY });
        }
        check(bad)
    }
}

/// One active state on an actor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateEntry {
    /// The [`PerceptionState`] record.
    pub state: RecordId,
    /// `Some(tick)`: timed, expires once the host tick reaches it. `None`: a condition.
    pub until: Option<u64>,
    /// Who pushed it (an item, spell, directive record), for the trace and targeted clears.
    pub source: Option<RecordId>,
}

/// What [`PerceptionStates::push`] did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PushOutcome {
    /// Took a free slot.
    Added,
    /// The state was already active: `until` is now the later of the two (never stacks).
    Refreshed,
    /// The array was full: this entry (the one expiring first) was replaced.
    Replaced(StateEntry),
    /// The array was full of conditions: nothing changed (a condition is never evicted; the
    /// adapter that holds it clears it).
    Full,
}

/// Sort key of an expiry: a condition never expires, so it is the latest.
fn expiry_key(until: Option<u64>) -> u64 {
    until.unwrap_or(u64::MAX)
}

/// An actor's active perception states (ai.md "Temporary states"): inline, at most
/// [`PERCEPTION_STATES_CAPACITY`], no heap of its own.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerceptionStates {
    slots: [Option<StateEntry>; PERCEPTION_STATES_CAPACITY],
}

impl PerceptionStates {
    pub fn new() -> Self {
        Self::default()
    }

    /// The active entries, in slot order.
    pub fn iter(&self) -> impl Iterator<Item = &StateEntry> {
        self.slots.iter().flatten()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }

    /// The active entry for `state`, if any.
    pub fn get(&self, state: &RecordId) -> Option<&StateEntry> {
        self.iter().find(|e| &e.state == state)
    }

    /// Pushes `state` until tick `until` (`None` = a condition). The same state refreshes: the
    /// later `until` wins (a condition counts as latest) and takes the new `source`; otherwise
    /// the old entry is kept. A new state takes a free slot or, when full, replaces the entry
    /// expiring first (earliest `until`, lowest slot on a tie); a full array of conditions refuses.
    pub fn push(&mut self, state: RecordId, until: Option<u64>, source: Option<RecordId>) -> PushOutcome {
        if let Some(e) = self.slots.iter_mut().flatten().find(|e| e.state == state) {
            if expiry_key(until) > expiry_key(e.until) {
                e.until = until;
                e.source = source;
            }
            return PushOutcome::Refreshed;
        }
        let entry = StateEntry { state, until, source };
        if let Some(free) = self.slots.iter_mut().find(|s| s.is_none()) {
            *free = Some(entry);
            return PushOutcome::Added;
        }
        let first = self
            .slots
            .iter_mut()
            .filter(|s| s.as_ref().is_some_and(|e| e.until.is_some()))
            .min_by_key(|s| s.as_ref().map_or(u64::MAX, |e| expiry_key(e.until)));
        match first {
            Some(slot) => PushOutcome::Replaced(slot.replace(entry).expect("filtered to occupied slots")),
            None => PushOutcome::Full,
        }
    }

    /// Removes `state` (timed or condition); `true` if it was active.
    pub fn clear(&mut self, state: &RecordId) -> bool {
        let mut hit = false;
        for s in &mut self.slots {
            if s.as_ref().is_some_and(|e| &e.state == state) {
                *s = None;
                hit = true;
            }
        }
        hit
    }

    /// Removes every timed entry whose `until ≤ now`; returns how many. Conditions never expire.
    pub fn expire(&mut self, now: u64) -> usize {
        let mut n = 0;
        for s in &mut self.slots {
            if s.as_ref().is_some_and(|e| e.until.is_some_and(|t| t <= now)) {
                *s = None;
                n += 1;
            }
        }
        n
    }

    /// The earliest `until` of the timed entries (`None` if there are none).
    pub fn next_expiry(&self) -> Option<u64> {
        self.iter().filter_map(|e| e.until).min()
    }
}

/// Base sight tunables (`ai.perception.*`, bound in `sf_ai_bevy`; ai.md "Tunables" table).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SightConfig {
    /// Sight range (UE `SightRadius`), meters (range 1-200).
    pub sight_range_m: f32,
    /// A seen target is lost beyond this (UE `LoseSightRadius`), meters (range 1-200).
    pub lose_sight_range_m: f32,
    /// Full visibility distance for `rate`, meters (range 1-200; never scaled by a profile).
    pub near_m: f32,
    /// Central cone half-angle, degrees (range 0-180).
    pub central_half_deg: f32,
    /// Peripheral cone half-angle, degrees (range 0-180).
    pub peripheral_half_deg: f32,
    /// Gain multiplier in the peripheral band, ratio (range 0-1).
    pub peripheral_mul: f32,
    /// Sight gain at full visibility at `near_m`, 1/s (range 0.05-20).
    pub rate: f32,
}

impl Default for SightConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.perception.sight_range_m in sf_ai_bevy; Guess)
            sight_range_m: 30.0,
            // tunables: const (bound from ai.perception.lose_sight_range_m in sf_ai_bevy; Guess)
            lose_sight_range_m: 36.0,
            // tunables: const (bound from ai.perception.near_m in sf_ai_bevy; Guess)
            near_m: 3.0,
            // tunables: const (bound from ai.perception.central_half_deg in sf_ai_bevy; Guess)
            central_half_deg: 30.0,
            // tunables: const (bound from ai.perception.peripheral_half_deg in sf_ai_bevy; Guess)
            peripheral_half_deg: 80.0,
            // tunables: const (bound from ai.perception.peripheral_mul in sf_ai_bevy; Guess)
            peripheral_mul: 0.35,
            // tunables: const (bound from ai.perception.rate in sf_ai_bevy; Guess)
            rate: 2.0,
        }
    }
}

/// Base search tunables (`ai.perception.*`, `AI8f`'s globals the profile scales; bound in `sf_ai_bevy`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchConfig {
    /// How long a search persists, seconds (range 0-300).
    pub search_ttl_s: f32,
    /// Initial error of the searched position, meters (range 0-30).
    pub search_error_m: f32,
    /// How fast the search area widens, m/s (range 0-5).
    pub search_widen_mps: f32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.perception.search_ttl_s in sf_ai_bevy; Guess)
            search_ttl_s: 30.0,
            // tunables: const (bound from ai.perception.search_error_m in sf_ai_bevy; Guess)
            search_error_m: 3.0,
            // tunables: const (bound from ai.perception.search_widen_mps in sf_ai_bevy; Guess)
            search_widen_mps: 0.5,
        }
    }
}

/// An actor's resolved sight parameters.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SightParams {
    /// `sight_range_m` × `sight_range`, meters.
    pub range_m: f32,
    /// `lose_sight_range_m` × `sight_range`, meters.
    pub lose_m: f32,
    /// `central_half_deg` × `sight_cone`, ≤ 180.
    pub central_half_deg: f32,
    /// `peripheral_half_deg` × `sight_cone`, ≤ 180.
    pub peripheral_half_deg: f32,
    /// `rate` × `sight_gain` × `alertness`, 1/s.
    pub rate: f32,
    /// The darkness ratio: `AI8b` feeds `light' = 1 - (1 - light) × darkness`.
    pub darkness: f32,
}

/// An actor's resolved search parameters (read by `AI8f`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchParams {
    /// `search_ttl_s` × `search_ttl`, seconds.
    pub ttl_s: f32,
    /// `search_error_m` × `search_error`, meters.
    pub error_m: f32,
    /// `search_widen_mps` × `search_error`, m/s.
    pub widen_mps: f32,
}

/// The cached resolved view of a profile and its states (ai.md "Hook"): callers pass `&eff.cfg`
/// where `AI8a` takes `&AwarenessConfig`. Rebuilt only when the inputs change, a tunable reloads, or
/// [`Self::needs_rebuild`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectivePerception {
    /// The global awareness config with `decay_*` × `decay` and `noise_gain` × `alertness`.
    pub cfg: AwarenessConfig,
    pub sight: SightParams,
    /// × a noise's reach (`HeardNoise.loudness_m`).
    pub hearing_mul: f32,
    pub search: SearchParams,
    /// The earliest tick a counted state expires (`None`: nothing timed).
    pub next_expiry: Option<u64>,
}

impl EffectivePerception {
    /// One compare per actor per tick: a counted state has expired by `now`.
    pub fn needs_rebuild(&self, now: u64) -> bool {
        self.next_expiry.is_some_and(|t| t <= now)
    }
}

/// Resolves `profile` × Π active `states` (each looked up by id; an unknown id or an entry already
/// expired at `now` counts as 1), clamps each field to its range and scales the global configs.
/// Band thresholds, `band_hysteresis`, `alert_hold_s`, `noise_cap`, `max_entries` and `near_m` are
/// never scaled.
pub fn resolve<'a>(
    profile: &PerceptionProfile,
    states: &PerceptionStates,
    lookup_state: impl Fn(&RecordId) -> Option<&'a PerceptionState>,
    cfg: &AwarenessConfig,
    sight: &SightConfig,
    search: &SearchConfig,
    now: u64,
) -> EffectivePerception {
    let mut eff = *profile;
    let mut next_expiry: Option<u64> = None;
    for entry in states.iter().filter(|e| e.until.is_none_or(|t| t > now)) {
        let Some(rec) = lookup_state(&entry.state) else { continue };
        if let Some(t) = entry.until {
            next_expiry = Some(next_expiry.map_or(t, |n| n.min(t)));
        }
        for f in PerceptionField::ALL {
            if let Some(m) = rec.get(f) {
                eff.set(f, eff.get(f) * m);
            }
        }
    }
    for f in PerceptionField::ALL {
        eff.set(f, f.clamp(eff.get(f)));
    }

    let mut out_cfg = *cfg;
    out_cfg.decay_curious *= eff.decay;
    out_cfg.decay_suspicious *= eff.decay;
    out_cfg.decay_searching *= eff.decay;
    out_cfg.noise_gain *= eff.alertness;

    EffectivePerception {
        cfg: out_cfg,
        sight: SightParams {
            range_m: sight.sight_range_m * eff.sight_range,
            lose_m: sight.lose_sight_range_m * eff.sight_range,
            central_half_deg: (sight.central_half_deg * eff.sight_cone).min(MAX_HALF_DEG),
            peripheral_half_deg: (sight.peripheral_half_deg * eff.sight_cone).min(MAX_HALF_DEG),
            rate: sight.rate * eff.sight_gain * eff.alertness,
            darkness: eff.darkness,
        },
        hearing_mul: eff.hearing,
        search: SearchParams {
            ttl_s: search.search_ttl_s * eff.search_ttl,
            error_m: search.search_error_m * eff.search_error,
            widen_mps: search.search_widen_mps * eff.search_error,
        },
        next_expiry,
    }
}
