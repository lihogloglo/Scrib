//! Save folders, file names and rotation (save.md "Slots and files").
//!
//! ```text
//! <user data>/saves/single/<world id>/<sequence>-<reason>.sfsave
//! <user data>/saves/coop/<world id>/...
//! <user data>/characters/<world id>/<player key>.sfchar
//! ```
//!
//! A folder per world groups a playthrough. Every save is a new file named by its sequence
//! number; automatic saves rotate once a newer one is verified and durable. Overwriting a chosen
//! save keeps the old file as `.bak` until the next verified write.

use std::path::{Path, PathBuf};

use crate::container::{Limits, read_meta};
use crate::fs::{SaveFs, WriteFileError, Written, write_atomic};
use crate::meta::{SaveMeta, SaveReason};

/// World save extension.
pub const SAVE_EXT: &str = "sfsave";
/// Guest character file extension.
pub const CHARACTER_EXT: &str = "sfchar";

/// Single-player and co-op world saves live apart (net.md "Saves").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SaveMode {
    Single,
    Coop,
}

impl SaveMode {
    fn dir_name(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Coop => "coop",
        }
    }
}

/// How many saves of each automatic kind to keep. Manual saves never rotate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RotationPolicy {
    /// `Auto` and `GuestLeft` saves (both automatic, one rotation).
    pub auto: usize,
    pub quick: usize,
    pub exit: usize,
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self { auto: 3, quick: 2, exit: 1 }
    }
}

impl RotationPolicy {
    /// The rotation group and its size; `None` for manual saves.
    fn group(&self, reason: SaveReason) -> Option<(&'static [SaveReason], usize)> {
        match reason {
            SaveReason::Manual => None,
            SaveReason::Quick => Some((&[SaveReason::Quick], self.quick)),
            SaveReason::Exit => Some((&[SaveReason::Exit], self.exit)),
            SaveReason::Auto | SaveReason::GuestLeft => Some((&[SaveReason::Auto, SaveReason::GuestLeft], self.auto)),
        }
    }
}

/// One file of a world's folder.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotEntry {
    pub path: PathBuf,
    pub sequence: u64,
    pub reason: SaveReason,
    /// The meta-only read (preamble, TOC, `sf/meta`); an error for an unreadable file.
    pub meta: Result<SaveMeta, String>,
}

/// What [`SlotDir::write`] did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotWrite {
    pub written: Written,
    /// Older automatic saves deleted by rotation.
    pub rotated: Vec<PathBuf>,
}

/// One world's save folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotDir {
    dir: PathBuf,
    pub policy: RotationPolicy,
}

/// `<root>/saves/<mode>/`.
pub fn saves_dir(root: &Path, mode: SaveMode) -> PathBuf {
    root.join("saves").join(mode.dir_name())
}

/// World ids print as 32 lowercase hex digits.
pub fn id_hex(id: u128) -> String {
    format!("{id:032x}")
}

/// `<root>/characters/<world id>/<player key>.sfchar`.
pub fn character_path(root: &Path, world_id: u128, player_key: u128) -> PathBuf {
    root.join("characters").join(id_hex(world_id)).join(format!("{}.{CHARACTER_EXT}", id_hex(player_key)))
}

/// The worlds with a save folder under `<root>/saves/<mode>/`.
pub fn list_worlds(fs: &dyn SaveFs, root: &Path, mode: SaveMode) -> Vec<(u128, SlotDir)> {
    let mut out: Vec<(u128, SlotDir)> = fs
        .list_dirs(&saves_dir(root, mode))
        .unwrap_or_default()
        .into_iter()
        .filter_map(|d| {
            let name = d.file_name()?.to_str()?;
            let id = u128::from_str_radix(name, 16).ok().filter(|_| name.len() == 32)?;
            Some((id, SlotDir::at(d)))
        })
        .collect();
    out.sort_by_key(|(id, _)| *id);
    out
}

/// Parses `<sequence>-<reason>.sfsave`.
pub fn parse_file_name(path: &Path) -> Option<(u64, SaveReason)> {
    if path.extension()?.to_str()? != SAVE_EXT {
        return None;
    }
    let stem = path.file_stem()?.to_str()?;
    let (seq, reason) = stem.split_once('-')?;
    Some((seq.parse().ok()?, SaveReason::from_token(reason)?))
}

impl SlotDir {
    /// The folder of `world_id` under `<root>/saves/<mode>/`.
    pub fn world(root: &Path, mode: SaveMode, world_id: u128) -> Self {
        Self::at(saves_dir(root, mode).join(id_hex(world_id)))
    }

    /// A folder at an explicit path.
    pub fn at(dir: PathBuf) -> Self {
        Self { dir, policy: RotationPolicy::default() }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// `000042-quick.sfsave`.
    pub fn file_name(sequence: u64, reason: SaveReason) -> String {
        format!("{sequence:06}-{reason}.{SAVE_EXT}")
    }

    pub fn path_of(&self, sequence: u64, reason: SaveReason) -> PathBuf {
        self.dir.join(Self::file_name(sequence, reason))
    }

    /// The folder's saves by sequence, oldest first, each with its meta-only read. Temp and
    /// backup files are not listed.
    pub fn list(&self, fs: &dyn SaveFs) -> Vec<SlotEntry> {
        let mut out: Vec<SlotEntry> = fs
            .list(&self.dir)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|path| {
                let (sequence, reason) = parse_file_name(&path)?;
                let meta = fs
                    .open(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|mut r| read_meta(&mut r, &Limits::default()).map(|(_, m)| m).map_err(|e| e.to_string()));
                Some(SlotEntry { path, sequence, reason, meta })
            })
            .collect();
        out.sort_by(|a, b| (a.sequence, &a.path).cmp(&(b.sequence, &b.path)));
        out
    }

    /// The newest readable save, of `reason` if given.
    pub fn latest(&self, fs: &dyn SaveFs, reason: Option<SaveReason>) -> Option<SlotEntry> {
        self.list(fs).into_iter().rev().find(|e| e.meta.is_ok() && reason.is_none_or(|r| r == e.reason))
    }

    /// One past the highest sequence on disk (1 for an empty folder).
    pub fn next_sequence(&self, fs: &dyn SaveFs) -> u64 {
        self.list(fs).last().map_or(1, |e| e.sequence + 1)
    }

    /// Writes a new save atomically, then rotates its automatic group if the write is durable.
    pub fn write(&self, fs: &dyn SaveFs, sequence: u64, reason: SaveReason, bytes: &[u8]) -> Result<SlotWrite, WriteFileError> {
        let path = self.path_of(sequence, reason);
        let written = write_atomic(fs, &path, bytes, true)?;
        let mut rotated = Vec::new();
        if let (true, Some((group, keep))) = (written.durable, self.policy.group(reason)) {
            let mut same: Vec<SlotEntry> = self.list(fs).into_iter().filter(|e| group.contains(&e.reason)).collect();
            let excess = same.len().saturating_sub(keep.max(1));
            for old in same.drain(..excess) {
                if old.path != path && fs.remove(&old.path).is_ok() {
                    rotated.push(old.path);
                }
            }
        }
        Ok(SlotWrite { written, rotated })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_names_round_trip() {
        for reason in SaveReason::ALL {
            let name = SlotDir::file_name(42, reason);
            assert_eq!(parse_file_name(Path::new(&name)), Some((42, reason)));
        }
        assert_eq!(SlotDir::file_name(7, SaveReason::Quick), "000007-quick.sfsave");
        assert_eq!(parse_file_name(Path::new("000007-quick.sfsave.bak")), None);
        assert_eq!(parse_file_name(Path::new("x-quick.sfsave")), None);
        assert_eq!(parse_file_name(Path::new("7-nope.sfsave")), None);
    }

    #[test]
    fn paths_follow_the_layout() {
        let root = Path::new("/u");
        assert_eq!(SlotDir::world(root, SaveMode::Coop, 0xab).dir(), Path::new("/u/saves/coop/000000000000000000000000000000ab"));
        assert_eq!(
            character_path(root, 1, 2),
            Path::new("/u/characters/00000000000000000000000000000001/00000000000000000000000000000002.sfchar")
        );
    }
}
