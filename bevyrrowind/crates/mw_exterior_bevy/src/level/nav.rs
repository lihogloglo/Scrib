//! A level's navmesh (slice `AI47b`): its tiles are built at load from the level's own static colliders
//! through the same `nav.tiles` bake as Vvardenfell's (`sf_nav_bake`, Recast), and made resident
//! in [`NavMeshes`], so wander and search plan paths in `level:` scenes. The standard for a small
//! authored space (UE/Unity build the navmesh from level collision in the editor or at load);
//! nothing bakes during play in the open world.
//!
//! [`LevelNavSource`] is the level's `NavGeometrySource`: the triangles of every static collider
//! (`collide`, not a prop), as `level::spawn` gives them to physics, in the level worldspace's one
//! nav region (`mw_bake::nav::REGION`, origin the worldspace's). The geometry's digest is the
//! tiles' recorded input (a virtual file, [`GEOMETRY`]), so the level's own store
//! (`<bake store>/level-nav`, apart from the shared bake's trace log) serves a warm load from cache
//! and an edited level re-bakes. Humanoid class only, as the baked ring keeps (`mw_walkers_bevy::nav`);
//! no off-mesh links or doors.

use std::sync::Arc;
use std::time::Instant;

use bevy::prelude::*;
use sf_bake::{BakeError, Baker, Digest, Inputs, ProductKey, RunCx, Store};
use sf_coords::WorldPos;
use sf_level::{Level, Shape};
use sf_nav::{AgentClass, BakedTile, NavLinks, TileInput, TileKey};
use sf_nav_bake::{NavBake, NavBakeConfig, NavGeometrySource, NavRegion, NavTileBounds, NavTileSource};
use sf_nav_bevy::{NavCorridorPlugin, NavMeshes, NavPlugin, NavQueryPlugin};
use sf_nav_recast::{RecastBackend, RecastConfig};
use sf_world::WorldspaceId;

use mw_bake::nav::{GROUND, HUMANOID, REGION, REGION_ORIGIN, areas, classes};

/// The virtual input file the tiles record: the level geometry's digest.
pub const GEOMETRY: &str = "level/nav-geometry";

/// A level's static collision, region-local meters (+Y up).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LevelGeometry {
    pub verts: Vec<Vec3>,
    pub tris: Vec<[u32; 3]>,
    /// XZ extent of the vertices (`x`, `z`).
    pub min: Vec2,
    pub max: Vec2,
}

impl LevelGeometry {
    /// Every static collider of `level`: objects with `collide`, minus props (dynamic boxes), as
    /// `level::spawn` inserts them into physics.
    pub fn of(level: &Level) -> Self {
        let mut g = Self { min: Vec2::splat(f32::MAX), max: Vec2::splat(f32::MIN), ..Self::default() };
        for o in &level.objects {
            if !o.collide || (o.dynamic.is_some() && matches!(o.shape, Shape::Box { .. })) {
                continue;
            }
            let data = o.mesh();
            let at = REGION_ORIGIN.delta_f32(WorldPos::from_f64(o.at.as_dvec3()));
            let rot = o.rotation();
            let base = u32::try_from(g.verts.len()).expect("level vertices fit u32");
            for p in &data.positions {
                let v = at + rot * Vec3::from(*p);
                g.min = g.min.min(v.xz());
                g.max = g.max.max(v.xz());
                g.verts.push(v);
            }
            g.tris.extend(data.triangles().iter().map(|t| t.map(|i| base + i)));
        }
        if g.verts.is_empty() {
            g.min = Vec2::ZERO;
            g.max = Vec2::ZERO;
        }
        g
    }

    /// The bytes' digest: vertices then triangles, little-endian.
    pub fn digest(&self) -> Digest {
        let mut bytes = Vec::with_capacity(self.verts.len() * 12 + self.tris.len() * 12);
        for v in &self.verts {
            for c in v.to_array() {
                bytes.extend(c.to_le_bytes());
            }
        }
        for t in &self.tris {
            for i in t {
                bytes.extend(i.to_le_bytes());
            }
        }
        Digest::of(&bytes)
    }

    /// The triangles over `b`, as `mw_bake::nav`'s `push_mesh` cuts a model (whole triangles).
    fn tile_input(&self, b: &NavTileBounds) -> TileInput {
        let mut input = TileInput::default();
        let mut remap = vec![u32::MAX; self.verts.len()];
        for t in &self.tris {
            let [a, bb, c] = t.map(|i| self.verts[i as usize]);
            let (lo, hi) = (a.xz().min(bb.xz()).min(c.xz()), a.xz().max(bb.xz()).max(c.xz()));
            if !b.overlaps(lo, hi) {
                continue;
            }
            let tri = t.map(|i| {
                let slot = &mut remap[i as usize];
                if *slot == u32::MAX {
                    *slot = u32::try_from(input.verts.len()).expect("tile vertices fit u32");
                    input.verts.push(self.verts[i as usize]);
                }
                *slot
            });
            input.tris.push(tri);
            input.tri_areas.push(GROUND);
        }
        input
    }
}

/// The level's `NavGeometrySource`: one region over the geometry, its triangles per tile.
struct LevelNavSource {
    ws: WorldspaceId,
    geometry: Arc<LevelGeometry>,
}

impl NavGeometrySource for LevelNavSource {
    fn regions(&self, ws: &WorldspaceId) -> Vec<NavRegion> {
        if *ws != self.ws || self.geometry.tris.is_empty() {
            return Vec::new();
        }
        vec![NavRegion { region: REGION, origin: REGION_ORIGIN, min: self.geometry.min, max: self.geometry.max }]
    }

    fn tile_input(&self, cx: &mut RunCx<'_>, ws: &WorldspaceId, _class: &AgentClass, bounds: &NavTileBounds) -> Result<NavTileSource, BakeError> {
        // The whole level is one input: any edit re-bakes every tile (a greybox has few).
        cx.file_fingerprint(GEOMETRY)?;
        if *ws != self.ws {
            return Ok(NavTileSource::default());
        }
        Ok(NavTileSource { input: self.geometry.tile_input(bounds), doors: Vec::new() })
    }

    fn links(&self, _cx: &mut RunCx<'_>, _ws: &WorldspaceId) -> Result<NavLinks, BakeError> {
        Ok(NavLinks::default())
    }
}

/// The bake's inputs: only [`GEOMETRY`], fingerprinted by the geometry's digest.
struct LevelInputs(Digest);

impl Inputs for LevelInputs {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok((path == GEOMETRY).then_some(self.0))
    }

    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        Err(BakeError::Other(format!("level nav: no file {path}")))
    }

    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

/// A level's built tiles.
#[derive(Debug, Default)]
pub struct LevelNav {
    /// Tiles with polygons (empty tiles are left out), in subject order, as `BakedTile` blobs.
    pub tiles: Vec<(TileKey, BakedTile)>,
    /// Tiles planned over the level's extent.
    pub planned: usize,
    /// Tiles built this run (the rest came from the store's cache).
    pub built: usize,
    /// Wall time, ms (reported only).
    pub ms: f64,
}

/// Builds `level`'s humanoid tiles in worldspace `ws` with the `nav.tiles` bake over `store`.
///
/// # Errors
/// The backend or bake config is invalid, or a tile failed to build or decode.
pub fn build(level: &Level, ws: &WorldspaceId, store: Store) -> Result<LevelNav, String> {
    let started = Instant::now();
    let geometry = Arc::new(LevelGeometry::of(level));
    let mut baker = Baker::new(store, Arc::new(LevelInputs(geometry.digest())));
    let source = Arc::new(LevelNavSource { ws: ws.clone(), geometry });
    let backend = RecastBackend::new(RecastConfig::new(classes())).map_err(|e| e.to_string())?;
    let cfg = NavBakeConfig { classes: classes(), areas: areas(), backend_tag: sf_nav_recast::BAKE_TAG.into() };
    let bake = NavBake::new(source, Arc::new(backend), cfg).map_err(|e| e.to_string())?;
    bake.register(&mut baker);
    let keys = bake.tile_keys(ws, HUMANOID);
    let products: Vec<ProductKey> = keys.iter().map(|k| ProductKey::new(sf_nav_bake::TILES, sf_nav_bake::tile_subject(k))).collect();
    let report = baker.bake(&products);
    if let Some((k, e)) = report.failures().next() {
        return Err(format!("{k}: {e}"));
    }
    let mut tiles = Vec::new();
    for (key, product) in keys.into_iter().zip(&products) {
        let digest = baker.product(product).map_err(|e| e.to_string())?;
        let bytes = baker.store().get(&digest).map_err(|e| format!("{product}: {e}"))?;
        if let Some(tile) = BakedTile::decode(&bytes).map_err(|e| format!("{product}: {e}"))? {
            tiles.push((key, tile));
        }
    }
    let built = baker.step_stats().get(sf_nav_bake::TILES).map_or(0, |s| usize::try_from(s.built).unwrap_or(usize::MAX));
    Ok(LevelNav { tiles, planned: products.len(), built, ms: started.elapsed().as_secs_f64() * 1e3 })
}

/// Builds the level's tiles at load and makes them resident. Without a navmesh in the app (a bake
/// with no tiles), adds the nav plugins and a Recast backend as `mw_walkers_bevy::nav` does.
pub fn add(app: &mut App, level: &Level, ws: &WorldspaceId, store: &Store) {
    let root = store.root().join("level-nav");
    let nav = match Store::open(&root).map_err(|e| format!("{}: {e}", root.display())).and_then(|s| build(level, ws, s)) {
        Ok(nav) => nav,
        Err(e) => {
            warn!("nav: level {}: no navmesh: {e}", level.name);
            return;
        }
    };
    if !app.world().contains_resource::<NavMeshes>() {
        let backend = match RecastBackend::new(RecastConfig::new(classes())) {
            Ok(b) => b,
            Err(e) => {
                warn!("nav: level {}: no navmesh backend: {e}", level.name);
                return;
            }
        };
        app.add_plugins((NavPlugin, NavQueryPlugin, NavCorridorPlugin)).insert_resource(NavMeshes(Box::new(backend)));
    }
    let failed = insert(&mut *app.world_mut().resource_mut::<NavMeshes>().0, &nav);
    info!(
        "nav: level tiles {} in {:.1} ms ({} planned, {} built, {} cached{})",
        nav.tiles.len(),
        nav.ms,
        nav.planned,
        nav.built,
        nav.planned.saturating_sub(nav.built),
        if failed > 0 { format!(", {failed} failed to load") } else { String::new() }
    );
}

/// Makes `nav`'s tiles resident; returns how many failed.
pub fn insert(backend: &mut dyn sf_nav::NavBackend, nav: &LevelNav) -> usize {
    let mut failed = 0;
    for (key, tile) in &nav.tiles {
        match backend.add_tile(key.clone(), &tile.blob) {
            Ok(_) | Err(sf_nav::NavError::TileLoaded(_)) => {}
            Err(e) => {
                warn!("nav: level tile {}: {e}", sf_nav_bake::tile_subject(key));
                failed += 1;
            }
        }
    }
    failed
}

#[cfg(test)]
mod tests {
    use sf_nav::{Goal, NavBackend, PathQuery, PathStatus, QueryFilter};
    use sf_world::Location;

    use super::*;

    fn perception_lab() -> Level {
        crate::level::find("perception-lab").expect("built-in level").1
    }

    #[test]
    fn perception_lab_gets_a_navmesh_and_a_path_around_the_search_boxes() {
        let level = perception_lab();
        let ws = crate::level::worldspace(&level.name);
        let dir = tempfile::tempdir().unwrap();
        let nav = build(&level, &ws, Store::open(dir.path()).unwrap()).unwrap();
        println!("nav: level tiles {} of {} in {:.1} ms ({} built)", nav.tiles.len(), nav.planned, nav.ms, nav.built);
        // 180 x 100 m floor on 19.2 m humanoid tiles: 10 x 6, every one with floor polygons.
        assert_eq!(nav.planned, 60);
        assert_eq!(nav.tiles.len(), 60);
        assert_eq!(nav.built, 60);

        let mut backend = RecastBackend::new(RecastConfig::new(classes())).unwrap();
        assert_eq!(insert(&mut backend, &nav), 0);
        // The search guard's post to the far side of the search boxes (through and around them),
        // across tile borders.
        let at = |x: f64, z: f64| Location::new(ws.clone(), WorldPos::from_f64(bevy::math::DVec3::new(x, 0.0, z)));
        let (from, to) = (at(60.0, 0.0), at(60.0, -16.0));
        let filter = QueryFilter::for_class(&areas(), HUMANOID);
        let r = backend.find_path_now(PathQuery::new(HUMANOID, from.clone(), Goal::Point(to.clone()), filter)).unwrap();
        assert_eq!(r.status, PathStatus::Complete);
        let (first, last) = (r.waypoints.first().unwrap(), r.waypoints.last().unwrap());
        let planar = |a: &Location, b: &Location| a.pos.delta_f32(b.pos).xz().length();
        assert!(planar(first, &from) < 0.05, "start on the mesh: {first:?}");
        assert!(planar(last, &to) < 0.05, "goal on the mesh: {last:?}");
        assert!(r.cost > 16.0, "detours around search-box-3: cost {}", r.cost);

        // A warm load reads every tile from the store's cache.
        let warm = build(&level, &ws, Store::open(dir.path()).unwrap()).unwrap();
        println!("nav: level tiles {} warm in {:.1} ms", warm.tiles.len(), warm.ms);
        assert_eq!((warm.tiles.len(), warm.built), (60, 0));
    }
}
