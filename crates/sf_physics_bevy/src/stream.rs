//! R0 activation → physics (physics.md "Streaming").
//!
//! Statics: [`PhysicsHook`] wraps the placement spawners of the classes the
//! app picks; activation queues a `Fixed` collider, deactivation removes it at
//! once. Each model's shape is built once on the task pool (from the app's
//! [`PhysicsShapeSource`], sharing the controller's triangle mesh) and shared
//! by every placement. Inserts run under [`PhysicsSettings::insert_budget_ms`]
//! per frame, refreshing the query structure every
//! [`PhysicsSettings::insert_batch`] bodies and stopping while the predicted
//! cost of the next refresh still fits: inserting a whole R0 at once costs
//! one 40 ms step (P1 spike).
//!
//! Items ([`ItemContent::is_item`], P4): `body.cook` shapes from
//! [`PhysicsBodySource`], inserted as sleeping dynamic bodies (`Fixed` when
//! the adapter holds them until grabbed), `DEBRIS` or `PROP` by mass and
//! size. Their visuals spawn under an [`ItemPivot`], which follows the body
//! while it moves. Deactivation writes a moved item's rest pose into the
//! `PlacementOverlay` (`move_to`, plus a `physics` state blob), so the cell
//! loads it there next time; activation reads the overlay too, for cells
//! whose visuals stayed loaded.
//!
//! Terrain: the active worldspace's R0 cells are covered with heightfield
//! chunks sampled from `TerrainHeightsMap` on the task pool; a chunk whose
//! heights aren't resident yet is retried when the map changes.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;

use bevy_asset::{AssetServer, UntypedHandle};
use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, IVec2, Vec2, Vec3};
use bevy_tasks::{AsyncComputeTaskPool, Task, block_on};
use bevy_time::{Fixed, Time};
use bevy_transform::components::Transform;
use sf_controller::StaticShape;
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyId, BodyShape, HeightShape, Layer, MeshShape, ShapeRef, SurfaceId};
use sf_terrain_bevy::TerrainHeightsMap;
use sf_terrain_bevy::sf_terrain::{TerrainHeights, WorldRect};
use sf_world::{IndexedPlacement, Level, Location, Partition, Placement, PlacementId, WorldspaceId};
use sf_world_bevy::{ActivateCx, ActiveWorldspace, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, Streaming, WorldOverlay};

use crate::{Physics, PhysicsSettings, PhysicsStats};

/// Loads a model's `collision.cook` shape (the app's bake store). Called on
/// the task pool.
#[derive(Resource, Clone)]
pub struct PhysicsShapeSource(pub ShapeLoader);

/// Model path → its collision shape.
pub type ShapeLoader = Arc<dyn Fn(&str) -> Option<Arc<StaticShape>> + Send + Sync>;

/// Item bodies: `body.cook` shapes and the adapter's item rules. Without it,
/// every hooked placement is static collision.
#[derive(Resource, Clone)]
pub struct PhysicsBodySource {
    pub loader: BodyLoader,
    pub items: Arc<dyn ItemContent>,
    /// Model path → its `support.cook` mesh: the render collision of statics
    /// that items rest inside ([`ItemContent::is_support`]). `None`: every
    /// static keeps its `collision.cook` shape.
    pub supports: Option<ShapeLoader>,
}

/// Model path → its body shape.
pub type BodyLoader = Arc<dyn Fn(&str) -> Option<Arc<BodyShape>> + Send + Sync>;

/// The adapter's item rules (physics.md "Adapter hooks", `item_body`).
pub trait ItemContent: Send + Sync + 'static {
    /// Whether a placement is a loose item: a body from its `body.cook`
    /// shape instead of static collision.
    fn is_item(&self, placement: &Placement) -> bool;
    /// Simulated mass and hold decision of an item; `volume` is its shape's
    /// unit-scale volume, m³.
    fn item_body(&self, worldspace: &WorldspaceId, placement: &Placement, volume: f32) -> ItemBody;
    /// Whether a static placement simulates with its `support.cook` mesh
    /// instead of its `collision.cook` shape (`items.check` found items
    /// inside its collision: a coarse proxy around shelves).
    fn is_support(&self, _worldspace: &WorldspaceId, _placement: &Placement) -> bool {
        false
    }
    /// An item body's surface material (contact reports: impact and slide
    /// sounds). Static meshes carry theirs per triangle (`collision.cook`).
    fn surface(&self, _worldspace: &WorldspaceId, _placement: &Placement) -> SurfaceId {
        SurfaceId::DEFAULT
    }
}

/// How one item goes into the world.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemBody {
    /// Simulated mass, kg (`None`: shape volume × the world's density).
    pub mass: Option<f32>,
    /// Held until grabbed (`items.check` unsupported, or the adapter's rule):
    /// inserted `Fixed`.
    pub hold: bool,
}

/// The visual root of an item placement: its spawner's entities hang under
/// it, and it follows the item's body while the body moves. Its transform is
/// relative to `origin` (the stream cell's origin, as the spawn's parent).
#[derive(Component, Clone, Debug)]
pub struct ItemPivot {
    pub worldspace: WorldspaceId,
    pub id: PlacementId,
    pub origin: WorldPos,
}

/// Overlay state owner of item bodies (`PlacementOverlay::set_state`).
pub const STATE_OWNER: &str = "physics";
/// Version byte of the `physics` state blob: v1 = asleep flag, linear and
/// angular velocity (6 × f32 LE).
pub const STATE_VERSION: u8 = 1;

/// Shape cache key prefix of body shapes (mesh shapes use the bare model path).
const BODY: &str = "body:";
/// Shape cache key prefix of `support.cook` meshes.
const SUPPORT: &str = "support:";

type Key = (WorldspaceId, PlacementId);
type Chunks = Vec<(IVec2, Option<Arc<HeightShape>>)>;

enum ShapeState {
    Loading(Task<Option<ShapeRef>>),
    Ready(Option<ShapeRef>),
}

struct Pending {
    shape: String,
    placement: Placement,
    /// Bounding radius around the placement's origin, m (`IndexedPlacement::radius`).
    radius: f32,
    item: bool,
}

/// Queued activations, placed bodies, the shape cache and terrain chunks.
#[derive(Resource, Default)]
pub struct PhysicsStreaming {
    pending: HashMap<Key, Pending>,
    order: VecDeque<Key>,
    waiting: HashMap<String, Vec<Key>>,
    shapes: HashMap<String, ShapeState>,
    /// Scaled copies being built, by model + scale.
    scaling: HashMap<String, Task<()>>,
    placed: HashMap<Key, BodyId>,
    /// Items in the world: the pose they went in at (moved = differs).
    items: HashMap<Key, WorldTransform>,
    /// Their placements (coverage freezes write the overlay).
    item_placements: HashMap<Key, Placement>,
    /// Seconds since the last coverage check.
    coverage_s: f32,
    terrain: Terrain,
    /// Predicted cost of one query refresh after a full batch, seconds:
    /// reserved out of the budget.
    refresh_s: f64,
    /// Recent refresh cost per body inserted, seconds (decaying maximum).
    refresh_per_body_s: f64,
    /// Recent cost of one `PhysicsWorld::insert`, statics and items apart,
    /// seconds (decaying maxima): the next insert must fit the budget.
    insert_static_s: f64,
    insert_item_s: f64,
}

#[derive(Default)]
struct Terrain {
    worldspace: Option<WorldspaceId>,
    chunks: HashMap<IVec2, BodyId>,
    /// Wanted, heights not resident: retried when the heights change.
    missing: HashSet<IVec2>,
    task: Option<Task<Chunks>>,
    /// Built, waiting for frame budget.
    ready: Chunks,
    building: HashSet<IVec2>,
}

impl PhysicsStreaming {
    pub fn pending_count(&self) -> usize {
        self.pending.len() + self.terrain.building.len()
    }

    pub fn terrain_count(&self) -> usize {
        self.terrain.chunks.len()
    }

    pub fn placed_count(&self) -> usize {
        self.placed.len()
    }

    /// Item bodies in the worlds.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Awake items (diagnostics): placement, the pose it went in at, how far
    /// it is from there now, and its linear and angular speed.
    pub fn awake_items(&self, physics: &Physics) -> Vec<(PlacementId, WorldTransform, f32, f32, f32)> {
        let mut out = Vec::new();
        for (key, start) in &self.items {
            let (Some(&id), Some(w)) = (self.placed.get(key), physics.worlds.get(&key.0)) else { continue };
            if w.is_sleeping(id) != Some(false) {
                continue;
            }
            let (Some(pose), Some((v, a))) = (w.pose(id), w.velocity(id)) else { continue };
            if v == Vec3::ZERO && a == Vec3::ZERO && pose.pos == start.pos {
                continue;
            }
            out.push((key.1, *start, start.pos.delta_f32(pose.pos).length(), v.length(), a.length()));
        }
        out
    }

    /// The body of a placement, if it has one.
    pub fn body(&self, worldspace: &WorldspaceId, id: PlacementId) -> Option<BodyId> {
        self.placed.get(&(worldspace.clone(), id)).copied()
    }

    /// Whether static collision around `pos` is still streaming in: a
    /// non-item placement of `worldspace` whose bounds reach the horizontal
    /// square of half size `margin` around `pos` was activated but its body
    /// is not in the world yet (shape loading, or waiting for insert budget).
    ///
    /// The collision-ready gate (physics.md "Streaming"): a character must
    /// not simulate where this holds, or it falls through a floor that
    /// arrives a few frames later (inserts are time-sliced, R0 activation is
    /// not). Items wait the same way, per worldspace, before they go in.
    pub fn collision_loading(&self, worldspace: &WorldspaceId, pos: WorldPos, margin: f32) -> bool {
        self.pending.iter().any(|((ws, _), p)| {
            if p.item || ws != worldspace {
                return false;
            }
            let d = pos.delta_f32(p.placement.transform.pos);
            let reach = margin.max(0.0) + p.radius;
            d.x.abs() <= reach && d.z.abs() <= reach
        })
    }
}

/// Wraps a placement spawner: R0 activation queues the placement's body
/// (an item's, or with `statics` a static collider), deactivation removes
/// it.
pub struct PhysicsHook {
    pub inner: Arc<dyn PlacementSpawner>,
    /// Non-items get static collision (else only items get bodies).
    pub statics: bool,
}

fn is_item(world: &World, placement: &Placement) -> bool {
    world.get_resource::<PhysicsBodySource>().is_some_and(|b| b.items.is_item(placement))
}

impl PlacementSpawner for PhysicsHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let p = &placement.placement;
        if !is_item(world, p) {
            return self.inner.spawn(world, cx, placement);
        }
        // Items spawn under a pivot that carries the placement's translation
        // and rotation (scale stays on the visuals: LOD ranges read it).
        let origin = p.transform.pos.offset(-cx.local.translation);
        let pivot_local = Transform { translation: cx.local.translation, rotation: cx.local.rotation, scale: Vec3::ONE };
        let pivot = world
            .spawn((
                pivot_local,
                Visibility::Inherited,
                ChildOf(cx.parent),
                ItemPivot { worldspace: cx.worldspace.clone(), id: p.id, origin },
            ))
            .id();
        let inner = SpawnCx { parent: pivot, local: Transform::from_scale(cx.local.scale), ..*cx };
        1 + self.inner.spawn(world, &inner, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.inner.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.inner.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.inner.activate(world, cx, placement);
        let mut p = placement.placement.clone();
        let Some(model) = &p.model else { return };
        let item = is_item(world, &p);
        if !item && !self.statics {
            return;
        }
        let key = (cx.worldspace.clone(), p.id);
        remove_placed(world, &key);
        if item && let Some(overlay) = world.get_resource::<WorldOverlay>() {
            // Visuals that stayed loaded carry the transform from before a move.
            let overlay = overlay.0.read().expect("overlay lock");
            if overlay.is_removed(p.id) {
                return;
            }
            if let Some((ws, moved)) = overlay.placed(p.id)
                && ws == cx.worldspace
            {
                p.transform = moved.transform;
            }
        }
        let support = !item
            && world.get_resource::<PhysicsBodySource>().is_some_and(|b| b.supports.is_some() && b.items.is_support(cx.worldspace, &p));
        let shape = if item {
            format!("{BODY}{}", model.as_str())
        } else if support {
            format!("{SUPPORT}{}", model.as_str())
        } else {
            model.as_str().to_owned()
        };
        if let Some(mut s) = world.get_resource_mut::<PhysicsStreaming>() {
            s.pending.insert(key.clone(), Pending { shape, placement: p, radius: placement.radius, item });
            s.order.push_back(key);
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.inner.deactivate(world, cx, placement);
        let key = (cx.worldspace.clone(), placement.placement.id);
        persist_item(world, &key, &placement.placement);
        remove_placed(world, &key);
    }
}

/// Writes an item that moved since it went in to the overlay (physics.md
/// "Deactivation"): `move_to` its current pose and a `physics` state blob.
/// Saves serialize the overlay; until they exist this lasts the session.
fn persist_item(world: &mut World, key: &Key, placement: &Placement) {
    let Some(s) = world.get_resource::<PhysicsStreaming>() else { return };
    let (Some(&inserted), Some(&id)) = (s.items.get(key), s.placed.get(key)) else { return };
    let Some(w) = world.get_resource::<Physics>().and_then(|p| p.worlds.get(&key.0)) else { return };
    let Some(pose) = w.pose(id) else { return };
    let moved = inserted.pos.delta(pose.pos).length() > 1e-3 || inserted.rotation.angle_between(pose.rotation) > 1e-3;
    if !moved {
        return;
    }
    let asleep = w.is_sleeping(id).unwrap_or(true);
    let (v, a) = w.velocity(id).unwrap_or_default();
    let mut blob = vec![STATE_VERSION, u8::from(asleep)];
    for x in v.to_array().into_iter().chain(a.to_array()) {
        blob.extend_from_slice(&x.to_le_bytes());
    }
    let Some(overlay) = world.get_resource::<WorldOverlay>() else { return };
    let mut overlay = overlay.0.write().expect("overlay lock");
    overlay.move_to(placement, &Location { worldspace: key.0.clone(), pos: pose.pos }, pose.rotation);
    overlay.set_state(placement.id, STATE_OWNER, blob);
}

/// Coverage check period (physics.md "Coverage": 4 Hz) and the margin around
/// a body that must be `Active`, meters.
const COVERAGE_PERIOD_S: f32 = 0.25;
const COVERAGE_MARGIN: f32 = 1.0;

/// Coverage (physics.md "Streaming"): every 0.25 s each awake item checks
/// that the cells around it are `Active`. One that rolled or was thrown out of
/// R0 freezes: its pose goes to the overlay (as on deactivation), its body
/// leaves the world, and its visual stays where it stopped (R1 semantics).
/// Nothing simulates without ground around it. Worldspaces the scheduler
/// doesn't know (tests, synthetic worlds) are always covered.
pub(crate) fn coverage(world: &mut World) {
    let dt = world.get_resource::<Time>().map_or(0.0, Time::delta_secs);
    {
        let Some(mut s) = world.get_resource_mut::<PhysicsStreaming>() else { return };
        s.coverage_s += dt;
        if s.coverage_s < COVERAGE_PERIOD_S || s.items.is_empty() {
            return;
        }
        s.coverage_s = 0.0;
    }
    let (Some(s), Some(physics), Some(streaming)) =
        (world.get_resource::<PhysicsStreaming>(), world.get_resource::<Physics>(), world.get_resource::<Streaming>())
    else {
        return;
    };
    let scheduler = streaming.scheduler();
    let mut out: Vec<(Key, Placement)> = Vec::new();
    for (key, placement) in &s.item_placements {
        let (Some(&id), Some(w)) = (s.placed.get(key), physics.worlds.get(&key.0)) else { continue };
        if w.is_sleeping(id) != Some(false) || scheduler.directory(&key.0).is_none() {
            continue;
        }
        let Some(pose) = w.pose(id) else { continue };
        if !scheduler.covers(&key.0, pose.pos, COVERAGE_MARGIN, Level::Active) {
            out.push((key.clone(), placement.clone()));
        }
    }
    for (key, placement) in out {
        persist_item(world, &key, &placement);
        remove_placed(world, &key);
    }
}

fn remove_placed(world: &mut World, key: &Key) {
    let Some(mut s) = world.get_resource_mut::<PhysicsStreaming>() else { return };
    s.pending.remove(key);
    s.items.remove(key);
    s.item_placements.remove(key);
    let Some(id) = s.placed.remove(key) else { return };
    if let Some(mut physics) = world.get_resource_mut::<Physics>()
        && let Some(w) = physics.worlds.get_mut(&key.0)
    {
        w.remove(id);
    }
    if let Some(mut stats) = world.get_resource_mut::<PhysicsStats>() {
        stats.removals += 1;
    }
}

/// Wraps `classes`' spawners with [`PhysicsHook`]: static collision, or an
/// item body for items (call once the spawners are final).
pub fn hook_physics(spawners: &mut PlacementSpawners, classes: &[sf_world::PlacementClass]) {
    hook(spawners, classes, true);
}

/// Wraps `classes`' spawners so that only their items get bodies (e.g.
/// carryable lights among the lights).
pub fn hook_items(spawners: &mut PlacementSpawners, classes: &[sf_world::PlacementClass]) {
    hook(spawners, classes, false);
}

fn hook(spawners: &mut PlacementSpawners, classes: &[sf_world::PlacementClass], statics: bool) {
    for &class in classes {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(PhysicsHook { inner, statics }));
        }
    }
}

/// An item's layer by the framework defaults (physics.md "Layers"): `DEBRIS`
/// when light and small, else `PROP`.
fn item_layer(settings: &PhysicsSettings, mass: Option<f32>, shape: &BodyShape, scale: Vec3) -> Layer {
    let extent = ((Vec3::from_array(shape.aabb[1]) - Vec3::from_array(shape.aabb[0])) * scale.abs()).max_element();
    let mass = mass.unwrap_or(f32::MAX);
    if mass < settings.debris_mass && extent < settings.debris_extent { Layer::DEBRIS } else { Layer::PROP }
}

/// Cost assumed for one insert (and one body's share of a refresh) before any
/// was measured, seconds: the first loading frame must not overshoot (P1 spike:
/// 12.7 µs per body inserted at once).
const INSERT_PRIOR_S: f64 = 15e-6;

fn measured(estimate: f64) -> f64 {
    if estimate > 0.0 { estimate } else { INSERT_PRIOR_S }
}

/// Polls shape builds and inserts queued bodies under the frame budget.
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines, clippy::cast_precision_loss)]
pub(crate) fn apply_inserts(
    mut streaming: ResMut<PhysicsStreaming>,
    mut physics: ResMut<Physics>,
    source: Option<Res<PhysicsShapeSource>>,
    bodies: Option<Res<PhysicsBodySource>>,
    settings: Res<PhysicsSettings>,
    mut stats: ResMut<PhysicsStats>,
) {
    let s = &mut *streaming;
    // Finished shape builds put their placements back at the front of the queue.
    for (model, state) in &mut s.shapes {
        if let ShapeState::Loading(task) = state
            && task.is_finished()
        {
            *state = ShapeState::Ready(block_on(task));
            for key in s.waiting.remove(model).unwrap_or_default().into_iter().rev() {
                s.order.push_front(key);
            }
        }
    }
    // So do finished scaled copies.
    s.scaling.retain(|tag, task| {
        if !task.is_finished() {
            return true;
        }
        for key in s.waiting.remove(tag).unwrap_or_default().into_iter().rev() {
            s.order.push_front(key);
        }
        false
    });
    if s.order.is_empty() {
        return;
    }
    let started = Instant::now();
    let budget = f64::from(settings.insert_budget_ms) * 1e-3;
    let mut batch = 0;
    let mut touched: Vec<WorldspaceId> = Vec::new();
    // Statics first, time-sliced: stop while the refresh the inserted bodies
    // need still fits. Then, without a refresh (rapier must add them to its
    // islands in a step first), up to `item_batch` items; their broad-phase
    // work lands in the next step.
    let mut deferred: VecDeque<Key> = VecDeque::new();
    let mut held: Vec<Key> = Vec::new();
    let (mut items_phase, mut items_done, mut inserted) = (false, 0, 0_usize);
    loop {
        let elapsed = started.elapsed().as_secs_f64();
        let next = if items_phase {
            // Time-sliced like the statics: the next item must fit (its insert
            // cost is a decaying maximum). One item per frame always goes in
            // when nothing else did, so items can't starve.
            let fits = elapsed + measured(s.insert_item_s) < budget || inserted == 0;
            (fits && items_done < settings.item_batch).then(|| deferred.pop_front()).flatten()
        } else {
            // At least one static per frame: a refresh estimate over the budget
            // (one slow refresh on a busy machine) must not stall streaming.
            let fits = elapsed + measured(s.insert_static_s) + measured(s.refresh_per_body_s) * ((batch + 1) as f64) < budget;
            (fits || (batch == 0 && inserted == 0)).then(|| s.order.pop_front()).flatten()
        };
        let Some(key) = next else {
            if items_phase {
                break;
            }
            if batch > 0 {
                refresh(&mut physics, &touched, batch, s);
                batch = 0;
            }
            items_phase = true;
            // Items go in only once their worldspace has no static left to
            // insert: one woken before the counter or floor under it exists
            // falls through the world (the streamed level's collision is
            // complete before its dynamic bodies, as engines attach a cell).
            let loading: HashSet<&WorldspaceId> = s.pending.iter().filter(|(_, p)| !p.item).map(|(k, _)| &k.0).collect();
            let (wait, go): (VecDeque<Key>, VecDeque<Key>) = deferred.drain(..).partition(|k| loading.contains(&k.0));
            deferred = go;
            held.extend(wait);
            continue;
        };
        let Some(pending) = s.pending.get(&key) else { continue };
        match s.shapes.get(&pending.shape) {
            Some(ShapeState::Ready(shape)) => {
                let (desc, item) = match shape.clone() {
                    Some(ShapeRef::Body(_)) if pending.item && !items_phase => {
                        deferred.push_back(key);
                        continue;
                    }
                    Some(ShapeRef::Body(body)) if pending.item => {
                        items_done += 1;
                        let p = &pending.placement;
                        let Some(content) = bodies.as_ref().map(|b| b.items.clone()) else { continue };
                        let ItemBody { mass, hold } = content.item_body(&key.0, p, body.volume);
                        let layer = item_layer(&settings, mass, &body, p.transform.scale);
                        let shape = ShapeRef::Body(body);
                        let desc = if hold {
                            BodyDesc::held(p.transform, shape, layer, mass)
                        } else {
                            BodyDesc { ccd: true, ..BodyDesc::item(p.transform, shape, layer, mass) }
                        };
                        let desc = desc.with_surface(content.surface(&key.0, p));
                        (desc, true)
                    }
                    Some(ShapeRef::Mesh(m)) => {
                        // A scale without a cached copy: built on the task pool first.
                        let scale = pending.placement.transform.scale;
                        if !m.has_scale(scale) {
                            let tag = format!("{}@{:?}", pending.shape, scale.to_array().map(f32::to_bits));
                            if !s.scaling.contains_key(&tag) {
                                let task = AsyncComputeTaskPool::get().spawn(async move { m.prepare_scale(scale) });
                                s.scaling.insert(tag.clone(), task);
                            }
                            s.waiting.entry(tag).or_default().push(key);
                            continue;
                        }
                        (BodyDesc::fixed(pending.placement.transform, ShapeRef::Mesh(m), Layer::STATIC), false)
                    }
                    _ => {
                        s.pending.remove(&key);
                        continue;
                    }
                };
                let removed = s.pending.remove(&key);
                let t = Instant::now();
                let id = physics.insert(&key.0, &desc);
                let cost = t.elapsed().as_secs_f64();
                let estimate = if item { &mut s.insert_item_s } else { &mut s.insert_static_s };
                *estimate = cost.max(*estimate * 0.95);
                inserted += 1;
                if !touched.contains(&key.0) {
                    touched.push(key.0.clone());
                }
                if item {
                    s.items.insert(key.clone(), desc.pose);
                    if let Some(p) = removed {
                        s.item_placements.insert(key.clone(), p.placement);
                    }
                }
                s.placed.insert(key, id);
                stats.inserts += 1;
                batch += usize::from(!item);
                if batch >= settings.insert_batch {
                    refresh(&mut physics, &touched, batch, s);
                    batch = 0;
                }
            }
            Some(ShapeState::Loading(_)) => {
                let shape = pending.shape.clone();
                s.waiting.entry(shape).or_default().push(key);
            }
            None => {
                let shape = pending.shape.clone();
                let task = if let Some(path) = shape.strip_prefix(BODY) {
                    bodies.as_ref().map(|b| {
                        let (load, m) = (b.loader.clone(), path.to_owned());
                        AsyncComputeTaskPool::get().spawn(async move { load(&m).map(ShapeRef::Body) })
                    })
                } else if let Some(path) = shape.strip_prefix(SUPPORT) {
                    bodies.as_ref().and_then(|b| b.supports.clone()).map(|load| {
                        let m = path.to_owned();
                        AsyncComputeTaskPool::get()
                            .spawn(async move { load(&m).map(|shape| ShapeRef::Mesh(MeshShape::from_static(&shape))) })
                    })
                } else {
                    source.as_ref().map(|src| {
                        let (src, m) = (src.0.clone(), shape.clone());
                        AsyncComputeTaskPool::get()
                            .spawn(async move { src(&m).map(|shape| ShapeRef::Mesh(MeshShape::from_static(&shape))) })
                    })
                };
                let built = task.map_or(ShapeState::Ready(None), ShapeState::Loading);
                s.shapes.insert(shape.clone(), built);
                s.waiting.entry(shape).or_default().push(key);
            }
        }
    }
    for key in deferred.into_iter().rev().chain(held) {
        s.order.push_front(key);
    }
    stats.insert_ms += started.elapsed().as_secs_f32() * 1e3;
}

/// Refreshes the query structure of `worlds` after `bodies` inserts and
/// updates the cost estimates (a decaying maximum per body: jumps up at
/// once, decays 10 % per refresh).
fn refresh(physics: &mut Physics, worlds: &[WorldspaceId], bodies: usize, s: &mut PhysicsStreaming) {
    if worlds.is_empty() {
        return;
    }
    let started = Instant::now();
    for ws in worlds {
        if let Some(w) = physics.worlds.get_mut(ws) {
            w.refresh_queries();
        }
    }
    #[allow(clippy::cast_precision_loss)]
    let per_body = started.elapsed().as_secs_f64() / bodies.max(1) as f64;
    s.refresh_per_body_s = per_body.max(s.refresh_per_body_s * 0.9);
    s.refresh_s = s.refresh_per_body_s * 8.0;
}

/// Item visuals follow their awake bodies (interpolated); a pose is written
/// only when it changed, so resting items cost no transform propagation.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn follow_items(
    physics: Res<Physics>,
    streaming: Res<PhysicsStreaming>,
    time: Res<Time<Fixed>>,
    held: Res<crate::HeldPoses>,
    mut pivots: Query<(&ItemPivot, &mut Transform)>,
) {
    if streaming.items.is_empty() {
        return;
    }
    let alpha = time.overstep_fraction();
    for (pivot, mut t) in &mut pivots {
        let key = (pivot.worldspace.clone(), pivot.id);
        let (Some(id), Some(w)) = (streaming.placed.get(&key), physics.worlds.get(&pivot.worldspace)) else { continue };
        if !streaming.items.contains_key(&key) {
            continue;
        }
        let carried = held.0.get(id).filter(|_| physics.worlds.active() == Some(&pivot.worldspace));
        let Some(pose) = carried.copied().or_else(|| w.pose_at(*id, alpha)) else { continue };
        let translation = pivot.origin.delta_f32(pose.pos);
        if t.translation != translation || t.rotation != pose.rotation {
            t.translation = translation;
            t.rotation = pose.rotation;
        }
    }
}

/// Heightfield chunks over the active worldspace's R0 cells.
#[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation)]
pub(crate) fn terrain_chunks(
    mut streaming: ResMut<PhysicsStreaming>,
    mut physics: ResMut<Physics>,
    active: Res<ActiveWorldspace>,
    scheduler: Option<Res<Streaming>>,
    heights: Option<Res<TerrainHeightsMap>>,
    settings: Res<PhysicsSettings>,
    mut stats: ResMut<PhysicsStats>,
) {
    let s = &mut *streaming;
    let reserve = s.refresh_s;
    let t = &mut s.terrain;
    let heights_changed = heights.as_ref().is_some_and(DetectChanges::is_changed);
    // Leaving a worldspace drops its chunks.
    if t.worldspace != active.0 {
        if let Some(w) = t.worldspace.as_ref().and_then(|ws| physics.worlds.get_mut(ws)) {
            for (_, id) in t.chunks.drain() {
                w.remove(id);
            }
        }
        t.chunks.clear();
        t.missing.clear();
        t.worldspace.clone_from(&active.0);
    }
    let Some(ws) = active.0.clone() else { return };
    let field = heights.as_ref().and_then(|h| h.0.get(&ws)).filter(|_| settings.terrain);
    let chunk = f64::from(settings.terrain_chunk);
    let Some(scheduler) = scheduler.as_ref().map(|s| s.scheduler()) else { return };
    let mut wanted = HashSet::new();
    if field.is_some()
        && let Some(Partition::Grid { .. }) = scheduler.directory(&ws).map(|d| d.partition)
        && let Some(dir) = scheduler.directory(&ws)
    {
        let extent = f64::from(dir.partition.cell_extent().unwrap_or(128.0));
        for (cell, level) in scheduler.resident(&ws) {
            if level < Level::Active {
                continue;
            }
            let min = dir.partition.cell_origin(cell).to_f64();
            let (x0, z0) = ((min.x / chunk).floor() as i32, (min.z / chunk).floor() as i32);
            let (x1, z1) = (((min.x + extent) / chunk).ceil() as i32, ((min.z + extent) / chunk).ceil() as i32);
            for x in x0..x1 {
                for z in z0..z1 {
                    wanted.insert(IVec2::new(x, z));
                }
            }
        }
    }
    // Removals now.
    let w = physics.worlds.get_mut(&ws);
    if let Some(w) = w {
        t.chunks.retain(|c, id| {
            let keep = wanted.contains(c);
            if !keep {
                w.remove(*id);
                stats.removals += 1;
            }
            keep
        });
    }
    t.missing.retain(|c| wanted.contains(c));
    if heights_changed {
        t.missing.clear();
    }
    // Finished builds → bodies, in the frame budget the statics left.
    if let Some(task) = t.task.take_if(|task| task.is_finished()) {
        t.ready.extend(block_on(task));
    }
    if !t.ready.is_empty() {
        let started = Instant::now();
        let budget = f64::from((settings.insert_budget_ms - stats.insert_ms).max(0.05)) * 1e-3;
        while started.elapsed().as_secs_f64() + reserve < budget {
            let Some((c, shape)) = t.ready.pop() else { break };
            t.building.remove(&c);
            if !wanted.contains(&c) || t.chunks.contains_key(&c) {
                continue;
            }
            let Some(shape) = shape else {
                t.missing.insert(c);
                continue;
            };
            let center = DVec3::new((f64::from(c.x) + 0.5) * chunk, 0.0, (f64::from(c.y) + 0.5) * chunk);
            let pose = WorldTransform { pos: WorldPos::from_f64(center), ..WorldTransform::IDENTITY };
            let desc = BodyDesc::fixed(pose, ShapeRef::Heightfield(shape), Layer::TERRAIN);
            let id = physics.insert(&ws, &desc);
            t.chunks.insert(c, id);
            stats.inserts += 1;
        }
        if let Some(w) = physics.worlds.get_mut(&ws) {
            w.refresh_queries();
        }
        stats.insert_ms += started.elapsed().as_secs_f32() * 1e3;
    }
    // Start the next build: a batch of the nearest missing chunks.
    let (Some(field), None, true) = (field, t.task.as_ref(), t.ready.is_empty()) else { return };
    let mut todo: Vec<IVec2> =
        wanted.iter().filter(|c| !t.chunks.contains_key(c) && !t.missing.contains(c) && !t.building.contains(c)).copied().collect();
    if todo.is_empty() {
        return;
    }
    let focus = physicsfocus_chunk(&physics, &ws, chunk);
    todo.sort_by_key(|c| (*c - focus).length_squared());
    todo.truncate(16);
    t.building.extend(todo.iter().copied());
    let (field, spacing, size) = (field.clone(), settings.terrain_spacing, settings.terrain_chunk);
    t.task = Some(AsyncComputeTaskPool::get().spawn(async move { build_chunks(&field, &todo, size, spacing) }));
}

/// The chunk at the world's anchor (build order: nearest first).
#[allow(clippy::cast_possible_truncation)]
fn physicsfocus_chunk(physics: &Physics, ws: &WorldspaceId, chunk: f64) -> IVec2 {
    let p = physics.worlds.get(ws).map_or(DVec3::ZERO, |w| w.anchor().to_f64());
    IVec2::new((p.x / chunk).floor() as i32, (p.z / chunk).floor() as i32)
}

/// Heightfields for `chunks`; `None` where any sample isn't resident.
fn build_chunks(field: &TerrainHeights, chunks: &[IVec2], size: f32, spacing: f32) -> Chunks {
    let rect_size = Vec2::splat(size);
    chunks
        .iter()
        .map(|&c| {
            let min = WorldPos::from_f64(DVec3::new(f64::from(c.x) * f64::from(size), 0.0, f64::from(c.y) * f64::from(size)));
            let rect = WorldRect { min, size: rect_size };
            let (nx, nz) = rect.sample_count(spacing);
            let mut out = vec![0.0; nx * nz];
            let missing = field.sample_rect(rect, spacing, &mut out);
            let shape = (missing == 0 && nx == nz).then(|| HeightShape::new(nx, spacing, &out)).flatten();
            (c, shape)
        })
        .collect()
}
