//! Path following for NPCs until `sf_ai` owns the intent (M10). The
//! framework follows a polyline; adapters supply the paths (their named
//! scaffold, e.g. `PathWalkScaffold`, turns authored path data into
//! [`PathNode`]s).

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_anim::spring::halflife_to_damping;
use sf_coords::WorldPos;

use crate::intent::{Gait, LocomotionIntent};
use crate::motion::{SpringParams, spring_character};
use crate::mover::MoverParams;

/// How a follower brakes into a stop node (locomotion.md "Arrival").
///
/// The commanded speed follows a constant-deceleration profile,
/// `v = sqrt(2·a·d)`, toward the point where the mover's velocity spring,
/// released to zero, comes to rest: the spring's own stopping distance
/// (`2v/y + a/y²` for a critically damped spring of damping `2y`, 0.39 s
/// of travel at the default half-life) is budgeted, so the walker stops on
/// the node instead of overshooting it. Total stop: that lag plus `v²/2a`.
/// Below `cutoff_speed` the command drops to zero: no creep.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArrivalParams {
    /// Planned deceleration at walking and sneaking gaits, m/s².
    pub walk_deceleration: f32,
    /// Planned deceleration at running and sprinting gaits, m/s².
    pub run_deceleration: f32,
    /// Commanded speed below which the plan cuts to zero, m/s.
    pub cutoff_speed: f32,
}

impl Default for ArrivalParams {
    /// Human-scale stops with the default 0.27 s velocity half-life, from
    /// the first visible slowing: a 1.5 m/s walk stops in 0.6 m (one step)
    /// and 0.9 s, a 3.5 m/s run in 2.4 m and 1.5 s (to < 0.1 m/s).
    fn default() -> Self {
        Self { walk_deceleration: 6.0, run_deceleration: 3.5, cutoff_speed: 0.1 }
    }
}

impl ArrivalParams {
    /// The planned deceleration for `gait`, m/s².
    pub fn deceleration(&self, gait: Gait) -> f32 {
        match gait {
            Gait::Walk | Gait::Sneak => self.walk_deceleration,
            Gait::Run | Gait::Sprint => self.run_deceleration,
        }
    }
}

/// One waypoint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PathNode {
    pub pos: WorldPos,
    /// Seconds to stand here; 0 walks through (corners are cut within the
    /// arrival radius).
    pub wait: f32,
    /// Idle-variant parameter for the graph while waiting here.
    pub idle_variant: u8,
    /// Yaw to face while waiting.
    pub facing: Option<f32>,
}

impl PathNode {
    pub fn at(pos: WorldPos) -> Self {
        Self { pos, wait: 0.0, idle_variant: 0, facing: None }
    }
}

/// What happens at the last node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PathMode {
    /// Stop at the last node.
    #[default]
    Once,
    /// Continue from the last node to the first.
    Loop,
    /// Walk back and forth; the ends are stops.
    PingPong,
}

/// The follower's output for one frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PathOutput {
    pub intent: LocomotionIntent,
    /// The node's idle variant while standing at it.
    pub idle_variant: Option<u8>,
    /// Index of the node being walked to or waited at.
    pub node: usize,
    pub finished: bool,
}

/// Follows a polyline of [`PathNode`]s.
#[derive(Clone, Debug)]
pub struct PathFollower {
    pub nodes: Vec<PathNode>,
    pub mode: PathMode,
    pub gait: Gait,
    /// Cruise speed, m/s.
    pub speed: f32,
    /// Corner-cutting radius at walk-through nodes, meters.
    pub arrival_radius: f32,
    /// How close a stop node must be reached, meters.
    pub stop_radius: f32,
    /// Braking into stop nodes; [`Self::for_mover`] takes the mover's.
    pub arrival: ArrivalParams,
    /// The mover's velocity half-life (its [`SpringParams`]), seconds.
    pub halflife_velocity: f32,
    target: usize,
    backward: bool,
    wait_left: Option<f32>,
    finished: bool,
    /// The plan reached zero: coasting to rest on the spring.
    stopping: bool,
    /// A copy of the mover's velocity spring driven by the commanded speed
    /// (speed m/s, its rate m/s²), to predict where the walker would stop.
    plan_speed: f32,
    plan_accel: f32,
}

impl PathFollower {
    pub fn new(nodes: Vec<PathNode>, mode: PathMode, speed: f32) -> Self {
        Self {
            nodes,
            mode,
            gait: Gait::Walk,
            speed,
            arrival_radius: 0.5,
            stop_radius: 0.15,
            arrival: ArrivalParams::default(),
            halflife_velocity: SpringParams::default().halflife_velocity,
            target: 0,
            backward: false,
            wait_left: None,
            finished: false,
            stopping: false,
            plan_speed: 0.0,
            plan_accel: 0.0,
        }
    }

    /// Brakes like the mover with these parameters (arrival, velocity spring).
    #[must_use]
    pub fn for_mover(mut self, params: &MoverParams) -> Self {
        self.arrival = params.arrival;
        self.halflife_velocity = params.springs.halflife_velocity;
        self
    }

    /// How far the mover's velocity spring still carries the walker if the
    /// command dropped to zero now, meters.
    fn coast_distance(&self) -> f32 {
        let y = halflife_to_damping(self.halflife_velocity) / 2.0;
        (2.0 * self.plan_speed / y + self.plan_accel / (y * y)).max(0.0)
    }

    /// The commanded speed `dist` meters before a stop node.
    fn approach_speed(&mut self, dist: f32) -> f32 {
        if self.stopping {
            return 0.0;
        }
        let room = (dist - self.coast_distance()).max(0.0);
        let v = self.speed.min((2.0 * self.arrival.deceleration(self.gait) * room).sqrt());
        if v < self.arrival.cutoff_speed {
            self.stopping = true;
            return 0.0;
        }
        v
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    /// The node after `i` in the walking direction, and the direction there.
    fn next(&self, i: usize, backward: bool) -> Option<(usize, bool)> {
        let n = self.nodes.len();
        match self.mode {
            PathMode::Once => (i + 1 < n).then_some((i + 1, false)),
            PathMode::Loop => (n > 1).then_some(((i + 1) % n, false)),
            PathMode::PingPong if n < 2 => None,
            PathMode::PingPong => match (backward, i) {
                (false, i) if i + 1 < n => Some((i + 1, false)),
                (true, 0) => Some((1, false)),
                (_, i) => Some((i - 1, true)),
            },
        }
    }

    fn is_stop(&self, i: usize) -> bool {
        let end = match self.mode {
            PathMode::Once => i + 1 == self.nodes.len(),
            PathMode::Loop => false,
            PathMode::PingPong => i == 0 || i + 1 == self.nodes.len(),
        };
        end || self.nodes[i].wait > 0.0
    }

    fn advance(&mut self) {
        self.stopping = false;
        match self.next(self.target, self.backward) {
            Some((i, b)) => (self.target, self.backward) = (i, b),
            None => self.finished = true,
        }
    }

    fn idle(&self, node: Option<&PathNode>) -> PathOutput {
        PathOutput {
            intent: LocomotionIntent { desired_facing: node.and_then(|n| n.facing), gait: self.gait, ..LocomotionIntent::IDLE },
            idle_variant: node.map(|n| n.idle_variant),
            node: self.target,
            finished: self.finished,
        }
    }

    /// The intent for a character at `pos` this frame.
    pub fn update(&mut self, pos: WorldPos, dt: f32) -> PathOutput {
        let out = self.plan(pos, dt);
        // The mover springs toward this speed over the same `dt`.
        let (mut x, mut v, mut a) = (Vec3::ZERO, Vec3::X * self.plan_speed, Vec3::X * self.plan_accel);
        let goal = Vec3::X * out.intent.desired_velocity.length();
        spring_character(&mut x, &mut v, &mut a, goal, self.halflife_velocity, dt);
        (self.plan_speed, self.plan_accel) = (v.x, a.x);
        out
    }

    fn plan(&mut self, pos: WorldPos, dt: f32) -> PathOutput {
        if self.nodes.is_empty() {
            self.finished = true;
        }
        if self.finished {
            return self.idle(self.nodes.get(self.target));
        }
        if let Some(left) = self.wait_left {
            let left = left - dt;
            if left > 0.0 {
                self.wait_left = Some(left);
                return self.idle(Some(&self.nodes[self.target]));
            }
            self.wait_left = None;
            let at = self.target;
            self.advance();
            if self.finished {
                return self.idle(Some(&self.nodes[at]));
            }
        }
        // Walk-through nodes reached within the radius hand over to the next.
        for _ in 0..self.nodes.len() {
            let node = self.nodes[self.target];
            let dist = flat(pos.delta_f32(node.pos)).length();
            if self.is_stop(self.target) {
                // A committed stop arrives once the spring has come to rest.
                let arrived = if self.stopping { self.plan_speed <= self.arrival.cutoff_speed } else { dist <= self.stop_radius };
                if arrived {
                    self.stopping = false;
                    if node.wait > 0.0 {
                        self.wait_left = Some(node.wait);
                    } else {
                        self.advance();
                        if !self.finished {
                            continue;
                        }
                    }
                    return self.idle(Some(&node));
                }
            } else if dist <= self.arrival_radius {
                self.advance();
                continue;
            }
            break;
        }
        let node = self.nodes[self.target];
        let to = flat(pos.delta_f32(node.pos));
        let dist = to.length();
        let speed = if self.is_stop(self.target) { self.approach_speed(dist) } else { self.speed };
        PathOutput {
            intent: LocomotionIntent { desired_velocity: to / dist.max(1e-6) * speed, desired_facing: None, gait: self.gait },
            idle_variant: None,
            node: self.target,
            finished: false,
        }
    }
}

fn flat(v: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z)
}
