//! Where the runtime's canonical tiles come from (terrain.md "Runtime"): the bake store
//! (`bake::BakedTerrain`, feature `bake`) or anything else that produces the same products on
//! demand, such as a generator evaluating a height function per tile (`sf_worldgen::tiles`).
//!
//! The runtime (`sf_terrain_bevy`) calls these from its async loads, so every method must be safe
//! to call from several threads at once, and a tile must depend only on its coordinate: GPU
//! clipmaps and CPU height queries read the same resident tiles, so whatever a provider returns
//! is what both see.
//!
//! **GPU fields** (worldgen.md "Pipeline" stage 7, slice G2): a provider whose heights are a pure
//! function of position may also hand the runtime that function in WGSL ([`TerrainTiles::gpu_field`]).
//! The runtime then fills the height, normal and macro clipmaps on the GPU straight from it, needs
//! no tile to draw (CDLOD bounds come from [`TerrainTiles::block_bounds`]) and generates CPU tiles
//! only for height queries (the CPU twin: [`TerrainTiles::tile`], the reference). The two may differ
//! by float rounding; rendering never feeds gameplay.

use std::sync::Arc;

use sf_world::WorldspaceId;

use crate::layers::LayerMap;
use crate::product::{LayerArrays, LayerCap, MacroProduct, TileProduct};
use crate::pyramid::BlockBounds;
use crate::tile::TileCoord;

/// A height field the GPU evaluates per clipmap texel.
///
/// `wgsl` defines, reading the host's `field_data: array<u32>` (a read-only storage buffer the host
/// declares and fills with [`GpuField::data`]):
///
/// - `fn field_height(g: vec2<i32>, level: u32) -> f32`: the height (m) at level-0 global sample
///   index `g` as height level `level` sees it (its band limit), the same value as sample `g` of the
///   level-`level` tile [`TerrainTiles::tile`] returns, to float rounding;
/// - `fn field_albedo(g: vec2<i32>, level: u32) -> vec3<f32>`: the macro albedo (linear RGB) at `g`
///   for macro level `level`, the same as [`TerrainTiles::macro_albedo`] gives there.
///
/// Its own names must not start with `fill_` (the host's).
#[derive(Clone)]
pub struct GpuField {
    pub wgsl: Arc<str>,
    pub data: Arc<[u32]>,
    /// For logs and the shader's path.
    pub label: String,
}

/// A source of canonical terrain tiles for one worldspace.
pub trait TerrainTiles: Send + Sync + 'static {
    /// The worldspace these tiles belong to.
    fn worldspace(&self) -> &WorldspaceId;

    /// Whether tile `c` exists. Missing tiles draw at [`Self::fill_height`] (or the layer table's
    /// fill height when the provider has layer arrays).
    fn has_tile(&self, c: TileCoord) -> bool;

    /// Tile `c` (`None` if it doesn't exist). `bounds` must cover this tile and every finer level
    /// under it (CDLOD tests nodes against any resident level).
    fn tile(&self, c: TileCoord) -> Option<Result<TileProduct, String>>;

    /// Macro albedo of tile `c`, if the provider has one.
    fn macro_albedo(&self, c: TileCoord) -> Option<Result<MacroProduct, String>>;

    /// The layer arrays at `size` (the largest if `None`) under `cap`, if the provider has any.
    fn layers_capped(&self, size: Option<u32>, cap: &LayerCap) -> Option<Result<LayerArrays, String>>;

    /// Height where no tile exists, meters, when the provider has no layer arrays.
    fn fill_height(&self) -> f32 {
        0.0
    }

    /// The heights as a GPU function, if the provider has one (see [`GpuField`]). A provider that
    /// returns one must also answer [`Self::block_bounds`].
    fn gpu_field(&self) -> Option<GpuField> {
        None
    }

    /// Conservative block bounds of tile `c` and every finer level under it (as [`TileProduct::bounds`]),
    /// without generating the tile; also covering what [`Self::gpu_field`] draws there.
    fn block_bounds(&self, _c: TileCoord) -> Option<BlockBounds> {
        None
    }

    /// Tile `c`'s layer map alone, without its heights (`None`: the provider has none apart from
    /// [`TileProduct::layers`]). A GPU-filled world draws its near layer levels from these, and the tile path
    /// takes one when a tile product carries no layers.
    fn layer_map(&self, _c: TileCoord) -> Option<Result<LayerMap, String>> {
        None
    }

    /// Whether [`Self::layer_map`] gives layer maps: a GPU-filled world then still streams them for its near
    /// layer levels (its heights stay on the GPU).
    fn has_layer_maps(&self) -> bool {
        false
    }
}
