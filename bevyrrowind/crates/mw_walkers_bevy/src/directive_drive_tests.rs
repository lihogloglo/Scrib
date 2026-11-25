//! `MA2d2`: the directive stack's serving directive drives the routine
//! ([`crate::directive_drive`]), on the wander drive's harness ([`super::drive_tests`]: the real
//! systems on the fixed tick over a flat 40 m navmesh room). The stacks are seeded as
//! `seed_directives` does (record order through `MorrowindPolicy`, the wander's idle table as the
//! `StayNear`'s extra).

use bevy_app::App;
use bevy_ecs::entity::Entity;
use bevy_math::Vec3;
use mw_ai::MorrowindPolicy;
use mw_ai::packages::PackageExtra;
use sf_ai::{DirectiveId, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, DirectiveStatus, GameTime};
use sf_ai_bevy::{AiActor, AiDirectives};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_world::Location;

use super::Walker;
use super::drive_tests::{app_with, home, nav_ws, spawn_wanderer};
use crate::directive_drive::DirectiveDrive;
use crate::directives::AiPackageExtras;

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
    let e = spawn_wanderer(app, &nav_ws(), home(), key, 11);
    let wander = app.world().get::<Walker>(e).unwrap().driver.planner.wander;
    let mut stack = DirectiveStack::new(ActorKey(key), GameTime::default());
    let mut events = Vec::new();
    let mut ids: Vec<_> = before
        .into_iter()
        .map(|k| stack.push(DirectiveSpec::new(k, DirectiveSource::Record), GameTime::default(), &MorrowindPolicy, &mut events))
        .collect();
    let stay = DirectiveKind::StayNear { home: Location::new(nav_ws(), home()), radius: wander.distance };
    let stay_id = stack.push(
        DirectiveSpec::new(stay, DirectiveSource::Record).repeating(),
        GameTime::default(),
        &MorrowindPolicy,
        &mut events,
    );
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
/// there (it leaves the stack) and the wander takes over around its home.
#[test]
fn a_go_to_walks_to_its_point_finishes_then_the_wander_resumes() {
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
    assert_eq!(drive.stats.wander_retargets, 0, "the record wander already had its home and radius");

    // The wander resumes around home (the point is outside its disc: it walks back in).
    let mut far_late = 0.0_f32;
    for t in 0..(60 * HZ) {
        app.update();
        let pos = app.world().get::<Mover>(e).unwrap().loco.position;
        if t >= 40 * HZ {
            far_late = far_late.max(planar(pos, home()));
        }
    }
    let walker = app.world().get::<Walker>(e).unwrap();
    assert!(walker.driver.arrival_count() >= 2, "{} wander arrivals after the go_to", walker.driver.arrival_count());
    assert!(far_late <= radius + 1.0, "wandered {far_late} m from home (radius {radius})");
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
