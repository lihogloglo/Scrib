//! The game feeds NPC light perception (`MA16`; ai.md "Light exposure (from game state, never the
//! GPU)"): [`write_light_inputs`] writes `sf_ai_bevy`'s [`AiLightInputs`] every fixed tick, after
//! the player moves ([`ActionSystems::Move`]) and before the sight checks
//! ([`AiPerceptionSystems::Sight`], whose light sampler reads it). Nothing here reads the renderer:
//! no `PointLight`, no exposure, no rendered ambient.
//!
//! - **Interior or exterior.** The active worldspace is an interior when it is an interior cell
//!   without "behave like exterior" ([`LightRecords::interiors`], built from the `CELL` records at
//!   start, as `mw_world::WorldRecords` names them). Anything else (the exterior, Mournhold's
//!   plazas, a `level:` greybox) is outdoors: the sampler casts its roof ray.
//! - **Sky, outdoors** ([`exterior_sky`]): the horizontal illuminance of the sky state
//!   ([`CurrentSky`], pure from clock and weather): for the sun and the moon, the light reaching the
//!   ground times `sin(elevation)` (0 below the horizon) plus its sky fill; weather dims both
//!   through the state. Mapped on a log scale (eyes and cameras see in stops, Weber-Fechner):
//!   `saturate(ln(E / dark) / ln(day / dark))` with [`SKY_DARK_LUX`] (0) and [`SKY_DAY_LUX`] (1).
//!   The sky's ambient floor (`SkyState::ambient`, a readability calibration of the renderer) is
//!   left out, so a moonless night reads 0. No [`CurrentSky`] (headless, no sky plugin): 1, day.
//! - **Sky, indoors** ([`interior_sky`]): the cell's `AMBI` ambient, linearised from sRGB, its
//!   Rec. 709 luminance over [`INTERIOR_AMBIENT_REF`], clamped to 0..1. A cell without `AMBI`
//!   takes [`MISSING_AMBIENT`], the renderer's fallback for the same case.
//! - **Placed lights** ([`LightHook`]): every `LIGH` placement entering R0 (the class `Light`
//!   spawner is wrapped at `PostStartup`, like the walkers' collider hook) is kept in
//!   [`PlacedLights`] until it leaves (an item taken leaves through the same hooks). The record
//!   gives the reach (`LHDT` radius, game units to meters with `mw_world::convert::meters`) and
//!   whether it is lit (`OFF_BY_DEFAULT` clear). Negative and zero-radius lights are not gameplay
//!   lights (none is kept). Only the active worldspace's lights are written (worldspace
//!   coordinates overlap).
//! - **Source point.** A placement's origin sits at its fixture's base (a candle's foot, a
//!   lantern's hook), inside or on the fixture's own collider, and the sampler's ray ends exactly at
//!   the source: the source is the origin lifted straight up by [`SOURCE_LIFT_M`]. A fixture right
//!   under a ceiling may then read occluded (the lift is a tunable).
//! - **Intensity.** A lit `LIGH` is [`LAMP_INTENSITY`]: its reach is the record's radius, its
//!   colour and flicker are looks, not gameplay (Thief's light gem also sums plain light values).
//! - **Carried lights.** A character's held item with a light that shows at its readiness (its
//!   mount for that readiness exists: `HeldItem::drawn`/`sheathed`) is a source at the carrier's
//!   chest ([`crate::sight::CHEST_RATIO`] of its standing height), lit whatever the record's
//!   `OFF_BY_DEFAULT` (carried lights burn), radius from the item's light. Carriers: NPCs (their
//!   mover, in the active worldspace) and the player's avatar (its parent's action body).
//!
//! Not covered: light spells (no magic effects yet), scripts switching a light on or off, the
//! carried light's exact hand position.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate, PostStartup};
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_math::Vec3;
use mw_world::CellEnvironment;
use sf_action_bevy::{ActionBody, ActionSystems};
use sf_ai::LightSource;
use sf_ai_bevy::{AiLightInputs, AiPerceptionSystems};
use sf_character::Readiness;
use sf_character_bevy::{CharacterReadiness, HeldProps};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_records::RecordDb;
use sf_sky_bevy::CurrentSky;
use sf_sky_bevy::sf_sky::{SkyLight, SkyState, luminance};
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_world::{IndexedPlacement, PlacementClass, PlacementFlags, PlacementId, WorldspaceId};
use sf_world_bevy::{ActivateCx, ActiveWorldspace, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx};
use tes3::esp::{Cell, Light, LightFlags};

use crate::walkers::ActorSpace;

// --- Tunables ---------------------------------------------------------------------------------

/// `mw.ai.light.sky_dark_lux`: [`LightInputSettings::sky_dark_lux`].
pub const SKY_DARK_LUX: Def<f32> = def!(float "mw.ai.light.sky_dark_lux" = 3.0)
    .range(0.01, 1000.0)
    .unit(Unit::Lux)
    .help("Horizontal sky illuminance an NPC reads as no sky light at all (sky level 0)")
    .source(Source::Reference("end of civil twilight, about 3.4 lux (IES Lighting Handbook)"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.light.sky_day_lux`: [`LightInputSettings::sky_day_lux`].
pub const SKY_DAY_LUX: Def<f32> = def!(float "mw.ai.light.sky_day_lux" = 10_000.0)
    .range(10.0, 150_000.0)
    .unit(Unit::Lux)
    .help("Horizontal sky illuminance an NPC reads as full daylight (sky level 1); log scale between")
    .source(Source::Reference("overcast daylight, about 10 000 lux (IES Lighting Handbook)"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.light.interior_ambient_ref`: [`LightInputSettings::interior_ambient_ref`].
pub const INTERIOR_AMBIENT_REF: Def<f32> = def!(float "mw.ai.light.interior_ambient_ref" = 0.25)
    .range(0.01, 1.0)
    .unit(Unit::Ratio)
    .help("Linear luminance of a cell's AMBI ambient that reads as full light indoors (sky level 1)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.light.source_lift_m`: [`LightInputSettings::source_lift_m`].
pub const SOURCE_LIFT_M: Def<f32> = def!(float "mw.ai.light.source_lift_m" = 0.3)
    .range(0.0, 2.0)
    .unit(Unit::Meters)
    .help("A placed light's gameplay source sits this far above its placement origin (clear of its fixture)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.light.lamp_intensity`: [`LightInputSettings::lamp_intensity`].
pub const LAMP_INTENSITY: Def<f32> = def!(float "mw.ai.light.lamp_intensity" = 1.0)
    .range(0.0, 2.0)
    .unit(Unit::Ratio)
    .help("Light exposure at the centre of a lit LIGH light, placed or carried (falls to 0 at its radius)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// The light inputs' live tunables (`mw.ai.light.*`).
pub const TUNABLES: &[&dyn AnyDef] = &[&SKY_DARK_LUX, &SKY_DAY_LUX, &INTERIOR_AMBIENT_REF, &SOURCE_LIFT_M, &LAMP_INTENSITY];

/// An interior cell's ambient without `AMBI` (sRGB grey): the renderer's fallback for the same case
/// (`mw_doors_bevy::interior_look`).
// tunables: const (data fallback: mirrors the renderer's missing-AMBI grey, not a trade-off)
pub const MISSING_AMBIENT: f32 = 0.3;

/// Values bound from [`TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LightInputSettings {
    pub sky_dark_lux: f32,
    pub sky_day_lux: f32,
    pub interior_ambient_ref: f32,
    pub source_lift_m: f32,
    pub lamp_intensity: f32,
}

impl Default for LightInputSettings {
    fn default() -> Self {
        Self {
            sky_dark_lux: SKY_DARK_LUX.default_value(),
            sky_day_lux: SKY_DAY_LUX.default_value(),
            interior_ambient_ref: INTERIOR_AMBIENT_REF.default_value(),
            source_lift_m: SOURCE_LIFT_M.default_value(),
            lamp_intensity: LAMP_INTENSITY.default_value(),
        }
    }
}

// --- Records ----------------------------------------------------------------------------------

/// A `LIGH` record as a gameplay light.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightRecord {
    /// Reach, meters.
    pub radius_m: f32,
    /// Lit when placed (`OFF_BY_DEFAULT` clear).
    pub on: bool,
}

/// The gameplay light of a `LIGH` record (module docs): `None` for a negative or zero-radius one.
pub fn light_record(light: &Light) -> Option<LightRecord> {
    let flags = light.data.flags;
    if flags.contains(LightFlags::NEGATIVE) || light.data.radius == 0 {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let radius_m = mw_world::convert::meters(light.data.radius as f32);
    Some(LightRecord { radius_m, on: !flags.contains(LightFlags::OFF_BY_DEFAULT) })
}

/// What the inputs need from the content, built once at start.
#[derive(Resource, Clone, Debug, Default)]
pub struct LightRecords {
    /// Gameplay lights by lowercase `LIGH` id.
    pub lights: Arc<HashMap<String, LightRecord>>,
    /// Interior worldspaces (cells without "behave like exterior") and their `AMBI` ambient (sRGB
    /// 0..1; `None` without `AMBI`).
    pub interiors: Arc<HashMap<WorldspaceId, Option<[f32; 3]>>>,
}

impl LightRecords {
    pub fn build(db: &RecordDb) -> Self {
        let lights = db
            .iter::<Light>()
            .filter_map(|(id, l)| Some((id.as_str().to_ascii_lowercase(), light_record(l)?)))
            .collect();
        let interiors = db
            .iter::<Cell>()
            .filter(|(_, c)| c.is_interior())
            .filter_map(|(_, c)| {
                let env = CellEnvironment::of(c);
                (!env.sky).then(|| (mw_world::ids::interior(&c.name), env.atmosphere.map(|a| a.ambient)))
            })
            .collect();
        Self { lights: Arc::new(lights), interiors: Arc::new(interiors) }
    }
}

// --- Placed lights ----------------------------------------------------------------------------

/// An R0 `LIGH` placement.
#[derive(Clone, Debug, PartialEq)]
pub struct PlacedLight {
    pub worldspace: WorldspaceId,
    /// The placement's origin (the source is lifted from it, [`SOURCE_LIFT_M`]).
    pub origin: WorldPos,
    pub record: LightRecord,
}

/// R0 `LIGH` placements by id, kept by [`LightHook`].
#[derive(Resource, Clone, Debug, Default)]
pub struct PlacedLights(pub HashMap<PlacementId, PlacedLight>);

/// Wraps the `Light` class spawner: R0 activation registers the placement in [`PlacedLights`]
/// (its record from [`LightRecords`]), deactivation forgets it.
pub struct LightHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for LightHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let p = &placement.placement;
        if p.flags.intersects(PlacementFlags::DISABLED | PlacementFlags::EDITOR_ONLY) {
            return;
        }
        let key = p.base.id.as_str().to_ascii_lowercase();
        let Some(record) = world.get_resource::<LightRecords>().and_then(|r| r.lights.get(&key).copied()) else { return };
        if let Some(mut placed) = world.get_resource_mut::<PlacedLights>() {
            placed.0.insert(p.id, PlacedLight { worldspace: cx.worldspace.clone(), origin: p.transform.pos, record });
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        if let Some(mut placed) = world.get_resource_mut::<PlacedLights>() {
            placed.0.remove(&placement.placement.id);
        }
    }
}

/// Wraps the spawners `exterior.rs` built (Startup) with [`LightHook`].
fn hook_lights(spawners: Option<ResMut<PlacementSpawners>>) {
    let Some(mut spawners) = spawners else { return };
    if let Some(inner) = spawners.for_class(PlacementClass::Light).cloned() {
        spawners.set(PlacementClass::Light, Arc::new(LightHook(inner)));
    }
}

// --- Sky --------------------------------------------------------------------------------------

/// Horizontal illuminance one sky light puts on level ground, lux: the beam reaching the ground
/// through the plane (`sin(elevation)`, 0 below the horizon) plus its sky fill.
pub fn horizontal_lux(light: &SkyLight) -> f32 {
    light.ground_illuminance() * light.elevation.sin().max(0.0) + light.sky_fill.max(0.0)
}

/// `lux` on the log scale from `dark` (0) to `day` (1), clamped; 0 for non-finite or non-positive
/// input, 1 when `day` is not above `dark`.
pub fn log_level(lux: f32, dark: f32, day: f32) -> f32 {
    if !lux.is_finite() || !dark.is_finite() || dark <= 0.0 || lux <= dark {
        return 0.0;
    }
    if !day.is_finite() || day <= dark {
        return 1.0;
    }
    ((lux / dark).ln() / (day / dark).ln()).clamp(0.0, 1.0)
}

/// The outdoor sky level of `state` (module docs "Sky, outdoors").
pub fn exterior_sky(state: &SkyState, settings: &LightInputSettings) -> f32 {
    let lux = horizontal_lux(&state.sun) + horizontal_lux(&state.moon);
    log_level(lux, settings.sky_dark_lux, settings.sky_day_lux)
}

/// sRGB transfer function to linear (IEC 61966-2-1).
fn srgb_to_linear(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// The indoor sky level of a cell's `AMBI` ambient (module docs "Sky, indoors").
pub fn interior_sky(ambient: Option<[f32; 3]>, settings: &LightInputSettings) -> f32 {
    let srgb = ambient.unwrap_or([MISSING_AMBIENT; 3]);
    let l = luminance(Vec3::from_array(srgb.map(srgb_to_linear)));
    let r = settings.interior_ambient_ref;
    if !r.is_finite() || r <= 0.0 {
        return 1.0;
    }
    (l / r).clamp(0.0, 1.0)
}

// --- The system -------------------------------------------------------------------------------

/// Writes [`AiLightInputs`] from the active worldspace, the sky state, the R0 lights and the
/// characters' carried lights (module docs).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)]
pub fn write_light_inputs(
    settings: Res<LightInputSettings>,
    records: Res<LightRecords>,
    placed: Res<PlacedLights>,
    active: Option<Res<ActiveWorldspace>>,
    sky: Option<Res<CurrentSky>>,
    mut inputs: ResMut<AiLightInputs>,
    carriers: Query<(&HeldProps, Option<&CharacterReadiness>, Option<&Mover>, Option<&ActorSpace>, Option<&ChildOf>)>,
    bodies: Query<&ActionBody>,
) {
    let active = active.and_then(|a| a.0.clone());
    let interior = active.as_ref().and_then(|ws| records.interiors.get(ws));
    let (sky_level, is_interior) = match interior {
        Some(ambient) => (interior_sky(*ambient, &settings), true),
        // tunables: const (no sky state: day, as the held-torch rule reads it)
        None => (sky.map_or(1.0, |s| exterior_sky(&s.0, &settings)), false),
    };
    let lift = if settings.source_lift_m.is_finite() { settings.source_lift_m.max(0.0) } else { 0.0 };
    let intensity = settings.lamp_intensity;
    let inputs = &mut *inputs;
    inputs.sky = sky_level;
    inputs.interior = is_interior;
    inputs.sources.clear();
    if let Some(ws) = &active {
        inputs.sources.extend(placed.0.values().filter(|l| &l.worldspace == ws).map(|l| LightSource {
            pos: l.origin.offset(Vec3::Y * lift),
            radius_m: l.record.radius_m,
            intensity,
            on: l.record.on,
        }));
    }
    for (held, readiness, mover, space, parent) in &carriers {
        let readiness = readiness.map_or(Readiness::default(), |r| r.0);
        let carrier = match mover {
            Some(m) => {
                if space.is_some_and(|s| Some(&s.0) != active.as_ref()) {
                    continue;
                }
                Some((m.loco.position, m.params.height))
            }
            None => parent.and_then(|p| bodies.get(p.parent()).ok()).map(|b| (b.loco.position, b.params.height)),
        };
        let Some((feet, height)) = carrier else { continue };
        for prop in &held.props {
            let item = &prop.item;
            let shows = match readiness {
                Readiness::Drawn => item.drawn.is_some(),
                Readiness::Sheathed => item.sheathed.is_some(),
            };
            let Some(light) = item.light.as_ref().filter(|_| shows) else { continue };
            inputs.sources.push(LightSource {
                pos: feet.offset(Vec3::Y * (crate::sight::CHEST_RATIO * height)),
                radius_m: light.radius,
                intensity,
                on: true,
            });
        }
    }
}

/// Adds the light inputs (called by [`crate::sight::add`]): settings and their tunables, the
/// records (empty until the host inserts [`LightRecords::build`]), the hook and the system.
pub fn add(app: &mut App) {
    app.init_resource::<LightInputSettings>()
        .init_resource::<LightRecords>()
        .init_resource::<PlacedLights>()
        .init_resource::<AiLightInputs>()
        .add_systems(PostStartup, hook_lights)
        .add_systems(
            FixedUpdate,
            write_light_inputs.after(ActionSystems::Move).before(AiPerceptionSystems::Sight),
        );
    let _dark = app.add_tunable(&SKY_DARK_LUX).bind_resource(|s: &mut LightInputSettings, v| s.sky_dark_lux = v);
    let _day = app.add_tunable(&SKY_DAY_LUX).bind_resource(|s: &mut LightInputSettings, v| s.sky_day_lux = v);
    let _ambient = app
        .add_tunable(&INTERIOR_AMBIENT_REF)
        .bind_resource(|s: &mut LightInputSettings, v| s.interior_ambient_ref = v);
    let _lift = app.add_tunable(&SOURCE_LIFT_M).bind_resource(|s: &mut LightInputSettings, v| s.source_lift_m = v);
    let _intensity = app.add_tunable(&LAMP_INTENSITY).bind_resource(|s: &mut LightInputSettings, v| s.lamp_intensity = v);
}

/// Inserts the content's [`LightRecords`] (the host's record db) and logs their counts.
pub fn insert_records(app: &mut App, db: &RecordDb) {
    let records = LightRecords::build(db);
    info!("light inputs: {} gameplay LIGH records, {} interior cells", records.lights.len(), records.interiors.len());
    app.insert_resource(records);
}

#[cfg(test)]
#[path = "light_inputs_tests.rs"]
mod tests;
