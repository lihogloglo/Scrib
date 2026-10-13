//! Layered virtual filesystem.
//!
//! Sources (loose directories, archives, generated content) are mounted in
//! priority order; for any path, the last-mounted source that has it wins.
//! The index is built once and is immutable afterwards, so lookups are
//! lock-free and the [`Vfs`] can be shared across threads. Changing the mod
//! list means building a new `Vfs`; a file change on disk (hot reload) means a
//! [`Vfs::rebuild`] of the changed directory layers, published through
//! [`SharedVfs`] (with the `watch` feature, driven by `watch::VfsWatcher`).
//!
//! Archive formats are not built in: adapters implement [`VfsSource`].

mod dir;
mod mem;
mod path;
mod rebuild;
mod shared;

#[cfg(feature = "watch")]
pub mod watch;

#[cfg(feature = "bevy")]
pub mod bevy;

pub use dir::DirSource;
pub use mem::MemSource;
pub use path::{PathError, VfsPath};
pub use rebuild::{ChangeKind, VfsChange};
pub use shared::SharedVfs;

use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

/// A provider of files for the VFS.
pub trait VfsSource: Send + Sync + 'static {
    /// Human-readable name used in diagnostics and provenance.
    fn label(&self) -> &str;

    /// Calls `visit` once per file this source provides.
    fn visit_files(&self, visit: &mut dyn FnMut(VfsPath));

    /// Reads a file this source reported in [`VfsSource::visit_files`].
    fn read(&self, path: &VfsPath) -> io::Result<Vec<u8>>;

    /// The file's location on disk, if it is a plain file there.
    fn real_path(&self, _path: &VfsPath) -> Option<PathBuf> {
        None
    }

    /// The size and modification time of a file on disk as indexed (no I/O),
    /// for stat-keyed fingerprints; `None` for sources without files on disk.
    fn stat(&self, _path: &VfsPath) -> Option<FileStat> {
        None
    }

    /// The directory on disk this source mirrors, for hot-reload watching.
    fn watch_root(&self) -> Option<&std::path::Path> {
        None
    }

    /// A fresh copy reflecting the files on disk now (hot reload); `None`
    /// when the source can't change (archives, memory).
    fn rescan(&self) -> io::Result<Option<Arc<dyn VfsSource>>> {
        Ok(None)
    }
}

/// A file's size and modification time (nanoseconds since the Unix epoch).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileStat {
    pub size: u64,
    pub mtime_ns: u128,
}

impl FileStat {
    pub fn of(meta: &std::fs::Metadata) -> Self {
        let mtime = meta.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok());
        Self { size: meta.len(), mtime_ns: mtime.map_or(0, |d| d.as_nanos()) }
    }
}

/// Position of a source in mount order (higher wins).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceIndex(pub u16);

/// Collects sources in priority order and builds a [`Vfs`].
#[derive(Default)]
pub struct VfsBuilder {
    sources: Vec<Arc<dyn VfsSource>>,
}

impl VfsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mounts a source above everything mounted so far.
    #[must_use]
    pub fn mount(mut self, source: impl VfsSource) -> Self {
        self.sources.push(Arc::new(source));
        self
    }

    /// Mounts an already shared source above everything mounted so far.
    #[must_use]
    pub fn mount_shared(mut self, source: Arc<dyn VfsSource>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn build(self) -> Vfs {
        let mut index: BTreeMap<VfsPath, Vec<SourceIndex>> = BTreeMap::new();
        for (i, source) in self.sources.iter().enumerate() {
            let si = SourceIndex(u16::try_from(i).expect("too many VFS sources"));
            source.visit_files(&mut |path| {
                let providers = index.entry(path).or_default();
                if providers.last() != Some(&si) {
                    providers.push(si);
                }
            });
        }
        Vfs { sources: self.sources, index }
    }
}

/// An immutable, layered view over all mounted sources.
pub struct Vfs {
    sources: Vec<Arc<dyn VfsSource>>,
    /// Every path, with the sources providing it in ascending priority.
    index: BTreeMap<VfsPath, Vec<SourceIndex>>,
}

/// A directory entry returned by [`Vfs::read_dir`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirEntry {
    File(VfsPath),
    Dir(String),
}

impl DirEntry {
    /// The entry's own name (last path segment).
    pub fn name(&self) -> &str {
        match self {
            Self::File(path) => path.file_name(),
            Self::Dir(name) => name,
        }
    }
}

/// Per-source statistics.
#[derive(Clone, Debug)]
pub struct SourceStats {
    pub label: String,
    /// Files this source provides.
    pub files: usize,
    /// Files where this source is the winner.
    pub winning: usize,
}

impl Vfs {
    pub fn builder() -> VfsBuilder {
        VfsBuilder::new()
    }

    /// Number of distinct paths.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn source(&self, index: SourceIndex) -> &dyn VfsSource {
        &*self.sources[usize::from(index.0)]
    }

    pub fn contains(&self, path: &VfsPath) -> bool {
        self.index.contains_key(path)
    }

    /// The source that wins for `path`.
    pub fn resolve(&self, path: &VfsPath) -> Option<SourceIndex> {
        self.index.get(path).and_then(|p| p.last().copied())
    }

    /// Every source providing `path`, lowest priority first.
    pub fn providers(&self, path: &VfsPath) -> &[SourceIndex] {
        self.index.get(path).map_or(&[], Vec::as_slice)
    }

    /// Reads the winning version of `path`.
    pub fn read(&self, path: &VfsPath) -> io::Result<Vec<u8>> {
        match self.resolve(path) {
            Some(source) => self.source(source).read(path),
            None => Err(io::Error::new(io::ErrorKind::NotFound, format!("{path} not found in VFS"))),
        }
    }

    /// Resolves `path`, letting files with an alternate extension take its
    /// slot when they come from the same or a higher-priority source.
    ///
    /// This is how modern formats replace legacy ones without renaming
    /// references: with alternates `["glb"]`, a mod shipping `foo.glb`
    /// replaces `foo.nif` from the base game, while a later mod shipping a
    /// new `foo.nif` still wins over an older `foo.glb`. On equal priority
    /// the alternate wins, earlier alternates first.
    pub fn resolve_preferring(&self, path: &VfsPath, alternates: &[&str]) -> Option<(VfsPath, SourceIndex)> {
        // Rank by (source priority, tie-break); the original ranks lowest on ties.
        let original = self.resolve(path).map(|s| ((s, 0), path.clone()));
        let alts = alternates.iter().enumerate().filter_map(|(i, ext)| {
            let candidate = path.with_extension(ext);
            self.resolve(&candidate).map(|s| ((s, alternates.len() - i), candidate))
        });
        original.into_iter().chain(alts).max_by_key(|(rank, _)| *rank).map(|((s, _), p)| (p, s))
    }

    /// Immediate children of a directory (`""` for the root), sorted, with
    /// subdirectories listed once.
    pub fn read_dir(&self, dir: &str) -> Vec<DirEntry> {
        let Some(prefix) = dir_prefix(dir) else { return Vec::new() };
        let mut entries = Vec::new();
        let mut last_dir: Option<&str> = None;
        // Paths under a prefix are contiguous in sorted order.
        for path in self.index.range(VfsPath::raw(prefix.clone())..).map(|(p, _)| p) {
            let Some(rest) = path.as_str().strip_prefix(&prefix) else { break };
            match rest.split_once('/') {
                Some((sub, _)) => {
                    if last_dir != Some(sub) {
                        entries.push(DirEntry::Dir(sub.to_owned()));
                        last_dir = Some(sub);
                    }
                }
                None => entries.push(DirEntry::File(path.clone())),
            }
        }
        // Full-path order differs from name order ("a-b/" sorts before "a/").
        entries.sort_by(|a, b| a.name().cmp(b.name()));
        entries
    }

    /// Whether `dir` contains any file (directories are implicit).
    pub fn is_dir(&self, dir: &str) -> bool {
        let Some(prefix) = dir_prefix(dir) else { return false };
        self.index.range(VfsPath::raw(prefix.clone())..).next().is_some_and(|(k, _)| k.as_str().starts_with(&prefix))
    }

    /// Iterates all paths in sorted order.
    pub fn paths(&self) -> impl Iterator<Item = &VfsPath> {
        self.index.keys()
    }

    pub fn stats(&self) -> Vec<SourceStats> {
        let mut stats: Vec<SourceStats> = self
            .sources
            .iter()
            .map(|s| SourceStats { label: s.label().to_owned(), files: 0, winning: 0 })
            .collect();
        for providers in self.index.values() {
            for p in providers {
                stats[usize::from(p.0)].files += 1;
            }
            if let Some(w) = providers.last() {
                stats[usize::from(w.0)].winning += 1;
            }
        }
        stats
    }
}

/// `"Meshes\\X"` -> `"meshes/x/"`, `""` -> `""`; `None` if invalid.
fn dir_prefix(dir: &str) -> Option<String> {
    match VfsPath::new(dir) {
        Ok(p) => Some(format!("{p}/")),
        Err(PathError::Empty) => Some(String::new()),
        Err(PathError::ParentSegment(_)) => None,
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_hot;
