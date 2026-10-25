//! The chooser: one pure step per character per fixed tick (action.md "The chooser").
//!
//! `step` decides; it doesn't move the body. The Sim owner's [`LocomotionIntent`] and launch
//! go out in [`ActionOutput`], and the shell runs one batched `KinematicMover::step_batch` for
//! every character afterwards (the mover batches its probes across characters), whose result
//! comes back as next tick's [`Sense`].

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_controller::{Gait, LocomotionIntent};

use crate::command::{ActionClass, ClassSet, Command};
use crate::ids::{CharacterId, RuleKey, SlotKey, TagId, TargetId, Tick};
use crate::rule::{CompiledRule, ExitOn, Launch, Mode, Phase, RuleSet};
use crate::state::{ActionClock, ActionState, CharacterParams};
use crate::windows::{Gates, SlotSource, gates, locate};

/// Extra choose rounds when a move ends or exits on its first tick (bounded settle).
pub const MAX_SETTLE: usize = 4;

/// Cosine of the `need_forward` cone (60°).
const FORWARD_COS: f32 = 0.5;

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
    pub events: Vec<ActionEvent>,
    pub refusals: Vec<Refusal>,
}

impl ActionOutput {
    fn reset(&mut self, tick: Tick) {
        self.tick = tick;
        self.started = false;
        self.launch = None;
        self.prompt = None;
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
    mode: Mode,
    focus: Option<Focus>,
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

    fn entry(&self, r: &CompiledRule) -> Result<(), Refused> {
        let e = &r.rule.entry;
        let within = |band: Option<(f32, f32)>, v: f32| band.is_none_or(|(lo, hi)| (lo..=hi).contains(&v));
        if !e.modes.is_empty() && !e.modes.contains(&self.mode) {
            return Err(Refused::Mode);
        }
        if !e.states.is_empty() && !e.states.iter().any(|&s| self.has(s)) {
            return Err(Refused::State);
        }
        if e.need_forward && !self.forward {
            return Err(Refused::Forward);
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
        Ok(())
    }
}

fn forced(r: &CompiledRule) -> bool {
    r.rule.category.framework_only() && r.classes.is_empty()
}

fn automatic(r: &CompiledRule) -> bool {
    r.classes.is_empty() && !forced(r)
}

/// Advances one character by one fixed tick: command → clock → buffer → candidates → pick →
/// start (+ bounded settle) → Sim intent → output.
pub fn step(ctx: &StepCtx<'_>, state: &mut ActionState, cmd: &Command, sense: &Sense, out: &mut ActionOutput) {
    let (rules, p, tick) = (ctx.rules, ctx.params, cmd.tick);
    out.reset(tick);
    let event = |target, kind| ActionEvent { tick, source: ctx.source, target, kind };

    let (landed, impact, coyote) = body_edges(state, sense, p, tick);
    out.events.extend(landed.then(|| event(None, EventKind::Land)));

    // 1. Command: modes.
    let stick_active = cmd.stick.length() > p.deadzone;
    update_mode(&mut state.mode.mode, cmd, stick_active);

    // 2. Clock.
    state.clock.advance();
    let mut cur = rules.index_of(state.mov);
    let gates_of = |cur: Option<usize>, state: &ActionState| {
        cur.map_or(Gates { ended: true, ..Gates::OPEN }, |i| gates(rules.at(i), ctx.slots, state.clock.elapsed))
    };

    // 3. Buffer: latch inside accept; an airborne Jump goes to the jump buffer instead.
    let g = gates_of(cur, state);
    for c in cmd.buttons.pressed.intersection(ClassSet::LATCHABLE).iter() {
        if c == ActionClass::Jump && !sense.grounded && !coyote {
            state.timers.jump_pressed = Some(tick);
        } else if g.accept.contains(c) {
            state.buffer.latched.insert(c);
        }
    }
    if !cmd.held(ActionClass::Guard) {
        state.buffer.latched.remove(ActionClass::Guard);
    }
    let jump_buffered = sense.grounded && state.timers.jump_pressed.is_some_and(|t| tick.since(t) <= u32::from(p.jump_buffer_ticks));

    let (stick, facing) = (cmd.stick, Vec2::new(sense.yaw.sin(), sense.yaw.cos()));
    let facts = Facts {
        grounded: sense.grounded,
        coyote,
        landed,
        vspeed: impact,
        speed: sense.planar_speed,
        forward: stick_active && stick.normalize_or_zero().dot(facing) >= FORWARD_COS,
        mode: state.mode.mode,
        focus: sense.focus,
    };

    // 4-8. Candidates, pick, start; settle while the new move ends or exits at once.
    let mut started = [usize::MAX; MAX_SETTLE + 1];
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
                });
                // An automatic move lasts while its entry holds.
                let lapsed = automatic(r) && !fresh && facts.entry(r).is_err();
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
        let exit_target = exit.and_then(|(_, to)| to);

        let mut winner = None;
        for &i in rules.priority() {
            let r = rules.at(i);
            // The current move restarts only once it is over (ended, exited or lapsed).
            if (Some(i) == cur && !open) || started.contains(&i) || state.timers.redo.iter().any(|&(k, _)| k == r.key) {
                continue;
            }
            let requested = !r.classes.intersection(firing).is_empty();
            let candidate = forced(r)
                || requested
                || exit_target == Some(i)
                || (auto_ok && (automatic(r) || r.rule.entry.auto_in.contains(&facts.mode)));
            if !candidate {
                continue;
            }
            match facts.entry(r) {
                Ok(()) => {
                    winner = Some(i);
                    break;
                }
                Err(reason) if requested => out.refusals.push(Refusal { rule: r.key, reason }),
                Err(_) => {}
            }
        }
        let Some(i) = winner else {
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
        start(ctx, rules.at(i), state, sense, facts.mode, out);
        started[round] = i;
        cur = Some(i);
    }

    // 9. Move (Sim owner): the mover's intent.
    let steer = cur.is_none_or(|i| rules.at(i).rule.steer);
    out.intent = if steer && stick_active { intent(p, facts.mode, stick, sense.yaw) } else { LocomotionIntent::IDLE };

    write_output(ctx, state, cur, &facts, out);
}

/// 10. Output: move, slot chain, playhead, and the "press to use" prompt.
fn write_output(ctx: &StepCtx<'_>, state: &ActionState, cur: Option<usize>, facts: &Facts, out: &mut ActionOutput) {
    let rules = ctx.rules;
    out.rule = state.mov;
    let Some(r) = cur.map(|i| rules.at(i)) else {
        (out.slot, out.chain, out.playhead) = (None, 0, 0.0);
        return;
    };
    let at = locate(r, ctx.slots, state.clock.elapsed);
    out.slot = at.slot;
    out.chain = at.chain;
    out.playhead = at.local.secs();
    if let Some(f) = facts.focus {
        let ready = gates(r, ctx.slots, state.clock.elapsed).accept.contains(ActionClass::Interact);
        out.prompt = rules
            .iter()
            .filter(|o| o.focus.is_some() && o.key != r.key)
            .find(|o| facts.entry(o).is_ok())
            .map(|o| Prompt { target: f.target, tag: f.tag, rule: o.key, ready });
    }
}

/// Sprint held with the stick is free-run and overrides sneak; Sneak held is sneak (a toggle
/// preference is the input layer's: [`ActionClass::Sneak`]).
fn update_mode(mode: &mut Mode, cmd: &Command, stick_active: bool) {
    *mode = if cmd.held(ActionClass::Sprint) && stick_active {
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
fn start(ctx: &StepCtx<'_>, r: &CompiledRule, state: &mut ActionState, sense: &Sense, mode: Mode, out: &mut ActionOutput) {
    let (p, tick) = (ctx.params, out.tick);
    let event = |target, kind| ActionEvent { tick, source: ctx.source, target, kind };
    state.mov = r.key;
    state.started = tick;
    state.clock = ActionClock::default();
    state.buffer.latched = ClassSet::EMPTY;
    if r.classes.contains(ActionClass::Jump) {
        state.timers.jump_pressed = None;
    }
    if r.rule.redo_ticks > 0 {
        state.timers.redo.push((r.key, Tick(tick.0.wrapping_add(u32::from(r.rule.redo_ticks)))));
    }
    let focus = r.focus.as_ref().and(sense.focus);
    out.events.push(event(focus.map(|f| f.target), EventKind::MoveStart(r.key)));
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

/// Gait from mode and stick length; walking scales with the stick up to the run threshold.
fn intent(p: &CharacterParams, mode: Mode, stick: Vec2, yaw: f32) -> LocomotionIntent {
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
    LocomotionIntent { desired_velocity: v, desired_facing: None, gait }
}
