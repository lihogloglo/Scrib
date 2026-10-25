//! The engine side of streaming: drives `sf_world::Scheduler` each frame.
//!
//! - *fetch*: index page (decoded-page cache) plus overlay, on the IO task
//!   pool; the level step's placements come back ([`LevelContent::step`]).
//! - *assets*: cooked models through the `AssetServer` (`baked://`),
//!   refcounted per model, turned into [`ModelTemplate`]s once loaded.
//! - *spawn*, *upload* (reveal), *activate*, *deactivate*, *despawn*: main
//!   thread, one exclusive system, each item timed with `Instant` against
//!   the scheduler's budgets.
//!
//! Entities: per streaming cell one high-precision `Stationary` entity at
//! the cell's origin; under it one hidden group per level, revealed at once
//! when the level completes (no partial pop-in); under each group one batch
//! per spawn item, so despawning is budgeted batch by batch.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use bevy_asset::{AssetServer, Assets, Handle, LoadState, RecursiveDependencyLoadState, UntypedHandle};
use bevy_camera::visibility::{RenderLayers, Visibility};
use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_pbr::StandardMaterial;
use bevy_platform::time::Instant;
use bevy_tasks::futures_lite::future;
use bevy_tasks::{IoTaskPool, Task};
use bevy_transform::components::Transform;
use big_space::prelude::{CellCoord, FloatingOrigin, Stationary};
use sf_coords::WorldPos;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::{Counters, unit};
use sf_model::LodScreen;
use sf_model_bevy::{CookedModelAsset, baked_path};
use sf_vfs::VfsPath;
use sf_world::{
    Anchor, AnchorId, AnchorRole, CellDirectory, DoorLink, IndexPage, IndexReader, IndexedPlacement, Level,
    LevelContent, Location, PageCache, Partition, PlacementId, PlacementOverlay, Request, RequestId, RequestKind, RingConfig,
    Scheduler, SchedulerConfig, Stage, StageStep, TickInput, Worldspace, WorldspaceId,
};

use crate::coords::{GRID_SWITCH, from_grid, to_grid};
use crate::far::{FarProxies, ProxySpawnCx};
use crate::precombine::{CellPrecombine, Precombines, bucket_ranges};
use crate::spawn::{
    ActivateCx, LoadedModel, ModelMaterials, ModelTemplate, PlacementSpawners, SpawnCx, TemplateLod, model_aabb,
    size_culled_end, spawn_template_ranges,
};
use crate::worldspace::{WorldspaceRegistry, WorldspaceRoot};

/// Longest run of frames the origin-shift guard may hold the main-thread
/// stages (a slow approach to a cell border must not starve streaming).
pub const MAX_GUARDED_FRAMES: u32 = 6;

/// Frames the origin-shift guard keeps holding after a shift: the render world
/// re-collects every mesh instance the frame after (pipelined rendering).
pub const SHIFT_COOLDOWN_FRAMES: u32 = 2;

/// Settings of the streaming runtime.
#[derive(Clone, Debug)]
pub struct StreamingConfig {
    pub scheduler: SchedulerConfig,
    /// LOD switch distances (screen error), bake.md "Runtime distance".
    pub lod: LodScreen,
    /// Placements per spawn item, and activations per activate item.
    pub placements_per_item: usize,
    /// Bake step and file extension of cooked meshes (`baked://<step>/<model>.<ext>`).
    pub mesh_step: String,
    pub mesh_ext: String,
    /// Decoded index pages kept in memory.
    pub page_cache_bytes: u64,
    /// Origin-shift guard (seconds of travel, 0 = off): while a floating
    /// origin is this close to changing grid cell, the main-thread stages
    /// (spawn, reveal, despawn, (de)activation) wait. A shift rewrites every
    /// resident `GlobalTransform`, and Bevy then re-collects every mesh
    /// instance; streaming work in the same frame stacks on that spike
    /// (streaming.md, "Spikes"). Only outward travel counts, and the guard
    /// holds at most [`MAX_GUARDED_FRAMES`] frames in a row, including
    /// [`SHIFT_COOLDOWN_FRAMES`] after the shift.
    pub origin_shift_guard_s: f32,
    /// Meshes of cooked models loading at once (0 = no cap; never capped in
    /// burst mode). Bevy inserts a finished load's meshes all in one frame
    /// and the render world allocates and copies every new mesh in the frame
    /// it is extracted, with no per-frame limit (`allocate_and_free_meshes`):
    /// a cell's models finishing together uploaded 200–470 meshes in one
    /// frame (+2–3 ms of render thread). Loads start while the models in
    /// flight times the running mean of meshes per model stays under this
    /// budget (at least one model), so it bounds the meshes arriving per
    /// frame; the rest queue in request order (streaming.md, "Spikes").
    pub max_loading_meshes: usize,
    /// Spawn near levels from their cells' precombines when [`Precombines`]
    /// is installed (precombine.md): merged buckets instead of one entity per
    /// member placement, LOD level and part. Only with a far field (its near
    /// level holds every placement of the cell).
    pub precombine: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            scheduler: SchedulerConfig::default(),
            lod: LodScreen::default(),
            placements_per_item: 16,
            mesh_step: "mesh.cook".into(),
            mesh_ext: "sfmesh".into(),
            page_cache_bytes: PageCache::DEFAULT_CAP,
            origin_shift_guard_s: 0.05,
            max_loading_meshes: 64,
            precombine: true,
        }
    }
}

/// Where the world index comes from (bake products in a game).
#[derive(Resource, Clone)]
pub struct WorldIndexSource(pub Arc<dyn IndexReader>);

/// Gameplay changes to placements; applied by every cell fetch.
#[derive(Resource, Clone, Default)]
pub struct WorldOverlay(pub Arc<RwLock<PlacementOverlay>>);

/// Worldspace records, for per-worldspace ring overrides.
#[derive(Resource, Clone, Default)]
pub struct WorldspaceRecords(pub BTreeMap<WorldspaceId, Worldspace>);

/// Door links: drive door prefetch (and the fade scaffold's travel).
#[derive(Resource, Clone, Default)]
pub struct DoorLinks(pub Vec<DoorLink>);

/// The player's worldspace: the only one that simulates (M2), and the one
/// the world-origin uniform follows.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct ActiveWorldspace(pub Option<WorldspaceId>);

/// A streaming source. Put it on a high-precision entity (a camera, the
/// player): a direct child of a worldspace root with a `CellCoord`.
#[derive(Component, Clone, Copy, Debug)]
pub struct StreamingAnchor {
    pub role: AnchorRole,
    /// Multiplies its ring radii.
    pub radius_scale: f32,
}

impl StreamingAnchor {
    pub fn visual() -> Self {
        Self { role: AnchorRole::Visual, radius_scale: 1.0 }
    }

    pub fn gameplay() -> Self {
        Self { role: AnchorRole::Gameplay, radius_scale: 1.0 }
    }
}

/// A streaming cell's entity.
#[derive(Component, Clone, Debug)]
pub struct StreamCell {
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
}

/// A cell reached a higher level.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct CellLoaded {
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
    pub level: Level,
}

/// A cell dropped to a lower level (`Unloaded`: gone).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct CellUnloaded {
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
    pub level: Level,
}

/// The active worldspace changed.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct WorldspaceActivated {
    pub worldspace: Option<WorldspaceId>,
}

/// Running totals for overlays and tests.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StreamStats {
    pub entities_spawned: u64,
    pub entities_despawned: u64,
    pub placements_spawned: u64,
    pub uncooked: u64,
    /// Placements whose chain ends early at their size-based draw distance
    /// (industry-audit.md J1).
    pub size_culled: u64,
    /// Assets spawners asked the assets stage to load before reveal
    /// ([`PlacementSpawner::prepare`]), summed over loads.
    pub prepared: u64,
    /// Placements drawn by precombine buckets instead of their own entities.
    pub precombined: u64,
    /// Near loads whose precombine was dropped (a member moved, removed or
    /// missing): the cell spawned the old way.
    pub precombine_fallbacks: u64,
    pub fetch_errors: u64,
    pub resident_cells: usize,
    pub models_loaded: usize,
}


type FetchResult = Result<Arc<Vec<IndexedPlacement>>, String>;
type DirResult = Result<Option<CellDirectory>, String>;

/// Per-request engine state.
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)] // independent per-request stage flags, not a state machine
struct Work {
    placements: Arc<Vec<IndexedPlacement>>,
    models: Vec<VfsPath>,
    /// Whether `models` were acquired (the assets stage started).
    acquired: bool,
    /// A far-field proxy was acquired for this load (R2 with a far field).
    proxy: bool,
    proxy_spawned: bool,
    next: usize,
    group: Option<Entity>,
    despawn: Option<Vec<Entity>>,
    released: Vec<VfsPath>,
    /// Spawner-prepared assets ([`PlacementSpawner::prepare`]), started once
    /// the models resolved; held until the work ends.
    prepared: Option<Vec<UntypedHandle>>,
    /// The assets were ready last frame (one settle frame before spawning).
    assets_settled: bool,
    /// The cell's precombine (near loads with [`Precombines`]); the fetch
    /// completes once it loaded or failed.
    near: Option<Handle<CellPrecombine>>,
    /// Its members, once validated against the fetched placements: no model,
    /// no visuals of their own.
    members: Arc<HashSet<PlacementId>>,
    near_spawned: bool,
}

struct Group {
    entity: Entity,
    models: Vec<VfsPath>,
    /// Held while the level is resident (its buckets' meshes are labeled
    /// assets of this load).
    _precombine: Option<Handle<CellPrecombine>>,
}

struct CellRuntime {
    entity: Entity,
    groups: BTreeMap<Level, Group>,
    active: Option<Arc<Vec<IndexedPlacement>>>,
}

enum SlotState {
    Loading,
    Ready(LoadedModel),
}

struct ModelSlot {
    /// `None` while queued behind [`StreamingConfig::max_loading_meshes`].
    handle: Option<Handle<CookedModelAsset>>,
    users: u32,
    state: SlotState,
}

/// What the main-thread stages need to know about a worldspace (copied
/// from the scheduler, which is busy running the stage).
#[derive(Clone, Copy)]
struct WsInfo {
    partition: Partition,
    rings: RingConfig,
}

/// Engine-side state the stage items work on.
struct Runtime {
    config: StreamingConfig,
    worlds: HashMap<WorldspaceId, WsInfo>,
    work: BTreeMap<RequestId, Work>,
    cells: HashMap<(WorldspaceId, IVec3), CellRuntime>,
    models: HashMap<VfsPath, ModelSlot>,
    /// Models waiting for a load slot, in request order.
    load_queue: VecDeque<VfsPath>,
    /// Running mean of meshes per loaded cooked model (load pacing).
    meshes_per_model: f32,
    materials: HashMap<String, Handle<StandardMaterial>>,
    stats: StreamStats,
    /// The far-field hook, refreshed every frame from [`FarProxies`].
    far: FarProxies,
    /// Cells a reload is unloading this frame (their deactivations discard).
    reloading: HashSet<(WorldspaceId, IVec3)>,
    /// Models held while a reload runs, so the cells come back from memory
    /// instead of reloading their assets (dropped once streaming is idle).
    pinned: Vec<Handle<CookedModelAsset>>,
}

/// The streaming runtime (one per app).
#[derive(Resource)]
pub struct Streaming {
    scheduler: Scheduler,
    rt: Runtime,
    cache: Arc<Mutex<PageCache>>,
    fetches: HashMap<RequestId, Task<FetchResult>>,
    directories: HashMap<WorldspaceId, Task<DirResult>>,
    unavailable: BTreeSet<WorldspaceId>,
    clock: Instant,
    previous: HashMap<Entity, (WorldPos, f64, Vec3)>,
    /// A floating-origin anchor is about to change grid cell (this frame).
    shift_soon: bool,
    /// Consecutive frames the guard held the stages back.
    guarded: u32,
    /// The floating-origin anchor's grid cell last frame, and the frames of
    /// cool-down left after it changed.
    origin_cell: Option<CellCoord>,
    cooldown: u32,
    links_seen: bool,
    /// The active worldspace the scheduler was last told about (outer
    /// `None`: never told).
    #[allow(clippy::option_option)]
    active_seen: Option<Option<WorldspaceId>>,
}

impl Streaming {
    pub fn new(config: StreamingConfig) -> Self {
        Self {
            scheduler: Scheduler::new(config.scheduler),
            cache: Arc::new(Mutex::new(PageCache::new(config.page_cache_bytes))),
            rt: Runtime {
                config,
                worlds: HashMap::new(),
                work: BTreeMap::new(),
                cells: HashMap::new(),
                models: HashMap::new(),
                load_queue: VecDeque::new(),
                meshes_per_model: 8.0,
                materials: HashMap::new(),
                stats: StreamStats::default(),
                far: FarProxies::default(),
                reloading: HashSet::new(),
                pinned: Vec::new(),
            },
            fetches: HashMap::new(),
            directories: HashMap::new(),
            unavailable: BTreeSet::new(),
            clock: Instant::now(),
            previous: HashMap::new(),
            shift_soon: false,
            guarded: 0,
            origin_cell: None,
            cooldown: 0,
            links_seen: false,
            active_seen: None,
        }
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub fn config(&self) -> &StreamingConfig {
        &self.rt.config
    }

    /// Budgets ×20 until R0 and R1 are resident (behind a fade or a
    /// loading screen).
    pub fn begin_burst(&mut self) {
        self.scheduler.begin_burst();
    }

    pub fn in_burst(&self) -> bool {
        self.scheduler.in_burst()
    }

    /// Nothing pending: no request, no directory load.
    pub fn is_idle(&self) -> bool {
        self.scheduler.requests().next().is_none() && self.directories.is_empty() && !self.scheduler.reloading()
    }

    pub fn stats(&self) -> StreamStats {
        self.rt.stats
    }

    /// Seconds on the streaming clock.
    pub fn now(&self) -> f64 {
        self.clock.elapsed().as_secs_f64()
    }

    /// Whether a worldspace's directory is loaded.
    pub fn has_directory(&self, ws: &WorldspaceId) -> bool {
        self.scheduler.directory(ws).is_some()
    }

    /// Reloads every resident cell behind a fade (a save load: save.md
    /// "Loading", the full replace). Each cell unloads through the normal
    /// stages (spawner hooks run, with [`crate::ActivateCx::discard`] set)
    /// and streams back with the current [`WorldOverlay`], so taken items
    /// return, moved items go back to their saved pose and nothing is
    /// doubled. Models stay pinned meanwhile; burst mode runs until done.
    /// [`crate::StartTravel::reload`] calls it at the fade's cut.
    pub fn reload_resident(&mut self) {
        self.rt.pinned.extend(self.rt.models.values().filter_map(|s| s.handle.clone()));
        self.scheduler.reload_resident();
        self.scheduler.begin_burst();
    }

    /// The entity of a resident streaming cell.
    pub fn cell_entity(&self, ws: &WorldspaceId, cell: IVec3) -> Option<Entity> {
        self.rt.cells.get(&(ws.clone(), cell)).map(|c| c.entity)
    }
}

// ---------------------------------------------------------------------------

/// Query of anchors: (entity, anchor, cell, transform, parent root).
pub(crate) type AnchorQuery =
    QueryState<(Entity, &'static StreamingAnchor, &'static CellCoord, &'static Transform, &'static ChildOf, Has<FloatingOrigin>)>;

/// The streaming driver (exclusive: spawn costs are measured for real).
pub(crate) fn drive_streaming(world: &mut World, anchors: &mut AnchorQuery, roots: &mut QueryState<&'static WorldspaceRoot>) {
    let started = Instant::now();
    let Some(mut streaming) = world.remove_resource::<Streaming>() else { return };
    let counters = world.get_resource::<DiagCounters>().map(|c| c.0.clone());
    let spawners = world.get_resource::<PlacementSpawners>().cloned().unwrap_or_default();
    streaming.drive(world, anchors, roots, &spawners, counters.as_deref());
    if let Some(c) = &counters {
        c.add_duration_named("stream/drive/ms", started.elapsed());
    }
    world.insert_resource(streaming);
}

impl Streaming {
    fn drive(
        &mut self,
        world: &mut World,
        anchor_q: &mut AnchorQuery,
        roots: &mut QueryState<&'static WorldspaceRoot>,
        spawners: &PlacementSpawners,
        counters: Option<&Counters>,
    ) {
        let now = self.now();
        self.rt.far = world.get_resource::<FarProxies>().cloned().unwrap_or_default();
        self.sync_settings(world);
        self.poll_directories(world);

        let anchors = self.anchors(world, anchor_q, roots, now);
        let t = Instant::now();
        let report = self.scheduler.update(&TickInput { now, anchors: &anchors });
        if let Some(c) = counters {
            c.add_duration_named("stream/tick/ms", t.elapsed());
        }

        for ws in &report.missing_worldspaces {
            self.request_directory(world, ws);
        }
        for cancelled in &report.cancelled {
            self.fetches.remove(&cancelled.id);
            self.rt.cancel(world, cancelled);
        }
        for id in &report.start {
            let Some(request) = self.scheduler.request(*id).cloned() else { continue };
            match request.stage {
                Stage::Fetch => self.start_fetch(world, &request),
                Stage::Assets => self.rt.start_assets(world, &request),
                _ => {}
            }
        }

        self.poll_fetches(world, now, spawners, counters);
        self.rt.poll_models(world, self.scheduler.in_burst());
        let world_ref: &World = world;
        let running: Vec<RequestId> =
            self.scheduler.requests().filter(|r| r.stage == Stage::Assets && r.running).map(|r| r.id).collect();
        let mut loaded = Vec::new();
        for id in running {
            let Some(r) = self.scheduler.request(id) else { continue };
            let ready = self.rt.work.get(&id).is_none_or(|w| {
                w.models.iter().all(|m| self.rt.model_ready(m))
                    && (!w.proxy || self.rt.far.proxies.as_ref().is_none_or(|p| p.ready(world_ref, &r.worldspace, r.cell)))
            });
            let spawning = r.kind == RequestKind::Load && r.to != Level::Active;
            if ready && (!spawning || self.rt.prepared(world_ref, id, spawners)) {
                // One frame between the last model arriving (its meshes upload
                // in that frame's render) and the spawn stage (new instances
                // and `VisibilityRange`s extract in the next), so the two
                // render-thread costs don't stack (streaming.md, "Spikes").
                let settle = spawning && !self.scheduler.in_burst();
                match self.rt.work.get_mut(&id) {
                    Some(w) if settle && !w.assets_settled => w.assets_settled = true,
                    _ => loaded.push(id),
                }
            }
        }
        for id in loaded {
            self.scheduler.assets_loaded(id, now);
        }

        // Origin-shift guard: the main-thread stages sit out the frames around
        // a floating-origin cell change (never in burst mode, behind a fade).
        if self.shift_soon && self.guarded < MAX_GUARDED_FRAMES && !self.scheduler.in_burst() {
            self.guarded += 1;
            if let Some(c) = counters {
                c.add_named("stream/shift_guard/frames", unit::COUNT, 1.0);
            }
        } else {
            self.guarded = 0;
            self.rt.reloading.clear();
            if self.scheduler.reloading() {
                let scheduler = &self.scheduler;
                self.rt.reloading.extend(
                    scheduler
                        .requests()
                        .filter(|r| r.kind == RequestKind::Unload && scheduler.is_reloading(&r.worldspace, r.cell))
                        .map(|r| (r.worldspace.clone(), r.cell)),
                );
            } else if !self.rt.pinned.is_empty() && self.is_idle() {
                self.rt.pinned.clear();
            }
            for stage in [Stage::Despawn, Stage::Deactivate, Stage::Spawn, Stage::Upload, Stage::Activate] {
                self.run_stage(world, stage, spawners, counters);
            }
        }

        self.rt.stats.resident_cells = self.rt.cells.len();
        self.rt.stats.models_loaded = self.rt.models.len();
        if let Some(c) = counters {
            self.report_counters(c, now);
        }
    }

    /// The anchors this frame, with velocities from position deltas.
    fn anchors(
        &mut self,
        world: &World,
        anchor_q: &mut AnchorQuery,
        roots: &mut QueryState<&'static WorldspaceRoot>,
        now: f64,
    ) -> Vec<Anchor> {
        let mut anchors = Vec::new();
        let mut seen = HashMap::new();
        let guard_s = self.rt.config.origin_shift_guard_s;
        let max_local = sf_coords::GRID_CELL / 2.0 + GRID_SWITCH;
        self.shift_soon = false;
        self.cooldown = self.cooldown.saturating_sub(1);
        for (entity, anchor, cell, transform, parent, is_origin) in anchor_q.iter(world) {
            let Ok(root) = roots.get(world, parent.parent()) else { continue };
            let pos = from_grid(cell, transform);
            let velocity = match self.previous.get(&entity) {
                Some((prev, t, v)) if now > *t => {
                    #[allow(clippy::cast_possible_truncation)]
                    let raw = (prev.delta(pos) / (now - t)).as_vec3();
                    // Light smoothing: one slow frame shouldn't spike the lookahead.
                    v.lerp(raw, 0.5)
                }
                Some((_, _, v)) => *v,
                None => Vec3::ZERO,
            };
            seen.insert(entity, (pos, now, velocity));
            // big_space recenters past `GRID_CELL / 2 + GRID_SWITCH` on any axis.
            // Only travel outward counts, so an anchor resting near a border
            // never holds streaming back.
            if is_origin && guard_s > 0.0 {
                // A shift happened last frame: the render world is still
                // re-collecting every instance, so wait a little longer.
                if self.origin_cell.is_some_and(|c| c != *cell) {
                    self.cooldown = SHIFT_COOLDOWN_FRAMES;
                }
                self.origin_cell = Some(*cell);
                let t = transform.translation;
                let outward = (velocity * t.signum()).max(Vec3::ZERO);
                self.shift_soon |= (t.abs() + outward * guard_s).cmpgt(Vec3::splat(max_local)).any();
                self.shift_soon |= self.cooldown > 0;
            }
            let mut a = Anchor::new(AnchorId(entity.index_u32()), anchor.role, Location::new(root.id.clone(), pos));
            a.velocity = velocity;
            a.view_dir = Some(transform.rotation * Vec3::NEG_Z);
            a.radius_scale = anchor.radius_scale;
            anchors.push(a);
        }
        self.previous = seen;
        anchors
    }

    fn sync_settings(&mut self, world: &mut World) {
        let links_changed = world.get_resource_ref::<DoorLinks>().is_some_and(|l| l.is_changed() || !self.links_seen);
        if links_changed && let Some(links) = world.get_resource::<DoorLinks>() {
            self.scheduler.set_links(links.0.clone());
            self.links_seen = true;
        }
        let active = world.get_resource::<ActiveWorldspace>().map(|a| a.0.clone()).unwrap_or_default();
        if self.active_seen.as_ref() != Some(&active) {
            self.scheduler.set_active_worldspace(active.clone());
            self.active_seen = Some(active.clone());
            if let Some(mut messages) = world.get_resource_mut::<Messages<WorldspaceActivated>>() {
                messages.write(WorldspaceActivated { worldspace: active });
            }
        }
    }

    // -- directories ---------------------------------------------------------

    fn request_directory(&mut self, world: &World, ws: &WorldspaceId) {
        if self.directories.contains_key(ws) || self.unavailable.contains(ws) || self.has_directory(ws) {
            return;
        }
        let Some(reader) = world.get_resource::<WorldIndexSource>().map(|r| r.0.clone()) else {
            log::error!("sf_world_bevy: no WorldIndexSource resource; can't stream {ws}");
            self.unavailable.insert(ws.clone());
            return;
        };
        let id = ws.clone();
        let task = IoTaskPool::get().spawn(async move {
            match reader.directory(&id)? {
                Some(bytes) => CellDirectory::from_bytes(&bytes).map(Some).map_err(|e| e.to_string()),
                None => Ok(None),
            }
        });
        self.directories.insert(ws.clone(), task);
    }

    fn poll_directories(&mut self, world: &mut World) {
        let done: Vec<WorldspaceId> =
            self.directories.iter().filter(|(_, t)| t.is_finished()).map(|(ws, _)| ws.clone()).collect();
        for ws in done {
            let task = self.directories.remove(&ws).expect("listed above");
            match future::block_on(task) {
                Ok(Some(dir)) => {
                    let overrides = world.get_resource::<WorldspaceRecords>().and_then(|r| r.0.get(&ws).and_then(|w| w.rings));
                    let base = self.scheduler.config().rings;
                    let rings = overrides.as_ref().map_or(base, |o| base.with_overrides(o));
                    self.rt.worlds.insert(ws.clone(), WsInfo { partition: dir.partition, rings });
                    WorldspaceRegistry::ensure_in(world, &ws);
                    self.scheduler.add_worldspace(dir, overrides.as_ref());
                }
                Ok(None) => {
                    log::warn!("sf_world_bevy: worldspace {ws} has no baked index");
                    self.unavailable.insert(ws);
                }
                Err(e) => {
                    log::error!("sf_world_bevy: loading the index of {ws}: {e}");
                    self.unavailable.insert(ws);
                }
            }
        }
    }

    // -- fetch ---------------------------------------------------------------

    fn start_fetch(&mut self, world: &World, request: &Request) {
        let Some(reader) = world.get_resource::<WorldIndexSource>().map(|r| r.0.clone()) else { return };
        let overlay = world.get_resource::<WorldOverlay>().map(|o| o.0.clone());
        let Some(dir) = self.scheduler.directory(&request.worldspace) else { return };
        let partition = dir.partition;
        let content = LevelContent::of(dir).with_proxies(self.rt.far.is_installed());
        let cache = self.cache.clone();
        let (ws, cell, from, to) = (request.worldspace.clone(), request.cell, request.from, request.to);
        let task = IoTaskPool::get().spawn(async move {
            let page = load_page(&*reader, &cache, &ws, Partition::page_of(cell))?;
            let indexed = page.cell(cell).map(|c| c.placements.as_slice()).unwrap_or_default();
            let placements = match &overlay {
                Some(o) => o.read().unwrap_or_else(PoisonError::into_inner).apply(&ws, partition, cell, indexed),
                None => indexed.to_vec(),
            };
            Ok(Arc::new(content.step(&placements, from, to).cloned().collect()))
        });
        self.fetches.insert(request.id, task);
        let near = match world.get_resource::<Precombines>() {
            Some(p)
                if self.rt.config.precombine
                    && self.rt.far.is_installed()
                    && request.kind == RequestKind::Load
                    && request.to == Level::Near =>
            {
                (p.path)(world, &request.worldspace, request.cell).map(|path| world.resource::<AssetServer>().load(path))
            }
            _ => None,
        };
        self.rt.work.insert(request.id, Work { near, ..Work::default() });
    }

    fn poll_fetches(&mut self, world: &World, now: f64, spawners: &PlacementSpawners, counters: Option<&Counters>) {
        let asset_server = world.resource::<AssetServer>();
        let work = &self.rt.work;
        // A near load's fetch also waits for its precombine (loaded or failed).
        let settled = |id: &RequestId| {
            work.get(id).and_then(|w| w.near.as_ref()).is_none_or(|h| {
                !matches!(asset_server.get_load_state(h.id()), Some(LoadState::NotLoaded | LoadState::Loading))
            })
        };
        let done: Vec<RequestId> =
            self.fetches.iter().filter(|(id, t)| t.is_finished() && settled(id)).map(|(id, _)| *id).collect();
        for id in done {
            let task = self.fetches.remove(&id).expect("listed above");
            let placements = future::block_on(task).unwrap_or_else(|e| {
                log::error!("sf_world_bevy: fetch failed: {e}");
                self.rt.stats.fetch_errors += 1;
                Arc::new(Vec::new())
            });
            let Some(request) = self.scheduler.request(id).cloned() else {
                self.rt.work.remove(&id);
                continue;
            };
            let near = self.rt.work.get(&id).and_then(|w| w.near.clone());
            let members = near.as_ref().and_then(|h| world.resource::<Assets<CellPrecombine>>().get(h)).and_then(|c| {
                let members = valid_members(c, &placements, world.get_resource::<WorldOverlay>());
                self.rt.stats.precombine_fallbacks += u64::from(members.is_none());
                members
            });
            // Models to load: only for spawning steps, only those a spawner
            // will use, and none for precombined members.
            let mut models = BTreeSet::new();
            if request.kind == RequestKind::Load && request.to != Level::Active {
                for p in placements.iter() {
                    if let Some(m) = &p.placement.model
                        && !members.as_ref().is_some_and(|s| s.contains(&p.placement.id))
                        && spawners.for_class(p.placement.class).is_some_and(|s| s.needs_model(p))
                    {
                        models.insert(m.clone());
                    }
                }
            }
            let upload = self.upload_estimate(&request);
            let work = self.rt.work.entry(id).or_default();
            work.placements = placements;
            work.models = models.into_iter().collect();
            match members {
                Some(m) => work.members = Arc::new(m),
                None => work.near = None,
            }
            let assets = u32::try_from(work.models.len()).unwrap_or(u32::MAX);
            self.scheduler.fetched(id, now, assets, upload);
            if let Some(c) = counters {
                c.add_named("stream/fetch/items", unit::COUNT, 1.0);
            }
        }
    }

    /// Bytes a load step will upload, from the directory's estimates.
    fn upload_estimate(&self, r: &Request) -> u64 {
        let Some(dir) = self.scheduler.directory(&r.worldspace) else { return 0 };
        let Some(i) = dir.find(r.cell) else { return 0 };
        let c = &dir.cells[i];
        match (r.to, &self.rt.far.proxies) {
            (Level::Far, Some(p)) => p.bytes(&r.worldspace, r.cell),
            (Level::Near, Some(_)) => c.bytes_all,
            (Level::Far, None) => c.bytes_far,
            (Level::Near, None) => c.bytes_all.saturating_sub(c.bytes_far),
            _ => 0,
        }
    }

    // -- main-thread stages --------------------------------------------------

    fn run_stage(&mut self, world: &mut World, stage: Stage, spawners: &PlacementSpawners, counters: Option<&Counters>) {
        let started = Instant::now();
        let Self { scheduler, rt, .. } = self;
        let report = scheduler.run_stage(stage, &mut |request| {
            let t = Instant::now();
            let done = match stage {
                Stage::Spawn => rt.spawn_item(world, request, spawners),
                Stage::Upload => rt.reveal(world, request),
                Stage::Activate => rt.activate_item(world, request, spawners, true),
                Stage::Deactivate => rt.activate_item(world, request, spawners, false),
                Stage::Despawn => rt.despawn_item(world, request),
                Stage::Fetch | Stage::Assets => unreachable!("async stages"),
            };
            #[allow(clippy::cast_precision_loss)]
            let cost = if stage == Stage::Upload { request.upload_bytes as f64 } else { t.elapsed().as_secs_f64() * 1000.0 };
            StageStep { cost, done }
        });

        for change in &report.completed {
            if change.to == Level::Unloaded {
                self.rt.cells.remove(&(change.worldspace.clone(), change.cell));
            }
            if change.to > change.from {
                if let Some(mut m) = world.get_resource_mut::<Messages<CellLoaded>>() {
                    m.write(CellLoaded { worldspace: change.worldspace.clone(), cell: change.cell, level: change.to });
                }
            } else if let Some(mut m) = world.get_resource_mut::<Messages<CellUnloaded>>() {
                m.write(CellUnloaded { worldspace: change.worldspace.clone(), cell: change.cell, level: change.to });
            }
        }

        if let Some(c) = counters {
            let name = stage_name(stage);
            if stage == Stage::Upload {
                c.add_named(&format!("stream/{name}/bytes"), unit::BYTES, report.used);
            }
            c.add_named(&format!("stream/{name}/items"), unit::COUNT, f64::from(report.items));
            if report.items > 0 {
                c.add_duration_named(&format!("stream/{name}/ms"), started.elapsed());
            }
        }
    }

    fn report_counters(&self, c: &Counters, now: f64) {
        for stage in [Stage::Fetch, Stage::Assets, Stage::Spawn, Stage::Upload, Stage::Activate, Stage::Deactivate, Stage::Despawn]
        {
            let (waiting, running) = self.scheduler.queue_depth(stage);
            #[allow(clippy::cast_precision_loss)]
            c.set_named(&format!("stream/{}/queue", stage_name(stage)), unit::COUNT, (waiting + running) as f64);
        }
        #[allow(clippy::cast_precision_loss)]
        {
            c.set_named("stream/requests", unit::COUNT, self.scheduler.requests().count() as f64);
            c.set_named("stream/resident_cells", unit::COUNT, self.rt.cells.len() as f64);
            c.set_named("stream/models", unit::COUNT, self.rt.models.len() as f64);
            c.set_named("stream/model_queue", unit::COUNT, self.rt.load_queue.len() as f64);
            let cache = self.cache.lock().unwrap_or_else(PoisonError::into_inner).used_bytes();
            c.set_named("stream/page_cache/bytes", unit::BYTES, cache as f64);
        }
        c.set_named("stream/oldest_request/s", unit::SECONDS, self.scheduler.oldest_request_age(now));
        c.set_named("stream/latency/s", unit::SECONDS, self.scheduler.latency());
    }
}

impl Runtime {
    // -- assets --------------------------------------------------------------

    fn start_assets(&mut self, world: &World, request: &Request) {
        let Some(work) = self.work.get_mut(&request.id) else { return };
        for model in &work.models {
            let slot = self.models.entry(model.clone()).or_insert_with(|| {
                self.load_queue.push_back(model.clone());
                ModelSlot { handle: None, users: 0, state: SlotState::Loading }
            });
            slot.users += 1;
        }
        work.acquired = true;
        if request.kind == RequestKind::Load
            && request.to == Level::Far
            && let Some(proxies) = &self.far.proxies
        {
            proxies.acquire(world, &request.worldspace, request.cell);
            work.proxy = true;
        }
    }

    fn model_ready(&self, model: &VfsPath) -> bool {
        self.models.get(model).is_none_or(|s| matches!(s.state, SlotState::Ready(_)))
    }

    /// A placement's resolved model, if its slot is ready.
    fn loaded_model(&self, placement: &IndexedPlacement) -> Option<&LoadedModel> {
        placement.placement.model.as_ref().and_then(|m| match self.models.get(m).map(|s| &s.state) {
            Some(SlotState::Ready(loaded)) => Some(loaded),
            _ => None,
        })
    }

    /// Whether the assets spawners prepare for this load have settled
    /// (loaded or failed, with dependencies). The first call, once every
    /// model resolved, asks each placement's spawner (weather.md bug B3).
    fn prepared(&mut self, world: &World, id: RequestId, spawners: &PlacementSpawners) -> bool {
        let asset_server = world.resource::<AssetServer>();
        let Some(work) = self.work.get(&id) else { return true };
        if work.prepared.is_none() {
            let mut seen = HashSet::new();
            let mut handles = Vec::new();
            for p in work.placements.iter() {
                let model = self.loaded_model(p);
                let Some(handle) = spawners.spawner_for(p, model).and_then(|s| s.prepare(asset_server, p, model)) else {
                    continue;
                };
                if seen.insert(handle.id()) {
                    handles.push(handle);
                }
            }
            self.stats.prepared += handles.len() as u64;
            if let Some(work) = self.work.get_mut(&id) {
                work.prepared = Some(handles);
            }
        }
        self.work.get(&id).and_then(|w| w.prepared.as_ref()).is_none_or(|handles| {
            handles.iter().all(|h| {
                !matches!(
                    asset_server.get_recursive_dependency_load_state(h.id()),
                    Some(RecursiveDependencyLoadState::NotLoaded | RecursiveDependencyLoadState::Loading)
                )
            })
        })
    }

    /// Finishes loads: builds templates of loaded models, marks failed ones
    /// uncooked.
    fn poll_models(&mut self, world: &mut World, burst: bool) {
        self.start_model_loads(world.resource::<AssetServer>(), burst);
        let loading: Vec<VfsPath> = self
            .models
            .iter()
            .filter(|(_, s)| s.handle.is_some() && matches!(s.state, SlotState::Loading))
            .map(|(m, _)| m.clone())
            .collect();
        if loading.is_empty() {
            return;
        }
        let mapper = world.get_resource::<ModelMaterials>().cloned().unwrap_or_default();
        world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
            let asset_server = world.resource::<AssetServer>().clone();
            let cooked = world.resource::<Assets<CookedModelAsset>>();
            for model in loading {
                let slot = self.models.get_mut(&model).expect("listed above");
                let Some(handle) = &slot.handle else { continue };
                let state = match asset_server.get_recursive_dependency_load_state(handle) {
                    Some(RecursiveDependencyLoadState::Loaded) => match cooked.get(handle) {
                        Some(asset) => {
                            if let Some(reason) = &asset.skipped {
                                LoadedModel::Uncooked(reason.as_str().into())
                            } else if asset.skin.is_some() {
                                // Deforming models spawn through sf_model_bevy::spawn_skinned
                                // (character/actor path), not the static template.
                                LoadedModel::Uncooked("skinned".into())
                            } else {
                                #[allow(clippy::cast_precision_loss)]
                                let n = asset.meshes.len() as f32;
                                self.meshes_per_model += (n - self.meshes_per_model) * 0.05;
                                LoadedModel::Cooked(Arc::new(build_template(
                                    asset,
                                    &mut materials,
                                    &mut self.materials,
                                    &mapper,
                                    &asset_server,
                                )))
                            }
                        }
                        None => continue,
                    },
                    Some(RecursiveDependencyLoadState::Failed(e)) => {
                        log::debug!("sf_world_bevy: {model} is not cooked: {e}");
                        LoadedModel::Uncooked(e.to_string().into())
                    }
                    _ => continue,
                };
                slot.state = SlotState::Ready(state);
            }
        });
    }

    /// Starts queued model loads while the expected meshes in flight stay
    /// under [`StreamingConfig::max_loading_meshes`] (all of them in burst
    /// mode or without a cap). Released models leave the queue unloaded.
    fn start_model_loads(&mut self, asset_server: &AssetServer, burst: bool) {
        if self.load_queue.is_empty() {
            return;
        }
        let budget = self.config.max_loading_meshes;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
        let cap = if burst || budget == 0 {
            usize::MAX
        } else {
            ((budget as f32 / self.meshes_per_model.max(1.0)) as usize).max(1)
        };
        let mut in_flight =
            self.models.values().filter(|s| s.handle.is_some() && matches!(s.state, SlotState::Loading)).count();
        while in_flight < cap {
            let Some(model) = self.load_queue.pop_front() else { break };
            let Some(slot) = self.models.get_mut(&model) else { continue };
            if slot.handle.is_some() {
                continue;
            }
            slot.handle =
                Some(asset_server.load(baked_path(&self.config.mesh_step, model.as_str(), Some(&self.config.mesh_ext))));
            in_flight += 1;
        }
    }

    fn release_models(&mut self, models: &[VfsPath]) {
        for m in models {
            if let Some(slot) = self.models.get_mut(m) {
                slot.users = slot.users.saturating_sub(1);
                if slot.users == 0 {
                    self.models.remove(m);
                }
            }
        }
    }

    // -- cancel --------------------------------------------------------------

    /// Undoes a cancelled load: its partly spawned group, its model refs,
    /// and the cell entity if the cell holds nothing else.
    fn cancel(&mut self, world: &mut World, request: &Request) {
        let Some(work) = self.work.remove(&request.id) else { return };
        if let Some(group) = work.group
            && let Ok(e) = world.get_entity_mut(group)
        {
            e.despawn();
        }
        if work.acquired {
            self.release_models(&work.models);
        }
        if work.proxy
            && let Some(proxies) = self.far.proxies.clone()
        {
            proxies.release(world, &request.worldspace, request.cell);
        }
        let key = (request.worldspace.clone(), request.cell);
        if request.from == Level::Unloaded
            && self.cells.get(&key).is_some_and(|c| c.groups.is_empty() && c.active.is_none())
            && let Some(c) = self.cells.remove(&key)
            && let Ok(e) = world.get_entity_mut(c.entity)
        {
            e.despawn();
        }
    }

    // -- main-thread stage items -----------------------------------------------

    fn info(&self, ws: &WorldspaceId) -> WsInfo {
        self.worlds.get(ws).copied().unwrap_or(WsInfo { partition: Partition::STREAMING, rings: self.config.scheduler.rings })
    }

    /// The cell's entity, spawned on first use.
    fn cell_entity(&mut self, world: &mut World, ws: &WorldspaceId, cell: IVec3) -> Entity {
        let key = (ws.clone(), cell);
        if let Some(c) = self.cells.get(&key) {
            return c.entity;
        }
        let entry = WorldspaceRegistry::ensure_in(world, ws);
        let (coord, transform) = to_grid(self.info(ws).partition.cell_origin(cell));
        let entity = world
            .spawn((
                StreamCell { worldspace: ws.clone(), cell },
                coord,
                transform,
                Stationary,
                Visibility::Inherited,
                ChildOf(entry.root),
            ))
            .id();
        self.cells.insert(key, CellRuntime { entity, groups: BTreeMap::new(), active: None });
        entity
    }

    /// What decides where placements of this load stop being drawn.
    fn ends(&self, world: &World, request: &Request, info: &WsInfo) -> Ends {
        let handoff = self.far.handoff.filter(|_| self.far.is_installed());
        // Size culling (industry-audit.md J1) keeps what the cell's proxy
        // represents drawn to the handoff.
        let kept = match (&self.far.proxies, handoff) {
            (Some(proxies), Some(_)) => proxies.kept_radius(world, &request.worldspace, request.cell),
            _ => 0.0,
        };
        Ends {
            lod: self.config.lod,
            rings: info.rings,
            content: LevelContent { far_min_radius: info.rings.far_min_radius, proxies: self.far.is_installed() },
            handoff,
            kept,
        }
    }

    /// Spawns a cell's precombine buckets (precombine.md) into a new batch.
    /// Each bucket draws as far as its farthest-reaching member: to the
    /// far-field handoff (measured from the bucket origin, which is the
    /// proxy's) or, for members that end earlier (size culling) or at the
    /// ring, that end plus the bucket's reach.
    fn spawn_precombine(
        &mut self,
        world: &mut World,
        request: &Request,
        handle: &Handle<CellPrecombine>,
        group: Entity,
        layers: &RenderLayers,
        ends: &Ends,
    ) -> u32 {
        let Some(work) = self.work.get(&request.id) else { return 0 };
        let (placements, members) = (work.placements.clone(), work.members.clone());
        let by_id: HashMap<PlacementId, &IndexedPlacement> =
            placements.iter().filter(|p| members.contains(&p.placement.id)).map(|p| (p.placement.id, p)).collect();
        let origin = self.info(&request.worldspace).partition.cell_origin(request.cell);
        let mapper = world.get_resource::<ModelMaterials>().cloned().unwrap_or_default();
        let mut buckets = Vec::new();
        world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
            let asset_server = world.resource::<AssetServer>().clone();
            let Some(combine) = world.resource::<Assets<CellPrecombine>>().get(handle) else { return };
            for bucket in &combine.buckets {
                let (mut end, mut band) = (0.0f32, 0.0f32);
                for p in bucket.members.iter().filter_map(|id| by_id.get(id)) {
                    let e = ends.of(p);
                    let reach = if e.at_handoff { 0.0 } else { bucket.reach };
                    if e.distance + reach > end {
                        (end, band) = (e.distance + reach, e.band);
                    }
                }
                let template = build_template(&bucket.model, &mut materials, &mut self.materials, &mapper, &asset_server);
                let errors: Vec<f32> = template.lods.iter().map(|l| l.error).collect();
                let ranges = bucket_ranges(&errors, &ends.lod, end, band, bucket.reach);
                buckets.push((template, ranges, end, band));
            }
        });
        let Some(center) = world.resource::<Assets<CellPrecombine>>().get(handle).map(|c| c.center) else { return 0 };
        let batch = world.spawn((Transform::IDENTITY, Visibility::Inherited, ChildOf(group))).id();
        let mut n = 1;
        for (template, ranges, end, band) in &buckets {
            let cx = SpawnCx {
                worldspace: &request.worldspace,
                cell: request.cell,
                level: request.to,
                parent: batch,
                layers,
                local: Transform::from_translation(origin.delta_f32(center)),
                model: None,
                max_distance: *end,
                end_band: *band,
                lod: &ends.lod,
            };
            n += spawn_template_ranges(world, &cx, template, ranges);
        }
        n
    }

    /// Spawns the next few placements of a load step into a new batch.
    fn spawn_item(&mut self, world: &mut World, request: &Request, spawners: &PlacementSpawners) -> bool {
        let cell_entity = self.cell_entity(world, &request.worldspace, request.cell);
        let info = self.info(&request.worldspace);
        let ends = self.ends(world, request, &info);
        let Some(work) = self.work.get_mut(&request.id) else { return true };
        let group =
            *work.group.get_or_insert_with(|| world.spawn((Transform::IDENTITY, Visibility::Hidden, ChildOf(cell_entity))).id());
        let layers = world
            .resource::<WorldspaceRegistry>()
            .get(&request.worldspace)
            .map_or_else(RenderLayers::default, |e| RenderLayers::layer(e.layer));
        let origin = info.partition.cell_origin(request.cell);
        // The far-field proxy first: one spawn item of its own.
        if work.proxy && !work.proxy_spawned {
            work.proxy_spawned = true;
            if let Some(proxies) = self.far.proxies.clone() {
                let cx = ProxySpawnCx {
                    worldspace: &request.worldspace,
                    cell: request.cell,
                    parent: group,
                    cell_origin: origin,
                    layers: &layers,
                };
                self.stats.entities_spawned += u64::from(proxies.spawn(world, &cx));
            }
            return self.work.get(&request.id).is_none_or(|w| w.next >= w.placements.len());
        }
        // Then the precombine's buckets: one spawn item of their own.
        if !work.near_spawned
            && let Some(handle) = work.near.clone()
        {
            work.near_spawned = true;
            let n = self.spawn_precombine(world, request, &handle, group, &layers, &ends);
            self.stats.entities_spawned += u64::from(n);
            return self.work.get(&request.id).is_none_or(|w| w.next >= w.placements.len());
        }
        let (start, end) = (work.next, (work.next + self.config.placements_per_item).min(work.placements.len()));
        if start >= end {
            return true;
        }
        let placements = work.placements.clone();
        let members = work.members.clone();
        let batch = world.spawn((Transform::IDENTITY, Visibility::Inherited, ChildOf(group))).id();
        let mut entities = 0u64;
        for p in &placements[start..end] {
            if members.contains(&p.placement.id) {
                self.stats.precombined += 1;
                continue;
            }
            let model = p.placement.model.as_ref().and_then(|m| match self.models.get(m).map(|s| &s.state) {
                Some(SlotState::Ready(loaded)) => Some(loaded),
                _ => None,
            });
            if matches!(model, Some(LoadedModel::Uncooked(_))) {
                self.stats.uncooked += 1;
            }
            let Some(spawner) = spawners.spawner_for(p, model) else { continue };
            let t = &p.placement.transform;
            let end = ends.of(p);
            let (max_distance, end_band) = (end.distance, end.band);
            self.stats.size_culled += u64::from(end.size_culled);
            let cx = SpawnCx {
                worldspace: &request.worldspace,
                cell: request.cell,
                level: request.to,
                parent: batch,
                layers: &layers,
                local: Transform { translation: origin.delta_f32(t.pos), rotation: t.rotation, scale: t.scale },
                model,
                max_distance,
                end_band,
                lod: &self.config.lod,
            };
            entities += u64::from(spawner.spawn(world, &cx, p));
        }
        self.stats.entities_spawned += entities + 1;
        self.stats.placements_spawned += (end - start) as u64;
        let work = self.work.get_mut(&request.id).expect("checked above");
        work.next = end;
        end >= placements.len()
    }

    /// The last stage of a load: reveal the level group at once.
    fn reveal(&mut self, world: &mut World, request: &Request) -> bool {
        let cell_entity = self.cell_entity(world, &request.worldspace, request.cell);
        let Some(mut work) = self.work.remove(&request.id) else { return true };
        let group = work.group.take().unwrap_or_else(|| world.spawn((Transform::IDENTITY, ChildOf(cell_entity))).id());
        if let Ok(mut e) = world.get_entity_mut(group) {
            e.insert(Visibility::Inherited);
        }
        let cell = self.cells.get_mut(&(request.worldspace.clone(), request.cell)).expect("spawned above");
        cell.groups.insert(request.to, Group { entity: group, models: std::mem::take(&mut work.models), _precombine: work.near.take() });
        true
    }

    /// Activation hooks for a few placements (R0 enter or leave).
    fn activate_item(&mut self, world: &mut World, request: &Request, spawners: &PlacementSpawners, on: bool) -> bool {
        let key = (request.worldspace.clone(), request.cell);
        let cell_entity = self.cell_entity(world, &request.worldspace, request.cell);
        if !on && !self.work.contains_key(&request.id) {
            let placements = self.cells.get_mut(&key).and_then(|c| c.active.take()).unwrap_or_default();
            self.work.insert(request.id, Work { placements, ..Work::default() });
        }
        let Some(work) = self.work.get_mut(&request.id) else { return true };
        let end = (work.next + self.config.placements_per_item).min(work.placements.len());
        let discard = !on && self.reloading.contains(&key);
        let cx = ActivateCx { worldspace: &request.worldspace, cell: request.cell, cell_entity, discard };
        for p in &work.placements[work.next..end] {
            if let Some(s) = spawners.for_class(p.placement.class) {
                if on {
                    s.activate(world, &cx, p);
                } else {
                    s.deactivate(world, &cx, p);
                }
            }
        }
        work.next = end;
        if work.next < work.placements.len() {
            return false;
        }
        let work = self.work.remove(&request.id).expect("checked above");
        if on && let Some(cell) = self.cells.get_mut(&key) {
            cell.active = Some(work.placements);
        }
        true
    }

    /// Despawns one batch of the level being unloaded (then its group, and
    /// the cell entity when the cell leaves its last level).
    fn despawn_item(&mut self, world: &mut World, request: &Request) -> bool {
        let key = (request.worldspace.clone(), request.cell);
        if !self.work.contains_key(&request.id) {
            let mut queue = Vec::new();
            let mut released = Vec::new();
            if let Some(cell) = self.cells.get_mut(&key) {
                if let Some(group) = cell.groups.remove(&request.from) {
                    if let Some(children) = world.get::<Children>(group.entity) {
                        queue.extend(children.iter());
                    }
                    queue.push(group.entity);
                    released = group.models;
                }
                if request.to == Level::Unloaded {
                    // Anything left (normally nothing), then the cell itself.
                    for (_, g) in std::mem::take(&mut cell.groups) {
                        queue.push(g.entity);
                        released.extend(g.models);
                    }
                    queue.push(cell.entity);
                }
            }
            queue.reverse(); // popped from the end: batches first, the cell last
            self.work.insert(request.id, Work { despawn: Some(queue), released, ..Work::default() });
        }
        let work = self.work.get_mut(&request.id).expect("inserted above");
        let queue = work.despawn.as_mut().expect("despawn work");
        if let Some(entity) = queue.pop()
            && let Ok(e) = world.get_entity_mut(entity)
        {
            e.despawn();
            self.stats.entities_despawned += 1;
        }
        if !queue.is_empty() {
            return false;
        }
        let work = self.work.remove(&request.id).expect("present");
        self.release_models(&work.released);
        if request.from == Level::Far
            && let Some(proxies) = self.far.proxies.clone()
        {
            proxies.release(world, &request.worldspace, request.cell);
        }
        true
    }
}

fn stage_name(stage: Stage) -> &'static str {
    match stage {
        Stage::Fetch => "fetch",
        Stage::Assets => "assets",
        Stage::Spawn => "spawn",
        Stage::Upload => "upload",
        Stage::Activate => "activate",
        Stage::Deactivate => "deactivate",
        Stage::Despawn => "despawn",
    }
}

/// Where placements of one load stop being drawn ([`Runtime::ends`]).
struct Ends {
    lod: LodScreen,
    rings: RingConfig,
    content: LevelContent,
    /// The far-field handoff distance and band, with a far field.
    handoff: Option<(f32, f32)>,
    /// The smallest radius the cell's proxy represents (size culling clamp).
    kept: f32,
}

/// One placement's end ([`Ends::of`]).
struct End {
    distance: f32,
    band: f32,
    /// Ends at the far-field handoff (not earlier).
    at_handoff: bool,
    /// Size culling ended it before its ring or the handoff.
    size_culled: bool,
}

impl Ends {
    fn of(&self, p: &IndexedPlacement) -> End {
        // A far-field proxy takes over from eligible placements at its
        // handoff; the rest keep their ring's range.
        let (max_distance, end_band, kept_radius) = match self.handoff {
            Some((d, band)) if p.placement.far_eligible() => (d, band, Some(self.kept)),
            _ if self.content.level_of(p) == Some(Level::Far) => (self.rings.far.unload, 0.0, None),
            _ => (self.rings.near.unload, 0.0, None),
        };
        let (distance, band) = size_culled_end(&self.lod, p, max_distance, end_band, kept_radius);
        let size_culled = distance < max_distance;
        End { distance, band, at_handoff: kept_radius.is_some() && !size_culled, size_culled }
    }
}

/// A precombine's members, if every one is among the fetched placements and
/// untouched by the overlay (precombine.md, phase 1: any moved, removed or
/// missing member drops the whole precombine; the cell spawns the old way).
fn valid_members(combine: &CellPrecombine, placements: &[IndexedPlacement], overlay: Option<&WorldOverlay>) -> Option<HashSet<PlacementId>> {
    let fetched: HashSet<PlacementId> = placements.iter().map(|p| p.placement.id).collect();
    let overlay = overlay.map(|o| o.0.read().unwrap_or_else(PoisonError::into_inner));
    let mut members = HashSet::new();
    for &id in combine.buckets.iter().flat_map(|b| &b.members) {
        let touched = overlay.as_ref().is_some_and(|o| o.is_removed(id) || o.placed(id).is_some());
        if touched || !fetched.contains(&id) {
            return None;
        }
        members.insert(id);
    }
    Some(members)
}

/// A decoded page, from the cache or the reader.
fn load_page(reader: &dyn IndexReader, cache: &Mutex<PageCache>, ws: &WorldspaceId, page: IVec3) -> Result<Arc<IndexPage>, String> {
    if let Some(p) = cache.lock().unwrap_or_else(PoisonError::into_inner).get(ws, page) {
        return Ok(p);
    }
    let bytes = reader.page(ws, page)?.ok_or_else(|| format!("{ws}: page {page} missing from the index"))?;
    let decoded = Arc::new(IndexPage::from_bytes(&bytes).map_err(|e| e.to_string())?);
    cache.lock().unwrap_or_else(PoisonError::into_inner).insert(ws, decoded.clone(), bytes.len());
    Ok(decoded)
}

/// Turns a loaded cooked model into a template, deduplicating materials by
/// description.
fn build_template(
    asset: &CookedModelAsset,
    materials: &mut Assets<StandardMaterial>,
    cache: &mut HashMap<String, Handle<StandardMaterial>>,
    mapper: &ModelMaterials,
    asset_server: &AssetServer,
) -> ModelTemplate {
    let mut material = |i: u32| -> Handle<StandardMaterial> {
        let desc = asset.materials.get(i as usize).cloned().unwrap_or_default();
        let key = format!("{desc:?}");
        cache.entry(key).or_insert_with(|| materials.add((mapper.0)(&desc, asset_server))).clone()
    };
    let lods = asset
        .lods
        .iter()
        .map(|lod| TemplateLod {
            error: lod.error,
            parts: lod.parts.iter().map(|(mesh, mat)| (asset.meshes[*mesh].clone(), material(*mat))).collect(),
        })
        .collect();
    ModelTemplate { lods, aabb: model_aabb(&asset.bounds) }
}
