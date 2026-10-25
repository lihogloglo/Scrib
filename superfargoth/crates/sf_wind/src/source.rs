//! Local wind sources: what gameplay adds (explosions, spells, storms).

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

/// Capacity of the source list (one storage buffer of this many entries).
pub const MAX_SOURCES: usize = 32;

/// The shape of a source's flow.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WindSourceKind {
    /// A uniform push along `direction` (a gust, a fan, a door draught).
    Directional { direction: Vec3 },
    /// Outward from the center (inward for negative strength): blasts,
    /// spell bursts, a storm radiating from a volcano.
    Radial,
    /// Around `axis` through the center, right-handed for positive
    /// strength: dust devils, whirlwinds.
    Vortex { axis: Vec3 },
}

/// One local source. Velocities add to the global wind.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindSource {
    pub kind: WindSourceKind,
    pub center: WorldPos,
    /// Peak speed in m/s.
    pub strength: f32,
    /// Full strength inside this radius (m)...
    pub inner_radius: f32,
    /// ...fading smoothly to zero here (m).
    pub radius: f32,
    /// Seconds to ramp up after it is added.
    pub attack: f32,
    /// Seconds from being added until it is removed; `None` lasts until
    /// [`crate::WindState::remove_source`].
    pub duration: Option<f32>,
    /// Seconds of fade-out before `duration` ends.
    pub release: f32,
}

impl WindSource {
    fn new(kind: WindSourceKind, center: WorldPos, strength: f32, radius: f32) -> Self {
        Self {
            kind,
            center,
            strength,
            inner_radius: 0.0,
            radius,
            attack: 0.0,
            duration: None,
            release: 0.0,
        }
    }

    pub fn directional(center: WorldPos, direction: Vec3, strength: f32, radius: f32) -> Self {
        let direction = direction.normalize_or(Vec3::X);
        Self::new(WindSourceKind::Directional { direction }, center, strength, radius)
    }

    pub fn radial(center: WorldPos, strength: f32, radius: f32) -> Self {
        Self::new(WindSourceKind::Radial, center, strength, radius)
    }

    pub fn vortex(center: WorldPos, axis: Vec3, strength: f32, radius: f32) -> Self {
        let axis = axis.normalize_or(Vec3::Y);
        Self::new(WindSourceKind::Vortex { axis }, center, strength, radius)
    }

    #[must_use]
    pub fn with_inner_radius(mut self, inner: f32) -> Self {
        self.inner_radius = inner;
        self
    }

    /// A timed source: ramps up over `attack`, removed after `duration`,
    /// fading out over the last `release` seconds.
    #[must_use]
    pub fn with_lifetime(mut self, attack: f32, duration: f32, release: f32) -> Self {
        self.attack = attack;
        self.duration = Some(duration);
        self.release = release;
        self
    }

    /// Strength multiplier `[0, 1]` at `age` seconds after being added.
    pub fn envelope(&self, age: f32) -> f32 {
        let rise = if self.attack > 0.0 { (age / self.attack).clamp(0.0, 1.0) } else { 1.0 };
        let fall = match self.duration {
            Some(d) if self.release > 0.0 => ((d - age) / self.release).clamp(0.0, 1.0),
            Some(d) if age >= d => 0.0,
            _ => 1.0,
        };
        rise * fall
    }

    /// Whether a timed source is over at `age`.
    pub fn expired(&self, age: f32) -> bool {
        self.duration.is_some_and(|d| age >= d)
    }
}

/// Handle returned by [`crate::WindState::add_source`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindSourceId(pub u64);
