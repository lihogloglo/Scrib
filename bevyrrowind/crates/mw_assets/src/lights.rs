//! Placed lights: positioned at the model's `AttachLight` node (the flame
//! of a candle, not the base of its holder) and animated (flicker, pulse).
//!
//! The night calibration ([`LightCalibration`]'s exposure floor and night ambient, the moonlight
//! ratio, and the framework's `sky.night.glow_ratio`) is four live tunables
//! (exposure-tonemap.md "Night darkness", slice E1), bound to the exterior's `SkyConfig` by
//! [`add_night_tunables`]. The three calibration ones carry framework names but are declared here,
//! with the game's values as their defaults, the way `mw_environment_bevy::quality_tunables` declares
//! framework-named values: their framework home (`sf_sky_bevy`, which owns `SkyConfig`) doesn't
//! declare or bind them yet; once it does, these become the builtin mod's `sf.tunable` records.

use bevy::light::GlobalAmbientLight;
use bevy::prelude::*;
use mw_world::{LightAnimation, LightInfo};
use sf_sky_bevy::sf_sky::NIGHT_GLOW_RATIO;
use sf_sky_bevy::{BudgetedShadow, GiSkipEmissive, ShadowReach, SkyConfig};
use sf_tunables_bevy::overrides::Refusal;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Group, Id, Net, Scope, Source, Type, Unit, def};
use sf_tunables_bevy::{OverrideRefusals, SetBy, TunableSystems, Tunables, TunablesAppExt};

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
    /// (owner option A, 2026-10-13; [`EXPOSURE_FLOOR`], live). Lights are calibrated for
    /// [`crate::SCENE_EV100`]; a night exterior opening to EV 0.5-2 showed
    /// them 30-90x brighter than "fully lit" and blew out walls near lamps.
    /// At the floor they read `2^(SCENE_EV100 - floor)` brighter, and the
    /// night's readability comes from [`Self::night_ambient_nits`] instead of
    /// a wide-open camera. Interiors sit at [`crate::SCENE_EV100`] (scheduled
    /// exposure) or meter above the floor (viewer auto exposure).
    pub exposure_floor_ev100: f32,
    /// The exterior's ambient floor at night (nits, sun below -12°; the sky's
    /// `SkySettings::ambient`; [`NIGHT_AMBIENT`], live): starlight and moonlit sky as one
    /// readable floor at the exposure floor. 4 nits (x the sky's blue ambient colour,
    /// luminance ~0.65) puts moonlit stone about 2 EV under mid grey at the old 3.5 floor,
    /// the usual "dark but readable" game night; 1 nit (LT-1, ~4 EV under) was rejected
    /// by the owner 2026-10-19 as "can't see anything".
    pub night_ambient_nits: f32,
    /// The full moons' light over the night floor's diffuse light ([`MOON_FLOOR_RATIO`], live).
    pub moonlight_floor_ratio: f32,
}

/// `exposure.min_ev100`: [`LightCalibration::exposure_floor_ev100`], the night exposure floor.
/// 4.5 (EX-1, owner 2026-11-17: "floor 4.5 looked nice"; was 3.5, option A 2026-10-13).
pub const EXPOSURE_FLOOR: Def<f32> = def!(float "exposure.min_ev100" = 4.5)
    .range(-10.0, 16.0)
    .help("Darkest exposure a view may take, EV100: the night exposure floor (higher: darker nights)")
    .source(Source::Owner("2026-11-17 EX-1"))
    .group(Group::Post)
    .scope(Scope::Player)
    .net(Net::Presentation);

/// `sky.night.ambient_nits`: [`LightCalibration::night_ambient_nits`], the night ambient floor.
pub const NIGHT_AMBIENT: Def<f32> = def!(float "sky.night.ambient_nits" = 4.0)
    .range(0.0, 20.0)
    .help("Exterior ambient floor at night, nits (sun below -12 deg); the moonlight and the night sky glow follow it")
    .source(Source::Owner("2026-10-19 LT-1 (1 nit rejected, 4 kept)"))
    .group(Group::Post)
    .scope(Scope::Player)
    .net(Net::Presentation);

/// `sky.moon.light_floor_ratio`, N3, the moonlight's strength: the moons, all full above the
/// atmosphere, light a face square to them with this many times the night floor's diffuse light
/// ([`LightCalibration::night_ambient_nits`]), so 31.4 lx at 4 nits. At the ground (the blue
/// `Sun Night Color` loses ~45 % in the air, Clear dims a little) that is measured
/// (`mw_weather` `print_moonlight`) ≈ 1.1-1.3x the floor's 2.6 nits on a lit face at full
/// Masser, 0.6x at half, 0 at new (Lambert phase law; the floor stays), 0.1-0.25x overcast:
/// clear modelling while the shadowed side still reads. Artistic (Skyrim, Witcher 3 and UE
/// light nights with an exaggerated moon): the real full moon (≈ 0.25 lx) is 1/125 of it.
pub const MOON_FLOOR_RATIO: Def<f32> = def!(float "sky.moon.light_floor_ratio" = 2.5)
    .range(0.0, 10.0)
    .unit(Unit::Ratio)
    .help("Full moonlight over the night ambient floor's diffuse light (lux = ratio x pi x nits)")
    .source(Source::Reference("artistic: Skyrim, Witcher 3 and UE light nights with an exaggerated moon"))
    .group(Group::Post)
    .scope(Scope::Player)
    .net(Net::Presentation);

/// The default of [`MOON_FLOOR_RATIO`].
pub const FULL_MOONLIGHT_FLOOR_RATIO: f32 = MOON_FLOOR_RATIO.default_value();

/// The night tunables [`add_night_tunables`] binds (in `mw_assets::TUNABLES`).
pub const NIGHT_TUNABLES: [&dyn AnyDef; 4] = [&EXPOSURE_FLOOR, &NIGHT_AMBIENT, &MOON_FLOOR_RATIO, &NIGHT_GLOW_RATIO];

impl Default for LightCalibration {
    fn default() -> Self {
        Self {
            lumens_per_radius_sq: 8_000.0,
            range_scale: 4.0,
            unshadowed_range_scale: 4.0,
            ambient_nits: 300.0,
            exposure_floor_ev100: EXPOSURE_FLOOR.default_value(),
            night_ambient_nits: NIGHT_AMBIENT.default_value(),
            moonlight_floor_ratio: MOON_FLOOR_RATIO.default_value(),
        }
    }
}

impl LightCalibration {
    /// The exposure range every view keeps to: [`Self::exposure_floor_ev100`] up.
    pub fn exposure_range(&self) -> sf_sky_bevy::sf_sky::ExposureRange {
        sf_sky_bevy::sf_sky::ExposureRange { min_ev100: self.exposure_floor_ev100, ..Default::default() }
    }

    /// The exterior sky under this calibration: the exposure floor (the range's top stays), the
    /// night ambient floor (fading out by -4° as the sky's own light takes over) and the moonlight.
    pub fn apply_sky(&self, sky: &mut sf_sky_bevy::sf_sky::SkySettings) {
        sky.exposure_range.min_ev100 = self.exposure_floor_ev100;
        sky.ambient = sf_sky_bevy::sf_sky::KeyCurve::new(vec![(-12.0, self.night_ambient_nits), (-4.0, 0.0)]);
        sky.moonlight = sf_sky_bevy::sf_sky::Moonlight { full_lux: self.full_moonlight_lux(), artistic: true };
    }

    /// The full moons' illuminance above the atmosphere, lux ([`Self::moonlight_floor_ratio`]):
    /// a Lambert face lit by `E` shows `albedo * E / π`, the floor `albedo * nits`.
    pub fn full_moonlight_lux(&self) -> f32 {
        self.moonlight_floor_ratio * std::f32::consts::PI * self.night_ambient_nits
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

/// The night tunables' ids ([`add_night_tunables`]).
#[derive(Resource, Clone, Copy)]
struct NightIds {
    floor: Id<f32>,
    ambient: Id<f32>,
    moon: Id<f32>,
    glow: Id<f32>,
}

impl NightIds {
    fn generations(self, tunables: &Tunables) -> [u64; 4] {
        [self.floor, self.ambient, self.moon, self.glow].map(|id| tunables.var(id.untyped()).generation())
    }

    /// The calibration's night values as the tunables say (the rest is the default's).
    fn calibration(self, tunables: &Tunables) -> LightCalibration {
        LightCalibration {
            exposure_floor_ev100: tunables.read(self.floor),
            night_ambient_nits: tunables.read(self.ambient),
            moonlight_floor_ratio: tunables.read(self.moon),
            ..LightCalibration::default()
        }
    }
}

/// Registers the night tunables ([`NIGHT_TUNABLES`]) and binds them live: [`LightCalibration`]'s
/// floor, ambient and moonlight ratio, and the exterior's `SkyConfig` ([`LightCalibration::apply_sky`]:
/// the exposure floor, the ambient curve, the moonlight; and `night_glow.ratio`), in `PreUpdate`
/// ([`TunableSystems::Bind`]), so a console `set` reaches this frame's `Update`.
///
/// `start` is the run's calibration: where its floor or night ambient differs from the tunable's
/// default it is the launch flag's (`--exposure-floor`, `--night-ambient`), written at the
/// `CommandLine` layer (a later `--set` of the same name wins); a value out of range is refused like
/// a bad `--set` ([`OverrideRefusals`]).
pub fn add_night_tunables(app: &mut App, start: &LightCalibration) {
    let ids = NightIds {
        floor: app.add_tunable(&EXPOSURE_FLOOR).bind_resource(|c: &mut LightCalibration, v| c.exposure_floor_ev100 = v).id(),
        ambient: app.add_tunable(&NIGHT_AMBIENT).bind_resource(|c: &mut LightCalibration, v| c.night_ambient_nits = v).id(),
        moon: app.add_tunable(&MOON_FLOOR_RATIO).bind_resource(|c: &mut LightCalibration, v| c.moonlight_floor_ratio = v).id(),
        glow: app.add_tunable(&NIGHT_GLOW_RATIO).id(),
    };
    start_value(app, &EXPOSURE_FLOOR, start.exposure_floor_ev100, SetBy::CommandLine, "--exposure-floor");
    start_value(app, &NIGHT_AMBIENT, start.night_ambient_nits, SetBy::CommandLine, "--night-ambient");
    start_value(app, &MOON_FLOOR_RATIO, start.moonlight_floor_ratio, SetBy::Code, "the run's LightCalibration");
    app.insert_resource(ids).add_systems(PreUpdate, sync_night_sky.in_set(TunableSystems::Bind));
}

/// Writes `value` at `layer` of `def` unless it is the default (the start didn't change it).
fn start_value(app: &mut App, def: &Def<f32>, value: f32, layer: SetBy, note: &str) {
    #[allow(clippy::float_cmp)] // exact: an unchanged value is the default itself
    if value == def.default_value() {
        return;
    }
    let world = app.world_mut();
    let refused = {
        let mut tunables = world.resource_mut::<Tunables>();
        let id = tunables.id_of(def).unwrap_or_else(|e| panic!("tunable {}: {e}", def.name()));
        tunables.set_noted(id.untyped(), value.to_value(), layer, Some(note.to_owned())).err()
    };
    if let Some(reason) = refused {
        let refusal = Refusal { name: def.name().to_owned(), reason: format!("{reason} ({note})") };
        warn!("tunable {refusal}");
        world.get_resource_or_init::<OverrideRefusals>().0.push(refusal);
    }
}

/// Copies the night tunables into `SkyConfig` when one changed (and on the first frame it exists).
#[allow(clippy::needless_pass_by_value)]
fn sync_night_sky(tunables: Res<Tunables>, ids: Res<NightIds>, sky: Option<ResMut<SkyConfig>>, mut seen: Local<Option<[u64; 4]>>) {
    let key = ids.generations(&tunables);
    if *seen == Some(key) {
        return;
    }
    let Some(mut sky) = sky else { return };
    *seen = Some(key);
    ids.calibration(&tunables).apply_sky(&mut sky.0);
    sky.0.night_glow.ratio = tunables.read(ids.glow);
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
        // The shadow budget's contrast gate reads the steady intensity, not the flicker.
        entity.insert((AnimatedLight { intensity: light.intensity, animation: info.animation, seed }, sf_sky_bevy::SteadyIntensity(light.intensity)));
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

    /// E1: the night exposure floor's default is the owner's EX-1 pick, 4.5 EV100 (was 3.5).
    #[test]
    #[allow(clippy::float_cmp)] // exact: declared values
    fn default_floor_is_ex1_pick() {
        assert_eq!(EXPOSURE_FLOOR.default_value(), 4.5);
        assert_eq!(LightCalibration::default().exposure_floor_ev100, 4.5);
        assert_eq!(LightCalibration::default().night_ambient_nits, 4.0);
        assert_eq!(LightCalibration::default().moonlight_floor_ratio, FULL_MOONLIGHT_FLOOR_RATIO);
    }

    /// An exterior-like app: tunables, the night binding and a sky config.
    fn night_app(start: &LightCalibration) -> App {
        let mut app = App::new();
        app.add_plugins(sf_tunables_bevy::TunablesPlugin).insert_resource(*start).insert_resource(SkyConfig(sf_sky_bevy::sf_sky::SkySettings::default()));
        add_night_tunables(&mut app, start);
        app.update();
        app
    }

    fn set(app: &mut App, name: &str, value: &str) {
        app.world_mut().write_message(sf_tunables_bevy::SetTunable::new(name, value, SetBy::Console));
        app.update();
    }

    fn sky(app: &App) -> &sf_sky_bevy::sf_sky::SkySettings {
        &app.world().resource::<SkyConfig>().0
    }

    /// The ambient curve's night key and its fade-out key.
    fn ambient_keys(app: &App) -> (f32, f32) {
        let curve = &sky(app).ambient;
        (curve.sample(-20.0), curve.sample(-4.0))
    }

    /// E1: each night tunable reaches `SkyConfig` (and `LightCalibration`) on the next update; the
    /// moonlight is ratio x pi x nits.
    #[test]
    #[allow(clippy::float_cmp)] // exact: copied values
    fn night_tunables_reach_the_sky_next_update() {
        let mut app = night_app(&LightCalibration::default());
        // First frame: the defaults.
        assert_eq!(sky(&app).exposure_range.min_ev100, 4.5);
        assert_eq!(ambient_keys(&app), (4.0, 0.0));
        assert_eq!(sky(&app).night_glow.ratio, 0.1);
        let pi = std::f32::consts::PI;
        assert!((sky(&app).moonlight.full_lux - 2.5 * pi * 4.0).abs() < 1e-5);
        assert!(sky(&app).moonlight.artistic);

        set(&mut app, "exposure.min_ev100", "5");
        assert_eq!(sky(&app).exposure_range.min_ev100, 5.0);
        assert_eq!(sky(&app).exposure_range.max_ev100, sf_sky_bevy::sf_sky::ExposureRange::default().max_ev100, "the top stays");
        assert_eq!(app.world().resource::<LightCalibration>().exposure_floor_ev100, 5.0);

        set(&mut app, "sky.night.ambient_nits", "2");
        assert_eq!(ambient_keys(&app), (2.0, 0.0));
        assert!((sky(&app).moonlight.full_lux - 2.5 * pi * 2.0).abs() < 1e-5, "moonlight follows the floor");
        assert_eq!(app.world().resource::<LightCalibration>().night_ambient_nits, 2.0);

        set(&mut app, "sky.moon.light_floor_ratio", "4");
        assert!((sky(&app).moonlight.full_lux - 4.0 * pi * 2.0).abs() < 1e-5, "{}", sky(&app).moonlight.full_lux);
        assert_eq!(app.world().resource::<LightCalibration>().moonlight_floor_ratio, 4.0);

        set(&mut app, "sky.night.glow_ratio", "0.03");
        assert_eq!(sky(&app).night_glow.ratio, 0.03);
        // The others kept.
        assert_eq!(sky(&app).exposure_range.min_ev100, 5.0);
        assert_eq!(ambient_keys(&app), (2.0, 0.0));
    }

    /// E1: the launch flags (`--exposure-floor`, `--night-ambient`, through the run's calibration)
    /// are the tunables' `CommandLine` layer; the console still wins over them, and an out-of-range
    /// flag is refused like a bad `--set`.
    #[test]
    #[allow(clippy::float_cmp)] // exact: copied values
    fn launch_flags_are_the_command_line_layer() {
        let start = LightCalibration { exposure_floor_ev100: 4.0, night_ambient_nits: 2.5, ..LightCalibration::default() };
        let mut app = night_app(&start);
        assert_eq!(sky(&app).exposure_range.min_ev100, 4.0);
        assert_eq!(ambient_keys(&app), (2.5, 0.0));
        let tunables = app.world().resource::<Tunables>();
        let floor = tunables.var(tunables.id("exposure.min_ev100").expect("registered"));
        assert_eq!(floor.set_by(), SetBy::CommandLine);
        set(&mut app, "exposure.min_ev100", "6");
        assert_eq!(sky(&app).exposure_range.min_ev100, 6.0);
        assert!(app.world().get_resource::<OverrideRefusals>().is_none_or(|r| r.0.is_empty()));

        let bad = night_app(&LightCalibration { exposure_floor_ev100: 99.0, ..LightCalibration::default() });
        assert_eq!(bad.world().resource::<OverrideRefusals>().0.len(), 1);
        assert_eq!(sky(&bad).exposure_range.min_ev100, 4.5, "refused: the default stays");
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
