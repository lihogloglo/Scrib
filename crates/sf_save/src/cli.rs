//! The `sf_save` command line: `info`, `verify`, `dump` (save.md "Interfaces").
//!
//! Section values are postcard (not self-describing), so `dump` prints a hex preview unless a
//! [`Decoder`] for the section's family is passed: a game can build its own `sf_save` binary
//! with typed dumps by calling [`run`] with its decoders.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Write;
use std::path::Path;

use crate::container::{Codec, SaveFile, SectionFlags, TocEntry};
use crate::key::SectionKey;
use crate::meta::SaveMeta;
use crate::registry::Family;
use crate::slots::id_hex;

/// Prints one section family's values: `(version, raw bytes) → text`.
pub struct Decoder {
    pub family: Family,
    pub decode: fn(u32, &[u8]) -> Result<String, String>,
}

const USAGE: &str = "usage:
  sf_save info <file>                 header, meta, sections, bytes per owner
  sf_save verify <file>               check every hash; exit 1 if anything is bad
  sf_save dump <file> [<owner/name>] [--out <path>]
                                      meta and section contents (hex unless a decoder knows
                                      the section); --out writes one section's raw bytes";

/// Runs a command; returns the process exit code (0 ok, 1 bad file, 2 usage).
pub fn run(args: &[String], out: &mut dyn Write, decoders: &[Decoder]) -> i32 {
    let result = match args {
        [cmd, file] if cmd == "info" => open(file).map(|f| info(&f)),
        [cmd, file] if cmd == "verify" => open(file).and_then(|f| verify(&f)),
        [cmd, file, rest @ ..] if cmd == "dump" => dump_args(rest).and_then(|(key, out_path)| {
            let f = open(file)?;
            dump(&f, key.as_ref(), out_path, decoders)
        }),
        _ => {
            let _ = writeln!(out, "{USAGE}");
            return 2;
        }
    };
    match result {
        Ok(text) => {
            let _ = out.write_all(text.as_bytes());
            0
        }
        Err(text) => {
            let _ = writeln!(out, "{text}");
            1
        }
    }
}

fn open(file: &str) -> Result<SaveFile, String> {
    let bytes = std::fs::read(file).map_err(|e| format!("{file}: {e}"))?;
    SaveFile::open(bytes).map_err(|e| format!("{file}: {e}"))
}

fn dump_args(rest: &[String]) -> Result<(Option<SectionKey>, Option<&str>), String> {
    let mut key = None;
    let mut out = None;
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        if a == "--out" {
            out = Some(it.next().ok_or("--out needs a path")?.as_str());
        } else if key.is_none() {
            key = Some(SectionKey::parse(a).map_err(|e| e.to_string())?);
        } else {
            return Err(format!("unexpected argument {a:?}\n{USAGE}"));
        }
    }
    if out.is_some() && key.is_none() {
        return Err("--out needs a section".into());
    }
    Ok((key, out))
}

fn flags_text(f: SectionFlags) -> String {
    let mut s = Vec::new();
    if f.contains(SectionFlags::OPTIONAL) {
        s.push("optional");
    }
    if f.contains(SectionFlags::DORMANT) {
        s.push("dormant");
    }
    s.join(",")
}

fn duration_text(ms: u64) -> String {
    let s = ms / 1000;
    format!("{}h{:02}m{:02}s", s / 3600, s / 60 % 60, s % 60)
}

fn meta_text(m: &SaveMeta) -> String {
    let mut t = String::new();
    let parent = m.parent.map_or_else(|| "-".to_owned(), id_hex);
    let _ = writeln!(t, "world {}  save {}  parent {parent}", id_hex(m.world_id), id_hex(m.save_id));
    let _ = writeln!(t, "sequence {}  reason {}  wall clock {} (unix s)", m.sequence, m.reason, m.wall_clock);
    let _ = writeln!(t, "game days {:.4}  play time {}  sim tick {}", m.game_days, duration_text(m.play_time_ms), m.sim_tick);
    let _ = writeln!(
        t,
        "content {} {}  profile {:?}  build {:?}  framework {}  grid cell {} m",
        m.content_id, m.content_version, m.profile, m.build_id, m.framework_version, m.grid_cell
    );
    let level = m.character_level.map(|l| format!(" (level {l})")).unwrap_or_default();
    let _ = writeln!(t, "character {:?}{level} at {:?}", m.character_name, m.location_label);
    let _ = writeln!(t, "party ({}):", m.party.len());
    for p in &m.party {
        let _ = writeln!(t, "  {} {:?}", id_hex(p.player_key), p.name);
    }
    let _ = writeln!(t, "load order ({}), fingerprint {}:", m.load_order.len(), hex(&m.profile_fingerprint[..8]));
    for r in &m.load_order {
        let _ = writeln!(t, "  {} {} manifest {}", r.id, r.version, hex(&r.manifest_hash[..8]));
    }
    t
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn entry_line(e: &TocEntry) -> String {
    let codec = match e.codec {
        Codec::Raw => "raw",
        Codec::Lz4 => "lz4",
    };
    format!("{:<40} v{:<4} {codec:<4} {:>10} {:>10}  {}", e.key.to_string(), e.version, e.stored_len, e.raw_len, flags_text(e.flags))
}

fn info(f: &SaveFile) -> String {
    let mut t = String::new();
    let p = f.preamble();
    let _ = writeln!(t, "kind {:?}  container v{}  {} bytes", p.kind, p.version, f.len());
    match f.meta() {
        Ok(m) => t.push_str(&meta_text(&m)),
        Err(e) => {
            let _ = writeln!(t, "meta: {e}");
        }
    }
    let _ = writeln!(t, "sections ({}):", f.entries().len());
    let _ = writeln!(t, "  {:<40} {:<5} {:<4} {:>10} {:>10}  flags", "key", "ver", "code", "stored", "raw");
    let mut owners: BTreeMap<&str, u64> = BTreeMap::new();
    for e in f.entries() {
        let _ = writeln!(t, "  {}", entry_line(e));
        *owners.entry(e.key.owner()).or_default() += e.stored_len;
    }
    let _ = writeln!(t, "stored bytes per owner:");
    for (owner, bytes) in owners {
        let _ = writeln!(t, "  {owner:<40} {bytes:>10}");
    }
    t
}

fn verify(f: &SaveFile) -> Result<String, String> {
    let bad = f.verify();
    if bad.is_empty() {
        Ok(format!("ok: {} sections, {} bytes\n", f.entries().len(), f.len()))
    } else {
        Err(bad.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n"))
    }
}

fn dump(f: &SaveFile, only: Option<&SectionKey>, out_path: Option<&str>, decoders: &[Decoder]) -> Result<String, String> {
    if let (Some(key), Some(path)) = (only, out_path) {
        let raw = f.raw(key).map_err(|e| e.to_string())?;
        std::fs::write(Path::new(path), &raw).map_err(|e| format!("{path}: {e}"))?;
        return Ok(format!("{key}: {} raw bytes written to {path}\n", raw.len()));
    }
    let mut t = String::new();
    if only.is_none() {
        match f.meta() {
            Ok(m) => {
                let _ = writeln!(t, "== sf/meta\n{m:#?}");
            }
            Err(e) => {
                let _ = writeln!(t, "== sf/meta: {e}");
            }
        }
    }
    let mut found = false;
    for e in f.entries().iter().skip(1).filter(|e| only.is_none_or(|k| *k == e.key)) {
        found = true;
        let _ = writeln!(t, "== {}", entry_line(e));
        match f.raw(&e.key) {
            Err(err) => {
                let _ = writeln!(t, "   {err}");
            }
            Ok(raw) => match decoders.iter().find(|d| d.family.matches(&e.key)) {
                Some(d) => match (d.decode)(e.version, &raw) {
                    Ok(text) => {
                        let _ = writeln!(t, "{text}");
                    }
                    Err(err) => {
                        let _ = writeln!(t, "   decode: {err}\n{}", hex_preview(&raw));
                    }
                },
                None => t.push_str(&hex_preview(&raw)),
            },
        }
    }
    match only {
        Some(key) if !found => Err(format!("no section {key}")),
        _ => Ok(t),
    }
}

/// The first 256 bytes, 32 per line.
fn hex_preview(raw: &[u8]) -> String {
    let mut t = String::new();
    for (i, chunk) in raw.chunks(32).take(8).enumerate() {
        let _ = writeln!(t, "   {:06x}  {}", i * 32, hex(chunk));
    }
    if raw.len() > 256 {
        let _ = writeln!(t, "   ... {} more bytes", raw.len() - 256);
    }
    t
}
