use std::collections::HashMap;
use std::sync::Arc;

use sf_bake::{BakeError, Baker, Digest, Inputs, ProductKey, Store};

use super::*;
use crate::steps::Importers;
use crate::{CookSettings, ImportError, Material, MeshData, Model, ModelImporter, Node, cook};

/// A `size` m square in the XZ plane with UVs `0..uv` (two triangles), material `material`.
fn quad(size: f32, uv: f32, material: u32) -> MeshData {
    MeshData {
        positions: vec![[0.0, 0.0, 0.0], [size, 0.0, 0.0], [size, 0.0, size], [0.0, 0.0, size]],
        normals: Some(vec![[0.0, 1.0, 0.0]; 4]),
        uvs: Some(vec![[0.0, 0.0], [uv, 0.0], [uv, uv], [0.0, uv]]),
        indices: vec![0, 2, 1, 0, 3, 2],
        material,
        ..MeshData::default()
    }
}

fn model(meshes: Vec<MeshData>, materials: usize) -> Model {
    let nodes = (0..u32::try_from(meshes.len()).unwrap())
        .map(|i| {
            let mut n = Node::new(format!("n{i}"));
            n.mesh = Some(i);
            n
        })
        .collect();
    Model { nodes, meshes, materials: vec![Material::default(); materials] }
}

fn density(m: &Model) -> Vec<Option<f32>> {
    let Cooked::Static(c) = cook(m, &CookSettings::default()).unwrap() else { panic!("static") };
    uv_density(&c).unwrap()
}

fn close(a: Option<f32>, b: f32) -> bool {
    a.is_some_and(|a| (a - b).abs() <= 1e-5 * b.max(1.0))
}

#[test]
fn a_one_metre_quad_with_unit_uvs_is_one_uv_per_metre() {
    let rho = density(&model(vec![quad(1.0, 1.0, 0)], 1));
    assert!(close(rho[0], 1.0), "{rho:?}");
}

#[test]
fn uv_scale_and_geometry_scale_move_rho_as_expected() {
    // Tiling twice over the same metre: twice the UV per metre.
    assert!(close(density(&model(vec![quad(1.0, 2.0, 0)], 1))[0], 2.0));
    // The same UVs over a 4 m square: a quarter.
    assert!(close(density(&model(vec![quad(4.0, 1.0, 0)], 1))[0], 0.25));
    // Both: 8 UV over 2 m.
    assert!(close(density(&model(vec![quad(2.0, 8.0, 0)], 1))[0], 4.0));
}

#[test]
fn degenerate_triangles_are_ignored() {
    let mut q = quad(1.0, 1.0, 0);
    // A zero-area triangle (collinear points) and a UV-collapsed one (all three on one texel).
    let base = u32::try_from(q.positions.len()).unwrap();
    q.positions.extend([[5.0, 0.0, 0.0], [6.0, 0.0, 0.0], [7.0, 0.0, 0.0], [0.0, 3.0, 0.0], [1.0, 3.0, 0.0], [0.0, 4.0, 0.0]]);
    q.normals.as_mut().unwrap().extend([[0.0, 1.0, 0.0]; 6]);
    q.uvs.as_mut().unwrap().extend([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.5, 0.5], [0.5, 0.5], [0.5, 0.5]]);
    q.indices.extend([base, base + 1, base + 2, base + 3, base + 4, base + 5]);
    // Checked on raw triangles too (the cooker may drop a zero-area triangle itself).
    assert!(triangle_density([[5.0, 0.0, 0.0], [6.0, 0.0, 0.0], [7.0, 0.0, 0.0]], [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]).is_none());
    assert!(triangle_density([[0.0, 3.0, 0.0], [1.0, 3.0, 0.0], [0.0, 4.0, 0.0]], [[0.5, 0.5]; 3]).is_none());
    assert!(close(density(&model(vec![q], 1))[0], 1.0));
    // Only degenerate triangles: no density (the streamer's fallback).
    let mut acc = DensityAccumulator::default();
    acc.push([[0.0; 3]; 3], [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    assert_eq!(acc.rho(), None);
}

#[test]
fn materials_get_their_own_density_and_outliers_are_trimmed() {
    // Material 0: 1 UV/m; material 1: 3 UV/m; material 2: unused.
    let rho = density(&model(vec![quad(1.0, 1.0, 0), quad(1.0, 3.0, 1)], 3));
    assert!(close(rho[0], 1.0) && close(rho[1], 3.0) && rho[2].is_none(), "{rho:?}");
    // One sliver at 100 UV/m with under 10 % of the weight drops out of the mean entirely.
    let mut acc = DensityAccumulator::default();
    for _ in 0..20 {
        acc.push([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]], [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    }
    acc.push([[0.0, 0.0, 0.0], [0.01, 0.0, 0.0], [0.0, 0.0, 0.01]], [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    assert!(close(acc.rho(), 1.0), "{:?}", acc.rho());
}

#[test]
fn no_uvs_means_no_density() {
    let mut q = quad(1.0, 1.0, 0);
    q.uvs = None;
    assert_eq!(density(&model(vec![q], 1)), vec![None]);
}

// --- the step ----------------------------------------------------------------

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

#[test]
fn the_step_reads_the_cook_and_only_a_changed_model_re_runs() {
    let dir = tempfile::tempdir().unwrap();
    let file = |m: &Model| postcard::to_stdvec(m).unwrap();
    let baker = |files: HashMap<String, Vec<u8>>| {
        let mut b = Baker::new(Store::open(dir.path()).unwrap(), Arc::new(Files(files)));
        let importers = Importers::new().with(Arc::new(PostcardImporter));
        b.register(MeshCookStep { importers, settings: CookSettings::default(), tangents: None }).register(MeshUvDensityStep);
        b
    };
    let keys = [ProductKey::new(MeshUvDensityStep::ID, "a.pcm"), ProductKey::new(MeshUvDensityStep::ID, "b.pcm")];
    let mut files = HashMap::from([("a.pcm".to_owned(), file(&model(vec![quad(1.0, 1.0, 0)], 1))), ("b.pcm".to_owned(), file(&model(vec![quad(1.0, 2.0, 0)], 1)))]);
    let first = baker(files.clone());
    assert_eq!(first.bake(&keys).failures().count(), 0);
    let read = |b: &Baker, k: &ProductKey| UvDensity::from_bytes(&b.store().get(&b.product(k).unwrap()).unwrap()).unwrap();
    assert!(close(read(&first, &keys[0]).materials[0], 1.0) && close(read(&first, &keys[1]).materials[0], 2.0));
    // b changes: its cook and density re-run, a's are hits.
    files.insert("b.pcm".to_owned(), file(&model(vec![quad(2.0, 2.0, 0)], 1)));
    let second = baker(files);
    assert_eq!(second.bake(&keys).failures().count(), 0);
    let stats = second.step_stats();
    assert_eq!((stats[MeshUvDensityStep::ID].built, stats[MeshUvDensityStep::ID].hits), (1, 1));
    assert_eq!(stats[MeshCookStep::ID].built, 1);
    assert!(close(read(&second, &keys[1]).materials[0], 1.0));
}
