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
//!
//! Hot reload: [`shared_asset_source`] reads whatever snapshot a [`SharedVfs`]
//! holds and, when Bevy watches for changes (its `file_watcher` feature or
//! `AssetPlugin::watch_for_changes_override`), connects Bevy's event channel
//! to [`AssetEvents`]; the VFS watcher sends the *winning* changes there, so
//! Bevy reloads loaded assets whose winner changed.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};

use bevy_asset::io::{
    AssetReader, AssetReaderError, AssetSourceBuilder, AssetSourceEvent, AssetWatcher, PathStream, Reader, VecReader,
};

use crate::{ChangeKind, DirEntry, SharedVfs, Vfs, VfsChange, VfsPath};

/// An [`AssetReader`] over the current snapshot of a [`SharedVfs`].
#[derive(Clone)]
pub struct VfsAssetReader {
    vfs: SharedVfs,
}

impl VfsAssetReader {
    pub fn new(vfs: Arc<Vfs>) -> Self {
        Self { vfs: SharedVfs::new(vfs) }
    }

    pub fn shared(vfs: SharedVfs) -> Self {
        Self { vfs }
    }

    // Reads are synchronous: archive sources are memory-mapped copies and
    // loose files are small. They run on Bevy's IO task pool, not the main
    // thread. Revisit if profiling shows pool starvation.
    fn read_bytes(&self, path: &Path, suffix: &str) -> Result<Vec<u8>, AssetReaderError> {
        let not_found = || AssetReaderError::NotFound(path.to_path_buf());
        let raw = path.to_str().ok_or_else(not_found)?;
        let vpath = VfsPath::new(&format!("{raw}{suffix}")).map_err(|_| not_found())?;
        let vfs = self.vfs.load();
        if !vfs.contains(&vpath) {
            return Err(not_found());
        }
        vfs.read(&vpath).map_err(|e| AssetReaderError::Io(Arc::new(e)))
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
        let vfs = self.vfs.load();
        if !vfs.is_dir(raw) {
            return Err(not_found());
        }
        let entries: Vec<PathBuf> = vfs
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
        Ok(path.to_str().is_some_and(|raw| self.vfs.load().is_dir(raw)))
    }
}

/// An asset source builder serving `vfs` (a fixed snapshot, no hot reload).
pub fn asset_source(vfs: Arc<Vfs>) -> AssetSourceBuilder {
    AssetSourceBuilder::new(move || Box::new(VfsAssetReader::new(vfs.clone())))
}

/// An asset source builder serving the current snapshot of `vfs`, with Bevy's
/// change events connected to `events` when Bevy watches for changes.
pub fn shared_asset_source(vfs: SharedVfs, events: AssetEvents) -> AssetSourceBuilder {
    AssetSourceBuilder::new(move || Box::new(VfsAssetReader::shared(vfs.clone()))).with_watcher(move |sender| {
        events.connect(sender);
        Some(Box::new(Connected))
    })
}

/// Bevy's side of the watcher: the real watching happens in the VFS watcher,
/// which sends through [`AssetEvents`].
struct Connected;

impl AssetWatcher for Connected {}

/// Bevy asset-source event channels fed with the VFS's winning changes.
#[derive(Clone, Default)]
pub struct AssetEvents(Arc<Mutex<Vec<async_channel::Sender<AssetSourceEvent>>>>);

impl AssetEvents {
    fn connect(&self, sender: async_channel::Sender<AssetSourceEvent>) {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).push(sender);
    }

    /// Whether Bevy connected a channel (it watches for changes).
    pub fn is_connected(&self) -> bool {
        !self.0.lock().unwrap_or_else(PoisonError::into_inner).is_empty()
    }

    /// Sends one event per change to every connected channel; returns how
    /// many events were sent.
    pub fn send(&self, changes: &[VfsChange]) -> usize {
        let mut senders = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        senders.retain(|s| !s.is_closed());
        let mut sent = 0;
        for sender in senders.iter() {
            for change in changes {
                if sender.try_send(source_event(change)).is_ok() {
                    sent += 1;
                }
            }
        }
        sent
    }
}

/// The Bevy event for one winning change; `<path>.meta` files map to the meta
/// events of `<path>`. Paths are VFS paths (lowercase), the form asset loads
/// should use so the asset server matches them.
pub fn source_event(change: &VfsChange) -> AssetSourceEvent {
    let raw = change.path.as_str();
    match (raw.strip_suffix(".meta"), change.kind) {
        (None, ChangeKind::Added) => AssetSourceEvent::AddedAsset(PathBuf::from(raw)),
        (None, ChangeKind::Modified) => AssetSourceEvent::ModifiedAsset(PathBuf::from(raw)),
        (None, ChangeKind::Removed) => AssetSourceEvent::RemovedAsset(PathBuf::from(raw)),
        (Some(asset), ChangeKind::Added) => AssetSourceEvent::AddedMeta(PathBuf::from(asset)),
        (Some(asset), ChangeKind::Modified) => AssetSourceEvent::ModifiedMeta(PathBuf::from(asset)),
        (Some(asset), ChangeKind::Removed) => AssetSourceEvent::RemovedMeta(PathBuf::from(asset)),
    }
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

    #[test]
    fn swapped_snapshot_is_read_and_winning_changes_reach_bevy() {
        let shared = SharedVfs::new(Arc::new(Vfs::builder().mount(MemSource::new("a").with("t/x.png", "old")).build()));
        let reader = VfsAssetReader::shared(shared.clone());
        let events = AssetEvents::default();
        let (tx, rx) = async_channel::unbounded();
        events.connect(tx);
        assert!(events.is_connected());
        shared.store(Arc::new(Vfs::builder().mount(MemSource::new("b").with("t/x.png", "new")).build()));
        block_on(async {
            let mut bytes = Vec::new();
            reader.read(Path::new("t/x.png")).await.unwrap().read_to_end(&mut bytes).await.unwrap();
            assert_eq!(bytes, b"new");
        });
        let changes = [
            VfsChange { path: VfsPath::new("t/x.png").unwrap(), kind: ChangeKind::Modified },
            VfsChange { path: VfsPath::new("t/x.png.meta").unwrap(), kind: ChangeKind::Added },
        ];
        assert_eq!(events.send(&changes), 2);
        assert!(matches!(rx.try_recv(), Ok(AssetSourceEvent::ModifiedAsset(p)) if p == Path::new("t/x.png")));
        assert!(matches!(rx.try_recv(), Ok(AssetSourceEvent::AddedMeta(p)) if p == Path::new("t/x.png")));
        drop(rx);
        assert_eq!(events.send(&changes), 0, "closed channels are dropped");
    }
}
