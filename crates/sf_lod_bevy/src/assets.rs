//! Asset loaders for far-field bake products, served by `baked://`:
//! nodes (`.sfnode`), the per-worldspace directory (`.sffar`) and impostor
//! atlases (`.sfimp`). Decoding runs on Bevy's loader tasks, off the main
//! thread.

use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetLoader, Handle, LoadContext, RenderAssetUsages};
use bevy_mesh::Mesh;
use bevy_reflect::TypePath;
use sf_lod::{FarDirectory, ImpostorAtlas, ImpostorInstance, LightPoint, NodeBlob, NodeHeader};
use sf_model_bevy::submesh_to_mesh;

#[derive(Debug, thiserror::Error)]
pub enum FarLoadError {
    #[error("reading far-field product: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] sf_lod::FormatError),
    #[error("proxy mesh: {0}")]
    Mesh(String),
}

/// A loaded node: its proxy meshes (labeled `mesh<i>`) and batches.
#[derive(Asset, TypePath, Debug)]
pub struct FarNodeAsset {
    pub header: NodeHeader,
    #[dependency]
    pub meshes: Vec<Handle<Mesh>>,
    pub impostors: Vec<ImpostorInstance>,
    pub lights: Vec<LightPoint>,
    /// Blob size, for residency counters.
    pub bytes: u64,
}

#[derive(Default, TypePath)]
pub struct FarNodeLoader;

impl AssetLoader for FarNodeLoader {
    type Asset = FarNodeAsset;
    type Settings = ();
    type Error = FarLoadError;

    async fn load(&self, reader: &mut dyn Reader, (): &(), cx: &mut LoadContext<'_>) -> Result<FarNodeAsset, FarLoadError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let blob = NodeBlob::from_bytes(&bytes)?;
        let mut meshes = Vec::new();
        if let Some(model) = &blob.mesh
            && let Some(lod) = model.lods.first()
        {
            for (i, sub) in lod.submeshes.iter().enumerate() {
                let mesh = submesh_to_mesh(model, sub, RenderAssetUsages::RENDER_WORLD, true)
                    .map_err(|e| FarLoadError::Mesh(e.to_string()))?;
                meshes.push(cx.add_labeled_asset(format!("mesh{i}"), mesh));
            }
        }
        Ok(FarNodeAsset { header: blob.header, meshes, impostors: blob.impostors, lights: blob.lights, bytes: bytes.len() as u64 })
    }

    fn extensions(&self) -> &[&str] {
        &["sfnode"]
    }
}

/// A worldspace's far directory.
#[derive(Asset, TypePath, Debug)]
pub struct FarDirectoryAsset(pub FarDirectory);

#[derive(Default, TypePath)]
pub struct FarDirectoryLoader;

impl AssetLoader for FarDirectoryLoader {
    type Asset = FarDirectoryAsset;
    type Settings = ();
    type Error = FarLoadError;

    async fn load(&self, reader: &mut dyn Reader, (): &(), _: &mut LoadContext<'_>) -> Result<FarDirectoryAsset, FarLoadError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(FarDirectoryAsset(FarDirectory::from_bytes(&bytes)?))
    }

    fn extensions(&self) -> &[&str] {
        &["sffar"]
    }
}

/// One model's impostor atlas (CPU side; packed into arrays at runtime).
#[derive(Asset, TypePath, Debug)]
pub struct ImpostorAtlasAsset(pub ImpostorAtlas);

#[derive(Default, TypePath)]
pub struct ImpostorAtlasLoader;

impl AssetLoader for ImpostorAtlasLoader {
    type Asset = ImpostorAtlasAsset;
    type Settings = ();
    type Error = FarLoadError;

    async fn load(&self, reader: &mut dyn Reader, (): &(), _: &mut LoadContext<'_>) -> Result<ImpostorAtlasAsset, FarLoadError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(ImpostorAtlasAsset(ImpostorAtlas::from_bytes(&bytes)?))
    }

    fn extensions(&self) -> &[&str] {
        &["sfimp"]
    }
}
