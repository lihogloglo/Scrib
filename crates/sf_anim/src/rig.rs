//! Rig profiles (rig-profile.md): a skeleton described as girdles, a
//! spine, N limbs with a solver each, a neck and head, tails and extra
//! chains, so every procedural feature works on any morphology. The
//! humanoid map (`sf_character`) is an optional layer that produces one;
//! [`detect_profile`] finds one from the rest pose for any skeleton (the
//! `rig.profile` bake step).
//!
//! Character space: Y up, facing +Z, +X left (the `sf_anim` convention).

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::ik::LimbSolver;
use crate::math::affine_rotation;
use crate::{FormatError, Skeleton, SkeletonId, tag};

/// Which defaults a skeleton gets (graph templates, mover mode, contact
/// thresholds). Ops never branch on it: they read limbs and girdles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RigTemplate {
    Biped,
    Quadruped,
    /// Six or more support legs (insects, spiders, centipedes).
    Multiped,
    /// Wings (birds stand on two legs and still fly).
    Flyer,
    Floater,
    Swimmer,
    Serpent,
    Rolling,
    /// Nothing the rules recognized: root motion and look-at only.
    #[default]
    Custom,
}

/// Side of a limb, from its rest position (+X is left).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
    #[default]
    Center,
}

/// How a leg meets the ground.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegKind {
    /// Heel down: humans, bears (three joints).
    Plantigrade,
    /// On the toes, hock above the ground: dogs, guars, birds (four joints).
    Digitigrade,
    /// On hooves: horses.
    Unguligrade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LimbKind {
    Leg(LegKind),
    Arm,
    Wing,
    Fin,
    Tentacle,
}

/// Where the knee or elbow bends when the animated chain gives no
/// direction (a straight rest pose, a fully extended leg).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum PoleHint {
    /// The animated bend only.
    #[default]
    Animated,
    /// A direction in the girdle joint's local frame, used when the
    /// animated chain is nearly straight.
    Axis(Vec3),
}

/// The ground-contact end of a support limb.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FootSpec {
    /// Toe joint beyond the effector (bends to the ground), if single.
    pub toe: Option<u16>,
    /// Effector height above the ground in the rest pose, meters.
    pub sole: f32,
    /// Largest tilt to the ground normal, radians.
    pub max_tilt: f32,
    /// Contact curve name (`contact.<effector>`, made by `anim.cook`).
    pub contact: String,
}

/// One limb: a joint chain from the girdle's child to the effector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Limb {
    /// Pairs limbs across rigs and graphs (`leg_fl`, `leftLeg`, `wing_l`).
    pub name: String,
    pub kind: LimbKind,
    pub side: Side,
    /// Front to back, 0 = the front pair.
    pub order: u8,
    /// Index into [`RigProfile::girdles`].
    pub girdle: u8,
    /// Root first, effector last.
    pub joints: Vec<u16>,
    pub effector: u16,
    pub solver: LimbSolver,
    pub pole: PoleHint,
    pub foot: Option<FootSpec>,
    /// Carries weight: foot placement, contacts, stride warping.
    pub support: bool,
    /// Rest-pose length of the chain (sum of segments), meters.
    pub length: f32,
}

/// A pelvis or shoulder girdle and the limbs it carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Girdle {
    pub joint: u16,
    /// Indices into [`RigProfile::limbs`].
    pub limbs: Vec<u8>,
}

/// A named joint chain, root first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Chain {
    pub name: String,
    pub joints: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeckHead {
    pub neck: Chain,
    pub head: u16,
    /// The head's aim axis in its local frame (rest-pose +Z).
    pub forward: Vec3,
    pub eyes: Vec<u16>,
}

/// Sizes derived at bake; thresholds in meters scale by these.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Measures {
    /// Vertical extent of the rest pose.
    pub height: f32,
    /// Root girdle above the lowest joint.
    pub hip_height: f32,
    /// Front-to-back extent of the rest pose.
    pub body_length: f32,
    /// Left-to-right extent of the support effectors.
    pub body_width: f32,
    /// Mean support-limb length (0 without support limbs).
    pub leg_length: f32,
}

/// A skeleton's limb-chain description (the `rig.profile` product).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RigProfile {
    pub skeleton: SkeletonId,
    pub template: RigTemplate,
    pub root_motion: Option<u16>,
    /// Root-most first (each later girdle is usually a descendant).
    pub girdles: Vec<Girdle>,
    /// Root girdle → neck base, root first.
    pub spine: Option<Chain>,
    pub neck_head: Option<NeckHead>,
    pub limbs: Vec<Limb>,
    /// Tails, antennae, tentacles not used for support.
    pub tails: Vec<Chain>,
    /// Anything else (jaw, ears, skirt joints).
    pub extra: Vec<Chain>,
    /// Left/right joint pairs (the `Mirror` node's table).
    pub mirror: Vec<(u16, u16)>,
    pub measures: Measures,
}

/// One limb as a detector or an adapter describes it, before
/// [`RigProfile::build`] fills in side, order, pole, foot and length.
#[derive(Clone, Debug, PartialEq)]
pub struct LimbDesc {
    /// Generated from kind, side and order when `None`.
    pub name: Option<String>,
    pub kind: LimbKind,
    pub joints: Vec<u16>,
    /// The girdle joint (usually the parent of `joints[0]`).
    pub girdle: u16,
    pub solver: LimbSolver,
    /// Bend direction in character space; derived from the rest pose when `None`.
    pub pole: Option<Vec3>,
    pub toe: Option<u16>,
    pub support: bool,
    pub side: Option<Side>,
}

/// Everything [`RigProfile::build`] needs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProfileParts {
    pub template: RigTemplate,
    pub root_motion: Option<u16>,
    pub limbs: Vec<LimbDesc>,
    pub spine: Option<Chain>,
    pub neck_head: Option<NeckHead>,
    pub tails: Vec<Chain>,
    pub extra: Vec<Chain>,
}

/// An adapter's hint for one limb, by joint names.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LimbHint {
    pub name: String,
    /// Root first, effector last.
    pub joints: Vec<String>,
    pub kind: Option<LimbKind>,
    pub solver: Option<LimbSolver>,
    /// Defaults to whether the kind is a leg.
    pub support: Option<bool>,
    /// Bend direction in character space.
    pub pole: Option<Vec3>,
}

/// Adapter hints for detection (`AnimRules::rig_hints`). Part of the bake key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RigHints {
    pub template: Option<RigTemplate>,
    /// Limbs given explicitly; detection skips their joints.
    pub limbs: Vec<LimbHint>,
    /// Extra named chains `(name, joints)`.
    pub extra: Vec<(String, Vec<String>)>,
    /// Support effectors lie within this fraction of the rest height above
    /// the lowest joint (default 0.05).
    pub support_height: Option<f32>,
    /// A complete profile (a reviewed `*.rigprofile.ron` override): used as is.
    pub profile: Option<RigProfile>,
}

impl RigHints {
    /// Stable bytes for the bake key.
    pub fn key_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("in-memory serialization")
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum RigError {
    #[error("rig hints name joint {0:?}, which the skeleton lacks")]
    UnknownJoint(String),
    #[error("limb {0:?}: its joints are not a parent-to-child chain")]
    BadChain(String),
    #[error("the profile was made for another skeleton")]
    Mismatch,
    #[error("the profile names joint index {0}, out of range")]
    BadIndex(usize),
}

const MAGIC: &[u8; 4] = b"SFRP";
/// Version of the cooked rig-profile format.
pub const RIG_PROFILE_FORMAT: u32 = 1;

fn u16_of(i: usize) -> u16 {
    u16::try_from(i).expect("joint index fits u16")
}

fn u8_of(i: usize) -> u8 {
    u8::try_from(i).unwrap_or(u8::MAX)
}

fn horizontal_len(v: Vec3) -> f32 {
    Vec3::new(v.x, 0.0, v.z).length()
}

fn depth(skeleton: &Skeleton, mut j: usize) -> usize {
    let mut d = 0;
    while let Some(p) = skeleton.parent(j) {
        d += 1;
        j = p;
    }
    d
}

/// Joints from `from` down to `to`, both included (`from` must be an ancestor).
fn path(skeleton: &Skeleton, from: usize, to: usize) -> Vec<usize> {
    let mut out = vec![to];
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
    out.reverse();
    out
}

impl RigProfile {
    /// Fills in girdles, sides, orders, names, poles, feet, mirror pairs and
    /// measures from the rest pose.
    #[allow(clippy::too_many_lines)]
    pub fn build(skeleton: &Skeleton, parts: ProfileParts) -> Self {
        let model = skeleton.rest_model();
        let pos: Vec<Vec3> = model.iter().map(|m| Vec3::from(m.translation)).collect();
        let rot: Vec<Quat> = model.iter().map(affine_rotation).collect();
        let (min, max) = pos.iter().fold((Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)), |(a, b), p| (a.min(*p), b.max(*p)));
        let extent = (max - min).max(Vec3::ZERO);
        let height = extent.y.max(1e-3);

        // Girdles, root-most first.
        let mut girdle_joints: Vec<u16> = parts.limbs.iter().map(|l| l.girdle).collect();
        girdle_joints.sort_by_key(|&g| (depth(skeleton, usize::from(g)), g));
        girdle_joints.dedup();
        let root_girdle = parts.limbs.iter().filter(|l| l.support).map(|l| l.girdle).min_by_key(|&g| (depth(skeleton, usize::from(g)), g));

        let seg_len = |joints: &[u16]| joints.windows(2).map(|w| pos[usize::from(w[0])].distance(pos[usize::from(w[1])])).sum::<f32>();
        let support_lengths: Vec<f32> = parts.limbs.iter().filter(|l| l.support).map(|l| seg_len(&l.joints)).collect();
        #[allow(clippy::cast_precision_loss)]
        let leg_length = if support_lengths.is_empty() { 0.0 } else { support_lengths.iter().sum::<f32>() / support_lengths.len() as f32 };
        let support_x: Vec<f32> = parts.limbs.iter().filter(|l| l.support).filter_map(|l| l.joints.last()).map(|&e| pos[usize::from(e)].x).collect();
        let body_width = support_x.iter().copied().fold(f32::NEG_INFINITY, f32::max) - support_x.iter().copied().fold(f32::INFINITY, f32::min);
        let measures = Measures {
            height,
            hip_height: root_girdle.map_or(0.0, |g| pos[usize::from(g)].y - min.y),
            body_length: extent.z,
            body_width: if support_x.len() >= 2 { body_width } else { 0.0 },
            leg_length,
        };

        // Sides, then orders (front to back) per support class.
        let side_of = |l: &LimbDesc| {
            l.side.unwrap_or_else(|| {
                let e = pos[usize::from(*l.joints.last().unwrap_or(&l.girdle))];
                let g = pos[usize::from(l.girdle)];
                let dx = e.x - g.x;
                if dx > 0.02 * height {
                    Side::Left
                } else if dx < -0.02 * height {
                    Side::Right
                } else {
                    Side::Center
                }
            })
        };
        let sides: Vec<Side> = parts.limbs.iter().map(side_of).collect();
        let mut orders = vec![0u8; parts.limbs.len()];
        let tolerance = 0.1 * extent.z.max(0.25 * height);
        for support in [true, false] {
            let mut idx: Vec<usize> = (0..parts.limbs.len()).filter(|&i| parts.limbs[i].support == support).collect();
            let z = |i: usize| pos[usize::from(parts.limbs[i].joints.first().copied().unwrap_or(parts.limbs[i].girdle))].z;
            idx.sort_by(|&a, &b| z(b).total_cmp(&z(a)));
            let mut order = 0usize;
            let mut anchor = idx.first().map_or(0.0, |&i| z(i));
            for &i in &idx {
                if anchor - z(i) > tolerance {
                    order += 1;
                    anchor = z(i);
                }
                orders[i] = u8_of(order);
            }
        }

        let mut limbs = Vec::with_capacity(parts.limbs.len());
        for (i, d) in parts.limbs.iter().enumerate() {
            let girdle = u8_of(girdle_joints.iter().position(|&g| g == d.girdle).unwrap_or(0));
            let side = sides[i];
            let effector = d.joints.last().copied().unwrap_or(d.girdle);
            let span = d.solver.span(d.joints.len());
            let solved = &d.joints[d.joints.len() - span..];
            // Pole: the rest bend, else a default per kind, in girdle space.
            let rest_bend = (span >= 3).then(|| {
                let (a, k, e) = (pos[usize::from(solved[0])], pos[usize::from(solved[1])], pos[usize::from(solved[span - 1])]);
                let line = e - a;
                let off = k - a;
                let side = off - line * (off.dot(line) / line.length_squared().max(1e-12));
                (side.length() > 0.02 * seg_len(solved)).then(|| side.normalize())
            });
            let default_axis = match d.kind {
                // Insect knees up; fins and tentacles curl up.
                LimbKind::Leg(_) if parts.template == RigTemplate::Multiped => Vec3::Y,
                LimbKind::Fin | LimbKind::Tentacle => Vec3::Y,
                // Hind (root-girdle) and biped knees forward.
                LimbKind::Leg(_) if Some(d.girdle) == root_girdle || parts.template == RigTemplate::Biped => Vec3::Z,
                // Elbows and front-leg elbows backward.
                LimbKind::Leg(_) | LimbKind::Arm | LimbKind::Wing => Vec3::NEG_Z,
            };
            let axis = d.pole.or(rest_bend.flatten()).unwrap_or(default_axis);
            let pole = if span >= 3 { PoleHint::Axis(rot[usize::from(d.girdle)].inverse() * axis) } else { PoleHint::Animated };
            let foot = d.support.then(|| FootSpec {
                toe: d.toe,
                sole: pos[usize::from(effector)].y - min.y,
                max_tilt: 30f32.to_radians(),
                contact: format!("contact.{}", skeleton.name(usize::from(effector))),
            });
            let name = d.name.clone().unwrap_or_else(|| {
                let kind = match d.kind {
                    LimbKind::Leg(_) => "leg",
                    LimbKind::Arm => "arm",
                    LimbKind::Wing => "wing",
                    LimbKind::Fin => "fin",
                    LimbKind::Tentacle => "tentacle",
                };
                let s = match side {
                    Side::Left => "l",
                    Side::Right => "r",
                    Side::Center => "c",
                };
                // Rows of this class (same kind and support): one row names
                // by side, two support rows front/back, more by row index.
                let same =
                    |k: usize| parts.limbs[k].support == d.support && std::mem::discriminant(&parts.limbs[k].kind) == std::mem::discriminant(&d.kind);
                let rows = (0..parts.limbs.len()).filter(|&k| same(k)).map(|k| orders[k]).max().unwrap_or(0);
                match rows {
                    0 => format!("{kind}_{s}"),
                    1 if d.support => {
                        format!("{kind}_{}{s}", if orders[i] == 0 { "f" } else { "b" })
                    }
                    _ => format!("{kind}_{s}{}", orders[i]),
                }
            });
            limbs.push(Limb {
                name,
                kind: d.kind,
                side,
                order: orders[i],
                girdle,
                joints: d.joints.clone(),
                effector,
                solver: d.solver,
                pole,
                foot,
                support: d.support,
                length: seg_len(&d.joints),
            });
        }
        // Unique names: later duplicates get a numeric suffix.
        for i in 0..limbs.len() {
            let mut k = 2;
            while limbs[..i].iter().any(|l| l.name == limbs[i].name) {
                limbs[i].name = format!("{}_{k}", limbs[i].name.trim_end_matches(|c: char| c.is_ascii_digit() || c == '_'));
                k += 1;
            }
        }
        let girdles = girdle_joints
            .iter()
            .enumerate()
            .map(|(gi, &joint)| Girdle { joint, limbs: (0..limbs.len()).filter(|&l| usize::from(limbs[l].girdle) == gi).map(u8_of).collect() })
            .collect();
        let mut mirror = Vec::new();
        for (i, l) in limbs.iter().enumerate() {
            if l.side != Side::Left {
                continue;
            }
            let pair = limbs.iter().enumerate().find(|(k, r)| {
                *k != i
                    && r.side == Side::Right
                    && r.order == l.order
                    && r.support == l.support
                    && std::mem::discriminant(&r.kind) == std::mem::discriminant(&l.kind)
                    && r.joints.len() == l.joints.len()
            });
            if let Some((_, r)) = pair {
                mirror.extend(l.joints.iter().copied().zip(r.joints.iter().copied()));
                if let (Some(a), Some(b)) = (l.foot.as_ref().and_then(|f| f.toe), r.foot.as_ref().and_then(|f| f.toe)) {
                    mirror.push((a, b));
                }
            }
        }
        Self {
            skeleton: skeleton.id(),
            template: parts.template,
            root_motion: parts.root_motion,
            girdles,
            spine: parts.spine,
            neck_head: parts.neck_head,
            limbs,
            tails: parts.tails,
            extra: parts.extra,
            mirror,
            measures,
        }
    }

    pub fn limb(&self, name: &str) -> Option<&Limb> {
        self.limbs.iter().find(|l| l.name == name)
    }

    pub fn support_limbs(&self) -> impl Iterator<Item = &Limb> {
        self.limbs.iter().filter(|l| l.support)
    }

    /// The girdle joint of a limb.
    pub fn girdle_joint(&self, limb: &Limb) -> u16 {
        self.girdles.get(usize::from(limb.girdle)).map_or(0, |g| g.joint)
    }

    /// Checks the profile against `skeleton`: same topology, indices in
    /// range, limb chains parent to child.
    pub fn validate(&self, skeleton: &Skeleton) -> Result<(), RigError> {
        if self.skeleton != skeleton.id() {
            return Err(RigError::Mismatch);
        }
        let n = skeleton.len();
        let ok = |j: u16| {
            if usize::from(j) < n { Ok(()) } else { Err(RigError::BadIndex(usize::from(j))) }
        };
        for l in &self.limbs {
            for &j in &l.joints {
                ok(j)?;
            }
            ok(l.effector)?;
            if l.joints.windows(2).any(|w| !skeleton.is_ancestor(usize::from(w[0]), usize::from(w[1])) || w[0] == w[1]) {
                return Err(RigError::BadChain(l.name.clone()));
            }
            if usize::from(l.girdle) >= self.girdles.len() {
                return Err(RigError::BadIndex(usize::from(l.girdle)));
            }
        }
        for g in &self.girdles {
            ok(g.joint)?;
        }
        let chains = self.spine.iter().chain(&self.tails).chain(&self.extra).chain(self.neck_head.as_ref().map(|n| &n.neck));
        for c in chains {
            for &j in &c.joints {
                ok(j)?;
            }
        }
        for &(a, b) in &self.mirror {
            ok(a)?;
            ok(b)?;
        }
        Ok(())
    }

    /// The cooked form: magic, format version, postcard.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend(RIG_PROFILE_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let body = crate::check_header(bytes, *MAGIC, RIG_PROFILE_FORMAT)?;
        postcard::from_bytes(body).map_err(|e| FormatError::Decode(e.to_string()))
    }
}

/// The rest-pose view of a skeleton that detection walks.
struct Topo<'a> {
    s: &'a Skeleton,
    pos: Vec<Vec3>,
    /// Non-helper children.
    kids: Vec<Vec<usize>>,
    helper: Vec<bool>,
    min_y: f32,
    height: f32,
}

impl<'a> Topo<'a> {
    fn new(s: &'a Skeleton) -> Self {
        let n = s.len();
        let pos: Vec<Vec3> = s.rest_model().iter().map(|m| Vec3::from(m.translation)).collect();
        let mut all = vec![Vec::new(); n];
        for j in 0..n {
            if let Some(p) = s.parent(j) {
                all[p].push(j);
            }
        }
        // Helpers: end sites and twist joints with only helpers below them.
        let mut helper = vec![false; n];
        for j in (0..n).rev() {
            let tagged = s.joint_tags(j).any(|t| t == tag::END || t == tag::TWIST);
            helper[j] = tagged && all[j].iter().all(|&c| helper[c]);
        }
        let kids = all.iter().map(|k| k.iter().copied().filter(|&c| !helper[c]).collect()).collect();
        let core = (0..n).filter(|&j| !helper[j]);
        let (min_y, max_y) = core.map(|j| pos[j].y).fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), y| (a.min(y), b.max(y)));
        let (min_y, max_y) = if min_y.is_finite() { (min_y, max_y) } else { (0.0, 0.0) };
        Self { s, pos, kids, helper, min_y, height: (max_y - min_y).max(1e-3) }
    }

    fn tagged(&self, j: usize, t: &str) -> bool {
        self.s.joint_tags(j).any(|x| x == t)
    }

    /// `j` or an ancestor carries `t`.
    fn under_tag(&self, mut j: usize, t: &str) -> bool {
        loop {
            if self.tagged(j, t) {
                return true;
            }
            match self.s.parent(j) {
                Some(p) => j = p,
                None => return false,
            }
        }
    }

    fn is_leaf(&self, j: usize) -> bool {
        !self.helper[j] && self.kids[j].is_empty()
    }

    fn leaves_under(&self, j: usize) -> impl Iterator<Item = usize> + '_ {
        self.s.subtree(j).filter(|&k| self.is_leaf(k))
    }

    /// Down from `j` while there is exactly one non-helper child.
    fn linear_from(&self, j: usize) -> Vec<usize> {
        let mut out = vec![j];
        let mut k = j;
        while self.kids[k].len() == 1 {
            k = self.kids[k][0];
            out.push(k);
        }
        out
    }

    fn seg_len(&self, joints: &[usize]) -> f32 {
        joints.windows(2).map(|w| self.pos[w[0]].distance(self.pos[w[1]])).sum()
    }
}

/// Detects a rig profile from the rest pose and tags (rig-profile.md
/// "Auto-detection"): support feet near the ground, legs walked up to their
/// girdle, the solver from joint count and the shape of the last segments,
/// the head and spine, arms, wings and tails from the branches off the
/// spine. A skeleton the rules can't read gets `Custom` with no limbs.
#[allow(clippy::too_many_lines)]
pub fn detect_profile(skeleton: &Skeleton, hints: &RigHints) -> Result<RigProfile, RigError> {
    if let Some(p) = &hints.profile {
        p.validate(skeleton)?;
        return Ok(p.clone());
    }
    let t = Topo::new(skeleton);
    let n = skeleton.len();
    let find = |name: &str| skeleton.find(name).ok_or_else(|| RigError::UnknownJoint(name.to_owned()));
    let mut limbs: Vec<LimbDesc> = Vec::new();
    let mut claimed = vec![false; n];

    // Hinted limbs first.
    for h in &hints.limbs {
        let joints = h.joints.iter().map(|j| find(j)).collect::<Result<Vec<_>, _>>()?;
        if joints.is_empty() || joints.windows(2).any(|w| w[0] == w[1] || !skeleton.is_ancestor(w[0], w[1])) {
            return Err(RigError::BadChain(h.name.clone()));
        }
        for k in skeleton.subtree(joints[0]) {
            claimed[k] = true;
        }
        let kind = h.kind.unwrap_or(LimbKind::Leg(LegKind::Plantigrade));
        limbs.push(LimbDesc {
            name: Some(h.name.clone()),
            kind,
            girdle: u16_of(skeleton.parent(joints[0]).unwrap_or(joints[0])),
            solver: h.solver.unwrap_or_else(|| LimbSolver::for_len(joints.len())),
            pole: h.pole,
            toe: None,
            support: h.support.unwrap_or(matches!(kind, LimbKind::Leg(_))),
            side: None,
            joints: joints.into_iter().map(u16_of).collect(),
        });
    }

    // Support legs: leaves near the ground, walked up while every leaf
    // below is a support leaf, stopping below a joint that carries legs on
    // both sides.
    let frac = hints.support_height.unwrap_or(0.05);
    let support_leaf: Vec<bool> =
        (0..n).map(|j| t.is_leaf(j) && !claimed[j] && t.pos[j].y <= t.min_y + frac * t.height && !t.under_tag(j, tag::TAIL)).collect();
    let all_support = |a: usize| t.leaves_under(a).all(|l| support_leaf[l]);
    // A joint with support branches on both sides of the midline (the
    // skeleton root's x) carries several legs: it is their girdle, even when
    // nothing else hangs off it (a spider's body, a legs-only pelvis). The
    // toes of one foot all lie on that foot's side, so a foot never stops
    // the walk.
    let mid_x = (0..n).find(|&j| skeleton.parent(j).is_none()).map_or(0.0, |r| t.pos[r].x);
    let margin = 0.02 * t.height;
    let bilateral = |p: usize| {
        let (mut left, mut right) = (false, false);
        for &c in &t.kids[p] {
            let (lo, hi) = t
                .leaves_under(c)
                .filter(|&l| support_leaf[l])
                .map(|l| t.pos[l].x - mid_x)
                .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), x| (lo.min(x), hi.max(x)));
            left |= lo.is_finite() && lo > margin;
            right |= hi.is_finite() && hi < -margin;
        }
        left && right
    };
    let mut leg_roots: Vec<usize> = Vec::new();
    for leaf in (0..n).filter(|&j| support_leaf[j]) {
        let mut a = leaf;
        while let Some(p) = skeleton.parent(a) {
            if skeleton.parent(p).is_none() || claimed[p] || !all_support(p) || bilateral(p) {
                break;
            }
            a = p;
        }
        if a != leaf && !leg_roots.contains(&a) {
            leg_roots.push(a);
        }
    }
    for &a in &leg_roots {
        let mut joints = t.linear_from(a);
        let last = *joints.last().expect("non-empty");
        // Toes: trailing joints tagged `toe`, else one near-horizontal last
        // segment. The effector is the joint above them.
        let mut toe = None;
        if t.kids[last].is_empty() {
            while joints.len() >= 3 && t.tagged(*joints.last().expect("non-empty"), tag::TOE) {
                toe = joints.pop();
            }
            if toe.is_none() && joints.len() >= 3 {
                let d = t.pos[last] - t.pos[joints[joints.len() - 2]];
                if d.y.abs() < 0.6 * horizontal_len(d) {
                    toe = joints.pop();
                }
            }
        }
        if joints.len() < 2 {
            continue;
        }
        let k = joints.len();
        let leg_len = t.seg_len(&joints).max(1e-3);
        let solver = match k {
            0 | 1 => LimbSolver::None,
            2 => LimbSolver::Ccd,
            3 => LimbSolver::TwoBone,
            4 | 5 => {
                let (h, e) = (t.pos[joints[k - 2]], t.pos[joints[k - 1]]);
                let seg = e - h;
                let steep = seg.y.abs() > horizontal_len(seg);
                let hock_up = h.y - t.min_y > 0.1 * leg_len;
                let segs: Vec<f32> = joints.windows(2).map(|w| t.pos[w[0]].distance(t.pos[w[1]])).collect();
                let longest = segs.iter().copied().fold(0.0, f32::max);
                let passive_root = k == 4 && segs[0] < 0.6 * longest;
                if steep && hock_up && !passive_root { LimbSolver::TwoBoneAnkle } else { LimbSolver::TwoBone }
            }
            _ => LimbSolver::Fabrik,
        };
        let effector = *joints.last().expect("non-empty");
        let hoof = t.tagged(effector, tag::HOOF) || toe.is_some_and(|x| t.tagged(x, tag::HOOF));
        let kind = LimbKind::Leg(match solver {
            _ if hoof => LegKind::Unguligrade,
            LimbSolver::TwoBoneAnkle => LegKind::Digitigrade,
            _ => LegKind::Plantigrade,
        });
        limbs.push(LimbDesc {
            name: None,
            kind,
            girdle: u16_of(skeleton.parent(a).unwrap_or(a)),
            solver,
            pole: None,
            toe: toe.map(u16_of),
            support: true,
            side: None,
            joints: joints.into_iter().map(u16_of).collect(),
        });
    }
    let leg_root_set: Vec<bool> = {
        let mut v = vec![false; n];
        for l in &limbs {
            if let Some(&r) = l.joints.first() {
                v[usize::from(r)] = true;
            }
        }
        v
    };

    // Body root: the root-most support girdle, else the lowest common
    // ancestor of the free leaves.
    let support_girdles: Vec<usize> = limbs.iter().filter(|l| l.support).map(|l| usize::from(l.girdle)).collect();
    let body_root = support_girdles.iter().copied().min_by_key(|&g| (depth(skeleton, g), g)).unwrap_or_else(|| {
        let leaves: Vec<usize> = (0..n).filter(|&j| t.is_leaf(j) && !claimed[j]).collect();
        let mut lca = leaves.first().copied().unwrap_or(0);
        for &l in &leaves[1..] {
            while !skeleton.is_ancestor(lca, l) {
                match skeleton.parent(lca) {
                    Some(p) => lca = p,
                    None => break,
                }
            }
        }
        lca
    });
    let in_leg = |j: usize| (0..n).any(|r| leg_root_set[r] && skeleton.is_ancestor(r, j));

    // Head: the tagged head, else the free leaf most up and forward of the
    // body root (moved up to the skull when ears or a jaw branch off it).
    let root_p = t.pos[body_root];
    let tagged_heads: Vec<usize> = skeleton.joints_with_tag(tag::HEAD).filter(|&h| skeleton.is_ancestor(body_root, h) && h != body_root).collect();
    let head = tagged_heads.first().copied().or_else(|| {
        let best = skeleton
            .subtree(body_root)
            .filter(|&j| t.is_leaf(j) && !claimed[j] && !in_leg(j) && !t.under_tag(j, tag::TAIL) && j != body_root)
            .max_by(|&a, &b| {
                let s = |j: usize| (t.pos[j].y - root_p.y) + (t.pos[j].z - root_p.z);
                s(a).total_cmp(&s(b))
            })?;
        let mut head = best;
        let mut j = best;
        while let Some(p) = skeleton.parent(j) {
            if p == body_root || t.pos[p].distance(t.pos[best]) > 0.1 * t.height {
                break;
            }
            if t.kids[p].len() >= 2 {
                head = p;
            }
            j = p;
        }
        Some(head)
    });

    let main_path: Vec<usize> = head.map_or_else(|| vec![body_root], |h| path(skeleton, body_root, h));
    let on_path = |j: usize| main_path.contains(&j);
    let body_extent =
        (t.pos.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max) - t.pos.iter().map(|p| p.x).fold(f32::INFINITY, f32::min)).max(0.0);
    let support_x: Vec<f32> = limbs.iter().filter(|l| l.support).filter_map(|l| l.joints.last()).map(|&e| t.pos[usize::from(e)].x).collect();
    let body_width = if support_x.len() >= 2 {
        support_x.iter().copied().fold(f32::NEG_INFINITY, f32::max) - support_x.iter().copied().fold(f32::INFINITY, f32::min)
    } else {
        0.25 * body_extent
    };

    // Branches off the spine: arms, wings, tails, extra chains.
    let mut tails: Vec<Chain> = Vec::new();
    let mut extra: Vec<Chain> = Vec::new();
    let path_end = main_path.len().saturating_sub(usize::from(head.is_some()));
    for &p in &main_path[..path_end] {
        for &c in &t.kids[p] {
            if on_path(c) || claimed[c] || leg_root_set[c] || skeleton.subtree(c).any(|k| leg_root_set[k]) {
                continue;
            }
            let chain = t.linear_from(c);
            let end = *chain.last().expect("non-empty");
            let names = |js: &[usize]| Chain { name: skeleton.name(js[0]).to_owned(), joints: js.iter().copied().map(u16_of).collect() };
            let tail_tagged = skeleton.subtree(c).any(|k| t.tagged(k, tag::TAIL));
            let wing_tagged = skeleton.subtree(c).any(|k| t.tagged(k, tag::WING));
            let hand_tagged = chain.iter().any(|&k| t.tagged(k, tag::HAND));
            let behind = p == body_root && t.pos[end].z < root_p.z - 0.05 * t.height;
            if t.kids[end].is_empty() && !hand_tagged {
                // A linear chain to a leaf.
                let span = t.pos[end] - t.pos[c];
                let long_flat = t.seg_len(&chain) > body_width.max(0.2 * t.height) && span.y.abs() < horizontal_len(span);
                let foot_tagged = tag::CONTACTS.iter().any(|x| t.tagged(end, x));
                if tail_tagged || (behind && !wing_tagged && !foot_tagged) {
                    tails.push(Chain { name: if tails.is_empty() { "tail".into() } else { format!("tail_{}", tails.len() + 1) }, ..names(&chain) });
                } else if foot_tagged && chain.len() >= 2 {
                    // A leg off the ground in the rest pose (a flyer's tucked legs).
                    limbs.push(LimbDesc {
                        name: None,
                        kind: LimbKind::Leg(LegKind::Plantigrade),
                        girdle: u16_of(p),
                        solver: if chain.len() == 3 { LimbSolver::TwoBone } else { LimbSolver::for_len(chain.len()) },
                        pole: None,
                        toe: None,
                        support: false,
                        side: None,
                        joints: chain.into_iter().map(u16_of).collect(),
                    });
                } else if wing_tagged || (long_flat && chain.len() >= 3) {
                    let solver = if (3..=5).contains(&chain.len()) { LimbSolver::TwoBone } else { LimbSolver::for_len(chain.len()) };
                    limbs.push(LimbDesc {
                        name: None,
                        kind: LimbKind::Wing,
                        girdle: u16_of(p),
                        solver,
                        pole: None,
                        toe: None,
                        support: false,
                        side: None,
                        joints: chain.into_iter().map(u16_of).collect(),
                    });
                } else {
                    extra.push(names(&chain));
                }
            } else {
                // Ends in a branching joint (a hand with fingers, a wing
                // hand) or at a tagged hand.
                let mut chain = chain;
                if let Some(h) = chain.iter().position(|&k| t.tagged(k, tag::HAND)) {
                    chain.truncate(h + 1);
                }
                let k = chain.len();
                let solver = match k {
                    0 | 1 => LimbSolver::None,
                    2 => LimbSolver::Ccd,
                    _ => LimbSolver::TwoBone,
                };
                limbs.push(LimbDesc {
                    name: None,
                    kind: if wing_tagged { LimbKind::Wing } else { LimbKind::Arm },
                    girdle: u16_of(p),
                    solver,
                    pole: None,
                    toe: None,
                    support: false,
                    side: None,
                    joints: chain.into_iter().map(u16_of).collect(),
                });
            }
        }
    }
    for (name, joints) in &hints.extra {
        let joints = joints.iter().map(|j| find(j).map(u16_of)).collect::<Result<Vec<_>, _>>()?;
        extra.push(Chain { name: name.clone(), joints });
    }

    // Spine and neck: up to the front-most girdle on the main path, then
    // to the head.
    let girdle_on_path = |j: usize| limbs.iter().any(|l| usize::from(l.girdle) == j);
    let front = main_path[..path_end].iter().rposition(|&j| girdle_on_path(j)).unwrap_or(0);
    let (spine_end, neck_start) = if front > 0 {
        (front + 1, front + 1)
    } else {
        let first_neck = main_path[..path_end].iter().position(|&j| t.tagged(j, tag::NECK)).unwrap_or(path_end);
        (first_neck.max(1).min(path_end.max(1)), first_neck)
    };
    let to_chain = |name: &str, js: &[usize]| Chain { name: name.to_owned(), joints: js.iter().copied().map(u16_of).collect() };
    let spine = (spine_end >= 2).then(|| to_chain("spine", &main_path[..spine_end]));
    let neck_head = head.map(|h| {
        let neck = if neck_start < path_end { &main_path[neck_start..path_end] } else { &[][..] };
        let rest = affine_rotation(&skeleton.rest_model()[h]);
        NeckHead {
            neck: to_chain("neck", neck),
            head: u16_of(h),
            forward: (rest.inverse() * Vec3::Z).normalize_or(Vec3::Z),
            eyes: skeleton.subtree(h).filter(|&e| t.tagged(e, tag::EYE)).map(u16_of).collect(),
        }
    });

    let support = limbs.iter().filter(|l| l.support).count();
    let wings = limbs.iter().filter(|l| l.kind == LimbKind::Wing).count();
    let template = hints.template.unwrap_or(match (support, wings) {
        (0..=2, w) if w > 0 => RigTemplate::Flyer,
        (2, _) => RigTemplate::Biped,
        (4, _) => RigTemplate::Quadruped,
        (s, _) if s >= 6 => RigTemplate::Multiped,
        (0, _) if limbs.is_empty() && main_path.len() >= 6 => RigTemplate::Serpent,
        (0, _) if limbs.is_empty() => RigTemplate::Custom,
        (0, _) => RigTemplate::Floater,
        _ => RigTemplate::Multiped,
    });
    Ok(RigProfile::build(
        skeleton,
        ProfileParts { template, root_motion: skeleton.root_motion_joint().map(u16_of), limbs, spine, neck_head, tails, extra },
    ))
}
