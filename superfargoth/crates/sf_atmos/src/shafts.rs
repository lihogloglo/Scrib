//! Screen-space light shaft occlusion (`docs/specs/aerial-shadow.md` "S3", ROADMAP 1.27 S3a, FG-12):
//! the CPU side of the classic post-process (Mitchell, GPU Gems 3 ch. 13; UE "Light Shaft
//! Occlusion"). A mask from the depth buffer (open sky = 1) is blurred radially toward the sun's
//! screen position in a few passes of a few taps each, each pass reaching further; the blurred mask
//! `o` ("how much of the way to the sun is open sky") then dims the haze's forward lobe toward the
//! sun: `1 − darkness · (1 − o) · lobe`.
//!
//! Here: where the sun sits on screen (none behind the camera), how strongly the effect applies as
//! the sun leaves the screen, the forward lobe normalised to 1 at the sun, the multiplier, and the
//! step of each blur pass. The shaders (`sf_atmos_bevy` `shaders/shafts_*.wesl`, vendored
//! `render_sky.wesl` patch "own, LSO") apply the same formulas; change both together.

use glam::{Mat4, Vec2, Vec3};

/// Below this clip `w` (view-space depth of the direction) the sun counts as behind the camera:
/// at 90° to the view its screen position runs off to infinity.
// tunables: const (degenerate-projection guard: a direction at 90° to the view)
const MIN_CLIP_W: f32 = 1e-4;

/// The largest |g| [`lobe`] takes (g = ±1 is a delta lobe).
// tunables: const (maths: keeps the phase function finite, g = ±1 is a delta)
const MAX_G: f32 = 0.999;

/// The sun's screen position in UV (0..1 across the view, y down, as the fullscreen passes'
/// `in.uv`), from the view's projection and the direction toward the sun in **view** space (a
/// direction: the camera's translation does not move it). `None` when the sun is behind the
/// camera (or at 90° to the view): the effect is off.
#[must_use]
pub fn sun_uv(clip_from_view: Mat4, toward_sun_view: Vec3) -> Option<Vec2> {
    let clip = clip_from_view * toward_sun_view.extend(0.0);
    if clip.w <= MIN_CLIP_W {
        return None;
    }
    let ndc = Vec2::new(clip.x, clip.y) / clip.w;
    Some(Vec2::new(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5))
}

/// How strongly the effect applies with the sun at `uv`: 1 on screen, fading smoothly to 0 as the
/// sun's distance outside the screen (in UV units, the larger of the two axes) reaches `fade`.
/// `fade <= 0`: a hard cut at the screen's edge.
#[must_use]
pub fn offscreen_strength(uv: Vec2, fade: f32) -> f32 {
    let outside = (-uv.x).max(uv.x - 1.0).max((-uv.y).max(uv.y - 1.0)).max(0.0);
    if fade <= 0.0 {
        return if outside > 0.0 { 0.0 } else { 1.0 };
    }
    let x = (outside / fade).clamp(0.0, 1.0);
    1.0 - x * x * (3.0 - 2.0 * x)
}

/// The haze's forward lobe toward the sun, a Henyey-Greenstein phase with asymmetry `g`
/// normalised to 1 at the sun: `HG(g, cos θ) / HG(g, 1) = ((1 − g)² / (1 + g² − 2g cos θ))^1.5`.
/// `cos_theta` is the cosine between the view ray and the direction toward the sun. g = 0: 1
/// everywhere (isotropic haze, all of it masked).
#[must_use]
pub fn lobe(g: f32, cos_theta: f32) -> f32 {
    let g = g.clamp(-MAX_G, MAX_G);
    // `1 + g² − 2g cos θ` written as `(1 − g)² + 2g (1 − cos θ)`: exactly `(1 − g)²` at the sun.
    let denom = ((1.0 - g) * (1.0 - g) + 2.0 * g * (1.0 - cos_theta.clamp(-1.0, 1.0))).max(1e-6);
    ((1.0 - g) * (1.0 - g) / denom).powf(1.5).min(1.0)
}

/// The multiplier on the sun's in-scattering: `1 − darkness · (1 − open) · lobe`, in `[0, 1]`.
/// `open` is the blurred mask (1: the way to the sun is open sky), `darkness` the occlusion's
/// strength (already scaled by [`offscreen_strength`]).
#[must_use]
pub fn occlusion(darkness: f32, open: f32, lobe: f32) -> f32 {
    (1.0 - darkness.clamp(0.0, 1.0) * (1.0 - open.clamp(0.0, 1.0)) * lobe.clamp(0.0, 1.0)).clamp(0.0, 1.0)
}

/// The step of blur pass `k` (0-based) of `passes`, `taps` taps each, as a fraction of the vector
/// from the pixel to the sun: tap `j` of pass `k` samples the previous pass at
/// `uv + (sun − uv) · j · step(k)`. Each pass steps `taps` times further than the one before (its
/// taps fall on the gaps the earlier passes filled: one long march of `taps^passes` samples, no
/// noise), and all passes together reach `length` of the way to the sun:
/// `(taps − 1) · Σ step(k) = length`. Zero for `taps < 2` or `passes == 0`.
#[must_use]
pub fn pass_step(k: u32, passes: u32, taps: u32, length: f32) -> f32 {
    if taps < 2 || passes == 0 || k >= passes {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let n = taps as f32;
    #[allow(clippy::cast_possible_wrap)]
    let first = length / (n.powi(passes as i32) - 1.0);
    #[allow(clippy::cast_possible_wrap)]
    let grown = n.powi(k as i32);
    first * grown
}

/// Rec. 709 luminance weights (Bevy's scene colour is linear Rec. 709).
// tunables: const (maths: the colour space's luminance weights)
pub const LUMINANCE: Vec3 = Vec3::new(0.2126, 0.7152, 0.0722);

/// Below this luminance a colour counts as black (no division by ~0).
// tunables: const (numerical guard: f16 scene colour's smallest normal is 6.1e-5)
const MIN_LUMINANCE: f32 = 6.1e-5;

/// The planet-shadow terminator's half width in the cosine to the zenith (≈ 0.5°): the twin of
/// `planet_lit_at` in `sf_atmos_bevy` `key_light.wesl`.
// tunables: const (twin of key_light.wesl planet_lit_at's smoothing: change both together)
const TERMINATOR_HALF_WIDTH: f32 = 0.004;

/// Light shaft bloom's source colour (S3b; UE "Light Shaft Bloom" `BloomMaxBrightness`,
/// `BloomThreshold`) of one **pre-exposed** scene colour: its luminance is first rescaled down to
/// `max` (UE: "after exposure is applied, scene color brightness larger than Bloom Max Brightness
/// is rescaled down to it"), then only what exceeds `threshold` blooms; the hue is kept:
/// `rgb · max(min(L, max) − threshold, 0) / L`.
#[must_use]
pub fn bloom_color(rgb: Vec3, threshold: f32, max: f32) -> Vec3 {
    let rgb = rgb.max(Vec3::ZERO);
    let l = rgb.dot(LUMINANCE);
    if l <= MIN_LUMINANCE {
        return Vec3::ZERO;
    }
    let kept = (l.min(max.max(0.0)) - threshold.max(0.0)).max(0.0);
    rgb * (kept / l)
}

/// The bloom source's weight around the sun's screen position: `(1 − d / radius)²`, 0 from
/// `radius` on, `d` the distance from `sun` to `uv` in screen heights (`aspect` = width / height:
/// a round falloff on a wide screen). `radius <= 0`: none.
#[must_use]
pub fn bloom_falloff(uv: Vec2, sun: Vec2, aspect: f32, radius: f32) -> f32 {
    if radius <= 0.0 {
        return 0.0;
    }
    let d = ((uv - sun) * Vec2::new(aspect, 1.0)).length();
    let w = 1.0 - (d / radius).clamp(0.0, 1.0);
    w * w
}

/// How much of the sun the camera sees past the planet's shadow (the night gate of the bloom):
/// 1 above the horizon, 0 below it, smoothed over ≈ 0.5° as the clouds' and fog's key light
/// (`key_light.wesl` `planet_lit_at`, its twin). `camera` is the camera's position from the
/// planet's centre, `inner_radius` the planet's radius (same units), `toward_sun` a unit vector.
#[must_use]
pub fn sun_seen(camera: Vec3, inner_radius: f32, toward_sun: Vec3) -> f32 {
    let r = camera.length();
    if r <= 0.0 {
        return 1.0;
    }
    let mu = (camera / r).dot(toward_sun);
    let rho = inner_radius / r.max(inner_radius);
    let mu_horizon = -(1.0 - rho * rho).max(0.0).sqrt();
    smoothstep(mu_horizon - TERMINATOR_HALF_WIDTH, mu_horizon + TERMINATOR_HALF_WIDTH, mu)
}

/// WGSL's `smoothstep`.
fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// How far toward the sun the passes `0..=k` reach together: `(taps − 1) · Σ step(0..=k)`.
#[must_use]
pub fn reach(k: u32, passes: u32, taps: u32, length: f32) -> f32 {
    #[allow(clippy::cast_precision_loss)]
    let gaps = taps.saturating_sub(1) as f32;
    (0..=k.min(passes.saturating_sub(1))).map(|i| pass_step(i, passes, taps, length)).sum::<f32>() * gaps
}

#[cfg(test)]
#[allow(clippy::float_cmp, reason = "exact: the formulas return these constants (1, 0, a clamp)")]
mod tests {
    use super::*;

    fn projection() -> Mat4 {
        reverse_z(1.0)
    }

    /// Bevy's infinite reverse-Z perspective (`ExtractedView::clip_from_view`), 90° vertical.
    pub(crate) fn reverse_z(aspect: f32) -> Mat4 {
        let (f, near) = (1.0, 0.1);
        Mat4::from_cols(
            glam::Vec4::new(f / aspect, 0.0, 0.0, 0.0),
            glam::Vec4::new(0.0, f, 0.0, 0.0),
            glam::Vec4::new(0.0, 0.0, 0.0, -1.0),
            glam::Vec4::new(0.0, 0.0, near, 0.0),
        )
    }

    #[test]
    fn the_sun_straight_ahead_sits_in_the_screens_centre() {
        let uv = sun_uv(projection(), Vec3::NEG_Z).expect("in front");
        assert!((uv - Vec2::splat(0.5)).length() < 1e-6, "{uv}");
    }

    #[test]
    fn up_and_right_map_to_uv_up_and_right() {
        // 45° up at a 90° vertical fov: the top edge (uv.y 0); 45° right: the right edge.
        let up = sun_uv(projection(), Vec3::new(0.0, 1.0, -1.0).normalize()).expect("in front");
        assert!((up - Vec2::new(0.5, 0.0)).length() < 1e-5, "{up}");
        let right = sun_uv(projection(), Vec3::new(1.0, 0.0, -1.0).normalize()).expect("in front");
        assert!((right - Vec2::new(1.0, 0.5)).length() < 1e-5, "{right}");
    }

    #[test]
    fn a_sun_behind_or_beside_the_camera_turns_the_effect_off() {
        assert_eq!(sun_uv(projection(), Vec3::Z), None);
        assert_eq!(sun_uv(projection(), Vec3::new(0.3, 0.2, 0.5).normalize()), None);
        assert_eq!(sun_uv(projection(), Vec3::X), None, "at 90° to the view");
    }

    #[test]
    fn the_strength_fades_as_the_sun_leaves_the_screen() {
        assert_eq!(offscreen_strength(Vec2::new(0.5, 0.5), 0.3), 1.0);
        assert_eq!(offscreen_strength(Vec2::new(0.0, 1.0), 0.3), 1.0, "on the edge");
        assert!((offscreen_strength(Vec2::new(1.15, 0.5), 0.3) - 0.5).abs() < 1e-6, "half way: smoothstep 0.5");
        assert!((offscreen_strength(Vec2::new(0.5, -0.15), 0.3) - 0.5).abs() < 1e-6, "above the screen");
        assert_eq!(offscreen_strength(Vec2::new(1.3, 0.5), 0.3), 0.0);
        assert_eq!(offscreen_strength(Vec2::new(-2.0, 3.0), 0.3), 0.0);
        let a = offscreen_strength(Vec2::new(1.05, 0.5), 0.3);
        let b = offscreen_strength(Vec2::new(1.2, 0.5), 0.3);
        assert!(1.0 > a && a > b && b > 0.0, "monotonic: {a} {b}");
        assert_eq!(offscreen_strength(Vec2::new(1.01, 0.5), 0.0), 0.0, "fade 0: a hard cut");
        assert_eq!(offscreen_strength(Vec2::new(0.99, 0.5), 0.0), 1.0);
    }

    #[test]
    fn the_lobe_is_one_at_the_sun_and_falls_off() {
        for g in [0.5, 0.7, 0.8, 0.95] {
            assert!((lobe(g, 1.0) - 1.0).abs() < 1e-6, "g {g}");
            let mut last = 1.0;
            for cos in [0.99, 0.9, 0.5, 0.0, -1.0] {
                let l = lobe(g, cos);
                assert!(l < last && l > 0.0, "g {g} cos {cos}: {l} after {last}");
                last = l;
            }
        }
        // g = 0.8 (the haze's Mie asymmetry): at 90° from the sun the lobe is ((0.04 / 1.64)^1.5).
        assert!((lobe(0.8, 0.0) - (0.04f32 / 1.64).powf(1.5)).abs() < 1e-6);
        assert!((lobe(0.0, -0.3) - 1.0).abs() < 1e-6, "isotropic");
    }

    #[test]
    fn the_occlusion_masks_only_what_is_closed_toward_the_sun() {
        assert_eq!(occlusion(0.95, 1.0, 1.0), 1.0, "open sky all the way: untouched");
        assert!((occlusion(0.95, 0.0, 1.0) - 0.05).abs() < 1e-6, "closed, at the sun: 1 - darkness");
        assert!((occlusion(0.95, 0.5, 1.0) - 0.525).abs() < 1e-6);
        assert_eq!(occlusion(0.95, 0.0, 0.0), 1.0, "side-lit haze is untouched");
        assert_eq!(occlusion(0.0, 0.0, 1.0), 1.0, "darkness 0 (sun off screen): stock");
    }

    #[test]
    fn each_pass_reaches_taps_times_further_and_all_reach_the_length() {
        let (passes, taps, length) = (3, 8, 1.0);
        let s: Vec<f32> = (0..passes).map(|k| pass_step(k, passes, taps, length)).collect();
        assert!((s[0] - 1.0 / 511.0).abs() < 1e-7, "{s:?}");
        assert!((s[1] / s[0] - 8.0).abs() < 1e-4 && (s[2] / s[1] - 8.0).abs() < 1e-4, "{s:?}");
        // Pass k's step is the span the earlier passes covered plus one: no gaps, no overlap.
        assert!((s[1] - (reach(0, passes, taps, length) + s[0])).abs() < 1e-6);
        assert!((reach(passes - 1, passes, taps, length) - length).abs() < 1e-5);
        assert!(reach(0, passes, taps, length) < reach(1, passes, taps, length));
        assert!((reach(1, 2, 4, 0.5) - 0.5).abs() < 1e-6);
        assert_eq!(pass_step(0, 3, 1, 1.0), 0.0, "one tap: no blur");
        assert_eq!(pass_step(0, 0, 8, 1.0), 0.0, "no pass");
        assert_eq!(pass_step(3, 3, 8, 1.0), 0.0, "past the last pass");
    }

    #[test]
    fn bloom_takes_only_what_exceeds_the_threshold_clamped_to_the_max_with_its_hue() {
        let grey = |l: f32| Vec3::splat(l);
        assert_eq!(bloom_color(grey(5.0), 8.0, 100.0), Vec3::ZERO, "below the threshold");
        assert_eq!(bloom_color(grey(8.0), 8.0, 100.0), Vec3::ZERO, "at it");
        assert!((bloom_color(grey(10.0), 8.0, 100.0) - grey(2.0)).length() < 1e-5, "the excess");
        assert!((bloom_color(grey(5000.0), 8.0, 100.0) - grey(92.0)).length() < 1e-3, "clamped to max, then thresholded");
        assert!((bloom_color(grey(3.0), 0.0, 100.0) - grey(3.0)).length() < 1e-5, "threshold 0: all of it");
        assert_eq!(bloom_color(grey(50.0), 8.0, 4.0), Vec3::ZERO, "max below the threshold: none");
        assert_eq!(bloom_color(Vec3::ZERO, 0.0, 100.0), Vec3::ZERO, "black");
        assert_eq!(bloom_color(Vec3::new(-1.0, -2.0, -3.0), 0.0, 100.0), Vec3::ZERO, "negative input");
        // A warm sun: luminance 20 → 12 kept, same hue (ratios between channels unchanged).
        let warm = Vec3::new(40.0, 16.0, 4.0) * (20.0 / Vec3::new(40.0, 16.0, 4.0).dot(LUMINANCE));
        let b = bloom_color(warm, 8.0, 100.0);
        assert!((b.dot(LUMINANCE) - 12.0).abs() < 1e-4, "{b}");
        assert!((b / b.x - warm / warm.x).length() < 1e-5, "hue kept: {b} vs {warm}");
    }

    #[test]
    fn the_bloom_falls_off_round_the_sun() {
        let sun = Vec2::new(0.5, 0.25);
        assert_eq!(bloom_falloff(sun, sun, 16.0 / 9.0, 0.5), 1.0, "at the sun");
        // 0.25 screen heights below: half the radius → (1 - 0.5)^2.
        assert!((bloom_falloff(Vec2::new(0.5, 0.5), sun, 16.0 / 9.0, 0.5) - 0.25).abs() < 1e-6);
        // Sideways the same 0.25 screen heights is 0.25 / aspect in UV: the falloff is round.
        let side = bloom_falloff(Vec2::new(0.5 + 0.25 * 9.0 / 16.0, 0.25), sun, 16.0 / 9.0, 0.5);
        assert!((side - 0.25).abs() < 1e-5, "{side}");
        assert_eq!(bloom_falloff(Vec2::new(0.5, 0.75), sun, 16.0 / 9.0, 0.5), 0.0, "at the radius");
        assert_eq!(bloom_falloff(Vec2::new(1.0, 1.0), sun, 16.0 / 9.0, 0.5), 0.0, "past it");
        assert_eq!(bloom_falloff(sun, sun, 1.0, 0.0), 0.0, "radius 0: none");
        let (a, b) = (bloom_falloff(Vec2::new(0.5, 0.3), sun, 1.0, 0.5), bloom_falloff(Vec2::new(0.5, 0.4), sun, 1.0, 0.5));
        assert!(1.0 > a && a > b && b > 0.0, "monotonic: {a} {b}");
    }

    #[test]
    fn the_bloom_stops_once_the_planet_hides_the_sun() {
        let radius = 6_360_000.0;
        let ground = Vec3::new(0.0, radius + 2.0, 0.0);
        assert_eq!(sun_seen(ground, radius, Vec3::Y), 1.0, "noon");
        assert_eq!(sun_seen(ground, radius, Vec3::new(1.0, 0.1, 0.0).normalize()), 1.0, "low sun, above the horizon");
        assert_eq!(sun_seen(ground, radius, Vec3::new(1.0, -0.1, 0.0).normalize()), 0.0, "set: night");
        assert_eq!(sun_seen(ground, radius, Vec3::NEG_Y), 0.0, "midnight");
        // At the camera's horizon (2 m up: mu_h = -sqrt(1 - (R/r)^2) ≈ -0.0008) the gate is half way.
        let r = ground.length();
        let mu_h = -(1.0 - (radius / r) * (radius / r)).sqrt();
        let horizon = Vec3::new((1.0 - mu_h * mu_h).sqrt(), mu_h, 0.0);
        assert!((sun_seen(ground, radius, horizon) - 0.5).abs() < 1e-3, "{}", sun_seen(ground, radius, horizon));
        // 10 km up the horizon dips (≈ 3.2°): a sun 2° below the flat horizon is still seen.
        let high = Vec3::new(0.0, radius + 10_000.0, 0.0);
        let dip = Vec3::new(2f32.to_radians().cos(), -(2f32.to_radians().sin()), 0.0);
        assert_eq!(sun_seen(high, radius, dip), 1.0, "seen past the dipped horizon");
        assert_eq!(sun_seen(Vec3::ZERO, radius, Vec3::Y), 1.0, "degenerate centre: ungated");
    }
}
