//! The route graph (nav.md "Route graph", NV12): the coarse level for long walks and every tier-0
//! actor. One graph per worldspace of roads, paths and streets (RDR2/GTA path nodes, UE `ZoneGraph`
//! lanes): nodes ([`RouteNode`]: junctions, ends, adapter anchors, door approaches) joined by
//! polyline edges ([`RouteEdge`]) whose cost is in **walk metres** (`cost_m`); a traveller's time
//! is `cost_m / speed_mps`, so per-class or per-actor speed applies at query time and nothing
//! re-bakes.
//!
//! - [`RouteGraphBuilder`] (`build`): the only way in. Merges equal keys, collapses degree-2
//!   chains, simplifies polylines, joins doors, splits long edges, clamps costs; input order never
//!   matters.
//! - [`RouteGraph::route`]: A* from any point to any point (straight joins at both ends, partial
//!   edges, a `Direct` straight walk when cheaper), giving a saveable [`RoutePath`].
//! - [`RouteGraph::position_at`]: where a traveller on a path is after `t_s` seconds (tier 0
//!   stores a path and progress, never a position).
//! - [`RouteGraph::nearest_point`]: the nearest road point (uniform XZ buckets, ring search).
//!
//! Ids are blake3 64-bit like [`LinkId`] (NV11a): [`RouteNodeId`] over the worldspace and the
//! node's key (a door's link and side, else its position quantised to [`ROUTE_ID_POS_STEP`]),
//! [`RouteEdgeId`] over both node ids, kind, direction and the quantised polyline. Nodes and edges
//! are sorted by id; dense indices follow that order. Everything is a pure function of the inputs:
//! costs sum in f64, ties break by index (= id) order.

#![allow(clippy::many_single_char_names, reason = "geometry: points a, b, q, segment fractions s, t")]

mod build;
mod query;

use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::sync::OnceLock;

use glam::Vec3;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sf_coords::WorldPos;
use sf_world::{LinkId, WorldspaceId};

use crate::types::{AgentClassId, ClassMask};
use crate::world_graph::LinkSide;

pub use build::{RouteBuildRules, RouteBuildStats, RouteGraphBuilder};

/// Bake product format of a [`RouteGraph`] ([`RouteGraph::encode`]); bump = rebuild.
pub const ROUTE_GRAPH_FORMAT: u16 = 1;

/// Longest edge, metres: longer ones are split, so a polyline's f32 offsets from its first node
/// stay within 0.5 mm. Format, not a tunable.
// tunables: const (format: f32 offset precision)
pub const MAX_EDGE_M: f32 = 4096.0;

/// Position quantum of the route ids, metres (1 cm). Format, not a tunable.
// tunables: const (format: id quantum)
pub const ROUTE_ID_POS_STEP: f64 = 0.01;

/// Default bucket edge of the nearest-point index, metres.
// tunables: const (bound from ai.nav.route.grid_cell_m in sf_nav_bevy, NV12f; Guess)
pub const DEFAULT_GRID_CELL_M: f64 = 64.0;

const NODE_ID_DOMAIN: &[u8] = b"sf.route.node/1";
const EDGE_ID_DOMAIN: &[u8] = b"sf.route.edge/1";

macro_rules! hex_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub u64);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:016x}", self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }

        /// Hex string in text formats (like [`LinkId`]), a plain `u64` in binary ones.
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                if s.is_human_readable() { s.collect_str(self) } else { s.serialize_u64(self.0) }
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                if d.is_human_readable() {
                    let s = String::deserialize(d)?;
                    u64::from_str_radix(&s, 16).map(Self).map_err(serde::de::Error::custom)
                } else {
                    u64::deserialize(d).map(Self)
                }
            }
        }
    };
}

hex_id!(
    /// A route node's stable id (see the module docs). Part of the save format: tier-0 routes
    /// name nodes and edges by id.
    RouteNodeId
);

hex_id!(
    /// A route edge's stable id (see the module docs).
    RouteEdgeId
);

impl RouteNodeId {
    /// The id of a non-door node at `pos` in `worldspace` (what [`RouteGraphBuilder::add_node`]
    /// returns for it).
    pub fn at(worldspace: &WorldspaceId, pos: WorldPos) -> Self {
        node_id(worldspace, &NodeKey::Pos(quantise_pos(pos)))
    }

    /// The id of the door node for `link`'s `side` end in `worldspace`.
    pub fn door(worldspace: &WorldspaceId, link: LinkId, side: LinkSide) -> Self {
        node_id(worldspace, &NodeKey::Door(link, side))
    }
}

/// What a node is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteNodeKind {
    /// Three or more edge ends (or a two-edge node the builder kept).
    Junction,
    /// A dead end (one edge end) or an isolated node.
    End,
    /// An adapter settlement or landmark tag ([`RouteGraph::anchor`]); never collapsed.
    Anchor(String),
    /// A door link end's approach point; never collapsed, joined to the nearest edge by a
    /// [`EdgeKind::Connector`] when the source gave it no edge.
    Door { link: LinkId, side: LinkSide },
}

/// One graph node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteNode {
    pub id: RouteNodeId,
    pub kind: RouteNodeKind,
    pub pos: WorldPos,
}

/// What an edge is: the traveller's [`TravelRules::kinds`] filters on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EdgeKind {
    Road,
    Path,
    Street,
    /// Builder-made join from a door to the nearest edge.
    Connector,
}

impl EdgeKind {
    fn tag(self) -> u8 {
        match self {
            Self::Road => 0,
            Self::Path => 1,
            Self::Street => 2,
            Self::Connector => 3,
        }
    }
}

bitflags::bitflags! {
    /// A set of [`EdgeKind`]s (UE ZoneGraph tag filters).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct EdgeKinds: u8 {
        const ROAD = 1;
        const PATH = 1 << 1;
        const STREET = 1 << 2;
        const CONNECTOR = 1 << 3;
    }
}

impl Default for EdgeKinds {
    fn default() -> Self {
        Self::all()
    }
}

impl EdgeKinds {
    pub fn of(kind: EdgeKind) -> Self {
        Self::from_bits_retain(1 << kind.tag())
    }

    pub fn has(self, kind: EdgeKind) -> bool {
        self.contains(Self::of(kind))
    }
}

/// One edge: a polyline from node `a` to node `b`, walkable from `a` to `b` (and back when
/// `two_way`; a two-way edge's `a` is its lower node id).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteEdge {
    pub id: RouteEdgeId,
    /// Dense node indices.
    pub a: u32,
    pub b: u32,
    pub kind: EdgeKind,
    pub two_way: bool,
    /// Who may walk it (the measured-cost slice clears a class whose walk fails).
    pub classes: ClassMask,
    /// Polyline length, metres (`cum_m` of its last point).
    pub length_m: f32,
    /// Walk metres to traverse it, at least `length_m`.
    pub cost_m: f32,
    /// Its points in [`RouteGraph::points`] / [`RouteGraph::cum_m`], first = node `a` (offset 0),
    /// last = node `b`; at least two.
    pub points: Range<u32>,
}

impl RouteEdge {
    fn allowed(&self, r: &TravelRules) -> bool {
        r.kinds.has(self.kind) && self.classes.contains(r.class)
    }

    /// Cost of walking `piece_m` metres of this edge (`cost_m × piece / length_m`).
    fn partial(&self, piece_m: f64) -> f64 {
        let (len, cost) = (f64::from(self.length_m), f64::from(self.cost_m));
        if piece_m >= len { cost } else { cost * piece_m / len }
    }
}

/// A worldspace's route graph (bake product `nav.routes`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteGraph {
    pub worldspace: WorldspaceId,
    /// Sorted by id.
    pub nodes: Vec<RouteNode>,
    /// Sorted by id.
    pub edges: Vec<RouteEdge>,
    /// Polyline points as offsets from their edge's node `a`, metres.
    pub points: Vec<Vec3>,
    /// Cumulative length along its edge at each point, metres (0 at the first).
    pub cum_m: Vec<f32>,
    /// Load-built adjacency, anchors and bucket index (never stored).
    #[serde(skip)]
    derived: Derived,
}

impl RouteGraph {
    /// The bake product bytes: format, then the graph (postcard).
    pub fn encode(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_stdvec(&(ROUTE_GRAPH_FORMAT, self))
    }

    /// `None` when the bytes are another [`ROUTE_GRAPH_FORMAT`] (rebuild).
    pub fn decode(bytes: &[u8]) -> Result<Option<Self>, postcard::Error> {
        let (format, rest): (u16, &[u8]) = postcard::take_from_bytes(bytes)?;
        if format != ROUTE_GRAPH_FORMAT {
            return Ok(None);
        }
        postcard::from_bytes(rest).map(Some)
    }

    /// Rebuilds the nearest-point index with `grid_cell_m` buckets (default
    /// [`DEFAULT_GRID_CELL_M`], built on first use).
    pub fn set_grid_cell(&mut self, grid_cell_m: f64) {
        self.derived.buckets = OnceLock::from(Buckets::build(self, grid_cell_m));
    }

    /// The node tagged `tag` (lowest id when several are).
    pub fn anchor(&self, tag: &str) -> Option<RouteNodeId> {
        let anchors = &self.adjacency().anchors;
        let i = anchors.partition_point(|(t, _)| t.as_str() < tag);
        anchors.get(i).filter(|(t, _)| t == tag).map(|&(_, n)| self.nodes[n as usize].id)
    }

    /// Dense index of node `id`.
    pub fn node_index(&self, id: RouteNodeId) -> Option<usize> {
        self.nodes.binary_search_by_key(&id, |n| n.id).ok()
    }

    /// Dense index of edge `id`.
    pub fn edge_index(&self, id: RouteEdgeId) -> Option<usize> {
        self.edges.binary_search_by_key(&id, |e| e.id).ok()
    }

    fn adjacency(&self) -> &Adjacency {
        self.derived.adjacency.get_or_init(|| Adjacency::build(self))
    }

    fn buckets(&self) -> &Buckets {
        self.derived.buckets.get_or_init(|| Buckets::build(self, DEFAULT_GRID_CELL_M))
    }

    fn edge_points(&self, e: &RouteEdge) -> &[Vec3] {
        &self.points[e.points.start as usize..e.points.end as usize]
    }

    fn edge_cum(&self, e: &RouteEdge) -> &[f32] {
        &self.cum_m[e.points.start as usize..e.points.end as usize]
    }
}

/// How a traveller uses the graph: its speed and class, which kinds of edge it takes, how far it
/// walks off the graph to join it and at what cost, and the search budget.
#[derive(Clone, Debug, PartialEq)]
pub struct TravelRules {
    /// Metres per second; a path's duration is `cost_m / speed_mps`.
    pub speed_mps: f64,
    pub class: AgentClassId,
    pub kinds: EdgeKinds,
    /// Farthest straight walk to or from the graph, metres; also the longest `Direct` walk.
    pub join_max_m: f64,
    /// Cost per metre of off-graph walking, at least 1 (smaller values count as 1).
    pub offroad_cost_mul: f64,
    /// A* node expansions before [`RouteError::Budget`].
    pub max_expansions: u32,
}

impl TravelRules {
    /// The defaults (nav.md "Route graph" tunables) for a traveller of `class` at `speed_mps`.
    pub fn new(speed_mps: f64, class: AgentClassId) -> Self {
        Self {
            speed_mps,
            class,
            kinds: EdgeKinds::all(),
            // tunables: const (bound from ai.nav.route.join_max_m in sf_nav_bevy, NV12f; Guess)
            join_max_m: 250.0,
            // tunables: const (bound from ai.nav.route.offroad_cost_mul in sf_nav_bevy, NV12f; Guess)
            offroad_cost_mul: 1.5,
            // tunables: const (bound from ai.nav.route.max_expansions in sf_nav_bevy, NV12f; Guess)
            max_expansions: 200_000,
        }
    }
}

/// Whether a path follows the graph or is one straight walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteKind {
    Graph,
    Direct,
}

/// A planned walk (what tier 0 saves with its elapsed seconds).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutePath {
    pub worldspace: WorldspaceId,
    pub speed_mps: f64,
    pub kind: RouteKind,
    /// At least one.
    pub legs: Vec<RouteLeg>,
    /// Walk metres, the sum of the legs' costs.
    pub cost_m: f64,
}

impl RoutePath {
    pub fn duration_s(&self) -> f64 {
        self.cost_m / self.speed_mps
    }
}

/// One leg of a [`RoutePath`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RouteLeg {
    /// Off-graph walk (joins, `Direct`), costing distance × `offroad_cost_mul`.
    Straight { a: WorldPos, b: WorldPos, cost_m: f64 },
    /// Along `edge` from `from_m` to `to_m` metres (`to_m < from_m` = against the polyline);
    /// `a` is where the leg starts.
    Edge { edge: RouteEdgeId, from_m: f32, to_m: f32, a: WorldPos, cost_m: f64 },
}

impl RouteLeg {
    pub fn cost_m(&self) -> f64 {
        match *self {
            Self::Straight { cost_m, .. } | Self::Edge { cost_m, .. } => cost_m,
        }
    }
}

/// Where a traveller is on a path.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutePose {
    pub loc: sf_world::Location,
    /// Unit XZ direction of travel (`x`, `z`).
    pub heading: glam::Vec2,
    /// Index of the current leg.
    pub leg: u16,
    /// At the path's end.
    pub done: bool,
}

/// A point on an edge.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutePoint {
    pub edge: RouteEdgeId,
    /// Metres along the edge's polyline.
    pub along_m: f32,
    pub pos: WorldPos,
    /// Distance from the query point, metres.
    pub dist_m: f64,
}

/// Which end of a route.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteEnd {
    From,
    To,
}

/// Why [`RouteGraph::route`] found no path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RouteError {
    #[error("route end is in another worldspace than the graph")]
    WrongSpace,
    #[error("no allowed edge within join reach of the {0:?} end")]
    NoJoin(RouteEnd),
    #[error("the ends are on different graph components")]
    Unreachable,
    #[error("route search went past its expansion budget")]
    Budget,
}

/// Derived, load-built state; equal for every graph (not part of the data).
#[derive(Clone, Default)]
struct Derived {
    adjacency: OnceLock<Adjacency>,
    buckets: OnceLock<Buckets>,
}

impl PartialEq for Derived {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for Derived {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Derived")
    }
}

/// One way out of a node.
#[derive(Clone, Copy)]
struct Arc {
    to: u32,
    edge: u32,
    /// Along the polyline (`a` → `b`).
    forward: bool,
}

/// CSR adjacency (per node, sorted by `(neighbour, edge id)`) and anchors sorted by
/// `(tag, node index)`.
#[derive(Clone)]
struct Adjacency {
    start: Vec<u32>,
    arcs: Vec<Arc>,
    anchors: Vec<(String, u32)>,
}

impl Adjacency {
    #[allow(clippy::cast_possible_truncation)] // dense indices fit u32
    fn build(g: &RouteGraph) -> Self {
        let mut per: Vec<Vec<Arc>> = vec![Vec::new(); g.nodes.len()];
        for (i, e) in g.edges.iter().enumerate() {
            per[e.a as usize].push(Arc { to: e.b, edge: i as u32, forward: true });
            if e.two_way {
                per[e.b as usize].push(Arc { to: e.a, edge: i as u32, forward: false });
            }
        }
        let mut start = Vec::with_capacity(per.len() + 1);
        let mut arcs = Vec::new();
        for mut list in per {
            start.push(arcs.len() as u32);
            list.sort_by_key(|a| (a.to, a.edge, !a.forward));
            arcs.extend(list);
        }
        start.push(arcs.len() as u32);
        let mut anchors: Vec<(String, u32)> = g
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| match &n.kind {
                RouteNodeKind::Anchor(tag) => Some((tag.clone(), i as u32)),
                _ => None,
            })
            .collect();
        anchors.sort();
        Self { start, arcs, anchors }
    }

    fn of(&self, node: usize) -> &[Arc] {
        &self.arcs[self.start[node] as usize..self.start[node + 1] as usize]
    }
}

/// Uniform XZ buckets of segments: each segment (by its first point's global index) is listed in
/// every bucket its AABB overlaps.
#[derive(Clone)]
struct Buckets {
    cell: f64,
    map: HashMap<(i64, i64), Vec<u32>>,
    /// Bucket key bounds (min x, min z, max x, max z); empty index = `None`.
    bounds: Option<(i64, i64, i64, i64)>,
    /// Edge index of each point.
    point_edge: Vec<u32>,
}

impl Buckets {
    #[allow(clippy::cast_possible_truncation)] // dense indices fit u32
    fn build(g: &RouteGraph, cell: f64) -> Self {
        let cell = if cell > 0.0 && cell.is_finite() { cell } else { DEFAULT_GRID_CELL_M };
        let mut map: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
        let mut point_edge = vec![0_u32; g.points.len()];
        let mut bounds: Option<(i64, i64, i64, i64)> = None;
        for (ei, e) in g.edges.iter().enumerate() {
            let origin = g.nodes[e.a as usize].pos.to_f64();
            for p in e.points.clone() {
                point_edge[p as usize] = ei as u32;
            }
            for p in e.points.start..e.points.end - 1 {
                let p0 = origin + g.points[p as usize].as_dvec3();
                let p1 = origin + g.points[p as usize + 1].as_dvec3();
                let (lo, hi) = (p0.min(p1), p0.max(p1));
                let (x0, z0) = (bucket(lo.x, cell), bucket(lo.z, cell));
                let (x1, z1) = (bucket(hi.x, cell), bucket(hi.z, cell));
                for x in x0..=x1 {
                    for z in z0..=z1 {
                        map.entry((x, z)).or_default().push(p);
                    }
                }
                bounds = Some(match bounds {
                    None => (x0, z0, x1, z1),
                    Some((a, b, c, d)) => (a.min(x0), b.min(z0), c.max(x1), d.max(z1)),
                });
            }
        }
        Self { cell, map, bounds, point_edge }
    }
}

#[allow(clippy::cast_possible_truncation)] // world coordinates / cell fit an i64
fn bucket(v: f64, cell: f64) -> i64 {
    (v / cell).floor() as i64
}

/// A node's identity before hashing: equal keys merge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NodeKey {
    Pos([i64; 3]),
    Door(LinkId, LinkSide),
}

#[allow(clippy::cast_possible_truncation)] // world coordinates / step fit an i64
fn quantise(v: f64) -> i64 {
    (v / ROUTE_ID_POS_STEP).round() as i64
}

fn quantise_pos(pos: WorldPos) -> [i64; 3] {
    pos.to_f64().to_array().map(quantise)
}

fn hash64(h: &blake3::Hasher) -> u64 {
    let bytes: [u8; 8] = h.finalize().as_bytes()[..8].try_into().expect("8-byte prefix");
    u64::from_le_bytes(bytes)
}

fn hash_space(h: &mut blake3::Hasher, worldspace: &WorldspaceId) {
    let name = worldspace.as_str().as_bytes();
    h.update(&(name.len() as u64).to_le_bytes());
    h.update(name);
}

/// Domain tag, length-prefixed worldspace id, then `0` + 3 × i64 position or `1` + u64 link id +
/// side byte, little-endian.
fn node_id(worldspace: &WorldspaceId, key: &NodeKey) -> RouteNodeId {
    let mut h = blake3::Hasher::new();
    h.update(NODE_ID_DOMAIN);
    hash_space(&mut h, worldspace);
    match key {
        NodeKey::Pos(q) => {
            h.update(&[0]);
            for c in q {
                h.update(&c.to_le_bytes());
            }
        }
        NodeKey::Door(link, side) => {
            h.update(&[1]);
            h.update(&link.0.to_le_bytes());
            h.update(&[u8::from(*side == LinkSide::B)]);
        }
    }
    RouteNodeId(hash64(&h))
}

/// Domain tag, both node ids, kind, direction byte, point count, then each point quantised
/// (3 × i64), little-endian.
fn edge_id(a: RouteNodeId, b: RouteNodeId, kind: EdgeKind, two_way: bool, points: &[WorldPos]) -> RouteEdgeId {
    let mut h = blake3::Hasher::new();
    h.update(EDGE_ID_DOMAIN);
    h.update(&a.0.to_le_bytes());
    h.update(&b.0.to_le_bytes());
    h.update(&[kind.tag(), u8::from(two_way)]);
    h.update(&(points.len() as u64).to_le_bytes());
    for p in points {
        for c in quantise_pos(*p) {
            h.update(&c.to_le_bytes());
        }
    }
    RouteEdgeId(hash64(&h))
}
