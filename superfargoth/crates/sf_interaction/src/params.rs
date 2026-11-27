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
    /// A door's swing from closed to open (and back), s (range 0.2-3).
    pub door_swing_s: f32,
    /// Delay before a close-behind door closes once its opener has left the doorway, s (range 0-10).
    pub door_close_behind_s: f32,
    /// A put-down spot within this of a resting prop is taken (slot frames and floor points), m (range 0.1-1.5).
    pub place_clearance_m: f32,
    /// Spacing of the floor fallback's golden-angle spiral (point k at this × √k), m (range 0.1-2).
    pub place_spread_m: f32,
    /// Spiral points the floor fallback tries before the anchor itself, count (range 1-64).
    pub place_floor_tries: u8,
    /// Beyond this from a client's camera a break spawns no debris bodies, m (range 5-100).
    pub debris_lod_m: f32,
    /// Breaks applied per tick; the rest wait for the next tick, count (range 1-32).
    pub debris_breaks_per_tick: u8,
    /// Debris pieces alive in a client's active space; past it the oldest start fading, count (range 0-256).
    pub debris_awake_max: u16,
    /// A debris piece starts fading this long after spawning if it has not slept, s (range 1-60).
    pub debris_fade_s: f32,
    /// A fading debris piece scales to zero over this, then despawns, s (range 0.1-3).
    pub debris_shrink_s: f32,
    /// Launch speed of a debris piece away from the hit, m/s (range 0-10).
    pub debris_speed_mps: f32,
    /// Seeded jitter of a piece's launch direction and speed, ratio (range 0-1).
    pub debris_spread: f32,
    /// Rule causes evaluated per tick; the rest wait, oldest first, count (range 8-1024).
    pub rules_evals_per_tick: u16,
    /// Surface wetness from which a target counts as wet (`sf_surface` scale), ratio (range 0-1).
    pub rules_wet_threshold: f32,
    /// How long `Wet` lasts once a target is dry again, s (range 5-900).
    pub rules_wet_ttl_s: f32,
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
            // tunables: const (bound from interact.door.swing_s in sf_interaction_bevy (IN33b2); Guess)
            door_swing_s: 0.8,
            // tunables: const (bound from interact.door.close_behind_s in sf_interaction_bevy (IN33b2); Guess)
            door_close_behind_s: 1.0,
            // tunables: const (bound from interact.place.clearance_m in sf_interaction_bevy (IN46b); Guess, a crate's half width)
            place_clearance_m: 0.35,
            // tunables: const (bound from interact.place.spread_m in sf_interaction_bevy (IN46b); Guess, reach √15 × 0.4 ≈ 1.5 m)
            place_spread_m: 0.4,
            // tunables: const (bound from interact.place.floor_tries in sf_interaction_bevy (IN46b); Guess)
            place_floor_tries: 16,
            // tunables: const (bound from interact.debris.lod_m in sf_interaction_bevy (IN1a); Guess, Chaos debris removal by distance)
            debris_lod_m: 25.0,
            // tunables: const (bound from interact.debris.breaks_per_tick in sf_interaction_bevy (IN1a); Guess)
            debris_breaks_per_tick: 6,
            // tunables: const (bound from interact.debris.awake_max in sf_interaction_bevy (IN1c); Guess, spec 2026-11-11)
            debris_awake_max: 48,
            // tunables: const (bound from interact.debris.fade_s in sf_interaction_bevy (IN1c); Guess, Chaos sleep removal)
            debris_fade_s: 8.0,
            // tunables: const (bound from interact.debris.shrink_s in sf_interaction_bevy (IN1c); Guess, Chaos scale on removal)
            debris_shrink_s: 0.5,
            // tunables: const (bound from interact.debris.speed_mps in sf_interaction_bevy (IN1c); Guess)
            debris_speed_mps: 2.5,
            // tunables: const (bound from interact.debris.spread in sf_interaction_bevy (IN1c); Guess)
            debris_spread: 0.35,
            // tunables: const (bound from interact.rules.evals_per_tick in sf_interaction_bevy (IN17b); Guess)
            rules_evals_per_tick: 64,
            // tunables: const (bound from interact.rules.wet_threshold in sf_interaction_bevy (IN17b); Guess, sf_surface wetness scale)
            rules_wet_threshold: 0.3,
            // tunables: const (bound from interact.rules.wet_ttl_s in sf_interaction_bevy (IN17b); Guess)
            rules_wet_ttl_s: 120.0,
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

    #[must_use]
    pub fn door_close_behind_ticks(&self) -> u64 {
        self.ticks(self.door_close_behind_s)
    }
}
