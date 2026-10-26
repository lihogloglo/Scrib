//! A takeover (ROADMAP 3.2b slice 2a): Delirium drives an animated character from its current pose, then says when
//! to hand it back to the animation: balanced and still, or down on the floor (the host plays the get-up).
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
    /// Still: centre of mass slower than this, m/s ...
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
        }
    }
}

/// Why the takeover ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandBack {
    /// Balanced and still: back to the graph where it stands.
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
    still: f32,
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
    /// `None` for the authored size. `floor`: the ground under the feet (sim z); a body already sunk into it lowers
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
            still: 0.0,
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

    /// An impulse (N s, sim space) on part `part` at `at` (sim space). Restarts the still timer.
    pub fn push(&mut self, part: usize, at: Vec3, impulse: Vec3) {
        self.agent.doll.impulse(part, at, impulse);
        self.still = 0.0;
    }

    pub fn frames(&self) -> Vec<Transform> {
        self.agent.doll.frames().collect()
    }

    /// Centre of mass and its velocity.
    pub fn com(&self) -> (Vec3, Vec3) {
        self.agent.doll.com()
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
        let speed = self.com().1.length();
        self.still = if speed < self.rules.still_speed { self.still + dt } else { 0.0 };
        let still = self.still >= self.rules.still_secs;
        if self.down.is_none() && (self.agent.fallen || frames[0].translation.z - self.floor < self.rules.fallen_ratio * self.stand) {
            self.down = Some(0.0);
        }
        match &mut self.down {
            Some(down) => {
                *down += dt;
                let (min, max) = self.rules.down_secs;
                ((still && *down >= min) || *down >= max).then_some(HandBack::Fallen)
            }
            None if still && self.time >= self.rules.min_secs => Some(HandBack::Balanced),
            None if self.time >= self.rules.max_secs => Some(HandBack::Timeout),
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
