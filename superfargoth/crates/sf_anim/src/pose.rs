//! Pose buffers and pose ops.
//!
//! Local poses are stored **structure-of-arrays, four joints wide**
//! ([`SoaTrs`], one `glam::Vec4` per component, as ozz-animation's
//! `SoaTransform`). Blending, masking and additive ops then have no
//! horizontal (cross-lane) work: a normalized-lerp of four quaternions is a
//! handful of 4-wide multiply-adds and one 4-wide normalize, where the
//! array-of-structs form needs a dot product and a normalize per joint.
//! Model-space poses are array-of-structs [`Affine3A`]s, because the
//! parent-child product is inherently one joint at a time.

use glam::{Affine3A, Quat, Vec3, Vec4};

use crate::{JointMask, Skeleton, Trs};

/// Four joints' local transforms, one SIMD lane per joint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoaTrs {
    pub tx: Vec4,
    pub ty: Vec4,
    pub tz: Vec4,
    pub rx: Vec4,
    pub ry: Vec4,
    pub rz: Vec4,
    pub rw: Vec4,
    pub sx: Vec4,
    pub sy: Vec4,
    pub sz: Vec4,
}

impl Default for SoaTrs {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl SoaTrs {
    pub const IDENTITY: Self = Self {
        tx: Vec4::ZERO,
        ty: Vec4::ZERO,
        tz: Vec4::ZERO,
        rx: Vec4::ZERO,
        ry: Vec4::ZERO,
        rz: Vec4::ZERO,
        rw: Vec4::ONE,
        sx: Vec4::ONE,
        sy: Vec4::ONE,
        sz: Vec4::ONE,
    };

    #[inline]
    pub fn get(&self, lane: usize) -> Trs {
        Trs {
            translation: Vec3::new(self.tx[lane], self.ty[lane], self.tz[lane]),
            rotation: Quat::from_xyzw(self.rx[lane], self.ry[lane], self.rz[lane], self.rw[lane]),
            scale: Vec3::new(self.sx[lane], self.sy[lane], self.sz[lane]),
        }
    }

    #[inline]
    pub fn set(&mut self, lane: usize, t: Trs) {
        self.set_translation(lane, t.translation);
        self.set_rotation(lane, t.rotation);
        self.set_scale(lane, t.scale);
    }

    #[inline]
    pub fn set_translation(&mut self, lane: usize, v: Vec3) {
        self.tx[lane] = v.x;
        self.ty[lane] = v.y;
        self.tz[lane] = v.z;
    }

    #[inline]
    pub fn set_rotation(&mut self, lane: usize, q: Quat) {
        self.rx[lane] = q.x;
        self.ry[lane] = q.y;
        self.rz[lane] = q.z;
        self.rw[lane] = q.w;
    }

    #[inline]
    pub fn set_scale(&mut self, lane: usize, v: Vec3) {
        self.sx[lane] = v.x;
        self.sy[lane] = v.y;
        self.sz[lane] = v.z;
    }
}

/// A skeleton's local pose (joint transforms relative to their parents).
#[derive(Clone, Debug, PartialEq)]
pub struct LocalPose {
    soa: Vec<SoaTrs>,
    len: usize,
}

impl LocalPose {
    /// `len` identity joints. Padding lanes are always identity.
    pub fn new(len: usize) -> Self {
        Self { soa: vec![SoaTrs::IDENTITY; len.div_ceil(4)], len }
    }

    pub fn from_trs(joints: &[Trs]) -> Self {
        let mut pose = Self::new(joints.len());
        for (j, t) in joints.iter().enumerate() {
            pose.set(j, *t);
        }
        pose
    }

    pub fn rest(skeleton: &Skeleton) -> Self {
        Self::from_trs(skeleton.rest())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn groups(&self) -> &[SoaTrs] {
        &self.soa
    }

    pub fn groups_mut(&mut self) -> &mut [SoaTrs] {
        &mut self.soa
    }

    #[inline]
    pub fn get(&self, joint: usize) -> Trs {
        debug_assert!(joint < self.len);
        self.soa[joint / 4].get(joint % 4)
    }

    #[inline]
    pub fn set(&mut self, joint: usize, t: Trs) {
        debug_assert!(joint < self.len);
        self.soa[joint / 4].set(joint % 4, t);
    }

    #[inline]
    pub fn set_rotation(&mut self, joint: usize, q: Quat) {
        self.soa[joint / 4].set_rotation(joint % 4, q);
    }

    #[inline]
    pub fn set_translation(&mut self, joint: usize, v: Vec3) {
        self.soa[joint / 4].set_translation(joint % 4, v);
    }

    #[inline]
    pub fn set_scale(&mut self, joint: usize, v: Vec3) {
        self.soa[joint / 4].set_scale(joint % 4, v);
    }

    pub fn copy_from(&mut self, other: &Self) {
        self.len = other.len;
        self.soa.clone_from(&other.soa);
    }

    pub fn to_vec(&self) -> Vec<Trs> {
        (0..self.len).map(|j| self.get(j)).collect()
    }
}

/// Per-joint weights in `[0, 1]` (a blend mask or blend profile), stored
/// four-wide like poses.
#[derive(Clone, Debug, PartialEq)]
pub struct JointWeights {
    soa: Vec<Vec4>,
    len: usize,
}

impl JointWeights {
    pub fn splat(len: usize, w: f32) -> Self {
        let mut s = Self { soa: vec![Vec4::ZERO; len.div_ceil(4)], len };
        for j in 0..len {
            s.set(j, w);
        }
        s
    }

    /// `inside` for joints in `mask`, `outside` for the others.
    pub fn from_mask(mask: &JointMask, inside: f32, outside: f32) -> Self {
        let mut s = Self::splat(mask.len(), outside);
        for j in mask.iter() {
            s.set(j, inside);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, joint: usize) -> f32 {
        self.soa[joint / 4][joint % 4]
    }

    pub fn set(&mut self, joint: usize, w: f32) {
        assert!(joint < self.len);
        self.soa[joint / 4][joint % 4] = w;
    }

    pub fn groups(&self) -> &[Vec4] {
        &self.soa
    }
}

// --- 4-wide quaternion helpers -------------------------------------------

#[inline]
fn dot4(a: &SoaTrs, b: &SoaTrs) -> Vec4 {
    a.rx * b.rx + a.ry * b.ry + a.rz * b.rz + a.rw * b.rw
}

#[inline]
fn normalize_rot(g: &mut SoaTrs) {
    let len2 = g.rx * g.rx + g.ry * g.ry + g.rz * g.rz + g.rw * g.rw;
    let inv = Vec4::ONE / len2.sqrt();
    g.rx *= inv;
    g.ry *= inv;
    g.rz *= inv;
    g.rw *= inv;
}

/// `a·(1-w) + b·w` per lane, rotations by normalized lerp on the short arc.
#[inline]
fn blend_group(a: &SoaTrs, b: &SoaTrs, w: Vec4) -> SoaTrs {
    let wa = Vec4::ONE - w;
    let wb = Vec4::select(dot4(a, b).cmplt(Vec4::ZERO), -w, w);
    let mut out = SoaTrs {
        tx: a.tx * wa + b.tx * w,
        ty: a.ty * wa + b.ty * w,
        tz: a.tz * wa + b.tz * w,
        rx: a.rx * wa + b.rx * wb,
        ry: a.ry * wa + b.ry * wb,
        rz: a.rz * wa + b.rz * wb,
        rw: a.rw * wa + b.rw * wb,
        sx: a.sx * wa + b.sx * w,
        sy: a.sy * wa + b.sy * w,
        sz: a.sz * wa + b.sz * w,
    };
    normalize_rot(&mut out);
    out
}

/// Hamilton product `a·b`, four lanes at once (rotation part only).
#[inline]
#[allow(clippy::many_single_char_names)]
fn quat_mul4(a: (Vec4, Vec4, Vec4, Vec4), b: (Vec4, Vec4, Vec4, Vec4)) -> (Vec4, Vec4, Vec4, Vec4) {
    let (ax, ay, az, aw) = a;
    let (bx, by, bz, bw) = b;
    let x = aw * bx + ax * bw + ay * bz - az * by;
    let y = aw * by - ax * bz + ay * bw + az * bx;
    let z = aw * bz + ax * by - ay * bx + az * bw;
    let w = aw * bw - ax * bx - ay * by - az * bz;
    (x, y, z, w)
}

// --- Pose ops --------------------------------------------------------------

/// `out = lerp(a, b, t)`: translations and scales lerp, rotations nlerp.
pub fn blend(out: &mut LocalPose, a: &LocalPose, b: &LocalPose, t: f32) {
    assert!(a.len == b.len, "poses of different skeletons");
    out.len = a.len;
    out.soa.resize(a.soa.len(), SoaTrs::IDENTITY);
    let w = Vec4::splat(t);
    for ((o, a), b) in out.soa.iter_mut().zip(&a.soa).zip(&b.soa) {
        *o = blend_group(a, b, w);
    }
}

/// `a = lerp(a, b, t · weights[joint])`: a masked (per-joint weighted)
/// blend in place. `weights = None` blends every joint by `t`.
pub fn blend_in_place(a: &mut LocalPose, b: &LocalPose, t: f32, weights: Option<&JointWeights>) {
    assert!(a.len == b.len, "poses of different skeletons");
    let t = Vec4::splat(t);
    match weights {
        Some(weights) => {
            assert!(weights.len == a.len, "weights of a different skeleton");
            for ((a, b), w) in a.soa.iter_mut().zip(&b.soa).zip(&weights.soa) {
                *a = blend_group(a, b, t * *w);
            }
        }
        None => {
            for (a, b) in a.soa.iter_mut().zip(&b.soa) {
                *a = blend_group(a, b, t);
            }
        }
    }
}

/// One input of [`blend_layers`].
#[derive(Clone, Copy, Debug)]
pub struct BlendLayer<'a> {
    pub pose: &'a LocalPose,
    pub weight: f32,
    /// Per-joint weights multiplied into `weight`.
    pub joint_weights: Option<&'a JointWeights>,
}

/// Weighted N-way blend (ozz's `BlendingJob`): every layer is accumulated
/// with its weight, rotations sign-corrected against the running sum. Joints
/// whose total weight is below `threshold` are completed with `rest`, so a
/// joint no layer drives keeps its rest transform.
pub fn blend_layers(out: &mut LocalPose, layers: &[BlendLayer<'_>], rest: &LocalPose, threshold: f32) {
    let len = rest.len;
    assert!(layers.iter().all(|l| l.pose.len == len), "poses of different skeletons");
    out.len = len;
    out.soa.resize(rest.soa.len(), SoaTrs::IDENTITY);
    let threshold = Vec4::splat(threshold.max(f32::EPSILON));
    for (g, o) in out.soa.iter_mut().enumerate() {
        let mut acc = SoaTrs {
            tx: Vec4::ZERO,
            ty: Vec4::ZERO,
            tz: Vec4::ZERO,
            rx: Vec4::ZERO,
            ry: Vec4::ZERO,
            rz: Vec4::ZERO,
            rw: Vec4::ZERO,
            sx: Vec4::ZERO,
            sy: Vec4::ZERO,
            sz: Vec4::ZERO,
        };
        let mut total = Vec4::ZERO;
        for layer in layers {
            let mut w = Vec4::splat(layer.weight.max(0.0));
            if let Some(jw) = layer.joint_weights {
                w *= jw.soa[g];
            }
            accumulate(&mut acc, &mut total, &layer.pose.soa[g], w);
        }
        let fill = (threshold - total).max(Vec4::ZERO);
        accumulate(&mut acc, &mut total, &rest.soa[g], fill);
        let inv = Vec4::ONE / total;
        acc.tx *= inv;
        acc.ty *= inv;
        acc.tz *= inv;
        acc.sx *= inv;
        acc.sy *= inv;
        acc.sz *= inv;
        normalize_rot(&mut acc);
        *o = acc;
    }
}

/// Adds `src · w` to a [`blend_layers`] accumulator, rotations
/// sign-corrected against the running sum.
#[inline]
fn accumulate(acc: &mut SoaTrs, total: &mut Vec4, src: &SoaTrs, w: Vec4) {
    let wr = Vec4::select(dot4(acc, src).cmplt(Vec4::ZERO), -w, w);
    acc.tx += src.tx * w;
    acc.ty += src.ty * w;
    acc.tz += src.tz * w;
    acc.rx += src.rx * wr;
    acc.ry += src.ry * wr;
    acc.rz += src.rz * wr;
    acc.rw += src.rw * wr;
    acc.sx += src.sx * w;
    acc.sy += src.sy * w;
    acc.sz += src.sz * w;
    *total += w;
}

/// The additive delta of `pose` over `base`: `Δt = t - t_b`,
/// `Δq = q · q_b⁻¹`, `Δs = s / s_b`.
pub fn make_additive(out: &mut LocalPose, pose: &LocalPose, base: &LocalPose) {
    assert!(pose.len == base.len, "poses of different skeletons");
    out.len = pose.len;
    out.soa.resize(pose.soa.len(), SoaTrs::IDENTITY);
    for ((o, p), b) in out.soa.iter_mut().zip(&pose.soa).zip(&base.soa) {
        let (rx, ry, rz, rw) = quat_mul4((p.rx, p.ry, p.rz, p.rw), (-b.rx, -b.ry, -b.rz, b.rw));
        *o = SoaTrs { tx: p.tx - b.tx, ty: p.ty - b.ty, tz: p.tz - b.tz, rx, ry, rz, rw, sx: p.sx / b.sx, sy: p.sy / b.sy, sz: p.sz / b.sz };
        normalize_rot(o);
    }
}

/// Applies an additive delta with weight `weight` (times per-joint
/// weights): `t += w·Δt`, `q = nlerp(I, Δq, w) · q`, `s *= lerp(1, Δs, w)`.
pub fn apply_additive(pose: &mut LocalPose, delta: &LocalPose, weight: f32, joint_weights: Option<&JointWeights>) {
    assert!(pose.len == delta.len, "poses of different skeletons");
    for (g, (p, d)) in pose.soa.iter_mut().zip(&delta.soa).enumerate() {
        let mut w = Vec4::splat(weight);
        if let Some(jw) = joint_weights {
            w *= jw.soa[g];
        }
        let wi = Vec4::ONE - w;
        // Short arc from identity: flip deltas with negative w.
        let ws = Vec4::select(d.rw.cmplt(Vec4::ZERO), -w, w);
        let (mut qx, mut qy, mut qz, mut qw) = (d.rx * ws, d.ry * ws, d.rz * ws, wi + d.rw * ws);
        let inv = Vec4::ONE / (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
        qx *= inv;
        qy *= inv;
        qz *= inv;
        qw *= inv;
        let (rx, ry, rz, rw) = quat_mul4((qx, qy, qz, qw), (p.rx, p.ry, p.rz, p.rw));
        p.rx = rx;
        p.ry = ry;
        p.rz = rz;
        p.rw = rw;
        p.tx += d.tx * w;
        p.ty += d.ty * w;
        p.tz += d.tz * w;
        p.sx *= wi + d.sx * w;
        p.sy *= wi + d.sy * w;
        p.sz *= wi + d.sz * w;
    }
}

/// Model-space joint transforms.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelPose {
    pub mats: Vec<Affine3A>,
}

impl ModelPose {
    pub fn new(len: usize) -> Self {
        Self { mats: vec![Affine3A::IDENTITY; len] }
    }

    pub fn translation(&self, joint: usize) -> Vec3 {
        Vec3::from(self.mats[joint].translation)
    }

    pub fn rotation(&self, joint: usize) -> Quat {
        crate::math::affine_rotation(&self.mats[joint])
    }
}

/// Local to model space for every joint; root joints are placed under `root`.
pub fn local_to_model(skeleton: &Skeleton, local: &LocalPose, model: &mut ModelPose, root: &Affine3A) {
    let n = skeleton.len();
    assert!(local.len == n, "pose of a different skeleton");
    model.mats.resize(n, Affine3A::IDENTITY);
    local_to_model_range(skeleton, local, model, root, 0..n);
}

/// Recomputes the model transforms of `joint` and its subtree after its
/// (or its descendants') local transforms changed.
pub fn update_subtree(skeleton: &Skeleton, local: &LocalPose, model: &mut ModelPose, root: &Affine3A, joint: usize) {
    local_to_model_range(skeleton, local, model, root, skeleton.subtree(joint));
}

fn local_to_model_range(skeleton: &Skeleton, local: &LocalPose, model: &mut ModelPose, root: &Affine3A, range: std::ops::Range<usize>) {
    let parents = skeleton.parents();
    let mats = &mut model.mats;
    let mut j = range.start;
    while j < range.end {
        let g = &local.soa[j / 4];
        let (tx, ty, tz) = (g.tx.to_array(), g.ty.to_array(), g.tz.to_array());
        let (rx, ry, rz, rw) = (g.rx.to_array(), g.ry.to_array(), g.rz.to_array(), g.rw.to_array());
        let (sx, sy, sz) = (g.sx.to_array(), g.sy.to_array(), g.sz.to_array());
        let lane_end = (j / 4 * 4 + 4).min(range.end);
        while j < lane_end {
            let l = j % 4;
            let m = Affine3A::from_scale_rotation_translation(
                Vec3::new(sx[l], sy[l], sz[l]),
                Quat::from_xyzw(rx[l], ry[l], rz[l], rw[l]),
                Vec3::new(tx[l], ty[l], tz[l]),
            );
            let p = parents[j];
            mats[j] = if p == crate::NO_PARENT { *root * m } else { mats[usize::from(p)] * m };
            j += 1;
        }
    }
}
