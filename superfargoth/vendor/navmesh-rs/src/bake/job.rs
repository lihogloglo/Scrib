//! Resumable, cancellable, incremental bake jobs (not in recastnavigation).

use alloc::borrow::Cow;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::manifest::{
    BakeManifest, GridLayout, MANIFEST_VERSION, ManifestDelta, ManifestEntry, TileKey,
    digest_bytes, input_hash,
};
use super::source::{GeometrySource, OffMeshSink, TriangleSink, VolumeSink};
use super::task::{Advance, OffMesh, TileTask, TriSource, VolumeRef};
use super::{BakeConfig, BakeError};
use crate::recast::TriangulationReport;
use crate::{Budget, CancelToken, Error, Progress, Workspace};

/// Settings of a [`BakeJob`] (not in recastnavigation). Customize [`Default`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub struct JobOptions {
    /// Most tiles whose intermediate state is alive at once in
    /// [`BakeJob::run_parallel_chunk`]. `0` means twice the worker count. A serial
    /// [`BakeJob::step`] always has one tile in flight.
    pub max_in_flight: usize,
    /// Collect per-tile bake telemetry; disabled by default.
    pub reports: bool,
}

/// One finished tile of a [`BakeJob`] (not in recastnavigation).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileOutput {
    /// The tile.
    pub key: TileKey,
    /// DNAV bytes, or `None` when the tile contains no polygons.
    pub data: Option<Vec<u8>>,
    /// Partial ear-clipping counts.
    pub triangulation: TriangulationReport,
    /// Digest of everything the tile's pipeline read.
    pub input_hash: u64,
    /// Digest of `data`; `None` when the tile has no polygons.
    pub output_hash: Option<u64>,
}

struct Fetched {
    triangles: TriangleSink,
    volumes: VolumeSink,
    off_mesh: OffMeshSink,
    input_hash: u64,
}

impl Fetched {
    fn items(&self) -> u64 {
        (self.triangles.len() + self.volumes.len() + self.off_mesh.len()) as u64
    }
}

/// Everything the job reads from its configuration, shared by every worker.
struct Plan<'p, S> {
    src: &'p S,
    cfg: &'p BakeConfig,
    grid: &'p GridLayout,
    config_hash: u64,
    reports: bool,
}

impl<S: GeometrySource> Plan<'_, S> {
    fn fetch(&self, key: TileKey, cancel: &CancelToken) -> Result<Fetched, Error> {
        if cancel.is_cancelled() {
            return Err(Error::Cancelled);
        }
        let (bmin, bmax) = self.grid.tile_bounds(key);
        let cap = self.cfg.tile_limits.max_bytes;
        let mut triangles = TriangleSink::new(bmin, bmax, cap);
        let mut volumes = VolumeSink::new(bmin, bmax, cap);
        let mut off_mesh = OffMeshSink::new(bmin, bmax, cap);
        self.src.triangles(bmin, bmax, &mut triangles)?;
        self.src.volumes(bmin, bmax, &mut volumes)?;
        self.src.off_mesh(bmin, bmax, &mut off_mesh)?;
        let input_hash = input_hash(self.config_hash, key, &triangles, &volumes, &off_mesh);
        Ok(Fetched {
            triangles,
            volumes,
            off_mesh,
            input_hash,
        })
    }

    fn task(
        &self,
        key: TileKey,
        fetched: Fetched,
        cancel: &CancelToken,
    ) -> Result<TileTask<'static>, Error> {
        let tile_cfg = self.grid.tile_config(self.cfg, key)?;
        let (verts, areas, mode) = fetched.triangles.into_parts();
        let mut task = TileTask::new(
            self.cfg,
            &tile_cfg,
            key.x,
            key.z,
            Cow::Owned(verts),
            TriSource::Fetched { areas, mode },
            VolumeRef::from_sink(fetched.volumes),
            OffMesh::Owned(fetched.off_mesh),
            Some(cancel.clone()),
        );
        if self.reports {
            task.enable_report();
        }
        Ok(task)
    }

    /// Fetches and bakes one tile from start to end.
    fn bake_tile(
        &self,
        key: TileKey,
        cancel: &CancelToken,
    ) -> (Result<TileOutput, BakeError>, Option<super::TileReport>) {
        let at = |error: Error| BakeError {
            tile: Some([key.x, key.z]),
            error,
        };
        let mut report = None;
        let result = (|| {
            let fetched = self.fetch(key, cancel).map_err(at)?;
            let hash = fetched.input_hash;
            let mut task = self.task(key, fetched, cancel).map_err(at)?;
            loop {
                match task.advance() {
                    Ok(Advance::More) => {}
                    Ok(Advance::Done(data, triangulation)) => {
                        report = task.report;
                        return Ok(output(key, hash, data, triangulation));
                    }
                    Err(error) => {
                        if let Some(stats) = &mut task.report {
                            stats.fail(&error);
                        }
                        report = task.report;
                        return Err(at(error));
                    }
                }
            }
        })();
        (result, report)
    }
}

fn output(
    key: TileKey,
    input_hash: u64,
    data: Option<Vec<u8>>,
    triangulation: TriangulationReport,
) -> TileOutput {
    TileOutput {
        key,
        output_hash: data.as_deref().map(digest_bytes),
        data,
        triangulation,
        input_hash,
    }
}

/// A bake of a set of tiles from a [`GeometrySource`] (not in recastnavigation).
///
/// The job fetches each tile's bordered column from the source, bakes it with the
/// same pipeline as [`tiles`](super::tiles), and hands finished tiles out in
/// row-major order through [`next_ready`](Self::next_ready). Three ways to drive it,
/// all producing the same bytes in the same order:
///
/// - [`step`](Self::step): serial, in slices of a work [`Budget`] (a frame's allowance);
/// - [`run_parallel`](Self::run_parallel): all remaining tiles on the rayon pool
///   (with the `parallel` feature; serial otherwise);
/// - a mix: a tile begun by `step` is finished serially before the next parallel chunk.
///
/// Work is split at pipeline stages, not inside them: a [`step`](Self::step) runs whole
/// stages, and the longest uninterrupted piece is one stage of one tile (or one fetch).
/// A job never touches a navmesh.
///
/// **Byte limits.** [`BakeConfig::tile_limits`] and [`BakeConfig::max_total_bytes`] count
/// bytes the way [`tiles`](super::tiles) does (the triangle lists of a tile cost the same
/// in both), so a job over an [`Input`](super::Input) succeeds and fails with
/// the same limits. One term differs: the Detour output bound of a tile counts the
/// off-mesh connections of its own column here and every connection of the input in
/// [`tiles`](super::tiles), so near the limit a job can succeed where `tiles` fails, never
/// the reverse. `max_total_bytes` is checked before each fetch and each parallel chunk
/// against the finished tiles not yet taken from [`next_ready`](Self::next_ready) plus the
/// tiles in flight; take tiles out as they finish to stay within it.
///
/// **Cancelling.** [`cancel_token`](Self::cancel_token) ends the job at once. Every tile
/// that [`next_ready`](Self::next_ready) returned before is whole and valid, and is listed
/// in [`manifest_delta`](Self::manifest_delta); every other tile of the job is absent, not
/// partial. A navmesh fed from the job therefore holds, per tile, either its old version
/// or its new one, and the delta says which tiles are new: the window in which neighbours
/// of the two kinds meet is the time between the cancel and the next job over the tiles
/// still stale. Install the delivered tiles of a cancelled job (or none), apply the delta
/// to the manifest, and run a new job over the same keys: [`skip_unchanged`](Self::skip_unchanged)
/// drops the delivered tiles and the job settles the rest.
/// [`TileInstaller`](super::TileInstaller) keeps the newest version of a
/// tile and replaces the resident one, so a new job's tiles supersede the old job's
/// whether they were installed or still queued.
///
/// ```
/// use navmesh_rs::bake::{BakeConfig, BakeJob, GridLayout, Input, JobOptions};
/// use navmesh_rs::{Budget, Progress};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let verts = [0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0, 0.0, 10.0, 10.0, 0.0, 0.0];
/// let tris = [0, 1, 2, 0, 2, 3];
/// let input = Input::new(&verts, &tris);
/// let cfg = BakeConfig::default();
/// let grid = GridLayout::from_input(&input, &cfg)?;
/// let mut job = BakeJob::new(&input, &cfg, &grid, grid.keys(), JobOptions::default())?;
/// let mut tiles = 0;
/// loop {
///     let progress = job.step(&mut Budget::new(10_000))?;
///     while let Some(tile) = job.next_ready() {
///         tiles += usize::from(tile.data.is_some());
///     }
///     if progress == Progress::Done {
///         break;
///     }
/// }
/// assert_eq!(tiles, 1);
/// # Ok(())
/// # }
/// ```
pub struct BakeJob<'s, S: GeometrySource> {
    src: &'s S,
    cfg: BakeConfig,
    grid: GridLayout,
    config_hash: u64,
    keys: Vec<TileKey>,
    next: usize,
    current: Option<(TileKey, u64, TileTask<'static>)>,
    ready: VecDeque<TileOutput>,
    delivered: Vec<ManifestEntry>,
    cancel: CancelToken,
    failed: Option<BakeError>,
    skipped: usize,
    opts: JobOptions,
    reports: super::BakeReport,
}

impl<'s, S: GeometrySource> BakeJob<'s, S> {
    /// Prepares a job for `keys`, baked with `cfg` on `grid`.
    ///
    /// `keys` are sorted into row-major order and deduplicated; the job returns tiles
    /// in that order. [`GridLayout::keys`] lists the whole grid.
    ///
    /// # Errors
    /// [`BakeError`] without a tile for an invalid `cfg` or a layout that does not
    /// match it, with the tile for a key outside the grid.
    pub fn new(
        src: &'s S,
        cfg: &BakeConfig,
        grid: &GridLayout,
        mut keys: Vec<TileKey>,
        opts: JobOptions,
    ) -> Result<Self, BakeError> {
        super::validate_config(cfg)?;
        grid.validate(cfg)?;
        if let Some(bad) = keys.iter().find(|k| !grid.contains(**k)) {
            return Err(BakeError {
                tile: Some([bad.x, bad.z]),
                error: Error::InvalidParameter("tile outside the grid"),
            });
        }
        keys.sort_unstable();
        keys.dedup();
        Ok(Self {
            src,
            cfg: *cfg,
            grid: *grid,
            config_hash: BakeManifest::config_hash(cfg, grid),
            keys,
            next: 0,
            current: None,
            ready: VecDeque::new(),
            delivered: Vec::new(),
            cancel: CancelToken::new(),
            failed: None,
            skipped: 0,
            opts,
            reports: super::BakeReport::default(),
        })
    }

    fn plan(&self) -> Plan<'_, S> {
        Plan {
            src: self.src,
            cfg: &self.cfg,
            grid: &self.grid,
            config_hash: self.config_hash,
            reports: self.opts.reports,
        }
    }

    /// Measurements for completed tiles and the first failed tile; independent of delivery.
    /// Empty unless [`JobOptions::reports`] was enabled. Skipped and unstarted tiles
    /// produce no report. The caller can attach installed-world reachability to a clone.
    pub fn report(&self) -> &super::BakeReport {
        &self.reports
    }

    /// A token that stops the job from any thread. Cancelling ends the job: later
    /// calls return [`Error::Cancelled`], [`next_ready`](Self::next_ready) returns
    /// nothing, and no partial tile ever appears. Tiles already returned stay valid.
    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    /// Tiles dropped by [`skip_unchanged`](Self::skip_unchanged).
    pub fn skipped(&self) -> usize {
        self.skipped
    }

    /// Whether every tile has been baked (finished tiles may still await
    /// [`next_ready`](Self::next_ready)).
    pub fn is_complete(&self) -> bool {
        self.failed.is_none() && self.current.is_none() && self.next >= self.keys.len()
    }

    fn check(&self) -> Result<(), BakeError> {
        if let Some(e) = &self.failed {
            return Err(e.clone());
        }
        if self.cancel.is_cancelled() {
            return Err(BakeError {
                tile: None,
                error: Error::Cancelled,
            });
        }
        Ok(())
    }

    /// Drops the keys whose input is unchanged since `prev` (a hash-only pass).
    ///
    /// Fetches each queued tile's column and compares its input hash with `prev`'s
    /// entry. A `prev` baked with a different configuration or layout (anything but the
    /// grid size) drops nothing and the call returns `Ok(false)`: every tile of the job is
    /// rebaked, and [`BakeManifest::retarget`] must move the manifest before
    /// [`BakeManifest::apply`] accepts the job's delta. Otherwise returns `Ok(true)`.
    /// Call before the first [`step`](Self::step). `ws` limits the pass.
    ///
    /// # Errors
    /// [`BakeError`] for a source failure, a limit, a cancel, or a job already started.
    pub fn skip_unchanged(
        &mut self,
        prev: &BakeManifest,
        ws: &mut Workspace,
    ) -> Result<bool, BakeError> {
        self.check()?;
        if self.next != 0 || self.current.is_some() {
            return Err(BakeError {
                tile: None,
                error: Error::InvalidParameter("skip_unchanged after the job started"),
            });
        }
        if prev.version != MANIFEST_VERSION
            || prev.config_hash != self.config_hash
            || !prev.grid.same_layout(&self.grid)
        {
            return Ok(false);
        }
        ws.begin();
        let plan = self.plan();
        let mut kept = Vec::with_capacity(self.keys.len());
        for &key in &self.keys {
            let at = |error: Error| BakeError {
                tile: Some([key.x, key.z]),
                error,
            };
            let fetched = plan.fetch(key, &self.cancel).map_err(at)?;
            ws.work(fetched.items().saturating_add(1)).map_err(at)?;
            if prev.entry(key).map(|e| e.input_hash) != Some(fetched.input_hash) {
                kept.push(key);
            }
        }
        self.skipped += self.keys.len() - kept.len();
        self.keys = kept;
        Ok(true)
    }

    /// Runs whole stages until the budget has no work left, a tile fails or the job ends.
    ///
    /// Returns [`Progress::Done`] when every tile is baked and [`Progress::Exhausted`]
    /// when `budget` ran out first; call again with a new allowance. A stage is not
    /// interrupted, so the last one may overshoot: the budget is then marked fully spent.
    /// At least one stage runs whenever the budget has work left, so a loop that
    /// refills the budget always finishes. The result does not depend on the budgets.
    ///
    /// # Errors
    /// The first failing tile in row-major order (tiles before it stay available through
    /// [`next_ready`](Self::next_ready)), or [`Error::Cancelled`].
    pub fn step(&mut self, budget: &mut Budget) -> Result<Progress, BakeError> {
        loop {
            self.check()?;
            if self.current.is_none() && self.next >= self.keys.len() {
                return Ok(Progress::Done);
            }
            if budget.remaining() == 0 {
                budget.exhaust();
                return Ok(Progress::Exhausted);
            }
            let cost = self.run_one_stage()?;
            if !budget.spend(cost) {
                budget.exhaust();
                return Ok(Progress::Exhausted);
            }
        }
    }

    /// One unit of `step`: fetch the next tile, or advance the tile in flight by one stage.
    fn run_one_stage(&mut self) -> Result<u64, BakeError> {
        let Some((key, hash, task)) = &mut self.current else {
            self.admit(1).map_err(|e| self.fail(e))?;
            let key = self.keys[self.next];
            let started = (|| {
                let plan = self.plan();
                let fetched = plan.fetch(key, &self.cancel)?;
                let (cost, hash) = (fetched.items(), fetched.input_hash);
                let task = plan.task(key, fetched, &self.cancel)?;
                Ok::<_, Error>((cost, hash, task))
            })();
            return match started {
                Ok((cost, hash, task)) => {
                    self.next += 1;
                    self.current = Some((key, hash, task));
                    Ok(cost)
                }
                Err(error) => Err(self.fail(BakeError {
                    tile: Some([key.x, key.z]),
                    error,
                })),
            };
        };
        let (key, hash) = (*key, *hash);
        let before = task.work_used();
        match task.advance() {
            Ok(Advance::More) => Ok(task.work_used() - before),
            Ok(Advance::Done(data, triangulation)) => {
                let cost = task.work_used() - before;
                if let Some(report) = task.report.take() {
                    self.reports.tiles.push(report);
                }
                self.current = None;
                self.ready.push_back(output(key, hash, data, triangulation));
                Ok(cost)
            }
            Err(error) => {
                if let Some(mut report) = task.report.take() {
                    report.fail(&error);
                    self.reports.tiles.push(report);
                }
                self.current = None;
                Err(self.fail(BakeError {
                    tile: Some([key.x, key.z]),
                    error,
                }))
            }
        }
    }

    fn fail(&mut self, error: BakeError) -> BakeError {
        if self.opts.reports
            && let Some([x, z]) = error.tile
            && self
                .reports
                .tiles
                .last()
                .is_none_or(|r| r.key.x != x || r.key.z != z)
            && let Ok(cfg) = self.grid.tile_config(&self.cfg, TileKey { x, z })
        {
            let mut report = super::TileReport::new(x, z, cfg.width, cfg.height, cfg.border_size);
            report.fail(&error.error);
            self.reports.tiles.push(report);
        }
        self.current = None;
        self.failed = Some(error.clone());
        error
    }

    /// Bakes one chunk of at most [`JobOptions::max_in_flight`] tiles on the rayon pool
    /// (the calling thread without the `parallel` feature), in the order of
    /// [`tiles_parallel`](super::tiles_parallel). A tile begun by [`step`](Self::step)
    /// is finished serially first.
    ///
    /// Returns [`Progress::Done`] when no tile remains and [`Progress::Exhausted`] when
    /// more chunks follow. Finished tiles are available from
    /// [`next_ready`](Self::next_ready) after each call.
    ///
    /// # Errors
    /// The first failing tile of the chunk in row-major order, or [`Error::Cancelled`].
    pub fn run_parallel_chunk(&mut self) -> Result<Progress, BakeError> {
        self.check()?;
        while self.current.is_some() {
            self.run_one_stage()?;
        }
        if self.next >= self.keys.len() {
            return Ok(Progress::Done);
        }
        let chunk = match self.opts.max_in_flight {
            0 => worker_count().saturating_mul(2),
            n => n,
        }
        .max(1);
        let end = self.keys.len().min(self.next.saturating_add(chunk));
        self.admit(end - self.next).map_err(|e| self.fail(e))?;
        let plan = self.plan();
        let results = map_ordered(&self.keys[self.next..end], |key| {
            plan.bake_tile(*key, &self.cancel)
        });
        self.next = end;
        for result in results {
            match result {
                (Ok(tile), report) => {
                    if let Some(report) = report {
                        self.reports.tiles.push(report);
                    }
                    self.ready.push_back(tile);
                }
                (Err(e), report) => {
                    if let Some(report) = report {
                        self.reports.tiles.push(report);
                    }
                    return Err(self.fail(e));
                }
            }
        }
        Ok(if self.next >= self.keys.len() {
            Progress::Done
        } else {
            Progress::Exhausted
        })
    }

    /// Bakes every remaining tile with [`run_parallel_chunk`](Self::run_parallel_chunk).
    /// Same bytes and order as [`step`](Self::step) and [`tiles`](super::tiles).
    ///
    /// # Errors
    /// As [`run_parallel_chunk`](Self::run_parallel_chunk).
    pub fn run_parallel(&mut self) -> Result<(), BakeError> {
        while self.run_parallel_chunk()? != Progress::Done {}
        Ok(())
    }

    /// The next finished tile, in row-major order, or `None` when none is ready (or the
    /// job was cancelled). Each tile appears once; it enters
    /// [`manifest_delta`](Self::manifest_delta) when returned.
    pub fn next_ready(&mut self) -> Option<TileOutput> {
        if self.cancel.is_cancelled() {
            return None;
        }
        let tile = self.ready.pop_front()?;
        self.delivered.push(ManifestEntry {
            key: tile.key,
            input_hash: tile.input_hash,
            output_hash: tile.output_hash,
        });
        Some(tile)
    }

    /// Manifest entries of the tiles returned by [`next_ready`](Self::next_ready) so far,
    /// in row-major order. [`BakeManifest::apply`] merges them into the previous manifest.
    pub fn manifest_delta(&self) -> ManifestDelta {
        ManifestDelta {
            config_hash: self.config_hash,
            entries: self.delivered.clone(),
        }
    }

    /// Checks `max_total_bytes` for `in_flight` tiles about to start: the finished tiles
    /// not yet taken, the tiles in flight and their working storage, as for
    /// [`tiles`](super::tiles).
    fn admit(&self, in_flight: usize) -> Result<(), BakeError> {
        let report_bytes = self
            .reports
            .tiles
            .iter()
            .try_fold(0usize, |n, r| {
                n.checked_add(core::mem::size_of::<super::TileReport>())
                    .and_then(|n| {
                        r.islands
                            .len()
                            .checked_mul(core::mem::size_of::<crate::connectivity::IslandSummary>())
                            .and_then(|v| n.checked_add(v))
                    })
            })
            .ok_or(BakeError {
                tile: None,
                error: Error::SizeOverflow,
            })?;
        let mut cfg = self.cfg;
        if cfg.max_total_bytes != usize::MAX {
            cfg.max_total_bytes =
                cfg.max_total_bytes
                    .checked_sub(report_bytes)
                    .ok_or(BakeError {
                        tile: None,
                        error: Error::LimitExceeded,
                    })?;
        }
        let retained = self.ready.len().saturating_add(in_flight);
        super::check_total_bytes(&cfg, 0, retained, in_flight)
            .map_err(|error| BakeError { tile: None, error })
    }
}

#[cfg(feature = "parallel")]
fn worker_count() -> usize {
    rayon::current_num_threads()
}

#[cfg(not(feature = "parallel"))]
fn worker_count() -> usize {
    1
}

/// Maps `f` over `items`, in parallel with the `parallel` feature, keeping the order.
fn map_ordered<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync + Send) -> Vec<R> {
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        items.par_iter().map(f).collect()
    }
    #[cfg(not(feature = "parallel"))]
    {
        items.iter().map(f).collect()
    }
}
