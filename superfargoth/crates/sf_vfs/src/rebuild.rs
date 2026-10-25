//! Watcher-driven rebuilds: rescan the changed directory layers, rebuild the
//! index, and list the paths whose winning file changed (mods.md "Hot reload").

use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::{FileStat, SourceIndex, Vfs, VfsBuilder, VfsPath, VfsSource};

/// How a path's winning file changed between two snapshots.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChangeKind {
    /// The path is new.
    Added,
    /// The winning file was edited or replaced (a new winner, or new contents).
    Modified,
    /// No layer provides the path any more.
    Removed,
}

/// One path whose winning file changed. Edits shadowed by a higher layer are
/// not changes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VfsChange {
    pub path: VfsPath,
    pub kind: ChangeKind,
}

impl Vfs {
    /// Directories on disk the layers mirror ([`VfsSource::watch_root`]),
    /// deduplicated, in mount order.
    pub fn watch_roots(&self) -> Vec<PathBuf> {
        let mut roots: Vec<PathBuf> = Vec::new();
        for root in self.sources.iter().filter_map(|s| s.watch_root()) {
            if !roots.iter().any(|r| r == root) {
                roots.push(root.to_path_buf());
            }
        }
        roots
    }

    /// The next snapshot: every layer whose watch root `dirty` accepts is
    /// rescanned ([`VfsSource::rescan`]), all others are shared as they are,
    /// so layer positions (and mods' layer ranges) stay valid. Returns the new
    /// snapshot and the paths whose winning file changed.
    ///
    /// Only paths of rescanned layers can change winners, so the diff looks at
    /// those, comparing the winner's layer and its indexed stat (size, mtime).
    pub fn rebuild(&self, dirty: &dyn Fn(&Path) -> bool) -> io::Result<(Vfs, Vec<VfsChange>)> {
        let mut sources: Vec<Arc<dyn VfsSource>> = Vec::with_capacity(self.sources.len());
        let mut rescanned = Vec::new();
        for (i, source) in self.sources.iter().enumerate() {
            let fresh = match source.watch_root() {
                Some(root) if dirty(root) => source.rescan()?,
                _ => None,
            };
            match fresh {
                Some(fresh) => {
                    rescanned.push(i);
                    sources.push(fresh);
                }
                None => sources.push(Arc::clone(source)),
            }
        }
        let next = VfsBuilder { sources }.build();
        let mut candidates = BTreeSet::new();
        for &i in &rescanned {
            self.sources[i].visit_files(&mut |p| {
                candidates.insert(p);
            });
            next.sources[i].visit_files(&mut |p| {
                candidates.insert(p);
            });
        }
        let changes = candidates
            .into_iter()
            .filter_map(|path| {
                let kind = match (self.winner(&path), next.winner(&path)) {
                    (None, Some(_)) => ChangeKind::Added,
                    (Some(_), None) => ChangeKind::Removed,
                    (Some(a), Some(b)) if a != b => ChangeKind::Modified,
                    _ => return None,
                };
                Some(VfsChange { path, kind })
            })
            .collect();
        Ok((next, changes))
    }

    /// The winning layer of `path` and its indexed stat.
    fn winner(&self, path: &VfsPath) -> Option<(SourceIndex, Option<FileStat>)> {
        let source = self.resolve(path)?;
        Some((source, self.source(source).stat(path)))
    }
}
