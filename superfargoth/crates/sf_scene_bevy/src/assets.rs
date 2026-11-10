//! Asset loaders for static-scene bake products, served by `baked://`.
//!
//! One loader, for one product: a cell's instance table (`.sfscene`,
//! [`sf_scene::CELLS_EXT`]). The other product, a chain's geometry
//! (`scene.chains`, [`sf_scene::CHAINS_EXT`]), is an `sf_model` container and
//! is read by `sf_model_bevy`'s existing `.sfmesh` loader: no asset type and
//! no loader of our own (reuse before building — the step still needs an
//! entry in the extensions map, [`crate::baked_extensions`]).
//!
//! Decoding runs on Bevy's loader tasks, off the main thread; so does the
//! table's validation, so a corrupt table is an ordinary `LoadState::Failed`
//! instead of a second error path at stream-in (static-scene.md §5.3).
//!
//! Template: `sf_lod_bevy::FarDirectoryLoader` (`sf_lod_bevy/src/assets.rs`).

use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetLoader, LoadContext};
use bevy_reflect::TypePath;
use sf_scene::{StaticCellTableView, TableError};

#[derive(Debug, thiserror::Error)]
pub enum SceneLoadError {
    #[error("reading a static-scene product: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Table(#[from] TableError),
}

/// One cell's baked instance table, bytes verbatim.
///
/// [`StaticCellTableView`] borrows the bytes (its instance section *is* the GPU
/// layout, so nothing is decoded on load), which is why the asset owns the
/// `Vec` and hands out views through [`Self::view`].
#[derive(Asset, TypePath, Debug)]
pub struct SceneTableAsset(pub Vec<u8>);

impl SceneTableAsset {
    /// The borrowing reader over the bytes. Cannot fail for an asset the loader
    /// produced (it parsed them already), so B3 may `expect` it; it stays a
    /// `Result` because the `Vec` is public.
    pub fn view(&self) -> Result<StaticCellTableView<'_>, TableError> {
        StaticCellTableView::parse(&self.0)
    }

    /// Byte size of the table, for the upload estimate.
    pub fn bytes(&self) -> u64 {
        self.0.len() as u64
    }
}

/// Loads a cell's instance table (`scene.cells`).
#[derive(Default, TypePath)]
pub struct SceneTableLoader;

/// `extensions()` returns a `&[&str]`; [`sf_scene::CELLS_EXT`] is the one
/// source of truth for the string, so this never spells it out.
const EXTENSIONS: &[&str] = &[sf_scene::CELLS_EXT];

impl AssetLoader for SceneTableLoader {
    type Asset = SceneTableAsset;
    type Settings = ();
    type Error = SceneLoadError;

    async fn load(&self, reader: &mut dyn Reader, (): &(), _: &mut LoadContext<'_>) -> Result<SceneTableAsset, SceneLoadError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        // Checks the header, the size and every cross-reference, then drops the
        // view: the runtime re-parses the owned bytes when it needs one.
        StaticCellTableView::parse(&bytes)?;
        Ok(SceneTableAsset(bytes))
    }

    fn extensions(&self) -> &[&str] {
        EXTENSIONS
    }
}
