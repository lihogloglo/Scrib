//! DetourTileCache: validated layers, obstacles and incremental navmesh rebuilding.
//!
//! Altered Rust translation of the pinned recastnavigation DetourTileCache sources;
//! see NOTICE. Codec selection is supplied by the application; raw layers are provided.
mod builder;
mod cache;
mod layer;
pub use builder::{
    LayerContour, LayerContourSet, LayerPolyMesh, build_contours, build_poly_mesh, build_regions,
};
pub use cache::*;
pub use layer::*;
