//! The graph templates, built in code and shipped as RON (`data/`).
//!
//! One builder makes every template, so the humanoid, first-person and
//! creature graphs share one state-machine shape (actors-adapter.md "Graph
//! assets as built"):
//!
//! - **Base layer**: the `locomotion` state machine. Movement states (idle,
//!   walk, run, sneak, turn, jump, swim) are picked by exclusive parameter
//!   conditions; reaction states (hit, knockdown, knockout, death) take
//!   priority and return to whichever movement state the parameters ask for.
//! - **Weapon stance** (humanoids): every movement state is a nested state
//!   machine over `stance`, the Morrowind movement set per weapon (`hh`,
//!   `1h`, `2c`, `2w`); stances without their own movement set (crossbow,
//!   spell) play the base set on the lower body with the stance idle on the
//!   upper body through the `upper` mask, as the original engine does (UE:
//!   Lyra's linked anim layers per weapon over one locomotion machine).
//! - **Lock-on strafe** (humanoids with the manifest's `loco.walk.*` /
//!   `loco.jog.*` slots): while [`params::STRAFING`], the `strafe_walk` and
//!   `strafe_run` states take over from walk and run, an 8-way blend space of
//!   those loops (the stance idle on the upper body for a drawn weapon); its
//!   lanes stay in step by their foot-down markers, so a direction change
//!   keeps the stride's phase. Free movement keeps the Morrowind sets.
//! - **Slots**: `upper` (masked: attacks, casts, equip, block) and `full`
//!   (scripted `PlayGroup`, `LoopGroup`) for humanoids; `full` for creatures.
//! - **Torch arm** (humanoids): the `Torch` group loops on the left arm
//!   (`left_arm` mask) while [`params::TORCH`] is up, over locomotion and the
//!   `upper` slot and under `full`: the original engine's torch overlay
//!   ranks above weapon actions and below scripted groups.
//!
//! Clip references are animation-group names (`WalkForward1h`), a group's
//! sub-action (`WeaponOneHand@chop`), the part before its loop
//! (`Jump@head`) or after it (`Knockout@tail`, `Jump@tail`; `Jump`'s loop is
//! one key time, a hold point); [`crate::AnimSet`] resolves them per
//! skeleton.

use std::collections::BTreeMap;

use sf_anim::RootMotionMode;
use sf_anim_graph::asset::{
    BlendMode, ClipFallback, ClipRef, ConditionSpec as C, GraphAsset, JointRef, LayerMode, LayerSpec, MaskRule, MaskSpec, NodeSpec, OpSpec,
    ParamKind, ParamSpec, Sample2D, StateSpec, TransitionSpec, ValueSpec,
};

/// Which graph to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Template {
    /// NPC skeletons (`base_anim`, `base_anim_female`, `base_animkna`).
    Humanoid,
    /// `base_anim.1st`: the humanoid graph without idle fidgets.
    FirstPerson,
    /// Every creature skeleton; mover modes are parameters.
    Creature,
}

impl Template {
    pub const ALL: [Self; 3] = [Self::Humanoid, Self::FirstPerson, Self::Creature];

    /// The asset's VFS path (as `mw_actors::ActorConfig` names it).
    pub fn path(self) -> &'static str {
        match self {
            Self::Humanoid => "animation/graphs/mw_humanoid.animgraph.ron",
            Self::FirstPerson => "animation/graphs/mw_first_person.animgraph.ron",
            Self::Creature => "animation/graphs/mw_creature.animgraph.ron",
        }
    }

    /// The template's name in the animation-sources manifest (`Slot::graph`).
    pub fn sources_name(self) -> &'static str {
        match self {
            Self::Humanoid => "humanoid",
            Self::FirstPerson => "first_person",
            Self::Creature => "creature",
        }
    }

    fn stances(self) -> &'static [Stance] {
        match self {
            Self::Humanoid | Self::FirstPerson => &Stance::ALL,
            Self::Creature => &[Stance::None],
        }
    }
}

/// Parameter names (the blackboard gameplay writes).
pub mod params {
    /// Vec2: movement direction in the character frame (`x` right, `y`
    /// forward), unit length while moving. Picks the directional clips.
    pub const MOVE: &str = "move";
    pub const MOVING: &str = "moving";
    pub const RUNNING: &str = "running";
    pub const SNEAKING: &str = "sneaking";
    /// In water (swim states). Swim-only creatures keep it set.
    pub const SWIMMING: &str = "swimming";
    /// False while airborne (jump state). Fliers keep it set.
    pub const ON_GROUND: &str = "on_ground";
    /// Airborne from a launch (a jump): the take-off plays. False, a plain
    /// fall (walked off a ledge) holds the airborne pose. Read on leaving the
    /// ground: set it with the launch, clear it on touchdown.
    pub const JUMPING: &str = "jumping";
    /// Turn rate in place, rad/s, positive to the left.
    pub const TURN: &str = "turn";
    /// [`super::Stance`] as a number.
    pub const STANCE: &str = "stance";
    /// Trigger: play idle variant `idle_variant` (2–9) once.
    pub const FIDGET: &str = "fidget";
    pub const IDLE_VARIANT: &str = "idle_variant";
    /// Trigger: play `Hit<hit_variant>` (1–5).
    pub const HIT: &str = "hit";
    pub const HIT_VARIANT: &str = "hit_variant";
    /// Trigger: fall and get up.
    pub const KNOCKDOWN: &str = "knockdown";
    /// Unconscious: loops `Knockout` until cleared, then gets up.
    pub const KNOCKED_OUT: &str = "knocked_out";
    /// Humanoids: the get-up `knockout` leaves to when `knocked_out` clears:
    /// 0 `Knockout@tail`, `i + 1` the state of [`super::GET_UPS`]`[i]` (when
    /// its clip resolved). Set by Delirium's fallen hand-back.
    pub const GET_UP: &str = "get_up";
    pub const DEAD: &str = "dead";
    pub const DEATH_VARIANT: &str = "death_variant";
    /// Humanoids, 0..1: a lit light is carried in the left hand (and shown).
    /// Weights the torch arm layer (the `Torch` group on the left arm).
    pub const TORCH: &str = "torch";
    /// Humanoids with the strafe slots: the body faces something other than its
    /// travel (locked on), so walk and run play the 8-way strafe loops.
    pub const STRAFING: &str = "strafing";
}

/// Slot names.
pub mod slots {
    /// Upper body (humanoids): attacks, casts, equip, block.
    pub const UPPER: &str = "upper";
    /// Full body: scripted groups, creature attacks.
    pub const FULL: &str = "full";
}

/// The `locomotion` machine's movement and reaction state names.
pub mod states {
    pub const IDLE: &str = "idle";
    pub const WALK: &str = "walk";
    pub const RUN: &str = "run";
    /// Locked on (humanoids with the strafe sets): the 8-way walk and jog loops.
    pub const STRAFE_WALK: &str = "strafe_walk";
    pub const STRAFE_RUN: &str = "strafe_run";
    pub const SNEAK: &str = "sneak";
    pub const SNEAK_IDLE: &str = "sneak_idle";
    pub const TURN_LEFT: &str = "turn_left";
    pub const TURN_RIGHT: &str = "turn_right";
    /// Airborne: `Jump@head` (take-off), holding its last pose until
    /// touchdown; the whole `Jump` where the group has no loop point.
    pub const JUMP: &str = "jump";
    /// Airborne without a launch: `Jump@head`'s last pose (its loop point)
    /// held, no take-off (Lyra's `FallLoop`, UE `IsFalling`). Dropped where the
    /// group has no loop point; a fall then enters JUMP as a jump does.
    pub const FALL: &str = "fall";
    /// Touchdown: `Jump@tail` once (knees absorb), then locomotion. Dropped
    /// with its transitions where the group has no loop point.
    pub const LAND: &str = "land";
    pub const SWIM_IDLE: &str = "swim_idle";
    pub const SWIM_WALK: &str = "swim_walk";
    pub const SWIM_RUN: &str = "swim_run";
    pub const SWIM_TURN_LEFT: &str = "swim_turn_left";
    pub const SWIM_TURN_RIGHT: &str = "swim_turn_right";
    pub const KNOCKDOWN: &str = "knockdown";
    pub const SWIM_KNOCKDOWN: &str = "swim_knockdown";
    pub const KNOCKOUT: &str = "knockout";
    pub const KNOCKOUT_END: &str = "knockout_end";
    pub const SWIM_KNOCKOUT: &str = "swim_knockout";
    pub const SWIM_DEATH: &str = "swim_death";
}

/// The machine node of the base layer.
pub const LOCOMOTION: &str = "locomotion";
/// Turn rate (rad/s) above which a standing actor plays a turn.
pub const TURN_THRESHOLD: f32 = 0.5;

/// A weapon stance: which Morrowind movement set plays, and the idle laid
/// over the upper body when the stance has no set of its own. The adapter
/// maps weapon types: bow and thrown → [`OneHand`](Self::OneHand) (the
/// original engine's long-blade fallback), two-handed melee →
/// [`TwoClose`](Self::TwoClose) or [`TwoWide`](Self::TwoWide).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Stance {
    None,
    HandToHand,
    OneHand,
    TwoClose,
    TwoWide,
    /// `1h` movement on the lower body, `IdleCrossbow` on the upper.
    Crossbow,
    /// Base movement on the lower body, `IdleSpell` on the upper.
    Spell,
}

impl Stance {
    pub const ALL: [Self; 7] = [Self::None, Self::HandToHand, Self::OneHand, Self::TwoClose, Self::TwoWide, Self::Crossbow, Self::Spell];

    /// The stance of an actor whose weapon (`None`: none wielded, as
    /// `mw_actors::held::Loadout::weapon_type` gives it) is drawn or
    /// sheathed; a readied spell is [`Spell`](Self::Spell) whatever the
    /// weapon. Sheathed → [`None`](Self::None); drawn without a weapon →
    /// hand-to-hand; short blade, one-handed long blade, blunt and axe, bow
    /// and thrown → [`OneHand`](Self::OneHand) (vanilla has no bow or thrown
    /// movement sets: the original falls back to the long-blade set);
    /// two-handed long blade, blunt and axe → [`TwoClose`](Self::TwoClose);
    /// staff and spear → [`TwoWide`](Self::TwoWide); crossbow →
    /// [`Crossbow`](Self::Crossbow). Ammunition is never wielded
    /// (hand-to-hand).
    pub fn of_weapon(readiness: sf_character::Readiness, weapon: Option<tes3::esp::WeaponType>) -> Self {
        use tes3::esp::WeaponType as W;
        if readiness == sf_character::Readiness::Sheathed {
            return Self::None;
        }
        match weapon {
            None | Some(W::Arrow | W::Bolt) => Self::HandToHand,
            Some(W::ShortBladeOneHand | W::LongBladeOneHand | W::BluntOneHand | W::AxeOneHand | W::MarksmanBow | W::MarksmanThrown) => Self::OneHand,
            Some(W::LongBladeTwoClose | W::BluntTwoClose | W::AxeTwoHand) => Self::TwoClose,
            Some(W::BluntTwoWide | W::SpearTwoWide) => Self::TwoWide,
            Some(W::MarksmanCrossbow) => Self::Crossbow,
        }
    }

    /// The `stance` parameter value.
    #[allow(clippy::cast_precision_loss)]
    pub fn value(self) -> f32 {
        Self::ALL.iter().position(|s| *s == self).expect("listed") as f32
    }

    /// State name in the stance machines.
    pub fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::HandToHand => "hh",
            Self::OneHand => "1h",
            Self::TwoClose => "2c",
            Self::TwoWide => "2w",
            Self::Crossbow => "crossbow",
            Self::Spell => "spell",
        }
    }

    /// Group-name suffix of the movement set.
    fn suffix(self) -> &'static str {
        match self {
            Self::None | Self::Spell => "",
            Self::HandToHand => "hh",
            Self::OneHand | Self::Crossbow => "1h",
            Self::TwoClose => "2c",
            Self::TwoWide => "2w",
        }
    }

    /// The stance's idle group.
    fn idle(self) -> &'static str {
        match self {
            Self::None => "Idle",
            Self::HandToHand => "Idlehh",
            Self::OneHand => "Idle1h",
            Self::TwoClose => "Idle2c",
            Self::TwoWide => "Idle2w",
            Self::Crossbow => "IdleCrossbow",
            Self::Spell => "IdleSpell",
        }
    }

    /// The upper-body overlay over the movement set, if any.
    fn overlay(self) -> Option<&'static str> {
        match self {
            Self::Crossbow => Some("IdleCrossbow"),
            Self::Spell => Some("IdleSpell"),
            _ => None,
        }
    }
}

/// Weapon groups and their sub-actions: the clips the `upper` slot plays.
pub const WEAPON_SECTIONS: &[(&str, &[&str])] = &[
    ("HandToHand", &["equip", "chop", "slash", "thrust", "unequip"]),
    ("WeaponOneHand", &["equip", "chop", "slash", "thrust", "unequip"]),
    ("WeaponTwoHand", &["equip", "chop", "slash", "thrust", "unequip"]),
    ("WeaponTwoWide", &["equip", "chop", "slash", "thrust", "unequip"]),
    ("BowAndArrow", &["equip", "shoot", "unequip"]),
    ("Crossbow", &["equip", "shoot", "unequip"]),
    ("ThrowWeapon", &["equip", "shoot", "unequip"]),
    ("SpellCast", &["equip", "self", "touch", "target", "unequip"]),
    ("Shield", &["block"]),
];

/// The get-ups by how a fallen body lies: `(state, clip slot)`, the
/// [`params::GET_UP`] value is the index + 1 (sources.ron `getup.*`).
pub const GET_UPS: [(&str, &str); 4] = [
    ("get_up_supine", "getup.supine"),
    ("get_up_prone", "getup.prone"),
    ("get_up_side_left", "getup.side_left"),
    ("get_up_side_right", "getup.side_right"),
];

const DIRECTIONS: [(&str, (f32, f32)); 4] =[("Forward", (0.0, 1.0)), ("Back", (0.0, -1.0)), ("Left", (-1.0, 0.0)), ("Right", (1.0, 0.0))];

const DIAG: f32 = std::f32::consts::FRAC_1_SQRT_2;

/// The lock-on strafe sets' directions: the manifest's `loco.<set>.<dir>`
/// slots on the unit circle, `x` right, `y` forward. A unit move direction
/// lies on the octagon's hull, so it blends the two neighbouring loops.
const STRAFE_DIRECTIONS: [(&str, (f32, f32)); 8] = [
    ("fwd", (0.0, 1.0)),
    ("fwd_right", (DIAG, DIAG)),
    ("right", (1.0, 0.0)),
    ("back_right", (DIAG, -DIAG)),
    ("back", (0.0, -1.0)),
    ("back_left", (-DIAG, -DIAG)),
    ("left", (-1.0, 0.0)),
    ("fwd_left", (-DIAG, DIAG)),
];

/// Whether the manifest has the template's strafe sets (`loco.walk.*` and
/// `loco.jog.*`): humanoids only.
fn has_strafe(t: Template) -> bool {
    let slots = &crate::sources::SourcesManifest::builtin().slots;
    t == Template::Humanoid
        && ["walk", "jog"].into_iter().all(|set| {
            STRAFE_DIRECTIONS.iter().any(|(dir, _)| slots.get(&format!("loco.{set}.{dir}")).is_some_and(|s| s.graph == t.sources_name() && !s.candidates.is_empty()))
        })
}

fn range(param: &str, v: f32) -> [C; 2] {
    [C::Greater(param.into(), v - 0.5), C::Less(param.into(), v + 0.5)]
}

/// `Jump@head` → `Jump1h@head`, `Walk` → `Walk1h`: the suffix on the group.
fn with_suffix(base: &str, suffix: &str) -> String {
    base.split_once('@').map_or_else(|| format!("{base}{suffix}"), |(g, part)| format!("{g}{suffix}@{part}"))
}

/// Inertialization between ground and swim gaits (idle, walk, run, turns, sneak), seconds.
/// Short so the body keeps up with a responsive mover (the player: locomotion.md
/// "Player feel"); not under ~0.12 s, where idle-to-run starts pop at the feet.
const GAIT_BLEND: f32 = 0.15;

/// Smoothing of the directional blend spaces' move direction, seconds.
const DIRECTION_SMOOTHING: f32 = 0.05;

fn inert(t: f32) -> BlendMode {
    BlendMode::Inertialize(t)
}

/// A movement state and the conditions that ask for it. The conditions of
/// all movement states are exclusive and cover every parameter combination.
struct Movement {
    state: &'static str,
    when: Vec<C>,
}

fn movements(t: Template) -> Vec<Movement> {
    use params::{JUMPING, MOVING, ON_GROUND, RUNNING, SNEAKING, STRAFING, SWIMMING, TURN};
    let is = |p: &str| C::IsTrue(p.into());
    let not = |p: &str| C::IsFalse(p.into());
    let left = C::Greater(TURN.into(), TURN_THRESHOLD);
    let right = C::Less(TURN.into(), -TURN_THRESHOLD);
    let still = [not(MOVING), C::Less(TURN.into(), TURN_THRESHOLD), C::Greater(TURN.into(), -TURN_THRESHOLD)];
    let land = |extra: &[C]| [&[not(SWIMMING), is(ON_GROUND)][..], extra].concat();
    let swim = |extra: &[C]| [&[is(SWIMMING)][..], extra].concat();
    // Sneaking is a humanoid mode; creatures ignore the flag.
    let sneak = t != Template::Creature;
    let walk_mode = |extra: &[C]| if sneak { land(&[&[not(SNEAKING)][..], extra].concat()) } else { land(extra) };
    // Locked on: walk and run hand over to the strafe states.
    let strafe = has_strafe(t);
    let free = |extra: &[C]| if strafe { walk_mode(&[extra, &[not(STRAFING)][..]].concat()) } else { walk_mode(extra) };
    let mut out = vec![
        Movement { state: states::IDLE, when: walk_mode(&still) },
        Movement { state: states::WALK, when: free(&[is(MOVING), not(RUNNING)]) },
        Movement { state: states::RUN, when: free(&[is(MOVING), is(RUNNING)]) },
        Movement { state: states::TURN_LEFT, when: walk_mode(&[not(MOVING), left.clone()]) },
        Movement { state: states::TURN_RIGHT, when: walk_mode(&[not(MOVING), right.clone()]) },
    ];
    if strafe {
        out.push(Movement { state: states::STRAFE_WALK, when: walk_mode(&[is(MOVING), not(RUNNING), is(STRAFING)]) });
        out.push(Movement { state: states::STRAFE_RUN, when: walk_mode(&[is(MOVING), is(RUNNING), is(STRAFING)]) });
    }
    if sneak {
        out.push(Movement { state: states::SNEAK, when: land(&[is(SNEAKING), is(MOVING)]) });
        out.push(Movement { state: states::SNEAK_IDLE, when: land(&[is(SNEAKING), not(MOVING)]) });
    }
    out.extend([
        Movement { state: states::JUMP, when: vec![not(SWIMMING), not(ON_GROUND), is(JUMPING)] },
        Movement { state: states::FALL, when: vec![not(SWIMMING), not(ON_GROUND), not(JUMPING)] },
        Movement { state: states::SWIM_IDLE, when: swim(&still) },
        Movement { state: states::SWIM_WALK, when: swim(&[is(MOVING), not(RUNNING)]) },
        Movement { state: states::SWIM_RUN, when: swim(&[is(MOVING), is(RUNNING)]) },
        Movement { state: states::SWIM_TURN_LEFT, when: swim(&[not(MOVING), left]) },
        Movement { state: states::SWIM_TURN_RIGHT, when: swim(&[not(MOVING), right]) },
    ]);
    out
}

struct Builder {
    t: Template,
    a: GraphAsset,
}

impl Builder {
    fn param(&mut self, name: &str, kind: ParamKind) {
        self.a.params.push(ParamSpec { name: name.into(), kind });
    }

    /// Registers a clip: required, or optional with a fallback (`None`: drop).
    fn clip(&mut self, reference: &str, optional: bool, fallback: Option<&str>) -> String {
        let r = if optional {
            ClipRef {
                source: reference.into(),
                optional: true,
                fallback: Some(fallback.map_or(ClipFallback::Drop, |f| ClipFallback::Clip(f.into()))),
            }
        } else {
            ClipRef::required(reference)
        };
        self.a.clips.entry(reference.into()).or_insert(r);
        reference.into()
    }

    /// A stance-specific group, falling back to the base set's. A part
    /// reference takes the suffix on its group (`Jump@head` → `Jump1h@head`).
    fn stance_clip(&mut self, base: &str, suffix: &str) -> String {
        let base_ref = self.base_clip(base);
        if suffix.is_empty() {
            return base_ref;
        }
        self.clip(&with_suffix(base, suffix), true, Some(&base_ref))
    }

    /// The clip `hold:<reference>` for a held pose: the stance's part, then
    /// the base set's, then dropped (no fallback to the whole group, whose
    /// end is not the held pose). Own names, so the chain differs from the
    /// plain reference's; the source loads once.
    fn hold_clip(&mut self, base: &str, suffix: &str) -> String {
        let mut entry = |source: &str, fallback: ClipFallback| {
            let name = format!("hold:{source}");
            self.a.clips.entry(name.clone()).or_insert(ClipRef { source: source.into(), optional: true, fallback: Some(fallback) });
            name
        };
        let base_ref = entry(base, ClipFallback::Drop);
        if suffix.is_empty() {
            return base_ref;
        }
        entry(&with_suffix(base, suffix), ClipFallback::Clip(base_ref))
    }

    /// A base-set group with its fallback chain: `Run<Dir>` → `Walk<Dir>`,
    /// `SwimRun<Dir>` → `SwimWalk<Dir>` → `Walk<Dir>` (creatures that swim
    /// or fly with their walk cycle), `<Group>@head` → `<Group>` (a group
    /// without a loop point plays whole).
    fn base_clip(&mut self, base: &str) -> String {
        let fallback = if let Some(dir) = base.strip_prefix("SwimRun") {
            Some(format!("SwimWalk{dir}"))
        } else if let Some(group) = base.strip_suffix("@head") {
            Some(group.to_string())
        } else {
            base.strip_prefix("SwimWalk").or_else(|| base.strip_prefix("Run")).map(|dir| format!("Walk{dir}"))
        };
        let fallback = fallback.map(|f| self.base_clip(&f));
        self.clip(base, base != "Idle", fallback.as_deref())
    }

    fn node(&mut self, name: &str, spec: NodeSpec) -> String {
        self.a.nodes.insert(name.into(), spec);
        name.into()
    }

    fn play(&mut self, name: &str, clip: &str, looping: Option<bool>) -> String {
        self.node(name, NodeSpec::Clip { clip: clip.into(), rate: ValueSpec::default(), looping, start: 0.0, sync: None, root_motion: None })
    }

    /// A directional blend space over `<prefix><Dir><suffix>`.
    fn directional(&mut self, name: &str, prefix: &str, suffix: &str) -> String {
        let samples = DIRECTIONS
            .iter()
            .map(|(dir, at)| {
                let base = format!("{prefix}{dir}");
                Sample2D { clip: self.stance_clip(&base, suffix), at: *at, rate: 1.0 }
            })
            .collect();
        let mv = params::MOVE;
        self.node(
            name,
            NodeSpec::BlendSpace2D {
                x: ValueSpec::Param(format!("{mv}.x")),
                y: ValueSpec::Param(format!("{mv}.y")),
                samples,
                rate: ValueSpec::default(),
                smoothing: DIRECTION_SMOOTHING,
                looping: None,
                sync: None,
                root_motion: None,
            },
        )
    }

    /// `base` with the stance idle over the upper body.
    fn overlay(&mut self, name: &str, base: String, idle: &str) -> String {
        let clip = self.clip(idle, true, Some("Idle"));
        let layer = self.play(&format!("{name}.upper"), &clip, None);
        self.node(
            name,
            NodeSpec::LayeredPerBone {
                base,
                layers: vec![LayerSpec { node: layer, mode: LayerMode::Override, mask: Some("upper".into()), weight: ValueSpec::default() }],
            },
        )
    }

    /// A state machine over the template's stances; `kit` makes each state's node.
    fn by_stance(&mut self, name: &str, mut kit: impl FnMut(&mut Self, Stance, &str) -> String) -> String {
        let stances = self.t.stances();
        if stances.len() == 1 {
            return kit(self, stances[0], name);
        }
        let mut st = Vec::new();
        let mut tr = Vec::new();
        for &s in stances {
            let node = kit(self, s, &format!("{name}.{}", s.name()));
            st.push(StateSpec { name: s.name().into(), node, root_motion: None });
            tr.push(TransitionSpec {
                from: "*".into(),
                to: s.name().into(),
                when: range(params::STANCE, s.value()).to_vec(),
                blend: inert(0.25),
                interruptible: true,
            });
        }
        self.node(name, NodeSpec::StateMachine { initial: None, states: st, transitions: tr })
    }

    /// Idle: the stance idle; unarmed, a fidget trigger plays `Idle<n>` once.
    fn idle(&mut self) -> String {
        let fidgets = self.t == Template::Humanoid || self.t == Template::Creature;
        self.by_stance("idle", |b, s, name| {
            let idle_clip = if s == Stance::None { b.clip("Idle", false, None) } else { b.clip(s.idle(), true, Some("Idle")) };
            if s != Stance::None || !fidgets {
                return b.play(name, &idle_clip, Some(true));
            }
            let base = b.play(&format!("{name}.base"), &idle_clip, Some(true));
            let mut st = vec![StateSpec { name: "base".into(), node: base, root_motion: None }];
            let mut tr = Vec::new();
            for n in 2..=9u8 {
                let clip = b.clip(&format!("Idle{n}"), true, None);
                let node = b.play(&format!("{name}.idle{n}"), &clip, Some(false));
                let state = format!("idle{n}");
                st.push(StateSpec { name: state.clone(), node, root_motion: None });
                let when = [&range(params::IDLE_VARIANT, f32::from(n))[..], &[C::Trigger(params::FIDGET.into())]].concat();
                tr.push(TransitionSpec { from: "base".into(), to: state.clone(), when, blend: inert(0.3), interruptible: true });
                tr.push(TransitionSpec { from: state, to: "base".into(), when: vec![C::Finished], blend: inert(0.3), interruptible: true });
            }
            b.node(name, NodeSpec::StateMachine { initial: Some("base".into()), states: st, transitions: tr })
        })
    }

    /// A strafe state's node (`strafe_walk`, `strafe_run`): the set's 8-way
    /// blend space per stance, a drawn weapon's stance idle over the upper
    /// body. Its own locomotion state, not a mode of `walk`/`run`: on a
    /// skeleton without the Morrowind set (golden saints) walk drops alone
    /// and the strafe state, never asked for by NPCs, changes nothing.
    fn strafing(&mut self, name: &str, set: &str) -> String {
        self.by_stance(name, |b, s, n| {
            if s == Stance::None {
                return b.strafe(n, set);
            }
            let base = b.strafe(&format!("{n}.lower"), set);
            b.overlay(n, base, s.overlay().unwrap_or(s.idle()))
        })
    }

    /// The strafe set's 8-way blend space over `move`; its lanes sync by
    /// their foot-down markers (phase otherwise), as every blend space's.
    fn strafe(&mut self, name: &str, set: &str) -> String {
        let samples = STRAFE_DIRECTIONS
            .iter()
            .map(|(dir, at)| (format!("loco.{set}.{dir}"), *at))
            .filter(|(clip, _)| self.a.clips.contains_key(clip))
            .map(|(clip, at)| Sample2D { clip, at, rate: 1.0 })
            .collect();
        let mv = params::MOVE;
        self.node(
            name,
            NodeSpec::BlendSpace2D {
                x: ValueSpec::Param(format!("{mv}.x")),
                y: ValueSpec::Param(format!("{mv}.y")),
                samples,
                rate: ValueSpec::default(),
                smoothing: DIRECTION_SMOOTHING,
                looping: None,
                sync: None,
                root_motion: None,
            },
        )
    }

    /// Walk, run or sneak: directional per stance.
    fn moving(&mut self, name: &str, prefix: &str) -> String {
        self.by_stance(name, |b, s, n| match s.overlay() {
            Some(idle) => {
                let base = b.directional(&format!("{n}.lower"), prefix, s.suffix());
                b.overlay(n, base, idle)
            }
            None => b.directional(n, prefix, s.suffix()),
        })
    }

    /// One clip per stance (`Turn<Side>`, `Jump`).
    fn single(&mut self, name: &str, group: &str, spell: Option<&str>, looping: Option<bool>) -> String {
        self.by_stance(name, |b, s, n| {
            if let (Stance::Spell, Some(g)) = (s, spell) {
                let fallback = b.clip(group, true, None);
                let c = b.clip(g, true, Some(&fallback));
                return b.play(n, &c, looping);
            }
            let c = b.stance_clip(group, s.suffix());
            if let Some(idle) = s.overlay() {
                let base = b.play(&format!("{n}.lower"), &c, looping);
                b.overlay(n, base, idle)
            } else {
                b.play(n, &c, looping)
            }
        })
    }

    /// One held frame per stance: the last of `part` (`Jump@head`, its loop
    /// point), as a clip started at its end that doesn't loop (clamped).
    fn hold(&mut self, name: &str, part: &str) -> String {
        self.by_stance(name, |b, s, n| {
            let clip = b.hold_clip(part, s.suffix());
            let held = |b: &mut Self, n: &str| {
                b.node(n, NodeSpec::Clip { clip: clip.clone(), rate: ValueSpec::default(), looping: Some(false), start: 1.0, sync: None, root_motion: None })
            };
            if let Some(idle) = s.overlay() {
                let base = held(b, &format!("{n}.lower"));
                b.overlay(n, base, idle)
            } else {
                held(b, n)
            }
        })
    }

    fn plain(&mut self, name: &str, group: &str, fallback: Option<&str>, looping: Option<bool>) -> String {
        let c = self.clip(group, true, fallback);
        self.play(name, &c, looping)
    }

    #[allow(clippy::too_many_lines)]
    fn build(mut self) -> GraphAsset {
        use params as p;
        for (name, kind) in [
            (p::MOVE, ParamKind::Vec2(0.0, 1.0)),
            (p::MOVING, ParamKind::Bool(false)),
            (p::RUNNING, ParamKind::Bool(false)),
            (p::SNEAKING, ParamKind::Bool(false)),
            (p::SWIMMING, ParamKind::Bool(false)),
            (p::ON_GROUND, ParamKind::Bool(true)),
            (p::JUMPING, ParamKind::Bool(false)),
            (p::TURN, ParamKind::Float(0.0)),
            (p::STANCE, ParamKind::Float(0.0)),
            (p::FIDGET, ParamKind::Trigger),
            (p::IDLE_VARIANT, ParamKind::Float(0.0)),
            (p::HIT, ParamKind::Trigger),
            (p::HIT_VARIANT, ParamKind::Float(1.0)),
            (p::KNOCKDOWN, ParamKind::Trigger),
            (p::KNOCKED_OUT, ParamKind::Bool(false)),
            (p::DEAD, ParamKind::Bool(false)),
            (p::DEATH_VARIANT, ParamKind::Float(1.0)),
        ] {
            self.param(name, kind);
        }
        let humanoid = self.t != Template::Creature;
        if humanoid {
            // The original engine's torso + arm groups: the chest's subtree.
            self.a.masks.insert(
                "upper".into(),
                MaskSpec { default: 0.0, rules: vec![MaskRule { joint: JointRef::Rig("chest".into()), weight: 1.0, subtree: true }] },
            );
            // The original engine's left-arm group: the left shoulder's subtree.
            self.a.masks.insert(
                "left_arm".into(),
                MaskSpec { default: 0.0, rules: vec![MaskRule { joint: JointRef::Rig("leftShoulder".into()), weight: 1.0, subtree: true }] },
            );
            self.param(params::TORCH, ParamKind::Float(0.0));
        }

        // Clip slots of the animation-sources manifest (libraries the user
        // brings): optional chains, dropped when no library is present.
        // Registered first: the strafe sets and the get-ups build on them.
        let (source_clips, source_slots) = crate::sources::SourcesManifest::builtin().graph_clips(self.t.sources_name());
        for (name, clip) in source_clips {
            self.a.clips.entry(name).or_insert(clip);
        }
        let strafe = has_strafe(self.t);
        if strafe {
            self.param(params::STRAFING, ParamKind::Bool(false));
        }

        // Movement states.
        let mut st: Vec<StateSpec> = Vec::new();
        let mut state = |name: &str, node: String, rm: Option<RootMotionMode>| st.push(StateSpec { name: name.into(), node, root_motion: rm });
        let idle = self.idle();
        state(states::IDLE, idle, None);
        let walk = self.moving("walk", "Walk");
        state(states::WALK, walk, None);
        let run = self.moving("run", "Run");
        state(states::RUN, run, None);
        if strafe {
            let sw = self.strafing("walk.locked", "walk");
            state(states::STRAFE_WALK, sw, None);
            let sr = self.strafing("run.locked", "jog");
            state(states::STRAFE_RUN, sr, None);
        }
        let tl = self.single("turn_left", "TurnLeft", Some("SpellTurnLeft"), Some(true));
        state(states::TURN_LEFT, tl, None);
        let tr = self.single("turn_right", "TurnRight", Some("SpellTurnRight"), Some(true));
        state(states::TURN_RIGHT, tr, None);
        if humanoid {
            let sneak = self.moving("sneak", "Sneak");
            state(states::SNEAK, sneak, None);
            let si = self.plain("sneak_idle", "IdleSneak", Some("Idle"), Some(true));
            state(states::SNEAK_IDLE, si, None);
        }
        // Jump split at its hold point (OpenMW; UE Lyra's JumpStart/Land):
        // the take-off held while airborne, the landing on touchdown.
        let jump = self.single("jump", "Jump@head", None, Some(false));
        state(states::JUMP, jump, Some(RootMotionMode::Ignore));
        // A fall without a launch holds the airborne pose, no take-off.
        let fall = self.hold("fall", "Jump@head");
        state(states::FALL, fall, Some(RootMotionMode::Ignore));
        let land = self.single("land", "Jump@tail", None, Some(false));
        state(states::LAND, land, Some(RootMotionMode::Ignore));
        let si = self.plain("swim_idle", "IdleSwim", Some("Idle"), Some(true));
        state(states::SWIM_IDLE, si, None);
        let sw = self.directional("swim_walk", "SwimWalk", "");
        state(states::SWIM_WALK, sw, None);
        let sr = self.directional("swim_run", "SwimRun", "");
        state(states::SWIM_RUN, sr, None);
        let stl = self.plain("swim_turn_left", "SwimTurnLeft", Some("TurnLeft"), Some(true));
        state(states::SWIM_TURN_LEFT, stl, None);
        let str_ = self.plain("swim_turn_right", "SwimTurnRight", Some("TurnRight"), Some(true));
        state(states::SWIM_TURN_RIGHT, str_, None);

        // Reaction states: the original accumulates Bip01 travel in these, so they drive.
        let drive = Some(RootMotionMode::Drive);
        let mut hits = Vec::new();
        for n in 1..=5u8 {
            let name = format!("hit{n}");
            let node = self.plain(&name, &format!("Hit{n}"), None, Some(false));
            state(&name, node, drive);
            hits.push((name, f32::from(n)));
        }
        let kd = self.plain("knockdown", "Knockdown", None, Some(false));
        state(states::KNOCKDOWN, kd, drive);
        let skd = self.plain("swim_knockdown", "SwimKnockdown", Some("Knockdown"), Some(false));
        state(states::SWIM_KNOCKDOWN, skd, drive);
        let ko = self.plain("knockout", "Knockout", Some("Knockdown"), Some(true));
        state(states::KNOCKOUT, ko, drive);
        let koe = self.plain("knockout_end", "Knockout@tail", None, Some(false));
        state(states::KNOCKOUT_END, koe, drive);
        // Get-ups by how the body lies (Delirium's fallen hand-back sets
        // `get_up`), on the template's get-up slots; each state drops with
        // its clip (no library: `knockout_end` only).
        let mut get_ups = Vec::new();
        for (v, (name, slot)) in GET_UPS.iter().enumerate() {
            if self.a.clips.contains_key(*slot) {
                let node = self.play(name, slot, Some(false));
                state(name, node, None);
                get_ups.push((*name, v + 1));
            }
        }
        if !get_ups.is_empty() {
            self.param(p::GET_UP, ParamKind::Float(0.0));
        }
        let sko = self.plain("swim_knockout", "SwimKnockout", Some("Knockout"), Some(true));
        state(states::SWIM_KNOCKOUT, sko, drive);
        let mut deaths = Vec::new();
        for n in 1..=5u8 {
            let name = format!("death{n}");
            let node = self.plain(&name, &format!("Death{n}"), None, Some(false));
            state(&name, node, drive);
            deaths.push((name, f32::from(n)));
        }
        let sd = self.plain("swim_death", "SwimDeath", Some("Death1"), Some(false));
        state(states::SWIM_DEATH, sd, drive);

        // Transitions, in priority order.
        let moves = movements(self.t);
        let move_names: Vec<&str> = moves.iter().map(|m| m.state).collect();
        let hit_names: Vec<&str> = hits.iter().map(|(n, _)| n.as_str()).collect();
        let death_names: Vec<&str> = [deaths.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), vec![states::SWIM_DEATH]].concat();
        let knock_once: Vec<&str> = [states::KNOCKDOWN, states::SWIM_KNOCKDOWN, states::KNOCKOUT_END].into_iter().chain(get_ups.iter().map(|g| g.0)).collect();
        let knock_loop = [states::KNOCKOUT, states::SWIM_KNOCKOUT];
        // Movement states plus the landing, which leaves on its own terms.
        let afoot: Vec<&str> = [&move_names[..], &[states::LAND]].concat();
        let living: Vec<&str> = [&afoot[..], &hit_names, &knock_once, &knock_loop].concat();
        let mut tr = Vec::new();
        let mut go = |from: &[&str], to: &str, when: Vec<C>, blend: BlendMode| {
            // A hit re-hits itself (restarts); other states never self-transition.
            let restart = to.starts_with("hit");
            for f in from.iter().filter(|f| restart || **f != to) {
                tr.push(TransitionSpec { from: (*f).into(), to: to.into(), when: when.clone(), blend, interruptible: true });
            }
        };
        let is = |x: &str| C::IsTrue(x.into());
        let not = |x: &str| C::IsFalse(x.into());
        // Death.
        go(&living, states::SWIM_DEATH, vec![is(p::DEAD), is(p::SWIMMING)], inert(0.15));
        for (name, v) in &deaths {
            go(&living, name, [&[is(p::DEAD), not(p::SWIMMING)][..], &range(p::DEATH_VARIANT, *v)].concat(), inert(0.15));
        }
        go(&living, "death1", vec![is(p::DEAD), not(p::SWIMMING)], inert(0.15));
        // Knockout (held), knockdown (once), hits (once, by variant).
        let up: Vec<&str> = [&afoot[..], &hit_names, &knock_once].concat();
        go(&up, states::SWIM_KNOCKOUT, vec![is(p::KNOCKED_OUT), is(p::SWIMMING)], inert(0.2));
        go(&up, states::KNOCKOUT, vec![is(p::KNOCKED_OUT), not(p::SWIMMING)], inert(0.2));
        for (name, v) in &get_ups {
            #[allow(clippy::cast_precision_loss)]
            let when = [&[not(p::KNOCKED_OUT)][..], &range(p::GET_UP, *v as f32)].concat();
            go(&[states::KNOCKOUT], name, when, inert(0.1));
        }
        go(&[states::KNOCKOUT], states::KNOCKOUT_END, vec![not(p::KNOCKED_OUT)], inert(0.1));
        let standing: Vec<&str> = [&afoot[..], &hit_names].concat();
        go(&standing, states::SWIM_KNOCKDOWN, vec![C::Trigger(p::KNOCKDOWN.into()), is(p::SWIMMING)], inert(0.15));
        go(&standing, states::KNOCKDOWN, vec![C::Trigger(p::KNOCKDOWN.into()), not(p::SWIMMING)], inert(0.15));
        for (name, v) in &hits {
            // A new hit restarts the reaction, even the same one.
            go(&standing, name, [&range(p::HIT_VARIANT, *v)[..], &[C::Trigger(p::HIT.into())]].concat(), inert(0.1));
        }
        // Landing: touchdown from the jump; airborne again jumps; a run cuts it
        // (Lyra lets locomotion interrupt a land at speed); else it ends below.
        let when_of = |s: &str| moves.iter().find(|m| m.state == s).map(|m| m.when.clone()).unwrap_or_default();
        go(&[states::JUMP, states::FALL], states::LAND, vec![not(p::SWIMMING), is(p::ON_GROUND)], inert(0.1));
        go(&[states::LAND], states::JUMP, when_of(states::JUMP), inert(0.1));
        go(&[states::LAND], states::FALL, when_of(states::FALL), inert(0.1));
        // Without FALL (no loop point: dropped with its transitions), a fall jumps.
        let airborne = vec![not(p::SWIMMING), not(p::ON_GROUND)];
        go(&[states::LAND], states::JUMP, airborne.clone(), inert(0.1));
        for run in moves.iter().filter(|m| m.state == states::RUN || m.state == states::STRAFE_RUN) {
            go(&[states::LAND], run.state, run.when.clone(), inert(0.2));
        }
        // Movement: exclusive targets; reactions return when they end.
        for m in &moves {
            go(&[states::LAND], m.state, [&[C::Finished][..], &m.when].concat(), inert(0.2));
            // JUMP and FALL don't swap in the air: `jumping` counts on leaving the ground.
            let from: Vec<&str> = match m.state {
                states::JUMP => move_names.iter().copied().filter(|&f| f != states::FALL).collect(),
                states::FALL => move_names.iter().copied().filter(|&f| f != states::JUMP).collect(),
                _ => move_names.clone(),
            };
            let blend = if matches!(m.state, states::JUMP | states::FALL) { 0.2 } else { GAIT_BLEND };
            go(&from, m.state, m.when.clone(), inert(blend));
            let once: Vec<&str> = [&hit_names[..], &knock_once].concat();
            go(&once, m.state, [&[C::Finished][..], &m.when].concat(), inert(0.25));
            // Knockout's exit when it has no `Knockout@tail` (a dropped
            // `knockout_end` takes its transitions with it; AN-13: the guar
            // looped `Knockout` forever after the get-up). Below `knockout_end`.
            go(&knock_loop, m.state, [&[not(p::KNOCKED_OUT)][..], &m.when].concat(), inert(0.25));
            go(&death_names, m.state, [&[not(p::DEAD)][..], &m.when].concat(), inert(0.3));
        }
        // FALL's stand-in where it dropped, below its own entry (never fires beside it).
        let grounded: Vec<&str> = move_names.iter().copied().filter(|&f| f != states::FALL).collect();
        go(&grounded, states::JUMP, airborne, inert(0.2));
        self.a.nodes.insert(LOCOMOTION.into(), NodeSpec::StateMachine { initial: Some(states::IDLE.into()), states: st, transitions: tr });

        // Slots: upper (masked) then full body, as UE's slot order.
        let mut top = LOCOMOTION.to_string();
        if humanoid {
            top = self.node(
                "upper_slot",
                NodeSpec::Slot { slot: slots::UPPER.into(), input: top, group: Some(slots::UPPER.into()), mask: Some("upper".into()) },
            );
            // The torch arm: `Torch` (its start..stop range, looped) on the
            // left arm while a lit light is carried. Skeletons without the
            // group (optional, dropped) keep the arm as the layers below.
            let clip = self.clip("Torch", true, None);
            let arm = self.play("torch_arm", &clip, Some(true));
            let weight = ValueSpec::Param(params::TORCH.into());
            let layers = vec![LayerSpec { node: arm, mode: LayerMode::Override, mask: Some("left_arm".into()), weight }];
            top = self.node("torch_layer", NodeSpec::LayeredPerBone { base: top, layers });
        }
        top = self.node("full_slot", NodeSpec::Slot { slot: slots::FULL.into(), input: top, group: Some(slots::FULL.into()), mask: None });
        self.a.layers = vec![LayerSpec { node: top, mode: LayerMode::Override, mask: None, weight: ValueSpec::default() }];

        // Clips gameplay plays by name: weapon sub-actions, creature attacks.
        let mut slot_clips = Vec::new();
        for (group, sections) in WEAPON_SECTIONS {
            for s in *sections {
                slot_clips.push(self.clip(&format!("{group}@{s}"), true, None));
            }
        }
        if !humanoid {
            for n in 1..=3 {
                slot_clips.push(self.clip(&format!("Attack{n}"), true, None));
            }
        }
        // The animation-sources slots (clips registered with the get-ups).
        // Locomotion loops play in the movement states, not in a montage slot.
        slot_clips.extend(source_slots.into_iter().filter(|s| !s.starts_with("loco.")));
        self.a.slot_clips = slot_clips;
        // Playback follows the measured speed over the cycle's authored root
        // speed (animfix 2026-10-12: walk 2.2 m/s, run 3.2 m/s; NPC walkers
        // 1.9-2.4 m/s, slower while avoiding). 0.5-1.5 keeps the cadence
        // believable; stride warping takes the rest so feet don't slide. Humanoids
        // reach 2.2: sidestep and backpedal clips' planted feet move at a third to
        // half of the body speed (`ClipMeta::stance_velocity`), and a strafe-run
        // needs that much playback for the foot lock to hold (anim slice 4b).
        self.a.procedural = vec![OpSpec::SpeedMatch {
            min: 0.5,
            max: if matches!(self.t, Template::Humanoid) { 2.2 } else { 1.5 },
            // Humanoids shorten their steps as they slow down (AN-16; owner 0.3, 2026-10-22),
            // cadence and step length about evenly (0.5, gait bands 2026-10-26: an NPC at
            // 2.2 m/s on the 1.9 m/s walk drops the pelvis 21 mm, 32 at 0.3). Speed-ups past
            // the stride warp's 1.2 go to the rate: the player's 7 m/s sprint plays the
            // 2.67 m/s run at 2.2 (4.75 steps/s; a sprint cycle would calm it). Run strafes
            // plant at 0.75-0.9 m/s and outrun any rate (`gait_bands` in tests/slope_walk.rs).
            rate_share: if matches!(self.t, Template::Humanoid) { 0.5 } else { 1.0 },
            max_lod: 2,
        }];
        if matches!(self.t, Template::Humanoid) {
            // Feet on the ground (walkers, 2026-10-12): legs from the rig
            // profile (the humanoid rig map's), the pelvis by its humanoid
            // bone; the host probes the ground at the graph's `probe_origins`.
            self.a.profile_limbs = sf_anim_graph::asset::ProfileLimbs::Support;
            // Slopes (AN-21): shorter steps uphill (26% at 30°), longer downhill
            // (14%), the cadence taking the rest. Steps shorten freely (0.6) but lengthen
            // by 20% at most (Lyra/GASP: ~1.1-1.15): longer steps need a pelvis drop (95 mm
            // at the old 1.6 cap, player run 4.5 m/s: a crouch; under 1 mm at 1.2, 26 at 1.3).
            // 1.15 let NPC run strafes skate (3.6 cm per stance).
            self.a.procedural.push(ron::from_str("StrideWarp(pelvis: Rig(\"hips\"), max: 1.2, grade: (up: 0.45, down: 0.25))").expect("StrideWarp defaults"));
            // Foot IK fades out in the air: the player feeds a debounced `on_ground`
            // (NPCs leave it at its default, true).
            self.a.procedural.push(ron::from_str("FootPlacement(pelvis: Rig(\"hips\"), gate: Some(\"on_ground\"))").expect("FootPlacement defaults"));
            // A subtle trunk lean into the slope (AN-21): 4° forward at 30° uphill,
            // 2° back downhill, over the chest and neck: the thighs hang off `Bip01 Spine`
            // (biped rigs), so leaning it would swing the placed legs.
            // No turn roll (gain 0): flat walking stays as authored.
            self.a.procedural.push(
                ron::from_str("Lean(joints: [(Rig(\"chest\"), 0.7), (Rig(\"neck\"), 0.3)], gain: 0.0, grade: (up: 0.4, down: 0.12, max: 0.21), halflife: 0.3)").expect("Lean defaults"),
            );
        }
        // Secondary motion (M4 P9), after the feet: hits kick springs on the
        // spine, neck and arms (the rig profile's; physics.rs sends impacts
        // below the knockdown threshold), and tails (guars, beast races) swing.
        // Not in first person: the view model's arms stay steady.
        if !matches!(self.t, Template::FirstPerson) {
            self.a.procedural.push(ron::from_str("HitReact()").expect("HitReact defaults"));
            self.a.procedural.push(ron::from_str("SpringBones()").expect("SpringBones defaults"));
        }
        self.a
    }
}

/// Builds a template's graph asset.
pub fn asset(t: Template) -> GraphAsset {
    Builder { t, a: GraphAsset { clips: BTreeMap::new(), ..GraphAsset::default() } }.build()
}

/// The asset as RON, as shipped in `data/`.
pub fn to_ron(asset: &GraphAsset) -> String {
    let config = ron::ser::PrettyConfig::new().depth_limit(2).indentor("  ").new_line("\n");
    let mut text = ron::ser::to_string_pretty(asset, config).expect("graph assets serialize");
    text.push('\n');
    text
}

#[cfg(test)]
pub(crate) fn movement_conditions(t: Template) -> Vec<(&'static str, Vec<C>)> {
    movements(t).into_iter().map(|m| (m.state, m.when)).collect()
}
