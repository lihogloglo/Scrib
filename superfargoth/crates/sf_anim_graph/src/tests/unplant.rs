//! `FootPlacement` turn unplant (P-IK5, UE Foot Placement plant settings' unplant angle): a
//! planted foot under a body yawing past `guards.unplant_angle` releases, below it holds, and the
//! release fades (no pop, nothing non-finite).
//! `cargo test -p sf_anim_graph --all-features --lib unplant -- --nocapture`.

use super::*;

const DT: f32 = 1.0 / 60.0;
const FLAT: [Option<GroundSample>; 2] = [Some(GroundSample { point: Vec3::ZERO, normal: Vec3::Y }); 2];

/// One update: the left foot's character-space offset from the animated one (a twin without the
/// op), the contact curve and the stats.
struct Frame {
    deviation: Vec3,
    contact: f32,
    stats: FootStats,
}

/// Walks the fixture cycle at its own speed (2 cm per update, as the foot-lock test) while the
/// root turns by `step` radians per update for updates `1..=frames`, with `ops` as the
/// procedural stack.
fn turn(ops: &str, step: f32, frames: usize) -> Vec<Frame> {
    let (rig, plain) = (proc_rig(ops), proc_rig(""));
    let (b, pb) = (rig.binding(), plain.binding());
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let (mut pinst, mut pout) = (GraphInstance::new(&pb), GraphOutput::new(&plain.graph));
    let fl = rig.joint("foot_l");
    let contact = rig.graph.curve("contact.foot_l").unwrap();
    (0..15)
        .map(|k| {
            let moved = RootDelta::new(Vec3::new(0.0, 0.0, 0.02), if k > 0 && k <= frames { step } else { 0.0 });
            inst.update(&b, DT, 0, &UpdateInputs { ground: &FLAT, moved, ..UpdateInputs::default() }, &mut out);
            pinst.update(&pb, DT, 0, &UpdateInputs::default(), &mut pout);
            for j in 0..rig.skeleton.len() {
                assert!(out.model.translation(j).is_finite(), "update {k}: joint {j} not finite");
            }
            Frame { deviation: out.model.translation(fl) - pout.model.translation(fl), contact: out.curves[contact], stats: out.feet }
        })
        .collect()
}

#[test]
fn planted_foot_releases_once_the_body_turns_past_the_unplant_angle() {
    // 720 deg/s (the player's walk turn) at 60 Hz: 12° per update. The left foot plants on the
    // first update (its contact curve is up for 0.32 s, longer than this test's turn).
    let step = 12f32.to_radians();
    let feet = r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"], guards: (unplant_angle: 0.7853982))"#;
    let run = turn(feet, step, 10);
    assert!(run.iter().all(|f| f.contact > 0.5), "the contact holds through the test");
    // Released on the first update the turn since the plant passes 45°: 4 × 12° = 48°; the drift
    // limit (0.3 of the reach) is not what releases it.
    let first = run.iter().position(|f| f.stats.unplanted_turn > 0).expect("a turn release");
    assert_eq!(first, 4, "released at update {first}");
    assert_eq!(run[first].stats.unplanted, 1, "{:?}", run[first].stats);
    assert!(run[..first].iter().all(|f| f.stats.unplanted == 0 && f.stats.locks == 1));
    // No pop: the release fades over `release / 2`, never more than half the held offset in one
    // update, and it goes (no re-lock while the turn goes on).
    let held = run[first - 1].deviation.length();
    let steps: Vec<f32> = (first..run.len()).map(|k| run[k].deviation.distance(run[k - 1].deviation)).collect();
    println!("held {held:.3} m, deviation change per update from the release: {steps:.3?}");
    assert!(held > 0.02, "the lock held the foot off the animation ({held} m)");
    assert!(steps.iter().all(|&s| s <= 0.5 * held), "release pops: {steps:?} (held {held})");
    assert!(run.last().unwrap().deviation.length() < 0.25 * held, "the release fades: {:?}", run.last().unwrap().deviation);

    // 40° in all (below the angle): the turn never releases the lock.
    let kept = turn(feet, 10f32.to_radians(), 4);
    assert!(kept.iter().all(|f| f.stats.unplanted_turn == 0), "{:?}", kept.last().map(|f| f.stats));
    assert!(kept[..=first].iter().all(|f| f.stats.unplanted == 0 && f.stats.locks == 1), "{:?}", kept[first].stats);

    // The limit off (more than any turn here): the drift limit alone, as before P-IK5.
    let off = turn(r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"], guards: (unplant_angle: 100.0))"#, step, 10);
    assert!(off.iter().all(|f| f.stats.unplanted_turn == 0));
}
