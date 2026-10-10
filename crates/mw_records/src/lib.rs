//! Morrowind content files (ESM/ESP) applied onto a [`RecordDb`].
//!
//! Every content file becomes one record source, and a synthesized mod for
//! load-order resolution (its masters become dependencies). Merge rules
//! follow the format's semantics:
//!
//! - Most kinds: a later file's record fully replaces the earlier one.
//!   The DELETED flag deletes it.
//! - Cells: later files *merge* into the cell. Header fields are replaced,
//!   references are added, replaced, or deleted one by one. Reference keys
//!   are rewritten from file-local master indices to global source IDs:
//!   in the database, `Cell::references` is keyed by
//!   `(SourceId of the defining file, refr_index)`, and
//!   `Reference::mast_index` holds that same `SourceId`.
//! - Dialogue responses (INFO) are keyed under their topic (see [`keys`]).
//!   Their `prev_id`/`next_id` links are kept; response *order* is resolved
//!   by the dialogue system, not here.

pub mod keys;

use std::time::{Duration, Instant};

use semver::{Version, VersionReq};
use sf_mods::{LoadOrderError, ModId, ModManifest, resolve_load_order};
use sf_records::{RecordDb, RecordError, RecordId, SourceId};
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Cell, Dialogue, EditorId, FileType, ObjectInfo, Plugin, TES3Object};

macro_rules! kinds {
    (
        simple { $($variant:ident => $tag:literal),* $(,)? }
        special { $($svariant:ident => $stag:literal),* $(,)? }
    ) => {
        /// Every record kind tag, in format order.
        pub const KINDS: &[&str] = &[$($tag,)* $($stag,)*];

        /// Registers every TES3 record kind under its four-letter tag.
        pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
            $(db.register::<tes3::esp::$variant>($tag)?;)*
            $(db.register::<tes3::esp::$svariant>($stag)?;)*
            Ok(())
        }

        /// Kinds whose records simply replace earlier definitions.
        fn apply_simple(db: &mut RecordDb, source: SourceId, object: TES3Object) -> Result<(), RecordError> {
            match object {
                $(TES3Object::$variant(record) => replace(db, source, record),)*
                other => unreachable!("{} is not a simple kind", tes3::esp::TypeInfo::tag_str(&other)),
            }
        }

        /// Runs `visitor` with the concrete type behind a kind tag
        /// (case-insensitive). Returns `None` for unknown tags.
        pub fn visit_kind<R>(tag: &str, visitor: impl KindVisitor<Output = R>) -> Option<R> {
            match tag.to_ascii_uppercase().as_str() {
                $($tag => Some(visitor.visit::<tes3::esp::$variant>()),)*
                $($stag => Some(visitor.visit::<tes3::esp::$svariant>()),)*
                _ => None,
            }
        }
    };
}

kinds! {
    simple {
        GameSetting => "GMST", GlobalVariable => "GLOB", Class => "CLAS", Faction => "FACT",
        Race => "RACE", Sound => "SOUN", SoundGen => "SNDG", Skill => "SKIL", MagicEffect => "MGEF",
        Script => "SCPT", Region => "REGN", Birthsign => "BSGN", StartScript => "SSCR",
        LandscapeTexture => "LTEX", Spell => "SPEL", Static => "STAT", Door => "DOOR",
        MiscItem => "MISC", Weapon => "WEAP", Container => "CONT", Creature => "CREA",
        Bodypart => "BODY", Light => "LIGH", Enchanting => "ENCH", Npc => "NPC_", Armor => "ARMO",
        Clothing => "CLOT", RepairItem => "REPA", Activator => "ACTI", Apparatus => "APPA",
        Lockpick => "LOCK", Probe => "PROB", Ingredient => "INGR", Book => "BOOK", Alchemy => "ALCH",
        LeveledItem => "LEVI", LeveledCreature => "LEVC", Dialogue => "DIAL",
    }
    special {
        Cell => "CELL", Landscape => "LAND", PathGrid => "PGRD", DialogueInfo => "INFO",
    }
}

/// Generic code over a record kind chosen at runtime (see [`visit_kind`]).
pub trait KindVisitor {
    type Output;
    fn visit<T: std::fmt::Debug + Send + Sync + 'static>(self) -> Self::Output;
}

/// A content file as loaded.
#[derive(Clone, Debug)]
pub struct ContentFile {
    /// File name as configured (e.g. `Morrowind.esm`).
    pub name: String,
    pub source: SourceId,
    pub file_type: FileType,
    /// Master file names, as the header lists them.
    pub masters: Vec<String>,
    pub objects: usize,
    pub read_time: Duration,
    pub parse_time: Duration,
    pub apply_time: Duration,
}

/// Non-fatal content problems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadWarning {
    /// An INFO record appeared before any DIAL record in its file.
    OrphanInfo { file: String, info: String },
    /// A reference names a master index the file's header doesn't have.
    BadMasterIndex { file: String, cell: String, mast_index: u32 },
    /// References moved between exterior cells (MVRF); not applied yet.
    MovedReferences { file: String, count: usize },
    /// A reference deletion targets a reference that does not exist.
    DeletedMissingReference { file: String, cell: String },
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("content file {0} not found in the VFS")]
    NotFound(String),
    #[error("invalid content file name {0:?}")]
    BadName(String),
    #[error("failed to read {name}: {source}")]
    Read { name: String, source: std::io::Error },
    #[error("failed to parse {name}: {source}")]
    Parse { name: String, source: std::io::Error },
    #[error("{name} has no TES3 header")]
    NoHeader { name: String },
    #[error(transparent)]
    Order(#[from] LoadOrderError),
    #[error(transparent)]
    Records(#[from] RecordError),
}

/// The loaded game data.
pub struct LoadedContent {
    pub db: RecordDb,
    /// Content files in the order they were applied.
    pub files: Vec<ContentFile>,
    pub warnings: Vec<LoadWarning>,
}

/// Mod ID for a content file: lowercased, with characters outside
/// `[a-z0-9_.-]` replaced by `_`.
pub fn mod_id_for(file_name: &str) -> Result<ModId, LoadError> {
    let id: String = file_name
        .chars()
        .map(|c| {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '-' | '_') { c } else { '_' }
        })
        .collect();
    ModId::new(id).map_err(|_| LoadError::BadName(file_name.to_owned()))
}

struct Pending {
    name: String,
    bytes: Vec<u8>,
    file_type: FileType,
    masters: Vec<String>,
    read_time: Duration,
}

/// Loads `requested` content files (configured order, e.g. from
/// `Morrowind.ini`) from the VFS root.
///
/// Final order: masters (ESM) before plugins (ESP), each in configured
/// order, then adjusted so every file loads after the masters it names.
pub fn load(vfs: &Vfs, requested: &[String]) -> Result<LoadedContent, LoadError> {
    // Read every file once and peek at its header for the dependency graph.
    let mut pending = Vec::with_capacity(requested.len());
    for name in requested {
        let path = VfsPath::new(name).map_err(|_| LoadError::BadName(name.clone()))?;
        if !vfs.contains(&path) {
            return Err(LoadError::NotFound(name.clone()));
        }
        let start = Instant::now();
        let bytes = vfs.read(&path).map_err(|source| LoadError::Read { name: name.clone(), source })?;
        let read_time = start.elapsed();

        let mut header_only = Plugin::new();
        header_only
            .load_bytes_filtered(&bytes, |tag| tag == *b"TES3")
            .map_err(|source| LoadError::Parse { name: name.clone(), source })?;
        let header = header_only.header().ok_or_else(|| LoadError::NoHeader { name: name.clone() })?;
        pending.push(Pending {
            name: name.clone(),
            file_type: header.file_type,
            masters: header.masters.iter().map(|(m, _)| m.clone()).collect(),
            bytes,
            read_time,
        });
    }

    let order = resolve_order(&pending)?;
    let mut db = RecordDb::new();
    register_kinds(&mut db)?;
    let mut files: Vec<ContentFile> = Vec::with_capacity(order.len());
    let mut warnings = Vec::new();

    for index in order {
        let file = &pending[index];
        let master_sources: Vec<Option<SourceId>> = file
            .masters
            .iter()
            .map(|m| files.iter().find(|f| f.name.eq_ignore_ascii_case(m)).map(|f| f.source))
            .collect();
        let dependencies: Vec<SourceId> = master_sources.iter().flatten().copied().collect();
        let source = db.add_source(file.name.clone(), &dependencies);

        let start = Instant::now();
        let mut plugin = Plugin::new();
        plugin.load_bytes(&file.bytes).map_err(|source| LoadError::Parse { name: file.name.clone(), source })?;
        let parse_time = start.elapsed();
        let objects = plugin.objects.len();

        let start = Instant::now();
        let mut ctx = ApplyCtx { db: &mut db, source, file: &file.name, masters: &master_sources, warnings: &mut warnings, topic: None, moved: 0 };
        for object in plugin.objects {
            ctx.apply(object)?;
        }
        if ctx.moved > 0 {
            ctx.warnings.push(LoadWarning::MovedReferences { file: file.name.clone(), count: ctx.moved });
        }
        files.push(ContentFile {
            name: file.name.clone(),
            source,
            file_type: file.file_type,
            masters: file.masters.clone(),
            objects,
            read_time: file.read_time,
            parse_time,
            apply_time: start.elapsed(),
        });
    }

    Ok(LoadedContent { db, files, warnings })
}

/// Ordering facts about known content that headers don't state, as
/// `(file, loads after)`. Applied as hints: they only matter when both files
/// are enabled.
///
/// The original engine sorted by file date, but installers (Steam) stamp
/// every file with the install time and `Morrowind.ini` order is arbitrary,
/// so neither is reliable. GOTY Bloodmoon.esm only lists Morrowind.esm as a
/// master, yet it re-includes and supersedes Tribunal's edits.
pub const KNOWN_ORDER: &[(&str, &str)] = &[("bloodmoon.esm", "tribunal.esm")];

/// Indices into `pending`, in load order.
fn resolve_order(pending: &[Pending]) -> Result<Vec<usize>, LoadError> {
    let mut manifests = Vec::with_capacity(pending.len());
    for file in pending {
        let mut manifest = ModManifest::synthesized(mod_id_for(&file.name)?, file.name.clone(), Version::new(0, 0, 0));
        for master in &file.masters {
            manifest.dependencies.insert(mod_id_for(master)?, VersionReq::STAR);
        }
        for (name, after) in KNOWN_ORDER {
            if file.name.eq_ignore_ascii_case(name) {
                manifest.load_after.push(mod_id_for(after)?);
            }
        }
        manifests.push(manifest);
    }
    // Profile: ESMs first, then ESPs, each in configured order.
    let mut profile: Vec<usize> = (0..pending.len()).collect();
    profile.sort_by_key(|&i| pending[i].file_type != FileType::Esm);
    let profile_ids: Vec<ModId> = profile.iter().map(|&i| manifests[i].id.clone()).collect();

    let ordered = resolve_load_order(&manifests, &profile_ids)?;
    Ok(ordered
        .iter()
        .map(|m| manifests.iter().position(|x| x.id == m.id).expect("resolved mods come from manifests"))
        .collect())
}

struct ApplyCtx<'a> {
    db: &'a mut RecordDb,
    source: SourceId,
    file: &'a str,
    /// Global source for each 1-based master index.
    masters: &'a [Option<SourceId>],
    warnings: &'a mut Vec<LoadWarning>,
    /// The most recent DIAL in this file; INFO records belong to it.
    topic: Option<Dialogue>,
    moved: usize,
}

impl ApplyCtx<'_> {
    fn apply(&mut self, object: TES3Object) -> Result<(), RecordError> {
        match object {
            TES3Object::Header(_) => Ok(()),
            TES3Object::Cell(cell) => self.apply_cell(cell),
            TES3Object::Landscape(land) => {
                let id = keys::landscape(&land);
                replace_with_id(self.db, self.source, id, land)
            }
            TES3Object::PathGrid(grid) => {
                let id = keys::path_grid(&grid);
                replace_with_id(self.db, self.source, id, grid)
            }
            TES3Object::DialogueInfo(info) => {
                let Some(topic) = &self.topic else {
                    self.warnings.push(LoadWarning::OrphanInfo { file: self.file.to_owned(), info: info.id.clone() });
                    return Ok(());
                };
                let id = keys::info(topic, &info);
                replace_with_id(self.db, self.source, id, info)
            }
            TES3Object::Dialogue(topic) => {
                self.topic = Some(topic.clone());
                replace(self.db, self.source, topic)
            }
            other => apply_simple(self.db, self.source, other),
        }
    }

    fn apply_cell(&mut self, mut cell: Cell) -> Result<(), RecordError> {
        let id = keys::cell(&cell);
        if cell.deleted() {
            return self.db.delete::<Cell>(self.source, id);
        }

        // Re-key references to global sources.
        let mut changes = Vec::with_capacity(cell.references.len());
        for (_, mut reference) in cell.references.drain() {
            let origin = if reference.mast_index == 0 {
                Some(self.source)
            } else {
                self.masters.get(reference.mast_index as usize - 1).copied().flatten()
            };
            let Some(origin) = origin else {
                self.warnings.push(LoadWarning::BadMasterIndex {
                    file: self.file.to_owned(),
                    cell: id.to_string(),
                    mast_index: reference.mast_index,
                });
                continue;
            };
            if reference.moved_cell.is_some() {
                self.moved += 1;
            }
            reference.mast_index = origin.0;
            changes.push(reference);
        }

        let mut missing_deletes = 0;
        self.db.upsert(
            self.source,
            id.clone(),
            || {
                let mut fresh = cell.clone();
                for reference in changes.iter().filter(|r| r.deleted != Some(true)) {
                    fresh.references.insert((reference.mast_index, reference.refr_index), reference.clone());
                }
                fresh
            },
            |existing: &mut Cell| {
                existing.flags = cell.flags;
                existing.name.clone_from(&cell.name);
                existing.data = cell.data.clone();
                if cell.region.is_some() {
                    existing.region.clone_from(&cell.region);
                }
                if cell.map_color.is_some() {
                    existing.map_color = cell.map_color;
                }
                if cell.water_height.is_some() {
                    existing.water_height = cell.water_height;
                }
                if cell.atmosphere_data.is_some() {
                    existing.atmosphere_data.clone_from(&cell.atmosphere_data);
                }
                for reference in &changes {
                    let key = (reference.mast_index, reference.refr_index);
                    if reference.deleted == Some(true) {
                        if existing.references.remove(&key).is_none() {
                            missing_deletes += 1;
                        }
                    } else {
                        existing.references.insert(key, reference.clone());
                    }
                }
            },
        )?;
        for _ in 0..missing_deletes {
            self.warnings.push(LoadWarning::DeletedMissingReference { file: self.file.to_owned(), cell: id.to_string() });
        }
        Ok(())
    }
}

fn replace<T: EditorId + ObjectInfo + Send + Sync + 'static>(
    db: &mut RecordDb,
    source: SourceId,
    record: T,
) -> Result<(), RecordError> {
    let id = RecordId::new(&record.editor_id());
    replace_with_id(db, source, id, record)
}

fn replace_with_id<T: ObjectInfo + Send + Sync + 'static>(
    db: &mut RecordDb,
    source: SourceId,
    id: RecordId,
    record: T,
) -> Result<(), RecordError> {
    if record.deleted() { db.delete::<T>(source, id) } else { db.define(source, id, record) }
}

#[cfg(test)]
mod tests;
