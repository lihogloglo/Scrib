//! [`RecastBackend`]: `sf_nav`'s [`NavBackend`] over `navmesh-rs` (pure Rust Recast + Detour,
//! recastnavigation `9f4ce644`; nav.md "Reuse", NV0 spike). Bevy-free. Built in NV2:
//!
//! - **Meshes**: one Detour `NavMesh` per (worldspace, agent class, nav region), created on its
//!   first tile and dropped with its last. Tiles are DNAV v7 bytes with 64-bit refs, square, of the
//!   class's [`BuildConfig::tile_size`], on a grid whose origin is the region's.
//! - **Regions** (nav.md "Nav regions", landmass's islands): each region has a [`WorldPos`]
//!   origin ([`RecastBackend::set_region_origin`], default the worldspace's), and a mesh works in
//!   region-local f32 meters; [`Location`]s convert at the seam, so precision never depends on how
//!   far the region is from the worldspace origin.
//! - **Refs**: our [`PolyRef`] is the Detour 64-bit ref with the mesh slot and its generation in
//!   tile-index bits a mesh never uses ([`refs`](crate::refs)); stale refs fail by Detour's salt
//!   or our generation.
//! - **Bake** ([`NavBackend::build_tile`]): Recast's per-stage tile pipeline, flags from the
//!   adapter's [`AreaTable`] ([`build`](crate::build)).
//! - **Queries**: `nearest` (`findNearestPoly`), sliced A* (`updateSlicedFindPath`) with the
//!   search state in the job, string pulling (`findStraightPath`), `raycast`, polygon state
//!   patches (`setPolyArea`/`setPolyFlags`, no rebuild). Goals: [`search`](crate::search).
//! - **Sliced A* state**: each job owns a Detour `DetachedQuery` (node pool, open list, sliced
//!   state; our vendored navmesh-rs patch makes it public) and attaches it to the mesh only for one
//!   step, so `find_path(&self, ..)` serves many workers with no `unsafe` and no per-worker query
//!   bound to a mesh. Finished jobs return their query to a small pool.
//!
//! Mesh changes follow the trait's contract through Detour's epoch: tiles added between steps are
//! tolerated (the search may miss them), removing a tile the search touched fails the job with
//! [`NavError::MeshChanged`] (navmesh-rs `Error::MeshChanged`), and so does dropping its mesh.
#![allow(
    clippy::many_single_char_names,
    reason = "geometry and Detour refs read best as a, b, p, q, r, as in Detour's sources"
)]

mod build;
mod filter;
mod refs;
mod search;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

use glam::Vec3;
use navmesh_rs::detour::{self, DetachedQuery, NavMesh, NavMeshParams, NavMeshQuery};
use sf_coords::WorldPos;
use sf_nav::{
    AgentClassId, AreaId, AreaTable, BlobPoly, BlobTile, BuildConfig, DebugPoly, JobState,
    NavBackend, NavError, NavPoint, NavRay, PathJob, PolyEdge, PolyFlags, PolyRef, PolyState,
    QueryFilter, SliceBudget, TileGeometry, TileInput, TileKey, TileRef, TileSide, TileSummary,
};
use sf_world::{Location, WorldspaceId};

use crate::filter::DtFilter;
use crate::refs::MeshTag;

/// Names this backend's tile format and bake settings for the bake's cache key (`sf_nav_bake`
/// `NavBakeConfig::backend_tag`). Bump the suffix when `build.rs` changes what a tile holds.
pub const BAKE_TAG: &str = "navmesh-rs 0.2.2 dnav7 poly64; sf_nav_recast build 1";

/// Most tiles one mesh holds at once (Detour slots): 2^13 = 8,192 tiles, ~120 MiB of humanoid
/// tiles at NV0's ~15 KiB each, past the High residency budget (nav.md "Budgets").
pub const MAX_TILES_PER_MESH: u32 = 1 << 13;

/// Most meshes resident at once: (worldspace, class, region) triples with a tile loaded.
pub const MAX_MESHES: usize = 1 << refs::SLOT_BITS;

/// Detour's 64-bit ref layout caps polygons per tile at 2^20.
const MAX_POLYS_PER_TILE: i32 = 1 << 20;

/// Detached queries kept for reuse; more are allocated on demand and dropped when returned.
const QUERY_POOL: usize = 16;

/// Backend settings.
#[derive(Clone, Debug, PartialEq)]
pub struct RecastConfig {
    /// Tile size per agent class: only classes listed here get meshes.
    pub classes: Vec<BuildConfig>,
    /// Detour slots per mesh, `1..=MAX_TILES_PER_MESH`.
    pub max_tiles: u32,
    /// A* search nodes per query (Detour `maxNodes`; NV0 used 4,096 for 350 m Balmora paths).
    pub max_nodes: u16,
    /// Most polygons in a path corridor; longer paths end `Partial` at the cap.
    pub max_path: usize,
}

impl RecastConfig {
    pub fn new(classes: Vec<BuildConfig>) -> Self {
        Self {
            classes,
            max_tiles: MAX_TILES_PER_MESH,
            max_nodes: 4096,
            max_path: 2048,
        }
    }

    fn validate(&self) -> Result<(), NavError> {
        if !(1..=MAX_TILES_PER_MESH).contains(&self.max_tiles)
            || self.max_nodes == 0
            || self.max_path == 0
        {
            return Err(NavError::InvalidConfig(format!(
                "max_tiles {} (1..={MAX_TILES_PER_MESH}), max_nodes {}, max_path {}",
                self.max_tiles, self.max_nodes, self.max_path
            )));
        }
        for (i, c) in self.classes.iter().enumerate() {
            c.validate()?;
            if self.classes[..i].iter().any(|o| o.class.id == c.class.id) {
                return Err(NavError::InvalidConfig(format!(
                    "agent class {:?} listed twice",
                    c.class.id
                )));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MeshKey {
    ws: WorldspaceId,
    class: AgentClassId,
    region: u16,
}

impl MeshKey {
    fn of(t: &TileKey) -> Self {
        Self {
            ws: t.ws.clone(),
            class: t.class,
            region: t.region,
        }
    }
}

pub(crate) struct Mesh {
    key: MeshKey,
    tag: MeshTag,
    /// Unique over the backend's life: a job started on a mesh that was dropped (and maybe
    /// re-created in the same slot) fails instead of reading the new one.
    serial: u64,
    origin: WorldPos,
    /// Boxed: navmesh-rs's resumable searches identify their navmesh by address.
    nav: Box<NavMesh>,
    tiles: HashMap<(i32, i32), detour::TileRef>,
}

impl Mesh {
    fn local(&self, p: WorldPos) -> [f32; 3] {
        self.origin.delta_f32(p).to_array()
    }

    fn world(&self, ws: &WorldspaceId, p: [f32; 3]) -> Location {
        Location::new(ws.clone(), self.origin.offset(Vec3::from_array(p)))
    }

    fn poly(&self, r: detour::PolyRef) -> PolyRef {
        refs::poly(self.tag, r)
    }
}

/// The navmesh-rs backend (nav.md "Interfaces"). Queries take `&self` and may run on many
/// threads; tile and polygon changes take `&mut self` and apply between ticks.
pub struct RecastBackend {
    cfg: RecastConfig,
    meshes: Vec<Option<Mesh>>,
    generations: Vec<u8>,
    by_key: BTreeMap<MeshKey, usize>,
    origins: HashMap<(WorldspaceId, u16), WorldPos>,
    next_serial: u64,
    queries: Mutex<Vec<DetachedQuery>>,
}

impl RecastBackend {
    pub fn new(cfg: RecastConfig) -> Result<Self, NavError> {
        cfg.validate()?;
        Ok(Self {
            cfg,
            meshes: (0..MAX_MESHES).map(|_| None).collect(),
            generations: vec![0; MAX_MESHES],
            by_key: BTreeMap::new(),
            origins: HashMap::new(),
            next_serial: 0,
            queries: Mutex::new(Vec::new()),
        })
    }

    pub fn config(&self) -> &RecastConfig {
        &self.cfg
    }

    /// Sets a nav region's origin (worldspace origin by default). Tiles of that region are in
    /// meters from it; changing it while one is resident fails.
    pub fn set_region_origin(
        &mut self,
        ws: &WorldspaceId,
        region: u16,
        origin: WorldPos,
    ) -> Result<(), NavError> {
        let busy = self
            .by_key
            .keys()
            .any(|k| &k.ws == ws && k.region == region);
        let old = self.region_origin(ws, region);
        if busy && old != origin {
            return Err(NavError::InvalidConfig(format!(
                "region {region} of {ws:?} has resident tiles: its origin is fixed"
            )));
        }
        self.origins.insert((ws.clone(), region), origin);
        Ok(())
    }

    pub fn region_origin(&self, ws: &WorldspaceId, region: u16) -> WorldPos {
        self.origins
            .get(&(ws.clone(), region))
            .copied()
            .unwrap_or(WorldPos::ORIGIN)
    }

    /// Every polygon of a resident tile, in Detour order (door polygon lists, debug draw).
    pub fn tile_polys(&self, key: &TileKey) -> Vec<PolyRef> {
        let Some(m) = self.mesh_of_key(&MeshKey::of(key)) else {
            return Vec::new();
        };
        let Some(&tr) = m.tiles.get(&(key.x, key.z)) else {
            return Vec::new();
        };
        let n = m.nav.tile_by_ref(tr).map_or(0, |t| t.polys.len());
        (0..n)
            .map(|i| {
                #[allow(
                    clippy::cast_possible_truncation,
                    reason = "Detour caps polygons per tile at 2^20"
                )]
                m.poly(tr | i as u64)
            })
            .collect()
    }

    /// Resident tiles over all meshes.
    pub fn resident_tiles(&self) -> usize {
        self.live_meshes().map(|m| m.tiles.len()).sum()
    }

    /// Bytes of resident tile data (DNAV) over all meshes; decoded arrays take about as much
    /// again (NV0).
    pub fn resident_bytes(&self) -> usize {
        self.live_meshes()
            .flat_map(|m| m.tiles.values().filter_map(|&t| m.nav.tile_by_ref(t)))
            .map(|t| t.data.len())
            .sum()
    }

    fn live_meshes(&self) -> impl Iterator<Item = &Mesh> {
        self.meshes.iter().flatten()
    }

    fn mesh_of_key(&self, k: &MeshKey) -> Option<&Mesh> {
        self.by_key.get(k).and_then(|&s| self.meshes[s].as_ref())
    }

    pub(crate) fn mesh(&self, tag: MeshTag) -> Option<&Mesh> {
        self.meshes.get(tag.slot)?.as_ref().filter(|m| m.tag == tag)
    }

    pub(crate) fn mesh_by_serial(&self, slot: usize, serial: u64) -> Option<&Mesh> {
        self.meshes
            .get(slot)?
            .as_ref()
            .filter(|m| m.serial == serial)
    }

    /// The mesh and Detour ref of one of our refs, if still valid.
    pub(crate) fn resolve(&self, p: PolyRef) -> Option<(&Mesh, detour::PolyRef)> {
        let (tag, r) = refs::split(p);
        self.mesh(tag)
            .filter(|m| m.nav.is_valid_poly_ref(r))
            .map(|m| (m, r))
    }

    fn class_config(&self, class: AgentClassId) -> Option<&BuildConfig> {
        self.cfg.classes.iter().find(|c| c.class.id == class)
    }

    /// A detached query from the pool, or a new one.
    pub(crate) fn take_query(&self, nav: &NavMesh) -> Result<DetachedQuery, NavError> {
        let pooled = self.queries.lock().ok().and_then(|mut v| v.pop());
        match pooled {
            Some(q) => Ok(q),
            None => DetachedQuery::new(nav, usize::from(self.cfg.max_nodes)).map_err(backend),
        }
    }

    pub(crate) fn give_query(&self, q: DetachedQuery) {
        if let Ok(mut v) = self.queries.lock()
            && v.len() < QUERY_POOL
        {
            v.push(q);
        }
    }

    /// Runs `f` with a query over `mesh` from the pool.
    pub(crate) fn with_query<R>(
        &self,
        mesh: &Mesh,
        f: impl FnOnce(&mut NavMeshQuery<'_>) -> R,
    ) -> Result<R, NavError> {
        let mut q = self.take_query(&mesh.nav)?.attach(&mesh.nav);
        let r = f(&mut q);
        self.give_query(q.detach());
        Ok(r)
    }

    /// The passing polygon nearest `at` in any region mesh of (worldspace, class): its mesh,
    /// Detour ref and mesh-local point. Ties go to the lower region.
    pub(crate) fn nearest_in(
        &self,
        at: &Location,
        class: AgentClassId,
        extent: Vec3,
        filter: &QueryFilter,
        only: Option<usize>,
    ) -> Option<(&Mesh, detour::PolyRef, [f32; 3])> {
        if !extent.is_finite() || extent.min_element() < 0.0 {
            return None;
        }
        let lo = MeshKey {
            ws: at.worldspace.clone(),
            class,
            region: 0,
        };
        let hi = MeshKey {
            region: u16::MAX,
            ..lo.clone()
        };
        let f = DtFilter(filter);
        let mut best: Option<(f32, &Mesh, detour::PolyRef, [f32; 3])> = None;
        for (_, &slot) in self.by_key.range(lo..=hi) {
            if only.is_some_and(|o| o != slot) {
                continue;
            }
            let Some(m) = self.meshes[slot].as_ref() else {
                continue;
            };
            let p = m.local(at.pos);
            let Ok(Ok((r, q, _))) =
                self.with_query(m, |q| q.find_nearest_poly(&p, &extent.to_array(), &f))
            else {
                continue;
            };
            if r == 0 {
                continue;
            }
            let d = Vec3::from_array(q).distance_squared(Vec3::from_array(p));
            if best.as_ref().is_none_or(|b| d < b.0) {
                best = Some((d, m, r, q));
            }
        }
        best.map(|(_, m, r, q)| (m, r, q))
    }
}

/// Detour's raycast reports "reached the end" as `t == FLT_MAX`, an exact sentinel.
#[allow(clippy::float_cmp, reason = "Detour's exact FLT_MAX sentinel")]
fn is_clear(hit: &detour::RaycastHit) -> bool {
    hit.t == f32::MAX
}

pub(crate) fn backend(e: navmesh_rs::Error) -> NavError {
    match e {
        navmesh_rs::Error::MeshChanged => NavError::MeshChanged,
        e => NavError::Backend(e.to_string()),
    }
}

fn bad_tile(key: &TileKey, reason: impl Into<String>) -> NavError {
    NavError::InvalidTile {
        key: Box::new(key.clone()),
        reason: reason.into(),
    }
}

impl NavBackend for RecastBackend {
    /// Recast's tile pipeline ([`build`](crate::build)). An empty `Vec` means the tile has no
    /// polygons (open sea, void): the bake stores no object and [`NavBackend::add_tile`]
    /// rejects it.
    fn build_tile(
        &self,
        input: &TileInput,
        cfg: &BuildConfig,
        areas: &AreaTable,
    ) -> Result<Vec<u8>, NavError> {
        build::build_tile(input, cfg, areas)
    }

    /// Decodes the DNAV bytes (navmesh-rs `MeshTile::parse`, which validates every index). Edge
    /// links are Detour's `neis`: 0 a wall, `i + 1` polygon `i`, `EXT_LINK | side` a portal
    /// (side 0 = +x, 2 = +z, 4 = -x, 6 = -z; `dtNavMesh` sides, our Z is Detour's Y).
    fn read_tile_blob(&self, blob: &[u8]) -> Result<BlobTile, NavError> {
        let t = detour::MeshTile::parse(blob.to_vec()).map_err(backend)?;
        let verts = t
            .verts
            .as_chunks::<3>()
            .0
            .iter()
            .map(|&v| Vec3::from_array(v))
            .collect();
        let edge = |nei: u16| {
            if nei & detour::EXT_LINK != 0 {
                match nei & 0xff {
                    0 => PolyEdge::Portal(TileSide::PosX),
                    2 => PolyEdge::Portal(TileSide::PosZ),
                    4 => PolyEdge::Portal(TileSide::NegX),
                    _ => PolyEdge::Portal(TileSide::NegZ),
                }
            } else if nei == 0 {
                PolyEdge::Wall
            } else {
                PolyEdge::Poly(u32::from(nei - 1))
            }
        };
        let polys = t
            .polys
            .iter()
            .map(|p| {
                let n = usize::from(p.vert_count);
                BlobPoly {
                    verts: p.verts[..n].iter().map(|&v| u32::from(v)).collect(),
                    edges: p.neis[..n].iter().map(|&e| edge(e)).collect(),
                    state: PolyState {
                        area: AreaId(p.area()),
                        flags: PolyFlags::from_bits_retain(p.flags),
                    },
                }
            })
            .collect();
        Ok(BlobTile { verts, polys })
    }

    fn add_tile(&mut self, key: TileKey, blob: &[u8]) -> Result<TileRef, NavError> {
        let size = self
            .class_config(key.class)
            .ok_or_else(|| bad_tile(&key, "no build config for its agent class"))?
            .tile_size();
        if blob.is_empty() {
            return Err(bad_tile(&key, "empty tile (no polygons)"));
        }
        let mk = MeshKey::of(&key);
        let slot = match self.by_key.get(&mk) {
            Some(&s) => s,
            None => self.new_mesh(&key, mk, size)?,
        };
        let m = self.meshes[slot]
            .as_mut()
            .expect("slot just found or filled");
        if m.tiles.contains_key(&(key.x, key.z)) {
            return Err(NavError::TileLoaded(Box::new(key)));
        }
        let added = m.nav.add_tile(blob.to_vec());
        let checked = added
            .map_err(|e| bad_tile(&key, e.to_string()))
            .and_then(|tr| {
                let h = &m.nav.tile_by_ref(tr).expect("just added").header;
                let (hx, hy, hl) = (h.x, h.y, h.layer);
                if (hx, hy, hl) == (key.x, key.z, 0) {
                    Ok(tr)
                } else {
                    let _ = m.nav.remove_tile(tr);
                    Err(bad_tile(
                        &key,
                        format!("the blob is tile ({hx}, {hy}) layer {hl}"),
                    ))
                }
            });
        match checked {
            Ok(tr) => {
                m.tiles.insert((key.x, key.z), tr);
                Ok(refs::tile(m.tag, tr))
            }
            Err(e) => {
                if m.tiles.is_empty() {
                    self.drop_mesh(slot);
                }
                Err(e)
            }
        }
    }

    fn remove_tile(&mut self, key: &TileKey) -> bool {
        let Some(&slot) = self.by_key.get(&MeshKey::of(key)) else {
            return false;
        };
        let Some(m) = self.meshes[slot].as_mut() else {
            return false;
        };
        let Some(tr) = m.tiles.remove(&(key.x, key.z)) else {
            return false;
        };
        // Only fails for a ref this mesh never handed out; the slot's salt advances either way.
        let _ = m.nav.remove_tile(tr);
        if m.tiles.is_empty() {
            self.drop_mesh(slot);
        }
        true
    }

    fn tile(&self, key: &TileKey) -> Option<TileRef> {
        let m = self.mesh_of_key(&MeshKey::of(key))?;
        m.tiles.get(&(key.x, key.z)).map(|&t| refs::tile(m.tag, t))
    }

    fn tile_summaries(&self) -> Vec<TileSummary> {
        self.live_meshes()
            .flat_map(|m| {
                m.tiles.iter().filter_map(|(&(x, z), &tr)| {
                    let t = m.nav.tile_by_ref(tr)?;
                    Some(TileSummary {
                        key: TileKey {
                            ws: m.key.ws.clone(),
                            class: m.key.class,
                            region: m.key.region,
                            x,
                            z,
                        },
                        bytes: t.data.len(),
                        polys: t
                            .polys
                            .iter()
                            .filter(|p| p.poly_type() != detour::POLYTYPE_OFFMESH_CONNECTION)
                            .count(),
                    })
                })
            })
            .collect()
    }

    /// The tile's ground polygons (off-mesh connections are links, drawn by their own layer);
    /// an edge is a wall where Detour stores no neighbour (`neis == 0`), as Detour's
    /// `duDebugDrawNavMeshPolyBoundaries` draws outer edges.
    fn tile_geometry(&self, key: &TileKey) -> Option<TileGeometry> {
        let m = self.mesh_of_key(&MeshKey::of(key))?;
        let t = m.nav.tile_by_ref(*m.tiles.get(&(key.x, key.z))?)?;
        let verts = t
            .verts
            .as_chunks::<3>()
            .0
            .iter()
            .map(|&v| Vec3::from_array(v))
            .collect();
        let polys = t
            .polys
            .iter()
            .filter(|p| p.poly_type() != detour::POLYTYPE_OFFMESH_CONNECTION)
            .map(|p| {
                let n = usize::from(p.vert_count);
                let walls = (0..n.min(8))
                    .filter(|&i| p.neis[i] == 0)
                    .fold(0u8, |w, i| w | 1 << i);
                DebugPoly {
                    verts: p.verts[..n].iter().map(|&v| u32::from(v)).collect(),
                    walls,
                    state: PolyState {
                        area: AreaId(p.area()),
                        flags: PolyFlags::from_bits_retain(p.flags),
                    },
                }
            })
            .collect();
        Some(TileGeometry {
            origin: m.origin,
            verts,
            polys,
        })
    }

    fn nearest(
        &self,
        at: &Location,
        class: AgentClassId,
        extent: Vec3,
        filter: &QueryFilter,
    ) -> Option<NavPoint> {
        self.nearest_in(at, class, extent, filter, None)
            .map(|(m, r, p)| NavPoint {
                loc: m.world(&at.worldspace, p),
                poly: m.poly(r),
            })
    }

    fn find_path(&self, job: &mut PathJob, budget: &mut SliceBudget) -> JobState {
        search::find_path(self, job, budget)
    }

    fn raycast(
        &self,
        from: &NavPoint,
        to: &Location,
        filter: &QueryFilter,
    ) -> Result<NavRay, NavError> {
        let ws = &from.loc.worldspace;
        if &to.worldspace != ws {
            return Err(NavError::WrongWorldspace {
                query: ws.clone(),
                goal: to.worldspace.clone(),
            });
        }
        let (m, start) = self
            .resolve(from.poly)
            .ok_or(NavError::UnknownPoly(from.poly))?;
        if &m.key.ws != ws {
            return Err(NavError::UnknownPoly(from.poly));
        }
        let (a, b) = (m.local(from.loc.pos), m.local(to.pos));
        let f = DtFilter(filter);
        let max_path = self.cfg.max_path;
        let (hit, end) = self.with_query(m, |q| {
            let (hit, _) = q.raycast(start, &a, &b, &f, max_path).map_err(backend)?;
            let last = hit.path.last().copied().unwrap_or(start);
            let t = if is_clear(&hit) { 1.0 } else { hit.t };
            let at = Vec3::from_array(a).lerp(Vec3::from_array(b), t).to_array();
            let end = q.closest_point_on_poly(last, &at).map_or(at, |(p, _)| p);
            Ok::<_, NavError>((hit, end))
        })??;
        let clear = is_clear(&hit);
        Ok(NavRay {
            hit: !clear,
            t: if clear { 1.0 } else { hit.t.clamp(0.0, 1.0) },
            end: m.world(ws, end),
            polys: hit.path.iter().map(|&r| m.poly(r)).collect(),
        })
    }

    fn poly_state(&self, poly: PolyRef) -> Option<PolyState> {
        let (m, r) = self.resolve(poly)?;
        Some(PolyState {
            area: AreaId(m.nav.poly_area(r).ok()?),
            flags: PolyFlags::from_bits_retain(m.nav.poly_flags(r).ok()?),
        })
    }

    fn set_poly_state(&mut self, polys: &[PolyRef], state: PolyState) -> Result<(), NavError> {
        if state.area.index() >= sf_nav::MAX_AREAS {
            return Err(NavError::InvalidConfig(format!(
                "area id {} outside 0..{}",
                state.area.0,
                sf_nav::MAX_AREAS
            )));
        }
        if let Some(&p) = polys.iter().find(|&&p| self.resolve(p).is_none()) {
            return Err(NavError::UnknownPoly(p));
        }
        for &p in polys {
            let (tag, r) = refs::split(p);
            let m = self.meshes[tag.slot].as_mut().expect("resolved above");
            m.nav.set_poly_area(r, state.area.0).map_err(backend)?;
            m.nav
                .set_poly_flags(r, state.flags.bits())
                .map_err(backend)?;
        }
        Ok(())
    }
}

impl RecastBackend {
    /// Creates the mesh for `mk` in the lowest free slot.
    fn new_mesh(&mut self, key: &TileKey, mk: MeshKey, size: f32) -> Result<usize, NavError> {
        let slot = self
            .meshes
            .iter()
            .position(Option::is_none)
            .ok_or_else(|| bad_tile(key, format!("all {MAX_MESHES} mesh slots in use")))?;
        #[allow(clippy::cast_possible_wrap, reason = "max_tiles <= 2^13")]
        let params = NavMeshParams {
            orig: [0.0; 3],
            tile_width: size,
            tile_height: size,
            max_tiles: self.cfg.max_tiles as i32,
            max_polys: MAX_POLYS_PER_TILE,
        };
        let nav = NavMesh::new(&params).map_err(backend)?;
        self.meshes[slot] = Some(Mesh {
            tag: MeshTag {
                slot,
                generation: self.generations[slot],
            },
            serial: self.next_serial,
            origin: self.region_origin(&key.ws, key.region),
            nav: Box::new(nav),
            tiles: HashMap::new(),
            key: mk.clone(),
        });
        self.next_serial += 1;
        self.by_key.insert(mk, slot);
        Ok(slot)
    }

    fn drop_mesh(&mut self, slot: usize) {
        if let Some(m) = self.meshes[slot].take() {
            self.by_key.remove(&m.key);
            self.generations[slot] = (self.generations[slot] + 1) & ((1 << refs::GEN_BITS) - 1);
        }
    }
}
