//! Sun and moon shadow gate (ROADMAP 1.25 CW-8), Bevy-free: a directional light's cascades
//! follow its visible contribution, not only its elevation. The same rule as the lamp gate
//! (`sf_sky_bevy::ShadowContrastGate`, ROADMAP 1.23b): a shadow darkens a receiver by the
//! light's share of the light it gets, its contrast `E_direct / (E_direct + E_fill)`. Under a
//! dimmed sun (haze, dust, a cloud shadow texture that blocks most of it) the sky's fill
//! drowns the shadow, yet its cascades cost a caster scan and two to four depth views.
//!
//! [`SunShadowGate::weight`] maps the contrast to `[0, 1]` across a band (0 at or below
//! `contrast_off`, 1 at or above `contrast_full`); a light holding its maps counts its
//! contrast `1 / hysteresis` times, so it doesn't flip at the edge. [`SunShadowGate::step`]
//! moves the shadow strength toward that weight at most `1 / fade_seconds` per second (a
//! weather change fades over seconds, never in one frame); the maps are on while the
//! strength is above 0. A light that starts casting (the sky hands shadows from the sun to
//! the moon, the viewer walks outdoors) starts at its weight, as the elevation rule did.
//!
//! Industry: Unity (`Light.shadowStrength`) and UE (`ShadowAmount`) expose a directional
//! shadow strength; engines drop shadow work that doesn't show (UE's per-light shadow
//! fade-out by screen size, the owner's 1.23 rule "lamp shadows by perceptual value"). A
//! shipped engine dropping sun cascades by weather is not documented that we know of: the
//! thresholds are `Guess`es anchored on the lamp gate's.

use sf_tunables::{AnyDef, Def, Group, Source, Unit, def};

use crate::sky::{SkyLight, SkyState};

/// `shadow.sun.gate`: [`SunShadowGate::enabled`].
pub const GATE: Def<bool> = def!(bool "shadow.sun.gate" = true)
    .help("Sun and moon cascades fade out and turn off when their direct light is drowned by the sky's fill")
    .source(Source::Owner("2026-11-17: on by default after the owner's play session with every gate on; A/B docs/ledger/perf.md 2026-11-17"))
    .group(Group::Shadows);

/// `shadow.sun.gate.contrast`: [`SunShadowGate::contrast_off`] and
/// [`SunShadowGate::contrast_full`]. The lamp gate's band (`shadow.lamp.day_gate.contrast`:
/// Weber's ~1-2 % raised for textured, tonemapped frames); nobody measured it for the sun.
pub const CONTRAST: Def<[f32; 2]> = def!(float2 "shadow.sun.gate.contrast" = [0.05, 0.15])
    .range([0.0, 0.0], [1.0, 1.0])
    .unit(Unit::Ratio)
    .help("Sun/moon shadow contrast (direct light's share of the light on a receiver facing it): maps off below the first, full strength above the second")
    .group(Group::Shadows);

/// `shadow.sun.gate.hysteresis`: [`SunShadowGate::hysteresis`] (the lamp budget's 0.8).
pub const HYSTERESIS: Def<f32> = def!(float "shadow.sun.gate.hysteresis" = 0.8)
    .range(0.1, 1.0)
    .unit(Unit::Ratio)
    .help("A light holding its cascades counts its shadow contrast divided by this, so it doesn't flip at the threshold")
    .group(Group::Shadows);

/// `shadow.sun.gate.fade_s`: [`SunShadowGate::fade_seconds`]. Weather transitions blend over
/// tens of seconds (Morrowind's `Transition Delta` 0.015/s: ~67 s), so the band is crossed in
/// a few seconds anyway; this caps the rate for sudden changes (a pinned cloud cover).
pub const FADE: Def<f32> = def!(float "shadow.sun.gate.fade_s" = 3.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("Seconds for the sun/moon shadow strength to fade fully in or out")
    .group(Group::Shadows);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[&GATE, &CONTRAST, &HYSTERESIS, &FADE];

/// When a directional light's shadow is worth its cascades (module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SunShadowGate {
    /// Off: the elevation rule alone, as before ([`GATE`]).
    pub enabled: bool,
    /// Contrast at or below which the light gets no cascades.
    pub contrast_off: f32,
    /// Contrast at or above which the shadow has full strength.
    pub contrast_full: f32,
    /// A holder's contrast counts `1 / hysteresis` times.
    pub hysteresis: f32,
    /// Seconds for a full fade (0: instant).
    pub fade_seconds: f32,
}

impl Default for SunShadowGate {
    fn default() -> Self {
        let [contrast_off, contrast_full] = CONTRAST.default_value();
        Self {
            enabled: GATE.default_value(),
            contrast_off,
            contrast_full,
            hysteresis: HYSTERESIS.default_value(),
            fade_seconds: FADE.default_value(),
        }
    }
}

/// One light's gate state.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GateState {
    /// Shadow strength, `[0, 1]`.
    pub strength: f32,
    /// The light has its maps (strength above 0).
    pub held: bool,
    /// The elevation rule let it cast last step (a fresh caster starts at its weight).
    pub casting: bool,
}

impl GateState {
    /// A light casting at `strength`.
    #[must_use]
    pub fn casting(strength: f32) -> Self {
        Self { strength, held: strength > 0.0, casting: true }
    }
}

impl SunShadowGate {
    /// Weight of a shadow of `contrast`: 0 at or below [`Self::contrast_off`], 1 at or above
    /// [`Self::contrast_full`], linear between; a holder counts `contrast / hysteresis`.
    #[must_use]
    pub fn weight(&self, contrast: f32, held: bool) -> f32 {
        let c = if held { contrast / self.hysteresis.max(1e-3) } else { contrast };
        if c >= self.contrast_full {
            1.0
        } else if c <= self.contrast_off || self.contrast_full <= self.contrast_off {
            0.0
        } else {
            (c - self.contrast_off) / (self.contrast_full - self.contrast_off)
        }
    }

    /// The next state of a light that the elevation rule lets cast (`casts`), with shadow
    /// `contrast`, after `dt` seconds. Not casting: reset at once (the sky's own hand-over,
    /// unchanged). Starting to cast: its weight at once, as the elevation rule did. Casting:
    /// toward its weight at most `dt / fade_seconds`, also back in after the gate dropped it.
    #[must_use]
    pub fn step(&self, state: GateState, casts: bool, contrast: f32, dt: f32) -> GateState {
        if !casts {
            return GateState::default();
        }
        let target = self.weight(contrast, state.held);
        let strength = if state.casting {
            let rate = if self.fade_seconds > 0.0 { dt.max(0.0) / self.fade_seconds } else { 1.0 };
            state.strength + (target - state.strength).clamp(-rate, rate)
        } else {
            target
        };
        GateState::casting(strength.clamp(0.0, 1.0))
    }
}

/// A light's direct illuminance on level ground, lux: ground illuminance times
/// `sin(elevation)`, times `occlusion` (the mean share a shadow texture on the light lets
/// through, e.g. cloud shadows; 1 = none). What the light adds to another light's fill.
#[must_use]
pub fn level_direct(light: &SkyLight, occlusion: f32) -> f32 {
    light.ground_illuminance() * light.elevation.sin().max(0.0) * occlusion.clamp(0.0, 1.0)
}

/// A light's direct illuminance on a receiver facing it, lux (above the horizon; times
/// `occlusion` as [`level_direct`]): the receiver whose shadow shows best, as the lamp gate's
/// (a wall facing a low sun gets ten times what the ground does at 6 degrees).
#[must_use]
pub fn facing_direct(light: &SkyLight, occlusion: f32) -> f32 {
    if light.elevation <= 0.0 {
        return 0.0;
    }
    light.ground_illuminance() * occlusion.clamp(0.0, 1.0)
}

/// Shadow contrast `direct / (direct + fill)`; 0 without light.
#[must_use]
pub fn contrast(direct: f32, fill: f32) -> f32 {
    let direct = direct.max(0.0);
    let total = direct + fill.max(0.0);
    if total > 0.0 { direct / total } else { 0.0 }
}

/// The shadow contrasts of the sun and the moon outdoors, on a receiver facing each: its
/// [`facing_direct`] against the fill (both lights' [`SkyLight::sky_fill`], the other light's
/// [`level_direct`], and `ambient_lux`, the flat ambient's illuminance, `π · L`). The level
/// fill on a facing receiver errs high and the single-scattering fill low; facing the light
/// keeps a low sun's shadows on walls. `sun_occlusion`: the mean share of the sun's direct
/// light its shadow texture lets through.
#[must_use]
pub fn contrasts(sky: &SkyState, sun_occlusion: f32, ambient_lux: f32) -> (f32, f32) {
    let fill = sky.sun.sky_fill + sky.moon.sky_fill + ambient_lux.max(0.0);
    let sun = contrast(facing_direct(&sky.sun, sun_occlusion), fill + level_direct(&sky.moon, 1.0));
    let moon = contrast(facing_direct(&sky.moon, 1.0), fill + level_direct(&sky.sun, sun_occlusion));
    (sun, moon)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp, reason = "strengths and weights are exact at the band's ends")]

    use super::*;
    use crate::{SkyClock, SkySettings, Weather};

    fn gate() -> SunShadowGate {
        SunShadowGate { enabled: true, ..SunShadowGate::default() }
    }

    fn sky(settings: &SkySettings, hour: f64, weather: &Weather) -> SkyState {
        settings.evaluate(&SkyClock::at(80.0, hour, &settings.calendar), weather)
    }

    /// Steps a casting light from `state` with `contrast` for `seconds` at 60 Hz.
    fn run(g: &SunShadowGate, mut state: GateState, contrast: f32, seconds: f32) -> Vec<GateState> {
        let dt = 1.0 / 60.0;
        let mut out = Vec::new();
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "small positive frame count")]
        for _ in 0..(seconds / dt).round() as u32 {
            state = g.step(state, true, contrast, dt);
            out.push(state);
        }
        out
    }

    #[test]
    fn sky_fill_is_a_share_of_the_beam_and_grows_with_haze() {
        let s = SkySettings::default();
        let clear = sky(&s, 12.0, &Weather::default());
        assert!(clear.sun.sky_fill > 0.0 && clear.sun.sky_fill < clear.sun.illuminance, "{}", clear.sun.sky_fill);
        let hazy = sky(&s, 12.0, &Weather { haze: 40.0, ..Weather::default() });
        let ratio = |k: &SkyState| k.sun.sky_fill / level_direct(&k.sun, 1.0);
        assert!(ratio(&hazy) > 2.0 * ratio(&clear), "{} vs {}", ratio(&hazy), ratio(&clear));
        let night = sky(&s, 0.0, &Weather::default());
        assert!(night.sun.elevation < 0.0);
        assert_eq!(night.sun.sky_fill, 0.0);
    }

    /// Clear noon: full strength at once, maps on (today's behaviour).
    #[test]
    fn clear_noon_keeps_full_cascades() {
        let s = SkySettings::default();
        let k = sky(&s, 12.0, &Weather::default());
        assert!(k.sun.casts_shadows);
        let (sun, _) = contrasts(&k, 1.0, 0.0);
        assert!(sun > 0.5, "clear noon contrast {sun}");
        let g = gate();
        let state = g.step(GateState::default(), true, sun, 1.0 / 60.0);
        assert_eq!(state, GateState::casting(1.0));
        assert!(run(&g, state, sun, 5.0).iter().all(|s| *s == state));
    }

    /// A drowned direct light (heavy haze, full cover, a cloud shadow texture letting a fifth
    /// through, low sun) fades out over `fade_seconds`, monotonically, then turns its maps off.
    #[test]
    fn drowned_direct_light_fades_out_then_drops_its_maps() {
        let s = SkySettings::default();
        let w = Weather { haze: 48.0, cloud_cover: 1.0, ..Weather::default() };
        let g = gate();
        // The evening hour where it sinks under the band (still above the horizon).
        let sun = (0..60)
            .map(|i| sky(&s, 16.0 + f64::from(i) * 0.05, &w))
            .filter(|k| k.sun.casts_shadows && k.sun.elevation > 0.0)
            .map(|k| contrasts(&k, 0.2, 0.0).0)
            .find(|c| *c < g.contrast_off * g.hysteresis)
            .expect("a drowned evening sun");
        let states = run(&g, GateState::casting(1.0), sun, g.fade_seconds + 0.5);
        assert!(states.windows(2).all(|p| p[1].strength <= p[0].strength), "monotonic");
        // Not in one frame: over the fade.
        assert!(states[0].strength > 0.95);
        let half = states.iter().position(|s| s.strength <= 0.5).expect("halfway");
        #[expect(clippy::cast_precision_loss, reason = "frame index")]
        let at = half as f32 / 60.0;
        assert!((at - 0.5 * g.fade_seconds).abs() < 0.1, "half strength after {at} s");
        let last = *states.last().unwrap();
        assert_eq!(last, GateState::casting(0.0));
        // Off stays off while it stays drowned.
        assert!(run(&g, last, sun, 2.0).iter().all(|s| !s.held));
    }

    /// A weather transition: the contrast falls through the band and comes back. Strength
    /// follows monotonically each way, the maps turn off once and on once, and a contrast
    /// wobbling at the edge doesn't flip them (hysteresis).
    #[test]
    fn transition_fades_monotonically_without_flip_flop() {
        let g = gate();
        let dt = 1.0 / 60.0;
        let mut state = g.step(GateState::default(), true, 0.6, dt);
        let mut toggles = 0;
        let mut prev = state;
        // 40 s down from 0.6 to 0, 40 s back up.
        for i in 0..4800u32 {
            #[expect(clippy::cast_precision_loss, reason = "frame index")]
            let t = i as f32 / 2400.0;
            let c = if t < 1.0 { 0.6 * (1.0 - t) } else { 0.6 * (t - 1.0) };
            state = g.step(state, true, c, dt);
            if t < 1.0 {
                assert!(state.strength <= prev.strength + 1e-6, "down at {t}");
            } else {
                assert!(state.strength + 1e-6 >= prev.strength, "up at {t}");
            }
            toggles += u32::from(state.held != prev.held);
            prev = state;
        }
        assert_eq!(toggles, 2, "off once, on once");
        assert_eq!(state.strength, 1.0);
        // Wobble: a holder whose contrast dips 10 % under the band's floor keeps its maps.
        let wobble = |i: u32| if i.is_multiple_of(2) { g.contrast_off * 1.05 } else { g.contrast_off * 0.9 };
        let mut held = GateState::casting(0.2);
        for i in 0..600u32 {
            held = g.step(held, true, wobble(i), dt);
            assert!(held.held, "flipped at frame {i}");
        }
        // An unheld light at the same wobble stays off.
        let mut off = GateState::default();
        for i in 0..600u32 {
            off = g.step(off, true, if i.is_multiple_of(2) { g.contrast_off * 0.99 } else { g.contrast_off * 0.9 }, dt);
            assert!(!off.held);
        }
    }

    /// Night: a dim moon against the ambient floor casts nothing; the elevation rule's
    /// hand-over (`casts` false) resets the state at once.
    #[test]
    fn dim_moon_at_night_gets_no_cascades() {
        let s = SkySettings { moon_shadow_min_elevation_deg: -90.0, ..SkySettings::default() };
        let k = (0..48)
            .map(|h| sky(&s, f64::from(h) * 0.5, &Weather::default()))
            .find(|k| k.moon.casts_shadows && k.moon.illuminance > 0.0)
            .expect("a moonlit hour");
        // A 4-nit ambient floor (`π · 4` lux) drowns the Earth moon's ~0.3 lux.
        let (_, moon) = contrasts(&k, 1.0, std::f32::consts::PI * 4.0);
        let g = gate();
        assert!(moon < g.contrast_off, "moon contrast {moon}");
        assert_eq!(g.step(GateState::default(), true, moon, 0.1), GateState::casting(0.0));
        assert_eq!(g.step(GateState::casting(1.0), false, 1.0, 0.1), GateState::default());
    }

    #[test]
    fn weight_band_and_hysteresis() {
        let g = gate();
        assert_eq!(g.weight(0.0, false), 0.0);
        assert_eq!(g.weight(g.contrast_off, false), 0.0);
        assert_eq!(g.weight(g.contrast_full, false), 1.0);
        assert!((g.weight(f32::midpoint(g.contrast_off, g.contrast_full), false) - 0.5).abs() < 1e-5);
        assert!(g.weight(g.contrast_off * 1.1, true) > g.weight(g.contrast_off * 1.1, false));
        assert_eq!(contrast(0.0, 0.0), 0.0);
        assert!((contrast(1.0, 3.0) - 0.25).abs() < 1e-6);
    }

    /// Back from gated off (the sky clears at once, e.g. a pinned cloud cover removed): the
    /// shadow fades in, it doesn't pop; a fresh caster (walking outdoors) starts at its weight.
    #[test]
    fn return_from_gated_off_fades_in() {
        let g = gate();
        let dt = 1.0 / 60.0;
        let back = g.step(GateState::casting(0.0), true, 0.6, dt);
        assert!(back.held && (back.strength - dt / g.fade_seconds).abs() < 1e-6, "{back:?}");
        let states = run(&g, back, 0.6, g.fade_seconds);
        assert!(states.windows(2).all(|p| p[1].strength >= p[0].strength));
        assert_eq!(states.last().unwrap().strength, 1.0);
        assert_eq!(g.step(GateState::default(), true, 0.6, dt), GateState::casting(1.0));
    }
}
