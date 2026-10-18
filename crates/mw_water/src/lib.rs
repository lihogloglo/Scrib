//! Morrowind water adapter (`docs/specs/water-adapter.md`): Morrowind's water facts in `sf_water`
//! terms. Skeleton from ROADMAP M6 W1: bodies from the worldspaces' water levels (exterior sea at 0,
//! interiors at `WHGT`), optics records by region, the `Morrowind.ini [Water]` keys with vanilla
//! fallbacks. Hidden interiors, moored boats and authored water meshes come with W3, W6 and W9.

use std::collections::HashMap;

use sf_coords::WorldPos;
use sf_water::{
    OpticsMap, ShoreSettings, WetMask,
    FoamTerms, RiverPolicy, SpectrumParams, WaterBody, WaterOptics, WaterOpticsSource, WaterSource,
};
use sf_world::{RecordRef, Worldspace, WorldspaceId};

#[cfg(test)]
mod tests;

/// Open-sea fetch, metres (water-adapter.md "Openness and fetch"): the Inner Sea is narrow and
/// Vvardenfell compressed, so less than a real ocean. Calibration constant.
pub const SEA_FETCH_M: f32 = 20_000.0;

/// Optics record ids.
pub mod optics {
    pub const SEA: &str = "mw_water/sea";
    pub const COASTAL: &str = "mw_water/coastal";
    pub const SWAMP: &str = "mw_water/swamp";
    pub const ASH: &str = "mw_water/ash";
    pub const NORTH: &str = "mw_water/north";
    pub const INTERIOR: &str = "mw_water/interior";
}

fn optics_ref(id: &str) -> RecordRef {
    RecordRef::new(WaterOptics::KIND, id)
}

/// The optics record for an exterior region name (`REGN` names as the W0 census found them on cells
/// with water). Unknown regions and the open sea use `mw_water/sea`.
pub fn optics_for_region(region: &str) -> &'static str {
    let r = region.to_ascii_lowercase();
    let has = |s: &str| r.contains(s);
    if has("bitter coast") {
        optics::SWAMP
    } else if has("ascadian") || has("west gash") || has("grazelands") || has("molag mar") {
        optics::COASTAL
    } else if has("ashlands") || has("red mountain") || has("molag amur") {
        optics::ASH
    } else if [
        "felsaad",
        "hirstaang",
        "isinfier",
        "moesring",
        "brodir",
        "thirsk",
    ]
    .iter()
    .any(|s| has(s))
    {
        optics::NORTH
    } else {
        optics::SEA
    }
}

/// Physical optics with a target visibility (2 % contrast distance) in the clearest channel;
/// `tint` scales the extinction per channel (the clearest channel 1), `scatter_share` is the
/// scattering part of the extinction. Tuned on WA-1/WA-2/WA-5 shots (W3).
fn optics_with(visibility_m: f32, tint: [f32; 3], scatter_share: f32) -> WaterOptics {
    let ext = (1.0f32 / 0.02).ln() / visibility_m;
    WaterOptics {
        absorption: tint.map(|t| ext * (t - scatter_share)),
        scattering: tint.map(|_| ext * scatter_share),
        ..WaterOptics::default()
    }
}

/// Foam terms (whitecaps, shore band, contact).
fn foam(whitecaps: f32, shore: f32, contact: f32) -> FoamTerms {
    FoamTerms {
        whitecaps,
        shore,
        contact,
    }
}

/// The adapter's optics records (sea, coastal, swamp, ash, north, interior).
///
/// W3 tuning (owner 6a: "too milky turquoise, colour must vary by area, shore outline too white"):
/// the body colour is `scattering / extinction` lit like a diffuse surface, so `scatter_share` is
/// the water's diffuse reflectance, which in nature follows the *backscatter* (≈ 0.33 `b_b / a`:
/// 2-5 % for open and coastal water, Morel & Prieur 1977), not the total scattering (W2 used
/// 7-25 %: milky). Hues by region: the open Inner Sea deep blue (blue clearest), coastal lagoons
/// turquoise (green clearest), the Bitter Coast swamp tea-olive (blue absorbed by tannins, red
/// and green clear), ash-fed water grey-brown, the north steel blue. Shore foam per region (calm
/// lagoons less, the north more).
pub fn optics_records(ini: &WaterIni) -> Vec<(&'static str, WaterOptics)> {
    let swamp = WaterOptics {
        foam: foam(0.0, 0.0, 0.3),
        caustics: 0.2,
        ..optics_with(3.0, [1.15, 1.0, 2.6], 0.035)
    };
    let ash = WaterOptics {
        surface_tint: Some([0.35, 0.34, 0.32]),
        foam: foam(0.6, 0.3, 0.6),
        ..optics_with(10.0, [1.3, 1.0, 1.5], 0.05)
    };
    let sea = WaterOptics {
        foam: foam(1.0, 0.45, 0.6),
        ..optics_with(25.0, [3.0, 1.15, 1.0], 0.025)
    };
    let coastal = WaterOptics {
        foam: foam(0.8, 0.3, 0.5),
        ..optics_with(14.0, [2.6, 1.0, 1.2], 0.04)
    };
    let north = WaterOptics {
        foam: foam(1.0, 0.6, 0.7),
        ..optics_with(18.0, [2.4, 1.2, 1.0], 0.02)
    };
    // Morrowind multiplies its fog distance by the Underwater*Fog values: indoor vs day sets the
    // interior visibility relative to the sea's.
    let indoor = 25.0 * ini.day_fog / ini.indoor_fog.max(0.1);
    vec![
        (optics::SEA, sea),
        (optics::COASTAL, coastal),
        (optics::SWAMP, swamp),
        (optics::ASH, ash),
        (optics::NORTH, north),
        (
            optics::INTERIOR,
            WaterOptics {
                foam: foam(0.0, 0.0, 0.4),
                ..optics_with(indoor.min(12.0), [2.0, 1.1, 1.0], 0.03)
            },
        ),
    ]
}

/// Morrowind's water: the sea at its single level everywhere outside, a pool at each interior's
/// level, flow only on existing water (owner decision 3).
pub struct MwWaterSource;

impl WaterSource for MwWaterSource {
    fn bodies(&self, id: &WorldspaceId, ws: &Worldspace, out: &mut Vec<WaterBody>) {
        if let Some(mut body) = WaterBody::from_worldspace(id, ws) {
            body.optics = Some(optics_ref(if ws.has_terrain {
                optics::SEA
            } else {
                optics::INTERIOR
            }));
            out.push(body);
        }
    }

    fn river_policy(&self) -> RiverPolicy {
        RiverPolicy::FlowOnly
    }

    fn ocean_spectrum(&self, _id: &WorldspaceId) -> SpectrumParams {
        SpectrumParams {
            fetch: SEA_FETCH_M,
            ..SpectrumParams::default()
        }
    }
}

/// Optics by region. The region lookup (exterior cell under a position → `REGN` name) comes from the
/// world adapter, so this crate needs no cell grid.
pub struct MwWaterOptics<F> {
    pub region_at: F,
}

impl<F: Fn(WorldPos) -> Option<String> + Send + Sync> WaterOpticsSource for MwWaterOptics<F> {
    fn optics_at(&self, _ws: &WorldspaceId, at: WorldPos) -> Option<RecordRef> {
        let region = (self.region_at)(at).unwrap_or_default();
        Some(optics_ref(optics_for_region(&region)))
    }
}

/// `Morrowind.ini [Water]` (water-adapter.md "Morrowind water facts"), vanilla values when a key is
/// missing or unreadable.
#[derive(Clone, Debug, PartialEq)]
pub struct WaterIni {
    /// sRGB 0-255.
    pub underwater_color: [u8; 3],
    pub underwater_color_weight: f32,
    pub sunrise_fog: f32,
    pub day_fog: f32,
    pub sunset_fog: f32,
    pub night_fog: f32,
    pub indoor_fog: f32,
    pub ripple_lifetime: f32,
    pub max_ripples: u32,
    /// Game units.
    pub near_water_radius: f32,
    pub near_water_points: u32,
    pub near_water_underwater_freq: f32,
    pub near_water_underwater_volume: f32,
    pub near_water_indoor_id: String,
    pub near_water_outdoor_id: String,
}

impl Default for WaterIni {
    fn default() -> Self {
        Self {
            underwater_color: [12, 30, 37],
            underwater_color_weight: 0.85,
            sunrise_fog: 3.0,
            day_fog: 2.5,
            sunset_fog: 3.0,
            night_fog: 4.0,
            indoor_fog: 3.0,
            ripple_lifetime: 3.0,
            max_ripples: 75,
            near_water_radius: 1000.0,
            near_water_points: 8,
            near_water_underwater_freq: 0.3,
            near_water_underwater_volume: 0.9,
            near_water_indoor_id: "Water Layer".into(),
            near_water_outdoor_id: "Water Layer".into(),
        }
    }
}

impl WaterIni {
    /// Parses the `[Water]` section of `Morrowind.ini` bytes (Windows-1252).
    pub fn parse(bytes: &[u8]) -> Self {
        let (text, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
        let mut keys: HashMap<String, String> = HashMap::new();
        let mut in_water = false;
        for line in text.lines().map(str::trim) {
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                in_water = name.trim().eq_ignore_ascii_case("water");
            } else if in_water && let Some((k, v)) = line.split_once('=') {
                keys.insert(k.trim().to_ascii_lowercase(), v.trim().to_owned());
            }
        }
        let num = |k: &str, d: f32| keys.get(k).and_then(|v| v.parse::<f32>().ok()).unwrap_or(d);
        let int = |k: &str, d: u32| keys.get(k).and_then(|v| v.parse::<u32>().ok()).unwrap_or(d);
        let d = Self::default();
        let color = keys.get("underwatercolor").and_then(|c| {
            let parts: Vec<u8> = c.split(',').filter_map(|p| p.trim().parse().ok()).collect();
            <[u8; 3]>::try_from(parts).ok()
        });
        Self {
            underwater_color: color.unwrap_or(d.underwater_color),
            underwater_color_weight: num("underwatercolorweight", d.underwater_color_weight),
            sunrise_fog: num("underwatersunrisefog", d.sunrise_fog),
            day_fog: num("underwaterdayfog", d.day_fog),
            sunset_fog: num("underwatersunsetfog", d.sunset_fog),
            night_fog: num("underwaternightfog", d.night_fog),
            indoor_fog: num("underwaterindoorfog", d.indoor_fog),
            ripple_lifetime: num("ripplelifetime", d.ripple_lifetime),
            max_ripples: int("maxnumberripples", d.max_ripples),
            near_water_radius: num("nearwaterradius", d.near_water_radius),
            near_water_points: int("nearwaterpoints", d.near_water_points),
            near_water_underwater_freq: num(
                "nearwaterunderwaterfreq",
                d.near_water_underwater_freq,
            ),
            near_water_underwater_volume: num(
                "nearwaterunderwatervolume",
                d.near_water_underwater_volume,
            ),
            near_water_indoor_id: keys
                .get("nearwaterindoorid")
                .cloned()
                .unwrap_or(d.near_water_indoor_id),
            near_water_outdoor_id: keys
                .get("nearwateroutdoorid")
                .cloned()
                .unwrap_or(d.near_water_outdoor_id),
        }
    }
}

/// The shore product's tuning for Vvardenfell (water-adapter.md "Openness and fetch on one sea
/// plane"): the framework defaults, the calibration point for WA-2 (Bitter Coast channels calm,
/// the Inner Sea open).
pub fn shore_settings() -> ShoreSettings {
    ShoreSettings::default()
}

/// Metres per Morrowind unit.
const UNIT_M: f64 = 0.9144 / 64.0;
/// Units per exterior cell side, and `LAND` vertices per side.
const CELL_UNITS: f64 = 8192.0;
const LAND_VERTS: usize = 65;

/// Index of an optics record in [`optics_records`] (the GPU optics table's order).
pub fn optics_index(id: &str) -> u8 {
    [
        optics::SEA,
        optics::COASTAL,
        optics::SWAMP,
        optics::ASH,
        optics::NORTH,
        optics::INTERIOR,
    ]
    .iter()
    .position(|o| *o == id)
    .and_then(|i| u8::try_from(i).ok())
    .unwrap_or(0)
}

/// Exterior cell bounds `(x0, y0, x1, y1)` of a set of cells, with one cell of open sea around.
pub fn cell_bounds(cells: impl IntoIterator<Item = (i32, i32)>) -> Option<(i32, i32, i32, i32)> {
    let mut b: Option<(i32, i32, i32, i32)> = None;
    for (x, y) in cells {
        b = Some(b.map_or((x, y, x, y), |(x0, y0, x1, y1)| {
            (x0.min(x), y0.min(y), x1.max(x), y1.max(y))
        }));
    }
    b.map(|(x0, y0, x1, y1)| (x0 - 1, y0 - 1, x1 + 1, y1 + 1))
}

/// The exterior's wet mask at the sea level (water-adapter.md "Openness and fetch on one sea plane")
/// for `water.shore`: every second `LAND` vertex (256 units ≈ 3.66 m texels), water where the
/// height is below 0, cells without `LAND` are water (the engine's fill is deep). `heights` are
/// in units, `[y * 65 + x]`, rows south to north (`mw_world::terrain::LandData`). Engine axes:
/// x = east, z = −north.
// `manual_midpoint`: `(ox + i) / 2` halves an even vertex index, it is not a midpoint.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_precision_loss, clippy::manual_midpoint)]
pub fn wet_mask<'a>(
    bounds: (i32, i32, i32, i32),
    lands: impl IntoIterator<Item = ((i32, i32), &'a [f32])>,
) -> WetMask {
    let (x0, y0, x1, y1) = bounds;
    let step = LAND_VERTS - 1;
    let cols = (x1 - x0 + 1) as usize * step;
    let rows = (y1 - y0 + 1) as usize * step;
    let (w, h) = (cols / 2 + 1, rows / 2 + 1);
    let texel = 2.0 * CELL_UNITS / step as f64 * UNIT_M;
    let origin = glam::DVec2::new(
        f64::from(x0) * CELL_UNITS * UNIT_M,
        -f64::from(y1 + 1) * CELL_UNITS * UNIT_M,
    );
    let mut mask = WetMask::new(w, h, texel, origin);
    for ((cx, cy), heights) in lands {
        if cx < x0 || cx > x1 || cy < y0 || cy > y1 || heights.len() < LAND_VERTS * LAND_VERTS {
            continue;
        }
        let (ox, oy) = ((cx - x0) as usize * step, (cy - y0) as usize * step);
        for j in 0..LAND_VERTS {
            let gy = oy + j;
            if gy % 2 != 0 {
                continue;
            }
            let row = (rows - gy) / 2;
            for i in (0..LAND_VERTS).filter(|i| (ox + i) % 2 == 0) {
                mask.wet[row * w + (ox + i) / 2] = heights[j * LAND_VERTS + i] < 0.0;
            }
        }
    }
    mask
}

/// Optics by exterior cell (water-adapter.md "Optics by region"): one texel per cell, rows south,
/// `region_of` gives a cell's `REGN` name (none: the open sea). The GPU blends between cell centres
/// (≈ 117 m, the spec's 128 m border blend).
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn optics_map(
    bounds: (i32, i32, i32, i32),
    region_of: impl Fn(i32, i32) -> Option<String>,
) -> OpticsMap {
    let (x0, y0, x1, y1) = bounds;
    let (w, h) = ((x1 - x0 + 1) as usize, (y1 - y0 + 1) as usize);
    let mut index = vec![0u8; w * h];
    for row in 0..h {
        for col in 0..w {
            let (cx, cy) = (x0 + col as i32, y1 - row as i32);
            if let Some(r) = region_of(cx, cy) {
                index[row * w + col] = optics_index(optics_for_region(&r));
            }
        }
    }
    let cell = CELL_UNITS * UNIT_M;
    OpticsMap {
        width: w,
        height: h,
        cell,
        origin: glam::DVec2::new(
            (f64::from(x0) + 0.5) * cell,
            -(f64::from(y1) + 0.5) * cell,
        ),
        index,
    }
}
