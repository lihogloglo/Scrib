//! Warp owner tests (B2a): the waist-high mantle rule against a box ledge in front of the
//! character. Logic is exact (which rule fires, refusal reasons); the landing is a sanity bound.

use std::sync::Arc;

use glam::{Vec2, Vec3};
use sf_anim::{Contact, ContactKind, RootPath, RootPose, Side};

use super::*;

const MANTLE: &str = "traverse.mantle.waist";
/// The retargeted UAL2 `ClimbUp_1m_RM` on the Morrowind skeleton (measured: 0.667 s, root rises
/// 1.19 m and travels 2.0 m forward, linearly).
const CLIP_SECS: f32 = 0.667;
/// Where the waist rule's move ends (its `stand`: both feet on top), clip seconds.
const WAIST_STAND: f32 = 0.55;

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
    println!("waist plan ratio {:.3}", plan.ratio());
    assert!(plan.ratio() <= 0.6, "under the rule's refusal limit");
    // The plan lands on top, the inset (0.4) past the lip (at 0.3), at the rule's stand time.
    let end = plan.root_at(&Line, WAIST_STAND);
    assert!((end.position - Vec3::new(0.0, 1.0, 0.7)).length() < 0.02, "{end:?}");
    // Played tick by tick: progress follows the clock up to the stand time, where the body is on
    // the top and the move ends (the clip's step past it never plays).
    let mut last = -1.0;
    for _ in 0..60 {
        if r.state.mov != RuleKey::of(MANTLE) {
            break;
        }
        if let Some(w) = r.out.warp {
            assert!(w.progress > last && w.progress <= WAIST_STAND + 1e-6);
            assert_eq!(w.settle, w.progress >= WAIST_STAND, "lands standing only at the stand time");
            last = w.progress;
        }
        r.step(Command::default(), None);
    }
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    assert!(r.state.warp.is_none());
    assert!((last - WAIST_STAND).abs() < 1e-4, "last progress {last}");
    assert!((r.feet.y - 1.0).abs() < 0.02 && (r.feet.z - 0.7).abs() < 0.02, "feet {:?}", r.feet);
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
    let o = Obstacle { depth: 0.6, ..ledge(0.6, 1.0, ObstacleClass::Mantle) };
    r.step(jump_forward(false), Some(o));
    let end = r.state.warp.expect("planned").plan.root_at(&Line, WAIST_STAND);
    // 0.6 deep minus the 0.3 radius: 0.3 past the lip (at 0.6), under the 0.4 inset.
    assert!((end.position.z - 0.9).abs() < 1e-4, "{end:?}");
}

#[test]
fn jump_forward_at_a_high_ledge_mantles_onto_the_top_with_the_high_clip() {
    let mut r = Run::new(true);
    r.step(jump_forward(false), Some(ledge(0.3, 2.0, ObstacleClass::Mantle)));
    assert!(r.started(HIGH), "events {:?}, refusals {:?}", r.out.events, r.out.refusals);
    assert_eq!(r.refused(MANTLE), Some(Refused::Probe), "above the waist band");
    let plan = r.state.warp.expect("plan kept in the state").plan;
    println!("high plan ratio {:.3}", plan.ratio());
    assert!(plan.ratio() <= 0.5, "under the rule's refusal limit");
    for _ in 0..120 {
        if r.state.mov != RuleKey::of(HIGH) {
            break;
        }
        r.step(Command::default(), None);
    }
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    // On top, the 0.4 m inset past the lip (at 0.3).
    assert!((r.feet.y - 2.0).abs() < 0.02 && (r.feet.z - 0.7).abs() < 0.02, "feet {:?}", r.feet);
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

// ---------------------------------------------------------------------------
// B2b: hands on the lip.

/// A root path with baked hand contacts.
struct Gripped<P> {
    path: P,
    contacts: Vec<Contact>,
}

impl<P: RootPath> RootPath for Gripped<P> {
    fn duration(&self) -> f32 {
        self.path.duration()
    }

    fn root_at(&self, t: f32) -> RootPose {
        self.path.root_at(t)
    }

    fn contacts(&self) -> &[Contact] {
        &self.contacts
    }
}

/// A hand contact at clip-space `position` (its root-frame offset from the line root, which
/// turns nothing).
fn hand(path: &impl RootPath, side: Side, start: f32, end: f32, position: Vec3) -> Contact {
    let local = position - path.root_at(start).position;
    Contact { kind: ContactKind::Hand, side, limb: "arm".into(), joint: "hand".into(), start, end, position, local }
}

/// The baked clips' hand contacts (measured): `ClimbUp_1m` plants the left hand only,
/// `ClimbUp_2m` the right, then the left.
fn waist_gripped() -> Gripped<Line> {
    Gripped { contacts: vec![hand(&Line, Side::Left, 0.167, 0.333, Vec3::new(0.23, 1.27, 0.89))], path: Line }
}

fn high_gripped() -> Gripped<HighLine> {
    let contacts = vec![
        hand(&HighLine, Side::Right, 0.333, 0.667, Vec3::new(-0.22, 1.94, 0.70)),
        hand(&HighLine, Side::Left, 0.467, 0.900, Vec3::new(0.26, 2.05, 0.87)),
    ];
    Gripped { contacts, path: HighLine }
}

/// Mantles the `dist`/`height` ledge with the gripped clips; the rule must be `rule`.
fn mantle_grip(dist: f32, height: f32, rule: &str) -> (Option<LedgeGrip>, sf_anim::WarpPlan) {
    let mut r = Run::new(false);
    r.slots = slots(false).with_root(MANTLE, Arc::new(waist_gripped())).with_root(HIGH, Arc::new(high_gripped()));
    r.step(jump_forward(false), Some(ledge(dist, height, ObstacleClass::Mantle)));
    assert!(r.started(rule), "{height} m: events {:?}, refusals {:?}", r.out.events, r.out.refusals);
    let w = r.state.warp.expect("planned");
    (w.grip, w.plan)
}

/// The target's distance from the lip line (along X through the lip at `dist`, `height`).
fn off_lip_line(target: Vec3, dist: f32, height: f32) -> f32 {
    Vec2::new(target.y - height, target.z - dist).length()
}

#[test]
#[allow(clippy::float_cmp)] // exact: 0 outside the ramps, 1 through the hold
fn mantle_hands_grip_the_lip_line_above_the_top() {
    for (height, rule, duration) in [(1.2, MANTLE, CLIP_SECS), (2.0, HIGH, HIGH_SECS)] {
        let dist = 0.6;
        let (grip, _) = mantle_grip(dist, height, rule);
        let grip = grip.expect("hand contacts give a grip");
        assert_eq!(grip.skipped, [None, None], "{rule}");
        for (i, h) in grip.hands.iter().enumerate() {
            let h = h.unwrap_or_else(|| panic!("{rule}: hand {i} held"));
            let t = h.target;
            assert!(t.is_finite() && h.gap.is_finite(), "{rule}: {h:?}");
            // Above the top, so outside the box (top at `height`, face at `dist`), on the lip line.
            assert!(t.y > height && t.z >= dist, "{rule}: {t:?}");
            assert!(off_lip_line(t, dist, height) < 0.06, "{rule}: {t:?}");
            // Left hand on the character's left (+X facing +Z), within the shoulder band.
            let side = if i == LEFT { 1.0 } else { -1.0 };
            assert!(t.x * side >= GRIP_LATERAL.0 - 1e-6 && t.x * side <= GRIP_LATERAL.1 + 1e-6, "{rule}: {t:?}");
            // Weights: 0 outside the ramps, 1 through the hold, between on the ramps.
            let w = |t: f32| h.weight(t);
            assert_eq!(w(h.start - GRIP_RAMP - 0.01), 0.0);
            assert_eq!(w(h.end + GRIP_RAMP + 0.01), 0.0);
            assert_eq!(w(h.start), 1.0);
            assert_eq!(w(h.end), 1.0);
            assert_eq!(w(f32::midpoint(h.start, h.end)), 1.0);
            let ramp = w(h.start - 0.5 * GRIP_RAMP);
            assert!(ramp > 0.0 && ramp < 1.0, "{ramp}");
            #[allow(clippy::cast_precision_loss)]
            for k in 0..=200 {
                let x = w(duration * k as f32 / 200.0);
                assert!(x.is_finite() && (0.0..=1.0).contains(&x), "{rule}: {x}");
            }
        }
    }
}

#[test]
fn a_one_handed_clip_mirrors_its_window_to_the_other_hand() {
    let (grip, _) = mantle_grip(0.6, 1.2, MANTLE);
    let [left, right] = grip.expect("grip").hands.map(|h| h.expect("both hands held"));
    assert!(!left.mirrored && right.mirrored);
    assert_eq!((right.start, right.end), (0.167, 0.333));
    assert_eq!((left.start, left.end), (0.167, 0.333));
    assert!((left.target.x + right.target.x).abs() < 1e-5, "mirrored across the lip point: {left:?} {right:?}");
    // The two-handed clip keeps its own windows.
    let (grip, _) = mantle_grip(0.6, 2.0, HIGH);
    let [left, right] = grip.expect("grip").hands.map(|h| h.expect("both hands held"));
    assert!(!left.mirrored && !right.mirrored);
    assert_eq!((left.start, left.end, right.start, right.end), (0.467, 0.900, 0.333, 0.667));
}

#[test]
#[allow(clippy::float_cmp)]
fn hands_far_from_the_lip_are_skipped_and_clips_without_contacts_have_no_grip() {
    // The clip plants the hands ~0.8 m above this lip: IK would drag them down the face.
    let path = waist_gripped();
    let plan = sf_anim::WarpPlan::new(&path, RootPose::IDENTITY, RootPose::new(Vec3::new(0.0, 0.45, 1.3), 0.0), sf_anim::WarpProfile::Late);
    let grip = LedgeGrip::plan(&path, &plan, Vec3::new(0.0, 0.45, 0.3), Vec3::Z).expect("hand contacts");
    assert_eq!(grip.hands, [None, None]);
    for s in grip.skipped {
        assert!(s.expect("skipped, kept for the log").gap > GRIP_MAX_GAP);
    }
    // The plain line clips have no contacts.
    let mut r = Run::new(true);
    r.step(jump_forward(false), Some(ledge(0.6, 1.2, ObstacleClass::Mantle)));
    assert!(r.started(MANTLE));
    assert_eq!(r.state.warp.expect("planned").grip, None);
    assert_eq!(hold_weight(0.1, 0.2, f32::NAN), 0.0);
}

#[test]
#[ignore = "measurement: grip targets vs the lip, and baked-contact gaps, by ledge distance"]
fn measure_grip_targets_against_the_lip() {
    for (height, rule) in [(1.2, MANTLE), (2.0, HIGH)] {
        #[allow(clippy::cast_precision_loss)]
        for k in 3..=10 {
            let dist = k as f32 * 0.1;
            let (grip, _) = mantle_grip(dist, height, rule);
            let grip = grip.expect("grip");
            for (i, name) in [(LEFT, "left"), (RIGHT, "right")] {
                let (h, state) = match (grip.hands[i], grip.skipped[i]) {
                    (Some(h), _) => (h, "held"),
                    (None, Some(h)) => (h, "SKIPPED"),
                    (None, None) => continue,
                };
                println!(
                    "{rule} lip {dist:.1} m {name:5}: target {:?}, to lip line {:.3} m, to lip point {:.3} m, baked gap {:.3} m, {:.3}-{:.3} s{} {state}",
                    h.target,
                    off_lip_line(h.target, dist, height),
                    h.target.distance(Vec3::new(0.0, height, dist)),
                    h.gap,
                    h.start,
                    h.end,
                    if h.mirrored { " (mirrored)" } else { "" },
                );
            }
        }
    }
}
