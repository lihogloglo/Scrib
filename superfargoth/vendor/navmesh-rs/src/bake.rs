//! Tiled baking extension, **not in recastnavigation**.
//!
//! [`tiles`] builds serially; [`tiles_parallel`] uses the current rayon pool when
//! `parallel` is enabled and otherwise uses the same serial implementation. Both
//! preserve input triangle order, return tiles in row-major (z, x) order, and
//! report the first failing tile in that order. No navmesh is mutated by workers.
//! [`BakedTiles::into_navmesh`] inserts the resulting tiles serially.
//!
//! The caller's workspace budgets validation, triangle bucketing and result-slot
//! allocation. [`BakeConfig::tile_limits`] applies independently to each tile,
//! cumulatively across its stages (bytes are conservative allocation charges,
//! not peak resident memory). Each tile owns a separate workspace. Concurrent
//! memory is admitted with [`BakeConfig::max_total_bytes`] before tile work.
//! This bounds charged storage, excluding allocator metadata, stacks and the pool.
//! Rayon owns its scheduler/pool allocation costs. Without `parallel`, this API
//! supports `no_std` + `alloc`, including wasm.
//!
//! **Triangles that cross a tile's bounds** are not split by the baker: every tile
//! whose bounds, widened by the border (`walkable_radius + 3` cells), overlap a
//! triangle's xz bounds receives the whole triangle, in input order. Each tile then
//! rasterises it with Recast's rasteriser, which clips it to that tile's heightfield
//! (the tile plus its border); the parts outside are dropped. The walkable-slope
//! test and the area mark use the triangle's own normal and are the same in every
//! tile that receives it, so a surface crossing a seam is rasterised identically
//! (up to the border) on both sides.

use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::fmt;

use crate::common::{floorf, fmax, fmin};
use crate::detour::{self, NavMesh, NavMeshCreateParams, NavMeshParams};
use crate::recast::{self, Config};
use crate::{Error, Limits, UpstreamFixes, Workspace};

mod install;
mod job;
mod manifest;
mod source;
mod task;

pub use install::{TileInstaller, install_cost};
mod report;
pub use report::{BakeReport, FilterCounts, TileReport};

pub use job::{BakeJob, JobOptions, TileOutput};
pub use manifest::{
    BAKE_REVISION, BakeManifest, DirtyTiles, GridLayout, MANIFEST_VERSION, ManifestDelta,
    ManifestEntry, TileKey,
};
pub use source::{
    GeometrySource, IndexedInput, ItemId, OffMeshSink, TriangleId, TriangleSink, VolumeSink,
};
#[cfg(test)]
use task::TileWorkspace;
use task::{Advance, OffMesh, TileTask, TriSource, VolumeRef};

/// Region partition used by the tile pipeline; not in recastnavigation as a type.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Partition {
    /// Distance field followed by watershed regions.
    #[default]
    Watershed,
    /// Monotone regions.
    Monotone,
    /// Layer regions (polygon tiles, not tile-cache layers).
    Layers,
}

/// Convex area marking after erosion; not in recastnavigation as a type.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub struct AreaVolume<'a> {
    /// Packed xyz vertices of a convex polygon.
    pub verts: &'a [f32],
    /// Minimum world-space height.
    pub hmin: f32,
    /// Maximum world-space height.
    pub hmax: f32,
    /// Recast area id, 0..=63.
    pub area: u8,
}

/// Borrowed tile-bake inputs; not in recastnavigation. Construct with [`Input::new`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Input<'a> {
    /// Packed xyz coordinates.
    pub verts: &'a [f32],
    /// Packed triangle vertex indices.
    pub tris: &'a [i32],
    /// Optional area per triangle; unwalkable slopes are cleared. With `None`,
    /// walkable triangles start as [`recast::WALKABLE_AREA`].
    pub areas: Option<&'a [u8]>,
    /// Convex area markings, applied in slice order after erosion.
    pub volumes: &'a [AreaVolume<'a>],
    /// Only the `off_mesh_con_*` fields are read; all other fields are ignored.
    /// Links retain their input order within each tile.
    pub off_mesh: NavMeshCreateParams<'a>,
}

impl<'a> Input<'a> {
    /// Borrows a triangle mesh with default walkability and no area volumes/links.
    pub fn new(verts: &'a [f32], tris: &'a [i32]) -> Self {
        Self {
            verts,
            tris,
            areas: None,
            volumes: &[],
            off_mesh: NavMeshCreateParams::default(),
        }
    }
}

/// Tile settings; not in recastnavigation as a type. Customize [`Default`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct BakeConfig {
    /// Cell-unit Recast settings. `tile_size` must be positive; bounds, grid
    /// dimensions and border are derived from the mesh. Defaults match the demo.
    pub recast: Config,
    /// Region partition.
    pub partition: Partition,
    /// Apply the low-hanging obstacle filter.
    pub filter_low_hanging: bool,
    /// Apply the ledge filter.
    pub filter_ledge: bool,
    /// Apply the low-ceiling filter.
    pub filter_low_height: bool,
    /// Agent height in world units, stored in Detour tiles.
    pub walkable_height: f32,
    /// Agent radius in world units, stored in Detour tiles.
    pub walkable_radius: f32,
    /// Agent climb in world units, stored in Detour tiles.
    pub walkable_climb: f32,
    /// Independent cumulative budget for each tile.
    pub tile_limits: Limits,
    /// Aggregate charged-storage budget for setup, retained outputs and active
    /// workers. `usize::MAX` disables it. A finite value requires a finite
    /// `tile_limits.max_bytes`. Before tile work, reserve setup charges plus
    /// `tile_count * min(max_tile_bytes, max_bytes)` retained bytes plus
    /// `min(tile_count, worker_count) * max_bytes` working bytes. Serial uses one
    /// worker; parallel uses the current rayon pool size. Conservative: outputs
    /// may be counted twice, including empty tiles. Excludes allocator metadata,
    /// thread stacks and rayon-owned storage. Failure has no tile location.
    pub max_total_bytes: usize,
    /// Reject incomplete ear clipping (not in recastnavigation). Defaults to false.
    pub strict_triangulation: bool,
    /// Upstream defect corrections applied while baking (not in recastnavigation).
    /// Only [`UpstreamFixes::ZERO_SAMPLE_DIST`] concerns baking. Defaults to
    /// [`UpstreamFixes::NONE`], which reproduces upstream.
    pub fixes: UpstreamFixes,
}

impl Default for BakeConfig {
    fn default() -> Self {
        Self {
            recast: Config {
                tile_size: 32,
                cs: 0.3,
                ch: 0.2,
                walkable_slope_angle: 45.0,
                walkable_height: 10,
                walkable_climb: 4,
                walkable_radius: 2,
                max_edge_len: 40,
                max_simplification_error: 1.3,
                min_region_area: 64,
                merge_region_area: 400,
                max_verts_per_poly: 6,
                detail_sample_dist: 0.3 * 6.0,
                detail_sample_max_error: 0.2,
                ..Config::default()
            },
            partition: Partition::Watershed,
            filter_low_hanging: true,
            filter_ledge: true,
            filter_low_height: true,
            walkable_height: 2.0,
            walkable_radius: 0.6,
            walkable_climb: 0.9,
            tile_limits: Limits::default(),
            max_total_bytes: usize::MAX,
            strict_triangulation: false,
            fixes: UpstreamFixes::NONE,
        }
    }
}

/// A bake failure with a deterministic tile location; not in recastnavigation.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BakeError {
    /// `[x, z]`, or `None` for validation/bucketing/result allocation failures.
    pub tile: Option<[i32; 2]>,
    /// Underlying pipeline error.
    pub error: Error,
}

impl From<Error> for BakeError {
    fn from(error: Error) -> Self {
        Self { tile: None, error }
    }
}
impl fmt::Display for BakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tile {:?}: {}", self.tile, self.error)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for BakeError {}

/// One row-major tile result; not in recastnavigation.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    /// Tile x coordinate.
    pub x: i32,
    /// Tile z coordinate (Detour's tile y).
    pub y: i32,
    /// Original DNAV bytes, or `None` when the tile contains no polygons.
    pub data: Option<Vec<u8>>,
    /// Partial ear-clipping counts (not in recastnavigation).
    pub triangulation: recast::TriangulationReport,
}

/// Ordered tiles and their Detour layout; not in recastnavigation.
#[non_exhaustive]
#[derive(Debug)]
pub struct BakedTiles {
    /// Layout for assembling the tile set.
    pub params: NavMeshParams,
    /// All grid tiles, including empty tiles, in row-major order.
    pub tiles: Vec<Tile>,
}

impl BakedTiles {
    /// Inserts nonempty tiles in row-major order, preserving serial link/ref order.
    /// The navmesh uses its own `limits`; insertion failures identify the tile.
    pub fn into_navmesh(self, limits: Limits) -> Result<NavMesh, BakeError> {
        let mut nav = NavMesh::with_limits(&self.params, limits)?;
        for tile in self.tiles {
            if let Some(data) = tile.data {
                nav.add_tile(data).map_err(|error| BakeError {
                    tile: Some([tile.x, tile.y]),
                    error,
                })?;
            }
        }
        Ok(nav)
    }
}

fn work(ws: &mut Workspace, n: usize) -> Result<(), Error> {
    ws.work(u64::try_from(n).map_err(|_| Error::SizeOverflow)?)
}
fn positive(v: f32) -> bool {
    v.is_finite() && v > 0.0
}
fn nonnegative(v: f32) -> bool {
    v.is_finite() && v >= 0.0
}
fn integer(v: f32) -> Result<i32, Error> {
    if !v.is_finite() || v < 0.0 || f64::from(v) > f64::from(i32::MAX) {
        return Err(Error::SizeOverflow);
    }
    Ok(v as i32) // finite, nonnegative and in range; truncation matches rcCalcGridSize
}

struct Prepared {
    configs: Vec<Config>,
    buckets: Vec<Vec<usize>>,
    volume_buckets: Vec<Vec<usize>>,
    params: NavMeshParams,
    columns: i32,
}

/// Checks every setting a tile bake reads, before any geometry is touched.
fn validate_config(c: &BakeConfig) -> Result<(), Error> {
    let base = c.recast;
    if !positive(base.cs)
        || !positive(base.ch)
        || !(1.0 / base.cs).is_finite()
        || !(1.0 / base.ch).is_finite()
        || base.tile_size <= 0
        || !(1..=recast::SPAN_MAX_HEIGHT).contains(&base.walkable_height)
        || !(0..=recast::SPAN_MAX_HEIGHT).contains(&base.walkable_climb)
        || base.walkable_radius < 0
        || base.max_edge_len < 0
        || base.min_region_area < 0
        || base.merge_region_area < 0
        || !(3..=6).contains(&base.max_verts_per_poly)
        || !nonnegative(base.walkable_slope_angle)
        || base.walkable_slope_angle > 90.0
        || !nonnegative(base.max_simplification_error)
        || !nonnegative(base.detail_sample_dist)
        || !nonnegative(base.detail_sample_max_error)
        || !positive(c.walkable_height)
        || !nonnegative(c.walkable_radius)
        || !nonnegative(c.walkable_climb)
    {
        return Err(Error::InvalidParameter("tile bake configuration"));
    }
    Ok(())
}

fn prepare(input: &Input<'_>, c: &BakeConfig, ws: &mut Workspace) -> Result<Prepared, Error> {
    ws.begin();
    validate_config(c)?;
    let base = c.recast;
    work(
        ws,
        input
            .verts
            .len()
            .checked_add(input.tris.len())
            .ok_or(Error::SizeOverflow)?,
    )?;
    recast::validate_mesh(input.verts, input.tris, input.tris.len() / 3)?;
    if input.verts.is_empty() || input.verts.iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput("empty or non-finite vertices"));
    }
    if let Some(a) = input.areas {
        work(ws, a.len())?;
        if a.len() != input.tris.len() / 3 || a.iter().any(|&v| v > 63) {
            return Err(Error::InvalidInput("triangle areas"));
        }
    }
    work(ws, input.volumes.len())?;
    for v in input.volumes {
        work(ws, v.verts.len())?;
        if v.verts.len() < 9
            || !v.verts.len().is_multiple_of(3)
            || v.verts.iter().any(|v| !v.is_finite())
            || !v.hmin.is_finite()
            || !v.hmax.is_finite()
            || v.hmin > v.hmax
            || v.area > 63
        {
            return Err(Error::InvalidInput("area volume"));
        }
    }
    validate_links(input, ws)?;
    let (bmin, bmax) = recast::calc_bounds(input.verts);
    if (0..3).any(|axis| !(bmax[axis] - bmin[axis]).is_finite())
        || !((bmax[1] - bmin[1]) / base.ch).is_finite()
        || !(bmax[1] + base.walkable_height as f32 * base.ch).is_finite()
    {
        return Err(Error::SizeOverflow);
    }

    let ts = base.tile_size;
    let gw = integer((bmax[0] - bmin[0]) / base.cs + 0.5)?;
    let gh = integer((bmax[2] - bmin[2]) / base.cs + 0.5)?;
    let tw = gw.checked_add(ts - 1).ok_or(Error::SizeOverflow)? / ts;
    let th = gh.checked_add(ts - 1).ok_or(Error::SizeOverflow)? / ts;
    if tw == 0 || th == 0 {
        return Err(Error::InvalidInput("empty tile grid"));
    }
    let count = usize::try_from(tw.checked_mul(th).ok_or(Error::SizeOverflow)?)
        .map_err(|_| Error::SizeOverflow)?;
    // Upstream demo ref-bit layout. Reject impossible layouts before grid allocation.
    if count > (1 << 14) {
        return Err(Error::FormatLimit("more than 16,384 grid tiles"));
    }
    work(ws, count)?;
    let mut configs = ws.vec_with_capacity(count)?;
    let mut buckets = ws.vec(count, Vec::new())?;
    let border = base
        .walkable_radius
        .checked_add(3)
        .ok_or(Error::SizeOverflow)?;
    let dim = ts
        .checked_add(border.checked_mul(2).ok_or(Error::SizeOverflow)?)
        .ok_or(Error::SizeOverflow)?;
    // Keep cell-coordinate products within the signed Recast arithmetic domain.
    if dim > i32::from(i16::MAX) {
        return Err(Error::FormatLimit("tile cell dimensions"));
    }
    let tcs = ts as f32 * base.cs;
    let b = border as f32 * base.cs;
    if !positive(tcs) || !b.is_finite() {
        return Err(Error::SizeOverflow);
    }
    for y in 0..th {
        for x in 0..tw {
            let mut cfg = base;
            cfg.border_size = border;
            cfg.width = dim;
            cfg.height = dim;
            cfg.bmin = [
                bmin[0] + x as f32 * tcs - b,
                bmin[1],
                bmin[2] + y as f32 * tcs - b,
            ];
            cfg.bmax = [
                bmin[0] + (x + 1) as f32 * tcs + b,
                bmax[1],
                bmin[2] + (y + 1) as f32 * tcs + b,
            ];
            if cfg.bmin.iter().chain(&cfg.bmax).any(|v| !v.is_finite()) {
                return Err(Error::SizeOverflow);
            }
            configs.push(cfg);
        }
    }
    // A spatial range per triangle, never a scan of all triangles per tile.
    // The extra tile of margin makes boundary rounding conservative; exact
    // bordered overlap below retains the driver's triangle selection/order.
    let margin = b + tcs;
    for (i, tri) in input.tris.as_chunks::<3>().0.iter().enumerate() {
        let mut lo = [f32::INFINITY; 2];
        let mut hi = [f32::NEG_INFINITY; 2];
        for &k in tri {
            let k = usize::try_from(k).map_err(|_| Error::InvalidInput("triangle index"))? * 3;
            lo[0] = fmin(lo[0], input.verts[k]);
            hi[0] = fmax(hi[0], input.verts[k]);
            lo[1] = fmin(lo[1], input.verts[k + 2]);
            hi[1] = fmax(hi[1], input.verts[k + 2]);
        }
        // Saturating float-to-int conversion followed by clamping is intentional
        // here: very large finite triangles can overlap the entire grid.
        let cell = |v: f32, n: i32| (floorf(v) as i32).clamp(0, n - 1);
        let x0 = cell((lo[0] - bmin[0] - margin) / tcs, tw);
        let x1 = cell((hi[0] - bmin[0] + margin) / tcs, tw);
        let y0 = cell((lo[1] - bmin[2] - margin) / tcs, th);
        let y1 = cell((hi[1] - bmin[2] + margin) / tcs, th);
        work(
            ws,
            usize::try_from(
                (x1 - x0 + 1)
                    .checked_mul(y1 - y0 + 1)
                    .ok_or(Error::SizeOverflow)?,
            )
            .map_err(|_| Error::SizeOverflow)?,
        )?;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let at = usize::try_from(y * tw + x).map_err(|_| Error::SizeOverflow)?;
                let cfg = &configs[at];
                if lo[0] <= cfg.bmax[0]
                    && hi[0] >= cfg.bmin[0]
                    && lo[1] <= cfg.bmax[2]
                    && hi[1] >= cfg.bmin[2]
                {
                    ws.push(&mut buckets[at], i)?;
                }
            }
        }
    }
    // Bucket volumes spatially once, preserving their input order per tile.
    // Tiles are vertical columns: use bordered xz overlap, and retain height
    // filtering in Recast's quantized span domain (not the geometry's y bounds).
    let mut volume_buckets = ws.vec(count, Vec::new())?;
    for (i, v) in input.volumes.iter().enumerate() {
        work(ws, v.verts.len())?;
        let (lo, hi) = recast::calc_bounds(v.verts);
        let cell = |v: f32, n: i32| (floorf(v) as i32).clamp(0, n - 1);
        let x0 = cell((lo[0] - bmin[0] - margin) / tcs, tw);
        let x1 = cell((hi[0] - bmin[0] + margin) / tcs, tw);
        let y0 = cell((lo[2] - bmin[2] - margin) / tcs, th);
        let y1 = cell((hi[2] - bmin[2] + margin) / tcs, th);
        work(
            ws,
            usize::try_from(
                (x1 - x0 + 1)
                    .checked_mul(y1 - y0 + 1)
                    .ok_or(Error::SizeOverflow)?,
            )
            .map_err(|_| Error::SizeOverflow)?,
        )?;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let at = (y * tw + x) as usize;
                let cfg = &configs[at];
                if lo[0] <= cfg.bmax[0]
                    && hi[0] >= cfg.bmin[0]
                    && lo[2] <= cfg.bmax[2]
                    && hi[2] >= cfg.bmin[2]
                {
                    ws.push(&mut volume_buckets[at], i)?;
                }
            }
        }
    }
    let tile_bits = count.next_power_of_two().trailing_zeros();
    Ok(Prepared {
        configs,
        buckets,
        volume_buckets,
        columns: tw,
        params: NavMeshParams {
            orig: bmin,
            tile_width: tcs,
            tile_height: tcs,
            max_tiles: 1 << tile_bits,
            max_polys: 1 << (22 - tile_bits).min(if detour::POLYREF_BITS == 64 { 20 } else { 22 }),
        },
    })
}

fn validate_links(input: &Input<'_>, ws: &mut Workspace) -> Result<(), Error> {
    let p = &input.off_mesh;
    let n =
        usize::try_from(p.off_mesh_con_count).map_err(|_| Error::InvalidInput("off-mesh count"))?;
    work(ws, n)?;
    if p.off_mesh_con_verts.len() != n.checked_mul(6).ok_or(Error::SizeOverflow)?
        || p.off_mesh_con_rad.len() != n
        || p.off_mesh_con_flags.len() != n
        || p.off_mesh_con_areas.len() != n
        || p.off_mesh_con_dir.len() != n
        || (!p.off_mesh_con_user_id.is_empty() && p.off_mesh_con_user_id.len() != n)
        || p.off_mesh_con_verts.iter().any(|v| !v.is_finite())
        || p.off_mesh_con_rad.iter().any(|&v| !nonnegative(v))
        || p.off_mesh_con_areas.iter().any(|&v| v > 63)
    {
        return Err(Error::InvalidInput("off-mesh connections"));
    }
    Ok(())
}

fn build_tile(
    input: &Input<'_>,
    c: &BakeConfig,
    cfg: &Config,
    bucket: &[usize],
    volume_bucket: &[usize],
    x: i32,
    y: i32,
) -> TileResult {
    build_tile_counted(input, c, cfg, bucket, volume_bucket, x, y).0
}

/// Builds one tile from a borrowed input; also returns the work the tile charged.
fn build_tile_counted(
    input: &Input<'_>,
    c: &BakeConfig,
    cfg: &Config,
    bucket: &[usize],
    volume_bucket: &[usize],
    x: i32,
    y: i32,
) -> (TileResult, u64) {
    let mut task = TileTask::new(
        c,
        cfg,
        x,
        y,
        Cow::Borrowed(input.verts),
        TriSource::Bucket { input, bucket },
        volume_bucket
            .iter()
            .map(|&i| VolumeRef::borrowed(&input.volumes[i]))
            .collect(),
        OffMesh::Borrowed(input.off_mesh),
        None,
    );
    loop {
        match task.advance() {
            Ok(Advance::More) => {}
            Ok(Advance::Done(data, report)) => return (Ok((data, report)), task.work_used()),
            Err(e) => return (Err(e), task.work_used()),
        }
    }
}

/// Bakes tiles serially; extension **not in recastnavigation**.
/// See the [module contract](self) for the separate setup and per-tile budgets.
pub fn tiles(
    input: &Input<'_>,
    config: &BakeConfig,
    ws: &mut Workspace,
) -> Result<BakedTiles, BakeError> {
    bake(input, config, ws, false)
}

/// Bakes tiles concurrently with the `parallel` feature, using the current rayon
/// pool; otherwise falls back to serial. Extension **not in recastnavigation**.
/// Results and the first error have the same tile order as [`tiles`], even if
/// later tiles finish first. Already running tiles finish before returning an error.
pub fn tiles_parallel(
    input: &Input<'_>,
    config: &BakeConfig,
    ws: &mut Workspace,
) -> Result<BakedTiles, BakeError> {
    bake(input, config, ws, true)
}

fn bake(
    input: &Input<'_>,
    c: &BakeConfig,
    ws: &mut Workspace,
    parallel: bool,
) -> Result<BakedTiles, BakeError> {
    // Also enforce the aggregate ceiling during setup, before its allocations.
    let setup_limits = ws.limits();
    let mut capped = setup_limits;
    capped.max_bytes = capped.max_bytes.min(c.max_total_bytes);
    ws.set_limits(capped);
    let prepared = prepare(input, c, ws);
    ws.set_limits(setup_limits);
    let p = prepared?;
    let slot_bytes = p
        .configs
        .len()
        .checked_mul(core::mem::size_of::<Tile>() + core::mem::size_of::<TileResult>())
        .and_then(|n| n.checked_add(ws.usage().bytes))
        .ok_or(Error::SizeOverflow)?;
    check_total_bytes(c, slot_bytes, p.configs.len(), worker_count(parallel))?;
    work(ws, p.configs.len())?;
    let mut output = ws.vec_with_capacity::<Tile>(p.configs.len())?;
    // Slots are allocated fallibly and byte-charged before rayon sees them.
    let mut results = ws.vec(
        p.configs.len(),
        Ok((None, recast::TriangulationReport::default())),
    )?;
    let columns = p.columns;
    let coord = |i: usize| -> Result<(i32, i32), Error> {
        let i = i32::try_from(i).map_err(|_| Error::SizeOverflow)?;
        Ok((i % columns, i / columns))
    };
    let build = |i: usize| -> TileResult {
        let (x, y) = coord(i)?;
        build_tile(
            input,
            c,
            &p.configs[i],
            &p.buckets[i],
            &p.volume_buckets[i],
            x,
            y,
        )
    };
    fill_results(&mut results, parallel, build);
    for (i, result) in results.into_iter().enumerate() {
        let (x, y) = coord(i)?;
        let (data, triangulation) = result.map_err(|error| BakeError {
            tile: Some([x, y]),
            error,
        })?;
        output.push(Tile {
            x,
            y,
            data,
            triangulation,
        });
    }
    Ok(BakedTiles {
        params: p.params,
        tiles: output,
    })
}

fn worker_count(parallel: bool) -> usize {
    #[cfg(feature = "parallel")]
    if parallel {
        return rayon::current_num_threads();
    }
    let _ = parallel;
    1
}

fn check_total_bytes(
    c: &BakeConfig,
    setup: usize,
    count: usize,
    workers: usize,
) -> Result<(), Error> {
    if c.max_total_bytes == usize::MAX {
        return Ok(());
    }
    if c.tile_limits.max_bytes == usize::MAX {
        return Err(Error::LimitExceeded);
    }
    let retained = count.checked_mul(c.tile_limits.max_tile_bytes.min(c.tile_limits.max_bytes));
    let active = count.min(workers).checked_mul(c.tile_limits.max_bytes);
    let required = retained
        .and_then(|n| active.and_then(|a| n.checked_add(a)))
        .and_then(|n| n.checked_add(setup));
    if required.is_none_or(|n| n > c.max_total_bytes) {
        return Err(Error::LimitExceeded);
    }
    Ok(())
}

type TileResult = Result<(Option<Vec<u8>>, recast::TriangulationReport), Error>;

fn fill_results(
    results: &mut [TileResult],
    parallel: bool,
    build: impl Fn(usize) -> TileResult + Sync,
) {
    #[cfg(feature = "parallel")]
    if parallel {
        use rayon::prelude::*;
        results
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, slot)| *slot = build(i));
    }
    #[cfg(not(feature = "parallel"))]
    let parallel = {
        let _ = parallel;
        false
    };
    if !parallel {
        for (i, slot) in results.iter_mut().enumerate() {
            *slot = build(i);
            if slot.is_err() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Usage;

    #[test]
    fn aggregate_reserves_retained_outputs_and_all_workers_before_work() {
        let mut c = BakeConfig::default();
        c.tile_limits.max_bytes = 100;
        c.tile_limits.max_tile_bytes = 20;
        c.max_total_bytes = 7 + 8 * 20 + 4 * 100;
        assert_eq!(check_total_bytes(&c, 7, 8, 4), Ok(()));
        c.max_total_bytes -= 1;
        assert_eq!(check_total_bytes(&c, 7, 8, 4), Err(Error::LimitExceeded));
        assert_eq!(check_total_bytes(&c, 7, 8, 1), Ok(()));
        c.tile_limits.max_bytes = usize::MAX;
        assert_eq!(check_total_bytes(&c, 7, 8, 1), Err(Error::LimitExceeded));
        c.tile_limits.max_bytes = usize::MAX - 1;
        assert_eq!(check_total_bytes(&c, 7, 8, 1), Err(Error::LimitExceeded));
        c.max_total_bytes = usize::MAX;
        assert_eq!(check_total_bytes(&c, 7, 8, 16), Ok(()));
    }

    #[test]
    fn volume_buckets_keep_border_overlap_order_and_quantized_heights() {
        let verts = [0., 0., 0., 0., 0., 9.6, 19.2, 0., 9.6, 19.2, 0., 0.];
        let tris = [0, 1, 2, 0, 2, 3];
        let local = [9., 0., 2., 9., 0., 7., 10., 0., 7., 10., 0., 2.];
        let outside = [100., 0., 2., 100., 0., 7., 101., 0., 7., 101., 0., 2.];
        let volumes = [
            AreaVolume {
                verts: &outside,
                hmin: -1.,
                hmax: 2.,
                area: 1,
            },
            AreaVolume {
                verts: &local,
                hmin: 0.1,
                hmax: 0.3,
                area: 3,
            },
            AreaVolume {
                verts: &local,
                hmin: 0.1,
                hmax: 0.3,
                area: 7,
            },
        ];
        let mut input = Input::new(&verts, &tris);
        input.volumes = &volumes;
        let c = BakeConfig::default();
        let p = prepare(&input, &c, &mut Workspace::new()).unwrap();
        assert_eq!(p.configs.len(), 2);
        for i in 0..2 {
            assert_eq!(p.volume_buckets[i], [1, 2]);
            let culled = build_tile(
                &input,
                &c,
                &p.configs[i],
                &p.buckets[i],
                &p.volume_buckets[i],
                i as i32,
                0,
            )
            .unwrap();
            let all = build_tile(
                &input,
                &c,
                &p.configs[i],
                &p.buckets[i],
                &[0, 1, 2],
                i as i32,
                0,
            )
            .unwrap();
            assert_eq!(culled, all);
            let no_volumes =
                build_tile(&input, &c, &p.configs[i], &p.buckets[i], &[], i as i32, 0).unwrap();
            assert_ne!(culled, no_volumes);
        }
    }

    #[test]
    fn spatial_volumes_scale_with_tiles_and_preserve_mark_order() {
        fn run(n: usize) -> (u64, u64) {
            let mut verts = Vec::new();
            let mut tris = Vec::new();
            let mut polygons = Vec::new();
            let tile_width = 9.6;
            for i in 0..n {
                let x = i as f32 * tile_width;
                let b = (verts.len() / 3) as i32;
                verts.extend_from_slice(&[
                    x,
                    0.,
                    0.,
                    x,
                    0.,
                    tile_width,
                    x + tile_width,
                    0.,
                    tile_width,
                    x + tile_width,
                    0.,
                    0.,
                ]);
                tris.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
                polygons.push([
                    x + 2.,
                    0.,
                    2.,
                    x + 2.,
                    0.,
                    4.,
                    x + 4.,
                    0.,
                    4.,
                    x + 4.,
                    0.,
                    2.,
                ]);
            }
            let volumes: Vec<_> = polygons
                .iter()
                .map(|v| AreaVolume {
                    verts: v,
                    hmin: -1.,
                    hmax: 2.,
                    area: 3,
                })
                .collect();
            let mut input = Input::new(&verts, &tris);
            input.volumes = &volumes;
            let c = BakeConfig::default();
            let mut ws = Workspace::new();
            let p = prepare(&input, &c, &mut ws).unwrap();
            assert_eq!(p.configs.len(), n);
            assert_eq!(p.volume_buckets.iter().map(Vec::len).sum::<usize>(), n);
            let setup = ws.usage().work;
            let mut work = 0;
            for i in 0..n {
                let (data, tile_work) = build_tile_counted(
                    &input,
                    &c,
                    &p.configs[i],
                    &p.buckets[i],
                    &p.volume_buckets[i],
                    i as i32,
                    0,
                );
                let data = data.unwrap();
                assert!(data.0.is_some());
                work += tile_work;
                if n == 8 {
                    let all: Vec<_> = (0..volumes.len()).collect();
                    let unculled =
                        build_tile(&input, &c, &p.configs[i], &p.buckets[i], &all, i as i32, 0)
                            .unwrap();
                    assert_eq!(data, unculled);
                }
            }
            (setup, work)
        }
        let a = run(8);
        let b = run(64);
        #[cfg(feature = "std")]
        std::println!(
            "tile/volume scaling 8 -> 64: setup {} -> {}, tile work {} -> {}",
            a.0,
            b.0,
            a.1,
            b.1
        );
        assert!(b.0 <= a.0 * 9);
        assert!(b.1 <= a.1 * 9);
    }

    #[test]
    fn tile_budget_is_cumulative_across_stage_resets() {
        let mut limits = Limits {
            max_work: 5,
            ..Limits::default()
        };
        let mut tile = TileWorkspace {
            ws: Workspace::new(),
            limits,
            usage: Usage::default(),
        };
        tile.stage(|ws| ws.work(3)).unwrap();
        assert_eq!(tile.stage(|ws| ws.work(3)), Err(Error::LimitExceeded));
        limits.max_bytes = 5;
        let mut tile = TileWorkspace {
            ws: Workspace::new(),
            limits,
            usage: Usage::default(),
        };
        tile.stage(|ws| ws.vec::<u8>(3, 0)).unwrap();
        assert_eq!(
            tile.stage(|ws| ws.vec::<u8>(3, 0)),
            Err(Error::LimitExceeded)
        );
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn completed_later_error_cannot_replace_first_tile_error() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let mut results =
            core::array::from_fn::<_, 4, _>(|_| Ok((None, recast::TriangulationReport::default())));
        pool.install(|| {
            fill_results(&mut results, true, |i| {
                if i == 1 {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    Err(Error::InvalidInput("earlier tile"))
                } else if i == 3 {
                    Err(Error::Failed)
                } else {
                    Ok((None, recast::TriangulationReport::default()))
                }
            })
        });
        let first = results
            .into_iter()
            .enumerate()
            .find_map(|(i, r)| r.err().map(|e| (i, e)));
        assert_eq!(first, Some((1, Error::InvalidInput("earlier tile"))));
    }
}
