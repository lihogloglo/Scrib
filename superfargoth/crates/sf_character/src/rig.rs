//! Rig maps: how one skeleton's joints map onto the standard humanoid,
//! plus its retarget pose, mirror pairs and extra chains. An adapter
//! provides one, or [`crate::auto_map`] detects it and
//! [`crate::auto_align`] fills the retarget pose.

use glam::{Affine3A, Quat};
use serde::{Deserialize, Serialize};
use sf_anim::{
    Chain as ProfileChain, LegKind, LimbDesc, LimbKind, LimbSolver, NeckHead, ProfileParts, RetargetChain, RetargetRig, RigProfile, RigTemplate,
    Skeleton, Trs,
};

use crate::humanoid::{BoneSet, Finger, HumanoidBone, Limb, Side};

/// A joint, by name (names are what crosses rigs and files).
pub type JointName = String;

/// A named joint chain, root first: tails, wings, skirt joints. Chains
/// pair across rigs by name; unpaired chains keep their retarget pose.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Chain {
    pub name: String,
    pub joints: Vec<JointName>,
}

/// A skeleton's map onto the standard humanoid (character.md).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RigMap {
    pub bones: Vec<(JointName, HumanoidBone)>,
    /// The joint whose motion becomes root motion.
    pub root_motion: JointName,
    /// Local rotations that bring the rest pose to the canonical T-pose
    /// (Y up, facing +Z). They replace the rest rotation of the joints
    /// listed; others keep theirs.
    pub retarget_pose: Vec<(JointName, Quat)>,
    /// Left/right joint pairs (the `Mirror` node's table).
    pub mirror: Vec<(JointName, JointName)>,
    pub extra_chains: Vec<Chain>,
}

/// Something wrong with a rig map against its skeleton.
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum RigMapError {
    #[error("rig map names joint {0:?}, which the skeleton lacks")]
    UnknownJoint(String),
    #[error("humanoid bone {0} is mapped twice")]
    DuplicateBone(&'static str),
    #[error("joint {0:?} is mapped to two humanoid bones")]
    DuplicateJoint(String),
    #[error("required humanoid bone {0} is not mapped")]
    MissingRequired(&'static str),
    #[error("{bone} ({joint:?}) is not a descendant of {parent} ({parent_joint:?})")]
    Hierarchy { bone: &'static str, joint: String, parent: &'static str, parent_joint: String },
    #[error("upperChest is mapped without chest (VRM 1.0 rule)")]
    UpperChestWithoutChest,
}

/// Options for [`RigMap::retarget_rig`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetargetOptions {
    /// Mark the leg chains for the IK fix-up (feet keep the source's
    /// scaled contact positions). Only on a target rig.
    pub ik_legs: bool,
}

impl Default for RetargetOptions {
    fn default() -> Self {
        Self { ik_legs: true }
    }
}

impl RigMap {
    /// The joint mapped to `bone`.
    pub fn joint(&self, bone: HumanoidBone) -> Option<&str> {
        self.bones.iter().find(|(_, b)| *b == bone).map(|(j, _)| j.as_str())
    }

    /// The humanoid bone a joint is mapped to.
    pub fn bone(&self, joint: &str) -> Option<HumanoidBone> {
        self.bones.iter().find(|(j, _)| j == joint).map(|(_, b)| *b)
    }

    pub fn bone_set(&self) -> BoneSet {
        BoneSet::of(&self.bones.iter().map(|(_, b)| *b).collect::<Vec<_>>())
    }

    /// Every problem with this map against `skeleton` (empty: valid).
    pub fn validate(&self, skeleton: &Skeleton) -> Vec<RigMapError> {
        let mut errors = Vec::new();
        let mut seen = BoneSet::EMPTY;
        let mut joints = std::collections::HashSet::new();
        for (joint, bone) in &self.bones {
            if skeleton.find(joint).is_none() {
                errors.push(RigMapError::UnknownJoint(joint.clone()));
            }
            if seen.contains(*bone) {
                errors.push(RigMapError::DuplicateBone(bone.name()));
            }
            seen.insert(*bone);
            if !joints.insert(joint.as_str()) {
                errors.push(RigMapError::DuplicateJoint(joint.clone()));
            }
        }
        let named = self.retarget_pose.iter().map(|(j, _)| j).chain(self.mirror.iter().flat_map(|(a, b)| [a, b]));
        let named = named.chain(self.extra_chains.iter().flat_map(|c| &c.joints)).chain(std::iter::once(&self.root_motion));
        for joint in named {
            if skeleton.find(joint).is_none() && !errors.contains(&RigMapError::UnknownJoint(joint.clone())) {
                errors.push(RigMapError::UnknownJoint(joint.clone()));
            }
        }
        for bone in HumanoidBone::ALL.into_iter().filter(|b| b.required() && !seen.contains(*b)) {
            errors.push(RigMapError::MissingRequired(bone.name()));
        }
        if seen.contains(HumanoidBone::UpperChest) && !seen.contains(HumanoidBone::Chest) {
            errors.push(RigMapError::UpperChestWithoutChest);
        }
        for (joint, bone) in &self.bones {
            let Some(parent) = bone.nearest_parent_in(seen) else {
                continue;
            };
            let parent_joint = self.joint(parent).unwrap_or_default();
            if let (Some(j), Some(p)) = (skeleton.find(joint), skeleton.find(parent_joint))
                && (j == p || !skeleton.is_ancestor(p, j))
            {
                errors.push(RigMapError::Hierarchy {
                    bone: bone.name(),
                    joint: joint.clone(),
                    parent: parent.name(),
                    parent_joint: parent_joint.to_owned(),
                });
            }
        }
        errors
    }

    /// The local retarget pose: rest transforms with `retarget_pose`'s
    /// rotations in place. Unknown joints are skipped.
    pub fn retarget_pose_local(&self, skeleton: &Skeleton) -> Vec<Trs> {
        let mut pose = skeleton.rest().to_vec();
        for (name, q) in &self.retarget_pose {
            if let Some(j) = skeleton.find(name) {
                pose[j].rotation = q.normalize();
            }
        }
        pose
    }

    /// Model-space transforms of the retarget pose.
    pub fn retarget_pose_model(&self, skeleton: &Skeleton) -> Vec<Affine3A> {
        model_of(skeleton, &self.retarget_pose_local(skeleton))
    }

    /// The `sf_anim` retargeting rig: one chain per humanoid bone group,
    /// named after it so chains pair across rigs:
    ///
    /// - `hips`, `head`, `jaw`, `left_eye`/`right_eye`, `left_shoulder`/…,
    ///   `left_toes`/…: one joint each.
    /// - `spine`: every joint from the spine bone up to the highest mapped
    ///   torso bone (unmapped joints between them included, so a five-joint
    ///   spine pairs with a three-joint one by arc length).
    /// - `neck`: the neck bone up to the head's parent.
    /// - `left_arm` (upper arm, lower arm, hand), `left_leg` (the joint path
    ///   from upper leg to foot, a digitigrade metatarsal included; IK with
    ///   [`RetargetOptions::ik_legs`], solver by joint count).
    /// - `left_thumb`, `left_index`, …: the mapped segments.
    /// - the extra chains, as named.
    pub fn retarget_rig(&self, skeleton: &Skeleton, options: RetargetOptions) -> Result<RetargetRig, RigMapError> {
        let find = |name: &str| skeleton.find(name).ok_or_else(|| RigMapError::UnknownJoint(name.to_owned()));
        let name = |b: HumanoidBone| self.joint(b).map(str::to_owned);
        let mut chains = Vec::new();
        let single = |chain: &str, b: HumanoidBone, chains: &mut Vec<RetargetChain>| {
            if let Some(j) = name(b) {
                chains.push(RetargetChain { name: chain.to_owned(), joints: vec![j], ik: false, solver: None });
            }
        };
        single("hips", HumanoidBone::Hips, &mut chains);
        // Spine: the path from the spine bone to the highest torso bone.
        if let Some(spine) = self.joint(HumanoidBone::Spine) {
            let top = [HumanoidBone::UpperChest, HumanoidBone::Chest].into_iter().find_map(|b| self.joint(b)).unwrap_or(spine);
            chains.push(RetargetChain { name: "spine".into(), joints: path(skeleton, find(spine)?, find(top)?), ik: false, solver: None });
        }
        if let (Some(neck), Some(head)) = (self.joint(HumanoidBone::Neck), self.joint(HumanoidBone::Head)) {
            let (neck, head) = (find(neck)?, find(head)?);
            let top = skeleton.parent(head).filter(|&p| skeleton.is_ancestor(neck, p)).unwrap_or(neck);
            chains.push(RetargetChain { name: "neck".into(), joints: path(skeleton, neck, top), ik: false, solver: None });
        }
        single("head", HumanoidBone::Head, &mut chains);
        single("jaw", HumanoidBone::Jaw, &mut chains);
        for side in [Side::Left, Side::Right] {
            let s = if side == Side::Left { "left" } else { "right" };
            let limb = |l: Limb| HumanoidBone::limb(side, l).expect("sided");
            single(&format!("{s}_eye"), limb(Limb::Eye), &mut chains);
            single(&format!("{s}_shoulder"), limb(Limb::Shoulder), &mut chains);
            single(&format!("{s}_toes"), limb(Limb::Toes), &mut chains);
            let three = |ls: [Limb; 3]| ls.iter().map(|l| name(limb(*l))).collect::<Option<Vec<_>>>();
            if let Some(joints) = three([Limb::UpperArm, Limb::LowerArm, Limb::Hand]) {
                chains.push(RetargetChain { name: format!("{s}_arm"), joints, ik: false, solver: None });
            }
            // The leg is the whole joint path from upper leg to foot, so a
            // digitigrade metatarsal between lower leg and foot stays in the
            // chain (solved with `TwoBoneAnkle`).
            if let (Some(upper), Some(foot)) = (self.joint(limb(Limb::UpperLeg)), self.joint(limb(Limb::Foot))) {
                let joints = path(skeleton, find(upper)?, find(foot)?);
                let solver = Some(LimbSolver::for_len(joints.len()));
                chains.push(RetargetChain { name: format!("{s}_leg"), joints, ik: options.ik_legs, solver });
            }
            for f in Finger::ALL {
                let joints: Vec<String> = (0..3).filter_map(|k| HumanoidBone::finger(side, f, k).and_then(name)).collect();
                if !joints.is_empty() {
                    chains.push(RetargetChain { name: format!("{s}_{}", f.name()), joints, ik: false, solver: None });
                }
            }
        }
        for c in &self.extra_chains {
            chains.push(RetargetChain { name: c.name.clone(), joints: c.joints.clone(), ik: false, solver: None });
        }
        for c in &chains {
            for j in &c.joints {
                find(j)?;
            }
        }
        Ok(RetargetRig { chains, retarget_pose: self.retarget_pose.clone(), hips: name(HumanoidBone::Hips) })
    }
}

impl RigMap {
    /// The skeleton's rig profile from this humanoid map (rig-profile.md,
    /// "Proposed changes to character.md" 1): template `Biped`, support
    /// limbs `leftLeg`/`rightLeg` (the joint path from upper leg to foot,
    /// `TwoBoneAnkle` when a digitigrade metatarsal makes it four joints;
    /// toes as the foot's toe), `leftArm`/`rightArm` (from the shoulder
    /// when mapped, which then stays passive), spine, neck and head from
    /// the humanoid bones, extra chains as tails (named `tail…`) or extra.
    pub fn profile(&self, skeleton: &Skeleton) -> Result<RigProfile, RigMapError> {
        let find = |name: &str| skeleton.find(name).ok_or_else(|| RigMapError::UnknownJoint(name.to_owned()));
        let idx = |b: HumanoidBone| self.joint(b).map(find).transpose();
        let u16_of = |j: usize| u16::try_from(j).expect("joint index fits u16");
        let path16 = |from: usize, to: usize| -> Vec<u16> { path(skeleton, from, to).iter().filter_map(|n| skeleton.find(n)).map(u16_of).collect() };
        let girdle_of = |first: usize| u16_of(skeleton.parent(first).unwrap_or(first));
        let mut limbs = Vec::new();
        for side in [Side::Left, Side::Right] {
            let s = if side == Side::Left { "left" } else { "right" };
            let limb = |l: Limb| HumanoidBone::limb(side, l).expect("sided");
            if let (Some(upper), Some(foot)) = (idx(limb(Limb::UpperLeg))?, idx(limb(Limb::Foot))?) {
                let joints = path16(upper, foot);
                let solver = LimbSolver::for_len(joints.len());
                let kind = LimbKind::Leg(if solver == LimbSolver::TwoBoneAnkle { LegKind::Digitigrade } else { LegKind::Plantigrade });
                limbs.push(LimbDesc {
                    name: Some(format!("{s}Leg")),
                    kind,
                    girdle: girdle_of(upper),
                    solver,
                    pole: None,
                    toe: idx(limb(Limb::Toes))?.map(u16_of),
                    support: true,
                    side: Some(if side == Side::Left { sf_anim::Side::Left } else { sf_anim::Side::Right }),
                    joints,
                });
            }
            if let (Some(upper), Some(hand)) = (idx(limb(Limb::UpperArm))?, idx(limb(Limb::Hand))?) {
                let first = idx(limb(Limb::Shoulder))?.filter(|&sh| skeleton.is_ancestor(sh, upper)).unwrap_or(upper);
                limbs.push(LimbDesc {
                    name: Some(format!("{s}Arm")),
                    kind: LimbKind::Arm,
                    girdle: girdle_of(first),
                    solver: LimbSolver::TwoBone,
                    pole: None,
                    toe: None,
                    support: false,
                    side: Some(if side == Side::Left { sf_anim::Side::Left } else { sf_anim::Side::Right }),
                    joints: path16(first, hand),
                });
            }
        }
        let spine = match (idx(HumanoidBone::Hips)?, idx(HumanoidBone::Spine)?) {
            (Some(hips), Some(spine)) => {
                let top = [HumanoidBone::UpperChest, HumanoidBone::Chest].into_iter().find_map(|b| idx(b).transpose()).transpose()?.unwrap_or(spine);
                Some(ProfileChain { name: "spine".into(), joints: path16(hips, top) })
            }
            _ => None,
        };
        let neck_head = match idx(HumanoidBone::Head)? {
            Some(head) => {
                let neck = match idx(HumanoidBone::Neck)? {
                    Some(neck) => {
                        skeleton.parent(head).filter(|&p| skeleton.is_ancestor(neck, p)).map_or_else(|| vec![u16_of(neck)], |top| path16(neck, top))
                    }
                    None => Vec::new(),
                };
                let rest = model_of(skeleton, skeleton.rest());
                let rot = Trs::from_affine(&rest[head]).rotation;
                let eyes =
                    [HumanoidBone::LeftEye, HumanoidBone::RightEye].into_iter().filter_map(|b| idx(b).transpose()).collect::<Result<Vec<_>, _>>()?;
                Some(NeckHead {
                    neck: ProfileChain { name: "neck".into(), joints: neck },
                    head: u16_of(head),
                    forward: (rot.inverse() * glam::Vec3::Z).normalize_or(glam::Vec3::Z),
                    eyes: eyes.into_iter().map(u16_of).collect(),
                })
            }
            None => None,
        };
        let (mut tails, mut extra) = (Vec::new(), Vec::new());
        for c in &self.extra_chains {
            let joints = c.joints.iter().map(|n| find(n).map(u16_of)).collect::<Result<Vec<_>, _>>()?;
            let chain = ProfileChain { name: c.name.clone(), joints };
            if c.name.to_ascii_lowercase().contains("tail") { tails.push(chain) } else { extra.push(chain) }
        }
        let root_motion = skeleton.find(&self.root_motion).map(u16_of);
        Ok(RigProfile::build(skeleton, ProfileParts { template: RigTemplate::Biped, root_motion, limbs, spine, neck_head, tails, extra }))
    }
}

/// Joint names from `from` down to `to` (inclusive), or just `to` when
/// `from` is not its ancestor.
fn path(skeleton: &Skeleton, from: usize, to: usize) -> Vec<String> {
    let mut out = vec![to];
    if skeleton.is_ancestor(from, to) {
        let mut j = to;
        while j != from {
            match skeleton.parent(j) {
                Some(p) => {
                    out.push(p);
                    j = p;
                }
                None => break,
            }
        }
    }
    out.reverse();
    out.into_iter().map(|j| skeleton.name(j).to_owned()).collect()
}

/// Local transforms to model space.
pub(crate) fn model_of(skeleton: &Skeleton, local: &[Trs]) -> Vec<Affine3A> {
    let mut out: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        out.push(match skeleton.parent(j) {
            Some(p) => out[p] * m,
            None => m,
        });
    }
    out
}
