//! `MA2c2`: a script's AI push lands on the actor's directive stack and the AI reads follow the
//! drive ([`crate::script_ai`]), on the directive drive's harness ([`super::directive_drive_tests`]:
//! the real systems on the fixed tick over a flat 40 m navmesh room).

use bevy_math::Vec3;
use mw_world::convert::UNIT_METERS_F64;
use sf_ai::{DirectiveDone, DirectiveEnd, DirectiveId, DirectiveKind, DirectiveSource, GameTime};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_world::Location;
use tes3::esp::{AiPackage, AiTravelPackage};

use super::directive_drive_tests::{HZ, spawn, stack};
use super::drive_tests::{app_with, home, nav_ws};
use crate::script_ai::{AiPackageKind, AiRun, AiRuns, ScriptAiPush, Served, next_run};

/// `AITravel`'s point in Morrowind units: 8 m east of home (engine `(x, y, z)` is units
/// `(x, -z, y)`).
#[allow(clippy::cast_possible_truncation)]
fn travel_units(at: WorldPos) -> [f32; 3] {
    let p = at.to_f64();
    [(p.x / UNIT_METERS_F64) as f32, (-p.z / UNIT_METERS_F64) as f32, (p.y / UNIT_METERS_F64) as f32]
}

fn run(app: &bevy_app::App, key: u128) -> AiRun {
    app.world().resource::<AiRuns>().get(ActorKey(key))
}

/// `"npc"->AITravel x y z` on an actor whose record has an `AI_W`: the travel cancels the wander
/// (`MorrowindPolicy`, `OpenMW` `AiSequence::stack`) and is the whole stack, with a script source;
/// the actor walks there; `GetCurrentAIPackage` reads Travel while it walks and
/// `GetAIPackageDone` stays 0, then reads 1 once it arrived (the empty stack keeps it; the last
/// package becomes none).
#[test]
fn a_script_travel_replaces_the_record_wander_walks_there_and_reads_done() {
    let mut app = app_with(|_| {});
    let (e, ids) = spawn(&mut app, 1, Vec::new());
    app.update();
    assert_eq!(run(&app, 1), AiRun { done: false, last: Some(AiPackageKind::Wander) }, "the record wander runs");

    let point = home().offset(Vec3::new(8.0, 0.0, 0.0));
    let package = AiPackage::Travel(AiTravelPackage { location: travel_units(point), reset: 0 });
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(1), package });
    app.update();
    let s = stack(&app, 1);
    assert_eq!(s.len(), 1, "the travel cancelled the record wander");
    assert!(s.get(ids[0]).is_none());
    let top = s.top().expect("the travel");
    assert_eq!(top.spec.source, DirectiveSource::Script);
    let DirectiveKind::GoTo(goal) = &top.spec.kind else { panic!("a go_to, got {:?}", top.spec.kind) };
    assert_eq!(goal.worldspace, nav_ws());
    assert!(goal.pos.delta_f32(point).length() < 0.01, "the goal is the pushed point");
    let travel = top.id;

    let mut arrived = None;
    for tick in 0..(20 * HZ) {
        app.update();
        let pos = app.world().get::<Mover>(e).unwrap().loco.position;
        assert!(home().delta_f32(pos).is_finite(), "tick {tick}: NaN position");
        if stack(&app, 1).get(travel).is_none() {
            arrived = Some((tick, pos));
            break;
        }
        assert_eq!(run(&app, 1), AiRun { done: false, last: Some(AiPackageKind::Travel) }, "tick {tick}: walking");
    }
    let (tick, at) = arrived.expect("the travel never finished");
    let off = at.delta_f32(point).with_y(0.0).length();
    assert!(off <= 2.0, "finished {off} m from the point");
    assert!(tick > 3 * HZ, "8 m at 1.5 m/s took {tick} ticks");
    assert!(stack(&app, 1).is_empty(), "nothing was left under the travel");
    assert_eq!(run(&app, 1), AiRun { done: true, last: Some(AiPackageKind::Travel) }, "the arrival tick");
    app.update();
    assert_eq!(run(&app, 1), AiRun { done: true, last: None }, "an empty stack keeps done, runs nothing");
}

/// A push for a key with no walker, or before the actor's stack is seeded, changes nothing.
#[test]
fn a_push_without_a_walker_or_a_stack_is_dropped() {
    let mut app = app_with(|_| {});
    let (_, ids) = spawn(&mut app, 1, Vec::new());
    let package = AiPackage::Travel(AiTravelPackage { location: [0.0; 3], reset: 0 });
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(7), package: package.clone() });
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(1), package: package.clone() });
    app.world_mut().resource_mut::<sf_ai_bevy::AiDirectives>().remove(ActorKey(1));
    let saved = {
        let mut s = sf_ai::DirectiveStack::new(ActorKey(1), GameTime::default());
        let _ = s.push(
            sf_ai::DirectiveSpec::new(DirectiveKind::GoTo(Location::new(nav_ws(), home())), DirectiveSource::Record),
            GameTime::default(),
            &mw_ai::MorrowindPolicy,
            &mut Vec::new(),
        );
        s
    };
    app.update();
    assert!(app.world().resource::<sf_ai_bevy::AiDirectives>().get(ActorKey(7)).is_none());
    assert!(app.world().resource::<sf_ai_bevy::AiDirectives>().get(ActorKey(1)).is_none(), "no stack was made");
    // Seeded again: a later push lands.
    app.world_mut().resource_mut::<sf_ai_bevy::AiDirectives>().insert(saved);
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(1), package });
    app.update();
    let s = stack(&app, 1);
    assert_eq!(s.top().map(|d| d.spec.source), Some(DirectiveSource::Script));
    assert!(s.get(ids[0]).is_none());
}

fn done(id: u32, end: DirectiveEnd) -> DirectiveDone {
    DirectiveDone { actor: ActorKey(1), id: DirectiveId(id), end, at: GameTime::default() }
}

/// `OpenMW` `AiSequence::execute` (`aisequence.cpp` @ 5f3aafb, 258-372), per served state.
#[test]
fn the_reads_follow_openmw_execute() {
    let a = ActorKey(1);
    let wander = Served::Directive(DirectiveId(3), Some(AiPackageKind::Wander));
    let fight = Served::Directive(DirectiveId(4), None);
    let ran = AiRun { done: false, last: Some(AiPackageKind::Wander) };
    let finished = AiRun { done: true, last: Some(AiPackageKind::Wander) };
    // Running on: not done, the package is the last one.
    assert_eq!(next_run(AiRun::default(), a, wander, &[]), ran);
    assert_eq!(next_run(finished, a, wander, &[]), ran);
    // Finished or expired on its tick: done.
    assert_eq!(next_run(ran, a, wander, &[done(3, DirectiveEnd::Expired)]), finished);
    assert_eq!(next_run(ran, a, wander, &[done(3, DirectiveEnd::Completed)]), finished);
    // Cancelled, or another actor's / directive's end: not done.
    assert_eq!(next_run(ran, a, wander, &[done(3, DirectiveEnd::Cancelled)]), ran);
    assert_eq!(next_run(ran, a, wander, &[done(5, DirectiveEnd::Completed)]), ran);
    let other = DirectiveDone { actor: ActorKey(2), ..done(3, DirectiveEnd::Completed) };
    assert_eq!(next_run(ran, a, wander, &[other]), ran);
    // A non-package (combat) keeps the last package; running on clears done, finishing keeps it.
    assert_eq!(next_run(finished, a, fight, &[]), ran);
    assert_eq!(next_run(finished, a, fight, &[done(4, DirectiveEnd::Completed)]), finished);
    // Empty: no package, done as it was; suspended: unchanged.
    assert_eq!(next_run(finished, a, Served::Empty, &[]), AiRun { done: true, last: None });
    assert_eq!(next_run(ran, a, Served::Suspended, &[]), ran);
}
