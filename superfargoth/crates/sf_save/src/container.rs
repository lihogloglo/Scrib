//! The container (save.md "File format"): a fixed 64-byte preamble, a postcard TOC, then the
//! sections in TOC order. Little-endian. Every section is hashed (blake3) and the TOC hash is in
//! the preamble, so truncation, bit rot and wrong files are errors, never panics.
//!
//! | Bytes | Preamble field |
//! |---|---|
//! | 0..4 | magic `SFSV` |
//! | 4..6 | container version (u16) |
//! | 6 | file kind (0 world, 1 character) |
//! | 7 | reserved, 0 |
//! | 8..16 | TOC offset (u64; 64 in version 1) |
//! | 16..24 | TOC length (u64) |
//! | 24..56 | blake3 of the other preamble bytes and the TOC |
//! | 56..64 | total file length (u64) |
//!
//! Section offsets in the TOC count from the end of the TOC, so the TOC can sit right after the
//! preamble and a save list reads only the first few KiB ([`read_meta`]).

use std::collections::HashSet;
use std::io::{self, Read, Seek, SeekFrom};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::codec::encode;
use crate::key::SectionKey;
use crate::meta::{META_VERSION, MetaError, SaveMeta};

pub const MAGIC: [u8; 4] = *b"SFSV";
/// The container version this build writes and reads.
pub const CONTAINER_VERSION: u16 = 1;
pub const PREAMBLE_LEN: usize = 64;
/// Sections smaller than this are stored raw (LZ4's frame cost outweighs its gain).
const LZ4_MIN: usize = 128;
/// LZ4 block's best ratio is just under 255:1: a larger declared raw length is corrupt.
const LZ4_MAX_RATIO: u64 = 255;

/// The owner and name of `sf/meta`.
pub fn meta_key() -> SectionKey {
    SectionKey::new("sf", "meta").expect("valid key")
}

/// What a file holds (save.md "Co-op").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileKind {
    /// A host world save (`.sfsave`).
    World,
    /// A guest's character file (`.sfchar`).
    Character,
}

impl FileKind {
    fn to_byte(self) -> u8 {
        match self {
            Self::World => 0,
            Self::Character => 1,
        }
    }

    fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::World),
            1 => Some(Self::Character),
            _ => None,
        }
    }
}

/// How a section's bytes are stored. New codecs (zstd) are new variants: older builds refuse
/// them by name, no container change needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Codec {
    Raw,
    /// One LZ4 block (`lz4_flex::block`).
    Lz4,
}

/// Per-section flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SectionFlags(pub u8);

impl SectionFlags {
    pub const NONE: Self = Self(0);
    /// Losing it is acceptable: a bad copy is skipped (thumbnails).
    pub const OPTIONAL: Self = Self(1);
    /// Carried forward unchanged for an owner the running profile lacks (save.md "Dormancy").
    pub const DORMANT: Self = Self(2);

    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[must_use]
    pub fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[must_use]
    pub fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

/// One section's table-of-contents entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TocEntry {
    pub key: SectionKey,
    /// The owner's schema version of the encoded value.
    pub version: u32,
    pub codec: Codec,
    /// From the end of the TOC.
    pub offset: u64,
    pub stored_len: u64,
    pub raw_len: u64,
    /// blake3 of the stored bytes.
    pub hash: [u8; 32],
    pub flags: SectionFlags,
}

/// Decode limits for untrusted files (shared saves, guest character files).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_toc_len: u64,
    pub max_sections: usize,
    pub max_raw_len: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_toc_len: 1 << 20, max_sections: 4096, max_raw_len: 256 << 20 }
    }
}

/// The decoded preamble.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Preamble {
    pub version: u16,
    pub kind: FileKind,
    pub toc_offset: u64,
    pub toc_len: u64,
    pub toc_hash: [u8; 32],
    pub total_len: u64,
}

/// Why a file or one of its sections can't be read.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ContainerError {
    #[error("file is {len} bytes, shorter than the {PREAMBLE_LEN}-byte preamble")]
    TooShort { len: u64 },
    #[error("not a save file (bad magic)")]
    BadMagic,
    #[error("container version {found} is not supported (this build reads {CONTAINER_VERSION})")]
    UnsupportedVersion { found: u16 },
    #[error("unknown file kind {0}")]
    UnknownKind(u8),
    #[error("reserved preamble byte is {0}, expected 0")]
    Reserved(u8),
    #[error("file is {actual} bytes but its preamble says {expected} (truncated or appended)")]
    Length { expected: u64, actual: u64 },
    #[error("table of contents at {offset}+{len} is malformed or out of bounds")]
    TocBounds { offset: u64, len: u64 },
    #[error("table of contents hash mismatch")]
    TocHash,
    #[error("table of contents: {0}")]
    TocDecode(String),
    #[error("{count} sections exceed the limit of {max}")]
    TooManySections { count: usize, max: usize },
    #[error("section {0} appears twice")]
    DuplicateKey(SectionKey),
    #[error("the first section must be sf/meta, stored raw")]
    MetaFirst,
    #[error("section {key} is out of place or out of bounds")]
    Layout { key: SectionKey },
    #[error("section {key} declares {raw_len} raw bytes, over the limit or impossible for its codec")]
    RawLength { key: SectionKey, raw_len: u64 },
    #[error("section {key} hash mismatch (corrupt)")]
    SectionHash { key: SectionKey },
    #[error("section {key} doesn't decompress: {message}")]
    Decompress { key: SectionKey, message: String },
    #[error("no section {0}")]
    Missing(SectionKey),
    #[error("sf/meta: {0}")]
    Meta(#[from] MetaError),
    #[error("read: {0}")]
    Io(String),
}

impl From<io::Error> for ContainerError {
    fn from(e: io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

/// The preamble's hash field: blake3 over every other preamble byte and the TOC, so a flipped
/// kind or length is caught like a flipped TOC byte.
fn toc_hash(preamble: &[u8; PREAMBLE_LEN], toc: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&preamble[..24]);
    h.update(&preamble[56..]);
    h.update(toc);
    *h.finalize().as_bytes()
}

impl Preamble {
    fn encode(&self) -> [u8; PREAMBLE_LEN] {
        let mut out = [0u8; PREAMBLE_LEN];
        out[0..4].copy_from_slice(&MAGIC);
        out[4..6].copy_from_slice(&self.version.to_le_bytes());
        out[6] = self.kind.to_byte();
        out[8..16].copy_from_slice(&self.toc_offset.to_le_bytes());
        out[16..24].copy_from_slice(&self.toc_len.to_le_bytes());
        out[24..56].copy_from_slice(&self.toc_hash);
        out[56..64].copy_from_slice(&self.total_len.to_le_bytes());
        out
    }

    /// Decodes and checks the fixed fields; `file_len` is the actual length.
    pub fn decode(bytes: &[u8], file_len: u64, limits: &Limits) -> Result<Self, ContainerError> {
        let Some(b) = bytes.get(..PREAMBLE_LEN) else {
            return Err(ContainerError::TooShort { len: file_len });
        };
        if b[0..4] != MAGIC {
            return Err(ContainerError::BadMagic);
        }
        let u64_at = |i: usize| u64::from_le_bytes(b[i..i + 8].try_into().expect("8 bytes"));
        let version = u16::from_le_bytes([b[4], b[5]]);
        if version != CONTAINER_VERSION {
            return Err(ContainerError::UnsupportedVersion { found: version });
        }
        let kind = FileKind::from_byte(b[6]).ok_or(ContainerError::UnknownKind(b[6]))?;
        if b[7] != 0 {
            return Err(ContainerError::Reserved(b[7]));
        }
        let p = Self {
            version,
            kind,
            toc_offset: u64_at(8),
            toc_len: u64_at(16),
            toc_hash: b[24..56].try_into().expect("32 bytes"),
            total_len: u64_at(56),
        };
        if p.total_len != file_len {
            return Err(ContainerError::Length { expected: p.total_len, actual: file_len });
        }
        let toc_end = p.toc_offset.checked_add(p.toc_len);
        if p.toc_offset != PREAMBLE_LEN as u64 || p.toc_len > limits.max_toc_len || toc_end.is_none_or(|end| end > file_len) {
            return Err(ContainerError::TocBounds { offset: p.toc_offset, len: p.toc_len });
        }
        Ok(p)
    }

    /// Where section offsets count from.
    pub fn area_start(&self) -> u64 {
        self.toc_offset + self.toc_len
    }
}

/// Decodes and checks a TOC: hash, limits, unique keys, `sf/meta` first and raw, sections
/// contiguous from the area start to `total_len`, plausible raw lengths.
fn decode_toc(p: &Preamble, toc: &[u8], limits: &Limits) -> Result<Vec<TocEntry>, ContainerError> {
    if toc_hash(&p.encode(), toc) != p.toc_hash {
        return Err(ContainerError::TocHash);
    }
    let entries: Vec<TocEntry> = crate::codec::decode(toc).map_err(|e| ContainerError::TocDecode(e.0))?;
    if entries.len() > limits.max_sections {
        return Err(ContainerError::TooManySections { count: entries.len(), max: limits.max_sections });
    }
    match entries.first() {
        Some(e) if e.key == meta_key() && e.codec == Codec::Raw => {}
        _ => return Err(ContainerError::MetaFirst),
    }
    let mut seen = HashSet::with_capacity(entries.len());
    let mut next = 0u64;
    let area = p.total_len - p.area_start();
    for e in &entries {
        if !seen.insert(&e.key) {
            return Err(ContainerError::DuplicateKey(e.key.clone()));
        }
        let end = e.offset.checked_add(e.stored_len);
        if e.offset != next || end.is_none_or(|end| end > area) {
            return Err(ContainerError::Layout { key: e.key.clone() });
        }
        next = e.offset + e.stored_len;
        let plausible = match e.codec {
            Codec::Raw => e.raw_len == e.stored_len,
            Codec::Lz4 => e.raw_len <= e.stored_len.saturating_mul(LZ4_MAX_RATIO),
        };
        if !plausible || e.raw_len > limits.max_raw_len {
            return Err(ContainerError::RawLength { key: e.key.clone(), raw_len: e.raw_len });
        }
    }
    if next != area {
        return Err(ContainerError::Layout { key: entries.last().expect("meta is present").key.clone() });
    }
    Ok(entries)
}

/// Builds a save file in memory. Sections are written in key order (`sf/meta` first), so equal
/// state gives equal bytes.
pub struct SaveWriter {
    kind: FileKind,
    meta: Vec<u8>,
    sections: Vec<(TocEntry, Vec<u8>)>,
}

/// Why a section can't be added.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum WriteError {
    #[error("section {0} added twice")]
    Duplicate(SectionKey),
    #[error("sf/meta is written by the writer itself")]
    Reserved,
    #[error("copied section {0} doesn't match its hash")]
    CopyHash(SectionKey),
}

impl SaveWriter {
    pub fn new(kind: FileKind, meta: &SaveMeta) -> Self {
        Self { kind, meta: meta.encode(), sections: Vec::new() }
    }

    fn check_new(&self, key: &SectionKey) -> Result<(), WriteError> {
        if *key == meta_key() {
            return Err(WriteError::Reserved);
        }
        if self.sections.iter().any(|(e, _)| e.key == *key) {
            return Err(WriteError::Duplicate(key.clone()));
        }
        Ok(())
    }

    /// Adds a section from its raw (postcard) bytes at schema `version`; compressed with LZ4 when
    /// that helps.
    pub fn add(&mut self, key: SectionKey, version: u32, flags: SectionFlags, raw: &[u8]) -> Result<(), WriteError> {
        self.check_new(&key)?;
        let lz4 = (raw.len() >= LZ4_MIN).then(|| lz4_flex::block::compress(raw)).filter(|c| c.len() < raw.len());
        let (codec, stored) = match lz4 {
            Some(c) => (Codec::Lz4, c),
            None => (Codec::Raw, raw.to_vec()),
        };
        let entry = TocEntry {
            key,
            version,
            codec,
            offset: 0,
            stored_len: stored.len() as u64,
            raw_len: raw.len() as u64,
            hash: *blake3::hash(&stored).as_bytes(),
            flags,
        };
        self.sections.push((entry, stored));
        Ok(())
    }

    /// Copies a section byte for byte from another file (dormant carry-forward): same codec,
    /// version and hash. `flags` replace the entry's.
    pub fn add_stored(&mut self, entry: &TocEntry, stored: &[u8], flags: SectionFlags) -> Result<(), WriteError> {
        self.check_new(&entry.key)?;
        if *blake3::hash(stored).as_bytes() != entry.hash || stored.len() as u64 != entry.stored_len {
            return Err(WriteError::CopyHash(entry.key.clone()));
        }
        self.sections.push((TocEntry { flags, offset: 0, ..entry.clone() }, stored.to_vec()));
        Ok(())
    }

    /// The file's bytes.
    pub fn finish(mut self) -> Vec<u8> {
        self.sections.sort_by(|a, b| a.0.key.cmp(&b.0.key));
        let meta_entry = TocEntry {
            key: meta_key(),
            version: META_VERSION,
            codec: Codec::Raw,
            offset: 0,
            stored_len: self.meta.len() as u64,
            raw_len: self.meta.len() as u64,
            hash: *blake3::hash(&self.meta).as_bytes(),
            flags: SectionFlags::NONE,
        };
        let mut toc = Vec::with_capacity(self.sections.len() + 1);
        let mut offset = 0;
        for (mut e, _) in std::iter::once((meta_entry, Vec::new())).chain(self.sections.iter().cloned()) {
            e.offset = offset;
            offset += e.stored_len;
            toc.push(e);
        }
        let toc_bytes = encode(&toc);
        let total = PREAMBLE_LEN as u64 + toc_bytes.len() as u64 + offset;
        let mut preamble = Preamble {
            version: CONTAINER_VERSION,
            kind: self.kind,
            toc_offset: PREAMBLE_LEN as u64,
            toc_len: toc_bytes.len() as u64,
            toc_hash: [0; 32],
            total_len: total,
        };
        preamble.toc_hash = toc_hash(&preamble.encode(), &toc_bytes);
        let mut out = Vec::with_capacity(usize::try_from(total).expect("save fits memory"));
        out.extend_from_slice(&preamble.encode());
        out.extend_from_slice(&toc_bytes);
        out.extend_from_slice(&self.meta);
        for (_, stored) in &self.sections {
            out.extend_from_slice(stored);
        }
        out
    }
}

/// An opened save file: preamble and TOC checked, sections verified on access.
#[derive(Clone, Debug)]
pub struct SaveFile {
    bytes: Vec<u8>,
    preamble: Preamble,
    toc: Vec<TocEntry>,
}

impl SaveFile {
    /// Opens with the default [`Limits`].
    pub fn open(bytes: Vec<u8>) -> Result<Self, ContainerError> {
        Self::open_with(bytes, &Limits::default())
    }

    /// Checks the preamble and the TOC (structure, hash, layout); section hashes are checked by
    /// [`SaveFile::stored`], [`SaveFile::raw`] and [`SaveFile::verify`].
    pub fn open_with(bytes: Vec<u8>, limits: &Limits) -> Result<Self, ContainerError> {
        let preamble = Preamble::decode(&bytes, bytes.len() as u64, limits)?;
        let toc_range = to_usize(preamble.toc_offset)..to_usize(preamble.area_start());
        let toc = decode_toc(&preamble, &bytes[toc_range], limits)?;
        Ok(Self { bytes, preamble, toc })
    }

    pub fn kind(&self) -> FileKind {
        self.preamble.kind
    }

    pub fn preamble(&self) -> &Preamble {
        &self.preamble
    }

    pub fn len(&self) -> u64 {
        self.bytes.len() as u64
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The whole file.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Every section, `sf/meta` first.
    pub fn entries(&self) -> &[TocEntry] {
        &self.toc
    }

    pub fn entry(&self, key: &SectionKey) -> Option<&TocEntry> {
        self.toc.iter().find(|e| e.key == *key)
    }

    fn stored_of(&self, e: &TocEntry) -> Result<&[u8], ContainerError> {
        let start = to_usize(self.preamble.area_start() + e.offset);
        let stored = &self.bytes[start..start + to_usize(e.stored_len)];
        if *blake3::hash(stored).as_bytes() == e.hash {
            Ok(stored)
        } else {
            Err(ContainerError::SectionHash { key: e.key.clone() })
        }
    }

    /// A section's stored bytes, hash-checked (for byte-for-byte copies).
    pub fn stored(&self, key: &SectionKey) -> Result<&[u8], ContainerError> {
        let e = self.entry(key).ok_or_else(|| ContainerError::Missing(key.clone()))?;
        self.stored_of(e)
    }

    /// A section's raw (decompressed) bytes, hash-checked.
    pub fn raw(&self, key: &SectionKey) -> Result<Vec<u8>, ContainerError> {
        let e = self.entry(key).ok_or_else(|| ContainerError::Missing(key.clone()))?;
        raw_of(e, self.stored_of(e)?)
    }

    pub fn meta(&self) -> Result<SaveMeta, ContainerError> {
        let e = &self.toc[0];
        Ok(SaveMeta::decode(e.version, self.stored_of(e)?)?)
    }

    /// Checks every section (hash and decompression); returns the bad ones.
    pub fn verify(&self) -> Vec<ContainerError> {
        self.toc.iter().filter_map(|e| self.stored_of(e).and_then(|s| raw_of(e, s)).err()).collect()
    }
}

fn to_usize(v: u64) -> usize {
    usize::try_from(v).expect("checked against the file length")
}

fn raw_of(e: &TocEntry, stored: &[u8]) -> Result<Vec<u8>, ContainerError> {
    match e.codec {
        Codec::Raw => Ok(stored.to_vec()),
        Codec::Lz4 => {
            let mut out = vec![0; to_usize(e.raw_len)];
            let fail = |message: String| ContainerError::Decompress { key: e.key.clone(), message };
            let n = lz4_flex::block::decompress_into(stored, &mut out).map_err(|err| fail(err.to_string()))?;
            if n == out.len() { Ok(out) } else { Err(fail(format!("{n} bytes, expected {}", out.len()))) }
        }
    }
}

/// Reads only the preamble, the TOC and `sf/meta` (a few KiB): what a save list needs. Checks
/// the file length, the TOC and the meta hash, not the other sections.
pub fn read_meta<R: Read + Seek>(reader: &mut R, limits: &Limits) -> Result<(FileKind, SaveMeta), ContainerError> {
    let file_len = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(0))?;
    if file_len < PREAMBLE_LEN as u64 {
        return Err(ContainerError::TooShort { len: file_len });
    }
    let mut pre = [0u8; PREAMBLE_LEN];
    reader.read_exact(&mut pre)?;
    let preamble = Preamble::decode(&pre, file_len, limits)?;
    let mut toc_bytes = vec![0; to_usize(preamble.toc_len)];
    reader.read_exact(&mut toc_bytes)?;
    let toc = decode_toc(&preamble, &toc_bytes, limits)?;
    let e = &toc[0];
    let mut stored = vec![0; to_usize(e.stored_len)];
    reader.read_exact(&mut stored)?;
    if *blake3::hash(&stored).as_bytes() != e.hash {
        return Err(ContainerError::SectionHash { key: e.key.clone() });
    }
    Ok((preamble.kind, SaveMeta::decode(e.version, &stored)?))
}
