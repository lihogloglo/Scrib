//! Parametric point lights and an enclosed environment (light-cost lab,
//! spec `docs/specs/test-levels.md`, "Lights"). Lights are generated from a
//! few parameters, not hand-placed, so a perf sweep can step one variable
//! (count, layout, radius, motion) from the command line.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// `count` point lights laid out around `at`. The host maps `radius` (its
/// light size, meters) onto its own range and power, as for its authored lights.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LightRig {
    pub count: u32,
    #[serde(default)]
    pub layout: LightLayout,
    /// Layout center; `at.y` is the lights' height.
    pub at: Vec3,
    /// The lights' nominal radius, meters.
    pub radius: f32,
    /// sRGB colour, 0..=1.
    #[serde(default = "warm")]
    pub color: Vec3,
    /// Every light orbits `at` at this rate, degrees per second (0: static);
    /// models carried torches (a moving light can't reuse cached shadow maps).
    #[serde(default)]
    pub orbit: f32,
}

fn warm() -> Vec3 {
    Vec3::new(1.0, 0.85, 0.6)
}

/// How a [`LightRig`] places its lights.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LightLayout {
    /// Rows of `ceil(sqrt(count))` lights `spacing` meters apart, centered.
    Grid { spacing: f32 },
    /// Evenly spaced on a horizontal circle of `radius` meters.
    Ring { radius: f32 },
}

impl Default for LightLayout {
    fn default() -> Self {
        Self::Grid { spacing: 3.0 }
    }
}

impl LightRig {
    /// The lights' positions `t` seconds in (orbiting about `at` by `orbit`).
    pub fn positions(&self, t: f32) -> Vec<Vec3> {
        let n = self.count as usize;
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let local: Vec<Vec3> = match self.layout {
            LightLayout::Grid { spacing } => {
                let cols = (n as f32).sqrt().ceil().max(1.0) as usize;
                let rows = n.div_ceil(cols);
                (0..n)
                    .map(|i| {
                        let (c, r) = ((i % cols) as f32, (i / cols) as f32);
                        Vec3::new((c - 0.5 * (cols as f32 - 1.0)) * spacing, 0.0, (r - 0.5 * (rows as f32 - 1.0)) * spacing)
                    })
                    .collect()
            }
            LightLayout::Ring { radius } => (0..n)
                .map(|i| {
                    let a = std::f32::consts::TAU * i as f32 / n as f32;
                    Vec3::new(radius * a.cos(), 0.0, radius * a.sin())
                })
                .collect(),
        };
        let turn = glam::Quat::from_rotation_y((self.orbit * t).to_radians());
        local.into_iter().map(|p| self.at + turn * p).collect()
    }

    pub(crate) fn problem(&self) -> Option<String> {
        let positive = |v: f32| v.is_finite() && v > 0.0;
        let layout = match self.layout {
            LightLayout::Grid { spacing: s } | LightLayout::Ring { radius: s } => positive(s),
        };
        if self.count > 1024 {
            Some(format!("lights: count {} over 1024", self.count))
        } else if !(layout && positive(self.radius) && self.at.is_finite() && self.color.is_finite() && self.orbit.is_finite()) {
            Some("lights: layout size and radius must be > 0, everything finite".into())
        } else {
            None
        }
    }
}

/// An enclosed level: no sky or sun, this ambient light instead (the host
/// treats it as an interior).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interior {
    /// sRGB ambient colour, 0..=1.
    #[serde(default = "dim")]
    pub ambient: Vec3,
}

fn dim() -> Vec3 {
    Vec3::splat(0.1)
}
