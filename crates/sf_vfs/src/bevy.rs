//! Bevy integration: serves a [`Vfs`] as an asset source.
//!
//! Register it as the default source so every `asset_server.load(..)` goes
//! through the mod layers:
//!
//! ```ignore
//! app.register_asset_source(AssetSourceId::Default, sf_vfs::bevy::asset_source(vfs));
//! app.add_plugins(DefaultPlugins); // must come after
//! ```
//!
//! Asset meta files (`<path>.meta`) are looked up in the VFS too, so mods can
//! ship loader settings next to their assets.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy_asset::io::{AssetReader, AssetReaderError, AssetSourceBuilder, PathStream, Reader, VecReader};

use crate::{DirEntry, Vfs, VfsPath};

/// An [`AssetReader`] over a shared, immutable [`Vfs`].
#[derive(Clone)]
pub struct VfsAssetReader {
    vfs: Arc<Vfs>,
}

impl VfsAssetReader {
    pub fn new(vfs: Arc<Vfs>) -> Self {
        Self { vfs }
    }

    // Reads are synchronous: archive sources are memory-mapped copies and
    // loose files are small. They run on Bevy's IO task pool, not the main
    // thread. Revisit if profiling shows pool starvation.
    fn read_bytes(&self, path: &Path, suffix: &str) -> Result<Vec<u8>, AssetReaderError> {
        let not_found = || AssetReaderError::NotFound(path.to_path_buf());
        let raw = path.to_str().ok_or_else(not_found)?;
        let vpath = VfsPath::new(&format!("{raw}{suffix}")).map_err(|_| not_found())?;
        if !self.vfs.contains(&vpath) {
            return Err(not_found());
        }
        self.vfs.read(&vpath).map_err(|e| AssetReaderError::Io(Arc::new(e)))
    }
}

impl AssetReader for VfsAssetReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        self.read_bytes(path, "").map(VecReader::new)
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        self.read_bytes(path, ".meta").map(VecReader::new)
    }

    async fn read_directory<'a>(&'a self, path: &'a Path) -> Result<Box<PathStream>, AssetReaderError> {
        let not_found = || AssetReaderError::NotFound(path.to_path_buf());
        let raw = path.to_str().ok_or_else(not_found)?;
        if !self.vfs.is_dir(raw) {
            return Err(not_found());
        }
        let entries: Vec<PathBuf> = self
            .vfs
            .read_dir(raw)
            .into_iter()
            .map(|entry| match entry {
                DirEntry::File(file) => PathBuf::from(file.as_str()),
                DirEntry::Dir(name) => path.join(name),
            })
            .collect();
        Ok(Box::new(futures_lite::stream::iter(entries)))
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        Ok(path.to_str().is_some_and(|raw| self.vfs.is_dir(raw)))
    }
}

/// An asset source builder serving `vfs`.
pub fn asset_source(vfs: Arc<Vfs>) -> AssetSourceBuilder {
    AssetSourceBuilder::new(move || Box::new(VfsAssetReader::new(vfs.clone())))
}

#[cfg(test)]
mod tests {
    use bevy_asset::io::AssetReader;
    use futures_lite::{StreamExt, future::block_on};

    use super::*;
    use crate::MemSource;

    fn reader() -> VfsAssetReader {
        let vfs = Vfs::builder()
            .mount(
                MemSource::new("s")
                    .with("meshes/a.nif", "nif")
                    .with("meshes/a.nif.meta", "meta")
                    .with("meshes/x/b.nif", "b"),
            )
            .build();
        VfsAssetReader::new(Arc::new(vfs))
    }

    #[test]
    fn reads_files_and_meta_case_insensitively() {
        let r = reader();
        block_on(async {
            let mut bytes = Vec::new();
            r.read(Path::new("Meshes/A.nif")).await.unwrap().read_to_end(&mut bytes).await.unwrap();
            assert_eq!(bytes, b"nif");
            assert_eq!(r.read_meta_bytes(Path::new("meshes/a.nif")).await.unwrap(), b"meta");
            assert!(matches!(r.read_meta(Path::new("meshes/x/b.nif")).await, Err(AssetReaderError::NotFound(_))));
            assert!(matches!(r.read(Path::new("nope.nif")).await, Err(AssetReaderError::NotFound(_))));
        });
    }

    #[test]
    fn lists_directories() {
        let r = reader();
        block_on(async {
            assert!(r.is_directory(Path::new("meshes")).await.unwrap());
            assert!(!r.is_directory(Path::new("meshes/a.nif")).await.unwrap());
            let entries: Vec<PathBuf> = r.read_directory(Path::new("meshes")).await.unwrap().collect().await;
            assert_eq!(
                entries,
                [PathBuf::from("meshes/a.nif"), PathBuf::from("meshes/a.nif.meta"), Path::new("meshes").join("x")]
            );
            assert!(r.read_directory(Path::new("missing")).await.is_err());
        });
    }
}
