//! The exposure pack (SW-5/SW-6; `sf_surface::view` "Exposure pack" has the model and the CPU
//! twin): the shelter and canopy maps' result eased over time, so cover and prints under trees
//! and roofs fade when the maps change instead of popping with the camera.
//!
//! - **Main world** ([`ExposureFrame`], written by `shelter::update_shelter` from its
//!   `ExposureClock`): this frame's pass (take in last frame's redraw, ease, or nothing) and this
//!   frame's redraw centre (the LOD origin of its views).
//! - **Render world** ([`ExposureGpu`]): two `Rgba16Float` 1024² packs (ping-pong, as the trail
//!   ring), one compute pass (`exposure.wgsl`) before the prepass on the frames that change the
//!   pack: R, G the solid and leaf casters' heights over the map centre (from the depth maps),
//!   B the lag, A the fresh ground exposure (the layer's tent at each texel centre, for a
//!   receiver below every caster). The fragment keeps its exact 3D tent (a roof top and the
//!   ground under it differ, which no 2D map can hold) and adds one gather of the lag. The current pack is the
//!   layer's shelter binding (mesh-view 45, `render::extract_view`); until a pass filled it the
//!   layer reads no shelter (open sky), as with the map off.
//!
//! Cost: a take-in pass reads 18 gathers per texel (~0.2 ms, once per redraw), an easing pass
//! one load and one store per texel (8 MB each way, ≈ 0.05 ms on the 4060 laptop) for
//! `EXPOSURE_RUN_S` after each redraw; steady frames run nothing. Memory: 16 MB (two packs).
//!
//! Also here: the views' LOD origins (`bevy_render::view::RenderViewLodOrigins`, vendored patch
//! "SW-5: per-view LOD origin"): this frame's shelter and canopy views resolve visibility ranges
//! from the map centre, as `shelter::split_casters` picks their casters.

use std::num::NonZeroU64;

use bevy_app::App;
use bevy_asset::{Assets, Handle, uuid_handle};
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_math::Vec3;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{
    sampler, texture_2d, texture_storage_2d, uniform_buffer_sized,
};
use bevy_render::render_resource::{
    AddressMode, BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
    BufferInitDescriptor, BufferUsages, CachedComputePipelineId, ComputePassDescriptor,
    ComputePipelineDescriptor, Extent3d, FilterMode, PipelineCache, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderStages, StorageTextureAccess, TextureDescriptor, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor,
};
use bevy_render::renderer::{RenderContext, RenderDevice};
use bevy_render::texture::GpuImage;
use bevy_render::view::RenderViewLodOrigins;
use bevy_render::{Extract, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use sf_diag_bevy::texview::{DebugTexture, DebugTextureAppExt, DebugTextures};
use sf_surface::view::{
    ExposureMode, ExposurePass, SHELTER_DOWN_M, SHELTER_LEAF_PASS, SHELTER_SIZE, SHELTER_UP_M,
};

use crate::shelter::{CanopyFrame, ShelterFrame};
use crate::{SurfaceGpu, SurfaceUniform};

const EXPOSURE_SHADER: Handle<Shader> = uuid_handle!("8b3e4f27-6c1d-4a95-b0e2-7d5f3a9c1e64");
const FORMAT: TextureFormat = TextureFormat::Rgba16Float;
/// The pass's uniform (`exposure.wgsl` `ExposureParams`): two `vec4` slots.
pub(crate) const PARAM_BYTES: usize = 32;

/// This frame's exposure work (main world, extracted).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ExposureFrame {
    /// The pass to run before this frame's prepass, if any.
    pub pass: Option<ExposurePass>,
    /// This frame's redraw centre (render space): its views' LOD origin.
    pub center: Option<Vec3>,
}

/// The pass's uniform bytes: `shift_mode` (shift x, y, mode, map edge) as i32, `ease` (factor,
/// depth range, depth below the centre, leaf pass) as f32.
pub(crate) fn params(pass: ExposurePass) -> [u8; PARAM_BYTES] {
    let s = pass.mode.shift();
    let ints = [s.x, s.y, pass.mode.code(), SHELTER_SIZE.cast_signed()];
    let floats = [pass.decay, SHELTER_UP_M + SHELTER_DOWN_M, SHELTER_DOWN_M, SHELTER_LEAF_PASS];
    let mut out = [0u8; PARAM_BYTES];
    for (i, v) in ints.iter().map(|v| v.to_le_bytes()).chain(floats.iter().map(|v| v.to_le_bytes())).enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&v);
    }
    out
}

/// Render world: the two packs, the pass and what it holds.
#[derive(Resource)]
pub(crate) struct ExposureGpu {
    sampled: [TextureView; 2],
    storage: [TextureView; 2],
    /// The pack bound this frame (written by this frame's pass, if any).
    current: usize,
    /// The bound pack holds the map the uniform names.
    valid: bool,
    sampler: Sampler,
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
    /// This frame's pass and the depth maps it reads.
    pending: Option<(ExposurePass, TextureView, TextureView)>,
    ready: Option<BindGroup>,
}

impl ExposureGpu {
    /// The pack the layer reads (mesh-view binding 45).
    pub fn current_view(&self) -> &TextureView {
        &self.sampled[self.current]
    }

    /// Whether the bound pack holds a map (else the layer must read no shelter).
    pub fn valid(&self) -> bool {
        self.valid
    }
}

fn pack(device: &RenderDevice, label: &'static str) -> (TextureView, TextureView) {
    let texture = device.create_texture(&TextureDescriptor {
        label: Some(label),
        size: Extent3d { width: SHELTER_SIZE, height: SHELTER_SIZE, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: FORMAT,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    (texture.create_view(&TextureViewDescriptor::default()), texture.create_view(&TextureViewDescriptor::default()))
}

impl FromWorld for ExposureGpu {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let (a, a_storage) = pack(device, "surface_exposure_pack_a");
        let (b, b_storage) = pack(device, "surface_exposure_pack_b");
        // The depth maps' gathers: unfiltered, clamped at the maps' edge (the layer's sampler).
        let gather_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("surface_exposure_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            ..SamplerDescriptor::default()
        });
        let unfiltered = TextureSampleType::Float { filterable: false };
        let layout = BindGroupLayoutDescriptor::new(
            "surface_exposure",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer_sized(false, NonZeroU64::new(PARAM_BYTES as u64)),
                    texture_2d(unfiltered),
                    texture_2d(unfiltered),
                    texture_2d(unfiltered),
                    sampler(SamplerBindingType::NonFiltering),
                    texture_storage_2d(FORMAT, StorageTextureAccess::WriteOnly),
                ),
            ),
        );
        let pipeline = world.resource::<PipelineCache>().queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("surface_exposure".into()),
            layout: vec![layout.clone()],
            shader: EXPOSURE_SHADER,
            entry_point: Some("main".into()),
            zero_initialize_workgroup_memory: false,
            ..ComputePipelineDescriptor::default()
        });
        Self {
            sampled: [a, b],
            storage: [a_storage, b_storage],
            current: 0,
            valid: false,
            sampler: gather_sampler,
            layout,
            pipeline,
            pending: None,
            ready: None,
        }
    }
}

/// Main-world resource always (the shelter writes it); render side when there is a renderer.
pub(crate) fn build(app: &mut App) {
    app.init_resource::<ExposureFrame>()
        .register_debug_texture(DebugTexture::render(
            "surface.exposure",
            "exposure pack (current): R solid / G leaf casters' height over the map centre (m), B lag, A fresh ground exposure 0..1",
            (SHELTER_SIZE, SHELTER_SIZE),
            FORMAT,
            |world| world.get_resource::<ExposureGpu>().map(|g| g.current_view().clone()),
        ))
        .add_systems(bevy_app::Startup, register_debug_textures.after(crate::setup));
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        shaders
            .insert(&EXPOSURE_SHADER, Shader::from_wgsl(include_str!("exposure.wgsl"), "embedded://sf_surface_bevy/exposure.wgsl"))
            .expect("uuid handles always insert");
    }
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<RenderViewLodOrigins>()
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<ExposureGpu>)
        .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
        .add_systems(Core3d, exposure_pass.before(Core3dSystems::Prepass));
}

/// The maps the pack is made from, for the `tex` viewer (`sf_texview_bevy`; read before the
/// render world takes their `RENDER_WORLD`-only data).
#[allow(clippy::needless_pass_by_value)]
fn register_debug_textures(gpu: Res<SurfaceGpu>, images: Res<Assets<Image>>, mut textures: ResMut<DebugTextures>) {
    let maps = [
        ("surface.shelter", "shelter map: top-down depth of solid casters (roofs, arches, rock), 128 m around the camera", &gpu.shelter),
        ("surface.canopy", "canopy map: top-down depth of leaf casters (partial shelter), 128 m around the camera", &gpu.canopy),
        ("surface.cells", "cell window: per-32 m cell surface state (R wet, G puddle, B snow, A ash), toroidal", &gpu.cells),
    ];
    for (name, description, handle) in maps {
        if let Some(image) = images.get(handle) {
            textures.register(DebugTexture::image(name, description, handle.clone(), image));
        }
    }
}

/// Sets this frame's LOD origins, decides this frame's pass and flips the packs (before
/// `render::extract_view` binds the current one). A pack that lost its pass (the pass never
/// ran, the pipeline or a map was not ready) is rebuilt fresh from the maps, which hold the map
/// the uniform names.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
pub(crate) fn extract(
    frame: Extract<Res<ExposureFrame>>,
    shelter: Extract<Res<ShelterFrame>>,
    canopy: Extract<Res<CanopyFrame>>,
    uniform: Extract<Res<SurfaceUniform>>,
    surface: Extract<Option<Res<SurfaceGpu>>>,
    images: Res<RenderAssets<GpuImage>>,
    origins: Option<ResMut<RenderViewLodOrigins>>,
    gpu: Option<ResMut<ExposureGpu>>,
    cache: Res<PipelineCache>,
) {
    if let (Some(center), Some(mut origins)) = (frame.center, origins) {
        for req in shelter.0.iter().chain(canopy.0.iter()) {
            origins.0.insert(req.retained(), center);
        }
    }
    let Some(mut gpu) = gpu else { return };
    if gpu.pending.take().is_some() || gpu.ready.take().is_some() {
        // Last frame's pass never ran: the bound pack holds nothing valid.
        gpu.valid = false;
    }
    if uniform.shelter.x <= 0.0 {
        // No map: the next one starts fresh.
        gpu.valid = false;
        return;
    }
    let mut pass = match (frame.pass, gpu.valid) {
        (Some(p), _) => p,
        (None, true) => return,
        (None, false) => ExposurePass { mode: ExposureMode::Fresh, decay: 1.0 },
    };
    if !gpu.valid {
        pass.mode = ExposureMode::Fresh;
    }
    let maps = surface.as_deref().and_then(|s| {
        let view = |h: &Handle<Image>| images.get(h).map(|g| g.texture_view.clone());
        Some((view(&s.shelter)?, view(&s.canopy)?))
    });
    let (Some((shelter_map, canopy_map)), Some(_)) = (maps, cache.get_compute_pipeline(gpu.pipeline)) else {
        gpu.valid = false;
        return;
    };
    gpu.current ^= 1;
    gpu.valid = true;
    gpu.pending = Some((pass, shelter_map, canopy_map));
}

fn prepare(gpu: Option<ResMut<ExposureGpu>>, device: Res<RenderDevice>, cache: Res<PipelineCache>) {
    let Some(mut gpu) = gpu else { return };
    let Some((pass, shelter_map, canopy_map)) = gpu.pending.take() else { return };
    let bytes = params(pass);
    let uniform = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("surface_exposure_params"),
        contents: &bytes,
        usage: BufferUsages::UNIFORM,
    });
    let (cur, prev) = (gpu.current, gpu.current ^ 1);
    let group = device.create_bind_group(
        "surface_exposure",
        &cache.get_bind_group_layout(&gpu.layout),
        &BindGroupEntries::sequential((
            uniform.as_entire_binding(),
            &shelter_map,
            &canopy_map,
            &gpu.sampled[prev],
            &gpu.sampler,
            &gpu.storage[cur],
        )),
    );
    gpu.ready = Some(group);
}

/// The exposure pass, once per frame (the first camera to render runs it), before the prepass:
/// every pass of the frame (prepass, main, the terrain's raise) reads the same pack.
fn exposure_pass(gpu: Option<ResMut<ExposureGpu>>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let Some(mut gpu) = gpu else { return };
    let Some(pipeline) = cache.get_compute_pipeline(gpu.pipeline) else { return };
    let Some(group) = gpu.ready.take() else { return };
    let groups = SHELTER_SIZE.div_ceil(8);
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some("surface_exposure"), timestamp_writes: None });
    let span = diagnostics.pass_span(&mut pass, "surface_exposure");
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &group, &[]);
    pass.dispatch_workgroups(groups, groups, 1);
    span.end(&mut pass);
}
