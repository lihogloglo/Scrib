//! Far-field runtime: node residency, entities, the no-hole fallback, and
//! counters (far-field.md, "Runtime").
//!
//! - L1 nodes ride the streaming far ring through [`FarHook`] (an
//!   `sf_world_bevy::CellProxies`): loaded in the assets stage, spawned in
//!   the spawn stage under the cell's level group, revealed with it,
//!   despawned with it.
//! - L2/L3 nodes are resident for the whole worldspace (their bytes are in
//!   the far directory), each under its own `Stationary` grid entity.
//! - Every part carries a `VisibilityRange` from `sf_lod`'s lead bands;
//!   Bevy does the switching and dithering. [`residency_pass`] only forces
//!   a node fully visible while children it covers aren't resident.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, PoisonError};

use bevy_asset::{AssetServer, Assets, Handle, RecursiveDependencyLoadState, RenderAssetUsages};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{NoAutoAabb, RenderLayers, Visibility, VisibilityRange};
use bevy_camera::Camera;
use bevy_color::{Color, LinearRgba};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_light::NotShadowCaster;
use bevy_math::{IVec3, UVec4, Vec3, Vec4};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_material::OpaqueRendererMethod;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_platform::time::Instant;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension};
use bevy_transform::components::{GlobalTransform, Transform};
use big_space::prelude::Stationary;
use sf_coords::WorldPos;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;
use sf_lod::products::{DIR_STEP, IMPOSTOR_STEP, directory_subject, level_step, node_subject};
use sf_lod::{FadeRange, FarDirectory, FarLevels, NodeKey, OctaMode};
use sf_model_bevy::baked_path;
use sf_world::{Level, WorldspaceId};
use sf_world_bevy::{CellProxies, ProxySpawnCx, Streaming, WorldspaceRegistry, to_grid};

use crate::assets::{FarDirectoryAsset, FarNodeAsset, ImpostorAtlasAsset};
use crate::materials::{FarShadowExt, ImpostorMaterial, ImpostorParams, LightSpriteMaterial, ProxyMaterial, SpriteParams, impostor_mesh, light_mesh};
use crate::{FarFieldDebug, FarFieldSettings, FarFieldStats};

/// Level tint colors (debug): L1 green, L2 blue, L3 purple.
pub const LEVEL_TINT: [[f32; 3]; 3] = [[0.25, 0.9, 0.3], [0.25, 0.45, 1.0], [0.7, 0.3, 0.95]];

/// Materials every node shares.
#[derive(Resource, Clone)]
pub struct FarMaterials {
    /// Proxy material per level (L1..L3): white `StandardMaterial` (vertex
    /// colors carry albedo) plus the far shadow map.
    pub proxy: [Handle<ProxyMaterial>; 3],
    pub lights: Handle<LightSpriteMaterial>,
}

pub(crate) fn create_materials(
    mut commands: Commands,
    mut proxies: ResMut<Assets<ProxyMaterial>>,
    mut sprites: ResMut<Assets<LightSpriteMaterial>>,
    settings: Res<FarFieldSettings>,
) {
    let proxy = [1u8, 2, 3].map(|level| {
        proxies.add(ProxyMaterial {
            base: StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.88,
                reflectance: 0.3,
                depth_bias: settings.proxy_depth_bias(level),
                // `proxy.wesl` is a forward fragment.
                opaque_render_method: OpaqueRendererMethod::Forward,
                ..StandardMaterial::default()
            },
            extension: FarShadowExt::default(),
        })
    });
    let lights = sprites.add(LightSpriteMaterial {
        params: SpriteParams { p: Vec4::new(1.0, settings.light_min_pixels, 0.35, 1.0), tint: Vec4::ONE },
    });
    commands.insert_resource(FarMaterials { proxy, lights });
}

/// An impostor model set ready to draw: one material, a layer per model.
struct ImpostorSet {
    material: Handle<ImpostorMaterial>,
    layers: HashMap<String, u32>,
}

/// An L1 node riding the streaming far ring.
struct L1Slot {
    handle: Handle<FarNodeAsset>,
    parts: Vec<Entity>,
    lead: f32,
    forced: bool,
    stats: NodeStats,
}

/// Per-node numbers for counters, taken at spawn.
#[derive(Clone, Copy, Default)]
struct NodeStats {
    triangles: u64,
    instances: u64,
    lights: u64,
    bytes: u64,
}

impl NodeStats {
    fn of(h: &sf_lod::NodeHeader, bytes: u64) -> Self {
        Self { triangles: h.triangles.into(), instances: h.instances.into(), lights: h.lights.into(), bytes }
    }

    fn add_to(self, out: &mut FarFieldStats) {
        out.triangles += self.triangles;
        out.impostors += self.instances;
        out.lights += self.lights;
        out.bytes += self.bytes;
    }
}

/// An L2/L3 node, resident for the whole worldspace.
struct UpperNode {
    handle: Handle<FarNodeAsset>,
    entity: Entity,
    parts: Vec<Entity>,
    lead: f32,
    forced: bool,
    /// Non-empty child nodes (L1 cells for L2, L2 keys for L3).
    children: Vec<NodeKey>,
    stats: NodeStats,
}

struct FarWorld {
    dir_handle: Handle<FarDirectoryAsset>,
    dir: Option<Arc<FarDirectory>>,
    failed: bool,
    l1: HashMap<IVec3, L1Slot>,
    upper: BTreeMap<NodeKey, UpperNode>,
    atlases: Vec<(String, Handle<ImpostorAtlasAsset>)>,
    impostors: Option<ImpostorSet>,
    impostors_failed: bool,
}

/// State shared by the streaming hook and the systems.
#[derive(Default)]
pub(crate) struct FarState {
    worlds: HashMap<WorldspaceId, FarWorld>,
}

/// The shared state handle (a resource, and inside [`FarHook`]).
#[derive(Resource, Clone, Default)]
pub(crate) struct FarShared(pub(crate) Arc<Mutex<FarState>>);

impl FarShared {
    fn lock(&self) -> std::sync::MutexGuard<'_, FarState> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

fn ensure_world<'a>(state: &'a mut FarState, assets: &AssetServer, ws: &WorldspaceId) -> &'a mut FarWorld {
    state.worlds.entry(ws.clone()).or_insert_with(|| FarWorld {
        dir_handle: assets.load(baked_path(DIR_STEP, &directory_subject(ws), Some("sffar"))),
        dir: None,
        failed: false,
        l1: HashMap::new(),
        upper: BTreeMap::new(),
        atlases: Vec::new(),
        impostors: None,
        impostors_failed: false,
    })
}

fn node_path(ws: &WorldspaceId, key: NodeKey) -> String {
    baked_path(level_step(key.level), &node_subject(ws, key), Some("sfnode"))
}

fn visibility(r: &FadeRange) -> VisibilityRange {
    // Shared slots: Bevy keeps one GPU slot per distinct range (sf_world_bevy::shared_range).
    sf_world_bevy::shared_range(VisibilityRange { start_margin: r.start.clone(), end_margin: r.end.clone(), use_aabb: false })
}

/// Runtime levels: the baked handoffs (from the directory) on the
/// configured screen.
fn levels_for(settings: &FarFieldSettings, dir: Option<&FarDirectory>) -> FarLevels {
    let mut levels = settings.levels;
    if let Some(dir) = dir {
        levels.handoff = dir.handoff;
        levels.band = dir.band;
    }
    levels
}

// -- the streaming hook ------------------------------------------------------------

/// `CellProxies` for the streaming far ring: L1 nodes.
pub(crate) struct FarHook(pub(crate) FarShared);

impl CellProxies for FarHook {
    fn acquire(&self, world: &World, ws: &WorldspaceId, cell: IVec3) {
        if !world.get_resource::<crate::FarWorldspaces>().is_none_or(|f| f.contains(ws)) {
            return;
        }
        let assets = world.resource::<AssetServer>();
        let mut state = self.0.lock();
        let w = ensure_world(&mut state, assets, ws);
        let key = NodeKey::of_cell(cell);
        w.l1.entry(cell).or_insert_with(|| L1Slot { handle: assets.load(node_path(ws, key)), parts: Vec::new(), lead: 0.0, forced: false, stats: NodeStats::default() });
    }

    fn ready(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> bool {
        let assets = world.resource::<AssetServer>();
        let state = self.0.lock();
        let Some(w) = state.worlds.get(ws) else { return true };
        if w.dir.is_none() && !w.failed {
            return false;
        }
        let Some(slot) = w.l1.get(&cell) else { return true };
        match assets.get_recursive_dependency_load_state(&slot.handle) {
            Some(RecursiveDependencyLoadState::Loaded) => {
                let has_impostors = world.resource::<Assets<FarNodeAsset>>().get(&slot.handle).is_some_and(|n| !n.impostors.is_empty());
                !has_impostors || w.impostors.is_some() || w.impostors_failed
            }
            Some(RecursiveDependencyLoadState::Failed(_)) | None => true,
            _ => false,
        }
    }

    fn spawn(&self, world: &mut World, cx: &ProxySpawnCx<'_>) -> u32 {
        let Some(materials) = world.get_resource::<FarMaterials>().cloned() else { return 0 };
        let settings = world.resource::<FarFieldSettings>().clone();
        let mut state = self.0.lock();
        let Some(w) = state.worlds.get_mut(cx.worldspace) else { return 0 };
        let levels = levels_for(&settings, w.dir.as_deref());
        let Some(slot) = w.l1.get(&cx.cell) else { return 0 };
        let handle = slot.handle.clone();
        let Some(node) = world.resource::<Assets<FarNodeAsset>>().get(&handle).map(clone_node) else { return 0 };
        if node.header.is_empty() {
            return 0;
        }
        let local = Transform::from_translation(cx.cell_origin.delta_f32(node.header.center()));
        let range = visibility(&levels.node_range(1, node.header.lead, false));
        let impostors = w.impostors.as_ref().map(|s| (s.material.clone(), s.layers.clone()));
        let parts = spawn_parts(world, cx.parent, local, &node, 1, &range, cx.layers, &materials, impostors.as_ref(), &settings);
        let n = u32::try_from(parts.len()).unwrap_or(u32::MAX);
        if let Some(slot) = w.l1.get_mut(&cx.cell) {
            slot.parts = parts;
            slot.lead = node.header.lead;
            slot.forced = false;
            slot.stats = NodeStats::of(&node.header, node.bytes);
        }
        n
    }

    /// The L1 bake's keep rule (`FarLevels::min_radius` at the node's lead,
    /// on the bake's screen: baked handoffs, bias 1). Unknown until the node
    /// loaded (0: keep every eligible placement); an empty node keeps none.
    fn kept_radius(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> f32 {
        let settings = world.resource::<FarFieldSettings>();
        let state = self.0.lock();
        let Some(w) = state.worlds.get(ws) else { return 0.0 };
        let Some(slot) = w.l1.get(&cell) else { return 0.0 };
        let Some(node) = world.resource::<Assets<FarNodeAsset>>().get(&slot.handle) else { return 0.0 };
        if node.header.is_empty() {
            return f32::INFINITY;
        }
        let mut levels = levels_for(settings, w.dir.as_deref());
        levels.screen.bias = 1.0;
        levels.min_radius(1, node.header.lead)
    }

    fn release(&self, _world: &mut World, ws: &WorldspaceId, cell: IVec3) {
        let mut state = self.0.lock();
        if let Some(w) = state.worlds.get_mut(ws) {
            w.l1.remove(&cell);
        }
    }

    fn bytes(&self, ws: &WorldspaceId, cell: IVec3) -> u64 {
        let state = self.0.lock();
        state
            .worlds
            .get(ws)
            .and_then(|w| w.dir.as_ref())
            .and_then(|d| d.find(&NodeKey::of_cell(cell)))
            .map_or(0, |n| n.bytes * 2)
    }
}

/// What spawning needs from a node asset (a cheap copy, so the asset
/// storage isn't borrowed while spawning).
struct NodeCopy {
    header: sf_lod::NodeHeader,
    meshes: Vec<Handle<Mesh>>,
    impostors: Vec<sf_lod::ImpostorInstance>,
    lights: Vec<sf_lod::LightPoint>,
    bytes: u64,
}

fn clone_node(n: &FarNodeAsset) -> NodeCopy {
    NodeCopy { header: n.header.clone(), meshes: n.meshes.clone(), impostors: n.impostors.clone(), lights: n.lights.clone(), bytes: n.bytes }
}

/// Spawns a node's parts (proxy mesh, impostors, lights) under `parent`.
#[allow(clippy::too_many_arguments)]
fn spawn_parts(
    world: &mut World,
    parent: Entity,
    local: Transform,
    node: &NodeCopy,
    level: u8,
    range: &VisibilityRange,
    layers: &RenderLayers,
    materials: &FarMaterials,
    impostors: Option<&(Handle<ImpostorMaterial>, HashMap<String, u32>)>,
    settings: &FarFieldSettings,
) -> Vec<Entity> {
    let h = &node.header;
    let bounds = Aabb::from_min_max(Vec3::from_array(h.min), Vec3::from_array(h.max).max(Vec3::from_array(h.min)));
    let cast_shadows = level == 1 && settings.l1_shadows;
    let mut parts = Vec::new();
    let mat = materials.proxy[usize::from(level.clamp(1, 3) - 1)].clone();
    for mesh in &node.meshes {
        let mut e = world.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), local, range.clone(), layers.clone(), bounds, NoAutoAabb, ChildOf(parent)));
        if !cast_shadows {
            e.insert(NotShadowCaster);
        }
        parts.push(e.id());
    }
    if let Some((material, layer_of)) = impostors
        && !node.impostors.is_empty()
    {
        let lookup = |i: u16| {
            let m = h.impostor_models.get(usize::from(i))?;
            layer_of.get(&m.model).map(|l| (*l, m.radius))
        };
        if let Some((mesh, (min, max))) = impostor_mesh(&node.impostors, &lookup) {
            let mesh = world.resource_mut::<Assets<Mesh>>().add(mesh);
            let mut e = world.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
                local,
                range.clone(),
                layers.clone(),
                Aabb::from_min_max(min, max),
                NoAutoAabb,
                ChildOf(parent),
            ));
            if !cast_shadows {
                e.insert(NotShadowCaster);
            }
            parts.push(e.id());
        }
    }
    if let Some((mesh, (min, max))) = light_mesh(&node.lights) {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(mesh);
        parts.push(
            world
                .spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(materials.lights.clone()),
                    local,
                    range.clone(),
                    layers.clone(),
                    Aabb::from_min_max(min, max),
                    NoAutoAabb,
                    NotShadowCaster,
                    ChildOf(parent),
                ))
                .id(),
        );
    }
    parts
}

// -- systems ---------------------------------------------------------------------------

/// Loads the far directory of every streamed worldspace; once loaded,
/// starts its impostor atlases and its L2/L3 nodes.
pub(crate) fn load_directories(
    mut commands: Commands,
    shared: Res<FarShared>,
    assets: Res<AssetServer>,
    dirs: Res<Assets<FarDirectoryAsset>>,
    streaming: Res<Streaming>,
    registry: Res<WorldspaceRegistry>,
    far: Res<crate::FarWorldspaces>,
) {
    let mut state = shared.lock();
    for (ws, entry) in registry.iter() {
        if !streaming.has_directory(ws) || !far.contains(ws) {
            continue;
        }
        let w = ensure_world(&mut state, &assets, ws);
        if w.dir.is_some() || w.failed {
            continue;
        }
        if let Some(RecursiveDependencyLoadState::Failed(e)) = assets.get_recursive_dependency_load_state(&w.dir_handle) {
            log::warn!("sf_lod_bevy: no far field for {ws}: {e}");
            w.failed = true;
            continue;
        }
        let Some(dir) = dirs.get(&w.dir_handle) else { continue };
        let dir = Arc::new(dir.0.clone());
        let l2_l3: u64 = dir.level_bytes(2) + dir.level_bytes(3);
        log::info!(
            "sf_lod_bevy: far field of {ws}: {} L1, {} L2, {} L3 nodes; L2+L3 {:.1} MiB resident; {} impostor models",
            dir.level(1).count(),
            dir.level(2).count(),
            dir.level(3).count(),
            f64::from(u32::try_from(l2_l3).unwrap_or(u32::MAX)) / 1_048_576.0,
            dir.impostor_models.len(),
        );
        w.atlases = dir
            .impostor_models
            .iter()
            .map(|m| (m.model.clone(), assets.load(baked_path(IMPOSTOR_STEP, &m.model, Some("sfimp")))))
            .collect();
        if w.atlases.is_empty() {
            w.impostors_failed = true;
        }
        // L2 and L3: resident for the whole worldspace.
        for level in [2u8, 3] {
            for n in dir.level(level) {
                let key = n.header.key;
                let children: Vec<NodeKey> = dir
                    .level(level - 1)
                    .filter(|c| c.header.key.parent() == Some(key))
                    .map(|c| c.header.key)
                    .collect();
                let (cell, transform) = to_grid(n.header.center());
                let entity = commands.spawn((cell, transform, Stationary, Visibility::Hidden, ChildOf(entry.root))).id();
                w.upper.insert(
                    key,
                    UpperNode { handle: assets.load(node_path(ws, key)), entity, parts: Vec::new(), lead: n.header.lead, forced: false, children, stats: NodeStats::of(&n.header, n.bytes) },
                );
            }
        }
        w.dir = Some(dir);
    }
}

/// Packs a worldspace's impostor atlases into two texture arrays once all
/// are loaded.
pub(crate) fn build_impostor_arrays(
    shared: Res<FarShared>,
    assets: Res<AssetServer>,
    atlases: Res<Assets<ImpostorAtlasAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ImpostorMaterial>>,
    debug: Res<FarFieldDebug>,
) {
    let mut state = shared.lock();
    for (ws, w) in &mut state.worlds {
        if w.impostors.is_some() || w.impostors_failed || w.atlases.is_empty() {
            continue;
        }
        let mut ready = true;
        for (model, h) in &w.atlases {
            match assets.get_load_state(h) {
                Some(bevy_asset::LoadState::Loaded) => {}
                Some(bevy_asset::LoadState::Failed(e)) => {
                    log::warn!("sf_lod_bevy: impostor atlas {model} of {ws} failed: {e}");
                }
                _ => ready = false,
            }
        }
        if !ready {
            continue;
        }
        let loaded: Vec<(&str, &sf_lod::ImpostorAtlas)> =
            w.atlases.iter().filter_map(|(m, h)| atlases.get(h).map(|a| (m.as_str(), &a.0))).collect();
        let Some((_, first)) = loaded.first() else {
            w.impostors_failed = true;
            continue;
        };
        let s = first.settings;
        let same: Vec<(&str, &sf_lod::ImpostorAtlas)> = loaded.iter().copied().filter(|(_, a)| a.settings == s).collect();
        let px = s.atlas_px();
        let mips = first.mip_levels;
        let mut albedo = Vec::new();
        let mut normal_depth = Vec::new();
        let mut layers = HashMap::new();
        for (i, (model, atlas)) in same.iter().enumerate() {
            albedo.extend_from_slice(&atlas.albedo);
            normal_depth.extend_from_slice(&atlas.normal_depth);
            layers.insert((*model).to_owned(), u32::try_from(i).unwrap_or(u32::MAX));
        }
        let n = same.len();
        let albedo = images.add(atlas_array(px, mips, n, TextureFormat::Rgba8UnormSrgb, albedo));
        let normal_depth = images.add(atlas_array(px, mips, n, TextureFormat::Rgba8Unorm, normal_depth));
        let tint = if debug.level_tint { Vec4::from((Vec3::from_array(LEVEL_TINT[0]), 0.5)) } else { Vec4::new(1.0, 1.0, 1.0, 0.5) };
        let material = materials.add(ImpostorMaterial {
            params: ImpostorParams { grid: UVec4::new(s.frames, u32::from(s.mode == OctaMode::Full), s.frame_px, 0), tint, far: Vec4::ZERO },
            albedo,
            normal_depth,
            far_map: Handle::default(),
        });
        w.impostors = Some(ImpostorSet { material, layers });
    }
}

fn atlas_array(px: u32, mips: u32, layers: usize, format: TextureFormat, data: Vec<u8>) -> Image {
    let extent = Extent3d { width: px, height: px, depth_or_array_layers: u32::try_from(layers.max(1)).unwrap_or(1) };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    image.texture_descriptor.mip_level_count = mips;
    image.data = Some(data);
    image.texture_view_descriptor =
        Some(TextureViewDescriptor { dimension: Some(TextureViewDimension::D2Array), ..TextureViewDescriptor::default() });
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::ClampToEdge,
        address_mode_v: ImageAddressMode::ClampToEdge,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..ImageSamplerDescriptor::default()
    });
    image
}

/// Spawns the parts of loaded L2/L3 nodes and reveals them.
pub(crate) fn spawn_upper_nodes(world: &mut World) {
    let Some(shared) = world.get_resource::<FarShared>().cloned() else { return };
    let Some(materials) = world.get_resource::<FarMaterials>().cloned() else { return };
    let settings = world.resource::<FarFieldSettings>().clone();
    let mut state = shared.lock();
    for (ws, w) in &mut state.worlds {
        let Some(layer) = world.resource::<WorldspaceRegistry>().get(ws).map(|e| e.layer) else { continue };
        let layers = RenderLayers::layer(layer);
        let levels = levels_for(&settings, w.dir.as_deref());
        let impostors = w.impostors.as_ref().map(|s| (s.material.clone(), s.layers.clone()));
        let impostors_settled = w.impostors.is_some() || w.impostors_failed;
        for node in w.upper.values_mut() {
            if !node.parts.is_empty() {
                continue;
            }
            let load = world.resource::<AssetServer>().get_recursive_dependency_load_state(&node.handle);
            if !matches!(load, Some(RecursiveDependencyLoadState::Loaded)) {
                continue;
            }
            let Some(copy) = world.resource::<Assets<FarNodeAsset>>().get(&node.handle).map(clone_node) else { continue };
            if !copy.impostors.is_empty() && !impostors_settled {
                continue;
            }
            let range = visibility(&levels.node_range(copy.header.key.level, node.lead, node.forced));
            node.parts =
                spawn_parts(world, node.entity, Transform::IDENTITY, &copy, copy.header.key.level, &range, &layers, &materials, impostors.as_ref(), &settings);
            if node.parts.is_empty() {
                // Nothing drawable: mark with the node entity itself so it
                // isn't retried.
                node.parts.push(node.entity);
            }
            if let Ok(mut e) = world.get_entity_mut(node.entity) {
                e.insert(Visibility::Inherited);
            }
        }
    }
}

/// The no-hole fallback (3D Tiles / UE: keep the parent until its children
/// arrive): forces nodes fully visible while the camera is where their
/// children should draw and any child isn't resident. Touches only nodes
/// whose forced state changes.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub(crate) fn residency_pass(
    shared: Res<FarShared>,
    settings: Res<FarFieldSettings>,
    streaming: Res<Streaming>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<&GlobalTransform>,
    mut ranges: Query<&mut VisibilityRange>,
    mut stats: ResMut<FarFieldStats>,
    counters: Option<Res<DiagCounters>>,
) {
    let started = Instant::now();
    let Some(camera) = cameras.iter().filter(|(c, _)| c.is_active).min_by_key(|(c, _)| c.order).map(|(_, t)| t.translation()) else {
        return;
    };
    let mut far = shared.lock();
    let mut out = FarFieldStats::default();
    for (ws, w) in &mut far.worlds {
        let levels = levels_for(&settings, w.dir.as_deref());
        let scheduler = streaming.scheduler();
        let mut set = |parts: &[Entity], range: VisibilityRange| {
            for &p in parts {
                if let Ok(mut r) = ranges.get_mut(p) {
                    *r = range.clone();
                }
            }
        };
        let FarWorld { l1, upper, .. } = w;
        // L1: children are the cell's objects (resident at `Near`).
        for (cell, slot) in l1.iter_mut() {
            let Some(&first) = slot.parts.first() else { continue };
            out.l1 += 1;
            out.entities += slot.parts.len();
            slot.stats.add_to(&mut out);
            let Ok(t) = transforms.get(first) else { continue };
            let d = t.translation().distance(camera);
            // Residency only matters where children should draw.
            let needs = levels.needs_children(1, slot.lead, d);
            let forced = needs && levels.forced(1, slot.lead, d, scheduler.level(ws, *cell) >= Level::Near);
            out.forced += usize::from(forced);
            if forced != slot.forced {
                slot.forced = forced;
                set(&slot.parts, visibility(&levels.node_range(1, slot.lead, forced)));
            }
        }
        // L2 then L3: children are L1 cells / L2 nodes.
        let l1_resident = |cell: IVec3| l1.get(&cell).is_some_and(|s| !s.parts.is_empty()) && scheduler.level(ws, cell) >= Level::Far;
        let l2_spawned: Vec<NodeKey> = upper.iter().filter(|(k, n)| k.level == 2 && !n.parts.is_empty()).map(|(k, _)| *k).collect();
        for (key, node) in upper.iter_mut() {
            if node.parts.is_empty() {
                continue;
            }
            if key.level == 2 {
                out.l2 += 1;
            } else {
                out.l3 += 1;
            }
            out.entities += node.parts.len();
            node.stats.add_to(&mut out);
            let Ok(t) = transforms.get(node.entity) else { continue };
            let d = t.translation().distance(camera);
            let needs = levels.needs_children(key.level, node.lead, d);
            let resident = !needs
                || node.children.iter().all(|c| if c.level == 1 { l1_resident(c.index()) } else { l2_spawned.binary_search(c).is_ok() });
            let forced = levels.forced(key.level, node.lead, d, resident);
            out.forced += usize::from(forced);
            if forced != node.forced {
                node.forced = forced;
                set(&node.parts, visibility(&levels.node_range(key.level, node.lead, forced)));
            }
        }
    }
    out.select_ms = started.elapsed().as_secs_f64() * 1000.0;
    if let Some(c) = counters {
        #[allow(clippy::cast_precision_loss)]
        {
            c.set_named("far/nodes/l1", unit::COUNT, out.l1 as f64);
            c.set_named("far/nodes/l2", unit::COUNT, out.l2 as f64);
            c.set_named("far/nodes/l3", unit::COUNT, out.l3 as f64);
            c.set_named("far/entities", unit::COUNT, out.entities as f64);
            c.set_named("far/triangles", unit::COUNT, out.triangles as f64);
            c.set_named("far/impostors", unit::COUNT, out.impostors as f64);
            c.set_named("far/lights", unit::COUNT, out.lights as f64);
            c.set_named("far/forced", unit::COUNT, out.forced as f64);
            c.set_named("far/resident/bytes", unit::BYTES, out.bytes as f64);
        }
        c.add_duration_named("far/select/ms", started.elapsed());
    }
    *stats = out;
}

/// Debug level tint and the light sprites' night factor.
#[allow(clippy::too_many_arguments)]
pub(crate) fn update_materials(
    debug: Res<FarFieldDebug>,
    far: Option<Res<FarMaterials>>,
    shared: Res<FarShared>,
    mut proxies: ResMut<Assets<ProxyMaterial>>,
    mut impostors: ResMut<Assets<ImpostorMaterial>>,
    mut sprites: ResMut<Assets<LightSpriteMaterial>>,
    night: Res<crate::FarLightsNight>,
    mut applied: Local<Option<(bool, f32)>>,
) {
    let Some(far) = far else { return };
    let wanted = (debug.level_tint, (night.0 * 100.0).round() / 100.0);
    if *applied == Some(wanted) {
        return;
    }
    *applied = Some(wanted);
    for (i, h) in far.proxy.iter().enumerate() {
        if let Some(mut m) = proxies.get_mut(h) {
            m.base.base_color = if debug.level_tint { Color::from(LinearRgba::rgb(LEVEL_TINT[i][0], LEVEL_TINT[i][1], LEVEL_TINT[i][2])) } else { Color::WHITE };
        }
    }
    let state = shared.lock();
    for w in state.worlds.values() {
        if let Some(set) = &w.impostors
            && let Some(mut m) = impostors.get_mut(&set.material)
        {
            let rgb = if debug.level_tint { Vec3::from_array(LEVEL_TINT[0]) } else { Vec3::ONE };
            m.params.tint = Vec4::from((rgb, m.params.tint.w));
        }
    }
    if let Some(mut m) = sprites.get_mut(&far.lights) {
        m.params.p.x = night.0;
        m.params.tint = if debug.level_tint { Vec4::new(1.0, 0.3, 0.3, 1.0) } else { Vec4::ONE };
    }
}

/// World position of a node center, for tools and tests.
pub fn node_center(dir: &FarDirectory, key: &NodeKey) -> Option<WorldPos> {
    dir.find(key).map(|n| n.header.center())
}

/// Hands the far shadow map to the proxy and impostor materials whenever
/// it changes (new contents, origin move) or impostor sets appear.
pub(crate) fn update_far_shadow(
    input: Res<crate::FarShadowInput>,
    far: Option<Res<FarMaterials>>,
    shared: Res<FarShared>,
    mut proxies: ResMut<Assets<ProxyMaterial>>,
    mut impostors: ResMut<Assets<ImpostorMaterial>>,
    mut applied: Local<Option<(crate::FarShadowInput, usize)>>,
) {
    let Some(far) = far else { return };
    let state = shared.lock();
    let sets: Vec<Handle<ImpostorMaterial>> = state.worlds.values().filter_map(|w| w.impostors.as_ref().map(|s| s.material.clone())).collect();
    drop(state);
    if applied.as_ref().is_some_and(|(i, n)| i == &*input && *n == sets.len()) {
        return;
    }
    *applied = Some((input.clone(), sets.len()));
    let map = input.image.clone().unwrap_or_default();
    let params = if input.image.is_some() { input.params } else { Vec4::ZERO };
    for h in &far.proxy {
        if let Some(mut m) = proxies.get_mut(h) {
            m.extension = FarShadowExt { far: params, map: map.clone() };
        }
    }
    for h in &sets {
        if let Some(mut m) = impostors.get_mut(h) {
            m.params.far = params;
            m.far_map = map.clone();
        }
    }
}
