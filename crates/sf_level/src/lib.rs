//! Synthetic test levels (spec: `docs/specs/test-levels.md`).
//!
//! A level is a small greybox space authored as a RON file: primitives
//! (ground, box, ramp, stairs, cylinder) with a material each, named
//! cameras, and spawn points. This crate is Bevy-free: it parses and
//! validates the file and builds each primitive's flat-shaded triangle mesh
//! (positions, normals, meter UVs, indices), which the host uses for both the
//! render mesh and the static collider, so what you see is what you collide
//! with. The host owns spawning, materials, sky and physics.
//!
//! Frame: meters, y up, x east, z south (Bevy's). Every primitive's `at` is
//! the center of its footprint at its bottom; ramps and stairs rise toward
//! their local -z (Bevy's forward), turned by `yaw` degrees about +y.

use std::collections::HashSet;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

mod mesh;
#[cfg(test)]
mod tests;

pub use mesh::MeshData;

/// A level file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Level {
    /// Short kebab-case name (the file stem by convention).
    pub name: String,
    /// One line: what the level is for.
    #[serde(default)]
    pub description: String,
    /// Named viewpoints; the first is the default.
    pub cameras: Vec<CameraDef>,
    /// Where characters start (hosts spawn the player or actors here).
    #[serde(default)]
    pub spawns: Vec<Spawn>,
    pub objects: Vec<Object>,
}

/// A viewpoint: eye and target in meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraDef {
    pub name: String,
    pub eye: Vec3,
    pub target: Vec3,
}

/// A named start point: feet position and facing (yaw degrees, 0 = -z).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spawn {
    pub name: String,
    pub at: Vec3,
    #[serde(default)]
    pub yaw: f32,
}

/// One primitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    /// Optional label (shown as the entity name; unique when given).
    #[serde(default)]
    pub name: Option<String>,
    pub shape: Shape,
    /// Center of the footprint at the bottom, meters.
    #[serde(default)]
    pub at: Vec3,
    /// Degrees about +y.
    #[serde(default)]
    pub yaw: f32,
    #[serde(default)]
    pub material: Material,
    /// Whether it gets a static collider.
    #[serde(default = "yes")]
    pub collide: bool,
}

fn yes() -> bool {
    true
}

impl Object {
    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw.to_radians())
    }

    /// The local mesh (see [`Shape::mesh`]).
    pub fn mesh(&self) -> MeshData {
        self.shape.mesh()
    }
}

/// Greybox primitives. Sizes in meters, angles in degrees.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    /// A floor slab `size` (x, z) wide whose top is at `at.y` (1 m thick).
    Ground { size: (f32, f32) },
    /// An axis-aligned box (x, y, z).
    Box { size: Vec3 },
    /// A wedge rising `rise` meters at `angle` degrees toward -z; its run is
    /// `rise / tan(angle)`.
    Ramp { width: f32, rise: f32, angle: f32 },
    /// Solid stairs climbing toward -z: `steps` steps of `rise` × `run`.
    Stairs { width: f32, steps: u32, rise: f32, run: f32 },
    /// An upright cylinder (pole, pillar).
    Cylinder { radius: f32, height: f32 },
}

impl Shape {
    /// The footprint depth along z (ramps and stairs: their run).
    pub fn length(&self) -> f32 {
        match *self {
            Self::Ground { size } => size.1,
            Self::Box { size } => size.z,
            Self::Ramp { rise, angle, .. } => rise / angle.to_radians().tan(),
            #[allow(clippy::cast_precision_loss)]
            Self::Stairs { steps, run, .. } => steps as f32 * run,
            Self::Cylinder { radius, .. } => 2.0 * radius,
        }
    }

    /// Height of the top above `at.y` (ground: 0, it extends below).
    pub fn height(&self) -> f32 {
        match *self {
            Self::Ground { .. } => 0.0,
            Self::Box { size } => size.y,
            Self::Ramp { rise, .. } => rise,
            #[allow(clippy::cast_precision_loss)]
            Self::Stairs { steps, rise, .. } => steps as f32 * rise,
            Self::Cylinder { height, .. } => height,
        }
    }

    /// Flat-shaded triangles in the object's frame (render and collision).
    pub fn mesh(&self) -> MeshData {
        mesh::build(self)
    }
}

/// A flat colour on the shared meter grid (prototype-grid style: every
/// surface shows 1 m squares, so scale reads in screenshots).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Material {
    /// Light grey: floors.
    #[default]
    Floor,
    /// Mid grey: walls and generic blocks.
    Grey,
    Dark,
    White,
    Orange,
    Yellow,
    Green,
    Blue,
    Red,
    Purple,
}

impl Material {
    /// Linear-ish sRGB base colour.
    pub fn srgb(self) -> [f32; 3] {
        match self {
            Self::Floor => [0.62, 0.62, 0.60],
            Self::Grey => [0.42, 0.42, 0.44],
            Self::Dark => [0.16, 0.16, 0.18],
            Self::White => [0.85, 0.85, 0.83],
            Self::Orange => [0.90, 0.45, 0.12],
            Self::Yellow => [0.88, 0.76, 0.18],
            Self::Green => [0.30, 0.62, 0.28],
            Self::Blue => [0.22, 0.42, 0.80],
            Self::Red => [0.75, 0.18, 0.16],
            Self::Purple => [0.52, 0.30, 0.70],
        }
    }
}

/// A level that failed to parse or validate.
#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error("{0}")]
    Parse(#[from] ron::error::SpannedError),
    #[error("level {level:?}: {problems:?}")]
    Invalid { level: String, problems: Vec<String> },
}

impl Level {
    /// Parses and validates a level file.
    pub fn parse(text: &str) -> Result<Self, LevelError> {
        let level: Self = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(text)?;
        let problems = level.problems();
        if problems.is_empty() { Ok(level) } else { Err(LevelError::Invalid { level: level.name, problems }) }
    }

    /// Everything wrong with the level (empty when valid).
    pub fn problems(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.trim().is_empty() {
            out.push("empty name".to_owned());
        }
        if self.cameras.is_empty() {
            out.push("no cameras".to_owned());
        }
        if self.objects.is_empty() {
            out.push("no objects".to_owned());
        }
        let mut names = HashSet::new();
        for c in &self.cameras {
            if !names.insert(("camera", c.name.to_ascii_lowercase())) {
                out.push(format!("camera {:?} defined twice", c.name));
            }
            if !(c.eye.is_finite() && c.target.is_finite()) || c.eye.distance(c.target) < 1e-3 {
                out.push(format!("camera {:?}: eye and target must be finite and apart", c.name));
            }
        }
        for s in &self.spawns {
            if !names.insert(("spawn", s.name.to_ascii_lowercase())) {
                out.push(format!("spawn {:?} defined twice", s.name));
            }
            if !s.at.is_finite() || !s.yaw.is_finite() {
                out.push(format!("spawn {:?}: not finite", s.name));
            }
        }
        for (i, o) in self.objects.iter().enumerate() {
            let label = o.name.clone().unwrap_or_else(|| format!("#{i}"));
            if let Some(n) = &o.name
                && !names.insert(("object", n.to_ascii_lowercase()))
            {
                out.push(format!("object {n:?} defined twice"));
            }
            if !o.at.is_finite() || !o.yaw.is_finite() {
                out.push(format!("object {label}: position or yaw not finite"));
            }
            if let Some(p) = o.shape.problem() {
                out.push(format!("object {label}: {p}"));
            }
        }
        out
    }
}

impl Shape {
    fn problem(&self) -> Option<String> {
        let positive = |v: f32| v.is_finite() && v > 0.0;
        match *self {
            Self::Ground { size } if !(positive(size.0) && positive(size.1)) => Some("ground size must be > 0".into()),
            Self::Box { size } if !(positive(size.x) && positive(size.y) && positive(size.z)) => Some("box size must be > 0".into()),
            Self::Ramp { width, rise, angle } => {
                if !(positive(width) && positive(rise)) {
                    Some("ramp width and rise must be > 0".into())
                } else if !(angle.is_finite() && angle > 0.5 && angle < 89.5) {
                    Some(format!("ramp angle {angle}° outside (0.5, 89.5)"))
                } else {
                    None
                }
            }
            Self::Stairs { width, steps, rise, run } => {
                if !(1..=500).contains(&steps) {
                    Some(format!("stairs: {steps} steps outside 1..=500"))
                } else if !(positive(width) && positive(rise) && positive(run)) {
                    Some("stairs width, rise and run must be > 0".into())
                } else {
                    None
                }
            }
            Self::Cylinder { radius, height } if !(positive(radius) && positive(height)) => {
                Some("cylinder radius and height must be > 0".into())
            }
            _ => None,
        }
    }
}
