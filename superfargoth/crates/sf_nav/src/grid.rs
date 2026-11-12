//! [`GridBackend`]: the test double behind [`NavBackend`] (nav.md "Interfaces"), so the core,
//! `sf_ai` and the adapter test without Recast.
//!
//! A 2.5D heightfield of square cells on the XZ plane (Y up), one walkable surface per column (no
//! overhangs, no second floor, no clearance check: those are the real backend's, NV2). Each cell is
//! one polygon. Moves go to the 8 neighbours, never cutting a corner past a cell the filter
//! rejects, and only when the height step is at most the class's climb. A* is exact: edge cost is
//! the 3D distance between cell centres times the mean of both cells' area costs, the heuristic is
//! the straight distance times the filter's cheapest cost, and ties break deterministically
//! (lower f, then higher g, then lower ref). Only region 0 exists; its origin is the worldspace's.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    reason = "cell indices and grid coordinates are small (tile_cells <= 4096, worlds well under 2^24 cells); geometry reads best as a, b, c, p"
)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use glam::{IVec2, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::{
    AgentClassId, AreaId, AreaTable, BuildConfig, Goal, JobState, NavBackend, NavError, NavPoint,
    NavRay, PathJob, PathQuery, PathResult, PathStatus, PolyRef, PolyState, QueryFilter,
    SliceBudget, TileInput, TileKey, TileRef,
};

const BLOB_VERSION: u32 = 1;
const MAX_TILE_CELLS: u16 = 4096;
/// Barycentric slack: a cell centre on a shared triangle edge belongs to both triangles.
const EPS: f32 = 1e-5;
const FIELD_BITS: u32 = 24;
const FIELD_MASK: u64 = (1 << FIELD_BITS) - 1;
/// Neighbour order: the four sides, then the diagonals (each diagonal's two sides are 0..4).
const DIRS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(-1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, -1),
];

/// One walkable cell: its surface height and patchable state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridCell {
    pub y: f32,
    pub state: PolyState,
}

impl GridCell {
    pub fn new(y: f32, area: AreaId, flags: crate::PolyFlags) -> Self {
        Self {
            y,
            state: PolyState { area, flags },
        }
    }
}

/// The grid every tile of one backend shares.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridConfig {
    pub cell_size: f32,
    pub tile_cells: u16,
}

#[derive(Serialize, Deserialize)]
struct TileBlob {
    version: u32,
    class: AgentClassId,
    cell_size: f32,
    tile_cells: u16,
    tile_x: i32,
    tile_z: i32,
    max_climb: f32,
    cells: Vec<Option<GridCell>>,
}

struct LiveTile {
    key: TileKey,
    max_climb: f32,
    cells: Vec<Option<GridCell>>,
}

struct Slot {
    salt: u16,
    tile: Option<LiveTile>,
}

/// A resident cell, as A* and the ray see it.
#[derive(Clone, Copy)]
struct CellView {
    poly: PolyRef,
    at: IVec2,
    cell: GridCell,
    max_climb: f32,
}

/// The exact-A* grid test double. See the module docs for its rules.
pub struct GridBackend {
    cfg: GridConfig,
    slots: Vec<Slot>,
    free: Vec<u32>,
    by_key: HashMap<TileKey, u32>,
    /// Bumped by every tile removal: in-flight jobs fail with [`NavError::MeshChanged`].
    epoch: u64,
}

impl GridBackend {
    pub fn new(cfg: GridConfig) -> Result<Self, NavError> {
        if cfg.cell_size.is_nan()
            || cfg.cell_size <= 0.0
            || cfg.tile_cells == 0
            || cfg.tile_cells > MAX_TILE_CELLS
        {
            return Err(NavError::InvalidConfig(format!(
                "grid cell size {} m, {} cells per tile (1..={MAX_TILE_CELLS})",
                cfg.cell_size, cfg.tile_cells
            )));
        }
        Ok(Self {
            cfg,
            slots: Vec::new(),
            free: Vec::new(),
            by_key: HashMap::new(),
            epoch: 0,
        })
    }

    pub fn config(&self) -> GridConfig {
        self.cfg
    }

    /// Loads cells from text rows for tests: row `r` is `z = r`, column `c` is `x = c`, and
    /// `legend` maps each character to a cell (`None`: not walkable). Every tile the rows touch
    /// becomes resident for (`ws`, `class`).
    pub fn insert_rows(
        &mut self,
        ws: &WorldspaceId,
        class: AgentClassId,
        max_climb: f32,
        rows: &[&str],
        legend: impl Fn(char) -> Option<GridCell>,
    ) -> Result<(), NavError> {
        let n = i32::from(self.cfg.tile_cells);
        let mut tiles: HashMap<(i32, i32), Vec<Option<GridCell>>> = HashMap::new();
        for (z, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                let (gx, gz) = (x as i32, z as i32);
                let cells = tiles
                    .entry((gx.div_euclid(n), gz.div_euclid(n)))
                    .or_insert_with(|| vec![None; (n * n) as usize]);
                cells[(gz.rem_euclid(n) * n + gx.rem_euclid(n)) as usize] = legend(ch);
            }
        }
        let mut keys: Vec<_> = tiles.into_iter().collect();
        keys.sort_by_key(|(k, _)| *k);
        for ((x, z), cells) in keys {
            self.insert(
                TileKey {
                    ws: ws.clone(),
                    class,
                    region: 0,
                    x,
                    z,
                },
                max_climb,
                cells,
            )?;
        }
        Ok(())
    }

    /// The global cell a polygon is, when resident.
    pub fn cell_of(&self, poly: PolyRef) -> Option<IVec2> {
        self.view(poly).map(|v| v.at)
    }

    /// The polygon of a global cell, when resident and walkable.
    pub fn poly_at(&self, ws: &WorldspaceId, class: AgentClassId, at: IVec2) -> Option<PolyRef> {
        self.locate(ws, class, at).map(|v| v.poly)
    }

    /// A cell's centre on its surface, region-local meters.
    pub fn center(&self, at: IVec2, y: f32) -> Vec3 {
        let cs = self.cfg.cell_size;
        Vec3::new((at.x as f32 + 0.5) * cs, y, (at.y as f32 + 0.5) * cs)
    }

    fn insert(
        &mut self,
        key: TileKey,
        max_climb: f32,
        cells: Vec<Option<GridCell>>,
    ) -> Result<TileRef, NavError> {
        if key.region != 0 {
            return Err(NavError::Unsupported("GridBackend has region 0 only"));
        }
        if self.by_key.contains_key(&key) {
            return Err(NavError::TileLoaded(Box::new(key)));
        }
        let slot = if let Some(s) = self.free.pop() {
            s
        } else {
            if self.slots.len() as u64 > FIELD_MASK {
                return Err(NavError::InvalidConfig(
                    "more than 2^24 resident tiles".into(),
                ));
            }
            self.slots.push(Slot {
                salt: 1,
                tile: None,
            });
            (self.slots.len() - 1) as u32
        };
        self.by_key.insert(key.clone(), slot);
        let s = &mut self.slots[slot as usize];
        s.tile = Some(LiveTile {
            key,
            max_climb,
            cells,
        });
        Ok(TileRef(
            u64::from(s.salt) << 48 | u64::from(slot) << FIELD_BITS,
        ))
    }

    fn decode(poly: PolyRef) -> (u16, usize, usize) {
        (
            (poly.0 >> 48) as u16,
            ((poly.0 >> FIELD_BITS) & FIELD_MASK) as usize,
            (poly.0 & FIELD_MASK) as usize,
        )
    }

    fn tile_of(&self, poly: PolyRef) -> Option<(&LiveTile, usize)> {
        let (salt, slot, idx) = Self::decode(poly);
        let s = self.slots.get(slot)?;
        let tile = s.tile.as_ref().filter(|_| s.salt == salt)?;
        tile.cells.get(idx)?.as_ref()?;
        Some((tile, idx))
    }

    fn view(&self, poly: PolyRef) -> Option<CellView> {
        let (tile, idx) = self.tile_of(poly)?;
        let n = i32::from(self.cfg.tile_cells);
        let local = IVec2::new(idx as i32 % n, idx as i32 / n);
        let at = IVec2::new(tile.key.x, tile.key.z) * n + local;
        Some(CellView {
            poly,
            at,
            cell: tile.cells[idx]?,
            max_climb: tile.max_climb,
        })
    }

    fn locate(&self, ws: &WorldspaceId, class: AgentClassId, at: IVec2) -> Option<CellView> {
        let n = i32::from(self.cfg.tile_cells);
        let key = TileKey {
            ws: ws.clone(),
            class,
            region: 0,
            x: at.x.div_euclid(n),
            z: at.y.div_euclid(n),
        };
        let slot = *self.by_key.get(&key)?;
        let s = &self.slots[slot as usize];
        let tile = s.tile.as_ref()?;
        let idx = (at.y.rem_euclid(n) * n + at.x.rem_euclid(n)) as usize;
        let cell = tile.cells[idx]?;
        let poly = PolyRef(u64::from(s.salt) << 48 | u64::from(slot) << FIELD_BITS | idx as u64);
        Some(CellView {
            poly,
            at,
            cell,
            max_climb: tile.max_climb,
        })
    }

    fn cell_index(&self, p: f32) -> i32 {
        (p / self.cfg.cell_size).floor() as i32
    }

    /// `to` is enterable from `from`: resident, passes the filter, within the climb.
    fn step_ok(from: &CellView, to: Option<CellView>, f: &QueryFilter) -> Option<CellView> {
        to.filter(|t| {
            f.passes(t.cell.state.area, t.cell.state.flags)
                && (t.cell.y - from.cell.y).abs() <= from.max_climb
        })
    }

    fn top_surface(input: &TileInput, x: f32, z: f32, cos_slope: f32) -> Option<(f32, AreaId)> {
        let p = Vec2::new(x, z);
        let mut top: Option<(f32, bool, AreaId)> = None;
        for (tri, &area) in input.tris.iter().zip(&input.tri_areas) {
            let [a, b, c] = tri.map(|i| input.verts[i as usize]);
            let (a2, v0, v1) = (
                Vec2::new(a.x, a.z),
                Vec2::new(b.x - a.x, b.z - a.z),
                Vec2::new(c.x - a.x, c.z - a.z),
            );
            let d = v0.perp_dot(v1);
            if d.abs() < 1e-9 {
                continue; // vertical: a wall, never a top surface
            }
            let v2 = p - a2;
            let (s, t) = (v2.perp_dot(v1) / d, v0.perp_dot(v2) / d);
            if s < -EPS || t < -EPS || s + t > 1.0 + EPS {
                continue;
            }
            let y = a.y + s * (b.y - a.y) + t * (c.y - a.y);
            let walkable =
                area != AreaId::NULL && (b - a).cross(c - a).normalize().y.abs() >= cos_slope;
            if top.is_none_or(|(ty, _, _)| y > ty) {
                top = Some((y, walkable, area));
            }
        }
        top.filter(|t| t.1).map(|(y, _, area)| (y, area))
    }
}

fn local(loc: &Location) -> Vec3 {
    loc.pos.to_f64().as_vec3()
}

fn located(ws: &WorldspaceId, p: Vec3) -> Location {
    Location::new(ws.clone(), WorldPos::from_f64(p.as_dvec3()))
}

impl NavBackend for GridBackend {
    fn build_tile(
        &self,
        input: &TileInput,
        cfg: &BuildConfig,
        areas: &AreaTable,
    ) -> Result<Vec<u8>, NavError> {
        cfg.validate()?;
        if cfg.cell_size.to_bits() != self.cfg.cell_size.to_bits()
            || cfg.tile_cells != self.cfg.tile_cells
        {
            return Err(NavError::InvalidConfig(
                "build config does not match the grid".into(),
            ));
        }
        if input.tri_areas.len() != input.tris.len() {
            return Err(NavError::InvalidConfig("one area per triangle".into()));
        }
        if input
            .tris
            .iter()
            .flatten()
            .any(|&i| i as usize >= input.verts.len())
        {
            return Err(NavError::InvalidConfig(
                "triangle vertex index out of range".into(),
            ));
        }
        let n = i32::from(cfg.tile_cells);
        let cos_slope = cfg.class.max_slope_deg.to_radians().cos();
        let mut cells = Vec::with_capacity((n * n) as usize);
        for j in 0..n {
            for i in 0..n {
                let c = self.center(IVec2::new(input.tile_x * n + i, input.tile_z * n + j), 0.0);
                cells.push(Self::top_surface(input, c.x, c.z, cos_slope).and_then(
                    |(y, mut area)| {
                        for v in &input.volumes {
                            if v.contains(Vec3::new(c.x, y, c.z)) {
                                area = v.area;
                            }
                        }
                        (area != AreaId::NULL).then(|| GridCell::new(y, area, areas.flags(area)))
                    },
                ));
            }
        }
        let blob = TileBlob {
            version: BLOB_VERSION,
            class: cfg.class.id,
            cell_size: cfg.cell_size,
            tile_cells: cfg.tile_cells,
            tile_x: input.tile_x,
            tile_z: input.tile_z,
            max_climb: cfg.class.max_climb,
            cells,
        };
        postcard::to_allocvec(&blob).map_err(|e| NavError::InvalidConfig(e.to_string()))
    }

    fn add_tile(&mut self, key: TileKey, blob: &[u8]) -> Result<TileRef, NavError> {
        let bad = |key: &TileKey, reason: String| NavError::InvalidTile {
            key: Box::new(key.clone()),
            reason,
        };
        let b: TileBlob = postcard::from_bytes(blob).map_err(|e| bad(&key, e.to_string()))?;
        let n = usize::from(b.tile_cells);
        let reason = if b.version != BLOB_VERSION {
            Some(format!("blob version {}", b.version))
        } else if b.cell_size.to_bits() != self.cfg.cell_size.to_bits()
            || b.tile_cells != self.cfg.tile_cells
        {
            Some(format!("grid {} m × {} cells", b.cell_size, b.tile_cells))
        } else if b.cells.len() != n * n {
            Some(format!("{} cells", b.cells.len()))
        } else if (b.class, b.tile_x, b.tile_z) != (key.class, key.x, key.z) {
            Some(format!(
                "baked for class {} tile ({}, {})",
                b.class.0, b.tile_x, b.tile_z
            ))
        } else {
            None
        };
        if let Some(r) = reason {
            return Err(bad(&key, r));
        }
        self.insert(key, b.max_climb, b.cells)
    }

    fn remove_tile(&mut self, key: &TileKey) -> bool {
        let Some(slot) = self.by_key.remove(key) else {
            return false;
        };
        let s = &mut self.slots[slot as usize];
        s.tile = None;
        s.salt = s.salt.wrapping_add(1).max(1);
        self.free.push(slot);
        self.epoch += 1;
        true
    }

    fn tile(&self, key: &TileKey) -> Option<TileRef> {
        let slot = *self.by_key.get(key)?;
        Some(TileRef(
            u64::from(self.slots[slot as usize].salt) << 48 | u64::from(slot) << FIELD_BITS,
        ))
    }

    fn tile_summaries(&self) -> Vec<crate::TileSummary> {
        self.by_key
            .iter()
            .filter_map(|(key, &slot)| {
                let tile = self.slots[slot as usize].tile.as_ref()?;
                Some(crate::TileSummary {
                    key: key.clone(),
                    bytes: tile.cells.len() * size_of::<Option<GridCell>>(),
                    polys: tile.cells.iter().flatten().count(),
                })
            })
            .collect()
    }

    /// One square per walkable cell at its height; a side is a wall when the neighbour across it
    /// isn't walkable or resident.
    fn tile_geometry(&self, key: &TileKey) -> Option<crate::TileGeometry> {
        let slot = *self.by_key.get(key)?;
        let tile = self.slots[slot as usize].tile.as_ref()?;
        let n = i32::from(self.cfg.tile_cells);
        let cs = self.cfg.cell_size;
        let mut geo = crate::TileGeometry {
            origin: WorldPos::ORIGIN,
            verts: Vec::new(),
            polys: Vec::new(),
        };
        for (idx, cell) in tile.cells.iter().enumerate() {
            let Some(cell) = cell else { continue };
            let at = IVec2::new(key.x * n + idx as i32 % n, key.z * n + idx as i32 / n);
            let (x0, z0) = (at.x as f32 * cs, at.y as f32 * cs);
            let (x1, z1) = (x0 + cs, z0 + cs);
            let base = geo.verts.len() as u32;
            geo.verts.extend([
                Vec3::new(x0, cell.y, z0),
                Vec3::new(x0, cell.y, z1),
                Vec3::new(x1, cell.y, z1),
                Vec3::new(x1, cell.y, z0),
            ]);
            // Edge i runs from corner i to i + 1: -x, +z, +x, -z.
            let sides = [IVec2::NEG_X, IVec2::Y, IVec2::X, IVec2::NEG_Y];
            let mut walls = 0u8;
            for (i, d) in sides.iter().enumerate() {
                if self.locate(&key.ws, key.class, at + *d).is_none() {
                    walls |= 1 << i;
                }
            }
            geo.polys.push(crate::DebugPoly {
                verts: (base..base + 4).collect(),
                walls,
                state: cell.state,
            });
        }
        Some(geo)
    }

    fn nearest(
        &self,
        at: &Location,
        class: AgentClassId,
        extent: Vec3,
        filter: &QueryFilter,
    ) -> Option<NavPoint> {
        let p = local(at);
        let cs = self.cfg.cell_size;
        let mut best: Option<(f32, PolyRef, Vec3)> = None;
        for gz in self.cell_index(p.z - extent.z)..=self.cell_index(p.z + extent.z) {
            for gx in self.cell_index(p.x - extent.x)..=self.cell_index(p.x + extent.x) {
                let Some(v) = self.locate(&at.worldspace, class, IVec2::new(gx, gz)) else {
                    continue;
                };
                if !filter.passes(v.cell.state.area, v.cell.state.flags)
                    || (v.cell.y - p.y).abs() > extent.y
                {
                    continue;
                }
                let (x0, z0) = (gx as f32 * cs, gz as f32 * cs);
                let q = Vec3::new(p.x.clamp(x0, x0 + cs), v.cell.y, p.z.clamp(z0, z0 + cs));
                let d2 = q.distance_squared(p);
                if best.is_none_or(|b| d2 < b.0) {
                    best = Some((d2, v.poly, q));
                }
            }
        }
        best.map(|(_, poly, q)| NavPoint {
            loc: located(&at.worldspace, q),
            poly,
        })
    }

    fn find_path(&self, job: &mut PathJob, budget: &mut SliceBudget) -> JobState {
        if job.is_finished() {
            return JobState::Failed(NavError::JobFinished);
        }
        let search = match job
            .scratch_mut()
            .take()
            .map(Box::<dyn std::any::Any + Send>::downcast::<Search>)
        {
            Some(Ok(s)) => Ok(*s),
            Some(Err(_)) => Err(NavError::Unsupported("a job started on another backend")),
            None => self.begin(job.query()),
        };
        let (out, spent) = match search {
            Ok(mut s) => {
                let (r, spent) = self.run(&mut s, job.query(), budget);
                if matches!(r, Ok(None)) {
                    *job.scratch_mut() = Some(Box::new(s));
                }
                (r, spent)
            }
            Err(e) => (Err(e), 0),
        };
        job.add_iterations(spent);
        match out {
            Ok(None) => JobState::InProgress,
            Ok(Some(r)) => {
                job.finish();
                JobState::Done(r)
            }
            Err(e) => {
                job.finish();
                JobState::Failed(e)
            }
        }
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
        let mut cur = self
            .view(from.poly)
            .ok_or(NavError::UnknownPoly(from.poly))?;
        let class = self
            .tile_of(from.poly)
            .map(|(t, _)| t.key.class)
            .ok_or(NavError::UnknownPoly(from.poly))?;
        let (a, b) = (local(&from.loc), local(to));
        let d = Vec2::new(b.x - a.x, b.z - a.z);
        if d == Vec2::ZERO {
            return Ok(NavRay {
                hit: false,
                t: 1.0,
                end: from.loc.clone(),
                polys: vec![cur.poly],
            });
        }
        let end = IVec2::new(self.cell_index(b.x), self.cell_index(b.z));
        let cs = self.cfg.cell_size;
        let axis = |p: f32, d: f32, c: i32| -> (i32, f32, f32) {
            if d > 0.0 {
                (1, ((c + 1) as f32 * cs - p) / d, cs / d)
            } else if d < 0.0 {
                (-1, (c as f32 * cs - p) / d, -cs / d)
            } else {
                (0, f32::INFINITY, f32::INFINITY)
            }
        };
        let (sx, mut tx, dx) = axis(a.x, d.x, cur.at.x);
        let (sz, mut tz, dz) = axis(a.z, d.y, cur.at.y);
        let mut polys = vec![cur.poly];
        let step =
            |from: &CellView, at: IVec2| Self::step_ok(from, self.locate(ws, class, at), filter);
        while cur.at != end {
            let (t, next) = match tx.partial_cmp(&tz) {
                Some(Ordering::Less) => {
                    tx += dx;
                    (tx - dx, step(&cur, cur.at + IVec2::new(sx, 0)))
                }
                Some(Ordering::Greater) => {
                    tz += dz;
                    (tz - dz, step(&cur, cur.at + IVec2::new(0, sz)))
                }
                _ => {
                    // Through a corner: both side cells must pass too.
                    let sides = step(&cur, cur.at + IVec2::new(sx, 0)).is_some()
                        && step(&cur, cur.at + IVec2::new(0, sz)).is_some();
                    tx += dx;
                    tz += dz;
                    (
                        tx - dx,
                        step(&cur, cur.at + IVec2::new(sx, sz)).filter(|_| sides),
                    )
                }
            };
            if t.is_nan() || t > 1.0 {
                break;
            }
            let Some(next) = next else {
                let p = Vec3::new(a.x + (b.x - a.x) * t, cur.cell.y, a.z + (b.z - a.z) * t);
                return Ok(NavRay {
                    hit: true,
                    t,
                    end: located(ws, p),
                    polys,
                });
            };
            cur = next;
            polys.push(cur.poly);
        }
        Ok(NavRay {
            hit: false,
            t: 1.0,
            end: located(ws, Vec3::new(b.x, cur.cell.y, b.z)),
            polys,
        })
    }

    fn poly_state(&self, poly: PolyRef) -> Option<PolyState> {
        self.view(poly).map(|v| v.cell.state)
    }

    fn set_poly_state(&mut self, polys: &[PolyRef], state: PolyState) -> Result<(), NavError> {
        if let Some(&p) = polys.iter().find(|&&p| self.tile_of(p).is_none()) {
            return Err(NavError::UnknownPoly(p));
        }
        for &p in polys {
            let (_, slot, idx) = Self::decode(p);
            if let Some(Some(c)) = self.slots[slot].tile.as_mut().map(|t| &mut t.cells[idx]) {
                c.state = state;
            }
        }
        Ok(())
    }
}

// The sliced A*.

enum Target {
    /// `Point` and `AnyOf` goals: the snapped polygons, their centres and the goal points on them.
    Polys(Vec<(PolyRef, Vec3, Vec3)>),
    Near {
        at: Vec3,
        radius: f32,
    },
    Away {
        from: Vec3,
        min_dist: f32,
    },
}

impl Target {
    fn reached(&self, poly: PolyRef, pos: Vec3) -> bool {
        match self {
            Self::Polys(t) => t.iter().any(|(p, ..)| *p == poly),
            Self::Near { at, radius } => pos.distance(*at) <= *radius,
            Self::Away { from, min_dist } => pos.distance(*from) >= *min_dist,
        }
    }

    /// How far `pos` is from satisfying the goal: picks a partial path's end.
    fn gap(&self, pos: Vec3) -> f32 {
        match self {
            Self::Polys(t) => t
                .iter()
                .map(|(_, c, _)| pos.distance(*c))
                .fold(f32::INFINITY, f32::min),
            Self::Near { at, radius } => (pos.distance(*at) - radius).max(0.0),
            Self::Away { from, min_dist } => (min_dist - pos.distance(*from)).max(0.0),
        }
    }

    /// Admissible and consistent: a lower bound on meters still to walk (none for Away: Dijkstra).
    fn heuristic(&self, pos: Vec3) -> f32 {
        match self {
            Self::Away { .. } => 0.0,
            _ => self.gap(pos),
        }
    }

    fn end_point(&self, poly: PolyRef) -> Option<Vec3> {
        match self {
            Self::Polys(t) => t.iter().find(|(p, ..)| *p == poly).map(|(.., e)| *e),
            _ => None,
        }
    }
}

struct Node {
    g: f32,
    parent: Option<PolyRef>,
    closed: bool,
    pos: Vec3,
}

#[derive(Clone, Copy, PartialEq)]
struct Open {
    f: f32,
    g: f32,
    poly: PolyRef,
}

impl Eq for Open {}

impl Ord for Open {
    /// The heap's greatest is the best: lowest f, then highest g, then lowest ref.
    fn cmp(&self, o: &Self) -> Ordering {
        o.f.total_cmp(&self.f)
            .then(self.g.total_cmp(&o.g))
            .then(o.poly.cmp(&self.poly))
    }
}

impl PartialOrd for Open {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

struct Search {
    epoch: u64,
    start: NavPoint,
    target: Target,
    h_scale: f32,
    open: BinaryHeap<Open>,
    nodes: HashMap<PolyRef, Node>,
    /// The closed node nearest the goal: (gap, poly).
    best: Option<(f32, PolyRef)>,
}

impl GridBackend {
    fn begin(&self, q: &PathQuery) -> Result<Search, NavError> {
        q.goal.check_worldspace(&q.from.worldspace)?;
        let start = self
            .nearest(&q.from, q.class, q.extent, &q.filter)
            .ok_or(NavError::StartOffMesh)?;
        let snap = |l: &Location| {
            self.nearest(l, q.class, q.extent, &q.filter).and_then(|p| {
                let v = self.view(p.poly)?;
                Some((p.poly, self.center(v.at, v.cell.y), local(&p.loc)))
            })
        };
        let target = match &q.goal {
            Goal::Point(l) => Target::Polys(vec![snap(l).ok_or(NavError::GoalOffMesh)?]),
            Goal::AnyOf(ls) if ls.is_empty() => return Err(NavError::EmptyGoal),
            Goal::AnyOf(ls) => {
                let t: Vec<_> = ls.iter().filter_map(snap).collect();
                if t.is_empty() {
                    return Err(NavError::GoalOffMesh);
                }
                Target::Polys(t)
            }
            Goal::Near { at, radius } => Target::Near {
                at: local(at),
                radius: *radius,
            },
            Goal::Away { from, min_dist } => Target::Away {
                from: local(from),
                min_dist: *min_dist,
            },
        };
        let v = self.view(start.poly).ok_or(NavError::StartOffMesh)?;
        let pos = self.center(v.at, v.cell.y);
        let h_scale = q.filter.min_cost();
        let mut s = Search {
            epoch: self.epoch,
            start,
            target,
            h_scale,
            open: BinaryHeap::new(),
            nodes: HashMap::new(),
            best: None,
        };
        s.open.push(Open {
            f: s.target.heuristic(pos) * h_scale,
            g: 0.0,
            poly: v.poly,
        });
        s.nodes.insert(
            v.poly,
            Node {
                g: 0.0,
                parent: None,
                closed: false,
                pos,
            },
        );
        Ok(s)
    }

    /// Expands until done or out of budget: `Ok(None)` = in progress. Also returns expansions spent.
    fn run(
        &self,
        s: &mut Search,
        q: &PathQuery,
        budget: &mut SliceBudget,
    ) -> (Result<Option<PathResult>, NavError>, u32) {
        if s.epoch != self.epoch {
            return (Err(NavError::MeshChanged), 0);
        }
        let ws = &q.from.worldspace;
        let mut spent = 0;
        while let Some(&top) = s.open.peek() {
            let node = &s.nodes[&top.poly];
            if node.closed || top.g > node.g {
                s.open.pop(); // stale entry
                continue;
            }
            if !budget.spend() {
                return (Ok(None), spent);
            }
            spent += 1;
            s.open.pop();
            let (g, pos) = (node.g, node.pos);
            s.nodes.get_mut(&top.poly).expect("open nodes exist").closed = true;
            let gap = s.target.gap(pos);
            if s.best.is_none_or(|(b, _)| gap < b) {
                s.best = Some((gap, top.poly));
            }
            if s.target.reached(top.poly, pos) {
                return (
                    Ok(Some(Self::result(s, ws, top.poly, PathStatus::Complete))),
                    spent,
                );
            }
            let Some(cur) = self.view(top.poly) else {
                return (Err(NavError::MeshChanged), spent);
            };
            let cost_cur = q.filter.area_cost(cur.cell.state.area);
            let mut side_ok = [false; 4];
            for (k, dir) in DIRS.iter().enumerate() {
                if k >= 4
                    && !(side_ok[usize::from(dir.x < 0)] && side_ok[2 + usize::from(dir.y < 0)])
                {
                    continue; // no corner cutting
                }
                let Some(nb) =
                    Self::step_ok(&cur, self.locate(ws, q.class, cur.at + *dir), &q.filter)
                else {
                    continue;
                };
                if k < 4 {
                    side_ok[k] = true;
                }
                let npos = self.center(nb.at, nb.cell.y);
                let ng = g + pos.distance(npos)
                    * 0.5
                    * (cost_cur + q.filter.area_cost(nb.cell.state.area));
                let entry = s.nodes.entry(nb.poly).or_insert(Node {
                    g: f32::INFINITY,
                    parent: None,
                    closed: false,
                    pos: npos,
                });
                if entry.closed || ng >= entry.g {
                    continue;
                }
                entry.g = ng;
                entry.parent = Some(top.poly);
                s.open.push(Open {
                    f: ng + s.target.heuristic(npos) * s.h_scale,
                    g: ng,
                    poly: nb.poly,
                });
            }
        }
        let best = s.best.map_or(s.start.poly, |(_, p)| p);
        (
            Ok(Some(Self::result(s, ws, best, PathStatus::Partial))),
            spent,
        )
    }

    fn result(s: &Search, ws: &WorldspaceId, end: PolyRef, status: PathStatus) -> PathResult {
        let mut polys = vec![end];
        while let Some(p) = s.nodes[polys.last().expect("not empty")].parent {
            polys.push(p);
        }
        polys.reverse();
        let mut waypoints = vec![s.start.loc.clone()];
        waypoints.extend(polys[1..].iter().map(|p| located(ws, s.nodes[p].pos)));
        if status == PathStatus::Complete
            && let Some(e) = s.target.end_point(end)
        {
            if polys.len() > 1 {
                waypoints.pop();
            }
            waypoints.push(located(ws, e));
        }
        PathResult {
            status,
            cost: s.nodes[&end].g,
            polys,
            waypoints,
        }
    }
}
