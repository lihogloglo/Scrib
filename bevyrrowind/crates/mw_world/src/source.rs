//! `sf_world::WorldSource` for Morrowind: regions are cells, placements are
//! references.
//!
//! Spec: `docs/specs/world-adapter.md` ("Placements").

use std::collections::BTreeMap;

use glam::Vec3;
use sf_records::{RecordDb, RecordId};
use sf_vfs::VfsPath;
use sf_world::{
    Fingerprint, ModelInfo, Placement, PlacementClass, PlacementFlags, RecordRef, RegionId, WorldSource,
    WorldspaceId, placement_bounds,
};
use tes3::esp::{
    Activator, Alchemy, Apparatus, Armor, Book, Cell, Clothing, Container, Creature, Door, Ingredient, LeveledCreature,
    LeveledItem, Light, Lockpick, MiscItem, Npc, Probe, Reference, RepairItem, Static, Weapon,
};

use crate::convert::{UNIT_METERS_F64, grid_of, world_transform};
use crate::terrain::HeightSampler;
use crate::{EDITOR_MARKERS, ids, model_path};

/// Containers: placed like statics, but the content pack spawns them (inventory, scripts).
pub const CONTAINER: PlacementClass = PlacementClass::Custom(1);
/// Activators: placed like statics, but the content pack spawns them (scripts, activation).
pub const ACTIVATOR: PlacementClass = PlacementClass::Custom(2);

/// A reference's base record: its kind tag, placement class, and model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base {
    /// The `mw_records` kind tag (`STAT`, `DOOR`, ...).
    pub tag: &'static str,
    pub class: PlacementClass,
    pub model: Option<String>,
}

/// Resolves a base record id across every kind a reference can place.
/// Leveled lists resolve at spawn time, so they have no model here.
pub fn resolve_base(db: &RecordDb, id: &str) -> Option<Base> {
    let key = RecordId::new(id);
    macro_rules! meshed {
        ($($ty:ty => $tag:literal, $class:expr;)*) => {
            $(if let Some(record) = db.get::<$ty>(&key) {
                return Some(Base { tag: $tag, class: $class, model: model_path(&record.mesh) });
            })*
        };
    }
    meshed! {
        Static => "STAT", PlacementClass::Static;
        Door => "DOOR", PlacementClass::Door;
        Light => "LIGH", PlacementClass::Light;
        Container => "CONT", CONTAINER;
        Activator => "ACTI", ACTIVATOR;
        Npc => "NPC_", PlacementClass::Actor;
        Creature => "CREA", PlacementClass::Actor;
        MiscItem => "MISC", PlacementClass::Dynamic;
        Weapon => "WEAP", PlacementClass::Dynamic;
        Armor => "ARMO", PlacementClass::Dynamic;
        Clothing => "CLOT", PlacementClass::Dynamic;
        Book => "BOOK", PlacementClass::Dynamic;
        Alchemy => "ALCH", PlacementClass::Dynamic;
        Apparatus => "APPA", PlacementClass::Dynamic;
        Ingredient => "INGR", PlacementClass::Dynamic;
        Lockpick => "LOCK", PlacementClass::Dynamic;
        Probe => "PROB", PlacementClass::Dynamic;
        RepairItem => "REPA", PlacementClass::Dynamic;
    }
    if db.get::<LeveledCreature>(&key).is_some() {
        return Some(Base { tag: "LEVC", class: PlacementClass::Actor, model: None });
    }
    if db.get::<LeveledItem>(&key).is_some() {
        return Some(Base { tag: "LEVI", class: PlacementClass::Dynamic, model: None });
    }
    None
}

/// Model bounds lookup for grounding: model VFS path → engine-space
/// `(min, max)` of the model at unit scale.
pub type ModelBounds<'a> = &'a (dyn Fn(&str) -> Option<([f32; 3], [f32; 3])> + Sync);

/// How [`MwWorldSource::build`] decides `GROUNDED`.
#[derive(Clone, Copy)]
pub struct SourceOptions<'a> {
    /// With bounds, a placement is grounded when its world bounds straddle
    /// the terrain surface (within `ground_tolerance_m`). Without, when its
    /// pivot is within `pivot_tolerance_m` of the surface.
    pub model_bounds: Option<ModelBounds<'a>>,
    pub ground_tolerance_m: f32,
    pub pivot_tolerance_m: f32,
}

impl Default for SourceOptions<'_> {
    fn default() -> Self {
        Self { model_bounds: None, ground_tolerance_m: 0.5, pivot_tolerance_m: 1.0 }
    }
}

/// What a build saw.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceStats {
    pub worldspaces: usize,
    pub regions: usize,
    pub placements: usize,
    /// Placements per class (`Debug` form of the class).
    pub by_class: BTreeMap<String, usize>,
    pub grounded: usize,
    pub editor_only: usize,
    /// References whose base record doesn't exist: (cell id, base id).
    pub unknown_bases: Vec<(RecordId, String)>,
    /// Moved references (MVRF) delivered from their destination cell.
    pub moved: usize,
    /// ... whose destination grid has no `CELL` record.
    pub moved_to_missing_cell: usize,
    /// ... whose position isn't inside the destination grid square.
    pub moved_outside_destination: usize,
    /// Moved references in interiors (MVRF only applies to exteriors; ignored).
    pub moved_interior: usize,
    /// Exterior references (not moved) whose position lies outside their
    /// cell's grid square. Harmless: the index assigns by position.
    pub outside_own_cell: usize,
}

struct Region {
    id: RegionId,
    cell: RecordId,
    fingerprint: Fingerprint,
    placements: Vec<Placement>,
}

/// A snapshot of every Morrowind placement, by worldspace and region (one
/// region per cell record or exterior grid square). Owns its data, so it
/// can move to bake threads; rebuild it when records change.
pub struct MwWorldSource {
    worldspaces: BTreeMap<WorldspaceId, Vec<Region>>,
    stats: SourceStats,
}

/// One reference waiting for its region: sort key, owning cell, reference.
struct Pending<'a> {
    file: &'a str,
    refr_index: u32,
    reference: &'a Reference,
}

impl MwWorldSource {
    /// Resolves every reference of every cell.
    pub fn build(db: &RecordDb, options: &SourceOptions<'_>) -> Self {
        let mut stats = SourceStats::default();
        // (worldspace, region cell id) → references delivered from it.
        let mut regions: BTreeMap<(WorldspaceId, RecordId), Vec<Pending<'_>>> = BTreeMap::new();
        // Lowercased content file names by source: the reference key.
        let files: Vec<String> = db.sources().iter().map(|s| s.name.to_ascii_lowercase()).collect();
        for (cell_id, cell) in db.iter::<Cell>() {
            let ws = ids::worldspace_of(cell);
            let mut own = Vec::with_capacity(cell.references.len());
            for (&(source, refr_index), reference) in &cell.references {
                let pending = Pending { file: files[source as usize].as_str(), refr_index, reference };
                match reference.moved_cell {
                    Some(dest) if !cell.is_interior() => {
                        stats.moved += 1;
                        let region = mw_records::keys::exterior(dest);
                        stats.moved_to_missing_cell += usize::from(db.get::<Cell>(&region).is_none());
                        stats.moved_outside_destination += usize::from(grid_of(reference.translation) != dest);
                        regions.entry((ws.clone(), region)).or_default().push(pending);
                        continue;
                    }
                    Some(_) => stats.moved_interior += 1,
                    None if !cell.is_interior() && grid_of(reference.translation) != cell.data.grid => {
                        stats.outside_own_cell += 1;
                    }
                    None => {}
                }
                own.push(pending);
            }
            regions.entry((ws, cell_id.clone())).or_default().extend(own);
        }

        let mut heights = HeightSampler::new(db);
        let exterior = ids::exterior();
        let mut worldspaces: BTreeMap<WorldspaceId, Vec<Region>> = BTreeMap::new();
        let mut bytes = Vec::new();
        for ((ws, cell), mut pending) in regions {
            pending.sort_unstable_by(|a, b| a.file.cmp(b.file).then(a.refr_index.cmp(&b.refr_index)));
            let is_exterior = ws == exterior;
            let mut placements = Vec::with_capacity(pending.len());
            for p in &pending {
                let Some(mut placement) = placement(db, p.file, p.refr_index, p.reference) else {
                    stats.unknown_bases.push((cell.clone(), p.reference.id.clone()));
                    continue;
                };
                if is_exterior && grounded(&placement, p.reference, options, &mut heights) {
                    placement.flags |= PlacementFlags::GROUNDED;
                }
                *stats.by_class.entry(format!("{:?}", placement.class)).or_default() += 1;
                stats.grounded += usize::from(placement.flags.contains(PlacementFlags::GROUNDED));
                stats.editor_only += usize::from(placement.flags.contains(PlacementFlags::EDITOR_ONLY));
                placements.push(placement);
            }
            bytes.clear();
            for p in &placements {
                encode(p, &mut bytes);
            }
            stats.placements += placements.len();
            let region = Region { id: ids::region_id(&cell), cell, fingerprint: Fingerprint::of(&bytes), placements };
            worldspaces.entry(ws).or_default().push(region);
        }
        stats.worldspaces = worldspaces.len();
        stats.regions = worldspaces.values().map(Vec::len).sum();
        Self { worldspaces, stats }
    }

    pub fn stats(&self) -> &SourceStats {
        &self.stats
    }

    /// Worldspaces with at least one region (every cell's worldspace).
    pub fn worldspaces(&self) -> impl Iterator<Item = &WorldspaceId> {
        self.worldspaces.keys()
    }

    /// The cell record id behind a region.
    pub fn region_cell(&self, ws: &WorldspaceId, region: RegionId) -> Option<&RecordId> {
        self.worldspaces.get(ws)?.iter().find(|r| r.id == region).map(|r| &r.cell)
    }

    /// Every placement of a worldspace, in region order.
    pub fn all_placements(&self, ws: &WorldspaceId) -> impl Iterator<Item = &Placement> {
        self.worldspaces.get(ws).into_iter().flatten().flat_map(|r| &r.placements)
    }
}

impl WorldSource for MwWorldSource {
    fn regions(&self, ws: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        for r in self.worldspaces.get(ws).into_iter().flatten() {
            out(r.id, r.fingerprint);
        }
    }

    fn placements(&self, ws: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        for r in self.worldspaces.get(ws).into_iter().flatten().filter(|r| r.id == region) {
            r.placements.iter().cloned().for_each(&mut *out);
        }
    }
}

/// One reference as a placement, `None` when its base doesn't exist.
/// `file` is the content file that first placed it (its key).
pub fn placement(db: &RecordDb, file: &str, refr_index: u32, reference: &Reference) -> Option<Placement> {
    let base = resolve_base(db, &reference.id)?;
    let marker = EDITOR_MARKERS.iter().any(|m| reference.id.eq_ignore_ascii_case(m));
    let (class, flags) =
        if marker { (PlacementClass::Marker, PlacementFlags::EDITOR_ONLY) } else { (base.class, PlacementFlags::empty()) };
    Some(Placement {
        id: ids::placement_id(file, refr_index),
        base: RecordRef::new(base.tag, reference.id.as_str()),
        model: base.model.and_then(|m| VfsPath::new(&m).ok()),
        transform: world_transform(reference.translation, reference.rotation, reference.scale.unwrap_or(1.0)),
        class,
        flags,
    })
}

/// Whether an exterior placement sits on the terrain (module docs).
fn grounded(p: &Placement, reference: &Reference, options: &SourceOptions<'_>, heights: &mut HeightSampler<'_>) -> bool {
    let eligible = matches!(p.class, PlacementClass::Static | PlacementClass::Door | PlacementClass::Custom(_))
        || (p.class == PlacementClass::Light && p.model.is_some());
    if !eligible {
        return false;
    }
    let [x, y, z] = reference.translation;
    let Some(ground) = heights.height(x, y) else { return false };
    #[allow(clippy::cast_possible_truncation)]
    let to_m = |units: f32| (f64::from(units) * UNIT_METERS_F64) as f32;
    let pivot = to_m(z);
    let ground = to_m(ground);
    let bounds = options.model_bounds.zip(p.model.as_ref()).and_then(|(lookup, model)| lookup(model.as_str()));
    match bounds {
        Some((min, max)) => {
            let info = ModelInfo { min: Vec3::from_array(min), max: Vec3::from_array(max), bytes: 0, lod_distance: 0.0 };
            let (aabb, _) = placement_bounds(p, Some(&info));
            let tol = options.ground_tolerance_m;
            pivot + aabb.min.y <= ground + tol && pivot + aabb.max.y >= ground - tol
        }
        None => (pivot - ground).abs() <= options.pivot_tolerance_m,
    }
}

/// Canonical bytes of a placement, for region fingerprints.
fn encode(p: &Placement, out: &mut Vec<u8>) {
    out.extend_from_slice(&p.id.0.to_le_bytes());
    out.extend_from_slice(p.base.kind.as_bytes());
    out.push(0);
    out.extend_from_slice(p.base.id.as_str().as_bytes());
    out.push(0);
    if let Some(m) = &p.model {
        out.extend_from_slice(m.as_str().as_bytes());
    }
    out.push(0);
    out.extend_from_slice(&p.transform.pos.to_bytes());
    for v in p.transform.rotation.to_array().into_iter().chain(p.transform.scale.to_array()) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    let class: [u8; 3] = match p.class {
        PlacementClass::Static => [0, 0, 0],
        PlacementClass::Dynamic => [1, 0, 0],
        PlacementClass::Actor => [2, 0, 0],
        PlacementClass::Light => [3, 0, 0],
        PlacementClass::Door => [4, 0, 0],
        PlacementClass::Marker => [5, 0, 0],
        PlacementClass::Custom(c) => {
            let [a, b] = c.to_le_bytes();
            [6, a, b]
        }
    };
    out.extend_from_slice(&class);
    out.extend_from_slice(&p.flags.bits().to_le_bytes());
}
