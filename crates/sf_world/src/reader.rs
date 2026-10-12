//! Where the runtime reads the world index from, and the decoded-page
//! cache the fetch stage keeps.
//!
//! The scheduler only reads the always-resident [`CellDirectory`]; the
//! engine's fetch stage reads one [`IndexPage`] per cell load through an
//! [`IndexReader`] (bake products in a game, memory in tests) and keeps
//! decoded pages in a [`PageCache`], so an R0 promotion right after an R1
//! load is a cache hit.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use glam::IVec3;

use crate::{CellDirectory, IndexPage, PlacementFlags, WorldIndex, WorldspaceId};
use crate::index::{IndexCell, IndexedPlacement};
use crate::rings::Level;

/// Reads index blobs ([`CellDirectory::to_bytes`], [`IndexPage::to_bytes`]).
/// Called from IO tasks, so implementations must be thread-safe and may
/// block.
pub trait IndexReader: Send + Sync {
    /// The worldspace's directory, `Ok(None)` if it has none (not baked).
    fn directory(&self, ws: &WorldspaceId) -> Result<Option<Vec<u8>>, String>;
    /// One page, `Ok(None)` if the worldspace has no content there.
    fn page(&self, ws: &WorldspaceId, page: IVec3) -> Result<Option<Vec<u8>>, String>;
}

/// An [`IndexReader`] over indexes held in memory (tests, tools, worlds
/// generated at runtime).
#[derive(Clone, Debug, Default)]
pub struct MemoryIndex {
    directories: BTreeMap<WorldspaceId, Vec<u8>>,
    pages: BTreeMap<(WorldspaceId, [i32; 3]), Vec<u8>>,
}

impl MemoryIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds (or replaces) a worldspace's whole index.
    pub fn insert(&mut self, index: &WorldIndex) {
        let ws = index.directory.worldspace.clone();
        self.pages.retain(|(w, _), _| *w != ws);
        for page in &index.pages {
            self.pages.insert((ws.clone(), page.page.to_array()), page.to_bytes());
        }
        self.directories.insert(ws, index.directory.to_bytes());
    }
}

impl IndexReader for MemoryIndex {
    fn directory(&self, ws: &WorldspaceId) -> Result<Option<Vec<u8>>, String> {
        Ok(self.directories.get(ws).cloned())
    }

    fn page(&self, ws: &WorldspaceId, page: IVec3) -> Result<Option<Vec<u8>>, String> {
        Ok(self.pages.get(&(ws.clone(), page.to_array())).cloned())
    }
}

/// Decoded pages, least recently used evicted past a byte cap (the
/// "decoded-cell CPU cache" budget, streaming.md).
#[derive(Debug)]
pub struct PageCache {
    cap_bytes: u64,
    used: u64,
    tick: u64,
    pages: HashMap<(WorldspaceId, [i32; 3]), CachedPage>,
}

#[derive(Debug)]
struct CachedPage {
    page: Arc<IndexPage>,
    bytes: u64,
    used_at: u64,
}

impl PageCache {
    /// Default cap: 256 MiB.
    pub const DEFAULT_CAP: u64 = 256 * 1024 * 1024;

    pub fn new(cap_bytes: u64) -> Self {
        Self { cap_bytes, used: 0, tick: 0, pages: HashMap::new() }
    }

    /// Bytes held (estimate).
    pub fn used_bytes(&self) -> u64 {
        self.used
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn get(&mut self, ws: &WorldspaceId, page: IVec3) -> Option<Arc<IndexPage>> {
        self.tick += 1;
        let entry = self.pages.get_mut(&(ws.clone(), page.to_array()))?;
        entry.used_at = self.tick;
        Some(entry.page.clone())
    }

    /// Caches a decoded page; `encoded_len` is its blob size (the estimate
    /// scales it to decoded size). Evicts least recently used pages first.
    pub fn insert(&mut self, ws: &WorldspaceId, page: Arc<IndexPage>, encoded_len: usize) {
        self.tick += 1;
        // Decoded placements are about 3× their encoded size (strings
        // shared per page become per-placement `Arc`s and `RecordId`s).
        let bytes = encoded_len as u64 * 3 + 256;
        let key = (ws.clone(), page.page.to_array());
        if let Some(old) = self.pages.insert(key, CachedPage { page, bytes, used_at: self.tick }) {
            self.used -= old.bytes;
        }
        self.used += bytes;
        while self.used > self.cap_bytes && self.pages.len() > 1 {
            let oldest = self.pages.iter().min_by_key(|(_, p)| p.used_at).map(|(k, _)| k.clone());
            let Some(oldest) = oldest else { break };
            if let Some(p) = self.pages.remove(&oldest) {
                self.used -= p.bytes;
            }
        }
    }
}

/// Which of a cell's placements belong to each level, and so what one
/// level step adds or removes. Levels are cumulative (streaming.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LevelContent {
    /// [`CellDirectory::far_min_radius`].
    pub far_min_radius: f32,
    /// The far ring holds the cell's far-field proxy (far-field.md), not
    /// placements: every placement spawns at `Near`.
    pub proxies: bool,
}

impl LevelContent {
    /// Far ring = large placements (no far field).
    pub fn of(directory: &CellDirectory) -> Self {
        Self { far_min_radius: directory.far_min_radius, proxies: false }
    }

    /// Far ring = the cell's far-field proxy.
    #[must_use]
    pub fn with_proxies(self, proxies: bool) -> Self {
        Self { proxies, ..self }
    }

    /// The level at which `p` is spawned: `Far` for the far subset, `Near`
    /// for other visual content, `None` for editor-only helpers (never
    /// spawned in play).
    pub fn level_of(&self, p: &IndexedPlacement) -> Option<Level> {
        if p.placement.flags.contains(PlacementFlags::EDITOR_ONLY) {
            None
        } else if !self.proxies && IndexCell::is_far(p, self.far_min_radius) {
            Some(Level::Far)
        } else {
            Some(Level::Near)
        }
    }

    /// The placements one level step touches: for a load `from → to` the
    /// ones it spawns, for an unload `to → from` the ones it despawns (spawn
    /// level in `(lower, higher]`). A step between `Near` and `Active`
    /// touches every spawned placement: activation (colliders, scripts, AI)
    /// applies to the whole resident cell.
    pub fn step<'a>(
        &self,
        placements: &'a [IndexedPlacement],
        from: Level,
        to: Level,
    ) -> impl Iterator<Item = &'a IndexedPlacement> + 'a {
        let (lo, hi) = if from < to { (from, to) } else { (to, from) };
        let this = *self;
        placements.iter().filter(move |p| match this.level_of(p) {
            None => false,
            Some(_) if hi == Level::Active => true,
            Some(level) => level > lo && level <= hi,
        })
    }
}
