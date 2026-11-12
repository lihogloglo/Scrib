//! The depth-aware ground-fog bank (FOG-BANK; weather.md "Fog", F2), the CPU twin of
//! `sf_atmos_bevy`'s `bank_edges.wesl` (change both together).
//!
//! The cloud trace runs at half resolution (a quarter of it per frame under the checkerboard)
//! and marches the bank noise to the FURTHEST scene depth of a 6x6 full-resolution window. At a
//! depth edge (land against sky) the composite's full-resolution pixels then took a bank value
//! marched to a depth they don't have, and texels flipping population with the jitter turned
//! that into grid-aligned steps (FG-7) and shimmer (WE-10). The standard fix of low-resolution
//! volumetrics (UE5's volumetric clouds and Nubis keep the min and max depth of each low-res
//! texel and upsample depth-aware; Hillaire 2020; Bavoil and Sainz' nearest-depth upsampling) is
//! applied to the bank: the trace records the bank ratio at both the NEAREST and the FURTHEST
//! depth of its window (one march, split at the near depth), and the composite interpolates
//! between them by each pixel's own log-depth, weighting the four taps bilaterally by how far
//! the pixel's depth lies outside each tap's `[near, far]` range.
//!
//! Each (distance, bank) pair is one 32-bit float of the `Rgba32Float` distance target: the
//! integer part a log2 distance in steps of `1 / LOG_STEPS` octave, doubled, plus the bank in
//! `[0, 1]` (`2k + bank`, so 1 never rolls into the next step).

/// Log-distance steps per octave (a step is ~2.2 % of the distance).
pub const LOG_STEPS: f32 = 32.0;
/// Octaves above 1 m the encoding covers (1 m to ~16 700 km).
pub const OCTAVES: f32 = 24.0;
/// Width (octaves) of the composite's bilateral depth weight: a pixel 0.25 octave (~19 %)
/// outside a tap's range weighs `e^-0.5` of one inside it.
pub const DEPTH_SIGMA: f32 = 0.25;
/// The depth weight's floor (× the bilinear weight): with no tap near the pixel's depth, the
/// taps fall back to plain bilinear.
pub const DEPTH_FLOOR: f32 = 1e-4;

/// A (distance m, bank) pair.
pub type Pair = (f32, f32);
/// A composite tap: (bilinear weight, near pair, far pair).
pub type Tap = (f32, Pair, Pair);

/// Packs a distance (m, clamped to `[1, 2^OCTAVES]`) and a bank modulation (clamped to `[0, 1]`).
#[must_use]
pub fn encode(t: f32, bank: f32) -> f32 {
    let k = (t.max(1.0).log2().clamp(0.0, OCTAVES) * LOG_STEPS).round();
    2.0 * k + bank.clamp(0.0, 1.0)
}

/// Unpacks [`encode`]: (distance m, bank).
#[must_use]
pub fn decode(v: f32) -> Pair {
    let k = (v * 0.5).floor().min(OCTAVES * LOG_STEPS);
    ((k / LOG_STEPS).exp2(), (v - 2.0 * k).clamp(0.0, 1.0))
}

/// Where depth `t` lies between a tap's near and far distance in log-depth (0 at `near`, 1 at
/// `far`; clamped; 1 when the range is empty).
#[must_use]
pub fn blend(t: f32, near: f32, far: f32) -> f32 {
    let (ln, lf, lt) = (near.max(1.0).log2(), far.max(1.0).log2(), t.max(1.0).log2());
    if lf - ln <= 1e-4 {
        return 1.0;
    }
    ((lt - ln) / (lf - ln)).clamp(0.0, 1.0)
}

/// The bilateral depth weight of a tap for a pixel at depth `t`: 1 inside the tap's
/// `[near, far]`, a Gaussian in octaves outside (`DEPTH_SIGMA`), never below `DEPTH_FLOOR`.
#[must_use]
pub fn depth_weight(t: f32, near: f32, far: f32) -> f32 {
    let (ln, lf, lt) = (near.max(1.0).log2(), far.max(1.0).log2(), t.max(1.0).log2());
    let e = (ln - lt).max(lt - lf).max(0.0) / DEPTH_SIGMA;
    (-0.5 * e * e).exp() + DEPTH_FLOOR
}

/// A tap's bank for a pixel at depth `t`: its near and far pairs interpolated by log-depth.
#[must_use]
pub fn bank_at(t: f32, near: Pair, far: Pair) -> f32 {
    let s = blend(t, near.0, far.0);
    near.1 + (far.1 - near.1) * s
}

/// The upsampled bank of a pixel at depth `t` over [`Tap`]s: the depth-weighted mean of [`bank_at`].
#[must_use]
pub fn upsample(t: f32, taps: &[Tap]) -> f32 {
    let (mut sum, mut weight) = (0.0, 0.0);
    for &(w, near, far) in taps {
        let w = w * depth_weight(t, near.0, far.0);
        sum += w * bank_at(t, near, far);
        weight += w;
    }
    if weight > 0.0 { sum / weight } else { 1.0 }
}

/// The share of a march interval `[a, b]` lying before the near depth `d` (the trace's split:
/// the near ratio accumulates each sample by it).
#[must_use]
pub fn near_share(a: f32, b: f32, d: f32) -> f32 {
    if b <= a {
        return if d >= b { 1.0 } else { 0.0 };
    }
    ((d - a) / (b - a)).clamp(0.0, 1.0)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact: clamped ends and range limits
mod tests {
    use super::*;

    #[test]
    fn pairs_round_trip() {
        for &t in &[1.0_f32, 7.5, 300.0, 2_500.0, 6_000.0, 20_000.0, 1.0e6] {
            for &b in &[0.0_f32, 1e-3, 0.37, 0.999, 1.0] {
                let (t2, b2) = decode(encode(t, b));
                assert!((t2 / t - 1.0).abs() < 0.012, "t {t} -> {t2}");
                assert!((b2 - b).abs() < 2e-4, "bank {b} -> {b2} at {t}");
            }
        }
        // Clamped ends: below 1 m, past the range, banks outside [0, 1].
        assert_eq!(decode(encode(0.0, 0.5)).0, 1.0);
        assert!((decode(encode(1e30, 0.5)).0 - OCTAVES.exp2()).abs() < 1.0);
        assert_eq!(decode(encode(100.0, 2.0)).1, 1.0);
        assert_eq!(decode(encode(100.0, -1.0)).1, 0.0);
        // Distances keep their order.
        assert!(decode(encode(1_000.0, 1.0)).0 < decode(encode(1_100.0, 0.0)).0);
    }

    #[test]
    fn blend_and_weights() {
        assert_eq!(blend(100.0, 100.0, 6_000.0), 0.0);
        assert_eq!(blend(6_000.0, 100.0, 6_000.0), 1.0);
        assert_eq!(blend(50.0, 100.0, 6_000.0), 0.0);
        assert!((blend(1_000.0, 100.0, 10_000.0) - 0.5).abs() < 1e-5);
        assert_eq!(blend(5.0, 300.0, 300.0), 1.0);
        assert_eq!(depth_weight(500.0, 100.0, 6_000.0), 1.0 + DEPTH_FLOOR);
        assert!(depth_weight(12_000.0, 100.0, 6_000.0) < 1e-3);
        assert!(depth_weight(110.0, 120.0, 120.0) > 0.8);
        assert!(depth_weight(40.0, 100.0, 6_000.0) > DEPTH_FLOOR * 0.99);
    }

    /// The FG-7 edge: land at 2 km beside sky. Taps that saw the sky (near 2 km, far 6 km) and
    /// taps that saw land only (2 km both) give a land pixel the land's bank, a sky pixel the
    /// sky's, whatever the mix of taps (a population flip changes nothing).
    #[test]
    fn edge_pixels_take_their_own_depth() {
        let land = (2_000.0, 0.3);
        let sky = (6_000.0, 0.9);
        let edge = (1.0, land, sky);
        let inner = (1.0, land, land);
        for taps in [vec![edge, edge, edge, edge], vec![edge, inner, inner, edge], vec![inner; 4]] {
            assert!((upsample(2_000.0, &taps) - 0.3).abs() < 1e-5);
        }
        assert!((upsample(6_000.0, &[edge, edge, inner, inner]) - 0.9).abs() < 1e-3);
        // No tap at the pixel's depth: the bilinear mean of the closest values.
        let lone = upsample(50.0, &[edge, inner]);
        assert!((0.0..=1.0).contains(&lone) && lone.is_finite());
    }

    #[test]
    fn near_share_splits_intervals() {
        assert_eq!(near_share(0.0, 100.0, 150.0), 1.0);
        assert_eq!(near_share(0.0, 100.0, -1.0), 0.0);
        assert!((near_share(100.0, 300.0, 150.0) - 0.25).abs() < 1e-6);
        assert_eq!(near_share(5.0, 5.0, 5.0), 1.0);
    }
}
