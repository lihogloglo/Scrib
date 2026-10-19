//! The world weather map on the GPU (weather.md "Distant weather (WE4)";
//! as built: "`WE4b` as built").
//!
//! The app (or `sf_weather_bevy`) runs [`sf_weather::WeatherMap::update`]
//! with CPU texels off and mirrors it into [`CloudWeatherMap`] with
//! [`CloudWeatherMap::sync`]. The render world uploads the raster (once) and
//! the climate table (when it changed), then the `weather_map` compute pass
//! (`sf_weather::MAP_WGSL`, the GPU twin of the CPU resolve) writes the
//! 256² world-anchored toroidal map, layers A, B, C, D (`Rgba16Float`): the
//! whole window when the table changed, only the entering strips on a
//! recenter, nothing on a uniform day (the clouds then keep their uniform
//! scalars). The clouds and the cloud shadow map sample layer A
//! (`cloud_density.wesl`, `map_a`).

use std::num::NonZeroU64;
use std::sync::Arc;

use bevy_app::App;
use bevy_asset::{uuid_handle, Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_math::IVec2;
use bevy_render::render_resource::binding_types::{storage_buffer_read_only_sized, texture_storage_2d_array, uniform_buffer_sized};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, Buffer, BufferInitDescriptor, BufferUsages, CachedComputePipelineId,
    ComputePipelineDescriptor, Extent3d, PipelineCache, ShaderStages, StorageTextureAccess, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension,
};
use bevy_render::renderer::{RenderContext, RenderDevice};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use sf_weather::{MapGpuGeometry, MapUpdate, WeatherMap, resolve_rects};

const MAP_SHADER: Handle<Shader> = uuid_handle!("6b0f6f0e-3c1a-4d1e-9a57-0d5e2f7b8c41");
const FORMAT: TextureFormat = TextureFormat::Rgba16Float;

/// Main world: the weather map the clouds read (none: the uniform
/// `Clouds` weather everywhere, the pre-WE4 path).
#[derive(Resource, Clone, Default)]
pub struct CloudWeatherMap {
    geometry: Option<MapGpuGeometry>,
    cells: Arc<Vec<u32>>,
    raster_gen: u64,
    rows: Arc<Vec<[f32; 4]>>,
    table_gen: u64,
    window: IVec2,
    uniform: bool,
    /// `envelope(rows)`, kept with the rows.
    envelope: [f32; 4],
}

/// Lowest cloud base, highest cloud top and most cirrus over the table's rows
/// (A, B, C, D per row): the trace's envelope shell. A blend of rows is a convex
/// combination, so every texel's layer lies inside it. `[3]`: 1 when some row
/// precipitates or the rows' height-fog densities differ (the trace's distant
/// shafts and far fog, `WE4d`), else 0.
fn envelope(rows: &[[f32; 4]]) -> [f32; 4] {
    let (mut e, mut fog) = ([f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.0], (f32::INFINITY, f32::NEG_INFINITY));
    for r in rows.as_chunks::<4>().0 {
        let (b, c, d) = (r[1], r[2], r[3]);
        e = [e[0].min(b[0]), e[1].max(b[0] + b[1].max(1.0)), e[2].max(c[0]), e[3].max(d[0])];
        fog = (fog.0.min(c[2]), fog.1.max(c[2]));
    }
    e[3] = f32::from(e[3] > 1e-3 || fog.1 - fog.0 > 1e-7);
    e
}

impl CloudWeatherMap {
    /// Mirrors `map` after `map.update(..)` (or `update_window`) returned
    /// `update`: the table when it was resolved anew, the window, the uniform
    /// flag. Run it with the map's CPU texels off (`set_cpu_texels(false)`).
    pub fn sync(&mut self, map: &WeatherMap, update: MapUpdate) {
        let geometry = map.gpu_geometry();
        if self.geometry != Some(geometry) {
            self.set_raster(map);
        }
        let (window, size) = map.window();
        let full = update.table_changed || update.texels == size.unsigned_abs().pow(2);
        if full || self.rows.is_empty() {
            self.rows = Arc::new(map.gpu_rows());
            self.envelope = envelope(&self.rows);
            self.table_gen += 1;
        }
        self.window = window;
        self.uniform = update.uniform;
    }

    /// Re-uploads `map`'s raster (a new worldspace or edited regions).
    pub fn set_raster(&mut self, map: &WeatherMap) {
        self.geometry = Some(map.gpu_geometry());
        self.cells = Arc::new(map.raster().gpu_cells());
        self.raster_gen += 1;
        self.table_gen += 1;
        self.rows = Arc::new(map.gpu_rows());
        self.envelope = envelope(&self.rows);
    }

    /// No map: the clouds use their uniform weather.
    pub fn clear(&mut self) {
        self.geometry = None;
    }
}

#[derive(Resource, Default)]
struct ExtractedMap(CloudWeatherMap);

/// The map window the clouds sample this frame.
#[derive(Clone, Copy)]
pub(crate) struct MapView {
    pub window: IVec2,
    pub size: i32,
    pub texel_m: f64,
    /// Lowest base, highest top, most cirrus over the map's climates; 1 when
    /// distant precipitation or fog differences exist (`WE4d`).
    pub envelope: [f32; 4],
}

/// Render world: the map texture, its inputs and the pending resolve.
#[derive(Resource)]
pub(crate) struct WeatherMapGpu {
    size: i32,
    /// Sampled view (layers A, B, C, D), bound by the clouds.
    pub view: TextureView,
    storage: TextureView,
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
    cells: Option<(u64, Buffer)>,
    rows: Option<(u64, Buffer)>,
    /// The window the texture holds (`None`: nothing valid).
    resolved: Option<IVec2>,
    /// The resolve to dispatch this frame: bind group, rects.
    pending: Option<(BindGroup, u32)>,
    /// Set when the clouds should sample the map this frame.
    pub active: Option<MapView>,
}

fn map_texture(device: &RenderDevice, size: i32) -> (TextureView, TextureView) {
    let edge = size.unsigned_abs();
    let texture = device.create_texture(&TextureDescriptor {
        label: Some("weather_map"),
        size: Extent3d { width: edge, height: edge, depth_or_array_layers: sf_weather::MAP_LAYERS },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: FORMAT,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let array = TextureViewDescriptor { dimension: Some(TextureViewDimension::D2Array), ..TextureViewDescriptor::default() };
    (texture.create_view(&array), texture.create_view(&array))
}

impl FromWorld for WeatherMapGpu {
    #[allow(clippy::cast_possible_wrap)]
    fn from_world(world: &mut World) -> Self {
        let size = sf_weather::MAP_SIZE as i32;
        let (view, storage) = map_texture(world.resource::<RenderDevice>(), size);
        let layout = BindGroupLayoutDescriptor::new(
            "weather_map",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer_sized(false, NonZeroU64::new(96)),
                    storage_buffer_read_only_sized(false, None),
                    storage_buffer_read_only_sized(false, None),
                    texture_storage_2d_array(FORMAT, StorageTextureAccess::WriteOnly),
                ),
            ),
        );
        let pipeline = world.resource::<PipelineCache>().queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("weather_map".into()),
            layout: vec![layout.clone()],
            shader: MAP_SHADER,
            entry_point: Some("resolve".into()),
            zero_initialize_workgroup_memory: false,
            ..ComputePipelineDescriptor::default()
        });
        Self { size, view, storage, layout, pipeline, cells: None, rows: None, resolved: None, pending: None, active: None }
    }
}

pub(crate) fn build(app: &mut App) {
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        shaders
            .insert(&MAP_SHADER, Shader::from_wgsl(sf_weather::MAP_WGSL, "embedded://sf_atmos_bevy/weather_map.wgsl"))
            .expect("uuid handles always insert");
    }
    app.init_resource::<CloudWeatherMap>();
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<ExtractedMap>()
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<WeatherMapGpu>)
        .add_systems(ExtractSchedule, extract)
        .add_systems(Render, prepare.in_set(RenderSystems::PrepareResources));
}

fn extract(map: Extract<Res<CloudWeatherMap>>, mut out: ResMut<ExtractedMap>) {
    out.0.clone_from(&map);
}

fn buffer(device: &RenderDevice, label: &'static str, contents: &[u8], usage: BufferUsages) -> Buffer {
    device.create_buffer_with_data(&BufferInitDescriptor { label: Some(label), contents, usage })
}

/// Uploads what changed and schedules the resolve (full or strips).
fn prepare(map: Res<ExtractedMap>, gpu: Option<ResMut<WeatherMapGpu>>, device: Res<RenderDevice>, cache: Res<PipelineCache>) {
    let Some(mut gpu) = gpu else { return };
    let m = &map.0;
    gpu.active = None;
    let ready = cache.get_compute_pipeline(gpu.pipeline).is_some();
    let Some(geometry) = m.geometry.filter(|_| !m.uniform && ready && !m.rows.is_empty()) else {
        gpu.resolved = None;
        gpu.pending = None;
        return;
    };
    if gpu.pending.take().is_some() {
        // Last frame's resolve never ran (no camera drew): start over.
        gpu.resolved = None;
    }
    if geometry.size != gpu.size {
        (gpu.view, gpu.storage) = map_texture(&device, geometry.size);
        gpu.size = geometry.size;
        gpu.resolved = None;
    }
    if gpu.cells.as_ref().is_none_or(|c| c.0 != m.raster_gen) {
        let bytes: Vec<u8> = m.cells.iter().flat_map(|c| c.to_le_bytes()).collect();
        gpu.cells = Some((m.raster_gen, buffer(&device, "weather_map_raster", &bytes, BufferUsages::STORAGE)));
        gpu.resolved = None;
    }
    if gpu.rows.as_ref().is_none_or(|r| r.0 != m.table_gen) {
        let bytes: Vec<u8> = m.rows.iter().flatten().flat_map(|x| x.to_le_bytes()).collect();
        gpu.rows = Some((m.table_gen, buffer(&device, "weather_map_rows", &bytes, BufferUsages::STORAGE)));
        gpu.resolved = None;
    }
    let rects = resolve_rects(m.window, geometry.size, gpu.resolved);
    if let (false, Some((_, cells)), Some((_, rows))) = (rects.is_empty(), &gpu.cells, &gpu.rows) {
        let params: Vec<u8> = geometry.params(m.window, &rects).iter().flat_map(|w| w.to_le_bytes()).collect();
        let params = buffer(&device, "weather_map_params", &params, BufferUsages::UNIFORM);
        let group = device.create_bind_group(
            "weather_map",
            &cache.get_bind_group_layout(&gpu.layout),
            &BindGroupEntries::sequential((params.as_entire_binding(), cells.as_entire_binding(), rows.as_entire_binding(), &gpu.storage)),
        );
        #[allow(clippy::cast_possible_truncation)]
        let n = rects.len() as u32;
        gpu.pending = Some((group, n));
    }
    gpu.resolved = Some(m.window);
    gpu.active = Some(MapView { window: m.window, size: geometry.size, texel_m: geometry.texel_m, envelope: m.envelope });
}

/// The resolve dispatch (GPU timer `render/weather_map/elapsed_gpu`); once
/// per frame, before the cloud shadow map.
pub(crate) fn map_pass(gpu: Option<ResMut<WeatherMapGpu>>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let Some(mut gpu) = gpu else { return };
    let Some(pipeline) = cache.get_compute_pipeline(gpu.pipeline) else { return };
    let Some((group, rects)) = gpu.pending.take() else { return };
    let groups = gpu.size.unsigned_abs().div_ceil(8);
    crate::render::compute(&mut ctx, "weather_map", pipeline, (&group, &[]), [groups, groups, rects]);
}
