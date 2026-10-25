//! Streaming over a synthetic world: a 20 × 20 km exterior with ~100k
//! placements of a few procedural models (baked on the fly into a cached
//! store), a precision test cluster 500 km out, and an interior worldspace
//! behind a door.
//!
//! ```text
//! cargo run -p sf_world_bevy --example stream --features bevy/dynamic_linking -- [options]
//!   --at <x,y,z>          start position (meters; default near the origin)
//!   --look-at <x,y,z>     start looking at a point
//!   --yaw <deg> --pitch <deg>
//!   --screenshot <png>    wait for streaming to settle, save, exit
//!   --rings               ring overlay (F1 toggles)
//!   --scenario run|fly    sf_diag fly-through (7 / 60 m/s), writes target/diag/<name>.json
//!   --anchor <x,y,z>      streaming anchors stay here (the camera only looks)
//!   --town <i>            start above town i, looking at it
//!   --bake-only           bake and print timings, then exit
//! ```
//! Keys: WASD/QE move, right mouse look, Shift ×10, Ctrl ×0.1, F1 rings.
//! Walking into the door of the house east of the start crosses into the
//! interior (and back) through the fade scaffold.

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

use bevy::camera::visibility::RenderLayers;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::light::{CascadeShadowConfigBuilder, GlobalAmbientLight};
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
use sf_model::steps::{Importers, MeshCookStep, ModelInfoStep};
use sf_model::{CookSettings, ImportError, LodPolicy, Material as ModelMaterial, MeshData, Model, ModelImporter, Node};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_vfs::VfsPath;
use sf_world::bake::{BakedIndex, WorldBake};
use sf_world::{
    DoorLink, Fingerprint, IndexParams, IndexedPlacement, LinkEnd, Partition, Placement, PlacementClass, PlacementFlags,
    PlacementId, RecordRef, RegionId, RingOverrides, RingRadii, WorldSource, Worldspace, WorldspaceId,
};
use sf_world_bevy::big_space::prelude::{CellCoord, FloatingOrigin};
use sf_world_bevy::{
    ActiveWorldspace, DoorLinks, FadeTransition, PlacementSpawner, PlacementSpawners, SpawnCx, StartTravel, StaticSpawner,
    Streaming, StreamingAnchor, StreamingConfig, StreamingDebug, StreamingSystems, WorldIndexSource, WorldPosOf,
    WorldStreamingPlugin, WorldspaceRecords, WorldspaceRegistry, to_grid,
};

/// Bumped when the generator changes: region fingerprints include it.
const GENERATOR: &str = "stream-example/4";
const HALF_EXTENT: f64 = 10_000.0;
const PRECISION_SITE: DVec3 = DVec3::new(500_000.0, 0.0, 500_000.0);
const DOOR_HOUSE: DVec3 = DVec3::new(60.0, 0.0, -20.0);

fn exterior() -> WorldspaceId {
    WorldspaceId::new("example/exterior")
}

fn interior() -> WorldspaceId {
    WorldspaceId::new("example/interior")
}

// -- arguments -------------------------------------------------------------------

#[derive(Resource, Clone, Debug, Default)]
struct Args {
    at: Option<DVec3>,
    look_at: Option<DVec3>,
    yaw: f32,
    pitch: f32,
    screenshot: Option<PathBuf>,
    rings: bool,
    scenario: Option<String>,
    bake_only: bool,
    town: Option<usize>,
    anchor: Option<DVec3>,
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
            "--yaw" => args.yaw = value().parse().expect("--yaw <degrees>"),
            "--pitch" => args.pitch = value().parse().expect("--pitch <degrees>"),
            "--screenshot" => args.screenshot = Some(PathBuf::from(value())),
            "--rings" => args.rings = true,
            "--scenario" => args.scenario = Some(value()),
            "--bake-only" => args.bake_only = true,
            "--anchor" => args.anchor = Some(parse_vec(&value())),
            "--town" => args.town = Some(value().parse().expect("--town <index>")),
            other => panic!("unknown argument {other}"),
        }
    }
    args
}

// -- procedural models -------------------------------------------------------------

/// Deterministic pseudo-random numbers (64-bit LCG).
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

/// Geometry with flat normals and vertex colors.
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

    /// An axis-aligned box (outward faces, or inward for rooms).
    fn cuboid(&mut self, min: Vec3, max: Vec3, color: [f32; 4], inward: bool) {
        let v = |x: bool, y: bool, z: bool| {
            Vec3::new(if x { max.x } else { min.x }, if y { max.y } else { min.y }, if z { max.z } else { min.z })
        };
        let faces = [
            [v(true, false, false), v(true, true, false), v(true, true, true), v(true, false, true)],
            [v(false, false, true), v(false, true, true), v(false, true, false), v(false, false, false)],
            [v(false, true, false), v(false, true, true), v(true, true, true), v(true, true, false)],
            [v(false, false, true), v(false, false, false), v(true, false, false), v(true, false, true)],
            [v(false, false, true), v(true, false, true), v(true, true, true), v(false, true, true)],
            [v(true, false, false), v(false, false, false), v(false, true, false), v(true, true, false)],
        ];
        for [a, b, c, d] in faces {
            if inward {
                self.quad(a, d, c, b, color);
            } else {
                self.quad(a, b, c, d, color);
            }
        }
    }

    /// A cone or frustum around +Y.
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

    /// A noisy subdivided octahedron (rocks).
    fn rock(&mut self, radius: f32, seed: u64, color: [f32; 4]) {
        let mut rng = Rng(seed);
        let bumps: Vec<(Vec3, f32)> = (0..6)
            .map(|_| {
                #[allow(clippy::cast_possible_truncation)]
                let d = Vec3::new(rng.range(-1.0, 1.0) as f32, rng.range(-0.3, 1.0) as f32, rng.range(-1.0, 1.0) as f32);
                #[allow(clippy::cast_possible_truncation)]
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

    fn model(self, double_sided: bool) -> Model {
        self.model_with(double_sided, LodPolicy::Simplify)
    }

    fn model_with(self, double_sided: bool, lod: LodPolicy) -> Model {
        let mut node = Node::new("root");
        node.mesh = Some(0);
        node.lod = lod;
        let material = ModelMaterial { double_sided, ..ModelMaterial::default() };
        Model { nodes: vec![node], meshes: vec![self.0], materials: vec![material] }
    }
}

/// Every model of the example, by VFS path.
fn models() -> BTreeMap<&'static str, Model> {
    let mut out = BTreeMap::new();
    // Ground tiles: 256 m, 8 m checkers, four tints.
    let tints = [(0.36, 0.45, 0.24), (0.42, 0.47, 0.27), (0.47, 0.44, 0.30), (0.33, 0.42, 0.26)];
    for (k, (r, g, b)) in tints.into_iter().enumerate() {
        let mut m = Builder::default();
        let n = 32;
        for i in 0..n {
            for j in 0..n {
                #[allow(clippy::cast_precision_loss)]
                let (x, z) = (i as f32 * 8.0 - 128.0, j as f32 * 8.0 - 128.0);
                let shade = if (i + j) % 2 == 0 { 1.0 } else { 0.88 };
                let c = srgb(r * shade, g * shade, b * shade);
                m.quad(Vec3::new(x, 0.0, z), Vec3::new(x, 0.0, z + 8.0), Vec3::new(x + 8.0, 0.0, z + 8.0), Vec3::new(x + 8.0, 0.0, z), c);
            }
        }
        // Flat with color seams: geometric error says nothing about the
        // checker, so keep it out of simplification.
        out.insert(["ground0.mdl", "ground1.mdl", "ground2.mdl", "ground3.mdl"][k], m.model_with(false, LodPolicy::Keep));
    }
    for (k, seed) in [11u64, 23, 37].into_iter().enumerate() {
        let mut m = Builder::default();
        m.rock(1.0, seed, srgb(0.48 + 0.04 * k as f32, 0.45, 0.41));
        out.insert(["rock0.mdl", "rock1.mdl", "rock2.mdl"][k], m.model(false));
    }
    let mut tree = Builder::default();
    tree.cone(Vec3::ZERO, 0.3, 0.2, 3.0, 8, srgb(0.36, 0.25, 0.16));
    for (y, r, h) in [(2.0, 2.4, 3.2), (3.8, 1.9, 2.8), (5.4, 1.3, 2.4)] {
        tree.cone(Vec3::Y * y, r, 0.0, h, 12, srgb(0.16, 0.36, 0.17));
    }
    out.insert("tree.mdl", tree.model(false));
    let mut house = Builder::default();
    house.cuboid(Vec3::new(-5.0, 0.0, -4.0), Vec3::new(5.0, 4.5, 4.0), srgb(0.82, 0.76, 0.62), false);
    house.cone(Vec3::Y * 4.5, 7.4, 0.0, 3.5, 4, srgb(0.55, 0.24, 0.18));
    house.cuboid(Vec3::new(5.0, 0.0, -0.8), Vec3::new(5.08, 2.2, 0.8), srgb(0.30, 0.20, 0.12), false);
    out.insert("house.mdl", house.model(false));
    let mut tower = Builder::default();
    tower.cone(Vec3::ZERO, 4.0, 3.2, 36.0, 16, srgb(0.62, 0.60, 0.56));
    tower.cone(Vec3::Y * 36.0, 4.4, 0.0, 6.0, 16, srgb(0.35, 0.33, 0.38));
    out.insert("tower.mdl", tower.model(false));
    let mut crate_ = Builder::default();
    crate_.cuboid(Vec3::splat(-0.5), Vec3::splat(0.5), srgb(0.58, 0.42, 0.25), false);
    out.insert("crate.mdl", crate_.model(false));
    // 1 mm fins on a pedestal: the precision probe.
    let mut comb = Builder::default();
    comb.cuboid(Vec3::new(-0.25, 0.0, -0.25), Vec3::new(0.25, 1.0, 0.25), srgb(0.3, 0.3, 0.32), false);
    for i in 0..12 {
        #[allow(clippy::cast_precision_loss)]
        let x = -0.024 + i as f32 * 0.004;
        let c = if i % 2 == 0 { srgb(0.95, 0.95, 0.95) } else { srgb(0.85, 0.1, 0.1) };
        comb.cuboid(Vec3::new(x, 1.0, -0.025), Vec3::new(x + 0.001, 1.04, 0.025), c, false);
    }
    out.insert("comb.mdl", comb.model(false));
    let mut room = Builder::default();
    room.cuboid(Vec3::new(-6.0, 0.0, -5.0), Vec3::new(6.0, 4.0, 5.0), srgb(0.62, 0.55, 0.45), true);
    room.cuboid(Vec3::new(-0.8, 0.0, 4.95), Vec3::new(0.8, 2.2, 5.0), srgb(0.30, 0.20, 0.12), false);
    out.insert("room.mdl", room.model(true));
    let mut table = Builder::default();
    table.cuboid(Vec3::new(-1.0, 0.75, -0.6), Vec3::new(1.0, 0.82, 0.6), srgb(0.45, 0.3, 0.18), false);
    for (x, z) in [(-0.9, -0.5), (0.9, -0.5), (-0.9, 0.5), (0.9, 0.5)] {
        table.cuboid(Vec3::new(x - 0.05, 0.0, z - 0.05), Vec3::new(x + 0.05, 0.75, z + 0.05), srgb(0.4, 0.27, 0.16), false);
    }
    out.insert("table.mdl", table.model(false));
    let mut lamp = Builder::default();
    lamp.cone(Vec3::ZERO, 0.15, 0.1, 0.3, 8, srgb(1.0, 0.85, 0.5));
    out.insert("lamp.mdl", lamp.model(false));
    out
}

// -- the world ---------------------------------------------------------------------

struct Gen {
    regions: BTreeMap<WorldspaceId, BTreeMap<RegionId, Vec<Placement>>>,
    towns: Vec<DVec3>,
    count: usize,
}

impl Gen {
    fn add(&mut self, ws: &WorldspaceId, model: &str, pos: DVec3, yaw: f32, scale: f32, class: PlacementClass) {
        let key = format!("{ws}/{}", self.count);
        self.count += 1;
        // 1 km source regions, as an adapter's cells would be.
        #[allow(clippy::cast_possible_truncation)]
        let region = RegionId::from_key(&format!("{}/{}", (pos.x / 1000.0).floor() as i64, (pos.z / 1000.0).floor() as i64));
        let placement = Placement {
            id: PlacementId::from_key("stream-example", key.as_bytes()),
            base: RecordRef::new("example.object", model),
            model: Some(VfsPath::new(model).expect("valid path")),
            transform: WorldTransform {
                pos: WorldPos::from_f64(pos),
                rotation: Quat::from_rotation_y(yaw),
                scale: Vec3::splat(scale),
            },
            class,
            flags: PlacementFlags::empty(),
        };
        self.regions.entry(ws.clone()).or_default().entry(region).or_default().push(placement);
    }
}

#[allow(clippy::cast_possible_truncation, clippy::too_many_lines)]
fn generate() -> Gen {
    let mut g = Gen { regions: BTreeMap::new(), towns: Vec::new(), count: 0 };
    let ext = exterior();
    let mut rng = Rng(7);
    let st = PlacementClass::Static;

    // Ground: one tile per 256 m.
    let tiles = (HALF_EXTENT / 256.0) as i64;
    for i in -tiles..tiles {
        for j in -tiles..tiles {
            let p = DVec3::new(i as f64 * 256.0, 0.0, j as f64 * 256.0);
            let tint = (i * 7 + j * 13).rem_euclid(4);
            g.add(&ext, &format!("ground{tint}.mdl"), p, 0.0, 1.0, st);
        }
    }
    // The start clearing: the precision probe and the door house.
    g.add(&ext, "comb.mdl", DVec3::ZERO, 0.0, 1.0, st);
    g.add(&ext, "house.mdl", DOOR_HOUSE, 0.0, 1.0, st);
    g.add(&ext, "tower.mdl", DVec3::new(-90.0, 0.0, -140.0), 0.0, 1.0, st);
    let clear = |p: DVec3| p.x.hypot(p.z) > 150.0;

    // Towns: houses on a jittered grid, crates, trees, a tower.
    while g.towns.len() < 40 {
        let c = DVec3::new(rng.range(-9000.0, 9000.0), 0.0, rng.range(-9000.0, 9000.0));
        if c.length() < 600.0 || g.towns.iter().any(|t| t.distance(c) < 1200.0) {
            continue;
        }
        g.towns.push(c);
        let yaw0 = rng.range(0.0, std::f64::consts::TAU) as f32;
        g.add(&ext, "tower.mdl", c, 0.0, rng.range(0.8, 1.3) as f32, st);
        for i in -4..=4 {
            for j in -4..=4 {
                if (i == 0 && j == 0) || rng.next() < 0.25 {
                    continue;
                }
                let p = c + DVec3::new(f64::from(i) * 26.0 + rng.range(-5.0, 5.0), 0.0, f64::from(j) * 24.0 + rng.range(-5.0, 5.0));
                g.add(&ext, "house.mdl", p, yaw0 + (rng.range(-0.2, 0.2) as f32), rng.range(0.9, 1.15) as f32, st);
                for _ in 0..rng.range(3.0, 12.0) as usize {
                    let q = p + DVec3::new(rng.range(-9.0, 9.0), 0.0, rng.range(-8.0, 8.0) + 6.5);
                    g.add(&ext, "crate.mdl", q + DVec3::Y * 0.5, rng.range(0.0, std::f64::consts::TAU) as f32, rng.range(0.6, 1.2) as f32, st);
                }
                for _ in 0..4 {
                    let q = p + DVec3::new(rng.range(-13.0, 13.0), 0.0, rng.range(-12.0, 12.0));
                    if q.distance(p) > 7.0 {
                        g.add(&ext, "tree.mdl", q, rng.range(0.0, std::f64::consts::TAU) as f32, rng.range(0.7, 1.3) as f32, st);
                    }
                }
            }
        }
    }

    // Wilderness: forest patches and rocks with a power-law size spread.
    let forest = |p: DVec3| ((p.x * 0.0011).sin() * (p.z * 0.0013).cos() + (p.x * 0.0003 + p.z * 0.0004).sin()) > 0.6;
    while g.count < 100_000 {
        let p = DVec3::new(rng.range(-HALF_EXTENT, HALF_EXTENT), 0.0, rng.range(-HALF_EXTENT, HALF_EXTENT));
        if !clear(p) || g.towns.iter().any(|t| t.distance(p) < 130.0) {
            continue;
        }
        if forest(p) || rng.next() < 0.15 {
            g.add(&ext, "tree.mdl", p, rng.range(0.0, std::f64::consts::TAU) as f32, rng.range(0.6, 1.5) as f32, st);
        } else {
            let s = (0.3 / (1.0 - rng.next() * 0.97)).min(9.0) as f32;
            let model = ["rock0.mdl", "rock1.mdl", "rock2.mdl"][(rng.next() * 3.0) as usize % 3];
            g.add(&ext, model, p - DVec3::Y * f64::from(s) * 0.15, rng.range(0.0, std::f64::consts::TAU) as f32, s, st);
        }
    }

    // 500 km out: ground, the probe, a house (precision test).
    for i in -1..=1 {
        for j in -1..=1 {
            g.add(&ext, "ground0.mdl", PRECISION_SITE + DVec3::new(f64::from(i) * 256.0, 0.0, f64::from(j) * 256.0), 0.0, 1.0, st);
        }
    }
    g.add(&ext, "comb.mdl", PRECISION_SITE, 0.0, 1.0, st);
    g.add(&ext, "house.mdl", PRECISION_SITE + DVec3::new(20.0, 0.0, -12.0), 0.3, 1.0, st);

    // The interior: a room, tables, crates, lamps.
    let int = interior();
    g.add(&int, "room.mdl", DVec3::ZERO, 0.0, 1.0, st);
    g.add(&int, "table.mdl", DVec3::new(-2.5, 0.0, -1.5), 0.2, 1.0, st);
    g.add(&int, "table.mdl", DVec3::new(2.5, 0.0, -2.0), -0.1, 1.0, st);
    for k in 0..6 {
        let p = DVec3::new(-5.0 + f64::from(k) * 0.9, 0.5, -4.2);
        g.add(&int, "crate.mdl", p, f32::from(u8::try_from(k).expect("small")) * 0.4, 0.8, st);
    }
    g.add(&int, "lamp.mdl", DVec3::new(-2.5, 0.82, -1.5), 0.0, 1.0, PlacementClass::Light);
    g.add(&int, "lamp.mdl", DVec3::new(3.0, 3.4, 2.0), 0.0, 1.0, PlacementClass::Light);
    g
}

/// `WorldSource` over the generated regions.
struct Source(BTreeMap<WorldspaceId, BTreeMap<RegionId, Vec<Placement>>>);

impl WorldSource for Source {
    fn regions(&self, ws: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        for r in self.0.get(ws).into_iter().flat_map(BTreeMap::keys) {
            out(*r, Fingerprint::of_parts(&[GENERATOR.as_bytes(), &r.0.to_le_bytes()]));
        }
    }
    fn placements(&self, ws: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        self.0.get(ws).and_then(|r| r.get(&region)).into_iter().flatten().cloned().for_each(out);
    }
}

/// Model files: postcard-encoded `Model`s.
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

struct World {
    store: Store,
    manifest: Manifest,
    towns: Vec<DVec3>,
}

fn bake() -> World {
    let started = std::time::Instant::now();
    let generated = generate();
    let generated_in = started.elapsed();
    let files: HashMap<String, Vec<u8>> =
        models().into_iter().map(|(k, m)| (k.to_owned(), postcard::to_stdvec(&m).expect("models serialize"))).collect();
    let model_paths: Vec<String> = files.keys().cloned().collect();
    let dir = std::env::temp_dir().join("sf_world_stream_example");
    let store = Store::open(&dir).expect("bake store");
    let mut baker = Baker::new(store.clone(), Arc::new(Files(files)));
    let importers = Importers::new().with(Arc::new(PostcardImporter));
    baker.register(ModelInfoStep { importers: importers.clone() });
    baker.register(MeshCookStep { importers, settings: CookSettings::default() });
    let count = generated.count;
    WorldBake::new(Arc::new(Source(generated.regions)), IndexParams::default()).register(&mut baker);
    let keys: Vec<ProductKey> = model_paths.iter().map(|p| ProductKey::new(MeshCookStep::ID, p.as_str())).collect();
    let cooked = baker.bake(&keys);
    let report = WorldBake::bake_worldspaces(&baker, &[exterior(), interior()]);
    for (key, err) in cooked.failures().chain(report.failures()) {
        eprintln!("bake failed: {key}: {err}");
    }
    println!("generated {count} placements in {:.0} ms; bake {:.0} ms ({})", generated_in.as_secs_f64() * 1000.0, (cooked.elapsed + report.elapsed).as_secs_f64() * 1000.0, dir.display());
    for (step, s) in &report.steps {
        println!("  {step:14} built {:5} hit {:5} run {:7.1} ms", s.built, s.hits, s.run_time.as_secs_f64() * 1000.0);
    }
    World { store, manifest: baker.manifest(), towns: generated.towns }
}

fn door_link() -> DoorLink {
    let outside = DOOR_HOUSE + DVec3::new(5.1, 0.0, 0.0);
    let end = |ws: WorldspaceId, frame: DVec3, arrival: DVec3, yaw: f32| LinkEnd {
        worldspace: ws,
        frame: WorldTransform { pos: WorldPos::from_f64(frame), rotation: Quat::from_rotation_y(yaw), scale: Vec3::ONE },
        arrival: WorldTransform { pos: WorldPos::from_f64(arrival), rotation: Quat::from_rotation_y(yaw), scale: Vec3::ONE },
        door: None,
        extent: Vec2::ZERO,
    };
    DoorLink {
        // Outside: facing the door (looking -X); inside: facing into the room (-Z).
        a: end(exterior(), outside, outside + DVec3::new(4.0, 1.7, 0.0), PI / 2.0),
        b: end(interior(), DVec3::new(0.0, 0.0, 4.9), DVec3::new(0.0, 1.7, 3.0), 0.0),
        two_way: true,
    }
}

// -- app ---------------------------------------------------------------------------

#[derive(Component)]
struct FlyCam;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct Paths {
    towns: Vec<DVec3>,
}

/// Interior lamps: the model plus a point light.
struct LampSpawner;

impl PlacementSpawner for LampSpawner {
    fn spawn(&self, world: &mut bevy::ecs::world::World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let n = StaticSpawner.spawn(world, cx, placement);
        world.spawn((
            PointLight { intensity: 120_000.0, range: 14.0, shadow_maps_enabled: false, ..default() },
            Transform::from_translation(cx.local.translation + Vec3::Y * 0.2),
            cx.layers.clone(),
            ChildOf(cx.parent),
        ));
        n + 1
    }
}

fn main() {
    let args = parse_args();
    let world = bake();
    if args.bake_only {
        return;
    }
    let link = door_link();
    let mut records = BTreeMap::new();
    let mut room = Worldspace::new("Interior");
    room.partition = Partition::Single;
    room.rings = Some(RingOverrides { active: Some(RingRadii::new(64.0, 96.0)), near: Some(RingRadii::new(128.0, 160.0)), ..default() });
    records.insert(interior(), room);
    records.insert(exterior(), Worldspace::new("Exterior"));

    let mut app = App::new();
    app.register_asset_source(
        BAKED_SOURCE,
        baked_source(world.store.clone(), BakedManifest::new(world.manifest.clone()), BTreeMap::from([(MeshCookStep::ID.to_owned(), "sfmesh".to_owned())])),
    );
    let benchmark = args.scenario.is_some();
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<TransformPlugin>()
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "sf_world stream".into(),
                    resolution: WindowResolution::new(1280, 720).with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
    );
    app.add_plugins(WorldStreamingPlugin { config: StreamingConfig::default() })
        .insert_resource(WorldIndexSource(Arc::new(BakedIndex::new(world.store.clone(), Arc::new(world.manifest.clone())))))
        .insert_resource(ActiveWorldspace(Some(exterior())))
        .insert_resource(DoorLinks(vec![link]))
        .insert_resource(WorldspaceRecords(records))
        .insert_resource(StreamingDebug { enabled: args.rings, ..default() })
        .insert_resource(ClearColor(sky()))
        .insert_resource(GlobalAmbientLight { color: Color::srgb(0.75, 0.82, 1.0), brightness: 600.0, ..default() })
        .insert_resource(Paths { towns: world.towns })
        .insert_resource(args.clone());

    let mut settings = DiagSettings {
        output_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/diag"),
        exit_on_finish: true,
        apply_transform: false,
        ..default()
    };
    settings.meta.insert("example".into(), "sf_world_bevy/stream".into());
    settings.meta.insert("profile".into(), if cfg!(debug_assertions) { "dev" } else { "release" }.into());
    let mut diag = SfDiagPlugin::new(settings);
    if let Some(name) = &args.scenario {
        let paths = app.world().resource::<Paths>();
        diag = diag.with_scenario(scenario(name, &paths.towns)).expect("valid scenario");
    }
    app.add_plugins(diag);

    let mut spawners = PlacementSpawners::default();
    spawners.set(PlacementClass::Light, Arc::new(LampSpawner));
    app.insert_resource(spawners)
        .add_systems(Startup, setup)
        .add_systems(Update, (fly_camera, follow_pose, follow_player, door_travel).chain().before(StreamingSystems::Drive))
        .add_systems(Update, (toggles, status, screenshot, hold_warmup));
    if benchmark {
        app.add_systems(Startup, |mut s: ResMut<Streaming>| s.begin_burst());
    }
    app.run();
}

fn sky() -> Color {
    Color::srgb(0.62, 0.74, 0.88)
}

fn scenario(name: &str, towns: &[DVec3]) -> Scenario {
    let (keyframes, speed) = if name == "run" {
        // From the clearing to the nearest town at eye height.
        let target = towns.iter().copied().min_by(|a, b| a.length().total_cmp(&b.length())).expect("towns");
        let dir = target.normalize();
        let side = DVec3::new(-dir.z, 0.0, dir.x);
        let keys: Vec<Keyframe> = [0.0, 120.0, 260.0, 420.0]
            .into_iter()
            .enumerate()
            .map(|(i, d)| Keyframe::at(dir * d + side * if i % 2 == 0 { 0.0 } else { 15.0 } + DVec3::Y * 1.7))
            .collect();
        (keys, 7.0)
    } else {
        // Town to town at 45 m, about 6 km.
        let mut keys = vec![Keyframe::at(DVec3::new(0.0, 45.0, 0.0))];
        let mut at = DVec3::ZERO;
        let mut left: Vec<DVec3> = towns.to_vec();
        let mut length = 0.0;
        while length < 6000.0 && !left.is_empty() {
            let (i, next) = left.iter().copied().enumerate().min_by(|(_, a), (_, b)| a.distance(at).total_cmp(&b.distance(at))).expect("towns");
            left.remove(i);
            length += next.distance(at);
            at = next;
            keys.push(Keyframe::at(next + DVec3::Y * 45.0));
        }
        (keys, 60.0)
    };
    Scenario {
        name: format!("stream_{name}"),
        warmup: Warmup { seconds: 3.0, frames: 120 },
        measure: Measure::Path { keyframes, speed },
        gates: Gate::wishlist_streaming(),
        spike_factor: 1.5,
        tolerance: default(),
    }
}

fn setup(mut commands: Commands, mut registry: ResMut<WorldspaceRegistry>, args: Res<Args>, paths: Res<Paths>) {
    let entry = registry.ensure(&mut commands, &exterior());
    let town = args.town.map(|i| paths.towns[i % paths.towns.len()]);
    let start = args.at.or(town.map(|t| t + DVec3::new(80.0, 35.0, 80.0))).unwrap_or(DVec3::new(-6.0, 1.7, 14.0));
    let (cell, mut transform) = to_grid(WorldPos::from_f64(start));
    transform.rotation = match args.look_at.or(town) {
        Some(target) => Transform::default().looking_to((target - start).as_vec3(), Vec3::Y).rotation,
        None => Quat::from_euler(EulerRot::YXZ, args.yaw.to_radians(), args.pitch.to_radians(), 0.0),
    };
    let camera = commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection { fov: 70f32.to_radians(), near: 0.02, ..default() }),
        cell,
        transform,
        FloatingOrigin,
        FlyCam,
        RenderLayers::from_layers(&[0, entry.layer]),
        ChildOf(entry.root),
    )).id();
    if !args.rings {
        commands.entity(camera).insert(DistanceFog { color: sky(), falloff: FogFalloff::Linear { start: 1400.0, end: 3200.0 }, ..default() });
    }
    match args.anchor {
        None => {
            commands.entity(camera).insert(StreamingAnchor::visual());
            commands.spawn((cell, transform, StreamingAnchor::gameplay(), Player, ChildOf(entry.root)));
        }
        Some(at) => {
            // Fixed anchors: look at the rings from outside.
            let (cell, transform) = to_grid(WorldPos::from_f64(at));
            commands.spawn((cell, transform, StreamingAnchor::visual(), ChildOf(entry.root)));
            commands.spawn((cell, transform, StreamingAnchor::gameplay(), ChildOf(entry.root)));
        }
    }
    commands.spawn((
        DirectionalLight { illuminance: 11_000.0, shadow_maps_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.7, -0.75, 0.0)),
        CascadeShadowConfigBuilder { maximum_distance: 250.0, first_cascade_far_bound: 25.0, ..default() }.build(),
        RenderLayers::from_layers(&[0, entry.layer]),
    ));
}

/// Free-fly: WASD/QE, right mouse to look.
fn fly_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    fade: Res<FadeTransition>,
    args: Res<Args>,
    mut cams: Query<&mut Transform, With<FlyCam>>,
) {
    if args.scenario.is_some() || fade.is_active() {
        return;
    }
    let Ok(mut t) = cams.single_mut() else { return };
    if mouse.pressed(MouseButton::Right) {
        let (yaw, pitch, _) = t.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw - motion.delta.x * 0.003;
        let pitch = (pitch - motion.delta.y * 0.003).clamp(-1.5, 1.5);
        t.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
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
    let mut speed = 8.0;
    if keys.pressed(KeyCode::ShiftLeft) {
        speed *= 10.0;
    }
    if keys.pressed(KeyCode::ControlLeft) {
        speed *= 0.1;
    }
    // Relative moves: big_space recenters the cell when needed.
    t.translation += dir.normalize_or_zero() * speed * time.delta_secs();
}

/// Scenario mode: the fly-through pose (f64) as grid coordinates.
fn follow_pose(pose: Res<FlyThroughPose>, mut cams: Query<(&mut CellCoord, &mut Transform), With<FlyCam>>) {
    let Some(pose) = pose.0 else { return };
    let Ok((mut cell, mut transform)) = cams.single_mut() else { return };
    let (c, mut t) = to_grid(WorldPos::from_f64(pose.position));
    t.rotation = pose.rotation.as_quat();
    *cell = c;
    *transform = t;
}

/// The player (R0 anchor) walks with the camera.
fn follow_player(
    mut commands: Commands,
    cams: Query<(&CellCoord, &Transform, &ChildOf), (With<FlyCam>, Without<Player>)>,
    mut players: Query<(Entity, &mut CellCoord, &mut Transform, &ChildOf), With<Player>>,
) {
    let (Ok((c, t, parent)), Ok((player, mut pc, mut pt, pp))) = (cams.single(), players.single_mut()) else { return };
    *pc = *c;
    *pt = *t;
    if pp.parent() != parent.parent() {
        commands.entity(player).insert(ChildOf(parent.parent()));
    }
}

/// Walking into a link end crosses it (once per approach).
fn door_travel(
    links: Res<DoorLinks>,
    fade: Res<FadeTransition>,
    positions: WorldPosOf,
    cams: Query<Entity, With<FlyCam>>,
    mut armed: Local<Option<bool>>,
    mut travel: MessageWriter<StartTravel>,
) {
    let Ok(cam) = cams.single() else { return };
    let Some(here) = positions.location(cam) else { return };
    if fade.is_active() {
        *armed = Some(false);
        return;
    }
    let mut near_any = false;
    for link in &links.0 {
        for (from, to) in link.exits() {
            if from.worldspace != here.worldspace {
                continue;
            }
            let d = here.pos.delta(from.frame.pos);
            if d.x.hypot(d.z) < 2.5 && d.y.abs() < 3.0 {
                near_any = true;
                if armed.unwrap_or(true) {
                    travel.write(StartTravel { traveler: cam, to: sf_world::Location::new(to.worldspace.clone(), to.arrival.pos), rotation: to.arrival.rotation, reload: false });
                    *armed = Some(false);
                }
            }
        }
    }
    if !near_any {
        *armed = Some(true);
    }
}

fn toggles(keys: Res<ButtonInput<KeyCode>>, mut debug: ResMut<StreamingDebug>) {
    if keys.just_pressed(KeyCode::F1) {
        debug.enabled = !debug.enabled;
    }
}

/// Holds the scenario's warm-up until the start is fully streamed.
fn hold_warmup(streaming: Res<Streaming>, scenario: Option<ResMut<DiagScenario>>) {
    if let Some(mut s) = scenario {
        s.hold = !streaming.is_idle();
    }
}

fn status(
    time: Res<Time<Real>>,
    streaming: Res<Streaming>,
    positions: WorldPosOf,
    cams: Query<Entity, With<FlyCam>>,
    mut windows: Query<&mut Window>,
    mut acc: Local<(f32, u32)>,
) {
    acc.0 += time.delta_secs();
    acc.1 += 1;
    if acc.0 < 0.25 {
        return;
    }
    let fps = acc.1 as f32 / acc.0;
    *acc = (0.0, 0);
    let (Ok(cam), Ok(mut window)) = (cams.single(), windows.single_mut()) else { return };
    let Some(loc) = positions.location(cam) else { return };
    let p = loc.pos.to_f64();
    let s = streaming.stats();
    window.title = format!(
        "sf_world stream | {fps:.0} fps | {} | ({:.1}, {:.1}, {:.1}) | cells {} | requests {} | placements {} | models {}",
        loc.worldspace,
        p.x,
        p.y,
        p.z,
        s.resident_cells,
        streaming.scheduler().requests().count(),
        s.placements_spawned,
        s.models_loaded,
    );
}

/// `--screenshot`: once streaming is idle (plus time for pipelines to
/// compile), save and exit.
fn screenshot(
    mut commands: Commands,
    args: Res<Args>,
    streaming: Res<Streaming>,
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
    if !streaming.is_idle() || time.elapsed_secs() < 2.0 {
        *settled = None;
        return;
    }
    let since = settled.get_or_insert(time.elapsed_secs());
    if time.elapsed_secs() - *since > 2.5 {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.clone()));
        *taken = Some(0);
    }
}
