//! B2a (animation.md open question 5): a `Clip` node whose playhead follows the clip's
//! `distance` curve to a parameter (UE5 Distance Matching), within its rate clamp.

use super::*;
use sf_anim::DISTANCE_CURVE;

const DT: f32 = 1.0 / 60.0;
const STOP_REST: f32 = 1.0;

/// A non-looping clip `d` s long with a `distance` curve `f(t)`.
fn distance_clip(s: &Skeleton, name: &str, d: f32, f: impl Fn(f32) -> f32) -> Clip {
    let src = SourceClip {
        name: name.into(),
        duration: d,
        tracks: vec![rot(d, "spine", move |t| Quat::from_rotation_x(0.1 * t))],
        curves: vec![SourceCurve { name: DISTANCE_CURVE.into(), curve: sampled(d, f) }],
        ..SourceClip::default()
    };
    cook_clip(s, &src, &rules(false, RootMotionMode::Ignore), &CookSettings::default()).unwrap()
}

/// The test clips plus `stop` (minus the metres left, 2 m -> 0 at 1 s, then at rest until
/// 1.4 s) and `start` (1.5 t² metres travelled over 1 s).
fn distance_source(s: &Skeleton) -> Source {
    let mut src = source(s);
    let stop = distance_clip(s, "stop", 1.4, |t| if t < STOP_REST { -2.0 * (1.0 - t / STOP_REST).powi(2) } else { 0.0 });
    let start = distance_clip(s, "start", 1.0, |t| 1.5 * t * t);
    for c in [stop, start] {
        src.clips.insert(c.name.clone(), c);
    }
    src
}

fn distance_rig(node: &str) -> Rig {
    let skeleton = biped();
    let mut src = distance_source(&skeleton);
    let ron = format!(r#"(params: [(name: "d", kind: Float(0.0))], nodes: {{ "m": {node} }}, layers: [(node: "m")])"#);
    let graph = compile_str("distance", &ron, &skeleton, &mut src).unwrap();
    let clips = graph.clips.iter().map(|c| Arc::new(src.clips[&c.reference].clone())).collect();
    Rig { skeleton, graph, clips }
}

/// The first frame time at which the clip's distance curve reaches 0 (the stop's rest frame).
fn rest_frame(rig: &Rig) -> f32 {
    let c = &rig.clips[0];
    let values = &c.curves[c.find_curve(DISTANCE_CURVE).unwrap()].values;
    values.iter().position(|&v| v >= 0.0).unwrap() as f32 / c.rate
}

/// The clip's own distance at `t` (the curve the playhead inverts).
fn curve_at(rig: &Rig, t: f32) -> f32 {
    let c = &rig.clips[0];
    c.curve(c.find_curve(DISTANCE_CURVE).unwrap(), t)
}

struct Run<'a> {
    rig: &'a Rig,
    inst: GraphInstance,
    out: GraphOutput,
}

impl<'a> Run<'a> {
    fn new(rig: &'a Rig) -> Self {
        let b = rig.binding();
        Self { rig, inst: GraphInstance::new(&b), out: GraphOutput::new(&rig.graph) }
    }

    /// Sets the distance, updates one tick and returns the playhead.
    fn step(&mut self, d: f32) -> f32 {
        self.inst.params.set_float(self.rig.param("d"), d);
        self.inst.update(&self.rig.binding(), DT, 0, &UpdateInputs::default(), &mut self.out);
        self.inst.player(self.rig.graph.node("m").unwrap()).unwrap().0[0].0
    }
}

#[test]
fn stop_follows_the_remaining_distance_and_rests_on_the_rest_frame() {
    let rig = distance_rig(r#"Clip(clip: "stop", distance: (value: "d"))"#);
    let rest = rest_frame(&rig);
    assert!((rest - STOP_REST).abs() < 0.05, "rest frame {rest}");
    let mut run = Run::new(&rig);
    assert_eq!(run.step(-2.0), 0.0, "still 2 m out: the first frame");
    // The character stops 10 % faster than the clip: the playhead is the curve's inverse.
    for k in 1..=80 {
        let t = (1.1 * k as f32 * DT).min(rest);
        let time = run.step(curve_at(&rig, t));
        assert!((time - t).abs() < 1e-4, "tick {k}: playhead {time}, inverse {t}");
    }
    // At the stop point it holds on the rest frame.
    for _ in 0..10 {
        assert!((run.step(0.0) - rest).abs() < 1e-4);
    }
}

#[test]
fn start_at_the_clips_own_pace_plays_at_normal_time() {
    let rig = distance_rig(r#"Clip(clip: "start", distance: (value: "d"))"#);
    let mut run = Run::new(&rig);
    for k in 1..=50 {
        let t = k as f32 * DT;
        let time = run.step(curve_at(&rig, t));
        assert!((time - t).abs() < 1e-4, "tick {k}: playhead {time}, clock {t}");
    }
    // Past the curve's last value: it heads for the end and holds there.
    let mut time = 0.0;
    for _ in 0..60 {
        time = run.step(10.0);
    }
    assert!((time - 1.0).abs() < 1e-5, "{time}");
}

#[test]
fn jumps_are_rate_clamped_and_the_playhead_never_goes_back() {
    let rig = distance_rig(r#"Clip(clip: "start", distance: (value: "d"))"#);
    let (lo, hi) = asset::distance_rate_clamp();
    let mut run = Run::new(&rig);
    // The distance jumps to the end: at most `hi` times the normal pace.
    for k in 1..=6 {
        let time = run.step(1.5);
        assert!((time - k as f32 * DT * hi).abs() < 1e-5, "tick {k}: {time}");
    }
    let at = 6.0 * DT * hi;
    // It goes back: the playhead holds.
    for _ in 0..5 {
        assert!((run.step(0.01) - at).abs() < 1e-6);
    }
    // A target barely ahead: at least `lo` times the normal pace.
    let time = run.step(curve_at(&rig, at + 0.1 * DT));
    assert!((time - (at + lo * DT)).abs() < 1e-5, "{time}");

    // A clamp of the node's own.
    let rig = distance_rig(r#"Clip(clip: "start", distance: (value: "d", rate_clamp: (0.5, 2.0)))"#);
    let mut run = Run::new(&rig);
    assert!((run.step(1.5) - 2.0 * DT).abs() < 1e-5);
}

#[test]
fn distance_matching_compiles_only_with_a_distance_curve() {
    let s = biped();
    let bad = |node: &str| {
        let ron = format!(r#"(params: [(name: "d", kind: Float(0.0))], nodes: {{ "m": {node} }}, layers: [(node: "m")])"#);
        compile_str("bad", &ron, &s, &mut distance_source(&s)).unwrap_err().message
    };
    let e = bad(r#"Clip(clip: "attack", distance: (value: "d"))"#);
    assert!(e.contains("\"attack\"") && e.contains("no \"distance\" curve"), "{e}");
    assert!(bad(r#"Clip(clip: "stop", distance: (value: "d", rate_clamp: (0.0, 1.0)))"#).contains("rate_clamp"));
    assert!(bad(r#"Clip(clip: "stop", looping: Some(true), distance: (value: "d"))"#).contains("non-looping"));
    assert!(bad(r#"Clip(clip: "stop", sync: (group: "g"), distance: (value: "d"))"#).contains("sync group"));
    assert!(bad(r#"Clip(clip: "stop", distance: (value: "nope"))"#).contains("unknown parameter"));

    // Without the field: no distance matching, and it round-trips through the asset unwritten.
    let rig = distance_rig(r#"Clip(clip: "stop")"#);
    let NodeKind::Player(p) = &rig.graph.nodes[rig.graph.node("m").unwrap() as usize].kind else { panic!() };
    assert_eq!(p.distance, None);
    let a = parse_graph(r#"(nodes: { "m": Clip(clip: "stop") }, layers: [(node: "m")])"#).unwrap();
    assert!(!ron::to_string(&a.nodes["m"]).unwrap().contains("distance"));
    // With it: the compiled form survives the byte round trip.
    let rig = distance_rig(r#"Clip(clip: "stop", distance: (value: "d"))"#);
    let NodeKind::Player(p) = &rig.graph.nodes[rig.graph.node("m").unwrap() as usize].kind else { panic!() };
    assert_eq!(p.distance.unwrap().rate_clamp, asset::distance_rate_clamp());
    let bytes = rig.graph.to_bytes();
    assert_eq!(CompiledGraph::from_bytes(&bytes).unwrap().to_bytes(), bytes);
}
