//! Budgeted installation of finished tiles into a navmesh (not in recastnavigation).

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::BakeError;
use super::job::TileOutput;
use super::manifest::TileKey;
use crate::detour::NavMesh;
use crate::{Budget, Error, Progress};

/// Work units one tile of `len` bytes costs: one per KiB of tile data, at least one.
///
/// Installing validates the tile and links it to its neighbours, both proportional to
/// the tile's size; `docs/VERIFICATION.md` lists the measured time per unit.
pub fn install_cost(len: usize) -> u64 {
    (len.div_ceil(1024) as u64).max(1)
}

/// Queues finished tiles and installs them into a [`NavMesh`] in slices of a work
/// [`Budget`] (not in recastnavigation).
///
/// [`NavMesh::add_tile`] validates and links a tile in one call, so a tile is the
/// smallest piece: one [`install`](Self::install) call takes at most the time of the
/// last tile it started, and the budget decides how many tiles that is. A budget of one
/// unit installs exactly one tile per call. The queue and the budget are the only
/// limits; the navmesh's own `max_tiles` (set from [`GridLayout::nav_mesh_params`]) is the
/// residency cap and is a parameter of the navmesh, not of this type.
///
/// **Rebakes.** A tile whose slot is already resident replaces it: the resident tile is
/// removed (its refs stop validating, as for [`NavMesh::remove_tile`]) and the new one
/// added, within one [`install`](Self::install) step. A tile without polygons
/// removes the resident tile of its key, if any, and is otherwise a no-op. The queue keeps
/// at most one entry per key, the newest push: an older queued version is dropped, not
/// installed, so a superseded job's tiles can never overwrite a newer job's. Entries
/// install in the order of their last push.
///
/// **Cancelled jobs.** Tiles are installed whole, one at a time. After a cancel the
/// navmesh holds, for each tile, a complete old or a complete new version, and
/// [`BakeJob::manifest_delta`](super::BakeJob::manifest_delta) lists the new ones.
/// [`clear`](Self::clear) drops what is still queued and says which keys it was.
///
/// [`GridLayout::nav_mesh_params`]: super::GridLayout::nav_mesh_params
#[derive(Debug, Default)]
pub struct TileInstaller {
    /// Queued entries in the order of their push; the key of the entry is `latest`'s.
    queue: BTreeMap<u64, (TileKey, Option<Vec<u8>>)>,
    /// Sequence number of the queued entry of each key.
    latest: BTreeMap<TileKey, u64>,
    next_seq: u64,
    installed: usize,
}

impl TileInstaller {
    /// An empty queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a finished tile, replacing a queued tile of the same key. A tile without
    /// polygons is queued as the removal of the resident tile of its key.
    pub fn push(&mut self, tile: TileOutput) {
        let seq = self.next_seq;
        self.next_seq += 1;
        if let Some(old) = self.latest.insert(tile.key, seq) {
            self.queue.remove(&old);
        }
        self.queue.insert(seq, (tile.key, tile.data));
    }

    /// Entries waiting to be installed (at most one per key).
    pub fn pending(&self) -> usize {
        self.queue.len()
    }

    /// Entries applied so far: tiles installed or replaced, and removals.
    pub fn installed(&self) -> usize {
        self.installed
    }

    /// Drops every queued entry and returns their keys, in row-major order. The navmesh
    /// keeps what it holds.
    pub fn clear(&mut self) -> Vec<TileKey> {
        self.queue.clear();
        core::mem::take(&mut self.latest).into_keys().collect()
    }

    /// Installs queued tiles, in the order queued, until `budget` has no work left.
    ///
    /// Returns [`Progress::Done`] when the queue is empty and [`Progress::Exhausted`] when
    /// the budget ran out first. At least one entry is applied whenever the budget has
    /// work left; the one that crosses the limit is applied whole and the budget is
    /// marked fully spent.
    ///
    /// # Errors
    /// A [`BakeError`] naming the tile whose bytes the navmesh refused (all or nothing: the
    /// navmesh keeps the tile it held for that key, which stays resident; the entry leaves
    /// the queue and later entries stay queued).
    pub fn install(
        &mut self,
        nav: &mut NavMesh,
        budget: &mut Budget,
    ) -> Result<Progress, BakeError> {
        loop {
            if self.queue.is_empty() {
                return Ok(Progress::Done);
            }
            if budget.remaining() == 0 {
                budget.exhaust();
                return Ok(Progress::Exhausted);
            }
            let Some((_, (key, data))) = self.queue.pop_first() else {
                return Ok(Progress::Done);
            };
            self.latest.remove(&key);
            let cost = data.as_ref().map_or(1, |d| install_cost(d.len()));
            apply(nav, key, data).map_err(|error| BakeError {
                tile: Some([key.x, key.z]),
                error,
            })?;
            self.installed += 1;
            if !budget.spend(cost) {
                budget.exhaust();
                return Ok(if self.queue.is_empty() {
                    Progress::Done
                } else {
                    Progress::Exhausted
                });
            }
        }
    }
}

/// Makes `data` the tile of `key` in `nav`, or removes the tile when `data` is `None`.
/// The resident tile is restored if the new one is refused.
fn apply(nav: &mut NavMesh, key: TileKey, data: Option<Vec<u8>>) -> Result<(), Error> {
    let resident = nav
        .tile_index_at(key.x, key.z, 0)
        .map(|i| nav.tile_ref(i))
        .filter(|&r| r != 0);
    let old = match resident {
        Some(r) => Some(nav.remove_tile(r)?),
        None => None,
    };
    let Some(data) = data else {
        return Ok(());
    };
    match nav.add_tile(data) {
        Ok(_) => Ok(()),
        Err(error) => {
            if let Some(old) = old {
                nav.add_tile(old)?;
            }
            Err(error)
        }
    }
}
