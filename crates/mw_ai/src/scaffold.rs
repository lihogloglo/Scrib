//! `PathWalkScaffold`: the named scaffold that drives NPC locomotion intents
//! from pathgrids and `AI_W` until `sf_ai` (M10) owns the intent
//! (locomotion.md "NPC drivers"; bevyrrowind CLAUDE.md rule 4).
//!
//! Removal path: delete this module and its `sf_controller_bevy` wiring once
//! `sf_ai` writes `LocomotionIntent`; `pathgrid` and `wander` stay as the
//! data `sf_ai`'s adapter reads.

use sf_controller::{LocomotionIntent, PathFollower, PathMode};
use sf_coords::WorldPos;

use crate::pathgrid::PathGraph;
use crate::wander::{WanderPlanner, WanderStep};

/// Extra seconds a walk may take beyond twice its nominal time (a backstop:
/// the progress check below normally fires first).
const WALK_SLACK: f32 = 5.0;
/// Seconds without progress towards the current path node before the actor
/// gives up the walk (`OpenMW` `AiWander` behaviour, observed: an actor that
/// makes no headway gives up its destination, stands, then picks another).
const STUCK_SECONDS: f32 = 3.0;
/// Getting this much closer to the current node (m) counts as progress.
const PROGRESS: f32 = 0.25;
/// A new goal this close to the abandoned one (m) is re-rolled once.
const SAME_GOAL: f32 = 0.5;

#[derive(Clone, Debug)]
enum State {
    Choosing,
    Walking {
        follower: PathFollower,
        left: f32,
        /// Current node index, its closest flat distance so far, and the
        /// seconds since that distance last improved by [`PROGRESS`].
        node: usize,
        best: f32,
        still: f32,
    },
    Idling { variant: u8, left: f32 },
}

/// One frame of scaffold output, for the mover and the animation graph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaffoldOutput {
    pub intent: LocomotionIntent,
    /// Idle group while standing (0 = plain `Idle`, 2..=9 = `IdleN`).
    pub idle_variant: Option<u8>,
    pub walking: bool,
    /// The package's duration ran out (and it does not repeat): the caller
    /// moves on to the next AI package. The actor stands meanwhile.
    pub expired: bool,
}

/// Wander driver for one actor: plans with [`WanderPlanner`], walks with
/// [`PathFollower`].
#[derive(Clone, Debug)]
pub struct PathWalkScaffold {
    pub planner: WanderPlanner,
    /// Walk speed, m/s (see [`crate::wander::walk_speed`]).
    pub walk_speed: f32,
    rng: fastrand::Rng,
    state: State,
    hours_left: f32,
    expired: bool,
    /// The goal of the last walk given up as stuck.
    abandoned: Option<WorldPos>,
    stuck: u32,
}

impl PathWalkScaffold {
    pub fn new(planner: WanderPlanner, walk_speed: f32, seed: u64) -> Self {
        let hours_left = planner.wander.duration_hours;
        Self {
            planner,
            walk_speed,
            rng: fastrand::Rng::with_seed(seed),
            state: State::Choosing,
            hours_left,
            expired: false,
            abandoned: None,
            stuck: 0,
        }
    }

    pub fn expired(&self) -> bool {
        self.expired
    }

    /// Walks given up for lack of progress so far.
    pub fn stuck_count(&self) -> u32 {
        self.stuck
    }

    /// The current walk's follower, if walking.
    pub fn follower(&self) -> Option<&PathFollower> {
        match &self.state {
            State::Walking { follower, .. } => Some(follower),
            _ => None,
        }
    }

    fn standing(&self, variant: Option<u8>) -> ScaffoldOutput {
        ScaffoldOutput {
            intent: LocomotionIntent::IDLE,
            idle_variant: variant,
            walking: false,
            expired: self.expired,
        }
    }

    /// Advances by `dt` real seconds and `game_hours` game hours for an actor
    /// at `pos` in a cell with pathgrid `graph`.
    pub fn update(
        &mut self,
        graph: Option<&PathGraph>,
        pos: WorldPos,
        dt: f32,
        game_hours: f32,
    ) -> ScaffoldOutput {
        if self.planner.wander.duration_hours > 0.0 && !self.expired {
            self.hours_left -= game_hours;
            if self.hours_left <= 0.0 {
                if self.planner.wander.repeat {
                    self.hours_left += self.planner.wander.duration_hours;
                } else {
                    self.expired = true;
                    self.state = State::Choosing;
                }
            }
        }
        if self.expired {
            return self.standing(Some(0));
        }
        // At most two passes: a finished state picks the next step at once.
        for _ in 0..2 {
            match &mut self.state {
                State::Choosing => {
                    let mut step = self.planner.next(graph, pos, &mut self.rng);
                    // Don't head straight back to the goal just given up.
                    if let (Some(gone), WanderStep::Walk { nodes, .. }) = (self.abandoned, &step)
                        && nodes
                            .last()
                            .is_some_and(|n| flat(n.pos, gone) < SAME_GOAL)
                    {
                        step = self.planner.next(graph, pos, &mut self.rng);
                    }
                    self.state = match step {
                        WanderStep::Walk { nodes, .. } => {
                            let mut length = 0.0;
                            let mut prev = pos;
                            for n in &nodes {
                                length += prev.delta_f32(n.pos).length();
                                prev = n.pos;
                            }
                            let left = 2.0 * length / self.walk_speed.max(0.1) + WALK_SLACK;
                            State::Walking {
                                follower: PathFollower::new(nodes, PathMode::Once, self.walk_speed),
                                left,
                                node: usize::MAX,
                                best: f32::MAX,
                                still: 0.0,
                            }
                        }
                        WanderStep::Idle { variant, seconds } => State::Idling {
                            variant,
                            left: seconds,
                        },
                    };
                }
                State::Walking {
                    follower,
                    left,
                    node,
                    best,
                    still,
                } => {
                    *left -= dt;
                    let out = follower.update(pos, dt);
                    if out.finished {
                        self.abandoned = None;
                        self.state = State::Choosing;
                        continue;
                    }
                    // Progress check towards the current node.
                    let t = follower.target();
                    let d = follower.nodes.get(t).map_or(0.0, |n| flat(pos, n.pos));
                    if t != *node || d < *best - PROGRESS {
                        (*node, *best, *still) = (t, d, 0.0);
                    } else {
                        *still += dt;
                    }
                    if *still >= STUCK_SECONDS || *left <= 0.0 {
                        // Give up: stand a moment, then pick another goal.
                        self.stuck += 1;
                        self.abandoned = follower.nodes.last().map(|n| n.pos);
                        self.state = State::Idling {
                            variant: 0,
                            left: 1.0 + self.rng.f32(),
                        };
                        return self.standing(Some(0));
                    }
                    return ScaffoldOutput {
                        intent: out.intent,
                        idle_variant: out.idle_variant,
                        walking: true,
                        expired: false,
                    };
                }
                State::Idling { variant, left } => {
                    *left -= dt;
                    if *left > 0.0 {
                        let v = *variant;
                        return self.standing(Some(v));
                    }
                    self.state = State::Choosing;
                }
            }
        }
        self.standing(Some(0))
    }
}

/// Horizontal distance, m.
fn flat(a: WorldPos, b: WorldPos) -> f32 {
    let d = a.delta_f32(b);
    d.x.hypot(d.z)
}
