//! Terrain **statistics** for a real-terrain baseline (WG-M; printed numbers, never pass/fail): the same land-only
//! measurements on any guide folder, real DEM or generated field, so generated fields can be compared with real
//! places at one spacing over comparable land.
//!
//! **Sampling.** Every field is measured at one spacing (`spacing_m`, 20 m by default). A field at another spacing
//! is resampled first: an integer coarsening factor `k` by `k × k` block means (class by majority, height = mean of
//! that class), anything else by Catmull-Rom ([`crate::resample`]). An optional region (guide-frame metres) crops
//! it first.
//!
//! **Land mask.** With `mask_sea`, samples below the guide's sea level are sea and are never measured; land within
//! `coast_m` of the sea (exact Euclidean distance transform, Felzenszwalb & Huttenlocher, "Distance transforms of
//! sampled functions", Theory of Computing 8, 2012) and within `edge_m` of the grid border is excluded too (cliffs,
//! beaches, truncated windows). What is left is a field's **valid land**. Zones split it further:
//!
//! - `all`: every valid sample.
//! - `cone` / `lowland` (with a summit): valid samples within `cone_m` of the summit / at least `lowland_m` from it
//!   (the ring between is in neither). The summit is a guide-frame point or, `auto`, the highest valid sample.
//! - `band` (with an elevation band): valid samples with `lo ≤ h ≤ hi`.
//!
//! **Metrics per zone:**
//!
//! 1. Hypsometry: height percentiles and the hypsometric integral `(mean − min)/(max − min)` (Strahler,
//!    "Hypsometric (area-altitude) analysis of erosional topography", GSA Bulletin 63, 1952; Pike & Wilson 1971
//!    show it equals the elevation-relief ratio). Slope from Horn's 3 × 3 gradient (Horn, "Hill shading and the
//!    reflectance map", Proc. IEEE 69, 1981): percentiles and the share above 30° and 45°.
//! 2. Multi-scale roughness, two independent estimates per wavelength band `[lo, hi)` (edges [`BAND_EDGES_M`]):
//!    - **`DoG` RMS:** RMS over the zone of `LP(lo) − LP(hi)`, where `LP(L)` is a Gaussian low-pass whose transfer is
//!      one half at wavelength `L` (`σ = L·√(ln 2 / 2)/π`). The low-pass is a Gaussian-weighted local *linear* fit
//!      over land only (first-order normalized convolution, Knutsson & Westin, CVPR 1993), so planes pass
//!      exactly, sea and the grid border bias nothing, and a plane has zero roughness. Gaussians below 4 samples
//!      are exact kernels, wider ones three box passes (Kovesi, "Fast almost-Gaussian filtering", DICTA 2010).
//!    - **PSD RMS:** the periodogram of square land patches (`patch` samples, half overlap, every sample valid
//!      land, assigned to the zone holding the patch centre), each detrended by a least-squares plane and
//!      Hann-windowed, normalized so its sum is the variance (Perron, Kirchner, Dietrich, "Spectral signatures of
//!      characteristic spatial scales and nonfractal structure in landscapes", JGR 113, F04003, 2008). Band
//!      power = the sum over the band's frequencies, mean over patches; reported as an RMS (bands up to patch/2).
//!    - The radially binned spectral density (10 bins per decade, mean over patches) gives the **spectral slope**
//!      `β` (density ∝ f^−β) over [`SLOPE_RANGES_M`].
//! 3. Valley spacing: the **dominant ridge-valley wavelength** is the peak of the binned spectrum normalized by
//!    its power-law fit (Perron et al. 2008 §3), fitted and searched over `[5 spacings, patch/4]` (longer wavelengths
//!    hold the edifice curvature a plane detrend leaves). **Drainage density** at
//!    fixed area thresholds (Horton 1945): D8 channel length on valid samples per valid km², the network routed
//!    over the whole field (Priority-Flood+ε, [`crate::stream::route`]) so upstream area is never cut by a zone.
//! 4. The channel metrics of [`crate::structure`] (orientation coherence, its drainage density, junction angles)
//!    and [`crate::metrics`] (main-stem sinuosity), called on a copy of the field whose non-valid samples are set
//!    below sea level (so a zone's channels drain to its edge; a lowland zone loses area from outside it). Local
//!    relief: max − min of land heights in square windows (edges [`crate::measure::MeasureParams::relief_windows_m`]).
//! 5. **Walkability** ([`Walkability`]): slope by central differences at the grid spacing on dry interior samples
//!    (sea = below the guide's sea level, excluded whatever `mask_sea` says), the share of the zone's dry land per
//!    slope band ([`WALK_BAND_EDGES_DEG`]), the walkable share (slope ≤ `max_walk_deg`, [`declared_walk`]), and the
//!    4-connected walkable components inside the zone (count, largest as a share of the zone's dry land).
//! 6. **Landforms table** (LF-0, `landforms.md` "Measurement", [`land_rows`]): the walkability of zone `all` and of
//!    each region of it (the guide's `regions.png`, [`region_labels`]; components are cut at region borders), its
//!    land shares under [`FLAT_DEG`] and at or above [`STEEP_DEG`], and with a base field on the same samples the
//!    untouched share (`|dh|` < [`UNTOUCHED_M`]) and the `|dh|` p50 / p95.
//!
//! Deterministic: `rayon` maps only pure per-row and per-patch functions, collected in order.

use std::fmt::Write as _;
use std::path::Path;

use rayon::prelude::*;

use crate::guide::{Guide, RegionMap};
use crate::metrics::channel_metrics;
use crate::resample::resample;
use crate::stream::route;
use crate::filter::Declared;
use crate::structure::{Structure, structure};
use crate::{Error, Grid, Result};

/// Wavelength band edges, metres; a field reports the bands whose low edge is at least two spacings (Nyquist) and
/// whose high edge is at most half its shorter side.
// tunables: const (metric definition: band edges fixed by the WG-M brief, 40 m to 10 km plus 10-40 m for 5 m fields)
pub const BAND_EDGES_M: [f64; 8] = [10.0, 20.0, 40.0, 100.0, 300.0, 1000.0, 3000.0, 10000.0];

/// Wavelength ranges of the spectral-slope fits, metres (each needs ≥ 3 bins inside the patch's range).
// tunables: const (metric definition: fine (5 m fields) / fine / mid / coarse slope ranges of the WG-M report)
pub const SLOPE_RANGES_M: [[f64; 2]; 4] = [[10.0, 40.0], [40.0, 100.0], [100.0, 1000.0], [1000.0, 5000.0]];

/// Log bins of the radial spectrum per decade of wavelength.
// tunables: const (metric definition: 10 log bins per decade)
pub const BINS_PER_DECADE: f64 = 10.0;

/// Height percentiles of the hypsometry, fractions.
// tunables: const (metric definition: reported percentiles)
pub const HEIGHT_QUANTILES: [f64; 7] = [0.01, 0.05, 0.25, 0.5, 0.75, 0.95, 0.99];

/// Upper edges of the walkability slope bands, degrees (bands `[0, e0)`, `[e0, e1)`, ..., `[e_last, 90]`).
// tunables: const (metric definition: report bands 0-15 / 15-25 / 25-35 / 35-46 / 46-60 / > 60 deg)
pub const WALK_BAND_EDGES_DEG: [f64; 5] = [15.0, 25.0, 35.0, 46.0, 60.0];

/// Land under this slope counts as flat ([`Walkability::flat`]), degrees.
// tunables: const (metric definition: landforms.md "Measurement", land share under 10 deg, Etna 52 %)
pub const FLAT_DEG: f64 = 10.0;

/// Land at or above this slope counts as steep ([`Walkability::steep`]), degrees.
// tunables: const (metric definition: landforms.md "Measurement", land share >= 35 deg, Etna 1.8 %)
pub const STEEP_DEG: f64 = 35.0;

/// A sample whose height moved less than this from the base field counts as untouched, metres.
// tunables: const (metric definition: landforms.md "Measurement", untouched = |dh| < 0.5 m vs the base)
pub const UNTOUCHED_M: f64 = 0.5;

/// Declared walkability values (`scrib-worldgen measure --max-walk-deg`).
pub fn declared_walk() -> Vec<Declared> {
    vec![Declared {
        name: "max_walk_deg",
        v: 46.0,
        unit: "deg",
        min: 20.0,
        max: 80.0,
        source: "Reference(sf_controller MoverParams::max_slope_deg default 46)".into(),
        help: "steepest slope a character walks up; steeper land counts as unwalkable",
    }]
}

/// Gaussians narrower than this many samples are exact kernels, wider ones three boxes.
// tunables: const (filter construction: exact kernel below 4 samples, Kovesi boxes above)
const EXACT_BELOW: f64 = 4.0;

/// A Gaussian under this many samples is the identity (the band's low edge is the Nyquist edge).
// tunables: const (filter construction: a Gaussian under half a sample is the identity)
const IDENTITY_BELOW: f64 = 0.5;

/// Where the summit of the cone / lowland split is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Summit {
    /// The highest valid sample (ties: lowest index).
    Auto,
    /// A guide-frame point, metres (x east, y north).
    At([f64; 2]),
}

/// The measurement protocol (every value is a metric definition; the CLI's defaults are the WG-M report's).
#[derive(Debug, Clone)]
pub struct MeasureParams {
    /// Spacing every field is measured at, metres.
    pub spacing_m: f64,
    /// Exclude sea (below the guide's sea level) and the coastal margin.
    pub mask_sea: bool,
    /// Land within this distance of the sea is excluded, metres (with `mask_sea`).
    pub coast_m: f64,
    /// Samples within this distance of the grid border are excluded, metres.
    pub edge_m: f64,
    /// Crop `[x0, y0, x1, y1]` in guide-frame metres before anything else.
    pub region_m: Option<[f64; 4]>,
    /// Cone / lowland split around a summit.
    pub summit: Option<Summit>,
    /// Cone zone: within this distance of the summit, metres.
    pub cone_m: f64,
    /// Lowland zone: at least this far from the summit, metres.
    pub lowland_m: f64,
    /// Elevation band zone `[lo, hi]`, metres.
    pub band_m: Option<[f64; 2]>,
    /// Channel threshold of the structure metrics, m².
    pub channel_m2: f64,
    /// Coherence window edge, metres.
    pub window_m: f64,
    /// Channel direction steps of the structure metrics.
    pub dir_steps: usize,
    /// Drainage-density thresholds, m².
    pub dd_thresholds_m2: Vec<f64>,
    /// Local-relief window edges, metres.
    pub relief_windows_m: Vec<f64>,
    /// Spectral patch edge, samples (a power of two).
    pub patch: usize,
    /// Walkable slope limit, degrees ([`declared_walk`]).
    pub max_walk_deg: f64,
}

impl MeasureParams {
    /// The WG-M report's protocol at `spacing_m`: sea masked, 500 m coast and 1 km edge margins, structure at
    /// 0.1 km² / 1 km window / 5 steps (as the S2b-S5 recipes), drainage density at 0.1 / 1 / 10 km², relief in
    /// 1 km and 5 km windows, 512-sample patches, no zones.
    pub fn standard(spacing_m: f64) -> Self {
        Self {
            spacing_m,
            mask_sea: true,
            coast_m: 500.0,
            edge_m: 1000.0,
            region_m: None,
            summit: None,
            cone_m: 10_000.0,
            lowland_m: 20_000.0,
            band_m: None,
            channel_m2: 0.1e6,
            window_m: 1000.0,
            dir_steps: 5,
            dd_thresholds_m2: vec![0.1e6, 1e6, 10e6],
            relief_windows_m: vec![1000.0, 5000.0],
            patch: 512,
            max_walk_deg: declared_walk()[0].v,
        }
    }
}

/// A field ready to measure: heights at the measuring spacing.
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub grid: Grid,
    /// Guide-frame metres of sample (0, 0).
    pub origin_m: [f64; 2],
    pub sea_level_m: f64,
    pub src_spacing_m: f64,
    /// How it was brought to the measuring spacing.
    pub resample: String,
    /// Region legend index per sample and the legend names (from the guide's `regions.png`, [`region_labels`]).
    pub regions: Option<(Vec<u8>, Vec<String>)>,
    /// Heights of the base field at the same samples (untouched share, `|dh|`), metres.
    pub base: Option<Vec<f64>>,
}

/// Height distribution of a zone.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Hypsometry {
    /// Heights at [`HEIGHT_QUANTILES`], metres.
    pub quantiles_m: Vec<f64>,
    pub min_m: f64,
    pub max_m: f64,
    pub mean_m: f64,
    /// `(mean − min)/(max − min)`.
    pub integral: f64,
}

/// Slope distribution of a zone, degrees.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Slopes {
    pub mean_deg: f64,
    pub p50_deg: f64,
    pub p90_deg: f64,
    pub p99_deg: f64,
    /// Share of samples steeper than 30° / 45°.
    pub over_30: f64,
    pub over_45: f64,
}

/// Roughness in one wavelength band.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Band {
    pub lo_m: f64,
    pub hi_m: f64,
    /// RMS of the difference of local-linear Gaussian low-passes, metres.
    pub dog_rms_m: f64,
    /// RMS from the patch periodograms, metres (`None`: band outside the patch or no patch).
    pub psd_rms_m: Option<f64>,
}

/// Spectral numbers of a zone.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Spectrum {
    pub patches: usize,
    pub patch_m: f64,
    /// `(lo_m, hi_m, β)` per [`SLOPE_RANGES_M`] entry that fits.
    pub beta: Vec<(f64, f64, f64)>,
    /// Wavelength of the normalized-spectrum peak, metres, and its ratio over the power-law fit.
    pub peak_m: Option<f64>,
    pub peak_ratio: Option<f64>,
    /// `(wavelength_m, mean density m²/(cycle/m)², fitted density)` per bin with data.
    pub bins: Vec<(f64, f64, f64)>,
}

/// Local relief of a zone in one window.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Relief {
    pub window_m: f64,
    pub mean_m: f64,
    pub p50_m: f64,
    pub p90_m: f64,
}

/// Every number of one zone.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ZoneReport {
    pub zone: String,
    pub samples: usize,
    pub area_km2: f64,
    pub height: Hypsometry,
    pub slope: Slopes,
    pub bands: Vec<Band>,
    pub spectrum: Spectrum,
    /// `(threshold_km2, km per km²)`.
    pub drainage_density: Vec<(f64, f64)>,
    pub structure: Structure,
    pub sinuosity: f64,
    pub stems: usize,
    pub relief: Vec<Relief>,
    pub walk: Walkability,
}

/// Walkability of a zone (printed numbers, never limits).
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Walkability {
    pub max_walk_deg: f64,
    /// Dry interior samples of the zone the shares are of.
    pub land: usize,
    /// `(lo_deg, hi_deg, share of land)` per [`WALK_BAND_EDGES_DEG`] band.
    pub bands: Vec<(f64, f64, f64)>,
    /// Share of land with slope ≤ `max_walk_deg`.
    pub walkable: f64,
    /// Largest 4-connected walkable component, share of land.
    pub largest: f64,
    /// Number of 4-connected walkable components.
    pub components: usize,
    /// Share of land with slope < [`FLAT_DEG`].
    pub flat: f64,
    /// Share of land with slope >= [`STEEP_DEG`].
    pub steep: f64,
}

/// One row of the landforms table (LF-0): the whole valid land (`all`) or one region of it.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct LandRow {
    /// `all`, a region's legend name, or `#<index>` for an unnamed legend index.
    pub region: String,
    pub walk: Walkability,
    /// Share of the row's land whose height moved less than [`UNTOUCHED_M`] from the base field (with a base).
    pub untouched: Option<f64>,
    /// `|dh|` vs the base at the 50th / 95th percentile of the row's land, metres (with a base).
    pub dh_p50_m: Option<f64>,
    pub dh_p95_m: Option<f64>,
}

/// One measured field.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct FieldReport {
    pub name: String,
    pub reference: bool,
    pub src_spacing_m: f64,
    pub spacing_m: f64,
    pub resample: String,
    pub dims: [usize; 2],
    pub origin_m: [f64; 2],
    pub sea_level_m: f64,
    /// Summit used by the cone / lowland split, guide-frame metres, and its height.
    pub summit_m: Option<[f64; 2]>,
    pub summit_height_m: Option<f64>,
    pub seconds: f64,
    pub zones: Vec<ZoneReport>,
    /// Landforms table (LF-0): the valid land of zone `all`, then each region present in it.
    pub land: Vec<LandRow>,
}

/// Per log bin (sum of density, count), and the variance per band, of one patch.
type PatchSpectrum = (Vec<(f64, usize)>, Vec<f64>);

/// Equal band edges (edges are copied from one table, the margin only absorbs arithmetic).
fn same(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn quantile(sorted: &[f64], f: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    sorted[((sorted.len() - 1) as f64 * f).round() as usize]
}

fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.par_sort_unstable_by(f64::total_cmp);
    v
}

// ------------------------------------------------------------------------------------------------ loading

/// Display name of a guide folder: its parent and own folder names (`real/etna-20m`).
pub fn field_name(dir: &Path) -> String {
    let own = dir.file_name().map(|s| s.to_string_lossy().into_owned());
    let parent = dir
        .parent()
        .and_then(|p| p.file_name())
        .map(|s| s.to_string_lossy().into_owned());
    match (parent, own) {
        (Some(p), Some(o)) => format!("{p}/{o}"),
        (None, Some(o)) => o,
        _ => dir.display().to_string(),
    }
}

/// Reads a guide folder, crops it to `params.region_m` and brings it to `params.spacing_m`.
pub fn load_field(dir: &Path, params: &MeasureParams) -> Result<Field> {
    let gd = Guide::read(dir)?;
    let grid = Grid {
        w: gd.w,
        h: gd.h,
        spacing: gd.spacing_m,
        data: gd.heights.iter().map(|&v| f64::from(v)).collect(),
    };
    let mut f = prepare_field(
        field_name(dir),
        grid,
        gd.origin_m,
        gd.sea_level_m,
        params,
    )?;
    if let Some(rm) = &gd.regions {
        f.regions = Some((region_labels(rm, gd.origin_m, gd.spacing_m, &f), rm.names.clone()));
    }
    Ok(f)
}

/// Region legend index of every sample of `f`, from a guide's region grid: region pixel `(px, py)` covers guide
/// frame `x` from `origin.x - spacing/2 + px * cell` (pixel (0, 0) covers sample (0, 0), as
/// [`RegionMap::at_sample`]), so crops and resamples keep each point in its region.
pub fn region_labels(rm: &RegionMap, guide_origin_m: [f64; 2], guide_spacing_m: f64, f: &Field) -> Vec<u8> {
    let (w, dx) = (f.grid.w, f.grid.spacing);
    let half = 0.5 * guide_spacing_m;
    (0..f.grid.len())
        .map(|i| {
            let x = f.origin_m[0] + (i % w) as f64 * dx;
            let y = f.origin_m[1] - (i / w) as f64 * dx;
            let px = ((x - guide_origin_m[0] + half) / rm.cell_m).floor() as isize;
            let py = ((guide_origin_m[1] - y + half) / rm.cell_m).floor() as isize;
            rm.at_clamped(px, py)
        })
        .collect()
}

/// Crops and resamples an in-memory field (see [`load_field`]).
pub fn prepare_field(
    name: String,
    mut grid: Grid,
    mut origin_m: [f64; 2],
    sea_level_m: f64,
    params: &MeasureParams,
) -> Result<Field> {
    let s = grid.spacing;
    if let Some([x0, y0, x1, y1]) = params.region_m {
        let c0 = (((x0 - origin_m[0]) / s).ceil().max(0.0) as usize).min(grid.w);
        let c1 = (((x1 - origin_m[0]) / s).floor().max(-1.0) + 1.0).min(grid.w as f64) as usize;
        let r0 = (((origin_m[1] - y1) / s).ceil().max(0.0) as usize).min(grid.h);
        let r1 = (((origin_m[1] - y0) / s).floor().max(-1.0) + 1.0).min(grid.h as f64) as usize;
        if c1 < c0 + 4 || r1 < r0 + 4 {
            return Err(Error::Invalid(format!("{name}: region leaves under 4 x 4 samples")));
        }
        let mut data = Vec::with_capacity((c1 - c0) * (r1 - r0));
        for r in r0..r1 {
            data.extend_from_slice(&grid.data[r * grid.w + c0..r * grid.w + c1]);
        }
        grid = Grid {
            w: c1 - c0,
            h: r1 - r0,
            spacing: s,
            data,
        };
        origin_m = [origin_m[0] + c0 as f64 * s, origin_m[1] - r0 as f64 * s];
    }
    let ratio = params.spacing_m / s;
    let k = ratio.round();
    let how;
    if (ratio - 1.0).abs() < 1e-9 {
        how = "none".to_string();
    } else if k >= 2.0 && (ratio - k).abs() < 1e-6 {
        let k = k as usize;
        grid = block_mean(&grid, k, sea_level_m);
        let shift = (k - 1) as f64 * 0.5 * s;
        origin_m = [origin_m[0] + shift, origin_m[1] - shift];
        how = format!("{k}x{k} block mean");
    } else {
        grid = resample(&grid, params.spacing_m);
        how = "Catmull-Rom".to_string();
    }
    Ok(Field {
        name,
        grid,
        origin_m,
        sea_level_m,
        src_spacing_m: s,
        resample: how,
        regions: None,
        base: None,
    })
}

/// `k × k` block means: a block is sea when most of its samples are, and takes the mean of that class.
fn block_mean(g: &Grid, k: usize, sea: f64) -> Grid {
    let (w, h) = (g.w / k, g.h / k);
    let mut data = vec![0.0; w * h];
    data.par_chunks_mut(w).enumerate().for_each(|(by, row)| {
        for (bx, v) in row.iter_mut().enumerate() {
            let (mut sl, mut nl, mut sw, mut nw) = (0.0, 0usize, 0.0, 0usize);
            for y in by * k..(by + 1) * k {
                for x in bx * k..(bx + 1) * k {
                    let z = g.data[y * g.w + x];
                    if z < sea {
                        sw += z;
                        nw += 1;
                    } else {
                        sl += z;
                        nl += 1;
                    }
                }
            }
            *v = if nw > nl {
                sw / nw as f64
            } else {
                sl / nl as f64
            };
        }
    });
    Grid {
        w,
        h,
        spacing: g.spacing * k as f64,
        data,
    }
}

// ------------------------------------------------------------------------------------------------ masks

/// Exact Euclidean distance (in samples) from each sample to the nearest `seed` sample; `f64::INFINITY` without
/// seeds. Felzenszwalb & Huttenlocher 2012, separable lower envelope of parabolas.
pub fn distance_transform(seed: &[bool], w: usize, h: usize) -> Vec<f64> {
    // tunables: const (maths: "infinite" squared distance seed of the transform)
    const BIG: f64 = 1e20;
    let mut d: Vec<f64> = seed.iter().map(|&s| if s { 0.0 } else { BIG }).collect();
    if !seed.iter().any(|&s| s) {
        return vec![f64::INFINITY; w * h];
    }
    d.par_chunks_mut(w).for_each(edt_1d);
    let mut t = transpose(&d, w, h);
    t.par_chunks_mut(h).for_each(edt_1d);
    transpose(&t, h, w).into_iter().map(f64::sqrt).collect()
}

/// 1-D squared distance transform in place.
fn edt_1d(f: &mut [f64]) {
    let n = f.len();
    let mut v = vec![0usize; n];
    let mut z = vec![0.0f64; n + 1];
    let src = f.to_vec();
    let mut k = 0usize;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        loop {
            let p = v[k];
            let s = ((src[q] + (q * q) as f64) - (src[p] + (p * p) as f64)) / (2.0 * (q - p) as f64);
            if s <= z[k] && k > 0 {
                k -= 1;
            } else {
                k += 1;
                v[k] = q;
                z[k] = s;
                z[k + 1] = f64::INFINITY;
                break;
            }
        }
    }
    k = 0;
    for (q, out) in f.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        let dq = q as f64 - p as f64;
        *out = dq * dq + src[p];
    }
}

fn transpose<T: Copy + Send + Sync>(a: &[T], w: usize, h: usize) -> Vec<T> {
    // Output is h wide, w tall: out[x * h + y] = a[y * w + x].
    let mut out = Vec::with_capacity(w * h);
    out.extend_from_slice(a);
    out.par_chunks_mut(h).enumerate().for_each(|(x, col)| {
        for (y, v) in col.iter_mut().enumerate() {
            *v = a[y * w + x];
        }
    });
    out
}

// ------------------------------------------------------------------------------------------------ filters

/// `σ` in metres of the Gaussian whose transfer is one half at wavelength `l_m`.
pub fn sigma_for_wavelength(l_m: f64) -> f64 {
    l_m * (std::f64::consts::LN_2 / 2.0).sqrt() / std::f64::consts::PI
}

/// One row: Gaussian of `sigma` samples, zero outside the row.
fn blur_row(row: &mut [f64], sigma: f64, buf: &mut Vec<f64>) {
    let n = row.len();
    buf.clear();
    buf.extend_from_slice(row);
    if sigma < EXACT_BELOW {
        let r = (3.0 * sigma).ceil() as usize;
        let mut k: Vec<f64> = (0..=r)
            .map(|i| (-((i * i) as f64) / (2.0 * sigma * sigma)).exp())
            .collect();
        let sum = k[0] + 2.0 * k[1..].iter().sum::<f64>();
        for v in &mut k {
            *v /= sum;
        }
        for (i, out) in row.iter_mut().enumerate() {
            let mut acc = k[0] * buf[i];
            for (j, kj) in k.iter().enumerate().skip(1) {
                if i >= j {
                    acc += kj * buf[i - j];
                }
                if i + j < n {
                    acc += kj * buf[i + j];
                }
            }
            *out = acc;
        }
        return;
    }
    // Kovesi 2010: three boxes of widths wl / wu whose variances sum to σ².
    let passes = 3.0;
    let ideal = (12.0 * sigma * sigma / passes + 1.0).sqrt();
    let mut wl = ideal.floor() as i64;
    if wl % 2 == 0 {
        wl -= 1;
    }
    let wu = wl + 2;
    let (wlf, s2) = (wl as f64, sigma * sigma);
    let m = ((12.0 * s2 - passes * wlf * wlf - 4.0 * passes * wlf - 3.0 * passes) / (-4.0 * wlf - 4.0))
        .round() as i64;
    let mut tmp = vec![0.0; n];
    for pass in 0..3 {
        let width = if pass < m { wl } else { wu };
        let r = ((width - 1) / 2) as usize;
        let inv = 1.0 / width as f64;
        // Running sum over [i − r, i + r], zero outside.
        let mut acc: f64 = buf[..r.min(n)].iter().sum();
        for (i, t) in tmp.iter_mut().enumerate() {
            if i + r < n {
                acc += buf[i + r];
            }
            *t = acc * inv;
            if i >= r {
                acc -= buf[i - r];
            }
        }
        buf.copy_from_slice(&tmp);
    }
    row.copy_from_slice(buf);
}

/// Separable Gaussian of `sigma` samples over a `w × h` field, zero outside the grid.
fn blur(a: &[f64], w: usize, h: usize, sigma: f64) -> Vec<f64> {
    let mut rows = a.to_vec();
    rows.par_chunks_mut(w).for_each_init(Vec::new, |buf, row| blur_row(row, sigma, buf));
    let mut t = transpose(&rows, w, h);
    t.par_chunks_mut(h).for_each_init(Vec::new, |buf, col| blur_row(col, sigma, buf));
    transpose(&t, h, w)
}

/// Gaussian-weighted local linear fit of `z` over the samples where `mask` holds, evaluated at every sample
/// (first-order normalized convolution). `NaN` where the mask has no weight.
pub fn lowpass_local_linear(z: &[f64], mask: &[bool], w: usize, h: usize, sigma: f64) -> Vec<f64> {
    let n = w * h;
    let (cx, cy) = (w as f64 * 0.5, h as f64 * 0.5);
    let xy = |i: usize| ((i % w) as f64 - cx, (i / w) as f64 - cy);
    let field = |f: &(dyn Fn(usize, f64, f64) -> f64 + Sync)| -> Vec<f64> {
        let v: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|i| {
                if mask[i] {
                    let (x, y) = xy(i);
                    f(i, x, y)
                } else {
                    0.0
                }
            })
            .collect();
        blur(&v, w, h, sigma)
    };
    let m0 = field(&|_, _, _| 1.0);
    let mx = field(&|_, x, _| x);
    let my = field(&|_, _, y| y);
    let mxx = field(&|_, x, _| x * x);
    let mxy = field(&|_, x, y| x * y);
    let myy = field(&|_, _, y| y * y);
    let hz = field(&|i, _, _| z[i]);
    let hx = field(&|i, x, _| z[i] * x);
    let hy = field(&|i, _, y| z[i] * y);
    (0..n)
        .into_par_iter()
        .map(|i| {
            let s0 = m0[i];
            if s0 <= 1e-12 {
                return f64::NAN;
            }
            let (x0, y0) = xy(i);
            let sx = mx[i] - x0 * s0;
            let sy = my[i] - y0 * s0;
            let sxx = mxx[i] - 2.0 * x0 * mx[i] + x0 * x0 * s0;
            let syy = myy[i] - 2.0 * y0 * my[i] + y0 * y0 * s0;
            let sxy = mxy[i] - x0 * my[i] - y0 * mx[i] + x0 * y0 * s0;
            let t0 = hz[i];
            let tx = hx[i] - x0 * hz[i];
            let ty = hy[i] - y0 * hz[i];
            // Cramer on [[s0 sx sy] [sx sxx sxy] [sy sxy syy]] (a b c)ᵀ = (t0 tx ty)ᵀ, a only.
            let det = s0 * (sxx * syy - sxy * sxy) - sx * (sx * syy - sxy * sy)
                + sy * (sx * sxy - sxx * sy);
            if det.abs() <= 1e-9 * s0 * sxx.abs().max(1e-30) * syy.abs().max(1e-30) {
                return t0 / s0;
            }
            let da = t0 * (sxx * syy - sxy * sxy) - sx * (tx * syy - sxy * ty)
                + sy * (tx * sxy - sxx * ty);
            da / det
        })
        .collect()
}

/// Sliding max (or min with `neg`) over a window of half-width `r` along a line; `±inf` samples are ignored.
fn sliding_extreme(line: &mut [f64], r: usize, neg: bool, dq: &mut std::collections::VecDeque<usize>) {
    let n = line.len();
    let src = line.to_vec();
    let key = |i: usize| if neg { -src[i] } else { src[i] };
    dq.clear();
    let mut next = 0usize;
    for (i, out) in line.iter_mut().enumerate() {
        let hi = (i + r).min(n - 1);
        while next <= hi {
            while dq.back().is_some_and(|&b| key(b) <= key(next)) {
                dq.pop_back();
            }
            dq.push_back(next);
            next += 1;
        }
        while dq.front().is_some_and(|&f| f + r < i) {
            dq.pop_front();
        }
        *out = src[dq[0]];
    }
}

/// Local relief: max − min of land heights in a square window of edge `2r + 1` samples (`NaN` with no land).
fn local_relief(z: &[f64], land: &[bool], w: usize, h: usize, r: usize) -> Vec<f64> {
    let ext = |neg: bool| {
        let fill = if neg { f64::INFINITY } else { f64::NEG_INFINITY };
        let mut a: Vec<f64> = z
            .iter()
            .zip(land)
            .map(|(&v, &l)| if l { v } else { fill })
            .collect();
        a.par_chunks_mut(w)
            .for_each_init(std::collections::VecDeque::new, |dq, row| {
                sliding_extreme(row, r, neg, dq);
            });
        let mut t = transpose(&a, w, h);
        t.par_chunks_mut(h)
            .for_each_init(std::collections::VecDeque::new, |dq, col| {
                sliding_extreme(col, r, neg, dq);
            });
        transpose(&t, h, w)
    };
    let (mx, mn) = (ext(false), ext(true));
    mx.iter()
        .zip(&mn)
        .map(|(&a, &b)| if a.is_finite() && b.is_finite() { a - b } else { f64::NAN })
        .collect()
}

/// Horn (1981) slope, degrees, edges clamped.
fn slope_deg(g: &Grid) -> Vec<f64> {
    let (w, h) = (g.w as isize, g.h as isize);
    let mut out = vec![0.0; g.len()];
    out.par_chunks_mut(g.w).enumerate().for_each(|(y, row)| {
        let y = y as isize;
        for (x, o) in row.iter_mut().enumerate() {
            let x = x as isize;
            let z = |dx: isize, dy: isize| g.at_clamped((x + dx).clamp(0, w - 1), (y + dy).clamp(0, h - 1));
            let gx = ((z(1, -1) + 2.0 * z(1, 0) + z(1, 1)) - (z(-1, -1) + 2.0 * z(-1, 0) + z(-1, 1)))
                / (8.0 * g.spacing);
            let gy = ((z(-1, 1) + 2.0 * z(0, 1) + z(1, 1)) - (z(-1, -1) + 2.0 * z(0, -1) + z(1, -1)))
                / (8.0 * g.spacing);
            *o = gx.hypot(gy).atan().to_degrees();
        }
    });
    out
}

/// Central-difference slope at the grid spacing, degrees, on dry interior samples (`NaN` on sea and the border).
pub fn walk_slope_deg(g: &Grid, sea_level: f64) -> Vec<f64> {
    let (w, h) = (g.w, g.h);
    let inv2 = 0.5 / g.spacing;
    let mut out = vec![f64::NAN; g.len()];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        if y == 0 || y + 1 >= h {
            return;
        }
        for (x, o) in row.iter_mut().enumerate().take(w - 1).skip(1) {
            let i = y * w + x;
            if g.data[i] < sea_level {
                continue;
            }
            let gx = (g.data[i + 1] - g.data[i - 1]) * inv2;
            let gy = (g.data[i + w] - g.data[i - w]) * inv2;
            *o = gx.hypot(gy).atan().to_degrees();
        }
    });
    out
}

/// Walkability of the samples in `mask` from [`walk_slope_deg`] (`slope`) on a `w`-wide grid.
pub fn walkability(slope: &[f64], w: usize, mask: &[bool], max_walk_deg: f64) -> Walkability {
    let n = slope.len();
    let land = |i: usize| mask[i] && slope[i].is_finite();
    let walkable = |i: usize| land(i) && slope[i] <= max_walk_deg;
    let mut lo = 0.0;
    let mut bands: Vec<(f64, f64, f64)> = WALK_BAND_EDGES_DEG
        .iter()
        .chain(std::iter::once(&90.0))
        .map(|&hi| {
            let b = (lo, hi, 0.0);
            lo = hi;
            b
        })
        .collect();
    let (mut count, mut flat, mut steep) = (0usize, 0usize, 0usize);
    for i in (0..n).filter(|&i| land(i)) {
        count += 1;
        flat += usize::from(slope[i] < FLAT_DEG);
        steep += usize::from(slope[i] >= STEEP_DEG);
        let k = WALK_BAND_EDGES_DEG.iter().take_while(|&&e| slope[i] >= e).count();
        bands[k].2 += 1.0;
    }
    let mut seen = vec![false; n];
    let mut stack = Vec::new();
    let (mut total, mut largest, mut components) = (0usize, 0usize, 0usize);
    for start in 0..n {
        if seen[start] || !walkable(start) {
            continue;
        }
        components += 1;
        seen[start] = true;
        stack.push(start);
        let mut size = 0usize;
        while let Some(c) = stack.pop() {
            size += 1;
            let (cx, cy) = (c % w, c / w);
            let mut visit = |j: usize| {
                if !seen[j] && walkable(j) {
                    seen[j] = true;
                    stack.push(j);
                }
            };
            if cx + 1 < w {
                visit(c + 1);
            }
            if cx > 0 {
                visit(c - 1);
            }
            if c + w < n {
                visit(c + w);
            }
            if cy > 0 {
                visit(c - w);
            }
        }
        total += size;
        largest = largest.max(size);
    }
    let share = |v: usize| if count > 0 { v as f64 / count as f64 } else { 0.0 };
    for b in &mut bands {
        b.2 = if count > 0 { b.2 / count as f64 } else { 0.0 };
    }
    Walkability {
        max_walk_deg,
        land: count,
        bands,
        walkable: share(total),
        largest: share(largest),
        components,
        flat: share(flat),
        steep: share(steep),
    }
}

/// The landforms table (LF-0): [`walkability`] of the whole `mask` (row `all`), then of `mask` and each region of
/// `regions` (labels per sample, legend names) that holds land, in legend order; with `dh` (field - base per
/// sample) also the untouched share and the `|dh|` percentiles over each row's land (dry interior samples).
pub fn land_rows(
    slope: &[f64],
    w: usize,
    mask: &[bool],
    regions: Option<(&[u8], &[String])>,
    dh: Option<&[f64]>,
    max_walk_deg: f64,
) -> Vec<LandRow> {
    let row = |name: String, m: &[bool]| {
        let walk = walkability(slope, w, m, max_walk_deg);
        let (mut untouched, mut dh_p50_m, mut dh_p95_m) = (None, None, None);
        if let Some(d) = dh {
            let ad = sorted(
                (0..m.len())
                    .filter(|&i| m[i] && slope[i].is_finite())
                    .map(|i| d[i].abs())
                    .collect(),
            );
            if !ad.is_empty() {
                untouched = Some(ad.partition_point(|&v| v < UNTOUCHED_M) as f64 / ad.len() as f64);
                dh_p50_m = Some(quantile(&ad, 0.5));
                dh_p95_m = Some(quantile(&ad, 0.95));
            }
        }
        LandRow {
            region: name,
            walk,
            untouched,
            dh_p50_m,
            dh_p95_m,
        }
    };
    let mut rows = vec![row("all".into(), mask)];
    if let Some((labels, names)) = regions {
        let mut present = [false; 256];
        for i in (0..mask.len()).filter(|&i| mask[i] && slope[i].is_finite()) {
            present[usize::from(labels[i])] = true;
        }
        for v in (0..=255u8).filter(|&v| present[usize::from(v)]) {
            let m: Vec<bool> = mask.iter().zip(labels).map(|(&k, &l)| k && l == v).collect();
            let name = names.get(usize::from(v)).filter(|n| !n.is_empty()).cloned();
            rows.push(row(name.unwrap_or_else(|| format!("#{v}")), &m));
        }
    }
    rows
}

// ------------------------------------------------------------------------------------------------ spectrum

/// In-place radix-2 complex FFT (`re.len()` a power of two), forward sign `e^{−2πi kn/N}`.
pub fn fft(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = -2.0 * std::f64::consts::PI / len as f64;
        let half = len / 2;
        for start in (0..n).step_by(len) {
            for k in 0..half {
                let (wi, wr) = (ang * k as f64).sin_cos();
                let (a, b) = (start + k, start + k + half);
                let tr = re[b] * wr - im[b] * wi;
                let ti = re[b] * wi + im[b] * wr;
                re[b] = re[a] - tr;
                im[b] = im[a] - ti;
                re[a] += tr;
                im[a] += ti;
            }
        }
        len <<= 1;
    }
}

/// Periodogram of one `p × p` patch of spacing `dx`: per log bin (sum of density, count), and the variance in each
/// `(lo, hi)` band. Plane-detrended, Hann-windowed, normalized so the sum over frequencies is the variance.
fn patch_spectrum(z: &[f64], p: usize, dx: f64, bands: &[(f64, f64)], nbins: usize) -> PatchSpectrum {
    // Least-squares plane (x, y centred: the normal equations decouple).
    let c = (p as f64 - 1.0) * 0.5;
    let (mut sz, mut sxz, mut syz, mut sxx) = (0.0, 0.0, 0.0, 0.0);
    for y in 0..p {
        for x in 0..p {
            let (xf, yf, v) = (x as f64 - c, y as f64 - c, z[y * p + x]);
            sz += v;
            sxz += xf * v;
            syz += yf * v;
        }
    }
    for x in 0..p {
        let xf = x as f64 - c;
        sxx += xf * xf * p as f64;
    }
    let (a, b, cc) = (sz / (p * p) as f64, sxz / sxx, syz / sxx);
    let hann: Vec<f64> = (0..p)
        .map(|i| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / (p as f64 - 1.0)).cos())
        .collect();
    let mut re = vec![0.0; p * p];
    let mut im = vec![0.0; p * p];
    let mut wss = 0.0;
    for y in 0..p {
        for x in 0..p {
            let wgt = hann[x] * hann[y];
            wss += wgt * wgt;
            re[y * p + x] = wgt * (z[y * p + x] - a - b * (x as f64 - c) - cc * (y as f64 - c));
        }
    }
    for y in 0..p {
        fft(&mut re[y * p..(y + 1) * p], &mut im[y * p..(y + 1) * p]);
    }
    let (mut cr, mut ci) = (vec![0.0; p], vec![0.0; p]);
    for x in 0..p {
        for y in 0..p {
            cr[y] = re[y * p + x];
            ci[y] = im[y * p + x];
        }
        fft(&mut cr, &mut ci);
        for y in 0..p {
            re[y * p + x] = cr[y];
            im[y * p + x] = ci[y];
        }
    }
    let norm = 1.0 / ((p * p) as f64 * wss);
    let df = 1.0 / (p as f64 * dx);
    let mut bins = vec![(0.0, 0usize); nbins];
    let mut band_var = vec![0.0; bands.len()];
    for ky in 0..p {
        let fy = if ky <= p / 2 { ky as f64 } else { ky as f64 - p as f64 };
        for kx in 0..p {
            if kx == 0 && ky == 0 {
                continue;
            }
            let fx = if kx <= p / 2 { kx as f64 } else { kx as f64 - p as f64 };
            let pk = (re[ky * p + kx].powi(2) + im[ky * p + kx].powi(2)) * norm;
            let f = fx.hypot(fy) * df;
            let lambda = 1.0 / f;
            for (bv, &(lo, hi)) in band_var.iter_mut().zip(bands) {
                if lambda >= lo && lambda < hi {
                    *bv += pk;
                }
            }
            let bi = (BINS_PER_DECADE * (lambda / dx).log10()).floor();
            if bi >= 0.0 && (bi as usize) < nbins {
                let e = &mut bins[bi as usize];
                e.0 += pk / (df * df);
                e.1 += 1;
            }
        }
    }
    (bins, band_var)
}

/// Least-squares line through `(x, y)`: (slope, intercept); `None` under 3 points.
fn fit_line(pts: &[(f64, f64)]) -> Option<(f64, f64)> {
    if pts.len() < 3 {
        return None;
    }
    let n = pts.len() as f64;
    let (mx, my) = (
        pts.iter().map(|p| p.0).sum::<f64>() / n,
        pts.iter().map(|p| p.1).sum::<f64>() / n,
    );
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    (sxx > 0.0).then(|| {
        let s = sxy / sxx;
        (s, my - s * mx)
    })
}

/// Spectral numbers from per-patch bins: mean density per bin, slopes, normalized peak.
fn summarize_spectrum(per_patch: &[&Vec<(f64, usize)>], dx: f64, p: usize) -> Spectrum {
    let nbins = per_patch.first().map_or(0, |b| b.len());
    let mut sp = Spectrum {
        patches: per_patch.len(),
        patch_m: p as f64 * dx,
        ..Spectrum::default()
    };
    if per_patch.is_empty() {
        return sp;
    }
    let lam_b = |b: usize| dx * 10f64.powf((b as f64 + 0.5) / BINS_PER_DECADE);
    let mut mean = vec![f64::NAN; nbins];
    for (b, m) in mean.iter_mut().enumerate() {
        let vals: Vec<f64> = per_patch
            .iter()
            .filter(|pb| pb[b].1 > 0)
            .map(|pb| pb[b].0 / pb[b].1 as f64)
            .collect();
        if vals.len() == per_patch.len() {
            *m = vals.iter().sum::<f64>() / vals.len() as f64;
        }
    }
    let max_l = sp.patch_m * 0.5;
    let pts_in = |lo: f64, hi: f64| -> Vec<(f64, f64)> {
        (0..nbins)
            .filter(|&b| mean[b] > 0.0 && lam_b(b) >= lo && lam_b(b) <= hi.min(max_l))
            .map(|b| (-lam_b(b).log10(), mean[b].log10()))
            .collect()
    };
    for [lo, hi] in SLOPE_RANGES_M {
        if lo < 2.0 * dx {
            continue;
        }
        if let Some((s, _)) = fit_line(&pts_in(lo, hi)) {
            sp.beta.push((lo, hi, -s));
        }
    }
    // Ridge-valley peak: up to a quarter patch, where the plane detrend no longer leaves whole-edifice curvature.
    let (lo, hi) = (5.0 * dx, sp.patch_m * 0.25);
    let fit = fit_line(&pts_in(lo, hi));
    for (b, &m) in mean.iter().enumerate() {
        if m > 0.0 {
            let fitted = fit.map_or(f64::NAN, |(s, c)| 10f64.powf(c - s * lam_b(b).log10()));
            sp.bins.push((lam_b(b), m, fitted));
        }
    }
    if fit.is_some() {
        let best = sp
            .bins
            .iter()
            .filter(|(l, _, _)| *l >= lo && *l <= hi)
            .max_by(|a, b| (a.1 / a.2).total_cmp(&(b.1 / b.2)));
        if let Some(&(l, m, f)) = best {
            sp.peak_m = Some(l);
            sp.peak_ratio = Some(m / f);
        }
    }
    sp
}

// ------------------------------------------------------------------------------------------------ measuring

/// Bands a field of spacing `dx` and shorter side `side_m` reports.
pub fn bands_for(dx: f64, side_m: f64) -> Vec<(f64, f64)> {
    BAND_EDGES_M
        .windows(2)
        .map(|e| (e[0], e[1]))
        .filter(|&(lo, hi)| lo >= 2.0 * dx - 1e-9 && hi <= side_m * 0.5)
        .collect()
}

struct Zone {
    name: &'static str,
    mask: Vec<bool>,
}

/// Measures one field.
#[allow(clippy::too_many_lines)]
pub fn measure_field(f: &Field, params: &MeasureParams, reference: bool) -> FieldReport {
    let t = std::time::Instant::now();
    let g = &f.grid;
    let (w, h, n, dx) = (g.w, g.h, g.len(), g.spacing);
    let min_h = g.data.iter().copied().fold(f64::INFINITY, f64::min);
    let sea = if params.mask_sea { f.sea_level_m } else { min_h - 1.0 };
    let wet: Vec<bool> = g.data.iter().map(|&v| v < sea).collect();
    let land: Vec<bool> = wet.iter().map(|&v| !v).collect();
    let dsea = distance_transform(&wet, w, h);
    let edge = (params.edge_m / dx).ceil() as usize;
    let base: Vec<bool> = (0..n)
        .map(|i| {
            let (x, y) = (i % w, i / w);
            land[i]
                && dsea[i] * dx >= params.coast_m
                && x >= edge
                && y >= edge
                && x + edge < w
                && y + edge < h
        })
        .collect();
    let mut rep = FieldReport {
        name: f.name.clone(),
        reference,
        src_spacing_m: f.src_spacing_m,
        spacing_m: dx,
        resample: f.resample.clone(),
        dims: [w, h],
        origin_m: f.origin_m,
        sea_level_m: f.sea_level_m,
        ..FieldReport::default()
    };
    // Zones.
    let mut zones = vec![Zone {
        name: "all",
        mask: base.clone(),
    }];
    let summit_xy = params.summit.and_then(|s| match s {
        Summit::Auto => (0..n)
            .filter(|&i| base[i])
            .max_by(|&a, &b| g.data[a].total_cmp(&g.data[b]).then(b.cmp(&a)))
            .map(|i| ((i % w) as f64, (i / w) as f64)),
        Summit::At([x, y]) => Some(((x - f.origin_m[0]) / dx, (f.origin_m[1] - y) / dx)),
    });
    if let Some((sx, sy)) = summit_xy {
        rep.summit_m = Some([f.origin_m[0] + sx * dx, f.origin_m[1] - sy * dx]);
        rep.summit_height_m = Some(g.at_clamped(sx.round() as isize, sy.round() as isize));
        let dist = |i: usize| ((i % w) as f64 - sx).hypot((i / w) as f64 - sy) * dx;
        zones.push(Zone {
            name: "cone",
            mask: (0..n).map(|i| base[i] && dist(i) <= params.cone_m).collect(),
        });
        zones.push(Zone {
            name: "lowland",
            mask: (0..n).map(|i| base[i] && dist(i) >= params.lowland_m).collect(),
        });
    }
    if let Some([lo, hi]) = params.band_m {
        zones.push(Zone {
            name: "band",
            mask: (0..n)
                .map(|i| base[i] && g.data[i] >= lo && g.data[i] <= hi)
                .collect(),
        });
    }
    // Field-wide layers.
    let slope = slope_deg(g);
    let walk_slope = walk_slope_deg(g, f.sea_level_m);
    let side_m = (w.min(h) as f64) * dx;
    let bands = bands_for(dx, side_m);
    let mean_land = {
        let (s, c) = g
            .data
            .iter()
            .zip(&land)
            .filter(|(_, l)| **l)
            .fold((0.0, 0usize), |(s, c), (v, _)| (s + v, c + 1));
        if c > 0 { s / c as f64 } else { 0.0 }
    };
    let zc: Vec<f64> = g.data.iter().map(|v| v - mean_land).collect();
    let mut band_sig: Vec<Vec<f32>> = Vec::with_capacity(bands.len());
    let lp = |l: f64| -> Vec<f64> {
        let s = sigma_for_wavelength(l) / dx;
        if s < IDENTITY_BELOW {
            zc.clone()
        } else {
            lowpass_local_linear(&zc, &land, w, h, s)
        }
    };
    let mut prev: Option<(f64, Vec<f64>)> = None;
    for &(lo, hi) in &bands {
        let lo_lp = match prev.take() {
            Some((l, v)) if (l - lo).abs() < 1e-9 => v,
            _ => lp(lo),
        };
        let hi_lp = lp(hi);
        band_sig.push(
            lo_lp
                .iter()
                .zip(&hi_lp)
                .map(|(a, b)| (a - b) as f32)
                .collect(),
        );
        prev = Some((hi, hi_lp));
    }
    drop(prev);
    let reliefs: Vec<(f64, Vec<f64>)> = params
        .relief_windows_m
        .iter()
        .map(|&wm| {
            let r = ((wm / dx - 1.0) * 0.5).round().max(0.0) as usize;
            (wm, local_relief(&g.data, &land, w, h, r))
        })
        .collect();
    let mut z = Vec::new();
    let routing = route(g, &wet, sea, &mut z);
    // Patches.
    let p = params.patch;
    let stride = (p / 2).max(1);
    let mut origins = Vec::new();
    if p.is_power_of_two() && p >= 8 && p <= w && p <= h {
        // Summed-area count of base samples for the "every sample valid" test.
        let mut sat = vec![0u32; (w + 1) * (h + 1)];
        for y in 0..h {
            let mut row = 0u32;
            for x in 0..w {
                row += u32::from(base[y * w + x]);
                sat[(y + 1) * (w + 1) + x + 1] = sat[y * (w + 1) + x + 1] + row;
            }
        }
        let full = (p * p) as u32;
        let mut y0 = 0;
        while y0 + p <= h {
            let mut x0 = 0;
            while x0 + p <= w {
                let (x1, y1) = (x0 + p, y0 + p);
                let c = sat[y1 * (w + 1) + x1] + sat[y0 * (w + 1) + x0]
                    - sat[y0 * (w + 1) + x1]
                    - sat[y1 * (w + 1) + x0];
                if c == full {
                    origins.push((x0, y0));
                }
                x0 += stride;
            }
            y0 += stride;
        }
    }
    let nbins = (BINS_PER_DECADE * (p as f64).log10()).ceil() as usize + 1;
    let spectra: Vec<PatchSpectrum> = origins
        .par_iter()
        .map(|&(x0, y0)| {
            let mut pz = Vec::with_capacity(p * p);
            for y in y0..y0 + p {
                pz.extend_from_slice(&g.data[y * w + x0..y * w + x0 + p]);
            }
            patch_spectrum(&pz, p, dx, &bands, nbins)
        })
        .collect();
    let patch_m = p as f64 * dx;
    // Per zone.
    for zone in &zones {
        let mask = &zone.mask;
        let idx: Vec<usize> = (0..n).filter(|&i| mask[i]).collect();
        if idx.is_empty() {
            continue;
        }
        let mut zr = ZoneReport {
            zone: zone.name.to_string(),
            samples: idx.len(),
            area_km2: idx.len() as f64 * dx * dx * 1e-6,
            ..ZoneReport::default()
        };
        let hs = sorted(idx.iter().map(|&i| g.data[i]).collect());
        let mean = hs.iter().sum::<f64>() / hs.len() as f64;
        let (mn, mx) = (hs[0], hs[hs.len() - 1]);
        zr.height = Hypsometry {
            quantiles_m: HEIGHT_QUANTILES.iter().map(|&q| quantile(&hs, q)).collect(),
            min_m: mn,
            max_m: mx,
            mean_m: mean,
            integral: if mx > mn { (mean - mn) / (mx - mn) } else { f64::NAN },
        };
        let ss = sorted(idx.iter().map(|&i| slope[i]).collect());
        let cnt = ss.len() as f64;
        zr.slope = Slopes {
            mean_deg: ss.iter().sum::<f64>() / cnt,
            p50_deg: quantile(&ss, 0.5),
            p90_deg: quantile(&ss, 0.9),
            p99_deg: quantile(&ss, 0.99),
            over_30: ss.iter().filter(|&&v| v > 30.0).count() as f64 / cnt,
            over_45: ss.iter().filter(|&&v| v > 45.0).count() as f64 / cnt,
        };
        let zone_patches: Vec<usize> = origins
            .iter()
            .enumerate()
            .filter(|(_, (x0, y0))| mask[(y0 + p / 2) * w + x0 + p / 2])
            .map(|(k, _)| k)
            .collect();
        for (b, &(lo, hi)) in bands.iter().enumerate() {
            let sig = &band_sig[b];
            let ms = idx
                .iter()
                .map(|&i| f64::from(sig[i]).powi(2))
                .filter(|v| v.is_finite())
                .sum::<f64>()
                / idx.len() as f64;
            let psd = (!zone_patches.is_empty() && hi <= 0.5 * patch_m + 1e-9).then(|| {
                (zone_patches.iter().map(|&k| spectra[k].1[b]).sum::<f64>()
                    / zone_patches.len() as f64)
                    .sqrt()
            });
            zr.bands.push(Band {
                lo_m: lo,
                hi_m: hi,
                dog_rms_m: ms.sqrt(),
                psd_rms_m: psd,
            });
        }
        let pb: Vec<&Vec<(f64, usize)>> = zone_patches.iter().map(|&k| &spectra[k].0).collect();
        zr.spectrum = summarize_spectrum(&pb, dx, p);
        zr.spectrum.patch_m = patch_m;
        let link = |i: usize| {
            let r = routing.receiver[i] as usize;
            ((r % w) as f64 - (i % w) as f64).hypot((r / w) as f64 - (i / w) as f64) * dx
        };
        for &t in &params.dd_thresholds_m2 {
            let len: f64 = idx
                .iter()
                .filter(|&&i| routing.area[i] >= t && routing.receiver[i] as usize != i)
                .map(|&i| link(i))
                .sum();
            // + 0.0: an empty f64 sum is -0.0.
            zr.drainage_density.push(((t * 1e-6 * 1e6).round() / 1e6, len * 1e-3 / zr.area_km2 + 0.0));
        }
        let below = sea - 1.0;
        let zg = Grid {
            data: (0..n).map(|i| if mask[i] { g.data[i] } else { below }).collect(),
            ..g.clone()
        };
        zr.structure = structure(&zg, sea, params.channel_m2, params.window_m, params.dir_steps);
        let cm = channel_metrics(&zg, sea, params.channel_m2);
        zr.sinuosity = cm.mean_sinuosity;
        zr.stems = cm.stems;
        for (wm, rel) in &reliefs {
            let rs = sorted(idx.iter().map(|&i| rel[i]).filter(|v| v.is_finite()).collect());
            zr.relief.push(Relief {
                window_m: *wm,
                mean_m: rs.iter().sum::<f64>() / rs.len().max(1) as f64,
                p50_m: quantile(&rs, 0.5),
                p90_m: quantile(&rs, 0.9),
            });
        }
        zr.walk = walkability(&walk_slope, w, mask, params.max_walk_deg);
        rep.zones.push(zr);
    }
    let dh: Option<Vec<f64>> = f
        .base
        .as_ref()
        .map(|b| g.data.iter().zip(b).map(|(v, b)| v - b).collect());
    rep.land = land_rows(
        &walk_slope,
        w,
        &zones[0].mask,
        f.regions.as_ref().map(|(l, n)| (l.as_slice(), n.as_slice())),
        dh.as_deref(),
        params.max_walk_deg,
    );
    rep.seconds = t.elapsed().as_secs_f64();
    rep
}

// ------------------------------------------------------------------------------------------------ report

fn fmt(v: f64, d: usize) -> String {
    if v.is_finite() {
        format!("{v:.d$}")
    } else {
        "-".into()
    }
}

fn opt(v: Option<f64>, d: usize) -> String {
    v.map_or_else(|| "-".into(), |v| fmt(v, d))
}

fn km2_label(m2: f64) -> String {
    format!("{}", (m2 * 1e-6 * 1e6).round() / 1e6)
}

fn len_label(m: f64) -> String {
    if m >= 1000.0 {
        format!("{} km", m / 1000.0)
    } else {
        format!("{m} m")
    }
}

/// A scalar metric of a zone for the gap table: (label, extractor).
type Metric = (String, Box<dyn Fn(&ZoneReport) -> Option<f64>>);

fn gap_metrics(bands: &[(f64, f64)], params: &MeasureParams) -> Vec<Metric> {
    let mut m: Vec<Metric> = vec![
        ("height p50 m".into(), Box::new(|z: &ZoneReport| z.height.quantiles_m.get(3).copied())),
        ("hypsometric integral".into(), Box::new(|z: &ZoneReport| Some(z.height.integral))),
        ("slope p50 deg".into(), Box::new(|z: &ZoneReport| Some(z.slope.p50_deg))),
        ("slope p90 deg".into(), Box::new(|z: &ZoneReport| Some(z.slope.p90_deg))),
        ("share > 30 deg".into(), Box::new(|z: &ZoneReport| Some(z.slope.over_30))),
    ];
    for (b, &(lo, hi)) in bands.iter().enumerate() {
        m.push((
            format!("DoG RMS {}-{} m", len_label(lo), len_label(hi)),
            Box::new(move |z: &ZoneReport| z.bands.get(b).map(|x| x.dog_rms_m)),
        ));
        m.push((
            format!("PSD RMS {}-{} m", len_label(lo), len_label(hi)),
            Box::new(move |z: &ZoneReport| z.bands.get(b).and_then(|x| x.psd_rms_m)),
        ));
    }
    for [lo, hi] in SLOPE_RANGES_M {
        m.push((
            format!("beta {}-{}", len_label(lo), len_label(hi)),
            Box::new(move |z: &ZoneReport| {
                z.spectrum
                    .beta
                    .iter()
                    .find(|b| same(b.0, lo) && same(b.1, hi))
                    .map(|b| b.2)
            }),
        ));
    }
    m.push(("PSD peak wavelength m".into(), Box::new(|z: &ZoneReport| z.spectrum.peak_m)));
    for (k, &t) in params.dd_thresholds_m2.iter().enumerate() {
        m.push((
            format!("drainage density @ {} km2", km2_label(t)),
            Box::new(move |z: &ZoneReport| z.drainage_density.get(k).map(|d| d.1)),
        ));
    }
    m.push(("coherence mean".into(), Box::new(|z: &ZoneReport| Some(z.structure.coherence_mean))));
    m.push(("junction mean deg".into(), Box::new(|z: &ZoneReport| Some(z.structure.junction_mean_deg))));
    m.push(("sinuosity".into(), Box::new(|z: &ZoneReport| Some(z.sinuosity))));
    for (k, &wm) in params.relief_windows_m.iter().enumerate() {
        m.push((
            format!("relief {} p50 m", len_label(wm)),
            Box::new(move |z: &ZoneReport| z.relief.get(k).map(|r| r.p50_m)),
        ));
    }
    m
}

/// Gap of `v` to the range `[lo, hi]`: `in`, or the signed distance and the ratio to the nearer end.
fn gap(v: f64, lo: f64, hi: f64) -> String {
    if !v.is_finite() || !lo.is_finite() {
        return "-".into();
    }
    let mag = v.abs().max(lo.abs()).max(hi.abs());
    let d = if mag >= 100.0 { 0 } else if mag >= 1.0 { 2 } else { 3 };
    if v > hi {
        format!("+{} (x{:.2})", fmt(v - hi, d), v / hi)
    } else if v < lo {
        format!("-{} (x{:.2})", fmt(lo - v, d), v / lo)
    } else {
        "in".into()
    }
}

/// Markdown tables of a set of reports (reference fields first, in the order given).
#[allow(clippy::too_many_lines)]
pub fn markdown(reports: &[FieldReport], params: &MeasureParams) -> String {
    let mut s = String::new();
    let rows = |s: &mut String, f: &dyn Fn(&FieldReport, &ZoneReport) -> String| {
        for r in reports {
            for z in &r.zones {
                let tag = if r.reference { " (ref)" } else { "" };
                let _ = writeln!(s, "| {}{tag} | {} | {} |", r.name, z.zone, f(r, z));
            }
        }
    };
    let _ = writeln!(s, "## Fields\n");
    let _ = writeln!(
        s,
        "| Field | Source spacing m | Resample | Dims @ {} m | Summit (x, y) m | Summit h m | Zones (km2) | s |\n|---|---|---|---|---|---|---|---|",
        params.spacing_m
    );
    for r in reports {
        let zones: Vec<String> = r.zones.iter().map(|z| format!("{} {:.0}", z.zone, z.area_km2)).collect();
        let _ = writeln!(
            s,
            "| {}{} | {} | {} | {} x {} | {} | {} | {} | {:.1} |",
            r.name,
            if r.reference { " (ref)" } else { "" },
            r.src_spacing_m,
            r.resample,
            r.dims[0],
            r.dims[1],
            r.summit_m.map_or_else(|| "-".into(), |m| format!("({:.0}, {:.0})", m[0], m[1])),
            opt(r.summit_height_m, 0),
            zones.join(", "),
            r.seconds
        );
    }
    let _ = writeln!(s, "\n## Hypsometry and slope\n");
    let _ = writeln!(
        s,
        "| Field | Zone | Area km2 | h p5 | h p25 | h p50 | h p75 | h p95 | h max | HI | slope mean | p50 | p90 | p99 | > 30 deg | > 45 deg |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    );
    rows(&mut s, &|_, z| {
        let q = &z.height.quantiles_m;
        format!(
            "{:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.3} | {:.1} | {:.1} | {:.1} | {:.1} | {:.3} | {:.3}",
            z.area_km2, q[1], q[2], q[3], q[4], q[5], z.height.max_m, z.height.integral,
            z.slope.mean_deg, z.slope.p50_deg, z.slope.p90_deg, z.slope.p99_deg, z.slope.over_30, z.slope.over_45
        )
    });
    let all_bands = BAND_EDGES_M.windows(2).map(|e| (e[0], e[1])).filter(|&(lo, _)| lo >= 2.0 * params.spacing_m - 1e-9);
    let all_bands: Vec<(f64, f64)> = all_bands.collect();
    let head: Vec<String> = all_bands.iter().map(|&(lo, hi)| format!("{}-{}", len_label(lo), len_label(hi))).collect();
    let sep = "|---".repeat(all_bands.len());
    let band_cell = |z: &ZoneReport, lo: f64, psd: bool| {
        z.bands.iter().find(|b| same(b.lo_m, lo)).map_or_else(
            || "-".into(),
            |b| if psd { opt(b.psd_rms_m, 2) } else { fmt(b.dog_rms_m, 2) },
        )
    };
    let _ = writeln!(s, "\n## Roughness by wavelength band: DoG RMS, m (local-linear Gaussian band-pass)\n");
    let _ = writeln!(s, "| Field | Zone | {} |\n|---|---{sep}|", head.join(" | "));
    rows(&mut s, &|_, z| all_bands.iter().map(|&(lo, _)| band_cell(z, lo, false)).collect::<Vec<_>>().join(" | "));
    // PSD columns: bands up to half a patch, slope ranges above the Nyquist edge.
    let half_patch = 0.5 * params.patch as f64 * params.spacing_m;
    let psd_bands: Vec<(f64, f64)> = all_bands.iter().copied().filter(|b| b.1 <= half_patch + 1e-9).collect();
    let psd_head: Vec<String> = psd_bands.iter().map(|&(lo, hi)| format!("{}-{}", len_label(lo), len_label(hi))).collect();
    let ranges: Vec<[f64; 2]> = SLOPE_RANGES_M.iter().copied().filter(|r| r[0] >= 2.0 * params.spacing_m - 1e-9).collect();
    let slope_head: Vec<String> = ranges
        .iter()
        .map(|[lo, hi]| format!("beta {}-{}", len_label(*lo), len_label(*hi)))
        .collect();
    let _ = writeln!(s, "\n## Roughness by wavelength band: PSD RMS, m, and spectral slope beta (patch periodograms)\n");
    let _ = writeln!(
        s,
        "| Field | Zone | Patches | {} | {} |\n|---|---|---{}{}|",
        psd_head.join(" | "),
        slope_head.join(" | "),
        "|---".repeat(psd_bands.len()),
        "|---".repeat(ranges.len())
    );
    rows(&mut s, &|_, z| {
        let b: Vec<String> = psd_bands.iter().map(|&(lo, _)| band_cell(z, lo, true)).collect();
        let be: Vec<String> = ranges
            .iter()
            .map(|[lo, hi]| {
                z.spectrum
                    .beta
                    .iter()
                    .find(|x| same(x.0, *lo) && same(x.1, *hi))
                    .map_or_else(|| "-".into(), |x| fmt(x.2, 2))
            })
            .collect();
        format!("{} | {} | {}", z.spectrum.patches, b.join(" | "), be.join(" | "))
    });
    let dd_head: Vec<String> = params.dd_thresholds_m2.iter().map(|t| format!("Dd @ {} km2", km2_label(*t))).collect();
    let _ = writeln!(s, "\n## Valley spacing and drainage density (km/km2)\n");
    let _ = writeln!(
        s,
        "| Field | Zone | PSD peak wavelength m | peak / fit | {} | 1/Dd @ 1 km2, m |\n|---|---|---|---{}|---|",
        dd_head.join(" | "),
        "|---".repeat(dd_head.len())
    );
    rows(&mut s, &|_, z| {
        let dd: Vec<String> = z.drainage_density.iter().map(|d| fmt(d.1, 3)).collect();
        let one = z
            .drainage_density
            .iter()
            .find(|d| (d.0 - 1.0).abs() < 1e-9)
            .map_or(f64::NAN, |d| 1000.0 / d.1);
        format!(
            "{} | {} | {} | {}",
            opt(z.spectrum.peak_m, 0),
            opt(z.spectrum.peak_ratio, 2),
            dd.join(" | "),
            fmt(one, 0)
        )
    });
    let _ = writeln!(
        s,
        "\n## Channel structure (structure.rs at {} km2, {} m window, {} steps; sinuosity from metrics.rs)\n",
        km2_label(params.channel_m2),
        params.window_m,
        params.dir_steps
    );
    let _ = writeln!(
        s,
        "| Field | Zone | Coherence mean | median | p90 | Junction mean deg | median | Junctions | Dd (structure) | Sinuosity | Stems |\n|---|---|---|---|---|---|---|---|---|---|---|"
    );
    rows(&mut s, &|_, z| {
        let st = &z.structure;
        format!(
            "{:.3} | {:.3} | {:.3} | {:.1} | {:.1} | {} | {:.3} | {:.2} | {}",
            st.coherence_mean, st.coherence_median, st.coherence_p90, st.junction_mean_deg,
            st.junction_median_deg, st.junctions, st.drainage_density, z.sinuosity, z.stems
        )
    });
    let rel_head: Vec<String> = params
        .relief_windows_m
        .iter()
        .map(|w| format!("{} mean | p50 | p90", len_label(*w)))
        .collect();
    let _ = writeln!(s, "\n## Local relief (max - min of land in square windows), m\n");
    let _ = writeln!(
        s,
        "| Field | Zone | {} |\n|---|---{}|",
        rel_head.join(" | "),
        "|---|---|---".repeat(params.relief_windows_m.len())
    );
    rows(&mut s, &|_, z| {
        z.relief
            .iter()
            .map(|r| format!("{:.0} | {:.0} | {:.0}", r.mean_m, r.p50_m, r.p90_m))
            .collect::<Vec<_>>()
            .join(" | ")
    });
    let _ = writeln!(
        s,
        "
## Walkability (central-difference slope, dry land; walkable <= {} deg, 4-connected)
",
        params.max_walk_deg
    );
    let mut walk_head: Vec<String> = std::iter::once(&0.0)
        .chain(&WALK_BAND_EDGES_DEG)
        .zip(&WALK_BAND_EDGES_DEG)
        .map(|(lo, hi)| format!("{lo}-{hi} deg %"))
        .collect();
    walk_head.push(format!("> {} deg %", WALK_BAND_EDGES_DEG[WALK_BAND_EDGES_DEG.len() - 1]));
    let _ = writeln!(
        s,
        "| Field | Zone | Land km2 | {} | Walkable % | Largest walkable % | Components |
|---|---|---{}|---|---|---|",
        walk_head.join(" | "),
        "|---".repeat(walk_head.len())
    );
    rows(&mut s, &|r, z| {
        let wk = &z.walk;
        let bands: Vec<String> = wk.bands.iter().map(|b| format!("{:.1}", 100.0 * b.2)).collect();
        format!(
            "{:.0} | {} | {:.1} | {:.1} | {}",
            wk.land as f64 * r.spacing_m * r.spacing_m * 1e-6,
            bands.join(" | "),
            100.0 * wk.walkable,
            100.0 * wk.largest,
            wk.components
        )
    });
    let _ = writeln!(
        s,
        "
## Landforms baseline per region (LF-0; zone `all` valid land, then each region of it)

Steep = slope >= {STEEP_DEG} deg, flat = < {FLAT_DEG} deg; untouched = |dh| < {UNTOUCHED_M} m vs the base field (`--base`).
Components are counted inside the row (a region's border cuts them).

| Field | Region | Land km2 | {} | >= {STEEP_DEG} deg % | < {FLAT_DEG} deg % | Walkable % | Largest walkable % | Components | Untouched % | dh p50 m | dh p95 m |
|---|---|---{}|---|---|---|---|---|---|---|---|",
        walk_head.join(" | "),
        "|---".repeat(walk_head.len())
    );
    for r in reports {
        for row in &r.land {
            let wk = &row.walk;
            let bands: Vec<String> = wk.bands.iter().map(|b| format!("{:.1}", 100.0 * b.2)).collect();
            let _ = writeln!(
                s,
                "| {} | {} | {:.0} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {} | {} | {} | {} |",
                r.name,
                row.region,
                wk.land as f64 * r.spacing_m * r.spacing_m * 1e-6,
                bands.join(" | "),
                100.0 * wk.steep,
                100.0 * wk.flat,
                100.0 * wk.walkable,
                100.0 * wk.largest,
                wk.components,
                opt(row.untouched.map(|v| 100.0 * v), 1),
                opt(row.dh_p50_m, 2),
                opt(row.dh_p95_m, 2),
            );
        }
    }
    // Gap section.
    let refs: Vec<&FieldReport> = reports.iter().filter(|r| r.reference).collect();
    let ours: Vec<&FieldReport> = reports.iter().filter(|r| !r.reference).collect();
    if !refs.is_empty() && !ours.is_empty() {
        let ref_names: Vec<&str> = refs.iter().map(|r| r.name.as_str()).collect();
        let _ = writeln!(
            s,
            "\n## Gap to the reference range ({})\n\n`in` = inside [min, max] of the references' same zone; otherwise the distance to the nearer end and the ratio value / that end.\n",
            ref_names.join(", ")
        );
        let metrics = gap_metrics(&all_bands, params);
        let mut zone_names: Vec<&str> = Vec::new();
        for r in reports {
            for z in &r.zones {
                if !zone_names.contains(&z.zone.as_str()) {
                    zone_names.push(&z.zone);
                }
            }
        }
        for zn in zone_names {
            let _ = writeln!(s, "### Zone `{zn}`\n");
            let names: Vec<&str> = ours.iter().map(|r| r.name.as_str()).collect();
            let _ = writeln!(
                s,
                "| Metric | Ref range | {} |\n|---|---{}|",
                names.join(" | "),
                "|---".repeat(names.len())
            );
            for (label, get) in &metrics {
                let rv: Vec<f64> = refs
                    .iter()
                    .filter_map(|r| r.zones.iter().find(|z| z.zone == zn))
                    .filter_map(get)
                    .filter(|v| v.is_finite())
                    .collect();
                if rv.is_empty() {
                    continue;
                }
                let lo = rv.iter().copied().fold(f64::INFINITY, f64::min);
                let hi = rv.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let mag = lo.abs().max(hi.abs());
                let d = if mag >= 100.0 { 0 } else if mag >= 1.0 { 2 } else { 3 };
                let cells: Vec<String> = ours
                    .iter()
                    .map(|r| {
                        r.zones
                            .iter()
                            .find(|z| z.zone == zn)
                            .and_then(get)
                            .map_or_else(|| "-".into(), |v| format!("{} ({})", fmt(v, d), gap(v, lo, hi)))
                    })
                    .collect();
                let _ = writeln!(s, "| {label} | {}-{} | {} |", fmt(lo, d), fmt(hi, d), cells.join(" | "));
            }
            s.push('\n');
        }
    }
    s
}

/// One `scrib-worldgen measure` invocation.
#[derive(Debug, Clone)]
pub struct MeasureRequest {
    /// Reference guide folders (measured first, the gap section's range).
    pub refs: Vec<std::path::PathBuf>,
    /// Other guide folders.
    pub dirs: Vec<std::path::PathBuf>,
    pub params: MeasureParams,
    /// Base guide folder: every measured field whose samples match it gets the untouched share and `|dh|` (LF-0).
    pub base: Option<std::path::PathBuf>,
    /// `.md` (also writes the `.json` beside it) or `.json`.
    pub out: std::path::PathBuf,
}

/// Measures every folder and writes the report; prints one line per field.
pub fn run_measure(req: &MeasureRequest) -> Result<Vec<FieldReport>> {
    let mut reports = Vec::new();
    let base = req.base.as_deref().map(|d| load_field(d, &req.params)).transpose()?;
    let all = req.refs.iter().map(|d| (d, true)).chain(req.dirs.iter().map(|d| (d, false)));
    for (dir, reference) in all {
        let mut f = load_field(dir, &req.params)?;
        if let Some(b) = &base {
            let same_grid = b.grid.w == f.grid.w
                && b.grid.h == f.grid.h
                && same(b.grid.spacing, f.grid.spacing)
                && same(b.origin_m[0], f.origin_m[0])
                && same(b.origin_m[1], f.origin_m[1]);
            if same_grid {
                f.base = Some(b.grid.data.clone());
            } else {
                println!("{}: samples differ from the base {}, no untouched share", f.name, b.name);
            }
        }
        let r = measure_field(&f, &req.params, reference);
        let zones: Vec<String> = r
            .zones
            .iter()
            .map(|z| format!("{} {:.0} km2", z.zone, z.area_km2))
            .collect();
        println!(
            "{}: {}x{} @ {} m ({}), summit {:?}, {}; {:.1} s",
            r.name,
            r.dims[0],
            r.dims[1],
            r.spacing_m,
            r.resample,
            r.summit_m.map(|m| [m[0].round(), m[1].round()]),
            zones.join(", "),
            r.seconds
        );
        reports.push(r);
    }
    if let Some(parent) = req.out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&reports)?;
    if req.out.extension().is_some_and(|e| e == "json") {
        std::fs::write(&req.out, json)?;
    } else {
        std::fs::write(&req.out, markdown(&reports, &req.params))?;
        std::fs::write(req.out.with_extension("json"), json)?;
    }
    Ok(reports)
}
