//! NIF particle systems: description and a CPU simulation.
//!
//! A [`ParticleSystem`] is converted from an `NiParticleSystemController`
//! and its modifier chain. [`Simulation`] advances one instance of it.
//! Morrowind's systems are small (tens of particles per emitter), so the
//! simulation runs on the CPU and the engine only turns the particle list
//! into camera-facing quads.
//!
//! Everything is in engine space (meters, Y up) and relative to the
//! particle geometry node; the engine supplies the emitter's transform
//! relative to that node every step.

use tes3::nif::glam::{Affine3A, Quat, Vec3, Vec4};
use tes3::nif::{
    ForceType, NiColorKey, NiGravity, NiParticleColorModifier, NiParticleGrowFade, NiParticleModifier, NiParticleRotation,
    NiParticleSystemController, NiStream,
};

use crate::anim::{Curve, Timing, color_curve};
use crate::coords;

/// Gravity in the data is weaker than it looks in game; the original
/// engine scales it up. Value from observation of vanilla effects.
const GRAVITY_SCALE: f32 = 1.6;

/// A modifier applied to every live particle.
#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    /// Constant acceleration along `direction` (meters/s²).
    PlanarGravity { acceleration: Vec3 },
    /// Acceleration toward (`strength > 0`) or away from `center`.
    SphericalGravity { center: Vec3, strength: f32, decay: f32 },
    /// Size ramps up over `grow` seconds and down over the last `fade`.
    GrowFade { grow: f32, fade: f32 },
    /// RGBA over normalized age (0 = birth, 1 = death).
    Color(Curve<Vec4>),
    /// Spin in the view plane (radians/s), random initial angle.
    Rotation { speed: f32 },
}

/// Where an emitter places new particles.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum EmitFrom {
    /// The emitter box, centered on the emitter node (`NiParticleSystemController`).
    #[default]
    Box,
    /// A random node of [`ParticleSystem::targets`] per particle
    /// (`NiBSPArrayController`: the emitter node and every node below it, so a
    /// flame atronach burns all over its skeleton), the box placed in that
    /// node's space. With `along_bones` (controller flag 0x8, set on every
    /// vanilla file) the particle starts at a random point between the
    /// node's parent and the node, so the flames cover the limbs.
    Nodes { along_bones: bool },
}

/// A node an [`EmitFrom::Nodes`] system emits from, in the particle node's
/// space. The model holds the rest pose; a host passes live ones to
/// [`Simulation::step`] when the nodes move (an actor's joints).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmitTarget {
    /// Index into [`crate::Model::nodes`].
    pub node: usize,
    /// Index into [`crate::Model::nodes`] of its parent (the bone's start).
    pub parent: Option<usize>,
    pub transform: Affine3A,
    /// The parent's origin (the node's own origin without a parent).
    pub parent_origin: Vec3,
}

/// One emitter, as authored.
#[derive(Clone, Debug, Default, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // independent flags of the format
pub struct ParticleSystem {
    /// The particle geometry node (index into [`crate::Model::nodes`]);
    /// particles live in its space.
    pub node: usize,
    /// Node particles are emitted from (defaults to [`Self::node`]).
    pub emitter: usize,
    /// The emitter's rest transform relative to [`Self::node`].
    pub emitter_transform: Affine3A,
    /// How new particles are placed.
    pub emit_from: EmitFrom,
    /// [`EmitFrom::Nodes`]: the nodes, at rest.
    pub targets: Vec<EmitTarget>,
    /// The emitter or a target moves relative to the particle node in the
    /// model's own animation (a keyframed node between them): a host
    /// should pass their live transforms.
    pub animated_emitter: bool,
    /// Index into [`crate::Model::materials`].
    pub material: usize,
    pub timing: Timing,
    pub auto_play: bool,
    /// Starts at a random point of its cycle (the animation node lacks flag
    /// 0x40); the model's keyframe controllers share the phase.
    pub random_start: bool,
    /// Particles move with the particle node (`NiBSParticleNode` flag
    /// 0x80). Otherwise, the default, they stay where they were emitted in
    /// the world while the node moves (a torch swung in a hand trails its
    /// flame): see [`Simulation::carry`].
    pub local_space: bool,
    pub max_particles: usize,
    /// Particles per second.
    pub birth_rate: f32,
    /// Emission window in key time (start inclusive, stop exclusive);
    /// empty means always. See [`Self::emits_at`].
    pub emit_start: f32,
    pub emit_stop: f32,
    pub lifespan: f32,
    pub lifespan_variation: f32,
    /// Meters/s.
    pub speed: f32,
    pub speed_variation: f32,
    /// Angle from the emitter's up axis, and its variation (radians).
    pub declination: f32,
    pub declination_variation: f32,
    /// Angle around the emitter's up axis, and its variation (radians).
    pub planar_angle: f32,
    pub planar_angle_variation: f32,
    /// Emitter box size in emitter space (meters, engine axes).
    pub emitter_box: Vec3,
    pub initial_color: Vec4,
    /// Particle radius (half the quad width) in meters.
    pub initial_size: f32,
    pub modifiers: Vec<Modifier>,
}

impl ParticleSystem {
    /// Converts a controller; node, emitter, and material are filled in by
    /// the caller.
    pub(crate) fn from_controller(stream: &NiStream, controller: &NiParticleSystemController, max_particles: usize) -> Self {
        let rate = if controller.use_birth_rate {
            controller.birth_rate
        } else {
            // Keep the pool full: one particle per slot per average lifetime.
            #[allow(clippy::cast_precision_loss)]
            let max = max_particles as f32;
            max / (controller.lifespan + controller.lifespan_variation / 2.0).max(0.01)
        };
        let box_mw = Vec3::new(controller.emitter_width, controller.emitter_height, controller.emitter_depth);
        Self {
            node: 0,
            emitter: 0,
            emitter_transform: Affine3A::IDENTITY,
            emit_from: EmitFrom::Box,
            targets: Vec::new(),
            animated_emitter: false,
            material: 0,
            timing: Timing::from_controller(&controller.base),
            auto_play: true,
            random_start: false,
            local_space: false,
            max_particles,
            birth_rate: rate,
            emit_start: controller.emit_start_time,
            emit_stop: controller.emit_stop_time,
            lifespan: controller.lifespan,
            lifespan_variation: controller.lifespan_variation,
            speed: controller.speed * coords::UNIT_METERS,
            speed_variation: controller.speed_variation * coords::UNIT_METERS,
            declination: controller.declination_angle,
            declination_variation: controller.declination_variation,
            planar_angle: controller.planar_angle,
            planar_angle_variation: controller.planar_angle_variation,
            emitter_box: coords::point(box_mw).abs(),
            initial_color: controller.initial_color,
            initial_size: controller.initial_size * coords::UNIT_METERS,
            modifiers: modifiers(stream, controller),
        }
    }

    /// Seconds a particle lives at most.
    pub fn max_lifetime(&self) -> f32 {
        self.lifespan + self.lifespan_variation.max(0.0)
    }

    /// Whether the system emits at controller clock `t` (seconds): the
    /// controller time ([`Timing::key_time`]) inside the emission window,
    /// start inclusive and stop exclusive (as `OpenMW`, black box).
    pub fn emits_at(&self, t: f32) -> bool {
        if self.emit_stop <= self.emit_start {
            return true;
        }
        let key = self.timing.key_time(t);
        key >= self.emit_start && key < self.emit_stop
    }

    /// Whether the emission window covers the whole controller range (a
    /// torch: always burning) rather than a moment of it (a creature's cast
    /// or death effect).
    pub fn emits_throughout(&self) -> bool {
        self.emit_stop <= self.emit_start || {
            let slack = WINDOW_SLACK * (self.timing.stop - self.timing.start);
            self.emit_start <= self.timing.start + slack && self.emit_stop >= self.timing.stop - slack
        }
    }
}

/// Windows within this share of the controller range of its ends count as
/// covering it (`atronach_fire.nif`'s body flames stop 1 s, 3.5 %, before the
/// end of its 28.7 s timeline).
const WINDOW_SLACK: f32 = 0.05;

fn modifiers(stream: &NiStream, controller: &NiParticleSystemController) -> Vec<Modifier> {
    let mut out = Vec::new();
    let mut next = controller.particle_modifier.key;
    for _ in 0..32 {
        let Some(object) = stream.objects.get(next) else {
            break;
        };
        if let Ok(gravity) = <&NiGravity>::try_from(object) {
            let strength = gravity.strength * GRAVITY_SCALE * coords::UNIT_METERS;
            out.push(match gravity.force_type {
                ForceType::Planar => {
                    Modifier::PlanarGravity { acceleration: coords::direction(gravity.direction).normalize_or_zero() * strength }
                }
                ForceType::Spherical => Modifier::SphericalGravity {
                    center: coords::point(gravity.position),
                    strength,
                    decay: gravity.decay / coords::UNIT_METERS,
                },
            });
        } else if let Ok(grow) = <&NiParticleGrowFade>::try_from(object) {
            out.push(Modifier::GrowFade { grow: grow.grow_time, fade: grow.fade_time });
        } else if let Ok(color) = <&NiParticleColorModifier>::try_from(object) {
            if let Some(data) = stream.get(color.color_data) {
                let NiColorKey::LinKey(keys) = &data.keys;
                out.push(Modifier::Color(color_curve(keys.iter().map(|k| (k.time, k.value)))));
            }
        } else if let Ok(rotation) = <&NiParticleRotation>::try_from(object) {
            out.push(Modifier::Rotation { speed: rotation.rotation_speed });
        }
        // Colliders and bombs are not simulated (one vanilla file uses a
        // planar collider).
        let Ok(modifier) = <&NiParticleModifier>::try_from(object) else {
            break;
        };
        next = modifier.next.key;
    }
    out
}

/// One live particle, in the particle node's space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub age: f32,
    pub lifetime: f32,
    /// Radius (half the quad width) in meters, after modifiers.
    pub size: f32,
    /// Linear RGBA, after modifiers.
    pub color: Vec4,
    /// In-plane rotation, radians.
    pub angle: f32,
}

/// The running state of one emitter instance.
#[derive(Clone, Debug)]
pub struct Simulation {
    pub particles: Vec<Particle>,
    /// Fractional particles owed by the birth rate.
    pending: f32,
    rng: u64,
}

impl Simulation {
    pub fn new(seed: u64) -> Self {
        Self { particles: Vec::new(), pending: 0.0, rng: seed | 1 }
    }

    /// Runs the system for its longest particle lifetime up to controller
    /// clock `now` (see [`ParticleSystem::emits_at`]) so it starts in its
    /// steady state (a torch is already burning when you walk in), at rest.
    /// `emitting` overrides the clock (hosts that gate emission themselves).
    pub fn prewarm(&mut self, system: &ParticleSystem, now: f32, emitting: Option<bool>) {
        const STEP: f32 = 1.0 / 30.0;
        let duration = system.max_lifetime().min(10.0);
        let mut t = now - duration;
        while t < now {
            let emits = emitting.unwrap_or_else(|| system.emits_at(t));
            self.step(system, STEP, emits, system.emitter_transform, &[]);
            t += STEP;
        }
    }

    /// Advances by `dt` seconds, emitting new particles if `emitting` (see
    /// [`ParticleSystem::emits_at`]). `emitter` maps emitter space to the
    /// particle node's space; `targets` are the live [`EmitTarget`]s of an
    /// [`EmitFrom::Nodes`] system (empty: the rest ones).
    pub fn step(&mut self, system: &ParticleSystem, dt: f32, emitting: bool, emitter: Affine3A, targets: &[EmitTarget]) {
        // Age, move, and retire.
        for p in &mut self.particles {
            p.age += dt;
        }
        self.particles.retain(|p| p.age < p.lifetime);
        for p in &mut self.particles {
            for modifier in &system.modifiers {
                match modifier {
                    Modifier::PlanarGravity { acceleration } => p.velocity += *acceleration * dt,
                    Modifier::SphericalGravity { center, strength, decay } => {
                        let offset = *center - p.position;
                        let distance = offset.length();
                        let falloff = if *decay > 0.0 { (-decay * distance).exp() } else { 1.0 };
                        p.velocity += offset.normalize_or_zero() * (strength * falloff * dt);
                    }
                    _ => {}
                }
            }
            p.position += p.velocity * dt;
        }

        // Emit.
        if emitting {
            self.pending += system.birth_rate * dt;
        } else {
            self.pending = 0.0;
        }
        while self.pending >= 1.0 {
            self.pending -= 1.0;
            if self.particles.len() >= system.max_particles {
                continue;
            }
            let targets = if targets.is_empty() { system.targets.as_slice() } else { targets };
            let particle = self.spawn(system, emitter, targets);
            self.particles.push(particle);
        }

        // Appearance.
        for p in &mut self.particles {
            let life = if p.lifetime > 0.0 { (p.age / p.lifetime).clamp(0.0, 1.0) } else { 1.0 };
            let mut scale = 1.0;
            for modifier in &system.modifiers {
                match modifier {
                    Modifier::GrowFade { grow, fade } => {
                        if *grow > 0.0 && p.age < *grow {
                            scale *= p.age / grow;
                        }
                        let left = p.lifetime - p.age;
                        if *fade > 0.0 && left < *fade {
                            scale *= left / fade;
                        }
                    }
                    Modifier::Color(curve) => p.color = curve.sample(life).unwrap_or(p.color),
                    Modifier::Rotation { speed } => p.angle += speed * dt,
                    _ => {}
                }
            }
            p.size = system.initial_size * scale.clamp(0.0, 1.0);
        }
    }

    /// Keeps world-space particles in place while their node moves: maps
    /// positions and velocities by `delta` (node space now ← node space at
    /// the previous step: `now⁻¹ · before`). Gravity keeps acting in node
    /// space, as in the original engine (its gravity turns with the node).
    pub fn carry(&mut self, delta: Affine3A) {
        for p in &mut self.particles {
            p.position = delta.transform_point3(p.position);
            p.velocity = delta.transform_vector3(p.velocity);
        }
    }

    fn spawn(&mut self, system: &ParticleSystem, emitter: Affine3A, targets: &[EmitTarget]) -> Particle {
        // The frame this particle is born in: the emitter, or one of the
        // nodes (moved along its bone).
        let emitter = match system.emit_from {
            EmitFrom::Nodes { along_bones } if !targets.is_empty() => {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
                let pick = ((self.unit() * targets.len() as f32) as usize).min(targets.len() - 1);
                let target = targets[pick];
                let mut frame = target.transform;
                if along_bones {
                    let start = tes3::nif::glam::Vec3A::from(target.parent_origin);
                    frame.translation = start.lerp(frame.translation, self.unit());
                }
                frame
            }
            _ => emitter,
        };
        let declination = system.declination + system.declination_variation * self.signed();
        let planar = system.planar_angle + system.planar_angle_variation * self.signed();
        // Morrowind emitter space: tilt off +Z by the declination, then turn
        // around Z by the planar angle.
        let direction_mw = Quat::from_rotation_z(planar) * Quat::from_rotation_y(declination) * Vec3::Z;
        let direction = emitter.transform_vector3(coords::direction(direction_mw)).normalize_or_zero();
        let speed = system.speed + system.speed_variation * self.signed();
        let offset = system.emitter_box * Vec3::new(self.signed(), self.signed(), self.signed()) * 0.5;
        let lifetime = system.lifespan + system.lifespan_variation * self.unit();
        let rotates = system.modifiers.iter().any(|m| matches!(m, Modifier::Rotation { .. }));
        Particle {
            position: emitter.transform_point3(offset),
            velocity: direction * speed,
            age: 0.0,
            lifetime,
            size: 0.0,
            color: system.initial_color,
            angle: if rotates { self.unit() * std::f32::consts::TAU } else { 0.0 },
        }
    }

    /// Uniform in `[0, 1)` (xorshift64*).
    fn unit(&mut self) -> f32 {
        self.rng ^= self.rng >> 12;
        self.rng ^= self.rng << 25;
        self.rng ^= self.rng >> 27;
        let bits = self.rng.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 40;
        #[allow(clippy::cast_precision_loss)]
        let value = bits as f32 / (1u64 << 24) as f32;
        value
    }

    /// Uniform in `[-1, 1)`.
    fn signed(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn torch() -> ParticleSystem {
        ParticleSystem {
            node: 0,
            emitter: 0,
            emitter_transform: Affine3A::IDENTITY,
            emit_from: EmitFrom::Box,
            targets: Vec::new(),
            animated_emitter: false,
            material: 0,
            timing: Timing { stop: 1.0, ..Timing::default() },
            auto_play: true,
            random_start: false,
            local_space: false,
            max_particles: 20,
            birth_rate: 20.0,
            emit_start: 0.0,
            emit_stop: 0.0,
            lifespan: 1.0,
            lifespan_variation: 0.0,
            speed: 1.0,
            speed_variation: 0.0,
            declination: 0.0,
            declination_variation: 0.0,
            planar_angle: 0.0,
            planar_angle_variation: 0.0,
            emitter_box: Vec3::ZERO,
            initial_color: Vec4::ONE,
            initial_size: 0.2,
            modifiers: vec![Modifier::GrowFade { grow: 0.5, fade: 0.5 }],
        }
    }

    #[test]
    fn emits_upward_and_reaches_steady_state() {
        let system = torch();
        let mut sim = Simulation::new(7);
        sim.prewarm(&system, 0.0, None);
        let n = sim.particles.len();
        assert!((15..=20).contains(&n), "about rate x lifetime particles, got {n}");
        // Declination 0 is Morrowind up, engine +Y.
        assert!(sim.particles.iter().all(|p| p.velocity.y > 0.99 && p.velocity.x.abs() < 1e-5));
        // Grow/fade keeps sizes within the authored size.
        assert!(sim.particles.iter().all(|p| p.size <= 0.2 + 1e-6));
        assert!(sim.particles.iter().any(|p| p.size > 0.1));
    }

    #[test]
    fn respects_the_pool_size_and_lifetimes() {
        let system = ParticleSystem { birth_rate: 1000.0, ..torch() };
        let mut sim = Simulation::new(1);
        for _ in 0..10 {
            sim.step(&system, 0.1, true, Affine3A::IDENTITY, &[]);
        }
        assert_eq!(sim.particles.len(), 20);
        assert!(sim.particles.iter().all(|p| p.age < p.lifetime));
    }

    #[test]
    fn color_follows_normalized_age_and_gravity_pulls() {
        let system = ParticleSystem {
            modifiers: vec![
                Modifier::Color(Curve::linear([(0.0, Vec4::ONE), (1.0, Vec4::ZERO)])),
                Modifier::PlanarGravity { acceleration: Vec3::new(0.0, -10.0, 0.0) },
            ],
            birth_rate: 1.0,
            ..torch()
        };
        let mut sim = Simulation::new(3);
        sim.step(&system, 1.0, true, Affine3A::IDENTITY, &[]);
        assert_eq!(sim.particles.len(), 1);
        for _ in 0..5 {
            sim.step(&system, 0.1, true, Affine3A::IDENTITY, &[]);
        }
        let p = sim.particles[0];
        assert!((p.color.x - 0.5).abs() < 0.01, "{p:?}");
        assert!(p.velocity.y < 0.0, "gravity overcame the launch speed");
    }

    #[test]
    fn emitter_transform_moves_and_turns_emission() {
        let system = ParticleSystem { birth_rate: 1.0, ..torch() };
        let emitter = Affine3A::from_rotation_translation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2), Vec3::new(1.0, 2.0, 3.0));
        let mut sim = Simulation::new(5);
        sim.step(&system, 1.0, true, emitter, &[]);
        let p = sim.particles[0];
        assert!((p.position - Vec3::new(1.0, 2.0, 3.0)).length() < 1e-5);
        assert!(p.velocity.x > 0.99, "up turned to +X: {:?}", p.velocity);
    }

    #[test]
    fn node_emitters_spread_particles_along_their_bones_and_follow_live_nodes() {
        let target = |node, y: f32| EmitTarget {
            node,
            parent: Some(0),
            transform: Affine3A::from_translation(Vec3::new(0.0, y, 0.0)),
            parent_origin: Vec3::ZERO,
        };
        let system = ParticleSystem {
            emit_from: EmitFrom::Nodes { along_bones: true },
            targets: vec![target(1, 1.0), target(2, -1.0)],
            birth_rate: 400.0,
            max_particles: 400,
            speed: 0.0,
            ..torch()
        };
        let mut sim = Simulation::new(11);
        sim.step(&system, 0.5, true, Affine3A::IDENTITY, &[]);
        let ys: Vec<f32> = sim.particles.iter().map(|p| p.position.y).collect();
        assert!(ys.iter().all(|y| y.abs() <= 1.0 + 1e-5), "on the bones");
        assert!(ys.iter().any(|&y| y > 0.5) && ys.iter().any(|&y| y < -0.5), "both bones used");
        assert!(ys.iter().any(|y| y.abs() < 0.5), "spread along the bone, not only at its end");

        // Live targets (a posed skeleton) replace the rest ones.
        let live = [EmitTarget {
            transform: Affine3A::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            parent_origin: Vec3::new(5.0, 0.0, 0.0),
            ..target(1, 0.0)
        }];
        let mut sim = Simulation::new(12);
        sim.step(&system, 0.1, true, Affine3A::IDENTITY, &live);
        assert!(!sim.particles.is_empty() && sim.particles.iter().all(|p| (p.position - Vec3::new(5.0, 0.0, 0.0)).length() < 1e-5));
    }

    #[test]
    fn emission_windows_follow_the_controller_clock() {
        let system = ParticleSystem { emit_start: 0.25, emit_stop: 0.5, timing: Timing { stop: 1.0, ..Timing::default() }, ..torch() };
        assert!(!system.emits_at(0.1) && system.emits_at(0.25) && system.emits_at(0.4) && !system.emits_at(0.5));
        assert!(system.emits_at(1.3), "loops with the controller");
        assert!(!system.emits_throughout());
        let always = ParticleSystem { emit_start: -1.0, emit_stop: 1.0, ..system.clone() };
        assert!(always.emits_throughout() && always.emits_at(0.9));
        // Prewarmed outside the window: nothing alive.
        let mut sim = Simulation::new(2);
        sim.prewarm(&ParticleSystem { emit_start: 0.6, emit_stop: 0.7, lifespan: 0.2, ..system.clone() }, 0.5, None);
        assert!(sim.particles.is_empty());
        sim.prewarm(&ParticleSystem { lifespan: 0.2, ..system }, 0.5, None);
        assert!(!sim.particles.is_empty(), "inside the window: alive");
    }

    #[test]
    fn carried_particles_stay_put_in_the_world_while_the_node_moves() {
        let system = torch();
        let mut sim = Simulation::new(3);
        sim.prewarm(&system, 0.0, None);
        let before = Affine3A::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let now = Affine3A::from_rotation_translation(Quat::from_rotation_y(0.7), Vec3::new(10.5, 0.2, -0.3));
        let world: Vec<(Vec3, Vec3)> =
            sim.particles.iter().map(|p| (before.transform_point3(p.position), before.transform_vector3(p.velocity))).collect();
        sim.carry(now.inverse() * before);
        for (p, (position, velocity)) in sim.particles.iter().zip(world) {
            assert!((now.transform_point3(p.position) - position).length() < 1e-4);
            assert!((now.transform_vector3(p.velocity) - velocity).length() < 1e-4);
        }
    }
}
