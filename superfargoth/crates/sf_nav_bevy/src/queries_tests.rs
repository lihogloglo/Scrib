//! nav.md "Build plan" NV6, Bevy side: the query service on a manually stepped fixed schedule
//! (virtual time held still, `FixedMain` run by hand). Exact: the tick a reply is published at,
//! the `nav.stats` queue fields, and parallel slices equal to serial ones.

use std::collections::BTreeMap;
use std::time::Duration;

use bevy::MinimalPlugins;
use bevy::time::TimeUpdateStrategy;
use bevy_app::FixedMain;
use glam::DVec3;
use sf_console_bevy::ConsolePlugin;
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_nav::{
    AgentClassId, AreaDef, AreaId, AreaTable, FullCoverage, Goal, GridBackend, GridCell,
    GridConfig, MAX_CLASSES, NavError, PathPriority, PathQuery, PathResult, PathStatus, PolyFlags,
    QueryFilter, QueryService, RequestId,
};
use sf_world::{Location, WorldspaceId};

use super::*;

const H: AgentClassId = AgentClassId(0);
const GROUND: AreaId = AreaId(1);

fn at(x: i32, z: i32) -> Location {
    Location::new(
        WorldspaceId::new("test"),
        WorldPos::from_f64(DVec3::new(f64::from(x) + 0.5, 0.0, f64::from(z) + 0.5)),
    )
}

fn table() -> AreaTable {
    AreaTable::new(vec![AreaDef {
        id: GROUND,
        name: "ground".into(),
        cost: [1.0; MAX_CLASSES],
        flags: PolyFlags::WALK,
    }])
    .unwrap()
}

/// A 40 × 5 field, 1 m cells, with a wall at x = 15 on rows 0..4.
fn field() -> GridBackend {
    let mut g = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 4,
    })
    .unwrap();
    let row = format!("{}#{}", ".".repeat(15), ".".repeat(24));
    let open = ".".repeat(40);
    let t = table();
    g.insert_rows(
        &WorldspaceId::new("test"),
        H,
        0.5,
        &[&row, &row, &row, &row, &open],
        |c| (c == '.').then(|| GridCell::new(0.0, GROUND, t.flags(GROUND))),
    )
    .unwrap();
    g
}

fn query(from: Location, goal: Goal) -> PathQuery {
    PathQuery::new(H, from, goal, QueryFilter::for_class(&table(), H))
}

/// Settings as given, applied after the first update binds the tunables' defaults.
fn app(settings: NavQuerySettings) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin, NavPlugin, NavQueryPlugin))
        // Virtual time stands still: the fixed loop never runs on its own.
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO))
        .insert_resource(NavMeshes(Box::new(field())));
    app.update();
    *app.world_mut().resource_mut::<NavQuerySettings>() = settings;
    app
}

fn settings(nodes_per_slice: u32, jobs_per_tick: u32, parallel: bool) -> NavQuerySettings {
    let mut s = NavQuerySettings::default();
    s.budget.nodes_per_slice = nodes_per_slice;
    s.budget.nodes_per_tick = nodes_per_slice * jobs_per_tick;
    s.budget.jobs_per_tick = jobs_per_tick;
    s.parallel = parallel;
    s
}

fn fixed_tick(app: &mut App) {
    app.world_mut().run_schedule(FixedMain);
}

#[test]
fn a_reply_is_published_at_the_next_fixed_tick_boundary() {
    let long = query(at(0, 0), Goal::Point(at(30, 0)));
    let total = {
        let mut s = QueryService::new(settings(1, 1, false).budget);
        s.request(ActorKey(1), long.clone(), PathPriority::Normal, 0);
        s.run_inline(&field(), &FullCoverage, 0)[0].iterations
    };
    let per_slice = 8;
    let slices = total.div_ceil(per_slice);
    assert!(slices > 3, "{total}");

    let mut app = app(settings(per_slice, 4, false));
    let id = app
        .world_mut()
        .resource_mut::<NavQueries>()
        .request(ActorKey(1), long, PathPriority::Combat);
    assert_eq!(app.world().resource::<NavQueries>().tick(), 0);
    // Ticks 1..=slices run it; tick slices + 1 publishes it at its start.
    for tick in 1..=u64::from(slices) {
        fixed_tick(&mut app);
        let q = app.world().resource::<NavQueries>();
        assert_eq!(q.tick(), tick);
        assert!(q.get(id).is_none(), "published early at tick {tick}");
        assert_eq!(q.is_pending(id), tick < u64::from(slices));
    }
    let c = app.world().resource::<NavCounters>().clone();
    assert_eq!(
        (c.queue_depth, c.running, c.jobs, c.results),
        (Some(0), Some(0), Some(1), Some(1))
    );
    assert_eq!(c.nodes, Some(total - per_slice * (slices - 1)));
    assert!(c.tick_ms.is_some_and(|t| t >= 0.0));

    fixed_tick(&mut app);
    let mut q = app.world_mut().resource_mut::<NavQueries>();
    let reply = q.take(id).expect("published at the boundary");
    assert_eq!((reply.requested, reply.finished), (0, u64::from(slices)));
    assert_eq!((reply.slices, reply.iterations), (slices, total));
    assert_eq!(reply.result.unwrap().status, PathStatus::Complete);
    assert!(q.take(id).is_none());

    // nav.stats: the queue fields, with the idle tick's run.
    let line = NavStats::of(app.world()).line();
    assert!(line.contains(", queue 0 (0 running), tick "), "{line}");
    assert!(line.ends_with(" ms (0 jobs, 0 nodes, 0 results), rebuilds n/a"), "{line}");
}

/// Requests of several kinds and priorities, stepped until every reply is published.
fn replies(settings: NavQuerySettings) -> BTreeMap<RequestId, Result<PathResult, NavError>> {
    let mut app = app(settings);
    let mut ids = Vec::new();
    {
        let mut q = app.world_mut().resource_mut::<NavQueries>();
        let mut ask = |actor, goal, from, p| ids.push(q.request(ActorKey(actor), query(from, goal), p));
        ask(3, Goal::Point(at(39, 3)), at(0, 0), PathPriority::Normal);
        ask(1, Goal::Point(at(20, 0)), at(2, 2), PathPriority::Combat);
        ask(2, Goal::Near { at: at(30, 4), radius: 2.0 }, at(0, 4), PathPriority::Visible);
        ask(2, Goal::Away { from: at(5, 1), min_dist: 12.0 }, at(5, 1), PathPriority::Background);
        ask(4, Goal::AnyOf(vec![at(35, 0), at(10, 3)]), at(1, 1), PathPriority::Normal);
        ask(5, Goal::Point(at(5, 30)), at(1, 1), PathPriority::Normal);
    }
    let mut out = BTreeMap::new();
    for _ in 0..500 {
        fixed_tick(&mut app);
        let mut q = app.world_mut().resource_mut::<NavQueries>();
        for id in &ids {
            if let Some(r) = q.take(*id) {
                out.insert(*id, r.result);
            }
        }
        if out.len() == ids.len() {
            return out;
        }
    }
    panic!("only {} of {} replies", out.len(), ids.len());
}

#[test]
fn parallel_slices_equal_serial_and_inline_results() {
    let serial = replies(settings(6, 4, false));
    assert_eq!(serial.len(), 6);
    assert_eq!(
        serial.values().filter(|r| r.is_ok()).count(),
        5,
        "{serial:?}"
    );
    assert_eq!(replies(settings(6, 4, true)), serial);
    assert_eq!(replies(settings(1, 6, true)), serial);
    // One slice per job, as the inline mode.
    assert_eq!(replies(settings(1_000_000, 8, true)), serial);
}
