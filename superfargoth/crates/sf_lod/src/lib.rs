//! Far field core: the HLOD node tree over the streaming grid, cluster
//! proxies (merge + simplify), octahedral impostors, light batches, and
//! (feature `bake`) the `lod.*` bake steps.
//!
//! Bevy-free. `sf_lod_bevy` loads node blobs and draws them; the
//! streaming scheduler (`sf_world`) decides residency. Visibility between
//! levels uses lead bands ([`levels`]): a coarser node is fully drawn
//! before any finer one fades out, so there are never holes.
//! Spec: `docs/specs/far-field.md`.

pub mod blob;
pub mod build;
pub mod cluster;
pub mod impostor;
pub mod levels;
pub mod near;
pub mod node;
pub mod octa;
pub mod policy;
pub mod products;
pub mod texture;

#[cfg(feature = "bake")]
pub mod bake;

pub use blob::{FarDirectory, ImpostorInstance, ImpostorModel, LightPoint, NodeBlob, NodeHeader, NodeSummary};
pub use build::{FarBakeSettings, FarModel, build_l1, build_parent, impostor_sphere, yaw_of};
pub use impostor::{CaptureMesh, ImpostorAtlas, ImpostorSettings, capture};
pub use levels::{FadeRange, FarLevels};
pub use near::{NearBucket, NearCombine, build_near, near_eligible};
pub use node::{FIRST_LEVEL, LAST_LEVEL, NodeKey};
pub use octa::OctaMode;
pub use texture::{TextureDecoder, TextureImage};
pub use policy::{DefaultFarSource, FarLight, FarPolicy, FarSource, ModelSummary, default_policy, far_eligible};

/// Decoding or encoding failed.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("not a {0} (bad magic)")]
    BadMagic(&'static str),
    #[error("decode: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("mesh: {0}")]
    Mesh(String),
    #[error("too large for 32-bit counts")]
    TooLarge,
}

#[cfg(test)]
mod tests;
