//! Climate stage, part (a) + (b): air temperature and an upwind moisture-budget precipitation sweep.
//!
//! **Temperature** is a linear lapse from the sea-level temperature plus an optional latitude term:
//! `T = sea_temperature_c - lapse_k_per_km * vertical_scale * max(h, 0) / 1000 + latitude_k_per_km * north_km`, with
//! `north_km` the sample's distance north of the grid's middle row.
//!
//! **Precipitation** follows the mapgen4 family of moisture-budget sweeps (conservative, temperature-limited
//! capacity): the air carries a moisture flux `q` (mm/yr km) across the grid along the dominant wind axis, one line
//! (a column or a row across that axis) at a time. Each cell reads the upwind flux from the previous line (a two-tap
//! linear interpolation when the wind is oblique), takes up moisture from the sea (and a little from land), rains out
//! what exceeds the temperature-limited capacity over a fallout length, and drizzles a small background fraction.
//! Windward slopes are wet and the lee is a rain shadow with no extra rule. For a wind along a grid axis the sweep
//! conserves mass exactly: inflow + source = rain + outflow, up to rounding.
//!
//! Cells of one line are independent, so a line is processed in parallel (rayon, chunked) from the finished previous
//! line only: the result is bit-identical on any thread count. The twelve values are tunables declared with unit,
//! range and source ([`declared_defaults`]); a params file gives them as a `[params]` table of
//! `{ v, unit, min, max, source }` entries, as the height-curve params do.

use std::collections::BTreeMap;

use rayon::prelude::*;
use serde::Deserialize;

use crate::filter::Declared;
use crate::grid::Grid;
use crate::stratovolcano::Param;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_climate.rs"]
mod tests;

/// Every value the params file must declare.
pub const PARAM_NAMES: &[&str] = &[
    "lapse_k_per_km",
    "sea_temperature_c",
    "vertical_scale",
    "latitude_k_per_km",
    "wind_toward_deg",
    "capacity",
    "capacity_per_k",
    "evaporation_km",
    "land_recycling",
    "fallout_km",
    "background_km",
    "inflow_saturation",
];

/// Wind components below this are snapped to zero, so the four axis winds have exactly zero side shift.
// tunables: const (maths: float tolerance of sin/cos at multiples of 90 deg, about 1e-16)
const AXIS_SNAP: f64 = 1e-12;

/// The declared values with their defaults.
#[allow(clippy::too_many_lines)] // one entry per declared value
pub fn declared_defaults() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared {
        name,
        v,
        unit,
        min,
        max,
        source: source.into(),
        help,
    };
    vec![
        d(
            "lapse_k_per_km",
            6.5,
            "K/km",
            4.0,
            9.8,
            "Industry(standard atmosphere lapse rate)",
            "temperature drop per km of height",
        ),
        d(
            "sea_temperature_c",
            18.0,
            "C",
            -5.0,
            30.0,
            "Guess",
            "air temperature at sea level",
        ),
        d(
            "vertical_scale",
            1.0,
            "ratio",
            0.5,
            4.0,
            "Guess",
            "heights count this many times for the lapse (exaggerates a small world's altitude bands)",
        ),
        d(
            "latitude_k_per_km",
            0.0,
            "K/km",
            -0.05,
            0.05,
            "Guess",
            "temperature change per km north of the grid's middle row",
        ),
        d(
            "wind_toward_deg",
            90.0,
            "deg",
            0.0,
            360.0,
            "Guess",
            "direction the wind blows toward, clockwise from north (90 = toward the east)",
        ),
        d(
            "capacity",
            150_000.0,
            "mm/yr km",
            5e4,
            5e5,
            "Guess",
            "moisture flux the air holds at sea temperature",
        ),
        d(
            "capacity_per_k",
            0.07,
            "1/K",
            0.04,
            0.08,
            "Industry(Clausius-Clapeyron, about 7 % per K)",
            "relative change of the capacity per kelvin",
        ),
        d(
            "evaporation_km",
            60.0,
            "km",
            20.0,
            200.0,
            "Guess",
            "length over which the sea refills the air toward its capacity",
        ),
        d(
            "land_recycling",
            0.15,
            "ratio",
            0.0,
            0.5,
            "Guess",
            "land evaporation as a fraction of the sea's",
        ),
        d(
            "fallout_km",
            6.0,
            "km",
            2.0,
            20.0,
            "Industry(Smith-Barstad fallout time 1000 s at 6-10 m/s)",
            "length over which moisture above capacity rains out",
        ),
        d(
            "background_km",
            400.0,
            "km",
            100.0,
            1000.0,
            "Guess",
            "length over which a background drizzle removes the carried moisture",
        ),
        d(
            "inflow_saturation",
            1.0,
            "ratio",
            0.0,
            1.0,
            "Industry(mapgen4 map.ts: wind enters the map at humidity 1)",
            "moisture of the air entering the grid, as a fraction of the capacity",
        ),
    ]
}

/// Climate values (see [`declared_defaults`] for units and meaning).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Climate {
    pub lapse_k_per_km: f64,
    pub sea_temperature_c: f64,
    pub vertical_scale: f64,
    pub latitude_k_per_km: f64,
    pub wind_toward_deg: f64,
    pub capacity: f64,
    pub capacity_per_k: f64,
    pub evaporation_km: f64,
    pub land_recycling: f64,
    pub fallout_km: f64,
    pub background_km: f64,
    pub inflow_saturation: f64,
}

impl Default for Climate {
    fn default() -> Self {
        Self::from_declared(&declared_defaults()).expect("defaults are valid")
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParamsFile {
    params: BTreeMap<String, Param>,
}

impl Climate {
    /// Builds the climate from declared values (ranges checked).
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("climate: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        Ok(Self {
            lapse_k_per_km: get("lapse_k_per_km")?,
            sea_temperature_c: get("sea_temperature_c")?,
            vertical_scale: get("vertical_scale")?,
            latitude_k_per_km: get("latitude_k_per_km")?,
            wind_toward_deg: get("wind_toward_deg")?,
            capacity: get("capacity")?,
            capacity_per_k: get("capacity_per_k")?,
            evaporation_km: get("evaporation_km")?,
            land_recycling: get("land_recycling")?,
            fallout_km: get("fallout_km")?,
            background_km: get("background_km")?,
            inflow_saturation: get("inflow_saturation")?,
        })
    }

    /// Parses a params file (TOML `[params]`): every name declared, no unknown one, units as declared, sources
    /// valid, values in their ranges.
    pub fn parse_toml(text: &str) -> Result<Self> {
        let file: ParamsFile = toml::from_str(text)?;
        for name in file.params.keys() {
            if !PARAM_NAMES.contains(&name.as_str()) {
                return Err(Error::Invalid(format!("climate: unknown param {name}")));
            }
        }
        let mut declared = declared_defaults();
        for d in &mut declared {
            let p = file
                .params
                .get(d.name)
                .ok_or_else(|| Error::Invalid(format!("climate: param {} missing", d.name)))?;
            if p.unit != d.unit {
                return Err(Error::Invalid(format!(
                    "climate: {} unit {:?}, expected {:?}",
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
}

/// Air temperature (C) per sample.
pub fn temperature(heights: &Grid, c: &Climate) -> Grid {
    let (w, h) = (heights.w, heights.h);
    let mid = (h as f64 - 1.0) * 0.5;
    let km = heights.spacing / 1000.0;
    let mut data = Vec::with_capacity(heights.data.len());
    for (i, &z) in heights.data.iter().enumerate() {
        let north_km = (mid - (i / w.max(1)) as f64) * km;
        data.push(
            c.sea_temperature_c - c.lapse_k_per_km * c.vertical_scale * z.max(0.0) / 1000.0
                + c.latitude_k_per_km * north_km,
        );
    }
    Grid {
        w,
        h,
        spacing: heights.spacing,
        data,
    }
}

/// Moisture flux sums of one sweep, in mm/yr km (exact for a wind along a grid axis; with an oblique wind
/// `inflow`/`outflow` count only the first and last line, not the side edges).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Budget {
    pub inflow: f64,
    pub source: f64,
    pub rain: f64,
    pub outflow: f64,
}

/// Result of [`precipitation`].
#[derive(Debug, Clone, PartialEq)]
pub struct Rain {
    /// mm/yr
    pub p: Grid,
    pub budget: Budget,
}

/// Wind unit vector `(east, north)` of a "toward" bearing, axis components exact.
fn wind(deg: f64) -> (f64, f64) {
    let (mut wx, mut wy) = deg.to_radians().sin_cos();
    if wx.abs() < AXIS_SNAP {
        wx = 0.0;
        wy = wy.signum();
    }
    if wy.abs() < AXIS_SNAP {
        wy = 0.0;
        wx = wx.signum();
    }
    (wx, wy)
}

/// Sweep geometry: lines in wind order, cells along a line, side shift per line (in cell index units).
struct Sweep {
    w: usize,
    h: usize,
    /// Lines are rows (else columns).
    rows: bool,
    /// The first line in wind order is the last row/column (else the first).
    reverse: bool,
    lines: usize,
    len: usize,
    t: f64,
}

impl Sweep {
    fn new(w: usize, h: usize, wx: f64, wy: f64) -> Self {
        if wy.abs() > wx.abs() {
            // Rows; wind toward north (wy > 0) starts at the south row. East = +column.
            Self {
                w,
                h,
                rows: true,
                reverse: wy > 0.0,
                lines: h,
                len: w,
                t: wx / wy.abs(),
            }
        } else {
            // Columns; wind toward west (wx < 0) starts at the east column. North = -row.
            Self {
                w,
                h,
                rows: false,
                reverse: wx < 0.0,
                lines: w,
                len: h,
                t: -wy / wx.abs(),
            }
        }
    }

    /// Grid index of cell `j` of line `k` (wind order).
    fn index(&self, k: usize, j: usize) -> usize {
        if self.rows {
            let row = if self.reverse { self.h - 1 - k } else { k };
            row * self.w + j
        } else {
            let col = if self.reverse { self.w - 1 - k } else { k };
            j * self.w + col
        }
    }
}

/// Upwind flux of cell `j`: the previous line at `j - t`, linearly interpolated; a tap outside the line takes `q_in`.
fn tap(prev: &[f64], j: usize, t: f64, q_in: f64) -> f64 {
    if t == 0.0 {
        return prev[j];
    }
    let p = j as f64 - t;
    let i0 = p.floor();
    let f = p - i0;
    let at = |i: f64| {
        if i >= 0.0 && i < prev.len() as f64 {
            prev[i as usize]
        } else {
            q_in
        }
    };
    if f == 0.0 {
        at(i0)
    } else {
        at(i0) * (1.0 - f) + at(i0 + 1.0) * f
    }
}

/// Precipitation (mm/yr) by an upwind moisture-budget sweep. `heights` (m above sea level, sea = `<= 0`) and
/// `temperature` (C, from [`temperature`]) must have the same size.
pub fn precipitation(heights: &Grid, temperature: &Grid, c: &Climate) -> Result<Rain> {
    let n = heights.w * heights.h;
    if heights.w != temperature.w
        || heights.h != temperature.h
        || heights.data.len() != n
        || temperature.data.len() != n
    {
        return Err(Error::Invalid(format!(
            "climate: heights {}x{} ({} samples) and temperature {}x{} ({} samples) differ",
            heights.w,
            heights.h,
            heights.data.len(),
            temperature.w,
            temperature.h,
            temperature.data.len()
        )));
    }
    let (wx, wy) = wind(c.wind_toward_deg);
    let sw = Sweep::new(heights.w, heights.h, wx, wy);
    let l_km = heights.spacing / 1000.0 / wx.abs().max(wy.abs());
    let fe = -(-l_km / c.evaporation_km).exp_m1();
    let ff = -(-l_km / c.fallout_km).exp_m1();
    let fb = -(-l_km / c.background_km).exp_m1();
    let q_in = c.inflow_saturation * c.capacity;
    let mut p = vec![0.0; n];
    let mut budget = Budget {
        inflow: 0.0,
        source: 0.0,
        rain: 0.0,
        outflow: 0.0,
    };
    let len = sw.len;
    if len == 0 || sw.lines == 0 {
        return Ok(Rain {
            p: Grid {
                w: heights.w,
                h: heights.h,
                spacing: heights.spacing,
                data: p,
            },
            budget,
        });
    }
    let chunk = len.div_ceil(rayon::current_num_threads()).max(1);
    let mut prev = vec![q_in; len];
    let (mut hl, mut tl) = (vec![0.0; len], vec![0.0; len]);
    // Per cell: [q_out, source, rain].
    let mut cur = vec![[0.0f64; 3]; len];
    for _ in 0..len {
        budget.inflow += q_in;
    }
    for k in 0..sw.lines {
        for j in 0..len {
            let i = sw.index(k, j);
            hl[j] = heights.data[i];
            tl[j] = temperature.data[i];
        }
        let first = k == 0;
        let prev_ref = &prev;
        cur.par_chunks_mut(chunk)
            .zip(hl.par_chunks(chunk))
            .zip(tl.par_chunks(chunk))
            .enumerate()
            .for_each(|(ci, ((out, hs), ts))| {
                for (o, (m, (&z, &t))) in out.iter_mut().zip(hs.iter().zip(ts)).enumerate() {
                    let j = ci * chunk + o;
                    let q_up = if first {
                        q_in
                    } else {
                        tap(prev_ref, j, sw.t, q_in)
                    };
                    let cap = c.capacity * (c.capacity_per_k * (t - c.sea_temperature_c)).exp();
                    let kf = if z <= 0.0 { 1.0 } else { c.land_recycling };
                    let s = kf * (cap - q_up).max(0.0) * fe;
                    let q1 = q_up + s;
                    let rc = (q1 - cap).max(0.0) * ff;
                    let rb = (q1 - rc) * fb;
                    *m = [q1 - rc - rb, s, rc + rb];
                }
            });
        let (mut line_s, mut line_r) = (0.0, 0.0);
        for (j, cell) in cur.iter().enumerate() {
            line_s += cell[1];
            line_r += cell[2];
            p[sw.index(k, j)] = cell[2] / l_km;
            prev[j] = cell[0];
        }
        budget.source += line_s;
        budget.rain += line_r;
    }
    budget.outflow = prev.iter().sum();
    Ok(Rain {
        p: Grid {
            w: heights.w,
            h: heights.h,
            spacing: heights.spacing,
            data: p,
        },
        budget,
    })
}
