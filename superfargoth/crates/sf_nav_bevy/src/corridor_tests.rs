//! Headless Bevy integration tests for [`NavCorridor`] + [`NavCorridorPlugin`] (`docs/specs/nav.md`
//! Build plan NV8: walk around a pillar and through a narrow doorway, and repair after a push).

use std::sync::Arc;
use std::time::Duration;

use bevy::MinimalPlugins;
use bevy::time::{Fixed, Time, TimeUpdateStrategy};
use glam::{Affine3A, DVec3, Vec2, Vec3};
use sf_console_bevy::ConsolePlugin;
use sf_controller::{
    CollisionBuilder, CollisionMesh, GroundHit, GroundProbe, HeightSurface, MoverParams, Nearest,
    ProbeQuery, StaticShape, StaticWorld, SurfaceId, TerrainProbe,
};
use sf_controller_bevy::{MoveIntent, Mover, SfControllerPlugin};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, BuildConfig, MAX_CLASSES, NavBackend,
    NavError, PolyFlags, QueryFilter, RequestId, TileInput, TileKey,
};
use sf_nav_recast::{RecastBackend, RecastConfig};
use sf_world::{Location, WorldspaceId};

use super::*;

const H: AgentClassId = AgentClassId(0);
const GROUND: AreaId = AreaId(1);

fn ws() -> WorldspaceId {
    WorldspaceId::new("test_room")
}

fn wpos(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::from_f64(DVec3::new(f64::from(x), f64::from(y), f64::from(z)))
}

fn loc(x: f32, y: f32, z: f32) -> Location {
    Location::new(ws(), wpos(x, y, z))
}

fn xyz(p: WorldPos) -> Vec3 {
    WorldPos::ORIGIN.delta_f32(p)
}

struct FlatFloor;

impl HeightSurface for FlatFloor {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        if dir.y.abs() < 1e-6 {
            return None;
        }
        let t = -xyz(origin).y / dir.y;
        (0.0..=max).contains(&t).then_some((t, Vec3::Y))
    }
}

#[derive(Resource)]
struct RoomGround {
    terrain: TerrainProbe<FlatFloor>,
    statics: StaticWorld,
}

impl GroundProbe for RoomGround {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

fn box_collision(half: Vec3) -> Arc<StaticShape> {
    let c = |x: f32, y: f32, z: f32| [x * half.x, y * half.y, z * half.z];
    let corners = [
        c(-1.0, -1.0, -1.0),
        c(1.0, -1.0, -1.0),
        c(1.0, 1.0, -1.0),
        c(-1.0, 1.0, -1.0),
        c(-1.0, -1.0, 1.0),
        c(1.0, -1.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(-1.0, 1.0, 1.0),
    ];
    let idx = [
        0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3,
        1, 2, 6, 1, 6, 5,
    ];
    let mut b = CollisionBuilder::new(1e-3);
    b.add(&corners, &idx, Affine3A::IDENTITY, SurfaceId(1));
    let mesh: CollisionMesh = b.finish();
    Arc::new(StaticShape::new(&mesh).unwrap())
}

#[derive(Default)]
struct RoomSoup {
    verts: Vec<Vec3>,
    tris: Vec<[u32; 3]>,
    areas: Vec<AreaId>,
}

impl RoomSoup {
    fn quad(&mut self, q: [Vec3; 4], area: AreaId) {
        let n = u32::try_from(self.verts.len()).unwrap();
        self.verts.extend(q);
        self.tris.extend([[n, n + 1, n + 2], [n, n + 2, n + 3]]);
        self.areas.extend([area, area]);
    }

    fn floor(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, y: f32) {
        self.quad(
            [
                Vec3::new(x0, y, z0),
                Vec3::new(x0, y, z1),
                Vec3::new(x1, y, z1),
                Vec3::new(x1, y, z0),
            ],
            GROUND,
        );
    }

    fn block(&mut self, lo: Vec3, hi: Vec3) {
        self.floor(lo.x, lo.z, hi.x, hi.z, hi.y);
        for [a, b] in [
            [Vec2::new(lo.x, lo.z), Vec2::new(hi.x, lo.z)],
            [Vec2::new(hi.x, lo.z), Vec2::new(hi.x, hi.z)],
            [Vec2::new(hi.x, hi.z), Vec2::new(lo.x, hi.z)],
            [Vec2::new(lo.x, hi.z), Vec2::new(lo.x, lo.z)],
        ] {
            self.quad(
                [
                    Vec3::new(a.x, lo.y, a.y),
                    Vec3::new(b.x, lo.y, b.y),
                    Vec3::new(b.x, hi.y, b.y),
                    Vec3::new(a.x, hi.y, a.y),
                ],
                AreaId::NULL,
            );
        }
    }
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

fn filter() -> QueryFilter {
    QueryFilter::for_class(&table(), H)
}

/// Builds a 16 m × 16 m room with:
/// - a central pillar obstacle at `x in [4.0..6.0], z in [7.0..9.0]`
/// - a dividing wall at `x in [10.0..10.6]` with a narrow 1.2 m doorway at `z in [7.4..8.6]`
fn build_room_app() -> App {
    build_room_app_with_tiles().0
}

/// [`build_room_app`] plus each resident tile's key and blob (to remove and re-add tiles).
fn build_room_app_with_tiles() -> (App, Vec<(TileKey, Vec<u8>)>) {
    let cfg = BuildConfig {
        class: AgentClass {
            id: H,
            radius: 0.3,
            height: 1.8,
            max_climb: 0.4,
            max_slope_deg: 45.0,
            can_swim: false,
        },
        cell_size: 0.2,
        cell_height: 0.1,
        tile_cells: 40, // 8 m tiles
    };

    let obstacles = [
        // Central pillar
        (Vec3::new(4.0, 0.0, 7.0), Vec3::new(6.0, 3.0, 9.0)),
        // Dividing wall with a 1.2 m doorway at z in [7.4..8.6]
        (Vec3::new(10.0, 0.0, 0.0), Vec3::new(10.6, 3.0, 7.4)),
        (Vec3::new(10.0, 0.0, 8.6), Vec3::new(10.6, 3.0, 16.0)),
    ];

    let mut soup = RoomSoup::default();
    soup.floor(0.0, 0.0, 16.0, 16.0, 0.0);
    let mut statics = StaticWorld::new(WorldPos::ORIGIN);
    for (lo, hi) in obstacles {
        soup.block(lo, hi);
        let shape = box_collision((hi - lo) * 0.5);
        statics.insert(
            &shape,
            WorldTransform::from_pos(WorldPos::ORIGIN.offset((lo + hi) * 0.5)),
        );
    }
    statics.rebuild();

    let areas = table();
    let mut backend = RecastBackend::new(RecastConfig::new(vec![cfg.clone()])).unwrap();
    let mut tiles = Vec::new();
    for tz in 0..2 {
        for tx in 0..2 {
            let input = TileInput {
                tile_x: tx,
                tile_z: tz,
                verts: soup.verts.clone(),
                tris: soup.tris.clone(),
                tri_areas: soup.areas.clone(),
                volumes: Vec::new(),
            };
            let blob = backend.build_tile(&input, &cfg, &areas).unwrap();
            let key = TileKey {
                ws: ws(),
                class: H,
                region: 0,
                x: tx,
                z: tz,
            };
            backend.add_tile(key.clone(), &blob).unwrap();
            tiles.push((key, blob));
        }
    }

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        ConsolePlugin,
        SfControllerPlugin::<RoomGround>::default(),
        NavPlugin,
        NavQueryPlugin,
        NavCorridorPlugin,
    ))
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .insert_resource(RoomGround {
        terrain: TerrainProbe(FlatFloor),
        statics,
    })
    .insert_resource(NavMeshes(Box::new(backend)));
    // Initial update binds tunables before spawning test walkers.
    app.update();
    (app, tiles)
}

fn inside_pillar_or_wall(p: Vec3) -> bool {
    let in_pillar = (4.05..=5.95).contains(&p.x) && (7.05..=8.95).contains(&p.z);
    let in_wall = (10.05..=10.55).contains(&p.x) && !(7.35..=8.65).contains(&p.z);
    in_pillar || in_wall
}

#[test]
fn walker_navigates_around_pillar_and_through_narrow_doorway_to_rest() {
    let mut app = build_room_app();
    let start = wpos(2.0, 0.0, 8.0);
    let goal = loc(13.5, 0.0, 8.0);

    let mut nav = NavCorridor::new(ActorKey(1), H, ws(), 2.2).with_filter(filter());
    nav.set_goal(goal.clone());
    let entity = app
        .world_mut()
        .spawn((
            Mover::new(start, 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            nav,
        ))
        .id();

    let mut passed_pillar = false;
    let mut passed_doorway = false;
    let mut arrived_tick = None;
    for tick in 1..=900 {
        app.update();
        let mover = app.world().get::<Mover>(entity).unwrap();
        let nav = app.world().get::<NavCorridor>(entity).unwrap();
        let p = xyz(mover.loco.position);
        assert!(
            !inside_pillar_or_wall(p),
            "entered obstacle at tick {tick}: {p:?}"
        );
        if (4.0..=6.0).contains(&p.x) && !(7.0..=9.0).contains(&p.z) {
            passed_pillar = true;
        }
        if (10.0..=10.6).contains(&p.x) && (7.4..=8.6).contains(&p.z) {
            passed_doorway = true;
        }
        assert!(nav.failed.is_none(), "corridor failed: {:?}", nav.failed);
        if nav.is_arrived() {
            arrived_tick = Some(tick);
            break;
        }
    }

    assert!(arrived_tick.is_some(), "walker did not arrive within 900 ticks");
    assert!(passed_pillar, "walker did not pass around the central pillar");
    assert!(passed_doorway, "walker did not pass through the 1.2 m doorway");

    // Let the velocity spring settle for a few ticks after arrival cut-off.
    for _ in 0..30 {
        app.update();
    }
    let mover = app.world().get::<Mover>(entity).unwrap();
    let nav = app.world().get::<NavCorridor>(entity).unwrap();
    let final_pos = xyz(mover.loco.position);
    let goal_pos = xyz(goal.pos);
    let dist = Vec2::new(final_pos.x - goal_pos.x, final_pos.z - goal_pos.z).length();
    assert!(
        dist <= nav.follower.stop_radius,
        "stopped {dist:.3} m from goal (stop_radius {:.3} m)",
        nav.follower.stop_radius
    );
    assert!(
        mover.loco.motion.velocity.length() < 0.02,
        "did not come to rest: velocity {:?}",
        mover.loco.motion.velocity
    );
}

#[test]
fn walker_repairs_corridor_after_lateral_push_and_reaches_goal() {
    let mut app = build_room_app();
    let start = wpos(2.0, 0.0, 8.0);
    let goal = loc(13.5, 0.0, 8.0);

    let mut nav = NavCorridor::new(ActorKey(2), H, ws(), 2.2).with_filter(filter());
    nav.set_goal(goal.clone());
    let entity = app
        .world_mut()
        .spawn((
            Mover::new(start, 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            nav,
        ))
        .id();

    let mut pushed = false;
    let mut passed_doorway = false;
    let mut arrived = false;
    for tick in 1..=1000 {
        app.update();
        {
            let mut mover = app.world_mut().get_mut::<Mover>(entity).unwrap();
            let p = xyz(mover.loco.position);
            assert!(
                !inside_pillar_or_wall(p),
                "entered obstacle at tick {tick}: {p:?}"
            );
            // Mid-walk beside the pillar, push the walker 1.5 m laterally away from the pillar.
            if !pushed && p.x >= 4.5 {
                let lateral = if p.z < 8.0 { -1.5 } else { 1.5 };
                mover.loco.position = mover.loco.position.offset(Vec3::new(0.0, 0.0, lateral));
                mover.snap();
                pushed = true;
            }
            if (10.0..=10.6).contains(&p.x) && (7.4..=8.6).contains(&p.z) {
                passed_doorway = true;
            }
        }
        let nav = app.world().get::<NavCorridor>(entity).unwrap();
        assert!(nav.failed.is_none(), "corridor failed: {:?}", nav.failed);
        if nav.is_arrived() {
            arrived = true;
            break;
        }
    }

    assert!(pushed, "never reached the pillar push trigger");
    assert!(passed_doorway, "never passed through the doorway after push");
    assert!(arrived, "did not reach goal after lateral push");

    for _ in 0..30 {
        app.update();
    }
    let mover = app.world().get::<Mover>(entity).unwrap();
    let nav = app.world().get::<NavCorridor>(entity).unwrap();
    let final_pos = xyz(mover.loco.position);
    let goal_pos = xyz(goal.pos);
    let dist = Vec2::new(final_pos.x - goal_pos.x, final_pos.z - goal_pos.z).length();
    assert!(
        dist <= nav.follower.stop_radius,
        "stopped {dist:.3} m from goal after push (stop_radius {:.3} m)",
        nav.follower.stop_radius
    );
    assert!(mover.loco.motion.velocity.length() < 0.02);
}

/// One observation between updates: records the in-flight request id (a request is held in
/// `pending` across at least one update boundary, its reply being taken on the next fixed tick,
/// so distinct ids count the requests issued), and returns `(failed, has corridor, arrived)`.
fn observe(app: &App, entity: Entity, seen: &mut Vec<RequestId>) -> (Option<NavError>, bool, bool) {
    let nav = app.world().get::<NavCorridor>(entity).unwrap();
    if let Some(id) = nav.pending
        && !seen.contains(&id)
    {
        seen.push(id);
    }
    (nav.failed.clone(), nav.corridor().is_some(), nav.is_arrived())
}

#[test]
fn failed_replan_requests_once_per_rearm_and_clears_on_success() {
    let (mut app, tiles) = build_room_app_with_tiles();
    let start = wpos(2.0, 0.0, 8.0);
    let goal = loc(13.5, 0.0, 8.0);

    let mut nav = NavCorridor::new(ActorKey(3), H, ws(), 2.2).with_filter(filter());
    nav.set_goal(goal.clone());
    let entity = app
        .world_mut()
        .spawn((
            Mover::new(start, 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            nav,
        ))
        .id();
    let mut seen = Vec::new();

    // 1. Walk with a valid corridor: one initial request, no replan.
    for _ in 0..30 {
        app.update();
        let (failed, _, _) = observe(&app, entity, &mut seen);
        assert!(failed.is_none(), "corridor failed early: {failed:?}");
    }
    assert_eq!(seen.len(), 1, "one initial request on the happy path: {seen:?}");
    assert!(app.world().get::<NavCorridor>(entity).unwrap().corridor().is_some());

    // 2. Remove the goal half of the room (x 8..16): the corridor trims to its valid prefix and
    //    replans; the goal (5.5 m from the remaining mesh, 2 m snap extents) is off the mesh.
    let (removed, kept): (Vec<_>, Vec<_>) = tiles.into_iter().partition(|(k, _)| k.x == 1);
    assert_eq!((removed.len(), kept.len()), (2, 2));
    {
        let mut meshes = app.world_mut().resource_mut::<NavMeshes>();
        for (k, _) in &removed {
            assert!(meshes.0.remove_tile(k));
        }
    }
    let mut failed_at = None;
    for tick in 1..=240 {
        app.update();
        let (failed, has_corridor, arrived) = observe(&app, entity, &mut seen);
        assert!(!arrived, "arrived on a goal that is off the mesh");
        if failed.is_some() && failed_at.is_none() {
            failed_at = Some(tick);
            assert!(has_corridor, "a failed replan keeps the old corridor's valid prefix");
        }
    }
    assert!(failed_at.is_some(), "the replan to an off-mesh goal never failed");
    let nav = app.world().get::<NavCorridor>(entity).unwrap();
    assert_eq!(nav.failed, Some(NavError::GoalOffMesh));
    assert!(nav.pending.is_none(), "no request in flight while failed");
    assert_eq!(
        seen.len(),
        2,
        "one replan after the tile removal, none while failed over 240 ticks"
    );

    // 3. Re-add the tiles: the mesh change re-arms one retry, which succeeds; `failed` clears and
    //    the walker resumes to the goal.
    {
        let mut meshes = app.world_mut().resource_mut::<NavMeshes>();
        for (k, blob) in &removed {
            meshes.0.add_tile(k.clone(), blob).unwrap();
        }
    }
    let mut cleared_at = None;
    let mut arrived = false;
    for tick in 1..=900 {
        app.update();
        let (failed, _, is_arrived) = observe(&app, entity, &mut seen);
        if failed.is_none() && cleared_at.is_none() {
            cleared_at = Some(tick);
            assert_eq!(seen.len(), 3, "exactly one retry after the mesh change");
        }
        if is_arrived {
            arrived = true;
            break;
        }
    }
    assert!(
        cleared_at.is_some_and(|t| t <= 3),
        "failed not cleared by the retry: {cleared_at:?}"
    );
    assert!(arrived, "walker did not resume to the goal after the retry");
    let final_pos = xyz(app.world().get::<Mover>(entity).unwrap().loco.position);
    let goal_pos = xyz(goal.pos);
    let dist = Vec2::new(final_pos.x - goal_pos.x, final_pos.z - goal_pos.z).length();
    assert!(dist < 1.0, "stopped {dist:.3} m from the goal");
}
