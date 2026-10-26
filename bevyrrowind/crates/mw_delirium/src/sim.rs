//! A headless Delirium harness: the asset installed, one agent on a floor, behaviour messages, pushes, and the
//! measurements the tests print (stands, push outcome, time to rest).

use delirium::collision::Collision;
use delirium::msg::{self, Value};
use delirium::nm::Agent;
use delirium::prelude::*;
use delirium::ragdoll::{Build, Ragdoll};
use delirium_physics::character::Character;

use crate::Body;

/// Step length, s (60 Hz).
pub const DT: f32 = 1.0 / 60.0;

/// Puts the character's asset in Delirium's global slot (first call per process wins).
pub fn install(character: &Character) {
    let (j, p, m, s) = character.asset();
    delirium::nmrs::set(delirium::nmrs::HOMER, j, p, m, s);
}

/// Which pose the body spawns in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpawnPose {
    /// The skeleton's bind pose (arms down; differs from the zero pose).
    Bind,
    /// The asset's zero pose (arms out, legs straight).
    Zero,
}

/// A body ready for the engine.
pub struct Spawned {
    pub build: Build,
    /// Part frames, sim space: lowest point on z = 0, pelvis over the origin.
    pub frames: Vec<Transform>,
    /// Sim from authoring space (a translation).
    pub sim_from_authoring: Transform,
}

/// Installs the asset and lays the build out in `pose` on the floor. `scale`: Delirium's per-actor size
/// (`Build::scaled`: every segment length times `height`, mass factor `weight`), `None` for the authored size.
///
/// # Panics
/// The character has no build.
pub fn spawn(body: &Body, pose: SpawnPose, scale: Option<(f32, f32)>) -> Spawned {
    install(&body.character);
    let (mut build, mut frames) = body.character.build().expect("the character carries its build");
    if pose == SpawnPose::Zero {
        for (f, z) in frames.iter_mut().zip(&body.zero) {
            f.rotation = z.rotation;
        }
        for c in 1..frames.len() {
            if let Some(p) = build.parts[c].parent {
                let pivot = frames[p].translation + frames[p].rotation * build.parts[c].anchor_parent;
                frames[c].translation = pivot - frames[c].rotation * build.parts[c].anchor;
            }
        }
    }
    if let Some((height, weight)) = scale {
        (build, frames) = build.scaled(&frames, &[height; 21], weight);
    }
    let floor = build.parts.iter().zip(&frames).map(|(p, f)| f.translation.z - p.radius - p.half).fold(f32::MAX, f32::min);
    let shift = frames[0].translation.with_z(floor);
    for f in &mut frames {
        f.translation -= shift;
    }
    Spawned { build, frames, sim_from_authoring: Transform::from_translation(-shift) }
}

/// A flat floor.
pub fn floor_world() -> Collision {
    let mut world = Collision::default();
    world.add_floor(0.0, 0.0, 0.0, 50.0, 0);
    world
}

/// One agent in a world.
pub struct Sim {
    pub build: Build,
    pub world: Collision,
    pub agent: Agent,
}

impl Sim {
    /// The agent at `frames` on the floor. `set_zero`: recalibrate the effector zeros and stance from these frames
    /// (`Agent::set_zero`) instead of the asset's zero pose.
    pub fn new(spawned: &Spawned, seed: f32, set_zero: bool) -> Sim {
        let doll = Ragdoll::new(&spawned.build, &spawned.frames, Vec3::ZERO);
        let mut agent = Agent::new(&spawned.build, doll, Quat::IDENTITY, seed);
        if set_zero {
            agent.set_zero(&spawned.build, &spawned.frames);
        }
        Sim { build: spawned.build.clone(), world: floor_world(), agent }
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

    /// Starts a behaviour with default parameters.
    pub fn start(&mut self, name: &str) {
        self.send(name, &[("start", Value::Bool(true))]);
    }

    pub fn step(&mut self) {
        self.agent.step(&self.build, &self.world, DT, &[], &[]);
    }

    pub fn frames(&self) -> Vec<Transform> {
        self.agent.doll.frames().collect()
    }

    pub fn finite(&self) -> bool {
        self.frames().iter().all(|f| f.translation.is_finite() && f.rotation.is_finite())
    }

    /// Fastest part, m/s.
    pub fn max_speed(&self) -> f32 {
        (0..self.build.parts.len()).map(|k| self.agent.doll.vel(k).0.length()).fold(0.0, f32::max)
    }

    pub fn pelvis_z(&self) -> f32 {
        self.frames()[0].translation.z
    }

    /// Centre of mass and its velocity.
    pub fn com(&self) -> (Vec3, Vec3) {
        self.agent.doll.com()
    }

    /// Lowest point of part `k` above the floor (capsules; boxes by their radius).
    pub fn clearance(&self, k: usize) -> f32 {
        let f = self.frames()[k];
        let p = &self.build.parts[k];
        let axis = f.rotation * Vec3::Y * p.half;
        (f.translation.z - axis.z.abs()) - p.radius
    }

    /// An impulse (N s) at part `part`'s centre.
    pub fn push(&mut self, part: usize, impulse: Vec3) {
        let at = self.frames()[part].translation;
        self.agent.doll.impulse(part, at, impulse);
    }

    /// The body's facing on the floor (from the right hip to the left hip, turned) and its right.
    pub fn facing(&self) -> (Vec3, Vec3) {
        let f = self.frames();
        let left = (f[1].translation - f[4].translation).with_z(0.0).normalize();
        (left.cross(Vec3::Z), -left)
    }
}

/// How a push ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Stayed,
    Stepped,
    Fell,
    Exploded,
}

/// One push case's result.
#[derive(Clone, Copy, Debug)]
pub struct PushResult {
    pub outcome: Outcome,
    /// Time from the push until the centre of mass stays under 0.05 m/s for 0.5 s (and no foot moves), s.
    pub rest: Option<f32>,
    /// Lowest pelvis height over the standing height.
    pub min_ratio: f32,
    /// Foot lifts (a foot leaving its spot by more than 8 cm, counted per new spot).
    pub steps: u32,
}

/// bodyBalance 1 s, a horizontal impulse at the chest (part 10) toward `deg` (0 = forward, 90 = right), then
/// `secs` more.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
pub fn push_case(spawned: &Spawned, set_zero: bool, deg: f32, impulse: f32, secs: f32) -> PushResult {
    let mut s = Sim::new(spawned, 0.3, set_zero);
    s.start("bodyBalance");
    for _ in 0..60 {
        s.step();
    }
    let z0 = s.pelvis_z();
    let f = s.frames();
    let mut feet = [f[3].translation, f[6].translation];
    let (fwd, right) = s.facing();
    let a = deg.to_radians();
    s.push(10, (fwd * a.cos() + right * a.sin()) * impulse);
    let (mut min_ratio, mut still, mut rest, mut steps) = (1.0f32, 0u32, None, 0u32);
    let n = (secs / DT) as u32;
    for k in 0..n {
        s.step();
        if !s.finite() || s.max_speed() > 30.0 {
            return PushResult { outcome: Outcome::Exploded, rest: None, min_ratio: 0.0, steps };
        }
        min_ratio = min_ratio.min(s.pelvis_z() / z0);
        let f = s.frames();
        let mut moved = false;
        for (i, k) in [3usize, 6].into_iter().enumerate() {
            if (f[k].translation - feet[i]).truncate().length() > 0.08 {
                steps += 1;
                feet[i] = f[k].translation;
                moved = true;
            }
        }
        if s.com().1.length() < 0.05 && !moved {
            still += 1;
        } else {
            still = 0;
            rest = None;
        }
        if still == 30 && rest.is_none() {
            rest = Some((k + 1 - 29) as f32 * DT);
        }
    }
    let fell = s.pelvis_z() < 0.5 * z0 || s.agent.fallen;
    let outcome = if fell {
        Outcome::Fell
    } else if steps > 0 {
        Outcome::Stepped
    } else {
        Outcome::Stayed
    };
    PushResult { outcome, rest: if fell { None } else { rest }, min_ratio, steps }
}
