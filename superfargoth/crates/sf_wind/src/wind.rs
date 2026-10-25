//! The weather's wind (moved here from `sf_sky`, which re-exports it).

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

use crate::{WindSettings, WindState};

/// Wind over the world: a mean velocity plus gusts. What weather sets and
/// blends; [`WindState`] turns it into the sampled field.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Wind {
    /// Mean horizontal velocity in m/s (x, z in the engine frame) at the
    /// reference height ([`WindSettings::reference_height`]).
    pub velocity: Vec2,
    /// Gust amplitude relative to the mean speed, `[0, 1]`: the speed swings
    /// between `1 - gustiness` (lulls) and `1 + gustiness` (gust fronts).
    pub gustiness: f32,
    /// Size of a gust cell in meters (snapped so it divides
    /// [`crate::PERIOD`]).
    pub gust_length: f32,
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(2.0, 0.5),
            gustiness: 0.3,
            gust_length: 60.0,
        }
    }
}

impl Wind {
    /// Linear blend; direction blends as a vector, so a 180° change passes
    /// through calm, not sideways.
    #[must_use]
    pub fn lerp(&self, to: &Self, t: f32) -> Self {
        Self {
            velocity: self.velocity.lerp(to.velocity, t),
            gustiness: self.gustiness + (to.gustiness - self.gustiness) * t,
            gust_length: self.gust_length + (to.gust_length - self.gust_length) * t,
        }
    }

    /// Mean speed in m/s.
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    /// Wind velocity (m/s) at absolute position `pos` (meters) and time `t`
    /// (seconds) for a wind that stayed constant since `t = 0`, at the
    /// reference height, with default [`WindSettings`] and no local sources.
    /// Stateless convenience; per-frame consumers use [`WindState::frame`].
    pub fn velocity_at(&self, pos: Vec3, t: f32) -> Vec3 {
        let settings = WindSettings::default();
        let mut state = WindState::new(*self);
        state.advance(t, &settings);
        state.velocity_at(&settings, WorldPos::from_f64(pos.as_dvec3()), settings.reference_height)
    }
}
