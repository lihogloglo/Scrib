//! R0 activation → physics (physics.md "Streaming").
//!
//! Statics: [`PhysicsHook`] wraps the placement spawners of the classes the
//! app picks; activation queues a `Fixed` collider, deactivation removes it at
//! once. Each model's shape is built once on the task pool (from the app's
//! [`PhysicsShapeSource`], sharing the controller's triangle mesh) and shared
//! by every placement. Inserts run under [`PhysicsSettings::insert_budget_ms`]
//! per frame, refreshing the query structure every
//! [`PhysicsSettings::insert_batch`] bodies: inserting a whole R0 at once
//! costs one 40 ms step (P1 spike).
//!
//! Terrain: the active worldspace's R0 cells are covered with heightfield
//! chunks sampled from `TerrainHeightsMap` on the task pool; a chunk whose
//! heights aren't resident yet is retried when the map changes.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;

use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, IVec2, Vec2};
use bevy_tasks::{AsyncComputeTaskPool, Task, block_on};
use sf_controller::StaticShape;
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyId, BodyShape, HeightShape, Layer, MeshShape, ShapeRef};
use sf_terrain_bevy::TerrainHeightsMap;
use sf_terrain_bevy::sf_terrain::{TerrainHeights, WorldRect};
use sf_world::{IndexedPlacement, Level, Partition, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::{ActivateCx, ActiveWorldspace, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, Streaming};

use crate::{Physics, PhysicsSettings, PhysicsStats};

/// Loads a model's `collision.cook` shape (the app's bake store). Called on
/// the task pool.
#[derive(Resource, Clone)]
pub struct PhysicsShapeSource(pub ShapeLoader);

/// Model path → its collision shape.
pub type ShapeLoader = Arc<dyn Fn(&str) -> Option<Arc<StaticShape>> + Send + Sync>;

/// Loads a model's `body.cook` shape, for the placement classes it names
/// (items). Without it, those classes use their `collision.cook` mesh.
#[derive(Resource, Clone)]
pub struct PhysicsBodySource {
    pub loader: BodyLoader,
    pub classes: Vec<PlacementClass>,
}

/// Model path → its body shape.
pub type BodyLoader = Arc<dyn Fn(&str) -> Option<Arc<BodyShape>> + Send + Sync>;

/// Shape cache key prefix of body shapes (mesh shapes use the bare model path).
const BODY: &str = "body:";

type Key = (WorldspaceId, PlacementId);
type Chunks = Vec<(IVec2, Option<Arc<HeightShape>>)>;

enum ShapeState {
    Loading(Task<Option<ShapeRef>>),
    Ready(Option<ShapeRef>),
}

/// Queued activations, placed bodies, the shape cache and terrain chunks.
#[derive(Resource, Default)]
pub struct PhysicsStreaming {
    pending: HashMap<Key, (String, WorldTransform)>,
    order: VecDeque<Key>,
    waiting: HashMap<String, Vec<Key>>,
    shapes: HashMap<String, ShapeState>,
    /// Scaled copies being built, by model + scale.
    scaling: HashMap<String, Task<()>>,
    placed: HashMap<Key, BodyId>,
    terrain: Terrain,
    /// Recent cost of one query refresh, seconds: reserved out of the budget.
    refresh_s: f64,
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
}

/// Wraps a placement spawner: R0 activation queues the placement's static
/// collider, deactivation removes it.
pub struct PhysicsHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for PhysicsHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let p = &placement.placement;
        let Some(model) = &p.model else { return };
        let key = (cx.worldspace.clone(), p.id);
        remove_placed(world, &key);
        let body = world.get_resource::<PhysicsBodySource>().is_some_and(|b| b.classes.contains(&p.class));
        let shape_key = if body { format!("{BODY}{}", model.as_str()) } else { model.as_str().to_owned() };
        if let Some(mut s) = world.get_resource_mut::<PhysicsStreaming>() {
            s.pending.insert(key.clone(), (shape_key, p.transform));
            s.order.push_back(key);
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        remove_placed(world, &(cx.worldspace.clone(), placement.placement.id));
    }
}

fn remove_placed(world: &mut World, key: &Key) {
    let Some(mut s) = world.get_resource_mut::<PhysicsStreaming>() else { return };
    s.pending.remove(key);
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

/// Wraps `classes`' spawners with [`PhysicsHook`] (call once the spawners are final).
pub fn hook_physics(spawners: &mut PlacementSpawners, classes: &[PlacementClass]) {
    for &class in classes {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(PhysicsHook(inner)));
        }
    }
}

/// Polls shape builds and inserts queued statics under the frame budget.
#[allow(clippy::needless_pass_by_value)]
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
    while started.elapsed().as_secs_f64() + s.refresh_s < budget {
        let Some(key) = s.order.pop_front() else { break };
        let Some((model, _)) = s.pending.get(&key) else { continue };
        match s.shapes.get(model) {
            Some(ShapeState::Ready(shape)) => {
                let shape = match shape.clone() {
                    Some(ShapeRef::Mesh(m)) => m,
                    // Items: fixed until P4 inserts them as sleeping dynamic bodies.
                    Some(body @ ShapeRef::Body(_)) => {
                        let (_, transform) = s.pending.remove(&key).expect("checked");
                        let id = physics.insert(&key.0, &BodyDesc::fixed(transform, body, Layer::PROP));
                        if !touched.contains(&key.0) {
                            touched.push(key.0.clone());
                        }
                        s.placed.insert(key, id);
                        stats.inserts += 1;
                        continue;
                    }
                    _ => {
                        s.pending.remove(&key);
                        continue;
                    }
                };
                // A scale without a cached copy: built on the task pool first.
                let scale = s.pending[&key].1.scale;
                if !shape.has_scale(scale) {
                    let tag = format!("{model}@{:?}", scale.to_array().map(f32::to_bits));
                    if !s.scaling.contains_key(&tag) {
                        let task = AsyncComputeTaskPool::get().spawn(async move { shape.prepare_scale(scale) });
                        s.scaling.insert(tag.clone(), task);
                    }
                    s.waiting.entry(tag).or_default().push(key);
                    continue;
                }
                let (_, transform) = s.pending.remove(&key).expect("checked");
                let desc = BodyDesc::fixed(transform, ShapeRef::Mesh(shape), Layer::STATIC);
                let id = physics.insert(&key.0, &desc);
                if !touched.contains(&key.0) {
                    touched.push(key.0.clone());
                }
                s.placed.insert(key, id);
                stats.inserts += 1;
                batch += 1;
                if batch >= settings.insert_batch {
                    batch = 0;
                    s.refresh_s = refresh(&mut physics, &touched, s.refresh_s);
                }
            }
            Some(ShapeState::Loading(_)) => {
                let model = model.clone();
                s.waiting.entry(model).or_default().push(key);
            }
            None => {
                let model = model.clone();
                let task = if let Some(path) = model.strip_prefix(BODY) {
                    bodies.as_ref().map(|b| {
                        let (load, m) = (b.loader.clone(), path.to_owned());
                        AsyncComputeTaskPool::get().spawn(async move { load(&m).map(ShapeRef::Body) })
                    })
                } else {
                    source.as_ref().map(|src| {
                        let (src, m) = (src.0.clone(), model.clone());
                        AsyncComputeTaskPool::get()
                            .spawn(async move { src(&m).map(|shape| ShapeRef::Mesh(MeshShape::from_static(&shape))) })
                    })
                };
                let built = task.map_or(ShapeState::Ready(None), ShapeState::Loading);
                s.shapes.insert(model.clone(), built);
                s.waiting.entry(model).or_default().push(key);
            }
        }
    }
    s.refresh_s = refresh(&mut physics, &touched, s.refresh_s);
    stats.insert_ms += started.elapsed().as_secs_f32() * 1e3;
}

/// Refreshes the query structure of `worlds`; returns the updated estimate of
/// one refresh (decays toward the latest cost, jumps up at once).
fn refresh(physics: &mut Physics, worlds: &[WorldspaceId], estimate: f64) -> f64 {
    if worlds.is_empty() {
        return estimate;
    }
    let started = Instant::now();
    for ws in worlds {
        if let Some(w) = physics.worlds.get_mut(ws) {
            w.refresh_queries();
        }
    }
    let cost = started.elapsed().as_secs_f64();
    cost.max(estimate * 0.9)
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
