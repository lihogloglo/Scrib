//! Exact tests of the query service (nav.md "Build plan" NV6: ordering and budget exact; sync
//! mode equals async results) on the grid double.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    reason = "small synthetic grids in whole meters; requests named a, b, c"
)]

use std::collections::BTreeMap;

use glam::DVec3;
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_world::{Location, WorldspaceId};

use crate::*;

const H: AgentClassId = AgentClassId(0);
const GROUND: AreaId = AreaId(1);
/// Cells per tile side (1 m cells).
const TILE: i32 = 4;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

/// The centre of cell (x, z) at height 0.
fn at(x: i32, z: i32) -> Location {
    Location::new(
        ws(),
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

/// `.` ground, anything else solid; 1 m cells, 4 per tile.
fn grid(rows: &[&str]) -> GridBackend {
    let mut g = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 4,
    })
    .unwrap();
    let t = table();
    g.insert_rows(&ws(), H, 0.5, rows, |c| {
        (c == '.').then(|| GridCell::new(0.0, GROUND, t.flags(GROUND)))
    })
    .unwrap();
    g
}

/// A 40 × 3 open field: tiles x 0..10, z 0.
fn field() -> GridBackend {
    let row = ".".repeat(40);
    grid(&[&row, &row, &row])
}

fn key(x: i32) -> TileKey {
    TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x,
        z: 0,
    }
}

fn query(from: Location, goal: Goal) -> PathQuery {
    PathQuery::new(H, from, goal, QueryFilter::for_class(&table(), H))
}

fn to(x: i32) -> PathQuery {
    query(at(0, 1), Goal::Point(at(x, 1)))
}

fn actor(n: u128) -> ActorKey {
    ActorKey(n)
}

fn budget(nodes_per_slice: u32, nodes_per_tick: u32, jobs_per_tick: u32) -> QueryBudget {
    QueryBudget {
        nodes_per_slice,
        nodes_per_tick,
        jobs_per_tick,
        age_boost_ticks: 0,
        edge_radius_m: 2.0,
    }
}

/// The tick loop the Bevy shell runs: at each tick, drain what is ready, then run. Until idle.
fn run_until_idle(
    s: &mut QueryService,
    g: &dyn NavBackend,
    cov: &dyn NavCoverage,
    mut tick: u64,
) -> Vec<PathReply> {
    let mut out = Vec::new();
    loop {
        out.extend(s.drain_ready(tick));
        if s.queue_len() == 0 && s.ready_len() == 0 {
            return out;
        }
        s.run(g, cov, tick);
        tick += 1;
        assert!(tick < 10_000, "the service never went idle");
    }
}

/// The expansions a request takes in one go.
fn expansions(g: &GridBackend, q: PathQuery) -> u32 {
    let mut s = QueryService::new(budget(1, 1, 1));
    s.request(actor(0), q, PathPriority::Normal, 0);
    s.run_inline(g, &FullCoverage, 0)[0].iterations
}

#[test]
fn replies_come_in_urgency_then_age_then_id_order_and_drain_in_id_order() {
    let g = field();
    // One job per tick; short paths finish in one slice.
    let mut s = QueryService::new(budget(10_000, 10_000, 1));
    let a = s.request(actor(5), to(3), PathPriority::Normal, 0);
    let b = s.request(actor(1), to(3), PathPriority::Combat, 2);
    let c = s.request(actor(3), to(3), PathPriority::Combat, 1);
    let d = s.request(actor(2), to(3), PathPriority::Visible, 0);
    let e = s.request(actor(2), to(3), PathPriority::Normal, 0);
    assert_eq!(s.queue_len(), 5);
    let mut finished = Vec::new();
    for tick in 10..15 {
        assert_eq!(s.run(&g, &FullCoverage, tick).finished, 1);
        // Nothing is ready in the tick that finished it.
        assert!(s.drain_ready(tick).is_empty());
        let r = s.drain_ready(tick + 1);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].finished, tick);
        finished.push(r[0].id);
    }
    // Combat by age (c asked at 1, b at 2), Visible, then Normal by age, then id (actor 2 < 5).
    assert_eq!(finished, [c, b, d, e, a]);

    // All in one tick: replies drain in RequestId order (actor, then submission).
    let mut s = QueryService::new(budget(100, 10_000, 8));
    let a = s.request(actor(5), to(3), PathPriority::Normal, 0);
    let b = s.request(actor(1), to(3), PathPriority::Combat, 2);
    let d = s.request(actor(2), to(3), PathPriority::Visible, 0);
    let e = s.request(actor(2), to(3), PathPriority::Background, 0);
    assert_eq!(s.run(&g, &FullCoverage, 3).finished, 4);
    let ids: Vec<_> = s.drain_ready(4).into_iter().map(|r| r.id).collect();
    assert_eq!(ids, [b, d, e, a]);
    assert!(d.seq < e.seq);
}

#[test]
fn a_long_path_takes_one_slice_per_tick_and_the_tick_caps_hold() {
    let g = field();
    let total = expansions(&g, to(39));
    assert!(total > 30, "{total}");
    for per_slice in [1, 7, 16, total, total + 5] {
        let mut s = QueryService::new(budget(per_slice, 1_000_000, 4));
        let id = s.request(actor(1), to(39), PathPriority::Normal, 0);
        let replies = run_until_idle(&mut s, &g, &FullCoverage, 0);
        assert_eq!(replies.len(), 1);
        let r = &replies[0];
        let slices = total.div_ceil(per_slice);
        assert_eq!((r.id, r.iterations, r.slices), (id, total, slices));
        // One slice per tick from tick 0: it finishes in tick `slices - 1`.
        assert_eq!(r.finished, u64::from(slices - 1));
        assert_eq!(r.result.as_ref().unwrap().status, PathStatus::Complete);
    }

    // The tick allowance is planned up front: 10 per slice, 15 per tick, three jobs -> the
    // first gets 10, the second 5, the third nothing (it keeps its place).
    let mut s = QueryService::new(budget(10, 15, 3));
    for n in 0..3 {
        s.request(actor(n), to(39), PathPriority::Normal, 0);
    }
    for tick in 0..3 {
        let r = s.run(&g, &FullCoverage, tick);
        assert_eq!((r.jobs, r.nodes, r.finished), (2, 15, 0));
    }
    // The job cap: one job per tick even with nodes to spare.
    let mut s = QueryService::new(budget(10, 1_000, 1));
    for n in 0..3 {
        s.request(actor(n), to(39), PathPriority::Normal, 0);
    }
    let r = s.run(&g, &FullCoverage, 0);
    assert_eq!((r.jobs, r.nodes), (1, 10));
    assert_eq!(s.running(), 1);
    assert_eq!(s.queue_len(), 3);
}

#[test]
fn an_old_background_request_rises_to_the_top_and_never_starves() {
    let g = field();
    let mut b = budget(10_000, 10_000, 1);
    b.age_boost_ticks = 4;
    let mut s = QueryService::new(b);
    let old = s.request(actor(9), to(3), PathPriority::Background, 0);
    let mut done = None;
    for tick in 0..40 {
        // Two combat requests a tick, one served: the combat backlog grows forever.
        s.request(actor(1), to(3), PathPriority::Combat, tick);
        s.request(actor(2), to(3), PathPriority::Combat, tick);
        s.run(&g, &FullCoverage, tick);
        if let Some(r) = s.drain_ready(tick + 1).into_iter().find(|r| r.id == old) {
            done = Some(r.finished);
            break;
        }
    }
    // Level 0 + tick / 4 reaches Combat (3) at tick 12, where it is the oldest.
    assert_eq!(done, Some(12));

    // Without ageing it waits behind the backlog.
    let mut s = QueryService::new(budget(10_000, 10_000, 1));
    let old = s.request(actor(9), to(3), PathPriority::Background, 0);
    for tick in 0..40 {
        s.request(actor(1), to(3), PathPriority::Combat, tick);
        s.request(actor(2), to(3), PathPriority::Combat, tick);
        s.run(&g, &FullCoverage, tick);
        assert!(s.drain_ready(tick + 1).iter().all(|r| r.id != old));
    }
}

/// Requests of every kind: long, partial (walled off), near, away, any-of, off-mesh.
fn mixed(s: &mut QueryService) {
    let p = PathPriority::Normal;
    s.request(actor(4), query(at(0, 0), Goal::Point(at(39, 4))), p, 0);
    s.request(
        actor(1),
        query(at(0, 0), Goal::Point(at(30, 0))),
        PathPriority::Combat,
        0,
    );
    s.request(
        actor(2),
        query(
            at(1, 1),
            Goal::Near {
                at: at(20, 2),
                radius: 3.0,
            },
        ),
        p,
        1,
    );
    s.request(
        actor(3),
        query(
            at(5, 0),
            Goal::Away {
                from: at(5, 0),
                min_dist: 9.0,
            },
        ),
        PathPriority::Background,
        0,
    );
    s.request(
        actor(2),
        query(at(0, 4), Goal::AnyOf(vec![at(25, 0), at(12, 4), at(39, 0)])),
        PathPriority::Visible,
        2,
    );
    s.request(actor(5), query(at(0, 0), Goal::Point(at(10, 20))), p, 0);
}

/// 40 × 5: a wall at x = 15 on rows 0..4 (row 4 passes) and a solid column at x = 36 that cuts
/// off x 37..40.
fn walled() -> GridBackend {
    let row = format!("{}#{}#...", ".".repeat(15), ".".repeat(20));
    let last = format!("{}#...", ".".repeat(36));
    grid(&[&row, &row, &row, &row, &last])
}

fn by_id(replies: Vec<PathReply>) -> BTreeMap<RequestId, Result<PathResult, NavError>> {
    replies.into_iter().map(|r| (r.id, r.result)).collect()
}

#[test]
fn inline_sliced_and_reordered_runs_give_identical_results() {
    let g = walled();
    let mut inline = QueryService::new(budget(1, 1, 1));
    mixed(&mut inline);
    let want = by_id(inline.run_inline(&g, &FullCoverage, 0));
    assert_eq!(want.len(), 6);
    let statuses: Vec<_> = want
        .values()
        .map(|r| r.as_ref().map(|p| p.status).map_err(Clone::clone))
        .collect();
    // In id order: actor 1 (Complete), actor 2 near and any-of, actor 3 away, actor 4 the
    // cut-off strip (Partial), actor 5 off the grid (off mesh).
    assert_eq!(
        statuses,
        [
            Ok(PathStatus::Complete),
            Ok(PathStatus::Complete),
            Ok(PathStatus::Complete),
            Ok(PathStatus::Complete),
            Ok(PathStatus::Partial),
            Err(NavError::GoalOffMesh),
        ]
    );

    for (per_slice, per_tick, jobs) in [(1, 3, 2), (7, 10, 2), (16, 1_000, 6), (64, 64, 1)] {
        let mut sliced = QueryService::new(budget(per_slice, per_tick, jobs));
        mixed(&mut sliced);
        assert_eq!(
            by_id(run_until_idle(&mut sliced, &g, &FullCoverage, 0)),
            want
        );

        // The slices of a tick are independent: running them backwards (as a worker pool may)
        // changes nothing.
        let mut rev = QueryService::new(budget(per_slice, per_tick, jobs));
        mixed(&mut rev);
        let mut got = Vec::new();
        for tick in 0.. {
            got.extend(rev.drain_ready(tick));
            if rev.queue_len() == 0 && rev.ready_len() == 0 {
                break;
            }
            let mut batch = rev.plan(tick, &FullCoverage);
            for slice in batch.slices_mut().iter_mut().rev() {
                slice.run(&g);
            }
            rev.commit(batch);
        }
        assert_eq!(by_id(got), want);
    }
}

#[test]
fn cancelled_requests_never_reply() {
    let g = field();
    let mut s = QueryService::new(budget(5, 100, 4));
    let queued = s.request(actor(1), to(39), PathPriority::Background, 0);
    let running = s.request(actor(2), to(39), PathPriority::Combat, 0);
    let finished = s.request(actor(3), to(1), PathPriority::Combat, 0);
    let kept = s.request(actor(4), to(2), PathPriority::Combat, 0);
    let other = s.request(actor(5), to(2), PathPriority::Background, 0);
    // Tick 0, four jobs: combat first (actors 2, 3, 4), then background by id: actor 1 runs,
    // actor 5 waits. The short paths (actors 3, 4) finish in their slice.
    let r = s.run(&g, &FullCoverage, 0);
    assert_eq!(r.jobs, 4);
    assert!(s.is_pending(running) && s.is_pending(queued) && s.is_pending(other));
    assert!(!s.is_pending(finished) && !s.is_pending(kept));
    assert!(s.cancel(queued)); // running, after one slice
    assert!(s.cancel(other)); // never started
    assert!(s.cancel(finished)); // finished, not drained
    assert!(!s.cancel(finished));
    assert_eq!(s.cancel_actor(actor(2)), 1);
    assert_eq!(s.cancel_actor(actor(2)), 0);
    assert_eq!(s.queue_len(), 0);
    let replies = run_until_idle(&mut s, &g, &FullCoverage, 1);
    assert_eq!(replies.iter().map(|r| r.id).collect::<Vec<_>>(), [kept]);
    assert_eq!(s.totals().cancelled, 4);

    // Cancelled while its batch is open: dropped at commit.
    let mut s = QueryService::new(budget(5, 100, 4));
    let id = s.request(actor(1), to(2), PathPriority::Normal, 0);
    let mut batch = s.plan(0, &FullCoverage);
    batch.run_serial(&g);
    assert!(s.is_pending(id));
    assert!(s.cancel(id));
    assert_eq!(s.commit(batch).finished, 0);
    assert!(s.drain_ready(5).is_empty());
    assert_eq!(s.queue_len(), 0);
}

/// Resident when the grid has the tile under `l`.
fn resident_in(g: &GridBackend) -> impl Fn(AgentClassId, &Location) -> bool + '_ {
    move |class, l| {
        let x = (l.pos.to_f64().x.floor() as i32).div_euclid(TILE);
        let z = (l.pos.to_f64().z.floor() as i32).div_euclid(TILE);
        g.tile(&TileKey {
            ws: l.worldspace.clone(),
            class,
            region: 0,
            x,
            z,
        })
        .is_some()
    }
}

#[test]
fn a_goal_beyond_the_resident_tiles_is_partial_at_the_ring_edge() {
    let mut g = field();
    for x in 6..10 {
        assert!(g.remove_tile(&key(x)));
    }
    // Tiles 0..6 stay: covered x < 24.
    let mut s = QueryService::new(budget(8, 100, 4));
    let far = s.request(actor(1), to(38), PathPriority::Normal, 0);
    let near = s.request(actor(2), to(20), PathPriority::Normal, 0);
    let any = s.request(
        actor(3),
        query(at(0, 1), Goal::AnyOf(vec![at(30, 0), at(10, 2)])),
        PathPriority::Normal,
        0,
    );
    let replies = {
        let cov = resident_in(&g);
        run_until_idle(&mut s, &g, &cov, 0)
    };
    let r: BTreeMap<_, _> = replies.into_iter().map(|r| (r.id, r)).collect();

    let far = &r[&far];
    assert!(far.at_ring_edge);
    let path = far.result.as_ref().unwrap();
    assert_eq!(path.status, PathStatus::Partial);
    let end = path.waypoints.last().unwrap().pos.to_f64();
    // Ends within the edge radius (2 m) of the edge point (24, 1.5) on the line, on mesh.
    assert!(
        end.x < 24.0 && (DVec3::new(24.0, 0.0, 1.5) - end).length() <= 2.0,
        "{end}"
    );
    assert!(path.waypoints.iter().all(|w| w.pos.to_f64().x < 24.0));

    assert!(!r[&near].at_ring_edge);
    assert_eq!(
        r[&near].result.as_ref().unwrap().status,
        PathStatus::Complete
    );
    // Any-of keeps its covered candidate: complete.
    assert!(!r[&any].at_ring_edge);
    let p = r[&any].result.as_ref().unwrap();
    assert_eq!(p.status, PathStatus::Complete);
    assert_eq!(p.waypoints.last(), Some(&at(10, 2)));

    // Without a coverage hook the same goal is simply off the mesh.
    let mut s = QueryService::new(budget(8, 100, 4));
    s.request(actor(1), to(38), PathPriority::Normal, 0);
    let r = s.run_inline(&g, &FullCoverage, 0);
    assert_eq!(r[0].result, Err(NavError::GoalOffMesh));
    assert!(!r[0].at_ring_edge);
}

#[test]
fn removing_a_tile_mid_job_fails_that_job_cleanly() {
    let mut g = field();
    let mut s = QueryService::new(budget(5, 100, 4));
    let id = s.request(actor(1), to(39), PathPriority::Normal, 0);
    let other = s.request(actor(2), to(39), PathPriority::Normal, 0);
    s.run(&g, &FullCoverage, 0);
    assert!(s.is_pending(id));
    assert!(g.remove_tile(&key(0)));
    assert!(s.cancel(other));
    s.run(&g, &FullCoverage, 1);
    let r = s.drain_ready(2);
    assert_eq!(r.len(), 1);
    assert_eq!((r[0].id, &r[0].result), (id, &Err(NavError::MeshChanged)));
    assert_eq!(s.queue_len(), 0);
}

#[test]
fn the_ring_covers_resident_and_empty_tiles_only() {
    let layout = RingLayout {
        classes: vec![ClassTiling {
            class: H,
            tile_size: 4.0,
        }],
        regions: BTreeMap::new(),
        default_regions: vec![RegionFrame {
            region: 0,
            origin: WorldPos::ORIGIN,
        }],
    };
    let mut ring = NavRing::new(layout);
    ring.update(
        Some(&at(1, 1)),
        RingRadii {
            load_m: 4.0,
            unload_m: 4.0,
        },
    );
    // Tiles within 4 m of (1.5, 1.5): x, z in -1..=1 (9 tiles).
    let keys = ring.next(64);
    assert_eq!(keys.len(), 9);
    for k in keys {
        let state = match (k.x, k.z) {
            (0, 0) => TileState::Resident,
            (1, 0) => TileState::Empty,
            (0, 1) => TileState::Unbaked,
            (1, 1) => TileState::Failed,
            _ => continue, // pending
        };
        ring.settle(k, state);
    }
    let covered = |x: i32, z: i32| ring.covers(H, &at(x, z));
    assert!(covered(1, 1) && covered(5, 1));
    assert!(!covered(1, 5) && !covered(5, 5) && !covered(-2, 1) && !covered(9, 1));
    assert!(!ring.covers(AgentClassId(1), &at(1, 1)));
}
