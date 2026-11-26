//! Seat tests (`IN2a`): sit and stand through [`SEAT_RULES`] against a sensed slot. Logic is exact
//! (which rule runs, refusals, events once each, poses on the slot frame within 1e-4).

use std::sync::Arc;

use glam::{Vec2, Vec3};
use sf_anim::{RootPath, RootPose};

use super::*;

const ENTER: &str = "interact.sit.enter";
const LOOP: &str = "interact.sit.loop";
const EXIT: &str = "interact.sit.exit";
const SEAT: TargetId = TargetId(42);
const SLOT: u8 = 1;
const ENTER_SECS: f32 = 1.0;
const EXIT_SECS: f32 = 0.8;

/// A sit-down stand-in: half a metre forward while turning round.
struct SitDown;

impl RootPath for SitDown {
    fn duration(&self) -> f32 {
        ENTER_SECS
    }

    fn root_at(&self, t: f32) -> RootPose {
        let u = (t / ENTER_SECS).clamp(0.0, 1.0);
        RootPose::new(Vec3::new(0.0, 0.0, 0.5) * u, std::f32::consts::PI * u)
    }
}

/// A stand-up stand-in: half a metre forward.
struct StandUp;

impl RootPath for StandUp {
    fn duration(&self) -> f32 {
        EXIT_SECS
    }

    fn root_at(&self, t: f32) -> RootPose {
        let u = (t / EXIT_SECS).clamp(0.0, 1.0);
        RootPose::new(Vec3::new(0.0, 0.0, 0.5) * u, 0.0)
    }
}

/// A slot in world space (the run converts it to the feet-relative sense each tick).
#[derive(Clone, Copy)]
struct WorldSeat {
    enter: RootPose,
    exit: RootPose,
}

/// The default seat: its frame 0.6 m ahead, facing back at the user; its exit 0.55 m in front of it.
fn chair() -> WorldSeat {
    let pi = std::f32::consts::PI;
    WorldSeat { enter: RootPose::new(Vec3::new(0.1, 0.0, 0.6), pi), exit: RootPose::new(Vec3::new(0.1, 0.0, 0.05), pi) }
}

fn rules() -> RuleSet {
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, SEAT_RULES);
    b.build().expect("base and seat rules load")
}

fn slots() -> SlotTable {
    SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with(ENTER, SlotInfo::once(ENTER_SECS))
        .with(LOOP, SlotInfo::looping())
        .with(EXIT, SlotInfo::once(EXIT_SECS))
        .with_root(ENTER, Arc::new(SitDown))
        .with_root(EXIT, Arc::new(StandUp))
}

struct Run {
    rules: RuleSet,
    slots: SlotTable,
    params: CharacterParams,
    state: ActionState,
    out: ActionOutput,
    tick: Tick,
    /// Feet and yaw, world.
    feet: Vec3,
    yaw: f32,
    anchor: Vec3,
    seat: Option<WorldSeat>,
    /// Every event so far.
    log: Vec<ActionEvent>,
}

impl Run {
    fn new(seat: Option<WorldSeat>) -> Self {
        Self {
            rules: rules(),
            slots: slots(),
            params: CharacterParams::default(),
            state: ActionState::new(RuleKey::of("loco")),
            out: ActionOutput::default(),
            tick: Tick(0),
            feet: Vec3::ZERO,
            yaw: 0.0,
            anchor: Vec3::ZERO,
            seat,
            log: Vec::new(),
        }
    }

    /// One tick; the body follows the warp output (a stand-in for the shell), else stays put.
    fn step(&mut self, mut cmd: Command) {
        self.tick = self.tick.next();
        cmd.tick = self.tick;
        let ctx = StepCtx { rules: &self.rules, slots: &self.slots, params: &self.params, source: CharacterId(3) };
        let rel = |p: RootPose| RootPose::new(p.position - self.feet, p.yaw);
        let seat = self.seat.map(|w| SeatSense { target: SEAT, slot: SLOT, enter: rel(w.enter), exit: rel(w.exit) });
        let sense = Sense { grounded: true, yaw: self.yaw, seat, ..Sense::default() };
        step(&ctx, &mut self.state, &cmd, &sense, &mut self.out);
        if self.out.started {
            self.anchor = self.feet;
        }
        if let Some(w) = self.out.warp {
            assert!(w.root.position.is_finite() && w.root.yaw.is_finite() && w.progress.is_finite());
            self.feet = self.anchor + w.root.position;
            self.yaw = w.root.yaw;
        }
        self.log.extend_from_slice(&self.out.events);
    }

    /// Steps `cmd` until the move is `id` (at most `max` ticks); panics if it never gets there.
    fn until(&mut self, id: &str, cmd: Command, max: usize) {
        for _ in 0..max {
            if self.state.mov == RuleKey::of(id) {
                return;
            }
            self.step(cmd);
        }
        assert_eq!(self.state.mov, RuleKey::of(id), "never reached {id}: events {:?}", self.log);
    }

    fn count(&self, kind: EventKind) -> usize {
        self.log.iter().filter(|e| e.kind == kind).count()
    }

    fn refused(&self, id: &str) -> Option<Refused> {
        self.out.refusals.iter().find(|r| r.rule == RuleKey::of(id)).map(|r| r.reason)
    }

    /// Sat down on the chair: Interact, then the enter warp to its end.
    fn seated() -> Self {
        let mut r = Self::new(Some(chair()));
        r.step(press(ActionClass::Interact));
        assert_eq!(r.state.mov, RuleKey::of(ENTER), "events {:?}, refusals {:?}", r.out.events, r.out.refusals);
        r.until(LOOP, Command::default(), 70);
        r
    }
}

fn press(c: ActionClass) -> Command {
    let mut cmd = Command::default();
    cmd.buttons.pressed.insert(c);
    cmd.buttons.held.insert(c);
    cmd
}

fn stick(y: f32) -> Command {
    Command { stick: Vec2::new(0.0, y), ..Command::default() }
}

/// Angle difference wrapped to (-pi, pi].
fn yaw_diff(a: f32, b: f32) -> f32 {
    (a - b + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

fn assert_on(r: &Run, pose: RootPose) {
    assert!((r.feet - pose.position).length() < 1e-4, "feet {:?}, want {:?}", r.feet, pose.position);
    assert!(yaw_diff(r.yaw, pose.yaw).abs() < 1e-4, "yaw {}, want {}", r.yaw, pose.yaw);
}

#[test]
fn interact_with_a_seat_sits_exactly_on_the_slot() {
    let r = Run::seated();
    assert_on(&r, chair().enter);
    let entered: Vec<_> = r.log.iter().filter(|e| e.kind == EventKind::SeatEntered(SLOT)).collect();
    assert_eq!(entered.len(), 1, "{:?}", r.log);
    assert_eq!(entered[0].target, Some(SEAT));
    assert_eq!(r.count(EventKind::SeatExited(SLOT)), 0);
    let w = r.out.warp.expect("the seated hold pins the root");
    assert_eq!(w.root.position, Vec3::ZERO);
    assert_eq!(r.out.intent.desired_velocity, Vec3::ZERO, "mover off");
    assert!(r.state.seat.is_some_and(|s| s.entered && s.target == SEAT && s.slot == SLOT));
}

#[test]
fn seated_holds_still_under_a_small_stick_or_no_input() {
    let mut r = Run::seated();
    let (feet, yaw, seen) = (r.feet, r.yaw, r.log.len());
    let small = r.params.deadzone * 0.5;
    for cmd in [stick(small), Command::default()] {
        for _ in 0..60 {
            r.step(cmd);
            assert_eq!(r.state.mov, RuleKey::of(LOOP));
            assert_eq!((r.feet, r.yaw), (feet, yaw));
            assert_eq!(r.out.intent.desired_velocity, Vec3::ZERO);
            assert_eq!(r.out.prompt, None);
        }
    }
    assert!(r.log[seen..].is_empty(), "no events while seated: {:?}", &r.log[seen..]);
}

/// Each exit input stands up onto the sensed exit: one exited event, back to locomotion, and the
/// press never starts anything else (no jump, no re-sit, no activation).
#[test]
fn jump_move_or_interact_stands_onto_the_exit() {
    for (name, first, then) in [
        ("jump", press(ActionClass::Jump), Command::default()),
        ("move", stick(1.0), stick(1.0)),
        ("interact", press(ActionClass::Interact), Command::default()),
    ] {
        let mut r = Run::seated();
        r.step(first);
        assert_eq!(r.state.mov, RuleKey::of(EXIT), "{name}: events {:?}", r.out.events);
        r.until("loco", then, 60);
        assert_on(&r, chair().exit);
        assert_eq!(r.count(EventKind::SeatEntered(SLOT)), 1, "{name}");
        assert_eq!(r.count(EventKind::SeatExited(SLOT)), 1, "{name}: {:?}", r.log);
        assert_eq!(r.count(EventKind::Launch), 0, "{name}: the press stood up, it didn't jump");
        assert_eq!(r.log.iter().filter(|e| e.kind == EventKind::MoveStart(RuleKey::of(ENTER))).count(), 1, "{name}: no re-sit");
        assert!(r.state.seat.is_none() && r.state.warp.is_none(), "{name}");
        // Stays in locomotion afterwards, no second exit.
        for _ in 0..10 {
            r.step(then);
        }
        assert_eq!(r.count(EventKind::SeatExited(SLOT)), 1, "{name}");
    }
}

#[test]
fn losing_the_seat_while_seated_forces_the_stand_up() {
    let mut r = Run::seated();
    r.seat = None;
    r.step(Command::default());
    assert_eq!(r.state.mov, RuleKey::of(EXIT), "invalidation stands up at once");
    r.until("loco", Command::default(), 60);
    // No exit pose sensed: back where the user sat down from.
    assert_on(&r, RootPose::new(Vec3::ZERO, 0.0));
    assert_eq!(r.count(EventKind::SeatExited(SLOT)), 1, "{:?}", r.log);
    for _ in 0..10 {
        r.step(Command::default());
    }
    assert_eq!(r.count(EventKind::SeatExited(SLOT)), 1);
}

#[test]
fn interact_without_a_seat_is_refused() {
    let mut r = Run::new(None);
    r.step(press(ActionClass::Interact));
    assert_eq!(r.refused(ENTER), Some(Refused::Slot), "{:?}", r.out.refusals);
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    assert!(r.state.seat.is_none() && r.state.warp.is_none());
    assert!(!r.out.started && r.log.is_empty(), "{:?}", r.log);
    assert_eq!(r.feet, Vec3::ZERO);
}

#[test]
fn a_seat_too_far_refuses_max_correction() {
    let far = WorldSeat { enter: RootPose::new(Vec3::new(0.0, 0.0, 3.0), std::f32::consts::PI), ..chair() };
    let mut r = Run::new(Some(far));
    r.step(press(ActionClass::Interact));
    assert_eq!(r.refused(ENTER), Some(Refused::MaxCorrection), "{:?}", r.out.refusals);
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    assert!(r.state.seat.is_none() && r.state.warp.is_none());
}

#[test]
fn stand_exit_needs_a_seated_owner() {
    let bad = r#"[(id: "x", kind: Interaction, category: Interact, entry: (classes: [Interact]), exits: [(on: Stand, to: Loco)])]"#;
    let errors = RuleSetBuilder::new().add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, bad).build().expect_err("refused");
    assert!(errors.iter().any(|e| e.message.contains("Stand")), "{errors:?}");
}
