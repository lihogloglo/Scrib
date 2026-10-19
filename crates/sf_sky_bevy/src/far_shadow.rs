//! The cached far static sun shadow map (shadows F3b, weather.md "Sun
//! cascades and the far static map"): one sun-aligned orthographic depth
//! map around the camera (± 2 km, 1 m texels) that receivers sample past
//! the last cascade. CPU policy (where the map sits, when it is redrawn):
//! [`sf_sky::FarShadowCache`].
//!
//! Render side: no pass of our own. On a redraw frame the map becomes one
//! more shadow view of the shadow-casting directional light, so Bevy's own
//! shadow pass draws it, with its material-specialised shadow pipelines
//! (alpha mask included), GPU preprocessing and batching, through the
//! shared [`crate::sun_view`] mechanism (casters injected as one more subview
//! of the light, the view spawned after `prepare_lights`, both dropped when
//! the light has no shadow maps by render time, e.g. in interiors).
//!
//! Between redraws the view doesn't exist, so steady frames cost nothing
//! (Bevy drops its bins; each redraw re-queues the casters as a new view).
//! One map, redrawn whole in one frame (64 MB at 4096²): strips into a
//! single sampled map would mix two views, and a back map doubles the VRAM.
//!
//! Casters: every shadow-casting mesh in the map's footprint except
//! [`NoFarShadow`] (hosts put it on terrain, which has its own far term).
//! Receivers sample [`FarSunShadow`] (depth texture + comparison sampler,
//! `clip_from_world`, params).

use bevy_app::{App, Plugin, PreUpdate, Startup};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageCompareFunction, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_light::{DirectionalLight, NotShadowCaster};
use bevy_math::{Mat4, Vec4};
use bevy_mesh::Mesh3d;
use bevy_pbr::Shadow;
use bevy_render::render_phase::ViewBinnedRenderPhases;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_render::{Render, RenderApp, RenderSystems};
use bevy_transform::components::GlobalTransform;
use sf_sky::{FarShadowCache, FarShadowSettings, FarShadowWork};

use crate::sun_view::{add_sun_view, SunViewRequest, SunViewSource};
use crate::{MoonLight, SkyCamera, SunLight};

/// Subview index of the far map among the light's subviews (cascades use 0..4).
const SUBVIEW: u32 = 0xFA;

/// Frames between redraws forced by newly spawned casters (streaming).
const REFRESH_FRAMES: u32 = 30;

/// The far map's settings; `None` turns it off (receivers keep the cascades
/// and their own far terms). `strips` is forced to 1 (one map, whole redraws).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct FarShadowConfig(pub Option<FarShadowSettings>);

impl Default for FarShadowConfig {
    fn default() -> Self {
        Self(Some(FarShadowSettings::default()))
    }
}

/// Keeps a shadow-casting mesh out of the far map (terrain, which receivers
/// shadow with their own horizon term).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NoFarShadow;

/// What receivers sample: a `Depth32Float` map (reverse Z, comparison
/// sampler `GreaterEqual`: 1 = lit), the view it was drawn with, and
/// params `(valid 0/1, texel m, depth range m, redraw count)`.
#[derive(Resource, Clone, Debug)]
pub struct FarSunShadow {
    pub image: Handle<Image>,
    pub clip_from_world: Mat4,
    pub params: Vec4,
}

/// Counters for benches and the status tool.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FarShadowStats {
    /// Redraws so far.
    pub redraws: u64,
    /// Casters in the last redraw.
    pub casters: usize,
}

/// This frame's redraw, handed to the render world.
#[derive(Resource, Clone, Debug, Default)]
struct FarShadowFrame(Option<SunViewRequest>);

impl SunViewSource for FarShadowFrame {
    const PASS_NAME: &'static str = "far_static_sun_shadow";
    fn request(&self) -> Option<&SunViewRequest> {
        self.0.as_ref()
    }
}

#[derive(Resource, Default)]
struct FarShadowState {
    cache: FarShadowCache,
    settings: Option<FarShadowSettings>,
}

/// The far static sun shadow map. Add after `SkyPlugin`.
pub struct FarShadowPlugin;

impl Plugin for FarShadowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FarShadowConfig>()
            .init_resource::<FarShadowStats>()
            .init_resource::<FarShadowState>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, update);
        add_sun_view::<FarShadowFrame>(app);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app.add_systems(Render, report.in_set(RenderSystems::PhaseSort));
    }
}

fn map_image(size: u32) -> Image {
    let mut image = Image::new_uninit(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Depth32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        label: Some("far_sun_shadow".into()),
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        compare: Some(ImageCompareFunction::GreaterEqual),
        ..ImageSamplerDescriptor::default()
    });
    image
}

fn setup(mut commands: Commands, config: Res<FarShadowConfig>, mut images: ResMut<Assets<Image>>) {
    // A 1² stand-in while off, so receivers always have a depth texture to bind.
    let size = config.0.map_or(1, |s| s.resolution);
    let image = images.add(map_image(size));
    commands.insert_resource(FarSunShadow { image, clip_from_world: Mat4::IDENTITY, params: Vec4::ZERO });
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn update(
    config: Res<FarShadowConfig>,
    mut state: ResMut<FarShadowState>,
    mut frame: ResMut<FarShadowFrame>,
    mut counters: ResMut<FarShadowStats>,
    out: Option<ResMut<FarSunShadow>>,
    mut images: ResMut<Assets<Image>>,
    lights: Query<(Entity, &DirectionalLight, &GlobalTransform), Or<(With<SunLight>, With<MoonLight>)>>,
    cameras: Query<(Entity, &GlobalTransform), With<SkyCamera>>,
    casters: Query<
        (Entity, &GlobalTransform, Option<&Aabb>, &InheritedVisibility, Option<&RenderEntity>),
        (With<Mesh3d>, Without<NotShadowCaster>, Without<NoFarShadow>),
    >,
    added: Query<(), (Added<Mesh3d>, Without<NotShadowCaster>, Without<NoFarShadow>)>,
    mut since: Local<(bool, u32)>,
) {
    frame.0 = None;
    // Streamed-in casters: redraw, at most every REFRESH_FRAMES while cells keep arriving.
    since.0 |= !added.is_empty();
    since.1 = since.1.saturating_add(1);
    if since.0 && since.1 >= REFRESH_FRAMES {
        state.cache.invalidate();
        *since = (false, 0);
    }
    let Some(mut out) = out else { return };
    let settings = config.0.map(|s| FarShadowSettings { strips: 1, ..s });
    if settings != state.settings {
        // New size or switched: fresh map, fresh cache.
        let size = settings.map_or(1, |s| s.resolution);
        out.image = images.add(map_image(size));
        out.params = Vec4::ZERO;
        state.cache = FarShadowCache::default();
        state.settings = settings;
    }
    let Some(settings) = settings else { return };
    let light = lights
        .iter()
        .filter(|(_, l, _)| l.shadow_maps_enabled && l.illuminance > 0.0)
        .max_by(|a, b| a.1.illuminance.total_cmp(&b.1.illuminance));
    let (Some((light, _, light_tf)), Some((camera, camera_tf))) = (light, cameras.iter().next()) else {
        out.params = Vec4::ZERO;
        return;
    };
    let FarShadowWork::Strip { view, .. } = state.cache.update(light_tf.back().as_vec3(), camera_tf.translation(), &settings) else {
        return;
    };
    // Casters whose bounds reach into the map's footprint.
    let reach = settings.reach;
    let mut list: Vec<(Entity, MainEntity)> = casters
        .iter()
        .filter(|(_, _, _, visible, _)| visible.get())
        .filter(|(_, tf, aabb, _, _)| {
            let (centre, radius) = aabb.map_or((tf.translation(), 0.0), |b| (tf.transform_point(b.center.into()), tf.radius_vec3a(b.half_extents)));
            let p = view.view_from_world.transform_point3(centre);
            p.x.abs() <= reach + radius && p.y.abs() <= reach + radius
        })
        .map(|(e, _, _, _, render)| (render.map_or(Entity::PLACEHOLDER, |r| **r), MainEntity::from(e)))
        .collect();
    list.sort_unstable();
    counters.redraws += 1;
    counters.casters = list.len();
    out.clip_from_world = view.clip_from_world;
    out.params = Vec4::new(1.0, settings.texel(), settings.depth_range, 0.0);
    frame.0 = Some(SunViewRequest {
        light,
        camera,
        subview: SUBVIEW,
        image: out.image.id(),
        size: settings.resolution,
        view_from_world: view.view_from_world,
        clip_from_world: view.clip_from_world,
        casters: list,
    });
}

/// Logs a redraw once its casters are queued: the casters handed to Bevy and
/// the bins its shadow pass draws (no bins: nothing drawn).
fn report(frame: Res<FarShadowFrame>, phases: Res<ViewBinnedRenderPhases<Shadow>>, mut count: Local<u64>) {
    let Some(redraw) = &frame.0 else { return };
    *count += 1;
    let (sets, bins) = phases
        .get(&redraw.retained())
        .map_or((0, 0), |p| (p.multidrawable_meshes.len(), p.batchable_meshes.len() + p.unbatchable_meshes.len()));
    let casters = redraw.casters.len();
    if *count <= 4 || count.is_power_of_two() {
        log::info!("far sun shadow redraw {}: {casters} casters, {sets} multidraw sets + {bins} bins queued", *count);
    } else {
        log::debug!("far sun shadow redraw {}: {casters} casters, {sets} multidraw sets + {bins} bins queued", *count);
    }
}
