//! Pathgrids and wander planning on the real game data, around Balmora.
//! Skips (passes) when no Morrowind install is detected.

use mw_ai::{CellKey, PathGrids, Wander, WanderPlanner, WanderStep};
use mw_install::MorrowindInstall;
use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64, world_pos};
use sf_records::RecordId;
use sf_vfs::Vfs;
use tes3::esp::{Cell, Npc, PathGrid};

/// Per interior: wanderers with grid nodes in range, their nearest-node
/// distances, and their homes.
type InteriorWanderers = std::collections::BTreeMap<String, (u32, Vec<f32>, Vec<[f32; 3]>)>;

fn is_balmora(name: &str) -> bool {
    name.to_ascii_lowercase().starts_with("balmora")
}

#[test]
#[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
fn balmora_pathgrids_and_wanderers() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = &content.db;

    let grids = PathGrids::build(db);
    eprintln!(
        "{} pathgrids, {} empty, {} errors",
        grids.grids.len(),
        grids.empty,
        grids.errors.len()
    );
    assert!(grids.grids.len() > 1000, "{}", grids.grids.len());
    assert!(
        grids.errors.len() * 100 < grids.grids.len(),
        "{:?}",
        grids.errors
    );

    // Every Balmora pathgrid parses; exterior points fall inside their square.
    let (mut balmora, mut exterior, mut points, mut edges, mut one_way) = (0, 0, 0, 0, 0);
    for (_, grid) in db
        .iter::<PathGrid>()
        .filter(|(_, g)| is_balmora(&g.cell) && !g.points.is_empty())
    {
        let key = PathGrids::key_of(db, grid);
        let graph = grids
            .get(&key)
            .unwrap_or_else(|| panic!("{key:?} did not parse"));
        balmora += 1;
        points += graph.len();
        edges += graph.edge_count();
        one_way += graph.one_way_edges();
        assert!(
            graph
                .edges()
                .all(|(a, b)| a < graph.len() && b < graph.len())
        );
        if let CellKey::Exterior(gx, gy) = key {
            exterior += 1;
            let tol = 256.0;
            for p in graph.positions() {
                let e = p.to_f64() / UNIT_METERS_F64;
                let (x, y) = (e.x, -e.z);
                let (x0, y0) = (f64::from(gx) * CELL_UNITS, f64::from(gy) * CELL_UNITS);
                assert!(
                    (x0 - tol..=x0 + CELL_UNITS + tol).contains(&x)
                        && (y0 - tol..=y0 + CELL_UNITS + tol).contains(&y),
                    "{key:?}: point ({x}, {y}) outside its cell"
                );
            }
        }
    }
    eprintln!(
        "Balmora: {balmora} grids ({exterior} exterior), {points} points, {edges} edges ({one_way} one-way)"
    );
    assert!(balmora > 20 && exterior >= 2, "{balmora} / {exterior}");

    // Balmora NPCs with a wander radius: near their grid, and their goals
    // are reachable grid nodes within the radius of home.
    let mut rng = fastrand::Rng::with_seed(5);
    let (mut wanderers, mut with_nodes, mut grid_walks) = (0, 0, 0);
    let mut nearest_ext = Vec::new();
    // Interiors: (wanderers with grid nodes in range, nearest-node distances).
    let mut interiors = InteriorWanderers::new();
    for (_, cell) in db.iter::<Cell>().filter(|(_, c)| is_balmora(&c.name)) {
        for r in cell.references.values() {
            let Some(npc) = db.get::<Npc>(&RecordId::new(&r.id)) else {
                continue;
            };
            let Some(mut wander) = Wander::of_npc(npc).filter(|w| w.distance > 0.0) else {
                continue;
            };
            wanderers += 1;
            let home = world_pos(r.translation);
            let key = if cell.is_interior() {
                CellKey::of_cell(cell)
            } else {
                CellKey::exterior_at(home)
            };
            let Some(graph) = grids.get(&key) else {
                continue;
            };
            let nearest = graph
                .position(graph.nearest(home).unwrap())
                .delta_f32(home)
                .length();
            if !cell.is_interior() {
                nearest_ext.push(nearest);
            }
            if graph.within(home, wander.distance).is_empty() {
                continue;
            }
            with_nodes += 1;
            if cell.is_interior() {
                let e = interiors.entry(cell.name.clone()).or_default();
                e.0 += 1;
                e.1.push(nearest);
                e.2.push(home.delta_f32(sf_coords::WorldPos::ORIGIN).to_array().map(|v| -v));
            }
            wander.idle_chances = [0; 8];
            let planner = WanderPlanner::new(wander, home);
            if let WanderStep::Walk {
                nodes,
                via_grid: true,
            } = planner.next(Some(graph), home, &mut rng)
            {
                let goal = nodes.last().unwrap().pos;
                assert!(
                    home.delta_f32(goal).length() <= wander.distance + 1e-3,
                    "{}",
                    r.id
                );
                grid_walks += 1;
            }
        }
    }
    nearest_ext.sort_by(f32::total_cmp);
    let median = nearest_ext
        .get(nearest_ext.len() / 2)
        .copied()
        .unwrap_or(f32::NAN);
    eprintln!(
        "{wanderers} wandering NPCs, {with_nodes} with grid nodes in range, {grid_walks} grid walks; \
         exterior NPC to nearest node median {median:.2} m over {}",
        nearest_ext.len()
    );
    // Interior wanderers walk their cell's grid (interior points are
    // absolute, like interior references): homes sit on the grid.
    for (name, (n, nearest, homes)) in &interiors {
        let worst = nearest.iter().copied().fold(0.0_f32, f32::max);
        eprintln!(
            "interior {name}: {n} wanderers with nodes, nearest node <= {worst:.1} m, homes {homes:.1?}"
        );
    }
    assert!(!interiors.is_empty(), "no Balmora interior wanderers");
    let all: Vec<f32> = interiors.values().flat_map(|(_, d, _)| d.iter().copied()).collect();
    let near = all.iter().filter(|&&d| d < 8.0).count();
    assert!(near * 10 >= all.len() * 8, "{near} / {} interior homes near a node", all.len());
    assert!(wanderers >= 10, "{wanderers}");
    assert!(
        with_nodes * 10 >= wanderers * 7,
        "{with_nodes} / {wanderers}"
    );
    assert!(
        grid_walks * 10 >= with_nodes * 9,
        "{grid_walks} / {with_nodes}"
    );
    // The exterior offset rule: street NPCs stand near their grid.
    assert!(nearest_ext.len() >= 3 && median < 8.0, "median {median} m");
}
