//! The worldspace graph (nav.md "Worldspace graph", "Routing" level 1, "Doors"): worldspaces are
//! the nodes, door links the edges, and [`WorldGraph::route`] picks which links an agent takes.
//!
//! Each link end has an **approach point** (the adapter's walkable point in front of the door) in
//! its own worldspace; one traverse joins the two. A route is A* over link exits with cost =
//! walking time: the straight leg from where the agent stands to the next approach point at
//! [`RouteRules::speed_mps`], plus the link's traverse time ([`TraverseTimes`], per
//! [`LinkKind`]), then the straight leg in the next space, and so on to the goal. The in-space
//! cost is the straight-line distance between points (like the HPA* stub) until the coarse graph
//! (NV12) gives each space its own travel times.
//!
//! Door state (open, closed, locked) comes from the host per query; a locked link is refused
//! unless the agent's [`RouteRules::unlock`] allows it (key, lockpick, a world rule), a closed
//! one is fine (the traverse opens it). One-way links ([`DoorLink::two_way`] false, or
//! [`LinkFlags::ONE_WAY`] from the host) only leave from their `a` end. Routes name links by
//! [`LinkId`] and are serde (the AI saves them), never by `Entity` or polygon.
//!
//! Deterministic: exits are kept in `(LinkId, side)` order and the open list breaks equal costs
//! by that order, so equal-cost routes resolve to the lower `LinkId` first and the result is a
//! pure function of the inputs.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, WorldspaceId};

// tunables: const (Guess: open a hinged door, step through the gate, close it; bound from
// ai.nav.* in the host adapter, NV11d)
pub const SEAMLESS_TRAVERSE_S: f64 = 3.0;

// tunables: const (Guess: activate, fade out, arrive; Bethesda load-door teleport; bound from
// ai.nav.* in the host adapter, NV11d)
pub const FADE_TRAVERSE_S: f64 = 1.0;

bitflags::bitflags! {
    /// How a link may be crossed: the adapter's world rule (`world.doors`: pack default, per-door
    /// overrides) and per-door data, supplied to [`WorldGraph::build`].
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct LinkFlags: u8 {
        /// Portal-ready: walked through in the world (nav.md "Doors", seamless link).
        const SEAMLESS = 1;
        /// Taken only out of every player's view (fade-only link); wins over `SEAMLESS`.
        const FADE_ONLY = 1 << 1;
        /// Leaves only from the link's `a` end, even if the record is two-way.
        const ONE_WAY = 1 << 2;
    }
}

/// How a link is traversed, which sets its traverse time and its operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkKind {
    /// Walked through in the world.
    Seamless,
    /// Out-of-view teleport (classic load door, trapdoor).
    Fade,
}

impl LinkKind {
    /// `Seamless` only when the flags say so and don't also force fade.
    pub fn from_flags(flags: LinkFlags) -> Self {
        if flags.contains(LinkFlags::SEAMLESS) && !flags.contains(LinkFlags::FADE_ONLY) {
            Self::Seamless
        } else {
            Self::Fade
        }
    }
}

/// Which end of a [`DoorLink`] a traveler leaves from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LinkSide {
    A,
    B,
}

/// A link's dynamic state, from the host each query.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoorState {
    Open,
    /// Shut but not locked: passable, the traverse opens it.
    Closed,
    /// Refused unless [`RouteRules::unlock`] allows it.
    Locked,
}

impl DoorState {
    /// How open the link is, 0-1: `Open` 1, `Closed` and `Locked` 0. Sight crosses a seamless
    /// link only above 0 ([`WorldGraph::line_of_sight`]); the host reports a door that has
    /// started to swing as `Open`.
    pub fn openness(self) -> f32 {
        match self {
            Self::Open => 1.0,
            Self::Closed | Self::Locked => 0.0,
        }
    }
}

/// Seconds a traverse takes, per [`LinkKind`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TraverseTimes {
    pub seamless_s: f64,
    pub fade_s: f64,
}

impl Default for TraverseTimes {
    fn default() -> Self {
        Self { seamless_s: SEAMLESS_TRAVERSE_S, fade_s: FADE_TRAVERSE_S }
    }
}

impl TraverseTimes {
    pub fn of(&self, kind: LinkKind) -> f64 {
        match kind {
            LinkKind::Seamless => self.seamless_s,
            LinkKind::Fade => self.fade_s,
        }
    }
}

/// The agent's side of a route query.
#[derive(Clone, Copy)]
pub struct RouteRules<'a> {
    /// Walking speed, meters per second (must be > 0).
    pub speed_mps: f64,
    pub traverse: TraverseTimes,
    /// Whether this agent may pass a locked link (key, lockpick, faction rule).
    pub unlock: &'a dyn Fn(LinkId) -> bool,
}

fn never(_: LinkId) -> bool {
    false
}

impl RouteRules<'static> {
    /// Walks at `speed_mps`, default traverse times, opens no locks.
    pub fn walking(speed_mps: f64) -> Self {
        Self { speed_mps, traverse: TraverseTimes::default(), unlock: &never }
    }
}

/// One directed crossing of a link: from the approach point on the leaving side to the one on
/// the arriving side.
#[derive(Clone, Debug, PartialEq)]
pub struct GraphExit {
    pub link: LinkId,
    pub side: LinkSide,
    pub kind: LinkKind,
    /// Approach point in the worldspace being left.
    pub depart: Location,
    /// Approach point in the worldspace reached.
    pub arrive: Location,
}

/// A link as the graph holds it.
#[derive(Clone, Debug, PartialEq)]
pub struct GraphLink {
    pub flags: LinkFlags,
    /// The record: portal frames and extents for the through-door queries (`portal_query`).
    pub link: DoorLink,
    /// Exit indices leaving from the `a` and `b` ends (`None` when one-way).
    exits: [Option<u32>; 2],
}

/// One crossing of a [`WorldRoute`]: walk `walk_s` to `depart`, then traverse `link` from `side`
/// in `traverse_s`, arriving at `arrive`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteLeg {
    pub link: LinkId,
    pub side: LinkSide,
    pub kind: LinkKind,
    pub depart: Location,
    pub arrive: Location,
    pub walk_s: f64,
    pub traverse_s: f64,
}

/// Which links to take, in order, and the walking time it costs (what the AI saves).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldRoute {
    pub links: Vec<LinkId>,
    /// Total seconds: every leg's walk and traverse plus `final_walk_s`.
    pub cost_s: f64,
    pub legs: Vec<RouteLeg>,
    /// The straight walk from the last arrival (or the start) to the goal.
    pub final_walk_s: f64,
}

/// The worldspace graph, built at load from the door links (nav.md "Worldspace graph").
#[derive(Clone, Debug, Default)]
pub struct WorldGraph {
    /// Sorted by `(link, side)`: the index is the A* node and the tie-break order.
    pub(crate) exits: Vec<GraphExit>,
    /// Exits leaving each worldspace, in exit order.
    pub(crate) by_space: BTreeMap<WorldspaceId, Vec<u32>>,
    pub(crate) index: HashMap<LinkId, GraphLink>,
    skipped: usize,
}

impl WorldGraph {
    /// Builds the graph. `approach` gives each end's approach point (the adapter's nearest
    /// walkable point in front of the door); a link with an end it can't place is skipped, as is
    /// a second link with an id already taken. `flags` is the host's seamless/fade/one-way rule.
    pub fn build(
        links: &[DoorLink],
        approach: impl Fn(&LinkEnd) -> Option<Location>,
        flags: impl Fn(&DoorLink) -> LinkFlags,
    ) -> Self {
        let mut graph = Self::default();
        let mut exits = Vec::new();
        for link in links {
            let id = link.id();
            if graph.index.contains_key(&id) {
                graph.skipped += 1;
                continue;
            }
            let (Some(at_a), Some(at_b)) = (approach(&link.a), approach(&link.b)) else {
                graph.skipped += 1;
                continue;
            };
            let f = flags(link);
            let kind = LinkKind::from_flags(f);
            exits.push(GraphExit {
                link: id,
                side: LinkSide::A,
                kind,
                depart: at_a.clone(),
                arrive: at_b.clone(),
            });
            if link.two_way && !f.contains(LinkFlags::ONE_WAY) {
                exits.push(GraphExit { link: id, side: LinkSide::B, kind, depart: at_b, arrive: at_a });
            }
            graph.index.insert(id, GraphLink { flags: f, link: link.clone(), exits: [None, None] });
        }
        exits.sort_by_key(|e| (e.link, e.side));
        for (i, exit) in exits.iter().enumerate() {
            let i = u32::try_from(i).expect("fewer than 2^32 link exits");
            let entry = graph.index.get_mut(&exit.link).expect("indexed above");
            entry.exits[usize::from(exit.side == LinkSide::B)] = Some(i);
            graph.by_space.entry(exit.depart.worldspace.clone()).or_default().push(i);
        }
        graph.exits = exits;
        graph
    }

    /// Links in the graph.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Links left out at build (no approach point for an end, or a duplicate id).
    pub fn skipped(&self) -> usize {
        self.skipped
    }

    pub fn link(&self, id: LinkId) -> Option<&GraphLink> {
        self.index.get(&id)
    }

    /// The record of link `id`: the index [`DoorLink::find`] defers to.
    pub fn door_link(&self, id: LinkId) -> Option<&DoorLink> {
        self.index.get(&id).map(|l| &l.link)
    }

    /// The crossing of `id` leaving from `side`, if that direction exists.
    pub fn exit(&self, id: LinkId, side: LinkSide) -> Option<&GraphExit> {
        let i = self.index.get(&id)?.exits[usize::from(side == LinkSide::B)]?;
        Some(&self.exits[i as usize])
    }

    /// Exits leaving `ws`, in `(LinkId, side)` order.
    pub fn exits_from(&self, ws: &WorldspaceId) -> impl Iterator<Item = &GraphExit> {
        self.by_space.get(ws).into_iter().flatten().map(|&i| &self.exits[i as usize])
    }

    /// The cheapest route from `from` to `to` in walking time, or `None` when no passable chain
    /// of links joins their worldspaces (or `rules.speed_mps` isn't positive). Same worldspace:
    /// an empty route costing the straight walk (in-space connectivity is the coarse graph's,
    /// NV12). `door_state` is the host's current state of each link.
    pub fn route(
        &self,
        from: &Location,
        to: &Location,
        door_state: &dyn Fn(LinkId) -> DoorState,
        rules: &RouteRules<'_>,
    ) -> Option<WorldRoute> {
        let speed = rules.speed_mps;
        if !(speed > 0.0 && speed.is_finite()) {
            return None;
        }
        let walk = |a: &Location, b: &Location| a.pos.delta(b.pos).length() / speed;
        if from.worldspace == to.worldspace {
            let final_walk_s = walk(from, to);
            return Some(WorldRoute {
                links: Vec::new(),
                cost_s: final_walk_s,
                legs: Vec::new(),
                final_walk_s,
            });
        }

        // Nodes: 0..n = "arrived through exit i", n = start, n + 1 = goal.
        let n = self.exits.len();
        let (start, goal) = (n, n + 1);
        let mut g = vec![f64::INFINITY; n + 2];
        let mut pred = vec![usize::MAX; n + 2];
        // Per exit: 0 = not asked yet, 1 = passable, 2 = refused.
        let mut pass = vec![0_u8; n];
        let mut open = BinaryHeap::new();
        g[start] = 0.0;
        open.push(Open { f: 0.0, g: 0.0, node: start });

        // Admissible: 0 outside the goal's space (any number of links may follow), the straight
        // walk to the goal inside it. Not consistent, so improved nodes are reopened.
        let h = |at: &Location| if at.worldspace == to.worldspace { walk(at, to) } else { 0.0 };

        while let Some(Open { g: g_here, node, .. }) = open.pop() {
            if g_here > g[node] {
                continue; // stale entry
            }
            if node == goal {
                return Some(self.unwind(&pred, &g, start, goal, from, rules, walk));
            }
            let at = if node == start { from } else { &self.exits[node].arrive };
            let mut relax = |next: usize, cost: f64, h_next: f64, open: &mut BinaryHeap<Open>| {
                if cost < g[next] {
                    g[next] = cost;
                    pred[next] = node;
                    open.push(Open { f: cost + h_next, g: cost, node: next });
                }
            };
            if at.worldspace == to.worldspace {
                relax(goal, g_here + walk(at, to), 0.0, &mut open);
            }
            for &i in self.by_space.get(&at.worldspace).into_iter().flatten() {
                let i = i as usize;
                let exit = &self.exits[i];
                if pass[i] == 0 {
                    let ok = match door_state(exit.link) {
                        DoorState::Open | DoorState::Closed => true,
                        DoorState::Locked => (rules.unlock)(exit.link),
                    };
                    pass[i] = if ok { 1 } else { 2 };
                }
                if pass[i] == 2 {
                    continue;
                }
                let cost = g_here + walk(at, &exit.depart) + rules.traverse.of(exit.kind);
                relax(i, cost, h(&exit.arrive), &mut open);
            }
        }
        None
    }

    #[allow(clippy::too_many_arguments)]
    fn unwind(
        &self,
        pred: &[usize],
        g: &[f64],
        start: usize,
        goal: usize,
        from: &Location,
        rules: &RouteRules<'_>,
        walk: impl Fn(&Location, &Location) -> f64,
    ) -> WorldRoute {
        let mut chain = Vec::new();
        let mut node = pred[goal];
        while node != start {
            chain.push(node);
            node = pred[node];
        }
        chain.reverse();
        let mut legs = Vec::with_capacity(chain.len());
        let mut at = from;
        for &i in &chain {
            let exit = &self.exits[i];
            legs.push(RouteLeg {
                link: exit.link,
                side: exit.side,
                kind: exit.kind,
                depart: exit.depart.clone(),
                arrive: exit.arrive.clone(),
                walk_s: walk(at, &exit.depart),
                traverse_s: rules.traverse.of(exit.kind),
            });
            at = &exit.arrive;
        }
        let last = chain.last().map_or(0.0, |&i| g[i]);
        WorldRoute {
            links: legs.iter().map(|l| l.link).collect(),
            cost_s: g[goal],
            legs,
            final_walk_s: g[goal] - last,
        }
    }
}

/// An open-list entry: min-heap on `f`, ties to the lower node (lower `(LinkId, side)`).
#[derive(Clone, Copy)]
struct Open {
    f: f64,
    g: f64,
    node: usize,
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
