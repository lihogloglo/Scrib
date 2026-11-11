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
//! 2. [`LoadPhase::Chains`]: every chain product it names, with its meshes.
//! 3. [`LoadPhase::Slabs`]: every chain level mesh is in [`ChainSlabTable`],
//!    i.e. the render world allocated it (skipped without a render world, and
//!    bounded by [`SLAB_WAIT_FRAMES`]).
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
use bevy_math::IVec3;
use bevy_mesh::Mesh;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::{Counters, unit};
use sf_model_bevy::{CookedModelAsset, baked_path};
use sf_scene::{
    CELLS_EXT, CELLS_STEP, CellPlan, ChainEntry, ChainKey, ChainSlot, NO_SLOT, SceneError, SceneSpace, SlotRange,
    SlotWrite, SpaceConfig, SpaceId, cell_subject,
};
use sf_world::{PlacementId, WorldspaceId};
use sf_world_bevy::{CellScene, SceneCellCx};

use crate::chains::{ChainAssets, ProductState, chain_assets, load_product, product_state};
use crate::slabs::{CellSlabs, ChainSlabTable};
use crate::{SceneConfig, SceneTableAsset};

/// Frames a planned-to-be cell waits for its chain meshes' slabs before it
/// plans without them. A mesh the render world never allocates must not hold
/// a streaming fetch open for ever; the chains still unresolved then get a
/// batch of their own (correct, just more draws), with one `warn!`.
pub const SLAB_WAIT_FRAMES: u64 = 120;

/// Bytes per batch output slot written (`GpuMeshInstance`, B3 brief §1.3).
const SLOT_WRITE_BYTES: u64 = 64;

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
    /// Indexed by `BatchId` (dense, never reused). TODO(B3b): the batch
    /// entities; nothing spawns them yet.
    pub batches: Vec<Option<Entity>>,
    /// Cells acquired and not yet committed (or fallen back, until released).
    pub loading: HashMap<IVec3, CellLoad>,
    /// The table of every resident cell: `SceneSpace::promote` needs its view
    /// (B3 brief §3 item 16), so it lives as long as the cell.
    pub tables: HashMap<IVec3, Handle<SceneTableAsset>>,
}

impl SpaceState {
    pub fn new(config: SpaceConfig) -> Self {
        Self {
            space: SceneSpace::new(config),
            chains: Vec::new(),
            batches: Vec::new(),
            loading: HashMap::new(),
            tables: HashMap::new(),
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

/// GPU writes queued for the render world. TODO(B3b/B4): empty until then.
#[derive(Resource, Default)]
pub struct SceneUploads {
    pub cells: Vec<CellUpload>,
    pub writes: Vec<SlotWrite>,
}

/// `First`: the frame counter, and every space frees what its frames in flight are done with.
pub fn tick_frame(mut frame: ResMut<SceneFrame>, mut spaces: ResMut<SceneSpaces>) {
    frame.0 += 1;
    for state in spaces.0.values_mut() {
        state.space.advance(frame.0);
    }
}

fn count(counters: Option<&Counters>, name: &str, value: f64) {
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
#[allow(clippy::too_many_arguments)] // one system, eight resources
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
) {
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
            step(&cx, worldspace, cell, state, slabs);
        }
    }
    if slabs.dirty {
        let mut wanted: HashSet<AssetId<Mesh>> = HashSet::new();
        for state in spaces.0.values() {
            for assets in state.chains.iter().flatten() {
                wanted.extend(assets.level_ids());
            }
            for load in state.loading.values() {
                if let LoadPhase::Slabs { chains, .. } = &load.phase {
                    wanted.extend(chains.iter().flat_map(|(_, a)| a.level_ids()));
                }
            }
        }
        slabs.wanted = wanted;
        slabs.dirty = false;
    }
}

/// Advances one cell as far as it can go this frame.
fn step(cx: &Cx<'_>, worldspace: &WorldspaceId, cell: IVec3, state: &mut SpaceState, slabs: &mut ChainSlabTable) {
    let Some(load) = state.loading.get_mut(&cell) else { return };
    loop {
        let next = match &load.phase {
            LoadPhase::Table => match table_step(cx, worldspace, cell, &load.table) {
                Some(next) => next,
                None => return,
            },
            LoadPhase::Chains { products } => match chains_step(cx, worldspace, cell, products) {
                Some(next) => {
                    slabs.dirty = true;
                    next
                }
                None => return,
            },
            LoadPhase::Slabs { chains, since } => {
                let resolved = chains.iter().all(|(_, a)| slabs.has_all(a.level_ids()));
                if slabs.tracking && !resolved {
                    if cx.frame.saturating_sub(*since) < SLAB_WAIT_FRAMES {
                        return;
                    }
                    log::warn!(
                        "sf_scene_bevy: cell {cell} of {worldspace}: the render world has not allocated every chain mesh after {SLAB_WAIT_FRAMES} frames; planning without them (their chains batch alone)"
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
        match chain_assets(assets, handle, entry) {
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
    let levels: HashMap<ChainKey, Vec<AssetId<Mesh>>> =
        chains.iter().map(|(e, a)| (e.key, a.level_ids().collect())).collect();
    let resolver = CellSlabs { levels: &levels, table: slabs };
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

/// The static scene's streaming hook, installed by `ScenePlugin` as
/// `SceneCells` when the scene is enabled.
#[derive(Default)]
pub struct SceneHook;

impl CellScene for SceneHook {
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle> {
        if !world.get_resource::<SceneConfig>()?.admits(worldspace) {
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
        if !world.get_resource::<SceneConfig>().is_some_and(|c| c.admits(worldspace)) {
            return true;
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
        let load = world.get_resource::<SceneSpaces>()?.0.get(worldspace)?.loading.get(&cell)?;
        let LoadPhase::Planned { plan } = &load.phase else { return None };
        let mut out = HashSet::with_capacity(plan.covered.len());
        let mut stale = 0usize;
        for &local in &plan.covered {
            match indexed.get(local as usize) {
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

    /// Commits the cell's plan (`SceneSpace::commit_in`) and returns the bytes
    /// it will upload: the scene-buffer copy plus 64 B per batch slot written.
    ///
    /// **Queues nothing yet.** TODO(B3b/B4): push `plan.scene_bytes` and
    /// `plan.writes` into [`SceneUploads`], make sure the plan's batches exist
    /// (zero-filled before their first range) and rebuild `plan.ranges`.
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> u64 {
        let Some(mut spaces) = world.get_resource_mut::<SceneSpaces>() else { return 0 };
        let Some(state) = spaces.0.get_mut(cx.worldspace) else { return 0 };
        if !matches!(state.loading.get(&cx.cell), Some(CellLoad { phase: LoadPhase::Planned { .. }, .. })) {
            return 0;
        }
        let Some(CellLoad { table, phase: LoadPhase::Planned { plan } }) = state.loading.remove(&cx.cell) else {
            unreachable!("checked above")
        };
        state.space.commit_in(cx.cell, &plan);
        state.tables.insert(cx.cell, table);
        if let Some(key) = state.space.take_overflow_warning() {
            log::warn!(
                "sf_scene_bevy: batch {key:?} of {} is full ({} slots): what overflows spawns as entities (scene/slots/overflow counts it; this warns once)",
                cx.worldspace,
                state.space.config().batch_capacity
            );
        }
        (plan.scene_bytes.len() as u64).saturating_add(plan.writes.len() as u64 * SLOT_WRITE_BYTES)
    }

    /// Idempotent: a planned cell gives its plan back (`cancel_in`), a resident
    /// one streams out (`plan_out`), a fallen-back or unknown one only drops
    /// its handles.
    ///
    /// TODO(B3b): rebuild the ranges of `CellRelease::ranges` and despawn a
    /// batch entity empty for longer than the frames in flight.
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3) {
        if let Some(inbox) = world.get_resource::<SceneInbox>() {
            inbox.lock().retain(|(w, c, _)| !(w == worldspace && *c == cell));
        }
        let Some(mut spaces) = world.get_resource_mut::<SceneSpaces>() else { return };
        let Some(state) = spaces.0.get_mut(worldspace) else { return };
        let changed = if let Some(load) = state.loading.remove(&cell) {
            if let LoadPhase::Planned { plan } = load.phase {
                state.space.cancel_in(&plan);
                state.drop_released(&plan.chain_slots);
            }
            true
        } else if state.space.resident(cell).is_some() {
            match state.space.plan_out(cell) {
                Ok(released) => {
                    for slot in released.freed_chains {
                        if let Some(entry) = state.chains.get_mut(slot as usize) {
                            *entry = None;
                        }
                    }
                }
                Err(e) => debug_assert!(false, "streaming out resident cell {cell}: {e}"),
            }
            state.tables.remove(&cell);
            true
        } else {
            false
        };
        if changed && let Some(mut slabs) = world.get_resource_mut::<ChainSlabTable>() {
            slabs.dirty = true;
        }
    }
}
