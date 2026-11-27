//! Fire heat core (interaction-world-rules.md "Fire", IN17c1): burning objects heat flammable
//! neighbours, a target past its material's `ignite_heat` starts burning, a burning object runs out
//! of fuel and is charred, `Wet` takes no heat and rain puts fires out. Host only, pure and
//! deterministic: same bodies, states, wind and tick, same result, whatever the bodies' order.
//!
//! - **Heat.** Each tick a flammable target that is not burning, `Wet` or `Charred` gains, from every
//!   burning body whose material spreads within `interact.fire.spread_radius_m`,
//!   `heat_per_s x falloff(d) x wind(theta) x (1 + jitter x j) / tick_hz`, with `falloff(d) = 1 - d / r`
//!   (linear, 0 at the radius) and `j` in `[-1, 1)` from `splitmix64` over `(target, second)`, so a
//!   target's jitter changes once per second and is the same on every machine. With no source in reach
//!   it cools at `interact.fire.cool_per_s` (never below 0).
//! - **Wind.** `wind(theta) = max(0, 1 + wind_bias x cos(theta) x min(|w| / wind_ref_mps, 1))`, `w` the
//!   weather's mean horizontal wind `(x, z)` and `theta` its angle to the source -> target direction
//!   on the ground plane (a target straight above its source has `cos = 0`).
//! - **Ignition** at `heat >= ignite_heat`: `Burning` is added (until removed) and the heat cleared.
//!   Heat is computed for every target from the burning set at the start of the tick, then the
//!   ignitions apply, so a body lit this tick heats others from the next one.
//! - **Fuel.** A body burns from the tick it was first seen `Burning` (lit by heat, or by a rule: a
//!   torch hit) for `fuel_s`, then `Burning` is removed, `Charred` added and [`FireOutput::Burnt`]
//!   carries the material's `burnt_stage` (`None`: the host uses the last destruction stage). A
//!   charred body takes no more heat.
//! - **Rain.** A burning body whose surface wetness (the host's sample, 0 indoors) is at least
//!   `interact.fire.rain_douse` goes out: `Burning` removed, `Wet` added for `interact.rules.wet_ttl_s`.
//!   Adding `Wet` to dry targets from wetness is the host's (wetness causes, IN39).
//!
//! - **Budgets (`IN17c2`).** The ground plane is cut into `interact.fire.region_m` squares; a region
//!   holds at most `interact.fire.burning_max` burning bodies (rule-lit ones count too). A heat
//!   ignition past the cap queues; each tick the queue drains oldest first (then placement) into freed
//!   room before the tick's new ignitions, and a queued body takes no heat and leaves the queue once
//!   it is `Wet`, `Charred`, burning by a rule, or no longer sent.
//! - **Round-robin heat checks.** A check is one source -> target distance test; sources come from
//!   grid buckets of `spread_radius_m` (3 x 3 around the target). At most
//!   `interact.fire.spread_checks_per_tick` checks run per tick (always at least one target), targets
//!   in placement order from after the last one checked, wrapping. A target is brought up to date
//!   with the ticks elapsed since its last check, each source counting only the ticks it has heated
//!   (from the tick after a heat ignition, from the first tick seen for a rule-lit body), so the cap
//!   slows fire and never redirects it; with room for every check it is exactly the uncapped tick.
//!   A target with no source in its buckets and no heat costs nothing.
//! - **Settlements.** A body whose material is a `structure` and that the host marks `in_settlement`
//!   never takes heat or `Burning` from spread; it burns only through an authored stage or rule.
//!
//! Damage while burning is the rule's (`DestructionRules`, host).

use std::collections::BTreeMap;

use glam::{Vec2, Vec3};
use sf_world::PlacementId;

use crate::claims::Tick;
use crate::def::TagId;
use crate::destroy::splitmix64;
use crate::params::InteractParams;
use crate::rules::Flammable;
use crate::states::{ObjectStates, Ttl};

/// Burning: the body is on fire.
pub const BURNING: TagId = TagId::of("burning");
/// Wet: takes no heat.
pub const WET: TagId = TagId::of("wet");
/// Charred: burnt out, never burns again.
pub const CHARRED: TagId = TagId::of("charred");

/// One flammable body the host considers this tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FireBody {
    pub placement: PlacementId,
    /// World position, m (Y up).
    pub pos: Vec3,
    /// Its material's burn data.
    pub flammable: Flammable,
    /// Surface wetness at it (`sf_surface` scale, 0 indoors), ratio.
    pub wetness: f32,
    /// Inside a settlement zone (the adapter's): a `structure` there is immune to spread.
    pub in_settlement: bool,
}

/// The weather input of a fire tick.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FireEnv {
    /// Mean horizontal wind `(x, z)`, m/s (`sf_weather`, pure from seed and clock; gusts are presentation).
    pub wind: Vec2,
}

/// What a fire tick did to one body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FireOutput {
    /// Heat passed `ignite_heat`: `Burning` added.
    Ignited,
    /// Fuel ran out: `Burning` removed, `Charred` added; the stage to show.
    Burnt { stage: Option<u8> },
    /// Rain put it out: `Burning` removed, `Wet` added.
    Doused,
}

/// One output of [`step_fire`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FireEvent {
    pub placement: PlacementId,
    pub output: FireOutput,
}

/// The fire table: heat per target, the tick each burning body started burning (and heating), the
/// round-robin state and the ignition queue.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fire {
    heat: BTreeMap<PlacementId, f32>,
    since: BTreeMap<PlacementId, Tick>,
    /// The first tick each burning body heats others.
    heats_from: BTreeMap<PlacementId, Tick>,
    /// The tick each target's heat was last brought up to date.
    checked: BTreeMap<PlacementId, Tick>,
    /// Ignitions waiting for room in their region, with the tick they queued.
    queued: BTreeMap<PlacementId, Tick>,
    /// The last target the round-robin checked.
    cursor: Option<PlacementId>,
    /// Heat checks the last tick ran.
    checks: usize,
}

impl Fire {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Heat accumulated by `placement` (0 when none), ratio.
    #[must_use]
    pub fn heat(&self, placement: PlacementId) -> f32 {
        self.heat.get(&placement).copied().unwrap_or(0.0)
    }

    /// The tick `placement` started burning, while it burns.
    #[must_use]
    pub fn burning_since(&self, placement: PlacementId) -> Option<Tick> {
        self.since.get(&placement).copied()
    }

    /// The queued ignitions, oldest first, then by placement.
    #[must_use]
    pub fn queued(&self) -> Vec<PlacementId> {
        let mut q: Vec<(Tick, PlacementId)> = self.queued.iter().map(|(p, t)| (*t, *p)).collect();
        q.sort_unstable();
        q.into_iter().map(|(_, p)| p).collect()
    }

    /// Heat checks (source -> target distance tests) the last tick ran, count.
    #[must_use]
    pub fn checks(&self) -> usize {
        self.checks
    }
}

/// The fire budget region of `pos`: `interact.fire.region_m` squares on the ground plane.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn fire_region(pos: Vec3, params: &InteractParams) -> (i32, i32) {
    let side = params.fire_region_m.max(f32::EPSILON);
    ((pos.x / side).floor() as i32, (pos.z / side).floor() as i32)
}

/// Whether spread can never reach `b` (a `structure` in a settlement zone).
fn immune(b: &FireBody) -> bool {
    b.flammable.structure && b.in_settlement
}

/// The per-target jitter `j` in `[-1, 1)` for `second`: `h = 0; for w in [placement >> 64,
/// placement as u64, second] { h = splitmix64(h ^ w) }`, top 24 bits.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn heat_jitter(placement: PlacementId, second: u64) -> f32 {
    let mut h = 0u64;
    for w in [(placement.0 >> 64) as u64, placement.0 as u64, second] {
        h = splitmix64(h ^ w);
    }
    ((h >> 40) as f32 / (1u64 << 24) as f32).mul_add(2.0, -1.0)
}

/// The wind factor for heat from `from` to `to` (module docs).
#[must_use]
pub fn wind_factor(from: Vec3, to: Vec3, env: &FireEnv, params: &InteractParams) -> f32 {
    let dir = Vec2::new(to.x - from.x, to.z - from.z);
    let speed = env.wind.length();
    if speed <= f32::EPSILON || dir.length_squared() <= f32::EPSILON {
        return 1.0;
    }
    let cos = dir.normalize().dot(env.wind / speed);
    let strength = (speed / params.fire_wind_ref_mps.max(f32::EPSILON)).min(1.0);
    (params.fire_wind_bias * cos).mul_add(strength, 1.0).max(0.0)
}

/// One fire tick at `now` over `bodies` (module docs). Returns the tick's events: douses, then
/// burn-outs (each in placement order), then ignitions (from the queue oldest first, then the new ones
/// in placement order).
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines
)]
pub fn step_fire(
    fire: &mut Fire,
    states: &mut ObjectStates,
    bodies: &[FireBody],
    env: &FireEnv,
    now: Tick,
    params: &InteractParams,
) -> Vec<FireEvent> {
    let mut order: Vec<&FireBody> = bodies.iter().collect();
    order.sort_by_key(|b| b.placement);
    order.dedup_by_key(|b| b.placement);
    let find = |p: PlacementId| order.binary_search_by_key(&p, |b| b.placement).ok();

    // Forget bodies the host no longer sends, and burn starts of bodies no longer burning.
    fire.heat.retain(|p, _| find(*p).is_some());
    fire.checked.retain(|p, _| find(*p).is_some());
    fire.queued.retain(|p, _| find(*p).is_some());
    fire.since.retain(|p, _| states.has(*p, BURNING));
    fire.heats_from.retain(|p, _| states.has(*p, BURNING));

    let mut out = Vec::new();
    let mut burning: Vec<&FireBody> = Vec::new();
    let mut burnt = Vec::new();
    for b in &order {
        let p = b.placement;
        if !states.has(p, BURNING) {
            continue;
        }
        let since = *fire.since.entry(p).or_insert(now);
        fire.heats_from.entry(p).or_insert(now);
        if b.wetness >= params.fire_rain_douse {
            states.remove(p, BURNING);
            states.add(p, WET, Ttl::Secs(params.rules_wet_ttl_s), now, params);
            fire.since.remove(&p);
            fire.heats_from.remove(&p);
            out.push(FireEvent { placement: p, output: FireOutput::Doused });
        } else if now.0.saturating_sub(since.0) >= params.ticks(b.flammable.fuel_s) {
            burnt.push(*b);
        } else {
            burning.push(b);
        }
    }
    for b in burnt {
        let p = b.placement;
        states.remove(p, BURNING);
        states.add(p, CHARRED, Ttl::UntilRemoved, now, params);
        fire.since.remove(&p);
        fire.heats_from.remove(&p);
        out.push(FireEvent { placement: p, output: FireOutput::Burnt { stage: b.flammable.burnt_stage } });
    }

    // Burning bodies per region, and the spreaders bucketed by reach.
    let mut regions: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    for b in order.iter().filter(|b| states.has(b.placement, BURNING)) {
        *regions.entry(fire_region(b.pos, params)).or_default() += 1;
    }
    let radius = params.fire_spread_radius_m.max(f32::EPSILON);
    let cell = |pos: Vec3| ((pos.x / radius).floor() as i32, (pos.z / radius).floor() as i32);
    let mut grid: BTreeMap<(i32, i32), Vec<usize>> = BTreeMap::new();
    for (i, s) in burning.iter().enumerate().filter(|(_, s)| s.flammable.spread) {
        grid.entry(cell(s.pos)).or_default().push(i);
    }

    // Targets with work to do, in placement order: (body, candidate sources, cost in checks).
    let mut targets: Vec<(&FireBody, Vec<usize>, usize)> = Vec::new();
    for b in &order {
        let p = b.placement;
        if states.has(p, BURNING)
            || states.has(p, WET)
            || states.has(p, CHARRED)
            || immune(b)
            || fire.queued.contains_key(&p)
        {
            fire.heat.remove(&p);
            fire.checked.remove(&p);
            continue;
        }
        fire.checked.entry(p).or_insert(Tick(now.0.saturating_sub(1)));
        let (cx, cz) = cell(b.pos);
        let mut sources = Vec::new();
        for dz in -1..=1 {
            for dx in -1..=1 {
                if let Some(v) = grid.get(&(cx + dx, cz + dz)) {
                    sources.extend(v.iter().copied().filter(|&i| burning[i].placement != p));
                }
            }
        }
        sources.sort_unstable();
        if sources.is_empty() && fire.heat(p) <= 0.0 {
            fire.checked.insert(p, now);
            continue;
        }
        let cost = sources.len().max(1);
        targets.push((b, sources, cost));
    }

    // Round-robin from after the last target checked, within the check budget.
    let dt = 1.0 / params.tick_hz.max(f32::EPSILON);
    let second = (now.0 as f64 / f64::from(params.tick_hz.max(f32::EPSILON))) as u64;
    let budget = usize::from(params.fire_spread_checks_per_tick);
    let start = fire.cursor.map_or(0, |c| targets.partition_point(|(b, _, _)| b.placement <= c));
    let mut used = 0;
    let mut ignite = Vec::new();
    for k in 0..targets.len() {
        let (b, sources, cost) = &targets[(start + k) % targets.len()];
        if k > 0 && used + cost > budget {
            break;
        }
        used += cost;
        let p = b.placement;
        fire.cursor = Some(p);
        let last = fire.checked.insert(p, now).unwrap_or(Tick(now.0.saturating_sub(1)));
        let elapsed = now.0.saturating_sub(last.0);
        let mut gain = 0.0f32;
        let mut reached = false;
        for &i in sources {
            let s = burning[i];
            let d = s.pos.distance(b.pos);
            if d > radius {
                continue;
            }
            reached = true;
            let from = fire.heats_from.get(&s.placement).map_or(now.0, |t| t.0);
            let ticks = elapsed.min((now.0 + 1).saturating_sub(from));
            let falloff = (1.0 - d / radius).max(0.0);
            gain += params.fire_heat_per_s * falloff * wind_factor(s.pos, b.pos, env, params) * ticks as f32;
        }
        let heat = fire.heat(p);
        let next = if reached {
            let jitter = params.fire_jitter.mul_add(heat_jitter(p, second), 1.0);
            gain.mul_add(jitter * dt, heat)
        } else {
            params.fire_cool_per_s.mul_add(-(dt * elapsed as f32), heat).max(0.0)
        };
        if next >= b.flammable.ignite_heat {
            ignite.push(**b);
            fire.heat.remove(&p);
        } else if next > 0.0 {
            fire.heat.insert(p, next);
        } else {
            fire.heat.remove(&p);
        }
    }
    fire.checks = used;
    ignite.sort_by_key(|b| b.placement);

    // The queue first, oldest first, into freed room; then the new ignitions, or the queue.
    let mut lit = Vec::new();
    for p in fire.queued() {
        let Some(i) = find(p) else { continue };
        let b = order[i];
        if states.has(p, BURNING) || states.has(p, WET) || states.has(p, CHARRED) || immune(b) {
            fire.queued.remove(&p);
            continue;
        }
        let n = regions.entry(fire_region(b.pos, params)).or_default();
        if *n < usize::from(params.fire_burning_max) {
            *n += 1;
            fire.queued.remove(&p);
            lit.push(p);
        }
    }
    for b in ignite {
        let n = regions.entry(fire_region(b.pos, params)).or_default();
        if *n < usize::from(params.fire_burning_max) {
            *n += 1;
            lit.push(b.placement);
        } else {
            fire.queued.insert(b.placement, now);
        }
    }
    for p in lit {
        states.add(p, BURNING, Ttl::UntilRemoved, now, params);
        fire.since.insert(p, now);
        fire.heats_from.insert(p, Tick(now.0 + 1));
        out.push(FireEvent { placement: p, output: FireOutput::Ignited });
    }
    out
}
