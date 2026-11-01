//! Render world: extraction, buffers, the `gi/probe_update` compute pass (`probe_update.wesl`).
//!
//! The pass binds Solari's scene bind group at group 0 (`RaytracingSceneBindings`, the TLAS,
//! geometry, materials and environment map; as `bevy_solari` `realtime/node.rs` does) and ours
//! at group 1: uniforms (with the sun), the Fibonacci directions, the per-probe irradiance and
//! visibility history, the two DDGI atlases (storage textures) and our light list. One workgroup
//! per probe of this frame's window, one ray per lane.
//!
//! The atlases (`sf_gi::AtlasLayout`) are render-world textures owned here. Once every probe has
//! been traced ([`GiTraced`]), `extract` gives the render-path cameras' views a
//! `bevy_pbr::GiProbeVolumeView` (our vendored `bevy_pbr` hunk, `superfargoth/vendor/README.md`):
//! the atlases, a linear clamp sampler and the volume uniform, which the PBR shader samples with
//! DDGI's visibility weights. Without a volume, the views lose it.
//!
//! The scene bind group exists whenever the TLAS has instances, lights or not (our `bevy_solari`
//! patch, `superfargoth/vendor/README.md`): Solari 0.20 has no point lights, and interiors no sun.
//! Direct light at hits comes from our list instead (`extract`: the active worldspace's point and
//! spot lights nearest the volume, and its sun), never from Solari's `directional_lights`. A
//! missing environment map is Solari's fallback cube at intensity 0 (black).

use std::collections::HashMap;
use std::sync::atomic::Ordering;

use bevy_app::App;
use bevy_asset::{load_embedded_asset, AssetId, AssetServer, Handle};
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_camera::visibility::{InheritedVisibility, RenderLayers};
use bevy_ecs::prelude::*;
use bevy_light::{DirectionalLight, PointLight, SpotLight};
use bevy_math::{Mat3, UVec3, UVec4, Vec3, Vec4};
use bevy_pbr::{GiProbeBindings, GiProbeVolumeUniform, GiProbeVolumeView};
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_mesh::Mesh;
use bevy_render::mesh::RenderMesh;
use bevy_render::render_asset::{prepare_assets, ExtractedAssets};
use bevy_render::render_resource::binding_types::{
    storage_buffer_read_only_sized, storage_buffer_sized, texture_storage_2d, uniform_buffer_sized,
};
use bevy_render::render_resource::{
    encase, AddressMode, BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, Buffer,
    BufferDescriptor, BufferInitDescriptor, BufferUsages, CachedComputePipelineId, CachedPipelineState,
    ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, FilterMode, PipelineCache, SamplerDescriptor, ShaderStages,
    StorageTextureAccess, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use bevy_render::renderer::{RenderContext, RenderDevice};
use bevy_render::sync_world::RenderEntity;
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use bevy_solari::scene::{RaytracingMesh3d, RaytracingSceneBindings};
use bevy_transform::components::GlobalTransform;
use sf_gi::{AtlasLayout, GiLight, GiSettings, LightTracker};
use sf_sky_bevy::RenderPathCamera;

use crate::{solari_verdict, GiSceneStats, GiTraced, GiVolume, GiVoxels, SceneCounts, Verdict};

/// Uniform block size (10 x vec4, `GiUniforms` in the shader).
const UNIFORM_BYTES: u64 = 160;
/// Both atlases: filterable, storage-writable (`Rg16Float` and `Rgb9e5Ufloat` are not core
/// storage formats).
const ATLAS_FORMAT: TextureFormat = TextureFormat::Rgba16Float;
const FLAG_SUN: u32 = 1;
const FLAG_BOUNCE: u32 = 2;

pub(crate) fn build(app: &mut App) {
    let traced = app.world().resource::<GiTraced>().clone();
    let stats = app.world().resource::<GiSceneStats>().clone();
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .insert_resource(traced)
        .insert_resource(stats)
        .init_resource::<MeshVerdicts>()
        .init_resource::<ExtractedGi>()
        .init_resource::<GiLightTracker>()
        .add_systems(
            RenderStartup,
            init_pipeline.after(bevy_render::init_gpu_resource::<RaytracingSceneBindings>),
        )
        .add_systems(ExtractSchedule, extract)
        .add_systems(
            Render,
            (
                classify_meshes.in_set(RenderSystems::PrepareAssets).before(prepare_assets::<RenderMesh>),
                prepare.in_set(RenderSystems::PrepareBindGroups),
                scene_stats.in_set(RenderSystems::PrepareBindGroups),
            ),
        )
        .add_systems(Core3d, probe_update.after(Core3dSystems::Prepass).before(Core3dSystems::MainPass));
}


/// Solari's BLAS verdict per mesh asset, classified from the extracted mesh data (the only
/// place it exists for render-world-only meshes), for the status log.
#[derive(Resource, Default)]
struct MeshVerdicts(HashMap<AssetId<Mesh>, Verdict>);

fn classify_meshes(extracted: Res<ExtractedAssets<RenderMesh>>, mut verdicts: ResMut<MeshVerdicts>) {
    for id in &extracted.removed {
        verdicts.0.remove(id);
    }
    for (id, mesh) in &extracted.extracted {
        if let Some(verdict) = solari_verdict(mesh) {
            verdicts.0.insert(*id, verdict);
        }
    }
}

/// This frame's scene counts for the main-world log.
fn scene_stats(
    meshes: Query<&RaytracingMesh3d>,
    verdicts: Res<MeshVerdicts>,
    scene: Res<RaytracingSceneBindings>,
    stats: Res<GiSceneStats>,
    extracted: Res<ExtractedGi>,
) {
    let mut counts = SceneCounts {
        accepted: scene.instance_count(),
        lights: u32::try_from(scene.light_count()).unwrap_or(u32::MAX),
        gi_lights: u32::try_from(extracted.lights.len()).unwrap_or(u32::MAX),
        gi_lights_changed: extracted.changed,
        sun: extracted.sun.is_some(),
        ..SceneCounts::default()
    };
    for mesh in &meshes {
        counts.mirrored += 1;
        match verdicts.0.get(&mesh.id()) {
            Some(Verdict::Compatible) => {}
            Some(Verdict::Disabled) => counts.disabled += 1,
            Some(Verdict::Attributes) => counts.attributes += 1,
            Some(Verdict::Topology) => counts.topology += 1,
            Some(Verdict::Indices) => counts.indices += 1,
            None => counts.unclassified += 1,
        }
    }
    if let Ok(mut shared) = stats.0.lock() {
        *shared = counts;
    }
}

#[derive(Clone, Copy)]
struct VolumeExtract {
    /// The main-world volume entity (a re-fit is a new entity: new atlases).
    entity: Entity,
    centre: Vec3,
    settings: GiSettings,
    bounce: bool,
    intensity: f32,
}

/// The first GI volume this frame (one per worldspace in this slice) and its lights.
#[derive(Resource, Default)]
struct ExtractedGi {
    volume: Option<VolumeExtract>,
    /// The nearest lights reaching the volume (`sf_gi::select_nearest`), plus lights gone since
    /// last frame (dark, changed) so their probes update fast.
    lights: Vec<GiLight>,
    /// Lights of `lights` that changed this frame (the gone ones included).
    changed: u32,
    /// Direction to the sun and its irradiance (linear colour x illuminance, lux).
    sun: Option<(Vec3, Vec3)>,
}

/// Each light's position and power last frame, by main-world entity.
#[derive(Resource, Default)]
struct GiLightTracker(LightTracker<Entity>);

type VolumeQuery<'w, 's> = Extract<'w, 's, Query<'static, 'static, (Entity, &'static GiVolume, &'static GlobalTransform), With<GiVoxels>>>;
type CameraQuery<'w, 's> =
    Extract<'w, 's, Query<'static, 'static, (&'static RenderEntity, Option<&'static RenderLayers>), With<RenderPathCamera>>>;
type LightQuery<'w, 's, L> =
    Extract<'w, 's, Query<'static, 'static, (Entity, &'static L, &'static GlobalTransform, &'static InheritedVisibility, Option<&'static RenderLayers>)>>;

/// The volume, plus the lights of the worldspace the render-path cameras see: point and spot
/// lights and the brightest directional light whose `RenderLayers` (none: layer 0) intersect the
/// cameras' (as `sf_sky_bevy` `mirror_meshes` filters meshes) and that are visible in the
/// hierarchy. `InheritedVisibility`, not `ViewVisibility`: Bevy culls a light whose range misses
/// the frustum, so lights behind the camera would drop out of a world-space volume as it turns.
#[allow(clippy::too_many_arguments)] // a system's parameters
fn extract(
    volumes: VolumeQuery,
    cameras: CameraQuery,
    pipeline: Option<Res<GiPipeline>>,
    traced: Res<GiTraced>,
    mut commands: Commands,
    points: LightQuery<PointLight>,
    spots: LightQuery<SpotLight>,
    suns: LightQuery<DirectionalLight>,
    mut tracker: ResMut<GiLightTracker>,
    mut out: ResMut<ExtractedGi>,
) {
    out.volume = volumes.iter().next().map(|(entity, v, transform)| VolumeExtract {
        entity,
        centre: transform.translation(),
        settings: v.settings,
        bounce: v.bounce,
        intensity: v.intensity,
    });
    // The views sample the volume once it was traced whole, with the atlases made for it.
    let bindings = pipeline.as_ref().and_then(|gi| gi.atlases.as_ref()).filter(|atlases| {
        traced.0.load(Ordering::Relaxed) && out.volume.is_some_and(|v| atlases.key == AtlasKey::of(&v))
    });
    for (render_entity, _) in &cameras {
        let Ok(mut view) = commands.get_entity(render_entity.id()) else { continue };
        match bindings {
            Some(atlases) => {
                view.try_insert(GiProbeVolumeView(atlases.bindings.clone()));
            }
            None => {
                view.try_remove::<GiProbeVolumeView>();
            }
        }
    }
    out.lights.clear();
    out.changed = 0;
    out.sun = None;
    let Some(volume) = out.volume else {
        return;
    };
    let view = cameras.iter().fold(RenderLayers::none(), |all, (_, layers)| all.union(layers.unwrap_or(&RenderLayers::default())));
    let seen = |visible: &InheritedVisibility, layers: Option<&RenderLayers>| {
        visible.get() && view.intersects(layers.unwrap_or(&RenderLayers::default()))
    };
    let linear = |c: bevy_color::Color| {
        let c = c.to_linear();
        Vec3::new(c.red, c.green, c.blue)
    };
    let tracker = &mut tracker.0;
    tracker.begin_frame();
    let mut lights = Vec::new();
    for (entity, light, transform, visible, layers) in &points {
        if !seen(visible, layers) {
            continue;
        }
        let position = transform.translation();
        let mut l = GiLight::point(position, linear(light.color), light.intensity, light.range, light.shadow_map_near_z);
        l.changed = tracker.observe(entity, position, light.intensity, light.range);
        lights.push(l);
    }
    for (entity, light, transform, visible, layers) in &spots {
        if !seen(visible, layers) {
            continue;
        }
        let position = transform.translation();
        let mut l = GiLight::point(position, linear(light.color), light.intensity, light.range, light.shadow_map_near_z)
            .with_spot(transform.forward().as_vec3(), light.inner_angle, light.outer_angle);
        l.changed = tracker.observe(entity, position, light.intensity, light.range);
        lights.push(l);
    }
    lights.extend(tracker.end_frame());
    let half = volume.settings.grid().extent() * 0.5;
    let max = usize::try_from(volume.settings.max_lights).unwrap_or(usize::MAX);
    sf_gi::select_nearest(&mut lights, volume.centre - half, volume.centre + half, max);
    out.changed = u32::try_from(lights.iter().filter(|l| l.changed).count()).unwrap_or(u32::MAX);
    out.lights = lights;
    out.sun = suns
        .iter()
        .filter(|(_, light, _, visible, layers)| light.illuminance > 0.0 && seen(visible, *layers))
        .max_by(|a, b| a.1.illuminance.total_cmp(&b.1.illuminance))
        .map(|(_, light, transform, _, _)| (transform.back().as_vec3(), linear(light.color) * light.illuminance));
}

/// What the atlases were made for: a new volume entity, grid or place makes new ones.
#[derive(Clone, Copy, PartialEq)]
struct AtlasKey {
    entity: Entity,
    dims: UVec3,
    spacing: f32,
    centre: Vec3,
}

impl AtlasKey {
    fn of(volume: &VolumeExtract) -> Self {
        Self { entity: volume.entity, dims: volume.settings.dims, spacing: volume.settings.spacing, centre: volume.centre }
    }
}

/// The volume's GPU state: both atlases, their interior-texel history and the views' bindings.
struct Atlases {
    key: AtlasKey,
    /// Irradiance (vec4, w > 0: has history) and visibility (vec2) per interior texel.
    irradiance_history: Buffer,
    visibility_history: Buffer,
    /// Storage views of the atlases' textures (the update pass writes them).
    irradiance_storage: bevy_render::render_resource::TextureView,
    visibility_storage: bevy_render::render_resource::TextureView,
    /// What the views bind (sampled views, sampler, volume uniform).
    bindings: GiProbeBindings,
}

#[derive(Resource)]
struct GiPipeline {
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
    frame: u32,
    atlases: Option<Atlases>,
    directions: Option<(u32, Buffer)>,
    /// Last logged pass status.
    status: String,
    /// Probes traced since the history was (re)created, saturating.
    covered: u32,
    /// This frame's dispatch: bind group, workgroups, probes in the grid.
    pending: Option<(BindGroup, u32, u32)>,
}

fn init_pipeline(
    mut commands: Commands,
    cache: Res<PipelineCache>,
    scene: Res<RaytracingSceneBindings>,
    assets: Res<AssetServer>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "gi_probe_update",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer_sized(false, std::num::NonZeroU64::new(UNIFORM_BYTES)),
                storage_buffer_read_only_sized(false, None),
                storage_buffer_sized(false, None),
                storage_buffer_sized(false, None),
                texture_storage_2d(ATLAS_FORMAT, StorageTextureAccess::WriteOnly),
                texture_storage_2d(ATLAS_FORMAT, StorageTextureAccess::WriteOnly),
                storage_buffer_read_only_sized(false, None),
            ),
        ),
    );
    let shader: Handle<Shader> = load_embedded_asset!(assets.as_ref(), "probe_update.wesl");
    let pipeline = cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("gi_probe_update".into()),
        layout: vec![scene.bind_group_layout.clone(), layout.clone()],
        shader,
        entry_point: Some("update".into()),
        ..ComputePipelineDescriptor::default()
    });
    commands.insert_resource(GiPipeline { layout, pipeline, frame: 0, atlases: None, directions: None, status: String::new(), covered: 0, pending: None });
}

/// New atlases (zeroed: no history, the first update takes its value as is) for `volume`.
fn make_atlases(device: &RenderDevice, volume: &VolumeExtract) -> Atlases {
    let grid = volume.settings.grid();
    let total = u64::from(grid.probe_count());
    let irradiance = AtlasLayout::irradiance(grid.dims);
    let visibility = AtlasLayout::visibility(grid.dims);
    let texture = |label: &'static str, layout: AtlasLayout| {
        let size = layout.size();
        device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d { width: size.x, height: size.y, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: ATLAS_FORMAT,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    };
    let irradiance_texture = texture("gi_irradiance_atlas", irradiance);
    let visibility_texture = texture("gi_visibility_atlas", visibility);
    let history = |label: &'static str, bytes: u64| {
        device.create_buffer(&BufferDescriptor { label: Some(label), size: bytes, usage: BufferUsages::STORAGE, mapped_at_creation: false })
    };
    let texels = |inner: u32| u64::from(inner * inner);
    let uniform = GiProbeVolumeUniform {
        origin_spacing: (volume.centre - grid.extent() * 0.5).extend(grid.spacing),
        dims: UVec4::new(grid.dims.x, grid.dims.y, grid.dims.z, sf_gi::IRRADIANCE_TEXELS),
        #[allow(clippy::cast_precision_loss)] // 14
        bias_intensity: Vec4::new(sf_gi::self_shadow_bias(grid.spacing), volume.intensity, sf_gi::VISIBILITY_TEXELS as f32, 0.0),
    };
    let mut bytes = encase::UniformBuffer::new(Vec::<u8>::new());
    bytes.write(&uniform).expect("GiProbeVolumeUniform encodes");
    let uniform = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("gi_probe_volume"),
        contents: bytes.as_ref(),
        usage: BufferUsages::UNIFORM,
    });
    let sampler = device.create_sampler(&SamplerDescriptor {
        label: Some("gi_probe_sampler"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        ..SamplerDescriptor::default()
    });
    let view = |t: &bevy_render::render_resource::Texture| t.create_view(&TextureViewDescriptor::default());
    Atlases {
        key: AtlasKey::of(volume),
        irradiance_history: history("gi_irradiance_history", total * texels(sf_gi::IRRADIANCE_TEXELS) * 16),
        visibility_history: history("gi_visibility_history", total * texels(sf_gi::VISIBILITY_TEXELS) * 8),
        irradiance_storage: view(&irradiance_texture),
        visibility_storage: view(&visibility_texture),
        bindings: GiProbeBindings { irradiance: view(&irradiance_texture), visibility: view(&visibility_texture), sampler, uniform },
    }
}

fn floats(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

#[allow(clippy::too_many_arguments)] // a system's parameters
fn prepare(
    extracted: Res<ExtractedGi>,
    pipeline: Option<ResMut<GiPipeline>>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    traced: Res<GiTraced>,
) {
    let Some(mut gi) = pipeline else { return };
    gi.pending = None;
    let Some(volume) = extracted.volume else {
        // No volume (none, or GI switched off): free the atlases.
        gi.atlases = None;
        return;
    };
    if cache.get_compute_pipeline(gi.pipeline).is_none() {
        return;
    }
    let grid = volume.settings.grid();
    let total = grid.probe_count();
    let rays = volume.settings.rays();
    if total == 0 {
        return;
    }
    if gi.atlases.as_ref().is_none_or(|a| a.key != AtlasKey::of(&volume)) {
        gi.atlases = Some(make_atlases(&device, &volume));
        gi.covered = 0;
        traced.0.store(false, Ordering::Relaxed);
    }
    if gi.directions.as_ref().is_none_or(|d| d.0 != rays) {
        let dirs: Vec<f32> = sf_gi::fibonacci_sphere(rays).iter().flat_map(|d| [d.x, d.y, d.z, 0.0]).collect();
        let buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("gi_directions"),
            contents: &floats(&dirs),
            usage: BufferUsages::STORAGE,
        });
        gi.directions = Some((rays, buffer));
    }
    gi.frame = gi.frame.wrapping_add(1);
    let (first, count) = sf_gi::frame_window(gi.frame, volume.settings.probes_per_frame, total);
    let rotation: Mat3 = sf_gi::frame_rotation(gi.frame);
    let origin = volume.centre - grid.extent() * 0.5;
    let (sun_dir, sun_e, sun_flag) = extracted.sun.map_or((Vec3::Y, Vec3::ZERO, 0), |(d, e)| (d, e, FLAG_SUN));
    let flags = sun_flag | if volume.bounce { FLAG_BOUNCE } else { 0 };
    let dims: UVec3 = grid.dims;
    let light_count = u32::try_from(extracted.lights.len()).unwrap_or(u32::MAX);
    let mut bytes = Vec::with_capacity(160);
    bytes.extend(floats(&[origin.x, origin.y, origin.z, grid.spacing]));
    bytes.extend([dims.x, dims.y, dims.z, rays].iter().flat_map(|v| v.to_le_bytes()));
    bytes.extend([first, count, total, flags].iter().flat_map(|v| v.to_le_bytes()));
    bytes.extend(floats(&[sun_dir.x, sun_dir.y, sun_dir.z, volume.settings.hysteresis.clamp(0.0, 1.0)]));
    bytes.extend(floats(&[sun_e.x, sun_e.y, sun_e.z, volume.settings.changed_hysteresis.clamp(0.0, 1.0)]));
    for c in [rotation.x_axis, rotation.y_axis, rotation.z_axis] {
        bytes.extend(floats(&[c.x, c.y, c.z, 0.0]));
    }
    bytes.extend([light_count, 0, 0, 0].iter().flat_map(|v| v.to_le_bytes()));
    bytes.extend(floats(&[sf_gi::max_ray_distance(grid.spacing), sf_gi::self_shadow_bias(grid.spacing), 0.0, 0.0]));
    debug_assert_eq!(bytes.len() as u64, UNIFORM_BYTES);
    let uniforms = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("gi_uniforms"),
        contents: &bytes,
        usage: BufferUsages::UNIFORM,
    });
    // At least one record: a storage binding can't be empty (the count says how many are real).
    let mut words: Vec<u32> = extracted.lights.iter().flat_map(GiLight::to_gpu).collect();
    if words.is_empty() {
        words.resize(sf_gi::GPU_LIGHT_WORDS, 0);
    }
    let light_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("gi_lights"),
        contents: &words.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>(),
        usage: BufferUsages::STORAGE,
    });
    let (Some(atlases), Some((_, directions))) = (&gi.atlases, &gi.directions) else { return };
    let group = device.create_bind_group(
        "gi_probe_update",
        &cache.get_bind_group_layout(&gi.layout),
        &BindGroupEntries::sequential((
            uniforms.as_entire_binding(),
            directions.as_entire_binding(),
            atlases.irradiance_history.as_entire_binding(),
            atlases.visibility_history.as_entire_binding(),
            &atlases.irradiance_storage,
            &atlases.visibility_storage,
            light_buffer.as_entire_binding(),
        )),
    );
    gi.pending = Some((group, count, total));
}

/// The dispatch with its GPU timer (`render/gi/probe_update/elapsed_gpu`); once per frame
/// (the first view to run takes it), after the prepass, before the main opaque pass.
fn probe_update(
    pipeline: Option<ResMut<GiPipeline>>,
    cache: Res<PipelineCache>,
    scene: Res<RaytracingSceneBindings>,
    traced: Res<GiTraced>,
    mut ctx: RenderContext,
) {
    let Some(mut gi) = pipeline else { return };
    // Logged on change: why the pass does (not) run.
    let status = match cache.get_compute_pipeline_state(gi.pipeline) {
        // Transient: the embedded shader isn't loaded yet; the cache retries.
        CachedPipelineState::Err(bevy_shader::ShaderCacheError::ShaderNotLoaded(_)) => {
            "waiting: shader loading".to_owned()
        }
        CachedPipelineState::Err(e) => format!("pipeline error: {e}"),
        CachedPipelineState::Ok(_) if scene.bind_group.is_none() => {
            "waiting: Solari scene empty (no RaytracingMesh3d with a BLAS yet)".to_owned()
        }
        CachedPipelineState::Ok(_) => "dispatching".to_owned(),
        _ => "waiting: pipeline compiling".to_owned(),
    };
    if gi.status != status && (gi.pending.is_some() || status != "dispatching") {
        if status.starts_with("pipeline error") {
            log::error!("gi: {status}");
        } else {
            log::info!("gi: {status}");
        }
        gi.status = status;
    }
    let (Some(compute), Some(scene_group)) = (cache.get_compute_pipeline(gi.pipeline), &scene.bind_group) else {
        return;
    };
    let Some((group, workgroups, total)) = gi.pending.take() else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx
        .command_encoder()
        .begin_compute_pass(&ComputePassDescriptor { label: Some("gi/probe_update"), timestamp_writes: None });
    let span = diagnostics.pass_span(&mut pass, "gi/probe_update");
    pass.set_pipeline(compute);
    pass.set_bind_group(0, scene_group, &[]);
    pass.set_bind_group(1, &group, &[]);
    pass.dispatch_workgroups(workgroups, 1, 1);
    span.end(&mut pass);
    // Shown (main world, `show_volumes`) once every probe has a first value.
    gi.covered = gi.covered.saturating_add(workgroups);
    if gi.covered >= total {
        traced.0.store(true, Ordering::Relaxed);
    }
}
