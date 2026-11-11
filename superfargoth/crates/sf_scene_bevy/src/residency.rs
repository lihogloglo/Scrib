//! Cell residency: which cells' tables are loading, planned, resident or
//! fallen back, and the [`CellScene`] hook streaming drives them through.
//!
//! ## The ruling this implements (scene-runtime-plan.md §5.3, 2026-11-11)
//!
//! **Plan before `covered`.** A near load's fetch stays open until the cell's
//! table, the chain products it names and their GPU slabs are resolved and
//! `SceneSpace::plan_in` has run (or the cell fell back); only then does
//! streaming ask [`CellScene::covered`], which answers exactly the placements
//! the plan placed. Overflowed placements (`CellPlan::overflowed`) are not in
//! it, and a cell whose chain failed answers `None`: either way their models
//! are gathered and they spawn as entities like any uncovered placement. This
//! is asset-gated stage completion (UE World Partition, Bethesda cell attach),
//! the shape the near precombine already uses in `sf_world_bevy`.
//!
//! ## The three-deep gate (B3 brief §1.2), as [`LoadPhase`]
//!
//! 1. [`LoadPhase::Table`]: the cell's `scene.cells` product resolves.
//! 2. [`LoadPhase::Chains`]: every chain product it names, with its meshes;
//!    each chain's material is made as it resolves (`chains::chain_material`).
//! 3. [`LoadPhase::Slabs`]: every chain level mesh and chain material is in
//!    [`ChainSlabTable`], i.e. the render world allocated the mesh and
//!    prepared the material into its bind group (skipped without a render
//!    world, and bounded by [`SLAB_WAIT_FRAMES`]). The batch key is fixed at
//!    `plan_in`, so the material's bind group must be known before it.
//!
//! then [`LoadPhase::Planned`] (planned, not committed) or
//! [`LoadPhase::Fallback`]. [`CellScene::upload`] commits a planned cell and
//! moves it from `loading` into the space's resident set.
//!
//! The hook gets `&World` in `acquire`, `ready` and `covered`, so it cannot
//! insert a load itself: `acquire` starts the table load and posts the cell to
//! [`SceneInbox`], which [`poll_cell_loads`] drains the same frame (it runs
//! after `StreamingSystems::Drive`).

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Mutex, PoisonError};

use bevy_asset::{AssetId, AssetServer, Assets, Handle, LoadState, UntypedHandle};
use bevy_asset::{AssetLoadError, io::AssetReaderError};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_mesh::Mesh;
use bevy_pbr::gpu_instance_batch::{GpuBatchedMesh3d, GpuBatchedMeshRange};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_render::sync_world::MainEntity;
use sf_coords::WorldPos;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::{Counters, unit};
use sf_model_bevy::{CookedModelAsset, baked_path};
use sf_scene::{
    BatchId, BatchKey, CELLS_EXT, CELLS_STEP, CellPlan, ChainEntry, ChainKey, ChainSlot, NO_SLOT, SceneError, SceneSpace, SlotRange,
    SlotWrite, SpaceConfig, SpaceId, cell_subject,
};
use sf_world::{PlacementId, WorldspaceId};
use sf_world_bevy::{CellScene, ModelMaterials, SceneCellCx, Streaming, WorldOrigin, WorldOverlay};

use crate::chains::{
    ChainAssets, ChainMaterials, MaterialMaker, ProductState, chain_assets, load_product, product_state,
};
use crate::slabs::{CellSlabs, ChainIds, ChainSlabTable};
use crate::upload::{BatchSlotWrite, SLOT_BYTES, gpu_instance};
use crate::{SceneConfig, SceneSupport, SceneTableAsset};

/// Frames a planned-to-be cell waits for its chain meshes' slabs before it
/// plans without them. A mesh the render world never allocates must not hold
/// a streaming fetch open for ever; the chains still unresolved then get a
/// batch of their own (correct, just more draws), with one `warn!`.
pub const SLAB_WAIT_FRAMES: u64 = 120;

/// The scene's frame counter: what releases are stamped with
/// (`SceneSpace::advance`).
#[derive(Resource, Default, Debug)]
pub struct SceneFrame(pub u64);

/// Every worldspace's retained scene.
#[derive(Resource, Default)]
pub struct SceneSpaces(pub BTreeMap<WorldspaceId, SpaceState>);

/// One worldspace's scene and its Bevy-side tables.
pub struct SpaceState {
    pub space: SceneSpace,
    /// Indexed by `ChainSlot`: the assets of every resident chain. Grown on
    /// demand to the registry's high water.
    pub chains: Vec<Option<ChainAssets>>,
    /// Indexed by `BatchId` (dense, never reused): each batch's
    /// `GpuBatchedMesh3d` entity, while it has one (B3 brief §4.3 steps 5, 8, 10).
    pub batches: Vec<Option<BatchEntity>>,
    /// Cells acquired and not yet committed (or fallen back, until released).
    pub loading: HashMap<IVec3, CellLoad>,
    /// The table of every resident cell: `SceneSpace::promote` needs its view
    /// (B3 brief §3 item 16), so it lives as long as the cell.
    pub tables: HashMap<IVec3, Handle<SceneTableAsset>>,
    /// Next to `tables`, each resident cell's covered placements, global → the
    /// table's cell-local id (`plan.covered` through `SceneCellCx::indexed`): how
    /// [`PromotePlacement`] reaches `SceneSpace::promote`. Dropped with the table.
    pub covered: HashMap<IVec3, HashMap<PlacementId, sf_scene::PlacementId>>,
}

impl SpaceState {
    pub fn new(config: SpaceConfig) -> Self {
        Self {
            space: SceneSpace::new(config),
            chains: Vec::new(),
            batches: Vec::new(),
            loading: HashMap::new(),
            tables: HashMap::new(),
            covered: HashMap::new(),
        }
    }

    /// The assets of the chain at `slot`, if it is resident.
    pub fn chain(&self, slot: ChainSlot) -> Option<&ChainAssets> {
        self.chains.get(slot as usize)?.as_ref()
    }

    fn store_chain(&mut self, slot: ChainSlot, assets: &ChainAssets) {
        let i = slot as usize;
        if self.chains.len() <= i {
            self.chains.resize(i + 1, None);
        }
        let entry = &mut self.chains[i];
        if entry.as_ref().is_none_or(|a| a.product.id() != assets.product.id()) {
            debug_assert!(entry.is_none(), "chain slot {slot} still held another chain's assets");
            *entry = Some(assets.clone());
        }
    }

    /// Drops the assets of every chain of `slots` that is no longer resident.
    fn drop_released(&mut self, slots: &[ChainSlot]) {
        for &slot in slots {
            if self.space.chains().entry(slot).is_none()
                && let Some(entry) = self.chains.get_mut(slot as usize)
            {
                *entry = None;
            }
        }
    }
}

/// A batch's entity, and the frame its range list last went empty.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatchEntity {
    pub entity: Entity,
    /// `Some(frame)` while `SceneSpace::batch_ranges` is empty: [`despawn_empty_batches`]
    /// despawns the entity once that has lasted more than the frames in flight.
    pub empty_since: Option<u64>,
}

/// Names a batch entity's batch: B4's way from the entity back to the space.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneBatch {
    pub space: SpaceId,
    pub batch: BatchId,
}

/// One cell between `acquire` and commit (or release).
pub struct CellLoad {
    pub table: Handle<SceneTableAsset>,
    pub phase: LoadPhase,
}

/// Where a cell's load stands (module docs).
pub enum LoadPhase {
    /// Waiting for the table.
    Table,
    /// The table resolved; its chains' products are loading. One entry per
    /// chain of the table, in its cell-local order.
    Chains { products: Vec<(ChainEntry, Handle<CookedModelAsset>)> },
    /// The products loaded; waiting for the render world to allocate their
    /// meshes (`since`: the frame this phase began).
    Slabs { chains: Vec<(ChainEntry, ChainAssets)>, since: u64 },
    /// `plan_in` ran; the plan is published by `upload` (`commit_in`) or given
    /// back by `release` (`cancel_in`).
    Planned { plan: Box<CellPlan> },
    /// The cell draws nothing from the scene: no product, a corrupt table, a
    /// failed chain or a `plan_in` error. Its placements spawn as entities.
    Fallback,
}

impl LoadPhase {
    /// Whether the hook has made its decision for the cell.
    pub fn decided(&self) -> bool {
        matches!(self, Self::Planned { .. } | Self::Fallback)
    }
}

/// Cells `acquire` started, for the next [`poll_cell_loads`].
#[derive(Resource, Default)]
pub struct SceneInbox(Mutex<Vec<Acquired>>);

/// A cell `acquire` started: its worldspace, its cell and its table load.
type Acquired = (WorldspaceId, IVec3, Handle<SceneTableAsset>);

impl SceneInbox {
    pub(crate) fn lock(&self) -> std::sync::MutexGuard<'_, Vec<Acquired>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn contains(&self, worldspace: &WorldspaceId, cell: IVec3) -> bool {
        self.lock().iter().any(|(w, c, _)| w == worldspace && *c == cell)
    }
}

/// One cell's scene-buffer upload. TODO(B3b/B4): filled by `upload`, read by
/// `extract_scene_uploads`; nothing writes it yet.
pub struct CellUpload {
    pub space: SpaceId,
    pub scene: SlotRange,
    pub bytes: Vec<u8>,
}

/// A new batch's first upload: all `slots` of its output written zero
/// (`is_active = 0`) before any of it draws, so a slot no cell wrote yet never
/// draws garbage (B3 brief §2.2, the empty-ranges footgun). Queued by
/// `upload` when it spawns the batch; read by B4.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatchZeroFill {
    pub space: SpaceId,
    pub batch: BatchId,
    pub entity: Entity,
    /// The batch's whole capacity (`SceneSpace::batch_capacity`: its class's).
    pub slots: u32,
}

/// GPU writes queued for the render world (`upload` module): `upload` fills
/// `zero_fills` and `writes`, `extract_scene_uploads` takes them.
/// TODO(B4b): `cells` stays empty until the scene buffer lands.
#[derive(Resource, Default)]
pub struct SceneUploads {
    pub cells: Vec<CellUpload>,
    /// One per `CellPlan::writes`, in plan order.
    pub writes: Vec<BatchSlotWrite>,
    pub zero_fills: Vec<BatchZeroFill>,
}

/// `First`: the frame counter, and every space frees what its frames in flight are done with.
pub fn tick_frame(mut frame: ResMut<SceneFrame>, mut spaces: ResMut<SceneSpaces>) {
    frame.0 += 1;
    for state in spaces.0.values_mut() {
        state.space.advance(frame.0);
    }
}

/// `First`, after [`tick_frame`]: despawns every batch entity whose range list
/// has been empty for more than the frames in flight (B3 brief §4.3 step 10).
/// One that refills before then keeps its entity, and with it its output buffer.
pub fn despawn_empty_batches(frame: Res<SceneFrame>, mut spaces: ResMut<SceneSpaces>, mut commands: Commands) {
    for state in spaces.0.values_mut() {
        let window = u64::from(state.space.config().frames_in_flight);
        for entry in &mut state.batches {
            if let Some(BatchEntity { entity, empty_since: Some(since) }) = *entry
                && frame.0.saturating_sub(since) > window
            {
                commands.entity(entity).despawn();
                *entry = None;
            }
        }
    }
}

/// Makes sure every batch of `batches` has its entity (B3 brief §4.3 step 5):
/// the batch's own capacity (its class's, `SpaceConfig::capacity_of_key`), no
/// range yet, no `Mesh3d`, no `NoCpuCulling`, no `NotShadowCaster`, and a
/// zero-fill of its whole capacity queued. Returns the batches it spawned.
fn ensure_batches(world: &mut World, state: &mut SpaceState, batches: &[(BatchId, BatchKey)]) -> Vec<BatchId> {
    let config = state.space.config();
    let frame = world.resource::<SceneFrame>().0;
    let mut spawned = Vec::new();
    for &(batch, _) in batches {
        let i = batch as usize;
        if state.batches.len() <= i {
            state.batches.resize(i + 1, None);
        }
        if state.batches[i].is_some() {
            continue;
        }
        let capacity = state.space.batch_capacity(batch);
        let entity = world
            .spawn((
                GpuBatchedMesh3d { mesh: Handle::default(), max_capacity: capacity, ranges: Vec::new() },
                // Re-pointed with `mesh` by `rebuild_ranges`: the first range's chain's material.
                MeshMaterial3d::<StandardMaterial>(Handle::default()),
                SceneBatch { space: config.space, batch },
            ))
            .id();
        // Empty until its ranges are rebuilt.
        state.batches[i] = Some(BatchEntity { entity, empty_since: Some(frame) });
        world.resource_mut::<SceneUploads>().zero_fills.push(BatchZeroFill {
            space: config.space,
            batch,
            entity,
            slots: capacity,
        });
        spawned.push(batch);
    }
    spawned
}

/// A batch's range list as the port takes it (B3 brief §4.3 step 8) and the
/// batch's own material (its first range's chain's, the chain whose mesh the
/// batch's `mesh` is), or `None` if a range names a chain level that is not
/// resident (a bug). A range's `material` is `None` when its chain's material
/// is the batch's, its id otherwise: the batch key put both in one bind group,
/// which is all the port requires (`gpu_instance_batch` `mod.rs:808-813`).
fn map_ranges(
    state: &SpaceState,
    batch: BatchId,
) -> Option<(Vec<GpuBatchedMeshRange>, Option<Handle<StandardMaterial>>)> {
    let specs = state.space.batch_ranges(batch);
    let own = match specs.first() {
        Some(r) => state.chain(r.chain)?.material.clone(),
        None => None,
    };
    let own_id = own.as_ref().map(Handle::id);
    let ranges = specs
        .into_iter()
        .map(|r| {
            let chain = state.chain(r.chain)?;
            let mesh = chain.levels.get(usize::from(r.level))?.clone();
            let material = chain.material_id().filter(|&m| Some(m) != own_id).map(AssetId::untyped);
            Some(GpuBatchedMeshRange {
                mesh,
                first_index: 0,
                index_count: 0,
                first_slot: r.first_slot,
                slot_count: r.slot_count,
                material,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some((ranges, own))
}

/// Rebuilds the range list of every batch of `batches` that has an entity from
/// `SceneSpace::batch_ranges`, re-points its `mesh` at the first range's mesh
/// (B3 brief §3 item 12) and its `MeshMaterial3d` at that range's chain's
/// material, and stamps the frame a list went empty.
fn rebuild_ranges(world: &mut World, state: &mut SpaceState, batches: impl IntoIterator<Item = BatchId>) {
    let frame = world.resource::<SceneFrame>().0;
    for batch in batches {
        let Some(Some(slot)) = state.batches.get(batch as usize).copied() else { continue };
        let (ranges, own) = map_ranges(state, batch).unwrap_or_else(|| {
            log::error!("sf_scene_bevy: batch {batch} names a chain level that is not resident; it draws nothing");
            (Vec::new(), None)
        });
        // An empty list keeps the last material, as it keeps the last mesh.
        if let Some(own) = own
            && let Some(mut material) = world.get_mut::<MeshMaterial3d<StandardMaterial>>(slot.entity)
            && material.0 != own
        {
            material.0 = own;
        }
        let Some(mut mesh) = world.get_mut::<GpuBatchedMesh3d>(slot.entity) else {
            debug_assert!(false, "batch {batch}'s entity {} lost its GpuBatchedMesh3d", slot.entity);
            continue;
        };
        // An empty list keeps the last mesh: every slot of such a batch is inactive.
        if let Some(first) = ranges.first() {
            mesh.mesh = first.mesh.clone();
        }
        let empty_since = if ranges.is_empty() { Some(slot.empty_since.unwrap_or(frame)) } else { None };
        mesh.ranges = ranges;
        state.batches[batch as usize] = Some(BatchEntity { empty_since, ..slot });
    }
}

pub(crate) fn count(counters: Option<&Counters>, name: &str, value: f64) {
    if let Some(c) = counters {
        c.add_named(name, unit::COUNT, value);
    }
}

/// What every cell's step reads.
struct Cx<'a> {
    asset_server: &'a AssetServer,
    tables: &'a Assets<SceneTableAsset>,
    products: Option<&'a Assets<CookedModelAsset>>,
    frame: u64,
    counters: Option<&'a Counters>,
}

/// `Update`, after `StreamingSystems::Drive`: picks up acquired cells and moves
/// every undecided one through the gate, planning it when it clears (§4.3
/// steps 1-4). Several phases may pass in one frame.
#[allow(clippy::too_many_arguments)] // one system, thirteen resources
pub fn poll_cell_loads(
    asset_server: Res<AssetServer>,
    tables: Res<Assets<SceneTableAsset>>,
    products: Option<Res<Assets<CookedModelAsset>>>,
    config: Res<SceneConfig>,
    frame: Res<SceneFrame>,
    inbox: Res<SceneInbox>,
    mut spaces: ResMut<SceneSpaces>,
    mut slabs: ResMut<ChainSlabTable>,
    counters: Option<Res<DiagCounters>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut chain_materials: ResMut<ChainMaterials>,
    mapper: Option<Res<ModelMaterials>>,
    streaming: Option<Res<Streaming>>,
) {
    let mapper = mapper.as_deref().cloned().unwrap_or_default();
    // `build_template`'s switch: the app's `--off matclass` reaches both paths.
    let fold = streaming.as_deref().is_none_or(|s| s.config().material_classes);
    let mut maker = materials.as_deref_mut().map(|materials| MaterialMaker {
        materials,
        cache: &mut chain_materials,
        mapper: &mapper,
        fold,
    });
    let acquired = std::mem::take(&mut *inbox.lock());
    for (worldspace, cell, table) in acquired {
        let next = SpaceId::try_from(spaces.0.len()).unwrap_or(SpaceId::MAX);
        let state =
            spaces.0.entry(worldspace).or_insert_with(|| SpaceState::new(SpaceConfig { space: next, ..config.space }));
        // A second acquire of a cell still loading keeps the first load.
        state.loading.entry(cell).or_insert(CellLoad { table, phase: LoadPhase::Table });
    }
    let cx = Cx {
        asset_server: &asset_server,
        tables: &tables,
        products: products.as_deref(),
        frame: frame.0,
        counters: counters.as_deref().map(|c| &*c.0),
    };
    let slabs = &mut *slabs;
    for (worldspace, state) in &mut spaces.0 {
        let undecided: Vec<IVec3> =
            state.loading.iter().filter(|(_, l)| !l.phase.decided()).map(|(&c, _)| c).collect();
        for cell in undecided {
            step(&cx, worldspace, cell, state, slabs, maker.as_mut());
        }
    }
    if slabs.dirty {
        let mut wanted: HashSet<AssetId<Mesh>> = HashSet::new();
        let mut wanted_materials: HashSet<AssetId<StandardMaterial>> = HashSet::new();
        let mut want = |a: &ChainAssets| {
            wanted.extend(a.level_ids());
            wanted_materials.extend(a.material_id());
        };
        for state in spaces.0.values() {
            state.chains.iter().flatten().for_each(&mut want);
            for load in state.loading.values() {
                if let LoadPhase::Slabs { chains, .. } = &load.phase {
                    for (_, a) in chains {
                        want(a);
                    }
                }
            }
        }
        // A material no chain holds any more is dropped with its last chain.
        chain_materials.0.retain(|_, h| wanted_materials.contains(&h.id()));
        // Drops the answers no longer wanted and marks new ids pending (`slabs` module docs).
        slabs.set_wanted(wanted, wanted_materials);
    }
}

/// Advances one cell as far as it can go this frame.
fn step(
    cx: &Cx<'_>,
    worldspace: &WorldspaceId,
    cell: IVec3,
    state: &mut SpaceState,
    slabs: &mut ChainSlabTable,
    mut maker: Option<&mut MaterialMaker<'_>>,
) {
    let Some(load) = state.loading.get_mut(&cell) else { return };
    loop {
        let next = match &load.phase {
            LoadPhase::Table => match table_step(cx, worldspace, cell, &load.table) {
                Some(next) => next,
                None => return,
            },
            LoadPhase::Chains { products } => match chains_step(cx, worldspace, cell, products, maker.as_deref_mut()) {
                Some(next) => {
                    slabs.dirty = true;
                    next
                }
                None => return,
            },
            LoadPhase::Slabs { chains, since } => {
                let resolved = chains.iter().all(|(_, a)| slabs.resolves(a));
                if slabs.tracking && !resolved {
                    if cx.frame.saturating_sub(*since) < SLAB_WAIT_FRAMES {
                        return;
                    }
                    log::warn!(
                        "sf_scene_bevy: cell {cell} of {worldspace}: the render world has not allocated every chain mesh and prepared every chain material after {SLAB_WAIT_FRAMES} frames; planning without them (their chains batch alone)"
                    );
                }
                slabs.dirty = true;
                let LoadPhase::Slabs { chains, .. } = std::mem::replace(&mut load.phase, LoadPhase::Fallback) else {
                    unreachable!("matched above")
                };
                let table = load.table.clone();
                let phase = plan(cx, worldspace, cell, &table, &chains, state, slabs);
                if let Some(load) = state.loading.get_mut(&cell) {
                    load.phase = phase;
                }
                return;
            }
            LoadPhase::Planned { .. } | LoadPhase::Fallback => return,
        };
        load.phase = next;
    }
}

fn is_not_found(e: &AssetLoadError) -> bool {
    matches!(e, AssetLoadError::AssetReaderError(AssetReaderError::NotFound(_)))
}

/// Gate 1: the table. `None`: still loading.
fn table_step(cx: &Cx<'_>, worldspace: &WorldspaceId, cell: IVec3, table: &Handle<SceneTableAsset>) -> Option<LoadPhase> {
    // An asset in `Assets` is resolved whatever the server knows of it (one added by hand has no
    // load state at all).
    let state = if cx.tables.contains(table.id()) { LoadState::Loaded } else { cx.asset_server.get_load_state(table.id())? };
    match state {
        LoadState::Loaded => {}
        // §4.4 rows 1-2: no product (not baked, a mod added the cell), or a table the loader
        // rejected. Both end at the entity path; only the corrupt one is worth an error.
        LoadState::Failed(e) if is_not_found(&e) => {
            log::debug!("sf_scene_bevy: cell {cell} of {worldspace} has no scene table; its statics spawn as entities");
            count(cx.counters, "scene/cells/missing", 1.0);
            return Some(LoadPhase::Fallback);
        }
        LoadState::Failed(e) => {
            log::error!("sf_scene_bevy: the scene table of cell {cell} of {worldspace} failed to load: {e}");
            count(cx.counters, "scene/cells/failed", 1.0);
            return Some(LoadPhase::Fallback);
        }
        LoadState::NotLoaded | LoadState::Loading => return None,
    }
    let entries = cx.tables.get(table).and_then(|asset| {
        let view = asset.view().ok()?;
        let n = u32::try_from(view.chain_count()).ok()?;
        (0..n).map(|i| ChainEntry::from_table(&view, i)).collect::<Option<Vec<_>>>()
    });
    let Some(entries) = entries else {
        log::error!("sf_scene_bevy: the scene table of cell {cell} of {worldspace} loaded but cannot be read");
        count(cx.counters, "scene/cells/failed", 1.0);
        return Some(LoadPhase::Fallback);
    };
    // Every chain of the table, resident or not: the asset server hands back the live handle of
    // a product another cell already loaded, so a shared chain resolves at once.
    let products = entries.into_iter().map(|e| {
        let handle = load_product(cx.asset_server, &e);
        (e, handle)
    });
    Some(LoadPhase::Chains { products: products.collect() })
}

/// Gate 2: the chain products. `None`: still loading.
fn chains_step(
    cx: &Cx<'_>,
    worldspace: &WorldspaceId,
    cell: IVec3,
    products: &[(ChainEntry, Handle<CookedModelAsset>)],
    mut maker: Option<&mut MaterialMaker<'_>>,
) -> Option<LoadPhase> {
    let chain_failed = |entry: &ChainEntry, why: &str| {
        log::error!(
            "sf_scene_bevy: chain {:#018x} ({}) of cell {cell} of {worldspace}: {why}; the whole cell spawns as entities",
            entry.key,
            entry.source.subject()
        );
        count(cx.counters, "scene/chains/failed", 1.0);
        Some(LoadPhase::Fallback)
    };
    let mut pending = false;
    for (entry, handle) in products {
        match product_state(cx.asset_server, handle) {
            ProductState::Pending => pending = true,
            ProductState::Failed(e) => return chain_failed(entry, &format!("its product failed to load: {e}")),
            ProductState::Loaded => {}
        }
    }
    let assets = cx.products?;
    if pending {
        return None;
    }
    let mut chains = Vec::with_capacity(products.len());
    for (entry, handle) in products {
        match chain_assets(assets, handle, entry, maker.as_deref_mut(), cx.asset_server) {
            Ok(a) => chains.push((entry.clone(), a)),
            Err(why) => return chain_failed(entry, &format!("its product disagrees with the table: {why}")),
        }
    }
    Some(LoadPhase::Slabs { chains, since: cx.frame })
}

/// `plan_in`, through the render world's slab answers (§4.3 step 4). Planned,
/// not committed: `upload` commits.
fn plan(
    cx: &Cx<'_>,
    worldspace: &WorldspaceId,
    cell: IVec3,
    table: &Handle<SceneTableAsset>,
    chains: &[(ChainEntry, ChainAssets)],
    state: &mut SpaceState,
    slabs: &ChainSlabTable,
) -> LoadPhase {
    let Some(view) = cx.tables.get(table).and_then(|a| a.view().ok()) else {
        log::error!("sf_scene_bevy: the scene table of cell {cell} of {worldspace} went away before planning");
        count(cx.counters, "scene/cells/failed", 1.0);
        return LoadPhase::Fallback;
    };
    let ids: HashMap<ChainKey, ChainIds> = chains.iter().map(|(e, a)| (e.key, ChainIds::from(a))).collect();
    let resolver = CellSlabs { chains: &ids, table: slabs };
    match state.space.plan_in(cell, &view, &resolver) {
        Ok(plan) => {
            // `chain_slots` is in the table's chain order, as `chains` is.
            for (&slot, (_, assets)) in plan.chain_slots.iter().zip(chains) {
                state.store_chain(slot, assets);
            }
            if !plan.overflowed.is_empty() {
                let left = plan.batch_slots.iter().filter(|&&s| s == NO_SLOT).count();
                log::debug!("sf_scene_bevy: cell {cell} of {worldspace}: {left} instances overflowed their batch; their placements spawn as entities");
                #[allow(clippy::cast_precision_loss)] // a count, far below 2^52
                count(cx.counters, "scene/slots/overflow", left as f64);
                count(cx.counters, "scene/overflow/cells", 1.0);
            }
            LoadPhase::Planned { plan: Box::new(plan) }
        }
        Err(e) => {
            log::error!("sf_scene_bevy: cell {cell} of {worldspace} cannot stream in: {e}; its statics spawn as entities");
            let name = if matches!(e, SceneError::Chain(_)) { "scene/chains/failed" } else { "scene/cells/failed" };
            count(cx.counters, name, 1.0);
            LoadPhase::Fallback
        }
    }
}

/// A committed cell's `plan_writes` resolved for the render world (B4a): each
/// batch's entity, and the instance's table row placed in render space, the
/// frame of the floating origin's cell `frame` (`sf_world_bevy::WorldOrigin`:
/// `GlobalTransform` is relative to it). A write with no batch entity or no
/// table row is a bug, dropped with an error. An origin shift re-queues every
/// resident cell through here in the new frame (`shift` module).
pub(crate) fn queue_slot_writes(
    tables: &Assets<SceneTableAsset>,
    state: &SpaceState,
    cell: IVec3,
    plan_writes: &[SlotWrite],
    frame: IVec3,
) -> Vec<BatchSlotWrite> {
    let view = state.tables.get(&cell).and_then(|t| tables.get(t)).and_then(|a| a.view().ok());
    let Some(view) = view else {
        log::error!("sf_scene_bevy: cell {cell}'s table went away before its batch slots were written");
        return Vec::new();
    };
    let offset = sf_scene::cell_offset(view.origin_cell(), WorldPos::new(frame, Vec3::ZERO));
    let mut out = Vec::with_capacity(plan_writes.len());
    for w in plan_writes {
        let entity = state.batches.get(w.batch as usize).copied().flatten().map(|b| b.entity);
        let (Some(entity), Some(row)) = (entity, view.instance(w.index as usize)) else {
            log::error!("sf_scene_bevy: cell {cell}: slot write {w:?} has no batch entity or table row; not written");
            continue;
        };
        out.push(BatchSlotWrite {
            entity: MainEntity::from(entity),
            batch: w.batch,
            slot: w.slot,
            instance: gpu_instance(row.transform, offset, w.active),
        });
    }
    out
}

/// Gameplay's request that the scene stop drawing one placement (scene-runtime-plan.md §7): its
/// slots stay allocated and are written inactive from the next publish. The scene never spawns;
/// the sender spawns (and reveals) the placement's entity first, so there is no pop. One way:
/// the promotion lasts until the cell streams out.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct PromotePlacement {
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
    /// The global id (`sf_world::PlacementId`), not the table's cell-local one.
    pub placement: PlacementId,
}

/// `Update`, in `SceneSystems::Promote`: each [`PromotePlacement`] maps its placement to the
/// table's local id ([`SpaceState::covered`]), promotes it (`SceneSpace::promote`) and queues
/// one inactive [`BatchSlotWrite`] per slot in [`SceneUploads::writes`]. An unknown worldspace,
/// a cell not resident, a placement the cell does not draw (overflowed, failed chain, not a
/// static) or one already promoted queues nothing.
pub fn apply_promotions(
    mut promotions: MessageReader<PromotePlacement>,
    mut spaces: ResMut<SceneSpaces>,
    tables: Res<Assets<SceneTableAsset>>,
    origin: Option<Res<WorldOrigin>>,
    mut uploads: ResMut<SceneUploads>,
) {
    let frame = origin.map_or(IVec3::ZERO, |o| o.cell);
    for m in promotions.read() {
        let Some(state) = spaces.0.get_mut(&m.worldspace) else {
            log::debug!("sf_scene_bevy: promotion of {:?} in {}: no scene there", m.placement, m.worldspace);
            continue;
        };
        let Some(&local) = state.covered.get(&m.cell).and_then(|c| c.get(&m.placement)) else {
            log::debug!(
                "sf_scene_bevy: promotion of {:?} in cell {} of {}: the cell is not resident or does not draw it",
                m.placement,
                m.cell,
                m.worldspace
            );
            continue;
        };
        let Some(view) = state.tables.get(&m.cell).and_then(|t| tables.get(t)).and_then(|a| a.view().ok()) else {
            log::error!("sf_scene_bevy: cell {}'s table went away while it was resident; promotion dropped", m.cell);
            continue;
        };
        let writes = match state.space.promote(m.cell, local, &view) {
            Ok(writes) => writes,
            Err(e) => {
                log::debug!("sf_scene_bevy: promotion of {:?} in cell {}: {e}", m.placement, m.cell);
                continue;
            }
        };
        if !writes.is_empty() {
            uploads.writes.extend(queue_slot_writes(&tables, state, m.cell, &writes, frame));
        }
    }
}

/// Whether the world overlay removed `id` or placed it anywhere (a mod or a save moved it): the
/// scene does not draw such a placement and streaming handles it like any other (a moved one
/// spawns from the overlay-applied list, a removed one spawns nothing). The precombine's test
/// (`sf_world_bevy` `valid_members`). No [`WorldOverlay`] resource: nothing is touched.
fn overlay_touched(world: &World, id: PlacementId) -> bool {
    world.get_resource::<WorldOverlay>().is_some_and(|o| {
        let o = o.0.read().unwrap_or_else(PoisonError::into_inner);
        o.is_removed(id) || o.placed(id).is_some()
    })
}

/// The static scene's streaming hook, installed by `ScenePlugin` as
/// `SceneCells` when the scene is enabled.
#[derive(Default)]
pub struct SceneHook;

/// Whether the scene draws `worldspace`: `Some(false)` when [`SceneConfig`] does not admit it or
/// the adapter has no GPU culling ([`SceneSupport::Unsupported`], plan §7: the scene is then
/// `--off scene`), `None` while the render world has not answered ([`SceneSupport::Unknown`]:
/// wait), `Some(true)` otherwise. No [`SceneSupport`] (no render app): not gated.
fn scene_on(world: &World, worldspace: &WorldspaceId) -> Option<bool> {
    if !world.get_resource::<SceneConfig>().is_some_and(|c| c.admits(worldspace)) {
        return Some(false);
    }
    match world.get_resource::<SceneSupport>() {
        None | Some(SceneSupport::Supported) => Some(true),
        Some(SceneSupport::Unsupported) => Some(false),
        Some(SceneSupport::Unknown) => None,
    }
}

impl CellScene for SceneHook {
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle> {
        // Unknown proceeds: the table loads while the adapter answers (within a frame).
        if scene_on(world, worldspace) == Some(false) {
            return None;
        }
        let inbox = world.get_resource::<SceneInbox>()?;
        let path = baked_path(CELLS_STEP, &cell_subject(worldspace.as_str(), cell), Some(CELLS_EXT));
        let handle: Handle<SceneTableAsset> = world.resource::<AssetServer>().load(path);
        inbox.lock().push((worldspace.clone(), cell, handle.clone()));
        // Held in the fetch's work, so the fetch waits for the table (gate 1).
        Some(handle.untyped())
    }

    fn ready(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> bool {
        match scene_on(world, worldspace) {
            Some(false) => return true,
            None => return false,
            Some(true) => {}
        }
        if world.get_resource::<SceneInbox>().is_some_and(|i| i.contains(worldspace, cell)) {
            return false;
        }
        // Not loading: committed, released, or never acquired — nothing to wait for.
        world
            .get_resource::<SceneSpaces>()
            .and_then(|s| s.0.get(worldspace))
            .and_then(|s| s.loading.get(&cell))
            .is_none_or(|l| l.phase.decided())
    }

    fn covered(
        &self,
        world: &World,
        worldspace: &WorldspaceId,
        cell: IVec3,
        indexed: &[PlacementId],
    ) -> Option<Arc<HashSet<PlacementId>>> {
        if scene_on(world, worldspace) == Some(false) {
            return None;
        }
        let load = world.get_resource::<SceneSpaces>()?.0.get(worldspace)?.loading.get(&cell)?;
        let LoadPhase::Planned { plan } = &load.phase else { return None };
        let mut out = HashSet::with_capacity(plan.covered.len());
        let mut stale = 0usize;
        for &local in &plan.covered {
            match indexed.get(local as usize) {
                // Moved or removed by the overlay: streaming's, not the scene's (B5b).
                Some(&id) if overlay_touched(world, id) => {}
                Some(&id) => {
                    out.insert(id);
                }
                None => stale += 1,
            }
        }
        if stale > 0 {
            log::error!(
                "sf_scene_bevy: cell {cell} of {worldspace}: {stale} table placements are past the cell's {} indexed placements (a stale scene.cells product)",
                indexed.len()
            );
        }
        Some(Arc::new(out))
    }

    /// Commits the cell's plan (`SceneSpace::commit_in`), makes sure its
    /// batches have their entities (a new one queues its zero-fill) and
    /// rebuilds `plan.ranges`, then queues `plan.writes` in [`SceneUploads`]
    /// ([`queue_slot_writes`]), followed by the inactive writes of each covered
    /// placement the overlay moved or removed ([`overlay_touched`]). Returns the bytes it will upload: the
    /// scene-buffer copy plus 64 B per batch slot written.
    ///
    /// TODO(B4b): push `plan.scene_bytes` into [`SceneUploads`].
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> u64 {
        // A cell planned while the adapter was Unknown that then turned out Unsupported: `covered`
        // said `None` (entities spawn), so it must not commit a batch too; `release` drops the plan.
        if scene_on(world, cx.worldspace) != Some(true) || !world.contains_resource::<SceneSpaces>() {
            return 0;
        }
        world.resource_scope(|world, mut spaces: Mut<SceneSpaces>| {
            let Some(state) = spaces.0.get_mut(cx.worldspace) else { return 0 };
            if !matches!(state.loading.get(&cx.cell), Some(CellLoad { phase: LoadPhase::Planned { .. }, .. })) {
                return 0;
            }
            let Some(CellLoad { table, phase: LoadPhase::Planned { plan } }) = state.loading.remove(&cx.cell) else {
                unreachable!("checked above")
            };
            state.space.commit_in(cx.cell, &plan);
            // B5b: what the overlay moved or removed leaves the scene: its planned slots are
            // written inactive after the plan's own writes (a later write to a slot wins) and it
            // stays out of the promotion map. The overlay is re-read here, not only in `covered`:
            // this also catches a removal that landed between the two (streaming then spawns
            // nothing for it, and the scene must not draw it either).
            let mut slot_writes = plan.writes.clone();
            let mut covered = HashMap::with_capacity(plan.covered.len());
            {
                let view = world.resource::<Assets<SceneTableAsset>>().get(&table).and_then(|a| a.view().ok());
                for &local in &plan.covered {
                    // A local past `indexed` was logged in `covered`.
                    let Some(&id) = cx.indexed.get(local as usize) else { continue };
                    if !overlay_touched(world, id) {
                        covered.insert(id, local);
                        continue;
                    }
                    let Some(view) = &view else {
                        log::error!("sf_scene_bevy: cell {}'s table went away at upload; {id:?} stays drawn", cx.cell);
                        continue;
                    };
                    match state.space.promote(cx.cell, local, view) {
                        Ok(writes) => slot_writes.extend(writes),
                        Err(e) => log::error!("sf_scene_bevy: cell {}: {id:?} cannot leave the scene: {e}", cx.cell),
                    }
                }
            }
            state.tables.insert(cx.cell, table);
            // The promotion map: `covered`'s answer, kept.
            state.covered.insert(cx.cell, covered);
            // Once per material class (and once for the single-chain batches) for the run.
            while let Some(key) = state.space.take_overflow_warning() {
                log::warn!(
                    "sf_scene_bevy: batch {key:?} of {} is full ({} slots): what overflows spawns as entities (scene/batch/{}/overflow counts it; this warns once per class)",
                    cx.worldspace,
                    state.space.config().capacity_of_key(&key),
                    crate::gauges::group_name(&key)
                );
            }
            // Spawned with no range first (step 5), then every changed list rebuilt (step 8).
            let mut changed = ensure_batches(world, state, &plan.batches);
            changed.extend_from_slice(&plan.ranges);
            changed.sort_unstable();
            changed.dedup();
            rebuild_ranges(world, state, changed);
            // After `ensure_batches`: every batch of the plan has its entity now.
            let frame = world.get_resource::<WorldOrigin>().map_or(IVec3::ZERO, |o| o.cell);
            let writes = queue_slot_writes(world.resource::<Assets<SceneTableAsset>>(), state, cx.cell, &slot_writes, frame);
            world.resource_mut::<SceneUploads>().writes.extend(writes);
            (plan.scene_bytes.len() as u64).saturating_add(slot_writes.len() as u64 * SLOT_BYTES)
        })
    }

    /// Idempotent: a planned cell gives its plan back (`cancel_in`), a resident
    /// one streams out (`plan_out`), a fallen-back or unknown one only drops
    /// its handles. The batches whose lists changed are rebuilt; one left
    /// empty is despawned by [`despawn_empty_batches`] after the frames in
    /// flight.
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3) {
        if let Some(inbox) = world.get_resource::<SceneInbox>() {
            inbox.lock().retain(|(w, c, _)| !(w == worldspace && *c == cell));
        }
        if !world.contains_resource::<SceneSpaces>() {
            return;
        }
        let changed = world.resource_scope(|world, mut spaces: Mut<SceneSpaces>| {
            let Some(state) = spaces.0.get_mut(worldspace) else { return false };
            if let Some(load) = state.loading.remove(&cell) {
                if let LoadPhase::Planned { plan } = load.phase {
                    state.space.cancel_in(&plan);
                    // A batch another cell committed meanwhile listed this plan's blocks.
                    rebuild_ranges(world, state, plan.ranges.iter().copied());
                    state.drop_released(&plan.chain_slots);
                }
                true
            } else if state.space.resident(cell).is_some() {
                match state.space.plan_out(cell) {
                    Ok(released) => {
                        rebuild_ranges(world, state, released.ranges.iter().copied());
                        for slot in released.freed_chains {
                            if let Some(entry) = state.chains.get_mut(slot as usize) {
                                *entry = None;
                            }
                        }
                    }
                    Err(e) => debug_assert!(false, "streaming out resident cell {cell}: {e}"),
                }
                state.tables.remove(&cell);
                state.covered.remove(&cell);
                true
            } else {
                false
            }
        });
        if changed && let Some(mut slabs) = world.get_resource_mut::<ChainSlabTable>() {
            slabs.dirty = true;
        }
    }

    /// Streaming spawned the placement's entity (`sf_world_bevy::PromoteToEntity`): queue the
    /// [`PromotePlacement`] that [`apply_promotions`] turns into inactive slot writes.
    fn promoted(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3, placement: PlacementId) {
        world.write_message(PromotePlacement { worldspace: worldspace.clone(), cell, placement });
    }
}
