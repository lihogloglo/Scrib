//! Soft aim (camera.md "First person", combat.md "Lock-on and camera" C4): with no lock-on, an
//! attack picks its own target early in the swing and its `Turn` windows steer toward it
//! (`Sense::soft_target`). Sekiro's auto-aim (R6 §5): the nearest enemy within ~3 m and +-30 deg
//! of the aim axis, acquired only in the first ~1/6 s of the attack; God of War auto-orients
//! attacks the same way (R8 §1.3). The camera never turns: in first person the axis is the view,
//! so the body swings where the player looks and the soft aim only corrects the last few degrees.
//!
//! The target belongs to the swing ([`SwingState::aim`]): a new move (a chained light) starts
//! with none and acquires again. The shell gathers the candidates (stable ids, aim points, line
//! of sight) and calls [`update_soft_aim`] before `sf_action::step`, as it does the lock.

use sf_action::{ActionState, Command, TargetId};
use sf_camera::{Candidate, SoftAimParams, soft_aim};
use sf_coords::WorldPos;

use crate::fighter::{FightState, SwingState};

/// The soft-aim cone's axis for this tick's command: the view yaw when the body faces the view
/// (first person, [`Command::face_view`]), else the active stick (past `deadzone`), else the
/// body's `yaw` (Sekiro: the stick, or straight ahead).
pub fn soft_aim_axis(cmd: &Command, yaw: f32, deadzone: f32) -> f32 {
    if cmd.face_view {
        cmd.view.yaw
    } else if cmd.stick.length() > deadzone {
        cmd.stick.x.atan2(cmd.stick.y)
    } else {
        yaw
    }
}

/// What the shell senses for one attacker's soft aim this tick.
#[derive(Clone, Copy, Debug)]
pub struct SoftAimInput<'a> {
    /// The attacker's eye: reach (horizontal), angle and line of sight are measured from it.
    pub origin: WorldPos,
    /// [`soft_aim_axis`].
    pub axis_yaw: f32,
    /// Every target candidate but the attacker (dead ones left out).
    pub candidates: &'a [Candidate<TargetId>],
    /// A lock-on target exists: it wins, the soft aim stays off.
    pub locked: bool,
}

/// The current move's swing, if `fight` holds the swing of `state`'s move.
fn current<'a>(fight: &'a mut FightState, state: &ActionState) -> Option<&'a mut SwingState> {
    fight.swing.as_mut().filter(|sw| sw.mov == state.mov)
}

/// Whether [`update_soft_aim`] would look for a target this tick (the shell gathers candidates
/// and casts line of sight only then): an attack's swing without a target, inside the window.
pub fn soft_aim_wanted(fight: &FightState, state: &ActionState, p: &SoftAimParams) -> bool {
    fight.swing.as_ref().is_some_and(|sw| {
        sw.mov == state.mov && sw.aim.is_none() && p.open(state.clock.elapsed.secs())
    })
}

/// One tick of the current swing's soft-aim target (module docs); returns it. With a lock the
/// target is dropped; without a swing (not attacking) there is none. Acquisition runs while the
/// move time is inside `p.window`; once picked, the target stays for the swing, wherever it goes
/// (the `Turn` windows track it, `NoTurn` holds the swing on its line).
pub fn update_soft_aim(
    fight: &mut FightState,
    state: &ActionState,
    s: &SoftAimInput<'_>,
    p: &SoftAimParams,
) -> Option<TargetId> {
    let elapsed = state.clock.elapsed.secs();
    let sw = current(fight, state)?;
    if s.locked {
        sw.aim = None;
        return None;
    }
    if sw.aim.is_none() && p.open(elapsed) {
        sw.aim = soft_aim(s.origin, s.axis_yaw, s.candidates, p);
    }
    sw.aim
}
