//! Headless course tests (test-levels.md "Headless runner"; research
//! `docs/research/animation/01-faith-runner-movement.md` §9): a level's
//! collide objects become a `StaticWorld`, a `KinematicMover` walks it at
//! 60 Hz with **input as a function of controller state** (not a timeline),
//! and the assertions are outcomes: what the context probes classified, where
//! the feet ended up. No Bevy.
//!
//! `levels/ledges.ron` is a framework copy of the game's test level
//! (`bevyrrowind/levels/ledges.ron`; the game's tests check they agree).

use std::sync::Arc;

use glam::Vec3;
use sf_controller::{
    Affordance, CollisionMesh, ContextHit, ContextParams, ContextProbes, ContextQuery, ContextRequest, Gait, KinematicMover,
    Locomotion, LocomotionIntent, MoveInput, MoverParams, RootMotion, StaticShape, StaticWorld, SurfaceId,
};
use sf_coords::{WorldPos, WorldTransform};
use sf_level::{Level, Object};

const LEDGES: &str = include_str!("levels/ledges.ron");
const DT: f32 = 1.0 / 60.0;
const WALK: f32 = 1.4;

/// A level's static collision.
struct Course {
    level: Level,
    world: StaticWorld,
}

impl Course {
    fn load(text: &str) -> Self {
        let level = Level::parse(text).expect("level parses");
        let mut world = StaticWorld::new(WorldPos::ORIGIN);
        for o in level.objects.iter().filter(|o| o.collide && o.dynamic.is_none()) {
            let data = o.mesh();
            let mesh = CollisionMesh {
                positions: data.positions.clone(),
                triangles: data.triangles(),
                surfaces: vec![SurfaceId::DEFAULT; data.indices.len() / 3],
            };
            let shape = Arc::new(StaticShape::new(&mesh).expect("non-empty mesh"));
            world.insert(&shape, WorldTransform { pos: pos(o.at), rotation: o.rotation(), scale: Vec3::ONE });
        }
        world.rebuild();
        Self { level, world }
    }

    fn object(&self, name: &str) -> &Object {
        self.level.objects.iter().find(|o| o.name.as_deref() == Some(name)).unwrap_or_else(|| panic!("no object {name}"))
    }
}

fn pos(v: Vec3) -> WorldPos {
    WorldPos::from_f64(v.as_dvec3())
}

/// One character on a course.
struct Sim<'a> {
    course: &'a Course,
    loco: Locomotion,
    params: MoverParams,
    context: ContextParams,
    mover: KinematicMover,
    probes: ContextProbes,
    /// Simulated seconds.
    t: f32,
}

impl<'a> Sim<'a> {
    fn at(course: &'a Course, feet: Vec3, dir: Vec3) -> Self {
        let params = MoverParams::default();
        Self {
            course,
            loco: Locomotion::new(pos(feet), dir.x.atan2(dir.z)),
            context: ContextParams::from_mover(&params),
            params,
            mover: KinematicMover::new(),
            probes: ContextProbes::new(),
            t: 0.0,
        }
    }

    fn feet(&self) -> Vec3 {
        self.loco.position.to_f64().as_vec3()
    }

    /// One context query from the feet along `dir`.
    fn probe(&mut self, dir: Vec3, query: ContextQuery) -> Option<ContextHit> {
        let mut out = [None];
        let request = ContextRequest { feet: self.loco.position, dir, params: &self.context, query };
        self.probes.run(&[request], &self.course.world, &mut out);
        out[0]
    }

    /// Steps until the script returns `None` (true) or `secs` pass (false).
    fn run(&mut self, secs: f32, mut script: impl FnMut(&mut Self) -> Option<LocomotionIntent>) -> bool {
        while self.t < secs {
            let Some(intent) = script(self) else { return true };
            let input = MoveInput { loco: &mut self.loco, intent, params: &self.params, root: RootMotion::default() };
            self.mover.step(input, &self.course.world, DT);
            self.t += DT;
            assert!(self.feet().y > -0.05, "fell through the floor at {:?}", self.feet());
        }
        false
    }
}

fn walk(dir: Vec3) -> LocomotionIntent {
    LocomotionIntent { desired_velocity: dir * WALK, desired_facing: None, gait: Gait::Walk }
}

/// The classification table (locomotion.md "Context probes as built").
fn expected(height: f32, depth: f32, p: &ContextParams) -> Affordance {
    if height <= p.step_up {
        Affordance::Step
    } else if height <= p.vault_max && depth <= p.vault_depth {
        Affordance::Vault
    } else if height <= p.mantle_max {
        Affordance::Mantle
    } else if height <= p.climb_max {
        Affordance::Climb
    } else {
        Affordance::Blocked
    }
}

/// Walks at each obstacle of the ledges level until the obstacle probe
/// answers, and checks its class and height against the table.
#[test]
fn ledges_obstacles_classify_by_height_and_depth() {
    let course = Course::load(LEDGES);
    let obstacles = [
        // Row A: heights, 1.5 m deep, approached from +z.
        ("h0.30", Vec3::NEG_Z),
        ("h0.50", Vec3::NEG_Z),
        ("h0.90", Vec3::NEG_Z),
        ("h1.20", Vec3::NEG_Z),
        ("h1.60", Vec3::NEG_Z),
        ("h2.00", Vec3::NEG_Z),
        ("h2.50", Vec3::NEG_Z),
        // Row B: depth decides vault vs mantle.
        ("thin-0.90", Vec3::NEG_Z),
        ("thin-1.20", Vec3::NEG_Z),
        ("deep-1.20", Vec3::NEG_Z),
        // The 3.5 m wall, from +x.
        ("wall-3.5", Vec3::NEG_X),
    ];
    let mut kinds = Vec::new();
    for (name, dir) in obstacles {
        let o = course.object(name);
        let sf_level::Shape::Box { size } = o.shape else { panic!("{name} is a box") };
        let along = (size * dir).length();
        let start = o.at - dir * (0.5 * along + 2.5);
        let mut sim = Sim::at(&course, start, dir);
        let mut hit = None;
        let done = sim.run(5.0, |s| {
            hit = s.probe(dir, ContextQuery::ObstacleAhead { range: 1.0 });
            if hit.is_some() { None } else { Some(walk(dir)) }
        });
        let hit = hit.unwrap_or_else(|| panic!("{name}: no obstacle found walking for {:.1} s (done {done})", sim.t));
        let want = expected(size.y, along, &sim.context);
        assert_eq!(hit.kind, want, "{name}: {hit:?}");
        assert!((hit.height - size.y).abs() < 0.03, "{name}: height {} vs {}", hit.height, size.y);
        kinds.push((name, hit.kind));
    }
    // The level's colour bands promise every class but Blocked shows up.
    for k in [Affordance::Step, Affordance::Vault, Affordance::Mantle, Affordance::Climb] {
        assert!(kinds.iter().any(|(_, got)| *got == k), "no {k:?} in {kinds:?}");
    }
}

/// Up the 20° ramp onto the first 1 m platform, then on until the drop
/// probe sees the first gap: the feet are on top and the drop is the
/// platform's height at its east edge.
#[test]
fn ramp_to_platform_then_drop_ahead() {
    let course = Course::load(LEDGES);
    let plat = course.object("plat-0");
    let east = plat.at.x + 1.5;
    let mut sim = Sim::at(&course, Vec3::new(-18.0, 0.0, plat.at.z), Vec3::X);
    let mut drop = None;
    let done = sim.run(12.0, |s| {
        if s.feet().x > plat.at.x - 1.0 {
            drop = s.probe(Vec3::X, ContextQuery::DropAhead { range: 1.5, min_drop: 0.5 });
        }
        if drop.is_some() { None } else { Some(walk(Vec3::X)) }
    });
    assert!(done, "never saw the gap (feet {:?})", sim.feet());
    let drop = drop.expect("drop");
    assert_eq!(drop.kind, Affordance::Drop);
    assert!((sim.feet().y - 1.0).abs() < 0.02, "feet on the platform top: {:?}", sim.feet());
    assert!((drop.height - 1.0).abs() < 0.05, "drop depth {}", drop.height);
    let edge = drop.point.to_f64().as_vec3();
    assert!((edge.x - east).abs() < 0.1, "edge at x {} vs {east}", edge.x);
    assert!(sim.feet().x < east, "stopped before the edge");
}
