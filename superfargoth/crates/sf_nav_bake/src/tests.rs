//! NV4 tests on synthetic geometry (nav.md "Build plan" NV4): cache hit on rerun, a moved box
//! re-bakes only the tiles whose reads it changed, each doorway has its own polygons, empty tiles
//! are empty, plus a printed bake measurement.
//!
//! The source keeps its world in in-memory "files", one per 10 m bucket (boxes, doors, whether
//! there is ground), so a tile's reads are the buckets under its bounds, as an adapter's are the
//! cells under them.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, Digest, Inputs, ProductKey, RunCx, StepStats, Store};
use sf_coords::WorldPos;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, AreaVolume, BakedTile, BuildConfig,
    CoarseGraph, CoarseNodeKind, DoorApproach, MAX_CLASSES, NavBackend, NavDoorId, NavLinks,
    PolyFlags, TileInput,
};
use sf_nav_recast::{BAKE_TAG, RecastBackend, RecastConfig};
use sf_world::{Location, WorldspaceId};

use crate::{
    COARSE, DoorVolume, NavBake, NavBakeConfig, NavGeometrySource, NavRegion, NavTileBounds,
    NavTileSource, TILES, coarse_subject, parse_tile_subject, tile_subject,
};

const GROUND: AreaId = AreaId(1);
const DOOR: AreaId = AreaId(2);
const BUCKET: f32 = 10.0;
const HUMANOID: AgentClassId = AgentClassId(0);
const LARGE: AgentClassId = AgentClassId(1);

// ---- host inputs: an in-memory file system ----

#[derive(Default)]
struct MemInputs(RwLock<BTreeMap<String, Vec<u8>>>);

impl MemInputs {
    fn put(&self, path: String, bytes: Vec<u8>) {
        self.0.write().unwrap().insert(path, bytes);
    }
}

impl Inputs for MemInputs {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.read().unwrap().get(path).map(|b| Digest::of(b)))
    }

    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0
            .read()
            .unwrap()
            .get(path)
            .cloned()
            .ok_or_else(|| BakeError::Other(format!("no file {path}")))
    }

    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

// ---- the synthetic world ----

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Boxy {
    id: u32,
    min: [f32; 3],
    max: [f32; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Door {
    id: u64,
    min: [f32; 2],
    max: [f32; 2],
    approach: [f32; 3],
}

#[derive(Default, Serialize, Deserialize)]
struct Bucket {
    ground: bool,
    boxes: Vec<Boxy>,
    doors: Vec<Door>,
}

struct Town {
    ws: WorldspaceId,
    /// Region extent (tiles cover it).
    extent: Vec2,
    /// Ground covers `[0, ground]` on XZ, at y = 0.
    ground: Vec2,
    boxes: Vec<Boxy>,
    doors: Vec<Door>,
}

#[allow(clippy::cast_possible_truncation, reason = "small test coordinates")]
fn bucket_range(min: Vec2, max: Vec2) -> impl Iterator<Item = (i32, i32)> {
    let (x0, x1) = (
        (min.x / BUCKET).floor() as i32,
        (max.x / BUCKET).floor() as i32,
    );
    let (z0, z1) = (
        (min.y / BUCKET).floor() as i32,
        (max.y / BUCKET).floor() as i32,
    );
    (x0..=x1).flat_map(move |x| (z0..=z1).map(move |z| (x, z)))
}

fn bucket_path(ws: &WorldspaceId, (x, z): (i32, i32)) -> String {
    format!("{}/bucket/{x}/{z}", ws.as_str())
}

fn xz(v: [f32; 3]) -> Vec2 {
    Vec2::new(v[0], v[2])
}

impl Town {
    #[allow(clippy::cast_precision_loss, reason = "small test coordinates")]
    fn buckets(&self) -> BTreeMap<(i32, i32), Bucket> {
        let mut out: BTreeMap<(i32, i32), Bucket> = BTreeMap::new();
        for b in bucket_range(Vec2::ZERO, self.extent) {
            let lo = Vec2::new(b.0 as f32, b.1 as f32) * BUCKET;
            out.entry(b).or_default().ground = lo.cmplt(self.ground).all();
        }
        for bx in &self.boxes {
            for b in bucket_range(xz(bx.min), xz(bx.max)) {
                out.entry(b).or_default().boxes.push(bx.clone());
            }
        }
        for d in &self.doors {
            for b in bucket_range(Vec2::from(d.min), Vec2::from(d.max)) {
                out.entry(b).or_default().doors.push(d.clone());
            }
        }
        out
    }

    fn write(&self, inputs: &MemInputs) {
        for (b, bucket) in self.buckets() {
            inputs.put(
                bucket_path(&self.ws, b),
                postcard::to_stdvec(&bucket).unwrap(),
            );
        }
        let links = NavLinks {
            links: Vec::new(),
            doors: self
                .doors
                .iter()
                .map(|d| DoorApproach {
                    door: NavDoorId(d.id),
                    at: Location::new(
                        self.ws.clone(),
                        WorldPos::ORIGIN.offset(Vec3::from(d.approach)),
                    ),
                })
                .collect(),
        };
        inputs.put(
            format!("{}/links", self.ws.as_str()),
            postcard::to_stdvec(&links).unwrap(),
        );
    }
}

/// Reads the buckets under a tile's bounds.
struct TownSource {
    ws: WorldspaceId,
    extent: Vec2,
}

fn push_quad(input: &mut TileInput, min: Vec2, max: Vec2, y: f32) {
    let base = u32::try_from(input.verts.len()).unwrap();
    input.verts.extend([
        Vec3::new(min.x, y, min.y),
        Vec3::new(max.x, y, min.y),
        Vec3::new(max.x, y, max.y),
        Vec3::new(min.x, y, max.y),
    ]);
    // Counter-clockwise seen from above (+Y up): Recast's walkable winding.
    input
        .tris
        .extend([[base, base + 2, base + 1], [base, base + 3, base + 2]]);
    input.tri_areas.extend([GROUND, GROUND]);
}

fn push_box(input: &mut TileInput, b: &Boxy) {
    let (lo, hi) = (Vec3::from(b.min), Vec3::from(b.max));
    let base = u32::try_from(input.verts.len()).unwrap();
    for i in 0..8u32 {
        input.verts.push(Vec3::new(
            if i & 1 == 0 { lo.x } else { hi.x },
            if i & 2 == 0 { lo.y } else { hi.y },
            if i & 4 == 0 { lo.z } else { hi.z },
        ));
    }
    // Outward-facing faces (index bits: x=1, y=2, z=4).
    let faces = [
        [2, 6, 7, 3], // top (+y)
        [0, 1, 5, 4], // bottom
        [1, 3, 7, 5], // +x
        [0, 4, 6, 2], // -x
        [4, 5, 7, 6], // +z
        [0, 2, 3, 1], // -z
    ];
    for f in faces {
        let [a, b, c, d] = f.map(|i| base + i);
        input.tris.extend([[a, b, c], [a, c, d]]);
        input.tri_areas.extend([GROUND, GROUND]);
    }
}

impl NavGeometrySource for TownSource {
    fn regions(&self, ws: &WorldspaceId) -> Vec<NavRegion> {
        if *ws != self.ws {
            return Vec::new();
        }
        vec![NavRegion {
            region: 0,
            origin: WorldPos::ORIGIN,
            min: Vec2::ZERO,
            max: self.extent,
        }]
    }

    #[allow(clippy::cast_precision_loss, reason = "small test coordinates")]
    fn tile_input(
        &self,
        cx: &mut RunCx<'_>,
        ws: &WorldspaceId,
        _class: &AgentClass,
        bounds: &NavTileBounds,
    ) -> Result<NavTileSource, BakeError> {
        let mut src = NavTileSource::default();
        let mut boxes = BTreeMap::new();
        let mut doors = BTreeMap::new();
        for b in bucket_range(bounds.min, bounds.max) {
            let path = bucket_path(ws, b);
            if !cx.file_exists(&path)? {
                continue;
            }
            let bucket: Bucket = postcard::from_bytes(&cx.read_file(&path)?)
                .map_err(|e| BakeError::Other(e.to_string()))?;
            if bucket.ground {
                let lo = Vec2::new(b.0 as f32, b.1 as f32) * BUCKET;
                push_quad(&mut src.input, lo, lo + BUCKET, 0.0);
            }
            for bx in bucket.boxes {
                if bounds.overlaps(xz(bx.min), xz(bx.max)) {
                    boxes.insert(bx.id, bx);
                }
            }
            for d in bucket.doors {
                if bounds.overlaps(Vec2::from(d.min), Vec2::from(d.max)) {
                    doors.insert(d.id, d);
                }
            }
        }
        for bx in boxes.values() {
            push_box(&mut src.input, bx);
        }
        src.doors = doors
            .values()
            .map(|d| DoorVolume {
                door: NavDoorId(d.id),
                volume: AreaVolume {
                    footprint: vec![
                        Vec2::new(d.min[0], d.min[1]),
                        Vec2::new(d.max[0], d.min[1]),
                        Vec2::new(d.max[0], d.max[1]),
                        Vec2::new(d.min[0], d.max[1]),
                    ],
                    min_y: -0.5,
                    max_y: 2.0,
                    area: DOOR,
                },
            })
            .collect();
        Ok(src)
    }

    fn links(&self, cx: &mut RunCx<'_>, ws: &WorldspaceId) -> Result<NavLinks, BakeError> {
        postcard::from_bytes(&cx.read_file(&format!("{}/links", ws.as_str()))?)
            .map_err(|e| BakeError::Other(e.to_string()))
    }
}

// ---- configuration ----

fn areas() -> AreaTable {
    AreaTable::new(vec![
        AreaDef {
            id: GROUND,
            name: "ground".into(),
            cost: [1.0; MAX_CLASSES],
            flags: PolyFlags::WALK,
        },
        AreaDef {
            id: DOOR,
            name: "door".into(),
            cost: [1.0; MAX_CLASSES],
            flags: PolyFlags::WALK | PolyFlags::DOOR,
        },
    ])
    .unwrap()
}

/// Test classes: coarse voxels so debug tests stay quick.
fn classes(cs_humanoid: f32, cells: u16) -> Vec<BuildConfig> {
    vec![
        BuildConfig {
            class: AgentClass {
                id: HUMANOID,
                radius: 0.3,
                height: 1.8,
                max_climb: 0.4,
                max_slope_deg: 45.0,
                can_swim: false,
            },
            cell_size: cs_humanoid,
            cell_height: 0.1,
            tile_cells: cells,
        },
        BuildConfig {
            class: AgentClass {
                id: LARGE,
                radius: 1.0,
                height: 3.0,
                max_climb: 0.6,
                max_slope_deg: 45.0,
                can_swim: false,
            },
            cell_size: 0.5,
            cell_height: 0.2,
            tile_cells: 64,
        },
    ]
}

struct Rig {
    town: Town,
    inputs: Arc<MemInputs>,
    bake: NavBake,
    store_dir: tempfile::TempDir,
}

impl Rig {
    fn new(town: Town, classes: Vec<BuildConfig>) -> Self {
        let inputs = Arc::new(MemInputs::default());
        town.write(&inputs);
        let backend = RecastBackend::new(RecastConfig::new(classes.clone())).unwrap();
        let source = TownSource {
            ws: town.ws.clone(),
            extent: town.extent,
        };
        let cfg = NavBakeConfig {
            classes,
            areas: areas(),
            backend_tag: BAKE_TAG.into(),
        };
        let bake = NavBake::new(Arc::new(source), Arc::new(backend), cfg).unwrap();
        Self {
            town,
            inputs,
            bake,
            store_dir: tempfile::tempdir().unwrap(),
        }
    }

    /// A fresh bake session over the same store (a new run of `<game> bake`).
    fn baker(&self) -> Baker {
        let mut baker = Baker::new(
            Store::open(self.store_dir.path()).unwrap(),
            self.inputs.clone(),
        );
        self.bake.register(&mut baker);
        baker
    }

    fn keys(&self) -> Vec<ProductKey> {
        self.bake.products(std::slice::from_ref(&self.town.ws))
    }
}

fn stats(baker: &Baker, step: &str) -> StepStats {
    baker.step_stats().get(step).copied().unwrap_or_default()
}

fn bake_ok(baker: &Baker, keys: &[ProductKey]) -> BTreeMap<ProductKey, Digest> {
    let report = baker.bake(keys);
    let failures: Vec<_> = report
        .failures()
        .map(|(k, e)| format!("{k}: {e}"))
        .collect();
    assert!(failures.is_empty(), "{failures:#?}");
    report
        .products
        .into_iter()
        .map(|(k, r)| (k, r.unwrap()))
        .collect()
}

fn product(baker: &Baker, key: &ProductKey) -> Vec<u8> {
    let d = baker.product(key).unwrap();
    baker.store().get(&d).unwrap()
}

/// A wall along x at `z` with doorway gaps `[x0, x1]`, doors spanning the wall's depth ±1 m.
fn wall_with_doors(
    z: f32,
    from: f32,
    to: f32,
    gaps: &[(f32, f32)],
    first_id: u32,
) -> (Vec<Boxy>, Vec<Door>) {
    let (mut boxes, mut doors) = (Vec::new(), Vec::new());
    let mut x = from;
    let mut id = first_id;
    for (i, &(g0, g1)) in gaps.iter().chain(std::iter::once(&(to, to))).enumerate() {
        if g0 > x {
            boxes.push(Boxy {
                id,
                min: [x, 0.0, z],
                max: [g0, 3.0, z + 0.3],
            });
            id += 1;
        }
        if g1 > g0 {
            doors.push(Door {
                id: 1000 + u64::from(first_id) + u64::try_from(i).unwrap(),
                min: [g0, z - 1.0],
                max: [g1, z + 1.3],
                approach: [f32::midpoint(g0, g1), 0.0, z - 0.6],
            });
        }
        x = g1;
    }
    (boxes, doors)
}

/// Two tiles by two of ground (humanoid 19.2 m tiles), an empty strip of tiles beyond it, a few
/// crates and a wall with two doorways.
fn small_town() -> Town {
    let (mut boxes, doors) = wall_with_doors(20.0, 2.0, 30.0, &[(6.0, 7.6), (14.0, 15.6)], 10);
    boxes.push(Boxy {
        id: 1,
        min: [4.0, 0.0, 4.0],
        max: [6.0, 1.5, 6.0],
    });
    boxes.push(Boxy {
        id: 2,
        min: [25.0, 0.0, 30.0],
        max: [27.0, 1.5, 32.0],
    });
    Town {
        ws: WorldspaceId::new("test/town"),
        extent: Vec2::new(57.6, 38.4),
        ground: Vec2::new(30.0, 38.4),
        boxes,
        doors,
    }
}

// ---- tests ----

#[test]
fn subjects_round_trip() {
    let key = sf_nav::TileKey {
        ws: WorldspaceId::new("a/b"),
        class: LARGE,
        region: 3,
        x: -4,
        z: 7,
    };
    assert_eq!(tile_subject(&key), "1/3/-4/7/a/b");
    assert_eq!(parse_tile_subject(&tile_subject(&key)), Some(key));
    assert_eq!(coarse_subject(&WorldspaceId::new("a/b"), HUMANOID), "0/a/b");
}

#[test]
fn rerun_is_a_full_cache_hit() {
    let rig = Rig::new(small_town(), classes(0.3, 64));
    let keys = rig.keys();
    let first = rig.baker();
    let a = bake_ok(&first, &keys);
    let tiles = stats(&first, TILES);
    assert_eq!(tiles.hits, 0);
    // 3 x 2 humanoid tiles + 2 x 2 large tiles over the 57.6 x 38.4 m extent.
    assert_eq!(tiles.built, 6 + 4);

    let second = rig.baker();
    let b = bake_ok(&second, &keys);
    assert_eq!(a, b);
    for (step, s) in second.step_stats() {
        assert_eq!(
            (s.built, s.failed),
            (0, 0),
            "{step} rebuilt on an unchanged rerun"
        );
    }
    assert_eq!(stats(&second, TILES).hits, 10);
    assert_eq!(stats(&second, COARSE).hits, 2);
}

#[test]
fn empty_tiles_store_nothing() {
    let rig = Rig::new(small_town(), classes(0.3, 64));
    let baker = rig.baker();
    let digests = bake_ok(&baker, &rig.keys());
    let empty = Digest::of(&[]);
    let mut kinds = BTreeMap::new();
    for k in rig.bake.tile_keys(&rig.town.ws, HUMANOID) {
        let d = digests[&ProductKey::new(TILES, tile_subject(&k))];
        *kinds.entry((k.x, d == empty)).or_insert(0) += 1;
        let bytes = baker.store().get(&d).unwrap();
        assert_eq!(BakedTile::decode(&bytes).unwrap().is_none(), d == empty);
    }
    // Ground ends at x = 30 m: the x = 2 column (38.4..57.6 m) is open void.
    assert_eq!(
        kinds,
        BTreeMap::from([((0, false), 2), ((1, false), 2), ((2, true), 2)])
    );
}

#[test]
fn each_doorway_has_its_own_polygons() {
    let rig = Rig::new(small_town(), classes(0.3, 64));
    let baker = rig.baker();
    bake_ok(&baker, &rig.keys());
    let backend = RecastBackend::new(RecastConfig::new(classes(0.3, 64))).unwrap();
    let mut per_door: BTreeMap<NavDoorId, BTreeSet<(i32, i32, u32)>> = BTreeMap::new();
    for k in rig.bake.tile_keys(&rig.town.ws, HUMANOID) {
        let bytes = product(&baker, &ProductKey::new(TILES, tile_subject(&k)));
        let Some(tile) = BakedTile::decode(&bytes).unwrap() else {
            continue;
        };
        let blob = backend.read_tile_blob(&tile.blob).unwrap();
        let door_area: BTreeSet<u32> = (0..blob.polys.len())
            .filter(|&i| blob.polys[i].state.area == DOOR)
            .map(|i| u32::try_from(i).unwrap())
            .collect();
        let mut listed = BTreeSet::new();
        for d in &tile.doors {
            for &p in &d.polys {
                assert!(listed.insert(p), "polygon {p} listed under two doors");
                assert!(blob.polys[p as usize].state.flags.contains(PolyFlags::DOOR));
                per_door.entry(d.door).or_default().insert((k.x, k.z, p));
            }
        }
        // Every door-area polygon belongs to some doorway.
        assert_eq!(listed, door_area, "tile ({}, {})", k.x, k.z);
    }
    let ids: Vec<NavDoorId> = rig.town.doors.iter().map(|d| NavDoorId(d.id)).collect();
    assert_eq!(per_door.keys().copied().collect::<Vec<_>>(), ids);
    assert!(per_door.values().all(|p| !p.is_empty()));

    // The coarse graph places each door's approach point.
    let coarse: CoarseGraph = postcard::from_bytes(&product(
        &baker,
        &ProductKey::new(COARSE, coarse_subject(&rig.town.ws, HUMANOID)),
    ))
    .unwrap();
    assert!(coarse.unplaced.is_empty(), "{:?}", coarse.unplaced);
    let placed: Vec<NavDoorId> = coarse
        .nodes
        .iter()
        .filter_map(|n| match n.kind {
            CoarseNodeKind::Door(d) => Some(d),
            CoarseNodeKind::Entrance(_) => None,
        })
        .collect();
    assert_eq!(placed, ids);

    // Sanity on the stub graph: the four ground tiles join across their shared sides, and every
    // door node reaches an entrance of its tile.
    let cross = coarse
        .edges
        .iter()
        .filter(|e| coarse.nodes[e.a as usize].tile != coarse.nodes[e.b as usize].tile)
        .count();
    assert!(cross >= 4, "{cross} cross-tile edges");
    assert!(
        coarse
            .edges
            .iter()
            .all(|e| e.a < e.b && e.cost.is_finite() && e.cost >= 0.0)
    );
    for (i, n) in coarse.nodes.iter().enumerate() {
        if matches!(n.kind, CoarseNodeKind::Door(_)) {
            let i = u32::try_from(i).unwrap();
            assert!(
                coarse.edges.iter().any(|e| e.a == i || e.b == i),
                "door node {i} has no edge"
            );
        }
    }
}

#[test]
fn moving_one_box_rebakes_only_the_tiles_it_touched() {
    let mut rig = Rig::new(small_town(), classes(0.3, 64));
    let keys = rig.keys();
    let before = bake_ok(&rig.baker(), &keys);

    // Move crate 1 by 1.5 m; rewrite only the buckets it left or entered.
    let old = rig.town.buckets();
    let crate1 = rig.town.boxes.iter_mut().find(|b| b.id == 1).unwrap();
    crate1.min[0] += 1.5;
    crate1.max[0] += 1.5;
    let new = rig.town.buckets();
    let mut changed = BTreeSet::new();
    for (b, bucket) in &new {
        let same = old.get(b).map(|o| postcard::to_stdvec(o).unwrap())
            == Some(postcard::to_stdvec(bucket).unwrap());
        if !same {
            rig.inputs.put(
                bucket_path(&rig.town.ws, *b),
                postcard::to_stdvec(bucket).unwrap(),
            );
            changed.insert(*b);
        }
    }
    assert!(!changed.is_empty());

    // The tiles that read a changed bucket: the source reads the buckets under tile + border.
    let mut expected = 0u32;
    for cfg in classes(0.3, 64) {
        let (size, border) = (cfg.tile_size(), cfg.border());
        for k in rig.bake.tile_keys(&rig.town.ws, cfg.class.id) {
            #[allow(clippy::cast_precision_loss, reason = "small tile indices")]
            let lo = Vec2::new(k.x as f32, k.z as f32) * size;
            if bucket_range(lo - border, lo + size + border).any(|b| changed.contains(&b)) {
                expected += 1;
            }
        }
    }
    let total = u32::try_from(rig.bake.tile_keys(&rig.town.ws, HUMANOID).len() + 4).unwrap();
    assert!(expected < total, "the move should leave some tiles alone");

    let baker = rig.baker();
    let after = bake_ok(&baker, &keys);
    let tiles = stats(&baker, TILES);
    assert_eq!((tiles.built, tiles.hits), (expected, total - expected));
    let changed_tiles = keys
        .iter()
        .filter(|k| k.step == TILES && before[*k] != after[*k])
        .count();
    assert!(changed_tiles >= 1 && changed_tiles <= expected as usize);
    // Nothing else read the buckets.
    assert_eq!(stats(&baker, crate::LINKS).built, 0);
}

/// A town of `n` x `n` houses, each a 6 m box with a doorway wall in front.
#[allow(clippy::cast_precision_loss, reason = "small test coordinates")]
#[allow(
    clippy::many_single_char_names,
    reason = "grid indices and coordinates"
)]
fn big_town(n: u32) -> Town {
    let (mut boxes, mut doors) = (Vec::new(), Vec::new());
    for i in 0..n {
        for j in 0..n {
            let (x, z) = (5.0 + 14.0 * i as f32, 5.0 + 14.0 * j as f32);
            let id = (i * n + j) * 10 + 100;
            boxes.push(Boxy {
                id,
                min: [x, 0.0, z],
                max: [x + 6.0, 4.0, z + 6.0],
            });
            let (b, d) = wall_with_doors(z + 8.0, x - 1.0, x + 7.0, &[(x + 2.0, x + 3.6)], id + 1);
            boxes.extend(b);
            doors.extend(d);
        }
    }
    let side = 14.0 * n as f32 + 10.0;
    Town {
        ws: WorldspaceId::new("test/big"),
        extent: Vec2::splat(side),
        ground: Vec2::splat(side),
        boxes,
        doors,
    }
}

#[test]
#[ignore = "measurement: cold bake ms and MiB per agent class on a synthetic town"]
fn measure_bake_per_class() {
    // NV0's calibration: humanoid cs 0.15 m, 128 cells (19.2 m); large cs 0.5 m, 64 cells.
    let rig = Rig::new(big_town(8), classes(0.15, 128));
    let baker = rig.baker();
    let ws = std::slice::from_ref(&rig.town.ws);
    for class in [HUMANOID, LARGE] {
        let keys = rig.bake.class_products(ws, class);
        let before = stats(&baker, TILES);
        let start = Instant::now();
        let digests = bake_ok(&baker, &keys);
        let wall = start.elapsed();
        let after = stats(&baker, TILES);
        let empty = Digest::of(&[]);
        let non_empty = digests
            .iter()
            .filter(|(k, d)| k.step == TILES && **d != empty)
            .count();
        let bytes = after.bytes_written - before.bytes_written;
        let cpu = after.run_time.saturating_sub(before.run_time).as_secs_f64() * 1e3;
        #[allow(clippy::cast_precision_loss, reason = "printing")]
        let mib = bytes as f64 / f64::from(1 << 20);
        println!(
            "class {}: {} tiles ({non_empty} non-empty), wall {:.0} ms, tile CPU {cpu:.0} ms ({:.1} ms/tile), {mib:.2} MiB",
            class.0,
            after.built - before.built,
            wall.as_secs_f64() * 1e3,
            cpu / f64::from((after.built - before.built).max(1)),
        );
    }
    let coarse: CoarseGraph = postcard::from_bytes(&product(
        &baker,
        &ProductKey::new(COARSE, coarse_subject(&rig.town.ws, HUMANOID)),
    ))
    .unwrap();
    println!(
        "humanoid coarse graph: {} nodes, {} edges, {} unplaced doors",
        coarse.nodes.len(),
        coarse.edges.len(),
        coarse.unplaced.len()
    );
}
