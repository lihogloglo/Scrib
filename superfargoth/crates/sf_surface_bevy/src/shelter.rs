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
//!
//! A redraw's casters are made visible ([`mark_casters_visible`], in Bevy's `CheckVisibility`):
//! Bevy extracts a mesh only while some view sees it (`ViewVisibility`), so a roof behind the
//! camera and outside the sun's cascades used to drop out of the redraw, and the cover under it
//! changed at the next redraw (SW-5/SW-6 "resets"). A view owning its casters' visibility is
//! Bevy's own pattern (each light's frusta mark their casters the same way).
//!
//! Camera- and centre-independent (SW-5d, owner 2026-11-16: trees gained or lost shelter at
//! each redraw): every caster draws its level visible at a fixed LOD distance,
//! [`SHELTER_LOD_DISTANCE_M`] = 0, i.e. its *finest* level, wherever the camera and the map
//! centre stand ([`split_casters`]); coarser levels and far proxies (ranges starting beyond 0)
//! never cast. The GPU agrees: the views resolve every visibility range at that distance,
//! undithered (`bevy_render::view::RenderViewLodDistances`, vendored patch "SW-5d: fixed
//! per-view LOD distance"; Bevy otherwise culls and dithers a camera-tied shadow view's levels
//! by the camera). So a world texel holds the same depth after any redraw, the standard for a
//! top-down occlusion capture of a static world: fixed content LOD, world-snapped texels; LOD
//! changes density for the main view only. (SW-5 picked levels by the map centre: the centre
//! jumps 16 m per redraw, so a tree 30-60 m out swapped sparse and dense leaves between
//! redraws.) The centre snaps to the world texel grid (`shelter_center_world`) and what the
//! maps say is eased in over ~2.4 s by the exposure pack (`crate::exposure`), so the remaining
//! changes (a caster streaming in, the edge fade moving) fade instead of popping. The uniform
//! switches to a redraw's clip the frame after its request, when the pack takes the drawn maps
//! in (`ExposureClock`).
//!
//! Far level (SW-5e, owner 2026-11-16: snow under trees still sank as the camera neared): the
//! raised cover stands out to the near-field handoff, but the near maps know shelter only within
//! ±64 m, so a tree 70-360 m out had full raised snow under it until the near map reached it. Now
//! a second pair of maps, the same capture over ±512 m at 1 m texels ([`ShelterFarSettings`],
//! `sf_surface::view::ShelterLevel::far`), knows it out there, as a stable shadow cascade: its
//! centre snaps to its own world texel grid, every caster draws at the level it shows at a fixed
//! coarser distance (150 m, [`crate::SHELTER_FAR_LOD_DISTANCE_M`]; still a near level, never a
//! proxy), and it redraws when the camera moved 64 m or casters streamed in (at most every 120
//! frames). Its pack eases like the near one; the layer blends near into far over the near map's
//! outer band and far into open sky over its own, and the terrain's raise ends where the far
//! level stops being known ([`ShelterReach`]).

use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{
    InheritedVisibility, RenderLayers, SetViewVisibility, ViewVisibility, VisibilityRange,
};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_light::NotShadowCaster;
use bevy_math::{Mat4, Vec3, Vec4};
use bevy_mesh::Mesh3d;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_time::{Real, Time};
use bevy_transform::components::GlobalTransform;
use sf_diag_bevy::DiagCounters;
use sf_sky_bevy::sun_view::{SunViewRequest, SunViewSource};
use sf_surface::view::{
    DebugView, ExposureClock, ExposureMode, ExposurePass, NEAR_LEVEL, SHELTER_LEAF_PASS, SHELTER_SIZE, ShelterLevel,
    ShelterTrigger, outer_uv_map,
};
use crate::cells::camera_position;
use crate::exposure::ExposureFrame;
use crate::{
    ShelterFarSettings, ShelterReach, SurfaceGpu, SurfaceOrigin, SurfaceSettings, SurfaceStats, SurfaceUniform,
    SurfaceWeatherCamera,
};

/// Subview indices among the anchor's subviews.
pub const SHELTER_SUBVIEW: u32 = 0xFC;
pub const CANOPY_SUBVIEW: u32 = 0xFD;
/// The far level's maps (SW-5e).
pub const FAR_SHELTER_SUBVIEW: u32 = 0xFA;
pub const FAR_CANOPY_SUBVIEW: u32 = 0xFB;

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

/// This frame's far shelter redraw request (SW-5e), both worlds.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct FarShelterFrame(pub Option<SunViewRequest>);

impl SunViewSource for FarShelterFrame {
    const PASS_NAME: &'static str = "surface_shelter_far";
    fn request(&self) -> Option<&SunViewRequest> {
        self.0.as_ref()
    }
}

/// This frame's far canopy redraw request (same frames as the far shelter), both worlds.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct FarCanopyFrame(pub Option<SunViewRequest>);

impl SunViewSource for FarCanopyFrame {
    const PASS_NAME: &'static str = "surface_canopy_far";
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

/// One level's trigger state across frames.
#[derive(Default)]
pub(crate) struct LevelState {
    trigger: ShelterTrigger,
    origin: Option<sf_coords::WorldPos>,
    /// Which exposure pass runs (`crate::exposure`), and the held map's world texel.
    clock: ExposureClock,
    /// The clip of the redraw requested last frame: the uniform takes it with its pass.
    pending_clip: Option<Mat4>,
}

/// Both levels' state across frames.
#[derive(Default)]
pub(crate) struct ShelterState {
    near: LevelState,
    far: LevelState,
}

/// One level's slots in the uniform: its clip, its `shelter` and its `canopy` vec4s.
pub(crate) struct LevelUniform<'a> {
    pub(crate) clip: &'a mut Mat4,
    pub(crate) shelter: &'a mut Vec4,
    pub(crate) canopy: &'a mut Vec4,
}

impl LevelUniform<'_> {
    fn off(&mut self) {
        *self.shelter = Vec4::ZERO;
        *self.canopy = Vec4::ZERO;
    }
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
    Option<&'a RenderLayers>,
    Has<ShelterFoliage>,
    Option<&'a VisibilityRange>,
);

/// A [`Caster`] query item.
type CasterItem<'a> = (
    Entity,
    &'a GlobalTransform,
    Option<&'a Aabb>,
    &'a InheritedVisibility,
    Option<&'a RenderEntity>,
    Option<&'a RenderLayers>,
    bool,
    Option<&'a VisibilityRange>,
);

/// The redraw's casters as `(render, main)` pairs, solid and foliage, sorted.
type Split = (Vec<(Entity, MainEntity)>, Vec<(Entity, MainEntity)>);

/// The LOD distance the shelter and canopy maps resolve every caster's visibility range at
/// (`RenderViewLodDistances`): 0 m, each caster's finest level.
// tunables: const (a definition, not a trade-off: the finest level is the one visible at 0 m)
pub const SHELTER_LOD_DISTANCE_M: f32 = 0.0;

/// Whether a caster with `range` (a LOD level; `None`: always) draws into a map that resolves
/// levels at the fixed `lod_distance`: Bevy's own rule (`VisibilityRange::is_visible_at_all`)
/// at that distance, whatever the caster's, the camera's or the map centre's position, as the
/// GPU does for these views (`RenderViewLodDistances`). Shared with the precipitation map's
/// twin (`sf_precip_bevy::occlusion::lod_casts`).
pub(crate) fn lod_casts(range: Option<&VisibilityRange>, lod_distance: f32) -> bool {
    range.is_none_or(|r| r.is_visible_at_all(lod_distance))
}

/// Only what `camera_layers` sees casts, like the view (`sun_view` gives an anchor's view the
/// camera's layers): other worldspaces' casters drop here, not in Bevy's shadow queue. A LOD
/// level casts when its range holds the maps' fixed `lod_distance` ([`lod_casts`]), never by
/// the camera's or the centre's distance. `level` bounds the square and the depth range.
pub(crate) fn split_casters<'a>(
    casters: impl Iterator<Item = CasterItem<'a>>,
    view_from_world: Mat4,
    lod_distance: f32,
    camera_layers: &RenderLayers,
    level: &ShelterLevel,
) -> Split {
    let layer_zero = RenderLayers::default();
    let range = level.depth_m();
    let half = level.half_m;
    let (mut solid, mut leaves) = (Vec::new(), Vec::new());
    for (e, tf, aabb, visible, render, layers, foliage, lod) in casters {
        if !visible.get() || !layers.unwrap_or(&layer_zero).intersects(camera_layers) {
            continue;
        }
        if !lod_casts(lod, lod_distance) {
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
        if p.x.abs() > half + radius || p.y.abs() > half + radius || -p.z > range + radius {
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

/// One level's frame (both levels run the same steps): off (`camera` `None`) resets it and clears
/// its uniform; a floating-origin shift keeps the held map where it is in the world; this frame's
/// exposure pass takes in last frame's redraw (the uniform switches to its clip with it); then,
/// with a camera entity to draw for (`drawable`), a redraw when the trigger says so, as its
/// `(view_from_world, clip_from_world)`. `test` is the layer's test for the level (`canopy.w`:
/// 0 the near tent, 1 the far bilinear).
#[allow(clippy::too_many_arguments)]
pub(crate) fn level_frame(
    state: &mut LevelState,
    level: &ShelterLevel,
    test: f32,
    origin: sf_coords::WorldPos,
    camera: Option<Vec3>,
    drawable: bool,
    dt: f32,
    streamed: bool,
    mut uniform: LevelUniform,
) -> (Option<ExposurePass>, Option<(Mat4, Mat4)>) {
    let Some(cam) = camera else {
        uniform.off();
        state.trigger.reset();
        state.clock.reset();
        state.pending_clip = None;
        return (None, None);
    };
    if state.origin != Some(origin) {
        // Render space moved under the maps: the held map stays where it is in the world (its
        // clip follows the shift, the pack is world-anchored), and a redraw draws it again
        // around the camera.
        if let Some(old) = state.origin {
            let shift = Mat4::from_translation(old.delta_f32(origin));
            *uniform.clip *= shift;
            if let Some(clip) = state.pending_clip.as_mut() {
                *clip *= shift;
            }
        }
        state.origin = Some(origin);
        state.trigger.reset();
    }
    // Last frame's redraw was drawn by last frame's render: this frame's pass takes it in, and
    // the uniform switches to its clip with it (every pass of a frame reads one map).
    let pass = state.clock.frame(dt);
    if let Some(p) = pass
        && p.mode != ExposureMode::Ease
        && let Some(clip) = state.pending_clip.take()
    {
        *uniform.clip = clip;
        *uniform.shelter = Vec4::new(1.0, level.depth_m(), level.texel_m(), level.half_m);
        // z: depth below the centre (the pack's heights are over the centre, `exposure.wgsl`).
        *uniform.canopy = Vec4::new(1.0, SHELTER_LEAF_PASS, level.down_m, test);
    }
    if !drawable || state.trigger.frame_level(level, origin.to_f64(), cam, streamed).is_none() {
        return (pass, None);
    }
    let center = state.trigger.center().expect("set by a redraw");
    let texel = state.trigger.center_texel().expect("set by a redraw");
    let (view_from_world, clip_from_world) = level.view(center);
    state.clock.redraw(texel);
    state.pending_clip = Some(clip_from_world);
    (pass, Some((view_from_world, clip_from_world)))
}

#[allow(clippy::too_many_arguments, clippy::type_complexity, clippy::too_many_lines)]
pub(crate) fn update_shelter(
    mut frames: (ResMut<ShelterFrame>, ResMut<CanopyFrame>, ResMut<FarShelterFrame>, ResMut<FarCanopyFrame>),
    mut exposure: ResMut<ExposureFrame>,
    mut reach: ResMut<ShelterReach>,
    time: Option<Res<Time<Real>>>,
    mut uniform: ResMut<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
    gpu: Res<SurfaceGpu>,
    anchor: Res<ShelterAnchor>,
    origin: Res<SurfaceOrigin>,
    settings: (Res<SurfaceSettings>, Res<ShelterFarSettings>),
    cameras: Query<(&Camera, &GlobalTransform), With<SurfaceWeatherCamera>>,
    camera_entities: Query<(Entity, &Camera, Option<&RenderLayers>), With<SurfaceWeatherCamera>>,
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
    let (frame, canopy, far_frame, far_canopy) = (&mut frames.0, &mut frames.1, &mut frames.2, &mut frames.3);
    let (settings, far_settings) = (&*settings.0, &*settings.1);
    frame.0 = None;
    canopy.0 = None;
    far_frame.0 = None;
    far_canopy.0 = None;
    *exposure = ExposureFrame::default();
    let now_reach = ShelterReach::of(settings, far_settings);
    if *reach != now_reach {
        *reach = now_reach;
    }
    let active = settings.shelter && (uniform.flags != 0 || uniform.debug != DebugView::Off);
    let cam = camera_position(&cameras).filter(|_| active);
    let far_cam = cam.filter(|_| far_settings.on);
    let camera = camera_entities
        .iter()
        .filter(|(_, c, _)| c.is_active)
        .min_by_key(|(_, c, _)| c.order)
        .map(|(e, _, layers)| (e, layers.cloned().unwrap_or_default()));
    let dt = time.map_or(0.0, |t| t.delta_secs());
    let streamed = !added.is_empty();
    let near_level = NEAR_LEVEL;
    let far_level = far_settings.level();
    // The far level first: the near pass reads the far pack as this frame holds it.
    let u = &mut *uniform;
    let (far_pass, far_redraw) = level_frame(
        &mut shelter.far,
        &far_level,
        1.0,
        origin.0,
        far_cam,
        camera.is_some(),
        dt,
        streamed,
        LevelUniform { clip: &mut u.far_clip_from_world, shelter: &mut u.far_shelter, canopy: &mut u.far_canopy },
    );
    let (pass, near_redraw) = level_frame(
        &mut shelter.near,
        &near_level,
        0.0,
        origin.0,
        cam,
        camera.is_some(),
        dt,
        streamed,
        LevelUniform { clip: &mut u.shelter_clip_from_world, shelter: &mut u.shelter, canopy: &mut u.canopy },
    );
    exposure.pass = pass;
    exposure.far_pass = far_pass;
    exposure.far = far_settings.on.then_some(far_level);
    exposure.far_lod_distance = far_settings.lod_distance_m;
    // Only a carry reads the far pack (what the band and the newly covered texels showed).
    if pass.is_some_and(|p| matches!(p.mode, ExposureMode::Carry(_))) && u.far_shelter.x > 0.0 {
        exposure.outer = Some(outer_uv_map(u.shelter_clip_from_world, near_level.half_m, u.far_clip_from_world, far_level.half_m));
    }
    let Some((camera, camera_layers)) = camera else {
        return;
    };
    let request = |subview, image: &Handle<Image>, view: (Mat4, Mat4), casters| SunViewRequest {
        light: anchor.0,
        camera,
        subview,
        image: image.id(),
        size: SHELTER_SIZE,
        view_from_world: view.0,
        clip_from_world: view.1,
        casters,
    };
    if let Some(view) = far_redraw {
        let (solid, leaves) = split_casters(casters.iter(), view.0, far_settings.lod_distance_m, &camera_layers, &far_level);
        stats.far_redraws += 1;
        stats.far_casters = solid.len();
        stats.far_canopy_casters = leaves.len();
        if let Some(c) = counters.as_deref() {
            c.add_named("surface/shelter_far_redraws", "count", 1.0);
        }
        far_frame.0 = Some(request(FAR_SHELTER_SUBVIEW, &gpu.far_shelter, view, solid));
        far_canopy.0 = Some(request(FAR_CANOPY_SUBVIEW, &gpu.far_canopy, view, leaves));
    }
    if let Some(view) = near_redraw {
        let (solid, leaves) = split_casters(casters.iter(), view.0, SHELTER_LOD_DISTANCE_M, &camera_layers, &near_level);
        stats.shelter_redraws += 1;
        stats.shelter_casters = solid.len();
        stats.canopy_casters = leaves.len();
        if let Some(c) = counters.as_deref() {
            c.add_named("surface/shelter_redraws", "count", 1.0);
        }
        frame.0 = Some(request(SHELTER_SUBVIEW, &gpu.shelter, view, solid));
        canopy.0 = Some(request(CANOPY_SUBVIEW, &gpu.canopy, view, leaves));
    }
}

/// `VisibilitySystems::CheckVisibility`, after the redraw is chosen (`SurfaceSystems`): this
/// frame's shelter and canopy casters (both levels) are visible to a view (the maps), so Bevy
/// extracts them even when no camera or light frustum holds them. The lists hold only each
/// caster's level at the map's fixed LOD distance ([`split_casters`]), whatever the camera sees.
/// The precipitation map's twin is `sf_precip_bevy::occlusion::mark_casters_visible`.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn mark_casters_visible(
    shelter: Res<ShelterFrame>,
    canopy: Res<CanopyFrame>,
    far_shelter: Res<FarShelterFrame>,
    far_canopy: Res<FarCanopyFrame>,
    mut casters: Query<&mut ViewVisibility>,
) {
    let requests = shelter.0.iter().chain(canopy.0.iter()).chain(far_shelter.0.iter()).chain(far_canopy.0.iter());
    for req in requests {
        for (_, main) in &req.casters {
            if let Ok(mut visibility) = casters.get_mut(main.id()) {
                visibility.set_visible();
            }
        }
    }
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
