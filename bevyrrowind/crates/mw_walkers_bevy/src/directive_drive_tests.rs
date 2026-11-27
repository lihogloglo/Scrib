//! `MA2d2`: the directive stack's serving directive drives the routine
//! ([`crate::directive_drive`]), on the wander drive's harness ([`super::drive_tests`]: the real
//! systems on the fixed tick over a flat 40 m navmesh room). The stacks are seeded as
//! `seed_directives` does (record order through `MorrowindPolicy`, the wander's idle table as the
//! `StayNear`'s extra). `MA2d4`: the wander's home at its first serve, repeats keeping extras and
//! home, one duration count, a walker for an actor spawned without one, standing on an empty
//! stack.

use bevy_app::App;
use bevy_ecs::entity::Entity;
use bevy_math::Vec3;
use mw_ai::MorrowindPolicy;
use mw_ai::packages::PackageExtra;
use mw_world::convert::UNIT_METERS_F64;
use sf_ai::{
    DirectiveId, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, DirectiveStatus, GameTime, Limit,
};
use sf_ai_bevy::{AiActor, AiDirectives};
use sf_controller_bevy::{MoveIntent, Mover, MoverParams};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_nav::QueryFilter;
use sf_nav_bevy::NavCorridor;
use sf_sky_bevy::GameClock;
use sf_world::Location;
use tes3::esp::{AiPackage, AiTravelPackage, AiWanderPackage};

use super::drive_tests::{SPEED, app_with, home, nav_ws, spawn_wanderer};
use super::{ActorSpace, Walker};
use crate::directive_drive::{DirectiveDrive, TRAVEL_END_TOLERANCE_M};
use crate::directives::AiPackageExtras;
use crate::reaction::Reaction;
use crate::script_ai::{AiRuns, ScriptAiPush};

/// Fixed ticks per second (the harness's 60 Hz).
pub(super) const HZ: usize = 60;
/// The `AI_T` point: 8 m east of home.
const TRAVEL: Vec3 = Vec3::new(8.0, 0.0, 0.0);

fn planar(a: WorldPos, b: WorldPos) -> f32 {
    a.delta_f32(b).with_y(0.0).length()
}

/// A wanderer as `attach` makes it (its `AI_W` radius and idle table), an [`AiActor`] whose stack
/// holds `before` (e.g. an `AI_T` listed first) and then the record wander, as `seed_directives`
/// seeds it. Returns the entity and the directive ids in push order (the wander last).
pub(super) fn spawn(app: &mut App, key: u128, before: Vec<DirectiveKind>) -> (Entity, Vec<DirectiveId>) {
    spawn_with(app, key, before, DirectiveSpec::repeating)
}

/// [`spawn`] with the wander's spec shaped by `stay` (a limit, no repeat), its stack started at
/// the [`GameClock`]'s instant if the app has one.
pub(super) fn spawn_with(
    app: &mut App,
    key: u128,
    before: Vec<DirectiveKind>,
    stay: impl FnOnce(DirectiveSpec) -> DirectiveSpec,
) -> (Entity, Vec<DirectiveId>) {
    let e = spawn_wanderer(app, &nav_ws(), home(), key, 11);
    let wander = app.world().get::<Walker>(e).unwrap().driver.planner.wander;
    let now = app.world().get_resource::<GameClock>().map_or_else(GameTime::default, |c| GameTime::from(c.now));
    let mut stack = DirectiveStack::new(ActorKey(key), now);
    let mut events = Vec::new();
    let mut ids: Vec<_> = before
        .into_iter()
        .map(|k| stack.push(DirectiveSpec::new(k, DirectiveSource::Record), now, &MorrowindPolicy, &mut events))
        .collect();
    let kind = DirectiveKind::StayNear { home: Location::new(nav_ws(), home()), radius: wander.distance };
    let stay_id = stack.push(stay(DirectiveSpec::new(kind, DirectiveSource::Record)), now, &MorrowindPolicy, &mut events);
    ids.push(stay_id);
    assert!(events.is_empty());
    app.world_mut().resource_mut::<AiDirectives>().insert(stack);
    app.world_mut().entity_mut(e).insert((
        AiActor(ActorKey(key)),
        AiPackageExtras(vec![(stay_id, PackageExtra::Wander(wander))]),
        DirectiveDrive::default(),
    ));
    (e, ids)
}

pub(super) fn stack(app: &App, key: u128) -> &DirectiveStack {
    app.world().resource::<AiDirectives>().get(ActorKey(key)).expect("a stack")
}

/// `AI_T` above `AI_W`: the wander pauses while the actor walks to the point; the `GoTo` finishes
/// there (it leaves the stack) and the wander takes over around the arrival point, its home at
/// its first serve (`MA2d4`, `OpenMW` `mInitialActorPosition`), not the spawn point.
#[test]
fn a_go_to_walks_to_its_point_finishes_then_the_wander_resumes_around_it() {
    let mut app = app_with(|_| {});
    let point = home().offset(TRAVEL);
    let (e, ids) = spawn(&mut app, 1, vec![DirectiveKind::GoTo(Location::new(nav_ws(), point))]);
    let (go_to, stay) = (ids[0], ids[1]);
    let radius = app.world().get::<Walker>(e).unwrap().driver.planner.wander.distance;
    let mut finished = None;
    for tick in 0..(20 * HZ) {
        app.update();
        let world = app.world();
        let pos = world.get::<Mover>(e).unwrap().loco.position;
        assert!(home().delta_f32(pos).is_finite(), "tick {tick}: NaN position");
        assert!(home().delta_f32(pos).y.abs() < 0.5, "tick {tick}: left the floor");
        if stack(&app, 1).status(go_to) == DirectiveStatus::Ended {
            finished = Some((tick, pos));
            break;
        }
        assert_eq!(stack(&app, 1).status(go_to), DirectiveStatus::Serving);
        assert!(world.get::<DirectiveDrive>(e).unwrap().owns(), "tick {tick}: the go_to does not own the actor");
        let walker = world.get::<Walker>(e).unwrap();
        assert_eq!(walker.driver.arrival_count(), 0, "tick {tick}: the wander ran during the go_to");
    }
    let (tick, at) = finished.expect("the go_to never finished");
    assert!(planar(at, point) <= 2.0, "finished {} m from the point", planar(at, point));
    assert!(tick > 3 * HZ, "8 m at 1.5 m/s took {tick} ticks");
    let s = stack(&app, 1);
    assert_eq!(s.len(), 1, "the finished go_to left the stack");
    assert_eq!(s.status(stay), DirectiveStatus::Serving, "the wander is the next top");
    let drive = app.world().get::<DirectiveDrive>(e).unwrap();
    assert!(!drive.owns());
    assert_eq!((drive.stats.goto_started, drive.stats.goto_finished, drive.stats.goto_failed_walks), (1, 1, 0));

    // The wander's home is where it first served: the arrival point.
    app.update();
    let drive = app.world().get::<DirectiveDrive>(e).unwrap();
    let wander_home = drive.home(stay).expect("the wander served");
    assert!(planar(wander_home, at) <= 0.1, "home {} m from the arrival", planar(wander_home, at));
    assert_eq!(drive.stats.wander_retargets, 1, "the wander moved its home to the arrival point");
    assert_eq!(app.world().get::<Walker>(e).unwrap().driver.planner.home, wander_home);
    let mut far_late = 0.0_f32;
    for t in 0..(60 * HZ) {
        app.update();
        let pos = app.world().get::<Mover>(e).unwrap().loco.position;
        if t >= 20 * HZ {
            far_late = far_late.max(planar(pos, wander_home));
        }
    }
    let walker = app.world().get::<Walker>(e).unwrap();
    assert!(walker.driver.arrival_count() >= 2, "{} wander arrivals after the go_to", walker.driver.arrival_count());
    assert!(far_late <= radius + 1.0, "wandered {far_late} m from the arrival (radius {radius})");
}

/// A record wander alone: the `StayNear` agrees with the walker, so the drive changes nothing
/// (no re-target, never owns) and the wander walks its disc as before.
#[test]
fn a_record_wander_alone_runs_unchanged() {
    let mut app = app_with(|_| {});
    let (e, ids) = spawn(&mut app, 1, Vec::new());
    let planner0 = app.world().get::<Walker>(e).unwrap().driver.planner.clone();
    let mut farthest = 0.0_f32;
    for tick in 0..(60 * HZ) {
        app.update();
        let world = app.world();
        assert!(!world.get::<DirectiveDrive>(e).unwrap().owns(), "tick {tick}: the wander was paused");
        farthest = farthest.max(planar(world.get::<Mover>(e).unwrap().loco.position, home()));
    }
    assert_eq!(stack(&app, 1).status(ids[0]), DirectiveStatus::Serving);
    let world = app.world();
    let walker = world.get::<Walker>(e).unwrap();
    assert_eq!(walker.driver.planner.home, planner0.home);
    assert_eq!(walker.driver.planner.wander, planner0.wander);
    assert_eq!(world.get::<DirectiveDrive>(e).unwrap().stats, crate::directive_drive::DirectiveDriveStats::default());
    assert!(walker.driver.arrival_count() >= 3, "{} arrivals", walker.driver.arrival_count());
    assert!(farthest > 1.0 && farthest <= planner0.wander.distance + 1.0, "farthest {farthest} m");
}

/// The engine position `at` in Morrowind units (engine `(x, y, z)` is units `(x, -z, y)`).
#[allow(clippy::cast_possible_truncation)]
fn units(at: WorldPos) -> [f32; 3] {
    let p = at.to_f64();
    [(p.x / UNIT_METERS_F64) as f32, (-p.z / UNIT_METERS_F64) as f32, (p.y / UNIT_METERS_F64) as f32]
}

fn position(app: &App, e: Entity) -> WorldPos {
    app.world().get::<Mover>(e).unwrap().loco.position
}

/// Runs until `key`'s stack is empty (a pushed travel arrived); returns where the actor stands.
fn walk_until_empty(app: &mut App, e: Entity, key: u128) -> WorldPos {
    for _ in 0..(20 * HZ) {
        app.update();
        if stack(app, key).is_empty() {
            return position(app, e);
        }
    }
    panic!("the travel never finished");
}

/// `MA2d4` (`OpenMW`: an empty `AiSequence` runs nothing): a script travel arrives, the stack is
/// empty, and the actor stays where it arrived instead of resuming the record wander at spawn.
#[test]
fn an_empty_stack_after_a_travel_stands_where_it_arrived() {
    let mut app = app_with(|_| {});
    let (e, _) = spawn(&mut app, 1, Vec::new());
    let point = home().offset(Vec3::new(8.0, 0.0, 0.0));
    let package = AiPackage::Travel(AiTravelPackage { location: units(point), reset: 0 });
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(1), package });
    let arrived = walk_until_empty(&mut app, e, 1);
    assert!(planar(arrived, point) <= 2.0, "arrived {} m from the point", planar(arrived, point));
    let arrivals = app.world().get::<Walker>(e).unwrap().driver.arrival_count();
    let mut farthest = 0.0_f32;
    for tick in 0..(10 * HZ) {
        app.update();
        assert!(app.world().get::<DirectiveDrive>(e).unwrap().stands(), "tick {tick}: the empty stack does not stand");
        farthest = farthest.max(planar(position(&app, e), arrived));
    }
    let tolerance = TRAVEL_END_TOLERANCE_M.default_value();
    assert!(farthest <= tolerance, "moved {farthest} m off the arrival (tolerance {tolerance} m)");
    assert_eq!(app.world().get::<Walker>(e).unwrap().driver.arrival_count(), arrivals, "the wander ran");
}

/// `MA2d4`: a script `AIWander` pushed after a walk wanders around where it first serves (the
/// arrival point), not around the spawn point.
#[test]
fn a_wander_pushed_after_a_walk_stays_near_the_arrival_point() {
    let mut app = app_with(|_| {});
    let (e, _) = spawn(&mut app, 1, Vec::new());
    let point = home().offset(Vec3::new(8.0, 0.0, 0.0));
    let package = AiPackage::Travel(AiTravelPackage { location: units(point), reset: 0 });
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(1), package });
    walk_until_empty(&mut app, e, 1);
    // 3 m around the arrival (the spawn point is 8 m away, outside it).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let distance = (3.0 / UNIT_METERS_F64) as u16;
    let package = AiPackage::Wander(AiWanderPackage { distance, idle2: 40, ..AiWanderPackage::default() });
    app.world_mut().write_message(ScriptAiPush { actor: ActorKey(1), package });
    let before = position(&app, e);
    app.update();
    let id = stack(&app, 1).top().expect("the pushed wander").id;
    let drive = app.world().get::<DirectiveDrive>(e).unwrap();
    let wander_home = drive.home(id).expect("the wander served");
    assert!(planar(wander_home, before) <= 0.05, "home {} m from where it stood", planar(wander_home, before));
    assert!(!drive.stands());
    let planner = &app.world().get::<Walker>(e).unwrap().driver.planner;
    assert_eq!(planner.home, wander_home);
    assert_eq!(planner.wander.idle_chances[0], 40, "the pushed package's idle table");
    let radius = planner.wander.distance;
    let mut farthest = 0.0_f32;
    for _ in 0..(40 * HZ) {
        app.update();
        farthest = farthest.max(planar(position(&app, e), wander_home));
    }
    assert!(farthest > 0.5, "the wander never walked ({farthest} m)");
    assert!(farthest <= radius + 1.0, "wandered {farthest} m from the arrival (radius {radius})");
    assert!(app.world().get::<Walker>(e).unwrap().driver.arrival_count() >= 2);
}

/// `MA2d4`: a repeating wander whose `Limit` runs out comes back under a new id with its idle
/// table and its home (`OpenMW` clones the package, `mInitialActorPosition` included).
#[test]
fn a_repeat_keeps_its_extras_and_home_under_the_new_id() {
    let mut app = app_with(|app| {
        app.insert_resource(GameClock::default());
    });
    let (e, ids) = spawn_with(&mut app, 1, Vec::new(), |s| s.repeating().limit(Limit::For { minutes: 60 }));
    let old = ids[0];
    app.update();
    let home0 = app.world().get::<DirectiveDrive>(e).unwrap().home(old).expect("served");
    let idle_table = app.world().get::<AiPackageExtras>(e).unwrap().get(old).cloned().expect("the idle table");
    app.world_mut().resource_mut::<GameClock>().skip(61 * 60_000);
    app.update();
    let s = stack(&app, 1);
    assert_eq!(s.status(old), DirectiveStatus::Ended);
    assert_eq!(s.len(), 1, "the repeat came back");
    let new = s.top().unwrap().id;
    assert!(new > old);
    let world = app.world();
    let extras = world.get::<AiPackageExtras>(e).unwrap();
    assert_eq!(extras.get(new), Some(&idle_table), "the idle table follows the new id");
    assert_eq!(extras.get(old), None);
    let drive = world.get::<DirectiveDrive>(e).unwrap();
    assert_eq!(drive.home(new), Some(home0), "the home follows the new id");
    assert_eq!(drive.home(old), None);
    assert_eq!(drive.stats.wander_retargets, 0, "the same wander, the same home");
}

/// `MA2d4`: an `AI_W` with a duration ends once, when the stack's `Limit` runs out (the wander
/// driver counts none): served up to it, then the empty stack stands.
#[test]
fn a_wander_duration_ends_once_at_its_limit() {
    let mut app = app_with(|app| {
        app.insert_resource(GameClock::default());
    });
    let (e, ids) = spawn_with(&mut app, 1, Vec::new(), |s| s.limit(Limit::For { minutes: 120 }));
    let stay = ids[0];
    for _ in 0..(5 * HZ) {
        app.update();
    }
    app.world_mut().resource_mut::<GameClock>().skip(119 * 60_000);
    app.update();
    assert_eq!(stack(&app, 1).status(stay), DirectiveStatus::Serving, "a minute is left");
    assert!(!app.world().get::<DirectiveDrive>(e).unwrap().stands());
    app.world_mut().resource_mut::<GameClock>().skip(2 * 60_000);
    app.update();
    assert_eq!(stack(&app, 1).status(stay), DirectiveStatus::Ended);
    assert!(stack(&app, 1).is_empty(), "no repeat");
    assert!(app.world().resource::<AiRuns>().get(ActorKey(1)).done, "GetAIPackageDone after the limit");
    let at = position(&app, e);
    let arrivals = app.world().get::<Walker>(e).unwrap().driver.arrival_count();
    let mut farthest = 0.0_f32;
    for _ in 0..(10 * HZ) {
        app.update();
        farthest = farthest.max(planar(position(&app, e), at));
    }
    assert!(app.world().get::<DirectiveDrive>(e).unwrap().stands());
    assert!(farthest <= TRAVEL_END_TOLERANCE_M.default_value(), "moved {farthest} m after the limit");
    assert_eq!(app.world().get::<Walker>(e).unwrap().driver.arrival_count(), arrivals, "the wander ran on");
}

/// `MA2d4`: a `StayNear` served by an actor spawned without a wander (a guard on post) gives it
/// the walker `attach` builds, around where it stands, and it wanders.
#[test]
fn a_stay_near_gives_an_actor_without_a_walker_one() {
    let mut app = app_with(|_| {});
    let filter = QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID);
    let e = app
        .world_mut()
        .spawn((
            Mover::new(home(), 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            ActorSpace(nav_ws()),
            NavCorridor::new(ActorKey(1), mw_bake::nav::HUMANOID, nav_ws(), SPEED).with_filter(filter),
            Reaction::new(SPEED),
            AiActor(ActorKey(1)),
            DirectiveDrive::default(),
        ))
        .id();
    let radius = 4.0;
    let mut s = DirectiveStack::new(ActorKey(1), GameTime::default());
    let kind = DirectiveKind::StayNear { home: Location::new(nav_ws(), home()), radius };
    let _ = s.push(DirectiveSpec::new(kind, DirectiveSource::Script), GameTime::default(), &MorrowindPolicy, &mut Vec::new());
    app.world_mut().resource_mut::<AiDirectives>().insert(s);
    app.update();
    app.update();
    let world = app.world();
    let walker = world.get::<Walker>(e).expect("a walker was given");
    assert_eq!(walker.driver.planner.home, home());
    assert!((walker.driver.planner.wander.distance - radius).abs() < 1e-6);
    assert!((walker.driver.walk_speed - SPEED).abs() < 1e-6, "the reaction's walk speed");
    assert_eq!(world.get::<DirectiveDrive>(e).unwrap().stats.walkers_given, 1);
    let mut farthest = 0.0_f32;
    for _ in 0..(40 * HZ) {
        app.update();
        farthest = farthest.max(planar(position(&app, e), home()));
    }
    let world = app.world();
    assert_eq!(world.get::<DirectiveDrive>(e).unwrap().stats.walkers_given, 1, "given once");
    assert!(world.get::<Walker>(e).unwrap().driver.arrival_count() >= 2, "it never wandered");
    assert!(farthest > 0.5 && farthest <= radius + 1.0, "farthest {farthest} m (radius {radius})");
}
