//! The 32 m cover cells of one worldspace and their climate weights.

use glam::{DVec2, IVec2};
use sf_weather::{ClimateIx, ClimateRaster, SurfaceRates};

use crate::{SurfaceTuning, Worldspace};

/// Cover cell size, meters. Cell `(x, z)` spans world X, Z `[x, x + 1) · CELL_M` ×
/// `[z, z + 1) · CELL_M` (world-anchored, independent of any floating origin).
pub const CELL_M: f64 = 32.0;

/// Half-width of the snow line's blend, meters.
pub const SNOW_LINE_BLEND_M: f32 = 50.0;

/// The cell holding world X, Z `p`.
#[allow(clippy::cast_possible_truncation)]
pub fn cell_of(p: DVec2) -> IVec2 {
    (p / CELL_M).floor().as_ivec2()
}

/// One climate of a cell's site.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SiteEntry {
    pub climate: ClimateIx,
    /// Border-blend weight at the cell centre.
    pub weight: f32,
    /// Snow altitude factor (`smoothstep(snow_line − 50 m, snow_line + 50 m, height)`; 1 without
    /// a snow line or a height).
    pub snow: f32,
}

/// A cell's climates, inline (border blends give one or two).
#[derive(Clone, Copy, Debug)]
struct CellSite {
    e: [SiteEntry; 2],
    n: u8,
}

/// The cover cells over a worldspace's climate raster, with each cell's climate weights at its
/// centre ([`ClimateRaster::site_at`]: the weather map's border blend) and per climate the
/// cells it touches. Static per raster: build once per worldspace.
pub struct CellGrid {
    worldspace: Worldspace,
    /// First cell and size, cells.
    min: IVec2,
    size: IVec2,
    sites: Vec<CellSite>,
    /// Per [`ClimateIx`]: the cells (indices into `sites`) where it has weight.
    by_climate: Vec<Vec<u32>>,
}

impl CellGrid {
    /// The cells covering `raster`'s bounds.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn new(worldspace: Worldspace, raster: &ClimateRaster) -> Self {
        let (lo, hi) = raster.bounds();
        let min = cell_of(lo);
        let max = (hi / CELL_M).ceil().as_ivec2();
        let size = (max - min).max(IVec2::ONE);
        let none = SiteEntry { climate: ClimateIx(0), weight: 0.0, snow: 1.0 };
        let mut sites = Vec::with_capacity((size.x as usize) * (size.y as usize));
        let mut by_climate: Vec<Vec<u32>> = Vec::new();
        for z in 0..size.y {
            for x in 0..size.x {
                let cell = min + IVec2::new(x, z);
                let centre = (cell.as_dvec2() + 0.5) * CELL_M;
                let mut entry = CellSite { e: [none; 2], n: 0 };
                for (c, w) in raster.site_at(centre) {
                    if w <= 0.0 || entry.n >= 2 {
                        continue;
                    }
                    entry.e[usize::from(entry.n)] = SiteEntry { climate: c, weight: w, snow: 1.0 };
                    entry.n += 1;
                    let ci = c.0 as usize;
                    if by_climate.len() <= ci {
                        by_climate.resize(ci + 1, Vec::new());
                    }
                    by_climate[ci].push(u32::try_from(sites.len()).expect("fewer than 2^32 cells"));
                }
                sites.push(entry);
            }
        }
        Self { worldspace, min, size, sites, by_climate }
    }

    /// Sets each cell's snow altitude factor from its climates' snow lines (meters; `None`: snow
    /// at any height) and the cell's height (meters; `None`: unknown, factor 1).
    pub fn set_altitude(&mut self, snow_line: impl Fn(ClimateIx) -> Option<f32>, height: impl Fn(IVec2) -> Option<f32>) {
        for ix in 0..self.sites.len() {
            let h = height(self.cell(ix));
            let site = &mut self.sites[ix];
            for e in &mut site.e[..usize::from(site.n)] {
                e.snow = match (snow_line(e.climate), h) {
                    (Some(line), Some(h)) => smoothstep(line - SNOW_LINE_BLEND_M, line + SNOW_LINE_BLEND_M, h),
                    _ => 1.0,
                };
            }
        }
    }

    pub fn worldspace(&self) -> &Worldspace {
        &self.worldspace
    }

    /// Number of cells.
    pub fn len(&self) -> usize {
        self.sites.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sites.is_empty()
    }

    /// First cell and the grid's size in cells.
    pub fn extent(&self) -> (IVec2, IVec2) {
        (self.min, self.size)
    }

    /// The index of `cell`, if inside.
    #[allow(clippy::cast_sign_loss)]
    pub fn index(&self, cell: IVec2) -> Option<usize> {
        let l = cell - self.min;
        (l.x >= 0 && l.y >= 0 && l.x < self.size.x && l.y < self.size.y).then(|| (l.x + l.y * self.size.x) as usize)
    }

    /// The cell at index `ix`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn cell(&self, ix: usize) -> IVec2 {
        let ix = ix as i32;
        self.min + IVec2::new(ix % self.size.x, ix / self.size.x)
    }

    /// The climates of cell `ix` with their weights and snow factors.
    pub fn site(&self, ix: usize) -> &[SiteEntry] {
        let s = &self.sites[ix];
        &s.e[..usize::from(s.n)]
    }

    /// The cells (indices) where climate `c` has weight.
    pub fn cells_of(&self, c: ClimateIx) -> &[u32] {
        self.by_climate.get(c.0 as usize).map_or(&[], Vec::as_slice)
    }
}

/// A cell's blended rates per game hour: snowfall (× altitude factor), ash fall, rain (the
/// wetting rate, which washes cover off) and drying.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CellRates {
    pub snow: f32,
    pub ash: f32,
    pub rain: f32,
    pub drying: f32,
}

/// A cell's blended rates over its site.
pub(crate) fn blend(site: &[SiteEntry], r: &[SurfaceRates], t: &SurfaceTuning) -> CellRates {
    let mut b = SurfaceRates::default();
    let mut snow = 0.0;
    for e in site {
        let Some(rc) = r.get(e.climate.0 as usize) else { continue };
        snow += rc.snow * e.weight * e.snow;
        b.ash += rc.ash * e.weight;
        b.wet += rc.wet * e.weight;
        b.drying += rc.drying * e.weight;
    }
    CellRates { snow, ash: b.ash, rain: b.wet, drying: t.drying(&b) }
}

/// GLSL `smoothstep`.
pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
