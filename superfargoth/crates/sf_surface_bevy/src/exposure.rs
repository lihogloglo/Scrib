//! The exposure pack (SW-5/SW-6; `sf_surface::view` "Exposure pack" has the model and the CPU
//! twin): the shelter and canopy maps' result eased over time, so cover and prints under trees
//! and roofs fade when the maps change instead of popping with the camera.
//!
//! - **Main world** ([`ExposureFrame`], written by `shelter::update_shelter` from its
//!   `ExposureClock`): this frame's pass (take in last frame's redraw, ease, or nothing).
//! - **Render world** ([`ExposureGpu`]): two `Rgba16Float` 1024² packs (ping-pong, as the trail
//!   ring), one compute pass (`exposure.wgsl`) before the prepass on the frames that change the
//!   pack: R, G the solid and leaf casters' heights over the map centre (from the depth maps),
//!   B the lag, A the fresh ground exposure (the layer's tent at each texel centre, for a
//!   receiver below every caster). The fragment keeps its exact 3D tent (a roof top and the
//!   ground under it differ, which no 2D map can hold) and adds one gather of the lag. The current pack is the
//!   layer's shelter binding (mesh-view 45, `render::extract_view`); until a pass filled it the
//!   layer reads no shelter (open sky), as with the map off.
//!
//! - **Far level** (SW-5e): the same pass on the far maps (±512 m, 1 m texels; a bilinear test
//!   instead of the tent, `test.x`), into two more packs (mesh-view binding 49, which held the
//!   raw canopy map until SW-5 moved the leaves into the pack's G). It runs first: a near take-in
//!   reads the far pack (`outer`) for what its band and the texels it newly covers showed, so the
//!   hand-over between the levels eases like the rest (`sf_surface::view::exposure_texel_in`).
//!
//! Cost: a take-in pass reads 18 gathers per texel (~0.2 ms, once per redraw; the far one 2 and
//! the near one 4 more loads), an easing pass one load and one store per texel (8 MB each way,
//! ≈ 0.05 ms on the 4060 laptop) for `EXPOSURE_RUN_S` after each redraw; steady frames run
//! nothing. Memory: 32 MB (four packs).
//!
//! Also here: the views' LOD distance (`bevy_render::view::RenderViewLodDistances`, vendored patch
//! "SW-5d: fixed per-view LOD distance"): this frame's shelter and canopy views resolve every
//! visibility range at `shelter::SHELTER_LOD_DISTANCE_M` (finest level), the far views at
//! `ShelterFarSettings::lod_distance_m`, as `split_casters` picks.

use std::num::NonZeroU64;

use bevy_app::App;
use bevy_asset::{Assets, Handle, uuid_handle};
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_math::Vec2;
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
use bevy_render::view::RenderViewLodDistances;
use bevy_render::{Extract, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use sf_diag_bevy::texview::{DebugTexture, DebugTextureAppExt, DebugTextures};
use sf_surface::view::{ExposureMode, ExposurePass, NEAR_LEVEL, SHELTER_LEAF_PASS, SHELTER_SIZE, ShelterLevel};

use crate::shelter::{CanopyFrame, FarCanopyFrame, FarShelterFrame, ShelterFrame};
use crate::{SurfaceGpu, SurfaceUniform};

const EXPOSURE_SHADER: Handle<Shader> = uuid_handle!("8b3e4f27-6c1d-4a95-b0e2-7d5f3a9c1e64");
const FORMAT: TextureFormat = TextureFormat::Rgba16Float;
/// The pass's uniform (`exposure.wgsl` `ExposureParams`): four `vec4` slots.
pub(crate) const PARAM_BYTES: usize = 64;

/// This frame's exposure work (main world, extracted).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ExposureFrame {
    /// The near pack's pass to run before this frame's prepass, if any.
    pub pass: Option<ExposurePass>,
    /// The far pack's (SW-5e), run before the near one.
    pub far_pass: Option<ExposurePass>,
    /// The far level's geometry while it is on.
    pub far: Option<ShelterLevel>,
    /// The far views' LOD distance (`RenderViewLodDistances`).
    pub far_lod_distance: f32,
    /// A near take-in pass's uv mapping into the far pack (`sf_surface::view::outer_uv_map`):
    /// `None` without a far map.
    pub outer: Option<(f32, Vec2)>,
}

/// The pass's uniform bytes: `shift_mode` (shift x, y, mode, map edge) as i32, `ease` (factor,
/// depth range, depth below the centre, leaf pass), `test` (x: 1 the far level's bilinear test, 0
/// the near tent) and `outer` (x: 1 when an outer pack holds a map, y: uv scale, zw: uv offset
/// into it) as f32.
pub(crate) fn params(pass: ExposurePass, level: &ShelterLevel, bilinear: bool, outer: Option<(f32, Vec2)>) -> [u8; PARAM_BYTES] {
    let s = pass.mode.shift();
    let ints = [s.x, s.y, pass.mode.code(), SHELTER_SIZE.cast_signed()];
    let (on, scale, offset) = outer.map_or((0.0, 0.0, Vec2::ZERO), |(scale, offset)| (1.0, scale, offset));
    let floats = [
        pass.decay,
        level.depth_m(),
        level.down_m,
        SHELTER_LEAF_PASS,
        if bilinear { 1.0 } else { 0.0 },
        0.0,
        0.0,
        0.0,
        on,
        scale,
        offset.x,
        offset.y,
    ];
    let mut out = [0u8; PARAM_BYTES];
    for (i, v) in ints.iter().map(|v| v.to_le_bytes()).chain(floats.iter().map(|v| v.to_le_bytes())).enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&v);
    }
    out
}

/// One level's packs (ping-pong) and its pass.
pub(crate) struct Pack {
    sampled: [TextureView; 2],
    storage: [TextureView; 2],
    /// The pack bound this frame (written by this frame's pass, if any).
    current: usize,
    /// The bound pack holds the map the uniform names.
    valid: bool,
    /// This frame's params and the depth maps the pass reads.
    pending: Option<([u8; PARAM_BYTES], TextureView, TextureView)>,
    ready: Option<BindGroup>,
}

impl Pack {
    fn new(device: &RenderDevice, labels: [&'static str; 2]) -> Self {
        let (a, a_storage) = pack(device, labels[0]);
        let (b, b_storage) = pack(device, labels[1]);
        Self { sampled: [a, b], storage: [a_storage, b_storage], current: 0, valid: false, pending: None, ready: None }
    }

    /// The pack the layer reads.
    pub fn current_view(&self) -> &TextureView {
        &self.sampled[self.current]
    }

    /// Whether the bound pack holds a map (else the layer must read no shelter at this level).
    pub fn valid(&self) -> bool {
        self.valid
    }

    /// Decides this frame's pass (module docs of `extract`) and flips the packs; `None` when
    /// none runs. `held`: the uniform names a map for this level.
    fn decide(&mut self, held: bool, pass: Option<ExposurePass>, maps: Option<(TextureView, TextureView)>, pipeline_ready: bool) -> Option<ExposurePass> {
        if self.pending.take().is_some() || self.ready.take().is_some() {
            // Last frame's pass never ran: the bound pack holds nothing valid.
            self.valid = false;
        }
        if !held {
            // No map: the next one starts fresh.
            self.valid = false;
            return None;
        }
        let mut pass = match (pass, self.valid) {
            (Some(p), _) => p,
            (None, true) => return None,
            (None, false) => ExposurePass { mode: ExposureMode::Fresh, decay: 1.0 },
        };
        if !self.valid {
            pass.mode = ExposureMode::Fresh;
        }
        let (Some((shelter, canopy)), true) = (maps, pipeline_ready) else {
            self.valid = false;
            return None;
        };
        self.current ^= 1;
        self.valid = true;
        self.pending = Some(([0; PARAM_BYTES], shelter, canopy));
        Some(pass)
    }
}

/// Render world: both levels' packs (SW-5e: near and far), the pass and what it holds.
#[derive(Resource)]
pub(crate) struct ExposureGpu {
    near: Pack,
    far: Pack,
    sampler: Sampler,
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}

impl ExposureGpu {
    /// The near pack the layer reads (mesh-view binding 45).
    pub fn current_view(&self) -> &TextureView {
        self.near.current_view()
    }

    /// Whether the near pack holds a map (else the layer must read no shelter).
    pub fn valid(&self) -> bool {
        self.near.valid()
    }

    /// The far pack the layer reads (mesh-view binding 49, SW-5e).
    pub fn far_view(&self) -> &TextureView {
        self.far.current_view()
    }

    /// Whether the far pack holds a map.
    pub fn far_valid(&self) -> bool {
        self.far.valid()
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
        let near = Pack::new(device, ["surface_exposure_pack_a", "surface_exposure_pack_b"]);
        let far = Pack::new(device, ["surface_exposure_far_pack_a", "surface_exposure_far_pack_b"]);
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
                    texture_2d(unfiltered),
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
        Self { near, far, sampler: gather_sampler, layout, pipeline }
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
        .register_debug_texture(DebugTexture::render(
            "surface.exposure_far",
            "far exposure pack (SW-5e, ±512 m): R solid / G leaf casters' height over the map centre (m), B lag, A fresh ground exposure 0..1",
            (SHELTER_SIZE, SHELTER_SIZE),
            FORMAT,
            |world| world.get_resource::<ExposureGpu>().map(|g| g.far_view().clone()),
        ))
        .add_systems(bevy_app::Startup, register_debug_textures.after(crate::setup));
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        shaders
            .insert(&EXPOSURE_SHADER, Shader::from_wgsl(include_str!("exposure.wgsl"), "embedded://sf_surface_bevy/exposure.wgsl"))
            .expect("uuid handles always insert");
    }
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<RenderViewLodDistances>()
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<ExposureGpu>)
        .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
        .add_systems(Core3d, exposure_pass.before(Core3dSystems::Prepass));
}

/// The maps the packs are made from, for the `tex` viewer (`sf_texview_bevy`; read before the
/// render world takes their `RENDER_WORLD`-only data).
#[allow(clippy::needless_pass_by_value)]
fn register_debug_textures(gpu: Res<SurfaceGpu>, images: Res<Assets<Image>>, mut textures: ResMut<DebugTextures>) {
    let maps = [
        ("surface.shelter", "shelter map: top-down depth of solid casters (roofs, arches, rock), 128 m around the camera", &gpu.shelter),
        ("surface.canopy", "canopy map: top-down depth of leaf casters (partial shelter), 128 m around the camera", &gpu.canopy),
        ("surface.shelter_far", "far shelter map (SW-5e): top-down depth of solid casters, 1024 m around the camera", &gpu.far_shelter),
        ("surface.canopy_far", "far canopy map (SW-5e): top-down depth of leaf casters, 1024 m around the camera", &gpu.far_canopy),
        ("surface.cells", "cell window: per-32 m cell surface state (R wet, G puddle, B snow, A ash), toroidal", &gpu.cells),
    ];
    for (name, description, handle) in maps {
        if let Some(image) = images.get(handle) {
            textures.register(DebugTexture::image(name, description, handle.clone(), image));
        }
    }
}

/// Both levels' redraw requests (their views' LOD distances).
type Views<'w> = (Res<'w, ShelterFrame>, Res<'w, CanopyFrame>, Res<'w, FarShelterFrame>, Res<'w, FarCanopyFrame>);

/// Sets this frame's LOD distances, decides each level's pass and flips its packs (before
/// `render::extract_view` binds the current ones), the far level first: the near pass reads the
/// far pack as it stands after the far pass. A pack that lost its pass (the pass never ran, the
/// pipeline or a map was not ready) is rebuilt fresh from the maps, which hold the map the uniform
/// names.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
pub(crate) fn extract(
    frame: Extract<Res<ExposureFrame>>,
    views: Extract<Views>,
    uniform: Extract<Res<SurfaceUniform>>,
    surface: Extract<Option<Res<SurfaceGpu>>>,
    images: Res<RenderAssets<GpuImage>>,
    lod_distances: Option<ResMut<RenderViewLodDistances>>,
    gpu: Option<ResMut<ExposureGpu>>,
    cache: Res<PipelineCache>,
) {
    let (shelter, canopy, far_shelter, far_canopy) = &*views;
    if let Some(mut lod_distances) = lod_distances {
        for req in shelter.0.iter().chain(canopy.0.iter()) {
            lod_distances.0.insert(req.retained(), crate::shelter::SHELTER_LOD_DISTANCE_M);
        }
        for req in far_shelter.0.iter().chain(far_canopy.0.iter()) {
            lod_distances.0.insert(req.retained(), frame.far_lod_distance);
        }
    }
    let Some(mut gpu) = gpu else { return };
    let ready = cache.get_compute_pipeline(gpu.pipeline).is_some();
    let view = |h: &Handle<Image>| images.get(h).map(|g| g.texture_view.clone());
    let maps = |a: fn(&SurfaceGpu) -> &Handle<Image>, b: fn(&SurfaceGpu) -> &Handle<Image>| {
        surface.as_deref().and_then(|s| Some((view(a(s))?, view(b(s))?)))
    };
    let far_maps = maps(|s| &s.far_shelter, |s| &s.far_canopy);
    let far_held = uniform.far_shelter.x > 0.0;
    if let Some(pass) = gpu.far.decide(far_held, frame.far_pass, far_maps, ready)
        && let Some(level) = frame.far
        && let Some(p) = gpu.far.pending.as_mut()
    {
        p.0 = params(pass, &level, true, None);
    }
    let near_maps = maps(|s| &s.shelter, |s| &s.canopy);
    // The near pack reads the far pack only while the far pack holds a map.
    let outer = frame.outer.filter(|_| gpu.far.valid);
    if let Some(pass) = gpu.near.decide(uniform.shelter.x > 0.0, frame.pass, near_maps, ready)
        && let Some(p) = gpu.near.pending.as_mut()
    {
        p.0 = params(pass, &NEAR_LEVEL, false, outer);
    }
    if gpu.far.pending.is_some() && frame.far.is_none() {
        // A far map with no level this frame (switched off as it was taken in): drop it.
        gpu.far.pending = None;
        gpu.far.valid = false;
    }
}

fn prepare(gpu: Option<ResMut<ExposureGpu>>, device: Res<RenderDevice>, cache: Res<PipelineCache>) {
    let Some(mut gpu) = gpu else { return };
    let gpu = &mut *gpu;
    let layout = cache.get_bind_group_layout(&gpu.layout);
    // The far pass's outer binding is unused (its outer is open sky): its own last pack. The
    // near pass reads the far pack the far pass writes this frame.
    let far_outer = gpu.far.current_view().clone();
    for (level, outer) in [(&mut gpu.far, None), (&mut gpu.near, Some(far_outer))] {
        let Some((bytes, shelter_map, canopy_map)) = level.pending.take() else { continue };
        let uniform = device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("surface_exposure_params"),
            contents: &bytes,
            usage: BufferUsages::UNIFORM,
        });
        let (cur, prev) = (level.current, level.current ^ 1);
        let outer = outer.unwrap_or_else(|| level.sampled[prev].clone());
        let group = device.create_bind_group(
            "surface_exposure",
            &layout,
            &BindGroupEntries::sequential((
                uniform.as_entire_binding(),
                &shelter_map,
                &canopy_map,
                &level.sampled[prev],
                &gpu.sampler,
                &level.storage[cur],
                &outer,
            )),
        );
        level.ready = Some(group);
    }
}

/// The exposure passes, once per frame (the first camera to render runs them), before the
/// prepass: every pass of the frame (prepass, main, the terrain's raise) reads the same packs.
/// The far level first: the near pass reads it.
fn exposure_pass(gpu: Option<ResMut<ExposureGpu>>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let Some(mut gpu) = gpu else { return };
    let Some(pipeline) = cache.get_compute_pipeline(gpu.pipeline) else { return };
    let groups = SHELTER_SIZE.div_ceil(8);
    for (name, group) in [("surface_exposure_far", gpu.far.ready.take()), ("surface_exposure", gpu.near.ready.take())] {
        let Some(group) = group else { continue };
        let diagnostics = ctx.diagnostic_recorder();
        let diagnostics = diagnostics.as_deref();
        let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some(name), timestamp_writes: None });
        let span = diagnostics.pass_span(&mut pass, name);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(groups, groups, 1);
        span.end(&mut pass);
    }
}
