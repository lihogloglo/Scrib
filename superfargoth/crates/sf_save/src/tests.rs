//! Container, atomic-write, slot and CLI tests (save.md "Tests"), on synthetic data.

use std::collections::BTreeMap;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use super::*;

fn key(text: &str) -> SectionKey {
    SectionKey::parse(text).unwrap()
}

fn meta(seq: u64, reason: SaveReason) -> SaveMeta {
    SaveMeta {
        world_id: 0x1234,
        save_id: 0x5600 + u128::from(seq),
        sequence: seq,
        reason,
        character_name: "Tester".into(),
        load_order: vec![ModRef { id: "base".into(), version: "1.0.0".into(), manifest_hash: [7; 32] }],
        party: vec![PartyMember { player_key: 9, name: "Host".into() }],
        grid_cell: 256.0,
        ..SaveMeta::default()
    }
}

/// A file with a raw section, a compressible one and an optional one.
fn sample(seq: u64) -> Vec<u8> {
    let mut w = SaveWriter::new(FileKind::World, &meta(seq, SaveReason::Manual));
    w.add(key("sf/rpg"), 1, SectionFlags::NONE, &encode(&(1u32, "quest".to_owned()))).unwrap();
    let big: Vec<u32> = (0..2000).map(|i| i % 17).collect();
    w.add(key("sf/overlay"), 2, SectionFlags::NONE, &encode(&big)).unwrap();
    w.add(key("sf/thumbnail"), 1, SectionFlags::OPTIONAL, &[1, 2, 3]).unwrap();
    w.finish()
}

#[test]
fn round_trip_orders_sections_and_compresses() {
    let bytes = sample(1);
    let f = SaveFile::open(bytes.clone()).unwrap();
    assert_eq!(f.kind(), FileKind::World);
    assert_eq!(f.meta().unwrap(), meta(1, SaveReason::Manual));
    let keys: Vec<String> = f.entries().iter().map(|e| e.key.to_string()).collect();
    assert_eq!(keys, ["sf/meta", "sf/overlay", "sf/rpg", "sf/thumbnail"]);
    let overlay = f.entry(&key("sf/overlay")).unwrap();
    assert_eq!((overlay.codec, overlay.version), (Codec::Lz4, 2));
    assert!(overlay.stored_len < overlay.raw_len / 4);
    let big: Vec<u32> = decode(&f.raw(&key("sf/overlay")).unwrap()).unwrap();
    assert_eq!(big.len(), 2000);
    assert_eq!(decode::<(u32, String)>(&f.raw(&key("sf/rpg")).unwrap()).unwrap(), (1, "quest".to_owned()));
    assert!(f.entry(&key("sf/thumbnail")).unwrap().flags.contains(SectionFlags::OPTIONAL));
    assert!(f.verify().is_empty());
    // Deterministic: equal input, equal bytes; insertion order doesn't matter.
    assert_eq!(sample(1), bytes);
    let mut w = SaveWriter::new(FileKind::World, &meta(1, SaveReason::Manual));
    w.add(key("sf/thumbnail"), 1, SectionFlags::OPTIONAL, &[1, 2, 3]).unwrap();
    w.add(key("sf/overlay"), 2, SectionFlags::NONE, &encode(&(0..2000).map(|i| i % 17).collect::<Vec<u32>>())).unwrap();
    w.add(key("sf/rpg"), 1, SectionFlags::NONE, &encode(&(1u32, "quest".to_owned()))).unwrap();
    assert_eq!(w.finish(), bytes);
}

#[test]
fn writer_refuses_duplicates_and_meta() {
    let mut w = SaveWriter::new(FileKind::World, &SaveMeta::default());
    w.add(key("sf/rpg"), 1, SectionFlags::NONE, &[]).unwrap();
    assert_eq!(w.add(key("sf/rpg"), 1, SectionFlags::NONE, &[]), Err(WriteError::Duplicate(key("sf/rpg"))));
    assert_eq!(w.add(meta_key(), 1, SectionFlags::NONE, &[]), Err(WriteError::Reserved));
}

#[test]
fn meta_only_read_needs_only_the_head_of_the_file() {
    let bytes = sample(3);
    let (kind, m) = read_meta(&mut Cursor::new(&bytes), &Limits::default()).unwrap();
    assert_eq!((kind, m), (FileKind::World, meta(3, SaveReason::Manual)));
    // Corrupting a later section doesn't stop the list from reading the meta.
    let f = SaveFile::open(bytes.clone()).unwrap();
    let rpg = f.entry(&key("sf/rpg")).unwrap();
    let mut bad = bytes;
    let at = usize::try_from(f.preamble().area_start() + rpg.offset).unwrap();
    bad[at] ^= 1;
    assert!(read_meta(&mut Cursor::new(&bad), &Limits::default()).is_ok());
    let f = SaveFile::open(bad).unwrap();
    assert_eq!(f.verify(), [ContainerError::SectionHash { key: key("sf/rpg") }]);
    assert!(f.raw(&key("sf/overlay")).is_ok());
}

#[test]
fn dormant_sections_copy_byte_for_byte() {
    let src = SaveFile::open(sample(1)).unwrap();
    let e = src.entry(&key("sf/overlay")).unwrap().clone();
    let stored = src.stored(&e.key).unwrap();
    let mut w = SaveWriter::new(FileKind::World, &meta(2, SaveReason::Auto));
    w.add_stored(&e, stored, e.flags.with(SectionFlags::DORMANT)).unwrap();
    assert_eq!(w.add_stored(&e, &stored[1..], SectionFlags::NONE), Err(WriteError::Duplicate(e.key.clone())));
    let out = SaveFile::open(w.finish()).unwrap();
    let copied = out.entry(&e.key).unwrap();
    assert_eq!((copied.hash, copied.version, copied.codec, copied.raw_len), (e.hash, e.version, e.codec, e.raw_len));
    assert!(copied.flags.contains(SectionFlags::DORMANT));
    assert_eq!(out.stored(&e.key).unwrap(), stored);
    let mut w = SaveWriter::new(FileKind::World, &meta(2, SaveReason::Auto));
    assert_eq!(w.add_stored(&e, &stored[1..], SectionFlags::NONE), Err(WriteError::CopyHash(e.key.clone())));
}

/// Opening and fully reading a file: every section decompressed and the meta decoded. Must
/// return, never panic.
fn read_all(bytes: &[u8]) -> Result<(), ContainerError> {
    let f = SaveFile::open(bytes.to_vec())?;
    f.meta()?;
    for e in f.entries() {
        f.raw(&e.key)?;
    }
    let _ = read_meta(&mut Cursor::new(bytes), &Limits::default())?;
    Ok(())
}

#[test]
fn truncation_at_every_offset_is_an_error() {
    let bytes = sample(1);
    for len in 0..bytes.len() {
        let r = read_all(&bytes[..len]);
        assert!(r.is_err(), "truncated to {len} bytes was accepted");
    }
    let mut longer = bytes.clone();
    longer.push(0);
    assert!(matches!(read_all(&longer), Err(ContainerError::Length { .. })));
}

#[test]
fn every_single_bit_flip_is_detected() {
    let bytes = sample(1);
    for i in 0..bytes.len() {
        for bit in 0..8 {
            let mut bad = bytes.clone();
            bad[i] ^= 1 << bit;
            assert!(read_all(&bad).is_err(), "flip of bit {bit} at byte {i} went unnoticed");
        }
    }
}

/// xorshift64*: a tiny deterministic generator for the mutation fuzz.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % n as u64).unwrap()
    }
}

/// Re-signs a mutated TOC (hash and lengths) so mutations reach the TOC decoder and the section
/// checks behind the hash, not just the hash check.
fn resign(bytes: &mut Vec<u8>, toc: &[u8]) {
    let mut out = bytes[..PREAMBLE_LEN].to_vec();
    let tail_start = usize::try_from(u64::from_le_bytes(bytes[8..16].try_into().unwrap()) + u64::from_le_bytes(bytes[16..24].try_into().unwrap()))
        .unwrap()
        .min(bytes.len());
    out.extend_from_slice(toc);
    out.extend_from_slice(&bytes[tail_start..]);
    out[16..24].copy_from_slice(&(toc.len() as u64).to_le_bytes());
    let total = out.len() as u64;
    out[56..64].copy_from_slice(&total.to_le_bytes());
    let mut h = blake3::Hasher::new();
    h.update(&out[..24]);
    h.update(&out[56..64]);
    h.update(toc);
    out[24..56].copy_from_slice(h.finalize().as_bytes());
    *bytes = out;
}

#[test]
fn mutation_fuzz_never_panics() {
    let base = sample(1);
    let f = SaveFile::open(base.clone()).unwrap();
    let toc_range = PREAMBLE_LEN..usize::try_from(f.preamble().area_start()).unwrap();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for round in 0..20_000 {
        let mut bytes = base.clone();
        // Half the rounds mutate the TOC and re-sign it, so they get past the TOC hash.
        let mut toc = bytes[toc_range.clone()].to_vec();
        let target_toc = round % 2 == 0;
        for _ in 0..=rng.below(4) {
            let buf = if target_toc { &mut toc } else { &mut bytes };
            match rng.below(4) {
                0 if !buf.is_empty() => {
                    let i = rng.below(buf.len());
                    buf[i] = u8::try_from(rng.next() & 0xff).unwrap();
                }
                1 if !buf.is_empty() => {
                    let i = rng.below(buf.len());
                    buf.remove(i);
                }
                2 => {
                    let i = rng.below(buf.len() + 1);
                    buf.insert(i, u8::try_from(rng.next() & 0xff).unwrap());
                }
                _ => {
                    let n = rng.below(buf.len() + 1);
                    buf.truncate(n);
                }
            }
        }
        if target_toc && bytes.len() >= PREAMBLE_LEN {
            resign(&mut bytes, &toc);
        }
        let _ = read_all(&bytes);
    }
}

#[test]
fn limits_are_enforced() {
    let bytes = sample(1);
    let tight = Limits { max_sections: 2, ..Limits::default() };
    assert!(matches!(SaveFile::open_with(bytes.clone(), &tight), Err(ContainerError::TooManySections { count: 4, max: 2 })));
    let tight = Limits { max_toc_len: 8, ..Limits::default() };
    assert!(matches!(SaveFile::open_with(bytes.clone(), &tight), Err(ContainerError::TocBounds { .. })));
    let tight = Limits { max_raw_len: 100, ..Limits::default() };
    assert!(matches!(SaveFile::open_with(bytes, &tight), Err(ContainerError::RawLength { .. })));
}

#[test]
fn malformed_tocs_are_named() {
    let base = sample(1);
    let f = SaveFile::open(base.clone()).unwrap();
    let rebuild = |toc: &[TocEntry]| {
        let mut bytes = base.clone();
        resign(&mut bytes, &encode(&toc.to_vec()));
        read_all(&bytes).unwrap_err()
    };
    let mut toc = f.entries().to_vec();
    toc.swap(0, 1);
    assert_eq!(rebuild(&toc), ContainerError::MetaFirst);
    let mut toc = f.entries().to_vec();
    toc[2].key = toc[1].key.clone();
    assert!(matches!(rebuild(&toc), ContainerError::DuplicateKey(_)));
    let mut toc = f.entries().to_vec();
    toc[1].offset += 1;
    assert!(matches!(rebuild(&toc), ContainerError::Layout { .. }));
    let mut toc = f.entries().to_vec();
    toc[1].raw_len = toc[1].stored_len * 300;
    assert!(matches!(rebuild(&toc), ContainerError::RawLength { .. }));
    let mut toc = f.entries().to_vec();
    toc[1].raw_len += 1;
    assert!(matches!(rebuild(&toc), ContainerError::Decompress { .. } | ContainerError::RawLength { .. }));
    let mut bytes = base.clone();
    bytes[4] = 9;
    assert_eq!(SaveFile::open(bytes).unwrap_err(), ContainerError::UnsupportedVersion { found: 9 });
    let mut bytes = base.clone();
    bytes[0] = b'X';
    assert_eq!(SaveFile::open(bytes).unwrap_err(), ContainerError::BadMagic);
    let mut bytes = base;
    bytes[6] = 5;
    assert_eq!(SaveFile::open(bytes).unwrap_err(), ContainerError::UnknownKind(5));
}

// ---- atomic writes over a fault-injecting file system --------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fault {
    /// The data write fails after writing half the bytes (a crash mid-write).
    Write,
    /// The read-back returns different bytes (bad sector, lying cache).
    Corrupt,
    ReadBack,
    /// The nth rename fails (0: old → .bak, 1: tmp → target when a backup is kept).
    Rename(usize),
    SyncDir,
}

#[derive(Default)]
struct MemFs {
    files: Mutex<BTreeMap<PathBuf, Vec<u8>>>,
    fault: Mutex<Option<Fault>>,
    renames: Mutex<usize>,
}

impl MemFs {
    fn fail(&self, f: Option<Fault>) {
        *self.fault.lock().unwrap() = f;
        *self.renames.lock().unwrap() = 0;
    }
    fn fault(&self) -> Option<Fault> {
        *self.fault.lock().unwrap()
    }
    fn get(&self, p: &Path) -> Option<Vec<u8>> {
        self.files.lock().unwrap().get(p).cloned()
    }
    fn names(&self) -> Vec<String> {
        self.files.lock().unwrap().keys().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()).collect()
    }
}

fn injected() -> io::Error {
    io::Error::other("injected fault")
}

impl SaveFs for MemFs {
    fn write_synced(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        if self.fault() == Some(Fault::Write) {
            self.files.lock().unwrap().insert(path.to_owned(), bytes[..bytes.len() / 2].to_vec());
            return Err(injected());
        }
        self.files.lock().unwrap().insert(path.to_owned(), bytes.to_vec());
        Ok(())
    }
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        match self.fault() {
            Some(Fault::ReadBack) => Err(injected()),
            Some(Fault::Corrupt) => {
                let mut b = self.get(path).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
                b[0] ^= 1;
                Ok(b)
            }
            _ => self.get(path).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound)),
        }
    }
    fn open(&self, path: &Path) -> io::Result<Box<dyn ReadSeek>> {
        Ok(Box::new(Cursor::new(self.get(path).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?)))
    }
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        let n = {
            let mut r = self.renames.lock().unwrap();
            *r += 1;
            *r - 1
        };
        if self.fault() == Some(Fault::Rename(n)) {
            return Err(injected());
        }
        let mut files = self.files.lock().unwrap();
        let b = files.remove(from).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
        files.insert(to.to_owned(), b);
        Ok(())
    }
    fn sync_dir(&self, _dir: &Path) -> io::Result<()> {
        if self.fault() == Some(Fault::SyncDir) { Err(injected()) } else { Ok(()) }
    }
    fn remove(&self, path: &Path) -> io::Result<()> {
        self.files.lock().unwrap().remove(path).map(|_| ()).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    }
    fn exists(&self, path: &Path) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }
    fn create_dir_all(&self, _dir: &Path) -> io::Result<()> {
        Ok(())
    }
    fn list(&self, dir: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(self.files.lock().unwrap().keys().filter(|p| p.parent() == Some(dir)).cloned().collect())
    }
    fn list_dirs(&self, _dir: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

#[test]
fn atomic_write_survives_a_failure_at_every_step() {
    let path = Path::new("/saves/w/000001-manual.sfsave");
    let old = sample(1);
    let new = sample(2);
    for fault in [Fault::Write, Fault::Corrupt, Fault::ReadBack, Fault::Rename(0), Fault::Rename(1)] {
        let fs = MemFs::default();
        fs.write_synced(path, &old).unwrap();
        fs.fail(Some(fault));
        let r = write_atomic(&fs, path, &new, true);
        assert!(r.is_err(), "{fault:?} reported success");
        fs.fail(None);
        assert_eq!(fs.get(path).as_deref(), Some(old.as_slice()), "{fault:?} lost the old save");
        assert!(!fs.exists(&with_suffix(path, "tmp")), "{fault:?} left the temp file");
    }
    // A failed directory sync still replaces the file but says it isn't durable.
    let fs = MemFs::default();
    fs.write_synced(path, &old).unwrap();
    fs.fail(Some(Fault::SyncDir));
    let w = write_atomic(&fs, path, &new, true).unwrap();
    assert!(!w.durable);
    assert_eq!(fs.get(path).as_deref(), Some(new.as_slice()));
    assert_eq!(fs.get(&with_suffix(path, "bak")).as_deref(), Some(old.as_slice()));
}

#[test]
fn atomic_write_keeps_one_backup() {
    let fs = MemFs::default();
    let path = Path::new("/s/x.sfsave");
    for (i, seq) in [1u64, 2, 3].into_iter().enumerate() {
        let w = write_atomic(&fs, path, &sample(seq), true).unwrap();
        assert!(w.durable);
        assert_eq!(w.backup.is_some(), i > 0);
    }
    assert_eq!(fs.get(path), Some(sample(3)));
    assert_eq!(fs.get(&with_suffix(path, "bak")), Some(sample(2)));
    assert_eq!(fs.names(), ["x.sfsave", "x.sfsave.bak"]);
}

#[test]
fn std_fs_writes_atomically_on_disk() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a").join("save.sfsave");
    write_atomic(&StdFs, &path, b"one", true).unwrap();
    let w = write_atomic(&StdFs, &path, b"two", true).unwrap();
    assert!(w.durable);
    assert_eq!(std::fs::read(&path).unwrap(), b"two");
    assert_eq!(std::fs::read(with_suffix(&path, "bak")).unwrap(), b"one");
    assert!(!with_suffix(&path, "tmp").exists());
}

#[test]
fn slots_rotate_automatic_saves_only() {
    let fs = MemFs::default();
    let slots = SlotDir::at(PathBuf::from("/saves/single/w"));
    let reasons = [
        SaveReason::Manual,
        SaveReason::Auto,
        SaveReason::Quick,
        SaveReason::Auto,
        SaveReason::Quick,
        SaveReason::GuestLeft,
        SaveReason::Quick,
        SaveReason::Auto,
        SaveReason::Exit,
        SaveReason::Exit,
        SaveReason::Manual,
    ];
    for reason in reasons {
        let seq = slots.next_sequence(&fs);
        let mut w = SaveWriter::new(FileKind::World, &meta(seq, reason));
        w.add(key("sf/rpg"), 1, SectionFlags::NONE, &[1]).unwrap();
        slots.write(&fs, seq, reason, &w.finish()).unwrap();
    }
    let left: Vec<String> = slots.list(&fs).iter().map(|e| e.path.file_name().unwrap().to_string_lossy().into_owned()).collect();
    assert_eq!(
        left,
        [
            "000001-manual.sfsave",
            "000004-auto.sfsave",
            "000005-quick.sfsave",
            "000006-guestleft.sfsave",
            "000007-quick.sfsave",
            "000008-auto.sfsave",
            "000010-exit.sfsave",
            "000011-manual.sfsave",
        ]
    );
    let all = slots.list(&fs);
    assert!(all.iter().all(|e| e.meta.as_ref().is_ok_and(|m| m.sequence == e.sequence && m.reason == e.reason)));
    assert_eq!(slots.latest(&fs, Some(SaveReason::Quick)).unwrap().sequence, 7);
    assert_eq!(slots.latest(&fs, None).unwrap().sequence, 11);
    // A non-durable write never rotates.
    fs.fail(Some(Fault::SyncDir));
    let w = slots.write(&fs, 12, SaveReason::Quick, &sample(12)).unwrap();
    assert!(w.rotated.is_empty() && !w.written.durable);
    fs.fail(None);
    assert_eq!(slots.list(&fs).iter().filter(|e| e.reason == SaveReason::Quick).count(), 3);
}

#[test]
fn slot_listing_reports_unreadable_files() {
    let fs = MemFs::default();
    let slots = SlotDir::at(PathBuf::from("/w"));
    fs.write_synced(&slots.path_of(1, SaveReason::Auto), b"garbage").unwrap();
    fs.write_synced(&slots.path_of(2, SaveReason::Auto), &sample(2)).unwrap();
    fs.write_synced(Path::new("/w/notes.txt"), b"x").unwrap();
    let list = slots.list(&fs);
    assert_eq!(list.len(), 2);
    assert!(list[0].meta.is_err());
    assert_eq!(slots.latest(&fs, None).unwrap().sequence, 2);
    assert_eq!(slots.next_sequence(&fs), 3);
}

#[test]
fn list_worlds_reads_hex_folders() {
    let dir = tempfile::tempdir().unwrap();
    let a = SlotDir::world(dir.path(), SaveMode::Single, 0xbeef);
    std::fs::create_dir_all(a.dir()).unwrap();
    std::fs::create_dir_all(saves_dir(dir.path(), SaveMode::Single).join("not-a-world")).unwrap();
    let worlds = list_worlds(&StdFs, dir.path(), SaveMode::Single);
    assert_eq!(worlds, [(0xbeef, a)]);
    assert!(list_worlds(&StdFs, dir.path(), SaveMode::Coop).is_empty());
}

#[test]
fn cli_info_verify_dump() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("s.sfsave");
    std::fs::write(&path, sample(5)).unwrap();
    let p = path.to_string_lossy().into_owned();
    let run = |args: &[&str], decoders: &[cli::Decoder]| {
        let mut out = Vec::new();
        let args: Vec<String> = args.iter().map(|s| (*s).to_owned()).collect();
        let code = cli::run(&args, &mut out, decoders);
        (code, String::from_utf8(out).unwrap())
    };
    let (code, text) = run(&["info", &p], &[]);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("sf/overlay") && text.contains("lz4") && text.contains("optional"), "{text}");
    assert!(text.contains("sequence 5") && text.contains("\"Tester\""), "{text}");
    let (code, text) = run(&["verify", &p], &[]);
    assert_eq!((code, text.starts_with("ok: 4 sections")), (0, true), "{text}");
    let decoders = [cli::Decoder {
        family: Family { owner: Some("sf".into()), family: "rpg".into() },
        decode: |_, raw| decode::<(u32, String)>(raw).map(|v| format!("{v:?}")).map_err(|e| e.0),
    }];
    let (code, text) = run(&["dump", &p], &decoders);
    assert_eq!(code, 0);
    assert!(text.contains("(1, \"quest\")") && text.contains("== sf/meta") && text.contains("000000  "), "{text}");
    let raw_path = dir.path().join("rpg.bin");
    let (code, _) = run(&["dump", &p, "sf/rpg", "--out", &raw_path.to_string_lossy()], &[]);
    assert_eq!(code, 0);
    assert_eq!(std::fs::read(&raw_path).unwrap(), encode(&(1u32, "quest".to_owned())));
    assert_eq!(run(&["dump", &p, "sf/nope"], &[]).0, 1);
    assert_eq!(run(&["bogus"], &[]).0, 2);
    // A corrupt section fails verify with its name.
    let mut bad = sample(5);
    let len = bad.len();
    bad[len - 1] ^= 0xff;
    std::fs::write(&path, bad).unwrap();
    let (code, text) = run(&["verify", &p], &[]);
    assert_eq!(code, 1);
    assert!(text.contains("sf/thumbnail"), "{text}");
}
