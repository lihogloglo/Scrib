//! Terrain core: canonical tiles, the refinement chain, the tile pyramid,
//! layer maps, and CPU height queries. No Bevy, no rendering.
//!
//! Adapters expose their heightfields through [`TerrainSource`] at whatever
//! spacing and origin they have. The bake turns them into **canonical
//! tiles**: level-k tiles of 513² samples at `0.5·2^k` m, aligned to the
//! `sf_coords` partition grid (level k = partition shift k). Every function
//! here is a pure, deterministic function of its inputs, so bake steps can
//! wrap them one tile at a time on any number of threads:
//!
//! - [`upsample`] resamples source heights into a level-0 tile (clamped
//!   Catmull-Rom);
//! - [`constrain`] blends back to the source's own surface under footprints;
//! - [`downsample`] builds level k from four level k−1 tiles (even indices);
//! - [`BlockBounds`] and [`EncodedHeights`] are the stored per-tile data;
//! - [`resample_layers`] / [`downsample_layers`] build the splat maps;
//! - [`TerrainHeights`] answers CPU queries over resident tiles plus
//!   [`HeightEdits`];
//! - [`cdlod`] selects render patches and [`clipmap`] fills the GPU
//!   clipmaps (the Bevy-free half of `sf_terrain_bevy`);
//! - `bake` (feature `bake`) wraps all of it into `sf_bake` steps, and
//!   [`TileProduct`] and friends are the stored formats.
//!
//! Global sample indices: level-0 sample `g` on an axis sits at absolute
//! coordinate `g · 0.5 − 128` m (the grid's cell corners are at
//! `256·k − 128`). Results depend only on these integers and the inputs,
//! never on which tile computes them, so tiles that share an edge agree on
//! it bit for bit.
//!
//! Spec: `docs/specs/terrain.md`.

// Filter and geometry math reads best with its textbook names (u, v, h, t, d).
#![allow(clippy::many_single_char_names)]

#[cfg(feature = "bake")]
pub mod bake;
pub mod cdlod;
pub mod clipmap;
mod constrain;
mod edits;
mod layers;
mod product;
mod pyramid;
mod query;
mod resample;
pub mod shadow;
mod source;
mod tile;

pub use product::{LayerArrays, LayerInfo, LayerTable, MacroProduct, ProductError, RtProxy, TileProduct, layer_id};

pub use constrain::{FOOTPRINT_FEATHER, Footprint, FootprintMask, constrain};
pub use edits::{BRICK_SAMPLES, BrickCoord, EditBrick, HeightEdits, TerrainChanged};
pub use layers::{
    LayerId, LayerMap, LayerParams, TintMap, downsample_layers, downsample_tint, resample_layers, resample_tint,
};
pub use pyramid::{BlockBounds, EncodedHeights, MAX_QUANT_STEP, downsample, top_level};
pub use query::{ResidentTile, TerrainHeights, TerrainHit, WorldRay, WorldRect};
pub use resample::upsample;
pub use source::{
    Diagonal, SourceLayers, SourceMosaic, SourceTile, SourceTileId, SourceTileInfo, SourceTint, TerrainSource,
    Triangulation,
};
pub use tile::{
    BASE_SPACING, BLOCK_QUADS, BLOCKS, CANONICAL_DIAGONAL, HeightTile, LAYER_SAMPLES, TILE_QUADS, TILE_SAMPLES,
    TINT_SAMPLES, TileCoord, global_coords,
};

/// Errors from building a [`SourceMosaic`] or a layer map.
#[derive(Debug, thiserror::Error)]
pub enum TerrainError {
    #[error("source tile {id:?}: {what} has {got} values, expected {expected}")]
    BadSize { id: SourceTileId, what: &'static str, got: usize, expected: usize },
    #[error("source tile {id:?}: {what} is not on the lattice of the other tiles")]
    OffLattice { id: SourceTileId, what: &'static str },
    #[error("source tile {id:?}: {what} differs from the other tiles")]
    Mismatch { id: SourceTileId, what: &'static str },
    #[error("tile {0:?} uses more than 256 terrain layers")]
    PaletteOverflow(TileCoord),
}

#[cfg(test)]
mod tests;
#[cfg(all(test, feature = "bake"))]
mod bake_tests;
