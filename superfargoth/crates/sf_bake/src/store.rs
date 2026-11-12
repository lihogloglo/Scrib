//! The on-disk cache: content-addressed objects, traces, profile manifests.
//!
//! ```text
//! <root>/objects/ab/cdef…        one product each, named by the blake3 of its contents
//! <root>/traces.log              how products were made (keys + dynamic inputs), one log per store
//! <root>/profiles/<p>.manifest   (step, subject) -> object, per mod set
//! ```
//!
//! Objects and manifests are written to a temporary file first and renamed
//! into place, so an interrupted bake never leaves a half-written one behind.
//!
//! Traces are an action cache in the style of Ninja's `.ninja_log` and Bazel's
//! action cache: one append-only log of length-prefixed, checksummed
//! `(key, trace)` records, the last record per key winning. It is read into
//! memory on first use, so a warm bake validates every product without opening
//! a file per product (one cold open costs milliseconds under an on-access
//! virus scanner). A torn final record (a crash mid-append) fails its checksum
//! and is cut off on load; the log is rewritten (temporary file + rename) when
//! dead records outnumber live ones. Object presence is answered from an
//! in-memory index listed once per shard directory, not one stat per object.
//!
//! Stores from before the log kept one file per trace (`traces/<step>/<h>`).
//! Those are still read when the log has no record for a key, and copied into
//! the log when read, so the first bake after the switch keeps its cache and
//! the next one reads only the log.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, PoisonError, RwLock};
use std::time::{Duration, SystemTime};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Digest, Input, ProductKey};

const TAG_RAW: u8 = 0;
const TAG_LZ4: u8 = 1;

/// The trace log's file name under the store root.
pub(crate) const TRACE_LOG: &str = "traces.log";
/// First bytes of a trace log; a file without them is discarded.
const LOG_MAGIC: &[u8; 8] = b"SFTRLOG1";
/// Record header: payload length (u32 LE) + the first bytes of its blake3.
const RECORD_HEADER: usize = 4 + RECORD_CHECK;
const RECORD_CHECK: usize = 8;

/// Handle to a cache directory. Cheap to share (clones share the in-memory
/// trace log and object index); all methods take `&self`.
#[derive(Clone)]
pub struct Store {
    root: PathBuf,
    shared: Arc<Shared>,
}

/// State loaded on first use, so opening a store just to read a manifest
/// (the runtime, `mods`) stays cheap.
#[derive(Default)]
struct Shared {
    /// `Err` holds why the log couldn't be loaded: traces then all miss.
    traces: OnceLock<Result<RwLock<TraceLog>, String>>,
    objects: OnceLock<RwLock<HashSet<Digest>>>,
}

impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Store").field("root", &self.root).finish_non_exhaustive()
    }
}

impl Store {
    /// Opens (creating if needed) a cache rooted at `root`.
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        for dir in ["objects", "profiles"] {
            fs::create_dir_all(root.join(dir))?;
        }
        Ok(Self { root, shared: Arc::default() })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn object_path(&self, d: &Digest) -> PathBuf {
        let hex = d.hex();
        self.root.join("objects").join(&hex[..2]).join(&hex[2..])
    }

    fn objects(&self) -> &RwLock<HashSet<Digest>> {
        self.shared.objects.get_or_init(|| RwLock::new(list_objects(&self.root.join("objects"))))
    }

    /// Stores `bytes` and returns their digest (of the uncompressed bytes).
    /// Already-present objects are not rewritten.
    pub fn put(&self, bytes: &[u8], compress: bool) -> io::Result<Digest> {
        let digest = Digest::of(bytes);
        if self.contains(&digest) {
            return Ok(digest);
        }
        let mut encoded = Vec::with_capacity(bytes.len() / 2 + 1);
        if compress {
            encoded.push(TAG_LZ4);
            encoded.extend_from_slice(&lz4_flex::compress_prepend_size(bytes));
        } else {
            encoded.push(TAG_RAW);
            encoded.extend_from_slice(bytes);
        }
        write_atomic(&self.object_path(&digest), &encoded)?;
        self.objects().write().unwrap_or_else(PoisonError::into_inner).insert(digest);
        Ok(digest)
    }

    /// Whether the object is stored. Answered from the in-memory index; an
    /// object missing from it is looked up on disk (another process may have
    /// stored it since the index was listed), so this never says no wrongly.
    pub fn contains(&self, d: &Digest) -> bool {
        if self.objects().read().unwrap_or_else(PoisonError::into_inner).contains(d) {
            return true;
        }
        let present = self.object_path(d).is_file();
        if present {
            self.objects().write().unwrap_or_else(PoisonError::into_inner).insert(*d);
        }
        present
    }

    /// Reads an object back (decompressed). Doesn't re-hash; see [`Store::verify`].
    pub fn get(&self, d: &Digest) -> io::Result<Vec<u8>> {
        let encoded = fs::read(self.object_path(d))?;
        match encoded.split_first() {
            Some((&TAG_RAW, rest)) => Ok(rest.to_vec()),
            Some((&TAG_LZ4, rest)) => {
                lz4_flex::decompress_size_prepended(rest).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("object {d} has an unknown encoding"))),
        }
    }

    /// Reads an object and checks it still hashes to its name.
    pub fn verify(&self, d: &Digest) -> io::Result<bool> {
        Ok(Digest::of(&self.get(d)?) == *d)
    }

    fn trace_log(&self) -> io::Result<&RwLock<TraceLog>> {
        self.shared
            .traces
            .get_or_init(|| TraceLog::load(&self.root).map(RwLock::new).map_err(|e| e.to_string()))
            .as_ref()
            .map_err(|e| io::Error::other(format!("trace log {}: {e}", self.root.join(TRACE_LOG).display())))
    }

    /// Where a trace lived before the trace log (one file per trace).
    fn legacy_trace_path(&self, key: &ProductKey) -> PathBuf {
        let name = Digest::of(key.subject.as_bytes()).hex();
        self.root.join("traces").join(&key.step).join(&name[..32])
    }

    pub(crate) fn load_trace(&self, key: &ProductKey) -> Option<Trace> {
        let log = self.trace_log().ok()?;
        {
            let log = log.read().unwrap_or_else(PoisonError::into_inner);
            if let Some(bytes) = log.trace_bytes(key) {
                return decode_trace(bytes, key);
            }
            if !log.legacy_steps.contains(&key.step) {
                return None;
            }
        }
        let trace = decode_trace(&fs::read(self.legacy_trace_path(key)).ok()?, key)?;
        // Copied into the log so the next bake finds it there. Skipped if a
        // newer trace was saved meanwhile; a failed append only costs a re-read.
        let mut log = log.write().unwrap_or_else(PoisonError::into_inner);
        if log.trace_bytes(key).is_none() {
            let _ = log.append(key, &trace);
        }
        Some(trace)
    }

    pub(crate) fn save_trace(&self, key: &ProductKey, trace: &Trace) -> io::Result<()> {
        self.trace_log()?.write().unwrap_or_else(PoisonError::into_inner).append(key, trace)
    }

    /// (live keys, records) in the trace log, dead records included.
    #[cfg(test)]
    pub(crate) fn trace_log_counts(&self) -> (usize, usize) {
        let log = self.trace_log().unwrap().read().unwrap();
        (log.index.len(), log.records)
    }

    /// Rewrites every logged trace as a pre-log per-file trace and deletes
    /// the log, giving a store as the old layout left it. Takes the last
    /// handle on the store, so the log is closed before it is deleted.
    #[cfg(test)]
    pub(crate) fn downgrade_to_legacy_traces(self) {
        let files: Vec<(PathBuf, Vec<u8>)> = {
            let log = self.trace_log().unwrap().read().unwrap();
            log.index.keys().map(|k| (self.legacy_trace_path(k), log.trace_bytes(k).unwrap().to_vec())).collect()
        };
        let log_path = self.root.join(TRACE_LOG);
        assert_eq!(Arc::strong_count(&self.shared), 1, "other handles keep the log open");
        drop(self);
        for (path, bytes) in files {
            write_atomic(&path, &bytes).unwrap();
        }
        fs::remove_file(log_path).unwrap();
    }

    fn manifest_path(&self, profile: &str) -> PathBuf {
        self.root.join("profiles").join(format!("{profile}.manifest"))
    }

    /// Loads a profile's manifest; empty if the profile was never baked.
    pub fn load_manifest(&self, profile: &str) -> io::Result<Manifest> {
        match fs::read(self.manifest_path(profile)) {
            Ok(bytes) => postcard::from_bytes(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Manifest::default()),
            Err(e) => Err(e),
        }
    }

    pub fn save_manifest(&self, profile: &str, manifest: &Manifest) -> io::Result<()> {
        let bytes = postcard::to_stdvec(manifest).map_err(io::Error::other)?;
        write_atomic(&self.manifest_path(profile), &bytes)
    }

    /// The profile fingerprint stored by the last bake of `profile`
    /// (mods.md "Bake cache interaction"); `None` if never stored or unreadable.
    /// Kept beside the manifest (`<profile>.fingerprint`) so manifests from
    /// before profile fingerprints still load.
    pub fn profile_fingerprint(&self, profile: &str) -> Option<Digest> {
        let bytes = fs::read(self.manifest_path(profile).with_extension("fingerprint")).ok()?;
        Some(Digest(bytes.try_into().ok()?))
    }

    /// Stores the profile fingerprint a bake ran with (after its manifest).
    pub fn save_profile_fingerprint(&self, profile: &str, fingerprint: &Digest) -> io::Result<()> {
        write_atomic(&self.manifest_path(profile).with_extension("fingerprint"), &fingerprint.0)
    }

    /// Deletes objects no profile manifest references, oldest first, until
    /// the store is at most `cap_bytes`. Referenced objects are never
    /// deleted. Temporary files from interrupted writes older than an hour
    /// are removed too.
    pub fn gc(&self, cap_bytes: u64) -> io::Result<GcReport> {
        let mut reachable = HashSet::new();
        for entry in fs::read_dir(self.root.join("profiles"))? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "manifest") {
                let bytes = fs::read(&path)?;
                let manifest: Manifest =
                    postcard::from_bytes(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                reachable.extend(manifest.products.values().copied());
            }
        }

        let mut report = GcReport::default();
        let mut unreachable = Vec::new();
        let now = SystemTime::now();
        for shard in fs::read_dir(self.root.join("objects"))? {
            let shard = shard?;
            if !shard.file_type()?.is_dir() {
                continue;
            }
            let prefix = shard.file_name().to_string_lossy().into_owned();
            for file in fs::read_dir(shard.path())? {
                let file = file?;
                let meta = file.metadata()?;
                let name = file.file_name().to_string_lossy().into_owned();
                if name.contains(".tmp") {
                    let age = meta.modified().ok().and_then(|m| now.duration_since(m).ok()).unwrap_or_default();
                    if age > Duration::from_secs(3600) {
                        fs::remove_file(file.path())?;
                    }
                    continue;
                }
                report.total_bytes += meta.len();
                let digest = parse_digest(&prefix, &name);
                if !digest.is_some_and(|d| reachable.contains(&d)) {
                    unreachable.push((meta.modified().unwrap_or(SystemTime::UNIX_EPOCH), meta.len(), file.path(), digest));
                }
            }
        }
        unreachable.sort();
        for (_, len, path, digest) in unreachable {
            if report.total_bytes <= cap_bytes {
                break;
            }
            fs::remove_file(path)?;
            if let (Some(index), Some(d)) = (self.shared.objects.get(), digest) {
                index.write().unwrap_or_else(PoisonError::into_inner).remove(&d);
            }
            report.total_bytes -= len;
            report.deleted_objects += 1;
            report.deleted_bytes += len;
        }
        Ok(report)
    }
}

/// What [`Store::gc`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GcReport {
    pub deleted_objects: u64,
    pub deleted_bytes: u64,
    /// Object bytes left in the store.
    pub total_bytes: u64,
}

/// A profile's products: (step, subject) -> object. This is what the
/// runtime loads at startup to find baked data.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub products: BTreeMap<ProductKey, Digest>,
}

impl Manifest {
    pub fn get(&self, step: &str, subject: &str) -> Option<Digest> {
        // BTreeMap<ProductKey, _> can't be queried by borrowed parts; the
        // runtime builds its own index if this shows up in profiles.
        self.products.get(&ProductKey::new(step, subject)).copied()
    }

    /// Adds `newer`'s entries over this manifest's (for partial bakes).
    pub fn merge(&mut self, newer: &Manifest) {
        self.products.extend(newer.products.iter().map(|(k, v)| (k.clone(), *v)));
    }
}

/// Bumped when the trace layout changes; old traces then count as misses.
const TRACE_FORMAT: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Trace {
    pub format: u32,
    pub subject: String,
    pub static_key: Digest,
    /// Inputs read while running, beyond the planned ones, with the
    /// fingerprints they had when read.
    pub dynamic: Vec<(Input, Digest)>,
    pub output: Digest,
}

impl Trace {
    pub fn new(subject: &str, static_key: Digest, dynamic: Vec<(Input, Digest)>, output: Digest) -> Self {
        Self { format: TRACE_FORMAT, subject: subject.to_owned(), static_key, dynamic, output }
    }
}

fn decode_trace(bytes: &[u8], key: &ProductKey) -> Option<Trace> {
    let trace: Trace = postcard::from_bytes(bytes).ok()?;
    // Guards against the (astronomically unlikely) subject-hash collision of
    // legacy trace files and against traces written by an incompatible format.
    (trace.subject == key.subject && trace.format == TRACE_FORMAT).then_some(trace)
}

/// The trace log in memory: the file's valid bytes, and where each key's
/// latest trace sits in them (traces decode on lookup).
struct TraceLog {
    /// Append handle; every record goes out in one write.
    file: File,
    bytes: Vec<u8>,
    index: HashMap<ProductKey, Range<usize>>,
    /// Records in `bytes`, superseded ones included.
    records: usize,
    /// Steps with a pre-log `traces/<step>/` directory to fall back on.
    legacy_steps: HashSet<String>,
}

impl TraceLog {
    fn load(root: &Path) -> io::Result<Self> {
        let path = root.join(TRACE_LOG);
        let mut bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e),
        };
        if !bytes.starts_with(LOG_MAGIC) {
            bytes = LOG_MAGIC.to_vec();
            write_atomic(&path, &bytes)?;
        }
        let (mut index, mut records, valid) = parse_log(&bytes);
        if records - index.len() > index.len() {
            // Mostly superseded records: keep the live ones only. Done before
            // the append handle opens, so the rename never replaces an open file.
            bytes = compacted(&bytes, &index)?;
            write_atomic(&path, &bytes)?;
            (index, records, _) = parse_log(&bytes);
        } else if valid < bytes.len() {
            // A torn or corrupt tail: cut it off so appends follow the last good record.
            bytes.truncate(valid);
            OpenOptions::new().write(true).open(&path)?.set_len(valid as u64)?;
        }
        let legacy_steps = match fs::read_dir(root.join("traces")) {
            Ok(dirs) => dirs
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect(),
            Err(_) => HashSet::new(),
        };
        let file = OpenOptions::new().append(true).open(&path)?;
        Ok(Self { file, bytes, index, records, legacy_steps })
    }

    fn trace_bytes(&self, key: &ProductKey) -> Option<&[u8]> {
        self.index.get(key).map(|r| &self.bytes[r.clone()])
    }

    fn append(&mut self, key: &ProductKey, trace: &Trace) -> io::Result<()> {
        let trace_bytes = postcard::to_stdvec(trace).map_err(io::Error::other)?;
        let record = encode_record(key, &trace_bytes)?;
        self.file.write_all(&record)?;
        let end = self.bytes.len() + record.len();
        self.bytes.extend_from_slice(&record);
        self.index.insert(key.clone(), end - trace_bytes.len()..end);
        self.records += 1;
        Ok(())
    }
}

/// A log holding only `index`'s records, in their log order.
fn compacted(bytes: &[u8], index: &HashMap<ProductKey, Range<usize>>) -> io::Result<Vec<u8>> {
    let mut live: Vec<(&ProductKey, &Range<usize>)> = index.iter().collect();
    live.sort_by_key(|(_, r)| r.start);
    let mut out = LOG_MAGIC.to_vec();
    for (key, range) in live {
        out.extend_from_slice(&encode_record(key, &bytes[range.clone()])?);
    }
    Ok(out)
}

/// `len (u32 LE) | blake3(payload)[..8] | payload`, payload = postcard(key) ++ postcard(trace).
fn encode_record(key: &ProductKey, trace_bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut payload = postcard::to_stdvec(key).map_err(io::Error::other)?;
    payload.extend_from_slice(trace_bytes);
    let len = u32::try_from(payload.len()).map_err(io::Error::other)?;
    let mut record = Vec::with_capacity(RECORD_HEADER + payload.len());
    record.extend_from_slice(&len.to_le_bytes());
    record.extend_from_slice(&blake3::hash(&payload).as_bytes()[..RECORD_CHECK]);
    record.extend_from_slice(&payload);
    Ok(record)
}

/// Indexes a log's records (last per key wins), stopping at the first record
/// that is cut short or fails its checksum. Returns (index, records, valid length).
fn parse_log(bytes: &[u8]) -> (HashMap<ProductKey, Range<usize>>, usize, usize) {
    let mut index = HashMap::new();
    let mut records = 0;
    let mut pos = LOG_MAGIC.len();
    while let Some((key, trace, next)) = parse_record(bytes, pos) {
        index.insert(key, trace);
        records += 1;
        pos = next;
    }
    (index, records, pos)
}

/// The record at `pos`: its key, its trace's byte range, and where the next record starts.
fn parse_record(bytes: &[u8], pos: usize) -> Option<(ProductKey, Range<usize>, usize)> {
    let header = bytes.get(pos..pos.checked_add(RECORD_HEADER)?)?;
    let len = usize::try_from(u32::from_le_bytes(header[..4].try_into().ok()?)).ok()?;
    let start = pos + RECORD_HEADER;
    let end = start.checked_add(len)?;
    let payload = bytes.get(start..end)?;
    if blake3::hash(payload).as_bytes()[..RECORD_CHECK] != header[4..] {
        return None;
    }
    let (key, rest) = postcard::take_from_bytes::<ProductKey>(payload).ok()?;
    Some((key, end - rest.len()..end, end))
}

/// Every object digest in the store: one directory listing per shard, in parallel.
fn list_objects(objects: &Path) -> HashSet<Digest> {
    let Ok(shards) = fs::read_dir(objects) else { return HashSet::new() };
    let shards: Vec<(String, PathBuf)> = shards
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .collect();
    shards
        .par_iter()
        .flat_map_iter(|(prefix, dir)| {
            fs::read_dir(dir)
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .filter_map(|f| parse_digest(prefix, &f.file_name().to_string_lossy()))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn parse_digest(prefix: &str, rest: &str) -> Option<Digest> {
    let hex = format!("{prefix}{rest}");
    if hex.len() != 64 {
        return None;
    }
    let mut out = [0; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(Digest(out))
}

/// Writes through a uniquely named temporary file and renames it into place.
fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let dir = path.parent().expect("store paths have a parent");
    fs::create_dir_all(dir)?;
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let file_name = path.file_name().expect("store paths have a file name").to_string_lossy();
    let tmp = dir.join(format!("{file_name}.{}-{n}.tmp", std::process::id()));
    fs::write(&tmp, bytes)?;
    match fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        // Another thread or process stored the same object first; theirs is
        // identical by construction for objects, and newer for the rest.
        Err(_) if path.exists() => {
            let _ = fs::remove_file(&tmp);
            Ok(())
        }
        Err(e) => {
            let _ = fs::remove_file(&tmp);
            Err(e)
        }
    }
}
