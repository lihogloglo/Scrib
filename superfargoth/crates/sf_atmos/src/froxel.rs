//! Froxel volumetric fog (weather.md "Fog", F3): the grid, its exponential
//! slice distribution and the integration step, CPU twins of
//! `froxel_common.wesl` (change both together).
//!
//! Technique: Wronski 2014 ("Volumetric fog", Assassin's Creed IV) and
//! Hillaire 2015 ("Physically based and unified volumetric rendering in
//! Frostbite"): a camera-aligned grid of screen tiles × depth slices holds
//! (in-scattered light × scattering, extinction); a second pass integrates
//! it front to back into (in-scatter, transmittance) that any pixel reads
//! with one trilinear fetch at its depth.

/// The froxel grid and its reach (one preset row of weather.md's budget table).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FroxelSettings {
    /// Tiles across, tiles down, depth slices.
    pub size: [u32; 3],
    /// First slice boundary, meters (the slices grow exponentially from here).
    pub near: f32,
    /// Where the froxels end, meters; the analytic layers take over past it.
    pub far: f32,
    /// Weight of the reprojected history, 0..1 (0: no temporal filter).
    pub history: f32,
    /// Clip the reprojected history to the box of this frame's samples around
    /// the froxel (FOG-CLAMP, [`clip_history`]). The clip bounds what stale
    /// history can carry, which is what lets `history` sit at [`HISTORY_CLAMPED`].
    pub clamp: bool,
    /// Point lights lit into the fog (the nearest ones), at most [`MAX_FOG_LIGHTS`].
    pub max_lights: u32,
    /// Where each frame's sample sits inside its froxel.
    pub jitter: FroxelJitter,
}

/// The froxels' per-frame sample placement (weather.md "Fog", "Temporal
/// sampling"). A jittered sample per frame blended into a reprojected
/// history is the standard estimator (Wronski 2014, Hillaire 2015); what
/// matters is that the error it leaves behind is noise, not a signal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FroxelJitter {
    /// Each froxel walks its own low-discrepancy sequence (R3, Roberts 2018)
    /// rotated by a per-froxel hash (Cranley-Patterson rotation), and the
    /// history is read at the froxel's centre. The residual is spatially
    /// uncorrelated noise, which the history and the screen's TAA/DLSS
    /// average away.
    #[default]
    PerFroxel,
    /// One Halton (2, 3, 5) offset for the whole volume, cycling every 16
    /// frames, history read at the jittered point (before 2026-10-19; A/B
    /// only). Every froxel errs in step: an exponential history of weight
    /// 0.9 keeps ~26 % of the cycle's fundamental, so the whole fog pulses
    /// at frame rate / 16, and screen TAA cannot clamp a coherent change.
    Shared,
    /// The froxel centre every frame (diagnosis: no temporal sampling, so
    /// shadow and density edges alias but hold still).
    Off,
}

/// Radical inverse of `i` in `base` (Halton).
#[allow(clippy::cast_precision_loss)]
fn halton(mut i: u32, base: u32) -> f32 {
    let (mut f, mut r) = (1.0, 0.0);
    while i > 0 {
        f /= base as f32;
        r += f * (i % base) as f32;
        i /= base;
    }
    r
}

/// The whole volume's sample offset for `frame`, 0..1 per axis (the
/// uniform's `jitter`); [`FroxelJitter::PerFroxel`] rotates it per froxel
/// by [`froxel_hash`] in the shader.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn frame_jitter(mode: FroxelJitter, frame: u32) -> [f32; 3] {
    match mode {
        FroxelJitter::PerFroxel => {
            // R3: powers of 1/g, g the 3D generalised golden ratio; never
            // repeats and every prefix is evenly spread (Roberts 2018).
            const G: f64 = 1.220_744_084_605_759_5;
            let n = f64::from(frame);
            [1.0 / G, 1.0 / (G * G), 1.0 / (G * G * G)].map(|a| (0.5 + a * n).fract() as f32)
        }
        FroxelJitter::Shared => {
            let j = frame % 16 + 1;
            [halton(j, 2), halton(j, 3), halton(j, 5)]
        }
        FroxelJitter::Off => [0.5; 3],
    }
}

/// A froxel's fixed offset, 0..1 per axis: pcg3d (Jarzynski and Olano
/// 2020). CPU twin of `froxel_inject.wesl`'s `froxel_hash` (keep in step).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn froxel_hash(cell: [u32; 3]) -> [f32; 3] {
    fn mix(v: &mut [u32; 3]) {
        v[0] = v[0].wrapping_add(v[1].wrapping_mul(v[2]));
        v[1] = v[1].wrapping_add(v[2].wrapping_mul(v[0]));
        v[2] = v[2].wrapping_add(v[0].wrapping_mul(v[1]));
    }
    let mut v = cell.map(|c| c.wrapping_mul(1_664_525).wrapping_add(1_013_904_223));
    mix(&mut v);
    v = v.map(|x| x ^ (x >> 16));
    mix(&mut v);
    v.map(|x| (x >> 8) as f32 / 16_777_216.0)
}

/// Where froxel `cell` samples in `frame`, 0..1 inside it (the shader's rule).
#[must_use]
pub fn sample_offset(mode: FroxelJitter, frame: u32, cell: [u32; 3]) -> [f32; 3] {
    let j = frame_jitter(mode, frame);
    if mode != FroxelJitter::PerFroxel {
        return j;
    }
    let h = froxel_hash(cell);
    [(j[0] + h[0]).fract(), (j[1] + h[1]).fract(), (j[2] + h[2]).fract()]
}

/// History weight with the neighbourhood clip (FOG-CLAMP): ~39 frames of
/// effective samples, the TAA weight the clip makes safe (Karis 2014 keeps
/// ~0.04 of the current frame); a froxel's residual noise falls by ~30 %
/// from 0.9 (std ∝ √((1 − w) / (1 + w))).
pub const HISTORY_CLAMPED: f32 = 0.95;
/// History weight without the clip (before FOG-CLAMP, `--off fogclamp`;
/// UE's volumetric-fog default, which leaves its history unclamped too).
pub const HISTORY_UNCLAMPED: f32 = 0.9;

/// Threads per inject workgroup along x and y (`froxel_inject.wesl`
/// `GROUP`). Each group computes a GROUP² block of froxels into workgroup
/// memory and writes the inner [`INJECT_TILE`]² of it: the one-froxel apron
/// is the neighbourhood the history clip reads (≈ 1.31× the froxels computed).
pub const INJECT_GROUP: u32 = 16;
/// Froxels each inject workgroup writes per axis (x, y).
pub const INJECT_TILE: u32 = INJECT_GROUP - 2;

/// The inject dispatch for a `size` grid (one group per tile and slice).
#[must_use]
pub const fn inject_groups(size: [u32; 3]) -> [u32; 3] {
    [size[0].div_ceil(INJECT_TILE), size[1].div_ceil(INJECT_TILE), size[2]]
}

/// The froxel (x, y) that thread `local` of workgroup `group` computes (the
/// shader's rule: the apron is clamped into the grid, so edge froxels
/// repeat), and whether that thread writes it.
#[must_use]
pub fn inject_cell(group: [u32; 2], local: [u32; 2], size: [u32; 2]) -> ([u32; 2], bool) {
    let mut cell = [0; 2];
    let mut writes = true;
    for a in 0..2 {
        // `group × TILE + local − 1`, the apron reaching one froxel out.
        let c = (group[a] * INJECT_TILE + local[a]).checked_sub(1);
        writes &= (1..=INJECT_TILE).contains(&local[a]) && c.is_some_and(|c| c < size[a]);
        cell[a] = c.unwrap_or(0).min(size[a] - 1);
    }
    (cell, writes)
}

/// Clips `prev` (last frame's reprojected value) to the box `lo..hi` of this
/// frame's samples around the froxel, along the line to the box's centre
/// (Karis 2014 "High quality temporal supersampling"; the UE4/Playdead TAA
/// clip). History inside the box is kept as is; stale history (a light gone,
/// a shadow moved, a disocclusion) is pulled to the box's surface. CPU twin
/// of `froxel_inject.wesl`'s `clip_history` (keep in step).
#[must_use]
pub fn clip_history(prev: [f32; 4], lo: [f32; 4], hi: [f32; 4]) -> [f32; 4] {
    let centre: [f32; 4] = std::array::from_fn(|i| f32::midpoint(hi[i], lo[i]));
    let mut m = 1.0_f32;
    for i in 0..4 {
        let e = 0.5 * (hi[i] - lo[i]) + 1e-7;
        m = m.max((prev[i] - centre[i]).abs() / e);
    }
    std::array::from_fn(|i| centre[i] + (prev[i] - centre[i]) / m)
}

/// The froxel shader's light array length (`froxel_common.wesl`).
pub const MAX_FOG_LIGHTS: usize = 32;

impl FroxelSettings {
    /// weather.md Medium: 128×72×48 to 256 m.
    #[must_use]
    pub const fn medium() -> Self {
        Self { size: [128, 72, 48], near: 0.5, far: 256.0, history: HISTORY_CLAMPED, clamp: true, max_lights: 16, jitter: FroxelJitter::PerFroxel }
    }
    /// weather.md High: 160×90×64 (16 px tiles at 1440p) to 384 m.
    #[must_use]
    pub const fn high() -> Self {
        Self { size: [160, 90, 64], near: 0.5, far: 384.0, history: HISTORY_CLAMPED, clamp: true, max_lights: 32, jitter: FroxelJitter::PerFroxel }
    }
    /// weather.md Ultra: 160×90×128 to 512 m (inside the 504 m cascades' reach, give or take).
    #[must_use]
    pub const fn ultra() -> Self {
        Self { size: [160, 90, 128], near: 0.5, far: 512.0, history: HISTORY_CLAMPED, clamp: true, max_lights: 32, jitter: FroxelJitter::PerFroxel }
    }

    /// Distance (along the view ray) of slice coordinate `z` in 0..=1:
    /// 0 at the camera's near boundary, 1 at `far`; exponential, so every
    /// slice is the same fraction of its distance (Frostbite's depth split).
    #[must_use]
    pub fn slice_distance(&self, z: f32) -> f32 {
        self.near * (self.far / self.near).powf(z)
    }

    /// Inverse of [`Self::slice_distance`], clamped to 0..=1.
    #[must_use]
    pub fn slice_coord(&self, d: f32) -> f32 {
        if d <= self.near {
            return 0.0;
        }
        ((d / self.near).ln() / (self.far / self.near).ln()).clamp(0.0, 1.0)
    }

    /// Thickness of slice `s`, meters (slice 0 also covers the camera's first `near` meters).
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn slice_thickness(&self, s: u32) -> f32 {
        let n = self.size[2] as f32;
        let d0 = if s == 0 { 0.0 } else { self.slice_distance(s as f32 / n) };
        self.slice_distance((s + 1) as f32 / n) - d0
    }
}

impl FroxelSettings {
    /// The history as before FOG-CLAMP (`--off fogclamp`): no clip, weight
    /// [`HISTORY_UNCLAMPED`] (a history already off stays off).
    #[must_use]
    pub fn without_clamp(self) -> Self {
        let history = if self.history > 0.0 { HISTORY_UNCLAMPED } else { 0.0 };
        Self { clamp: false, history, ..self }
    }
}

impl Default for FroxelSettings {
    fn default() -> Self {
        Self::high()
    }
}

/// One integration step (Hillaire 2015, energy-conserving): a slab of
/// `thickness` m with in-scattering `scatter` (σs × incoming light) and
/// extinction `sigma_t`, appended behind the running (light, transmittance).
#[must_use]
pub fn integrate_step(light: [f32; 3], transmittance: f32, scatter: [f32; 3], sigma_t: f32, thickness: f32) -> ([f32; 3], f32) {
    let t = (-sigma_t * thickness).exp();
    // ∫ S e^(−σt x) dx over the slab; S·thickness when σt is ~0.
    let k = if sigma_t > 1e-7 { (1.0 - t) / sigma_t } else { thickness };
    let out = [light[0] + transmittance * scatter[0] * k, light[1] + transmittance * scatter[1] * k, light[2] + transmittance * scatter[2] * k];
    (out, transmittance * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices_round_trip_and_cover_the_range() {
        let f = FroxelSettings::high();
        assert!((f.slice_distance(0.0) - f.near).abs() < 1e-5);
        assert!((f.slice_distance(1.0) - f.far).abs() < 1e-2);
        for d in [1.0_f32, 10.0, 100.0, 383.0] {
            assert!((f.slice_distance(f.slice_coord(d)) - d).abs() < d * 1e-4, "{d}");
        }
        let total: f32 = (0..f.size[2]).map(|s| f.slice_thickness(s)).sum();
        assert!((total - f.far).abs() < 1e-2, "{total}");
        // Each slice is ~10 % of its distance at High: 38 m deep at the far end.
        assert!(f.slice_thickness(63) < 40.0 && f.slice_thickness(63) > 36.0);
    }

    /// Std over frames of a region's mean after the shader's history (weight
    /// 0.9): 256 froxels, each straddling a shadow edge across x at its own
    /// place and a density ramp along z (the pulse the owner saw, 2026-10-19).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn region_flicker(mode: FroxelJitter) -> f32 {
        let n = 256_u32;
        let mut hist = vec![0.0_f32; n as usize];
        let mut means = Vec::new();
        for frame in 0..600 {
            for (i, h) in hist.iter_mut().enumerate() {
                let o = sample_offset(mode, frame, [i as u32 % 16, i as u32 / 16, 7]);
                let edge = (i as f32 + 0.5) / n as f32;
                let s = f32::from(u8::from(o[0] < edge)) + o[2];
                *h = if frame == 0 { s } else { s + (*h - s) * 0.9 };
            }
            if frame >= 300 {
                means.push(hist.iter().sum::<f32>() / n as f32);
            }
        }
        let m = means.iter().sum::<f32>() / means.len() as f32;
        (means.iter().map(|x| (x - m) * (x - m)).sum::<f32>() / means.len() as f32).sqrt()
    }

    #[test]
    fn per_froxel_jitter_leaves_noise_not_a_pulse() {
        let shared = region_flicker(FroxelJitter::Shared);
        let per = region_flicker(FroxelJitter::PerFroxel);
        println!("region flicker (std of the mean): shared {shared:.4}, per froxel {per:.4}");
        assert!(shared > 0.02, "the old shared jitter pulses the region: {shared}");
        assert!(per < shared / 4.0 && per < 0.01, "per froxel {per} vs shared {shared}");
        // Unbiased: each froxel's samples spread evenly over its cell.
        for cell in [[0, 0, 0], [3, 9, 1], [159, 89, 63]] {
            let mut sum = [0.0_f32; 3];
            for frame in 0..256 {
                let o = sample_offset(FroxelJitter::PerFroxel, frame, cell);
                for (s, v) in sum.iter_mut().zip(o) {
                    assert!((0.0..1.0).contains(&v));
                    *s += v / 256.0;
                }
            }
            assert!(sum.iter().all(|s| (s - 0.5).abs() < 0.02), "{cell:?}: {sum:?}");
        }
    }

    #[test]
    fn inject_tiles_write_every_froxel_once_with_its_neighbours_at_hand() {
        for size in [FroxelSettings::medium().size, FroxelSettings::high().size, [17, 15, 1]] {
            let g = inject_groups(size);
            assert_eq!(g[2], size[2]);
            let mut writes = vec![0_u32; (size[0] * size[1]) as usize];
            for gy in 0..g[1] {
                for gx in 0..g[0] {
                    let at = |l: [u32; 2]| inject_cell([gx, gy], l, [size[0], size[1]]);
                    for ly in 0..INJECT_GROUP {
                        for lx in 0..INJECT_GROUP {
                            let (c, w) = at([lx, ly]);
                            assert!(c[0] < size[0] && c[1] < size[1]);
                            if !w {
                                continue;
                            }
                            writes[(c[1] * size[0] + c[0]) as usize] += 1;
                            // The 3×3 the clip reads is the clamped neighbours, computed by this group.
                            for (dx, dy) in [(0, 0), (2, 0), (0, 2), (2, 2)] {
                                let (n, _) = at([lx + dx - 1, ly + dy - 1]);
                                let want = [(c[0] + dx).saturating_sub(1).min(size[0] - 1), (c[1] + dy).saturating_sub(1).min(size[1] - 1)];
                                assert_eq!(n, want, "{size:?} group ({gx}, {gy}) local ({lx}, {ly})");
                            }
                        }
                    }
                }
            }
            assert!(writes.iter().all(|&n| n == 1), "{size:?}");
        }
    }

    #[test]
    fn clip_keeps_agreeing_history_and_pulls_stale_history_to_the_box() {
        let (lo, hi) = ([0.0, 0.0, 0.0, 0.1], [1.0, 2.0, 1.0, 0.3]);
        let keep = clip_history([0.5, 1.0, 0.2, 0.2], lo, hi);
        assert!(keep.iter().zip([0.5, 1.0, 0.2, 0.2]).all(|(a, b)| (a - b).abs() < 1e-6), "{keep:?}");
        // Twice the box's half extent out in x: halfway back toward the centre on every axis.
        let c = clip_history([1.5, 1.0, 0.5, 0.2], lo, hi);
        let want = [1.0, 1.0, 0.5, 0.2];
        assert!(c.iter().zip(want).all(|(a, b)| (a - b).abs() < 1e-5), "{c:?}");
        // A flat box (every sample the same) clips to it.
        let f = clip_history([4.0, 0.0, 0.0, 0.0], [1.0; 4], [1.0; 4]);
        assert!(f.iter().all(|v| (v - 1.0).abs() < 1e-3), "{f:?}");
    }

    /// The inject's estimator on a 32×32 slice: a shadow edge across the
    /// froxels at a slant and a density ramp along each froxel's depth, with
    /// the light switched off at frame 300. Returns (mean per-froxel std over
    /// frames 100..300, frames until every froxel is under 2 % of lit).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn estimator(history: f32, clamp: bool) -> (f32, u32) {
        const N: usize = 32;
        let mut hist = vec![0.0_f32; N * N];
        let mut sum = vec![0.0_f32; N * N];
        let mut sum2 = vec![0.0_f32; N * N];
        let mut cleared = u32::MAX;
        for frame in 0..400 {
            let lit = frame < 300;
            let cur: Vec<f32> = (0..N * N)
                .map(|i| {
                    let (x, y) = ((i % N) as u32, (i / N) as u32);
                    let o = sample_offset(FroxelJitter::PerFroxel, frame, [x, y, 7]);
                    let shadowed = (x as f32 + o[0]) + 0.6 * (y as f32 + o[1]) > 22.0;
                    if lit && !shadowed { 1.0 + 0.5 * o[2] } else { 0.0 }
                })
                .collect();
            let next: Vec<f32> = (0..N * N)
                .map(|i| {
                    if frame == 0 || history == 0.0 {
                        return cur[i];
                    }
                    let (x, y) = ((i % N) as i32, (i / N) as i32);
                    let mut prev = hist[i];
                    if clamp {
                        let (mut lo, mut hi) = (f32::MAX, f32::MIN);
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let (nx, ny) = ((x + dx).clamp(0, N as i32 - 1), (y + dy).clamp(0, N as i32 - 1));
                                let v = cur[ny as usize * N + nx as usize];
                                (lo, hi) = (lo.min(v), hi.max(v));
                            }
                        }
                        prev = clip_history([prev, 0.0, 0.0, 0.0], [lo, 0.0, 0.0, 0.0], [hi, 0.0, 0.0, 0.0])[0];
                    }
                    cur[i] + (prev - cur[i]) * history
                })
                .collect();
            hist = next;
            if (100..300).contains(&frame) {
                for (i, h) in hist.iter().enumerate() {
                    sum[i] += h;
                    sum2[i] += h * h;
                }
            }
            if !lit && cleared == u32::MAX && hist.iter().all(|&h| h < 0.02) {
                cleared = frame - 299;
            }
        }
        let std = sum.iter().zip(&sum2).map(|(s, s2)| (s2 / 200.0 - (s / 200.0).powi(2)).max(0.0).sqrt()).sum::<f32>() / (N * N) as f32;
        (std, cleared)
    }

    #[test]
    fn clipped_history_is_quieter_and_drops_a_light_switched_off() {
        let (old_std, old_clear) = estimator(HISTORY_UNCLAMPED, false);
        let (std, clear) = estimator(HISTORY_CLAMPED, true);
        let (std90, clear90) = estimator(HISTORY_UNCLAMPED, true);
        println!("froxel noise (mean temporal std) / frames to clear a light switched off:");
        println!("  unclamped 0.9: {old_std:.4} / {old_clear}; clamped 0.95: {std:.4} / {clear}; clamped 0.9: {std90:.4} / {clear90}");
        // The FOG-CLAMP default is the quieter of the two paths and lets go of a light the old path smears.
        assert!(std < old_std, "{std} vs {old_std}");
        assert!(clear < old_clear && clear <= 3, "{clear} vs {old_clear}");
    }

    #[test]
    fn without_clamp_is_the_old_history() {
        let f = FroxelSettings::high();
        assert!(f.clamp && (f.history - HISTORY_CLAMPED).abs() < 1e-6);
        let old = f.without_clamp();
        assert!(!old.clamp && (old.history - HISTORY_UNCLAMPED).abs() < 1e-6);
        assert!(FroxelSettings { history: 0.0, ..f }.without_clamp().history.abs() < f32::EPSILON, "a history off stays off");
    }

    #[test]
    fn integration_matches_the_closed_form_of_a_homogeneous_medium() {
        // Constant medium: L = S/σt (1 − e^(−σt d)), T = e^(−σt d), however it is sliced.
        let f = FroxelSettings::high();
        let (s, sigma) = (2.0_f32, 0.01_f32);
        let (mut l, mut t) = ([0.0; 3], 1.0);
        for i in 0..f.size[2] {
            (l, t) = integrate_step(l, t, [s * sigma; 3], sigma, f.slice_thickness(i));
        }
        let expect_t = (-sigma * f.far).exp();
        assert!((t - expect_t).abs() < 1e-4, "{t} vs {expect_t}");
        assert!((l[0] - s * (1.0 - expect_t)).abs() < 1e-3, "{}", l[0]);
        // Empty medium: nothing changes.
        let (l0, t0) = integrate_step([0.5; 3], 0.7, [0.0; 3], 0.0, 10.0);
        assert!((l0[0] - 0.5).abs() < 1e-6 && (t0 - 0.7).abs() < 1e-6);
    }
}
