//! Locked-on locomotion (combat.md "Lock-on and camera", C1c): the body faces the target at the
//! mover's own turn rates and the stick strafes; a sprint faces its run; unlocking frees the facing.
//! The decisions run on the real kinematic mover over flat ground.

use std::f32::consts::{FRAC_PI_2, PI};

use glam::{Vec2, Vec3};
use sf_controller::{KinematicMover, Locomotion, LocomotionIntent, MoveInput, MoverParams, RootMotion, TerrainProbe, TurnRates, wrap_angle};
use sf_coords::WorldPos;

use super::*;

const DT: f32 = 1.0 / 60.0;
/// The capped turn rate the tests give the mover, rad/s (180 deg/s).
const TURN: f32 = PI;

struct FlatGround;

impl sf_controller::HeightSurface for FlatGround {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        let t = -WorldPos::ORIGIN.delta_f32(origin).y / dir.y;
        (dir.y.abs() > 1e-6 && (0.0..=max).contains(&t)).then_some((t, Vec3::Y))
    }
}

struct Sim {
    rules: RuleSet,
    slots: SlotTable,
    params: CharacterParams,
    mover_params: MoverParams,
    state: ActionState,
    loco: Locomotion,
    mover: KinematicMover,
    out: ActionOutput,
    tick: Tick,
    /// The lock target's world position (`None`: unlocked).
    target: Option<Vec3>,
}

impl Sim {
    /// Standing at the origin facing +Z, locked on a target `target`.
    fn new(target: Option<Vec3>) -> Self {
        let slots = SlotTable::new().with("loco.move", SlotInfo::looping()).with("air.fall", SlotInfo::looping());
        let mut mover_params = MoverParams::default();
        mover_params.springs.turn = Some(TurnRates::uniform(TURN));
        let mut sim = Self {
            rules: RuleSet::from_ron(BASE_RULES).expect("base rules load"),
            slots,
            params: CharacterParams::default(),
            mover_params,
            state: ActionState::new(RuleKey::of("loco")),
            loco: Locomotion::new(WorldPos::ORIGIN, 0.0),
            mover: KinematicMover::new(),
            out: ActionOutput::default(),
            tick: Tick(0),
            target,
        };
        // Settle onto the ground with no intent.
        let probe = TerrainProbe(FlatGround);
        sim.mover.step(MoveInput { loco: &mut sim.loco, intent: LocomotionIntent::IDLE, params: &sim.mover_params, root: RootMotion::default() }, &probe, DT);
        sim
    }

    fn pos(&self) -> Vec3 {
        WorldPos::ORIGIN.delta_f32(self.loco.position)
    }

    fn yaw(&self) -> f32 {
        self.loco.motion.yaw
    }

    /// The bearing from the body to the target.
    fn bearing(&self) -> f32 {
        let d = self.target.expect("locked") - self.pos();
        d.x.atan2(d.z)
    }

    /// One tick: the action step, then the mover on its intent.
    fn run(&mut self, stick: Vec2, held: ClassSet) -> LocomotionIntent {
        self.tick = self.tick.next();
        let mut cmd = Command { tick: self.tick, stick, ..Command::default() };
        cmd.buttons.held = held;
        let v = self.loco.motion.velocity;
        let sense = Sense {
            grounded: self.loco.grounded.is_some(),
            planar_speed: Vec3::new(v.x, 0.0, v.z).length(),
            yaw: self.loco.motion.yaw,
            lock_target: self.target.map(|t| t - self.pos()),
            ..Sense::default()
        };
        let ctx = StepCtx { rules: &self.rules, slots: &self.slots, params: &self.params, source: CharacterId(7) };
        step(&ctx, &mut self.state, &cmd, &sense, &mut self.out);
        let probe = TerrainProbe(FlatGround);
        let intent = self.out.intent;
        self.mover.step(MoveInput { loco: &mut self.loco, intent, params: &self.mover_params, root: RootMotion::default() }, &probe, DT);
        intent
    }

    /// The camera-relative "right" stick of a camera framing the target.
    fn right_stick(&self) -> Vec2 {
        let b = self.bearing();
        Vec2::new(b.cos(), -b.sin())
    }
}

fn sprint() -> ClassSet {
    let mut held = ClassSet::EMPTY;
    held.insert(ActionClass::Sprint);
    held
}

#[test]
fn locked_sideways_stick_circles_the_target_while_facing_it() {
    let mut sim = Sim::new(Some(Vec3::new(0.0, 0.0, 4.0)));
    let start = sim.pos();
    for t in 0..90 {
        let stick = sim.right_stick();
        let intent = sim.run(stick, ClassSet::EMPTY);
        assert_eq!(intent.desired_facing.map(|f| wrap_angle(f - sim.bearing()).abs() < 0.1), Some(true), "tick {t}: {intent:?}");
        // Tangential: no push toward or away from the target, at the run's side speed.
        let to = (sim.target.unwrap() - sim.pos()).normalize();
        let v = intent.desired_velocity;
        assert!(v.dot(to).abs() < 0.05 * v.length(), "tick {t}: radial {v} toward {to}");
        assert!((v.length() - sim.params.gaits.run.side).abs() < 0.05, "tick {t}: speed {}", v.length());
        assert!(wrap_angle(sim.yaw() - sim.bearing()).abs() < 0.1, "tick {t}: yaw {} bearing {}", sim.yaw(), sim.bearing());
    }
    let moved = sim.pos() - start;
    assert!(moved.length() > 3.0, "the body circled: {moved}");
    let r = (sim.target.unwrap() - sim.pos()).length();
    assert!((3.5..5.5).contains(&r), "stayed around the target: radius {r}");
}

#[test]
fn locked_back_stick_walks_backwards_facing_the_target() {
    let mut sim = Sim::new(Some(Vec3::new(0.0, 0.0, 4.0)));
    let mut intent = LocomotionIntent::IDLE;
    for _ in 0..60 {
        intent = sim.run(Vec2::new(0.0, -1.0), ClassSet::EMPTY);
    }
    assert!(intent.desired_velocity.z < 0.0 && (intent.desired_velocity.length() - sim.params.gaits.run.back).abs() < 0.05, "{intent:?}");
    assert!(sim.pos().z < -1.0 && wrap_angle(sim.yaw()).abs() < 0.05, "backed off facing +Z: {} yaw {}", sim.pos(), sim.yaw());
}

#[test]
fn locked_facing_turns_at_the_mover_rate_never_snaps() {
    // Target straight behind: standing still, the body turns round at 180 deg/s.
    let mut sim = Sim::new(Some(Vec3::new(0.0, 0.0, -4.0)));
    let mut last = sim.yaw();
    for t in 1..=90 {
        let intent = sim.run(Vec2::ZERO, ClassSet::EMPTY);
        assert!(intent.desired_facing.is_some() && intent.desired_velocity == Vec3::ZERO, "{intent:?}");
        let turned = wrap_angle(sim.yaw() - last).abs();
        assert!(turned <= TURN * DT + 1e-4, "tick {t}: turned {turned} rad");
        last = sim.yaw();
        if t == 30 {
            assert!(wrap_angle(sim.yaw() - sim.bearing()).abs() > FRAC_PI_2 * 0.9, "half way round after 0.5 s: {}", sim.yaw());
        }
    }
    assert!(wrap_angle(sim.yaw() - sim.bearing()).abs() < 1e-3, "faces the target after 1.5 s: {}", sim.yaw());
}

#[test]
fn locked_sprint_faces_its_run_and_the_lock_returns_after() {
    let mut sim = Sim::new(Some(Vec3::new(0.0, 0.0, 4.0)));
    let mut intent = LocomotionIntent::IDLE;
    for _ in 0..60 {
        intent = sim.run(Vec2::new(1.0, 0.0), sprint());
    }
    assert_eq!(sim.state.mode.mode, Mode::FreeRun);
    assert!(intent.desired_facing.is_none(), "sprint drops the lock facing: {intent:?}");
    assert!(wrap_angle(sim.yaw() - FRAC_PI_2).abs() < 0.05, "faces the run (+X): {}", sim.yaw());
    // Sprint released, stick still held: back to strafing, turning back to the target.
    for _ in 0..60 {
        intent = sim.run(Vec2::new(1.0, 0.0), ClassSet::EMPTY);
    }
    assert_eq!(intent.desired_facing.map(|f| wrap_angle(f - sim.bearing()).abs() < 0.02), Some(true), "{intent:?}");
    assert!(wrap_angle(sim.yaw() - sim.bearing()).abs() < 0.1, "faces the target again: {} vs {}", sim.yaw(), sim.bearing());
}

#[test]
fn unlocking_restores_free_facing() {
    let mut sim = Sim::new(Some(Vec3::new(0.0, 0.0, 4.0)));
    for _ in 0..20 {
        sim.run(Vec2::new(1.0, 0.0), ClassSet::EMPTY);
    }
    assert!(wrap_angle(sim.yaw() - sim.bearing()).abs() < 0.1);
    sim.target = None;
    let mut intent = LocomotionIntent::IDLE;
    for _ in 0..60 {
        intent = sim.run(Vec2::new(1.0, 0.0), ClassSet::EMPTY);
    }
    assert!(intent.desired_facing.is_none(), "{intent:?}");
    assert!(wrap_angle(sim.yaw() - FRAC_PI_2).abs() < 0.05, "faces the stick (+X) again: {}", sim.yaw());
    assert!((intent.desired_velocity.length() - sim.params.gaits.run.forward).abs() < 0.05, "forward speed: {intent:?}");
}

#[test]
fn lock_side_classifies_the_stick_against_the_target() {
    // Target ahead along +Z: right of a +Z facing is -X (Side::of's convention).
    let ahead = Vec2::new(0.0, 5.0);
    assert_eq!(StickSide::of(ahead, Vec2::new(0.0, 1.0)), Some(StickSide::Front));
    assert_eq!(StickSide::of(ahead, Vec2::new(0.0, -1.0)), Some(StickSide::Back));
    assert_eq!(StickSide::of(ahead, Vec2::new(1.0, 0.0)), Some(StickSide::Left));
    assert_eq!(StickSide::of(ahead, Vec2::new(-1.0, 0.2)), Some(StickSide::Right));
    // Relative to the target, not the world: target to the +X side, stick +Z is its right.
    assert_eq!(StickSide::of(Vec2::new(3.0, 0.0), Vec2::new(0.0, 1.0)), Some(StickSide::Right));
    assert_eq!(StickSide::of(ahead, Vec2::ZERO), None);
}
