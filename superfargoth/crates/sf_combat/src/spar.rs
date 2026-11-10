//! A sparring partner's strike schedule (combat.md "C2a"): a scripted command writer that makes
//! a character attack on a fixed, deterministic cadence, so a player can be hit, react and (C2b)
//! guard without an AI. It writes an ordinary [`Command`] (`ActionSystems::Command`), as `sf_ai`
//! will for NPC combat (C6): the partner's swings run through the chooser, the sweeps, the
//! contact order and the rules like anyone's. Content gives the numbers (the game's
//! `sparring.ron`).
//!
//! Industry: a fighting game's training-mode dummy ("record / playback", Street Fighter 6, Tekken
//! 8) and a souls-like's practice target strike on a fixed timer, not an AI, so the defender can
//! learn the timing; ours presses its class every `period` while the target stands in `reach`.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_action::{ActionClass, Command, TICK_HZ, Tick, ViewDir};

/// When and how far a sparring partner strikes (content data; seconds and meters).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SparSchedule {
    /// The class it presses (its sheet's opener of that class starts: `Light` = `light1`).
    pub class: ActionClass,
    /// Seconds from the partner's start to its first strike.
    pub first: f32,
    /// Seconds between strikes.
    pub period: f32,
    /// It strikes only while the target's feet are within this horizontal distance of its own
    /// (m, centre to centre); the sweep still decides whether the blade lands.
    pub reach: f32,
}

impl SparSchedule {
    /// Finite, `first >= 0`, `period` at least one tick, `reach > 0`.
    pub fn valid(&self) -> bool {
        #[allow(clippy::cast_precision_loss)]
        let tick = 1.0 / TICK_HZ as f32;
        self.first.is_finite()
            && self.first >= 0.0
            && self.period.is_finite()
            && self.period >= tick
            && self.reach.is_finite()
            && self.reach > 0.0
    }

    /// Seconds as whole fixed ticks (rounded).
    fn ticks(secs: f32) -> u32 {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let t = (secs.max(0.0) * TICK_HZ as f32).round() as u32;
        t
    }

    /// Is the partner's tick `since` (ticks since it started sparring, its first tick 0) a strike
    /// tick? At `first`, then every `period`.
    pub fn due(&self, since: u32) -> bool {
        let (first, period) = (Self::ticks(self.first), Self::ticks(self.period).max(1));
        since >= first && (since - first).is_multiple_of(period)
    }

    /// The partner's command for `tick`, `since` ticks into its sparring, with `to_target` the
    /// offset from its feet to its target's (world axes, m; `None`: no living target, no input).
    /// It looks at the target (`view`, with `face_view`: an attack squares up to it and its `Turn`
    /// windows aim along it) and presses [`Self::class`] on a due tick while the target is in
    /// [`Self::reach`]. A due tick out of reach, or inside a move that doesn't accept the press
    /// (a reaction), is skipped: the next strike waits a whole period.
    pub fn command(&self, tick: Tick, since: u32, to_target: Option<Vec3>) -> Command {
        let mut cmd = Command::empty(tick);
        let Some(to) = to_target else { return cmd };
        let flat = Vec3::new(to.x, 0.0, to.z);
        if flat.length_squared() > 1e-8 {
            cmd.view = ViewDir {
                yaw: flat.x.atan2(flat.z),
                pitch: 0.0,
            };
            cmd.face_view = true;
        }
        if self.due(since) && flat.length() <= self.reach {
            cmd.buttons.pressed.insert(self.class);
            cmd.buttons.held.insert(self.class);
            cmd.hold_ticks[self.class.index()] = 1;
        }
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPAR: SparSchedule = SparSchedule {
        class: ActionClass::Light,
        first: 0.5,
        period: 2.0,
        reach: 2.0,
    };

    #[test]
    fn strikes_at_first_then_every_period() {
        let due: Vec<u32> = (0..300).filter(|&t| SPAR.due(t)).collect();
        assert_eq!(due, [30, 150, 270]);
        assert!(SPAR.valid());
        assert!(
            !SparSchedule {
                period: 0.0,
                ..SPAR
            }
            .valid()
                && !SparSchedule { reach: 0.0, ..SPAR }.valid()
        );
        assert!(
            !SparSchedule {
                first: f32::NAN,
                ..SPAR
            }
            .valid()
        );
    }

    #[test]
    fn presses_only_on_a_due_tick_in_reach_and_looks_at_the_target() {
        let at = |since, to| SPAR.command(Tick(100 + since), since, to);
        // In reach (1.5 m to the east, +X): pressed on the due tick only, facing it.
        let east = Some(Vec3::new(1.5, 0.3, 0.0));
        let c = at(30, east);
        assert!(
            c.pressed(ActionClass::Light)
                && c.held(ActionClass::Light)
                && c.hold_ticks[ActionClass::Light.index()] == 1
        );
        assert_eq!(c.tick, Tick(130));
        assert!(
            (c.view.yaw - std::f32::consts::FRAC_PI_2).abs() < 1e-6
                && c.face_view
                && c.player.is_none()
        );
        assert!(
            !at(31, east).pressed(ActionClass::Light) && !at(29, east).pressed(ActionClass::Light)
        );
        assert!(
            at(31, east).face_view,
            "it keeps looking at the target between strikes"
        );
        // Out of reach (2.5 m), or no target: no press.
        assert!(!at(30, Some(Vec3::new(0.0, 0.0, 2.5))).pressed(ActionClass::Light));
        assert_eq!(at(30, None), Command::empty(Tick(130)));
        // Exactly at reach strikes (inclusive); the height difference doesn't count.
        assert!(at(150, Some(Vec3::new(0.0, 5.0, -2.0))).pressed(ActionClass::Light));
    }
}
