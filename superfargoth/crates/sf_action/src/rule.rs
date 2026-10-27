//! Move rules: data records (RON), compiled into a priority-ordered [`RuleSet`] with load-time
//! validation that names the mod (action.md "Move rule (data)").

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::command::{ActionClass, ClassSet};
use crate::ids::{MovesetKey, RuleKey, SlotKey, TagId};
use crate::windows::SlotSource;

/// What a move is (presentation and tooling; the chooser orders by [`Category`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoveKind {
    Locomotion,
    Sneak,
    Air,
    Traversal,
    Docked,
    Combat,
    Paired,
    Interaction,
    Forced,
}

/// The priority list, highest first (action.md "Priority list"): declaration order is priority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Category {
    Forced,
    Landing,
    AirCatch,
    Paired,
    Dodge,
    Attack,
    Guard,
    Traversal,
    Jump,
    Interact,
    Passover,
    Locomotion,
}

impl Category {
    /// Framework-only categories: a mod rule can't enter above `Paired`.
    pub fn framework_only(self) -> bool {
        self < Self::Paired
    }

    /// The window policy a rule with `windows: Category` gets when its slot authors none.
    /// A1 knows loops, air and landings (unrestricted); traversal, interaction and forced moves
    /// commit until they end. Combat defaults (light/heavy/dodge fractions) arrive with E1.
    pub fn default_policy(self) -> WindowPolicy {
        match self {
            Self::Landing | Self::Jump | Self::Locomotion | Self::Passover => WindowPolicy::Unrestricted,
            _ => WindowPolicy::Committed,
        }
    }
}

/// Who moves the gameplay body (action.md "Motion owners"). A1 built `Sim`, B2a `Warp`;
/// `Docked`, `Clip` and `Physics` arrive with B3, E1 and the ragdoll work.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Owner {
    #[default]
    Sim,
    /// The slot chain's baked root track, corrected onto `target` by a [`sf_anim::WarpPlan`] fixed
    /// at start; refused (next candidate) when the correction is over `refuse_ratio` of the
    /// chain's root distance (`MaxCorrection`). No gravity, no steering while it plays.
    Warp { target: WarpTarget, profile: sf_anim::WarpProfile, refuse_ratio: f32, steer: bool },
    /// The slot chain's baked root track at the action clock, yaw separate from translation
    /// (action.md "Motion owners": Clip): translation is the root delta over the tick x
    /// `root_scale`, rotated by the gameplay yaw; yaw turns at the open `Turn` window's rate toward
    /// the lock target, else the stick, never inside `NoTurn`. Gravity stays on (the shell's mover).
    /// `rate`: clip seconds per move second (slot lengths are move time; root tracks clip time).
    Clip {
        root_scale: f32,
        square_up: SquareUp,
        #[serde(default = "one")]
        rate: f32,
    },
}

fn one() -> f32 {
    1.0
}

/// A Clip move's facing on its first tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SquareUp {
    /// Keep the current facing.
    #[default]
    Keep,
    /// Snap to the stick when it is active (unlocked rolls).
    Stick,
    /// Face the lock target ([`crate::Sense::lock_target`]; locked side rolls, whose clip carries
    /// the body sideways); [`SquareUp::Stick`] when not locked on.
    Lock,
}

/// Where the active stick points relative to the lock target's direction ([`Entry::lock`]):
/// front and back within +-45 deg, as `sf_combat::Side` classifies hits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StickSide {
    Front,
    Back,
    Left,
    Right,
}

impl StickSide {
    /// The side of `toward` (planar: world x, z) that `stick` points at; `None` when either is zero.
    pub fn of(toward: glam::Vec2, stick: glam::Vec2) -> Option<Self> {
        let fwd = toward.try_normalize()?;
        let stick = stick.try_normalize()?;
        // Right of a facing (sin y, cos y) is (-cos y, sin y): (x, z) -> (-z, x).
        let (f, r) = (stick.dot(fwd), stick.dot(glam::Vec2::new(-fwd.y, fwd.x)));
        Some(if f.abs() >= r.abs() {
            if f >= 0.0 { Self::Front } else { Self::Back }
        } else if r >= 0.0 {
            Self::Right
        } else {
            Self::Left
        })
    }
}

/// Where a Warp owner's plan lands.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WarpTarget {
    /// On top of the sensed obstacle, `inset` m past the lip (less if the top is shallower),
    /// facing into its face.
    LedgeTop { inset: f32 },
}

/// Obstacle classes the shell's context probe reports (`sf_controller::Affordance`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObstacleClass {
    Step,
    Vault,
    Mantle,
    Climb,
}

/// A world query a rule needs to pass (action.md "Move rule": `probe`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProbeNeed {
    /// An obstacle ahead of one of these classes whose top is within `height`, in character
    /// heights ([`crate::CharacterParams::height`]).
    Obstacle { classes: Vec<ObstacleClass>, height: (f32, f32) },
}

/// Movement profile (action.md "Mode profiles"). `Combat` stance layers over these.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Normal,
    Sneak,
    FreeRun,
}

/// Body-state predicates. `Landed` is the touchdown tick (also `Grounded`); `Coyote` is the
/// grace window after walking off an edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phase {
    Grounded,
    Coyote,
    AirRising,
    AirFalling,
    Landed,
}

/// Entry predicates: fixed vocabulary, all must pass (empty lists pass).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Entry {
    /// Request classes that start it; empty = automatic (or forced in framework categories).
    pub classes: Vec<ActionClass>,
    /// Needs the stick within 60° of the facing.
    pub need_forward: bool,
    pub modes: Vec<Mode>,
    /// Any of these.
    pub states: Vec<Phase>,
    /// Planar speed band, m/s.
    pub speed: Option<(f32, f32)>,
    /// Vertical speed band, m/s; on the `Landed` tick this is the impact speed.
    pub vspeed: Option<(f32, f32)>,
    /// Modes in which a requested rule also fires with no press (passovers, B5).
    pub auto_in: Vec<Mode>,
    /// Needs an interaction focus: `Some([])` any, `Some(tags)` one of these tags.
    pub focus: Option<Vec<String>>,
    /// Needs a sensed obstacle (B2a: ledges for mantles).
    pub probe: Option<ProbeNeed>,
    /// Starts only while the character's moveset ([`crate::ModeState::moveset`]) is this one.
    pub moveset: Option<String>,
    /// The move it may start from (any of these; empty = any): combo chains and openers.
    pub from: Vec<Prev>,
    /// Needs a lock-on target with the active stick on this side of it ([`StickSide::of`]; C1c
    /// locked rolls).
    pub lock: Option<StickSide>,
}

/// A move a rule may start from ([`Entry::from`]). Not `From`, which would shadow the std trait.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Prev {
    Any,
    /// An automatic Locomotion move (or none).
    Loco,
    /// This rule (id).
    Rule(String),
}

/// [`Prev`], compiled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrevKey {
    Any,
    Loco,
    Rule(RuleKey),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowPolicy {
    /// The category's default ([`Category::default_policy`]).
    #[default]
    Category,
    /// Accept and cancel every class.
    Unrestricted,
    /// Accept every class, cancel none until the move ends (hard landing, heavy whiff).
    Committed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitOn {
    /// The slot chain ended (never for a looping slot).
    End,
    /// The body touched down this tick.
    Touchdown,
    /// The body is off the ground (walked off an edge, knocked up).
    Airborne,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitTo {
    /// The automatic pick (locomotion, fall).
    Loco,
    Rule(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exit {
    pub on: ExitOn,
    pub to: ExitTo,
}

/// A vertical launch on the move's first tick.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Launch {
    /// The character's jump height (stat band × mode profile): `v = sqrt(2 g h)`.
    JumpHeight,
    /// A fixed vertical speed, m/s.
    Speed(f32),
}

/// One move, as data. Patched field-level like other records once the shell registers it as an
/// `sf_records` kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveRule {
    pub id: String,
    pub kind: MoveKind,
    pub category: Category,
    /// Order inside the category, higher first; ties by load order.
    #[serde(default)]
    pub rank: i32,
    /// Clip chain; empty = an instant move (zero length).
    #[serde(default)]
    pub slots: Vec<String>,
    #[serde(default)]
    pub owner: Owner,
    #[serde(default)]
    pub entry: Entry,
    #[serde(default)]
    pub windows: WindowPolicy,
    #[serde(default)]
    pub exits: Vec<Exit>,
    #[serde(default)]
    pub launch: Option<Launch>,
    /// Sim owner: the stick steers during the move. `false` stops the character (hard landing).
    #[serde(default = "yes")]
    pub steer: bool,
    /// Ticks before this rule may start again.
    #[serde(default)]
    pub redo_ticks: u16,
}

fn yes() -> bool {
    true
}

/// Where a rule came from, for reports and the mod category limit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Origin {
    Framework,
    Mod(String),
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Framework => f.write_str("framework"),
            Self::Mod(m) => write!(f, "mod '{m}'"),
        }
    }
}

/// A load or validation problem, naming the mod and rule.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{origin}{}: {message}", .rule.as_ref().map(|r| format!(", rule '{r}'")).unwrap_or_default())]
pub struct LoadError {
    pub origin: Origin,
    pub rule: Option<String>,
    pub message: String,
}

/// A rule with its ids hashed and exits resolved.
#[derive(Clone, Debug)]
pub struct CompiledRule {
    pub key: RuleKey,
    pub rule: MoveRule,
    pub origin: Origin,
    pub classes: ClassSet,
    pub slots: Vec<SlotKey>,
    pub focus: Option<Vec<TagId>>,
    /// Exit triggers with their target's index (`None` = `Loco`).
    pub exits: Vec<(ExitOn, Option<usize>)>,
    pub policy: WindowPolicy,
    pub moveset: Option<MovesetKey>,
    /// Empty = from any move.
    pub from: Vec<PrevKey>,
}

impl CompiledRule {
    /// Started only through [`crate::ActionState::forced`] (reactions): `kind: Forced`.
    pub fn force_only(&self) -> bool {
        self.rule.kind == MoveKind::Forced
    }
}

impl CompiledRule {
    pub fn id(&self) -> &str {
        &self.rule.id
    }
}

/// Collects rule files in load order; a later rule with the same id replaces the earlier one.
#[derive(Default)]
pub struct RuleSetBuilder {
    rules: Vec<(Origin, MoveRule)>,
    errors: Vec<LoadError>,
}

impl RuleSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a RON list of rules (`[ (id: ..), .. ]`).
    pub fn add_ron(&mut self, origin: &Origin, text: &str) -> &mut Self {
        match ron::from_str::<Vec<MoveRule>>(text) {
            Ok(rules) => {
                for r in rules {
                    self.add(origin.clone(), r);
                }
            }
            Err(e) => self.errors.push(LoadError { origin: origin.clone(), rule: None, message: e.to_string() }),
        }
        self
    }

    pub fn add(&mut self, origin: Origin, rule: MoveRule) -> &mut Self {
        if let Some(slot) = self.rules.iter_mut().find(|(_, r)| r.id == rule.id) {
            *slot = (origin, rule);
        } else {
            self.rules.push((origin, rule));
        }
        self
    }

    /// Validates and compiles; every problem is listed.
    #[allow(clippy::too_many_lines)]
    pub fn build(&self) -> Result<RuleSet, Vec<LoadError>> {
        let mut errors = self.errors.clone();
        let mut by_key: HashMap<RuleKey, usize> = HashMap::new();
        let err = |o: &Origin, r: &MoveRule, m: String| LoadError { origin: o.clone(), rule: Some(r.id.clone()), message: m };
        for (i, (origin, r)) in self.rules.iter().enumerate() {
            let key = RuleKey::of(&r.id);
            if let Some(&j) = by_key.get(&key) {
                errors.push(err(origin, r, format!("id hash collides with rule '{}'; rename one", self.rules[j].1.id)));
            }
            by_key.insert(key, i);
            if r.id.is_empty() {
                errors.push(err(origin, r, "empty id".into()));
            }
            if matches!(origin, Origin::Mod(_)) && r.category.framework_only() && r.kind != MoveKind::Forced {
                errors.push(err(origin, r, format!("category {:?} is framework-only; mods start at Paired", r.category)));
            }
            for (name, band) in [("speed", r.entry.speed), ("vspeed", r.entry.vspeed)] {
                if band.is_some_and(|(lo, hi)| lo.is_nan() || hi.is_nan() || lo > hi) {
                    errors.push(err(origin, r, format!("{name} band is empty or NaN")));
                }
            }
            if !r.entry.auto_in.is_empty() && r.entry.classes.is_empty() {
                errors.push(err(origin, r, "auto_in needs request classes (an automatic rule fires anyway)".into()));
            }
            if r.entry.classes.iter().any(|c| !ClassSet::LATCHABLE.contains(*c)) {
                errors.push(err(origin, r, "Move, Sprint and Sneak are state classes, not requests".into()));
            }
            if r.entry.focus.is_some() && !r.entry.classes.contains(&ActionClass::Interact) {
                errors.push(err(origin, r, "focus needs the Interact class".into()));
            }
            if let Owner::Warp { refuse_ratio, target: WarpTarget::LedgeTop { inset }, .. } = r.owner {
                if !(refuse_ratio.is_finite() && refuse_ratio >= 0.0 && inset.is_finite()) {
                    errors.push(err(origin, r, "warp refuse_ratio and inset must be finite, ratio ≥ 0".into()));
                }
                if r.slots.is_empty() || r.launch.is_some() {
                    errors.push(err(origin, r, "a Warp owner needs slots (its root path) and no launch".into()));
                }
            }
            if let Owner::Clip { root_scale, rate, .. } = r.owner
                && !(root_scale.is_finite() && root_scale >= 0.0 && rate.is_finite() && rate > 0.0)
            {
                errors.push(err(origin, r, "clip root_scale must be finite and >= 0, rate finite and > 0".into()));
            }
            if r.kind == MoveKind::Forced && !r.entry.classes.is_empty() {
                errors.push(err(origin, r, "a Forced-kind move starts only when forced: no request classes".into()));
            }
            if r.entry.moveset.as_deref() == Some("") {
                errors.push(err(origin, r, "empty moveset".into()));
            }
            if let Some(ProbeNeed::Obstacle { height: (lo, hi), .. }) = r.entry.probe
                && (lo.is_nan() || hi.is_nan() || lo > hi)
            {
                errors.push(err(origin, r, "probe height band is empty or NaN".into()));
            }
            if let Some(Launch::Speed(v)) = r.launch
                && !(v.is_finite() && v > 0.0)
            {
                errors.push(err(origin, r, "launch speed must be positive".into()));
            }
        }
        let mut rules = Vec::with_capacity(self.rules.len());
        for (origin, r) in &self.rules {
            let mut exits = Vec::with_capacity(r.exits.len());
            for e in &r.exits {
                let to = match &e.to {
                    ExitTo::Loco => None,
                    ExitTo::Rule(id) => {
                        let Some(&j) = by_key.get(&RuleKey::of(id)) else {
                            errors.push(err(origin, r, format!("exit to unknown rule '{id}'")));
                            continue;
                        };
                        Some(j)
                    }
                };
                exits.push((e.on, to));
            }
            rules.push(CompiledRule {
                key: RuleKey::of(&r.id),
                origin: origin.clone(),
                classes: ClassSet::of(&r.entry.classes),
                slots: r.slots.iter().map(|s| SlotKey::of(s)).collect(),
                focus: r.entry.focus.as_ref().map(|tags| tags.iter().map(|t| TagId::of(t)).collect()),
                exits,
                policy: match r.windows {
                    WindowPolicy::Category => r.category.default_policy(),
                    p => p,
                },
                moveset: r.entry.moveset.as_deref().map(MovesetKey::of),
                from: r
                    .entry
                    .from
                    .iter()
                    .map(|p| match p {
                        Prev::Any => PrevKey::Any,
                        Prev::Loco => PrevKey::Loco,
                        Prev::Rule(id) => PrevKey::Rule(RuleKey::of(id)),
                    })
                    .collect(),
                rule: r.clone(),
            });
        }
        if !rules.iter().any(|r| r.rule.category == Category::Locomotion && r.classes.is_empty()) {
            errors.push(LoadError {
                origin: Origin::Framework,
                rule: None,
                message: "no automatic Locomotion rule: the priority list needs a fallback".into(),
            });
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let mut order: Vec<usize> = (0..rules.len()).collect();
        order.sort_by_key(|&i| (rules[i].rule.category, std::cmp::Reverse(rules[i].rule.rank), i));
        let index = rules.iter().enumerate().map(|(i, r)| (r.key, i)).collect();
        Ok(RuleSet { rules, order, index })
    }
}

/// Compiled rules in priority order.
#[derive(Clone, Debug)]
pub struct RuleSet {
    rules: Vec<CompiledRule>,
    order: Vec<usize>,
    index: HashMap<RuleKey, usize>,
}

impl RuleSet {
    /// One RON source from the framework: a convenience for tests and tools.
    pub fn from_ron(text: &str) -> Result<Self, Vec<LoadError>> {
        RuleSetBuilder::new().add_ron(&Origin::Framework, text).build()
    }

    pub fn get(&self, key: RuleKey) -> Option<&CompiledRule> {
        self.index.get(&key).map(|&i| &self.rules[i])
    }

    pub(crate) fn index_of(&self, key: RuleKey) -> Option<usize> {
        self.index.get(&key).copied()
    }

    pub(crate) fn at(&self, i: usize) -> &CompiledRule {
        &self.rules[i]
    }

    /// Rule indices, highest priority first.
    pub(crate) fn priority(&self) -> &[usize] {
        &self.order
    }

    /// Rules, highest priority first.
    pub fn iter(&self) -> impl Iterator<Item = &CompiledRule> {
        self.order.iter().map(|&i| &self.rules[i])
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Slots no source provides: load errors once clips are known (action.md "unknown slots").
    pub fn check_slots(&self, slots: &dyn SlotSource) -> Vec<LoadError> {
        let mut out = Vec::new();
        for r in &self.rules {
            for (name, key) in r.rule.slots.iter().zip(&r.slots) {
                if slots.slot(*key).is_none() {
                    out.push(LoadError { origin: r.origin.clone(), rule: Some(r.rule.id.clone()), message: format!("unknown slot '{name}'") });
                }
            }
        }
        out
    }
}
