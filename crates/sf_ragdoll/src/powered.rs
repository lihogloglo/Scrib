//! Powered ragdolls (P8; physics.md "Ragdolls and the animation blend",
//! animation.md layer 4).
//!
//! Drives are rapier's implicit, acceleration-based joint motors, i.e.
//! stable PD (Tan, Liu, Turk, *Stable Proportional-Derivative Controllers*,
//! 2011): stiffness = strength · region gain · ω², damping 2·√stiffness
//! (ζ = 1), independent of the bodies' masses and stable at the fixed step.
//! Targets are the animated pose's relative rotations between each joint's
//! two frames, with the pose's relative angular velocity as feed-forward
//! ([`PhysicsWorld::set_joint_target`]). A world-space pelvis drive (UE's
//! world-space physical animation) pulls the root body toward its animated
//! world pose from a kinematic anchor body through a six-axis motor joint;
//! the anchor's own velocity makes the drive's velocity target exact.
//!
//! Hits add an impulse and a strength dip on the hit body's region that
//! recovers linearly; [`balance`] classifies the center of mass against the
//! support polygon; [`get_up_root`] places the character for a get-up clip.

use std::f32::consts::PI;

use glam::{Affine3A, Quat, Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyKind, JointDesc, JointKind, JointSolver, Layer, PhysicsWorld, ShapeRef};

use crate::instance::{body_world, rigid};
use crate::{RagdollDef, RagdollInstance};

/// How a region follows the animation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Drive {
    /// As stiff as the solver allows: the full-strength gain ×
    /// [`DriveParams::kinematic_gain`] (the bodies stay dynamic, so contacts
    /// still push them; true kinematic bodies would need a body-type switch).
    Kinematic,
    /// Stable PD toward the animation; `strength` scales the stiffness
    /// (1 = full strength).
    Powered { strength: f32 },
    Limp,
}

/// Per-region drives and the world-space pelvis drive.
#[derive(Clone, Debug, PartialEq)]
pub struct StrengthProfile {
    /// Regions not listed are limp.
    pub region: Vec<(u8, Drive)>,
    /// Strength of the world-space pelvis drive (0 = off).
    pub pelvis_world: f32,
}

impl StrengthProfile {
    /// Every region on `drive`.
    pub fn uniform(def: &RagdollDef, drive: Drive, pelvis_world: f32) -> Self {
        Self { region: (0..def.regions.len()).filter_map(|r| u8::try_from(r).ok()).map(|r| (r, drive)).collect(), pelvis_world }
    }

    pub fn limp() -> Self {
        Self { region: Vec::new(), pelvis_world: 0.0 }
    }

    pub fn drive(&self, region: u8) -> Drive {
        self.region.iter().find(|(r, _)| *r == region).map_or(Drive::Limp, |&(_, d)| d)
    }

    /// Powered strengths and the pelvis drive times `k` (a fade to limp;
    /// `Kinematic` regions become `Powered { strength: k · kinematic }`).
    #[must_use]
    pub fn scaled(&self, k: f32, params: &DriveParams) -> Self {
        let k = k.max(0.0);
        let region = self
            .region
            .iter()
            .map(|&(r, d)| {
                let d = match d {
                    Drive::Kinematic => Drive::Powered { strength: k * params.kinematic_gain },
                    Drive::Powered { strength } => Drive::Powered { strength: strength * k },
                    Drive::Limp => Drive::Limp,
                };
                (r, d)
            })
            .collect();
        Self { region, pelvis_world: self.pelvis_world * k }
    }

    /// No region driven and no pelvis drive.
    pub fn is_limp(&self) -> bool {
        self.pelvis_world <= 0.0
            && self.region.iter().all(|(_, d)| match d {
                Drive::Limp => true,
                Drive::Powered { strength } => *strength <= 0.0,
                Drive::Kinematic => false,
            })
    }
}

/// A hit on a powered ragdoll: an impulse at a world point on one body and
/// a strength dip on that body's region recovering over `recover` s.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitImpulse {
    pub body: u16,
    pub point: WorldPos,
    /// N·s, world.
    pub impulse: Vec3,
    /// 0–1: the share of the region's strength lost at the hit.
    pub dip: f32,
    pub recover: f32,
}

/// A recovering strength dip.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dip {
    pub region: u8,
    pub amount: f32,
    pub recover: f32,
    pub elapsed: f32,
}

impl Dip {
    fn factor(&self) -> f32 {
        let left = if self.recover > 0.0 { (1.0 - self.elapsed / self.recover).clamp(0.0, 1.0) } else { 0.0 };
        1.0 - self.amount.clamp(0.0, 1.0) * left
    }
}

/// Drive gains.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriveParams {
    /// Joint drive angular frequency at full strength and gain 1, rad/s.
    pub omega: f32,
    /// `Kinematic` regions: stiffness × this.
    pub kinematic_gain: f32,
    /// World-space pelvis drive frequency at `pelvis_world` 1, rad/s.
    pub pelvis_omega: f32,
    /// Gravity compensation of the pelvis drive, 0–1: the share of the
    /// whole body's weight (and its torque about the pelvis) the drive
    /// carries as a feed-forward, scaled by `pelvis_world`. Without it a
    /// spring of `pelvis_omega` sags by g / ω² (6.8 cm at 12 rad/s).
    pub gravity_compensation: f32,
    /// Extra solver iterations while powered (the motors keep the chain
    /// together, so it needs fewer than a limp ragdoll's).
    pub powered_iterations: usize,
}

impl Default for DriveParams {
    fn default() -> Self {
        Self { omega: 160.0, kinematic_gain: 4.0, pelvis_omega: 12.0, gravity_compensation: 1.0, powered_iterations: 8 }
    }
}

/// Stiffness multiplier of a region by its name: tails, wings and fins
/// (light, passive appendages) follow softer.
pub fn region_gain(name: &str) -> f32 {
    if ["tail", "wing", "fin"].iter().any(|s| name.contains(s)) { 0.5 } else { 1.0 }
}

/// A critically damped drive of angular frequency `omega` on a body that
/// carries `load` times its own inertia (its subtree): rapier's
/// acceleration-based motor scales by the body's own effective mass, so both
/// gains scale by `load` (scaling only the stiffness would leave the damping
/// ratio at 1 / √load: an underdamped, ringing drive).
fn critical(omega: f32, load: f32) -> sf_physics::Drive {
    sf_physics::Drive { stiffness: omega * omega * load, damping: 2.0 * omega * load }
}

fn gains(drive: Drive, gain: f32, load: f32, dip: f32, params: &DriveParams) -> Option<sf_physics::Drive> {
    let strength = match drive {
        Drive::Limp => return None,
        Drive::Kinematic => params.kinematic_gain,
        Drive::Powered { strength } => strength,
    } * gain
        * dip;
    // Joint drives keep damping 2·√stiffness (ζ = 1/√load on the subtree):
    // the measured best for the walk (the fully critical gains over-damp
    // the stiff 160 rad/s joint motors against the feed-forward and lag
    // ~13°; tests `powered_ragdoll_tracks_a_walk`, P8b).
    let stiffness = strength * load * params.omega * params.omega;
    (strength > 0.0).then(|| sf_physics::Drive { stiffness, damping: 2.0 * stiffness.sqrt() })
}

/// The rate of a relative rotation's vector part (`w ≥ 0` hemisphere), which
/// is what rapier's angular motor velocity constrains (its Jacobian rows
/// are the derivative of that vector part).
fn imag_rate(a: Quat, b: Quat, dt: f32) -> Vec3 {
    let flip = |q: Quat| if q.w < 0.0 { -q } else { q };
    let (a, b) = (flip(a), flip(b));
    if dt > 1e-5 { (Vec3::new(b.x, b.y, b.z) - Vec3::new(a.x, a.y, a.z)) / dt } else { Vec3::ZERO }
}

/// The animated pose a powered ragdoll follows this step.
#[derive(Clone, Copy, Debug)]
pub struct DriveTarget<'a> {
    /// The character's model frame (no scale).
    pub root: WorldTransform,
    /// Model-space joint transforms (the animated pose).
    pub model: &'a [Affine3A],
    /// The pose `dt` seconds earlier (velocity feed-forward).
    pub previous: Option<(&'a [Affine3A], f32)>,
}

impl RagdollInstance {
    /// Sets every joint drive toward `target` under `profile` (call before
    /// each physics step, or each frame), the world-space pelvis drive, the
    /// bodies' layer (`POWERED` while anything is driven, else `RAGDOLL`),
    /// and wakes driven bodies.
    pub fn drive(&mut self, world: &mut PhysicsWorld, def: &RagdollDef, target: &DriveTarget<'_>, profile: &StrengthProfile, params: &DriveParams) {
        let model = target.model;
        if model.len() != def.owner.len() || self.joints.len() != def.joints.len() {
            return;
        }
        let rot = |pose: &[Affine3A], b: u16| {
            let body = &def.bodies[usize::from(b)];
            rigid(&pose[usize::from(body.joint)]).0 * body.local.1
        };
        // Composite-inertia gain scaling: an acceleration-based motor only
        // sees its two bodies, but a joint holds its whole subtree, so the
        // stiffness scales by the subtree's inertia about the joint over the
        // child body's own (subtree-scaled PD gains, Zordan & Hodgins 2002).
        let sub = Subtrees::new(def, model, self.scale, &self.inertia);
        let mut any = false;
        for ((_, id), j) in self.joints.iter().zip(&def.joints) {
            let region = def.bodies[usize::from(j.child)].region;
            let dip = self.dips.iter().filter(|d| d.region == region).map(Dip::factor).product::<f32>();
            let name = def.regions.get(usize::from(region)).map_or("", String::as_str);
            let child = usize::from(j.child);
            let pivot = rigid(&model[usize::from(def.bodies[child].joint)]);
            let pivot = (pivot.1 + pivot.0 * def.bodies[child].local.0 + rot(model, j.child) * j.frame_child.0) * self.scale;
            let load = sub.inertia_about(child, pivot) / sub.own[child].max(1e-9);
            let g = gains(profile.drive(region), region_gain(name), load, dip, params);
            any |= g.is_some();
            let frame = |pose: &[Affine3A]| (rot(pose, j.parent) * j.frame_parent.1, rot(pose, j.child) * j.frame_child.1);
            let (f1, f2) = frame(model);
            let relative = f1.inverse() * f2;
            let velocity = match target.previous {
                Some((prev, dt)) if prev.len() == model.len() && g.is_some() => {
                    let (p1, p2) = frame(prev);
                    imag_rate(p1.inverse() * p2, relative, dt)
                }
                _ => Vec3::ZERO,
            };
            world.set_joint_target(*id, relative, velocity, g);
        }
        let pelvis = profile.pelvis_world.max(0.0);
        if pelvis > 0.0 {
            any = true;
            let b0 = &def.bodies[0];
            let goal = body_world(b0.local, &model[usize::from(b0.joint)], &target.root, self.scale);
            let anchor = if let Some(a) = self.anchor {
                Some(a)
            } else {
                {
                    let a = world.insert(&BodyDesc::new(BodyKind::Kinematic, goal, ShapeRef::Sphere(0.01), Layer::empty()));
                    let desc = JointDesc {
                        parent: a,
                        child: self.bodies[0],
                        frame_parent: (Vec3::ZERO, Quat::IDENTITY),
                        frame_child: (Vec3::ZERO, Quat::IDENTITY),
                        kind: JointKind::Free,
                        contacts: false,
                    };
                    if let Some(j) = world.insert_joint(&desc, JointSolver::Impulse) {
                        Some((a, j))
                    } else {
                        world.remove(a);
                        None
                    }
                }
            };
            self.anchor = anchor;
            if let Some((a, j)) = anchor {
                world.set_kinematic_target(a, self.compensated(world, def, target, &sub, goal, params));
                let omega = params.pelvis_omega * pelvis.sqrt();
                let linear = sub.mass[0] / (def.bodies[0].mass * self.scale.powi(3)).max(1e-6);
                let angular = sub.inertia_about(0, sub.com(0)) / sub.own[0].max(1e-9);
                world.set_joint_target(j, Quat::IDENTITY, Vec3::ZERO, Some(critical(omega, angular)));
                world.set_joint_linear_drive(j, Some(critical(omega, linear)));
            }
        } else if let Some((a, _)) = self.anchor.take() {
            world.remove(a);
        }
        let iterations = if any { params.powered_iterations } else { self.iterations };
        if any != self.powered || iterations != self.applied_iterations {
            let layer = if any { Layer::POWERED } else { Layer::RAGDOLL };
            for &b in &self.bodies {
                world.set_layer(b, layer);
                world.set_iterations(b, iterations);
            }
            self.powered = any;
            self.applied_iterations = iterations;
        }
        if any {
            for &b in &self.bodies {
                world.wake(b);
            }
        }
    }

    /// The anchor pose of the world-space pelvis drive: the root body's
    /// goal moved so the drive's spring carries the whole body's weight and
    /// its torque about the pelvis as a feed-forward (gravity compensation;
    /// the force and torque scale with the drive's strength, as the spring
    /// does, so a fading drive lets the body down).
    fn compensated(&self, world: &PhysicsWorld, def: &RagdollDef, target: &DriveTarget<'_>, sub: &Subtrees, goal: WorldTransform, params: &DriveParams) -> WorldTransform {
        let comp = params.gravity_compensation.clamp(0.0, 1.0);
        let w2 = params.pelvis_omega * params.pelvis_omega;
        if comp <= 0.0 || w2 <= 0.0 {
            return goal;
        }
        let gravity = world.params().gravity;
        let b0 = &def.bodies[0];
        let (r0, t0) = rigid(&target.model[usize::from(b0.joint)]);
        let lever = target.root.rotation * (sub.com(0) - (t0 + r0 * b0.local.0) * self.scale);
        let torque = lever.cross(gravity * sub.mass[0]);
        let theta = -torque * comp / (w2 * sub.inertia_about(0, sub.com(0)).max(1e-9));
        WorldTransform { pos: goal.pos.offset(-gravity * comp / w2), rotation: (Quat::from_scaled_axis(theta) * goal.rotation).normalize(), ..goal }
    }

    /// Applies a hit and starts its strength dip. False if the body is gone.
    pub fn hit(&mut self, world: &mut PhysicsWorld, def: &RagdollDef, hit: &HitImpulse) -> bool {
        let Some(&body) = self.bodies.get(usize::from(hit.body)) else {
            return false;
        };
        if !world.apply_impulse(body, hit.impulse, Some(hit.point)) {
            return false;
        }
        if hit.dip > 0.0 && hit.recover > 0.0 {
            let region = def.bodies[usize::from(hit.body)].region;
            self.dips.push(Dip { region, amount: hit.dip.min(1.0), recover: hit.recover, elapsed: 0.0 });
        }
        true
    }

    /// Advances the strength dips' recovery.
    pub fn advance(&mut self, dt: f32) {
        for d in &mut self.dips {
            d.elapsed += dt;
        }
        self.dips.retain(|d| d.elapsed < d.recover);
    }

    /// The body nearest a world point (a hit's target).
    pub fn nearest_body(&self, world: &PhysicsWorld, point: WorldPos) -> Option<u16> {
        self.bodies
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| Some((i, world.pose(b)?.pos.delta_f32(point).length_squared())))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .and_then(|(i, _)| u16::try_from(i).ok())
    }

    /// The root body's speed, m/s.
    pub fn pelvis_speed(&self, world: &PhysicsWorld) -> f32 {
        self.bodies.first().and_then(|&b| world.velocity(b)).map_or(0.0, |(v, _)| v.length())
    }

    /// The character model frame the root body implies: where the model
    /// frame would be if the root body sat in it as in `reference` (a model
    /// pose, e.g. the rest pose). Its forward (+Z) tells face up from face
    /// down, its position follows the pelvis (culling, cell moves).
    pub fn implied_frame(&self, world: &PhysicsWorld, def: &RagdollDef, reference: &[Affine3A], alpha: f32) -> Option<WorldTransform> {
        let p = world.pose_at(*self.bodies.first()?, alpha)?;
        let b0 = &def.bodies[0];
        let (r, t) = rigid(reference.get(usize::from(b0.joint))?);
        let rotation = (p.rotation * (r * b0.local.1).inverse()).normalize();
        let offset = rotation * ((t + r * b0.local.0) * self.scale);
        Some(WorldTransform { pos: p.pos.offset(-offset), rotation, scale: Vec3::ONE })
    }

    /// Center of mass (world) and total mass.
    pub fn center_of_mass(&self, world: &PhysicsWorld, def: &RagdollDef) -> Option<(WorldPos, f32)> {
        let origin = world.pose(*self.bodies.first()?)?.pos;
        let s3 = self.scale.powi(3);
        let (mut sum, mut mass) = (Vec3::ZERO, 0.0);
        for (b, &id) in def.bodies.iter().zip(&self.bodies) {
            let p = world.pose(id)?;
            let m = b.mass * s3;
            sum += origin.delta_f32(p.pos) * m;
            mass += m;
        }
        (mass > 0.0).then(|| (origin.offset(sum / mass), mass))
    }
}

/// Per body: its subtree's mass, first and second moments of mass and summed
/// own inertias (model space, scaled), and its own scalar inertia about its
/// center (a solid cylinder of the capsule's length, about a cross axis).
struct Subtrees {
    mass: Vec<f32>,
    moment: Vec<Vec3>,
    second: Vec<f32>,
    own: Vec<f32>,
    sum_own: Vec<f32>,
}

impl Subtrees {
    #[allow(clippy::many_single_char_names)]
    fn new(def: &RagdollDef, model: &[Affine3A], scale: f32, floor: &[f32]) -> Self {
        let s3 = scale.powi(3);
        let n = def.bodies.len();
        let mut out = Self { mass: vec![0.0; n], moment: vec![Vec3::ZERO; n], second: vec![0.0; n], own: vec![0.0; n], sum_own: vec![0.0; n] };
        for (i, b) in def.bodies.iter().enumerate() {
            let m = b.mass * s3;
            let (r, t) = rigid(&model[usize::from(b.joint)]);
            let p = (t + r * b.local.0) * scale;
            let (radius, length) = (b.shape.radius * scale, 2.0 * (b.shape.half + b.shape.radius) * scale);
            let inertia = (m * (3.0 * radius * radius + length * length) / 12.0).max(floor.get(i).copied().unwrap_or(0.0));
            out.mass[i] = m;
            out.moment[i] = p * m;
            out.second[i] = m * p.length_squared();
            out.own[i] = inertia;
            out.sum_own[i] = inertia;
        }
        for i in (1..n).rev() {
            if let Some(p) = def.bodies[i].parent.map(usize::from) {
                out.mass[p] += out.mass[i];
                let mi = out.moment[i];
                out.moment[p] += mi;
                out.second[p] += out.second[i];
                out.sum_own[p] += out.sum_own[i];
            }
        }
        out
    }

    fn com(&self, i: usize) -> Vec3 {
        self.moment[i] / self.mass[i].max(1e-9)
    }

    /// The subtree's inertia about a point (parallel axis, scalar).
    fn inertia_about(&self, i: usize, q: Vec3) -> f32 {
        self.sum_own[i] + self.second[i] - 2.0 * q.dot(self.moment[i]) + self.mass[i] * q.length_squared()
    }
}
/// Balance against the support polygon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Balance {
    Stable,
    /// Outside by more than the stumble margin: a stumble step can recover.
    Stumble,
    /// Outside by more than the fall margin: knockdown.
    Fall,
}

/// The center of mass against the horizontal convex hull of the support
/// points (feet in contact), physics.md "Balance".
pub fn balance(com: Vec3, support: &[Vec3], stumble_margin: f32, fall_margin: f32) -> Balance {
    if support.is_empty() {
        return Balance::Fall;
    }
    let c = Vec2::new(com.x, com.z);
    let mut pts: Vec<Vec2> = support.iter().map(|p| Vec2::new(p.x, p.z)).collect();
    pts.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    pts.dedup_by(|a, b| a.distance_squared(*b) < 1e-8);
    let hull = convex_hull(&pts);
    let outside = outside_distance(c, &hull);
    if outside > fall_margin {
        Balance::Fall
    } else if outside > stumble_margin {
        Balance::Stumble
    } else {
        Balance::Stable
    }
}

/// Andrew's monotone chain; `pts` sorted and deduplicated.
fn convex_hull(pts: &[Vec2]) -> Vec<Vec2> {
    if pts.len() < 3 {
        return pts.to_vec();
    }
    let cross = |o: Vec2, a: Vec2, b: Vec2| (a - o).perp_dot(b - o);
    let mut hull: Vec<Vec2> = Vec::with_capacity(pts.len() * 2);
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &Vec2>> = if pass == 0 { Box::new(pts.iter()) } else { Box::new(pts.iter().rev()) };
        for &p in iter {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
                hull.pop();
            }
            hull.push(p);
        }
        hull.pop();
    }
    hull
}

/// Distance from `c` to a convex polygon (counter-clockwise), 0 inside.
fn outside_distance(c: Vec2, hull: &[Vec2]) -> f32 {
    let seg = |a: Vec2, b: Vec2| {
        let ab = b - a;
        let t = if ab.length_squared() > 0.0 { ((c - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
        c.distance(a + ab * t)
    };
    match hull.len() {
        0 => f32::INFINITY,
        1 => c.distance(hull[0]),
        2 => seg(hull[0], hull[1]),
        n => {
            let inside = (0..n).all(|i| (hull[(i + 1) % n] - hull[i]).perp_dot(c - hull[i]) >= 0.0);
            if inside { 0.0 } else { (0..n).map(|i| seg(hull[i], hull[(i + 1) % n])).fold(f32::INFINITY, f32::min) }
        }
    }
}

/// Where to put the character's model frame for a get-up: yaw only, so the
/// get-up clip's first-frame root body (`clip_body`: rotation and position
/// in model space at scale 1) lands on the ragdoll's root body (`body`,
/// world) as closely as a yaw and a translation allow. The height follows
/// the body (the host may snap it to the ground).
///
/// `roll` is the **`ROLL_TO_BACK` scaffold**: with only a face-up get-up
/// clip, a body lying face down is first turned 180° about its spine
/// (`spine`: the root body's long axis in its own frame, toward the head),
/// so the inertialized hand-back rolls it over while it starts to rise.
/// (AN-10: the roll used the frame's up axis, vertical, so face-down bodies
/// got a 180° heading flip and spun through the hand-back.)
/// Removal: face-down get-up clips from the adapter.
pub fn get_up_root(body: &WorldTransform, clip_body: (Quat, Vec3), spine: Vec3, scale: f32, roll: bool) -> WorldTransform {
    let mut rot = body.rotation;
    if roll {
        rot = (rot * Quat::from_axis_angle(spine.try_normalize().unwrap_or(Vec3::Y), PI)).normalize();
    }
    let yaw = yaw_of(rot * clip_body.0.inverse());
    WorldTransform { pos: body.pos.offset(-(yaw * clip_body.1 * scale)), rotation: yaw, scale: Vec3::ONE }
}

/// The rotation's heading about +Y: its twist about Y, or (rotations near
/// 180° about a horizontal axis) the heading of its projected forward.
pub fn yaw_of(q: Quat) -> Quat {
    let twist = Quat::from_xyzw(0.0, q.y, 0.0, q.w);
    if twist.length_squared() > 1e-4 {
        return twist.normalize();
    }
    let f = q * Vec3::Z;
    let f = Vec3::new(f.x, 0.0, f.z).try_normalize().unwrap_or(Vec3::Z);
    Quat::from_rotation_y(f.x.atan2(f.z))
}

#[cfg(test)]
mod tests;
