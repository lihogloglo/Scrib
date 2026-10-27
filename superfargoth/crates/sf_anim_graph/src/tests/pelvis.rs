//! Pelvis bob measurement (pose audit S1): the hips' model-space height with the leg ops on minus
//! off, on flat ground, split by op. Prints numbers; asserts sanity only. Run with
//! `cargo test -p sf_anim_graph --all-features --lib pelvis -- --ignored --nocapture`.

use std::f32::consts::PI;

use super::*;

/// The fixture walk (`cycle`: 1 s, 1.2 m/s, 3 cm hip bob) with a contact curve on each foot:
/// left down for 0..0.4 s, right for 0.5..0.9 s. Its legs swing on sines, so the ankle lifts
/// while the contact holds (it has no real stance).
fn swing_walk(s: &Skeleton) -> Clip {
    let (d, speed, swing) = (1.0f32, NATURAL, 0.5f32);
    let w = move |t: f32| (TAU * t / d).sin();
    let src = SourceClip {
        name: "swing".into(),
        duration: d,
        tracks: vec![
            trans(d, "root", move |t| Vec3::new(0.0, 0.0, speed * t)),
            trans(d, "hips", move |t| Vec3::new(0.0, 1.0 + 0.03 * (2.0 * TAU * t / d).cos(), 0.0)),
            rot(d, "spine", move |t| Quat::from_rotation_x(0.1 * w(t))),
            rot(d, "thigh_l", move |t| Quat::from_rotation_x(swing * w(t))),
            rot(d, "shin_l", move |t| Quat::from_rotation_x(0.4 * (1.0 - (TAU * t / d).cos()))),
            rot(d, "thigh_r", move |t| Quat::from_rotation_x(-swing * w(t))),
            rot(d, "shin_r", move |t| Quat::from_rotation_x(0.4 * (1.0 + (TAU * t / d).cos()))),
        ],
        curves: vec![
            SourceCurve { name: "contact.foot_l".into(), curve: sampled(d, |t| if t < 0.4 { 1.0 } else { 0.0 }) },
            SourceCurve { name: "contact.foot_r".into(), curve: sampled(d, |t| if (0.5..0.9).contains(&t) { 1.0 } else { 0.0 }) },
        ],
        markers: vec![marker(0.0, "down.l"), marker(0.5, "down.r")],
        ..SourceClip::default()
    };
    cook_clip(s, &src, &rules(true, RootMotionMode::Match), &CookSettings::default()).unwrap()
}

/// Ankle path of a foot at cycle phase `p` (0..1, stance first), root space: stance for half
/// the cycle sliding back at the root speed at a constant 8 cm, then `rise` m of heel-off over
/// the second half of stance; the swing arcs 10 cm up and forward.
fn ankle(p: f32, rise: f32) -> (f32, f32) {
    let smooth = |x: f32| x * x * (3.0 - 2.0 * x);
    if p < 0.5 {
        (0.08 + rise * smooth(((p - 0.25) / 0.25).clamp(0.0, 1.0)), 0.3 - NATURAL * p)
    } else {
        let s = (p - 0.5) / 0.5;
        (0.08 + rise * (1.0 - smooth(s)) + 0.1 * (PI * s).sin(), -0.3 + 0.6 * smooth(s))
    }
}

/// Thigh and shin X rotations putting the biped's ankle at `(y, z)` below a hip joint at `hip_y`
/// (analytic two-bone, sagittal plane, knee bending forward).
fn leg_ik(hip_y: f32, (y, z): (f32, f32)) -> (f32, f32) {
    let (a, b) = (Vec2::new(-0.45, 0.02), Vec2::new(-0.45, -0.02)); // (y, z) rest offsets
    let t = Vec2::new(y - hip_y, z);
    let d = t.length().min(a.length() + b.length() - 1e-4);
    let (aa, bb) = (a.dot(b), a.y * b.x - a.x * b.y);
    let c = (d * d - a.length_squared() - b.length_squared()) / 2.0;
    let r = (aa * aa + bb * bb).sqrt();
    let knee = bb.atan2(aa) + (c / r).clamp(-1.0, 1.0).acos();
    let v = a + Vec2::new(b.x * knee.cos() - b.y * knee.sin(), b.x * knee.sin() + b.y * knee.cos());
    let thigh = (v.x * t.y - v.y * t.x).atan2(v.dot(t));
    (thigh, knee)
}

/// A walk with planted stances (1 s, 1.2 m/s): each ankle flat while its contact holds
/// (`rise` m of heel-off in its second half), hips 3 cm lowest at double support.
fn planted_walk(s: &Skeleton, name: &str, rise: f32) -> Clip {
    let d = 1.0f32;
    let hips = |t: f32| 0.98 - 0.015 * (2.0 * TAU * t).cos();
    let leg = move |t: f32, shift: f32| leg_ik(hips(t) - 0.05, ankle((t + shift).rem_euclid(1.0), rise));
    let src = SourceClip {
        name: name.into(),
        duration: d,
        tracks: vec![
            trans(d, "root", |t| Vec3::new(0.0, 0.0, NATURAL * t)),
            trans(d, "hips", move |t| Vec3::new(0.0, hips(t), 0.0)),
            rot(d, "thigh_l", move |t| Quat::from_rotation_x(leg(t, 0.0).0)),
            rot(d, "shin_l", move |t| Quat::from_rotation_x(leg(t, 0.0).1)),
            rot(d, "thigh_r", move |t| Quat::from_rotation_x(leg(t, 0.5).0)),
            rot(d, "shin_r", move |t| Quat::from_rotation_x(leg(t, 0.5).1)),
        ],
        curves: vec![
            SourceCurve { name: "contact.foot_l".into(), curve: sampled(d, |t| if t < 0.45 { 1.0 } else { 0.0 }) },
            SourceCurve { name: "contact.foot_r".into(), curve: sampled(d, |t| if (0.5..0.95).contains(&t) { 1.0 } else { 0.0 }) },
        ],
        markers: vec![marker(0.0, "down.l"), marker(0.5, "down.r")],
        ..SourceClip::default()
    };
    cook_clip(s, &src, &rules(true, RootMotionMode::Match), &CookSettings::default()).unwrap()
}

const GRAPH: &str = r#"(
    nodes: {"w": Clip(clip: "CLIP")},
    layers: [(node: "w")],
    legs: [
        (name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l"), contact: "contact.foot_l"),
        (name: "r", hip: Joint("thigh_r"), knee: Joint("shin_r"), foot: Joint("foot_r"), contact: "contact.foot_r"),
    ],
    procedural: [OPS],
)"#;

/// The humanoid's data (mw_humanoid.animgraph.ron:307-309): rate takes `ratio^0.3`, the stride the rest.
const SPEED: &str = "SpeedMatch(min: 0.5, max: 2.2, rate_share: 0.3)";
const STRIDE: &str = r#"StrideWarp(pelvis: Joint("hips"), legs: ["l", "r"], min: 0.6, max: 1.6)"#;
const FEET: &str = r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"], body: Lower)"#;
const NATURAL: f32 = 1.2;

fn rig(clip: &str, ops: &str) -> Rig {
    let skeleton = biped();
    let mut src = source(&skeleton);
    for c in [swing_walk(&skeleton), planted_walk(&skeleton, "planted", 0.0), planted_walk(&skeleton, "heeloff", 0.02)] {
        src.clips.insert(c.name.clone(), c);
    }
    let graph = compile_str("pelvis", &GRAPH.replace("CLIP", clip).replace("OPS", ops), &skeleton, &mut src).unwrap();
    let clips = graph.clips.iter().map(|c| Arc::new(src.clips[&c.reference].clone())).collect();
    Rig { skeleton, graph, clips }
}

#[derive(Default)]
struct Offset {
    sum2: f32,
    peak: f32,
    step: f32,
    n: u32,
    prev: Option<f32>,
}

impl Offset {
    fn add(&mut self, o: f32) {
        assert!(o.is_finite() && o.abs() < 0.5, "pelvis offset {o}");
        self.sum2 += o * o;
        self.peak = self.peak.max(o.abs());
        if let Some(p) = self.prev {
            self.step = self.step.max((o - p).abs());
        }
        self.prev = Some(o);
        self.n += 1;
    }
    fn rms(&self) -> f32 {
        (self.sum2 / self.n.max(1) as f32).sqrt()
    }
}

/// One fixture: a row per update rate and speed ratio.
fn measure(clip: &str) {
    let flat = [Some(GroundSample { point: Vec3::ZERO, normal: Vec3::Y }); 2];
    let mut ankle = [(f32::INFINITY, f32::NEG_INFINITY); 2];
    println!("\n== clip `{clip}` ==");
    println!(
        "{:>3} {:>3} {:>5} | {:^19} | {:^19} | {:^19} | {:>5} {:>5} {:>7} {:>7} {:>4} {:>5} {:>5} {:>5}",
        "hz", "spd", "strd", "StrideWarp only", "FootPlacement only", "both", "drFP", "drift", "capFP", "capBoth", "unpl", "sdrop", "flSW", "flBo"
    );
    for hz in [30.0f32, 60.0, 144.0] {
        let dt = 1.0 / hz;
        for ratio in [1.0f32, 1.2, 1.4, 1.6] {
            // The playback rate SpeedMatch picks with the stride warp running; the runs without
            // it are pinned to the same rate, so every run plays the same clip time.
            let m = ratio.powf(0.3).clamp(ratio / 1.6, ratio / 0.6).clamp(0.5, 2.2);
            let fixed = format!("SpeedMatch(min: {m}, max: {m})");
            let cases = [format!("{SPEED}, {STRIDE}"), format!("{fixed}, {FEET}"), format!("{SPEED}, {STRIDE}, {FEET}")];
            let base = rig(clip, &fixed);
            let rigs: Vec<Rig> = cases.iter().map(|ops| rig(clip, ops)).collect();
            let bb = base.binding();
            let bs: Vec<Binding<'_>> = rigs.iter().map(Rig::binding).collect();
            let (mut bi, mut bo) = (GraphInstance::new(&bb), GraphOutput::new(&base.graph));
            let mut insts: Vec<(GraphInstance, GraphOutput)> =
                rigs.iter().zip(&bs).map(|(r, b)| (GraphInstance::new(b), GraphOutput::new(&r.graph))).collect();
            let (hips, feet) = (base.joint("hips"), [base.joint("foot_l"), base.joint("foot_r")]);
            let contacts = [base.graph.curve("contact.foot_l").unwrap(), base.graph.curve("contact.foot_r").unwrap()];
            let velocity = Vec3::Z * NATURAL * ratio;
            let inputs = UpdateInputs { ground: &flat, moved: RootDelta::new(velocity * dt, 0.0), velocity, ..UpdateInputs::default() };
            let warm = hz.round() as usize;
            let frames = warm + (4.0 / m * hz).round() as usize;
            let mut offs: Vec<Offset> = cases.iter().map(|_| Offset::default()).collect();
            let mut stats = vec![FootStats::default(); cases.len()];
            // Largest rise of a contact ankle over the animated one (a leg short of its target), m.
            let mut rise = vec![0.0f32; cases.len()];
            for k in 0..frames {
                bi.update(&bb, dt, 0, &inputs, &mut bo);
                for ((inst, out), b) in insts.iter_mut().zip(&bs) {
                    inst.update(b, dt, 0, &inputs, out);
                }
                if k < warm {
                    continue;
                }
                let y0 = bo.model.translation(hips).y;
                for (i, (_, out)) in insts.iter().enumerate() {
                    offs[i].add(out.model.translation(hips).y - y0);
                    stats[i] += out.feet;
                }
                for (a, (&f, &c)) in ankle.iter_mut().zip(feet.iter().zip(&contacts)) {
                    if bo.curves[c] > 0.5 {
                        let y = bo.model.translation(f).y;
                        for (fl, (_, out)) in rise.iter_mut().zip(&insts) {
                            *fl = fl.max(out.model.translation(f).y - y);
                        }
                        *a = (a.0.min(y), a.1.max(y));
                    }
                }
            }
            let cell = |o: &Offset| format!("{:5.1} {:5.1} {:5.2}", o.rms() * 1e3, o.peak * 1e3, o.step * 1e3);
            let (fp, both) = (&stats[1], &stats[2]);
            println!(
                "{hz:>3} {ratio:>3.1} {:>5.3} | {} | {} | {} | {:>5.3} {:>5.3} {:>3}/{:<3} {:>3}/{:<3} {:>4} {:>5.3} {:>5.1} {:>5.1}",
                ratio / m,
                cell(&offs[0]),
                cell(&offs[1]),
                cell(&offs[2]),
                fp.drift_max,
                both.drift_max,
                fp.cap_bound,
                offs[1].n,
                both.cap_bound,
                offs[2].n,
                both.unplanted,
                stats[0].stride_drop_max.max(both.stride_drop_max),
                rise[0] * 1e3,
                rise[2] * 1e3,
            );
        }
    }
    println!(
        "ankle height while its contact holds (no ops, model space): left {:.3}..{:.3} m, right {:.3}..{:.3} m",
        ankle[0].0, ankle[0].1, ankle[1].0, ankle[1].1
    );
}

#[test]
#[ignore = "measurement: pelvis bob"]
fn pelvis_bob_on_flat_ground() {
    println!("pelvis bob: hips y with ops minus without, flat ground at y = 0, 4 cycles after 1 s of warm-up.");
    println!("spd = speed / clip speed (1.2 m/s); strd = the stride ratio StrideWarp takes (rate takes spd^0.3).");
    println!("cells: rms, peak (mm), largest per-frame change (mm); drFP/drift = lock drift max (m), FootPlacement only / both;");
    println!("flSW, flBo = largest contact ankle rise over the animated one (mm; a leg short of its target), StrideWarp only / both.");
    println!("cap = updates where the sprung pelvis sat above the reach cap (the leg reach clamp took it) / measured updates; unpl, sdrop = both run (sdrop m).");
    for clip in ["swing", "planted", "heeloff"] {
        measure(clip);
    }
    println!("\nswing: the fixture walk; its ankle lifts while the contact holds, so FootPlacement's numbers are a fixture artifact.");
    println!("planted: flat stance (cause 1 can't show); heeloff: 2 cm of heel-off inside the contact (cause 1 shows).");
}
