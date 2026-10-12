//! The streaming scheduler: pure, deterministic data.
//!
//! Each frame, [`Scheduler::update`] takes the anchors and decides each
//! streaming cell's target [`Level`] from the rings (load radius to rise,
//! unload radius to fall: hysteresis), measured from the anchor to the
//! cell's content AABB. A cell moves one level at a time; each move is a
//! [`Request`] that walks through its [`Stage`]s. Async stages (fetch,
//! assets) are started here within in-flight caps and completed by the
//! engine side; main-thread stages run through [`Scheduler::run_stage`]
//! within per-frame budgets, stopping at the first item that crosses the
//! budget. Same inputs, same outputs: no clocks, no hash-map order, explicit
//! tie-breaks everywhere.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use glam::{DVec3, IVec3, Vec3};
use sf_coords::WorldPos;

use crate::rings::{Level, RingConfig, RingRadii};
use crate::{CellDirectory, DoorLink, Location, RingOverrides, WorldspaceId};

/// Identifies an anchor across frames (for the engine's bookkeeping).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnchorId(pub u32);

/// What an anchor drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorRole {
    /// Drives R0 (simulation): the player, a scripted remote event.
    Gameplay,
    /// Drives R1 and R2 (what's seen): cameras. A free camera can roam
    /// without activating physics.
    Visual,
}

/// A streaming source.
#[derive(Clone, Debug, PartialEq)]
pub struct Anchor {
    pub id: AnchorId,
    pub role: AnchorRole,
    pub location: Location,
    /// Meters per second, for prefetch along travel.
    pub velocity: Vec3,
    /// Unit view direction; weights R1+ priority (cells behind count double).
    pub view_dir: Option<Vec3>,
    /// Multiplies this anchor's ring radii.
    pub radius_scale: f32,
}

impl Anchor {
    pub fn new(id: AnchorId, role: AnchorRole, location: Location) -> Self {
        Self { id, role, location, velocity: Vec3::ZERO, view_dir: None, radius_scale: 1.0 }
    }
}

/// One frame's input.
#[derive(Clone, Copy, Debug)]
pub struct TickInput<'a> {
    /// Seconds on any monotonic clock (only differences matter).
    pub now: f64,
    pub anchors: &'a [Anchor],
}

/// The steps of a request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    /// Async: index page read plus overlay (IO pool).
    Fetch,
    /// Async: models, LODs, textures.
    Assets,
    /// Main thread, time budget: entities spawn hidden.
    Spawn,
    /// Main thread, byte budget: GPU upload; the cell is revealed when done.
    Upload,
    /// Main thread, time budget: colliders, scripts, AI (R0 only).
    Activate,
    /// Main thread, time budget: dynamic state goes into the overlay.
    Deactivate,
    /// Main thread, time budget.
    Despawn,
}

impl Stage {
    pub fn is_async(self) -> bool {
        matches!(self, Self::Fetch | Self::Assets)
    }

    fn of(kind: RequestKind, to: Level) -> &'static [Stage] {
        match (kind, to) {
            (RequestKind::Load, Level::Active) => &[Self::Fetch, Self::Activate],
            (RequestKind::Load, _) => &[Self::Fetch, Self::Assets, Self::Spawn, Self::Upload],
            (RequestKind::Unload, Level::Near) => &[Self::Deactivate],
            (RequestKind::Unload, _) => &[Self::Despawn],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestKind {
    /// One level up: adds the difference.
    Load,
    /// One level down: removes the difference.
    Unload,
}

/// A cell moving one level. Fields are for reading; the scheduler owns them.
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub id: RequestId,
    pub kind: RequestKind,
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
    pub from: Level,
    pub to: Level,
    /// The cell's target level as of the last update (may be several
    /// levels beyond `to`).
    pub target: Level,
    pub stage: Stage,
    /// Async stage started, or main-thread stage partly done.
    pub running: bool,
    /// Items done in the current main-thread stage.
    pub progress: u32,
    /// 0 = R0 (gameplay), 1 = door prefetch, 2 = visual rings, 3 = kept
    /// only by a worldspace left recently (`SchedulerConfig::linger_secs`).
    pub class: u8,
    /// Effective distance, view-weighted for R1+; lower first.
    pub priority: f32,
    pub created: f64,
    pub fetch_started: Option<f64>,
    /// Asset loads this request holds in flight (reported by the fetch).
    pub assets: u32,
    /// Bytes it will upload (reported by the fetch).
    pub upload_bytes: u64,
    step: u8,
    cell_index: u32,
}

/// Per-frame budgets. Defaults are for the 120 fps target.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Budgets {
    pub spawn_ms: f64,
    pub activate_ms: f64,
    pub deactivate_ms: f64,
    pub despawn_ms: f64,
    pub upload_bytes: f64,
    pub fetches_in_flight: u32,
    pub assets_in_flight: u32,
    /// Multiplies every budget in burst mode.
    pub burst_factor: f64,
}

impl Default for Budgets {
    fn default() -> Self {
        Self {
            spawn_ms: 1.0,
            activate_ms: 0.5,
            deactivate_ms: 0.5,
            despawn_ms: 0.5,
            upload_bytes: 8.0 * 1024.0 * 1024.0,
            fetches_in_flight: 8,
            assets_in_flight: 64,
            burst_factor: 20.0,
        }
    }
}

impl Budgets {
    /// The defaults with time budgets scaled to a target frame time.
    pub fn for_frame_time(frame_ms: f64) -> Self {
        let s = frame_ms / (1000.0 / 120.0);
        let d = Self::default();
        Self {
            spawn_ms: d.spawn_ms * s,
            activate_ms: d.activate_ms * s,
            deactivate_ms: d.deactivate_ms * s,
            despawn_ms: d.despawn_ms * s,
            ..d
        }
    }

    /// The per-frame limit of a main-thread stage (ms or bytes).
    pub fn limit(&self, stage: Stage) -> f64 {
        match stage {
            Stage::Spawn => self.spawn_ms,
            Stage::Upload => self.upload_bytes,
            Stage::Activate => self.activate_ms,
            Stage::Deactivate => self.deactivate_ms,
            Stage::Despawn => self.despawn_ms,
            Stage::Fetch => f64::from(self.fetches_in_flight),
            Stage::Assets => f64::from(self.assets_in_flight),
        }
    }
}

/// Scheduler settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SchedulerConfig {
    pub rings: RingConfig,
    pub budgets: Budgets,
    /// A gameplay anchor this close to a link end prefetches the other side
    /// (load); it stops past `unload`.
    pub door_prefetch: RingRadii,
    /// Velocity lookahead is `clamp(2 × measured latency, min, max)` seconds.
    pub lookahead_min: f64,
    pub lookahead_max: f64,
    /// R1+ priority: `d × (1 + view_weight · (1 − cos θ))`.
    pub view_weight: f32,
    /// Load latency assumed before any is measured (seconds).
    pub initial_latency: f64,
    /// Measure the far ring (R2) to each cell's *page* (its 1 km index
    /// page, the far field's L2 node) instead of the cell, so all cells of
    /// a page arrive and leave together (far-field.md, "Residency").
    /// Priority still uses the cell's own distance.
    pub far_by_page: bool,
    /// Residency cache for the worldspace an anchor just left (a door, a
    /// teleport): its resident cells stay (nothing new loads) for this many
    /// seconds, and for as long as a gameplay anchor stands within
    /// `door_prefetch.unload` of a link end leading back, so a return trip
    /// finds them loaded (the cell cache of door-based engines). 0 turns
    /// it off.
    pub linger_secs: f64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            rings: RingConfig::default(),
            budgets: Budgets::default(),
            door_prefetch: RingRadii::new(24.0, 32.0),
            lookahead_min: 0.5,
            lookahead_max: 4.0,
            view_weight: 0.5,
            initial_latency: 0.25,
            far_by_page: false,
            linger_secs: 5.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldspaceState {
    /// No resident cells.
    Inactive,
    /// Resident around a link end, invisible and not simulated.
    Prefetched,
    /// The player's worldspace.
    Active,
}

/// A cell finished moving to a new level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellChange {
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
    pub from: Level,
    pub to: Level,
}

/// A load abandoned because its cell left the unload radius. The engine
/// undoes whatever its stage had done (`stage`, `running`, `progress`).
pub type Cancelled = Request;

/// What [`Scheduler::update`] decided.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TickReport {
    /// Async stages to start now (the request's `stage` says which).
    pub start: Vec<RequestId>,
    pub cancelled: Vec<Cancelled>,
    /// Anchors (or door prefetches) point into worldspaces without a
    /// directory: add them with [`Scheduler::add_worldspace`].
    pub missing_worldspaces: Vec<WorldspaceId>,
    /// Burst mode ended this tick (R0 and R1 resident).
    pub burst_ended: bool,
}

/// One item of main-thread work, as reported by the engine.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StageStep {
    /// Measured milliseconds (time stages) or bytes (upload).
    pub cost: f64,
    /// The request's stage is complete.
    pub done: bool,
}

/// What [`Scheduler::run_stage`] did this frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StageReport {
    pub items: u32,
    pub used: f64,
    pub limit: f64,
    pub completed: Vec<CellChange>,
}

// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Bounds {
    min: DVec3,
    max: DVec3,
}

impl Bounds {
    const EMPTY: Self = Self { min: DVec3::splat(f64::INFINITY), max: DVec3::splat(f64::NEG_INFINITY) };

    fn union(self, o: Self) -> Self {
        Self { min: self.min.min(o.min), max: self.max.max(o.max) }
    }

    fn closest(&self, p: DVec3) -> DVec3 {
        p.clamp(self.min, self.max)
    }

    fn distance(&self, p: DVec3) -> f64 {
        self.closest(p).distance(p)
    }
}

struct PageGeo {
    bounds: Bounds,
    first: u32,
    count: u32,
}

#[derive(Clone, Copy)]
struct CellState {
    level: Level,
    request: Option<RequestId>,
}

/// Per-cell wants gathered this tick (valid when `tick` matches).
#[derive(Clone, Copy)]
struct Want {
    tick: u64,
    /// Highest level whose load radius reaches the cell.
    load: Level,
    /// Highest level whose unload radius reaches it.
    keep: Level,
    class: u8,
    priority: f32,
}

impl Want {
    const NONE: Self = Self { tick: 0, load: Level::Unloaded, keep: Level::Unloaded, class: u8::MAX, priority: f32::INFINITY };
}

struct WorldStream {
    directory: CellDirectory,
    rings: RingConfig,
    cells: Vec<Bounds>,
    pages: Vec<PageGeo>,
    state: Vec<CellState>,
    scratch: Vec<Want>,
    visited: Vec<u32>,
    /// Cells resident or with a request.
    live: BTreeSet<u32>,
    prefetched: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Driver {
    Gameplay,
    Visual,
    Prefetch,
}

#[derive(Clone)]
struct EffAnchor {
    ws: WorldspaceId,
    pos: DVec3,
    pred: DVec3,
    view: Option<Vec3>,
    driver: Driver,
    scale: f32,
    /// A lingering anchor (`SchedulerConfig::linger_secs`): keeps what is
    /// resident, loads nothing.
    keep_only: bool,
}

/// Where an anchor was when it left a worldspace.
struct Ghost {
    id: AnchorId,
    anchor: EffAnchor,
    left: f64,
}

/// The scheduler. See the module docs.
pub struct Scheduler {
    config: SchedulerConfig,
    worlds: BTreeMap<WorldspaceId, WorldStream>,
    requests: BTreeMap<RequestId, Request>,
    next_request: u64,
    links: Vec<DoorLink>,
    /// (link index, leaving from b) currently prefetching.
    prefetching: BTreeSet<(usize, bool)>,
    /// Each anchor's last effective pose, to notice it changing worldspace.
    last_seen: BTreeMap<AnchorId, EffAnchor>,
    ghosts: Vec<Ghost>,
    active: Option<WorldspaceId>,
    burst: bool,
    latency: f64,
    tick: u64,
    now: f64,
}

impl Scheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            config,
            worlds: BTreeMap::new(),
            requests: BTreeMap::new(),
            next_request: 0,
            links: Vec::new(),
            prefetching: BTreeSet::new(),
            last_seen: BTreeMap::new(),
            ghosts: Vec::new(),
            active: None,
            burst: false,
            latency: config.initial_latency,
            tick: 0,
            now: 0.0,
        }
    }

    pub fn config(&self) -> &SchedulerConfig {
        &self.config
    }

    /// Makes a worldspace streamable. Returns false (and changes nothing)
    /// if it is already present.
    pub fn add_worldspace(&mut self, directory: CellDirectory, overrides: Option<&RingOverrides>) -> bool {
        if self.worlds.contains_key(&directory.worldspace) {
            return false;
        }
        let rings = overrides.map_or(self.config.rings, |o| self.config.rings.with_overrides(o));
        let cells: Vec<Bounds> = directory
            .cells
            .iter()
            .map(|c| {
                let origin = directory.partition.cell_origin(c.cell).to_f64();
                if c.bounds.is_empty() {
                    // No extent: stream by the cell's own box.
                    let edge = f64::from(directory.partition.cell_extent().unwrap_or(0.0));
                    Bounds { min: origin, max: origin + DVec3::splat(edge) }
                } else {
                    Bounds { min: origin + c.bounds.min.as_dvec3(), max: origin + c.bounds.max.as_dvec3() }
                }
            })
            .collect();
        let pages = directory
            .pages
            .iter()
            .map(|p| PageGeo {
                bounds: cells[p.first as usize..(p.first + p.count) as usize].iter().fold(Bounds::EMPTY, |a, b| a.union(*b)),
                first: p.first,
                count: p.count,
            })
            .collect();
        let n = cells.len();
        let world = WorldStream {
            rings,
            cells,
            pages,
            state: vec![CellState { level: Level::Unloaded, request: None }; n],
            scratch: vec![Want::NONE; n],
            visited: Vec::new(),
            live: BTreeSet::new(),
            prefetched: false,
            directory,
        };
        self.worlds.insert(world.directory.worldspace.clone(), world);
        true
    }

    /// Forgets a worldspace with nothing resident or pending. Returns false
    /// if it still has live cells.
    pub fn remove_worldspace(&mut self, ws: &WorldspaceId) -> bool {
        match self.worlds.get(ws) {
            Some(w) if w.live.is_empty() => {
                self.worlds.remove(ws);
                true
            }
            Some(_) => false,
            None => true,
        }
    }

    pub fn directory(&self, ws: &WorldspaceId) -> Option<&CellDirectory> {
        self.worlds.get(ws).map(|w| &w.directory)
    }

    /// The door links used for prefetch.
    pub fn set_links(&mut self, links: Vec<DoorLink>) {
        self.links = links;
        self.prefetching.clear();
    }

    /// The player's worldspace.
    pub fn set_active_worldspace(&mut self, ws: Option<WorldspaceId>) {
        self.active = ws;
    }

    /// Budgets ×`burst_factor` until R0 and R1 are resident (behind a fade:
    /// teleport, fast travel, loading a save).
    pub fn begin_burst(&mut self) {
        self.burst = true;
    }

    pub fn in_burst(&self) -> bool {
        self.burst
    }

    /// Current velocity lookahead, seconds.
    pub fn lookahead(&self) -> f64 {
        (2.0 * self.latency).clamp(self.config.lookahead_min, self.config.lookahead_max)
    }

    /// Measured load latency (EMA), seconds.
    pub fn latency(&self) -> f64 {
        self.latency
    }

    pub fn level(&self, ws: &WorldspaceId, cell: IVec3) -> Level {
        self.worlds
            .get(ws)
            .and_then(|w| w.directory.find(cell).map(|i| w.state[i].level))
            .unwrap_or(Level::Unloaded)
    }

    /// Whether every streaming cell touched by the horizontal square of half
    /// size `margin` around `pos` is at `level` or above in `ws`: the
    /// processing-range test (streaming.md "Processing range"). An actor
    /// simulates only where this holds for [`Level::Active`], so it never
    /// moves without the colliders R0 activation builds around it. Cells
    /// with no content (not in the directory) count as unloaded.
    pub fn covers(&self, ws: &WorldspaceId, pos: WorldPos, margin: f32, level: Level) -> bool {
        let Some(dir) = self.directory(ws) else { return false };
        let m = margin.max(0.0);
        let mut seen = [IVec3::MAX; 4];
        for (i, (dx, dz)) in [(-m, -m), (m, -m), (-m, m), (m, m)].into_iter().enumerate() {
            let cell = dir.partition.cell_of(pos.offset(Vec3::new(dx, 0.0, dz)));
            if seen[..i].contains(&cell) {
                continue;
            }
            seen[i] = cell;
            if self.level(ws, cell) < level {
                return false;
            }
        }
        true
    }

    /// Resident cells of a worldspace with their levels, in index order.
    pub fn resident(&self, ws: &WorldspaceId) -> impl Iterator<Item = (IVec3, Level)> + '_ {
        self.worlds.get(ws).into_iter().flat_map(|w| {
            w.live.iter().filter_map(|&i| {
                let s = w.state[i as usize];
                (s.level != Level::Unloaded).then(|| (w.directory.cells[i as usize].cell, s.level))
            })
        })
    }

    pub fn worldspace_state(&self, ws: &WorldspaceId) -> WorldspaceState {
        if self.active.as_ref() == Some(ws) {
            return WorldspaceState::Active;
        }
        match self.worlds.get(ws) {
            Some(w) if w.prefetched || !w.live.is_empty() => WorldspaceState::Prefetched,
            _ => WorldspaceState::Inactive,
        }
    }

    pub fn request(&self, id: RequestId) -> Option<&Request> {
        self.requests.get(&id)
    }

    /// Every pending request, oldest first.
    pub fn requests(&self) -> impl Iterator<Item = &Request> {
        self.requests.values()
    }

    /// (waiting, running) requests in a stage: queue depth for diagnostics.
    pub fn queue_depth(&self, stage: Stage) -> (usize, usize) {
        let mut out = (0, 0);
        for r in self.requests.values().filter(|r| r.stage == stage) {
            if r.running {
                out.1 += 1;
            } else {
                out.0 += 1;
            }
        }
        out
    }

    /// Age of the oldest pending request, seconds.
    pub fn oldest_request_age(&self, now: f64) -> f64 {
        self.requests.values().map(|r| now - r.created).fold(0.0, f64::max)
    }

    // -- the tick -----------------------------------------------------------

    /// Decides targets, creates and cancels requests, and starts async
    /// stages within the in-flight caps.
    pub fn update(&mut self, input: &TickInput<'_>) -> TickReport {
        self.tick += 1;
        self.now = input.now;
        let mut report = TickReport::default();
        let lookahead = self.lookahead();

        let mut anchors: Vec<EffAnchor> = input
            .anchors
            .iter()
            .map(|a| {
                let pos = a.location.pos.to_f64();
                EffAnchor {
                    ws: a.location.worldspace.clone(),
                    pos,
                    pred: pos + a.velocity.as_dvec3() * lookahead,
                    view: a.view_dir.map(Vec3::normalize_or_zero),
                    driver: match a.role {
                        AnchorRole::Gameplay => Driver::Gameplay,
                        AnchorRole::Visual => Driver::Visual,
                    },
                    scale: a.radius_scale,
                    keep_only: false,
                }
            })
            .collect();
        self.linger(input.anchors, &anchors);
        anchors.extend(self.ghosts.iter().map(|g| g.anchor.clone()));
        self.door_prefetch(input.anchors, &mut anchors);

        for w in self.worlds.values_mut() {
            w.visited.clear();
            w.prefetched = false;
        }
        for a in &anchors {
            let Some(w) = self.worlds.get_mut(&a.ws) else {
                if !report.missing_worldspaces.contains(&a.ws) {
                    report.missing_worldspaces.push(a.ws.clone());
                }
                continue;
            };
            w.prefetched |= a.driver == Driver::Prefetch || a.keep_only;
            gather(w, a, self.tick, self.config.view_weight, self.config.far_by_page);
        }

        for (ws, w) in &mut self.worlds {
            decide(ws, w, self.tick, input.now, &mut self.requests, &mut self.next_request, &mut report);
        }

        self.start_async(input.now, &mut report);

        // Burst mode waits for the active worldspace (every worldspace when
        // none is set), not for loads a lingering one still finishes.
        let active = self.active.as_ref();
        if self.burst
            && !self.requests.values().any(|r| {
                r.kind == RequestKind::Load && r.target >= Level::Near && active.is_none_or(|a| *a == r.worldspace)
            })
        {
            self.burst = false;
            report.burst_ended = true;
        }
        report
    }

    /// Residency cache (`SchedulerConfig::linger_secs`): an anchor that
    /// changed worldspace leaves a keep-only ghost where it was. A ghost
    /// ends when its anchor comes back, or once it is older than
    /// `linger_secs` and no gameplay anchor stands near a link end leading
    /// back to its worldspace.
    fn linger(&mut self, input: &[Anchor], anchors: &[EffAnchor]) {
        if self.config.linger_secs <= 0.0 {
            self.ghosts.clear();
            self.last_seen.clear();
            return;
        }
        for (a, eff) in input.iter().zip(anchors) {
            // Back in a worldspace it left: the anchor itself takes over.
            self.ghosts.retain(|g| !(g.id == a.id && g.anchor.ws == eff.ws));
            if let Some(prev) = self.last_seen.insert(a.id, eff.clone())
                && prev.ws != eff.ws
            {
                self.ghosts.retain(|g| !(g.id == a.id && g.anchor.ws == prev.ws));
                let anchor = EffAnchor { pred: prev.pos, keep_only: true, ..prev };
                self.ghosts.push(Ghost { id: a.id, anchor, left: self.now });
            }
        }
        let live: BTreeSet<AnchorId> = input.iter().map(|a| a.id).collect();
        self.last_seen.retain(|id, _| live.contains(id));
        let radius = f64::from(self.config.door_prefetch.unload);
        let (now, secs, links) = (self.now, self.config.linger_secs, &self.links);
        let near_way_back = |ws: &WorldspaceId| {
            links.iter().flat_map(DoorLink::exits).any(|(from, to)| {
                to.worldspace == *ws
                    && input.iter().any(|a| {
                        a.role == AnchorRole::Gameplay
                            && a.location.worldspace == from.worldspace
                            && a.location.pos.delta(from.frame.pos).length() <= radius
                    })
            })
        };
        self.ghosts.retain(|g| now - g.left < secs || near_way_back(&g.anchor.ws));
    }

    /// Door prefetch: a gameplay anchor near a link end adds a prefetch
    /// anchor at the other end's arrival point (R0 radius, priority just
    /// below R0).
    fn door_prefetch(&mut self, input: &[Anchor], out: &mut Vec<EffAnchor>) {
        let radii = self.config.door_prefetch;
        let mut now_prefetching = BTreeSet::new();
        for (li, link) in self.links.iter().enumerate() {
            for (exit, (from, to)) in link.exits().enumerate() {
                let key = (li, exit == 1);
                let was = self.prefetching.contains(&key);
                let near = input.iter().filter(|a| a.role == AnchorRole::Gameplay && a.location.worldspace == from.worldspace).any(|a| {
                    let d = a.location.pos.delta(from.frame.pos).length();
                    d <= f64::from(radii.load) || (was && d <= f64::from(radii.unload))
                });
                if near && now_prefetching.insert(key) {
                    let pos = to.arrival.pos.to_f64();
                    out.push(EffAnchor {
                        ws: to.worldspace.clone(),
                        pos,
                        pred: pos,
                        view: None,
                        driver: Driver::Prefetch,
                        scale: 1.0,
                        keep_only: false,
                    });
                }
            }
        }
        self.prefetching = now_prefetching;
    }

    fn start_async(&mut self, now: f64, report: &mut TickReport) {
        let burst = if self.burst { self.config.budgets.burst_factor } else { 1.0 };
        let b = &self.config.budgets;
        let mut fetching = 0.0;
        let mut assets = 0.0;
        let mut waiting: Vec<(&Request, Stage)> = Vec::new();
        for r in self.requests.values() {
            match (r.stage, r.running) {
                (Stage::Fetch, true) => fetching += 1.0,
                (Stage::Assets, true) => assets += f64::from(r.assets.max(1)),
                (s, false) if s.is_async() => waiting.push((r, s)),
                _ => {}
            }
        }
        waiting.sort_by(|(a, _), (b, _)| load_order(a, b));
        let fetch_cap = f64::from(b.fetches_in_flight) * burst;
        let asset_cap = f64::from(b.assets_in_flight) * burst;
        let mut started = Vec::new();
        for (r, stage) in waiting {
            // Stop each stage at the first item over its cap.
            let ok = match stage {
                Stage::Fetch if fetching < fetch_cap => {
                    fetching += 1.0;
                    true
                }
                Stage::Assets if assets < asset_cap => {
                    assets += f64::from(r.assets.max(1));
                    true
                }
                _ => false,
            };
            if ok {
                started.push(r.id);
            }
        }
        for id in started {
            let r = self.requests.get_mut(&id).expect("request exists");
            r.running = true;
            if r.stage == Stage::Fetch {
                r.fetch_started = Some(now);
            }
            report.start.push(id);
        }
    }

    // -- engine callbacks ---------------------------------------------------

    /// The fetch of `id` finished: the cell's page is decoded and the
    /// overlay applied. `assets` and `upload_bytes` describe what the next
    /// stages will do. Returns false for unknown (e.g. cancelled) requests.
    pub fn fetched(&mut self, id: RequestId, now: f64, assets: u32, upload_bytes: u64) -> bool {
        let Some(r) = self.requests.get_mut(&id) else { return false };
        if r.stage != Stage::Fetch || !r.running {
            return false;
        }
        r.assets = assets;
        r.upload_bytes = upload_bytes;
        if r.to == Level::Active {
            self.sample_latency(id, now);
        }
        self.advance(id);
        true
    }

    /// The assets of `id` are loaded.
    pub fn assets_loaded(&mut self, id: RequestId, now: f64) -> bool {
        let Some(r) = self.requests.get(&id) else { return false };
        if r.stage != Stage::Assets || !r.running {
            return false;
        }
        self.sample_latency(id, now);
        self.advance(id);
        true
    }

    fn sample_latency(&mut self, id: RequestId, now: f64) {
        if let Some(start) = self.requests[&id].fetch_started {
            self.latency += 0.2 * ((now - start).max(0.0) - self.latency);
        }
    }

    /// Runs one main-thread stage for this frame. `work` performs one item
    /// of the given request (spawn a few placements, upload one mesh, …)
    /// and reports its cost. Requests go in priority order (loads) or
    /// oldest first (unloads). The stage stops at the first item that
    /// crosses the budget, so it overshoots by at most one item.
    pub fn run_stage(&mut self, stage: Stage, work: &mut dyn FnMut(&Request) -> StageStep) -> StageReport {
        assert!(!stage.is_async(), "{stage:?} is async: complete it with fetched/assets_loaded");
        let burst = if self.burst { self.config.budgets.burst_factor } else { 1.0 };
        let limit = self.config.budgets.limit(stage) * burst;
        let mut queue: Vec<&Request> = self.requests.values().filter(|r| r.stage == stage).collect();
        queue.sort_by(|a, b| load_order(a, b));
        let queue: Vec<RequestId> = queue.into_iter().map(|r| r.id).collect();

        let mut report = StageReport { limit, ..StageReport::default() };
        'requests: for id in queue {
            loop {
                if report.used >= limit {
                    break 'requests;
                }
                let step = work(&self.requests[&id]);
                report.used += step.cost.max(0.0);
                report.items += 1;
                let r = self.requests.get_mut(&id).expect("request exists");
                r.running = true;
                r.progress += 1;
                if step.done {
                    if let Some(change) = self.advance(id) {
                        report.completed.push(change);
                    }
                    break;
                }
            }
        }
        report
    }

    /// Moves a request to its next stage, finishing it after the last one.
    fn advance(&mut self, id: RequestId) -> Option<CellChange> {
        let r = self.requests.get_mut(&id).expect("request exists");
        let stages = Stage::of(r.kind, r.to);
        r.step += 1;
        r.progress = 0;
        r.running = false;
        if let Some(&next) = stages.get(usize::from(r.step)) {
            r.stage = next;
            return None;
        }
        let r = self.requests.remove(&id).expect("request exists");
        let w = self.worlds.get_mut(&r.worldspace).expect("requests belong to known worldspaces");
        let i = r.cell_index as usize;
        w.state[i] = CellState { level: r.to, request: None };
        if r.to == Level::Unloaded {
            w.live.remove(&r.cell_index);
        }
        Some(CellChange { worldspace: r.worldspace, cell: r.cell, from: r.from, to: r.to })
    }
}

/// Loads by (class, priority, id); unloads by id (oldest first), after loads
/// never mix in one stage anyway.
fn load_order(a: &Request, b: &Request) -> Ordering {
    match (a.kind, b.kind) {
        (RequestKind::Load, RequestKind::Load) => {
            a.class.cmp(&b.class).then(a.priority.total_cmp(&b.priority)).then(a.id.cmp(&b.id))
        }
        _ => a.id.cmp(&b.id),
    }
}

/// Priority class of cells only a lingering anchor keeps.
const LINGER_CLASS: u8 = 3;

/// Records what one anchor wants of the cells in its reach.
#[allow(clippy::cast_possible_truncation)] // f64 distances narrowed for priority
fn gather(world: &mut WorldStream, anchor: &EffAnchor, tick: u64, view_weight: f32, far_by_page: bool) {
    let rings = &world.rings;
    let scale = anchor.scale;
    // (level, radii, priority class) of each ring this anchor drives.
    let driven: &[(Level, RingRadii, u8)] = match anchor.driver {
        Driver::Gameplay => &[(Level::Active, rings.active.scaled(scale), 0)],
        Driver::Prefetch => &[(Level::Active, rings.active, 1)],
        Driver::Visual => &[(Level::Near, rings.near.scaled(scale), 2), (Level::Far, rings.far.scaled(scale), 2)],
    };
    let reach = f64::from(driven.iter().map(|ring| ring.1.unload).fold(0.0, f32::max));
    let distance = |b: &Bounds| b.distance(anchor.pos).min(b.distance(anchor.pred));

    for page in &world.pages {
        let page_dist = distance(&page.bounds);
        if page_dist > reach {
            continue;
        }
        let page_dist = page_dist as f32;
        for ci in page.first..page.first + page.count {
            let bounds = &world.cells[ci as usize];
            let dist = distance(bounds);
            // A page-measured far ring reaches every cell of a page in reach.
            if dist > reach && !far_by_page {
                continue;
            }
            let dist = dist as f32;
            let want = &mut world.scratch[ci as usize];
            if want.tick != tick {
                *want = Want { tick, ..Want::NONE };
                world.visited.push(ci);
            }
            for &(level, radii, class) in driven {
                // Lingering cells rank after everything else.
                let class = if anchor.keep_only { LINGER_CLASS } else { class };
                let ring_dist = if far_by_page && level == Level::Far { page_dist } else { dist };
                if ring_dist <= radii.load && !anchor.keep_only {
                    want.load = want.load.max(level);
                }
                if ring_dist <= radii.unload {
                    want.keep = want.keep.max(level);
                    let prio = if level == Level::Active { dist } else { dist * view_factor(bounds, anchor, view_weight) };
                    if (class, prio) < (want.class, want.priority) {
                        want.class = class;
                        want.priority = prio;
                    }
                }
            }
        }
    }
}

/// `1 + w·(1 − cos θ)` between the view direction and the cell.
#[allow(clippy::cast_possible_truncation)]
fn view_factor(bounds: &Bounds, a: &EffAnchor, weight: f32) -> f32 {
    let Some(view) = a.view else { return 1.0 };
    let to = (bounds.closest(a.pos) - a.pos).as_vec3();
    let len = to.length();
    if len < 1e-3 || view == Vec3::ZERO {
        return 1.0;
    }
    1.0 + weight * (1.0 - view.dot(to / len))
}

/// Turns this tick's wants into request changes for one worldspace.
fn decide(
    ws: &WorldspaceId,
    w: &mut WorldStream,
    tick: u64,
    now: f64,
    requests: &mut BTreeMap<RequestId, Request>,
    next_request: &mut u64,
    report: &mut TickReport,
) {
    let mut order = std::mem::take(&mut w.visited);
    let visited = order.len();
    order.extend(w.live.iter().copied().filter(|&i| w.scratch[i as usize].tick != tick));

    for &ci in &order {
        let i = ci as usize;
        let want = if w.scratch[i].tick == tick { w.scratch[i] } else { Want::NONE };
        let (class, priority) = (want.class, want.priority);
        let state = w.state[i];
        // A load in flight counts as reached for hysteresis: it is only
        // cancelled once the cell leaves that level's unload radius.
        let committed = match state.request.map(|id| &requests[&id]) {
            Some(r) if r.kind == RequestKind::Load => r.to,
            _ => state.level,
        };
        let target = want.load.max(committed.min(want.keep));

        if let Some(id) = state.request {
            let r = requests.get_mut(&id).expect("cell request exists");
            if r.kind == RequestKind::Load && target < r.to {
                report.cancelled.push(requests.remove(&id).expect("exists"));
                w.state[i].request = None;
                if state.level == Level::Unloaded {
                    w.live.remove(&ci);
                }
            } else {
                r.target = target;
                if r.kind == RequestKind::Load {
                    r.class = class;
                    r.priority = priority;
                }
                continue;
            }
        }

        let (kind, to) = match target.cmp(&state.level) {
            Ordering::Greater => (RequestKind::Load, state.level.up().expect("below target")),
            Ordering::Less => (RequestKind::Unload, state.level.down().expect("above target")),
            Ordering::Equal => continue,
        };
        let id = RequestId(*next_request);
        *next_request += 1;
        requests.insert(
            id,
            Request {
                id,
                kind,
                worldspace: ws.clone(),
                cell: w.directory.cells[i].cell,
                from: state.level,
                to,
                target,
                stage: Stage::of(kind, to)[0],
                running: false,
                progress: 0,
                class,
                priority,
                created: now,
                fetch_started: None,
                assets: 0,
                upload_bytes: 0,
                step: 0,
                cell_index: ci,
            },
        );
        w.state[i].request = Some(id);
        w.live.insert(ci);
    }
    order.truncate(visited);
    w.visited = order;
}
