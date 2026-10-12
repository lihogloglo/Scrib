//! Item bodies: the Morrowind rules of the framework's `body.cook` and
//! `items.check` steps (`superfargoth/docs/specs/physics.md`, "Bake products";
//! spec: `docs/specs/physics-adapter.md`, "Items").
//!
//! - [`MwBodyRules`]: a model's `RootCollisionNode` triangles, hulled per
//!   node, when it has one; else the framework fit of the render mesh.
//!   Unlike static collision, the `NC` marker is ignored (an item must be
//!   grabbable).
//! - [`MwItemsRules`]: which placements are items (`PlacementClass::Dynamic`
//!   plus carryable lights), carryable lights always held until grabbed,
//!   terrain in the exterior.
//! - [`item_mass`]: record weight → simulated mass.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use glam::Vec3;
use mw_nif::NodeKind;
use rayon::prelude::*;
use sf_bake::{BakeError, Baker, Manifest, PlanCx, ProductKey, RunCx, StepStats, Store};
use sf_physics::cook::{BodyShape, BodySource, FitSettings, PartShape};
use sf_physics::steps::{BODY_COOK, BodyCookStep, BodyRules, CheckSettings, Hold, ITEMS_CHECK, ItemChecks, ItemsCheckStep, ItemsRules};
use sf_records::{RecordDb, RecordId};
use sf_world::{Placement, PlacementClass, WorldspaceId};
use tes3::esp::{
    Alchemy, Apparatus, Armor, Book, Clothing, Ingredient, Light, LightFlags, Lockpick, MiscItem, Probe, RepairItem, Weapon,
};

/// Morrowind weights read as roughly pounds (physics-adapter.md "Mass"): a
/// visual calibration constant, not a rule.
pub const KG_PER_WEIGHT: f32 = 0.5;
/// Density for weightless items (gold, clutter), kg/m³.
pub const WEIGHTLESS_DENSITY: f32 = 400.0;
/// The framework's simulated mass clamp (`PhysicsParams::mass_range`).
pub const MASS_RANGE: (f32, f32) = (0.2, 60.0);

/// Simulated mass of an item: weight × [`KG_PER_WEIGHT`], or volume ×
/// [`WEIGHTLESS_DENSITY`] for weight 0, clamped to [`MASS_RANGE`]. Burden
/// and encumbrance keep the record weight. (Moves to `mw_physics` with P4.)
pub fn item_mass(weight: f32, volume: f32) -> f32 {
    let kg = if weight > 0.0 { weight * KG_PER_WEIGHT } else { volume * WEIGHTLESS_DENSITY };
    kg.clamp(MASS_RANGE.0, MASS_RANGE.1)
}

fn is_nif(subject: &str) -> bool {
    subject.rsplit('.').next().is_some_and(|ext| ext.eq_ignore_ascii_case("nif"))
}

/// Authored collision of an item model: one point set per mesh under a
/// `RootCollisionNode`, in model space. `None` without one (or when it has
/// no geometry), and for skinned models.
pub fn authored_parts(nif: &mw_nif::Model) -> Option<Vec<Vec<Vec3>>> {
    if nif.had_skin || !nif.nodes.iter().any(|n| n.kind == NodeKind::Collision) {
        return None;
    }
    let world = mw_nif::world_transforms(nif);
    let parts: Vec<Vec<Vec3>> = nif
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| {
            let mesh = nif.meshes.get(node.mesh?)?;
            if !mesh.collision || mesh.positions.is_empty() {
                return None;
            }
            let t = glam::Affine3A::from_cols_array(&world[i].to_cols_array());
            Some(mesh.positions.iter().map(|p| t.transform_point3(Vec3::from_array(*p))).collect())
        })
        .collect();
    (!parts.is_empty()).then_some(parts)
}

/// The Morrowind [`BodyRules`] (module docs).
#[derive(Clone, Copy, Debug, Default)]
pub struct MwBodyRules;

impl MwBodyRules {
    /// Bump when [`authored_parts`]' output changes.
    pub const VERSION: u32 = 1;
}

impl BodyRules for MwBodyRules {
    fn plan(&self, subject: &str, cx: &mut PlanCx) {
        if is_nif(subject) {
            cx.file(subject);
        }
    }

    fn authored(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Option<Vec<Vec<Vec3>>>, BakeError> {
        if !is_nif(subject) {
            return Ok(None);
        }
        let bytes = cx.read_file(subject)?;
        let nif = mw_nif::load(&bytes).map_err(|e| BakeError::step(&ProductKey::new(BODY_COOK, subject), e))?;
        Ok(authored_parts(&nif))
    }

    fn version(&self) -> u32 {
        Self::VERSION
    }
}

/// The Morrowind [`ItemsRules`] (module docs).
pub struct MwItemsRules {
    /// Base ids of lights with the *Can Carry* flag.
    carryable: HashSet<RecordId>,
    /// Digest of `carryable` (part of the key: a flag flip re-checks pages).
    digest: Vec<u8>,
}

impl MwItemsRules {
    pub const VERSION: u32 = 1;

    pub fn new(db: &RecordDb) -> Self {
        let mut ids: Vec<RecordId> =
            db.iter::<Light>().filter(|(_, l)| l.data.flags.contains(LightFlags::CAN_CARRY)).map(|(id, _)| id.clone()).collect();
        ids.sort();
        let joined = ids.iter().map(RecordId::as_str).collect::<Vec<_>>().join("\n");
        let digest = sf_bake::Digest::of(joined.as_bytes()).hex().into_bytes();
        Self { carryable: ids.into_iter().collect(), digest }
    }
}

impl ItemsRules for MwItemsRules {
    fn is_item(&self, p: &Placement) -> bool {
        p.class == PlacementClass::Dynamic || (p.class == PlacementClass::Light && self.carryable.contains(&p.base.id))
    }
    /// Carryable lights hang or stand where nothing would hold them in most
    /// of vanilla (physics-adapter.md "Items").
    fn always_hold(&self, p: &Placement) -> bool {
        p.class == PlacementClass::Light
    }
    fn has_terrain(&self, ws: &WorldspaceId) -> bool {
        *ws == crate::world::exterior()
    }
    fn plan(&self, _subject: &str, cx: &mut PlanCx) {
        cx.param("carryable", self.digest.clone());
    }
    fn version(&self) -> u32 {
        Self::VERSION
    }
}

/// Registers `body.cook` (in [`crate::model_baker`]).
pub fn register(baker: &mut Baker) {
    BodyCookStep::register(Arc::new(MwBodyRules), FitSettings::default(), baker);
}

/// Registers `items.check` (needs the records for carryable lights).
pub fn register_items(baker: &mut Baker, db: &RecordDb) {
    ItemsCheckStep::register(Arc::new(MwItemsRules::new(db)), CheckSettings::default(), baker);
}

/// One item model: its records' kinds and weights.
#[derive(Clone, Debug, Default)]
pub struct ItemModel {
    pub kinds: BTreeSet<&'static str>,
    pub weights: Vec<f32>,
}

/// Every model of an item record (MISC, WEAP, ARMO, CLOT, BOOK, INGR, ALCH,
/// APPA, LOCK, PROB, REPA, carryable LIGH).
pub fn item_models(db: &RecordDb) -> BTreeMap<String, ItemModel> {
    let mut out: BTreeMap<String, ItemModel> = BTreeMap::new();
    let mut add = |kind: &'static str, mesh: &str, weight: f32| {
        if let Some(path) = crate::model_path(mesh) {
            let m = out.entry(path).or_default();
            m.kinds.insert(kind);
            m.weights.push(weight);
        }
    };
    macro_rules! collect {
        ($($ty:ty => $kind:literal),* $(,)?) => {$(
            for (_, r) in db.iter::<$ty>() {
                add($kind, &r.mesh, r.data.weight);
            }
        )*};
    }
    collect!(
        MiscItem => "MISC", Weapon => "WEAP", Armor => "ARMO", Clothing => "CLOT", Book => "BOOK", Ingredient => "INGR",
        Alchemy => "ALCH", Apparatus => "APPA", Lockpick => "LOCK", Probe => "PROB", RepairItem => "REPA",
    );
    for (_, l) in db.iter::<Light>().filter(|(_, l)| l.data.flags.contains(LightFlags::CAN_CARRY)) {
        add("LIGH", &l.mesh, l.data.weight);
    }
    out
}

/// `body.cook` of every item model.
pub fn body_keys<'a>(models: impl IntoIterator<Item = &'a String>) -> Vec<ProductKey> {
    models.into_iter().map(|m| ProductKey::new(BODY_COOK, m.as_str())).collect()
}

/// `items.check` of every `world.page` in the manifest.
pub fn items_keys(manifest: &Manifest) -> Vec<ProductKey> {
    manifest
        .products
        .keys()
        .filter(|k| k.step == sf_world::bake::PAGE_STEP)
        .map(|k| ProductKey::new(ITEMS_CHECK, k.subject.as_str()))
        .collect()
}


/// Bakes `body.cook` for every item model the bake knows (`known`, sorted)
/// and reads the products back.
pub fn bake_bodies(baker: &Baker, db: &RecordDb, known: &[String]) -> BodyReport {
    let mut items = item_models(db);
    items.retain(|p, _| known.binary_search(p).is_ok());
    let t = std::time::Instant::now();
    let report = baker.bake(&body_keys(items.keys()));
    let wall = t.elapsed();
    let mut out = body_report(baker.store(), &baker.manifest(), &items);
    out.step = report.steps.get(BODY_COOK).map(|s| (*s, wall));
    let errors: BTreeMap<String, String> = report.failures().map(|(k, e)| (k.subject.clone(), e.to_string())).collect();
    for (path, e) in &mut out.failed {
        if let Some(why) = errors.get(path) {
            e.clone_from(why);
        }
    }
    out
}

/// Registers `items.check` and runs it over every `world.page` baked so far.
pub fn check_items(baker: &mut Baker, db: &RecordDb) -> ItemsReport {
    register_items(baker, db);
    let keys = items_keys(&baker.manifest());
    let t = std::time::Instant::now();
    let report = baker.bake(&keys);
    let wall = t.elapsed();
    let mut out = items_report(baker.store(), &baker.manifest(), &keys);
    out.step = report.steps.get(ITEMS_CHECK).map(|s| (*s, wall));
    out
}

/// What `body.cook` produced over the item models.
#[derive(Clone, Debug, Default)]
pub struct BodyReport {
    pub models: u32,
    pub by_source: BTreeMap<BodySource, u32>,
    /// Single-part kinds ("box", "capsule", ...), decompositions excluded.
    pub by_kind: BTreeMap<&'static str, u32>,
    /// Vertex counts of every hull part.
    pub hull_vertices: Vec<u32>,
    /// Piece counts of decompositions.
    pub pieces: Vec<u32>,
    /// Simulated mass over item records: min, max, records clamped low / high.
    pub mass: (f32, f32, u32, u32),
    pub records: u32,
    pub failed: Vec<(String, String)>,
    pub step: Option<(StepStats, Duration)>,
}

fn kind_name(p: &PartShape) -> &'static str {
    match p {
        PartShape::Hull(_) => "hull",
        PartShape::Box(_) => "box",
        PartShape::Capsule { .. } => "capsule",
        PartShape::Cylinder { .. } => "cylinder",
        PartShape::Sphere(_) => "sphere",
    }
}

/// Reads every item model's `body.cook` product.
pub fn body_report(store: &Store, manifest: &Manifest, models: &BTreeMap<String, ItemModel>) -> BodyReport {
    let rows: Vec<Result<BodyShape, String>> = models
        .par_iter()
        .map(|(path, _)| {
            let digest = manifest.get(BODY_COOK, path).ok_or("no body.cook product")?;
            let bytes = store.get(&digest).map_err(|e| e.to_string())?;
            BodyShape::from_bytes(&bytes).map_err(|e| e.to_string())
        })
        .collect();
    let mut out = BodyReport { mass: (f32::MAX, 0.0, 0, 0), ..BodyReport::default() };
    for ((path, model), row) in models.iter().zip(rows) {
        let body = match row {
            Ok(b) => b,
            Err(e) => {
                out.failed.push((path.clone(), e));
                continue;
            }
        };
        out.models += 1;
        *out.by_source.entry(body.source).or_default() += 1;
        if body.source == BodySource::Decomposition {
            out.pieces.push(u32::try_from(body.parts.len()).unwrap_or(u32::MAX));
        } else if let Some(p) = body.parts.first() {
            *out.by_kind.entry(kind_name(&p.shape)).or_default() += 1;
        }
        out.hull_vertices.extend(body.parts.iter().filter_map(|p| u32::try_from(p.hull_vertices()).ok()).filter(|&n| n > 0));
        for &w in &model.weights {
            let raw = if w > 0.0 { w * KG_PER_WEIGHT } else { body.volume * WEIGHTLESS_DENSITY };
            let kg = item_mass(w, body.volume);
            out.records += 1;
            out.mass.0 = out.mass.0.min(kg);
            out.mass.1 = out.mass.1.max(kg);
            out.mass.2 += u32::from(raw < MASS_RANGE.0);
            out.mass.3 += u32::from(raw > MASS_RANGE.1);
        }
    }
    out
}

fn stats(v: &[u32]) -> String {
    if v.is_empty() {
        return "none".into();
    }
    let sum: u64 = v.iter().map(|&x| u64::from(x)).sum();
    #[allow(clippy::cast_precision_loss)]
    let mean = sum as f64 / v.len() as f64;
    format!("{} (min {}, mean {mean:.1}, max {})", v.len(), v.iter().min().unwrap_or(&0), v.iter().max().unwrap_or(&0))
}

impl fmt::Display for BodyReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "body.cook: {} item models, {} failed", self.models, self.failed.len())?;
        let sources: Vec<String> = self.by_source.iter().map(|(s, n)| format!("{s:?} {n}")).collect();
        writeln!(f, "  sources: {}", sources.join(", "))?;
        let kinds: Vec<String> = self.by_kind.iter().map(|(k, n)| format!("{k} {n}")).collect();
        writeln!(f, "  single-part kinds: {}", kinds.join(", "))?;
        writeln!(f, "  hull parts: {}", stats(&self.hull_vertices))?;
        writeln!(f, "  decompositions: {} pieces", stats(&self.pieces))?;
        writeln!(
            f,
            "  mass over {} records: {:.2}..{:.1} kg ({} clamped up to {}, {} clamped down to {})",
            self.records, self.mass.0, self.mass.1, self.mass.2, MASS_RANGE.0, self.mass.3, MASS_RANGE.1
        )?;
        if let Some((s, wall)) = &self.step {
            writeln!(f, "  step: built {} hit {} failed {}, run {:.2} s summed, wall {wall:.2?}", s.built, s.hits, s.failed, s.run_time.as_secs_f64())?;
        }
        for (path, e) in self.failed.iter().take(5) {
            writeln!(f, "  failed: {path}: {e}")?;
        }
        Ok(())
    }
}

/// What `items.check` found over every page.
#[derive(Clone, Debug, Default)]
pub struct ItemsReport {
    pub pages: u32,
    pub items: u32,
    pub by_hold: BTreeMap<Hold, u32>,
    /// Items that started more than 1 cm inside something.
    pub overlaps: u32,
    /// Unsupported items per page (interiors: per cell), most first.
    pub unsupported: Vec<(String, u32)>,
    pub statics: u64,
    pub terrain_patches: u64,
    pub failed: Vec<(String, String)>,
    pub step: Option<(StepStats, Duration)>,
}

/// Reads every `items.check` product of `keys`.
pub fn items_report(store: &Store, manifest: &Manifest, keys: &[ProductKey]) -> ItemsReport {
    let mut out = ItemsReport::default();
    for key in keys {
        let checks = manifest
            .get(&key.step, &key.subject)
            .ok_or_else(|| "no product".to_owned())
            .and_then(|d| store.get(&d).map_err(|e| e.to_string()))
            .and_then(|b| ItemChecks::from_bytes(&b));
        let checks = match checks {
            Ok(c) => c,
            Err(e) => {
                out.failed.push((key.subject.clone(), e));
                continue;
            }
        };
        out.pages += 1;
        out.statics += u64::from(checks.statics);
        out.terrain_patches += u64::from(checks.terrain_patches);
        let mut unsupported = 0;
        for c in &checks.items {
            out.items += 1;
            *out.by_hold.entry(c.hold).or_default() += 1;
            out.overlaps += u32::from(c.overlap() > 0.01);
            unsupported += u32::from(c.hold == Hold::Unsupported);
        }
        if unsupported > 0 {
            out.unsupported.push((key.subject.clone(), unsupported));
        }
    }
    out.unsupported.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}

impl fmt::Display for ItemsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let holds: Vec<String> = self.by_hold.iter().map(|(h, n)| format!("{h:?} {n}")).collect();
        writeln!(
            f,
            "items.check: {} pages, {} items ({}), {} overlaps > 1 cm; {} statics + {} terrain patches tested, {} failed",
            self.pages,
            self.items,
            holds.join(", "),
            self.overlaps,
            self.statics,
            self.terrain_patches,
            self.failed.len()
        )?;
        let top: Vec<String> = self.unsupported.iter().take(8).map(|(p, n)| format!("{p} ({n})")).collect();
        writeln!(f, "  most unsupported: {}", top.join(", "))?;
        if let Some((s, wall)) = &self.step {
            writeln!(f, "  step: built {} hit {} failed {}, run {:.2} s summed, wall {wall:.2?}", s.built, s.hits, s.failed, s.run_time.as_secs_f64())?;
        }
        for (page, e) in self.failed.iter().take(5) {
            writeln!(f, "  failed: {page}: {e}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mw_nif::coords::Trs;

    use super::*;

    fn node(parent: Option<usize>, kind: NodeKind, mesh: Option<usize>) -> mw_nif::Node {
        mw_nif::Node { name: String::new(), parent, transform: Trs::IDENTITY, kind, visible: true, mesh }
    }

    fn cube(collision: bool) -> mw_nif::Mesh {
        let positions = (0u8..8).map(|i| [f32::from(i & 1), f32::from((i >> 1) & 1), f32::from((i >> 2) & 1)]).collect();
        mw_nif::Mesh { positions, indices: vec![0, 1, 2], collision, ..mw_nif::Mesh::default() }
    }

    #[test]
    fn collision_nodes_become_parts_and_nc_is_ignored() {
        let mut nif = mw_nif::Model {
            nodes: vec![
                node(None, NodeKind::Node, None),
                node(Some(0), NodeKind::Geometry, Some(0)),
                node(Some(0), NodeKind::Collision, None),
                node(Some(2), NodeKind::Geometry, Some(1)),
            ],
            meshes: vec![cube(false), cube(true)],
            ..mw_nif::Model::default()
        };
        nif.nodes[2].transform.translation = [0.0, 2.0, 0.0];
        nif.no_collision = true;
        let parts = authored_parts(&nif).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(parts[0].iter().all(|p| p.y >= 2.0 - 1e-6), "collision node transform applied");
        // Without a collision node, or skinned: the framework fit.
        nif.had_skin = true;
        assert!(authored_parts(&nif).is_none());
        nif.had_skin = false;
        nif.nodes.truncate(2);
        assert!(authored_parts(&nif).is_none());
    }

    #[test]
    fn mass_from_weight_or_volume_clamped() {
        assert!((item_mass(10.0, 0.0) - 5.0).abs() < 1e-6);
        assert!((item_mass(0.0, 0.001) - 0.4).abs() < 1e-6, "weightless: volume × 400");
        assert!((item_mass(0.1, 0.0) - MASS_RANGE.0).abs() < 1e-6);
        assert!((item_mass(500.0, 0.0) - MASS_RANGE.1).abs() < 1e-6);
    }
}
