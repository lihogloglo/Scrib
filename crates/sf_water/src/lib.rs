//! Bevy-free water core (water.md): water body records, the directional wave spectrum and its
//! cascades, the CPU wave twin, and the [`WaterQuery`] that rendering, buoyancy, swimming, wetness,
//! sound and AI share. The GPU side (FFT cascades, surface, water pass) is `sf_water_bevy` (W2).
//!
//! Waves are a pure function of `(seed, spectrum inputs, time, absolute XZ)`: saves store nothing
//! for water, and the floating origin never moves a wave ([`twin::tile_coords`]).

pub mod query;
pub mod records;
pub mod shore;
pub mod spectrum;
pub mod twin;

#[cfg(test)]
mod tests;

pub use query::{
    BedHeights, Capsule, Immersion, WaterQuery, WaterSample, WaterWorld, WorldspaceWater,
};
pub use records::{
    FoamTerms, RiverPoint, WaterBody, WaterKind, WaterOptics, WaterShape, WaveTerms,
};
pub use shore::{OpticsMap, ShoreField, ShoreSettings, ShoreStats, WetMask};
pub use spectrum::{Cascades, DEFAULT_CASCADES, GRAVITY, H0, REPEAT_PERIOD, SpectrumParams, WindInput};
pub use twin::{CascadeTwin, WaveSample, WaveTwin, origin_phase, tile_coords};

use sf_coords::WorldPos;
use sf_world::{RecordRef, Worldspace, WorldspaceId};

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
