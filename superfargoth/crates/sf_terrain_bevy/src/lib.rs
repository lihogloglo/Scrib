//! Bevy plugin for `sf_terrain`. Spec: `docs/specs/terrain.md`.
//!
//! - **Residency**: canonical tiles stream from the bake store
//!   ([`TerrainWorld`]) on the async compute pool, around the
//!   [`TerrainCamera`], and also feed the CPU [`TerrainHeightsMap`].
//! - **Clipmaps**: 10 × 512² `R32Float` heights (0.5 m … 256 m), 3 levels of
//!   splat layers (1–4 m) and 10 levels of macro albedo, updated
//!   toroidally in strips under a per-frame upload budget
//!   ([`TerrainSettings::upload_budget`]).
//! - **Geometry**: CDLOD ([`sf_terrain::cdlod`]) selects patches each
//!   frame; each patch is an entity sharing one grid mesh and one
//!   [`TerrainMaterial`], so Bevy batches them into indirect draws, culls
//!   them per view (main and shadow cascades) and draws them in the depth
//!   prepass. The vertex shader morphs and displaces.
//! - **Origin**: patch transforms are relative to [`TerrainOrigin`], the
//!   floating origin; hosts on `big_space` implement [`RenderOrigin`] for
//!   their origin resource and add [`TerrainOriginPlugin`].

mod far_map;
mod material;
mod params;
mod render;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{Assets, Handle, RenderAssetUsages, embedded_asset};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{NoAutoAabb, VisibilitySystems};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_math::{DVec2, Vec3};
use bevy_mesh::{Indices, Mesh, Mesh3d, MeshTag, PrimitiveTopology};
use bevy_pbr::{MaterialPlugin, MeshMaterial3d};
use bevy_render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use bevy_render::storage::ShaderBuffer;
use bevy_render::{ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shader::load_shader_library;
use bevy_tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use bevy_transform::TransformSystems;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_coords::WorldPos;
use sf_terrain::bake::BakedTerrain;
use sf_terrain::cdlod::{self, CdlodSettings, MIN_LOD, NodeKey, SelectedNode};
use sf_terrain::clipmap::{self, CLIPMAP_SIZE, ClipWindow, SampleRect};
use sf_terrain::{
    BLOCKS, BlockBounds, LayerId, LayerMap, LayerTable, ResidentTile, TerrainHeights, TileCoord, global_coords,
};
use sf_world::WorldspaceId;

pub use far_map::{FAR_MAP_SIZE, FAR_MAP_TEXEL, FarShadowMap};
pub use material::TerrainMaterial;
use params::Params;
pub use params::{HEIGHT_LEVELS, LAYER_LEVELS, MACRO_LEVELS};
pub use render::{ClipmapWrites, TextureWrite};
pub use sf_terrain;

/// Tunables. Defaults follow terrain.md and the streaming budgets.
#[derive(Resource, Clone, Debug)]
pub struct TerrainSettings {
    pub cdlod: CdlodSettings,
    /// GPU clipmap bytes per frame (terrain.md: ≤ 2 MiB/frame at 60 m/s).
    pub upload_budget: usize,
    /// Budget while nothing is resident yet (startup, teleport), as
    /// streaming's burst mode.
    pub burst_budget: usize,
    /// Tile loads in flight.
    pub max_loads: usize,
    /// Clipmap windows move in steps of this many samples.
    pub snap: i64,
    /// Distance where splatting hands over to the macro albedo, meters.
    pub splat_range: f32,
    /// Height-blend transition depth (Mishkinis), in height units.
    pub blend_depth: f32,
    /// Layer arrays to load (`None`: the largest baked).
    pub layer_array_size: Option<u32>,
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            cdlod: CdlodSettings::default(),
            upload_budget: 2 << 20,
            burst_budget: 64 << 20,
            max_loads: 16,
            snap: 16,
            splat_range: 1000.0,
            blend_depth: 0.2,
            layer_array_size: None,
        }
    }
}

/// Debug views (terrain.md "Verification").
#[derive(Resource, Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct TerrainDebug {
    /// Color patches by LOD (fractional: morphing).
    pub lod_colors: bool,
    /// Overlay the triangle edges.
    pub wireframe: bool,
    /// Stop updating the selection (inspect it from elsewhere).
    pub freeze: bool,
    /// Spawn patches without frustum culling (to tell culling bugs from
    /// selection bugs).
    pub no_culling: bool,
    /// Turn off the terrain shadow past the shadow cascades (A/B; see
    /// `shaders/far_shadow.wesl`).
    pub no_far_shadows: bool,
}

/// The view that drives selection, morphing and residency. One per app.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TerrainCamera;

/// The worldspace whose terrain is shown, read from the bake store.
#[derive(Resource, Clone)]
pub struct TerrainWorld {
    pub baked: Arc<BakedTerrain>,
}

/// The world position of render-space (0, 0, 0): the floating origin.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TerrainOrigin(pub WorldPos);

/// The floating-origin glue: implement for the resource that holds the
/// origin (e.g. `sf_world_bevy`'s `WorldOrigin`: `WorldPos::new(cell, 0)`)
/// and add [`TerrainOriginPlugin`]. Without it, [`TerrainOrigin`] is a
/// fixed local origin the app may set.
pub trait RenderOrigin: Resource {
    fn render_origin(&self) -> WorldPos;
}

/// Copies an origin resource into [`TerrainOrigin`] every frame.
pub struct TerrainOriginPlugin<O: RenderOrigin>(std::marker::PhantomData<O>);

impl<O: RenderOrigin> Default for TerrainOriginPlugin<O> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<O: RenderOrigin> Plugin for TerrainOriginPlugin<O> {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_origin::<O>.before(TerrainSystems::Update));
    }
}

fn sync_origin<O: RenderOrigin>(source: Option<Res<O>>, mut origin: ResMut<TerrainOrigin>) {
    if let Some(source) = source {
        let o = source.render_origin();
        if origin.0 != o {
            origin.0 = o;
        }
    }
}

/// CPU height queries per worldspace (base + edits), filled from the tiles
/// the renderer streams. Physics, AI and foliage read these.
#[derive(Resource, Default)]
pub struct TerrainHeightsMap(pub HashMap<WorldspaceId, TerrainHeights>);

/// Counters for `sf_diag` (the app forwards them).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainStats {
    pub patches: usize,
    pub resident_tiles: usize,
    pub loads_in_flight: usize,
    /// Clipmap bytes uploaded this frame.
    pub upload_bytes: usize,
    /// CDLOD selection + entity diff, milliseconds.
    pub select_ms: f32,
    /// Whole terrain update, milliseconds.
    pub update_ms: f32,
    /// Height levels resident on the GPU.
    pub height_levels: u32,
}

/// Fire after editing a worldspace's `HeightEdits`: GPU clipmap levels
/// covering the rect are rewritten.
#[derive(Message, Clone, Copy, Debug)]
pub struct TerrainEdited(pub sf_terrain::TerrainChanged);

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainSystems {
    Update,
}

/// Marks patch entities.
#[derive(Component, Clone, Copy, Debug)]
pub struct TerrainPatch(pub NodeKey);

#[derive(Default)]
pub struct TerrainPlugin {
    pub settings: TerrainSettings,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "shaders/common.wesl");
        load_shader_library!(app, "shaders/surface_state.wesl");
        load_shader_library!(app, "shaders/far_shadow.wesl");
        load_shader_library!(app, "shaders/surface.wesl");
        embedded_asset!(app, "shaders/terrain.wesl");
        embedded_asset!(app, "shaders/terrain_prepass.wesl");

        app.add_plugins(MaterialPlugin::<TerrainMaterial>::default())
            .insert_resource(self.settings.clone())
            .init_resource::<TerrainDebug>()
            .init_resource::<TerrainOrigin>()
            .init_resource::<TerrainHeightsMap>()
            .init_resource::<TerrainStats>()
            .init_resource::<ClipmapWrites>()
            .add_message::<TerrainEdited>()
            .configure_sets(
                PostUpdate,
                TerrainSystems::Update
                    .after(TransformSystems::Propagate)
                    .before(VisibilitySystems::VisibilityPropagate)
                    .before(VisibilitySystems::CalculateBounds),
            )
            .add_systems(bevy_app::Startup, far_map::setup_far_map)
            .add_systems(
                PostUpdate,
                // `update_far_map` runs before `update_terrain` so `FarShadowMap::params`
                // tracks any `TerrainOrigin` shift before `update_terrain` writes `P_FAR` (SH-11).
                (setup_terrain, far_map::update_far_map, update_terrain).chain().in_set(TerrainSystems::Update),
            );

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<render::PendingWrites>()
                .add_systems(ExtractSchedule, render::extract_writes)
                .add_systems(Render, render::write_clipmaps.in_set(RenderSystems::PrepareResources));
        }
    }
}

/// A resident tile.
struct Loaded {
    resident: Arc<ResidentTile>,
    layers: Option<LayerMap>,
    macro_rgba: Option<Vec<[u8; 4]>>,
}

enum Slot {
    Loading(Task<Option<Loaded>>),
    Ready(Arc<Loaded>),
    Absent,
}

/// GPU-side state of the active terrain.
#[derive(Resource)]
struct TerrainState {
    ws: WorldspaceId,
    baked: Arc<BakedTerrain>,
    material: Handle<TerrainMaterial>,
    params: Handle<ShaderBuffer>,
    heights: Handle<Image>,
    normals: Handle<Image>,
    layer_ids: Handle<Image>,
    layer_weights: Handle<Image>,
    macro_image: Handle<Image>,
    patch: Handle<Mesh>,
    quadrant: Handle<Mesh>,
    table: LayerTable,
    slots: HashMap<LayerId, u8>,
    tiles: HashMap<TileCoord, Slot>,
    height_windows: [Option<ClipWindow>; HEIGHT_LEVELS],
    layer_windows: [Option<ClipWindow>; LAYER_LEVELS],
    macro_windows: [Option<ClipWindow>; MACRO_LEVELS],
    /// Patch entities and the height bounds their `Aabb` was built from.
    nodes: HashMap<NodeKey, (Entity, (f32, f32))>,
    origin: Option<WorldPos>,
    /// Frames left in which clipmap uploads wait after an origin shift.
    shift_settle: u8,
    fill: f32,
    fill_slot: u8,
    fill_rgba: [u8; 4],
    /// Highest height of any tile loaded so far (world meters): the far
    /// shadow's march stops once its ray is above it.
    max_height: f32,
}

/// The 33² (or 17²) grid patch: integer grid coordinates in POSITION,
/// quads split along the canonical diagonal, counter-clockwise from +Y.
fn patch_mesh(quads: u32) -> Mesh {
    let n = quads + 1;
    let mut positions = Vec::with_capacity((n * n) as usize);
    for j in 0..n {
        for i in 0..n {
            #[allow(clippy::cast_precision_loss)]
            positions.push([i as f32, 0.0, j as f32]);
        }
    }
    let v = |i: u32, j: u32| u16::try_from(j * n + i).expect("small patch");
    let mut indices = Vec::with_capacity((quads * quads * 6) as usize);
    for j in 0..quads {
        for i in 0..quads {
            indices.extend_from_slice(&[v(i, j), v(i + 1, j + 1), v(i + 1, j)]);
            indices.extend_from_slice(&[v(i, j), v(i, j + 1), v(i + 1, j + 1)]);
        }
    }
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_indices(Indices::U16(indices))
}

fn array_view(image: &mut Image) {
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::D2Array),
        ..TextureViewDescriptor::default()
    });
}

/// An uninitialized clipmap array, written in strips from the render world.
fn clipmap_image(layers: usize, format: TextureFormat, filtered: bool) -> Image {
    let size = u32::try_from(CLIPMAP_SIZE).expect("small");
    let extent = Extent3d { width: size, height: size, depth_or_array_layers: u32::try_from(layers).expect("small") };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    array_view(&mut image);
    let filter = if filtered { ImageFilterMode::Linear } else { ImageFilterMode::Nearest };
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: filter,
        min_filter: filter,
        ..ImageSamplerDescriptor::default()
    });
    image
}

/// A layer texture array with its mip chain (bake layout: layer-major).
fn layer_array(size: u32, mips: u32, layers: usize, format: TextureFormat, data: Vec<u8>) -> Image {
    let extent =
        Extent3d { width: size, height: size, depth_or_array_layers: u32::try_from(layers.max(1)).expect("small") };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    image.texture_descriptor.mip_level_count = mips;
    image.data = Some(data);
    array_view(&mut image);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 16,
        ..ImageSamplerDescriptor::default()
    });
    image
}

/// Creates the GPU resources when a [`TerrainWorld`] appears or changes.
#[allow(clippy::too_many_arguments)]
fn setup_terrain(
    mut commands: Commands,
    world: Option<Res<TerrainWorld>>,
    state: Option<ResMut<TerrainState>>,
    settings: Res<TerrainSettings>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    far_map: Option<Res<FarShadowMap>>,
) {
    let Some(world) = world else {
        if let Some(state) = state {
            for (e, _) in state.nodes.values() {
                commands.entity(*e).despawn();
            }
            commands.remove_resource::<TerrainState>();
        }
        return;
    };
    if !world.is_changed() && state.is_some() {
        return;
    }
    if let Some(state) = state {
        for (e, _) in state.nodes.values() {
            commands.entity(*e).despawn();
        }
    }
    let baked = world.baked.clone();
    let (table, albedo, nar) = match baked.layers(settings.layer_array_size) {
        Some(Ok(a)) => {
            let n = a.table.layers.len();
            let albedo = layer_array(a.size, a.mips, n, TextureFormat::Rgba8UnormSrgb, a.albedo_height);
            let nar = layer_array(a.size, a.mips, n, TextureFormat::Rgba8Unorm, a.normal_ao_roughness);
            (a.table, albedo, nar)
        }
        other => {
            if let Some(Err(e)) = other {
                log::error!("terrain layer arrays: {e}");
            }
            let one = |c: [u8; 4], f| layer_array(1, 1, 1, f, c.to_vec());
            (
                LayerTable::default(),
                one([128, 128, 128, 255], TextureFormat::Rgba8UnormSrgb),
                one([128, 128, 255, 230], TextureFormat::Rgba8Unorm),
            )
        }
    };
    if table.layers.len() > params::MAX_LAYERS {
        log::error!("terrain: {} layers, only {} are drawn", table.layers.len(), params::MAX_LAYERS);
    }
    let slots = table.layers.iter().enumerate().filter_map(|(n, l)| u8::try_from(n).ok().map(|s| (l.id, s))).collect();
    let heights = images.add(clipmap_image(HEIGHT_LEVELS, TextureFormat::R32Float, false));
    let normals = images.add(clipmap_image(HEIGHT_LEVELS, TextureFormat::Rg8Snorm, true));
    let layer_ids = images.add(clipmap_image(LAYER_LEVELS, TextureFormat::Rgba8Uint, false));
    let layer_weights = images.add(clipmap_image(LAYER_LEVELS, TextureFormat::Rgba8Unorm, false));
    let macro_image = images.add(clipmap_image(MACRO_LEVELS, TextureFormat::Rgba8UnormSrgb, true));
    let params = buffers.add(ShaderBuffer::new(Params::default().0, RenderAssetUsages::default()));
    let material = materials.add(TerrainMaterial {
        params: params.clone(),
        heights: heights.clone(),
        normals: normals.clone(),
        layer_ids: layer_ids.clone(),
        layer_weights: layer_weights.clone(),
        macro_albedo: macro_image.clone(),
        albedo_height: images.add(albedo),
        normal_ao_roughness: images.add(nar),
        far_map: far_map.map(|m| m.image.clone()).unwrap_or_default(),
    });
    // What the bake used where there is no source, for tiles that don't exist.
    let fill = table.fill_height;
    let fill_slot = table.fill_layer.and_then(|id| table.slice_of(id)).map_or(0, |s| u8::try_from(s).unwrap_or(0));
    let fill_rgba =
        table.fill_layer.and_then(|id| table.layers.iter().find(|l| l.id == id)).map_or([70, 70, 70, 255], |l| {
            let c = l.mean_albedo.map(sf_terrain::bake::linear_to_srgb);
            [c[0], c[1], c[2], 255]
        });
    commands.insert_resource(TerrainState {
        ws: baked.worldspace().clone(),
        baked,
        material,
        params,
        heights,
        normals,
        layer_ids,
        layer_weights,
        macro_image,
        patch: meshes.add(patch_mesh(cdlod::PATCH_QUADS)),
        quadrant: meshes.add(patch_mesh(cdlod::PATCH_QUADS / 2)),
        table,
        slots,
        tiles: HashMap::new(),
        height_windows: [None; HEIGHT_LEVELS],
        layer_windows: [None; LAYER_LEVELS],
        macro_windows: [None; MACRO_LEVELS],
        nodes: HashMap::new(),
        origin: None,
        shift_settle: 0,
        fill,
        fill_slot,
        fill_rgba,
        max_height: fill,
    });
}

fn level_u8(l: usize) -> u8 {
    u8::try_from(l).expect("small level")
}

/// Tiles at `level` overlapping a window (plus `margin` samples) whose
/// lattice has `per` samples per tile.
fn window_tiles(w: ClipWindow, level: u8, per: i64, margin: i64, out: &mut HashSet<TileCoord>) {
    let lo = |a: usize| (w.min[a] - margin).div_euclid(per);
    let hi = |a: usize| (w.min[a] + CLIPMAP_SIZE - 1 + margin).div_euclid(per);
    for z in lo(1)..=hi(1) {
        for x in lo(0)..=hi(0) {
            if let (Ok(x), Ok(z)) = (i32::try_from(x), i32::try_from(z)) {
                out.insert(TileCoord::new(level, x, z));
            }
        }
    }
}

/// What each clipmap kind needs: (heights, layers, macro) windows' tiles.
struct Wanted {
    heights: [ClipWindow; HEIGHT_LEVELS],
    layers: [ClipWindow; LAYER_LEVELS],
    macros: [ClipWindow; MACRO_LEVELS],
}

impl Wanted {
    fn new(cam: DVec2, snap: i64) -> Self {
        let s = |shift: usize| u32::try_from(shift).expect("small");
        Self {
            heights: std::array::from_fn(|l| ClipWindow::centered(cam, s(l), snap)),
            layers: std::array::from_fn(|l| ClipWindow::centered(cam, s(l + 1), snap)),
            macros: std::array::from_fn(|l| ClipWindow::centered(cam, s(l + 1), snap)),
        }
    }

    fn tiles(&self, margin: i64) -> HashSet<TileCoord> {
        let mut out = HashSet::new();
        let per_h = i64::try_from(sf_terrain::TILE_QUADS).expect("small");
        let per_l = per_h / 2;
        for (l, w) in self.heights.iter().enumerate() {
            window_tiles(*w, level_u8(l), per_h, margin, &mut out);
        }
        for (l, w) in self.layers.iter().enumerate() {
            window_tiles(*w, level_u8(l), per_l, margin, &mut out);
        }
        for (l, w) in self.macros.iter().enumerate() {
            window_tiles(*w, level_u8(l), per_l, margin, &mut out);
        }
        out
    }
}

fn load_tile(baked: Arc<BakedTerrain>, c: TileCoord, wants_layers: bool) -> Option<Loaded> {
    let product = match baked.tile(c)? {
        Ok(p) => p,
        Err(e) => {
            log::error!("terrain tile {c:?}: {e}");
            return None;
        }
    };
    let macro_rgba = baked.macro_albedo(c).and_then(Result::ok).map(|m| m.rgba);
    let tile = product.height_tile();
    Some(Loaded {
        resident: Arc::new(ResidentTile { tile, bounds: product.bounds }),
        layers: if wants_layers { product.layers } else { None },
        macro_rgba,
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Height,
    Layer,
    Macro,
}

/// Queues the strips a window move exposes. Returns the bytes queued.
fn upload_level(
    state: &TerrainState,
    edits: Option<&sf_terrain::HeightEdits>,
    kind: Kind,
    level: usize,
    old: Option<ClipWindow>,
    new: ClipWindow,
    writes: &mut Vec<TextureWrite>,
) -> usize {
    let lv = level_u8(level);
    let ready = |c: TileCoord| match state.tiles.get(&c) {
        Some(Slot::Ready(t)) => Some(t.clone()),
        _ => None,
    };
    let mut bytes = 0;
    for rect in clipmap::exposed(old, new) {
        for (piece, texel) in clipmap::toroidal_pieces(rect) {
            let n = piece.count();
            let size = [u32::try_from(piece.size[0]).expect("small"), u32::try_from(piece.size[1]).expect("small")];
            let layer = u32::try_from(level).expect("small");
            match kind {
                Kind::Height => {
                    // One sample of margin: normals difference their neighbors.
                    let grown = SampleRect {
                        min: [piece.min[0] - 1, piece.min[1] - 1],
                        size: [piece.size[0] + 2, piece.size[1] + 2],
                    };
                    let held: HashMap<TileCoord, Arc<Loaded>> =
                        tiles_of(grown, lv, 512).filter_map(|c| ready(c).map(|t| (c, t))).collect();
                    let tile = |c: TileCoord| held.get(&c).map(|t| &t.resident.tile);
                    let mut out = vec![0.0f32; n];
                    clipmap::fill_heights(lv, piece, &tile, edits, state.fill, &mut out);
                    let data: Vec<u8> = out.iter().flat_map(|v| v.to_le_bytes()).collect();
                    let mut normals = vec![[0i8; 2]; n];
                    clipmap::fill_normals(lv, piece, &tile, edits, state.fill, &mut normals);
                    let normals: Vec<u8> = normals.iter().flat_map(|v| v.map(i8::cast_unsigned)).collect();
                    bytes += data.len() + normals.len();
                    writes.push(TextureWrite {
                        image: state.heights.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data,
                    });
                    writes.push(TextureWrite {
                        image: state.normals.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 2,
                        data: normals,
                    });
                }
                Kind::Layer => {
                    let held: HashMap<TileCoord, Arc<Loaded>> =
                        tiles_of(piece, lv, 256).filter_map(|c| ready(c).map(|t| (c, t))).collect();
                    let tile = |c: TileCoord| held.get(&c).and_then(|t| t.layers.as_ref());
                    let slot_of = |id: LayerId| state.slots.get(&id).copied().unwrap_or(0);
                    let mut ids = vec![[0u8; 4]; n];
                    let mut weights = vec![[0u8; 4]; n];
                    clipmap::fill_layers(lv, piece, &tile, &slot_of, state.fill_slot, &mut ids, &mut weights);
                    let ids: Vec<u8> = ids.concat();
                    let weights: Vec<u8> = weights.concat();
                    bytes += ids.len() + weights.len();
                    writes.push(TextureWrite {
                        image: state.layer_ids.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data: ids,
                    });
                    writes.push(TextureWrite {
                        image: state.layer_weights.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data: weights,
                    });
                }
                Kind::Macro => {
                    let held: HashMap<TileCoord, Arc<Loaded>> =
                        tiles_of(piece, lv, 256).filter_map(|c| ready(c).map(|t| (c, t))).collect();
                    let tile = |c: TileCoord| held.get(&c).and_then(|t| t.macro_rgba.as_deref());
                    let mut out = vec![[0u8; 4]; n];
                    clipmap::fill_rgba(lv, piece, &tile, state.fill_rgba, &mut out);
                    let data: Vec<u8> = out.concat();
                    bytes += data.len();
                    writes.push(TextureWrite {
                        image: state.macro_image.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data,
                    });
                }
            }
        }
    }
    bytes
}

fn tiles_of(rect: SampleRect, level: u8, per: i64) -> impl Iterator<Item = TileCoord> {
    let (x0, z0) = (rect.min[0].div_euclid(per), rect.min[1].div_euclid(per));
    let (x1, z1) = ((rect.min[0] + rect.size[0] - 1).div_euclid(per), (rect.min[1] + rect.size[1] - 1).div_euclid(per));
    (z0..=z1).flat_map(move |z| {
        (x0..=x1).filter_map(move |x| Some(TileCoord::new(level, i32::try_from(x).ok()?, i32::try_from(z).ok()?)))
    })
}

/// Whether every tile a window reads is loaded (or known absent).
fn window_ready(state: &TerrainState, w: ClipWindow, level: usize, per: i64, margin: i64) -> bool {
    let mut needed = HashSet::new();
    window_tiles(w, level_u8(level), per, margin, &mut needed);
    needed.iter().all(|c| matches!(state.tiles.get(c), Some(Slot::Ready(_) | Slot::Absent)))
}

/// Conservative node bounds from the finest resident level (bounds are
/// merged up the pyramid in the bake).
#[allow(clippy::many_single_char_names)]
fn node_bounds(state: &TerrainState, lod: i8, x: i64, z: i64) -> (f32, f32) {
    let blocks = i64::try_from(BLOCKS).expect("small");
    let lookup = |k: u8, bx: i64, bz: i64| -> Option<(f32, f32)> {
        let c =
            TileCoord::new(k, i32::try_from(bx.div_euclid(blocks)).ok()?, i32::try_from(bz.div_euclid(blocks)).ok()?);
        match state.tiles.get(&c)? {
            Slot::Ready(t) => {
                let b: &BlockBounds = &t.resident.bounds;
                Some(b.get(usize::try_from(bx.rem_euclid(blocks)).ok()?, usize::try_from(bz.rem_euclid(blocks)).ok()?))
            }
            Slot::Absent => Some((state.fill, state.fill)),
            Slot::Loading(_) => None,
        }
    };
    let first = u8::try_from(lod.max(0)).expect("non-negative");
    for k in first..=level_u8(HEIGHT_LEVELS - 1) {
        let level_k = i8::try_from(k).expect("small");
        let found = if level_k == lod {
            let mut acc: Option<(f32, f32)> = None;
            for (dx, dz) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let Some(b) = lookup(k, x * 2 + dx, z * 2 + dz) else {
                    acc = None;
                    break;
                };
                acc = Some(acc.map_or(b, |a| (a.0.min(b.0), a.1.max(b.1))));
            }
            acc
        } else {
            let shift = u32::try_from(i32::from(level_k) - i32::from(lod) - 1).expect("k > lod");
            lookup(k, x >> shift, z >> shift)
        };
        if let Some(b) = found {
            return b;
        }
    }
    (state.fill, state.fill)
}

/// Frames (the shift's and the next) in which clipmap uploads wait after an
/// origin shift.
const SHIFT_SETTLE_FRAMES: u8 = 2;

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn update_terrain(
    mut commands: Commands,
    state: Option<ResMut<TerrainState>>,
    settings: Res<TerrainSettings>,
    debug: Res<TerrainDebug>,
    origin: Res<TerrainOrigin>,
    far_map: Option<Res<FarShadowMap>>,
    mut heights_map: ResMut<TerrainHeightsMap>,
    mut terrain_stats: ResMut<TerrainStats>,
    mut writes: ResMut<ClipmapWrites>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut edited: MessageReader<TerrainEdited>,
    camera: Query<&GlobalTransform, With<TerrainCamera>>,
) {
    let Some(mut state) = state else { return };
    let Some(cam) = camera.iter().next() else { return };
    let started = Instant::now();
    let state = &mut *state;
    let cam_render = cam.translation();
    let cam_world = origin.0.offset(cam_render);
    let cam_g = global_coords(cam_world);
    let origin_g = global_coords(origin.0);
    let origin_moved = state.origin != Some(origin.0);
    state.shift_settle = if origin_moved { SHIFT_SETTLE_FRAMES } else { state.shift_settle.saturating_sub(1) };

    // 1. Finished loads.
    let heights = heights_map.0.entry(state.ws.clone()).or_default();
    let mut in_flight = 0;
    for (c, slot) in &mut state.tiles {
        if let Slot::Loading(task) = slot {
            match block_on(poll_once(task)) {
                Some(Some(t)) => {
                    let top = t.resident.bounds.max.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                    state.max_height = state.max_height.max(top);
                    heights.insert(t.resident.clone());
                    *slot = Slot::Ready(Arc::new(t));
                }
                Some(None) => *slot = Slot::Absent,
                None => in_flight += 1,
            }
        }
        let _ = c;
    }

    // 2. Requests (coarse levels first, so far terrain is never missing) and eviction.
    let wanted = Wanted::new(cam_g, settings.snap);
    let keep = wanted.tiles(128);
    let mut requests: Vec<TileCoord> = keep.iter().copied().filter(|c| !state.tiles.contains_key(c)).collect();
    requests.sort_by_key(|c| (std::cmp::Reverse(c.level), c.z, c.x));
    let pool = AsyncComputeTaskPool::get();
    for c in requests {
        if !state.baked.has_tile(c) {
            state.tiles.insert(c, Slot::Absent);
            continue;
        }
        if in_flight >= settings.max_loads {
            break;
        }
        let baked = state.baked.clone();
        let wants_layers = usize::from(c.level) < LAYER_LEVELS;
        state.tiles.insert(c, Slot::Loading(pool.spawn(async move { load_tile(baked, c, wants_layers) })));
        in_flight += 1;
    }
    let evict: Vec<TileCoord> = state
        .tiles
        .iter()
        .filter(|(c, s)| !keep.contains(c) && !matches!(s, Slot::Loading(_)))
        .map(|(c, _)| *c)
        .collect();
    for c in evict {
        state.tiles.remove(&c);
        heights.remove(c);
    }

    // 3. Edits invalidate the clipmap levels they touch.
    for TerrainEdited(_) in edited.read() {
        state.height_windows = [None; HEIGHT_LEVELS];
    }

    // 4. Clipmap uploads, one whole level move at a time, under the budget.
    let nothing_resident = state.height_windows.iter().all(Option::is_none);
    // Uploads wait out the frames around an origin shift (streaming.md,
    // "Spikes"): the render world is busy re-collecting every instance.
    let budget = if nothing_resident {
        settings.burst_budget
    } else if state.shift_settle > 0 {
        0
    } else {
        settings.upload_budget
    };
    let mut used = 0;
    let edits = Some(heights.edits());
    let per_h = i64::try_from(sf_terrain::TILE_QUADS).expect("small");
    for level in (0..HEIGHT_LEVELS).rev() {
        for kind in [Kind::Height, Kind::Macro, Kind::Layer] {
            if used >= budget {
                break;
            }
            let (old, new, per) = match kind {
                Kind::Height => (state.height_windows[level], wanted.heights[level], per_h),
                Kind::Macro => (state.macro_windows[level], wanted.macros[level], per_h / 2),
                Kind::Layer if level < LAYER_LEVELS => (state.layer_windows[level], wanted.layers[level], per_h / 2),
                Kind::Layer => continue,
            };
            if old == Some(new) || !window_ready(state, new, level, per, i64::from(kind == Kind::Height)) {
                continue;
            }
            used += upload_level(state, edits, kind, level, old, new, &mut writes.0);
            match kind {
                Kind::Height => state.height_windows[level] = Some(new),
                Kind::Macro => state.macro_windows[level] = Some(new),
                Kind::Layer => state.layer_windows[level] = Some(new),
            }
        }
    }

    // 5. CDLOD selection and the patch entities.
    let select_start = Instant::now();
    // An origin shift re-expresses the live patches in place (new transform
    // and bounds on the same entities) rather than respawning them: no entity
    // churn in the shift frame, which already rewrites every transform
    // (streaming.md, "Spikes"). A frozen selection still starts over.
    state.origin = Some(origin.0);
    if origin_moved && debug.freeze {
        for (_, (e, _)) in state.nodes.drain() {
            commands.entity(e).despawn();
        }
    }
    if !debug.freeze {
        let mut selected: Vec<SelectedNode> = Vec::with_capacity(1024);
        let camera3 = bevy_math::DVec3::new(cam_g.x, cam_world.to_f64().y, cam_g.y);
        let bounds = |lod: i8, x: i64, z: i64| node_bounds(state, lod, x, z);
        cdlod::select(camera3, &settings.cdlod, &bounds, &mut selected);
        let oy = origin.0.to_f64().y;
        let mut live: HashSet<NodeKey> = HashSet::with_capacity(selected.len());
        let displacement = state.table.layers.iter().map(|l| l.displacement_m.abs()).fold(0.0f32, f32::max);
        for node in &selected {
            live.insert(node.key);
            let key = node.key;
            let b = (node.min_height, node.max_height);
            let existing = state.nodes.get(&key).copied();
            if !origin_moved && existing.is_some_and(|(_, old)| old == b) {
                continue;
            }
            let g = key.first_sample();
            let spacing = NodeKey::spacing(key.lod);
            #[allow(clippy::cast_possible_truncation)]
            let translation = Vec3::new(((g.x - origin_g.x) * 0.5) as f32, 0.0, ((g.y - origin_g.y) * 0.5) as f32);
            let transform = Transform::from_translation(translation).with_scale(Vec3::new(spacing, 1.0, spacing));
            let margin = if key.lod < 0 { displacement + 0.05 } else { 0.05 };
            #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
            let (lo, hi) =
                ((f64::from(node.min_height) - oy) as f32 - margin, (f64::from(node.max_height) - oy) as f32 + margin);
            #[allow(clippy::cast_precision_loss)]
            let quads = key.quads() as f32;
            let aabb = Aabb::from_min_max(Vec3::new(0.0, lo, 0.0), Vec3::new(quads, hi.max(lo + 0.01), quads));
            if let Some((e, _)) = existing {
                // Bounds were coarser (or a fill guess) when it spawned, or
                // the origin moved. Replacing components keeps the archetype.
                commands.entity(e).insert((aabb, transform, GlobalTransform::from(transform)));
                state.nodes.insert(key, (e, b));
                continue;
            }
            let mesh = if key.quadrant.is_some() { state.quadrant.clone() } else { state.patch.clone() };
            let tag = u32::try_from(i32::from(key.lod) - i32::from(MIN_LOD)).expect("lod >= MIN_LOD");
            let e = commands
                .spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(state.material.clone()),
                    transform,
                    GlobalTransform::from(transform),
                    aabb,
                    NoAutoAabb,
                    MeshTag::new(tag),
                    TerrainPatch(key),
                ))
                .id();
            if debug.no_culling {
                commands.entity(e).insert(bevy_camera::visibility::NoFrustumCulling);
            }
            state.nodes.insert(key, (e, b));
        }
        state.nodes.retain(|k, (e, _)| {
            let keep = live.contains(k);
            if !keep {
                commands.entity(*e).despawn();
            }
            keep
        });
    }
    let select_ms = select_start.elapsed().as_secs_f32() * 1e3;

    // 6. Shader parameters.
    let mut p = Params::default();
    let oi = [origin_g.x.floor(), origin_g.y.floor()];
    #[allow(clippy::cast_possible_truncation)]
    let oi_i = oi.map(|v| v as i64);
    let o_abs = origin.0.to_f64();
    #[allow(clippy::cast_possible_truncation)]
    {
        p.set_f(params::P_CAMERA, [cam_render.x, cam_render.y, cam_render.z, o_abs.y as f32]);
        let low = |v: i64| i32::try_from(v.rem_euclid(1024)).expect("small");
        p.0[params::P_ORIGIN] = [
            ((origin_g.x - oi[0]) as f32).to_bits(),
            ((origin_g.y - oi[1]) as f32).to_bits(),
            low(oi_i[0]).cast_unsigned(),
            low(oi_i[1]).cast_unsigned(),
        ];
        let debug_bits =
            (u32::from(debug.lod_colors) * params::DEBUG_LOD) | (u32::from(debug.wireframe) * params::DEBUG_WIREFRAME)
                | (u32::from(debug.no_far_shadows) * params::DEBUG_NO_FAR_SHADOWS);
        p.0[params::P_MISC] = [
            (o_abs.x.rem_euclid(1024.0) as f32).to_bits(),
            (o_abs.z.rem_euclid(1024.0) as f32).to_bits(),
            state.fill.to_bits(),
            debug_bits,
        ];
    }
    let mask =
        |w: &[Option<ClipWindow>]| w.iter().enumerate().fold(0u32, |m, (l, w)| m | (u32::from(w.is_some()) << l));
    p.0[params::P_VALID] = [
        mask(&state.height_windows),
        mask(&state.layer_windows),
        mask(&state.macro_windows),
        u32::try_from(state.table.layers.len().min(params::MAX_LAYERS)).expect("small"),
    ];
    p.set_morph(&settings.cdlod);
    let level_slot = |w: Option<ClipWindow>, shift: u32| -> [i32; 4] {
        let o = [oi_i[0] >> shift, oi_i[1] >> shift];
        let base = |a: usize| i32::try_from(o[a].rem_euclid(CLIPMAP_SIZE)).expect("small");
        let rel = |a: usize| w.map_or(0, |w| i32::try_from(w.min[a] - o[a]).unwrap_or(0));
        [base(0), base(1), rel(0), rel(1)]
    };
    for l in 0..HEIGHT_LEVELS {
        let s = u32::try_from(l).expect("small");
        p.set_i(params::P_HEIGHT + l, level_slot(state.height_windows[l], s));
        p.set_i(params::P_MACRO + l, level_slot(state.macro_windows[l], s + 1));
    }
    for l in 0..LAYER_LEVELS {
        p.set_i(params::P_LAYER + l, level_slot(state.layer_windows[l], u32::try_from(l).expect("small") + 1));
    }
    p.set_f(params::P_DETAIL, [settings.splat_range, settings.blend_depth, 0.0, state.max_height]);
    for (n, l) in state.table.layers.iter().take(params::MAX_LAYERS).enumerate() {
        p.0[params::P_LAYERS + 2 * n] = [l.tiling_m.to_bits(), l.displacement_m.to_bits(), u32::from(l.triplanar), 0];
        p.set_f(params::P_LAYERS + 2 * n + 1, [l.mean_albedo[0], l.mean_albedo[1], l.mean_albedo[2], l.mean_height]);
    }
    if let Some(far) = &far_map {
        p.set_f(params::P_FAR, far.params.to_array());
    }
    if let Some(mut buffer) = buffers.get_mut(&state.params)
        && let Some(slots) = buffer.cast_slice_mut::<[u32; 4]>()
    {
        slots.copy_from_slice(p.as_slice());
    }

    *terrain_stats = TerrainStats {
        patches: state.nodes.len(),
        resident_tiles: state.tiles.values().filter(|s| matches!(s, Slot::Ready(_))).count(),
        loads_in_flight: in_flight,
        upload_bytes: used,
        select_ms,
        update_ms: started.elapsed().as_secs_f32() * 1e3,
        height_levels: mask(&state.height_windows).count_ones(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_mesh::VertexAttributeValues;

    /// The WGSL slot constants must match `params.rs`.
    #[test]
    fn shader_param_slots_match_rust() {
        let wgsl = include_str!("shaders/common.wesl");
        let slot = |name: &str| -> usize {
            let line = wgsl.lines().find(|l| l.starts_with(&format!("const {name}: u32 = "))).expect(name);
            let value = line.split('=').nth(1).unwrap().trim();
            value.chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap()
        };
        assert_eq!(slot("P_CAMERA"), params::P_CAMERA);
        assert_eq!(slot("P_ORIGIN"), params::P_ORIGIN);
        assert_eq!(slot("P_MISC"), params::P_MISC);
        assert_eq!(slot("P_VALID"), params::P_VALID);
        assert_eq!(slot("P_MORPH"), params::P_MORPH);
        assert_eq!(slot("P_HEIGHT"), params::P_HEIGHT);
        assert_eq!(slot("P_LAYER"), params::P_LAYER);
        assert_eq!(slot("P_MACRO"), params::P_MACRO);
        assert_eq!(slot("P_DETAIL"), params::P_DETAIL);
        assert_eq!(slot("P_LAYERS"), params::P_LAYERS);
        assert_eq!(slot("P_FAR"), params::P_FAR);
        assert_eq!(slot("HEIGHT_LEVELS"), HEIGHT_LEVELS);
        assert_eq!(slot("LAYER_LEVELS"), LAYER_LEVELS);
        assert_eq!(params::P_MORPH + params::MORPH_LODS, params::P_HEIGHT);
        assert_eq!(params::P_MACRO + MACRO_LEVELS, params::P_DETAIL);
    }

    /// Patches split quads along the canonical diagonal, counter-clockwise from +Y.
    #[test]
    fn patch_mesh_matches_the_canonical_triangulation() {
        let mesh = patch_mesh(cdlod::PATCH_QUADS);
        let Some(VertexAttributeValues::Float32x3(p)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!() };
        let Some(Indices::U16(idx)) = mesh.indices() else { panic!() };
        assert_eq!(p.len(), 33 * 33);
        assert_eq!(idx.len(), 32 * 32 * 6);
        for t in idx.as_chunks::<3>().0 {
            let [a, b, c] = [t[0], t[1], t[2]].map(|i| Vec3::from(p[usize::from(i)]));
            assert!((b - a).cross(c - a).y > 0.0, "clockwise triangle {t:?}");
        }
        // First quad: (0,0)-(1,1) is the shared edge.
        assert_eq!(&idx[..6], &[0, 34, 1, 0, 33, 34]);
    }
}
