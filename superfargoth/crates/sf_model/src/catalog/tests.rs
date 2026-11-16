//! `pipeline.catalog` over small synthetic models baked through `mesh.cook` / `mesh.rig`.

use std::collections::HashMap;
use std::sync::Arc;

use sf_bake::{BakeError, Baker, Digest, Inputs, ProductKey, Store};

use super::*;
use crate::steps::{Importers, MeshCookStep, MeshRigStep};
use crate::{CookSettings, ImportError, MeshData, Model, ModelImporter, Node};

/// Test importer: the file is a postcard-serialized `Model`.
struct PostcardImporter;
impl ModelImporter for PostcardImporter {
    fn extensions(&self) -> &[&str] {
        &["pcm"]
    }
    fn version(&self) -> u32 {
        1
    }
    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        postcard::from_bytes(bytes).map_err(|e| ImportError { path: path.into(), message: e.to_string() })
    }
}

struct Files(HashMap<String, Vec<u8>>);
impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0.get(path).cloned().ok_or_else(|| BakeError::Other(format!("missing {path}")))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

fn triangle(material: u32) -> MeshData {
    MeshData {
        positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        normals: Some(vec![[0.0, 0.0, 1.0]; 3]),
        indices: vec![0, 1, 2],
        material,
        ..MeshData::default()
    }
}

/// One node, one triangle, one material.
fn prop(material: Material) -> Model {
    let mut root = Node::new("root");
    root.mesh = Some(0);
    Model { nodes: vec![root], meshes: vec![triangle(0)], materials: vec![material] }
}

/// Two rigid parts under two transform nodes: `mesh.rig` skins it, `mesh.cook` keeps it static.
fn segmented() -> Model {
    let body = Node::new("body");
    let mut leg = Node::new("leg");
    leg.parent = Some(0);
    leg.translation = [0.0, 2.0, 0.0];
    let mut torso = Node::new("torso shape");
    torso.parent = Some(0);
    torso.mesh = Some(0);
    let mut shin = Node::new("shin shape");
    shin.parent = Some(1);
    shin.mesh = Some(1);
    Model { nodes: vec![body, leg, torso, shin], meshes: vec![triangle(0), triangle(0)], materials: vec![Material::default()] }
}

struct Source(Vec<CatalogModel>);
impl CatalogSource for Source {
    fn fingerprint(&self) -> [u8; 32] {
        [7; 32]
    }
    fn models(&self) -> &[CatalogModel] {
        &self.0
    }
    fn builtins(&self) -> Vec<String> {
        vec!["water".into(), "terrain".into(), "water".into()]
    }
}

const STATIC: EntityFlags = EntityFlags { shadow_caster: true, crossfade: true };

fn model(step: &str, subject: &str) -> CatalogModel {
    CatalogModel { step: step.into(), subject: subject.into(), flags: STATIC }
}

fn files() -> Arc<Files> {
    let masked = Material { alpha: AlphaMode::Mask(0.5), ..Material::default() };
    let tinted = Material { name: "tinted".into(), base_color: [0.5, 0.2, 0.1, 1.0], ..Material::default() };
    let mut files = HashMap::new();
    for (path, m) in [
        ("meshes/a.pcm", prop(Material::default())),
        // Same class and layout as `a`, other per-material data: one specimen.
        ("meshes/b.pcm", prop(tinted)),
        ("meshes/c.pcm", prop(masked)),
        ("meshes/body.pcm", segmented()),
    ] {
        files.insert(path.to_owned(), postcard::to_stdvec(&m).unwrap());
    }
    Arc::new(Files(files))
}

/// Bakes the catalog in a fresh store; returns its bytes.
fn bake(models: Vec<CatalogModel>) -> Vec<u8> {
    let dir = tempfile::tempdir().unwrap();
    let importers = Importers::new().with(Arc::new(PostcardImporter));
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files());
    baker
        .register(MeshCookStep { importers: importers.clone(), settings: CookSettings::default(), tangents: None })
        .register(MeshRigStep { importers, settings: CookSettings::default(), tangents: None })
        .register(CatalogStep(Arc::new(Source(models))));
    let key = ProductKey::new(CATALOG_STEP, CATALOG_SUBJECT);
    let report = baker.bake(std::slice::from_ref(&key));
    assert_eq!(report.failures().count(), 0, "{:?}", report.products);
    let catalog = load_catalog(baker.store(), &baker.manifest()).expect("the catalog loads back");
    let bytes = baker.store().get(&baker.product(&key).unwrap()).unwrap();
    assert_eq!(Catalog::decode(&bytes).unwrap(), catalog);
    bytes
}

fn models() -> Vec<CatalogModel> {
    vec![
        model("mesh.cook", "meshes/a.pcm"),
        model("mesh.cook", "meshes/b.pcm"),
        model("mesh.cook", "meshes/c.pcm"),
        model("mesh.cook", "meshes/body.pcm"),
        model("mesh.rig", "meshes/body.pcm"),
        // Not in the files: counted as failed, not fatal.
        model("mesh.cook", "meshes/missing.pcm"),
    ]
}

#[test]
fn catalog_is_deterministic_deduplicated_and_splits_skinned_layouts() {
    let bytes = bake(models());
    let mut shuffled = models();
    shuffled.reverse();
    shuffled.push(model("mesh.cook", "meshes/a.pcm"));
    assert_eq!(bake(shuffled), bytes, "same input in any order: byte-identical catalog");

    let catalog = Catalog::decode(&bytes).unwrap();
    assert_eq!(catalog.format, CATALOG_FORMAT);
    let opaque = default_signature(&Material::default());
    let statics: Vec<&CatalogEntry> = catalog.entries.iter().filter(|e| !e.layout.skinned).collect();
    let shared: Vec<&&CatalogEntry> = statics.iter().filter(|e| e.material == opaque).collect();
    assert_eq!(shared.len(), 1, "a, b and the body's static cook share class and layout: one specimen");
    let expected = Representative { step: "mesh.cook".into(), model: "meshes/a.pcm".into(), lod: 0, submesh: 0, material: 0 };
    assert_eq!(shared[0].representative, expected, "the first model in sorted order represents it");
    assert!(shared[0].uses >= 3, "a, b and the body share it: {}", shared[0].uses);
    assert!(statics.iter().any(|e| e.material.fields["alpha"] == "mask"), "the masked material is its own specimen");
    assert_eq!(statics.len(), 2);

    let skinned: Vec<&CatalogEntry> = catalog.entries.iter().filter(|e| e.layout.skinned).collect();
    assert_eq!(skinned.len(), 1, "the rig's skinned layout is a distinct entry");
    assert_eq!(skinned[0].material, opaque, "same material as the static specimen, other layout");
    assert_eq!(skinned[0].representative.step, "mesh.rig");
    assert_eq!(catalog.builtins, ["terrain", "water"]);
    assert_eq!(catalog.stats.failed, 1);
    assert_eq!(catalog.stats.models, 5);
    println!(
        "pipeline.catalog (synthetic): {} specimens ({} skinned), {} builtins; models {}, submeshes {}, skipped {}, failed {}, excluded {}",
        catalog.entries.len(),
        catalog.skinned(),
        catalog.builtins.len(),
        catalog.stats.models,
        catalog.stats.submeshes,
        catalog.stats.skipped,
        catalog.stats.failed,
        catalog.stats.excluded,
    );
}

#[test]
fn excluded_materials_and_entity_flags() {
    let reader = |m: &Material| (m.alpha != AlphaMode::Blend).then(|| default_signature(m));
    let mut model_data = prop(Material { alpha: AlphaMode::Blend, ..Material::default() });
    model_data.materials.push(Material::default());
    model_data.meshes.push(triangle(1));
    let mut second = Node::new("second");
    second.parent = Some(0);
    second.mesh = Some(1);
    model_data.nodes.push(second);
    let Cooked::Static(cooked) = crate::cook(&model_data, &CookSettings::default()).unwrap() else { panic!("static") };
    let bytes = Cooked::Static(cooked).to_bytes();
    let actor = EntityFlags { shadow_caster: true, crossfade: false };
    let models = [model("mesh.cook", "x"), CatalogModel { flags: actor, ..model("mesh.cook", "y") }];
    let (found, stats) = CatalogStep::enumerate(&models, &reader, |_| Ok(bytes.clone()));
    assert!(stats.excluded > 0, "the blended submesh is kept out");
    assert_eq!(found.len(), 2, "other entity flags: another specimen of the same submesh");
    assert!(found.keys().all(|(_, m, _)| m.fields["alpha"] == "opaque"));
}
