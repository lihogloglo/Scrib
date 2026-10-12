//! Stat-keyed memo of file content hashes, so a warm start hashes nothing.
//!
//! Same idea as git's index and ninja's restat: a file whose identity
//! (source, path, size, modification time) hasn't changed keeps its hash.
//! Archive members use the archive's stat plus their offset as identity.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

use serde::{Deserialize, Serialize};

use crate::Digest;

/// Identity of a file's current content, as cheap filesystem metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatKey {
    /// Which source provides the file (a mod folder, an archive path, …).
    pub source: String,
    /// Path inside the source (or `path@offset` for archive members).
    pub path: String,
    pub size: u64,
    /// Modification time in nanoseconds since the Unix epoch (0 if unknown;
    /// the size and source still change with most edits).
    pub mtime_ns: u128,
}

/// A persistent `StatKey -> Digest` table. Thread-safe.
pub struct FileHashMemo {
    file: PathBuf,
    entries: Mutex<HashMap<StatKey, Digest>>,
    dirty: Mutex<bool>,
}

impl FileHashMemo {
    /// Loads the memo stored at `file` (empty if absent or unreadable).
    pub fn load(file: impl Into<PathBuf>) -> Self {
        let file = file.into();
        let entries = fs::read(&file)
            .ok()
            .and_then(|bytes| postcard::from_bytes::<Vec<(StatKey, Digest)>>(&bytes).ok())
            .map(|v| v.into_iter().collect())
            .unwrap_or_default();
        Self { file, entries: Mutex::new(entries), dirty: Mutex::new(false) }
    }

    /// The content hash for `key`, calling `read` only when the stat is new.
    pub fn fingerprint(&self, key: StatKey, read: impl FnOnce() -> io::Result<Vec<u8>>) -> io::Result<Digest> {
        if let Some(d) = self.entries.lock().unwrap_or_else(PoisonError::into_inner).get(&key) {
            return Ok(*d);
        }
        let digest = Digest::of(&read()?);
        self.entries.lock().unwrap_or_else(PoisonError::into_inner).insert(key, digest);
        *self.dirty.lock().unwrap_or_else(PoisonError::into_inner) = true;
        Ok(digest)
    }

    /// Writes the memo back if anything changed.
    pub fn save(&self) -> io::Result<()> {
        let mut dirty = self.dirty.lock().unwrap_or_else(PoisonError::into_inner);
        if !*dirty {
            return Ok(());
        }
        let mut entries: Vec<(StatKey, Digest)> = self
            .entries
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        // Deterministic file contents.
        entries.sort_by(|a, b| (&a.0.source, &a.0.path).cmp(&(&b.0.source, &b.0.path)));
        let bytes = postcard::to_stdvec(&entries).map_err(io::Error::other)?;
        if let Some(dir) = self.file.parent() {
            fs::create_dir_all(dir)?;
        }
        let tmp = self.file.with_extension("tmp");
        fs::write(&tmp, bytes)?;
        fs::rename(&tmp, &self.file)?;
        *dirty = false;
        Ok(())
    }

    /// The stat key of a plain file on disk.
    pub fn stat_file(source: &str, path: &str, real_path: &Path) -> io::Result<StatKey> {
        let meta = fs::metadata(real_path)?;
        let mtime_ns = meta
            .modified()
            .ok()
            .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_nanos());
        Ok(StatKey { source: source.to_owned(), path: path.to_owned(), size: meta.len(), mtime_ns })
    }
}
