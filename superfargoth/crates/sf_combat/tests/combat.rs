//! Headless combat tests (combat.md "Tests"): scripted `Command` streams through
//! `sf_action::step` + `sf_combat::step` on `data/test_sword.ron`, with synthetic slots,
//! straight-line socket tracks for the blade and a straight-line root track for the roll.
//!
//! Logic keeps exact assertions (timing in ticks, contact order, costs, refusals); feel numbers are
//! printed by the `#[ignore = "measurement: ..."]` tests.

// Exact float asserts are the point (pool arithmetic, times on the 60 Hz grid); tick counts print
// as seconds.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use glam::{Vec2, Vec3};
use sf_action::{
    ActionClass, ActionEvent, ActionOutput, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, EventKind, InputSampler, Origin,
    RATE_ONE, Refused, RootPath, RuleKey, RuleSet, RuleSetBuilder, Sense, SlotInfo, SlotKey, SlotTable, StepCtx, TapHold, Tick,
};
use sf_anim::{RootPose, SocketTrack, Trs};
use sf_combat::{
    At, Body, CombatCtx, CombatEvent, CombatOutput, CombatRules, CompiledSheet, Contact, Defense, FightState, Fighter, HitStop, HurtShape,
    Movesets, Offense, Outcome, Pool, PoolDelta, RegenState, Side, SocketSource, SocketTracks, StandardRules, Swing, SwingPlan, Tier, Unit, Vitals, gate,
    load,
};

const SHEET: &str = include_str!("data/test_sword.ron");
const DT: f32 = 1.0 / 60.0;
const BLADE_Y: f32 = 1.2;
const BLADE_Z: f32 = 0.3;

/// Synthetic clip lengths (clip seconds).
const CLIPS: &[(&str, f32)] = &[
    ("attack.test.light1", 0.5),
    ("attack.test.light1_rec", 0.5),
    ("attack.test.light2", 0.9),
    ("attack.test.light3", 1.0),
    ("attack.test.thrust", 0.8),
    ("dodge.test.roll", 1.0),
    ("react.test.small.front", 0.4),
    ("react.test.medium.front", 0.6),
];

/// Roll root track: straight ahead over the clip.
const ROLL_CLIP_DIST: f32 = 3.0;

fn clip_len(slot: &str) -> Option<f32> {
    CLIPS.iter().find(|(s, _)| *s == slot).map(|&(_, l)| l)
}

fn sheet() -> CompiledSheet {
    load(SHEET, &clip_len).expect("test sheet loads")
}

/// A straight line along +Z (the clip's forward) over `len` seconds.
struct Line {
    len: f32,
    dist: f32,
}

impl RootPath for Line {
    fn duration(&self) -> f32 {
        self.len
    }

    fn root_at(&self, t: f32) -> RootPose {
        RootPose::new(Vec3::new(0.0, 0.0, self.dist * (t / self.len).clamp(0.0, 1.0)), 0.0)
    }
}

fn blade_at(x: f32) -> Trs {
    Trs { translation: Vec3::new(x, BLADE_Y, BLADE_Z), ..Trs::IDENTITY }
}

fn ramp(t: f32, a: f32, b: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * ((t - a) / (b - a)).clamp(0.0, 1.0)
}

/// Straight-line socket tracks: a horizontal slash crosses x = 0 (the blade points forward, +Z);
/// the thrust holds the blade straight ahead.
fn sockets() -> HashMap<SlotKey, SocketTrack> {
    let mut m = HashMap::new();
    let mut add = |slot: &str, f: &dyn Fn(f32) -> f32| {
        m.insert(SlotKey::of(slot), SocketTrack::from_fn(clip_len(slot).unwrap(), 60.0, |t| blade_at(f(t))));
    };
    add("attack.test.light1", &|t| ramp(t, 0.18, 0.32, -1.0, 1.0));
    add("attack.test.light1_rec", &|_| 1.0);
    add("attack.test.light2", &|t| ramp(t, 0.20, 0.34, 1.0, -1.0));
    add("attack.test.light3", &|t| if t < 0.35 { ramp(t, 0.18, 0.30, -1.0, 1.0) } else { ramp(t, 0.40, 0.52, 1.0, -1.0) });
    add("attack.test.thrust", &|_| 0.0);
    m
}

/// Loco slots as in `sf_action`'s tests, the sheet's slots, the roll's root track (unless the
/// character rolls in place).
fn slot_table(sheet: &CompiledSheet, roll_root: bool) -> SlotTable {
    let base = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5));
    let t = sheet.fill(base);
    if roll_root { t.with_root("dodge.test.roll", Arc::new(Line { len: 1.0, dist: ROLL_CLIP_DIST })) } else { t }
}

fn rule_set(sheet: &CompiledSheet) -> RuleSet {
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES);
    sheet.add_rules(&mut b, &Origin::Mod("test".into()));
    b.build().expect("rules build")
}

struct Actor {
    fighter: Fighter,
    fight: FightState,
    state: ActionState,
    out: ActionOutput,
    body: Body,
    slots: SlotTable,
    /// (tick, event) of every action event.
    events: Vec<ActionEvent>,
}

struct World {
    sheet: CompiledSheet,
    rules: RuleSet,
    movesets: Movesets,
    sockets: Box<dyn SocketSource>,
    params: CharacterParams,
    combat: Box<dyn CombatRules>,
    actors: Vec<Actor>,
    tick: Tick,
    /// Every combat event with its tick.
    log: Vec<(Tick, CombatEvent)>,
}

fn loco() -> RuleKey {
    RuleKey::of("loco")
}

impl World {
    fn new(combat: Box<dyn CombatRules>) -> Self {
        let sheet = sheet();
        let rules = rule_set(&sheet);
        let mut movesets = Movesets::default();
        movesets.insert(sheet.clone());
        Self { sheet, rules, movesets, sockets: Box::new(sockets()), params: CharacterParams::default(), combat, actors: Vec::new(), tick: Tick(0), log: Vec::new() }
    }

    fn standard() -> Self {
        Self::new(Box::new(StandardRules::default()))
    }

    /// Adds a fighter at `pos` facing `yaw` (0 faces +Z); returns its index.
    fn spawn(&mut self, pos: Vec3, yaw: f32) -> usize {
        let i = self.actors.len();
        let mut state = ActionState::new(loco());
        state.mode.moveset = Some(self.sheet.key);
        let fighter = Fighter {
            id: CharacterId(100 + i as u64),
            moveset: self.sheet.key,
            vitals: Vitals { health: Pool::full(100.0), stamina: Pool::full(100.0), poise: None },
            weapon: self.sheet.weapon,
            hurt: HurtShape::from_capsule(0.4, 1.8),
            offense: Offense { attack: 10.0 },
            defense: Defense { absorb: 0.0, guard_absorb: 0.0, guard_stability: 0.0 },
            hostile: true,
            min_tier: Tier::Small,
        };
        self.actors.push(Actor {
            fighter,
            fight: FightState::default(),
            state,
            out: ActionOutput::default(),
            body: Body::at(pos, yaw),
            slots: slot_table(&self.sheet, true),
            events: Vec::new(),
        });
        i
    }

    /// Attacker at the origin facing +Z, a dummy `dist` ahead facing it; settled in Loco.
    fn duel(dist: f32) -> Self {
        let mut w = Self::standard();
        w.spawn(Vec3::ZERO, 0.0);
        w.spawn(Vec3::new(0.0, 0.0, dist), std::f32::consts::PI);
        w.settle();
        w
    }

    fn settle(&mut self) {
        for _ in 0..5 {
            self.idle();
        }
        for a in &self.actors {
            assert_eq!(a.state.mov, loco());
        }
    }

    fn key(&self, mov: &str) -> RuleKey {
        self.sheet.rule_key(mov)
    }

    /// One fixed tick: gate -> `sf_action::step` -> flat-ground mover -> `sf_combat::step`.
    /// `cmds[i]` drives actor `i` (missing: idle).
    fn run(&mut self, cmds: &[Command]) -> CombatOutput {
        self.tick = self.tick.next();
        let tick = self.tick;
        for (i, a) in self.actors.iter_mut().enumerate() {
            let mut cmd = cmds.get(i).copied().unwrap_or_default();
            cmd.tick = tick;
            let sense = Sense { grounded: true, yaw: a.body.yaw, refused: gate(&*self.combat, &a.fighter, &a.fight), ..Sense::default() };
            let ctx = StepCtx { rules: &self.rules, slots: &a.slots, params: &self.params, source: a.fighter.id };
            sf_action::step(&ctx, &mut a.state, &cmd, &sense, &mut a.out);
            a.events.extend(a.out.events.iter().copied());
            a.body.prev_pos = a.body.pos;
            a.body.prev_yaw = a.body.yaw;
            a.body.pos += a.out.intent.desired_velocity * DT;
            if let Some(c) = a.out.clip {
                a.body.pos += c.delta;
                a.body.yaw = c.yaw;
            }
        }
        let ctx = CombatCtx { tick, movesets: &self.movesets, sockets: &*self.sockets };
        let mut units: Vec<Unit<'_>> = self
            .actors
            .iter_mut()
            .map(|a| Unit { fighter: &mut a.fighter, fight: &mut a.fight, action: &mut a.state, output: &a.out, body: a.body })
            .collect();
        let mut out = CombatOutput::default();
        sf_combat::step(&ctx, &mut units, &*self.combat, &mut out);
        self.log.extend(out.events.iter().map(|&e| (tick, e)));
        out
    }

    fn idle(&mut self) -> CombatOutput {
        self.run(&[])
    }

    /// Actor 0 presses `c` this tick.
    fn press(&mut self, c: ActionClass) -> CombatOutput {
        self.run(&[press(c)])
    }

    fn started(&self, i: usize, mov: RuleKey) -> bool {
        self.actors[i].out.started && self.actors[i].state.mov == mov
    }

    /// Runs idle ticks until actor `i` is back in Loco (at most `max`).
    fn until_loco(&mut self, i: usize, max: u32) {
        for _ in 0..max {
            if self.actors[i].state.mov == loco() {
                return;
            }
            self.idle();
        }
        panic!("actor {i} not back in Loco after {max} ticks");
    }

    fn landings(&self) -> Vec<(Tick, CharacterId, CharacterId, u8)> {
        self.log
            .iter()
            .filter_map(|&(t, e)| match e {
                CombatEvent::HitLanded { attacker, defender, hit, .. } => Some((t, attacker, defender, hit)),
                _ => None,
            })
            .collect()
    }
}

fn press(c: ActionClass) -> Command {
    let mut cmd = Command::default();
    cmd.buttons.pressed.insert(c);
    cmd.buttons.held.insert(c);
    cmd
}

// ---------------------------------------------------------------------------------------------
// Sheet

#[test]
fn at_parses_seconds_integers_and_end() {
    let v: Vec<At> = ron::from_str("[0.25, 1, End]").unwrap();
    assert_eq!(v, vec![At::Secs(0.25), At::Secs(1.0), At::End]);
    let (a, b): (f32, At) = ron::from_str("(0.1, End)").unwrap();
    assert_eq!((a, b), (0.1, At::End));
    assert!(ron::from_str::<At>("Later").is_err());
    assert!(ron::from_str::<At>("()").is_err());
    for a in [At::Secs(0.5), At::End] {
        assert_eq!(ron::from_str::<At>(&ron::to_string(&a).unwrap()).unwrap(), a);
    }
}

#[test]
fn sheet_compiles_rules_slots_and_table() {
    let s = sheet();
    assert!(s.warnings.is_empty(), "{:?}", s.warnings);
    // Five moves and two reactions.
    assert_eq!(s.rules.len(), 7);
    let rank = |m: &str| s.rules.iter().find(|r| r.id == format!("test_sword.{m}")).unwrap().rank;
    // Chain targets outrank the opener that could also take the press (light1, from Any).
    assert!(rank("light2") > rank("light1"));
    assert!(rank("light3") > rank("light1"));
    let l1 = &s.moves.moves[&s.rule_key("light1")];
    assert_eq!(l1.length.secs(), 1.0);
    assert_eq!(l1.regen_from.secs(), 0.80);
    assert_eq!(l1.hits.len(), 1);
    assert_eq!(l1.hits[0].stop, 0.05);
    let roll = &s.moves.moves[&s.rule_key("roll")];
    // 1 s clip at rate 1.25.
    assert_eq!(roll.length.secs(), 0.8);
    assert_eq!(roll.cost, 12.0);
    // The thrust row takes the sheet default stop.
    assert_eq!(s.moves.moves[&s.rule_key("thrust")].hits[0].stop, 0.06);
    // Windows split per slot: light1's hit window lives in its first slot only.
    let slot = |n: &str| &s.slots.iter().find(|(k, _)| k == n).unwrap().1;
    let hit = |i: &SlotInfo| i.windows.iter().filter(|w| matches!(w.kind, sf_action::WindowKind::Hit(0))).count();
    assert_eq!(hit(slot("attack.test.light1")), 1);
    assert_eq!(hit(slot("attack.test.light1_rec")), 0);
    assert!(s.reactions.lookup(Tier::Small, Side::Back).is_some());
}

#[test]
fn sheet_load_errors_name_the_move() {
    let bad = SHEET.replace(r#"next: { Light: "light2" }"#, r#"next: { Light: "light9" }"#);
    let errs = load(&bad, &clip_len).unwrap_err();
    assert_eq!(errs.len(), 1, "{errs:?}");
    assert_eq!(errs[0].mov.as_deref(), Some("light1"));
    assert!(errs[0].message.contains("unknown move 'light9'"), "{}", errs[0]);
    // A chain into a move that doesn't take the class.
    let bad = SHEET.replace(r#"next: { Light: "light2" }"#, r#"next: { Light: "thrust" }"#);
    assert!(load(&bad, &clip_len).is_err());
}

#[test]
fn a_dropped_slot_shortens_the_move_with_a_warning() {
    let s = load(SHEET, &|n| if n == "attack.test.light1_rec" { None } else { clip_len(n) }).unwrap();
    assert_eq!(s.warnings.len(), 1, "{:?}", s.warnings);
    assert_eq!(s.moves.moves[&s.rule_key("light1")].length.secs(), 0.5);
}

// ---------------------------------------------------------------------------------------------
// Chain, input, latency

#[test]
fn chain_fires_on_the_cancel_tick_and_wraps() {
    let mut w = World::duel(10.0);
    let (l1, l2, l3) = (w.key("light1"), w.key("light2"), w.key("light3"));
    // First response on the press tick from Loco.
    w.press(ActionClass::Light);
    assert!(w.started(0, l1));
    let refusals = &w.actors[0].out.refusals;
    assert!(refusals.iter().any(|r| r.rule == l2 && r.reason == Refused::Prev), "{refusals:?}");
    // A press inside Accept (0.10 s = tick 6) fires on Cancel(Light) (0.45 s = tick 27).
    for (mov, next, cancel) in [(l1, l2, 27), (l2, l3, 27), (l3, l1, 39)] {
        assert_eq!(w.actors[0].state.mov, mov);
        for k in 1..=cancel {
            let out = if k == 10 { w.press(ActionClass::Light) } else { w.idle() };
            assert!(out.events.is_empty());
            if k < cancel {
                assert_eq!(w.actors[0].state.mov, mov, "tick {k}");
            }
        }
        assert!(w.started(0, next), "{next:?} on the Cancel(Light) tick");
    }
    // Leaving the move resets the chain: back in Loco, Light opens with light1.
    w.until_loco(0, 120);
    w.press(ActionClass::Light);
    assert!(w.started(0, l1));
}

#[test]
fn a_press_before_accept_is_lost() {
    let mut w = World::duel(10.0);
    let l1 = w.key("light1");
    let seen = w.actors[0].events.len();
    w.press(ActionClass::Light);
    assert!(w.started(0, l1));
    for _ in 1..5 {
        w.idle();
    }
    // Tick 5: clock 5/60 s < Accept 0.10 s.
    w.press(ActionClass::Light);
    assert_eq!(w.actors[0].state.buffer.latched, sf_action::ClassSet::EMPTY);
    w.until_loco(0, 120);
    let starts = w.actors[0].events[seen..].iter().filter(|e| matches!(e.kind, EventKind::MoveStart(_))).count();
    // light1, then Loco.
    assert_eq!(starts, 2);
}

#[test]
fn dodge_starts_on_the_press_tick_from_loco() {
    let mut w = World::duel(10.0);
    w.press(ActionClass::Dodge);
    assert!(w.started(0, w.key("roll")));
}

#[test]
fn tap_hold_dodge_and_sprint() {
    // Released after 17 held samples: Dodge on the release tick.
    let mut w = World::duel(10.0);
    let roll = w.key("roll");
    let mut input = InputSampler::new();
    let b = input.bind(TapHold::DODGE_SPRINT);
    input.press_shared(b);
    for k in 1..=17 {
        let cmd = input.sample(Tick(k), None);
        assert!(!cmd.pressed(ActionClass::Dodge) && !cmd.held(ActionClass::Sprint), "sample {k}");
        w.run(&[cmd]);
        assert_eq!(w.actors[0].state.mov, loco());
    }
    input.release_shared(b);
    let cmd = input.sample(Tick(18), None);
    assert!(cmd.pressed(ActionClass::Dodge));
    w.run(&[cmd]);
    assert!(w.started(0, roll));

    // Held 30 samples: Sprint from sample 18, never a Dodge.
    let mut w = World::duel(10.0);
    let mut input = InputSampler::new();
    let b = input.bind(TapHold::DODGE_SPRINT);
    input.press_shared(b);
    for k in 1..=31 {
        if k == 31 {
            input.release_shared(b);
        }
        let cmd = input.sample(Tick(k), None);
        assert!(!cmd.pressed(ActionClass::Dodge), "sample {k}");
        assert_eq!(cmd.held(ActionClass::Sprint), (18..=30).contains(&k), "sample {k}");
        w.run(&[cmd]);
        assert_ne!(w.actors[0].state.mov, roll);
    }
}

#[test]
fn refusals_name_the_moveset_and_prev() {
    let mut w = World::duel(10.0);
    w.actors[0].state.mode.moveset = None;
    w.press(ActionClass::Light);
    assert_eq!(w.actors[0].state.mov, loco());
    let r = &w.actors[0].out.refusals;
    assert!(r.iter().any(|r| r.rule == w.key("light1") && r.reason == Refused::Moveset), "{r:?}");
}

// ---------------------------------------------------------------------------------------------
// Sweeps, contact order, forced moves

#[test]
fn one_landing_per_hit_and_defender_then_a_forced_reaction() {
    let mut w = World::duel(1.0);
    w.press(ActionClass::Light);
    w.until_loco(0, 200);
    let landed = w.landings();
    assert_eq!(landed.len(), 1, "{:?}", w.log);
    let (t, attacker, defender, hit) = landed[0];
    assert_eq!((attacker, defender, hit), (w.actors[0].fighter.id, w.actors[1].fighter.id, 0));
    assert_eq!(w.actors[1].fighter.vitals.health.cur, 90.0);
    let ev = w.log.iter().find(|(tt, e)| *tt == t && matches!(e, CombatEvent::HitLanded { .. })).unwrap().1;
    let CombatEvent::HitLanded { tier, side, kind, .. } = ev else { unreachable!() };
    assert_eq!((tier, side, kind), (Tier::Small, Side::Front, sf_action::TagId::of("slash")));
    // The forced reaction starts on the next tick, whatever the dummy was doing.
    let react = w.sheet.reactions.lookup(Tier::Small, Side::Front).unwrap();
    let start = w.actors[1].events.iter().find(|e| e.kind == EventKind::MoveStart(react)).expect("reaction started");
    assert_eq!(start.tick, t.next());
}

#[test]
fn light3_lands_twice_and_the_chain_lands_four_hits() {
    let mut w = World::duel(1.0);
    let l3 = w.key("light3");
    let mut by_move: Vec<(RuleKey, u8)> = Vec::new();
    // Mash Light until light3 plays, then let it finish.
    for _ in 0..400 {
        let out = if w.actors[0].state.mov == l3 { w.idle() } else { w.press(ActionClass::Light) };
        for e in &out.events {
            if let CombatEvent::HitLanded { hit, .. } = e {
                by_move.push((w.actors[0].state.mov, *hit));
            }
        }
        if w.actors[0].state.mov == loco() && by_move.iter().any(|&(m, _)| m == l3) {
            break;
        }
    }
    assert_eq!(by_move, vec![(w.key("light1"), 0), (w.key("light2"), 0), (l3, 0), (l3, 1)]);
}

#[test]
fn out_of_reach_never_hits() {
    let mut w = World::duel(3.0);
    for _ in 0..300 {
        w.press(ActionClass::Light);
    }
    w.press(ActionClass::Heavy);
    w.until_loco(0, 200);
    assert!(w.log.is_empty(), "{:?}", w.log);
}

/// Counts `resolve_contact` calls; otherwise standard.
#[derive(Default)]
struct Counting {
    contacts: Arc<AtomicUsize>,
}

impl CombatRules for Counting {
    fn begin_swing(&self, s: &Swing<'_>) -> SwingPlan {
        StandardRules::default().begin_swing(s)
    }
    fn resolve_contact(&self, c: &Contact, a: &Fighter, d: &Fighter) -> Outcome {
        self.contacts.fetch_add(1, Ordering::Relaxed);
        StandardRules::default().resolve_contact(c, a, d)
    }
    fn can_start(&self, who: &Fighter, class: ActionClass) -> bool {
        StandardRules::default().can_start(who, class)
    }
    fn cost(&self, who: &Fighter, base: f32, strength: f32) -> PoolDelta {
        StandardRules::default().cost(who, base, strength)
    }
    fn regen(&self, who: &Fighter, state: RegenState) -> PoolDelta {
        StandardRules::default().regen(who, state)
    }
}

#[test]
fn iframes_leave_the_pair_unspent_until_they_close() {
    let counting = Counting::default();
    let contacts = Arc::clone(&counting.contacts);
    let mut w = World::new(Box::new(counting));
    w.spawn(Vec3::ZERO, 0.0);
    let d = w.spawn(Vec3::new(0.0, 0.0, 1.0), std::f32::consts::PI);
    // The dummy rolls in place (no root track), so only its i-frames matter.
    w.actors[d].slots = slot_table(&w.sheet, false);
    w.settle();
    // Same tick: the attacker thrusts (hit 0.10-0.60 s, blade held on the dummy), the dummy rolls
    // (i-frames 0.02-0.40 s, tested at the end of its tick).
    w.run(&[press(ActionClass::Heavy), press(ActionClass::Dodge)]);
    let a0 = w.tick;
    assert!(w.started(0, w.key("thrust")) && w.started(d, w.key("roll")));
    for _ in 0..40 {
        w.idle();
    }
    let evaded: Vec<u32> = w.log.iter().filter(|(_, e)| matches!(e, CombatEvent::Evaded { .. })).map(|(t, _)| t.0 - a0.0).collect();
    assert_eq!(evaded, (5..=22).collect::<Vec<_>>());
    let landed = w.landings();
    assert_eq!(landed.len(), 1);
    assert_eq!(landed[0].0.0 - a0.0, 23);
    // I-frames come before the rules: one contact reached them.
    assert_eq!(contacts.load(Ordering::Relaxed), 1);
}

#[test]
fn a_dead_defender_is_skipped_and_refuses_everything() {
    let mut w = World::duel(1.0);
    w.actors[1].fighter.vitals.health.cur = 5.0;
    for _ in 0..120 {
        w.press(ActionClass::Light);
    }
    let died: Vec<_> = w.log.iter().filter(|(_, e)| matches!(e, CombatEvent::Died { .. })).collect();
    assert_eq!(died.len(), 1);
    assert_eq!(w.landings().len(), 1, "no contact after death: {:?}", w.log);
    assert_eq!(w.log.len(), 2);
    assert_eq!(w.actors[1].fighter.vitals.health.cur, 0.0);
    assert_eq!(gate(&StandardRules::default(), &w.actors[1].fighter, &w.actors[1].fight), sf_action::ClassSet::LATCHABLE);
}

// ---------------------------------------------------------------------------------------------
// Stamina

#[test]
fn zero_stamina_refuses_light_and_dodge() {
    let mut w = World::duel(10.0);
    for (c, mov) in [(ActionClass::Light, "light1"), (ActionClass::Dodge, "roll")] {
        w.actors[0].fighter.vitals.stamina.cur = 0.0;
        w.press(c);
        assert_eq!(w.actors[0].state.mov, loco());
        let r = &w.actors[0].out.refusals;
        assert!(r.iter().any(|r| r.rule == w.key(mov) && r.reason == Refused::Gate), "{r:?}");
    }
}

#[test]
fn cancelled_before_its_hit_window_costs_nothing() {
    let mut w = World::duel(10.0);
    let (l2, roll) = (w.key("light2"), w.key("roll"));
    w.press(ActionClass::Light);
    for k in 1..=27 {
        if k == 10 { w.press(ActionClass::Light) } else { w.idle() };
    }
    assert!(w.started(0, l2));
    // light2: Accept from 0.05 s (tick 3), Cancel(Dodge) from 0.10 s (tick 6), hit from 0.20 s.
    for k in 1..=6 {
        if k == 4 { w.press(ActionClass::Dodge) } else { w.idle() };
    }
    assert!(w.started(0, roll));
    // light1's row (12) and the roll (12); light2's row never opened.
    assert_eq!(w.actors[0].fighter.vitals.stamina.cur, 76.0);
}

#[test]
fn regen_waits_for_cancel_move() {
    let mut w = World::duel(10.0);
    w.actors[0].fighter.vitals.stamina.cur = 50.0;
    w.press(ActionClass::Light);
    let mut stamina = vec![w.actors[0].fighter.vitals.stamina.cur];
    for _ in 1..60 {
        w.idle();
        stamina.push(w.actors[0].fighter.vitals.stamina.cur);
    }
    // No regen at the start; the row charges when its window opens (0.18 s: tick 10); regen opens
    // at Cancel(Move) (0.80 s: tick 48) at 45/s.
    assert!(stamina[..10].iter().all(|&s| s == 50.0), "{stamina:?}");
    assert!(stamina[10..48].iter().all(|&s| s == 38.0), "{stamina:?}");
    assert_eq!(stamina[48], 38.75);
}

#[test]
fn overdraw_clamps_at_zero() {
    let mut w = World::duel(10.0);
    w.actors[0].fighter.vitals.stamina.cur = 5.0;
    w.press(ActionClass::Light);
    assert!(w.started(0, w.key("light1")));
    for _ in 1..=10 {
        w.idle();
    }
    assert_eq!(w.actors[0].fighter.vitals.stamina.cur, 0.0);
}

#[test]
fn sprint_and_jump_cost_only_near_a_hostile() {
    let regen = 45.0 / 60.0;
    for (dist, near) in [(5.0, true), (25.0, false)] {
        let mut w = World::duel(dist);
        w.actors[0].fighter.vitals.stamina.cur = 50.0;
        let mut held = sf_action::ClassSet::EMPTY;
        held.insert(ActionClass::Sprint);
        let sprint = Command { stick: Vec2::new(0.0, -1.0), buttons: sf_action::ButtonEdges { held, ..Default::default() }, ..Command::default() };
        w.run(&[sprint]);
        // Draining ticks don't regenerate; calm-world sprinting is free.
        let want = if near { 50.0 - 11.0 / 60.0 } else { 50.0 + regen };
        assert_eq!(w.actors[0].fighter.vitals.stamina.cur, want, "sprint at {dist} m");
        w.actors[0].fighter.vitals.stamina.cur = 50.0;
        w.press(ActionClass::Jump);
        assert!(w.actors[0].out.launch.is_some());
        let want = if near { 50.0 - 10.0 + regen } else { 50.0 + regen };
        assert_eq!(w.actors[0].fighter.vitals.stamina.cur, want, "jump at {dist} m");
    }
}

#[test]
fn sprint_stops_when_winded_and_resumes_after_refilling() {
    let mut w = World::duel(5.0);
    w.actors[0].fighter.vitals.stamina.cur = 0.3;
    let mut held = sf_action::ClassSet::EMPTY;
    held.insert(ActionClass::Sprint);
    let sprint = Command { stick: Vec2::new(0.0, -1.0), buttons: sf_action::ButtonEdges { held, ..Default::default() }, ..Command::default() };
    let mut modes = Vec::new();
    for _ in 0..120 {
        w.run(&[sprint]);
        modes.push((w.actors[0].state.mode.mode, w.actors[0].fight.winded, w.actors[0].fighter.vitals.stamina.cur));
    }
    let first_winded = modes.iter().position(|m| m.1).expect("sprinting drains the pool");
    // Winded (from the next tick's gate): no sprint until the pool is back to the resume share,
    // then sprint again.
    let back = modes[first_winded..].iter().position(|m| !m.1).expect("refills") + first_winded;
    for &(mode, _, cur) in &modes[first_winded + 1..back] {
        assert_ne!(mode, sf_action::Mode::FreeRun, "sprinting while winded at {cur}");
    }
    assert!(modes[back].2 >= 100.0 * FightState::SPRINT_RESUME);
    assert!(modes[back + 1..].iter().any(|m| m.0 == sf_action::Mode::FreeRun));
}

// ---------------------------------------------------------------------------------------------
// Reach prefilter

fn tracks() -> SocketTracks {
    let mut t = SocketTracks::default();
    for (slot, track) in sockets() {
        t.insert(slot, track);
    }
    t
}

#[test]
fn reach_prefilter_keeps_every_landing_and_bounds_the_track() {
    let t = tracks();
    // Blade sockets sit at x in [-1, 1], z = 0.3: the furthest is sqrt(1 + 0.09).
    let r = t.reach(SlotKey::of("attack.test.light1")).unwrap();
    assert!((r - (1.0f32 + BLADE_Z * BLADE_Z).sqrt()).abs() < 1e-4, "{r}");
    let run = |prefilter: bool, dist: f32| {
        let mut w = World::duel(dist);
        if prefilter {
            w.sockets = Box::new(tracks());
        }
        for _ in 0..300 {
            w.press(ActionClass::Light);
        }
        w.landings()
    };
    for dist in [1.0, 1.6, 3.0] {
        assert_eq!(run(true, dist), run(false, dist), "at {dist} m");
    }
    assert!(!run(true, 1.0).is_empty());
}

// ---------------------------------------------------------------------------------------------
// Hit-stop

#[test]
fn hit_stop_holds_both_clocks_then_eases_and_latched_input_fires_after() {
    let mut w = World::duel(1.0);
    let l2 = w.key("light2");
    w.press(ActionClass::Light);
    let mut landed = None;
    for _ in 0..30 {
        let out = w.idle();
        if out.events.iter().any(|e| matches!(e, CombatEvent::HitLanded { .. })) {
            landed = Some(w.tick);
            break;
        }
    }
    landed.expect("light1 lands at 1 m");
    // stop 0.05 s = 3 ticks hold, sheet ease 0.10 s = 6 ticks.
    let expect = HitStop { t: 0, hold: 3, ease: 6 };
    assert_eq!(w.actors[0].fight.hit_stop, Some(expect));
    assert_eq!(w.actors[1].fight.hit_stop, Some(expect));
    let mut rates = vec![(w.actors[0].state.clock.rate, w.actors[1].state.clock.rate)];
    let frozen = w.actors[0].state.clock.elapsed;
    // The tick on which the attacker's clock (advanced by last tick's rate) reaches Cancel(Light).
    let mut reach = None;
    let mut fired = None;
    for k in 1..=40 {
        // Light pressed during the hold latches (Accept is open) and waits for the clock.
        let clock = w.actors[0].state.clock;
        if reach.is_none() && clock.elapsed.0 + u32::from(clock.rate) >= 27_000 {
            reach = Some(k);
        }
        if k == 1 { w.press(ActionClass::Light) } else { w.idle() };
        if k <= 3 {
            assert_eq!(w.actors[0].state.clock.elapsed, frozen, "attacker clock held on tick {k}");
        }
        if k == 9 {
            assert_eq!((w.actors[0].fight.hit_stop, w.actors[1].fight.hit_stop), (None, None), "over after hold + ease");
        }
        if k < 10 {
            rates.push((w.actors[0].state.clock.rate, w.actors[1].state.clock.rate));
        }
        if fired.is_none() && w.started(0, l2) {
            fired = Some(k);
        }
    }
    assert!(rates.iter().all(|(a, d)| a == d), "{rates:?}");
    let a: Vec<u16> = rates.iter().map(|r| r.0).collect();
    assert_eq!(&a[..3], &[0, 0, 0], "{a:?}");
    assert!(a[3..9].iter().all(|&r| 0 < r && r < RATE_ONE) && a[3..9].windows(2).all(|p| p[0] < p[1]), "{a:?}");
    assert_eq!(a[9], RATE_ONE);
    // The latched Light fires on the first tick the slowed clock reaches Cancel(Light) (0.45 s).
    assert_eq!(fired, reach);
    assert!(fired.is_some(), "latched Light fired after the stop");
}

#[test]
fn a_second_hit_replaces_the_stop() {
    let mut w = World::standard();
    w.spawn(Vec3::ZERO, 0.0);
    let d = w.spawn(Vec3::new(0.0, 0.0, 1.0), std::f32::consts::PI);
    let b = w.spawn(Vec3::new(0.0, 0.0, 2.0), std::f32::consts::PI);
    w.settle();
    // Two attackers either side of the dummy, the second two ticks behind.
    w.press(ActionClass::Light);
    w.idle();
    w.run(&[Command::default(), Command::default(), press(ActionClass::Light)]);
    let mut stop = HashMap::new();
    for _ in 0..30 {
        w.idle();
        stop.insert(w.tick, w.actors[d].fight.hit_stop.map(|s| s.t));
    }
    let landed = w.landings();
    assert_eq!(landed.len(), 2, "{:?}", w.log);
    let (t1, t2) = (landed[0].0, landed[1].0);
    assert_eq!(t2.0 - t1.0, 2);
    assert_eq!(landed[1].1, w.actors[b].fighter.id);
    assert_eq!(stop[&t1], Some(0));
    assert_eq!(stop[&t1.next()], Some(1));
    assert_eq!(stop[&t2], Some(0), "the second hit restarts the dummy's stop");
    assert_eq!(w.actors[d].fighter.vitals.health.cur, 80.0);
}

// ---------------------------------------------------------------------------------------------
// The rules seam

struct AlwaysMiss;

impl CombatRules for AlwaysMiss {
    fn begin_swing(&self, _: &Swing<'_>) -> SwingPlan {
        SwingPlan::FULL
    }
    fn resolve_contact(&self, _: &Contact, _: &Fighter, _: &Fighter) -> Outcome {
        Outcome::miss()
    }
    fn can_start(&self, _: &Fighter, _: ActionClass) -> bool {
        true
    }
    fn cost(&self, _: &Fighter, _: f32, _: f32) -> PoolDelta {
        PoolDelta::ZERO
    }
    fn regen(&self, _: &Fighter, _: RegenState) -> PoolDelta {
        PoolDelta::ZERO
    }
}

#[test]
fn an_always_miss_double_changes_no_pool_and_stops_nothing() {
    let mut w = World::new(Box::new(AlwaysMiss));
    w.spawn(Vec3::ZERO, 0.0);
    w.spawn(Vec3::new(0.0, 0.0, 1.0), std::f32::consts::PI);
    w.settle();
    let before: Vec<Vitals> = w.actors.iter().map(|a| a.fighter.vitals).collect();
    w.press(ActionClass::Light);
    for _ in 0..80 {
        w.idle();
        assert!(w.actors.iter().all(|a| a.fight.hit_stop.is_none() && a.state.clock.rate == RATE_ONE));
    }
    let after: Vec<Vitals> = w.actors.iter().map(|a| a.fighter.vitals).collect();
    assert_eq!(before, after);
    let missed = w.log.iter().filter(|(_, e)| matches!(e, CombatEvent::Missed { .. })).count();
    assert_eq!((missed, w.landings().len()), (1, 0), "{:?}", w.log);
    assert_eq!(w.actors[1].state.mov, loco());
}

#[test]
fn an_always_can_start_double_never_refuses() {
    let mut w = World::new(Box::new(AlwaysMiss));
    w.spawn(Vec3::ZERO, 0.0);
    w.settle();
    w.actors[0].fighter.vitals.stamina.cur = 0.0;
    w.press(ActionClass::Light);
    assert!(w.started(0, w.key("light1")));
    assert!(!w.actors[0].out.refusals.iter().any(|r| r.reason == Refused::Gate));
    w.until_loco(0, 200);
    w.press(ActionClass::Dodge);
    assert!(w.started(0, w.key("roll")));
}

// ---------------------------------------------------------------------------------------------
// Determinism

fn scripted_run() -> Vec<Vec<u8>> {
    let mut w = World::duel(1.2);
    let mut frames = Vec::new();
    for k in 0..360u32 {
        let c = match k % 90 {
            0 | 10 | 40 => Some(ActionClass::Light),
            55 => Some(ActionClass::Dodge),
            70 => Some(ActionClass::Heavy),
            _ => None,
        };
        let dummy = if k == 200 { press(ActionClass::Dodge) } else { Command::default() };
        w.run(&[c.map(press).unwrap_or_default(), dummy]);
        for a in &w.actors {
            frames.push(postcard::to_stdvec(&(&a.state, &a.fight, &a.fighter)).unwrap());
        }
    }
    assert!(!w.landings().is_empty(), "the script lands hits");
    frames
}

#[test]
fn one_command_stream_twice_is_byte_identical() {
    assert_eq!(scripted_run(), scripted_run());
}

// ---------------------------------------------------------------------------------------------
// Feel: sanity only (rule 5); numbers in the measurement tests.

#[test]
fn roll_travels_the_scaled_root_track_and_stays_finite() {
    let mut w = World::duel(10.0);
    let roll = w.key("roll");
    w.press(ActionClass::Dodge);
    assert!(w.started(0, roll));
    w.until_loco(0, 120);
    let p = w.actors[0].body.pos;
    assert!(p.is_finite() && w.actors[0].body.yaw.is_finite());
    assert_eq!(p.y, 0.0);
    // Data, not feel: the clip's 3 m root track x root_scale 0.75, all of it (rate 1.25 included).
    assert!((p.z - ROLL_CLIP_DIST * 0.75).abs() < 1e-3, "{p}");
    assert!(p.x.abs() < 1e-4);
}

#[test]
fn square_up_snaps_the_roll_to_the_stick() {
    let mut w = World::duel(10.0);
    let mut cmd = press(ActionClass::Dodge);
    cmd.stick = Vec2::new(1.0, 0.0);
    w.run(&[cmd]);
    assert!(w.started(0, w.key("roll")));
    assert!((w.actors[0].body.yaw - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
}

#[test]
#[ignore = "measurement: first contact time vs dummy distance"]
fn measure_contact_time_vs_distance() {
    println!("light1: first contact after the press, by dummy distance");
    for i in 0..=17 {
        let d = 0.8 + 0.1 * i as f32;
        let mut w = World::duel(d);
        w.press(ActionClass::Light);
        let t0 = w.tick;
        w.until_loco(0, 200);
        match w.landings().first() {
            Some(l) => println!("  {d:.1} m: {:.3} s", (l.0.0 - t0.0) as f32 * DT),
            None => println!("  {d:.1} m: out of reach"),
        }
    }
}

#[test]
#[ignore = "measurement: stamina per chain"]
fn measure_stamina_per_chain() {
    let mut w = World::duel(1.0);
    let l3 = w.key("light3");
    let s0 = w.actors[0].fighter.vitals.stamina.cur;
    let mut min = s0;
    for _ in 0..400 {
        if w.actors[0].state.mov == l3 { w.idle() } else { w.press(ActionClass::Light) };
        min = min.min(w.actors[0].fighter.vitals.stamina.cur);
        if w.actors[0].state.mov == loco() && w.landings().len() >= 4 {
            break;
        }
    }
    println!("light1-2-3 chain: stamina {s0} -> lowest {min} (spent {}), back to {}", s0 - min, w.actors[0].fighter.vitals.stamina.cur);
}

#[test]
#[ignore = "measurement: roll distance and i-frame share"]
fn measure_roll() {
    let mut w = World::duel(10.0);
    let roll = w.key("roll");
    w.press(ActionClass::Dodge);
    let t0 = w.tick;
    let mut iframe_ticks = 0;
    let spec = w.movesets.spec(w.sheet.key, roll).unwrap().clone();
    while w.actors[0].state.mov == roll {
        if spec.iframes_at(w.actors[0].out.clock.1).0 != 0 {
            iframe_ticks += 1;
        }
        w.idle();
    }
    let ticks = w.tick.0 - t0.0;
    println!(
        "roll: {:.2} m in {:.3} s ({ticks} ticks), i-frames {iframe_ticks} ticks = {:.0}%",
        w.actors[0].body.pos.length(),
        ticks as f32 * DT,
        100.0 * iframe_ticks as f32 / ticks as f32
    );
}

#[test]
#[ignore = "measurement: hit-stop total"]
fn measure_hit_stop() {
    let mut w = World::duel(1.0);
    w.press(ActionClass::Light);
    let (mut slow, mut lost) = (0, 0u32);
    for _ in 0..80 {
        w.idle();
        let r = w.actors[0].state.clock.rate;
        if r < RATE_ONE {
            slow += 1;
            lost += u32::from(RATE_ONE - r);
        }
    }
    println!("light1 hit-stop: {slow} slow ticks, {:.3} s of clock lost", lost as f32 / (f32::from(RATE_ONE) * 60.0));
}
