//! Headless simulation: a compiled graph driven by a scripted parameter
//! sequence, reporting the playing clips, machine states and events. Used
//! by the validator tests; a viewer's parameter sliders do the same live.

use std::sync::Arc;

use glam::{Vec2, Vec3};
use sf_anim::{Clip, EventEdge, RootDelta, Skeleton};
use sf_anim_graph::slot::LATE_CLIP;
use sf_anim_graph::{
    BindError, Binding, CompiledGraph, GraphInstance, GraphOutput, GroundSample, IkTarget, MontageClip, MontageId, NodeKind, ParamType, Params,
    SlotError, SlotNotice, SlotRequest, UpdateInputs, WarpTarget,
};

/// An event that fired, by name.
#[derive(Clone, Debug, PartialEq)]
pub struct Fired {
    /// Simulation time, seconds.
    pub time: f32,
    /// The clip reference (`WalkForward`, `WeaponOneHand@chop`); `"<late>"`
    /// for a late-bound montage clip.
    pub clip: String,
    pub name: String,
    pub payload: String,
    pub edge: EventEdge,
    pub montage: Option<MontageId>,
}

pub struct Sim {
    pub graph: CompiledGraph,
    pub skeleton: Skeleton,
    pub clips: Vec<Arc<Clip>>,
    pub instance: GraphInstance,
    pub out: GraphOutput,
    pub time: f32,
    pub events: Vec<Fired>,
    /// Montage starts and ends, with the simulation time.
    pub notices: Vec<(f32, SlotNotice)>,
    /// The character's measured velocity, character space (what the
    /// controller feeds; `SpeedMatch` scales playback by it).
    pub velocity: Vec3,
    /// How the character moved since the previous update, in its frame then
    /// (keeps foot locks in the world); used by the next update, then cleared.
    pub moved: RootDelta,
    /// Turn rate, rad/s.
    pub turn_rate: f32,
    /// Animation LOD tier of the updates (0 = A0).
    pub lod: usize,
    /// Per graph leg: the ground under it (character space), as the
    /// controller's foot probes at `out.probe_origins` give it.
    pub ground: Vec<Option<GroundSample>>,
    ik: Vec<Option<IkTarget>>,
    warp: Vec<Option<WarpTarget>>,
}

impl Sim {
    pub fn new(graph: CompiledGraph, skeleton: Skeleton, clips: Vec<Arc<Clip>>) -> Result<Self, BindError> {
        let instance = GraphInstance::new(&Binding::new(&graph, &skeleton, &clips)?);
        let out = GraphOutput::new(&graph);
        let (ground, ik, warp) = (vec![None; graph.probe_count()], vec![None; graph.ik_targets.len()], vec![None; graph.warp_targets.len()]);
        Ok(Self { graph, skeleton, clips, instance, out, time: 0.0, events: Vec::new(), notices: Vec::new(), velocity: Vec3::ZERO, moved: RootDelta::ZERO, turn_rate: 0.0, lod: 0, ground, ik, warp })
    }

    fn binding(&self) -> Binding<'_> {
        Binding { graph: &self.graph, skeleton: &self.skeleton, clips: &self.clips }
    }

    /// Sets a float, bool (0/1) or trigger (fires when > 0.5) parameter.
    pub fn set(&mut self, name: &str, v: f32) {
        let id = self.graph.param(name).unwrap_or_else(|| panic!("no param {name:?}"));
        let ty = self.graph.params[usize::from(id.0)].ty;
        let params: &mut Params = &mut self.instance.params;
        match ty {
            ParamType::Trigger if v > 0.5 => params.fire(id),
            ParamType::Bool => params.set_bool(id, v > 0.5),
            _ => params.set_float(id, v),
        }
    }

    pub fn set_vec2(&mut self, name: &str, v: Vec2) {
        let id = self.graph.param(name).unwrap_or_else(|| panic!("no param {name:?}"));
        self.instance.params.set_vec2(id, v);
    }

    /// Runs `seconds` at `dt`, collecting events.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn run(&mut self, seconds: f32, dt: f32) {
        let steps = (seconds / dt).round().max(1.0) as usize;
        for _ in 0..steps {
            let inputs = UpdateInputs {
                velocity: self.velocity,
                moved: std::mem::take(&mut self.moved),
                turn_rate: self.turn_rate,
                ground: &self.ground,
                ik_targets: &self.ik,
                warp_targets: &self.warp,
                ..UpdateInputs::default()
            };
            let b = Binding { graph: &self.graph, skeleton: &self.skeleton, clips: &self.clips };
            self.instance.update(&b, dt, self.lod, &inputs, &mut self.out);
            self.time += dt;
            self.notices.extend(self.out.slot_notices.iter().map(|n| (self.time, *n)));
            for e in &self.out.events {
                let (clip, payload) = if e.clip == LATE_CLIP {
                    ("<late>".to_owned(), String::new())
                } else {
                    let c = usize::from(e.clip);
                    let payload = self.clips[c].events.get(e.event as usize).map(|x| x.payload.clone()).unwrap_or_default();
                    (self.graph.clips[c].reference.clone(), payload)
                };
                let name = self.graph.events[usize::from(e.id)].clone();
                self.events.push(Fired { time: self.time, clip, name, payload, edge: e.edge, montage: e.montage });
            }
        }
    }

    /// Current state name of the machine at node path `path`.
    pub fn state(&self, path: &str) -> Option<&str> {
        let node = self.graph.node(path)?;
        let i = self.instance.current_state(node)?;
        self.graph.state_names(node).get(i).copied()
    }

    /// Clips played by the graph's players in the last update, with their
    /// weights (player weight × lane weight; montages not included).
    pub fn playing(&self) -> Vec<(String, f32)> {
        let mut out: Vec<(String, f32)> = Vec::new();
        for (i, n) in self.graph.nodes.iter().enumerate() {
            let NodeKind::Player(p) = &n.kind else { continue };
            let Some((lanes, w)) = self.instance.player(u32::try_from(i).expect("node index")) else { continue };
            for (k, (_, lw)) in lanes.iter().enumerate() {
                let weight = w * if lanes.len() == 1 { 1.0 } else { *lw };
                if weight > 1e-3 {
                    let name = self.graph.clips[usize::from(p.lanes[k])].reference.clone();
                    match out.iter_mut().find(|(c, _)| *c == name) {
                        Some(e) => e.1 += weight,
                        None => out.push((name, weight)),
                    }
                }
            }
        }
        out.sort_by(|a, b| b.1.total_cmp(&a.1));
        out
    }

    /// Whether `clip` plays with at least `min` weight.
    pub fn plays(&self, clip: &str, min: f32) -> bool {
        self.playing().iter().any(|(c, w)| c == clip && *w >= min)
    }

    /// Plays a clip slot of the graph in a montage slot.
    pub fn play_slot(&mut self, slot: &str, req: SlotRequest) -> Result<MontageId, SlotError> {
        let id = self.graph.slot(slot).ok_or(SlotError::NoSlot)?;
        let (graph, skeleton, clips) = (&self.graph, &self.skeleton, &self.clips);
        self.instance.play_slot(&Binding { graph, skeleton, clips }, id, req)
    }

    /// A request for the graph clip named `reference`.
    pub fn request(&self, reference: &str) -> Option<SlotRequest> {
        crate::script::bound_group(&self.graph, reference).map(|c| SlotRequest::new(MontageClip::Bound(c)))
    }

    /// Events named `name` since `since` (simulation time).
    pub fn fired(&self, name: &str, since: f32) -> Vec<&Fired> {
        self.events.iter().filter(|e| e.name == name && e.time > since).collect()
    }

    #[doc(hidden)]
    pub fn binding_check(&self) -> bool {
        Binding::new(&self.graph, &self.skeleton, &self.clips).is_ok() && self.binding().clips.len() == self.clips.len()
    }
}
