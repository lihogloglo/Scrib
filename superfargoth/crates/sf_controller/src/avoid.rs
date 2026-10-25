//! Local avoidance between characters: ORCA, optimal reciprocal collision
//! avoidance (van den Berg, Guy, Lin, Manocha, "Reciprocal n-Body Collision
//! Avoidance", ISRR 2009 / Springer STAR 70, 2011), written from the paper.
//!
//! Each agent's preferred velocity (the intent's `desired_velocity`) becomes
//! the closest velocity that keeps it collision-free with its neighbours for
//! `time_horizon` seconds, assuming they do the same. Every neighbour adds a
//! half-plane of allowed velocities; a 2D incremental linear program (the
//! paper's section 5) picks the velocity, and when the half-planes leave no
//! room, a 3D program minimizes the largest violation. Runs on the XZ plane,
//! before the mover's springs (locomotion.md "Local avoidance").
//!
//! Additions on top of the paper, each a standard practice:
//! - **Responsibility** (RVO2 2.0's weights, UE RVO avoidance weights): the
//!   share of a pair's avoidance each side takes. A
//!   [`AvoidRole::NonReactive`] agent (the player) takes none, so others go
//!   round it. A yielding agent takes [`AvoidParams::yield_weight`] against
//!   the other's 1: an idle agent (preferred speed under `idle_speed`)
//!   against a moving one, so standing characters step aside for walkers,
//!   and the lower [`AvoidAgent::priority`] of two agents walking against
//!   each other. The share applies to a required change; the slack of a
//!   pair off a collision course is split the other way round (see
//!   `orca_line`).
//! - **Keep-right bias and priority**: an agent with an oncoming neighbour
//!   turns its preferred velocity slightly right, so opposing streams form
//!   lanes, and of an opposing pair the lower priority yields. Both break
//!   the symmetric deadlock ORCA is known for in corridors and doorways.
//!
//! Neighbours come from a per-frame spatial hash of the agents (cells of
//! half `neighbour_dist`, searched ring by ring), closest `max_neighbours` first. The result is
//! deterministic and independent of batch order: [`Avoidance::solve_one`]
//! (a brute-force scan) equals [`Avoidance::solve`] bit for bit.

use glam::{Vec2, Vec3};
use sf_coords::WorldPos;

const EPS: f32 = 1e-5;

/// How an agent takes part in avoidance.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvoidRole {
    /// Avoids others and is avoided.
    #[default]
    Reactive,
    /// Is avoided but avoids nobody (the player): its velocity is its own.
    NonReactive,
}

/// One character's input to avoidance.
#[derive(Clone, Copy, Debug)]
pub struct AvoidAgent {
    /// Feet position.
    pub position: WorldPos,
    /// Current velocity (`MotionState::velocity`): what the others see.
    pub velocity: Vec3,
    /// The velocity it would like (`LocomotionIntent::desired_velocity`).
    pub preferred: Vec3,
    /// Capsule radius.
    pub radius: f32,
    /// Fastest horizontal speed avoidance may ask for (at least the preferred speed).
    pub max_speed: f32,
    pub role: AvoidRole,
    /// Of two characters walking against each other, the lower priority
    /// yields (takes `yield_weight`); equal priorities go by a heading rule.
    /// Use coarse ranks (escort, guard, citizen; 0 by default), not per-
    /// character values: mixed priorities within a stream jam doorways.
    pub priority: f32,
    /// Direction of travel along its route (the path segment's), deciding who
    /// is oncoming and who has right of way; steering toward a waypoint
    /// (a doorway) cuts across the flow. Zero falls back to `preferred`.
    pub travel: Vec3,
}

/// Tuning, shared by all agents of a batch.
#[derive(Clone, Copy, Debug)]
pub struct AvoidParams {
    /// Seconds ahead an agent stays collision-free (ORCA's τ).
    pub time_horizon: f32,
    /// Neighbours farther than this (horizontal) are ignored; grid cells are half of it.
    pub neighbour_dist: f32,
    /// Closest neighbours considered per agent.
    pub max_neighbours: usize,
    /// Characters this far apart vertically never interact (as `separate`).
    pub vertical_gap: f32,
    /// A yielding agent's responsibility against the other's 1: an idle agent
    /// against a moving one, the lower `priority` of an opposing pair.
    pub yield_weight: f32,
    /// Preferred speeds under this are idle.
    pub idle_speed: f32,
    /// Right turn (radians) of the preferred velocity with an oncoming neighbour.
    pub side_bias: f32,
    /// Extra clearance added to every pair's combined radius.
    pub margin: f32,
}

impl Default for AvoidParams {
    fn default() -> Self {
        Self {
            time_horizon: 2.0,
            neighbour_dist: 5.0,
            max_neighbours: 8,
            vertical_gap: 1.5,
            yield_weight: 9.0,
            idle_speed: 0.1,
            side_bias: 0.15,
            margin: 0.05,
        }
    }
}

/// A half-plane of allowed velocities: the left side of `direction` through `point`.
#[derive(Clone, Copy, Debug)]
struct Line {
    point: Vec2,
    direction: Vec2,
}

/// Reusable buffers; one per thread.
#[derive(Default)]
pub struct Avoidance {
    local: Vec<Vec3>,
    cell: Vec<(i32, i32)>,
    bucket_start: Vec<u32>,
    sorted: Vec<u32>,
    neighbours: Vec<(f32, u32)>,
    lines: Vec<Line>,
    projected: Vec<Line>,
}

impl Avoidance {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Avoiding velocities for every agent, written to `out` (same length):
    /// horizontal from ORCA, vertical kept from `preferred`.
    ///
    /// # Panics
    /// If `out` and `agents` differ in length.
    pub fn solve(&mut self, agents: &[AvoidAgent], params: &AvoidParams, dt: f32, out: &mut [Vec3]) {
        assert_eq!(agents.len(), out.len(), "one output per agent");
        let Some(first) = agents.first() else { return };
        // Cells of half the neighbour distance, searched ring by ring (rings
        // 0..=2 cover it); a ring stops the search once the k-th closest so
        // far is nearer than anything beyond it can be. Crowds find their
        // neighbours in the first ring, sparse agents scan at most 5 × 5 cells.
        let cell_size = params.neighbour_dist * 0.5;
        self.build_grid(agents, first.position, cell_size);
        let n_buckets = self.bucket_start.len() - 1;
        let k_max = params.max_neighbours;
        for (i, slot) in out.iter_mut().enumerate() {
            self.neighbours.clear();
            let (cx, cz) = self.cell[i];
            for ring in 0..=2i32 {
                for dz in -ring..=ring {
                    for dx in -ring..=ring {
                        if dx.abs().max(dz.abs()) != ring {
                            continue;
                        }
                        let c = (cx + dx, cz + dz);
                        let b = bucket(c, n_buckets);
                        for k in self.bucket_start[b]..self.bucket_start[b + 1] {
                            let j = self.sorted[k as usize];
                            if self.cell[j as usize] == c {
                                consider(&self.local, i, j as usize, params, &mut self.neighbours);
                            }
                        }
                    }
                }
                if k_max > 0 && self.neighbours.len() >= k_max {
                    keep_closest(&mut self.neighbours, k_max);
                    #[allow(clippy::cast_precision_loss)]
                    let reach = ring as f32 * cell_size;
                    if self.neighbours[k_max - 1].0 < reach * reach {
                        break;
                    }
                }
            }
            *slot = self.agent_velocity(agents, i, params, dt);
        }
    }

    /// One agent's avoiding velocity with a brute-force neighbour scan;
    /// equals its entry from [`Self::solve`].
    pub fn solve_one(&mut self, agents: &[AvoidAgent], i: usize, params: &AvoidParams, dt: f32) -> Vec3 {
        let Some(first) = agents.first() else { return Vec3::ZERO };
        self.local.clear();
        self.local.extend(agents.iter().map(|a| first.position.delta_f32(a.position)));
        self.neighbours.clear();
        for j in 0..agents.len() {
            consider(&self.local, i, j, params, &mut self.neighbours);
        }
        self.agent_velocity(agents, i, params, dt)
    }

    /// Spatial hash (Teschner et al. 2003) by counting sort: agents' cells
    /// hashed into a power-of-two table at least twice the agent count.
    fn build_grid(&mut self, agents: &[AvoidAgent], anchor: WorldPos, cell_size: f32) {
        let inv = 1.0 / cell_size.max(EPS);
        self.local.clear();
        self.cell.clear();
        for a in agents {
            let l = anchor.delta_f32(a.position);
            self.local.push(l);
            #[allow(clippy::cast_possible_truncation)]
            self.cell.push(((l.x * inv).floor() as i32, (l.z * inv).floor() as i32));
        }
        let n_buckets = (agents.len() * 2).next_power_of_two();
        self.bucket_start.clear();
        self.bucket_start.resize(n_buckets + 1, 0);
        for &c in &self.cell {
            self.bucket_start[bucket(c, n_buckets) + 1] += 1;
        }
        for b in 0..n_buckets {
            self.bucket_start[b + 1] += self.bucket_start[b];
        }
        self.sorted.clear();
        self.sorted.resize(agents.len(), 0);
        let mut fill = self.bucket_start.clone();
        for (i, &c) in self.cell.iter().enumerate() {
            let b = bucket(c, n_buckets);
            #[allow(clippy::cast_possible_truncation)]
            {
                self.sorted[fill[b] as usize] = i as u32;
            }
            fill[b] += 1;
        }
    }

    /// The ORCA velocity for agent `i` from `self.neighbours` (unsorted candidates).
    fn agent_velocity(&mut self, agents: &[AvoidAgent], i: usize, params: &AvoidParams, dt: f32) -> Vec3 {
        let a = &agents[i];
        let preferred = flat(a.preferred);
        let keep_y = |v: Vec2| Vec3::new(v.x, a.preferred.y, v.y);
        if a.role == AvoidRole::NonReactive {
            return a.preferred;
        }
        // The closest `max_neighbours` candidates, ties by index, go first: the
        // incremental program is order-dependent, and a failure falls back
        // from the failing line on. Selection then a sort of the few kept.
        keep_closest(&mut self.neighbours, params.max_neighbours);
        self.neighbours.sort_unstable_by(by_distance);

        let idle_sq = params.idle_speed * params.idle_speed;
        let idle = preferred.length_squared() < idle_sq;
        let travel = travel_dir(a);
        let velocity = flat(a.velocity);
        let tau = params.time_horizon.max(EPS);
        let mut oncoming = false;
        self.lines.clear();
        for &(_, j) in &self.neighbours {
            let b = &agents[j as usize];
            let d = a.position.delta_f32(b.position);
            let mut rel_pos = Vec2::new(d.x, d.z);
            if rel_pos == Vec2::ZERO {
                // Coincident: split along X by index, opposite ways for the two (as `separate`).
                rel_pos.x = if i < j as usize { 1e-4 } else { -1e-4 };
            }
            let other_pref = flat(b.preferred);
            let other_idle = other_pref.length_squared() < idle_sq;
            let other_travel = travel_dir(b);
            let opposing = !idle && !other_idle && other_travel.dot(travel) < 0.0;
            oncoming |= opposing && rel_pos.dot(travel) > 0.0;
            // Responsibilities: 1 each, a yielding side `yield_weight` (idle
            // against moving, or the lower priority of an opposing pair).
            let (mut mine, mut theirs) = (1.0, 1.0);
            if idle != other_idle {
                if idle { mine = params.yield_weight } else { theirs = params.yield_weight }
            } else if opposing {
                let yields = match a.priority.total_cmp(&b.priority) {
                    std::cmp::Ordering::Equal => !right_of_way(travel - other_travel),
                    order => order.is_lt(),
                };
                if yields { mine = params.yield_weight } else { theirs = params.yield_weight }
            }
            let shares = if b.role == AvoidRole::NonReactive {
                (1.0, 1.0)
            } else {
                (mine / (mine + theirs), theirs / (mine + theirs))
            };
            let combined = a.radius + b.radius + params.margin;
            self.lines.push(orca_line(rel_pos, velocity - flat(b.velocity), velocity, combined, tau, dt, shares));
        }

        let mut goal = preferred;
        if oncoming && params.side_bias != 0.0 {
            // Seen from above (Y up), (x, z) is a left-handed plane: a positive
            // angle turns right (yaw 0 faces +Z, its right is -X).
            goal = Vec2::from_angle(params.side_bias).rotate(goal);
        }
        let max_speed = a.max_speed.max(preferred.length());
        let mut result = Vec2::ZERO;
        let failed = linear_program2(&self.lines, max_speed, goal, false, &mut result);
        if failed < self.lines.len() {
            linear_program3(&self.lines, failed, max_speed, &mut result, &mut self.projected);
        }
        // Near-parallel lines in the 3D program can overshoot the disc by float error.
        keep_y(result.clamp_length_max(max_speed))
    }
}

/// Adds agent `j` to `i`'s candidates when it is a different agent within reach.
/// Distances come from positions relative to the batch's first agent, the
/// same in [`Avoidance::solve`] and [`Avoidance::solve_one`].
fn consider(local: &[Vec3], i: usize, j: usize, params: &AvoidParams, out: &mut Vec<(f32, u32)>) {
    if i == j {
        return;
    }
    let d = local[j] - local[i];
    let dist_sq = d.x * d.x + d.z * d.z;
    if d.y.abs() <= params.vertical_gap && dist_sq < params.neighbour_dist * params.neighbour_dist {
        #[allow(clippy::cast_possible_truncation)]
        out.push((dist_sq, j as u32));
    }
}

/// Right of way between two opposing agents of equal priority, from the
/// difference `d` of their headings (mine minus theirs): the one heading
/// toward +X wins when the pair runs mostly along X, else toward +Z. A
/// fixed convention, like a traffic rule, so a whole stream yields to the
/// opposing one together; `d` negates exactly for the other side, so the
/// pair always agrees. Per-character tie-breaks (an id hash) mix yielding
/// and non-yielding agents in both streams and jam doorways (measured).
fn right_of_way(d: Vec2) -> bool {
    if d.x.abs() >= d.y.abs() { d.x > 0.0 } else { d.y > 0.0 }
}

#[allow(clippy::cast_sign_loss)]
fn bucket((x, z): (i32, i32), n_buckets: usize) -> usize {
    let h = (x as u32).wrapping_mul(73_856_093) ^ (z as u32).wrapping_mul(19_349_663);
    h as usize & (n_buckets - 1)
}

fn flat(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.z)
}

/// `a × b`, the 2D cross product.
fn det(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

/// The ORCA half-plane for one neighbour (paper section 4): the smallest
/// change `u` of the relative velocity that leaves the truncated velocity
/// obstacle; allowed velocities are on the side `u` points to. `shares` is
/// (this agent's part of a required change, its part of the slack when the
/// pair is already off a collision course). The paper and RVO2 use one
/// share for both, so a low-responsibility agent also gets little slack and
/// crawls toward a standing one; the neighbour's parts are the complements,
/// so each pair still sums to `u` and the reciprocal guarantee holds.
/// Overlapping agents use a horizon of one frame so they separate at once.
fn orca_line(rel_pos: Vec2, rel_vel: Vec2, velocity: Vec2, radius: f32, tau: f32, dt: f32, shares: (f32, f32)) -> Line {
    let dist_sq = rel_pos.length_squared();
    let r_sq = radius * radius;
    let (direction, u);
    if dist_sq > r_sq {
        // w: from the cut-off disc's center (rel_pos / τ) to the relative velocity.
        let w = rel_vel - rel_pos / tau;
        let w_len_sq = w.length_squared();
        let along = w.dot(rel_pos);
        if along < 0.0 && along * along > r_sq * w_len_sq {
            // Closest boundary is the cut-off disc (radius / τ).
            let w_len = w_len_sq.sqrt();
            let unit = w / w_len;
            direction = Vec2::new(unit.y, -unit.x);
            u = unit * (radius / tau - w_len);
        } else {
            // Closest boundary is a leg of the cone, tangent to the disc.
            let leg = (dist_sq - r_sq).sqrt();
            direction = if det(rel_pos, w) > 0.0 {
                Vec2::new(rel_pos.x * leg - rel_pos.y * radius, rel_pos.x * radius + rel_pos.y * leg) / dist_sq
            } else {
                -Vec2::new(rel_pos.x * leg + rel_pos.y * radius, -rel_pos.x * radius + rel_pos.y * leg) / dist_sq
            };
            u = direction * rel_vel.dot(direction) - rel_vel;
        }
    } else {
        // Already overlapping: leave the disc of radius / dt within one frame.
        let inv_dt = 1.0 / dt.max(EPS);
        let w = rel_vel - rel_pos * inv_dt;
        let w_len = w.length();
        let unit = if w_len > EPS { w / w_len } else { rel_pos.try_normalize().map_or(Vec2::X, |d| -d) };
        direction = Vec2::new(unit.y, -unit.x);
        u = unit * (radius * inv_dt - w_len);
    }
    // The current velocity breaks the line exactly when a change is required.
    let share = if dist_sq <= r_sq || det(direction, u) > 0.0 { shares.0 } else { shares.1 };
    Line { point: velocity + u * share, direction }
}

/// Optimizes along line `n` within the speed disc and the earlier lines'
/// half-planes (paper section 5, the 1D step). `false` when infeasible.
fn linear_program1(lines: &[Line], n: usize, radius: f32, goal: Vec2, along_goal: bool, result: &mut Vec2) -> bool {
    let line = lines[n];
    let dot = line.point.dot(line.direction);
    let disc = dot * dot + radius * radius - line.point.length_squared();
    if disc < 0.0 {
        return false; // The line misses the speed disc.
    }
    let s = disc.sqrt();
    let (mut t_left, mut t_right) = (-dot - s, -dot + s);
    for other in &lines[..n] {
        let denom = det(line.direction, other.direction);
        let numer = det(other.direction, line.point - other.point);
        if denom.abs() <= EPS {
            if numer < 0.0 {
                return false; // Parallel and outside.
            }
            continue;
        }
        let t = numer / denom;
        if denom >= 0.0 {
            t_right = t_right.min(t);
        } else {
            t_left = t_left.max(t);
        }
        if t_left > t_right {
            return false;
        }
    }
    let t = if along_goal {
        if goal.dot(line.direction) > 0.0 { t_right } else { t_left }
    } else {
        line.direction.dot(goal - line.point).clamp(t_left, t_right)
    };
    *result = line.point + line.direction * t;
    true
}

/// The 2D program: the velocity closest to `goal` (or farthest along it when
/// `along_goal`, `goal` a unit vector) in the speed disc and every half-plane.
/// Returns the index of the first line it couldn't satisfy, or `lines.len()`.
fn linear_program2(lines: &[Line], radius: f32, goal: Vec2, along_goal: bool, result: &mut Vec2) -> usize {
    *result = if along_goal {
        goal * radius
    } else if goal.length_squared() > radius * radius {
        goal.normalize_or_zero() * radius
    } else {
        goal
    };
    for i in 0..lines.len() {
        if det(lines[i].direction, lines[i].point - *result) > 0.0 {
            let before = *result;
            if !linear_program1(lines, i, radius, goal, along_goal, result) {
                *result = before;
                return i;
            }
        }
    }
    lines.len()
}

/// The 3D program for crowded cases (paper section 5.2): from line `begin`
/// on, minimize the largest distance a velocity lies outside any half-plane,
/// by a 2D program on the bisectors of each violated line with the earlier ones.
fn linear_program3(lines: &[Line], begin: usize, radius: f32, result: &mut Vec2, projected: &mut Vec<Line>) {
    let mut distance = 0.0;
    for i in begin..lines.len() {
        let li = lines[i];
        if det(li.direction, li.point - *result) <= distance {
            continue;
        }
        projected.clear();
        for lj in &lines[..i] {
            let d = det(li.direction, lj.direction);
            let point = if d.abs() <= EPS {
                if li.direction.dot(lj.direction) > 0.0 {
                    continue; // Same direction: lj is implied.
                }
                (li.point + lj.point) * 0.5
            } else {
                li.point + li.direction * (det(lj.direction, li.point - lj.point) / d)
            };
            projected.push(Line { point, direction: (lj.direction - li.direction).normalize_or_zero() });
        }
        let before = *result;
        let toward = Vec2::new(-li.direction.y, li.direction.x);
        if linear_program2(projected, radius, toward, true, result) < projected.len() {
            // Only float error can get here; keep the previous result.
            *result = before;
        }
        distance = det(li.direction, li.point - *result);
    }
}

/// The unit direction of travel: `travel`, else `preferred`, flattened.
fn travel_dir(a: &AvoidAgent) -> Vec2 {
    let t = flat(a.travel);
    if t.length_squared() > EPS { t.normalize() } else { flat(a.preferred).normalize_or_zero() }
}

/// Candidates by distance, ties by index: a total order, so the kept set
/// doesn't depend on the scan order.
#[allow(clippy::trivially_copy_pass_by_ref)] // The slice comparator signature.
fn by_distance(x: &(f32, u32), y: &(f32, u32)) -> std::cmp::Ordering {
    x.0.total_cmp(&y.0).then(x.1.cmp(&y.1))
}

/// Keeps the `k` closest candidates (unsorted, the k-th closest last).
fn keep_closest(candidates: &mut Vec<(f32, u32)>, k: usize) {
    if k == 0 {
        candidates.clear();
    } else if k < candidates.len() {
        candidates.select_nth_unstable_by(k - 1, by_distance);
        candidates.truncate(k);
    } else if k == candidates.len() {
        // Put the k-th closest last for the ring test.
        let last = (0..k).max_by(|&a, &b| by_distance(&candidates[a], &candidates[b])).unwrap_or(0);
        candidates.swap(last, k - 1);
    }
}
