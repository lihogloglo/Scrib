//! The shelter and canopy maps (surface-weather.md "GPU resources"): vertical depth maps of
//! everything above, 1024² over 128 m (0.125 m texels) around the camera, that the layer
//! compares against to tell open sky from cover. The shelter map holds solid casters (roofs,
//! arches, rocks: full shelter); the canopy map holds the casters the game marks
//! [`ShelterFoliage`] (leaves: partial shelter, [`SHELTER_LEAF_PASS`] of the rain gets through).
//!
//! Canonical technique: the top-down precipitation occlusion depth map (a shadow map from a
//! view straight above), used for rain splashes in Remember Me and for wet masks and ripples
//! in S.T.A.L.K.E.R. Clear Sky; Clockwork's `surfaceRoof`. Foliage as a second map, split on
//! the CPU by a marker, instead of Clockwork's checkerboard (leaves on even texels of one map,
//! a foliage flag read in the depth shader): no material flag in Bevy's shadow pass, full roof
//! resolution, and the layer reads leaf cover as a plain fraction.
//!
//! Render side: two extra views of Bevy's own shadow pass (`sf_sky_bevy::sun_view`), owned by
//! a [`DepthViewAnchor`](sf_sky_bevy::sun_view::DepthViewAnchor) of this plugin, not by the
//! sun: they redraw at night and whatever the sky does with its shadows (W2b; W2 rode the sun
//! and kept a stale map while it had none). The views are vertical and world-stable, unlike
//! the precipitation map, which follows the wind and would move wet patches as it turns.
//!
//! Redrawn ([`ShelterTrigger`]) when the camera moved 16 m horizontally or 8 m vertically,
//! when casters streamed in or got their foliage mark (at most every 30 frames) and on a
//! floating-origin shift; steady frames cost nothing. Only while a layer is on or a debug view
//! shows. Skinned meshes (characters) and [`NoSurfaceShelter`] entities (the game tags terrain
//! patches: a heightfield shelters nothing from straight above) are not casters.
//! `SurfaceSettings::shelter` off (Low preset, `--off shelter`): no maps, exposure 1
//! everywhere.

use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_light::NotShadowCaster;
use bevy_math::{Mat4, Vec4};
use bevy_mesh::Mesh3d;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_transform::components::GlobalTransform;
use sf_diag_bevy::DiagCounters;
use sf_sky_bevy::sun_view::{SunViewRequest, SunViewSource};
use sf_surface::view::{
    DebugView, SHELTER_DOWN_M, SHELTER_HALF_M, SHELTER_LEAF_PASS, SHELTER_SIZE, SHELTER_TEXEL_M,
    SHELTER_UP_M, ShelterTrigger, shelter_view,
};
use crate::cells::camera_position;
use crate::{
    SurfaceGpu, SurfaceOrigin, SurfaceSettings, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera,
};

/// Subview indices among the anchor's subviews.
pub const SHELTER_SUBVIEW: u32 = 0xFC;
pub const CANOPY_SUBVIEW: u32 = 0xFD;

/// Keeps a shadow-casting mesh out of the shelter and canopy maps.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NoSurfaceShelter;

/// Marks a leaf mesh (the game decides from its content): it draws into the canopy map and
/// shelters partially instead of fully.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShelterFoliage;

/// The anchor entity owning both views (spawned by the plugin).
#[derive(Resource, Clone, Copy, Debug)]
pub(crate) struct ShelterAnchor(pub Entity);

/// This frame's shelter redraw request, both worlds.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct ShelterFrame(pub Option<SunViewRequest>);

impl SunViewSource for ShelterFrame {
    const PASS_NAME: &'static str = "surface_shelter";
    fn request(&self) -> Option<&SunViewRequest> {
        self.0.as_ref()
    }
}

/// This frame's canopy redraw request (same frames as the shelter), both worlds.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct CanopyFrame(pub Option<SunViewRequest>);

impl SunViewSource for CanopyFrame {
    const PASS_NAME: &'static str = "surface_canopy";
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

type Caster<'a> = (
    Entity,
    &'a GlobalTransform,
    Option<&'a Aabb>,
    &'a InheritedVisibility,
    Option<&'a RenderEntity>,
    Has<ShelterFoliage>,
);

/// A [`Caster`] query item.
type CasterItem<'a> = (
    Entity,
    &'a GlobalTransform,
    Option<&'a Aabb>,
    &'a InheritedVisibility,
    Option<&'a RenderEntity>,
    bool,
);

/// The redraw's casters as `(render, main)` pairs, solid and foliage, sorted.
type Split = (Vec<(Entity, MainEntity)>, Vec<(Entity, MainEntity)>);

fn split_casters<'a>(casters: impl Iterator<Item = CasterItem<'a>>, view_from_world: Mat4) -> Split {
    let range = SHELTER_UP_M + SHELTER_DOWN_M;
    let (mut solid, mut leaves) = (Vec::new(), Vec::new());
    for (e, tf, aabb, visible, render, foliage) in casters {
        if !visible.get() {
            continue;
        }
        let (c, radius) = aabb.map_or((tf.translation(), 0.0), |b| {
            (
                tf.transform_point(b.center.into()),
                tf.radius_vec3a(b.half_extents),
            )
        });
        let p = view_from_world.transform_point3(c);
        // Looking down −Z from the top plane: anything above it still casts (depth clamp).
        if p.x.abs() > SHELTER_HALF_M + radius
            || p.y.abs() > SHELTER_HALF_M + radius
            || -p.z > range + radius
        {
            continue;
        }
        let pair = (
            render.map_or(Entity::PLACEHOLDER, |r| **r),
            MainEntity::from(e),
        );
        if foliage {
            leaves.push(pair);
        } else {
            solid.push(pair);
        }
    }
    solid.sort_unstable();
    leaves.sort_unstable();
    (solid, leaves)
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn update_shelter(
    mut frame: ResMut<ShelterFrame>,
    mut canopy: ResMut<CanopyFrame>,
    mut uniform: ResMut<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
    gpu: Res<SurfaceGpu>,
    anchor: Res<ShelterAnchor>,
    origin: Res<SurfaceOrigin>,
    settings: Res<SurfaceSettings>,
    cameras: Query<(&Camera, &GlobalTransform), With<SurfaceWeatherCamera>>,
    camera_entities: Query<(Entity, &Camera), With<SurfaceWeatherCamera>>,
    casters: Query<Caster, CasterFilter>,
    added: Query<
        (),
        (
            Or<(Added<Mesh3d>, Added<ShelterFoliage>)>,
            Without<NotShadowCaster>,
            Without<NoSurfaceShelter>,
            Without<SkinnedMesh>,
        ),
    >,
    mut shelter: Local<ShelterState>,
    counters: Option<Res<DiagCounters>>,
) {
    frame.0 = None;
    canopy.0 = None;
    let active = settings.shelter && (uniform.flags != 0 || uniform.debug != DebugView::Off);
    let (true, Some(cam)) = (active, camera_position(&cameras)) else {
        uniform.shelter = Vec4::ZERO;
        uniform.canopy = Vec4::ZERO;
        shelter.trigger.reset();
        return;
    };
    if shelter.origin != Some(origin.0) {
        // Render space moved under the maps: draw them again where they now are.
        shelter.origin = Some(origin.0);
        shelter.trigger.reset();
        uniform.shelter = Vec4::ZERO;
        uniform.canopy = Vec4::ZERO;
    }
    let camera = camera_entities
        .iter()
        .filter(|(_, c)| c.is_active)
        .min_by_key(|(_, c)| c.order)
        .map(|(e, _)| e);
    let Some(camera) = camera else {
        return;
    };
    if shelter.trigger.frame(cam, !added.is_empty()).is_none() {
        return;
    }
    let center = shelter.trigger.center().expect("set by a redraw");
    let (view_from_world, clip_from_world) = shelter_view(center);
    let (solid, leaves) = split_casters(casters.iter(), view_from_world);
    stats.shelter_redraws += 1;
    stats.shelter_casters = solid.len();
    stats.canopy_casters = leaves.len();
    if let Some(c) = counters {
        c.add_named("surface/shelter_redraws", "count", 1.0);
    }
    let range = SHELTER_UP_M + SHELTER_DOWN_M;
    uniform.shelter_clip_from_world = clip_from_world;
    uniform.shelter = Vec4::new(1.0, range, SHELTER_TEXEL_M, SHELTER_HALF_M);
    uniform.canopy = Vec4::new(1.0, SHELTER_LEAF_PASS, 0.0, 0.0);
    let request = |subview, image: &Handle<Image>, casters| SunViewRequest {
        light: anchor.0,
        camera,
        subview,
        image: image.id(),
        size: SHELTER_SIZE,
        view_from_world,
        clip_from_world,
        casters,
    };
    frame.0 = Some(request(SHELTER_SUBVIEW, &gpu.shelter, solid));
    canopy.0 = Some(request(CANOPY_SUBVIEW, &gpu.canopy, leaves));
}

/// Creates a map's image.
pub(crate) fn shelter_handle(images: &mut Assets<Image>) -> Handle<Image> {
    images.add(shelter_image())
}

/// Spawns the views' anchor.
pub(crate) fn spawn_anchor(commands: &mut Commands) {
    let e = commands
        .spawn(sf_sky_bevy::sun_view::depth_view_anchor())
        .id();
    commands.insert_resource(ShelterAnchor(e));
}
