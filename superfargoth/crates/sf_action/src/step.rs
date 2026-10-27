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
use crate::grip::LedgeGrip;
use sf_anim::{RootChain, RootPath, RootPose, WarpPlan};

use crate::ids::{CharacterId, ClockTime, MovesetKey, RuleKey, SlotKey, TagId, TargetId, Tick, TICK_HZ};
use crate::rule::{Category, CompiledRule, ExitOn, Launch, StickSide, Mode, ObstacleClass, Owner, Phase, PrevKey, ProbeNeed, RuleSet, SquareUp, WarpTarget};
use crate::state::{ActionClock, ActionState, CharacterParams, RATE_ONE, WarpState};
use crate::windows::{Gates, SlotSource, WindowKind, gates, locate, open_windows};

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
    /// No obstacle of the rule's classes and height band.
    Probe,
    /// A Warp rule's slots have no root path.
    NoRootPath,
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
    r.rule.category.framework_only() && r.classes.is_empty() && !r.force_only()
}

fn automatic(r: &CompiledRule) -> bool {
    r.classes.is_empty() && !forced(r) && !r.force_only()
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

    // 1. Command: modes.
    let stick_active = cmd.stick.length() > p.deadzone;
    update_mode(&mut state.mode.mode, cmd, stick_active, sense.refused);

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
            let by = r.classes.intersection(firing);
            let requested = !by.is_empty();
            let other = forced(r) || exit_target == Some(i) || (auto_ok && (automatic(r) || r.rule.entry.auto_in.contains(&facts.mode)));
            if !(requested || other) {
                continue;
            }
            let cur_rule = cur.map(|c| rules.at(c));
            let gated = requested && !other && by.intersection(sense.refused) == by;
            let checked = facts
                .entry(r)
                .and(if prev_ok(r, cur_rule) { Ok(()) } else { Err(Refused::Prev) })
                .and(if gated { Err(Refused::Gate) } else { Ok(()) })
                .and_then(|()| plan_warp(ctx, r, sense));
            match checked {
                Ok(plan) => {
                    winner = Some((i, plan));
                    break;
                }
                Err(reason) if requested => out.refusals.push(Refusal { rule: r.key, reason }),
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

    // 9. Move (Sim owner): the mover's intent, slowed with the clock under hit-stop. Locked on,
    // the body faces the target, standing still too.
    let lock = lock_facing(sense, facts.mode);
    out.intent = match (stick_active, cur.is_none_or(|i| steers(rules.at(i)))) {
        (true, true) => intent(p, facts.mode, stick, sense.yaw, lock),
        (false, true) => LocomotionIntent { desired_facing: lock, ..LocomotionIntent::IDLE },
        (_, false) => LocomotionIntent::IDLE,
    };
    if state.clock.rate < RATE_ONE {
        out.intent.desired_velocity *= f32::from(state.clock.rate) / f32::from(RATE_ONE);
    }
    let t0 = state.clock.elapsed;
    out.clock = (t0, ClockTime(t0.0.saturating_add(u32::from(state.clock.rate))));
    let fresh = out.started;
    write_clip(ctx, cur, cmd, sense, fresh, out);

    write_output(ctx, state, cur, &facts, out);
}

/// The stick steers the Sim owner unless the rule says no; never a Warp or Clip owner.
fn steers(r: &CompiledRule) -> bool {
    r.rule.steer && matches!(r.rule.owner, Owner::Sim)
}

/// Clip owner: yaw (square-up on the first tick, then `Turn` toward the lock target or the stick
/// unless `NoTurn` is open, plus the clip's own root yaw) and the planar root delta over the
/// tick's clock interval x `root_scale`.
fn write_clip(ctx: &StepCtx<'_>, cur: Option<usize>, cmd: &Command, sense: &Sense, fresh: bool, out: &mut ActionOutput) {
    let Some(r) = cur.map(|i| ctx.rules.at(i)) else { return };
    let Owner::Clip { root_scale, square_up, rate } = r.rule.owner else { return };
    let (t0, t1) = out.clock;
    let stick = (cmd.stick.length() > ctx.params.deadzone).then_some(cmd.stick);
    let mut yaw = sense.yaw;
    if fresh {
        let lock = (square_up == SquareUp::Lock).then(|| lock_dir(sense)).flatten();
        let to = match square_up {
            SquareUp::Keep => None,
            SquareUp::Stick | SquareUp::Lock => lock.or(stick),
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
    let aim = lock_dir(sense).or(stick);
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
    out.clip = Some(ClipOutput { delta, yaw, progress: c1.min(duration) });
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
    out.warp = with_root_path(r, ctx.slots, |path| {
        let t = t.min(path.duration());
        WarpOutput { root: w.plan.root_at(path, t), progress: t }
    });
}

/// Runs `f` on the rule's slot chain root path (one slot: its path; several: a [`RootChain`]).
/// `None` if a slot has no root path.
fn with_root_path<R>(r: &CompiledRule, slots: &dyn SlotSource, f: impl FnOnce(&dyn RootPath) -> R) -> Option<R> {
    if let [one] = r.slots.as_slice() {
        return slots.root(*one).map(f);
    }
    let parts = r.slots.iter().map(|k| slots.root(*k)).collect::<Option<Vec<_>>>()?;
    Some(f(&RootChain::new(parts)))
}

/// A Warp rule's plan (validators `EndPoseFree` via the probe's room check, `MaxCorrection`):
/// start at the feet facing into the face, target on top `inset` past the lip (kept a radius
/// inside a shallower top). Other owners plan nothing.
fn plan_warp(ctx: &StepCtx<'_>, r: &CompiledRule, sense: &Sense) -> Result<Option<WarpState>, Refused> {
    let Owner::Warp { target: WarpTarget::LedgeTop { inset }, profile, refuse_ratio, .. } = r.rule.owner else {
        return Ok(None);
    };
    let o = sense.obstacle.ok_or(Refused::Probe)?;
    let into = Vec3::new(-o.normal.x, 0.0, -o.normal.z).normalize_or_zero();
    if into == Vec3::ZERO {
        return Err(Refused::Probe);
    }
    let yaw = into.x.atan2(into.z);
    let room = if o.depth.is_finite() { (o.depth - ctx.params.radius).max(0.0) } else { inset };
    let lip = Vec3::new(o.lip.x, o.height, o.lip.z);
    let top = lip + into * inset.min(room);
    let plan = with_root_path(r, ctx.slots, |path| {
        WarpPlan::try_new(path, RootPose::new(Vec3::ZERO, yaw), RootPose::new(top, yaw), profile, refuse_ratio)
    })
    .ok_or(Refused::NoRootPath)?
    .map_err(|_| Refused::MaxCorrection)?;
    // Hands on the lip (B2b), from the slot clip's baked hand contacts.
    let grip = with_root_path(r, ctx.slots, |path| LedgeGrip::plan(path, &plan, lip, into)).flatten();
    Ok(Some(WarpState { plan, grip }))
}

/// 10. Output: move, slot chain, playhead, and the "press to use" prompt.
fn write_output(ctx: &StepCtx<'_>, state: &ActionState, cur: Option<usize>, facts: &Facts, out: &mut ActionOutput) {
    write_warp(ctx, state, cur, out);
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
