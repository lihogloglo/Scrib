//! Footsteps (audio.md "Footsteps"): a generic hook from (animation event, surface) to a cue.
//! Event names come from the animation's text keys / notifies, surface tags from the host's
//! ground query (a material, a texture class); both are content strings, never framework ones.
//! `StrideTimer` is the fallback trigger for rigs without step events: one step per stride of
//! travelled distance, the classic distance-based cadence.

use std::collections::HashMap;

use sf_records::RecordId;

/// (event, surface) -> cue. A `None` surface is the event's fallback on unknown ground.
#[derive(Clone, Debug, Default)]
pub struct FootstepMap {
    map: HashMap<(String, Option<String>), RecordId>,
}

impl FootstepMap {
    /// Maps `event` on `surface` (`None` = any surface without its own entry) to `cue`.
    /// Keys are case-insensitive.
    pub fn insert(&mut self, event: &str, surface: Option<&str>, cue: RecordId) {
        self.map.insert((event.to_ascii_lowercase(), surface.map(str::to_ascii_lowercase)), cue);
    }

    /// The cue for `event` on `surface`: the exact pair first, then the event's fallback.
    pub fn resolve(&self, event: &str, surface: Option<&str>) -> Option<&RecordId> {
        let event = event.to_ascii_lowercase();
        surface
            .and_then(|s| self.map.get(&(event.clone(), Some(s.to_ascii_lowercase()))))
            .or_else(|| self.map.get(&(event, None)))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// Which foot a distance-driven step lands on (alternates).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foot {
    Left,
    Right,
}

/// Distance-based step trigger: fires each `stride_m` of ground travel, the first one after
/// half a stride so a character that starts walking is heard at once.
#[derive(Clone, Debug)]
pub struct StrideTimer {
    travelled_m: f32,
    next: Foot,
}

impl Default for StrideTimer {
    fn default() -> Self {
        Self { travelled_m: f32::NAN, next: Foot::Left }
    }
}

impl StrideTimer {
    /// Adds `distance_m` of ground travel at the current `stride_m` (longer when running);
    /// returns the foot when a step lands. At most one step per call.
    pub fn advance(&mut self, distance_m: f32, stride_m: f32) -> Option<Foot> {
        let stride = stride_m.max(0.05);
        if self.travelled_m.is_nan() {
            self.travelled_m = stride * 0.5;
        }
        self.travelled_m += distance_m.max(0.0);
        if self.travelled_m < stride {
            return None;
        }
        self.travelled_m = (self.travelled_m - stride).min(stride);
        let foot = self.next;
        self.next = if foot == Foot::Left { Foot::Right } else { Foot::Left };
        Some(foot)
    }

    /// Call when airborne or standing still: the next walk starts with a half stride.
    pub fn reset(&mut self) {
        self.travelled_m = f32::NAN;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_surface_then_event_fallback() {
        let mut m = FootstepMap::default();
        m.insert("FootLeft", None, RecordId::new("step_dirt"));
        m.insert("footleft", Some("Stone"), RecordId::new("step_stone"));
        assert_eq!(m.resolve("footleft", Some("stone")).map(RecordId::as_str), Some("step_stone"));
        assert_eq!(m.resolve("FOOTLEFT", Some("sand")).map(RecordId::as_str), Some("step_dirt"));
        assert_eq!(m.resolve("footleft", None).map(RecordId::as_str), Some("step_dirt"));
        assert!(m.resolve("footright", Some("stone")).is_none());
    }

    #[test]
    fn stride_timer_alternates_feet_each_stride() {
        let mut t = StrideTimer::default();
        let mut steps = Vec::new();
        // 10 m at 0.1 m per frame with a 1 m stride: first step at 0.5 m, then every 1 m.
        for i in 0..100 {
            if let Some(f) = t.advance(0.1, 1.0) {
                steps.push((i, f));
            }
        }
        assert_eq!(steps.len(), 10);
        assert!((3..=5).contains(&steps[0].0), "{steps:?}");
        assert!(steps.windows(2).all(|w| w[0].1 != w[1].1));
        t.reset();
        assert!(t.advance(0.3, 1.0).is_none());
        assert!(t.advance(0.3, 1.0).is_some());
    }
}
