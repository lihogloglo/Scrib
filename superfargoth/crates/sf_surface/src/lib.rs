//! Bevy-free surface weather state (surface-weather.md "Data model" → "CPU state").
//!
//! - **State** ([`SurfaceState`]): wetness and puddle fill per climate ([`ClimateSurface`],
//!   indexed by `sf_weather`'s [`ClimateIx`]); snow and ash cover per 32 m cell
//!   ([`CellSurface`], sparse: only non-zero cells, per worldspace). All in `[0, 1]`.
//! - **Stepping** ([`advance`]): fixed steps of `step_h` game hours (10 game minutes) on a grid
//!   anchored at game hour 0, so how the clock is split into calls never changes the result;
//!   a long jump (rest, wait, load) runs as many steps, capped at `catch_up_h`. Each step reads
//!   the rates at its mid time from a [`RateSource`] ([`WeatherRates`]: the weather blend, a pure
//!   function of seed, overrides and clock). No clock and no RNG here: any peer with the same
//!   inputs reaches the same state bit for bit (net.md rule 3).
//! - **Cells** ([`CellGrid`]): per worldspace, the 32 m cells over the climate raster with
//!   their climate weights at the cell centre ([`ClimateRaster::site_at`], the weather map's
//!   border blend), so cover agrees with the far weather.
//! - **Query** ([`sample`]): the four channels at a world point (gameplay reads it on the host;
//!   footstep variants).
//! - **Save** ([`SurfaceSave`]): quantized to 1/65535, non-zero entries only; stored as a field
//!   of the `sf/weather` save section.
//!
//! Rates are forward Euler on piecewise-constant rates (Clockwork's per-region accumulators,
//! `apps/openmw/mwrender/clockwork/scene/surfaceweather.cpp`, re-expressed per game hour; no
//! code ported). Rain washes snow and ash off at a rate proportional to its wetting rate
//! ([`SurfaceTuning::snow_wash`], [`SurfaceTuning::ash_wash`]: Clockwork's rain-quickened melt
//! and `sWashRate`, linear here).

pub mod cover;
mod grid;
pub mod kickup;
pub mod layer;
mod save;
pub mod trail;
pub mod view;

use std::collections::HashMap;

use glam::{DVec2, IVec2};
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_weather::{ClimateIx, ClimateRaster, PRECIP, SurfaceRates, WeatherBook, WeatherState};

pub use grid::{CELL_M, CellGrid, SNOW_LINE_BLEND_M, SiteEntry, cell_of, smoothstep};
pub use save::{QUANTUM, SavedCell, SavedCells, SurfaceSave};
pub use sf_weather;

/// A worldspace's record id (the inner id of `sf_world::WorldspaceId`; `sf_world` is not a
/// dependency, to keep this core light).
pub type Worldspace = RecordId;

/// The step and decay constants (surface-weather.md "Data model"). Start values from
/// Clockwork's, re-calibrated from the owner's look.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SurfaceTuning {
    /// Fixed step, game hours (1/6: 10 game minutes).
    pub step_h: f64,
    /// Longest catch-up one [`advance`] steps through, game hours; an older state (or a new one)
    /// starts this long before the target.
    pub catch_up_h: f64,
    /// Full wet → dry, game hours, where no blended preset sets a drying rate.
    pub dry_h: f32,
    /// Puddles dry at this multiple of the drying rate.
    pub puddle_dry_mult: f32,
    /// Snow melts at this multiple (1/3: three times slower than wetness).
    pub snow_melt_mult: f32,
    /// Ash clears at this multiple (1/4).
    pub ash_clear_mult: f32,
    /// Puddles rise only while the climate's wetness is at least this.
    pub puddle_from_wet: f32,
    /// Rain washes snow off: snow cleared per unit of wetting rate (`SurfaceRates::wet`, the
    /// rain's intensity), while drying stops. 1/3: a storm wetting at 2/h melts 2/3 per hour,
    /// four times the dry melt (Clockwork's `(1 + 3·rain)` melt factor).
    pub snow_wash: f32,
    /// Rain washes ash off, much faster than snow: ash cleared per unit of wetting rate. 2: a
    /// storm (wetting 2/h) clears full ash in 15 game minutes (Clockwork's `sWashRate`: e-fold
    /// in 4 minutes), 32 times the dry clearing.
    pub ash_wash: f32,
    /// The wetting rate (`SurfaceRates::wet`, per hour) of the heaviest rain: rain intensity 1 for
    /// the puddles' rain rings (W4). 2: a storm.
    pub storm_wet_rate: f32,
}

impl Default for SurfaceTuning {
    fn default() -> Self {
        Self {
            step_h: 1.0 / 6.0,
            catch_up_h: 72.0,
            dry_h: 2.0,
            puddle_dry_mult: 0.5,
            snow_melt_mult: 1.0 / 3.0,
            ash_clear_mult: 0.25,
            puddle_from_wet: 0.7,
            snow_wash: 1.0 / 3.0,
            ash_wash: 2.0,
            storm_wet_rate: 2.0,
        }
    }
}

impl SurfaceTuning {
    /// The drying rate per hour under `rates`: none while water falls, else the blended presets'
    /// rate, else `1 / dry_h`.
    pub fn drying(&self, rates: &SurfaceRates) -> f32 {
        if rates.wet > 0.0 {
            0.0
        } else if rates.drying > 0.0 {
            rates.drying
        } else {
            1.0 / self.dry_h.max(1e-3)
        }
    }
}

/// One channel of the state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    Wet,
    Puddle,
    Snow,
    Ash,
}

impl Channel {
    pub const ALL: [Channel; 4] = [Channel::Wet, Channel::Puddle, Channel::Snow, Channel::Ash];

    pub fn name(self) -> &'static str {
        match self {
            Channel::Wet => "wet",
            Channel::Puddle => "puddle",
            Channel::Snow => "snow",
            Channel::Ash => "ash",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.name().eq_ignore_ascii_case(s))
    }

    /// Kept per climate (wet, puddle) rather than per cell (snow, ash).
    pub fn per_climate(self) -> bool {
        matches!(self, Channel::Wet | Channel::Puddle)
    }
}

/// Wetness and puddle fill of one climate.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClimateSurface {
    pub wet: f32,
    pub puddle: f32,
}

/// Snow and ash cover of one 32 m cell.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellSurface {
    pub snow: f32,
    pub ash: f32,
}

impl CellSurface {
    fn is_zero(self) -> bool {
        self.snow <= 0.0 && self.ash <= 0.0
    }
}

/// The deterministic surface state (saved; the host's is authoritative in co-op).
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceState {
    /// Game hours of the last step (on the step grid); `-∞` before the first [`advance`].
    pub t_h: f64,
    /// Indexed by [`ClimateIx`].
    pub climates: Vec<ClimateSurface>,
    /// Per worldspace, the non-zero 32 m cells ([`cell_of`]). No empty inner maps.
    pub cells: HashMap<Worldspace, HashMap<IVec2, CellSurface>>,
}

impl Default for SurfaceState {
    fn default() -> Self {
        Self { t_h: f64::NEG_INFINITY, climates: Vec::new(), cells: HashMap::new() }
    }
}

impl SurfaceState {
    /// Whether [`advance`] has run (or a save set the time).
    pub fn started(&self) -> bool {
        self.t_h.is_finite()
    }

    pub fn climate(&self, c: ClimateIx) -> ClimateSurface {
        self.climates.get(c.0 as usize).copied().unwrap_or_default()
    }

    pub fn cell(&self, ws: &Worldspace, cell: IVec2) -> CellSurface {
        self.cells.get(ws).and_then(|m| m.get(&cell)).copied().unwrap_or_default()
    }

    /// Number of stored (non-zero) cells.
    pub fn active_cells(&self) -> usize {
        self.cells.values().map(HashMap::len).sum()
    }

    /// Forces a climate channel (console `surface set`, scripts). `channel` must be per climate.
    pub fn set_climate(&mut self, c: ClimateIx, channel: Channel, v: f32) {
        let i = c.0 as usize;
        if self.climates.len() <= i {
            self.climates.resize(i + 1, ClimateSurface::default());
        }
        let s = &mut self.climates[i];
        match channel {
            Channel::Wet => s.wet = v.clamp(0.0, 1.0),
            Channel::Puddle => s.puddle = v.clamp(0.0, 1.0),
            Channel::Snow | Channel::Ash => {}
        }
    }

    /// Forces a cell channel. `channel` must be per cell; a cell left at zero is dropped.
    pub fn set_cell(&mut self, ws: &Worldspace, cell: IVec2, channel: Channel, v: f32) {
        let v = v.clamp(0.0, 1.0);
        let map = self.cells.entry(ws.clone()).or_default();
        let s = map.entry(cell).or_default();
        match channel {
            Channel::Snow => s.snow = v,
            Channel::Ash => s.ash = v,
            Channel::Wet | Channel::Puddle => {}
        }
        if s.is_zero() {
            map.remove(&cell);
        }
        if map.is_empty() {
            self.cells.remove(ws);
        }
    }

    /// Everything dry and bare; the time stays.
    pub fn clear(&mut self) {
        self.climates.iter_mut().for_each(|c| *c = ClimateSurface::default());
        self.cells.clear();
    }
}

/// The surface rates of each climate over time.
pub trait RateSource {
    /// Climates are `0..climate_count`.
    fn climate_count(&self) -> usize;
    /// The climate's rates (per game hour) at game hour `t_h`.
    fn rates(&self, climate: ClimateIx, t_h: f64) -> SurfaceRates;
}

/// The weather's rates: each preset's [`SurfaceRates`] weighted by its precipitation channel
/// weight in the climate's blend (what `sf_weather::evaluate` sums into `WeatherFrame::surface`
/// for a site of that climate alone).
pub struct WeatherRates<'a> {
    pub book: &'a WeatherBook,
    pub state: &'a WeatherState,
}

impl RateSource for WeatherRates<'_> {
    fn climate_count(&self) -> usize {
        self.book.climate_count()
    }

    fn rates(&self, climate: ClimateIx, t_h: f64) -> SurfaceRates {
        let mut r = SurfaceRates::default();
        for (p, w) in self.state.blend(self.book, climate, t_h) {
            let s = &self.book.preset_record(p).surface;
            let w = w[PRECIP];
            r.wet += s.wet * w;
            r.puddle += s.puddle * w;
            r.snow += s.snow * w;
            r.ash += s.ash * w;
            r.drying += s.drying * w;
        }
        r
    }
}

/// Steps `state` to game hour `to_h` (surface-weather.md "Step"): every step of the grid
/// `k · step_h` in `(t_h, to_h]`, at most `catch_up_h` of them; `grids` are the worldspaces
/// whose cells step (others keep their cover until their grid is back). A clock that went
/// back re-anchors the time without stepping. Returns the steps taken.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, reason = "step indices of game hours stay far below 2^52")]
pub fn advance(state: &mut SurfaceState, to_h: f64, rates: &dyn RateSource, grids: &[&CellGrid], tuning: &SurfaceTuning) -> u32 {
    let step = tuning.step_h.max(1e-6);
    let last = (to_h / step + 1e-9).floor() as i64;
    let max_steps = ((tuning.catch_up_h / step).round() as i64).max(1);
    let first = if state.started() {
        let done = (state.t_h / step).round() as i64;
        if last <= done {
            if last < done {
                state.t_h = last as f64 * step;
            }
            return 0;
        }
        done + 1
    } else {
        last - max_steps + 1
    };
    let first = first.max(last - max_steps + 1);
    let n = rates.climate_count();
    if state.climates.len() < n {
        state.climates.resize(n, ClimateSurface::default());
    }
    let mut r = vec![SurfaceRates::default(); n];
    let dt = step as f32;
    for k in first..=last {
        let mid = (k as f64 - 0.5) * step;
        for (i, ri) in r.iter_mut().enumerate() {
            *ri = rates.rates(ClimateIx(u32::try_from(i).expect("climate index fits u32")), mid);
        }
        step_climates(&mut state.climates, &r, dt, tuning);
        for g in grids {
            step_cells(&mut state.cells, g, &r, dt, tuning);
        }
    }
    state.t_h = last as f64 * step;
    u32::try_from(last - first + 1).unwrap_or(u32::MAX)
}

fn step_climates(climates: &mut [ClimateSurface], rates: &[SurfaceRates], dt: f32, t: &SurfaceTuning) {
    for (c, r) in climates.iter_mut().zip(rates) {
        let d = t.drying(r);
        c.wet = unit(c.wet + (r.wet - d) * dt);
        c.puddle = unit(if r.puddle > 0.0 && c.wet >= t.puddle_from_wet {
            c.puddle + r.puddle * dt
        } else {
            c.puddle - t.puddle_dry_mult * d * dt
        });
    }
}

/// A cover channel: rises at `rate` while it falls, else decays at `mult · d`; rain washes it
/// off at `wash` either way (a mixed site: sleet on falling snow, rain through an ash fall).
fn cover(v: f32, rate: f32, mult: f32, d: f32, wash: f32, dt: f32) -> f32 {
    unit(if rate > 0.0 { v + (rate - wash) * dt } else { v - (mult * d + wash) * dt })
}

/// Clamped to [0, 1]; below half a save quantum is 0 (so drained channels and cells drop
/// instead of lingering at float residue).
fn unit(v: f32) -> f32 {
    let v = v.clamp(0.0, 1.0);
    if v < 0.5 * QUANTUM { 0.0 } else { v }
}

fn falls(r: &SurfaceRates) -> bool {
    r.snow > 0.0 || r.ash > 0.0
}

fn step_cells(cells: &mut HashMap<Worldspace, HashMap<IVec2, CellSurface>>, grid: &CellGrid, r: &[SurfaceRates], dt: f32, t: &SurfaceTuning) {
    let falling = |c: ClimateIx| r.get(c.0 as usize).is_some_and(falls);
    let mut map = cells.remove(grid.worldspace()).unwrap_or_default();
    // Cells some climate covers this step; each once, by the first such climate of its site.
    for (ci, rc) in r.iter().enumerate() {
        if !falls(rc) {
            continue;
        }
        let ci = ClimateIx(u32::try_from(ci).expect("climate index fits u32"));
        for &ix in grid.cells_of(ci) {
            let ix = ix as usize;
            let site = grid.site(ix);
            if site.iter().find(|e| falling(e.climate)).map(|e| e.climate) != Some(ci) {
                continue;
            }
            let cell = grid.cell(ix);
            let b = grid::blend(site, r, t);
            let s = map.entry(cell).or_default();
            s.snow = cover(s.snow, b.snow, t.snow_melt_mult, b.drying, t.snow_wash * b.rain, dt);
            s.ash = cover(s.ash, b.ash, t.ash_clear_mult, b.drying, t.ash_wash * b.rain, dt);
        }
    }
    // The rest decay (and rain washes them).
    let bare = 1.0 / t.dry_h.max(1e-3);
    for (cell, s) in &mut map {
        let (d, rain) = match grid.index(*cell) {
            Some(ix) => {
                let site = grid.site(ix);
                if site.iter().any(|e| falling(e.climate)) {
                    continue;
                }
                let b = grid::blend(site, r, t);
                (b.drying, b.rain)
            }
            None => (bare, 0.0),
        };
        s.snow = cover(s.snow, 0.0, t.snow_melt_mult, d, t.snow_wash * rain, dt);
        s.ash = cover(s.ash, 0.0, t.ash_clear_mult, d, t.ash_wash * rain, dt);
    }
    map.retain(|_, s| !s.is_zero());
    if !map.is_empty() {
        cells.insert(grid.worldspace().clone(), map);
    }
}

/// The four channels at a point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfaceSample {
    pub wet: f32,
    pub puddle: f32,
    pub snow: f32,
    pub ash: f32,
}

impl SurfaceSample {
    pub fn get(&self, c: Channel) -> f32 {
        match c {
            Channel::Wet => self.wet,
            Channel::Puddle => self.puddle,
            Channel::Snow => self.snow,
            Channel::Ash => self.ash,
        }
    }
}

/// The surface at world X, Z `p` of worldspace `ws`: wetness and puddles blended over the
/// climates of `raster.site_at(p)`, cover from `p`'s 32 m cell (no filtering: the GPU texture
/// filters its own copy).
pub fn sample(state: &SurfaceState, raster: &ClimateRaster, ws: &Worldspace, p: DVec2) -> SurfaceSample {
    let mut out = SurfaceSample::default();
    for (c, w) in raster.site_at(p) {
        let s = state.climate(c);
        out.wet += s.wet * w;
        out.puddle += s.puddle * w;
    }
    let cell = state.cell(ws, cell_of(p));
    out.snow = cell.snow;
    out.ash = cell.ash;
    out
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod view_tests;
#[cfg(test)]
mod cover_tests;
#[cfg(test)]
mod trail_tests;
#[cfg(test)]
mod kickup_tests;
