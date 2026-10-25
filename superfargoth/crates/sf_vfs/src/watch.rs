//! The hot-reload watcher (feature `watch`, dev builds; mods.md "Hot reload").
//!
//! One debounced `notify` watcher (`notify-debouncer-full`, the crate and
//! version Bevy's own `file_watcher` uses) covers every directory layer of the
//! current snapshot. A batch of events marks the layers whose roots contain a
//! changed path; a worker thread rebuilds those ([`Vfs::rebuild`]), swaps the
//! [`SharedVfs`] snapshot, and hands the winning changes to a callback, which
//! routes them (Bevy asset events, script and record reloads).
//!
//! Why not Bevy's `FileWatcher` directly: it maps one directory to one asset
//! source and forwards raw directory events, while the VFS is one source over
//! many layers where an edit shadowed by a higher layer must emit nothing.
//!
//! Platform notes: on Windows `notify` uses `ReadDirectoryChangesW`; editors
//! that save through a temporary file and rename produce remove/create/rename
//! bursts, and a full event buffer produces a "rescan" event. Both are covered
//! because a batch only marks layers dirty and the rebuild compares the
//! rescanned listing (stats) with the previous one. Mods added to or removed
//! from the profile, archives and legacy plugins are not watched (restart).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify_debouncer_full::notify::{self, RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};

use crate::{SharedVfs, Vfs, VfsChange};

/// The default debounce window: editors write in bursts; well under a second
/// end to end (MS-1: "updates within a second").
pub const DEFAULT_DEBOUNCE: Duration = Duration::from_millis(150);

/// A published snapshot and what changed in it.
pub struct VfsUpdate {
    /// The new snapshot (already stored in the [`SharedVfs`]).
    pub vfs: Arc<Vfs>,
    /// Paths whose winning file changed, sorted.
    pub changes: Vec<VfsChange>,
    /// Rescan + index rebuild + diff time.
    pub rebuild: Duration,
}

/// What the watcher reports to its callback.
pub enum WatchEvent {
    Updated(VfsUpdate),
    /// A watcher or rescan error; the previous snapshot stays current.
    Failed(String),
}

/// Watches the directory layers of a [`SharedVfs`] for as long as it lives.
pub struct VfsWatcher {
    _debouncer: Debouncer<RecommendedWatcher, RecommendedCache>,
    roots: Vec<PathBuf>,
}

impl VfsWatcher {
    /// Starts watching every [`Vfs::watch_roots`] of the current snapshot.
    /// `on_event` runs on the watcher's worker thread after each swap that
    /// changed a winner (and on errors); batches without winning changes
    /// (shadowed edits, metadata-only touches) are silent.
    pub fn start(
        shared: SharedVfs,
        debounce: Duration,
        mut on_event: impl FnMut(WatchEvent) + Send + 'static,
    ) -> notify::Result<Self> {
        let roots = shared.load().watch_roots();
        let (tx, rx) = mpsc::channel::<Batch>();
        let mut debouncer = new_debouncer(debounce, None, move |result: DebounceEventResult| {
            let batch = match result {
                Ok(events) => {
                    let rescan = events.iter().any(|e| e.need_rescan());
                    Batch::Paths { rescan, paths: events.into_iter().flat_map(|e| e.event.paths).collect() }
                }
                Err(errors) => Batch::Errors(errors.iter().map(ToString::to_string).collect()),
            };
            let _ = tx.send(batch);
        })?;
        for root in &roots {
            debouncer.watch(root, RecursiveMode::Recursive)?;
        }
        std::thread::Builder::new()
            .name("sf_vfs watcher".into())
            .spawn(move || worker(&shared, &rx, &mut on_event))
            .map_err(|e| notify::Error::generic(&e.to_string()))?;
        Ok(Self { _debouncer: debouncer, roots })
    }

    /// The watched directories.
    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }
}

enum Batch {
    Paths { rescan: bool, paths: Vec<PathBuf> },
    Errors(Vec<String>),
}

/// Rebuilds and swaps per batch (coalescing batches that queued up meanwhile)
/// until the debouncer, and with it the sender, is dropped.
fn worker(shared: &SharedVfs, rx: &mpsc::Receiver<Batch>, on_event: &mut dyn FnMut(WatchEvent)) {
    while let Ok(first) = rx.recv() {
        let (mut rescan, mut paths) = (false, Vec::new());
        for batch in std::iter::once(first).chain(rx.try_iter()) {
            match batch {
                Batch::Paths { rescan: r, paths: p } => {
                    rescan |= r;
                    paths.extend(p);
                }
                Batch::Errors(errors) => on_event(WatchEvent::Failed(errors.join("; "))),
            }
        }
        if !rescan && paths.is_empty() {
            continue;
        }
        let start = Instant::now();
        let current = shared.load();
        let dirty = |root: &Path| rescan || paths.iter().any(|p| p.starts_with(root));
        match current.rebuild(&dirty) {
            Ok((next, changes)) => {
                let next = Arc::new(next);
                shared.store(Arc::clone(&next));
                if !changes.is_empty() {
                    on_event(WatchEvent::Updated(VfsUpdate { vfs: next, changes, rebuild: start.elapsed() }));
                }
            }
            Err(e) => on_event(WatchEvent::Failed(format!("rescan failed: {e}"))),
        }
    }
}
