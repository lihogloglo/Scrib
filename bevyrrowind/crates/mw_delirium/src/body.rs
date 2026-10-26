//! A Delirium character (the `nm-character-1` asset and its build) from Morrowind's humanoid skeleton.
//!
//! Ported from skyrim-delirium (GPL-3.0-only), `fit-test/examples/make_character.rs` at commit
//! `e44cb38d54bd79d8f2b54cc863a0df36df7e0542` (github.com/YoucefNabil/skyrim-delirium), which builds one from the
//! Skyrim skeleton's joint positions; its Spine3 rule (halfway from the chest bone to the neck) comes from the same
//! repository's `tools/skel_joints.py`. Changes: the joint positions and bone rotations are read from an
//! [`sf_anim::Skeleton`] (Morrowind's `Bip01` bones through the humanoid rig map) instead of a JSON dump; segment
//! masses, the arms-out angle and the torque scale are [`Options`]; the per-part bone table ([`PartTable`]) replaces
//! their plugin's rig text file; the self-checks moved into the tests. The numbers (Winter segment masses, part sizes,
//! joint ranges, strengths) are theirs.
//!
//! What follows Delirium's behaviours is convention (from their notes): the joint axes, hinges at the knees and
//! elbows, the mirrored right side carrying the flips, the zero pose with the upper arms out from the body and the
//! legs straight down, the pelvis frame a = up, b = left, c = back with its capsule across the hips.

use delirium::prelude::*;
use delirium_physics::character::{BuildPart, Character, FORMAT, Frame, Joint, JointLimit, Part};
use sf_anim::Skeleton;
use sf_character::HumanoidBone as B;

use crate::pose::{PartBone, PartTable};
use crate::{authoring_from_model, authoring_point};

/// Delirium's 21 parts, in its order.
pub const NAMES: [&str; 21] = [
    "Buttocks", "Thigh_Left", "Shin_Left", "Foot_Left", "Thigh_Right", "Shin_Right", "Foot_Right", "Spine0", "Spine1", "Spine2",
    "Spine3", "Neck", "Head", "Clavicle_Left", "Upper_Arm_Left", "Lower_Arm_Left", "Hand_Left", "Clavicle_Right",
    "Upper_Arm_Right", "Lower_Arm_Right", "Hand_Right",
];
/// Each part's parent part.
pub const PARENT: [i32; 21] = [-1, 0, 1, 2, 0, 4, 5, 0, 7, 8, 9, 10, 11, 10, 13, 14, 15, 10, 17, 18, 19];
/// Delirium's 20 joints; joint `i` connects part `i + 1` to its parent.
pub const JOINTS: [&str; 20] = [
    "hip_left", "knee_left", "ankle_left", "hip_right", "knee_right", "ankle_right", "spine_0", "spine_1", "spine_2", "spine_3",
    "neck_lower", "neck_upper", "clavicle_left", "shoulder_left", "elbow_left", "wrist_left", "clavicle_right",
    "shoulder_right", "elbow_right", "wrist_right",
];
/// Part `k`'s parent part.
pub fn parent(k: usize) -> Option<usize> {
    usize::try_from(PARENT[k]).ok()
}
const RIGHT: [usize; 7] = [4, 5, 6, 17, 18, 19, 20];

/// The humanoid bone each part reads its joint position from (`None`: Spine3, halfway from the chest bone to the
/// neck). Morrowind has three spine bones (`Bip01 Spine`, `Spine1`, `Spine2`) for Delirium's four spine parts.
const BONE: [Option<B>; 21] = [
    Some(B::Hips), Some(B::LeftUpperLeg), Some(B::LeftLowerLeg), Some(B::LeftFoot), Some(B::RightUpperLeg),
    Some(B::RightLowerLeg), Some(B::RightFoot), Some(B::Spine), Some(B::Chest), Some(B::UpperChest), None, Some(B::Neck),
    Some(B::Head), Some(B::LeftShoulder), Some(B::LeftUpperArm), Some(B::LeftLowerArm), Some(B::LeftHand),
    Some(B::RightShoulder), Some(B::RightUpperArm), Some(B::RightLowerArm), Some(B::RightHand),
];

/// Segment masses, % of the body (after Winter's segment tables; the trunk split over pelvis, four spine parts and the
/// shoulder girdles, head and neck 6.9 / 1.2). The trunk carries its weight low and the feet include boots: with the
/// chest heavier the balancer reads a high centre of mass (skyrim-delirium's finding).
pub const MASS_PCT_TRUNK_LOW: [f32; 21] =
    [14.2, 10.0, 4.65, 2.0, 10.0, 4.65, 2.0, 9.0, 8.0, 7.0, 7.4, 1.2, 6.9, 1.5, 2.8, 1.6, 0.6, 1.5, 2.8, 1.6, 0.6];
/// The same total with an even trunk and bare feet (for the "what mattered" comparison only).
pub const MASS_PCT_EVEN: [f32; 21] =
    [11.2, 10.0, 4.65, 1.45, 10.0, 4.65, 1.45, 8.6, 8.6, 9.6, 9.6, 1.2, 6.9, 1.5, 2.8, 1.6, 0.6, 1.5, 2.8, 1.6, 0.6];

/// Knobs of the character.
#[derive(Clone, Debug)]
pub struct Options {
    /// Zero pose: upper arms out from straight down, degrees (skyrim-delirium: 55).
    pub arm_out_deg: f32,
    /// Scales every joint's peak torque.
    pub torque: f32,
    /// Body mass, kg.
    pub body_kg: f32,
    /// Segment masses, % of `body_kg`.
    pub mass_pct: [f32; 21],
}

impl Default for Options {
    fn default() -> Self {
        Self { arm_out_deg: 55.0, torque: 1.0, body_kg: 80.0, mass_pct: MASS_PCT_TRUNK_LOW }
    }
}

/// A Delirium character for one skeleton: the asset and build (`character`), and where its parts sit on the bones.
pub struct Body {
    pub character: Character,
    pub table: PartTable,
    /// Part frames in the bind pose (authoring space: Z up, facing +Y, metres; the skeleton's model space through
    /// [`crate::authoring_point`]).
    pub bind: Vec<Transform>,
    /// Part frames in the zero pose (arms out, legs straight), authoring space.
    pub zero: Vec<Transform>,
    /// Bent at the bind pose, degrees: knee, elbow, hip out (abduction).
    pub bind_bends: [f32; 3],
}

impl Body {
    /// The `nm-character-1` text (with its build).
    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self.character).expect("serialisable")
    }
}

/// Builds the character from a humanoid skeleton in model space (`sf_anim` convention: Y up, facing +Z, metres), read in
/// its rest pose. The skeleton must auto-map as a humanoid ([`mw_anim::rig::humanoid`]).
///
/// # Errors
/// The skeleton does not map as a humanoid, or misses one of the 20 bones.
#[allow(clippy::too_many_lines, clippy::similar_names, clippy::many_single_char_names)]
pub fn character_from_skeleton(skeleton: &Skeleton, opt: &Options) -> Result<Body, String> {
    let rig = mw_anim::rig::humanoid(skeleton).map_err(|e| format!("not a humanoid: {e}"))?;
    let model = skeleton.rest_model();
    let joint = |b: B| -> Result<usize, String> {
        let name = rig.map.joint(b).ok_or_else(|| format!("no {b:?} bone"))?;
        skeleton.find(name).ok_or_else(|| format!("no joint {name}"))
    };
    let bone_pos = |j: usize| authoring_point(model[j].translation.into());
    // a bone frame carried into authoring space as a rigid body (A * M), so that part and bone frames compose
    let bone_rot = |j: usize| {
        let (_, r, _) = model[j].to_scale_rotation_translation();
        (Quat::from_mat3a(&authoring_from_model().matrix3) * r).normalize()
    };
    let upper_chest = joint(B::UpperChest)?;
    let neck = joint(B::Neck)?;
    let mut pv = Vec::with_capacity(21);
    for b in BONE {
        pv.push(match b {
            Some(b) => bone_pos(joint(b)?),
            None => (bone_pos(upper_chest) + bone_pos(neck)) * 0.5,
        });
    }

    let arm_out = opt.arm_out_deg.to_radians();
    let up = Vec3::Z;
    let left = (pv[1] - pv[4]).normalize();
    let fwd = left.cross(up).normalize(); // right x fwd = up
    let (right, back) = (-left, -fwd);
    let side = |k: usize| if RIGHT.contains(&k) { right } else { left };
    let mirror = |v: Vec3| v - 2.0 * v.dot(left) * left;

    // the far end of each part, and where it points
    let tip = |k: usize| -> Vec3 {
        match k {
            0 => pv[0] + up * 0.1,
            3 | 6 => pv[k] + fwd * 0.15,
            12 => pv[12] + up * 0.2,
            16 | 20 => pv[k] + (pv[k] - pv[k - 1]).normalize() * 0.18,
            _ => pv[(0..21).find(|&c| parent(c) == Some(k)).expect("a child")],
        }
    };
    let along = |k: usize| (tip(k) - pv[k]).normalize();

    // part frames (bind): the capsule runs along y. Pelvis and spine lie across the body, limbs point back to their
    // joint, z is the body's back for everything but the shoulder girdles
    let yz = |y: Vec3, zh: Vec3| {
        let y = y.normalize();
        let z = (zh - y * zh.dot(y)).normalize();
        Quat::from_mat3(&Mat3::from_cols(y.cross(z), y, z)).normalize()
    };
    let xy = |xh: Vec3, y: Vec3| {
        let y = y.normalize();
        let x = (xh - y * xh.dot(y)).normalize();
        Quat::from_mat3(&Mat3::from_cols(x, y, x.cross(y))).normalize()
    };
    let rot: Vec<Quat> = (0..21)
        .map(|k| match k {
            0 => xy(up, left),
            7..=10 => xy(along(k), left),
            11 | 12 => yz(along(k), back),
            3 | 6 => yz(up, back),
            13 | 17 => xy(up, -along(k)),
            _ => yz(-along(k), back),
        })
        .collect();

    // sizes (m): capsule radius and half length (between the cap centres), or a box's half extents
    let len = |k: usize| (tip(k) - pv[k]).length();
    let limb = |k: usize, r: f32| (r, (len(k) * 0.5 - 0.8 * r).max(0.01));
    let size: Vec<(f32, f32, Option<Vec3>)> = (0..21)
        .map(|k| match k {
            0 => (0.11, 0.06, None),
            1 | 4 => {
                let (r, h) = limb(k, 0.075);
                (r, h, None)
            }
            2 | 5 => {
                let (r, h) = limb(k, 0.055);
                (r, h, None)
            }
            3 | 6 => (0.04, 0.0, Some(Vec3::new(0.05, 0.04, 0.135))), // boxes: radius = half height
            7 => (0.10, 0.05, None),
            8 => (0.10, 0.06, None),
            9 => (0.105, 0.065, None),
            10 => (0.10, 0.08, None),
            11 => (0.06, len(k) * 0.5, None), // neck: caps reach into the chest and the head (no self-collision)
            12 => (0.085, 0.03, None),
            13 | 17 | 15 | 19 => {
                let (r, h) = limb(k, 0.04);
                (r, h, None)
            }
            14 | 18 => {
                let (r, h) = limb(k, 0.048);
                (r, h, None)
            }
            _ => (0.03, 0.0, Some(Vec3::new(0.03, 0.08, 0.045))),
        })
        .collect();
    // part centres: the torso sits in front of the spine bones, the head above its joint, the foot ahead of the ankle
    let centre: Vec<Vec3> = (0..21)
        .map(|k| match k {
            0 => pv[0],
            7..=9 => (pv[k] + tip(k)) * 0.5 + fwd * 0.06,
            10 => (pv[k] + tip(k)) * 0.5 + fwd * 0.05,
            12 => pv[12] + up * 0.09 + fwd * 0.03,
            3 | 6 => pv[k] + fwd * 0.065 - up * 0.045,
            16 | 20 => pv[k] + along(k) * 0.08,
            _ => (pv[k] + tip(k)) * 0.5,
        })
        .collect();

    // zero pose: the bind pose with the upper arms turned out to arm_out from straight down (pure abduction)
    let mut zrot = rot.clone();
    let (mut zc, mut zp) = (centre.clone(), pv.clone());
    for (sh, s) in [(14usize, 1.0f32), (18, -1.0)] {
        let u = along(sh);
        let now = (u.dot(side(sh))).atan2(-u.dot(up));
        let turn = Quat::from_axis_angle(fwd * s, arm_out - now);
        for k in [sh, sh + 1, sh + 2] {
            zrot[k] = (turn * rot[k]).normalize();
            zc[k] = pv[sh] + turn * (centre[k] - pv[sh]);
            zp[k] = pv[sh] + turn * (pv[k] - pv[sh]);
        }
    }
    // and the legs straight down: the balancer stands the pelvis at 0.99 of this pose's pelvis-to-feet height, so a
    // bent or splayed zero leg would make it crouch
    for h in [1usize, 4] {
        let th = Quat::from_rotation_arc(along(h), -up);
        for k in [h, h + 1, h + 2] {
            zrot[k] = (th * rot[k]).normalize();
            zc[k] = pv[h] + th * (centre[k] - pv[h]);
            zp[k] = pv[h] + th * (pv[k] - pv[h]);
        }
        let (kn, o) = (h + 1, zp[h + 1]);
        let tk = Quat::from_rotation_arc((zp[kn + 1] - o).normalize(), -up);
        for k in [kn, kn + 1] {
            zrot[k] = (tk * zrot[k]).normalize();
            zc[k] = o + tk * (zc[k] - o);
            zp[k] = o + tk * (zp[k] - o);
        }
        // the foot stays flat (shin upright over a flat foot: the ankle's neutral)
        let f = h + 2;
        zrot[f] = rot[f];
        zc[f] = zp[f] + (centre[f] - pv[f]);
    }
    let zalong = |k: usize| match k {
        1..=6 | 14..=16 | 18..=20 => (zrot[k] * (rot[k].inverse() * along(k))).normalize(),
        _ => along(k),
    };

    // joints: twist axis (z) along the child part, lean axis (x) the body's front; the ankle twists along the foot and
    // leans about up; knees and elbows are hinges about the body's right
    let hinge = |k: usize| matches!(k, 2 | 5 | 15 | 19);
    let axes = |k: usize| -> (Vec3, Vec3) {
        match k {
            3 | 6 => (fwd, up),
            2 | 5 => (right, right.cross(Vec3::Z)),
            15 | 19 => {
                let n = zalong(k - 1).cross(zalong(k)).normalize();
                let n = if n.dot(right) < 0.0 { -n } else { n };
                (n, n.cross(Vec3::Z))
            }
            _ => (zalong(k), fwd),
        }
    };
    // the joint frame as Delirium builds it (nmrs::joints): z twist, x lean made square to it, y = z x x
    let frame = |k: usize| {
        let (z, x0) = axes(k);
        let z = z.normalize();
        let x = (x0 - z * x0.dot(z)).normalize();
        (x, z.cross(x), z)
    };

    // ranges (deg), from an upright neutral, as rotations about body axes for the left side and the middle:
    // (axis, low, high). Delirium's raw angles: lean1 about the frame's y, lean2 about -x, twist about z.
    let bend = |a: Vec3, b: Vec3, about: Vec3| a.cross(b).dot(about).atan2(a.dot(b)).to_degrees();
    let knee_bent = bend(zalong(1), zalong(2), -right);
    let elbow_bent = bend(zalong(14), zalong(15), axes(15).0);
    let hip_out = bend(-up, zalong(1), fwd);
    let motions = |k: usize| -> Vec<(Vec3, f32, f32)> {
        let al = zalong(k);
        match k {
            1 => vec![(right, -25.0, 120.0), (fwd, -25.0 - hip_out, 45.0 - hip_out), (al, -45.0, 40.0)],
            2 => vec![(-right, -3.0 - knee_bent, 145.0 - knee_bent)],
            // centred and wide in flexion: the balancer needs room both ways at the ankle to catch a hit
            3 => vec![(right, -55.0, 45.0), (-up, -17.0, 17.0), (-fwd, -22.0, 22.0)],
            7 => vec![(-right, -15.0, 25.0), (-fwd, -10.0, 10.0), (up, -5.0, 5.0)],
            8 => vec![(-right, -12.0, 20.0), (-fwd, -12.0, 12.0), (up, -8.0, 8.0)],
            9 => vec![(-right, -10.0, 15.0), (-fwd, -12.0, 12.0), (up, -15.0, 15.0)],
            10 => vec![(-right, -10.0, 12.0), (-fwd, -10.0, 10.0), (up, -15.0, 15.0)],
            11 => vec![(-right, -30.0, 25.0), (-fwd, -20.0, 20.0), (up, -35.0, 35.0)],
            12 => vec![(-right, -30.0, 25.0), (-fwd, -20.0, 20.0), (up, -40.0, 40.0)],
            13 => vec![(-up, -25.0, 20.0), (fwd, -10.0, 35.0), (al, -20.0, 20.0)],
            14 => {
                let out = opt.arm_out_deg;
                vec![(al.cross(fwd).normalize(), -50.0, 110.0), (fwd, -30.0 - out, 120.0 - out), (al, -60.0, 70.0)]
            }
            15 => vec![(axes(15).0, -elbow_bent, 145.0 - elbow_bent)],
            16 => vec![(-fwd, -60.0, 60.0), (al.cross(fwd).normalize(), -30.0, 25.0), (al, -40.0, 40.0)],
            _ => unreachable!("no ranges for part {k}"),
        }
    };
    // the same part on the left, for a right one
    let twin = |k: usize| match k {
        4..=6 => k - 3,
        17..=20 => k - 4,
        _ => k,
    };
    let limits = |k: usize| -> Result<[[f32; 2]; 3], String> {
        let (x, y, z) = frame(k);
        let mut out = [[0.0f32; 2]; 3];
        let mut seen = [false; 3];
        for (a, lo, hi) in motions(twin(k)) {
            // a right part moves as the mirror image: about -M(a), same angle
            let a = if twin(k) == k { a } else { -mirror(a) };
            let a = a.normalize();
            let (slot, s) = if hinge(k) {
                (0, a.dot(z).signum())
            } else {
                let d = [a.dot(y), -a.dot(x), a.dot(z)];
                let m = (0..3).max_by(|&p, &q| d[p].abs().total_cmp(&d[q].abs())).expect("three");
                (m, d[m].signum())
            };
            if seen[slot] {
                return Err(format!("{}: two motions on one axis", NAMES[k]));
            }
            seen[slot] = true;
            let (l, h) = ((s * lo).to_radians(), (s * hi).to_radians());
            out[slot] = [l.min(h), l.max(h)];
        }
        Ok(out)
    };

    // muscles: gain k (strength k^2, damping 2k; behaviours set their own), peak torques in N m (lean, twist) after
    // published isometric joint strengths for adult men
    let muscle = |i: usize| -> (f32, [f32; 2]) {
        match JOINTS[i].trim_end_matches("_left").trim_end_matches("_right") {
            "hip" => (12.0, [220.0, 70.0]),
            "knee" => (12.0, [200.0, 0.0]),
            "ankle" => (9.0, [150.0, 40.0]),
            "spine_0" => (12.0, [220.0, 90.0]),
            "spine_1" => (12.0, [200.0, 80.0]),
            "spine_2" => (12.0, [180.0, 70.0]),
            "spine_3" => (12.0, [160.0, 60.0]),
            "neck_lower" => (8.0, [50.0, 25.0]),
            "neck_upper" => (8.0, [40.0, 20.0]),
            "clavicle" => (10.0, [70.0, 30.0]),
            "shoulder" => (10.0, [90.0, 45.0]),
            "elbow" => (10.0, [75.0, 0.0]),
            _ => (9.0, [15.0, 8.0]), // wrist
        }
    };

    // parts that can touch (not neighbours): arms with the torso, legs and head; legs with each other and the trunk
    let mut pairs: Vec<[usize; 2]> = vec![];
    let mut add = |a: &[usize], b: &[usize]| {
        for &p in a {
            for &q in b {
                if p != q && !pairs.iter().any(|&[x, y]| (x, y) == (p.min(q), p.max(q))) {
                    pairs.push([p.min(q), p.max(q)]);
                }
            }
        }
    };
    for arm in [[14usize, 15, 16], [18, 19, 20]] {
        add(&arm[1..], &[0, 7, 8, 9, 10]);
        add(&arm[..1], &[0, 7, 8]);
        add(&arm, &[1, 2, 4, 5, 12]);
    }
    add(&[15, 16], &[19, 20]);
    add(&[1, 2, 3], &[4, 5, 6]);
    add(&[1, 4], &[8, 9, 10]);
    add(&[2, 3, 5, 6], &[7, 8, 9, 10, 12]);
    add(&[1, 4], &[12]);

    // the character: asset coordinates flip x and y, and the zero pose faces the other way to the bind pose
    // (Delirium's convention), which cancel: the asset holds the zero pose in authoring coordinates
    let mrows = |q: Quat, c: Vec3| {
        let m = Mat3::from_quat(q);
        [m.row(0).to_array(), m.row(1).to_array(), m.row(2).to_array(), c.to_array()]
    };
    let kg = |k: usize| opt.mass_pct[k] * opt.body_kg / 100.0;
    let mut joints = Vec::with_capacity(20);
    for (i, name) in JOINTS.iter().enumerate() {
        let k = i + 1;
        let (z, x0) = axes(k);
        let (g, max) = muscle(i);
        let r = RIGHT.contains(&k) && !hinge(k);
        joints.push(Joint {
            name: (*name).into(),
            index: u32::try_from(i).expect("20 joints"),
            three: !hinge(k),
            pos: zp[k].to_array(),
            twist_axis: z.to_array(),
            lean_axis: x0.to_array(),
            limits: limits(k)?,
            flip: [false, r, r],
            muscle_stiffness: 1.0,
            damping: 2.0 * g,
            strength: g * g,
            max_torque: max.map(|m| m * opt.torque),
        });
    }
    let parts: Vec<Part> = (0..21).map(|k| Part { name: NAMES[k].into(), frame: mrows(zrot[k], zc[k]), mass: kg(k) }).collect();
    let fr = |p: Vec3, q: Quat| Frame { pos: p.to_array(), rot: q.to_array() };
    let build: Vec<BuildPart> = (0..21)
        .map(|k| {
            let (radius, half, boxed) = size[k];
            let (anchor, anchor_parent) = match parent(k) {
                None => (rot[0].inverse() * ((pv[1] + pv[4]) * 0.5 - centre[0]), Vec3::ZERO),
                Some(p) => (rot[k].inverse() * (pv[k] - centre[k]), rot[p].inverse() * (pv[k] - centre[p])),
            };
            BuildPart {
                name: NAMES[k].into(),
                parent: parent(k),
                mass: kg(k),
                radius,
                half,
                offset: fr(Vec3::ZERO, Quat::IDENTITY),
                rest: Quat::IDENTITY.to_array(),
                anchor_parent: anchor_parent.to_array(),
                anchor: anchor.to_array(),
                limit: JointLimit::Cone { swing: 1.0, twist: 0.5 },
                lower: k <= 6,
                reach: matches!(k, 14 | 15 | 18 | 19),
                bind: rot[k].to_array(),
                boxed: boxed.map(|e| e.to_array()),
                inertia: [0.0; 3],
                frame: fr(centre[k], rot[k]),
            }
        })
        .collect();
    let character = Character { format: FORMAT.into(), joints, parts, self_collision: pairs, build: Some(build) };

    // the bone table: each part's bone, and the bone's bind frame in the part's bind frame. The chest bone
    // (`Bip01 Spine2`) carries the clavicles and the neck, like Delirium's Spine3: Spine3 writes it, Spine2 only reads it
    let mut table = Vec::with_capacity(21);
    for k in 0..21 {
        let (j, write) = match k {
            9 => (upper_chest, false),
            10 => (upper_chest, true),
            _ => (joint(BONE[k].expect("bone"))?, true),
        };
        let part = Transform::from_translation(centre[k]).with_rotation(rot[k]);
        let bone = Transform::from_translation(bone_pos(j)).with_rotation(bone_rot(j));
        let off = part.compute_affine().inverse() * bone.compute_affine();
        let (_, r, t) = off.to_scale_rotation_translation();
        table.push(PartBone { joint: j, write, offset: Transform::from_translation(t).with_rotation(r.normalize()) });
    }
    let tf = |c: &[Vec3], r: &[Quat]| (0..21).map(|k| Transform::from_translation(c[k]).with_rotation(r[k])).collect();
    Ok(Body {
        character,
        table: PartTable { parts: table },
        bind: tf(&centre, &rot),
        zero: tf(&zc, &zrot),
        bind_bends: [knee_bent, elbow_bent, hip_out],
    })
}
