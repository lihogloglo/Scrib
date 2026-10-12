//! The far field on a synthetic 16 × 16 km world: rolling hills (a plain
//! ground mesh standing in for terrain), 30 towns with houses, towers and
//! lamp posts, forests and rocks, ~80k placements. Everything bakes on the
//! fly into a cached store (models, world index, far field), then streams
//! with the far field: objects to ~400 m, L1 proxies with tree impostors to
//! ~2 km, L2 to ~8 km, L3 beyond, light sprites at night.
//!
//! ```text
//! cargo run --release -p sf_lod_bevy --example far_field -- [options]
//!   --at <x,y,z>        camera position (m)        --look-at <x,y,z>
//!   --fov <deg>         vertical field of view (default 70)
//!   --screenshot <png>  wait for streaming and the far field to settle, save, exit
//!   --tint              level tint (L1 green, L2 blue, L3 purple)
//!   --night             night: dark sky, light sprites on
//!   --no-far            plain streaming (no far field), for comparison
//!   --scenario fly      sf_diag fly-through at 60 m/s, writes target/diag/far_fly.json
//!   --bake-only         bake, print timings, exit
//! ```
//! Keys: WASD/QE move, right mouse look, Shift ×10, F2 level tint.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::too_many_lines,
    clippy::many_single_char_names,
    clippy::struct_excessive_bools,
    clippy::too_many_arguments,
    clippy::type_complexity
)]

use std::collections::{BTreeMap, HashMap};
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::RenderLayers;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::light::{CascadeShadowConfigBuilder, GlobalAmbientLight};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::transform::TransformPlugin;
use bevy::window::{PresentMode, WindowResolution};
use glam::DVec3;
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_diag_bevy::sf_diag::{Gate, Keyframe, Measure, Scenario, Warmup};
use sf_diag_bevy::{DiagScenario, DiagSettings, FlyThroughPose, SfDiagPlugin};
use sf_lod::bake::FarBake;
use sf_lod::{FarBakeSettings, FarLight, FarPolicy, FarSource, ModelSummary};
use sf_lod_bevy::{FarFieldDebug, FarFieldPlugin, FarFieldSettings, FarFieldStats, FarLightsNight, baked_extensions, far_streaming_config};
use sf_model::steps::{Importers, MeshCookStep, ModelInfoStep};
use sf_model::{CookSettings, ImportError, Material as ModelMaterial, MeshData, Model, ModelImporter, Node};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_vfs::VfsPath;
use sf_world::bake::{BakedIndex, WorldBake};
use sf_world::{Fingerprint, IndexParams, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, RegionId, WorldSource, Worldspace, WorldspaceId};
use sf_world_bevy::big_space::prelude::{CellCoord, FloatingOrigin};
use sf_world_bevy::{
    ActiveWorldspace, Streaming, StreamingAnchor, StreamingConfig, StreamingSystems, WorldIndexSource, WorldStreamingPlugin,
    WorldspaceRecords, WorldspaceRegistry, to_grid,
};

const GENERATOR: &str = "far-field-example/1";
const HALF: f64 = 8_000.0;

fn exterior() -> WorldspaceId {
    WorldspaceId::new("far/exterior")
}

/// Rolling hills (meters).
fn height(x: f64, z: f64) -> f64 {
    55.0 * (x / 1700.0).sin() * (z / 1450.0).cos() + 22.0 * (x / 520.0 + z / 610.0).sin() + 8.0 * (z / 230.0).sin() * (x / 290.0).cos()
}

// -- arguments -----------------------------------------------------------------------

#[derive(Resource, Clone, Debug, Default)]
struct Args {
    at: Option<DVec3>,
    look_at: Option<DVec3>,
    fov: Option<f32>,
    screenshot: Option<PathBuf>,
    tint: bool,
    night: bool,
    no_far: bool,
    scenario: Option<String>,
    bake_only: bool,
}

fn parse_vec(s: &str) -> DVec3 {
    let v: Vec<f64> = s.split(',').map(|x| x.trim().parse().expect("number")).collect();
    assert_eq!(v.len(), 3, "expected x,y,z");
    DVec3::new(v[0], v[1], v[2])
}

fn parse_args() -> Args {
    let mut args = Args::default();
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = || it.next().unwrap_or_else(|| panic!("{arg} needs a value"));
        match arg.as_str() {
            "--at" => args.at = Some(parse_vec(&value())),
            "--look-at" => args.look_at = Some(parse_vec(&value())),
            "--fov" => args.fov = Some(value().parse().expect("--fov <degrees>")),
            "--screenshot" => args.screenshot = Some(PathBuf::from(value())),
            "--tint" => args.tint = true,
            "--night" => args.night = true,
            "--no-far" => args.no_far = true,
            "--scenario" => args.scenario = Some(value()),
            "--bake-only" => args.bake_only = true,
            other => panic!("unknown argument {other}"),
        }
    }
    args
}

// -- models ----------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        x
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}

fn srgb(r: f32, g: f32, b: f32) -> [f32; 4] {
    let c = Color::srgb(r, g, b).to_linear();
    [c.red, c.green, c.blue, 1.0]
}

#[derive(Default)]
struct Builder(MeshData);

impl Builder {
    fn tri(&mut self, a: Vec3, b: Vec3, c: Vec3, color: [f32; 4]) {
        let n = (b - a).cross(c - a).normalize_or_zero();
        let m = &mut self.0;
        let base = u32::try_from(m.positions.len()).expect("small mesh");
        for p in [a, b, c] {
            m.positions.push(p.to_array());
            m.normals.get_or_insert_with(Vec::new).push(n.to_array());
            m.colors.get_or_insert_with(Vec::new).push(color);
        }
        m.indices.extend([base, base + 1, base + 2]);
    }

    fn quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, color: [f32; 4]) {
        self.tri(a, b, c, color);
        self.tri(a, c, d, color);
    }

    fn cuboid(&mut self, min: Vec3, max: Vec3, color: [f32; 4]) {
        let v = |x: bool, y: bool, z: bool| Vec3::new(if x { max.x } else { min.x }, if y { max.y } else { min.y }, if z { max.z } else { min.z });
        let faces = [
            [v(true, false, false), v(true, true, false), v(true, true, true), v(true, false, true)],
            [v(false, false, true), v(false, true, true), v(false, true, false), v(false, false, false)],
            [v(false, true, false), v(false, true, true), v(true, true, true), v(true, true, false)],
            [v(false, false, true), v(false, false, false), v(true, false, false), v(true, false, true)],
            [v(false, false, true), v(true, false, true), v(true, true, true), v(false, true, true)],
            [v(true, false, false), v(false, false, false), v(false, true, false), v(true, true, false)],
        ];
        for [a, b, c, d] in faces {
            self.quad(a, b, c, d, color);
        }
    }

    fn cone(&mut self, base: Vec3, r0: f32, r1: f32, h: f32, segments: u32, color: [f32; 4]) {
        #[allow(clippy::cast_precision_loss)]
        let ring = |i: u32, r: f32, y: f32| {
            let a = i as f32 / segments as f32 * 2.0 * PI;
            base + Vec3::new(a.cos() * r, y, a.sin() * r)
        };
        for i in 0..segments {
            let (a0, a1) = (ring(i, r0, 0.0), ring(i + 1, r0, 0.0));
            let (b0, b1) = (ring(i, r1, h), ring(i + 1, r1, h));
            if r1 > 1e-4 {
                self.quad(a0, b0, b1, a1, color);
                self.tri(base + Vec3::Y * h, b1, b0, color);
            } else {
                self.tri(a0, b0, a1, color);
            }
            self.tri(base, a0, a1, color);
        }
    }

    fn rock(&mut self, radius: f32, seed: u64, color: [f32; 4]) {
        let mut rng = Rng(seed);
        #[allow(clippy::cast_possible_truncation)]
        let bumps: Vec<(Vec3, f32)> = (0..6)
            .map(|_| {
                let d = Vec3::new(rng.range(-1.0, 1.0) as f32, rng.range(-0.3, 1.0) as f32, rng.range(-1.0, 1.0) as f32);
                (d.normalize_or(Vec3::Y), rng.range(0.1, 0.35) as f32)
            })
            .collect();
        let shape = |d: Vec3| {
            let r = bumps.iter().fold(1.0, |acc, (b, s)| acc + s * d.dot(*b).max(0.0).powi(3));
            Vec3::new(d.x * r * radius, (d.y * 0.7 + 0.15) * r * radius, d.z * r * radius)
        };
        let corners = [Vec3::X, Vec3::Y, Vec3::Z, Vec3::NEG_X, Vec3::NEG_Y, Vec3::NEG_Z];
        let faces = [[1, 0, 2], [1, 2, 3], [1, 3, 5], [1, 5, 0], [4, 2, 0], [4, 3, 2], [4, 5, 3], [4, 0, 5]];
        let n = 8u32;
        for [a, b, c] in faces {
            let (a, b, c) = (corners[a], corners[b], corners[c]);
            #[allow(clippy::cast_precision_loss)]
            let p = |i: u32, j: u32| shape((a + (b - a) * (i as f32 / n as f32) + (c - a) * (j as f32 / n as f32)).normalize());
            for i in 0..n {
                for j in 0..n - i {
                    self.tri(p(i, j), p(i, j + 1), p(i + 1, j), color);
                    if i + j + 1 < n {
                        self.tri(p(i + 1, j), p(i, j + 1), p(i + 1, j + 1), color);
                    }
                }
            }
        }
    }

    fn model(self) -> Model {
        let mut node = Node::new("root");
        node.mesh = Some(0);
        Model { nodes: vec![node], meshes: vec![self.0], materials: vec![ModelMaterial::default()] }
    }
}

fn models() -> BTreeMap<&'static str, Model> {
    let mut out = BTreeMap::new();
    for (k, seed) in [11u64, 23, 37].into_iter().enumerate() {
        let mut m = Builder::default();
        #[allow(clippy::cast_precision_loss)]
        m.rock(1.0, seed, srgb(0.48 + 0.04 * k as f32, 0.45, 0.41));
        out.insert(["rock0.mdl", "rock1.mdl", "rock2.mdl"][k], m.model());
    }
    // A conifer: trunk and three tiers.
    let mut tree = Builder::default();
    tree.cone(Vec3::ZERO, 0.3, 0.2, 3.0, 8, srgb(0.36, 0.25, 0.16));
    for (y, r, h) in [(2.0, 2.4, 3.2), (3.8, 1.9, 2.8), (5.4, 1.3, 2.4)] {
        tree.cone(Vec3::Y * y, r, 0.0, h, 12, srgb(0.16, 0.36, 0.17));
    }
    out.insert("tree.mdl", tree.model());
    // A broadleaf: trunk and a lumpy crown.
    let mut oak = Builder::default();
    oak.cone(Vec3::ZERO, 0.35, 0.25, 3.5, 8, srgb(0.33, 0.24, 0.17));
    let mut crown = Builder::default();
    crown.rock(3.2, 91, srgb(0.27, 0.42, 0.16));
    for p in &mut crown.0.positions {
        p[1] += 4.0;
    }
    oak.0.indices.extend(crown.0.indices.iter().map(|i| i + u32::try_from(oak.0.positions.len()).expect("small")));
    oak.0.positions.extend(crown.0.positions);
    oak.0.normals.get_or_insert_with(Vec::new).extend(crown.0.normals.unwrap_or_default());
    oak.0.colors.get_or_insert_with(Vec::new).extend(crown.0.colors.unwrap_or_default());
    out.insert("oak.mdl", oak.model());
    let mut house = Builder::default();
    house.cuboid(Vec3::new(-5.0, -1.0, -4.0), Vec3::new(5.0, 4.5, 4.0), srgb(0.82, 0.76, 0.62));
    house.cone(Vec3::Y * 4.5, 7.4, 0.0, 3.5, 4, srgb(0.55, 0.24, 0.18));
    house.cuboid(Vec3::new(5.0, 0.0, -0.8), Vec3::new(5.08, 2.2, 0.8), srgb(0.30, 0.20, 0.12));
    out.insert("house.mdl", house.model());
    let mut tower = Builder::default();
    tower.cone(Vec3::NEG_Y * 2.0, 4.0, 3.2, 38.0, 16, srgb(0.62, 0.60, 0.56));
    tower.cone(Vec3::Y * 36.0, 4.4, 0.0, 6.0, 16, srgb(0.35, 0.33, 0.38));
    out.insert("tower.mdl", tower.model());
    let mut crate_ = Builder::default();
    crate_.cuboid(Vec3::splat(-0.5), Vec3::splat(0.5), srgb(0.58, 0.42, 0.25));
    out.insert("crate.mdl", crate_.model());
    // A lamp post: pole and a glowing head.
    let mut lamp = Builder::default();
    lamp.cuboid(Vec3::new(-0.08, 0.0, -0.08), Vec3::new(0.08, 4.0, 0.08), srgb(0.2, 0.2, 0.22));
    lamp.cuboid(Vec3::new(-0.25, 4.0, -0.25), Vec3::new(0.25, 4.5, 0.25), srgb(1.0, 0.85, 0.55));
    out.insert("lamp.mdl", lamp.model());
    out
}

// -- the world ---------------------------------------------------------------------------

struct Gen {
    regions: BTreeMap<RegionId, Vec<Placement>>,
    towns: Vec<DVec3>,
    count: usize,
}

impl Gen {
    fn add(&mut self, model: &str, pos: DVec3, yaw: f32, scale: f32, class: PlacementClass) {
        let key = format!("{}", self.count);
        self.count += 1;
        #[allow(clippy::cast_possible_truncation)]
        let region = RegionId::from_key(&format!("{}/{}", (pos.x / 1000.0).floor() as i64, (pos.z / 1000.0).floor() as i64));
        let ground = DVec3::new(pos.x, pos.y + height(pos.x, pos.z), pos.z);
        self.regions.entry(region).or_default().push(Placement {
            id: PlacementId::from_key("far-example", key.as_bytes()),
            base: RecordRef::new("example.object", model),
            model: Some(VfsPath::new(model).expect("valid path")),
            transform: WorldTransform { pos: WorldPos::from_f64(ground), rotation: Quat::from_rotation_y(yaw), scale: Vec3::splat(scale) },
            class,
            flags: PlacementFlags::GROUNDED,
        });
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn generate() -> Gen {
    let mut g = Gen { regions: BTreeMap::new(), towns: Vec::new(), count: 0 };
    let mut rng = Rng(7);
    let st = PlacementClass::Static;
    while g.towns.len() < 30 {
        let c = DVec3::new(rng.range(-7000.0, 7000.0), 0.0, rng.range(-7000.0, 7000.0));
        if g.towns.iter().any(|t| t.distance(c) < 1400.0) {
            continue;
        }
        g.towns.push(c);
        let yaw0 = rng.range(0.0, std::f64::consts::TAU) as f32;
        g.add("tower.mdl", c, 0.0, rng.range(0.8, 1.3) as f32, st);
        for i in -4..=4 {
            for j in -4..=4 {
                if (i == 0 && j == 0) || rng.next() < 0.25 {
                    continue;
                }
                let p = c + DVec3::new(f64::from(i) * 26.0 + rng.range(-5.0, 5.0), 0.0, f64::from(j) * 24.0 + rng.range(-5.0, 5.0));
                g.add("house.mdl", p, yaw0 + rng.range(-0.2, 0.2) as f32, rng.range(0.9, 1.15) as f32, st);
                for _ in 0..rng.range(2.0, 8.0) as usize {
                    let q = p + DVec3::new(rng.range(-9.0, 9.0), 0.5, rng.range(-8.0, 8.0) + 6.5);
                    g.add("crate.mdl", q, rng.range(0.0, std::f64::consts::TAU) as f32, rng.range(0.6, 1.2) as f32, st);
                }
                if (i + j) % 2 == 0 {
                    g.add("lamp.mdl", p + DVec3::new(7.0, 0.0, 6.0), 0.0, 1.0, PlacementClass::Light);
                }
                for _ in 0..3 {
                    let q = p + DVec3::new(rng.range(-13.0, 13.0), 0.0, rng.range(-12.0, 12.0));
                    if q.distance(p) > 7.0 {
                        let m = if rng.next() < 0.5 { "tree.mdl" } else { "oak.mdl" };
                        g.add(m, q, rng.range(0.0, std::f64::consts::TAU) as f32, rng.range(0.7, 1.3) as f32, st);
                    }
                }
            }
        }
    }
    let forest = |p: DVec3| ((p.x * 0.0011).sin() * (p.z * 0.0013).cos() + (p.x * 0.0003 + p.z * 0.0004).sin()) > 0.55;
    while g.count < 80_000 {
        let p = DVec3::new(rng.range(-HALF, HALF), 0.0, rng.range(-HALF, HALF));
        if g.towns.iter().any(|t| t.distance(p) < 140.0) {
            continue;
        }
        if forest(p) || rng.next() < 0.12 {
            let m = if forest(p) && rng.next() < 0.7 { "tree.mdl" } else { "oak.mdl" };
            g.add(m, p, rng.range(0.0, std::f64::consts::TAU) as f32, rng.range(0.7, 1.5) as f32, st);
        } else {
            let s = (0.3 / (1.0 - rng.next() * 0.97)).min(9.0) as f32;
            let model = ["rock0.mdl", "rock1.mdl", "rock2.mdl"][(rng.next() * 3.0) as usize % 3];
            g.add(model, p - DVec3::Y * f64::from(s) * 0.15, rng.range(0.0, std::f64::consts::TAU) as f32, s, st);
        }
    }
    g
}

struct Source(BTreeMap<RegionId, Vec<Placement>>);

impl WorldSource for Source {
    fn regions(&self, ws: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        if *ws == exterior() {
            for r in self.0.keys() {
                out(*r, Fingerprint::of_parts(&[GENERATOR.as_bytes(), &r.0.to_le_bytes()]));
            }
        }
    }
    fn placements(&self, ws: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        if *ws == exterior() {
            self.0.get(&region).into_iter().flatten().cloned().for_each(out);
        }
    }
}

/// The adapter hook: trees are impostors, lamps glow warm at their head.
struct ExampleFar;

impl FarSource for ExampleFar {
    fn fingerprint(&self) -> [u8; 32] {
        Digest::of(GENERATOR.as_bytes()).0
    }
    fn policy(&self, model: &str, _: &ModelSummary) -> Option<FarPolicy> {
        matches!(model, "tree.mdl" | "oak.mdl").then_some(FarPolicy::Impostor)
    }
    fn light(&self, _: &Placement) -> Option<FarLight> {
        Some(FarLight { color: [1.0, 0.7, 0.38], intensity: 1.0, radius: 5.0, offset: [0.0, 4.25, 0.0] })
    }
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

struct Baked {
    store: Store,
    manifest: Manifest,
    towns: Vec<DVec3>,
}

fn bake() -> Baked {
    let started = std::time::Instant::now();
    let generated = generate();
    let generated_in = started.elapsed();
    let files: HashMap<String, Vec<u8>> =
        models().into_iter().map(|(k, m)| (k.to_owned(), postcard::to_stdvec(&m).expect("models serialize"))).collect();
    let paths: Vec<String> = files.keys().cloned().collect();
    let dir = std::env::temp_dir().join("sf_lod_far_field_example");
    let store = Store::open(&dir).expect("bake store");
    let mut baker = Baker::new(store.clone(), Arc::new(Files(files)));
    let importers = Importers::new().with(Arc::new(PostcardImporter));
    baker.register(ModelInfoStep { importers: importers.clone() });
    baker.register(MeshCookStep { importers, settings: CookSettings::default() });
    let count = generated.count;
    WorldBake::new(Arc::new(Source(generated.regions)), IndexParams::default()).register(&mut baker);
    FarBake::new(Arc::new(ExampleFar), FarBakeSettings::default()).register(&mut baker);
    let keys: Vec<ProductKey> = paths.iter().map(|p| ProductKey::new(MeshCookStep::ID, p.as_str())).collect();
    let cooked = baker.bake(&keys);
    let world = WorldBake::bake_worldspaces(&baker, &[exterior()]);
    let far = FarBake::bake_worldspaces(&baker, &[exterior()]);
    for (key, err) in cooked.failures().chain(world.failures()).chain(far.failures()) {
        eprintln!("bake failed: {key}: {err}");
    }
    println!(
        "generated {count} placements in {:.0} ms; bake: models {:.0} ms, world {:.0} ms, far field {:.0} ms ({})",
        generated_in.as_secs_f64() * 1000.0,
        cooked.elapsed.as_secs_f64() * 1000.0,
        world.elapsed.as_secs_f64() * 1000.0,
        far.elapsed.as_secs_f64() * 1000.0,
        dir.display()
    );
    for (step, s) in &far.steps {
        println!("  {step:14} built {:6} hit {:6} run {:9.1} ms  {:9} bytes", s.built, s.hits, s.run_time.as_secs_f64() * 1000.0, s.bytes_written);
    }
    Baked { store, manifest: baker.manifest(), towns: generated.towns }
}

// -- app ----------------------------------------------------------------------------------

#[derive(Component)]
struct FlyCam;

#[derive(Resource)]
struct Towns(Vec<DVec3>);

fn main() {
    let args = parse_args();
    let baked = bake();
    if args.bake_only {
        return;
    }
    let mut extensions: BTreeMap<String, String> = BTreeMap::from([(MeshCookStep::ID.to_owned(), "sfmesh".to_owned())]);
    extensions.extend(baked_extensions());

    let mut app = App::new();
    app.register_asset_source(BAKED_SOURCE, baked_source(baked.store.clone(), BakedManifest::new(baked.manifest.clone()), extensions));
    app.add_plugins(DefaultPlugins.build().disable::<TransformPlugin>().set(WindowPlugin {
        primary_window: Some(Window {
            title: "sf_lod far field".into(),
            resolution: WindowResolution::new(1600, 900).with_scale_factor_override(1.0),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }));
    let far = FarFieldSettings::default();
    let config = if args.no_far { StreamingConfig::default() } else { far_streaming_config(StreamingConfig::default(), &far.levels) };
    app.add_plugins(WorldStreamingPlugin { config });
    if !args.no_far {
        app.add_plugins(FarFieldPlugin { settings: far });
        app.insert_resource(FarFieldDebug { level_tint: args.tint });
        app.insert_resource(FarLightsNight(if args.night { 1.0 } else { 0.0 }));
    }
    let mut records = BTreeMap::new();
    records.insert(exterior(), Worldspace::new("Exterior"));
    app.insert_resource(WorldIndexSource(Arc::new(BakedIndex::new(baked.store.clone(), Arc::new(baked.manifest.clone())))))
        .insert_resource(ActiveWorldspace(Some(exterior())))
        .insert_resource(WorldspaceRecords(records))
        .insert_resource(ClearColor(sky(args.night)))
        .insert_resource(GlobalAmbientLight {
            color: if args.night { Color::srgb(0.35, 0.42, 0.65) } else { Color::srgb(0.75, 0.82, 1.0) },
            brightness: if args.night { 40.0 } else { 600.0 },
            ..default()
        })
        .insert_resource(Towns(baked.towns))
        .insert_resource(args.clone());

    let mut settings = DiagSettings { output_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/diag"), exit_on_finish: true, apply_transform: false, ..default() };
    settings.meta.insert("example".into(), "sf_lod_bevy/far_field".into());
    settings.meta.insert("far".into(), (!args.no_far).to_string());
    let mut diag = SfDiagPlugin::new(settings);
    if let Some(name) = &args.scenario {
        let towns = &app.world().resource::<Towns>().0;
        diag = diag.with_scenario(scenario(name, towns)).expect("valid scenario");
    }
    app.add_plugins(diag)
        .add_systems(Startup, (setup, spawn_ground))
        .add_systems(Update, (fly_camera, follow_pose).chain().before(StreamingSystems::Drive))
        .add_systems(Update, (toggles, status, screenshot, hold_warmup));
    if args.scenario.is_some() {
        app.add_systems(Startup, |mut s: ResMut<Streaming>| s.begin_burst());
    }
    app.run();
}

fn sky(night: bool) -> Color {
    if night { Color::srgb(0.012, 0.016, 0.035) } else { Color::srgb(0.62, 0.74, 0.88) }
}

fn scenario(name: &str, towns: &[DVec3]) -> Scenario {
    assert_eq!(name, "fly", "scenarios: fly");
    // Town to town at 60 m/s, 60 m above the hills, about 8 km.
    let mut keys = vec![Keyframe::at(DVec3::new(0.0, height(0.0, 0.0) + 60.0, 0.0))];
    let mut at = DVec3::ZERO;
    let mut left: Vec<DVec3> = towns.to_vec();
    let mut length = 0.0;
    while length < 8000.0 && !left.is_empty() {
        let (i, next) = left.iter().copied().enumerate().min_by(|(_, a), (_, b)| a.distance(at).total_cmp(&b.distance(at))).expect("towns");
        left.remove(i);
        length += next.distance(at);
        at = next;
        keys.push(Keyframe::at(next + DVec3::Y * (height(next.x, next.z) + 60.0)));
    }
    Scenario {
        name: format!("far_{name}"),
        warmup: Warmup { seconds: 3.0, frames: 120 },
        measure: Measure::Path { keyframes: keys, speed: 60.0 },
        gates: Gate::wishlist_streaming(),
        spike_factor: 1.5,
        tolerance: default(),
    }
}

fn setup(mut commands: Commands, mut registry: ResMut<WorldspaceRegistry>, args: Res<Args>, towns: Res<Towns>) {
    let entry = registry.ensure(&mut commands, &exterior());
    let town = towns.0[0];
    let start = args.at.unwrap_or(town + DVec3::new(600.0, height(town.x + 600.0, town.z + 600.0) + 120.0, 600.0));
    let (cell, mut transform) = to_grid(WorldPos::from_f64(start));
    let target = args.look_at.unwrap_or(town + DVec3::Y * height(town.x, town.z));
    transform.rotation = Transform::default().looking_to((target - start).as_vec3(), Vec3::Y).rotation;
    let fov = args.fov.unwrap_or(70.0).to_radians();
    let camera = commands
        .spawn((
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection { fov, near: 0.1, ..default() }),
            cell,
            transform,
            FloatingOrigin,
            FlyCam,
            StreamingAnchor::visual(),
            RenderLayers::from_layers(&[0, entry.layer]),
            DistanceFog {
                color: if args.night { Color::srgb(0.02, 0.025, 0.05) } else { Color::srgb(0.66, 0.76, 0.88) },
                falloff: FogFalloff::from_visibility_colors(if args.night { 9_000.0 } else { 22_000.0 }, Color::srgb(0.66, 0.76, 0.88), Color::srgb(0.85, 0.88, 0.95)),
                ..default()
            },
            ChildOf(entry.root),
        ))
        .id();
    commands.spawn((cell, transform, StreamingAnchor::gameplay(), ChildOf(entry.root)));
    let _ = camera;
    commands.spawn((
        DirectionalLight { illuminance: if args.night { 30.0 } else { 11_000.0 }, shadow_maps_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.7, if args.night { -0.5 } else { -0.75 }, 0.0)),
        CascadeShadowConfigBuilder { maximum_distance: 600.0, first_cascade_far_bound: 40.0, ..default() }.build(),
        RenderLayers::from_layers(&[0, entry.layer]),
    ));
}

/// The ground: 2 km tiles of 32 m quads following `height`, vertex-colored,
/// standing in for terrain (not streamed: a few large static entities).
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn spawn_ground(
    mut commands: Commands,
    mut registry: ResMut<WorldspaceRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    args: Res<Args>,
) {
    let entry = registry.ensure(&mut commands, &exterior());
    let material = materials.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.95, ..default() });
    let (tile, n) = (2000.0f64, 63usize);
    let step = tile / n as f64;
    let tiles = (2.0 * HALF / tile).ceil() as i64;
    for ti in 0..=tiles {
        for tj in 0..=tiles {
            let x0 = -HALF - tile * 0.5 + ti as f64 * tile;
            let z0 = -HALF - tile * 0.5 + tj as f64 * tile;
            let center = DVec3::new(x0 + tile * 0.5, 0.0, z0 + tile * 0.5);
            let (mut positions, mut normals, mut colors) = (Vec::new(), Vec::new(), Vec::new());
            for j in 0..=n {
                for i in 0..=n {
                    let (x, z) = (x0 + i as f64 * step, z0 + j as f64 * step);
                    let h = height(x, z);
                    let dx = height(x + 1.0, z) - height(x - 1.0, z);
                    let dz = height(x, z + 1.0) - height(x, z - 1.0);
                    positions.push([(x - center.x) as f32, h as f32, (z - center.z) as f32]);
                    normals.push(Vec3::new(-dx as f32, 2.0, -dz as f32).normalize().to_array());
                    let v = (0.5 + 0.5 * ((x * 0.013).sin() * (z * 0.011).cos())) as f32;
                    // Neutral gray in tint mode, so level colors read clearly.
                    let c = if args.tint { Color::srgb(0.45, 0.45, 0.45) } else { Color::srgb(0.30 + 0.08 * v, 0.40 + 0.06 * v, 0.22 + 0.03 * v) }.to_linear();
                    colors.push([c.red, c.green, c.blue, 1.0]);
                }
            }
            let mut indices = Vec::new();
            for j in 0..n {
                for i in 0..n {
                    let a = u16::try_from(j * (n + 1) + i).expect("small");
                    let b = a + u16::try_from(n + 1).expect("small");
                    indices.extend([a, b, a + 1, a + 1, b, b + 1]);
                }
            }
            let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
                .with_inserted_indices(Indices::U16(indices));
            let (cell, transform) = to_grid(WorldPos::from_f64(center));
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                cell,
                transform,
                RenderLayers::layer(entry.layer),
                ChildOf(entry.root),
            ));
        }
    }
}

fn fly_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    args: Res<Args>,
    mut cams: Query<&mut Transform, With<FlyCam>>,
) {
    if args.scenario.is_some() {
        return;
    }
    let Ok(mut t) = cams.single_mut() else { return };
    if mouse.pressed(MouseButton::Right) {
        let (yaw, pitch, _) = t.rotation.to_euler(EulerRot::YXZ);
        t.rotation = Quat::from_euler(EulerRot::YXZ, yaw - motion.delta.x * 0.003, (pitch - motion.delta.y * 0.003).clamp(-1.5, 1.5), 0.0);
    }
    let mut dir = Vec3::ZERO;
    for (key, d) in [(KeyCode::KeyW, Vec3::NEG_Z), (KeyCode::KeyS, Vec3::Z), (KeyCode::KeyA, Vec3::NEG_X), (KeyCode::KeyD, Vec3::X)] {
        if keys.pressed(key) {
            dir += t.rotation * d;
        }
    }
    if keys.pressed(KeyCode::KeyE) {
        dir += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyQ) {
        dir -= Vec3::Y;
    }
    let speed = if keys.pressed(KeyCode::ShiftLeft) { 200.0 } else { 20.0 };
    t.translation += dir.normalize_or_zero() * speed * time.delta_secs();
}

fn follow_pose(pose: Res<FlyThroughPose>, mut cams: Query<(&mut CellCoord, &mut Transform), With<FlyCam>>, mut anchors: Query<(&mut CellCoord, &mut Transform), (With<StreamingAnchor>, Without<FlyCam>)>) {
    let Ok((mut cell, mut transform)) = cams.single_mut() else { return };
    if let Some(pose) = pose.0 {
        let (c, mut t) = to_grid(WorldPos::from_f64(pose.position));
        t.rotation = pose.rotation.as_quat();
        *cell = c;
        *transform = t;
    }
    // The gameplay anchor walks with the camera.
    for (mut c, mut t) in &mut anchors {
        *c = *cell;
        *t = *transform;
    }
}

fn toggles(keys: Res<ButtonInput<KeyCode>>, debug: Option<ResMut<FarFieldDebug>>) {
    if keys.just_pressed(KeyCode::F2)
        && let Some(mut d) = debug
    {
        d.level_tint = !d.level_tint;
    }
}

fn hold_warmup(streaming: Res<Streaming>, scenario: Option<ResMut<DiagScenario>>) {
    if let Some(mut s) = scenario {
        s.hold = !streaming.is_idle();
    }
}

fn status(time: Res<Time<Real>>, streaming: Res<Streaming>, stats: Option<Res<FarFieldStats>>, mut windows: Query<&mut Window>, mut acc: Local<(f32, u32)>, entities: Query<Entity>) {
    acc.0 += time.delta_secs();
    acc.1 += 1;
    if acc.0 < 0.5 {
        return;
    }
    #[allow(clippy::cast_precision_loss)]
    let fps = acc.1 as f32 / acc.0;
    *acc = (0.0, 0);
    let Ok(mut window) = windows.single_mut() else { return };
    let s = streaming.stats();
    let far = stats.map(|s| *s).unwrap_or_default();
    window.title = format!(
        "far field | {fps:.0} fps | cells {} | placements {} | far: L1 {} L2 {} L3 {} entities {} tris {} imp {} lights {} forced {} sel {:.3} ms | world entities {}",
        s.resident_cells,
        s.placements_spawned,
        far.l1,
        far.l2,
        far.l3,
        far.entities,
        far.triangles,
        far.impostors,
        far.lights,
        far.forced,
        far.select_ms,
        entities.iter().count(),
    );
    println!("{}", window.title);
}

#[allow(clippy::too_many_arguments)]
fn screenshot(
    mut commands: Commands,
    args: Res<Args>,
    streaming: Res<Streaming>,
    stats: Option<Res<FarFieldStats>>,
    time: Res<Time<Real>>,
    mut settled: Local<Option<f32>>,
    mut taken: Local<Option<u32>>,
    shots: Query<(), With<Screenshot>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &args.screenshot else { return };
    if let Some(frames) = taken.as_mut() {
        *frames += 1;
        if *frames > 5 && shots.is_empty() {
            exit.write(AppExit::Success);
        }
        return;
    }
    // Settled: nothing streaming and no node forced by missing children
    // (give up waiting after a minute, so a stuck node can't hang a run).
    let far_busy = stats.is_some_and(|s| s.forced > 0);
    let waiting = (!streaming.is_idle() || far_busy) && time.elapsed_secs() < 60.0;
    if waiting || time.elapsed_secs() < 3.0 {
        *settled = None;
        return;
    }
    let since = settled.get_or_insert(time.elapsed_secs());
    if time.elapsed_secs() - *since > 3.0 {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.clone()));
        *taken = Some(0);
    }
}
