//! The `animgraph.compile` bake step (animation.md "Bake steps"), on the
//! conventions of `sf_anim::steps`.
//!
//! Subject: [`graph_subject`] — `<graph file>|<skeleton>`, where the
//! skeleton is a `skeleton.cook` subject. Planned inputs: the graph file
//! and the skeleton product. Clip products (and subgraph files) are read
//! during the run, so they are recorded as dynamic inputs: re-cooking a
//! clip recompiles the graphs that play it. Before loading a clip, the step
//! checks its source files ([`GraphRules::clip_files`]); the check is a
//! dynamic input even when a file is missing, so adding a bring-your-own
//! source later recompiles a graph that fell back or dropped (animation.md
//! "Optional clips").
//!
//! Adapter data (rig bone names, mirror pairs) is not a file, so it enters
//! the bake key as a parameter: the postcard bytes of [`GraphRules::rig`].

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sf_anim::Clip;
use sf_anim::steps::{ANIM_COOK, RIG_PROFILE, SKELETON_COOK, anim_subject};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};

use crate::asset::{GraphAsset, parse_graph};
use crate::compile::{CompileSource, compile};
use crate::compiled::{GRAPH_FORMAT, Provenance};

pub const ANIMGRAPH_COMPILE: &str = "animgraph.compile";

/// The `animgraph.compile` subject for `graph` on the skeleton from `skeleton`.
pub fn graph_subject(graph: &str, skeleton: &str) -> String {
    format!("{graph}|{skeleton}")
}

/// `(graph, skeleton)` of an `animgraph.compile` subject.
pub fn parse_graph_subject(subject: &str) -> Option<(&str, &str)> {
    subject.split_once('|')
}

/// Rig data a graph compiles against: rig bone → joint names (for
/// `JointRef::Rig`), and left/right pairs for `Mirror` nodes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphRig {
    pub joints: Vec<(String, String)>,
    pub mirror: Vec<(String, String)>,
}

/// The default clip-reference form: `<file>#<clip>` → the `anim.cook`
/// product of that clip on `skeleton`.
pub fn default_clip_product(skeleton: &str, reference: &str) -> Option<ProductKey> {
    let (file, clip) = reference.split_once('#')?;
    Some(ProductKey::new(ANIM_COOK, anim_subject(skeleton, file, clip)))
}

/// Adapter hooks of `animgraph.compile`. Must be deterministic.
pub trait GraphRules: Send + Sync {
    /// The product a clip reference names (e.g. an `anim.retarget` product
    /// when the clip comes from another skeleton).
    fn clip_product(&self, _graph: &str, skeleton: &str, reference: &str) -> Option<ProductKey> {
        default_clip_product(skeleton, reference)
    }
    /// The source files a clip reference needs: `animgraph.compile` checks
    /// them (recorded as dependencies, so a file added later recompiles
    /// the graph) before loading the clip, and a missing optional clip
    /// takes its fallback. Default: the `<file>` of `<file>#<clip>`.
    fn clip_files(&self, _graph: &str, _skeleton: &str, reference: &str) -> Vec<String> {
        reference.split_once('#').map(|(file, _)| vec![file.to_owned()]).unwrap_or_default()
    }
    /// Where a resolved clip came from (source library, license, whether
    /// derivatives may be shared), carried into the compiled graph for
    /// export checks. Default: the project's own, shareable content.
    fn provenance(&self, _graph: &str, _reference: &str, _product: &ProductKey) -> Provenance {
        Provenance::default()
    }
    fn rig(&self, _skeleton: &str) -> GraphRig {
        GraphRig::default()
    }
    /// Bump when [`Self::clip_product`], [`Self::clip_files`] or
    /// [`Self::provenance`] change their answers (part of the key).
    fn version(&self) -> u32 {
        0
    }
}

/// Framework defaults: `file#clip` references, no rig data.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultGraphRules;

impl GraphRules for DefaultGraphRules {}

/// `animgraph.compile`: a graph asset compiled for one skeleton.
pub struct AnimGraphCompileStep(pub Arc<dyn GraphRules>);

impl AnimGraphCompileStep {
    pub fn register(rules: Arc<dyn GraphRules>, baker: &mut Baker) {
        baker.register(Self(rules));
    }

    fn parse(subject: &str) -> Result<(&str, &str), BakeError> {
        parse_graph_subject(subject).ok_or_else(|| BakeError::step(&ProductKey::new(ANIMGRAPH_COMPILE, subject), "bad subject"))
    }
}

struct StepSource<'a, 'b> {
    cx: &'a mut RunCx<'b>,
    rules: &'a dyn GraphRules,
    graph: &'a str,
    skeleton: &'a str,
    rig: GraphRig,
}

fn read_text(cx: &mut RunCx<'_>, path: &str) -> Result<GraphAsset, String> {
    let bytes = cx.read_file(path).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("{path}: {e}"))?;
    parse_graph(text).map_err(|e| format!("{path}: {e}"))
}

impl CompileSource for StepSource<'_, '_> {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        let key =
            self.rules.clip_product(self.graph, self.skeleton, reference).ok_or_else(|| format!("unresolvable clip reference {reference:?}"))?;
        let bytes = self.cx.product(&key.step, &key.subject).map_err(|e| e.to_string())?;
        let clip = Clip::from_bytes(&bytes).map_err(|e| e.to_string())?;
        Ok((key, clip))
    }
    /// Present when the adapter maps the reference and every source file
    /// exists. Each check is a dynamic input, missing files included.
    fn available(&mut self, reference: &str) -> Result<bool, String> {
        if self.rules.clip_product(self.graph, self.skeleton, reference).is_none() {
            return Ok(false);
        }
        for file in self.rules.clip_files(self.graph, self.skeleton, reference) {
            if !self.cx.file_exists(&file).map_err(|e| e.to_string())? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn provenance(&mut self, reference: &str, product: &ProductKey) -> Provenance {
        self.rules.provenance(self.graph, reference, product)
    }
    fn subgraph(&mut self, path: &str) -> Result<GraphAsset, String> {
        read_text(self.cx, path)
    }
    fn rig_joint(&self, rig: &str) -> Option<String> {
        self.rig.joints.iter().find(|(r, _)| r == rig).map(|(_, j)| j.clone())
    }
    fn mirror_pairs(&self) -> Vec<(String, String)> {
        self.rig.mirror.clone()
    }
    /// The `rig.profile` product of the graph's skeleton (a dynamic input:
    /// only graphs that use `profile_limbs` depend on it).
    fn profile(&mut self) -> Result<Option<sf_anim::RigProfile>, String> {
        let bytes = self.cx.product(RIG_PROFILE, self.skeleton).map_err(|e| e.to_string())?;
        sf_anim::RigProfile::from_bytes(&bytes).map(Some).map_err(|e| e.to_string())
    }
}

impl Step for AnimGraphCompileStep {
    fn id(&self) -> &'static str {
        ANIMGRAPH_COMPILE
    }
    fn version(&self) -> u32 {
        3
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (graph, skeleton) = Self::parse(subject)?;
        cx.file(graph).product(SKELETON_COOK, skeleton);
        cx.param("rig", postcard::to_stdvec(&self.0.rig(skeleton)).expect("in-memory serialization"));
        cx.param("rules", self.0.version().to_le_bytes().to_vec());
        cx.param("format", GRAPH_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(ANIMGRAPH_COMPILE, subject);
        let (graph, skeleton_path) = Self::parse(subject)?;
        let asset = read_text(cx, graph).map_err(|e| BakeError::step(&key, e))?;
        let skel_bytes = cx.product(SKELETON_COOK, skeleton_path)?;
        let skeleton = sf_anim::Skeleton::from_bytes(&skel_bytes).map_err(|e| BakeError::step(&key, format!("skeleton {skeleton_path:?}: {e}")))?;
        let mut source = StepSource { cx, rules: self.0.as_ref(), graph, skeleton: skeleton_path, rig: self.0.rig(skeleton_path) };
        let compiled = compile(graph, &asset, &skeleton, &mut source).map_err(|e| BakeError::step(&key, e))?;
        Ok(compiled.to_bytes())
    }
}
