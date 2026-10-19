//! The precipitation occlusion map (weather.md "Precipitation", Occlusion):
//! one top-down orthographic depth map around the camera that the drop shader
//! samples to kill streaks under roofs and trees.
//!
//! Same technique as `sf_sky_bevy`'s far sun map (no pass of our own): on a
//! redraw frame the map becomes one more shadow view of a shadow-casting
//! directional light, looking straight down, so Bevy's shadow pass draws the
//! casters (alpha mask included). Redrawn when the camera moved
//! [`RECENTER_M`] horizontally, [`RECENTER_Y_M`] vertically, or casters
//! streamed in; steady frames cost nothing. Terrain and far proxies carry
//! [`NoPrecipOcclusion`] (hosts add it): the ground is not an occluder.

use core::any::TypeId;

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};

use bevy_app::{App, PreUpdate, Startup};
use bevy_asset::{AssetId, Assets, Handle, RenderAssetUsages};
use bevy_camera::primitives::{Aabb, Frustum};
use bevy_camera::visibility::{InheritedVisibility, RenderLayers};
use bevy_camera::{Camera, Camera3d};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemChangeTick;
use bevy_image::{Image, ImageSampler};
use bevy_light::{DirectionalLight, NotShadowCaster};
use bevy_math::{Dir3, Mat4, UVec4, Vec3, Vec4};
use bevy_mesh::Mesh3d;
use bevy_pbr::{prepare_lights, ExtractedDirectionalLight, LightEntity, Shadow, ShadowView, ViewLightEntities};
use bevy_render::batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_phase::ViewBinnedRenderPhases;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_render::texture::{DepthStencilViewAttachment, DepthStencilViews, GpuImage};
use bevy_render::view::{
    collect_visible_cpu_culled_entities, ExtractedView, NoIndirectDrawing, RenderExtractedShadowMapVisibleEntities,
    RenderExtractedVisibleEntities, RenderExtractedVisibleEntitiesClass, RenderShadowMapVisibleEntities, RenderVisibleEntities,
    RetainedViewEntity,
};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shape::{HalfSpace, ViewFrustum};
use bevy_transform::components::GlobalTransform;

/// Subview index among the light's subviews (cascades 0..4, far sun map 0xFA).
const SUBVIEW: u32 = 0xFB;
/// Map edge in texels.
pub const MAP_SIZE: u32 = 512;
/// Half the covered square, meters (the largest drop box is 32 m).
pub const HALF_M: f32 = 40.0;
/// Depth range above and below the camera, meters.
pub const ABOVE_M: f32 = 60.0;
pub const BELOW_M: f32 = 40.0;
/// Recenter distances, meters.
pub const RECENTER_M: f32 = 6.0;
pub const RECENTER_Y_M: f32 = 8.0;
/// Frames between redraws forced by streamed-in casters.
const REFRESH_FRAMES: u32 = 30;

/// Keeps a shadow-casting mesh out of the occlusion map (terrain, far proxies).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NoPrecipOcclusion;

/// `false` turns the map off (rain falls through roofs).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrecipOcclusionConfig(pub bool);

impl Default for PrecipOcclusionConfig {
    fn default() -> Self {
        Self(true)
    }
}

/// What the drop shader samples: the `Depth32Float` map (reverse Z: 1 at the
/// top, 0 empty), `clip_from_world`, params `(valid, depth range m, 0, 0)`.
#[derive(Resource, Clone, Debug)]
pub struct PrecipOcclusion {
    pub image: Handle<Image>,
    pub clip_from_world: Mat4,
    pub params: Vec4,
}

/// Counters for the status tool.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PrecipOcclusionStats {
    pub redraws: u64,
    pub casters: usize,
}

#[derive(Resource, Clone, Debug, Default)]
struct Frame(Option<Redraw>);

#[derive(Clone, Debug)]
struct Redraw {
    view_from_world: Mat4,
    clip_from_world: Mat4,
    light: Entity,
    camera: Entity,
    image: AssetId<Image>,
    casters: Vec<(Entity, MainEntity)>,
}

impl Redraw {
    fn retained(&self) -> RetainedViewEntity {
        RetainedViewEntity::new(MainEntity::from(self.light), Some(MainEntity::from(self.camera)), SUBVIEW)
    }
}

/// The top-down view centered on `center`: `(view_from_world, clip_from_world)`.
pub fn top_down(center: Vec3) -> (Mat4, Mat4) {
    let eye = Vec3::new(center.x, center.y + ABOVE_M, center.z);
    let view = look_to_mat4(eye, Dir3::NEG_Y.as_vec3(), Vec3::NEG_Z);
    // Reverse Z (near/far swapped): 1 at the top plane, 0 at the bottom.
    let proj = orthographic(-HALF_M, HALF_M, -HALF_M, HALF_M, ABOVE_M + BELOW_M, 0.0);
    (view, proj * view)
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<PrecipOcclusionConfig>()
        .init_resource::<PrecipOcclusionStats>()
        .init_resource::<Frame>()
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, update);
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app.init_resource::<Frame>().add_systems(ExtractSchedule, extract).add_systems(
        Render,
        (
            inject_casters.in_set(RenderSystems::PrepareAssets).before(collect_visible_cpu_culled_entities),
            prepare_view.in_set(RenderSystems::CreateViews).after(prepare_lights),
        ),
    );
}

fn map_image(size: u32) -> Image {
    let mut image = Image::new_uninit(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Depth32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    image.sampler = ImageSampler::nearest();
    image
}

pub(crate) fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = images.add(map_image(MAP_SIZE));
    commands.insert_resource(PrecipOcclusion { image, clip_from_world: Mat4::IDENTITY, params: Vec4::ZERO });
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn update(
    config: Res<PrecipOcclusionConfig>,
    mut frame: ResMut<Frame>,
    mut stats: ResMut<PrecipOcclusionStats>,
    out: Option<ResMut<PrecipOcclusion>>,
    lights: Query<(Entity, &DirectionalLight)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera3d>>,
    casters: Query<
        (Entity, &GlobalTransform, Option<&Aabb>, &InheritedVisibility, Option<&RenderEntity>),
        (With<Mesh3d>, Without<NotShadowCaster>, Without<NoPrecipOcclusion>),
    >,
    added: Query<(), (Added<Mesh3d>, Without<NotShadowCaster>, Without<NoPrecipOcclusion>)>,
    mut since: Local<(bool, u32)>,
    mut last: Local<Option<Vec3>>,
) {
    frame.0 = None;
    since.0 |= !added.is_empty();
    since.1 = since.1.saturating_add(1);
    let Some(mut out) = out else { return };
    if !config.0 {
        out.params = Vec4::ZERO;
        return;
    }
    let light = lights.iter().filter(|(_, l)| l.shadow_maps_enabled).max_by(|a, b| a.1.illuminance.total_cmp(&b.1.illuminance));
    let cam = cameras.iter().filter(|(_, c, _)| c.is_active).min_by_key(|(_, c, _)| c.order);
    let (Some((light, _)), Some((camera, _, camera_tf))) = (light, cam) else {
        out.params = Vec4::ZERO;
        *last = None;
        return;
    };
    let pos = camera_tf.translation();
    let moved = last.is_none_or(|l| (l - pos).with_y(0.0).length() > RECENTER_M || (l.y - pos.y).abs() > RECENTER_Y_M);
    let streamed = since.0 && since.1 >= REFRESH_FRAMES;
    if !moved && !streamed {
        return;
    }
    *since = (false, 0);
    let center = if moved { pos } else { last.unwrap_or(pos) };
    *last = Some(center);
    let (view_from_world, clip_from_world) = top_down(center);
    let reach = HALF_M * core::f32::consts::SQRT_2;
    let mut found: Vec<(Entity, MainEntity)> = casters
        .iter()
        .filter(|(_, _, _, visible, _)| visible.get())
        .filter(|(_, tf, aabb, _, _)| {
            let (c, radius) = aabb.map_or((tf.translation(), 0.0), |b| (tf.transform_point(b.center.into()), tf.radius_vec3a(b.half_extents)));
            let p = view_from_world.transform_point3(c);
            p.x.abs() <= reach + radius && p.y.abs() <= reach + radius
        })
        .map(|(e, _, _, _, render)| (render.map_or(Entity::PLACEHOLDER, |r| **r), MainEntity::from(e)))
        .collect();
    found.sort_unstable();
    stats.redraws += 1;
    stats.casters = found.len();
    out.clip_from_world = clip_from_world;
    out.params = Vec4::new(1.0, ABOVE_M + BELOW_M, 0.0, 0.0);
    frame.0 = Some(Redraw { view_from_world, clip_from_world, light, camera, image: out.image.id(), casters: found });
}

fn extract(mut commands: Commands, frame: Extract<Res<Frame>>) {
    commands.insert_resource(Frame(frame.0.clone()));
}

fn inject_casters(
    frame: Res<Frame>,
    ticks: SystemChangeTick,
    mut lights: Query<
        (&MainEntity, &mut RenderShadowMapVisibleEntities, &mut RenderExtractedShadowMapVisibleEntities),
        With<ExtractedDirectionalLight>,
    >,
) {
    let Some(redraw) = &frame.0 else { return };
    for (main, mut visible, mut extracted) in &mut lights {
        if main.id() != redraw.light {
            continue;
        }
        let key = redraw.retained();
        visible.subviews.insert(key, (ticks.this_run(), RenderVisibleEntities::default()));
        let mut list = RenderExtractedVisibleEntities::default();
        list.classes.insert(TypeId::of::<Mesh3d>(), RenderExtractedVisibleEntitiesClass { entities: redraw.casters.clone() });
        extracted.subviews.insert(key, list);
    }
}

#[allow(clippy::too_many_arguments)]
fn prepare_view(
    mut commands: Commands,
    frame: Res<Frame>,
    images: Res<RenderAssets<GpuImage>>,
    lights: Query<(Entity, &MainEntity, Option<&RenderLayers>), With<ExtractedDirectionalLight>>,
    mut views: Query<(&MainEntity, &mut ViewLightEntities, Has<NoIndirectDrawing>)>,
    mut phases: ResMut<ViewBinnedRenderPhases<Shadow>>,
    support: Res<GpuPreprocessingSupport>,
    mut spawned: Local<Option<Entity>>,
) {
    if let Some(old) = spawned.take() {
        commands.entity(old).despawn();
    }
    let Some(redraw) = &frame.0 else { return };
    let Some(gpu) = images.get(redraw.image) else { return };
    let Some((light, _, layers)) = lights.iter().find(|(_, m, _)| m.id() == redraw.light) else { return };
    let Some((_, mut view_lights, no_indirect)) = views.iter_mut().find(|(m, _, _)| m.id() == redraw.camera) else { return };
    let mode = support.min(if no_indirect { GpuPreprocessingMode::PreprocessingOnly } else { GpuPreprocessingMode::Culling });
    let key = redraw.retained();
    let world_from_view = redraw.view_from_world.inverse();
    let mut frustum = ViewFrustum::from_clip_from_world(&redraw.clip_from_world);
    // Casters above the top plane still cast (like Bevy's cascades).
    let near = frustum.half_spaces[ViewFrustum::NEAR_PLANE_IDX].normal();
    frustum.half_spaces[ViewFrustum::NEAR_PLANE_IDX] = HalfSpace::new(near.extend(f32::INFINITY));
    let depth_attachment =
        DepthStencilViewAttachment::new(DepthStencilViews::DepthOnly { depth_view: gpu.texture_view.clone() }, Some(0.0), None);
    let mut entity = commands.spawn((
        ShadowView { depth_attachment, pass_name: "precip_occlusion".into() },
        ExtractedView {
            retained_view_entity: key,
            viewport: UVec4::new(0, 0, MAP_SIZE, MAP_SIZE),
            world_from_view: GlobalTransform::from(world_from_view),
            clip_from_view: redraw.clip_from_world * world_from_view,
            clip_from_world: Some(redraw.clip_from_world),
            target_format: bevy_pbr::CORE_3D_SHADOW_MAP_FORMAT,
            color_grading: bevy_render::view::ColorGrading::default(),
            invert_culling: false,
        },
        Frustum(frustum),
        LightEntity::Directional { light_entity: light, cascade_index: 0 },
        layers.cloned().unwrap_or_default(),
    ));
    if mode != GpuPreprocessingMode::Culling {
        entity.insert(NoIndirectDrawing);
    }
    view_lights.lights.push(entity.id());
    phases.prepare_for_new_frame(key, mode);
    *spawned = Some(entity.id());
}
