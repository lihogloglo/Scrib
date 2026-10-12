//! Carry, drag and throw (physics.md "Carry and throw"), Bevy-free.
//!
//! A [`Carrier`] is one holder's grab state. The host feeds it the holder's
//! view (eye pose, looking along −Z) every frame with [`Carrier::record_view`],
//! calls [`Carrier::before_step`] / [`Carrier::after_step`] around each fixed
//! step, and renders a carried item at [`Carrier::render_pose`].
//!
//! - **Carry** (`mass ≤ carry_mass`): a kinematic hand body at the hold point
//!   and a 6-DOF joint from the hand to the item's center of mass, motors on
//!   every axis (acceleration-based, critically damped, `settle_time`),
//!   linear force capped at `carry_force` (N) and torque at
//!   `carry_force · torque_arm`. Over the cap the item lags; farther than
//!   `break_distance` for `break_time` it drops.
//! - **Drag** (`carry_mass < mass ≤ drag_mass`): the same hand, but only the
//!   linear motors (a ball joint at the grab point), capped at what slides
//!   `drag_mass` over a floor of friction `drag_friction`: the object rests its
//!   weight on the floor and trails. The hand keeps the grab point's
//!   horizontal distance and height below the eye from the moment of the
//!   grab and turns with the view's yaw only.
//! - **Too heavy** (`mass > drag_mass`): [`GrabResult::TooHeavy`].
//!
//! **Holder-space interpolation.** The view moves at frame rate and the item
//! at the fixed step. Each step's hand target is the view *at the step's
//! time* (extrapolated from the last two frames' views, so several steps in
//! one frame don't move the hand in jumps), and the item's pose is recorded
//! relative to that hand. A carried item renders at
//! `hand(view_now) · lerp(rel_prev, rel_curr, alpha)`: no beat between the
//! two clocks (the Godotwind carry vibration, ARCHITECTURE §7.3).

use glam::{Quat, Vec3};
use rapier3d::dynamics::{GenericJointBuilder, ImpulseJointHandle, JointAxesMask, JointAxis, MotorModel};
use rapier3d::math::Pose;
use sf_controller::ProbeQuery;
use sf_coords::{WorldPos, WorldTransform};

use crate::body::{BodyDesc, BodyId, BodyKind};
use crate::layer::Layer;
use crate::shape::ShapeRef;
use crate::world::PhysicsWorld;

/// What a holder can lift, drag and throw (from the adapter: physics.md
/// "Holder limits"). The framework never names the stat behind it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HolderLimits {
    /// Grab force cap, N (linear motors; torque cap is this × `torque_arm`).
    pub carry_force: f32,
    /// Heaviest simulated mass carried, kg.
    pub carry_mass: f32,
    /// Heaviest simulated mass dragged, kg.
    pub drag_mass: f32,
    /// Throw impulse along the view, N·s.
    pub throw_impulse: f32,
}

impl Default for HolderLimits {
    /// An average holder.
    fn default() -> Self {
        Self { carry_force: 240.0, carry_mass: 15.0, drag_mass: 120.0, throw_impulse: 6.0 }
    }
}

impl HolderLimits {
    /// Carry, drag or too heavy for a body of `mass` kg.
    pub fn mode_for(&self, mass: f32) -> Option<GrabMode> {
        if mass <= self.carry_mass {
            Some(GrabMode::Carry)
        } else if mass <= self.drag_mass {
            Some(GrabMode::Drag)
        } else {
            None
        }
    }
}

/// Tuning shared by every holder (physics.md defaults).
#[derive(Clone, Copy, Debug)]
pub struct CarryParams {
    /// Pick ray length from the eye, m.
    pub reach: f32,
    /// Eye to the carried item's surface, m (the item's bounding radius is added).
    pub hold_distance: f32,
    /// Motor settle time (critically damped), s.
    pub settle_time: f32,
    /// Lag beyond which, held for `break_time` seconds, the grab drops, m.
    pub break_distance: f32,
    pub break_time: f32,
    /// Throw speed cap, m/s.
    pub max_throw_speed: f32,
    /// Floor friction a drag must overcome at `drag_mass` (sets the drag force cap).
    pub drag_friction: f32,
    /// Torque cap lever: `carry_force · torque_arm` N·m.
    pub torque_arm: f32,
    /// Extra solver iterations for the grabbed body's island.
    pub extra_iterations: usize,
    /// Layers that can be grabbed (limp ragdolls join with P7).
    pub pick_layers: Layer,
    /// Layers that block the pick ray without being grabbable.
    pub block_layers: Layer,
    /// Views older than this aren't extrapolated further, s.
    pub max_extrapolation: f64,
}

impl Default for CarryParams {
    fn default() -> Self {
        Self {
            reach: 2.5,
            hold_distance: 0.6,
            settle_time: 0.15,
            break_distance: 1.0,
            break_time: 0.3,
            max_throw_speed: 15.0,
            drag_friction: 0.6,
            torque_arm: 0.3,
            extra_iterations: 4,
            pick_layers: Layer::DEBRIS | Layer::PROP,
            block_layers: Layer::STATIC | Layer::TERRAIN,
            max_extrapolation: 0.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrabMode {
    Carry,
    Drag,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GrabResult {
    Grabbed { body: BodyId, mode: GrabMode, mass: f32 },
    /// Heavier than `drag_mass`: feedback for the game, nothing moves.
    TooHeavy { body: BodyId, mass: f32 },
    /// Nothing grabbable under the crosshair within reach.
    Nothing,
    /// Already holding something.
    Busy,
}

/// Why a grab ended on its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarryEnd {
    /// Lagged past `break_distance` for `break_time`.
    Broke(BodyId),
    /// The body left the world (unloaded, frozen, despawned).
    Lost(BodyId),
}

#[derive(Clone, Copy, Debug)]
struct Rel {
    pos: Vec3,
    rot: Quat,
}

struct Grab {
    body: BodyId,
    hand: BodyId,
    joint: ImpulseJointHandle,
    mode: GrabMode,
    mass: f32,
    layer: Layer,
    /// Grab point in the body's frame (center of mass when carried).
    anchor: Vec3,
    /// Carry: the item's rotation in the hand frame; distance eye → hand.
    rot: Quat,
    distance: f32,
    /// Drag: the hand's horizontal distance and height below the eye.
    drag: (f32, f32),
    /// Hand targets of the last two steps (velocity for throws).
    target: Option<WorldTransform>,
    target_prev: Option<WorldTransform>,
    /// Item relative to the hand target after the last two steps.
    rel: [Option<Rel>; 2],
    stretched: f32,
}

/// One holder's grab state (see the module docs).
pub struct Carrier {
    pub limits: HolderLimits,
    pub params: CarryParams,
    /// The holder's own body (character capsule): no contacts with what it holds.
    pub holder_body: Option<BodyId>,
    views: [Option<(f64, WorldTransform)>; 2],
    grab: Option<Grab>,
}

fn forward(rot: Quat) -> Vec3 {
    rot * Vec3::NEG_Z
}

fn yaw_of(rot: Quat) -> Quat {
    let f = forward(rot);
    let h = Vec3::new(f.x, 0.0, f.z);
    if h.length_squared() < 1e-8 {
        // Looking straight up or down: the view's up vector points along the facing.
        let u = rot * Vec3::Y;
        let u = Vec3::new(u.x, 0.0, u.z).normalize_or(Vec3::NEG_Z) * f.y.signum();
        return Quat::from_rotation_arc(Vec3::NEG_Z, u);
    }
    Quat::from_rotation_arc(Vec3::NEG_Z, h.normalize())
}

impl Carrier {
    pub fn new(limits: HolderLimits) -> Self {
        Self { limits, params: CarryParams::default(), holder_body: None, views: [None, None], grab: None }
    }

    /// The holder's eye pose at `time` (seconds, the same clock as the
    /// fixed step's time). Call once per frame after the view moved.
    pub fn record_view(&mut self, time: f64, view: WorldTransform) {
        if let Some((t, _)) = self.views[1]
            && time <= t
        {
            self.views[1] = Some((time, view));
            return;
        }
        self.views = [self.views[1], Some((time, view))];
    }

    /// The view at `time`: interpolated between, or extrapolated from, the last
    /// two recorded views (at most `max_extrapolation` past the newest).
    pub fn view_at(&self, time: f64) -> Option<WorldTransform> {
        let (t1, v1) = self.views[1]?;
        let Some((t0, v0)) = self.views[0].filter(|(t0, _)| t1 - t0 > 1e-6) else { return Some(v1) };
        let time = time.min(t1 + self.params.max_extrapolation);
        #[allow(clippy::cast_possible_truncation)]
        let s = ((time - t0) / (t1 - t0)) as f32;
        let d = v0.pos.delta(v1.pos);
        #[allow(clippy::cast_possible_truncation)]
        let pos = v0.pos.offset(d.as_vec3() * s);
        let (axis, angle) = (v1.rotation * v0.rotation.inverse()).to_axis_angle();
        let rotation = (Quat::from_axis_angle(axis, angle * s) * v0.rotation).normalize();
        Some(WorldTransform { pos, rotation, scale: Vec3::ONE })
    }

    pub fn held(&self) -> Option<BodyId> {
        self.grab.as_ref().map(|g| g.body)
    }

    pub fn mode(&self) -> Option<GrabMode> {
        self.grab.as_ref().map(|g| g.mode)
    }

    /// The holder's speed factor: `1 − mass / (2 · drag_mass)` while dragging, else 1.
    pub fn speed_scale(&self) -> f32 {
        match &self.grab {
            Some(g) if g.mode == GrabMode::Drag => (1.0 - g.mass / (2.0 * self.limits.drag_mass)).clamp(0.0, 1.0),
            _ => 1.0,
        }
    }

    /// The hand's pose for a view.
    fn hand_for(g: &Grab, view: &WorldTransform) -> WorldTransform {
        match g.mode {
            GrabMode::Carry => {
                WorldTransform { pos: view.pos.offset(forward(view.rotation) * g.distance), rotation: view.rotation, scale: Vec3::ONE }
            }
            GrabMode::Drag => {
                let yaw = yaw_of(view.rotation);
                let pos = view.pos.offset(forward(yaw) * g.drag.0 + Vec3::Y * g.drag.1);
                WorldTransform { pos, rotation: yaw, scale: Vec3::ONE }
            }
        }
    }

    /// The first body under the crosshair within reach, if grabbable: (body, hit point, distance).
    pub fn pick(&self, world: &PhysicsWorld) -> Option<(BodyId, WorldPos, f32)> {
        let (_, view) = self.views[1]?;
        let mut probe = world.probe(self.params.pick_layers | self.params.block_layers);
        if let Some(h) = self.holder_body {
            probe = probe.excluding(h);
        }
        let (body, hit) = probe.cast(&ProbeQuery::ray(view.pos, forward(view.rotation), self.params.reach))?;
        let layer = world.layer(body)?;
        let grabbable = layer.intersects(self.params.pick_layers)
            && matches!(world.kind(body)?, BodyKind::Dynamic { .. } | BodyKind::Held);
        grabbable.then_some((body, hit.point, hit.distance))
    }

    /// Grabs what is under the crosshair (the latest recorded view).
    pub fn grab(&mut self, world: &mut PhysicsWorld) -> GrabResult {
        if self.grab.is_some() {
            return GrabResult::Busy;
        }
        let Some((body, point, _)) = self.pick(world) else { return GrabResult::Nothing };
        self.grab_body(world, body, point)
    }

    /// Grabs `body` at the world point `point` (a pick hit).
    pub fn grab_body(&mut self, world: &mut PhysicsWorld, body: BodyId, point: WorldPos) -> GrabResult {
        if self.grab.is_some() {
            return GrabResult::Busy;
        }
        let (Some((_, view)), Some(mass), Some(layer)) = (self.views[1], world.mass(body), world.layer(body)) else {
            return GrabResult::Nothing;
        };
        let Some(mode) = self.limits.mode_for(mass) else { return GrabResult::TooHeavy { body, mass } };
        world.release(body);
        world.wake(body);
        let Some(pose) = world.pose(body) else { return GrabResult::Nothing };
        let Some((com, radius)) = world.mass_center(body) else { return GrabResult::Nothing };
        let p = self.params;
        let eye_to_point = view.pos.delta(point).as_vec3();
        let mut g = Grab {
            body,
            hand: body,
            joint: ImpulseJointHandle::invalid(),
            mode,
            mass,
            layer,
            anchor: com,
            rot: Quat::IDENTITY,
            distance: p.hold_distance + radius,
            drag: (Vec3::new(eye_to_point.x, 0.0, eye_to_point.z).length().max(0.3), eye_to_point.y),
            target: None,
            target_prev: None,
            rel: [None, None],
            stretched: 0.0,
        };
        if mode == GrabMode::Drag {
            g.anchor = pose.rotation.inverse() * pose.pos.delta(point).as_vec3();
        }
        let hand_pose = Self::hand_for(&g, &view);
        g.rot = hand_pose.rotation.inverse() * pose.rotation;
        g.hand = world.insert(&BodyDesc::new(BodyKind::Kinematic, hand_pose, ShapeRef::Sphere(0.01), Layer::empty()));
        world.set_layer(body, Layer::HELD);
        world.set_grabbed(body, p.extra_iterations);
        if let Some(h) = self.holder_body {
            world.exclude_pair(body, h);
        }
        g.joint = self.insert_joint(world, &g);
        self.grab = Some(g);
        GrabResult::Grabbed { body, mode, mass }
    }

    /// The linear motors' force cap: `carry_force` when carrying; when
    /// dragging, at least what slides `drag_mass` over a floor of friction
    /// `drag_friction` (else "light enough to move" would not move: S 50 has
    /// 240 N of grip and a 120 kg drag mass).
    fn force_cap(&self, mode: GrabMode, g: f32) -> f32 {
        match mode {
            GrabMode::Carry => self.limits.carry_force,
            GrabMode::Drag => self.limits.carry_force.max(self.params.drag_friction * g * self.limits.drag_mass),
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn insert_joint(&self, world: &mut PhysicsWorld, g: &Grab) -> ImpulseJointHandle {
        let omega = 5.8 / self.params.settle_time;
        let (k, c) = (omega * omega, 2.0 * omega);
        let mut b = GenericJointBuilder::new(JointAxesMask::empty())
            .local_frame1(Pose::from_parts(Vec3::ZERO, if g.mode == GrabMode::Carry { g.rot } else { Quat::IDENTITY }))
            .local_frame2(Pose::from_parts(g.anchor, Quat::IDENTITY))
            .contacts_enabled(false);
        let linear = [JointAxis::LinX, JointAxis::LinY, JointAxis::LinZ];
        let angular = [JointAxis::AngX, JointAxis::AngY, JointAxis::AngZ];
        for axis in linear {
            b = b
                .motor_model(axis, MotorModel::AccelerationBased)
                .motor_position(axis, 0.0, k, c)
                .motor_max_force(axis, self.force_cap(g.mode, world.params().gravity.length()));
        }
        if g.mode == GrabMode::Carry {
            for axis in angular {
                b = b
                    .motor_model(axis, MotorModel::AccelerationBased)
                    .motor_position(axis, 0.0, k, c)
                    .motor_max_force(axis, self.limits.carry_force * self.params.torque_arm);
            }
        }
        let (Some(h), Some(i)) = (world.handle(g.hand), world.handle(g.body)) else { return ImpulseJointHandle::invalid() };
        world.engine.impulse_joints.insert(h, i, b.build(), true)
    }

    /// Turns the carried item in the hand frame (yaw about the view's up,
    /// pitch about its right axis), radians.
    pub fn rotate(&mut self, world: &mut PhysicsWorld, yaw: f32, pitch: f32) {
        let Some(g) = self.grab.as_mut().filter(|g| g.mode == GrabMode::Carry) else { return };
        g.rot = (Quat::from_rotation_y(yaw) * Quat::from_rotation_x(pitch) * g.rot).normalize();
        if let Some(j) = world.engine.impulse_joints.get_mut(g.joint, true) {
            j.data.set_local_frame1(Pose::from_parts(Vec3::ZERO, g.rot));
        }
    }

    /// Before a fixed step that ends at `time`: the hand moves to the view at that time.
    pub fn before_step(&mut self, world: &mut PhysicsWorld, time: f64) -> Option<CarryEnd> {
        let g = self.grab.as_ref()?;
        if !world.contains(g.body) {
            let body = g.body;
            self.end(world);
            return Some(CarryEnd::Lost(body));
        }
        let view = self.view_at(time)?;
        let target = Self::hand_for(g, &view);
        world.set_kinematic_target(g.hand, target);
        let g = self.grab.as_mut()?;
        g.target_prev = g.target.replace(target);
        None
    }

    /// After a fixed step: the item relative to the hand, and the break check.
    pub fn after_step(&mut self, world: &mut PhysicsWorld) -> Option<CarryEnd> {
        let dt = world.params().dt;
        let break_distance = self.params.break_distance;
        let break_time = self.params.break_time;
        let g = self.grab.as_mut()?;
        let (Some(hand), Some(item)) = (g.target, world.pose(g.body)) else { return None };
        let inv = hand.rotation.inverse();
        let rel = Rel { pos: inv * hand.pos.delta(item.pos).as_vec3(), rot: (inv * item.rotation).normalize() };
        g.rel = [g.rel[1].or(Some(rel)), Some(rel)];
        let anchor = item.pos.offset(item.rotation * g.anchor);
        if hand.pos.delta(anchor).as_vec3().length() > break_distance {
            g.stretched += dt;
        } else {
            g.stretched = 0.0;
        }
        if g.stretched > break_time {
            let body = g.body;
            self.end(world);
            return Some(CarryEnd::Broke(body));
        }
        None
    }

    /// Where to draw the carried item this frame (holder space, see the
    /// module docs); `None` when dragging (the body's own interpolated pose
    /// is right: its weight is on the floor, not in the hand).
    pub fn render_pose(&self, view_now: &WorldTransform, alpha: f32) -> Option<(BodyId, WorldTransform)> {
        let g = self.grab.as_ref().filter(|g| g.mode == GrabMode::Carry)?;
        let (Some(a), Some(b)) = (g.rel[0], g.rel[1]) else { return None };
        let hand = Self::hand_for(g, view_now);
        let pos = a.pos.lerp(b.pos, alpha);
        let rot = a.rot.slerp(b.rot, alpha);
        Some((g.body, WorldTransform { pos: hand.pos.offset(hand.rotation * pos), rotation: hand.rotation * rot, scale: Vec3::ONE }))
    }

    /// Lets go: the body keeps its velocity.
    pub fn drop(&mut self, world: &mut PhysicsWorld) -> Option<BodyId> {
        let body = self.grab.as_ref()?.body;
        self.end(world);
        Some(body)
    }

    /// Throws a carried item: the hand's velocity plus `throw_impulse / mass`
    /// along the view, at most `max_throw_speed`. A dragged object is
    /// released, not thrown. Returns the body and its launch speed.
    pub fn throw(&mut self, world: &mut PhysicsWorld) -> Option<(BodyId, f32)> {
        let g = self.grab.as_ref()?;
        let (body, mode, mass) = (g.body, g.mode, g.mass);
        let hand_velocity = match (g.target_prev, g.target) {
            (Some(a), Some(b)) => a.pos.delta(b.pos).as_vec3() / world.params().dt,
            _ => Vec3::ZERO,
        };
        let view = self.views[1].map(|(_, v)| v);
        self.end(world);
        if mode == GrabMode::Drag {
            return Some((body, world.velocity(body).map_or(0.0, |v| v.0.length())));
        }
        let dir = view.map_or(Vec3::NEG_Z, |v| forward(v.rotation));
        let mut v = hand_velocity + dir * (self.limits.throw_impulse / mass.max(1e-3));
        let speed = v.length();
        if speed > self.params.max_throw_speed {
            v *= self.params.max_throw_speed / speed;
        }
        let angular = world.velocity(body).map_or(Vec3::ZERO, |v| v.1);
        world.set_velocity(body, v, angular);
        Some((body, v.length()))
    }

    fn end(&mut self, world: &mut PhysicsWorld) {
        let Some(g) = self.grab.take() else { return };
        world.engine.remove_impulse_joint(g.joint);
        world.remove(g.hand);
        if world.contains(g.body) {
            world.set_layer(g.body, g.layer);
            world.set_grabbed(g.body, 0);
            if let Some(h) = self.holder_body {
                world.include_pair(g.body, h);
            }
            world.wake(g.body);
        }
    }
}
