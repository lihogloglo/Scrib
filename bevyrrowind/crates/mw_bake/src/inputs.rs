//! Bake inputs over the mounted VFS and the record database.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::UNIX_EPOCH;

use mw_install::MorrowindInstall;
use sf_bake::{BakeError, Digest, FileHashMemo, Inputs, StatKey};
use sf_records::{RecordDb, RecordError, RecordId};
use sf_vfs::{Vfs, VfsPath};

use crate::sources::{AnimSources, VIRTUAL_ROOT};

/// [`Inputs`] for Morrowind content.
///
/// Files are fingerprinted by their winning VFS source, through a
/// stat-keyed [`FileHashMemo`]: loose files by their own stat, archive
/// members by the archive's stat plus the member's path (unique inside the
/// archive; `tes3` doesn't expose member offsets, and any change to the
/// archive changes its stat anyway). A warm start therefore reads one stat
/// per archive and one per loose file, and hashes nothing.
///
/// Records are fingerprinted by their resolved value (see
/// [`VfsInputs::record_fingerprint`]).
pub struct VfsInputs {
    vfs: Arc<Vfs>,
    records: Option<Arc<RecordDb>>,
    /// Records that aren't TES3 records (bake-side kinds such as
    /// `terrain.source_tile`), with their fingerprints: (kind, id) → digest.
    extra: HashMap<(String, String), Digest>,
    /// Source label -> archive file on disk.
    archives: HashMap<String, PathBuf>,
    /// Archive stats, read once per session.
    archive_stats: Mutex<HashMap<String, (u64, u128)>>,
    memo: FileHashMemo,
    /// The user's animation libraries: `animsrc/<source>/<file>` paths.
    anim_sources: Option<Arc<AnimSources>>,
}

impl VfsInputs {
    /// `memo_file` keeps file hashes between sessions (inside the cache root).
    pub fn new(vfs: Arc<Vfs>, install: &MorrowindInstall, memo_file: impl Into<PathBuf>) -> Self {
        let archives =
            install.archives().into_iter().filter_map(|name| install.data_file(&name).map(|path| (name, path))).collect();
        Self { vfs, records: None, extra: HashMap::new(), archives, archive_stats: Mutex::default(), memo: FileHashMemo::load(memo_file), anim_sources: None }
    }

    /// Makes records available as bake inputs.
    #[must_use]
    pub fn with_records(mut self, records: Arc<RecordDb>) -> Self {
        self.records = Some(records);
        self
    }

    /// Adds fingerprints of records the host computes itself (kinds the
    /// TES3 database doesn't hold: terrain source tiles and layers).
    #[must_use]
    pub fn with_extra_records(mut self, records: impl IntoIterator<Item = ((String, String), Digest)>) -> Self {
        self.extra.extend(records);
        self
    }

    /// Serves the animation libraries' virtual paths (`animsrc/...`,
    /// [`AnimSources::virtual_path`]) from the user's folder: fingerprinted
    /// by stat like loose files; a missing file is a missing input.
    #[must_use]
    pub fn with_anim_sources(mut self, sources: Arc<AnimSources>) -> Self {
        self.anim_sources = Some(sources);
        self
    }

    /// The library file behind a virtual library path (`None`: missing).
    fn library_file(&self, path: &str) -> Option<PathBuf> {
        self.anim_sources.as_ref().and_then(|s| s.real_path(path))
    }

    pub fn vfs(&self) -> &Arc<Vfs> {
        &self.vfs
    }

    /// Writes the file-hash memo back (call after a bake).
    pub fn save(&self) -> std::io::Result<()> {
        self.memo.save()
    }

    fn archive_stat(&self, label: &str, path: &Path) -> std::io::Result<(u64, u128)> {
        let mut stats = self.archive_stats.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(stat) = stats.get(label) {
            return Ok(*stat);
        }
        let meta = fs::metadata(path)?;
        let mtime = meta.modified().ok().and_then(|m| m.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos());
        stats.insert(label.to_owned(), (meta.len(), mtime));
        Ok((meta.len(), mtime))
    }
}

fn vfs_path(path: &str) -> Result<VfsPath, BakeError> {
    VfsPath::new(path).map_err(|e| BakeError::Other(format!("{path:?}: {e}")))
}

impl Inputs for VfsInputs {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        if path.starts_with(VIRTUAL_ROOT) {
            let Some(real) = self.library_file(path) else { return Ok(None) };
            let key = FileHashMemo::stat_file("library", &real.to_string_lossy(), &real)?;
            return Ok(Some(self.memo.fingerprint(key, || fs::read(&real))?));
        }
        let Ok(vpath) = VfsPath::new(path) else { return Ok(None) };
        let Some(index) = self.vfs.resolve(&vpath) else { return Ok(None) };
        let source = self.vfs.source(index);
        let read = || source.read(&vpath);
        let key = if let Some(real) = source.real_path(&vpath) {
            Some(FileHashMemo::stat_file("loose", &real.to_string_lossy(), &real)?)
        } else if let Some(archive) = self.archives.get(source.label()) {
            let (size, mtime_ns) = self.archive_stat(source.label(), archive)?;
            let source = format!("archive:{}", archive.display());
            Some(StatKey { source, path: vpath.as_str().to_owned(), size, mtime_ns })
        } else {
            None
        };
        Ok(Some(match key {
            Some(key) => self.memo.fingerprint(key, read)?,
            // A source without stats (in-memory): hash every time.
            None => Digest::of(&read()?),
        }))
    }

    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        if path.starts_with(VIRTUAL_ROOT) {
            let real = self.library_file(path).ok_or_else(|| BakeError::Other(format!("{path}: library file missing")))?;
            return Ok(fs::read(&real)?);
        }
        Ok(self.vfs.read(&vfs_path(path)?)?)
    }

    /// The fingerprint of a record's resolved value: the kind's registered
    /// fingerprint function (`RecordDb::fingerprint`; TES3 kinds are serde
    /// kinds, hashed as blake3 of their postcard bytes), so a mod's patch to
    /// a record changes it. Kind names match as given, else upper-cased
    /// (TES3 tags).
    fn record_fingerprint(&self, kind: &str, id: &str) -> Result<Option<Digest>, BakeError> {
        if let Some(d) = self.extra.get(&(kind.to_owned(), id.to_owned())) {
            return Ok(Some(*d));
        }
        let db = self.records.as_deref().ok_or_else(|| BakeError::Other("records are not bake inputs here".into()))?;
        let id = RecordId::new(id);
        let result = match db.fingerprint(kind, &id) {
            Err(RecordError::UnknownKind(_)) => db.fingerprint(&kind.to_ascii_uppercase(), &id),
            other => other,
        };
        result.map(|fp| fp.map(Digest)).map_err(|e| BakeError::Other(format!("record {kind} {id}: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use sf_vfs::{DirSource, MemSource};

    use super::*;

    fn install(dir: &Path) -> MorrowindInstall {
        let data = dir.join("Data Files");
        fs::create_dir_all(data.join("meshes")).unwrap();
        fs::write(data.join("Morrowind.esm"), b"").unwrap();
        MorrowindInstall::open(dir).unwrap()
    }

    #[test]
    fn loose_files_hash_once_per_stat() {
        let dir = tempfile::tempdir().unwrap();
        let install = install(dir.path());
        let data = &install.data_files;
        fs::write(data.join("meshes").join("a.nif"), b"one").unwrap();
        let vfs = Vfs::builder()
            .mount(MemSource::new("mem").with("meshes/m.nif", "memory"))
            .mount(DirSource::scan("Data Files", data).unwrap())
            .build();
        let memo = dir.path().join("memo");
        let inputs = VfsInputs::new(Arc::new(vfs), &install, &memo);

        let a = inputs.file_fingerprint("meshes/a.nif").unwrap().unwrap();
        assert_eq!(a, Digest::of(b"one"));
        assert_eq!(inputs.file_fingerprint("meshes/m.nif").unwrap(), Some(Digest::of(b"memory")));
        assert_eq!(inputs.file_fingerprint("meshes/missing.nif").unwrap(), None);
        assert_eq!(inputs.read_file("meshes/a.nif").unwrap(), b"one");
        inputs.save().unwrap();

        // A new session trusts the memo while the stat is unchanged...
        let vfs = Vfs::builder().mount(DirSource::scan("Data Files", data).unwrap()).build();
        let warm = VfsInputs::new(Arc::new(vfs), &install, &memo);
        assert_eq!(warm.file_fingerprint("meshes/a.nif").unwrap(), Some(a));
        // ...and re-hashes when it changes (size differs here).
        fs::write(data.join("meshes").join("a.nif"), b"three").unwrap();
        assert_eq!(warm.file_fingerprint("meshes/a.nif").unwrap(), Some(Digest::of(b"three")));
    }

    #[test]
    fn records_need_a_database() {
        let dir = tempfile::tempdir().unwrap();
        let install = install(dir.path());
        let inputs = VfsInputs::new(Arc::new(Vfs::builder().build()), &install, dir.path().join("memo"));
        assert!(inputs.record_fingerprint("STAT", "x").is_err());

        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let source = db.add_source("test", &[]);
        let record = tes3::esp::Static { id: "rock".into(), mesh: "x\\rock.nif".into(), ..Default::default() };
        db.define(source, RecordId::new("rock"), record).unwrap();
        let inputs = inputs.with_records(Arc::new(db));
        let fp = inputs.record_fingerprint("STAT", "rock").unwrap();
        assert!(fp.is_some());
        assert_eq!(inputs.record_fingerprint("stat", "rock").unwrap(), fp);
        assert_eq!(inputs.record_fingerprint("STAT", "other").unwrap(), None);
        assert_eq!(inputs.record_fingerprint("CELL", "x").unwrap(), None);
        assert!(inputs.record_fingerprint("NOPE", "x").is_err());
    }
}
