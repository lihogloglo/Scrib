//! `compile`: a [`GraphAsset`] plus a skeleton and the clips it names →
//! a [`CompiledGraph`] (the work of the `animgraph.compile` bake step).
//!
//! - Node references are inlined depth-first: every reference becomes its
//!   own node instance, laid out so a subtree is an index range.
//! - Masks are resolved to per-joint weights on this skeleton (joint
//!   names, tags, or rig names through the adapter).
//! - Blend spaces are normalized per axis and Delaunay-triangulated.
//! - Clips are checked against the skeleton; their events and curves fill
//!   the graph-wide event and curve tables.
//! - Subgraphs are inlined; their parameters merge by name.
//! - Pose kinds (full or additive) are checked through every node.

use std::collections::HashMap;

use glam::{Vec2, Vec3};
use sf_anim::ik::FullBody;
use sf_anim::{Clip, LimbSolver, RigProfile, RootMotionMode, Skeleton};
use sf_bake::ProductKey;

use crate::asset::{
    BlendMode, ClipFallback, ClipRef, ConditionSpec, GraphAsset, JointRef, LayerMode, LayerSpec, LegSpec, NodeSpec, OpSpec, ParamKind, ProfileLimbs,
    SyncSpec, ValueSpec,
};
use crate::blendspace::triangulate;
use crate::compiled::{
    Blend, BlendSpace, BodyEffector, ClipResolution, ClipSlot, CompiledGraph, Condition, GirdleSeg, HitReactOp, Layer, Leg, Machine, MirrorTable, Node, NodeKind,
    Op, OpEntry, ParamDef, ParamType, PlayerNode, Provenance, SlotDef, SpringBonesOp, State, Transition, Value, WarpWindow,
};

/// Where `compile` gets what the asset names.
pub trait CompileSource {
    /// The cooked clip a clip reference names, and the product it came from.
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String>;
    /// Whether the clip a reference names can be had (its source files
    /// exist). A missing optional clip takes its fallback; a missing
    /// required one is an error. Hosts record the check as a dependency,
    /// so a source added later recompiles the graph.
    fn available(&mut self, _reference: &str) -> Result<bool, String> {
        Ok(true)
    }
    /// Where a resolved clip came from (license facts for exports).
    fn provenance(&mut self, _reference: &str, _product: &ProductKey) -> Provenance {
        Provenance::default()
    }
    /// Another graph asset (for `Subgraph` nodes).
    fn subgraph(&mut self, path: &str) -> Result<GraphAsset, String>;
    /// The joint name of a rig bone (`JointRef::Rig`).
    fn rig_joint(&self, _rig: &str) -> Option<String> {
        None
    }
    /// Left/right joint pairs from the adapter's rig, for `Mirror` nodes.
    fn mirror_pairs(&self) -> Vec<(String, String)> {
        Vec::new()
    }
    /// The skeleton's rig profile (for `profile_limbs`), if it has one.
    fn profile(&mut self) -> Result<Option<RigProfile>, String> {
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("graph {graph:?}: {message}")]
pub struct CompileError {
    pub graph: String,
    pub message: String,
}

type R<T> = Result<T, String>;

struct Scope {
    asset: GraphAsset,
    label: String,
}

struct Compiler<'a> {
    skeleton: &'a Skeleton,
    source: &'a mut dyn CompileSource,
    out: CompiledGraph,
    scopes: Vec<Scope>,
    subgraph_scopes: HashMap<String, usize>,
    clip_slots: HashMap<String, u16>,
    product_slots: HashMap<ProductKey, u16>,
    /// Per (scope, clip name), its resolution (`None`: dropped).
    resolved: HashMap<(usize, String), Option<u16>>,
    clips: Vec<Clip>,
    mask_cache: HashMap<(usize, String), u16>,
    stack: Vec<(usize, String)>,
    referenced_events: Vec<String>,
}

fn u16_of(i: usize, what: &str) -> R<u16> {
    u16::try_from(i).map_err(|_| format!("too many {what}"))
}

fn u32_of(i: usize) -> u32 {
    u32::try_from(i).expect("node count fits u32")
}

/// Compiles a graph asset for `skeleton`.
pub fn compile(name: &str, asset: &GraphAsset, skeleton: &Skeleton, source: &mut dyn CompileSource) -> Result<CompiledGraph, CompileError> {
    let err = |message: String| CompileError { graph: name.to_owned(), message };
    let joints = u16::try_from(skeleton.len()).map_err(|_| err("skeleton too large".into()))?;
    let root_joint = u16::try_from(skeleton.root_motion_joint().unwrap_or(0)).unwrap_or(0);
    let mut c = Compiler {
        skeleton,
        source,
        out: CompiledGraph::new(name.to_owned(), skeleton.id(), joints, root_joint),
        scopes: vec![Scope { asset: asset.clone(), label: String::new() }],
        subgraph_scopes: HashMap::new(),
        clip_slots: HashMap::new(),
        product_slots: HashMap::new(),
        resolved: HashMap::new(),
        clips: Vec::new(),
        mask_cache: HashMap::new(),
        stack: Vec::new(),
        referenced_events: Vec::new(),
    };
    c.run().map_err(err)?;
    Ok(c.out)
}

/// Parses and compiles RON graph text.
pub fn compile_str(name: &str, text: &str, skeleton: &Skeleton, source: &mut dyn CompileSource) -> Result<CompiledGraph, CompileError> {
    let asset = crate::asset::parse_graph(text).map_err(|message| CompileError { graph: name.to_owned(), message })?;
    compile(name, &asset, skeleton, source)
}

impl Compiler<'_> {
    fn run(&mut self) -> R<()> {
        let params = self.scopes[0].asset.params.clone();
        self.merge_params(&params)?;
        let warp_targets = self.scopes[0].asset.warp_targets.clone();
        for t in &warp_targets {
            Self::intern(&mut self.out.warp_targets, t, "warp targets")?;
        }
        let layers = self.scopes[0].asset.layers.clone();
        self.out.root = self.layers(0, &layers, "")?.ok_or("the base layer dropped (its optional clips are all missing): nothing to play")?;
        for name in &self.referenced_events {
            let fired = self.clips.iter().any(|c| c.events.iter().any(|e| &e.name == name));
            if !fired {
                return Err(format!("a transition waits for event {name:?}, which no clip of the graph fires"));
            }
        }
        let slot_clips = self.scopes[0].asset.slot_clips.clone();
        for name in &slot_clips {
            if let Some(clip) = self.clip_slot(0, name).map_err(|e| format!("slot clip {name:?}: {e}"))? {
                if self.clips[usize::from(clip)].meta.additive {
                    return Err(format!("slot clip {name:?} is additive; slots play full poses"));
                }
                if !self.out.slot_clips.iter().any(|(n, _)| n == name) {
                    self.out.slot_clips.push((name.clone(), clip));
                }
            }
        }
        self.legs()?;
        self.ops()?;
        Ok(())
    }

    /// The slot named `name`, declared in `group` (one group per slot).
    fn slot_def(&mut self, name: &str, group: &str) -> R<u16> {
        let gid = Self::intern(&mut self.out.slot_groups, group, "slot groups")?;
        if let Some(i) = self.out.slots.iter().position(|s| s.name == name) {
            if self.out.slots[i].group != gid {
                return Err(format!("slot {name:?} is declared in two slot groups"));
            }
            return u16_of(i, "slots");
        }
        let id = u16_of(self.out.slots.len(), "slots")?;
        self.out.slots.push(SlotDef { name: name.to_owned(), group: gid });
        Ok(id)
    }

    // --- Tables -------------------------------------------------------------------------

    fn merge_params(&mut self, params: &[crate::asset::ParamSpec]) -> R<()> {
        for p in params {
            let (ty, default) = match p.kind {
                ParamKind::Float(v) => (ParamType::Float, [v, 0.0]),
                ParamKind::Bool(b) => (ParamType::Bool, [if b { 1.0 } else { 0.0 }, 0.0]),
                ParamKind::Vec2(x, y) => (ParamType::Vec2, [x, y]),
                ParamKind::Trigger => (ParamType::Trigger, [0.0, 0.0]),
            };
            match self.out.params.iter().find(|q| q.name == p.name) {
                Some(q) if q.ty != ty => {
                    return Err(format!("parameter {:?} declared as {:?} and {ty:?}", p.name, q.ty));
                }
                Some(_) => {}
                None => self.out.params.push(ParamDef { name: p.name.clone(), ty, default }),
            }
        }
        u16_of(self.out.params.len(), "parameters")?;
        Ok(())
    }

    fn value(&self, v: &ValueSpec) -> R<Value> {
        let name = match v {
            ValueSpec::Const(c) => return Ok(Value::Const(*c)),
            ValueSpec::Param(n) => n,
        };
        let find = |n: &str| self.out.params.iter().position(|p| p.name == n);
        if let Some(i) = find(name) {
            if self.out.params[i].ty == ParamType::Vec2 {
                return Err(format!("parameter {name:?} is a Vec2: read {name}.x or {name}.y"));
            }
            return Ok(Value::Param(u16_of(i, "parameters")?, 0));
        }
        if let Some((base, comp)) = name.rsplit_once('.') {
            let comp = match comp {
                "x" => 0,
                "y" => 1,
                _ => return Err(format!("unknown parameter {name:?}")),
            };
            if let Some(i) = find(base).filter(|&i| self.out.params[i].ty == ParamType::Vec2) {
                return Ok(Value::Param(u16_of(i, "parameters")?, comp));
            }
        }
        Err(format!("unknown parameter {name:?}"))
    }

    fn param_of_type(&self, name: &str, types: &[ParamType]) -> R<u16> {
        let i = self.out.params.iter().position(|p| p.name == name).ok_or_else(|| format!("unknown parameter {name:?}"))?;
        if !types.contains(&self.out.params[i].ty) {
            return Err(format!("parameter {name:?} is a {:?}, expected {types:?}", self.out.params[i].ty));
        }
        u16_of(i, "parameters")
    }

    fn intern(table: &mut Vec<String>, name: &str, what: &str) -> R<u16> {
        if let Some(i) = table.iter().position(|n| n == name) {
            return u16_of(i, what);
        }
        table.push(name.to_owned());
        u16_of(table.len() - 1, what)
    }

    /// Resolves the clip name `local` of `scope` to a slot, following the
    /// fallbacks of missing optional clips; `None` when it dropped. The
    /// resolution is recorded in the compiled graph.
    fn clip_slot(&mut self, scope: usize, local: &str) -> R<Option<u16>> {
        let key = (scope, local.to_owned());
        if let Some(&slot) = self.resolved.get(&key) {
            return Ok(slot);
        }
        let mut name = local.to_owned();
        let mut tried: Vec<String> = Vec::new();
        let mut missing = Vec::new();
        let slot = loop {
            if tried.contains(&name) {
                return Err(format!("clip {local:?}: fallback cycle through {name:?}"));
            }
            // A name the graph doesn't declare is a raw, required reference.
            let entry = self.scopes[scope].asset.clips.get(&name).cloned().unwrap_or_else(|| ClipRef::required(name.clone()));
            tried.push(name.clone());
            let source = &entry.source;
            // Required clips load directly: a missing one is an error.
            if !entry.optional {
                let via = if tried.len() > 1 { format!(" (required fallback of {local:?})") } else { String::new() };
                break Some(self.load_slot(&name, source).map_err(|e| format!("{e}{via}"))?);
            }
            if self.source.available(source).map_err(|e| format!("clip {name:?} ({source}): {e}"))? {
                break Some(self.load_slot(&name, source)?);
            }
            missing.push(source.clone());
            match entry.fallback {
                Some(ClipFallback::Clip(next)) => name = next,
                Some(ClipFallback::Drop) | None => break None,
            }
        };
        let label = &self.scopes[scope].label;
        let name = if label.is_empty() { local.to_owned() } else { format!("{label}:{local}") };
        self.out.resolutions.push(ClipResolution { name, missing, slot });
        self.resolved.insert(key, slot);
        Ok(slot)
    }

    /// The slot of a present clip reference (one per product).
    fn load_slot(&mut self, local: &str, reference: &str) -> R<u16> {
        if let Some(&s) = self.clip_slots.get(reference) {
            return Ok(s);
        }
        let reference = reference.to_owned();
        let (product, clip) = self.source.clip(&reference).map_err(|e| format!("clip {local:?} ({reference}): {e}"))?;
        if let Some(&s) = self.product_slots.get(&product) {
            self.clip_slots.insert(reference, s);
            return Ok(s);
        }
        if clip.skeleton != self.skeleton.id() || clip.joint_count() != self.skeleton.len() {
            return Err(format!("clip {local:?} ({reference}) is bound to another skeleton"));
        }
        let slot = u16_of(self.clips.len(), "clips")?;
        let events = clip.events.iter().map(|e| Self::intern(&mut self.out.events, &e.name, "events")).collect::<R<Vec<_>>>()?;
        let mut curves = Vec::with_capacity(clip.curves.len());
        for (i, cv) in clip.curves.iter().enumerate() {
            curves.push((u16_of(i, "curves")?, Self::intern(&mut self.out.curves, &cv.name, "curves")?));
        }
        let windows = clip
            .events
            .iter()
            .filter(|e| e.name == "warp")
            .map(|e| self.warp_window(&clip, e).map_err(|m| format!("clip {local:?}: warp window at {}: {m}", e.time)))
            .collect::<R<Vec<_>>>()?;
        self.out.warp_windows.push(windows);
        self.out.clip_events.push(events);
        self.out.clip_curves.push(curves);
        let provenance = self.source.provenance(&reference, &product);
        self.out.clips.push(ClipSlot {
            reference: reference.clone(),
            product: product.clone(),
            duration: clip.duration,
            additive: clip.meta.additive,
            provenance,
        });
        self.clips.push(clip);
        self.clip_slots.insert(reference, slot);
        self.product_slots.insert(product, slot);
        Ok(slot)
    }

    fn warp_window(&mut self, clip: &Clip, e: &sf_anim::Event) -> R<WarpWindow> {
        let table = &mut self.out.warp_targets;
        crate::compiled::parse_warp_window(self.skeleton, clip, e, &mut |name| Self::intern(table, name, "warp targets"))
    }

    fn sync(&mut self, s: Option<&SyncSpec>) -> R<Option<(u16, crate::asset::SyncRole)>> {
        s.map(|s| Ok((Self::intern(&mut self.out.sync_groups, &s.group, "sync groups")?, s.role))).transpose()
    }

    // --- Joints and masks ---------------------------------------------------------------

    fn joints_of(&self, r: &JointRef) -> R<Vec<usize>> {
        let s = self.skeleton;
        match r {
            JointRef::Joint(n) => s.find(n).map(|j| vec![j]).ok_or_else(|| format!("no joint {n:?}")),
            JointRef::Tag(t) => {
                let v: Vec<usize> = s.joints_with_tag(t).collect();
                if v.is_empty() { Err(format!("no joint tagged {t:?}")) } else { Ok(v) }
            }
            JointRef::Rig(r) => {
                let name = self.source.rig_joint(r).ok_or_else(|| format!("the rig maps no joint to {r:?}"))?;
                s.find(&name).map(|j| vec![j]).ok_or_else(|| format!("rig bone {r:?} maps to missing joint {name:?}"))
            }
        }
    }

    fn joint(&self, r: &JointRef) -> R<u16> {
        let v = self.joints_of(r)?;
        if v.len() != 1 {
            return Err(format!("{r:?} names {} joints, expected one", v.len()));
        }
        u16_of(v[0], "joints")
    }

    fn mask(&mut self, scope: usize, name: &str) -> R<u16> {
        let key = (scope, name.to_owned());
        if let Some(&m) = self.mask_cache.get(&key) {
            return Ok(m);
        }
        let spec = self.scopes[scope].asset.masks.get(name).cloned().ok_or_else(|| format!("unknown mask {name:?}"))?;
        let mut w = vec![spec.default; self.skeleton.len()];
        for rule in &spec.rules {
            for j in self.joints_of(&rule.joint)? {
                if rule.subtree {
                    for k in self.skeleton.subtree(j) {
                        w[k] = rule.weight;
                    }
                } else {
                    w[j] = rule.weight;
                }
            }
        }
        for x in &mut w {
            *x = x.clamp(0.0, 1.0);
        }
        let idx = if let Some(i) = self.out.masks.iter().position(|m| *m == w) {
            u16_of(i, "masks")?
        } else {
            self.out.masks.push(w);
            u16_of(self.out.masks.len() - 1, "masks")?
        };
        self.mask_cache.insert(key, idx);
        Ok(idx)
    }

    fn mirror_table(&mut self) -> R<()> {
        if self.out.mirror.is_some() {
            return Ok(());
        }
        let spec = self.scopes[0].asset.mirror.clone();
        let n = self.skeleton.len();
        let mut pairs: Vec<u16> = (0..n).map(|j| u16_of(j, "joints")).collect::<R<_>>()?;
        let mut all = spec.pairs.clone();
        all.extend(self.source.mirror_pairs());
        if all.is_empty() {
            return Err("a Mirror node needs mirror pairs (graph `mirror` or the adapter's rig)".into());
        }
        for (a, b) in &all {
            let (ja, jb) = (self.joint(&JointRef::Joint(a.clone()))?, self.joint(&JointRef::Joint(b.clone()))?);
            pairs[usize::from(ja)] = jb;
            pairs[usize::from(jb)] = ja;
        }
        let corrections = crate::mirror::corrections(self.skeleton, &pairs, spec.axis);
        self.out.mirror = Some(MirrorTable { axis: spec.axis, pairs, corrections });
        Ok(())
    }

    // --- Nodes --------------------------------------------------------------------------

    /// A pass-through to `input` (a node whose other inputs dropped, or a
    /// subgraph's root).
    fn pass(&self, input: u32) -> (NodeKind, bool) {
        (NodeKind::RateScale { input, rate: Value::Const(1.0) }, self.kind_of(input))
    }

    fn kind_of(&self, node: u32) -> bool {
        self.out.nodes[node as usize].additive
    }

    /// The `Layers` node of a layer list; `None` when the base dropped.
    fn layers(&mut self, scope: usize, layers: &[LayerSpec], path: &str) -> R<Option<u32>> {
        let (first, rest) = layers.split_first().ok_or("no layers")?;
        if first.mode != LayerMode::Override || first.mask.is_some() || first.weight != ValueSpec::Const(1.0) {
            return Err("the first layer is the base: Override, no mask, weight 1".into());
        }
        let idx = self.placeholder(path);
        let Some(base) = self.node(scope, &first.node, path, None)? else {
            self.out.nodes.truncate(idx as usize);
            return Ok(None);
        };
        let compiled = self.layer_list(scope, rest, path, self.kind_of(base))?;
        self.finish(idx, NodeKind::Layers { base, layers: compiled }, self.kind_of(base));
        Ok(Some(idx))
    }

    /// Layers over a base; a layer whose node dropped is left out.
    fn layer_list(&mut self, scope: usize, layers: &[LayerSpec], path: &str, base_additive: bool) -> R<Vec<Layer>> {
        let mut out = Vec::with_capacity(layers.len());
        for l in layers {
            let Some(node) = self.node(scope, &l.node, path, None)? else { continue };
            let additive = self.kind_of(node);
            match l.mode {
                LayerMode::Override if additive != base_additive => {
                    return Err(format!("layer {:?} overrides a {} pose with a {} one", l.node, kind_name(base_additive), kind_name(additive)));
                }
                LayerMode::Additive if !additive => {
                    return Err(format!("additive layer {:?} doesn't output an additive pose", l.node));
                }
                _ => {}
            }
            let mask = l.mask.as_deref().map(|m| self.mask(scope, m)).transpose()?;
            out.push(Layer { node, mode: l.mode, mask, weight: self.value(&l.weight)? });
        }
        Ok(out)
    }

    fn placeholder(&mut self, path: &str) -> u32 {
        let idx = u32_of(self.out.nodes.len());
        self.out.nodes.push(Node { name: path.to_owned(), kind: NodeKind::PoseInput { slot: 0 }, subtree_end: 0, additive: false });
        idx
    }

    fn finish(&mut self, idx: u32, kind: NodeKind, additive: bool) {
        let end = u32_of(self.out.nodes.len());
        let n = &mut self.out.nodes[idx as usize];
        n.kind = kind;
        n.subtree_end = end;
        n.additive = additive;
    }

    /// Compiles a node reference; `None` when it dropped (animation.md
    /// "Optional clips"): a node left with nothing to play is removed with
    /// its subtree, and a node that still has one input becomes a
    /// pass-through to it.
    #[allow(clippy::too_many_lines)]
    fn node(&mut self, scope: usize, name: &str, parent: &str, inherited: Option<RootMotionMode>) -> R<Option<u32>> {
        let key = (scope, name.to_owned());
        if self.stack.contains(&key) {
            return Err(format!("node {name:?} contains itself"));
        }
        let spec = self.scopes[scope].asset.nodes.get(name).cloned().ok_or_else(|| format!("unknown node {name:?}"))?;
        let label = &self.scopes[scope].label;
        let path = match (parent.is_empty(), label.is_empty()) {
            (true, true) => name.to_owned(),
            (true, false) => format!("{label}:{name}"),
            (false, _) => format!("{parent}/{name}"),
        };
        self.stack.push(key);
        let idx = self.placeholder(&path);
        let at = |e: String| format!("{path}: {e}");
        let compiled = match &spec {
            NodeSpec::Clip { clip, rate, looping, start, sync, root_motion } => 'clip: {
                let Some(slot) = self.clip_slot(scope, clip).map_err(at)? else { break 'clip None };
                let c = &self.clips[usize::from(slot)];
                let looping = looping.unwrap_or(c.loop_range.is_some());
                let mode = root_motion.or(inherited).unwrap_or(c.meta.root_motion_mode);
                let additive = c.meta.additive;
                let player = PlayerNode {
                    lanes: vec![slot],
                    lane_rates: vec![1.0],
                    rate: self.value(rate).map_err(at)?,
                    looping,
                    start: start.clamp(0.0, 1.0),
                    sync: self.sync(sync.as_ref()).map_err(at)?,
                    root_motion: mode,
                    space: None,
                };
                Some((NodeKind::Player(player), additive))
            }
            NodeSpec::BlendSpace1D { x, samples, rate, smoothing, looping, sync, root_motion } => {
                let pts: Vec<(String, Vec2, f32)> = samples.iter().map(|s| (s.clip.clone(), Vec2::new(s.at, 0.0), s.rate)).collect();
                let space = (self.value(x).map_err(at)?, None, *smoothing);
                self.blend_space(scope, &pts, space, rate, *looping, sync.as_ref(), root_motion.or(inherited)).map_err(at)?
            }
            NodeSpec::BlendSpace2D { x, y, samples, rate, smoothing, looping, sync, root_motion } => {
                let pts: Vec<(String, Vec2, f32)> = samples.iter().map(|s| (s.clip.clone(), Vec2::new(s.at.0, s.at.1), s.rate)).collect();
                let space = (self.value(x).map_err(at)?, Some(self.value(y).map_err(at)?), *smoothing);
                self.blend_space(scope, &pts, space, rate, *looping, sync.as_ref(), root_motion.or(inherited)).map_err(at)?
            }
            NodeSpec::Blend { a, b, alpha, smoothing, mask } => {
                match (self.node(scope, a, &path, inherited)?, self.node(scope, b, &path, inherited)?) {
                    (Some(na), Some(nb)) => {
                        if self.kind_of(na) != self.kind_of(nb) {
                            return Err(at("blends a full pose with an additive one".into()));
                        }
                        let mask = mask.as_deref().map(|m| self.mask(scope, m)).transpose().map_err(at)?;
                        let kind = NodeKind::Blend { a: na, b: nb, alpha: self.value(alpha).map_err(at)?, smoothing: *smoothing, mask };
                        Some((kind, self.kind_of(na)))
                    }
                    // One side dropped: the other plays alone.
                    (Some(n), None) | (None, Some(n)) => Some(self.pass(n)),
                    (None, None) => None,
                }
            }
            NodeSpec::Additive { base, additive, weight, mask } => {
                match (self.node(scope, base, &path, inherited)?, self.node(scope, additive, &path, inherited)?) {
                    (Some(nb), Some(na)) => {
                        if self.kind_of(nb) || !self.kind_of(na) {
                            return Err(at("needs a full base pose and an additive pose".into()));
                        }
                        let mask = mask.as_deref().map(|m| self.mask(scope, m)).transpose().map_err(at)?;
                        Some((NodeKind::Additive { base: nb, additive: na, weight: self.value(weight).map_err(at)?, mask }, false))
                    }
                    (Some(nb), None) => Some(self.pass(nb)),
                    (None, _) => None,
                }
            }
            NodeSpec::LayeredPerBone { base, layers } => match self.node(scope, base, &path, inherited)? {
                Some(nb) => {
                    let additive = self.kind_of(nb);
                    let layers = self.layer_list(scope, layers, &path, additive).map_err(at)?;
                    Some((NodeKind::Layers { base: nb, layers }, additive))
                }
                None => None,
            },
            NodeSpec::StateMachine { initial, states, transitions } => 'machine: {
                if states.is_empty() {
                    return Err(at("a state machine needs states".into()));
                }
                let mut compiled = Vec::with_capacity(states.len());
                let mut dropped: Vec<&str> = Vec::new();
                let mut additive = None;
                for s in states {
                    if compiled.iter().any(|c: &State| c.name == s.name) || dropped.contains(&s.name.as_str()) {
                        return Err(at(format!("two states named {:?}", s.name)));
                    }
                    let Some(n) = self.node(scope, &s.node, &path, s.root_motion.or(inherited))? else {
                        dropped.push(&s.name);
                        continue;
                    };
                    if *additive.get_or_insert(self.kind_of(n)) != self.kind_of(n) {
                        return Err(at("states mix full and additive poses".into()));
                    }
                    compiled.push(State { name: s.name.clone(), node: n });
                }
                if compiled.is_empty() {
                    break 'machine None;
                }
                let state_index = |n: &str| {
                    compiled.iter().position(|s| s.name == n).map(|i| u16::try_from(i).unwrap_or(0)).ok_or_else(|| {
                        if dropped.contains(&n) {
                            at(format!("state {n:?} dropped (its optional clips are missing)"))
                        } else {
                            at(format!("unknown state {n:?}"))
                        }
                    })
                };
                let initial = match initial {
                    Some(n) => state_index(n)?,
                    None if dropped.contains(&states[0].name.as_str()) => {
                        return Err(at("the initial state dropped (its optional clips are missing)".into()));
                    }
                    None => 0,
                };
                let mut ts = Vec::with_capacity(transitions.len());
                for t in transitions {
                    // Transitions into or out of a dropped state go with it.
                    if dropped.contains(&t.from.as_str()) || dropped.contains(&t.to.as_str()) {
                        continue;
                    }
                    let from = if t.from == "*" { None } else { Some(state_index(&t.from)?) };
                    let to = state_index(&t.to)?;
                    let when = t.when.iter().map(|c| self.condition(c)).collect::<R<Vec<_>>>().map_err(at)?;
                    let blend = match t.blend {
                        BlendMode::Inertialize(d) => Blend::Inertialize(d.max(0.0)),
                        BlendMode::Crossfade(d) => Blend::Crossfade(d.max(0.0)),
                        BlendMode::Cut => Blend::Cut,
                    };
                    ts.push(Transition { from, to, when, blend, interruptible: t.interruptible });
                }
                Some((NodeKind::StateMachine(Machine { states: compiled, initial, transitions: ts }), additive.unwrap_or(false)))
            }
            NodeSpec::RateScale { input, rate } => match self.node(scope, input, &path, inherited)? {
                Some(n) => Some((NodeKind::RateScale { input: n, rate: self.value(rate).map_err(at)? }, self.kind_of(n))),
                None => None,
            },
            NodeSpec::Mirror { input, weight } => match self.node(scope, input, &path, inherited)? {
                Some(n) => {
                    if self.kind_of(n) {
                        return Err(at("can't mirror an additive pose".into()));
                    }
                    self.mirror_table().map_err(at)?;
                    Some((NodeKind::Mirror { input: n, weight: self.value(weight).map_err(at)? }, false))
                }
                None => None,
            },
            NodeSpec::PoseInput { slot } => {
                Some((NodeKind::PoseInput { slot: Self::intern(&mut self.out.pose_inputs, slot, "pose inputs").map_err(at)? }, false))
            }
            NodeSpec::Slot { slot, input, group, mask } => match self.node(scope, input, &path, inherited)? {
                Some(n) => {
                    if self.kind_of(n) {
                        return Err(at("a slot plays full poses; its input is additive".into()));
                    }
                    let slot = self.slot_def(slot, group.as_deref().unwrap_or("default")).map_err(at)?;
                    let mask = mask.as_deref().map(|m| self.mask(scope, m)).transpose().map_err(at)?;
                    Some((NodeKind::Slot { slot, input: n, mask }, false))
                }
                None => None,
            },
            NodeSpec::Subgraph { graph } => {
                let sub = self.subgraph(graph).map_err(at)?;
                let layers = self.scopes[sub].asset.layers.clone();
                // The subgraph's root replaces the placeholder, which becomes
                // a pass-through.
                self.layers(sub, &layers, &path).map_err(at)?.map(|inner| self.pass(inner))
            }
        };
        self.stack.pop();
        if let Some((kind, additive)) = compiled {
            self.finish(idx, kind, additive);
            Ok(Some(idx))
        } else {
            // Depth-first layout: the dropped subtree is the tail.
            self.out.nodes.truncate(idx as usize);
            Ok(None)
        }
    }

    fn subgraph(&mut self, path: &str) -> R<usize> {
        if let Some(&s) = self.subgraph_scopes.get(path) {
            return Ok(s);
        }
        if self.scopes.len() > 64 {
            return Err("subgraphs nest too deep".into());
        }
        let asset = self.source.subgraph(path).map_err(|e| format!("subgraph {path:?}: {e}"))?;
        self.merge_params(&asset.params)?;
        self.scopes.push(Scope { asset, label: path.to_owned() });
        let s = self.scopes.len() - 1;
        self.subgraph_scopes.insert(path.to_owned(), s);
        Ok(s)
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    fn blend_space(
        &mut self,
        scope: usize,
        samples: &[(String, Vec2, f32)],
        (x, y, smoothing): (Value, Option<Value>, f32),
        rate: &ValueSpec,
        looping: Option<bool>,
        sync: Option<&SyncSpec>,
        mode: Option<RootMotionMode>,
    ) -> R<Option<(NodeKind, bool)>> {
        if samples.is_empty() {
            return Err("a blend space needs samples".into());
        }
        // Dropped samples leave the space; the rest are re-normalized and
        // re-triangulated. With none left, the space drops.
        let mut lanes = Vec::with_capacity(samples.len());
        let mut kept = Vec::with_capacity(samples.len());
        for s in samples {
            if let Some(lane) = self.clip_slot(scope, &s.0)? {
                lanes.push(lane);
                kept.push(s.clone());
            }
        }
        if lanes.is_empty() {
            return Ok(None);
        }
        let samples = &kept[..];
        let clips: Vec<&Clip> = lanes.iter().map(|&s| &self.clips[usize::from(s)]).collect();
        let additive = clips[0].meta.additive;
        if clips.iter().any(|c| c.meta.additive != additive) {
            return Err("blend-space samples mix full and additive clips".into());
        }
        let looping = looping.unwrap_or(clips.iter().all(|c| c.loop_range.is_some()));
        let mode = mode.unwrap_or(clips[0].meta.root_motion_mode);
        let raw: Vec<Vec2> = samples.iter().map(|s| s.1).collect();
        let min = raw.iter().copied().fold(Vec2::splat(f32::INFINITY), Vec2::min);
        let mut max = raw.iter().copied().fold(Vec2::splat(f32::NEG_INFINITY), Vec2::max);
        if max.x - min.x < 1e-6 {
            max.x = min.x + 1.0;
        }
        if max.y - min.y < 1e-6 {
            max.y = min.y + 1.0;
        }
        let points: Vec<Vec2> = raw.iter().map(|p| (*p - min) / (max - min)).collect();
        let points: Vec<Vec2> = if y.is_some() { points } else { points.into_iter().map(|p| Vec2::new(p.x, 0.0)).collect() };
        let (triangles, edges) = triangulate(&points)?;
        let space = BlendSpace { x, y, min, max, smoothing, points, triangles, edges };
        let player = PlayerNode {
            lanes,
            lane_rates: samples.iter().map(|s| s.2.max(1e-3)).collect(),
            rate: self.value(rate)?,
            looping,
            start: 0.0,
            sync: self.sync(sync)?,
            root_motion: mode,
            space: Some(space),
        };
        Ok(Some((NodeKind::Player(player), additive)))
    }

    fn condition(&mut self, c: &ConditionSpec) -> R<Condition> {
        Ok(match c {
            ConditionSpec::IsTrue(p) => Condition::IsTrue(self.value(&ValueSpec::Param(p.clone()))?),
            ConditionSpec::IsFalse(p) => Condition::IsFalse(self.value(&ValueSpec::Param(p.clone()))?),
            ConditionSpec::Greater(p, x) => Condition::Greater(self.value(&ValueSpec::Param(p.clone()))?, *x),
            ConditionSpec::Less(p, x) => Condition::Less(self.value(&ValueSpec::Param(p.clone()))?, *x),
            ConditionSpec::Trigger(p) => Condition::Trigger(self.param_of_type(p, &[ParamType::Trigger])?),
            ConditionSpec::Event(e) => {
                self.referenced_events.push(e.clone());
                Condition::Event(Self::intern(&mut self.out.events, e, "events")?)
            }
            ConditionSpec::PhaseAtLeast(x) => Condition::PhaseAtLeast(*x),
            ConditionSpec::TimeLeft(x) => Condition::TimeLeft(*x),
            ConditionSpec::Finished => Condition::Finished,
            ConditionSpec::StateTime(x) => Condition::StateTime(*x),
        })
    }

    // --- Limbs and ops ------------------------------------------------------------------

    fn contact_curve(&self, name: &str, curve: Option<&String>, required: bool) -> R<Option<u16>> {
        match curve {
            Some(c) => match self.out.curve(c) {
                Some(i) => Ok(Some(u16_of(i, "curves")?)),
                None if required => Err(format!("limb {name:?}: no clip has curve {c:?}")),
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    /// The heel-roll curve (`roll.<effector>`) beside a contact curve
    /// (`contact.<effector>`); None when the clips cooked none.
    fn roll_curve(&self, contact: Option<&String>) -> R<Option<u16>> {
        let Some(name) = contact.and_then(|c| c.strip_prefix("contact.")).map(|e| format!("roll.{e}")) else { return Ok(None) };
        self.out.curve(&name).map(|i| u16_of(i, "curves")).transpose()
    }

    /// A limb from a spec, checked: a parent-to-child chain of 2–8 joints.
    fn limb(&self, l: &LegSpec) -> R<Leg> {
        let refs: Vec<&JointRef> = if l.joints.is_empty() {
            [&l.hip, &l.knee, &l.foot]
                .into_iter()
                .map(|r| r.as_ref().ok_or_else(|| format!("limb {:?}: give `joints` or hip, knee and foot", l.name)))
                .collect::<R<Vec<_>>>()?
        } else {
            l.joints.iter().collect()
        };
        let joints = refs.into_iter().map(|r| self.joint(r)).collect::<R<Vec<u16>>>()?;
        let s = self.skeleton;
        if !(2..=8).contains(&joints.len()) || joints.windows(2).any(|w| w[0] == w[1] || !s.is_ancestor(usize::from(w[0]), usize::from(w[1]))) {
            return Err(format!("limb {:?}: joints must be a parent-to-child chain of 2 to 8 joints", l.name));
        }
        let first = usize::from(joints[0]);
        let girdle = match &l.girdle {
            Some(g) => self.joint(g)?,
            None => u16_of(s.parent(first).unwrap_or(first), "joints")?,
        };
        let rest = s.rest_model();
        let girdle_rot = sf_anim::Trs::from_affine(&rest[usize::from(girdle)]).rotation;
        let solver = l.solver.unwrap_or_else(|| LimbSolver::for_len(joints.len()));
        let pole = match l.pole {
            Some((x, y, z)) => Some(girdle_rot.inverse() * Vec3::new(x, y, z).normalize_or(Vec3::Z)),
            None => rest_bend(s, &joints, solver).map(|b| girdle_rot.inverse() * b),
        };
        // Graph-defined limbs name no foot: no tip probe.
        let toe = None;
        let sole = rest[usize::from(last_of(&joints))].translation.y;
        Ok(Leg {
            name: l.name.clone(),
            joints,
            solver,
            girdle,
            pole,
            support: l.support,
            contact: self.contact_curve(&l.name, l.contact.as_ref(), true)?,
            roll: self.roll_curve(l.contact.as_ref())?,
            toe,
            sole,
        })
    }

    fn legs(&mut self) -> R<()> {
        let specs = self.scopes[0].asset.legs.clone();
        for l in &specs {
            if self.out.legs.iter().any(|x| x.name == l.name) {
                return Err(format!("two limbs named {:?}", l.name));
            }
            let leg = self.limb(l)?;
            self.out.legs.push(leg);
        }
        let which = self.scopes[0].asset.profile_limbs;
        if which != ProfileLimbs::None {
            let profile = self.source.profile()?.ok_or("profile_limbs needs the skeleton's rig profile")?;
            profile.validate(self.skeleton).map_err(|e| e.to_string())?;
            // Rigid heel-to-toe feet (tip probes): bipeds; quadruped paws stay point contacts.
            let one_girdle = profile.support_limbs().map(|l| l.girdle).collect::<std::collections::BTreeSet<_>>().len() == 1;
            for pl in profile.limbs.iter().filter(|l| l.support || which == ProfileLimbs::All) {
                if self.out.legs.iter().any(|x| x.name == pl.name) {
                    continue; // the graph's own definition wins
                }
                let contact = pl.foot.as_ref().map(|f| f.contact.clone());
                let pole = match pl.pole {
                    sf_anim::PoleHint::Axis(a) => Some(a),
                    sf_anim::PoleHint::Animated => None,
                };
                self.out.legs.push(Leg {
                    name: pl.name.clone(),
                    joints: pl.joints.clone(),
                    solver: pl.solver,
                    girdle: profile.girdle_joint(pl),
                    pole,
                    support: pl.support,
                    contact: self.contact_curve(&pl.name, contact.as_ref(), false)?,
                    roll: self.roll_curve(contact.as_ref())?,
                    toe: if one_girdle && pl.support && pl.solver == LimbSolver::TwoBone && pl.kind == sf_anim::LimbKind::Leg(sf_anim::LegKind::Plantigrade) { tip_of(self.skeleton, &self.skeleton.rest_model(), last_of(&pl.joints), pl.foot.as_ref().and_then(|f| f.toe)) } else { None },
                    sole: self.skeleton.rest_model()[usize::from(last_of(&pl.joints))].translation.y,
                });
            }
        }
        // Toe probe slots follow the leg probes.
        let mut slot = self.out.legs.len();
        for leg in &mut self.out.legs {
            if let Some((_, s, _)) = &mut leg.toe {
                *s = u16_of(slot, "probes")?;
                slot += 1;
            }
        }
        Ok(())
    }

    /// Limb indices by name; empty names: every support limb.
    fn leg_list(&self, names: &[String]) -> R<Vec<u16>> {
        if names.is_empty() {
            return (0..self.out.legs.len()).filter(|&i| self.out.legs[i].support).map(|i| u16_of(i, "legs")).collect();
        }
        names.iter().map(|n| u16_of(self.out.leg(n).ok_or_else(|| format!("unknown limb {n:?}"))?, "legs")).collect()
    }

    /// The girdles of `legs`, root-most first, on one chain; the root
    /// girdle is moved through `pelvis`.
    fn girdles(&self, pelvis: u16, legs: &[u16]) -> R<Vec<GirdleSeg>> {
        let s = self.skeleton;
        let depth = |mut j: usize| {
            let mut d = 0;
            while let Some(p) = s.parent(j) {
                d += 1;
                j = p;
            }
            d
        };
        let mut joints: Vec<u16> = legs.iter().map(|&l| self.out.legs[usize::from(l)].girdle).collect();
        joints.sort_by_key(|&g| (depth(usize::from(g)), g));
        joints.dedup();
        for w in joints.windows(2) {
            if !s.is_ancestor(usize::from(w[0]), usize::from(w[1])) {
                return Err(format!("girdles {:?} and {:?} are not on one chain", s.name(usize::from(w[0])), s.name(usize::from(w[1]))));
            }
        }
        let mut out = Vec::with_capacity(joints.len());
        for (k, &g) in joints.iter().enumerate() {
            let legs_here =
                (0..legs.len()).filter(|&i| self.out.legs[usize::from(legs[i])].girdle == g).map(|i| u16_of(i, "legs")).collect::<R<Vec<_>>>()?;
            let spine = if k == 0 {
                Vec::new()
            } else {
                // From the previous girdle down to (not including) this one.
                let mut chain = Vec::new();
                let mut j = s.parent(usize::from(g));
                while let Some(p) = j {
                    chain.push(u16_of(p, "joints")?);
                    if p == usize::from(joints[k - 1]) {
                        break;
                    }
                    j = s.parent(p);
                }
                chain.reverse();
                chain
            };
            out.push(GirdleSeg { joint: if k == 0 { pelvis } else { g }, spine, legs: legs_here });
        }
        Ok(out)
    }

    fn ik_target(&mut self, name: &str) -> R<u16> {
        Self::intern(&mut self.out.ik_targets, name, "IK targets")
    }

    #[allow(clippy::too_many_lines)]
    fn ops(&mut self) -> R<()> {
        let specs = self.scopes[0].asset.procedural.clone();
        let weights = |c: &Self, v: &[(JointRef, f32)]| v.iter().map(|(j, w)| Ok((c.joint(j)?, *w))).collect::<R<Vec<_>>>();
        for spec in &specs {
            let (op, max_lod) = match spec {
                OpSpec::SpeedMatch { min, max, rate_share, max_lod } => (Op::SpeedMatch { min: *min, max: *max, rate_share: rate_share.clamp(0.0, 1.0) }, *max_lod),
                OpSpec::StrideWarp { pelvis, legs, min, max, grade, pelvis_halflife, hold, max_drop, blend_halflife, gate, max_lod } => {
                    let (pelvis, legs) = (self.joint(pelvis)?, self.leg_list(legs)?);
                    let (girdles, pelvis_halflife, hold, max_drop) = (self.girdles(pelvis, &legs)?, *pelvis_halflife, *hold, *max_drop);
                    let gate = gate.as_deref().map(|n| self.param_of_type(n, &[ParamType::Bool, ParamType::Float])).transpose()?;
                    let blend_halflife = blend_halflife.max(0.0);
                    (Op::StrideWarp { pelvis, girdles, legs, min: *min, max: *max, grade: *grade, pelvis_halflife, hold, max_drop, blend_halflife, gate }, *max_lod)
                }
                OpSpec::OrientationWarp { pelvis, spine, max_angle, halflife, max_lod } => {
                    let spine = weights(self, spine)?;
                    (Op::OrientationWarp { pelvis: self.joint(pelvis)?, spine, max_angle: *max_angle, halflife: *halflife }, *max_lod)
                }
                OpSpec::FootPlacement { pelvis, legs, max_foot_angle, release, pelvis_halflife, body, roll, guards, gate, max_lod } => {
                    let (pelvis, legs) = (self.joint(pelvis)?, self.leg_list(legs)?);
                    let gate = gate.as_deref().map(|n| self.param_of_type(n, &[ParamType::Bool, ParamType::Float])).transpose()?;
                    (
                        Op::FootPlacement {
                            girdles: self.girdles(pelvis, &legs)?,
                            legs,
                            max_foot_angle: *max_foot_angle,
                            release: *release,
                            pelvis_halflife: *pelvis_halflife,
                            body: *body,
                            roll: roll.clamp(0.0, 1.0),
                            guards: *guards,
                            gate,
                        },
                        *max_lod,
                    )
                }
                OpSpec::LookAt { chain, forward, total_limit, halflife, max_lod } => {
                    let chain = chain.iter().map(|(j, w, l)| Ok((self.joint(j)?, *w, *l))).collect::<R<Vec<_>>>()?;
                    if chain.is_empty() {
                        return Err("LookAt needs a chain".into());
                    }
                    let forward = Vec3::new(forward.0, forward.1, forward.2).normalize_or(Vec3::Z);
                    (Op::LookAt { chain, forward, total_limit: *total_limit, halflife: *halflife }, *max_lod)
                }
                OpSpec::SpineBend { spine, gain, max_angle, halflife, max_lod } => {
                    (Op::SpineBend { spine: weights(self, spine)?, gain: *gain, max_angle: *max_angle, halflife: *halflife }, *max_lod)
                }
                OpSpec::Lean { joints, gain, max_angle, grade, halflife, max_lod } => {
                    (Op::Lean { joints: weights(self, joints)?, gain: *gain, max_angle: *max_angle, grade: *grade, halflife: *halflife }, *max_lod)
                }
                OpSpec::LimbIk { limb, target, weight, rotation, max_lod } => {
                    let leg = self.out.legs[self.out.leg(limb).ok_or_else(|| format!("unknown limb {limb:?}"))?].clone();
                    (Op::LimbIk { limb: leg, target: self.ik_target(target)?, weight: self.value(weight)?, rotation: *rotation }, *max_lod)
                }
                OpSpec::HandIk { upper, lower, hand, target, weight, max_lod } => {
                    let spec = LegSpec {
                        name: "hand".into(),
                        hip: None,
                        knee: None,
                        foot: None,
                        joints: vec![upper.clone(), lower.clone(), hand.clone()],
                        solver: Some(LimbSolver::TwoBone),
                        girdle: None,
                        pole: Some((0.0, 0.0, -1.0)),
                        support: false,
                        contact: None,
                    };
                    let leg = self.limb(&spec)?;
                    (Op::LimbIk { limb: leg, target: self.ik_target(target)?, weight: self.value(weight)?, rotation: false }, *max_lod)
                }
                OpSpec::FullBodyIk { effectors, pin_support, pin, iterations, max_lod } => {
                    let profile = self.source.profile()?.ok_or("FullBodyIk needs the skeleton's rig profile")?;
                    profile.validate(self.skeleton).map_err(|e| e.to_string())?;
                    let body = FullBody::new(self.skeleton, &profile).ok_or("FullBodyIk: the rig profile has no body to solve")?;
                    let in_body = |j: u16, what: &str| {
                        body.particle(usize::from(j)).map(|_| j).ok_or_else(|| format!("FullBodyIk: {what} is not part of the body"))
                    };
                    let mut list = Vec::with_capacity(effectors.len());
                    for e in effectors {
                        let joint = match (&e.limb, &e.joint) {
                            (Some(l), None) => self.out.legs[self.out.leg(l).ok_or_else(|| format!("unknown limb {l:?}"))?].effector(),
                            (None, Some(j)) => self.joint(j)?,
                            _ => return Err("a FullBodyIk effector names exactly one of limb or joint".into()),
                        };
                        list.push(BodyEffector {
                            joint: in_body(joint, &e.target)?,
                            target: self.ik_target(&e.target)?,
                            weight: self.value(&e.weight)?,
                            pull: e.pull.max(0.0),
                            rotation: e.rotation,
                        });
                    }
                    let mut names: Vec<String> =
                        if *pin_support { self.out.legs.iter().filter(|l| l.support).map(|l| l.name.clone()).collect() } else { Vec::new() };
                    names.extend(pin.iter().cloned());
                    let mut pins = Vec::with_capacity(names.len());
                    for n in &names {
                        let joint = self.out.legs[self.out.leg(n).ok_or_else(|| format!("unknown limb {n:?}"))?].effector();
                        if !pins.contains(&joint) && !list.iter().any(|e| e.joint == joint) {
                            pins.push(in_body(joint, n)?);
                        }
                    }
                    (Op::FullBodyIk { body: Box::new(body), effectors: list, pins, iterations: *iterations }, *max_lod)
                }
                OpSpec::ChainIk { chain, solver, target, weight, max_lod } => {
                    let joints = chain.iter().map(|j| self.joint(j)).collect::<R<Vec<_>>>()?;
                    let s = self.skeleton;
                    if joints.len() < 2 || joints.windows(2).any(|w| w[0] == w[1] || !s.is_ancestor(usize::from(w[0]), usize::from(w[1]))) {
                        return Err("ChainIk needs a parent-to-child chain of at least 2 joints".into());
                    }
                    (Op::ChainIk { joints, solver: *solver, target: self.ik_target(target)?, weight: self.value(weight)? }, *max_lod)
                }
                OpSpec::HitReact { joints, depth, falloff, gain, max_angle, halflife, max_lod } => {
                    let mut list = weights(self, joints)?;
                    if list.is_empty() {
                        // A skeleton without a profile gets an op with no joints (no reaction).
                        if let Some(p) = self.source.profile()? {
                            let mut add = |js: &[u16]| list.extend(js.iter().map(|&j| (j, 1.0)));
                            // The root girdle stays: turning it would swing the support legs.
                            let root = p.girdles.first().map(|g| g.joint);
                            if let Some(spine) = &p.spine {
                                add(&spine.joints.iter().copied().filter(|&j| Some(j) != root).collect::<Vec<_>>());
                            }
                            if let Some(nh) = &p.neck_head {
                                add(&nh.neck.joints);
                                add(&[nh.head]);
                            }
                            for l in p.limbs.iter().filter(|l| !l.support) {
                                add(&l.joints);
                            }
                        }
                    }
                    list.sort_by_key(|j| j.0);
                    list.dedup_by_key(|j| j.0);
                    let op = HitReactOp {
                        joints: list,
                        depth: (*depth).max(1),
                        falloff: falloff.clamp(0.0, 1.0),
                        gain: gain.max(0.0),
                        max_angle: max_angle.max(0.0),
                        halflife: halflife.max(1e-3),
                    };
                    (Op::HitReact(op), *max_lod)
                }
                OpSpec::SpringBones { chains: names, explicit, frequency, damping, gravity, inertia, max_angle, max_lod } => {
                    let mut chains = Vec::new();
                    if let Some(p) = self.source.profile()? {
                        let all = p.tails.iter().chain(&p.extra);
                        for c in all {
                            let wanted = if names.is_empty() { p.tails.iter().any(|t| t.name == c.name) } else { names.contains(&c.name) };
                            if wanted {
                                chains.push(c.joints.clone());
                            }
                        }
                    }
                    for c in explicit {
                        chains.push(c.iter().map(|j| self.joint(j)).collect::<R<Vec<_>>>()?);
                    }
                    let s = self.skeleton;
                    // Each joint's child in the chain must be its descendant.
                    chains.retain(|c| c.len() >= 2 && c.windows(2).all(|w| w[0] != w[1] && s.is_ancestor(usize::from(w[0]), usize::from(w[1]))));
                    let op = SpringBonesOp {
                        chains,
                        // Explicit Verlet at the substep stays stable well below ω·h = 1.
                        frequency: frequency.clamp(0.0, 0.5 / (std::f32::consts::TAU * crate::SPRING_SUBSTEP)),
                        damping: damping.max(0.0),
                        gravity: *gravity,
                        inertia: inertia.clamp(0.0, 1.0),
                        max_angle: max_angle.clamp(0.0, std::f32::consts::PI),
                    };
                    (Op::SpringBones(op), *max_lod)
                }
            };
            self.out.ops.push(OpEntry { op, max_lod });
        }
        Ok(())
    }
}

fn kind_name(additive: bool) -> &'static str {
    if additive { "additive" } else { "full" }
}

/// The rest-pose bend direction of a limb's solved span (character
/// space), if the chain is bent at all.
fn rest_bend(s: &Skeleton, joints: &[u16], limb_solver: LimbSolver) -> Option<Vec3> {
    let span = limb_solver.span(joints.len());
    if span < 3 {
        return None;
    }
    let chain = &joints[joints.len() - span..];
    let rest = s.rest_model();
    let p = |j: u16| Vec3::from(rest[usize::from(j)].translation);
    let (a, k, e) = (p(chain[0]), p(chain[1]), p(chain[span - 1]));
    let length: f32 = chain.windows(2).map(|w| p(w[0]).distance(p(w[1]))).sum();
    let line = e - a;
    let off = k - a;
    let side = off - line * (off.dot(line) / line.length_squared().max(1e-12));
    (side.length() > 0.02 * length).then(|| side.normalize())
}

fn last_of(joints: &[u16]) -> u16 {
    *joints.last().expect("limbs have joints")
}

/// A support limb's foot tip ([`Leg::toe`]): `toe` if given, else the
/// effector's child farthest from it horizontally in the rest pose (at
/// least 3 cm: a foot, not a stub); its probe slot is set later.
fn tip_of(s: &Skeleton, rest: &[glam::Affine3A], effector: u16, toe: Option<u16>) -> Option<(u16, u16, f32)> {
    let e = usize::from(effector);
    let flat = |j: usize| {
        let d = rest[j].translation - rest[e].translation;
        d.x.hypot(d.z)
    };
    let tip = toe.map(usize::from).or_else(|| (0..s.len()).filter(|&j| s.parent(j) == Some(e)).max_by(|&a, &b| flat(a).total_cmp(&flat(b))))?;
    (flat(tip) >= 0.03).then_some((u16::try_from(tip).ok()?, 0, rest[tip].translation.y))
}
