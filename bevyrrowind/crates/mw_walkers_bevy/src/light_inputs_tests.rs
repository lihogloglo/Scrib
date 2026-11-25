//! Headless tests of the game's light inputs (`MA16`, [`crate::light_inputs`]) through the game's
//! sight wiring ([`crate::sight::add`]) on `sf_physics_bevy::Physics`: two `LIGH` placements enter
//! R0 through [`LightHook`] (one lit, one `OFF_BY_DEFAULT`) under a midnight sky; a target beside
//! each is sampled by `sf_ai_bevy`'s light system. And the sky curve: noon outdoors above midnight.

use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use sf_ai::PerceptTarget;
use sf_ai_bevy::{AiLightExposure, AiLightInputs, Perceivable, PerceivablePose};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_physics::{BodyDesc, Layer, PhysicsParams, PhysicsWorlds, ShapeRef};
use sf_physics_bevy::Physics;
use sf_sky_bevy::CurrentSky;
use sf_sky_bevy::sf_sky::{SkyClock, SkySettings, Weather};
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, ActiveWorldspace, PlacementSpawner, SpawnCx};
use tes3::esp::{Light, LightFlags};

use super::{LightHook, LightInputSettings, LightRecords, exterior_sky, interior_sky, light_record};

const HZ: f64 = 60.0;
/// The `LIGH` radius of both fixtures, game units (256 u = 3.66 m).
const RADIUS_UNITS: u32 = 256;
/// The lamps stand at x = ∓`LAMP_X_M`, each target 1 m toward the middle.
const LAMP_X_M: f32 = 8.0;

/// A spawner that spawns nothing (the hook's inner one).
struct Nothing;
impl PlacementSpawner for Nothing {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
}

fn at(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(x, y, z))
}

fn lamp(radius: u32, flags: LightFlags) -> Light {
    let mut l = Light::default();
    l.data.radius = radius;
    l.data.flags = flags;
    l
}

/// The sky at `hour` of day 0 under clear weather (the framework's default model).
fn sky_at(hour: f64) -> sf_sky_bevy::sf_sky::SkyState {
    let settings = SkySettings::default();
    settings.evaluate(&SkyClock::at(0.0, hour, &settings.calendar), &Weather::default())
}

fn app(ws: &WorldspaceId) -> App {
    let mut physics = Physics { worlds: PhysicsWorlds::default(), params: PhysicsParams::default(), terrain: None };
    let floor = WorldTransform { pos: at(0.0, -0.5, 0.0), ..WorldTransform::IDENTITY };
    physics.insert(ws, &BodyDesc::fixed(floor, ShapeRef::Box(Vec3::new(30.0, 0.5, 30.0)), Layer::STATIC));
    physics.worlds.set_active(Some(ws.clone()));
    physics.worlds.active_world_mut().expect("active world").refresh_queries();

    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), TimePlugin))
        .insert_resource(Time::<Fixed>::from_hz(HZ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / HZ)))
        .insert_resource(physics)
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .insert_resource(CurrentSky(sky_at(0.0)));
    crate::sight::add::<Physics>(&mut app);
    let lights = [("lit_lamp", lamp(RADIUS_UNITS, LightFlags::empty())), ("dark_lamp", lamp(RADIUS_UNITS, LightFlags::OFF_BY_DEFAULT))]
        .into_iter()
        .filter_map(|(id, l)| Some((id.to_owned(), light_record(&l)?)))
        .collect();
    app.insert_resource(LightRecords { lights: Arc::new(lights), ..LightRecords::default() });
    app.update();
    app
}

/// `id` enters R0 through the hook at `origin`.
fn activate(app: &mut App, ws: &WorldspaceId, n: u128, id: &str, origin: WorldPos) {
    let placement = IndexedPlacement {
        placement: Placement {
            id: PlacementId(n),
            base: RecordRef::new("LIGH", id),
            model: None,
            transform: WorldTransform { pos: origin, ..WorldTransform::IDENTITY },
            class: PlacementClass::Light,
            flags: PlacementFlags::empty(),
        },
        radius: 0.0,
    };
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let cx = ActivateCx { worldspace: ws, cell: IVec3::ZERO, cell_entity, discard: false };
    LightHook(Arc::new(Nothing)).activate(world, &cx, &placement);
}

/// A sampled target (an actor) standing at `feet`, 1.8 m tall.
fn target(app: &mut App, key: u128, feet: WorldPos) -> Entity {
    app.world_mut()
        .spawn((
            Perceivable {
                target: PerceptTarget::Actor(ActorKey(key)),
                chest_m: crate::sight::CHEST_RATIO * 1.8,
                head_m: crate::sight::HEAD_RATIO * 1.8,
            },
            PerceivablePose { feet, velocity: Vec3::ZERO },
        ))
        .id()
}

#[test]
fn a_lit_lamp_lights_its_neighbour_and_an_unlit_one_leaves_the_sky() {
    let ws = WorldspaceId::new("light_yard");
    let mut app = app(&ws);
    activate(&mut app, &ws, 1, "Lit_Lamp", at(-LAMP_X_M, 0.0, 0.0));
    activate(&mut app, &ws, 2, "dark_lamp", at(LAMP_X_M, 0.0, 0.0));
    let near_lit = target(&mut app, 11, at(-LAMP_X_M + 1.0, 0.0, 0.0));
    let near_off = target(&mut app, 12, at(LAMP_X_M - 1.0, 0.0, 0.0));
    for _ in 0..3 {
        app.update();
    }

    let inputs = app.world().resource::<AiLightInputs>().clone();
    let exposure = |e: Entity| app.world().get::<AiLightExposure>(e).expect("sampled").0;
    let (lit, off) = (exposure(near_lit), exposure(near_off));
    println!("midnight sky {:.4}; exposure beside the lit lamp {lit:.4}, beside the unlit one {off:.4}", inputs.sky);
    println!("sources: {:?}", inputs.sources);

    assert!(!inputs.interior);
    assert_eq!(inputs.sources.len(), 2, "{:?}", inputs.sources);
    let lift_m = LightInputSettings::default().source_lift_m;
    let lit_src = inputs.sources.iter().find(|s| s.on).expect("the lit lamp");
    assert_eq!(lit_src.pos, at(-LAMP_X_M, lift_m, 0.0));
    assert!((lit_src.radius_m - mw_world::convert::meters(256.0)).abs() < 1e-6, "{lit_src:?}");
    assert_eq!(inputs.sources.iter().filter(|s| !s.on).count(), 1, "the OFF_BY_DEFAULT lamp is listed, off");
    assert!(inputs.sky < 0.15, "midnight reads dark: {}", inputs.sky);
    assert!(lit > 0.5, "beside a lit lamp: {lit}");
    // Exact: the falloff window `(1 - (d/r)^k)^2` (k = 2, sf_ai's default) from the chest to the
    // lifted source, plus the sky term.
    let chest = at(-LAMP_X_M + 1.0, crate::sight::CHEST_RATIO * 1.8, 0.0);
    let d = chest.delta_f32(lit_src.pos).length();
    let window = (1.0 - (d / lit_src.radius_m).powf(sf_ai::LightConfig::default().light_falloff_exp)).max(0.0).powi(2);
    assert!((lit - (window + inputs.sky).min(1.0)).abs() < 1e-5, "{lit} vs {window} + {}", inputs.sky);
    assert!((off - inputs.sky).abs() < 1e-6, "beside an unlit lamp the sky term alone: {off} vs {}", inputs.sky);
}

#[test]
fn noon_outdoors_reads_brighter_than_midnight() {
    let s = LightInputSettings::default();
    let (noon, midnight) = (exterior_sky(&sky_at(12.0), &s), exterior_sky(&sky_at(0.0), &s));
    let (lit_room, cave) = (interior_sky(Some([0.5; 3]), &s), interior_sky(Some([0.08; 3]), &s));
    println!("sky level: noon {noon:.4}, midnight {midnight:.4}; interior AMBI 0.5 grey {lit_room:.4}, 0.08 grey {cave:.4}");
    let hours: Vec<String> = [7.5, 7.75, 8.0, 9.0, 15.0, 16.0, 16.25, 16.5].iter().map(|&h| format!("{h}h {:.3}", exterior_sky(&sky_at(h), &s))).collect();
    println!("sky level by hour (day 0, a winter day: sun up ~7.7-16.3 h, clear): {}", hours.join(", "));
    assert!(noon > midnight, "noon {noon} <= midnight {midnight}");
    assert!((0.0..=1.0).contains(&noon) && (0.0..=1.0).contains(&midnight));
    assert!(lit_room > cave);
}
