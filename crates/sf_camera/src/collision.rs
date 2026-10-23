//! Camera collision (camera.md "Collision"): a sphere cast from the pivot to the desired eye
//! through the same [`GroundProbe`] the mover uses. Push in at once on a hit, ease back out
//! slower, never put the eye inside geometry; fade the character as the eye gets close.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_controller::{GroundProbe, ProbeQuery};
use sf_coords::WorldPos;

use crate::{ease_factor, smoothstep};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollisionParams {
    /// Sphere radius of the eye (covers the near plane's corners), metres.
    pub radius: f32,
    /// Extra gap kept between the eye sphere and the surface, metres.
    pub margin: f32,
    /// Ease-out half-life once the obstruction clears (push-in is immediate).
    pub out_halflife: f32,
    /// Arm length where the character starts fading (`fade` 0) and is fully faded (`fade` 1).
    pub fade_start: f32,
    pub fade_end: f32,
}

impl Default for CollisionParams {
    fn default() -> Self {
        Self { radius: 0.2, margin: 0.05, out_halflife: 0.35, fade_start: 0.9, fade_end: 0.45 }
    }
}

/// One frame of the collision-resolved arm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArmResult {
    pub eye: WorldPos,
    /// Pivot-to-eye distance now in use, metres.
    pub length: f32,
    /// Longest free length along the arm this frame (the cast).
    pub free: f32,
    /// Near-character fade for the render body's dither, 0 (opaque) to 1 (hidden).
    pub fade: f32,
    pub blocked: bool,
}

/// Frame state of the arm (presentation only): how far it is pushed in. The push-in depth, not
/// the length, is what eases, so a desired arm that grows by itself (zoom, mode distance, both
/// eased by the rig already) is followed exactly and only the recovery from an obstruction eases.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ArmCollision {
    depth: Option<f32>,
}

impl ArmCollision {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.depth = None;
    }

    /// Resolve the arm `pivot + arm` against `probe`. The eye never goes past the cast's free
    /// length, so with the pivot outside geometry the eye sphere is outside it too.
    pub fn update(&mut self, probe: &impl GroundProbe, pivot: WorldPos, arm: Vec3, p: &CollisionParams, dt: f32) -> ArmResult {
        let want = arm.length();
        if want <= 1e-4 {
            self.depth = Some(0.0);
            return ArmResult { eye: pivot, length: 0.0, free: 0.0, fade: 1.0, blocked: false };
        }
        let dir = arm / want;
        let reach = want + p.margin;
        let hit = probe.probe_one(ProbeQuery::sphere(pivot, dir, reach, p.radius + p.margin));
        let free = hit.map_or(want, |h| h.distance.min(want));
        let blocked_by = want - free;
        let depth = match self.depth {
            // Push in at once; never longer than what is free this frame.
            Some(cur) if blocked_by >= cur => blocked_by,
            Some(cur) => (cur + (blocked_by - cur) * ease_factor(p.out_halflife, dt)).max(blocked_by),
            None => blocked_by,
        };
        self.depth = Some(depth);
        let length = want - depth;
        let span = (p.fade_start - p.fade_end).max(1e-4);
        ArmResult {
            eye: pivot.offset(dir * length),
            length,
            free,
            fade: smoothstep((p.fade_start - length) / span),
            blocked: hit.is_some_and(|h| h.distance < want),
        }
    }
}
