//! Creature-ready ops and motion warping (animation.md "Next-gen roadmap",
//! rig-profile.md "Verification"): a guar-like quadruped, an insect-like
//! hexapod, a cliff-racer-like flyer and the biped regression, walking on
//! slopes and stairs with foot placement; warp windows snapping attacks to
//! static and moving targets and a vault onto a ledge.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]

use std::collections::HashMap;
use std::f32::consts::FRAC_PI_6;
use std::sync::Arc;

use glam::{Quat, Vec3};
use sf_anim::fixtures::{biped, flyer, gait, hexapod, quadruped, sampled};
use sf_anim::{
    Clip, ClipRules, ContactRules, CookSettings, Event, Looping, RigHints, RigProfile, RootDelta, RootMotionMode, RootMotionRules, Skeleton,
    SourceClip, SourceTrack, cook_clip, detect_profile,
};
use sf_bake::ProductKey;

use crate::*;

struct Source {
    clips: HashMap<String, Clip>,
    profile: Option<RigProfile>,
}

impl CompileSource for Source {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        let c = self.clips.get(reference).cloned().ok_or_else(|| format!("no clip {reference}"))?;
        Ok((ProductKey::new("anim.cook", reference), c))
    }
    fn subgraph(&mut self, _path: &str) -> Result<asset::GraphAsset, String> {
        Err("no subgraphs".into())
    }
    fn profile(&mut self) -> Result<Option<RigProfile>, String> {
        Ok(self.profile.clone())
    }
}

struct Rig {
    skeleton: Skeleton,
    profile: RigProfile,
    graph: CompiledGraph,
    clips: Vec<Arc<Clip>>,
}

impl Rig {
    fn new(skeleton: Skeleton, clips: Vec<Clip>, ron: &str) -> Self {
        let profile = detect_profile(&skeleton, &RigHints::default()).unwrap();
        let mut src = Source { clips: clips.into_iter().map(|c| (c.name.clone(), c)).collect(), profile: Some(profile.clone()) };
        let graph = compile_str("test", ron, &skeleton, &mut src).unwrap();
        let clips = graph.clips.iter().map(|c| Arc::new(src.clips[&c.reference].clone())).collect();
        Self { skeleton, profile, graph, clips }
    }

    fn binding(&self) -> Binding<'_> {
        Binding::new(&self.graph, &self.skeleton, &self.clips).unwrap()
    }

    fn joint(&self, name: &str) -> usize {
        self.skeleton.find(name).unwrap()
    }
}

fn walk_rules() -> ClipRules {
    ClipRules {
        looping: Looping::Whole,
        root_motion: RootMotionRules { mode: RootMotionMode::Match, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    }
}

/// A walk on a skeleton's support limbs (diagonal sequence for four legs,
/// alternating tripods for six) and the speed that matches its stance sweep.
fn walk_clip(s: &Skeleton) -> (Clip, f32) {
    let (swing, duty) = (0.15f32, 0.5f32);
    let p = detect_profile(s, &RigHints::default()).unwrap();
    let supports: Vec<_> = p.support_limbs().collect();
    let rest = s.rest_model();
    let y = |j: u16| rest[usize::from(j)].translation.y;
    let hang = supports.iter().map(|l| y(l.joints[0]) - y(l.effector)).sum::<f32>() / supports.len() as f32;
    let speed = 2.0 * hang * swing.sin() / duty;
    let n = supports.len() as f32;
    let legs: Vec<(String, String, f32)> = supports
        .iter()
        .enumerate()
        .map(|(i, l)| {
            // Four legs: diagonal pairs (fl+br, fr+bl) half a cycle apart; six:
            // tripods (l0 r1 l2 against r0 l1 r2); two: alternating.
            let phase = match supports.len() {
                4 => [0.0, 0.5, 0.5, 0.0][i],
                6 => [0.0, 0.5, 0.5, 0.0, 0.0, 0.5][i],
                _ => i as f32 / n,
            };
            (s.name(usize::from(l.joints[0])).to_owned(), s.name(usize::from(l.effector)).to_owned(), phase)
        })
        .collect();
    let refs: Vec<(&str, &str, f32)> = legs.iter().map(|(a, b, c)| (a.as_str(), b.as_str(), *c)).collect();
    (cook_clip(s, &gait(s, "walk", 1.0, speed, swing, duty, &refs), &walk_rules(), &CookSettings::default()).unwrap(), speed)
}

/// Analytic ground: height and normal at (x, z).
type Ground = dyn Fn(f32, f32) -> (f32, Vec3);

fn slope(deg: f32) -> impl Fn(f32, f32) -> (f32, Vec3) {
    let t = deg.to_radians().tan();
    move |_, z| (z * t, Vec3::new(0.0, 1.0, -t).normalize())
}

fn stairs(rise: f32, run: f32) -> impl Fn(f32, f32) -> (f32, Vec3) {
    move |_, z| (rise * (z.max(0.0) / run).floor(), Vec3::Y)
}

#[derive(Debug)]
struct Walked {
    /// Per support limb: mean foot speed during contact, m/s.
    slide: Vec<f32>,
    /// 95th percentile of |foot height − (ground + sole)| during contact.
    height_p95: f32,
    /// Mean change of the pelvis → chest pitch against the flat-ground walk, radians.
    pitch: Option<f32>,
}

/// Walks a rig forward at `speed` over `ground` for `frames` at 60 Hz:
/// the root follows the ground under it with the mover's step smoothing
/// (critical spring, half-life 0.08 s); ground samples come from the
/// graph's probe origins, as the controller provides them.
fn walk(rig: &Rig, ground: &Ground, speed: f32, frames: usize, pitch: Option<(&str, &str)>) -> Walked {
    let b = rig.binding();
    let dt = 1.0 / 60.0;
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let (mut flat_inst, mut flat_out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let legs: Vec<usize> = (0..rig.graph.legs.len()).filter(|&i| rig.graph.legs[i].support).collect();
    let soles: Vec<f32> = legs
        .iter()
        .map(|&i| {
            let e = usize::from(rig.graph.legs[i].effector());
            rig.profile.limbs.iter().find(|l| usize::from(l.effector) == e).and_then(|l| l.foot.as_ref()).map_or(0.0, |f| f.sole)
        })
        .collect();
    let mut z = 0.0f32;
    let mut drawn = ground(0.0, 0.0).0;
    let mut drawn_v = 0.0f32;
    let mut prev: Vec<Option<Vec3>> = vec![None; legs.len()];
    let (mut slide, mut count) = (vec![0.0f32; legs.len()], vec![0u32; legs.len()]);
    let mut heights = Vec::new();
    let (mut pitch_sum, mut pitch_n) = (0.0f32, 0u32);
    let flat: Vec<Option<GroundSample>> = vec![Some(GroundSample { point: Vec3::ZERO, normal: Vec3::Y }); rig.graph.legs.len()];
    for k in 0..frames {
        // As the controller glue does it: the mover moves first, then the
        // probes go out at the last update's origins in the new frame.
        let old = drawn;
        z += speed * dt;
        sf_anim::spring::critical_spring(&mut drawn, &mut drawn_v, ground(0.0, z).0, 0.08, dt);
        let samples: Vec<Option<GroundSample>> = out
            .probe_origins
            .iter()
            .map(|o| {
                let (gy, n) = ground(o.x, z + o.z);
                Some(GroundSample { point: Vec3::new(o.x, gy - drawn, o.z), normal: n })
            })
            .collect();
        let moved = RootDelta::new(Vec3::new(0.0, drawn - old, speed * dt), 0.0);
        let inputs = UpdateInputs { velocity: Vec3::new(0.0, 0.0, speed), moved, ground: &samples, ..UpdateInputs::default() };
        inst.update(&b, dt, 0, &inputs, &mut out);
        let flat_moved = RootDelta::new(Vec3::new(0.0, 0.0, speed * dt), 0.0);
        let flat_inputs = UpdateInputs { velocity: Vec3::new(0.0, 0.0, speed), moved: flat_moved, ground: &flat, ..UpdateInputs::default() };
        flat_inst.update(&b, dt, 0, &flat_inputs, &mut flat_out);
        let warm = k > 60;
        for (n, &i) in legs.iter().enumerate() {
            let leg = &rig.graph.legs[i];
            let f = out.model.translation(usize::from(leg.effector())) + Vec3::new(0.0, drawn, z);
            let contact = leg.contact.map_or(0.0, |c| out.curves[usize::from(c)]);
            if contact > 0.5 {
                if let (Some(p), true) = (prev[n], warm) {
                    slide[n] += f.distance(p) / dt;
                    count[n] += 1;
                }
                if warm {
                    heights.push((f.y - (ground(f.x, f.z).0 + soles[n])).abs());
                }
                prev[n] = Some(f);
            } else {
                prev[n] = None;
            }
        }
        if let (Some((a, c)), true) = (pitch, warm) {
            let angle = |m: &sf_anim::ModelPose| {
                let v = m.translation(rig.joint(c)) - m.translation(rig.joint(a));
                v.y.atan2(Vec3::new(v.x, 0.0, v.z).length())
            };
            pitch_sum += angle(&out.model) - angle(&flat_out.model);
            pitch_n += 1;
        }
    }
    heights.sort_by(f32::total_cmp);
    let p95 = heights.get(heights.len() * 95 / 100).copied().unwrap_or(0.0);
    Walked {
        slide: slide.iter().zip(&count).map(|(s, &c)| if c > 0 { s / c as f32 } else { 0.0 }).collect(),
        height_p95: p95,
        pitch: (pitch_n > 0).then(|| pitch_sum / pitch_n as f32),
    }
}

fn assert_planted(what: &str, w: &Walked) {
    for (i, s) in w.slide.iter().enumerate() {
        assert!(*s <= 0.02, "{what}: foot {i} slides {s:.4} m/s ({w:?})");
    }
    assert!(w.height_p95 <= 0.02, "{what}: foot height error p95 {:.4} m ({w:?})", w.height_p95);
}

const QUADRUPED: &str = r#"(
    nodes: {"w": Clip(clip: "walk")},
    layers: [(node: "w")],
    profile_limbs: Support,
    procedural: [FootPlacement(pelvis: Joint("pelvis"), body: Fit, roll: 0.5)],
)"#;

#[test]
fn quadruped_feet_stay_planted_and_the_body_pitches_on_slopes_and_stairs() {
    let s = quadruped(1.0);
    let (clip, speed) = walk_clip(&s);
    let rig = Rig::new(s.clone(), vec![clip], QUADRUPED);
    assert_eq!(rig.graph.legs.len(), 4);
    assert_eq!(rig.graph.legs.iter().filter(|l| l.solver == sf_anim::LimbSolver::TwoBoneAnkle).count(), 2);
    let Op::FootPlacement { girdles, .. } = &rig.graph.ops[0].op else { panic!() };
    assert_eq!(girdles.len(), 2, "pelvis and chest");
    for deg in [10.0f32, 20.0] {
        let w = walk(&rig, &slope(deg), 1.0, 240, Some(("pelvis", "chest")));
        println!("quadruped {deg}° slope: {w:?}");
        assert_planted(&format!("quadruped {deg}°"), &w);
        let pitch = w.pitch.unwrap().to_degrees();
        assert!((pitch - deg).abs() <= 3.0, "{deg}° slope: body pitch {pitch:.2}°");
    }
    // Feet landing just past a step edge sample the step they land on
    // (probes in the moved frame, origins extrapolated one update ahead).
    let w = walk(&rig, &stairs(0.15, 0.3), speed, 240, None);
    println!("quadruped stairs: {w:?}");
    assert_planted("quadruped stairs", &w);
    let w = walk(&rig, &stairs(0.3, 1.2), speed, 240, None);
    println!("quadruped steps: {w:?}");
    assert_planted("quadruped steps", &w);
}

/// Probe origins lead the animated effectors by one update, so the probe
/// the controller casts next frame lands where the next update's foot is.
#[test]
fn probe_origins_predict_the_next_update_effectors() {
    let s = quadruped(1.0);
    let ron = r#"(
        nodes: {"w": Clip(clip: "walk")},
        layers: [(node: "w")],
        profile_limbs: Support,
    )"#;
    let (clip, _) = walk_clip(&s);
    let rig = Rig::new(s, vec![clip], ron);
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let effectors = |out: &GraphOutput| -> Vec<Vec3> { rig.graph.legs.iter().map(|l| out.model.translation(usize::from(l.effector()))).collect() };
    let (mut led, mut naive, mut n) = (0.0f32, 0.0f32, 0u32);
    let mut last: Option<(Vec<Vec3>, Vec<Vec3>)> = None;
    for _ in 0..120 {
        inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs::default(), &mut out);
        let now = effectors(&out);
        if let Some((origins, before)) = &last {
            for i in 0..now.len() {
                led += origins[i].distance(now[i]);
                naive += before[i].distance(now[i]);
                n += 1;
            }
        }
        last = Some((out.probe_origins.clone(), now));
    }
    let (led, naive) = (led / n as f32, naive / n as f32);
    println!("probe prediction error: {:.2} mm led, {:.2} mm at the current effector", led * 1e3, naive * 1e3);
    assert!(led < 0.2 * naive, "led {led} m against {naive} m");
}

#[test]
fn hexapod_and_biped_feet_stay_planted() {
    let s = hexapod();
    let ron = r#"(
        nodes: {"w": Clip(clip: "walk")},
        layers: [(node: "w")],
        profile_limbs: Support,
        procedural: [FootPlacement(pelvis: Joint("thorax"), body: Fit)],
    )"#;
    let (clip, speed) = walk_clip(&s);
    let rig = Rig::new(s.clone(), vec![clip], ron);
    assert_eq!(rig.graph.legs.len(), 6);
    for (what, ground) in [("slope", Box::new(slope(15.0)) as Box<Ground>), ("stairs", Box::new(stairs(0.05, 0.2)))] {
        let w = walk(&rig, ground.as_ref(), speed, 240, None);
        println!("hexapod {what}: {w:?}");
        assert_planted(&format!("hexapod {what}"), &w);
    }

    // The biped regression: explicit hip/knee/foot legs, the default
    // `Lower` body fit (the pelvis only drops).
    let s = biped();
    let ron = r#"(
        nodes: {"w": Clip(clip: "walk")},
        layers: [(node: "w")],
        legs: [
            (name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l"), contact: "contact.foot_l"),
            (name: "r", hip: Joint("thigh_r"), knee: Joint("shin_r"), foot: Joint("foot_r"), contact: "contact.foot_r"),
        ],
        procedural: [FootPlacement(pelvis: Joint("hips"))],
    )"#;
    let (clip, speed) = walk_clip(&s);
    let rig = Rig::new(s.clone(), vec![clip], ron);
    for (what, ground) in [("slope", Box::new(slope(15.0)) as Box<Ground>), ("stairs", Box::new(stairs(0.15, 0.3)))] {
        let w = walk(&rig, ground.as_ref(), speed, 240, None);
        println!("biped {what}: {w:?}");
        assert_planted(&format!("biped {what}"), &w);
    }
}

#[test]
fn flyer_limbs_and_chains_reach_with_rotation() {
    let s = flyer();
    let idle = SourceClip { name: "hover".into(), duration: 1.0, ..SourceClip::default() };
    let clip = cook_clip(&s, &idle, &walk_rules(), &CookSettings::default()).unwrap();
    let ron = r#"(
        nodes: {"h": Clip(clip: "hover")},
        layers: [(node: "h")],
        profile_limbs: All,
        procedural: [
            LimbIk(limb: "leg_l", target: "perch"),
            LimbIk(limb: "wing_r", target: "tip"),
            ChainIk(chain: [Joint("tail1"), Joint("tail2"), Joint("tail3")], solver: Ccd, target: "tail"),
        ],
    )"#;
    let rig = Rig::new(s, vec![clip], ron);
    assert!(rig.graph.leg("wing_l").is_some() && rig.graph.leg("leg_r").is_some());
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let grip = Quat::from_rotation_x(-0.6);
    let slot = |n: &str| rig.graph.ik_target(n).unwrap();
    let mut targets = vec![None; rig.graph.ik_targets.len()];
    targets[slot("perch")] = Some(IkTarget { position: Vec3::new(0.12, 0.6, 0.15), rotation: Some(grip) });
    targets[slot("tip")] = Some(IkTarget::from(Vec3::new(-0.7, 1.3, 0.2)));
    targets[slot("tail")] = Some(IkTarget::from(Vec3::new(0.2, 0.8, -0.8)));
    inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs { ik_targets: &targets, ..UpdateInputs::default() }, &mut out);
    let foot = rig.joint("foot_l");
    assert!(out.model.translation(foot).distance(Vec3::new(0.12, 0.6, 0.15)) < 2e-3);
    assert!(sf_anim::angle_between(out.model.rotation(foot), grip) < 1e-3);
    assert!(out.model.translation(rig.joint("wing_hand_r")).distance(Vec3::new(-0.7, 1.3, 0.2)) < 2e-3);
    assert!(out.model.translation(rig.joint("tail3")).distance(Vec3::new(0.2, 0.8, -0.8)) < 5e-3);
}

#[test]
fn spine_bend_and_lean_follow_turns() {
    let s = quadruped(1.0);
    let ron = r#"(
        nodes: {"w": Clip(clip: "walk")},
        layers: [(node: "w")],
        procedural: [
            SpineBend(spine: [(Joint("spine1"), 0.2), (Joint("spine2"), 0.3), (Joint("chest"), 0.2), (Joint("neck"), 0.3)], halflife: 0.01),
            Lean(joints: [(Joint("pelvis"), 1.0)], halflife: 0.01),
        ],
    )"#;
    let (clip, _) = walk_clip(&s);
    let rig = Rig::new(s.clone(), vec![clip], ron);
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let inputs = UpdateInputs { turn_rate: 1.0, acceleration: Vec3::new(3.0, 0.0, 0.0), ..UpdateInputs::default() };
    for _ in 0..30 {
        inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
    }
    // Turning left (+yaw): the head swings left of the pelvis; lean rolls left side down.
    let head = out.model.translation(rig.joint("head"));
    assert!(head.x > 0.1, "head {head}");
    let up = out.model.rotation(rig.joint("pelvis")) * Vec3::Y;
    assert!(up.x > 0.05, "pelvis up {up}");
}

// --- Motion warping -----------------------------------------------------------------------

/// A Drive clip on the biped: the root moves `forward` m over 1 s and, for
/// a vault, rises `up` m; one warp window over [0.2, 0.8].
fn warp_clip(s: &Skeleton, name: &str, forward: f32, up: f32, payload: &str) -> Clip {
    let src = SourceClip {
        name: name.into(),
        duration: 1.2,
        tracks: vec![
            SourceTrack {
                joint: "root".into(),
                translation: Some(sampled(1.2, move |t| Vec3::new(0.0, up * t.min(1.0), forward * t.min(1.0)))),
                ..SourceTrack::default()
            },
            SourceTrack {
                joint: "upperarm_r".into(),
                rotation: Some(sampled(1.2, |t| Quat::from_rotation_z(0.6) * Quat::from_rotation_y(1.5 * t))),
                ..SourceTrack::default()
            },
        ],
        events: vec![Event { time: 0.2, end: Some(0.8), name: "warp".into(), payload: payload.into() }],
        ..SourceClip::default()
    };
    let rules = ClipRules {
        root_motion: RootMotionRules { mode: RootMotionMode::Drive, vertical: up != 0.0, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    };
    cook_clip(s, &src, &rules, &CookSettings::default()).unwrap()
}

fn warp_rig(payload: &str, forward: f32, up: f32) -> Rig {
    let s = biped();
    let clip = warp_clip(&s, "act", forward, up, payload);
    let ron = r#"(
        warp_targets: ["enemy"],
        nodes: {"a": Clip(clip: "act", root_motion: Drive)},
        layers: [(node: "a")],
    )"#;
    Rig::new(s, vec![clip], ron)
}

struct Warped {
    /// World root position and yaw when the window closed.
    end: (Vec3, f32),
    /// Target position and yaw at that moment.
    target: (Vec3, f32),
    /// Largest root speed ratio between consecutive frames around the window start.
    start_jump: f32,
    /// World position of `joint` at the window's end (if asked).
    joint: Option<Vec3>,
    saw_vertical: bool,
}

/// Plays a warp clip from the origin facing +Z, the target moving with
/// `velocity`; the root delta is applied each frame (the mover's one-frame
/// latency is the same: targets are given in the frame the character is in).
fn play_warp(rig: &Rig, target: (Vec3, f32), velocity: Vec3, joint: Option<&str>) -> Warped {
    let b = rig.binding();
    let dt = 1.0 / 60.0;
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let (mut pos, mut yaw) = (Vec3::ZERO, 0.0f32);
    let mut speeds = Vec::new();
    let mut t = 0.0f32;
    let mut saw_vertical = false;
    let slot = rig.graph.warp_target("enemy").unwrap();
    loop {
        let tp = target.0 + velocity * t;
        let rot = Quat::from_rotation_y(yaw);
        let mut targets = vec![None; rig.graph.warp_targets.len()];
        targets[slot] = Some(WarpTarget { position: rot.inverse() * (tp - pos), yaw: target.1 - yaw, velocity: rot.inverse() * velocity });
        inst.update(&b, dt, 0, &UpdateInputs { warp_targets: &targets, ..UpdateInputs::default() }, &mut out);
        pos += rot * out.root.translation;
        yaw += out.root.yaw;
        t += dt;
        speeds.push((t, out.root.translation.length() / dt));
        saw_vertical |= out.warp.is_some_and(|w| w.vertical);
        if out.warp.is_some_and(|w| w.remaining <= 1e-5) {
            let jumps = speeds.windows(2).filter(|w| (0.1..0.45).contains(&w[1].0)).map(|w| w[1].1.max(1e-3) / w[0].1.max(1e-3));
            let start_jump = jumps.fold(1.0f32, |a, r| a.max(r).max(1.0 / r));
            let joint = joint.map(|j| pos + Quat::from_rotation_y(yaw) * out.model.translation(rig.joint(j)));
            return Warped { end: (pos, yaw), target: (target.0 + velocity * t, target.1), start_jump, joint, saw_vertical };
        }
        assert!(t < 1.2, "the window never closed");
    }
}

#[test]
fn warped_attacks_snap_to_static_and_moving_targets() {
    // Static target off the authored line, a 30° facing to end with.
    let rig = warp_rig("enemy;skew;rot=default", 1.0, 0.0);
    let target = (Vec3::new(0.35, 0.0, 1.3), FRAC_PI_6);
    let w = play_warp(&rig, target, Vec3::ZERO, None);
    let err = w.end.0.distance(w.target.0);
    let yaw_err = (w.end.1 - w.target.1).abs().to_degrees();
    println!("static: {err:.4} m, {yaw_err:.3}°, start jump {:.3}", w.start_jump);
    assert!(err <= 0.02 && yaw_err <= 2.0, "static: {err} m, {yaw_err}°");
    assert!(w.start_jump <= 1.2, "root speed jump {} around the window start", w.start_jump);

    // A target 30% farther than the authored path: still no speed jump > 20%.
    let w = play_warp(&rig, (Vec3::new(0.0, 0.0, 0.2 + 0.6 * 1.3 + 0.2), 0.0), Vec3::ZERO, None);
    assert!(w.start_jump <= 1.2, "far target: speed jump {}", w.start_jump);
    assert!(w.end.0.distance(w.target.0) <= 0.02);

    // A target moving at 3 m/s across: tracked to within 5 cm, facing it.
    let rig = warp_rig("enemy;skew;rot=facing", 1.0, 0.0);
    let w = play_warp(&rig, (Vec3::new(-0.8, 0.0, 1.4), 0.0), Vec3::new(3.0, 0.0, 0.0), None);
    let err = w.end.0.distance(w.target.0);
    println!("moving: {err:.4} m");
    assert!(err <= 0.05, "moving target: {err} m");

    // Warp point on a joint: the hand, not the root, lands on the target.
    let rig = warp_rig("enemy;skew;y=warp;rot=default;point=hand_r", 1.0, 0.0);
    let w = play_warp(&rig, (Vec3::new(0.2, 1.3, 1.5), -0.3), Vec3::ZERO, Some("hand_r"));
    let err = w.joint.unwrap().distance(w.target.0);
    assert!(err <= 0.02, "hand warp point: {err} m");

    // No target set: the authored motion plays unchanged.
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    inst.update(&b, 0.5, 0, &UpdateInputs::default(), &mut out);
    assert!(out.warp.is_none() && (out.root.translation.z - 0.5).abs() < 1e-3);
}

#[test]
fn a_warped_vault_lands_on_the_ledge_height() {
    let rig = warp_rig("enemy;skew;y=warp;rot=none", 1.0, 0.8);
    let ledge = Vec3::new(0.0, 1.1, 1.25);
    let w = play_warp(&rig, (ledge, 0.0), Vec3::ZERO, None);
    let err = w.end.0.distance(ledge);
    println!("vault: {err:.4} m");
    assert!(err <= 0.02, "vault end error {err} m ({:?})", w.end.0);
    assert!(w.saw_vertical, "the window reports traversal");
    // `y=ignore` keeps the authored height.
    let rig = warp_rig("enemy;skew;rot=none", 1.0, 0.8);
    let w = play_warp(&rig, (ledge, 0.0), Vec3::ZERO, None);
    assert!((w.end.0.y - 0.8 * 0.8).abs() < 0.02 && (w.end.0.z - ledge.z).abs() < 0.02, "{:?}", w.end.0);
}

/// `cargo test --release -p sf_anim_graph --all-features -- --ignored --nocapture warp_and_creature_timing`
#[test]
#[ignore = "timing; run in release"]
fn warp_and_creature_timing() {
    let rig = warp_rig("enemy;skew;rot=facing", 1.0, 0.0);
    let b = rig.binding();
    let slot = rig.graph.warp_target("enemy").unwrap();
    let mut targets = vec![None; rig.graph.warp_targets.len()];
    targets[slot] = Some(WarpTarget { position: Vec3::new(0.3, 0.0, 1.2), ..WarpTarget::default() });
    let n = 20_000u32;
    let time = |with: bool| {
        let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
        let inputs = UpdateInputs { warp_targets: if with { &targets } else { &[] }, ..UpdateInputs::default() };
        let start = std::time::Instant::now();
        for k in 0..n {
            if k % 30 == 0 {
                inst = GraphInstance::new(&b);
            }
            inst.update(&b, 0.5 / 30.0, 3, &inputs, &mut out);
        }
        start.elapsed().as_secs_f64() * 1e6 / f64::from(n)
    };
    let (plain, warped) = (time(false), time(true));
    println!("warp: {:.3} µs per update ({plain:.3} → {warped:.3})", warped - plain);

    let s = quadruped(1.0);
    let (clip, speed) = walk_clip(&s);
    let rig = Rig::new(s.clone(), vec![clip], QUADRUPED);
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let ground = vec![Some(GroundSample { point: Vec3::ZERO, normal: Vec3::new(0.0, 1.0, -0.3).normalize() }); 4];
    let inputs =
        UpdateInputs { velocity: Vec3::Z * speed, ground: &ground, moved: RootDelta::new(Vec3::Z * speed / 60.0, 0.0), ..UpdateInputs::default() };
    let start = std::time::Instant::now();
    for _ in 0..n {
        inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
    }
    println!("quadruped A0 update ({} joints, 4 legs, Fit): {:.2} µs", rig.skeleton.len(), start.elapsed().as_secs_f64() * 1e6 / f64::from(n));
}

/// Walking down a 20° slope the nearly straight front legs over-reach once
/// the body has pitched (it was a 2.7 cm/s front-foot slide): the reach
/// correction lowers the whole body by the worst shortfall, which is exact
/// in one pass and keeps the fitted pitch.
#[test]
fn quadruped_downhill_feet_stay_planted() {
    let s = quadruped(1.0);
    let (clip, speed) = walk_clip(&s);
    let rig = Rig::new(s, vec![clip], QUADRUPED);
    let w = walk(&rig, &slope(-20.0), speed, 240, Some(("pelvis", "chest")));
    println!("quadruped downhill 20°: {w:?}");
    assert!((w.pitch.unwrap().to_degrees() + 20.0).abs() <= 3.0, "downhill pitch {}", w.pitch.unwrap().to_degrees());
    assert_planted("quadruped downhill", &w);
}

/// `FullBodyIk` after `FootPlacement`: a biped reaches past its arm while
/// the placed feet stay pinned and the spine bends; a quadruped grazes with
/// four feet planted (animation.md "Layer 2").
#[test]
fn full_body_ik_reaches_with_the_body_and_keeps_placed_feet() {
    let s = biped();
    let idle = SourceClip { name: "idle".into(), duration: 1.0, ..SourceClip::default() };
    let clip = cook_clip(&s, &idle, &walk_rules(), &CookSettings::default()).unwrap();
    let ron = r#"(
        nodes: {"i": Clip(clip: "idle")},
        layers: [(node: "i")],
        profile_limbs: All,
        procedural: [
            FootPlacement(pelvis: Joint("hips")),
            FullBodyIk(effectors: [(limb: "arm_l", target: "grab")]),
        ],
    )"#;
    let rig = Rig::new(s, vec![clip], ron);
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let mut targets = vec![None; rig.graph.ik_targets.len()];
    let goal = Vec3::new(0.3, 1.2, 0.75);
    targets[rig.graph.ik_target("grab").unwrap()] = Some(IkTarget::from(goal));
    let ground = vec![Some(GroundSample { point: Vec3::new(0.0, 0.05, 0.0), normal: Vec3::Y }); rig.graph.legs.len()];
    inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs { ground: &ground, ..UpdateInputs::default() }, &mut out);
    let feet: Vec<Vec3> = ["foot_l", "foot_r"].iter().map(|f| out.model.translation(rig.joint(f))).collect();
    let chest = out.model.translation(rig.joint("chest"));
    inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs { ground: &ground, ik_targets: &targets, ..UpdateInputs::default() }, &mut out);
    assert!(out.model.translation(rig.joint("hand_l")).distance(goal) < 0.02, "hand misses");
    for (f, p) in ["foot_l", "foot_r"].iter().zip(&feet) {
        assert!(out.model.translation(rig.joint(f)).distance(*p) < 5e-3, "{f} moved");
    }
    assert!(out.model.translation(rig.joint("chest")).z > chest.z + 0.1, "the chest did not lean in");
    // At A1 the op is off (lod0).
    inst.update(&b, 1.0 / 60.0, 1, &UpdateInputs { ground: &ground, ik_targets: &targets, ..UpdateInputs::default() }, &mut out);
    assert!(out.model.translation(rig.joint("hand_l")).distance(goal) > 0.2);

    // Quadruped: the head as a joint effector, four feet pinned.
    let s = quadruped(1.0);
    let idle = SourceClip { name: "idle".into(), duration: 1.0, ..SourceClip::default() };
    let clip = cook_clip(&s, &idle, &walk_rules(), &CookSettings::default()).unwrap();
    let ron = r#"(
        nodes: {"i": Clip(clip: "idle")},
        layers: [(node: "i")],
        profile_limbs: Support,
        procedural: [FullBodyIk(effectors: [(joint: Joint("head"), target: "graze", pull: 0.5)], iterations: 20)],
    )"#;
    let rig = Rig::new(s, vec![clip], ron);
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs::default(), &mut out);
    let rest = out.model.clone();
    let goal = rest.translation(rig.joint("head")) + Vec3::new(0.0, -0.25, 0.1);
    let mut targets = vec![None; rig.graph.ik_targets.len()];
    targets[rig.graph.ik_target("graze").unwrap()] = Some(IkTarget::from(goal));
    inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs { ik_targets: &targets, ..UpdateInputs::default() }, &mut out);
    assert!(out.model.translation(rig.joint("head")).distance(goal) < 0.015, "head misses");
    for l in rig.profile.support_limbs() {
        let e = usize::from(l.effector);
        assert!(out.model.translation(e).distance(rest.translation(e)) < 5e-3, "{} moved", l.name);
    }
}

// --- Secondary motion (P9: HitReact, SpringBones) -----------------------------------------

/// A still quadruped (guar-like, with a tail) with `ops` after the graph.
fn still_quadruped(ops: &str) -> Rig {
    let s = quadruped(1.0);
    let idle = SourceClip { name: "idle".into(), duration: 1.0, ..SourceClip::default() };
    let clip = cook_clip(&s, &idle, &walk_rules(), &CookSettings::default()).unwrap();
    let ron = format!(r#"(nodes: {{"i": Clip(clip: "idle")}}, layers: [(node: "i")], procedural: [{ops}])"#);
    Rig::new(s, vec![clip], &ron)
}

/// The tail tip's offset from its animated position at `times` (s), the
/// character strafing at 3 m/s for the first 0.5 s, then standing, at
/// updates of `dt`.
fn tail_offsets(rig: &Rig, dt: f32, times: &[f32]) -> Vec<Vec3> {
    let b = rig.binding();
    let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let tip = rig.joint("tail3");
    let animated: Vec3 = rig.skeleton.rest_model()[tip].translation.into();
    let mut found = Vec::new();
    let mut t = 0.0f32;
    let mut next = 0;
    while next < times.len() {
        let moving = t < 0.5 - 1e-4;
        let moved = RootDelta::new(if moving { Vec3::X * 3.0 * dt } else { Vec3::ZERO }, 0.0);
        inst.update(&b, dt, 0, &UpdateInputs { moved, ..UpdateInputs::default() }, &mut out);
        t += dt;
        if t >= times[next] - 1e-4 {
            found.push(out.model.translation(tip) - animated);
            next += 1;
        }
    }
    found
}

#[test]
fn spring_bones_swing_decay_and_match_at_any_frame_rate() {
    let rig = still_quadruped("SpringBones()");
    assert!(matches!(&rig.graph.ops[0].op, Op::SpringBones(op) if op.chains == vec![vec![
        u16::try_from(rig.joint("tail1")).unwrap(), u16::try_from(rig.joint("tail2")).unwrap(), u16::try_from(rig.joint("tail3")).unwrap()
    ]]));
    let times = [0.25, 0.5, 1.0, 4.0, 10.0];
    let reference = tail_offsets(&rig, 1.0 / 60.0, &times);
    // Strafing right (+X) the tail trails left (−X), then settles back.
    assert!(reference[0].x < -0.02 && reference[1].x < -0.02, "trail {reference:?}");
    assert!(reference[3].length() < 1e-3 && reference[4].length() < 1e-4, "decay {reference:?}");
    for dt in [1.0 / 30.0, 1.0 / 144.0, 1.0 / 7.0] {
        let other = tail_offsets(&rig, dt, &times);
        // At 7 fps the strafe lands in 0.43 m jumps: only the settling compares.
        let from = if dt > 0.1 { 3 } else { 0 };
        for (k, (a, b)) in reference.iter().zip(&other).enumerate().skip(from) {
            // The motion lands per update, so slow updates differ a little mid-swing.
            let tolerance = if k < 3 { 0.25 * reference[0].length().max(0.01) } else { 1e-3 };
            assert!(a.distance(*b) < tolerance, "dt {dt}: t {} {a} vs {b}", times[k]);
        }
        // No drift: back on the animation.
        assert!(other[4].length() < 1e-4, "dt {dt}: drift {}", other[4]);
    }
}

#[test]
fn spring_bones_sag_is_frame_rate_independent() {
    let rig = still_quadruped("SpringBones(gravity: 9.81)");
    let sag: Vec<Vec3> = [1.0 / 30.0, 1.0 / 60.0, 1.0 / 144.0].iter().map(|&dt| tail_offsets(&rig, dt, &[8.0])[0]).collect();
    // Lower than animated (gravity against the pull), the same at every rate.
    assert!(sag[1].y < -0.01, "sag {sag:?}");
    assert!(sag[0].distance(sag[1]) < 1e-3 && sag[2].distance(sag[1]) < 1e-3, "sag {sag:?}");
}

#[test]
fn hit_react_kicks_peaks_and_settles_at_any_frame_rate() {
    let rig = still_quadruped("HitReact()");
    let Op::HitReact(op) = &rig.graph.ops[0].op else { panic!("HitReact") };
    // Spine, neck and head react; the root girdle and the support legs don't.
    assert!(op.joints.iter().any(|j| usize::from(j.0) == rig.joint("chest")));
    assert!(!op.joints.iter().any(|j| usize::from(j.0) == rig.joint("paw_l") || usize::from(j.0) == rig.joint("pelvis")));
    let (chest, head) = (rig.joint("chest"), rig.joint("head"));
    let rest_head: Vec3 = rig.skeleton.rest_model()[head].translation.into();
    let hit = Hit { point: rig.skeleton.rest_model()[chest].translation.into(), direction: Vec3::X, strength: 1.0, joint: None };
    // The hit joint follows the kicked critical spring: gain · y·e · t · e^(−y·t), peak `gain` at t = 1/y.
    let y = sf_anim::spring::halflife_to_damping(op.halflife) / 2.0;
    let analytic = |t: f32| op.gain * y * std::f32::consts::E * t * (-y * t).exp();
    for dt in [1.0 / 30.0, 1.0 / 60.0, 1.0 / 144.0] {
        let b = rig.binding();
        let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
        inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
        inst.hit(hit);
        let (mut peak, mut head_x) = (0.0f32, 0.0f32);
        for k in 1..=(2.0 / dt).round() as usize {
            inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
            let a = inst.hit_reaction();
            // Closed-form springs: on the curve at every rate.
            assert!((a - analytic(k as f32 * dt)).abs() < 1e-3, "dt {dt}: step {k}: {a} vs {}", analytic(k as f32 * dt));
            if a > peak {
                peak = a;
                head_x = out.model.translation(head).x - rest_head.x;
            }
        }
        assert!(peak > 0.9 * op.gain, "dt {dt}: peak {peak}");
        assert!(head_x > 0.02, "dt {dt}: head pushed {head_x}");
        assert!(inst.hit_reaction() < 1e-4 && out.model.translation(head).distance(rest_head) < 1e-4, "dt {dt}: settles");
    }
}
