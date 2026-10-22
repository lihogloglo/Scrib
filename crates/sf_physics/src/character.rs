//! Characters in the physics world (physics.md "Controller", P5).
//!
//! The kinematic mover (`sf_controller`) keeps moving characters at frame
//! rate; physics only sees them through two things here:
//!
//! - a **kinematic capsule** per character (`CHARACTER`), whose target before
//!   each fixed step is the mover's latest feet position, so falling and
//!   thrown bodies hit characters (`Hit` events on the capsule at first
//!   contact, with the impact momentum from [`impact_impulse`]) and
//!   are deflected by them. It is inset from the mover's capsule so a prop the
//!   mover rests against isn't shoved by an infinitely heavy kinematic body;
//! - **capped pushes**: after each mover step, every `DEBRIS`/`PROP` body the
//!   capsule touches (inflated by `reach`) gets the impulse that brings it to
//!   the character's speed along the contact normal, capped at
//!   `push_force · dt` (UE CMC's push force; Jolt `CharacterVirtual`'s max
//!   strength). A light cup takes the character's speed; a 60 kg crate blocks
//!   the mover (its sweeps include `PROP`) and yields to sustained pushing.

use glam::Vec3;
use parry3d::shape::Capsule;
use rapier3d::math::Pose;
use rapier3d::pipeline::QueryFilter;
use sf_coords::{WorldPos, WorldTransform};

use crate::body::{BodyDesc, BodyId, BodyKind};
use crate::layer::Layer;
use crate::shape::ShapeRef;
use crate::world::PhysicsWorld;

/// How a character pushes dynamic bodies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PushParams {
    /// Largest force a character exerts on a body it walks into, newtons
    /// (physics.md default 400 N).
    pub push_force: f32,
    /// Bodies within this distance of the mover's capsule are touched: more
    /// than the mover's wall skin, so a prop the mover stopped against counts.
    pub reach: f32,
    /// The kinematic capsule is this much thinner than the mover's.
    pub capsule_inset: f32,
}

impl Default for PushParams {
    fn default() -> Self {
        Self { push_force: 400.0, reach: 0.04, capsule_inset: 0.03 }
    }
}

/// A character's capsule: feet position, radius and height (the mover's shape).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacterShape {
    pub radius: f32,
    pub height: f32,
}

impl CharacterShape {
    fn half(self, radius: f32) -> f32 {
        (self.height * 0.5 - radius).max(0.0)
    }

    fn center(self, feet: WorldPos) -> WorldPos {
        feet.offset(Vec3::Y * (self.height * 0.5))
    }
}

impl BodyDesc {
    /// A character's kinematic capsule standing at `feet`; a body wider than it is high
    /// (radius over half the height: a crab) gets an upright cylinder of its full
    /// footprint instead, as a capsule cannot reach below its own height.
    pub fn character(feet: WorldPos, shape: CharacterShape, push: &PushParams) -> Self {
        let r = (shape.radius - push.capsule_inset).max(0.02);
        let pose = WorldTransform::from_pos(shape.center(feet));
        let flat = shape.radius > 0.5 * shape.height;
        let shape_ref = if flat { ShapeRef::Cylinder { r, half: 0.5 * shape.height } } else { ShapeRef::Capsule { r, half: shape.half(r) } };
        Self::new(BodyKind::Kinematic, pose, shape_ref, Layer::CHARACTER)
    }
}

/// Impact momentum (N·s) of a body hitting a character, at first contact
/// ([`crate::PhysicsEvent::Hit`]): the **incoming body's momentum along the
/// contact normal**, `m · v_n`, from its velocity before the solver ran.
///
/// This is how character setups judge being hit: hit-reaction systems
/// (Euphoria-style) are driven by the incoming body's momentum, and `PhysX`
/// reports contact begin (`eNOTIFY_TOUCH_FOUND`) with pre-solver velocities
/// (`ePRE_SOLVER_VELOCITY`) for exactly this, rather than the solver's
/// contact force. A kinematic capsule has infinite
/// mass, so the solver's impulse on a body it walks into or pins against
/// the ground is whatever stops the overlap, unbounded and spread over
/// steps, and says nothing about being hit. Two filters follow from it:
/// - **per contact start**: only the step a pair begins touching counts, so
///   sustained or wedged contact never adds up;
/// - **the other body's own approach speed**, capped by the closing speed:
///   `min(v_other · n, (v_other − v_target) · n)`, so walking into a resting
///   body is no hit, a ball chasing a retreating character hits softer, and
///   running into a ball doesn't hit harder than the ball was thrown.
///
/// `toward`: the unit contact normal pointing from the other body into the target.
#[must_use]
pub fn impact_impulse(mass: f32, other_velocity: Vec3, target_velocity: Vec3, toward: Vec3) -> f32 {
    let own = other_velocity.dot(toward);
    let closing = (other_velocity - target_velocity).dot(toward);
    mass.max(0.0) * own.min(closing).max(0.0)
}

/// A body moving toward a character faster than this (m/s) is incoming, a
/// hit, and not pushed ([`PhysicsWorld::push`]); slower ones (a cup rolling
/// into the walker's path) are pushed as usual.
const INCOMING: f32 = 0.25;

/// What one push pass did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pushed {
    /// Bodies that got an impulse.
    pub bodies: u32,
    /// Largest impulse applied, N·s.
    pub max_impulse: f32,
}

impl PhysicsWorld {
    /// Moves a character's capsule (from [`BodyDesc::character`]) to `feet`
    /// by the end of the next step.
    pub fn move_character(&mut self, id: BodyId, feet: WorldPos, shape: CharacterShape) -> bool {
        self.set_kinematic_target(id, WorldTransform::from_pos(shape.center(feet)))
    }

    /// Pushes the `DEBRIS` and `PROP` bodies the character's capsule at `feet`
    /// touches on its leading side while it tries to move with `velocity` (m/s, the intended
    /// velocity: the mover removes the part into whatever blocked it; only the
    /// horizontal part pushes): each gets `min(m · Δv_n, push_force · dt)` along the
    /// horizontal normal from the capsule's axis to its center of mass, where
    /// `Δv_n` is how much faster the character moves along it than the body.
    /// Held bodies stay held (a brushed lantern never drops).
    pub fn push(&mut self, feet: WorldPos, shape: CharacterShape, velocity: Vec3, params: &PushParams, dt: f32) -> Pushed {
        let flat = Vec3::new(velocity.x, 0.0, velocity.z);
        let speed = flat.length();
        let mut out = Pushed::default();
        if speed < 1e-3 || dt <= 0.0 {
            return out;
        }
        let dir = flat / speed;
        let center = self.to_local(shape.center(feet));
        let r = shape.radius + params.reach;
        let capsule = Capsule::new_y(shape.half(shape.radius), r);
        let filter = QueryFilter { groups: Some((Layer::DEBRIS | Layer::PROP).query_groups()), ..QueryFilter::default() };
        let touched: Vec<_> = self
            .engine
            .intersect_shape(Pose::from_translation(center), &capsule, filter)
            .filter_map(|(_, c)| c.parent())
            .collect();
        let cap = params.push_force * dt;
        for handle in touched {
            let Some(rb) = self.engine.bodies.get(handle) else { continue };
            if !rb.is_dynamic() {
                continue;
            }
            let com = rb.center_of_mass();
            let away = Vec3::new(com.x - center.x, 0.0, com.z - center.z);
            let n = away.try_normalize().unwrap_or(dir);
            // Only bodies the character moves toward: one behind or beside it isn't
            // pushed, and one coming at it is never braked (PH-10: a ball thrown at a
            // walker's back lost 2 m/s to the "push" before it hit).
            // Nor is a body flying at the character (faster than `INCOMING`): that's a
            // hit for the solver contact and `PhysicsEvent::Hit`, which the push zone
            // (wider than the capsule by `reach` + `capsule_inset`) would otherwise
            // pre-empt by batting it away first.
            let along = dir.dot(n);
            let closing = rb.linvel().dot(n);
            let dv = speed * along - closing;
            if along <= 0.0 || closing < -INCOMING || dv <= 0.0 {
                continue;
            }
            let impulse = (rb.mass() * dv).min(cap);
            let Some(id) = u32::try_from(rb.user_data).ok().and_then(|i| self.id_at(i)) else { continue };
            self.apply_impulse(id, n * impulse, None);
            out.bodies += 1;
            out.max_impulse = out.max_impulse.max(impulse);
        }
        out
    }
}
