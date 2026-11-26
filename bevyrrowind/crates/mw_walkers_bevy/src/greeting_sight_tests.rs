//! `AI-13`: the walker greeting ([`crate::greeting`]) with the game's real sight wiring
//! ([`crate::sight::add`] on `sf_physics_bevy::Physics`, as `super::sight_tests` builds it) on the
//! wander drive's harness ([`super::drive_tests`], which adds the greeting and the perception
//! reactions). No faked awareness: the NPC's meter fills from its own sight checks. Light: the
//! headless default (no `CurrentSky`: the sky reads 1, day; the floor is the only collider, so the
//! roof ray finds the open sky).

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use mw_game_core::player::{HOST_KEY, PlayerBody};
use sf_action_bevy::ActionBody;
use sf_ai::{AwarenessBand, PerceptTarget};
use sf_ai_bevy::{AiActor, AiAwareness};
use sf_controller::Locomotion;
use sf_controller_bevy::{MoveIntent, Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_nav::QueryFilter;
use sf_nav_bevy::NavCorridor;
use sf_physics::{BodyDesc, Layer, PhysicsParams, PhysicsWorlds, ShapeRef};
use sf_physics_bevy::Physics;

use super::ActorSpace;
use super::drive_tests::{SPEED, app_with, home, nav_ws};
use super::greeting_tests::{Voice, hello_sheet, hello_table_and_voice};
use crate::greeting::Greeting;
use crate::reaction::{Phase, Reaction};

const HZ: usize = 60;
const PLAYER: PerceptTarget = PerceptTarget::Player(HOST_KEY);
/// Hello 30 x `iGreetDistanceMultiplier` 6 = 2.57 m: the player stands 1.5 m away.
const NEAR_M: f32 = 1.5;
/// The NPC's key.
const KEY: u128 = 2;

fn at(p: WorldPos) -> WorldTransform {
    WorldTransform { pos: p, ..WorldTransform::IDENTITY }
}

/// The drive harness with the game's sight on a physics floor (top at y = 0, under the whole
/// navmesh room), the Hello table and the test voice.
fn app() -> App {
    app_with(|app| {
        let ws = nav_ws();
        let mut physics = Physics { worlds: PhysicsWorlds::default(), params: PhysicsParams::default(), terrain: None };
        let floor = BodyDesc::fixed(at(home().offset(Vec3::new(0.0, -0.5, 0.0))), ShapeRef::Box(Vec3::new(30.0, 0.5, 30.0)), Layer::STATIC);
        physics.insert(&ws, &floor);
        physics.worlds.set_active(Some(ws));
        physics.worlds.active_world_mut().expect("active world").refresh_queries();
        app.insert_resource(physics);
        crate::sight::add::<Physics>(app);
        hello_table_and_voice(app, true);
    })
}

/// A placed NPC on post as `attach` makes one (mover, corridor, reactions, sight; no `Walker`, no
/// directive) with Hello 30, at home facing `yaw`.
fn npc(app: &mut App, yaw: f32) -> Entity {
    let ws = nav_ws();
    let filter = QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID);
    let params = MoverParams::default();
    let sheet = hello_sheet(app, KEY, 30.0);
    app.world_mut()
        .spawn((
            Mover::new(home(), yaw, params, WorldPos::ORIGIN),
            MoveIntent::default(),
            ActorSpace(ws.clone()),
            NavCorridor::new(ActorKey(KEY), mw_bake::nav::HUMANOID, ws, SPEED).with_filter(filter),
            Reaction::new(SPEED),
            AiActor(ActorKey(KEY)),
            crate::sight::observer(params.height),
            sheet,
        ))
        .id()
}

/// The player's body as `mw_player_bevy` spawns it, standing at `offset` from home.
fn player(app: &mut App, offset: Vec3) -> Entity {
    let body = ActionBody::new(Locomotion::new(home().offset(offset), 0.0), MoverParams::default(), WorldPos::ORIGIN);
    app.world_mut().spawn((PlayerBody, body)).id()
}

/// What one tick showed.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Sample {
    band: AwarenessBand,
    phase: Phase,
    asks: u32,
}

fn sample(app: &App, e: Entity) -> Sample {
    let w = app.world();
    Sample {
        band: w.get::<AiAwareness>(e).expect("observer brings its meter").0.band(PLAYER),
        phase: w.get::<Reaction>(e).expect("reactions").phase(),
        asks: w.resource::<Voice>().asks,
    }
}

/// `ticks` fixed ticks; prints each change of band, reaction phase or asks with its tick.
fn run(app: &mut App, e: Entity, ticks: usize) -> Vec<Sample> {
    let mut out: Vec<Sample> = Vec::with_capacity(ticks);
    for t in 0..ticks {
        app.update();
        let s = sample(app, e);
        if out.last() != Some(&s) {
            let level = app.world().get::<AiAwareness>(e).unwrap().0.get(PLAYER).map_or(0.0, |a| a.level);
            let g = app.world().get::<Greeting>(e).map(|g| g.greet.state);
            eprintln!("tick {t}: {s:?} level {level:.3} greeting {g:?}");
        }
        out.push(s);
    }
    out
}

#[allow(clippy::cast_possible_truncation)]
fn angle_diff(a: f32, b: f32) -> f32 {
    mw_ai::settings::normalize_angle(f64::from(a - b)).abs() as f32
}

/// The player in plain view 1.5 m from an NPC on post, 70 degrees off its facing (inside the
/// peripheral cone, past the 60 degrees a smooth greeting turn needs), at noon in the open: it
/// greets once, holds still and turns to face the player.
///
/// Timeline before the `AI-13` fix (sight period 6 ticks at 60 Hz; the band rises one step per
/// check at 1.5 m), player straight ahead:
/// tick 6 Curious (level 0.2), tick 12 Suspicious (0.4), tick 18 Searching (0.7, the reaction owns:
/// `Phase::Search`), tick 24 Alerted (1.0, latched; the search hands the actor back: `Routine`),
/// then Alerted + Routine for the rest of the 10 s, greeting `None`, zero Hello asks: the greeting
/// read Alerted as "in combat" and reset every tick.
///
/// After the fix, at 70 degrees: tick 12 Curious, 30 the reaction turns (`React`), 36 Suspicious,
/// 54 Searching (`Search`), 60 Alerted (`Return`: back to its post and yaw), 117 `Routine`, 135
/// Hello asked (two full greeting updates later): 123 ticks, about 2 s, from first sight.
#[test]
fn an_npc_that_sees_the_player_in_plain_view_greets_once_facing_it() {
    let mut app = app();
    let e = npc(&mut app, 0.0);
    let off = 70.0_f32.to_radians();
    let offset = Vec3::new(off.sin(), 0.0, off.cos()) * NEAR_M;
    player(&mut app, offset);
    let samples = run(&mut app, e, 10 * HZ);

    let asked = samples.iter().position(|s| s.asks >= 1).expect("no Hello asked in 10 s");
    let first_sight = samples.iter().position(|s| s.band != AwarenessBand::Unaware).expect("never saw the player");
    eprintln!("first sight at tick {first_sight}, Hello asked at tick {asked}");
    assert_eq!(samples.last().unwrap().asks, 1, "greeted again while the player stayed");
    assert!(samples[asked..].iter().all(|s| s.band == AwarenessBand::Alerted && s.phase == Phase::Routine), "{:?}", &samples[asked..]);
}

/// The greeting's hold and turn, checked tick by tick from the ask (as `run`, with the hold).
#[test]
fn a_greeting_after_real_sight_holds_still_and_turns_to_the_player() {
    let mut app = app();
    let e = npc(&mut app, 0.0);
    let off = 70.0_f32.to_radians();
    player(&mut app, Vec3::new(off.sin(), 0.0, off.cos()) * NEAR_M);
    let mut asked = None;
    for t in 0..10 * HZ {
        app.update();
        if app.world().resource::<Voice>().asks >= 1 {
            asked = Some(t);
            break;
        }
    }
    asked.expect("no Hello asked in 10 s");
    let g = app.world().get::<Greeting>(e).unwrap();
    assert!(g.in_progress() && g.holds(), "{g:?}");
    assert_eq!(app.world().get::<MoveIntent>(e).unwrap().0.desired_velocity, Vec3::ZERO, "it stops");

    let at = app.world().get::<Mover>(e).unwrap().loco.position;
    for _ in 0..2 * HZ {
        app.update();
        assert!(app.world().get::<Greeting>(e).unwrap().holds(), "the hold lasts the greeting");
    }
    let m = app.world().get::<Mover>(e).unwrap();
    assert!(at.delta_f32(m.loco.position).with_y(0.0).length() < 0.05, "it moved while greeting");
    let to_player = m.loco.position.delta_f32(home().offset(Vec3::new(off.sin(), 0.0, off.cos()) * NEAR_M));
    let facing = to_player.x.atan2(to_player.z);
    assert!(angle_diff(m.loco.motion.yaw, facing) < 0.1, "yaw {} vs {facing}", m.loco.motion.yaw);
}
