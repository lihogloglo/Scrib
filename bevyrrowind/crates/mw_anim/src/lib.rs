//! Morrowind keyframe animation as `sf_anim` source clips.
//!
//! Actor animation lives in `.kf` files (`xbase_anim.kf`) and in the
//! keyframe controllers of animated `.nif` files (`base_anim.nif`): one
//! timeline per file, per-bone keyframe controllers, and root text keys
//! that split the timeline into animation groups ([`text_keys`]). This
//! crate turns each group into one [`SourceClip`] (times from the group
//! start), with the group's text keys as [`sf_anim::Event`]s, and gives
//! the bake the group's loop range and sub-action sections for its
//! [`ClipRules`].
//!
//! Curves are in engine space (meters, Y up), as `mw_nif` converts them, so
//! the clips match skeletons cooked from `mw_nif` models.
//!
//! Skins are not here (`mw_bake` cooks them); geometry-morph weight curves
//! are, as clip curves (`Animation::morphs`), and so are the model's
//! visibility and material controllers ([`effects`]).

pub mod convert;
pub mod effects;
pub mod rig;
mod report;
pub mod text_keys;

use mw_nif::anim::{Curve as NifCurve, Target, TransformTrack};
use mw_nif::{Model, ModelError};
use sf_anim::{AnimImporter, AnimSource, ClipRules, ImportError, RootMotionRules, SourceClip};

pub use report::{ReportOptions, report};
pub use text_keys::{Group, Section, events, groups};

/// Bone keyframes and text keys of one animation file.
#[derive(Clone, Debug, Default)]
pub struct Animation {
    /// `(time, "Group: Event")`, one entry per line.
    pub text_keys: Vec<(f32, String)>,
    /// `(bone name, track)`; key times on the file's timeline.
    pub tracks: Vec<(String, TransformTrack)>,
    /// `("<node>#<target>", weight)`: geometry-morph weight curves of the
    /// file's meshes (creature bodies, jaws), named as `mw_bake` names the
    /// cooked morph targets; key times on the file's timeline. Each group's
    /// clip carries its cut as a float curve, so the animation graph blends
    /// morph weights with the pose and the character applies them by name.
    pub morphs: Vec<(String, NifCurve<f32>)>,
    /// Visibility and material controllers of the model, keyed on the same
    /// timeline ([`effects`]); clips carry their cut by name.
    pub effects: effects::Effects,
}

impl Animation {
    /// A `.kf` file.
    pub fn from_kf(bytes: &[u8]) -> Result<Self, ModelError> {
        let file = mw_nif::load_kf(bytes)?;
        Ok(Self { text_keys: file.text_keys, tracks: file.tracks, ..Self::default() })
    }

    /// The keyframe controllers of a `.nif` file.
    pub fn from_nif(bytes: &[u8]) -> Result<Self, ModelError> {
        Ok(Self::from_model(&mw_nif::load(bytes)?))
    }

    /// Picks the format by extension (`.kf`, else NIF).
    pub fn from_file(path: &str, bytes: &[u8]) -> Result<Self, ModelError> {
        if path.to_ascii_lowercase().ends_with(".kf") { Self::from_kf(bytes) } else { Self::from_nif(bytes) }
    }

    /// The keyframe controllers of a converted model.
    pub fn from_model(model: &Model) -> Self {
        let tracks = model
            .controllers
            .iter()
            .filter_map(|c| match &c.target {
                Target::Transform { node, track } => Some((model.nodes[*node].name.clone(), track.clone())),
                _ => None,
            })
            .collect();
        Self { text_keys: model.text_keys.clone(), tracks, morphs: morph_curves(model), effects: effects::Effects::of_model(model) }
    }

    /// The animation groups, in timeline order.
    pub fn groups(&self) -> Vec<Group> {
        groups(&self.text_keys)
    }

    /// One group as a source clip named after it.
    pub fn source_clip(&self, group: &Group) -> SourceClip {
        SourceClip {
            name: group.name.clone(),
            duration: group.duration(),
            tracks: self.tracks.iter().map(|(bone, track)| convert::track(bone, track, group.start, group.stop)).collect(),
            curves: self
                .morphs
                .iter()
                .chain(&self.effects.curves)
                .filter(|(_, c)| !c.is_empty())
                .map(|(name, c)| convert::float(name, c, group.start, group.stop))
                .chain(self.effects.steps.iter().filter(|(_, k)| !k.is_empty()).map(|(name, k)| convert::step(name, k, group.start, group.stop)))
                .collect(),
            events: group.events.clone(),
            markers: Vec::new(),
        }
    }

    /// Every group as a source clip. The root-motion joint is the animated
    /// bones root ([`rig::root_motion_joint`]): `Bip01`, the Biped root,
    /// carries the travel (checked on `xbase_anim.kf`: `WalkForward` moves
    /// `Bip01` 2.9 m, `Bip01 NonAccum` not at all), else `Root Bone` (shalk,
    /// scrib, cliff racer…).
    pub fn anim_source(&self) -> AnimSource {
        let root_motion = rig::root_motion_joint(self.tracks.iter().map(|(bone, _)| bone.as_str())).map(str::to_owned);
        AnimSource { skeleton: None, clips: self.groups().iter().map(|g| self.source_clip(g)).collect(), root_motion }
    }
}

/// The 3ds Max Biped root, which carries root motion in Morrowind animation.
pub const ROOT_MOTION_BONE: &str = "Bip01";

/// Groups whose loop tail is a get-up: authored in place (the actor stays put
/// while knocked out), cut from the middle of the take, ending standing away
/// from the take's origin (Knockout's tail: 1.07 m). Their tails cook in place,
/// anchored on the last frame (`RootMotionRules::anchor_end`): the clip ends
/// standing over its origin, where the host puts the capsule (AN-6).
pub const GET_UP_GROUPS: [&str; 2] = ["Knockout", "SwimKnockout"];

/// The root-motion rules of a group's clip (`part`: `"tail"` or a section).
pub fn root_motion_rules(group: &str, part: Option<&str>) -> RootMotionRules {
    if part == Some("tail") && GET_UP_GROUPS.iter().any(|g| g.eq_ignore_ascii_case(group)) {
        RootMotionRules { extract: false, anchor_end: true, ..RootMotionRules::default() }
    } else {
        RootMotionRules::default()
    }
}

/// The cook rules of a group's clip: its default looping (the clip is
/// already cut to the group, so no range).
pub fn clip_rules(group: &Group) -> ClipRules {
    ClipRules { looping: group.looping(), ..ClipRules::default() }
}

/// The cook rules for one section of a group's clip (`chop` of
/// `WeaponOneHand`): its range, played once, with the group's events.
pub fn section_rules(section: &Section) -> ClipRules {
    ClipRules { range: Some((section.start, section.stop)), ..ClipRules::default() }
}

/// `sf_anim` importer for `.kf` files: one source clip per animation group.
/// Animated NIFs go through [`NifAnimImporter`].
#[derive(Clone, Copy, Debug, Default)]
pub struct KfAnimImporter;

/// `sf_anim` importer for `.nif` files in the animation bake (the animation
/// importers are their own registry, apart from the model importers): the
/// actor skeleton ([`rig::skeleton_source`]) and one source clip per
/// animation group of the model's own keyframe controllers (creature
/// models, `x<model>.nif`). `AnimSource::root_motion` is the skeleton's
/// root-motion joint, so `skeleton.cook` with `detect_root_motion` and
/// guessed tags cooks exactly [`rig::skeleton`].
#[derive(Clone, Copy, Debug, Default)]
pub struct NifAnimImporter;

impl AnimImporter for NifAnimImporter {
    fn extensions(&self) -> &[&str] {
        &["nif"]
    }

    fn version(&self) -> u32 {
        // 2: TCB tangents scaled to key spacing (mw_nif).
        // 3: rigid root branches join the skeleton (`rig::skeleton_source`).
        // 4: geometry-morph weight curves as clip curves (`Animation::morphs`).
        // 5: billboard nodes join the skeleton; visibility and material curves (`effects`).
        // 6: translation snaps are curve breaks (`convert::snap_breaks`).
        // 7: particle emission windows (`effects`, `emit:` curves).
        // 8: root motion on `Root Bone` when there is no `Bip01` (`rig::root_motion_joint`).
        // 9: visibility as rest-neutral `hide:` curves (`effects::HIDE`).
        // 10: `show:` curves for nodes hidden at rest (`effects::SHOW`).
        10
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<AnimSource, ImportError> {
        let model = mw_nif::load(bytes).map_err(|e| ImportError { path: path.to_owned(), message: e.to_string() })?;
        let (joints, rules) = rig::skeleton_source(&model);
        let mut source = Animation::from_model(&model).anim_source();
        source.skeleton = Some(joints);
        source.root_motion = rules.root_motion_joint;
        Ok(source)
    }
}

impl AnimImporter for KfAnimImporter {
    fn extensions(&self) -> &[&str] {
        &["kf"]
    }

    fn version(&self) -> u32 {
        // 2: TCB tangents scaled to key spacing (mw_nif).
        // 3: geometry-morph weight curves from the companion `.nif`.
        // 4: visibility and material curves from the companion (`effects`).
        // 5: translation snaps are curve breaks (`convert::snap_breaks`).
        // 6: particle emission windows from the companion (`effects`, `emit:` curves).
        // 7: root motion on `Root Bone` when there is no `Bip01` (`rig::root_motion_joint`).
        // 8: visibility as rest-neutral `hide:` curves (`effects::HIDE`).
        // 9: `show:` curves for nodes hidden at rest (`effects::SHOW`).
        9
    }

    /// `x<model>.kf` keys the bones; the geometry-morph weights of the same
    /// timeline live in the model's controllers (`x<model>.nif`: the kwama
    /// forager's body). Clips cut both.
    fn companion(&self, path: &str) -> Option<String> {
        let stem = path.strip_suffix(".kf").or_else(|| path.strip_suffix(".KF"))?;
        Some(format!("{stem}.nif"))
    }

    fn import_with(&self, path: &str, bytes: &[u8], companion: Option<&[u8]>) -> Result<AnimSource, ImportError> {
        let mut animation = Animation::from_kf(bytes).map_err(|e| ImportError { path: path.to_owned(), message: e.to_string() })?;
        // A companion that fails to parse only loses its morph curves.
        if let Some(model) = companion.and_then(|c| mw_nif::load(c).ok()) {
            animation.morphs = morph_curves(&model);
            animation.effects = effects::Effects::of_model(&model);
        }
        Ok(animation.anim_source())
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<AnimSource, ImportError> {
        let animation = Animation::from_kf(bytes).map_err(|e| ImportError { path: path.to_owned(), message: e.to_string() })?;
        Ok(animation.anim_source())
    }
}

/// Geometry-morph weight curves of a model's meshes, named
/// `"<node>#<target>"` (targets from 1; target 0 is the base shape).
fn morph_curves(model: &Model) -> Vec<(String, NifCurve<f32>)> {
    let mut out = Vec::new();
    for node in &model.nodes {
        let Some(morph) = node.mesh.and_then(|m| model.meshes.get(m)).and_then(|m| m.morph.as_ref()) else { continue };
        for (i, target) in morph.targets.iter().enumerate() {
            out.push((format!("{}#{}", node.name, i + 1), target.weight.clone()));
        }
    }
    out
}

#[cfg(test)]
mod tests;
