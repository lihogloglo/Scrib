//! Retargeting: a clip on skeleton S played on skeleton T (`anim.retarget`,
//! character.md), as UE5's IK Retargeter does it.
//!
//! Each skeleton has a [`RetargetRig`]: named chains (a humanoid hub bone
//! is a one-joint chain; a spine is several joints) and a retarget pose
//! (rest rotations overridden so every rig stands in the same T-pose,
//! Y up, facing +Z). Chains pair across rigs by name. Per frame:
//!
//! 1. Every mapped source joint's model-space rotation is turned into a
//!    delta from its retarget pose, `D = qS · Rs⁻¹`, and the paired target
//!    joint gets `qT = D · Rt` (character.md's `qS · Rs⁻¹ · Rt`), converted
//!    back to local with T's hierarchy. Joints of differently sized chains
//!    pair by normalized arc length along the chain (equal counts pair by
//!    index), blending the two nearest source deltas. Unmapped target joints
//!    keep their retarget-pose local transform.
//! 2. The hips' model position moves by the source's offset from its
//!    retarget pose, scaled by `hips_height(T) / hips_height(S)`; root
//!    motion and the speed meta scale likewise.
//! 3. Optional IK fix-up: a target chain with `ik` (a leg of any joint
//!    count) pins its end to the source end's scaled position with its limb
//!    solver (two-bone, `TwoBoneAnkle` for digitigrade legs, FABRIK beyond),
//!    horizontally by the hips ratio (so strides match the scaled root
//!    motion) and vertically by the limb-length ratio, weighted by the
//!    source's `contact.<end>` curve when the clip has
//!    one, so planted feet stay planted on a rig with other proportions.
//! 4. Optional root motion from the target pelvis
//!    ([`Retargeter::frames_with_root`], UE 5.4's "Root Motion: generate
//!    from target pelvis"), for sources with no usable root bone (mocap
//!    takes): the source keeps its motion in the pose, every frame is
//!    retargeted in the take's space, the clip is re-based so the target
//!    hips stand over their retarget-pose ground point facing +Z at frame 0
//!    (heading measured on the target hips against their retarget pose,
//!    never against the source's rest), then root motion is extracted from
//!    the target hips with the clip's [`RootMotionRules`].
//!
//! Joint local axes may differ between rigs (Blender's +Y bones against
//! 3ds Max's +X bones): only model-space deltas cross over.

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::cook::{ancestors, extract_root_motion, model_of, root_meta};
use crate::ik::{LimbGoal, LimbSolver, limb_pole, solve_limb};
use crate::math::{affine_rotation, nlerp, twist_angle};
use crate::{Clip, CookError, CookSettings, FrameClip, LocalPose, ModelPose, RootMotionRules, Skeleton, Trs, cook_frames};

/// One named joint chain of a rig, root first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetargetChain {
    /// Pairs the chain with the other rig's chain of the same name
    /// (humanoid bone or chain names, e.g. `"spine"`, `"left_leg"`).
    pub name: String,
    pub joints: Vec<String>,
    /// On a target rig: pin the chain's end to the source end's scaled
    /// position with IK. Needs at least two joints.
    pub ik: bool,
    /// The IK solver; by joint count when `None` (3: two-bone, 4–5:
    /// `TwoBoneAnkle`, more: FABRIK).
    #[serde(default)]
    pub solver: Option<LimbSolver>,
}

/// A skeleton's retargeting data (built from a character.md `RigMap`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetargetRig {
    pub chains: Vec<RetargetChain>,
    /// Local rotations of the retarget pose (T-pose) where it differs from
    /// the rest pose. They replace the rest rotation; joints not listed
    /// keep theirs.
    pub retarget_pose: Vec<(String, Quat)>,
    /// The joint whose translation is retargeted and whose retarget-pose
    /// height sets the scale (usually the hips). `None`: scale 1, and no
    /// translation crosses over.
    pub hips: Option<String>,
}

impl RetargetRig {
    /// Stable bytes for the bake key.
    pub fn key_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("in-memory serialization")
    }
}

/// How one target joint is driven: by the blend of two source joints'
/// deltas (`source.0 == source.1` for a one-to-one pair).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RetargetPair {
    pub target: usize,
    pub source: (usize, usize),
    /// Weight of `source.1`.
    pub alpha: f32,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RetargetError {
    #[error("{rig} rig names joint {joint:?}, which its skeleton lacks")]
    UnknownJoint { rig: &'static str, joint: String },
    #[error("chain {0:?} is marked for IK but has fewer than two joints")]
    IkChain(String),
    #[error("the rigs share no chain names")]
    NoSharedChains,
    #[error("the clip is bound to another skeleton")]
    SkeletonMismatch,
    #[error("additive clips can't be retargeted: retarget the base clip, then make it additive on the target")]
    Additive,
    #[error("frame data size mismatch")]
    BadFrames,
    #[error("root motion from the target pelvis needs a hips joint on both rigs")]
    NoHips,
    #[error("root motion from the target pelvis needs a source cooked without root-motion extraction")]
    SourceRootMotion,
    #[error("cooking the retargeted clip: {0}")]
    Cook(#[from] CookError),
}

/// A leg (or arm) whose end is pinned on the target.
#[derive(Clone, Debug)]
struct IkChain {
    target: Vec<usize>,
    solver: LimbSolver,
    source_end: usize,
    /// Target over source chain length (vertical goal offsets).
    limb_scale: f32,
}

/// A compiled (source, target) pair of rigs.
#[derive(Clone, Debug)]
pub struct Retargeter {
    source: Skeleton,
    target: Skeleton,
    pairs: Vec<RetargetPair>,
    /// Per target joint, its index in `pairs`.
    pair_of: Vec<Option<usize>>,
    /// Model-space retarget-pose rotations, inverted for the source.
    source_inv: Vec<Quat>,
    target_rot: Vec<Quat>,
    /// Target retarget pose, local.
    target_pose: Vec<Trs>,
    /// (source hips, target hips, source position, target position).
    hips: Option<(usize, usize, Vec3, Vec3)>,
    source_model: Vec<Affine3A>,
    target_model: Vec<Affine3A>,
    scale: f32,
    ik: Vec<IkChain>,
    /// Source joint name → target joint name, for chain joints paired one
    /// to one and for chain ends (renames `contact.*` and `roll.*` curves
    /// and `down.*` markers).
    renames: Vec<(String, String)>,
}

/// The retarget pose (local) of a skeleton.
fn retarget_pose(skeleton: &Skeleton, rig: &RetargetRig, which: &'static str) -> Result<Vec<Trs>, RetargetError> {
    let mut pose = skeleton.rest().to_vec();
    for (name, q) in &rig.retarget_pose {
        let j = find(skeleton, name, which)?;
        pose[j].rotation = q.normalize();
    }
    Ok(pose)
}

fn find(skeleton: &Skeleton, name: &str, rig: &'static str) -> Result<usize, RetargetError> {
    skeleton.find(name).ok_or_else(|| RetargetError::UnknownJoint { rig, joint: name.to_owned() })
}

/// Normalized arc length of each joint along a chain (model positions).
#[allow(clippy::cast_precision_loss)]
fn arc_params(chain: &[usize], model: &[Affine3A]) -> Vec<f32> {
    let p = |j: usize| Vec3::from(model[j].translation);
    let mut acc = vec![0.0f32; chain.len()];
    for i in 1..chain.len() {
        acc[i] = acc[i - 1] + p(chain[i]).distance(p(chain[i - 1]));
    }
    let total = acc.last().copied().unwrap_or(0.0);
    if total > 1e-6 {
        acc.iter().map(|a| a / total).collect()
    } else if chain.len() > 1 {
        (0..chain.len()).map(|i| i as f32 / (chain.len() - 1) as f32).collect()
    } else {
        vec![0.0]
    }
}

impl Retargeter {
    pub fn new(source: &Skeleton, source_rig: &RetargetRig, target: &Skeleton, target_rig: &RetargetRig) -> Result<Self, RetargetError> {
        let source_pose = retarget_pose(source, source_rig, "source")?;
        let target_pose = retarget_pose(target, target_rig, "target")?;
        let mut source_model = Vec::new();
        let mut target_model = Vec::new();
        model_of(source, &source_pose, &mut source_model);
        model_of(target, &target_pose, &mut target_model);

        let resolve = |s: &Skeleton, chain: &RetargetChain, rig| chain.joints.iter().map(|n| find(s, n, rig)).collect::<Result<Vec<_>, _>>();
        let mut pairs = Vec::new();
        let mut pair_of = vec![None; target.len()];
        let mut ik = Vec::new();
        let mut renames = Vec::new();
        for chain in &source_rig.chains {
            resolve(source, chain, "source")?;
        }
        for t_chain in &target_rig.chains {
            let t_joints = resolve(target, t_chain, "target")?;
            if t_chain.ik && t_joints.len() < 2 {
                return Err(RetargetError::IkChain(t_chain.name.clone()));
            }
            let Some(s_chain) = source_rig.chains.iter().find(|c| c.name == t_chain.name) else {
                continue;
            };
            let s_joints = resolve(source, s_chain, "source")?;
            let (Some(&s_end), Some(&t_end)) = (s_joints.last(), t_joints.last()) else {
                continue;
            };
            if t_chain.ik {
                let chain_len = |c: &[usize], m: &[Affine3A]| {
                    c.windows(2).map(|w| Vec3::from(m[w[0]].translation).distance(Vec3::from(m[w[1]].translation))).sum::<f32>()
                };
                let (ls, lt) = (chain_len(&s_joints, &source_model), chain_len(&t_joints, &target_model));
                ik.push(IkChain {
                    solver: t_chain.solver.unwrap_or_else(|| LimbSolver::for_len(t_joints.len())),
                    target: t_joints.clone(),
                    source_end: s_end,
                    limb_scale: if ls > 1e-4 { lt / ls } else { 1.0 },
                });
            }
            renames.push((source.name(s_end).to_owned(), target.name(t_end).to_owned()));
            if s_joints.len() == t_joints.len() {
                for (&s, &t) in s_joints.iter().zip(&t_joints) {
                    pair_of[t] = Some(pairs.len());
                    pairs.push(RetargetPair { target: t, source: (s, s), alpha: 0.0 });
                    renames.push((source.name(s).to_owned(), target.name(t).to_owned()));
                }
                continue;
            }
            let su = arc_params(&s_joints, &source_model);
            let tu = arc_params(&t_joints, &target_model);
            for (&t, &u) in t_joints.iter().zip(&tu) {
                let pair = if s_joints.len() == 1 {
                    RetargetPair { target: t, source: (s_joints[0], s_joints[0]), alpha: 0.0 }
                } else {
                    let k = su.partition_point(|&x| x <= u).saturating_sub(1).min(s_joints.len() - 2);
                    let span = su[k + 1] - su[k];
                    let alpha = if span > 1e-6 { ((u - su[k]) / span).clamp(0.0, 1.0) } else { 0.0 };
                    RetargetPair { target: t, source: (s_joints[k], s_joints[k + 1]), alpha }
                };
                pair_of[t] = Some(pairs.len());
                pairs.push(pair);
            }
        }
        if pairs.is_empty() {
            return Err(RetargetError::NoSharedChains);
        }
        renames.sort();
        renames.dedup_by(|a, b| a.0 == b.0);

        let hips = match (&source_rig.hips, &target_rig.hips) {
            (Some(s), Some(t)) => {
                let (s, t) = (find(source, s, "source")?, find(target, t, "target")?);
                Some((s, t, Vec3::from(source_model[s].translation), Vec3::from(target_model[t].translation)))
            }
            _ => None,
        };
        let scale = match hips {
            Some((_, _, ps, pt)) if ps.y.abs() > 1e-4 && pt.y.abs() > 1e-4 => pt.y / ps.y,
            _ => 1.0,
        };
        Ok(Self {
            source: source.clone(),
            target: target.clone(),
            pairs,
            pair_of,
            source_inv: source_model.iter().map(|m| affine_rotation(m).inverse()).collect(),
            target_rot: target_model.iter().map(affine_rotation).collect(),
            target_pose,
            hips,
            source_model,
            target_model,
            scale,
            ik,
            renames,
        })
    }

    pub fn source(&self) -> &Skeleton {
        &self.source
    }

    pub fn target(&self) -> &Skeleton {
        &self.target
    }

    pub fn pairs(&self) -> &[RetargetPair] {
        &self.pairs
    }

    /// Target hips height over source hips height.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// The target's retarget pose (local).
    pub fn target_retarget_pose(&self) -> &[Trs] {
        &self.target_pose
    }

    /// Retargets one local pose of the source skeleton. `ik_weights` holds
    /// one weight per IK chain (missing entries count as 1).
    pub fn pose(&self, source_local: &[Trs], ik_weights: &[f32], out: &mut [Trs]) {
        let (n_s, n_t) = (self.source.len(), self.target.len());
        assert!(source_local.len() == n_s && out.len() == n_t, "pose of a different skeleton");
        let mut s_model = Vec::with_capacity(n_s);
        model_of(&self.source, source_local, &mut s_model);
        let delta = |s: usize| (affine_rotation(&s_model[s]) * self.source_inv[s]).normalize();
        let mut t_model: Vec<Affine3A> = Vec::with_capacity(n_t);
        for (j, o) in out.iter_mut().enumerate() {
            let parent = self.target.parent(j).map_or(Affine3A::IDENTITY, |p| t_model[p]);
            let mut local = self.target_pose[j];
            if let Some(p) = self.pair_of[j] {
                let pair = self.pairs[p];
                let d =
                    if pair.source.0 == pair.source.1 { delta(pair.source.0) } else { nlerp(delta(pair.source.0), delta(pair.source.1), pair.alpha) };
                local.rotation = (affine_rotation(&parent).inverse() * d * self.target_rot[j]).normalize();
            }
            if let Some((hs, ht, ps, pt)) = self.hips
                && ht == j
            {
                let p = pt + (Vec3::from(s_model[hs].translation) - ps) * self.scale;
                local.translation = parent.inverse().transform_point3(p);
            }
            *o = local;
            t_model.push(parent * local.to_affine());
        }
        if self.ik.is_empty() {
            return;
        }
        let mut lp = LocalPose::from_trs(out);
        let mut mp = ModelPose { mats: t_model };
        for (i, chain) in self.ik.iter().enumerate() {
            let w = ik_weights.get(i).copied().unwrap_or(1.0).clamp(0.0, 1.0);
            if w <= 0.0 {
                continue;
            }
            let c = *chain.target.last().expect("IK chains have joints");
            let s = chain.source_end;
            let d = Vec3::from(s_model[s].translation) - Vec3::from(self.source_model[s].translation);
            let goal = Vec3::from(self.target_model[c].translation) + Vec3::new(d.x * self.scale, d.y * chain.limb_scale, d.z * self.scale);
            let span = chain.solver.span(chain.target.len());
            let pole = limb_pole(&mp, &chain.target[chain.target.len() - span..], None);
            let goal = LimbGoal { position: goal, rotation: None, pole, weight: w };
            solve_limb(&self.target, &mut lp, &mut mp, &Affine3A::IDENTITY, &chain.target, chain.solver, &goal);
        }
        for (j, o) in out.iter_mut().enumerate() {
            *o = lp.get(j);
        }
    }

    /// Retargets uniformly sampled frames: poses, root motion (scaled),
    /// curves and markers (contact names follow the joints), events, meta.
    pub fn frames(&self, clip: &FrameClip) -> Result<FrameClip, RetargetError> {
        let (n_s, n_t) = (self.source.len(), self.target.len());
        if clip.frames.len() != clip.frame_count * n_s {
            return Err(RetargetError::BadFrames);
        }
        if clip.meta.additive {
            return Err(RetargetError::Additive);
        }
        let contact: Vec<Option<usize>> = self
            .ik
            .iter()
            .map(|c| {
                let name = format!("contact.{}", self.source.name(c.source_end));
                clip.curves.iter().position(|k| k.name == name)
            })
            .collect();
        let mut frames = vec![Trs::IDENTITY; clip.frame_count * n_t];
        let mut weights = vec![1.0; self.ik.len()];
        for f in 0..clip.frame_count {
            for (w, c) in weights.iter_mut().zip(&contact) {
                *w = c.map_or(1.0, |c| clip.curves[c].values.get(f).copied().unwrap_or(0.0));
            }
            self.pose(&clip.frames[f * n_s..(f + 1) * n_s], &weights, &mut frames[f * n_t..(f + 1) * n_t]);
        }
        let s = self.scale;
        let mut out = FrameClip {
            name: clip.name.clone(),
            rate: clip.rate,
            duration: clip.duration,
            frame_count: clip.frame_count,
            frames,
            root_motion: clip.root_motion.clone(),
            curves: clip.curves.clone(),
            events: clip.events.clone(),
            markers: clip.markers.clone(),
            loop_range: clip.loop_range,
            meta: clip.meta.clone(),
        };
        if let Some(rm) = &mut out.root_motion {
            for f in &mut rm.frames {
                f[0] *= s;
                f[1] *= s;
                f[2] *= s;
            }
        }
        for c in &mut out.curves {
            for prefix in ["contact.", "roll."] {
                if let Some(n) = c.name.strip_prefix(prefix).and_then(|j| self.rename(j)) {
                    c.name = format!("{prefix}{n}");
                    break;
                }
            }
        }
        for m in &mut out.markers {
            if let Some(n) = m.name.strip_prefix("down.").and_then(|j| self.rename(j)) {
                m.name = format!("down.{n}");
            }
        }
        out.meta.average_velocity *= s;
        out.meta.stance_velocity *= s;
        out.meta.cycle_distance *= s;
        out.meta.unbound_joints.clear();
        Ok(out)
    }

    /// [`Self::frames`] with root motion generated from the target pelvis
    /// (module docs, step 4). The clip must carry no root motion (its
    /// source cooked with [`RootMotionRules::extract`] off): its motion is
    /// in the pose, in the take's space. With `rules.extract` off (or no
    /// axis kept) the clip is only re-based.
    pub fn frames_with_root(&self, clip: &FrameClip, rules: &RootMotionRules) -> Result<FrameClip, RetargetError> {
        let Some((_, ht, _, pt)) = self.hips else {
            return Err(RetargetError::NoHips);
        };
        if clip.root_motion.is_some() {
            return Err(RetargetError::SourceRootMotion);
        }
        let mut out = self.frames(clip)?;
        let (n_t, n) = (self.target.len(), out.frame_count);
        out.meta.root_motion_mode = rules.mode;
        if n == 0 {
            return Ok(out);
        }
        // Frame 0's hips ground frame: ground position less the retarget
        // pose's ground offset, heading against the retarget-pose rotation.
        let up = ancestors(&self.target, ht);
        let parent_of = |pose: &[Trs]| up.iter().fold(Affine3A::IDENTITY, |m, &j| m * pose[j].to_affine());
        let m0 = parent_of(&out.frames[..n_t]) * out.frames[ht].to_affine();
        let heading = Quat::from_rotation_y(twist_angle(affine_rotation(&m0) * self.target_rot[ht].inverse(), Vec3::Y));
        let ground = |p: Vec3| Vec3::new(p.x, 0.0, p.z);
        let origin = ground(Vec3::from(m0.translation)) - heading * ground(pt);
        let anchor = Affine3A::from_rotation_translation(heading, origin).inverse();
        for pose in out.frames.chunks_exact_mut(n_t) {
            let parent = parent_of(pose);
            let t = Trs::from_affine(&(parent.inverse() * anchor * parent * pose[ht].to_affine()));
            pose[ht].translation = t.translation;
            pose[ht].rotation = t.rotation;
        }
        if rules.extract && (rules.horizontal || rules.vertical || rules.yaw || rules.attitude) {
            out.root_motion = Some(extract_root_motion(&self.target, &mut out.frames, n, ht, rules));
        }
        Ok(out)
    }

    /// [`Self::clip`] with root motion generated from the target pelvis
    /// ([`Self::frames_with_root`]); the root meta (velocity, turn rate,
    /// distance) is measured on the new track.
    pub fn clip_with_root(&self, clip: &Clip, rules: &RootMotionRules, settings: &CookSettings) -> Result<Clip, RetargetError> {
        if clip.skeleton != self.source.id() {
            return Err(RetargetError::SkeletonMismatch);
        }
        let frames = self.frames_with_root(&FrameClip::from_clip(clip), rules)?;
        let mut out = cook_frames(&self.target, &frames, settings)?;
        if out.root_motion.is_some() {
            root_meta(&mut out);
        }
        Ok(out)
    }

    fn rename(&self, source_joint: &str) -> Option<&str> {
        self.renames.binary_search_by(|(s, _)| s.as_str().cmp(source_joint)).ok().map(|i| self.renames[i].1.as_str())
    }

    /// Retargets a cooked clip and cooks the result for the target.
    pub fn clip(&self, clip: &Clip, settings: &CookSettings) -> Result<Clip, RetargetError> {
        if clip.skeleton != self.source.id() {
            return Err(RetargetError::SkeletonMismatch);
        }
        let frames = self.frames(&FrameClip::from_clip(clip))?;
        Ok(cook_frames(&self.target, &frames, settings)?)
    }
}

/// Retargets `clip` from `source` to `target` (both with their rigs) and
/// cooks it for the target.
pub fn retarget_clip(
    source: &Skeleton,
    source_rig: &RetargetRig,
    target: &Skeleton,
    target_rig: &RetargetRig,
    clip: &Clip,
    settings: &CookSettings,
) -> Result<Clip, RetargetError> {
    Retargeter::new(source, source_rig, target, target_rig)?.clip(clip, settings)
}
