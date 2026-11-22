//! The renderer's residency state (Bevy-free, so it is tested without an App): the batch plan, the two tile rings,
//! the scattered tiles' CPU copies, the batch slot pools, and the reconcile pass that turns all of it into slot
//! writes.
//!
//! - **Plan.** Mesh species are grouped by model key ([`Group`]); one group draws through two batches, one that
//!   casts shadows (its near-shadow species in shadow-ring tiles) and one that doesn't (everything else). A
//!   species' **batch slot** (the tag's bits 24-31) is its index in its group, the same in both batches, so the
//!   scatter can bake it in before any batch exists.
//! - **Tiles.** A scattered tile keeps its instances (36 B each) on the CPU: moving between the shadow and plain
//!   batch, a model that resolves late, and a floating-origin shift all rewrite from the copy, never re-scatter.
//! - **Reconcile.** Every frame each (tile, species) is placed in the batch it should be in; a move frees the old
//!   block (zero-fill of its live slots) and writes the new one in one run. An origin shift rewrites every placed
//!   block in place, in the shift frame (as `sf_scene_bevy::shift`).

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::sync::Arc;

use bevy_math::{DVec3, Mat3};
use sf_groundcover::{
    GroundSampler, Groundcover, Instance, Overrides, RenderKind, ScatterSettings, Scatterer, ShadowClass, SpeciesIndex,
    TileKey, TileStats,
};

use crate::ground::{GroundSource, LatticeGround};
use crate::pool::SlotPool;
use crate::ring::{Measure, RingSettings, TileRing, by_distance, max_ring_tiles, nearest_distance};
use crate::tunables as t;

/// Species one batch's material describes (`groundcover_common.wesl` `species`, a fixed uniform array).
// tunables: const (layout: the material uniform's species array length)
pub const MAX_SPECIES_PER_BATCH: usize = 16;

/// One GPU slot (`GpuMeshInstance`, 64 B) as little-endian words: the affine rows (12 f32), `is_active`, `tag`,
/// two pad words.
pub type SlotWords = [u32; 16];

/// Renderer settings (`groundcover.render.*` and the core's scatter tunables).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderSettings {
    pub scatter: ScatterSettings,
    pub mesh_range_m: f32,
    pub hysteresis_m: f32,
    pub shadow_range_m: f32,
    pub fade_width_m: f32,
    pub fade_shrink_m: f32,
    pub max_blocks_per_batch: u32,
    pub jobs_per_frame: u32,
    pub max_jobs_in_flight: u32,
    pub ground_cache_m: f32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            scatter: ScatterSettings::default(),
            mesh_range_m: t::MESH_RANGE.default_value(),
            hysteresis_m: t::HYSTERESIS.default_value(),
            shadow_range_m: t::SHADOW_RANGE.default_value(),
            fade_width_m: t::FADE_WIDTH.default_value(),
            fade_shrink_m: t::FADE_SHRINK.default_value(),
            max_blocks_per_batch: t::MAX_BLOCKS_PER_BATCH.default_value(),
            jobs_per_frame: t::JOBS_PER_FRAME.default_value(),
            max_jobs_in_flight: t::MAX_JOBS_IN_FLIGHT.default_value(),
            ground_cache_m: t::GROUND_CACHE_SPACING.default_value(),
        }
    }
}

impl RenderSettings {
    /// The shadow ring's range: the tunable, clamped so a casting tile (whole tile within range + hysteresis)
    /// never reaches the fade band, where casters would fade by the light's view instead of the camera's.
    pub fn effective_shadow_range(&self) -> f32 {
        self.shadow_range_m.min(self.mesh_range_m - self.fade_width_m - self.hysteresis_m).max(0.0)
    }

    pub fn ring(&self) -> RingSettings {
        RingSettings { tile_size_m: self.scatter.tile_size_m, range_m: self.mesh_range_m, hysteresis_m: self.hysteresis_m }
    }

    pub fn shadow_ring(&self) -> RingSettings {
        RingSettings {
            tile_size_m: self.scatter.tile_size_m,
            range_m: self.effective_shadow_range(),
            hysteresis_m: self.hysteresis_m,
        }
    }
}

/// The mesh species of one model key.
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub model: String,
    /// Species in batch-slot order (index = slot).
    pub species: Vec<SpeciesIndex>,
    /// Slots per tile block: the largest `min(cap, grid cells)` of the group's species.
    pub block_slots: u32,
    /// Tile blocks of the plain batch and of the casting one (0: the group never casts).
    pub blocks: [u32; 2],
}

/// Where one species draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpeciesPlan {
    pub group: usize,
    pub slot: u8,
    /// Near shadows (and a non-zero shadow range).
    pub casts: bool,
}

/// The batches a groundcover set draws through.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plan {
    pub groups: Vec<Group>,
    /// By species index; `None` for blade species and species past [`MAX_SPECIES_PER_BATCH`] in their group.
    pub species: Vec<Option<SpeciesPlan>>,
}

/// Batch index of (group, casting): `group * 2 + casting`.
pub const fn batch_index(group: usize, casting: bool) -> usize {
    group * 2 + casting as usize
}

impl Plan {
    pub fn new(cover: &Groundcover, settings: &RenderSettings) -> Self {
        let scatter = Scatterer::new(cover, &EMPTY, settings.scatter);
        let mut plan = Self { groups: Vec::new(), species: vec![None; cover.len()] };
        let casts_at_all = settings.effective_shadow_range() > 0.0;
        for s in cover.indices() {
            let sp = cover.species(s);
            let RenderKind::Mesh { model, .. } = &sp.render else { continue };
            let group = if let Some(g) = plan.groups.iter().position(|g| g.model == *model) {
                g
            } else {
                plan.groups.push(Group { model: model.clone(), species: Vec::new(), block_slots: 1, blocks: [0; 2] });
                plan.groups.len() - 1
            };
            let g = &mut plan.groups[group];
            if g.species.len() >= MAX_SPECIES_PER_BATCH {
                log::warn!(
                    "sf_groundcover_bevy: model {model:?} has more than {MAX_SPECIES_PER_BATCH} species; {:?} is not drawn",
                    sp.id
                );
                continue;
            }
            // Bounded by MAX_SPECIES_PER_BATCH.
            #[allow(clippy::cast_possible_truncation)]
            let slot = g.species.len() as u8;
            g.species.push(s);
            let n = scatter.grid(s).n;
            let ceiling = n.saturating_mul(n).min(settings.scatter.max_instances_per_tile).max(1);
            g.block_slots = g.block_slots.max(ceiling);
            plan.species[s.get()] = Some(SpeciesPlan { group, slot, casts: casts_at_all && sp.shadow == ShadowClass::Near });
        }
        let s = settings.scatter.tile_size_m;
        let all = max_ring_tiles(s, settings.mesh_range_m, settings.hysteresis_m);
        let near = max_ring_tiles(s, settings.effective_shadow_range(), settings.hysteresis_m);
        for (gi, g) in plan.groups.iter_mut().enumerate() {
            // Bounded by MAX_SPECIES_PER_BATCH.
            #[allow(clippy::cast_possible_truncation)]
            let (n_all, n_cast) = (
                g.species.len() as u32,
                plan.species.iter().flatten().filter(|p| p.group == gi && p.casts).count() as u32,
            );
            // Keep blocks × block_slots inside a u32.
            let cap = settings.max_blocks_per_batch.min(u32::MAX / g.block_slots).max(1);
            g.blocks = [all.saturating_mul(n_all).clamp(1, cap), near.saturating_mul(n_cast).min(cap)];
        }
        plan
    }

    pub fn batch_count(&self) -> usize {
        self.groups.len() * 2
    }

    /// (blocks, block slots) of batch `b`; 0 blocks = the batch is never used.
    pub fn batch_layout(&self, b: usize) -> (u32, u32) {
        let g = &self.groups[b / 2];
        (g.blocks[b % 2], g.block_slots)
    }
}

static EMPTY: std::sync::LazyLock<Overrides> = std::sync::LazyLock::new(Overrides::none);

/// What a scatter job needs, shared with the task.
#[derive(Clone)]
pub struct JobInputs {
    pub cover: Arc<Groundcover>,
    pub overrides: Arc<Overrides>,
    pub ground: Arc<dyn GroundSource>,
    pub settings: RenderSettings,
    /// (species, batch slot) to scatter, in species order.
    pub species: Arc<[(SpeciesIndex, u8)]>,
}

/// One scattered species of a tile.
#[derive(Clone, Debug, PartialEq)]
pub struct SpeciesTile {
    pub species: SpeciesIndex,
    pub instances: Vec<Instance>,
    pub stats: TileStats,
}

/// A finished scatter job.
#[derive(Clone, Debug, PartialEq)]
pub struct TileScatter {
    pub tile: TileKey,
    pub species: Vec<SpeciesTile>,
    /// Whether the ground cache was used.
    pub cached: bool,
}

/// The scatter job: every planned mesh species of `tile`, through the ground cache when it saves samples.
pub fn scatter_tile(inputs: &JobInputs, tile: TileKey) -> TileScatter {
    let scatter = Scatterer::new(&inputs.cover, &inputs.overrides, inputs.settings.scatter);
    let direct: u64 = inputs.species.iter().map(|&(s, _)| u64::from(scatter.grid(s).n).pow(2)).sum();
    let size = inputs.settings.scatter.tile_size_m;
    let spacing = inputs.settings.ground_cache_m;
    let cache = LatticeGround::worth_it(size, spacing, direct).then(|| LatticeGround::build(&*inputs.ground, tile, size, spacing));
    let ground: &dyn GroundSampler = match &cache {
        Some(c) => c,
        None => &*inputs.ground,
    };
    let species = inputs
        .species
        .iter()
        .map(|&(s, slot)| {
            let mut instances = Vec::new();
            let stats = scatter.scatter_tile(ground, tile, s, slot, &mut instances);
            SpeciesTile { species: s, instances, stats }
        })
        .collect();
    TileScatter { tile, species, cached: cache.is_some() }
}

/// A placed (tile, species) block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub batch: usize,
    pub block: u32,
}

/// A resident, scattered tile.
#[derive(Clone, Debug)]
pub struct TileData {
    pub scatter: TileScatter,
    /// Parallel to `scatter.species`.
    pub placed: Vec<Option<Placement>>,
}

/// One slot write run for the GPU: `slots` go to `first_slot..` of batch `batch` (zeros included).
#[derive(Clone, Debug, PartialEq)]
pub struct SlotWrite {
    pub batch: usize,
    pub first_slot: u32,
    pub slots: Vec<SlotWords>,
}

/// The slot of one instance placed at `origin`: rotation × scale in the rows, the f64 tile translation.
pub fn slot_words(inst: &Instance, tile: TileKey, tile_size_m: f32, origin: DVec3) -> SlotWords {
    let t = inst.translation_from(tile, tile_size_m, origin);
    let m = Mat3::from_quat(inst.rotation()) * inst.scale;
    let rows = [m.row(0), m.row(1), m.row(2)];
    let mut w = [0u32; 16];
    for (r, row) in rows.iter().enumerate() {
        w[r * 4] = row.x.to_bits();
        w[r * 4 + 1] = row.y.to_bits();
        w[r * 4 + 2] = row.z.to_bits();
        w[r * 4 + 3] = t[r].to_bits();
    }
    w[12] = 1;
    w[13] = inst.tag;
    w
}

/// Little-endian bytes of `slots` (64 B each, the `GpuMeshInstance` layout).
pub fn slot_bytes(slots: &[SlotWords]) -> Vec<u8> {
    slots.iter().flatten().flat_map(|w| w.to_le_bytes()).collect()
}

/// Counters of the last reconcile and the live state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResidencyStats {
    pub resident: u32,
    pub scattered: u32,
    pub casting: u32,
    /// (tile, species) scatters that hit the per-tile cap.
    pub capped: u32,
    /// (tile, species) blocks that found no free block (batch too small).
    pub unplaced: u32,
    /// Slots written by the last reconcile (zeros included).
    pub written_slots: u64,
}

/// Residency of one groundcover set (see the module docs).
pub struct Residency {
    pub plan: Plan,
    pub settings: RenderSettings,
    ring: TileRing,
    shadow: TileRing,
    tiles: BTreeMap<TileKey, TileData>,
    pools: Vec<Option<SlotPool>>,
    /// Origin of the last reconcile.
    origin: Option<DVec3>,
    writes: Vec<SlotWrite>,
    pub stats: ResidencyStats,
}

impl Residency {
    pub fn new(plan: Plan, settings: RenderSettings) -> Self {
        let pools = vec![None; plan.batch_count()];
        Self {
            plan,
            settings,
            ring: TileRing::default(),
            shadow: TileRing::default(),
            tiles: BTreeMap::new(),
            pools,
            origin: None,
            writes: Vec::new(),
            stats: ResidencyStats::default(),
        }
    }

    /// (species, batch slot) a job scatters: every planned species, in species order.
    pub fn job_species(&self) -> Arc<[(SpeciesIndex, u8)]> {
        self.plan
            .species
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                // Species indices fit a u16 (Groundcover::new).
                #[allow(clippy::cast_possible_truncation)]
                let s = SpeciesIndex(i as u16);
                p.map(|p| (s, p.slot))
            })
            .collect()
    }

    pub fn resident(&self) -> &BTreeSet<TileKey> {
        self.ring.members()
    }

    pub fn is_casting(&self, tile: TileKey) -> bool {
        self.shadow.contains(tile)
    }

    pub fn tile(&self, tile: TileKey) -> Option<&TileData> {
        self.tiles.get(&tile)
    }

    pub fn tiles(&self) -> impl Iterator<Item = (&TileKey, &TileData)> {
        self.tiles.iter()
    }

    pub fn pool(&self, batch: usize) -> Option<&SlotPool> {
        self.pools.get(batch).and_then(Option::as_ref)
    }

    /// Moves both rings to viewer `p` (world XZ). Tiles that left are freed (their writes queued). Returns the
    /// tiles that left the residency ring (pending jobs for them are stale).
    pub fn update_rings(&mut self, p: [f64; 2]) -> Vec<TileKey> {
        let delta = self.ring.update(p, &self.settings.ring(), Measure::Nearest);
        for k in &delta.left {
            if let Some(data) = self.tiles.remove(k) {
                for placement in data.placed.into_iter().flatten() {
                    self.release(placement);
                }
            }
        }
        let resident = self.ring.members().clone();
        self.shadow.update_among(p, &self.settings.shadow_ring(), Measure::Farthest, Some(&resident));
        delta.left
    }

    /// Up to `n` resident tiles still to scatter, not in `in_flight`, whose ground is `ready`: nearest first,
    /// ties by key (the deterministic job order).
    pub fn next_jobs(&self, p: [f64; 2], in_flight: &BTreeSet<TileKey>, ready: impl Fn(TileKey) -> bool, n: usize) -> Vec<TileKey> {
        let size = self.settings.scatter.tile_size_m;
        let mut todo: Vec<(f64, TileKey)> = self
            .ring
            .members()
            .iter()
            .filter(|k| !self.tiles.contains_key(k) && !in_flight.contains(k))
            .map(|&k| (nearest_distance(k, size, p), k))
            .collect();
        todo.sort_by(by_distance);
        todo.into_iter().map(|(_, k)| k).filter(|&k| ready(k)).take(n).collect()
    }

    /// A finished job. Dropped when its tile is no longer resident (or already scattered).
    pub fn insert(&mut self, scatter: TileScatter) -> bool {
        if !self.ring.contains(scatter.tile) || self.tiles.contains_key(&scatter.tile) {
            return false;
        }
        let placed = vec![None; scatter.species.len()];
        self.tiles.insert(scatter.tile, TileData { scatter, placed });
        true
    }

    /// Batch `b`'s model resolved: its pool exists from now on (sized by the plan). Returns its (blocks, slots).
    pub fn open_batch(&mut self, b: usize) -> Option<(u32, u32)> {
        let (blocks, slots) = self.plan.batch_layout(b);
        if blocks == 0 || self.pools[b].is_some() {
            return None;
        }
        self.pools[b] = Some(SlotPool::new(blocks, slots));
        Some((blocks, slots))
    }

    pub fn is_open(&self, b: usize) -> bool {
        self.pools.get(b).is_some_and(Option::is_some)
    }

    fn release(&mut self, p: Placement) {
        let Some(pool) = self.pools[p.batch].as_mut() else { return };
        let range = pool.free(p.block);
        push_zeros(&mut self.writes, p.batch, range);
    }

    /// Places every (tile, species) in the batch it belongs in at `origin` (render frame = world − origin) and
    /// returns the queued writes (frees first, in order). An origin change rewrites every placed block.
    pub fn reconcile(&mut self, origin: DVec3) -> Vec<SlotWrite> {
        let shift = self.origin.is_some_and(|o| o != origin);
        self.origin = Some(origin);
        let size = self.settings.scatter.tile_size_m;
        let mut stats = ResidencyStats { resident: u32_len(self.ring.len()), casting: u32_len(self.shadow.len()), ..Default::default() };
        for (&tile, data) in &mut self.tiles {
            stats.scattered += 1;
            let casting_tile = self.shadow.contains(tile);
            for (sp, placed) in data.scatter.species.iter().zip(&mut data.placed) {
                stats.capped += u32::from(sp.stats.capped);
                let Some(plan) = self.plan.species.get(sp.species.get()).copied().flatten() else { continue };
                let want = (!sp.instances.is_empty())
                    .then(|| batch_index(plan.group, casting_tile && plan.casts))
                    .filter(|&b| self.pools[b].is_some());
                let current = placed.map(|p| p.batch);
                if current == want {
                    if shift && let Some(p) = *placed {
                        write_block(&mut self.writes, &mut self.pools, p, &sp.instances, tile, size, origin);
                    }
                    continue;
                }
                if let Some(p) = placed.take() {
                    let pool = self.pools[p.batch].as_mut().expect("a placed block's pool exists");
                    let range = pool.free(p.block);
                    push_zeros(&mut self.writes, p.batch, range);
                }
                let Some(b) = want else { continue };
                let Some(block) = self.pools[b].as_mut().and_then(SlotPool::allocate) else {
                    stats.unplaced += 1;
                    continue;
                };
                let p = Placement { batch: b, block };
                write_block(&mut self.writes, &mut self.pools, p, &sp.instances, tile, size, origin);
                *placed = Some(p);
            }
        }
        let writes = std::mem::take(&mut self.writes);
        stats.written_slots = writes.iter().map(|w| w.slots.len() as u64).sum();
        self.stats = stats;
        writes
    }
}

fn u32_len(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

fn push_zeros(writes: &mut Vec<SlotWrite>, batch: usize, range: Range<u32>) {
    if !range.is_empty() {
        writes.push(SlotWrite { batch, first_slot: range.start, slots: vec![[0; 16]; range.len()] });
    }
}

/// One run for block `p`: the instances, then zeros over the block's previous live tail.
fn write_block(
    writes: &mut Vec<SlotWrite>,
    pools: &mut [Option<SlotPool>],
    p: Placement,
    instances: &[Instance],
    tile: TileKey,
    size: f32,
    origin: DVec3,
) {
    let pool = pools[p.batch].as_mut().expect("a placed block's pool exists");
    let count = u32::try_from(instances.len()).unwrap_or(u32::MAX);
    let range = pool.set_live(p.block, count);
    let mut slots: Vec<SlotWords> = instances.iter().take(range.len()).map(|i| slot_words(i, tile, size, origin)).collect();
    slots.resize(range.len(), [0; 16]);
    writes.push(SlotWrite { batch: p.batch, first_slot: range.start, slots });
}
