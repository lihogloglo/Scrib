//! `NV11d1`: the worldspace graph as a resource (nav.md "Worldspace graph", "Doors").
//!
//! - [`NavWorldGraph`]: the [`WorldGraph`] built from the host's door links, rebuilt whenever the
//!   links or the [`NavLinkRules`] change (`is_changed`), logging `nav: world graph N links (M
//!   skipped)` once per rebuild; [`NavWorldGraphStats`] keeps that rebuild's counts and the exits
//!   leaving each worldspace for `nav.links`.
//! - The links come from any host resource through [`NavWorldGraphPlugin`]'s accessor, e.g.
//!   `NavWorldGraphPlugin::new(|l: &DoorLinks| &l.0)` for `sf_world_bevy::DoorLinks`. No edge to
//!   `sf_world_bevy`: it carries the render, PBR and UI stack, and this shell stays thin and shallow
//!   (build-perf.md rules 2, 3 and 6); the accessor reads the host's resource in place, so there is
//!   no copy and change detection is the source's own.
//! - [`NavLinkRules`]: the adapter hook for each end's approach point and each link's flags (the
//!   `world.doors` host world rule). Default: the approach point is the end's `arrival` position in
//!   its worldspace, and every link is [`LinkFlags::FADE_ONLY`] (classic load doors), plus
//!   [`LinkFlags::ONE_WAY`] when the record isn't two-way. The game adapter replaces both (pathgrid
//!   approach nodes, seamless doors) in a later slice.
//! - [`NavDoorStates`]: each link's dynamic state, written by the host each tick. A link the host
//!   hasn't reported reads [`DoorState::Closed`]: shut but unlocked, so routes pass it (the traverse
//!   opens it) and sight doesn't (openness 0), the state a door rests in until something opens or
//!   locks it.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::marker::PhantomData;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_log::info;
use sf_nav::{DoorState, LinkFlags, LinkSide, WorldGraph};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, WorldspaceId};

use crate::NavSystems;

/// The worldspace graph the router and the through-door queries read.
#[derive(Resource, Clone, Debug, Default)]
pub struct NavWorldGraph(pub WorldGraph);

/// The last rebuild's counts (`nav.links`).
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct NavWorldGraphStats {
    pub links: usize,
    pub skipped: usize,
    /// Exits leaving each worldspace (a two-way link counts once per side).
    pub exits: BTreeMap<WorldspaceId, usize>,
    pub rebuilds: u64,
}

impl NavWorldGraphStats {
    /// `nav: world graph 3 links (1 skipped), 5 exits: a 2, b 1, e 2; 2 rebuilds`.
    pub fn line(&self) -> String {
        let per_space = self
            .exits
            .iter()
            .map(|(ws, n)| format!("{ws} {n}"))
            .collect::<Vec<_>>()
            .join(", ");
        let exits: usize = self.exits.values().sum();
        format!(
            "nav: world graph {} links ({} skipped), {exits} exits: {}; {} rebuilds",
            self.links,
            self.skipped,
            if per_space.is_empty() {
                "none"
            } else {
                &per_space
            },
            self.rebuilds,
        )
    }
}

type ApproachFn = dyn Fn(&LinkEnd) -> Option<Location> + Send + Sync;
type FlagsFn = dyn Fn(&DoorLink) -> LinkFlags + Send + Sync;

/// The adapter's door rules for building the graph: replace the resource to change them (the
/// graph rebuilds).
#[derive(Resource)]
pub struct NavLinkRules {
    /// Each end's approach point (the walkable point in front of the door) in its own worldspace;
    /// `None` leaves the link out of the graph.
    pub approach: Box<ApproachFn>,
    /// The link's flags: seamless, fade-only, one-way.
    pub flags: Box<FlagsFn>,
}

impl NavLinkRules {
    pub fn new(
        approach: impl Fn(&LinkEnd) -> Option<Location> + Send + Sync + 'static,
        flags: impl Fn(&DoorLink) -> LinkFlags + Send + Sync + 'static,
    ) -> Self {
        Self {
            approach: Box::new(approach),
            flags: Box::new(flags),
        }
    }

    /// The default approach point: where a traveler arrives at that end.
    pub fn arrival_approach(end: &LinkEnd) -> Option<Location> {
        Some(Location::new(end.worldspace.clone(), end.arrival.pos))
    }

    /// The default flags: fade-only (classic load doors), one-way when the record is.
    pub fn classic_flags(link: &DoorLink) -> LinkFlags {
        if link.two_way {
            LinkFlags::FADE_ONLY
        } else {
            LinkFlags::FADE_ONLY | LinkFlags::ONE_WAY
        }
    }

    /// Builds a graph from `links` with these rules.
    pub fn build(&self, links: &[DoorLink]) -> WorldGraph {
        WorldGraph::build(links, &*self.approach, &*self.flags)
    }
}

impl Default for NavLinkRules {
    fn default() -> Self {
        Self::new(Self::arrival_approach, Self::classic_flags)
    }
}

/// Each link's dynamic state, from the host each tick; unreported links read
/// [`DoorState::Closed`].
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct NavDoorStates(pub HashMap<LinkId, DoorState>);

impl NavDoorStates {
    /// The state of `id` (closed unless the host says otherwise).
    pub fn state(&self, id: LinkId) -> DoorState {
        self.0.get(&id).copied().unwrap_or(DoorState::Closed)
    }
}

/// Builds [`NavWorldGraph`] from the host's link resource `R` (read through `links`) and
/// rebuilds it when `R` or [`NavLinkRules`] changes. Rebuilds in `Update`, in
/// [`NavSystems::WorldGraph`].
pub struct NavWorldGraphPlugin<R: Resource> {
    links: fn(&R) -> &[DoorLink],
    _source: PhantomData<fn() -> R>,
}

impl<R: Resource> NavWorldGraphPlugin<R> {
    pub fn new(links: fn(&R) -> &[DoorLink]) -> Self {
        Self {
            links,
            _source: PhantomData,
        }
    }
}

impl<R: Resource> Plugin for NavWorldGraphPlugin<R> {
    fn build(&self, app: &mut App) {
        let links = self.links;
        app.init_resource::<NavWorldGraph>()
            .init_resource::<NavWorldGraphStats>()
            .init_resource::<NavLinkRules>()
            .init_resource::<NavDoorStates>()
            .add_systems(
                Update,
                (move |source: Option<Res<R>>,
                       rules: Res<NavLinkRules>,
                       graph: ResMut<NavWorldGraph>,
                       stats: ResMut<NavWorldGraphStats>| {
                    rebuild(
                        source.as_ref().map(|s| (links(s), s.is_changed())),
                        &rules,
                        graph,
                        stats,
                    );
                })
                .in_set(NavSystems::WorldGraph),
            );
    }
}

/// Rebuilds when the source (`(links, changed)`; `None` while the host hasn't inserted it) or the
/// rules changed.
fn rebuild(
    source: Option<(&[DoorLink], bool)>,
    rules: &Res<NavLinkRules>,
    mut graph: ResMut<NavWorldGraph>,
    mut stats: ResMut<NavWorldGraphStats>,
) {
    let Some((links, changed)) = source else {
        return;
    };
    if !changed && !rules.is_changed() {
        return;
    }
    let built = rules.build(links);
    // Exits per worldspace; a duplicate id names the same exits, so each id counts once.
    let mut seen = HashSet::new();
    let mut exits = BTreeMap::new();
    for id in links.iter().map(DoorLink::id).filter(|id| seen.insert(*id)) {
        for side in [LinkSide::A, LinkSide::B] {
            if let Some(exit) = built.exit(id, side) {
                *exits.entry(exit.depart.worldspace.clone()).or_insert(0) += 1;
            }
        }
    }
    info!(
        "nav: world graph {} links ({} skipped)",
        built.len(),
        built.skipped()
    );
    *stats = NavWorldGraphStats {
        links: built.len(),
        skipped: built.skipped(),
        exits,
        rebuilds: stats.rebuilds + 1,
    };
    graph.0 = built;
}
