//! Skeletons: joint hierarchy in depth-first order, rest pose, tags, and
//! joint LOD masks. The bake product of `skeleton.cook`.

use glam::{Affine3A, Vec3};
use serde::{Deserialize, Serialize};
use sf_bake::Digest;

use crate::{FormatError, Trs};

/// Parent index of a root joint.
pub const NO_PARENT: u16 = u16::MAX;

/// Fingerprint of a skeleton's topology (joint names and parents). A clip
/// is bound to one; the rest pose may change without rebinding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkeletonId(pub u64);

/// A joint as an importer or adapter provides it: parents in any order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceJoint {
    pub name: String,
    /// Index into the same source list.
    pub parent: Option<usize>,
    pub rest: Trs,
}

/// A set of joints as a bitset.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JointMask {
    bits: Vec<u64>,
    len: usize,
}

impl JointMask {
    pub fn empty(len: usize) -> Self {
        Self { bits: vec![0; len.div_ceil(64)], len }
    }

    pub fn full(len: usize) -> Self {
        let mut m = Self::empty(len);
        for j in 0..len {
            m.insert(j);
        }
        m
    }

    #[inline]
    pub fn contains(&self, joint: usize) -> bool {
        joint < self.len && self.bits[joint / 64] & (1 << (joint % 64)) != 0
    }

    pub fn insert(&mut self, joint: usize) {
        assert!(joint < self.len, "joint {joint} out of range");
        self.bits[joint / 64] |= 1 << (joint % 64);
    }

    pub fn remove(&mut self, joint: usize) {
        if joint < self.len {
            self.bits[joint / 64] &= !(1 << (joint % 64));
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn count(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.len).filter(|&j| self.contains(j))
    }
}

/// Which joints each LOD drops, by tag. A dropped joint takes its
/// descendants with it, so every mask is closed under "parent of".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LodRules {
    /// Tags dropped from LOD1 on.
    pub lod1: Vec<String>,
    /// Tags dropped from LOD2 on (in addition to `lod1`).
    pub lod2: Vec<String>,
    /// Joints (by name) that every LOD keeps, with their ancestors
    /// (attachment sockets, for example).
    pub keep: Vec<String>,
}

impl Default for LodRules {
    fn default() -> Self {
        let tags = |t: &[&str]| t.iter().map(|s| (*s).to_owned()).collect();
        Self { lod1: tags(&[tag::FINGER, tag::FACE, tag::EYE, tag::JAW, tag::TWIST, tag::END]), lod2: tags(&[tag::TOE]), keep: Vec::new() }
    }
}

/// Adapter rig data for `skeleton.cook`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkeletonRules {
    /// The joint whose motion becomes root motion.
    pub root_motion_joint: Option<String>,
    /// With no `root_motion_joint`, use the one the importer detected
    /// ([`crate::detect_root_motion_joint`]: a moving `root`, else the
    /// shallowest moving joint). Off by default.
    #[serde(default)]
    pub detect_root_motion: bool,
    /// Extra `(joint, tag)` pairs.
    pub tags: Vec<(String, String)>,
    /// Also guess tags from joint names ([`guess_tag`]).
    pub guess_tags: bool,
    pub lod: LodRules,
}

impl Default for SkeletonRules {
    fn default() -> Self {
        Self { root_motion_joint: None, detect_root_motion: false, tags: Vec::new(), guess_tags: true, lod: LodRules::default() }
    }
}

/// Well-known joint tags. Tags are free strings; these are the ones the
/// framework itself reads (contacts, LOD defaults).
pub mod tag {
    pub const HIPS: &str = "hips";
    pub const SPINE: &str = "spine";
    pub const NECK: &str = "neck";
    pub const HEAD: &str = "head";
    pub const EYE: &str = "eye";
    pub const JAW: &str = "jaw";
    pub const FACE: &str = "face";
    pub const HAND: &str = "hand";
    pub const FINGER: &str = "finger";
    pub const FOOT: &str = "foot";
    pub const TOE: &str = "toe";
    pub const TWIST: &str = "twist";
    /// End sites and nubs (leaf helper joints).
    pub const END: &str = "end";
    /// Tail, antenna and tentacle joints (never support limbs).
    pub const TAIL: &str = "tail";
    /// Wing joints (rig-profile.md: the limb becomes a `Wing`).
    pub const WING: &str = "wing";
    /// A claw or talon: a ground-contact effector.
    pub const CLAW: &str = "claw";
    /// A hoof: a ground-contact effector of an unguligrade leg.
    pub const HOOF: &str = "hoof";
    /// Any other joint whose ground contacts `anim.cook` should detect.
    pub const CONTACT: &str = "contact";

    /// Tags that mark ground-contact joints when no rig profile names them.
    pub const CONTACTS: [&str; 4] = [FOOT, CLAW, HOOF, CONTACT];
}

/// Guesses a tag from common English joint naming (`LeftFoot`, `foot_l`,
/// `hand.R`, `Finger0Nub`, `upperarm_twist_01`). Adapters with their own
/// conventions pass explicit tags instead.
pub fn guess_tag(name: &str) -> Option<&'static str> {
    let tokens = tokenize(name);
    let has = |words: &[&str]| tokens.iter().any(|t| words.contains(&t.as_str()));
    if has(&["end", "nub", "tip", "top"]) {
        return Some(tag::END);
    }
    if has(&["twist", "roll"]) {
        return Some(tag::TWIST);
    }
    if has(&["hoof", "hooves"]) {
        return Some(tag::HOOF);
    }
    if has(&["claw", "claws", "talon", "talons"]) {
        return Some(tag::CLAW);
    }
    if has(&["finger", "thumb", "index", "middle", "ring", "pinky", "little"]) {
        return Some(tag::FINGER);
    }
    if has(&["toe", "toes", "ball", "toebase"]) {
        return Some(tag::TOE);
    }
    if has(&["foot", "ankle"]) {
        return Some(tag::FOOT);
    }
    if has(&["hand", "wrist", "palm"]) {
        return Some(tag::HAND);
    }
    if has(&["wing", "wings"]) {
        return Some(tag::WING);
    }
    if has(&["tail", "antenna", "antennae", "tentacle"]) {
        return Some(tag::TAIL);
    }
    if has(&["eye", "eyelid", "brow", "eyebrow"]) {
        return Some(tag::EYE);
    }
    if has(&["jaw", "tongue", "teeth", "lip", "mouth", "cheek", "nose"]) {
        return Some(tag::FACE);
    }
    if has(&["head"]) {
        return Some(tag::HEAD);
    }
    if has(&["neck"]) {
        return Some(tag::NECK);
    }
    // Not "hip": several rigs name the thigh `hip_l`.
    if has(&["hips", "pelvis"]) {
        return Some(tag::HIPS);
    }
    if has(&["spine", "chest"]) {
        return Some(tag::SPINE);
    }
    None
}

/// Lowercase tokens split at non-alphanumerics, digit runs and camelCase.
fn tokenize(name: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut prev: Option<char> = None;
    for c in name.chars() {
        let boundary = match prev {
            None => false,
            Some(p) => {
                !c.is_alphanumeric() || (c.is_uppercase() && p.is_lowercase()) || (c.is_ascii_digit() != p.is_ascii_digit() && p.is_alphanumeric())
            }
        };
        if boundary && !cur.is_empty() {
            tokens.push(std::mem::take(&mut cur));
        }
        if c.is_alphanumeric() {
            cur.extend(c.to_lowercase());
        }
        prev = Some(c);
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    tokens
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SkeletonError {
    #[error("skeleton has no joints")]
    Empty,
    #[error("too many joints ({0}, max 65534)")]
    TooMany(usize),
    #[error("duplicate joint name {0:?}")]
    DuplicateName(String),
    #[error("joint {0:?} has an invalid parent or is part of a cycle")]
    BadParent(String),
    #[error("no joint named {0:?}")]
    UnknownJoint(String),
}

/// A skeleton. Joints are in depth-first order: a parent always precedes
/// its children and every subtree is a contiguous range, so local-to-model
/// is one forward loop and a subtree update is a slice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skeleton {
    names: Vec<String>,
    parents: Vec<u16>,
    /// One past the last joint of each joint's subtree.
    subtree_end: Vec<u16>,
    rest: Vec<Trs>,
    root_motion_joint: Option<u16>,
    /// `(joint, tag)`, sorted.
    tags: Vec<(u16, String)>,
    /// Joints evaluated at LOD0..=2. LOD3 evaluates none.
    lod: [JointMask; 3],
}

const MAGIC: &[u8; 4] = b"SFSK";
/// Version of the cooked skeleton format.
pub const SKELETON_FORMAT: u32 = 1;

impl Skeleton {
    /// Builds a skeleton from joints in any order. Also returns, for each
    /// source joint, its index in the skeleton.
    pub fn new(joints: &[SourceJoint]) -> Result<(Self, Vec<usize>), SkeletonError> {
        Self::with_rules(joints, &SkeletonRules { guess_tags: false, ..SkeletonRules::default() })
    }

    /// [`Skeleton::new`] plus root motion joint, tags and LOD masks.
    pub fn with_rules(joints: &[SourceJoint], rules: &SkeletonRules) -> Result<(Self, Vec<usize>), SkeletonError> {
        let n = joints.len();
        if n == 0 {
            return Err(SkeletonError::Empty);
        }
        if n >= usize::from(NO_PARENT) {
            return Err(SkeletonError::TooMany(n));
        }
        let mut seen = std::collections::HashSet::new();
        for j in joints {
            if !seen.insert(j.name.as_str()) {
                return Err(SkeletonError::DuplicateName(j.name.clone()));
            }
        }
        let mut children = vec![Vec::new(); n];
        let mut roots = Vec::new();
        for (i, j) in joints.iter().enumerate() {
            match j.parent {
                None => roots.push(i),
                Some(p) if p < n && p != i => children[p].push(i),
                Some(_) => return Err(SkeletonError::BadParent(j.name.clone())),
            }
        }
        // Depth-first preorder, children in source order.
        let mut order = Vec::with_capacity(n);
        let mut stack: Vec<usize> = roots.into_iter().rev().collect();
        while let Some(i) = stack.pop() {
            order.push(i);
            stack.extend(children[i].iter().rev());
        }
        if order.len() != n {
            // Joints unreachable from a root sit on a parent cycle.
            let mut reached = vec![false; n];
            for &i in &order {
                reached[i] = true;
            }
            let bad = reached.iter().position(|r| !r).unwrap_or(0);
            return Err(SkeletonError::BadParent(joints[bad].name.clone()));
        }
        let mut remap = vec![0usize; n];
        for (new, &old) in order.iter().enumerate() {
            remap[old] = new;
        }
        let to_u16 = |i: usize| u16::try_from(i).expect("joint count checked above");
        let names: Vec<String> = order.iter().map(|&i| joints[i].name.clone()).collect();
        let parents: Vec<u16> = order.iter().map(|&i| joints[i].parent.map_or(NO_PARENT, |p| to_u16(remap[p]))).collect();
        let rest = order.iter().map(|&i| joints[i].rest).collect();
        let mut subtree_end: Vec<u16> = (1..=n).map(to_u16).collect();
        for j in (0..n).rev() {
            if parents[j] != NO_PARENT {
                let p = usize::from(parents[j]);
                subtree_end[p] = subtree_end[p].max(subtree_end[j]);
            }
        }
        let mut skeleton = Self {
            names,
            parents,
            subtree_end,
            rest,
            root_motion_joint: None,
            tags: Vec::new(),
            lod: [JointMask::full(n), JointMask::full(n), JointMask::full(n)],
        };
        skeleton.apply_rules(rules)?;
        Ok((skeleton, remap))
    }

    fn apply_rules(&mut self, rules: &SkeletonRules) -> Result<(), SkeletonError> {
        let find = |s: &Self, name: &str| s.find(name).ok_or_else(|| SkeletonError::UnknownJoint(name.to_owned()));
        self.root_motion_joint = match &rules.root_motion_joint {
            Some(name) => Some(to_u16(find(self, name)?)),
            None => None,
        };
        let mut tags = std::collections::BTreeSet::new();
        if rules.guess_tags {
            for (j, name) in self.names.iter().enumerate() {
                if let Some(t) = guess_tag(name) {
                    tags.insert((to_u16(j), t.to_owned()));
                }
            }
        }
        for (joint, t) in &rules.tags {
            tags.insert((to_u16(find(self, joint)?), t.clone()));
        }
        self.tags = tags.into_iter().collect();

        let n = self.len();
        let mut keep = vec![false; n];
        for name in &rules.lod.keep {
            let mut j = Some(find(self, name)?);
            while let Some(i) = j {
                keep[i] = true;
                j = self.parent(i);
            }
        }
        let mut dropped = vec![false; n];
        for (level, drop_tags) in [(1, &rules.lod.lod1), (2, &rules.lod.lod2)] {
            for j in 0..n {
                let tagged = self.joint_tags(j).any(|t| drop_tags.iter().any(|d| d == t));
                let parent_dropped = self.parent(j).is_some_and(|p| dropped[p]);
                dropped[j] = !keep[j] && (dropped[j] || tagged || parent_dropped);
            }
            let mut mask = JointMask::empty(n);
            for j in (0..n).filter(|&j| !dropped[j]) {
                mask.insert(j);
            }
            self.lod[level] = mask;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn name(&self, joint: usize) -> &str {
        &self.names[joint]
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    #[inline]
    pub fn parent(&self, joint: usize) -> Option<usize> {
        let p = self.parents[joint];
        (p != NO_PARENT).then_some(usize::from(p))
    }

    /// Raw parent table ([`NO_PARENT`] for roots).
    pub fn parents(&self) -> &[u16] {
        &self.parents
    }

    /// The joints of `joint`'s subtree, itself included: `joint..end`.
    pub fn subtree(&self, joint: usize) -> std::ops::Range<usize> {
        joint..usize::from(self.subtree_end[joint])
    }

    pub fn is_ancestor(&self, ancestor: usize, joint: usize) -> bool {
        self.subtree(ancestor).contains(&joint)
    }

    pub fn rest(&self) -> &[Trs] {
        &self.rest
    }

    pub fn root_motion_joint(&self) -> Option<usize> {
        self.root_motion_joint.map(usize::from)
    }

    pub fn set_root_motion_joint(&mut self, joint: Option<usize>) {
        self.root_motion_joint = joint.map(to_u16);
    }

    /// Every `(joint, tag)` pair.
    pub fn tags(&self) -> impl Iterator<Item = (usize, &str)> {
        self.tags.iter().map(|(j, t)| (usize::from(*j), t.as_str()))
    }

    pub fn joint_tags(&self, joint: usize) -> impl Iterator<Item = &str> {
        self.tags.iter().filter(move |(j, _)| usize::from(*j) == joint).map(|(_, t)| t.as_str())
    }

    pub fn joints_with_tag<'a>(&'a self, t: &'a str) -> impl Iterator<Item = usize> + 'a {
        self.tags.iter().filter(move |(_, x)| x == t).map(|(j, _)| usize::from(*j))
    }

    /// Joints evaluated at animation LOD `level` (0–2; 3 and above evaluate none).
    pub fn lod_mask(&self, level: usize) -> Option<&JointMask> {
        self.lod.get(level)
    }

    /// Fingerprint of names and parents.
    pub fn id(&self) -> SkeletonId {
        let bytes = postcard::to_stdvec(&(&self.names, &self.parents)).expect("in-memory serialization");
        let d = Digest::of(&bytes);
        SkeletonId(u64::from_le_bytes(d.0[..8].try_into().expect("8 bytes")))
    }

    /// Model-space rest transforms.
    pub fn rest_model(&self) -> Vec<Affine3A> {
        let mut out: Vec<Affine3A> = Vec::with_capacity(self.len());
        for (j, t) in self.rest.iter().enumerate() {
            let local = t.to_affine();
            let m = match self.parent(j) {
                Some(p) => out[p] * local,
                None => local,
            };
            out.push(m);
        }
        out
    }

    /// Per joint, the largest rest-pose distance to any joint of its subtree.
    pub fn subtree_reach(&self) -> Vec<f32> {
        let model = self.rest_model();
        (0..self.len())
            .map(|j| {
                let p = Vec3::from(model[j].translation);
                self.subtree(j).map(|k| Vec3::from(model[k].translation).distance(p)).fold(0.0, f32::max)
            })
            .collect()
    }

    /// The cooked form: magic, format version, postcard.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend(SKELETON_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let body = crate::check_header(bytes, *MAGIC, SKELETON_FORMAT)?;
        let s: Self = postcard::from_bytes(body).map_err(|e| FormatError::Decode(e.to_string()))?;
        let n = s.names.len();
        let valid = s.parents.len() == n
            && s.rest.len() == n
            && s.subtree_end.len() == n
            && s.parents.iter().enumerate().all(|(j, &p)| p == NO_PARENT || usize::from(p) < j)
            && s.lod.iter().all(|m| m.len() == n);
        if !valid {
            return Err(FormatError::Decode("inconsistent skeleton tables".into()));
        }
        Ok(s)
    }
}

fn to_u16(i: usize) -> u16 {
    u16::try_from(i).expect("joint index fits u16")
}
