//! Morrowind water adapter (`docs/specs/water-adapter.md`): Morrowind's water facts in `sf_water`
//! terms. Skeleton from ROADMAP M6 W1: bodies from the worldspaces' water levels (exterior sea at 0,
//! interiors at `WHGT`), optics records by region, the `Morrowind.ini [Water]` keys with vanilla
//! fallbacks. Hidden interiors, moored boats and authored water meshes come with W3, W6 and W9.

use std::collections::HashMap;

use sf_coords::WorldPos;
use sf_water::{
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

/// The adapter's optics records (sea, coastal, swamp, ash, north, interior).
pub fn optics_records(ini: &WaterIni) -> Vec<(&'static str, WaterOptics)> {
    let swamp = WaterOptics {
        foam: FoamTerms {
            whitecaps: 0.0,
            shore: 0.0,
            contact: 0.0,
        },
        caustics: 0.2,
        ..optics_with(3.0, [1.6, 1.0, 1.4], 0.5)
    };
    let ash = WaterOptics {
        surface_tint: Some([0.35, 0.34, 0.32]),
        ..optics_with(12.0, [1.5, 1.0, 1.1], 0.4)
    };
    // Morrowind multiplies its fog distance by the Underwater*Fog values: indoor vs day sets the
    // interior visibility relative to the sea's.
    let indoor = 25.0 * ini.day_fog / ini.indoor_fog.max(0.1);
    vec![
        (optics::SEA, optics_with(25.0, [3.0, 1.0, 1.2], 0.2)),
        (optics::COASTAL, optics_with(15.0, [2.5, 1.0, 1.5], 0.3)),
        (optics::SWAMP, swamp),
        (optics::ASH, ash),
        (optics::NORTH, optics_with(20.0, [2.5, 1.1, 1.0], 0.25)),
        (
            optics::INTERIOR,
            optics_with(indoor.min(12.0), [2.0, 1.0, 1.2], 0.4),
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
