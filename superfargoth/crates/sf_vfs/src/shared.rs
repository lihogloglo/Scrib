//! The current VFS snapshot, swappable while readers keep running
//! (mods.md "Hot reload", ARCHITECTURE §10).

use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::Vfs;

/// A shared handle to the current [`Vfs`] snapshot.
///
/// Readers call [`SharedVfs::load`] and keep the returned `Arc` for as long as
/// they need a consistent view; a hot-reload rebuild [`store`](SharedVfs::store)s
/// the next snapshot. Both sides are lock-free (`arc-swap`): a reader never waits
/// for a swap, and a swap never waits for readers, which finish on the snapshot
/// they loaded. Clones share the same slot.
#[derive(Clone)]
pub struct SharedVfs(Arc<ArcSwap<Vfs>>);

impl SharedVfs {
    pub fn new(vfs: Arc<Vfs>) -> Self {
        Self(Arc::new(ArcSwap::new(vfs)))
    }

    /// The current snapshot.
    pub fn load(&self) -> Arc<Vfs> {
        self.0.load_full()
    }

    /// Publishes `vfs` as the current snapshot; returns the previous one.
    pub fn store(&self, vfs: Arc<Vfs>) -> Arc<Vfs> {
        self.0.swap(vfs)
    }
}

impl From<Arc<Vfs>> for SharedVfs {
    fn from(vfs: Arc<Vfs>) -> Self {
        Self::new(vfs)
    }
}

impl std::fmt::Debug for SharedVfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedVfs").field("files", &self.0.load().len()).finish()
    }
}
