//! [`RouteGraph::route`], [`RouteGraph::position_at`] and [`RouteGraph::nearest_point`] (nav.md
//! "Route graph" Queries). Inline, no tiles; costs sum in f64 and ties break by index (= id).

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use glam::{DVec3, Vec2};
use sf_coords::WorldPos;
use sf_world::Location;

use super::build::seg_project;
use super::{
    RouteEdge, RouteEnd, RouteError, RouteGraph, RouteKind, RouteLeg, RoutePath, RoutePoint,
    RoutePose, TravelRules, bucket,
};

/// How the search reached a node (or the virtual goal).
#[derive(Clone, Copy)]
enum Pred {
    None,
    /// From the start point along the start edge (`forward` = towards its `b`).
    Start { forward: bool },
    /// From node `from` along edge `edge`.
    Via { from: u32, edge: u32, forward: bool },
    /// Goal only: from node `from` along the goal edge.
    Goal { from: u32, forward: bool },
    /// Goal only: along the start edge, which is also the goal edge.
    SameEdge,
}

impl RouteGraph {
    /// The cheapest walk from `from` to `to` (see the module docs and nav.md "Route graph").
    pub fn route(&self, from: &Location, to: &Location, r: &TravelRules) -> Result<RoutePath, RouteError> {
        if from.worldspace != self.worldspace || to.worldspace != self.worldspace {
            return Err(RouteError::WrongSpace);
        }
        let mul = r.offroad_cost_mul.max(1.0);
        let straight = from.pos.delta(to.pos).length();
        let direct = (straight <= r.join_max_m).then_some(straight * mul);
        let joins = (self.nearest(from, r, r.join_max_m), self.nearest(to, r, r.join_max_m));
        let ((si, sp), (gi, gp)) = match joins {
            (Some(s), Some(g)) => (s, g),
            (s, _) => {
                return match direct {
                    Some(cost) => Ok(self.direct(from, to, r, cost)),
                    None if s.is_none() => Err(RouteError::NoJoin(RouteEnd::From)),
                    None => Err(RouteError::NoJoin(RouteEnd::To)),
                };
            }
        };
        match (self.search(&sp, si, &gp, gi, r, mul, direct), direct) {
            (Ok(path), Some(cost)) if cost <= path.cost_m => Ok(self.direct(from, to, r, cost)),
            (Err(_), Some(cost)) => Ok(self.direct(from, to, r, cost)),
            (Ok(mut path), _) => {
                if sp.dist_m > 0.0 {
                    path.legs.insert(0, RouteLeg::Straight { a: from.pos, b: sp.pos, cost_m: sp.dist_m * mul });
                }
                if gp.dist_m > 0.0 {
                    path.legs.push(RouteLeg::Straight { a: gp.pos, b: to.pos, cost_m: gp.dist_m * mul });
                }
                if path.legs.is_empty() {
                    path.legs.push(RouteLeg::Straight { a: sp.pos, b: sp.pos, cost_m: 0.0 });
                }
                Ok(path)
            }
            (Err(e), None) => Err(e),
        }
    }

    fn direct(&self, from: &Location, to: &Location, r: &TravelRules, cost_m: f64) -> RoutePath {
        RoutePath {
            worldspace: self.worldspace.clone(),
            speed_mps: r.speed_mps,
            kind: RouteKind::Direct,
            legs: vec![RouteLeg::Straight { a: from.pos, b: to.pos, cost_m }],
            cost_m,
        }
    }

    /// A* between two edge points; the path's `cost_m` includes both straight joins, its legs
    /// don't yet (the caller adds them).
    #[allow(clippy::too_many_arguments, clippy::cast_possible_truncation)] // dense indices fit u32
    fn search(
        &self,
        sp: &RoutePoint,
        si: usize,
        gp: &RoutePoint,
        gi: usize,
        r: &TravelRules,
        mul: f64,
        direct: Option<f64>,
    ) -> Result<RoutePath, RouteError> {
        let adj = self.adjacency();
        let n = self.nodes.len();
        let goal = n;
        let (es, eg) = (&self.edges[si], &self.edges[gi]);
        let (s, t) = (f64::from(sp.along_m), f64::from(gp.along_m));
        let (s_len, g_len) = (f64::from(es.length_m), f64::from(eg.length_m));
        let start_leg = sp.dist_m * mul;
        let goal_leg = gp.dist_m * mul;
        let mut g = vec![f64::INFINITY; n + 1];
        let mut pred = vec![Pred::None; n + 1];
        let mut open = BinaryHeap::new();
        let h = |i: usize| self.nodes[i].pos.delta(gp.pos).length();
        let relax = |node: usize, cost: f64, p: Pred, g: &mut [f64], pred: &mut [Pred], open: &mut BinaryHeap<Open>| {
            if cost < g[node] {
                g[node] = cost;
                pred[node] = p;
                let f = if node == goal { cost } else { cost + h(node) };
                open.push(Open { f, g: cost, node: node as u32 });
            }
        };
        relax(es.b as usize, start_leg + es.partial(s_len - s), Pred::Start { forward: true }, &mut g, &mut pred, &mut open);
        if es.two_way {
            relax(es.a as usize, start_leg + es.partial(s), Pred::Start { forward: false }, &mut g, &mut pred, &mut open);
        }
        if si == gi {
            let piece = if t >= s { Some(t - s) } else { es.two_way.then_some(s - t) };
            if let Some(piece) = piece {
                relax(goal, start_leg + es.partial(piece) + goal_leg, Pred::SameEdge, &mut g, &mut pred, &mut open);
            }
        }
        let mut expansions = 0_u32;
        while let Some(Open { f, g: g_here, node }) = open.pop() {
            let node = node as usize;
            if g_here > g[node] {
                continue; // stale entry
            }
            if node == goal {
                return Ok(self.unwind(&pred, g[goal], si, s, gi, t, r));
            }
            if direct.is_some_and(|d| f >= d) {
                break; // the graph can't beat the straight walk
            }
            expansions += 1;
            if expansions > r.max_expansions {
                return Err(RouteError::Budget);
            }
            if node == eg.a as usize {
                let p = Pred::Goal { from: node as u32, forward: true };
                relax(goal, g_here + eg.partial(t) + goal_leg, p, &mut g, &mut pred, &mut open);
            }
            if eg.two_way && node == eg.b as usize {
                let p = Pred::Goal { from: node as u32, forward: false };
                relax(goal, g_here + eg.partial(g_len - t) + goal_leg, p, &mut g, &mut pred, &mut open);
            }
            for arc in adj.of(node) {
                let e = &self.edges[arc.edge as usize];
                if !e.allowed(r) {
                    continue;
                }
                let p = Pred::Via { from: node as u32, edge: arc.edge, forward: arc.forward };
                relax(arc.to as usize, g_here + f64::from(e.cost_m), p, &mut g, &mut pred, &mut open);
            }
        }
        Err(RouteError::Unreachable)
    }

    #[allow(clippy::too_many_arguments)]
    fn unwind(&self, pred: &[Pred], cost_m: f64, si: usize, s: f64, gi: usize, t: f64, r: &TravelRules) -> RoutePath {
        let goal = self.nodes.len();
        let (es, eg) = (&self.edges[si], &self.edges[gi]);
        let mut rev: Vec<RouteLeg> = Vec::new();
        let push = |legs: &mut Vec<RouteLeg>, ei: usize, from_m: f64, to_m: f64, cost: f64| {
            if from_m.total_cmp(&to_m).is_ne() {
                let e = &self.edges[ei];
                #[allow(clippy::cast_possible_truncation)] // along-edge metres are f32 by design
                let (from_m, to_m) = (from_m as f32, to_m as f32);
                let a = self.edge_pos(e, f64::from(from_m));
                legs.push(RouteLeg::Edge { edge: e.id, from_m, to_m, a, cost_m: cost });
            }
        };
        match pred[goal] {
            Pred::SameEdge => {
                push(&mut rev, si, s, t, es.partial((t - s).abs()));
            }
            Pred::Goal { from, forward } => {
                let g_len = f64::from(eg.length_m);
                if forward {
                    push(&mut rev, gi, 0.0, t, eg.partial(t));
                } else {
                    push(&mut rev, gi, g_len, t, eg.partial(g_len - t));
                }
                let mut node = from as usize;
                loop {
                    match pred[node] {
                        Pred::Via { from, edge, forward } => {
                            let e = &self.edges[edge as usize];
                            let len = f64::from(e.length_m);
                            let (a, b) = if forward { (0.0, len) } else { (len, 0.0) };
                            // A full edge is a leg even at zero length (its cost may not be).
                            let a_pos = self.edge_pos(e, a);
                            #[allow(clippy::cast_possible_truncation)] // f32 metres by design
                            rev.push(RouteLeg::Edge {
                                edge: e.id,
                                from_m: a as f32,
                                to_m: b as f32,
                                a: a_pos,
                                cost_m: f64::from(e.cost_m),
                            });
                            node = from as usize;
                        }
                        Pred::Start { forward } => {
                            let s_len = f64::from(es.length_m);
                            if forward {
                                push(&mut rev, si, s, s_len, es.partial(s_len - s));
                            } else {
                                push(&mut rev, si, s, 0.0, es.partial(s));
                            }
                            break;
                        }
                        _ => unreachable!("every reached node has a predecessor"),
                    }
                }
            }
            _ => unreachable!("the goal was reached"),
        }
        rev.reverse();
        RoutePath { worldspace: self.worldspace.clone(), speed_mps: r.speed_mps, kind: RouteKind::Graph, legs: rev, cost_m }
    }

    /// Where a traveller on `path` is `t_s` seconds after leaving (clamped to the path).
    pub fn position_at(&self, path: &RoutePath, t_s: f64) -> RoutePose {
        let duration = path.duration_s();
        let last = path.legs.len().saturating_sub(1);
        let (leg, u, done) = if t_s >= duration || path.legs.len() <= 1 && path.cost_m <= 0.0 {
            (last, 1.0, true)
        } else {
            let d = (t_s.max(0.0) * path.speed_mps).min(path.cost_m);
            let mut acc = 0.0;
            let mut found = (last, 1.0);
            for (i, l) in path.legs.iter().enumerate() {
                let c = l.cost_m();
                if d < acc + c {
                    found = (i, (d - acc) / c);
                    break;
                }
                acc += c;
            }
            (found.0, found.1, false)
        };
        let (pos, heading) = self.leg_pose(path, leg, u);
        RoutePose {
            loc: Location::new(path.worldspace.clone(), pos),
            heading: heading.unwrap_or(Vec2::NEG_Y),
            leg: u16::try_from(leg).unwrap_or(u16::MAX),
            done,
        }
    }

    /// Position at fraction `u` of leg `i`, and the unit XZ heading there (`None` when the leg
    /// and every leg before it have no XZ extent).
    fn leg_pose(&self, path: &RoutePath, i: usize, u: f64) -> (WorldPos, Option<Vec2>) {
        let Some(leg) = path.legs.get(i) else { return (WorldPos::ORIGIN, None) };
        let (pos, heading) = match *leg {
            RouteLeg::Straight { a, b, .. } => {
                let d = a.delta(b);
                let pos = if u >= 1.0 {
                    b
                } else if u <= 0.0 {
                    a
                } else {
                    a.offset((d * u).as_vec3())
                };
                (pos, unit_xz(d))
            }
            RouteLeg::Edge { edge, from_m, to_m, .. } => {
                let e = self.edge_index(edge).map(|ei| &self.edges[ei]);
                let Some(e) = e else { return (WorldPos::ORIGIN, None) };
                let (from_m, to_m) = (f64::from(from_m), f64::from(to_m));
                let m = if u >= 1.0 { to_m } else { from_m + (to_m - from_m) * u };
                let forward = to_m >= from_m;
                (self.edge_pos(e, m), self.edge_heading(e, m, forward))
            }
        };
        match heading {
            Some(h) => (pos, Some(h)),
            None if i > 0 => (pos, self.leg_pose(path, i - 1, 1.0).1),
            None => (pos, None),
        }
    }

    /// The segment containing `m` metres along `e`: at a vertex, the one being entered.
    fn segment_at(&self, e: &RouteEdge, m: f64, forward: bool) -> usize {
        let cum = self.edge_cum(e);
        let k = if forward {
            cum.partition_point(|&c| f64::from(c) <= m)
        } else {
            cum.partition_point(|&c| f64::from(c) < m)
        };
        k.saturating_sub(1).min(cum.len() - 2)
    }

    /// The point `m` metres along `e`.
    fn edge_pos(&self, e: &RouteEdge, m: f64) -> WorldPos {
        let (pts, cum) = (self.edge_points(e), self.edge_cum(e));
        let k = self.segment_at(e, m, true);
        let (c0, c1) = (f64::from(cum[k]), f64::from(cum[k + 1]));
        let off = if m <= c0 {
            pts[k].as_dvec3()
        } else if m >= c1 {
            pts[k + 1].as_dvec3()
        } else {
            let frac = (m - c0) / (c1 - c0);
            pts[k].as_dvec3() + (pts[k + 1].as_dvec3() - pts[k].as_dvec3()) * frac
        };
        self.nodes[e.a as usize].pos.offset(off.as_vec3())
    }

    /// Direction of travel at `m` along `e`: the current segment's, else the previous one's.
    fn edge_heading(&self, e: &RouteEdge, m: f64, forward: bool) -> Option<Vec2> {
        let pts = self.edge_points(e);
        let k = self.segment_at(e, m, forward);
        let seg = |k: usize| {
            let d = (pts[k + 1] - pts[k]).as_dvec3();
            unit_xz(if forward { d } else { -d })
        };
        if forward {
            (0..=k).rev().find_map(seg)
        } else {
            (k..pts.len() - 1).find_map(seg)
        }
    }

    /// The nearest point on an edge `r` allows, within `max_m` (ties: distance, edge id, along).
    pub fn nearest_point(&self, at: &Location, r: &TravelRules, max_m: f64) -> Option<RoutePoint> {
        self.nearest(at, r, max_m).map(|(_, p)| p)
    }

    /// [`nearest_point`](Self::nearest_point) with the edge's index.
    pub(crate) fn nearest(&self, at: &Location, r: &TravelRules, max_m: f64) -> Option<(usize, RoutePoint)> {
        if at.worldspace != self.worldspace || max_m.is_nan() || max_m < 0.0 {
            return None;
        }
        let b = self.buckets();
        let (min_x, min_z, max_x, max_z) = b.bounds?;
        let abs = at.pos.to_f64();
        let cell = b.cell;
        let (cx, cz) = (bucket(abs.x, cell), bucket(abs.z, cell));
        #[allow(clippy::cast_precision_loss)] // bucket indices are small
        let border = {
            let (x0, z0) = (cx as f64 * cell, cz as f64 * cell);
            (abs.x - x0).min(x0 + cell - abs.x).min(abs.z - z0).min(z0 + cell - abs.z).max(0.0)
        };
        let mut best: Option<Cand> = None;
        let mut ring = 0_i64;
        loop {
            #[allow(clippy::cast_precision_loss)] // ring counts are small
            let inner = if ring == 0 { 0.0 } else { (ring - 1) as f64 * cell + border };
            if inner > max_m || best.as_ref().is_some_and(|c| inner > c.dist) {
                break;
            }
            let mut visit = |x: i64, z: i64| {
                for &p in b.map.get(&(x, z)).into_iter().flatten() {
                    let ei = b.point_edge[p as usize] as usize;
                    if !self.edges[ei].allowed(r) {
                        continue;
                    }
                    let c = self.project(ei, p, at.pos);
                    if c.dist <= max_m && best.as_ref().is_none_or(|bc| c.better(bc)) {
                        best = Some(c);
                    }
                }
            };
            if ring == 0 {
                visit(cx, cz);
            } else {
                for dx in -ring..=ring {
                    visit(cx + dx, cz - ring);
                    visit(cx + dx, cz + ring);
                }
                for dz in -ring + 1..ring {
                    visit(cx - ring, cz + dz);
                    visit(cx + ring, cz + dz);
                }
            }
            if cx - ring <= min_x && cx + ring >= max_x && cz - ring <= min_z && cz + ring >= max_z {
                break; // every bucket seen
            }
            ring += 1;
        }
        best.map(|c| (c.edge, self.point(c)))
    }

    /// [`nearest`](Self::nearest) by scanning every segment (the index's reference).
    #[cfg(test)]
    pub(crate) fn nearest_linear(&self, at: &Location, r: &TravelRules, max_m: f64) -> Option<RoutePoint> {
        let mut best: Option<Cand> = None;
        for (ei, e) in self.edges.iter().enumerate() {
            if !e.allowed(r) {
                continue;
            }
            for p in e.points.start..e.points.end - 1 {
                let c = self.project(ei, p, at.pos);
                if c.dist <= max_m && best.as_ref().is_none_or(|bc| c.better(bc)) {
                    best = Some(c);
                }
            }
        }
        best.map(|c| self.point(c))
    }

    /// Projects `at` on segment `p` → `p + 1` of edge `ei`.
    fn project(&self, ei: usize, p: u32, at: WorldPos) -> Cand {
        let e = &self.edges[ei];
        let q = self.nodes[e.a as usize].pos.delta(at);
        let (p0, p1) = (self.points[p as usize].as_dvec3(), self.points[p as usize + 1].as_dvec3());
        let (dist, t) = seg_project(p0, p1, q);
        let (c0, c1) = (self.cum_m[p as usize], self.cum_m[p as usize + 1]);
        #[allow(clippy::cast_possible_truncation)] // along-edge metres are f32 by design
        let along = (f64::from(c0) + t * f64::from(c1 - c0)) as f32;
        Cand { dist, edge: ei, along, offset: p0 + (p1 - p0) * t }
    }

    fn point(&self, c: Cand) -> RoutePoint {
        let e = &self.edges[c.edge];
        RoutePoint {
            edge: e.id,
            along_m: c.along,
            pos: self.nodes[e.a as usize].pos.offset(c.offset.as_vec3()),
            dist_m: c.dist,
        }
    }
}

/// A nearest-point candidate.
struct Cand {
    dist: f64,
    edge: usize,
    along: f32,
    offset: DVec3,
}

impl Cand {
    fn better(&self, o: &Self) -> bool {
        self.dist
            .total_cmp(&o.dist)
            .then(self.edge.cmp(&o.edge))
            .then(self.along.total_cmp(&o.along))
            == Ordering::Less
    }
}

#[allow(clippy::cast_possible_truncation)] // a unit vector fits f32
fn unit_xz(d: DVec3) -> Option<Vec2> {
    let v = glam::DVec2::new(d.x, d.z);
    let len = v.length();
    (len > 0.0).then(|| (v / len).as_vec2())
}

/// An open-list entry: min-heap on `f`, ties to the lower node index (= lower id).
#[derive(Clone, Copy)]
struct Open {
    f: f64,
    g: f64,
    node: u32,
}

impl PartialEq for Open {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Open {}

impl PartialOrd for Open {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Open {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed: BinaryHeap is a max-heap.
        other.f.total_cmp(&self.f).then_with(|| other.node.cmp(&self.node))
    }
}
