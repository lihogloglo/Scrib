//! The `baked://` asset source: bake products by (step, subject).
//!
//! Asset path: `baked://<step>/<subject>[.<ext>]`, e.g.
//! `baked://mesh.cook/meshes/x/rock.nif.sfmesh`. The optional extension
//! (registered per step) is what makes Bevy pick the right loader; it is
//! stripped before the manifest lookup.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, PoisonError, RwLock};

use bevy_asset::io::{
    AssetReader, AssetReaderError, AssetSourceBuilder, PathStream, Reader, VecReader,
};
use sf_bake::{Manifest, Store};

/// Name of the asset source (`baked://`).
pub const BAKED_SOURCE: &str = "baked";

/// The live profile manifest, shared with the reader so a re-bake can swap
/// it without rebuilding the asset source.
#[derive(Clone, Default)]
pub struct BakedManifest(Arc<RwLock<Manifest>>);

impl BakedManifest {
    pub fn new(manifest: Manifest) -> Self {
        Self(Arc::new(RwLock::new(manifest)))
    }

    pub fn replace(&self, manifest: Manifest) {
        *self.0.write().unwrap_or_else(PoisonError::into_inner) = manifest;
    }

    pub fn get(&self, step: &str, subject: &str) -> Option<sf_bake::Digest> {
        self.0
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(step, subject)
    }
}

/// Reads products out of a bake [`Store`].
#[derive(Clone)]
pub struct BakedReader {
    store: Store,
    manifest: BakedManifest,
    /// step id -> file extension appended to asset paths.
    extensions: Arc<BTreeMap<String, String>>,
}

impl BakedReader {
    pub fn new(
        store: Store,
        manifest: BakedManifest,
        extensions: BTreeMap<String, String>,
    ) -> Self {
        Self {
            store,
            manifest,
            extensions: Arc::new(extensions),
        }
    }

    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, AssetReaderError> {
        let not_found = || AssetReaderError::NotFound(path.to_path_buf());
        let raw = path.to_str().ok_or_else(not_found)?.replace('\\', "/");
        let (step, rest) = raw.split_once('/').ok_or_else(not_found)?;
        let subject = match self.extensions.get(step) {
            Some(ext) => rest
                .strip_suffix(ext.as_str())
                .and_then(|s| s.strip_suffix('.'))
                .unwrap_or(rest),
            None => rest,
        };
        let digest = self.manifest.get(step, subject).ok_or_else(not_found)?;
        self.store
            .get(&digest)
            .map_err(|e| AssetReaderError::Io(Arc::new(e)))
    }
}

impl AssetReader for BakedReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        self.read_bytes(path).map(VecReader::new)
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        Err::<VecReader, _>(AssetReaderError::NotFound(path.to_path_buf()))
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        Err(AssetReaderError::NotFound(path.to_path_buf()))
    }

    async fn is_directory<'a>(&'a self, _path: &'a Path) -> Result<bool, AssetReaderError> {
        Ok(false)
    }
}

/// An asset source builder serving bake products. Register it under
/// [`BAKED_SOURCE`] before `AssetPlugin`.
pub fn baked_source(
    store: Store,
    manifest: BakedManifest,
    extensions: BTreeMap<String, String>,
) -> AssetSourceBuilder {
    let reader = BakedReader::new(store, manifest, extensions);
    AssetSourceBuilder::new(move || Box::new(reader.clone()))
}

/// The asset path of a product: `baked://<step>/<subject>[.<ext>]`.
pub fn baked_path(step: &str, subject: &str, ext: Option<&str>) -> String {
    match ext {
        Some(ext) => format!("{BAKED_SOURCE}://{step}/{subject}.{ext}"),
        None => format!("{BAKED_SOURCE}://{step}/{subject}"),
    }
}
