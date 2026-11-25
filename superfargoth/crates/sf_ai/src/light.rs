//! `AI8d1`: the light exposure core (`ai.md` "Light exposure (from game state, never the GPU)",
//! "Sight gain"). Pure, Bevy-free, allocation-free per call; rendered light never enters (net rule 3),
//! so the result is the same at any frame rate.
//!
//! - [`VisibilityFactors`]: what the game's `DetectionRules::concealment` reads about a target
//!   (crime.md `DetectIn.factors`); `Copy`, its stat modifiers inline (at most
//!   [`VISIBILITY_MODIFIERS_CAPACITY`]).
//! - [`LightSource`] + [`light_exposure`]: 0..1 at a point from the host's sky level, its one upward
//!   roof ray, and the strongest `light_rays` sources within `light_search_m`, each behind one
//!   line-of-sight callback (occluded adds nothing).
//! - [`is_dark`]: the `dark` world-state test (`< dark_threshold`). The observer's `darkness` term is
//!   [`crate::apply_darkness`] (`sight.rs`, `AI8b1`).
//!
//! **Falloff** (one source at distance `d`, radius `r`): `intensity × saturate(1 − (d/r)^k)²`, the
//! windowing term of UE4 (Karis 2013, "Real Shading in Unreal Engine 4") and Frostbite (Lagarde
//! 2014), which reaches 0 at `r` with a zero slope. Those engines multiply it by `1/d²`; a gameplay
//! source carries a radius, not lumens, so the window alone does the work, with `k` =
//! `light_falloff_exp` (default 2: 0.5625 at half radius; 4 is the engines' flat-topped window).
//! Sources add, the sky term adds, the total clamps to 0..1 (Thief's light gem sums its lights).
//!
//! **Bad input** (stated rule): a non-finite `sky` counts as 0, `sky` clamps to 0..1. A source that is
//! off, has a non-finite or ≤ 0 `radius_m` or `intensity`, or a non-finite distance adds nothing
//! (negative "darkness" lights are a render effect, never gameplay). A non-finite or negative config
//! value counts as 0 (`light_search_m` 0 finds no source; `light_falloff_exp` 0 is a hard edge). The
//! result is always finite, in 0..1.

use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

use crate::ids::StatId;

/// Stat modifiers per [`VisibilityFactors`] (a capacity: sneak, chameleon, invisibility, one spare).
pub const VISIBILITY_MODIFIERS_CAPACITY: usize = 4;

/// Upper bound of `light_rays` (ai.md range 0-4; the inline top-k array size).
// tunables: const (capacity: ai.md caps light_rays at 4; the tunable is LightConfig.light_rays)
pub const MAX_LIGHT_RAYS: usize = 4;

/// The target's posture as the eye sees it (crime.md `DetectionRules` reads it; the framework
/// assigns no concealment to it).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stance {
    #[default]
    Upright,
    Crouched,
    Prone,
}

/// What a target looks like to an observer this check (ai.md "Sight gain"; crime.md
/// `DetectIn.factors`). The game's `DetectionRules::concealment` turns it into a number; `sf_ai`
/// never interprets `modifiers`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisibilityFactors {
    /// Light at the target after the observer's `darkness` ([`crate::apply_darkness`]), 0..1.
    pub light: f32,
    pub stance: Stance,
    /// Target speed, m/s.
    pub speed_mps: f32,
    /// Target size, ratio (1 = a standard humanoid).
    pub size: f32,
    /// The target's own noise multiplier (armour, sneaking), ratio.
    pub noise_mul: f32,
    /// How well the target is disguised to this observer, 0 (none) to 1 (perfect).
    pub disguise: f32,
    /// Stat values the adapter reads (sneak skill, chameleon magnitude, ...), inline; use
    /// [`Self::push_modifier`] / [`Self::modifiers`].
    pub modifiers: [Option<(StatId, f32)>; VISIBILITY_MODIFIERS_CAPACITY],
}

impl Default for VisibilityFactors {
    fn default() -> Self {
        Self {
            // tunables: const (neutral input: full light, the target hides nothing)
            light: 1.0,
            stance: Stance::Upright,
            // tunables: const (neutral input: standing still)
            speed_mps: 0.0,
            // tunables: const (identity ratio: a standard humanoid)
            size: 1.0,
            // tunables: const (identity ratio: unscaled noise)
            noise_mul: 1.0,
            // tunables: const (neutral input: no disguise)
            disguise: 0.0,
            modifiers: [None; VISIBILITY_MODIFIERS_CAPACITY],
        }
    }
}

impl VisibilityFactors {
    /// Adds `(stat, value)` in the first free slot; `false` (nothing changed) when all are taken.
    pub fn push_modifier(&mut self, stat: StatId, value: f32) -> bool {
        match self.modifiers.iter_mut().find(|m| m.is_none()) {
            Some(slot) => {
                *slot = Some((stat, value));
                true
            }
            None => false,
        }
    }

    /// The set modifiers, in slot order.
    pub fn modifiers(&self) -> impl Iterator<Item = (StatId, f32)> + '_ {
        self.modifiers.iter().flatten().copied()
    }
}

/// A gameplay light (IN19 `Lit`: radius, intensity, on). The game maps its lamps, torches and spells
/// to these; the renderer's lights never enter.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightSource {
    pub pos: WorldPos,
    /// Reach: contribution falls to 0 here, meters.
    pub radius_m: f32,
    /// Contribution at the centre, 0..1 typical (more saturates sooner).
    pub intensity: f32,
    pub on: bool,
}

/// Light exposure tunables (`ai.perception.*`, bound in `sf_ai_bevy`; ai.md "Tunables" table).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightConfig {
    /// Sources farther than this are not considered, meters (range 1-60).
    pub light_search_m: f32,
    /// Strongest sources checked by a line-of-sight ray, count (range 0-4).
    pub light_rays: u32,
    /// Exposure below this at an actor is `dark` (world-state key), 0..1 (range 0-1).
    pub dark_threshold: f32,
    /// Share of the sky term left under a roof (outdoors, upward ray hit), ratio (range 0-1).
    pub roof_sky_mul: f32,
    /// Exponent `k` of the falloff window `saturate(1 − (d/r)^k)²` (range 1-8; 4 = UE4/Frostbite).
    pub light_falloff_exp: f32,
}

impl Default for LightConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.perception.light_search_m in sf_ai_bevy; ai.md table)
            light_search_m: 15.0,
            // tunables: const (bound from ai.perception.light_rays in sf_ai_bevy; ai.md table)
            light_rays: 2,
            // tunables: const (bound from ai.perception.dark_threshold in sf_ai_bevy; ai.md table)
            dark_threshold: 0.15,
            // tunables: const (bound from ai.perception.roof_sky_mul in sf_ai_bevy; Guess: indirect skylight under an open-sided roof)
            roof_sky_mul: 0.25,
            // tunables: const (bound from ai.perception.light_falloff_exp in sf_ai_bevy; Guess, UE4/Frostbite window uses 4)
            light_falloff_exp: 2.0,
        }
    }
}

/// A finite, non-negative config value (else 0, the module's bad-input rule).
fn non_neg(v: f32) -> f32 {
    if v.is_finite() { v.max(0.0) } else { 0.0 }
}

/// One source's unoccluded contribution at distance `d` (module docs "Falloff"); 0 for bad input or
/// `d ≥ radius_m`.
pub fn source_contribution(src: &LightSource, d: f32, falloff_exp: f32) -> f32 {
    let ok = src.on
        && src.radius_m.is_finite()
        && src.radius_m > 0.0
        && src.intensity.is_finite()
        && src.intensity > 0.0
        && d.is_finite()
        && d >= 0.0;
    if !ok || d >= src.radius_m {
        return 0.0;
    }
    let k = non_neg(falloff_exp);
    let window = (1.0 - (d / src.radius_m).powf(k)).clamp(0.0, 1.0);
    src.intensity * window * window
}

/// The sky term: `sky` clamped to 0..1 (non-finite = 0), × `roof_sky_mul` when covered.
fn sky_term(sky: f32, roof_covered: bool, cfg: &LightConfig) -> f32 {
    let sky = if sky.is_finite() { sky.clamp(0.0, 1.0) } else { 0.0 };
    if roof_covered { sky * non_neg(cfg.roof_sky_mul).min(1.0) } else { sky }
}

/// Light exposure at `at`, 0..1 (ai.md "Light exposure").
///
/// - `sky`: the host's sky level 0..1, already built from clock, weather and the interior flag
///   (an interior passes its ambient level and `roof_covered = false`).
/// - `roof_covered`: outdoors, whether the host's one upward ray hit a roof (then the sky term is
///   scaled by `roof_sky_mul`).
/// - `sources`: candidate lights (the host's spatial query; unsorted is fine). Of those on and
///   within `light_search_m`, the `light_rays` strongest by unoccluded contribution are checked by
///   `los(at, source.pos)` (`true` = clear), strongest first; an occluded one adds nothing and is
///   not replaced. Ties keep the earlier source.
///
/// No allocation; `los` is called at most `min(light_rays, MAX_LIGHT_RAYS)` times.
pub fn light_exposure(
    at: WorldPos,
    sky: f32,
    roof_covered: bool,
    sources: &[LightSource],
    los: &mut dyn FnMut(WorldPos, WorldPos) -> bool,
    cfg: &LightConfig,
) -> f32 {
    let rays = usize::try_from(cfg.light_rays).unwrap_or(MAX_LIGHT_RAYS).min(MAX_LIGHT_RAYS);
    let search = non_neg(cfg.light_search_m);
    // Top `rays` by contribution, descending: (contribution, source index).
    let mut top = [(0.0_f32, 0_usize); MAX_LIGHT_RAYS];
    let mut n = 0;
    if rays > 0 {
        for (i, src) in sources.iter().enumerate() {
            let d = at.delta_f32(src.pos).length();
            if d.is_nan() || d > search {
                continue;
            }
            let c = source_contribution(src, d, cfg.light_falloff_exp);
            if c <= 0.0 {
                continue;
            }
            // Insertion: first slot this one beats strictly (ties keep the earlier source).
            let Some(at_slot) = (0..rays).find(|&s| s >= n || c > top[s].0) else {
                continue;
            };
            let last = n.min(rays - 1);
            for s in (at_slot..last).rev() {
                top[s + 1] = top[s];
            }
            top[at_slot] = (c, i);
            n = (n + 1).min(rays);
        }
    }
    let mut total = sky_term(sky, roof_covered, cfg);
    for &(c, i) in &top[..n] {
        if los(at, sources[i].pos) {
            total += c;
        }
    }
    total.clamp(0.0, 1.0)
}

/// Whether `light` is below `dark_threshold` (the `dark` world-state key). A NaN `light` is dark.
pub fn is_dark(light: f32, cfg: &LightConfig) -> bool {
    light.is_nan() || light < non_neg(cfg.dark_threshold)
}
