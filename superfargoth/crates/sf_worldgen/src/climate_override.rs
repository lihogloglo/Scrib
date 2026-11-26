//! Per-region climate override (pipeline stage 6, `docs/specs/worldgen.md` "Climate"): after the precipitation
//! sweep, a world's recipe may scale, add or pin temperature and precipitation per region (the painted-ecotope
//! equivalent), blended across region borders so climate classes do not cut in straight lines. The result is not fed
//! back into the sweep.
//!
//! The blend is a box filter of the region masks: the weight of region `k` at a sample is the share of its
//! `(2r+1) x (2r+1)` window (clipped to the grid) labelled `k`, counted as an integer with separable sliding sums
//! over the region's bounding box grown by `r`, so the cost is O(samples) per region whatever the border width.
//! Weights are integer counts over the window's sample count, so a full window reproduces the region's adjustment
//! bit for bit and an empty one leaves the input untouched. Rows are mapped in parallel with no cross-thread
//! reduction, so the output is the same on any thread count.

use rayon::prelude::*;

use crate::grid::Grid;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_climate_override.rs"]
mod tests;

/// `x -> pin`, or `x * scale + add` when there is no pin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Adjust {
    pub scale: f64,
    pub add: f64,
    pub pin: Option<f64>,
}

impl Default for Adjust {
    fn default() -> Self {
        // tunables: const (the identity adjustment: scale 1, add 0, no pin leaves every value as it is)
        Self { scale: 1.0, add: 0.0, pin: None }
    }
}

impl Adjust {
    /// The adjusted value of `x`.
    #[must_use]
    pub fn apply(&self, x: f64) -> f64 {
        match self.pin {
            Some(p) => p,
            None => x * self.scale + self.add,
        }
    }

    /// Whether this adjustment leaves every value as it is (scale 1, add 0, no pin).
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.pin.is_none() && self.scale.to_bits() == 1.0f64.to_bits() && self.add == 0.0
    }

    fn is_finite(&self) -> bool {
        self.scale.is_finite() && self.add.is_finite() && self.pin.is_none_or(f64::is_finite)
    }
}

/// One region's override: `region` is the legend index of the region grid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegionOverride {
    pub region: u8,
    pub temperature: Adjust,
    pub rain: Adjust,
}

/// Window counts of one region over its bounding box grown by the box radius (inclusive bounds).
struct RegionCounts {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    /// Row-major over `(x1 - x0 + 1) x (y1 - y0 + 1)`.
    counts: Vec<u32>,
}

impl RegionCounts {
    fn get(&self, x: usize, y: usize) -> u32 {
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return 0;
        }
        self.counts[(y - self.y0) * (self.x1 - self.x0 + 1) + (x - self.x0)]
    }
}

/// Temperature (C) and rain (mm/yr) with every listed region's override applied, blended across region borders
/// with a box of `blend_m` metres. `labels` = region legend index per sample (row-major, as the grids).
///
/// # Errors
/// [`Error::Invalid`] when the grids differ in size or spacing, the spacing is not a positive finite number,
/// `labels` is not one per sample, a region is listed twice, `blend_m` is negative or not finite, or an
/// adjustment value is not finite.
pub fn apply_overrides(
    temperature: &Grid,
    rain: &Grid,
    labels: &[u8],
    overrides: &[RegionOverride],
    blend_m: f64,
) -> Result<(Grid, Grid)> {
    validate(temperature, rain, labels, overrides, blend_m)?;
    let (w, h) = (temperature.w, temperature.h);
    if overrides.is_empty() || w * h == 0 {
        return Ok((temperature.clone(), rain.clone()));
    }
    let r = box_radius(blend_m, temperature.spacing, w.max(h));
    let regions: Vec<Option<RegionCounts>> =
        overrides.iter().map(|o| region_counts(labels, w, h, o.region, r)).collect();

    // An identity adjustment is a no-op on its channel (its term merges into the input's), so each channel blends
    // only the regions that change it: the other channel stays bit-identical where it is not overridden.
    let t_regions: Vec<(Adjust, &RegionCounts)> = overrides
        .iter()
        .zip(&regions)
        .filter_map(|(o, rc)| rc.as_ref().filter(|_| !o.temperature.is_identity()).map(|rc| (o.temperature, rc)))
        .collect();
    let p_regions: Vec<(Adjust, &RegionCounts)> = overrides
        .iter()
        .zip(&regions)
        .filter_map(|(o, rc)| rc.as_ref().filter(|_| !o.rain.is_identity()).map(|rc| (o.rain, rc)))
        .collect();

    let blend = |data: &[f64], active: &[(Adjust, &RegionCounts)], floor: Option<f64>| -> Vec<f64> {
        let mut out = data.to_vec();
        if active.is_empty() {
            return out;
        }
        out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let ny = window_len(y, r, h);
            for (x, v) in row.iter_mut().enumerate() {
                let sum_c: u32 = active.iter().map(|(_, rc)| rc.get(x, y)).sum();
                if sum_c == 0 {
                    continue;
                }
                let n = window_len(x, r, w) * ny;
                let n_f = f64::from(n);
                let input = *v;
                let mut acc = f64::from(n - sum_c) / n_f * input;
                for (adjust, rc) in active {
                    let c = rc.get(x, y);
                    if c != 0 {
                        acc += f64::from(c) / n_f * adjust.apply(input);
                    }
                }
                *v = floor.map_or(acc, |f| acc.max(f));
            }
        });
        out
    };

    let t_out = blend(&temperature.data, &t_regions, None);
    // Rain is a depth per year: an add that overshoots stops at zero.
    let p_out = blend(&rain.data, &p_regions, Some(0.0));
    Ok((
        Grid { w, h, spacing: temperature.spacing, data: t_out },
        Grid { w, h, spacing: rain.spacing, data: p_out },
    ))
}

fn validate(
    temperature: &Grid,
    rain: &Grid,
    labels: &[u8],
    overrides: &[RegionOverride],
    blend_m: f64,
) -> Result<()> {
    let invalid = |m: String| Err(Error::Invalid(format!("climate override: {m}")));
    if temperature.w != rain.w || temperature.h != rain.h {
        return invalid(format!(
            "temperature is {}x{} but rain is {}x{}",
            temperature.w, temperature.h, rain.w, rain.h
        ));
    }
    if temperature.spacing.to_bits() != rain.spacing.to_bits() {
        return invalid(format!(
            "temperature spacing {} m differs from rain spacing {} m",
            temperature.spacing, rain.spacing
        ));
    }
    if !(temperature.spacing.is_finite() && temperature.spacing > 0.0) {
        return invalid(format!("spacing {} m is not a positive finite number", temperature.spacing));
    }
    let samples = temperature.w * temperature.h;
    if temperature.data.len() != samples || rain.data.len() != samples {
        return invalid(format!("grid data is not {samples} samples"));
    }
    if labels.len() != samples {
        return invalid(format!("{} labels for {samples} samples", labels.len()));
    }
    if !(blend_m.is_finite() && blend_m >= 0.0) {
        return invalid(format!("blend width {blend_m} m is negative or not finite"));
    }
    let mut seen = [false; 256];
    for o in overrides {
        if std::mem::replace(&mut seen[usize::from(o.region)], true) {
            return invalid(format!("region {} is listed twice", o.region));
        }
        if !o.temperature.is_finite() || !o.rain.is_finite() {
            return invalid(format!("region {} has a non-finite scale, add or pin", o.region));
        }
    }
    Ok(())
}

/// Box radius in samples, `round(blend_m / (2 * spacing))`, capped at `max_r` (a wider box covers the grid anyway).
fn box_radius(blend_m: f64, spacing: f64, max_r: usize) -> usize {
    let r = (blend_m / (2.0 * spacing)).round();
    #[allow(clippy::cast_precision_loss)]
    let cap = max_r as f64;
    if r >= cap {
        max_r
    } else {
        // `r` is a non-negative integer below `max_r`.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let r = r as usize;
        r
    }
}

/// Number of samples of `[i - r, i + r]` inside `[0, len)`.
fn window_len(i: usize, r: usize, len: usize) -> u32 {
    let lo = i.saturating_sub(r);
    let hi = (i + r).min(len - 1);
    u32::try_from(hi - lo + 1).expect("window side fits in u32")
}

/// Window counts of `region` over its bounding box grown by `r`, or `None` when no sample carries it.
fn region_counts(labels: &[u8], w: usize, h: usize, region: u8, r: usize) -> Option<RegionCounts> {
    let (mut bx0, mut bx1, mut by0, mut by1) = (usize::MAX, 0, usize::MAX, 0);
    for (i, _) in labels.iter().enumerate().filter(|&(_, &l)| l == region) {
        let (x, y) = (i % w, i / w);
        bx0 = bx0.min(x);
        bx1 = bx1.max(x);
        by0 = by0.min(y);
        by1 = by1.max(y);
    }
    if bx0 == usize::MAX {
        return None;
    }
    let x0 = bx0.saturating_sub(r);
    let x1 = (bx1 + r).min(w - 1);
    let y0 = by0.saturating_sub(r);
    let y1 = (by1 + r).min(h - 1);
    let bw = x1 - x0 + 1;
    let bh = y1 - y0 + 1;

    // Horizontal sliding sums of the region mask, per row of the grown box (samples outside it are never `region`).
    let mut horiz = vec![0u32; bw * bh];
    horiz.par_chunks_mut(bw).enumerate().for_each(|(j, row)| {
        let src = &labels[(y0 + j) * w + x0..=(y0 + j) * w + x1];
        let mut prefix = vec![0u32; bw + 1];
        for (i, &l) in src.iter().enumerate() {
            prefix[i + 1] = prefix[i] + u32::from(l == region);
        }
        for (i, out) in row.iter_mut().enumerate() {
            let lo = i.saturating_sub(r);
            let hi = (i + r).min(bw - 1);
            *out = prefix[hi + 1] - prefix[lo];
        }
    });

    // Vertical sliding sums of the horizontal sums, through a column prefix over the box's rows.
    let mut prefix = vec![0u32; bw * (bh + 1)];
    for j in 0..bh {
        for i in 0..bw {
            prefix[(j + 1) * bw + i] = prefix[j * bw + i] + horiz[j * bw + i];
        }
    }
    let mut counts = vec![0u32; bw * bh];
    counts.par_chunks_mut(bw).enumerate().for_each(|(j, row)| {
        let lo = j.saturating_sub(r);
        let hi = (j + r).min(bh - 1);
        for (i, out) in row.iter_mut().enumerate() {
            *out = prefix[(hi + 1) * bw + i] - prefix[lo * bw + i];
        }
    });

    Some(RegionCounts { x0, x1, y0, y1, counts })
}
