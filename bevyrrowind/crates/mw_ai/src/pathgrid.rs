//! PGRD pathgrids as directed graphs in engine space, with A* queries.
//!
//! A pathgrid is a hand-authored waypoint graph per cell (the TES3
//! construction set's "path grid"): points in Morrowind units, plus a flat
//! connection list where point `i` owns the next `connection_count` entries,
//! in point order. Connections are directed as stored; most come in pairs.
//! Interior points are absolute; exterior points are relative to the cell's
//! south-west corner (`grid * 8192` units), the rule `OpenMW` applies
//! (`CoordinateConverter`, GPL reference read for behavior).

use std::collections::HashMap;

use glam::DVec3;
use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{EdgeFiltered, EdgeRef};
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordId};
use tes3::esp::{Cell, PathGrid};

/// The cell a pathgrid belongs to.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CellKey {
    /// Exterior grid square (Morrowind `x`, `y`).
    Exterior(i32, i32),
    /// Interior cell name, ASCII-lowercased.
    Interior(String),
}

impl CellKey {
    pub fn interior(name: &str) -> Self {
        Self::Interior(name.to_ascii_lowercase())
    }

    pub fn of_cell(cell: &Cell) -> Self {
        if cell.is_interior() {
            Self::interior(&cell.name)
        } else {
            let (x, y) = cell.data.grid;
            Self::Exterior(x, y)
        }
    }

    /// The exterior grid square containing an engine position.
    #[allow(clippy::cast_possible_truncation)]
    pub fn exterior_at(pos: WorldPos) -> Self {
        let p = pos.to_f64() / (CELL_UNITS * UNIT_METERS_F64);
        Self::Exterior(p.x.floor() as i32, (-p.z).floor() as i32)
    }
}

/// A pathgrid record that cannot become a graph.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PathGridError {
    #[error("pathgrid {cell:?}: points claim {expected} connections, the record has {found}")]
    Connections {
        cell: String,
        expected: usize,
        found: usize,
    },
    #[error("pathgrid {cell:?}: point {point} connects to missing point {target}")]
    BadTarget {
        cell: String,
        point: usize,
        target: u32,
    },
}

/// One cell's pathgrid: nodes are engine positions, edges carry their
/// length in meters.
#[derive(Clone, Debug, Default)]
pub struct PathGraph {
    graph: DiGraph<WorldPos, f32>,
}

/// A Morrowind position in units (f64) to engine space.
fn to_engine(p: DVec3) -> WorldPos {
    WorldPos::from_f64(DVec3::new(p.x, p.z, -p.y) * UNIT_METERS_F64)
}

impl PathGraph {
    /// Builds the graph of `grid`, which belongs to `cell` (exterior points
    /// get the cell's corner added). Self-loops and duplicate connections
    /// are dropped.
    pub fn from_record(grid: &PathGrid, cell: &CellKey) -> Result<Self, PathGridError> {
        let offset = match *cell {
            CellKey::Exterior(x, y) => DVec3::new(f64::from(x), f64::from(y), 0.0) * CELL_UNITS,
            CellKey::Interior(_) => DVec3::ZERO,
        };
        let expected: usize = grid
            .points
            .iter()
            .map(|p| usize::from(p.connection_count))
            .sum();
        if expected != grid.connections.len() {
            return Err(PathGridError::Connections {
                cell: grid.cell.clone(),
                expected,
                found: grid.connections.len(),
            });
        }
        let mut graph = DiGraph::with_capacity(grid.points.len(), expected);
        for p in &grid.points {
            graph.add_node(to_engine(DVec3::from(p.location.map(f64::from)) + offset));
        }
        let mut next = 0;
        for (i, p) in grid.points.iter().enumerate() {
            let targets = &grid.connections[next..next + usize::from(p.connection_count)];
            next += targets.len();
            for &t in targets {
                let j = t as usize;
                if j >= grid.points.len() {
                    return Err(PathGridError::BadTarget {
                        cell: grid.cell.clone(),
                        point: i,
                        target: t,
                    });
                }
                let (a, b) = (NodeIndex::new(i), NodeIndex::new(j));
                if a != b && graph.find_edge(a, b).is_none() {
                    graph.add_edge(a, b, graph[a].delta_f32(graph[b]).length());
                }
            }
        }
        Ok(Self { graph })
    }

    pub fn len(&self) -> usize {
        self.graph.node_count()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }

    pub fn position(&self, node: usize) -> WorldPos {
        self.graph[NodeIndex::new(node)]
    }

    pub fn positions(&self) -> impl Iterator<Item = WorldPos> + '_ {
        self.graph.node_weights().copied()
    }

    /// Directed edges as `(from, to)`.
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.graph
            .edge_references()
            .map(|e| (e.source().index(), e.target().index()))
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Edges without a reverse edge.
    pub fn one_way_edges(&self) -> usize {
        self.graph
            .edge_references()
            .filter(|e| self.graph.find_edge(e.target(), e.source()).is_none())
            .count()
    }

    /// The node closest to `pos` other than those in `skip`.
    pub fn nearest_excluding(&self, pos: WorldPos, skip: &[usize]) -> Option<usize> {
        self.positions()
            .map(|p| pos.delta_f32(p).length_squared())
            .enumerate()
            .filter(|(i, _)| !skip.contains(i))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }

    /// The node closest to `pos` (3D, as `OpenMW` picks the start node).
    pub fn nearest(&self, pos: WorldPos) -> Option<usize> {
        self.positions()
            .map(|p| pos.delta_f32(p).length_squared())
            .enumerate()
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }

    /// Nodes within `radius` meters of `center` (3D).
    pub fn within(&self, center: WorldPos, radius: f32) -> Vec<usize> {
        let r2 = radius * radius;
        self.positions()
            .enumerate()
            .filter(|(_, p)| center.delta_f32(*p).length_squared() <= r2)
            .map(|(i, _)| i)
            .collect()
    }

    /// The shortest node path from `from` to `to` (both included) along
    /// directed edges, by A* with the straight-line heuristic.
    pub fn path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        self.path_avoiding(from, to, &[])
    }

    /// [`Self::path`] without the directed edges in `blocked` (edges an
    /// actor found it can't walk: replanning around them).
    pub fn path_avoiding(&self, from: usize, to: usize, blocked: &[(usize, usize)]) -> Option<Vec<usize>> {
        if from >= self.len() || to >= self.len() {
            return None;
        }
        let goal = NodeIndex::new(to);
        let goal_pos = self.graph[goal];
        let open = EdgeFiltered::from_fn(&self.graph, |e| !blocked.contains(&(e.source().index(), e.target().index())));
        let (_, nodes) = petgraph::algo::astar(
            &open,
            NodeIndex::new(from),
            |n| n == goal,
            |e| *e.weight(),
            |n| self.graph[n].delta_f32(goal_pos).length(),
        )?;
        Some(nodes.into_iter().map(NodeIndex::index).collect())
    }

    /// The node at `pos` (within 1 cm), if any.
    pub fn node_at(&self, pos: WorldPos) -> Option<usize> {
        self.nearest(pos).filter(|&n| self.position(n).delta_f32(pos).length() < 0.01)
    }

    /// Length of a node path in meters.
    pub fn path_length(&self, nodes: &[usize]) -> f32 {
        nodes
            .windows(2)
            .map(|w| self.position(w[0]).delta_f32(self.position(w[1])).length())
            .sum()
    }
}

/// Every loaded pathgrid, by cell.
#[derive(Debug, Default)]
pub struct PathGrids {
    pub grids: HashMap<CellKey, PathGraph>,
    /// Records that could not become graphs (skipped).
    pub errors: Vec<PathGridError>,
    /// Records with no points (skipped, as `OpenMW` does: issue #6209).
    pub empty: usize,
}

impl PathGrids {
    /// A grid's cell: an interior when an interior cell has its name,
    /// otherwise the exterior square of its grid coordinates.
    pub fn key_of(db: &RecordDb, grid: &PathGrid) -> CellKey {
        let interior = !grid.cell.is_empty()
            && db
                .get::<Cell>(&RecordId::new(&grid.cell))
                .is_some_and(Cell::is_interior);
        if interior {
            CellKey::interior(&grid.cell)
        } else {
            let (x, y) = grid.data.grid;
            CellKey::Exterior(x, y)
        }
    }

    /// Builds a graph for each `PGRD` record in the database.
    pub fn build(db: &RecordDb) -> Self {
        let mut out = Self::default();
        for (_, grid) in db.iter::<PathGrid>() {
            if grid.points.is_empty() {
                out.empty += 1;
                continue;
            }
            let key = Self::key_of(db, grid);
            match PathGraph::from_record(grid, &key) {
                Ok(graph) => {
                    out.grids.insert(key, graph);
                }
                Err(e) => out.errors.push(e),
            }
        }
        out
    }

    pub fn get(&self, cell: &CellKey) -> Option<&PathGraph> {
        self.grids.get(cell)
    }
}
