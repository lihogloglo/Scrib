//! Scatter v2 fields and hat suitability (spec `worldgen-scatter.md` "Design" 1-2; slice WG-E2b).
//!
//! - [`Fields`]: what the scatter passes read at a point (climate, class, terrain shape, distance to water). Two
//!   providers: [`NeutralFields`] (one sample everywhere, for tests and for runs without a climate product) and
//!   [`GridFields`] (row-major rasters in the guide frame, bilinear; the class is the nearest sample and the aspect
//!   is interpolated as a unit vector, so 359 deg and 1 deg blend to 0 deg, not 180). [`GridFields::from_products`]
//!   builds it from a guide and its climate product (slice WG-E2i.1; terrain rasters in [`terrain_shape`]).
//! - [`hat`]: a suitability hat `[e_min, i_min, i_max, e_max]`, 0 outside, 1 inside, linear between (Gain 2017 via
//!   Cordonnier 2017).
//! - [`Suitability`]: one species' hats combined by minimum (Liebig's law; Ecormier-Nocca 2021, Cordonnier 2017),
//!   gated by its class list. Species mix along a gradient through overlapping hats, with no extra rule.
//!
//! Frame: x east, y north, metres; raster row 0 is the north edge, sample (0, 0) sits at `origin_m` (the guide
//! convention of `geology.rs` and `cliffs.rs`). Nothing here names a game.

use rayon::prelude::*;

use crate::climate_map::ClimateMap;
use crate::guide::Guide;
use crate::measure::distance_transform;
use crate::scatter_species::{Hat, Species};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_scatter_fields.rs"]
mod tests;

/// What the fields answer at one point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldSample {
    /// Mean annual temperature, C.
    pub temperature_c: f64,
    /// Annual precipitation, mm/yr.
    pub precipitation_mm: f64,
    /// Soil moisture, 0..1.
    pub moisture: f64,
    /// Index into [`Fields::class_names`]; `None` where no class is known.
    pub class: Option<u8>,
    /// Ground slope, degrees from horizontal.
    pub slope_deg: f64,
    /// Downslope direction, degrees clockwise from north, in `[0, 360)`.
    pub aspect_deg: f64,
    /// Height above sea level, m.
    pub altitude_m: f64,
    /// Plan curvature, 1/m (positive = convex).
    pub curvature: f64,
    /// Distance to the nearest inland water or sea, m.
    pub water_distance_m: f64,
}

/// The fields a scatter runs on (spec "Design" 1).
pub trait Fields: Sync {
    /// The sample at (x east, y north), metres.
    fn sample(&self, x: f64, y: f64) -> FieldSample;
    /// Names of the class ids [`FieldSample::class`] indexes.
    fn class_names(&self) -> &[String];
}

/// One sample everywhere.
#[derive(Debug, Clone, PartialEq)]
pub struct NeutralFields {
    pub sample: FieldSample,
    pub class_names: Vec<String>,
}

impl Fields for NeutralFields {
    fn sample(&self, _x: f64, _y: f64) -> FieldSample {
        self.sample
    }

    fn class_names(&self) -> &[String] {
        &self.class_names
    }
}

/// Rasters in the guide frame, one value per sample, row-major, row 0 north.
#[derive(Debug, Clone, PartialEq)]
pub struct GridFields {
    pub w: usize,
    pub h: usize,
    /// Sample spacing, metres.
    pub spacing_m: f64,
    /// Guide-frame metres of sample (0, 0) (the north-west sample).
    pub origin_m: [f64; 2],
    pub temperature_c: Vec<f32>,
    pub precipitation_mm: Vec<f32>,
    pub moisture: Vec<f32>,
    /// Index into `class_names`, per sample; [`u8::MAX`] = no class.
    pub classes: Vec<u8>,
    pub class_names: Vec<String>,
    pub slope_deg: Vec<f32>,
    pub aspect_deg: Vec<f32>,
    pub altitude_m: Vec<f32>,
    pub curvature: Vec<f32>,
    pub water_distance_m: Vec<f32>,
}

/// The class id of a sample with no class.
// tunables: const (format: the no-class sentinel of the class raster)
pub const NO_CLASS: u8 = u8::MAX;

impl GridFields {
    /// A `w` x `h` grid with every sample set to `fill`.
    #[must_use]
    pub fn filled(w: usize, h: usize, spacing_m: f64, origin_m: [f64; 2], fill: &FieldSample, class_names: Vec<String>) -> Self {
        let n = w * h;
        // Rasters are f32 (the climate product's type).
        #[allow(clippy::cast_possible_truncation)]
        let v = |x: f64| vec![x as f32; n];
        Self {
            w,
            h,
            spacing_m,
            origin_m,
            temperature_c: v(fill.temperature_c),
            precipitation_mm: v(fill.precipitation_mm),
            moisture: v(fill.moisture),
            classes: vec![fill.class.unwrap_or(NO_CLASS); n],
            class_names,
            slope_deg: v(fill.slope_deg),
            aspect_deg: v(fill.aspect_deg),
            altitude_m: v(fill.altitude_m),
            curvature: v(fill.curvature),
            water_distance_m: v(fill.water_distance_m),
        }
    }

    /// The fields of `guide` and its climate product `climate` (same dims, spacing and origin): temperature,
    /// precipitation, moisture and class from the product; slope, aspect and plan curvature from the guide heights
    /// ([`terrain_shape`]); altitude = height - sea level; water distance = Euclidean distance to the nearest sea
    /// sample (height <= sea level), [`NO_WATER_M`] on a map with no sea. Inland water is not in this raster: the
    /// scatter takes the minimum with its exact lake and river distance (`scatter::WaterMask`, WG-E2g).
    pub fn from_products(guide: &Guide, climate: &ClimateMap) -> Result<Self> {
        // The product is written from this guide's own values, so they compare equal bit for bit.
        #[allow(clippy::float_cmp)]
        let other = (guide.w, guide.h) != (climate.w, climate.h)
            || guide.spacing_m != climate.spacing_m
            || guide.origin_m != climate.origin_m;
        if other {
            return Err(Error::Invalid(format!(
                "scatter fields: guide {} x {} at {} m from {:?} and climate {} x {} at {} m from {:?} differ",
                guide.w,
                guide.h,
                guide.spacing_m,
                guide.origin_m,
                climate.w,
                climate.h,
                climate.spacing_m,
                climate.origin_m
            )));
        }
        let (w, h) = (guide.w, guide.h);
        let z: Vec<f64> = guide.heights.iter().map(|&x| f64::from(x)).collect();
        let t = terrain_shape(&z, w, h, guide.spacing_m);
        let sea: Vec<bool> = z.iter().map(|&x| x <= guide.sea_level_m).collect();
        // Rasters are f32 (the climate product's type).
        #[allow(clippy::cast_possible_truncation)]
        let f32s = |v: &[f64]| v.iter().map(|&x| x as f32).collect::<Vec<f32>>();
        #[allow(clippy::cast_possible_truncation)]
        let water_distance_m = distance_transform(&sea, w, h)
            .iter()
            .map(|&d| if d.is_finite() { (d * guide.spacing_m) as f32 } else { NO_WATER_M })
            .collect();
        #[allow(clippy::cast_possible_truncation)]
        let altitude_m = z.iter().map(|&x| (x - guide.sea_level_m) as f32).collect();
        let fields = Self {
            w,
            h,
            spacing_m: guide.spacing_m,
            origin_m: guide.origin_m,
            temperature_c: climate.temperature.clone(),
            precipitation_mm: climate.precipitation.clone(),
            moisture: climate.moisture.clone(),
            classes: climate.classes.clone(),
            class_names: climate.class_names.clone(),
            slope_deg: f32s(&t.slope_deg),
            aspect_deg: f32s(&t.aspect_deg),
            altitude_m,
            curvature: f32s(&t.curvature),
            water_distance_m,
        };
        fields.check()?;
        Ok(fields)
    }

    /// Checks the grid: at least one sample, a positive finite spacing, every raster `w * h` long, class ids in
    /// range (or [`NO_CLASS`]), at most 255 class names.
    pub fn check(&self) -> Result<()> {
        let n = self.w * self.h;
        if n == 0 || !(self.spacing_m.is_finite() && self.spacing_m > 0.0) {
            return Err(Error::Invalid(format!(
                "scatter fields: grid {} x {} at spacing {} m is empty or has a bad spacing",
                self.w, self.h, self.spacing_m
            )));
        }
        let rasters: [(&str, usize); 9] = [
            ("temperature_c", self.temperature_c.len()),
            ("precipitation_mm", self.precipitation_mm.len()),
            ("moisture", self.moisture.len()),
            ("classes", self.classes.len()),
            ("slope_deg", self.slope_deg.len()),
            ("aspect_deg", self.aspect_deg.len()),
            ("altitude_m", self.altitude_m.len()),
            ("curvature", self.curvature.len()),
            ("water_distance_m", self.water_distance_m.len()),
        ];
        for (name, len) in rasters {
            if len != n {
                return Err(Error::Invalid(format!(
                    "scatter fields: raster {name} has {len} samples, the grid {n}"
                )));
            }
        }
        if self.class_names.len() > usize::from(NO_CLASS) {
            return Err(Error::Invalid(format!(
                "scatter fields: {} class names, at most {}",
                self.class_names.len(),
                NO_CLASS
            )));
        }
        if let Some(&c) = self
            .classes
            .iter()
            .find(|&&c| c != NO_CLASS && usize::from(c) >= self.class_names.len())
        {
            return Err(Error::Invalid(format!(
                "scatter fields: class id {c} with {} class names",
                self.class_names.len()
            )));
        }
        Ok(())
    }

    /// Fractional (col, row) of a guide-frame point, clamped to the grid.
    fn cell(&self, x: f64, y: f64) -> (f64, f64) {
        let col = ((x - self.origin_m[0]) / self.spacing_m).clamp(0.0, (self.w - 1) as f64);
        let row = ((self.origin_m[1] - y) / self.spacing_m).clamp(0.0, (self.h - 1) as f64);
        (col, row)
    }

    /// Bilinear weights of (col, row): the four sample indices and their weights.
    fn corners(&self, col: f64, row: f64) -> [(usize, f64); 4] {
        // Clamped to 0..w-1 and 0..h-1 by `cell`.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (c0, r0) = (col.floor() as usize, row.floor() as usize);
        let (c1, r1) = ((c0 + 1).min(self.w - 1), (r0 + 1).min(self.h - 1));
        let (tx, ty) = (col - c0 as f64, row - r0 as f64);
        let w = self.w;
        [
            (r0 * w + c0, (1.0 - tx) * (1.0 - ty)),
            (r0 * w + c1, tx * (1.0 - ty)),
            (r1 * w + c0, (1.0 - tx) * ty),
            (r1 * w + c1, tx * ty),
        ]
    }
}

/// Water distance of a map with no water sample, m (finite, so a bilinear blend with a zero weight stays a number).
// tunables: const (format: the "no water on this map" sentinel of the distance raster)
pub const NO_WATER_M: f32 = f32::MAX;

/// Terrain shape per sample (see [`terrain_shape`]).
#[derive(Debug, Clone, PartialEq)]
pub struct TerrainShape {
    /// Degrees from horizontal.
    pub slope_deg: Vec<f64>,
    /// Downslope direction, degrees clockwise from north, in `[0, 360)`; 0 where flat.
    pub aspect_deg: Vec<f64>,
    /// Plan (contour) curvature, 1/m, positive = convex (spurs, peaks), negative = concave (hollows); 0 where flat.
    pub curvature: Vec<f64>,
}

/// Slope, aspect and plan curvature of heights `z` (`w` x `h`, row-major, row 0 north, spacing `l` m), edges clamped.
/// Slope and aspect from Horn's 3x3 gradient (Horn 1981; the GDAL and `ArcGIS` default). Plan curvature from the
/// Evans-Young second-order partials `p, q, r, s, t` (central differences) as
/// `-(q^2 r - 2 p q s + p^2 t) / (p^2 + q^2)^(3/2)` (Florinsky 2016, "Digital Terrain Analysis", ch. 2), with y north.
#[must_use]
pub fn terrain_shape(z: &[f64], w: usize, h: usize, l: f64) -> TerrainShape {
    // tunables: const (maths: squared gradient below which a sample is flat and has no aspect or contour)
    const FLAT: f64 = 1e-18;
    let at = |x: isize, y: isize| {
        let xc = x.clamp(0, w.cast_signed() - 1).cast_unsigned();
        let yc = y.clamp(0, h.cast_signed() - 1).cast_unsigned();
        z[yc * w + xc]
    };
    let mut rows: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); w * h];
    rows.par_chunks_mut(w).enumerate().for_each(|(y, line)| {
        let yi = y.cast_signed();
        for (x, o) in line.iter_mut().enumerate() {
            let xi = x.cast_signed();
            // Offsets: dx east, dy south (row order); north is dy = -1.
            let s = |dx: isize, dy: isize| at(xi + dx, yi + dy);
            let de = ((s(1, -1) + 2.0 * s(1, 0) + s(1, 1)) - (s(-1, -1) + 2.0 * s(-1, 0) + s(-1, 1))) / (8.0 * l);
            let dn = ((s(-1, -1) + 2.0 * s(0, -1) + s(1, -1)) - (s(-1, 1) + 2.0 * s(0, 1) + s(1, 1))) / (8.0 * l);
            let slope = de.hypot(dn).atan().to_degrees();
            let flat = de * de + dn * dn < FLAT;
            let aspect = if flat { 0.0 } else { (-de).atan2(-dn).to_degrees().rem_euclid(360.0) };
            let p = (s(1, 0) - s(-1, 0)) / (2.0 * l);
            let q = (s(0, -1) - s(0, 1)) / (2.0 * l);
            let r = (s(1, 0) - 2.0 * s(0, 0) + s(-1, 0)) / (l * l);
            let t = (s(0, -1) - 2.0 * s(0, 0) + s(0, 1)) / (l * l);
            let sxy = (s(1, -1) - s(-1, -1) - s(1, 1) + s(-1, 1)) / (4.0 * l * l);
            let g2 = p * p + q * q;
            let curv = if g2 < FLAT { 0.0 } else { -(q * q * r - 2.0 * p * q * sxy + p * p * t) / g2.powf(1.5) };
            *o = (slope, aspect, curv);
        }
    });
    TerrainShape {
        slope_deg: rows.iter().map(|v| v.0).collect(),
        aspect_deg: rows.iter().map(|v| v.1).collect(),
        curvature: rows.iter().map(|v| v.2).collect(),
    }
}

/// Bilinear blend of one raster at precomputed corners.
fn blend(v: &[f32], k: &[(usize, f64); 4]) -> f64 {
    k.iter().map(|&(i, wt)| f64::from(v[i]) * wt).sum()
}

impl Fields for GridFields {
    fn sample(&self, x: f64, y: f64) -> FieldSample {
        let (col, row) = self.cell(x, y);
        let k = self.corners(col, row);
        // Aspect as a unit vector, so the blend never crosses the 0/360 seam the long way round.
        let (mut sx, mut cy) = (0.0, 0.0);
        for &(i, wt) in &k {
            let a = f64::from(self.aspect_deg[i]).to_radians();
            sx += a.sin() * wt;
            cy += a.cos() * wt;
        }
        let aspect = sx.atan2(cy).to_degrees().rem_euclid(360.0);
        // Nearest sample: `cell` clamps to the grid, so the rounded indices are in range.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let nearest = (row.round() as usize) * self.w + col.round() as usize;
        let c = self.classes[nearest];
        FieldSample {
            temperature_c: blend(&self.temperature_c, &k),
            precipitation_mm: blend(&self.precipitation_mm, &k),
            moisture: blend(&self.moisture, &k),
            class: (c != NO_CLASS).then_some(c),
            slope_deg: blend(&self.slope_deg, &k),
            aspect_deg: aspect,
            altitude_m: blend(&self.altitude_m, &k),
            curvature: blend(&self.curvature, &k),
            water_distance_m: blend(&self.water_distance_m, &k),
        }
    }

    fn class_names(&self) -> &[String] {
        &self.class_names
    }
}

/// The hat `h` at `x`: 0 outside `[e_min, e_max]` (and for a NaN `x`), 1 inside `[i_min, i_max]`, linear between.
#[must_use]
pub fn hat(h: &Hat, x: f64) -> f64 {
    if x.is_nan() || x < h.e_min || x > h.e_max {
        0.0
    } else if x >= h.i_min && x <= h.i_max {
        1.0
    } else if x < h.i_min {
        // e_min <= x < i_min, so i_min > e_min.
        (x - h.e_min) / (h.i_min - h.e_min)
    } else {
        // i_max < x <= e_max, so e_max > i_max.
        (h.e_max - x) / (h.e_max - h.i_max)
    }
}

/// One species' suitability against one [`Fields`]: its hats, and its class list resolved to the fields' ids.
#[derive(Debug, Clone, PartialEq)]
pub struct Suitability {
    hats: Vec<(Driver, Hat)>,
    /// `None`: any class (the species lists none). Otherwise one flag per class id.
    classes: Option<Vec<bool>>,
}

/// The driver a hat reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Driver {
    Temperature,
    Precipitation,
    Moisture,
    Slope,
    Altitude,
    WaterDistance,
}

impl Driver {
    fn read(self, s: &FieldSample) -> f64 {
        match self {
            Self::Temperature => s.temperature_c,
            Self::Precipitation => s.precipitation_mm,
            Self::Moisture => s.moisture,
            Self::Slope => s.slope_deg,
            Self::Altitude => s.altitude_m,
            Self::WaterDistance => s.water_distance_m,
        }
    }
}

impl Suitability {
    /// Resolves `species` against the class names of `fields`. A listed class the fields don't have matches
    /// nowhere (a world without that biome); a species with no class list lives in any class, and where none is
    /// known.
    #[must_use]
    pub fn new(species: &Species, class_names: &[String]) -> Self {
        let h = &species.hats;
        let hats = [
            (Driver::Temperature, h.temperature_c),
            (Driver::Precipitation, h.precipitation_mm),
            (Driver::Moisture, h.moisture),
            (Driver::Slope, h.slope_deg),
            (Driver::Altitude, h.altitude_m),
            (Driver::WaterDistance, h.water_distance_m),
        ]
        .into_iter()
        .filter_map(|(d, hat)| hat.map(|hat| (d, hat)))
        .collect();
        let classes = (!species.classes.is_empty())
            .then(|| class_names.iter().map(|n| species.classes.contains(n)).collect());
        Self { hats, classes }
    }

    /// The suitability at a sample: the minimum of its hats (1 with none), 0 outside its classes.
    #[must_use]
    pub fn at(&self, s: &FieldSample) -> f64 {
        if let Some(flags) = &self.classes {
            let inside = s.class.is_some_and(|c| flags.get(usize::from(c)).copied().unwrap_or(false));
            if !inside {
                return 0.0;
            }
        }
        self.hats.iter().map(|(d, h)| hat(h, d.read(s))).fold(1.0, f64::min)
    }
}
