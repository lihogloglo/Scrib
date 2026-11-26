//! `MA21` approach points, door states and the plugin wiring on synthetic data, plus a real-install
//! measurement (ignored).

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use mw_ai::{CellKey, PathGraph, PathGrids};
use mw_world::convert::UNIT_METERS_F64;
use mw_world::ids;
use sf_coords::{WorldPos, WorldTransform};
use sf_nav::DoorState;
use sf_nav_bevy::{NavDoorStates, NavWorldGraph, NavWorldGraphStats};
use sf_vfs::{MemSource, Vfs};
use sf_world::WorldspaceId;
use sf_world_bevy::DoorLinks;
use tes3::esp::{
    Cell, CellFlags, Door, FileType, Header, PathGrid, PathGridPoint, Plugin, Reference, TES3Object, TravelDestination,
};

use super::*;

/// Morrowind units (x east, y north, z up) to an engine position.
fn mw(p: [i32; 3]) -> WorldPos {
    let [x, y, z] = p.map(f64::from);
    WorldPos::from_f64(glam::DVec3::new(x, z, -y) * UNIT_METERS_F64)
}

/// `points` (units) as `key`'s pathgrid, no edges.
fn grids(key: &CellKey, points: &[[i32; 3]]) -> PathGrids {
    let grid = PathGrid {
        points: points.iter().map(|&location| PathGridPoint { location, auto_generated: 0, connection_count: 0 }).collect(),
        ..PathGrid::default()
    };
    let graph = PathGraph::from_record(&grid, key).unwrap();
    PathGrids { grids: HashMap::from([(key.clone(), graph)]), ..PathGrids::default() }
}

/// An end in `ws` whose door stands at `door` and whose arrival is at `arrival`, facing north
/// (Morrowind +y, engine -Z).
fn end(ws: WorldspaceId, door: [i32; 3], arrival: [i32; 3]) -> LinkEnd {
    let at = |p| WorldTransform { pos: mw(p), ..WorldTransform::IDENTITY };
    LinkEnd { worldspace: ws, frame: at(door), arrival: at(arrival), door: None, extent: Vec2::ZERO }
}

// Door at the origin, arrival 70 units (1 m) north of it.
const DOOR: [i32; 3] = [0, 0, 0];
const ARRIVAL: [i32; 3] = [0, 70, 0];
/// In front, 105 units (1.5 m) from the arrival.
const FRONT: [i32; 3] = [0, 175, 0];
/// Behind the door, closer to the arrival (84 units, 1.2 m).
const BEHIND: [i32; 3] = [0, -14, 0];
/// In front, 350 units (5 m) from the arrival: past the 4 m default.
const FAR: [i32; 3] = [0, 420, 0];

#[test]
fn approach_is_the_nearest_front_node_in_range() {
    let room = ids::interior("Room");
    let all = grids(&CellKey::interior("Room"), &[FAR, BEHIND, FRONT]);
    let end = end(room, DOOR, ARRIVAL);
    let max = DOOR_APPROACH_M.default_value();
    assert_eq!(approach_point(&all, &end, max), Some(mw(FRONT)));
    // Nothing in front within range: no node.
    let only_far = grids(&CellKey::interior("room"), &[FAR, BEHIND]);
    assert_eq!(approach_point(&only_far, &end, max), None);
    // A wider range reaches the far node.
    assert_eq!(approach_point(&only_far, &end, 6.0), Some(mw(FAR)));
}

#[test]
fn approach_rule_falls_back_to_the_arrival_and_counts() {
    let room = ids::interior("Room");
    let counts = Arc::new(ApproachCounts::default());
    let with = link_rules(Arc::new(grids(&CellKey::interior("Room"), &[FAR, BEHIND, FRONT])), 4.0, counts.clone());
    let e = end(room.clone(), DOOR, ARRIVAL);
    assert_eq!((with.approach)(&e), Some(Location::new(room.clone(), mw(FRONT))));
    let without = link_rules(Arc::new(grids(&CellKey::interior("Room"), &[FAR, BEHIND])), 4.0, counts.clone());
    assert_eq!((without.approach)(&e), Some(Location::new(room, mw(ARRIVAL))));
    assert_eq!((counts.pathgrid.load(Ordering::Relaxed), counts.fallback.load(Ordering::Relaxed)), (1, 1));
}

#[test]
fn exterior_approach_searches_the_neighbouring_square() {
    // Arrival in square (0, 0) near its west edge; the node belongs to square (-1, 0)'s grid
    // (points relative to that square's south-west corner): absolute (-35, 140, 0).
    let west = grids(&CellKey::Exterior(-1, 0), &[[8192 - 35, 140, 0]]);
    let end = end(ids::exterior(), [35, 0, 0], [35, 70, 0]);
    assert_eq!(approach_point(&west, &end, 4.0), Some(mw([-35, 140, 0])));
}

// --- Door states, from records -------------------------------------------------------------

fn reference(refr_index: u32, id: &str, at: [f32; 3], cell: &str, to: [f32; 3], lock: Option<i32>) -> Reference {
    Reference {
        refr_index,
        id: id.into(),
        translation: at,
        destination: Some(TravelDestination { translation: to, rotation: [0.0; 3], cell: cell.into() }),
        lock_level: lock,
        ..Reference::default()
    }
}

fn cell(name: &str, grid: Option<(i32, i32)>, refs: Vec<Reference>) -> TES3Object {
    let mut cell = Cell { name: name.into(), ..Cell::default() };
    match grid {
        Some(g) => cell.data.grid = g,
        None => cell.data.flags = CellFlags::IS_INTERIOR,
    }
    for r in refs {
        cell.references.insert((r.mast_index, r.refr_index), r);
    }
    cell.into()
}

/// A paired Room <-> exterior link whose interior door is locked (level 50), and a one-way door
/// out of the Closet, unlocked (level 0).
fn content() -> RecordDb {
    let objects = vec![
        Header { file_type: FileType::Esm, ..Header::default() }.into(),
        Door { id: "door".into(), mesh: "d\\door.nif".into(), ..Door::default() }.into(),
        cell("Room", None, vec![reference(1, "door", [0.0; 3], "", [1000.0, 900.0, 0.0], Some(50))]),
        cell("Closet", None, vec![reference(2, "door", [0.0; 3], "", [-3000.0, 0.0, 0.0], Some(0))]),
        cell("", Some((0, 0)), vec![reference(3, "door", [1000.0, 1000.0, 0.0], "Room", [10.0, -100.0, 0.0], None)]),
    ];
    let bytes = Plugin { objects }.save_bytes().unwrap();
    let vfs = Vfs::builder().mount(MemSource::new("data").with("Test.esm", bytes)).build();
    mw_records::load(&vfs, &["Test.esm".to_owned()]).unwrap().db
}

#[test]
fn a_locked_door_reference_locks_its_link() {
    let db = content();
    let links = mw_doors_bevy::WorldLinks::build(&db).links;
    assert_eq!(links.len(), 2, "{links:?}");
    let room = links.iter().find(|l| l.two_way).unwrap();
    let closet = links.iter().find(|l| !l.two_way).unwrap();
    let states = door_states(&links, &locked_doors(&db));
    assert_eq!(states.0, HashMap::from([(room.id(), DoorState::Locked)]));
    assert_eq!(states.state(closet.id()), DoorState::Closed);
}

#[test]
fn the_game_builds_the_graph_with_its_rules() {
    let db = content();
    let links = mw_doors_bevy::WorldLinks::build(&db).links;
    let mut app = App::new();
    add(&mut app, Some(&db), &links);
    app.insert_resource(DoorLinks(links.clone()));
    app.update();
    let graph = &app.world().resource::<NavWorldGraph>().0;
    assert_eq!((graph.len(), graph.skipped()), (2, 0));
    assert_eq!(app.world().resource::<NavWorldGraphStats>().rebuilds, 1);
    let locked = app.world().resource::<NavDoorStates>();
    assert_eq!(locked.0.len(), 1);
    // The report logged the counts (no pathgrids: every end fell back) and reset them.
    let counts = &app.world().resource::<ApproachCounters>().0;
    assert_eq!((counts.pathgrid.load(Ordering::Relaxed), counts.fallback.load(Ordering::Relaxed)), (0, 0));
    // A changed range rebuilds the rules and so the graph.
    app.world_mut().resource_mut::<DoorApproach>().max_m = 8.0;
    app.update();
    assert_eq!(app.world().resource::<NavWorldGraphStats>().rebuilds, 2);
}

// --- Real install --------------------------------------------------------------------------

#[test]
#[ignore = "needs a Morrowind install"]
#[allow(clippy::cast_precision_loss)]
fn real_install_door_approaches_and_seyda_neen_route() {
    use sf_nav::RouteRules;
    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = &content.db;
    let links = mw_doors_bevy::WorldLinks::build(db).links;

    let start = std::time::Instant::now();
    let grids = Arc::new(PathGrids::build(db));
    let grids_ms = start.elapsed().as_secs_f64() * 1e3;
    let counts = Arc::new(ApproachCounts::default());
    let rules = link_rules(grids.clone(), DOOR_APPROACH_M.default_value(), counts.clone());
    let graph = rules.build(&links);
    let total_ms = start.elapsed().as_secs_f64() * 1e3;
    let locks_start = std::time::Instant::now();
    let states = door_states(&links, &locked_doors(db));
    let locks_ms = locks_start.elapsed().as_secs_f64() * 1e3;
    let (pathgrid, fallback) = (counts.pathgrid.load(Ordering::Relaxed), counts.fallback.load(Ordering::Relaxed));
    eprintln!(
        "{} door links: graph {} links ({} skipped); approach {pathgrid} from pathgrid, {fallback} arrival fallback \
         ({:.0}% pathgrid); {} locked; pathgrids {grids_ms:.1} ms + graph {:.1} ms = {total_ms:.1} ms, locks {locks_ms:.1} ms",
        links.len(),
        graph.len(),
        graph.skipped(),
        100.0 * pathgrid as f64 / (pathgrid + fallback).max(1) as f64,
        states.0.len(),
        total_ms - grids_ms,
    );
    assert!(graph.len() > 1000, "{}", graph.len());
    assert_eq!(pathgrid + fallback, 2 * graph.len());

    // Seyda Neen, outside, to the census office.
    let office = ids::interior("Seyda Neen, Census and Excise Office");
    let door = links
        .iter()
        .find(|l| [&l.a, &l.b].iter().any(|e| e.worldspace == office) && [&l.a, &l.b].iter().any(|e| e.worldspace == ids::exterior()))
        .expect("an office door to the exterior");
    let (outside, inside) = if door.a.worldspace == office { (&door.b, &door.a) } else { (&door.a, &door.b) };
    let from = Location::new(ids::exterior(), outside.arrival.pos.offset(Vec3::new(15.0, 0.0, 0.0)));
    let to = Location::new(office, inside.arrival.pos);
    let route = graph
        .route(&from, &to, &|id| states.state(id), &RouteRules::walking(1.5))
        .expect("a route into the office");
    eprintln!(
        "Seyda Neen -> census office: {} hop(s), {:.1} s (legs: {:?})",
        route.links.len(),
        route.cost_s,
        route.legs.iter().map(|l| (l.link, l.walk_s, l.traverse_s)).collect::<Vec<_>>()
    );
    assert_eq!(route.links.len(), 1);
}
