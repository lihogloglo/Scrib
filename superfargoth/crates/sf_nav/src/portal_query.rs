//! The shared through-door query (nav.md "The shared through-door query", NV11c): one Bevy-free
//! API for everything that crosses spaces (AI sight and hearing, audio, crime witnesses, net
//! interest, quest markers).
//!
//! - [`map`] / [`map_direction`]: a point or direction carried through a link's rigid portal
//!   transform ([`DoorLink::cross`]).
//! - [`WorldGraph::path_distance`]: how far a sound travels. Same space: the straight line. Across
//!   spaces: the cheapest chain of [`WorldGraph`] exits in metres, straight legs through each
//!   opening's centre (the link end's portal frame), no traverse time (`OpenMW`'s `heard` distance
//!   through doors). Every link carries sound, seamless or fade-only, open or shut; links not
//!   [`DoorState::Open`] are counted in [`PortalPath::closed_doors`] and may weigh extra metres in
//!   the choice of way ([`PathRules::closed_door_m`], hearing's `closed_door_m`). Hop-bounded
//!   Dijkstra: labels `(exit, hops)`, a label is dropped when its exit was already settled with no
//!   more hops (settled labels cost no more), so a cheaper way with too many hops never hides a
//!   valid one. One-way links carry sound only from their `a` end, like routes.
//! - [`WorldGraph::line_of_sight`]: same space, one [`LosProvider`] ray. Across spaces, only
//!   through a seamless link that joins the two spaces directly and is open
//!   ([`DoorState::openness`] above 0): the segment from the viewer to the target seen through
//!   the portal (the target mapped back into the viewer's space) must cross the door plane
//!   inside the opening (half [`LinkEnd::extent`] each way on the frame's X and Y; a zero
//!   component is a point opening on that axis), then two rays: viewer to that crossing point,
//!   and the crossing point mapped to the target. Fade-only or shut links, a crossing outside
//!   the opening, or no direct link give [`Los::BlockedByLink`]; sight crosses one link at most.
//!
//! Rays go through the host's [`LosProvider`] (batched physics in the adapter): no physics here.
//! Rays that cross a link end on its door plane, so the provider should not count the door
//! leaf's own collider as a hit.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use glam::{Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_world::{DoorLink, LinkEnd, LinkId, Location};
use smallvec::SmallVec;

use crate::world_graph::{DoorState, LinkKind, LinkSide, WorldGraph};

/// The links of a [`PortalPath`], in crossing order.
pub type PortalLinks = SmallVec<[LinkId; 4]>;

/// The end of `link` left through from `from`, and the end it leads to.
pub fn ends(link: &DoorLink, from: LinkSide) -> (&LinkEnd, &LinkEnd) {
    match from {
        LinkSide::A => (&link.a, &link.b),
        LinkSide::B => (&link.b, &link.a),
    }
}

/// The other side of a link.
pub fn other(side: LinkSide) -> LinkSide {
    match side {
        LinkSide::A => LinkSide::B,
        LinkSide::B => LinkSide::A,
    }
}

/// `loc` (in the worldspace of `link`'s `from` end) carried through the portal into the other
/// end's worldspace: a point in front of `from` lands behind the other end, where it is seen
/// from there through the opening.
pub fn map(loc: &Location, link: &DoorLink, from: LinkSide) -> Location {
    let (f, t) = ends(link, from);
    let pose = DoorLink::cross(f, t, &WorldTransform::from_pos(loc.pos));
    Location::new(t.worldspace.clone(), pose.pos)
}

/// [`map`] for a direction (velocity, view direction).
pub fn map_direction(dir: Vec3, link: &DoorLink, from: LinkSide) -> Vec3 {
    let (f, t) = ends(link, from);
    DoorLink::cross_direction(f, t, dir)
}

/// The caller's side of [`WorldGraph::path_distance`].
#[derive(Clone, Copy)]
pub struct PathRules<'a> {
    /// Most links the path may cross.
    pub max_hops: usize,
    /// Extra metres each link not [`DoorState::Open`] weighs when choosing the way (0: pure
    /// distance); never part of [`PortalPath::metres`].
    pub closed_door_m: f32,
    /// The host's current state of each link.
    pub door_state: &'a dyn Fn(LinkId) -> DoorState,
}

/// How a sound gets from one place to another.
#[derive(Clone, Debug, PartialEq)]
pub struct PortalPath {
    /// Straight legs through each opening's centre, meters.
    pub metres: f32,
    pub links: PortalLinks,
    /// Links on the way that are not [`DoorState::Open`] (closed or locked).
    pub closed_doors: u32,
}

/// What a [`WorldGraph::line_of_sight`] found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Los {
    Clear,
    /// A [`LosProvider`] ray hit something.
    Blocked,
    /// No open seamless link lets this line through (none joins the spaces, fade-only, shut, or
    /// the line misses the opening).
    BlockedByLink,
}

/// The host's ray test, one worldspace at a time (physics lives in the adapter).
pub trait LosProvider {
    /// Whether the segment `from` -> `to` is unobstructed; both are in the same worldspace.
    fn ray(&self, from: &Location, to: &Location) -> bool;
}

/// A search label: reached `node` after `hops` links.
#[derive(Clone, Copy)]
struct Label {
    node: usize,
    hops: usize,
    /// Metres plus closed-door weight: what the search minimises.
    cost: f64,
    metres: f64,
    closed: u32,
    parent: usize,
}

/// Open-list entry: min-heap on cost, ties to the lower node, then fewer hops.
#[derive(Clone, Copy)]
struct Pending {
    cost: f64,
    node: usize,
    hops: usize,
    label: usize,
}

impl PartialEq for Pending {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Pending {}

impl PartialOrd for Pending {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pending {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed: BinaryHeap is a max-heap.
        other
            .cost
            .total_cmp(&self.cost)
            .then_with(|| other.node.cmp(&self.node))
            .then_with(|| other.hops.cmp(&self.hops))
            .then_with(|| other.label.cmp(&self.label))
    }
}

fn metres(a: WorldPos, b: WorldPos) -> f64 {
    a.delta(b).length()
}

#[allow(clippy::cast_possible_truncation, reason = "metres within a world fit f32")]
fn to_f32(m: f64) -> f32 {
    m as f32
}

impl WorldGraph {
    /// The opening centres of exit `i`: where it is entered and where it comes out.
    fn openings(&self, i: usize) -> (&LinkEnd, &LinkEnd) {
        let exit = &self.exits[i];
        ends(&self.index[&exit.link].link, exit.side)
    }

    /// How far a sound at `a` travels to `b` (module docs), or `None` when no chain of at most
    /// `rules.max_hops` links joins their worldspaces.
    pub fn path_distance(&self, a: &Location, b: &Location, rules: &PathRules<'_>) -> Option<PortalPath> {
        if a.worldspace == b.worldspace {
            return Some(PortalPath {
                metres: to_f32(metres(a.pos, b.pos)),
                links: PortalLinks::new(),
                closed_doors: 0,
            });
        }
        // NaN or negative weighs nothing.
        let weight = f64::from(rules.closed_door_m.max(0.0));
        // Nodes: 0..n = "came out of exit i", n = start, n + 1 = goal.
        let n = self.exits.len();
        let (start, goal) = (n, n + 1);
        // Fewest hops of a settled label per exit (`usize::MAX`: none yet).
        let mut settled = vec![usize::MAX; n];
        // Per exit: 0 = not asked yet, 1 = open, 2 = shut.
        let mut shut = vec![0_u8; n];
        let mut labels =
            vec![Label { node: start, hops: 0, cost: 0.0, metres: 0.0, closed: 0, parent: usize::MAX }];
        let mut open = BinaryHeap::from([Pending { cost: 0.0, node: start, hops: 0, label: 0 }]);

        while let Some(Pending { label: li, .. }) = open.pop() {
            let here = labels[li];
            if here.node == goal {
                return Some(self.unwind_path(&labels, li));
            }
            let (ws, at) = if here.node == start {
                (&a.worldspace, a.pos)
            } else {
                if settled[here.node] <= here.hops {
                    continue; // dominated: settled before at no more cost, with no more hops
                }
                settled[here.node] = here.hops;
                let (_, out) = self.openings(here.node);
                (&out.worldspace, out.frame.pos)
            };
            let mut push = |labels: &mut Vec<Label>, next: Label| {
                open.push(Pending { cost: next.cost, node: next.node, hops: next.hops, label: labels.len() });
                labels.push(next);
            };
            if *ws == b.worldspace {
                let leg = metres(at, b.pos);
                push(
                    &mut labels,
                    Label {
                        node: goal,
                        cost: here.cost + leg,
                        metres: here.metres + leg,
                        parent: li,
                        ..here
                    },
                );
            }
            if here.hops >= rules.max_hops {
                continue;
            }
            let hops = here.hops + 1;
            for &j in self.by_space.get(ws).into_iter().flatten() {
                let j = j as usize;
                if settled[j] <= hops {
                    continue;
                }
                if shut[j] == 0 {
                    shut[j] = if (rules.door_state)(self.exits[j].link) == DoorState::Open { 1 } else { 2 };
                }
                let is_shut = shut[j] == 2;
                let (into, _) = self.openings(j);
                let leg = metres(at, into.frame.pos);
                push(
                    &mut labels,
                    Label {
                        node: j,
                        hops,
                        cost: here.cost + leg + if is_shut { weight } else { 0.0 },
                        metres: here.metres + leg,
                        closed: here.closed + u32::from(is_shut),
                        parent: li,
                    },
                );
            }
        }
        None
    }

    fn unwind_path(&self, labels: &[Label], goal: usize) -> PortalPath {
        let n = self.exits.len();
        let end = labels[goal];
        let mut links = PortalLinks::new();
        let mut li = end.parent;
        while labels[li].node < n {
            links.push(self.exits[labels[li].node].link);
            li = labels[li].parent;
        }
        links.reverse();
        PortalPath { metres: to_f32(end.metres), links, closed_doors: end.closed }
    }

    /// Whether `a` sees `b` (module docs): one ray in the same space, else through one open
    /// seamless link joining the two spaces. `door_state` is the host's current state of each link.
    pub fn line_of_sight<L: LosProvider + ?Sized>(
        &self,
        a: &Location,
        b: &Location,
        door_state: &dyn Fn(LinkId) -> DoorState,
        los: &L,
    ) -> Los {
        if a.worldspace == b.worldspace {
            return if los.ray(a, b) { Los::Clear } else { Los::Blocked };
        }
        // Links joining the two spaces, with the side the viewer looks through (either direction
        // of travel: sight ignores one-way).
        let mut through: SmallVec<[(LinkId, LinkSide); 4]> = self
            .exits_from(&a.worldspace)
            .filter(|e| e.arrive.worldspace == b.worldspace)
            .map(|e| (e.link, e.side))
            .chain(
                self.exits_from(&b.worldspace)
                    .filter(|e| e.arrive.worldspace == a.worldspace)
                    .map(|e| (e.link, other(e.side))),
            )
            .collect();
        through.sort_unstable();
        through.dedup();
        let mut verdict = Los::BlockedByLink;
        for (id, side) in through {
            let Some(graph_link) = self.link(id) else { continue };
            if LinkKind::from_flags(graph_link.flags) != LinkKind::Seamless
                || door_state(id).openness() <= 0.0
            {
                continue;
            }
            let Some(at_door) = crossing(a, b, &graph_link.link, side) else { continue };
            let beyond = map(&at_door, &graph_link.link, side);
            if los.ray(a, &at_door) && los.ray(&beyond, b) {
                return Los::Clear;
            }
            verdict = Los::Blocked;
        }
        verdict
    }
}

/// Where the segment from `a` to `b` seen through the portal (b mapped back into a's space)
/// crosses the door plane of `link`'s `from` end, if it does so inside the opening with `a` in
/// front of the plane and `b` beyond it.
fn crossing(a: &Location, b: &Location, link: &DoorLink, from: LinkSide) -> Option<Location> {
    let (near, far) = ends(link, from);
    let to_local = near.frame.rotation.inverse();
    let la = to_local * near.frame.pos.delta_f32(a.pos);
    let b_seen = DoorLink::cross(far, near, &WorldTransform::from_pos(b.pos)).pos;
    let lb = to_local * near.frame.pos.delta_f32(b_seen);
    if !(la.z > 0.0 && lb.z < 0.0) {
        return None;
    }
    let p = la + (lb - la) * (la.z / (la.z - lb.z));
    let half: Vec2 = near.extent * 0.5;
    let within = |v: f32, h: f32| if h > 0.0 { (v.abs() <= h).then_some(v) } else { Some(0.0) };
    let on_plane = Vec3::new(within(p.x, half.x)?, within(p.y, half.y)?, 0.0);
    Some(Location::new(near.worldspace.clone(), near.frame.pos.offset(near.frame.rotation * on_plane)))
}
