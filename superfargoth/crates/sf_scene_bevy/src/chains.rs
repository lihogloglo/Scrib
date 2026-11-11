//! A chain's geometry on the Bevy side: its `scene.chains` product (an
//! `sf_model` `.sfmesh`, read by `sf_model_bevy`'s loader), one mesh per
//! chain level, and its material.

use std::collections::HashMap;

use bevy_asset::{AssetId, AssetServer, Assets, Handle, LoadState, RecursiveDependencyLoadState};
use bevy_ecs::prelude::Resource;
use bevy_mesh::Mesh;
use bevy_pbr::StandardMaterial;
use sf_model_bevy::{CookedModelAsset, baked_path};
use sf_scene::{CHAINS_EXT, CHAINS_STEP, ChainEntry};
use sf_world_bevy::ModelMaterials;

/// One resident chain's assets, held by `SpaceState::chains` at the chain's slot.
#[derive(Clone, Debug)]
pub struct ChainAssets {
    pub product: Handle<CookedModelAsset>,
    /// One mesh per chain level, finest first.
    pub levels: Vec<Handle<Mesh>>,
    /// The chain's material ([`chain_material`]), made when its product
    /// resolves. `None` only in an app without `Assets<StandardMaterial>`.
    pub material: Option<Handle<StandardMaterial>>,
}

impl ChainAssets {
    pub fn level_ids(&self) -> impl Iterator<Item = AssetId<Mesh>> + '_ {
        self.levels.iter().map(Handle::id)
    }

    pub fn material_id(&self) -> Option<AssetId<StandardMaterial>> {
        self.material.as_ref().map(Handle::id)
    }
}

/// Every chain material made, one per `ChainEntry::material_key` (the content
/// hash of the cooked material: equal keys, one bindless slot). Pruned to the
/// materials a resident or loading chain still holds whenever
/// `ChainSlabTable::dirty` rebuilds what the scene asks the render world about.
#[derive(Resource, Default, Debug)]
pub struct ChainMaterials(pub HashMap<u64, Handle<StandardMaterial>>);

/// What making a chain material needs: the entity path's mapper and its
/// matclass switch (`StreamingConfig::material_classes`, the app's `--off matclass`).
pub struct MaterialMaker<'a> {
    pub materials: &'a mut Assets<StandardMaterial>,
    pub cache: &'a mut ChainMaterials,
    pub mapper: &'a ModelMaterials,
    pub fold: bool,
}

/// The chain's material: its product's `materials[0]` through
/// [`ModelMaterials`] and, unless matclass is off, `matclass::fold`, made once
/// per `ChainEntry::material_key`. The same steps in the same order as
/// `sf_world_bevy`'s `build_template`, so a chain's material folds into the
/// class its entity-path twin does.
pub fn chain_material(
    maker: &mut MaterialMaker<'_>,
    asset: &CookedModelAsset,
    entry: &ChainEntry,
    asset_server: &AssetServer,
) -> Handle<StandardMaterial> {
    let MaterialMaker { materials, cache, mapper, fold } = maker;
    cache
        .0
        .entry(entry.material_key)
        .or_insert_with(|| {
            let desc = asset.materials.first().cloned().unwrap_or_default();
            let mut made = (mapper.0)(&desc, asset_server);
            if *fold {
                sf_world_bevy::matclass::fold(&mut made);
            }
            materials.add(made)
        })
        .clone()
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
/// With `maker`, the chain's material too ([`chain_material`]).
pub fn chain_assets(
    products: &Assets<CookedModelAsset>,
    handle: &Handle<CookedModelAsset>,
    entry: &ChainEntry,
    maker: Option<&mut MaterialMaker<'_>>,
    asset_server: &AssetServer,
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
    let material = maker.map(|m| chain_material(m, asset, entry, asset_server));
    Ok(ChainAssets { product: handle.clone(), levels, material })
}
