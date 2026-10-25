//! The world index: placements assigned to streaming cells, grouped in
//! pages, with the per-cell summary the scheduler reads.
//!
//! Every placement belongs to exactly one cell, chosen by its pivot. Each
//! cell keeps a *content AABB* (the union of its placements' model bounds),
//! which is what ring distances measure against, so large objects arrive in
//! time. Within a cell, placements are sorted by class, then by descending
//! bounds radius (big things first), then by id, so the order is the same
//! whatever order the source delivered them in.
//!
//! These are pure functions; the bake pipeline wraps them as steps
//! (`world.index` per page, plus the directory per worldspace).

use std::collections::{BTreeMap, BTreeSet};

use glam::{IVec3, Mat3, Vec3};
use serde::{Deserialize, Serialize};
use sf_vfs::VfsPath;

use crate::{Partition, Placement, PlacementFlags, PlacementId, WorldSource, WorldspaceId};

/// What the index needs to know about a model (from the bake's
/// `model.info`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModelInfo {
    /// Model-space bounds.
    pub min: Vec3,
    pub max: Vec3,
    /// Resident size estimate (CPU + GPU) of the model, for byte budgets.
    pub bytes: u64,
    /// Distance beyond which even the coarsest LOD isn't drawn (0 = no limit
    /// known).
    pub lod_distance: f32,
}

/// Per-placement overhead in the byte estimate (entity, components,
/// transforms).
pub const PLACEMENT_BYTES: u64 = 256;

/// An axis-aligned box. Empty when `min > max` on any axis.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub const EMPTY: Self = Self { min: Vec3::splat(f32::INFINITY), max: Vec3::splat(f32::NEG_INFINITY) };

    pub fn point(p: Vec3) -> Self {
        Self { min: p, max: p }
    }

    pub fn is_empty(&self) -> bool {
        self.min.cmpgt(self.max).any()
    }

    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self { min: self.min.min(other.min), max: self.max.max(other.max) }
    }

    #[must_use]
    pub fn translated(self, by: Vec3) -> Self {
        Self { min: self.min + by, max: self.max + by }
    }

    /// Whether `other` lies inside, with `eps` meters of slack for rounding.
    pub fn contains(&self, other: &Self, eps: f32) -> bool {
        other.is_empty() || ((other.min + eps).cmpge(self.min).all() && (other.max - eps).cmple(self.max).all())
    }
}

/// A placement in the index, with its precomputed bounds radius.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexedPlacement {
    pub placement: Placement,
    /// Half-diagonal of the scaled model bounds (0 without a model).
    pub radius: f32,
}

/// One streaming cell's content.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexCell {
    pub cell: IVec3,
    /// Content AABB, relative to the cell's origin (`Partition::cell_origin`).
    pub bounds: Aabb,
    /// Byte estimate of the far-ring subset and of everything.
    pub bytes_far: u64,
    pub bytes_all: u64,
    /// The largest coarsest-LOD distance of the cell's models.
    pub lod_distance: f32,
    pub placements: Vec<IndexedPlacement>,
}

impl IndexCell {
    /// Whether a placement belongs to the far ring (R2).
    pub fn is_far(p: &IndexedPlacement, far_min_radius: f32) -> bool {
        p.radius >= far_min_radius && !p.placement.flags.intersects(PlacementFlags::NEAR_ONLY | PlacementFlags::EDITOR_ONLY)
    }

    /// The far-ring subset, in index order.
    pub fn far(&self, far_min_radius: f32) -> impl Iterator<Item = &IndexedPlacement> {
        self.placements.iter().filter(move |p| Self::is_far(p, far_min_radius))
    }

    pub fn summary(&self, far_min_radius: f32) -> CellSummary {
        CellSummary {
            cell: self.cell,
            bounds: self.bounds,
            bytes_far: self.bytes_far,
            bytes_all: self.bytes_all,
            lod_distance: self.lod_distance,
            placements: u32::try_from(self.placements.len()).expect("cell placement count fits u32"),
            far_placements: u32::try_from(self.far(far_min_radius).count()).expect("fits u32"),
        }
    }
}

/// One page: up to 8×8×8 cells. The unit the runtime fetches and the bake
/// caches.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexPage {
    pub partition: Partition,
    pub page: IVec3,
    /// Sorted by cell key.
    pub cells: Vec<IndexCell>,
}

impl IndexPage {
    pub fn cell(&self, cell: IVec3) -> Option<&IndexCell> {
        self.cells.binary_search_by_key(&cell.to_array(), |c| c.cell.to_array()).ok().map(|i| &self.cells[i])
    }
}

/// A cell as the scheduler sees it: no placements, just where and how big.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CellSummary {
    pub cell: IVec3,
    /// Content AABB relative to the cell origin.
    pub bounds: Aabb,
    pub bytes_far: u64,
    pub bytes_all: u64,
    pub lod_distance: f32,
    pub placements: u32,
    pub far_placements: u32,
}

/// A page's slice of [`CellDirectory::cells`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageSummary {
    pub page: IVec3,
    pub first: u32,
    pub count: u32,
}

/// The always-resident part of a worldspace's index: every cell's summary,
/// sorted page-major (pages in key order, cells in key order within).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CellDirectory {
    pub worldspace: WorldspaceId,
    pub partition: Partition,
    pub far_min_radius: f32,
    pub cells: Vec<CellSummary>,
    pub pages: Vec<PageSummary>,
}

impl CellDirectory {
    /// The cells of one page.
    pub fn page_cells(&self, page: &PageSummary) -> &[CellSummary] {
        &self.cells[page.first as usize..(page.first + page.count) as usize]
    }

    /// Index of `cell` in [`CellDirectory::cells`].
    pub fn find(&self, cell: IVec3) -> Option<usize> {
        let page = self
            .pages
            .binary_search_by_key(&Partition::page_of(cell).to_array(), |p| p.page.to_array())
            .ok()
            .map(|i| self.pages[i])?;
        self.page_cells(&page)
            .binary_search_by_key(&cell.to_array(), |c| c.cell.to_array())
            .ok()
            .map(|i| page.first as usize + i)
    }
}

/// Index build settings (part of the bake key).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndexParams {
    pub partition: Partition,
    /// `RingConfig::far_min_radius`.
    pub far_min_radius: f32,
}

impl Default for IndexParams {
    fn default() -> Self {
        Self { partition: Partition::STREAMING, far_min_radius: 4.0 }
    }
}

/// What a build saw.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IndexStats {
    pub placements: usize,
    pub cells: usize,
    pub pages: usize,
    /// Models referenced without a `ModelInfo` (indexed as points).
    pub missing_models: Vec<VfsPath>,
    /// Ids delivered more than once (the source has a bug; all copies kept).
    pub duplicate_ids: Vec<PlacementId>,
}

/// A whole worldspace's index.
#[derive(Clone, Debug, PartialEq)]
pub struct WorldIndex {
    pub directory: CellDirectory,
    pub pages: Vec<IndexPage>,
    pub stats: IndexStats,
}

/// A placement's world bounds relative to its pivot, and its radius.
/// Without a model (or its info), a point at the pivot with radius 0.
pub fn placement_bounds(p: &Placement, info: Option<&ModelInfo>) -> (Aabb, f32) {
    let Some(info) = info else { return (Aabb::point(Vec3::ZERO), 0.0) };
    let t = &p.transform;
    let center = (info.min + info.max) * 0.5 * t.scale;
    let half = (info.max - info.min) * 0.5 * t.scale.abs();
    let rot = Mat3::from_quat(t.rotation);
    let abs = Mat3::from_cols(rot.x_axis.abs(), rot.y_axis.abs(), rot.z_axis.abs());
    let c = rot * center;
    let h = abs * half;
    let radius = ((info.max - info.min) * 0.5).length() * t.scale.abs().max_element();
    (Aabb { min: c - h, max: c + h }, radius)
}

/// A placement being indexed, with its bounds relative to the cell origin.
type Entry = (IndexedPlacement, Aabb, Option<ModelInfo>);

/// Page-major sort key of a cell.
fn cell_order(cell: IVec3) -> [i32; 6] {
    let p = Partition::page_of(cell);
    [p.x, p.y, p.z, cell.x, cell.y, cell.z]
}

/// Splits placements by the page their cell falls in, pages in key order,
/// placements in input order. Lets a bake cache per-source-region buckets.
pub fn bucket_by_page(partition: Partition, placements: impl IntoIterator<Item = Placement>) -> Vec<(IVec3, Vec<Placement>)> {
    let mut pages: BTreeMap<[i32; 3], Vec<Placement>> = BTreeMap::new();
    for p in placements {
        let page = Partition::page_of(partition.cell_of(p.transform.pos));
        pages.entry(page.to_array()).or_default().push(p);
    }
    pages.into_iter().map(|(k, v)| (IVec3::from_array(k), v)).collect()
}

/// Builds index pages from placements. Deterministic: the output doesn't
/// depend on input order (except among duplicate ids).
pub fn build_pages(
    params: &IndexParams,
    placements: impl IntoIterator<Item = Placement>,
    models: &dyn Fn(&VfsPath) -> Option<ModelInfo>,
) -> (Vec<IndexPage>, IndexStats) {
    let partition = params.partition;
    let mut stats = IndexStats::default();
    let mut missing = BTreeSet::new();
    let mut ids = Vec::new();
    // Cell (page-major key) → placements with their pivot-relative bounds.
    let mut cells: BTreeMap<[i32; 6], Vec<Entry>> = BTreeMap::new();
    for p in placements {
        let info = p.model.as_ref().and_then(|m| {
            let info = models(m);
            if info.is_none() {
                missing.insert(m.clone());
            }
            info
        });
        let cell = partition.cell_of(p.transform.pos);
        let pivot = partition.cell_origin(cell).delta_f32(p.transform.pos);
        let (bounds, radius) = placement_bounds(&p, info.as_ref());
        ids.push(p.id);
        stats.placements += 1;
        cells.entry(cell_order(cell)).or_default().push((
            IndexedPlacement { placement: p, radius },
            bounds.translated(pivot),
            info,
        ));
    }

    ids.sort_unstable();
    stats.duplicate_ids = ids.windows(2).filter(|w| w[0] == w[1]).map(|w| w[0]).collect();
    stats.duplicate_ids.dedup();
    stats.missing_models = missing.into_iter().collect();

    let mut pages: Vec<IndexPage> = Vec::new();
    for (key, mut entries) in cells {
        let cell = IVec3::new(key[3], key[4], key[5]);
        entries.sort_by(|(a, ..), (b, ..)| {
            a.placement
                .class
                .cmp(&b.placement.class)
                .then(b.radius.total_cmp(&a.radius))
                .then(a.placement.id.cmp(&b.placement.id))
        });
        let mut bounds = Aabb::EMPTY;
        let mut lod_distance = 0.0f32;
        let mut models_all = BTreeMap::new();
        let mut models_far = BTreeMap::new();
        let (mut n_all, mut n_far) = (0u64, 0u64);
        for (ip, b, info) in &entries {
            bounds = bounds.union(*b);
            let far = IndexCell::is_far(ip, params.far_min_radius);
            n_all += 1;
            n_far += u64::from(far);
            if let (Some(model), Some(info)) = (&ip.placement.model, info) {
                lod_distance = lod_distance.max(info.lod_distance);
                models_all.insert(model.clone(), info.bytes);
                if far {
                    models_far.insert(model.clone(), info.bytes);
                }
            }
        }
        let index_cell = IndexCell {
            cell,
            bounds,
            bytes_far: models_far.values().sum::<u64>() + n_far * PLACEMENT_BYTES,
            bytes_all: models_all.values().sum::<u64>() + n_all * PLACEMENT_BYTES,
            lod_distance,
            placements: entries.into_iter().map(|(ip, ..)| ip).collect(),
        };
        let page = Partition::page_of(cell);
        match pages.last_mut() {
            Some(last) if last.page == page => last.cells.push(index_cell),
            _ => pages.push(IndexPage { partition, page, cells: vec![index_cell] }),
        }
    }
    stats.pages = pages.len();
    stats.cells = pages.iter().map(|p| p.cells.len()).sum();
    (pages, stats)
}

/// The directory of a worldspace from all its pages (any order).
pub fn build_directory(worldspace: WorldspaceId, params: &IndexParams, pages: &[IndexPage]) -> CellDirectory {
    let mut sorted: Vec<&IndexPage> = pages.iter().collect();
    sorted.sort_by_key(|p| p.page.to_array());
    let mut cells = Vec::new();
    let mut summaries = Vec::with_capacity(sorted.len());
    for page in sorted {
        let first = u32::try_from(cells.len()).expect("cell count fits u32");
        cells.extend(page.cells.iter().map(|c| c.summary(params.far_min_radius)));
        summaries.push(PageSummary {
            page: page.page,
            first,
            count: u32::try_from(page.cells.len()).expect("fits u32"),
        });
    }
    CellDirectory { worldspace, partition: params.partition, far_min_radius: params.far_min_radius, cells, pages: summaries }
}

/// Indexes a whole worldspace from a source in one pass.
pub fn build_index(
    worldspace: &WorldspaceId,
    params: &IndexParams,
    source: &dyn WorldSource,
    models: &dyn Fn(&VfsPath) -> Option<ModelInfo>,
) -> WorldIndex {
    let mut regions = Vec::new();
    source.regions(worldspace, &mut |r, _| regions.push(r));
    let mut placements = Vec::new();
    for r in regions {
        source.placements(worldspace, r, &mut |p| placements.push(p));
    }
    let (pages, stats) = build_pages(params, placements, models);
    let directory = build_directory(worldspace.clone(), params, &pages);
    WorldIndex { directory, pages, stats }
}
