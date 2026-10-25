//! Water records (water.md "Data model"). Adapters generate them; mods patch them field by field.

use glam::DVec2;
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_world::{RecordRef, Worldspace, WorldspaceId};

/// What kind of water a body is: decides its mesh, waves and defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WaterKind {
    Ocean,
    Lake,
    River,
    Pool,
}

/// One point of a river ribbon.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiverPoint {
    /// World XZ, metres.
    pub center: DVec2,
    pub width: f32,
    /// Surface height (world Y).
    pub height: f64,
    /// Downstream speed, m/s.
    pub flow_speed: f32,
}

/// Where a body's water is, in world XZ.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WaterShape {
    /// Everywhere the bed is below the level (the ocean).
    Infinite,
    /// A closed outline with holes (lakes, authored pools).
    Polygon {
        outline: Vec<DVec2>,
        holes: Vec<Vec<DVec2>>,
    },
    /// A river centre line with widths, heights and flow per point.
    Ribbon { spline: Vec<RiverPoint> },
    /// The whole worldspace (interiors).
    Bounds,
}

/// Which wave terms a body uses.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaveTerms {
    /// Index of the first cascade used (0 = all; lakes start at the shorter ones).
    pub first_cascade: u8,
    /// Amplitude scale on the spectrum (fetch-scaled for lakes; 0 = flat).
    pub amplitude: f32,
}

impl WaveTerms {
    pub const FLAT: Self = Self {
        first_cascade: u8::MAX,
        amplitude: 0.0,
    };

    /// Defaults per kind: the ocean uses every cascade; lakes the short ones; rivers and pools are flat
    /// (flow normals and ripples are GPU detail).
    pub fn for_kind(kind: WaterKind) -> Self {
        match kind {
            WaterKind::Ocean => Self {
                first_cascade: 0,
                amplitude: 1.0,
            },
            WaterKind::Lake => Self {
                first_cascade: 2,
                amplitude: 1.0,
            },
            WaterKind::River | WaterKind::Pool => Self::FLAT,
        }
    }
}

/// A water body record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaterBody {
    #[serde(with = "record_id")]
    pub id: RecordId,
    pub worldspace: WorldspaceId,
    pub kind: WaterKind,
    pub shape: WaterShape,
    /// World Y of the rest surface (rivers: per point).
    pub level: f64,
    /// Overlaps: higher wins.
    pub priority: i32,
    /// A [`WaterOptics`] record.
    pub optics: Option<RecordRef>,
    pub waves: WaveTerms,
    /// Baked flow map product (W7).
    pub flow: Option<String>,
}

impl WaterBody {
    pub const KIND: &'static str = "sf.water_body";

    /// The default body of a worldspace from `Worldspace::sea_level`: an infinite ocean for
    /// worldspaces with terrain, a whole-worldspace pool otherwise (interiors).
    pub fn from_worldspace(id: &WorldspaceId, ws: &Worldspace) -> Option<Self> {
        let level = ws.sea_level?;
        let (kind, shape) = if ws.has_terrain {
            (WaterKind::Ocean, WaterShape::Infinite)
        } else {
            (WaterKind::Pool, WaterShape::Bounds)
        };
        Some(Self {
            id: RecordId::new(&format!("{}#water", id.as_str())),
            worldspace: id.clone(),
            kind,
            shape,
            level: f64::from(level),
            priority: 0,
            optics: None,
            waves: WaveTerms::for_kind(kind),
            flow: None,
        })
    }
}

/// Foam look.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FoamTerms {
    /// Whitecap coverage scale (Jacobian threshold bias); 0 = none.
    pub whitecaps: f32,
    /// Shoreline band strength.
    pub shore: f32,
    /// Contact foam strength (rocks, piers, legs).
    pub contact: f32,
}

/// Physical optics, per metre, linear RGB.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaterOptics {
    pub absorption: [f32; 3],
    pub scattering: [f32; 3],
    /// Henyey-Greenstein asymmetry.
    pub phase_g: f32,
    pub roughness: f32,
    pub foam: FoamTerms,
    pub refraction_strength: f32,
    pub caustics: f32,
    /// Floating film (swamp scum, ash).
    pub surface_tint: Option<[f32; 3]>,
}

impl WaterOptics {
    pub const KIND: &'static str = "sf.water_optics";
}

impl Default for WaterOptics {
    /// Clear ocean water: pure-water absorption (red ≈ 0.45/m, green 0.06, blue 0.02) plus a little
    /// particle scattering. Adapters calibrate their own.
    fn default() -> Self {
        Self {
            absorption: [0.45, 0.065, 0.02],
            scattering: [0.003, 0.004, 0.005],
            phase_g: 0.8,
            roughness: 0.03,
            foam: FoamTerms {
                whitecaps: 1.0,
                shore: 1.0,
                contact: 1.0,
            },
            refraction_strength: 0.3,
            caustics: 1.0,
            surface_tint: None,
        }
    }
}

impl WaterOptics {
    /// Distance at which the view through the water drops to 2 % in the clearest channel.
    pub fn visibility(&self) -> f32 {
        let ext = (0..3)
            .map(|c| self.absorption[c] + self.scattering[c])
            .fold(f32::MAX, f32::min);
        (1.0f32 / 0.02).ln() / ext.max(1e-6)
    }
}

/// `RecordId` as a plain string (`sf_records` keeps ids serde-free).
mod record_id {
    use serde::{Deserialize, Deserializer, Serializer};
    use sf_records::RecordId;

    pub(super) fn serialize<S: Serializer>(id: &RecordId, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(id.as_str())
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<RecordId, D::Error> {
        Ok(RecordId::from(String::deserialize(d)?))
    }
}
