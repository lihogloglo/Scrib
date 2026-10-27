//! A takeover (ROADMAP 3.2b slice 2a): Delirium drives an animated character from its current pose, then says when
//! to hand it back to the animation: recovered (after a blend-out, slice 2b), or down on the floor (the host plays the
//! get-up).
//!
//! Blend-out (slice 2b, the powered-ragdoll blend-out to animation): once the balance has caught the body (no catch step
//! under way, centre of mass slow), [`Takeover::weight`] ramps from 1 to 0 and the host shows the animation through it
//! (its physics-pose weight), so the balancer's own stance (the asset's zero pose, arms out) never stands on its own.
//! A push or a new catch step brings the physics back.
//!
//! Roll-over before the get-up (DE-1): a yaw-only get-up frame leaves the tilt difference between the lying pelvis and
//! the get-up clip's first pose for the hand-back's blend to sweep (up to 165 deg: the "magic flip"). With the clip's
//! tilt set ([`Takeover::set_get_up`]), a body down and ready first rolls over to it (a velocity servo on the torso, the
//! Euphoria-style roll before a get-up); measured by `takeover_real_install::get_up_blend_rotation`.
//!
//! Bevy-free. The host turns the animated pose into part frames ([`crate::bones_to_parts`], sim space: Z up, metres,
//! the actor's own authoring frame at the takeover), steps [`Takeover::step`] at a fixed rate and copies the parts back
//! ([`crate::pose_to_bones`]) every frame. [`pick`] finds the part under a ray (the dev click tool).

use delirium::collision::Collision;
use delirium::msg::{self, Value};
use delirium::nm::Agent;
use delirium::prelude::*;
use delirium::ragdoll::{Build, PartDef, Ragdoll};

use crate::Body;
use crate::sim::install;

/// Hand-back rules.
#[derive(Clone, Copy, Debug)]
pub struct Rules {
    /// Still: centre of mass slower than this, m/s ... (also counts as recovered)
    pub still_speed: f32,
    /// ... for this long, s.
    pub still_secs: f32,
    /// No hand-back before this, s (the push has to land first).
    pub min_secs: f32,
    /// Down: pelvis under this share of its standing height (or Delirium's own `fallen`).
    pub fallen_ratio: f32,
    /// Down this long at least before the get-up, s; at most this long even if not still.
    pub down_secs: (f32, f32),
    /// Balancing this long without coming to rest: hand back anyway, s.
    pub max_secs: f32,
    /// Pelvis sinking faster than this at the takeover: start with `catchFall` instead of `bodyBalance`, m/s.
    pub falling_speed: f32,
    /// Start velocities are clamped to these (m/s, rad/s): a pose jump between two frames is no motion.
    pub max_velocity: (f32, f32),
    /// Recovered (the blend-out starts): balancing, no catch step under way, centre of mass slower than this
    /// horizontally, m/s ... (Delirium's own `Agent::settled`: 0.25 m/s for 0.4 s)
    pub recover_speed: f32,
    /// ... for this long, s.
    pub recover_secs: f32,
    /// Blend-out: [`Takeover::weight`] goes from 1 to 0 over this, s, then the hand-back; 0: hands back at once.
    pub blend_out: f32,
    /// A push or a new catch step during the blend-out: back to 1 over this, s.
    pub blend_in: f32,
    /// Roll-over before the get-up ([`Takeover::set_get_up`]): down and ready, the body rolls until its pelvis tilt is
    /// within this of the get-up clip's first pose, rad (the hand-back's blend sweeps only what is left) ...
    pub roll_tolerance: f32,
    /// ... at most this fast about the roll axis, rad/s (slowing near the target, [`Self::roll_gain`]) ...
    pub roll_speed: f32,
    /// ... the roll rate asked per radian left, 1/s ...
    pub roll_gain: f32,
    /// ... for at most this long, s (then the get-up anyway).
    pub roll_secs: f32,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            still_speed: 0.05,
            still_secs: 0.5,
            min_secs: 0.5,
            fallen_ratio: 0.5,
            down_secs: (1.5, 4.0),
            max_secs: 15.0,
            falling_speed: 1.0,
            max_velocity: (8.0, 15.0),
            recover_speed: 0.25,
            recover_secs: 0.15,
            blend_out: 0.5,
            blend_in: 0.1,
            roll_tolerance: 20f32.to_radians(),
            roll_speed: 2.5,
            roll_gain: 4.0,
            roll_secs: 3.0,
        }
    }
}

/// Why the takeover ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandBack {
    /// Recovered (or still) and blended out: back to the graph where it stands.
    Balanced,
    /// Balanced but never still within [`Rules::max_secs`]: back to the graph anyway.
    Timeout,
    /// Down and still (or [`Rules::down_secs`] up): the host plays the get-up.
    Fallen,
    /// The simulation blew up (non-finite or runaway): back to the graph at once.
    Exploded,
}

/// One character under Delirium.
pub struct Takeover {
    pub build: Build,
    pub world: Collision,
    pub agent: Agent,
    pub rules: Rules,
    /// Floor height, sim z.
    pub floor: f32,
    /// Pelvis height over the floor at the takeover.
    pub stand: f32,
    /// Seconds since the takeover.
    pub time: f32,
    /// Seconds down (`Some` once it fell).
    pub down: Option<f32>,
    /// The behaviour it started with.
    pub behaviour: &'static str,
    /// The physics share of the shown pose (1: Delirium's pose, 0: the animation's), the host's physics-pose weight.
    pub weight: f32,
    /// Seconds rolling over before the get-up (`Some` once the roll started).
    pub rolling: Option<f32>,
    still: f32,
    calm: f32,
    blending: bool,
    /// The get-up clip's first pose: sim up (+Z) in its pelvis part's frame ([`get_up_tilt`]).
    get_up: Option<Vec3>,
}

/// The torso parts the roll-over turns (Buttocks, Spine0-3); the limbs follow through the joints.
const TORSO: [usize; 5] = [0, 7, 8, 9, 10];

/// Up (sim +Z) in the frame of a part at `rotation` (sim space): the part's tilt, free of its heading. Two poses whose
/// pelvis parts have the same tilt differ by a turn about the vertical only, which a yaw-only get-up frame takes up.
pub fn tilt_of(rotation: Quat) -> Vec3 {
    rotation.inverse() * Vec3::Z
}

/// The whole-body rotation a yaw-only get-up frame leaves to the hand-back's blend: the angle between the lying pelvis
/// part's tilt (`rotation`, sim space) and the get-up clip's (`clip`, [`tilt_of`]), rad.
pub fn get_up_tilt(rotation: Quat, clip: Vec3) -> f32 {
    tilt_of(rotation).angle_between(clip)
}

/// Linear and angular velocity between two frames of a part.
fn velocity(a: &Transform, b: &Transform, dt: f32, max: (f32, f32)) -> (Vec3, Vec3) {
    let v = ((b.translation - a.translation) / dt).clamp_length_max(max.0);
    let (axis, angle) = (b.rotation * a.rotation.inverse()).normalize().to_axis_angle();
    let angle = if angle > std::f32::consts::PI { angle - std::f32::consts::TAU } else { angle };
    let w = (axis * angle / dt).clamp_length_max(max.1);
    if v.is_finite() && w.is_finite() { (v, w) } else { (Vec3::ZERO, Vec3::ZERO) }
}

/// Lowest point of a part (capsule along its frame's Y, or box) over z = 0.
fn lowest(p: &PartDef, f: &Transform) -> f32 {
    let m = Mat3::from_quat(f.rotation);
    match p.boxed {
        Some(e) => f.translation.z - (m.x_axis.z.abs() * e.x + m.y_axis.z.abs() * e.y + m.z_axis.z.abs() * e.z),
        None => f.translation.z - (m.y_axis.z * p.half).abs() - p.radius,
    }
}

impl Takeover {
    /// Takes the character over at `parts` (sim space), moving from `previous` (`dt` earlier) when known.
    /// `scale`: Delirium's per-actor size (`Build::scaled`: segment lengths times `height`, mass factor `weight`),
    /// `None` for the authored size. `floor`: the ground under the pelvis (sim z); a body already sunk into it lowers
    /// it to its lowest point. Starts `bodyBalance`, or `catchFall` when the pelvis is already falling.
    ///
    /// # Panics
    /// The character carries no build, `parts` is not one frame per part, or Delirium refuses the behaviour message.
    pub fn start(body: &Body, parts: &[Transform], previous: Option<(&[Transform], f32)>, scale: Option<(f32, f32)>, floor: f32, seed: f32, rules: Rules) -> Takeover {
        install(&body.character);
        let (mut build, bind) = body.character.build().expect("the character carries its build");
        assert_eq!(parts.len(), build.parts.len(), "one frame per part");
        if let Some((height, weight)) = scale {
            build = build.scaled(&bind, &[height; 21], weight).0;
        }
        let floor = build.parts.iter().zip(parts).map(|(p, f)| lowest(p, f)).fold(floor, f32::min);
        let mut world = Collision::default();
        world.add_floor(parts[0].translation.x, parts[0].translation.y, floor, 50.0, 0);
        let mut doll = Ragdoll::new(&build, parts, Vec3::ZERO);
        let mut sinking = 0.0;
        if let Some((prev, dt)) = previous.filter(|(p, dt)| p.len() == parts.len() && *dt > 1e-4) {
            for (k, (a, b)) in prev.iter().zip(parts).enumerate() {
                let (v, w) = velocity(a, b, dt, rules.max_velocity);
                doll.set_body(k, b.translation, b.rotation, v, w);
                if k == 0 {
                    sinking = -v.z;
                }
            }
        }
        let agent = Agent::new(&build, doll, Quat::IDENTITY, seed);
        let behaviour = if sinking > rules.falling_speed { "catchFall" } else { "bodyBalance" };
        let mut t = Takeover {
            build,
            world,
            agent,
            rules,
            floor,
            stand: (parts[0].translation.z - floor).max(0.1),
            time: 0.0,
            down: None,
            behaviour,
            weight: 1.0,
            rolling: None,
            still: 0.0,
            calm: 0.0,
            blending: false,
            get_up: None,
        };
        t.send(behaviour, &[("start", Value::Bool(true))]);
        t
    }

    /// Sends a behaviour message.
    ///
    /// # Panics
    /// Delirium refuses the message.
    pub fn send(&mut self, name: &str, params: &[(&str, Value)]) {
        let mut st = std::mem::take(&mut self.agent.msgs);
        let r = msg::send(&mut self.agent, &self.build, &self.world, &mut st, name, params);
        self.agent.msgs = st;
        if let Err(e) = r {
            panic!("{name}: {e:?}");
        }
    }

    /// An impulse (N s, sim space) on part `part` at `at` (sim space). Restarts the still timers and cancels a
    /// blend-out.
    pub fn push(&mut self, part: usize, at: Vec3, impulse: Vec3) {
        self.agent.doll.impulse(part, at, impulse);
        self.still = 0.0;
        self.calm = 0.0;
        self.blending = false;
    }

    /// Whether the blend-out to the animation runs.
    pub fn blending(&self) -> bool {
        self.blending
    }

    /// Whether Delirium's balancer is taking a catch step (`None`: not balancing).
    pub fn stepping(&self) -> Option<bool> {
        self.agent.balance_steps().map(|(_, stepping)| stepping != 0)
    }

    pub fn frames(&self) -> Vec<Transform> {
        self.agent.doll.frames().collect()
    }

    /// Centre of mass and its velocity.
    pub fn com(&self) -> (Vec3, Vec3) {
        self.agent.doll.com()
    }

    /// The get-up clip's first pose as its pelvis part's tilt ([`tilt_of`] of the part's rotation in the clip's model
    /// frame, sim axes). Down and ready, the body then rolls over to it before the [`HandBack::Fallen`] hand-back (the
    /// Euphoria-style roll before a get-up: the get-up clip lies on one side, the blend only takes up what is left).
    pub fn set_get_up(&mut self, clip: Vec3) {
        self.get_up = clip.try_normalize();
    }

    /// The pelvis tilt left to the get-up clip's, rad (`None`: no get-up clip set).
    pub fn get_up_left(&self) -> Option<f32> {
        Some(get_up_tilt(self.agent.doll.frames().next()?.rotation, self.get_up?))
    }

    /// One roll-over step; `true` once done (within [`Rules::roll_tolerance`] and slow, out of time, or no clip).
    /// A velocity servo about the shortest arc from the clip's up (as the body carries it) to the vertical: couples on
    /// the torso parts, sized for a part of radius `ROLL_RADIUS`; the joints drag the limbs along.
    fn roll_over(&mut self, dt: f32) -> bool {
        const ROLL_RADIUS: f32 = 0.25;
        const SETTLED: f32 = 1.0;
        let Some(clip) = self.get_up else { return true };
        let rolled = self.rolling.get_or_insert(0.0);
        *rolled += dt;
        let out_of_time = *rolled >= self.rules.roll_secs;
        let frames = self.frames();
        let left = get_up_tilt(frames[0].rotation, clip);
        let spine = (frames[10].translation - frames[0].translation).normalize_or(Vec3::X);
        let axis = (frames[0].rotation * clip).cross(Vec3::Z).try_normalize().unwrap_or(spine);
        let mass: f32 = TORSO.iter().map(|&k| self.agent.doll.part_mass(k)).sum();
        let spin = TORSO.iter().map(|&k| self.agent.doll.part_mass(k) * self.agent.doll.vel(k).1.dot(axis)).sum::<f32>() / mass;
        if out_of_time || (left < self.rules.roll_tolerance && spin.abs() < SETTLED) {
            return true;
        }
        let want = (self.rules.roll_gain * left).min(self.rules.roll_speed);
        let side = axis.any_orthonormal_vector();
        for k in TORSO {
            let (m, w) = (self.agent.doll.part_mass(k), self.agent.doll.vel(k).1);
            let angular = m * ROLL_RADIUS * ROLL_RADIUS * (want - w.dot(axis));
            // A couple: +f at +r, -f at -r (r = 0.1 side, f along axis x side) is an angular impulse of 0.2 f about axis.
            let at = frames[k].translation;
            let f = axis.cross(side) * (angular / 0.2);
            self.agent.doll.impulse(k, at + side * 0.1, f);
            self.agent.doll.impulse(k, at - side * 0.1, -f);
        }
        false
    }

    /// Whether it is down (fell this takeover).
    pub fn fell(&self) -> bool {
        self.down.is_some()
    }

    /// One step of `dt` seconds; `Some` when the character goes back to the animation.
    pub fn step(&mut self, dt: f32) -> Option<HandBack> {
        self.agent.step(&self.build, &self.world, dt, &[], &[]);
        self.time += dt;
        let frames = self.frames();
        let runaway = (0..self.build.parts.len()).any(|k| self.agent.doll.vel(k).0.length() > 30.0);
        if runaway || !frames.iter().all(|f| f.translation.is_finite() && f.rotation.is_finite()) {
            return Some(HandBack::Exploded);
        }
        let velocity = self.com().1;
        self.still = if velocity.length() < self.rules.still_speed { self.still + dt } else { 0.0 };
        let still = self.still >= self.rules.still_secs;
        let stepping = self.stepping();
        let calm = stepping == Some(false) && velocity.truncate().length() < self.rules.recover_speed;
        self.calm = if calm { self.calm + dt } else { 0.0 };
        if self.down.is_none() && (self.agent.fallen || frames[0].translation.z - self.floor < self.rules.fallen_ratio * self.stand) {
            self.down = Some(0.0);
        }
        // Blend-out once recovered; a new catch step or a fall cancels it (a push too, `push`).
        if self.down.is_none() && self.time >= self.rules.min_secs && (still || self.calm >= self.rules.recover_secs) {
            self.blending = true;
        } else if stepping == Some(true) || self.down.is_some() {
            self.blending = false;
        }
        let rate = |secs: f32| if secs > 0.0 { dt / secs } else { 1.0 };
        self.weight = if self.blending {
            (self.weight - rate(self.rules.blend_out)).max(0.0)
        } else {
            (self.weight + rate(self.rules.blend_in)).min(1.0)
        };
        match &mut self.down {
            Some(down) => {
                *down += dt;
                let (min, max) = self.rules.down_secs;
                let ready = self.rolling.is_some() || (still && *down >= min) || *down >= max;
                (ready && self.roll_over(dt)).then_some(HandBack::Fallen)
            }
            None if self.blending && self.weight <= 0.0 => Some(HandBack::Balanced),
            None if !self.blending && self.time >= self.rules.max_secs => Some(HandBack::Timeout),
            None => None,
        }
    }
}

/// The part a ray hits first: `(part, distance along the ray, point)`, sim space. `dir` normalized. Capsules are
/// tested exactly as capsules; boxes as capsules along their longest side.
pub fn pick(build: &Build, parts: &[Transform], origin: Vec3, dir: Vec3, max: f32) -> Option<(usize, f32, Vec3)> {
    let mut best: Option<(usize, f32, Vec3)> = None;
    for (k, (p, f)) in build.parts.iter().zip(parts).enumerate() {
        let (axis, half, radius) = match p.boxed {
            Some(e) => {
                let m = Mat3::from_quat(f.rotation);
                let mut sides = [(m.x_axis, e.x), (m.y_axis, e.y), (m.z_axis, e.z)];
                sides.sort_by(|a, b| b.1.total_cmp(&a.1));
                (sides[0].0, sides[0].1 - sides[1].1, sides[1].1)
            }
            None => (f.rotation * Vec3::Y, p.half, p.radius),
        };
        if let Some(t) = ray_capsule(origin, dir, f.translation - axis * half, f.translation + axis * half, radius)
            && t <= max
            && best.is_none_or(|b| t < b.1)
        {
            best = Some((k, t, origin + dir * t));
        }
    }
    best
}

/// Distance along the ray (`dir` normalized) to a capsule (segment `a`-`b`, `radius`), if it hits.
pub fn ray_capsule(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, radius: f32) -> Option<f32> {
    // Ray against the infinite cylinder, clipped to the segment, then the two end spheres.
    let mut best: Option<f32> = None;
    let mut keep = |t: f32| {
        if t >= 0.0 && best.is_none_or(|b| t < b) {
            best = Some(t);
        }
    };
    let ab = b - a;
    let len2 = ab.length_squared();
    if len2 > 1e-10 {
        let ao = origin - a;
        let (d_ab, o_ab) = (dir.dot(ab), ao.dot(ab));
        let qa = len2 - d_ab * d_ab;
        let qb = len2 * ao.dot(dir) - o_ab * d_ab;
        let qc = len2 * (ao.length_squared() - radius * radius) - o_ab * o_ab;
        let disc = qb * qb - qa * qc;
        if qa.abs() > 1e-10 && disc >= 0.0 {
            let t = (-qb - disc.sqrt()) / qa;
            let s = o_ab + t * d_ab;
            if (0.0..=len2).contains(&s) {
                keep(t);
            }
        }
    }
    for c in [a, b] {
        let oc = origin - c;
        let qb = oc.dot(dir);
        let disc = qb * qb - (oc.length_squared() - radius * radius);
        if disc >= 0.0 {
            keep(-qb - disc.sqrt());
        }
    }
    best
}
