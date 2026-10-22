//! The authoring form of a graph: `*.animgraph.ron` (animation.md "Graph
//! asset"). Names everywhere; `compile` resolves them per skeleton.
//!
//! Nodes are a closed enum, so a graph is data: mods replace or patch the
//! file like any other content. New node kinds are framework code.
//!
//! ```ron
//! (
//!     params: [(name: "speed", kind: Float(0.0)), (name: "attack", kind: Trigger)],
//!     clips: {
//!         "walk": "anims/walk.glb#walk",
//!         "run": "anims/run.glb#run",
//!         // Bring-your-own source: falls back to "walk" when missing.
//!         "walk_mocap": (source: "byo/mocap.glb#walk", optional: true, fallback: Clip("walk")),
//!     },
//!     masks: { "upper": (rules: [(joint: Tag("spine"), weight: 1.0)]) },
//!     nodes: {
//!         "move": BlendSpace1D(x: "speed", samples: [(clip: "walk", at: 1.2), (clip: "run", at: 3.5)],
//!                              sync: (group: "feet")),
//!     },
//!     layers: [(node: "move")],
//! )
//! ```

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_anim::{LimbSolver, RootMotionMode};

/// A whole graph asset.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GraphAsset {
    /// The blackboard: typed parameters written by gameplay, AI and scripts.
    pub params: Vec<ParamSpec>,
    /// Local clip name → clip reference. The default reference form is
    /// `<file>#<clip>` (an `anim.cook` product on the graph's skeleton);
    /// adapters may map references differently (`GraphRules`). A plain
    /// string is required; `(source: .., optional: true, fallback: ..)`
    /// may be missing ([`ClipRef`]).
    pub clips: BTreeMap<String, ClipRef>,
    /// Weighted per-joint masks (UE blend profiles), by name.
    pub masks: BTreeMap<String, MaskSpec>,
    pub nodes: BTreeMap<String, NodeSpec>,
    /// Evaluated bottom-up: the first layer is the base.
    pub layers: Vec<LayerSpec>,
    /// Limbs for the procedural ops, by name (legs, arms, wings: the name
    /// is historical; `limbs` is accepted too).
    #[serde(alias = "limbs")]
    pub legs: Vec<LegSpec>,
    /// Also take limbs from the skeleton's rig profile (rig-profile.md), by
    /// their profile names, so one graph serves every skeleton of a template.
    pub profile_limbs: ProfileLimbs,
    /// Warp target slots (animation.md "Motion warping"), filled per update
    /// by gameplay; clip `warp` windows name them.
    pub warp_targets: Vec<String>,
    /// Procedural ops run after the graph, in order.
    pub procedural: Vec<OpSpec>,
    /// Left/right pairs for `Mirror` nodes (added to the adapter's pairs).
    pub mirror: MirrorSpec,
    /// Clips (names in `clips`) that gameplay may play in `Slot` nodes by
    /// name without any node playing them (animation.md "Slots and
    /// montages"). An optional one that is missing is simply not offered.
    pub slot_clips: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParamSpec {
    pub name: String,
    pub kind: ParamKind,
}

/// A parameter type and its default.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParamKind {
    Float(f32),
    Bool(bool),
    /// Components are read as `"<name>.x"` and `"<name>.y"`.
    Vec2(f32, f32),
    /// Set by gameplay, cleared when a transition consumes it or at the end
    /// of the update.
    Trigger,
}

/// A clip reference (animation.md "Optional clips"). In RON, either a
/// plain string (a required clip) or
/// `(source: "<reference>", optional: true, fallback: Clip("<name>") | Drop)`.
///
/// When an optional clip's source is missing, `compile` takes its
/// `fallback`: another clip name of the same graph (whose own entry may be
/// optional in turn) or a raw reference, or `Drop` (the default), which
/// removes the clip from the graph: a blend-space sample is dropped and the
/// space re-triangulated, and a node left with nothing to play drops with
/// its state and the transitions into and out of it. A missing required
/// clip is an error.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ClipRef {
    pub source: String,
    pub optional: bool,
    pub fallback: Option<ClipFallback>,
}

/// What replaces a missing optional clip.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClipFallback {
    /// Another clip name of the graph, or a raw clip reference (required).
    Clip(String),
    /// Remove the clip and what only it fed.
    Drop,
}

impl ClipRef {
    /// A required clip.
    pub fn required(source: impl Into<String>) -> Self {
        Self { source: source.into(), optional: false, fallback: None }
    }
}

impl<'de> Deserialize<'de> for ClipRef {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Fields {
            source: String,
            #[serde(default)]
            optional: bool,
            #[serde(default)]
            fallback: Option<ClipFallback>,
        }
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = ClipRef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a clip reference string or (source: .., optional: .., fallback: ..)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<ClipRef, E> {
                Ok(ClipRef::required(v))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<ClipRef, A::Error> {
                // Fields are read straight from the format's map access, so
                // RON's implicit `Some` and enum syntax work as elsewhere.
                let f = Fields::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                if f.fallback.is_some() && !f.optional {
                    return Err(serde::de::Error::custom(format!("clip {:?}: a fallback needs `optional: true`", f.source)));
                }
                Ok(ClipRef { source: f.source, optional: f.optional, fallback: f.fallback })
            }
        }
        d.deserialize_any(V)
    }
}

/// A number: a constant, or a parameter by name (`"speed"`, `"dir.x"`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueSpec {
    Const(f32),
    Param(String),
}

impl Default for ValueSpec {
    fn default() -> Self {
        Self::Const(1.0)
    }
}

impl From<f32> for ValueSpec {
    fn from(v: f32) -> Self {
        Self::Const(v)
    }
}

impl From<&str> for ValueSpec {
    fn from(v: &str) -> Self {
        Self::Param(v.to_owned())
    }
}

/// A joint, resolved per skeleton at compile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JointRef {
    /// By joint name.
    Joint(String),
    /// By skeleton tag (`spine`, `hand`…). In a mask, every tagged joint;
    /// elsewhere the tag must name exactly one joint.
    Tag(String),
    /// By rig bone name (e.g. a humanoid bone), through the adapter's rig map.
    Rig(String),
}

/// Per-joint weights: every joint starts at `default`; rules apply in order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MaskSpec {
    pub default: f32,
    pub rules: Vec<MaskRule>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaskRule {
    pub joint: JointRef,
    pub weight: f32,
    /// Also set every descendant (default). A later rule on a descendant
    /// overrides it, so "spine 0.3, chest 0.7, neck 1.0" ramps up the torso.
    #[serde(default = "yes")]
    pub subtree: bool,
}

fn yes() -> bool {
    true
}

/// How a layer combines with what is below it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerMode {
    #[default]
    Override,
    /// The layer's node must output an additive pose.
    Additive,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerSpec {
    pub node: String,
    #[serde(default)]
    pub mode: LayerMode,
    #[serde(default)]
    pub mask: Option<String>,
    #[serde(default)]
    pub weight: ValueSpec,
}

/// Which member leads a sync group (UE's group roles).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncRole {
    /// Leads when it has the highest weight.
    #[default]
    CanLead,
    /// Leads whenever active.
    AlwaysLead,
    /// Always follows.
    NeverLead,
}

/// Membership of a player (clip or blend space) in a sync group. Members
/// align by sync markers when the leader's current marker pair exists in
/// the follower, else by normalized phase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncSpec {
    pub group: String,
    #[serde(default)]
    pub role: SyncRole,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sample1D {
    pub clip: String,
    pub at: f32,
    #[serde(default = "one")]
    pub rate: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sample2D {
    pub clip: String,
    pub at: (f32, f32),
    #[serde(default = "one")]
    pub rate: f32,
}

fn one() -> f32 {
    1.0
}

fn crossfade_default() -> BlendMode {
    BlendMode::Inertialize(0.2)
}

/// A node definition. A node may be referenced from several places: each
/// reference is its own instance (own clock, own state).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeSpec {
    /// Plays one clip.
    Clip {
        clip: String,
        #[serde(default)]
        rate: ValueSpec,
        /// Defaults to whether the clip has a loop range.
        #[serde(default)]
        looping: Option<bool>,
        /// Start phase, 0–1 of the clip's duration.
        #[serde(default)]
        start: f32,
        #[serde(default)]
        sync: Option<SyncSpec>,
        /// Defaults to the enclosing state's mode, else the clip's.
        #[serde(default)]
        root_motion: Option<RootMotionMode>,
    },
    /// Clips placed on one axis; at most two play.
    BlendSpace1D {
        x: ValueSpec,
        samples: Vec<Sample1D>,
        #[serde(default)]
        rate: ValueSpec,
        /// Half-life of the input's critically damped smoothing, seconds (0 = none).
        #[serde(default)]
        smoothing: f32,
        #[serde(default)]
        looping: Option<bool>,
        #[serde(default)]
        sync: Option<SyncSpec>,
        #[serde(default)]
        root_motion: Option<RootMotionMode>,
    },
    /// Clips placed on a plane, Delaunay-triangulated at compile; at most
    /// three play. Outside the samples' hull the input is projected onto it.
    BlendSpace2D {
        x: ValueSpec,
        y: ValueSpec,
        samples: Vec<Sample2D>,
        #[serde(default)]
        rate: ValueSpec,
        #[serde(default)]
        smoothing: f32,
        #[serde(default)]
        looping: Option<bool>,
        #[serde(default)]
        sync: Option<SyncSpec>,
        #[serde(default)]
        root_motion: Option<RootMotionMode>,
    },
    /// `lerp(a, b, alpha)`, optionally per joint through a mask. A bool
    /// parameter reads as 0 or 1; `smoothing` turns that into a fade.
    Blend {
        a: String,
        b: String,
        alpha: ValueSpec,
        #[serde(default)]
        smoothing: f32,
        #[serde(default)]
        mask: Option<String>,
    },
    /// Applies an additive pose on top of `base`.
    Additive {
        base: String,
        additive: String,
        #[serde(default)]
        weight: ValueSpec,
        #[serde(default)]
        mask: Option<String>,
    },
    /// Layers over a base, as the graph's top-level layers (UE "Layered
    /// blend per bone").
    LayeredPerBone { base: String, layers: Vec<LayerSpec> },
    StateMachine {
        /// Defaults to the first state.
        #[serde(default)]
        initial: Option<String>,
        states: Vec<StateSpec>,
        /// In priority order: the first whose conditions hold fires.
        #[serde(default)]
        transitions: Vec<TransitionSpec>,
    },
    /// Scales the playback rate of everything below.
    RateScale { input: String, rate: ValueSpec },
    /// Mirrors the input across the rig's mirror plane, by `weight`.
    Mirror {
        input: String,
        #[serde(default)]
        weight: ValueSpec,
    },
    /// A pose supplied from outside (a ragdoll in M4), by slot name.
    PoseInput { slot: String },
    /// A montage slot (UE's Slot node): plays `input` until gameplay plays a
    /// clip in slot `slot` ([`crate::GraphInstance::play_slot`]), then that
    /// clip over it, through `mask` when set (an upper-body slot). Nodes
    /// naming the same slot show the same montage. `group` (default
    /// `"default"`): playing in one slot of a group stops the others.
    Slot {
        slot: String,
        input: String,
        #[serde(default)]
        group: Option<String>,
        #[serde(default)]
        mask: Option<String>,
    },
    /// Another graph asset, inlined at compile. Its parameters merge with
    /// this graph's by name; its procedural ops are ignored.
    Subgraph { graph: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateSpec {
    pub name: String,
    pub node: String,
    /// Root-motion mode for the players under this state that set none.
    #[serde(default)]
    pub root_motion: Option<RootMotionMode>,
}

/// How a transition moves from one state to the next.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendMode {
    /// Switch at once and decay the pose offset over this many seconds
    /// (Bollo 2018). Only the new state is evaluated.
    Inertialize(f32),
    /// Evaluate both and fade over this many seconds (smoothstep weights).
    Crossfade(f32),
    /// Switch at once.
    Cut,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransitionSpec {
    /// A state name, or `"*"` for any state (but `to`).
    pub from: String,
    pub to: String,
    /// All must hold. Empty: always (an automatic transition).
    #[serde(default)]
    pub when: Vec<ConditionSpec>,
    #[serde(default = "crossfade_default")]
    pub blend: BlendMode,
    /// Whether other transitions may fire while this one is fading in.
    #[serde(default = "yes")]
    pub interruptible: bool,
}

/// A transition condition. Phase and time conditions read the source
/// state's highest-weighted player.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConditionSpec {
    IsTrue(String),
    IsFalse(String),
    Greater(String, f32),
    Less(String, f32),
    /// A trigger parameter is set (and is consumed by the transition).
    Trigger(String),
    /// A clip under the source state fired this event (point or window
    /// enter) on the previous update.
    Event(String),
    /// Normalized time in the current cycle is at least this.
    PhaseAtLeast(f32),
    /// At most this many seconds left (non-looping players; UE's automatic rule).
    TimeLeft(f32),
    /// A non-looping player reached its end.
    Finished,
    /// Seconds spent in the source state.
    StateTime(f32),
}

/// Which rig-profile limbs a graph adds to its limb table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileLimbs {
    #[default]
    None,
    /// Support limbs (legs).
    Support,
    /// Every limb (legs, arms, wings…).
    All,
}

/// A limb: `joints` root first, effector last (2–8 joints; extra leading
/// joints such as a clavicle stay passive per the solver's span), or the
/// classic `hip`/`knee`/`foot` triple.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegSpec {
    pub name: String,
    #[serde(default)]
    pub hip: Option<JointRef>,
    #[serde(default)]
    pub knee: Option<JointRef>,
    #[serde(default)]
    pub foot: Option<JointRef>,
    #[serde(default)]
    pub joints: Vec<JointRef>,
    /// By joint count when omitted (3: two-bone, 4–5: `TwoBoneAnkle`, more: FABRIK).
    #[serde(default)]
    pub solver: Option<LimbSolver>,
    /// The girdle (pelvis or chest) carrying the limb; the parent of its
    /// first joint when omitted.
    #[serde(default)]
    pub girdle: Option<JointRef>,
    /// Bend direction in character space, used when the animated chain is
    /// nearly straight; the rest-pose bend when omitted.
    #[serde(default)]
    pub pole: Option<(f32, f32, f32)>,
    /// Carries weight (foot placement and stride warping take it by default).
    #[serde(default = "yes")]
    pub support: bool,
    /// Contact curve name (e.g. `contact.foot_l`, made by `anim.cook`).
    #[serde(default)]
    pub contact: Option<String>,
}

/// Robustness limits of `FootPlacement` (UE Foot Placement / Leg IK
/// practice). Lengths are fractions of each leg's reach, so they hold for
/// any rig size and body scale.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FootGuards {
    /// A ground sample further than this above or below the animated foot
    /// is rejected (a probe through a roof, a ledge, a stale or foreign
    /// hit): the foot keeps its animated height.
    pub max_step: f32,
    /// Lowest girdle offset: the pelvis never drops further, whatever the
    /// targets ask (a leg that still can't reach goes straight).
    pub max_drop: f32,
    /// Highest girdle offset (`Fit` raises the uphill girdle).
    pub max_rise: f32,
    /// A foot lock is released once it drifts this far (horizontally) from
    /// the animated foot: speed mismatch, turning, a teleport.
    pub unplant: f32,
    /// Above this speed (m/s) the op blends out.
    pub max_speed: f32,
    /// Halflife of the op's weight (blend in and out), seconds.
    pub blend_halflife: f32,
    /// Halflife of each foot's height offset, seconds.
    pub foot_halflife: f32,
}

impl Default for FootGuards {
    fn default() -> Self {
        Self { max_step: 0.5, max_drop: 0.4, max_rise: 0.5, unplant: 0.3, max_speed: 8.0, blend_halflife: 0.1, foot_halflife: 0.04 }
    }
}

/// How foot placement moves the girdles (rig-profile.md).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyFit {
    /// Each girdle only drops, by the largest need of its legs (bipeds).
    #[default]
    Lower,
    /// Each girdle follows the mean ground offset of its legs, capped so
    /// every leg still reaches: on slopes and stairs the body pitches (and
    /// with `roll`, rolls) to the ground, as quadrupeds do.
    Fit,
}

fn lod1() -> u8 {
    1
}

fn lod0() -> u8 {
    0
}

fn lod2() -> u8 {
    2
}

/// A procedural op. `max_lod` is the last animation LOD it runs at.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OpSpec {
    /// Playback rate of `Match` players = actual speed / clip speed,
    /// clamped; stride warping takes the rest.
    SpeedMatch {
        #[serde(default = "speed_min")]
        min: f32,
        #[serde(default = "speed_max")]
        max: f32,
        #[serde(default = "lod2")]
        max_lod: u8,
    },
    /// Scales foot targets along the velocity about each leg's girdle and
    /// lowers the girdles so the legs reach. `legs` empty: every support limb.
    StrideWarp {
        pelvis: JointRef,
        #[serde(default)]
        legs: Vec<String>,
        #[serde(default = "stride_min")]
        min: f32,
        #[serde(default = "stride_max")]
        max: f32,
        #[serde(default = "lod1")]
        max_lod: u8,
    },
    /// Turns the lower body toward the velocity and the spine back.
    OrientationWarp {
        pelvis: JointRef,
        /// Spine joints with their share of the counter-rotation.
        spine: Vec<(JointRef, f32)>,
        #[serde(default = "sixty_degrees")]
        max_angle: f32,
        #[serde(default = "tenth")]
        halflife: f32,
        #[serde(default = "lod1")]
        max_lod: u8,
    },
    /// Feet on the ground: height and normal from ground samples, locked
    /// while the contact curve is above 0.5; one offset per girdle (the
    /// root girdle moves `pelvis`), the body pitched between girdles and
    /// rolled by `roll`. `legs` empty: every support limb.
    FootPlacement {
        pelvis: JointRef,
        #[serde(default)]
        legs: Vec<String>,
        #[serde(default = "thirty_degrees")]
        max_foot_angle: f32,
        /// Seconds to release a lock: the offset from the animated foot decays with half-life `release / 2`.
        #[serde(default = "tenth")]
        release: f32,
        #[serde(default = "pelvis_halflife")]
        pelvis_halflife: f32,
        #[serde(default)]
        body: BodyFit,
        /// Share of the girdles' fitted side-slope roll the body takes (0–1).
        #[serde(default)]
        roll: f32,
        #[serde(default)]
        guards: FootGuards,
        #[serde(default = "lod1")]
        max_lod: u8,
    },
    /// Spreads a yaw over the spine by turn rate, head leading (quadrupeds
    /// bend into turns, Frey GDC 2016): `gain` radians per rad/s, each
    /// joint taking its share.
    SpineBend {
        spine: Vec<(JointRef, f32)>,
        #[serde(default = "spine_gain")]
        gain: f32,
        #[serde(default = "thirty_degrees")]
        max_angle: f32,
        #[serde(default = "tenth")]
        halflife: f32,
        #[serde(default = "lod1")]
        max_lod: u8,
    },
    /// Rolls joints into turns by lateral acceleration (`gain` radians per
    /// m/s², each joint its share).
    Lean {
        joints: Vec<(JointRef, f32)>,
        #[serde(default = "lean_gain")]
        gain: f32,
        #[serde(default = "fifteen_degrees")]
        max_angle: f32,
        #[serde(default = "tenth")]
        halflife: f32,
        #[serde(default = "lod1")]
        max_lod: u8,
    },
    /// IK of any limb of the table to a target slot, position plus the
    /// target's rotation when it has one and `rotation` is set; the solver
    /// is the limb's.
    LimbIk {
        limb: String,
        target: String,
        #[serde(default)]
        weight: ValueSpec,
        #[serde(default = "yes")]
        rotation: bool,
        #[serde(default = "lod0")]
        max_lod: u8,
    },
    /// FABRIK or CCD over a named chain (tails, necks, tentacles) to a target slot.
    ChainIk {
        chain: Vec<JointRef>,
        #[serde(default = "fabrik")]
        solver: LimbSolver,
        target: String,
        #[serde(default)]
        weight: ValueSpec,
        #[serde(default = "lod0")]
        max_lod: u8,
    },
    /// Spreads a look-at over a chain (root first): (joint, weight, limit).
    LookAt {
        chain: Vec<(JointRef, f32, f32)>,
        /// Aim axis in the last joint's frame.
        forward: (f32, f32, f32),
        #[serde(default = "eighty_degrees")]
        total_limit: f32,
        #[serde(default = "look_halflife")]
        halflife: f32,
        #[serde(default = "lod0")]
        max_lod: u8,
    },
    /// Full-body IK (XPBD over the rig profile, `sf_anim::ik::FullBody`):
    /// effectors from IK target slots; support limbs (and `pin`) stay
    /// where they are when the op runs (after `FootPlacement`: the placed
    /// feet) while the pelvis and spine give way.
    FullBodyIk {
        #[serde(default)]
        effectors: Vec<EffectorSpec>,
        #[serde(default = "yes")]
        pin_support: bool,
        /// More limbs pinned where they are (by limb name).
        #[serde(default)]
        pin: Vec<String>,
        #[serde(default = "ten")]
        iterations: u8,
        #[serde(default = "lod0")]
        max_lod: u8,
    },
    /// Two-bone IK of a hand to a target slot (compiles to a `LimbIk`).
    HandIk {
        upper: JointRef,
        lower: JointRef,
        hand: JointRef,
        target: String,
        #[serde(default)]
        weight: ValueSpec,
        #[serde(default = "lod0")]
        max_lod: u8,
    },
    /// Kinematic hit reactions: a hit (`GraphInstance::hit`) kicks critical
    /// rotation springs on the hit joint and up to `depth - 1` reactive
    /// ancestors (gain × `falloff` per level), about `lever × direction`;
    /// they peak and settle back to the animation. `joints` empty: the rig
    /// profile's spine, neck, head and non-support limbs (gain 1).
    HitReact {
        #[serde(default)]
        joints: Vec<(JointRef, f32)>,
        #[serde(default = "hit_depth")]
        depth: u8,
        #[serde(default = "half")]
        falloff: f32,
        /// Peak angle on the hit joint per unit strength, radians.
        #[serde(default = "hit_gain")]
        gain: f32,
        #[serde(default = "thirty_degrees")]
        max_angle: f32,
        #[serde(default = "hit_halflife")]
        halflife: f32,
        #[serde(default = "lod2")]
        max_lod: u8,
    },
    /// Verlet secondary motion on joint chains (tails, ears, antennae,
    /// hair): `chains` names the rig profile's tails and extra chains (empty:
    /// every tail), `explicit` adds chains by joint, root first.
    SpringBones {
        #[serde(default)]
        chains: Vec<String>,
        #[serde(default)]
        explicit: Vec<Vec<JointRef>>,
        /// Natural frequency of the pull toward the animated pose, Hz.
        #[serde(default = "spring_frequency")]
        frequency: f32,
        /// Damping ratio (1: critical, no overshoot).
        #[serde(default = "spring_damping")]
        damping: f32,
        /// m/s², down.
        #[serde(default)]
        gravity: f32,
        /// Share of the character's world motion the chains feel (0–1).
        #[serde(default = "half")]
        inertia: f32,
        #[serde(default = "sixty_degrees")]
        max_angle: f32,
        #[serde(default = "lod1")]
        max_lod: u8,
    },
}

fn hit_depth() -> u8 {
    3
}
fn half() -> f32 {
    0.5
}
fn hit_gain() -> f32 {
    0.35
}
fn hit_halflife() -> f32 {
    0.12
}
fn spring_frequency() -> f32 {
    3.0
}
fn spring_damping() -> f32 {
    0.6
}

fn speed_min() -> f32 {
    0.75
}
fn speed_max() -> f32 {
    1.3
}
fn stride_min() -> f32 {
    0.6
}
fn stride_max() -> f32 {
    1.6
}
fn sixty_degrees() -> f32 {
    60f32.to_radians()
}
fn fifteen_degrees() -> f32 {
    15f32.to_radians()
}
fn spine_gain() -> f32 {
    0.25
}
fn lean_gain() -> f32 {
    0.04
}
fn fabrik() -> LimbSolver {
    LimbSolver::Fabrik
}
fn thirty_degrees() -> f32 {
    30f32.to_radians()
}
fn eighty_degrees() -> f32 {
    80f32.to_radians()
}
fn tenth() -> f32 {
    0.1
}
fn pelvis_halflife() -> f32 {
    0.08
}
fn look_halflife() -> f32 {
    0.15
}

/// The mirror plane's normal axis in character space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    #[default]
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MirrorSpec {
    /// Joint name pairs; unpaired joints mirror onto themselves.
    pub pairs: Vec<(String, String)>,
    pub axis: Axis,
}

/// Parses a graph asset. `Option` fields may omit `Some(..)`.
#[cfg(feature = "bake")]
pub fn parse_graph(text: &str) -> Result<GraphAsset, String> {
    use ron::extensions::Extensions;
    ron::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_VARIANT_NEWTYPES)
        .from_str(text)
        .map_err(|e| e.to_string())
}

/// One `FullBodyIk` effector: a limb's end or a joint, to an IK target slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectorSpec {
    /// A limb of the table (its effector), or …
    #[serde(default)]
    pub limb: Option<String>,
    /// … any joint of the body (a head).
    #[serde(default)]
    pub joint: Option<JointRef>,
    pub target: String,
    #[serde(default)]
    pub weight: ValueSpec,
    /// Share of an out-of-reach stretch that pre-pulls the body.
    #[serde(default = "one")]
    pub pull: f32,
    /// Match the target's rotation when it has one.
    #[serde(default = "yes")]
    pub rotation: bool,
}

fn ten() -> u8 {
    10
}
