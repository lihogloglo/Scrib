//! The tier allocator on the fixed tick (slice `AI9b`; `docs/specs/ai.md` "Tiers and budget"): the
//! `sf_ai::alloc` core ([`Allocator`]) over the simulated set, its `ai.alloc.*` / `sim.cpu_tier`
//! tunables bound into [`AiAllocSettings`], each actor's [`AiTier`] and the [`AiAllocStats`]
//! for `ai.stats`. Host only (tiers are host-authoritative, like sight).
//!
//! One system, `allocate_tiers`, in [`AiAllocSystems::Allocate`] (`FixedUpdate`, before
//! [`AiPerceptionSystems::Sight`] and `ControllerSystems::Drive`, so this tick's steps read this
//! tick's tiers):
//!
//! - **The simulated set**: every [`AiActor`] with a [`Mover`], [`Dormant`] ones skipped (as sight
//!   does). Its distance is to the nearest **player** of all: a [`Perceivable`] whose target is
//!   `PerceptTarget::Player` (its `Mover`, else its [`PerceivablePose`]). Plain distance: a
//!   [`WorldPos`] carries no worldspace, every spawned mover shares the active one (as the ground
//!   probe does). No player: every actor ranks farthest (visibility and pins still count).
//! - **Inputs the host writes** (no render dependency here): [`AiInView`] (visible to any player:
//!   the adapter decides, camera frustum or any player's view) and [`AiPin`] (combat, dialogue,
//!   scripted scene).
//! - **Output**: [`AiTier`] on every actor of the set (inserted on its first tick, updated from
//!   [`Allocation::changes`], removed when it leaves the set: despawned, dormant or without a
//!   mover), [`AiAllocStats`] (counts, resolved CPU tier, caps, tick, the allocate call's µs when
//!   timings are on).
//! - **CPU tier** (slice `AI9d`): `sim.cpu_tier` defaults to `auto`, which [`auto_cpu_tier`]
//!   resolves from the [`CpuHost`] (logical threads, detected once; `headless`, set by the host) to
//!   pick the `ai.alloc.caps_*` pair; an explicit tier wins. [`AiAllocSettings::cpu_tier`] resolves
//!   on every read, so a change to the tunable or the host applies on the next tick.
//!
//! - **Decide cadence** (slice `AI9g`): [`AiTier::decides_at`] says whether a tick is one of the
//!   actor's decide ticks (`sf_ai::decide_due` from its staggered `first_decide_tick` on the
//!   tier's `ai.alloc.*_decide_s` period); hosts read it through [`AiDecideGate`] and pass it as
//!   `sf_ai::ExecContext::decide`, so the HTN planner's periodic work runs at Full 0.15 s / Reduced
//!   1 s while operators and events run every tick. [`AiDecideStats`] counts the plans per tier
//!   over the last second (rolled here, on the allocator's tick) for the `ai.tiers` line.

use std::time::Instant;

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_time::{Fixed, Time};
use sf_ai::{
    ActorKey, AllocConfig, AllocInput, AllocStats, Allocator, CpuTier, PerceptTarget, Pin, Tier, TierCaps,
    decide_due, first_decide_after, seconds_to_ticks,
};
use sf_controller_bevy::{ControllerSystems, Dormant, Mover};
use sf_coords::WorldPos;
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Type, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::AiActor;
use crate::perception::{Perceivable, PerceivablePose};
use crate::sight::AiPerceptionSystems;

// --- Tunables ---------------------------------------------------------------------------------

/// Gameplay metadata shared by every allocator tunable (host-authoritative, dev scope, live).
const fn gp<T: Type>(d: Def<T>) -> Def<T> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

/// [`gp`] for a cap (a count of actors).
const fn cap(d: Def<u32>) -> Def<u32> {
    gp(d).unit(Unit::Count).range(0, 100_000)
}

/// The `sim.cpu_tier` choices: `auto` (the default, [`auto_cpu_tier`] from the [`CpuHost`]), then
/// the fixed tiers in [`CpuTier::ALL`] order.
pub const CPU_TIER_CHOICES: &[&str] = &["auto", "floor", "medium", "high", "ultra", "dedicated"];

pub const CPU_TIER: Def<&'static str> = gp(def!(choice "sim.cpu_tier" = "auto")
    .choices(CPU_TIER_CHOICES)
    .help("Host CPU tier, apart from the graphics preset: auto (from the logical thread count, dedicated when headless), floor (4-core handheld), medium (6 cores), high (8-core laptop, the reference), ultra (12+), dedicated (headless server); picks the ai.alloc.caps_* pair"));

/// [`auto_cpu_tier`]'s table: the highest logical-thread count of each tier below Ultra, from the
/// `ai.md` "Tiers and budget" CPU tier reference rows (SMT on, two threads per core):
///
/// | Logical threads | Tier | Reference row |
/// |---|---|---|
/// | ≤ 8 | Floor | Steam Deck (4 cores / 8 threads, Zen 2) |
/// | 9-12 | Medium | 6-core desktop (12 threads) |
/// | 13-16 | High | Ryzen 7 7435HS laptop (8 cores / 16 threads, the reference) |
/// | > 16 | Ultra | 12+ cores (24+ threads) |
///
/// Headless is Dedicated whatever the count. Threads, not physical cores: `std` gives only the
/// logical count (and honours affinity and container quotas), and no physical-core crate is a
/// workspace dependency. An SMT-off part ranks one row low (an 8-core without SMT reads as Floor);
/// an explicit `sim.cpu_tier` covers it.
// tunables: const (table of the spec's reference hardware; the override is sim.cpu_tier itself)
pub const AUTO_TIER_MAX_THREADS: [(usize, CpuTier); 3] =
    [(8, CpuTier::Floor), (12, CpuTier::Medium), (16, CpuTier::High)];

/// The CPU tier `sim.cpu_tier = auto` resolves to on a host with `logical_threads` threads
/// ([`AUTO_TIER_MAX_THREADS`]); `headless` (no render thread: a dedicated server) is Dedicated.
pub fn auto_cpu_tier(logical_threads: usize, headless: bool) -> CpuTier {
    if headless {
        return CpuTier::Dedicated;
    }
    AUTO_TIER_MAX_THREADS
        .iter()
        .find(|&&(max, _)| logical_threads <= max)
        .map_or(CpuTier::Ultra, |&(_, tier)| tier)
}

/// What `sim.cpu_tier = auto` reads: the host's logical threads (detected once, at
/// [`AiAllocSettings`]' creation) and whether it runs headless, which the host sets (a dedicated
/// server writes `AiAllocSettings::host.headless = true`; this crate has no window dependency to
/// detect it, and a headless test app is not a server).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuHost {
    pub logical_threads: usize,
    pub headless: bool,
}

impl CpuHost {
    /// This machine: [`std::thread::available_parallelism`] (1 when unknown, so Floor), windowed.
    pub fn detect() -> Self {
        Self { logical_threads: std::thread::available_parallelism().map_or(1, std::num::NonZero::get), headless: false }
    }
}

pub const CAPS_FLOOR_FULL: Def<u32> = cap(def!(count "ai.alloc.caps_floor_full" = 48)
    .help("Tier-2 (full) actor cap at sim.cpu_tier floor; only pins exceed it (ai.md tier table)"));
pub const CAPS_FLOOR_REDUCED: Def<u32> = cap(def!(count "ai.alloc.caps_floor_reduced" = 160)
    .help("Tier-1 (reduced) actor cap at sim.cpu_tier floor; only in-view holds exceed it (ai.md tier table)"));
pub const CAPS_MEDIUM_FULL: Def<u32> = cap(def!(count "ai.alloc.caps_medium_full" = 80)
    .help("Tier-2 (full) actor cap at sim.cpu_tier medium (provisional until AIM)"));
pub const CAPS_MEDIUM_REDUCED: Def<u32> = cap(def!(count "ai.alloc.caps_medium_reduced" = 260)
    .help("Tier-1 (reduced) actor cap at sim.cpu_tier medium (provisional until AIM)"));
pub const CAPS_HIGH_FULL: Def<u32> = cap(def!(count "ai.alloc.caps_high_full" = 128)
    .help("Tier-2 (full) actor cap at sim.cpu_tier high (provisional until AIM)"));
pub const CAPS_HIGH_REDUCED: Def<u32> = cap(def!(count "ai.alloc.caps_high_reduced" = 400)
    .help("Tier-1 (reduced) actor cap at sim.cpu_tier high (provisional until AIM)"));
pub const CAPS_ULTRA_FULL: Def<u32> = cap(def!(count "ai.alloc.caps_ultra_full" = 192)
    .help("Tier-2 (full) actor cap at sim.cpu_tier ultra (provisional until AIM)"));
pub const CAPS_ULTRA_REDUCED: Def<u32> = cap(def!(count "ai.alloc.caps_ultra_reduced" = 600)
    .help("Tier-1 (reduced) actor cap at sim.cpu_tier ultra (provisional until AIM)"));
pub const CAPS_DEDICATED_FULL: Def<u32> = cap(def!(count "ai.alloc.caps_dedicated_full" = 256)
    .help("Tier-2 (full) actor cap at sim.cpu_tier dedicated (provisional until AIM)"));
pub const CAPS_DEDICATED_REDUCED: Def<u32> = cap(def!(count "ai.alloc.caps_dedicated_reduced" = 1000)
    .help("Tier-1 (reduced) actor cap at sim.cpu_tier dedicated (provisional until AIM)"));

/// The (full, reduced) cap defs of each CPU tier, in [`CpuTier::ALL`] order.
pub const CAPS: [(&Def<u32>, &Def<u32>); 5] = [
    (&CAPS_FLOOR_FULL, &CAPS_FLOOR_REDUCED),
    (&CAPS_MEDIUM_FULL, &CAPS_MEDIUM_REDUCED),
    (&CAPS_HIGH_FULL, &CAPS_HIGH_REDUCED),
    (&CAPS_ULTRA_FULL, &CAPS_ULTRA_REDUCED),
    (&CAPS_DEDICATED_FULL, &CAPS_DEDICATED_REDUCED),
];

pub const FULL_MARGIN_M: Def<f32> = gp(def!(float "ai.alloc.full_margin_m" = 5.0)
    .range(0.0, 50.0)
    .unit(Unit::Meters)
    .help("Tier-2 hysteresis: an incumbent ranks this much nearer, so a challenger must beat it by this (MassLOD-style buffer)"));
pub const REDUCED_MARGIN_M: Def<f32> = gp(def!(float "ai.alloc.reduced_margin_m" = 10.0)
    .range(0.0, 100.0)
    .unit(Unit::Meters)
    .help("Tier-1 hysteresis: a tier-1 or tier-2 incumbent ranks this much nearer (wider outer band)"));
pub const FULL_DECIDE_S: Def<f32> = gp(def!(float "ai.alloc.full_decide_s" = 0.15)
    .range(0.1, 0.2)
    .unit(Unit::Seconds)
    .help("Tier-2 decide period, staggered per actor (ai.md: 5-10 Hz)"));
pub const REDUCED_DECIDE_S: Def<f32> = gp(def!(float "ai.alloc.reduced_decide_s" = 1.0)
    .range(0.5, 2.0)
    .unit(Unit::Seconds)
    .help("Tier-1 decide period, staggered per actor (ai.md: ~1 Hz)"));

/// The allocator tunables (`sim.cpu_tier`, `ai.alloc.*`), bound by [`add_alloc_tunables`].
pub const ALLOC_TUNABLES: &[&dyn AnyDef] = &[
    &CPU_TIER,
    &CAPS_FLOOR_FULL,
    &CAPS_FLOOR_REDUCED,
    &CAPS_MEDIUM_FULL,
    &CAPS_MEDIUM_REDUCED,
    &CAPS_HIGH_FULL,
    &CAPS_HIGH_REDUCED,
    &CAPS_ULTRA_FULL,
    &CAPS_ULTRA_REDUCED,
    &CAPS_DEDICATED_FULL,
    &CAPS_DEDICATED_REDUCED,
    &FULL_MARGIN_M,
    &REDUCED_MARGIN_M,
    &FULL_DECIDE_S,
    &REDUCED_DECIDE_S,
];

/// The fixed CPU tier named `name` (a [`CPU_TIER_CHOICES`] entry after `auto`, any case); `None`
/// for `auto` (and any other name).
pub fn cpu_tier_from_name(name: &str) -> Option<CpuTier> {
    CPU_TIER_CHOICES[1..].iter().position(|c| c.eq_ignore_ascii_case(name)).map(|i| CpuTier::ALL[i])
}

/// The index of `tier` in [`CpuTier::ALL`] (and [`CAPS`], [`AiAllocSettings::caps_table`]).
fn tier_index(tier: CpuTier) -> usize {
    CpuTier::ALL.iter().position(|&t| t == tier).unwrap_or_default()
}

/// A cap count as a slot count.
fn slots(v: u32) -> usize {
    usize::try_from(v).unwrap_or(usize::MAX)
}

/// The allocator's settings, bound from [`ALLOC_TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct AiAllocSettings {
    /// `sim.cpu_tier` as chosen: a fixed tier, or `None` for `auto` ([`Self::cpu_tier`] resolves it).
    pub cpu_tier_choice: Option<CpuTier>,
    /// What `auto` resolves from.
    pub host: CpuHost,
    /// The `ai.alloc.caps_*` pair of each CPU tier, in [`CpuTier::ALL`] order.
    pub caps_table: [TierCaps; 5],
    pub config: AllocConfig,
}

impl Default for AiAllocSettings {
    fn default() -> Self {
        Self {
            cpu_tier_choice: cpu_tier_from_name(CPU_TIER.default_value()),
            host: CpuHost::detect(),
            caps_table: CAPS.map(|(full, reduced)| TierCaps {
                full: slots(full.default_value()),
                reduced: slots(reduced.default_value()),
            }),
            config: AllocConfig {
                full_margin_m: FULL_MARGIN_M.default_value(),
                reduced_margin_m: REDUCED_MARGIN_M.default_value(),
                full_decide_s: FULL_DECIDE_S.default_value(),
                reduced_decide_s: REDUCED_DECIDE_S.default_value(),
            },
        }
    }
}

impl AiAllocSettings {
    /// The resolved CPU tier: the explicit `sim.cpu_tier`, else [`auto_cpu_tier`] from
    /// [`Self::host`] (so it follows both the tunable and a host's `headless` at once).
    pub fn cpu_tier(&self) -> CpuTier {
        self.cpu_tier_choice.unwrap_or_else(|| auto_cpu_tier(self.host.logical_threads, self.host.headless))
    }

    /// The caps of the resolved CPU tier.
    pub fn caps(&self) -> TierCaps {
        self.caps_table[tier_index(self.cpu_tier())]
    }

    /// Overrides the caps of the resolved CPU tier (tests, a server's command line goes through the
    /// tunables instead).
    pub fn set_caps(&mut self, caps: TierCaps) {
        self.caps_table[tier_index(self.cpu_tier())] = caps;
    }
}

macro_rules! bind {
    ($app:ident, $def:expr, |$s:ident, $v:ident| $body:expr) => {
        let _ = $app.add_tunable(&$def).bind_resource(|$s: &mut AiAllocSettings, $v| $body);
    };
}

/// Registers [`ALLOC_TUNABLES`] and binds them to [`AiAllocSettings`] (the caller inserts the
/// resource; [`AiAllocPlugin`] does).
pub fn add_alloc_tunables(app: &mut App) {
    // `auto` (and nothing else: the def only takes its choices) is `None`.
    bind!(app, CPU_TIER, |s, v| s.cpu_tier_choice = cpu_tier_from_name(v));
    for (i, (full, reduced)) in CAPS.into_iter().enumerate() {
        let _ = app.add_tunable(full).bind_resource(move |s: &mut AiAllocSettings, v| s.caps_table[i].full = slots(v));
        let _ = app
            .add_tunable(reduced)
            .bind_resource(move |s: &mut AiAllocSettings, v| s.caps_table[i].reduced = slots(v));
    }
    bind!(app, FULL_MARGIN_M, |s, v| s.config.full_margin_m = v);
    bind!(app, REDUCED_MARGIN_M, |s, v| s.config.reduced_margin_m = v);
    bind!(app, FULL_DECIDE_S, |s, v| s.config.full_decide_s = v);
    bind!(app, REDUCED_DECIDE_S, |s, v| s.config.reduced_decide_s = v);
}

// --- Components and resources -----------------------------------------------------------------

/// The actor's simulation tier, written by the allocator only (on every actor of the simulated
/// set). Absent: not in the set (dormant, no mover, or not allocated yet).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiTier {
    pub tier: Tier,
    /// The first tick (on [`AiAllocStats::tick`]'s clock) the actor decides in this tier, staggered
    /// over its decide period; `None` for tier 0 (events only).
    pub first_decide_tick: Option<u64>,
}

impl AiTier {
    /// Whether `tick` (on [`AiAllocStats::tick`]'s clock) is one of this actor's decide ticks:
    /// `sf_ai::decide_due` on its tier's period from `first_decide_tick` (so a tier change restarts
    /// the cadence at the new tier's first tick). Never for tier 0 (events only, no planner).
    pub fn decides_at(&self, tick: u64, config: &AllocConfig, tick_s: f32) -> bool {
        config
            .decide_period_ticks(self.tier, tick_s)
            .is_some_and(|period| self.first_decide_tick.is_none_or(|first| decide_due(first, period, tick)))
    }
}

/// Plans per tier in one counting window ([`AiDecideStats`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DecideCounts {
    /// Plans (any planning that produced an outcome) by actors at tier 2 / tier 1 / without a tier.
    pub full: u32,
    pub reduced: u32,
    pub untiered: u32,
    /// Of those, plans on a tick that was not the actor's decide tick (events).
    pub events: u32,
}

/// How often the planner ran per tier (`AI9g`): the counts of the last full second and of the
/// second running now, rolled by the allocator on its tick. Hosts add plans through
/// [`AiDecideGate::record_plan`]; `ai.tiers` prints [`Self::last_second`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiDecideStats {
    current: DecideCounts,
    last: DecideCounts,
    window_start: u64,
}

impl AiDecideStats {
    /// The counts of the last full one-second window.
    pub fn last_second(&self) -> DecideCounts {
        self.last
    }

    /// Adds one plan by an actor at `tier` (`None`: no tier yet); `on_decide_tick` false = an event.
    pub fn record_plan(&mut self, tier: Option<Tier>, on_decide_tick: bool) {
        let c = &mut self.current;
        match tier {
            Some(Tier::Full) => c.full += 1,
            Some(Tier::Reduced) => c.reduced += 1,
            // Tier 0 runs no planner; a host that plans for it anyway is counted with the untiered.
            Some(Tier::Logical) | None => c.untiered += 1,
        }
        c.events += u32::from(!on_decide_tick);
    }

    /// Closes the window once `window_ticks` have passed since it opened.
    fn roll(&mut self, tick: u64, window_ticks: u32) {
        if tick.saturating_sub(self.window_start) >= u64::from(window_ticks) {
            self.last = std::mem::take(&mut self.current);
            self.window_start = tick;
        }
    }
}

/// The decide gate for a host's planner system (`AI9g`): reads the allocator's tick and config and
/// counts plans into [`AiDecideStats`]. Without the allocator every tick is a decide tick.
#[derive(SystemParam)]
pub struct AiDecideGate<'w> {
    alloc: Option<Res<'w, AiAllocStats>>,
    settings: Option<Res<'w, AiAllocSettings>>,
    fixed: Option<Res<'w, Time<Fixed>>>,
    stats: Option<ResMut<'w, AiDecideStats>>,
}

impl AiDecideGate<'_> {
    /// Whether this tick is a decide tick of an actor at `tier` (`None`: not allocated yet, decides
    /// every tick until its first allocation).
    pub fn due(&self, tier: Option<&AiTier>) -> bool {
        let (Some(t), Some(alloc), Some(settings)) = (tier, self.alloc.as_deref(), self.settings.as_deref()) else {
            return true;
        };
        let tick_s = self.fixed.as_deref().map_or(Time::<Fixed>::default().timestep(), Time::<Fixed>::timestep).as_secs_f32();
        t.decides_at(alloc.tick, &settings.config, tick_s)
    }

    /// Counts one plan by an actor at `tier`, on its decide tick (`due`) or as an event.
    pub fn record_plan(&mut self, tier: Option<&AiTier>, due: bool) {
        if let Some(stats) = self.stats.as_deref_mut() {
            stats.record_plan(tier.map(|t| t.tier), due);
        }
    }
}

/// Marker the host writes on an actor visible to any player (camera frustum, any player's view,
/// an open seamless doorway: the adapter decides). A visible actor ranks first and never drops
/// to tier 0.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiInView;

/// Why the host keeps an actor at tier 2 whatever the cap (combat, dialogue, a scripted scene).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AiPin(pub Pin);

/// The last allocation, for `ai.stats`.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct AiAllocStats {
    pub stats: AllocStats,
    /// The resolved CPU tier it ran under ([`AiAllocSettings::cpu_tier`]).
    pub cpu_tier: CpuTier,
    /// The caps it ran under.
    pub caps: TierCaps,
    /// The allocator's logical tick: one per `FixedUpdate`, counted from 1 like `AiPerceptClock`.
    pub tick: u64,
    /// Microseconds of the last [`Allocator::allocate`] call (`None` with timings off).
    pub allocate_us: Option<f32>,
}

impl Default for AiAllocStats {
    fn default() -> Self {
        let (cpu_tier, stats) = (CpuTier::default(), AllocStats::default());
        // tunables: const (state: the tick counter starts before the first tick)
        Self { stats, cpu_tier, caps: cpu_tier.caps(), tick: 0, allocate_us: None }
    }
}

/// The allocator and its per-tick scratch.
#[derive(Resource, Debug, Default)]
struct AllocState {
    allocator: Allocator,
    timings: bool,
    inputs: Vec<AllocInput>,
    /// This tick's (key, entity), sorted by key.
    keys: Vec<(ActorKey, Entity)>,
    /// Last tick's, for the actors that left the set.
    prev_keys: Vec<(ActorKey, Entity)>,
    /// Actors of the set without an [`AiTier`] yet.
    missing: Vec<(ActorKey, Entity)>,
    players: Vec<WorldPos>,
}

/// Fixed-tick allocator system set.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AiAllocSystems {
    /// `allocate_tiers`, before [`AiPerceptionSystems::Sight`] and `ControllerSystems::Drive`.
    Allocate,
}

/// The allocator on the fixed tick: [`AiAllocSettings`] (bound to [`ALLOC_TUNABLES`]),
/// [`AiAllocStats`] and the system in [`AiAllocSystems::Allocate`]. [`crate::AiPlugin`] adds it.
#[derive(Clone, Copy, Debug)]
pub struct AiAllocPlugin {
    /// Whether [`AiAllocStats::allocate_us`] is measured.
    pub timings: bool,
}

impl Default for AiAllocPlugin {
    fn default() -> Self {
        // tunables: const (switch the host passes, like AiPlugin::timings)
        Self { timings: true }
    }
}

impl Plugin for AiAllocPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiAllocSettings>()
            .init_resource::<AiAllocStats>()
            .init_resource::<AiDecideStats>()
            .insert_resource(AllocState { timings: self.timings, ..AllocState::default() })
            .configure_sets(
                FixedUpdate,
                AiAllocSystems::Allocate.before(AiPerceptionSystems::Sight).before(ControllerSystems::Drive),
            )
            .add_systems(FixedUpdate, allocate_tiers.in_set(AiAllocSystems::Allocate));
        add_alloc_tunables(app);
    }
}

// --- System -----------------------------------------------------------------------------------

// tunables: const (maths: seconds to microseconds)
const MICROS_PER_S: f32 = 1e6;
// tunables: const (units: the AiDecideStats window is one second, a per-second rate)
const DECIDE_WINDOW_S: f32 = 1.0;

/// Distance from `feet` to the nearest of `players`, meters (infinite with no player).
#[allow(clippy::cast_possible_truncation)] // distances between loaded actors fit f32
fn nearest_player_m(feet: WorldPos, players: &[WorldPos]) -> f32 {
    players.iter().map(|&p| feet.delta(p).length()).fold(f64::INFINITY, f64::min) as f32
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity)]
fn allocate_tiers(
    fixed: Option<Res<Time<Fixed>>>,
    settings: Res<AiAllocSettings>,
    mut state: ResMut<AllocState>,
    mut out: ResMut<AiAllocStats>,
    decides: Option<ResMut<AiDecideStats>>,
    players: Query<(&Perceivable, Option<&Mover>, Option<&PerceivablePose>)>,
    mut actors: Query<(Entity, &AiActor, &Mover, Option<&AiPin>, Has<AiInView>, Option<&mut AiTier>), Without<Dormant>>,
    mut commands: Commands,
) {
    let tick_s = fixed.map_or(Time::<Fixed>::default().timestep(), |f| f.timestep()).as_secs_f32();
    out.tick += 1;
    let tick = out.tick;
    if let Some(mut d) = decides {
        d.roll(tick, seconds_to_ticks(DECIDE_WINDOW_S, tick_s));
    }
    let caps = settings.caps();
    let config = settings.config;
    let AllocState { allocator, timings, inputs, keys, prev_keys, missing, players: player_pos } = &mut *state;

    player_pos.clear();
    for (p, mover, pose) in &players {
        if !matches!(p.target, PerceptTarget::Player(_)) {
            continue;
        }
        if let Some(feet) = mover.map(|m| m.loco.position).or(pose.map(|q| q.feet)) {
            player_pos.push(feet);
        }
    }

    inputs.clear();
    std::mem::swap(keys, prev_keys);
    keys.clear();
    missing.clear();
    for (entity, actor, mover, pin, visible, tier) in &actors {
        inputs.push(AllocInput {
            actor: actor.0,
            nearest_player_m: nearest_player_m(mover.loco.position, player_pos),
            visible,
            pin: pin.map(|p| p.0),
        });
        keys.push((actor.0, entity));
        if tier.is_none() {
            missing.push((actor.0, entity));
        }
    }
    keys.sort_unstable();

    let started = timings.then(Instant::now);
    let alloc = allocator.allocate(inputs, caps, &config, tick, tick_s);
    out.allocate_us = started.map(|t| t.elapsed().as_secs_f32() * MICROS_PER_S);
    out.stats = alloc.stats;
    out.cpu_tier = settings.cpu_tier();
    out.caps = caps;

    // Changes on actors that carry a tier; the rest get theirs below.
    for c in &alloc.changes {
        let Ok(i) = keys.binary_search_by_key(&c.actor, |&(k, _)| k) else { continue };
        if let Ok((.., Some(mut t))) = actors.get_mut(keys[i].1) {
            *t = AiTier { tier: c.to, first_decide_tick: c.first_decide_tick };
        }
    }
    for &(key, entity) in missing.iter() {
        let tier = allocator.tier(key);
        let first_decide_tick = config.decide_period_ticks(tier, tick_s).map(|p| first_decide_after(key, tick, p));
        commands.entity(entity).insert(AiTier { tier, first_decide_tick });
    }
    // Left the set: drop the tier (the entity may be gone already).
    for key in &alloc.removed {
        let Ok(i) = prev_keys.binary_search_by_key(key, |&(k, _)| k) else { continue };
        if let Ok(mut e) = commands.get_entity(prev_keys[i].1) {
            e.try_remove::<AiTier>();
        }
    }
}
