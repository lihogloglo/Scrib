//! The navmesh bake (`docs/specs/nav.md` "Bake", NV4a): three `sf_bake` steps over the adapter's
//! [`NavGeometrySource`], backend-agnostic (the host passes a [`NavBackend`]; the game uses
//! `sf_nav_recast`). Bevy-free.
//!
//! - **`nav.tiles`**, subject (worldspace, class, region, tile): asks the source for the tile's
//!   geometry plus the Recast border ([`BuildConfig::border`]); the source's reads go through the
//!   step's [`RunCx`], so they are the product's dynamic dependencies, and a moved wall re-bakes
//!   only the tiles whose reads it changed. Doorways get their own polygons: each
//!   [`DoorVolume`] is marked as a convex area volume after erosion (Recast
//!   `rcMarkConvexPolyArea`, whose regions never cross areas), and the tile lists the polygons
//!   under each door ([`DoorPolys`]). A tile with no polygons is zero bytes: every empty tile
//!   shares the store's one empty object and the runtime skips it.
//! - **`nav.links`**, subject worldspace: the source's off-mesh links and door approach points,
//!   sorted ([`NavLinks`]).
//! - **`nav.coarse`**, subject (worldspace, class): the coarse graph ([`CoarseGraph`]) from the
//!   worldspace's tile products (planned inputs, so the scheduler bakes tiles first) and its links:
//!   one entrance node per run of tile-border portal edges per connected polygon set (HPA*,
//!   Botea et al. 2004), door approach nodes, and straight-line costs between nodes of one
//!   connected set and across matching entrances. **Stub until NV12**: costs are meters, not
//!   Detour path lengths times area costs.
//!
//! Parallelism is `sf_bake`'s: every tile is its own product on the rayon pool.

mod coarse;
#[cfg(test)]
mod tests;

use std::sync::Arc;

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_coords::WorldPos;
use sf_nav::{
    AgentClass, AgentClassId, AreaTable, AreaVolume, BakedTile, BlobTile, BuildConfig, DoorPolys,
    NavBackend, NavDoorId, NavError, NavLinks, TileInput, TileKey,
};
use sf_world::WorldspaceId;

/// Step ids.
pub const TILES: &str = "nav.tiles";
pub const LINKS: &str = "nav.links";
pub const COARSE: &str = "nav.coarse";

/// A nav region of a worldspace (nav.md "Nav regions"): its origin and the XZ extent, in
/// region-local meters, that tiles cover.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavRegion {
    pub region: u16,
    pub origin: WorldPos,
    pub min: Vec2,
    pub max: Vec2,
}

/// The XZ rectangle (region-local meters) a tile's input must cover: the tile plus its border.
/// Geometry is a column: the source returns everything over the rectangle at any height.
#[derive(Clone, Debug, PartialEq)]
pub struct NavTileBounds {
    pub region: u16,
    pub tile_x: i32,
    pub tile_z: i32,
    pub min: Vec2,
    pub max: Vec2,
}

impl NavTileBounds {
    pub fn overlaps(&self, min: Vec2, max: Vec2) -> bool {
        self.min.cmple(max).all() && min.cmple(self.max).all()
    }
}

/// A doorway's footprint (nav.md "Doors"): the source drops the door leaf from the geometry and
/// marks the doorway with this volume, whose polygons the bake lists under `door`.
#[derive(Clone, Debug, PartialEq)]
pub struct DoorVolume {
    pub door: NavDoorId,
    pub volume: AreaVolume,
}

/// One tile's geometry from the source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NavTileSource {
    /// Triangles, their areas and non-door area volumes; `tile_x`/`tile_z` are set by the step.
    pub input: TileInput,
    pub doors: Vec<DoorVolume>,
}

/// The adapter's geometry for the bake (nav.md "Interfaces"): the same cooked collision the mover
/// uses. Every read that shapes a result goes through `cx` (recorded as a dynamic dependency);
/// results must be deterministic (same reads, same order).
pub trait NavGeometrySource: Send + Sync {
    /// The worldspace's nav regions. Plan-time data: cheap and deterministic (an index the host
    /// already holds); a change re-plans the worldspace.
    fn regions(&self, ws: &WorldspaceId) -> Vec<NavRegion>;

    /// Geometry over `bounds` for one agent class (a class that can't swim may drop seabed).
    fn tile_input(
        &self,
        cx: &mut RunCx<'_>,
        ws: &WorldspaceId,
        class: &AgentClass,
        bounds: &NavTileBounds,
    ) -> Result<NavTileSource, BakeError>;

    /// Off-mesh links and door approach points of the worldspace.
    fn links(&self, cx: &mut RunCx<'_>, ws: &WorldspaceId) -> Result<NavLinks, BakeError>;
}

/// What the bake builds.
#[derive(Clone, Debug, PartialEq)]
pub struct NavBakeConfig {
    /// One navmesh per class per worldspace.
    pub classes: Vec<BuildConfig>,
    pub areas: AreaTable,
    /// Names the backend's tile format and settings (`sf_nav_recast::BAKE_TAG`): part of every
    /// key, so a backend change re-bakes.
    pub backend_tag: String,
}

struct Shared {
    source: Arc<dyn NavGeometrySource>,
    backend: Arc<dyn NavBackend>,
    cfg: NavBakeConfig,
}

impl Shared {
    fn class(&self, id: AgentClassId) -> Option<&BuildConfig> {
        self.cfg.classes.iter().find(|c| c.class.id == id)
    }

    fn tile_keys(&self, ws: &WorldspaceId, cfg: &BuildConfig) -> Vec<TileKey> {
        let size = cfg.tile_size();
        let mut regions = self.source.regions(ws);
        regions.sort_by_key(|r| r.region);
        let mut keys = Vec::new();
        for r in &regions {
            let (x0, x1) = tile_range(r.min.x, r.max.x, size);
            let (z0, z1) = tile_range(r.min.y, r.max.y, size);
            for x in x0..=x1 {
                for z in z0..=z1 {
                    keys.push(TileKey {
                        ws: ws.clone(),
                        class: cfg.class.id,
                        region: r.region,
                        x,
                        z,
                    });
                }
            }
        }
        keys
    }

    fn class_params(&self, cx: &mut PlanCx, cfg: &BuildConfig) -> Result<(), BakeError> {
        cx.param("class", encode(cfg)?)
            .param("areas", encode(&self.cfg.areas)?)
            .param("backend", self.cfg.backend_tag.as_bytes());
        Ok(())
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "tile indices of a region (≤ 16 km) are small"
)]
fn tile_range(min: f32, max: f32, size: f32) -> (i32, i32) {
    let lo = (min / size).floor() as i32;
    let hi = ((max / size).ceil() as i32 - 1).max(lo);
    (lo, hi)
}

fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>, BakeError> {
    postcard::to_stdvec(v).map_err(|e| BakeError::Other(format!("nav bake encode: {e}")))
}

fn nav_err(step: &str, subject: &str, e: &NavError) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), e)
}

/// `nav.tiles` subject: `class/region/x/z/worldspace` (the worldspace last: ids may hold `/`).
pub fn tile_subject(key: &TileKey) -> String {
    format!(
        "{}/{}/{}/{}/{}",
        key.class.0,
        key.region,
        key.x,
        key.z,
        key.ws.as_str()
    )
}

pub fn parse_tile_subject(s: &str) -> Option<TileKey> {
    let mut it = s.splitn(5, '/');
    let class = AgentClassId(it.next()?.parse().ok()?);
    let region = it.next()?.parse().ok()?;
    let x = it.next()?.parse().ok()?;
    let z = it.next()?.parse().ok()?;
    let ws = WorldspaceId::new(it.next()?);
    Some(TileKey {
        ws,
        class,
        region,
        x,
        z,
    })
}

/// `nav.coarse` subject: `class/worldspace`.
pub fn coarse_subject(ws: &WorldspaceId, class: AgentClassId) -> String {
    format!("{}/{}", class.0, ws.as_str())
}

fn parse_coarse_subject(s: &str) -> Option<(AgentClassId, WorldspaceId)> {
    let (class, ws) = s.split_once('/')?;
    Some((AgentClassId(class.parse().ok()?), WorldspaceId::new(ws)))
}

/// The navmesh bake: registers its steps on a [`Baker`] and names the products to bake.
#[derive(Clone)]
pub struct NavBake(Arc<Shared>);

impl NavBake {
    pub fn new(
        source: Arc<dyn NavGeometrySource>,
        backend: Arc<dyn NavBackend>,
        cfg: NavBakeConfig,
    ) -> Result<Self, NavError> {
        for (i, c) in cfg.classes.iter().enumerate() {
            c.validate()?;
            if cfg.classes[..i].iter().any(|o| o.class.id == c.class.id) {
                return Err(NavError::InvalidConfig(format!(
                    "agent class {:?} listed twice",
                    c.class.id
                )));
            }
        }
        Ok(Self(Arc::new(Shared {
            source,
            backend,
            cfg,
        })))
    }

    pub fn register(&self, baker: &mut Baker) {
        baker
            .register(TilesStep(self.0.clone()))
            .register(LinksStep(self.0.clone()))
            .register(CoarseStep(self.0.clone()));
    }

    /// Every tile of `ws` for `class` (over the source's region extents), in subject order.
    pub fn tile_keys(&self, ws: &WorldspaceId, class: AgentClassId) -> Vec<TileKey> {
        self.0
            .class(class)
            .map_or_else(Vec::new, |c| self.0.tile_keys(ws, c))
    }

    /// The products of one class in `worldspaces`: its tiles and coarse graphs.
    pub fn class_products(
        &self,
        worldspaces: &[WorldspaceId],
        class: AgentClassId,
    ) -> Vec<ProductKey> {
        let mut keys = Vec::new();
        for ws in worldspaces {
            keys.extend(
                self.tile_keys(ws, class)
                    .iter()
                    .map(|k| ProductKey::new(TILES, tile_subject(k))),
            );
            keys.push(ProductKey::new(COARSE, coarse_subject(ws, class)));
        }
        keys
    }

    /// Every product for `worldspaces`, all classes: the host's bake roots.
    pub fn products(&self, worldspaces: &[WorldspaceId]) -> Vec<ProductKey> {
        let mut keys: Vec<ProductKey> = worldspaces
            .iter()
            .map(|ws| ProductKey::new(LINKS, ws.as_str()))
            .collect();
        for c in &self.0.cfg.classes {
            keys.extend(self.class_products(worldspaces, c.class.id));
        }
        keys
    }
}

struct TilesStep(Arc<Shared>);

impl TilesStep {
    fn cfg(&self, subject: &str) -> Result<(TileKey, &BuildConfig), BakeError> {
        let bad = |m: &str| BakeError::step(&ProductKey::new(TILES, subject), m);
        let key = parse_tile_subject(subject).ok_or_else(|| bad("bad subject"))?;
        let cfg = self
            .0
            .class(key.class)
            .ok_or_else(|| bad("no build config for the class"))?;
        Ok((key, cfg))
    }
}

impl Step for TilesStep {
    fn id(&self) -> &'static str {
        TILES
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (_, cfg) = self.cfg(subject)?;
        self.0.class_params(cx, cfg)
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (key, cfg) = self.cfg(subject)?;
        let (size, border) = (cfg.tile_size(), cfg.border());
        #[allow(clippy::cast_precision_loss, reason = "tile indices are small")]
        let lo = Vec2::new(key.x as f32 * size, key.z as f32 * size);
        let bounds = NavTileBounds {
            region: key.region,
            tile_x: key.x,
            tile_z: key.z,
            min: lo - border,
            max: lo + size + border,
        };
        let NavTileSource {
            mut input,
            mut doors,
        } = self.0.source.tile_input(cx, &key.ws, &cfg.class, &bounds)?;
        input.tile_x = key.x;
        input.tile_z = key.z;
        doors.sort_by_key(|d| d.door);
        // Door volumes last: a later volume wins where they overlap the adapter's own.
        input.volumes.extend(doors.iter().map(|d| d.volume.clone()));
        let blob = self
            .0
            .backend
            .build_tile(&input, cfg, &self.0.cfg.areas)
            .map_err(|e| nav_err(TILES, subject, &e))?;
        if blob.is_empty() {
            return Ok(Vec::new());
        }
        let decoded = self
            .0
            .backend
            .read_tile_blob(&blob)
            .map_err(|e| nav_err(TILES, subject, &e))?;
        let doors = door_polys(&decoded, &doors, cfg.cell_height);
        BakedTile { blob, doors }
            .encode()
            .map_err(|e| BakeError::step(&ProductKey::new(TILES, subject), e))
    }
}

/// The polygons under each door volume, by area and centroid; a polygon goes to the first door
/// (by id) that holds it. Recast regions never cross areas, so a doorway's polygons are its own.
fn door_polys(tile: &BlobTile, doors: &[DoorVolume], cell_height: f32) -> Vec<DoorPolys> {
    let mut taken = vec![false; tile.polys.len()];
    let mut out = Vec::new();
    for d in doors {
        // Polygon vertices sit on span tops, which may round past the volume by a cell or two.
        let slack = 2.0 * cell_height;
        let v = AreaVolume {
            min_y: d.volume.min_y - slack,
            max_y: d.volume.max_y + slack,
            ..d.volume.clone()
        };
        let mut polys = Vec::new();
        for (i, p) in tile.polys.iter().enumerate() {
            if !taken[i] && p.state.area == d.volume.area && v.contains(p.centroid(tile)) {
                taken[i] = true;
                polys.push(u32::try_from(i).expect("Detour caps polygons per tile at 2^20"));
            }
        }
        if !polys.is_empty() {
            out.push(DoorPolys {
                door: d.door,
                polys,
            });
        }
    }
    out
}

struct LinksStep(Arc<Shared>);

impl Step for LinksStep {
    fn id(&self) -> &'static str {
        LINKS
    }

    fn version(&self) -> u32 {
        1
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let ws = WorldspaceId::new(subject);
        let mut links = self.0.source.links(cx, &ws)?;
        links.links.sort_by_key(|l| l.id);
        links.doors.sort_by_key(|d| d.door);
        encode(&links)
    }
}

struct CoarseStep(Arc<Shared>);

impl CoarseStep {
    fn cfg(&self, subject: &str) -> Result<(WorldspaceId, &BuildConfig), BakeError> {
        let bad = |m: &str| BakeError::step(&ProductKey::new(COARSE, subject), m);
        let (class, ws) = parse_coarse_subject(subject).ok_or_else(|| bad("bad subject"))?;
        let cfg = self
            .0
            .class(class)
            .ok_or_else(|| bad("no build config for the class"))?;
        Ok((ws, cfg))
    }
}

impl Step for CoarseStep {
    fn id(&self) -> &'static str {
        COARSE
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, cfg) = self.cfg(subject)?;
        self.0.class_params(cx, cfg)?;
        let mut regions = self.0.source.regions(&ws);
        regions.sort_by_key(|r| r.region);
        cx.param("regions", encode(&regions)?);
        cx.product(LINKS, ws.as_str());
        for k in self.0.tile_keys(&ws, cfg) {
            cx.product(TILES, tile_subject(&k));
        }
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, cfg) = self.cfg(subject)?;
        let fail = |m: String| BakeError::step(&ProductKey::new(COARSE, subject), m);
        let links: NavLinks = postcard::from_bytes(&cx.product(LINKS, ws.as_str())?)
            .map_err(|e| fail(e.to_string()))?;
        let mut tiles = Vec::new();
        for k in self.0.tile_keys(&ws, cfg) {
            let bytes = cx.product(TILES, &tile_subject(&k))?;
            if let Some(t) = BakedTile::decode(&bytes).map_err(|e| fail(e.to_string()))? {
                let blob = self
                    .0
                    .backend
                    .read_tile_blob(&t.blob)
                    .map_err(|e| nav_err(COARSE, subject, &e))?;
                tiles.push((k, blob));
            }
        }
        let mut regions = self.0.source.regions(&ws);
        regions.sort_by_key(|r| r.region);
        let graph = coarse::build(cfg, &regions, &tiles, &links);
        encode(&graph)
    }
}

/// Region-local position of a world position in `region`.
fn local(region: &NavRegion, at: WorldPos) -> Vec3 {
    region.origin.delta_f32(at)
}
