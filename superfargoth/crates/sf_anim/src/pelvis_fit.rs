//! [`ClipRules::pelvis_fit`](crate::ClipRules::pelvis_fit): fit a seated
//! clip to a seat height at cook time.
//!
//! A generated or captured sit keeps its pelvis at the height of the seat it
//! was made on. This rule lifts (or lowers) the pelvis so the seated hub sits
//! at `target_m`, then re-solves each leg with two-bone IK so the feet keep
//! their pre-fit model positions and rotations: the offline counterpart of a
//! retargeter's pelvis offset with leg IK goals (UE5 IK Retargeter "Pelvis"
//! offset + leg goals). Runtime foot IK then only absorbs the last few cm.
//!
//! **Weights.** `h(f)` is the pelvis height above the floor (model-space y)
//! of frame `f`; `L = target_m - h(hub)`. When the clip never rises more
//! than `still_m` above the hub (a seated loop), every frame gets `L`.
//! Otherwise frame `f` gets `w(f)·L`, `w = clamp((h_stand - h) / (h_stand -
//! h_hub), 0, 1)` with `h_stand = max h`: standing frames are untouched
//! (bit for bit), the hub gets exactly `L`, frames below the hub get `L`.
//!
//! **Where it runs.** In `anim.cook` on the frames as sampled from the source,
//! before root-motion extraction (the lift moves the pelvis only, never the
//! root-motion joint; a root-motion joint at or below the pelvis must not
//! extract vertical motion), and on any [`FrameClip`] through
//! [`fit_frame_clip`] (a retargeted clip on its target rig, whose pelvis
//! height includes the root track's vertical channel). Out of reach (a seat
//! too high for the legs): the solver clamps at its maximum extension, the
//! feet leave their contacts and [`PelvisFitStats::max_foot_err_m`] says by
//! how much.

use glam::{Affine3A, Vec3};
use serde::{Deserialize, Serialize};
use sf_tunables::{Def, Lifetime, Net, Scope, Source, def};

use crate::ik::{LimbGoal, LimbSolver, limb_pole, solve_limb};
use crate::rig::LimbKind;
use crate::{FrameClip, LocalPose, ModelPose, RigProfile, Skeleton, Trs, local_to_model, update_subtree};

/// The end of the clip that holds the seated hub pose.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HubEnd {
    /// Exit pieces (stand up from the seat) and loops start on the hub.
    #[default]
    First,
    /// Enter pieces (sit down) end on the hub.
    Last,
}

/// Seat-height fit of a seated clip (module docs).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PelvisFit {
    /// Pelvis height above the floor at the hub frame, meters (the seat
    /// height plus the rig's seated pelvis offset).
    pub target_m: f32,
    pub hub: HubEnd,
    /// Below this rise above the hub, meters, the clip counts as seated
    /// throughout (a loop) and every frame gets the whole lift.
    /// Default [`STILL_M`].
    #[serde(default = "default_still_m")]
    pub still_m: f32,
}

/// The default [`PelvisFit::still_m`]: a seated idle's breathing and weight
/// shifts bob the pelvis a few cm, a stand-up raises it tens of cm.
pub const STILL_M: Def<f32> = def!(float "anim.cook.pelvis_fit.still_m" = 0.05)
    .range(0.0, 0.3)
    .help("Pelvis rise above the seated hub, m, below which a pelvis-fitted clip counts as seated throughout (every frame gets the whole seat-height lift)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Presentation)
    .lifetime(Lifetime::Bake);

fn default_still_m() -> f32 {
    STILL_M.default_value()
}

impl PelvisFit {
    /// A fit to `target_m` with the default [`Self::still_m`].
    pub fn new(target_m: f32, hub: HubEnd) -> Self {
        Self { target_m, hub, still_m: default_still_m() }
    }
}

/// What a pelvis fit did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PelvisFitStats {
    /// `L`: the hub frame's lift, meters (negative lowers).
    pub lift_m: f32,
    /// Largest distance, over frames and legs, between a foot after IK and
    /// its pre-fit position, meters (0 up to rounding when every frame is
    /// in reach).
    pub max_foot_err_m: f32,
    /// Frames given a non-zero lift.
    pub frames_lifted: u32,
}

/// Why no pelvis fit was applied (the clip is then cooked unfitted).
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum PelvisFitError {
    #[error("no rig profile with support legs on one girdle")]
    NoLegs,
    #[error("pelvis fit target {0} m is not a finite height")]
    BadTarget(f32),
}

/// One leg a fit re-solves.
#[derive(Clone, Debug, PartialEq)]
pub struct FitLeg {
    /// Root first, foot (the effector) last.
    pub joints: Vec<usize>,
    pub solver: LimbSolver,
    /// Knee direction for a straight leg, in the pelvis joint's frame.
    pub pole_axis: Option<Vec3>,
}

/// The pelvis and the legs it carries.
#[derive(Clone, Debug, PartialEq)]
pub struct FitRig {
    pub pelvis: usize,
    pub legs: Vec<FitLeg>,
}

impl FitRig {
    /// From a rig profile: the girdle of the hind-most support legs (the
    /// pelvis of a biped or a quadruped) and its support legs.
    pub fn from_profile(profile: &RigProfile) -> Option<Self> {
        let is_leg = |l: &&crate::rig::Limb| l.support && matches!(l.kind, LimbKind::Leg(_)) && l.joints.len() >= 2;
        let hind = profile.limbs.iter().filter(is_leg).max_by_key(|l| l.order)?;
        let girdle = hind.girdle;
        let legs: Vec<FitLeg> = profile
            .limbs
            .iter()
            .filter(is_leg)
            .filter(|l| l.girdle == girdle)
            .map(|l| FitLeg {
                joints: l.joints.iter().map(|&j| usize::from(j)).collect(),
                solver: l.solver,
                pole_axis: match l.pole {
                    crate::rig::PoleHint::Axis(a) => Some(a),
                    crate::rig::PoleHint::Animated => None,
                },
            })
            .collect();
        Some(Self { pelvis: usize::from(profile.girdle_joint(hind)), legs })
    }
}

/// The lift of every pose, from the clip's pelvis heights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FitPlan {
    lift: f32,
    stand: f32,
    hub: f32,
    /// Seated throughout: every pose gets the whole lift.
    still: bool,
}

impl FitPlan {
    /// `heights`: the pelvis height of each frame, in order (the hub is the
    /// first or the last).
    pub(crate) fn new(heights: &[f32], fit: &PelvisFit) -> Result<Self, PelvisFitError> {
        if !fit.target_m.is_finite() {
            return Err(PelvisFitError::BadTarget(fit.target_m));
        }
        let hub = match fit.hub {
            HubEnd::First => heights.first(),
            HubEnd::Last => heights.last(),
        }
        .copied()
        .unwrap_or(0.0);
        let stand = heights.iter().copied().fold(hub, f32::max);
        // A rise of (nearly) zero is still too: the weight has no span then.
        let still = stand - hub < fit.still_m.max(f32::EPSILON);
        Ok(Self { lift: fit.target_m - hub, stand, hub, still })
    }

    pub(crate) fn lift(&self) -> f32 {
        self.lift
    }

    /// The share of the lift a pose with pelvis height `h` gets.
    pub(crate) fn weight(&self, h: f32) -> f32 {
        if self.still { 1.0 } else { ((self.stand - h) / (self.stand - self.hub)).clamp(0.0, 1.0) }
    }
}

/// Pelvis height of a local pose (model space, `offset` added: a root
/// track's vertical channel).
pub(crate) fn pelvis_height(skeleton: &Skeleton, rig: &FitRig, local: &[Trs], offset: f32) -> f32 {
    // The pelvis's model translation, through its ancestors only.
    let mut chain = vec![rig.pelvis];
    while let Some(k) = chain.last().and_then(|&j| skeleton.parent(j)) {
        chain.push(k);
    }
    let m = chain.iter().rev().fold(Affine3A::IDENTITY, |m, &k| m * local[k].to_affine());
    m.translation.y + offset
}

/// Fits one local pose: lifts the pelvis by `plan`'s share for its height
/// and re-solves the legs. Returns the weight and the largest foot error.
/// A pose with weight 0 is left untouched.
pub(crate) fn fit_pose(skeleton: &Skeleton, rig: &FitRig, plan: &FitPlan, local: &mut [Trs], offset: f32) -> (f32, f32) {
    let w = plan.weight(pelvis_height(skeleton, rig, local, offset));
    let dy = w * plan.lift;
    if w <= 0.0 || dy.abs() < f32::MIN_POSITIVE {
        return (w, 0.0);
    }
    let root = Affine3A::IDENTITY;
    let mut lp = LocalPose::from_trs(local);
    let mut mp = ModelPose::new(skeleton.len());
    local_to_model(skeleton, &lp, &mut mp, &root);
    let goals: Vec<(Vec3, glam::Quat)> = rig
        .legs
        .iter()
        .map(|l| {
            let end = *l.joints.last().expect("legs have joints");
            (mp.translation(end), mp.rotation(end))
        })
        .collect();
    // Lift the pelvis in model space: its parent (and the root-motion
    // joint) stay where they are.
    let p = rig.pelvis;
    let parent = skeleton.parent(p).map_or(root, |q| mp.mats[q]);
    let lifted = mp.translation(p) + Vec3::Y * dy;
    lp.set_translation(p, parent.inverse().transform_point3(lifted));
    update_subtree(skeleton, &lp, &mut mp, &root, p);
    let mut err = 0.0f32;
    for (leg, &(position, rotation)) in rig.legs.iter().zip(&goals) {
        let span = leg.solver.span(leg.joints.len()).max(2).min(leg.joints.len());
        let solved = &leg.joints[leg.joints.len() - span..];
        let axis = leg.pole_axis.map(|a| mp.rotation(p) * a);
        let pole = limb_pole(&mp, solved, axis);
        let goal = LimbGoal { position, rotation: Some(rotation), pole, weight: 1.0 };
        err = err.max(solve_limb(skeleton, &mut lp, &mut mp, &root, &leg.joints, leg.solver, &goal));
    }
    local[p] = lp.get(p);
    for leg in &rig.legs {
        for &j in &leg.joints {
            local[j] = lp.get(j);
        }
    }
    (w, err)
}

/// Fits `n` frames of `skeleton.len()` local poses each (`offsets`: per
/// frame, added to the pelvis height; empty for none).
pub(crate) fn fit_frames(
    skeleton: &Skeleton,
    rig: &FitRig,
    frames: &mut [Trs],
    offsets: &[f32],
    fit: &PelvisFit,
) -> Result<PelvisFitStats, PelvisFitError> {
    let joints = skeleton.len();
    let offset = |f: usize| offsets.get(f).copied().unwrap_or(0.0);
    let heights: Vec<f32> = frames.chunks_exact(joints).enumerate().map(|(f, pose)| pelvis_height(skeleton, rig, pose, offset(f))).collect();
    let plan = FitPlan::new(&heights, fit)?;
    let mut stats = PelvisFitStats { lift_m: plan.lift, ..PelvisFitStats::default() };
    for (f, pose) in frames.chunks_exact_mut(joints).enumerate() {
        let (w, err) = fit_pose(skeleton, rig, &plan, pose, offset(f));
        if w > 0.0 {
            stats.frames_lifted += 1;
        }
        stats.max_foot_err_m = stats.max_foot_err_m.max(err);
    }
    Ok(stats)
}

/// Fits a [`FrameClip`] (a retargeted clip on its target rig, before it is
/// cooked with [`crate::cook_frames`]). The root track is untouched; its
/// vertical channel counts in the pelvis height. Not for additive clips
/// (their frames are deltas): callers skip those.
pub fn fit_frame_clip(skeleton: &Skeleton, profile: &RigProfile, clip: &mut FrameClip, fit: &PelvisFit) -> Result<PelvisFitStats, PelvisFitError> {
    let rig = FitRig::from_profile(profile).filter(|r| !r.legs.is_empty()).ok_or(PelvisFitError::NoLegs)?;
    let offsets: Vec<f32> = clip.root_motion.as_ref().map(|t| t.frames.iter().map(|f| f[1]).collect()).unwrap_or_default();
    fit_frames(skeleton, &rig, &mut clip.frames, &offsets, fit)
}
