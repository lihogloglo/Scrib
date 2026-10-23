//! Bevy-free save files (`docs/specs/save.md`, slice S1).
//!
//! A save is **one self-contained file**: a 64-byte preamble, a table of contents and
//! **owner-keyed sections** (`sf/overlay`, `mw/globals`, `<mod>/script`), each with its own schema
//! version, postcard-encoded, LZ4-compressed when that helps and blake3-hashed
//! ([`SaveWriter`], [`SaveFile`]). `sf/meta` comes first and raw, so a save list reads only a
//! few KiB ([`read_meta`]).
//!
//! - [`Registry`]: section families, their current and oldest versions and the stepwise
//!   migrations (`v → v + 1`) that load runs; newer versions are refused by name.
//! - [`write_atomic`]: temp file, `fsync`, read-back, optional `.bak`, rename, directory sync,
//!   over a [`SaveFs`] trait so tests can fail each step.
//! - [`SlotDir`]: per-world folders, `<sequence>-<reason>.sfsave` names, rotation of automatic
//!   saves.
//! - [`cli`]: `sf_save info|verify|dump`.
//!
//! Owners of saved state keep their own types and versions; `sf_save_bevy` snapshots and
//! applies them in an app.

pub mod cli;
mod codec;
mod container;
mod fs;
mod key;
mod meta;
mod registry;
mod slots;

#[cfg(test)]
mod tests;

pub use codec::{DecodeError, decode, encode};
pub use container::{
    CONTAINER_VERSION, Codec, ContainerError, FileKind, Limits, MAGIC, PREAMBLE_LEN, Preamble, SaveFile, SaveWriter,
    SectionFlags, TocEntry, WriteError, meta_key, read_meta,
};
pub use fs::{ReadSeek, SaveFs, StdFs, WriteFileError, Written, with_suffix, write_atomic};
pub use key::{KeyError, MAX_NAME_LEN, MAX_OWNER_LEN, SectionKey};
pub use meta::{META_VERSION, MetaError, ModRef, PartyMember, SaveMeta, SaveReason};
pub use registry::{Family, MigrateError, Registry, RegistryError, SectionSpec, StepFn};
pub use slots::{
    CHARACTER_EXT, RotationPolicy, SAVE_EXT, SaveMode, SlotDir, SlotEntry, SlotWrite, character_path, id_hex, list_worlds,
    parse_file_name, saves_dir,
};
