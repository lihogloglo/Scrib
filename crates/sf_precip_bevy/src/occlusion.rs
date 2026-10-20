//! The precipitation occlusion map (weather.md "Precipitation", Occlusion):
//! one top-down orthographic depth map around the camera that the drop shader
//! samples to kill streaks under roofs and trees.
//!
//! Render side: [`sf_sky_bevy::sun_view`] (no pass of our own): on a
//! redraw frame the map becomes one more shadow view of a shadow-casting
//! directional light, looking straight down, so Bevy's shadow pass draws the
//! casters (alpha mask included). Redrawn when the camera moved
//! [`RECENTER_M`] horizontally, [`RECENTER_Y_M`] vertically, or casters
//! streamed in; steady frames cost nothing. Terrain and far proxies carry
//! [`NoPrecipOcclusion`] (hosts add it): the ground is not an occluder.

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};

use bevy_app::{App, PreUpdate, Startup};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::InheritedVisibility;
use bevy_camera::{Camera, Camera3d};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_light::{DirectionalLight, NotShadowCaster};
use bevy_math::{Dir3, Mat4, Vec3, Vec4};
use bevy_mesh::Mesh3d;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_transform::components::GlobalTransform;
use sf_sky_bevy::sun_view::{add_sun_view, SunViewRequest, SunViewSource};

/// Subview index among the light's subviews (cascades use 0..4).
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
struct Frame(Option<SunViewRequest>);

impl SunViewSource for Frame {
    const PASS_NAME: &'static str = "precip_occlusion";
    fn request(&self) -> Option<&SunViewRequest> {
        self.0.as_ref()
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
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, update);
    add_sun_view::<Frame>(app);
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
    frame.0 = Some(SunViewRequest {
        light,
        camera,
        subview: SUBVIEW,
        image: out.image.id(),
        size: MAP_SIZE,
        view_from_world,
        clip_from_world,
        casters: found,
    });
}

