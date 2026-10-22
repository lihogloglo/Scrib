//! The evaluator: one [`GraphInstance`] per character, updated by
//! [`GraphInstance::update`], a pure function of (instance, parameters,
//! dt, inputs), so characters update in parallel and deterministically.
//!
//! One update runs these passes (animation.md "Evaluation"):
//! 1. **Prepare**, top-down: parameter smoothing, state-machine
//!    transitions, blend weights. Every reached player (clip or blend
//!    space) is recorded with its weight and playback rate. Zero-weight
//!    branches are skipped, so their clocks hold.
//! 2. **Advance**: unsynced players and sync-group leaders move by their
//!    rate; followers are placed by markers or phase. Events and root
//!    motion come out of the playhead moves.
//! 3. **Evaluate**, depth-first into pooled local poses (only joints in the
//!    LOD mask are sampled), carrying root motion and curves alongside.
//! 4. **Inertialization** offsets, pose history, then model space and the
//!    procedural ops.

use std::sync::Arc;

use glam::{Affine3A, Quat, Vec2, Vec3};
use sf_anim::spring::critical_spring;
use sf_anim::{
    BlendLayer, Clip, EventEdge, FiredEvent, JointMask, JointWeights, LocalPose, ModelPose, RootDelta, RootMotionMode, Skeleton, Trs, apply_additive,
    blend_in_place,
    blend_layers, local_to_model,
};

use crate::asset::{LayerMode, SyncRole};
use crate::compiled::{
    Blend, CompiledGraph, Condition, Machine, NodeKind, Op, ParamId, ParamType, PlayerNode, Value, WarpMode, WarpRotation, WarpWindow,
};
use crate::inertial::Inertializer;
use crate::procedural::{OpState, ProceduralCx, run_ops};
use crate::secondary::{ClothBinding, ClothRun, Hit};
use crate::slot::{
    Montage, MontageClip, MontageId, MontageStatus, NoticeKind, SlotError, SlotId, SlotNotice, SlotRequest, SlotStart, SlotState, clip_of, curve_map,
    event_ids, warp_windows,
};
use crate::sync::{loop_span, phase, sync_time, wrapped_delta};
use crate::variety::Variety;

const EPS: f32 = 1e-5;

/// A compiled graph with the skeleton and clips it plays. `clips` is in
/// the graph's clip-slot order ([`CompiledGraph::clips`]).
#[derive(Clone, Copy)]
pub struct Binding<'a> {
    pub graph: &'a CompiledGraph,
    pub skeleton: &'a Skeleton,
    pub clips: &'a [Arc<Clip>],
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BindError {
    #[error("the graph was compiled for another skeleton")]
    Skeleton,
    #[error("the graph names {expected} clips, {found} given")]
    ClipCount { expected: usize, found: usize },
    #[error("clip {0:?} is bound to another skeleton")]
    ClipSkeleton(String),
}

impl<'a> Binding<'a> {
    pub fn new(graph: &'a CompiledGraph, skeleton: &'a Skeleton, clips: &'a [Arc<Clip>]) -> Result<Self, BindError> {
        let id = skeleton.id();
        if graph.skeleton != id || graph.joint_count() != skeleton.len() {
            return Err(BindError::Skeleton);
        }
        if clips.len() != graph.clips.len() {
            return Err(BindError::ClipCount { expected: graph.clips.len(), found: clips.len() });
        }
        if let Some(c) = clips.iter().find(|c| c.skeleton != id) {
            return Err(BindError::ClipSkeleton(c.name.clone()));
        }
        Ok(Self { graph, skeleton, clips })
    }
}

/// The blackboard: parameter values, written by gameplay.
#[derive(Clone, Debug, PartialEq)]
pub struct Params {
    values: Vec<[f32; 2]>,
    types: Vec<ParamType>,
}

impl Params {
    pub fn new(graph: &CompiledGraph) -> Self {
        Self { values: graph.params.iter().map(|p| p.default).collect(), types: graph.params.iter().map(|p| p.ty).collect() }
    }

    pub fn set_float(&mut self, id: ParamId, v: f32) {
        self.values[usize::from(id.0)][0] = v;
    }

    pub fn set_bool(&mut self, id: ParamId, v: bool) {
        self.values[usize::from(id.0)][0] = if v { 1.0 } else { 0.0 };
    }

    pub fn set_vec2(&mut self, id: ParamId, v: Vec2) {
        self.values[usize::from(id.0)] = v.to_array();
    }

    /// Sets a trigger until a transition consumes it or the update ends.
    pub fn fire(&mut self, id: ParamId) {
        self.set_bool(id, true);
    }

    pub fn get(&self, id: ParamId) -> [f32; 2] {
        self.values[usize::from(id.0)]
    }

    #[inline]
    pub(crate) fn value(&self, v: Value) -> f32 {
        match v {
            Value::Const(c) => c,
            Value::Param(i, c) => self.values[usize::from(i)][usize::from(c)],
        }
    }

    fn clear_triggers(&mut self) {
        for (v, t) in self.values.iter_mut().zip(&self.types) {
            if *t == ParamType::Trigger {
                v[0] = 0.0;
            }
        }
    }
}

/// A ground hit under a leg, character space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundSample {
    pub point: Vec3,
    pub normal: Vec3,
}

/// Per-update inputs besides parameters. Everything is in character
/// space (the model space of the skeleton, Y up).
#[derive(Clone, Copy, Debug, Default)]
pub struct UpdateInputs<'a> {
    /// The character's actual velocity, m/s (speed matching and warping).
    pub velocity: Vec3,
    /// How the character moved since the previous update, in its frame at
    /// the previous update (keeps foot locks planted in the world).
    pub moved: RootDelta,
    /// Per probe ([`CompiledGraph::probe_count`]: each leg, then each leg toe): the ground under it, from
    /// probes at the previous update's [`GraphOutput::probe_origins`].
    pub ground: &'a [Option<GroundSample>],
    pub look_target: Option<Vec3>,
    /// Per IK target slot ([`CompiledGraph::ik_targets`]).
    pub ik_targets: &'a [Option<IkTarget>],
    /// Per warp target slot ([`CompiledGraph::warp_targets`]), refreshed
    /// every update so moving targets are tracked.
    pub warp_targets: &'a [Option<WarpTarget>],
    /// Turn rate, rad/s (`SpineBend`).
    pub turn_rate: f32,
    /// Acceleration, m/s², character space (`Lean`).
    pub acceleration: Vec3,
    /// Per pose-input slot ([`CompiledGraph::pose_inputs`]).
    pub poses: &'a [&'a LocalPose],
}

/// An IK goal, character space: a point and optionally the effector's rotation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IkTarget {
    pub position: Vec3,
    pub rotation: Option<Quat>,
}

impl From<Vec3> for IkTarget {
    fn from(position: Vec3) -> Self {
        Self { position, rotation: None }
    }
}

/// Where a warp window should bring the character (character space):
/// the warp point's position and the facing to end with.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WarpTarget {
    pub position: Vec3,
    /// Radians about +Y (0 faces +Z).
    pub yaw: f32,
    /// Velocity of a moving target, m/s, character space: the warp aims
    /// where the target will be at the end of this update, which removes
    /// the update's latency (`ZERO` for static targets).
    pub velocity: Vec3,
}

/// The warp running this update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveWarp {
    /// Warp target slot.
    pub target: u16,
    /// Clip seconds left in the window.
    pub remaining: f32,
    /// Distance between where the warped path now ends and the target
    /// (non-zero only when `max_dist` clamps), meters.
    pub error: f32,
    /// Facing still to turn after this update, radians.
    pub yaw_error: f32,
    /// The window warps vertical motion: the mover runs in traversal.
    pub vertical: bool,
}

/// An event that fired during an update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphEvent {
    /// Clip slot and the event's index in that clip.
    pub clip: u16,
    pub event: u32,
    /// Graph-wide event id ([`CompiledGraph::events`]).
    pub id: u16,
    pub edge: EventEdge,
    /// Weight of the clip that fired it.
    pub weight: f32,
    /// Top-level layer it came from.
    pub layer: u16,
    /// The montage that fired it (`clip` is [`crate::slot::LATE_CLIP`] for a
    /// late-bound clip); `None` for the graph's own players.
    pub montage: Option<MontageId>,
}

/// What an update produces.
#[derive(Clone, Debug)]
pub struct GraphOutput {
    /// Final local pose (graph, inertialization, procedural ops).
    pub pose: LocalPose,
    /// The same in model space.
    pub model: ModelPose,
    /// False at LOD 3: clocks, events and root motion only.
    pub posed: bool,
    /// Root motion of `Drive` players, weighted by their share of the pose.
    pub root: RootDelta,
    /// How much of the pose comes from `Drive` players (0–1). The mover
    /// blends between the root delta and its own velocity by this.
    pub drive_weight: f32,
    pub events: Vec<GraphEvent>,
    /// Per graph curve; 0 where no playing clip has it.
    pub curves: Vec<f32>,
    /// Per limb: where to probe for ground before the next update (the
    /// animated effector, character space).
    pub probe_origins: Vec<Vec3>,
    /// The motion warp applied this update, if any.
    pub warp: Option<ActiveWarp>,
    /// Montage starts and ends since the previous update.
    pub slot_notices: Vec<SlotNotice>,
    /// What `FootPlacement` corrected this update (guards that fired).
    pub feet: FootStats,
}

/// `FootPlacement` guard counts of one update (hosts sum them as
/// diagnostics: a healthy crowd rarely clamps).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FootStats {
    /// Lock drift: how far the animated foot strayed from its locked spot
    /// (horizontal, m), on every update a foot was locked in stance (its
    /// contact curve up; the release fade doesn't count). Largest, sum
    /// and count (mean = sum / count). A foot whose clip outruns the body
    /// drifts until it passes the unplant distance.
    pub drift_max: f32,
    pub drift_sum: f32,
    pub drift_count: u32,
    /// Ground samples rejected (step limit, non-finite, too steep).
    pub rejected: u32,
    /// Foot targets pulled back within the leg's reach of its hip (by more
    /// than 2% of it).
    pub reach_clamped: u32,
    /// Girdle offsets cut by the drop or rise limit.
    pub pelvis_clamped: u32,
    /// Locks released because they drifted past the unplant distance.
    pub unplanted: u32,
    /// Updates whose `moved` jumped further than a leg reaches (locks reset).
    pub teleports: u32,
    /// Non-finite results caught (the op was skipped and its state reset).
    pub non_finite: u32,
}

impl std::ops::AddAssign for FootStats {
    fn add_assign(&mut self, o: Self) {
        self.drift_max = self.drift_max.max(o.drift_max);
        self.drift_sum += o.drift_sum;
        self.drift_count += o.drift_count;
        self.rejected += o.rejected;
        self.reach_clamped += o.reach_clamped;
        self.pelvis_clamped += o.pelvis_clamped;
        self.unplanted += o.unplanted;
        self.teleports += o.teleports;
        self.non_finite += o.non_finite;
    }
}

impl GraphOutput {
    pub fn new(graph: &CompiledGraph) -> Self {
        let n = graph.joint_count();
        Self {
            pose: LocalPose::new(n),
            model: ModelPose::new(n),
            posed: false,
            root: RootDelta::ZERO,
            drive_weight: 0.0,
            events: Vec::new(),
            curves: vec![0.0; graph.curves.len()],
            probe_origins: vec![Vec3::ZERO; graph.probe_count()],
            warp: None,
            slot_notices: Vec::new(),
            feet: FootStats::default(),
        }
    }
}

// --- Runtime node state ----------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
struct Smooth {
    value: f32,
    velocity: f32,
    init: bool,
}

impl Smooth {
    fn update(&mut self, goal: f32, halflife: f32, dt: f32) -> f32 {
        if !self.init || halflife <= 0.0 {
            *self = Self { value: goal, velocity: 0.0, init: true };
        } else {
            critical_spring(&mut self.value, &mut self.velocity, goal, halflife, dt);
        }
        self.value
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Lane {
    time: f32,
    weight: f32,
    /// Clip seconds per second over the last advance.
    speed: f32,
    root: RootDelta,
    /// Playhead before the last move, and whether that move wrapped.
    prev: f32,
    wrapped: bool,
}

#[derive(Clone, Debug, Default)]
struct PlayerState {
    lanes: Vec<Lane>,
    weight: f32,
    prev_weight: f32,
    rate: f32,
    layer: u16,
    leader: usize,
    fresh: bool,
    finished: bool,
    input: [Smooth; 2],
    /// `(lane, event)` from the last advance.
    events: Vec<(u16, FiredEvent)>,
}

#[derive(Clone, Debug, Default)]
struct MachineState {
    current: u16,
    /// States fading out and their weight when the fade started.
    fading: Vec<(u16, f32)>,
    target_start: f32,
    alpha: f32,
    duration: f32,
    interruptible: bool,
    state_time: f32,
    /// Active states and their weights this update.
    weights: Vec<(u16, f32)>,
}

#[derive(Clone, Debug)]
enum NodeState {
    Stateless,
    Player(PlayerState),
    Smooth(Smooth),
    Machine(MachineState),
}

/// One evaluation frame of the pose stack.
#[derive(Clone, Debug)]
struct Frame {
    pose: LocalPose,
    /// NaN where no clip below has the curve.
    curves: Vec<f32>,
    root: RootDelta,
    drive: f32,
}

#[derive(Clone, Debug)]
struct Scratch {
    frames: Vec<Frame>,
    curve_w: Vec<f32>,
    model: ModelPose,
    model2: ModelPose,
}

impl Scratch {
    fn ensure(&mut self, len: usize, joints: usize, curves: usize) {
        while self.frames.len() < len {
            self.frames.push(Frame { pose: LocalPose::new(joints), curves: vec![f32::NAN; curves], root: RootDelta::ZERO, drive: 0.0 });
        }
    }
}

/// A playing graph: node states, parameters, pose history.
#[derive(Clone, Debug)]
pub struct GraphInstance {
    pub params: Params,
    nodes: Vec<NodeState>,
    ticks: Vec<u32>,
    groups: Vec<Vec<u32>>,
    request: Option<f32>,
    inertial: Inertializer,
    prev: LocalPose,
    prev2: LocalPose,
    prev_dt: f32,
    history: u8,
    lookahead: LocalPose,
    rest: LocalPose,
    identity: LocalPose,
    scratch: Scratch,
    ops: Vec<OpState>,
    speed_match: Option<(f32, f32)>,
    stride: (f32, f32),
    anim_velocity: Vec3,
    /// Per limb: the animated effector at the previous posed update (empty
    /// when there is none), for extrapolating probe origins.
    effectors: Vec<Vec3>,
    /// Per montage slot ([`CompiledGraph::slots`]).
    slots: Vec<SlotState>,
    next_montage: u32,
    /// The physics input (a ragdoll's pose and per-joint weights), blended
    /// over the graph's pose; see [`GraphInstance::set_physics`].
    physics: Option<PhysicsInput>,
    /// A hand-back blend requested by [`GraphInstance::inertialize_from`].
    handback: Option<f32>,
    /// Hits for the `HitReact` ops, queued by [`GraphInstance::hit`].
    hits: Vec<Hit>,
    /// Bound cloth parts ([`GraphInstance::set_cloth`]).
    cloth: Vec<ClothRun>,
    /// Phase offset and rate jitter ([`GraphInstance::set_variety`]).
    variety: Variety,
}

/// A pose supplied from outside the graph (a ragdoll's), with a weight per
/// joint: 0 keeps the animation, 1 shows the physics pose.
#[derive(Clone, Debug)]
struct PhysicsInput {
    pose: Vec<Trs>,
    weights: Vec<f32>,
}

/// Read-only context of the prepare and advance passes.
struct Cx<'a> {
    g: &'a CompiledGraph,
    clips: &'a [Arc<Clip>],
    dt: f32,
}

impl GraphInstance {
    pub fn new(b: &Binding<'_>) -> Self {
        let g = b.graph;
        let n = g.joint_count();
        let nodes = g
            .nodes
            .iter()
            .map(|node| match &node.kind {
                NodeKind::Player(p) => NodeState::Player(PlayerState { lanes: vec![Lane::default(); p.lanes.len()], ..PlayerState::default() }),
                NodeKind::Blend { .. } => NodeState::Smooth(Smooth::default()),
                NodeKind::StateMachine(_) => NodeState::Machine(MachineState::default()),
                _ => NodeState::Stateless,
            })
            .collect();
        let mut inst = Self {
            params: Params::new(g),
            nodes,
            ticks: Vec::new(),
            groups: vec![Vec::new(); g.sync_groups.len()],
            request: None,
            inertial: Inertializer::default(),
            prev: LocalPose::new(n),
            prev2: LocalPose::new(n),
            prev_dt: 0.0,
            history: 0,
            lookahead: LocalPose::new(n),
            rest: LocalPose::rest(b.skeleton),
            identity: LocalPose::new(n),
            scratch: Scratch { frames: Vec::new(), curve_w: vec![0.0; g.curves.len()], model: ModelPose::new(n), model2: ModelPose::new(n) },
            physics: None,
            handback: None,
            hits: Vec::new(),
            cloth: Vec::new(),
            ops: g.ops.iter().map(|o| OpState::new(&o.op)).collect(),
            speed_match: None,
            stride: (0.0, 0.0),
            anim_velocity: Vec3::ZERO,
            effectors: Vec::new(),
            slots: vec![SlotState::default(); g.slots.len()],
            next_montage: 0,
            variety: Variety::default(),
        };
        inst.reset_range(b, 0, u32::try_from(g.nodes.len()).expect("node count fits u32"));
        inst
    }

    /// Gives this character its own phase and rate ([`Variety`]): looping
    /// clips start `variety.phase` of a cycle in (the running ones shift now)
    /// and play `variety.rate` times as fast, except root-motion-driven ones.
    pub fn set_variety(&mut self, b: &Binding<'_>, variety: Variety) {
        let delta = variety.phase - self.variety.phase;
        self.variety = variety;
        if delta == 0.0 {
            return;
        }
        for (node, state) in b.graph.nodes.iter().zip(&mut self.nodes) {
            let (NodeKind::Player(p), NodeState::Player(st)) = (&node.kind, state) else { continue };
            if !p.looping {
                continue;
            }
            for (lane, &clip) in st.lanes.iter_mut().zip(&p.lanes) {
                let (a, e) = loop_span(&b.clips[usize::from(clip)], true);
                if e - a > 1e-6 {
                    lane.time = a + (lane.time - a + delta * (e - a)).rem_euclid(e - a);
                    lane.prev = lane.time;
                }
            }
        }
    }

    /// The variety set by [`GraphInstance::set_variety`].
    pub fn variety(&self) -> Variety {
        self.variety
    }

    /// Forgets the pose history (after a teleport or a pose snap), so the
    /// next switch cuts instead of inertializing from a stale pose.
    pub fn reset_history(&mut self) {
        self.history = 0;
        self.inertial.cancel();
        self.effectors.clear();
    }

    /// The physics input (physics.md "Ragdolls and the animation blend"): a
    /// local pose blended over the graph's output per joint by `weights`
    /// (0 = animation, 1 = physics), after inertialization and before the
    /// pose history and the procedural ops, so a later
    /// [`inertialize_from`](Self::inertialize_from) or state switch blends
    /// out of the shown pose and the ops (foot placement, look-at) act only
    /// on the animated share. `None`, or a pose of the wrong length, clears
    /// it. This is the graph's built-in physics stage: graphs need no
    /// authored `PoseInput` node for a ragdoll (their `PoseInput` slots stay
    /// for other external poses).
    pub fn set_physics(&mut self, physics: Option<(&[Trs], &[f32])>) {
        match physics {
            Some((pose, weights)) if pose.len() == self.rest.len() && weights.len() == pose.len() => {
                let p = self.physics.get_or_insert_with(|| PhysicsInput { pose: Vec::new(), weights: Vec::new() });
                p.pose.clear();
                p.pose.extend_from_slice(pose);
                p.weights.clear();
                p.weights.extend_from_slice(weights);
            }
            _ => self.physics = None,
        }
    }

    /// Whether a physics input is set.
    pub fn has_physics(&self) -> bool {
        self.physics.is_some()
    }

    /// The hand-back from physics (a get-up): the next update inertializes
    /// from `current` (moving from `previous`, `dt` earlier) into whatever
    /// the graph then produces, over `duration` seconds. Both poses are
    /// local, relative to the character frame the next update runs in (the
    /// host may have moved the root under the ragdoll's pelvis first).
    pub fn inertialize_from(&mut self, current: &[Trs], previous: &[Trs], dt: f32, duration: f32) {
        let n = self.rest.len();
        if current.len() != n || previous.len() != n || dt <= 0.0 {
            return;
        }
        for j in 0..n {
            self.prev.set(j, current[j]);
            self.prev2.set(j, previous[j]);
        }
        self.prev_dt = dt;
        self.history = 2;
        self.handback = Some(duration);
    }

    /// Whether an inertialization blend is running.
    pub fn inertializing(&self) -> bool {
        self.inertial.is_active()
    }

    /// Queues a hit for the graph's `HitReact` ops (character space); it
    /// kicks at the next update. Graphs without the op ignore it.
    pub fn hit(&mut self, hit: Hit) {
        self.hits.push(hit);
    }

    /// The largest `HitReact` offset now, radians (diagnostics, tests).
    /// Binds cloth parts (an outfit's lattices; empty: none). Each update
    /// then runs the `ClothLattice` op on them after the procedural ops.
    pub fn set_cloth(&mut self, cloth: Vec<Arc<ClothBinding>>) {
        self.cloth = cloth.into_iter().map(ClothRun::new).collect();
    }

    /// The bound cloth parts and their last pose, in [`Self::set_cloth`] order.
    pub fn cloth(&self) -> &[ClothRun] {
        &self.cloth
    }

    pub fn cloth_mut(&mut self) -> &mut [ClothRun] {
        &mut self.cloth
    }

    pub fn hit_reaction(&self) -> f32 {
        self.ops.iter().map(|o| if let OpState::Hit(h) = o { h.max_angle() } else { 0.0 }).fold(0.0, f32::max)
    }

    /// The current state of a state-machine node.
    pub fn current_state(&self, node: u32) -> Option<usize> {
        match &self.nodes[node as usize] {
            NodeState::Machine(m) => Some(usize::from(m.current)),
            _ => None,
        }
    }

    /// Active states of a state-machine node and their weights.
    pub fn state_weights(&self, node: u32) -> Vec<(usize, f32)> {
        match &self.nodes[node as usize] {
            NodeState::Machine(m) => m.weights.iter().map(|&(s, w)| (usize::from(s), w)).collect(),
            _ => Vec::new(),
        }
    }

    /// A player node's lanes: `(time, weight within the player)`, and the
    /// player's weight this update.
    pub fn player(&self, node: u32) -> Option<(Vec<(f32, f32)>, f32)> {
        match &self.nodes[node as usize] {
            NodeState::Player(p) => Some((p.lanes.iter().map(|l| (l.time, l.weight)).collect(), p.weight)),
            _ => None,
        }
    }

    // --- Slots ---------------------------------------------------------------------------

    /// Plays a clip in a montage slot (animation.md "Slots and montages").
    /// Takes effect at the next update, which inertializes into it.
    ///
    /// # Errors
    /// No such slot or clip, an additive clip, a late clip on another
    /// skeleton, or (`Interrupt`) a higher-priority montage playing.
    pub fn play_slot(&mut self, b: &Binding<'_>, slot: SlotId, req: SlotRequest) -> Result<MontageId, SlotError> {
        let s = usize::from(slot.0);
        if s >= self.slots.len() {
            return Err(SlotError::NoSlot);
        }
        match &req.clip {
            MontageClip::Bound(c) => {
                let clip = b.clips.get(usize::from(*c)).ok_or(SlotError::NoClip)?;
                if clip.meta.additive {
                    return Err(SlotError::Additive);
                }
            }
            MontageClip::Late(l) if l.clip.skeleton != b.skeleton.id() => return Err(SlotError::Skeleton),
            MontageClip::Late(_) => {}
        }
        let st = &mut self.slots[s];
        if req.start == SlotStart::Interrupt && st.playing.as_ref().is_some_and(|m| !m.ended && m.req.priority > req.priority) {
            return Err(SlotError::Busy);
        }
        let id = MontageId(self.next_montage);
        self.next_montage = self.next_montage.wrapping_add(1);
        match req.start {
            SlotStart::Queue if st.playing.is_some() => st.queue.push_back((id, req)),
            SlotStart::Queue => self.start_montage(b, s, id, req),
            SlotStart::Interrupt => {
                self.end_slot(b.graph, b.clips, s, req.blend_in);
                self.start_montage(b, s, id, req);
            }
        }
        Ok(id)
    }

    /// Stops the slot's montage and drops its queue; the slot's input comes
    /// back over `blend_out` seconds of inertialization.
    pub fn stop_slot(&mut self, b: &Binding<'_>, slot: SlotId, blend_out: f32) {
        if usize::from(slot.0) < self.slots.len() {
            self.end_slot(b.graph, b.clips, usize::from(slot.0), blend_out);
        }
    }

    /// What plays in a slot.
    pub fn montage(&self, slot: SlotId) -> Option<MontageStatus> {
        let st = self.slots.get(usize::from(slot.0))?;
        st.playing.as_ref().map(|m| MontageStatus { id: m.id, time: m.time, loops_left: m.loops_left, queued: st.queue.len() })
    }

    /// Ends what plays in slot `s` and its queue (interrupted, unless it
    /// had just played out), asking for a `blend` back.
    fn end_slot(&mut self, g: &CompiledGraph, clips: &[Arc<Clip>], s: usize, blend: f32) {
        let slot = SlotId(u16::try_from(s).unwrap_or(u16::MAX));
        let st = &mut self.slots[s];
        if let Some(m) = st.playing.take() {
            st.notices.push(SlotNotice { slot, montage: m.id, kind: NoticeKind::Ended { interrupted: !m.ended } });
            // Windows it is inside close (UE ends the notify states of an
            // interrupted montage), reported with the next update's events.
            if !m.ended && !m.fresh {
                let clip = clip_of(clips, &m.req.clip);
                for (i, e) in clip.events.iter().enumerate() {
                    if e.end.is_some_and(|end| e.time <= m.time && m.time < end) {
                        let event = u32::try_from(i).unwrap_or(u32::MAX);
                        let (c, id) = event_ids(g, &m.req.clip, event);
                        st.exits.push(GraphEvent {
                            clip: c,
                            event,
                            id,
                            edge: EventEdge::Exit,
                            weight: st.weight,
                            layer: st.layer,
                            montage: Some(m.id),
                        });
                    }
                }
            }
            st.blend = Some(blend);
        }
        for (id, _) in st.queue.drain(..) {
            st.notices.push(SlotNotice { slot, montage: id, kind: NoticeKind::Ended { interrupted: true } });
        }
    }

    /// Starts a montage in slot `s`, ending the montages of the slot's
    /// group mates.
    fn start_montage(&mut self, b: &Binding<'_>, s: usize, id: MontageId, req: SlotRequest) {
        let g = b.graph;
        let group = g.slots[s].group;
        for k in 0..self.slots.len() {
            if k != s && g.slots[k].group == group {
                self.end_slot(g, b.clips, k, req.blend_in);
            }
        }
        let slot = SlotId(u16::try_from(s).unwrap_or(u16::MAX));
        let st = &mut self.slots[s];
        st.blend = Some(req.blend_in);
        st.notices.push(SlotNotice { slot, montage: id, kind: NoticeKind::Started });
        let m = Montage::new(id, req.clone(), clip_of(b.clips, &req.clip));
        st.playing = Some(m);
    }

    /// Montages leave once they've shown their end, the next queued one
    /// starts, and the playing ones advance.
    fn tick_slots(&mut self, b: &Binding<'_>, dt: f32) {
        for s in 0..self.slots.len() {
            self.slots[s].weight = 0.0;
            if self.slots[s].playing.as_ref().is_some_and(|m| m.ended) {
                let blend_out = self.slots[s].playing.as_ref().map_or(0.0, |m| m.req.blend_out);
                let next = self.slots[s].queue.pop_front();
                self.end_slot(b.graph, b.clips, s, blend_out);
                if let Some((id, req)) = next {
                    self.start_montage(b, s, id, req);
                }
            }
            if let Some(m) = &mut self.slots[s].playing {
                let c = m.req.clip.clone();
                m.advance(clip_of(b.clips, &c), dt);
            }
        }
    }

    /// Advances the graph by `dt` seconds at animation LOD `lod` (0–2
    /// sample the skeleton's LOD joint mask; 3 and above advance clocks,
    /// events and root motion only).
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, b: &Binding<'_>, dt: f32, lod: usize, inputs: &UpdateInputs<'_>, out: &mut GraphOutput) {
        let g = b.graph;
        let dt = dt.max(0.0);
        for &n in &self.ticks {
            if let NodeState::Player(p) = &mut self.nodes[n as usize] {
                p.prev_weight = p.weight;
                p.weight = 0.0;
            }
        }
        self.ticks.clear();
        self.request = None;
        self.speed_match = g.ops.iter().find_map(|o| match o.op {
            Op::SpeedMatch { min, max } if lod <= usize::from(o.max_lod) => Some((min, max)),
            _ => None,
        });
        self.stride = (0.0, 0.0);
        self.anim_velocity = Vec3::ZERO;

        self.tick_slots(b, dt);
        out.feet = FootStats::default();
        out.slot_notices.clear();
        for st in &mut self.slots {
            out.slot_notices.append(&mut st.notices);
        }

        let cx = Cx { g, clips: b.clips, dt };
        self.prepare(b, &cx, g.root, 1.0, 1.0, 0);
        self.advance(&cx, inputs);
        // A slot switch inertializes where the slot is seen.
        for st in &mut self.slots {
            if let Some(d) = st.blend.take()
                && st.weight > EPS
            {
                self.request = Some(self.request.map_or(d, |r| r.min(d)));
            }
        }
        // The hand-back owns the blend: a state switch on the same update (the
        // host clearing its knocked-out flag) starts from the same pose, and
        // its shorter blend snapped the lying pose onto the clip (AN-5).
        if let Some(d) = self.handback.take() {
            self.request = Some(d);
        }
        self.collect_events(g, out);
        self.params.clear_triggers();

        let posed = lod < 3;
        let mask = if posed { b.skeleton.lod_mask(lod) } else { None };
        let curves = g.curves.len();
        let joints = g.joint_count();
        self.scratch.ensure(1, joints, curves);
        let ecx = EvalCx {
            g,
            clips: b.clips,
            skeleton: b.skeleton,
            nodes: &self.nodes,
            params: &self.params,
            rest: &self.rest,
            identity: &self.identity,
            mask,
            posed,
            shift: 0.0,
            poses: inputs.poses,
            slots: &self.slots,
        };
        eval(&ecx, &mut self.scratch, g.root, 0);
        let top = &self.scratch.frames[0];
        out.root = top.root;
        out.warp = self.apply_warp(g, b.clips, inputs, dt, &mut out.root);
        out.drive_weight = top.drive;
        out.curves.clear();
        out.curves.extend(top.curves.iter().map(|&c| if c.is_nan() { 0.0 } else { c }));
        out.posed = posed;
        if !posed {
            self.reset_history();
            self.hits.clear();
            for c in &mut self.cloth {
                c.reset();
            }
            return;
        }
        out.pose.copy_from(&top.pose);

        // Inertialization: a switch this update starts a new blend.
        self.inertial.advance(dt);
        if let Some(duration) = self.request {
            if self.history >= 2 && dt > 0.0 {
                let ahead = EvalCx { shift: dt, ..ecx };
                eval(&ahead, &mut self.scratch, g.root, 0);
                self.lookahead.copy_from(&self.scratch.frames[0].pose);
                self.inertial.start(&self.prev, &self.prev2, self.prev_dt, &out.pose, &self.lookahead, dt, duration);
            } else {
                self.inertial.cancel();
            }
        }
        self.inertial.apply(&mut out.pose);
        if let Some(p) = &self.physics {
            for (j, (&w, &t)) in p.weights.iter().zip(&p.pose).enumerate() {
                if w >= 1.0 {
                    out.pose.set(j, t);
                } else if w > 0.0 {
                    let a = out.pose.get(j);
                    out.pose.set(
                        j,
                        Trs {
                            translation: a.translation.lerp(t.translation, w),
                            rotation: a.rotation.slerp(t.rotation, w).normalize(),
                            scale: a.scale.lerp(t.scale, w),
                        },
                    );
                }
            }
        }
        if dt > 0.0 {
            std::mem::swap(&mut self.prev, &mut self.prev2);
            self.prev.copy_from(&out.pose);
            self.prev_dt = dt;
            self.history = (self.history + 1).min(2);
        }

        // Model space and the procedural ops.
        local_to_model(b.skeleton, &out.pose, &mut out.model, &Affine3A::IDENTITY);
        self.probe_origins(g, dt, out);
        let stride_ratio = if self.stride.1 > EPS { self.stride.0 / self.stride.1 } else { 1.0 };
        let mut pcx = ProceduralCx {
            graph: g,
            skeleton: b.skeleton,
            params: &self.params,
            inputs,
            dt,
            lod,
            stride_ratio,
            anim_velocity: self.anim_velocity,
            curves: &out.curves,
            feet: FootStats::default(),
            hits: &self.hits,
        };
        run_ops(&mut pcx, &mut self.ops, &mut out.pose, &mut out.model);
        // Ops (foot placement, look-at, hit springs) leave the joints physics
        // fully owns alone (R0: feet planted on the slope under a ragdoll).
        if let Some(p) = &self.physics {
            let mut owned = false;
            for (j, (&w, &t)) in p.weights.iter().zip(&p.pose).enumerate() {
                if w >= 1.0 {
                    out.pose.set(j, t);
                    owned = true;
                }
            }
            if owned {
                local_to_model(b.skeleton, &out.pose, &mut out.model, &Affine3A::IDENTITY);
            }
        }
        // `ClothLattice` (physics.md "Cloth"): last, on the final pose (after foot placement).
        for c in &mut self.cloth {
            c.update(&out.model, inputs.moved, dt, lod);
        }
        out.feet = pcx.feet;
        self.hits.clear();
    }

    /// Where the controller should probe before the next update: each
    /// animated effector (before IK) extrapolated one update ahead by its
    /// model-space motion over this one. The controller casts them after
    /// the mover has moved the character, in the new frame, so this is
    /// where the next update's animated foot will be; a landing foot then
    /// samples the step it lands on, not the one it left a frame earlier
    /// (the footprint prediction of Johansen 2009, to first order). The
    /// lead is capped at a quarter of the limb's root-to-effector distance
    /// so a pose snap can't throw a probe far away.
    fn probe_origins(&mut self, g: &CompiledGraph, dt: f32, out: &mut GraphOutput) {
        out.probe_origins.clear();
        let history = self.effectors.len() == g.legs.len();
        let mut leads = Vec::with_capacity(g.legs.len());
        for (i, leg) in g.legs.iter().enumerate() {
            let f = out.model.translation(usize::from(leg.effector()));
            let lead = if history && dt > 0.0 {
                let cap = 0.25 * out.model.translation(usize::from(leg.joints[0])).distance(f);
                (f - self.effectors[i]).clamp_length_max(cap)
            } else {
                Vec3::ZERO
            };
            out.probe_origins.push(f + lead);
            leads.push(lead);
        }
        // Toe probes (slots after the legs, in leg order) share their foot's lead.
        for (leg, lead) in g.legs.iter().zip(leads) {
            if let Some((toe, _, _)) = leg.toe {
                out.probe_origins.push(out.model.translation(usize::from(toe)) + lead);
            }
        }
        if dt > 0.0 {
            self.effectors.clear();
            self.effectors.extend(g.legs.iter().map(|l| out.model.translation(usize::from(l.effector()))));
        }
    }

    // --- Motion warping ----------------------------------------------------------------

    /// Motion warping (Witkin & Popović 1995, as UE5's Motion Warping): the
    /// highest-weighted `Drive` lane inside a `warp` window whose target is
    /// set has the window's part of its root delta warped so the warp point
    /// arrives at the target at the window's end. Translation: the
    /// remaining authored path R is mapped onto the vector to the target D
    /// by the rank-one skew `δ' = δ + (D − R)(R·δ)/|R|²` (or a uniform
    /// scale), eased in over `ease` of the window; recomputed every update
    /// against the current target, so moving targets are tracked and the
    /// last update lands exactly. Rotation: the remaining turn to the
    /// target facing (or toward the target) spread until `rot_time`,
    /// limited by `max_turn`. Returns what it did; `root` is adjusted by
    /// the lane's share of the pose.
    fn apply_warp(&self, g: &CompiledGraph, clips: &[Arc<Clip>], inputs: &UpdateInputs<'_>, dt: f32, root: &mut RootDelta) -> Option<ActiveWarp> {
        let mut best: Option<(f32, &Clip, &WarpWindow, f32, f32)> = None;
        for &n in &self.ticks {
            let (NodeKind::Player(def), NodeState::Player(st)) = (&g.nodes[n as usize].kind, &self.nodes[n as usize]) else {
                continue;
            };
            if def.root_motion != RootMotionMode::Drive {
                continue;
            }
            for (i, lane) in st.lanes.iter().enumerate() {
                let w = st.weight * lane.weight;
                if w <= EPS || lane.wrapped || lane.time <= lane.prev {
                    continue;
                }
                for win in &g.warp_windows[usize::from(def.lanes[i])] {
                    let open = lane.prev < win.end && lane.time > win.start;
                    let target_set = inputs.warp_targets.get(usize::from(win.target)).is_some_and(Option::is_some);
                    if open && target_set && best.is_none_or(|b| w > b.0) {
                        best = Some((w, &clips[usize::from(def.lanes[i])], win, lane.prev, lane.time));
                    }
                }
            }
        }
        for st in &self.slots {
            let Some(m) = &st.playing else { continue };
            if m.req.root_motion != RootMotionMode::Drive || st.weight <= EPS || m.wrapped || m.time <= m.prev {
                continue;
            }
            for win in warp_windows(g, &m.req.clip) {
                let open = m.prev < win.end && m.time > win.start;
                let target_set = inputs.warp_targets.get(usize::from(win.target)).is_some_and(Option::is_some);
                if open && target_set && best.is_none_or(|b| st.weight > b.0) {
                    best = Some((st.weight, clip_of(clips, &m.req.clip), win, m.prev, m.time));
                }
            }
        }
        let (w, clip, win, t0, t1) = best?;
        let target = inputs.warp_targets[usize::from(win.target)]?;
        let (a, b) = (t0.max(win.start), t1.min(win.end));
        let (pre, mid, post) = (clip.root_delta(t0, a), clip.root_delta(a, b), clip.root_delta(b, t1));
        // Everything below is in the root's frame at `a`.
        let ahead = if t1 > t0 { dt * (b - t0) / (t1 - t0) } else { 0.0 };
        let target_pos = Quat::from_rotation_y(-pre.yaw) * (target.position + target.velocity * ahead - pre.translation);
        let rem = clip.root_delta(a, win.end);
        let span = (win.end - win.start).max(1e-6);
        let remaining = (win.end - a).max(1e-6);

        let yaw_goal = match win.rotation {
            WarpRotation::Default => wrap_angle(target.yaw - pre.yaw),
            WarpRotation::Facing if Vec2::new(target_pos.x, target_pos.z).length() > 1e-3 => target_pos.x.atan2(target_pos.z),
            WarpRotation::Facing | WarpRotation::None => rem.yaw,
        };
        let correction = wrap_angle(yaw_goal - rem.yaw);
        let rot_end = win.start + win.rot_time * span;
        let frac = if rot_end - a > 1e-6 { ((b - a) / (rot_end - a)).min(1.0) } else { 1.0 };
        let max_step = if dt > 0.0 { win.max_turn * dt } else { f32::INFINITY };
        let add_yaw = (correction * frac).clamp(-max_step, max_step);

        let mut d = target_pos - Quat::from_rotation_y(rem.yaw + correction) * win.point;
        let r = rem.translation;
        if !win.vertical {
            d.y = r.y;
        }
        let mut error = 0.0;
        let c = d - r;
        if c.length() > win.max_dist {
            error = c.length() - win.max_dist;
            d = r + c.normalize() * win.max_dist;
        }
        let delta = mid.translation;
        let linear = d * ((b - a) / remaining);
        let warped = match win.mode {
            WarpMode::Skew if r.length_squared() > 1e-8 => delta + (d - r) * (delta.dot(r) / r.length_squared()),
            WarpMode::Scale if r.length() > 1e-4 => delta * (d.length() / r.length()),
            _ => linear,
        };
        let ease = if win.ease > 0.0 { smoothstep((b - win.start) / (win.ease * span)) } else { 1.0 };
        let warped = delta + (warped - delta) * ease;
        let authored = pre.then(mid).then(post);
        let new = pre.then(RootDelta { translation: warped, yaw: mid.yaw + add_yaw, tilt: mid.tilt }).then(post);
        root.translation += (new.translation - authored.translation) * w;
        root.yaw += (new.yaw - authored.yaw) * w;
        Some(ActiveWarp { target: win.target, remaining: win.end - b, error, yaw_error: correction - add_yaw, vertical: win.vertical })
    }

    // --- Prepare ------------------------------------------------------------------------

    fn player_mut(&mut self, n: usize) -> &mut PlayerState {
        match &mut self.nodes[n] {
            NodeState::Player(p) => p,
            _ => unreachable!("player node without player state"),
        }
    }

    fn prepare(&mut self, b: &Binding<'_>, cx: &Cx<'_>, node: u32, w: f32, rate: f32, layer: u16) {
        let n = node as usize;
        let g = cx.g;
        match &g.nodes[n].kind {
            NodeKind::Player(p) => {
                // Variety: looping clips that don't drive root motion (idles, and
                // locomotion, whose speed match divides it back out).
                let jitter = if p.looping && p.root_motion != RootMotionMode::Drive { self.variety.rate } else { 1.0 };
                let own_rate = rate * self.params.value(p.rate) * jitter;
                let (x, y) = match &p.space {
                    Some(s) => (self.params.value(s.x), s.y.map_or(0.0, |v| self.params.value(v))),
                    None => (0.0, 0.0),
                };
                let st = self.player_mut(n);
                st.weight = w;
                st.rate = own_rate;
                st.layer = layer;
                match &p.space {
                    Some(space) => {
                        let sx = st.input[0].update(x, space.smoothing, cx.dt);
                        let sy = st.input[1].update(y, space.smoothing, cx.dt);
                        let (weights, count) = space.weights(space.normalize(Vec2::new(sx, sy)));
                        for lane in &mut st.lanes {
                            lane.weight = 0.0;
                        }
                        for &(i, wt) in &weights[..count] {
                            st.lanes[usize::from(i)].weight = wt;
                        }
                    }
                    None => st.lanes[0].weight = 1.0,
                }
                self.ticks.push(node);
            }
            NodeKind::Blend { a, b: bb, alpha, smoothing, mask } => {
                let goal = self.params.value(*alpha).clamp(0.0, 1.0);
                let NodeState::Smooth(s) = &mut self.nodes[n] else { unreachable!() };
                let al = s.update(goal, *smoothing, cx.dt).clamp(0.0, 1.0);
                match mask {
                    Some(m) => {
                        let mr = g.mask_weights()[usize::from(*m)].get(usize::from(g.root_joint));
                        self.prepare(b, cx, *a, w * (1.0 - al * mr), rate, layer);
                    }
                    None if 1.0 - al > EPS => self.prepare(b, cx, *a, w * (1.0 - al), rate, layer),
                    None => {}
                }
                if al > EPS {
                    self.prepare(b, cx, *bb, w * al, rate, layer);
                }
            }
            NodeKind::Additive { base, additive, weight, .. } => {
                self.prepare(b, cx, *base, w, rate, layer);
                let aw = self.params.value(*weight).clamp(0.0, 1.0);
                if aw > EPS {
                    self.prepare(b, cx, *additive, w * aw, rate, layer);
                }
            }
            NodeKind::Layers { base, layers } => {
                self.prepare(b, cx, *base, w, rate, layer);
                let top = node == g.root;
                for (i, l) in layers.iter().enumerate() {
                    let lw = self.params.value(l.weight).clamp(0.0, 1.0);
                    if lw > EPS {
                        let id = if top { u16::try_from(i + 1).unwrap_or(u16::MAX) } else { layer };
                        self.prepare(b, cx, l.node, w * lw, rate, id);
                    }
                }
            }
            NodeKind::StateMachine(m) => {
                self.tick_machine(b, cx, n, m);
                let NodeState::Machine(ms) = &self.nodes[n] else { unreachable!() };
                let active: Vec<(u16, f32)> = ms.weights.clone();
                for (s, sw) in active {
                    if sw > EPS {
                        self.prepare(b, cx, m.states[usize::from(s)].node, w * sw, rate, layer);
                    }
                }
            }
            NodeKind::RateScale { input, rate: r } => {
                let r = self.params.value(*r);
                self.prepare(b, cx, *input, w, rate * r, layer);
            }
            NodeKind::Mirror { input, .. } => self.prepare(b, cx, *input, w, rate, layer),
            NodeKind::PoseInput { .. } => {}
            NodeKind::Slot { slot, input, mask } => {
                let st = &mut self.slots[usize::from(*slot)];
                if w > st.weight {
                    st.layer = layer;
                }
                st.weight += w;
                // A full-body montage hides the input, which then holds (UE's
                // slot without "always update source pose"); a masked one
                // shows the input elsewhere, so it plays on.
                if st.playing.is_none() || mask.is_some() {
                    self.prepare(b, cx, *input, w, rate, layer);
                }
            }
        }
    }

    /// Resets the nodes `start..end` (a subtree) to their entry state.
    fn reset_range(&mut self, b: &Binding<'_>, start: u32, end: u32) {
        let offset = self.variety.phase;
        for k in start as usize..end as usize {
            match (&b.graph.nodes[k].kind, &mut self.nodes[k]) {
                (NodeKind::Player(p), NodeState::Player(st)) => {
                    for (lane, &clip) in st.lanes.iter_mut().zip(&p.lanes) {
                        let c = &b.clips[usize::from(clip)];
                        let (a, e) = loop_span(c, p.looping);
                        let time = if p.looping { a + (p.start + offset).rem_euclid(1.0) * (e - a) } else { p.start * c.duration };
                        *lane = Lane { time, prev: time, ..Lane::default() };
                    }
                    st.fresh = true;
                    st.finished = false;
                    st.input = [Smooth::default(); 2];
                    st.events.clear();
                }
                (NodeKind::StateMachine(m), NodeState::Machine(ms)) => {
                    *ms = MachineState {
                        current: m.initial,
                        alpha: 1.0,
                        target_start: 1.0,
                        interruptible: true,
                        weights: vec![(m.initial, 1.0)],
                        ..MachineState::default()
                    };
                }
                (_, NodeState::Smooth(s)) => *s = Smooth::default(),
                _ => {}
            }
        }
    }

    /// The highest-weighted player under `node` (by the previous update's
    /// weights; the first one if none played yet).
    fn dominant(&self, g: &CompiledGraph, node: u32) -> Option<usize> {
        let end = g.nodes[node as usize].subtree_end as usize;
        let mut best: Option<(usize, f32)> = None;
        for k in node as usize..end {
            if let NodeState::Player(p) = &self.nodes[k]
                && best.is_none_or(|(_, w)| p.prev_weight > w)
            {
                best = Some((k, p.prev_weight));
            }
        }
        best.map(|(k, _)| k)
    }

    fn holds(&self, cx: &Cx<'_>, state_node: u32, state_time: f32, c: &Condition) -> bool {
        let g = cx.g;
        let player = |k: usize| match (&g.nodes[k].kind, &self.nodes[k]) {
            (NodeKind::Player(def), NodeState::Player(st)) => Some((def, st)),
            _ => None,
        };
        let lead = || {
            self.dominant(g, state_node).and_then(player).map(|(def, st)| {
                let lane = st.lanes[st.leader.min(st.lanes.len() - 1)];
                (&cx.clips[usize::from(def.lanes[st.leader.min(def.lanes.len() - 1)])], def.looping, lane.time, st.finished)
            })
        };
        match *c {
            Condition::IsTrue(v) => self.params.value(v) > 0.5,
            Condition::IsFalse(v) => self.params.value(v) <= 0.5,
            Condition::Greater(v, x) => self.params.value(v) > x,
            Condition::Less(v, x) => self.params.value(v) < x,
            Condition::Trigger(p) => self.params.values[usize::from(p)][0] > 0.5,
            Condition::Event(id) => {
                let end = g.nodes[state_node as usize].subtree_end as usize;
                (state_node as usize..end).filter_map(player).any(|(def, st)| {
                    st.events.iter().any(|(lane, e)| {
                        let clip = usize::from(def.lanes[usize::from(*lane)]);
                        e.edge != EventEdge::Exit && g.clip_events[clip].get(e.event as usize) == Some(&id)
                    })
                })
            }
            Condition::PhaseAtLeast(x) => lead().is_some_and(|(clip, looping, t, _)| phase(clip, looping, t) >= x),
            Condition::TimeLeft(x) => lead().is_some_and(|(clip, looping, t, _)| !looping && clip.duration - t <= x),
            Condition::Finished => lead().is_some_and(|(.., finished)| finished),
            Condition::StateTime(x) => state_time >= x,
        }
    }

    fn tick_machine(&mut self, b: &Binding<'_>, cx: &Cx<'_>, n: usize, m: &Machine) {
        let NodeState::Machine(ms) = &mut self.nodes[n] else { unreachable!() };
        ms.state_time += cx.dt;
        let (current, alpha, interruptible, state_time) = (ms.current, ms.alpha, ms.interruptible, ms.state_time);
        let state_node = m.states[usize::from(current)].node;
        let fired = if alpha >= 1.0 || interruptible {
            m.transitions.iter().find(|t| {
                let from_ok = match t.from {
                    Some(f) => f == current,
                    None => t.to != current,
                };
                from_ok && t.when.iter().all(|c| self.holds(cx, state_node, state_time, c))
            })
        } else {
            None
        };
        if let Some(t) = fired {
            for c in &t.when {
                if let Condition::Trigger(p) = c {
                    self.params.values[usize::from(*p)][0] = 0.0;
                }
            }
            let NodeState::Machine(ms) = &self.nodes[n] else { unreachable!() };
            let current_w = ms.weights.iter().find(|(s, _)| *s == t.to).map_or(0.0, |x| x.1);
            let restart = current_w <= EPS || t.to == current;
            let snapshot: Vec<(u16, f32)> = ms.weights.iter().copied().filter(|(s, _)| *s != t.to).collect();
            if restart {
                let node = m.states[usize::from(t.to)].node;
                self.reset_range(b, node, cx.g.nodes[node as usize].subtree_end);
            }
            let NodeState::Machine(ms) = &mut self.nodes[n] else { unreachable!() };
            ms.current = t.to;
            ms.state_time = 0.0;
            ms.interruptible = t.interruptible;
            // Re-entering the current state can't fade against itself: it
            // inertializes instead.
            let blend = match t.blend {
                Blend::Crossfade(d) if t.to == current => Blend::Inertialize(d),
                other => other,
            };
            match blend {
                Blend::Crossfade(d) if d > 0.0 => {
                    ms.fading = snapshot;
                    ms.target_start = if restart { 0.0 } else { current_w };
                    ms.alpha = 0.0;
                    ms.duration = d;
                }
                Blend::Inertialize(d) => {
                    ms.fading.clear();
                    ms.alpha = 1.0;
                    ms.target_start = 1.0;
                    self.request = Some(self.request.map_or(d, |r| r.min(d)));
                }
                _ => {
                    ms.fading.clear();
                    ms.alpha = 1.0;
                    ms.target_start = 1.0;
                }
            }
        } else if ms_alpha(&self.nodes[n]) < 1.0 {
            let NodeState::Machine(ms) = &mut self.nodes[n] else { unreachable!() };
            ms.alpha = (ms.alpha + cx.dt / ms.duration.max(EPS)).min(1.0);
        }
        let NodeState::Machine(ms) = &mut self.nodes[n] else { unreachable!() };
        if ms.alpha >= 1.0 {
            ms.fading.clear();
        }
        let s = smoothstep(ms.alpha);
        let target = ms.target_start + (1.0 - ms.target_start) * s;
        ms.weights.clear();
        let current = ms.current;
        ms.weights.extend(ms.fading.iter().map(|&(st, w0)| (st, w0 * (1.0 - s))));
        ms.weights.push((current, target));
    }

    // --- Advance ------------------------------------------------------------------------

    fn advance(&mut self, cx: &Cx<'_>, inputs: &UpdateInputs<'_>) {
        let ticks = std::mem::take(&mut self.ticks);
        let mut groups = std::mem::take(&mut self.groups);
        for gr in &mut groups {
            gr.clear();
        }
        for &n in &ticks {
            match &cx.g.nodes[n as usize].kind {
                NodeKind::Player(PlayerNode { sync: Some((gid, _)), .. }) => groups[usize::from(*gid)].push(n),
                _ => self.advance_own(cx, n as usize, inputs),
            }
        }
        for members in &groups {
            let Some(leader) = self.choose_leader(cx, members) else {
                continue;
            };
            self.advance_own(cx, leader, inputs);
            for &f in members {
                if f as usize != leader {
                    self.follow(cx, f as usize, leader);
                }
            }
        }
        self.groups = groups;
        self.ticks = ticks;
    }

    fn choose_leader(&self, cx: &Cx<'_>, members: &[u32]) -> Option<usize> {
        let role = |n: u32| match &cx.g.nodes[n as usize].kind {
            NodeKind::Player(p) => p.sync.map_or(SyncRole::CanLead, |s| s.1),
            _ => SyncRole::NeverLead,
        };
        let weight = |n: u32| match &self.nodes[n as usize] {
            NodeState::Player(p) => p.weight,
            _ => 0.0,
        };
        let best = |pred: &dyn Fn(SyncRole) -> bool| {
            members.iter().copied().filter(|&n| pred(role(n))).fold(None, |acc: Option<u32>, n| match acc {
                Some(a) if weight(a) >= weight(n) => Some(a),
                _ => Some(n),
            })
        };
        best(&|r| r == SyncRole::AlwaysLead).or_else(|| best(&|r| r == SyncRole::CanLead)).or_else(|| best(&|_| true)).map(|n| n as usize)
    }

    fn advance_own(&mut self, cx: &Cx<'_>, n: usize, inputs: &UpdateInputs<'_>) {
        let NodeKind::Player(def) = &cx.g.nodes[n].kind else {
            return;
        };
        let speed_match = self.speed_match;
        let mut stride = None;
        let mut anim_vel = Vec3::ZERO;
        let mut stride_vel = Vec3::ZERO;
        let st = self.player_mut(n);
        let mut rate = st.rate;
        let dt = cx.dt;
        if def.root_motion == RootMotionMode::Match {
            let total: f32 = st.lanes.iter().map(|l| l.weight).sum::<f32>().max(EPS);
            for (lane, &c) in st.lanes.iter().zip(&def.lanes) {
                let meta = &cx.clips[usize::from(c)].meta;
                anim_vel += meta.average_velocity * (lane.weight / total);
                stride_vel += meta.stride_velocity() * (lane.weight / total);
            }
            // Speed matching plays by the planted feet's speed where it
            // differs from the root's (sidesteps, backpedals), so a foot
            // lock holds; the body direction stays the root's.
            let natural = Vec2::new(stride_vel.x, stride_vel.z).length() * rate;
            if let Some((min, max)) = speed_match
                && natural > 1e-3
            {
                let speed = Vec2::new(inputs.velocity.x, inputs.velocity.z).length();
                let m = (speed / natural).clamp(min, max);
                rate *= m;
                stride = Some(speed / (natural * m));
            }
        }
        let leader = argmax(st.lanes.iter().map(|l| l.weight));
        st.leader = leader;
        // A blend space plays at the weighted length of its samples.
        let length = |i: usize| {
            let c = &cx.clips[usize::from(def.lanes[i])];
            let (a, b) = loop_span(c, def.looping);
            (b - a) / def.lane_rates[i].max(EPS)
        };
        let weighted: f32 = (0..st.lanes.len()).map(|i| st.lanes[i].weight * length(i)).sum();
        let leader_len = length(leader);
        let delta = if def.lanes.len() == 1 || weighted <= EPS {
            dt * rate * def.lane_rates[leader]
        } else {
            dt * rate * leader_len * def.lane_rates[leader] / weighted
        };
        let clip = &cx.clips[usize::from(def.lanes[leader])];
        st.events.clear();
        let mut fired = Vec::new();
        let lane = &mut st.lanes[leader];
        let adv = clip.advance(lane.time, delta, def.looping, st.fresh, Some(&mut fired));
        lane.prev = lane.time;
        lane.wrapped = adv.wraps > 0;
        lane.time = adv.time;
        lane.root = adv.root;
        if dt > 0.0 {
            lane.speed = delta / dt;
        }
        st.finished = adv.finished;
        st.events.extend(fired.into_iter().map(|e| (u16::try_from(leader).unwrap_or(0), e)));
        sync_lanes(cx, def, st, leader, delta >= 0.0);
        st.fresh = false;
        let w = st.weight;
        if let Some(r) = stride {
            self.stride.0 += r * w;
            self.stride.1 += w;
        }
        self.anim_velocity += anim_vel * w;
    }

    fn follow(&mut self, cx: &Cx<'_>, f: usize, leader: usize) {
        let (NodeKind::Player(ldef), NodeKind::Player(fdef)) = (&cx.g.nodes[leader].kind, &cx.g.nodes[f].kind) else {
            return;
        };
        let NodeState::Player(lst) = &self.nodes[leader] else {
            return;
        };
        let l_lane = lst.leader.min(lst.lanes.len() - 1);
        let (l_clip, l_time, l_forward) = (&cx.clips[usize::from(ldef.lanes[l_lane])], lst.lanes[l_lane].time, lst.lanes[l_lane].speed >= 0.0);
        let st = self.player_mut(f);
        let lead = argmax(st.lanes.iter().map(|l| l.weight));
        st.leader = lead;
        st.events.clear();
        let clip = &cx.clips[usize::from(fdef.lanes[lead])];
        let old = st.lanes[lead].time;
        let new = sync_time(clip, fdef.looping, old, l_clip, ldef.looping, l_time);
        move_lane(cx.dt, clip, fdef.looping, st, lead, old, new, l_forward);
        sync_lanes(cx, fdef, st, lead, l_forward);
        st.finished = false;
        st.fresh = false;
    }

    // --- Events -------------------------------------------------------------------------

    fn collect_events(&mut self, g: &CompiledGraph, out: &mut GraphOutput) {
        out.events.clear();
        // Montages fire all their events, outside the layer selection.
        for st in &mut self.slots {
            out.events.append(&mut st.exits);
            if let Some(m) = &st.playing {
                for e in &m.events {
                    let (clip, id) = event_ids(g, &m.req.clip, e.event);
                    out.events.push(GraphEvent { clip, event: e.event, id, edge: e.edge, weight: st.weight, layer: st.layer, montage: Some(m.id) });
                }
            }
        }
        // Per layer, the highest-weighted lane fires point events.
        let mut dominant: Vec<(u16, usize, usize, f32)> = Vec::new();
        for &n in &self.ticks {
            let NodeState::Player(st) = &self.nodes[n as usize] else {
                continue;
            };
            for (i, lane) in st.lanes.iter().enumerate() {
                let w = st.weight * lane.weight;
                match dominant.iter_mut().find(|d| d.0 == st.layer) {
                    Some(d) if d.3 >= w => {}
                    Some(d) => *d = (st.layer, n as usize, i, w),
                    None => dominant.push((st.layer, n as usize, i, w)),
                }
            }
        }
        for &n in &self.ticks {
            let (NodeKind::Player(def), NodeState::Player(st)) = (&g.nodes[n as usize].kind, &self.nodes[n as usize]) else {
                continue;
            };
            for &(lane, e) in &st.events {
                let w = st.weight * st.lanes[usize::from(lane)].weight;
                let keep = match e.edge {
                    EventEdge::Point => dominant.iter().any(|d| d.0 == st.layer && d.1 == n as usize && d.2 == usize::from(lane)),
                    EventEdge::Enter | EventEdge::Exit => w >= 0.5 - EPS,
                };
                if keep {
                    let clip = def.lanes[usize::from(lane)];
                    let id = g.clip_events[usize::from(clip)].get(e.event as usize).copied().unwrap_or(u16::MAX);
                    out.events.push(GraphEvent { clip, event: e.event, id, edge: e.edge, weight: w, layer: st.layer, montage: None });
                }
            }
        }
    }
}

fn ms_alpha(s: &NodeState) -> f32 {
    match s {
        NodeState::Machine(m) => m.alpha,
        _ => 1.0,
    }
}

fn smoothstep(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

fn argmax(it: impl Iterator<Item = f32>) -> usize {
    let mut best = (0, f32::NEG_INFINITY);
    for (i, w) in it.enumerate() {
        if w > best.1 {
            best = (i, w);
        }
    }
    best.0
}

/// Moves lane `i` from `old` to `new`, collecting events and root motion
/// when the move is in the play direction.
#[allow(clippy::too_many_arguments)]
fn move_lane(dt: f32, clip: &Clip, looping: bool, st: &mut PlayerState, i: usize, old: f32, new: f32, forward: bool) {
    let delta = wrapped_delta(old, new, loop_span(clip, looping), looping);
    let lane_idx = u16::try_from(i).unwrap_or(0);
    let mut fired = Vec::new();
    let lane = &mut st.lanes[i];
    if (delta >= 0.0) == forward {
        let adv = clip.advance(old, delta, looping, st.fresh, Some(&mut fired));
        lane.root = adv.root;
    } else {
        lane.root = RootDelta::ZERO;
    }
    lane.prev = lane.time;
    lane.wrapped = (delta >= 0.0 && new < old) || (delta < 0.0 && new > old);
    lane.time = new;
    if dt > 0.0 {
        lane.speed = delta / dt;
    }
    st.events.extend(fired.into_iter().map(|e| (lane_idx, e)));
}

/// Places every lane of a blend space relative to its leader lane.
fn sync_lanes(cx: &Cx<'_>, def: &PlayerNode, st: &mut PlayerState, leader: usize, forward: bool) {
    if def.lanes.len() < 2 {
        return;
    }
    let l_clip = &cx.clips[usize::from(def.lanes[leader])];
    let l_time = st.lanes[leader].time;
    for i in 0..def.lanes.len() {
        if i == leader {
            continue;
        }
        let clip = &cx.clips[usize::from(def.lanes[i])];
        let old = st.lanes[i].time;
        let new = sync_time(clip, def.looping, old, l_clip, def.looping, l_time);
        move_lane(cx.dt, clip, def.looping, st, i, old, new, forward);
    }
}

// --- Evaluate --------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct EvalCx<'a> {
    g: &'a CompiledGraph,
    clips: &'a [Arc<Clip>],
    skeleton: &'a Skeleton,
    nodes: &'a [NodeState],
    params: &'a Params,
    rest: &'a LocalPose,
    identity: &'a LocalPose,
    mask: Option<&'a JointMask>,
    posed: bool,
    /// Seconds to look ahead (inertialization's target velocity).
    shift: f32,
    poses: &'a [&'a LocalPose],
    slots: &'a [SlotState],
}

fn lerp_root(a: RootDelta, b: RootDelta, t: f32) -> RootDelta {
    RootDelta { translation: a.translation.lerp(b.translation, t), yaw: a.yaw + (b.yaw - a.yaw) * t, tilt: a.tilt.lerp(b.tilt, t) }
}

fn lerp_curves(a: &mut [f32], b: &[f32], t: f32) {
    for (x, &y) in a.iter_mut().zip(b) {
        if y.is_nan() {
            continue;
        }
        *x = if x.is_nan() { y } else { *x + (y - *x) * t };
    }
}

/// [`lerp_curves`] through a joint mask: a curve linked to a joint
/// ([`CompiledGraph::curve_joints`]) blends by `t` times that joint's mask
/// weight, so a masked layer never overrides curves of joints it leaves
/// alone (an upper-body idle over a walk keeps the walk's foot contacts).
fn lerp_curves_masked(a: &mut [f32], b: &[f32], t: f32, mask: Option<&JointWeights>, joints: &[Option<u16>]) {
    let Some(mask) = mask else {
        lerp_curves(a, b, t);
        return;
    };
    for (i, (x, &y)) in a.iter_mut().zip(b).enumerate() {
        let w = joints.get(i).copied().flatten().map_or(t, |j| t * mask.get(usize::from(j)));
        if y.is_nan() || w <= EPS {
            continue;
        }
        *x = if x.is_nan() { y } else { *x + (y - *x) * w };
    }
}

/// Combines frames `d+1 .. d+1+weights.len()` into frame `d` with weights.
fn combine(ecx: &EvalCx<'_>, s: &mut Scratch, d: usize, weights: &[f32], additive: bool) {
    let (lo, hi) = s.frames.split_at_mut(d + 1);
    let out = &mut lo[d];
    let inputs = &hi[..weights.len()];
    let total: f32 = weights.iter().sum::<f32>().max(EPS);
    if ecx.posed {
        let layers: Vec<BlendLayer<'_>> =
            inputs.iter().zip(weights).map(|(f, &w)| BlendLayer { pose: &f.pose, weight: w, joint_weights: None }).collect();
        let rest = if additive { ecx.identity } else { ecx.rest };
        blend_layers(&mut out.pose, &layers, rest, 1e-4);
    }
    let mut root = RootDelta::ZERO;
    let mut drive = 0.0;
    for (f, &w) in inputs.iter().zip(weights) {
        root.translation += f.root.translation * (w / total);
        root.yaw += f.root.yaw * (w / total);
        root.tilt += f.root.tilt * (w / total);
        drive += f.drive * (w / total);
    }
    out.root = root;
    out.drive = drive;
    s.curve_w.iter_mut().for_each(|x| *x = 0.0);
    out.curves.iter_mut().for_each(|x| *x = 0.0);
    for (f, &w) in inputs.iter().zip(weights) {
        for ((acc, cw), &v) in out.curves.iter_mut().zip(s.curve_w.iter_mut()).zip(&f.curves) {
            if !v.is_nan() {
                *acc += v * w;
                *cw += w;
            }
        }
    }
    for (acc, &cw) in out.curves.iter_mut().zip(&s.curve_w) {
        *acc = if cw > 0.0 { *acc / cw } else { f32::NAN };
    }
}

fn sample_time(clip: &Clip, looping: bool, lane: &Lane, shift: f32) -> f32 {
    if shift == 0.0 {
        return lane.time;
    }
    let t = lane.time + shift * lane.speed;
    let (a, b) = loop_span(clip, looping);
    if looping && b - a > 1e-6 && lane.time >= a { a + (t - a).rem_euclid(b - a) } else { t.clamp(0.0, clip.duration) }
}

/// Evaluates `node` into frame `d` (frames above `d` are scratch).
#[allow(clippy::too_many_lines)]
fn eval(ecx: &EvalCx<'_>, s: &mut Scratch, node: u32, d: usize) {
    let g = ecx.g;
    let joints = g.joint_count();
    let curves = g.curves.len();
    let n = node as usize;
    let gnode = &g.nodes[n];
    match &gnode.kind {
        NodeKind::Player(def) => {
            let NodeState::Player(st) = &ecx.nodes[n] else { unreachable!() };
            let active: Vec<usize> = (0..st.lanes.len()).filter(|&i| st.lanes[i].weight > 0.0).collect();
            s.ensure(d + 2 + active.len(), joints, curves);
            if active.len() <= 1 {
                let i = active.first().copied().unwrap_or(0);
                let clip = &ecx.clips[usize::from(def.lanes[i])];
                let f = &mut s.frames[d];
                if ecx.posed {
                    clip.sample(sample_time(clip, def.looping, &st.lanes[i], ecx.shift), &mut f.pose, ecx.mask);
                }
                write_curves(&g.clip_curves[usize::from(def.lanes[i])], clip, st.lanes[i].time, &mut f.curves);
                let drive = def.root_motion == RootMotionMode::Drive;
                f.root = if drive { st.lanes[i].root } else { RootDelta::ZERO };
                f.drive = if drive { 1.0 } else { 0.0 };
            } else {
                let mut weights = Vec::with_capacity(active.len());
                for (k, &i) in active.iter().enumerate() {
                    let clip = &ecx.clips[usize::from(def.lanes[i])];
                    let f = &mut s.frames[d + 1 + k];
                    if ecx.posed {
                        clip.sample(sample_time(clip, def.looping, &st.lanes[i], ecx.shift), &mut f.pose, ecx.mask);
                    }
                    write_curves(&g.clip_curves[usize::from(def.lanes[i])], clip, st.lanes[i].time, &mut f.curves);
                    let drive = def.root_motion == RootMotionMode::Drive;
                    f.root = if drive { st.lanes[i].root } else { RootDelta::ZERO };
                    f.drive = if drive { 1.0 } else { 0.0 };
                    weights.push(st.lanes[i].weight);
                }
                combine(ecx, s, d, &weights, gnode.additive);
            }
        }
        NodeKind::Blend { a, b, mask, .. } => {
            let NodeState::Smooth(sm) = &ecx.nodes[n] else { unreachable!() };
            let al = sm.value.clamp(0.0, 1.0);
            if mask.is_none() && al <= EPS {
                eval(ecx, s, *a, d);
            } else if mask.is_none() && al >= 1.0 - EPS {
                eval(ecx, s, *b, d);
            } else {
                s.ensure(d + 2, joints, curves);
                eval(ecx, s, *a, d);
                if al > EPS {
                    eval(ecx, s, *b, d + 1);
                    let mw = mask.map(|m| &g.mask_weights()[usize::from(m)]);
                    let rw = mw.map_or(1.0, |m| m.get(usize::from(g.root_joint)));
                    let (lo, hi) = s.frames.split_at_mut(d + 1);
                    let (x, y) = (&mut lo[d], &hi[0]);
                    if ecx.posed {
                        blend_in_place(&mut x.pose, &y.pose, al, mw);
                    }
                    lerp_curves(&mut x.curves, &y.curves, al);
                    x.root = lerp_root(x.root, y.root, al * rw);
                    x.drive += (y.drive - x.drive) * al * rw;
                }
            }
        }
        NodeKind::Additive { base, additive, weight, mask } => {
            eval(ecx, s, *base, d);
            let aw = ecx.params.value(*weight).clamp(0.0, 1.0);
            if aw > EPS && ecx.posed {
                s.ensure(d + 2, joints, curves);
                eval(ecx, s, *additive, d + 1);
                let mw = mask.map(|m| &g.mask_weights()[usize::from(m)]);
                let (lo, hi) = s.frames.split_at_mut(d + 1);
                apply_additive(&mut lo[d].pose, &hi[0].pose, aw, mw);
            }
        }
        NodeKind::Layers { base, layers } => {
            eval(ecx, s, *base, d);
            for l in layers {
                let lw = ecx.params.value(l.weight).clamp(0.0, 1.0);
                if lw <= EPS {
                    continue;
                }
                s.ensure(d + 2, joints, curves);
                eval(ecx, s, l.node, d + 1);
                let mw = l.mask.map(|m| &g.mask_weights()[usize::from(m)]);
                let (lo, hi) = s.frames.split_at_mut(d + 1);
                let (x, y) = (&mut lo[d], &hi[0]);
                match l.mode {
                    LayerMode::Override => {
                        if ecx.posed {
                            blend_in_place(&mut x.pose, &y.pose, lw, mw);
                        }
                        let rw = mw.map_or(1.0, |m| m.get(usize::from(g.root_joint)));
                        lerp_curves_masked(&mut x.curves, &y.curves, lw, mw, g.curve_joints());
                        x.root = lerp_root(x.root, y.root, lw * rw);
                        x.drive += (y.drive - x.drive) * lw * rw;
                    }
                    LayerMode::Additive => {
                        if ecx.posed {
                            apply_additive(&mut x.pose, &y.pose, lw, mw);
                        }
                    }
                }
            }
        }
        NodeKind::StateMachine(m) => {
            let NodeState::Machine(ms) = &ecx.nodes[n] else { unreachable!() };
            let active: Vec<(u16, f32)> = ms.weights.iter().copied().filter(|x| x.1 > EPS).collect();
            if active.len() <= 1 {
                let st = active.first().map_or(ms.current, |x| x.0);
                eval(ecx, s, m.states[usize::from(st)].node, d);
            } else {
                s.ensure(d + 1 + active.len(), joints, curves);
                for (k, &(st, _)) in active.iter().enumerate() {
                    eval(ecx, s, m.states[usize::from(st)].node, d + 1 + k);
                }
                let weights: Vec<f32> = active.iter().map(|x| x.1).collect();
                combine(ecx, s, d, &weights, gnode.additive);
            }
        }
        NodeKind::RateScale { input, .. } => eval(ecx, s, *input, d),
        NodeKind::Mirror { input, weight } => {
            eval(ecx, s, *input, d);
            let mw = ecx.params.value(*weight).clamp(0.0, 1.0);
            if let (Some(table), true) = (&g.mirror, mw > EPS) {
                s.ensure(d + 2, joints, curves);
                let Scratch { frames, model, model2, .. } = s;
                let (lo, hi) = frames.split_at_mut(d + 1);
                let (x, y) = (&mut lo[d], &mut hi[0]);
                if ecx.posed {
                    crate::mirror::mirror_pose(ecx.skeleton, table, &x.pose, &mut y.pose, model, model2);
                    blend_in_place(&mut x.pose, &y.pose, mw, None);
                }
                let mirrored = crate::mirror::mirror_root(table.axis, x.root);
                x.root = lerp_root(x.root, mirrored, mw);
            }
        }
        NodeKind::Slot { slot, input, mask } => {
            let Some(m) = &ecx.slots[usize::from(*slot)].playing else {
                eval(ecx, s, *input, d);
                return;
            };
            let clip = clip_of(ecx.clips, &m.req.clip);
            let mw = mask.map(|k| &g.mask_weights()[usize::from(k)]);
            let at = if mw.is_some() {
                s.ensure(d + 2, joints, curves);
                eval(ecx, s, *input, d);
                d + 1
            } else {
                d
            };
            let f = &mut s.frames[at];
            if ecx.posed {
                clip.sample(m.sample_time(clip, ecx.shift), &mut f.pose, ecx.mask);
            }
            write_curves(curve_map(g, &m.req.clip), clip, m.time, &mut f.curves);
            let drive = m.req.root_motion == RootMotionMode::Drive;
            f.root = if drive { m.root } else { RootDelta::ZERO };
            f.drive = if drive { 1.0 } else { 0.0 };
            if let Some(mw) = mw {
                let rw = mw.get(usize::from(g.root_joint));
                let (lo, hi) = s.frames.split_at_mut(d + 1);
                let (x, y) = (&mut lo[d], &hi[0]);
                if ecx.posed {
                    blend_in_place(&mut x.pose, &y.pose, 1.0, Some(mw));
                }
                lerp_curves_masked(&mut x.curves, &y.curves, 1.0, Some(mw), g.curve_joints());
                x.root = lerp_root(x.root, y.root, rw);
                x.drive += (y.drive - x.drive) * rw;
            }
        }
        NodeKind::PoseInput { slot } => {
            let f = &mut s.frames[d];
            if ecx.posed {
                match ecx.poses.get(usize::from(*slot)) {
                    Some(p) if p.len() == joints => f.pose.copy_from(p),
                    _ => f.pose.copy_from(ecx.rest),
                }
            }
            f.curves.iter_mut().for_each(|c| *c = f32::NAN);
            f.root = RootDelta::ZERO;
            f.drive = 0.0;
        }
    }
}

fn write_curves(map: &[(u16, u16)], clip: &Clip, time: f32, out: &mut [f32]) {
    out.fill(f32::NAN);
    for &(local, global) in map {
        out[usize::from(global)] = clip.curve(usize::from(local), time);
    }
}

/// Wraps an angle to `[-π, π)`.
fn wrap_angle(a: f32) -> f32 {
    use std::f32::consts::{PI, TAU};
    (a + PI).rem_euclid(TAU) - PI
}
