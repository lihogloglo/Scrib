//! `LW4a`: the stealth detection seam (crime.md "Stealth and detection"). Pure, Bevy-free,
//! allocation-free per call.
//!
//! - [`DetectionRules::concealment`]: how hidden a target is from one observer right now, larger is
//!   better hidden, 0..~200. `sf_ai` runs the detection model on it (ai.md "Models": Meter gain ×
//!   [`visibility_from_concealment`], or Classic's one roll against it); crime never reads light
//!   itself, it gets `sf_ai`'s [`VisibilityFactors`] (light after the observer's `darkness`).
//! - [`DetectIn`]: the observer and target views, distance, facing angle and factors. The views carry
//!   stats only; the observer's duty, drink, sleep and mood are its `sf_ai` `PerceptionProfile` /
//!   `PerceptionStates` (`AI0g`), which scale sight and hearing there, never here.
//! - [`StealthState`]: the target's sneak flag and speed class, plain data `sf_crime_bevy` builds
//!   from `sf_action`'s `Sneak` mode and the mover.
//! - [`StandardDetection`]: the framework's modern default (Skyrim / AC Shadows: light, stance,
//!   speed; facing as in TLOU and Hitman). Additive points:
//!
//!   `light_concealment × (1 − light) + stance + behind_concealment × behind(angle)
//!    − min(speed_reveal_per_mps × max(0, speed − speed_free_mps), speed_reveal_max)`,
//!
//!   clamped to `0..=max_concealment`. `behind` is 0 inside `facing_front_rad` and linear to 1
//!   straight behind. With the defaults an upright, still target in front has concealment
//!   `100 × (1 − light)`, so its visibility is its light: exactly the light-only sight it replaces
//!   (the light-only visibility sight used before `LW4b1`). Distance, size, noise, disguise and `factors.modifiers`
//!   are ignored: distance is `sf_ai`'s `f_dist`, and the modifiers' stat ids (a sneak skill, a magic
//!   effect) are the game's, read only by the game's own rules.
//!
//! **Bad input** (stated rule): a non-finite `light` reads as 1 (full light), `light` clamps to
//! 0..1; a non-finite or negative `speed_mps` reads as 0; a non-finite `angle_rad` reads as 0 (in
//! front), its magnitude clamps to 0..π. A non-finite or negative config value counts as 0
//! (`facing_front_rad` ≥ π never hides from behind). The result is always finite, in
//! `0..=max_concealment`. [`visibility_from_concealment`] reads a `NaN` as 1 (fully visible).

use std::f32::consts::{FRAC_PI_4, PI};

use serde::{Deserialize, Serialize};
use sf_ai::{Stance, StatId, VisibilityFactors};

/// Stats per [`StatView`] (a capacity: the few stats a game's rules read for one character).
// tunables: const (capacity: inline stat slots per view, not a quality/cost trade)
pub const STAT_VIEW_CAPACITY: usize = 8;

/// The concealment that maps to visibility 0 (ai.md "Sight gain": `1 - concealment / 100`).
// tunables: const (maths: the scale of the concealment-to-visibility map, fixed by ai.md and SightCheck::visibility)
const CONCEALMENT_SCALE: f32 = 100.0;

/// How fast a target moves, as the eye classes it (plain data; `sf_crime_bevy` maps the gait and
/// the mover's speed onto it).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Speed {
    #[default]
    Still,
    Walk,
    Run,
    Sprint,
}

/// A target's stealth state (crime.md: built read-only by `sf_crime_bevy` from `sf_action`'s
/// `Sneak` mode and the character's movement; this crate never moves a character).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StealthState {
    pub sneaking: bool,
    pub moving: Speed,
}

/// A character's stats as the rules see them: `(StatId, value)` pairs inline (at most
/// [`STAT_VIEW_CAPACITY`]). The stat ids are the game's; the framework names none.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StatView {
    stats: [Option<(StatId, f32)>; STAT_VIEW_CAPACITY],
}

impl StatView {
    /// Sets `stat` to `value` (replacing an earlier value), else takes the first free slot;
    /// `false` (nothing changed) when `stat` is new and all slots are taken.
    pub fn set(&mut self, stat: StatId, value: f32) -> bool {
        if let Some(slot) = self.stats.iter_mut().flatten().find(|(s, _)| *s == stat) {
            slot.1 = value;
            return true;
        }
        match self.stats.iter_mut().find(|s| s.is_none()) {
            Some(slot) => {
                *slot = Some((stat, value));
                true
            }
            None => false,
        }
    }

    /// The value of `stat`, if set.
    #[must_use]
    pub fn get(&self, stat: StatId) -> Option<f32> {
        self.iter().find_map(|(s, v)| (s == stat).then_some(v))
    }

    /// The set stats, in slot order.
    pub fn iter(&self) -> impl Iterator<Item = (StatId, f32)> + '_ {
        self.stats.iter().flatten().copied()
    }
}

/// The target as the rules see it: its stats and stealth state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetView {
    pub stats: StatView,
    pub stealth: StealthState,
}

/// The observer as the rules see it: stats only (its duty, drink, sleep and mood are `sf_ai`'s
/// perception profile and states, never crime's).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ObserverView {
    pub stats: StatView,
}

/// One concealment query (crime.md `DetectIn`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DetectIn {
    pub target: TargetView,
    pub observer: ObserverView,
    /// Eye to target, meters.
    pub distance_m: f32,
    /// Angle between the observer's facing and the target, radians (0 = in front, π = behind).
    pub angle_rad: f32,
    /// What the observer sees of the target this check (`sf_ai`: light, stance, speed, ...).
    pub factors: VisibilityFactors,
}

/// The crime rule seam for stealth (crime.md decision 6: crime owns the formula, AI owns detection).
pub trait DetectionRules: Send + Sync {
    /// How hidden `i.target` is from `i.observer` right now; larger is better hidden. 0..~200
    /// (100 and above is invisible to the Meter model).
    fn concealment(&self, i: &DetectIn) -> f32;
}

/// Sight visibility from concealment: `clamp(1 - c / 100, 0, 1)` (ai.md "Sight gain",
/// `sf_ai::SightCheck::visibility`). A `NaN` reads as fully visible.
#[must_use]
pub fn visibility_from_concealment(c: f32) -> f32 {
    if c.is_nan() {
        return 1.0;
    }
    (1.0 - c / CONCEALMENT_SCALE).clamp(0.0, 1.0)
}

/// [`StandardDetection`] tunables (`crime.stealth.*`, bound in `sf_crime_bevy`; crime.md "Stealth and
/// detection"). Concealment is in points (0..~200, 100 = invisible to the Meter model).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardDetectionConfig {
    /// Concealment in full darkness (scaled by `1 − light`), points (range 0-200).
    pub light_concealment: f32,
    /// Concealment from crouching, points (range 0-100).
    pub crouched_concealment: f32,
    /// Concealment from lying prone, points (range 0-150).
    pub prone_concealment: f32,
    /// Speed up to which moving reveals nothing (a sneak walk), m/s (range 0-10).
    pub speed_free_mps: f32,
    /// Concealment lost per m/s above `speed_free_mps`, points per m/s (range 0-50).
    pub speed_reveal_per_mps: f32,
    /// Most concealment speed can take away, points (range 0-200).
    pub speed_reveal_max: f32,
    /// Half-angle of the observer's front, where facing hides nothing, radians (range 0-π).
    pub facing_front_rad: f32,
    /// Concealment straight behind the observer (linear from `facing_front_rad`), points (range 0-100).
    pub behind_concealment: f32,
    /// Upper clamp of the result, points (range 100-400; crime.md "0..~200").
    pub max_concealment: f32,
}

impl Default for StandardDetectionConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from crime.stealth.light_concealment in sf_crime_bevy; 100 keeps an upright still target's visibility = its light, the AI8d2 light-only sight)
            light_concealment: 100.0,
            // tunables: const (bound from crime.stealth.crouched_concealment in sf_crime_bevy; Guess: Skyrim/AC crouch hides about a third of a lit target)
            crouched_concealment: 30.0,
            // tunables: const (bound from crime.stealth.prone_concealment in sf_crime_bevy; Guess: prone hides more than crouched, MGS/AC)
            prone_concealment: 50.0,
            // tunables: const (bound from crime.stealth.speed_free_mps in sf_crime_bevy; Guess: a sneak walk reveals nothing)
            speed_free_mps: 1.5,
            // tunables: const (bound from crime.stealth.speed_reveal_per_mps in sf_crime_bevy; Guess: a 4 m/s run loses 25 points)
            speed_reveal_per_mps: 10.0,
            // tunables: const (bound from crime.stealth.speed_reveal_max in sf_crime_bevy; Guess: a sprint cancels crouch and most darkness)
            speed_reveal_max: 60.0,
            // tunables: const (bound from crime.stealth.facing_front_rad in sf_crime_bevy; Guess: 45 degrees, inside the central sight cone)
            facing_front_rad: FRAC_PI_4,
            // tunables: const (bound from crime.stealth.behind_concealment in sf_crime_bevy; Guess: behind hides less than darkness, more than crouch)
            behind_concealment: 40.0,
            // tunables: const (bound from crime.stealth.max_concealment in sf_crime_bevy; crime.md range 0..~200)
            max_concealment: 200.0,
        }
    }
}

/// A finite, non-negative config value (else 0, the module's bad-input rule).
fn non_neg(v: f32) -> f32 {
    if v.is_finite() { v.max(0.0) } else { 0.0 }
}

/// The framework's modern default [`DetectionRules`]: light, stance, speed and facing (module docs).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StandardDetection {
    pub config: StandardDetectionConfig,
}

impl StandardDetection {
    /// Rules with `config`.
    #[must_use]
    pub fn new(config: StandardDetectionConfig) -> Self {
        Self { config }
    }
}

impl DetectionRules for StandardDetection {
    fn concealment(&self, i: &DetectIn) -> f32 {
        let c = &self.config;
        let f = &i.factors;

        let light = if f.light.is_finite() { f.light.clamp(0.0, 1.0) } else { 1.0 };
        let from_light = non_neg(c.light_concealment) * (1.0 - light);

        let from_stance = match f.stance {
            Stance::Upright => 0.0,
            Stance::Crouched => non_neg(c.crouched_concealment),
            Stance::Prone => non_neg(c.prone_concealment),
        };

        let front = non_neg(c.facing_front_rad);
        let angle = if i.angle_rad.is_finite() { i.angle_rad.abs().min(PI) } else { 0.0 };
        let behind = if front >= PI { 0.0 } else { ((angle - front) / (PI - front)).clamp(0.0, 1.0) };
        let from_facing = non_neg(c.behind_concealment) * behind;

        let speed = non_neg(f.speed_mps);
        let reveal = (non_neg(c.speed_reveal_per_mps) * (speed - non_neg(c.speed_free_mps)).max(0.0))
            .min(non_neg(c.speed_reveal_max));

        let total = from_light + from_stance + from_facing - reveal;
        if total.is_finite() { total.clamp(0.0, non_neg(c.max_concealment)) } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(light: f32, stance: Stance, speed_mps: f32, angle_rad: f32) -> DetectIn {
        DetectIn {
            distance_m: 10.0,
            angle_rad,
            factors: VisibilityFactors { light, stance, speed_mps, ..VisibilityFactors::default() },
            ..DetectIn::default()
        }
    }

    fn conceal(i: &DetectIn) -> f32 {
        StandardDetection::default().concealment(i)
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn upright_full_light_in_front_is_unconcealed() {
        let c = conceal(&input(1.0, Stance::Upright, 0.0, 0.0));
        assert!(close(c, 0.0), "{c}");
        assert!(close(visibility_from_concealment(c), 1.0));
    }

    #[test]
    fn upright_still_in_front_visibility_equals_light() {
        for light in [0.0, 0.25, 0.5, 0.9] {
            let c = conceal(&input(light, Stance::Upright, 0.0, 0.0));
            assert!(close(visibility_from_concealment(c), light), "light {light}: c {c}");
        }
    }

    #[test]
    fn crouched_beats_upright_and_prone_beats_crouched() {
        let up = conceal(&input(0.5, Stance::Upright, 0.0, 0.0));
        let crouch = conceal(&input(0.5, Stance::Crouched, 0.0, 0.0));
        let prone = conceal(&input(0.5, Stance::Prone, 0.0, 0.0));
        assert!(close(up, 50.0), "{up}");
        assert!(close(crouch, 80.0), "{crouch}");
        assert!(close(prone, 100.0), "{prone}");
    }

    #[test]
    fn moving_fast_reveals() {
        let still = conceal(&input(0.5, Stance::Crouched, 0.0, 0.0));
        let sneak_walk = conceal(&input(0.5, Stance::Crouched, 1.5, 0.0));
        let run = conceal(&input(0.5, Stance::Crouched, 4.0, 0.0));
        let sprint = conceal(&input(0.5, Stance::Crouched, 20.0, 0.0));
        assert!(close(sneak_walk, still), "{sneak_walk} vs {still}");
        assert!(close(run, 55.0), "{run}");
        assert!(close(sprint, 20.0), "capped at speed_reveal_max: {sprint}");
        assert!(sprint < run && run < still);
    }

    #[test]
    fn behind_hides_more_than_in_front() {
        let front = conceal(&input(0.5, Stance::Upright, 0.0, 0.0));
        let edge = conceal(&input(0.5, Stance::Upright, 0.0, FRAC_PI_4));
        let side = conceal(&input(0.5, Stance::Upright, 0.0, -PI / 2.0));
        let behind = conceal(&input(0.5, Stance::Upright, 0.0, PI));
        assert!(close(edge, front), "inside the front: {edge}");
        assert!(close(side, 50.0 + 40.0 / 3.0), "{side}");
        assert!(close(behind, 90.0), "{behind}");
        assert!(behind > side && side > front);
    }

    #[test]
    fn result_is_finite_and_in_range() {
        let rules = StandardDetection::default();
        let lights = [0.0, 0.5, 1.0, -3.0, 7.0, f32::NAN, f32::INFINITY];
        let speeds = [0.0, 3.0, 1e9, -2.0, f32::NAN, f32::INFINITY];
        let angles = [0.0, 1.0, PI, 10.0, -10.0, f32::NAN];
        for light in lights {
            for speed in speeds {
                for angle in angles {
                    for stance in [Stance::Upright, Stance::Crouched, Stance::Prone] {
                        let c = rules.concealment(&input(light, stance, speed, angle));
                        assert!(c.is_finite() && (0.0..=200.0).contains(&c), "{light} {speed} {angle} {stance:?}: {c}");
                    }
                }
            }
        }
        let max = rules.concealment(&input(0.0, Stance::Prone, 0.0, PI));
        assert!(close(max, 190.0), "{max}");
    }

    #[test]
    fn bad_config_counts_as_zero() {
        let rules = StandardDetection::new(StandardDetectionConfig {
            light_concealment: f32::NAN,
            crouched_concealment: -5.0,
            behind_concealment: f32::INFINITY,
            facing_front_rad: 4.0,
            ..StandardDetectionConfig::default()
        });
        let c = rules.concealment(&input(0.0, Stance::Crouched, 0.0, PI));
        assert!(close(c, 0.0), "{c}");
    }

    #[test]
    fn modifiers_and_views_are_ignored_by_standard_rules() {
        let plain = input(0.3, Stance::Crouched, 2.0, 2.0);
        let mut modded = plain;
        assert!(modded.factors.push_modifier(StatId(7), 100.0));
        assert!(modded.target.stats.set(StatId(7), 100.0));
        assert!(modded.observer.stats.set(StatId(3), 50.0));
        modded.target.stealth = StealthState { sneaking: true, moving: Speed::Walk };
        assert!(close(conceal(&plain), conceal(&modded)));
    }

    #[test]
    fn visibility_from_concealment_endpoints() {
        assert!(close(visibility_from_concealment(0.0), 1.0));
        assert!(close(visibility_from_concealment(50.0), 0.5));
        assert!(close(visibility_from_concealment(100.0), 0.0));
        assert!(close(visibility_from_concealment(200.0), 0.0));
        assert!(close(visibility_from_concealment(-30.0), 1.0));
        assert!(close(visibility_from_concealment(f32::INFINITY), 0.0));
        assert!(close(visibility_from_concealment(f32::NEG_INFINITY), 1.0));
        assert!(close(visibility_from_concealment(f32::NAN), 1.0));
    }

    #[test]
    fn stat_view_set_get_replace_and_capacity() {
        let mut v = StatView::default();
        assert_eq!(v.get(StatId(1)), None);
        assert!(v.set(StatId(1), 10.0));
        assert!(v.set(StatId(1), 20.0));
        assert_eq!(v.get(StatId(1)), Some(20.0));
        for i in 2..=8 {
            assert!(v.set(StatId(i), f32::from(i)));
        }
        assert!(!v.set(StatId(9), 9.0), "full");
        assert!(v.set(StatId(8), 80.0), "replacing still works when full");
        assert_eq!(v.iter().count(), STAT_VIEW_CAPACITY);
        assert_eq!(v.get(StatId(8)), Some(80.0));
    }
}
