//! The Morrowind [`RouteSource`] of `nav.routes` (`NV12c2`; `bevyrrowind/docs/specs/ai-adapter.md`
//! "Route source", `superfargoth/docs/specs/nav.md` "Route graph").
//!
//! One [`RouteInput`] for the exterior worldspace; interiors (those that behave like exteriors
//! too) give none.
//!
//! 1. Every node of every exterior PGRD is a node; each connected pair of points is one two-way
//!    `Path` edge (`cost_m: None`), one-way connections included (counted).
//! 2. Cross-cell joins: each pair of 8-neighbour cells once; two nodes join when each is the
//!    other's nearest in the other cell and they are within [`ROUTE_JOIN_M`] (3D, f64; ties
//!    lower point index). Ours: `OpenMW` keeps one graph per pathgrid (`pathgrid.cpp` 174).
//! 3. Settlement anchors: named exterior cells grouped by ASCII-lowercased name; one anchor per
//!    group, on the node nearest (XZ) the mean of the group's cell centres among the group's
//!    nodes in the largest component touching it.
//! 4. Door ends: each exterior end of a door link at `mw_ai::nav::approach_point` (MA21's rule,
//!    so door nodes sit on the `WorldGraph` approaches), else its arrival.
//!
//! Every choice breaks ties by `(cell x, cell y, point index)` or `LinkId`, never by record
//! order. The input is built once from the records; its postcard bytes are the step's plan
//! parameter (with both tunables), so a mod editing a PGRD, a cell name or a door re-bakes the
//! exterior, as `mw.nav.cell` does with its cell lists.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use mw_ai::nav::{DOOR_APPROACH_M, approach_point};
use mw_ai::pathgrid::{CellKey, PathGraph, PathGrids};
use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64};
use mw_world::ids;
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx};
use sf_coords::WorldPos;
use sf_nav::{EdgeKind, LinkSide, RouteNodeKind};
use sf_nav_bake::{RouteBake, RouteInput, RouteSource, decode_routes, default_route_build_rules};
use sf_records::RecordDb;
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_world::{DoorLink, LinkEnd, LinkId, WorldspaceId};
use tes3::esp::Cell;

#[cfg(test)]
#[path = "routes_tests.rs"]
mod tests;

/// `mw.nav.route.join_m`: [`RouteSourceRules::join_m`].
pub const ROUTE_JOIN_M: Def<f64> = def!("mw.nav.route.join_m" = 20.0_f64)
    .range(0.0, 117.0)
    .unit(Unit::Meters)
    .help("Farthest two pathgrid nodes of neighbouring exterior cells join in the route graph (each must be the other's nearest in the other cell)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

/// The route source's own tunables, for the host to register (`mw.nav.door_approach_m` is
/// MA21's, registered by the game).
pub const TUNABLES: &[&dyn AnyDef] = &[&ROUTE_JOIN_M];

/// Exterior cell edge, metres.
const CELL_M: f64 = CELL_UNITS * UNIT_METERS_F64;

/// The adapter values that shape the input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteSourceRules {
    /// Cross-cell join distance, metres ([`ROUTE_JOIN_M`]).
    pub join_m: f64,
    /// Door approach range, metres (`mw.nav.door_approach_m`).
    pub approach_m: f32,
}

impl Default for RouteSourceRules {
    fn default() -> Self {
        Self { join_m: ROUTE_JOIN_M.default_value(), approach_m: DOOR_APPROACH_M.default_value() }
    }
}

/// What building the input did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteSourceStats {
    /// Exterior pathgrids read.
    pub grids: usize,
    pub nodes: usize,
    /// Pathgrid edges (connected point pairs), joins not included.
    pub edges: usize,
    /// Stored connections without their reverse (made two-way).
    pub one_way: usize,
    pub joins: usize,
    /// Named exterior cell groups.
    pub groups: usize,
    pub anchors: usize,
    /// Groups with no pathgrid node (no anchor), by tag.
    pub unanchored: Vec<String>,
    /// Exterior door ends on a pathgrid node / on their arrival.
    pub door_pathgrid: usize,
    pub door_arrival: usize,
    /// Interior door ends (no node).
    pub door_interior: usize,
    /// Component sizes after joins (node count), largest first.
    pub components: Vec<u32>,
}

/// Named exterior cells by ASCII-lowercased name (region-only cells have none).
pub fn settlement_groups(db: &RecordDb) -> BTreeMap<String, BTreeSet<(i32, i32)>> {
    let mut out: BTreeMap<String, BTreeSet<(i32, i32)>> = BTreeMap::new();
    for (_, cell) in db.iter::<Cell>() {
        if !cell.is_interior() && !cell.name.is_empty() {
            out.entry(cell.name.to_ascii_lowercase()).or_default().insert(cell.data.grid);
        }
    }
    out
}

fn find(parent: &mut [u32], mut i: u32) -> u32 {
    while parent[i as usize] != i {
        let p = parent[i as usize];
        parent[i as usize] = parent[p as usize];
        i = p;
    }
    i
}

/// The nearest of `set` to `p` (3D squared distance, f64), lower index on a tie.
fn nearest(p: WorldPos, set: &[WorldPos]) -> Option<(usize, f64)> {
    let mut best: Option<(usize, f64)> = None;
    for (i, &q) in set.iter().enumerate() {
        let d2 = p.delta(q).length_squared();
        if best.is_none_or(|(_, b)| d2 < b) {
            best = Some((i, d2));
        }
    }
    best
}

fn idx(i: usize) -> u32 {
    u32::try_from(i).expect("route input over 2^32 nodes")
}

/// The exterior's route input from its pathgrids, settlement groups and door links.
#[allow(clippy::too_many_lines, reason = "the rules in order, one pass each")]
pub fn exterior_route_input(
    grids: &PathGrids,
    groups: &BTreeMap<String, BTreeSet<(i32, i32)>>,
    links: &[DoorLink],
    rules: &RouteSourceRules,
) -> (RouteInput, RouteSourceStats) {
    let mut stats = RouteSourceStats::default();
    // Exterior grids in (x, y) order: a node's global index orders by (x, y, point index).
    let cells: BTreeMap<(i32, i32), &PathGraph> = grids
        .grids
        .iter()
        .filter_map(|(k, g)| match *k {
            CellKey::Exterior(x, y) => Some(((x, y), g)),
            CellKey::Interior(_) => None,
        })
        .collect();
    let mut base: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    let mut pos: Vec<WorldPos> = Vec::new();
    for (&c, g) in &cells {
        base.insert(c, pos.len());
        pos.extend(g.positions());
    }
    stats.grids = cells.len();
    stats.nodes = pos.len();

    // 1. Pathgrid edges: one two-way edge per connected pair.
    let mut pairs: BTreeSet<(u32, u32)> = BTreeSet::new();
    for (c, g) in &cells {
        let b = base[c];
        stats.one_way += g.one_way_edges();
        for (i, j) in g.edges() {
            let (i, j) = (idx(b + i), idx(b + j));
            pairs.insert((i.min(j), i.max(j)));
        }
    }
    stats.edges = pairs.len();

    // 2. Cross-cell joins.
    let max_join_d2 = rules.join_m * rules.join_m;
    let mut joins: Vec<(u32, u32)> = Vec::new();
    for (&(x, y), ga) in &cells {
        for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
            let other = (x + dx, y + dy);
            let Some(gb) = cells.get(&other) else { continue };
            let (ba, bb) = (base[&(x, y)], base[&other]);
            let (pa, pb) = (&pos[ba..ba + ga.len()], &pos[bb..bb + gb.len()]);
            let a_of_b: Vec<Option<usize>> = pb.iter().map(|&p| nearest(p, pa).map(|(i, _)| i)).collect();
            for (i, &p) in pa.iter().enumerate() {
                if let Some((j, d2)) = nearest(p, pb)
                    && a_of_b[j] == Some(i)
                    && d2 <= max_join_d2
                {
                    joins.push((idx(ba + i), idx(bb + j)));
                }
            }
        }
    }
    stats.joins = joins.len();

    // Components (union-find over the input after joins).
    let mut parent: Vec<u32> = (0..idx(pos.len())).collect();
    for &(a, b) in pairs.iter().chain(&joins) {
        let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
        if ra != rb {
            // The lower index is the root: a root is its component's lowest node.
            parent[ra.max(rb) as usize] = ra.min(rb);
        }
    }
    let roots: Vec<u32> = (0..idx(pos.len())).map(|i| find(&mut parent, i)).collect();
    let mut size: BTreeMap<u32, u32> = BTreeMap::new();
    for &r in &roots {
        *size.entry(r).or_default() += 1;
    }
    stats.components = size.values().copied().collect();
    stats.components.sort_unstable_by(|a, b| b.cmp(a));

    // 3. Settlement anchors.
    let mut kinds: Vec<RouteNodeKind> = vec![RouteNodeKind::Junction; pos.len()];
    stats.groups = groups.len();
    for (tag, group) in groups {
        let nodes: Vec<usize> = group
            .iter()
            .filter_map(|c| Some((base.get(c).copied()?, cells[c].len())))
            .flat_map(|(b, n)| b..b + n)
            .collect();
        // Largest component touching the group; tie: the lowest root (its lowest node).
        let Some(comp) = nodes.iter().map(|&n| roots[n]).min_by_key(|&r| (std::cmp::Reverse(size[&r]), r)) else {
            stats.unanchored.push(tag.clone());
            continue;
        };
        #[allow(clippy::cast_precision_loss, reason = "a few cells")]
        let k = group.len() as f64;
        let (sx, sz) = group.iter().fold((0.0, 0.0), |(sx, sz), &(gx, gy)| {
            (sx + (f64::from(gx) + 0.5) * CELL_M, sz - (f64::from(gy) + 0.5) * CELL_M)
        });
        let (cx, cz) = (sx / k, sz / k);
        let mut best: Option<(usize, f64)> = None;
        for &n in nodes.iter().filter(|&&n| roots[n] == comp) {
            let p = pos[n].to_f64();
            let d2 = (p.x - cx).powi(2) + (p.z - cz).powi(2);
            if best.is_none_or(|(b, bd)| d2.total_cmp(&bd).then(n.cmp(&b)).is_lt()) {
                best = Some((n, d2));
            }
        }
        let (n, _) = best.expect("the component touches the group");
        kinds[n] = RouteNodeKind::Anchor(tag.clone());
        stats.anchors += 1;
    }

    let mut input = RouteInput::default();
    for (p, kind) in pos.iter().zip(kinds) {
        input.node(*p, kind);
    }
    for &(a, b) in pairs.iter().chain(&joins) {
        input.edge(a, b, EdgeKind::Path, true, Vec::new(), None);
    }

    // 4. Door ends, by (LinkId, side).
    let ext = ids::exterior();
    let mut ends: Vec<(LinkId, bool, &LinkEnd)> =
        links.iter().flat_map(|l| [(l.id(), false, &l.a), (l.id(), true, &l.b)]).collect();
    ends.sort_by_key(|&(id, b, _)| (id.0, b));
    ends.dedup_by_key(|&mut (id, b, _)| (id.0, b));
    for (id, b, end) in ends {
        if end.worldspace != ext {
            stats.door_interior += 1;
            continue;
        }
        let at = if let Some(p) = approach_point(grids, end, rules.approach_m) {
            stats.door_pathgrid += 1;
            p
        } else {
            stats.door_arrival += 1;
            end.arrival.pos
        };
        input.door(id, if b { LinkSide::B } else { LinkSide::A }, at);
    }
    (input, stats)
}

/// [`exterior_route_input`] over the records.
pub fn route_input_from_records(db: &RecordDb, rules: &RouteSourceRules) -> (RouteInput, RouteSourceStats) {
    let grids = PathGrids::build(db);
    let links: Vec<DoorLink> = mw_world::WorldRecords::build(db).door_links.into_iter().map(|(_, l)| l).collect();
    exterior_route_input(&grids, &settlement_groups(db), &links, rules)
}

/// The Morrowind route source: the exterior's input, built once from the records.
pub struct MwRouteSource {
    input: Option<RouteInput>,
    bytes: Vec<u8>,
    rules: RouteSourceRules,
    stats: RouteSourceStats,
}

impl MwRouteSource {
    pub fn new(db: &RecordDb, rules: RouteSourceRules) -> Self {
        let (input, stats) = route_input_from_records(db, &rules);
        Self::from_input(input, stats, rules)
    }

    /// A source over a built input (an input with no node and no door is no route graph).
    pub fn from_input(input: RouteInput, stats: RouteSourceStats, rules: RouteSourceRules) -> Self {
        let bytes = postcard::to_stdvec(&input).expect("route input serializes");
        let input = (!input.nodes.is_empty() || !input.doors.is_empty()).then_some(input);
        Self { input, bytes, rules, stats }
    }

    pub fn input(&self) -> Option<&RouteInput> {
        self.input.as_ref()
    }

    pub fn stats(&self) -> &RouteSourceStats {
        &self.stats
    }
}

impl RouteSource for MwRouteSource {
    fn plan(&self, ws: &WorldspaceId, cx: &mut PlanCx) -> Result<(), BakeError> {
        if *ws == ids::exterior() {
            cx.param(ROUTE_JOIN_M.name(), self.rules.join_m.to_le_bytes().to_vec())
                .param(DOOR_APPROACH_M.name(), self.rules.approach_m.to_le_bytes().to_vec())
                .param("mw.nav.route.input", self.bytes.clone());
        }
        Ok(())
    }

    fn route_input(&self, _cx: &mut RunCx<'_>, ws: &WorldspaceId) -> Result<Option<RouteInput>, BakeError> {
        Ok(if *ws == ids::exterior() { self.input.clone() } else { None })
    }
}

/// What [`bake_routes`] did.
#[derive(Debug, Default)]
pub struct RouteReport {
    pub source: RouteSourceStats,
    pub nodes: usize,
    pub edges: usize,
    pub points: usize,
    pub bytes: usize,
    /// Records to input.
    pub prepare: Duration,
    /// The `nav.routes` bake (a hit when nothing changed).
    pub bake: Duration,
    pub failures: Vec<(ProductKey, Arc<BakeError>)>,
}

impl std::fmt::Display for RouteReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = &self.source;
        write!(
            f,
            "routes: {} pathgrid nodes, {} edges ({} one-way), {} joins, {} anchors of {} groups, doors {} pathgrid / {} arrival; graph {} nodes, {} edges, {} points, {} bytes; prepare {:.2?}, bake {:.2?}, {} failed",
            s.nodes,
            s.edges,
            s.one_way,
            s.joins,
            s.anchors,
            s.groups,
            s.door_pathgrid,
            s.door_arrival,
            self.nodes,
            self.edges,
            self.points,
            self.bytes,
            self.prepare,
            self.bake,
            self.failures.len()
        )
    }
}

/// Registers `nav.routes` over [`MwRouteSource`] (default rules) and bakes the exterior's.
pub fn bake_routes(baker: &mut Baker, db: &RecordDb) -> RouteReport {
    let t = Instant::now();
    let source = Arc::new(MwRouteSource::new(db, RouteSourceRules::default()));
    let mut report = RouteReport { source: source.stats().clone(), prepare: t.elapsed(), ..RouteReport::default() };
    let bake = RouteBake::new(source, default_route_build_rules());
    bake.register(baker);
    let keys = bake.products(&[ids::exterior()]);
    let t = Instant::now();
    let r = baker.bake(&keys);
    report.bake = t.elapsed();
    report.failures.extend(r.failures().map(|(k, e)| (k.clone(), e.clone())));
    if let Ok(d) = baker.product(&keys[0])
        && let Ok(bytes) = baker.store().get(&d)
    {
        report.bytes = bytes.len();
        if let Ok(Some(g)) = decode_routes(&bytes) {
            (report.nodes, report.edges, report.points) = (g.nodes.len(), g.edges.len(), g.points.len());
        }
    }
    report
}
