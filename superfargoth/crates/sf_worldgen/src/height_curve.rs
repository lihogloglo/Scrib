//! Sea-level-anchored height transfer curve: a hypsometric remap of heights near the shore, like the Curve /
//! Remap nodes of World Machine or Gaea (a monotone transfer function applied per sample).
//!
//! A guide scaled up uniformly (horizontally and vertically) turns every gentle shore into a bluff: a 2-3 m rise
//! within one vertex becomes 20-30 m. The curve flattens the band near sea level and leaves the rest alone. Input
//! `h` is a height in metres relative to the guide's sea level; land (`h >= 0`) and water (`h < 0`) have their own
//! branch, each with a slope at the shore `a` (`0 < a <= 1`) and a knee `K` (metres). With `t = |h| / K`:
//!
//! - `t >= 1`: `f(h) = h` (identity: mountains and the deep sea don't change);
//! - `t < 1`: `|f| = K (a t + 2 (1 - a) t^2 - (1 - a) t^3)`, the sign of `h` kept.
//!
//! This cubic Hermite segment has `f(0) = 0`, `f'(0) = a`, `f(±K) = ±K` and `f'(±K) = 1` (C1 at the knee), and
//! `f' > 0` on `[0, K]` (its largest slope factor is `a + 4 (1 - a) / 3`, at `t = 2/3`). Since
//! `|f| / |h| = a + (1 - a) t (2 - t) <= 1`, the curve never moves a sample away from sea level, and since the sign
//! is kept, the wet/dry class (the coastline) never changes.
//!
//! The four values are tunables declared with unit, range and source ([`declared_defaults`]); a params file gives
//! them as a `[params]` table of `{ v, unit, min, max, source }` entries, as the stratovolcano params do.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::filter::Declared;
use crate::stratovolcano::Param;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_height_curve.rs"]
mod tests;

/// Every value the params file must declare.
pub const PARAM_NAMES: &[&str] = &["land_slope", "land_knee_m", "water_slope", "water_knee_m"];

/// The declared values with their defaults.
pub fn declared_defaults() -> Vec<Declared> {
    let d = |name, v, unit, min, max, help| Declared {
        name,
        v,
        unit,
        min,
        max,
        source: "Guess".into(),
        help,
    };
    vec![
        d("land_slope", 0.1, "ratio", 0.001, 1.0, "slope of the land branch at the shore (1 = unchanged)"),
        d("land_knee_m", 300.0, "m", 1.0, 10_000.0, "land heights at or above this are unchanged"),
        d("water_slope", 0.3, "ratio", 0.001, 1.0, "slope of the water branch at the shore (1 = unchanged)"),
        d("water_knee_m", 300.0, "m", 1.0, 10_000.0, "depths at or below this are unchanged"),
    ]
}

/// One branch of the curve (land or water).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Branch {
    /// Slope at the shore, `0 < slope <= 1`.
    pub slope: f64,
    /// Knee, metres from sea level: identity beyond it.
    pub knee_m: f64,
}

impl Branch {
    fn new(name: &str, slope: f64, knee_m: f64) -> Result<Self> {
        if !(slope.is_finite() && slope > 0.0 && slope <= 1.0) {
            return Err(Error::Invalid(format!("height curve: {name} slope {slope} outside (0, 1]")));
        }
        if !(knee_m.is_finite() && knee_m > 0.0) {
            return Err(Error::Invalid(format!("height curve: {name} knee {knee_m} m must be > 0")));
        }
        Ok(Self { slope, knee_m })
    }

    /// `|f|` for a distance `d >= 0` from sea level.
    #[must_use]
    pub fn magnitude(&self, d: f64) -> f64 {
        if d >= self.knee_m {
            return d;
        }
        let t = d / self.knee_m;
        let b = 1.0 - self.slope;
        self.knee_m * t * (self.slope + t * (2.0 * b - b * t))
    }
}

/// The two branches.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct HeightCurve {
    pub land: Branch,
    pub water: Branch,
}

impl Default for HeightCurve {
    fn default() -> Self {
        Self::from_declared(&declared_defaults()).expect("defaults are valid")
    }
}

/// A params file: `[params]` with every name of [`PARAM_NAMES`].
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParamsFile {
    params: BTreeMap<String, Param>,
}

impl HeightCurve {
    /// Builds the curve from declared values (ranges checked).
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("height curve: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        Ok(Self {
            land: Branch::new("land", get("land_slope")?, get("land_knee_m")?)?,
            water: Branch::new("water", get("water_slope")?, get("water_knee_m")?)?,
        })
    }

    /// Parses a params file (TOML `[params]`): every name declared, no unknown one, units as declared, sources
    /// valid, values in their ranges.
    pub fn parse_toml(text: &str) -> Result<Self> {
        let file: ParamsFile = toml::from_str(text)?;
        for name in file.params.keys() {
            if !PARAM_NAMES.contains(&name.as_str()) {
                return Err(Error::Invalid(format!("height curve: unknown param {name}")));
            }
        }
        let mut declared = declared_defaults();
        for d in &mut declared {
            let p = file
                .params
                .get(d.name)
                .ok_or_else(|| Error::Invalid(format!("height curve: param {} missing", d.name)))?;
            if p.unit != d.unit {
                return Err(Error::Invalid(format!(
                    "height curve: {} unit {:?}, expected {:?}",
                    d.name, p.unit, d.unit
                )));
            }
            crate::recipe::check_source(d.name, &p.source)?;
            d.v = p.v;
            d.min = p.min;
            d.max = p.max;
            d.source.clone_from(&p.source);
        }
        Self::from_declared(&declared)
    }

    /// `f(h)`, `h` in metres relative to sea level.
    #[must_use]
    pub fn apply(&self, h: f64) -> f64 {
        if h >= 0.0 {
            self.land.magnitude(h)
        } else {
            -self.water.magnitude(-h)
        }
    }

    /// `f(h)` in f32 (computed in f64, rounded once). A non-zero sample whose image would round to 0 keeps its
    /// own value, so the sign (the wet/dry class) survives rounding too.
    #[must_use]
    pub fn apply_f32(&self, h: f32) -> f32 {
        let r = self.apply(f64::from(h)) as f32;
        if r == 0.0 && h != 0.0 { h } else { r }
    }
}
