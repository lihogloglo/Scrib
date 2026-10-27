//! Continuous stride warp (pose audit F6 + F8): `StrideWarp`'s weight and ratio springs carry a
//! state without a `Match` player (a jump), and the foot ops fade out past their LOD cut-off
//! instead of resetting the same update. Sanity asserts only; the per-frame curves are printed.
//! `cargo test -p sf_anim_graph --all-features --lib stride_warp_eases -- --nocapture`.

use super::*;

const DT: f32 = 1.0 / 60.0;
/// 1.5 × the walk's 1.2 m/s; `SpeedMatch(max: 1.0)` leaves all of it to the stride.
const SPEED: f32 = 1.8;
const MATCH: &str = "SpeedMatch(max: 1.0)";
const STRIDE: &str = r#"StrideWarp(pelvis: Joint("hips"), legs: ["l", "r"])"#;

/// The procedural fixture with a walk state (`Match`) and an "air" state playing the walk in
/// place (`Ignore`: no stride ratio), switched by the `air` parameter, inertialized.
fn machine_rig(ops: &str) -> Rig {
    let ron = PROC
        .replace(r#"params: [(name: "hand", kind: Float(1.0))]"#, r#"params: [(name: "hand", kind: Float(1.0)), (name: "air", kind: Bool(false))]"#)
        .replace(
            r#"nodes: {"w": Clip(clip: "walk")}"#,
            r#"nodes: {
                "sm": StateMachine(states: [(name: "walk", node: "w"), (name: "air", node: "j")], transitions: [
                    (from: "walk", to: "air", when: [IsTrue("air")], blend: Inertialize(0.2)),
                    (from: "air", to: "walk", when: [IsFalse("air")], blend: Inertialize(0.2)),
                ]),
                "w": Clip(clip: "walk"),
                "j": Clip(clip: "walk", root_motion: Ignore),
            }"#,
        )
        .replace(r#"layers: [(node: "w")]"#, r#"layers: [(node: "sm")]"#);
    assert!(ron.contains("\"sm\"") && ron.contains("\"air\"") && ron.contains("(node: \"sm\")"), "fixture changed");
    Rig::new(&ron.replace("OPS", ops))
}

/// Largest change of a per-frame series over `range` (frames), of all its values.
fn max_over(series: &[f32], range: std::ops::Range<usize>) -> f32 {
    series[range].iter().fold(0.0f32, |m, &v| m.max(v))
}

/// Walk at 1.5× with the stride warp, switch to a state without a `Match` player (60) and back
/// (120). The warp's foot displacement (ops foot - plain foot) may change no faster around
/// the switches than while walking (10%: no one-frame jump); `blend_halflife: 0` (the old behavior) jumps.
#[test]
fn stride_warp_eases_across_a_state_without_match() {
    let plain = machine_rig(MATCH);
    let smooth = machine_rig(&format!("{MATCH}, {STRIDE}"));
    let instant = machine_rig(&format!(r#"{MATCH}, StrideWarp(pelvis: Joint("hips"), legs: ["l", "r"], blend_halflife: 0.0)"#));
    let rigs = [&plain, &smooth, &instant];
    let binds: Vec<Binding<'_>> = rigs.iter().map(|r| r.binding()).collect();
    let mut insts: Vec<GraphInstance> = binds.iter().map(GraphInstance::new).collect();
    let mut outs: Vec<GraphOutput> = rigs.iter().map(|r| GraphOutput::new(&r.graph)).collect();
    let feet = [plain.joint("foot_l"), plain.joint("foot_r")];
    let velocity = Vec3::Z * SPEED;
    let inputs = UpdateInputs { moved: RootDelta::new(velocity * DT, 0.0), velocity, ..UpdateInputs::default() };
    let (to_air, to_walk, end) = (60usize, 120usize, 180usize);
    // Per variant (smooth, instant): the displacement last frame and its per-frame change.
    let mut last = [[Vec3::ZERO; 2]; 2];
    let mut change = [vec![0.0f32; end], vec![0.0f32; end]];
    let mut feet_last = [[Vec3::ZERO; 2]; 2];
    let mut feet_step = [vec![0.0f32; end], vec![0.0f32; end]];
    println!("  k  air  plain foot step  ops foot step  disp change smooth  instant (mm)");
    for k in 0..end {
        for (i, r) in rigs.iter().enumerate() {
            insts[i].params.set_bool(r.graph.param("air").unwrap(), (to_air..to_walk).contains(&k));
            insts[i].update(&binds[i], DT, 0, &inputs, &mut outs[i]);
        }
        for v in 0..2 {
            let d = feet.map(|f| outs[v + 1].model.translation(f) - outs[0].model.translation(f));
            change[v][k] = if k == 0 { 0.0 } else { d.iter().zip(&last[v]).fold(0.0f32, |m, (a, b)| m.max(a.distance(*b))) };
            last[v] = d;
            let p = feet.map(|f| outs[v].model.translation(f));
            feet_step[v][k] = if k == 0 { 0.0 } else { p.iter().zip(&feet_last[v]).fold(0.0f32, |m, (a, b)| m.max(a.distance(*b))) };
            feet_last[v] = p;
        }
        if (to_air - 3..to_air + 12).contains(&k) || (to_walk - 3..to_walk + 12).contains(&k) {
            let air = u8::from((to_air..to_walk).contains(&k));
            println!(
                "{k:3}  {air}    {:7.2}          {:7.2}        {:7.2}             {:7.2}",
                feet_step[0][k] * 1e3,
                feet_step[1][k] * 1e3,
                change[0][k] * 1e3,
                change[1][k] * 1e3
            );
        }
    }
    let steady = max_over(&change[0], 20..to_air);
    let switch = max_over(&change[0], to_air..to_air + 30).max(max_over(&change[0], to_walk..to_walk + 30));
    let jump = max_over(&change[1], to_air..to_air + 2).max(max_over(&change[1], to_walk..to_walk + 2));
    println!("displacement change: walking {:.2} mm/frame; around the switches {:.2} (smooth), {:.2} (instant, old)", steady * 1e3, switch * 1e3, jump * 1e3);
    assert!(steady > 0.0 && switch.is_finite(), "no warp");
    // The displacement itself swings with the stride (and the switch restarts the clip under
    // inertialization): within 10% of walking's rate, no one-frame jump.
    assert!(switch <= 1.1 * steady + 1e-3, "the warp changes faster around the switch ({switch} m) than walking ({steady} m)");
    assert!(jump > 4.0 * steady, "the instant variant should jump ({jump} m vs {steady} m): test lost its point");
}

/// Walk at 1.5× onto a step with `StrideWarp` and `FootPlacement` (both cut off past LOD 1),
/// go to LOD 2 at frame 60 and back to 1 at 150. The ops' hips offset (ops hips - plain hips)
/// fades out and back in: no frame changes it faster than a critical spring of the blend
/// half-life from its value at the cut-off, and the ops reset before the LOD comes back.
#[test]
fn stride_warp_eases_out_past_the_lod_cut_off() {
    let plain = proc_rig(MATCH);
    let ops = proc_rig(&format!(r#"{MATCH}, {STRIDE}, FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"])"#));
    let (pb, ob) = (plain.binding(), ops.binding());
    let (mut pi, mut po, mut oi, mut oo) = (GraphInstance::new(&pb), GraphOutput::new(&plain.graph), GraphInstance::new(&ob), GraphOutput::new(&ops.graph));
    let hips = plain.joint("hips");
    let up = Vec3::Y;
    let ground = [Some(GroundSample { point: Vec3::ZERO, normal: up }), Some(GroundSample { point: Vec3::new(0.0, 0.15, 0.0), normal: up })];
    let velocity = Vec3::Z * SPEED;
    let inputs = UpdateInputs { ground: &ground, moved: RootDelta::new(velocity * DT, 0.0), velocity, ..UpdateInputs::default() };
    let (up_at, down_at, end) = (60usize, 150usize, 240usize);
    let mut offset = vec![Vec3::ZERO; end];
    let mut change = vec![0.0f32; end];
    let mut weight = vec![0.0f32; end];
    println!("  k  lod  hips offset (mm)  change (mm)  foot weight");
    for k in 0..end {
        let lod = if (up_at..down_at).contains(&k) { 2 } else { 1 };
        pi.update(&pb, DT, lod, &inputs, &mut po);
        oi.update(&ob, DT, lod, &inputs, &mut oo);
        offset[k] = oo.model.translation(hips) - po.model.translation(hips);
        change[k] = if k == 0 { 0.0 } else { offset[k].distance(offset[k - 1]) };
        weight[k] = oo.feet.weight_max;
        if (up_at - 2..up_at + 40).step_by(2).any(|f| f == k) || (down_at - 2..down_at + 20).step_by(2).any(|f| f == k) || k == down_at - 1 {
            println!("{k:3}  {lod}    {:8.2}          {:6.2}       {:.3}", offset[k].length() * 1e3, change[k] * 1e3, weight[k]);
        }
    }
    let at_cut = offset[up_at - 1].length();
    let steady = max_over(&change, 20..up_at);
    // A critical spring from x0 moves at most 0.51 x0 / halflife per second (halflife 0.1 s).
    let bound = 0.6 * at_cut * DT / 0.1 + steady + 5e-4;
    let worst = max_over(&change, up_at..end);
    println!("hips offset at the cut-off {:.2} mm; largest change after it {:.2} mm/frame (bound {:.2}, walking {:.2})", at_cut * 1e3, worst * 1e3, bound * 1e3, steady * 1e3);
    assert!(at_cut > 5e-3, "the ops barely moved the hips ({at_cut} m): test lost its point");
    assert!(worst <= bound, "the hips offset jumped {worst} m in a frame (bound {bound} m)");
    assert!(offset[down_at - 1].length() < 1e-3 && weight[down_at - 1] < 1e-3, "the ops didn't fade out and reset at LOD 2");
    assert!(weight[down_at] < 0.5 && weight[end - 1] > 0.99, "the feet should fade back in");
}
