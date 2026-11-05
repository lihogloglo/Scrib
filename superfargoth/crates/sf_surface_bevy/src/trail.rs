//! Trails, slice W6a (surface-weather.md "Trails (W6) and why DLSS needs motion vectors"): the
//! trail ring, its stamps and their refill. Shading only so far (`layer.wesl`
//! `surface_apply_cover` cuts the cover and adds the print's gradient); W6b displaces terrain.
//!
//! - **Main world** ([`update_trails`]): the host writes [`TrailStamp`] messages (presentation
//!   only: they never feed gameplay); the stamps within [`STAMP_RANGE_M`] of the camera (nearest
//!   [`MAX_STAMPS`]), the ring window around the camera, the ring depth (the deepest cover look,
//!   `SurfaceUniform::cover[c].y`) and the refill (`sf_surface::trail::Refill`, from the weather's
//!   snow and ash fall at the camera over game hours) go to [`TrailFrame`]; the uniform gets the
//!   ring's placement (slots `trail`, `trail_cells`).
//! - **Render world** ([`TrailGpu`]): two `Rgba16Float` 1024² textures (current, previous; W6b
//!   reads both for motion vectors), one compute pass (`trail.wgsl`, the twin of
//!   `sf_surface::trail::TrailPass::run`) when there is work (stamps, a refill quantum, the window
//!   moved); otherwise nothing runs and the current ring stays bound. Mesh-view binding 52 holds
//!   the current ring (sampled with the cells' repeating, filtering sampler: the ring is a torus).
//!
//! Why one compute pass instead of instanced stamp quads with Min/Max blending (the spec's
//! first sketch): re-centering, refill and stamps all read the previous ring and write the next
//! one, which W6b needs anyway as the motion vectors' previous ring; one full-ring pass is
//! ~16 MB of traffic (≈ 0.06 ms on the 4060 laptop) on the frames that change it, and the
//! stamps' Min/Max combine is exact per texel with no blend state.

use std::num::NonZeroU64;

use bevy_asset::{Assets, Handle, uuid_handle};
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_ecs::prelude::*;
use bevy_math::{DVec2, IVec2, IVec4, Vec2, Vec4};
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_resource::binding_types::{texture_2d, texture_storage_2d, uniform_buffer_sized};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BufferInitDescriptor, BufferUsages,
    CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, PipelineCache, ShaderStages,
    StorageTextureAccess, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
    TextureViewDescriptor,
};
use bevy_render::renderer::{RenderContext, RenderDevice};
use bevy_render::{Extract, Render, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;
use sf_surface::trail::{
    MAX_STAMPS, RING_SIZE, RING_TEXEL_M, Refill, STAMP_RANGE_M, Stamp, TrailPass, refill_rate_m_per_h, window_min,
};
use sf_weather_bevy::CurrentWeather;

use crate::{SurfaceOrigin, SurfaceSettings, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera};

const TRAIL_SHADER: Handle<Shader> = uuid_handle!("2f0d6a59-8c1e-4b7a-9f36-5e1c0a7b3d92");
const FORMAT: TextureFormat = TextureFormat::Rgba16Float;

/// One print (a planted foot, later a lying body): an oriented ellipse at a world position.
/// Presentation only: stamps never feed gameplay (net rule).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct TrailStamp {
    pub at: WorldPos,
    /// Facing on the ground (world X, Z); the ellipse's long axis. Zero: +X.
    pub forward: Vec2,
    /// Half length along `forward`, half width across (m).
    pub half_m: Vec2,
    /// Depth as a share of the cover depth (1: down to the ground).
    pub depth: f32,
}

/// This frame's ring work (main world, extracted).
#[derive(Resource, Clone, Debug, Default)]
pub struct TrailFrame {
    /// Trails on with a cover look to cut (else the shading ignores the ring).
    pub on: bool,
    pub new_min: IVec2,
    /// Render X, Z of `new_min`'s min corner.
    pub corner: Vec2,
    pub depth_m: f32,
    pub refill_m: f32,
    pub stamps: Vec<Stamp>,
}

#[derive(Default)]
pub(crate) struct TrailClock {
    refill: Refill,
    hours: Option<f64>,
}

/// The ring depth: the deepest cover look (`SurfaceUniform::cover[c]`: x 1 / tiling, 0 = no
/// look; y depth).
pub(crate) fn ring_depth(uniform: &SurfaceUniform) -> f32 {
    uniform.cover.iter().filter(|c| c.x > 0.0).map(|c| c.y).fold(0.0, f32::max)
}

/// Gathers the stamps, places the ring and releases refill (see the module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::cast_possible_truncation)]
pub(crate) fn update_trails(
    settings: Res<SurfaceSettings>,
    origin: Res<SurfaceOrigin>,
    cameras: Query<&GlobalTransform, With<SurfaceWeatherCamera>>,
    weather: Option<Res<CurrentWeather>>,
    mut stamps: MessageReader<TrailStamp>,
    mut uniform: ResMut<SurfaceUniform>,
    mut frame: ResMut<TrailFrame>,
    mut stats: ResMut<SurfaceStats>,
    mut clock: Local<TrailClock>,
) {
    // Last frame's placement, for the displaced terrain's motion vectors (W6b): the render world
    // binds last frame's ring beside it (`TrailGpu::previous`).
    let (prev, prev_cells) = (uniform.trail, uniform.trail_cells);
    if uniform.trail_prev != prev || uniform.trail_prev_cells != prev_cells {
        uniform.trail_prev = prev;
        uniform.trail_prev_cells = prev_cells;
    }
    let depth = ring_depth(&uniform);
    let camera =cameras.iter().next().map(GlobalTransform::translation);
    let on = settings.layer && settings.cover && settings.trails && depth > 0.0;
    let Some(camera) = camera.filter(|_| on) else {
        stamps.clear();
        clock.hours = None;
        frame.on = false;
        frame.stamps.clear();
        if uniform.trail != Vec4::ZERO {
            uniform.trail = Vec4::ZERO;
        }
        return;
    };
    let world = origin.0.offset(camera).to_f64();
    let new_min = window_min(DVec2::new(world.x, world.z));
    let o = origin.0.to_f64();
    let corner = (new_min.as_dvec2() * RING_TEXEL_M - DVec2::new(o.x, o.z)).as_vec2();
    // Refill from the fall at the camera, over game hours.
    let mut refill_m = 0.0;
    if let Some(w) = weather.as_deref().filter(|w| w.valid) {
        let dh = clock.hours.map_or(0.0, |h| w.hours - h);
        clock.hours = Some(w.hours);
        let rate = refill_rate_m_per_h(w.frame.surface.snow, w.frame.surface.ash, cover_depth(&uniform, 0), cover_depth(&uniform, 1));
        refill_m = clock.refill.add(rate, dh);
    }
    // The nearest stamps in range.
    let cam_xz = Vec2::new(camera.x, camera.z);
    let mut near: Vec<(f32, Stamp)> = stamps
        .read()
        .filter_map(|s| {
            let p = origin.0.delta_f32(s.at);
            let center = Vec2::new(p.x, p.z);
            let d = center.distance(cam_xz);
            (d <= STAMP_RANGE_M && s.depth > 0.0).then(|| {
                let axis = s.forward.try_normalize().unwrap_or(Vec2::X);
                (d, Stamp { center, axis, half: s.half_m.max(Vec2::splat(0.01)), depth: s.depth.min(1.0) })
            })
        })
        .collect();
    near.sort_by(|a, b| a.0.total_cmp(&b.0));
    near.truncate(MAX_STAMPS);
    stats.trail_stamps += near.len() as u64;
    frame.on = true;
    frame.new_min = new_min;
    frame.corner = corner;
    frame.depth_m = depth;
    frame.refill_m = refill_m;
    frame.stamps = near.into_iter().map(|(_, s)| s).collect();
    let trail = Vec4::new(corner.x, corner.y, (1.0 / RING_TEXEL_M) as f32, depth);
    if uniform.trail != trail {
        uniform.trail = trail;
    }
    let cells = IVec4::new(new_min.x.rem_euclid(RING_SIZE), new_min.y.rem_euclid(RING_SIZE), RING_SIZE, 0);
    if uniform.trail_cells != cells {
        uniform.trail_cells = cells;
    }
}

fn cover_depth(uniform: &SurfaceUniform, c: usize) -> f32 {
    let l = uniform.cover[c];
    if l.x > 0.0 { l.y } else { 0.0 }
}

// -- render world ----------------------------------------------------------------------------

/// Render world: the two rings, the pass and what it holds.
#[derive(Resource)]
pub(crate) struct TrailGpu {
    sampled: [TextureView; 2],
    storage: [TextureView; 2],
    /// The ring bound this frame (written by this frame's pass, if any).
    pub current: usize,
    /// The ring bound last frame (W6b motion vectors: equal to `current` when no pass ran).
    pub previous: usize,
    /// The window the current ring holds (`None`: clear it all on the next pass).
    held: Option<IVec2>,
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
    pending: Option<TrailPass>,
    ready: Option<BindGroup>,
}

impl TrailGpu {
    pub fn current_view(&self) -> &TextureView {
        &self.sampled[self.current]
    }

    /// Last frame's ring (mesh-view binding 53).
    pub fn previous_view(&self) -> &TextureView {
        &self.sampled[self.previous]
    }
}

fn ring(device: &RenderDevice, label: &'static str) -> (TextureView, TextureView) {
    let edge = RING_SIZE.unsigned_abs();
    let texture = device.create_texture(&TextureDescriptor {
        label: Some(label),
        size: Extent3d { width: edge, height: edge, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: FORMAT,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    (texture.create_view(&TextureViewDescriptor::default()), texture.create_view(&TextureViewDescriptor::default()))
}

impl FromWorld for TrailGpu {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let (a, a_storage) = ring(device, "surface_trail_ring_a");
        let (b, b_storage) = ring(device, "surface_trail_ring_b");
        let layout = BindGroupLayoutDescriptor::new(
            "surface_trail",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer_sized(false, NonZeroU64::new((sf_surface::trail::PARAM_SLOTS * 16) as u64)),
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    texture_storage_2d(FORMAT, StorageTextureAccess::WriteOnly),
                ),
            ),
        );
        let pipeline = world.resource::<PipelineCache>().queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("surface_trail".into()),
            layout: vec![layout.clone()],
            shader: TRAIL_SHADER,
            entry_point: Some("main".into()),
            zero_initialize_workgroup_memory: false,
            ..ComputePipelineDescriptor::default()
        });
        Self { sampled: [a, b], storage: [a_storage, b_storage], current: 0, previous: 0, held: None, layout, pipeline, pending: None, ready: None }
    }
}

pub(crate) fn build(app: &mut bevy_app::App) {
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        shaders
            .insert(&TRAIL_SHADER, Shader::from_wgsl(include_str!("trail.wgsl"), "embedded://sf_surface_bevy/trail.wgsl"))
            .expect("uuid handles always insert");
    }
    let Some(render_app) = app.get_sub_app_mut(bevy_render::RenderApp) else { return };
    render_app
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<TrailGpu>)
        .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
        .add_systems(Core3d, trail_pass.before(Core3dSystems::Prepass));
}

/// Decides this frame's pass and flips the rings (before `render::extract_view` binds the
/// current one).
pub(crate) fn extract(frame: Extract<Res<TrailFrame>>, gpu: Option<ResMut<TrailGpu>>, cache: Res<PipelineCache>) {
    let Some(mut gpu) = gpu else { return };
    gpu.previous = gpu.current;
    if gpu.pending.take().is_some() || gpu.ready.take().is_some() {
        // Last frame's pass never ran: its ring holds nothing valid.
        gpu.held = None;
    }
    if !frame.on || cache.get_compute_pipeline(gpu.pipeline).is_none() {
        return;
    }
    if frame.stamps.is_empty() && frame.refill_m <= 0.0 && gpu.held == Some(frame.new_min) {
        return;
    }
    let pass = TrailPass {
        corner: frame.corner,
        depth_m: frame.depth_m,
        new_min: frame.new_min,
        old_min: gpu.held,
        refill_m: frame.refill_m,
        stamps: frame.stamps.clone(),
    };
    gpu.current ^= 1;
    gpu.held = Some(frame.new_min);
    gpu.pending = Some(pass);
}

fn prepare(gpu: Option<ResMut<TrailGpu>>, device: Res<RenderDevice>, cache: Res<PipelineCache>) {
    let Some(mut gpu) = gpu else { return };
    let Some(pass) = gpu.pending.take() else { return };
    let bytes: Vec<u8> = pass.slots().iter().flatten().flat_map(|w| w.to_le_bytes()).collect();
    let params = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("surface_trail_params"),
        contents: &bytes,
        usage: BufferUsages::UNIFORM,
    });
    let (cur, prev) = (gpu.current, gpu.current ^ 1);
    let group = device.create_bind_group(
        "surface_trail",
        &cache.get_bind_group_layout(&gpu.layout),
        &BindGroupEntries::sequential((params.as_entire_binding(), &gpu.sampled[prev], &gpu.storage[cur])),
    );
    gpu.ready = Some(group);
}

/// The ring pass, once per frame (the first camera to render runs it), before the prepass.
fn trail_pass(gpu: Option<ResMut<TrailGpu>>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let Some(mut gpu) = gpu else { return };
    let Some(pipeline) = cache.get_compute_pipeline(gpu.pipeline) else { return };
    let Some(group) = gpu.ready.take() else { return };
    let groups = RING_SIZE.unsigned_abs().div_ceil(8);
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some("surface_trail"), timestamp_writes: None });
    let span = diagnostics.pass_span(&mut pass, "surface_trail");
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &group, &[]);
    pass.dispatch_workgroups(groups, groups, 1);
    span.end(&mut pass);
}
