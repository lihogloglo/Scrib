//! Exact test of the vendored `bevy_grass` near tier (slice GC-3n, own patch "near tier"): the Rust twins of the
//! cull's per-chunk candidate count, the ring's density multiplier and the per-layer keep in `generate.wesl`. Here
//! because cargo cannot test the vendored crate, which is not a workspace member.

// Exact by design: the settings and multipliers compared are exactly representable, and the counts are small.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use bevy_grass::prelude::GrassSettings;
use bevy_grass::superfargoth_test::{TILE_POINTS, candidates_needed, chunk_layout, layer_keep, near_boost};

use crate::blades::{NearTier, with_near_tier};

/// `bevy_grass`'s chunk side, in metres.
const CHUNK: f32 = 16.0;
/// Our blade style's density, blades per m².
const DENSITY: f32 = 150.0;

fn ring() -> GrassSettings {
    with_near_tier(GrassSettings::default(), NearTier { density: 2.5, start_m: 4.0, end_m: 7.0 })
}

#[test]
fn the_crate_default_is_off_and_ours_is_the_tunables() {
    assert_eq!(GrassSettings::default().near_density, 1.0);
    for d in [0.0, 3.0, 5.5, 7.0, 50.0] {
        assert_eq!(near_boost(&GrassSettings::default(), d), 1.0);
    }
    assert_eq!(NearTier::default(), NearTier { density: 2.5, start_m: 4.0, end_m: 7.0 });
}

#[test]
fn the_boost_tapers_linearly_from_start_to_end() {
    let s = ring();
    for (d, want) in [(0.0, 2.5), (2.0, 2.5), (4.0, 2.5), (5.5, 1.75), (7.0, 1.0), (10.0, 1.0), (90.0, 1.0)] {
        assert!((near_boost(&s, d) - want).abs() < 1e-6, "boost at {d} m = {}, want {want}", near_boost(&s, d));
    }
    // Out of range: thinning near the camera is not a near tier, and the ring keeps its order.
    let wrong = GrassSettings { near_density: 0.5, near_start_m: 6.0, near_end_m: 3.0, ..GrassSettings::default() };
    assert_eq!(near_boost(&wrong, 0.0), 1.0);
    let sanitized = wrong.sanitized();
    assert_eq!((sanitized.near_start_m, sanitized.near_end_m), (6.0, 6.0));
}

#[test]
fn near_chunks_get_candidates_for_the_boosted_density_and_far_chunks_are_unchanged() {
    // 150 / m² -> 7 tiles per side, 196 candidates / m²: within 10 m (falloff 1) upstream keeps 150 / 196.
    let (tiles, candidate_density) = chunk_layout(CHUNK, DENSITY);
    assert_eq!(tiles, 7);
    assert!((candidate_density - 196.0).abs() < 1e-3);
    let cells = tiles * tiles;
    let keep = DENSITY / candidate_density;
    let s = ring();
    // Upstream: ceil(0.7653 * 1024) = 784 points per cell. Boosted 2.5x: ceil(1.9133 * 1024) = 1960, two layers.
    let upstream = 784 * cells;
    for (d, want) in [(0.0, 1960 * cells), (4.0, 1960 * cells), (5.5, 1372 * cells), (7.0, upstream), (30.0, upstream)] {
        let boost = near_boost(&s, d);
        let needed = candidates_needed(CHUNK, DENSITY, keep, boost);
        assert_eq!(needed, want, "chunk at {d} m");
        // Enough candidates for density x boost.
        let per_cell = needed / cells;
        assert!(per_cell as f32 / TILE_POINTS as f32 * candidate_density >= DENSITY * boost, "chunk at {d} m");
    }
    // A boost of 1 is upstream's count at any keep, a style denser than its tiles included (keep clamped to 1).
    for keep in [0.0_f32, 0.1, 0.5, 0.7653, 1.0, 2.0] {
        let upstream = (keep.clamp(0.0, 1.0) * TILE_POINTS as f32).ceil() as u32 * cells;
        assert_eq!(candidates_needed(CHUNK, DENSITY, keep, 1.0), upstream, "keep {keep}");
    }
    // At most four layers, whatever the boost.
    assert_eq!(candidates_needed(CHUNK, DENSITY, 1.0, 9.0), 4 * TILE_POINTS * cells);
}

#[test]
fn layers_keep_exactly_the_boosted_density() {
    let keep = DENSITY / 196.0;
    for boost in [1.0, 1.3, 1.75, 2.5, 4.0] {
        let kept: f32 = (0..4).map(|layer| layer_keep(keep, boost, layer)).sum();
        assert!((kept - keep * boost).abs() < 1e-6, "boost {boost}: kept {kept}");
    }
    // Layer 0 at a boost of 1 is upstream's keep; the extra layers keep nothing.
    assert_eq!(layer_keep(keep, 1.0, 0), keep);
    assert_eq!((layer_keep(keep, 1.0, 1), layer_keep(keep, 1.0, 2)), (0.0, 0.0));
    // Layer 0 fills first, then layer 1: 0.7653 x 2.5 = 1.9133.
    assert_eq!(layer_keep(keep, 2.5, 0), 1.0);
    assert!((layer_keep(keep, 2.5, 1) - (keep * 2.5 - 1.0)).abs() < 1e-6);
}
