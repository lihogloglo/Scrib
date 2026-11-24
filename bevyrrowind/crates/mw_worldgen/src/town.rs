//! `worldgen-town` (S3a, scrib-worldgen `docs/specs/settlements.md` "Milestone 2 files"): the settlement tool's
//! `placements.json` (format 1) as a **native mod** Scrib loads like any other (`sf_mods::NativeRecords`), one
//! `sf.placement_group` record ([`sf_world::PlacementGroup`]) holding the town's instances.
//!
//! - Mod folder (`--out`): `mod.toml` (id = `--id`, else the folder's name; `[content] assets = ["data"]`,
//!   `records = ["records/town.ron"]`) and `data/records/town.ron`, one `define` of kind `sf.placement_group`, id
//!   `worldgen.town.<name>` ([`group_id`]), worldspace [`mw_world::ids::EXTERIOR`] (the exterior a `view
//!   "world:<guide>"` run streams).
//! - One placement per instance, in file order, key = the instance's `id` (stable, unique; a repeated id is an
//!   error). Base record: the first `base:<record id>` tag (sorted, as `kit.json` writes them) of the instance's kit
//!   model, resolved against the content's records ([`mw_world::source::resolve_base`]: kind tag, class, model), as
//!   the source references flora copies. Skipped and counted: an instance whose kit id is not in `kit.json`, a model
//!   with no `base:` tag, a base that does not resolve.
//! - Transform ([`engine_transform`]): the conversion `flora::placements` applies to its items (guide metres x east,
//!   y north, z up → engine Y up, −Z north; rotation `from_rotation_arc(Y, up) * from_rotation_y(yaw)` with up =
//!   guide +z), yaw = `yaw_deg` (counter-clockwise about +z from above, which is a turn about engine +Y), unit
//!   scale, no flags (the tool's z is final: not `GROUNDED`).
//! - `kit_blake3` must equal the blake3 of the `kit.json` bytes read (else the instances name another kit).
//! - `--streets <streets.json>` (optional): the town's streets, a road network v1 file (`sf_worldgen::roads`, guide
//!   frame), parse-checked, then copied byte for byte to `data/worldgen/roads/<mod id>.json` ([`ROADS_DIR`]), where
//!   the game reads every mod's road networks from the VFS and draws them with the guide's roads. Without it, a
//!   stale copy from an earlier run is removed.
//! - `--gardens <town-gardens.toml>` (optional, with `--guide <dir>`): trees and shrubs on the `features` of kind
//!   `"garden"` ([`crate::garden`]), appended to the same group after the instances, keys `garden:<lot>:<n>`,
//!   `GROUNDED`, height from the guide's ground. Without it (or with `--no-gardens`) `features` is not read and the
//!   mod is byte for byte what it was before gardens existed.
//!
//! **Temporary scaffold (owner 2026-11-24): the garden planting.** Isolated behind `--gardens` and
//! [`crate::garden`]; its removal path is Magna Urbis' own land-use planting (gardens, yards, orchards) or flora
//! garden zones, plus small kit structures on slivers (then the module, `bevyrrowind/worldgen/town-gardens.toml`
//! and the two flags go).
//!
//! The file's numbers are written in their shortest round-trip form (`{:?}`: positions as absolute engine metres
//! read back to the same `WorldPos`; the rotation is the flora one, which `WorldTransform`'s serde form normalizes
//! on read, up to an ulp); same inputs, same bytes. Loading it needs `sf.placement_group` registered with serde (`RecordDb::register_serde`).

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use glam::{DVec3, Quat, Vec3};
use serde::Deserialize;
use sf_coords::{WorldPos, WorldTransform};
use sf_records::RecordId;
use sf_vfs::VfsPath;
use sf_world::{GroupPlacement, PlacementFlags, PlacementGroup, RecordRef};

use crate::DumpError;
use crate::garden::{self, GardenCounts, GardenTable};
use crate::kit::{self, KitJson};
use mw_world::source::Base;

/// The `placements.json` format this module reads.
// tunables: const (format version of the contract)
pub const FORMAT: u32 = 1;
/// The record file inside the mod's asset root (VFS path).
pub const RECORDS_PATH: &str = "records/town.ron";
/// The mod's asset root, relative to its folder.
pub const ASSET_ROOT: &str = "data";
/// The VFS folder of a mod's road networks (road network v1 files, one per mod, `<mod id>.json`): the game reads
/// every `*.json` in it from the loaded mods (`mw_exterior_bevy::worldgen_roads`).
pub const ROADS_DIR: &str = "worldgen/roads";
/// The tag prefix naming a kit model's base record (`kit.rs`).
pub const BASE_TAG: &str = "base:";

/// `placements.json` (contract, format 1): the fields this reader uses; the rest are checked for presence only.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlacementsFile {
    pub format: u32,
    pub name: String,
    pub producer: String,
    pub recipe: String,
    pub seed: String,
    pub kit_blake3: String,
    pub origin_m: [f64; 2],
    pub lots: Vec<serde_json::Value>,
    pub instances: Vec<Instance>,
    pub streets: Vec<serde_json::Value>,
    pub features: Vec<serde_json::Value>,
    #[serde(default)]
    pub grade: Option<String>,
}

/// One placed kit model (contract).
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    pub id: String,
    pub kit: String,
    #[serde(default)]
    pub lot: Option<String>,
    /// Pivot, world metres (x east, y north, z up).
    pub pos: [f64; 3],
    /// Counter-clockwise about +z from above, degrees.
    pub yaw_deg: f64,
    pub role: String,
    #[serde(default)]
    pub door: Option<[f64; 2]>,
}

impl PlacementsFile {
    /// Parses `placements.json` bytes, checking the format.
    ///
    /// # Errors
    /// Not JSON of this shape, or another format.
    pub fn parse(bytes: &[u8]) -> Result<Self, DumpError> {
        let file: Self = serde_json::from_slice(bytes)?;
        if file.format != FORMAT {
            return Err(DumpError::Invalid(format!("placements format {} (expected {FORMAT})", file.format)));
        }
        Ok(file)
    }
}

/// What the conversion skipped and kept.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TownCounts {
    pub instances: usize,
    pub placed: usize,
    /// Kit id not in `kit.json`.
    pub unknown_kit: usize,
    /// Kit model without a `base:` tag.
    pub no_base: usize,
    /// `base:` record not in the content.
    pub unresolved: usize,
    /// Distinct models placed whose kit entry lists more than one base (the first is used).
    pub multi_base_models: usize,
    /// The skipped kit ids (sorted, distinct).
    pub skipped_kits: Vec<String>,
    /// The garden pass (`--gardens`); `None`: not run.
    pub gardens: Option<GardenCounts>,
}

/// The town as one placement group.
#[derive(Clone, Debug, PartialEq)]
pub struct Town {
    pub id: RecordId,
    pub group: PlacementGroup,
    pub counts: TownCounts,
}

/// The group's record id for town `name`.
pub fn group_id(name: &str) -> RecordId {
    RecordId::new(&format!("worldgen.town.{name}"))
}

/// A guide-frame pivot and counter-clockwise yaw (radians, about +z) as an engine transform, exactly as
/// `flora::placements` converts an item standing upright ([module docs](self)).
pub fn engine_transform(pos: [f64; 3], yaw: f32) -> WorldTransform {
    let up_guide = [0.0f32, 0.0, 1.0];
    let up = Vec3::new(up_guide[0], up_guide[2], -up_guide[1]);
    let rotation = Quat::from_rotation_arc(Vec3::Y, up) * Quat::from_rotation_y(yaw);
    WorldTransform { pos: WorldPos::from_f64(DVec3::new(pos[0], pos[2], -pos[1])), rotation, scale: Vec3::ONE }
}

/// The placements of `file` over `kit`, bases resolved by `resolve` (record id → base).
///
/// # Errors
/// Two instances share an id.
pub fn build(file: &PlacementsFile, kit: &KitJson, resolve: impl Fn(&str) -> Option<Base>) -> Result<Town, DumpError> {
    let id = group_id(&file.name);
    let mut counts = TownCounts { instances: file.instances.len(), ..TownCounts::default() };
    let mut keys = BTreeSet::new();
    let mut skipped = BTreeSet::new();
    let mut multi = BTreeSet::new();
    let mut placements = Vec::with_capacity(file.instances.len());
    for inst in &file.instances {
        if !keys.insert(inst.id.as_str()) {
            return Err(DumpError::Invalid(format!("placements: instance id {:?} repeats", inst.id)));
        }
        let Some(model) = kit.models.iter().find(|m| m.id == inst.kit) else {
            counts.unknown_kit += 1;
            skipped.insert(inst.kit.clone());
            continue;
        };
        let mut bases = model.tags.iter().filter_map(|t| t.strip_prefix(BASE_TAG));
        let Some(base_id) = bases.next() else {
            counts.no_base += 1;
            skipped.insert(inst.kit.clone());
            continue;
        };
        if bases.next().is_some() {
            multi.insert(model.id.as_str());
        }
        let Some(base) = resolve(base_id) else {
            counts.unresolved += 1;
            skipped.insert(inst.kit.clone());
            continue;
        };
        #[allow(clippy::cast_possible_truncation)] // An angle: f32 is the rotation's own precision.
        let yaw = inst.yaw_deg.to_radians() as f32;
        placements.push(GroupPlacement {
            key: inst.id.clone(),
            base: RecordRef::new(base.tag, base_id),
            model: base.model.as_deref().and_then(|m| VfsPath::new(m).ok()),
            transform: engine_transform(inst.pos, yaw),
            class: base.class,
            flags: PlacementFlags::empty(),
        });
    }
    counts.placed = placements.len();
    counts.multi_base_models = multi.len();
    let group = PlacementGroup { worldspace: mw_world::ids::exterior(), placements };
    counts.skipped_kits = skipped.into_iter().collect();
    Ok(Town { id, group, counts })
}

/// Appends the garden plants of `file`'s features to `town` ([`crate::garden::plant`]).
///
/// # Errors
/// A garden feature that does not parse, a garden key that repeats an instance id.
pub fn add_gardens(
    town: &mut Town,
    file: &PlacementsFile,
    table: &GardenTable,
    resolve: impl Fn(&str) -> Option<Base>,
    height: &dyn Fn(f64, f64) -> f64,
) -> Result<(), DumpError> {
    let (plants, counts) = garden::plant(table, &file.features, resolve, height)?;
    let keys: BTreeSet<&str> = town.group.placements.iter().map(|p| p.key.as_str()).collect();
    if let Some(p) = plants.iter().find(|p| keys.contains(p.key.as_str())) {
        return Err(DumpError::Invalid(format!("placements: garden key {:?} repeats an instance id", p.key)));
    }
    town.group.placements.extend(plants);
    town.counts.gardens = Some(counts);
    Ok(())
}

/// A RON string literal (`"` and `\` escaped; ids and paths are plain text).
fn ron_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        if matches!(c, '"' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// `, flags: "A | B"` (the bitflags serde text form), or nothing for no flags (the field's default).
fn flags_field(flags: PlacementFlags) -> String {
    if flags.is_empty() {
        return String::new();
    }
    let names: Vec<&str> = flags.iter_names().map(|(n, _)| n).collect();
    format!(", flags: {}", ron_str(&names.join(" | ")))
}

/// The record file: a header comment and one `define` of the group.
pub fn to_ron(town: &Town, file: &PlacementsFile) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "// Generated by `bevyrrowind worldgen-town` (S3a) from placements.json `{}` ({}, recipe {}, seed {}, kit \
         blake3 {}). Do not edit: re-run the command.",
        file.name, file.producer, file.recipe, file.seed, file.kit_blake3
    );
    let _ = writeln!(
        s,
        "// {} placements of {} instances; skipped: {} unknown kit id, {} kit model without base, {} unresolved base.",
        town.counts.placed, town.counts.instances, town.counts.unknown_kit, town.counts.no_base, town.counts.unresolved
    );
    if let Some(g) = &town.counts.gardens {
        let _ = write!(s, "// Temporary scaffold (owner 2026-11-24), garden plants:{}", garden::report_line(g).trim_start_matches("  gardens:"));
    }
    s.push_str("[\n");
    let _ = writeln!(
        s,
        "  define(kind: {}, id: {}, value: (",
        ron_str(PlacementGroup::KIND),
        ron_str(town.id.as_str())
    );
    let _ = writeln!(s, "    worldspace: {},", ron_str(town.group.worldspace.as_str()));
    s.push_str("    placements: [\n");
    for p in &town.group.placements {
        let pos = p.transform.pos.to_f64();
        let r = p.transform.rotation.to_array();
        let _ = write!(
            s,
            "      (key: {}, base: (kind: {}, id: {}), ",
            ron_str(&p.key),
            ron_str(&p.base.kind),
            ron_str(p.base.id.as_str())
        );
        if let Some(m) = &p.model {
            let _ = write!(s, "model: Some({}), ", ron_str(m.as_str()));
        }
        let _ = writeln!(
            s,
            "transform: (pos: [{:?}, {:?}, {:?}], rotation: ({:?}, {:?}, {:?}, {:?})), class: {:?}{}),",
            pos.x,
            pos.y,
            pos.z,
            r[0],
            r[1],
            r[2],
            r[3],
            p.class,
            flags_field(p.flags)
        );
    }
    s.push_str("    ],\n  )),\n]\n");
    s
}

/// `mod.toml` of the town mod.
pub fn manifest(mod_id: &str, file: &PlacementsFile) -> String {
    format!(
        "# Generated by `bevyrrowind worldgen-town` (S3a). Do not edit: re-run the command.\n\
         id = {id}\nname = {name}\nversion = \"0.1.0\"\n\
         description = {desc}\n\n[content]\nassets = [{root}]\nrecords = [{rec}]\n",
        id = ron_str(mod_id),
        name = ron_str(&format!("Worldgen town {}", file.name)),
        desc = ron_str(&format!(
            "The settlement tool's {} town ({}, recipe {}, seed {}) as one sf.placement_group.",
            file.name, file.producer, file.recipe, file.seed
        )),
        root = ron_str(ASSET_ROOT),
        rec = ron_str(RECORDS_PATH),
    )
}

/// Whether `id` is a valid mod id (`sf_mods::ModId`: lower-case ASCII letters, digits, `_`, `-`, `.`).
pub fn valid_mod_id(id: &str) -> bool {
    !id.is_empty() && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.'))
}

/// The command's inputs.
#[derive(Clone, Debug)]
pub struct TownArgs {
    /// The tool's `placements.json`.
    pub placements: PathBuf,
    /// The folder holding the `kit.json` the placements name.
    pub kit: PathBuf,
    /// The mod folder (created; its two files replaced).
    pub out: PathBuf,
    /// Mod id; `None`: the folder's name.
    pub id: Option<String>,
    /// The tool's `streets.json` (road network v1, guide frame); `None`: no streets.
    pub streets: Option<PathBuf>,
    /// Temporary scaffold: `town-gardens.toml` ([`crate::garden`]); `None`: no garden plants.
    pub gardens: Option<PathBuf>,
    /// The guide folder whose ground the garden plants stand on (needed with `gardens`).
    pub guide: Option<PathBuf>,
}

/// The mod's road network file for `mod_id` (relative to the mod folder).
pub fn streets_path(mod_id: &str) -> PathBuf {
    Path::new(ASSET_ROOT).join(ROADS_DIR).join(format!("{mod_id}.json"))
}

/// Reads and parse-checks a road network v1 file; returns its bytes and segment count.
///
/// # Errors
/// Unreadable, or not a valid road network v1 file.
pub fn read_streets(path: &Path) -> Result<(Vec<u8>, usize), DumpError> {
    let bytes = std::fs::read(path).map_err(|e| DumpError::Invalid(format!("{}: {e}", path.display())))?;
    let net = sf_worldgen::roads::RoadNetwork::from_reader(bytes.as_slice())
        .map_err(|e| DumpError::Invalid(format!("{}: not a road network v1 file: {e}", path.display())))?;
    Ok((bytes, net.segments.len()))
}

/// Writes the mod of `town` into `dir`; returns the record file's path.
///
/// # Errors
/// A write fails.
pub fn write(dir: &Path, mod_id: &str, town: &Town, file: &PlacementsFile) -> Result<PathBuf, DumpError> {
    let records = dir.join(ASSET_ROOT).join(RECORDS_PATH);
    std::fs::create_dir_all(records.parent().expect("a file path"))?;
    std::fs::write(dir.join("mod.toml"), manifest(mod_id, file))?;
    std::fs::write(&records, to_ron(town, file))?;
    Ok(records)
}

/// The command: reads, checks the kit hash, builds, writes; returns the report.
///
/// # Errors
/// Unreadable or invalid inputs, a kit hash mismatch, a bad mod id, a failed write.
pub fn run(args: &TownArgs, resolve: impl Fn(&str) -> Option<Base>) -> Result<String, DumpError> {
    let file = PlacementsFile::parse(&std::fs::read(&args.placements)?)?;
    let kit_bytes = std::fs::read(args.kit.join(kit::KIT_FILE))?;
    let hash = kit::hash(&kit_bytes);
    if hash != file.kit_blake3 {
        return Err(DumpError::Invalid(format!(
            "{}: kit blake3 {hash} is not the placements' kit_blake3 {}",
            args.kit.display(),
            file.kit_blake3
        )));
    }
    let kit = kit::read(&args.kit)?;
    let mod_id = match &args.id {
        Some(id) => id.clone(),
        None => args.out.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
    };
    if !valid_mod_id(&mod_id) {
        return Err(DumpError::Invalid(format!("mod id {mod_id:?}: lower-case letters, digits, '_', '-', '.' only")));
    }
    let streets = args.streets.as_deref().map(read_streets).transpose()?;
    let gardens = match (&args.gardens, &args.guide) {
        (None, _) => None,
        (Some(_), None) => return Err(DumpError::Invalid("--gardens needs --guide (the ground the plants stand on)".into())),
        (Some(t), Some(g)) => {
            let table = GardenTable::read(t)?;
            let tiles = sf_worldgen::tiles::GuideTiles::open(g, mw_world::ids::exterior())
                .map_err(|e| DumpError::Invalid(format!("{}: {e}", g.display())))?;
            Some((table, tiles))
        }
    };
    let mut town = build(&file, &kit, &resolve)?;
    if let Some((table, tiles)) = &gardens {
        add_gardens(&mut town, &file, table, &resolve, &|x, y| f64::from(tiles.height(x, -y, 0)))?;
    }
    let path = write(&args.out, &mod_id, &town, &file)?;
    let mut s = report(&town, &mod_id, &path);
    let roads = args.out.join(streets_path(&mod_id));
    if let Some((bytes, segments)) = streets {
        std::fs::create_dir_all(roads.parent().expect("a file path"))?;
        std::fs::write(&roads, bytes)?;
        let _ = writeln!(s, "  streets: {segments} segments: {}", roads.display());
    } else {
        match std::fs::remove_file(&roads) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e.into()),
            _ => {}
        }
    }
    Ok(s)
}

/// The summary lines.
pub fn report(town: &Town, mod_id: &str, records: &Path) -> String {
    let c = &town.counts;
    let mut s = format!(
        "worldgen-town: mod {mod_id}, group {} in {}: {} placements of {} instances; skipped {} unknown kit id, {} \
         kit model without base, {} unresolved base; {} models with several bases (first used)\n",
        town.id.as_str(),
        town.group.worldspace.as_str(),
        c.placed,
        c.instances,
        c.unknown_kit,
        c.no_base,
        c.unresolved,
        c.multi_base_models
    );
    for k in &c.skipped_kits {
        let _ = writeln!(s, "  skipped kit id: {k}");
    }
    if let Some(g) = &c.gardens {
        s.push_str(&garden::report_line(g));
    }
    let _ = writeln!(s, "  records: {}", records.display());
    s
}
