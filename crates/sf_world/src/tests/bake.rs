use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

use glam::{DVec3, IVec3};
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, Store};
use sf_model::steps::{Importers, ModelInfoStep};
use sf_model::{ImportError, Material, MeshData, Model, ModelImporter, Node};

use super::{placement, ws};
use bake::{BakedIndex, DIR_STEP, LAYOUT_STEP, PAGE_STEP, REGION_STEP, RegionBuckets, WorldBake, page_subject};
use crate::*;

/// Model files: `"<half extent>"` text, imported as a cube of that size.
#[derive(Default)]
struct Files(RwLock<HashMap<String, Vec<u8>>>);

impl Files {
    fn set(&self, path: &str, half: f32) {
        self.0.write().unwrap().insert(path.to_owned(), half.to_string().into_bytes());
    }
}

impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.read().unwrap().get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0.read().unwrap().get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
    }
    fn record_fingerprint(&self, _kind: &str, _id: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

struct CubeImporter;

impl ModelImporter for CubeImporter {
    fn extensions(&self) -> &[&str] {
        &["cube"]
    }
    fn version(&self) -> u32 {
        1
    }
    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        let err = |m: &str| ImportError { path: path.into(), message: m.into() };
        let h: f32 = std::str::from_utf8(bytes).map_err(|_| err("utf8"))?.trim().parse().map_err(|_| err("number"))?;
        let mesh = MeshData {
            positions: vec![[-h, -h, -h], [h, -h, -h], [h, h, h], [-h, h, h]],
            indices: vec![0, 1, 2, 0, 2, 3],
            ..MeshData::default()
        };
        let mut node = Node::new("cube");
        node.mesh = Some(0);
        Ok(Model { nodes: vec![node], meshes: vec![mesh], materials: vec![Material::default()] })
    }
}

/// A world source whose regions the test edits.
#[derive(Default)]
struct Regions(RwLock<BTreeMap<RegionId, Vec<Placement>>>);

impl Regions {
    fn set(&self, key: &str, placements: Vec<Placement>) {
        self.0.write().unwrap().insert(RegionId::from_key(key), placements);
    }
}

impl WorldSource for Regions {
    fn regions(&self, w: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        if *w != ws("ext") {
            return;
        }
        for (r, ps) in self.0.read().unwrap().iter() {
            // A content fingerprint, as adapters compute from their records.
            let text = format!("{ps:?}");
            out(*r, Fingerprint::of(text.as_bytes()));
        }
    }
    fn placements(&self, w: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        if *w != ws("ext") {
            return;
        }
        self.0.read().unwrap().get(&region).into_iter().flatten().cloned().for_each(out);
    }
}

fn row(prefix: &str, x0: f64, z: f64, n: usize, model: &str) -> Vec<Placement> {
    #[allow(clippy::cast_precision_loss)]
    (0..n)
        .map(|i| placement(&format!("{prefix}{i}"), DVec3::new(x0 + i as f64 * 20.0, 0.0, z), Some(model), PlacementClass::Static))
        .collect()
}

struct Fixture {
    _dir: tempfile::TempDir,
    store: Store,
    files: Arc<Files>,
    source: Arc<Regions>,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        let files = Arc::new(Files::default());
        files.set("a.cube", 1.0);
        files.set("b.cube", 5.0);
        files.set("c.cube", 0.5);
        let source = Arc::new(Regions::default());
        // Region "a": page (0,0,0). Region "b": pages (1,0,0) and (2,0,0)
        // (1 km pages). Region "c": page (5,0,5), plus one object in (0,0,0).
        source.set("a", row("a", 10.0, 10.0, 20, "a.cube"));
        source.set("b", row("b", 1100.0, 50.0, 60, "b.cube"));
        let mut c = row("c", 5100.0, 5100.0, 10, "c.cube");
        c.push(placement("c-moved", DVec3::new(300.0, 0.0, 300.0), Some("b.cube"), PlacementClass::Static));
        source.set("c", c);
        Self { _dir: dir, store, files, source }
    }

    /// One bake session over the shared store, returning the baker.
    fn bake(&self) -> Baker {
        let mut baker = Baker::new(self.store.clone(), self.files.clone());
        baker.register(ModelInfoStep { importers: Importers::new().with(Arc::new(CubeImporter)) });
        WorldBake::new(self.source.clone(), IndexParams::default()).register(&mut baker);
        let report = baker.bake(&[WorldBake::directory_key(&ws("ext"))]);
        assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
        baker
    }
}

/// (built, hits) of a step this session.
fn counts(baker: &Baker, step: &str) -> (u32, u32) {
    baker.step_stats().get(step).map_or((0, 0), |s| (s.built, s.hits))
}

#[test]
fn baked_index_matches_the_one_pass_build() {
    let fx = Fixture::new();
    let baker = fx.bake();
    let index = BakedIndex::new(fx.store.clone(), Arc::new(baker.manifest()));

    let models = |p: &sf_vfs::VfsPath| {
        let half = match p.as_str() {
            "a.cube" => 1.0,
            "b.cube" => 5.0,
            "c.cube" => 0.5,
            _ => return None,
        };
        let info = sf_model::ModelInfo {
            bounds: sf_model::Aabb { min: [-half; 3], max: [half; 3] },
            triangles: 2,
            vertices: 4,
            animated: false,
            skinned: false,
        };
        bake::index_model_info(&info)
    };
    let expected = build_index(&ws("ext"), &IndexParams::default(), &*fx.source, &models);

    let dir = CellDirectory::from_bytes(&index.directory(&ws("ext")).unwrap().unwrap()).unwrap();
    assert_eq!(dir, expected.directory);
    assert_eq!(dir.pages.len(), 4);
    for page in &expected.pages {
        let stored = IndexPage::from_bytes(&index.page(&ws("ext"), page.page).unwrap().unwrap()).unwrap();
        assert_eq!(&stored, page);
    }
    assert!(index.page(&ws("ext"), IVec3::new(9, 9, 9)).unwrap().is_none());
    assert!(index.directory(&ws("other")).unwrap().is_none());
}

#[test]
fn changing_one_region_rebuilds_only_its_pages() {
    let fx = Fixture::new();
    let first = fx.bake();
    assert_eq!(counts(&first, REGION_STEP), (3, 0));
    assert_eq!(counts(&first, PAGE_STEP), (4, 0));

    // Nothing changed: everything is a hit.
    let warm = fx.bake();
    assert_eq!(counts(&warm, REGION_STEP), (0, 3));
    assert_eq!(counts(&warm, LAYOUT_STEP), (0, 1));
    assert_eq!(counts(&warm, PAGE_STEP), (0, 4));
    assert_eq!(counts(&warm, DIR_STEP), (0, 1));
    assert_eq!(warm.manifest(), first.manifest());

    // Region "b" (pages (1,0,0) and (2,0,0)) changes; its page set doesn't.
    let mut b = row("b", 1100.0, 50.0, 60, "b.cube");
    b[3].transform.pos = b[3].transform.pos.offset(glam::Vec3::new(0.0, 0.0, 7.0));
    fx.source.set("b", b);
    let edited = fx.bake();
    assert_eq!(counts(&edited, REGION_STEP), (1, 2));
    // The layout re-runs (an input changed) but produces the same bytes...
    assert_eq!(counts(&edited, LAYOUT_STEP), (1, 0));
    // ...so only the pages region "b" touches rebuild, and only the one
    // holding the moved object gets new bytes.
    assert_eq!(counts(&edited, PAGE_STEP), (2, 2));
    assert_eq!(counts(&edited, DIR_STEP), (1, 0));
    let changed: Vec<_> = edited
        .manifest()
        .products
        .iter()
        .filter(|(k, d)| k.step == PAGE_STEP && first.manifest().products.get(k) != Some(d))
        .map(|(k, _)| k.subject.clone())
        .collect();
    assert_eq!(changed, vec![page_subject(&ws("ext"), IVec3::new(1, 0, 0))]);

    // A model file change rebuilds its model.info and the pages using it:
    // "c.cube" only appears in page (5,0,5).
    fx.files.set("c.cube", 0.75);
    let remodeled = fx.bake();
    assert_eq!(counts(&remodeled, REGION_STEP), (0, 3));
    assert_eq!(counts(&remodeled, PAGE_STEP), (1, 3));
}

#[test]
fn region_buckets_round_trip_and_expose_page_keys() {
    let placements = row("r", -900.0, 10.0, 80, "a.cube");
    let buckets = RegionBuckets { partition: Partition::STREAMING, pages: bucket_by_page(Partition::STREAMING, placements) };
    assert!(buckets.pages.len() >= 2);
    let bytes = buckets.to_bytes();
    assert_eq!(RegionBuckets::from_bytes(&bytes).unwrap(), buckets);
    let keys: Vec<IVec3> = buckets.pages.iter().map(|(p, _)| *p).collect();
    assert_eq!(RegionBuckets::page_keys(&bytes).unwrap(), keys);
    assert!(RegionBuckets::from_bytes(&IndexPage { partition: Partition::Single, page: IVec3::ZERO, cells: vec![] }.to_bytes()).is_err());
}

#[test]
fn subjects_survive_slashes_in_worldspace_ids() {
    let w = WorldspaceId::new("mods/odd/name");
    let subject = page_subject(&w, IVec3::new(-3, 0, 12));
    assert_eq!(subject, "mods/odd/name/-3,0,12");
    assert_eq!(bake::parse_page(&subject), Some((w.clone(), IVec3::new(-3, 0, 12))));
    let region = bake::region_subject(&w, RegionId(0xab));
    assert_eq!(bake::parse_region(&region), Some((w, RegionId(0xab))));
    assert_eq!(bake::parse_page("no-slash"), None);
    assert_eq!(bake::parse_page("ws/1,2"), None);
}

#[test]
fn per_worldspace_partitions() {
    let fx = Fixture::new();
    let mut baker = Baker::new(fx.store.clone(), fx.files.clone());
    baker.register(ModelInfoStep { importers: Importers::new().with(Arc::new(CubeImporter)) });
    let single = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    let overrides = BTreeMap::from([(ws("ext"), single)]);
    WorldBake::with_overrides(fx.source.clone(), IndexParams::default(), overrides).register(&mut baker);
    let report = WorldBake::bake_worldspaces(&baker, &[ws("ext")]);
    assert_eq!(report.failures().count(), 0);
    let index = BakedIndex::new(fx.store.clone(), Arc::new(baker.manifest()));
    let dir = CellDirectory::from_bytes(&index.directory(&ws("ext")).unwrap().unwrap()).unwrap();
    assert_eq!(dir.partition, Partition::Single);
    assert_eq!(dir.cells.len(), 1, "a Single worldspace is one cell");
    assert_eq!(dir.cells[0].placements, 20 + 60 + 10 + 1);
}

/// Worldspaces with one region each (an exterior and interiors).
struct Worlds(BTreeMap<WorldspaceId, Vec<Placement>>);

impl WorldSource for Worlds {
    fn regions(&self, w: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        if let Some(ps) = self.0.get(w) {
            out(RegionId::from_key(w.as_str()), Fingerprint::of(format!("{ps:?}").as_bytes()));
        }
    }
    fn placements(&self, w: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        if region == RegionId::from_key(w.as_str()) {
            self.0.get(w).into_iter().flatten().cloned().for_each(out);
        }
    }
}

/// Interiors index as their own `Single` worldspaces next to a gridded
/// exterior, in one parallel bake; the result doesn't depend on the order
/// the worldspaces are named in.
#[test]
fn interiors_index_as_single_worldspaces() {
    let files = Arc::new(Files::default());
    files.set("a.cube", 1.0);
    let mut worlds = BTreeMap::from([(ws("ext"), row("e", 10.0, 10.0, 80, "a.cube"))]);
    let interiors: Vec<WorldspaceId> = (0..5).map(|i| ws(&format!("Town, House {i}"))).collect();
    for (i, w) in interiors.iter().enumerate() {
        // Interiors spread over hundreds of meters still load whole.
        worlds.insert(w.clone(), row(&format!("i{i}-"), -400.0, f64::from(u32::try_from(i).unwrap()) * 300.0, 30, "a.cube"));
    }
    let source = Arc::new(Worlds(worlds));
    let single = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    let overrides: BTreeMap<_, _> = interiors.iter().map(|w| (w.clone(), single)).collect();
    let bake = |order: &[WorldspaceId]| {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        let mut baker = Baker::new(store.clone(), files.clone());
        baker.register(ModelInfoStep { importers: Importers::new().with(Arc::new(CubeImporter)) });
        WorldBake::with_overrides(source.clone(), IndexParams::default(), overrides.clone()).register(&mut baker);
        let report = WorldBake::bake_worldspaces(&baker, order);
        assert_eq!(report.failures().count(), 0);
        (dir, store, baker.manifest())
    };
    let mut order = vec![ws("ext")];
    order.extend(interiors.iter().cloned());
    let (_d1, store, manifest) = bake(&order);
    order.reverse();
    let (_d2, _, manifest2) = bake(&order);
    assert_eq!(manifest.products, manifest2.products, "same products whatever the order");

    let index = BakedIndex::new(store, Arc::new(manifest));
    let ext = CellDirectory::from_bytes(&index.directory(&ws("ext")).unwrap().unwrap()).unwrap();
    assert_eq!(ext.partition, Partition::STREAMING);
    assert!(ext.cells.len() > 1);
    for w in &interiors {
        let dir = CellDirectory::from_bytes(&index.directory(w).unwrap().expect("every interior has a directory")).unwrap();
        assert_eq!(dir.partition, Partition::Single, "{w:?}");
        assert_eq!(dir.cells.len(), 1, "{w:?} loads whole");
        assert_eq!(dir.cells[0].placements, 30);
    }
}
