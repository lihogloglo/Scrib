//! `MA21`: the game's worldspace graph (`sf_nav_bevy::NavWorldGraph`, nav.md "Doors") over its
//! door links (`sf_world_bevy::DoorLinks`, from `mw_doors_bevy`).
//!
//! - Approach points ([`approach_point`]): each link end's walkable point is the pathgrid node of
//!   that end's cell nearest its arrival position, in front of the door plane (along the arrival
//!   facing) and within `mw.nav.door_approach_m` ([`DOOR_APPROACH_M`]). Interiors look up their own
//!   grid by name; the exterior the arrival's grid square and its 8 neighbours (a door near a cell
//!   border). No such node: the arrival position ([`NavLinkRules::arrival_approach`]), so the
//!   link stays in the graph. Both cases are counted.
//! - Flags: [`NavLinkRules::classic_flags`] (fade-only, one-way when the record is).
//! - Door states ([`door_states`]): a link whose door reference (either end) carries a lock level
//!   above 0 at load reads [`DoorState::Locked`]; every other link is unreported (closed).
//! - One line per graph rebuild: `nav: door approach N from pathgrid, M arrival fallback, L locked`
//!   (link ends, then links), after the plugin's `nav: world graph ...` line.
//!
//! Spec: `bevyrrowind/docs/specs/ai-adapter.md` (MA21), `superfargoth/docs/specs/nav.md` "Doors".

use std::collections::HashSet;
use std::hash::BuildHasher;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use bevy::prelude::*;
use mw_ai::PathGrids;
use mw_world::ids;
use sf_nav::DoorState;
use sf_nav_bevy::{NavDoorStates, NavLinkRules, NavSystems, NavWorldGraphPlugin, NavWorldGraphStats};
use sf_records::{RecordDb, SourceId};
use sf_tunables_bevy::TunablesAppExt;
use sf_world::{DoorLink, LinkEnd, Location, PlacementId};
use sf_world_bevy::DoorLinks;
use tes3::esp::Cell;

/// `mw.nav.door_approach_m` ([`DoorApproach::max_m`]) and the approach rule live in Bevy-free
/// `mw_ai::nav` (`NV12c2`), so the route bake places its door nodes with the same function.
pub use mw_ai::nav::{DOOR_APPROACH_M, approach_point};

/// The approach search's live values, bound from `mw.nav.*`; a change rebuilds the rules (and so
/// the graph).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct DoorApproach {
    pub max_m: f32,
}

impl Default for DoorApproach {
    fn default() -> Self {
        Self { max_m: DOOR_APPROACH_M.default_value() }
    }
}

/// The pathgrids the approach rule reads (built once at load).
///
/// `MA21b`: the walkers build their own copy (`mw_walkers_bevy` `Pathgrids`); one shared resource
/// replaces both.
#[derive(Resource, Clone, Default)]
pub struct NavPathGrids(pub Arc<PathGrids>);

/// Link ends placed since the last report.
#[derive(Default, Debug)]
pub struct ApproachCounts {
    pub pathgrid: AtomicUsize,
    pub fallback: AtomicUsize,
}

/// The counters the current rules write.
#[derive(Resource, Clone, Default)]
struct ApproachCounters(Arc<ApproachCounts>);

/// The rules: [`approach_point`] or the arrival, counted in `counts`; classic flags.
///
/// `world.doors` (the seamless vs classic world rule, ai-adapter.md) will replace
/// [`NavLinkRules::classic_flags`] once it exists.
pub fn link_rules(grids: Arc<PathGrids>, max_m: f32, counts: Arc<ApproachCounts>) -> NavLinkRules {
    NavLinkRules::new(
        move |end: &LinkEnd| {
            if let Some(p) = approach_point(&grids, end, max_m) {
                counts.pathgrid.fetch_add(1, Ordering::Relaxed);
                Some(Location::new(end.worldspace.clone(), p))
            } else {
                counts.fallback.fetch_add(1, Ordering::Relaxed);
                NavLinkRules::arrival_approach(end)
            }
        },
        NavLinkRules::classic_flags,
    )
}

/// Door references (with a destination) carrying a lock level above 0, by placement id (as
/// `mw_world::worldspaces::door_ends` names them).
pub fn locked_doors(db: &RecordDb) -> HashSet<PlacementId> {
    let mut out = HashSet::new();
    for (_, cell) in db.iter::<Cell>() {
        for (&(source, refr_index), reference) in &cell.references {
            if reference.destination.is_some() && reference.lock_level.is_some_and(|l| l > 0) {
                out.insert(ids::placement_id(&db.source(SourceId(source)).name, refr_index));
            }
        }
    }
    out
}

/// [`DoorState::Locked`] for each link with a locked door at either end; nothing for the rest
/// (they read closed).
///
/// Static, from the load: the game has no runtime lock state yet. The interaction lane's locks
/// (Lock/Unlock, lockpicking) will write their changes here.
pub fn door_states<S: BuildHasher>(links: &[DoorLink], locked: &HashSet<PlacementId, S>) -> NavDoorStates {
    let is_locked = |e: &LinkEnd| e.door.is_some_and(|d| locked.contains(&d));
    NavDoorStates(
        links
            .iter()
            .filter(|l| is_locked(&l.a) || is_locked(&l.b))
            .map(|l| (l.id(), DoorState::Locked))
            .collect(),
    )
}

/// Adds the worldspace graph over [`DoorLinks`] with the game's rules. `db`: the loaded records
/// (`None`: no pathgrids or locks, every approach is the arrival). Call before `links` move into
/// `mw_doors_bevy::add`.
pub fn add(app: &mut App, db: Option<&RecordDb>, links: &[DoorLink]) {
    let grids = Arc::new(db.map(PathGrids::build).unwrap_or_default());
    let states = db.map(|db| door_states(links, &locked_doors(db))).unwrap_or_default();
    let counts = ApproachCounters::default();
    let settings = DoorApproach::default();
    app.add_plugins(NavWorldGraphPlugin::new(|l: &DoorLinks| &l.0))
        .insert_resource(link_rules(grids.clone(), settings.max_m, counts.0.clone()))
        .insert_resource(NavPathGrids(grids))
        .insert_resource(states)
        .insert_resource(counts)
        .insert_resource(settings)
        .add_systems(Update, rebuild_rules.before(NavSystems::WorldGraph))
        .add_systems(Update, report.after(NavSystems::WorldGraph));
    let _approach = app.add_tunable(&DOOR_APPROACH_M).bind_resource(|s: &mut DoorApproach, v| s.max_m = v);
}

/// A changed [`DoorApproach`] replaces the rules (the plugin then rebuilds the graph).
fn rebuild_rules(
    settings: Res<DoorApproach>,
    grids: Res<NavPathGrids>,
    counts: Res<ApproachCounters>,
    mut rules: ResMut<NavLinkRules>,
) {
    if settings.is_changed() && !settings.is_added() {
        *rules = link_rules(grids.0.clone(), settings.max_m, counts.0.clone());
    }
}

/// Logs each rebuild's approach counts and the locked links, then resets the counts.
fn report(
    stats: Res<NavWorldGraphStats>,
    counts: Res<ApproachCounters>,
    doors: Res<NavDoorStates>,
    mut reported: Local<u64>,
) {
    if stats.rebuilds == *reported {
        return;
    }
    *reported = stats.rebuilds;
    let pathgrid = counts.0.pathgrid.swap(0, Ordering::Relaxed);
    let fallback = counts.0.fallback.swap(0, Ordering::Relaxed);
    let locked = doors.0.values().filter(|s| **s == DoorState::Locked).count();
    info!("nav: door approach {pathgrid} from pathgrid, {fallback} arrival fallback, {locked} locked");
}

#[cfg(test)]
#[path = "nav_links_tests.rs"]
mod tests;
