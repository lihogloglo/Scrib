//! The core's tunables (interaction.md "Tunables"): plain fields here, bound from the `interact.*`
//! registry entries by `sf_interaction_bevy` (IN31).

use serde::{Deserialize, Serialize};

/// Claim, queue and focus tunables, plus the fixed tick rate leases and dwells are counted in.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractParams {
    /// Ambient use priority (range 0-255).
    pub priority_ambient: u8,
    /// Schedule use priority (range 0-255).
    pub priority_schedule: u8,
    /// Quest use priority (range 0-255).
    pub priority_quest: u8,
    /// Player use priority (range 0-255).
    pub priority_player: u8,
    /// A Reserved claim's lease without progress, s (range 1-60).
    pub lease_s: f32,
    /// Owner move beyond which claims are invalidated, m (range 0.05-2).
    pub move_tolerance_m: f32,
    /// Owner turn beyond which claims are invalidated, degrees (range 1-90).
    pub move_tolerance_deg: f32,
    /// Queue capacity of kinds that queue by default (door, workstation), count (range 0-16).
    pub queue_capacity: u8,
    /// A queue ticket's lease without renewal, s (range 5-300).
    pub ticket_lease_s: f32,
    /// Hand claims a user may hold while Occupied, count (range 0-4).
    pub hand_max: u8,
    /// Hold of interact after which the focus lists every verb, s (range 0.1-1.5; a menu row).
    pub focus_hold_s: f32,
    /// Score lead, as a fraction of the focused target's score, a challenger needs to take focus,
    /// ratio (range 0-1).
    pub focus_switch_margin: f32,
    /// How long a challenger must keep that lead before focus switches, s (range 0-1).
    pub focus_switch_s: f32,
    /// Fixed simulation rate leases count in, ticks per second.
    pub tick_hz: f32,
}

impl Default for InteractParams {
    fn default() -> Self {
        Self {
            // tunables: const (bound from interact.claim.priority_ambient in sf_interaction_bevy; Guess, order from UE Smart Objects)
            priority_ambient: 10,
            // tunables: const (bound from interact.claim.priority_schedule in sf_interaction_bevy; Guess)
            priority_schedule: 20,
            // tunables: const (bound from interact.claim.priority_quest in sf_interaction_bevy; Guess, = player, roadmap decision 1)
            priority_quest: 30,
            // tunables: const (bound from interact.claim.priority_player in sf_interaction_bevy; Guess)
            priority_player: 30,
            // tunables: const (bound from interact.claim.lease_s in sf_interaction_bevy; Guess, spec 2026-11-11)
            lease_s: 10.0,
            // tunables: const (bound from interact.claim.move_tolerance_m in sf_interaction_bevy; Guess)
            move_tolerance_m: 0.25,
            // tunables: const (bound from interact.claim.move_tolerance_deg in sf_interaction_bevy; Guess)
            move_tolerance_deg: 20.0,
            // tunables: const (bound from interact.queue.capacity in sf_interaction_bevy; Guess)
            queue_capacity: 3,
            // tunables: const (bound from interact.queue.ticket_lease_s in sf_interaction_bevy; Guess)
            ticket_lease_s: 30.0,
            // tunables: const (bound from interact.claim.hand_max in sf_interaction_bevy; Guess, two hands)
            hand_max: 2,
            // tunables: const (bound from interact.focus.hold_s in sf_interaction_bevy; Guess, RDR2-style hold prompt)
            focus_hold_s: 0.35,
            // tunables: const (bound from interact.focus.switch_margin in sf_interaction_bevy; Guess, hysteresis against prompt flicker, gta6.md 2)
            focus_switch_margin: 0.2,
            // tunables: const (bound from interact.focus.switch_s in sf_interaction_bevy; Guess)
            focus_switch_s: 0.15,
            // tunables: const (bound from the app's fixed timestep in sf_interaction_bevy; net.md 60 Hz tick)
            tick_hz: 60.0,
        }
    }
}

impl InteractParams {
    /// Seconds to whole ticks at `tick_hz`, rounded, at least one.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn ticks(&self, secs: f32) -> u64 {
        ((f64::from(secs.max(0.0)) * f64::from(self.tick_hz)).round() as u64).max(1)
    }

    #[must_use]
    pub fn lease_ticks(&self) -> u64 {
        self.ticks(self.lease_s)
    }

    #[must_use]
    pub fn ticket_lease_ticks(&self) -> u64 {
        self.ticks(self.ticket_lease_s)
    }

    #[must_use]
    pub fn focus_hold_ticks(&self) -> u64 {
        self.ticks(self.focus_hold_s)
    }

    #[must_use]
    pub fn focus_switch_ticks(&self) -> u64 {
        self.ticks(self.focus_switch_s)
    }
}
