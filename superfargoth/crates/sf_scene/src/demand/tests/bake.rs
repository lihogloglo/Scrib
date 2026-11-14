//! `mesh.uv_density` + `texture.demand` end to end on a small generated world.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use glam::{DVec3, Quat, Vec3};
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::steps::{Importers, MeshCookStep};
use sf_model::uv_density::MeshUvDensityStep;
use sf_model::{CookSettings, ImportError, Material, MeshData, Model, ModelImporter, Node};
use sf_vfs::VfsPath;
use sf_world::bake::WorldBake;
use sf_world::{
    Fingerprint, IndexParams, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, RegionId, WorldSource, WorldspaceId,
};

use crate::demand::bake::{BaseTextures, DemandBake, build_cell_demand, model_demand};
use crate::demand::{CellDemand, DEMAND_STEP, MAX_CLUSTER_RADIUS, texture_id};

/// Test importer: the file is `<size> <uv tiling> <texture>`, a `size` m square tiled `uv` times.
struct QuadImporter;

impl ModelImporter for QuadImporter {
    fn extensions(&self) -> &[&str] {
        &["quad"]
    }
    fn version(&self) -> u32 {
        1
    }
    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        let err = || ImportError { path: path.into(), message: "bad quad".into() };
        let text = std::str::from_utf8(bytes).map_err(|_| err())?;
        let mut it = text.split_whitespace();
        let size: f32 = it.next().and_then(|v| v.parse().ok()).ok_or_else(err)?;
        let uv: f32 = it.next().and_then(|v| v.parse().ok()).ok_or_else(err)?;
        let texture = it.next().ok_or_else(err)?;
        let mesh = MeshData {
            positions: vec![[0.0, 0.0, 0.0], [size, 0.0, 0.0], [size, 0.0, size], [0.0, 0.0, size]],
            normals: Some(vec![[0.0, 1.0, 0.0]; 4]),
            uvs: Some(vec![[0.0, 0.0], [uv, 0.0], [uv, uv], [0.0, uv]]),
            indices: vec![0, 2, 1, 0, 3, 2],
            ..MeshData::default()
        };
        let mut node = Node::new("quad");
        node.mesh = Some(0);
        let material = Material { base_color_texture: Some(texture.into()), ..Material::default() };
        Ok(Model { nodes: vec![node], meshes: vec![mesh], materials: vec![material] })
    }
}

struct Files(Mutex<HashMap<String, Vec<u8>>>);

impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.lock().unwrap().get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0.lock().unwrap().get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

struct World(Vec<Placement>);

impl WorldSource for World {
    fn regions(&self, _: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        out(RegionId::from_key("0"), Fingerprint::of(&(self.0.len() as u64).to_le_bytes()));
    }
    fn placements(&self, _: &WorldspaceId, _: RegionId, out: &mut dyn FnMut(Placement)) {
        self.0.iter().cloned().for_each(out);
    }
}

fn placement(k: u64, model: &str, pos: DVec3, scale: f32) -> Placement {
    Placement {
        id: PlacementId::from_key("t", &k.to_le_bytes()),
        base: RecordRef::new("t.obj", model),
        model: Some(VfsPath::new(model).unwrap()),
        transform: WorldTransform { pos: WorldPos::from_f64(pos), rotation: Quat::IDENTITY, scale: Vec3::splat(scale) },
        class: PlacementClass::Static,
        flags: PlacementFlags::empty(),
    }
}

/// A fresh store folder under the system temp dir (removed on drop).
struct TempStore(std::path::PathBuf);

impl TempStore {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("sf_scene_demand_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        Self(dir)
    }
}

impl Drop for TempStore {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn ws() -> WorldspaceId {
    WorldspaceId::new("t/ext")
}

fn baker(store: &TempStore, files: Arc<Files>, world: Vec<Placement>) -> Baker {
    let mut b = Baker::new(Store::open(&store.0).unwrap(), files);
    b.register(MeshCookStep { importers: Importers::new().with(Arc::new(QuadImporter)), settings: CookSettings::default(), tangents: None });
    WorldBake::new(Arc::new(World(world)), IndexParams::default()).register(&mut b);
    Arc::new(DemandBake::new(Arc::new(BaseTextures))).register(&mut b);
    let r = WorldBake::bake_worldspaces(&b, &[ws()]);
    assert_eq!(r.failures().count(), 0);
    b
}

fn demands(b: &Baker) -> BTreeMap<String, CellDemand> {
    DemandBake::cell_keys(b, &[ws()])
        .unwrap()
        .into_iter()
        .map(|k| {
            let d = CellDemand::read(&b.store().get(&b.product(&k).unwrap()).unwrap()).unwrap();
            (k.subject, d)
        })
        .collect()
}

#[test]
fn a_changed_model_re_runs_only_its_density_and_the_cells_that_place_it() {
    let store = TempStore::new("trace");
    let files = Arc::new(Files(Mutex::new(HashMap::from([
        ("a.quad".to_owned(), b"1 1 textures/a.dds".to_vec()),
        ("b.quad".to_owned(), b"1 2 textures/b.dds".to_vec()),
        ("c.quad".to_owned(), b"2 1 textures/c.dds".to_vec()),
    ]))));
    // Cell X (around x = 10 m) places a and c, cell Y (x = 300 m) b and c; c at scale 2 in Y.
    let world = vec![
        placement(1, "a.quad", DVec3::new(10.0, 0.0, 10.0), 1.0),
        placement(2, "c.quad", DVec3::new(12.0, 0.0, 10.0), 1.0),
        placement(3, "b.quad", DVec3::new(300.0, 0.0, 10.0), 1.0),
        placement(4, "c.quad", DVec3::new(302.0, 0.0, 10.0), 2.0),
    ];
    let first = baker(&store, files.clone(), world.clone());
    let report = DemandBake::bake_worldspaces(&first, &[ws()]);
    assert!(report.failures.is_empty(), "{:?}", report.failures);
    assert_eq!((report.cells, report.clusters, report.textures), (2, 2, 3), "{report}");
    let cells = demands(&first);
    let (x, y) = (&cells.values().next().unwrap().clusters, &cells.values().nth(1).unwrap().clusters);
    let rho = |c: &[crate::demand::DemandCluster], path: &str| {
        c[0].textures.iter().find(|t| t.id == texture_id(path)).and_then(|t| t.rho)
    };
    assert_eq!((rho(x, "textures/a.dds"), rho(x, "textures/c.dds")), (Some(1.0), Some(0.5)));
    assert_eq!((rho(y, "textures/b.dds"), rho(y, "textures/c.dds")), (Some(2.0), Some(0.25)), "c's ρ halves at scale 2");

    // a changes (now tiled 4 times): its cook + density, and cell X only, re-run.
    files.0.lock().unwrap().insert("a.quad".into(), b"1 4 textures/a.dds".to_vec());
    let second = baker(&store, files, world);
    let report = DemandBake::bake_worldspaces(&second, &[ws()]);
    assert!(report.failures.is_empty());
    let stats = second.step_stats();
    let built_hits = |step: &str| (stats[step].built, stats[step].hits);
    assert_eq!(built_hits(MeshUvDensityStep::ID), (1, 2), "a rebuilt; b and c hit");
    assert_eq!(built_hits(DEMAND_STEP), (1, 1), "cell X rebuilt, Y a hit");
    let x = &demands(&second).into_values().next().unwrap().clusters;
    assert_eq!(rho(x, "textures/a.dds"), Some(4.0));
}

#[test]
fn model_demand_lists_every_texture_once_with_its_densest_use() {
    // Two materials of one model on the same texture at 1 and 3 UV/m, and a third texture.
    let mut model = QuadImporter.import("m.quad", b"1 1 textures/a.dds").unwrap();
    let mut second = model.meshes[0].clone();
    second.material = 1;
    second.uvs = Some(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]]);
    model.meshes.push(second);
    let mut node = Node::new("second");
    node.mesh = Some(1);
    model.nodes.push(node);
    model.materials.push(model.materials[0].clone());
    model.materials.push(Material { base_color_texture: Some("textures/z.dds".into()), ..Material::default() });
    let cooked = sf_model::cook(&model, &CookSettings::default()).unwrap();
    let density = sf_model::uv_density::UvDensity::of(&cooked).unwrap();
    let demand = model_demand(&BaseTextures, &cooked, &density).unwrap();
    let a = demand.textures.iter().find(|t| t.id == texture_id("textures/a.dds")).unwrap();
    assert!((a.rho.unwrap() - 3.0).abs() < 1e-5);
    let z = demand.textures.iter().find(|t| t.id == texture_id("textures/z.dds")).unwrap();
    assert_eq!(z.rho, None, "an unused material has no density");
    assert_eq!(demand.textures.len(), 2);
    // A placement too far for one cluster: two clusters, each with the model's textures.
    let at = |x| placement(0, "m.quad", DVec3::new(x, 0.0, 0.0), 1.0);
    let cell = build_cell_demand(glam::IVec3::ZERO, &[at(0.0), at(5.0), at(60.0)], &[Some(&demand); 3]);
    assert_eq!(cell.clusters.len(), 2);
    assert!(cell.clusters.iter().all(|c| c.sphere.radius <= MAX_CLUSTER_RADIUS && c.textures.len() == 2));
}

#[test]
#[ignore = "measurement: texture.demand + mesh.uv_density step time on a synthetic world"]
fn measure_demand_bake() {
    // 32 × 32 streaming cells (128 m) of 40 placements over 200 models: 40,960 placements, about
    // the vanilla exterior's static count.
    let (side, per_cell, models) = (32u32, 40u32, 200u32);
    let store = TempStore::new("measure");
    let files: HashMap<String, Vec<u8>> = (0..models)
        .map(|m| (format!("m{m}.quad"), format!("{} {} textures/t{}.dds", 1 + m % 7, 1 + m % 3, m % 150).into_bytes()))
        .collect();
    let mut seed = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut world = Vec::new();
    for k in 0..side * side * per_cell {
        let (cx, cz) = (f64::from(k / per_cell % side), f64::from(k / per_cell / side));
        #[allow(clippy::cast_precision_loss)]
        let (dx, dz) = ((next() % 12_800) as f64 / 100.0, (next() % 12_800) as f64 / 100.0);
        let model = format!("m{}.quad", next() % u64::from(models));
        world.push(placement(u64::from(k), &model, DVec3::new(cx * 128.0 + dx, 0.0, cz * 128.0 + dz), 1.0));
    }
    let b = baker(&store, Arc::new(Files(Mutex::new(files))), world);
    let cook = Instant::now();
    let keys: Vec<ProductKey> = (0..models).map(|m| ProductKey::new(MeshCookStep::ID, format!("m{m}.quad"))).collect();
    b.bake(&keys);
    let cook = cook.elapsed();
    let report = DemandBake::bake_worldspaces(&b, &[ws()]);
    let stats = b.step_stats();
    print!("{report}");
    println!(
        "measure: {} placements; mesh.cook {cook:.2?} (not part of S3b); run time summed over threads: mesh.uv_density {:.3} s ({} built), texture.demand {:.3} s ({} built)",
        side * side * per_cell,
        stats[MeshUvDensityStep::ID].run_time.as_secs_f64(),
        stats[MeshUvDensityStep::ID].built,
        stats[DEMAND_STEP].run_time.as_secs_f64(),
        stats[DEMAND_STEP].built,
    );
}
