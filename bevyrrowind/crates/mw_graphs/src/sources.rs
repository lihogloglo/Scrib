//! The animation-sources manifest (`docs/specs/animation-sources.md`
//! "Import profile"): third-party clip libraries the user brings, and which
//! of their clips fill which clip slot of which graph.
//!
//! The manifest is ours (GPL) and ships in `data/animation/sources.ron`;
//! the libraries never do. The graph builder turns every slot into a chain
//! of optional graph clips (`slot` → `slot.alt1` → … → the slot's
//! fallback), each sourced by a candidate reference `source:clip` (see
//! [`Candidate::reference`]); `mw_bake` resolves those references to
//! `anim.cook` + `anim.retarget` products when the library file is present.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use sf_anim::{ClipRules, Looping, RootMotionMode, RootMotionRules};
use sf_anim_graph::asset::{ClipFallback, ClipRef};

/// The shipped manifest's path under `data/`.
pub const SOURCES_PATH: &str = "animation/sources.ron";

/// The shipped manifest text.
pub const SOURCES: &str = include_str!("../data/animation/sources.ron");

/// A source's license class (animation-sources.md "Sources and licenses").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum License {
    CC0,
    CC_BY_4_0,
    CC_BY_NC_4_0,
    CC_BY_NC_ND_4_0,
    /// Adobe's Mixamo FAQ terms: no redistribution of the files.
    MixamoTerms,
    /// CMU mocap: inside a product, never as a data pack.
    CmuTerms,
}

impl License {
    /// SPDX id (or a `LicenseRef-` for non-SPDX terms).
    pub fn tag(self) -> &'static str {
        match self {
            Self::CC0 => "CC0-1.0",
            Self::CC_BY_4_0 => "CC-BY-4.0",
            Self::CC_BY_NC_4_0 => "CC-BY-NC-4.0",
            Self::CC_BY_NC_ND_4_0 => "CC-BY-NC-ND-4.0",
            Self::MixamoTerms => "LicenseRef-Mixamo",
            Self::CmuTerms => "LicenseRef-CMU-Mocap",
        }
    }

    /// Whether cooked derivatives may be shared (conservative: the NC
    /// classes and vendor terms say no until the owner decides otherwise).
    pub fn redistributable(self) -> bool {
        matches!(self, Self::CC0 | Self::CC_BY_4_0)
    }
}

/// A source's up axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    #[default]
    Y,
    Z,
}

/// One expected file of a source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFile {
    /// File name in the user's library folder.
    pub name: String,
    /// SHA-256 (hex) of the file the manifest was written against.
    pub sha256: String,
}

/// One clip library.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[serde(default)]
    pub title: String,
    pub license: License,
    /// The official page (downloads are manual).
    pub page: String,
    #[serde(default)]
    pub attribution: Option<String>,
    /// Never shared, whatever the license says.
    #[serde(default)]
    pub private_only: bool,
    pub files: Vec<SourceFile>,
    /// Source units per meter's inverse (BVH in cm: 0.01); importers with
    /// their own units (glTF, FBX) ignore it.
    #[serde(default = "one")]
    pub unit_scale: f32,
    #[serde(default)]
    pub up: Axis,
    /// The library has no root-motion bone (mocap takes: the hips are the
    /// root and the take wanders the capture volume): root motion is
    /// generated from the target pelvis after retargeting
    /// (`sf_anim::Retargeter::frames_with_root`), so every clip starts over
    /// its root facing forward.
    #[serde(default)]
    pub root_from_pelvis: bool,
    /// BVH takes whose zero pose is no usable reference (LAFAN1: every bone
    /// along one line): this frame of each take is its skeleton's rest, the
    /// pose auto-align and the retargeter start from (mocap takes open on the
    /// actor's T-pose: frame 0). `None`: the zero pose.
    #[serde(default)]
    pub rest_frame: Option<u32>,
}

fn one() -> f32 {
    1.0
}

impl Source {
    /// Whether cooked derivatives may be shared.
    pub fn redistributable(&self) -> bool {
        !self.private_only && self.license.redistributable()
    }
}

/// One candidate clip for a slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub source: String,
    /// The clip's name in the file.
    pub clip: String,
    /// The file of the source holding it (default: the source's first).
    #[serde(default)]
    pub file: Option<String>,
    /// Seconds of the source clip to keep (long takes).
    #[serde(default)]
    pub range: Option<(f32, f32)>,
}

impl Candidate {
    /// The graph clip reference: `source:clip`, `source:file:clip` when the
    /// file is named, `@start-end` appended for a range.
    pub fn reference(&self) -> String {
        let mut r = match &self.file {
            Some(f) => format!("{}:{f}:{}", self.source, self.clip),
            None => format!("{}:{}", self.source, self.clip),
        };
        if let Some((a, b)) = self.range {
            let _ = write!(r, "@{a}-{b}");
        }
        r
    }
}

/// One clip slot (action.md "Move taxonomy" names).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    /// The graph template it joins (`humanoid`, `first_person`, `creature`).
    pub graph: String,
    pub candidates: Vec<Candidate>,
    #[serde(default)]
    pub looping: Looping,
    /// How the cooked clip's root motion moves the character.
    #[serde(default = "drive")]
    pub root: RootMotionMode,
    /// Keep the root's vertical travel (climbs, mantles).
    #[serde(default)]
    pub vertical: bool,
    /// No root motion: the clip plays in place, re-based on its last frame
    /// (`RootMotionRules::anchor_end`): get-ups, which start lying wherever
    /// the body fell and end standing over the character's frame.
    #[serde(default)]
    pub in_place: bool,
    /// After the last candidate: another graph clip (a Morrowind group) or drop.
    #[serde(default = "drop")]
    pub fallback: ClipFallback,
}

fn drive() -> RootMotionMode {
    RootMotionMode::Drive
}

fn drop() -> ClipFallback {
    ClipFallback::Drop
}

impl Slot {
    /// The cook rules of one of its candidates.
    pub fn rules(&self, candidate: &Candidate) -> ClipRules {
        ClipRules {
            range: candidate.range,
            looping: self.looping,
            root_motion: RootMotionRules {
                mode: self.root,
                vertical: self.vertical,
                extract: !self.in_place,
                anchor_end: self.in_place,
                ..RootMotionRules::default()
            },
            ..ClipRules::default()
        }
    }

    /// Graph clip name of candidate `i`: the slot name, then `slot.alt<i>`.
    pub fn clip_name(slot: &str, i: usize) -> String {
        if i == 0 { slot.to_owned() } else { format!("{slot}.alt{i}") }
    }
}

/// What a candidate reference resolves to.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceRef {
    pub reference: String,
    pub source: String,
    /// File name in the library folder.
    pub file: String,
    pub clip: String,
    pub rules: ClipRules,
    /// The source's [`Source::root_from_pelvis`]: the root-motion rules
    /// applied on the target after retargeting (`rules` then cooks the
    /// source with its motion left in the pose).
    pub retarget_root: Option<RootMotionRules>,
}

/// The manifest.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SourcesManifest {
    pub sources: BTreeMap<String, Source>,
    pub slots: BTreeMap<String, Slot>,
}

impl SourcesManifest {
    /// Parses and validates: candidates name known sources and files, and
    /// one reference always means the same file, clip and cook rules.
    pub fn parse(text: &str) -> Result<Self, String> {
        let manifest: Self = ron::from_str(text).map_err(|e| format!("{SOURCES_PATH}: {e}"))?;
        manifest.references()?;
        Ok(manifest)
    }

    /// The shipped manifest.
    pub fn builtin() -> &'static Self {
        static BUILTIN: OnceLock<SourcesManifest> = OnceLock::new();
        BUILTIN.get_or_init(|| Self::parse(SOURCES).expect("the shipped sources manifest is valid"))
    }

    /// Every candidate reference, deduplicated, in slot order.
    pub fn references(&self) -> Result<Vec<SourceRef>, String> {
        let mut out: Vec<SourceRef> = Vec::new();
        for (name, slot) in &self.slots {
            for c in &slot.candidates {
                let source = self.sources.get(&c.source).ok_or_else(|| format!("slot {name}: unknown source {:?}", c.source))?;
                let file = match &c.file {
                    Some(f) => source.files.iter().find(|x| &x.name == f).ok_or_else(|| format!("slot {name}: {} has no file {f:?}", c.source))?,
                    None => source.files.first().ok_or_else(|| format!("source {}: no files", c.source))?,
                };
                let rules = slot.rules(c);
                let retarget_root = source.root_from_pelvis.then(|| rules.root_motion.clone());
                let r = SourceRef { reference: c.reference(), source: c.source.clone(), file: file.name.clone(), clip: c.clip.clone(), rules, retarget_root };
                match out.iter().find(|x| x.reference == r.reference) {
                    Some(x) if *x != r => return Err(format!("slot {name}: {} is used with other cook rules elsewhere", r.reference)),
                    Some(_) => {}
                    None => out.push(r),
                }
            }
        }
        Ok(out)
    }

    /// The optional clips a graph template gets: `(clip name, clip ref)`
    /// chains per slot, and the slot names (montage `slot_clips`).
    pub fn graph_clips(&self, graph: &str) -> (Vec<(String, ClipRef)>, Vec<String>) {
        let mut clips = Vec::new();
        let mut slots = Vec::new();
        for (name, slot) in self.slots.iter().filter(|(_, s)| s.graph == graph && !s.candidates.is_empty()) {
            let n = slot.candidates.len();
            for (i, c) in slot.candidates.iter().enumerate() {
                let fallback = if i + 1 < n { ClipFallback::Clip(Slot::clip_name(name, i + 1)) } else { slot.fallback.clone() };
                clips.push((Slot::clip_name(name, i), ClipRef { source: c.reference(), optional: true, fallback: Some(fallback) }));
            }
            slots.push(name.clone());
        }
        (clips, slots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_manifest_parses_with_the_mantle_slot() {
        let m = SourcesManifest::builtin();
        let ual1 = &m.sources["quaternius_ual1"];
        assert_eq!(ual1.license, License::CC0);
        assert!(ual1.redistributable());
        assert_eq!(ual1.files[0].sha256.len(), 64);
        assert_eq!(m.sources["quaternius_ual2"].license, License::CC0);
        let lafan1 = &m.sources["lafan1"];
        assert_eq!(lafan1.license, License::CC_BY_NC_ND_4_0);
        assert!(lafan1.private_only && !lafan1.redistributable() && lafan1.root_from_pelvis);
        assert_eq!(lafan1.rest_frame, Some(0), "LAFAN1 takes open on the actor's T-pose");
        assert!(lafan1.files.iter().all(|f| f.sha256.len() == 64));
        let slot = &m.slots["traverse.mantle.waist"];
        assert_eq!(slot.candidates[0].reference(), "quaternius_ual2:ClimbUp_1m_RM");
        assert_eq!(slot.candidates[1].reference(), "lafan1:bvh/obstacles4_subject3.bvh:obstacles4_subject3@199.4-201.2");
        let refs = m.references().unwrap();
        let root = refs.iter().find(|r| r.reference.contains("obstacles4")).and_then(|r| r.retarget_root.clone()).unwrap();
        assert!(root.vertical && root.extract && root.mode == RootMotionMode::Progress);
        // Get-ups: in place, re-based on the last frame (no root track).
        let getup = refs.iter().find(|r| r.reference.contains("fallAndGetUp2_subject3")).and_then(|r| r.retarget_root.clone()).unwrap();
        assert!(!getup.extract && getup.anchor_end);
        assert!(refs.iter().filter(|r| r.source != "lafan1").all(|r| r.retarget_root.is_none()));
        assert_eq!(m.slots["traverse.climb.up"].candidates[0].reference(), "quaternius_ual1:ClimbLedge_RM");
        let rules = slot.rules(&slot.candidates[0]);
        assert_eq!(rules.root_motion.mode, RootMotionMode::Progress);
        assert!(rules.root_motion.vertical && rules.root_motion.extract);
        let (clips, slots) = m.graph_clips("humanoid");
        assert_eq!(
            slots,
            [
                "attack.sword_1h.light1",
                "attack.sword_1h.light1_rec",
                "attack.sword_1h.light2",
                "attack.sword_1h.light2_rec",
                "attack.sword_1h.light3",
                "dodge.roll.fwd",
                "dodge.roll.left",
                "dodge.roll.right",
                "getup.prone",
                "getup.side_right",
                "getup.supine",
                "react.large.front",
                "react.medium.front",
                "react.small.back",
                "react.small.front",
                "react.small.left",
                "react.small.right",
                "traverse.climb.up",
                "traverse.mantle.waist"
            ]
        );
        // Combat slots fall back to Morrowind's groups (rolls drop).
        assert_eq!(m.slots["react.small.front"].fallback, ClipFallback::Clip("Hit1".into()));
        assert_eq!(m.slots["dodge.roll.fwd"].fallback, ClipFallback::Drop);
        // Get-ups: the UAL clip first, LaFAN1 as its alternative.
        assert_eq!(
            clips.iter().find(|(n, _)| n == "getup.prone").unwrap().1.fallback,
            Some(ClipFallback::Clip("getup.prone.alt1".into()))
        );
        assert_eq!(clips[0].1.fallback, Some(ClipFallback::Clip("attack.sword_1h.light1.alt1".into())));
    }

    #[test]
    fn candidates_chain_and_references_stay_consistent() {
        let text = r#"(
          sources: { "a": (license: CC_BY_NC_4_0, page: "", files: [(name: "a.glb", sha256: "")]),
                     "b": (license: CC0, page: "", private_only: true, files: [(name: "b1.bvh", sha256: ""), (name: "b2.bvh", sha256: "")]) },
          slots: { "loco.walk.fwd": (graph: "humanoid", candidates: [(source: "a", clip: "Walk"), (source: "b", file: Some("b2.bvh"), clip: "Take", range: Some((1.0, 2.5)))],
                     looping: Whole, root: Match, fallback: Clip("WalkForward")) },
        )"#;
        let m = SourcesManifest::parse(text).unwrap();
        assert!(!m.sources["a"].redistributable() && !m.sources["b"].redistributable());
        let refs = m.references().unwrap();
        assert_eq!(refs[1].reference, "b:b2.bvh:Take@1-2.5");
        assert_eq!(refs[1].file, "b2.bvh");
        let (clips, _) = m.graph_clips("humanoid");
        assert_eq!(clips[0].0, "loco.walk.fwd");
        assert_eq!(clips[0].1.fallback, Some(ClipFallback::Clip("loco.walk.fwd.alt1".into())));
        assert_eq!(clips[1].1.fallback, Some(ClipFallback::Clip("WalkForward".into())));
        assert!(m.graph_clips("creature").0.is_empty());

        let bad = text.replace("(source: \"b\", file", "(source: \"a\", clip: \"Walk\", range: Some((0.0, 1.0))), (source: \"b\", file");
        assert!(SourcesManifest::parse(&bad).is_ok(), "a different range is a different reference");
        let clash = r#"(sources: { "a": (license: CC0, page: "", files: [(name: "a.glb", sha256: "")]) },
          slots: { "x": (graph: "humanoid", candidates: [(source: "a", clip: "C")]),
                   "y": (graph: "humanoid", candidates: [(source: "a", clip: "C")], vertical: true) })"#;
        assert!(SourcesManifest::parse(clash).unwrap_err().contains("other cook rules"));
        assert!(SourcesManifest::parse(&text.replace("source: \"a\"", "source: \"zz\"")).is_err());
    }
}
