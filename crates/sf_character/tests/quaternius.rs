//! Auto-map and auto-align on real CC0 Quaternius rigs (tests/data): the
//! Universal rig (UE-style names) and the Universal Animation Library 1
//! rig (Rigify `DEF-` names). No manual fixes allowed.

use std::collections::HashMap;

use glam::{Quat, Vec3};
use sf_anim::{AnimImporter, GltfAnimImporter, Skeleton};
use sf_character::{Finger, HumanoidBone, HumanoidBone as B, Limb as L, RetargetOptions, RigMap, Side, auto_map};

fn load(name: &str) -> Skeleton {
    let path = format!("{}/tests/data/{name}", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("fixture");
    let src = GltfAnimImporter.import(name, &bytes).expect("imports");
    Skeleton::new(&src.skeleton.expect("skin joints")).expect("valid").0
}

/// The expected map: `prefix`/`suffix` style names for every bone the rigs have.
fn expected(rigify: bool) -> Vec<(HumanoidBone, String)> {
    let mut out = Vec::new();
    let c = |ue: &str, def: &str| {
        if rigify { format!("DEF-{def}") } else { ue.to_owned() }
    };
    out.extend([
        (B::Hips, c("pelvis", "hips")),
        (B::Spine, c("spine_01", "spine.001")),
        (B::Chest, c("spine_02", "spine.002")),
        (B::UpperChest, c("spine_03", "spine.003")),
        (B::Neck, c("neck_01", "neck")),
        (B::Head, c("Head", "head")),
    ]);
    for side in [Side::Left, Side::Right] {
        let (ue, def) = if side == Side::Left { ("l", "L") } else { ("r", "R") };
        let s = |u: &str, d: &str| {
            if rigify { format!("DEF-{d}.{def}") } else { format!("{u}_{ue}") }
        };
        let limb = |l| HumanoidBone::limb(side, l).unwrap();
        out.extend([
            (limb(L::Shoulder), s("clavicle", "shoulder")),
            (limb(L::UpperArm), s("upperarm", "upper_arm")),
            (limb(L::LowerArm), s("lowerarm", "forearm")),
            (limb(L::Hand), s("hand", "hand")),
            (limb(L::UpperLeg), s("thigh", "thigh")),
            (limb(L::LowerLeg), s("calf", "shin")),
            (limb(L::Foot), s("foot", "foot")),
            (limb(L::Toes), s("ball", "toe")),
        ]);
        for f in Finger::ALL {
            for k in 0..3 {
                let ue_name = format!("{}_0{}", if f == Finger::Little { "pinky" } else { f.name() }, k + 1);
                let def_name = match f {
                    Finger::Thumb => format!("thumb.0{}", k + 1),
                    Finger::Little => format!("f_pinky.0{}", k + 1),
                    _ => format!("f_{}.0{}", f.name(), k + 1),
                };
                out.push((HumanoidBone::finger(side, f, k).unwrap(), s(&ue_name, &def_name)));
            }
        }
    }
    out
}

fn check(file: &str, rigify: bool) -> (Skeleton, RigMap) {
    let skeleton = load(file);
    let (mut map, report) = auto_map(&skeleton);
    assert!(report.is_clean(), "{file}: {report:#?}");
    let want = expected(rigify);
    assert_eq!(map.bones.len(), want.len(), "{file}: {:?}", map.bones);
    for (bone, joint) in &want {
        assert_eq!(map.joint(*bone), Some(joint.as_str()), "{file}: {bone:?}");
    }
    assert_eq!(map.root_motion, "root");
    assert!(map.extra_chains.is_empty() && report.unmapped.is_empty(), "{file}: {:?} {:?}", map.extra_chains, report.unmapped);
    assert_eq!(map.mirror.len(), if rigify { 23 } else { 29 }, "{file}: {:?}", map.mirror);

    let align = map.auto_align(&skeleton);
    assert!(align.facing_correction_deg < 0.01, "{file}: Quaternius rigs face +Z, Y up");
    assert!(align.max_residual_deg < 0.05, "{file}: {align:?}");
    let biggest = align.rotated.iter().map(|(_, a)| *a).fold(0.0f32, f32::max);
    println!("{file}: {} joints rotated, largest {biggest:.1}°", align.rotated.len());
    map.retarget_rig(&skeleton, RetargetOptions::default()).expect("retarget rig");
    (skeleton, map)
}

#[test]
fn quaternius_universal_rig_auto_maps_without_fixes() {
    check("quaternius_universal_rig.gltf", false);
}

#[test]
fn quaternius_ual1_rig_auto_maps_without_fixes() {
    let (skeleton, map) = check("quaternius_ual1_rig.gltf", true);

    // The library's own T-pose clip against the auto-aligned retarget pose:
    // limb directions agree.
    let tpose_json = std::fs::read_to_string(format!("{}/tests/data/quaternius_ual1_tpose.json", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let keys: HashMap<String, [f32; 4]> = serde_json::from_str(&tpose_json).unwrap();
    let mut local = skeleton.rest().to_vec();
    for (name, q) in &keys {
        if let Some(j) = skeleton.find(name) {
            local[j].rotation = Quat::from_array(*q).normalize();
        }
    }
    let clip_model = model(&skeleton, &local);
    let aligned_model = model(&skeleton, &map.retarget_pose_local(&skeleton));
    for (a, b) in [
        (B::LeftUpperArm, B::LeftLowerArm),
        (B::RightLowerArm, B::RightHand),
        (B::LeftUpperLeg, B::LeftLowerLeg),
        (B::RightLowerLeg, B::RightFoot),
        (B::Spine, B::Chest),
    ] {
        let d = |m: &[Vec3]| {
            let (ja, jb) = (skeleton.find(map.joint(a).unwrap()).unwrap(), skeleton.find(map.joint(b).unwrap()).unwrap());
            (m[jb] - m[ja]).normalize()
        };
        let angle = d(&clip_model).angle_between(d(&aligned_model)).to_degrees();
        println!("{a:?}→{b:?}: A_TPose vs auto-align {angle:.2}°");
        assert!(angle < 12.0, "{a:?}→{b:?}: {angle}°");
    }
}

fn model(skeleton: &Skeleton, local: &[sf_anim::Trs]) -> Vec<Vec3> {
    let mut mats: Vec<glam::Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        mats.push(skeleton.parent(j).map_or(m, |p| mats[p] * m));
    }
    mats.iter().map(|m| Vec3::from(m.translation)).collect()
}
