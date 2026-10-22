//! Unit tests (animation.md "Verification").
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]

use std::collections::HashMap;
use std::f32::consts::TAU;
use std::sync::{Arc, RwLock};

use glam::{Quat, Vec2, Vec3};
use rayon::prelude::*;
use sf_anim::{
    AdditiveBase, Clip, ClipRules, ContactRules, CookSettings, Curve, Event, EventEdge, LocalPose, Looping, RootDelta, RootMotionMode,
    RootMotionRules, Skeleton, SkeletonRules, SourceClip, SourceCurve, SourceJoint, SourceTrack, SyncMarker, Trs, cook_clip,
};
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, RunCx, Step, Store};

use crate::asset::parse_graph;
use crate::*;

// --- Fixtures ----------------------------------------------------------------------------

fn j(name: &str, parent: Option<usize>, t: [f32; 3]) -> SourceJoint {
    SourceJoint { name: name.to_owned(), parent, rest: Trs::from_translation(Vec3::from(t)) }
}

/// A small symmetric biped, Y up, facing +Z, meters.
fn biped() -> Skeleton {
    let mut v = vec![
        j("root", None, [0.0, 0.0, 0.0]),
        j("hips", Some(0), [0.0, 1.0, 0.0]),
        j("spine", Some(1), [0.0, 0.1, 0.0]),
        j("chest", Some(2), [0.0, 0.25, 0.0]),
        j("head", Some(3), [0.0, 0.3, 0.0]),
        j("thigh_l", Some(1), [0.1, -0.05, 0.0]),
        j("shin_l", Some(5), [0.0, -0.45, 0.02]),
        j("foot_l", Some(6), [0.0, -0.45, -0.02]),
        j("thigh_r", Some(1), [-0.1, -0.05, 0.0]),
        j("shin_r", Some(8), [0.0, -0.45, 0.02]),
        j("foot_r", Some(9), [0.0, -0.45, -0.02]),
        j("upperarm_l", Some(3), [0.15, 0.2, 0.0]),
        j("forearm_l", Some(11), [0.28, 0.0, 0.0]),
        j("hand_l", Some(12), [0.25, 0.0, 0.0]),
        j("upperarm_r", Some(3), [-0.15, 0.2, 0.0]),
        j("forearm_r", Some(14), [-0.28, 0.0, 0.0]),
        j("hand_r", Some(15), [-0.25, 0.0, 0.0]),
    ];
    v[11].rest.rotation = Quat::from_rotation_z(-0.6);
    v[14].rest.rotation = Quat::from_rotation_z(0.6);
    let rules = SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() };
    Skeleton::with_rules(&v, &rules).unwrap().0
}

fn sampled<T: sf_anim::CurveValue>(duration: f32, f: impl Fn(f32) -> T) -> Curve<T> {
    let n = (duration / 0.025).round() as usize;
    let times: Vec<f32> = (0..=n).map(|i| duration * i as f32 / n as f32).collect();
    let values = times.iter().map(|&t| f(t)).collect();
    Curve::linear(times, values)
}

fn rot(d: f32, joint: &str, f: impl Fn(f32) -> Quat) -> SourceTrack {
    SourceTrack { joint: joint.into(), rotation: Some(sampled(d, f)), ..SourceTrack::default() }
}

fn trans(d: f32, joint: &str, f: impl Fn(f32) -> Vec3) -> SourceTrack {
    SourceTrack { joint: joint.into(), translation: Some(sampled(d, f)), ..SourceTrack::default() }
}

fn marker(time: f32, name: &str) -> SyncMarker {
    SyncMarker { time, name: name.into() }
}

fn rules(looping: bool, mode: RootMotionMode) -> ClipRules {
    ClipRules {
        looping: if looping { Looping::Whole } else { Looping::Once },
        root_motion: RootMotionRules { mode, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    }
}

/// A locomotion cycle: root forward at `speed`, legs swing; footsteps at
/// the `down.l` / `down.r` markers.
fn cycle(s: &Skeleton, name: &str, d: f32, speed: f32, swing: f32, right_down: f32) -> Clip {
    let w = move |t: f32| (TAU * t / d).sin();
    let src = SourceClip {
        name: name.into(),
        duration: d,
        tracks: vec![
            trans(d, "root", move |t| Vec3::new(0.0, 0.0, speed * t)),
            trans(d, "hips", move |t| Vec3::new(0.0, 1.0 + 0.03 * (2.0 * TAU * t / d).cos(), 0.0)),
            rot(d, "spine", move |t| Quat::from_rotation_x(0.1 * w(t))),
            rot(d, "thigh_l", move |t| Quat::from_rotation_x(swing * w(t))),
            rot(d, "shin_l", move |t| Quat::from_rotation_x(0.4 * (1.0 - (TAU * t / d).cos()))),
            rot(d, "thigh_r", move |t| Quat::from_rotation_x(-swing * w(t))),
            rot(d, "shin_r", move |t| Quat::from_rotation_x(0.4 * (1.0 + (TAU * t / d).cos()))),
            rot(d, "upperarm_l", move |t| Quat::from_rotation_z(-0.6) * Quat::from_rotation_x(0.3 * w(t))),
        ],
        curves: vec![SourceCurve { name: "contact.foot_l".into(), curve: sampled(d, move |t| if t < right_down * 0.8 { 1.0 } else { 0.0 }) }],
        events: vec![Event::point(0.0, "step"), Event::point(right_down, "step")],
        markers: vec![marker(0.0, "down.l"), marker(right_down, "down.r")],
    };
    cook_clip(s, &src, &rules(true, RootMotionMode::Match), &CookSettings::default()).unwrap()
}

fn idle(s: &Skeleton) -> Clip {
    let src = SourceClip {
        name: "idle".into(),
        duration: 2.0,
        tracks: vec![rot(2.0, "spine", |t| Quat::from_rotation_x(0.05 * (TAU * t / 2.0).sin()))],
        ..SourceClip::default()
    };
    cook_clip(s, &src, &rules(true, RootMotionMode::Ignore), &CookSettings::default()).unwrap()
}

fn attack(s: &Skeleton) -> Clip {
    let src = SourceClip {
        name: "attack".into(),
        duration: 0.8,
        tracks: vec![
            trans(0.8, "root", |t| Vec3::new(0.0, 0.0, 0.6 * t)),
            rot(0.8, "upperarm_r", |t| Quat::from_rotation_z(0.6) * Quat::from_rotation_y(2.0 * t)),
            rot(0.8, "chest", |t| Quat::from_rotation_y(-0.5 * t)),
        ],
        events: vec![Event::point(0.3, "hit"), Event::window(0.4, 0.7, "combo")],
        ..SourceClip::default()
    };
    cook_clip(s, &src, &rules(false, RootMotionMode::Drive), &CookSettings::default()).unwrap()
}

fn wave(s: &Skeleton) -> Clip {
    let src = SourceClip {
        name: "wave".into(),
        duration: 1.0,
        tracks: vec![
            rot(1.0, "upperarm_l", |t| Quat::from_rotation_z(0.8 + 0.3 * (TAU * t).sin())),
            rot(1.0, "head", |t| Quat::from_rotation_y(0.4 * (TAU * t).sin())),
            rot(1.0, "spine", |_| Quat::from_rotation_x(-0.2)),
        ],
        ..SourceClip::default()
    };
    cook_clip(s, &src, &rules(true, RootMotionMode::Ignore), &CookSettings::default()).unwrap()
}

fn lean(s: &Skeleton) -> Clip {
    let src =
        SourceClip { name: "lean".into(), duration: 1.0, tracks: vec![rot(1.0, "spine", |_| Quat::from_rotation_z(0.3))], ..SourceClip::default() };
    let r = ClipRules { additive: Some(AdditiveBase::Rest), ..rules(true, RootMotionMode::Ignore) };
    cook_clip(s, &src, &r, &CookSettings::default()).unwrap()
}

/// In-memory clips and subgraphs; references are the clip names.
#[derive(Default)]
struct Source {
    clips: HashMap<String, Clip>,
    subgraphs: HashMap<String, String>,
    rig: Vec<(String, String)>,
}

impl CompileSource for Source {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        let c = self.clips.get(reference).cloned().ok_or_else(|| format!("no clip {reference}"))?;
        Ok((ProductKey::new("anim.cook", reference), c))
    }
    fn subgraph(&mut self, path: &str) -> Result<asset::GraphAsset, String> {
        parse_graph(self.subgraphs.get(path).ok_or("no such subgraph")?)
    }
    fn rig_joint(&self, rig: &str) -> Option<String> {
        self.rig.iter().find(|(r, _)| r == rig).map(|(_, j)| j.clone())
    }
}

fn source(s: &Skeleton) -> Source {
    let mut src = Source::default();
    for c in [cycle(s, "walk", 1.0, 1.2, 0.5, 0.5), cycle(s, "run", 0.6, 3.6, 0.9, 0.2), idle(s), attack(s), wave(s), lean(s)] {
        src.clips.insert(c.name.clone(), c);
    }
    src
}

struct Rig {
    skeleton: Skeleton,
    graph: CompiledGraph,
    clips: Vec<Arc<Clip>>,
}

impl Rig {
    fn new(ron: &str) -> Self {
        let skeleton = biped();
        let mut src = source(&skeleton);
        let graph = compile_str("test", ron, &skeleton, &mut src).unwrap();
        let clips = graph.clips.iter().map(|c| Arc::new(src.clips[&c.reference].clone())).collect();
        Self { skeleton, graph, clips }
    }

    fn binding(&self) -> Binding<'_> {
        Binding::new(&self.graph, &self.skeleton, &self.clips).unwrap()
    }

    fn joint(&self, name: &str) -> usize {
        self.skeleton.find(name).unwrap()
    }

    fn param(&self, name: &str) -> ParamId {
        self.graph.param(name).unwrap()
    }
}

const MAIN: &str = r#"
(
    params: [
        (name: "speed", kind: Float(1.2)),
        (name: "moving", kind: Bool(false)),
        (name: "attack", kind: Trigger),
        (name: "upper", kind: Float(0.0)),
        (name: "lean", kind: Float(0.0)),
    ],
    masks: {
        "upper": (rules: [(joint: Tag("spine"), weight: 0.5), (joint: Joint("chest"), weight: 1.0)]),
    },
    nodes: {
        "locomotion": StateMachine(
            states: [
                (name: "idle", node: "idle_clip"),
                (name: "move", node: "move"),
                (name: "attack", node: "attack_clip", root_motion: Drive),
            ],
            transitions: [
                (from: "*", to: "attack", when: [Trigger("attack")], blend: Inertialize(0.2)),
                (from: "attack", to: "idle", when: [TimeLeft(0.1)], blend: Inertialize(0.25)),
                (from: "idle", to: "move", when: [IsTrue("moving")], blend: Crossfade(0.3)),
                (from: "move", to: "idle", when: [IsFalse("moving")], blend: Inertialize(0.3)),
            ],
        ),
        "idle_clip": Clip(clip: "idle"),
        "attack_clip": Clip(clip: "attack"),
        "move": BlendSpace1D(x: "speed", samples: [(clip: "walk", at: 1.2), (clip: "run", at: 3.6)], sync: (group: "feet")),
        "wave_clip": Clip(clip: "wave"),
        "lean_clip": Clip(clip: "lean"),
    },
    layers: [
        (node: "locomotion"),
        (node: "wave_clip", mask: "upper", weight: "upper"),
        (node: "lean_clip", mode: Additive, weight: "lean"),
    ],
)
"#;

fn step(rig: &Rig, inst: &mut GraphInstance, out: &mut GraphOutput, dt: f32) {
    inst.update(&rig.binding(), dt, 0, &UpdateInputs::default(), out);
}

// --- Asset and compile -------------------------------------------------------------------

#[test]
fn ron_asset_parses_with_defaults_and_compiles_deterministically() {
    let asset = parse_graph(MAIN).unwrap();
    assert_eq!(asset.params.len(), 5);
    match &asset.nodes["move"] {
        asset::NodeSpec::BlendSpace1D { x, sync, rate, .. } => {
            assert_eq!(*x, asset::ValueSpec::Param("speed".into()));
            assert_eq!(*rate, asset::ValueSpec::Const(1.0));
            assert_eq!(sync.as_ref().unwrap().group, "feet");
        }
        other => panic!("{other:?}"),
    }
    let rig = Rig::new(MAIN);
    let g = &rig.graph;
    assert_eq!(g.clips.len(), 6);
    assert_eq!(g.sync_groups, ["feet"]);
    assert!(g.events.iter().any(|e| e == "hit") && g.events.iter().any(|e| e == "step"));
    assert_eq!(g.curves, ["contact.foot_l"]);
    // Depth-first: every subtree is a range, children after parents.
    for (i, n) in g.nodes.iter().enumerate() {
        assert!(n.subtree_end as usize > i);
    }
    let sm = g.node("locomotion").unwrap();
    assert_eq!(g.state_names(sm), ["idle", "move", "attack"]);
    // The attack state's Drive mode reaches its clip.
    let NodeKind::Player(p) = &g.nodes[g.node("attack_clip").unwrap() as usize].kind else { panic!() };
    assert_eq!(p.root_motion, RootMotionMode::Drive);
    assert!(g.nodes[g.node("lean_clip").unwrap() as usize].additive);

    // Byte round trip, and two compiles are identical.
    let bytes = g.to_bytes();
    let back = CompiledGraph::from_bytes(&bytes).unwrap();
    assert_eq!(back.to_bytes(), bytes);
    assert_eq!(Rig::new(MAIN).graph.to_bytes(), bytes);
    assert!(CompiledGraph::from_bytes(&bytes[..10]).is_err());
}

#[test]
fn compile_rejects_bad_graphs() {
    let s = biped();
    let bad = |ron: &str| compile_str("bad", ron, &s, &mut source(&s)).unwrap_err().message;
    let wrap = |nodes: &str, layers: &str| {
        format!("(params: [(name: \"f\", kind: Float(0.0)), (name: \"t\", kind: Trigger)], nodes: {{ {nodes} }}, layers: [{layers}])")
    };
    assert!(bad(&wrap(r#""a": Clip(clip: "nope")"#, r#"(node: "a")"#)).contains("no clip"));
    assert!(bad(&wrap(r#""a": Blend(a: "a", b: "a", alpha: 0.5)"#, r#"(node: "a")"#)).contains("contains itself"));
    assert!(bad(&wrap(r#""a": Clip(clip: "walk", rate: "zz")"#, r#"(node: "a")"#)).contains("unknown parameter"));
    assert!(bad(&wrap(r#""a": Clip(clip: "walk"), "l": Clip(clip: "lean")"#, r#"(node: "a"), (node: "l")"#)).contains("additive"));
    assert!(
        bad(&wrap(r#""a": Additive(base: "a2", additive: "a3"), "a2": Clip(clip: "walk"), "a3": Clip(clip: "run")"#, r#"(node: "a")"#))
            .contains("additive pose")
    );
    assert!(
        bad(&wrap(
            r#""m": StateMachine(states: [(name: "x", node: "a")], transitions: [(from: "x", to: "y")]), "a": Clip(clip: "walk")"#,
            r#"(node: "m")"#
        ))
        .contains("unknown state")
    );
    assert!(
        bad(&wrap(
            r#""m": StateMachine(states: [(name: "x", node: "a")], transitions: [(from: "*", to: "x", when: [Event("boom")])]), "a": Clip(clip: "walk")"#,
            r#"(node: "m")"#
        ))
        .contains("no clip of the graph fires")
    );
    assert!(bad(&wrap(r#""a": Clip(clip: "walk")"#, r#"(node: "a", weight: 0.5)"#)).contains("base"));
    assert!(bad(&wrap(r#""a": Mirror(input: "b"), "b": Clip(clip: "walk")"#, r#"(node: "a")"#)).contains("mirror pairs"));
    assert!(
        bad(&wrap(r#""a": BlendSpace1D(x: "f", samples: [(clip: "walk", at: 1.0), (clip: "run", at: 1.0)])"#, r#"(node: "a")"#))
            .contains("same position")
    );
}

#[test]
fn weighted_masks_resolve_rules_in_order() {
    let rig = Rig::new(MAIN);
    let m = &rig.graph.masks[0];
    let w = |n: &str| m[rig.joint(n)];
    assert_eq!((w("root"), w("hips"), w("thigh_l"), w("foot_r")), (0.0, 0.0, 0.0, 0.0));
    assert_eq!(w("spine"), 0.5);
    assert_eq!((w("chest"), w("head"), w("hand_l"), w("upperarm_r")), (1.0, 1.0, 1.0, 1.0));
    // Rig names resolve through the adapter; subtree-less rules touch one joint.
    let s = biped();
    let mut src = source(&s);
    src.rig = vec![("LeftUpperArm".into(), "upperarm_l".into())];
    let ron = r#"(nodes: {"a": Clip(clip: "walk")}, masks: {"arm": (default: 0.25, rules: [(joint: Rig("LeftUpperArm"), weight: 1.0, subtree: false)])},
        layers: [(node: "a"), (node: "a", mask: "arm")])"#;
    let g = compile_str("rig", ron, &s, &mut src).unwrap();
    assert_eq!(g.masks[0][s.find("upperarm_l").unwrap()], 1.0);
    assert_eq!(g.masks[0][s.find("forearm_l").unwrap()], 0.25);
}

#[test]
fn blend_spaces_triangulate_and_weight() {
    let ron = r#"(
        params: [(name: "dir", kind: Vec2(0.0, 0.0)), (name: "s", kind: Float(0.0))],
        nodes: {
            "plane": BlendSpace2D(x: "dir.x", y: "dir.y", samples: [
                (clip: "idle", at: (0.0, 0.0)), (clip: "walk", at: (0.0, 2.0)), (clip: "run", at: (0.0, -2.0)),
                (clip: "wave", at: (2.0, 0.0)), (clip: "attack", at: (-2.0, 0.0))]),
            "line": BlendSpace1D(x: "s", samples: [(clip: "walk", at: 1.0), (clip: "run", at: 3.0), (clip: "idle", at: 0.0)]),
        },
        layers: [(node: "plane"), (node: "line", weight: 0.0)],
    )"#;
    let rig = Rig::new(ron);
    let space = |name: &str| match &rig.graph.nodes[rig.graph.node(name).unwrap() as usize].kind {
        NodeKind::Player(PlayerNode { space: Some(s), .. }) => s.clone(),
        _ => panic!(),
    };
    let plane = space("plane");
    assert_eq!(plane.triangles.len(), 4, "a plus shape is four triangles");
    assert_eq!(plane.edges.len(), 4, "its hull is a diamond");
    let weights = |s: &BlendSpace, p: Vec2| {
        let (w, n) = s.weights(s.normalize(p));
        let mut v = vec![0.0; s.points.len()];
        for &(i, x) in &w[..n] {
            v[usize::from(i)] = x;
        }
        (v, n)
    };
    // At a sample: that sample alone.
    for (i, p) in [(0, Vec2::ZERO), (1, Vec2::new(0.0, 2.0)), (3, Vec2::new(2.0, 0.0))] {
        let (v, _) = weights(&plane, p);
        assert!((v[i] - 1.0).abs() < 1e-5, "{v:?}");
    }
    // Inside: barycentric, at most three, summing to one, reproducing the point.
    for p in [Vec2::new(0.5, 0.7), Vec2::new(-1.2, -0.3), Vec2::new(0.9, -0.9)] {
        let (v, n) = weights(&plane, p);
        assert!(n <= 3);
        assert!((v.iter().sum::<f32>() - 1.0).abs() < 1e-5);
        let at = [Vec2::ZERO, Vec2::new(0.0, 2.0), Vec2::new(0.0, -2.0), Vec2::new(2.0, 0.0), Vec2::new(-2.0, 0.0)];
        let back: Vec2 = v.iter().zip(at).map(|(w, a)| a * *w).sum();
        assert!(back.distance(p) < 1e-4, "{p} -> {back}");
    }
    // Outside the hull: projected onto the nearest edge (two samples).
    let (v, n) = weights(&plane, Vec2::new(3.0, 3.0));
    assert_eq!(n, 2);
    assert!((v[1] - 0.5).abs() < 1e-4 && (v[3] - 0.5).abs() < 1e-4, "{v:?}");
    // 1D: sorted segments, clamped ends.
    let line = space("line");
    assert_eq!(line.edges.len(), 2);
    let (v, _) = weights(&line, Vec2::new(2.0, 0.0));
    assert!((v[0] - 0.5).abs() < 1e-5 && (v[1] - 0.5).abs() < 1e-5);
    let (v, n) = weights(&line, Vec2::new(9.0, 0.0));
    assert_eq!((n, v[1]), (1, 1.0));
    // Collinear 2D samples become segments.
    let (tris, edges) = triangulate(&[Vec2::ZERO, Vec2::new(1.0, 1.0), Vec2::new(0.5, 0.5)]).unwrap();
    assert!(tris.is_empty());
    assert_eq!(edges, vec![[0, 2], [2, 1]]);
}

// --- Evaluation --------------------------------------------------------------------------

#[test]
fn state_machine_transitions_triggers_and_crossfades() {
    let rig = Rig::new(MAIN);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let sm = rig.graph.node("locomotion").unwrap();
    let dt = 1.0 / 60.0;
    for _ in 0..10 {
        step(&rig, &mut inst, &mut out, dt);
    }
    assert_eq!(inst.current_state(sm), Some(0));

    // idle → move crossfades over 0.3 s with weights summing to one.
    inst.params.set_bool(rig.param("moving"), true);
    let mut last = 0.0;
    for k in 0..30 {
        step(&rig, &mut inst, &mut out, dt);
        let w = inst.state_weights(sm);
        let total: f32 = w.iter().map(|x| x.1).sum();
        assert!((total - 1.0).abs() < 1e-5, "{w:?}");
        let target = w.iter().find(|x| x.0 == 1).unwrap().1;
        assert!(target >= last, "frame {k}: {target} < {last}");
        last = target;
    }
    assert_eq!(inst.current_state(sm), Some(1));
    assert!(inst.state_weights(sm).len() == 1, "{:?}", inst.state_weights(sm));

    // A trigger fires the any-state transition once and is consumed.
    inst.params.fire(rig.param("attack"));
    step(&rig, &mut inst, &mut out, dt);
    assert_eq!(inst.current_state(sm), Some(2));
    assert!(inst.inertializing());
    assert_eq!(inst.params.get(rig.param("attack"))[0], 0.0);
    let hit = rig.graph.event("hit").unwrap();
    let combo = rig.graph.event("combo").unwrap();
    let mut hits = 0;
    let mut combo_edges = Vec::new();
    let mut frames = 0;
    while inst.current_state(sm) == Some(2) {
        assert!(out.drive_weight > 0.99);
        hits += out.events.iter().filter(|e| e.id == hit).count();
        combo_edges.extend(out.events.iter().filter(|e| e.id == combo).map(|e| e.edge));
        step(&rig, &mut inst, &mut out, dt);
        frames += 1;
        assert!(frames < 100, "attack never ends");
    }
    // TimeLeft(0.1) of a 0.8 s clip: about 0.7 s in the state.
    assert!((40..=44).contains(&frames), "{frames}");
    assert_eq!(hits, 1);
    assert_eq!(combo_edges, [EventEdge::Enter, EventEdge::Exit]);
    // Back to idle (moving is still set, so on to move next frame).
    assert_eq!(inst.current_state(sm), Some(0));
    step(&rig, &mut inst, &mut out, dt);
    assert_eq!(inst.current_state(sm), Some(1));
}

/// Foot positions of a run through a state machine that switches walk →
/// run at `switch` and back at `back` (if any), with `blend`.
fn switch_run(blend: &str, dt: f32, switch: f32, back: Option<f32>, total: f32) -> Vec<Vec3> {
    let ron = format!(
        r#"(
        params: [(name: "go", kind: Bool(false))],
        nodes: {{
            "sm": StateMachine(states: [(name: "walk", node: "w"), (name: "run", node: "r")],
                transitions: [(from: "walk", to: "run", when: [IsTrue("go")], blend: {blend}),
                              (from: "run", to: "walk", when: [IsFalse("go")], blend: {blend})]),
            "w": Clip(clip: "walk"),
            "r": Clip(clip: "run", start: 0.3),
        }},
        layers: [(node: "sm")],
    )"#
    );
    let rig = Rig::new(&ron);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let (foot, hand) = (rig.joint("foot_l"), rig.joint("hand_l"));
    let go = rig.param("go");
    let n = (total / dt).round() as usize;
    let mut track = Vec::with_capacity(n);
    for k in 0..n {
        let t = k as f32 * dt;
        inst.params.set_bool(go, t + 1e-6 >= switch && back.is_none_or(|bk| t + 1e-6 < bk));
        inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
        track.push(out.model.translation(foot) + out.model.translation(hand));
    }
    track
}

/// Largest pose jump and velocity jump within `window` frames of frame `at`.
fn jumps(track: &[Vec3], dt: f32, at: usize, window: usize) -> (f32, f32) {
    let mut pose: f32 = 0.0;
    let mut vel: f32 = 0.0;
    for k in at.saturating_sub(window).max(2)..(at + window).min(track.len()) {
        pose = pose.max(track[k].distance(track[k - 1]));
        let (v1, v0) = ((track[k] - track[k - 1]) / dt, (track[k - 1] - track[k - 2]) / dt);
        vel = vel.max(v1.distance(v0));
    }
    (pose, vel)
}

#[test]
fn inertialized_switches_are_continuous_in_pose_and_velocity() {
    let (coarse, fine) = (1.0 / 60.0, 1.0 / 240.0);
    let at = |dt: f32, t: f32| (t / dt).round() as usize;
    for (switch, back) in [(0.5, None), (0.5, Some(0.62))] {
        let a = switch_run("Inertialize(0.3)", coarse, switch, back, 1.2);
        let b = switch_run("Inertialize(0.3)", fine, switch, back, 1.2);
        let cut = switch_run("Cut", fine, switch, back, 1.2);
        for t in std::iter::once(switch).chain(back) {
            let (pa, va) = jumps(&a, coarse, at(coarse, t), 2);
            let (pb, vb) = jumps(&b, fine, at(fine, t), 2);
            let (pc, _) = jumps(&cut, fine, at(fine, t), 2);
            // Pose: the per-frame step is speed × dt (no jump), so it
            // shrinks with dt; a cut jumps by the full pose difference.
            assert!(pa / coarse < 8.0 && pb / fine < 8.0, "pose step {pa} @60 Hz, {pb} @240 Hz at {t}");
            assert!(pc > 10.0 * pb, "cut {pc} vs inertialized {pb}");
            // Velocity: the per-frame change is acceleration × dt, so it
            // shrinks with dt too (a velocity discontinuity wouldn't).
            assert!(vb < va * 0.45, "velocity jump {va} @60 Hz, {vb} @240 Hz at {t}");
        }
    }
    // After the blend the output is the new state's pose exactly.
    let rig = Rig::new(
        r#"(params: [(name: "go", kind: Bool(false))], nodes: {"sm": StateMachine(states: [(name: "a", node: "w"), (name: "b", node: "r")],
        transitions: [(from: "a", to: "b", when: [IsTrue("go")], blend: Inertialize(0.2))]), "w": Clip(clip: "walk"), "r": Clip(clip: "run")},
        layers: [(node: "sm")])"#,
    );
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    for k in 0..40 {
        if k == 10 {
            inst.params.set_bool(rig.param("go"), true);
        }
        inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs::default(), &mut out);
    }
    assert!(!inst.inertializing());
    let (lanes, _) = inst.player(rig.graph.node("r").unwrap()).unwrap();
    let mut want = LocalPose::new(rig.skeleton.len());
    rig.clips[rig.graph.clips.iter().position(|c| c.reference == "run").unwrap()].sample(lanes[0].0, &mut want, None);
    assert_eq!(out.pose, want);
}

#[test]
fn sync_groups_align_markers_and_fire_footsteps_once() {
    let ron = r#"(
        params: [(name: "a", kind: Float(0.3))],
        nodes: {
            "mix": Blend(a: "w", b: "r", alpha: "a"),
            "w": Clip(clip: "walk", sync: (group: "feet")),
            "r": Clip(clip: "run", sync: (group: "feet")),
        },
        layers: [(node: "mix")],
    )"#;
    let rig = Rig::new(ron);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let (w, r) = (rig.graph.node("w").unwrap(), rig.graph.node("r").unwrap());
    let time = |inst: &GraphInstance, n: u32| inst.player(n).unwrap().0[0].0;
    // walk: down.l 0.0, down.r 0.5 (1.0 s); run: down.l 0.0, down.r 0.2 (0.6 s).
    let expected_run = |tw: f32| {
        if tw < 0.5 { tw / 0.5 * 0.2 } else { 0.2 + (tw - 0.5) / 0.5 * 0.4 }
    };
    let step_id = rig.graph.event("step").unwrap();
    let mut steps = 0;
    let dt = 1.0 / 60.0;
    for k in 0..240 {
        if k == 120 {
            inst.params.set_float(rig.param("a"), 0.8);
        }
        inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
        let (tw, tr) = (time(&inst, w), time(&inst, r));
        if k < 120 {
            // Walk leads (0.7): run sits at the same fraction between the same markers.
            assert!((tr - expected_run(tw)).abs() < 1e-4, "frame {k}: walk {tw} run {tr}");
            assert!((tw - (k + 1) as f32 * dt).rem_euclid(1.0) < 1e-3 || (tw - (k + 1) as f32 * dt).rem_euclid(1.0) > 0.999);
        } else {
            // Run leads (0.8) and walk follows.
            let expected_walk = if tr < 0.2 { tr / 0.2 * 0.5 } else { 0.5 + (tr - 0.2) / 0.4 * 0.5 };
            assert!((tw - expected_walk).abs() < 1e-4, "frame {k}: walk {tw} run {tr}");
        }
        if k < 120 {
            steps += out.events.iter().filter(|e| e.id == step_id).count();
        }
    }
    // Two seconds of walk-led blend: four footsteps, not eight.
    assert_eq!(steps, 4);

    // Without markers, followers keep the same normalized phase.
    let ron = r#"(nodes: {"mix": Blend(a: "i", b: "w", alpha: 0.4), "i": Clip(clip: "idle", sync: (group: "g")),
        "w": Clip(clip: "wave", sync: (group: "g", role: AlwaysLead))}, layers: [(node: "mix")])"#;
    let rig = Rig::new(ron);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    for _ in 0..50 {
        inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
    }
    let (ti, tw) = (time(&inst, rig.graph.node("i").unwrap()), time(&inst, rig.graph.node("w").unwrap()));
    assert!((tw - 50.0 * dt).abs() < 1e-4, "the always-leader plays at its own rate: {tw}");
    assert!((ti / 2.0 - tw / 1.0).abs() < 1e-4, "idle {ti} vs wave {tw}");
}

#[test]
fn layers_masks_and_additive_compose() {
    let rig = Rig::new(MAIN);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let dt = 1.0 / 30.0;
    step(&rig, &mut inst, &mut out, dt);
    let base = out.pose.clone();
    let mut inst2 = inst.clone();
    let mut out2 = GraphOutput::new(&rig.graph);
    inst2.params.set_float(rig.param("upper"), 1.0);
    step(&rig, &mut inst, &mut out, dt);
    step(&rig, &mut inst2, &mut out2, dt);
    let _ = base;
    let wave_clip = &rig.clips[rig.graph.clips.iter().position(|c| c.reference == "wave").unwrap()];
    let (lanes, _) = inst2.player(rig.graph.node("wave_clip").unwrap()).unwrap();
    let mut wave_pose = LocalPose::new(rig.skeleton.len());
    wave_clip.sample(lanes[0].0, &mut wave_pose, None);
    let head = rig.joint("head");
    let leg = rig.joint("thigh_l");
    let spine = rig.joint("spine");
    // Mask 1: the layer's pose; mask 0: the base's; mask 0.5: halfway.
    assert!(sf_anim::angle_between(out2.pose.get(head).rotation, wave_pose.get(head).rotation) < 1e-4);
    assert_eq!(out2.pose.get(leg), out.pose.get(leg));
    let half = sf_anim::nlerp(out.pose.get(spine).rotation, wave_pose.get(spine).rotation, 0.5);
    assert!(sf_anim::angle_between(out2.pose.get(spine).rotation, half) < 1e-4);

    // The additive lean rotates the spine by its delta on top.
    let mut inst3 = inst.clone();
    let mut out3 = GraphOutput::new(&rig.graph);
    let mut inst4 = inst.clone();
    let mut out4 = GraphOutput::new(&rig.graph);
    inst3.params.set_float(rig.param("lean"), 1.0);
    step(&rig, &mut inst3, &mut out3, dt);
    step(&rig, &mut inst4, &mut out4, dt);
    let want = Quat::from_rotation_z(0.3) * out4.pose.get(spine).rotation;
    assert!(sf_anim::angle_between(out3.pose.get(spine).rotation, want) < 1e-3);
    assert_eq!(out3.pose.get(leg), out4.pose.get(leg));
}

#[test]
fn root_motion_blends_drive_players_only() {
    let ron = r#"(
        params: [(name: "a", kind: Float(0.5))],
        nodes: {
            "mix": Blend(a: "x", b: "y", alpha: "a"),
            "x": Clip(clip: "attack", root_motion: Drive),
            "y": Clip(clip: "attack", root_motion: Drive, rate: 2.0),
            "walk": Clip(clip: "walk"),
            "both": Blend(a: "mix", b: "walk", alpha: 0.5),
        },
        layers: [(node: "both")],
    )"#;
    let rig = Rig::new(ron);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let dt = 0.1;
    inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
    // Attack moves 0.6 m/s. x at rate 1 moves 0.06, y at rate 2 moves 0.12:
    // mixed 0.09, then half of the pose is a Match walk (no drive).
    assert!((out.root.translation.z - 0.045).abs() < 2e-3, "{:?}", out.root);
    assert!((out.drive_weight - 0.5).abs() < 1e-5);
    // The walk alone drives nothing: the mover owns a Match character.
    let walk = Rig::new(r#"(nodes: {"w": Clip(clip: "walk")}, layers: [(node: "w")])"#);
    let wb = walk.binding();
    let mut wi = GraphInstance::new(&wb);
    let mut wo = GraphOutput::new(&walk.graph);
    wi.update(&wb, dt, 0, &UpdateInputs::default(), &mut wo);
    assert_eq!((wo.root, wo.drive_weight), (RootDelta::ZERO, 0.0));
}

#[test]
fn lod3_advances_clocks_events_and_root_motion_without_a_pose() {
    let rig = Rig::new(r#"(nodes: {"a": Clip(clip: "attack")}, layers: [(node: "a")])"#);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let mut hits = 0;
    let mut dist = 0.0;
    for _ in 0..20 {
        inst.update(&b, 0.05, 3, &UpdateInputs::default(), &mut out);
        assert!(!out.posed);
        hits += out.events.iter().filter(|e| e.edge == EventEdge::Point).count();
        dist += out.root.translation.z;
    }
    assert_eq!(hits, 1);
    assert!((dist - 0.48).abs() < 2e-3, "{dist}");
    let (lanes, _) = inst.player(rig.graph.node("a").unwrap()).unwrap();
    assert_eq!(lanes[0].0, 0.8);
}

#[test]
fn evaluation_is_identical_on_one_and_many_threads() {
    let rig = Rig::new(MAIN);
    let b = rig.binding();
    let run = |threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| {
            (0..48)
                .into_par_iter()
                .map(|i| {
                    let mut inst = GraphInstance::new(&b);
                    let mut out = GraphOutput::new(&rig.graph);
                    let mut trace = Vec::new();
                    for k in 0..90 {
                        let t = k as f32 / 60.0;
                        inst.params.set_float(rig.param("speed"), 1.2 + 2.4 * ((t + i as f32 * 0.1).sin() * 0.5 + 0.5));
                        inst.params.set_bool(rig.param("moving"), (k / 20 + i) % 3 != 0);
                        inst.params.set_float(rig.param("upper"), (i % 4) as f32 / 3.0);
                        if k == 37 + i % 5 {
                            inst.params.fire(rig.param("attack"));
                        }
                        inst.update(&b, 1.0 / 60.0 + (i % 3) as f32 * 1e-3, i % 3, &UpdateInputs::default(), &mut out);
                        trace.push((out.pose.to_vec(), out.root, out.events.len(), out.curves.clone()));
                    }
                    trace
                })
                .collect::<Vec<_>>()
        })
    };
    let one = run(1);
    let many = run(8);
    assert!(one == many, "results differ between thread counts");
}

#[test]
fn mirror_reflects_poses_and_root_motion() {
    let ron = r#"(
        params: [(name: "m", kind: Float(1.0))],
        nodes: {"mir": Mirror(input: "w", weight: "m"), "w": Clip(clip: "walk")},
        mirror: (pairs: [("thigh_l", "thigh_r"), ("shin_l", "shin_r"), ("foot_l", "foot_r"),
                         ("upperarm_l", "upperarm_r"), ("forearm_l", "forearm_r"), ("hand_l", "hand_r")]),
        layers: [(node: "mir")],
    )"#;
    let rig = Rig::new(ron);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let mut plain_inst = inst.clone();
    let mut plain = GraphOutput::new(&rig.graph);
    plain_inst.params.set_float(rig.param("m"), 0.0);
    for _ in 0..7 {
        inst.update(&b, 1.0 / 30.0, 0, &UpdateInputs::default(), &mut out);
        plain_inst.update(&b, 1.0 / 30.0, 0, &UpdateInputs::default(), &mut plain);
    }
    let flip = |v: Vec3| Vec3::new(-v.x, v.y, v.z);
    for (a, bb) in [("foot_l", "foot_r"), ("hand_l", "hand_r"), ("head", "head")] {
        let (m, p) = (out.model.translation(rig.joint(a)), plain.model.translation(rig.joint(bb)));
        assert!(m.distance(flip(p)) < 1e-4, "{a}: {m} vs mirrored {p}");
    }
    // The rest pose mirrors onto itself.
    let table = rig.graph.mirror.as_ref().unwrap();
    let rest = LocalPose::rest(&rig.skeleton);
    let mut mirrored = LocalPose::new(rig.skeleton.len());
    let (mut m1, mut m2) = (sf_anim::ModelPose::new(0), sf_anim::ModelPose::new(0));
    mirror::mirror_pose(&rig.skeleton, table, &rest, &mut mirrored, &mut m1, &mut m2);
    for jn in 0..rig.skeleton.len() {
        let (x, y) = (mirrored.get(jn), rest.get(jn));
        assert!(x.translation.distance(y.translation) < 1e-5 && sf_anim::angle_between(x.rotation, y.rotation) < 1e-4, "joint {jn}");
    }
    let r = mirror::mirror_root(asset::Axis::X, RootDelta::new(Vec3::new(1.0, 0.0, 2.0), 0.3));
    assert_eq!((r.translation, r.yaw), (Vec3::new(-1.0, 0.0, 2.0), -0.3));
}

#[test]
fn subgraphs_inline_and_merge_parameters() {
    let s = biped();
    let mut src = source(&s);
    src.subgraphs.insert(
        "upper.animgraph.ron".into(),
        r#"(params: [(name: "w", kind: Float(0.0))], nodes: {"x": Blend(a: "i", b: "v", alpha: "w"), "i": Clip(clip: "idle"), "v": Clip(clip: "wave")},
            layers: [(node: "x")])"#
            .into(),
    );
    let ron = r#"(params: [(name: "w", kind: Float(0.0))], nodes: {"sub": Subgraph(graph: "upper.animgraph.ron")}, layers: [(node: "sub")])"#;
    let g = compile_str("outer", ron, &s, &mut src).unwrap();
    assert_eq!(g.params.len(), 1);
    assert_eq!(g.clips.len(), 2);
    assert!(g.node("sub/x/v").is_some(), "{:?}", g.nodes.iter().map(|n| &n.name).collect::<Vec<_>>());
    // A subgraph of the wrong parameter type is an error.
    let ron = r#"(params: [(name: "w", kind: Bool(false))], nodes: {"sub": Subgraph(graph: "upper.animgraph.ron")}, layers: [(node: "sub")])"#;
    assert!(compile_str("outer", ron, &s, &mut src).is_err());
}

// --- Procedural ops ----------------------------------------------------------------------

const PROC: &str = r#"(
    params: [(name: "hand", kind: Float(1.0))],
    nodes: {"w": Clip(clip: "walk")},
    layers: [(node: "w")],
    legs: [
        (name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l"), contact: "contact.foot_l"),
        (name: "r", hip: Joint("thigh_r"), knee: Joint("shin_r"), foot: Joint("foot_r")),
    ],
    procedural: [OPS],
)"#;

fn proc_rig(ops: &str) -> Rig {
    Rig::new(&PROC.replace("OPS", ops))
}

#[test]
fn speed_match_scales_playback_within_limits() {
    let rig = proc_rig("SpeedMatch()");
    let b = rig.binding();
    let w = rig.graph.node("w").unwrap();
    for (speed, rate) in [(1.5, 1.25), (3.0, 1.3), (0.3, 0.75)] {
        let mut inst = GraphInstance::new(&b);
        let mut out = GraphOutput::new(&rig.graph);
        let inputs = UpdateInputs { velocity: Vec3::new(0.0, 0.0, speed), ..UpdateInputs::default() };
        inst.update(&b, 0.1, 0, &inputs, &mut out);
        let t = inst.player(w).unwrap().0[0].0;
        assert!((t - 0.1 * rate).abs() < 1e-4, "speed {speed}: time {t}");
        // Beyond SpeedMatch's LOD the clip plays at its own rate.
        let mut inst = GraphInstance::new(&b);
        inst.update(&b, 0.1, 3, &inputs, &mut out);
        assert!((inst.player(w).unwrap().0[0].0 - 0.1).abs() < 1e-6);
    }
}

#[test]
fn foot_placement_follows_ground_and_locks_contacts() {
    let rig = proc_rig(r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"])"#);
    let plain = proc_rig("");
    let (b, pb) = (rig.binding(), plain.binding());
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let mut pinst = GraphInstance::new(&pb);
    let mut pout = GraphOutput::new(&plain.graph);
    let (fl, fr) = (rig.joint("foot_l"), rig.joint("foot_r"));
    let up = Vec3::Y;
    let dt = 1.0 / 60.0;
    // The left foot stands on flat ground; the right on a 0.15 m step.
    let ground = [Some(GroundSample { point: Vec3::ZERO, normal: up }), Some(GroundSample { point: Vec3::new(0.0, 0.15, 0.0), normal: up })];
    let mut locked = None;
    for k in 0..20 {
        // The character walks forward 2 cm per frame.
        let moved = RootDelta::new(Vec3::new(0.0, 0.0, 0.02), 0.0);
        let inputs = UpdateInputs { ground: &ground, moved, ..UpdateInputs::default() };
        inst.update(&b, dt, 0, &inputs, &mut out);
        pinst.update(&pb, dt, 0, &UpdateInputs::default(), &mut pout);
        let (l, r) = (out.model.translation(fl), out.model.translation(fr));
        // While the contact curve holds (first 0.32 s), the left foot stays put
        // in the world: in character space it slides back by the motion.
        let contact = out.curves[rig.graph.curve("contact.foot_l").unwrap()];
        if contact > 0.5 {
            let at = locked.get_or_insert(l);
            assert!(l.distance(*at) < 2e-3, "frame {k}: {l} vs lock {at}");
            *at -= Vec3::new(0.0, 0.0, 0.02);
        }
        if k > 15 {
            // The right foot (no contact curve, never locked) is raised by the step.
            let pr = pout.model.translation(fr);
            assert!((r.y - (pr.y + 0.15)).abs() < 2e-3, "right {r} vs animated {pr}");
        }
    }
    assert_eq!(out.probe_origins.len(), 2);
}

/// The walkers' blow-up (animfix 2026-10-12): legs far behind the body,
/// pelvis on the ground. Trigger: the character travels much faster than
/// its cycle (avoidance pushes, a stuck walker sliding, a gait whose speed
/// match is clamped), so a planted foot's lock drifts beyond the leg's
/// reach; the reach cap then pulled the pelvis down to the lock's height.
/// Without the guards the test reproduces it; with them the pelvis stays
/// within the drop limit and every foot within its leg's reach.
#[test]
fn foot_placement_guards_stop_lock_drift_teleports_and_bad_samples() {
    let unguarded = r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"],
                        guards: (max_step: 100.0, max_drop: 100.0, max_rise: 100.0, unplant: 100.0, max_speed: 1000.0))"#;
    let guarded = r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"])"#;
    let flat = [Some(GroundSample { point: Vec3::ZERO, normal: Vec3::Y }); 2];
    // Worst pelvis drop below the plain animation, and worst hip-to-foot
    // excess over the leg's length, while sliding at 6 m/s (cycle: 1.2 m/s).
    let run = |ops: &str, ground: &[Option<GroundSample>], moved: &dyn Fn(usize) -> RootDelta| {
        let (rig, plain) = (proc_rig(ops), proc_rig(""));
        let (b, pb) = (rig.binding(), plain.binding());
        let (mut inst, mut out) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
        let (mut pinst, mut pout) = (GraphInstance::new(&pb), GraphOutput::new(&plain.graph));
        let (hips, thigh, foot) = (rig.joint("hips"), rig.joint("thigh_l"), rig.joint("foot_l"));
        let rest = rig.skeleton.rest_model();
        let leg = Vec3::from(rest[thigh].translation).distance(Vec3::from(rest[foot].translation));
        let (mut drop, mut stretch, mut stats) = (0.0f32, 0.0f32, FootStats::default());
        for k in 0..120 {
            let inputs = UpdateInputs { ground, moved: moved(k), velocity: Vec3::Z * 6.0, ..UpdateInputs::default() };
            inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
            pinst.update(&pb, 1.0 / 60.0, 0, &UpdateInputs::default(), &mut pout);
            assert!(out.model.translation(hips).is_finite());
            drop = drop.max(pout.model.translation(hips).y - out.model.translation(hips).y);
            stretch = stretch.max(out.model.translation(foot).distance(out.model.translation(thigh)) - leg);
            stats += out.feet;
        }
        (drop, stretch, leg, stats)
    };
    let slide = |_| RootDelta::new(Vec3::new(0.0, 0.0, 0.1), 0.0);
    let (drop, _, leg, _) = run(unguarded, &flat, &slide);
    assert!(drop > 0.5 * leg, "unguarded: lock drift drops the pelvis {drop} (leg {leg})");
    let (drop, stretch, leg, stats) = run(guarded, &flat, &slide);
    assert!(drop <= 0.4 * leg + 1e-3, "guarded drop {drop} (leg {leg})");
    assert!(stretch < 2e-3, "guarded leg stretch {stretch}");
    assert!(stats.unplanted > 0, "{stats:?}");

    // A teleport (5 m in one update) resets the locks instead of stretching legs to them.
    let jump = |k| RootDelta::new(Vec3::new(0.0, 0.0, if k == 10 { 5.0 } else { 0.02 }), 0.0);
    let (drop, stretch, leg, stats) = run(guarded, &flat, &jump);
    assert!(stats.teleports == 1 && drop <= 0.4 * leg + 1e-3 && stretch < 2e-3, "{stats:?} drop {drop} stretch {stretch}");

    // A probe hit 2 m below (through a floor, a stale ground) is rejected;
    // a non-finite one too. Neither moves the pelvis.
    let still = |_| RootDelta::new(Vec3::new(0.0, 0.0, 0.02), 0.0);
    for bad in [Vec3::new(0.0, -2.0, 0.0), Vec3::new(f32::NAN, 0.0, 0.0)] {
        let ground = [Some(GroundSample { point: bad, normal: Vec3::Y }); 2];
        let (drop, stretch, leg, stats) = run(guarded, &ground, &still);
        assert!(stats.rejected >= 2 * 120 && drop <= 0.4 * leg + 1e-3 && stretch < 2e-3, "{bad}: {stats:?} drop {drop}");
    }

    // No ground samples at all (probes off, dormant, in place): the feet
    // play the animation, no locks hold them.
    let (drop, stretch, _, stats) = run(guarded, &[None, None], &slide);
    assert!(drop.abs() < 1e-4 && stretch < 2e-3 && stats == FootStats::default(), "{stats:?} drop {drop}");
}

#[test]
fn look_at_and_hand_ik_reach_targets() {
    let ops = r#"LookAt(chain: [(Joint("chest"), 0.3, 0.6), (Joint("head"), 1.0, 1.2)], forward: (0.0, 0.0, 1.0), halflife: 0.01),
                 HandIk(upper: Joint("upperarm_l"), lower: Joint("forearm_l"), hand: Joint("hand_l"), target: "grip", weight: "hand")"#;
    let rig = proc_rig(ops);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let head = rig.joint("head");
    let look = Vec3::new(2.0, 1.65, 2.0);
    let grip = Vec3::new(0.4, 1.3, 0.25);
    let targets = [Some(IkTarget::from(grip))];
    let inputs = UpdateInputs { look_target: Some(look), ik_targets: &targets, ..UpdateInputs::default() };
    for _ in 0..30 {
        inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
    }
    let fwd = out.model.rotation(head) * Vec3::Z;
    let want = (look - out.model.translation(head)).normalize();
    assert!(fwd.angle_between(want) < 0.05, "head looks {fwd}, target {want}");
    assert!(out.model.translation(rig.joint("hand_l")).distance(grip) < 1e-3);
    // Beyond its LOD the look-at stops.
    inst.update(&b, 1.0 / 60.0, 1, &inputs, &mut out);
    assert!((out.model.rotation(head) * Vec3::Z).angle_between(want) > 0.3);
}

#[test]
fn stride_and_orientation_warping() {
    let rig = proc_rig(
        r#"SpeedMatch(), StrideWarp(pelvis: Joint("hips"), legs: ["l", "r"]),
           OrientationWarp(pelvis: Joint("hips"), spine: [(Joint("spine"), 0.5), (Joint("chest"), 0.5)], halflife: 0.001)"#,
    );
    let plain = proc_rig("SpeedMatch()");
    let (b, pb) = (rig.binding(), plain.binding());
    let (fl, fr, chest, hips) = (rig.joint("foot_l"), rig.joint("foot_r"), rig.joint("chest"), rig.joint("hips"));
    // Too fast for the rate clamp (2 m/s over 1.2 × 1.3): strides lengthen.
    let fast = UpdateInputs { velocity: Vec3::new(0.0, 0.0, 2.0), ..UpdateInputs::default() };
    let (mut i1, mut o1, mut i2, mut o2) =
        (GraphInstance::new(&b), GraphOutput::new(&rig.graph), GraphInstance::new(&pb), GraphOutput::new(&plain.graph));
    for _ in 0..12 {
        i1.update(&b, 1.0 / 60.0, 0, &fast, &mut o1);
        i2.update(&pb, 1.0 / 60.0, 0, &fast, &mut o2);
    }
    // Each foot lands on its target: scaled along the velocity about the
    // pelvis by the residual ratio, the pelvis lowered so both reach.
    let ratio = 2.0 / (1.2 * 1.3);
    let c = o2.model.translation(hips).z;
    for f in [fl, fr] {
        let p = o2.model.translation(f);
        let target = Vec3::new(p.x, p.y, p.z + (p.z - c) * (ratio - 1.0));
        assert!(o1.model.translation(f).distance(target) < 2e-3, "{} vs {target}", o1.model.translation(f));
    }
    assert!(o1.model.translation(hips).y < o2.model.translation(hips).y);

    // Moving diagonally: the hips turn toward the velocity, the chest keeps facing +Z.
    let diag = UpdateInputs { velocity: Vec3::new(1.2, 0.0, 1.2) / 2f32.sqrt(), ..UpdateInputs::default() };
    let (mut i1, mut o1) = (GraphInstance::new(&b), GraphOutput::new(&rig.graph));
    let (mut i2, mut o2) = (GraphInstance::new(&pb), GraphOutput::new(&plain.graph));
    for _ in 0..5 {
        i1.update(&b, 1.0 / 60.0, 0, &diag, &mut o1);
        i2.update(&pb, 1.0 / 60.0, 0, &diag, &mut o2);
    }
    let yaw = |q: Quat| {
        let f = q * Vec3::Z;
        f.x.atan2(f.z)
    };
    let turned = yaw(o1.model.rotation(hips)) - yaw(o2.model.rotation(hips));
    assert!((turned - std::f32::consts::FRAC_PI_4).abs() < 0.02, "hips turned {turned}");
    assert!((yaw(o1.model.rotation(chest)) - yaw(o2.model.rotation(chest))).abs() < 0.02);
}

// --- Bake step ---------------------------------------------------------------------------

#[derive(Default)]
struct Files(RwLock<HashMap<String, Vec<u8>>>);

impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.read().unwrap().get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0.read().unwrap().get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
    }
    fn record_fingerprint(&self, _kind: &str, _id: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

/// Stands in for `skeleton.cook` / `anim.cook`: products from a table.
struct Stub(&'static str, Arc<RwLock<HashMap<String, Vec<u8>>>>);

impl Step for Stub {
    fn id(&self) -> &'static str {
        self.0
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut sf_bake::PlanCx) -> Result<(), BakeError> {
        let bytes = self.1.read().unwrap().get(subject).cloned().unwrap_or_default();
        cx.param("bytes", Digest::of(&bytes).0.to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.1.read().unwrap().get(subject).cloned().ok_or_else(|| BakeError::step(&ProductKey::new(self.0, subject), "missing"))
    }
}

#[test]
fn animgraph_compile_step_bakes_caches_and_tracks_clips() {
    use crate::steps::{ANIMGRAPH_COMPILE, AnimGraphCompileStep, DefaultGraphRules, graph_subject};
    let s = biped();
    let skel_path = "chars/hero.glb";
    let products: Arc<RwLock<HashMap<String, Vec<u8>>>> = Arc::default();
    let anims: Arc<RwLock<HashMap<String, Vec<u8>>>> = Arc::default();
    products.write().unwrap().insert(skel_path.into(), s.to_bytes());
    let src = source(&s);
    for name in ["walk", "run"] {
        anims.write().unwrap().insert(sf_anim::steps::anim_subject(skel_path, "anims/loco.glb", name), src.clips[name].to_bytes());
    }
    let files = Arc::new(Files::default());
    let graph = r#"(params: [(name: "s", kind: Float(0.0))], clips: {"walk": "anims/loco.glb#walk", "run": "anims/loco.glb#run"},
        nodes: {"m": BlendSpace1D(x: "s", samples: [(clip: "walk", at: 1.2), (clip: "run", at: 3.6)])}, layers: [(node: "m")])"#;
    files.0.write().unwrap().insert("graphs/hero.animgraph.ron".into(), graph.as_bytes().to_vec());
    let dir = tempfile::tempdir().unwrap();
    let session = || {
        let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files.clone());
        baker.register(Stub(sf_anim::steps::SKELETON_COOK, products.clone())).register(Stub(sf_anim::steps::ANIM_COOK, anims.clone()));
        AnimGraphCompileStep::register(Arc::new(DefaultGraphRules), &mut baker);
        baker
    };
    let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject("graphs/hero.animgraph.ron", skel_path));
    let baker = session();
    let report = baker.bake(std::slice::from_ref(&key));
    assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
    let g = CompiledGraph::from_bytes(&baker.store().get(&baker.product(&key).unwrap()).unwrap()).unwrap();
    assert_eq!(g.clips.len(), 2);
    assert_eq!(g.clips[0].product.step, sf_anim::steps::ANIM_COOK);
    assert_eq!(g.skeleton, s.id());
    assert_eq!(baker.step_stats()[ANIMGRAPH_COMPILE].built, 1);

    // A warm session builds nothing and reproduces the product.
    let again = session();
    assert_eq!(again.bake(std::slice::from_ref(&key)).failures().count(), 0);
    assert_eq!(again.step_stats()[ANIMGRAPH_COMPILE].built, 0);
    assert!(again.verify(std::slice::from_ref(&key))[0].1.as_ref().unwrap());

    // Re-cooking a clip it plays recompiles the graph (a dynamic input).
    let faster = cycle(&s, "walk", 0.9, 1.3, 0.5, 0.45);
    anims.write().unwrap().insert(sf_anim::steps::anim_subject(skel_path, "anims/loco.glb", "walk"), faster.to_bytes());
    let third = session();
    assert_eq!(third.bake(std::slice::from_ref(&key)).failures().count(), 0);
    assert_eq!(third.step_stats()[ANIMGRAPH_COMPILE].built, 1);
    let g2 = CompiledGraph::from_bytes(&third.store().get(&third.product(&key).unwrap()).unwrap()).unwrap();
    assert!((g2.clips[0].duration - 0.9).abs() < 1e-6);

    // Errors stay per product.
    files.0.write().unwrap().insert("graphs/bad.animgraph.ron".into(), b"(layers: [(node: \"nope\")])".to_vec());
    assert!(third.product(&ProductKey::new(ANIMGRAPH_COMPILE, graph_subject("graphs/bad.animgraph.ron", skel_path))).is_err());
    assert!(third.product(&ProductKey::new(ANIMGRAPH_COMPILE, "no separator")).is_err());
}

/// Cost of one A0 update: two layers + an additive layer, a synced 1D
/// blend space under a state machine, foot placement, look-at and hand IK.
/// `cargo test --release -p sf_anim_graph --all-features -- --ignored --nocapture a0_update`
#[test]
#[ignore = "timing; run in release"]
fn a0_update_timing() {
    let ops = r#"
        legs: [
            (name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l"), contact: "contact.foot_l"),
            (name: "r", hip: Joint("thigh_r"), knee: Joint("shin_r"), foot: Joint("foot_r")),
        ],
        procedural: [
            SpeedMatch(), StrideWarp(pelvis: Joint("hips"), legs: ["l", "r"]),
            OrientationWarp(pelvis: Joint("hips"), spine: [(Joint("spine"), 0.5), (Joint("chest"), 0.5)]),
            FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"]),
            LookAt(chain: [(Joint("chest"), 0.3, 0.6), (Joint("head"), 1.0, 1.2)], forward: (0.0, 0.0, 1.0)),
            HandIk(upper: Joint("upperarm_l"), lower: Joint("forearm_l"), hand: Joint("hand_l"), target: "grip"),
        ],
    )"#;
    let full = MAIN.trim_end().trim_end_matches(')').to_owned() + ops;
    let rig = Rig::new(&full);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    inst.params.set_bool(rig.param("moving"), true);
    inst.params.set_float(rig.param("speed"), 2.0);
    inst.params.set_float(rig.param("upper"), 0.5);
    inst.params.set_float(rig.param("lean"), 0.5);
    let ground = [Some(GroundSample { point: Vec3::ZERO, normal: Vec3::Y }); 2];
    let targets = [Some(IkTarget::from(Vec3::new(0.4, 1.3, 0.25)))];
    let inputs = UpdateInputs {
        velocity: Vec3::new(0.3, 0.0, 2.2),
        ground: &ground,
        look_target: Some(Vec3::new(1.0, 1.6, 2.0)),
        ik_targets: &targets,
        ..UpdateInputs::default()
    };
    for _ in 0..200 {
        inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
    }
    let n = 20_000;
    let start = std::time::Instant::now();
    for _ in 0..n {
        inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
    }
    let us = start.elapsed().as_secs_f64() * 1e6 / f64::from(n);
    println!("A0 update ({} joints): {us:.2} µs", rig.skeleton.len());
}

#[test]
fn variety_offsets_looping_phase_and_jitters_rate_except_root_motion_drive() {
    let rig = Rig::new(MAIN);
    let b = rig.binding();
    let idle = rig.graph.node("idle_clip").unwrap();
    let time = |inst: &GraphInstance, n| inst.player(n).unwrap().0[0].0;
    let mut out = GraphOutput::new(&rig.graph);

    // Default: authored phase and rate.
    let mut plain = GraphInstance::new(&b);
    assert!(time(&plain, idle).abs() < 1e-6);
    step(&rig, &mut plain, &mut out, 0.1);
    assert!((time(&plain, idle) - 0.1).abs() < 1e-5);

    // A quarter of the 2 s cycle in, 4 % fast; shifting a running instance works too.
    let v = Variety { phase: 0.25, rate: 1.04 };
    let mut a = GraphInstance::new(&b);
    a.set_variety(&b, v);
    assert!((time(&a, idle) - 0.5).abs() < 1e-5, "{}", time(&a, idle));
    step(&rig, &mut a, &mut out, 0.1);
    assert!((time(&a, idle) - (0.5 + 0.1 * 1.04)).abs() < 1e-5);
    let mut late = GraphInstance::new(&b);
    step(&rig, &mut late, &mut out, 0.1);
    late.set_variety(&b, v);
    assert!((time(&late, idle) - 0.6).abs() < 1e-5, "{}", time(&late, idle));
    // A wrap keeps the playhead inside the cycle.
    late.set_variety(&b, Variety { phase: 0.9, rate: 1.0 });
    assert!((0.0..2.0).contains(&time(&late, idle)));

    // Root-motion Drive clips (the attack) keep their rate, and are not looping so keep their start.
    let attack = rig.graph.node("attack_clip").unwrap();
    step(&rig, &mut a, &mut out, 0.05);
    a.params.fire(rig.param("attack"));
    step(&rig, &mut a, &mut out, 0.1);
    let t = time(&a, attack);
    assert!(t > 0.0 && t <= 0.1 + 1e-5, "attack time {t}");
}

mod optional;
mod slot;

/// A masked override layer blends a leg's contact curve by the mask weight
/// of the leg's foot (curve "linked bones"): an upper-body overlay keeps
/// the base's contacts, a layer over the legs takes its own.
#[test]
fn masked_layers_keep_contacts_of_joints_they_leave_alone() {
    let graph = |mask: &str| {
        format!(
            r#"(
    masks: {{
        "upper": (rules: [(joint: Joint("chest"), weight: 1.0)]),
        "legs": (rules: [(joint: Joint("hips"), weight: 1.0)]),
    }},
    nodes: {{"w": Clip(clip: "walk"), "r": Clip(clip: "run")}},
    layers: [(node: "w"), (node: "r", mask: "{mask}")],
    legs: [(name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l"), contact: "contact.foot_l")],
)"#
        )
    };
    let base = Rig::new(r#"(nodes: {"w": Clip(clip: "walk")}, layers: [(node: "w")])"#);
    let (upper, legs) = (Rig::new(&graph("upper")), Rig::new(&graph("legs")));
    let run = Rig::new(r#"(nodes: {"r": Clip(clip: "run")}, layers: [(node: "r")])"#);
    let mut sims: Vec<(&Rig, GraphInstance, GraphOutput)> =
        [&base, &upper, &legs, &run].into_iter().map(|r| (r, GraphInstance::new(&r.binding()), GraphOutput::new(&r.graph))).collect();
    let mut differ = 0;
    for _ in 0..60 {
        let mut c = Vec::new();
        for (rig, inst, out) in &mut sims {
            step(rig, inst, out, 1.0 / 60.0);
            c.push(out.curves[rig.graph.curve("contact.foot_l").unwrap()]);
        }
        assert!((c[1] - c[0]).abs() < 1e-6, "upper-body layer changed the walk's contact: {c:?}");
        assert!((c[2] - c[3]).abs() < 1e-6, "a layer over the legs brings its own contact: {c:?}");
        differ += usize::from((c[0] - c[3]).abs() > 0.5);
    }
    assert!(differ > 5, "walk and run contacts differ somewhere");
}

/// Followers of a looping clip whose loop sits inside it (an intro before,
/// an outro after, with foot markers in both) land inside the loop: placed
/// past the loop's end, a lane played out to the clip's end and parked
/// there once it led (walkers5: torch-bearers' walk lanes frozen at 1.333 s).
#[test]
fn synced_followers_stay_inside_their_loop() {
    let s = biped();
    let with = |loop_range: (f32, f32), markers: &[(f32, &str)]| {
        let mut c = cycle(&s, "c", 1.333, 1.5, 0.5, 0.5);
        c.loop_range = Some(loop_range);
        c.markers = markers.iter().map(|&(t, n)| marker(t, n)).collect();
        c
    };
    // WalkForward-like leader and WalkRight-like follower (Morrowind's data).
    let leader = with((0.2, 1.2), &[(0.0, "down.l"), (0.283, "down.r"), (0.808, "down.l"), (1.208, "down.r")]);
    let follower = with((0.133, 1.067), &[(0.0, "down.r"), (0.133, "down.l"), (0.617, "down.r"), (1.1, "down.l"), (1.267, "down.r")]);
    let mut current = 0.5;
    for k in 0..400 {
        let lt = 0.2 + (k as f32 * 0.0137).rem_euclid(1.0);
        let t = sync::sync_time(&follower, true, current, &leader, true, lt);
        assert!((0.0..1.067).contains(&t), "leader {lt}: follower at {t}, past its loop end 1.067");
        // As a leader, the follower keeps moving.
        let adv = follower.advance(t, 1.0 / 60.0, true, false, None);
        assert!(adv.time != t && !adv.finished, "leader {lt}: parked at {t}");
        current = t;
    }
    // Only the loop's markers count, and pairs wrap at the loop's end:
    // the leader at 1.0 is 0.192 s into its l 0.808 → (wrap) r 0.283 gap
    // of 0.475 s; the follower sits as far into its l 0.133 → r 0.617.
    let t = sync::sync_time(&follower, true, 0.3, &leader, true, 1.0);
    let expected = 0.133 + 0.192 / 0.475 * (0.617 - 0.133);
    assert!((t - expected).abs() < 1e-3, "follower at {t}, expected {expected}");
}

// --- Physics input and hand-back (P8) ------------------------------------------------------

/// A physics pose (the rest pose swaying, standing in for a ragdoll) shown
/// at weight 1 until `switch`, then handed back to the graph's clip, with
/// `inertialize_from` or by just dropping the input.
fn handback_run(dt: f32, switch: f32, total: f32, inertialize: bool) -> Vec<Vec3> {
    let rig = Rig::new(r#"(params: [], nodes: {"w": Clip(clip: "run")}, layers: [(node: "w")])"#);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let (foot, hand) = (rig.joint("foot_l"), rig.joint("hand_l"));
    let rest = rig.skeleton.rest().to_vec();
    let physics = |t: f32| -> Vec<Trs> {
        rest.iter()
            .map(|&x| Trs { rotation: (x.rotation * Quat::from_rotation_x(0.8 * (1.5 * t).sin())).normalize(), ..x })
            .collect()
    };
    let ones = vec![1.0; rest.len()];
    let mut track = Vec::new();
    for k in 0..(total / dt).round() as usize {
        let t = k as f32 * dt;
        if t + 1e-6 < switch {
            inst.set_physics(Some((&physics(t), &ones)));
        } else if inst.has_physics() {
            inst.set_physics(None);
            if inertialize {
                inst.inertialize_from(&physics(t - dt), &physics(t - 2.0 * dt), dt, 0.4);
            }
        }
        inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
        track.push(out.model.translation(foot) + out.model.translation(hand));
    }
    track
}

#[test]
fn physics_handback_is_continuous_at_shrinking_dt() {
    let (coarse, fine, switch) = (1.0 / 60.0, 1.0 / 240.0, 0.5);
    let at = |dt: f32| (switch / dt).round() as usize;
    let a = handback_run(coarse, switch, 1.0, true);
    let b = handback_run(fine, switch, 1.0, true);
    let cut = handback_run(fine, switch, 1.0, false);
    let (pa, va) = jumps(&a, coarse, at(coarse), 2);
    let (pb, vb) = jumps(&b, fine, at(fine), 2);
    let (pc, _) = jumps(&cut, fine, at(fine), 2);
    eprintln!("hand-back: pose step {pa:.4}/{pb:.4} m (60/240 Hz), cut {pc:.4}; velocity jump {va:.3}/{vb:.3} m/s");
    assert!(pa / coarse < 8.0 && pb / fine < 8.0, "pose step {pa} @60 Hz, {pb} @240 Hz");
    assert!(pc > 10.0 * pb, "cut {pc} vs inertialized {pb}");
    assert!(vb < va * 0.45, "velocity jump {va} @60 Hz, {vb} @240 Hz");
}

/// R0: procedural ops leave joints at physics weight 1 alone. Foot placement
/// on a step would otherwise lift a ragdoll's foot off its body.
#[test]
fn ops_skip_joints_physics_owns() {
    let rig = proc_rig(r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"])"#);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let up = Vec3::Y;
    let ground = [Some(GroundSample { point: Vec3::new(0.0, 0.2, 0.0), normal: up }), Some(GroundSample { point: Vec3::new(0.0, 0.15, 0.0), normal: up })];
    let rest = rig.skeleton.rest().to_vec();
    let ones = vec![1.0; rest.len()];
    let mut model = sf_anim::ModelPose::new(rest.len());
    sf_anim::local_to_model(&rig.skeleton, &LocalPose::rest(&rig.skeleton), &mut model, &glam::Affine3A::IDENTITY);
    for _ in 0..20 {
        inst.set_physics(Some((&rest, &ones)));
        let inputs = UpdateInputs { ground: &ground, ..UpdateInputs::default() };
        inst.update(&b, 1.0 / 60.0, 0, &inputs, &mut out);
    }
    for (j, &r) in rest.iter().enumerate() {
        let a = out.pose.get(j);
        assert!(a.translation.distance(r.translation) < 1e-5 && a.rotation.angle_between(r.rotation) < 1e-5, "{}: ops moved it", rig.skeleton.name(j));
        assert!(out.model.translation(j).distance(model.translation(j)) < 1e-5, "{}: stale model", rig.skeleton.name(j));
    }
}
