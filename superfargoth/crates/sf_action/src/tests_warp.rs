//! Warp owner tests (B2a): the waist-high mantle rule against a box ledge in front of the
//! character. Logic is exact (which rule fires, refusal reasons); the landing is a sanity bound.

use std::sync::Arc;

use glam::{Vec2, Vec3};
use sf_anim::{RootPath, RootPose};

use super::*;

const MANTLE: &str = "traverse.mantle.waist";
/// The retargeted UAL2 `ClimbUp_1m_RM` on the Morrowind skeleton (measured: 0.667 s, root rises
/// 1.19 m and travels 2.0 m forward, linearly).
const CLIP_SECS: f32 = 0.667;

/// A straight-line root track like the measured clip's.
struct Line;

impl RootPath for Line {
    fn duration(&self) -> f32 {
        CLIP_SECS
    }

    fn root_at(&self, t: f32) -> RootPose {
        let u = (t / CLIP_SECS).clamp(0.0, 1.0);
        RootPose::new(Vec3::new(0.0, 1.19, 2.0) * u, 0.0)
    }
}

const HIGH: &str = "traverse.mantle.high";
/// The retargeted UAL2 `ClimbUp_2m_RM` (measured: 1.3 s, root rises 2.38 m and travels 2.0 m
/// forward; a straight line stands in for it).
const HIGH_SECS: f32 = 1.3;

struct HighLine;

impl RootPath for HighLine {
    fn duration(&self) -> f32 {
        HIGH_SECS
    }

    fn root_at(&self, t: f32) -> RootPose {
        let u = (t / HIGH_SECS).clamp(0.0, 1.0);
        RootPose::new(Vec3::new(0.0, 2.38, 2.0) * u, 0.0)
    }
}

fn rules() -> RuleSet {
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, TRAVERSAL_RULES);
    b.build().expect("base and traversal rules load")
}

fn slots(root: bool) -> SlotTable {
    let t = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with(MANTLE, SlotInfo::once(CLIP_SECS))
        .with(HIGH, SlotInfo::once(HIGH_SECS));
    if root { t.with_root(MANTLE, Arc::new(Line)).with_root(HIGH, Arc::new(HighLine)) } else { t }
}

/// A box ledge whose face is `dist` m ahead (+Z) of the feet, `height` m tall, 1.5 m deep.
fn ledge(dist: f32, height: f32, class: ObstacleClass) -> Obstacle {
    Obstacle { class, lip: Vec3::new(0.0, height, dist), normal: Vec3::NEG_Z, height, depth: 1.5, clearance: 3.0 }
}

struct Run {
    rules: RuleSet,
    slots: SlotTable,
    params: CharacterParams,
    state: ActionState,
    out: ActionOutput,
    tick: Tick,
    /// Feet, world.
    feet: Vec3,
    anchor: Vec3,
}

impl Run {
    fn new(root: bool) -> Self {
        Self {
            rules: rules(),
            slots: slots(root),
            params: CharacterParams::default(),
            state: ActionState::new(RuleKey::of("loco")),
            out: ActionOutput::default(),
            tick: Tick(0),
            feet: Vec3::ZERO,
            anchor: Vec3::ZERO,
        }
    }

    /// One tick; the body follows the warp output (a stand-in for the shell), else stays put.
    fn step(&mut self, mut cmd: Command, obstacle: Option<Obstacle>) {
        self.tick = self.tick.next();
        cmd.tick = self.tick;
        let ctx = StepCtx { rules: &self.rules, slots: &self.slots, params: &self.params, source: CharacterId(3) };
        let sense = Sense { grounded: true, obstacle, ..Sense::default() };
        step(&ctx, &mut self.state, &cmd, &sense, &mut self.out);
        if self.out.started {
            self.anchor = self.feet;
        }
        if let Some(w) = self.out.warp {
            assert!(w.root.position.is_finite() && w.root.yaw.is_finite() && w.progress.is_finite());
            self.feet = self.anchor + w.root.position;
        }
    }

    fn started(&self, id: &str) -> bool {
        self.out.events.iter().any(|e| e.kind == EventKind::MoveStart(RuleKey::of(id)))
    }

    fn refused(&self, id: &str) -> Option<Refused> {
        self.out.refusals.iter().find(|r| r.rule == RuleKey::of(id)).map(|r| r.reason)
    }
}

fn jump_forward(sneak: bool) -> Command {
    let mut cmd = Command { stick: Vec2::new(0.0, 1.0), ..Command::default() };
    cmd.buttons.pressed.insert(ActionClass::Jump);
    cmd.buttons.held.insert(ActionClass::Jump);
    if sneak {
        cmd.buttons.held.insert(ActionClass::Sneak);
    }
    cmd
}

#[test]
fn jump_forward_at_a_waist_ledge_mantles_onto_the_top() {
    let mut r = Run::new(true);
    let o = ledge(0.3, 1.0, ObstacleClass::Mantle);
    r.step(jump_forward(false), Some(o));
    assert!(r.started(MANTLE), "events {:?}, refusals {:?}", r.out.events, r.out.refusals);
    assert_eq!(r.out.launch, None);
    assert_eq!(r.out.intent.desired_velocity, Vec3::ZERO, "no steering while warping");
    let plan = r.state.warp.expect("plan kept in the state").plan;
    assert!(plan.ratio() <= 0.35);
    // The plan's end is on top, the inset past the lip.
    let end = plan.root_at(&Line, CLIP_SECS);
    assert!((end.position - Vec3::new(0.0, 1.0, 1.3)).length() < 0.02, "{end:?}");
    // Played tick by tick: progress follows the clock, the body ends on the top, then walks.
    let mut last = -1.0;
    for _ in 0..60 {
        if r.state.mov != RuleKey::of(MANTLE) {
            break;
        }
        if let Some(w) = r.out.warp {
            assert!(w.progress > last && w.progress <= CLIP_SECS + 1e-6);
            last = w.progress;
        }
        r.step(Command::default(), None);
    }
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    assert!(r.state.warp.is_none());
    assert!((last - CLIP_SECS).abs() < 1e-4, "last progress {last}");
    assert!((r.feet.y - 1.0).abs() < 0.02 && (r.feet.z - 1.3).abs() < 0.02, "feet {:?}", r.feet);
}

#[test]
fn up_also_mantles_but_not_without_forward_input() {
    let mut r = Run::new(true);
    let mut cmd = Command { stick: Vec2::new(0.0, 1.0), ..Command::default() };
    cmd.buttons.pressed.insert(ActionClass::Up);
    r.step(cmd, Some(ledge(0.3, 1.0, ObstacleClass::Mantle)));
    assert!(r.started(MANTLE));

    let mut r = Run::new(true);
    let mut cmd = Command::default();
    cmd.buttons.pressed.insert(ActionClass::Jump);
    cmd.buttons.held.insert(ActionClass::Jump);
    r.step(cmd, Some(ledge(0.3, 1.0, ObstacleClass::Mantle)));
    assert_eq!(r.refused(MANTLE), Some(Refused::Forward));
    assert!(r.started("air.jump"));
}

#[test]
fn sneak_never_mantles() {
    let mut r = Run::new(true);
    r.step(jump_forward(true), Some(ledge(0.3, 1.0, ObstacleClass::Mantle)));
    assert_eq!(r.refused(MANTLE), Some(Refused::Mode));
    assert!(!r.started(MANTLE));
}

#[test]
fn ledges_out_of_the_band_or_class_jump_instead() {
    for o in [ledge(0.3, 2.5, ObstacleClass::Mantle), ledge(0.3, 0.3, ObstacleClass::Mantle), ledge(0.3, 1.0, ObstacleClass::Vault)] {
        let mut r = Run::new(true);
        r.step(jump_forward(false), Some(o));
        assert_eq!(r.refused(MANTLE), Some(Refused::Probe), "{o:?}");
        assert!(r.started("air.jump"), "{o:?}");
    }
    let mut r = Run::new(true);
    r.step(jump_forward(false), None);
    assert_eq!(r.refused(MANTLE), Some(Refused::Probe));
    assert!(r.started("air.jump"));
}

#[test]
fn too_large_a_correction_refuses_into_a_jump() {
    let mut r = Run::new(true);
    r.step(jump_forward(false), Some(ledge(3.0, 1.0, ObstacleClass::Mantle)));
    assert_eq!(r.refused(MANTLE), Some(Refused::MaxCorrection));
    assert!(r.started("air.jump") && r.out.launch.is_some());
    assert!(r.state.warp.is_none());
}

#[test]
fn a_slot_without_root_motion_refuses_into_a_jump() {
    let mut r = Run::new(false);
    r.step(jump_forward(false), Some(ledge(0.3, 1.0, ObstacleClass::Mantle)));
    assert_eq!(r.refused(MANTLE), Some(Refused::NoRootPath));
    assert!(r.started("air.jump"));
}

#[test]
fn shallow_tops_keep_the_target_a_radius_inside() {
    let mut r = Run::new(true);
    let o = Obstacle { depth: 1.2, ..ledge(0.6, 1.0, ObstacleClass::Mantle) };
    r.step(jump_forward(false), Some(o));
    let end = r.state.warp.expect("planned").plan.root_at(&Line, CLIP_SECS);
    // 1.2 deep minus the 0.3 radius: 0.9 past the lip (at 0.6).
    assert!((end.position.z - 1.5).abs() < 1e-4, "{end:?}");
}

#[test]
fn jump_forward_at_a_high_ledge_mantles_onto_the_top_with_the_high_clip() {
    let mut r = Run::new(true);
    r.step(jump_forward(false), Some(ledge(0.3, 2.0, ObstacleClass::Mantle)));
    assert!(r.started(HIGH), "events {:?}, refusals {:?}", r.out.events, r.out.refusals);
    assert_eq!(r.refused(MANTLE), Some(Refused::Probe), "above the waist band");
    let plan = r.state.warp.expect("plan kept in the state").plan;
    assert!(plan.ratio() <= 0.35);
    for _ in 0..120 {
        if r.state.mov != RuleKey::of(HIGH) {
            break;
        }
        r.step(Command::default(), None);
    }
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    // On top, the 1.2 m inset past the lip (at 0.3), the 1.5 m deep top less a radius allows it.
    assert!((r.feet.y - 2.0).abs() < 0.02 && (r.feet.z - 1.5).abs() < 0.02, "feet {:?}", r.feet);
}

#[test]
fn the_high_band_starts_where_the_waist_band_ends_and_stops_below_a_climb() {
    // (height, the rule that fires; None: a plain jump). The shared edge (0.85 heights) is the waist's.
    let h = CharacterParams::default().height;
    let cases = [(1.5, Some(MANTLE)), (0.85 * h, Some(MANTLE)), (0.86 * h, Some(HIGH)), (1.6, Some(HIGH)), (2.2, Some(HIGH)), (2.5, None)];
    for (height, fires) in cases {
        for (dist, up) in [(0.3, false), (0.3, true), (0.8, false)] {
            let mut r = Run::new(true);
            let mut cmd = jump_forward(false);
            if up {
                cmd = Command { stick: Vec2::new(0.0, 1.0), ..Command::default() };
                cmd.buttons.pressed.insert(ActionClass::Up);
            }
            r.step(cmd, Some(ledge(dist, height, ObstacleClass::Mantle)));
            let fired = [MANTLE, HIGH].into_iter().find(|id| r.started(id));
            assert_eq!(fired, fires, "{height} m at {dist} m (up {up}): refusals {:?}", r.out.refusals);
            if fires.is_none() {
                assert_eq!(r.refused(HIGH), Some(Refused::Probe));
            }
        }
    }
}
