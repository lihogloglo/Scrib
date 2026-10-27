//! Bake steps (animation.md "Bake steps"): `skeleton.cook`, `rig.profile`, `anim.cook`
//! and `anim.retarget`, sharing one [`AnimBake`] (importers, adapter
//! rules, cook settings), as `sf_terrain`'s steps share `TerrainBake`.
//!
//! Subjects:
//! - `skeleton.cook`: the skeleton source file (a model or animation file
//!   with a skeleton), e.g. `meshes/hero.glb`.
//! - `rig.profile`: the skeleton source file, as `skeleton.cook`.
//! - `anim.cook`: [`anim_subject`] — `<skeleton>|<clip file>|<clip>`. The
//!   clip name comes last so it may contain anything.
//! - `anim.retarget`: [`retarget_subject`] — `<target skeleton>|<anim.cook
//!   subject>`.
//!
//! Adapter data (skeleton rules, clip rules, rigs) is not a file, so it
//! enters the bake key as parameters: the postcard bytes of what the
//! adapter returned.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, Digest, PlanCx, ProductKey, RunCx, Step};

use crate::{
    AnimImporter, AnimSource, CLIP_FORMAT, Clip, ClipRules, CookSettings, RIG_PROFILE_FORMAT, RetargetRig, Retargeter, RigHints, RigProfile,
    RootMotionRules, SKELETON_FORMAT, Skeleton, SkeletonRules, cook_clip_for, detect_profile,
};

pub const SKELETON_COOK: &str = "skeleton.cook";
pub const ANIM_COOK: &str = "anim.cook";
pub const ANIM_RETARGET: &str = "anim.retarget";
pub const RIG_PROFILE: &str = "rig.profile";

/// The registered animation importers, chosen by file extension.
#[derive(Clone, Default)]
pub struct AnimImporters(Vec<Arc<dyn AnimImporter>>);

impl AnimImporters {
    pub fn new() -> Self {
        Self::default()
    }

    /// The framework's importers: glTF, BVH (centimeters, Y up; adapters
    /// add a configured [`crate::BvhAnimImporter`] for other spaces) and
    /// FBX, each with its feature.
    pub fn framework() -> Self {
        Self(vec![
            #[cfg(feature = "gltf")]
            Arc::new(crate::GltfAnimImporter),
            #[cfg(feature = "bvh")]
            Arc::new(crate::BvhAnimImporter::default()),
            #[cfg(feature = "fbx")]
            Arc::new(crate::FbxAnimImporter),
        ])
    }

    /// Adds an importer; later ones win for the same extension.
    #[must_use]
    pub fn with(mut self, importer: Arc<dyn AnimImporter>) -> Self {
        self.0.push(importer);
        self
    }

    pub fn for_path(&self, path: &str) -> Option<&Arc<dyn AnimImporter>> {
        let ext = path.rsplit_once('.')?.1.to_ascii_lowercase();
        self.0.iter().rev().find(|i| i.extensions().contains(&ext.as_str()))
    }

    fn plan(&self, path: &str, cx: &mut PlanCx) {
        cx.file(path);
        let importer = self.for_path(path);
        let mut key = importer.map_or(0, |i| i.version()).to_le_bytes().to_vec();
        // Settings only when present, so settings-free importers keep their keys.
        key.extend(importer.map(|i| i.settings_key(path)).unwrap_or_default());
        cx.param(format!("importer:{path}"), key);
    }

    fn importer(&self, key: &ProductKey, path: &str) -> Result<&Arc<dyn AnimImporter>, BakeError> {
        self.for_path(path).ok_or_else(|| BakeError::step(key, format!("no animation importer for {path:?}")))
    }
}

/// Which source clip an `anim.cook` product comes from, and how to cook it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClipSpec {
    /// The clip's name in the source file.
    pub source: String,
    pub rules: ClipRules,
}

impl ClipSpec {
    /// The source clip of that name, default rules.
    pub fn named(name: &str) -> Self {
        Self { source: name.to_owned(), rules: ClipRules::default() }
    }
}

/// Adapter rig and clip data. Must be deterministic: what it returns is
/// part of the bake key.
pub trait AnimRules: Send + Sync {
    /// Rules for the skeleton in `path`.
    fn skeleton(&self, _path: &str) -> SkeletonRules {
        SkeletonRules::default()
    }
    /// How clip `clip` of `file` is cooked for the skeleton from
    /// `skeleton`. Adapters splitting one long take return the take as
    /// `source` and the part as `rules.range`.
    fn clip(&self, _file: &str, clip: &str, _skeleton: &str) -> ClipSpec {
        ClipSpec::named(clip)
    }
    /// Rig-profile detection hints for the skeleton from `path` (a reviewed
    /// `*.rigprofile.ron` override goes in [`RigHints::profile`]).
    fn rig_hints(&self, _path: &str) -> RigHints {
        RigHints::default()
    }
    /// The retarget rig of the skeleton from `path`.
    fn rig(&self, _path: &str) -> Option<RetargetRig> {
        None
    }
    /// Root motion generated on the target after retargeting (UE 5.4's
    /// "root motion from target pelvis"; retarget.rs step 4) for clip `clip`
    /// of `file` cooked on `skeleton`, for sources with no usable root bone
    /// (mocap takes). `Some(rules)`: `anim.cook` keeps the motion in the pose
    /// (no extraction, no end anchor) and `anim.retarget` re-bases the target
    /// hips to the origin facing +Z at frame 0, then extracts with `rules`.
    /// `None` (default): the source cook's root motion carries over. Enters
    /// the bake keys only when `Some`.
    fn retarget_root(&self, _file: &str, _clip: &str, _skeleton: &str) -> Option<RootMotionRules> {
        None
    }
}

/// Framework defaults: guessed tags, default LODs, whole clips, no rigs.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultRules;

impl AnimRules for DefaultRules {}

/// One import, shared by every step run that reads the same file bytes.
type Imported = Arc<OnceLock<Result<Arc<AnimSource>, String>>>;

/// Shared state of the animation steps.
pub struct AnimBake {
    pub importers: AnimImporters,
    pub rules: Arc<dyn AnimRules>,
    pub settings: CookSettings,
    /// Per-session import memo, by (path, content digest): a source file
    /// with many clips (one take per file is rare: legacy and library
    /// files hold dozens to hundreds) is parsed and converted once, not
    /// once per `anim.cook`. Concurrent cooks of the same file wait for the
    /// one import. Holds every imported source until [`Self::clear_imports`].
    ///
    /// Blocking here is safe only because the initializer (an importer's
    /// `import`) is plain sequential code: no rayon, no baker calls. A rayon
    /// worker waiting in a `join` can steal another job, so an initializer
    /// that parallelized could end up beneath a job waiting on its own cell
    /// and deadlock (bake.md "Concurrency"). Keep importers sequential.
    imports: Mutex<HashMap<(String, Digest), Imported>>,
}

impl AnimBake {
    pub fn new(importers: AnimImporters, rules: Arc<dyn AnimRules>, settings: CookSettings) -> Arc<Self> {
        Arc::new(Self { importers, rules, settings, imports: Mutex::default() })
    }

    /// Drops the import memo (call after a bake phase to free the sources).
    pub fn clear_imports(&self) {
        self.imports.lock().unwrap_or_else(PoisonError::into_inner).clear();
    }

    /// Imports `path` through its importer, once per content per session.
    /// The file read is still recorded as an input of the calling step.
    fn import(&self, key: &ProductKey, path: &str, cx: &mut RunCx<'_>) -> Result<Arc<AnimSource>, BakeError> {
        let importer = self.importers.importer(key, path)?;
        let bytes = cx.read_file(path)?;
        let companion = match importer.companion(path) {
            Some(c) if cx.file_exists(&c)? => Some(cx.read_file(&c)?),
            _ => None,
        };
        // The memo key covers the companion too (its content changes the source).
        let digest = companion.as_ref().map_or_else(|| Digest::of(&bytes), |c| Digest::of(&[bytes.as_slice(), c.as_slice()].concat()));
        let slot = {
            let mut imports = self.imports.lock().unwrap_or_else(PoisonError::into_inner);
            imports.entry((path.to_owned(), digest)).or_default().clone()
        };
        slot.get_or_init(|| importer.import_with(path, &bytes, companion.as_deref()).map(Arc::new).map_err(|e| e.to_string()))
            .clone()
            .map_err(|e| BakeError::step(key, e))
    }

    /// Registers every animation step.
    pub fn register(self: &Arc<Self>, baker: &mut Baker) {
        baker
            .register(SkeletonCookStep(self.clone()))
            .register(RigProfileStep(self.clone()))
            .register(AnimCookStep(self.clone()))
            .register(AnimRetargetStep(self.clone()));
    }
}

/// The `anim.cook` subject for clip `clip` of `file` on the skeleton from `skeleton`.
pub fn anim_subject(skeleton: &str, file: &str, clip: &str) -> String {
    format!("{skeleton}|{file}|{clip}")
}

/// `(skeleton, file, clip)` of an `anim.cook` subject.
pub fn parse_anim_subject(subject: &str) -> Option<(&str, &str, &str)> {
    let mut it = subject.splitn(3, '|');
    Some((it.next()?, it.next()?, it.next()?))
}

/// The `anim.retarget` subject: an `anim.cook` product on another skeleton.
pub fn retarget_subject(target_skeleton: &str, anim_subject: &str) -> String {
    format!("{target_skeleton}|{anim_subject}")
}

/// `(target skeleton, anim.cook subject)` of an `anim.retarget` subject.
pub fn parse_retarget_subject(subject: &str) -> Option<(&str, &str)> {
    let (target, anim) = subject.split_once('|')?;
    parse_anim_subject(anim)?;
    Some((target, anim))
}

fn bytes_of<T: Serialize>(value: &T) -> Vec<u8> {
    postcard::to_stdvec(value).expect("in-memory serialization")
}

/// Reads a `skeleton.cook` product.
fn read_skeleton(key: &ProductKey, path: &str, cx: &mut RunCx<'_>) -> Result<Skeleton, BakeError> {
    let bytes = cx.product(SKELETON_COOK, path)?;
    Skeleton::from_bytes(&bytes).map_err(|e| BakeError::step(key, format!("skeleton {path:?}: {e}")))
}

/// `skeleton.cook`: the skeleton of a source file, with tags, LOD masks and
/// the root-motion joint from the adapter's [`SkeletonRules`].
pub struct SkeletonCookStep(pub Arc<AnimBake>);

impl Step for SkeletonCookStep {
    fn id(&self) -> &'static str {
        SKELETON_COOK
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.0.importers.plan(subject, cx);
        cx.param("rules", bytes_of(&self.0.rules.skeleton(subject)));
        cx.param("format", SKELETON_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(SKELETON_COOK, subject);
        let source = self.0.import(&key, subject, cx)?;
        let joints = source.skeleton.clone().ok_or_else(|| BakeError::step(&key, "the file has no skeleton"))?;
        let mut rules = self.0.rules.skeleton(subject);
        if rules.root_motion_joint.is_none() && rules.detect_root_motion {
            rules.root_motion_joint.clone_from(&source.root_motion);
        }
        let (skeleton, _) = Skeleton::with_rules(&joints, &rules).map_err(|e| BakeError::step(&key, e))?;
        Ok(skeleton.to_bytes())
    }
}

/// Reads a `rig.profile` product.
fn read_profile(key: &ProductKey, path: &str, cx: &mut RunCx<'_>) -> Result<RigProfile, BakeError> {
    let bytes = cx.product(RIG_PROFILE, path)?;
    RigProfile::from_bytes(&bytes).map_err(|e| BakeError::step(key, format!("rig profile {path:?}: {e}")))
}

/// `rig.profile`: the skeleton's [`RigProfile`], detected from its rest pose
/// and tags with the adapter's [`RigHints`] (rig-profile.md). Its own
/// product, so a profile tweak doesn't re-cook skeletons.
pub struct RigProfileStep(pub Arc<AnimBake>);

impl Step for RigProfileStep {
    fn id(&self) -> &'static str {
        RIG_PROFILE
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(SKELETON_COOK, subject);
        cx.param("hints", self.0.rules.rig_hints(subject).key_bytes());
        cx.param("format", RIG_PROFILE_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(RIG_PROFILE, subject);
        let skeleton = read_skeleton(&key, subject, cx)?;
        let profile = detect_profile(&skeleton, &self.0.rules.rig_hints(subject)).map_err(|e| BakeError::step(&key, e))?;
        Ok(profile.to_bytes())
    }
}

/// `anim.cook`: one source clip cooked for one skeleton. Contacts default
/// to the skeleton's support limbs from its `rig.profile`.
pub struct AnimCookStep(pub Arc<AnimBake>);

impl AnimCookStep {
    fn parse(subject: &str) -> Result<(&str, &str, &str), BakeError> {
        parse_anim_subject(subject).ok_or_else(|| BakeError::step(&ProductKey::new(ANIM_COOK, subject), "bad subject"))
    }
}

impl Step for AnimCookStep {
    fn id(&self) -> &'static str {
        ANIM_COOK
    }
    fn version(&self) -> u32 {
        // 3: rate search skips rates whose resampling error can't pass, guessed bit search (animcook).
        // 4: contacts from height and speed over the ground with hysteresis, stance ends at heel-off (walkers3/4).
        // 5: `ClipMeta::stance_velocity` (toe-aware planted-foot speed of travelling loops).
        // 6: `roll.<foot>` heel-roll curves, heel-off to toe-off (AN-9).
        // 7: contact gaps up to `ContactRules::gap` at a loop's seam closed (AN-17).
        // 8: contact-consistent root track of travelling loops, `ContactRules::root_blend` (AN-19).
        // 9: horizontal extraction starts the root-motion joint over its rest ground position
        //    (`cook::start_anchor`: split-take clips no longer draw the body off its capsule).
        9
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (skeleton, file, clip) = Self::parse(subject)?;
        self.0.importers.plan(file, cx);
        cx.product(SKELETON_COOK, skeleton).product(RIG_PROFILE, skeleton);
        cx.param("rules", bytes_of(&self.0.rules.clip(file, clip, skeleton)));
        if let Some(root) = self.0.rules.retarget_root(file, clip, skeleton) {
            cx.param("retarget_root", bytes_of(&root));
        }
        cx.param("settings", self.0.settings.key_bytes());
        cx.param("format", CLIP_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(ANIM_COOK, subject);
        let (skeleton_path, file, clip) = Self::parse(subject)?;
        let skeleton = read_skeleton(&key, skeleton_path, cx)?;
        let profile = read_profile(&key, skeleton_path, cx)?;
        let source = self.0.import(&key, file, cx)?;
        let mut spec = self.0.rules.clip(file, clip, skeleton_path);
        if self.0.rules.retarget_root(file, clip, skeleton_path).is_some() {
            // The motion stays in the pose: `anim.retarget` extracts it on the target.
            spec.rules.root_motion.extract = false;
            spec.rules.root_motion.anchor_end = false;
        }
        let src = source
            .clips
            .iter()
            .find(|c| c.name == spec.source)
            .ok_or_else(|| BakeError::step(&key, format!("{file:?} has no clip {:?}", spec.source)))?;
        let mut cooked = cook_clip_for(&skeleton, Some(&profile), src, &spec.rules, &self.0.settings).map_err(|e| BakeError::step(&key, e))?;
        clip.clone_into(&mut cooked.name);
        Ok(cooked.to_bytes())
    }
}

/// `anim.retarget`: a cooked clip carried onto another skeleton through
/// both skeletons' [`RetargetRig`]s, then cooked for it.
pub struct AnimRetargetStep(pub Arc<AnimBake>);

impl AnimRetargetStep {
    fn parse(subject: &str) -> Result<(&str, &str, &str), BakeError> {
        let bad = || BakeError::step(&ProductKey::new(ANIM_RETARGET, subject), "bad subject");
        let (target, anim) = parse_retarget_subject(subject).ok_or_else(bad)?;
        let (source, _, _) = parse_anim_subject(anim).ok_or_else(bad)?;
        Ok((target, anim, source))
    }

    fn rig(&self, key: &ProductKey, path: &str) -> Result<RetargetRig, BakeError> {
        self.0.rules.rig(path).ok_or_else(|| BakeError::step(key, format!("skeleton {path:?} has no retarget rig")))
    }

    /// [`AnimRules::retarget_root`] of the `anim.cook` subject `anim`.
    fn root(&self, anim: &str) -> Option<RootMotionRules> {
        let (skeleton, file, clip) = parse_anim_subject(anim)?;
        self.0.rules.retarget_root(file, clip, skeleton)
    }
}

impl Step for AnimRetargetStep {
    fn id(&self) -> &'static str {
        ANIM_RETARGET
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (target, anim, source) = Self::parse(subject)?;
        cx.product(ANIM_COOK, anim).product(SKELETON_COOK, source).product(SKELETON_COOK, target);
        cx.param("source_rig", bytes_of(&self.0.rules.rig(source)));
        cx.param("target_rig", bytes_of(&self.0.rules.rig(target)));
        if let Some(root) = self.root(anim) {
            cx.param("retarget_root", bytes_of(&root));
        }
        cx.param("settings", self.0.settings.key_bytes());
        cx.param("format", CLIP_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(ANIM_RETARGET, subject);
        let (target_path, anim, source_path) = Self::parse(subject)?;
        let clip_bytes = cx.product(ANIM_COOK, anim)?;
        let clip = Clip::from_bytes(&clip_bytes).map_err(|e| BakeError::step(&key, format!("clip {anim:?}: {e}")))?;
        let source = read_skeleton(&key, source_path, cx)?;
        let target = read_skeleton(&key, target_path, cx)?;
        let (source_rig, target_rig) = (self.rig(&key, source_path)?, self.rig(&key, target_path)?);
        let retargeter = Retargeter::new(&source, &source_rig, &target, &target_rig).map_err(|e| BakeError::step(&key, e))?;
        let cooked = match self.root(anim) {
            Some(root) => retargeter.clip_with_root(&clip, &root, &self.0.settings),
            None => retargeter.clip(&clip, &self.0.settings),
        }
        .map_err(|e| BakeError::step(&key, e))?;
        Ok(cooked.to_bytes())
    }
}
