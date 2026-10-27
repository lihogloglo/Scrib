//! BVH and FBX importers, source spaces and root-motion detection.

use glam::{Quat, Vec3};

use crate::{Axis, Curve, SourceClip, SourceJoint, SourceSpace, SourceTrack, Trs, detect_root_motion_joint};

#[cfg(any(feature = "bvh", feature = "fbx"))]
fn model_positions(joints: &[SourceJoint], local: &[Trs]) -> Vec<Vec3> {
    let mut mats: Vec<glam::Affine3A> = Vec::with_capacity(joints.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        mats.push(joints[j].parent.map_or(m, |p| mats[p] * m));
    }
    mats.iter().map(|m| Vec3::from(m.translation)).collect()
}

#[test]
fn source_space_converts_axes_units_and_handedness() {
    assert!(SourceSpace { up: Axis::PosY, forward: Axis::NegY, ..SourceSpace::default() }.converter().is_none());
    let id = SourceSpace::default().converter().unwrap();
    let t = Trs::new(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_x(0.3), Vec3::new(1.0, 2.0, 3.0));
    assert!(id.trs(t).translation.distance(t.translation) < 1e-6 && id.trs(t).rotation.dot(t.rotation).abs() > 1.0 - 1e-6);
    // Z up, facing −Y (3ds Max), inches: up → Y, forward → Z, left (+X) stays.
    let max = SourceSpace { unit_scale: 0.0254, up: Axis::PosZ, forward: Axis::NegY, right_handed: true }.converter().unwrap();
    assert!(max.translation(Vec3::new(1.0, -2.0, 3.0)).distance(Vec3::new(1.0, 3.0, 2.0) * 0.0254) < 1e-6);
    assert!(max.scale(Vec3::new(1.0, 2.0, 3.0)).distance(Vec3::new(1.0, 3.0, 2.0)) < 1e-6);
    // A rotation about the source up axis is a rotation about +Y.
    let r = max.rotation(Quat::from_rotation_z(0.5));
    assert!(r.angle_between(Quat::from_rotation_y(0.5)) < 1e-5, "{r:?}");
    // Left-handed Y up facing +Z: X flips (mirror), rotations stay proper and agree with points.
    let lh = SourceSpace { right_handed: false, ..SourceSpace::default() }.converter().unwrap();
    let (q, v) = (Quat::from_euler(glam::EulerRot::ZXY, 0.4, -0.7, 1.1), Vec3::new(0.3, -1.2, 0.8));
    assert!(lh.translation(v).distance(Vec3::new(-0.3, -1.2, 0.8)) < 1e-6);
    assert!((lh.rotation(q) * lh.translation(v)).distance(lh.translation(q * v)) < 1e-5);
}

#[test]
fn root_motion_detection_prefers_a_moving_root_then_the_shallowest_mover() {
    let j = |name: &str, parent: Option<usize>| SourceJoint { name: name.into(), parent, rest: Trs::IDENTITY };
    let joints = vec![j("Armature", None), j("mixamorig:Root", Some(0)), j("mixamorig:Hips", Some(1)), j("mixamorig:Spine", Some(2))];
    let moving = |name: &str, d: f32| SourceTrack {
        joint: name.into(),
        translation: Some(Curve::linear(vec![0.0, 1.0], vec![Vec3::ZERO, Vec3::new(0.0, 0.0, d)])),
        ..SourceTrack::default()
    };
    let clip = |tracks| vec![SourceClip { name: "c".into(), duration: 1.0, tracks, ..SourceClip::default() }];
    assert_eq!(detect_root_motion_joint(&joints, &clip(vec![moving("mixamorig:Hips", 1.0), moving("mixamorig:Root", 1.0)])), Some(1));
    // An in-place root: the hips carry the motion.
    assert_eq!(detect_root_motion_joint(&joints, &clip(vec![moving("mixamorig:Hips", 1.0), moving("mixamorig:Root", 0.0)])), Some(2));
    // Nothing moves: the named root; no root either: none.
    assert_eq!(detect_root_motion_joint(&joints, &clip(vec![])), Some(1));
    assert_eq!(detect_root_motion_joint(&joints[2..3].iter().map(|j| SourceJoint { parent: None, ..j.clone() }).collect::<Vec<_>>(), &[]), None);
}

#[cfg(feature = "bvh")]
mod bvh {
    use super::*;
    use crate::{AnimImporter, BoundSource, BvhAnimImporter, ClipRules, CookSettings, Skeleton, SkeletonRules, cook_clip};

    /// Hips (6 channels) → Spine → Bip01 Head (a name with a space) → End
    /// Site, and a leg; 4 frames at 120 fps; centimeters, Y up.
    const BVH: &str = "HIERARCHY
ROOT Hips
{
\tOFFSET 0.0 90.0 0.0
\tCHANNELS 6 Xposition Yposition Zposition Zrotation Xrotation Yrotation
\tJOINT Spine
\t{
\t\tOFFSET 0 10 0
\t\tCHANNELS 3 Zrotation Xrotation Yrotation
\t\tJOINT Bip01 Head
\t\t{
\t\t\tOFFSET 0 40 0
\t\t\tCHANNELS 3 Zrotation Xrotation Yrotation
\t\t\tEnd Site
\t\t\t{
\t\t\t\tOFFSET 0 15 0
\t\t\t}
\t\t}
\t}
\tJOINT LeftUpLeg
\t{
\t\tOFFSET 10 -5 0
\t\tCHANNELS 3 Zrotation Xrotation Yrotation
\t\tEnd Site
\t\t{
\t\t\tOFFSET 0 -85 0
\t\t}
\t}
}
MOTION
Frames: 4
Frame Time: 0.00833333
0 90 0   0 0 0   0 0 0   0 0 0   0 0 0
0 90 1   0 0 0   90 90 0   0 0 0   0 0 0
0 90 2   0 0 0   0 0 0   0 0 0   0 30 0
0 90 3   0 0 0   0 0 0   0 0 0   0 0 0
";

    #[test]
    fn bvh_imports_hierarchy_channels_rate_and_root_motion() {
        let src = BvhAnimImporter::default().import("byo/walk.bvh", BVH.as_bytes()).unwrap();
        let joints = src.skeleton.as_ref().unwrap();
        let names: Vec<&str> = joints.iter().map(|j| j.name.as_str()).collect();
        assert_eq!(names, ["Hips", "Spine", "Bip01 Head", "Bip01 Head_End", "LeftUpLeg", "LeftUpLeg_End"]);
        let parents: Vec<Option<usize>> = joints.iter().map(|j| j.parent).collect();
        assert_eq!(parents, [None, Some(0), Some(1), Some(2), Some(0), Some(4)]);
        assert!(joints[0].rest.translation.distance(Vec3::new(0.0, 0.9, 0.0)) < 1e-6, "centimeters → meters");
        assert!(joints[3].rest.translation.distance(Vec3::new(0.0, 0.15, 0.0)) < 1e-6);
        assert_eq!(src.root_motion.as_deref(), Some("Hips"));

        let clip = &src.clips[0];
        assert_eq!(clip.name, "walk");
        assert!((clip.duration - 3.0 * 0.008_333_33).abs() < 1e-6);
        assert_eq!(clip.tracks.len(), 4, "end sites have no track");
        let (skeleton, _) = Skeleton::new(joints).unwrap();
        let bound = BoundSource::new(clip, &skeleton);
        assert!(bound.unbound.is_empty());
        let rate = bound.native_rate().unwrap();
        assert!((rate - 120.0).abs() < 0.01, "{rate}");
        // Frame 1: Spine `Zrotation 90 Xrotation 90` is Rz·Rx (listed order), hips moved 1 cm forward.
        let spine = clip.tracks.iter().find(|t| t.joint == "Spine").unwrap();
        let want = Quat::from_rotation_z(90f32.to_radians()) * Quat::from_rotation_x(90f32.to_radians());
        assert!(spine.rotation.as_ref().unwrap().values[1].dot(want).abs() > 1.0 - 1e-6);
        assert!(spine.translation.is_none(), "rotation-only joints keep their offset");
        let hips = clip.tracks[0].translation.as_ref().unwrap();
        assert!(hips.values[1].distance(Vec3::new(0.0, 0.9, 0.01)) < 1e-6, "position channels replace the offset");

        // anim.cook with the detected root: 3 cm of forward travel becomes root motion.
        let rules = SkeletonRules { root_motion_joint: src.root_motion.clone(), ..SkeletonRules::default() };
        let (skeleton, _) = Skeleton::with_rules(joints, &rules).unwrap();
        let cooked = cook_clip(&skeleton, clip, &ClipRules::default(), &CookSettings::default()).unwrap();
        let (p, yaw) = cooked.root_at(cooked.duration);
        assert!(p.distance(Vec3::new(0.0, 0.0, 0.03)) < 1e-4 && yaw.abs() < 1e-4, "{p} {yaw}");
    }

    /// A reference frame makes that frame's pose the skeleton's rest (its
    /// rotations and position channels); the clip and other paths don't change.
    #[test]
    fn bvh_rest_frame_takes_the_rest_pose_from_a_frame() {
        let plain = BvhAnimImporter::default();
        let importer = BvhAnimImporter::default().with_rest_frame("byo/mocap/", 1).with_rest_frame("byo/mocap/far/", 99);
        assert_eq!(importer.rest_frame_for("byo/walk.bvh"), None);
        assert_eq!(importer.settings_key("byo/walk.bvh"), plain.settings_key("byo/walk.bvh"), "other files keep their key");
        assert_ne!(importer.settings_key("byo/mocap/walk.bvh"), plain.settings_key("byo/mocap/walk.bvh"));
        let zero = plain.import("byo/mocap/walk.bvh", BVH.as_bytes()).unwrap();
        let src = importer.import("byo/mocap/walk.bvh", BVH.as_bytes()).unwrap();
        let joints = src.skeleton.as_ref().unwrap();
        let want = Quat::from_rotation_z(90f32.to_radians()) * Quat::from_rotation_x(90f32.to_radians());
        assert!(joints[1].rest.rotation.dot(want).abs() > 1.0 - 1e-6, "Spine rests at frame 1");
        assert!(joints[0].rest.translation.distance(Vec3::new(0.0, 0.9, 0.01)) < 1e-6, "Hips position channels");
        assert_eq!(joints[1].rest.translation, zero.skeleton.as_ref().unwrap()[1].rest.translation, "rotation-only joints keep their offset");
        assert_eq!(joints[3].rest, zero.skeleton.as_ref().unwrap()[3].rest, "end sites keep theirs");
        assert_eq!(src.clips, zero.clips);
        let far = importer.import("byo/mocap/far/walk.bvh", BVH.as_bytes()).unwrap();
        assert_eq!(far.skeleton, zero.skeleton, "a frame past the end keeps the zero pose");
    }

    #[test]
    fn bvh_spaces_convert_model_positions_and_follow_path_overrides() {
        let zup = SourceSpace { unit_scale: 0.0254, up: Axis::PosZ, forward: Axis::NegY, right_handed: true };
        let importer = BvhAnimImporter::default()
            .with_override("byo/cmu/", zup)
            .with_override("byo/cmu/lh/", SourceSpace { right_handed: false, ..SourceSpace::centimeters() });
        assert_eq!(importer.space_for("byo/cmu/01_01.bvh"), zup);
        assert!(!importer.space_for("byo/cmu/lh/a.bvh").right_handed);
        assert_eq!(importer.space_for("byo/100style/a.bvh"), SourceSpace::centimeters());
        assert_ne!(importer.settings_key("byo/cmu/01_01.bvh"), importer.settings_key("byo/100style/a.bvh"));

        // Raw (unit, unconverted) frame-1 model positions vs the converted import's.
        let raw = BvhAnimImporter::new(SourceSpace::default()).import("a.bvh", BVH.as_bytes()).unwrap();
        for (path, space) in [("byo/cmu/a.bvh", zup), ("byo/cmu/lh/a.bvh", importer.space_for("byo/cmu/lh/a.bvh"))] {
            let conv = space.converter().unwrap();
            let got = importer.import(path, BVH.as_bytes()).unwrap();
            let pose = |src: &crate::AnimSource| {
                let joints = src.skeleton.as_ref().unwrap();
                let (skeleton, order) = Skeleton::new(joints).unwrap();
                let mut local = vec![Trs::IDENTITY; skeleton.len()];
                BoundSource::new(&src.clips[0], &skeleton).sample(src.clips[0].duration / 3.0, skeleton.rest(), &mut local);
                // Back to source order.
                let mut by_source = vec![Trs::IDENTITY; joints.len()];
                for (s, &k) in order.iter().enumerate() {
                    by_source[s] = local[k];
                }
                model_positions(joints, &by_source)
            };
            let (want, have) = (pose(&raw), pose(&got));
            for (w, h) in want.iter().zip(&have) {
                assert!(conv.translation(*w).distance(*h) < 1e-5, "{path}: {w} → {h}");
            }
        }
    }

    #[test]
    fn bvh_rejects_malformed_files() {
        let imp = BvhAnimImporter::default();
        for (bad, why) in [
            ("HIERARCHY\nROOT a\n{\nOFFSET 0 0 0\n", "no MOTION"),
            ("HIERARCHY\nROOT a\n{\nOFFSET 0 0 0\nMOTION\nFrames: 0\nFrame Time: 0.1\n", "unclosed"),
            ("HIERARCHY\nROOT a\n{\nCHANNELS 1 Wrotation\n}\nMOTION\nFrames: 0\nFrame Time: 0.1\n", "channel"),
            ("HIERARCHY\nROOT a\n{\n}\nMOTION\nFrames: 1\nFrame Time: 0\n", "Frame Time"),
            ("HIERARCHY\nJOINT a\n{\n}\nMOTION\nFrames: 0\nFrame Time: 0.1\n", "outside"),
        ] {
            let e = imp.import("x.bvh", bad.as_bytes()).unwrap_err();
            assert!(e.message.contains(why), "{why}: {e}");
        }
        // A truncated take keeps its complete frames; a rest-pose-only file has no clip.
        let cut = BVH.rsplit_once("0 90 3").unwrap().0;
        assert_eq!(imp.import("x.bvh", cut.as_bytes()).unwrap().clips[0].tracks[0].translation.as_ref().unwrap().times.len(), 3);
        let still = imp.import("x.bvh", b"HIERARCHY\nROOT a {\nOFFSET 0 0 0\n}\nMOTION\nFrames: 0\nFrame Time: 0.1\n").unwrap();
        assert!(still.clips.is_empty() && still.skeleton.unwrap().len() == 1);
    }

    #[test]
    fn framework_importers_cover_bvh_and_fbx_by_extension() {
        let all = crate::steps::AnimImporters::framework();
        assert!(all.for_path("byo/Walk.BVH").is_some());
        assert_eq!(all.for_path("x.fbx").is_some(), cfg!(feature = "fbx"));
    }
}

#[cfg(feature = "fbx")]
mod fbx {
    use super::*;
    use crate::{AnimImporter, BoundSource, FbxAnimImporter, Skeleton};

    /// `tests/data/biped_walk.fbx` (CC0, made by `make_biped_walk_fbx.py`):
    /// Blender (Z up, facing −Y, meters), 24 fps, 1 s; the armature object
    /// travels 1 m forward, the left thigh swings ±30°.
    #[test]
    fn fbx_imports_a_blender_rig_in_framework_space() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/biped_walk.fbx");
        let bytes = std::fs::read(path).unwrap();
        let src = FbxAnimImporter.import("biped_walk.fbx", &bytes).unwrap();
        let joints = src.skeleton.as_ref().unwrap();
        assert_eq!(joints.len(), 21, "{:?}", joints.iter().map(|j| &j.name).collect::<Vec<_>>());
        assert_eq!(joints[0].name, "Armature");
        let find = |n: &str| joints.iter().position(|j| j.name == n).unwrap_or_else(|| panic!("{n}"));
        for j in joints {
            assert!(j.rest.scale.distance(Vec3::ONE) < 1e-4, "{}: unit scale folded into translations: {}", j.name, j.rest.scale);
        }
        let rest: Vec<Trs> = joints.iter().map(|j| j.rest).collect();
        let m = model_positions(joints, &rest);
        let (hips, head, l_up, r_up, l_toe) = (
            find("mixamorig:Hips"),
            find("mixamorig:Head"),
            find("mixamorig:LeftUpLeg"),
            find("mixamorig:RightUpLeg"),
            find("mixamorig:LeftToeBase"),
        );
        println!("hips {} head {} left thigh {} toe {}", m[hips], m[head], m[l_up], m[l_toe]);
        assert!(m[hips].distance(Vec3::new(0.0, 1.0, 0.0)) < 1e-3, "Y up, meters: {}", m[hips]);
        assert!(m[head].distance(Vec3::new(0.0, 1.5, 0.0)) < 1e-3);
        assert!((m[l_up] - m[r_up]).distance(Vec3::new(0.2, 0.0, 0.0)) < 1e-3, "left is +X");
        assert!(m[l_toe].z > m[l_up].z + 0.1, "the rig faces +Z (toes forward): {}", m[l_toe]);

        let clip = &src.clips[0];
        assert!((clip.duration - 1.0).abs() < 1e-4, "{}", clip.duration);
        let (skeleton, _) = Skeleton::new(joints).unwrap();
        let bound = BoundSource::new(clip, &skeleton);
        assert!((bound.native_rate().unwrap() - 24.0).abs() < 0.01);
        assert_eq!(src.root_motion.as_deref(), Some("Armature"), "the armature object carries the travel");
        let arm = clip.tracks.iter().find(|t| t.joint == "Armature").unwrap().translation.as_ref().unwrap();
        let travel = arm.sample(clip.duration) - arm.sample(0.0);
        assert!(travel.distance(Vec3::new(0.0, 0.0, 1.0)) < 1e-3, "1 m forward (+Z): {travel}");
        let thigh = clip.tracks.iter().find(|t| t.joint == "mixamorig:LeftUpLeg").unwrap().rotation.as_ref().unwrap();
        let swing = thigh.sample(0.25).angle_between(joints[l_up].rest.rotation).to_degrees();
        assert!((swing - 30.0).abs() < 0.5, "{swing}°");
        assert!(clip.tracks.iter().all(|t| t.joint != "mixamorig:Spine"), "static joints have no track");
    }
}
