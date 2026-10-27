//! `FootPlacement` weight gating ("IK policy as data", pose audit F5 / S4): a gate parameter,
//! the physics share of its legs, and no new lock while a hand-back from physics inertializes.
//! Sanity asserts only; the weight curves are printed.
//! `cargo test -p sf_anim_graph --all-features --lib gates -- --nocapture`.

use super::*;
use crate::asset::FootGuards;

const FEET: &str = r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"], gate: "ik")"#;

/// The procedural rig with a `Bool` parameter `ik` gating its `FootPlacement`.
fn gated_rig() -> Rig {
    let ron = PROC.replace(r#"params: [(name: "hand", kind: Float(1.0))]"#, r#"params: [(name: "hand", kind: Float(1.0)), (name: "ik", kind: Bool(true))]"#);
    assert!(ron.contains("\"ik\""), "fixture params changed");
    Rig::new(&ron.replace("OPS", FEET))
}

const DT: f32 = 1.0 / 60.0;
const FLAT: [Option<GroundSample>; 2] = [Some(GroundSample { point: Vec3::ZERO, normal: Vec3::Y }); 2];

/// One update walking 2 cm per frame (the walk's 1.2 m/s) on flat ground.
fn walk(rig: &Rig, inst: &mut GraphInstance, out: &mut GraphOutput) {
    let inputs = UpdateInputs { ground: &FLAT, moved: RootDelta::new(Vec3::new(0.0, 0.0, 0.02), 0.0), ..UpdateInputs::default() };
    inst.update(&rig.binding(), DT, 0, &inputs, out);
}

/// A splayed ragdoll stance: thighs spread sideways, knees bent, hips low.
fn splayed(rig: &Rig) -> Vec<Trs> {
    let bends = [("thigh_l", Quat::from_rotation_z(-0.6)), ("thigh_r", Quat::from_rotation_z(0.6)), ("shin_l", Quat::from_rotation_x(0.8)), ("shin_r", Quat::from_rotation_x(0.5))];
    let hips = rig.joint("hips");
    rig.skeleton
        .rest()
        .iter()
        .enumerate()
        .map(|(j, &x)| {
            let q = bends.iter().find(|(n, _)| rig.joint(n) == j).map_or(Quat::IDENTITY, |&(_, q)| q);
            Trs { rotation: (x.rotation * q).normalize(), translation: x.translation - Vec3::Y * 0.3 * f32::from(j == hips), ..x }
        })
        .collect()
}

#[test]
fn gate_param_fades_the_feet_and_drops_their_locks() {
    let rig = gated_rig();
    let b = rig.binding();
    let ik = rig.graph.param("ik").unwrap();
    let halflife = FootGuards::default().blend_halflife;
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let mut locked = 0;
    for _ in 0..60 {
        walk(&rig, &mut inst, &mut out);
        locked += out.feet.locks;
    }
    assert!(out.feet.weight_max > 0.99 && locked > 0, "gate on: weight {} locks {locked}", out.feet.weight_max);
    inst.params.set_bool(ik, false);
    let mut curve = Vec::new();
    for k in 0..60 {
        walk(&rig, &mut inst, &mut out);
        curve.push(out.feet.weight_max);
        let t = (k + 1) as f32 * DT;
        // A critical spring: ~0 within a few halflives; below 1e-3 the op resets its locks.
        if t > 5.0 * halflife {
            assert!(out.feet.weight_max < 0.02, "t {t}: weight {}", out.feet.weight_max);
        }
        if t > 8.0 * halflife {
            assert!(out.feet.weight_max < 1e-3 && out.feet.locks == 0, "t {t}: weight {} locks {}", out.feet.weight_max, out.feet.locks);
        }
    }
    println!("gate off (halflife {halflife} s), weight every 3 updates: {:?}", curve.iter().step_by(3).map(|w| (w * 1000.0).round() / 1000.0).collect::<Vec<_>>());
    // Gated off from the first update: the op never runs, no lock ever.
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    inst.params.set_bool(ik, false);
    for _ in 0..60 {
        walk(&rig, &mut inst, &mut out);
        assert!(out.feet.weight_max == 0.0 && out.feet.locks == 0, "{:?}", out.feet);
    }
}

/// A ragdoll owning the legs (weight 1) for 0.5 s, then handed back without an
/// inertialization: the op restarts at weight 0 and fades in over its halflife.
/// A half-weight ragdoll halves the weight goal.
#[test]
fn physics_on_the_legs_fades_and_resets_the_feet() {
    let rig = gated_rig();
    let b = rig.binding();
    let halflife = FootGuards::default().blend_halflife;
    let pose = splayed(&rig);
    let n = pose.len();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    for _ in 0..60 {
        walk(&rig, &mut inst, &mut out);
    }
    let full = vec![1.0; n];
    inst.set_physics(Some((&pose, &full)));
    for _ in 0..30 {
        walk(&rig, &mut inst, &mut out);
        assert!(out.feet.weight_max == 0.0 && out.feet.locks == 0, "owned by physics: {:?}", out.feet);
    }
    inst.set_physics(None);
    let mut curve = Vec::new();
    for _ in 0..30 {
        walk(&rig, &mut inst, &mut out);
        curve.push(out.feet.weight_max);
    }
    println!("hand-back (halflife {halflife} s), weight every 2 updates: {:?}", curve.iter().step_by(2).map(|w| (w * 1000.0).round() / 1000.0).collect::<Vec<_>>());
    assert!(curve[0] < 0.05, "first update after the hand-back: weight {}", curve[0]);
    assert!(curve.windows(2).all(|w| w[1] >= w[0] - 1e-6), "the weight fades in monotonically");
    let at = |t: f32| curve[((t / DT).round() as usize).saturating_sub(1)];
    assert!(at(halflife) > 0.2 && at(halflife) < 0.9 && at(5.0 * halflife) > 0.9, "fade-in: {} at 1 halflife, {} at 5", at(halflife), at(5.0 * halflife));

    // Half the legs' weight: the op settles at half weight.
    let half = vec![0.5; n];
    inst.set_physics(Some((&pose, &half)));
    for _ in 0..60 {
        walk(&rig, &mut inst, &mut out);
    }
    println!("physics 0.5 on the legs: weight {:.3}", out.feet.weight_max);
    assert!((out.feet.weight_max - 0.5).abs() < 0.05, "weight {}", out.feet.weight_max);
}

/// A get-up: physics owns the body, then hands back with `inertialize_from` its splayed
/// stance. While the blend runs no foot plants (the blending foot is still splayed); after it
/// ends the left foot locks at its next contact.
#[test]
fn no_new_lock_while_a_hand_back_inertializes() {
    let rig = gated_rig();
    let b = rig.binding();
    let pose = splayed(&rig);
    let full = vec![1.0; pose.len()];
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    for _ in 0..30 {
        walk(&rig, &mut inst, &mut out);
    }
    inst.set_physics(Some((&pose, &full)));
    for _ in 0..30 {
        walk(&rig, &mut inst, &mut out);
    }
    inst.set_physics(None);
    let duration = 0.5;
    inst.inertialize_from(&pose, &pose, DT, duration);
    let contact = rig.graph.curve("contact.foot_l").unwrap();
    let (mut blended, mut ended, mut first_lock, mut contact_while_blending) = (0usize, None, None, 0usize);
    for k in 0..150 {
        walk(&rig, &mut inst, &mut out);
        if inst.inertializing() {
            blended += 1;
            contact_while_blending += usize::from(out.curves[contact] > 0.5 && out.feet.weight_max > 1e-3);
            assert_eq!(out.feet.locks, 0, "update {k}: a lock while the hand-back blends");
        } else {
            ended.get_or_insert(k);
            if out.feet.locks > 0 {
                first_lock.get_or_insert(k);
            }
        }
    }
    println!(
        "hand-back blend: {blended} updates inertializing ({contact_while_blending} with the contact up and the op on), ended at update {ended:?}, first lock at {first_lock:?}"
    );
    assert!(blended > 0 && contact_while_blending > 0, "the test never had a contact during the blend");
    assert!(first_lock.is_some(), "no lock after the blend ended");
}
