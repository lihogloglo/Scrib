//! Rings: what is resident at which distance. One table, shared with LOD
//! selection (`VisibilityRange` reads [`RingConfig::resident_until`]).

use serde::{Deserialize, Serialize};

use crate::RingOverrides;

/// A streaming cell's residency level. Levels are cumulative: a cell at
/// `Near` also holds everything `Far` holds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Level {
    /// Nothing resident.
    #[default]
    Unloaded,
    /// R2: placements with bounds radius ≥ `far_min_radius`, coarsest LOD.
    Far,
    /// R1: every visual placement and light, full LOD chain; dynamic
    /// objects frozen, visual-only.
    Near,
    /// R0: everything, simulated (physics, scripts, AI).
    Active,
}

impl Level {
    /// The loaded levels, lowest first.
    pub const LOADED: [Self; 3] = [Self::Far, Self::Near, Self::Active];

    pub fn up(self) -> Option<Self> {
        match self {
            Self::Unloaded => Some(Self::Far),
            Self::Far => Some(Self::Near),
            Self::Near => Some(Self::Active),
            Self::Active => None,
        }
    }

    pub fn down(self) -> Option<Self> {
        match self {
            Self::Unloaded => None,
            Self::Far => Some(Self::Unloaded),
            Self::Near => Some(Self::Far),
            Self::Active => Some(Self::Near),
        }
    }
}

/// Load and unload radius of one ring, in meters, measured from an anchor
/// to a cell's content bounds. `unload > load` is the hysteresis band.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RingRadii {
    pub load: f32,
    pub unload: f32,
}

impl RingRadii {
    pub const fn new(load: f32, unload: f32) -> Self {
        Self { load, unload }
    }

    #[must_use]
    pub fn scaled(self, s: f32) -> Self {
        Self { load: self.load * s, unload: self.unload * s }
    }
}

/// The ring table. Defaults are the "high" quality preset (streaming.md).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RingConfig {
    /// R0.
    pub active: RingRadii,
    /// R1.
    pub near: RingRadii,
    /// R2.
    pub far: RingRadii,
    /// R3: terrain-only view distance (terrain clipmaps own its residency).
    pub horizon: f32,
    /// Placements at least this large (bounds radius, m) are part of R2.
    pub far_min_radius: f32,
}

impl Default for RingConfig {
    fn default() -> Self {
        Self {
            active: RingRadii::new(192.0, 256.0),
            near: RingRadii::new(768.0, 896.0),
            far: RingRadii::new(3000.0, 3250.0),
            horizon: 8192.0,
            far_min_radius: 4.0,
        }
    }
}

impl RingConfig {
    /// The radii of the ring that makes cells resident at `level`.
    /// `Unloaded` has none (zero).
    pub fn radii(&self, level: Level) -> RingRadii {
        match level {
            Level::Unloaded => RingRadii::new(0.0, 0.0),
            Level::Far => self.far,
            Level::Near => self.near,
            Level::Active => self.active,
        }
    }

    /// Scales every radius (quality presets).
    #[must_use]
    pub fn scaled(&self, s: f32) -> Self {
        Self {
            active: self.active.scaled(s),
            near: self.near.scaled(s),
            far: self.far.scaled(s),
            horizon: self.horizon * s,
            far_min_radius: self.far_min_radius,
        }
    }

    /// This table with a worldspace's overrides applied.
    #[must_use]
    pub fn with_overrides(&self, o: &RingOverrides) -> Self {
        Self {
            active: o.active.unwrap_or(self.active),
            near: o.near.unwrap_or(self.near),
            far: o.far.unwrap_or(self.far),
            horizon: o.horizon.unwrap_or(self.horizon),
            far_min_radius: self.far_min_radius,
        }
    }

    /// The distance up to which content of `level` can be resident (its
    /// unload radius): the upper bound for LOD visibility ranges.
    pub fn resident_until(&self, level: Level) -> f32 {
        self.radii(level).unload
    }

    /// The largest unload radius of any placement ring.
    pub fn max_unload(&self) -> f32 {
        self.active.unload.max(self.near.unload).max(self.far.unload)
    }
}
