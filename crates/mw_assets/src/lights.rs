//! Placed lights: positioned at the model's `AttachLight` node (the flame
//! of a candle, not the base of its holder) and animated (flicker, pulse).

use bevy::light::GlobalAmbientLight;
use bevy::prelude::*;
use mw_world::{LightAnimation, LightInfo};

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
}

impl Default for LightCalibration {
    fn default() -> Self {
        Self { lumens_per_radius_sq: 8_000.0, range_scale: 4.0, ambient_nits: 300.0 }
    }
}

impl LightCalibration {
    /// The scene's ambient light for a cell's ambient color (sRGB).
    pub fn ambient(&self, [r, g, b]: [u8; 3]) -> GlobalAmbientLight {
        GlobalAmbientLight { color: Color::srgb_u8(r, g, b), brightness: self.ambient_nits, ..GlobalAmbientLight::default() }
    }

    pub fn point_light(&self, info: &LightInfo) -> PointLight {
        let [r, g, b] = info.color;
        PointLight {
            color: Color::srgb(r, g, b),
            intensity: self.lumens_per_radius_sq * info.radius * info.radius,
            range: info.radius * self.range_scale,
            radius: 0.05,
            shadow_maps_enabled: false,
            ..PointLight::default()
        }
    }
}

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
    let mut entity = commands.spawn((light, Visibility::Inherited));
    if info.animation != LightAnimation::Steady {
        entity.insert(AnimatedLight { intensity: light.intensity, animation: info.animation, seed });
    }
    let id = entity.id();
    match model {
        Some(root) => {
            commands.entity(id).insert((Transform::IDENTITY, ChildOf(root)));
            commands.entity(root).insert(CarriedLight(id));
        }
        None => {
            commands.entity(id).insert(transform);
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
            commands.entity(light.0).insert((ChildOf(node), Transform::IDENTITY));
        }
    }
}

pub(crate) fn animate_lights(time: Res<Time>, mut lights: Query<(&AnimatedLight, &mut PointLight)>) {
    let now = time.elapsed_secs();
    for (animated, mut light) in &mut lights {
        light.intensity = animated.intensity * animated.animation.brightness(now, animated.seed);
    }
}
