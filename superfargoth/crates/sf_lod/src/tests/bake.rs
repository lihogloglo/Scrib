//! The `lod.*` steps end to end on a tiny generated world: products exist
//! for every level, impostor policy and atlases work, and bytes are the
//! same with 1 and 8 bake threads.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use glam::{DVec3, Quat, Vec3};
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, Manifest, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::steps::{Importers, MeshCookStep, MeshRestStep, ModelInfoStep};
use sf_model::{AlphaMode, CookSettings, ImportError, Material, MeshData, Model, ModelImporter, Node};
use sf_vfs::VfsPath;
use sf_world::bake::WorldBake;
use sf_world::{
    Fingerprint, IndexParams, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, RegionId, WorldSource, WorldspaceId,
};

use super::Rng;
use crate::bake::{FarBake, IMPOSTOR_STEP, POLICY_STEP, decode_policy};
use crate::{FarBakeSettings, FarDirectory, FarPolicy, ImpostorAtlas, NodeBlob};

fn box_mesh(min: Vec3, max: Vec3, color: [f32; 4]) -> MeshData {
    let mut m = MeshData::default();
    let corners: Vec<Vec3> =
        (0..8).map(|i| Vec3::new(if i & 1 == 0 { min.x } else { max.x }, if i & 2 == 0 { min.y } else { max.y }, if i & 4 == 0 { min.z } else { max.z })).collect();
    let faces = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
    for f in faces {
        let base = u32::try_from(m.positions.len()).expect("small");
        let n = (corners[f[1]] - corners[f[0]]).cross(corners[f[2]] - corners[f[0]]).normalize();
        for k in f {
            m.positions.push(corners[k].to_array());
            m.normals.get_or_insert_with(Vec::new).push(n.to_array());
            m.colors.get_or_insert_with(Vec::new).push(color);
        }
        m.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    m
}

fn models() -> BTreeMap<&'static str, Model> {
    let mut out = BTreeMap::new();
    let mut house = Node::new("house");
    house.mesh = Some(0);
    out.insert(
        "house.mdl",
        Model { nodes: vec![house], meshes: vec![box_mesh(Vec3::new(-5.0, 0.0, -4.0), Vec3::new(5.0, 5.0, 4.0), [0.8, 0.7, 0.6, 1.0])], materials: vec![Material::default()] },
    );
    // An animated model (a "mill" whose only node turns): `mesh.cook` skips
    // it, so the far field uses its rest pose (`mesh.rest`).
    let mut mill = Node::new("mill");
    mill.mesh = Some(0);
    mill.animated = true;
    out.insert(
        "mill.mdl",
        Model { nodes: vec![mill], meshes: vec![box_mesh(Vec3::new(-3.0, 0.0, -3.0), Vec3::new(3.0, 8.0, 3.0), [0.6, 0.6, 0.6, 1.0])], materials: vec![Material::default()] },
    );
    // A "tree": a trunk plus tall alpha-masked cards (impostor by default).
    let mut trunk = Node::new("trunk");
    trunk.mesh = Some(0);
    let mut leaves = Node::new("leaves");
    leaves.mesh = Some(1);
    let mut cards = box_mesh(Vec3::new(-2.0, 1.0, -0.05), Vec3::new(2.0, 9.0, 0.05), [0.2, 0.5, 0.2, 1.0]);
    cards.material = 1;
    let mask = Material { alpha: AlphaMode::Mask(0.5), double_sided: true, base_color_texture: Some("leaf.tex".into()), ..Material::default() };
    out.insert(
        "tree.mdl",
        Model {
            nodes: vec![trunk, leaves],
            meshes: vec![box_mesh(Vec3::new(-0.2, 0.0, -0.2), Vec3::new(0.2, 1.0, 0.2), [0.4, 0.3, 0.2, 1.0]), cards],
            materials: vec![Material::default(), mask],
        },
    );
    out
}

struct Files(HashMap<String, Vec<u8>>);

impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0.get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

struct PostcardImporter;

impl ModelImporter for PostcardImporter {
    fn extensions(&self) -> &[&str] {
        &["mdl"]
    }
    fn version(&self) -> u32 {
        1
    }
    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        postcard::from_bytes(bytes).map_err(|e| ImportError { path: path.into(), message: e.to_string() })
    }
}

struct Source(BTreeMap<RegionId, Vec<Placement>>);

impl WorldSource for Source {
    fn regions(&self, _: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        for r in self.0.keys() {
            out(*r, Fingerprint::of(&r.0.to_le_bytes()));
        }
    }
    fn placements(&self, _: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        self.0.get(&region).into_iter().flatten().cloned().for_each(out);
    }
}

fn world() -> Source {
    let mut rng = Rng(11);
    let mut regions: BTreeMap<RegionId, Vec<Placement>> = BTreeMap::new();
    for i in 0..600u32 {
        let pos = DVec3::new(rng.next() * 2500.0 - 400.0, 0.0, rng.next() * 1500.0 - 300.0);
        let (model, class) = match i % 7 {
            0 => ("house.mdl", PlacementClass::Light),
            1 => ("mill.mdl", PlacementClass::Static),
            2 | 3 => ("house.mdl", PlacementClass::Static),
            _ => ("tree.mdl", PlacementClass::Static),
        };
        #[allow(clippy::cast_possible_truncation)]
        let region = RegionId::from_key(&format!("{}", (pos.x / 1000.0).floor() as i64));
        regions.entry(region).or_default().push(Placement {
            id: PlacementId::from_key("t", &i.to_le_bytes()),
            base: RecordRef::new("t.obj", model),
            model: Some(VfsPath::new(model).expect("path")),
            #[allow(clippy::cast_possible_truncation)]
            transform: WorldTransform { pos: WorldPos::from_f64(pos), rotation: Quat::from_rotation_y(rng.next() as f32 * 6.0), scale: Vec3::ONE },
            class,
            flags: PlacementFlags::empty(),
        });
    }
    Source(regions)
}

fn bake(threads: usize, dir: &std::path::Path) -> (Store, Manifest) {
    let mut files: HashMap<String, Vec<u8>> =
        models().into_iter().map(|(k, m)| (k.to_owned(), postcard::to_stdvec(&m).expect("serializes"))).collect();
    // A 4×4 leaf texture: green, alpha checkered.
    let mut leaf = vec![4u8, 4];
    for i in 0..16u8 {
        leaf.extend([40, 160, 50, if (i + i / 4) % 2 == 0 { 255 } else { 0 }]);
    }
    files.insert("leaf.tex".into(), leaf);
    let store = Store::open(dir).expect("store");
    let mut baker = Baker::new(store.clone(), Arc::new(Files(files)));
    let importers = Importers::new().with(Arc::new(PostcardImporter));
    baker.register(ModelInfoStep { importers: importers.clone() });
    baker.register(MeshCookStep { importers: importers.clone(), settings: CookSettings::default() });
    baker.register(MeshRestStep { importers, settings: CookSettings::default() });
    WorldBake::new(Arc::new(world()), IndexParams::default()).register(&mut baker);
    FarBake::with_textures(Arc::new(crate::DefaultFarSource), FarBakeSettings::default(), Arc::new(RawDecoder)).register(&mut baker);
    let ws = WorldspaceId::new("t/ext");
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().expect("pool");
    pool.install(|| {
        let r = WorldBake::bake_worldspaces(&baker, std::slice::from_ref(&ws));
        assert_eq!(r.failures().count(), 0);
        let r = FarBake::bake_worldspaces(&baker, std::slice::from_ref(&ws));
        let failures: Vec<String> = r.failures().map(|(k, e)| format!("{k}: {e}")).collect();
        assert!(failures.is_empty(), "{failures:?}");
    });
    (store, baker.manifest())
}

#[test]
fn far_bake_is_complete_and_deterministic() {
    let (a_dir, b_dir) = (tempfile::tempdir().expect("tmp"), tempfile::tempdir().expect("tmp"));
    let (store, a) = bake(1, a_dir.path());
    let (_, b) = bake(8, b_dir.path());
    let lod = |m: &Manifest| -> Vec<(String, Digest)> {
        m.products.iter().filter(|(k, _)| k.step.starts_with("lod.")).map(|(k, d)| (format!("{k}"), *d)).collect()
    };
    let (la, lb) = (lod(&a), lod(&b));
    assert!(la.len() > 30, "{} lod products", la.len());
    assert_eq!(la, lb, "same bytes with 1 and 8 threads");

    let dir = FarDirectory::from_bytes(&store.get(&a.get("lod.dir", "t/ext").expect("dir")).expect("bytes")).expect("decodes");
    for level in 1..=3 {
        assert!(dir.level(level).count() > 0, "level {level} has nodes");
    }
    assert_eq!(dir.impostor_models.len(), 1, "the tree");
    assert_eq!(dir.impostor_models[0].model, "tree.mdl");
    let product = decode_policy(&store.get(&a.get(POLICY_STEP, "tree.mdl").expect("policy")).expect("bytes")).expect("decodes");
    let policy = product.policy;
    // The leaf texture: green, half its texels cut out.
    let leaf = product.albedo[1];
    assert!((leaf[3] - 0.5).abs() < 1e-6 && leaf[1] > leaf[0] * 3.0, "leaf albedo {leaf:?}");
    assert_eq!(policy, FarPolicy::Impostor);
    let mill = decode_policy(&store.get(&a.get(POLICY_STEP, "mill.mdl").expect("policy")).expect("bytes")).expect("decodes");
    assert_eq!(mill.policy, FarPolicy::Merge, "an animated model merges through its rest pose");
    assert!(a.get("mesh.rest", "mill.mdl").is_some() && a.get("mesh.rest", "house.mdl").is_none(), "rest cooks only skipped models");
    let atlas = ImpostorAtlas::from_bytes(&store.get(&a.get(IMPOSTOR_STEP, "tree.mdl").expect("atlas")).expect("bytes")).expect("decodes");
    assert!(atlas.coverage() > 0.05, "coverage {}", atlas.coverage());

    let l = FarBakeSettings::default().levels;
    let mut lights = 0;
    let mut instances = 0;
    for n in &dir.nodes {
        let h = &n.header;
        assert!(h.error <= l.error_budget(h.key.level, h.lead) + 1e-4, "{:?}: error {} > budget", h.key, h.error);
        if h.key.level == 1 {
            lights += h.lights;
            instances += h.instances;
        }
        let blob = NodeBlob::from_bytes(&store.get(&a.get(crate::bake::level_step(h.key.level), &crate::bake::node_subject(&dir.worldspace, h.key)).expect("node")).expect("bytes")).expect("decodes");
        assert_eq!(&blob.header, h);
    }
    assert_eq!(lights, 600 / 7 + 1, "every Light placement is a sprite at L1");
    assert!(instances > 250, "trees are impostors at L1: {instances}");

    // Near precombines (precombine.md): every static house and tree joins its
    // cell's precombine; lights (class) and the animated mill (not cooked) don't.
    let mut members = 0;
    for (key, digest) in a.products.iter().filter(|(k, _)| k.step == crate::bake::NEAR_STEP) {
        let near = crate::NearCombine::from_bytes(&store.get(digest).expect("bytes")).unwrap_or_else(|e| panic!("{key}: {e}"));
        members += near.member_count();
    }
    assert_eq!(members, 600 - 2 * 86, "houses and trees");
}

/// Test textures: `[width, height, rgba…]`.
struct RawDecoder;

impl crate::TextureDecoder for RawDecoder {
    fn version(&self) -> u32 {
        1
    }
    fn decode(&self, _path: &str, bytes: &[u8]) -> Option<crate::TextureImage> {
        Some(crate::TextureImage { width: u32::from(bytes[0]), height: u32::from(bytes[1]), rgba: bytes[2..].to_vec() })
    }
}
