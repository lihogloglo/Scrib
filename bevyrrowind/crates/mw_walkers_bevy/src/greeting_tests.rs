//! `MA4c2`: the walker greeting ([`crate::greeting`]) on the wander drive's harness
//! ([`super::drive_tests`]: the real systems on the fixed tick over a flat navmesh room), the
//! actor seeded as `seed_directives` does ([`super::directive_drive_tests::spawn`]). A test voice
//! answers the Hello requests between Decide and Apply, as `mw_audio_bevy::voices` does.

use std::f32::consts::{FRAC_PI_2, PI};
use std::sync::Arc;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use mw_game_core::player::{HOST_KEY, PlayerBody};
use sf_action_bevy::ActionBody;
use sf_ai::{AwarenessBand, DirectiveKind, LastSeen, PerceptClock, PerceptTarget, Stimulus};
use sf_ai_bevy::{AiAwareness, AiPerceptClock, AiPerceptionSettings};
use sf_controller::Locomotion;
use sf_controller_bevy::{MoveIntent, Mover, MoverMode, MoverParams};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_stats_bevy::sf_stats::{
    ApplyCtx, Cause, CauseKind, Inputs, Settle, Sheet, StatDef, StatIdx, StatKind, StatOp, StatOwner, StatTable, TestModel,
};
use sf_stats_bevy::{StatTableRes, Stats};
use sf_world::Location;

use super::Walker;
use super::directive_drive_tests::{HZ, spawn};
use super::drive_tests::{app_with, home, nav_ws};
use crate::greeting::{GreetSystems, Greeting, GreetingState, HELLO, HelloAsk};
use crate::physics::Dead;

const PLAYER: PerceptTarget = PerceptTarget::Player(HOST_KEY);
/// The sight clock's (frozen) tick: nothing advances it on this harness.
const TICK: u64 = 100;
/// Ticks of one full greeting update (`sUpdateHelloInterval` 0.25 s at 60 Hz, plus rounding).
const FULL: usize = 16;
/// Hello 30 x `iGreetDistanceMultiplier` 6 = 180 units = 2.57 m: the player stands 1.5 m away.
const NEAR_M: f32 = 1.5;
/// Past `fGreetDistanceReset` (512 units = 7.3 m).
const AWAY_M: f32 = 12.0;

/// The test voice: says (or refuses) every asked Hello and counts the asks.
#[derive(Resource)]
struct Voice {
    say: bool,
    asks: u32,
}

fn answer(mut voice: ResMut<Voice>, mut greetings: Query<&mut Greeting>) {
    for mut g in &mut greetings {
        if g.hello == HelloAsk::Asked {
            voice.asks += 1;
            let say = voice.say;
            g.answer_hello(say);
        }
    }
}

/// The harness with a one-stat table (Hello), the sight clock, the test voice and the host body
/// far away.
fn app(say: bool) -> (App, Entity) {
    let mut app = app_with(|app| {
        let def = StatDef { key: HELLO.into(), kind: StatKind::Value { min: 0.0, cap: None }, group: "ai".into(), public: false, label: String::new() };
        let table = StatTable::new(vec![def], &[]).expect("table builds");
        app.insert_resource(StatTableRes(Arc::new(table)))
            .insert_resource(AiPerceptClock { tick: TICK, tick_s: 1.0 / 60.0 })
            .init_resource::<AiPerceptionSettings>()
            .insert_resource(Voice { say, asks: 0 })
            .add_systems(FixedUpdate, answer.after(GreetSystems::Decide).before(GreetSystems::Apply));
    });
    let feet = home().offset(Vec3::new(0.0, 0.0, AWAY_M + 10.0));
    let body = ActionBody::new(Locomotion::new(feet, 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let player = app.world_mut().spawn((PlayerBody, body)).id();
    (app, player)
}

/// An NPC with `hello` on its sheet, its stack holding `before` above the record wander.
fn npc(app: &mut App, key: u128, hello: f32, before: Vec<DirectiveKind>) -> Entity {
    let (e, _) = spawn(app, key, before);
    let table = Arc::clone(&app.world().resource::<StatTableRes>().0);
    let rules = TestModel::default().rules();
    let owner = StatOwner::Actor(ActorKey(key));
    let inputs = Inputs::default();
    let mut sheet = Sheet::new(&table);
    let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Rules, source: None, tick: 0 }, inputs: &inputs };
    let ops = [StatOp::SetBase { stat: StatIdx(0), value: hello }];
    assert_eq!(sf_stats_bevy::sf_stats::apply(&table, &rules, &mut sheet, &ops, &ctx, &mut Vec::new()), 0);
    sheet.settle(&table, &rules, &inputs, Settle::NewCharacter);
    app.world_mut().entity_mut(e).insert((Stats::new(owner, sheet, inputs), seen()));
    e
}

/// An awareness meter that just saw the player (a faint sight: the band stays Unaware).
fn seen() -> AiAwareness {
    let cfg = AiPerceptionSettings::default().awareness;
    let mut meter = AiAwareness::default();
    let sight = Stimulus::Sight { target: PLAYER, gain: 0.01, pos: home(), velocity: Vec3::ZERO };
    meter.0.apply(&cfg, &sight, PerceptClock::new(TICK, 1.0 / 60.0));
    assert_eq!(meter.0.band(PLAYER), AwarenessBand::Unaware);
    meter
}

fn mover(app: &App, e: Entity) -> &Mover {
    app.world().get::<Mover>(e).expect("a mover")
}

fn greeting(app: &App, e: Entity) -> &Greeting {
    app.world().get::<Greeting>(e).expect("every AI actor with a mover greets")
}

fn asks(app: &App) -> u32 {
    app.world().resource::<Voice>().asks
}

/// Puts the player at `offset` from the NPC's feet.
fn place(app: &mut App, player: Entity, npc: Entity, offset: Vec3) {
    let at = mover(app, npc).loco.position.offset(offset);
    app.world_mut().get_mut::<ActionBody>(player).expect("player body").loco.position = at;
}

/// `ticks` fixed ticks with the player kept at `offset` from the NPC.
fn follow(app: &mut App, player: Entity, npc: Entity, offset: Vec3, ticks: usize) {
    for _ in 0..ticks {
        place(app, player, npc, offset);
        app.update();
    }
}

/// Ticks until the voice was asked `n` times, the player kept at `offset` (`None` past `max`).
fn until_asked(app: &mut App, player: Entity, npc: Entity, offset: Vec3, n: u32, max: usize) -> Option<usize> {
    for tick in 0..max {
        place(app, player, npc, offset);
        app.update();
        if asks(app) >= n {
            return Some(tick);
        }
    }
    None
}

#[allow(clippy::cast_possible_truncation)]
fn angle_diff(a: f32, b: f32) -> f32 {
    mw_ai::settings::normalize_angle(f64::from(a - b)).abs() as f32
}

/// Approach, Hello once, stand facing the player, resume, re-greet only after leaving.
fn greets_once_facing(offset: Vec3, facing_yaw: f32) {
    let (mut app, player) = app(true);
    let e = npc(&mut app, 1, 30.0, vec![]);
    // Far away: no greeting.
    follow(&mut app, player, e, Vec3::Z * AWAY_M, HZ);
    assert_eq!((asks(&app), greeting(&app, e).greet.state), (0, GreetingState::None));

    // Within reach and seen, its back to the player (with smooth movement a greeting turn starts
    // only past 60 degrees): Hello after the initial delay (2 full updates).
    let mut tick = None;
    for t in 0..4 * FULL {
        place(&mut app, player, e, offset);
        app.world_mut().get_mut::<Mover>(e).unwrap().loco.motion.yaw = facing_yaw + PI;
        app.update();
        if asks(&app) == 1 {
            tick = Some(t);
            break;
        }
    }
    let tick = tick.expect("the NPC never greeted");
    // The second full update in reach comes one interval (15 ticks) after the first.
    assert!(tick + 1 >= FULL - 1, "greeted after {} ticks, before two full updates", tick + 1);
    let g = greeting(&app, e);
    assert!(g.in_progress() && g.holds(), "{g:?}");
    assert_eq!(g.stats.greeted, 1);
    let intent = app.world().get::<MoveIntent>(e).unwrap().0;
    assert_eq!(intent.desired_velocity, Vec3::ZERO, "it stops");
    let (stuck, arrivals) = {
        let w = app.world().get::<Walker>(e).unwrap();
        (w.driver.stuck_count(), w.driver.arrival_count())
    };

    // It turns its body to the player and stands there.
    let mut at = mover(&app, e).loco.position;
    for t in 0..2 * HZ {
        place(&mut app, player, e, offset);
        app.update();
        assert!(greeting(&app, e).holds(), "the hold lasts between full updates");
        if t + 1 == HZ {
            // Stopped from a walk by now: it stands from here on.
            at = mover(&app, e).loco.position;
        }
    }
    let m = mover(&app, e);
    assert!(angle_diff(m.loco.motion.yaw, facing_yaw) < 0.1, "yaw {} vs {facing_yaw}", m.loco.motion.yaw);
    let facing = app.world().get::<MoveIntent>(e).unwrap().0.desired_facing.expect("a stand facing");
    assert!(angle_diff(facing, facing_yaw) < 0.1, "desired facing {facing}");
    assert!(at.delta_f32(m.loco.position).with_y(0.0).length() < 0.05, "it moved while greeting");
    assert!(greeting(&app, e).in_progress());

    // After iGreetDuration (4 s from the Hello): Done, the routine resumes.
    follow(&mut app, player, e, offset, 3 * HZ);
    let g = greeting(&app, e);
    assert_eq!(g.greet.state, GreetingState::Done);
    assert!(!g.holds());
    follow(&mut app, player, e, offset, 40 * HZ);
    let w = app.world().get::<Walker>(e).unwrap();
    assert!(w.driver.arrival_count() > arrivals, "the wander never resumed");
    assert_eq!(w.driver.stuck_count(), stuck, "the greeting stalled the wander's walk");
    assert_eq!(asks(&app), 1, "greeted twice without the player leaving");

    // Past the reset distance and back: greeted again.
    follow(&mut app, player, e, Vec3::Z * AWAY_M, 2 * FULL);
    assert_eq!(greeting(&app, e).greet.state, GreetingState::None);
    until_asked(&mut app, player, e, offset, 2, 4 * FULL).expect("no second greeting after the player came back");
}

/// The player north of the NPC (-Z): it ends facing north (engine yaw PI).
#[test]
fn greets_once_facing_a_player_to_the_north() {
    greets_once_facing(Vec3::NEG_Z * NEAR_M, PI);
}

/// The player east of the NPC (+X): it ends facing east (engine yaw PI/2).
#[test]
fn greets_once_facing_a_player_to_the_east() {
    greets_once_facing(Vec3::X * NEAR_M, FRAC_PI_2);
}

/// No voice answer (already speaking, no Hello line): the greeting goes Done, no stop, no turn.
#[test]
fn a_greeting_that_cannot_say_hello_is_done_without_turning() {
    let (mut app, player) = app(false);
    let e = npc(&mut app, 1, 30.0, vec![]);
    until_asked(&mut app, player, e, Vec3::NEG_Z * NEAR_M, 1, 4 * FULL).expect("asked");
    let g = greeting(&app, e);
    assert_eq!(g.greet.state, GreetingState::Done);
    assert!(!g.holds() && g.action().turn_to.is_none(), "{g:?}");
    assert_eq!((g.stats.greeted, g.stats.silent), (0, 1));
    follow(&mut app, player, e, Vec3::NEG_Z * NEAR_M, 2 * HZ);
    let g = greeting(&app, e);
    assert!(!g.holds() && g.action().turn_to.is_none());
    assert_eq!(asks(&app), 1, "no retry while the player stays");
}

/// The asks over 3 s with the player kept near an NPC set up by `setup`.
fn asks_with(before: Vec<DirectiveKind>, setup: impl FnOnce(&mut App, Entity, Entity)) -> (u32, GreetingState) {
    let (mut app, player) = app(true);
    let e = npc(&mut app, 1, 30.0, before);
    setup(&mut app, e, player);
    follow(&mut app, player, e, Vec3::NEG_Z * NEAR_M, 3 * HZ);
    (asks(&app), greeting(&app, e).greet.state)
}

#[test]
fn gates_combat_swimming_package_unseen_dead_player() {
    let none = (0, GreetingState::None);
    let alerted = asks_with(vec![], |app, e, _| {
        let cfg = AiPerceptionSettings::default().awareness;
        let seen = LastSeen { pos: home(), tick: TICK, velocity: Vec3::ZERO, confidence: 1.0 };
        let mut a = app.world_mut().get_mut::<AiAwareness>(e).unwrap();
        a.0.raise_to_floor(&cfg, PLAYER, AwarenessBand::Alerted, seen, PerceptClock::new(TICK, 1.0 / 60.0));
    });
    assert_eq!(alerted, none, "in combat with the player");
    let swimming = asks_with(vec![], |app, e, _| {
        app.world_mut().get_mut::<Mover>(e).unwrap().loco.mode = MoverMode::Swim { surface: 1.0 };
    });
    assert_eq!(swimming, none, "swimming");
    let follow_package = asks_with(vec![DirectiveKind::Follow(ActorKey(99))], |_, _, _| {});
    assert_eq!(follow_package, none, "a Follow package serving");
    let unseen = asks_with(vec![], |app, e, _| {
        app.world_mut().entity_mut(e).remove::<AiAwareness>();
    });
    assert_eq!(unseen, none, "unseen");
    let dead = asks_with(vec![], |app, _, player| {
        app.world_mut().entity_mut(player).insert(Dead);
    });
    assert_eq!(dead, none, "the player is dead");
    // A Travel package (`GoTo`) greets, as a Wander does.
    let travel = asks_with(vec![DirectiveKind::GoTo(Location::new(nav_ws(), home().offset(Vec3::X * 8.0)))], |_, _, _| {});
    assert_eq!(travel.0, 1, "a GoTo serving greets");
    // Hello 0 never greets.
    let (mut app, player) = app(true);
    let e = npc(&mut app, 1, 0.0, vec![]);
    follow(&mut app, player, e, Vec3::NEG_Z * 0.3, 3 * HZ);
    assert_eq!(asks(&app), 0, "Hello 0");
}
