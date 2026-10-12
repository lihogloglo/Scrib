//! Local avoidance (ORCA) on the real mover: head-on passes, a non-reactive
//! player, give-way, a doorway swap, batched = single, and timing.
#![allow(clippy::cast_sign_loss)]

use glam::Vec2;

use super::*;

const WALK: f32 = 1.5;

/// A tiny deterministic generator (xorshift32) for scattered agents.
struct Rng(u32);

impl Rng {
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 >> 8) as f32 / (1u32 << 24) as f32
    }
    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.next()
    }
}

/// Characters walking waypoints on the kinematic mover, with avoidance
/// between the intent and the mover (the order the plugin will run).
struct Crowd {
    world: World,
    locos: Vec<Locomotion>,
    roles: Vec<AvoidRole>,
    /// Remaining waypoints per character, the next first.
    routes: Vec<Vec<Vec3>>,
    /// Direction from start to goal: the route's flow.
    travel: Vec<Vec3>,
    /// A waypoint also counts as passed once the character is past it along this axis (doorways).
    pass_axis: Vec<Option<Vec3>>,
    params: MoverParams,
    avoid_params: AvoidParams,
    avoiding: bool,
    /// Each character's priority by index (default all equal).
    priority_of: fn(usize) -> f32,
    mover: KinematicMover,
    avoidance: Avoidance,
    time: f32,
    /// Smallest gap between two capsules seen (center distance minus radii).
    min_gap: f32,
    /// Last step's avoiding velocities.
    desired: Vec<Vec3>,
}

impl Crowd {
    fn new(world: World) -> Self {
        Self {
            world,
            locos: Vec::new(),
            roles: Vec::new(),
            routes: Vec::new(),
            travel: Vec::new(),
            pass_axis: Vec::new(),
            params: MoverParams::default(),
            avoid_params: AvoidParams::default(),
            avoiding: true,
            priority_of: |_| 0.0,
            mover: KinematicMover::new(),
            avoidance: Avoidance::new(),
            time: 0.0,
            min_gap: f32::INFINITY,
            desired: Vec::new(),
        }
    }

    fn add(&mut self, start: Vec3, route: &[Vec3], role: AvoidRole) -> usize {
        let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(start), 0.0);
        loco.grounded = self.world.probe_one(ProbeQuery::down(WorldPos::ORIGIN.offset(start + Vec3::Y), 2.0));
        self.locos.push(loco);
        self.roles.push(role);
        self.routes.push(route.to_vec());
        self.travel.push(route.last().map_or(Vec3::ZERO, |&g| g - start));
        self.pass_axis.push(None);
        self.locos.len() - 1
    }

    fn position(&self, i: usize) -> Vec3 {
        xyz(self.locos[i].position)
    }

    fn arrived(&self, i: usize) -> bool {
        self.routes[i].len() <= 1 && self.routes[i].last().is_none_or(|g| horizontal_len(*g - self.position(i)) < 0.3)
    }

    /// Preferred velocity: walk to the next waypoint, brake into the last.
    fn preferred(&mut self, i: usize) -> Vec3 {
        let here = self.position(i);
        while self.routes[i].len() > 1 {
            let to = self.routes[i][0] - here;
            // A doorway waypoint is passed by crossing it, a plain one by reaching it.
            let passed = match self.pass_axis[i] {
                Some(axis) => to.dot(axis) < -0.4,
                None => horizontal_len(to) < 0.5,
            };
            if passed {
                self.routes[i].remove(0);
            } else {
                break;
            }
        }
        let Some(&target) = self.routes[i].first() else { return Vec3::ZERO };
        let to = Vec3::new(target.x - here.x, 0.0, target.z - here.z);
        let dist = to.length();
        let speed = if self.routes[i].len() == 1 { WALK.min(dist * 1.5) } else { WALK };
        if dist < 0.05 { Vec3::ZERO } else { to / dist * speed }
    }

    fn step(&mut self) {
        let preferred: Vec<Vec3> = (0..self.locos.len()).map(|i| self.preferred(i)).collect();
        let mut desired = preferred.clone();
        if self.avoiding {
            let agents: Vec<AvoidAgent> = self
                .locos
                .iter()
                .zip(&preferred)
                .zip(&self.roles)
                .enumerate()
                .map(|(i, ((loco, &preferred), &role))| AvoidAgent {
                    position: loco.position,
                    velocity: loco.motion.velocity,
                    preferred,
                    radius: self.params.radius,
                    max_speed: WALK,
                    role,
                    priority: (self.priority_of)(i),
                    travel: self.travel[i],
                })
                .collect();
            self.avoidance.solve(&agents, &self.avoid_params, DT, &mut desired);
        }
        let params = self.params;
        let mut items: Vec<MoveInput<'_>> = self
            .locos
            .iter_mut()
            .zip(&desired)
            .map(|(loco, &v)| MoveInput { loco, intent: toward(v), params: &params, root: RootMotion::default() })
            .collect();
        self.mover.step_batch(&mut items, &self.world, DT);
        self.desired = desired;
        self.time += DT;
        for i in 0..self.locos.len() {
            for j in i + 1..self.locos.len() {
                let gap = horizontal_len(self.position(j) - self.position(i)) - 2.0 * self.params.radius;
                self.min_gap = self.min_gap.min(gap);
            }
        }
    }

    /// Steps until everyone arrived or `limit` seconds passed; returns the time taken.
    fn run(&mut self, limit: f32) -> Option<f32> {
        while self.time < limit {
            self.step();
            if (0..self.locos.len()).all(|i| self.arrived(i)) {
                return Some(self.time);
            }
        }
        None
    }
}

fn horizontal_len(v: Vec3) -> f32 {
    Vec2::new(v.x, v.z).length()
}

#[test]
fn head_on_agents_pass_without_contact() {
    let mut crowd = Crowd::new(World::new());
    let (a, b) = (Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 5.0));
    crowd.add(a, &[b], AvoidRole::Reactive);
    crowd.add(b, &[a], AvoidRole::Reactive);
    let time = crowd.run(20.0);
    println!("head-on: arrived {time:?} s, min gap {:.3} m", crowd.min_gap);
    assert!(time.is_some_and(|t| t < 12.0), "{time:?}");
    assert!(crowd.min_gap > 0.0, "contact: gap {}", crowd.min_gap);

    // The same without avoidance collides (the test is not trivially passing).
    let mut blind = Crowd::new(World::new());
    blind.avoiding = false;
    blind.add(a, &[b], AvoidRole::Reactive);
    blind.add(b, &[a], AvoidRole::Reactive);
    blind.run(20.0);
    assert!(blind.min_gap < -0.5, "{}", blind.min_gap);
}

#[test]
fn head_on_at_an_angle_and_offset_passes() {
    for (offset, angle) in [(0.1f32, 0.0f32), (-0.3, 0.0), (0.0, 0.5), (0.2, -1.2)] {
        let mut crowd = Crowd::new(World::new());
        let dir = Vec3::new(angle.sin(), 0.0, angle.cos());
        let side = Vec3::new(dir.z, 0.0, -dir.x) * offset;
        let (a, b) = (-dir * 5.0, dir * 5.0 + side);
        crowd.add(a, &[dir * 5.0], AvoidRole::Reactive);
        crowd.add(b, &[a + side], AvoidRole::Reactive);
        let time = crowd.run(20.0);
        assert!(time.is_some() && crowd.min_gap > 0.0, "offset {offset} angle {angle}: {time:?}, gap {}", crowd.min_gap);
    }
}

#[test]
fn player_is_avoided_but_does_not_avoid() {
    let mut crowd = Crowd::new(World::new());
    let (a, b) = (Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 5.0));
    let player = crowd.add(a, &[b], AvoidRole::NonReactive);
    crowd.add(b, &[a], AvoidRole::Reactive);
    let mut player_drift = 0.0f32;
    while crowd.time < 12.0 {
        crowd.step();
        player_drift = player_drift.max(crowd.position(player).x.abs());
    }
    println!("player head-on: player drift {player_drift:.4} m, min gap {:.3} m", crowd.min_gap);
    assert!(player_drift < 1e-4, "the player kept its line: {player_drift}");
    assert!(crowd.min_gap > 0.0, "contact: gap {}", crowd.min_gap);
}

#[test]
fn idle_characters_give_way() {
    let mut crowd = Crowd::new(World::new());
    let (a, b) = (Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 5.0));
    let walker = crowd.add(a, &[b], AvoidRole::Reactive);
    let idle = crowd.add(Vec3::new(0.05, 0.0, 0.0), &[], AvoidRole::Reactive);
    let mut walker_drift = 0.0f32;
    while crowd.time < 10.0 {
        crowd.step();
        walker_drift = walker_drift.max(crowd.position(walker).x.abs());
    }
    let idle_moved = horizontal_len(crowd.position(idle) - Vec3::new(0.05, 0.0, 0.0));
    println!("give way: walker drift {walker_drift:.3} m, idle moved {idle_moved:.3} m, min gap {:.3} m", crowd.min_gap);
    assert!(crowd.arrived(walker));
    assert!(crowd.min_gap > 0.0, "contact: gap {}", crowd.min_gap);
    assert!(idle_moved > walker_drift, "the idle character stepped aside more than the walker swerved");
}

/// A wall along X at z = 0, with a doorway `width` wide centered on x = 0.
fn doorway(width: f32) -> World {
    let mut world = World::new();
    let h = width / 2.0;
    world.add_box(Vec3::new(-12.0, 0.0, -0.1), Vec3::new(-h, 2.5, 0.1));
    world.add_box(Vec3::new(h, 0.0, -0.1), Vec3::new(12.0, 2.5, 0.1));
    world.statics.rebuild();
    world
}

fn doorway_swap(avoid_params: AvoidParams, priority_of: fn(usize) -> f32) -> (Option<f32>, f32) {
    let mut crowd = Crowd::new(doorway(1.5));
    crowd.avoid_params = avoid_params;
    crowd.priority_of = priority_of;
    let xs = [-1.5f32, -0.5, 0.5, 1.5];
    for side in [-1.0f32, 1.0] {
        for &x in &xs {
            let start = Vec3::new(x, 0.0, 4.0 * side);
            let goal = Vec3::new(x, 0.0, -4.0 * side);
            let i = crowd.add(start, &[Vec3::ZERO, goal], AvoidRole::Reactive);
            crowd.pass_axis[i] = Some(Vec3::new(0.0, 0.0, -side));
        }
    }
    let time = crowd.run(20.0);
    (time, crowd.min_gap)
}

#[test]
fn eight_agents_swap_through_a_doorway() {
    // Equal priorities (the heading rule decides) and one side outranking
    // the other; per-character priorities interleaved across the sides (an
    // id hash) are measured, not required: they jam (locomotion.md).
    let equal: fn(usize) -> f32 = |_| 0.0;
    let by_side: fn(usize) -> f32 = |i| (i / 4) as f32;
    let (time, gap) = doorway_swap(AvoidParams::default(), equal);
    let (ranked, ranked_gap) = doorway_swap(AvoidParams::default(), by_side);
    let (interleaved, _) = doorway_swap(AvoidParams::default(), |i| ((i * 5) % 8) as f32);
    let (unbiased, _) = doorway_swap(AvoidParams { side_bias: 0.0, ..AvoidParams::default() }, equal);
    let (no_yield, _) = doorway_swap(AvoidParams { yield_weight: 1.0, ..AvoidParams::default() }, equal);
    println!(
        "doorway 1.5 m, 4 + 4 agents: done {time:?} s (min gap {gap:.3} m), ranked sides {ranked:?} s (gap {ranked_gap:.3}); \
         interleaved priorities {interleaved:?}, without keep-right bias {unbiased:?}, without yielding {no_yield:?}"
    );
    assert!(time.is_some_and(|t| t < 20.0), "deadlock: {time:?}");
    assert!(ranked.is_some_and(|t| t < 20.0), "deadlock with ranked sides: {ranked:?}");
}

fn scattered(n: usize, extent: f32, seed: u32) -> Vec<AvoidAgent> {
    let mut rng = Rng(seed);
    (0..n)
        .map(|k| {
            let heading = rng.range(-PI, PI);
            let pace = if k % 5 == 0 { 0.0 } else { rng.range(0.8, 2.0) };
            let preferred = Vec3::new(heading.sin(), 0.0, heading.cos()) * pace;
            AvoidAgent {
                position: p(rng.range(0.0, extent), rng.range(0.0, 0.3), rng.range(0.0, extent)),
                velocity: preferred * rng.range(0.5, 1.0),
                preferred,
                radius: rng.range(0.25, 0.45),
                max_speed: 2.0,
                role: if k == 7 { AvoidRole::NonReactive } else { AvoidRole::Reactive },
                travel: if k % 4 == 1 { Vec3::X } else { Vec3::ZERO },
                priority: (k % 3) as f32,
            }
        })
        .collect()
}

#[test]
fn batched_equals_single() {
    // Sparse, a dense cluster (neighbour cap active) and cells across a cell
    // boundary near the anchor's negative side.
    for (n, extent, seed) in [(150, 40.0, 1), (150, 8.0, 2), (40, 3.0, 3)] {
        let mut agents = scattered(n, extent, seed);
        agents[0].position = p(extent * 0.5, 0.0, extent * 0.5);
        let params = AvoidParams::default();
        let mut batch = vec![Vec3::ZERO; n];
        let mut avoidance = Avoidance::new();
        avoidance.solve(&agents, &params, DT, &mut batch);
        let mut changed = 0;
        for (i, &v) in batch.iter().enumerate() {
            let single = Avoidance::new().solve_one(&agents, i, &params, DT);
            assert_eq!(v, single, "agent {i} of {n} (extent {extent})");
            assert!(v.is_finite() && Vec2::new(v.x, v.z).length() <= agents[i].max_speed.max(horizontal_len(agents[i].preferred)) + 1e-4, "agent {i}: {v} from {:?}", agents[i]);
            changed += usize::from(v != agents[i].preferred);
        }
        assert!(changed > 0);
    }
}

#[test]
#[ignore = "timing; run in release"]
fn avoidance_timing_150_agents() {
    // The crowd street: 150 walkers over 40 × 40 m, and a dense square (12 × 12 m).
    for (label, extent) in [("street 40 m", 40.0), ("dense 12 m", 12.0)] {
        let agents = scattered(150, extent, 9);
        let params = AvoidParams::default();
        let mut out = vec![Vec3::ZERO; agents.len()];
        let mut avoidance = Avoidance::new();
        let mut times = Vec::new();
        for _ in 0..400 {
            let t0 = std::time::Instant::now();
            avoidance.solve(&agents, &params, DT, &mut out);
            times.push(t0.elapsed().as_secs_f64() * 1e3);
        }
        times.sort_by(f64::total_cmp);
        let (median, p95) = (times[200], times[380]);
        println!("avoidance, 150 agents, {label}: median {median:.4} ms, p95 {p95:.4} ms");
        if !cfg!(debug_assertions) {
            assert!(median <= 0.1, "{label}: {median} ms");
        }
    }
}

#[test]
fn coincident_agents_separate_opposite_ways() {
    let agent = AvoidAgent {
        position: p(1.0, 0.0, 2.0),
        velocity: Vec3::ZERO,
        preferred: Vec3::ZERO,
        radius: 0.3,
        max_speed: 1.5,
        role: AvoidRole::Reactive,
        priority: 0.0,
        travel: Vec3::ZERO,
    };
    let mut out = [Vec3::ZERO; 2];
    Avoidance::new().solve(&[agent, agent], &AvoidParams::default(), DT, &mut out);
    assert!(out[0].x < -0.1 && out[1].x > 0.1, "{out:?}");
}
