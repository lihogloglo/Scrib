//! Atomic file replacement (save.md "Corruption safety").
//!
//! A save is never written in place: write `<file>.tmp` beside the target, `fsync` it, read it
//! back and open it as a [`SaveFile`] (preamble, TOC, every section hash), optionally copy the
//! old file to `<file>.bak`, rename the temp file over the target in one step, then `fsync` the
//! directory (LWN 323076: `rename` is atomic for the name, not the data). The target name always
//! holds a complete save: the backup is a copy, never a move (as POSIX `link` + `rename` or Win32
//! `ReplaceFileW` do). On Windows `std::fs::rename` replaces atomically (`MoveFileExW` with
//! `MOVEFILE_REPLACE_EXISTING`), a sharing violation (antivirus, sync clients) is retried a few
//! times with backoff, and directories aren't synced. The file layer is a trait so tests can
//! fail each step.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use thiserror::Error;

use crate::container::{ContainerError, SaveFile};

/// A readable, seekable file (meta-only reads).
pub trait ReadSeek: Read + Seek + Send {}
impl<T: Read + Seek + Send> ReadSeek for T {}

/// The file operations saving needs.
pub trait SaveFs: Send + Sync {
    /// Creates or truncates `path`, writes `bytes` and syncs the data to disk.
    fn write_synced(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    fn open(&self, path: &Path) -> io::Result<Box<dyn ReadSeek>>;
    /// Renames, replacing an existing `to`.
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    /// Copies `from` over `to` (created or truncated) and syncs `to`; `from` is untouched.
    fn copy(&self, from: &Path, to: &Path) -> io::Result<()> {
        let bytes = self.read(from)?;
        self.write_synced(to, &bytes)
    }
    /// Makes renames in `dir` durable (no-op where directories can't be synced).
    fn sync_dir(&self, dir: &Path) -> io::Result<()>;
    fn remove(&self, path: &Path) -> io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, dir: &Path) -> io::Result<()>;
    /// Files (not directories) directly in `dir`; empty when `dir` doesn't exist.
    fn list(&self, dir: &Path) -> io::Result<Vec<PathBuf>>;
    /// Subdirectories directly in `dir`; empty when `dir` doesn't exist.
    fn list_dirs(&self, dir: &Path) -> io::Result<Vec<PathBuf>>;
}

/// The real file system.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdFs;

fn entries(dir: &Path, want_dirs: bool) -> io::Result<Vec<PathBuf>> {
    let rd = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut out = Vec::new();
    for entry in rd {
        let entry = entry?;
        if entry.file_type()?.is_dir() == want_dirs {
            out.push(entry.path());
        }
    }
    out.sort();
    Ok(out)
}

impl SaveFs for StdFs {
    fn write_synced(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        let mut f = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        f.write_all(bytes)?;
        f.sync_all()
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    fn open(&self, path: &Path) -> io::Result<Box<dyn ReadSeek>> {
        Ok(Box::new(io::BufReader::new(File::open(path)?)))
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        fs::rename(from, to)
    }

    fn copy(&self, from: &Path, to: &Path) -> io::Result<()> {
        fs::copy(from, to)?;
        OpenOptions::new().write(true).open(to)?.sync_all()
    }

    #[cfg(unix)]
    fn sync_dir(&self, dir: &Path) -> io::Result<()> {
        File::open(dir)?.sync_all()
    }

    #[cfg(not(unix))]
    fn sync_dir(&self, _dir: &Path) -> io::Result<()> {
        Ok(())
    }

    fn remove(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dir)
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<PathBuf>> {
        entries(dir, false)
    }

    fn list_dirs(&self, dir: &Path) -> io::Result<Vec<PathBuf>> {
        entries(dir, true)
    }
}

/// Which step of [`write_atomic`] failed.
#[derive(Debug, Error)]
pub enum WriteFileError {
    #[error("creating {0}: {1}")]
    CreateDir(PathBuf, #[source] io::Error),
    #[error("writing {0}: {1}")]
    Write(PathBuf, #[source] io::Error),
    #[error("reading back {0}: {1}")]
    ReadBack(PathBuf, #[source] io::Error),
    #[error("{0} doesn't verify after writing: {1}")]
    Verify(PathBuf, #[source] ContainerError),
    #[error("keeping the previous {0} as .bak: {1}")]
    Backup(PathBuf, #[source] io::Error),
    #[error("replacing {0}: {1}")]
    Rename(PathBuf, #[source] io::Error),
}

/// What [`write_atomic`] did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Written {
    pub path: PathBuf,
    /// Where the previous file was copied (`keep_backup` and a previous file existed).
    pub backup: Option<PathBuf>,
    /// The directory sync succeeded: the rename survives a power loss. When false the new file
    /// is in place but callers must not delete older saves on its strength.
    pub durable: bool,
}

/// `<path>.<suffix>` (`x.sfsave.tmp`).
pub fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".");
    s.push(suffix);
    PathBuf::from(s)
}

/// How often the final rename is tried when Windows reports a sharing violation.
const RENAME_ATTEMPTS: u32 = 5;
/// The first wait between attempts; it doubles (10 + 20 + 40 + 80 ms at most).
const RENAME_BACKOFF: Duration = Duration::from_millis(10);

/// A transient Windows lock on the file (`ERROR_ACCESS_DENIED` 5, `ERROR_SHARING_VIOLATION` 32):
/// an antivirus or sync client holding it open for a moment.
fn is_sharing_violation(e: &io::Error) -> bool {
    cfg!(windows) && matches!(e.raw_os_error(), Some(5 | 32))
}

/// `fs.rename`, retried with backoff on a sharing violation.
fn rename_retrying(fs: &dyn SaveFs, from: &Path, to: &Path) -> io::Result<()> {
    let mut wait = RENAME_BACKOFF;
    for _ in 1..RENAME_ATTEMPTS {
        match fs.rename(from, to) {
            Err(e) if is_sharing_violation(&e) => {
                std::thread::sleep(wait);
                wait *= 2;
            }
            r => return r,
        }
    }
    fs.rename(from, to)
}

/// Opens `bytes` as a save file and checks every section (hash and decompression).
fn verify_container(bytes: Vec<u8>) -> Result<(), ContainerError> {
    let file = SaveFile::open(bytes)?;
    file.verify().into_iter().next().map_or(Ok(()), Err)
}

/// Replaces `path` with the save file `bytes` atomically (module docs). On any error the previous
/// file is untouched and no temp file remains; once `path` exists it is never missing.
pub fn write_atomic(fs: &dyn SaveFs, path: &Path, bytes: &[u8], keep_backup: bool) -> Result<Written, WriteFileError> {
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    fs.create_dir_all(dir).map_err(|e| WriteFileError::CreateDir(dir.to_owned(), e))?;
    let tmp = with_suffix(path, "tmp");
    let cleanup = |err: WriteFileError| {
        let _ = fs.remove(&tmp);
        err
    };
    fs.write_synced(&tmp, bytes).map_err(|e| cleanup(WriteFileError::Write(tmp.clone(), e)))?;
    let back = fs.read(&tmp).map_err(|e| cleanup(WriteFileError::ReadBack(tmp.clone(), e)))?;
    verify_container(back).map_err(|e| cleanup(WriteFileError::Verify(tmp.clone(), e)))?;
    let mut backup = None;
    if keep_backup && fs.exists(path) {
        let previous = with_suffix(path, "bak");
        fs.copy(path, &previous).map_err(|e| cleanup(WriteFileError::Backup(path.to_owned(), e)))?;
        backup = Some(previous);
    }
    rename_retrying(fs, &tmp, path).map_err(|e| cleanup(WriteFileError::Rename(path.to_owned(), e)))?;
    let durable = fs.sync_dir(dir).is_ok();
    Ok(Written { path: path.to_owned(), backup, durable })
}
