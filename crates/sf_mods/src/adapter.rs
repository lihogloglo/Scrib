//! The content-pack side of the pipeline: synthesized mods, archive layers,
//! and record formats (mods.md "Interfaces").

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use sf_records::{RecordDb, SourceId};
use sf_vfs::{SourceIndex, Vfs, VfsPath, VfsSource};

use crate::{IssueKind, ModId, ModIssue, ModManifest};

/// Implemented by content packs (and by the framework's native record loader)
/// to plug game-specific content into the mod pipeline. Every method has a
/// default, so an adapter implements only what it provides.
pub trait ContentAdapter {
    /// Unique name; synthesized manifests carry it in [`ModManifest::adapter`].
    fn name(&self) -> &str;

    /// Synthesized mods for content the adapter finds by itself (legacy plugins
    /// and their archives, loose legacy data, the content pack's builtin mod).
    /// Problems go to `issues`.
    fn discover(&self, issues: &mut Vec<ModIssue>) -> Vec<ModManifest> {
        let _ = issues;
        Vec::new()
    }

    /// VFS layers (archives, generated sources) for a mod this adapter
    /// synthesized, lowest priority first. They are mounted below the
    /// manifest's own asset roots, if it has any.
    fn mount(&self, manifest: &ModManifest) -> Result<Vec<Arc<dyn VfsSource>>, String> {
        let _ = manifest;
        Ok(Vec::new())
    }

    /// Record file extensions this adapter applies (lowercase, no dot).
    fn record_extensions(&self) -> &[&str] {
        &[]
    }

    /// Registers the adapter's record kinds before any record file is applied.
    fn register_kinds(&self, db: &mut RecordDb) {
        let _ = db;
    }

    /// Applies one record file (an extension from [`Self::record_extensions`])
    /// into `cx.source`. Non-fatal findings go to [`RecordCx::warn`].
    fn apply_records(&self, cx: &mut RecordCx<'_>, file: &VfsPath) -> Result<(), RecordFileError> {
        let _ = cx;
        Err(RecordFileError::new(format!("{} does not apply record files ({file})", self.name())))
    }

    /// Runs once after every mod's records are applied (adapter-derived sources).
    fn finish_records(&self, db: &mut RecordDb) {
        let _ = db;
    }
}

/// A record file that failed to apply, with a position when the format has one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordFileError {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl RecordFileError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), line: None, column: None }
    }

    #[must_use]
    pub fn at(mut self, line: u32, column: u32) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

impl fmt::Display for RecordFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// A mod's own view of the VFS: the layers mounted for it.
///
/// A record file is read from the mod's own layers, never from a later mod
/// that ships the same path. Synthesized mods may also read from the whole
/// VFS (a legacy plugin lives in the shared data folder, not in its archive).
#[derive(Clone, Copy)]
pub struct ModFiles<'a> {
    pub vfs: &'a Vfs,
    pub layers: &'a Range<u16>,
    /// Fall back to the whole VFS when the mod's layers lack a path.
    pub shared: bool,
}

impl ModFiles<'_> {
    /// The layer that provides `path` for this mod (its own highest first).
    pub fn find(&self, path: &VfsPath) -> Option<SourceIndex> {
        let own = self.vfs.providers(path).iter().rev().find(|s| self.layers.contains(&s.0)).copied();
        own.or_else(|| if self.shared { self.vfs.resolve(path) } else { None })
    }

    pub fn contains(&self, path: &VfsPath) -> bool {
        self.find(path).is_some()
    }

    pub fn read(&self, path: &VfsPath) -> io::Result<Vec<u8>> {
        match self.find(path) {
            Some(source) => self.vfs.source(source).read(path),
            None => Err(io::Error::new(io::ErrorKind::NotFound, format!("{path} not found in the mod's files"))),
        }
    }

    /// The file on disk, for loaders that map or stream large files.
    pub fn real_path(&self, path: &VfsPath) -> Option<PathBuf> {
        self.find(path).and_then(|source| self.vfs.source(source).real_path(path))
    }
}

/// What a record loader gets for one mod.
pub struct RecordCx<'a> {
    pub manifest: &'a ModManifest,
    /// The record source of this mod (named by its id; depends on its dependencies' sources).
    pub source: SourceId,
    pub db: &'a mut RecordDb,
    pub files: ModFiles<'a>,
    pub(crate) sources: &'a HashMap<ModId, SourceId>,
    pub(crate) issues: &'a mut Vec<ModIssue>,
}

impl RecordCx<'_> {
    /// The record source of an earlier loaded mod (masters, dependencies).
    pub fn source_of(&self, id: &ModId) -> Option<SourceId> {
        self.sources.get(id).copied()
    }

    /// Reports a non-fatal finding in `file`.
    pub fn warn(&mut self, file: &VfsPath, message: impl Into<String>) {
        let kind = IssueKind::RecordWarning { path: file.to_string(), message: message.into() };
        self.issues.push(ModIssue::new(Some(&self.manifest.id), kind));
    }
}
