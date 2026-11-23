//! The camera medium on the Bevy side (water.md "Underwater", slice U1): each [`WaterCamera`]'s
//! [`WaterMedium`] (above, split or under the water; the body; the depth below the surface), the
//! [`WaterMediumCue`] messages for audio and gameplay, and the [`WaterMediumUniform`] the render
//! world gets on the camera's render entity.
//!
//! Slice U3 (the underwater post): a camera fully under the water also gets an
//! [`UnderwaterUniform`] (its body's optics, the light at the surface, its depth) and
//! [`bevy_pbr::SkyOccluded`] on its render entity, so the post draws (`render.rs`
//! `underwater_pass`) and the sky, the aerial perspective and the `sf_atmos` composite skip it
//! ([`post_on`]).
//!
//! The CPU water query belongs to the host (it steps it with its physics), so the host runs the
//! update: a system in [`crate::WaterSystems::Medium`] taking [`WaterMediums`] and calling
//! [`WaterMediums::update`] with its active worldspace's `sf_water::WaterQuery::sample`. The decision
//! itself is `sf_water::CameraMedium` (Bevy-free, tested there).

use bevy_app::{App, PostUpdate};
use bevy_camera::visibility::InheritedVisibility;
use bevy_camera::Projection;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_light::{AmbientLight, DirectionalLight, GlobalAmbientLight};
use bevy_math::{IVec3, Vec3, Vec4};
use bevy_pbr::SkyOccluded;
use bevy_render::render_resource::ShaderType;
use bevy_render::sync_world::RenderEntity;
use bevy_render::{Extract, ExtractSchedule, RenderApp};
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;
use sf_coords::WorldPos;
use sf_tunables::{def, Cost, Def, Effect, FeatureSwitch, Group, Source, Unit};
use sf_water::{optics_index_at, sun_term, CameraMedium, MediumState, NearPlane, WaterKind, WaterSample};

use crate::{WaterCamera, WaterLook, WaterRegionsInput, WaterSurface, WaterSystems};

/// `water.underwater`: track each water camera's medium (the underwater pass's switch; off = always above).
pub const WATER_UNDERWATER: Def<bool> = def!(bool "water.underwater" = true)
    .help("Track whether each water camera is above, split by or under the water; off = always above")
    .source(Source::Owner("2026-11-17: underwater pass, water.md \"Underwater\" Plan"))
    .group(Group::Water);

/// The feature switch on [`WATER_UNDERWATER`] (slice FS-2b; the host registers it beside the
/// tunable). Off: every camera is above, so the underwater post never runs.
pub const UNDERWATER: FeatureSwitch = FeatureSwitch::new(&WATER_UNDERWATER)
    .label("Underwater view")
    .effect(Effect::Skips)
    .cost(&[Cost::Gpu("water_underwater")])
    .at(file!(), line!());

/// `water.medium.hysteresis_m`: a near-plane point flips above or under only this far past the surface.
pub const WATER_MEDIUM_HYSTERESIS_M: Def<f32> = def!(float "water.medium.hysteresis_m" = 0.01)
    .range(0.0, 0.1)
    .unit(Unit::Meters)
    .help("The camera's medium flips only once a near-plane point is this far past the water surface")
    .source(Source::Industry("Crest underwater: 1 cm (water.md \"Underwater\")"))
    .group(Group::Water);

/// `water.underwater.kd_scale`: how fast the light dims with the camera's depth (U3 post).
pub const WATER_UNDERWATER_KD_SCALE: Def<f32> = def!(float "water.underwater.kd_scale" = 1.0)
    .range(0.25, 4.0)
    .unit(Unit::Ratio)
    .help("Under water, the surface light dims with the camera's depth as exp(-Kd z), Kd = this x the body's extinction")
    .source(Source::Industry("diffuse attenuation Kd ~ a + b_b (Mobley, Light and Water); Crest's underwater out-scattering by depth"))
    .group(Group::Water);

/// `water.underwater.sky_fill`: the sky's light under water, a share of the sun's (U3 post).
pub const WATER_UNDERWATER_SKY_FILL: Def<f32> = def!(float "water.underwater.sky_fill" = 0.25)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Under water, the sky's diffuse light as a share of the sun's (the environment map is not sampled by the post)")
    .source(Source::Guess)
    .group(Group::Water);

/// The uniform's stand-in for "no water here" and "bed unknown" depths (metres; ±∞ on the CPU).
pub const NO_WATER_DEPTH_M: f32 = 1.0e4; // tunables: const (format: a finite ∞ for the GPU)

/// The medium's settings (bound from [`WATER_UNDERWATER`], [`WATER_MEDIUM_HYSTERESIS_M`],
/// [`WATER_UNDERWATER_KD_SCALE`] and [`WATER_UNDERWATER_SKY_FILL`] by the host; live).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WaterMediumSettings {
    pub enabled: bool,
    pub hysteresis_m: f32,
    /// The underwater post (U3) on a camera fully under the water. Off (the host's `--off
    /// underwater`): the medium is still tracked, but the frame is drawn as above the water (sky,
    /// aerial perspective and the `sf_atmos` composite included: the old look, an A/B).
    pub post: bool,
    /// `Kd` = this × the body's extinction ([`WATER_UNDERWATER_KD_SCALE`]).
    pub kd_scale: f32,
    /// Sky light as a share of the sun's ([`WATER_UNDERWATER_SKY_FILL`]).
    pub sky_fill: f32,
}

impl Default for WaterMediumSettings {
    fn default() -> Self {
        Self {
            enabled: WATER_UNDERWATER.default_value(),
            hysteresis_m: WATER_MEDIUM_HYSTERESIS_M.default_value(),
            post: true,
            kd_scale: WATER_UNDERWATER_KD_SCALE.default_value(),
            sky_fill: WATER_UNDERWATER_SKY_FILL.default_value(),
        }
    }
}

/// One [`WaterCamera`]'s medium this frame (required by `WaterCamera`).
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct WaterMedium {
    /// State, body, depth below the surface at the near plane's centre, whether the state changed.
    pub medium: CameraMedium,
    /// The optics table index ([`WaterRegionsInput::looks`]) at the near plane's centre: the region's
    /// in the ocean, 0 in other bodies (their look is their `WaterSurface::look`) or with no map.
    pub optics: u32,
}

impl WaterMedium {
    #[must_use]
    pub fn state(&self) -> MediumState {
        self.medium.state
    }
}

/// A cue for audio and gameplay (water.md "Exposure and cues").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediumCue {
    /// `underwater`: the camera's eye went under (`true`) or came out (`false`); a low-pass follows it.
    Underwater { entered: bool },
    /// `surface_break`: the eye crossed the surface this frame (`downward` going in).
    SurfaceBreak { downward: bool },
}

/// A [`MediumCue`] of one water camera.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaterMediumCue {
    pub camera: Entity,
    pub cue: MediumCue,
}

/// What the render world reads (on the camera's render entity, extracted every frame). The
/// underwater post (U3) is gated on its `state` ([`post_on`]); the waterline (U4) will bind it.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct WaterMediumUniform {
    /// [`MediumState::as_u32`]: 0 above, 1 split, 2 under.
    pub state: u32,
    /// Optics table index ([`WaterMedium::optics`]).
    pub optics: u32,
    /// Metres below the surface at the near plane's centre (negative above; −[`NO_WATER_DEPTH_M`]: no water).
    pub depth: f32,
    /// Water column at the centre, metres ([`NO_WATER_DEPTH_M`]: bed unknown, a deep-water colour).
    pub column: f32,
}

impl From<&WaterMedium> for WaterMediumUniform {
    fn from(m: &WaterMedium) -> Self {
        Self {
            state: m.medium.state.as_u32(),
            optics: m.optics,
            depth: m.medium.depth.clamp(-NO_WATER_DEPTH_M, NO_WATER_DEPTH_M),
            column: m.medium.column.clamp(0.0, NO_WATER_DEPTH_M),
        }
    }
}

/// The underwater post's uniform (`water_underwater.wesl` `Underwater`; field order is the layout),
/// on the render entity of a camera the post draws for ([`post_on`]). The CPU reference of what
/// the shader does with it is `sf_water::underwater_colour`.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct UnderwaterUniform {
    /// The body's absorption per metre, linear RGB (w unused).
    pub absorption: Vec4,
    /// The body's scattering per metre, linear RGB (w unused).
    pub scattering: Vec4,
    /// Σ `sf_water::sun_term` over the directional lights (w unused).
    pub sun: Vec4,
    /// The view's ambient (camera `AmbientLight`, else `GlobalAmbientLight`) plus the sky fill × `sun`.
    pub ambient: Vec4,
    /// (camera depth below the surface m, 0 to [`NO_WATER_DEPTH_M`]; `Kd` scale; 0; 0).
    pub params: Vec4,
}

impl UnderwaterUniform {
    /// From the camera's medium, its body's look, the light at the surface and the settings.
    #[must_use]
    pub fn new(m: &WaterMedium, look: &WaterLook, sun: Vec3, ambient: Vec3, settings: &WaterMediumSettings) -> Self {
        let depth = if m.medium.depth > 0.0 { m.medium.depth.min(NO_WATER_DEPTH_M) } else { 0.0 };
        Self {
            absorption: look.absorption.extend(0.0),
            scattering: look.scattering.extend(0.0),
            sun: sun.extend(0.0),
            ambient: (ambient + sun * settings.sky_fill.max(0.0)).extend(0.0),
            params: Vec4::new(depth, settings.kd_scale.max(0.0), 0.0, 0.0),
        }
    }

    /// The same medium for the CPU reference (`sf_water::underwater_colour`).
    #[must_use]
    pub fn optics(&self) -> sf_water::UnderwaterOptics {
        sf_water::UnderwaterOptics { absorption: self.absorption.truncate(), scattering: self.scattering.truncate(), kd_scale: self.params.y }
    }
}

/// The underwater post draws for this camera: the post is on and the camera's medium is
/// [`MediumState::Under`] (uniform state 2). Split (1) is the waterline's (U4), above (0) nothing.
#[must_use]
pub fn post_on(settings: &WaterMediumSettings, medium: &WaterMediumUniform) -> bool {
    settings.post && medium.state == MediumState::Under.as_u32()
}

/// The light at the surface: Σ over the directional lights of `sf_water::sun_term` (illuminance ×
/// linear colour, toward the light = the light's +Z).
fn surface_sun<'a>(lights: impl Iterator<Item = (&'a DirectionalLight, &'a GlobalTransform)>) -> Vec3 {
    lights
        .map(|(l, t)| {
            let c = l.color.to_linear();
            sun_term(Vec3::new(c.red, c.green, c.blue) * l.illuminance, t.rotation() * Vec3::Z)
        })
        .sum()
}

/// The flat ambient that lights the camera's surfaces (`color × brightness`): the camera's own
/// [`AmbientLight`], else the [`GlobalAmbientLight`] (Bevy's rule; `sf_atmos_bevy::view_ambient`).
fn view_ambient(camera: Option<&AmbientLight>, global: Option<&GlobalAmbientLight>) -> Vec3 {
    let (color, brightness) = match (camera, global) {
        (Some(a), _) => (a.color, a.brightness),
        (None, Some(g)) => (g.color, g.brightness),
        (None, None) => return Vec3::ZERO,
    };
    let c = color.to_linear();
    Vec3::new(c.red, c.green, c.blue) * brightness
}

/// The look of the body the camera is in: in the ocean the optics map's entry at the near plane's
/// centre (nearest cell; the surface blends them) or the ocean's [`WaterLook`]; in another body the
/// visible [`WaterSurface`] whose bounds hold the eye (render space) and whose level is nearest the
/// surface above it (`eye.y` + the depth below it), else the ocean's.
fn body_look<'a>(m: &WaterMedium, eye: Vec3, ocean: &WaterLook, regions: &WaterRegionsInput, surfaces: impl Iterator<Item = (&'a WaterSurface, &'a InheritedVisibility)>) -> WaterLook {
    if m.medium.kind.is_none_or(|k| k == WaterKind::Ocean) {
        if regions.map.is_some() && !regions.looks.is_empty() {
            let i = (m.optics as usize).min(crate::MAX_LOOKS - 1).min(regions.looks.len() - 1);
            return regions.looks[i];
        }
        return *ocean;
    }
    let level = eye.y + m.medium.depth;
    let holds = |s: &WaterSurface| s.bounds.is_none_or(|[lo, hi]| eye.x >= lo.x && eye.x <= hi.x && eye.z >= lo.y && eye.z <= hi.y);
    surfaces
        .filter(|(s, v)| v.get() && holds(s))
        .min_by(|a, b| (a.0.level - level).abs().total_cmp(&(b.0.level - level).abs()))
        .map_or(*ocean, |(s, _)| s.look)
}

/// The host's handle on the water cameras' media: call [`Self::update`] once a frame from a
/// system in [`WaterSystems::Medium`].
#[derive(SystemParam)]
pub struct WaterMediums<'w, 's> {
    settings: Res<'w, WaterMediumSettings>,
    regions: Res<'w, WaterRegionsInput>,
    cameras: Query<'w, 's, (Entity, &'static GlobalTransform, Option<&'static Projection>, &'static mut WaterMedium), With<WaterCamera>>,
    cues: MessageWriter<'w, WaterMediumCue>,
}

impl WaterMediums<'_, '_> {
    /// Every water camera's medium from its near plane (render space + `origin_cell`: the floating
    /// origin's cell, as in [`crate::WaterFrame::origin_cell`]) and `sample`, the active worldspace's
    /// water query in world coordinates.
    pub fn update(&mut self, origin_cell: IVec3, sample: impl Fn(WorldPos) -> Option<WaterSample>) {
        let settings = *self.settings;
        for (camera, transform, projection, mut medium) in &mut self.cameras {
            let cues = if settings.enabled {
                let eye = WorldPos::new(origin_cell, transform.translation());
                let plane = projection.map_or_else(
                    || NearPlane::point(eye),
                    |p| {
                        // The near plane is NDC z = 1 (Bevy's reverse Z), for any projection.
                        let view_from_clip = p.get_clip_from_view().inverse();
                        let corners = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)].map(|(x, y)| view_from_clip.project_point3(Vec3::new(x, y, 1.0)));
                        NearPlane::from_view(eye, transform.rotation(), corners)
                    },
                );
                medium.medium.update(&plane, &sample, settings.hysteresis_m)
            } else {
                medium.medium.force_above()
            };
            let map = self.regions.map.as_ref().filter(|_| medium.medium.kind == Some(WaterKind::Ocean));
            let optics = map.map_or(0, |m| u32::from(optics_index_at(m, medium.medium.centre_xz)));
            medium.optics = optics;
            if let Some(entered) = cues.underwater {
                self.cues.write(WaterMediumCue { camera, cue: MediumCue::Underwater { entered } });
            }
            if let Some(downward) = cues.surface_break {
                self.cues.write(WaterMediumCue { camera, cue: MediumCue::SurfaceBreak { downward } });
            }
        }
    }
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<WaterMediumSettings>()
        .add_message::<WaterMediumCue>()
        .configure_sets(PostUpdate, WaterSystems::Medium.after(TransformSystems::Propagate).before(WaterSystems::Drive));
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.add_systems(ExtractSchedule, extract);
    }
}

type CameraItem = (RenderEntity, Option<&'static WaterMedium>, Has<WaterCamera>, &'static GlobalTransform, Option<&'static AmbientLight>);

/// The medium uniform on every water camera's render entity; on a camera the underwater post
/// draws for ([`post_on`]) also its [`UnderwaterUniform`] and [`SkyOccluded`] (the sky, the aerial
/// perspective and the `sf_atmos` composite skip it), removed again once it is not.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn extract(
    mut commands: Commands,
    settings: Extract<Res<WaterMediumSettings>>,
    ocean: Extract<Res<WaterLook>>,
    regions: Extract<Res<WaterRegionsInput>>,
    global_ambient: Extract<Option<Res<GlobalAmbientLight>>>,
    lights: Extract<Query<(&DirectionalLight, &GlobalTransform)>>,
    surfaces: Extract<Query<(&WaterSurface, &InheritedVisibility)>>,
    cameras: Extract<Query<CameraItem, With<bevy_camera::Camera3d>>>,
) {
    let mut sun = None;
    for (entity, medium, water, transform, ambient) in &cameras {
        let mut e = commands.entity(entity);
        let Some(m) = medium.filter(|_| water) else {
            e.remove::<(WaterMediumUniform, UnderwaterUniform, SkyOccluded, crate::render::UnderwaterBindings)>();
            continue;
        };
        let uniform = WaterMediumUniform::from(m);
        e.insert(uniform);
        if post_on(&settings, &uniform) {
            let sun = *sun.get_or_insert_with(|| surface_sun(lights.iter()));
            let look = body_look(m, transform.translation(), &ocean, &regions, surfaces.iter());
            let ambient = view_ambient(ambient, global_ambient.as_deref());
            e.insert((UnderwaterUniform::new(m, &look, sun, ambient, &settings), SkyOccluded));
        } else {
            e.remove::<(UnderwaterUniform, SkyOccluded, crate::render::UnderwaterBindings)>();
        }
    }
}
