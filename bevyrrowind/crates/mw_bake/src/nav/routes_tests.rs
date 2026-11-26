//! `NV12c2`: the Morrowind route source over synthetic pathgrids, cells and door links (exact,
//! `simplify_m` 0), and one real-install measurement.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Instant;

use glam::Vec2;
use sf_bake::{Digest, Inputs, Store};
use sf_coords::WorldTransform;
use sf_nav::{AgentClassId, RouteBuildRules, RouteError, RouteGraph, RouteKind, RouteNodeId, TravelRules};
use sf_vfs::{MemSource, Vfs};
use sf_world::Location;
use tes3::esp::{CellFlags, FileType, Header, PathGrid, PathGridPoint, Plugin, TES3Object};

use super::*;

struct NoInputs;

impl Inputs for NoInputs {
    fn file_fingerprint(&self, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        Err(BakeError::Other(format!("no file {path}")))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

/// Builder rules of the exact tests: every point kept.
fn exact() -> RouteBuildRules {
    RouteBuildRules { simplify_m: 0.0, ..default_route_build_rules() }
}

fn ext() -> WorldspaceId {
    ids::exterior()
}

/// Absolute Morrowind units to an engine position.
fn mw(p: [i32; 3]) -> WorldPos {
    let [x, y, z] = p.map(f64::from);
    WorldPos::from_f64(glam::DVec3::new(x, z, -y) * UNIT_METERS_F64)
}

/// Metres to whole Morrowind units.
#[allow(clippy::cast_possible_truncation, reason = "test distances")]
fn units(m: f64) -> i32 {
    (m / UNIT_METERS_F64).round() as i32
}

/// Cell edge, units.
const CELL: i32 = 8192;
/// Mid-cell (y), units.
const MID: i32 = 4096;

/// A pathgrid record of `cell` at `grid` with cell-relative `points` and directed `links`.
fn pgrd(cell: &str, grid: (i32, i32), points: &[[i32; 3]], links: &[(u32, u32)]) -> PathGrid {
    let mut g = PathGrid { cell: cell.into(), ..PathGrid::default() };
    g.data.grid = grid;
    for (i, &location) in points.iter().enumerate() {
        let targets: Vec<u32> = links.iter().filter(|l| l.0 as usize == i).map(|l| l.1).collect();
        g.points.push(PathGridPoint {
            location,
            auto_generated: 0,
            connection_count: u8::try_from(targets.len()).unwrap(),
        });
        g.connections.extend(targets);
    }
    g
}

/// Both directions of each pair.
fn both(pairs: &[(u32, u32)]) -> Vec<(u32, u32)> {
    pairs.iter().flat_map(|&(a, b)| [(a, b), (b, a)]).collect()
}

/// A line of points along x (cell-relative) at y `MID`, connected both ways.
fn line(grid: (i32, i32), xs: &[i32]) -> PathGrid {
    let points: Vec<[i32; 3]> = xs.iter().map(|&x| [x, MID, 0]).collect();
    let pairs: Vec<(u32, u32)> = (1..u32::try_from(xs.len()).unwrap()).map(|i| (i - 1, i)).collect();
    pgrd("", grid, &points, &both(&pairs))
}

/// Exterior pathgrids without records.
fn grids(records: &[PathGrid]) -> PathGrids {
    let map: HashMap<CellKey, PathGraph> = records
        .iter()
        .map(|r| {
            let key = CellKey::Exterior(r.data.grid.0, r.data.grid.1);
            let g = PathGraph::from_record(r, &key).unwrap();
            (key, g)
        })
        .collect();
    PathGrids { grids: map, ..PathGrids::default() }
}

fn input_of(records: &[PathGrid], rules: &RouteSourceRules) -> (RouteInput, RouteSourceStats) {
    exterior_route_input(&grids(records), &BTreeMap::new(), &[], rules)
}

fn build(input: &RouteInput) -> RouteGraph {
    input.build(&ext(), &exact()).unwrap().0
}

fn kinds(g: &RouteGraph) -> (usize, usize) {
    let j = g.nodes.iter().filter(|n| n.kind == RouteNodeKind::Junction).count();
    let e = g.nodes.iter().filter(|n| n.kind == RouteNodeKind::End).count();
    (j, e)
}

/// Graph-only travel: no direct walk, joins only on the graph.
fn on_graph() -> TravelRules {
    TravelRules { join_max_m: 0.5, ..TravelRules::new(1.5, AgentClassId(0)) }
}

fn at(p: WorldPos) -> Location {
    Location::new(ext(), p)
}

/// 4 m (280 units) from the border between cells (0, 0) and (1, 0), on each side.
fn two_lines(gap_units: i32) -> [PathGrid; 2] {
    let near = CELL - gap_units / 2;
    let far = gap_units - gap_units / 2;
    [line((0, 0), &[near - 2000, near - 1000, near]), line((1, 0), &[far, far + 1000, far + 2000])]
}

#[test]
fn neighbouring_lines_join_into_one_edge() {
    let (input, stats) = input_of(&two_lines(560), &RouteSourceRules::default());
    assert_eq!((stats.nodes, stats.edges, stats.joins, stats.one_way), (6, 4, 1, 0));
    assert_eq!((input.nodes.len(), input.edges.len()), (6, 5));
    assert!(input.edges.iter().all(|e| e.two_way && e.kind == EdgeKind::Path && e.cost_m.is_none()));
    assert_eq!(stats.components, vec![6]);
    let g = build(&input);
    // One chain: two ends, one edge through all six points.
    assert_eq!((g.nodes.len(), g.edges.len(), g.points.len()), (2, 1, 6));
    assert_eq!(kinds(&g), (0, 2));
}

#[test]
fn a_gap_past_join_m_leaves_two_components() {
    let lines = two_lines(560);
    let (a, b) = (mw([CELL - 280, MID, 0]), mw([CELL + 280, MID, 0]));
    let gap = a.delta(b).length();
    let rules = RouteSourceRules { join_m: gap - 0.01, ..RouteSourceRules::default() };
    let (input, stats) = input_of(&lines, &rules);
    assert_eq!((stats.joins, stats.components.clone()), (0, vec![3, 3]));
    let g = build(&input);
    let (from, to) = (mw([CELL - 2280, MID, 0]), mw([CELL + 2280, MID, 0]));
    assert_eq!(g.route(&at(from), &at(to), &on_graph()).unwrap_err(), RouteError::Unreachable);
    // At exactly the gap it joins (≤).
    let (_, stats) = input_of(&lines, &RouteSourceRules { join_m: gap, ..RouteSourceRules::default() });
    assert_eq!(stats.joins, 1);
    let (input, _) = input_of(&lines, &RouteSourceRules::default());
    assert_eq!(build(&input).route(&at(from), &at(to), &on_graph()).unwrap().kind, RouteKind::Graph);
}

#[test]
fn joins_take_mutual_nearest_and_lower_index_on_a_tie() {
    // A: one node 1 m from the border; B: nodes 5 m and 3 m from it (that order).
    let one = units(1.0);
    let a = pgrd("", (0, 0), &[[CELL - one, MID, 0]], &[]);
    let b = pgrd("", (1, 0), &[[units(4.0), MID, 0], [units(2.0), MID, 0]], &[]);
    let (input, stats) = input_of(&[a.clone(), b], &RouteSourceRules::default());
    assert_eq!(stats.joins, 1);
    assert_eq!((input.edges[0].a, input.edges[0].b), (0, 2));
    // Two B nodes at one spot: the lower index joins.
    let b = pgrd("", (1, 0), &[[units(2.0), MID, 0], [units(2.0), MID, 0]], &[]);
    let (input, stats) = input_of(&[a, b], &RouteSourceRules::default());
    assert_eq!(stats.joins, 1);
    assert_eq!((input.edges[0].a, input.edges[0].b), (0, 1));
}

#[test]
fn diagonal_neighbours_join() {
    let d = units(2.0);
    // (0, 0) north-east corner to (1, 1) south-west corner: offset (1, 1).
    let a = pgrd("", (0, 0), &[[CELL - d, CELL - d, 0]], &[]);
    let b = pgrd("", (1, 1), &[[d, d, 0]], &[]);
    assert_eq!(input_of(&[a, b], &RouteSourceRules::default()).1.joins, 1);
    // (0, 1) south-east corner to (1, 0) north-west corner: offset (1, -1).
    let a = pgrd("", (0, 1), &[[CELL - d, d, 0]], &[]);
    let b = pgrd("", (1, 0), &[[d, CELL - d, 0]], &[]);
    assert_eq!(input_of(&[a, b], &RouteSourceRules::default()).1.joins, 1);
}

#[test]
fn a_y_shaped_grid_folds_to_one_junction_and_three_ends() {
    let c = [MID, MID, 0];
    let points = [c, [MID + 500, MID, 0], [MID + 1000, MID, 0], [MID - 500, MID + 300, 0], [MID - 1000, MID + 600, 0], [MID - 500, MID - 300, 0], [MID - 1000, MID - 600, 0]];
    let y = pgrd("", (0, 0), &points, &both(&[(0, 1), (1, 2), (0, 3), (3, 4), (0, 5), (5, 6)]));
    let (input, _) = input_of(&[y], &RouteSourceRules::default());
    let g = build(&input);
    assert_eq!(kinds(&g), (1, 3));
    assert_eq!((g.nodes.len(), g.edges.len()), (4, 3));
}

#[test]
fn a_one_way_connection_is_two_way_and_counted() {
    let points = [[1000, MID, 0], [2000, MID, 0], [3000, MID, 0]];
    let grid = pgrd("", (0, 0), &points, &[(0, 1), (1, 2), (2, 1)]);
    let (input, stats) = input_of(&[grid], &RouteSourceRules::default());
    assert_eq!((stats.edges, stats.one_way), (2, 1));
    let g = build(&input);
    let (p0, p2) = (mw([1000, MID, 0]), mw([3000, MID, 0]));
    for (from, to) in [(p0, p2), (p2, p0)] {
        assert_eq!(g.route(&at(from), &at(to), &on_graph()).unwrap().kind, RouteKind::Graph);
    }
}

// --- Records ---------------------------------------------------------------------------------

fn cell(name: &str, grid: (i32, i32), flags: CellFlags) -> TES3Object {
    let mut c = Cell { name: name.into(), ..Cell::default() };
    c.data.grid = grid;
    c.data.flags = flags;
    c.into()
}

fn db(mut objects: Vec<TES3Object>) -> RecordDb {
    objects.insert(0, Header { file_type: FileType::Esm, ..Header::default() }.into());
    let bytes = Plugin { objects }.save_bytes().unwrap();
    let vfs = Vfs::builder().mount(MemSource::new("data").with("Test.esm", bytes)).build();
    mw_records::load(&vfs, &["Test.esm".to_owned()]).unwrap().db
}

/// `TestTown` over (0, 0) and (1, 0) (centre: their shared border's middle). A 4-node line in
/// (0, 0) ending 30 m from the border; a 2-node pair in (1, 0) 1 m from it (nearer the centre,
/// smaller, unjoined). An unnamed cell (3, 3) with a grid; a named cell (8, 8) without one; a
/// tie town at (5, 5) whose two first nodes share a spot; an interior and a behave-like-exterior
/// interior with grids.
fn town() -> Vec<TES3Object> {
    let thirty = units(30.0);
    let one = units(1.0);
    let main = line((0, 0), &[CELL - thirty - 3000, CELL - thirty - 2000, CELL - thirty - 1000, CELL - thirty]);
    let small = line((1, 0), &[one, one + 500]);
    let unnamed = line((3, 3), &[1000, 2000]);
    let tie = pgrd("", (5, 5), &[[MID, MID, 0], [MID, MID, 0], [MID + 500, MID, 0]], &both(&[(0, 2), (1, 2)]));
    let interior = PathGrid { cell: "Room".into(), ..pgrd("", (0, 0), &[[0, 0, 0], [100, 0, 0]], &both(&[(0, 1)])) };
    let like_ext = PathGrid { cell: "Plaza".into(), ..pgrd("", (0, 0), &[[0, 0, 0], [100, 0, 0]], &both(&[(0, 1)])) };
    vec![
        cell("TestTown", (0, 0), CellFlags::empty()),
        cell("TestTown", (1, 0), CellFlags::empty()),
        cell("", (3, 3), CellFlags::empty()),
        cell("Emptyville", (8, 8), CellFlags::empty()),
        cell("TieTown", (5, 5), CellFlags::empty()),
        cell("Room", (0, 0), CellFlags::IS_INTERIOR),
        cell("Plaza", (0, 0), CellFlags::IS_INTERIOR | CellFlags::BEHAVES_LIKE_EXTERIOR),
        PathGrid { cell: "TestTown".into(), ..main }.into(),
        PathGrid { cell: "TestTown".into(), ..small }.into(),
        unnamed.into(),
        PathGrid { cell: "TieTown".into(), ..tie }.into(),
        interior.into(),
        like_ext.into(),
    ]
}

fn anchors(input: &RouteInput) -> BTreeMap<String, (usize, WorldPos)> {
    input
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match &n.kind {
            RouteNodeKind::Anchor(t) => Some((t.clone(), (i, n.pos))),
            _ => None,
        })
        .collect()
}

fn bake(source: MwRouteSource) -> (tempfile::TempDir, Baker, Vec<ProductKey>) {
    let dir = tempfile::tempdir().unwrap();
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), Arc::new(NoInputs));
    let bake = RouteBake::new(Arc::new(source), exact());
    bake.register(&mut baker);
    let keys = bake.products(&[ext(), ids::interior("Room"), ids::interior("Plaza")]);
    let r = baker.bake(&keys);
    let failures: Vec<String> = r.failures().map(|(k, e)| format!("{k}: {e}")).collect();
    assert!(failures.is_empty(), "{failures:?}");
    (dir, baker, keys)
}

fn product(baker: &Baker, key: &ProductKey) -> Vec<u8> {
    baker.store().get(&baker.product(key).unwrap()).unwrap()
}

#[test]
fn anchors_follow_the_group_rules() {
    let db = db(town());
    let (input, stats) = route_input_from_records(&db, &RouteSourceRules::default());
    assert_eq!((stats.groups, stats.anchors), (3, 2));
    assert_eq!(stats.unanchored, vec!["emptyville".to_owned()]);
    let a = anchors(&input);
    assert_eq!(a.keys().collect::<Vec<_>>(), ["testtown", "tietown"]);
    // The main line's border end, not the nearer node of the smaller pair.
    let thirty = units(30.0);
    assert_eq!(a["testtown"].1, mw([CELL - thirty, MID, 0]));
    // Two nodes at one spot: the lower index.
    let tie_base = input.nodes.iter().position(|n| n.pos == mw([5 * CELL + MID, 5 * CELL + MID, 0])).unwrap();
    assert_eq!(a["tietown"].0, tie_base);
    let (_dir, baker, keys) = bake(MwRouteSource::new(&db, RouteSourceRules::default()));
    let g = decode_routes(&product(&baker, &keys[0])).unwrap().unwrap();
    let id = g.anchor("testtown").unwrap();
    assert_eq!(g.nodes[g.node_index(id).unwrap()].pos, mw([CELL - thirty, MID, 0]));
}

#[test]
fn interior_grids_are_ignored_and_interiors_have_no_graph() {
    let db = db(town());
    let (_, stats) = route_input_from_records(&db, &RouteSourceRules::default());
    // Exterior nodes only: 4 + 2 + 2 + 3.
    assert_eq!((stats.grids, stats.nodes), (4, 11));
    let (_dir, baker, keys) = bake(MwRouteSource::new(&db, RouteSourceRules::default()));
    assert!(decode_routes(&product(&baker, &keys[0])).unwrap().is_some());
    for k in &keys[1..] {
        assert_eq!(k.step, ROUTES_STEP);
        assert!(decode_routes(&product(&baker, k)).unwrap().is_none(), "{k}");
    }
}

const ROUTES_STEP: &str = sf_nav_bake::ROUTES;

#[test]
fn record_order_never_shows() {
    let forward = db(town());
    let mut objects = town();
    objects.reverse();
    objects.rotate_left(3);
    let shuffled = db(objects);
    let encode = |db: &RecordDb| {
        let (input, _) = route_input_from_records(db, &RouteSourceRules::default());
        (postcard::to_stdvec(&input).unwrap(), build(&input).encode().unwrap())
    };
    assert_eq!(encode(&forward), encode(&shuffled));
}

// --- Door ends -------------------------------------------------------------------------------

/// An end in `ws` with its door at `door` and its arrival at `arrival` (absolute units), facing
/// north (Morrowind +y, engine -Z).
fn end(ws: WorldspaceId, door: [i32; 3], arrival: [i32; 3]) -> LinkEnd {
    let t = |p| WorldTransform { pos: mw(p), ..WorldTransform::IDENTITY };
    LinkEnd { worldspace: ws, frame: t(door), arrival: t(arrival), door: None, extent: Vec2::ZERO }
}

#[test]
fn door_nodes_sit_on_the_world_graph_approach() {
    // Cell (0, 0): a node in front of door 1 (1.5 m from its arrival), one behind it (nearer),
    // and a line far from door 2 (none within 4 m: the arrival).
    let (dx, dy) = (2000, 2000);
    let front = [dx, dy + 175, 0];
    let behind = [dx, dy - 14, 0];
    let grid = pgrd("", (0, 0), &[behind, front, [6000, 6000, 0], [7000, 6000, 0]], &both(&[(2, 3)]));
    let grids = grids(&[grid]);
    let room = ids::interior("Room");
    let one = DoorLink { a: end(ext(), [dx, dy, 0], [dx, dy + 70, 0]), b: end(room.clone(), [0, 0, 0], [0, 70, 0]), two_way: true };
    let two = DoorLink { a: end(room, [500, 0, 0], [500, 70, 0]), b: end(ext(), [2000, 6000, 0], [2000, 6070, 0]), two_way: true };
    let links = [one.clone(), two.clone()];
    let (input, stats) = exterior_route_input(&grids, &BTreeMap::new(), &links, &RouteSourceRules::default());
    assert_eq!((stats.door_pathgrid, stats.door_arrival, stats.door_interior), (1, 1, 2));
    assert_eq!(input.doors.len(), 2);
    let max = DOOR_APPROACH_M.default_value();
    let d1 = input.doors.iter().find(|d| d.link == one.id()).unwrap();
    assert_eq!((d1.side, d1.pos), (LinkSide::A, mw(front)));
    assert_eq!(Some(d1.pos), approach_point(&grids, &one.a, max));
    let d2 = input.doors.iter().find(|d| d.link == two.id()).unwrap();
    assert_eq!((d2.side, d2.pos), (LinkSide::B, two.b.arrival.pos));
    assert_eq!(approach_point(&grids, &two.b, max), None);
    let g = build(&input);
    for d in &input.doors {
        let id = RouteNodeId::door(&ext(), d.link, d.side);
        let n = &g.nodes[g.node_index(id).unwrap()];
        assert_eq!((n.pos, &n.kind), (d.pos, &RouteNodeKind::Door { link: d.link, side: d.side }));
    }
}

// --- Measurement -----------------------------------------------------------------------------

#[test]
#[ignore = "measurement: Vvardenfell route graph (counts vs nav.md Budgets, components, route timings)"]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss, clippy::many_single_char_names, reason = "one measurement printout")]
fn route_graph_over_the_real_install() {
    let explicit = std::env::var_os("MORROWIND_DIR").map(std::path::PathBuf::from);
    let (install, _) = mw_install::MorrowindInstall::detect(explicit.as_deref()).expect("Morrowind install");
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).expect("vfs").vfs;
    let content = mw_records::load(&vfs, &install.game_files()).expect("records");
    let db = content.db;

    let all = PathGrids::build(&db);
    let records = db.iter::<PathGrid>().count();
    let ext_records = db.iter::<PathGrid>().filter(|(_, r)| matches!(PathGrids::key_of(&db, r), CellKey::Exterior(..))).count();
    let ext_cells = db.iter::<Cell>().filter(|(_, c)| !c.is_interior()).count();
    println!("nv12c2: PGRD records {records} ({ext_records} exterior, {} empty, {} errors); exterior CELL records {ext_cells}", all.empty, all.errors.len());
    let strip: Vec<String> = (-4..=-1).flat_map(|x| (-3..=3).map(move |y| (x, y))).filter(|&(x, y)| all.get(&CellKey::Exterior(x, y)).is_some()).map(|c| format!("{c:?}")).collect();
    println!("nv12c2: grids in x -4..-1, y -3..3 (Balmora -3,-2 .. Caldera -2,2): {strip:?}");
    let t = Instant::now();
    let (input, s) = route_input_from_records(&db, &RouteSourceRules::default());
    let source_ms = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let (g, b) = input.build(&ext(), &default_route_build_rules()).unwrap();
    let build_ms = t.elapsed().as_secs_f64() * 1e3;
    let bytes = g.encode().unwrap().len();
    println!("nv12c2: input {} grids, {} nodes, {} pathgrid edges ({} one-way), {} joins, {} doors; source {source_ms:.1} ms", s.grids, s.nodes, s.edges, s.one_way, s.joins, input.doors.len());
    println!(
        "nv12c2: graph {} nodes (budget 5k), {} edges (6k), {} points (60k), {:.2} MiB (1.5); in {} nodes {} edges; collapsed {}, simplified {}, merged {}/{}; build {build_ms:.1} ms",
        g.nodes.len(),
        g.edges.len(),
        g.points.len(),
        bytes as f64 / (1024.0 * 1024.0),
        b.nodes_in,
        b.edges_in,
        b.collapsed_nodes,
        b.simplified_points,
        b.merged_nodes,
        b.merged_edges
    );
    println!("nv12c2: anchors {} of {} groups; without: {:?}", s.anchors, s.groups, s.unanchored);
    println!(
        "nv12c2: door nodes {} pathgrid / {} arrival ({} interior ends); joined {}, isolated {}",
        s.door_pathgrid, s.door_arrival, s.door_interior, b.doors_joined, b.doors_isolated
    );
    let mut hist: BTreeMap<&str, (usize, u32)> = BTreeMap::new();
    for &c in &s.components {
        let bucket = match c {
            1 => "a 1",
            2..=9 => "b 2-9",
            10..=99 => "c 10-99",
            100..=999 => "d 100-999",
            _ => "e 1000+",
        };
        let e = hist.entry(bucket).or_default();
        e.0 += 1;
        e.1 += c;
    }
    println!("nv12c2: {} components (largest {:?}); by size: {hist:?} (count, nodes)", s.components.len(), &s.components[..s.components.len().min(5)]);

    // The bake itself (cold store).
    let dir = tempfile::tempdir().unwrap();
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), Arc::new(NoInputs));
    let report = bake_routes(&mut baker, &db);
    println!("nv12c2: {report}");

    // Route timings.
    let rules = TravelRules::new(1.5, crate::nav::HUMANOID);
    let pos = |tag: &str| g.nodes[g.node_index(g.anchor(tag).expect(tag)).unwrap()].pos;
    let (balmora, caldera) = (pos("balmora"), pos("caldera"));
    let t = Instant::now();
    let bc = g.route(&at(balmora), &at(caldera), &rules);
    let us = t.elapsed().as_secs_f64() * 1e6;
    println!("nv12c2: balmora -> caldera {:?} ({:?} m), {us:.0} µs", bc.as_ref().map(|p| p.kind), bc.as_ref().map(|p| p.cost_m.round()));
    let anchors: Vec<(String, WorldPos)> = g
        .nodes
        .iter()
        .filter_map(|n| match &n.kind {
            RouteNodeKind::Anchor(t) => Some((t.clone(), n.pos)),
            _ => None,
        })
        .collect();
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = |n: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        usize::try_from(seed % n as u64).unwrap()
    };
    let (mut worst, mut total, mut kinds) = ((0.0, String::new()), 0.0, BTreeMap::<String, usize>::new());
    for _ in 0..50 {
        let (a, b) = (&anchors[next(anchors.len())], &anchors[next(anchors.len())]);
        let t = Instant::now();
        let r = g.route(&at(a.1), &at(b.1), &rules);
        let ms = t.elapsed().as_secs_f64() * 1e3;
        total += ms;
        *kinds.entry(r.map_or_else(|e| format!("{e:?}"), |p| format!("{:?}", p.kind))).or_default() += 1;
        if ms > worst.0 {
            worst = (ms, format!("{} -> {}", a.0, b.0));
        }
    }
    println!("nv12c2: 50 random anchor routes: avg {:.2} ms, worst {:.2} ms ({}); outcomes {kinds:?}", total / 50.0, worst.0, worst.1);
    // Pathgrids are per settlement: time routes that do connect (random node pairs, Graph only).
    let (mut n, mut tries, mut worst, mut total) = (0, 0, (0.0_f64, 0.0_f64), 0.0);
    while n < 50 && tries < 20_000 {
        tries += 1;
        let (a, b) = (g.nodes[next(g.nodes.len())].pos, g.nodes[next(g.nodes.len())].pos);
        let t = Instant::now();
        let r = g.route(&at(a), &at(b), &rules);
        let ms = t.elapsed().as_secs_f64() * 1e3;
        if let Ok(p) = r
            && p.kind == RouteKind::Graph
            && p.cost_m > 300.0
        {
            n += 1;
            total += ms;
            if ms > worst.0 {
                worst = (ms, p.cost_m);
            }
        }
    }
    println!("nv12c2: {n} connected node pairs (> 300 m, of {tries} tries): avg {:.2} ms, worst {:.2} ms ({:.0} m)", total / f64::from(n.max(1)), worst.0, worst.1);
    // Vanilla exterior PGRDs cover settlements only, so towns are separate components until
    // inter-town edges exist (NV12c3); that slice turns this into `assert_eq!(.., Ok(Graph))`.
    println!("nv12c2: balmora -> caldera is Graph: {}", bc.map(|p| p.kind) == Ok(RouteKind::Graph));
}
