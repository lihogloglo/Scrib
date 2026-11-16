//! SW-6 (owner 2026-11-16, Skaal Village `surface set snow 1`): the raised cover stood as a vertical,
//! jagged wall along drift and layer borders. These tests run the CPU twin of the raise (`layer.wesl`
//! `surface_raise_tops` / `surface_cover_bury`, terrain `layer_raise_top`) along 1-D ground profiles
//! at 0.05 m steps: the new field never stands steeper than the repose angle for any tops (real noise
//! atlas, any layer p90s, even random texture data), keeps the 2026-11-15 bury intent, and the old one
//! is measured for the record (`#[ignore = "measurement: ..."]`).

#![allow(
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names
)]

use crate::layer::{COVER_DRIFT_TILE_M, COVER_TOP_MAX, bury_gain, cover_bury, cover_height, prefilter_level, raise_filter_m};
use crate::trail::{LIP_HEIGHT, cover_raise, trail_height};
use crate::view::{NOISE_SIZE, noise_atlas, noise_atlas_mips};

/// The knee-deep snow look's depth (m), as W6b ships it.
const DEPTH: f32 = 0.45;
/// The repose the defaults assume (`terrain.raise.repose_deg`).
const REPOSE_DEG: f32 = 30.0;
/// Terrain's coarsest layer level texel (m; `common.wesl` `layer_raise_top`: 0.5 m x `2^LAYER_LEVELS`).
const BASE_TEXEL_M: f32 = 4.0;
/// The minimum filter (`terrain.raise.filter_min_m`).
const MIN_FILTER_M: f32 = 0.5;
/// The profile step (m).
const STEP: f32 = 0.05;
/// Sampling slack on the bound (finite differences of a smooth field at 0.05 m).
const EPS: f32 = 1e-3;

fn tan_repose() -> f32 {
    REPOSE_DEG.to_radians().tan()
}

fn filter_m() -> f32 {
    raise_filter_m(DEPTH, tan_repose(), MIN_FILTER_M, BASE_TEXEL_M)
}

fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343) ^ (y as u32).wrapping_mul(0xd816_3841) ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^= h >> 12;
    (h >> 8) as f32 / 16_777_215.0
}

/// A mip chain of one channel, mip 0 first (`size` texels square).
struct Chain {
    size: usize,
    levels: Vec<Vec<f32>>,
}

impl Chain {
    /// The atlas's G channel (the drift), all levels as the GPU image holds them.
    fn drift() -> Self {
        let mips = noise_atlas_mips();
        let mut levels = Vec::new();
        let (mut n, mut at) = (NOISE_SIZE as usize, 0);
        while n >= 1 {
            levels.push(mips[at..at + n * n].iter().map(|t| f32::from(t[1]) / 255.0).collect());
            at += n * n;
            if n == 1 {
                break;
            }
            n /= 2;
        }
        Self { size: NOISE_SIZE as usize, levels }
    }

    /// Random texels (any data a texture could hold) box-mipped like [`noise_atlas_mips`].
    fn random(seed: u32) -> Self {
        let n = NOISE_SIZE as usize;
        let mut levels = vec![(0..n * n).map(|i| hash((i % n) as i32, (i / n) as i32, seed)).collect::<Vec<_>>()];
        let mut m = n;
        while m > 1 {
            let h = m / 2;
            let up = levels.last().expect("a level");
            let next = (0..h * h)
                .map(|i| {
                    let (x, y) = (i % h, i / h);
                    (up[2 * y * m + 2 * x] + up[2 * y * m + 2 * x + 1] + up[(2 * y + 1) * m + 2 * x] + up[(2 * y + 1) * m + 2 * x + 1]) / 4.0
                })
                .collect();
            levels.push(next);
            m = h;
        }
        Self { size: n, levels }
    }

    /// Bilinear, repeating, at `uv` on level `l` (texel centres at (i + 0.5) / n).
    fn bilinear(&self, l: usize, uv: [f32; 2]) -> f32 {
        let n = (self.size >> l).max(1);
        let tex = &self.levels[l];
        let (x, y) = (uv[0] * n as f32 - 0.5, uv[1] * n as f32 - 0.5);
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let at = |i: f32, j: f32| {
            let (i, j) = ((i as i64).rem_euclid(n as i64) as usize, (j as i64).rem_euclid(n as i64) as usize);
            tex[j * n + i]
        };
        let a = at(x0, y0) + (at(x0 + 1.0, y0) - at(x0, y0)) * fx;
        let b = at(x0, y0 + 1.0) + (at(x0 + 1.0, y0 + 1.0) - at(x0, y0 + 1.0)) * fx;
        a + (b - a) * fy
    }

    /// `textureSampleLevel` with a linear mip filter at `level` (clamped to the chain).
    fn sample(&self, uv: [f32; 2], level: f32) -> f32 {
        let top = (self.levels.len() - 1) as f32;
        let level = level.clamp(0.0, top);
        let l0 = level.floor();
        let f = level - l0;
        let a = self.bilinear(l0 as usize, uv);
        if f <= 0.0 {
            return a;
        }
        a + (self.bilinear((l0 as usize + 1).min(self.levels.len() - 1), uv) - a) * f
    }

    /// The drift at render XZ `xz` (`layer.wesl` `surface_cover_drift_at`).
    fn drift_at(&self, xz: [f32; 2], level: f32) -> f32 {
        self.sample([xz[0] / COVER_DRIFT_TILE_M + 0.17, xz[1] / COVER_DRIFT_TILE_M + 0.83], level)
    }
}

/// The drift's level for the raise filter `w` (`surface_prefilter_level` on the atlas's texel).
fn drift_level(w: f32) -> f32 {
    prefilter_level(w, COVER_DRIFT_TILE_M / NOISE_SIZE as f32)
}

/// Bilinear of per-texel values `v(i, j)` on a lattice of `s` m (terrain `layer_raise_top`).
fn bilinear_lattice(v: &impl Fn(i32, i32) -> f32, xz: [f32; 2], s: f32) -> f32 {
    let (x, y) = (xz[0] / s, xz[1] / s);
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let (i, j) = (x0 as i32, y0 as i32);
    let a = v(i, j) + (v(i + 1, j) - v(i, j)) * fx;
    let b = v(i, j + 1) + (v(i + 1, j + 1) - v(i, j + 1)) * fx;
    a + (b - a) * fy
}

/// The new raise (m) at `xz`: one channel of `DEPTH`, amount `a`, the layers' p90 per base texel `p90`,
/// the drift chain, the look's mean `look`.
fn raise_new(a: f32, p90: &impl Fn(i32, i32) -> f32, drift: &Chain, look: f32, xz: [f32; 2]) -> f32 {
    let w = filter_m();
    let base = bilinear_lattice(p90, xz, BASE_TEXEL_M);
    let top = cover_height(base, look, drift.drift_at(xz, drift_level(w)));
    DEPTH * cover_bury(a, top)
}

/// The old bury (W6b, 2026-11-15): `saturate((a - t) / (1 - t))`, top clamped to 0.999.
fn bury_old(a: f32, top: f32) -> f32 {
    let t = top.clamp(0.0, 0.999);
    ((a - t) / (1.0 - t)).clamp(0.0, 1.0)
}

/// The old top's base (`layer_blend_top`, 2026-11-15): two layers whose splat weight ramps over the
/// finest layer level's 1 m texels across x = 0, each layer's height prefiltered to 0.5 m (a value
/// noise of 0.5 m texels, 0.5 +- 0.2), the Mishkinis height blend (depth 0.2), heights lifted to the
/// layer's p90.
fn base_old(p90: [f32; 2], xz: [f32; 2]) -> f32 {
    let w_b = (xz[0] + 0.5).clamp(0.0, 1.0);
    let w = [1.0 - w_b, w_b];
    let h = |k: u32| 0.5 + 0.4 * (bilinear_lattice(&|i, j| hash(i, j, 40 + k), xz, 0.5) - 0.5);
    let hl = [h(0), h(1)];
    let hw = [hl[0] + w[0], hl[1] + w[1]];
    let top = hw[0].max(hw[1]) - 0.2;
    let b = [(hw[0] - top).max(0.0), (hw[1] - top).max(0.0)];
    let sum = (b[0] + b[1]).max(1e-5);
    (b[0] * hl[0].max(p90[0]) + b[1] * hl[1].max(p90[1])) / sum
}

fn raise_old(a: f32, p90: [f32; 2], drift: &Chain, look: f32, xz: [f32; 2]) -> f32 {
    DEPTH * bury_old(a, cover_height(base_old(p90, xz), look, drift.drift_at(xz, 0.0)))
}

/// The steepest |dh/ds| of `h` along the profile `origin + s dir`, s in -len..len, at `step`.
fn steepest(h: impl Fn([f32; 2]) -> f32, origin: [f32; 2], dir: [f32; 2], len: f32, step: f32) -> f32 {
    let n = (2.0 * len / step) as i32;
    let at = |i: i32| {
        let s = -len + i as f32 * step;
        h([origin[0] + s * dir[0], origin[1] + s * dir[1]])
    };
    let mut last = at(0);
    let mut max: f32 = 0.0;
    for i in 1..=n {
        let v = at(i);
        max = max.max((v - last).abs() / step);
        last = v;
    }
    max
}

const DIAGONAL: [f32; 2] = [std::f32::consts::FRAC_1_SQRT_2, std::f32::consts::FRAC_1_SQRT_2];
const AMOUNTS: [f32; 7] = [0.3, 0.5, 0.7, 0.9, 0.95, 0.98, 1.0];

/// The width the defaults derive: wide enough for the bound, finite.
#[test]
fn filter_width_follows_the_repose_bound() {
    let w = filter_m();
    assert!((4.0..4.5).contains(&w), "{w}");
    // Steeper repose or a thinner cover: narrower; no cover: the minimum.
    assert!(raise_filter_m(DEPTH, 35f32.to_radians().tan(), MIN_FILTER_M, BASE_TEXEL_M) < w);
    assert!(raise_filter_m(0.2, tan_repose(), MIN_FILTER_M, BASE_TEXEL_M) < w);
    assert_eq!(raise_filter_m(0.0, tan_repose(), MIN_FILTER_M, BASE_TEXEL_M), MIN_FILTER_M);
    assert_eq!(raise_filter_m(0.01, 10.0, 2.0, BASE_TEXEL_M), 2.0);
    // Too deep for the base's own term: the drift tile (one constant texel).
    assert_eq!(raise_filter_m(5.0, tan_repose(), MIN_FILTER_M, BASE_TEXEL_M), COVER_DRIFT_TILE_M);
    // The level reads no steeper than range / w: at a power of two the mip itself.
    let t0 = COVER_DRIFT_TILE_M / NOISE_SIZE as f32;
    assert_eq!(prefilter_level(t0, t0), 0.0);
    assert_eq!(prefilter_level(4.0 * t0, t0), 2.0);
    assert!((prefilter_level(3.0 * t0, t0) - (1.0 + 2.0 / 3.0)).abs() < 1e-6);
    for i in 1..200u8 {
        let w = t0 * (1.0 + f32::from(i) * 0.1);
        let l = prefilter_level(w, t0);
        let (m, f) = (l.floor(), l.fract());
        let bound = (1.0 - f) / (t0 * m.exp2()) + f / (t0 * (m + 1.0).exp2());
        assert!((bound - 1.0 / w).abs() < 1e-4 / w, "w {w}: {bound} vs {}", 1.0 / w);
    }
}

/// The bury's slope in amount and top is at most `bury_gain` everywhere (finite differences).
#[test]
fn bury_gain_bounds_the_bury() {
    let g = bury_gain();
    let h = 1e-3;
    let mut max: f32 = 0.0;
    for i in 0..=200u16 {
        for j in 0..=200u16 {
            let (a, t) = (f32::from(i) / 200.0, f32::from(j) / 200.0);
            let da = (cover_bury((a + h).min(1.0), t) - cover_bury(a, t)).abs() / h;
            let dt = (cover_bury(a, (t + h).min(1.0)) - cover_bury(a, t)).abs() / h;
            max = max.max(da).max(dt);
        }
    }
    assert!(max <= g * 1.01, "{max} > {g}");
}

/// The bound: across layer boundaries (any two p90s), along and across drift borders (the real
/// atlas, along X, Z and the diagonal), at every amount (to 1) and tops past `COVER_TOP_MAX`, the
/// raise never stands steeper than the repose angle.
#[test]
fn raise_never_steeper_than_repose_on_real_tops() {
    let drift = Chain::drift();
    let limit = tan_repose() + EPS;
    let pairs = [(0.6, 0.85), (0.0, 1.0), (1.0, 0.0), (0.85, 0.95), (0.95, 1.0)];
    for (pa, pb) in pairs {
        let p90 = |i: i32, _j: i32| if i < 0 { pa } else { pb };
        for a in AMOUNTS {
            for (k, dir) in [[1.0, 0.0], DIAGONAL].into_iter().enumerate() {
                for row in 0..4 {
                    let origin = [0.0, 3.7 * row as f32 + 0.3 * k as f32];
                    let s = steepest(|p| raise_new(a, &p90, &drift, 0.5, p), origin, dir, 16.0, STEP);
                    assert!(s <= limit, "p90 {pa}/{pb}, amount {a}, dir {dir:?}, row {row}: {s} > {limit}");
                }
            }
        }
    }
    // Drift borders over the whole tile, on a stony layer (tops up to 0.92) and a flat one.
    for p in [0.0, 0.85, 1.0] {
        let p90 = |_i: i32, _j: i32| p;
        for a in AMOUNTS {
            for dir in [[1.0, 0.0], [0.0, 1.0], DIAGONAL] {
                let s = steepest(|q| raise_new(a, &p90, &drift, 0.5, q), [32.0, 32.0], dir, 32.0, STEP);
                assert!(s <= limit, "p90 {p}, amount {a}, dir {dir:?}: {s} > {limit}");
            }
        }
    }
}

/// The bound holds for any tops: random p90 per base texel and a random drift texture (box mips), along
/// three directions, every amount.
#[test]
fn raise_never_steeper_than_repose_on_any_tops() {
    let limit = tan_repose() + EPS;
    for seed in 0..3u32 {
        let drift = Chain::random(100 + seed);
        let p90 = |i: i32, j: i32| hash(i, j, 200 + seed);
        for look in [0.0, 0.5, 1.0] {
            for a in AMOUNTS {
                for dir in [[1.0, 0.0], [0.0, 1.0], DIAGONAL, [0.6, -0.8]] {
                    let s = steepest(|q| raise_new(a, &p90, &drift, look, q), [10.0, 20.0], dir, 24.0, STEP);
                    assert!(s <= limit, "seed {seed}, look {look}, amount {a}, dir {dir:?}: {s} > {limit}");
                }
            }
        }
    }
}

/// An amount edge as smooth as the 32 m cells (a climate border) stays under the bound too; the
/// shelter's 0.75 m tent (roof drip lines) is not filtered (measured below).
#[test]
fn cell_amount_edges_stay_under_repose() {
    let limit = tan_repose() + EPS;
    let drift = Chain::drift();
    for p in [0.0, 0.85] {
        let p90 = |_i: i32, _j: i32| p;
        let amount = |x: f32| (x / 32.0 + 0.5).clamp(0.0, 1.0);
        let s = steepest(|q| raise_new(amount(q[0]), &p90, &drift, 0.5, q), [0.0, 5.0], [1.0, 0.0], 24.0, STEP);
        assert!(s <= limit, "p90 {p}: {s}");
    }
}

/// The 2026-11-15 intent, kept: a thin cover over paving (snow 0.4 over cobble tops, p90 0.85) stays
/// at ground level for any drift; past the relief it rises monotonically; amount 1 raises the full
/// depth over open ground (p90 up to 0.7, the look's mean and drift mid) and over flat ground (p90 0)
/// at any drift; the bury is exactly 0 at and below the top.
#[test]
fn bury_intent_kept() {
    for i in 0..=20u8 {
        let d = f32::from(i) / 20.0;
        assert_eq!(cover_bury(0.4, cover_height(0.85, 0.5, d)), 0.0, "paving, drift {d}");
        assert_eq!(cover_bury(1.0, cover_height(0.0, 0.5, d)), 1.0, "flat, drift {d}");
    }
    for i in 0..=14u8 {
        let p = f32::from(i) / 20.0;
        assert!(cover_bury(1.0, cover_height(p, 0.5, 0.5)) > 1.0 - 1e-5, "p90 {p}");
    }
    for t in [0.0, 0.3, 0.6, 0.8, 1.0] {
        assert_eq!(cover_bury(t, t), 0.0);
        assert_eq!(cover_bury(t - 0.1, t), 0.0);
        let mut last = 0.0;
        for i in 0..=40u8 {
            let b = cover_bury(f32::from(i) / 40.0, t);
            assert!(b >= last, "top {t}, step {i}");
            last = b;
        }
        let full = (1.0 - t) / (1.0 - t.min(COVER_TOP_MAX));
        assert!((last - full * full * (3.0 - 2.0 * full)).abs() < 1e-6, "top {t}: {last}");
    }
    assert_eq!(cover_bury(1.0, COVER_TOP_MAX), 1.0);
}

/// Where the raise is full, prints and lips combine with the depth exactly as before (W6b).
#[test]
fn trail_combination_unchanged_where_full() {
    let ring = DEPTH;
    for v in [[0.0, 0.0], [-0.2, 0.0], [-DEPTH, 0.0], [0.0, LIP_HEIGHT * ring], [-0.1, 0.05]] {
        for t in [0.0, 0.3, COVER_TOP_MAX] {
            let r = cover_raise(1.0, t, DEPTH, v, ring);
            assert_eq!(r, (DEPTH + trail_height(v, DEPTH, ring)).max(0.0), "{v:?} top {t}");
        }
    }
}

/// The level-0 atlas is unchanged by the mip chain, and each level lies within the range of the one
/// above (box average).
#[test]
fn noise_mips_extend_the_atlas() {
    let atlas = noise_atlas();
    let mips = noise_atlas_mips();
    let n = NOISE_SIZE as usize;
    assert_eq!(&mips[..n * n], atlas.as_slice());
    let total: usize = (0..=n.trailing_zeros()).map(|l| (n >> l) * (n >> l)).sum();
    assert_eq!(mips.len(), total);
}

/// SW-6 measurement: the steepest raise slope (rise over run, and degrees) old vs new at each test
/// edge: a layer boundary (p90 0.6 | 0.85), a drift border on a stony layer, both at amount 0.95 and
/// 0.98, and the old field sampled at LOD 0's 0.5 m vertices; a roof drip line (exposure through the
/// shelter's 0.75 m tent) for the record.
#[test]
#[ignore = "measurement: SW-6 raise slope, old vs new"]
fn measure_raise_slopes() {
    let drift = Chain::drift();
    let deg = |s: f32| s.atan().to_degrees();
    println!("filter width {:.2} m (repose {REPOSE_DEG} deg, depth {DEPTH} m), drift level {:.2}", filter_m(), drift_level(filter_m()));
    for a in [0.9, 0.95, 0.98, 1.0] {
        let p90n = |i: i32, _j: i32| if i < 0 { 0.6 } else { 0.85 };
        let old = steepest(|p| raise_old(a, [0.6, 0.85], &drift, 0.5, p), [0.0, 1.3], [1.0, 0.0], 8.0, STEP);
        let old_v = steepest(|p| raise_old(a, [0.6, 0.85], &drift, 0.5, p), [0.0, 1.0], [1.0, 0.0], 8.0, 0.5);
        let new = steepest(|p| raise_new(a, &p90n, &drift, 0.5, p), [0.0, 1.3], [1.0, 0.0], 8.0, STEP);
        println!(
            "layer edge  a {a:.2}: old {old:.2} ({:.0} deg; {old_v:.2} on 0.5 m vertices)  new {new:.3} ({:.1} deg)",
            deg(old),
            deg(new)
        );
        let flat = |_i: i32, _j: i32| 0.85;
        let old = steepest(|p| raise_old(a, [0.85, 0.85], &drift, 0.5, p), [32.0, 32.0], [1.0, 0.0], 32.0, STEP);
        let new = steepest(|p| raise_new(a, &flat, &drift, 0.5, p), [32.0, 32.0], [1.0, 0.0], 32.0, STEP);
        println!("drift edge  a {a:.2}: old {old:.2} ({:.0} deg)  new {new:.3} ({:.1} deg)", deg(old), deg(new));
    }
    // The old field's steepest over a sweep of amounts and layer p90s (drift border, whole tile),
    // with the look's mean and with a look height of 0.5 m texels (0.2..0.8, a model: the real
    // look is pack data), along X at 0.05 m.
    let look_texels = |p: [f32; 2]| 0.5 + 0.6 * (bilinear_lattice(&|i, j| hash(i, j, 77), p, 0.5) - 0.5);
    for with_look in [false, true] {
        let mut worst = (0.0f32, 0.0f32, 0.0f32);
        for ai in 3..=20u8 {
            let a = f32::from(ai) / 20.0;
            for pi in 0..=10u8 {
                let p = f32::from(pi) / 10.0;
                let h = |q: [f32; 2]| {
                    let look = if with_look { look_texels(q) } else { 0.5 };
                    DEPTH * bury_old(a, cover_height(p, look, drift.drift_at(q, 0.0)))
                };
                let s = steepest(h, [32.0, 32.0], [1.0, 0.0], 32.0, STEP);
                if s > worst.0 {
                    worst = (s, a, p);
                }
            }
        }
        let (s, a, p) = worst;
        let new = steepest(|q| raise_new(a, &|_i, _j| p, &drift, 0.5, q), [32.0, 32.0], [1.0, 0.0], 32.0, STEP);
        println!("old worst (look texels {with_look}): {s:.2} ({:.0} deg) at amount {a:.2}, p90 {p:.1}; new there {new:.3}", deg(s));
    }
    // The old bury's gain alone: a 0.01 top step at amount 0.98 next to top 0.97.
    println!("old bury jump for top 0.97 -> 0.96 at amount 0.98: {:.2} of the depth", bury_old(0.98, 0.96) - bury_old(0.98, 0.97));
    // Roof drip line: exposure 1 -> 0 through a 0.75 m tent (amount = exposure), open ground.
    let tent = |x: f32| {
        let u = (x / 0.375).clamp(-1.0, 1.0);
        let v = 1.0 - u.abs();
        if u < 0.0 { 1.0 - v * v / 2.0 } else { v * v / 2.0 }
    };
    let flat = |_i: i32, _j: i32| 0.6;
    let old = steepest(|p| raise_old(tent(p[0]), [0.6, 0.6], &drift, 0.5, p), [0.0, 2.0], [1.0, 0.0], 2.0, STEP);
    let new = steepest(|p| raise_new(tent(p[0]), &flat, &drift, 0.5, p), [0.0, 2.0], [1.0, 0.0], 2.0, STEP);
    println!("drip line (shelter tent, not filtered): old {old:.2} ({:.0} deg)  new {new:.2} ({:.0} deg)", deg(old), deg(new));
}
