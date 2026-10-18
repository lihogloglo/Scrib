//! Character cloth as a bone lattice solved with XPBD (physics.md "Cloth";
//! M4 P10). Bevy-free: the cook (`cloth.cook`, `sf_character`) builds a
//! [`ClothLattice`]; the procedural op (`ClothLattice` in `sf_anim_graph`)
//! feeds [`ClothState::step`] the animated particle targets and the body
//! capsules each update and aims the lattice joints at the result.
//!
//! The technique is the standard character-cloth stack:
//!
//! - **XPBD with small steps** (Macklin, Müller, Chentanez, *XPBD*, MIG 2016;
//!   Macklin et al., *Small Steps in Physics Simulation*, SCA 2019): a fixed
//!   1/240 s substep accumulated over the frame, one constraint pass per
//!   substep, compliances in m/N so stiffness does not depend on the step.
//! - **Character space with inertia scaling** (UE Chaos Cloth's fictitious
//!   forces, `NvCloth`'s linear/angular inertia): particles live in the
//!   character's frame; the frame's linear acceleration and its yaw
//!   (Euler, centrifugal and Coriolis terms) enter as fictitious forces
//!   scaled by `inertia_linear` / `inertia_angular`. Constant velocity is
//!   free (Galilean), so a running robe trails only when the runner speeds
//!   up, stops or turns, and the amount is a tuning knob, not frame rate.
//! - **Long-range attachments** (Kim, Chentanez, Müller, *Long Range
//!   Attachments*, SCA 2012): one-sided tethers from each free particle to
//!   its column's pinned particle, at the geodesic (down-the-column) rest
//!   length, the "accurate" setup of Chaos's geodesic tethers. They stop
//!   the stretch a single Gauss-Seidel pass leaves in a long chain.
//! - **Max distance** to the animated position (Chaos, `NvCloth`, Jolt skinned
//!   soft bodies): 0 at the pins, growing down the lattice.
//! - **Capsule collision** with the body (legs, pelvis, tail), swept by
//!   interpolating each capsule across the substeps, plus a thickness.
//! - **Teleport detection**: a root jump or snap turn past a threshold keeps
//!   the cloth's character-space shape and drops its velocity, as Chaos's
//!   teleport mode does; [`ClothState::reset`] re-seeds from the animation
//!   (LOD cut).
//!
//! No crate fits (physics.md "Cloth": `bevy_silk` and `bevy_softbody` are
//! Bevy-bound mesh cloth, `rapier-cloth-core` has no character constraints,
//! avian has no cloth); the solver is small, so it is ours.

use glam::{Mat3, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::RootDelta;

/// The cloth substep, s (Small Steps: many substeps, one iteration each).
pub const CLOTH_SUBSTEP: f32 = 1.0 / 240.0;
/// Most substeps one update runs (a long hitch drops the rest of the time).
pub const CLOTH_MAX_SUBSTEPS: u32 = 16;

/// What a distance constraint holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    /// Neighbours along a row or column.
    Structural,
    /// Diagonal neighbours.
    Shear,
    /// Every other particle along a row or column.
    Bend,
}

impl EdgeKind {
    fn index(self) -> usize {
        match self {
            Self::Structural => 0,
            Self::Shear => 1,
            Self::Bend => 2,
        }
    }
}

/// One lattice particle (a cloth joint).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClothParticle {
    /// Rest position in character space (the bind pose).
    pub rest: Vec3,
    /// The skeleton joint the particle rides on when not simulated (pins:
    /// the girdle joint the cloth hangs from).
    pub joint: u16,
    /// Inverse mass, 1/kg; 0 pins the particle to its animated position.
    pub inv_mass: f32,
    /// How far the particle may leave its animated position, m.
    pub max_distance: f32,
}

/// A distance constraint between two particles.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClothEdge {
    pub a: u16,
    pub b: u16,
    /// Rest length, m.
    pub rest: f32,
    pub kind: EdgeKind,
}

/// A one-sided long-range attachment: `particle` stays within `length`
/// (× [`ClothParams::tether_scale`]) of the pinned `anchor`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClothTether {
    pub particle: u16,
    pub anchor: u16,
    /// Geodesic rest distance to the anchor, m.
    pub length: f32,
}

/// A body capsule the cloth collides with, in its joint's frame (from the
/// rig's ragdoll capsules: legs, pelvis, tail). The op poses it into a
/// [`Capsule`] each update.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClothCollider {
    pub joint: u16,
    /// Capsule centre in the joint's frame.
    pub center: Vec3,
    /// Unit axis in the joint's frame.
    pub axis: Vec3,
    pub radius: f32,
    /// Half the segment length (the cylinder part), m.
    pub half_length: f32,
}

/// A posed capsule, character space: the segment `a`–`b` swept by `radius`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Capsule {
    pub a: Vec3,
    pub b: Vec3,
    pub radius: f32,
}

impl Capsule {
    fn lerp(self, to: Self, t: f32) -> Self {
        Self { a: self.a.lerp(to.a, t), b: self.b.lerp(to.b, t), radius: self.radius + (to.radius - self.radius) * t }
    }

    /// The closest point on the segment to `p`.
    pub fn closest(&self, p: Vec3) -> Vec3 {
        let ab = self.b - self.a;
        let len2 = ab.length_squared();
        let t = if len2 > 1e-12 { ((p - self.a).dot(ab) / len2).clamp(0.0, 1.0) } else { 0.0 };
        self.a + ab * t
    }

    /// Signed distance from `p` to the capsule surface (negative inside).
    pub fn distance(&self, p: Vec3) -> f32 {
        (p - self.closest(p)).length() - self.radius
    }
}

/// How a grid of rest positions becomes a lattice ([`ClothLattice::grid`]).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridSpec {
    /// Rows from the attachment line (row 0, pinned) to the hem.
    pub rows: u8,
    /// Particles per row.
    pub cols: u8,
    /// A skirt or robe closes around the body; a cape is open.
    pub ring: bool,
    /// Mass of the free particles together, kg.
    pub mass: f32,
    /// Max distance as a fraction of each particle's tether length.
    pub max_distance_scale: f32,
}

impl Default for GridSpec {
    fn default() -> Self {
        Self { rows: 5, cols: 10, ring: true, mass: 1.5, max_distance_scale: 0.6 }
    }
}

/// A cooked cloth lattice (physics.md "Cloth lattice").
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClothLattice {
    pub rows: u8,
    pub cols: u8,
    pub ring: bool,
    /// Row-major: particle `r * cols + c`.
    pub particles: Vec<ClothParticle>,
    pub edges: Vec<ClothEdge>,
    pub tethers: Vec<ClothTether>,
    pub colliders: Vec<ClothCollider>,
}

impl ClothLattice {
    /// Builds a lattice from row-major rest positions (character space) and
    /// each particle's joint: row 0 is pinned; structural, shear and bend
    /// edges along and across; a tether from every free particle to its
    /// column's pin at the length down the column; max distance grows with
    /// that length. Colliders are left to the caller.
    ///
    /// # Panics
    /// If `rest` or `joints` don't hold `rows * cols` entries.
    #[allow(clippy::cast_precision_loss)]
    pub fn grid(spec: &GridSpec, rest: &[Vec3], joints: &[u16]) -> Self {
        let (rows, cols) = (usize::from(spec.rows), usize::from(spec.cols));
        let n = rows * cols;
        assert_eq!(rest.len(), n, "cloth grid: {rows} x {cols} rest positions expected");
        assert_eq!(joints.len(), n, "cloth grid: {rows} x {cols} joints expected");
        let id = |r: usize, c: usize| u16::try_from(r * cols + c % cols).expect("cloth lattice over 65535 particles");
        let free = (n - cols).max(1) as f32;
        let inv_mass = if spec.mass > 0.0 { free / spec.mass } else { 1.0 };
        // Geodesic distance down each column to the pin.
        let mut down = vec![0.0f32; n];
        for r in 1..rows {
            for c in 0..cols {
                down[r * cols + c] = down[(r - 1) * cols + c] + rest[r * cols + c].distance(rest[(r - 1) * cols + c]);
            }
        }
        let particles = (0..n)
            .map(|i| {
                let pinned = i < cols;
                ClothParticle {
                    rest: rest[i],
                    joint: joints[i],
                    inv_mass: if pinned { 0.0 } else { inv_mass },
                    max_distance: down[i] * spec.max_distance_scale,
                }
            })
            .collect();
        let mut edges = Vec::new();
        let mut edge = |a: u16, b: u16, kind| {
            let rest = rest[usize::from(a)].distance(rest[usize::from(b)]);
            edges.push(ClothEdge { a, b, rest, kind });
        };
        // Columns across a row: a ring wraps, an open sheet stops.
        let across = |step: usize| if spec.ring { cols } else { cols.saturating_sub(step) };
        for r in 0..rows {
            for c in 0..across(1) {
                edge(id(r, c), id(r, c + 1), EdgeKind::Structural);
            }
            for c in 0..across(2) {
                if cols > 2 {
                    edge(id(r, c), id(r, c + 2), EdgeKind::Bend);
                }
            }
        }
        for r in 1..rows {
            for c in 0..cols {
                edge(id(r - 1, c), id(r, c), EdgeKind::Structural);
            }
            for c in 0..across(1) {
                edge(id(r - 1, c), id(r, c + 1), EdgeKind::Shear);
                edge(id(r - 1, c + 1), id(r, c), EdgeKind::Shear);
            }
        }
        for r in 2..rows {
            for c in 0..cols {
                edge(id(r - 2, c), id(r, c), EdgeKind::Bend);
            }
        }
        let tethers = (1..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .map(|(r, c)| ClothTether { particle: id(r, c), anchor: id(0, c), length: down[r * cols + c] })
            .collect();
        Self { rows: spec.rows, cols: spec.cols, ring: spec.ring, particles, edges, tethers, colliders: Vec::new() }
    }

    /// The largest strain of the structural edges in `pos`, as a fraction of
    /// rest length (0.05 = 5 % stretch). `hem_only` limits it to the last row.
    pub fn max_stretch(&self, pos: &[Vec3], hem_only: bool) -> f32 {
        let hem = (usize::from(self.rows).saturating_sub(1)) * usize::from(self.cols);
        self.edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Structural && e.rest > 1e-6)
            .filter(|e| !hem_only || (usize::from(e.a) >= hem && usize::from(e.b) >= hem))
            .map(|e| (pos[usize::from(e.a)].distance(pos[usize::from(e.b)]) - e.rest) / e.rest)
            .fold(0.0, f32::max)
    }
}

/// Solver settings (an adapter's `ClothRules` picks them per part).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClothParams {
    /// XPBD compliance per [`EdgeKind`] (structural, shear, bend), m/N; 0 is rigid.
    pub compliance: [f32; 3],
    /// Velocity damping relative to the character, 1/s.
    pub damping: f32,
    /// Gravity in character space, m/s².
    pub gravity: Vec3,
    /// Fraction of the character's linear acceleration the cloth feels.
    pub inertia_linear: f32,
    /// Fraction of the character's turning (Euler, centrifugal, Coriolis) the cloth feels.
    pub inertia_angular: f32,
    /// Tether slack: 1 allows no stretch past the rest geodesic.
    pub tether_scale: f32,
    /// Cloth thickness added to every capsule radius, m.
    pub thickness: f32,
    /// A root move longer than this in one update is a teleport, m.
    pub teleport_distance: f32,
    /// A root turn larger than this in one update is a teleport, rad.
    pub teleport_angle: f32,
}

impl Default for ClothParams {
    fn default() -> Self {
        Self {
            compliance: [0.0, 2e-4, 2e-3],
            damping: 1.0,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            inertia_linear: 0.5,
            inertia_angular: 0.3,
            tether_scale: 1.0,
            thickness: 0.015,
            teleport_distance: 1.0,
            teleport_angle: 1.0,
        }
    }
}

/// [`ClothState::rotations`] for any positions `pos` (one per lattice
/// particle, character space), e.g. the animated targets of a cloth that
/// isn't simulated this update (animation LOD).
pub fn lattice_rotations(lattice: &ClothLattice, pos: &[Vec3], out: &mut Vec<Quat>) {
    out.clear();
    let (rows, cols) = (usize::from(lattice.rows), usize::from(lattice.cols));
    if pos.len() != rows * cols || rows == 0 || cols == 0 {
        return;
    }
    let frame = |p: &dyn Fn(usize) -> Vec3, r: usize, c: usize| {
        let at = |r: usize, c: usize| p(r * cols + c);
        let (r0, r1) = (r.saturating_sub(1), (r + 1).min(rows - 1));
        let (c0, c1) = if lattice.ring { ((c + cols - 1) % cols, (c + 1) % cols) } else { (c.saturating_sub(1), (c + 1).min(cols - 1)) };
        let down = at(r1, c) - at(r0, c);
        let along = at(r, c1) - at(r, c0);
        let x = along.normalize_or(Vec3::X);
        let y = (down - x * down.dot(x)).normalize_or(x.any_orthonormal_vector());
        Quat::from_mat3(&Mat3::from_cols(x, y, x.cross(y)))
    };
    let now = |i: usize| pos[i];
    let rest = |i: usize| lattice.particles[i].rest;
    for r in 0..rows {
        for c in 0..cols {
            out.push((frame(&now, r, c) * frame(&rest, r, c).inverse()).normalize());
        }
    }
}

/// One update's inputs, character space.
#[derive(Clone, Copy, Debug)]
pub struct ClothInputs<'a> {
    /// Each particle's animated (skinned) position now.
    pub animated: &'a [Vec3],
    /// The body capsules now (posed [`ClothCollider`]s), in the same order every update.
    pub capsules: &'a [Capsule],
    /// The character's motion since the last update (`UpdateInputs::moved`).
    pub moved: RootDelta,
    /// Seconds since the last update.
    pub dt: f32,
    /// Extra acceleration on the free particles (wind, drag), m/s².
    pub acceleration: Vec3,
}

/// What one [`ClothState::step`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StepReport {
    pub substeps: u32,
    /// The root jumped or snapped past the thresholds: velocities dropped.
    pub teleported: bool,
    /// The state was (re)seeded from the animation; nothing simulated.
    pub seeded: bool,
}

/// One cloth instance's simulation state.
#[derive(Clone, Debug, Default)]
pub struct ClothState {
    pos: Vec<Vec3>,
    /// Positions one substep ago: `pos - prev` is the velocity relative to the character.
    prev: Vec<Vec3>,
    /// Animated targets and capsules at the last simulated update (swept to the new ones across the substeps).
    anim_prev: Vec<Vec3>,
    caps_prev: Vec<Capsule>,
    /// The root's velocity in its own frame (m/s) and yaw rate (rad/s) over the last update.
    frame_velocity: Vec3,
    yaw_rate: f32,
    /// Unsimulated time, s (less than one substep after an update).
    carry: f32,
    init: bool,
}

impl ClothState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Re-seeds from the animated pose at the next update (LOD cut, respawn).
    pub fn reset(&mut self) {
        self.init = false;
    }

    /// The particles, character space (empty before the first update).
    pub fn positions(&self) -> &[Vec3] {
        &self.pos
    }

    /// Each particle's rotation from its rest frame to its current one
    /// (character space), from the lattice tangents down the column and
    /// along the row: the lattice joints' orientation for skinning.
    pub fn rotations(&self, lattice: &ClothLattice, out: &mut Vec<Quat>) {
        lattice_rotations(lattice, &self.pos, out);
    }

    /// Advances the cloth by `inputs.dt` in fixed substeps.
    ///
    /// # Panics
    /// If `inputs.animated` doesn't hold one position per particle.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]
    pub fn step(&mut self, lattice: &ClothLattice, params: &ClothParams, inputs: &ClothInputs<'_>) -> StepReport {
        let n = lattice.particles.len();
        assert_eq!(inputs.animated.len(), n, "cloth step: one animated position per particle");
        let dt = inputs.dt.max(0.0);
        let moved = inputs.moved;
        if !self.init || self.pos.len() != n || self.caps_prev.len() != inputs.capsules.len() {
            self.pos = inputs.animated.to_vec();
            self.prev = inputs.animated.to_vec();
            self.anim_prev = inputs.animated.to_vec();
            self.caps_prev = inputs.capsules.to_vec();
            (self.frame_velocity, self.yaw_rate) = if dt > 0.0 {
                (Quat::from_rotation_y(-moved.yaw) * moved.translation / dt, moved.yaw / dt)
            } else {
                (Vec3::ZERO, 0.0)
            };
            self.carry = 0.0;
            self.init = true;
            return StepReport { seeded: true, ..StepReport::default() };
        }
        let h = CLOTH_SUBSTEP;
        let teleported = moved.translation.length() > params.teleport_distance || moved.yaw.abs() > params.teleport_angle;
        if teleported {
            // Keep the character-space shape, drop the velocity, don't sweep
            // the capsules across the jump; the frame velocity stays the one
            // from before, so the next update's inertia compares like with like.
            self.prev.clone_from(&self.pos);
            self.anim_prev.copy_from_slice(inputs.animated);
            self.caps_prev.copy_from_slice(inputs.capsules);
        } else if dt > 0.0 {
            // Fictitious forces from the change of the frame's motion over
            // this update, as one velocity change (the same total at any
            // frame rate): -a_lin (linear) and -dω × r (Euler).
            let undo = Quat::from_rotation_y(-moved.yaw);
            let velocity = undo * moved.translation / dt;
            let yaw_rate = moved.yaw / dt;
            let dv = -(velocity - undo * self.frame_velocity) * params.inertia_linear;
            let dw = Vec3::Y * ((yaw_rate - self.yaw_rate) * params.inertia_angular);
            for (i, p) in lattice.particles.iter().enumerate() {
                if p.inv_mass > 0.0 {
                    self.prev[i] -= (dv - dw.cross(self.pos[i])) * h;
                }
            }
            self.frame_velocity = velocity;
            self.yaw_rate = yaw_rate;
        }
        self.carry += dt;
        let mut steps = (self.carry / h).floor() as u32;
        if steps > CLOTH_MAX_SUBSTEPS {
            steps = CLOTH_MAX_SUBSTEPS;
            self.carry = h * steps as f32;
        }
        self.carry -= h * steps as f32;
        if steps == 0 {
            return StepReport { teleported, ..StepReport::default() };
        }
        let keep = (-params.damping * h).exp();
        let omega = Vec3::Y * self.yaw_rate;
        let alpha = params.compliance.map(|c| c / (h * h));
        let mut caps = Vec::with_capacity(inputs.capsules.len());
        for k in 1..=steps {
            let t = k as f32 / steps as f32;
            caps.clear();
            caps.extend(self.caps_prev.iter().zip(inputs.capsules).map(|(a, b)| a.lerp(*b, t)));
            // Predict: damped velocity, gravity, wind, centrifugal and Coriolis.
            for (i, p) in lattice.particles.iter().enumerate() {
                let target = self.anim_prev[i].lerp(inputs.animated[i], t);
                let x = self.pos[i];
                if p.inv_mass <= 0.0 {
                    self.prev[i] = x;
                    self.pos[i] = target;
                    continue;
                }
                let v = (x - self.prev[i]) * keep;
                let fictitious = -(omega.cross(omega.cross(x)) + 2.0 * omega.cross(v / h)) * params.inertia_angular;
                let a = params.gravity + inputs.acceleration + fictitious;
                self.prev[i] = x;
                self.pos[i] = x + v + a * (h * h);
            }
            // Distance constraints (one XPBD pass: λ starts at 0 each substep).
            for e in &lattice.edges {
                let (a, b) = (usize::from(e.a), usize::from(e.b));
                let (wa, wb) = (lattice.particles[a].inv_mass, lattice.particles[b].inv_mass);
                let w = wa + wb;
                if w <= 0.0 {
                    continue;
                }
                let d = self.pos[b] - self.pos[a];
                let len = d.length();
                if len < 1e-9 {
                    continue;
                }
                let lambda = -(len - e.rest) / (w + alpha[e.kind.index()]);
                let n = d / len;
                self.pos[a] -= n * (wa * lambda);
                self.pos[b] += n * (wb * lambda);
            }
            // Long-range attachments (one-sided).
            for tether in &lattice.tethers {
                let (i, anchor) = (usize::from(tether.particle), self.pos[usize::from(tether.anchor)]);
                let limit = tether.length * params.tether_scale;
                let d = self.pos[i] - anchor;
                let len = d.length();
                if len > limit {
                    self.pos[i] = anchor + d * (limit / len);
                }
            }
            // Max distance to the animated position, then the body: last, so
            // no particle ends a substep inside a capsule.
            for (i, p) in lattice.particles.iter().enumerate() {
                if p.inv_mass <= 0.0 {
                    continue;
                }
                let target = self.anim_prev[i].lerp(inputs.animated[i], t);
                let d = self.pos[i] - target;
                if d.length_squared() > p.max_distance * p.max_distance {
                    self.pos[i] = target + d.normalize_or_zero() * p.max_distance;
                }
                for c in &caps {
                    let q = c.closest(self.pos[i]);
                    let r = c.radius + params.thickness;
                    let d = self.pos[i] - q;
                    let len2 = d.length_squared();
                    if len2 < r * r {
                        // On the axis: out the way the particle came from.
                        let out = d.try_normalize().or_else(|| (self.prev[i] - q).try_normalize()).unwrap_or(Vec3::Z);
                        self.pos[i] = q + out * r;
                    }
                }
            }
        }
        self.anim_prev.copy_from_slice(inputs.animated);
        self.caps_prev.copy_from_slice(inputs.capsules);
        StepReport { substeps: steps, teleported, seeded: false }
    }
}

#[cfg(test)]
#[path = "cloth_tests.rs"]
mod tests;
