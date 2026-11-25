//! `AI9a` (tier allocator core): which actors of the simulated set run at tier 2 (full), tier 1
//! (reduced) or tier 0 (logical) this tick (`docs/specs/ai.md` "Tiers and budget").
//!
//! The `AnimBudget` / UE5 `MassLOD` pattern: significance from the distance to the **nearest player
//! of all**, visibility to any player and pins (combat, dialogue, scripted scenes); **fixed counts**
//! per CPU tier ([`CpuTier::caps`]), one global cap across regions, never adapted to measured time,
//! so traces and the tier-invariance test reproduce. Pure: inputs in, [`Allocation`] out, no clock
//! but the logical tick the caller passes.
//!
//! Per band (tier 2 first, then tier 1 among the rest):
//! - **Pins outrank caps**: every pinned actor is tier 2, even past the cap
//!   ([`AllocStats::over_cap_full`]), never a fighter demoted.
//! - **Hard caps between tier 2 and tier 1**: past the pins, tier 2 holds at most its cap; when
//!   over, the lowest-ranked actors drop to tier 1, visible ones included.
//! - **Demotion to tier 0 only out of view**: a visible actor already in tier 1 or 2 never drops to
//!   tier 0; it holds tier 1 ([`AllocStats::held_in_view`] counts the ones the plain ranking would
//!   have dropped). Holds take tier-1 slots before newcomers, so tier 1 only exceeds its cap through
//!   them ([`AllocStats::over_cap_reduced`]) and tier 2 only through pins, never through promotion.
//! - The free slots go by rank: visible before unseen, then the nearer, ties by [`ActorKey`].
//! - **Hysteresis**: an incumbent of the band (or of a higher band, for tier 1) ranks as if
//!   [`AllocConfig::full_margin_m`] (or `reduced_margin_m`) nearer, so a challenger must be that much
//!   nearer to take its slot and actors at the boundary don't flicker.
//!
//! **Staggered first update**: an actor entering tier 2 or 1 gets its first decide tick from
//! [`stagger_phase`] (`hash(ActorKey) % period`, the spec's "Scheduling" rule), so a batch of
//! promotions spreads over the decide period instead of deciding on one tick.
//!
//! **Cost** (`AI9c`): no maps. The inputs are sorted by key once (linear when they come in last
//! tick's order), merged with last tick's sorted tiers, ranked by one packed integer key per actor
//! and band, and each band is split with `select_nth_unstable` (O(n), no full sort); the buffers
//! live in the [`Allocator`] and are reused. Outputs come out in key order with no extra sort.
//!
//! The Bevy system, the `ai.alloc.*` tunables and `ai.stats` live in `sf_ai_bevy` (`AI9b`).


use serde::{Deserialize, Serialize};
use sf_ids::ActorKey;

use crate::sight::{seconds_to_ticks, stagger_phase};

/// An actor's simulation tier (`ai.md` "Tiers and budget"): how often and how precisely it is
/// stepped, never which rules apply (tier invariance).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tier {
    /// Tier 0: route progress on the world graph, decides on events only.
    #[default]
    Logical,
    /// Tier 1: ~1 Hz staggered decisions, corridor + mover without foot probes.
    Reduced,
    /// Tier 2: 5-10 Hz staggered decisions, full movement and perception.
    Full,
}

/// Why an actor must stay tier 2 whatever the cap (`ai.md`: combat, dialogue and quest pins).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Pin {
    Combat,
    Dialogue,
    /// A quest or scripted scene the actor plays in.
    Scripted,
}

/// The host's CPU tier (`sim.cpu_tier`: auto from cores, overridable on servers), apart from the
/// graphics preset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CpuTier {
    /// Steam Deck class (4c Zen 2).
    Floor,
    /// 6-core desktop, Steam Machine class.
    Medium,
    /// 8-core laptop (the reference machine).
    #[default]
    High,
    /// 12+ cores.
    Ultra,
    /// Headless server, no render thread (at 8 cores; measured per core later).
    Dedicated,
}

impl CpuTier {
    /// Every CPU tier, lowest first.
    pub const ALL: [CpuTier; 5] = [Self::Floor, Self::Medium, Self::High, Self::Ultra, Self::Dedicated];

    /// The fixed tier-2 / tier-1 caps of this CPU tier (`ai.md` table; set by `AIM`).
    pub fn caps(self) -> TierCaps {
        match self {
            // tunables: const (bound from ai.alloc.caps_floor in sf_ai_bevy; source: ai.md tier table, research §8)
            Self::Floor => TierCaps { full: 48, reduced: 160 },
            // tunables: const (bound from ai.alloc.caps_medium in sf_ai_bevy; source: ai.md tier table, provisional until AIM)
            Self::Medium => TierCaps { full: 80, reduced: 260 },
            // tunables: const (bound from ai.alloc.caps_high in sf_ai_bevy; source: ai.md tier table, provisional until AIM)
            Self::High => TierCaps { full: 128, reduced: 400 },
            // tunables: const (bound from ai.alloc.caps_ultra in sf_ai_bevy; source: ai.md tier table, provisional until AIM)
            Self::Ultra => TierCaps { full: 192, reduced: 600 },
            // tunables: const (bound from ai.alloc.caps_dedicated in sf_ai_bevy; source: ai.md tier table, provisional until AIM)
            Self::Dedicated => TierCaps { full: 256, reduced: 1000 },
        }
    }
}

/// Fixed actor counts per band: one global budget across regions and players.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TierCaps {
    /// Tier-2 slots (only pins may exceed it).
    pub full: usize,
    /// Tier-1 slots (in-view holds may exceed it).
    pub reduced: usize,
}

/// Allocator tunables (`ai.alloc.*`, bound in `sf_ai_bevy`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocConfig {
    /// Tier-2 hysteresis: an incumbent ranks this much nearer, meters (range 0-50).
    pub full_margin_m: f32,
    /// Tier-1 hysteresis: a tier-1 or tier-2 incumbent ranks this much nearer, meters (range 0-100).
    pub reduced_margin_m: f32,
    /// Tier-2 decide period, seconds (range 0.1-0.2: 5-10 Hz).
    pub full_decide_s: f32,
    /// Tier-1 decide period, seconds (range 0.5-2).
    pub reduced_decide_s: f32,
}

impl Default for AllocConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.alloc.full_margin_m in sf_ai_bevy; Guess: a few strides, MassLOD-style buffer)
            full_margin_m: 5.0,
            // tunables: const (bound from ai.alloc.reduced_margin_m in sf_ai_bevy; Guess: wider outer band, MassLOD-style buffer)
            reduced_margin_m: 10.0,
            // tunables: const (bound from ai.alloc.full_decide_s in sf_ai_bevy; source: ai.md tier 2 "5-10 Hz")
            full_decide_s: 0.15,
            // tunables: const (bound from ai.alloc.reduced_decide_s in sf_ai_bevy; source: ai.md tier 1 "~1 Hz")
            reduced_decide_s: 1.0,
        }
    }
}

impl AllocConfig {
    /// The decide period of `tier` in whole ticks (≥ 1), or `None` for tier 0 (events only).
    pub fn decide_period_ticks(&self, tier: Tier, tick_s: f32) -> Option<u32> {
        match tier {
            Tier::Logical => None,
            Tier::Reduced => Some(seconds_to_ticks(self.reduced_decide_s, tick_s)),
            Tier::Full => Some(seconds_to_ticks(self.full_decide_s, tick_s)),
        }
    }
}

/// One actor of the simulated set, as the shell measured it this tick.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocInput {
    pub actor: ActorKey,
    /// Distance to the nearest player of all, meters (non-finite = farthest; negative = 0).
    pub nearest_player_m: f32,
    /// Visible to any player (through an open seamless doorway too).
    pub visible: bool,
    pub pin: Option<Pin>,
}

/// An actor whose tier changed this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierChange {
    pub actor: ActorKey,
    /// The previous tier (tier 0 for an actor new to the set).
    pub from: Tier,
    pub to: Tier,
    /// The first tick after this one on which the actor decides in its new tier
    /// ([`stagger_phase`] of its decide period); `None` for tier 0.
    pub first_decide_tick: Option<u64>,
}

/// Counts for `ai.stats`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocStats {
    pub full: usize,
    pub reduced: usize,
    pub logical: usize,
    /// Pinned actors (all tier 2).
    pub pinned: usize,
    /// Tier-2 actors past the cap (pins only; `ai.stats` logs it).
    pub over_cap_full: usize,
    /// Tier-1 actors past the cap (in-view holds).
    pub over_cap_reduced: usize,
    /// Visible tier-1 or tier-2 incumbents kept in tier 1 that the plain ranking would have dropped
    /// to tier 0.
    pub held_in_view: usize,
}

/// One tick's allocation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    /// Every actor of the input and its tier, in [`ActorKey`] order.
    pub tiers: Vec<(ActorKey, Tier)>,
    /// The actors whose tier changed, in [`ActorKey`] order.
    pub changes: Vec<TierChange>,
    /// Actors allocated last tick but absent from this tick's input (left the set), in key order.
    pub removed: Vec<ActorKey>,
    pub stats: AllocStats,
}

/// The allocator's state: each known actor's current tier, plus per-call scratch reused across
/// ticks. Not saved (derived each tick).
#[derive(Clone, Debug, Default)]
pub struct Allocator {
    /// Each known actor's tier, sorted by key.
    current: Vec<(ActorKey, Tier)>,
    /// This call's cleaned inputs, sorted by key, deduplicated.
    entries: Vec<Entry>,
    /// Packed rank keys ([`rank_key`]) of one band's candidates.
    ranks: Vec<u128>,
    /// Second rank buffer (the tier-1 band).
    scratch: Vec<u128>,
}

/// Equal when they track the same tiers (the scratch buffers don't count).
impl PartialEq for Allocator {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

/// One input, cleaned, with the actor's tier before this tick and the one it gets.
#[derive(Clone, Copy, Debug)]
struct Entry {
    actor: ActorKey,
    dist: f32,
    visible: bool,
    pinned: bool,
    prev: Tier,
    to: Tier,
}

/// A finite, non-negative distance (non-finite ranks farthest).
fn clean_distance(d: f32) -> f32 {
    if d.is_nan() { f32::INFINITY } else { d.max(0.0) }
}

/// A finite, non-negative margin (else 0).
fn clean_margin(m: f32) -> f32 {
    if m.is_finite() { m.max(0.0) } else { 0.0 }
}

/// `x`'s bits mapped so that unsigned order is [`f32::total_cmp`] order.
fn total_order_bits(x: f32) -> u32 {
    let b = x.to_bits();
    if b >> 31 == 1 { !b } else { b | (1 << 31) }
}

// tunables: const (layout: low bits of a packed rank key hold the entry index)
const RANK_IDX_BITS: u32 = 64;

/// The rank of `e` (entry `idx`) within `band`, packed so that a smaller key ranks first: pins,
/// then visible, then nearer (an incumbent of the band or above `margin` nearer), then key (the
/// entries are in key order, so the index breaks ties as the key would).
fn rank_key(e: &Entry, idx: usize, band: Tier, margin: f32) -> u128 {
    let eff = if e.prev >= band { e.dist - margin } else { e.dist };
    let hi = (u64::from(!e.pinned) << 33) | (u64::from(!e.visible) << 32) | u64::from(total_order_bits(eff));
    (u128::from(hi) << RANK_IDX_BITS) | idx as u128
}

/// The entry index of a packed rank key.
#[allow(clippy::cast_possible_truncation)] // the low bits are a usize index by construction
fn rank_idx(r: u128) -> usize {
    r as u64 as usize
}

/// Moves the `n` best (smallest) keys of `v` to its front, in any order; O(len).
fn select_best(v: &mut [u128], n: usize) {
    if n > 0 && n < v.len() {
        v.select_nth_unstable(n);
    }
}

impl Allocator {
    pub fn new() -> Self {
        Self::default()
    }

    /// The actor's current tier (tier 0 when unknown).
    pub fn tier(&self, actor: ActorKey) -> Tier {
        self.current.binary_search_by_key(&actor, |&(k, _)| k).map_or_else(|_| Tier::default(), |i| self.current[i].1)
    }

    /// Number of actors the allocator tracks.
    pub fn len(&self) -> usize {
        self.current.len()
    }

    pub fn is_empty(&self) -> bool {
        self.current.is_empty()
    }

    /// Allocates this tick's tiers for the simulated set `inputs` (any order; a duplicated actor
    /// counts once, its most significant entry), under `caps`, and remembers them for the next
    /// tick's hysteresis. `tick` is the logical tick, `tick_s` its length (decide periods).
    ///
    /// Cost: one key sort of the inputs (linear when they arrive in the same order as last tick)
    /// and one linear selection per band, no map.
    pub fn allocate(
        &mut self,
        inputs: &[AllocInput],
        caps: TierCaps,
        config: &AllocConfig,
        tick: u64,
        tick_s: f32,
    ) -> Allocation {
        let Self { current, entries, ranks, scratch } = self;
        entries.clear();
        entries.extend(inputs.iter().map(|i| Entry {
            actor: i.actor,
            dist: clean_distance(i.nearest_player_m),
            visible: i.visible,
            pinned: i.pin.is_some(),
            prev: Tier::Logical,
            to: Tier::Logical,
        }));
        // Duplicates: key order, then most significant first, keep the first (order-independent:
        // duplicates of equal rank are interchangeable).
        entries.sort_unstable_by(|a, b| {
            a.actor.cmp(&b.actor).then_with(|| rank_key(a, 0, Tier::Full, 0.0).cmp(&rank_key(b, 0, Tier::Full, 0.0)))
        });
        entries.dedup_by_key(|e| e.actor);

        // Last tick's tiers (both lists in key order): `prev`, and the actors that left the set.
        let mut removed = Vec::new();
        let mut j = 0;
        for e in entries.iter_mut() {
            while j < current.len() && current[j].0 < e.actor {
                removed.push(current[j].0);
                j += 1;
            }
            if j < current.len() && current[j].0 == e.actor {
                e.prev = current[j].1;
                j += 1;
            }
        }
        removed.extend(current[j..].iter().map(|&(k, _)| k));

        let mut stats = AllocStats { pinned: entries.iter().filter(|e| e.pinned).count(), ..AllocStats::default() };

        // Tier 2: pins rank first, so the band is the best max(pins, cap) (a hard cap past the
        // pins: no in-view holds).
        let full_margin = clean_margin(config.full_margin_m);
        ranks.clear();
        ranks.extend(entries.iter().enumerate().map(|(i, e)| rank_key(e, i, Tier::Full, full_margin)));
        let n_full = caps.full.max(stats.pinned).min(ranks.len());
        select_best(ranks, n_full);
        for &r in &ranks[..n_full] {
            entries[rank_idx(r)].to = Tier::Full;
        }

        // Tier 1 among the rest: in-view holds (tier-2 drops included), then free slots by rank.
        let reduced_margin = clean_margin(config.reduced_margin_m);
        scratch.clear();
        scratch.extend(ranks[n_full..].iter().map(|&r| {
            let i = rank_idx(r);
            rank_key(&entries[i], i, Tier::Reduced, reduced_margin)
        }));
        stats.held_in_view = fill_reduced(entries, scratch, ranks, caps.reduced);

        let mut tiers = Vec::with_capacity(entries.len());
        let mut changes = Vec::new();
        for e in entries.iter() {
            let to = e.to;
            match to {
                Tier::Full => stats.full += 1,
                Tier::Reduced => stats.reduced += 1,
                Tier::Logical => stats.logical += 1,
            }
            tiers.push((e.actor, to));
            if to != e.prev {
                let first_decide_tick = config
                    .decide_period_ticks(to, tick_s)
                    .map(|period| first_decide_after(e.actor, tick, period));
                changes.push(TierChange { actor: e.actor, from: e.prev, to, first_decide_tick });
            }
        }
        stats.over_cap_full = stats.full.saturating_sub(caps.full);
        stats.over_cap_reduced = stats.reduced.saturating_sub(caps.reduced);

        current.clear();
        current.extend_from_slice(&tiers);
        Allocation { tiers, changes, removed, stats }
    }
}

/// Fills tier 1 with `free` slots from the candidates whose tier-1 rank keys are `cands` (no
/// pins, no tier-2 winners): visible incumbents (`prev >= Reduced`, tier-2 drops included) always
/// stay; the remaining slots go by rank. `spare` is a scratch buffer. Returns how many holds the
/// plain ranking (the best `free`) would have dropped.
fn fill_reduced(entries: &mut [Entry], cands: &mut [u128], spare: &mut Vec<u128>, free: usize) -> usize {
    let is_hold = |e: &Entry| e.visible && e.prev >= Tier::Reduced;
    if free >= cands.len() {
        for &r in cands.iter() {
            entries[rank_idx(r)].to = Tier::Reduced;
        }
        return 0;
    }
    select_best(cands, free);
    let (best, worse) = cands.split_at(free);
    let dropped = worse.iter().filter(|&&r| is_hold(&entries[rank_idx(r)])).count();
    if dropped == 0 {
        // Every hold is among the best `free`: the band is exactly those.
        for &r in best {
            entries[rank_idx(r)].to = Tier::Reduced;
        }
        return 0;
    }
    // Every hold stays; the slots left (if any) go to the best non-holds, all among `best`.
    let holds = dropped + best.iter().filter(|&&r| is_hold(&entries[rank_idx(r)])).count();
    spare.clear();
    for &r in cands.iter() {
        let e = &mut entries[rank_idx(r)];
        if is_hold(e) {
            e.to = Tier::Reduced;
        }
    }
    spare.extend(best.iter().copied().filter(|&r| !is_hold(&entries[rank_idx(r)])));
    let open = free.saturating_sub(holds);
    select_best(spare, open);
    for &r in &spare[..open] {
        entries[rank_idx(r)].to = Tier::Reduced;
    }
    dropped
}

/// The first tick after `tick` on which `actor` decides with a `period`-tick decide period
/// ([`stagger_phase`]).
pub fn first_decide_after(actor: ActorKey, tick: u64, period: u32) -> u64 {
    let phase = u64::from(stagger_phase(actor, period));
    let period = u64::from(period.max(1));
    let next = tick.saturating_add(1);
    let offset = (phase + period - next % period) % period;
    next.saturating_add(offset)
}
