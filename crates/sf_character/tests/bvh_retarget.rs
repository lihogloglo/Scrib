//! Round trip: a mocap-style BVH (Mixamo/CMU names, centimeters, Y up,
//! 60 fps) → `BvhAnimImporter` → auto-map + auto-align → retarget onto a
//! 3ds Max Biped-style skeleton (`Bip01 …` names, Z up facing −Y, meters,
//! arbitrary joint frames, an extra `Bip01` root) with the same proportions.
//! Every mapped joint must land within 1 cm and every limb segment within
//! 1° of the source pose.

use std::fmt::Write as _;

use glam::{Affine3A, EulerRot, Quat, Vec3};
use sf_anim::{AnimImporter, BoundSource, BvhAnimImporter, Retargeter, Skeleton, SkeletonRules, SourceJoint, Trs};
use sf_character::{HumanoidBone, RetargetOptions, auto_map};

struct J {
    bvh: String,
    bip: String,
    parent: Option<usize>,
    /// Centimeters, Y up, facing +Z, left = +X.
    offset: Vec3,
    end: Option<Vec3>,
}

fn body() -> Vec<J> {
    let j = |bvh: &str, bip: &str, parent, offset: [f32; 3], end: Option<[f32; 3]>| J {
        bvh: bvh.into(),
        bip: bip.into(),
        parent,
        offset: offset.into(),
        end: end.map(Vec3::from),
    };
    let mut out = vec![
        j("Hips", "Bip01 Pelvis", None, [0.0, 100.0, 0.0], None),
        j("Spine", "Bip01 Spine", Some(0), [0.0, 10.0, 0.0], None),
        j("Chest", "Bip01 Spine1", Some(1), [0.0, 15.0, 0.0], None),
        j("Neck", "Bip01 Neck", Some(2), [0.0, 22.0, 0.0], None),
        j("Head", "Bip01 Head", Some(3), [0.0, 10.0, 0.0], Some([0.0, 15.0, 0.0])),
    ];
    for (bvh, bip, s) in [("Left", "L", 1.0), ("Right", "R", -1.0)] {
        let base = out.len();
        let name = |a: &str, b: &str| (format!("{bvh}{a}"), format!("Bip01 {bip} {b}"));
        let arm =
            [("Shoulder", "Clavicle", [4.0 * s, 18.0, 0.0]), ("Arm", "UpperArm", [14.0 * s, 0.0, 0.0]), ("ForeArm", "Forearm", [28.0 * s, 0.0, 0.0])];
        for (k, (a, b, o)) in arm.into_iter().enumerate() {
            let (n, m) = name(a, b);
            out.push(j(&n, &m, Some(if k == 0 { 2 } else { base + k - 1 }), o, None));
        }
        let (n, m) = name("Hand", "Hand");
        out.push(j(&n, &m, Some(base + 2), [25.0 * s, 0.0, 0.0], Some([10.0 * s, 0.0, 0.0])));
        let leg = [("UpLeg", "Thigh", [10.0 * s, -5.0, 0.0]), ("Leg", "Calf", [0.0, -43.0, 0.0]), ("Foot", "Foot", [0.0, -44.0, 0.0])];
        for (k, (a, b, o)) in leg.into_iter().enumerate() {
            let (n, m) = name(a, b);
            out.push(j(&n, &m, Some(if k == 0 { 0 } else { base + 4 + k - 1 }), o, None));
        }
        let (n, m) = name("ToeBase", "Toe0");
        out.push(j(&n, &m, Some(base + 6), [0.0, -6.0, 13.0], Some([0.0, 0.0, 8.0])));
    }
    out
}

const FPS: f32 = 60.0;
const FRAMES: usize = 31;

/// `(Z, X, Y)` rotation channels in degrees of joint `bvh` at time `t`.
fn angles(bvh: &str, t: f32) -> [f32; 3] {
    let s = (std::f32::consts::TAU * t).sin();
    let c = (std::f32::consts::TAU * t).cos();
    match bvh {
        "Hips" => [0.0, 5.0, 10.0 * s],
        "Spine" => [0.0, 0.0, 15.0 * s],
        "Chest" => [0.0, 10.0 * s, 0.0],
        "Head" => [0.0, 0.0, 20.0 * s],
        "LeftArm" => [-30.0 - 30.0 * s, 0.0, 0.0],
        "RightArm" => [50.0 * s, 0.0, 10.0],
        "LeftForeArm" => [0.0, 0.0, 20.0 + 20.0 * s],
        "LeftUpLeg" => [0.0, -30.0 * s, 0.0],
        "RightUpLeg" => [0.0, 30.0 * s, 5.0],
        "LeftLeg" | "RightLeg" => [0.0, 10.0 + 15.0 * (1.0 - c), 0.0],
        "LeftFoot" => [0.0, 15.0 * s, 0.0],
        _ => [0.0; 3],
    }
}

fn dfs(body: &[J], j: usize, order: &mut Vec<usize>) {
    order.push(j);
    for c in (0..body.len()).filter(|&c| body[c].parent == Some(j)) {
        dfs(body, c, order);
    }
}

fn bvh_text(body: &[J]) -> String {
    fn joint(body: &[J], j: usize, depth: usize, out: &mut String) {
        let pad = "  ".repeat(depth);
        let kind = if body[j].parent.is_none() { "ROOT" } else { "JOINT" };
        let o = body[j].offset;
        let channels = if body[j].parent.is_none() {
            "6 Xposition Yposition Zposition Zrotation Xrotation Yrotation"
        } else {
            "3 Zrotation Xrotation Yrotation"
        };
        let _ = writeln!(out, "{pad}{kind} {}\n{pad}{{\n{pad}  OFFSET {} {} {}\n{pad}  CHANNELS {channels}", body[j].bvh, o.x, o.y, o.z);
        for c in (0..body.len()).filter(|&c| body[c].parent == Some(j)) {
            joint(body, c, depth + 1, out);
        }
        if let Some(e) = body[j].end {
            let _ = writeln!(out, "{pad}  End Site\n{pad}  {{\n{pad}    OFFSET {} {} {}\n{pad}  }}", e.x, e.y, e.z);
        }
        let _ = writeln!(out, "{pad}}}");
    }
    let mut out = String::from("HIERARCHY\n");
    joint(body, 0, 0, &mut out);
    let _ = writeln!(out, "MOTION\nFrames: {FRAMES}\nFrame Time: {}", 1.0 / FPS);
    // Channel rows follow the hierarchy's depth-first order.
    let mut order = Vec::new();
    dfs(body, 0, &mut order);
    for f in 0..FRAMES {
        #[allow(clippy::cast_precision_loss)]
        let t = f as f32 / FPS;
        for &j in &order {
            if body[j].parent.is_none() {
                let p = Vec3::new(0.0, 100.0 + 3.0 * (2.0 * std::f32::consts::TAU * t).sin(), 120.0 * t);
                let _ = write!(out, "{} {} {} ", p.x, p.y, p.z);
            }
            let [z, x, y] = angles(&body[j].bvh, t);
            let _ = write!(out, "{z} {x} {y} ");
        }
        out.push('\n');
    }
    out
}

/// The Biped skeleton: same body in meters, turned to Z up facing −Y, with
/// arbitrary joint frames under an extra `Bip01` root.
fn biped(body: &[J]) -> Vec<SourceJoint> {
    let to_max = Quat::from_rotation_x(90f32.to_radians());
    let mut model_pos = vec![Vec3::ZERO; body.len()];
    for (i, j) in body.iter().enumerate() {
        model_pos[i] = j.parent.map_or(Vec3::ZERO, |p| model_pos[p]) + j.offset * 0.01;
    }
    #[allow(clippy::cast_precision_loss)]
    let frame = |i: usize| to_max * Quat::from_euler(EulerRot::XYZ, 0.3 * i as f32, -0.2 * i as f32, 0.1 * i as f32 + 0.5);
    let mut out = vec![SourceJoint { name: "Bip01".into(), parent: None, rest: Trs::from_rotation(to_max) }];
    for (i, j) in body.iter().enumerate() {
        let (pp, pq) = j.parent.map_or((Vec3::ZERO, to_max), |p| (to_max * model_pos[p], frame(p)));
        let rest = Trs::new(pq.inverse() * (to_max * model_pos[i] - pp), pq.inverse() * frame(i), Vec3::ONE);
        out.push(SourceJoint { name: j.bip.clone(), parent: Some(j.parent.map_or(0, |p| p + 1)), rest });
    }
    out
}

fn model(skeleton: &Skeleton, local: &[Trs]) -> Vec<Affine3A> {
    let mut mats: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        mats.push(skeleton.parent(j).map_or(m, |p| mats[p] * m));
    }
    mats
}

#[test]
fn bvh_retargets_onto_a_biped_rig_within_1_cm_and_1_degree() {
    let body = body();
    let text = bvh_text(&body);
    let src = BvhAnimImporter::default().import("byo/mocap/walk.bvh", text.as_bytes()).unwrap();
    assert_eq!(src.root_motion.as_deref(), Some("Hips"));
    let rules = SkeletonRules { root_motion_joint: src.root_motion.clone(), ..SkeletonRules::default() };
    let (source, _) = Skeleton::with_rules(src.skeleton.as_ref().unwrap(), &rules).unwrap();
    let (target, _) = Skeleton::new(&biped(&body)).unwrap();

    let (mut source_map, report) = auto_map(&source);
    assert!(report.missing_required.is_empty(), "{report:#?}");
    let (mut target_map, report) = auto_map(&target);
    assert!(report.missing_required.is_empty(), "{report:#?}");
    // Both name styles map every body joint to the same humanoid bone.
    let mut pairs: Vec<(HumanoidBone, usize, usize)> = Vec::new();
    for j in &body {
        let bone = source_map.bone(&j.bvh).unwrap_or_else(|| panic!("{} unmapped: {:?}", j.bvh, source_map.bones));
        assert_eq!(target_map.bone(&j.bip), Some(bone), "{} vs {}", j.bvh, j.bip);
        pairs.push((bone, source.find(&j.bvh).unwrap(), target.find(&j.bip).unwrap()));
    }
    let align = target_map.auto_align(&target);
    assert!((align.facing_correction_deg - 90.0).abs() < 0.01, "Z up → Y up: {align:?}");
    assert!(source_map.auto_align(&source).facing_correction_deg < 0.01);
    let source_rig = source_map.retarget_rig(&source, RetargetOptions { ik_legs: false }).unwrap();
    let target_rig = target_map.retarget_rig(&target, RetargetOptions::default()).unwrap();
    let retargeter = Retargeter::new(&source, &source_rig, &target, &target_rig).unwrap();
    assert!((retargeter.scale() - 1.0).abs() < 1e-4, "same proportions: {}", retargeter.scale());

    let clip = &src.clips[0];
    let bound = BoundSource::new(clip, &source);
    assert!((bound.native_rate().unwrap() - FPS).abs() < 0.01);
    let (mut s_local, mut t_local) = (vec![Trs::IDENTITY; source.len()], vec![Trs::IDENTITY; target.len()]);
    let (mut worst_cm, mut worst_deg) = (0.0f32, 0.0f32);
    for f in 0..FRAMES {
        #[allow(clippy::cast_precision_loss)]
        let t = f as f32 / FPS;
        bound.sample(t, source.rest(), &mut s_local);
        retargeter.pose(&s_local, &[], &mut t_local);
        let (sm, tm) = (model(&source, &s_local), model(&target, &t_local));
        let pos = |m: &[Affine3A], j: usize| Vec3::from(m[j].translation);
        for &(bone, s, t) in &pairs {
            let cm = pos(&sm, s).distance(pos(&tm, t)) * 100.0;
            worst_cm = worst_cm.max(cm);
            assert!(cm < 1.0, "frame {f} {bone:?}: {cm} cm");
        }
        // Limb segments: parent bone → bone.
        for &(bone, s, t) in &pairs {
            let Some(&(_, ps, pt)) = pairs.iter().find(|(b, _, _)| Some(*b) == bone.parent()) else { continue };
            let (ds, dt) = (pos(&sm, s) - pos(&sm, ps), pos(&tm, t) - pos(&tm, pt));
            let deg = ds.angle_between(dt).to_degrees();
            worst_deg = worst_deg.max(deg);
            assert!(deg < 1.0, "frame {f} {bone:?}: {deg}°");
        }
    }
    println!("BVH → Biped: worst {worst_cm:.3} cm, {worst_deg:.3}° over {FRAMES} frames, {} joints", pairs.len());
}
