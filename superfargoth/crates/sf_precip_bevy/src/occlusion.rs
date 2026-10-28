//! The precipitation occlusion map (weather.md "Precipitation", Occlusion):
//! one orthographic depth map around the camera, looking along the drops'
//! travel direction (a directional shadow map from the "precipitation
//! light"), that the drop shader samples to kill drops with something
//! upstream of them: roofs and trees for rain, buildings and hills upwind for
//! a blowing ash storm. Rain falling straight down is the degenerate case, a
//! top-down view. The view is never shallower than [`MIN_ELEVATION`] (a lee
//! shelter of about 3.7 obstacle heights, not a 15:1 line behind every bump).
//!
//! Render side: [`sf_sky_bevy::sun_view`] (no pass of our own): on a
//! redraw frame the map becomes one more shadow view of a shadow-casting
//! directional light, with our own view matrix, so Bevy's shadow pass draws
//! the casters (alpha mask included). Redrawn when the camera moved
//! [`RECENTER_M`] horizontally, [`RECENTER_Y_M`] vertically, the travel
//! direction turned [`REDIRECT_DEG`], or casters streamed in; steady frames
//! cost nothing. Far proxies carry [`NoPrecipOcclusion`] (hosts add it);
//! terrain must stay in (hills shelter their lee from a storm).

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};

use bevy_app::{App, PreUpdate, Startup};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::InheritedVisibility;
use bevy_camera::{Camera, Camera3d};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_light::{DirectionalLight, NotShadowCaster};
use bevy_math::{Mat4, Vec3, Vec4};
use bevy_mesh::Mesh3d;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_transform::components::GlobalTransform;
use sf_sky_bevy::sun_view::{add_sun_view, SunViewRequest, SunViewSource};

use crate::{velocity, Precip};

/// Subview index among the light's subviews (cascades use 0..4).
const SUBVIEW: u32 = 0xFB;
/// Map edge in texels.
pub const MAP_SIZE: u32 = 512;
/// Half the covered square, meters: the far drop box (64 m) seen from any
/// direction spans at most 32·√2 ≈ 45 m.
pub const HALF_M: f32 = 48.0;
/// Depth range upstream (toward the source) and downstream of the camera,
/// meters. Downstream covers the far box's corner (32·√3 ≈ 55 m).
pub const UPWIND_M: f32 = 100.0;
pub const DOWNWIND_M: f32 = 60.0;
/// Shallowest view, radians below the horizon (15°).
pub const MIN_ELEVATION: f32 = 0.261_799_4;
/// Recenter distances, meters.
pub const RECENTER_M: f32 = 6.0;
pub const RECENTER_Y_M: f32 = 8.0;
/// Turn of the travel direction that redraws, degrees.
pub const REDIRECT_DEG: f32 = 3.0;
/// Frames between redraws forced by streamed-in casters.
const REFRESH_FRAMES: u32 = 30;
/// Frames between redraws forced by a turning wind.
const REDIRECT_FRAMES: u32 = 8;

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

/// The direction the map looks along for drops moving at `velocity`: the
/// travel direction, lowered to at least [`MIN_ELEVATION`] below the
/// horizon; straight down when the drops don't move sideways.
pub fn view_dir(velocity: Vec3) -> Vec3 {
    let (Some(d), Some(h)) = (velocity.try_normalize(), velocity.with_y(0.0).try_normalize()) else {
        return Vec3::NEG_Y;
    };
    let (sin_min, cos_min) = MIN_ELEVATION.sin_cos();
    if -d.y >= sin_min { d } else { h * cos_min - Vec3::Y * sin_min }
}

/// The view direction of the strongest falling slot, if any.
pub fn travel(precip: &Precip) -> Option<Vec3> {
    precip
        .slots
        .iter()
        .flatten()
        .filter(|s| precip.enabled && s.intensity > 0.0)
        .max_by(|a, b| a.intensity.total_cmp(&b.intensity))
        .map(|s| view_dir(velocity(s, precip.wind)))
}

/// The view along `dir` (from [`view_dir`]) centered on `center`:
/// `(view_from_world, clip_from_world)`. The eye sits [`UPWIND_M`] upstream.
pub fn along(center: Vec3, dir: Vec3) -> (Mat4, Mat4) {
    let eye = center - dir * UPWIND_M;
    // `view_dir` keeps `dir` at least 15° below the horizon: never parallel to -Z.
    let view = look_to_mat4(eye, dir, Vec3::NEG_Z);
    // Reverse Z (near/far swapped): 1 at the upstream plane, 0 downstream.
    let proj = orthographic(-HALF_M, HALF_M, -HALF_M, HALF_M, UPWIND_M + DOWNWIND_M, 0.0);
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
    precip: Res<Precip>,
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
    mut last_dir: Local<Option<Vec3>>,
) {
    frame.0 = None;
    since.0 |= !added.is_empty();
    since.1 = since.1.saturating_add(1);
    let Some(mut out) = out else { return };
    if !config.0 {
        out.params = Vec4::ZERO;
        *last = None;
        *last_dir = None;
        return;
    }
    let Some(dir) = travel(&precip) else {
        out.params = Vec4::ZERO;
        *last = None;
        *last_dir = None;
        return;
    };
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
    let turned = since.1 >= REDIRECT_FRAMES && last_dir.is_none_or(|l| l.dot(dir) < REDIRECT_DEG.to_radians().cos());
    if !moved && !streamed && !turned {
        return;
    }
    *since = (false, 0);
    let center = if moved { pos } else { last.unwrap_or(pos) };
    *last = Some(center);
    *last_dir = Some(dir);
    let (view_from_world, clip_from_world) = along(center, dir);
    let reach = HALF_M * core::f32::consts::SQRT_2;
    let range = UPWIND_M + DOWNWIND_M;
    let mut found: Vec<(Entity, MainEntity)> = casters
        .iter()
        .filter(|(_, _, _, visible, _)| visible.get())
        .filter(|(_, tf, aabb, _, _)| {
            let (c, radius) = aabb.map_or((tf.translation(), 0.0), |b| (tf.transform_point(b.center.into()), tf.radius_vec3a(b.half_extents)));
            let p = view_from_world.transform_point3(c);
            // The view looks down -Z: in range while 0 <= -z <= range.
            p.x.abs() <= reach + radius && p.y.abs() <= reach + radius && p.z <= radius && -p.z <= range + radius
        })
        .map(|(e, _, _, _, render)| (render.map_or(Entity::PLACEHOLDER, |r| **r), MainEntity::from(e)))
        .collect();
    found.sort_unstable();
    stats.redraws += 1;
    stats.casters = found.len();
    out.clip_from_world = clip_from_world;
    out.params = Vec4::new(1.0, range, 0.0, 0.0);
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

