//! The physics stage runs after the procedural ops (pose audit F4 / S5): a ragdoll fading in
//! over a walk with its leg ops on. Sanity asserts only; the foot's distance to the ragdoll's
//! foot is printed. `cargo test -p sf_anim_graph --all-features --lib physics -- --nocapture`.

use super::*;

/// Weight 0 → 1 over 0.3 s (the death / knockdown fade, `RagdollConfig::fade`) while the biped
/// walks onto a step with `SpeedMatch`, `StrideWarp` and `FootPlacement` on. Each update must equal the
/// per-joint local blend of a twin without physics (the animated pose with its ops, which never
/// see the ragdoll's pose) and the physics pose; joints at weight 1 show the physics pose exactly.
/// The ops only take the physics weight: `FootPlacement` and `StrideWarp` fade by `1 - w` over their legs, so
/// the twin's ops are gated by a parameter set to `1 - w` (the same weight goals).
#[test]
fn physics_blends_after_the_ops() {
    let ops = r#"SpeedMatch(), StrideWarp(pelvis: Joint("hips"), legs: ["l", "r"]GATE), FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"]GATE)"#;
    let rig = proc_rig(&ops.replace("GATE", ""));
    let twin_rig = proc_rig(&ops.replace("GATE", r#", gate: "hand""#));
    let (b, tb) = (rig.binding(), twin_rig.binding());
    let gate = twin_rig.graph.param("hand").unwrap();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let (mut twin, mut twin_out) = (GraphInstance::new(&tb), GraphOutput::new(&twin_rig.graph));
    let up = Vec3::Y;
    let ground = [Some(GroundSample { point: Vec3::new(0.0, 0.2, 0.0), normal: up }), Some(GroundSample { point: Vec3::new(0.0, 0.15, 0.0), normal: up })];
    let dt = 1.0 / 60.0;
    let velocity = Vec3::Z * 1.5;
    let inputs = UpdateInputs { ground: &ground, moved: RootDelta::new(velocity * dt, 0.0), velocity, ..UpdateInputs::default() };
    // A "ragdoll": the rest pose slumped forward at the spine and the hips, knees bent.
    let rest = rig.skeleton.rest().to_vec();
    let bend = |name: &str, a: f32| (rig.joint(name), Quat::from_rotation_x(a));
    let bends = [bend("hips", 0.6), bend("spine", 0.4), bend("shin_l", 0.9), bend("shin_r", 0.5)];
    let physics: Vec<Trs> = rest
        .iter()
        .enumerate()
        .map(|(j, &x)| match bends.iter().find(|(k, _)| *k == j) {
            Some(&(_, q)) => Trs { rotation: (x.rotation * q).normalize(), translation: x.translation - Vec3::Y * 0.4 * f32::from(j == rig.joint("hips")), ..x },
            None => x,
        })
        .collect();
    let mut phys_model = sf_anim::ModelPose::new(rest.len());
    let mut phys_local = LocalPose::new(rest.len());
    for (j, &t) in physics.iter().enumerate() {
        phys_local.set(j, t);
    }
    sf_anim::local_to_model(&rig.skeleton, &phys_local, &mut phys_model, &glam::Affine3A::IDENTITY);
    let (foot, hand) = (rig.joint("foot_l"), rig.joint("hand_l"));
    let mut model = sf_anim::ModelPose::new(rest.len());
    let (warm, fade) = (60usize, 18usize); // 1 s of walking, then 0.3 s at 60 Hz
    let mut checked_half = false;
    for k in 0..warm + fade + 12 {
        let w = (k.saturating_sub(warm) as f32 / fade as f32).min(1.0);
        // The left hand is owned by physics from the start, at weight 1.
        let weights: Vec<f32> = (0..rest.len()).map(|j| if j == hand { 1.0 } else { w }).collect();
        inst.set_physics(Some((&physics, &weights)));
        inst.update(&b, dt, 0, &inputs, &mut out);
        // The hand at weight 1 is no leg joint: the feet don't fade for it.
        twin.params.set_float(gate, 1.0 - w);
        twin.update(&tb, dt, 0, &inputs, &mut twin_out);
        for (j, (&wj, &p)) in weights.iter().zip(&physics).enumerate() {
            let (got, anim) = (out.pose.get(j), twin_out.pose.get(j));
            assert!(got.translation.is_finite() && got.rotation.is_finite(), "{}: not finite", rig.skeleton.name(j));
            if wj >= 1.0 {
                assert_eq!(got, p, "{}: weight 1 is not the physics pose", rig.skeleton.name(j));
            } else {
                let want = Trs {
                    translation: anim.translation.lerp(p.translation, wj),
                    rotation: anim.rotation.slerp(p.rotation, wj).normalize(),
                    scale: anim.scale.lerp(p.scale, wj),
                };
                // Quaternion components, not `angle_between`: its acos near 1 is ~1e-3 rad noisy in f32.
                let (dt_, dq) = (got.translation.distance(want.translation), (got.rotation - want.rotation).length().min((got.rotation + want.rotation).length()));
                assert!(dt_ < 1e-5 && dq < 1e-5, "{} at w {wj}: {dt_} m, {dq} quat off the blend of the animated pose with its ops", rig.skeleton.name(j));
            }
        }
        checked_half |= (w - 0.5).abs() < 1e-6;
        // The model pose follows the final local pose.
        sf_anim::local_to_model(&rig.skeleton, &out.pose, &mut model, &glam::Affine3A::IDENTITY);
        assert!(out.model.translation(foot).distance(model.translation(foot)) < 1e-5, "stale model pose");
        if k >= warm {
            let d = out.model.translation(foot).distance(phys_model.translation(foot));
            let ops = out.model.translation(foot).distance(twin_out.model.translation(foot));
            println!("t {:5.3} s  w {w:4.2}  foot to ragdoll foot {:6.3} m  to animated+ops foot {:6.3} m", (k - warm) as f32 * dt, d, ops);
        }
    }
    assert!(checked_half, "the fade never hit w = 0.5");
}
