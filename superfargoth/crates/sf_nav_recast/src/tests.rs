//! Synthetic-geometry tests (nav.md "Build plan" NV2): the NV1 contract scenarios on the real
//! backend (sliced = one-shot, tile removal, goal kinds, door patches, raycasts, area costs and
//! flags) plus geometry only Recast has: boxes, ramps within and over the slope limit, two floors
//! joined by stairs. Exact where the result is discrete (status, chosen route, errors, counts);
//! positions and costs within voxel tolerances. Sanity everywhere: finite, on the mesh.
#![allow(
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    reason = "synthetic layouts in small whole meters"
)]

use std::time::Instant;

use glam::{DVec3, Vec2, Vec3};
use sf_coords::WorldPos;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, AreaVolume, BuildConfig, DEFAULT_EXTENT,
    Goal, JobState, MAX_CLASSES, NavBackend, NavError, PathJob, PathQuery, PathResult, PathStatus,
    PolyFlags, PolyState, QueryFilter, SliceBudget, TileInput, TileKey,
};
use sf_world::{Location, WorldspaceId};

use crate::{RecastBackend, RecastConfig};

const H: AgentClassId = AgentClassId(0);
const L: AgentClassId = AgentClassId(1);
const GROUND: AreaId = AreaId(1);
const WATER: AreaId = AreaId(2);
const DOOR: AreaId = AreaId(3);
const SHUT: AreaId = AreaId(4);

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn loc(x: f32, y: f32, z: f32) -> Location {
    Location::new(
        ws(),
        WorldPos::from_f64(DVec3::new(x.into(), y.into(), z.into())),
    )
}

fn xyz(l: &Location) -> Vec3 {
    l.pos.to_f64().as_vec3()
}

fn table() -> AreaTable {
    let mut water = [1.0; MAX_CLASSES];
    water[H.index()] = 10.0;
    let mut shut = [1.0; MAX_CLASSES];
    shut[H.index()] = 100.0;
    let def = |id, name: &str, cost, flags| AreaDef {
        id,
        name: name.into(),
        cost,
        flags,
    };
    AreaTable::new(vec![
        def(GROUND, "ground", [1.0; MAX_CLASSES], PolyFlags::WALK),
        def(WATER, "water", water, PolyFlags::SWIM),
        def(DOOR, "door", [1.0; MAX_CLASSES], PolyFlags::DOOR),
        def(SHUT, "door_closed", shut, PolyFlags::DOOR),
    ])
    .unwrap()
}

fn class(id: AgentClassId) -> AgentClass {
    AgentClass {
        id,
        radius: if id == H { 0.3 } else { 0.8 },
        height: if id == H { 1.8 } else { 2.5 },
        max_climb: 0.4,
        max_slope_deg: 45.0,
        can_swim: false,
    }
}

/// 0.2 m voxels, 0.1 m high, 40 per tile: 8 m tiles (tests); erosion 2 cells = 0.4 m.
fn cfg(id: AgentClassId) -> BuildConfig {
    BuildConfig {
        class: class(id),
        cell_size: 0.2,
        cell_height: 0.1,
        tile_cells: 40,
    }
}

fn backend() -> RecastBackend {
    RecastBackend::new(RecastConfig::new(vec![cfg(H), cfg(L)])).unwrap()
}

fn key(x: i32, z: i32) -> TileKey {
    TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x,
        z,
    }
}

fn filter() -> QueryFilter {
    QueryFilter::for_class(&table(), H)
}

fn query(from: Location, goal: Goal) -> PathQuery {
    PathQuery::new(H, from, goal, filter())
}

/// A triangle soup in region-local meters.
#[derive(Clone, Default)]
struct Soup {
    verts: Vec<Vec3>,
    tris: Vec<[u32; 3]>,
    areas: Vec<AreaId>,
    volumes: Vec<AreaVolume>,
}

impl Soup {
    /// Corners in order `a b c d`; triangles `a b c`, `a c d` (Recast's +Y winding when `a b c d`
    /// goes (x0,z0) (x0,z1) (x1,z1) (x1,z0)).
    fn quad(&mut self, q: [Vec3; 4], area: AreaId) {
        let n = self.verts.len() as u32;
        self.verts.extend(q);
        self.tris.extend([[n, n + 1, n + 2], [n, n + 2, n + 3]]);
        self.areas.extend([area, area]);
    }

    fn floor(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, y: f32) {
        self.ramp(x0, z0, x1, z1, y, y);
    }

    /// A slope rising along +x from `y0` at `x0` to `y1` at `x1`.
    fn ramp(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, y0: f32, y1: f32) {
        self.quad(
            [
                Vec3::new(x0, y0, z0),
                Vec3::new(x0, y0, z1),
                Vec3::new(x1, y1, z1),
                Vec3::new(x1, y1, z0),
            ],
            GROUND,
        );
    }

    /// A closed box: walkable top, walls and bottom.
    fn block(&mut self, lo: Vec3, hi: Vec3) {
        self.floor(lo.x, lo.z, hi.x, hi.z, hi.y);
        let v = |x: f32, y: f32, z: f32| Vec3::new(x, y, z);
        let (a, b) = (lo, hi);
        self.quad(
            [
                v(a.x, a.y, a.z),
                v(b.x, a.y, a.z),
                v(b.x, a.y, b.z),
                v(a.x, a.y, b.z),
            ],
            GROUND,
        );
        self.quad(
            [
                v(a.x, a.y, a.z),
                v(a.x, b.y, a.z),
                v(b.x, b.y, a.z),
                v(b.x, a.y, a.z),
            ],
            GROUND,
        );
        self.quad(
            [
                v(a.x, a.y, b.z),
                v(b.x, a.y, b.z),
                v(b.x, b.y, b.z),
                v(a.x, b.y, b.z),
            ],
            GROUND,
        );
        self.quad(
            [
                v(a.x, a.y, a.z),
                v(a.x, a.y, b.z),
                v(a.x, b.y, b.z),
                v(a.x, b.y, a.z),
            ],
            GROUND,
        );
        self.quad(
            [
                v(b.x, a.y, a.z),
                v(b.x, b.y, a.z),
                v(b.x, b.y, b.z),
                v(b.x, a.y, b.z),
            ],
            GROUND,
        );
    }

    fn volume(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, area: AreaId) {
        self.volumes.push(AreaVolume {
            footprint: vec![
                Vec2::new(x0, z0),
                Vec2::new(x1, z0),
                Vec2::new(x1, z1),
                Vec2::new(x0, z1),
            ],
            min_y: -1.0,
            max_y: 4.0,
            area,
        });
    }

    fn input(&self, x: i32, z: i32) -> TileInput {
        TileInput {
            tile_x: x,
            tile_z: z,
            verts: self.verts.clone(),
            tris: self.tris.clone(),
            tri_areas: self.areas.clone(),
            volumes: self.volumes.clone(),
        }
    }
}

/// Bakes and loads tiles `0..nx × 0..nz` of `soup` for class H; returns how many had polygons.
fn load(b: &mut RecastBackend, soup: &Soup, nx: i32, nz: i32) -> usize {
    let mut n = 0;
    for z in 0..nz {
        for x in 0..nx {
            let blob = b.build_tile(&soup.input(x, z), &cfg(H), &table()).unwrap();
            if !blob.is_empty() {
                b.add_tile(key(x, z), &blob).unwrap();
                n += 1;
            }
        }
    }
    n
}

/// Floor `0..16 × 0..8` (two tiles).
fn strip() -> Soup {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 16.0, 8.0, 0.0);
    s
}

/// Floor `0..16 × 0..16` cut by a wall at x 7.5..8.5 with gaps at z 2..4 (a door) and 12..14.
fn two_rooms() -> Soup {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 16.0, 16.0, 0.0);
    for (z0, z1) in [(0.0, 2.0), (4.0, 12.0), (14.0, 16.0)] {
        s.block(Vec3::new(7.5, 0.0, z0), Vec3::new(8.5, 2.0, z1));
    }
    s.volume(7.4, 2.0, 8.6, 4.0, DOOR);
    s
}

fn backend_with(soup: &Soup, nx: i32, nz: i32) -> RecastBackend {
    let mut b = backend();
    assert_eq!(
        load(&mut b, soup, nx, nz),
        usize::try_from(nx * nz).unwrap()
    );
    b
}

fn path(b: &RecastBackend, from: Location, goal: Goal) -> PathResult {
    b.find_path_now(query(from, goal)).unwrap()
}

/// Finite, every waypoint on the mesh (it snaps to itself within a few centimeters).
fn assert_sane(b: &RecastBackend, r: &PathResult) {
    assert!(r.cost.is_finite() && r.cost >= 0.0, "cost {}", r.cost);
    assert!(!r.polys.is_empty() && !r.waypoints.is_empty());
    for w in &r.waypoints {
        let p = xyz(w);
        assert!(p.is_finite(), "waypoint {p}");
        let n = b
            .nearest(w, H, Vec3::new(0.05, 0.5, 0.05), &QueryFilter::default())
            .unwrap_or_else(|| panic!("waypoint {p} is off the mesh"));
        let q = xyz(&n.loc);
        assert!(
            Vec2::new(q.x - p.x, q.z - p.z).length() < 0.06,
            "{p} snaps to {q}"
        );
    }
}

fn close(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn straight_path_across_tiles() {
    let b = backend_with(&strip(), 2, 1);
    let r = path(&b, loc(1.0, 0.0, 4.0), Goal::Point(loc(15.0, 0.0, 4.0)));
    assert_eq!(r.status, PathStatus::Complete);
    assert_eq!(
        r.waypoints.len(),
        2,
        "an open floor is one straight segment"
    );
    assert!(r.polys.len() >= 2, "the corridor crosses the tile seam");
    assert!(close(r.cost, 14.0, 0.02), "cost {}", r.cost);
    let (a, e) = (xyz(&r.waypoints[0]), xyz(&r.waypoints[1]));
    assert_eq!((a.x, a.z, e.x, e.z), (1.0, 4.0, 15.0, 4.0));
    assert_sane(&b, &r);
}

#[test]
fn wall_gap_routes_through_it() {
    let b = backend_with(&two_rooms(), 2, 2);
    let r = path(&b, loc(2.0, 0.0, 6.0), Goal::Point(loc(14.0, 0.0, 6.0)));
    assert_eq!(r.status, PathStatus::Complete);
    assert!(r.cost > 12.5, "the wall forces a detour: cost {}", r.cost);
    // Every point inside the wall's band (plus the radius) is in the door gap.
    for w in &r.waypoints {
        let p = xyz(w);
        if (7.2..=8.8).contains(&p.x) {
            assert!((2.0..=4.0).contains(&p.z), "waypoint {p} crosses the wall");
        }
    }
    assert!(r.waypoints.iter().any(|w| (2.0..=4.0).contains(&xyz(w).z)));
    assert_sane(&b, &r);
}

fn platform(ramp_x0: f32) -> Soup {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, ramp_x0, 8.0, 0.0);
    s.ramp(ramp_x0, 2.0, 10.0, 6.0, 0.0, 2.0);
    s.block(Vec3::new(10.0, 0.0, 0.0), Vec3::new(16.0, 2.0, 8.0));
    s
}

#[test]
fn ramps_within_the_slope_limit_connect_steeper_ones_do_not() {
    // 2 m over 6 m: 18 degrees.
    let b = backend_with(&platform(4.0), 2, 1);
    let r = path(&b, loc(2.0, 0.0, 4.0), Goal::Point(loc(13.0, 2.0, 4.0)));
    assert_eq!(r.status, PathStatus::Complete);
    assert!(close(xyz(r.waypoints.last().unwrap()).y, 2.0, 0.15));
    assert_sane(&b, &r);
    // 2 m over 0.8 m: 68 degrees, over the 45 degree limit.
    let b = backend_with(&platform(9.2), 2, 1);
    let r = path(&b, loc(2.0, 0.0, 4.0), Goal::Point(loc(13.0, 2.0, 4.0)));
    assert_eq!(r.status, PathStatus::Partial);
    assert!(xyz(r.waypoints.last().unwrap()).y < 0.5, "stays below");
    assert_sane(&b, &r);
}

/// Ground floor `0..16 × 0..8`; an upper floor (slab, top at 3 m) over x 8..16; 15 steps of
/// 0.2 m × 0.4 m from x = 2 to 8 along z 0..3 lead up to it.
fn two_floors() -> Soup {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 16.0, 8.0, 0.0);
    s.block(Vec3::new(8.0, 2.8, 0.0), Vec3::new(16.0, 3.0, 8.0));
    for i in 0..15 {
        let x = 2.0 + 0.4 * i as f32;
        s.block(
            Vec3::new(x, 0.0, 0.0),
            Vec3::new(x + 0.4, 0.2 * (i + 1) as f32, 3.0),
        );
    }
    s
}

#[test]
fn two_floors_joined_by_stairs() {
    let b = backend_with(&two_floors(), 2, 1);
    let tight = Vec3::new(0.5, 0.5, 0.5);
    let down = b
        .nearest(&loc(12.0, 0.2, 6.0), H, tight, &filter())
        .unwrap();
    let up = b
        .nearest(&loc(12.0, 3.1, 6.0), H, tight, &filter())
        .unwrap();
    assert_ne!(down.poly, up.poly, "one polygon per floor");
    assert!(close(xyz(&down.loc).y, 0.0, 0.15) && close(xyz(&up.loc).y, 3.0, 0.15));
    let mut q = query(loc(12.0, 0.2, 6.0), Goal::Point(loc(12.0, 3.1, 6.0)));
    q.extent = tight;
    let r = b.find_path_now(q).unwrap();
    assert_eq!(r.status, PathStatus::Complete);
    let end = xyz(r.waypoints.last().unwrap());
    assert!(close(end.y, 3.0, 0.15), "ends upstairs: {end}");
    // Out from under the slab, round to the foot of the stairs, up, and back over the slab.
    assert!(
        r.waypoints.iter().any(|w| xyz(w).x < 4.0),
        "goes to the stairs"
    );
    assert!(r.cost > 18.0, "cost {}", r.cost);
    assert_sane(&b, &r);
}

fn door_polys(b: &RecastBackend) -> Vec<sf_nav::PolyRef> {
    (0..2)
        .flat_map(|z| (0..2).map(move |x| key(x, z)))
        .flat_map(|k| b.tile_polys(&k))
        .filter(|&p| b.poly_state(p).unwrap().area == DOOR)
        .collect()
}

#[test]
fn door_poly_patch_reroutes_without_rebuild() {
    let mut b = backend_with(&two_rooms(), 2, 2);
    let (from, to) = (loc(2.0, 0.0, 3.0), loc(14.0, 0.0, 3.0));
    let open = path(&b, from.clone(), Goal::Point(to.clone()));
    let doors = door_polys(&b);
    assert!(!doors.is_empty(), "the doorway has its own polygons");
    assert!(
        open.polys.iter().any(|p| doors.contains(p)),
        "the short way is the door"
    );
    for &d in &doors {
        assert_eq!(b.poly_state(d).unwrap().flags, PolyFlags::DOOR);
    }
    let tiles = b.resident_tiles();

    // Locked: excluded by the default filter, the long way round.
    let locked = PolyState {
        area: DOOR,
        flags: PolyFlags::DOOR | PolyFlags::DISABLED,
    };
    b.set_poly_state(&doors, locked).unwrap();
    let round = path(&b, from.clone(), Goal::Point(to.clone()));
    assert_eq!(round.status, PathStatus::Complete);
    assert!(!round.polys.iter().any(|p| doors.contains(p)));
    assert!(
        round.waypoints.iter().any(|w| xyz(w).z > 11.0),
        "through the far gap"
    );
    assert!(round.cost > open.cost + 10.0);
    assert_sane(&b, &round);

    // Shut but not locked: pathable at the shut-door cost (100 for H), so still the long way;
    // a class that pays 1 for it takes the door.
    let shut = PolyState {
        area: SHUT,
        flags: PolyFlags::DOOR,
    };
    b.set_poly_state(&doors, shut).unwrap();
    assert_eq!(
        path(&b, from.clone(), Goal::Point(to.clone())).polys,
        round.polys
    );
    let mut cheap = filter();
    cheap.set_area_cost(SHUT, 1.0);
    let r = b
        .find_path_now(PathQuery::new(
            H,
            from.clone(),
            Goal::Point(to.clone()),
            cheap,
        ))
        .unwrap();
    assert_eq!(r.polys, open.polys);

    // Reopened: the original path, bit for bit; no tile was touched.
    let reopen = PolyState {
        area: DOOR,
        flags: PolyFlags::DOOR,
    };
    b.set_poly_state(&doors, reopen).unwrap();
    assert_eq!(path(&b, from, Goal::Point(to)), open);
    assert_eq!(b.resident_tiles(), tiles);
}

#[test]
fn raycast_hits_the_wall_and_clears_the_gap() {
    let b = backend_with(&two_rooms(), 2, 2);
    let f = filter();
    let at = |x, z| b.nearest(&loc(x, 0.0, z), H, DEFAULT_EXTENT, &f).unwrap();
    let hit = b.raycast(&at(4.0, 6.0), &loc(12.0, 0.0, 6.0), &f).unwrap();
    assert!(hit.hit);
    // The mesh edge is the wall face less the 0.4 m erosion: x = 7.1, t = 3.1 / 8.
    assert!(close(hit.t, 3.1 / 8.0, 0.03), "t {}", hit.t);
    assert!(close(xyz(&hit.end).x, 4.0 + 8.0 * hit.t, 0.01));
    assert!(xyz(&hit.end).is_finite() && !hit.polys.is_empty());
    let clear = b.raycast(&at(4.0, 3.0), &loc(12.0, 0.0, 3.0), &f).unwrap();
    assert!(!clear.hit);
    assert_eq!(clear.t, 1.0);
    let e = xyz(&clear.end);
    assert_eq!((e.x, e.z), (12.0, 3.0));
    assert!(close(e.y, 0.0, 0.15));
    let elsewhere = Location::new(WorldspaceId::new("other"), loc(0.0, 0.0, 0.0).pos);
    assert!(matches!(
        b.raycast(&at(4.0, 3.0), &elsewhere, &f),
        Err(NavError::WrongWorldspace { .. })
    ));
}

fn run_sliced(b: &RecastBackend, q: PathQuery, per_step: u32) -> (PathResult, u32, u32) {
    let mut job = PathJob::new(q);
    let mut steps = 0;
    loop {
        steps += 1;
        match b.find_path(&mut job, &mut SliceBudget::new(per_step)) {
            JobState::InProgress => {}
            JobState::Done(r) => return (r, job.iterations(), steps),
            JobState::Failed(e) => panic!("{e}"),
        }
    }
}

#[test]
fn sliced_search_equals_one_shot() {
    let b = backend_with(&two_rooms(), 2, 2);
    let goals = [
        Goal::Point(loc(14.0, 0.0, 6.0)),
        Goal::Near {
            at: loc(14.0, 0.0, 6.0),
            radius: 2.0,
        },
        Goal::Away {
            from: loc(1.0, 0.0, 6.0),
            min_dist: 9.0,
        },
        Goal::AnyOf(vec![loc(14.0, 0.0, 14.0), loc(14.0, 0.0, 1.0)]),
    ];
    for g in goals {
        let q = query(loc(2.0, 0.0, 6.0), g);
        let (whole, iters, _) = run_sliced(&b, q.clone(), u32::MAX);
        assert_sane(&b, &whole);
        assert!(iters > 2, "{:?}: {iters} expansions", q.goal);
        for per_step in [1, 2, 5] {
            let (r, n, steps) = run_sliced(&b, q.clone(), per_step);
            assert_eq!(r, whole, "{:?} in slices of {per_step}", q.goal);
            assert_eq!(n, iters);
            assert!(steps >= iters.div_ceil(per_step));
        }
    }
    // One tick's budget shared by two jobs.
    let mut a = PathJob::new(query(loc(2.0, 0.0, 6.0), Goal::Point(loc(14.0, 0.0, 6.0))));
    let mut c = PathJob::new(query(loc(14.0, 0.0, 14.0), Goal::Point(loc(2.0, 0.0, 1.0))));
    let mut tick = SliceBudget::new(3);
    assert_eq!(b.find_path(&mut a, &mut tick), JobState::InProgress);
    assert_eq!(tick.remaining(), 0);
    assert_eq!(b.find_path(&mut c, &mut tick), JobState::InProgress);
    assert_eq!(c.iterations(), 0, "no budget left this tick");
}

#[test]
fn removing_a_touched_tile_fails_inflight_jobs_adding_does_not() {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 32.0, 8.0, 0.0);
    let mut b = backend();
    let blob = |b: &RecastBackend, x| b.build_tile(&s.input(x, 0), &cfg(H), &table()).unwrap();
    for x in 0..3 {
        let t = blob(&b, x);
        b.add_tile(key(x, 0), &t).unwrap();
    }
    let q = || query(loc(1.0, 0.0, 4.0), Goal::Point(loc(22.0, 0.0, 4.0)));
    let one = || SliceBudget::new(1);

    // Added between steps: tolerated.
    let mut job = PathJob::new(q());
    assert_eq!(b.find_path(&mut job, &mut one()), JobState::InProgress);
    let t3 = blob(&b, 3);
    b.add_tile(key(3, 0), &t3).unwrap();
    let JobState::Done(r) = b.find_path(&mut job, &mut SliceBudget::unlimited()) else {
        panic!("the job survives an added tile");
    };
    assert_eq!(r.status, PathStatus::Complete);

    // A tile the search never touched: tolerated (Detour tracks touched tiles).
    let mut job = PathJob::new(q());
    assert_eq!(b.find_path(&mut job, &mut one()), JobState::InProgress);
    assert!(b.remove_tile(&key(3, 0)));
    assert!(matches!(
        b.find_path(&mut job, &mut SliceBudget::unlimited()),
        JobState::Done(_)
    ));

    // The start tile: the job fails and must restart.
    let mut job = PathJob::new(q());
    assert_eq!(b.find_path(&mut job, &mut one()), JobState::InProgress);
    assert!(b.remove_tile(&key(0, 0)));
    assert_eq!(
        b.find_path(&mut job, &mut one()),
        JobState::Failed(NavError::MeshChanged)
    );
    assert_eq!(
        b.find_path(&mut job, &mut one()),
        JobState::Failed(NavError::JobFinished)
    );

    // The whole mesh dropped (last tile removed) and rebuilt: an old job fails too.
    let mut job = PathJob::new(query(loc(9.0, 0.0, 4.0), Goal::Point(loc(22.0, 0.0, 4.0))));
    assert_eq!(b.find_path(&mut job, &mut one()), JobState::InProgress);
    for x in 1..3 {
        assert!(b.remove_tile(&key(x, 0)));
    }
    for x in 0..3 {
        let t = blob(&b, x);
        b.add_tile(key(x, 0), &t).unwrap();
    }
    assert_eq!(
        b.find_path(&mut job, &mut one()),
        JobState::Failed(NavError::MeshChanged)
    );
}

#[test]
fn goal_kinds() {
    let b = backend_with(&strip(), 2, 1);
    let from = loc(2.0, 0.0, 4.0);

    let near = path(
        &b,
        from.clone(),
        Goal::Near {
            at: loc(14.0, 0.0, 4.0),
            radius: 3.0,
        },
    );
    assert_eq!(near.status, PathStatus::Complete);
    let end = xyz(near.waypoints.last().unwrap());
    assert!(
        close(end.distance(xyz(&loc(14.0, end.y, 4.0))), 3.0, 0.01),
        "{end}"
    );
    assert!(close(near.cost, 9.0, 0.02), "cost {}", near.cost);
    assert_sane(&b, &near);
    let already = path(
        &b,
        from.clone(),
        Goal::Near {
            at: loc(3.0, 0.0, 4.0),
            radius: 2.0,
        },
    );
    assert_eq!(
        (already.status, already.waypoints.len()),
        (PathStatus::Complete, 1)
    );

    let away = path(
        &b,
        loc(3.0, 0.0, 4.0),
        Goal::Away {
            from: from.clone(),
            min_dist: 6.0,
        },
    );
    assert_eq!(away.status, PathStatus::Complete);
    let end = xyz(away.waypoints.last().unwrap());
    assert!(end.distance(xyz(&from)) >= 6.0 - 0.15, "{end}");
    assert_sane(&b, &away);
    // Farther than the mesh reaches: the farthest polygon, partial.
    let cornered = path(
        &b,
        loc(3.0, 0.0, 4.0),
        Goal::Away {
            from: from.clone(),
            min_dist: 100.0,
        },
    );
    assert_eq!(cornered.status, PathStatus::Partial);
    assert!(xyz(cornered.waypoints.last().unwrap()).x > 10.0);

    // The cheaper of two slots, even listed second; off-mesh slots are skipped.
    let pick = path(
        &b,
        from.clone(),
        Goal::AnyOf(vec![
            loc(100.0, 0.0, 100.0),
            loc(14.0, 0.0, 4.0),
            loc(5.0, 0.0, 4.0),
        ]),
    );
    assert_eq!(pick.status, PathStatus::Complete);
    let end = xyz(pick.waypoints.last().unwrap());
    assert_eq!((end.x, end.z), (5.0, 4.0));
    assert!(close(pick.cost, 3.0, 0.02));
    let err = |g| b.find_path_now(query(from.clone(), g)).unwrap_err();
    assert_eq!(err(Goal::AnyOf(vec![])), NavError::EmptyGoal);
    assert_eq!(
        err(Goal::AnyOf(vec![loc(100.0, 0.0, 0.0)])),
        NavError::GoalOffMesh
    );
    assert_eq!(
        err(Goal::Point(loc(100.0, 0.0, 0.0))),
        NavError::GoalOffMesh
    );
    let off = b.find_path_now(query(loc(-50.0, 0.0, 0.0), Goal::Point(from.clone())));
    assert_eq!(off.unwrap_err(), NavError::StartOffMesh);
    let other = Location::new(WorldspaceId::new("other"), from.pos);
    assert!(matches!(
        err(Goal::Point(other)),
        NavError::WrongWorldspace { .. }
    ));
}

#[test]
fn unreachable_goal_gives_partial_path() {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 6.0, 8.0, 0.0);
    s.floor(10.0, 0.0, 16.0, 8.0, 0.0);
    let b = backend_with(&s, 2, 1);
    let r = path(&b, loc(1.0, 0.0, 4.0), Goal::Point(loc(15.0, 0.0, 4.0)));
    assert_eq!(r.status, PathStatus::Partial);
    let end = xyz(r.waypoints.last().unwrap());
    // The nearest point of the start island: its edge less the erosion (0.4 m) and the ledge
    // filter's cell at the drop (0.2 m): x = 5.4.
    assert!(close(end.x, 5.4, 0.05) && close(end.z, 4.0, 0.05), "{end}");
    assert_sane(&b, &r);
}

/// Floor `0..16 × 0..16` with water over x 6..10 for z below `water_z1`.
fn pond(water_z1: f32) -> Soup {
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 16.0, 16.0, 0.0);
    s.volume(6.0, -1.0, 10.0, water_z1, WATER);
    s
}

#[test]
fn area_costs_and_flags_change_the_route() {
    let b = backend_with(&pond(12.0), 2, 2);
    let (from, to) = (loc(2.0, 0.0, 4.0), loc(14.0, 0.0, 4.0));
    // Water costs 10 for H: round the pond's end.
    let dry = path(&b, from.clone(), Goal::Point(to.clone()));
    assert!(dry.waypoints.iter().any(|w| xyz(w).z > 11.5));
    assert_sane(&b, &dry);
    // Costing 1: straight through, a waypoint at each area change.
    let mut wade = filter();
    wade.set_area_cost(WATER, 1.0);
    let wet = b
        .find_path_now(PathQuery::new(
            H,
            from.clone(),
            Goal::Point(to.clone()),
            wade,
        ))
        .unwrap();
    assert_eq!(wet.waypoints.len(), 4);
    assert!(wet.waypoints.iter().all(|w| close(xyz(w).z, 4.0, 0.01)));
    assert!(close(wet.cost, 12.0, 0.02));
    assert!(wet.cost < dry.cost && dry.cost < 40.0 + 8.0);
    // A class that can't swim excludes the SWIM flag: also round.
    let no_swim = QueryFilter::for_agent(&table(), &class(H));
    let round = b
        .find_path_now(PathQuery::new(
            H,
            from.clone(),
            Goal::Point(to.clone()),
            no_swim.clone(),
        ))
        .unwrap();
    assert_eq!(round.status, PathStatus::Complete);
    assert!(round.waypoints.iter().any(|w| xyz(w).z > 11.5));
    // Water across the whole room: no way over for a non-swimmer.
    let b = backend_with(&pond(17.0), 2, 2);
    let r = b
        .find_path_now(PathQuery::new(H, from, Goal::Point(to), no_swim))
        .unwrap();
    assert_eq!(r.status, PathStatus::Partial);
    assert!(
        xyz(r.waypoints.last().unwrap()).x < 6.3,
        "stops at the shore"
    );
}

#[test]
fn build_tile_flags_come_from_the_area_table() {
    let b = backend_with(&pond(12.0), 2, 2);
    let t = table();
    let mut seen = Vec::new();
    for k in [key(0, 0), key(1, 0), key(0, 1), key(1, 1)] {
        for p in b.tile_polys(&k) {
            let s = b.poly_state(p).unwrap();
            assert_eq!(s.flags, t.flags(s.area), "area {:?}", s.area);
            seen.push(s.area);
        }
    }
    assert!(seen.contains(&GROUND) && seen.contains(&WATER));
    // Nothing walkable: no polygons, no blob.
    let mut wall = Soup::default();
    wall.block(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.2, 3.0, 8.0));
    assert!(
        b.build_tile(&wall.input(0, 0), &cfg(H), &t)
            .unwrap()
            .is_empty()
    );
    assert!(
        b.build_tile(&Soup::default().input(0, 0), &cfg(H), &t)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn nearest_snaps_within_extent() {
    let b = backend_with(&strip(), 2, 1);
    let f = filter();
    let p = b
        .nearest(&loc(4.0, 1.0, 4.0), H, DEFAULT_EXTENT, &f)
        .unwrap();
    let q = xyz(&p.loc);
    assert_eq!((q.x, q.z), (4.0, 4.0));
    assert!(close(q.y, 0.0, 0.15));
    assert!(
        b.nearest(&loc(4.0, 10.0, 4.0), H, DEFAULT_EXTENT, &f)
            .is_none()
    );
    assert!(
        b.nearest(&loc(4.0, 0.0, 4.0), L, DEFAULT_EXTENT, &f)
            .is_none(),
        "no large tiles"
    );
    let other = Location::new(WorldspaceId::new("other"), loc(4.0, 0.0, 4.0).pos);
    assert!(b.nearest(&other, H, DEFAULT_EXTENT, &f).is_none());
    assert!(
        b.nearest(&loc(30.0, 0.0, 4.0), H, DEFAULT_EXTENT, &f)
            .is_none()
    );
    let edge = b
        .nearest(&loc(17.0, 0.0, 4.0), H, DEFAULT_EXTENT, &f)
        .unwrap();
    // A floor edge (a drop): erosion 0.4 m plus the ledge filter's 0.2 m.
    assert!(close(xyz(&edge.loc).x, 15.4, 0.05), "{}", xyz(&edge.loc));
}

#[test]
fn region_origin_offsets_locations() {
    let origin = WorldPos::from_f64(DVec3::new(10_000.0, 50.0, -7_000.0));
    let mut b = backend();
    b.set_region_origin(&ws(), 0, origin).unwrap();
    assert_eq!(load(&mut b, &strip(), 2, 1), 2);
    let at = |x: f32, z: f32| Location::new(ws(), origin.offset(Vec3::new(x, 0.0, z)));
    let r = path(&b, at(1.0, 4.0), Goal::Point(at(15.0, 4.0)));
    assert_eq!(r.status, PathStatus::Complete);
    let w = r.waypoints[0].pos;
    let d = origin.delta_f32(w);
    assert_eq!((d.x, d.z), (1.0, 4.0));
    assert!(close(r.cost, 14.0, 0.02));
    assert!(matches!(
        b.set_region_origin(&ws(), 0, WorldPos::ORIGIN),
        Err(NavError::InvalidConfig(_))
    ));
    b.set_region_origin(&ws(), 0, origin).unwrap();
}

#[test]
fn tile_residency_errors_and_stale_refs() {
    let mut b = backend();
    let s = strip();
    let t0 = b.build_tile(&s.input(0, 0), &cfg(H), &table()).unwrap();
    let tile = b.add_tile(key(0, 0), &t0).unwrap();
    assert_eq!(b.tile(&key(0, 0)), Some(tile));
    assert_eq!(
        b.add_tile(key(0, 0), &t0),
        Err(NavError::TileLoaded(Box::new(key(0, 0))))
    );
    assert!(matches!(
        b.add_tile(key(5, 0), &t0),
        Err(NavError::InvalidTile { .. })
    ));
    assert!(matches!(
        b.add_tile(key(1, 0), &[]),
        Err(NavError::InvalidTile { .. })
    ));
    let unknown = TileKey {
        class: AgentClassId(7),
        ..key(0, 0)
    };
    assert!(matches!(
        b.add_tile(unknown, &t0),
        Err(NavError::InvalidTile { .. })
    ));
    assert!(matches!(
        b.add_tile(key(1, 0), &[1, 2, 3]),
        Err(NavError::InvalidTile { .. })
    ));

    let p = b
        .nearest(&loc(4.0, 0.0, 4.0), H, DEFAULT_EXTENT, &filter())
        .unwrap()
        .poly;
    assert!(b.poly_state(p).is_some());
    assert!(b.remove_tile(&key(0, 0)));
    assert!(!b.remove_tile(&key(0, 0)));
    assert_eq!(b.tile(&key(0, 0)), None);
    assert_eq!(b.poly_state(p), None);
    let st = PolyState {
        area: GROUND,
        flags: PolyFlags::WALK,
    };
    assert_eq!(b.set_poly_state(&[p], st), Err(NavError::UnknownPoly(p)));
    b.add_tile(key(0, 0), &t0).unwrap();
    assert_eq!(
        b.poly_state(p),
        None,
        "an old ref stays stale after a reload"
    );
    let p2 = b
        .nearest(&loc(4.0, 0.0, 4.0), H, DEFAULT_EXTENT, &filter())
        .unwrap()
        .poly;
    assert_ne!(p2, p);
    assert!(b.poly_state(p2).is_some());
    assert_eq!(b.resident_tiles(), 1);
    assert!(b.resident_bytes() > 0);
}

#[test]
fn classes_get_their_own_meshes() {
    // A 1.2 m gap: wide enough for the humanoid (0.6 m), not for the large class (1.6 m).
    let mut s = Soup::default();
    s.floor(0.0, 0.0, 16.0, 8.0, 0.0);
    s.block(Vec3::new(7.5, 0.0, 0.0), Vec3::new(8.5, 2.0, 3.4));
    s.block(Vec3::new(7.5, 0.0, 4.6), Vec3::new(8.5, 2.0, 8.0));
    let mut b = backend_with(&s, 2, 1);
    for x in 0..2 {
        let blob = b.build_tile(&s.input(x, 0), &cfg(L), &table()).unwrap();
        b.add_tile(
            TileKey {
                class: L,
                ..key(x, 0)
            },
            &blob,
        )
        .unwrap();
    }
    let f = filter();
    let h = b
        .nearest(&loc(4.0, 0.0, 4.0), H, DEFAULT_EXTENT, &f)
        .unwrap();
    let l = b
        .nearest(&loc(4.0, 0.0, 4.0), L, DEFAULT_EXTENT, &f)
        .unwrap();
    assert_ne!(h.poly, l.poly);
    let go = |c| {
        let q = PathQuery::new(
            c,
            loc(2.0, 0.0, 4.0),
            Goal::Point(loc(14.0, 0.0, 4.0)),
            f.clone(),
        );
        b.find_path_now(q).unwrap()
    };
    assert_eq!(go(H).status, PathStatus::Complete);
    let r = go(L);
    assert_eq!(r.status, PathStatus::Partial);
    assert!(xyz(r.waypoints.last().unwrap()).x < 7.5);
}

#[test]
fn config_validates() {
    let mut c = RecastConfig::new(vec![cfg(H), cfg(H)]);
    assert!(RecastBackend::new(c.clone()).is_err(), "a class twice");
    c.classes.pop();
    c.max_tiles = crate::MAX_TILES_PER_MESH + 1;
    assert!(RecastBackend::new(c.clone()).is_err());
    c.max_tiles = 16;
    assert!(RecastBackend::new(c).is_ok());
    let b = backend();
    let mut bad = strip().input(0, 0);
    bad.tri_areas.pop();
    assert!(matches!(
        b.build_tile(&bad, &cfg(H), &table()),
        Err(NavError::InvalidConfig(_))
    ));
}

/// A 6 × 6 tile town at the NV0 humanoid calibration (0.15 m voxels, 128 per tile: 19.2 m
/// tiles): ground, 6 × 6 m houses every 14 m with a 20 degree ramp up to every other roof.
#[test]
#[ignore = "measurement: ms per tile and query µs on a synthetic town"]
fn measure_bake_and_queries() {
    let real = BuildConfig {
        class: AgentClass {
            radius: 0.3,
            ..class(H)
        },
        cell_size: 0.15,
        cell_height: 0.1,
        tile_cells: 128,
    };
    let n = 6;
    let side = f32::from(real.tile_cells) * real.cell_size * n as f32;
    let mut s = Soup::default();
    s.floor(0.0, 0.0, side, side, 0.0);
    let mut i = 0;
    let mut x = 4.0;
    while x + 6.0 < side {
        let mut z = 4.0;
        while z + 6.0 < side {
            s.block(Vec3::new(x, 0.0, z), Vec3::new(x + 6.0, 3.0, z + 6.0));
            if i % 2 == 0 {
                s.ramp(x - 8.2, z + 1.0, x, z + 5.0, 0.0, 3.0);
            }
            i += 1;
            z += 14.0;
        }
        x += 14.0;
    }
    let mut b = RecastBackend::new(RecastConfig::new(vec![real.clone()])).unwrap();
    let t = table();
    let (mut ms, mut worst, mut bytes, mut add_us) = (0.0, 0.0f64, 0, 0.0);
    for tz in 0..n {
        for tx in 0..n {
            let input = s.input(tx, tz);
            let t0 = Instant::now();
            let blob = b.build_tile(&input, &real, &t).unwrap();
            let dt = t0.elapsed().as_secs_f64() * 1e3;
            ms += dt;
            worst = worst.max(dt);
            bytes += blob.len();
            let t1 = Instant::now();
            b.add_tile(key(tx, tz), &blob).unwrap();
            add_us += t1.elapsed().as_secs_f64() * 1e6;
        }
    }
    let tiles = f64::from(n * n);
    println!(
        "bake: {} tiles, {:.1} ms/tile mean, {worst:.1} ms worst, {:.1} KiB/tile, add_tile {:.0} µs",
        n * n,
        ms / tiles,
        bytes as f64 / tiles / 1024.0,
        add_us / tiles
    );
    let f = filter();
    let reps = 200;
    let t0 = Instant::now();
    for k in 0..reps {
        let p = 1.0 + (k % 50) as f32 * 2.0;
        let _ = b.nearest(&loc(p, 0.5, side - p), H, DEFAULT_EXTENT, &f);
    }
    let near_us = t0.elapsed().as_secs_f64() * 1e6 / f64::from(reps);
    let q = query(
        loc(2.0, 0.0, 2.0),
        Goal::Point(loc(side - 2.0, 0.0, side - 2.0)),
    );
    let first = b.find_path_now(q.clone()).unwrap();
    let t0 = Instant::now();
    for _ in 0..reps {
        let _ = b.find_path_now(q.clone()).unwrap();
    }
    let path_us = t0.elapsed().as_secs_f64() * 1e6 / f64::from(reps);
    let (_, iters, steps) = run_sliced(&b, q, 64);
    let from = b
        .nearest(&loc(2.0, 0.0, 2.0), H, DEFAULT_EXTENT, &f)
        .unwrap();
    let t0 = Instant::now();
    for _ in 0..reps {
        let _ = b.raycast(&from, &loc(side - 2.0, 0.0, 2.0), &f).unwrap();
    }
    let ray_us = t0.elapsed().as_secs_f64() * 1e6 / f64::from(reps);
    println!(
        "queries: nearest {near_us:.1} µs, {side:.0} m diagonal path {path_us:.0} µs \
         ({:?}, {} polys, {iters} expansions, {steps} slices of 64), raycast {ray_us:.1} µs",
        first.status,
        first.polys.len()
    );
}
