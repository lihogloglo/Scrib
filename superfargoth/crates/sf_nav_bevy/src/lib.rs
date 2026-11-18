//! Bevy shell for [`sf_nav`] (`docs/specs/nav.md`). Built in NV3 (debug draw):
//!
//! - [`NavMeshes`]: the resident navmesh as a `dyn` [`NavBackend`] resource (the game or a test
//!   inserts it; NV5 streaming and the NV6 query service build on it). No backend code compiles
//!   here: `sf_nav_recast` or the `GridBackend` double plug in behind the trait.
//! - `nav.stats`: tiles resident per agent class, MiB, query queue depth, ms per tick, rebuilds
//!   ([`NavStats`]); counters a later slice owns ([`NavCounters`]: NV6 queue and tick time, NV10
//!   rebuilds) print `n/a` until it fills them. [`brp_stats_system`] serves the same as JSON
//!   ([`BRP_STATS_METHOD`]) for the app that owns BRP.
//! - `nav.draw <layer> [actor]`: toggles a debug draw layer ([`NavDraw`]). Built: `tiles` (every
//!   resident tile's polygons, walls brighter) and `paths [actor]` (paths recorded in
//!   [`NavDebugPaths`], one actor's or all). `links`, `doors`, `obstacles`, `boundary`, `coarse` and
//!   `graph` print which slice builds their data. `nav.draw off` clears every layer.
//! - Feature `debug`: the layers as immediate-mode gizmos (UE's navmesh show flag, Detour's
//!   `duDebugDrawNavMesh`), drawn only when the app has Bevy's gizmo plugin; positions go through
//!   [`NavDrawFrame`] (the render origin and the worldspace to show). [`NavDrawn`] counts what the
//!   last frame drew, for log-line checks (no visual judgement by agents).
//! - `NV4c`, [`NavRingPlugin`] (module [`residency`]): baked `nav.tiles` products kept resident in
//!   [`NavMeshes`] around one [`NavAnchor`] with load/unload hysteresis and a per-frame tile cap;
//!   `nav.stats` adds its counts ([`RingStats`]). Scaffold `NavRingScaffold`: NV5 replaces it with
//!   per-anchor, per-portal, budgeted residency on the streaming scheduler.
//! - NV6, [`NavQueryPlugin`] (module [`queries`]): the path query service on the fixed tick.
//!   Callers request through [`NavQueries`] and take replies published at the next tick boundary;
//!   `nav.stats` shows the queue, running jobs and the last tick's run (`queue 3 (1 running), tick
//!   0.04 ms (2 jobs, 256 nodes, 1 results)`).

use std::collections::BTreeMap;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use serde::Serialize;
use serde_json::Value;
use sf_coords::WorldPos;
use sf_nav::{AgentClassId, NavBackend, PathResult};
use sf_world::{Location, WorldspaceId};

mod console;
pub mod corridor;
#[cfg(feature = "debug")]
mod gizmos;
pub mod queries;
pub mod residency;

#[cfg(test)]
mod corridor_tests;
#[cfg(test)]
mod queries_tests;
#[cfg(test)]
mod residency_tests;
#[cfg(test)]
mod tests;

pub use corridor::{
    CORRIDOR_LOOKAHEAD_POLYS, CORRIDOR_MAX_CORNERS, CORRIDOR_OPTIMIZE_RANGE_M,
    CORRIDOR_REPAIR_DRIFT_M, CORRIDOR_REPAIR_NODES, CORRIDOR_TOPOLOGY_PERIOD_S, NavCorridor,
    NavCorridorPlugin, NavCorridorSettings,
};
pub use queries::{NavQueries, NavQueryPlugin, NavQuerySettings};
pub use residency::{
    BakedNavRing, NavAnchor, NavRingPlugin, NavRingSettings, RING_LOAD_M, RING_TILES_PER_FRAME,
    RING_UNLOAD_M, RingStats,
};

/// The nav systems' sets, for hosts to order against: write [`NavAnchor`] before `Residency`
/// (`Update`), [`NavDrawFrame`] before `Draw` (`PostUpdate`, feature `debug`); request paths
/// before `Queries` (`FixedUpdate`) to have them planned that tick; replies are published in
/// `Publish` (`FixedFirst`), before any `FixedUpdate` system; `Corridor` (`FixedUpdate`, inside
/// `ControllerSystems::Drive`) steps [`NavCorridor`]s before `Queries`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NavSystems {
    Residency,
    Draw,
    Publish,
    Corridor,
    Queries,
}

/// The BRP method [`brp_stats_system`] answers.
pub const BRP_STATS_METHOD: &str = "sf_nav/stats";

/// Adds the nav resources and the `nav.*` console commands; with feature `debug`, the gizmos.
pub struct NavPlugin;

impl Plugin for NavPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavCounters>()
            .init_resource::<NavDebugPaths>()
            .init_resource::<NavDraw>()
            .init_resource::<NavDrawFrame>()
            .init_resource::<NavDrawn>();
        console::register(app);
        #[cfg(feature = "debug")]
        gizmos::add(app);
    }
}

/// The resident navmesh: every loaded tile of every worldspace, agent class and nav region.
#[derive(Resource)]
pub struct NavMeshes(pub Box<dyn NavBackend>);

/// Counters the query service and runtime patches fill (NV6 [`NavQueryPlugin`]: the queue and
/// the last tick's run; NV10 rebuilds); `None` reads `n/a`.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct NavCounters {
    /// Requests queued or running.
    pub queue_depth: Option<usize>,
    /// The last tick's run, wall time (reported only).
    pub tick_ms: Option<f32>,
    pub rebuilds: Option<u64>,
    /// Requests with a search under way.
    pub running: Option<usize>,
    /// The last tick's run: jobs stepped, expansions, replies finished.
    pub jobs: Option<u32>,
    pub nodes: Option<u32>,
    pub results: Option<u32>,
}

/// The last path of each actor, for `nav.draw paths`. Keyed by the actor's printed id (the AI's
/// key, a test's label): the nav layer doesn't know actor types. Debug state, never gameplay's.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct NavDebugPaths(pub BTreeMap<String, Vec<Location>>);

impl NavDebugPaths {
    /// Keeps `result`'s waypoints as `actor`'s path (replacing the last one).
    pub fn record(&mut self, actor: impl Into<String>, result: &PathResult) {
        self.0.insert(actor.into(), result.waypoints.clone());
    }
}

/// Which debug layers are on (all off by default).
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct NavDraw {
    pub tiles: bool,
    pub paths: Option<PathLayer>,
}

/// Whose recorded paths `nav.draw paths` shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathLayer {
    All,
    Actor(String),
}

/// Where gizmos go: drawn positions are meters from `origin` (the render origin: with a floating
/// origin, the position the camera's frame puts at zero), and only `worldspace` is drawn (`None`:
/// every worldspace, for single-space tests). The game updates both each frame.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct NavDrawFrame {
    pub worldspace: Option<WorldspaceId>,
    pub origin: WorldPos,
}

impl Default for NavDrawFrame {
    fn default() -> Self {
        Self {
            worldspace: None,
            origin: WorldPos::ORIGIN,
        }
    }
}

/// What the gizmos drew last frame (zero with the layers off or without feature `debug`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NavDrawn {
    pub tiles: usize,
    pub polys: usize,
    pub path_points: usize,
}

/// One agent class's resident tiles.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClassStats {
    pub class: u8,
    pub tiles: usize,
    pub polys: usize,
    pub bytes: usize,
}

/// `nav.stats` and the BRP answer.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NavStats {
    /// A [`NavMeshes`] backend is in the app.
    pub backend: bool,
    pub tiles: usize,
    pub polys: usize,
    pub bytes: usize,
    pub mib: f64,
    /// By class id, ascending.
    pub classes: Vec<ClassStats>,
    pub queue_depth: Option<usize>,
    pub tick_ms: Option<f32>,
    pub rebuilds: Option<u64>,
    pub running: Option<usize>,
    pub jobs: Option<u32>,
    pub nodes: Option<u32>,
    pub results: Option<u32>,
    /// The baked ring's counts, with [`NavRingPlugin`].
    pub ring: Option<RingStats>,
}

impl NavStats {
    pub fn of(world: &World) -> Self {
        let mut classes: BTreeMap<AgentClassId, ClassStats> = BTreeMap::new();
        let backend = world.get_resource::<NavMeshes>();
        for t in backend.map(|b| b.0.tile_summaries()).unwrap_or_default() {
            let c = classes.entry(t.key.class).or_insert(ClassStats {
                class: t.key.class.0,
                tiles: 0,
                polys: 0,
                bytes: 0,
            });
            c.tiles += 1;
            c.polys += t.polys;
            c.bytes += t.bytes;
        }
        let classes: Vec<ClassStats> = classes.into_values().collect();
        let bytes = classes.iter().map(|c| c.bytes).sum::<usize>();
        let counters = world
            .get_resource::<NavCounters>()
            .cloned()
            .unwrap_or_default();
        #[allow(
            clippy::cast_precision_loss,
            reason = "MiB for display; resident bytes stay far below 2^52"
        )]
        let mib = bytes as f64 / (1024.0 * 1024.0);
        Self {
            backend: backend.is_some(),
            tiles: classes.iter().map(|c| c.tiles).sum(),
            polys: classes.iter().map(|c| c.polys).sum(),
            bytes,
            mib,
            classes,
            queue_depth: counters.queue_depth,
            tick_ms: counters.tick_ms,
            rebuilds: counters.rebuilds,
            running: counters.running,
            jobs: counters.jobs,
            nodes: counters.nodes,
            results: counters.results,
            ring: world
                .get_resource::<BakedNavRing>()
                .map(BakedNavRing::stats),
        }
    }

    /// The `nav.stats` line: `nav: 6 tiles (class 0: 4, class 1: 2), 812 polys, 0.08 MiB,
    /// queue n/a, tick n/a ms, rebuilds n/a`.
    pub fn line(&self) -> String {
        if !self.backend {
            return "nav: no navmesh backend (0 tiles)".to_owned();
        }
        let per_class = self
            .classes
            .iter()
            .map(|c| format!("class {}: {}", c.class, c.tiles))
            .collect::<Vec<_>>()
            .join(", ");
        let per_class = if per_class.is_empty() {
            String::new()
        } else {
            format!(" ({per_class})")
        };
        let na = |v: Option<String>| v.unwrap_or_else(|| "n/a".to_owned());
        let ring = self.ring.as_ref().map_or_else(String::new, |r| {
            format!(
                "; ring: {} pending, {} empty, {} unbaked, {} failed, {} loaded, {} dropped, last {:.2} ms",
                r.pending, r.empty, r.unbaked, r.failed, r.loads, r.drops, r.last_ms
            )
        });
        let running = self
            .running
            .map_or_else(String::new, |r| format!(" ({r} running)"));
        let run = match (self.jobs, self.nodes, self.results) {
            (Some(j), Some(n), Some(r)) => format!(" ({j} jobs, {n} nodes, {r} results)"),
            _ => String::new(),
        };
        format!(
            "nav: {} tiles{per_class}, {} polys, {:.2} MiB, queue {}{running}, tick {} ms{run}, rebuilds {}{ring}",
            self.tiles,
            self.polys,
            self.mib,
            na(self.queue_depth.map(|q| q.to_string())),
            na(self.tick_ms.map(|t| format!("{t:.2}"))),
            na(self.rebuilds.map(|r| r.to_string())),
        )
    }
}

/// The [`BRP_STATS_METHOD`] answer: [`NavStats`] as JSON (no parameters). In the app that owns
/// BRP:
///
/// ```ignore
/// RemotePlugin::default().with_method_main(sf_nav_bevy::BRP_STATS_METHOD,
///     |In(p): In<Option<Value>>, w: &World| sf_nav_bevy::brp_stats_system(In(p), w)
///         .map_err(|message| BrpError { code: error_codes::INTERNAL_ERROR, message, data: None }))
/// ```
pub fn brp_stats_system(In(_params): In<Option<Value>>, world: &World) -> Result<Value, String> {
    serde_json::to_value(NavStats::of(world)).map_err(|e| format!("{BRP_STATS_METHOD}: {e}"))
}
