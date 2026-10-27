//! Lock-on ownership (combat.md "Lock-on and camera", C1c): one tick of a character's locked
//! target (`ActionState::lock`) over the `sf_camera::lockon` math. The shell gathers the
//! candidates (stable ids, aim points, line of sight), calls [`update_lock`] before
//! `sf_action::step`, senses the target for `Sense::lock_target` and mirrors it to the camera.
//!
//! A lock press acquires the best candidate (range and cone around the view yaw, line of sight),
//! else asks the camera to recentre; a press while locked releases; `Next` switches to the side's
//! nearest candidate. The lock breaks past `break_range`, after `los_timeout` without line of
//! sight, when the target is gone (dead or despawned), and on any traversal, which re-offers the
//! same target for `reacquire_window` after touchdown (Elden Ring keeps it through a jump: a
//! jump is not a traversal).

use sf_action::{LockRequest, LockSide, TargetId};
use sf_camera::{BreakReason, Candidate, LockParams, LockTimers, acquire, switch_target};
use sf_coords::WorldPos;

/// What the shell senses for one character's lock this tick.
#[derive(Clone, Copy, Debug)]
pub struct LockInput<'a> {
    /// This tick's command's request.
    pub request: LockRequest,
    /// The character's eye: range, angle and line of sight are measured from it.
    pub origin: WorldPos,
    /// The view yaw the command carries (the acquire cone's axis).
    pub view_yaw: f32,
    /// Every lockable character but this one (dead ones left out).
    pub candidates: &'a [Candidate<TargetId>],
    /// A traversal (mantle, climb, vault) plays.
    pub traversing: bool,
    pub grounded: bool,
}

/// What [`update_lock`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockEvent {
    /// Nothing changed (locked or not).
    Kept,
    Acquired(TargetId),
    /// A press released the lock.
    Released,
    Switched(TargetId),
    /// The target a traversal broke came back after touchdown.
    Reacquired(TargetId),
    Broke(BreakReason),
    /// A lock press found no target: the camera recentres behind the character.
    Recentre,
}

/// The 32-bit key `LockTimers` remembers a broken target by (the id's halves folded).
pub fn lock_key(id: TargetId) -> u32 {
    let [a, b, c, d, e, f, g, h] = id.0.to_le_bytes();
    u32::from_le_bytes([a, b, c, d]) ^ u32::from_le_bytes([e, f, g, h])
}

/// One tick of `lock` (the character's `ActionState::lock`) and its `timers` (module docs). A
/// press during a traversal is ignored: the broken target comes back on its own after touchdown.
pub fn update_lock(lock: &mut Option<TargetId>, timers: &mut LockTimers, s: &LockInput<'_>, p: &LockParams, dt: f32) -> LockEvent {
    if let Some(target) = *lock {
        if s.traversing {
            *lock = None;
            return LockEvent::Broke(timers.break_for_traversal(lock_key(target)));
        }
        match s.request {
            LockRequest::Toggle => {
                *lock = None;
                *timers = LockTimers::default();
                return LockEvent::Released;
            }
            LockRequest::Next(side) => {
                if let Some(next) = switch_target(s.origin, target, s.candidates, side == LockSide::Right, p) {
                    *lock = Some(next);
                    timers.no_los = 0.0;
                    return LockEvent::Switched(next);
                }
            }
            LockRequest::None => {}
        }
        let entry = s.candidates.iter().find(|c| c.id == target);
        return match timers.check(s.origin, entry, p, dt) {
            Ok(()) => LockEvent::Kept,
            Err(reason) => {
                *lock = None;
                LockEvent::Broke(reason)
            }
        };
    }
    if s.traversing {
        return LockEvent::Kept;
    }
    if s.request == LockRequest::Toggle {
        // A press of its own replaces a pending re-offer.
        *timers = LockTimers::default();
        return match acquire(s.origin, s.view_yaw, s.candidates, p) {
            Some(id) => {
                *lock = Some(id);
                LockEvent::Acquired(id)
            }
            None => LockEvent::Recentre,
        };
    }
    if s.grounded {
        timers.touchdown(p);
    }
    match timers.reacquire(s.origin, s.candidates, lock_key, p, dt) {
        Some(id) => {
            *lock = Some(id);
            LockEvent::Reacquired(id)
        }
        None => LockEvent::Kept,
    }
}
