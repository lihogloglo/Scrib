//! The shelter map (surface-weather.md "GPU resources"): a vertical depth map of everything
//! above, 1024² over 128 m (0.125 m texels) around the camera, that the layer compares against
//! to tell open sky from cover (roofs, arches, trees).
//!
//! Canonical technique: the top-down precipitation occlusion depth map (a shadow map from a
//! light straight above), used for rain splashes in Remember Me and for wet masks and ripples
//! in S.T.A.L.K.E.R. Clear Sky; Clockwork's `surfaceRoof`. Render side: a second subview of
//! [`sf_sky_bevy::sun_view`] (as WE-5's precipitation map, `sf_precip_bevy/src/occlusion.rs`),
//! so Bevy's own shadow pass draws the casters (alpha mask included); no pass of our own. The
//! view is vertical and world-stable, unlike the precipitation map, which follows the wind and
//! would move wet patches as it turns.
//!
//! Redrawn ([`ShelterTrigger`]) when the camera moved 16 m horizontally or 8 m vertically,
//! when casters streamed in (at most every 30 frames) and on a floating-origin shift; steady
//! frames cost nothing. Only while a layer is on or a debug view shows. The view rides a
//! shadow-casting directional light: with none (the sky turns the sun's shadows off at night
//! and indoors) the last map stays until the light returns. Skinned meshes (characters) and
//! [`NoSurfaceShelter`] entities (the game tags terrain patches: a heightfield shelters nothing
//! from straight above) are not casters. `SurfaceSettings::shelter` off (Low preset, `--off
//! shelter`): no map, exposure 1 everywhere.

use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_light::{DirectionalLight, NotShadowCaster};
use bevy_math::Vec4;
use bevy_mesh::Mesh3d;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_transform::components::GlobalTransform;
use sf_diag_bevy::DiagCounters;
use sf_sky_bevy::sun_view::{SunViewRequest, SunViewSource};
use sf_surface::view::{
    DebugView, SHELTER_DOWN_M, SHELTER_HALF_M, SHELTER_SIZE, SHELTER_TEXEL_M, SHELTER_UP_M,
    ShelterTrigger, shelter_view,
};
use crate::cells::camera_position;
use crate::{
    SurfaceGpu, SurfaceOrigin, SurfaceSettings, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera,
};

/// Subview index among the light's subviews (cascades 0..4, precipitation 0xFB).
pub const SHELTER_SUBVIEW: u32 = 0xFC;

/// Keeps a shadow-casting mesh out of the shelter map.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NoSurfaceShelter;

/// This frame's redraw request, both worlds.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct ShelterFrame(pub Option<SunViewRequest>);

impl SunViewSource for ShelterFrame {
    const PASS_NAME: &'static str = "surface_shelter";
    fn request(&self) -> Option<&SunViewRequest> {
        self.0.as_ref()
    }
}

pub(crate) fn shelter_image() -> Image {
    let mut image = Image::new_uninit(
        Extent3d {
            width: SHELTER_SIZE,
            height: SHELTER_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        TextureFormat::Depth32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    image.sampler = ImageSampler::nearest();
    image
}

/// The shelter's trigger state across frames.
#[derive(Default)]
pub(crate) struct ShelterState {
    trigger: ShelterTrigger,
    origin: Option<sf_coords::WorldPos>,
}

type CasterFilter = (
    With<Mesh3d>,
    Without<NotShadowCaster>,
    Without<NoSurfaceShelter>,
    Without<SkinnedMesh>,
);

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn update_shelter(
    mut frame: ResMut<ShelterFrame>,
    mut uniform: ResMut<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
    gpu: Res<SurfaceGpu>,
    origin: Res<SurfaceOrigin>,
    settings: Res<SurfaceSettings>,
    lights: Query<(Entity, &DirectionalLight)>,
    cameras: Query<(&Camera, &GlobalTransform), With<SurfaceWeatherCamera>>,
    camera_entities: Query<(Entity, &Camera), With<SurfaceWeatherCamera>>,
    casters: Query<
        (
            Entity,
            &GlobalTransform,
            Option<&Aabb>,
            &InheritedVisibility,
            Option<&RenderEntity>,
        ),
        CasterFilter,
    >,
    added: Query<
        (),
        (
            Added<Mesh3d>,
            Without<NotShadowCaster>,
            Without<NoSurfaceShelter>,
            Without<SkinnedMesh>,
        ),
    >,
    mut shelter: Local<ShelterState>,
    counters: Option<Res<DiagCounters>>,
) {
    frame.0 = None;
    let active = settings.shelter && (uniform.flags != 0 || uniform.debug != DebugView::Off);
    let (true, Some(cam)) = (active, camera_position(&cameras)) else {
        uniform.shelter = Vec4::ZERO;
        shelter.trigger.reset();
        return;
    };
    if shelter.origin != Some(origin.0) {
        // Render space moved under the map: draw it again where it now is.
        shelter.origin = Some(origin.0);
        shelter.trigger.reset();
        uniform.shelter = Vec4::ZERO;
    }
    let light = lights
        .iter()
        .filter(|(_, l)| l.shadow_maps_enabled)
        .max_by(|a, b| a.1.illuminance.total_cmp(&b.1.illuminance));
    let camera = camera_entities
        .iter()
        .filter(|(_, c)| c.is_active)
        .min_by_key(|(_, c)| c.order)
        .map(|(e, _)| e);
    let (Some((light, _)), Some(camera)) = (light, camera) else {
        return;
    };
    if shelter.trigger.frame(cam, !added.is_empty()).is_none() {
        return;
    }
    let center = shelter.trigger.center().expect("set by a redraw");
    let (view_from_world, clip_from_world) = shelter_view(center);
    let range = SHELTER_UP_M + SHELTER_DOWN_M;
    let mut found: Vec<(Entity, MainEntity)> = casters
        .iter()
        .filter(|(_, _, _, visible, _)| visible.get())
        .filter(|(_, tf, aabb, _, _)| {
            let (c, radius) = aabb.map_or((tf.translation(), 0.0), |b| {
                (
                    tf.transform_point(b.center.into()),
                    tf.radius_vec3a(b.half_extents),
                )
            });
            let p = view_from_world.transform_point3(c);
            // Looking down −Z from the top plane: anything above it still casts (depth clamp).
            p.x.abs() <= SHELTER_HALF_M + radius
                && p.y.abs() <= SHELTER_HALF_M + radius
                && -p.z <= range + radius
        })
        .map(|(e, _, _, _, render)| {
            (
                render.map_or(Entity::PLACEHOLDER, |r| **r),
                MainEntity::from(e),
            )
        })
        .collect();
    found.sort_unstable();
    stats.shelter_redraws += 1;
    stats.shelter_casters = found.len();
    if let Some(c) = counters {
        c.add_named("surface/shelter_redraws", "count", 1.0);
    }
    uniform.shelter_clip_from_world = clip_from_world;
    uniform.shelter = Vec4::new(1.0, range, SHELTER_TEXEL_M, SHELTER_HALF_M);
    frame.0 = Some(SunViewRequest {
        light,
        camera,
        subview: SHELTER_SUBVIEW,
        image: gpu.shelter.id(),
        size: SHELTER_SIZE,
        view_from_world,
        clip_from_world,
        casters: found,
    });
}

/// Creates the map's image.
pub(crate) fn shelter_handle(images: &mut Assets<Image>) -> Handle<Image> {
    images.add(shelter_image())
}
