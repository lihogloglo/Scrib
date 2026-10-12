//! The `body.cook` and `items.check` bake steps (physics.md "Bake products"),
//! on the conventions of `sf_model::steps` and `sf_controller::steps`.
//!
//! - `body.cook`, subject a model path: a [`BodyShape`] from the adapter's
//!   authored collision ([`BodyRules::authored`]) or a fit of the `mesh.cook`
//!   LOD0 (`mesh.rest` for models the plain cook skips). Mods can ship
//!   `bodies/<model path>.sfbody`, which wins.
//! - `items.check`, subject a `world.page` subject: a **settle test** of the
//!   page's items. Every item becomes an awake body over the static collision
//!   and terrain around it, the world runs [`CheckSettings::settle_time`],
//!   and an item that dropped more than [`CheckSettings::support_tolerance`]
//!   is unsupported: the runtime holds it fixed until grabbed. The same
//!   idea as an editor's "simulate and keep" settle pass (UE "Keep Simulation
//!   Changes", the Creation Kit's Havok Sim), run as a check instead of a fix.
//!   Items that rose or slid are reported as overlaps (they started inside
//!   something and depenetrated).

use std::collections::BTreeMap;
use std::sync::Arc;

use glam::{DVec3, Vec3};
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_controller::CollisionMesh;
use sf_controller::steps::COLLISION_COOK;
use sf_coords::{WorldPos, WorldTransform};
use sf_model::Cooked;
use sf_model::steps::{MeshCookStep, MeshRestStep};
use sf_terrain::bake::{TILE_STEP, tile_subject};
use sf_terrain::{ResidentTile, TerrainHeights, TileCoord, TileProduct};
use sf_world::bake::PAGE_STEP;
use sf_world::{IndexPage, Placement, PlacementClass, PlacementFlags, WorldspaceId};

use crate::cook::{BodyShape, FitSettings, fit_authored, fit_render};
use crate::{BodyDesc, BodyKind, HeightShape, Layer, MeshShape, PhysicsParams, PhysicsWorld, ShapeRef};

pub const BODY_COOK: &str = "body.cook";
pub const ITEMS_CHECK: &str = "items.check";

/// Version of the `items.check` product format.
pub const ITEMS_FORMAT: u32 = 1;
const ITEMS_MAGIC: [u8; 4] = *b"SFIC";

fn step_error(step: &str, subject: &str, e: impl std::fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), e)
}

/// Adapter hooks of `body.cook`. Must be deterministic.
pub trait BodyRules: Send + Sync {
    /// Static inputs of [`Self::authored`] (e.g. the model file).
    fn plan(&self, _subject: &str, _cx: &mut PlanCx) {}
    /// Authored collision as point sets, one hull each, in model space;
    /// `None` fits the render mesh.
    fn authored(&self, _subject: &str, _cx: &mut RunCx<'_>) -> Result<Option<Vec<Vec<Vec3>>>, BakeError> {
        Ok(None)
    }
    /// Bump when the rules' output changes (part of the key).
    fn version(&self) -> u32 {
        0
    }
}

/// Framework defaults: always fit the render mesh.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultBodyRules;

impl BodyRules for DefaultBodyRules {}

/// `body.cook`: a movable model's collision and mass properties.
pub struct BodyCookStep {
    pub rules: Arc<dyn BodyRules>,
    pub settings: FitSettings,
}

impl BodyCookStep {
    pub fn register(rules: Arc<dyn BodyRules>, settings: FitSettings, baker: &mut Baker) {
        baker.register(Self { rules, settings });
    }
}

/// LOD0 positions and triangles of a static cook (`None` otherwise).
type Tris = (Vec<Vec3>, Vec<[u32; 3]>);
type Placed = Option<(Arc<MeshShape>, Vec3, Vec3)>;

fn lod0(cooked: &Cooked) -> Result<Option<Tris>, String> {
    let Cooked::Static(model) = cooked else { return Ok(None) };
    let Some(lod) = model.lods.first() else { return Ok(None) };
    let (mut positions, mut triangles) = (Vec::new(), Vec::new());
    for sub in &lod.submeshes {
        let (vertices, indices) = sub.decode().map_err(|e| e.to_string())?;
        let base = u32::try_from(positions.len()).map_err(|e| e.to_string())?;
        positions.extend(vertices.iter().map(|v| Vec3::from_array(v.position)));
        triangles.extend(indices.as_chunks::<3>().0.iter().map(|t| t.map(|i| i + base)));
    }
    Ok(Some((positions, triangles)))
}

impl Step for BodyCookStep {
    fn id(&self) -> &'static str {
        BODY_COOK
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(MeshCookStep::ID, subject);
        self.rules.plan(subject, cx);
        cx.param("settings", postcard::to_stdvec(&self.settings).expect("in-memory serialization"));
        cx.param("rules", self.rules.version().to_le_bytes().to_vec());
        cx.param("format", crate::cook::BODY_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let fail = |e: &dyn std::fmt::Display| step_error(BODY_COOK, subject, e);
        if let Some(parts) = self.rules.authored(subject, cx)? {
            return fit_authored(&parts, &self.settings).map(|b| b.to_bytes()).map_err(|e| fail(&e));
        }
        let cooked = Cooked::from_bytes(&cx.product(MeshCookStep::ID, subject)?).map_err(|e| fail(&e))?;
        let mesh = if let Some(mesh) = lod0(&cooked).map_err(|e| fail(&e))? {
            mesh
        } else {
            let rest = Cooked::from_bytes(&cx.product(MeshRestStep::ID, subject)?).map_err(|e| fail(&e))?;
            lod0(&rest).map_err(|e| fail(&e))?.ok_or_else(|| fail(&"no static geometry (mesh.cook or mesh.rest)"))?
        };
        fit_render(&mesh.0, &mesh.1, &self.settings).map(|b| b.to_bytes()).map_err(|e| fail(&e))
    }
    fn override_path(&self, subject: &str) -> Option<String> {
        Some(format!("bodies/{subject}.sfbody"))
    }
}

/// Adapter hooks of `items.check`. Must be deterministic.
pub trait ItemsRules: Send + Sync {
    /// Whether a placement is a loose item (gets a body).
    fn is_item(&self, placement: &Placement) -> bool {
        placement.class == PlacementClass::Dynamic
    }
    /// Items held until grabbed whatever the settle test says.
    fn always_hold(&self, _placement: &Placement) -> bool {
        false
    }
    /// Whether a non-item placement is static collision.
    fn is_static(&self, placement: &Placement) -> bool {
        !matches!(placement.class, PlacementClass::Actor | PlacementClass::Marker | PlacementClass::Dynamic)
    }
    /// Whether the worldspace has terrain (`terrain.tile` products, level 0).
    fn has_terrain(&self, _ws: &WorldspaceId) -> bool {
        false
    }
    /// Inputs the rules read (records digests and the like).
    fn plan(&self, _subject: &str, _cx: &mut PlanCx) {}
    fn version(&self) -> u32 {
        0
    }
}

/// Settle-test settings.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSettings {
    /// Simulated seconds.
    pub settle_time: f32,
    /// A drop beyond this (meters) means unsupported.
    pub support_tolerance: f32,
    /// Static collision within this many meters of an item's bounds joins the test.
    pub reach: f32,
    /// Half size of the terrain patch under each item, meters (0.5 m samples).
    pub terrain_patch: f32,
}

impl Default for CheckSettings {
    fn default() -> Self {
        Self { settle_time: 0.5, support_tolerance: 0.05, reach: 1.0, terrain_patch: 2.0 }
    }
}

/// Why an item is held until grabbed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Hold {
    /// Rests where placed: simulated normally.
    No,
    /// Dropped in the settle test (hanging, floating).
    Unsupported,
    /// The adapter's rule ([`ItemsRules::always_hold`]).
    Rules,
    /// No `body.cook` product: kept as placed.
    NoShape,
}

/// One item's settle result. Displacements are meters over the settle time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemCheck {
    /// `PlacementId` bits.
    pub id: u128,
    pub hold: Hold,
    pub drop: f32,
    pub rise: f32,
    /// Horizontal displacement.
    pub shift: f32,
}

impl ItemCheck {
    /// Whether the item started inside something (it rose or slid).
    pub fn overlap(&self) -> f32 {
        self.rise.max(self.shift)
    }
}

/// The `items.check` product: every item of a page, in index order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemChecks {
    pub items: Vec<ItemCheck>,
    /// Statics and terrain patches the test inserted.
    pub statics: u32,
    pub terrain_patches: u32,
}

impl ItemChecks {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = ITEMS_MAGIC.to_vec();
        out.extend_from_slice(&ITEMS_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 8 || bytes[..4] != ITEMS_MAGIC {
            return Err("items check: bad magic".into());
        }
        let format = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
        if format != ITEMS_FORMAT {
            return Err(format!("items check: format {format}, expected {ITEMS_FORMAT}"));
        }
        postcard::from_bytes(&bytes[8..]).map_err(|e| e.to_string())
    }
}

/// `items.check`: the settle test of one index page (module docs).
pub struct ItemsCheckStep {
    pub rules: Arc<dyn ItemsRules>,
    pub settings: CheckSettings,
}

impl ItemsCheckStep {
    pub fn register(rules: Arc<dyn ItemsRules>, settings: CheckSettings, baker: &mut Baker) {
        baker.register(Self { rules, settings });
    }
}

/// The worldspace of a `world.page` subject (`"<ws>/<x>,<y>,<z>"`).
fn page_worldspace(subject: &str) -> Option<WorldspaceId> {
    subject.rsplit_once('/').map(|(ws, _)| WorldspaceId::new(ws))
}

/// Axis-aligned bounds of a model-space box under a placement transform, in
/// the frame of `anchor`.
fn placed_bounds(anchor: WorldPos, t: &WorldTransform, min: Vec3, max: Vec3) -> (Vec3, Vec3) {
    let origin = anchor.delta_f32(t.pos);
    let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for i in 0..8 {
        let c = Vec3::new(
            if i & 1 == 0 { min.x } else { max.x },
            if i & 2 == 0 { min.y } else { max.y },
            if i & 4 == 0 { min.z } else { max.z },
        );
        let p = origin + t.rotation * (c * t.scale);
        lo = lo.min(p);
        hi = hi.max(p);
    }
    (lo, hi)
}

/// One item of a settle test.
#[derive(Clone)]
pub struct SettleItem {
    pub id: u128,
    pub pose: WorldTransform,
    pub shape: Option<Arc<BodyShape>>,
    pub always_hold: bool,
}

/// Inserts `items` awake into `world` (which holds the statics and terrain
/// around them), steps it for [`CheckSettings::settle_time`] and reports
/// each item's displacement and hold decision, in order.
pub fn settle(world: &mut PhysicsWorld, items: &[SettleItem], settings: &CheckSettings) -> Vec<ItemCheck> {
    let bodies: Vec<_> = items
        .iter()
        .map(|it| {
            let shape = it.shape.clone()?;
            Some(world.insert(&BodyDesc::new(BodyKind::Dynamic { asleep: false }, it.pose, ShapeRef::Body(shape), Layer::PROP)))
        })
        .collect();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let steps = (settings.settle_time / world.params().dt).ceil() as usize;
    world.refresh_queries();
    for _ in 0..steps {
        world.step();
    }
    drop(world.drain_events());
    items
        .iter()
        .zip(bodies)
        .map(|(it, body)| {
            let Some(end) = body.and_then(|b| world.pose(b)) else {
                return ItemCheck { id: it.id, hold: Hold::NoShape, drop: 0.0, rise: 0.0, shift: 0.0 };
            };
            let d = it.pose.pos.delta_f32(end.pos);
            let (drop, rise, shift) = ((-d.y).max(0.0), d.y.max(0.0), Vec3::new(d.x, 0.0, d.z).length());
            let hold = if it.always_hold {
                Hold::Rules
            } else if drop > settings.support_tolerance {
                Hold::Unsupported
            } else {
                Hold::No
            };
            ItemCheck { id: it.id, hold, drop, rise, shift }
        })
        .collect()
}

fn overlaps(a: (Vec3, Vec3), b: (Vec3, Vec3)) -> bool {
    a.0.cmple(b.1).all() && b.0.cmple(a.1).all()
}

/// Level-0 terrain tiles, read through the bake context as needed.
struct TerrainReader {
    ws: WorldspaceId,
    heights: TerrainHeights,
    tried: std::collections::BTreeSet<(i32, i32)>,
}

impl TerrainReader {
    fn height(&mut self, p: WorldPos, cx: &mut RunCx<'_>) -> Option<f32> {
        let coord = TileCoord::containing(p, 0);
        if self.tried.insert((coord.x, coord.z))
            && let Ok(bytes) = cx.product(TILE_STEP, &tile_subject(&self.ws, coord))
            && let Ok(tile) = TileProduct::from_bytes(&bytes)
        {
            self.heights.insert(Arc::new(ResidentTile::new(tile.height_tile())));
        }
        self.heights.height(p)
    }
}

impl Step for ItemsCheckStep {
    fn id(&self) -> &'static str {
        ITEMS_CHECK
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(PAGE_STEP, subject);
        self.rules.plan(subject, cx);
        cx.param("settings", postcard::to_stdvec(&self.settings).expect("in-memory serialization"));
        cx.param("rules", self.rules.version().to_le_bytes().to_vec());
        cx.param("format", ITEMS_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let ws = page_worldspace(subject).ok_or_else(|| step_error(ITEMS_CHECK, subject, "not a page subject"))?;
        let page = IndexPage::from_bytes(&cx.product(PAGE_STEP, subject)?).map_err(|e| step_error(ITEMS_CHECK, subject, e))?;
        let placements: Vec<&Placement> = page.cells.iter().flat_map(|c| c.placements.iter().map(|p| &p.placement)).collect();
        let items: Vec<&Placement> = placements
            .iter()
            .copied()
            .filter(|p| p.model.is_some() && !p.flags.contains(PlacementFlags::EDITOR_ONLY) && self.rules.is_item(p))
            .collect();
        let mut out = ItemChecks::default();
        let Some(first) = items.first() else { return Ok(out.to_bytes()) };
        let anchor = first.transform.pos;

        // Item shapes (a failed cook keeps the item as placed).
        let mut shapes: BTreeMap<&str, Option<Arc<BodyShape>>> = BTreeMap::new();
        for item in &items {
            let model = item.model.as_ref().expect("filtered").as_str();
            if !shapes.contains_key(model) {
                let shape = cx.product(BODY_COOK, model).ok().and_then(|b| BodyShape::from_bytes(&b).ok()).map(Arc::new);
                shapes.insert(model, shape);
            }
        }
        let shape_of = |p: &Placement| shapes.get(p.model.as_ref().expect("filtered").as_str()).cloned().flatten();
        let reach = Vec3::splat(self.settings.reach);
        let zones: Vec<(Vec3, Vec3)> = items
            .iter()
            .filter_map(|p| {
                let s = shape_of(p)?;
                let (lo, hi) = placed_bounds(anchor, &p.transform, Vec3::from_array(s.aabb[0]), Vec3::from_array(s.aabb[1]));
                Some((lo - reach, hi + reach))
            })
            .collect();

        let mut world = PhysicsWorld::new(anchor, PhysicsParams::default());
        // Static collision near any item.
        let mut meshes: BTreeMap<&str, Placed> = BTreeMap::new();
        for p in placements.iter().filter(|p| !self.rules.is_item(p) && self.rules.is_static(p)) {
            let Some(model) = &p.model else { continue };
            let model = model.as_str();
            let entry = meshes.entry(model).or_insert_with(|| {
                let mesh = cx.product(COLLISION_COOK, model).ok().and_then(|b| CollisionMesh::from_bytes(&b).ok())?;
                let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
                for q in &mesh.positions {
                    lo = lo.min(Vec3::from_array(*q));
                    hi = hi.max(Vec3::from_array(*q));
                }
                Some((MeshShape::new(&mesh)?, lo, hi))
            });
            let Some((shape, lo, hi)) = entry else { continue };
            let bounds = placed_bounds(anchor, &p.transform, *lo, *hi);
            if zones.iter().any(|z| overlaps(*z, bounds)) {
                world.insert(&BodyDesc::fixed(p.transform, ShapeRef::Mesh(shape.clone()), Layer::STATIC));
                out.statics += 1;
            }
        }
        // Terrain patches under the items, on a shared 0.5 m grid.
        if self.rules.has_terrain(&ws) {
            let mut terrain = TerrainReader { ws: ws.clone(), heights: TerrainHeights::new(), tried: std::collections::BTreeSet::default() };
            let mut done = std::collections::BTreeSet::new();
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let n = (self.settings.terrain_patch * 4.0).round() as usize + 1;
            for item in &items {
                let abs = WorldPos::ORIGIN.delta(item.transform.pos);
                let snapped = DVec3::new((abs.x * 2.0).round() / 2.0, 0.0, (abs.z * 2.0).round() / 2.0);
                #[allow(clippy::cast_possible_truncation)]
                let key = ((snapped.x * 2.0) as i64, (snapped.z * 2.0) as i64);
                if !done.insert(key) {
                    continue;
                }
                #[allow(clippy::cast_possible_truncation)]
                let center = item.transform.pos.offset(DVec3::new(snapped.x - abs.x, 0.0, snapped.z - abs.z).as_vec3());
                let half = self.settings.terrain_patch;
                let mut heights = Vec::with_capacity(n * n);
                for z in 0..n {
                    for x in 0..n {
                        #[allow(clippy::cast_precision_loss)]
                        let at = center.offset(Vec3::new(x as f32 * 0.5 - half, 0.0, z as f32 * 0.5 - half));
                        heights.extend(terrain.height(at, cx));
                    }
                }
                #[allow(clippy::cast_possible_truncation)]
                let ground = center.offset(Vec3::new(0.0, -(abs.y as f32), 0.0));
                if let Some(shape) = HeightShape::new(n, 0.5, &heights) {
                    let pose = WorldTransform { pos: ground, ..WorldTransform::IDENTITY };
                    world.insert(&BodyDesc::fixed(pose, ShapeRef::Heightfield(shape), Layer::TERRAIN));
                    out.terrain_patches += 1;
                }
            }
        }
        // The items, awake, then the settle.
        let tests: Vec<SettleItem> = items
            .iter()
            .map(|p| SettleItem { id: p.id.0, pose: p.transform, shape: shape_of(p), always_hold: self.rules.always_hold(p) })
            .collect();
        out.items = settle(&mut world, &tests, &self.settings);
        Ok(out.to_bytes())
    }
}

