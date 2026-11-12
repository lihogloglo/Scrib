//! `nav.coarse`: HPA*'s abstract graph (Botea, Müller, Schaeffer 2004, "Near Optimal
//! Hierarchical Path-Finding") over Detour tiles instead of grid clusters. A tile is the cluster;
//! its connected polygon sets (polygon edge links) split it further; an entrance is a run of
//! portal edges on one tile side within one set. The first costs are straight-line meters (intra-set
//! between every pair, inter-tile between overlapping entrances); NV12 replaces them with Detour
//! path lengths and per-class area costs.

use std::collections::BTreeMap;

use glam::{Vec2, Vec3};
use sf_nav::{
    BlobPoly, BlobTile, BuildConfig, CoarseEdge, CoarseGraph, CoarseNode, CoarseNodeKind, NavLinks,
    PolyEdge, TileKey, TileSide,
};

use crate::{NavRegion, local};

/// Portal edges closer than this along a side are one entrance (meters).
const JOIN_EPS: f32 = 0.01;

/// An entrance under construction: its span along the side and the points it covers.
struct Entrance {
    side: TileSide,
    component: u32,
    lo: f32,
    hi: f32,
    y_sum: f32,
    y_n: f32,
}

/// The coordinate along a side (z on ±x sides, x on ±z sides).
fn along(side: TileSide, p: Vec3) -> f32 {
    match side {
        TileSide::PosX | TileSide::NegX => p.z,
        TileSide::PosZ | TileSide::NegZ => p.x,
    }
}

/// Union-find root with path halving.
fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

/// Connected polygon sets, numbered in order of their lowest polygon.
fn components(tile: &BlobTile) -> Vec<u32> {
    let n = tile.polys.len();
    let mut parent: Vec<usize> = (0..n).collect();
    for (i, p) in tile.polys.iter().enumerate() {
        for e in &p.edges {
            if let PolyEdge::Poly(j) = *e
                && (j as usize) < n
            {
                let (a, b) = (find(&mut parent, i), find(&mut parent, j as usize));
                parent[a.max(b)] = a.min(b);
            }
        }
    }
    let mut ids: BTreeMap<usize, u32> = BTreeMap::new();
    (0..n)
        .map(|i| {
            let root = find(&mut parent, i);
            let next = u32::try_from(ids.len()).expect("Detour caps polygons per tile at 2^20");
            *ids.entry(root).or_insert(next)
        })
        .collect()
}

/// Portal edges per (component, side): (lo, hi, sum of both end heights).
type Spans = BTreeMap<(u32, TileSide), Vec<(f32, f32, f32)>>;

fn entrances(tile: &BlobTile, comp: &[u32]) -> Vec<Entrance> {
    // (component, side) -> spans (lo, hi, y at each end).
    let mut spans: Spans = BTreeMap::new();
    for (i, p) in tile.polys.iter().enumerate() {
        let pts: Vec<Vec3> = p.points(tile).collect();
        for (e, edge) in p.edges.iter().enumerate() {
            if let PolyEdge::Portal(side) = *edge {
                let (a, b) = (pts[e], pts[(e + 1) % pts.len()]);
                let (u, v) = (along(side, a), along(side, b));
                spans
                    .entry((comp[i], side))
                    .or_default()
                    .push((u.min(v), u.max(v), a.y + b.y));
            }
        }
    }
    let mut out = Vec::new();
    for ((component, side), mut list) in spans {
        list.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
        let mut cur: Option<Entrance> = None;
        for (lo, hi, y2) in list {
            match &mut cur {
                Some(c) if lo <= c.hi + JOIN_EPS => {
                    c.hi = c.hi.max(hi);
                    c.y_sum += y2;
                    c.y_n += 2.0;
                }
                _ => {
                    out.extend(cur.take());
                    cur = Some(Entrance {
                        side,
                        component,
                        lo,
                        hi,
                        y_sum: y2,
                        y_n: 2.0,
                    });
                }
            }
        }
        out.extend(cur);
    }
    out
}

/// Whether `p` lies on the polygon's XZ footprint (convex, either winding).
fn covers(poly: &BlobPoly, tile: &BlobTile, p: Vec3) -> bool {
    let pts: Vec<Vec2> = poly.points(tile).map(|v| Vec2::new(v.x, v.z)).collect();
    let at = Vec2::new(p.x, p.z);
    let (mut pos, mut neg) = (false, false);
    for i in 0..pts.len() {
        let side = (pts[(i + 1) % pts.len()] - pts[i]).perp_dot(at - pts[i]);
        pos |= side > 1e-5;
        neg |= side < -1e-5;
    }
    pts.len() >= 3 && !(pos && neg)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "tile indices are small"
)]
pub(crate) fn build(
    cfg: &BuildConfig,
    regions: &[NavRegion],
    tiles: &[(TileKey, BlobTile)],
    links: &NavLinks,
) -> CoarseGraph {
    let size = cfg.tile_size();
    let mut nodes: Vec<CoarseNode> = Vec::new();
    // Per node: its entrance span, for matching across tiles.
    let mut spans: Vec<Option<(f32, f32)>> = Vec::new();
    let mut index: BTreeMap<(u16, i32, i32), (&BlobTile, Vec<u32>)> = BTreeMap::new();
    for (key, tile) in tiles {
        let comp = components(tile);
        let (x0, z0) = (key.x as f32 * size, key.z as f32 * size);
        for e in entrances(tile, &comp) {
            let mid = f32::midpoint(e.lo, e.hi);
            let y = e.y_sum / e.y_n;
            let at = match e.side {
                TileSide::PosX => Vec3::new(x0 + size, y, mid),
                TileSide::NegX => Vec3::new(x0, y, mid),
                TileSide::PosZ => Vec3::new(mid, y, z0 + size),
                TileSide::NegZ => Vec3::new(mid, y, z0),
            };
            nodes.push(CoarseNode {
                region: key.region,
                tile: (key.x, key.z),
                component: e.component,
                kind: CoarseNodeKind::Entrance(e.side),
                at,
            });
            spans.push(Some((e.lo, e.hi)));
        }
        index.insert((key.region, key.x, key.z), (tile, comp));
    }

    // Door approach points: on the polygon under them (closest in height within the agent's).
    let mut unplaced = Vec::new();
    for d in &links.doors {
        let placed = regions.iter().find_map(|r| {
            let p = local(r, d.at.pos);
            let (tx, tz) = ((p.x / size).floor() as i32, (p.z / size).floor() as i32);
            let (tile, comp) = index.get(&(r.region, tx, tz))?;
            let best = tile
                .polys
                .iter()
                .enumerate()
                .filter(|(_, poly)| covers(poly, tile, p))
                .map(|(i, poly)| (i, (poly.centroid(tile).y - p.y).abs()))
                .filter(|&(_, dy)| dy <= cfg.class.height)
                .min_by(|a, b| a.1.total_cmp(&b.1))?;
            Some(CoarseNode {
                region: r.region,
                tile: (tx, tz),
                component: comp[best.0],
                kind: CoarseNodeKind::Door(d.door),
                at: p,
            })
        });
        match placed {
            Some(n) => {
                nodes.push(n);
                spans.push(None);
            }
            None => unplaced.push(d.door),
        }
    }

    let edges = edges(cfg, &nodes, &spans);
    CoarseGraph {
        class: cfg.class.id,
        nodes,
        edges,
        unplaced,
    }
}

/// Every pair of nodes on one connected set of one tile, plus +x / +z entrances to the
/// neighbour's opposite entrances they overlap. `spans[i]` is node `i`'s entrance span.
fn edges(cfg: &BuildConfig, nodes: &[CoarseNode], spans: &[Option<(f32, f32)>]) -> Vec<CoarseEdge> {
    let mut edges = Vec::new();
    let edge = |a: usize, b: usize, nodes: &[CoarseNode]| CoarseEdge {
        a: u32::try_from(a.min(b)).expect("node count fits u32"),
        b: u32::try_from(a.max(b)).expect("node count fits u32"),
        cost: nodes[a].at.distance(nodes[b].at),
    };
    // Within a tile: every pair on one connected set.
    let mut groups: BTreeMap<(u16, (i32, i32), u32), Vec<usize>> = BTreeMap::new();
    for (i, n) in nodes.iter().enumerate() {
        groups
            .entry((n.region, n.tile, n.component))
            .or_default()
            .push(i);
    }
    for members in groups.values() {
        for (k, &a) in members.iter().enumerate() {
            for &b in &members[k + 1..] {
                edges.push(edge(a, b, nodes));
            }
        }
    }
    // Across tiles: a +x / +z entrance to the neighbour's opposite entrance it overlaps.
    let mut by_side: BTreeMap<(u16, (i32, i32), TileSide), Vec<usize>> = BTreeMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if let CoarseNodeKind::Entrance(s) = n.kind {
            by_side.entry((n.region, n.tile, s)).or_default().push(i);
        }
    }
    for (&(region, (x, z), side), list) in &by_side {
        if !matches!(side, TileSide::PosX | TileSide::PosZ) {
            continue;
        }
        let Some(other) = by_side.get(&(region, side.step(x, z), side.opposite())) else {
            continue;
        };
        for &a in list {
            for &b in other {
                let ((alo, ahi), (blo, bhi)) =
                    (spans[a].expect("entrance"), spans[b].expect("entrance"));
                let overlap = alo < bhi + JOIN_EPS && blo < ahi + JOIN_EPS;
                if overlap && (nodes[a].at.y - nodes[b].at.y).abs() <= cfg.class.height {
                    edges.push(edge(a, b, nodes));
                }
            }
        }
    }
    edges.sort_by_key(|e| (e.a, e.b));
    edges
}
