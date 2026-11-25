//! `AI8d1` light exposure core: exact tests (ai.md "Light exposure").

use glam::Vec3;
use sf_coords::WorldPos;

use crate::*;

const EPS: f32 = 1e-6;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() <= EPS * a.abs().max(b.abs()).max(1.0)
}

fn at() -> WorldPos {
    WorldPos::ORIGIN
}

fn src(x: f32, radius_m: f32, intensity: f32) -> LightSource {
    LightSource { pos: WorldPos::ORIGIN.offset(Vec3::new(x, 0.0, 0.0)), radius_m, intensity, on: true }
}

/// Exposure with every ray clear; returns (exposure, rays cast).
fn clear(sky: f32, covered: bool, sources: &[LightSource], cfg: &LightConfig) -> (f32, usize) {
    let mut casts = 0;
    let mut los = |_: WorldPos, _: WorldPos| {
        casts += 1;
        true
    };
    let e = light_exposure(at(), sky, covered, sources, &mut los, cfg);
    (e, casts)
}

#[test]
fn defaults_match_spec_table() {
    let cfg = LightConfig::default();
    assert!(close(cfg.light_search_m, 15.0));
    assert_eq!(cfg.light_rays, 2);
    assert!(close(cfg.dark_threshold, 0.15));
}

#[test]
fn sky_only_open_and_covered() {
    let cfg = LightConfig::default();
    assert_eq!(clear(0.6, false, &[], &cfg), (0.6, 0));
    let (covered, casts) = clear(0.6, true, &[], &cfg);
    assert!(close(covered, 0.6 * 0.25), "{covered}");
    assert_eq!(casts, 0);
}

#[test]
fn source_at_centre_half_radius_and_beyond() {
    let cfg = LightConfig::default();
    let (centre, _) = clear(0.0, false, &[src(0.0, 10.0, 0.8)], &cfg);
    assert!(close(centre, 0.8), "{centre}");
    // k = 2: 0.8 × (1 − 0.5²)² = 0.8 × 0.5625.
    let (half, _) = clear(0.0, false, &[src(5.0, 10.0, 0.8)], &cfg);
    assert!(close(half, 0.45), "{half}");
    // Beyond its radius (inside light_search_m): nothing, no ray.
    assert_eq!(clear(0.0, false, &[src(12.0, 10.0, 0.8)], &cfg), (0.0, 0));
    // Exactly at the radius: 0.
    assert!(close(source_contribution(&src(0.0, 10.0, 0.8), 10.0, 2.0), 0.0));
}

#[test]
fn falloff_exponent_four_is_the_engine_window() {
    // UE4/Frostbite window: (1 − 0.5⁴)² = 0.87890625.
    assert!(close(source_contribution(&src(0.0, 10.0, 1.0), 5.0, 4.0), 0.878_906_25));
}

#[test]
fn occluded_source_adds_nothing() {
    let cfg = LightConfig::default();
    let mut casts = 0;
    let mut los = |_: WorldPos, _: WorldPos| {
        casts += 1;
        false
    };
    let e = light_exposure(at(), 0.3, false, &[src(1.0, 10.0, 0.8)], &mut los, &cfg);
    assert!(close(e, 0.3), "{e}");
    assert_eq!(casts, 1);
}

#[test]
fn two_rays_pick_the_two_strongest_of_three() {
    let cfg = LightConfig::default();
    let sources = [src(0.0, 10.0, 0.1), src(0.0, 10.0, 0.3), src(0.0, 10.0, 0.2)];
    let mut order = Vec::new();
    let mut los = |_: WorldPos, to: WorldPos| {
        order.push(to);
        true
    };
    let e = light_exposure(at(), 0.0, false, &sources, &mut los, &cfg);
    assert!(close(e, 0.5), "{e}");
    assert_eq!(order.len(), 2);

    // Strongest first: block only the first ray cast, the 0.3 one; the 0.1 one never replaces it.
    let mut first = true;
    let mut los = |_: WorldPos, _: WorldPos| !std::mem::take(&mut first);
    let e = light_exposure(at(), 0.0, false, &sources, &mut los, &cfg);
    assert!(close(e, 0.2), "{e}");
}

#[test]
fn ray_count_bounds() {
    let sources = [src(0.0, 10.0, 0.1), src(0.0, 10.0, 0.1), src(0.0, 10.0, 0.1), src(0.0, 10.0, 0.1), src(0.0, 10.0, 0.1)];
    let none = LightConfig { light_rays: 0, ..LightConfig::default() };
    assert_eq!(clear(0.2, false, &sources, &none), (0.2, 0));
    let many = LightConfig { light_rays: 9, ..LightConfig::default() };
    let (e, casts) = clear(0.0, false, &sources, &many);
    assert_eq!(casts, MAX_LIGHT_RAYS);
    assert!(close(e, 0.4), "{e}");
}

#[test]
fn out_of_search_range_source_ignored() {
    let cfg = LightConfig::default();
    // Its radius reaches us, but it is past light_search_m (15).
    assert_eq!(clear(0.0, false, &[src(20.0, 100.0, 1.0)], &cfg), (0.0, 0));
    let wide = LightConfig { light_search_m: 30.0, ..cfg };
    let (e, casts) = clear(0.0, false, &[src(20.0, 100.0, 1.0)], &wide);
    assert_eq!(casts, 1);
    assert!(close(e, (1.0_f32 - 0.2 * 0.2).powi(2)), "{e}");
}

#[test]
fn total_clamps_at_one() {
    let cfg = LightConfig::default();
    let (e, _) = clear(0.9, false, &[src(0.0, 10.0, 0.8), src(0.0, 10.0, 3.0)], &cfg);
    assert!(close(e, 1.0), "{e}");
}

#[test]
fn darkness_zero_one_two() {
    assert!(close(apply_darkness(0.4, 0.0), 1.0));
    assert!(close(apply_darkness(0.4, 1.0), 0.4));
    assert!(close(apply_darkness(0.4, 2.0), 0.0));
    assert!(close(apply_darkness(0.7, 2.0), 0.4));
}

#[test]
fn dark_threshold() {
    let cfg = LightConfig::default();
    assert!(is_dark(0.1, &cfg));
    assert!(!is_dark(0.15, &cfg));
    assert!(!is_dark(0.5, &cfg));
    assert!(is_dark(f32::NAN, &cfg));
}

#[test]
fn bad_inputs_follow_the_stated_rule() {
    let cfg = LightConfig::default();
    assert_eq!(clear(f32::NAN, false, &[], &cfg), (0.0, 0));
    assert_eq!(clear(-1.0, false, &[], &cfg), (0.0, 0));
    assert_eq!(clear(2.0, false, &[], &cfg), (1.0, 0));
    let off = LightSource { on: false, ..src(0.0, 10.0, 0.8) };
    let bad = [
        off,
        src(0.0, 10.0, f32::NAN),
        src(0.0, 10.0, -0.5),
        src(0.0, 0.0, 0.8),
        src(0.0, -3.0, 0.8),
        src(0.0, f32::INFINITY, 0.8),
        src(0.0, f32::NAN, 0.8),
    ];
    assert_eq!(clear(0.0, false, &bad, &cfg), (0.0, 0));
    let nan_cfg = LightConfig { light_search_m: f32::NAN, roof_sky_mul: f32::NAN, ..cfg };
    assert_eq!(clear(0.5, true, &[src(1.0, 10.0, 0.8)], &nan_cfg), (0.0, 0));
    // Negative distance never happens; a NaN one adds nothing.
    assert!(close(source_contribution(&src(0.0, 10.0, 0.8), f32::NAN, 2.0), 0.0));
    // Darkness: NaN-free inputs clamp; negative darkness counts as 0.
    assert!(close(apply_darkness(-1.0, 1.0), 0.0));
    assert!(close(apply_darkness(0.4, -1.0), 1.0));
}

#[test]
fn ties_keep_the_earlier_source() {
    let cfg = LightConfig { light_rays: 1, ..LightConfig::default() };
    let a = src(1.0, 10.0, 0.5);
    let b = LightSource { pos: WorldPos::ORIGIN.offset(Vec3::new(-1.0, 0.0, 0.0)), ..a };
    let mut hit = None;
    let mut los = |_: WorldPos, to: WorldPos| {
        hit = Some(to);
        true
    };
    light_exposure(at(), 0.0, false, &[a, b], &mut los, &cfg);
    assert_eq!(hit, Some(a.pos));
}

#[test]
fn visibility_factors_modifiers_and_serde_round_trip() {
    let mut f = VisibilityFactors { light: 0.3, stance: Stance::Crouched, speed_mps: 1.5, ..VisibilityFactors::default() };
    for i in 0..VISIBILITY_MODIFIERS_CAPACITY {
        assert!(f.push_modifier(StatId(u16::try_from(i).unwrap()), 10.0));
    }
    assert!(!f.push_modifier(StatId(99), 1.0));
    assert_eq!(f.modifiers().count(), VISIBILITY_MODIFIERS_CAPACITY);
    let copy = f;
    let json = serde_json::to_string(&copy).unwrap();
    let back: VisibilityFactors = serde_json::from_str(&json).unwrap();
    assert_eq!(back, f);
    let neutral = VisibilityFactors::default();
    assert_eq!(neutral.modifiers().count(), 0);
    let back: VisibilityFactors = serde_json::from_str(&serde_json::to_string(&neutral).unwrap()).unwrap();
    assert_eq!(back, neutral);
}
