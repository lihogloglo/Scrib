//! The chooser: one pure step per character per fixed tick (action.md "The chooser").
//!
//! `step` decides; it doesn't move the body. The Sim owner's [`LocomotionIntent`] and launch
//! go out in [`ActionOutput`], and the shell runs one batched `KinematicMover::step_batch` for
//! every character afterwards (the mover batches its probes across characters), whose result
//! comes back as next tick's [`Sense`].

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_controller::{Gait, LocomotionIntent};

use crate::approach;
use crate::command::{ActionClass, ClassSet, Command};
use crate::grip::{LEFT, LedgeGrip, RIGHT, hold_target};
use sf_anim::{RootChain, RootPath, RootPose, WarpPlan};

use crate::ids::{CharacterId, ClockTime, MovesetKey, RuleKey, SlotKey, TagId, TargetId, Tick, TICK_HZ};
use crate::rule::{Category, CompiledRule, ExitOn, Launch, StickSide, Mode, ObstacleClass, Owner, Phase, PrevKey, ProbeNeed, RuleSet, SquareUp, WarpTarget};
use crate::state::{ActionClock, ActionState, CharacterParams, RATE_ONE, SeatState, WarpState};
use crate::windows::{Gates, SlotSource, WindowKind, buffering, gates, locate, open_windows};

/// Extra choose rounds when a move ends or exits on its first tick (bounded settle).
pub const MAX_SETTLE: usize = 4;

/// Cosine of the `need_forward` cone (60°).
const FORWARD_COS: f32 = 0.5;

/// Cosine of the traversal input cone (65°): a `need_forward` rule with an obstacle probe needs the
/// stick within this of the face's inward normal, whatever the body faces (Uncharted, TLOU, ALS gate
/// a mantle on the input toward the wall, not the facing). 60° approaches mantle; a 70°+ glancing
/// run along a face does not.
const TRAVERSE_INPUT_COS: f32 = 0.422_618_27;

/// What the interaction query found in front of the character (the shell's cone or ray from
/// `Command.view`; `sf_interaction` points from C1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Focus {
    pub target: TargetId,
    pub tag: TagId,
}

/// The body after last tick's mover step, plus this tick's world queries.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sense {
    pub grounded: bool,
    /// m/s, up positive.
    pub vertical_speed: f32,
    /// Horizontal speed, m/s.
    pub planar_speed: f32,
    /// Facing, radians (yaw 0 faces +Z).
    pub yaw: f32,
    pub focus: Option<Focus>,
    /// The context probe's obstacle ahead (the shell asks along the stick when it could matter).
    pub obstacle: Option<Obstacle>,
    /// Request classes the start gate refuses now (combat: `CombatRules::can_start` false). A press
    /// of one that would start a move is dropped with [`Refused::Gate`].
    pub refused: ClassSet,
    /// The lock-on target's position relative to the feet (world axes): Clip `Turn` windows aim at it.
    pub lock_target: Option<Vec3>,
    /// The attack's soft-aim target relative to the feet (world axes; combat.md "Lock-on and
    /// camera", C4): with no lock, an Attack's `Turn` windows aim at it.
    pub soft_target: Option<Vec3>,
    /// The interaction slot this character holds the claim on (`IN2a`): the interaction shell fills
    /// it only while this user holds the claim (the step checks no claims). [`WarpTarget::Slot`]
    /// enters it, [`Owner::Seated`] holds it while it stays sensed.
    pub seat: Option<SeatSense>,
}

/// A claimed interaction slot (a seat, a bench end, a workstation), as the interaction shell
/// senses it. Poses are relative to the feet this tick, world axes, world yaw (like [`Obstacle`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeatSense {
    pub target: TargetId,
    /// The slot index on the target (`sf_interaction::SlotRef::slot`).
    pub slot: u8,
    /// The slot frame the enter clip's end lands on.
    pub enter: RootPose,
    /// Where standing up lands (the shell's `pick_exit`).
    pub exit: RootPose,
}

/// An obstacle ahead, classified by the shell's context probe. Positions are relative to the
/// feet, world axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstacle {
    pub class: ObstacleClass,
    /// The lip (front top edge), from the feet.
    pub lip: Vec3,
    /// The face normal, toward the character.
    pub normal: Vec3,
    /// Top above the feet, m.
    pub height: f32,
    /// Depth from the lip along the probe direction, m (`f32::INFINITY`: deeper than the scan).
    pub depth: f32,
    /// Room above the top, m.
    pub clearance: f32,
    /// The floor past a vault's far face, from the feet (`sf_controller::ContextHit::landing`).
    pub landing: Option<Vec3>,
    /// A vault's clear run-out past the lip along the face's inward normal, m (0: not measured).
    pub room: f32,
}

/// Shared, read-only inputs of a step.
#[derive(Clone, Copy)]
pub struct StepCtx<'a> {
    pub rules: &'a RuleSet,
    pub slots: &'a dyn SlotSource,
    pub params: &'a CharacterParams,
    /// The character, for events.
    pub source: CharacterId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    MoveStart(RuleKey),
    MoveEnd(RuleKey),
    /// Left the ground by a launch.
    Launch,
    /// Touched down.
    Land,
    /// "Press to use": a request to activate the focus target. Host-validated in co-op.
    Activate(TagId),
    /// The enter warp landed in slot `.0` of the event's target ([`Owner::Seated`] began): the
    /// interaction shell occupies the claim. Once per seating.
    SeatEntered(u8),
    /// The exit move ended (or anything else took the body out of the slot): the shell releases
    /// it. Once per [`EventKind::SeatEntered`]; an enter cut short before landing sends neither.
    SeatExited(u8),
    /// The offered slot `.0` of the event's target ([`crate::Entry::offered`]) started no move
    /// (refused: too far, not grounded, another move running): the shell releases the claim. Once
    /// per offer; the step never retries while the same slot stays sensed.
    SeatRefused(u8),
}

/// Gameplay events are data: sent, replayed, de-duplicated by (tick, source, kind).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionEvent {
    pub tick: Tick,
    pub source: CharacterId,
    pub target: Option<TargetId>,
    pub kind: EventKind,
}

/// The entry predicate that refused a requested rule (course tests, debug overlay).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refused {
    Mode,
    State,
    Forward,
    Speed,
    VSpeed,
    Focus,
    /// No obstacle of the rule's classes and height band.
    Probe,
    /// A Warp rule's slots have no root path.
    NoRootPath,
    /// A `Vault` warp's slot clip has no hand contact to plant on the top.
    NoContact,
    /// The warp correction is over the rule's `refuse_ratio`.
    MaxCorrection,
    /// The start gate refused the class ([`Sense::refused`]: no stamina).
    Gate,
    /// Another moveset is equipped.
    Moveset,
    /// The current move isn't one the rule starts from ([`crate::Entry::from`]).
    Prev,
    /// Not locked on, or the stick is on another side of the target ([`crate::Entry::lock`]).
    Lock,
    /// A [`WarpTarget::Slot`] move with no interaction slot sensed ([`Sense::seat`]).
    Slot,
    /// An [`Owner::Approach`] move was cut short: the stick past the deadzone, a Jump, another move
    /// forced in, or its exit rule refusing on arrival with no reason of its own.
    ApproachCancelled,
    /// An [`Owner::Approach`] move ran its `timeout_s` without arriving.
    ApproachTimeout,
    /// An [`Owner::Approach`] move's target is further than its `max_m` at the start.
    ApproachTooFar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refusal {
    pub rule: RuleKey,
    pub reason: Refused,
}

/// "Press to use" prompt for the UI: what Interact would do now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prompt {
    pub target: TargetId,
    pub tag: TagId,
    /// The rule a press would start.
    pub rule: RuleKey,
    /// A press now is accepted (else the current move blocks it: grey the prompt).
    pub ready: bool,
}

/// Everything presentation and the mover read. Reused across ticks (no per-tick allocation once
/// warm).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionOutput {
    pub tick: Tick,
    pub rule: RuleKey,
    pub slot: Option<SlotKey>,
    pub chain: u8,
    /// Seconds into the current slot.
    pub playhead: f32,
    /// A move started this tick (the first response).
    pub started: bool,
    /// Sim owner: the mover's intent.
    pub intent: LocomotionIntent,
    /// Vertical speed to set before the mover step, m/s.
    pub launch: Option<f32>,
    pub prompt: Option<Prompt>,
    /// What the current move's `Accept` windows latch at the end of this tick (empty in a free
    /// move): the input shell resolves a shared tap/hold button as its tap on press while its tap
    /// class is in here, since its hold can't start mid-move ([`crate::InputSampler::set_buffering`]).
    pub buffering: ClassSet,
    /// Warp owner: where the body is at the end of this tick and the chain progress the graph plays.
    pub warp: Option<WarpOutput>,
    /// Clip owner: this tick's root motion and yaw.
    pub clip: Option<ClipOutput>,
    /// The action-clock interval this tick plays, `(t0, t1]` (equal under a full hit-stop). Chain
    /// time of the current move; windows, root motion and sweeps use it.
    pub clock: (ClockTime, ClockTime),
    pub events: Vec<ActionEvent>,
    pub refusals: Vec<Refusal>,
}

/// A Warp move's pose at the end of a tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WarpOutput {
    /// Feet position and yaw relative to the feet on the move's start tick (world axes).
    pub root: RootPose,
    /// Seconds into the slot chain (`GraphInstance::set_slot_progress`).
    pub progress: f32,
    /// The move lands standing on this tick (its rule's `stand` reached): no planar momentum
    /// carries out of it.
    pub settle: bool,
}

/// A Clip move's motion this tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClipOutput {
    /// Planar root translation this tick, world axes, m (the slot root track's delta x
    /// `root_scale`, rotated by the gameplay yaw). The shell moves the capsule by it, gravity on.
    pub delta: Vec3,
    /// The gameplay yaw at the end of the tick, radians.
    pub yaw: f32,
    /// Clip seconds into the slot chain at the end of the tick (move time x `rate`;
    /// `GraphInstance::set_slot_progress`).
    pub progress: f32,
}

impl ActionOutput {
    fn reset(&mut self, tick: Tick) {
        self.tick = tick;
        self.started = false;
        self.launch = None;
        self.prompt = None;
        self.warp = None;
        self.clip = None;
        self.events.clear();
        self.refusals.clear();
    }
}

/// Per-tick facts the entry predicates read (independent flags, not a state machine).
#[allow(clippy::struct_excessive_bools)]
struct Facts {
    grounded: bool,
    coyote: bool,
    landed: bool,
    vspeed: f32,
    speed: f32,
    forward: bool,
    /// The stick's direction, unit (active only).
    stick: Option<Vec2>,
    mode: Mode,
    focus: Option<Focus>,
    obstacle: Option<Obstacle>,
    body_height: f32,
    moveset: Option<MovesetKey>,
    /// The stick's side of the lock target (locked and the stick active).
    lock_side: Option<StickSide>,
}

impl Facts {
    fn has(&self, p: Phase) -> bool {
        match p {
            Phase::Grounded => self.grounded,
            Phase::Coyote => self.coyote,
            Phase::AirRising => !self.grounded && self.vspeed > 0.0,
            Phase::AirFalling => !self.grounded && self.vspeed <= 0.0,
            Phase::Landed => self.landed,
        }
    }

    /// `need_forward`: the stick toward the sensed face for an obstacle rule (the traversal input
    /// cone), else within the forward cone of the facing.
    fn forward_ok(&self, e: &crate::Entry) -> bool {
        match (&e.probe, self.obstacle, self.stick) {
            (Some(ProbeNeed::Obstacle { .. }), Some(o), Some(stick)) => {
                let into = Vec2::new(-o.normal.x, -o.normal.z).normalize_or_zero();
                into != Vec2::ZERO && stick.dot(into) >= TRAVERSE_INPUT_COS
            }
            _ => self.forward,
        }
    }

    fn entry(&self, r: &CompiledRule) -> Result<(), Refused> {
        let e = &r.rule.entry;
        let within = |band: Option<(f32, f32)>, v: f32| band.is_none_or(|(lo, hi)| (lo..=hi).contains(&v));
        if r.moveset.is_some() && r.moveset != self.moveset {
            return Err(Refused::Moveset);
        }
        if !e.modes.is_empty() && !e.modes.contains(&self.mode) {
            return Err(Refused::Mode);
        }
        if !e.states.is_empty() && !e.states.iter().any(|&s| self.has(s)) {
            return Err(Refused::State);
        }
        if e.need_forward && !self.forward_ok(e) {
            return Err(Refused::Forward);
        }
        if e.lock.is_some() && e.lock != self.lock_side {
            return Err(Refused::Lock);
        }
        if !within(e.speed, self.speed) {
            return Err(Refused::Speed);
        }
        if !within(e.vspeed, self.vspeed) {
            return Err(Refused::VSpeed);
        }
        if let Some(tags) = &r.focus {
            match self.focus {
                Some(f) if tags.is_empty() || tags.contains(&f.tag) => {}
                _ => return Err(Refused::Focus),
            }
        }
        if let Some(ProbeNeed::Obstacle { classes, height: (lo, hi) }) = &e.probe {
            let h = self.body_height.max(1e-3);
            match self.obstacle {
                Some(o) if classes.contains(&o.class) && (lo * h..=hi * h).contains(&o.height) => {}
                _ => return Err(Refused::Probe),
            }
        }
        Ok(())
    }
}

fn forced(r: &CompiledRule) -> bool {
    r.rule.category.framework_only() && r.classes.is_empty() && !r.force_only() && !r.rule.entry.offered
}

fn automatic(r: &CompiledRule) -> bool {
    r.classes.is_empty() && !forced(r) && !r.force_only() && !r.rule.entry.offered
}

/// `r` may start from the current move `cur` ([`crate::Entry::from`]).
fn prev_ok(r: &CompiledRule, cur: Option<&CompiledRule>) -> bool {
    r.from.is_empty()
        || r.from.iter().any(|p| match p {
            PrevKey::Any => true,
            PrevKey::Loco => cur.is_none_or(|c| c.rule.category == Category::Locomotion && automatic(c)),
            PrevKey::Rule(k) => cur.is_some_and(|c| c.key == *k),
        })
}

/// Advances one character by one fixed tick: command → clock → buffer → candidates → pick →
/// start (+ bounded settle) → Sim intent → output.
#[allow(clippy::too_many_lines)]
pub fn step(ctx: &StepCtx<'_>, state: &mut ActionState, cmd: &Command, sense: &Sense, out: &mut ActionOutput) {
    let (rules, p, tick) = (ctx.rules, ctx.params, cmd.tick);
    out.reset(tick);
    let event = |target, kind| ActionEvent { tick, source: ctx.source, target, kind };

    let (landed, impact, coyote) = body_edges(state, sense, p, tick);
    out.events.extend(landed.then(|| event(None, EventKind::Land)));
    // A claimed slot sensed new this tick is an offer (`IN2b1`), answered this tick.
    let sensed = sense.seat.map(|s| (s.target, s.slot));
    let offer = sensed.filter(|_| sensed != state.seat_sensed);
    state.seat_sensed = sensed;

    // 1. Command: modes.
    let stick_active = cmd.stick.length() > p.deadzone;
    update_mode(&mut state.mode.mode, cmd, stick_active, sense.refused);

    // 2. Clock.
    state.clock.advance();
    let mut cur = rules.index_of(state.mov);
    // Seated (IN2a): requests don't latch; the stick, a Jump or an Interact press (or the slot
    // sense going away) is the exit input ([`ExitOn::Stand`]).
    let seated = cur.is_some_and(|i| rules.at(i).rule.owner == Owner::Seated);
    let stand = seated
        && (stick_active || cmd.buttons.pressed.contains(ActionClass::Jump) || cmd.buttons.pressed.contains(ActionClass::Interact) || seat_lost(state.seat.as_ref(), sense.seat.as_ref()));
    let approach = approach::running(ctx, state, cur, cmd, sense, stick_active);
    let gates_of = |cur: Option<usize>, state: &ActionState| {
        cur.map_or(Gates { ended: true, ..Gates::OPEN }, |i| {
            let r = rules.at(i);
            if stood(r, state.clock.elapsed) { Gates { ended: true, ..Gates::OPEN } } else { gates(r, ctx.slots, state.clock.elapsed) }
        })
    };

    // 3. Buffer: latch inside accept; an airborne Jump goes to the jump buffer instead.
    let g = gates_of(cur, state);
    let g = if seated { Gates { accept: ClassSet::EMPTY, ..g } } else { g };
    let pressed = if seated { ClassSet::EMPTY } else { cmd.buttons.pressed.intersection(ClassSet::LATCHABLE) };
    for c in pressed.iter() {
        if c == ActionClass::Jump && !sense.grounded && !coyote {
            state.timers.jump_pressed = Some(tick);
        } else if g.accept.contains(c) {
            state.buffer.latched.insert(c);
        }
    }
    // A held hold class (Guard) latches each tick like a fresh press while it is accepted and not
    // refused (no refusal every tick it stays down): a guard held through an attack rises at the
    // attack's `Cancel(Guard)`, and again after a reaction. Released, it leaves the buffer.
    let holding = cmd.buttons.held.intersection(ClassSet::HOLD);
    state.buffer.latched = state.buffer.latched.union(holding.intersection(g.accept).intersection(ClassSet(!sense.refused.0)));
    state.buffer.latched = state.buffer.latched.intersection(ClassSet(!(ClassSet::HOLD.0 & !holding.0)));
    let jump_buffered = sense.grounded && state.timers.jump_pressed.is_some_and(|t| tick.since(t) <= u32::from(p.jump_buffer_ticks));

    let (stick, facing) = (cmd.stick, Vec2::new(sense.yaw.sin(), sense.yaw.cos()));
    let facts = Facts {
        grounded: sense.grounded,
        coyote,
        landed,
        vspeed: impact,
        speed: sense.planar_speed,
        forward: stick_active && stick.normalize_or_zero().dot(facing) >= FORWARD_COS,
        stick: stick_active.then(|| stick.normalize_or_zero()),
        mode: state.mode.mode,
        focus: sense.focus,
        obstacle: sense.obstacle,
        body_height: p.height,
        moveset: state.mode.moveset,
        lock_side: lock_dir(sense).filter(|_| stick_active).and_then(|d| StickSide::of(d, stick)),
    };

    // 4-8. Candidates, pick, start; settle while the new move ends or exits at once. A forced
    // reaction goes first, whatever the windows say.
    let mut started = [usize::MAX; MAX_SETTLE + 2];
    let mut exit_refused = None;
    if let Some(i) = state.forced.take().and_then(|f| rules.index_of(f.rule)) {
        if let Some(old) = cur {
            out.events.push(event(None, EventKind::MoveEnd(rules.at(old).key)));
        }
        start(ctx, rules.at(i), state, sense, facts.mode, None, out);
        started[MAX_SETTLE + 1] = i;
        cur = Some(i);
    }
    for round in 0..=MAX_SETTLE {
        let g = gates_of(cur, state);
        let (exit, open) = match cur {
            Some(i) => {
                let r = rules.at(i);
                let fresh = started.contains(&i);
                let exit = r.exits.iter().find(|(on, _)| match on {
                    ExitOn::End => g.ended,
                    ExitOn::Touchdown => landed && !fresh,
                    ExitOn::Airborne => !sense.grounded && !fresh,
                    ExitOn::Stand => stand && !fresh,
                    ExitOn::Arrived => approach.is_some_and(|a| a.arrived(i)) && !fresh,
                });
                // An automatic move lasts while its entry holds; a hold move while its class is held.
                let released = r.holds() && !fresh && cmd.buttons.held.intersection(r.classes).is_empty();
                let quit = approach.is_some_and(|a| a.quits(i)) && !fresh;
                let lapsed = released || quit || (automatic(r) && !fresh && facts.entry(r).is_err());
                (exit.copied(), g.ended || exit.is_some() || lapsed)
            }
            None => (None, true),
        };
        let cancel = if open { ClassSet::ALL } else { g.cancel };
        let mut firing = state.buffer.latched.intersection(cancel);
        if jump_buffered && cancel.contains(ActionClass::Jump) {
            firing.insert(ActionClass::Jump);
        }
        let auto_ok = open || (cancel.contains(ActionClass::Move) && stick_active);
        // An offer fires where an Interact press would, without latching.
        let offering = offer.is_some() && cancel.contains(ActionClass::Interact);
        let exit_target = exit.and_then(|(_, to)| to);

        let mut winner = None;
        for &i in rules.priority() {
            let r = rules.at(i);
            let by = r.classes.intersection(firing);
            let requested = !by.is_empty() || (offering && r.rule.entry.offered);
            // The current move restarts once it is over (ended, exited or lapsed), or earlier when
            // a press it takes fires through its own cancel window (a roll chained into a roll).
            if (Some(i) == cur && !open && !requested) || started.contains(&i) || state.timers.redo.iter().any(|&(k, _)| k == r.key) {
                continue;
            }
            let other = forced(r) || exit_target == Some(i) || (auto_ok && (automatic(r) || r.rule.entry.auto_in.contains(&facts.mode)));
            if !(requested || other) {
                continue;
            }
            let cur_rule = cur.map(|c| rules.at(c));
            let gated = !by.is_empty() && !other && by.intersection(sense.refused) == by;
            let checked = facts
                .entry(r)
                .and(if prev_ok(r, cur_rule) { Ok(()) } else { Err(Refused::Prev) })
                .and(if gated { Err(Refused::Gate) } else { Ok(()) })
                .and_then(|()| approach::check(ctx, r, sense))
                .and_then(|()| plan_warp(ctx, r, sense, state.seat.as_ref()));
            match checked {
                Ok(plan) => {
                    winner = Some((i, plan));
                    break;
                }
                Err(reason) if requested => out.refusals.push(Refusal { rule: r.key, reason }),
                Err(reason) if exit_target == Some(i) => exit_refused = Some(reason),
                Err(_) => {}
            }
        }
        let Some((i, plan)) = winner else {
            // Requests wait for windows, not for predicates: one that could fire now but no
            // rule took is dropped (its refusal is logged), so a stale press never fires later.
            state.buffer.latched = state.buffer.latched.intersection(ClassSet(!firing.0));
            if firing.contains(ActionClass::Jump) {
                state.timers.jump_pressed = None;
            }
            break;
        };
        if let Some(old) = cur {
            out.events.push(event(None, EventKind::MoveEnd(rules.at(old).key)));
        }
        start(ctx, rules.at(i), state, sense, facts.mode, plan, out);
        started[round] = i;
        cur = Some(i);
    }
    if let Some((target, slot)) = offer
        && !started.iter().any(|&i| i != usize::MAX && rules.at(i).rule.entry.offered)
    {
        out.events.push(event(Some(target), EventKind::SeatRefused(slot)));
    }
    if let Some((a, refusal)) = approach::answer(approach, &started, exit_refused) {
        out.events.push(event(Some(a.target), EventKind::SeatRefused(a.slot)));
        out.refusals.push(refusal);
    }

    // 9. Move (Sim owner): the mover's intent, slowed with the clock under hit-stop. Locked on,
    // the body faces the target, standing still too.
    let lock = lock_facing(sense, facts.mode);
    out.intent = match (stick_active, cur.is_none_or(|i| steers(rules.at(i)))) {
        (true, true) => intent(p, facts.mode, stick, sense.yaw, lock),
        (false, true) => rest(facts.mode, lock),
        (_, false) => rest(facts.mode, None),
    };
    if let Some(walk) = approach::intent(ctx, state, cur, sense, facts.mode) {
        out.intent = walk;
    }
    if state.clock.rate < RATE_ONE {
        out.intent.desired_velocity *= f32::from(state.clock.rate) / f32::from(RATE_ONE);
    }
    let t0 = state.clock.elapsed;
    out.clock = (t0, ClockTime(t0.0.saturating_add(u32::from(state.clock.rate))));
    let fresh = out.started;
    write_clip(ctx, cur, cmd, sense, fresh, out);

    write_output(ctx, state, cur, &facts, sense.yaw, out);
}

/// The stick steers the Sim owner unless the rule says no; never a Warp or Clip owner.
fn steers(r: &CompiledRule) -> bool {
    r.rule.steer && matches!(r.rule.owner, Owner::Sim)
}

/// Clip owner: yaw (square-up on the first tick, then `Turn` toward the lock target, else an
/// attack's soft-aim target, else the stick, unless `NoTurn` is open, plus the clip's own root
/// yaw) and the planar root delta over the tick's clock interval x `root_scale`. With
/// [`Command::face_view`] (first person) an attack or a guard ([`faces_view`]) squares up to the
/// view and its free aim is the view, not the stick: the body swings where the player looks, and a
/// held guard follows the view at its `Turn` window's rate (the capped turn-with-view, combat.md
/// "C2b"). A hold move's looping last slot wraps its progress ([`looped_progress`]).
fn write_clip(ctx: &StepCtx<'_>, cur: Option<usize>, cmd: &Command, sense: &Sense, fresh: bool, out: &mut ActionOutput) {
    let Some(r) = cur.map(|i| ctx.rules.at(i)) else { return };
    let Owner::Clip { root_scale, square_up, rate } = r.rule.owner else { return };
    let (t0, t1) = out.clock;
    let attack = r.rule.category == Category::Attack;
    let view = (cmd.face_view && faces_view(r.rule.category)).then(|| Vec2::new(cmd.view.yaw.sin(), cmd.view.yaw.cos()));
    let stick = (cmd.stick.length() > ctx.params.deadzone).then_some(cmd.stick);
    let free = view.or(stick);
    let mut yaw = sense.yaw;
    if fresh {
        let lock = (square_up == SquareUp::Lock).then(|| lock_dir(sense)).flatten();
        let to = match square_up {
            SquareUp::Keep => view,
            SquareUp::Stick | SquareUp::Lock => lock.or(free),
        };
        if let Some(d) = to {
            yaw = d.x.atan2(d.y);
        }
    }
    let (mut turn, mut no_turn) = (None, false);
    for w in open_windows(r, ctx.slots, t0) {
        match w.kind {
            WindowKind::Turn(rate) => turn = Some(turn.map_or(rate, |t: u16| t.max(rate))),
            WindowKind::NoTurn => no_turn = true,
            _ => {}
        }
    }
    let soft = sense.soft_target.filter(|_| attack).map(|v| Vec2::new(v.x, v.z)).filter(|v| v.length_squared() > 1e-6);
    let aim = lock_dir(sense).or(soft).or(free);
    if let (Some(rate), Some(aim), false) = (turn, aim, no_turn) {
        #[allow(clippy::cast_precision_loss)]
        let max = f32::from(rate).to_radians() * (t1.0 - t0.0) as f32 / (1000.0 * TICK_HZ as f32);
        let want = aim.x.atan2(aim.y);
        let diff = (want - yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        yaw += diff.clamp(-max, max);
    }
    // Root tracks and slot progress run in clip time: move time x `rate`.
    let (c0, c1) = (t0.secs() * rate, t1.secs() * rate);
    let (local, dyaw) = clip_root(r, ctx.slots, |path| {
        let (a, b) = (path.root_at(c0), path.root_at(c1));
        (glam::Quat::from_rotation_y(-a.yaw) * (b.position - a.position), b.yaw - a.yaw)
    })
    .unwrap_or((Vec3::ZERO, 0.0));
    yaw += dyaw;
    let delta = glam::Quat::from_rotation_y(yaw) * Vec3::new(local.x, 0.0, local.z) * root_scale;
    let duration = clip_root(r, ctx.slots, |p| p.duration()).unwrap_or(f32::INFINITY);
    let progress = looped_progress(r, ctx.slots, c1).unwrap_or(c1.min(duration));
    out.clip = Some(ClipOutput { delta, yaw, progress });
}

/// Categories whose Clip moves face the view in first person ([`Command::face_view`]): attacks
/// (square up and aim the swing) and guards (follow the view at the `Turn` rate).
const fn faces_view(c: Category) -> bool {
    matches!(c, Category::Attack | Category::Guard)
}

/// A hold move whose chain ends in a looping slot ([`crate::SlotInfo::length`] `None`, a guard's held
/// pose): chain progress (clip seconds) wrapped inside that slot's clip, whose length is its root
/// path's duration; the slots before it play once. `None` when the last present slot doesn't loop,
/// a present slot has no root path, or the clock hasn't reached the loop.
fn looped_progress(r: &CompiledRule, slots: &dyn SlotSource, c: f32) -> Option<f32> {
    let present: Vec<SlotKey> = r.slots.iter().copied().filter(|k| slots.slot(*k).is_some()).collect();
    let (&last, before) = present.split_last()?;
    if !r.holds() || slots.slot(last)?.length.is_some() {
        return None;
    }
    let mut prior = 0.0;
    for k in before {
        prior += slots.root(*k)?.duration();
    }
    let len = slots.root(last)?.duration();
    (len > 1e-6 && c > prior).then(|| prior + (c - prior).rem_euclid(len))
}

/// Runs `f` on the root path of the rule's present slots (dropped slots skipped); `None` when a
/// present slot has no root track or no slot is present.
fn clip_root<R>(r: &CompiledRule, slots: &dyn SlotSource, f: impl FnOnce(&dyn RootPath) -> R) -> Option<R> {
    let parts = r.slots.iter().filter(|k| slots.slot(**k).is_some()).map(|k| slots.root(*k)).collect::<Option<Vec<_>>>()?;
    match parts.as_slice() {
        [] => None,
        [one] => Some(f(*one)),
        _ => Some(f(&RootChain::new(parts))),
    }
}

/// Warp owner: the planned root at the end of this tick (the clock one tick on, clamped to the
/// chain), so the last tick of the move lands on the target.
fn write_warp(ctx: &StepCtx<'_>, state: &ActionState, cur: Option<usize>, out: &mut ActionOutput) {
    let (Some(r), Some(w)) = (cur.map(|i| ctx.rules.at(i)), state.warp) else { return };
    if !matches!(r.rule.owner, Owner::Warp { .. }) {
        return;
    }
    let t = ClockTime(state.clock.elapsed.0.saturating_add(u32::from(state.clock.rate))).secs();
    let stands = matches!(r.rule.owner, Owner::Warp { stand: Some(_), .. });
    out.warp = with_root_path(r, ctx.slots, |path| {
        let t = t.min(path.duration());
        WarpOutput { root: w.plan.root_at(path, t), progress: t, settle: stands && t >= path.duration() }
    });
}

/// A Warp rule whose `stand` time the clock has reached: the move is over (feet planted on top).
fn stood(r: &CompiledRule, t: ClockTime) -> bool {
    matches!(r.rule.owner, Owner::Warp { stand: Some(s), .. } if t.secs() >= s)
}

/// Runs `f` on the rule's slot chain root path (one slot: its path; several: a [`RootChain`]),
/// cut at a Warp rule's `stand` time. `None` if a slot has no root path.
pub(crate) fn with_root_path<R>(r: &CompiledRule, slots: &dyn SlotSource, f: impl FnOnce(&dyn RootPath) -> R) -> Option<R> {
    let stand = match r.rule.owner {
        Owner::Warp { stand, .. } => stand,
        _ => None,
    };
    let f = |path: &dyn RootPath| match stand {
        Some(end) => f(&StandAt { path, end }),
        None => f(path),
    };
    if let [one] = r.slots.as_slice() {
        return slots.root(*one).map(f);
    }
    let parts = r.slots.iter().map(|k| slots.root(*k)).collect::<Option<Vec<_>>>()?;
    Some(f(&RootChain::new(parts)))
}

/// A root path cut at `end` (a Warp rule's `stand`): its root holds there, the warp window is
/// clamped into it (a window starting past it spans the whole cut path), contacts pass through.
struct StandAt<'a> {
    path: &'a dyn RootPath,
    end: f32,
}

impl RootPath for StandAt<'_> {
    fn duration(&self) -> f32 {
        self.end.min(self.path.duration())
    }

    fn root_at(&self, t: f32) -> RootPose {
        self.path.root_at(t.min(self.duration()))
    }

    fn warp_window(&self) -> Option<(f32, f32)> {
        let d = self.duration();
        self.path.warp_window().filter(|&(a, _)| a < d).map(|(a, b)| (a, b.min(d)))
    }

    fn contacts(&self) -> &[sf_anim::Contact] {
        self.path.contacts()
    }
}

/// A Warp rule's plan: [`plan_slot`] for the seat targets, else [`plan_obstacle`]. Other owners
/// plan nothing.
fn plan_warp(ctx: &StepCtx<'_>, r: &CompiledRule, sense: &Sense, seat: Option<&SeatState>) -> Result<Option<WarpState>, Refused> {
    let Owner::Warp { target, profile, refuse_ratio, .. } = r.rule.owner else {
        return Ok(None);
    };
    match target {
        WarpTarget::Slot | WarpTarget::SlotExit => plan_slot(ctx, r, target, (profile, refuse_ratio), sense, seat).map(Some),
        WarpTarget::LedgeTop { .. } | WarpTarget::Vault => plan_obstacle(ctx, r, target, profile, refuse_ratio, sense).map(Some),
    }
}

/// An interaction slot's warp, from the feet at their current yaw: `Slot` onto the sensed enter
/// pose (refused with none, or over `refuse_ratio`); `SlotExit` onto the sensed exit, else the
/// pose the user entered from, else the clip's own end (never refused).
fn plan_slot(
    ctx: &StepCtx<'_>,
    r: &CompiledRule,
    target: WarpTarget,
    (profile, refuse_ratio): (sf_anim::WarpProfile, f32),
    sense: &Sense,
    seat: Option<&SeatState>,
) -> Result<WarpState, Refused> {
    let start = RootPose::new(Vec3::ZERO, sense.yaw);
    let plan = if target == WarpTarget::Slot {
        let s = sense.seat.ok_or(Refused::Slot)?;
        with_root_path(r, ctx.slots, |path| WarpPlan::try_new(path, start, s.enter, profile, refuse_ratio))
            .ok_or(Refused::NoRootPath)?
            .map_err(|_| Refused::MaxCorrection)?
    } else {
        let held = match (seat, sense.seat) {
            (Some(s), Some(x)) if x.target == s.target && x.slot == s.slot => Some(x.exit),
            (Some(s), _) => Some(s.back),
            (None, x) => x.map(|x| x.exit),
        };
        with_root_path(r, ctx.slots, |path| WarpPlan::new(path, start, held.unwrap_or_else(|| start.then(path.root_at(path.duration()))), profile))
            .ok_or(Refused::NoRootPath)?
    };
    Ok(WarpState { plan, grip: None })
}

/// An obstacle warp (validators `EndPoseFree` via the probe's room check, `MaxCorrection`): start
/// at the feet facing into the face; `LedgeTop`: target on top `inset` past the lip (kept a
/// radius inside a shallower top); `Vault`: [`Vault::plan`].
fn plan_obstacle(ctx: &StepCtx<'_>, r: &CompiledRule, target: WarpTarget, profile: sf_anim::WarpProfile, refuse_ratio: f32, sense: &Sense) -> Result<WarpState, Refused> {
    let o = sense.obstacle.ok_or(Refused::Probe)?;
    let into = Vec3::new(-o.normal.x, 0.0, -o.normal.z).normalize_or_zero();
    if into == Vec3::ZERO {
        return Err(Refused::Probe);
    }
    let yaw = into.x.atan2(into.z);
    let lip = Vec3::new(o.lip.x, o.height, o.lip.z);
    let start = RootPose::new(Vec3::ZERO, yaw);
    let (plan, mirror) = match target {
        WarpTarget::LedgeTop { inset } => {
            let room = if o.depth.is_finite() { (o.depth - ctx.params.radius).max(0.0) } else { inset };
            let top = lip + into * inset.min(room);
            let plan = with_root_path(r, ctx.slots, |path| WarpPlan::try_new(path, start, RootPose::new(top, yaw), profile, refuse_ratio))
                .ok_or(Refused::NoRootPath)?
                .map_err(|_| Refused::MaxCorrection)?;
            (plan, true)
        }
        WarpTarget::Vault => {
            let vault = Vault { o: &o, lip, into, start, radius: ctx.params.radius };
            let (plan, ratio) = with_root_path(r, ctx.slots, |path| vault.plan(path, profile)).ok_or(Refused::NoRootPath)??;
            if ratio > refuse_ratio {
                return Err(Refused::MaxCorrection);
            }
            (plan, false)
        }
        WarpTarget::Slot | WarpTarget::SlotExit => return Err(Refused::Probe),
    };
    // Hands on the lip (B2b), from the slot clip's baked hand contacts.
    let grip = with_root_path(r, ctx.slots, |path| LedgeGrip::plan(path, &plan, lip, into, mirror)).flatten();
    Ok(WarpState { plan, grip })
}

/// A vault's measured obstacle, for [`Vault::plan`].
struct Vault<'a> {
    o: &'a Obstacle,
    /// The lip on top, from the feet.
    lip: Vec3,
    /// The face's flat inward direction, unit.
    into: Vec3,
    /// The feet, facing `into`.
    start: RootPose,
    radius: f32,
}

impl Vault<'_> {
    /// Two legs, as GASP's vault warps onto its front ledge then its back floor: the clip's first
    /// hand contact lands on its hold on the lip ([`hold_target`], where the hand IK holds it) by
    /// the plant; the root moves as baked while the hand is down; then the clip's own landing
    /// (carried by the first leg) is pulled in to the probed run-out, a radius short of what it
    /// hit, and onto the far floor. Refused without a far floor or a hand contact.
    ///
    /// Close to the face (the hold nearer than the clip's hand reaches at the plant) the first leg
    /// stops the root at the takeoff instead of running it backwards, and the hand IK reaches the
    /// rest (at most [`VAULT_REACH_SLACK`]). Returns the plan and its [`Vault::ratio`].
    fn plan(&self, path: &dyn RootPath, profile: sf_anim::WarpProfile) -> Result<(WarpPlan, f32), Refused> {
        let (o, into) = (self.o, self.into);
        let landing = o.landing.ok_or(Refused::Probe)?;
        let c = path
            .contacts()
            .iter()
            .filter(|c| c.kind == sf_anim::ContactKind::Hand)
            .min_by(|a, b| a.start.total_cmp(&b.start))
            .ok_or(Refused::NoContact)?;
        let side = if c.side == sf_anim::Side::Left { LEFT } else { RIGHT };
        let at_plant = self.start.then(path.root_at(c.start));
        let baked_hand = at_plant.position + glam::Quat::from_rotation_y(at_plant.yaw) * c.local;
        let mut shift = hold_target(self.lip, into, side, c.local.x) - baked_hand;
        let behind = -(at_plant.position - self.start.position + shift).dot(into);
        if behind > VAULT_REACH_SLACK {
            return Err(Refused::MaxCorrection);
        }
        shift += into * behind.max(0.0);
        let end = self.start.then(path.root_at(path.duration())).position + shift;
        let past = (end - self.lip).dot(into);
        let reach = past.min(o.room - self.radius).max(o.depth + self.radius);
        let end = end + into * (reach - past);
        let target = RootPose::new(Vec3::new(end.x, landing.y, end.z), self.start.yaw);
        let plan = WarpPlan::with_waypoint(path, self.start, (c.start, c.end), at_plant.position + shift, target, profile);
        let ratio = self.ratio(&plan, at_plant.position - self.start.position);
        Ok((plan, ratio))
    }

    /// The worse leg's correction share (the rule's `refuse_ratio` bounds it), as
    /// [`WarpPlan::ratio`] but with the first leg's slowdown along the face's normal free down
    /// to a standstill (`run_up`: the baked root travel to the plant, world): a takeoff from close
    /// in shortens the run-up, which reads as braking into the plant, while the vertical and
    /// sideways parts and any stretch still count. The second leg is [`WarpPlan::ratio`]'s.
    fn ratio(&self, plan: &WarpPlan, run_up: Vec3) -> f32 {
        let share = |c: f32, d: f32| if d > 1e-4 { c / d } else if c <= 1e-3 { 0.0 } else { f32::INFINITY };
        let Some(w) = plan.waypoint else { return plan.ratio() };
        let along = w.correction.dot(self.into);
        let slow = along.clamp(-run_up.dot(self.into).max(0.0), 0.0);
        let first = share((w.correction - self.into * slow).length(), run_up.length());
        let second = share((plan.correction - w.correction).length(), (plan.baked_end.position - w.baked.1).length());
        first.max(second)
    }
}

/// How far short of the clip's own reach a vault's planted hand may hold the lip, m, when the
/// takeoff is closer to the face than the clip plants from (the hand IK bends the arm that much).
const VAULT_REACH_SLACK: f32 = 0.3;

/// 10. Output: move, slot chain, playhead, and the "press to use" prompt.
fn write_output(ctx: &StepCtx<'_>, state: &ActionState, cur: Option<usize>, facts: &Facts, yaw: f32, out: &mut ActionOutput) {
    write_warp(ctx, state, cur, out);
    let rules = ctx.rules;
    out.rule = state.mov;
    let Some(r) = cur.map(|i| rules.at(i)) else {
        (out.slot, out.chain, out.playhead, out.buffering) = (None, 0, 0.0, ClassSet::EMPTY);
        return;
    };
    out.buffering = if stood(r, state.clock.elapsed) { ClassSet::EMPTY } else { buffering(r, ctx.slots, state.clock.elapsed) };
    let at = locate(r, ctx.slots, state.clock.elapsed);
    out.slot = at.slot;
    out.chain = at.chain;
    out.playhead = at.local.secs();
    if r.rule.owner == Owner::Seated {
        // Pinned where the enter landed (the move's start feet), the loop clip wrapped on its root
        // path's length. Interact stands up here: no prompt, nothing buffers.
        let yaw = state.seat.map_or(yaw, |s| s.yaw);
        let len = at.slot.and_then(|k| ctx.slots.root(k)).map_or(0.0, RootPath::duration);
        let progress = if len > 1e-6 { out.playhead.rem_euclid(len) } else { out.playhead };
        out.warp = Some(WarpOutput { root: RootPose::new(Vec3::ZERO, yaw), progress, settle: false });
        out.buffering = ClassSet::EMPTY;
        return;
    }
    if let Some(f) = facts.focus {
        let ready = gates(r, ctx.slots, state.clock.elapsed).accept.contains(ActionClass::Interact);
        out.prompt = rules
            .iter()
            .filter(|o| o.focus.is_some() && o.key != r.key)
            .find(|o| facts.entry(o).is_ok())
            .map(|o| Prompt { target: f.target, tag: f.tag, rule: o.key, ready });
    }
}

/// Sprint held with the stick is free-run and overrides sneak, unless the gate refuses Sprint
/// (combat: winded); Sneak held is sneak (a toggle preference is the input layer's:
/// [`ActionClass::Sneak`]).
fn update_mode(mode: &mut Mode, cmd: &Command, stick_active: bool, refused: ClassSet) {
    *mode = if cmd.held(ActionClass::Sprint) && stick_active && !refused.contains(ActionClass::Sprint) {
        Mode::FreeRun
    } else if cmd.held(ActionClass::Sneak) {
        Mode::Sneak
    } else {
        Mode::Normal
    };
}

/// Body edges and air timers: returns (landed this tick, vertical speed for bands — the impact
/// speed when landed —, coyote time open).
fn body_edges(state: &mut ActionState, sense: &Sense, p: &CharacterParams, tick: Tick) -> (bool, f32, bool) {
    let landed = sense.grounded && !state.body.grounded;
    let left = !sense.grounded && state.body.grounded;
    let impact = if landed { state.body.air_vspeed } else { sense.vertical_speed };
    if sense.grounded {
        state.body.launched = false;
        state.timers.left_ground = None;
    } else {
        state.body.air_vspeed = sense.vertical_speed;
        if left && !state.body.launched {
            state.timers.left_ground = Some(tick);
        }
    }
    state.body.grounded = sense.grounded;
    state.timers.redo.retain(|&(_, ready)| ready > tick);
    let coyote = !sense.grounded
        && !state.body.launched
        && state.timers.left_ground.is_some_and(|t| tick.since(t) < u32::from(p.coyote_ticks));
    (landed, impact, coyote)
}

/// 7. Start: set the move, clear the buffer, apply first-tick launches, emit the first response.
#[allow(clippy::too_many_arguments)]
fn start(
    ctx: &StepCtx<'_>,
    r: &CompiledRule,
    state: &mut ActionState,
    sense: &Sense,
    mode: Mode,
    warp: Option<WarpState>,
    out: &mut ActionOutput,
) {
    let (p, tick) = (ctx.params, out.tick);
    let event = |target, kind| ActionEvent { tick, source: ctx.source, target, kind };
    state.mov = r.key;
    state.started = tick;
    // A new move restarts the clock but keeps its rate: a running hit-stop spans the change.
    state.clock = ActionClock { rate: state.clock.rate, ..ActionClock::default() };
    state.buffer.latched = ClassSet::EMPTY;
    state.warp = warp;
    if r.classes.contains(ActionClass::Jump) {
        state.timers.jump_pressed = None;
    }
    if r.rule.redo_ticks > 0 {
        state.timers.redo.push((r.key, Tick(tick.0.wrapping_add(u32::from(r.rule.redo_ticks)))));
    }
    state.approach = approach::on_start(r, sense);
    if let Some(left) = seat_chain(r, state, sense) {
        out.events.push(event(Some(left.target), EventKind::SeatExited(left.slot)));
    }
    let focus = r.focus.as_ref().and(sense.focus);
    out.events.push(event(focus.map(|f| f.target), EventKind::MoveStart(r.key)));
    if r.rule.owner == Owner::Seated
        && let Some(seat) = state.seat.as_mut().filter(|s| !s.entered)
    {
        seat.entered = true;
        out.events.push(event(Some(seat.target), EventKind::SeatEntered(seat.slot)));
    }
    if let Some(f) = focus {
        out.events.push(event(Some(f.target), EventKind::Activate(f.tag)));
    }
    if let Some(l) = r.rule.launch {
        let v = match l {
            Launch::JumpHeight => (2.0 * p.gravity * p.jump_height * p.profile(mode).jump_scale).max(0.0).sqrt(),
            Launch::Speed(v) => v,
        };
        out.launch = Some(v);
        state.body.launched = true;
        state.timers.left_ground = None;
        out.events.push(event(None, EventKind::Launch));
    }
    out.started = true;
}

/// The seat chain (`IN2a`) on a move start, before its `MoveStart`: a [`WarpTarget::Slot`] move
/// records the slot it enters ([`SeatState`]); a [`Owner::Seated`] or [`WarpTarget::SlotExit`] move
/// keeps it; any other move leaves the slot and returns it if the enter had landed (the caller
/// sends [`EventKind::SeatExited`], once).
fn seat_chain(r: &CompiledRule, state: &mut ActionState, sense: &Sense) -> Option<SeatState> {
    if matches!(r.rule.owner, Owner::Seated | Owner::Warp { target: WarpTarget::SlotExit, .. }) {
        return None;
    }
    let left = state.seat.take().filter(|s| s.entered);
    if let Owner::Warp { target: WarpTarget::Slot, .. } = r.rule.owner {
        state.seat = sense.seat.map(|s| SeatState {
            target: s.target,
            slot: s.slot,
            back: RootPose::new(-s.enter.position, sense.yaw),
            yaw: s.enter.yaw,
            entered: false,
        });
    }
    left
}

/// The held slot is no longer sensed (gone, or the shell reports another one): a seated move
/// stands up at once. No seat memory (a loaded save) counts as lost.
fn seat_lost(seat: Option<&SeatState>, sensed: Option<&SeatSense>) -> bool {
    match (seat, sensed) {
        (Some(s), Some(x)) => x.target != s.target || x.slot != s.slot,
        _ => true,
    }
}

/// The lock target's planar direction from the feet (world XZ as x, y), unless standing on it.
fn lock_dir(sense: &Sense) -> Option<Vec2> {
    sense.lock_target.map(|v| Vec2::new(v.x, v.z)).filter(|v| v.length_squared() > 1e-6)
}

/// Locked-on facing (combat.md "Lock-on and camera"): the yaw toward the target, except in a
/// free-run sprint, which faces its run (Elden Ring); it returns when the sprint ends.
fn lock_facing(sense: &Sense, mode: Mode) -> Option<f32> {
    lock_dir(sense).filter(|_| mode != Mode::FreeRun).map(|d| d.x.atan2(d.y))
}

/// Gait from mode and stick length; walking scales with the stick up to the run threshold.
/// With no `desired_facing` the body turns to face the stick, so the gait's forward speed
/// applies whatever the current facing (a reversal keeps its speed instead of asking for
/// the back speed while the body turns round). With one (`lock`: locked on) the body turns
/// to it at the mover's own turn rates and the stick strafes, the side and back speeds
/// measured against that facing.
fn intent(p: &CharacterParams, mode: Mode, stick: Vec2, yaw: f32, lock: Option<f32>) -> LocomotionIntent {
    let yaw = match lock {
        Some(l) => l,
        None if stick.length_squared() > 1e-12 => stick.x.atan2(stick.y),
        None => yaw,
    };
    let len = stick.length();
    let gait = match mode {
        Mode::Sneak => Gait::Sneak,
        Mode::FreeRun => Gait::Sprint,
        Mode::Normal if len >= p.run_threshold => Gait::Run,
        Mode::Normal => Gait::Walk,
    };
    let dir = Vec3::new(stick.x, 0.0, stick.y);
    let input = match gait {
        Gait::Run | Gait::Sprint => dir.normalize_or_zero(),
        Gait::Walk | Gait::Sneak => (dir / p.run_threshold.max(1e-3)).clamp_length_max(1.0),
    };
    let v = p.gaits.get(gait).velocity(input, yaw) * p.profile(mode).speed_scale;
    LocomotionIntent { desired_velocity: v, desired_facing: lock, gait }
}

/// At rest (no stick, or a move that does not steer): zero velocity, and the stance holds:
/// sneak keeps [`Gait::Sneak`] (crouch is held by its button, not by speed), every other
/// mode keeps the idle [`Gait::Walk`].
fn rest(mode: Mode, facing: Option<f32>) -> LocomotionIntent {
    let gait = if mode == Mode::Sneak { Gait::Sneak } else { Gait::Walk };
    LocomotionIntent { desired_facing: facing, gait, ..LocomotionIntent::IDLE }
}
