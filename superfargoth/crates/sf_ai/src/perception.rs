//! `AI8a`: the awareness core (`ai.md` "Perception and stimuli" → "Awareness"). Pure, Bevy-free,
//! host-only, on the fixed tick.
//!
//! - [`AwarenessMeter`], one per actor: bounded [`Awareness`] entries (one per [`PerceptTarget`],
//!   at most [`AwarenessConfig::max_entries`], the lowest evicted), one [`Distraction`] slot
//!   (loudest recent wins), and the asleep / groggy state.
//! - [`Stimulus`] is the meter's input: sight gain (the gain formula, cone, rays and light are
//!   `AI8b`, `AI8d`, so it arrives already computed), a heard noise (propagation is `AI8c`, so it
//!   arrives with its attenuated reach and path distance), touch and damage.
//! - Five [`AwarenessBand`]s with hysteresis: a band is entered at its threshold and left below
//!   it minus `band_hysteresis`; Alerted (level 1) is latched and holds while the target is in
//!   contact, then drops to the Searching floor `alert_hold_s` after the last contact. Decay
//!   starts `decay_delay_s` after the last gain, at the current band's rate, integrated across
//!   band changes so a tier that updates less often ends at the same level (tier invariance).
//! - [`player_detection`]: the HUD sneak eye's `PlayerDetection { level, band }` for one player.
//!
//! Not saved (ai.md "Saves"): no serde on the meter; [`AwarenessMeter::raise_to_floor`] seeds a
//! loaded directive's band floor. Only [`PlayerDetection`] and [`AwarenessBand`] carry serde (the
//! replicated HUD value, ai.md "Net").

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};

// tunables: const (ai.md: Alerted is level 1, the top of the 0..1 meter)
const FULL: f32 = 1.0;
// tunables: const (ai.md: a sighting, touch or hit places the target exactly; confidence is 0..1)
const CERTAIN: f32 = 1.0;

/// Who an [`Awareness`] entry is about (ai.md "Stimuli" → Targets). Every player, local or
/// remote, is one target; actors only when the shell flags them (hostiles, companions).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PerceptTarget {
    Player(PlayerKey),
    // The spec writes `ActorRef`; the crate's actor reference is `ActorKey` (as in `DirectiveKind`).
    Actor(ActorKey),
}

/// The five awareness bands, lowest first (ai.md "Awareness").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AwarenessBand {
    #[default]
    Unaware,
    /// Look toward.
    Curious,
    /// Turn, step toward the last known position.
    Suspicious,
    /// HTN `search` at the last known position (`AI8f`).
    Searching,
    /// Level 1, latched: drives pick combat, report or flee.
    Alerted,
}

impl AwarenessBand {
    /// The next band down (Unaware stays Unaware).
    fn below(self) -> Self {
        match self {
            Self::Unaware | Self::Curious => Self::Unaware,
            Self::Suspicious => Self::Curious,
            Self::Searching => Self::Suspicious,
            Self::Alerted => Self::Searching,
        }
    }
}

/// Where and how surely the target was last placed (ai.md `LastSeen`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LastSeen {
    pub pos: WorldPos,
    /// Logical tick of the placement.
    pub tick: u64,
    /// Target velocity at the placement, m/s (zero for noise, touch and damage).
    pub velocity: Vec3,
    /// 0..1: 1 for sight, touch and damage, `noise_confidence` for a revealing noise.
    pub confidence: f32,
}

/// One actor's awareness of one target (ai.md `Awareness`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Awareness {
    pub target: PerceptTarget,
    /// 0..1.
    pub level: f32,
    pub band: AwarenessBand,
    pub last_seen: LastSeen,
    /// Tick of the last positive gain; decay starts `decay_delay_s` after it.
    pub last_gain: u64,
    /// Tick of the last sight, touch or damage contact; the Alerted hold counts from it.
    pub last_contact: Option<u64>,
    /// Tick this entry was last advanced to (decay and hold are integrated from here).
    pub updated: u64,
}

/// The investigate-the-spot slot filled by non-revealing noises (ai.md `Distraction`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distraction {
    pub at: WorldPos,
    pub tick: u64,
    /// The attenuated reach of the noise that filled it, meters (louder wins).
    pub loudness_m: f32,
}

/// The logical clock the meter runs on: the fixed tick and its length (not a tunable: the sim's
/// fixed timestep).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PerceptClock {
    pub tick: u64,
    pub tick_s: f32,
}

impl PerceptClock {
    pub fn new(tick: u64, tick_s: f32) -> Self {
        Self { tick, tick_s }
    }

    /// Seconds from `earlier` to now (0 if `earlier` is in the future).
    #[allow(clippy::cast_precision_loss)] // tick deltas are small; f32 seconds suffice
    fn since(self, earlier: u64) -> f32 {
        self.tick.saturating_sub(earlier) as f32 * self.tick_s
    }
}

/// A noise as one listener hears it, after propagation (`AI8c` computes both distances).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeardNoise {
    /// Who made it, if anyone.
    pub source: Option<PerceptTarget>,
    /// Footsteps reveal their source; a thrown cup does not.
    pub reveals_source: bool,
    /// Where it happened (where a thrown object landed).
    pub at: WorldPos,
    /// Attenuated reach: `loudness_m × hearing_mul × weather_mask`, meters.
    pub loudness_m: f32,
    /// `path_distance(at → listener)` plus `closed_door_m` per closed door, meters.
    pub distance_m: f32,
}

/// One perception input to an [`AwarenessMeter`] (ai.md "Stimuli").
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stimulus {
    /// A sight check's gain (`rate × dt × f_dist × f_cone × visibility × alertness`, `AI8b`)
    /// with the target's position and velocity.
    Sight { target: PerceptTarget, gain: f32, pos: WorldPos, velocity: Vec3 },
    /// A heard noise.
    Noise(HeardNoise),
    /// Bumped into: always perceived, always wakes.
    Touch { by: PerceptTarget, at: WorldPos },
    /// Hurt: always perceived, always wakes; `by` is `None` for sourceless damage.
    Damage { by: Option<PerceptTarget>, at: WorldPos },
}

/// What [`AwarenessMeter::apply`] did with a stimulus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StimulusOutcome {
    /// The target's entry was raised (or refreshed) to this band.
    Raised(AwarenessBand),
    /// A non-revealing noise filled the [`Distraction`] slot.
    Distraction,
    /// A non-revealing noise was quieter than the current distraction.
    Ignored,
    /// Not heard (beyond reach), or too faint to wake a sleeper.
    Unheard,
    /// Asleep: sight is not perceived.
    Asleep,
    /// The table was full of stronger entries: the new target was evicted at once.
    Evicted,
    /// Sourceless damage: woke the actor, no entry.
    Woke,
    /// NaN, infinite or negative input: nothing changed.
    Invalid,
}

/// Awareness tunables (`ai.awareness.*`, bound in `sf_ai_bevy`; ai.md "Tunables" table).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwarenessConfig {
    /// Level at which Curious is entered.
    pub curious: f32,
    /// Level at which Suspicious is entered.
    pub suspicious: f32,
    /// Level at which Searching is entered (also the floor Alerted drops to).
    pub searching: f32,
    /// A band is left below its threshold minus this.
    pub band_hysteresis: f32,
    /// Seconds Alerted holds after the last contact before dropping to the Searching floor.
    pub alert_hold_s: f32,
    /// Seconds after the last gain before decay starts.
    pub decay_delay_s: f32,
    /// Decay in Curious (and Unaware), 1/s.
    pub decay_curious: f32,
    /// Decay in Suspicious, 1/s.
    pub decay_suspicious: f32,
    /// Decay in Searching (after the Alerted hold), 1/s.
    pub decay_searching: f32,
    /// Gain of a revealing noise at zero distance.
    pub noise_gain: f32,
    /// Hearing alone never raises a level above this.
    pub noise_cap: f32,
    /// `LastSeen::confidence` of a revealing noise's position.
    pub noise_confidence: f32,
    /// A sleeper wakes only to a noise with at least this much reach left at its ears, meters.
    pub asleep_wake_m: f32,
    /// Seconds of low alertness after waking.
    pub groggy_s: f32,
    /// Entries per actor (the lowest evicted).
    pub max_entries: usize,
    /// Spec gap: how long a distraction stays "recent", seconds (then any noise replaces it and
    /// [`AwarenessMeter::update`] clears it).
    pub distraction_ttl_s: f32,
    /// Spec gap ("alertness higher from Suspicious up"): sight alertness multiplier toward a
    /// target at Suspicious or above.
    pub alert_alertness: f32,
    /// Spec gap ("lower while groggy"): sight alertness multiplier while groggy.
    pub groggy_alertness: f32,
}

impl Default for AwarenessConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.awareness.curious in sf_ai_bevy)
            curious: 0.1,
            // tunables: const (bound from ai.awareness.suspicious in sf_ai_bevy)
            suspicious: 0.35,
            // tunables: const (bound from ai.awareness.searching in sf_ai_bevy)
            searching: 0.65,
            // tunables: const (bound from ai.awareness.band_hysteresis in sf_ai_bevy)
            band_hysteresis: 0.05,
            // tunables: const (bound from ai.awareness.alert_hold_s in sf_ai_bevy)
            alert_hold_s: 6.0,
            // tunables: const (bound from ai.awareness.decay_delay_s in sf_ai_bevy)
            decay_delay_s: 3.0,
            // tunables: const (bound from ai.awareness.decay_curious in sf_ai_bevy)
            decay_curious: 0.10,
            // tunables: const (bound from ai.awareness.decay_suspicious in sf_ai_bevy)
            decay_suspicious: 0.05,
            // tunables: const (bound from ai.awareness.decay_searching in sf_ai_bevy)
            decay_searching: 0.02,
            // tunables: const (bound from ai.awareness.noise_gain in sf_ai_bevy)
            noise_gain: 0.5,
            // tunables: const (bound from ai.awareness.noise_cap in sf_ai_bevy)
            noise_cap: 0.65,
            // tunables: const (bound from ai.awareness.noise_confidence in sf_ai_bevy)
            noise_confidence: 0.5,
            // tunables: const (bound from ai.awareness.asleep_wake_m in sf_ai_bevy)
            asleep_wake_m: 12.0,
            // tunables: const (bound from ai.awareness.groggy_s in sf_ai_bevy)
            groggy_s: 4.0,
            // tunables: const (bound from ai.awareness.max_entries in sf_ai_bevy)
            max_entries: 8,
            // tunables: const (bound from ai.awareness.distraction_ttl_s in sf_ai_bevy; Guess)
            distraction_ttl_s: 10.0,
            // tunables: const (bound from ai.awareness.alert_alertness in sf_ai_bevy; Guess)
            alert_alertness: 1.5,
            // tunables: const (bound from ai.awareness.groggy_alertness in sf_ai_bevy; Guess)
            groggy_alertness: 0.5,
        }
    }
}

impl AwarenessConfig {
    /// The level at which `band` is entered.
    pub fn enter(&self, band: AwarenessBand) -> f32 {
        match band {
            AwarenessBand::Unaware => 0.0,
            AwarenessBand::Curious => self.curious,
            AwarenessBand::Suspicious => self.suspicious,
            AwarenessBand::Searching => self.searching,
            AwarenessBand::Alerted => FULL,
        }
    }

    /// The level below which `band` is left (its threshold minus `band_hysteresis`; Unaware is
    /// never left downward, Alerted only by the hold).
    fn stay(&self, band: AwarenessBand) -> f32 {
        match band {
            AwarenessBand::Unaware => 0.0,
            b => self.enter(b) - self.band_hysteresis,
        }
    }

    /// Decay rate of `band`, 1/s (Unaware decays at the Curious rate; Alerted is latched).
    fn decay(&self, band: AwarenessBand) -> f32 {
        match band {
            AwarenessBand::Unaware | AwarenessBand::Curious => self.decay_curious,
            AwarenessBand::Suspicious => self.decay_suspicious,
            AwarenessBand::Searching => self.decay_searching,
            AwarenessBand::Alerted => 0.0,
        }
    }

    /// The highest band whose threshold `level` reaches.
    pub fn band_for(&self, level: f32) -> AwarenessBand {
        [AwarenessBand::Alerted, AwarenessBand::Searching, AwarenessBand::Suspicious, AwarenessBand::Curious]
            .into_iter()
            .find(|&b| level >= self.enter(b))
            .unwrap_or(AwarenessBand::Unaware)
    }

    /// The band after a level change from band `current` (hysteresis): up to the highest band
    /// entered, or down to the highest band at or below `current` whose stay level still holds.
    /// Alerted is latched: only [`AwarenessMeter::update`]'s hold drops it.
    pub fn next_band(&self, current: AwarenessBand, level: f32) -> AwarenessBand {
        let up = self.band_for(level);
        if up >= current || current == AwarenessBand::Alerted {
            return up.max(current);
        }
        let mut b = current;
        while b != AwarenessBand::Unaware && level < self.stay(b) {
            b = b.below();
        }
        b
    }

    /// The gain a heard noise gives its source's entry: `noise_gain × (1 - d / reach)`, or
    /// `None` when unheard (`d > reach`) or invalid.
    pub fn noise_gain(&self, noise: &HeardNoise) -> Option<f32> {
        let (reach, d) = (noise.loudness_m, noise.distance_m);
        if !valid(reach) || !valid(d) || reach <= 0.0 || d > reach {
            return None;
        }
        Some(self.noise_gain * (1.0 - d / reach))
    }
}

fn valid(x: f32) -> bool {
    x.is_finite() && x >= 0.0
}

fn valid_pos(p: WorldPos) -> bool {
    p.local.is_finite()
}

impl Awareness {
    fn new(target: PerceptTarget, last_seen: LastSeen, now: u64) -> Self {
        Self {
            target,
            level: 0.0,
            band: AwarenessBand::Unaware,
            last_seen,
            last_gain: now,
            last_contact: None,
            updated: now,
        }
    }

    /// Advances hold and decay from `updated` to `now`.
    fn settle(&mut self, cfg: &AwarenessConfig, now: PerceptClock) {
        let since_update = now.since(self.updated);
        let since_gain = now.since(self.last_gain);
        let mut decay_s = since_update.min(since_gain - cfg.decay_delay_s).max(0.0);
        if self.band == AwarenessBand::Alerted {
            let held = self.last_contact.map_or(f32::INFINITY, |c| now.since(c) - cfg.alert_hold_s);
            if held >= 0.0 {
                // Dropped `held` seconds ago: decay only counts from the drop on.
                self.band = AwarenessBand::Searching;
                self.level = cfg.searching;
                decay_s = decay_s.min(held);
            } else {
                decay_s = 0.0;
            }
        }
        self.decay(cfg, decay_s);
        self.updated = self.updated.max(now.tick);
    }

    /// Integrates `t` seconds of decay, changing band at each stay level it crosses.
    fn decay(&mut self, cfg: &AwarenessConfig, mut t: f32) {
        while t > 0.0 && self.level > 0.0 {
            let rate = cfg.decay(self.band);
            if rate.is_nan() || rate <= 0.0 {
                return;
            }
            let floor = cfg.stay(self.band).max(0.0);
            let need = (self.level - floor) / rate;
            if t <= need {
                self.level = (self.level - rate * t).max(floor);
                return;
            }
            t -= need;
            self.level = floor;
            if self.band == AwarenessBand::Unaware {
                return;
            }
            self.band = self.band.below();
        }
    }

    /// Adds `gain` (capped at `cap`, never lowering), places the target and re-bands.
    fn gain(&mut self, cfg: &AwarenessConfig, gain: f32, cap: f32, seen: LastSeen, now: u64) {
        if self.level < cap {
            self.level = (self.level + gain).min(cap);
        }
        if gain > 0.0 {
            self.last_gain = now;
        }
        self.last_seen = seen;
        self.band = cfg.next_band(self.band, self.level);
    }
}

/// One actor's perception memory (ai.md "Awareness"). Not saved.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AwarenessMeter {
    /// Sorted by target (deterministic iteration).
    entries: Vec<Awareness>,
    distraction: Option<Distraction>,
    asleep: bool,
    /// Tick of the last wake (groggy for `groggy_s` from it).
    woke: Option<u64>,
}

impl AwarenessMeter {
    pub fn new() -> Self {
        Self::default()
    }

    /// All entries, sorted by target.
    pub fn entries(&self) -> &[Awareness] {
        &self.entries
    }

    pub fn get(&self, target: PerceptTarget) -> Option<&Awareness> {
        self.entries.iter().find(|e| e.target == target)
    }

    /// The band toward `target` (Unaware if untracked).
    pub fn band(&self, target: PerceptTarget) -> AwarenessBand {
        self.get(target).map_or(AwarenessBand::Unaware, |e| e.band)
    }

    /// The strongest entry: highest band, then level, then most recent gain, then lowest target.
    pub fn strongest(&self) -> Option<&Awareness> {
        self.entries.iter().max_by(|a, b| {
            a.band
                .cmp(&b.band)
                .then(a.level.total_cmp(&b.level))
                .then(a.last_gain.cmp(&b.last_gain))
                .then(b.target.cmp(&a.target))
        })
    }

    pub fn distraction(&self) -> Option<&Distraction> {
        self.distraction.as_ref()
    }

    /// Takes the distraction (the investigate behaviour consumed it).
    pub fn take_distraction(&mut self) -> Option<Distraction> {
        self.distraction.take()
    }

    pub fn asleep(&self) -> bool {
        self.asleep
    }

    /// Bed activity starts (no sight; only loud noise, touch and damage wake).
    pub fn fall_asleep(&mut self) {
        self.asleep = true;
    }

    /// Wakes the actor (groggy for `groggy_s`). No-op when awake.
    pub fn wake(&mut self, now: u64) {
        if self.asleep {
            self.asleep = false;
            self.woke = Some(now);
        }
    }

    /// Within `groggy_s` of waking.
    pub fn groggy(&self, cfg: &AwarenessConfig, now: PerceptClock) -> bool {
        self.woke.is_some_and(|w| now.since(w) < cfg.groggy_s)
    }

    /// The `alertness` factor of the sight gain toward `target` (`AI8b` multiplies it in):
    /// `groggy_alertness` while groggy, `alert_alertness` from Suspicious up, else 1.
    pub fn alertness(&self, cfg: &AwarenessConfig, target: PerceptTarget, now: PerceptClock) -> f32 {
        if self.groggy(cfg, now) {
            cfg.groggy_alertness
        } else if self.band(target) >= AwarenessBand::Suspicious {
            cfg.alert_alertness
        } else {
            1.0
        }
    }

    /// Advances every entry to `now` (hold, decay), drops entries decayed to 0 and expires an
    /// old distraction. Call once per perception tick; [`Self::apply`] settles its own entry.
    pub fn update(&mut self, cfg: &AwarenessConfig, now: PerceptClock) {
        for e in &mut self.entries {
            e.settle(cfg, now);
        }
        // Spec gap: an Unaware entry at level 0 is forgotten (LKP included).
        self.entries.retain(|e| e.level > 0.0 || e.band != AwarenessBand::Unaware);
        if self.distraction.is_some_and(|d| now.since(d.tick) >= cfg.distraction_ttl_s) {
            self.distraction = None;
        }
    }

    /// Applies one stimulus at `now`.
    pub fn apply(&mut self, cfg: &AwarenessConfig, stimulus: &Stimulus, now: PerceptClock) -> StimulusOutcome {
        let t = now.tick;
        match *stimulus {
            Stimulus::Sight { target, gain, pos, velocity } => {
                if !valid(gain) || !valid_pos(pos) || !velocity.is_finite() {
                    return StimulusOutcome::Invalid;
                }
                if self.asleep {
                    return StimulusOutcome::Asleep;
                }
                let seen = LastSeen { pos, tick: t, velocity, confidence: CERTAIN };
                self.raise(cfg, target, seen, now, |e| {
                    e.gain(cfg, gain, FULL, seen, t);
                    e.last_contact = Some(t);
                })
            }
            Stimulus::Noise(noise) => self.hear(cfg, &noise, now),
            Stimulus::Touch { by, at } => {
                if !valid_pos(at) {
                    return StimulusOutcome::Invalid;
                }
                self.wake(t);
                let seen = LastSeen { pos: at, tick: t, velocity: Vec3::ZERO, confidence: CERTAIN };
                // Spec gap: touch knows someone is there but not who means harm: Searching floor.
                self.raise(cfg, by, seen, now, |e| {
                    e.floor(cfg, AwarenessBand::Searching, seen, t);
                    e.last_contact = Some(t);
                })
            }
            Stimulus::Damage { by, at } => {
                if !valid_pos(at) {
                    return StimulusOutcome::Invalid;
                }
                self.wake(t);
                let Some(by) = by else { return StimulusOutcome::Woke };
                let seen = LastSeen { pos: at, tick: t, velocity: Vec3::ZERO, confidence: CERTAIN };
                // Spec gap: being hit by a known attacker alerts at once (counts as contact).
                self.raise(cfg, by, seen, now, |e| {
                    e.floor(cfg, AwarenessBand::Alerted, seen, t);
                    e.last_contact = Some(t);
                })
            }
        }
    }

    /// Raises `target` to at least `band`'s floor (a loaded `Search`/`Report`/combat directive
    /// starts its actor there, ai.md "Saves"). Alerted counts as a contact at `now`.
    pub fn raise_to_floor(
        &mut self,
        cfg: &AwarenessConfig,
        target: PerceptTarget,
        band: AwarenessBand,
        last_seen: LastSeen,
        now: PerceptClock,
    ) -> StimulusOutcome {
        if !valid_pos(last_seen.pos) || !last_seen.velocity.is_finite() || !valid(last_seen.confidence) {
            return StimulusOutcome::Invalid;
        }
        let t = now.tick;
        self.raise(cfg, target, last_seen, now, |e| {
            e.floor(cfg, band, last_seen, t);
            if band == AwarenessBand::Alerted {
                e.last_contact = Some(t);
            }
        })
    }

    fn hear(&mut self, cfg: &AwarenessConfig, noise: &HeardNoise, now: PerceptClock) -> StimulusOutcome {
        if !valid(noise.loudness_m) || !valid(noise.distance_m) || !valid_pos(noise.at) {
            return StimulusOutcome::Invalid;
        }
        let Some(gain) = cfg.noise_gain(noise) else { return StimulusOutcome::Unheard };
        if self.asleep {
            // Spec "only noise received at ≥ asleep_wake_m wakes": read as the reach left at the
            // sleeper's ears (`loudness_m - distance_m`).
            if noise.loudness_m - noise.distance_m < cfg.asleep_wake_m {
                return StimulusOutcome::Unheard;
            }
            self.wake(now.tick);
        }
        let t = now.tick;
        match noise.source {
            Some(source) if noise.reveals_source => {
                let seen = LastSeen { pos: noise.at, tick: t, velocity: Vec3::ZERO, confidence: cfg.noise_confidence };
                self.raise(cfg, source, seen, now, |e| e.gain(cfg, gain, cfg.noise_cap, seen, t))
            }
            // Spec gap: a revealing noise with no source is a distraction too.
            _ => {
                let fresh = Distraction { at: noise.at, tick: t, loudness_m: noise.loudness_m };
                let replace = self.distraction.is_none_or(|d| {
                    now.since(d.tick) >= cfg.distraction_ttl_s || fresh.loudness_m >= d.loudness_m
                });
                if replace {
                    self.distraction = Some(fresh);
                    StimulusOutcome::Distraction
                } else {
                    StimulusOutcome::Ignored
                }
            }
        }
    }

    /// Finds or inserts `target`'s entry, settles it to `now`, runs `f`, then evicts the lowest
    /// entry if over `max_entries` (lowest level, then oldest gain, then highest target).
    fn raise(
        &mut self,
        cfg: &AwarenessConfig,
        target: PerceptTarget,
        seen: LastSeen,
        now: PerceptClock,
        f: impl FnOnce(&mut Awareness),
    ) -> StimulusOutcome {
        let i = match self.entries.binary_search_by(|e| e.target.cmp(&target)) {
            Ok(i) => i,
            Err(i) => {
                self.entries.insert(i, Awareness::new(target, seen, now.tick));
                i
            }
        };
        let e = &mut self.entries[i];
        e.settle(cfg, now);
        f(e);
        let band = e.band;
        if self.entries.len() > cfg.max_entries {
            // Compare levels as of `now`, not as of each entry's last update.
            for e in &mut self.entries {
                e.settle(cfg, now);
            }
            let lowest = self
                .entries
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.level.total_cmp(&b.level).then(a.last_gain.cmp(&b.last_gain)).then(b.target.cmp(&a.target))
                })
                .map(|(j, _)| j);
            if let Some(j) = lowest {
                let gone = self.entries.remove(j).target;
                if gone == target {
                    return StimulusOutcome::Evicted;
                }
            }
        }
        StimulusOutcome::Raised(band)
    }
}

impl Awareness {
    /// Lifts the level to `band`'s threshold (never lowering) and enters at least `band`.
    fn floor(&mut self, cfg: &AwarenessConfig, band: AwarenessBand, seen: LastSeen, now: u64) {
        let lvl = cfg.enter(band);
        if self.level < lvl {
            self.level = lvl;
            self.last_gain = now;
        }
        self.last_seen = seen;
        self.band = cfg.next_band(self.band, self.level).max(band);
    }
}

/// The HUD sneak eye's value for one player (ai.md "HUD"). Replicated to that player's client.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerDetection {
    /// Max level toward the player over the counting actors.
    pub level: f32,
    /// Max band toward the player over the same actors.
    pub band: AwarenessBand,
}

/// `PlayerDetection` for `player` over `meters`: the tier 2 and tier 1 actors that count (the
/// caller leaves out allies and followers). Spec gap: the band is the max band (hysteresis can
/// hold one actor's band above another's at a lower level), not the band of the max level.
pub fn player_detection<'a>(meters: impl IntoIterator<Item = &'a AwarenessMeter>, player: PlayerKey) -> PlayerDetection {
    let target = PerceptTarget::Player(player);
    meters.into_iter().filter_map(|m| m.get(target)).fold(PlayerDetection::default(), |acc, e| PlayerDetection {
        level: acc.level.max(e.level),
        band: acc.band.max(e.band),
    })
}
