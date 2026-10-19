//! The world weather map (weather.md "Distant weather (WE4)"; as built:
//! "`WE4a` as built"): the weather everywhere, not only at the camera.
//!
//! - [`ClimateRaster`]: which climate owns each grid square of a
//!   worldspace (the adapter's region raster). Squares without a climate
//!   (most sea) take the nearest one's (dilation by vector propagation,
//!   Danielsson 1980), so a coastal storm continues out to sea and two
//!   regions meet at sea along their Voronoi border, blended like any other.
//!   [`ClimateRaster::site_at`] gives the climate weights at any point: the
//!   own square's climate and the nearest other climate within half the
//!   border band, through [`border_blend`].
//! - [`ClimateRow`]: one climate evaluated **once at weight 1** (the
//!   continuous channels of a [`WeatherFrame`]). `evaluate` is linear in the
//!   climate weights for every continuous channel, so any point's weather is
//!   the weighted sum of rows: the map never evaluates per texel.
//! - [`WeatherMap`]: the table of rows plus a world-anchored toroidal window
//!   of [`MapTexel`]s around the camera (the CPU twin of `WE4b`'s resolve
//!   pass). Texel `t` (world texel coordinates, from world position / texel
//!   size, independent of any floating origin) lives at storage index
//!   `t mod size`, so recentering rewrites only the strips that entered the
//!   window. When every row's map channels agree (the common clear day) the
//!   map is **uniform** and nothing is resolved.
//!
//! Near and far can't disagree: the camera's site is
//! [`ClimateRaster::site_at`] of the same raster, `evaluate(site)` drives the
//! near effects, and [`WeatherMap::sample`] at the camera equals that frame's
//! continuous channels (test `map_at_the_camera_is_the_current_weather`).
//! Everything is a function of (seed, overrides, clock, raster): co-op peers
//! build the same map without messages.

use glam::{DVec2, IVec2, Vec2};
use sf_atmos::fog::{FogLayer, KOSCHMIEDER};
use smallvec::SmallVec;

use crate::book::{ClimateIx, WeatherBook};
use crate::frame::{MAX_VISIBILITY_M, WeatherFrame, WeatherInput, border_blend, evaluate_into};
use crate::records::PrecipLook;
use crate::state::WeatherState;

/// Climate weights at a point ([`border_blend`]'s output).
pub type Site = SmallVec<[(ClimateIx, f32); 2]>;

/// Which climate owns each grid square of a worldspace.
#[derive(Clone, Debug)]
pub struct ClimateRaster {
    /// World X, Z of square (0, 0)'s minimum corner, meters.
    origin: DVec2,
    /// Square size, meters.
    cell_m: f64,
    width: i32,
    height: i32,
    /// Owner per square (row-major, Z rows), after filling unowned squares.
    own: Vec<ClimateIx>,
    /// Border band width, meters.
    band_m: f32,
    /// Squares searched around the own square for a neighbor climate (half the band, rounded up).
    reach: i32,
    /// Every climate in the raster, sorted, once each.
    climates: Vec<ClimateIx>,
}

impl ClimateRaster {
    /// A raster of `width × height` squares of `cell_m` meters from world
    /// X, Z `origin`; `cells` row-major (index `x + z × width`), `None` for
    /// squares without a climate. `None` when no square has one.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn new(origin: DVec2, cell_m: f64, width: u32, height: u32, cells: &[Option<ClimateIx>], band_m: f32) -> Option<Self> {
        assert_eq!(cells.len(), (width as usize) * (height as usize), "cells must be width × height");
        let own = fill_nearest(width as i32, height as i32, cells)?;
        let mut climates = own.clone();
        climates.sort_unstable();
        climates.dedup();
        let reach = ((f64::from(band_m) * 0.5 / cell_m).ceil() as i32).max(1);
        Some(Self { origin, cell_m, width: width as i32, height: height as i32, own, band_m, reach, climates })
    }

    /// Every climate in the raster, sorted, once each.
    pub fn climates(&self) -> &[ClimateIx] {
        &self.climates
    }

    pub fn band_m(&self) -> f32 {
        self.band_m
    }

    /// The climate owning square `(x, z)` (after filling), if inside.
    #[allow(clippy::cast_sign_loss, reason = "x, z checked non-negative")]
    pub fn climate(&self, x: i32, z: i32) -> Option<ClimateIx> {
        ((0..self.width).contains(&x) && (0..self.height).contains(&z)).then(|| self.own[(x + z * self.width) as usize])
    }

    /// The climate weights at world X, Z `p`: the own square's climate and
    /// the nearest other one within half the band (distance from `p` to that
    /// square), blended across the band. Past the raster: the edge squares'
    /// (the position is clamped).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]
    pub fn site_at(&self, p: DVec2) -> Site {
        let size = DVec2::new(f64::from(self.width), f64::from(self.height));
        let l = ((p - self.origin) / self.cell_m).clamp(DVec2::ZERO, size - 1e-9);
        let g = l.floor().as_ivec2();
        let f = l - g.as_dvec2();
        let own = self.own[(g.x + g.y * self.width) as usize];
        let gap = |d: i32, f: f64| match d.signum() {
            -1 => f + f64::from(-d - 1),
            1 => (1.0 - f) + f64::from(d - 1),
            _ => 0.0,
        };
        let mut best: Option<(f64, ClimateIx)> = None;
        for dz in -self.reach..=self.reach {
            for dx in -self.reach..=self.reach {
                let Some(c) = self.climate(g.x + dx, g.y + dz).filter(|c| *c != own) else { continue };
                let d = gap(dx, f.x).hypot(gap(dz, f.y)) * self.cell_m;
                if best.is_none_or(|(b, bc)| d.total_cmp(&b).then(c.cmp(&bc)).is_lt()) {
                    best = Some((d, c));
                }
            }
        }
        border_blend(own, best.map(|b| b.1), best.map_or(f32::MAX, |b| b.0 as f32), self.band_m)
    }
}

/// Owners for every square: authored ones kept, the rest the nearest
/// authored square's (Euclidean, by vector propagation from the authored
/// squares; ties to the lower climate index, then the lower source square).
#[allow(clippy::cast_sign_loss)]
fn fill_nearest(width: i32, height: i32, cells: &[Option<ClimateIx>]) -> Option<Vec<ClimateIx>> {
    let ix = |x: i32, z: i32| (x + z * width) as usize;
    // Per square: the source square it takes its climate from.
    let mut src: Vec<Option<IVec2>> = vec![None; cells.len()];
    let mut queue = std::collections::VecDeque::new();
    for z in 0..height {
        for x in 0..width {
            if cells[ix(x, z)].is_some() {
                src[ix(x, z)] = Some(IVec2::new(x, z));
                queue.push_back(IVec2::new(x, z));
            }
        }
    }
    if queue.is_empty() {
        return None;
    }
    let key = |at: IVec2, s: IVec2| ((at - s).length_squared(), cells[ix(s.x, s.y)].expect("source is authored"), s.y, s.x);
    while let Some(at) = queue.pop_front() {
        let s = src[ix(at.x, at.y)].expect("queued squares have a source");
        for dz in -1..=1 {
            for dx in -1..=1 {
                let n = at + IVec2::new(dx, dz);
                if (dx, dz) == (0, 0) || !(0..width).contains(&n.x) || !(0..height).contains(&n.y) || cells[ix(n.x, n.y)].is_some() {
                    continue;
                }
                let better = src[ix(n.x, n.y)].is_none_or(|old| key(n, s) < key(n, old));
                if better {
                    src[ix(n.x, n.y)] = Some(s);
                    queue.push_back(n);
                }
            }
        }
    }
    Some(src.iter().map(|s| { let s = s.expect("every square reached"); cells[ix(s.x, s.y)].expect("authored") }).collect())
}

/// Channel indices of a [`ClimateRow`].
pub mod row {
    pub const COVERAGE: usize = 0;
    pub const CLOUD_TYPE: usize = 1;
    pub const DENSITY: usize = 2;
    pub const WETNESS: usize = 3;
    pub const STORM: usize = 4;
    pub const EROSION: usize = 5;
    pub const BASE: usize = 6;
    pub const THICKNESS: usize = 7;
    /// Wind speed at cloud altitude, m/s.
    pub const CLOUD_SPEED: usize = 8;
    pub const CIRRUS: usize = 9;
    pub const CIRRUS_HEIGHT: usize = 10;
    /// Overcast dimming (the sky's darkness).
    pub const DIMMING: usize = 11;
    pub const HAZE: usize = 12;
    pub const DUST: usize = 13;
    /// Ground wind, world X and Z, m/s.
    pub const WIND_X: usize = 14;
    pub const WIND_Z: usize = 15;
    pub const GUSTINESS: usize = 16;
    /// Height fog density (1/m), falloff, base.
    pub const FOG_DENSITY: usize = 17;
    pub const FOG_FALLOFF: usize = 18;
    pub const FOG_BASE: usize = 19;
    pub const BANK_DENSITY: usize = 20;
    pub const BANK_FALLOFF: usize = 21;
    pub const BANK_COVERAGE: usize = 22;
    /// Total precipitation intensity (all kinds).
    pub const PRECIP: usize = 23;
    /// Σ intensity × `PrecipKind::extinction`, 1/m.
    pub const PRECIP_EXTINCTION: usize = 24;
    /// Strike rate, per game hour.
    pub const LIGHTNING: usize = 25;
    pub const WET: usize = 26;
    pub const PUDDLE: usize = 27;
    pub const SNOW: usize = 28;
    pub const ASH: usize = 29;
    pub const DRYING: usize = 30;
    pub const LEN: usize = 31;
}

/// The continuous channels of a [`WeatherFrame`]: one climate at weight 1
/// (a table row), or a blend of rows (a sample of the map).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClimateRow {
    /// Indexed by the [`row`] constants.
    pub ch: [f32; row::LEN],
    /// The heaviest precipitation kind's look (`None`: dry).
    pub look: Option<PrecipLook>,
}

impl Default for ClimateRow {
    fn default() -> Self {
        Self::from_frame(&WeatherBook::default(), &WeatherFrame::default())
    }
}

impl ClimateRow {
    /// The continuous channels of `frame`.
    pub fn from_frame(book: &WeatherBook, frame: &WeatherFrame) -> Self {
        let c = &frame.clouds;
        let f = &frame.fog;
        let s = &frame.surface;
        let (mut precip, mut ext, mut heaviest) = (0.0, 0.0, (0.0, None));
        for e in &frame.precip {
            let kind = book.precip_kind(e.kind);
            precip += e.intensity;
            ext += kind.extinction.max(0.0) * e.intensity;
            if e.intensity > heaviest.0 {
                heaviest = (e.intensity, Some(kind.look));
            }
        }
        let w = frame.sky.wind.velocity;
        Self {
            ch: [
                c.coverage,
                c.cloud_type,
                c.density,
                c.wetness,
                c.storm,
                c.erosion,
                c.base,
                c.thickness,
                c.wind.length(),
                c.cirrus,
                c.cirrus_height,
                frame.sky.overcast_dimming,
                frame.sky.haze,
                frame.sky.dust,
                w.x,
                w.y,
                frame.sky.wind.gustiness,
                f.height.density,
                f.height.falloff,
                f.height.base,
                f.banks.layer.density,
                f.banks.layer.falloff,
                f.banks.coverage,
                precip,
                ext,
                frame.lightning_per_hour,
                s.wet,
                s.puddle,
                s.snow,
                s.ash,
                s.drying,
            ],
            look: heaviest.1,
        }
    }

    pub fn wind(&self) -> Vec2 {
        Vec2::new(self.ch[row::WIND_X], self.ch[row::WIND_Z])
    }

    /// Meteorological visibility at `altitude` (the frame's `visibility_m`
    /// formula over these channels).
    pub fn visibility_m(&self, altitude: f32) -> f32 {
        let layer = FogLayer { density: self.ch[row::FOG_DENSITY], falloff: self.ch[row::FOG_FALLOFF], base: self.ch[row::FOG_BASE] };
        let ext = layer.density_at(altitude) + self.ch[row::PRECIP_EXTINCTION];
        if ext > 0.0 { (KOSCHMIEDER / ext).min(MAX_VISIBILITY_M) } else { MAX_VISIBILITY_M }
    }
}

/// One texel of the weather map: textures A, B, C of the spec's resolve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MapTexel {
    /// Coverage, cloud type (genus), wetness, storm.
    pub a: [f32; 4],
    /// Cloud base, thickness, density, erosion.
    pub b: [f32; 4],
    /// Cirrus, precipitation intensity, height-fog density (1/m), overcast dimming (darkness).
    pub c: [f32; 4],
    /// The heaviest precipitation's look.
    pub look: Option<PrecipLook>,
}

impl MapTexel {
    pub fn from_row(r: &ClimateRow) -> Self {
        use row::{BASE, CIRRUS, CLOUD_TYPE, COVERAGE, DENSITY, DIMMING, EROSION, FOG_DENSITY, PRECIP, STORM, THICKNESS, WETNESS};
        let ch = &r.ch;
        Self {
            a: [ch[COVERAGE], ch[CLOUD_TYPE], ch[WETNESS], ch[STORM]],
            b: [ch[BASE], ch[THICKNESS], ch[DENSITY], ch[EROSION]],
            c: [ch[CIRRUS], ch[PRECIP], ch[FOG_DENSITY], ch[DIMMING]],
            look: r.look,
        }
    }

    /// Largest channel difference, relative for magnitudes over 1 (heights in meters); infinite when the looks differ.
    pub fn max_diff(&self, o: &Self) -> f32 {
        if self.look != o.look {
            return f32::INFINITY;
        }
        let d = |a: &[f32; 4], b: &[f32; 4]| a.iter().zip(b).map(|(x, y)| (x - y).abs() / x.abs().max(y.abs()).max(1.0)).fold(0.0, f32::max);
        d(&self.a, &o.a).max(d(&self.b, &o.b)).max(d(&self.c, &o.c))
    }
}

/// Map channels closer than this are equal (uniform map, unchanged table).
pub const MAP_EPSILON: f32 = 1e-3;
/// Texels per side of the weather map (weather.md WE4 "Weather map").
pub const MAP_SIZE: u32 = 256;

/// What [`WeatherMap::update`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MapUpdate {
    /// A row's map channels moved past [`MAP_EPSILON`] since the last resolve.
    pub table_changed: bool,
    /// The window moved (1/8-map steps).
    pub recentered: bool,
    /// Every row has the same map channels: nothing resolved, the uniform row stands for the map.
    pub uniform: bool,
    /// Texels written this update.
    pub texels: u32,
}

/// The world weather map: climate table + toroidal texel window.
pub struct WeatherMap {
    raster: ClimateRaster,
    size: i32,
    texel_m: f64,
    /// Current rows, parallel to `raster.climates`.
    rows: Vec<ClimateRow>,
    /// Rows at the last resolve.
    resolved: Vec<ClimateRow>,
    uniform: bool,
    /// World texel coordinates of the window's minimum corner.
    window: IVec2,
    /// The texels hold `resolved` over `window`.
    valid: bool,
    texels: Vec<MapTexel>,
    scratch: WeatherFrame,
}

impl WeatherMap {
    /// A `size²` map of `texel_m` texels (≤ half the border band, so the band spans ≥ 4 texels) over `raster`.
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(raster: ClimateRaster, size: u32, texel_m: f64) -> Self {
        assert!(size >= 8 && size.is_multiple_of(8), "map size must be a multiple of 8");
        let n = raster.climates.len();
        Self {
            raster,
            size: size as i32,
            texel_m,
            rows: vec![ClimateRow::default(); n],
            resolved: vec![ClimateRow::default(); n],
            uniform: false,
            window: IVec2::ZERO,
            valid: false,
            texels: vec![MapTexel::default(); (size as usize) * (size as usize)],
            scratch: WeatherFrame::default(),
        }
    }

    pub fn raster(&self) -> &ClimateRaster {
        &self.raster
    }

    /// The climate table (rows parallel to `raster().climates()`), for upload.
    pub fn rows(&self) -> &[ClimateRow] {
        &self.rows
    }

    pub fn row(&self, c: ClimateIx) -> Option<&ClimateRow> {
        self.raster.climates.binary_search(&c).ok().map(|i| &self.rows[i])
    }

    /// The map's row when it is uniform.
    pub fn uniform_row(&self) -> Option<&ClimateRow> {
        self.uniform.then(|| &self.rows[0])
    }

    /// Evaluates every climate once at weight 1 (no lightning window: strikes stay the camera frame's).
    pub fn update_table(&mut self, book: &WeatherBook, state: &WeatherState, hours: f64, hour_of_day: f32) {
        for (i, &c) in self.raster.climates.iter().enumerate() {
            let input = WeatherInput { hours, prev_hours: hours, hour_of_day, altitude: 0.0, climates: &[(c, 1.0)] };
            evaluate_into(book, state, &input, &mut self.scratch);
            self.rows[i] = ClimateRow::from_frame(book, &self.scratch);
        }
        let first = MapTexel::from_row(&self.rows[0]);
        self.uniform = self.rows.iter().all(|r| MapTexel::from_row(r).max_diff(&first) <= MAP_EPSILON);
    }

    /// The table at `hours`, then the window around world X, Z `camera`:
    /// resolved when the table changed or the window moved (only the
    /// entering strips), skipped when uniform.
    pub fn update(&mut self, book: &WeatherBook, state: &WeatherState, hours: f64, hour_of_day: f32, camera: DVec2) -> MapUpdate {
        self.update_table(book, state, hours, hour_of_day);
        let table_changed =
            self.rows.iter().zip(&self.resolved).any(|(r, o)| MapTexel::from_row(r).max_diff(&MapTexel::from_row(o)) > MAP_EPSILON);
        let window = self.window_for(camera);
        let mut out = MapUpdate { table_changed, recentered: window != self.window, uniform: self.uniform, texels: 0 };
        if self.uniform {
            self.valid = false;
            self.window = window;
            return out;
        }
        if !self.valid || table_changed {
            self.resolved.copy_from_slice(&self.rows);
            out.texels = self.resolve(window, None);
        } else if out.recentered {
            out.texels = self.resolve(window, Some(self.window));
        }
        self.window = window;
        self.valid = true;
        out
    }

    /// World texel coordinates of `p` (world-anchored: no floating origin).
    #[allow(clippy::cast_possible_truncation)]
    pub fn texel_of(&self, p: DVec2) -> IVec2 {
        let t = (p / self.texel_m).floor();
        IVec2::new(t.x as i32, t.y as i32)
    }

    /// Storage index of world texel `t` (toroidal).
    #[allow(clippy::cast_sign_loss)]
    pub fn storage_index(&self, t: IVec2) -> usize {
        (t.x.rem_euclid(self.size) + t.y.rem_euclid(self.size) * self.size) as usize
    }

    /// The window (minimum world texel) for a camera at `p`: centered on it in 1/8-map steps.
    pub fn window_for(&self, p: DVec2) -> IVec2 {
        let step = self.size / 8;
        let t = self.texel_of(p);
        IVec2::new(t.x.div_euclid(step), t.y.div_euclid(step)) * step - IVec2::splat(self.size / 2)
    }

    /// The window's minimum world texel and size.
    pub fn window(&self) -> (IVec2, i32) {
        (self.window, self.size)
    }

    /// World X, Z of texel `t`'s center.
    pub fn texel_center(&self, t: IVec2) -> DVec2 {
        (t.as_dvec2() + 0.5) * self.texel_m
    }

    /// The resolved texel holding `p` (`None` when uniform, unresolved or outside the window).
    pub fn texel_at(&self, p: DVec2) -> Option<&MapTexel> {
        let t = self.texel_of(p) - self.window;
        (self.valid && (0..self.size).contains(&t.x) && (0..self.size).contains(&t.y)).then(|| &self.texels[self.storage_index(t + self.window)])
    }

    /// The texels in storage order (row-major over `t mod size`), for upload.
    pub fn texels(&self) -> &[MapTexel] {
        &self.texels
    }

    /// The weather at world X, Z `p` from the current table: exactly the
    /// continuous channels `evaluate` gives for [`ClimateRaster::site_at`].
    pub fn sample(&self, p: DVec2) -> ClimateRow {
        self.sample_site(&self.raster.site_at(p))
    }

    /// The blend of the table's rows by `site`'s weights (climates outside the table contribute nothing).
    pub fn sample_site(&self, site: &[(ClimateIx, f32)]) -> ClimateRow {
        Self::blend(&self.raster.climates, &self.rows, site)
    }

    fn blend(climates: &[ClimateIx], rows: &[ClimateRow], site: &[(ClimateIx, f32)]) -> ClimateRow {
        let mut out = ClimateRow { ch: [0.0; row::LEN], look: None };
        let mut heaviest = 0.0;
        let total: f32 = site.iter().filter(|(c, w)| *w > 0.0 && climates.binary_search(c).is_ok()).map(|e| e.1).sum();
        if total <= 0.0 {
            return ClimateRow::default();
        }
        for &(c, w) in site {
            let Ok(i) = climates.binary_search(&c) else { continue };
            if w <= 0.0 {
                continue;
            }
            let w = w / total;
            let r = &rows[i];
            for (o, x) in out.ch.iter_mut().zip(&r.ch) {
                *o += x * w;
            }
            if r.look.is_some() && r.ch[row::PRECIP] * w > heaviest {
                heaviest = r.ch[row::PRECIP] * w;
                out.look = r.look;
            }
        }
        out
    }

    /// Writes the texels of `window` from the resolved rows, skipping those inside `keep` (the previous window).
    #[allow(clippy::cast_sign_loss)]
    fn resolve(&mut self, window: IVec2, keep: Option<IVec2>) -> u32 {
        let inside = |w: IVec2, t: IVec2| (0..self.size).contains(&(t.x - w.x)) && (0..self.size).contains(&(t.y - w.y));
        let mut n = 0;
        for j in 0..self.size {
            for i in 0..self.size {
                let t = window + IVec2::new(i, j);
                if keep.is_some_and(|k| inside(k, t)) {
                    continue;
                }
                let site = self.raster.site_at(self.texel_center(t));
                let r = Self::blend(&self.raster.climates, &self.resolved, &site);
                let k = self.storage_index(t);
                self.texels[k] = MapTexel::from_row(&r);
                n += 1;
            }
        }
        n
    }
}
