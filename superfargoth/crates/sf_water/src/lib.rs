//! Bevy-free water core (water.md): water body records, the directional wave spectrum and its
//! cascades, the CPU wave twin, and the [`WaterQuery`] that rendering, buoyancy, swimming, wetness,
//! sound and AI share. The GPU side (FFT cascades, surface, water pass) is `sf_water_bevy` (W2).
//!
//! Waves are a pure function of `(seed, spectrum inputs, time, absolute XZ)`: saves store nothing
//! for water, and the floating origin never moves a wave ([`twin::tile_coords`]).

pub mod body_mesh;
#[cfg(test)]
mod body_mesh_tests;
pub mod medium;
#[cfg(test)]
mod medium_tests;
pub mod query;
pub mod records;
pub mod sea_wind;
#[cfg(test)]
mod sea_wind_tests;
pub mod shore;
pub mod shore_waves;
pub mod spectrum;
pub mod sspr;
pub mod swim;
#[cfg(test)]
mod swim_tests;
pub mod twin;
pub mod underwater;
#[cfg(test)]
mod underwater_tests;
pub mod wet_tiles;
#[cfg(test)]
mod wet_tiles_tests;

#[cfg(test)]
mod tests;

pub use body_mesh::{BodyAabb, BodyMesh, LakeMeshParams, RiverMeshParams, RiverSample, lake_mesh, river_mesh, river_samples};
pub use medium::{CameraMedium, MediumCues, MediumState, NearPlane, optics_index_at};
pub use query::{
    BedHeights, Capsule, Immersion, WaterQuery, WaterSample, WaterWorld, WorldspaceWater,
};
pub use records::{
    FoamTerms, RiverPoint, WaterBody, WaterKind, WaterOptics, WaterShape, WaveTerms,
};
pub use sea_wind::{SeaTransition, SeaWind, SeaWindSettings, wind_input};
pub use shore::{OpticsMap, ShoreField, ShoreSettings, ShoreStats, SurfFoam, SurfShape, WetMask};
pub use shore_waves::{ShoreWave, ShoreWaves, SurfFoamInput, SurfFoamTerms, SurfLife, shore_foam_weight, surf_birth, surf_envelope, surf_foam, surf_life, swash_edge};
pub use spectrum::{Cascades, DEFAULT_CASCADES, GRAVITY, H0, REPEAT_PERIOD, SpectrumParams, WindInput};
pub use sspr::{PlaneCandidate, reflection_plane};
// Renamed at the root: `Medium` would clash with the `medium` module (camera medium).
pub use swim::{Medium as SwimMedium, SwimParams, medium as swim_medium};
pub use twin::{CascadeTwin, WaveSample, WaveTwin, origin_phase, tile_coords};
pub use underwater::{UnderwaterLight, UnderwaterOptics, sun_term, underwater_colour};
pub use wet_tiles::WetTiles;

use sf_coords::WorldPos;
use sf_world::{RecordRef, Worldspace, WorldspaceId};

/// This crate's tunables (declared beside their settings structs, whose `Default`s read them).
pub const TUNABLES: &[&dyn sf_tunables::AnyDef] = &[
    &body_mesh::LAKE_MAX_TRIANGLE_M2,
    &body_mesh::LAKE_MIN_ANGLE_DEG,
    &body_mesh::LAKE_MAX_STEINER,
    &body_mesh::RIVER_SPACING_M,
    &body_mesh::RIVER_CROSS_SAMPLES,
    &body_mesh::RIVER_BANK_EXTEND_M,
    &shore::SURF_HEIGHT_GAIN,
    &shore::SURF_BREAK_CREST_MIN_M,
    &shore::SURF_BREAK_SHARPNESS,
    &shore::SURF_GAP_CELL_M,
    &shore::SURF_FOAM_BORE_DECAY_RAD,
    &shore::SURF_FOAM_FRONT_RAD,
    &shore::SURF_FOAM_LACE_CELL_M,
    &shore::SURF_FOAM_LACE_AGED,
    &shore::SURF_FOAM_ALBEDO_AGED,
    &shore::SURF_FOAM_AGED_TINT,
    &shore::SURF_FOAM_AGED_OPACITY,
    &shore::SURF_FOAM_BUMP_M,
    &shore::SURF_FOAM_SWASH_RESIDUE_RAD,
    &shore::SURF_FOAM_SWASH_SPREAD,
    &shore::SHORE_FOAM_FULL_BREAKING,
];

/// Default CPU twin grid (water.md: 64²).
pub const TWIN_RESOLUTION: usize = 64;

impl WaveTwin {
    /// The twin of the first `count` cascades (1-4) of a sea state, on an `n × n` grid each.
    pub fn new(
        params: &SpectrumParams,
        cascades: &Cascades,
        seed: u32,
        count: usize,
        n: usize,
    ) -> Self {
        assert!(
            (1..=4).contains(&count) && count <= cascades.len(),
            "the twin evaluates 1-4 cascades"
        );
        let cascades = (0..count)
            .map(|i| CascadeTwin::new(&H0::generate(params, cascades, i, n, seed), params))
            .collect();
        Self { cascades }
    }

    /// The twin of a spectrum transition (W-WIND, [`SeaWind`]): `mix(from, to, w)` with `w` from
    /// [`Self::set_mix`] (0 = the `from` sea). Same seed, so the phases are the same in both.
    pub fn blended(
        from: &SpectrumParams,
        to: &SpectrumParams,
        cascades: &Cascades,
        seed: u32,
        count: usize,
        n: usize,
    ) -> Self {
        assert!(
            (1..=4).contains(&count) && count <= cascades.len(),
            "the twin evaluates 1-4 cascades"
        );
        let cascades = (0..count)
            .map(|i| {
                let a = H0::generate(from, cascades, i, n, seed);
                let b = H0::generate(to, cascades, i, n, seed);
                CascadeTwin::blended(&a, &b, to)
            })
            .collect();
        Self { cascades }
    }

    /// The transition weight of every cascade ([`SeaTransition::weight`]).
    pub fn set_mix(&mut self, w: f32) {
        for c in &mut self.cascades {
            c.set_mix(w);
        }
    }

    /// Twin from already generated (e.g. blended) amplitudes.
    pub fn from_h0(h0: &[H0], params: &SpectrumParams) -> Self {
        assert!(
            (1..=4).contains(&h0.len()),
            "the twin evaluates 1-4 cascades"
        );
        Self {
            cascades: h0.iter().map(|h| CascadeTwin::new(h, params)).collect(),
        }
    }
}

/// How a content pack gets rivers (water.md "Rivers").
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RiverPolicy {
    /// Use the content's river records.
    Authored,
    /// No new water: bake flow on the water the content already has.
    FlowOnly,
    /// Create rivers above sea level from drainage.
    Generate { min_catchment_km2: f32, carve: bool },
}

/// Adapter hook: the water bodies of a worldspace.
pub trait WaterSource: Send + Sync {
    /// Bodies of `ws`. The default is the one body `Worldspace::sea_level` implies.
    fn bodies(&self, id: &WorldspaceId, ws: &Worldspace, out: &mut Vec<WaterBody>) {
        out.extend(WaterBody::from_worldspace(id, ws));
    }

    fn river_policy(&self) -> RiverPolicy {
        RiverPolicy::Authored
    }

    /// The ocean's sea state inputs other than wind (fetch, depth, amplitude, chop) for `ws`.
    fn ocean_spectrum(&self, _id: &WorldspaceId) -> SpectrumParams {
        SpectrumParams::default()
    }
}

/// The framework default: bodies from `sea_level` only.
pub struct DefaultWaterSource;

impl WaterSource for DefaultWaterSource {}

/// Adapter hook: optics by area (a swamp inside the ocean). Rasterized per worldspace at bake (W3).
pub trait WaterOpticsSource: Send + Sync {
    fn optics_at(&self, ws: &WorldspaceId, at: WorldPos) -> Option<RecordRef>;
}
