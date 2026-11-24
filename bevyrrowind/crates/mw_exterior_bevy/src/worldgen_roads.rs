//! Road ribbons in a generated world (slice GR-1b): when the guide folder has `roads.json`
//! (`sf_worldgen::roads`, format v1), every segment's draped ribbon is built off the main thread on the
//! terrain's own height function (the app passes `GuideTiles::height` at level 0) and spawned as one mesh
//! entity per ribbon chunk, at its world position under the floating origin (a high-precision child of the
//! worldspace root: `CellCoord` + `Transform`, like every static), with its own [`Aabb`] so Bevy culls it.
//!
//! The ribbon recipe is the industry road-spline one (UE Landscape Splines' spline meshes, Horizon-style decal
//! ribbons; `sf_worldgen::roads` module doc): a conforming strip `lift_m` above the ground, its edges faded
//! through the vertex colour alpha, drawn alpha-blended with a depth bias ([`ROAD_DEPTH_BIAS`]) so it never
//! z-fights the terrain, and casting no shadow. The material is the game's cooked one of a Morrowind road
//! `LTEX` texture the app picks ([`RoadTextures`]: a paved one for paved/flagstone/cobble surfaces, a dirt road
//! one for the rest), tiled with `u` across the width and `v` along in metres per repeat.
//!
//! Mods contribute road networks too (S1b; everything is a mod): every `*.json` the VFS serves in
//! [`WorldgenRoads::mod_roads_dir`] (the app passes `mw_worldgen::town::ROADS_DIR`, where `worldgen-town --streets`
//! writes a town's streets) is one more road network v1 file, drawn exactly like the guide's. Each spawned chunk
//! names its network in [`WorldgenRoad::source`] ([`GUIDE_SOURCE`], or the file's VFS path); a bad file is a warning
//! naming it, and the others still draw.
//!
//! Not yet: bridge and causeway decks (the ribbon's spans are skipped, a later slice), and the ribbon does not
//! follow a live `worldgen.filter` rebuild (it stays on the surface it was built on). `--off roads` (the app)
//! leaves the roads out.
//!
//! Engine frame: the ribbon is built in the guide frame (x east, y north, z up) and mapped to Bevy's y-up frame
//! as `(x, y, z) → (x, z, −y)`, a rotation that keeps the counter-clockwise winding seen from above.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use bevy::asset::RenderAssetUsages;
use bevy::camera::primitives::Aabb;
use bevy::camera::visibility::RenderLayers;
use bevy::light::NotShadowCaster;
use bevy::math::DVec3;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, futures::check_ready};
use mw_assets::SharedVfs;
use sf_vfs::{DirEntry, Vfs};
use mw_assets::cooked::{TextureCompanions, cooked_material_with};
use sf_coords::WorldPos;
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{Def, Source, Unit, def};
use sf_world::WorldspaceId;
use sf_world_bevy::{WorldspaceRegistry, to_grid};
use sf_worldgen::roads::{RibbonChunk, RibbonParams, RoadNetwork, Segment, ribbon};

#[cfg(test)]
#[path = "worldgen_roads_tests.rs"]
mod tests;

/// `worldgen.road_depth_bias`: the road material's constant depth bias (`StandardMaterial::depth_bias`, wgpu
/// `DepthBiasState::constant`), on top of the ribbon's `lift_m`, so the ribbon wins over the terrain it lies on.
pub const ROAD_DEPTH_BIAS: Def<f32> = def!(float "worldgen.road_depth_bias" = 16.0)
    .range(0.0, 2000.0)
    .unit(Unit::None)
    .help("Generated world: depth bias of the road ribbons over the terrain (higher wins at a distance)")
    .source(Source::Guess);

/// `worldgen.road_opacity`: the road material's opacity inside the edge fade (the edges fade to 0 from it).
pub const ROAD_OPACITY: Def<f32> = def!(float "worldgen.road_opacity" = 1.0)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Generated world: opacity of the road ribbons inside their faded edges")
    .source(Source::Guess);

/// Engine-frame height of the ground at `(x, z)`, metres (the terrain's own surface).
pub type GroundHeight = Arc<dyn Fn(f64, f64) -> f64 + Send + Sync>;

/// VFS paths of the road textures (`None`: an untextured road colour).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoadTextures {
    /// For paved, flagstone, cobble and stone surfaces.
    pub paved: Option<String>,
    /// For every other surface (dirt, gravel, ...).
    pub unpaved: Option<String>,
}

impl RoadTextures {
    /// The texture for a segment's `surface` (the other one when it has none).
    pub fn for_surface(&self, surface: &str) -> Option<&str> {
        let (first, second) = if is_paved(surface) { (&self.paved, &self.unpaved) } else { (&self.unpaved, &self.paved) };
        first.as_deref().or(second.as_deref())
    }
}

/// Whether a `roads.json` surface is a paved one.
pub fn is_paved(surface: &str) -> bool {
    let s = surface.to_ascii_lowercase();
    ["paved", "flagstone", "cobble", "stone", "brick"].iter().any(|k| s.contains(k))
}

/// What the generated world's roads need: the guide folder (its `roads.json`), the VFS folder of the mods' road
/// networks, the ground and the textures.
#[derive(Clone)]
pub struct WorldgenRoads {
    pub guide_dir: PathBuf,
    /// VFS folder whose `*.json` files are the loaded mods' road networks (`None`: the guide's only).
    pub mod_roads_dir: Option<String>,
    pub height: GroundHeight,
    pub textures: RoadTextures,
}

/// [`WorldgenRoad::source`] of the guide's own `roads.json`.
pub const GUIDE_SOURCE: &str = "guide";

/// One spawned ribbon chunk.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct WorldgenRoad {
    /// The network it comes from: [`GUIDE_SOURCE`], or the mod file's VFS path.
    pub source: Arc<str>,
    /// The segment id in that network.
    pub segment: u32,
}

/// The road networks to draw, in order: the guide's `roads.json` (when present), then every `*.json` of
/// `roads.mod_roads_dir` in `vfs` (VFS path order). Unreadable or invalid files are skipped with a warning.
pub fn networks(roads: &WorldgenRoads, vfs: Option<&Vfs>) -> Vec<(Arc<str>, RoadNetwork)> {
    let mut out = Vec::new();
    match RoadNetwork::read(&roads.guide_dir) {
        Ok(Some(n)) => out.push((Arc::from(GUIDE_SOURCE), n)),
        Ok(None) => info!("worldgen roads: {} has no roads.json", roads.guide_dir.display()),
        Err(e) => warn!("worldgen roads: {}: {e}; skipped", roads.guide_dir.join("roads.json").display()),
    }
    let (Some(dir), Some(vfs)) = (roads.mod_roads_dir.as_deref(), vfs) else { return out };
    for entry in vfs.read_dir(dir) {
        let DirEntry::File(path) = entry else { continue };
        if !path.extension().is_some_and(|e| e.eq_ignore_ascii_case("json")) {
            continue;
        }
        let parsed = vfs
            .read(&path)
            .map_err(|e| e.to_string())
            .and_then(|b| RoadNetwork::from_reader(b.as_slice()).map_err(|e| e.to_string()));
        match parsed {
            Ok(n) => out.push((Arc::from(path.as_str()), n)),
            Err(e) => warn!("worldgen roads: {path}: {e}; skipped"),
        }
    }
    out
}

/// The live look of the road materials ([`ROAD_DEPTH_BIAS`], [`ROAD_OPACITY`]).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct RoadLook {
    depth_bias: f32,
    opacity: f32,
}

/// The road materials, for the live look.
#[derive(Resource, Default)]
struct RoadMaterials(Vec<Handle<StandardMaterial>>);

/// A chunk built off the main thread.
struct BuiltChunk {
    /// Engine frame, metres (absolute).
    origin: DVec3,
    mesh: Mesh,
    aabb: Aabb,
}

/// A segment built off the main thread.
struct BuiltSegment {
    /// Index of its network in [`RoadBuild::sources`].
    source: usize,
    id: u32,
    texture: Option<String>,
    length_m: f64,
    chunks: Vec<BuiltChunk>,
}

/// The road build: the source until it starts, then the tasks and their results.
#[derive(Resource)]
struct RoadBuild {
    worldspace: WorldspaceId,
    source: Option<WorldgenRoads>,
    started: Option<Instant>,
    /// The networks being built, in [`networks`] order, with their segment counts.
    sources: Vec<(Arc<str>, usize)>,
    segments: usize,
    tasks: Vec<Task<BuiltSegment>>,
    done: Vec<BuiltSegment>,
    built_ms: Option<f64>,
}

/// Registers the roads of a generated world (`roads: None`: only the tunables, so `--set` knows them).
pub(crate) fn add(app: &mut App, worldspace: &WorldspaceId, roads: Option<&WorldgenRoads>) {
    app.insert_resource(RoadLook { depth_bias: ROAD_DEPTH_BIAS.default_value(), opacity: ROAD_OPACITY.default_value() });
    let _bias = app.add_tunable(&ROAD_DEPTH_BIAS).bind_resource(|l: &mut RoadLook, v| l.depth_bias = v);
    let _opacity = app.add_tunable(&ROAD_OPACITY).bind_resource(|l: &mut RoadLook, v| l.opacity = v);
    let Some(roads) = roads else { return };
    app.init_resource::<RoadMaterials>()
        .insert_resource(RoadBuild {
            worldspace: worldspace.clone(),
            source: Some(roads.clone()),
            started: None,
            sources: Vec::new(),
            segments: 0,
            tasks: Vec::new(),
            done: Vec::new(),
            built_ms: None,
        })
        .add_systems(Startup, start_build)
        .add_systems(Update, (finish_build.run_if(resource_exists::<RoadBuild>), apply_look).chain());
}

/// Reads the guide's and the mods' road networks ([`networks`]) and starts one ribbon task per segment.
fn start_build(mut commands: Commands, mut build: ResMut<RoadBuild>, vfs: Option<Res<SharedVfs>>) {
    let Some(source) = build.source.take() else { return };
    let networks = networks(&source, vfs.as_ref().map(|v| &*v.0));
    if networks.is_empty() {
        info!(
            "worldgen roads: none (no roads.json in {}, no mod network in {})",
            source.guide_dir.display(),
            source.mod_roads_dir.as_deref().unwrap_or("-")
        );
        commands.remove_resource::<RoadBuild>();
        return;
    }
    let params = Arc::new(RibbonParams::default());
    let pool = AsyncComputeTaskPool::get();
    build.started = Some(Instant::now());
    for (index, (name, network)) in networks.into_iter().enumerate() {
        build.segments += network.segments.len();
        build.sources.push((name, network.segments.len()));
        for seg in network.segments {
            let (params, height) = (params.clone(), source.height.clone());
            let texture = source.textures.for_surface(&seg.surface).map(str::to_owned);
            build.tasks.push(pool.spawn(async move { build_segment(index, &seg, &params, &*height, texture) }));
        }
    }
}

/// A segment's ribbon as engine-frame meshes.
fn build_segment(
    source: usize,
    seg: &Segment,
    params: &RibbonParams,
    height: &(dyn Fn(f64, f64) -> f64 + Send + Sync),
    texture: Option<String>,
) -> BuiltSegment {
    // The ribbon asks the guide frame (x east, y north): engine (x, z) = (x, −y).
    let r = ribbon(seg, params, &|x, y| height(x, -y));
    BuiltSegment { source, id: seg.id, texture, length_m: r.length_m, chunks: r.chunks.iter().map(chunk_mesh).collect() }
}

/// Guide frame (z up) to engine frame (y up).
fn to_engine(p: [f32; 3]) -> [f32; 3] {
    [p[0], p[2], -p[1]]
}

/// One chunk's render mesh: engine-frame positions and normals, the edge fade in the vertex colour alpha, and
/// tangents along `u` (across) for the normal maps.
fn chunk_mesh(c: &RibbonChunk) -> BuiltChunk {
    let origin = DVec3::new(c.origin[0], c.origin[2], -c.origin[1]);
    let positions: Vec<[f32; 3]> = c.positions.iter().map(|&p| to_engine(p)).collect();
    let normals: Vec<[f32; 3]> = c.normals.iter().map(|&n| to_engine(n)).collect();
    let colors: Vec<[f32; 4]> = c.fade.iter().map(|&f| [1.0, 1.0, 1.0, f]).collect();
    let cols = c.cols.max(1);
    let tangents: Vec<[f32; 4]> = (0..positions.len())
        .map(|i| {
            let row = i / cols * cols;
            let across = Vec3::from(positions[row + cols - 1]) - Vec3::from(positions[row]);
            let n = Vec3::from(normals[i]);
            let t = (across - n * n.dot(across)).normalize_or(Vec3::X);
            [t.x, t.y, t.z, 1.0]
        })
        .collect();
    let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in &positions {
        lo = lo.min(Vec3::from(*p));
        hi = hi.max(Vec3::from(*p));
    }
    let aabb = if positions.is_empty() { Aabb::from_min_max(Vec3::ZERO, Vec3::ZERO) } else { Aabb::from_min_max(lo, hi) };
    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, c.uvs.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U32(c.indices.clone()));
    BuiltChunk { origin, mesh, aabb }
}

/// Collects finished ribbons; once every one is in and the worldspace root exists, spawns the chunks.
#[allow(clippy::too_many_arguments)] // a system: the spawn's resources
fn finish_build(
    mut commands: Commands,
    mut build: ResMut<RoadBuild>,
    registry: Res<WorldspaceRegistry>,
    look: Res<RoadLook>,
    vfs: Option<Res<SharedVfs>>,
    server: Option<Res<AssetServer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut owned: ResMut<RoadMaterials>,
) {
    let Some(started) = build.started else { return };
    let build = &mut *build;
    build.tasks.retain_mut(|task| match check_ready(task) {
        Some(seg) => {
            build.done.push(seg);
            false
        }
        None => true,
    });
    if !build.tasks.is_empty() {
        return;
    }
    let built_ms = *build.built_ms.get_or_insert_with(|| started.elapsed().as_secs_f64() * 1e3);
    let Some(entry) = registry.get(&build.worldspace) else { return };
    let layers = RenderLayers::layer(entry.layer);
    let companions = vfs.as_ref().map(|v| TextureCompanions::read(&v.0));
    let mut by_texture: Vec<(Option<String>, Handle<StandardMaterial>)> = Vec::new();
    let (mut chunks, mut vertices, mut length_m) = (0usize, 0usize, 0.0);
    let mut per_source = vec![0usize; build.sources.len()];
    let mut done = std::mem::take(&mut build.done);
    done.sort_by_key(|s| (s.source, s.id));
    for seg in done {
        length_m += seg.length_m;
        let source = build.sources[seg.source].0.clone();
        let material = if let Some((_, h)) = by_texture.iter().find(|(t, _)| *t == seg.texture) {
            h.clone()
        } else {
            let m = road_material(seg.texture.as_deref(), vfs.as_deref(), companions.as_ref(), server.as_deref(), *look);
            let h = materials.add(m);
            owned.0.push(h.clone());
            by_texture.push((seg.texture.clone(), h.clone()));
            h
        };
        for c in seg.chunks {
            vertices += c.mesh.count_vertices();
            let (cell, transform) = to_grid(WorldPos::from_f64(c.origin));
            commands.spawn((
                Name::new(format!("worldgen road {source} {} chunk", seg.id)),
                WorldgenRoad { source: source.clone(), segment: seg.id },
                Mesh3d(meshes.add(c.mesh)),
                MeshMaterial3d(material.clone()),
                c.aabb,
                NotShadowCaster,
                cell,
                transform,
                layers.clone(),
                ChildOf(entry.root),
            ));
            chunks += 1;
            per_source[seg.source] += 1;
        }
    }
    for ((name, segments), chunks) in build.sources.iter().zip(&per_source) {
        info!("worldgen roads: {name} {segments} segments, {chunks} chunks");
    }
    let textures: Vec<String> = by_texture.iter().map(|(t, _)| t.clone().unwrap_or_else(|| "untextured".into())).collect();
    info!(
        "worldgen roads: {} segments, {chunks} chunks, {:.1} km, built in {built_ms:.0} ms ({vertices} vertices; textures {})",
        build.segments,
        length_m / 1000.0,
        textures.join(", "),
    );
    commands.remove_resource::<RoadBuild>();
}

/// The road material: the cooked material of `texture` (or a plain road colour), alpha-blended through the
/// vertex fade, depth-biased over the terrain.
fn road_material(
    texture: Option<&str>,
    vfs: Option<&SharedVfs>,
    companions: Option<&TextureCompanions>,
    server: Option<&AssetServer>,
    look: RoadLook,
) -> StandardMaterial {
    let cooked = match (texture, vfs, companions, server) {
        (Some(t), Some(vfs), Some(companions), Some(server)) => {
            let source = sf_model::Material { base_color_texture: Some(t.to_owned()), ..sf_model::Material::default() };
            cooked_material_with(&source, &vfs.0, companions, server)
        }
        _ => StandardMaterial { base_color: Color::srgb(0.42, 0.36, 0.28), perceptual_roughness: 0.95, ..StandardMaterial::default() },
    };
    let mut m = StandardMaterial { alpha_mode: AlphaMode::Blend, double_sided: false, ..cooked };
    apply(&mut m, look);
    m
}

fn apply(m: &mut StandardMaterial, look: RoadLook) {
    m.depth_bias = look.depth_bias;
    m.base_color = m.base_color.with_alpha(look.opacity);
}

/// A live [`ROAD_DEPTH_BIAS`] / [`ROAD_OPACITY`] change rewrites the road materials.
fn apply_look(look: Res<RoadLook>, owned: Option<Res<RoadMaterials>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let Some(owned) = owned else { return };
    if !look.is_changed() {
        return;
    }
    for h in &owned.0 {
        if let Some(mut m) = materials.get_mut(h) {
            apply(&mut m, *look);
        }
    }
}
