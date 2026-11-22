//! Shadowed aerial perspective (`docs/specs/aerial-shadow.md`, ROADMAP 1.27, FG-12): the terrain's
//! shadow-height map ([`FogShadowHeights`]) shadows the sun's single scattering in Bevy's aerial
//! view LUT (vendored `bevy_pbr` patch "own, AS: aerial shadow", [`AtmosphereHeightShadow`]), so
//! haze in front of rock the sun cannot reach stops glowing. [`aerial_height_shadow`] copies the map
//! and the light it was made for onto every camera with Bevy's `AtmosphereSettings` when they change.
//!
//! The far-shadow visibility rule (penumbra, bias, border fade) is shared with the froxels'
//! `far_visibility` (`shaders/froxel_inject.wesl`): one set of tunables ([`FarShadowShape`]) feeds
//! both passes.

use bevy_app::{App, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_math::{Vec3, Vec4};
use bevy_pbr::{AtmosphereHeightShadow, AtmosphereSettings};
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;
use sf_tunables::{Def, Group, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::{CloudShadowLight, FogShadowHeights};

/// `sky.aerial.shadow`: the terrain's shadow-height map shadows the sun's haze in Bevy's aerial
/// perspective. Off by default (owner 2026-11-22, FG-12: blocky shafts that swim with the camera,
/// halo left): stock Bevy, the haze lit through the terrain. On: `--set sky.aerial.shadow=true`.
pub const AERIAL_SHADOW: Def<bool> = def!(bool "sky.aerial.shadow" = false)
    .help("The terrain shadows the sun's light in the distant haze (aerial perspective): no glow in front of shaded terrain")
    .source(Source::Industry("UE5 SkyAtmosphere 'Cast Shadows on Atmosphere'; Hillaire 2020 volumetric shadows in the aerial LUT"))
    .group(Group::Sky);

/// `sky.far_shadow.penumbra_per_m`: the far shadow's penumbra half width per meter of occluder
/// distance (a soft edge widening with the occluder's distance, as a real penumbra).
pub const FAR_SHADOW_PENUMBRA_PER_M: Def<f32> = def!(float "sky.far_shadow.penumbra_per_m" = 0.012)
    .range(0.0, 0.1)
    .unit(Unit::Ratio)
    .help("Penumbra half width per meter of occluder distance of the terrain's far shadow in the fog and haze")
    .source(Source::Reference("sf_lod_bevy far_shadow_map.wesl FAR_PENUMBRA (the far field's rule, kept in step)"))
    .group(Group::Sky);

/// `sky.far_shadow.penumbra_min_m`: the far shadow's penumbra half width floor.
pub const FAR_SHADOW_PENUMBRA_MIN: Def<f32> = def!(float "sky.far_shadow.penumbra_min_m" = 2.0)
    .range(0.0, 50.0)
    .unit(Unit::Meters)
    .help("Smallest penumbra half width of the terrain's far shadow in the fog and haze")
    .source(Source::Reference("sf_lod_bevy far_shadow_map.wesl FAR_PENUMBRA_MIN (kept in step)"))
    .group(Group::Sky);

/// `sky.far_shadow.bias_m`: height added to a point before the far shadow test (no self-shadowing
/// of the 24 m texels on slopes).
pub const FAR_SHADOW_BIAS: Def<f32> = def!(float "sky.far_shadow.bias_m" = 6.0)
    .range(0.0, 100.0)
    .unit(Unit::Meters)
    .help("Height bias of the terrain's far shadow test in the fog and haze")
    .source(Source::Reference("sf_lod_bevy far_shadow_map.wesl FAR_BIAS (kept in step)"))
    .group(Group::Sky);

/// `sky.far_shadow.bias_per_m`: the far shadow's bias growth per meter of distance from the camera.
pub const FAR_SHADOW_BIAS_PER_M: Def<f32> = def!(float "sky.far_shadow.bias_per_m" = 0.0015)
    .range(0.0, 0.05)
    .unit(Unit::Ratio)
    .help("Height bias growth per meter of view distance of the terrain's far shadow test in the fog and haze")
    .source(Source::Reference("sf_lod_bevy far_shadow_map.wesl FAR_BIAS_PER_M (kept in step)"))
    .group(Group::Sky);

/// `sky.far_shadow.fade_start`: where the far shadow starts fading to lit, as a fraction of the
/// map's half extent (no hard edge where the map ends).
pub const FAR_SHADOW_FADE_START: Def<f32> = def!(float "sky.far_shadow.fade_start" = 0.85)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Fraction of the far shadow map's half extent where its shadow starts fading to lit")
    .source(Source::Reference("sf_lod_bevy far_shadow_map.wesl FAR_FADE_START (kept in step)"))
    .group(Group::Sky);

/// The far-shadow visibility rule's values (bound from the `sky.far_shadow.*` tunables), read by
/// the froxels' `far_visibility` and the aerial view LUT's height shadow.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct FarShadowShape {
    /// Penumbra half width per meter of occluder distance.
    pub penumbra_per_m: f32,
    /// Penumbra half width floor, meters.
    pub penumbra_min: f32,
    /// Height bias, meters.
    pub bias: f32,
    /// Bias growth per meter of distance from the camera.
    pub bias_per_m: f32,
    /// Fade to lit from this fraction of the map's half extent.
    pub fade_start: f32,
}

impl Default for FarShadowShape {
    fn default() -> Self {
        Self {
            penumbra_per_m: FAR_SHADOW_PENUMBRA_PER_M.default_value(),
            penumbra_min: FAR_SHADOW_PENUMBRA_MIN.default_value(),
            bias: FAR_SHADOW_BIAS.default_value(),
            bias_per_m: FAR_SHADOW_BIAS_PER_M.default_value(),
            fade_start: FAR_SHADOW_FADE_START.default_value(),
        }
    }
}

impl FarShadowShape {
    /// The froxels' `far_vis` (penumbra per m, penumbra floor, bias, bias per m).
    pub(crate) fn froxel_vec(self) -> Vec4 {
        Vec4::new(self.penumbra_per_m, self.penumbra_min, self.bias, self.bias_per_m)
    }
}

/// [`AERIAL_SHADOW`]'s value: the aerial view LUT gets the height shadow.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AerialShadow(pub bool);

impl Default for AerialShadow {
    fn default() -> Self {
        Self(AERIAL_SHADOW.default_value())
    }
}

/// Registers the tunables, binds them, and adds [`aerial_height_shadow`].
pub(crate) fn build(app: &mut App) {
    app.init_resource::<FarShadowShape>().init_resource::<AerialShadow>();
    let _bound = app.add_tunable(&AERIAL_SHADOW).bind_resource(|a: &mut AerialShadow, on| a.0 = on);
    let _bound = app.add_tunable(&FAR_SHADOW_PENUMBRA_PER_M).bind_resource(|s: &mut FarShadowShape, v| s.penumbra_per_m = v);
    let _bound = app.add_tunable(&FAR_SHADOW_PENUMBRA_MIN).bind_resource(|s: &mut FarShadowShape, v| s.penumbra_min = v);
    let _bound = app.add_tunable(&FAR_SHADOW_BIAS).bind_resource(|s: &mut FarShadowShape, v| s.bias = v);
    let _bound = app.add_tunable(&FAR_SHADOW_BIAS_PER_M).bind_resource(|s: &mut FarShadowShape, v| s.bias_per_m = v);
    let _bound = app.add_tunable(&FAR_SHADOW_FADE_START).bind_resource(|s: &mut FarShadowShape, v| s.fade_start = v);
    app.add_systems(PostUpdate, aerial_height_shadow.after(TransformSystems::Propagate));
}

/// The camera component the map, the light and the switch give: none when the switch is off, the
/// map has no extent yet, or no light carries the shadow.
#[must_use]
pub fn wanted_height_shadow(heights: &FogShadowHeights, shape: FarShadowShape, enabled: bool, light_dir: Option<Vec3>) -> Option<AtmosphereHeightShadow> {
    let image = heights.image.clone().filter(|_| enabled && heights.params.z > 0.0)?;
    Some(AtmosphereHeightShadow {
        image,
        params: heights.params,
        light_dir: light_dir?,
        penumbra_per_m: shape.penumbra_per_m,
        penumbra_min: shape.penumbra_min,
        bias: shape.bias,
        bias_per_m: shape.bias_per_m,
        fade_start: shape.fade_start,
    })
}

/// Copies [`FogShadowHeights`] (and the [`CloudShadowLight`] it shadows, the sun the terrain's far
/// map is made for, as the froxels' `shadow_light`) onto every camera with `AtmosphereSettings` as
/// an [`AtmosphereHeightShadow`], writing only when it changes; removes it when there is none.
#[allow(clippy::type_complexity)]
pub fn aerial_height_shadow(
    mut commands: Commands,
    heights: Option<Res<FogShadowHeights>>,
    shape: Res<FarShadowShape>,
    enabled: Res<AerialShadow>,
    lights: Query<&GlobalTransform, (With<CloudShadowLight>, With<DirectionalLight>)>,
    mut cameras: Query<(Entity, Option<&mut AtmosphereHeightShadow>), With<AtmosphereSettings>>,
) {
    let light_dir = lights.iter().next().and_then(|t| t.back().as_vec3().try_normalize());
    let wanted = heights.and_then(|h| wanted_height_shadow(&h, *shape, enabled.0, light_dir));
    for (entity, current) in &mut cameras {
        match (&wanted, current) {
            (Some(w), Some(mut c)) => {
                let _ = c.set_if_neq(w.clone());
            }
            (Some(w), None) => {
                commands.entity(entity).insert(w.clone());
            }
            (None, Some(_)) => {
                commands.entity(entity).remove::<AtmosphereHeightShadow>();
            }
            (None, None) => {}
        }
    }
}
