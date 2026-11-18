//! **Layout stage** (WG-S4): turns a source height field into low-frequency guides only, so the source surface
//! never feeds the final heights directly.
//!
//! Why: WG-S2/S2b took the authored source (a hand-sculpted grid) scaled ×S as the terrain and eroded it; every
//! source vertex became a spike and the erosion only cut grooves into the finished blobs (ledger
//! `docs/ledger/worldgen.md`, S2/S2b). In landscape-evolution terrain generation the uplift map only says *where*
//! land rises; the relief itself emerges from uplift against fluvial erosion over a long time (Cordonnier, Braun,
//! Cani, Benes, Galin, Peytavie, Guérin, "Large Scale Terrain Generation from Tectonic Uplift and Fluvial
//! Erosion", CGF 35(2), 2016; Schott, Paris, Fournier, Guérin, Galin, "Large-scale terrain authoring through
//! interactive erosion simulation", ACM TOG 42(5), 2023). So the layout keeps three things:
//!
//! - the **land/sea mask** of the source, exactly (the coast is kept, worldgen.md "Stage rules");
//! - a **heavily low-passed relief**: the source height above sea (0 on sea), Gaussian-filtered with a cutoff
//!   given as a wavelength in world metres (separable Gaussian, edge-clamped, normalised weights);
//! - a low-passed **sea floor** (the sea never moves later; it only needs to be smooth).
//!
//! The region map is the third guide; it is sampled by the runner ([`crate::landform`]).
//!
//! Gaussian cutoff: the amplitude response of a Gaussian of standard deviation σ is `exp(-2π²σ²/λ²)`; it is 1/2
//! at wavelength `λc` for `σ = λc √(ln 2 / 2) / π ≈ 0.187 λc` ([`sigma_for_cutoff`]). The kernel is truncated at
//! 3σ (99.7 % of the mass) and renormalised, so a constant stays constant.
//!
//! Deterministic: both passes are pure per-row maps (parallel rows, fixed summation order per sample).

use rayon::prelude::*;

use crate::Grid;

/// Kernel truncation in standard deviations.
// tunables: const (maths: 3σ holds 99.7 % of a Gaussian)
const TRUNCATE_SIGMAS: f64 = 3.0;

/// Gaussian σ, metres, whose amplitude response is 1/2 at wavelength `cutoff_m`.
pub fn sigma_for_cutoff(cutoff_m: f64) -> f64 {
    cutoff_m * (std::f64::consts::LN_2 / 2.0).sqrt() / std::f64::consts::PI
}

/// Normalised 1D Gaussian taps for σ in samples, centre tap at index `radius`.
fn taps(sigma_samples: f64) -> Vec<f64> {
    let radius = (TRUNCATE_SIGMAS * sigma_samples).ceil().max(0.0) as usize;
    let mut k: Vec<f64> = (0..=2 * radius)
        .map(|i| {
            let d = i as f64 - radius as f64;
            (-0.5 * d * d / (sigma_samples * sigma_samples)).exp()
        })
        .collect();
    let sum: f64 = k.iter().sum();
    for v in &mut k {
        *v /= sum;
    }
    k
}

/// Separable Gaussian low-pass of `g` with standard deviation `sigma_m` metres, edges clamped. `sigma_m ≤ 0`
/// returns a copy.
pub fn gaussian(g: &Grid, sigma_m: f64) -> Grid {
    if sigma_m <= 0.0 || g.is_empty() {
        return g.clone();
    }
    let k = taps(sigma_m / g.spacing);
    let r = (k.len() / 2) as isize;
    let (w, h) = (g.w, g.h);
    let mut tmp = vec![0.0; g.len()];
    tmp.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let src = &g.data[y * w..(y + 1) * w];
        for (x, out) in row.iter_mut().enumerate() {
            let mut acc = 0.0;
            for (t, kt) in k.iter().enumerate() {
                let xx = (x as isize + t as isize - r).clamp(0, w as isize - 1) as usize;
                acc += kt * src[xx];
            }
            *out = acc;
        }
    });
    let mut data = vec![0.0; g.len()];
    data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (t, kt) in k.iter().enumerate() {
            let yy = (y as isize + t as isize - r).clamp(0, h as isize - 1) as usize;
            let src = &tmp[yy * w..(yy + 1) * w];
            for (out, &v) in row.iter_mut().zip(src) {
                *out += kt * v;
            }
        }
    });
    Grid {
        w,
        h,
        spacing: g.spacing,
        data,
    }
}

/// The low-frequency guides of one source field on one grid.
#[derive(Debug, Clone)]
pub struct Layout {
    /// Land (source at or above sea level), exactly the source's class.
    pub land: Vec<bool>,
    /// Low-passed height above sea, metres in the source's own vertical units (≥ 0; 0 on sea).
    pub relief: Grid,
    /// Low-passed sea floor, strictly below sea level on sea samples (sea level on land, unused there).
    pub sea_floor: Vec<f64>,
}

/// Builds the layout of `src` (already sampled on the target grid): mask, low-passed relief, low-passed sea floor.
pub fn layout(src: &Grid, sea_level: f64, sigma_m: f64) -> Layout {
    let land: Vec<bool> = src.data.iter().map(|&v| v >= sea_level).collect();
    let above = Grid {
        data: src
            .data
            .iter()
            .zip(&land)
            .map(|(&v, &l)| if l { v - sea_level } else { 0.0 })
            .collect(),
        ..src.clone()
    };
    let below = Grid {
        data: src
            .data
            .iter()
            .zip(&land)
            .map(|(&v, &l)| if l { 0.0 } else { v - sea_level })
            .collect(),
        ..src.clone()
    };
    let mut relief = gaussian(&above, sigma_m);
    for (v, &l) in relief.data.iter_mut().zip(&land) {
        *v = if l { v.max(0.0) } else { 0.0 };
    }
    let floor = gaussian(&below, sigma_m);
    let below_sea = sea_level.next_down();
    let sea_floor = floor
        .data
        .iter()
        .zip(&land)
        .map(|(&v, &l)| {
            if l {
                sea_level
            } else {
                (sea_level + v).min(below_sea)
            }
        })
        .collect();
    Layout {
        land,
        relief,
        sea_floor,
    }
}
