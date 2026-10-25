//! The on-disk cache: content-addressed objects, traces, profile manifests.
//!
//! ```text
//! <root>/objects/ab/cdef…   one product each, named by the blake3 of its contents
//! <root>/traces/<step>/<h>  how a product was made (keys + dynamic inputs)
//! <root>/profiles/<p>.manifest   (step, subject) -> object, per mod set
//! ```
//!
//! Every write goes to a temporary file first and is renamed into place, so
//! an interrupted bake never leaves a half-written object or trace behind.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::{Digest, Input, ProductKey};

const TAG_RAW: u8 = 0;
const TAG_LZ4: u8 = 1;

/// Handle to a cache directory. Cheap to share; all methods take `&self`.
#[derive(Clone, Debug)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    /// Opens (creating if needed) a cache rooted at `root`.
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        for dir in ["objects", "traces", "profiles"] {
            fs::create_dir_all(root.join(dir))?;
        }
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn object_path(&self, d: &Digest) -> PathBuf {
        let hex = d.hex();
        self.root.join("objects").join(&hex[..2]).join(&hex[2..])
    }

    /// Stores `bytes` and returns their digest (of the uncompressed bytes).
    /// Already-present objects are not rewritten.
    pub fn put(&self, bytes: &[u8], compress: bool) -> io::Result<Digest> {
        let digest = Digest::of(bytes);
        let path = self.object_path(&digest);
        if path.exists() {
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
        write_atomic(&path, &encoded)?;
        Ok(digest)
    }

    pub fn contains(&self, d: &Digest) -> bool {
        self.object_path(d).is_file()
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

    fn trace_path(&self, key: &ProductKey) -> PathBuf {
        let name = Digest::of(key.subject.as_bytes()).hex();
        self.root.join("traces").join(&key.step).join(&name[..32])
    }

    pub(crate) fn load_trace(&self, key: &ProductKey) -> Option<Trace> {
        let bytes = fs::read(self.trace_path(key)).ok()?;
        let trace: Trace = postcard::from_bytes(&bytes).ok()?;
        // Guards against the (astronomically unlikely) subject-hash collision
        // and against traces written by an incompatible format.
        (trace.subject == key.subject && trace.format == TRACE_FORMAT).then_some(trace)
    }

    pub(crate) fn save_trace(&self, key: &ProductKey, trace: &Trace) -> io::Result<()> {
        let bytes = postcard::to_stdvec(trace).map_err(io::Error::other)?;
        write_atomic(&self.trace_path(key), &bytes)
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
                let referenced = parse_digest(&prefix, &name).is_some_and(|d| reachable.contains(&d));
                if !referenced {
                    unreachable.push((meta.modified().unwrap_or(SystemTime::UNIX_EPOCH), meta.len(), file.path()));
                }
            }
        }
        unreachable.sort();
        for (_, len, path) in unreachable {
            if report.total_bytes <= cap_bytes {
                break;
            }
            fs::remove_file(path)?;
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
        // identical by construction for objects, and newer for traces.
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
