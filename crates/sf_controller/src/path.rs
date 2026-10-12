//! Path following for NPCs until `sf_ai` owns the intent (M10). The
//! framework follows a polyline; adapters supply the paths (their named
//! scaffold, e.g. `PathWalkScaffold`, turns authored path data into
//! [`PathNode`]s).

use glam::Vec3;
use sf_coords::WorldPos;

use crate::intent::{Gait, LocomotionIntent};

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
    /// Speed per meter of remaining distance when approaching a stop (1/s).
    /// The default suits the default velocity half-life: approaching more
    /// steeply than the springs can follow would overshoot.
    pub braking: f32,
    /// Slowest approach speed, so the last centimeters don't take forever.
    pub min_speed: f32,
    target: usize,
    backward: bool,
    wait_left: Option<f32>,
    finished: bool,
}

impl PathFollower {
    pub fn new(nodes: Vec<PathNode>, mode: PathMode, speed: f32) -> Self {
        let halflife = crate::motion::SpringParams::default().halflife_velocity;
        Self {
            nodes,
            mode,
            gait: Gait::Walk,
            speed,
            arrival_radius: 0.5,
            stop_radius: 0.15,
            braking: Self::braking_for(halflife),
            min_speed: 0.25,
            target: 0,
            backward: false,
            wait_left: None,
            finished: false,
        }
    }

    /// A braking gain that a velocity spring of this half-life follows
    /// without overshoot (the closed loop's poles stay real).
    pub fn braking_for(halflife_velocity: f32) -> f32 {
        std::f32::consts::LN_2 / (4.0 * halflife_velocity.max(1e-3))
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
                if dist <= self.stop_radius {
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
        let speed = if self.is_stop(self.target) { self.speed.min((dist * self.braking).max(self.min_speed)) } else { self.speed };
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
