//! Clip references → cooked clips for one skeleton: the `CompileSource`
//! the validator and the headless simulation compile against.

use std::collections::HashMap;
use std::sync::Arc;

use mw_anim::{Animation, Group};
use sf_anim::{Clip, ClipRules, CookSettings, Looping, RigProfile, Skeleton, cook_clip};
use sf_anim_graph::asset::GraphAsset;
use sf_anim_graph::{CompileSource, CompiledGraph};
use sf_bake::ProductKey;
use sf_character::{HumanoidBone, RigMap};

/// A humanoid skeleton and its animation files, highest priority first
/// (female and beast files layer over the male one, as the original engine
/// loads them).
#[derive(Clone, Copy, Debug)]
pub struct HumanoidSet {
    pub name: &'static str,
    pub skeleton: &'static str,
    pub animations: &'static [&'static str],
    pub first_person: bool,
}

/// The four humanoid skeletons.
pub const HUMANOID_SETS: [HumanoidSet; 4] = [
    HumanoidSet { name: "male", skeleton: "meshes/base_anim.nif", animations: &["meshes/xbase_anim.kf"], first_person: false },
    HumanoidSet {
        name: "female",
        skeleton: "meshes/base_anim_female.nif",
        animations: &["meshes/xbase_anim_female.kf", "meshes/xbase_anim.kf"],
        first_person: false,
    },
    HumanoidSet { name: "beast", skeleton: "meshes/base_animkna.nif", animations: &["meshes/xbase_animkna.kf"], first_person: false },
    HumanoidSet { name: "first_person", skeleton: "meshes/base_anim.1st.nif", animations: &["meshes/xbase_anim.1st.kf"], first_person: true },
];

/// The animation files of a creature model (`r\Guar.NIF` → `meshes/r/guar.nif`),
/// highest priority first: its `x<name>.kf` (or `x<name>.nif`) when present,
/// the model's own keyframes, and for biped creatures (CREA flag `Biped`)
/// the humanoid `xbase_anim.kf` under them, as the original engine layers
/// them (dremora, golden saints, liches and skeletons carry few or no groups
/// of their own). `exists` says whether a VFS path exists. Returns the model
/// path and the files.
pub fn creature_animations(model: &str, biped: bool, exists: impl Fn(&str) -> bool) -> (String, Vec<String>) {
    let path = format!("meshes/{}", model.replace('\\', "/")).to_ascii_lowercase();
    let (dir, file) = path.rsplit_once('/').map_or(("", path.as_str()), |(d, f)| (d, f));
    let stem = file.rsplit_once('.').map_or(file, |(s, _)| s);
    let mut out = Vec::new();
    for x in [format!("{dir}/x{stem}.kf"), format!("{dir}/x{stem}.nif")] {
        if exists(&x) {
            out.push(x);
            break;
        }
    }
    out.push(path.clone());
    if biped {
        out.push(HUMANOID_SETS[0].animations[0].to_owned());
    }
    (path, out)
}

/// A reference resolved: file index, group, and the clip range in it.
type Resolved<'a> = (usize, &'a Group, Option<(f32, f32)>);

/// One skeleton with its animation groups: resolves the graph templates'
/// clip references (`Group`, `Group@section`, `Group@head`, `Group@tail`)
/// and cooks them.
pub struct AnimSet {
    pub name: String,
    pub skeleton: Skeleton,
    /// Humanoid skeletons: the rig map (`JointRef::Rig` names are VRM bones).
    pub rig: Option<RigMap>,
    pub profile: Option<RigProfile>,
    /// `(file, animation)` in priority order.
    pub animations: Vec<(String, Animation)>,
    groups: Vec<(usize, Group)>,
    cooked: HashMap<String, Arc<Clip>>,
    settings: CookSettings,
}

impl AnimSet {
    pub fn new(name: impl Into<String>, skeleton: Skeleton, animations: Vec<(String, Animation)>) -> Self {
        let mut groups: Vec<(usize, Group)> = Vec::new();
        for (i, (_, a)) in animations.iter().enumerate() {
            for g in a.groups() {
                // Earlier files win (layering).
                if !groups.iter().any(|(_, h)| h.name.eq_ignore_ascii_case(&g.name)) {
                    groups.push((i, g));
                }
            }
        }
        Self { name: name.into(), skeleton, rig: None, profile: None, animations, groups, cooked: HashMap::new(), settings: CookSettings::default() }
    }

    /// Adds the humanoid rig map (`mw_anim::rig::humanoid`).
    #[must_use]
    pub fn with_humanoid_rig(mut self) -> Self {
        if let Ok(rig) = mw_anim::rig::humanoid(&self.skeleton) {
            self.profile = rig.map.profile(&self.skeleton).ok();
            self.rig = Some(rig.map);
        }
        self
    }

    /// Group names, in file order.
    pub fn group_names(&self) -> impl Iterator<Item = &str> {
        self.groups.iter().map(|(_, g)| g.name.as_str())
    }

    pub fn group(&self, name: &str) -> Option<&Group> {
        self.groups.iter().find(|(_, g)| g.name.eq_ignore_ascii_case(name)).map(|(_, g)| g)
    }

    fn resolve(&self, reference: &str) -> Result<Resolved<'_>, String> {
        let (group, part) = reference.split_once('@').map_or((reference, None), |(g, p)| (g, Some(p)));
        let (file, g) = self.groups.iter().find(|(_, g)| g.name.eq_ignore_ascii_case(group)).ok_or_else(|| format!("no group {group:?}"))?;
        let range = match part {
            None => None,
            Some(p @ ("head" | "tail")) => Some(g.loop_part(p).ok_or_else(|| format!("{group} has no loop {p}"))?),
            Some(s) => {
                let sec = g.section(s).ok_or_else(|| format!("{group} has no {s:?} section"))?;
                Some((sec.start, sec.stop))
            }
        };
        Ok((*file, g, range))
    }

    /// The cooked clip a reference names (cached).
    pub fn cooked(&mut self, reference: &str) -> Result<Arc<Clip>, String> {
        if let Some(c) = self.cooked.get(reference) {
            return Ok(c.clone());
        }
        let (file, group, range) = self.resolve(reference)?;
        let animation = &self.animations[file].1;
        let mut source = animation.source_clip(group);
        reference.clone_into(&mut source.name);
        let rules = match range {
            Some(r) => ClipRules {
                range: Some(r),
                looping: Looping::Once,
                root_motion: mw_anim::root_motion_rules(&group.name, reference.split_once('@').map(|(_, p)| p)),
                ..ClipRules::default()
            },
            None => mw_anim::clip_rules(group),
        };
        let clip = Arc::new(cook_clip(&self.skeleton, &source, &rules, &self.settings).map_err(|e| format!("{reference}: {e}"))?);
        self.cooked.insert(reference.to_owned(), clip.clone());
        Ok(clip)
    }

    /// The clips of a compiled graph, in its clip-slot order (for a `Binding`).
    pub fn binding_clips(&mut self, graph: &CompiledGraph) -> Result<Vec<Arc<Clip>>, String> {
        graph.clips.iter().map(|c| self.cooked(&c.reference)).collect()
    }

    /// Compiles a graph asset for this skeleton.
    pub fn compile(&mut self, name: &str, asset: &GraphAsset) -> Result<CompiledGraph, String> {
        let skeleton = self.skeleton.clone();
        sf_anim_graph::compile(name, asset, &skeleton, self).map_err(|e| e.to_string())
    }
}

impl CompileSource for AnimSet {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        let clip = self.cooked(reference)?;
        Ok((ProductKey::new("anim.cook", format!("{}#{reference}", self.name)), (*clip).clone()))
    }

    fn available(&mut self, reference: &str) -> Result<bool, String> {
        Ok(self.resolve(reference).is_ok())
    }

    fn subgraph(&mut self, path: &str) -> Result<GraphAsset, String> {
        Err(format!("no subgraph {path:?}: Morrowind templates are single files"))
    }

    fn rig_joint(&self, rig: &str) -> Option<String> {
        let bone = HumanoidBone::from_name(rig)?;
        self.rig.as_ref()?.joint(bone).map(str::to_owned)
    }

    fn profile(&mut self) -> Result<Option<RigProfile>, String> {
        Ok(self.profile.clone())
    }
}
