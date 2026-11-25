//! Headless App tests of the player's visibility (`MA18`, [`crate::visibility`]) through the game's
//! sight wiring ([`crate::sight::add`] on `sf_physics_bevy::Physics`): the player body (`PlayerBody`
//! with its `ActionBody` and `ActionCharacter`, its mode set by hand: no chooser in the app) and the
//! host's sheet (`PlayerCharacter` + `Stats` on a one-stat table). The dim room is an interior whose
//! ambient puts the player in half light, as the perception lab's dark lane does.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use mw_game_core::player::{HOST_KEY, PlayerBody, PlayerCharacter};
use sf_action::{ActionState, CharacterId, CharacterParams, Mode, RuleKey};
use sf_action_bevy::{ActionBody, ActionCharacter};
use sf_ai::{PerceptTarget, Stance, StatId};
use sf_ai_bevy::{AiActor, AiAwareness, AiLightExposure, AiVisibilityFactors};
use sf_controller::Locomotion;
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_physics::{BodyDesc, Layer, PhysicsParams, PhysicsWorlds, ShapeRef};
use sf_physics_bevy::Physics;
use sf_stats_bevy::sf_stats::{
    ApplyCtx, Cause, CauseKind, Inputs, Settle, Sheet, StatDef, StatIdx, StatKind, StatOp, StatOwner, StatTable, TestModel,
};
use sf_stats_bevy::{StatTableRes, Stats};
use sf_world::WorldspaceId;
use sf_world_bevy::ActiveWorldspace;

use super::SNEAK_SKILL;
use crate::light_inputs::LightRecords;

const HZ: f64 = 60.0;
const PLAYER: PerceptTarget = PerceptTarget::Player(HOST_KEY);
/// The player stands this far ahead of the guard (+Z, inside the default 30 m sight range).
const DIST_M: f32 = 10.0;
/// The room's `AMBI` ambient (sRGB): a dim interior, the player in half light.
const DIM_AMBIENT: [f32; 3] = [0.35, 0.35, 0.35];
/// The test sheet's Sneak skill.
const SNEAK: f32 = 47.0;
/// The one-stat table's Sneak index.
const SNEAK_IDX: StatIdx = StatIdx(0);

fn at(x: f32, y: f32, z: f32) -> WorldTransform {
    WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(x, y, z)), ..WorldTransform::IDENTITY }
}

/// The game's sight wiring in a floored room; `dim`: an interior with [`DIM_AMBIENT`] (else no
/// light inputs' worldspace: full light).
fn app(dim: bool) -> App {
    let ws = WorldspaceId::new("sneak_room");
    let mut physics = Physics { worlds: PhysicsWorlds::default(), params: PhysicsParams::default(), terrain: None };
    physics.insert(&ws, &BodyDesc::fixed(at(0.0, -0.5, 0.0), ShapeRef::Box(Vec3::new(30.0, 0.5, 30.0)), Layer::STATIC));
    physics.worlds.set_active(Some(ws.clone()));
    physics.worlds.active_world_mut().expect("active world").refresh_queries();

    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), TimePlugin))
        .insert_resource(Time::<Fixed>::from_hz(HZ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / HZ)))
        .insert_resource(physics)
        .insert_resource(ActiveWorldspace(Some(ws.clone())));
    crate::sight::add::<Physics>(&mut app);
    if dim {
        let interiors = HashMap::from([(ws, Some(DIM_AMBIENT))]);
        app.insert_resource(LightRecords { interiors: Arc::new(interiors), ..LightRecords::default() });
    }
    app.update();
    app
}

/// A one-stat table (Sneak) and the host's sheet with [`SNEAK`].
fn sheet(app: &mut App) -> Entity {
    let def = StatDef {
        key: SNEAK_SKILL.into(),
        kind: StatKind::Value { min: 0.0, cap: None },
        group: "skill".into(),
        public: true,
        label: String::new(),
    };
    let table = StatTable::new(vec![def], &[]).expect("table builds");
    let rules = TestModel::default().rules();
    let owner = StatOwner::Player(HOST_KEY);
    let inputs = Inputs::default();
    let mut s = Sheet::new(&table);
    let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Rules, source: None, tick: 0 }, inputs: &inputs };
    let ops = [StatOp::SetBase { stat: SNEAK_IDX, value: SNEAK }];
    assert_eq!(sf_stats_bevy::sf_stats::apply(&table, &rules, &mut s, &ops, &ctx, &mut Vec::new()), 0);
    s.settle(&table, &rules, &inputs, Settle::NewCharacter);
    app.insert_resource(StatTableRes(Arc::new(table)));
    app.world_mut().spawn((PlayerCharacter { key: HOST_KEY }, Stats::new(owner, s, inputs))).id()
}

/// A placed guard at the origin facing +Z, built as `attach` builds it (awake).
fn guard(app: &mut App) -> Entity {
    let params = MoverParams::default();
    app.world_mut()
        .spawn((Mover::new(WorldPos::ORIGIN, 0.0, params, WorldPos::ORIGIN), AiActor(ActorKey(7)), crate::sight::observer(params.height)))
        .id()
}

/// The player's body `DIST_M` ahead of the guard, facing it, in `mode`.
fn player(app: &mut App, mode: Mode) -> Entity {
    let feet = WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, DIST_M));
    let body = ActionBody::new(Locomotion::new(feet, std::f32::consts::PI), MoverParams::default(), WorldPos::ORIGIN);
    let mut character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    character.state.mode.mode = mode;
    app.world_mut().spawn((PlayerBody, body, character)).id()
}

fn set_mode(app: &mut App, p: Entity, mode: Mode) {
    app.world_mut().get_mut::<ActionCharacter>(p).expect("player chooser").state.mode.mode = mode;
}

fn factors(app: &App, p: Entity) -> AiVisibilityFactors {
    *app.world().get::<AiVisibilityFactors>(p).expect("the wiring gave the body its factors")
}

fn level(app: &App, guard: Entity) -> f32 {
    app.world().get::<AiAwareness>(guard).expect("observer brings its meter").0.get(PLAYER).map_or(0.0, |a| a.level)
}

/// The guard's awareness after `frames` frames (one fixed tick each) with the player in `mode`.
fn awareness_after(mode: Mode, frames: usize) -> (f32, Option<f32>) {
    let mut app = app(true);
    let g = guard(&mut app);
    let p = player(&mut app, mode);
    for _ in 0..frames {
        app.update();
    }
    (level(&app, g), app.world().get::<AiLightExposure>(p).map(|l| l.0))
}

#[test]
fn sneak_mode_crouches_the_player_and_leaving_it_stands_them_up() {
    let mut app = app(false);
    let p = player(&mut app, Mode::Normal);
    app.update();
    assert_eq!(factors(&app, p).stance, Stance::Upright);

    set_mode(&mut app, p, Mode::Sneak);
    app.update();
    let f = factors(&app, p);
    assert_eq!(f.stance, Stance::Crouched);
    assert_eq!((f.size, f.disguise), (1.0, 0.0), "only the stance moves: {f:?}");

    // Unchanged state: no write (change detection sees mode changes only).
    let tick = app.world().get_entity(p).unwrap().get_change_ticks::<AiVisibilityFactors>().unwrap().changed;
    app.update();
    let again = app.world().get_entity(p).unwrap().get_change_ticks::<AiVisibilityFactors>().unwrap().changed;
    assert_eq!(tick, again, "rewritten without a change");

    set_mode(&mut app, p, Mode::FreeRun);
    app.update();
    assert_eq!(factors(&app, p).stance, Stance::Upright, "free-run is upright");
    set_mode(&mut app, p, Mode::Sneak);
    app.update();
    set_mode(&mut app, p, Mode::Normal);
    app.update();
    assert_eq!(factors(&app, p).stance, Stance::Upright);
}

#[test]
fn modifiers_carry_the_sneak_skill() {
    let mut app = app(false);
    sheet(&mut app);
    let p = player(&mut app, Mode::Sneak);
    app.update();
    let mods: Vec<_> = factors(&app, p).modifiers.into_iter().flatten().collect();
    // No Chameleon stat in the table: Sneak alone.
    assert_eq!(mods, vec![(StatId(SNEAK_IDX.0), SNEAK)]);
}

#[test]
fn without_a_sheet_there_are_no_modifiers() {
    let mut app = app(false);
    let p = player(&mut app, Mode::Sneak);
    app.update();
    assert!(factors(&app, p).modifiers.iter().all(Option::is_none));
}

#[test]
fn a_guard_grows_less_aware_of_a_sneaking_player_in_half_light() {
    // Two seconds facing the player, 10 m away, in a dim room.
    let frames = 120;
    let (standing, light) = awareness_after(Mode::Normal, frames);
    let (sneaking, _) = awareness_after(Mode::Sneak, frames);
    let light = light.expect("the dim room samples the player's light");
    assert!(light > 0.0 && light < 1.0, "not half light: {light}");
    assert!(standing > 0.0, "the guard never saw the standing player (light {light})");
    assert!(sneaking < standing, "sneaking {sneaking} not below standing {standing} (light {light})");
}

#[test]
#[ignore = "measurement: the guard's awareness of a standing and a sneaking player in the dim room, per half second"]
fn measure_awareness_standing_and_sneaking_in_half_light() {
    for frames in [30, 60, 90, 120, 180, 240] {
        let (standing, light) = awareness_after(Mode::Normal, frames);
        let (sneaking, _) = awareness_after(Mode::Sneak, frames);
        println!("{frames:>3} ticks: standing {standing:.3}, sneaking {sneaking:.3} (light {light:?})");
    }
}
