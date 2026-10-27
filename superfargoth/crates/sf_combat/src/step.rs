//! The combat step (combat.md "The per-tick pipeline", items 4-8): after `sf_action::step` and the
//! mover, per fixed tick: vitals (start and hit-window costs, regen), weapon sweeps, contact
//! order, outcomes, reactions and hit-stop. The gate (item 1) is [`gate`], read into
//! `Sense::refused` before the action step.

use std::collections::HashMap;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_action::{ActionOutput, ActionState, CharacterId, ClassSet, ClockTime, Forced, Mode, RATE_ONE, SlotKey, TICK_HZ, TagId, Tick};
use sf_anim::SocketTrack;
use smallvec::SmallVec;

use crate::fighter::{FightState, Fighter, SwingState};
use crate::hitstop::HitStop;
use crate::react::{Side, Tier};
use crate::rules::{CombatRules, Contact, DefenderState, RegenState, Swing};
use crate::shape::{HurtShape, SWEEP_STEPS, WeaponShape};
use crate::sheet::{HitRow, MoveSpec, Movesets};

/// Baked socket tracks by slot (`sf_anim::sample_socket_track` at load).
pub trait SocketSource {
    fn socket(&self, slot: SlotKey) -> Option<&SocketTrack>;

    /// The socket's furthest horizontal distance from the root over the slot's clip, m: the sweep's
    /// reach prefilter. `None` (the default) tests every defender.
    fn reach(&self, _slot: SlotKey) -> Option<f32> {
        None
    }
}

impl<S: std::hash::BuildHasher> SocketSource for HashMap<SlotKey, SocketTrack, S> {
    fn socket(&self, slot: SlotKey) -> Option<&SocketTrack> {
        self.get(&slot)
    }
}

/// Socket tracks with their reach bound, computed once when a track is added (at load).
#[derive(Clone, Debug, Default)]
pub struct SocketTracks {
    tracks: HashMap<SlotKey, (SocketTrack, f32)>,
}

impl SocketTracks {
    pub fn insert(&mut self, slot: SlotKey, track: SocketTrack) {
        let reach = track.frames.iter().map(|f| Vec3::new(f.translation.x, 0.0, f.translation.z).length()).fold(0.0, f32::max);
        self.tracks.insert(slot, (track, reach));
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
}

impl SocketSource for SocketTracks {
    fn socket(&self, slot: SlotKey) -> Option<&SocketTrack> {
        self.tracks.get(&slot).map(|(t, _)| t)
    }

    fn reach(&self, slot: SlotKey) -> Option<f32> {
        self.tracks.get(&slot).map(|&(_, r)| r)
    }
}

/// How far a move's blade can reach from the attacker's root this tick (horizontal, m): the
/// furthest socket over the move's slots, the blade's length and radius, and the root's travel
/// this tick. `None` when a slot has no bound.
fn reach_bound(sockets: &dyn SocketSource, spec: &MoveSpec, w: WeaponShape, body: &Body) -> Option<f32> {
    let mut socket = 0.0f32;
    for s in &spec.slots {
        socket = socket.max(sockets.reach(s.key)?);
    }
    Some(socket + w.a.length().max(w.b.length()) + w.radius + body.pos.distance(body.prev_pos))
}

/// A hurt shape's horizontal extent around its root, plus the root's travel this tick (m).
fn hurt_bound(hurt: &HurtShape, body: &Body) -> f32 {
    let flat = |v: Vec3| Vec3::new(v.x, 0.0, v.z).length();
    (flat(hurt.bottom).max(flat(hurt.top)) + hurt.radius).max(flat(hurt.head) + hurt.head_radius) + body.pos.distance(body.prev_pos)
}

/// Shared, read-only inputs.
#[derive(Clone, Copy)]
pub struct CombatCtx<'a> {
    pub tick: Tick,
    pub movesets: &'a Movesets,
    pub sockets: &'a dyn SocketSource,
}

/// A body's pose after this tick's mover step and before it, in one local frame the shell picks
/// (positions relative to an anchor: `WorldPos` deltas, never `Entity`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Body {
    pub pos: Vec3,
    pub yaw: f32,
    pub prev_pos: Vec3,
    pub prev_yaw: f32,
}

impl Body {
    pub fn at(pos: Vec3, yaw: f32) -> Self {
        Self { pos, yaw, prev_pos: pos, prev_yaw: yaw }
    }

    /// The pose a share `f` of the way through the tick.
    pub fn lerp(&self, f: f32) -> (Vec3, f32) {
        let dyaw = (self.yaw - self.prev_yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        (self.prev_pos.lerp(self.pos, f), self.prev_yaw + dyaw * f)
    }
}

/// One combatant this tick.
pub struct Unit<'a> {
    pub fighter: &'a mut Fighter,
    pub fight: &'a mut FightState,
    pub action: &'a mut ActionState,
    /// This tick's `sf_action::step` output.
    pub output: &'a ActionOutput,
    pub body: Body,
}

/// Presentation and gameplay messages.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CombatEvent {
    HitLanded { attacker: CharacterId, defender: CharacterId, hit: u8, point: Vec3, kind: TagId, tier: Tier, side: Side },
    /// A sweep touched a defender inside its i-frames: no landing, the pair stays unspent.
    Evaded { attacker: CharacterId, defender: CharacterId, hit: u8 },
    /// The rules said the contact doesn't land (the pair is spent).
    Missed { attacker: CharacterId, defender: CharacterId, hit: u8 },
    Died { who: CharacterId },
}

#[derive(Clone, Debug, Default)]
pub struct CombatOutput {
    pub events: Vec<CombatEvent>,
}

/// The start gate (pipeline item 1): classes `who` may not start now, for `Sense::refused`. A
/// winded fighter (`FightState::winded`) may not sprint either.
pub fn gate<R: CombatRules + ?Sized>(rules: &R, who: &Fighter, fight: &FightState) -> ClassSet {
    if who.dead() {
        return ClassSet::LATCHABLE;
    }
    let mut s = ClassSet::EMPTY;
    for c in ClassSet::LATCHABLE.iter().filter(|&c| !rules.can_start(who, c)) {
        s.insert(c);
    }
    if fight.winded {
        s.insert(sf_action::ActionClass::Sprint);
    }
    s
}

struct Pending {
    attacker: usize,
    defender: usize,
    hit: u8,
    row: HitRow,
    point: Vec3,
}

/// Pipeline items 4-8 for every unit. Deterministic: units in slice order, contacts in
/// (attacker, hit row, sub-step, defender) order.
pub fn step<R: CombatRules + ?Sized>(ctx: &CombatCtx<'_>, units: &mut [Unit<'_>], rules: &R, out: &mut CombatOutput) {
    out.events.clear();
    let near: Vec<bool> = (0..units.len()).map(|i| hostile_near(units, i)).collect();
    for (u, near) in units.iter_mut().zip(near) {
        vitals(ctx, u, rules, near);
    }
    let pending = sweep(ctx, units);
    let mut stops: Vec<Option<HitStop>> = vec![None; units.len()];
    for p in pending {
        resolve(ctx, units, rules, &p, &mut stops, out);
    }
    for (u, stop) in units.iter_mut().zip(stops) {
        match (stop, &mut u.fight.hit_stop) {
            (Some(s), h) => *h = Some(s),
            (None, Some(s)) => s.t = s.t.saturating_add(1),
            (None, None) => {}
        }
        if u.fight.hit_stop.is_some_and(HitStop::done) {
            u.fight.hit_stop = None;
        }
        u.action.clock.rate = u.fight.hit_stop.map_or(RATE_ONE, HitStop::rate);
    }
}

/// Item 4: a combat move start begins its swing and charges its start cost; an opening hit window
/// charges its row's stamina; a jump (a launch start) and sprinting cost the sheet's `costs` while
/// a hostile is `near`; regen while open (not in a combat move, or past its first `Cancel(Move)`;
/// never on a draining sprint tick).
fn vitals<R: CombatRules + ?Sized>(ctx: &CombatCtx<'_>, u: &mut Unit<'_>, rules: &R, near: bool) {
    if u.fighter.dead() {
        u.fight.regen_open = false;
        return;
    }
    let spec = ctx.movesets.spec(u.fighter.moveset, u.action.mov);
    let (t0, t1) = u.output.clock;
    if u.output.started {
        u.fight.swing = None;
        if let Some(spec) = spec.filter(|s| !s.classes.is_empty()) {
            let plan = rules.begin_swing(&Swing { attacker: u.fighter.id, move_key: u.action.mov, strength: 1.0, candidates: &[] });
            if spec.cost > 0.0 {
                let d = rules.cost(u.fighter, spec.cost, plan.strength);
                u.fighter.vitals.apply(d);
            }
            if !spec.hits.is_empty() {
                u.fight.swing = Some(SwingState { mov: u.action.mov, start: ctx.tick, plan, landed: SmallVec::default(), charged: 0 });
            }
        }
    }
    if let (Some(sw), Some(spec)) = (&mut u.fight.swing, spec)
        && sw.mov == u.action.mov
    {
        for (i, row) in spec.hits.iter().enumerate() {
            let bit = 1u32 << i;
            if sw.charged & bit == 0 && t1 >= row.start {
                sw.charged |= bit;
                if row.stamina > 0.0 {
                    let d = rules.cost(u.fighter, row.stamina, sw.plan.strength);
                    u.fighter.vitals.apply(d);
                }
            }
        }
    } else {
        u.fight.swing = None;
    }
    // Jump and sprint cost stamina only while a hostile is near (calm-world movement is free, ER);
    // a sprinting tick drains instead of regenerating.
    let costs = ctx.movesets.sheets.get(&u.fighter.moveset).map(|s| s.costs).unwrap_or_default();
    if near && u.output.started && u.output.launch.is_some() && costs.jump > 0.0 {
        let d = rules.cost(u.fighter, costs.jump, 1.0);
        u.fighter.vitals.apply(d);
    }
    let sprinting = spec.is_none() && u.action.mode.mode == Mode::FreeRun && u.output.intent.desired_velocity != Vec3::ZERO;
    let draining = near && sprinting && costs.sprint_per_s > 0.0;
    if draining {
        #[allow(clippy::cast_precision_loss)]
        let d = rules.cost(u.fighter, costs.sprint_per_s / TICK_HZ as f32, 1.0);
        u.fighter.vitals.apply(d);
    }
    let open = !draining && spec.is_none_or(|s| t0 >= s.regen_from);
    u.fight.regen_open = open;
    if open {
        let d = rules.regen(u.fighter, RegenState::Open);
        u.fighter.vitals.apply(d);
    }
    u.fight.update_winded(u.fighter.vitals.stamina);
}

/// Within this range of a living hostile fighter, sprinting and jumping cost stamina (m).
pub const HOSTILE_RANGE: f32 = 20.0;

/// Is a living hostile fighter other than unit `i` within [`HOSTILE_RANGE`]?
fn hostile_near(units: &[Unit<'_>], i: usize) -> bool {
    let at = units[i].body.pos;
    units
        .iter()
        .enumerate()
        .any(|(j, o)| j != i && o.fighter.hostile && !o.fighter.dead() && o.body.pos.distance_squared(at) <= HOSTILE_RANGE * HOSTILE_RANGE)
}

/// Item 5: each open hit row in `(t0, t1]`, the blade at [`SWEEP_STEPS`] sub-steps on the
/// interpolated root poses, against every other living unit's hurt shape. The first touch of a
/// `(row, defender)` pair not yet landed is a contact.
fn sweep(ctx: &CombatCtx<'_>, units: &[Unit<'_>]) -> Vec<Pending> {
    let mut pending = Vec::new();
    for (ai, a) in units.iter().enumerate() {
        let Some(sw) = a.fight.swing.as_ref().filter(|s| s.plan.connects && s.mov == a.action.mov) else { continue };
        let Some(spec) = ctx.movesets.spec(a.fighter.moveset, sw.mov) else { continue };
        let (t0, t1) = a.output.clock;
        if t1 <= t0 || a.fighter.dead() {
            continue;
        }
        let w = a.fighter.weapon;
        // Reach prefilter: defenders whose hurt shape can't meet the blade this tick are skipped.
        let reach = reach_bound(ctx.sockets, spec, w, &a.body);
        let in_reach: Vec<bool> = units
            .iter()
            .map(|d| {
                reach.is_none_or(|r| {
                    let gap = Vec3::new(d.body.pos.x - a.body.pos.x, 0.0, d.body.pos.z - a.body.pos.z).length();
                    gap <= r + hurt_bound(&d.fighter.hurt, &d.body)
                })
            })
            .collect();
        if !in_reach.iter().enumerate().any(|(di, &ok)| ok && di != ai) {
            continue;
        }
        for (n, row) in spec.hits.iter().enumerate() {
            let (lo, hi) = (t0.0.max(row.start.0), t1.0.min(row.end.0));
            if t1 < row.start || t0 >= row.end || lo > hi {
                continue;
            }
            let hit = u8::try_from(n).unwrap_or(u8::MAX);
            let mut found: Vec<usize> = Vec::new();
            for k in 0..SWEEP_STEPS {
                #[allow(clippy::cast_precision_loss)]
                let s = k as f32 / (SWEEP_STEPS - 1) as f32;
                #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let t = ClockTime(lo + ((hi - lo) as f32 * s).round() as u32);
                #[allow(clippy::cast_precision_loss)]
                let f = (t.0 - t0.0) as f32 / (t1.0 - t0.0) as f32;
                let Some((slot, clip_t)) = spec.slot_at(t) else { continue };
                let Some(track) = ctx.sockets.socket(slot) else { continue };
                let (pos, yaw) = a.body.lerp(f);
                let rot = Quat::from_rotation_y(yaw);
                let (p, q) = (pos + rot * track.point(clip_t, w.a), pos + rot * track.point(clip_t, w.b));
                for (di, d) in units.iter().enumerate() {
                    if di == ai || !in_reach[di] || found.contains(&di) || d.fighter.dead() || sw.landed.iter().any(|&(h, id)| h == hit && id == d.fighter.id) {
                        continue;
                    }
                    let (dpos, dyaw) = d.body.lerp(f);
                    if let Some(point) = d.fighter.hurt.touch(dpos, dyaw, p, q, w.radius) {
                        found.push(di);
                        pending.push(Pending { attacker: ai, defender: di, hit, row: *row, point });
                    }
                }
            }
        }
    }
    pending
}

/// Items 6-8 for one contact: dead -> skip; open i-frames -> no landing, pair unspent; (guard: C2);
/// then the rules, pools, tier and side, death, the forced reaction and hit-stop on both sides.
fn resolve<R: CombatRules + ?Sized>(
    ctx: &CombatCtx<'_>,
    units: &mut [Unit<'_>],
    rules: &R,
    p: &Pending,
    stops: &mut [Option<HitStop>],
    out: &mut CombatOutput,
) {
    let (a, d) = (&units[p.attacker], &units[p.defender]);
    let (aid, did) = (a.fighter.id, d.fighter.id);
    if d.fighter.dead() || a.fighter.dead() {
        return;
    }
    let dspec = ctx.movesets.spec(d.fighter.moveset, d.action.mov);
    if dspec.is_some_and(|s| s.iframes_at(d.output.clock.1).covers(p.row.flags)) {
        out.events.push(CombatEvent::Evaded { attacker: aid, defender: did, hit: p.hit });
        return;
    }
    let Some(sw) = a.fight.swing.as_ref() else { return };
    let toward = Vec3::new(d.body.pos.x - a.body.pos.x, 0.0, d.body.pos.z - a.body.pos.z).normalize_or_zero();
    let side = Side::of(d.body.yaw, -toward);
    let in_reaction = dspec.is_some_and(|s| s.category == sf_action::Category::Forced);
    let contact = Contact {
        attacker: aid,
        defender: did,
        move_key: sw.mov,
        hit: p.hit,
        strength: sw.plan.strength,
        kind: p.row.kind,
        mv: p.row.mv,
        flags: p.row.flags,
        point: p.point,
        direction: toward,
        side,
        defender_state: DefenderState { in_reaction, ..DefenderState::default() },
    };
    let outcome = rules.resolve_contact(&contact, a.fighter, d.fighter);
    if let Some(sw) = units[p.attacker].fight.swing.as_mut() {
        sw.landed.push((p.hit, did));
    }
    units[p.attacker].fighter.vitals.apply(outcome.attacker);
    if !outcome.landed {
        out.events.push(CombatEvent::Missed { attacker: aid, defender: did, hit: p.hit });
        return;
    }
    let victim = &mut units[p.defender];
    victim.fighter.vitals.apply(outcome.defender);
    let mut tier = outcome.tier_hint.unwrap_or(p.row.tier).max(victim.fighter.min_tier);
    let killed = victim.fighter.dead();
    if killed {
        tier = Tier::Death;
    }
    if let Some(rule) = ctx.movesets.sheets.get(&victim.fighter.moveset).and_then(|s| s.reactions.lookup(tier, side)) {
        victim.action.forced = Some(Forced { rule, side: side.index() });
    }
    let ease = ctx.movesets.sheets.get(&units[p.attacker].fighter.moveset).map_or(0.0, |s| s.hit_stop.ease);
    let stop = HitStop::new(p.row.stop, ease);
    if !stop.is_empty() {
        stops[p.attacker] = Some(stop);
        stops[p.defender] = Some(stop);
    }
    out.events.push(CombatEvent::HitLanded { attacker: aid, defender: did, hit: p.hit, point: p.point, kind: p.row.kind, tier, side });
    if killed {
        out.events.push(CombatEvent::Died { who: did });
    }
}
