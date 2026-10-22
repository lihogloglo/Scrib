//! Body handles, descriptions, events and world parameters.

use glam::Vec3;
use sf_coords::{WorldPos, WorldTransform};

use crate::layer::Layer;
use crate::shape::ShapeRef;

/// A body in a [`crate::PhysicsWorld`]: slot index + generation, never an
/// engine handle. A removed body's id stays dead even when its slot is reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BodyId {
    pub(crate) index: u32,
    pub(crate) generation: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyKind {
    /// Never moves (statics, terrain).
    Fixed,
    /// Simulated. `asleep` inserts it sleeping: it stays exactly where it was
    /// placed until something touches it (placed items).
    Dynamic { asleep: bool },
    /// Moved by its owner through [`crate::PhysicsWorld::set_kinematic_target`]
    /// (character capsules, the carrying hand); pushes dynamic bodies.
    Kinematic,
    /// Fixed until released: a placed item nothing supports (a lantern on a
    /// hook). [`crate::PhysicsWorld::release`] or an impulse makes it an awake
    /// dynamic body with its mass, damping and CCD; contacts never do.
    Held,
}

/// Linear and angular damping of placed items. The angular term stands in
/// for rolling resistance, which rapier doesn't model: without it a knocked
/// bottle rolls across the floor for many seconds (physics.md "Items": rest
/// within 3 s).
pub const ITEM_DAMPING: (f32, f32) = (0.1, 1.5);

/// Everything needed to insert a body.
#[derive(Clone)]
pub struct BodyDesc {
    pub kind: BodyKind,
    /// Position, rotation and scale. Scale shapes the collider (mesh shapes
    /// get a cached scaled copy); it isn't simulated.
    pub pose: WorldTransform,
    pub shape: ShapeRef,
    pub layer: Layer,
    /// Simulated mass in kg, clamped to [`PhysicsParams::mass_range`]; `None`
    /// is the shape's volume × [`PhysicsParams::density`]. Gameplay weight
    /// lives in records, not here.
    pub mass: Option<f32>,
    /// Continuous collision detection (thrown and fast bodies).
    pub ccd: bool,
    /// Linear and angular damping.
    pub damping: (f32, f32),
    /// Extra solver iterations for this body's island (ragdolls, held items).
    pub extra_iterations: usize,
    /// Floor on each principal moment of inertia, kg·m² (0: the shape's own). Inertia
    /// conditioning for jointed chains: a small hub body (a pelvis) between heavy neighbours
    /// makes the joint rows ill-conditioned, so limits give way and the hub spins.
    pub min_inertia: f32,
    /// Owner data (a placement id, an entity): returned by [`crate::PhysicsWorld::user`].
    pub user: u64,
}

impl BodyDesc {
    pub fn new(kind: BodyKind, pose: WorldTransform, shape: ShapeRef, layer: Layer) -> Self {
        Self { kind, pose, shape, layer, mass: None, ccd: false, damping: (0.0, 0.05), extra_iterations: 0, user: 0, min_inertia: 0.0 }
    }

    /// A fixed body (static placement, terrain chunk).
    pub fn fixed(pose: WorldTransform, shape: ShapeRef, layer: Layer) -> Self {
        Self::new(BodyKind::Fixed, pose, shape, layer)
    }

    /// A placed item: dynamic, inserted asleep, with [`ITEM_DAMPING`].
    pub fn item(pose: WorldTransform, shape: ShapeRef, layer: Layer, mass: Option<f32>) -> Self {
        Self { mass, damping: ITEM_DAMPING, ..Self::new(BodyKind::Dynamic { asleep: true }, pose, shape, layer) }
    }

    /// A placed item held where it was placed ([`BodyKind::Held`]) until
    /// released: with [`ITEM_DAMPING`] and CCD once it moves.
    pub fn held(pose: WorldTransform, shape: ShapeRef, layer: Layer, mass: Option<f32>) -> Self {
        Self { mass, damping: ITEM_DAMPING, ccd: true, ..Self::new(BodyKind::Held, pose, shape, layer) }
    }

    #[must_use]
    pub fn with_user(mut self, user: u64) -> Self {
        self.user = user;
        self
    }
}

/// What happened during a step, drained with [`crate::PhysicsWorld::drain_events`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhysicsEvent {
    /// A dynamic body started moving.
    Woke(BodyId),
    /// A dynamic body came to rest; its pose is now its rest pose.
    Slept(BodyId),
    /// Contact impulse (N·s) above the threshold between two bodies.
    /// Impact sounds, damage, knockback.
    /// `normal`: the contact normal from `a` to `b` (the impulse on `b`'s
    /// direction; zero if unknown).
    ContactForce { a: BodyId, b: BodyId, impulse: f32, point: WorldPos, normal: Vec3 },
    /// A dynamic body `by` started touching `target`, a body on
    /// [`PhysicsParams::hit_layers`] (a character's capsule): hit reactions
    /// and knockdowns. `impulse` (N·s) is the impact momentum
    /// ([`crate::impact_impulse`]): `by`'s mass times its own approach speed
    /// along the contact normal before the solver ran, so a character walking
    /// into a resting body, or pressing one into the ground, is no hit.
    /// `direction`: the contact normal toward `target` (unit, world).
    Hit { target: BodyId, by: BodyId, impulse: f32, point: WorldPos, direction: Vec3 },
    /// A dynamic body fell below [`PhysicsParams::kill_height`]; it is back
    /// at its last rest pose, asleep.
    FellOut(BodyId),
}

/// World parameters. Defaults are the physics.md values.
#[derive(Clone, Debug)]
pub struct PhysicsParams {
    /// Fixed step, seconds.
    pub dt: f32,
    pub gravity: Vec3,
    /// The anchor moves (by whole grid cells) when the focus is farther than this, meters.
    pub rebase_distance: f32,
    /// Simulated mass range, kg (stacked mass ratios stay solvable).
    pub mass_range: (f32, f32),
    /// Density for bodies without a mass, kg/m³.
    pub density: f32,
    /// Depenetration speed cap, m/s: authored overlaps resolve gently.
    pub max_corrective_velocity: f32,
    /// Dynamic shapes thinner than `thin_extent` (meters, on any axis) get a
    /// `thin_skin` contact skin.
    pub thin_extent: f32,
    pub thin_skin: f32,
    /// Layers that report [`PhysicsEvent::ContactForce`], and the threshold in newtons.
    pub contact_force_layers: Layer,
    pub contact_force_threshold: f32,
    /// Layers that report [`PhysicsEvent::Hit`] (first contact with a dynamic body).
    pub hit_layers: Layer,
    /// Absolute Y (meters) under which a dynamic body counts as fallen out;
    /// `None`: `kill_margin` below the lowest fixed body ([`crate::PhysicsWorld::kill_height`]).
    pub kill_height: Option<f64>,
    /// Meters below the lowest fixed body for the automatic kill height.
    pub kill_margin: f32,
    /// Rolling resistance, rad/s²: awake `DEBRIS` and `PROP` bodies touching
    /// something lose this much angular speed per second (rapier has no
    /// rolling friction; physically `μr·g·r / (I/m)`, about 20 for a 6 cm ball
    /// at μr 0.05). Knocked bottles and fruit stop rolling within seconds.
    pub rolling_resistance: f32,
}

impl Default for PhysicsParams {
    fn default() -> Self {
        Self {
            dt: 1.0 / 60.0,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            rebase_distance: 512.0,
            mass_range: (0.2, 60.0),
            density: 1000.0,
            max_corrective_velocity: 0.5,
            thin_extent: 0.02,
            thin_skin: 0.004,
            contact_force_layers: Layer::DEBRIS | Layer::PROP | Layer::RAGDOLL | Layer::CHARACTER | Layer::PROJECTILE,
            contact_force_threshold: 200.0,
            hit_layers: Layer::CHARACTER,
            kill_height: None,
            kill_margin: 10.0,
            rolling_resistance: 20.0,
        }
    }
}
