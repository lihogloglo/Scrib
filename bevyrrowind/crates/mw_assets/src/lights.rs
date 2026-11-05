//! Placed lights: positioned at the model's `AttachLight` node (the flame
//! of a candle, not the base of its holder) and animated (flicker, pulse).

use bevy::light::GlobalAmbientLight;
use bevy::prelude::*;
use mw_world::{LightAnimation, LightInfo};
use sf_sky_bevy::{BudgetedShadow, GiSkipEmissive, ShadowReach};

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
    /// Range cutoff as a multiple of the record's radius, for a light holding
    /// a shadow map (its shadow stops it at walls).
    pub range_scale: f32,
    /// Range cutoff as a multiple of the record's radius without a shadow
    /// map (lights.md "Reach follows shadow state", 2026-11-05): short, so an
    /// unshadowed lamp doesn't light the next room through the wall. `OpenMW`'s
    /// GL path uses 1.75, Clockwork 1.0; the budget blends between the two
    /// reaches with the shadow fade ([`sf_sky_bevy::ShadowReach`]).
    pub unshadowed_range_scale: f32,
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

/// N3, the moonlight's strength: the moons, all full above the atmosphere, light a face
/// square to them with this many times the night floor's diffuse light
/// ([`LightCalibration::night_ambient_nits`]), so 31.4 lx. At the ground (the blue
/// `Sun Night Color` loses ~45 % in the air, Clear dims a little) that is measured
/// (`mw_weather` `print_moonlight`) ≈ 1.1-1.3x the floor's 2.6 nits on a lit face at full
/// Masser, 0.6x at half, 0 at new (Lambert phase law; the floor stays), 0.1-0.25x overcast:
/// clear modelling while the shadowed side still reads. Artistic (Skyrim, Witcher 3 and UE
/// light nights with an exaggerated moon): the real full moon (≈ 0.25 lx) is 1/125 of it.
pub const FULL_MOONLIGHT_FLOOR_RATIO: f32 = 2.5;

impl Default for LightCalibration {
    fn default() -> Self {
        Self { lumens_per_radius_sq: 8_000.0, range_scale: 4.0, unshadowed_range_scale: 4.0, ambient_nits: 300.0, exposure_floor_ev100: 3.5, night_ambient_nits: 4.0 }
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
        sky.moonlight = sf_sky_bevy::sf_sky::Moonlight { full_lux: self.full_moonlight_lux(), artistic: true };
    }

    /// The full moons' illuminance above the atmosphere, lux ([`FULL_MOONLIGHT_FLOOR_RATIO`]):
    /// a Lambert face lit by `E` shows `albedo * E / π`, the floor `albedo * nits`.
    pub fn full_moonlight_lux(&self) -> f32 {
        FULL_MOONLIGHT_FLOOR_RATIO * std::f32::consts::PI * self.night_ambient_nits
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

    /// A light's reach by shadow state: [`Self::unshadowed_range_scale`] and
    /// [`Self::range_scale`] times its radius. Spawn it with the light (and a
    /// `BudgetedShadow`): the shadow budget then writes the range.
    pub fn reach(&self, radius: f32) -> ShadowReach {
        ShadowReach { unshadowed: radius * self.unshadowed_range_scale, shadowed: radius * self.range_scale }
    }

    /// The light for a record, at its unshadowed reach (no shadow map yet).
    pub fn point_light(&self, info: &LightInfo) -> PointLight {
        let [r, g, b] = info.color;
        PointLight {
            color: Color::srgb(r, g, b),
            intensity: self.lumens_per_radius_sq * info.radius * info.radius,
            range: self.reach(info.radius).unshadowed,
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
    let mut entity = commands.spawn((light, BudgetedShadow::default(), calibration.reach(info.radius), Visibility::Inherited));
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

/// A lamp body's glow is its light's: marks the emissive meshes of a model
/// that carries a placed light ([`CarriedLight`]) with [`GiSkipEmissive`], so
/// probe GI doesn't light the room with both the flame mesh and the light
/// (openmw-rtx `SceneUtil::LampBody`; research `mw-lighting-rules-2026-11-05.md`
/// §2.3). Checks meshes as they spawn under a carrier, and a carrier's
/// meshes when it gains its light. On screen the lamp still glows.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn mark_lamp_bodies(
    mut commands: Commands,
    materials: Res<Assets<StandardMaterial>>,
    new_meshes: Query<Entity, (Added<MeshMaterial3d<StandardMaterial>>, With<Mesh3d>)>,
    new_carriers: Query<Entity, Added<CarriedLight>>,
    meshes: Query<&MeshMaterial3d<StandardMaterial>, (With<Mesh3d>, Without<GiSkipEmissive>)>,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    carriers: Query<(), With<CarriedLight>>,
) {
    let mut mark = |entity: Entity| {
        let glows = meshes
            .get(entity)
            .ok()
            .and_then(|m| materials.get(&m.0))
            .is_some_and(|m| m.emissive.red.max(m.emissive.green).max(m.emissive.blue) > 0.0);
        if glows {
            commands.entity(entity).try_insert(GiSkipEmissive);
        }
    };
    for mesh in &new_meshes {
        if parents.iter_ancestors(mesh).any(|a| carriers.contains(a)) {
            mark(mesh);
        }
    }
    for root in &new_carriers {
        for node in children.iter_descendants(root) {
            mark(node);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// N3: the exterior sky gets the artistic moonlight at the calibrated ratio of the floor.
    #[test]
    fn apply_sky_sets_the_moonlight_from_the_floor() {
        let calibration = LightCalibration::default();
        let mut sky = sf_sky_bevy::sf_sky::SkySettings::default();
        calibration.apply_sky(&mut sky);
        assert!(sky.moonlight.artistic);
        let want = FULL_MOONLIGHT_FLOOR_RATIO * std::f32::consts::PI * calibration.night_ambient_nits;
        assert!((sky.moonlight.full_lux - want).abs() < 1e-5, "{}", sky.moonlight.full_lux);
    }

    /// Reach by shadow state (lights.md "Reach follows shadow state"): default 4 x both ways
    /// (owner 2026-11-05: the short reach darkened towns and interiors); a 1.5 x calibration still splits.
    #[test]
    fn reach_by_shadow_state() {
        assert!((LightCalibration::default().unshadowed_range_scale - 4.0).abs() < 1e-6);
        let calibration = LightCalibration { unshadowed_range_scale: 1.5, ..LightCalibration::default() };

        assert!((calibration.range_scale - 4.0).abs() < 1e-6);
        // A 128-unit lamp (1.8288 m): 2.74 m without a shadow, 7.32 m with one.
        let radius = 128.0 * 0.014_287_5;
        let reach = calibration.reach(radius);
        assert!((reach.unshadowed - radius * 1.5).abs() < 1e-5, "{reach:?}");
        assert!((reach.shadowed - radius * 4.0).abs() < 1e-5, "{reach:?}");
        let info = LightInfo { color: [1.0, 0.8, 0.6], radius, animation: LightAnimation::Steady, fire: false };
        let light = calibration.point_light(&info);
        assert!((light.range - reach.unshadowed).abs() < 1e-6);
        assert!(!light.shadow_maps_enabled);
    }

    /// Lamp bodies (research §2.3): the emissive meshes of a model carrying
    /// a light are marked, its other meshes and other models' glowing meshes
    /// are not; meshes spawning under a carrier later are marked too.
    #[test]
    fn marks_emissive_meshes_of_light_carriers_only() {
        let mut world = World::new();
        world.init_resource::<Assets<StandardMaterial>>();
        let (glow, matte) = {
            let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
            (
                materials.add(StandardMaterial { emissive: LinearRgba::rgb(4.0, 1.0, 0.1), ..default() }),
                materials.add(StandardMaterial::default()),
            )
        };
        let mesh = |world: &mut World, material: &Handle<StandardMaterial>, parent: Entity| {
            world.spawn((Mesh3d(Handle::default()), MeshMaterial3d(material.clone()), ChildOf(parent))).id()
        };
        let light = world.spawn(PointLight::default()).id();
        let lamp = world.spawn(CarriedLight(light)).id();
        let node = world.spawn(ChildOf(lamp)).id();
        let flame = mesh(&mut world, &glow, node);
        let holder = mesh(&mut world, &matte, lamp);
        let sign = world.spawn_empty().id();
        let painted_glow = mesh(&mut world, &glow, sign);

        let mut schedule = Schedule::default();
        schedule.add_systems(mark_lamp_bodies);
        schedule.run(&mut world);
        assert!(world.get::<GiSkipEmissive>(flame).is_some());
        assert!(world.get::<GiSkipEmissive>(holder).is_none());
        assert!(world.get::<GiSkipEmissive>(painted_glow).is_none());

        // A mesh arriving after the carrier got its light (async model spawn).
        let late = mesh(&mut world, &glow, node);
        let late_matte = mesh(&mut world, &matte, node);
        schedule.run(&mut world);
        assert!(world.get::<GiSkipEmissive>(late).is_some());
        assert!(world.get::<GiSkipEmissive>(late_matte).is_none());

        // A carrier gaining its light after its meshes spawned.
        let torch = world.spawn_empty().id();
        let torch_flame = mesh(&mut world, &glow, torch);
        schedule.run(&mut world);
        assert!(world.get::<GiSkipEmissive>(torch_flame).is_none());
        world.entity_mut(torch).insert(CarriedLight(light));
        schedule.run(&mut world);
        assert!(world.get::<GiSkipEmissive>(torch_flame).is_some());
    }
}
