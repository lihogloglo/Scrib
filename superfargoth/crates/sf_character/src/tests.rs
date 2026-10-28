use glam::{Quat, Vec3};
use sf_anim::{AnimImporter, GltfAnimImporter, Retargeter, Skeleton, SourceJoint, Trs};
use sf_records::{RecordDb, RecordId};
use sf_vfs::VfsPath;

use crate::rig::model_of;
use crate::*;
use HumanoidBone as B;

// ---------------------------------------------------------------------------
// Synthetic rigs: joint names of common conventions over one body layout.

/// Model-space positions of a 1.8 m body, T-pose, facing +Z, left = +X.
fn body(key: &str, side: f32) -> Vec3 {
    let x = |v: f32| v * side;
    match key {
        "hips" => Vec3::new(0.0, 1.0, 0.0),
        "neck" => Vec3::new(0.0, 1.5, 0.0),
        "neck2" => Vec3::new(0.0, 1.56, 0.0),
        "head" => Vec3::new(0.0, 1.62, 0.0),
        "head_end" => Vec3::new(0.0, 1.8, 0.0),
        "eye" => Vec3::new(x(0.03), 1.68, 0.08),
        "jaw" => Vec3::new(0.0, 1.6, 0.05),
        "clavicle" => Vec3::new(x(0.03), 1.45, 0.03),
        "upper_arm" => Vec3::new(x(0.18), 1.45, 0.0),
        "lower_arm" => Vec3::new(x(0.46), 1.45, 0.0),
        "hand" => Vec3::new(x(0.72), 1.45, 0.0),
        "thigh" => Vec3::new(x(0.1), 0.95, 0.0),
        "calf" => Vec3::new(x(0.1), 0.5, 0.0),
        "foot" => Vec3::new(x(0.1), 0.08, 0.0),
        "toe" => Vec3::new(x(0.1), 0.02, 0.12),
        "toe_end" => Vec3::new(x(0.1), 0.02, 0.18),
        "tail" => Vec3::new(0.0, 0.95, -0.1),
        k if k.starts_with("spine") => {
            let t: f32 = k[5..].parse().expect("spine<fraction>");
            Vec3::new(0.0, 1.0 + 0.5 * t, 0.0)
        }
        k if k.starts_with("finger") => {
            // finger<f><segment>: f 0 = thumb .. 4; segment 0..=3 (3 = end).
            let f = f32::from(k.as_bytes()[6] - b'0');
            let s = f32::from(k.as_bytes()[7] - b'0');
            if f == 0.0 {
                Vec3::new(x(0.74 + 0.025 * s), 1.44 - 0.005 * s, 0.03 + 0.02 * s)
            } else {
                Vec3::new(x(0.8 + 0.03 * s), 1.45, 0.04 - 0.02 * f)
            }
        }
        _ => panic!("no body position {key}"),
    }
}

struct RigSpec {
    joints: Vec<(String, Option<String>, Vec3)>,
}

impl RigSpec {
    fn new() -> Self {
        Self { joints: Vec::new() }
    }

    fn add(&mut self, name: &str, parent: Option<&str>, pos: Vec3) -> &mut Self {
        self.joints.push((name.to_owned(), parent.map(str::to_owned), pos));
        self
    }

    /// Builds a skeleton whose joints have scrambled local axes (as
    /// different tools export them): model positions are kept, model
    /// rotations are pseudo-random.
    fn build(&self, a_pose: bool, face_neg_z: bool) -> Skeleton {
        let mut seed = 0x2545_f491_u32;
        let mut rnd = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f32::from(u16::try_from(seed >> 16).expect("16 bits")) / 65535.0
        };
        let names: Vec<&str> = self.joints.iter().map(|j| j.0.as_str()).collect();
        let index = |n: &str| names.iter().position(|x| *x == n).unwrap_or_else(|| panic!("no joint {n}"));
        let mut pos: Vec<Vec3> = self.joints.iter().map(|j| j.2).collect();
        if a_pose {
            // Arms 45° down: rotate everything from the upper arm outward about the shoulder.
            for (i, (name, _, _)) in self.joints.iter().enumerate() {
                let lower = name.to_ascii_lowercase();
                let arm = ["upperarm", "upper_arm", "arm", "forearm", "hand", "finger", "thumb", "index", "middle", "ring", "pinky"]
                    .iter()
                    .any(|w| lower.replace(' ', "").contains(w))
                    && !lower.contains("shoulder")
                    && !lower.contains("clavicle");
                if arm {
                    let side = pos[i].x.signum();
                    let pivot = body("upper_arm", side);
                    pos[i] = pivot + Quat::from_rotation_z(-side * std::f32::consts::FRAC_PI_4) * (pos[i] - pivot);
                }
            }
        }
        if face_neg_z {
            for p in &mut pos {
                *p = Quat::from_rotation_y(std::f32::consts::PI) * *p;
            }
        }
        let model_rot: Vec<Quat> = (0..pos.len())
            .map(|_| {
                let axis = Vec3::new(rnd() - 0.5, rnd() - 0.5, rnd() - 0.5).normalize_or(Vec3::Y);
                Quat::from_axis_angle(axis, rnd() * std::f32::consts::TAU)
            })
            .collect();
        let joints: Vec<SourceJoint> = self
            .joints
            .iter()
            .enumerate()
            .map(|(i, (name, parent, _))| {
                let p = parent.as_deref().map(index);
                let (pr, pp) = p.map_or((Quat::IDENTITY, Vec3::ZERO), |p| (model_rot[p], pos[p]));
                let rest = Trs::new(pr.inverse() * (pos[i] - pp), (pr.inverse() * model_rot[i]).normalize(), Vec3::ONE);
                SourceJoint { name: name.clone(), parent: p, rest }
            })
            .collect();
        Skeleton::new(&joints).expect("valid skeleton").0
    }
}

/// 3ds Max Biped naming: thighs under the first spine joint, clavicles
/// under the neck, a non-accumulating node above the pelvis, a tail, a
/// weapon bone and forearm twist helpers.
fn biped() -> RigSpec {
    let mut r = RigSpec::new();
    r.add("Bip01", None, Vec3::ZERO)
        .add("Bip01 NonAccum", Some("Bip01"), Vec3::new(0.0, 1.0, 0.0))
        .add("Bip01 Pelvis", Some("Bip01 NonAccum"), body("hips", 0.0))
        .add("Bip01 Spine", Some("Bip01 Pelvis"), body("spine0.1", 0.0))
        .add("Bip01 Spine1", Some("Bip01 Spine"), body("spine0.4", 0.0))
        .add("Bip01 Spine2", Some("Bip01 Spine1"), body("spine0.7", 0.0))
        .add("Bip01 Neck", Some("Bip01 Spine2"), body("neck", 0.0))
        .add("Bip01 Head", Some("Bip01 Neck"), body("head", 0.0))
        .add("Bip01 HeadNub", Some("Bip01 Head"), body("head_end", 0.0))
        .add("Bip01 Tail", Some("Bip01 Pelvis"), body("tail", 0.0))
        .add("Bip01 Tail1", Some("Bip01 Tail"), body("tail", 0.0) + Vec3::new(0.0, -0.2, -0.1))
        .add("Bip01 Tail2", Some("Bip01 Tail1"), body("tail", 0.0) + Vec3::new(0.0, -0.4, -0.2));
    for (s, side) in [("L", 1.0), ("R", -1.0)] {
        let n = |x: &str| format!("Bip01 {s} {x}");
        r.add(&n("Thigh"), Some("Bip01 Spine"), body("thigh", side))
            .add(&n("Calf"), Some(&n("Thigh")), body("calf", side))
            .add(&n("Foot"), Some(&n("Calf")), body("foot", side))
            .add(&n("Toe0"), Some(&n("Foot")), body("toe", side))
            .add(&n("Toe0Nub"), Some(&n("Toe0")), body("toe_end", side))
            .add(&n("Clavicle"), Some("Bip01 Neck"), body("clavicle", side))
            .add(&n("UpperArm"), Some(&n("Clavicle")), body("upper_arm", side))
            .add(&n("Forearm"), Some(&n("UpperArm")), body("lower_arm", side))
            .add(&n("ForeTwist"), Some(&n("Forearm")), body("lower_arm", side).lerp(body("hand", side), 0.5))
            .add(&n("Hand"), Some(&n("Forearm")), body("hand", side));
        for f in 0..5 {
            let mut parent = n("Hand");
            for k in 0..4 {
                let name = match k {
                    0 => n(&format!("Finger{f}")),
                    3 => n(&format!("Finger{f}Nub")),
                    _ => n(&format!("Finger{f}{k}")),
                };
                r.add(&name, Some(&parent), body(&format!("finger{f}{k}"), side));
                parent = name;
            }
        }
    }
    let rh = "Bip01 R Hand";
    r.add("Weapon Bone", Some(rh), body("hand", -1.0) + Vec3::new(-0.05, 0.0, 0.0));
    r
}

/// Mixamo naming: `mixamorig:` prefix, `LeftUpLeg`/`LeftLeg`, four finger
/// joints (the fourth is an end site without an end name).
fn mixamo() -> RigSpec {
    let mut r = RigSpec::new();
    let m = |x: &str| format!("mixamorig:{x}");
    r.add(&m("Hips"), None, body("hips", 0.0))
        .add(&m("Spine"), Some(&m("Hips")), body("spine0.2", 0.0))
        .add(&m("Spine1"), Some(&m("Spine")), body("spine0.5", 0.0))
        .add(&m("Spine2"), Some(&m("Spine1")), body("spine0.8", 0.0))
        .add(&m("Neck"), Some(&m("Spine2")), body("neck", 0.0))
        .add(&m("Head"), Some(&m("Neck")), body("head", 0.0))
        .add(&m("HeadTop_End"), Some(&m("Head")), body("head_end", 0.0))
        .add(&m("LeftEye"), Some(&m("Head")), body("eye", 1.0))
        .add(&m("RightEye"), Some(&m("Head")), body("eye", -1.0));
    for (s, side) in [("Left", 1.0), ("Right", -1.0)] {
        let n = |x: &str| m(&format!("{s}{x}"));
        r.add(&n("UpLeg"), Some(&m("Hips")), body("thigh", side))
            .add(&n("Leg"), Some(&n("UpLeg")), body("calf", side))
            .add(&n("Foot"), Some(&n("Leg")), body("foot", side))
            .add(&n("ToeBase"), Some(&n("Foot")), body("toe", side))
            .add(&n("Toe_End"), Some(&n("ToeBase")), body("toe_end", side))
            .add(&n("Shoulder"), Some(&m("Spine2")), body("clavicle", side))
            .add(&n("Arm"), Some(&n("Shoulder")), body("upper_arm", side))
            .add(&n("ForeArm"), Some(&n("Arm")), body("lower_arm", side))
            .add(&n("Hand"), Some(&n("ForeArm")), body("hand", side));
        for (f, word) in ["Thumb", "Index", "Middle", "Ring", "Pinky"].iter().enumerate() {
            let mut parent = n("Hand");
            for k in 0..4 {
                let name = n(&format!("Hand{word}{}", k + 1));
                r.add(&name, Some(&parent), body(&format!("finger{f}{k}"), side));
                parent = name;
            }
        }
    }
    r
}

/// UE5 mannequin naming: five spine joints, two neck joints, twist joints,
/// finger metacarpals, and IK helper joints under the root.
fn ue5() -> RigSpec {
    let mut r = RigSpec::new();
    r.add("root", None, Vec3::ZERO).add("pelvis", Some("root"), body("hips", 0.0));
    let mut parent = "pelvis".to_owned();
    for k in 1..=5 {
        let name = format!("spine_0{k}");
        #[allow(clippy::cast_precision_loss)]
        r.add(&name, Some(&parent), body(&format!("spine{}", 0.15 * k as f32), 0.0));
        parent = name;
    }
    r.add("neck_01", Some("spine_05"), body("neck", 0.0))
        .add("neck_02", Some("neck_01"), body("neck2", 0.0))
        .add("head", Some("neck_02"), body("head", 0.0))
        .add("ik_foot_root", Some("root"), Vec3::ZERO)
        .add("ik_hand_root", Some("root"), Vec3::ZERO);
    for (s, side) in [("l", 1.0), ("r", -1.0)] {
        let n = |x: &str| format!("{x}_{s}");
        r.add(&n("thigh"), Some("pelvis"), body("thigh", side))
            .add(&n("thigh_twist_01"), Some(&n("thigh")), body("thigh", side).lerp(body("calf", side), 0.5))
            .add(&n("calf"), Some(&n("thigh")), body("calf", side))
            .add(&n("foot"), Some(&n("calf")), body("foot", side))
            .add(&n("ball"), Some(&n("foot")), body("toe", side))
            .add(&n("clavicle"), Some("spine_05"), body("clavicle", side))
            .add(&n("upperarm"), Some(&n("clavicle")), body("upper_arm", side))
            .add(&n("upperarm_twist_01"), Some(&n("upperarm")), body("upper_arm", side).lerp(body("lower_arm", side), 0.5))
            .add(&n("lowerarm"), Some(&n("upperarm")), body("lower_arm", side))
            .add(&n("lowerarm_twist_01"), Some(&n("lowerarm")), body("lower_arm", side).lerp(body("hand", side), 0.5))
            .add(&n("hand"), Some(&n("lowerarm")), body("hand", side))
            .add(&n("ik_foot"), Some("ik_foot_root"), body("foot", side))
            .add(&n("ik_hand"), Some("ik_hand_root"), body("hand", side));
        for (f, word) in ["thumb", "index", "middle", "ring", "pinky"].iter().enumerate() {
            let mut parent = n("hand");
            if f > 0 {
                let meta = n(&format!("{word}_metacarpal"));
                r.add(&meta, Some(&parent), body("hand", side).lerp(body(&format!("finger{f}0"), side), 0.5));
                parent = meta;
            }
            for k in 0..3 {
                let name = n(&format!("{word}_0{}", k + 1));
                r.add(&name, Some(&parent), body(&format!("finger{f}{k}"), side));
                parent = name;
            }
        }
    }
    r
}

/// VRM / Unity humanoid naming.
fn vrm_names() -> RigSpec {
    let mut r = RigSpec::new();
    r.add("Hips", None, body("hips", 0.0))
        .add("Spine", Some("Hips"), body("spine0.2", 0.0))
        .add("Chest", Some("Spine"), body("spine0.5", 0.0))
        .add("UpperChest", Some("Chest"), body("spine0.8", 0.0))
        .add("Neck", Some("UpperChest"), body("neck", 0.0))
        .add("Head", Some("Neck"), body("head", 0.0))
        .add("Jaw", Some("Head"), body("jaw", 0.0));
    for (s, side) in [("Left", 1.0), ("Right", -1.0)] {
        let n = |x: &str| format!("{s}{x}");
        r.add(&n("Eye"), Some("Head"), body("eye", side))
            .add(&n("UpperLeg"), Some("Hips"), body("thigh", side))
            .add(&n("LowerLeg"), Some(&n("UpperLeg")), body("calf", side))
            .add(&n("Foot"), Some(&n("LowerLeg")), body("foot", side))
            .add(&n("Toes"), Some(&n("Foot")), body("toe", side))
            .add(&n("Shoulder"), Some("UpperChest"), body("clavicle", side))
            .add(&n("UpperArm"), Some(&n("Shoulder")), body("upper_arm", side))
            .add(&n("LowerArm"), Some(&n("UpperArm")), body("lower_arm", side))
            .add(&n("Hand"), Some(&n("LowerArm")), body("hand", side));
        for (f, word) in ["Thumb", "Index", "Middle", "Ring", "Little"].iter().enumerate() {
            let segs = if f == 0 { ["Metacarpal", "Proximal", "Distal"] } else { ["Proximal", "Intermediate", "Distal"] };
            let mut parent = n("Hand");
            for (k, seg) in segs.iter().enumerate() {
                let name = n(&format!("{word}{seg}"));
                r.add(&name, Some(&parent), body(&format!("finger{f}{k}"), side));
                parent = name;
            }
        }
    }
    r
}

/// NVIDIA SOMA-77 naming (Kimodo and `MotionBricks` BVH): a static `Root` wrapper
/// above the hips, `Spine1`, `Spine2`, `Chest`, two neck joints, `LeftLeg` =
/// the thigh and `LeftShin` the lower leg, four-joint fingers whose first
/// joint is the metacarpal, and `*End` leaf joints (Kimodo writes no End Site).
fn soma77() -> RigSpec {
    let mut r = RigSpec::new();
    r.add("Root", None, Vec3::ZERO)
        .add("Hips", Some("Root"), body("hips", 0.0))
        .add("Spine1", Some("Hips"), body("spine0.2", 0.0))
        .add("Spine2", Some("Spine1"), body("spine0.5", 0.0))
        .add("Chest", Some("Spine2"), body("spine0.8", 0.0))
        .add("Neck1", Some("Chest"), body("neck", 0.0))
        .add("Neck2", Some("Neck1"), body("neck2", 0.0))
        .add("Head", Some("Neck2"), body("head", 0.0))
        .add("HeadEnd", Some("Head"), body("head_end", 0.0))
        .add("Jaw", Some("Head"), body("jaw", 0.0));
    for (s, side) in [("Left", 1.0), ("Right", -1.0)] {
        let n = |x: &str| format!("{s}{x}");
        r.add(&n("Eye"), Some("Head"), body("eye", side))
            .add(&n("Shoulder"), Some("Chest"), body("clavicle", side))
            .add(&n("Arm"), Some(&n("Shoulder")), body("upper_arm", side))
            .add(&n("ForeArm"), Some(&n("Arm")), body("lower_arm", side))
            .add(&n("Hand"), Some(&n("ForeArm")), body("hand", side));
        for (f, word) in ["Thumb", "Index", "Middle", "Ring", "Pinky"].iter().enumerate() {
            let segs: &[&str] = if f == 0 { &["1", "2", "3", "End"] } else { &["1", "2", "3", "4", "End"] };
            let mut parent = n("Hand");
            for (k, seg) in segs.iter().enumerate() {
                let name = n(&format!("Hand{word}{seg}"));
                r.add(&name, Some(&parent), body(&format!("finger{f}{k}"), side));
                parent = name;
            }
        }
        r.add(&n("Leg"), Some("Hips"), body("thigh", side))
            .add(&n("Shin"), Some(&n("Leg")), body("calf", side))
            .add(&n("Foot"), Some(&n("Shin")), body("foot", side))
            .add(&n("ToeBase"), Some(&n("Foot")), body("toe", side))
            .add(&n("ToeEnd"), Some(&n("ToeBase")), body("toe_end", side));
    }
    r
}

fn assert_maps(map: &RigMap, expected: &[(HumanoidBone, &str)]) {
    for (bone, joint) in expected {
        assert_eq!(map.joint(*bone), Some(*joint), "{bone:?}");
    }
}

fn finger_count(map: &RigMap) -> usize {
    map.bones.iter().filter(|(_, b)| b.finger_of().is_some()).count()
}

// ---------------------------------------------------------------------------

#[test]
fn humanoid_table_matches_vrm_1_0() {
    assert_eq!(HumanoidBone::ALL.len(), 55);
    assert_eq!(HumanoidBone::ALL.iter().filter(|b| b.required()).count(), 15);
    for (i, b) in HumanoidBone::ALL.into_iter().enumerate() {
        assert_eq!(b.index(), i);
        assert_eq!(HumanoidBone::from_name(b.name()), Some(b));
        assert_eq!(b.mirror().mirror(), b);
        assert_eq!(b.mirror().side(), b.side().mirror());
        assert_eq!(
            b.mirror().name().trim_start_matches("left").trim_start_matches("right"),
            b.name().trim_start_matches("left").trim_start_matches("right")
        );
        if let Some(p) = b.parent() {
            assert!(p.index() < i, "{b:?} after its parent");
        }
    }
    assert_eq!(HumanoidBone::finger(Side::Right, Finger::Thumb, 0), Some(HumanoidBone::RightThumbMetacarpal));
    assert_eq!(HumanoidBone::LeftRingDistal.finger_of(), Some((Side::Left, Finger::Ring, 2)));
}

#[test]
fn auto_maps_biped_naming() {
    let skeleton = biped().build(true, true);
    let (map, report) = auto_map(&skeleton);
    assert_maps(
        &map,
        &[
            (B::Hips, "Bip01 Pelvis"),
            (B::Spine, "Bip01 Spine"),
            (B::Chest, "Bip01 Spine1"),
            (B::UpperChest, "Bip01 Spine2"),
            (B::Neck, "Bip01 Neck"),
            (B::Head, "Bip01 Head"),
            (B::LeftShoulder, "Bip01 L Clavicle"),
            (B::LeftUpperArm, "Bip01 L UpperArm"),
            (B::LeftLowerArm, "Bip01 L Forearm"),
            (B::LeftHand, "Bip01 L Hand"),
            (B::RightUpperLeg, "Bip01 R Thigh"),
            (B::RightLowerLeg, "Bip01 R Calf"),
            (B::RightFoot, "Bip01 R Foot"),
            (B::RightToes, "Bip01 R Toe0"),
            (B::LeftThumbMetacarpal, "Bip01 L Finger0"),
            (B::LeftThumbDistal, "Bip01 L Finger02"),
            (B::RightLittleProximal, "Bip01 R Finger4"),
            (B::RightLittleDistal, "Bip01 R Finger42"),
        ],
    );
    assert_eq!(finger_count(&map), 30);
    assert!(report.is_clean(), "{report:#?}");
    assert_eq!(map.root_motion, "Bip01 Pelvis", "no joint is named root: the hips carry root motion unless the adapter says otherwise");
    assert_eq!(map.extra_chains, vec![Chain { name: "tail".into(), joints: vec!["Bip01 Tail".into(), "Bip01 Tail1".into(), "Bip01 Tail2".into()] }]);
    assert_eq!(report.unmapped, vec!["Bip01 Tail", "Bip01 Tail1", "Bip01 Tail2"]);
    assert!(map.mirror.contains(&("Bip01 L ForeTwist".into(), "Bip01 R ForeTwist".into())));
    assert!(map.mirror.contains(&("Bip01 L Finger21".into(), "Bip01 R Finger21".into())));
    assert!(map.validate(&skeleton).is_empty());
}

#[test]
fn auto_maps_mixamo_naming() {
    let skeleton = mixamo().build(false, false);
    let (map, report) = auto_map(&skeleton);
    assert_maps(
        &map,
        &[
            (B::Hips, "mixamorig:Hips"),
            (B::UpperChest, "mixamorig:Spine2"),
            (B::LeftUpperLeg, "mixamorig:LeftUpLeg"),
            (B::LeftLowerLeg, "mixamorig:LeftLeg"),
            (B::LeftToes, "mixamorig:LeftToeBase"),
            (B::RightShoulder, "mixamorig:RightShoulder"),
            (B::RightUpperArm, "mixamorig:RightArm"),
            (B::RightLowerArm, "mixamorig:RightForeArm"),
            (B::LeftEye, "mixamorig:LeftEye"),
            (B::LeftThumbMetacarpal, "mixamorig:LeftHandThumb1"),
            (B::LeftIndexDistal, "mixamorig:LeftHandIndex3"),
        ],
    );
    assert_eq!(finger_count(&map), 30);
    assert!(report.is_clean(), "{report:#?}");
}

#[test]
fn auto_maps_ue5_naming() {
    let skeleton = ue5().build(true, false);
    let (map, report) = auto_map(&skeleton);
    assert_maps(
        &map,
        &[
            (B::Hips, "pelvis"),
            (B::Spine, "spine_01"),
            (B::Chest, "spine_03"),
            (B::UpperChest, "spine_05"),
            (B::Neck, "neck_01"),
            (B::Head, "head"),
            (B::LeftUpperLeg, "thigh_l"),
            (B::LeftLowerLeg, "calf_l"),
            (B::LeftToes, "ball_l"),
            (B::LeftUpperArm, "upperarm_l"),
            (B::LeftLowerArm, "lowerarm_l"),
            (B::LeftHand, "hand_l"),
            (B::LeftIndexProximal, "index_01_l"),
            (B::LeftThumbMetacarpal, "thumb_01_l"),
        ],
    );
    assert_eq!(finger_count(&map), 30);
    assert_eq!(map.root_motion, "root");
    assert!(report.is_clean(), "{report:#?}");
    assert!(map.extra_chains.is_empty(), "{:?}", map.extra_chains);
    // The spine chain spans all five spine joints; the neck chain both neck joints.
    let rig = map.retarget_rig(&skeleton, RetargetOptions::default()).unwrap();
    let chain = |n: &str| rig.chains.iter().find(|c| c.name == n).map(|c| c.joints.len());
    assert_eq!((chain("spine"), chain("neck"), chain("left_leg")), (Some(5), Some(2), Some(3)));
    assert!(rig.chains.iter().find(|c| c.name == "left_leg").unwrap().ik);
}

#[test]
fn auto_maps_vrm_naming_completely() {
    let skeleton = vrm_names().build(false, false);
    let (map, report) = auto_map(&skeleton);
    assert!(report.is_clean(), "{report:#?}");
    assert_eq!(map.bones.len(), 55, "every VRM bone");
    for (joint, bone) in &map.bones {
        assert!(bone.name().eq_ignore_ascii_case(joint), "{joint} → {bone:?}");
    }
}

#[test]
fn auto_maps_soma77_naming() {
    let skeleton = soma77().build(false, false);
    let (mut map, report) = auto_map(&skeleton);
    assert_maps(
        &map,
        &[
            (B::Hips, "Hips"),
            (B::Spine, "Spine1"),
            (B::Chest, "Spine2"),
            (B::UpperChest, "Chest"),
            (B::Neck, "Neck1"),
            (B::Head, "Head"),
            (B::Jaw, "Jaw"),
            (B::LeftEye, "LeftEye"),
            (B::LeftUpperLeg, "LeftLeg"),
            (B::LeftLowerLeg, "LeftShin"),
            (B::RightUpperLeg, "RightLeg"),
            (B::RightLowerLeg, "RightShin"),
            (B::LeftFoot, "LeftFoot"),
            (B::LeftToes, "LeftToeBase"),
            (B::LeftShoulder, "LeftShoulder"),
            (B::LeftUpperArm, "LeftArm"),
            (B::LeftLowerArm, "LeftForeArm"),
            (B::RightHand, "RightHand"),
            (B::LeftThumbMetacarpal, "LeftHandThumb1"),
            (B::LeftThumbDistal, "LeftHandThumb3"),
            (B::LeftIndexProximal, "LeftHandIndex2"),
            (B::LeftIndexDistal, "LeftHandIndex4"),
            (B::RightLittleProximal, "RightHandPinky2"),
        ],
    );
    assert_eq!(finger_count(&map), 30);
    assert_eq!(map.root_motion, "Root");
    assert!(report.is_clean(), "{report:#?}");
    assert!(map.extra_chains.is_empty(), "{:?}", map.extra_chains);
    map.auto_align(&skeleton);
    assert_t_pose(&skeleton, &map);
}

#[test]
fn reports_what_it_cannot_map() {
    let mut r = RigSpec::new();
    r.add("Hips", None, body("hips", 0.0)).add("Thing", Some("Hips"), body("spine0.5", 0.0));
    let skeleton = r.build(false, false);
    let (map, report) = auto_map(&skeleton);
    assert!(!report.is_clean());
    assert!(report.missing_required.contains(&HumanoidBone::Head));
    assert!(report.errors.contains(&RigMapError::MissingRequired("leftHand")));
    assert_eq!(map.joint(HumanoidBone::Hips), Some("Hips"));
}

#[test]
fn validate_catches_bad_maps() {
    let skeleton = vrm_names().build(false, false);
    let (mut map, _) = auto_map(&skeleton);
    map.bones.retain(|(_, b)| *b != HumanoidBone::Chest);
    map.bones.push(("LeftHand".into(), HumanoidBone::Jaw));
    map.bones.push(("Nope".into(), HumanoidBone::LeftEye));
    let errors = map.validate(&skeleton);
    assert!(errors.contains(&RigMapError::UpperChestWithoutChest), "{errors:?}");
    assert!(errors.contains(&RigMapError::DuplicateJoint("LeftHand".into())));
    assert!(errors.contains(&RigMapError::DuplicateBone("leftEye")));
    assert!(errors.contains(&RigMapError::UnknownJoint("Nope".into())));
    assert!(errors.iter().any(|e| matches!(e, RigMapError::Hierarchy { bone: "jaw", .. })), "{errors:?}");
}

/// Model-space direction from `a` to `b` in the map's retarget pose.
fn dir(skeleton: &Skeleton, map: &RigMap, a: HumanoidBone, b: HumanoidBone) -> Vec3 {
    let model = map.retarget_pose_model(skeleton);
    let p = |x: HumanoidBone| Vec3::from(model[skeleton.find(map.joint(x).unwrap()).unwrap()].translation);
    (p(b) - p(a)).normalize()
}

fn assert_t_pose(skeleton: &Skeleton, map: &RigMap) {
    let checks = [
        (B::LeftUpperArm, B::LeftLowerArm, Vec3::X),
        (B::LeftLowerArm, B::LeftHand, Vec3::X),
        (B::RightUpperArm, B::RightLowerArm, Vec3::NEG_X),
        (B::RightLowerArm, B::RightHand, Vec3::NEG_X),
        (B::LeftUpperLeg, B::LeftLowerLeg, Vec3::NEG_Y),
        (B::RightLowerLeg, B::RightFoot, Vec3::NEG_Y),
        (B::Hips, B::Spine, Vec3::Y),
        (B::Spine, B::Chest, Vec3::Y),
        (B::Neck, B::Head, Vec3::Y),
    ];
    for (a, b, want) in checks {
        let got = dir(skeleton, map, a, b);
        assert!(got.angle_between(want).to_degrees() < 0.05, "{a:?}→{b:?}: {got}");
    }
    // Facing +Z: the toes are in front of the feet, the left leg on +X.
    assert!(dir(skeleton, map, B::LeftFoot, B::LeftToes).z > 0.5);
    assert!(dir(skeleton, map, B::RightUpperLeg, B::LeftUpperLeg).x > 0.99);
}

#[test]
fn auto_align_points_a_collinear_foot_forward() {
    // A mocap zero pose's leg (LaFAN1-style BVH): the toes continue the shin's
    // line, straight down or folded back up it. The foot's rest direction is
    // then meaningless, so it points forward (+Z, flat) after the align.
    for sign in [1.0, -1.0] {
        let mut spec = vrm_names();
        let at = |spec: &RigSpec, n: &str| spec.joints.iter().find(|j| j.0 == n).unwrap().2;
        for s in ["Left", "Right"] {
            let (knee, ankle) = (at(&spec, &format!("{s}LowerLeg")), at(&spec, &format!("{s}Foot")));
            let toe = ankle + (ankle - knee).normalize() * 0.12 * sign;
            spec.joints.iter_mut().find(|j| j.0 == format!("{s}Toes")).unwrap().2 = toe;
        }
        let skeleton = spec.build(false, false);
        let (mut map, _) = auto_map(&skeleton);
        let report = map.auto_align(&skeleton);
        assert!(report.max_residual_deg < 0.05, "{report:?}");
        for (foot, toes) in [(B::LeftFoot, B::LeftToes), (B::RightFoot, B::RightToes)] {
            let d = dir(&skeleton, &map, foot, toes);
            assert!(d.angle_between(Vec3::Z).to_degrees() < 0.05, "{foot:?} → toes {d} (sign {sign})");
        }
        assert_t_pose(&skeleton, &map);
    }
}

#[test]
fn auto_align_reaches_the_canonical_t_pose() {
    // A-pose, facing −Z, scrambled joint axes.
    let skeleton = biped().build(true, true);
    let (mut map, _) = auto_map(&skeleton);
    let report = map.auto_align(&skeleton);
    assert!((report.facing_correction_deg - 180.0).abs() < 0.01, "{report:?}");
    assert!(report.max_residual_deg < 0.05, "{report:?}");
    assert!(map.retarget_pose.iter().any(|(j, _)| j == "Bip01"), "the facing turn sits on the topmost joint");
    assert_t_pose(&skeleton, &map);

    for (spec, a_pose) in [(mixamo(), true), (ue5(), true), (vrm_names(), false), (soma77(), false)] {
        let skeleton = spec.build(a_pose, false);
        let (mut map, _) = auto_map(&skeleton);
        let report = map.auto_align(&skeleton);
        assert!(report.facing_correction_deg < 0.01);
        assert_t_pose(&skeleton, &map);
    }
}

/// The universal Quaternius rig fixture as a skeleton.
fn quaternius_universal() -> Skeleton {
    let bytes = include_bytes!("../tests/data/quaternius_universal_rig.gltf");
    let src = GltfAnimImporter.import("quaternius_universal_rig.gltf", bytes).expect("fixture imports");
    Skeleton::new(&src.skeleton.expect("skin joints")).expect("valid").0
}

#[test]
#[allow(clippy::many_single_char_names)]
fn retarget_round_trip_between_quaternius_and_a_biped_rig() {
    let s = quaternius_universal();
    let t = biped().build(true, true);
    let (mut sm, sr) = auto_map(&s);
    let (mut tm, tr) = auto_map(&t);
    assert!(sr.is_clean() && tr.is_clean(), "{sr:#?} {tr:#?}");
    sm.auto_align(&s);
    tm.auto_align(&t);
    let options = RetargetOptions { ik_legs: false };
    let (s_rig, t_rig) = (sm.retarget_rig(&s, options).unwrap(), tm.retarget_rig(&t, options).unwrap());
    let fwd = Retargeter::new(&s, &s_rig, &t, &t_rig).unwrap();
    let back = Retargeter::new(&t, &t_rig, &s, &s_rig).unwrap();

    // A source pose: the retarget pose with every mapped joint bent, the hips moved.
    let mut pose = sm.retarget_pose_local(&s);
    for (k, (name, _)) in sm.bones.iter().enumerate() {
        let j = s.find(name).unwrap();
        #[allow(clippy::cast_precision_loss)]
        let a = 0.3 + 0.05 * k as f32;
        pose[j].rotation = (pose[j].rotation * Quat::from_euler(glam::EulerRot::XYZ, 0.4 * a.sin(), 0.3 * a.cos(), 0.2)).normalize();
    }
    let hips = s.find(sm.joint(HumanoidBone::Hips).unwrap()).unwrap();
    pose[hips].translation += Vec3::new(0.05, -0.1, 0.2);

    let mut mid = vec![Trs::IDENTITY; t.len()];
    let mut out = vec![Trs::IDENTITY; s.len()];
    fwd.pose(&pose, &[], &mut mid);
    back.pose(&mid, &[], &mut out);
    let (a, b) = (model_of(&s, &pose), model_of(&s, &out));
    let rot = |m: &glam::Affine3A| m.to_scale_rotation_translation().1;
    let mut worst = 0.0f32;
    for (name, bone) in &sm.bones {
        if tm.joint(*bone).is_none() {
            continue;
        }
        let j = s.find(name).unwrap();
        worst = worst.max(rot(&a[j]).angle_between(rot(&b[j])).to_degrees());
    }
    assert!(worst < 1.0, "worst mapped bone {worst}°");
    let drift = Vec3::from(a[hips].translation).distance(Vec3::from(b[hips].translation));
    assert!(drift < 0.01, "hips drift {drift} m");
    for bone in [HumanoidBone::LeftHand, HumanoidBone::RightFoot] {
        let j = s.find(sm.joint(bone).unwrap()).unwrap();
        let d = Vec3::from(a[j].translation).distance(Vec3::from(b[j].translation));
        assert!(d < 0.01, "{bone:?} drift {d} m");
    }
}

#[test]
fn rig_map_round_trips_through_ron() {
    let skeleton = biped().build(true, true);
    let (mut map, _) = auto_map(&skeleton);
    map.auto_align(&skeleton);
    let text = ron::ser::to_string_pretty(&map, ron::ser::PrettyConfig::default()).unwrap();
    let back: RigMap = ron::from_str(&text).unwrap();
    assert_eq!(back.bones, map.bones);
    assert_eq!(back.mirror, map.mirror);
    assert_eq!(back.retarget_pose.len(), map.retarget_pose.len());
    for ((a, qa), (b, qb)) in back.retarget_pose.iter().zip(&map.retarget_pose) {
        assert_eq!(a, b);
        assert!(qa.abs_diff_eq(*qb, 1e-6), "{qa} {qb}");
    }
}

// ---------------------------------------------------------------------------
// Rig profiles from rig maps; digitigrade legs (audit 16–17).

/// A beast-race variant of the Bip01 rig: digitigrade legs with a
/// `HorseLink` metatarsal between calf and foot (knee forward, hock back).
fn beast() -> RigSpec {
    let mut r = biped();
    for (s, side) in [("L", 1.0f32), ("R", -1.0)] {
        let n = |x: &str| format!("Bip01 {s} {x}");
        let calf = r.joints.iter().position(|j| j.0 == n("Calf")).unwrap();
        r.joints[calf].2 = Vec3::new(0.1 * side, 0.58, 0.1);
        let foot = r.joints.iter().position(|j| j.0 == n("Foot")).unwrap();
        r.joints[foot].1 = Some(n("HorseLink"));
        r.joints[foot].2 = Vec3::new(0.1 * side, 0.08, 0.02);
        r.joints.insert(foot, (n("HorseLink"), Some(n("Calf")), Vec3::new(0.1 * side, 0.28, -0.08)));
    }
    r
}

#[test]
fn rig_maps_produce_profiles_and_creatures_need_none() {
    let s = biped().build(false, false);
    let (mut map, _) = auto_map(&s);
    map.auto_align(&s);
    let p = map.profile(&s).unwrap();
    assert_eq!(p.template, sf_anim::RigTemplate::Biped);
    let leg = p.limb("leftLeg").unwrap();
    let names: Vec<&str> = leg.joints.iter().map(|&j| s.name(usize::from(j))).collect();
    assert_eq!(names, ["Bip01 L Thigh", "Bip01 L Calf", "Bip01 L Foot"]);
    assert_eq!((leg.solver, leg.support), (sf_anim::LimbSolver::TwoBone, true));
    assert_eq!(s.name(usize::from(leg.foot.as_ref().unwrap().toe.unwrap())), "Bip01 L Toe0");
    let arm = p.limb("rightArm").unwrap();
    assert_eq!(s.name(usize::from(arm.joints[0])), "Bip01 R Clavicle");
    assert_eq!(arm.joints.len(), 4);
    assert_eq!(p.tails.len(), 1, "the Bip01 tail chain");
    assert!(p.neck_head.is_some() && p.spine.is_some());
    p.validate(&s).unwrap();
    // A creature recipe names a profile, not a humanoid map.
    let creature = RigRef::Profile(VfsPath::new("meshes/guar.glb").unwrap());
    assert_ne!(creature, RigRef::Humanoid(VfsPath::new("meshes/guar.glb").unwrap()));
}

#[test]
#[allow(clippy::many_single_char_names)]
fn digitigrade_legs_keep_foot_contacts_through_retarget() {
    let (s, t) = (biped().build(false, false), beast().build(false, false));
    let (mut sm, _) = auto_map(&s);
    let (mut tm, report) = auto_map(&t);
    sm.auto_align(&s);
    tm.auto_align(&t);
    assert!(tm.validate(&t).is_empty(), "{report:#?}");
    let foot = |m: &RigMap, sk: &Skeleton, side: &str| sk.find(m.joint(if side == "L" { B::LeftFoot } else { B::RightFoot }).unwrap()).unwrap();
    // The beast leg chain is the whole path, metatarsal included.
    let rig = tm.retarget_rig(&t, RetargetOptions::default()).unwrap();
    let leg = rig.chains.iter().find(|c| c.name == "left_leg").unwrap();
    assert_eq!(leg.joints, ["Bip01 L Thigh", "Bip01 L Calf", "Bip01 L HorseLink", "Bip01 L Foot"]);
    assert_eq!(leg.solver, Some(sf_anim::LimbSolver::TwoBoneAnkle));
    let p = tm.profile(&t).unwrap();
    assert_eq!(p.limb("leftLeg").unwrap().kind, sf_anim::LimbKind::Leg(sf_anim::LegKind::Digitigrade));

    let r = Retargeter::new(&s, &sm.retarget_rig(&s, RetargetOptions { ik_legs: false }).unwrap(), &t, &rig).unwrap();
    // Walk-like source poses: legs swung, knees bent, hips bobbing.
    let s_rest = model_of(&s, &sm.retarget_pose_local(&s));
    let t_rest = model_of(&t, &tm.retarget_pose_local(&t));
    let mut out = vec![Trs::IDENTITY; t.len()];
    let mut worst = 0.0f32;
    for k in 0..12 {
        #[allow(clippy::cast_precision_loss)]
        let a = k as f32 * 0.5;
        let mut pose = sm.retarget_pose_local(&s);
        for (bone, amp) in [(B::LeftUpperLeg, 0.4), (B::RightUpperLeg, -0.4), (B::LeftLowerLeg, 0.3), (B::RightLowerLeg, 0.2)] {
            let j = s.find(sm.joint(bone).unwrap()).unwrap();
            pose[j].rotation = (pose[j].rotation * Quat::from_rotation_x(amp * a.sin())).normalize();
        }
        let hips = s.find(sm.joint(B::Hips).unwrap()).unwrap();
        pose[hips].translation.y -= 0.05 * (1.0 + a.cos());
        r.pose(&pose, &[1.0, 1.0], &mut out);
        let (sp, tp) = (model_of(&s, &pose), model_of(&t, &out));
        for side in ["L", "R"] {
            let (sf, tf) = (foot(&sm, &s, side), foot(&tm, &t, side));
            let d = Vec3::from(sp[sf].translation) - Vec3::from(s_rest[sf].translation);
            // Horizontal by the hips ratio, vertical by the leg-length ratio.
            let t_leg = (Vec3::new(0.1, 0.95, 0.0).distance(Vec3::new(0.1, 0.58, 0.1)))
                + Vec3::new(0.1, 0.58, 0.1).distance(Vec3::new(0.1, 0.28, -0.08))
                + Vec3::new(0.1, 0.28, -0.08).distance(Vec3::new(0.1, 0.08, 0.02));
            let s_len = Vec3::new(0.1, 0.95, 0.0).distance(Vec3::new(0.1, 0.5, 0.0)) + Vec3::new(0.1, 0.5, 0.0).distance(Vec3::new(0.1, 0.08, 0.0));
            let want = Vec3::from(t_rest[tf].translation) + Vec3::new(d.x * r.scale(), d.y * t_leg / s_len, d.z * r.scale());
            worst = worst.max(Vec3::from(tp[tf].translation).distance(want));
        }
    }
    assert!(worst < 0.01, "worst foot miss {worst} m");
}

// ---------------------------------------------------------------------------
// Slots and outfits.

struct Content {
    db: RecordDb,
    slots: SlotRegistry,
}

fn part(db: &mut RecordDb, src: sf_records::SourceId, id: &str, slot: SlotId) -> PartRef {
    let model = VfsPath::new(&format!("meshes/{id}.glb")).unwrap();
    db.define(src, RecordId::new(id), BodyPart { model, slot, kind: PartKind::Skinned, node_filter: None }).unwrap();
    RecordId::new(id)
}

fn content() -> Content {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let slots = SlotRegistry::with_humanoid_defaults();
    let s = |n: &str| slots.get(n).unwrap();
    for (id, slot_name) in [
        ("skin_chest", slot::CHEST),
        ("skin_legs_l", slot::LEFT_UPPER_LEG),
        ("skin_legs_r", slot::RIGHT_UPPER_LEG),
        ("skin_head", slot::HEAD),
        ("hair_long", slot::HAIR),
        ("shirt", slot::CHEST),
        ("cuirass", slot::CHEST),
        ("robe_top", slot::CHEST),
        ("robe_skirt", slot::LEFT_UPPER_LEG),
        ("greave_l", slot::LEFT_UPPER_LEG),
        ("helm", slot::HEAD),
        ("arms_1p", slot::LEFT_HAND),
        ("gauntlet_1p", slot::LEFT_HAND),
    ] {
        part(&mut db, src, id, s(slot_name));
    }
    let w = |parts: &[(&str, &str)], hides: &[&str], layer: Layer, fp: &[(&str, &str)]| Wearable {
        parts: parts.iter().map(|(sl, p)| (s(sl), RecordId::new(p))).collect(),
        hides: hides.iter().map(|h| s(h)).collect(),
        layer,
        first_person: fp.iter().map(|(sl, p)| (s(sl), RecordId::new(p))).collect(),
    };
    let items = [
        ("shirt", w(&[(slot::CHEST, "shirt")], &[], Layer::CLOTHING, &[])),
        ("cuirass", w(&[(slot::CHEST, "cuirass")], &[], Layer::ARMOR, &[])),
        ("robe", w(&[(slot::CHEST, "robe_top"), (slot::LEFT_UPPER_LEG, "robe_skirt")], &[slot::RIGHT_UPPER_LEG], Layer::OUTER, &[])),
        ("greaves", w(&[(slot::LEFT_UPPER_LEG, "greave_l")], &[], Layer::ARMOR, &[])),
        ("helm", w(&[(slot::HEAD, "helm")], &[slot::HAIR], Layer::ARMOR, &[])),
        ("gauntlets", w(&[], &[], Layer::ARMOR, &[(slot::LEFT_HAND, "gauntlet_1p")])),
        ("vest", w(&[(slot::CHEST, "shirt")], &[], Layer::CLOTHING, &[])),
        ("tunic", w(&[(slot::CHEST, "robe_top")], &[], Layer::CLOTHING, &[])),
    ];
    for (id, item) in items {
        db.define(src, RecordId::new(id), item).unwrap();
    }
    Content { db, slots }
}

fn recipe(worn: &[&str]) -> CharacterRecipe {
    let path = |p: &str| VfsPath::new(p).unwrap();
    CharacterRecipe {
        skeleton: path("skeletons/human.glb"),
        rig: RigRef::Humanoid(path("skeletons/human.glb")),
        base: ["skin_chest", "skin_legs_l", "skin_legs_r"].iter().map(|s| RecordId::new(s)).collect(),
        head: Some(RecordId::new("skin_head")),
        hair: Some(RecordId::new("hair_long")),
        worn: worn.iter().map(|s| RecordId::new(s)).collect(),
        held: Vec::new(),
        scale: Vec3::ONE,
        graph: path("graphs/humanoid.ron"),
        tint: TintParams::default(),
        shape: ShapeWeights::default(),
        first_person: Some(FirstPersonRecipe {
            skeleton: path("skeletons/arms.glb"),
            rig: None,
            parts: vec![RecordId::new("arms_1p")],
            graph: path("graphs/arms.ron"),
            fov_y_deg: 60.0,
        }),
    }
}

fn visible(c: &Content, outfit: &Outfit) -> Vec<(String, String)> {
    outfit.parts.iter().map(|(s, p)| (c.slots.name(*s).unwrap().to_owned(), p.as_str().to_owned())).collect()
}

fn resolve_worn(c: &Content, worn: &[&str]) -> Outfit {
    resolve_recipe(&c.db, &c.slots, &recipe(worn), View::ThirdPerson).unwrap()
}

#[test]
fn slot_resolution_layers_hides_and_ties() {
    let c = content();
    let naked = resolve_worn(&c, &[]);
    assert_eq!(naked.parts.len(), 5);

    // Armor over clothing whatever the equip order.
    let a = resolve_worn(&c, &["cuirass", "shirt"]);
    let b = resolve_worn(&c, &["shirt", "cuirass"]);
    assert!(visible(&c, &a).contains(&("chest".into(), "cuirass".into())));
    assert_eq!(a, b, "same look, same key, whatever the equip order");

    // The helmet hides the hair; the robe (outer) covers chest and left leg and hides the right leg.
    let robe = resolve_worn(&c, &["robe", "cuirass", "greaves", "helm"]);
    let v = visible(&c, &robe);
    assert!(v.contains(&("chest".into(), "robe_top".into())));
    assert!(v.contains(&("left_upper_leg".into(), "robe_skirt".into())));
    assert!(!v.iter().any(|(s, _)| s == "right_upper_leg" || s == "hair"), "{v:?}");
    assert!(v.contains(&("head".into(), "helm".into())));

    // Ties within a layer: the later equip wins.
    let tie_a = resolve_worn(&c, &["tunic", "shirt"]);
    let tie_b = resolve_worn(&c, &["shirt", "tunic"]);
    assert!(visible(&c, &tie_a).contains(&("chest".into(), "shirt".into())));
    assert!(visible(&c, &tie_b).contains(&("chest".into(), "robe_top".into())));

    // Keys: equal looks reached differently share a key; different looks don't.
    assert_eq!(resolve_worn(&c, &["shirt"]).key, resolve_worn(&c, &["vest"]).key, "both show the same part");
    assert_ne!(naked.key, a.key);
    assert_ne!(a.key, robe.key);
    assert_eq!(naked.key, resolve_worn(&c, &["gauntlets"]).key, "first-person-only items don't change the body");
}

#[test]
fn first_person_view_uses_view_model_parts() {
    let c = content();
    let fp = resolve_recipe(&c.db, &c.slots, &recipe(&["gauntlets", "cuirass"]), View::FirstPerson).unwrap();
    assert_eq!(visible(&c, &fp), vec![("left_hand".to_owned(), "gauntlet_1p".to_owned())]);
    let bare = resolve_recipe(&c.db, &c.slots, &recipe(&["cuirass"]), View::FirstPerson).unwrap();
    assert_eq!(visible(&c, &bare), vec![("left_hand".to_owned(), "arms_1p".to_owned())]);
}

#[test]
fn part_node_filter_and_mirrored_rigid_offset() {
    let c = content();
    let mut left = c.db.get::<BodyPart>(&RecordId::new("skin_legs_l")).unwrap().clone();
    assert!(left.keeps_node(["Root", "Tri Right Leg"]), "no filter keeps everything");
    left.node_filter = Some("Left Leg".into());
    assert!(left.keeps_node(["Root", "Legs", "tri left leg 01"]), "case-insensitive, any node on the path");
    assert!(!left.keeps_node(["Root", "Tri Right Leg"]));
    let PartKind::Rigid { joint, offset } = PartKind::rigid("Left Clavicle") else { unreachable!() };
    assert_eq!((joint.as_str(), offset), ("Left Clavicle", glam::Affine3A::IDENTITY));
    let mirrored = glam::Affine3A::from_scale(Vec3::new(-1.0, 1.0, 1.0));
    assert!(mirrored.matrix3.determinant() < 0.0, "a mirroring offset is told apart by its determinant");
}

#[test]
fn outfit_key_is_stable_across_registries() {
    // Same slots registered in another order: other ids, same key.
    let c = content();
    let mut other = SlotRegistry::new();
    other.register("hair", BoneSet::EMPTY);
    other.register_humanoid_defaults();
    let parts = |slots: &SlotRegistry| vec![(slots.get("hair").unwrap(), RecordId::new("h")), (slots.get("chest").unwrap(), RecordId::new("c"))];
    assert_ne!(c.slots.get("hair"), other.get("hair"));
    assert_eq!(outfit_key(&c.slots, &parts(&c.slots)), outfit_key(&other, &parts(&other)));
}

#[test]
fn missing_records_are_reported() {
    let c = content();
    let err = resolve_recipe(&c.db, &c.slots, &recipe(&["nope"]), View::ThirdPerson).unwrap_err();
    assert_eq!(err, CharacterError::MissingRecord { kind: Wearable::KIND, id: RecordId::new("nope") });
}

#[test]
fn slot_registry_regions() {
    let mut slots = SlotRegistry::with_humanoid_defaults();
    let hand = slots.get(slot::LEFT_HAND).unwrap();
    assert!(slots.def(hand).unwrap().region.contains(HumanoidBone::LeftIndexDistal));
    assert!(slots.covering(HumanoidBone::Head).count() >= 3);
    let n = slots.len();
    let again = slots.register(slot::TAIL, BoneSet::of(&[HumanoidBone::Hips]));
    assert_eq!(slots.len(), n);
    assert!(slots.def(again).unwrap().region.contains(HumanoidBone::Hips), "re-registering extends the region");
}

/// An adapter sketch: the trait is object-safe and Bevy-free.
#[test]
fn character_source_is_object_safe() {
    struct Fixed;
    impl CharacterSource for Fixed {
        fn recipe(&self, kind: &str, id: &RecordId, state: &CharacterState) -> Result<CharacterRecipe, CharacterError> {
            if kind != "npc" {
                return Err(CharacterError::UnknownBase { kind: kind.to_owned(), id: id.clone() });
            }
            let mut r = recipe(&["shirt"]);
            if let Some(worn) = &state.worn {
                r.worn.clone_from(worn);
            }
            Ok(r)
        }
        fn rig_maps(&self) -> Vec<(VfsPath, RigMap)> {
            Vec::new()
        }
    }
    let source: Box<dyn CharacterSource> = Box::new(Fixed);
    let state = CharacterState { worn: Some(vec![RecordId::new("robe")]), held: None };
    let r = source.recipe("npc", &RecordId::new("guard"), &state).unwrap();
    assert_eq!(r.worn, vec![RecordId::new("robe")]);
    assert!(source.recipe("creature", &RecordId::new("rat"), &state).is_err());
}

/// Held items: a weapon shown only when drawn, an off-hand torch that stays
/// at its socket but hides next to a drawn two-handed weapon, its light.
#[test]
fn held_items_mount_by_readiness() {
    let path = |p: &str| VfsPath::new(p).unwrap();
    let grip = glam::Affine3A::from_translation(Vec3::new(0.0, 0.05, 0.0));
    let sword = HeldItem {
        item: RecordId::new("claymore"),
        model: path("meshes/claymore.glb"),
        hand: Hand::Main,
        drawn: Some(Mount { socket: "hand_r_socket".into(), offset: grip }),
        sheathed: Some(Mount::at("back_sheath")),
        light: None,
    };
    let mut torch = HeldItem::always(RecordId::new("torch"), path("meshes/torch.glb"), Hand::Off, Mount::at("hand_l_socket"));
    torch.drawn = None;
    torch.light = Some(HeldLight { attach_node: Some("flame".into()), color: Vec3::new(1.0, 0.6, 0.3), radius: 4.0 });
    assert_eq!(sword.mount(Readiness::Drawn).map(|m| (m.socket.as_str(), m.offset)), Some(("hand_r_socket", grip)));
    assert_eq!(sword.mount(Readiness::Sheathed).map(|m| m.socket.as_str()), Some("back_sheath"));
    assert_eq!(torch.mount(Readiness::Sheathed).map(|m| m.socket.as_str()), Some("hand_l_socket"));
    assert!(torch.mount(Readiness::Drawn).is_none());
    assert_eq!(Readiness::default(), Readiness::Sheathed);
}

// Outfit assembly: morph-bearing parts (character.md "Morph-bearing parts").
mod assembly;
