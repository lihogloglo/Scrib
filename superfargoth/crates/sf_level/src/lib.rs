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
//! Part 2 (TS1b): `actors` (a host record id at a spawn, an optional outfit,
//! a scripted path and a periodic action; ids are opaque strings here) and
//! props (`dynamic: Some(mass)` boxes the host simulates; `interact` is a
//! named hook for interaction points, unused until P11).
//!
//! Frame: meters, y up, x east, z south (Bevy's). Every primitive's `at` is
//! the center of its footprint at its bottom; ramps and stairs rise toward
//! their local -z (Bevy's forward), turned by `yaw` degrees about +y.

use std::collections::HashSet;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

mod lights;
mod mesh;
#[cfg(test)]
mod tests;

pub use lights::{Interior, LightLayout, LightRig};
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
    /// Characters the host spawns and drives (see [`Actor`]).
    #[serde(default)]
    pub actors: Vec<Actor>,
    /// Parametric point lights (the light-cost lab, see [`LightRig`]).
    #[serde(default)]
    pub lights: Option<LightRig>,
    /// Enclosed: no sky, this ambient (see [`Interior`]).
    #[serde(default)]
    pub interior: Option<Interior>,
    /// Time of day in hours the host starts the level at, unless told otherwise.
    #[serde(default)]
    pub hour: Option<f64>,
    /// The host's weather name (e.g. "clear") it pins, unless told otherwise; opaque here.
    #[serde(default)]
    pub weather: Option<String>,
}

/// A character at a spawn point. `id`, `outfit` and the action's names are
/// the host's (record ids, item ids, animation-graph parameters): this crate
/// only checks they are not empty.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    /// The host's character record id.
    pub id: String,
    /// A spawn name: where it starts and which way it faces.
    pub spawn: String,
    /// Worn items replacing the record's own equipment (host item ids).
    #[serde(default)]
    pub outfit: Option<Vec<String>>,
    /// A scripted walk; none stands at the spawn.
    #[serde(default)]
    pub path: Option<ActorPath>,
    /// A named animation action fired periodically while standing (at the
    /// spawn, or waiting at a path's stops).
    #[serde(default)]
    pub action: Option<Action>,
}

/// A scripted walk: feet positions visited in order (the host steers its
/// mover through them; corners are cut, stops are waited at).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActorPath {
    pub points: Vec<Vec3>,
    #[serde(default)]
    pub mode: PathMode,
    /// Cruise speed, m/s.
    #[serde(default = "walk_speed")]
    pub speed: f32,
    /// Run gait instead of walk (the speed still decides how fast).
    #[serde(default)]
    pub run: bool,
    /// Seconds standing at each stop (the ends of a back-and-forth, the
    /// last point of a one-way walk, the first point of a loop).
    #[serde(default)]
    pub wait: f32,
}

fn walk_speed() -> f32 {
    1.4
}

/// What happens at the last point.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathMode {
    /// From the last point back to the first, forever.
    #[default]
    Loop,
    /// Back and forth; the ends are stops.
    PingPong,
    /// Walk once and stop at the last point.
    Once,
}

/// A periodic named action: `params` are set, then `trigger` fires, every
/// `every` seconds while the actor stands.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub trigger: String,
    #[serde(default = "action_every")]
    pub every: f32,
    #[serde(default)]
    pub params: Vec<(String, f32)>,
}

fn action_every() -> f32 {
    6.0
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
    /// A prop: a dynamic body of this mass (kg) instead of a static
    /// collider (boxes only).
    #[serde(default)]
    pub dynamic: Option<f32>,
    /// Interaction-point hook (P11: sit, open, pick up). Parsed and kept for
    /// the host; nothing uses it yet.
    #[serde(default)]
    pub interact: Option<String>,
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

    /// The footprint (x width, z length) before `yaw`.
    pub fn footprint(&self) -> (f32, f32) {
        let width = match *self {
            Self::Ground { size } => size.0,
            Self::Box { size } => size.x,
            Self::Ramp { width, .. } | Self::Stairs { width, .. } => width,
            Self::Cylinder { radius, .. } => 2.0 * radius,
        };
        (width, self.length())
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
            if let Some(mass) = o.dynamic {
                if !matches!(o.shape, Shape::Box { .. }) {
                    out.push(format!("object {label}: only boxes can be dynamic"));
                }
                if !(mass.is_finite() && mass > 0.0) {
                    out.push(format!("object {label}: mass {mass} must be > 0"));
                }
            }
        }
        for (i, a) in self.actors.iter().enumerate() {
            let label = format!("actor #{i} {:?}", a.id);
            if a.id.trim().is_empty() {
                out.push(format!("{label}: empty id"));
            }
            if self.spawn(&a.spawn).is_none() {
                out.push(format!("{label}: no spawn {:?}", a.spawn));
            }
            if a.outfit.as_ref().is_some_and(|o| o.iter().any(|item| item.trim().is_empty())) {
                out.push(format!("{label}: empty outfit item"));
            }
            if let Some(p) = &a.path {
                if p.points.len() < 2 {
                    out.push(format!("{label}: a path needs at least 2 points"));
                }
                if !p.points.iter().all(|v| v.is_finite()) {
                    out.push(format!("{label}: path point not finite"));
                }
                if !(p.speed.is_finite() && p.speed > 0.0 && p.wait.is_finite() && p.wait >= 0.0) {
                    out.push(format!("{label}: path speed must be > 0 and wait >= 0"));
                }
            }
            if let Some(action) = &a.action
                && (action.trigger.trim().is_empty() || !(action.every.is_finite() && action.every > 0.0))
            {
                out.push(format!("{label}: action needs a trigger and every > 0"));
            }
        }
        if let Some(p) = self.lights.as_ref().and_then(LightRig::problem) {
            out.push(p);
        }
        if self.interior.as_ref().is_some_and(|i| !i.ambient.is_finite()) {
            out.push("interior: ambient not finite".to_owned());
        }
        out
    }

    /// A spawn by name (case-insensitive).
    pub fn spawn(&self, name: &str) -> Option<&Spawn> {
        self.spawns.iter().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    /// The horizontal bounds (min, max) of every object's footprint, a
    /// conservative box (each footprint's circumscribed square); `None`
    /// without objects. Hosts use it to cover the level (streaming cells).
    pub fn bounds(&self) -> Option<(Vec3, Vec3)> {
        self.objects.iter().fold(None, |acc, o| {
            let (w, l) = o.shape.footprint();
            let r = 0.5 * w.hypot(l);
            let (lo, hi) = (o.at - Vec3::new(r, 0.0, r), o.at + Vec3::new(r, o.shape.height(), r));
            Some(acc.map_or((lo, hi), |(a, b): (Vec3, Vec3)| (a.min(lo), b.max(hi))))
        })
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
