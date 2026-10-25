//! Bevy glue for cooked models.
//!
//! - [`SfMeshLoader`] loads `.sfmesh` bytes into a [`CookedModelAsset`]: one
//!   Bevy `Mesh` per LOD and material (labeled `lod<k>/<i>`), with Bevy's own
//!   vertex compression applied by default (quantized positions, octahedral
//!   normals, 16-bit UVs, 16-bit indices where possible).
//! - [`baked_source`] serves bake products as the `baked://` asset source,
//!   resolving `<step>/<subject>` through the profile manifest.
//!
//! - Skinned models (`Cooked::Skinned`) load into the same asset with a
//!   [`CookedSkin`] (bake.md "Bevy mapping"); [`spawn_skinned`] binds them
//!   to a skeleton's joint entities by name.
//!
//! Spawning static models (materials, `VisibilityRange` per LOD) lives with
//! streaming in `sf_world_bevy`.

mod loader;
mod skinned;
mod source;
mod spawn;

pub use loader::{
    CookedLodMeshes, CookedModelAsset, CookedRayTracing, SfMeshLoadError, SfMeshLoader,
    SfMeshLoaderSettings, compression_args, static_model_asset, submesh_to_mesh,
    submesh_to_ray_tracing_mesh,
};
pub use skinned::{
    CookedSkin, MAX_JOINTS, SkinnedMeshInfo, gather_weights, inverse_bindposes, morph_attributes,
    skinned_submesh_to_mesh,
};
pub use source::{BAKED_SOURCE, BakedManifest, BakedReader, baked_path, baked_source};
pub use spawn::{
    MissingJoints, ModelMorphWeights, MorphWeightSource, resolve_joints, spawn_rest_skeleton,
    spawn_skinned, sync_morph_weights,
};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::AssetApp;

/// Registers [`CookedModelAsset`] and its loader. Needs Bevy's `MeshPlugin`
/// (part of `DefaultPlugins`). Register [`baked_source`] separately, before
/// `AssetPlugin`, as with any asset source.
pub struct SfModelPlugin;

impl Plugin for SfModelPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CookedModelAsset>()
            .register_asset_loader(SfMeshLoader)
            .add_systems(PostUpdate, sync_morph_weights);
    }
}

#[cfg(test)]
mod tests;
