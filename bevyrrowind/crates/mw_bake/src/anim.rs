//! Morrowind actor animation through the bake (superfargoth
//! `animation.md` "Bake steps"; `docs/specs/bake-adapter.md` "Animation").
//!
//! The framework steps do the work: `skeleton.cook`, `rig.profile`,
//! `anim.cook` (`sf_anim::steps`) and `animgraph.compile`
//! (`sf_anim_graph::steps`). This module supplies what they need:
//!
//! - [`actor_sets`]: every (skeleton, animation files, graphs) the actors
//!   use, from `mw_actors` recipes over `NPC_` and `CREA` (first person
//!   included), with the original engine's file layering.
//! - [`AnimGroupsStep`] (`mw.anim.groups`): a file's animation groups
//!   (names, loop range, sub-action sections), read from its text keys.
//!   Clip rules and graph references depend on them, and bake keys must be
//!   known at plan time, so [`bake_animation`] runs in two phases:
//!   skeletons and group tables first (cheap, warm after the first bake),
//!   then clips and graphs with [`MwAnimRules`] built from those products.
//! - [`MwAnimRules`]: the adapter's `AnimRules` (cook rules per group,
//!   section and loop tail; rig hints) and `GraphRules` (graph reference
//!   `Group`, `Group@section`, `Group@tail` → `anim.cook` product on the
//!   actor's skeleton, with layering).
//! - Library clips ([`crate::sources`], animation-sources.md): a graph
//!   reference `source:clip` cooks on the library file's own skeleton
//!   (`skeleton.cook` of `animsrc/<source>/<file>`, auto-mapped humanoid rig)
//!   and `anim.retarget`s onto the actor's humanoid skeleton, with the
//!   source's license as provenance; a missing file drops the clip
//!   ([`SourceClipsStep`] lists a present file's clips first).
//! - [`AnimLibrary`]: the Bevy-free loading API: an actor's cooked
//!   skeleton, rig profile, compiled graph and clips from the store.
//!
//! Subjects: `skeleton.cook` = skeleton model (`meshes/base_anim.nif`,
//! `meshes/r/guar.nif`); `anim.cook` = `<skeleton>|<file>|<reference>`
//! (`meshes/base_anim_female.nif|meshes/xbase_anim.kf|WeaponOneHand@chop`);
//! `animgraph.compile` = `<graph>|<skeleton>`.

use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mw_actors::{CREATURE_KIND, MwActors, NPC_KIND};
use mw_anim::{Animation, KfAnimImporter, NifAnimImporter};
use mw_graphs::{HUMANOID_SETS, HumanoidSet};
use serde::{Deserialize, Serialize};
use sf_anim::steps::{
    ANIM_COOK, ANIM_RETARGET, AnimBake, AnimImporters, AnimRules, ClipSpec, RIG_PROFILE, SKELETON_COOK, anim_subject, parse_anim_subject,
    retarget_subject,
};
use sf_anim::{Clip, ClipRules, CookSettings, Looping, RetargetRig, RigHints, RigProfile, RootMotionRules, Skeleton, SkeletonRules};
use sf_anim_graph::{CompiledGraph, Provenance};
use sf_anim_graph::steps::{ANIMGRAPH_COMPILE, AnimGraphCompileStep, GraphRig, GraphRules, graph_subject};
use sf_bake::{BakeError, Baker, Digest, Manifest, PlanCx, ProductKey, RunCx, Step, Store};
use sf_character::{CharacterRecipe, CharacterSource, CharacterState, HumanoidBone, RetargetOptions};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Creature, Npc, NpcFlags, Race, RaceFlags};

use crate::sources::AnimSources;

/// The `mw.anim.groups` step id.
pub const ANIM_GROUPS: &str = "mw.anim.groups";

/// The `mw.anim.source_clips` step id.
pub const SOURCE_CLIPS: &str = "mw.anim.source_clips";

/// The animation importers of the bake: `.kf` and `.nif` (skeletons and
/// model keyframes), glTF/GLB and BVH (the user's clip libraries,
/// [`crate::sources`]). BVH files take their source's `unit_scale` and `up`
/// from the shipped manifest (Z up: facing -Y, the Blender / 3ds Max layout).
pub fn anim_importers() -> AnimImporters {
    let mut bvh = sf_anim::BvhAnimImporter::default();
    for (name, source) in &mw_graphs::sources::SourcesManifest::builtin().sources {
        let (up, forward) = match source.up {
            mw_graphs::sources::Axis::Y => (sf_anim::Axis::PosY, sf_anim::Axis::PosZ),
            mw_graphs::sources::Axis::Z => (sf_anim::Axis::PosZ, sf_anim::Axis::NegY),
        };
        let space = sf_anim::SourceSpace { unit_scale: source.unit_scale, up, forward, right_handed: true };
        bvh = bvh.with_override(AnimSources::virtual_path(name, ""), space);
    }
    AnimImporters::new()
        .with(Arc::new(KfAnimImporter))
        .with(Arc::new(NifAnimImporter))
        .with(Arc::new(sf_anim::GltfAnimImporter))
        .with(Arc::new(bvh))
}

// ---------------------------------------------------------------------------
// Group tables.

/// One animation group of a file, as the cook rules need it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupInfo {
    /// As written in the file (`WalkForward`).
    pub name: String,
    pub duration: f32,
    pub looping: Looping,
    /// `Loop Start` .. `Loop Stop`, seconds from the group start.
    pub loop_range: Option<(f32, f32)>,
    /// `(action word, start, stop)`, seconds from the group start.
    pub sections: Vec<(String, f32, f32)>,
}

impl GroupInfo {
    pub fn of(group: &mw_anim::Group) -> Self {
        Self {
            name: group.name.clone(),
            duration: group.duration(),
            looping: group.looping(),
            loop_range: group.loop_range,
            sections: group.sections.iter().map(|s| (s.name.clone(), s.start, s.stop)).collect(),
        }
    }

    /// The cook rules of the whole group (`None`), its loop tail
    /// (`"tail"`) or a section (`"chop"`): as `mw_graphs::AnimSet::cooked`.
    pub fn rules(&self, part: Option<&str>) -> Option<ClipRules> {
        let once = |range| ClipRules { range: Some(range), looping: Looping::Once, root_motion: mw_anim::root_motion_rules(&self.name, part), ..ClipRules::default() };
        match part {
            None => Some(ClipRules { looping: self.looping, ..ClipRules::default() }),
            Some("tail") => self.loop_range.map(|(_, stop)| once((stop, self.duration))),
            Some(s) => self.sections.iter().find(|(n, ..)| n == s).map(|(_, a, b)| once((*a, *b))),
        }
    }
}

/// `Group@part` → (`Group`, `Some(part)`).
fn split_reference(reference: &str) -> (&str, Option<&str>) {
    reference.split_once('@').map_or((reference, None), |(g, p)| (g, Some(p)))
}

/// `mw.anim.groups`: the animation groups of a `.kf` or `.nif` file, from
/// its text keys (postcard `Vec<GroupInfo>`, timeline order).
pub struct AnimGroupsStep;

impl AnimGroupsStep {
    pub fn decode(bytes: &[u8]) -> Result<Vec<GroupInfo>, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl Step for AnimGroupsStep {
    fn id(&self) -> &'static str {
        ANIM_GROUPS
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.file(subject);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let bytes = cx.read_file(subject)?;
        let animation =
            Animation::from_file(subject, &bytes).map_err(|e| BakeError::step(&ProductKey::new(ANIM_GROUPS, subject), e.to_string()))?;
        let groups: Vec<GroupInfo> = animation.groups().iter().map(GroupInfo::of).collect();
        Ok(postcard::to_stdvec(&groups).expect("in-memory serialization"))
    }
}

/// `mw.anim.source_clips`: the clip names a library file holds (postcard
/// `Vec<String>`), so a slot whose clip the user's version of the file lacks
/// falls back instead of failing its graph.
pub struct SourceClipsStep;

impl SourceClipsStep {
    pub fn decode(bytes: &[u8]) -> Result<Vec<String>, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl Step for SourceClipsStep {
    fn id(&self) -> &'static str {
        SOURCE_CLIPS
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.file(subject);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(SOURCE_CLIPS, subject);
        let bytes = cx.read_file(subject)?;
        let gltf = std::path::Path::new(subject).extension().is_some_and(|e| e.eq_ignore_ascii_case("glb") || e.eq_ignore_ascii_case("gltf"));
        let names = if gltf {
            sf_anim::list_gltf_clips(subject, &bytes).map_err(|e| BakeError::step(&key, e.to_string()))?
        } else {
            let importer = anim_importers().for_path(subject).cloned().ok_or_else(|| BakeError::step(&key, "no animation importer"))?;
            importer.import(subject, &bytes).map_err(|e| BakeError::step(&key, e.to_string()))?.clips.into_iter().map(|c| c.name).collect()
        };
        Ok(postcard::to_stdvec(&names).expect("in-memory serialization"))
    }
}

// ---------------------------------------------------------------------------
// Actor animation sets.

/// One skeleton the actors use, with its animation files and graphs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActorAnimSet {
    /// The `skeleton.cook` subject.
    pub skeleton: String,
    /// Animation files, highest priority first (a group comes from the
    /// first file that has it).
    pub files: Vec<String>,
    /// Files that would join the set if a mod added them (`x<model>.kf`):
    /// graph compiles check them, so adding one recompiles.
    pub candidates: Vec<String>,
    /// Humanoid rig map (NPC skeletons, first person, biped creatures);
    /// else a creature rig profile.
    pub humanoid: bool,
    /// Graph assets played on it (`animgraph.compile` graph paths).
    pub graphs: BTreeSet<String>,
    /// Actor records using it.
    pub actors: u32,
}

/// Every animation set the actors use.
#[derive(Clone, Debug, Default)]
pub struct ActorSets {
    /// By skeleton.
    pub sets: BTreeMap<String, ActorAnimSet>,
    /// Actors whose recipe failed: `kind id: error`.
    pub skipped: Vec<String>,
    /// A skeleton seen with two file lists (the first, in record id order, is kept).
    pub conflicts: Vec<String>,
    /// Creature body models (a creature's model is both its skeleton and
    /// its body): cooked for their rig as `mesh.rig` ([`Self::body_rig_keys`]).
    pub bodies: BTreeSet<String>,
    /// Rigid body-part models (`BODY` meshes) the plain `mesh.cook` skips for
    /// an animated node ([`rigid_part_needs_rig`]): cooked as `mesh.rig` too,
    /// which character assembly prefers, so the part shows at its rest pose.
    pub rigid_parts: BTreeSet<String>,
}

/// Whether a body-part model needs the `mesh.rig` cook to render as a rigid
/// part: nothing in it deforms, but a node is animated, so `sf_model::cook`
/// skips it (`Cooked::Skipped { "animated node" }`). Vanilla: 18 models,
/// iron/glass/bonemold/adamantium/orcish boots, common shoes 1 and 2, some
/// pauldrons and pants knees (2026-10-12, `.sessions/missingparts.md`). Their
/// roots carry a one-key, non-auto-play transform controller; the original
/// renderer never runs controllers on body parts other than the head (time
/// stays 0), so the rest pose `mesh.rig` binds is what shows.
pub fn rigid_part_needs_rig(model: &sf_model::Model) -> bool {
    !model.meshes.iter().any(sf_model::MeshData::deforms) && model.nodes.iter().any(|n| n.animated)
}

impl ActorSets {
    fn add(&mut self, mut set: ActorAnimSet, graph: &str) {
        match self.sets.entry(set.skeleton.clone()) {
            Entry::Vacant(e) => {
                set.graphs.insert(graph.to_owned());
                set.actors = 1;
                e.insert(set);
            }
            Entry::Occupied(mut e) => {
                let kept = e.get_mut();
                if kept.files != set.files || kept.humanoid != set.humanoid {
                    self.conflicts.push(format!("{}: {:?} kept over {:?}", set.skeleton, kept.files, set.files));
                }
                kept.graphs.insert(graph.to_owned());
                kept.actors += 1;
            }
        }
    }

    /// Every animation file of every set, sorted.
    pub fn files(&self) -> BTreeSet<&str> {
        self.sets.values().flat_map(|s| s.files.iter().map(String::as_str)).collect()
    }

    /// `(graph, skeleton)` pairs.
    pub fn graph_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.sets.values().flat_map(|s| s.graphs.iter().map(|g| (g.as_str(), s.skeleton.as_str())))
    }

    /// `mesh.rig` keys of every creature body ([`sf_model::cook_rig`]:
    /// rigid meshes on their nodes, so per-node animated creatures render
    /// through the character pipeline like skinned ones) and of the
    /// [`Self::rigid_parts`].
    pub fn body_rig_keys(&self) -> Vec<ProductKey> {
        self.bodies.iter().chain(&self.rigid_parts).map(|b| ProductKey::new(sf_model::steps::MeshRigStep::ID, b.as_str())).collect()
    }

    /// Keeps only the sets whose skeleton passes `keep` (tests, tools).
    #[must_use]
    pub fn filtered(mut self, keep: impl Fn(&str) -> bool) -> Self {
        self.sets.retain(|s, _| keep(s));
        self.bodies.retain(|b| keep(b));
        self.rigid_parts.retain(|b| keep(b));
        self
    }
}

/// `meshes/dir/name.nif` → (`meshes/dir`, `name`).
fn dir_stem(path: &str) -> (&str, &str) {
    let (dir, file) = path.rsplit_once('/').unwrap_or(("", path));
    (dir, file.rsplit_once('.').map_or(file, |(s, _)| s))
}

/// A humanoid skeleton's files: a stock skeleton plays its own set
/// ([`HUMANOID_SETS`]); a custom one (an NPC's model, a `.1st` variant)
/// layers its `x<model>.kf` over the race and gender set `base`.
fn humanoid_set(skeleton: &str, base: &HumanoidSet, exists: &impl Fn(&str) -> bool) -> ActorAnimSet {
    let owned = |files: &[&str]| files.iter().map(|f| (*f).to_owned()).collect::<Vec<_>>();
    if let Some(stock) = HUMANOID_SETS.iter().find(|s| s.skeleton == skeleton) {
        return ActorAnimSet { skeleton: skeleton.to_owned(), files: owned(stock.animations), humanoid: true, ..ActorAnimSet::default() };
    }
    let (dir, stem) = dir_stem(skeleton);
    let x = format!("{dir}/x{stem}.kf");
    let (mut files, candidates) = if exists(&x) { (vec![x], Vec::new()) } else { (Vec::new(), vec![x]) };
    files.extend(owned(base.animations));
    ActorAnimSet { skeleton: skeleton.to_owned(), files, candidates, humanoid: true, ..ActorAnimSet::default() }
}

/// The animation sets of every `NPC_` and `CREA` record, from `mw_actors`
/// recipes (skeleton and graph paths: one rule, shared with the runtime).
/// Records are visited in id order, so the result is deterministic.
pub fn actor_sets(db: &Arc<RecordDb>, vfs: &Arc<Vfs>) -> ActorSets {
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));
    // Recipes only name part models; nothing here probes them.
    let actors = MwActors::with_probe(db.clone(), vfs.clone(), Box::new(|_| None));
    let state = CharacterState::default();
    let mut sets = ActorSets::default();

    let mut npcs: Vec<(&RecordId, &Npc)> = db.iter::<Npc>().collect();
    npcs.sort_by(|a, b| a.0.cmp(b.0));
    for (id, npc) in npcs {
        let recipe = match actors.recipe(NPC_KIND, id, &state) {
            Ok(r) => r,
            Err(e) => {
                sets.skipped.push(format!("{NPC_KIND} {id:?}: {e}"));
                continue;
            }
        };
        let female = npc.npc_flags.contains(NpcFlags::FEMALE);
        let beast = db.get::<Race>(&RecordId::new(&npc.race)).is_some_and(|r| r.data.flags.contains(RaceFlags::BEAST_RACE));
        let base = &HUMANOID_SETS[if beast { 2 } else { usize::from(female) }];
        sets.add(humanoid_set(recipe.skeleton.as_str(), base, &exists), recipe.graph.as_str());
        if let Some(fp) = &recipe.first_person {
            sets.add(humanoid_set(fp.skeleton.as_str(), &HUMANOID_SETS[3], &exists), fp.graph.as_str());
        }
    }

    let mut creatures: Vec<(&RecordId, &Creature)> = db.iter::<Creature>().collect();
    creatures.sort_by(|a, b| a.0.cmp(b.0));
    for (id, crea) in creatures {
        let recipe = match actors.recipe(CREATURE_KIND, id, &state) {
            Ok(r) => r,
            Err(e) => {
                sets.skipped.push(format!("{CREATURE_KIND} {id:?}: {e}"));
                continue;
            }
        };
        let biped = mw_graphs::creature_template(u32::from(crea.creature_flags.bits())) == mw_graphs::Template::Humanoid;
        let (_, mut files) = mw_graphs::creature_animations(&crea.mesh, biped, exists);
        files.retain(|f| exists(f));
        let (dir, stem) = dir_stem(recipe.skeleton.as_str());
        let candidates = [format!("{dir}/x{stem}.kf"), format!("{dir}/x{stem}.nif")].into_iter().filter(|c| !files.contains(c)).collect();
        sets.bodies.insert(recipe.skeleton.as_str().to_owned());
        let set = ActorAnimSet { skeleton: recipe.skeleton.as_str().to_owned(), files, candidates, humanoid: biped, ..ActorAnimSet::default() };
        sets.add(set, recipe.graph.as_str());
    }

    // Body-part models (skin, head, hair and item parts are all `BODY`
    // meshes) whose plain cook would skip them.
    let models: BTreeSet<String> =
        db.iter::<tes3::esp::Bodypart>().filter_map(|(_, b)| mw_world::model_path(&b.mesh)).filter(|m| exists(m)).collect();
    for path in models {
        let Ok(vpath) = VfsPath::new(&path) else { continue };
        let Some(nif) = vfs.read(&vpath).ok().and_then(|bytes| mw_nif::load(&bytes).ok()) else { continue };
        if !nif.had_skin && rigid_part_needs_rig(&crate::to_model(&nif)) {
            sets.rigid_parts.insert(path);
        }
    }
    sets
}

// ---------------------------------------------------------------------------
// Adapter rules.

/// What the rules know about one skeleton.
#[derive(Clone, Debug, Default)]
struct SkeletonData {
    files: Vec<String>,
    /// `files` plus `candidates`: what a graph compile checks.
    watch: Vec<String>,
    hints: RigHints,
    rig: GraphRig,
    /// The humanoid retarget rig (`anim.retarget` source or target).
    retarget: Option<RetargetRig>,
}

/// A library clip reference (`quaternius_ual1:ClimbLedge_RM`) as the rules
/// resolve it.
#[derive(Clone, Debug)]
struct LibraryRef {
    /// Virtual path of the library file (the source skeleton too).
    file: String,
    spec: ClipSpec,
    /// Root motion generated on the target (`AnimRules::retarget_root`).
    retarget_root: Option<RootMotionRules>,
    provenance: Provenance,
    /// `None`: the file is missing (the graph compile checks it and drops
    /// the clip); `Some(false)`: present but unusable (no such clip, or
    /// its skeleton didn't map), so the reference doesn't resolve.
    usable: Option<bool>,
}

/// Base of [`GraphRules::version`].
const GRAPH_RULES_VERSION: u32 = 2;

/// The adapter's `AnimRules` and `GraphRules`, built from the group tables
/// and cooked skeletons of [`bake_animation`]'s first phase. Deterministic
/// for the same products.
#[derive(Clone, Debug, Default)]
pub struct MwAnimRules {
    groups: HashMap<String, Vec<GroupInfo>>,
    skeletons: HashMap<String, SkeletonData>,
    /// Library clip references ([`crate::sources`]) by reference.
    library: BTreeMap<String, LibraryRef>,
}

impl MwAnimRules {
    /// The skeleton rules: names guessed into tags, root motion on the
    /// joint the NIF importer reports (`Bip01` when present): exactly
    /// `mw_anim::rig::skeleton`.
    pub fn skeleton_rules() -> SkeletonRules {
        SkeletonRules { guess_tags: true, detect_root_motion: true, ..SkeletonRules::default() }
    }

    /// Adds a file's groups.
    pub fn add_groups(&mut self, file: &str, groups: Vec<GroupInfo>) {
        self.groups.insert(file.to_owned(), groups);
    }

    /// Adds a skeleton's set with its cooked skeleton: humanoid sets get the
    /// humanoid rig map (rig-profile override and `JointRef::Rig` names),
    /// creatures `mw_anim::rig::creature_hints`.
    pub fn add_skeleton(&mut self, set: &ActorAnimSet, skeleton: &Skeleton) {
        let mut data = SkeletonData {
            files: set.files.clone(),
            watch: set.files.iter().chain(&set.candidates).cloned().collect(),
            ..SkeletonData::default()
        };
        if set.humanoid {
            if let Ok(rig) = mw_anim::rig::humanoid(skeleton) {
                data.hints.profile = rig.map.profile(skeleton).ok();
                data.rig.joints =
                    HumanoidBone::ALL.iter().filter_map(|b| rig.map.joint(*b).map(|j| (b.name().to_owned(), j.to_owned()))).collect();
                // Target of library clips: legs keep the source's contacts by IK.
                data.retarget = rig.map.retarget_rig(skeleton, RetargetOptions::default()).ok();
            }
        } else {
            data.hints = mw_anim::rig::creature_hints(&set.skeleton);
        }
        self.skeletons.insert(set.skeleton.clone(), data);
    }

    /// Adds every library clip reference of `sources` (unresolved until
    /// [`Self::add_library_file`] sees its file).
    pub fn add_library(&mut self, sources: &AnimSources) {
        for (file, r) in sources.references() {
            let source = &sources.manifest.sources[&r.source];
            let provenance = Provenance { source: r.source.clone(), license: source.license.tag().to_owned(), redistributable: source.redistributable() };
            let spec = ClipSpec { source: r.clip.clone(), rules: r.rules.clone() };
            self.library.insert(r.reference.clone(), LibraryRef { file, spec, retarget_root: r.retarget_root.clone(), provenance, usable: None });
        }
    }

    /// A present library file: its cooked skeleton (auto-mapped humanoid
    /// rig, the retarget source) and the clip names it holds. Returns the
    /// references it can't serve, with why.
    pub fn add_library_file(&mut self, file: &str, skeleton: Option<&Skeleton>, clips: &[String]) -> Vec<(String, String)> {
        let rig = skeleton.and_then(|s| {
            let (mut map, _) = sf_character::auto_map(s);
            map.auto_align(s);
            let retarget = map.retarget_rig(s, RetargetOptions { ik_legs: false }).ok()?;
            Some((s, map, retarget))
        });
        let mapped = rig.is_some();
        if let Some((s, map, retarget)) = rig {
            let hints = RigHints { profile: map.profile(s).ok(), ..RigHints::default() };
            self.skeletons.insert(file.to_owned(), SkeletonData { hints, retarget: Some(retarget), ..SkeletonData::default() });
        }
        let mut refused = Vec::new();
        for (reference, r) in self.library.iter_mut().filter(|(_, r)| r.file == file) {
            let has_clip = clips.contains(&r.spec.source);
            r.usable = Some(mapped && has_clip);
            if !mapped {
                refused.push((reference.clone(), "its skeleton has no humanoid retarget rig".to_owned()));
            } else if !has_clip {
                refused.push((reference.clone(), format!("the file has no clip {:?} (another version?)", r.spec.source)));
            }
        }
        refused
    }

    /// The `anim.retarget` product of a usable (or not yet checked) library
    /// reference on a humanoid target skeleton.
    fn library_product(&self, skeleton: &str, reference: &str) -> Option<ProductKey> {
        let r = self.library.get(reference).filter(|r| r.usable != Some(false))?;
        self.skeletons.get(skeleton)?.retarget.as_ref()?;
        Some(ProductKey::new(ANIM_RETARGET, retarget_subject(skeleton, &anim_subject(&r.file, &r.file, reference))))
    }

    fn file_group(&self, file: &str, group: &str) -> Option<&GroupInfo> {
        self.groups.get(file)?.iter().find(|g| g.name.eq_ignore_ascii_case(group))
    }

    /// The file a reference's group comes from on `skeleton` (the first
    /// file of its set that has the group) and the clip spec, or `None`
    /// when the group, section or loop is missing.
    pub fn resolve(&self, skeleton: &str, reference: &str) -> Option<(&str, ClipSpec)> {
        let (group, part) = split_reference(reference);
        let set = self.skeletons.get(skeleton)?;
        let (file, info) = set.files.iter().find_map(|f| self.file_group(f, group).map(|g| (f.as_str(), g)))?;
        Some((file, ClipSpec { source: info.name.clone(), rules: info.rules(part)? }))
    }

    /// Every group playable on `skeleton`, layered: `(file, group name)`.
    pub fn set_groups(&self, skeleton: &str) -> Vec<(&str, &str)> {
        let Some(set) = self.skeletons.get(skeleton) else { return Vec::new() };
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for file in &set.files {
            for g in self.groups.get(file).into_iter().flatten() {
                if seen.insert(g.name.to_ascii_lowercase()) {
                    out.push((file.as_str(), g.name.as_str()));
                }
            }
        }
        out
    }
}

impl AnimRules for MwAnimRules {
    fn skeleton(&self, _path: &str) -> SkeletonRules {
        Self::skeleton_rules()
    }
    /// The subject pins the file, so only that file's group table counts.
    /// Library references carry the manifest's clip and cook rules.
    fn clip(&self, file: &str, clip: &str, _skeleton: &str) -> ClipSpec {
        if let Some(r) = self.library.get(clip).filter(|r| r.file == file) {
            return r.spec.clone();
        }
        let (group, part) = split_reference(clip);
        self.file_group(file, group)
            .and_then(|g| Some(ClipSpec { source: g.name.clone(), rules: g.rules(part)? }))
            .unwrap_or_else(|| ClipSpec::named(clip))
    }
    fn rig_hints(&self, path: &str) -> RigHints {
        self.skeletons.get(path).map(|s| s.hints.clone()).unwrap_or_default()
    }
    fn rig(&self, path: &str) -> Option<RetargetRig> {
        self.skeletons.get(path).and_then(|s| s.retarget.clone())
    }
    /// Library sources without a root bone (`root_from_pelvis`).
    fn retarget_root(&self, file: &str, clip: &str, _skeleton: &str) -> Option<RootMotionRules> {
        self.library.get(clip).filter(|r| r.file == file).and_then(|r| r.retarget_root.clone())
    }
}

impl GraphRules for MwAnimRules {
    /// Morrowind references cook on the actor's skeleton; library
    /// references (`source:clip`) cook on the library's skeleton and
    /// retarget onto the actor's.
    fn clip_product(&self, _graph: &str, skeleton: &str, reference: &str) -> Option<ProductKey> {
        if self.library.contains_key(reference) {
            return self.library_product(skeleton, reference);
        }
        let (file, _) = self.resolve(skeleton, reference)?;
        Some(ProductKey::new(ANIM_COOK, anim_subject(skeleton, file, reference)))
    }
    /// Every file of the set and its candidates: a group added to a
    /// higher-priority file (or a new `x<model>.kf`) recompiles the graph.
    /// A library reference needs its library file: dropping the file into
    /// the folder recompiles the graphs that asked for it.
    fn clip_files(&self, _graph: &str, skeleton: &str, reference: &str) -> Vec<String> {
        if let Some(r) = self.library.get(reference) {
            return vec![r.file.clone()];
        }
        self.skeletons.get(skeleton).map(|s| s.watch.clone()).unwrap_or_default()
    }
    /// Library clips carry their source and license; Morrowind's own clips
    /// keep the default.
    fn provenance(&self, _graph: &str, reference: &str, _product: &ProductKey) -> Provenance {
        self.library.get(reference).map(|r| r.provenance.clone()).unwrap_or_default()
    }
    fn rig(&self, skeleton: &str) -> GraphRig {
        self.skeletons.get(skeleton).map(|s| s.rig.clone()).unwrap_or_default()
    }
    /// 2: library references. A present library file that can't serve a
    /// reference changes the answer without changing any recorded input
    /// (the reference no longer resolves, so its file isn't checked), so
    /// those references are folded in: fixing the file recompiles.
    fn version(&self) -> u32 {
        let refused: Vec<&str> = self.library.iter().filter(|(_, r)| r.usable == Some(false)).map(|(k, _)| k.as_str()).collect();
        if refused.is_empty() {
            return GRAPH_RULES_VERSION;
        }
        let d = Digest::of(refused.join("\n").as_bytes());
        GRAPH_RULES_VERSION ^ (u32::from_le_bytes([d.0[0], d.0[1], d.0[2], d.0[3]]) | 1 << 31)
    }
}

// ---------------------------------------------------------------------------
// The bake.

/// What [`bake_animation`] did.
#[derive(Clone, Debug, Default)]
pub struct AnimBakeReport {
    pub sets: usize,
    pub files: usize,
    /// Whole-group `anim.cook` products requested (sections and loop tails
    /// cook on demand while graphs compile).
    pub group_clips: usize,
    pub graphs: usize,
    /// Phase 1 (skeletons, group tables) and phase 2 (clips, graphs).
    pub phase1: Duration,
    pub phase2: Duration,
    pub failures: Vec<(ProductKey, String)>,
    /// Library clips retargeted (`anim.retarget` products requested).
    pub library_clips: usize,
    /// Per manifest slot: where it came from or why it fell back
    /// (`traverse.mantle.waist: quaternius_ual1:ClimbLedge_RM`).
    pub slots: Vec<String>,
}

/// Per manifest slot, its first usable candidate's `anim.retarget` keys onto
/// every set whose graphs include the slot's template (as the graph compile
/// will pick it), and the report line: the reference, or the fallback with
/// why each candidate was passed over.
fn library_keys(rules: &MwAnimRules, sets: &ActorSets, sources: &AnimSources, refused: &[(String, String)], keys: &mut Vec<ProductKey>) -> Vec<String> {
    let mut lines = Vec::new();
    for (name, slot) in &sources.manifest.slots {
        let graph = mw_graphs::Template::ALL.into_iter().find(|t| t.sources_name() == slot.graph).map(mw_graphs::Template::path);
        let mut why = Vec::new();
        let mut from = None;
        for c in &slot.candidates {
            let reference = c.reference();
            let Some(r) = rules.library.get(&reference) else { continue };
            match r.usable {
                Some(true) => {
                    for set in sets.sets.values().filter(|s| graph.is_some_and(|g| s.graphs.contains(g))) {
                        keys.extend(rules.library_product(&set.skeleton, &reference));
                    }
                    from = Some(reference);
                    break;
                }
                Some(false) => {
                    let reason = refused.iter().find(|(x, _)| *x == reference).map_or("unusable", |(_, w)| w.as_str());
                    why.push(format!("{reference}: {reason}"));
                }
                None => why.push(format!("{reference}: {} not in the library folder", r.file)),
            }
        }
        lines.push(match from {
            Some(r) => format!("{name}: {r}"),
            None => format!("{name}: fallback {:?} ({})", slot.fallback, why.join("; ")),
        });
    }
    lines
}

fn product_bytes(baker: &Baker, key: &ProductKey) -> Result<Vec<u8>, String> {
    let digest = baker.product(key).map_err(|e| e.to_string())?;
    baker.store().get(&digest).map_err(|e| e.to_string())
}

/// Registers the animation steps on `baker` and bakes every set: phase 1
/// `skeleton.cook` and `mw.anim.groups` (and, for each library file of
/// `sources` that is present, its skeleton and clip names); phase 2 every
/// layered group's `anim.cook`, every usable library clip's `anim.retarget`
/// onto the humanoid skeletons whose graph has its slot, and every
/// `(graph, skeleton)` pair's `animgraph.compile` with [`MwAnimRules`].
/// Leaves the phase-2 steps registered. The baker's inputs must serve the
/// library paths ([`crate::VfsInputs::with_anim_sources`]).
pub fn bake_animation(baker: &mut Baker, sets: &ActorSets, sources: &AnimSources) -> AnimBakeReport {
    let start = Instant::now();
    let mut report = AnimBakeReport { sets: sets.sets.len(), ..AnimBakeReport::default() };
    let files = sets.files();
    report.files = files.len();
    let library_files: BTreeSet<String> =
        sources.references().into_iter().map(|(file, _)| file).filter(|f| sources.real_path(f).is_some()).collect();

    // Phase 1: the skeleton rules don't depend on phase-1 products, so the
    // skeleton keys equal phase 2's.
    let phase1 = AnimBake::new(anim_importers(), Arc::new(MwAnimRules::default()), CookSettings::default());
    phase1.register(baker);
    baker.register(AnimGroupsStep);
    baker.register(SourceClipsStep);
    let mut keys: Vec<ProductKey> = files.iter().map(|f| ProductKey::new(ANIM_GROUPS, *f)).collect();
    keys.extend(sets.sets.keys().map(|s| ProductKey::new(SKELETON_COOK, s.as_str())));
    for file in &library_files {
        keys.extend([ProductKey::new(SKELETON_COOK, file.as_str()), ProductKey::new(SOURCE_CLIPS, file.as_str())]);
    }
    let first = baker.bake(&keys);
    phase1.clear_imports();
    report.failures.extend(first.failures().map(|(k, e)| (k.clone(), e.to_string())));

    let mut rules = MwAnimRules::default();
    for file in &files {
        let key = ProductKey::new(ANIM_GROUPS, *file);
        if let Ok(groups) = product_bytes(baker, &key).and_then(|b| AnimGroupsStep::decode(&b).map_err(|e| e.to_string())) {
            rules.add_groups(file, groups);
        }
    }
    for set in sets.sets.values() {
        let key = ProductKey::new(SKELETON_COOK, set.skeleton.as_str());
        if let Ok(skeleton) = product_bytes(baker, &key).and_then(|b| Skeleton::from_bytes(&b).map_err(|e| e.to_string())) {
            rules.add_skeleton(set, &skeleton);
        }
    }
    rules.add_library(sources);
    let mut refused = Vec::new();
    for file in &library_files {
        let skeleton = product_bytes(baker, &ProductKey::new(SKELETON_COOK, file.as_str())).ok().and_then(|b| Skeleton::from_bytes(&b).ok());
        let clips = product_bytes(baker, &ProductKey::new(SOURCE_CLIPS, file.as_str()))
            .ok()
            .and_then(|b| SourceClipsStep::decode(&b).ok())
            .unwrap_or_default();
        refused.extend(rules.add_library_file(file, skeleton.as_ref(), &clips));
    }
    report.phase1 = start.elapsed();

    // Phase 2.
    let rules = Arc::new(rules);
    let bake = AnimBake::new(anim_importers(), rules.clone(), CookSettings::default());
    bake.register(baker);
    AnimGraphCompileStep::register(rules.clone(), baker);
    let mut keys = Vec::new();
    for set in sets.sets.values().filter(|s| rules.skeletons.contains_key(&s.skeleton)) {
        let groups = rules.set_groups(&set.skeleton);
        report.group_clips += groups.len();
        keys.extend(groups.into_iter().map(|(file, group)| ProductKey::new(ANIM_COOK, anim_subject(&set.skeleton, file, group))));
    }
    report.slots = library_keys(&rules, sets, sources, &refused, &mut keys);
    report.library_clips = keys.iter().filter(|k| k.step == ANIM_RETARGET).count();
    let graphs: Vec<ProductKey> =
        sets.graph_pairs().filter(|(_, s)| rules.skeletons.contains_key(*s)).map(|(g, s)| ProductKey::new(ANIMGRAPH_COMPILE, graph_subject(g, s))).collect();
    report.graphs = graphs.len();
    // Clips first: a graph reads its clips at run time (not planned), so
    // baked together a graph would cook unfinished clips inline, one by one,
    // and race the scheduler for them (StepStats::duplicates).
    let clips = baker.bake(&keys);
    let compiled = baker.bake(&graphs);
    bake.clear_imports();
    for stage in [&clips, &compiled] {
        report.failures.extend(stage.failures().map(|(k, e)| (k.clone(), e.to_string())));
    }
    report.phase2 = start.elapsed().saturating_sub(report.phase1);
    report
}

// ---------------------------------------------------------------------------
// Loading (Bevy-free; `sf_anim_bevy` wraps it).

/// A load failure: the product key (or path) and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimLoadError(pub String);

impl fmt::Display for AnimLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for AnimLoadError {}

/// One actor's animation, ready to bind: the skeleton, its rig profile,
/// the compiled graph and its clips in clip-slot order.
#[derive(Clone, Debug)]
pub struct ActorAnimation {
    pub skeleton: Arc<Skeleton>,
    pub profile: Option<RigProfile>,
    pub graph: Arc<CompiledGraph>,
    /// `graph.clips[i]` → `clips[i]` (an `sf_anim_graph::Binding`'s clips).
    pub clips: Vec<Arc<Clip>>,
}

/// The baked animation of a profile: reads products through the manifest.
/// Whole-group clips are indexed per skeleton for `PlayGroup` / `LoopGroup`
/// (any group of the set, not only those the graph names).
pub struct AnimLibrary {
    store: Store,
    manifest: Manifest,
    /// Skeleton → lowercase group name → `anim.cook` key.
    groups: HashMap<String, BTreeMap<String, ProductKey>>,
}

impl AnimLibrary {
    pub fn new(store: Store, manifest: Manifest) -> Self {
        let mut groups: HashMap<String, BTreeMap<String, ProductKey>> = HashMap::new();
        for key in manifest.products.keys().filter(|k| k.step == ANIM_COOK) {
            if let Some((skeleton, _, clip)) = parse_anim_subject(&key.subject)
                && !clip.contains('@')
            {
                groups.entry(skeleton.to_owned()).or_default().insert(clip.to_ascii_lowercase(), key.clone());
            }
        }
        Self { store, manifest, groups }
    }

    fn digest(&self, key: &ProductKey) -> Result<Digest, AnimLoadError> {
        self.manifest.products.get(key).copied().ok_or_else(|| AnimLoadError(format!("{key:?} is not baked")))
    }

    fn bytes(&self, key: &ProductKey) -> Result<Vec<u8>, AnimLoadError> {
        self.store.get(&self.digest(key)?).map_err(|e| AnimLoadError(format!("{key:?}: {e}")))
    }

    pub fn skeleton(&self, skeleton: &str) -> Result<Skeleton, AnimLoadError> {
        let key = ProductKey::new(SKELETON_COOK, skeleton);
        Skeleton::from_bytes(&self.bytes(&key)?).map_err(|e| AnimLoadError(format!("{key:?}: {e}")))
    }

    /// The rig profile, when it was baked (every clip cook bakes it).
    pub fn profile(&self, skeleton: &str) -> Result<Option<RigProfile>, AnimLoadError> {
        let key = ProductKey::new(RIG_PROFILE, skeleton);
        if self.digest(&key).is_err() {
            return Ok(None);
        }
        RigProfile::from_bytes(&self.bytes(&key)?).map(Some).map_err(|e| AnimLoadError(format!("{key:?}: {e}")))
    }

    pub fn graph(&self, graph: &str, skeleton: &str) -> Result<CompiledGraph, AnimLoadError> {
        let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject(graph, skeleton));
        CompiledGraph::from_bytes(&self.bytes(&key)?).map_err(|e| AnimLoadError(format!("{key:?}: {e}")))
    }

    pub fn clip(&self, key: &ProductKey) -> Result<Clip, AnimLoadError> {
        Clip::from_bytes(&self.bytes(key)?).map_err(|e| AnimLoadError(format!("{key:?}: {e}")))
    }

    /// The whole-group clips of a skeleton: lowercase group name → key.
    pub fn groups(&self, skeleton: &str) -> Option<&BTreeMap<String, ProductKey>> {
        self.groups.get(skeleton)
    }

    /// A group's clip on a skeleton (case-insensitive), for scripted play.
    pub fn group_clip(&self, skeleton: &str, group: &str) -> Result<Clip, AnimLoadError> {
        let key = self
            .groups(skeleton)
            .and_then(|g| g.get(&group.to_ascii_lowercase()))
            .ok_or_else(|| AnimLoadError(format!("{skeleton}: no group {group:?}")))?;
        self.clip(key)
    }

    /// The graphs baked for a skeleton (ASCII case-insensitive), as
    /// `(graph, skeleton)` subjects for [`Self::load`].
    pub fn graphs(&self, skeleton: &str) -> Vec<(String, String)> {
        self.manifest
            .products
            .keys()
            .filter(|k| k.step == ANIMGRAPH_COMPILE)
            .filter_map(|k| sf_anim_graph::steps::parse_graph_subject(&k.subject))
            .filter(|(_, s)| s.eq_ignore_ascii_case(skeleton))
            .map(|(g, s)| (g.to_owned(), s.to_owned()))
            .collect()
    }

    /// One graph on one skeleton, with its clips.
    pub fn load(&self, skeleton: &str, graph: &str) -> Result<ActorAnimation, AnimLoadError> {
        let compiled = self.graph(graph, skeleton)?;
        let clips = compiled.clips.iter().map(|slot| self.clip(&slot.product).map(Arc::new)).collect::<Result<_, _>>()?;
        Ok(ActorAnimation {
            skeleton: Arc::new(self.skeleton(skeleton)?),
            profile: self.profile(skeleton)?,
            graph: Arc::new(compiled),
            clips,
        })
    }

    /// An actor recipe's third-person animation and, when it has one, its
    /// first-person view model's.
    pub fn load_recipe(&self, recipe: &CharacterRecipe) -> Result<(ActorAnimation, Option<ActorAnimation>), AnimLoadError> {
        let body = self.load(recipe.skeleton.as_str(), recipe.graph.as_str())?;
        let first = recipe.first_person.as_ref().map(|fp| self.load(fp.skeleton.as_str(), fp.graph.as_str())).transpose()?;
        Ok((body, first))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group() -> GroupInfo {
        GroupInfo {
            name: "WeaponOneHand".into(),
            duration: 3.0,
            looping: Looping::Once,
            loop_range: Some((0.5, 1.0)),
            sections: vec![("chop".into(), 0.2, 0.8)],
        }
    }

    #[test]
    fn group_rules_cut_sections_and_tails() {
        let g = group();
        assert_eq!(g.rules(None).unwrap().range, None);
        assert_eq!(g.rules(Some("chop")).unwrap().range, Some((0.2, 0.8)));
        assert_eq!(g.rules(Some("tail")).unwrap().range, Some((1.0, 3.0)));
        assert!(g.rules(Some("slash")).is_none());
    }

    #[test]
    fn references_resolve_through_layers() {
        let mut rules = MwAnimRules::default();
        rules.add_groups("hi.kf", vec![GroupInfo { name: "Idle".into(), sections: Vec::new(), ..group() }]);
        rules.add_groups("lo.kf", vec![group(), GroupInfo { name: "idle".into(), ..group() }]);
        let set = ActorAnimSet { skeleton: "s.nif".into(), files: vec!["hi.kf".into(), "lo.kf".into()], ..ActorAnimSet::default() };
        rules.skeletons.insert(set.skeleton.clone(), SkeletonData { files: set.files.clone(), ..SkeletonData::default() });
        assert_eq!(rules.resolve("s.nif", "IDLE").unwrap().0, "hi.kf");
        assert_eq!(rules.resolve("s.nif", "WeaponOneHand@chop").unwrap().0, "lo.kf");
        // The winning layer lacks the section: missing (the graph's fallback applies).
        assert!(rules.resolve("s.nif", "Idle@chop").is_none());
        assert_eq!(rules.set_groups("s.nif"), vec![("hi.kf", "Idle"), ("lo.kf", "WeaponOneHand")]);
        let key = GraphRules::clip_product(&rules, "g", "s.nif", "WeaponOneHand@chop").unwrap();
        assert_eq!(key.subject, "s.nif|lo.kf|WeaponOneHand@chop");
        assert_eq!(AnimRules::clip(&rules, "lo.kf", "WeaponOneHand@chop", "s.nif").rules.range, Some((0.2, 0.8)));
    }

    #[test]
    fn library_references_retarget_and_drop() {
        const R: &str = "quaternius_ual1:ClimbLedge_RM";
        const FILE: &str = "animsrc/quaternius_ual1/AnimationLibrary_Godot.glb";
        let mut rules = MwAnimRules::default();
        rules.add_library(&AnimSources::none());
        assert!(rules.library.is_empty());
        rules.add_library(&AnimSources::new(mw_graphs::sources::SourcesManifest::builtin().clone(), ""));
        rules.skeletons.insert("t.nif".into(), SkeletonData { retarget: Some(RetargetRig::default()), ..SkeletonData::default() });
        rules.skeletons.insert("creature.nif".into(), SkeletonData::default());

        // File not seen yet: the product is named, the compile checks the file.
        let key = GraphRules::clip_product(&rules, "g", "t.nif", R).unwrap();
        assert_eq!((key.step.as_str(), key.subject.as_str()), (ANIM_RETARGET, format!("t.nif|{FILE}|{FILE}|{R}").as_str()));
        assert!(GraphRules::clip_product(&rules, "g", "creature.nif", R).is_none(), "no humanoid rig");
        assert_eq!(GraphRules::clip_files(&rules, "g", "t.nif", R), [FILE]);
        let p = GraphRules::provenance(&rules, "g", R, &key);
        assert_eq!((p.source.as_str(), p.license.as_str(), p.redistributable), ("quaternius_ual1", "CC0-1.0", true));
        let spec = AnimRules::clip(&rules, FILE, R, FILE);
        assert_eq!(spec.source, "ClimbLedge_RM");
        assert_eq!(spec.rules.root_motion.mode, sf_anim::RootMotionMode::Progress);
        assert_eq!(GraphRules::version(&rules), GRAPH_RULES_VERSION);

        // Present without the clip (or a rig): unresolvable, and the rules' version moves.
        let refused = rules.add_library_file(FILE, None, &["Idle_Loop".to_owned()]);
        assert_eq!(refused.len(), 1);
        assert!(GraphRules::clip_product(&rules, "g", "t.nif", R).is_none());
        assert_ne!(GraphRules::version(&rules), GRAPH_RULES_VERSION);
    }
}
