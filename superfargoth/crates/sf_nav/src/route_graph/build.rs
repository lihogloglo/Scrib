//! [`RouteGraphBuilder`]: the only way to make a [`RouteGraph`]. Pure and order-independent:
//! every pass walks nodes and edges in key order, so shuffled input gives identical bytes.
//!
//! Passes: merge equal node keys and equal edges, collapse degree-2 chains of one kind (anchors and
//! doors kept, costs add), Douglas-Peucker each polyline (`simplify_m`), join each edge-less door
//! to the nearest edge within `door_connect_m` (split at the projection + a `Connector`), split
//! edges longer than [`MAX_EDGE_M`], clamp `cost_m` to at least `length_m`, then hash ids, sort and
//! pack.

use std::collections::{BTreeMap, HashMap};

use glam::{DVec3, Vec3};
use sf_coords::WorldPos;
use sf_world::{LinkId, WorldspaceId};

use super::{
    EdgeKind, MAX_EDGE_M, NodeKey, RouteEdge, RouteGraph, RouteNode, RouteNodeId,
    RouteNodeKind, edge_id, node_id, quantise_pos,
};
use crate::types::ClassMask;
use crate::world_graph::LinkSide;

/// Bake-side rules of [`RouteGraphBuilder::build`] (nav.md "Route graph" tunables).
#[derive(Clone, Debug, PartialEq)]
pub struct RouteBuildRules {
    /// Douglas-Peucker tolerance, metres (0 = keep every point).
    pub simplify_m: f64,
    /// Farthest a door joins an edge, metres; beyond, it stays isolated.
    pub door_connect_m: f64,
}

impl Default for RouteBuildRules {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.nav.route.simplify_m in sf_nav_bake, NV12f; Guess)
            simplify_m: 0.25,
            // tunables: const (bound from ai.nav.route.door_connect_m in sf_nav_bake, NV12f; Guess)
            door_connect_m: 60.0,
        }
    }
}

/// What [`RouteGraphBuilder::build`] did (bake log, tests).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RouteBuildStats {
    pub nodes_in: u32,
    pub edges_in: u32,
    /// Input nodes folded into another with the same key.
    pub merged_nodes: u32,
    /// Edges folded into an identical one (same ends, kind, direction and points).
    pub merged_edges: u32,
    /// Edges naming an unknown node, or a zero-length loop.
    pub dropped_edges: u32,
    /// Degree-2 nodes removed by chain collapse.
    pub collapsed_nodes: u32,
    /// Polyline points removed by Douglas-Peucker.
    pub simplified_points: u32,
    /// Edges whose given `cost_m` was below their length.
    pub clamped_costs: u32,
    pub doors_joined: u32,
    /// Doors with no edge and none within `door_connect_m`.
    pub doors_isolated: u32,
    /// Edges split for being longer than [`MAX_EDGE_M`].
    pub split_long: u32,
    pub nodes: u32,
    pub edges: u32,
    pub points: u32,
}

/// Collects nodes and edges for one worldspace, then [`build`](Self::build)s the graph.
#[derive(Clone, Debug)]
pub struct RouteGraphBuilder {
    worldspace: WorldspaceId,
    nodes: Vec<(RouteNodeId, WKind, WorldPos)>,
    edges: Vec<EdgeIn>,
}

#[derive(Clone, Debug)]
struct EdgeIn {
    a: RouteNodeId,
    b: RouteNodeId,
    kind: EdgeKind,
    two_way: bool,
    interior: Vec<WorldPos>,
    cost_m: Option<f32>,
}

/// A node's kind while building: `Junction` and `End` are decided from the final degree.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum WKind {
    Plain,
    Anchor(String),
    Door(LinkId, LinkSide),
}

struct WNode {
    kind: WKind,
    pos: WorldPos,
    /// Incident edge indices (a loop is listed twice).
    edges: Vec<usize>,
}

#[derive(Clone)]
struct WEdge {
    a: RouteNodeId,
    b: RouteNodeId,
    kind: EdgeKind,
    two_way: bool,
    classes: ClassMask,
    /// Node `a`'s position first, node `b`'s last.
    pts: Vec<WorldPos>,
    /// `None` = the polyline's length.
    cost: Option<f64>,
}

impl WEdge {
    fn reverse(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
        self.pts.reverse();
    }

    /// Two-way edges run from the lower node id (a loop: the lower point sequence).
    fn orient(&mut self) {
        if !self.two_way {
            return;
        }
        let flip = match self.a.cmp(&self.b) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => {
                let q: Vec<[i64; 3]> = self.pts.iter().map(|p| quantise_pos(*p)).collect();
                q.iter().rev().lt(q.iter())
            }
        };
        if flip {
            self.reverse();
        }
    }

    fn len(&self) -> f64 {
        poly_len(&self.pts)
    }

    fn cost_or_len(&self) -> f64 {
        self.cost.unwrap_or_else(|| self.len())
    }
}

type EdgeKey = (RouteNodeId, RouteNodeId, EdgeKind, bool, Vec<[i64; 3]>);

impl RouteGraphBuilder {
    pub fn new(worldspace: WorldspaceId) -> Self {
        Self { worldspace, nodes: Vec::new(), edges: Vec::new() }
    }

    /// Adds a node and returns its id. `Junction` and `End` are hints (the build sets them from the
    /// final degree); a `Door` kind is [`add_door`](Self::add_door).
    pub fn add_node(&mut self, pos: WorldPos, kind: RouteNodeKind) -> RouteNodeId {
        let (key, kind) = match kind {
            RouteNodeKind::Door { link, side } => (NodeKey::Door(link, side), WKind::Door(link, side)),
            RouteNodeKind::Anchor(tag) => (NodeKey::Pos(quantise_pos(pos)), WKind::Anchor(tag)),
            RouteNodeKind::Junction | RouteNodeKind::End => (NodeKey::Pos(quantise_pos(pos)), WKind::Plain),
        };
        let id = node_id(&self.worldspace, &key);
        self.nodes.push((id, kind, pos));
        id
    }

    /// Adds the approach point of `link`'s `side` end; its id ignores `pos`, so a retuned
    /// approach keeps it.
    pub fn add_door(&mut self, link: LinkId, side: LinkSide, pos: WorldPos) -> RouteNodeId {
        self.add_node(pos, RouteNodeKind::Door { link, side })
    }

    /// Adds an edge from `a` to `b` through `interior` points. `cost_m: None` = the polyline's
    /// length; one-way edges are walked from `a` to `b` only. Walkable by every class.
    pub fn add_edge(
        &mut self,
        a: RouteNodeId,
        b: RouteNodeId,
        kind: EdgeKind,
        two_way: bool,
        interior: &[WorldPos],
        cost_m: Option<f32>,
    ) {
        let cost_m = cost_m.filter(|c| c.is_finite());
        self.edges.push(EdgeIn { a, b, kind, two_way, interior: interior.to_vec(), cost_m });
    }

    /// Builds the graph (see the module docs for the passes).
    #[allow(clippy::cast_possible_truncation)] // counts and dense indices fit u32
    pub fn build(self, rules: &RouteBuildRules) -> (RouteGraph, RouteBuildStats) {
        let mut stats = RouteBuildStats {
            nodes_in: self.nodes.len() as u32,
            edges_in: self.edges.len() as u32,
            ..RouteBuildStats::default()
        };
        let mut w = Work { worldspace: self.worldspace, nodes: BTreeMap::new(), edges: Vec::new() };
        w.merge_nodes(self.nodes, &mut stats);
        w.merge_edges(self.edges, &mut stats);
        w.collapse(&mut stats);
        w.simplify(rules.simplify_m, &mut stats);
        w.join_doors(rules.door_connect_m, &mut stats);
        w.split_long(&mut stats);
        let graph = w.pack(&mut stats);
        (graph, stats)
    }
}

struct Work {
    worldspace: WorldspaceId,
    nodes: BTreeMap<RouteNodeId, WNode>,
    /// Tombstoned when replaced.
    edges: Vec<Option<WEdge>>,
}

impl Work {
    fn merge_nodes(&mut self, nodes: Vec<(RouteNodeId, WKind, WorldPos)>, stats: &mut RouteBuildStats) {
        for (id, kind, pos) in nodes {
            match self.nodes.get_mut(&id) {
                None => {
                    self.nodes.insert(id, WNode { kind, pos, edges: Vec::new() });
                }
                Some(n) => {
                    stats.merged_nodes += 1;
                    // An anchor beats a plain node, the lower tag another anchor; the lower
                    // position wins (order-independent).
                    let better = match (&n.kind, &kind) {
                        (WKind::Plain, WKind::Anchor(_)) => true,
                        (WKind::Anchor(a), WKind::Anchor(b)) => b < a,
                        _ => false,
                    };
                    if better {
                        n.kind = kind;
                    }
                    if pos_lt(pos, n.pos) {
                        n.pos = pos;
                    }
                }
            }
        }
    }

    fn merge_edges(&mut self, edges: Vec<EdgeIn>, stats: &mut RouteBuildStats) {
        let mut unique: BTreeMap<EdgeKey, WEdge> = BTreeMap::new();
        for e in edges {
            let (Some(na), Some(nb)) = (self.nodes.get(&e.a), self.nodes.get(&e.b)) else {
                stats.dropped_edges += 1;
                continue;
            };
            if e.a == e.b && e.interior.is_empty() {
                stats.dropped_edges += 1;
                continue;
            }
            let mut pts = Vec::with_capacity(e.interior.len() + 2);
            pts.push(na.pos);
            pts.extend_from_slice(&e.interior);
            pts.push(nb.pos);
            let mut we = WEdge {
                a: e.a,
                b: e.b,
                kind: e.kind,
                two_way: e.two_way,
                classes: ClassMask::ALL,
                pts,
                cost: e.cost_m.map(f64::from),
            };
            we.orient();
            let key = (we.a, we.b, we.kind, we.two_way, we.pts.iter().map(|p| quantise_pos(*p)).collect());
            match unique.get_mut(&key) {
                None => {
                    unique.insert(key, we);
                }
                Some(old) => {
                    stats.merged_edges += 1;
                    old.classes = ClassMask(old.classes.0 | we.classes.0);
                    // `None` (= length) is the lowest a clamped cost can be.
                    old.cost = match (old.cost, we.cost) {
                        (Some(x), Some(y)) => Some(x.min(y)),
                        _ => None,
                    };
                }
            }
        }
        for e in unique.into_values() {
            self.push_edge(e);
        }
    }

    fn push_edge(&mut self, e: WEdge) -> usize {
        let i = self.edges.len();
        self.nodes.get_mut(&e.a).expect("edge end").edges.push(i);
        self.nodes.get_mut(&e.b).expect("edge end").edges.push(i);
        self.edges.push(Some(e));
        i
    }

    /// Replaces the first `old` in `node`'s incidence with `new`.
    fn relink(&mut self, node: RouteNodeId, old: usize, new: usize) {
        let list = &mut self.nodes.get_mut(&node).expect("edge end").edges;
        if let Some(slot) = list.iter_mut().find(|e| **e == old) {
            *slot = new;
        }
    }

    /// Adds `e` without touching its ends' incidence (the caller relinks).
    fn push_unlinked(&mut self, e: WEdge) -> usize {
        self.edges.push(Some(e));
        self.edges.len() - 1
    }

    fn collapse(&mut self, stats: &mut RouteBuildStats) {
        let ids: Vec<RouteNodeId> = self.nodes.keys().copied().collect();
        for v in ids {
            let n = &self.nodes[&v];
            if n.kind != WKind::Plain || n.edges.len() != 2 || n.edges[0] == n.edges[1] {
                continue;
            }
            let (i, j) = (n.edges[0], n.edges[1]);
            let (e1, e2) = (self.edges[i].clone().expect("live"), self.edges[j].clone().expect("live"));
            if e1.a == e1.b
                || e2.a == e2.b
                || e1.kind != e2.kind
                || e1.two_way != e2.two_way
                || e1.classes != e2.classes
            {
                continue;
            }
            // `first` ends at v, `second` starts there.
            let (mut first, mut second, fi, si) = if e1.b == v && e2.a == v {
                (e1, e2, i, j)
            } else if e2.b == v && e1.a == v {
                (e2, e1, j, i)
            } else if e1.two_way {
                let (mut f, mut s) = (e1, e2);
                if f.b != v {
                    f.reverse();
                }
                if s.a != v {
                    s.reverse();
                }
                (f, s, i, j)
            } else {
                continue; // one-way edges meeting head to head or tail to tail
            };
            let cost = match (first.cost, second.cost) {
                (None, None) => None,
                _ => Some(first.cost_or_len() + second.cost_or_len()),
            };
            first.pts.extend_from_slice(&second.pts[1..]);
            second.pts.clear();
            let merged = WEdge { b: second.b, pts: std::mem::take(&mut first.pts), cost, ..first };
            let (x, y) = (merged.a, merged.b);
            self.edges[fi] = None;
            self.edges[si] = None;
            let k = self.push_unlinked(merged);
            self.relink(x, fi, k);
            self.relink(y, si, k);
            self.nodes.remove(&v);
            stats.collapsed_nodes += 1;
        }
    }

    #[allow(clippy::cast_possible_truncation)] // point counts fit u32
    fn simplify(&mut self, tol: f64, stats: &mut RouteBuildStats) {
        for e in self.edges.iter_mut().flatten() {
            let before = e.pts.len();
            e.pts = douglas_peucker(&e.pts, tol);
            stats.simplified_points += (before - e.pts.len()) as u32;
        }
    }

    fn join_doors(&mut self, reach: f64, stats: &mut RouteBuildStats) {
        let doors: Vec<RouteNodeId> = self
            .nodes
            .iter()
            .filter(|(_, n)| matches!(n.kind, WKind::Door(..)) && n.edges.is_empty())
            .map(|(id, _)| *id)
            .collect();
        for door in doors {
            let at = self.nodes[&door].pos;
            // Nearest (distance, edge index, segment, t) among non-connector edges.
            let mut best: Option<(f64, usize, usize, f64)> = None;
            for (ei, e) in self.edges.iter().enumerate() {
                let Some(e) = e else { continue };
                if e.kind == EdgeKind::Connector {
                    continue;
                }
                for k in 0..e.pts.len() - 1 {
                    let (d, t) = seg_project(at.delta(e.pts[k]), at.delta(e.pts[k + 1]), DVec3::ZERO);
                    if d <= reach && best.is_none_or(|(bd, ..)| d < bd) {
                        best = Some((d, ei, k, t));
                    }
                }
            }
            let Some((_, ei, k, t)) = best else {
                stats.doors_isolated += 1;
                continue;
            };
            let classes = self.edges[ei].as_ref().expect("live").classes;
            let join = self.split_edge_at(ei, k, t);
            let pts = vec![at, self.nodes[&join].pos];
            let mut c = WEdge { a: door, b: join, kind: EdgeKind::Connector, two_way: true, classes, pts, cost: None };
            c.orient();
            self.push_edge(c);
            stats.doors_joined += 1;
        }
    }

    /// Splits edge `ei` at segment `k`, fraction `t`, and returns the node there (an existing end
    /// when the point is one).
    fn split_edge_at(&mut self, ei: usize, k: usize, t: f64) -> RouteNodeId {
        let e = self.edges[ei].clone().expect("live");
        let last = e.pts.len() - 1;
        let vertex = if t <= 0.0 { Some(k) } else if t >= 1.0 { Some(k + 1) } else { None };
        match vertex {
            Some(0) => return e.a,
            Some(v) if v == last => return e.b,
            _ => {}
        }
        let p = match vertex {
            Some(v) => e.pts[v],
            None => e.pts[k].offset((e.pts[k].delta(e.pts[k + 1]) * t).as_vec3()),
        };
        let id = node_id(&self.worldspace, &NodeKey::Pos(quantise_pos(p)));
        if id == e.a || id == e.b {
            return id;
        }
        let pos = self.nodes.entry(id).or_insert(WNode { kind: WKind::Plain, pos: p, edges: Vec::new() }).pos;
        let (mut head, mut tail) = if let Some(v) = vertex {
            (e.pts[..=v].to_vec(), e.pts[v..].to_vec())
        } else {
            let mut h = e.pts[..=k].to_vec();
            h.push(p);
            let mut t = vec![p];
            t.extend_from_slice(&e.pts[k + 1..]);
            (h, t)
        };
        *head.last_mut().expect("point") = pos;
        tail[0] = pos;
        self.replace_with_pieces(ei, vec![head, tail], &[id]);
        id
    }

    /// Replaces edge `ei` by consecutive `pieces` joined at `inner` nodes; costs split by length.
    fn replace_with_pieces(&mut self, ei: usize, pieces: Vec<Vec<WorldPos>>, inner: &[RouteNodeId]) {
        let e = self.edges[ei].take().expect("live");
        let total = e.len();
        let mut ends = vec![e.a];
        ends.extend_from_slice(inner);
        ends.push(e.b);
        let mut idx = Vec::with_capacity(pieces.len());
        for (i, pts) in pieces.into_iter().enumerate() {
            let cost = e.cost.map(|c| if total > 0.0 { c * poly_len(&pts) / total } else { 0.0 });
            let piece = WEdge { a: ends[i], b: ends[i + 1], pts, cost, ..e.clone() };
            idx.push(self.push_unlinked(piece));
        }
        self.relink(e.a, ei, idx[0]);
        self.relink(e.b, ei, *idx.last().expect("piece"));
        for (i, node) in inner.iter().enumerate() {
            let list = &mut self.nodes.get_mut(node).expect("inner node").edges;
            list.push(idx[i]);
            list.push(idx[i + 1]);
        }
    }

    fn split_long(&mut self, stats: &mut RouteBuildStats) {
        let max = f64::from(MAX_EDGE_M);
        for ei in 0..self.edges.len() {
            let Some(e) = &self.edges[ei] else { continue };
            let len = e.len();
            if len <= max {
                continue;
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a small positive count
            let n = (len / max).ceil() as usize;
            #[allow(clippy::cast_precision_loss)] // a small count
            let piece_len = len / n as f64;
            let mut rest = e.pts.clone();
            let mut pieces = Vec::with_capacity(n);
            let mut inner = Vec::with_capacity(n - 1);
            for _ in 1..n {
                let (head, tail) = cut(&rest, piece_len);
                let p = tail[0];
                let id = node_id(&self.worldspace, &NodeKey::Pos(quantise_pos(p)));
                self.nodes.entry(id).or_insert(WNode { kind: WKind::Plain, pos: p, edges: Vec::new() });
                inner.push(id);
                pieces.push(head);
                rest = tail;
            }
            pieces.push(rest);
            self.replace_with_pieces(ei, pieces, &inner);
            stats.split_long += 1;
        }
    }

    #[allow(clippy::cast_possible_truncation)] // dense indices and counts fit u32
    fn pack(self, stats: &mut RouteBuildStats) -> RouteGraph {
        let index: HashMap<RouteNodeId, u32> =
            self.nodes.keys().enumerate().map(|(i, id)| (*id, i as u32)).collect();
        let nodes: Vec<RouteNode> = self
            .nodes
            .iter()
            .map(|(id, n)| {
                let kind = match &n.kind {
                    WKind::Plain if n.edges.len() <= 1 => RouteNodeKind::End,
                    WKind::Plain => RouteNodeKind::Junction,
                    WKind::Anchor(tag) => RouteNodeKind::Anchor(tag.clone()),
                    WKind::Door(link, side) => RouteNodeKind::Door { link: *link, side: *side },
                };
                RouteNode { id: *id, kind, pos: n.pos }
            })
            .collect();

        let mut packed: Vec<Packed> = Vec::new();
        for mut e in self.edges.into_iter().flatten() {
            e.orient();
            let origin = self.nodes[&e.a].pos;
            let mut offsets: Vec<Vec3> = e.pts.iter().map(|p| origin.delta_f32(*p)).collect();
            offsets[0] = Vec3::ZERO;
            let mut cum = Vec::with_capacity(offsets.len());
            let mut acc = 0.0_f64;
            cum.push(0.0_f32);
            for w in offsets.windows(2) {
                acc += (w[1] - w[0]).as_dvec3().length();
                cum.push(acc as f32);
            }
            let length_m = *cum.last().expect("two points");
            let cost_m = match e.cost {
                None => length_m,
                Some(c) => {
                    if c < acc {
                        stats.clamped_costs += 1;
                    }
                    (c as f32).max(length_m)
                }
            };
            let id = edge_id(e.a, e.b, e.kind, e.two_way, &e.pts);
            let edge = RouteEdge {
                id,
                a: index[&e.a],
                b: index[&e.b],
                kind: e.kind,
                two_way: e.two_way,
                classes: e.classes,
                length_m,
                cost_m,
                points: 0..0,
            };
            packed.push(Packed { edge, offsets, cum });
        }
        packed.sort_by_key(|p| p.edge.id);
        let mut edges: Vec<RouteEdge> = Vec::with_capacity(packed.len());
        let (mut points, mut cum_m) = (Vec::new(), Vec::new());
        for p in packed {
            if let Some(prev) = edges.last_mut().filter(|e| e.id == p.edge.id) {
                stats.merged_edges += 1;
                prev.classes = ClassMask(prev.classes.0 | p.edge.classes.0);
                prev.cost_m = prev.cost_m.min(p.edge.cost_m);
                continue;
            }
            let start = points.len() as u32;
            points.extend(p.offsets);
            cum_m.extend(p.cum);
            edges.push(RouteEdge { points: start..points.len() as u32, ..p.edge });
        }
        stats.nodes = nodes.len() as u32;
        stats.edges = edges.len() as u32;
        stats.points = points.len() as u32;
        RouteGraph { worldspace: self.worldspace, nodes, edges, points, cum_m, derived: super::Derived::default() }
    }
}

/// An edge being packed, with its points.
struct Packed {
    edge: RouteEdge,
    offsets: Vec<Vec3>,
    cum: Vec<f32>,
}

/// Lexicographic position order (absolute metres, total order).
fn pos_lt(a: WorldPos, b: WorldPos) -> bool {
    let (a, b) = (a.to_f64().to_array(), b.to_f64().to_array());
    a.iter().zip(&b).map(|(x, y)| x.total_cmp(y)).find(|o| o.is_ne()) == Some(std::cmp::Ordering::Less)
}

fn poly_len(pts: &[WorldPos]) -> f64 {
    pts.windows(2).map(|w| w[0].delta(w[1]).length()).sum()
}

/// Distance from `q` to segment `a`-`b` and the projection's fraction along it.
pub(super) fn seg_project(a: DVec3, b: DVec3, q: DVec3) -> (f64, f64) {
    let d = b - a;
    let len2 = d.length_squared();
    let t = if len2 > 0.0 { ((q - a).dot(d) / len2).clamp(0.0, 1.0) } else { 0.0 };
    ((a + d * t - q).length(), t)
}

/// Douglas-Peucker on a polyline (ends kept; ties keep the first farthest point).
fn douglas_peucker(pts: &[WorldPos], tol: f64) -> Vec<WorldPos> {
    let n = pts.len();
    if n <= 2 || tol.is_nan() || tol <= 0.0 {
        return pts.to_vec();
    }
    let origin = pts[0];
    let v: Vec<DVec3> = pts.iter().map(|p| origin.delta(*p)).collect();
    let mut keep = vec![false; n];
    keep[0] = true;
    keep[n - 1] = true;
    let mut stack = vec![(0, n - 1)];
    while let Some((s, e)) = stack.pop() {
        if e <= s + 1 {
            continue;
        }
        let (mut far, mut far_d) = (0, -1.0);
        for (i, p) in v.iter().enumerate().take(e).skip(s + 1) {
            let (d, _) = seg_project(v[s], v[e], *p);
            if d > far_d {
                (far, far_d) = (i, d);
            }
        }
        if far_d > tol {
            keep[far] = true;
            stack.push((s, far));
            stack.push((far, e));
        }
    }
    pts.iter().zip(keep).filter(|(_, k)| *k).map(|(p, _)| *p).collect()
}

/// Cuts a polyline `d` metres along: (points up to the cut, points from it).
fn cut(pts: &[WorldPos], d: f64) -> (Vec<WorldPos>, Vec<WorldPos>) {
    let mut acc = 0.0;
    for k in 0..pts.len() - 1 {
        let seg = pts[k].delta(pts[k + 1]);
        let len = seg.length();
        if acc + len >= d && len > 0.0 {
            let frac = (d - acc) / len;
            if frac >= 1.0 {
                return (pts[..=k + 1].to_vec(), pts[k + 1..].to_vec());
            }
            if frac <= 0.0 {
                return (pts[..=k].to_vec(), pts[k..].to_vec());
            }
            let p = pts[k].offset((seg * frac).as_vec3());
            let mut head = pts[..=k].to_vec();
            head.push(p);
            let mut tail = vec![p];
            tail.extend_from_slice(&pts[k + 1..]);
            return (head, tail);
        }
        acc += len;
    }
    let last = pts.len() - 1;
    (pts.to_vec(), pts[last..].to_vec())
}
