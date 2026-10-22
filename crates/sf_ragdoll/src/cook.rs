//! `ragdoll.cook`: a [`RagdollDef`] from a skeleton's rest pose and rig
//! profile (physics.md "`sf_ragdoll`"). The industry's auto-setup (UE's
//! Physics Asset tool, Unity's Ragdoll Wizard) does the same from bone
//! lengths: one capsule per major bone, cone/twist limits at ball joints,
//! hinges at knees and elbows, adjacent and initially overlapping bodies
//! excluded from collision. Our input is the rig profile, so it works for
//! any morphology (bipeds, quadrupeds, flyers, tails).
//!
//! Without skin extents (the skinned meshes are separate per outfit) the
//! radii come from bone lengths and girdle widths; adapters and mods patch
//! them with [`BodyOverride`]s.

use std::f32::consts::PI;

use glam::{Mat3, Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::{LimbKind, RigProfile, Side, Skeleton};

use crate::{Capsule, RagBody, RagJoint, RagJointKind, RagdollDef};

/// What a skeleton does on death (adapter rules).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RagdollPolicy {
    /// The death clip hands over to a limp ragdoll.
    #[default]
    Ragdoll,
    /// The death clip only (ghosts, swarms, rigs whose ragdoll would look wrong).
    DeathClip,
    /// No death handling.
    None,
}

/// A per-joint patch by joint name (adapter or mod override file).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BodyOverride {
    pub joint: String,
    /// Capsule radius at the skeleton's scale, m.
    pub radius: Option<f32>,
    /// No body for this joint (it moves with its parent's body).
    pub skip: bool,
    /// Replaces the joint limits.
    pub kind: Option<RagJointKind>,
}

/// Cook settings (adapter `RagdollRules`; part of the bake key).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookParams {
    pub policy: RagdollPolicy,
    /// Total mass at the skeleton's scale, kg; `None` keeps volume × density.
    pub total_mass: Option<f32>,
    pub density: f32,
    /// No body lighter than its parent body ÷ this (solver mass ratios: a
    /// 0.4 kg hand under a 75 kg pull stretches its wrist).
    pub max_mass_ratio: f32,
    pub max_spine_bodies: usize,
    pub max_chain_bodies: usize,
    /// Capsule radius ÷ segment length: arms and legs, hands and feet, wings and fins, tails.
    pub limb_radius: f32,
    pub effector_radius: f32,
    pub wing_radius: f32,
    pub chain_radius: f32,
    /// Extra chains shorter than this fraction of the height get no bodies (jaws, ears).
    pub min_extra_chain: f32,
    pub overrides: Vec<BodyOverride>,
    /// Anatomical swing ranges of limb roots and the neck (AN-20).
    #[serde(default)]
    pub ranges: JointRanges,
}

impl Default for CookParams {
    fn default() -> Self {
        Self {
            policy: RagdollPolicy::Ragdoll,
            total_mass: None,
            density: 1000.0,
            max_mass_ratio: 4.0,
            max_spine_bodies: 3,
            max_chain_bodies: 4,
            limb_radius: 0.2,
            effector_radius: 0.3,
            wing_radius: 0.08,
            chain_radius: 0.2,
            min_extra_chain: 0.15,
            overrides: Vec::new(),
            ranges: JointRanges::default(),
        }
    }
}

impl CookParams {
    /// Stable bytes for the bake key.
    pub fn key_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("in-memory serialization")
    }

    fn patch(&self, name: &str) -> Option<&BodyOverride> {
        self.overrides.iter().find(|o| o.joint.eq_ignore_ascii_case(name))
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum RagdollError {
    #[error("the rig profile was made for another skeleton")]
    Mismatch,
    #[error("no girdle, spine, tail or limb to root a ragdoll on")]
    NoCore,
}

/// The region name of the spine bodies (pelvis to chest).
pub const CORE_REGION: &str = "core";
/// The region name of the neck-and-head body.
pub const HEAD_REGION: &str = "head";

fn deg(d: f32) -> f32 {
    d * PI / 180.0
}

/// Per spine segment, degrees: a spine of three or four segments adds up, so each allows far
/// less than a limb joint (shipped physics assets and Unity's ragdoll wizard use about 10-20 of
/// twist and 10-15 of swing per segment). The old 30/30 let a 4-segment spine fold 120 degrees.
/// Tighter limits also stretch joints more in a hard tumble (see `settles_on_stairs`).
const SPINE_SWING_DEG: f32 = 15.0;
const SPINE_TWIST_DEG: f32 = 15.0;

fn ball(swing: f32, twist: f32) -> RagJointKind {
    RagJointKind::Ball { swing: [[-deg(swing), deg(swing)]; 2], twist: [-deg(twist), deg(twist)] }
}

/// An anatomical ball joint's ranges, degrees, from the rest pose: `forward`/`back` swing
/// the bone toward the character's front or back (flexion, extension), `out`/`inward`
/// away from or toward the midline (abduction, adduction), `twist` either way.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Swing {
    pub forward: f32,
    pub back: f32,
    pub out: f32,
    pub inward: f32,
    pub twist: f32,
}

/// Knee and elbow hyperextension past straight, degrees. Normal adult range of motion
/// (AAOS, Greene & Heckman, *Clinical Measurement of Joint Motion*, 1994): elbow 0-10°
/// hyperextension, knee 0-10° (genu recurvatum past that); a limp body sits at the loose
/// end. 5° caught a 300 N·s knockdown's elbow 7.6° past it (AN-20).
pub const HYPEREXTENSION: f32 = 10.0;
/// Anatomical ranges per joint kind (data-overridable through [`CookParams::ranges`];
/// a [`BodyOverride::kind`] still replaces one joint). Defaults sit inside human active
/// ranges (hip flexion ~120°, extension ~20°, abduction ~45°; shoulder flexion ~180°,
/// extension ~50°): a limp body is looser than a living one only where it is loose.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct JointRanges {
    /// Arm and wing roots.
    pub shoulder: Swing,
    /// Leg roots.
    pub hip: Swing,
    /// The neck base (the head body).
    pub neck: Swing,
}

impl Default for JointRanges {
    fn default() -> Self {
        Self {
            shoulder: Swing { forward: 90.0, back: 30.0, out: 80.0, inward: 30.0, twist: 45.0 },
            hip: Swing { forward: 80.0, back: 15.0, out: 35.0, inward: 10.0, twist: 20.0 },
            neck: Swing { forward: 45.0, back: 30.0, out: 30.0, inward: 30.0, twist: 30.0 },
        }
    }
}

/// An anatomical ball joint for `bone` (model space; the character faces +Z): frame X
/// along the bone, Z = bone × forward (a positive swing about Z turns the bone forward:
/// flexion), Y across both (a positive swing about Y turns the bone toward -Z).
/// `outward` (from the midline) signs abduction; without one the side ranges are symmetric.
fn anatomical(bone: Vec3, outward: Option<Vec3>, r: &Swing) -> (RagJointKind, Quat) {
    let b = bone.normalize_or(Vec3::NEG_Y);
    let forward = if b.dot(Vec3::Z).abs() > 0.95 { Vec3::Y } else { Vec3::Z };
    let z = b.cross(forward).normalize();
    let frame = Quat::from_mat3(&Mat3::from_cols(b, z.cross(b), z)).normalize();
    let flex = [-deg(r.back), deg(r.forward)];
    let o = outward.map(|o| o - b * b.dot(o)).filter(|o| o.length() > 1e-3);
    let side = match o {
        Some(o) if (-z).dot(o) >= 0.0 => [-deg(r.inward), deg(r.out)],
        Some(_) => [-deg(r.out), deg(r.inward)],
        None => [-deg(r.out.max(r.inward)), deg(r.out.max(r.inward))],
    };
    (RagJointKind::Ball { swing: [side, flex], twist: [-deg(r.twist), deg(r.twist)] }, frame)
}

/// A body before masses and frames.
struct Spec {
    joint: u16,
    start: Vec3,
    /// Capsule axis (model space; the bone direction unless lateral).
    axis: Vec3,
    center: Vec3,
    radius: f32,
    half: f32,
    region: u8,
    kind: RagJointKind,
    /// Joint frame orientation in model space (X = twist or hinge axis).
    frame: Quat,
}

fn frame_x(x: Vec3) -> Quat {
    Quat::from_rotation_arc(Vec3::X, x.normalize_or(Vec3::Y))
}

/// A hinge frame: X = `axis`, Y = the bone.
fn frame_hinge(axis: Vec3, bone: Vec3) -> Quat {
    let y = (bone - axis * axis.dot(bone)).normalize_or(axis.any_orthonormal_vector());
    Quat::from_mat3(&Mat3::from_cols(axis, y, axis.cross(y))).normalize()
}

struct Builder<'a> {
    skeleton: &'a Skeleton,
    params: &'a CookParams,
    pos: Vec<Vec3>,
    reach: Vec<f32>,
    height: f32,
    specs: Vec<Spec>,
    regions: Vec<String>,
}

impl Builder<'_> {
    fn region(&mut self, name: &str) -> u8 {
        let i = self.regions.iter().position(|r| r == name).unwrap_or_else(|| {
            self.regions.push(name.to_owned());
            self.regions.len() - 1
        });
        u8::try_from(i).unwrap_or(u8::MAX)
    }

    fn taken(&self, joint: u16) -> bool {
        self.specs.iter().any(|s| s.joint == joint)
    }

    fn skipped(&self, joint: u16) -> bool {
        self.params.patch(self.skeleton.name(usize::from(joint))).is_some_and(|o| o.skip)
    }

    /// The direction a chain continues past `joint`: toward its subtree's
    /// farthest joint, else along its parent bone.
    fn onward(&self, joint: u16, fallback: Vec3) -> Vec3 {
        let j = usize::from(joint);
        let p = self.pos[j];
        let far = self.skeleton.subtree(j).map(|k| self.pos[k]).max_by(|a, b| a.distance_squared(p).total_cmp(&b.distance_squared(p)));
        match far {
            Some(f) if f.distance(p) > 1e-3 => (f - p).normalize(),
            _ => fallback,
        }
    }

    fn parent_dir(&self, joint: u16) -> Vec3 {
        let j = usize::from(joint);
        self.skeleton.parent(j).map_or(Vec3::Y, |p| (self.pos[j] - self.pos[p]).normalize_or(Vec3::Y))
    }

    /// A capsule along the bone `start → end`.
    #[allow(clippy::too_many_arguments)]
    fn push_bone(&mut self, joint: u16, start: Vec3, end: Vec3, ratio: f32, region: u8, kind: RagJointKind, frame: Option<Quat>) {
        let len = start.distance(end);
        if len < 1e-3 || self.taken(joint) || self.skipped(joint) {
            return;
        }
        let axis = (end - start) / len;
        let name = self.skeleton.name(usize::from(joint));
        let patch = self.params.patch(name);
        let radius = patch.and_then(|o| o.radius).unwrap_or((ratio * len).max(0.004 * self.height).min(0.5 * len)).max(0.002);
        let kind = patch.and_then(|o| o.kind).unwrap_or(kind);
        let half = (0.5 * len - radius).max(0.0);
        self.specs.push(Spec {
            joint,
            start,
            axis,
            center: 0.5 * (start + end),
            radius,
            half,
            region,
            kind,
            frame: frame.unwrap_or_else(|| frame_x(axis)),
        });
    }

    /// Spine bodies: the root girdle and at most `max_spine_bodies`
    /// segments up to the neck base, as wide as their girdle.
    fn core(&mut self, profile: &RigProfile) -> Result<(), RagdollError> {
        let neck: Vec<u16> = profile.neck_head.as_ref().map(|nh| nh.neck.joints.iter().copied().chain([nh.head]).collect()).unwrap_or_default();
        let mut spine: Vec<u16> = profile.spine.as_ref().map(|c| c.joints.clone()).unwrap_or_default();
        spine.retain(|j| !neck.contains(j));
        if let Some(g) = profile.girdles.first() {
            spine.push(g.joint);
        }
        spine.sort_unstable();
        spine.dedup();
        // Keep one ancestor line (the root girdle's descendants on the spine).
        if let Some(&first) = spine.first() {
            spine.retain(|&j| self.skeleton.is_ancestor(usize::from(first), usize::from(j)) || j == first);
        }
        if spine.is_empty() {
            let fallback = profile
                .tails
                .first()
                .and_then(|t| t.joints.first().copied())
                .or_else(|| profile.limbs.first().map(|l| profile.girdle_joint(l)))
                .ok_or(RagdollError::NoCore)?;
            spine.push(fallback);
        }
        let k = self.params.max_spine_bodies.clamp(1, spine.len());
        let mut picks: Vec<u16> = (0..k).map(|i| spine[i * spine.len() / k]).collect();
        // The shoulder girdle gets its own body when the spine has room.
        if k >= 2
            && let Some(g) = profile.girdles.get(1).map(|g| g.joint)
            && spine.contains(&g)
            && g > picks[k - 2]
        {
            picks[k - 1] = g;
        }
        let neck_base = profile.neck_head.as_ref().map(|nh| nh.neck.joints.first().copied().unwrap_or(nh.head));
        // Girdle widths: lateral distance between a left and a right limb root.
        let width_at = |b: &Self, joint: u16| -> Option<f32> {
            let g = profile.girdles.iter().find(|g| g.joint == joint)?;
            let roots = |side| {
                g.limbs.iter().filter_map(|&l| profile.limbs.get(usize::from(l))).find(|l| l.side == side).map(|l| b.pos[usize::from(l.joints[0])])
            };
            let (l, r) = (roots(Side::Left)?, roots(Side::Right)?);
            Some((l.x - r.x).abs())
        };
        let mut widths: Vec<Option<f32>> = picks
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                // Girdles between this pick and the next belong to this body.
                let next = picks.get(i + 1).copied().unwrap_or(u16::MAX);
                profile.girdles.iter().filter(|g| g.joint >= p && g.joint < next).filter_map(|g| width_at(self, g.joint)).reduce(f32::max)
            })
            .collect();
        let known: Vec<f32> = widths.iter().flatten().copied().collect();
        #[allow(clippy::cast_precision_loss)]
        let fill = if known.is_empty() { 0.15 * self.height } else { known.iter().sum::<f32>() / known.len() as f32 };
        for w in &mut widths {
            w.get_or_insert(fill);
        }
        let region = self.region(CORE_REGION);
        for (i, &joint) in picks.iter().enumerate() {
            let start = self.pos[usize::from(joint)];
            let end = match (picks.get(i + 1), neck_base) {
                (Some(&n), _) => self.pos[usize::from(n)],
                (None, Some(n)) if self.pos[usize::from(n)].distance(start) > 1e-3 => self.pos[usize::from(n)],
                _ => start + self.onward(joint, self.parent_dir(joint)) * (0.1 * self.height),
            };
            let len = start.distance(end).max(1e-3);
            let bone = (end - start) / len;
            let width = widths[i].unwrap_or(fill);
            let kind = ball(SPINE_SWING_DEG, SPINE_TWIST_DEG);
            if len >= width {
                let ratio = (0.4 * width / len).min(0.5);
                self.push_bone(joint, start, end, ratio, region, kind, None);
            } else {
                // Wider than long (a pelvis, a chest): a capsule across the girdle.
                let patch = self.params.patch(self.skeleton.name(usize::from(joint)));
                let radius = patch.and_then(|o| o.radius).unwrap_or((0.5 * len).max(0.4 * width).min(0.5 * width)).max(0.005);
                if self.taken(joint) || self.skipped(joint) {
                    continue;
                }
                self.specs.push(Spec {
                    joint,
                    start,
                    axis: Vec3::X,
                    center: 0.5 * (start + end),
                    radius,
                    half: (0.5 * width - radius).max(0.0),
                    region,
                    kind: patch.and_then(|o| o.kind).unwrap_or(kind),
                    frame: frame_x(bone),
                });
            }
        }
        if self.specs.is_empty() {
            return Err(RagdollError::NoCore);
        }
        Ok(())
    }

    /// Neck and head: one body from the neck base past the head.
    fn head(&mut self, profile: &RigProfile) {
        let Some(nh) = &profile.neck_head else { return };
        let base = nh.neck.joints.first().copied().unwrap_or(nh.head);
        let head = usize::from(nh.head);
        let dir = if base == nh.head { self.parent_dir(nh.head) } else { (self.pos[head] - self.pos[usize::from(base)]).normalize_or(Vec3::Y) };
        let head_len = self.reach[head].clamp(0.08 * self.height, 0.25 * self.height);
        let start = self.pos[usize::from(base)];
        let end = self.pos[head] + dir * head_len;
        let len = start.distance(end).max(1e-3);
        let region = self.region(HEAD_REGION);
        let (kind, frame) = anatomical(end - start, None, &self.params.ranges.neck);
        self.push_bone(base, start, end, (0.4 * head_len / len).min(0.5), region, kind, Some(frame));
    }

    fn limbs(&mut self, profile: &RigProfile) {
        for limb in &profile.limbs {
            let region = self.region(&limb.name);
            let n = limb.joints.len();
            let mut prev_len = 0.0f32;
            let mut prev_dir = self.parent_dir(limb.joints[0]);
            let mut hinges = 0;
            for (i, &joint) in limb.joints.iter().enumerate() {
                let start = self.pos[usize::from(joint)];
                let effector = i + 1 == n;
                let end = if effector {
                    let dir = match limb.foot.as_ref().and_then(|f| f.toe) {
                        Some(t) if self.pos[usize::from(t)].distance(start) > 1e-3 => (self.pos[usize::from(t)] - start).normalize(),
                        _ => self.onward(joint, prev_dir),
                    };
                    let len = if prev_len > 0.0 { self.reach[usize::from(joint)].clamp(0.3 * prev_len, prev_len) } else { 0.1 * self.height };
                    start + dir * len
                } else {
                    self.pos[usize::from(limb.joints[i + 1])]
                };
                let len = start.distance(end);
                if len < 1e-3 {
                    continue;
                }
                let bone = (end - start) / len;
                let thin = matches!(limb.kind, LimbKind::Wing | LimbKind::Fin);
                let ratio = if thin {
                    self.params.wing_radius
                } else if effector {
                    self.params.effector_radius
                } else {
                    self.params.limb_radius
                };
                let hinge_kind = matches!(limb.kind, LimbKind::Leg(_) | LimbKind::Arm | LimbKind::Wing);
                let (kind, frame) = if i == 0 {
                    let outward = start - self.pos[usize::from(profile.girdle_joint(limb))];
                    let outward = (limb.side != Side::Center).then_some(outward);
                    match limb.kind {
                        LimbKind::Arm | LimbKind::Wing => {
                            let (k, f) = anatomical(bone, outward, &self.params.ranges.shoulder);
                            (k, Some(f))
                        }
                        LimbKind::Leg(_) => {
                            let (k, f) = anatomical(bone, outward, &self.params.ranges.hip);
                            (k, Some(f))
                        }
                        LimbKind::Fin | LimbKind::Tentacle => (ball(45.0, 30.0), None),
                    }
                } else if effector || !hinge_kind {
                    (ball(35.0, 15.0), None)
                } else {
                    // Knee, elbow, hock: bend further the way the rest pose bends,
                    // else knees back, elbows forward, alternating down a leg.
                    let u = prev_dir;
                    let rest_bend = u.angle_between(bone);
                    let (axis, bent) = if rest_bend > deg(5.0) {
                        (u.cross(bone).normalize(), rest_bend)
                    } else {
                        let default = match limb.kind {
                            LimbKind::Leg(_) => {
                                if hinges % 2 == 0 {
                                    -Vec3::Z
                                } else {
                                    Vec3::Z
                                }
                            }
                            _ => Vec3::Z,
                        };
                        let b = default - bone * bone.dot(default);
                        let axis = if b.length() > 1e-3 { bone.cross(b).normalize() } else { Vec3::X };
                        (axis, 0.0)
                    };
                    hinges += 1;
                    let lo = -(bent + deg(HYPEREXTENSION));
                    let hi = (deg(150.0) - bent).max(deg(5.0));
                    (RagJointKind::Hinge { range: [lo, hi] }, Some(frame_hinge(axis, bone)))
                };
                self.push_bone(joint, start, end, ratio, region, kind, frame);
                prev_len = len;
                prev_dir = bone;
            }
        }
    }

    /// Tails and long extra chains: at most `max_chain_bodies` each.
    fn chains(&mut self, profile: &RigProfile) {
        let long = |b: &Self, joints: &[u16]| {
            joints.windows(2).map(|w| b.pos[usize::from(w[0])].distance(b.pos[usize::from(w[1])])).sum::<f32>() >= b.params.min_extra_chain * b.height
        };
        let chains: Vec<_> = profile.tails.iter().chain(profile.extra.iter().filter(|c| long(self, &c.joints))).cloned().collect();
        for chain in chains {
            let joints: Vec<u16> = chain.joints.iter().copied().filter(|&j| !self.taken(j)).collect();
            if joints.is_empty() {
                continue;
            }
            let region = self.region(&chain.name);
            let k = self.params.max_chain_bodies.clamp(1, joints.len());
            let picks: Vec<u16> = (0..k).map(|i| joints[i * joints.len() / k]).collect();
            let last = *joints.last().expect("non-empty");
            let mut prev_dir = self.parent_dir(picks[0]);
            for (i, &joint) in picks.iter().enumerate() {
                let start = self.pos[usize::from(joint)];
                let end = match picks.get(i + 1) {
                    Some(&n) => self.pos[usize::from(n)],
                    None if last != joint => self.pos[usize::from(last)] + self.onward(last, prev_dir) * self.reach[usize::from(last)],
                    None => start + self.onward(joint, prev_dir) * self.reach[usize::from(joint)].max(0.05 * self.height),
                };
                self.push_bone(joint, start, end, self.params.chain_radius, region, ball(35.0, 20.0), None);
                prev_dir = (end - start).normalize_or(prev_dir);
            }
        }
    }
}

/// Closest distance between segments `p1q1` and `p2q2` (Ericson, RTCD 5.1.9).
#[allow(clippy::many_single_char_names)]
fn segment_distance(p1: Vec3, q1: Vec3, p2: Vec3, q2: Vec3) -> f32 {
    let (d1, d2, r) = (q1 - p1, q2 - p2, p1 - p2);
    let (a, e, f) = (d1.length_squared(), d2.length_squared(), d2.dot(r));
    let (s, t) = if a <= 1e-9 && e <= 1e-9 {
        (0.0, 0.0)
    } else if a <= 1e-9 {
        (0.0, (f / e).clamp(0.0, 1.0))
    } else {
        let c = d1.dot(r);
        if e <= 1e-9 {
            ((-c / a).clamp(0.0, 1.0), 0.0)
        } else {
            let b = d1.dot(d2);
            let denom = a * e - b * b;
            let mut s = if denom > 1e-9 { ((b * f - c * e) / denom).clamp(0.0, 1.0) } else { 0.0 };
            let mut t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            }
            (s, t)
        }
    };
    (p1 + d1 * s).distance(p2 + d2 * t)
}

fn capsule_volume(c: Capsule) -> f32 {
    PI * c.radius * c.radius * (2.0 * c.half + 4.0 / 3.0 * c.radius)
}

/// Cooks a skeleton's ragdoll from its rest pose and rig profile.
pub fn cook(skeleton: &Skeleton, profile: &RigProfile, params: &CookParams) -> Result<RagdollDef, RagdollError> {
    if profile.skeleton != skeleton.id() {
        return Err(RagdollError::Mismatch);
    }
    let rest = skeleton.rest_model();
    let pos: Vec<Vec3> = rest.iter().map(|m| Vec3::from(m.translation)).collect();
    let rot: Vec<Quat> = rest.iter().map(|m| m.to_scale_rotation_translation().1.normalize()).collect();
    let height = if profile.measures.height > 0.01 {
        profile.measures.height
    } else {
        let (lo, hi) = pos.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| (lo.min(p.y), hi.max(p.y)));
        (hi - lo).max(0.1)
    };
    let mut b = Builder { skeleton, params, pos, reach: skeleton.subtree_reach(), height, specs: Vec::new(), regions: Vec::new() };
    b.core(profile)?;
    b.head(profile);
    b.limbs(profile);
    b.chains(profile);
    let Builder { mut specs, regions, .. } = b;
    // Root first, then skeleton order (parents before children).
    let root_joint = specs[0].joint;
    specs[1..].sort_by_key(|s| s.joint);
    let n = skeleton.len();
    let mut owner: Vec<Option<u16>> = vec![None; n];
    for j in 0..n {
        owner[j] = match specs.iter().position(|s| usize::from(s.joint) == j) {
            Some(i) => u16::try_from(i).ok(),
            None => skeleton.parent(j).and_then(|p| owner[p]),
        };
    }
    let body_rot = |s: &Spec| Quat::from_rotation_arc(Vec3::Y, s.axis);
    let mut bodies: Vec<RagBody> = Vec::with_capacity(specs.len());
    let mut joints = Vec::new();
    for (i, s) in specs.iter().enumerate() {
        let j = usize::from(s.joint);
        let br = body_rot(s);
        let shape = Capsule { radius: s.radius, half: s.half };
        let parent = if i == 0 {
            None
        } else {
            Some(skeleton.parent(j).and_then(|p| owner[p]).filter(|&p| usize::from(p) < i).unwrap_or(0))
        };
        bodies.push(RagBody {
            joint: s.joint,
            shape,
            local: (rot[j].inverse() * (s.center - rest_pos(&rest, j)), (rot[j].inverse() * br).normalize()),
            mass: capsule_volume(shape) * params.density,
            region: s.region,
            parent,
        });
        if let Some(p) = parent {
            let ps = &specs[usize::from(p)];
            let pr = body_rot(ps);
            let pivot = s.start;
            joints.push(RagJoint {
                parent: p,
                child: u16::try_from(i).expect("fewer than 65536 bodies"),
                frame_parent: (pr.inverse() * (pivot - ps.center), (pr.inverse() * s.frame).normalize()),
                frame_child: (br.inverse() * (pivot - s.center), (br.inverse() * s.frame).normalize()),
                kind: s.kind,
            });
        }
    }
    debug_assert_eq!(bodies[0].joint, root_joint);
    // The core chain (pelvis, spine) carries the limbs: no core body lighter than its heaviest
    // child. A small pelvis capsule under two heavy thighs was the lightest body of the tree,
    // and its joints gave way under a hit (AN-18: the torso spun, the spine broke its limits).
    if let Some(core) = regions.iter().position(|r| r == CORE_REGION) {
        for i in (1..bodies.len()).rev() {
            let p = usize::from(bodies[i].parent.expect("non-root"));
            if usize::from(bodies[p].region) == core {
                bodies[p].mass = bodies[p].mass.max(bodies[i].mass);
            }
        }
    }
    // Mass balancing down the tree, then the adapter's total.
    for i in 1..bodies.len() {
        let p = usize::from(bodies[i].parent.expect("non-root"));
        let floor = bodies[p].mass / params.max_mass_ratio.max(1.0);
        bodies[i].mass = bodies[i].mass.max(floor);
    }
    let sum: f32 = bodies.iter().map(|b| b.mass).sum();
    let total = params.total_mass.unwrap_or(sum);
    for b in &mut bodies {
        b.mass *= total / sum;
    }
    // Bodies that overlap at rest never collide (UE: "disable collision on
    // overlapping bodies"); jointed neighbors are off through their joint.
    let mut no_collide = Vec::new();
    for a in 0..specs.len() {
        for c in a + 1..specs.len() {
            if bodies[c].parent == u16::try_from(a).ok() {
                continue;
            }
            let (sa, sc) = (&specs[a], &specs[c]);
            let seg = |s: &Spec| (s.center - s.axis * s.half, s.center + s.axis * s.half);
            let ((p1, q1), (p2, q2)) = (seg(sa), seg(sc));
            if segment_distance(p1, q1, p2, q2) < sa.radius + sc.radius {
                no_collide.push((u16::try_from(a).expect("u16"), u16::try_from(c).expect("u16")));
            }
        }
    }
    Ok(RagdollDef { skeleton: skeleton.id(), policy: params.policy, bodies, joints, regions, total_mass: total, no_collide, owner })
}

fn rest_pos(rest: &[glam::Affine3A], j: usize) -> Vec3 {
    Vec3::from(rest[j].translation)
}

#[cfg(test)]
pub(crate) fn segment_distance_for_tests(p1: Vec3, q1: Vec3, p2: Vec3, q2: Vec3) -> f32 {
    segment_distance(p1, q1, p2, q2)
}
