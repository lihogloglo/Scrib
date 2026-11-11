//! A chain's geometry on the Bevy side: its `scene.chains` product (an
//! `sf_model` `.sfmesh`, read by `sf_model_bevy`'s loader) and one mesh per
//! chain level.

use bevy_asset::{AssetId, AssetServer, Assets, Handle, LoadState, RecursiveDependencyLoadState};
use bevy_mesh::Mesh;
use bevy_pbr::StandardMaterial;
use sf_model_bevy::{CookedModelAsset, baked_path};
use sf_scene::{CHAINS_EXT, CHAINS_STEP, ChainEntry};

/// One resident chain's assets, held by `SpaceState::chains` at the chain's slot.
#[derive(Clone, Debug)]
pub struct ChainAssets {
    pub product: Handle<CookedModelAsset>,
    /// One mesh per chain level, finest first.
    pub levels: Vec<Handle<Mesh>>,
    /// TODO(B3b): the chain's material (`materials[0]` through `ModelMaterials` and
    /// `matclass::fold`, deduplicated per `ChainEntry::material_key`). `None` until then.
    pub material: Option<Handle<StandardMaterial>>,
}

impl ChainAssets {
    pub fn level_ids(&self) -> impl Iterator<Item = AssetId<Mesh>> + '_ {
        self.levels.iter().map(Handle::id)
    }
}

/// Starts loading a chain's product.
pub fn load_product(asset_server: &AssetServer, entry: &ChainEntry) -> Handle<CookedModelAsset> {
    asset_server.load(baked_path(CHAINS_STEP, &entry.source.subject(), Some(CHAINS_EXT)))
}

/// Where a chain product's load stands.
#[derive(Debug)]
pub enum ProductState {
    Pending,
    Loaded,
    Failed(String),
}

/// The product and every mesh it depends on.
pub fn product_state(asset_server: &AssetServer, handle: &Handle<CookedModelAsset>) -> ProductState {
    match asset_server.get_load_state(handle.id()) {
        Some(LoadState::Failed(e)) => return ProductState::Failed(e.to_string()),
        Some(LoadState::Loaded) => {}
        _ => return ProductState::Pending,
    }
    match asset_server.get_recursive_dependency_load_state(handle.id()) {
        Some(RecursiveDependencyLoadState::Loaded) => ProductState::Loaded,
        Some(RecursiveDependencyLoadState::Failed(e)) => ProductState::Failed(e.to_string()),
        _ => ProductState::Pending,
    }
}

/// The level meshes of a loaded product, checked against what the cell's
/// table says about the chain: one LOD level per chain level, each one part.
pub fn chain_assets(
    products: &Assets<CookedModelAsset>,
    handle: &Handle<CookedModelAsset>,
    entry: &ChainEntry,
) -> Result<ChainAssets, String> {
    let asset = products.get(handle).ok_or("loaded but not in Assets<CookedModelAsset>")?;
    if let Some(reason) = &asset.skipped {
        return Err(format!("the bake skipped it: {reason}"));
    }
    if asset.lods.len() != entry.levels.len() {
        return Err(format!("{} LOD levels, the table names {}", asset.lods.len(), entry.levels.len()));
    }
    let mut levels = Vec::with_capacity(asset.lods.len());
    for (l, lod) in asset.lods.iter().enumerate() {
        let &(mesh, _) = lod.parts.first().ok_or_else(|| format!("level {l} has no mesh"))?;
        levels.push(asset.meshes.get(mesh).ok_or_else(|| format!("level {l} names mesh {mesh}, which is missing"))?.clone());
    }
    Ok(ChainAssets { product: handle.clone(), levels, material: None })
}
