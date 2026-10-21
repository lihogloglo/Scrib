//! First-person look limits (camera.md "First person"): per-move yaw and pitch limits around the
//! move's entry yaw, eased in over the rule's `ease` time and released eased, and `ignore_look`
//! windows. The eye itself (the baked virtual camera track) arrives with slice D2.

use serde::{Deserialize, Serialize};

use crate::{Look, smoothstep, wrap_angle};

/// A move rule's `look` limits, radians (`MoveRule.look` is authored in degrees: use
/// [`Self::from_degrees`]).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LookLimits {
    /// Half range of yaw around the entry yaw.
    pub yaw: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    /// Seconds to ease the limits in (and out on release).
    pub ease: f32,
}

impl LookLimits {
    pub fn from_degrees(yaw: f32, pitch: (f32, f32), ease: f32) -> Self {
        Self { yaw: yaw.to_radians(), pitch_min: pitch.0.to_radians(), pitch_max: pitch.1.to_radians(), ease }
    }

    fn clamp(&self, look: Look, entry_yaw: f32) -> Look {
        let rel = wrap_angle(look.yaw - entry_yaw).clamp(-self.yaw, self.yaw);
        Look { yaw: wrap_angle(entry_yaw + rel), pitch: look.pitch.clamp(self.pitch_min, self.pitch_max) }
    }
}

/// First-person look state (presentation; the shell keeps one per local player).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FirstPersonLook {
    limits: Option<LookLimits>,
    /// The last limits, kept while they ease out.
    fading: Option<LookLimits>,
    entry_yaw: f32,
    /// 0 free .. 1 fully limited, ramped linearly over `ease`, applied through a smoothstep.
    weight: f32,
}

impl FirstPersonLook {
    pub fn new() -> Self {
        Self::default()
    }

    /// Current limit weight after smoothing, 0..1.
    pub fn weight(&self) -> f32 {
        smoothstep(self.weight)
    }

    /// Set the active move's limits (`None`: free look). Call when the move changes; `entry_yaw`
    /// is the character's yaw at move start. Same limits again are a no-op.
    pub fn set_limits(&mut self, limits: Option<LookLimits>, entry_yaw: f32) {
        if limits == self.limits && (limits.is_none() || (self.entry_yaw - entry_yaw).abs() < 1e-6) {
            return;
        }
        if let Some(l) = limits {
            if self.limits.is_none() && self.fading.is_none() {
                self.weight = 0.0;
            }
            self.entry_yaw = entry_yaw;
            self.fading = None;
            self.limits = Some(l);
        } else {
            self.fading = self.limits.take();
        }
    }

    /// Apply this frame's look input `delta` to `look` (the shell's accumulated look, `pitch`
    /// clamped to `free_pitch` outside limits) and return the new look, which the shell stores
    /// back so the player never has to unwind input pushed past a limit. `ignore_look` drops the
    /// input (rule window).
    pub fn update(&mut self, look: Look, delta: Look, ignore_look: bool, free_pitch: (f32, f32), dt: f32) -> Look {
        let delta = if ignore_look { Look::default() } else { delta };
        let free = look.turned(delta, free_pitch.0, free_pitch.1);
        let (active, rising) = match (self.limits, self.fading) {
            (Some(l), _) => (l, true),
            (None, Some(l)) => (l, false),
            (None, None) => return free,
        };
        let rate = if active.ease > 0.0 { dt / active.ease } else { 1.0 };
        self.weight = if rising { (self.weight + rate).min(1.0) } else { (self.weight - rate).max(0.0) };
        if !rising && self.weight <= 0.0 {
            self.fading = None;
            return free;
        }
        let limited = active.clamp(free, self.entry_yaw);
        let w = smoothstep(self.weight);
        Look { yaw: wrap_angle(free.yaw + wrap_angle(limited.yaw - free.yaw) * w), pitch: free.pitch + (limited.pitch - free.pitch) * w }
    }
}
