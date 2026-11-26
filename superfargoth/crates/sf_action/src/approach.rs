//! The approach before an enter (`IN2c4`, interaction.md "The approach before the enter"): an
//! [`Owner::Approach`] move walks the body onto the pose its [`ExitOn::Arrived`] exit's Warp rule
//! starts from with zero correction, turns it in place to that pose's yaw, then exits into the
//! rule (UE Smart Objects' move-to-entrance, Bethesda's furniture entry point). Pure functions;
//! [`crate::step`] calls them.

use glam::{Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::RootPose;
use sf_controller::{Gait, LocomotionIntent};

use crate::command::{ActionClass, Command};
use crate::ids::{RuleKey, TICK_HZ, TargetId};
use crate::rule::{CompiledRule, ExitOn, ExitTo, Mode, MoveRule, Owner};
use crate::state::{ActionState, CharacterParams};
use crate::step::{Refusal, Refused, SeatSense, Sense, StepCtx, with_root_path};

/// The slot an approach walks to ([`ActionState::approach`]): kept while it runs, so the ticks
/// after the offer need no new one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApproachState {
    pub target: TargetId,
    /// The slot index on the target (`sf_interaction::SlotRef::slot`).
    pub slot: u8,
}

/// Why a running approach ends this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cause {
    /// Within `arrive_m` and `arrive_rad`: the `Arrived` exit fires.
    Arrived,
    /// The stick past the deadzone or a Jump press.
    Cancelled,
    /// `timeout_s` of move time elapsed.
    Timeout,
    /// The slot sense is gone or another slot (claim lost): no event, the shell knows.
    Lost,
}

/// The approach running at the start of a tick ([`running`]), read again after the chooser.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Running {
    /// The approach rule's index.
    rule: usize,
    key: RuleKey,
    held: Option<ApproachState>,
    /// The `Arrived` exit's rule index.
    exit: Option<usize>,
    cause: Option<Cause>,
}

impl Running {
    /// Rule `i` is this approach and it arrived: its `Arrived` exit fires.
    pub(crate) fn arrived(&self, i: usize) -> bool {
        i == self.rule && self.cause == Some(Cause::Arrived)
    }

    /// Rule `i` is this approach and it ends this tick other than by arriving: it lapses.
    pub(crate) fn quits(&self, i: usize) -> bool {
        i == self.rule && matches!(self.cause, Some(Cause::Cancelled | Cause::Timeout | Cause::Lost))
    }
}

/// Load-time checks of a rule's approach parts; the message, if any.
pub(crate) fn validate(r: &MoveRule) -> Option<&'static str> {
    let arrived = r.exits.iter().any(|e| e.on == ExitOn::Arrived);
    match r.owner {
        Owner::Approach { arrive_m, arrive_rad, timeout_s, max_m } => {
            if ![arrive_m, arrive_rad, timeout_s, max_m].iter().all(|v| v.is_finite() && *v > 0.0) {
                Some("approach arrive_m, arrive_rad, timeout_s and max_m must be finite and > 0")
            } else if r.slots.is_empty() || r.launch.is_some() || !r.exits.iter().any(|e| e.on == ExitOn::Arrived && matches!(e.to, ExitTo::Rule(_))) {
                Some("an Approach owner needs a slot (the locomotion loop it shows), an exit on Arrived to a rule and no launch")
            } else {
                None
            }
        }
        _ if arrived => Some("exit on Arrived needs an Approach owner"),
        _ => None,
    }
}

/// The `Arrived` exit's rule index.
fn exit_of(r: &CompiledRule) -> Option<usize> {
    r.exits.iter().find(|(on, _)| *on == ExitOn::Arrived).and_then(|&(_, to)| to)
}

/// The pose the `Arrived` exit's rule starts from with zero warp correction, relative to the feet
/// this tick (world axes, world yaw): the slot frame `E` composed with the inverse of that rule's
/// whole root travel `B` (its Warp root path, `stand`-cut, at its end), so `S ⊕ B = E`:
/// `S.yaw = E.yaw - B.yaw`, `S.pos = E.pos - rot(S.yaw) B.pos`. `None`: no exit rule or no root path.
pub(crate) fn target(ctx: &StepCtx<'_>, r: &CompiledRule, seat: &SeatSense) -> Option<RootPose> {
    let travel = with_root_path(ctx.rules.at(exit_of(r)?), ctx.slots, |p| p.root_at(p.duration().max(0.0)))?;
    let yaw = seat.enter.yaw - travel.yaw;
    Some(RootPose::new(seat.enter.position - Quat::from_rotation_y(yaw) * travel.position, yaw))
}

fn planar(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.z)
}

/// Angle difference wrapped to [-pi, pi].
fn yaw_gap(a: f32, b: f32) -> f32 {
    (a - b + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

/// The start gate of an approach rule (other rules pass): a sensed slot ([`Refused::Slot`]), a
/// root path for its exit rule ([`Refused::NoRootPath`]), the target within `max_m` planar
/// ([`Refused::ApproachTooFar`]).
pub(crate) fn check(ctx: &StepCtx<'_>, r: &CompiledRule, sense: &Sense) -> Result<(), Refused> {
    let Owner::Approach { max_m, .. } = r.rule.owner else { return Ok(()) };
    let seat = sense.seat.ok_or(Refused::Slot)?;
    let t = target(ctx, r, &seat).ok_or(Refused::NoRootPath)?;
    if planar(t.position).length() > max_m { Err(Refused::ApproachTooFar) } else { Ok(()) }
}

/// The slot a move starting now walks to: an approach records the sensed one, any other move none.
pub(crate) fn on_start(r: &CompiledRule, sense: &Sense) -> Option<ApproachState> {
    if !matches!(r.rule.owner, Owner::Approach { .. }) {
        return None;
    }
    sense.seat.map(|s| ApproachState { target: s.target, slot: s.slot })
}

/// The sensed slot, if it is the one held.
fn held_seat(held: Option<ApproachState>, sense: &Sense) -> Option<SeatSense> {
    let a = held?;
    sense.seat.filter(|s| s.target == a.target && s.slot == a.slot)
}

/// The approach running at the start of this tick (`cur`), with why it ends now, if it does.
/// Order: the claim lost, then the stick or a Jump, then arrival, then the timeout.
pub(crate) fn running(ctx: &StepCtx<'_>, state: &ActionState, cur: Option<usize>, cmd: &Command, sense: &Sense, stick_active: bool) -> Option<Running> {
    let i = cur?;
    let r = ctx.rules.at(i);
    let Owner::Approach { arrive_m, arrive_rad, timeout_s, .. } = r.rule.owner else { return None };
    let cause = match held_seat(state.approach, sense) {
        None => Some(Cause::Lost),
        Some(_) if stick_active || cmd.buttons.pressed.contains(ActionClass::Jump) => Some(Cause::Cancelled),
        Some(seat) if target(ctx, r, &seat).is_some_and(|t| planar(t.position).length() <= arrive_m && yaw_gap(sense.yaw, t.yaw).abs() <= arrive_rad) => {
            Some(Cause::Arrived)
        }
        Some(_) if state.clock.elapsed.secs() >= timeout_s => Some(Cause::Timeout),
        Some(_) => None,
    };
    Some(Running { rule: i, key: r.key, held: state.approach, exit: exit_of(r), cause })
}

/// The walk intent while rule `cur` is a running approach: a walk-gait velocity straight at the
/// target, facing the way it walks, capped so one tick never overshoots (speed <= distance / tick);
/// within `arrive_m`, zero velocity and the target's yaw (the mover turns in place). Sneaking, the
/// sneak gait. `None` when `cur` is not an approach or its slot is not sensed (it ends this tick).
pub(crate) fn intent(ctx: &StepCtx<'_>, state: &ActionState, cur: Option<usize>, sense: &Sense, mode: Mode) -> Option<LocomotionIntent> {
    let r = ctx.rules.at(cur?);
    let Owner::Approach { arrive_m, .. } = r.rule.owner else { return None };
    let seat = held_seat(state.approach, sense)?;
    let gait = if mode == Mode::Sneak { Gait::Sneak } else { Gait::Walk };
    let still = LocomotionIntent { gait, ..LocomotionIntent::IDLE };
    let Some(t) = target(ctx, r, &seat) else { return Some(still) };
    Some(walk(ctx.params, mode, gait, t, arrive_m))
}

/// [`intent`] towards `t` (relative to the feet).
fn walk(p: &CharacterParams, mode: Mode, gait: Gait, t: RootPose, arrive_m: f32) -> LocomotionIntent {
    let d = planar(t.position);
    let dist = d.length();
    if dist <= arrive_m {
        return LocomotionIntent { desired_facing: Some(t.yaw), gait, ..LocomotionIntent::IDLE };
    }
    #[allow(clippy::cast_precision_loss)]
    let per_tick = TICK_HZ as f32;
    let speed = (p.gaits.get(gait).forward * p.profile(mode).speed_scale).min(dist * per_tick);
    let dir = d / dist;
    LocomotionIntent { desired_velocity: Vec3::new(dir.x, 0.0, dir.y) * speed, desired_facing: Some(dir.x.atan2(dir.y)), gait }
}

/// After the chooser: an approach that was running and is over answers its offer once, unless it
/// handed over to its exit rule or lost the claim: the slot and the refusal to report (the caller
/// sends [`crate::EventKind::SeatRefused`]). `exit_refused`: why the exit rule refused on arrival.
pub(crate) fn answer(was: Option<Running>, started: &[usize], exit_refused: Option<Refused>) -> Option<(ApproachState, Refusal)> {
    let w = was?;
    let over = started.iter().any(|&s| s != usize::MAX);
    if !over || w.cause == Some(Cause::Lost) || w.exit.is_some_and(|j| started.contains(&j)) {
        return None;
    }
    let reason = match w.cause {
        Some(Cause::Timeout) => Refused::ApproachTimeout,
        Some(Cause::Arrived) => exit_refused.unwrap_or(Refused::ApproachCancelled),
        // The stick, a Jump, or a forced move took over.
        _ => Refused::ApproachCancelled,
    };
    Some((w.held?, Refusal { rule: w.key, reason }))
}
