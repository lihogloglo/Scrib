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
//!   `sf_surface::trail::TrailPass::run`) when there is work ([`TrailFrame::has_work`]: stamps, a
//!   refill quantum, a capture, released press decay, the window moved); otherwise nothing runs
//!   and the current ring stays bound. Mesh-view binding 52 holds the current ring (sampled with
//!   the cells' repeating, filtering sampler: the ring is a torus). B holds the deformer ceiling
//!   (S1a, [`crate::deform`]): the pass merges last frame's capture. A holds the press (GC-T1):
//!   feet press it within [`TrailPressSettings::radius_m`], it springs back over
//!   [`TrailPressSettings::recovery_s`] (`sf_surface::trail::PressClock`, frame seconds); the
//!   deformer capture presses it under bodies (GC-T2b) and around them within
//!   [`TrailPressSettings::spread_m`] (GC-T3).
//! - **Ring readers** ([`TrailViews`], GC-3d2): the render world's two rings and where each lies,
//!   for any plugin that reads the ring outside the mesh-view bindings (blades). The pass runs in
//!   the root `RenderGraph` schedule before `camera_driver` ([`TrailPassSystems`]), so every
//!   reader this frame, in any camera or before them, sees this frame's ring.
//!
//! Why one compute pass instead of instanced stamp quads with Min/Max blending (the spec's
//! first sketch): re-centering, refill and stamps all read the previous ring and write the next
//! one, which W6b needs anyway as the motion vectors' previous ring; one full-ring pass is
//! ~16 MB of traffic (≈ 0.06 ms on the 4060 laptop) on the frames that change it, and the
//! stamps' Min/Max combine is exact per texel with no blend state.

use std::num::NonZeroU64;

use bevy_asset::{AssetId, Assets, Handle, uuid_handle};
use bevy_core_pipeline::schedule::camera_driver;
use bevy_ecs::prelude::*;
use bevy_math::{DVec2, IVec2, IVec4, Vec2, Vec4};
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_image::Image;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{
    storage_buffer_read_only_sized, texture_2d, texture_depth_2d, texture_storage_2d, uniform_buffer_sized,
};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BufferInitDescriptor, BufferUsages,
    CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, PipelineCache, ShaderStages,
    StorageTextureAccess, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
    TextureViewDescriptor,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderGraph, RenderGraphSystems};
use bevy_render::texture::GpuImage;
use bevy_render::{Extract, Render, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;
use sf_diag_bevy::texview::{DebugTexture, DebugTextureAppExt};
use sf_surface::trail::{
    MAX_STAMPS, Press, PressClock, RING_SIZE, RING_TEXEL_M, Refill, STAMP_RANGE_M, Stamp, TrailPass, refill_rate_m_per_h,
    window_min,
};
use sf_tunables::{Def, Source, Unit, def};
use sf_weather_bevy::CurrentWeather;

use crate::{SurfaceOrigin, SurfaceSettings, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera};

const TRAIL_SHADER: Handle<Shader> = uuid_handle!("2f0d6a59-8c1e-4b7a-9f36-5e1c0a7b3d92");
const FORMAT: TextureFormat = TextureFormat::Rgba16Float;

/// `surface.trail.press_recovery_s`: [`TrailPressSettings::recovery_s`].
pub const PRESS_RECOVERY_S: Def<f32> = def!(float "surface.trail.press_recovery_s" = 2.0)
    .range(0.25, 10.0)
    .unit(Unit::Seconds)
    .help("Plants and blades pressed by a foot spring back with this e-folding time (the press channel, A); snow prints stay")
    .source(Source::Guess);

/// `surface.trail.press_radius_m`: [`TrailPressSettings::radius_m`].
pub const PRESS_RADIUS_M: Def<f32> = def!(float "surface.trail.press_radius_m" = 0.35)
    .range(0.1, 1.5)
    .unit(Unit::Meters)
    .help("A foot presses plants and blades this far around it (full at the foot, none at this distance)")
    .source(Source::Guess);

/// `surface.trail.press_height_m`: [`TrailPressSettings::height_m`].
pub const PRESS_HEIGHT_M: Def<f32> = def!(float "surface.trail.press_height_m" = 1.0)
    .range(0.1, 1.5)
    .unit(Unit::Meters)
    .help("A body, leg or creature presses plants and blades under it from this height down (full on the ground, none at this height; roughly knee to thigh height of a walking body)")
    .source(Source::Guess);

/// `surface.trail.press_spread_m`: [`TrailPressSettings::spread_m`] (GC-T3).
pub const PRESS_SPREAD_M: Def<f32> = def!(float "surface.trail.press_spread_m" = 0.45)
    .range(0.0, 1.5)
    .unit(Unit::Meters)
    .help("A body's press spreads this far around it (full at the body, none at this distance), so legs push the plants beside and ahead of them; 0 = only under the body (an interaction sphere's radius, a leg's reach into vegetation)")
    .source(Source::Guess);

/// The press channel's settings (GC-T1; the tunables above).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct TrailPressSettings {
    pub recovery_s: f32,
    pub radius_m: f32,
    /// The deformer capture's press height (GC-T2b, `sf_surface::deform::press_of`).
    pub height_m: f32,
    /// The deformer capture's press spread (GC-T3, `sf_surface::deform::Capture::spread_press_at`).
    pub spread_m: f32,
}

impl Default for TrailPressSettings {
    fn default() -> Self {
        Self {
            recovery_s: PRESS_RECOVERY_S.default_value(),
            radius_m: PRESS_RADIUS_M.default_value(),
            height_m: PRESS_HEIGHT_M.default_value(),
            spread_m: PRESS_SPREAD_M.default_value(),
        }
    }
}

/// Who asks for the ring besides a cover look (GC-T2a). A plugin that reads the press channel (A)
/// sets [`press`](Self::press) while it has readers near the camera; the ring and its stamps then
/// run even with no snow or ash look, at depth 0: R/G (the print) and B stay 0 and the shading
/// ignores the ring (`SurfaceUniform::trail.w` is the depth), only A carries the press.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrailDemand {
    pub press: bool,
}

/// The ring pass's set (render app, root `RenderGraph` schedule, in `RenderGraphSystems::Render`
/// before `camera_driver`). A reader recording its own passes in that schedule orders them after it.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrailPassSystems;

/// Metres inside a ring's edge where it reads 0 (`layer.wesl` `surface_ring_lookup`: the texels
/// at the edge are refilled as the window moves).
// tunables: const (format: mirrors `layer.wesl` `surface_ring_lookup`, checked by `shader_tests`)
pub const RING_EDGE_MARGIN_M: f32 = 0.5;
/// Metres over which the ring fades in after [`RING_EDGE_MARGIN_M`] (`layer.wesl` `TRAIL_EDGE_FADE_M`).
// tunables: const (format: mirrors `layer.wesl` `TRAIL_EDGE_FADE_M`, checked by `shader_tests`)
pub const RING_EDGE_FADE_M: f32 = 2.0;

/// Where a ring lies, in the layout of `SurfaceUniform::trail` / `trail_cells`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TrailPlacement {
    /// xy: render X, Z of the window's first texel's min corner (m); z: texels per metre (0: no
    /// ring); w: the print depth (m; 0 for a press-only ring, which still holds the press, A).
    pub place: Vec4,
    /// xy: the texel holding the window's first corner (the ring is a torus); z: texels per side.
    pub cells: IVec4,
}

impl TrailPlacement {
    /// Whether a ring is placed (the ring is on).
    pub fn is_placed(&self) -> bool {
        self.place.z > 0.0 && self.cells.z > 0
    }
}

/// Render world (GC-3d2): the ring bound this frame and last frame's, and where each lies, for
/// readers outside the mesh-view bindings (52, 53). Updated every frame in `ExtractSchedule` while
/// the trail plugin's render part exists; a placement that is not placed means no ring (off, or no
/// camera). Channels: R depression, G lip, B deformer ceiling, A press 0..1 (`Rgba16Float`, one
/// texel per `1 / place.z` m, unfiltered reads address the texel the point is in).
#[derive(Resource, Clone, Debug)]
pub struct TrailViews {
    pub current: TextureView,
    pub previous: TextureView,
    pub placement: TrailPlacement,
    pub previous_placement: TrailPlacement,
}

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
    /// Trails on with a cover look to cut or a press demand ([`TrailDemand`]; else no pass runs).
    pub on: bool,
    pub new_min: IVec2,
    /// Render X, Z of `new_min`'s min corner.
    pub corner: Vec2,
    pub depth_m: f32,
    pub refill_m: f32,
    pub stamps: Vec<Stamp>,
    /// The deformer capture to merge (S1a, [`crate::deform`]): last frame's request, drawn by
    /// last frame's shadow pass (this frame's draws after the ring pass), and its target.
    pub capture: Option<(sf_surface::deform::Capture, AssetId<Image>)>,
    /// The press decay released this frame and the stamps' press reach (GC-T1).
    pub press: Press,
    /// The deformer capture's press height this frame (m, GC-T2b): [`TrailPressSettings::height_m`]
    /// with a press demand ([`TrailDemand`]), else 0 (the capture presses nothing and runs only
    /// over snow or ash).
    pub capture_press_m: f32,
    /// The deformer capture's press spread this frame (m, GC-T3): [`TrailPressSettings::spread_m`].
    pub capture_spread_m: f32,
}

impl TrailFrame {
    /// Whether the ring pass has work this frame with the ring holding the window `held`:
    /// stamps, a refill quantum, a capture to merge, released press decay (until the last press
    /// is under the floor, `sf_surface::trail::PressClock`), or the window moved.
    pub fn has_work(&self, held: Option<IVec2>) -> bool {
        !self.stamps.is_empty() || self.refill_m > 0.0 || self.capture.is_some() || self.press.dt_s > 0.0 || held != Some(self.new_min)
    }
}

#[derive(Default)]
pub(crate) struct TrailClock {
    refill: Refill,
    hours: Option<f64>,
    press: PressClock,
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
    time: Option<Res<Time>>,
    press: Option<Res<TrailPressSettings>>,
    demand: Option<Res<TrailDemand>>,
    mut clock: Local<TrailClock>,
) {
    // Last frame's placement, for the displaced terrain's motion vectors (W6b): the render world
    // binds last frame's ring beside it (`TrailGpu::previous`).
    let (prev, prev_cells) = (uniform.trail, uniform.trail_cells);
    if uniform.trail_prev != prev || uniform.trail_prev_cells != prev_cells {
        uniform.trail_prev = prev;
        uniform.trail_prev_cells = prev_cells;
    }
    // The print's depth: the deepest cover look, 0 when there is none to cut (a press-only ring,
    // GC-T2a: R/G stay 0 and the shading ignores the ring).
    let cut = settings.cover && ring_depth(&uniform) > 0.0;
    let depth = if cut { ring_depth(&uniform) } else { 0.0 };
    let press_only = demand.is_some_and(|d| d.press);
    let camera = cameras.iter().next().map(GlobalTransform::translation);
    let on = settings.layer && settings.trails && (cut || press_only);
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
    // The press's spring back (GC-T1), over this frame's (virtual) seconds: a paused game holds it.
    let press = press.as_deref().copied().unwrap_or_default();
    let dt = time.as_deref().map_or(0.0, Time::delta_secs);
    // A press this frame: a stamp's, or the capture's merged this pass (GC-T2b; last frame's
    // request, `deform::update_deform` hands it over after this system: the clock restarts a
    // frame late, which only keeps the pass running one frame longer).
    let captured = frame.capture.as_ref().is_some_and(|(c, _)| c.press_m > 0.0);
    let pressed = (!frame.stamps.is_empty() && press.radius_m > 0.0) || captured;
    let dt_s = clock.press.advance(dt, press.recovery_s, pressed);
    frame.press = Press { dt_s, recovery_s: press.recovery_s, radius_m: press.radius_m };
    frame.capture_press_m = if press_only { press.height_m } else { 0.0 };
    frame.capture_spread_m = press.spread_m;
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
    /// This frame's pass and the capture target it merges (S1a).
    pending: Option<(TrailPass, Option<AssetId<Image>>)>,
    ready: Option<BindGroup>,
    /// Bound in place of a capture when a pass has none (1×1, cleared: nothing carves).
    no_capture: TextureView,
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
        let no_capture = device
            .create_texture(&TextureDescriptor {
                label: Some("surface_trail_no_capture"),
                size: Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&TextureViewDescriptor::default());
        let layout = BindGroupLayoutDescriptor::new(
            "surface_trail",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer_sized(false, NonZeroU64::new((sf_surface::trail::PARAM_SLOTS * 16) as u64)),
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    texture_storage_2d(FORMAT, StorageTextureAccess::WriteOnly),
                    texture_depth_2d(),
                    storage_buffer_read_only_sized(false, NonZeroU64::new(4)),
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
        Self {
            sampled: [a, b],
            storage: [a_storage, b_storage],
            current: 0,
            previous: 0,
            held: None,
            layout,
            pipeline,
            pending: None,
            ready: None,
            no_capture,
        }
    }
}

pub(crate) fn build(app: &mut bevy_app::App) {
    let edge = RING_SIZE.unsigned_abs();
    app.register_debug_texture(DebugTexture::render(
        "surface.trail",
        "trail ring (current): R depression (m, <= 0), G lip (m, >= 0), B deformer ceiling code (m, <= 0; S1a; `tex show surface.trail b -0.5 0`), A press 0..1 (GC-T1; `tex show surface.trail a 0 1`), 1024 texels around the camera",
        (edge, edge),
        FORMAT,
        |world| world.get_resource::<TrailGpu>().map(|g| g.current_view().clone()),
    ));
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        shaders
            .insert(&TRAIL_SHADER, Shader::from_wgsl(include_str!("trail.wgsl"), "embedded://sf_surface_bevy/trail.wgsl"))
            .expect("uuid handles always insert");
    }
    let Some(render_app) = app.get_sub_app_mut(bevy_render::RenderApp) else { return };
    render_app
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<TrailGpu>)
        .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
        .add_systems(
            RenderGraph,
            trail_pass.in_set(TrailPassSystems).in_set(RenderGraphSystems::Render).before(camera_driver),
        );
}

/// Publishes [`TrailViews`] after [`extract`] flipped the rings (so it binds what
/// `render::extract_view` binds), with the uniform's placements.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn publish_views(
    mut commands: Commands,
    gpu: Option<Res<TrailGpu>>,
    uniform: Extract<Res<SurfaceUniform>>,
    views: Option<ResMut<TrailViews>>,
) {
    let Some(gpu) = gpu else { return };
    let next = TrailViews {
        current: gpu.current_view().clone(),
        previous: gpu.previous_view().clone(),
        placement: TrailPlacement { place: uniform.trail, cells: uniform.trail_cells },
        previous_placement: TrailPlacement { place: uniform.trail_prev, cells: uniform.trail_prev_cells },
    };
    match views {
        Some(mut views) => *views = next,
        None => commands.insert_resource(next),
    }
}

/// Decides this frame's pass and flips the rings (before `render::extract_view` binds the
/// current one). A capture to merge is work (S1a: a standing leg re-writes its hole each frame
/// against the refill).
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
    if !frame.has_work(gpu.held) {
        return;
    }
    let (capture, image) = frame.capture.clone().map_or((None, None), |(c, i)| (Some(c), Some(i)));
    let pass = TrailPass {
        corner: frame.corner,
        depth_m: frame.depth_m,
        new_min: frame.new_min,
        old_min: gpu.held,
        refill_m: frame.refill_m,
        stamps: frame.stamps.clone(),
        capture,
        press: frame.press,
    };
    gpu.current ^= 1;
    gpu.held = Some(frame.new_min);
    gpu.pending = Some((pass, image));
}

#[allow(clippy::needless_pass_by_value)]
fn prepare(gpu: Option<ResMut<TrailGpu>>, device: Res<RenderDevice>, cache: Res<PipelineCache>, images: Res<RenderAssets<GpuImage>>) {
    let Some(mut gpu) = gpu else { return };
    let Some((mut pass, image)) = gpu.pending.take() else { return };
    // The capture's target not on the GPU yet: no capture this pass.
    let target = image.and_then(|i| images.get(i)).map(|g| g.texture_view.clone());
    if target.is_none() {
        pass.capture = None;
    }
    let bytes: Vec<u8> = pass.slots().iter().flatten().flat_map(|w| w.to_le_bytes()).collect();
    let params = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("surface_trail_params"),
        contents: &bytes,
        usage: BufferUsages::UNIFORM,
    });
    let ground_bytes: Vec<u8> = pass.ground_words().iter().flat_map(|w| w.to_le_bytes()).collect();
    let ground = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("surface_trail_ground"),
        contents: &ground_bytes,
        usage: BufferUsages::STORAGE,
    });
    let (cur, prev) = (gpu.current, gpu.current ^ 1);
    let capture = target.as_ref().unwrap_or(&gpu.no_capture);
    let group = device.create_bind_group(
        "surface_trail",
        &cache.get_bind_group_layout(&gpu.layout),
        &BindGroupEntries::sequential((
            params.as_entire_binding(),
            &gpu.sampled[prev],
            &gpu.storage[cur],
            capture,
            ground.as_entire_binding(),
        )),
    );
    gpu.ready = Some(group);
}

/// The ring pass, once per frame, before any camera renders (root `RenderGraph`, before
/// `camera_driver`): the terrain, the mesh plants and the blades all read this frame's ring.
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
