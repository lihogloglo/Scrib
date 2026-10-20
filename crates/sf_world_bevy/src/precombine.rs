//! The near-cell precombine hook (precombine.md, "Runtime"): when a cell's
//! near level loads, streaming also loads the cell's [`CellPrecombine`], skips
//! the models and visuals of its members, and spawns its buckets instead (a
//! handful of merged meshes per cell instead of one entity per placement, LOD
//! level and material part). Members still activate (colliders, scripts).
//!
//! The asset type lives here so this crate stays free of the bake's types: a
//! content-side plugin (`sf_lod_bevy`) registers a loader producing it and
//! inserts [`Precombines`] with the asset path of a cell. Without the resource,
//! or with [`StreamingConfig::precombine`](crate::StreamingConfig) off,
//! nothing changes. A cell whose precombine is missing, failed to load, or
//! names a member the overlay moved or removed spawns the old way.

use std::sync::Arc;

use bevy_asset::Asset;
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use bevy_reflect::TypePath;
use sf_coords::WorldPos;
use sf_model::{LodRange, LodScreen};
use sf_model_bevy::CookedModelAsset;
use sf_world::{PlacementId, WorldspaceId};

/// A cell's merged statics, by size bucket.
#[derive(Asset, TypePath, Debug)]
pub struct CellPrecombine {
    /// The buckets' shared origin (the far field's L1 node center, so a
    /// bucket ending at the handoff matches its proxy's fade exactly).
    pub center: WorldPos,
    pub buckets: Vec<PrecombineBucket>,
}

/// One merged model: members of one size class.
#[derive(Debug)]
pub struct PrecombineBucket {
    /// Largest member radius.
    pub max_radius: f32,
    /// Distance from [`CellPrecombine::center`] to the farthest member point:
    /// added to every LOD switch distance (no member coarsens earlier than it
    /// would alone).
    pub reach: f32,
    /// Placements drawn by this bucket (sorted).
    pub members: Vec<PlacementId>,
    /// Positions relative to [`CellPrecombine::center`].
    pub model: CookedModelAsset,
}

/// Where a cell's [`CellPrecombine`] loads from (an asset path), set by the
/// plugin that registers its loader.
#[derive(Resource, Clone)]
pub struct Precombines {
    pub path: PrecombinePath,
}

/// A cell's precombine asset path; `None` where the bake has none (a
/// worldspace without a far field, such as an interior).
pub type PrecombinePath = Arc<dyn Fn(&World, &WorldspaceId, IVec3) -> Option<String> + Send + Sync>;

/// LOD ranges of a bucket: its chain (errors in cell space) ending at
/// `end` with a fade band `end_band`, every switch distance pushed out by
/// `reach` (the ranges are measured from the bucket's origin, its members sit
/// up to `reach` away from it).
/// As [`chain_ranges`](crate::chain_ranges) otherwise.
pub fn bucket_ranges(errors: &[f32], lod: &LodScreen, end: f32, end_band: f32, reach: f32) -> Vec<LodRange> {
    // `visibility_range` centers bands on boundaries: end at the band's middle.
    let max = if end_band > 0.0 { end + end_band * 0.5 } else { end };
    let margin = |d: f32| if d > 0.0 && d < max { d * lod.crossfade } else { 0.0 };
    let mut out = Vec::with_capacity(errors.len());
    let mut start = 0.0f32;
    for i in 0..errors.len() {
        let stop = errors.get(i + 1).map_or(max, |&e| (lod.distance(e) + reach).min(max)).max(start);
        out.push(LodRange { start, end: stop, start_margin: margin(start), end_margin: margin(stop) });
        start = stop;
    }
    if end_band > 0.0
        && let Some(last) = out.iter_mut().rev().find(|r| r.end > r.start)
    {
        last.end_margin = end_band;
    }
    out
}
