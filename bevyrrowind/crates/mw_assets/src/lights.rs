//! Placed lights: positioned at the model's `AttachLight` node (the flame
//! of a candle, not the base of its holder) and animated (flicker, pulse).

use bevy::light::GlobalAmbientLight;
use bevy::prelude::*;
use mw_world::{LightAnimation, LightInfo};
use sf_sky_bevy::BudgetedShadow;

/// Marks a model's `AttachLight` node.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct AttachLight;

/// On a placed model's root: the light it carries, moved under the
/// model's [`AttachLight`] node once the model has spawned.
#[derive(Component, Clone, Copy, Debug)]
pub struct CarriedLight(pub Entity);

/// A light whose brightness varies over time.
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimatedLight {
    pub intensity: f32,
    pub animation: LightAnimation,
    pub seed: u32,
}

/// Maps Morrowind lighting to physically based lighting.
///
/// The original renderer computes `texture * (ambient + lights)` where a
/// light's contribution falls off linearly as `radius / (3 * distance)`:
/// one third at the radius, and a long, dim tail. Here lights are physical
/// (inverse-square), with luminous power proportional to radius squared so
/// every light reaches the same brightness at the same fraction of its
/// radius, and a range cutoff well past the radius to keep the tail.
/// "Fully lit" is [`crate::emissive_nits`] at [`crate::SCENE_EV100`].
#[derive(Resource, Clone, Copy, Debug)]
pub struct LightCalibration {
    /// Luminous power per square meter of radius (lumens / m²).
    pub lumens_per_radius_sq: f32,
    /// Range cutoff as a multiple of the record's radius.
    pub range_scale: f32,
    /// Ambient luminance for a white cell ambient color (nits).
    pub ambient_nits: f32,
    /// The darkest exposure any view may take (EV100): the night floor
    /// (owner option A, 2026-10-13). Lights are calibrated for
    /// [`crate::SCENE_EV100`]; a night exterior opening to EV 0.5-2 showed
    /// them 30-90x brighter than "fully lit" and blew out walls near lamps.
    /// At the floor they read `2^(SCENE_EV100 - floor)` brighter, and the
    /// night's readability comes from [`Self::night_ambient_nits`] instead of
    /// a wide-open camera. Interiors sit at [`crate::SCENE_EV100`] (scheduled
    /// exposure) or meter above the floor (viewer auto exposure).
    pub exposure_floor_ev100: f32,
    /// The exterior's ambient floor at night (nits, sun below -12°; the sky's
    /// `SkySettings::ambient`): starlight and moonlit sky as one readable
    /// floor at the exposure floor. 4 nits (x the sky's blue ambient colour,
    /// luminance ~0.65) puts moonlit stone about 2 EV under mid grey, the usual
    /// "dark but readable" game night; 1 nit (LT-1, ~4 EV under) was rejected
    /// by the owner 2026-10-19 as "can't see anything".
    pub night_ambient_nits: f32,
}

impl Default for LightCalibration {
    fn default() -> Self {
        Self { lumens_per_radius_sq: 8_000.0, range_scale: 4.0, ambient_nits: 300.0, exposure_floor_ev100: 3.5, night_ambient_nits: 4.0 }
    }
}

impl LightCalibration {
    /// The exposure range every view keeps to: [`Self::exposure_floor_ev100`] up.
    pub fn exposure_range(&self) -> sf_sky_bevy::sf_sky::ExposureRange {
        sf_sky_bevy::sf_sky::ExposureRange { min_ev100: self.exposure_floor_ev100, ..Default::default() }
    }

    /// The exterior sky under this calibration: the exposure floor and the
    /// night ambient floor (fading out by -4° as the sky's own light takes over).
    pub fn apply_sky(&self, sky: &mut sf_sky_bevy::sf_sky::SkySettings) {
        sky.exposure_range = self.exposure_range();
        sky.ambient = sf_sky_bevy::sf_sky::KeyCurve::new(vec![(-12.0, self.night_ambient_nits), (-4.0, 0.0)]);
    }

    /// The scene's ambient light for a cell's ambient color (sRGB).
    pub fn ambient(&self, [r, g, b]: [u8; 3]) -> GlobalAmbientLight {
        GlobalAmbientLight { color: Color::srgb_u8(r, g, b), brightness: self.ambient_nits, ..GlobalAmbientLight::default() }
    }

    /// Where a light of `radius` lights a white surface facing it to exactly
    /// "fully lit" at [`crate::SCENE_EV100`]: `L = I / (π d²)` with
    /// `I = Φ / 4π` equals [`crate::emissive_nits`]. Closer is brighter
    /// (inverse-square, no clamp); the original saturates within `radius / 3`.
    pub fn fully_lit_distance(&self, radius: f32) -> f32 {
        radius * (self.lumens_per_radius_sq / (4.0 * std::f32::consts::PI * std::f32::consts::PI * crate::emissive_nits())).sqrt()
    }

    pub fn point_light(&self, info: &LightInfo) -> PointLight {
        let [r, g, b] = info.color;
        PointLight {
            color: Color::srgb(r, g, b),
            intensity: self.lumens_per_radius_sq * info.radius * info.radius,
            range: info.radius * self.range_scale,
            radius: 0.05,
            // Shadows only through the `PointShadowBudget` (`BudgetedShadow`).
            shadow_maps_enabled: false,
            shadow_map_near_z: SHADOW_NEAR_Z,
            ..PointLight::default()
        }
    }
}

/// Casters nearer a light than this (meters) don't shadow it: a lantern's
/// cage or a torch's own head around the flame (`AttachLight`) would
/// otherwise stripe the room with its own bars.
pub const SHADOW_NEAR_Z: f32 = 0.25;

/// Spawns the light for a placement. With a `model` root, the light starts
/// at the root and moves to the model's `AttachLight` node when it spawns.
pub fn spawn_light(
    commands: &mut Commands,
    calibration: &LightCalibration,
    info: &LightInfo,
    model: Option<Entity>,
    transform: Transform,
    seed: u32,
) -> Entity {
    let light = calibration.point_light(info);
    let mut entity = commands.spawn((light, BudgetedShadow::default(), Visibility::Inherited));
    if info.animation != LightAnimation::Steady {
        entity.insert(AnimatedLight { intensity: light.intensity, animation: info.animation, seed });
    }
    let id = entity.id();
    match model {
        Some(root) => {
            commands.entity(id).try_insert((Transform::IDENTITY, ChildOf(root)));
            commands.entity(root).try_insert(CarriedLight(id));
        }
        None => {
            commands.entity(id).try_insert(transform);
        }
    }
    id
}

pub(crate) fn attach_lights(
    mut commands: Commands,
    added: Query<Entity, Added<AttachLight>>,
    parents: Query<&ChildOf>,
    carriers: Query<&CarriedLight>,
) {
    for node in &added {
        if let Some(light) = parents.iter_ancestors(node).find_map(|a| carriers.get(a).ok()) {
            commands.entity(light.0).try_insert((ChildOf(node), Transform::IDENTITY));
        }
    }
}

pub(crate) fn animate_lights(
    time: Res<Time>,
    mut lights: Query<(&AnimatedLight, &mut PointLight, Option<&InheritedVisibility>)>,
) {
    let now = time.elapsed_secs();
    for (animated, mut light, vis) in &mut lights {
        if vis.is_some_and(|v| !v.get()) {
            continue;
        }
        let next = animated.intensity * animated.animation.brightness(now, animated.seed);
        if (light.intensity - next).abs() > animated.intensity * 0.015 {
            light.intensity = next;
        }
    }
}
