//! The compiled graph: the bake product of `animgraph.compile`, bound to one
//! skeleton. Names are resolved to indices, masks to per-joint weights,
//! blend spaces to triangles, and nodes laid out depth-first so every
//! node's subtree is a contiguous index range.

use std::sync::OnceLock;

use glam::{Affine3A, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::ik::FullBody;
use sf_anim::{JointWeights, LimbSolver, RootMotionMode, SkeletonId};
use sf_bake::ProductKey;

use crate::asset::{Axis, BodyFit, FootGuards, LayerMode, SyncRole};

/// A compiled number: constant or a parameter component.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Const(f32),
    /// Parameter index and component (0, or 1 for a `Vec2`'s y).
    Param(u16, u8),
}

/// A parameter's type and default.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParamType {
    Float,
    Bool,
    Vec2,
    Trigger,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    pub ty: ParamType,
    pub default: [f32; 2],
}

/// A clip the graph plays: the product it is loaded from, plus the facts
/// the compiler checked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClipSlot {
    /// The reference as written in the graph.
    pub reference: String,
    pub product: ProductKey,
    pub duration: f32,
    pub additive: bool,
    /// Where the clip came from (adapter data, for export license checks).
    pub provenance: Provenance,
}

/// Where a clip came from. Adapter-defined (`CompileSource::provenance`,
/// `GraphRules::provenance`): the framework knows no libraries or licenses,
/// it only carries the facts into the compiled graph so an export can
/// refuse clips that may not be shared.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    /// Source id (e.g. a clip library); empty for the project's own content.
    pub source: String,
    /// License tag (e.g. an SPDX id); empty when unknown.
    pub license: String,
    /// Whether derivatives (cooked clips, graphs that embed them) may be
    /// redistributed.
    pub redistributable: bool,
}

impl Default for Provenance {
    /// The project's own content: shareable.
    fn default() -> Self {
        Self { source: String::new(), license: String::new(), redistributable: true }
    }
}

/// How one authored clip name resolved (animation.md "Optional clips"):
/// the report of which slot came from which source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipResolution {
    /// The local clip name, `<subgraph>:<name>` inside a subgraph.
    pub name: String,
    /// The references found missing, in the order they were tried (empty
    /// when the primary source was present).
    pub missing: Vec<String>,
    /// The clip slot it resolved to, `None` when it dropped.
    pub slot: Option<u16>,
}

impl ClipResolution {
    /// Whether a fallback (or a drop) replaced the primary source.
    pub fn fell_back(&self) -> bool {
        !self.missing.is_empty()
    }
}

/// A clip-playing node or blend space: one lane per clip.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerNode {
    /// Clip slot per lane.
    pub lanes: Vec<u16>,
    /// Per-lane rate factor.
    pub lane_rates: Vec<f32>,
    pub rate: Value,
    pub looping: bool,
    /// Start phase, 0–1.
    pub start: f32,
    pub sync: Option<(u16, SyncRole)>,
    pub root_motion: RootMotionMode,
    /// `None` for a single clip.
    pub space: Option<BlendSpace>,
}

/// A triangulated blend space over normalized coordinates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlendSpace {
    pub x: Value,
    /// `None` for a 1D space.
    pub y: Option<Value>,
    /// Input range per axis, mapped to `[0, 1]`.
    pub min: Vec2,
    pub max: Vec2,
    pub smoothing: f32,
    /// Sample positions, normalized.
    pub points: Vec<Vec2>,
    /// Delaunay triangles (indices into `points`).
    pub triangles: Vec<[u16; 3]>,
    /// Hull edges (2D) or consecutive segments (1D, or collinear 2D).
    pub edges: Vec<[u16; 2]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub node: u32,
    pub mode: LayerMode,
    pub mask: Option<u16>,
    pub weight: Value,
}

/// How a compiled transition blends.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Blend {
    Inertialize(f32),
    Crossfade(f32),
    Cut,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    IsTrue(Value),
    IsFalse(Value),
    Greater(Value, f32),
    Less(Value, f32),
    Trigger(u16),
    Event(u16),
    PhaseAtLeast(f32),
    TimeLeft(f32),
    Finished,
    StateTime(f32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    /// `None` = any state.
    pub from: Option<u16>,
    pub to: u16,
    pub when: Vec<Condition>,
    pub blend: Blend,
    pub interruptible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub name: String,
    pub node: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    pub states: Vec<State>,
    pub initial: u16,
    pub transitions: Vec<Transition>,
}

/// A compiled node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    Player(PlayerNode),
    Blend {
        a: u32,
        b: u32,
        alpha: Value,
        smoothing: f32,
        mask: Option<u16>,
    },
    Additive {
        base: u32,
        additive: u32,
        weight: Value,
        mask: Option<u16>,
    },
    Layers {
        base: u32,
        layers: Vec<Layer>,
    },
    StateMachine(Machine),
    RateScale {
        input: u32,
        rate: Value,
    },
    Mirror {
        input: u32,
        weight: Value,
    },
    PoseInput {
        slot: u16,
    },
    /// A montage slot (UE's Slot node): passes `input` through until a
    /// montage plays in slot `slot`, then shows the montage over it
    /// (through `mask` when set). See [`crate::slot`].
    Slot {
        slot: u16,
        input: u32,
        mask: Option<u16>,
    },
}

/// A montage slot: its name and slot group (playing in one slot of a group
/// stops the montages in the group's other slots, as UE's slot groups).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotDef {
    pub name: String,
    pub group: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// Path of node names from the root (`"locomotion/walk"`), for debugging.
    pub name: String,
    pub kind: NodeKind,
    /// One past the last node of this node's subtree.
    pub subtree_end: u32,
    /// Outputs an additive (delta) pose.
    pub additive: bool,
}

/// A limb for the procedural ops (a leg, an arm, a wing): any joint count,
/// solved by its [`LimbSolver`] (rig-profile.md).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Leg {
    pub name: String,
    /// Root first, effector last.
    pub joints: Vec<u16>,
    pub solver: LimbSolver,
    /// The girdle joint carrying the limb (pivot of stride warping, frame of `pole`).
    pub girdle: u16,
    /// Bend direction in the girdle joint's frame, for nearly straight chains.
    pub pole: Option<Vec3>,
    pub support: bool,
    /// Graph curve index of the contact curve.
    pub contact: Option<u16>,
}

impl Leg {
    pub fn effector(&self) -> u16 {
        *self.joints.last().expect("limbs have joints")
    }

    /// The joints the solver moves (its span), root first.
    pub fn solved(&self) -> &[u16] {
        &self.joints[self.joints.len() - self.solver.span(self.joints.len())..]
    }
}

/// One girdle of a multi-girdle body fit, root-most first.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GirdleSeg {
    /// The joint the fit translates (root girdle: the op's pelvis) or
    /// reaches for (other girdles).
    pub joint: u16,
    /// Joints rotated to pitch the body toward this girdle's height: the
    /// previous girdle and the spine joints after it (empty for the root).
    pub spine: Vec<u16>,
    /// Indices into the op's leg list.
    pub legs: Vec<u16>,
}

/// How a warp window maps the remaining root translation onto the target.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarpMode {
    /// The remaining path skewed onto the target (UE's default).
    #[default]
    Skew,
    /// Uniform scale of the remaining translation's length, direction kept.
    Scale,
}

/// How a warp window turns the root.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarpRotation {
    /// Match the target's facing.
    #[default]
    Default,
    /// Face the target point.
    Facing,
    None,
}

/// A `warp` event window of a clip, parsed at compile (animation.md
/// "Layer 1: motion warping").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarpWindow {
    pub start: f32,
    pub end: f32,
    /// Warp target slot.
    pub target: u16,
    pub mode: WarpMode,
    pub rotation: WarpRotation,
    /// Warp vertical translation too (`y=warp`): the mover suspends ground
    /// snap and gravity while the window is open (traversal).
    pub vertical: bool,
    /// The rotation completes at this fraction of the window.
    pub rot_time: f32,
    /// Largest correction of the remaining path at an update, meters.
    pub max_dist: f32,
    /// Largest added turn rate, radians per second.
    pub max_turn: f32,
    /// The warp eases in over this fraction of the window (no root-speed jump).
    pub ease: f32,
    /// The warp point at the window's end in the root's frame there (zero:
    /// the root; else a joint, UE's "bone" warp point).
    pub point: Vec3,
}

/// Parses a `warp` event window of `clip`: payload
/// `<target>;<skew|scale>[;key=value]*` with keys `rot=default|facing|none`,
/// `y=ignore|warp`, `rot_time`, `max_dist` (m), `max_turn` (deg/s), `ease`,
/// `point=root|<joint>`. `target` maps the target name to its slot (the
/// compiler interns it; a late-bound clip looks it up).
///
/// # Errors
/// A malformed payload, or a target `target` refuses.
pub fn parse_warp_window(
    skeleton: &sf_anim::Skeleton,
    clip: &sf_anim::Clip,
    e: &sf_anim::Event,
    target: &mut dyn FnMut(&str) -> Result<u16, String>,
) -> Result<WarpWindow, String> {
    use sf_anim::{LocalPose, ModelPose, local_to_model};
    let end = e.end.ok_or("a warp event must be a window")?;
    let mut parts = e.payload.split(';').map(str::trim);
    let name = parts.next().filter(|t| !t.is_empty()).ok_or("no warp target")?;
    let target = target(name)?;
    let mode = match parts.next().unwrap_or("skew") {
        "skew" | "" => WarpMode::Skew,
        "scale" => WarpMode::Scale,
        m => return Err(format!("unknown warp mode {m:?}")),
    };
    let mut w = WarpWindow {
        start: e.time,
        end,
        target,
        mode,
        rotation: WarpRotation::Default,
        vertical: false,
        rot_time: 1.0,
        max_dist: f32::INFINITY,
        max_turn: f32::INFINITY,
        ease: 0.25,
        point: Vec3::ZERO,
    };
    let num = |v: &str| v.parse::<f32>().map_err(|_| format!("bad number {v:?}"));
    for kv in parts.filter(|p| !p.is_empty()) {
        let (k, v) = kv.split_once('=').ok_or_else(|| format!("expected key=value, got {kv:?}"))?;
        match (k.trim(), v.trim()) {
            ("rot", "default") => w.rotation = WarpRotation::Default,
            ("rot", "facing") => w.rotation = WarpRotation::Facing,
            ("rot", "none") => w.rotation = WarpRotation::None,
            ("y", "warp") => w.vertical = true,
            ("y", "ignore") => w.vertical = false,
            ("rot_time", v) => w.rot_time = num(v)?.clamp(0.0, 1.0),
            ("max_dist", v) => w.max_dist = num(v)?.max(0.0),
            ("max_turn", v) => w.max_turn = num(v)?.max(0.0).to_radians(),
            ("ease", v) => w.ease = num(v)?.clamp(0.0, 1.0),
            ("point", "root") => w.point = Vec3::ZERO,
            ("point", joint) => {
                // The joint's position at the window's end, in the root's frame there.
                let j = skeleton.find(joint).ok_or_else(|| format!("no joint {joint:?}"))?;
                let mut pose = LocalPose::new(skeleton.len());
                clip.sample(end, &mut pose, None);
                let mut model = ModelPose::new(skeleton.len());
                local_to_model(skeleton, &pose, &mut model, &Affine3A::IDENTITY);
                w.point = model.translation(j);
            }
            (k, v) => return Err(format!("unknown warp option {k}={v}")),
        }
    }
    Ok(w)
}

/// A compiled procedural op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Op {
    SpeedMatch {
        min: f32,
        max: f32,
    },
    StrideWarp {
        pelvis: u16,
        legs: Vec<u16>,
        girdles: Vec<GirdleSeg>,
        min: f32,
        max: f32,
    },
    OrientationWarp {
        pelvis: u16,
        spine: Vec<(u16, f32)>,
        max_angle: f32,
        halflife: f32,
    },
    FootPlacement {
        legs: Vec<u16>,
        girdles: Vec<GirdleSeg>,
        max_foot_angle: f32,
        release: f32,
        pelvis_halflife: f32,
        body: BodyFit,
        roll: f32,
        guards: FootGuards,
    },
    LookAt {
        chain: Vec<(u16, f32, f32)>,
        forward: Vec3,
        total_limit: f32,
        halflife: f32,
    },
    SpineBend {
        spine: Vec<(u16, f32)>,
        gain: f32,
        max_angle: f32,
        halflife: f32,
    },
    Lean {
        joints: Vec<(u16, f32)>,
        gain: f32,
        max_angle: f32,
        halflife: f32,
    },
    /// A limb (from the table, or a `HandIk` triple) to an IK target slot.
    LimbIk {
        limb: Leg,
        target: u16,
        weight: Value,
        rotation: bool,
    },
    /// Full-body IK: the solver built from the rig profile at compile.
    FullBodyIk {
        body: Box<FullBody>,
        effectors: Vec<BodyEffector>,
        /// Joints pinned where they are when the op runs.
        pins: Vec<u16>,
        iterations: u8,
    },
    ChainIk {
        joints: Vec<u16>,
        solver: LimbSolver,
        target: u16,
        weight: Value,
    },
}

/// One effector of a `FullBodyIk` op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyEffector {
    pub joint: u16,
    /// IK target slot.
    pub target: u16,
    pub weight: Value,
    pub pull: f32,
    pub rotation: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpEntry {
    pub op: Op,
    pub max_lod: u8,
}

/// Mirror data: per joint, its counterpart and the correction that maps
/// the counterpart's reflected model transform back onto this joint's
/// frame (so the rest pose mirrors onto itself).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MirrorTable {
    pub axis: Axis,
    pub pairs: Vec<u16>,
    pub corrections: Vec<Affine3A>,
}

/// A compiled graph.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledGraph {
    pub name: String,
    pub skeleton: SkeletonId,
    pub joint_count: u16,
    /// The joint whose mask weight scales root motion through masked blends.
    pub root_joint: u16,
    pub params: Vec<ParamDef>,
    pub clips: Vec<ClipSlot>,
    /// Per authored clip name, how it resolved (fallbacks, drops), in
    /// first-use order.
    pub resolutions: Vec<ClipResolution>,
    /// Per mask, per joint weight.
    pub masks: Vec<Vec<f32>>,
    pub nodes: Vec<Node>,
    /// The root (a `Layers` node made from the top-level layers).
    pub root: u32,
    pub sync_groups: Vec<String>,
    /// Graph-wide event names, and per clip slot, its events' ids.
    pub events: Vec<String>,
    pub clip_events: Vec<Vec<u16>>,
    /// Graph-wide curve names, and per clip slot, `(clip curve, graph curve)`.
    pub curves: Vec<String>,
    pub clip_curves: Vec<Vec<(u16, u16)>>,
    pub pose_inputs: Vec<String>,
    pub ik_targets: Vec<String>,
    pub warp_targets: Vec<String>,
    /// Per clip slot, its warp windows.
    pub warp_windows: Vec<Vec<WarpWindow>>,
    pub legs: Vec<Leg>,
    pub ops: Vec<OpEntry>,
    pub mirror: Option<MirrorTable>,
    /// Montage slots and slot groups (`Slot` nodes).
    pub slots: Vec<SlotDef>,
    pub slot_groups: Vec<String>,
    /// Clips a slot may play by name (the graph's `slot_clips`): local
    /// name and clip slot. Absent when an optional clip dropped.
    pub slot_clips: Vec<(String, u16)>,
    #[serde(skip)]
    mask_weights: OnceLock<Vec<JointWeights>>,
}

const MAGIC: &[u8; 4] = b"SFAG";
/// Version of the compiled graph format.
pub const GRAPH_FORMAT: u32 = 5;

impl CompiledGraph {
    #[cfg(feature = "bake")]
    pub(crate) fn new(name: String, skeleton: SkeletonId, joint_count: u16, root_joint: u16) -> Self {
        Self {
            name,
            skeleton,
            joint_count,
            root_joint,
            params: Vec::new(),
            clips: Vec::new(),
            resolutions: Vec::new(),
            masks: Vec::new(),
            nodes: Vec::new(),
            root: 0,
            sync_groups: Vec::new(),
            events: Vec::new(),
            clip_events: Vec::new(),
            curves: Vec::new(),
            clip_curves: Vec::new(),
            pose_inputs: Vec::new(),
            ik_targets: Vec::new(),
            warp_targets: Vec::new(),
            warp_windows: Vec::new(),
            legs: Vec::new(),
            ops: Vec::new(),
            mirror: None,
            slots: Vec::new(),
            slot_groups: Vec::new(),
            slot_clips: Vec::new(),
            mask_weights: OnceLock::new(),
        }
    }

    /// The clip slots whose provenance forbids redistribution: an export
    /// (mod packaging, a shared bake cache) refuses the graph if any.
    pub fn non_redistributable(&self) -> impl Iterator<Item = &ClipSlot> {
        self.clips.iter().filter(|c| !c.provenance.redistributable)
    }

    /// How the clip name `name` resolved.
    pub fn resolution(&self, name: &str) -> Option<&ClipResolution> {
        self.resolutions.iter().find(|r| r.name == name)
    }

    /// The first node whose path is `path` or ends with `/path`.
    pub fn node(&self, path: &str) -> Option<u32> {
        let suffix = format!("/{path}");
        self.nodes.iter().position(|n| n.name == path || n.name.ends_with(&suffix)).and_then(|i| u32::try_from(i).ok())
    }

    pub fn joint_count(&self) -> usize {
        usize::from(self.joint_count)
    }

    pub fn param(&self, name: &str) -> Option<ParamId> {
        self.params.iter().position(|p| p.name == name).map(|i| ParamId(u16::try_from(i).expect("param count fits u16")))
    }

    pub fn event(&self, name: &str) -> Option<u16> {
        self.events.iter().position(|e| e == name).and_then(|i| u16::try_from(i).ok())
    }

    pub fn curve(&self, name: &str) -> Option<usize> {
        self.curves.iter().position(|c| c == name)
    }

    pub fn pose_input(&self, name: &str) -> Option<usize> {
        self.pose_inputs.iter().position(|c| c == name)
    }

    pub fn ik_target(&self, name: &str) -> Option<usize> {
        self.ik_targets.iter().position(|c| c == name)
    }

    pub fn warp_target(&self, name: &str) -> Option<usize> {
        self.warp_targets.iter().position(|c| c == name)
    }

    /// A montage slot by name.
    pub fn slot(&self, name: &str) -> Option<crate::slot::SlotId> {
        self.slots.iter().position(|s| s.name == name).and_then(|i| u16::try_from(i).ok()).map(crate::slot::SlotId)
    }

    /// The clip slot of a `slot_clips` entry, by its local name.
    pub fn slot_clip(&self, name: &str) -> Option<u16> {
        self.slot_clips.iter().find(|(n, _)| n == name).map(|&(_, c)| c)
    }

    pub fn leg(&self, name: &str) -> Option<usize> {
        self.legs.iter().position(|l| l.name == name)
    }

    /// Masks as four-wide joint weights.
    pub fn mask_weights(&self) -> &[JointWeights] {
        self.mask_weights.get_or_init(|| {
            self.masks
                .iter()
                .map(|m| {
                    let mut w = JointWeights::splat(m.len(), 0.0);
                    for (j, &x) in m.iter().enumerate() {
                        w.set(j, x);
                    }
                    w
                })
                .collect()
        })
    }

    /// The state names of a state-machine node, for debugging.
    pub fn state_names(&self, node: u32) -> Vec<&str> {
        match &self.nodes[node as usize].kind {
            NodeKind::StateMachine(m) => m.states.iter().map(|s| s.name.as_str()).collect(),
            _ => Vec::new(),
        }
    }

    /// The compiled form: magic, format version, postcard.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend(GRAPH_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, sf_anim::FormatError> {
        use sf_anim::FormatError;
        if bytes.len() < 8 || bytes[..4] != MAGIC[..] {
            return Err(FormatError::Magic);
        }
        let found = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
        if found != GRAPH_FORMAT {
            return Err(FormatError::Version { found, expected: GRAPH_FORMAT });
        }
        let g: Self = postcard::from_bytes(&bytes[8..]).map_err(|e| FormatError::Decode(e.to_string()))?;
        g.validate().map_err(|m| FormatError::Decode(m.to_owned()))?;
        Ok(g)
    }

    fn validate(&self) -> Result<(), &'static str> {
        let n = self.nodes.len();
        let node_ok = |i: u32| (i as usize) < n;
        let joints = self.joint_count();
        if !node_ok(self.root) || self.masks.iter().any(|m| m.len() != joints) || usize::from(self.root_joint) >= joints.max(1) {
            return Err("graph tables");
        }
        if self.slots.iter().any(|s| usize::from(s.group) >= self.slot_groups.len())
            || self.slot_clips.iter().any(|&(_, c)| usize::from(c) >= self.clips.len())
        {
            return Err("slot tables");
        }
        if self.clip_events.len() != self.clips.len() || self.clip_curves.len() != self.clips.len() || self.warp_windows.len() != self.clips.len() {
            return Err("clip tables");
        }
        for (i, node) in self.nodes.iter().enumerate() {
            if (node.subtree_end as usize) <= i || node.subtree_end as usize > n {
                return Err("node ranges");
            }
            let kids_ok = match &node.kind {
                NodeKind::Player(p) => p.lanes.iter().all(|&c| usize::from(c) < self.clips.len()) && p.lane_rates.len() == p.lanes.len(),
                NodeKind::Blend { a, b, .. } => node_ok(*a) && node_ok(*b),
                NodeKind::Additive { base, additive, .. } => node_ok(*base) && node_ok(*additive),
                NodeKind::Layers { base, layers } => node_ok(*base) && layers.iter().all(|l| node_ok(l.node)),
                NodeKind::StateMachine(m) => {
                    !m.states.is_empty()
                        && usize::from(m.initial) < m.states.len()
                        && m.states.iter().all(|s| node_ok(s.node))
                        && m.transitions.iter().all(|t| usize::from(t.to) < m.states.len())
                }
                NodeKind::RateScale { input, .. } | NodeKind::Mirror { input, .. } => node_ok(*input),
                NodeKind::PoseInput { slot } => usize::from(*slot) < self.pose_inputs.len(),
                NodeKind::Slot { slot, input, mask } => {
                    usize::from(*slot) < self.slots.len() && node_ok(*input) && mask.is_none_or(|m| usize::from(m) < self.masks.len())
                }
            };
            if !kids_ok {
                return Err("node references");
            }
        }
        Ok(())
    }
}

/// A parameter handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParamId(pub u16);
