//! Full-body IK (animation.md "Layer 2: IK, from limbs to full body").
//!
//! A position-based solver in the shape of UE5's Full Body IK (`PBIK`):
//! one particle per body joint (girdles, spine, neck, head, limb joints,
//! from the rig profile), effector goals pinned hard, and per joint a
//! stiffness that keeps its bones near their animated directions. Soft
//! constraints take compliance-weighted PBD steps under-relaxed by
//! `RELAXATION` (XPBD multipliers, Macklin et al. 2016, need far more
//! than 10 iterations at these compliances and full steps oscillate); the
//! goals are met exactly by the post-passes, not by convergence. Bone
//! lengths are hard PBD constraints (Müller et al., *Position Based
//! Dynamics*, 2007), solved last in each iteration.
//!
//! - **Pre-pull** (UE's `PrePull` root behavior): before iterating, the body
//!   moves by the average stretch of the effectors that are out of their
//!   chain's reach, so far reaches converge in a few iterations.
//! - **Root anchor**: the root particle (the lowest common ancestor of the
//!   body, usually the pelvis) is held softly at its animated position;
//!   with feet pinned it shifts within the legs' reach.
//! - **Hinges**: every interior joint of a limb's solved span (knee, elbow,
//!   hock) stays on its animated side of the line through its neighbours
//!   (or on the side of the limb's pole hint when the animation is
//!   straight), so a limb can straighten but never flips.
//! - **Rotations** are recovered top-down from the solved positions: each
//!   joint takes the swing from its animated child direction to the solved
//!   one (the animated twist about the bone is kept), joints with several
//!   children average the frames through each secondary child, and the
//!   joint above a hinge aligns the bend plane so knees and elbows turn
//!   about their hinge only. Each joint's local rotation change is then
//!   clamped to its limit, so limits hold exactly in the output.
//!
//! The solve is a pure, sequential function of its inputs: identical on any
//! thread count and continuous in the targets (no jitter frame to frame).

use glam::{Affine3A, Quat, Vec3, Vec3A};
use serde::{Deserialize, Serialize};

use super::clamp_angle;
use crate::math::{affine_rotation, align_hemisphere, angle_between, rotation_between_frames};
use crate::rig::{PoleHint, RigProfile};
use crate::{LocalPose, ModelPose, Skeleton};

/// No particle.
pub const NO_JOINT: u16 = u16::MAX;

/// Gauss-Seidel relaxation of the soft constraints: a full step (1.0)
/// makes the stiff torso and the hard constraints trade the same error back
/// and forth every iteration (a period-2 oscillation); half a step damps it.
const RELAXATION: f32 = 0.5;

/// A bone of the body solve: child and parent particles, their inverse
/// masses, and the stiffness step (relaxation over mass plus compliance).
#[derive(Clone, Copy, Debug)]
struct Link {
    i: usize,
    p: usize,
    wi: f32,
    wp: f32,
    step: f32,
}

/// What a body joint is, for its defaults (stiffness, limit, mass).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyRole {
    /// The lowest common ancestor of the body (usually the pelvis).
    Root,
    /// Spine and girdle joints, and joints linking body parts.
    Spine,
    /// Neck and head.
    Neck,
    /// Limb joints before the solver's span (clavicle, scapula).
    LimbBase,
    /// First joint of a limb's solved span (hip, shoulder).
    LimbRoot,
    /// Interior joints of a solved span (knee, elbow, hock).
    LimbMid,
    /// The limb's effector (foot, hand, hoof).
    LimbEnd,
}

impl BodyRole {
    /// Default (compliance, limit, inverse mass) of a joint with this role.
    fn defaults(self) -> (f32, f32, f32) {
        use std::f32::consts::PI;
        match self {
            Self::Root => (0.01, 45f32.to_radians(), 0.25),
            Self::Spine => (0.01, 30f32.to_radians(), 0.5),
            Self::Neck => (0.03, 50f32.to_radians(), 0.5),
            Self::LimbBase => (0.02, 30f32.to_radians(), 0.5),
            Self::LimbRoot => (5.0, 120f32.to_radians(), 1.0),
            Self::LimbMid => (5.0, 150f32.to_radians(), 1.0),
            Self::LimbEnd => (5.0, PI, 1.0),
        }
    }

    /// Below a limb root: solved by the limb pass, not by the particles.
    fn is_passive(self) -> bool {
        matches!(self, Self::LimbMid | Self::LimbEnd)
    }

    fn is_limb_chain(self) -> bool {
        matches!(self, Self::LimbMid | Self::LimbEnd)
    }
}

/// A hinge (knee, elbow, hock): the joint stays on one side of the line
/// from `prev` to `next` (particle indices).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hinge {
    pub prev: u16,
    pub next: u16,
    /// Bend direction for a straight animated limb: (joint, axis in that
    /// joint's frame), from the profile's [`PoleHint::Axis`].
    pub pole: Option<(u16, Vec3)>,
}

/// One particle of the body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyJoint {
    pub joint: u16,
    /// Parent particle ([`NO_JOINT`] for the root).
    pub parent: u16,
    /// Child particles.
    pub children: Vec<u16>,
    /// The child that orients the joint's frame (the largest sub-body).
    pub primary: u16,
    pub role: BodyRole,
    /// XPBD compliance of this joint's rotation: how easily the bones to its
    /// children leave their animated directions (0 = rigid).
    pub compliance: f32,
    /// Largest change of the joint's local rotation, radians.
    pub limit: f32,
    /// Inverse mass: how much this particle moves against its neighbours.
    pub inv_mass: f32,
    pub hinge: Option<Hinge>,
}

/// An IK goal for one body joint (model space of the pose).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effector {
    pub joint: usize,
    pub position: Vec3,
    /// Model rotation of the joint, if it should be matched.
    pub rotation: Option<Quat>,
    /// Blend from the animated pose toward the goal, 0–1.
    pub weight: f32,
    /// Share of this effector's out-of-reach stretch that pre-pulls the
    /// whole body (0 for planted feet, 1 for a reaching hand).
    pub pull: f32,
}

/// A limb of the final pass.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BodyLimb {
    /// Skeleton joints, root first.
    joints: Vec<usize>,
    solver: super::LimbSolver,
    pole: Option<(u16, Vec3)>,
}

/// Whole-body settings.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullBodySettings {
    /// XPBD compliance of the root's anchor to its animated position.
    pub root_compliance: f32,
    /// Comfortable reach of a limb as a fraction of its full reach: past it
    /// the body moves instead of the limb locking straight.
    pub reach: f32,
}

impl Default for FullBodySettings {
    fn default() -> Self {
        Self {
            root_compliance: 1.0,
            reach: 0.95,
        }
    }
}

/// The full-body solver of one (skeleton, rig profile): built once, shared
/// by every character of the skeleton.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullBody {
    /// Particles in skeleton order (parents first; the root is index 0).
    pub joints: Vec<BodyJoint>,
    pub settings: FullBodySettings,
    /// Skeleton joint → particle ([`NO_JOINT`] outside the body).
    lookup: Vec<u16>,
    /// Limbs solved exactly after the body, and per particle its limb ([`NO_JOINT`]: none).
    limbs: Vec<BodyLimb>,
    limb_of: Vec<u16>,
    /// Particles the body solve moves (not the root; limb joints below the
    /// limb root are left to the limb pass).
    active: Vec<u16>,
}

/// Per-character working memory of [`FullBody::solve`] (reuse it to avoid
/// allocating every update).
#[derive(Clone, Debug, Default)]
pub struct FullBodyScratch {
    anim: Vec<Vec3A>,
    pos: Vec<Vec3A>,
    /// Animated bone to the parent, its length and direction (zero: none).
    rest: Vec<Vec3A>,
    length: Vec<f32>,
    dir: Vec<Vec3A>,
    /// The animated bone turned with the parent's swing, this iteration.
    expected: Vec<Vec3A>,
    /// (sin, cos) of the limit of a joint whose children are cone-limited in the solve.
    cone: Vec<Option<(f32, f32)>>,
    inv_mass: Vec<f32>,
    side: Vec<Vec3A>,
    rot: Vec<Quat>,
    placed: Vec<Vec3A>,
    anim_local: Vec<Quat>,
    swing: Vec<Quat>,
    goal: Vec<Option<(Quat, f32)>>,
    /// Per joint, the child on the path to a pinned non-limb effector ([`NO_JOINT`]: none).
    prio: Vec<u16>,
    links: Vec<Link>,
    targets: Vec<(u16, Vec3A, f32)>,
    spheres: Vec<(u16, Vec3A, f32, bool)>,
}

fn idx(j: u16) -> usize {
    usize::from(j)
}

fn u16_of(i: usize) -> u16 {
    u16::try_from(i).expect("fewer than 65535 joints")
}

impl FullBody {
    /// Builds the body from the profile: girdles, spine, neck and head, and
    /// every solvable limb, closed up to their common ancestor. `None` when
    /// the profile has fewer than two such joints.
    #[allow(clippy::too_many_lines)]
    pub fn new(skeleton: &Skeleton, profile: &RigProfile) -> Option<Self> {
        let n = skeleton.len();
        let mut role: Vec<Option<BodyRole>> = vec![None; n];
        let mut add = |j: u16, r: BodyRole| {
            if let Some(slot) = role.get_mut(idx(j))
                && slot.is_none()
            {
                *slot = Some(r);
            }
        };
        for l in profile
            .limbs
            .iter()
            .filter(|l| l.solver != super::LimbSolver::None)
        {
            let len = l.joints.len();
            let start = len - l.solver.span(len);
            for (k, &j) in l.joints.iter().enumerate() {
                add(
                    j,
                    match k {
                        _ if k < start => BodyRole::LimbBase,
                        _ if k == len - 1 => BodyRole::LimbEnd,
                        _ if k == start => BodyRole::LimbRoot,
                        _ => BodyRole::LimbMid,
                    },
                );
            }
        }
        for g in &profile.girdles {
            add(g.joint, BodyRole::Spine);
        }
        for &j in profile.spine.iter().flat_map(|c| &c.joints) {
            add(j, BodyRole::Spine);
        }
        if let Some(nh) = &profile.neck_head {
            for &j in &nh.neck.joints {
                add(j, BodyRole::Neck);
            }
            add(nh.head, BodyRole::Neck);
        }
        // Close up to the lowest common ancestor.
        let members: Vec<usize> = (0..n).filter(|&j| role[j].is_some()).collect();
        let &first = members.first()?;
        let mut lca = first;
        for &j in &members {
            while !skeleton.subtree(lca).contains(&j) {
                lca = skeleton.parent(lca)?;
            }
        }
        for &j in &members {
            let mut k = j;
            while k != lca {
                k = skeleton.parent(k)?;
                if role[k].is_none() {
                    role[k] = Some(BodyRole::Spine);
                }
            }
        }
        role[lca] = Some(BodyRole::Root);
        let mut lookup = vec![NO_JOINT; n];
        let mut joints = Vec::new();
        for j in lca..n {
            let Some(r) = role[j] else { continue };
            if !skeleton.subtree(lca).contains(&j) {
                continue;
            }
            lookup[j] = u16_of(joints.len());
            let parent = if j == lca {
                NO_JOINT
            } else {
                skeleton.parent(j).map_or(NO_JOINT, |p| lookup[p])
            };
            let (compliance, limit, inv_mass) = r.defaults();
            joints.push(BodyJoint {
                joint: u16_of(j),
                parent,
                children: Vec::new(),
                primary: NO_JOINT,
                role: r,
                compliance,
                limit,
                inv_mass,
                hinge: None,
            });
        }
        if joints.len() < 2 {
            return None;
        }
        // Children, and sub-body sizes (children come after parents).
        let mut size = vec![1usize; joints.len()];
        for i in (1..joints.len()).rev() {
            if joints[i].role.is_passive() {
                continue;
            }
            let p = idx(joints[i].parent);
            size[p] += size[i];
            joints[p].children.insert(0, u16_of(i));
        }
        let active: Vec<u16> = (1..joints.len())
            .filter(|&i| !joints[i].role.is_passive())
            .map(u16_of)
            .collect();
        for b in &mut joints {
            b.primary = b
                .children
                .iter()
                .copied()
                .max_by_key(|&c| (size[idx(c)], std::cmp::Reverse(c)))
                .unwrap_or(NO_JOINT);
        }
        // Hinges: interior joints of each solved span.
        for l in profile
            .limbs
            .iter()
            .filter(|l| l.solver != super::LimbSolver::None)
        {
            let len = l.joints.len();
            let span = &l.joints[len - l.solver.span(len)..];
            let pole = match l.pole {
                PoleHint::Axis(a) => Some((profile.girdle_joint(l), a)),
                PoleHint::Animated => None,
            };
            for w in span.windows(3) {
                let (a, m, c) = (lookup[idx(w[0])], lookup[idx(w[1])], lookup[idx(w[2])]);
                if a != NO_JOINT && m != NO_JOINT && c != NO_JOINT && joints[idx(m)].hinge.is_none()
                {
                    joints[idx(m)].hinge = Some(Hinge {
                        prev: a,
                        next: c,
                        pole,
                    });
                }
            }
        }
        // Limbs for the final pass, by their end particle.
        let mut limbs = Vec::new();
        let mut limb_of = vec![NO_JOINT; joints.len()];
        for l in profile
            .limbs
            .iter()
            .filter(|l| l.solver != super::LimbSolver::None)
        {
            let end = lookup[idx(l.effector)];
            if end == NO_JOINT
                || limb_of[idx(end)] != NO_JOINT
                || l.joints.last() != Some(&l.effector)
            {
                continue;
            }
            limb_of[idx(end)] = u16_of(limbs.len());
            let pole = match l.pole {
                PoleHint::Axis(a) => Some((profile.girdle_joint(l), a)),
                PoleHint::Animated => None,
            };
            limbs.push(BodyLimb {
                joints: l.joints.iter().map(|&j| idx(j)).collect(),
                solver: l.solver,
                pole,
            });
        }
        Some(Self {
            joints,
            settings: FullBodySettings::default(),
            lookup,
            limbs,
            limb_of,
            active,
        })
    }

    /// The particle of a skeleton joint, if it is part of the body.
    pub fn particle(&self, joint: usize) -> Option<usize> {
        self.lookup
            .get(joint)
            .filter(|&&p| p != NO_JOINT)
            .map(|&p| idx(p))
    }

    /// The body joint of a skeleton joint, for tuning its stiffness, limit
    /// or mass.
    pub fn joint_mut(&mut self, joint: usize) -> Option<&mut BodyJoint> {
        let p = self.particle(joint)?;
        self.joints.get_mut(p)
    }

    /// Where a reaching chain starts and its length: walks up from particle
    /// `i` through limb and neck joints to the limb root or the torso.
    fn chain_base(&self, i: usize, anim: &[Vec3A]) -> (usize, f32) {
        let (mut k, mut reach) = (i, 0.0);
        loop {
            let b = &self.joints[k];
            if b.parent == NO_JOINT || !(b.role.is_limb_chain() || b.role == BodyRole::Neck) {
                return (k, reach);
            }
            let p = idx(b.parent);
            reach += anim[k].distance(anim[p]);
            k = p;
        }
    }

    /// Solves the effectors on the pose (model space under `root`) with
    /// `iterations` (10 is the spec's budget point), writing the local pose
    /// and refreshing the model pose of the body's subtree. Effectors on
    /// joints outside the body are ignored. Returns the largest remaining
    /// distance of an effector to its (weighted) goal.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn solve(
        &self,
        skeleton: &Skeleton,
        local: &mut LocalPose,
        model: &mut ModelPose,
        root: &Affine3A,
        effectors: &[Effector],
        iterations: usize,
        scratch: &mut FullBodyScratch,
    ) -> f32 {
        let n = self.joints.len();
        let sc = scratch;
        sc.anim.clear();
        sc.anim.extend(
            self.joints
                .iter()
                .map(|b| model.mats[idx(b.joint)].translation),
        );
        sc.pos.clone_from(&sc.anim);
        sc.rest.clear();
        sc.rest.extend(self.joints.iter().enumerate().map(|(i, b)| {
            if i == 0 {
                Vec3A::ZERO
            } else {
                sc.anim[i] - sc.anim[idx(b.parent)]
            }
        }));
        sc.length.clear();
        sc.length.extend(sc.rest.iter().map(|r| r.length()));
        sc.dir.clear();
        sc.dir.extend(sc.rest.iter().map(|r| r.normalize_or_zero()));
        sc.expected.clear();
        sc.expected.resize(n, Vec3A::ZERO);
        sc.cone.clear();
        sc.cone.extend(self.joints.iter().map(|b| {
            (b.parent != NO_JOINT
                && matches!(
                    b.role,
                    BodyRole::Spine | BodyRole::Neck | BodyRole::LimbBase
                ))
            .then(|| b.limit.sin_cos())
        }));
        sc.inv_mass.clear();
        sc.inv_mass.extend(self.joints.iter().map(|b| b.inv_mass));
        sc.swing.clear();
        sc.swing.resize(n, Quat::IDENTITY);
        sc.anim_local.clear();
        sc.anim_local
            .extend(self.joints.iter().map(|b| local.get(idx(b.joint)).rotation));
        sc.goal.clear();
        sc.goal.resize(n, None);
        sc.targets.clear();
        for e in effectors {
            let w = e.weight.clamp(0.0, 1.0);
            let Some(i) = self.particle(e.joint).filter(|_| w > 0.0) else {
                continue;
            };
            let target = sc.anim[i].lerp(Vec3A::from(e.position), w);
            sc.targets.push((u16_of(i), target, e.pull.max(0.0)));
            if let Some(q) = e.rotation {
                sc.goal[i] = Some((q, w));
            }
        }
        if sc.targets.is_empty() {
            return 0.0;
        }
        let settled = sc
            .targets
            .iter()
            .all(|&(i, t, _)| t.distance(sc.anim[idx(i)]) < 1e-5)
            && sc.goal.iter().zip(&self.joints).all(|(g, b)| {
                g.is_none_or(|(q, w)| angle_between(model.rotation(idx(b.joint)), q) * w < 1e-5)
            });
        if settled {
            return 0.0;
        }

        // Pre-pull: the body moves by the mean out-of-reach stretch.
        let mut pull = Vec3A::ZERO;
        for &(i, target, share) in &sc.targets {
            if share > 0.0 {
                let (base, reach) = self.chain_base(idx(i), &sc.anim);
                let d = target - sc.anim[base];
                let dist = d.length();
                if dist > reach && dist > 1e-6 {
                    pull += d * ((dist - reach) / dist * share.min(1.0));
                }
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let pull = pull / sc.targets.len() as f32;
        if pull.length_squared() > 1e-12 {
            for p in &mut sc.pos {
                *p += pull;
            }
        }
        // Limb ends constrain their limb's root to a sphere of comfortable
        // reach around the goal (the limb pass then solves the limb exactly);
        // other effectors (a head) are pinned hard.
        sc.spheres.clear();
        for &(i, target, share) in &sc.targets {
            let l = self.limb_of[idx(i)];
            if l == NO_JOINT {
                sc.pos[idx(i)] = target;
                sc.inv_mass[idx(i)] = 0.0;
                continue;
            }
            let limb = &self.limbs[idx(l)];
            let span = &limb.joints[limb.joints.len() - limb.solver.span(limb.joints.len())..];
            let Some(r) = span.first().and_then(|&j| self.particle(j)) else {
                continue;
            };
            let radius = limb_reach(model, span, limb.solver) * self.settings.reach.clamp(0.0, 1.0);
            sc.spheres.push((u16_of(r), target, radius, share <= 0.0));
        }
        // Reaching limbs first, planted ones last (they win a conflict).
        sc.spheres.sort_by_key(|s| s.3);
        // Joints on the path to a pinned non-limb effector (a head) turn to
        // that child exactly in the rotation recovery.
        sc.prio.clear();
        sc.prio.resize(n, NO_JOINT);
        for &(i, _, _) in &sc.targets {
            let mut k = i;
            while self.limb_of[idx(i)] == NO_JOINT && self.joints[idx(k)].parent != NO_JOINT {
                let p = self.joints[idx(k)].parent;
                sc.prio[idx(p)] = k;
                k = p;
            }
        }

        // Hinge sides in the animated pose (zero: no defined side).
        sc.side.clear();
        sc.side.resize(n, Vec3A::ZERO);
        for (m, b) in self.joints.iter().enumerate() {
            let Some(h) = b.hinge else { continue };
            let (a, c) = (sc.anim[idx(h.prev)], sc.anim[idx(h.next)]);
            let line = (c - a).normalize_or_zero();
            let off = sc.anim[m] - a;
            let perp = off - line * off.dot(line);
            let chain = sc.length[m] + sc.length[idx(h.next)];
            sc.side[m] = if perp.length() > 0.02 * chain {
                perp.normalize()
            } else if let Some((j, axis)) = h.pole {
                let dir = Vec3A::from(model.rotation(idx(j)) * axis);
                (dir - line * dir.dot(line)).normalize_or_zero()
            } else {
                Vec3A::ZERO
            };
        }

        // The bones of the iteration with their masses, after the pins.
        sc.links.clear();
        for i in self.active.iter().map(|&a| idx(a)) {
            let p = idx(self.joints[i].parent);
            let (wi, wp) = (sc.inv_mass[i], sc.inv_mass[p]);
            if wi + wp > 0.0 {
                let step = RELAXATION / (wi + wp + self.joints[p].compliance.max(0.0));
                sc.links.push(Link { i, p, wi, wp, step });
            }
        }
        let root_alpha = self.settings.root_compliance.max(0.0);
        let w0 = sc.inv_mass[0];
        let root_step = if w0 + root_alpha > 0.0 {
            w0 / (w0 + root_alpha)
        } else {
            0.0
        };
        for _ in 0..iterations {
            // Root anchor (soft).
            let c = sc.pos[0] - sc.anim[0];
            sc.pos[0] -= c * root_step;
            // Each joint's frame turned with its own bone (swing only; the
            // root keeps its animated frame, leaves need none).
            for p in self.active.iter().map(|&a| idx(a)) {
                let b = &self.joints[p];
                if b.children.is_empty() {
                    continue;
                }
                let a = sc.dir[p];
                sc.swing[p] = match (sc.pos[p] - sc.pos[idx(b.parent)]).try_normalize() {
                    Some(d) if a != Vec3A::ZERO => Quat::from_rotation_arc(a.into(), d.into()),
                    _ => Quat::IDENTITY,
                };
            }
            // Joint stiffness (soft): bones keep their animated directions in
            // their parent's turned frame.
            for &Link { i, p, wi, wp, step } in &sc.links {
                let expected = sc.swing[p].mul_vec3a(sc.rest[i]);
                sc.expected[i] = expected;
                let dl = (expected - (sc.pos[i] - sc.pos[p])) * step;
                sc.pos[i] += dl * wi;
                sc.pos[p] -= dl * wp;
            }
            // Torso limits in the solve, so a capped joint hands the rest of
            // the bend to its neighbours: the bone to each child stays
            // within the parent joint's limit of its animated direction
            // turned with the parent's own bone (swing only).
            for &Link { i, p, wi, .. } in &sc.links {
                let Some((sin, cos)) = sc.cone[p] else {
                    continue;
                };
                if wi <= 0.0 || sc.length[i] <= 0.0 {
                    continue;
                }
                let e = sc.expected[i] / sc.length[i];
                let now = sc.pos[i] - sc.pos[p];
                let (along, dist2) = (now.dot(e), now.length_squared());
                // Outside the cone (no square root unless it is): along < cos·dist.
                let outside = if cos >= 0.0 {
                    along < 0.0 || along * along < cos * cos * dist2
                } else {
                    along < 0.0 && along * along > cos * cos * dist2
                };
                if outside && let Some(perp) = (now - e * along).try_normalize() {
                    sc.pos[i] = sc.pos[p] + (e * cos + perp * sin) * dist2.sqrt();
                }
            }
            // Limb roots within reach of their goals (hard inequalities).
            for &(r, target, radius, _) in &sc.spheres {
                let d = sc.pos[idx(r)] - target;
                let dist = d.length();
                if dist > radius {
                    sc.pos[idx(r)] = target + d * (radius / dist);
                }
            }
            // Bone lengths (hard), swept down then up the tree so a
            // correction crosses the whole body in one iteration.
            for &Link { i, p, wi, wp, .. } in sc.links.iter().rev() {
                let d = sc.pos[i] - sc.pos[p];
                let len = d.length();
                if len > 1e-9 {
                    let corr = d * ((len - sc.length[i]) / (len * (wi + wp)));
                    sc.pos[i] -= corr * wi;
                    sc.pos[p] += corr * wp;
                }
            }
        }

        self.recover(skeleton, local, model, root, sc);
        self.refresh(skeleton, local, model, root, idx(self.joints[0].joint));
        // The particles placed the pinned non-limb effectors (a head)
        // exactly; what the recovery lost along the chain is mostly radial,
        // which rotations cannot remove: the body translates by its mean.
        let mut residual = Vec3A::ZERO;
        let mut count = 0.0f32;
        for &(i, target, _) in &sc.targets {
            if self.limb_of[idx(i)] == NO_JOINT {
                residual += target - model.mats[idx(self.joints[idx(i)].joint)].translation;
                count += 1.0;
            }
        }
        if count > 0.0 {
            self.shift_root(skeleton, local, model, root, residual / count);
        }
        // Planted limbs (pull 0) must reach their pins: the root translates
        // by the worst shortfall (exact for one limb), a few rounds.
        for _ in 0..4 {
            let mut worst = Vec3A::ZERO;
            for &(i, target, share) in &sc.targets {
                let l = self.limb_of[idx(i)];
                if share > 0.0 || l == NO_JOINT {
                    continue;
                }
                let limb = &self.limbs[idx(l)];
                let span = &limb.joints[limb.joints.len() - limb.solver.span(limb.joints.len())..];
                let Some(&hip) = span.first() else { continue };
                let reach = limb_reach(model, span, limb.solver);
                let d = target - model.mats[hip].translation;
                let dist = d.length();
                if dist - reach > worst.length() {
                    worst = d * ((dist - reach) / dist);
                }
            }
            if worst.length_squared() < 1e-10 {
                break;
            }
            self.shift_root(skeleton, local, model, root, worst);
        }
        // Limited CCD up each effector's chain to the root's child, removing
        // what the recovery left: a head to its goal, a reaching limb's root
        // to within comfortable reach of its goal (limbs land exactly below).
        for &(i, target, share) in &sc.targets {
            let l = self.limb_of[idx(i)];
            if l == NO_JOINT {
                self.polish_chain(skeleton, local, model, root, sc, idx(i), target, 0.0);
            } else if share > 0.0 {
                let limb = &self.limbs[idx(l)];
                let span = &limb.joints[limb.joints.len() - limb.solver.span(limb.joints.len())..];
                let Some(r) = span.first().and_then(|&j| self.particle(j)) else {
                    continue;
                };
                let radius =
                    limb_reach(model, span, limb.solver) * self.settings.reach.clamp(0.0, 1.0);
                self.polish_chain(skeleton, local, model, root, sc, r, target, radius);
            }
        }
        // Limb pass (as Final IK's full-body biped maps limbs after the
        // body): each pinned limb end solves exactly with its limb's own
        // solver from the placed girdle, knee or elbow toward its solved side.
        for &(i, target, _) in &sc.targets {
            let l = self.limb_of[idx(i)];
            if l == NO_JOINT {
                continue;
            }
            let limb = &self.limbs[idx(l)];
            let end = *limb.joints.last().expect("limbs have joints");
            // The limb's own rotation as recovered (goal rotations are applied there).
            let rotation = Some(model.rotation(end));
            let span = &limb.joints[limb.joints.len() - limb.solver.span(limb.joints.len())..];
            // Pole: the knee's animated side turned with the limb, which is
            // continuous in the goal (the solved pose's own bend can vanish).
            let side = span
                .get(1)
                .and_then(|&k| self.particle(k))
                .map_or(Vec3A::ZERO, |k| sc.side[k]);
            let pole = match (
                span.first().and_then(|&j| self.particle(j)),
                side != Vec3A::ZERO,
            ) {
                (Some(h), true) => {
                    let hip = model.mats[span[0]].translation;
                    let (u0, u) = (sc.anim[idx(i)] - sc.anim[h], target - hip);
                    let turn = match (u0.try_normalize(), u.try_normalize()) {
                        (Some(a), Some(b)) => Quat::from_rotation_arc(a.into(), b.into()),
                        _ => Quat::IDENTITY,
                    };
                    let knee = model.mats[span[1]].translation;
                    Vec3::from(knee)
                        + turn * Vec3::from(side) * limb_reach(model, span, limb.solver).max(0.1)
                }
                _ => super::limb_pole(
                    model,
                    span,
                    limb.pole.map(|(j, a)| model.rotation(idx(j)) * a),
                ),
            };
            let goal = super::LimbGoal {
                position: target.into(),
                rotation,
                pole,
                weight: 1.0,
            };
            super::solve_limb(
                skeleton,
                local,
                model,
                root,
                &limb.joints,
                limb.solver,
                &goal,
            );
            // Limits hold over the limb pass too (it turns only its span;
            // a limit of a half turn or more never binds).
            for &j in span {
                if let Some(k) = self
                    .particle(j)
                    .filter(|&k| self.joints[k].limit < std::f32::consts::PI)
                {
                    self.clamp_joint(skeleton, local, model, root, sc, k);
                }
            }
        }
        // The joints outside the body (fingers, jaw, leaf twists) follow once.
        self.refresh_rest(skeleton, local, model, root);
        let mut error = 0.0f32;
        for &(i, target, _) in &sc.targets {
            error = error.max(
                model.mats[idx(self.joints[idx(i)].joint)]
                    .translation
                    .distance(target),
            );
        }
        error
    }

    /// Refreshes the model transforms of the particles in `from`'s subtree
    /// only (the body is closed up to its root, so a particle's parent is a
    /// particle or above the body). Joints outside the body (fingers) wait
    /// for [`Self::refresh_rest`], once per solve.
    fn refresh(
        &self,
        skeleton: &Skeleton,
        local: &LocalPose,
        model: &mut ModelPose,
        root: &Affine3A,
        from: usize,
    ) {
        for j in skeleton.subtree(from) {
            if self.lookup[j] != NO_JOINT {
                let parent = skeleton.parent(j).map_or(*root, |p| model.mats[p]);
                model.mats[j] = parent * local.get(j).to_affine();
            }
        }
    }

    /// Refreshes the joints below the body root that are not particles,
    /// from their already refreshed parents (parents come first).
    fn refresh_rest(
        &self,
        skeleton: &Skeleton,
        local: &LocalPose,
        model: &mut ModelPose,
        root: &Affine3A,
    ) {
        for j in skeleton.subtree(idx(self.joints[0].joint)) {
            if self.lookup[j] == NO_JOINT {
                let parent = skeleton.parent(j).map_or(*root, |p| model.mats[p]);
                model.mats[j] = parent * local.get(j).to_affine();
            }
        }
    }

    /// Translates the body's root joint by `delta` (model space).
    fn shift_root(
        &self,
        skeleton: &Skeleton,
        local: &mut LocalPose,
        model: &mut ModelPose,
        root: &Affine3A,
        delta: Vec3A,
    ) {
        if delta.length_squared() < 1e-12 {
            return;
        }
        let j0 = idx(self.joints[0].joint);
        let lin = skeleton
            .parent(j0)
            .map_or(root.matrix3, |p| model.mats[p].matrix3);
        let t = local.get(j0).translation;
        local.set_translation(j0, t + Vec3::from(lin.inverse() * delta));
        self.refresh(skeleton, local, model, root, j0);
    }
    /// Clamps particle `k`'s local rotation change (against its animated
    /// local rotation) to its limit.
    fn clamp_joint(
        &self,
        skeleton: &Skeleton,
        local: &mut LocalPose,
        model: &mut ModelPose,
        root: &Affine3A,
        sc: &FullBodyScratch,
        k: usize,
    ) {
        let j = idx(self.joints[k].joint);
        let anim = sc.anim_local[k];
        let mut d = (local.get(j).rotation * anim.inverse()).normalize();
        if d.w < 0.0 {
            d = -d;
        }
        if 2.0 * d.w.min(1.0).acos() > self.joints[k].limit + 1e-5 {
            local.set_rotation(j, (clamp_angle(d, self.joints[k].limit) * anim).normalize());
            self.refresh(skeleton, local, model, root, j);
        }
    }

    /// Particle `k`'s local rotation `q` with its change from the animated
    /// local rotation clamped to the joint's limit.
    fn clamped(&self, sc: &FullBodyScratch, k: usize, q: Quat) -> Quat {
        let anim = sc.anim_local[k];
        let mut d = (q * anim.inverse()).normalize();
        if d.w < 0.0 {
            d = -d;
        }
        (clamp_angle(d, self.joints[k].limit) * anim).normalize()
    }

    /// One CCD sweep from particle `i`'s parent up to the root's
    /// child (at most 16 joints), each joint clamped to its limit, until
    /// `i` is within `radius` (+1 mm) of `target`. Only the path to `i` is
    /// refreshed per step; the subtree once at the end.
    #[allow(clippy::too_many_arguments)]
    fn polish_chain(
        &self,
        skeleton: &Skeleton,
        local: &mut LocalPose,
        model: &mut ModelPose,
        root: &Affine3A,
        sc: &FullBodyScratch,
        i: usize,
        target: Vec3A,
        radius: f32,
    ) {
        // Skeleton joints from `i` up to the root's child (no allocation).
        let (mut path, mut len) = ([0usize; 16], 0);
        let mut k = u16_of(i);
        while k != NO_JOINT && self.joints[idx(k)].parent != NO_JOINT && len < path.len() {
            path[len] = idx(k);
            len += 1;
            k = self.joints[idx(k)].parent;
        }
        if len < 2 {
            return;
        }
        let joint = |m: usize| idx(self.joints[path[m]].joint);
        let end = joint(0);
        let parent_mat =
            |model: &ModelPose, j: usize| skeleton.parent(j).map_or(*root, |p| model.mats[p]);
        // The nearest point within `radius` of the target.
        let mut moved = false;
        let aim = |model: &ModelPose| {
            let d = model.mats[end].translation - target;
            let dist = d.length();
            (dist > radius + 1e-3).then(|| {
                if radius > 0.0 {
                    target + d * (radius / dist)
                } else {
                    target
                }
            })
        };
        // Stops at 1 mm, or when a pass gains under 1 mm (limits bind).
        let mut last = f32::INFINITY;
        for _ in 0..1 {
            for (m, &k) in path[..len].iter().enumerate().skip(1) {
                let Some(goal) = aim(model) else { break };
                let j = joint(m);
                let pivot = model.mats[j].translation;
                let (Some(a), Some(b)) = (
                    (model.mats[end].translation - pivot).try_normalize(),
                    (goal - pivot).try_normalize(),
                ) else {
                    continue;
                };
                let pm = parent_mat(model, j);
                let parent_rot = affine_rotation(&pm);
                let delta = Quat::from_rotation_arc(a.into(), b.into());
                let q = self.clamped(
                    sc,
                    k,
                    parent_rot.inverse() * delta * parent_rot * local.get(j).rotation,
                );
                local.set_rotation(j, q);
                moved = true;
                model.mats[j] = pm * local.get(j).to_affine();
                for n in (0..m).rev() {
                    let c = joint(n);
                    model.mats[c] = model.mats[joint(n + 1)] * local.get(c).to_affine();
                }
            }
            let gap = model.mats[end].translation.distance(target) - radius;
            if gap <= 1e-3 || last - gap < 1e-3 {
                break;
            }
            last = gap;
        }
        if moved {
            self.refresh(skeleton, local, model, root, joint(len - 1));
        }
    }

    /// Top-down rotation recovery from the solved particles (see the module
    /// docs), with each joint's local change clamped to its limit.
    fn recover(
        &self,
        skeleton: &Skeleton,
        local: &mut LocalPose,
        model: &ModelPose,
        root: &Affine3A,
        sc: &mut FullBodyScratch,
    ) {
        let n = self.joints.len();
        sc.rot.clear();
        sc.rot.resize(n, Quat::IDENTITY);
        sc.placed.clear();
        sc.placed.resize(n, Vec3A::ZERO);
        for i in 0..n {
            let b = &self.joints[i];
            if b.role.is_passive() {
                continue;
            }
            let j = idx(b.joint);
            let mut trs = local.get(j);
            let (parent_rot, parent_pos, parent_lin) = if b.parent == NO_JOINT {
                let m = skeleton.parent(j).map_or(*root, |p| model.mats[p]);
                (affine_rotation(&m), m.translation, m.matrix3)
            } else {
                let p = idx(b.parent);
                (
                    sc.rot[p],
                    sc.placed[p],
                    glam::Mat3A::from_quat(sc.rot[p]) * scale_of(model, idx(self.joints[p].joint)),
                )
            };
            if b.parent == NO_JOINT {
                let delta = sc.pos[0] - sc.anim[0];
                trs.translation += Vec3::from(parent_lin.inverse() * delta);
            }
            let x = parent_pos + parent_lin * Vec3A::from(trs.translation);
            sc.placed[i] = x;
            let anim_local = trs.rotation;
            let new_rot = match sc.goal[i] {
                Some((q, w)) => {
                    let anim_rot = model.rotation(j);
                    Some(anim_rot.slerp(align_hemisphere(q, anim_rot), w))
                }
                None => self.swing(i, x, sc).map(|q| q * model.rotation(j)),
            };
            if let Some(new_rot) = new_rot {
                let mut d = (parent_rot.inverse() * new_rot * anim_local.inverse()).normalize();
                if d.w < 0.0 {
                    d = -d;
                }
                trs.rotation = (clamp_angle(d, b.limit) * anim_local).normalize();
            }
            sc.rot[i] = (parent_rot * trs.rotation).normalize();
            local.set(j, trs);
        }
    }

    /// The model-space swing taking particle `i`'s animated children to
    /// their solved positions, from its placed position `x`; `None` for a
    /// leaf (it keeps its local rotation).
    fn swing(&self, i: usize, x: Vec3A, sc: &FullBodyScratch) -> Option<Quat> {
        let b = &self.joints[i];
        if b.primary == NO_JOINT {
            return None;
        }
        let c = idx(if sc.prio[i] == NO_JOINT {
            b.primary
        } else {
            sc.prio[i]
        });
        let from = sc.anim[c] - sc.anim[i];
        let to = sc.pos[c] - x;
        let (Some(f), Some(t)) = (from.try_normalize(), to.try_normalize()) else {
            return Some(Quat::IDENTITY);
        };
        // The joint above a hinge aligns its bend plane.
        if let Some(h) = self.joints[c].hinge
            && idx(h.prev) == i
            && sc.side[c] != Vec3A::ZERO
        {
            let line = (sc.pos[idx(h.next)] - x).normalize_or_zero();
            let off = sc.pos[c] - x;
            let now = off - line * off.dot(line);
            // Only a clear bend on the animated side defines the plane.
            let chain = sc.length[c] + sc.length[idx(h.next)];
            let turned = Quat::from_rotation_arc(f.into(), t.into());
            if now.length() > 0.02 * chain
                && now.dot(Vec3A::from(turned * Vec3::from(sc.side[c]))) > 0.0
            {
                return Some(rotation_between_frames(
                    f.into(),
                    sc.side[c].into(),
                    t.into(),
                    now.into(),
                ));
            }
        }
        if b.children.len() == 1 || sc.prio[i] != NO_JOINT {
            return Some(Quat::from_rotation_arc(f.into(), t.into()));
        }
        // Several children: the rotation best fitting every child direction
        // (Müller et al., "A Robust Method to Extract the Rotational Part of
        // Deformations", MIG 2016), warm-started from the primary's swing.
        let mut cov = glam::Mat3A::ZERO;
        for &k in &b.children {
            let k = idx(k);
            if let (Some(a), Some(c)) = (
                (sc.anim[k] - sc.anim[i]).try_normalize(),
                (sc.pos[k] - x).try_normalize(),
            ) {
                cov += glam::Mat3A::from_cols(c * a.x, c * a.y, c * a.z);
            }
        }
        let mut q = Quat::from_rotation_arc(f.into(), t.into());
        for _ in 0..4 {
            let r = glam::Mat3A::from_quat(q);
            let omega = r.x_axis.cross(cov.x_axis)
                + r.y_axis.cross(cov.y_axis)
                + r.z_axis.cross(cov.z_axis);
            let omega = omega
                / ((r.x_axis.dot(cov.x_axis)
                    + r.y_axis.dot(cov.y_axis)
                    + r.z_axis.dot(cov.z_axis))
                .abs()
                    + 1e-9);
            let w = omega.length();
            if w < 1e-6 {
                break;
            }
            q = (Quat::from_axis_angle((omega / w).into(), w) * q).normalize();
        }
        Some(q)
    }
}

/// Uniform scale of a joint's model matrix (rotations assume uniform scale).
fn scale_of(model: &ModelPose, joint: usize) -> f32 {
    model.mats[joint].matrix3.x_axis.length()
}

/// Reach of a limb's solved span, slightly short of straight. A
/// `TwoBoneAnkle` leg reaches as thigh plus the shin-to-toe virtual bone.
fn limb_reach(model: &ModelPose, span: &[usize], solver: super::LimbSolver) -> f32 {
    let p = |j: usize| model.mats[j].translation;
    let length = match (solver, span) {
        (super::LimbSolver::TwoBoneAnkle, &[a, b, _, d]) => {
            p(a).distance(p(b)) + p(b).distance(p(d))
        }
        _ => span
            .windows(2)
            .map(|w| p(w[0]).distance(p(w[1])))
            .sum::<f32>(),
    };
    length * 0.999
}
