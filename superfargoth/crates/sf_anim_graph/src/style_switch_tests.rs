//! Tests of `GraphInstance::set_style` (AN14b2a): the runtime class switch
//! samples the class clips through one resolved map, keeps the phase and
//! inertializes a switch on a playing lane (animation-style.md "Class remap").
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use std::collections::HashMap;
use std::sync::Arc;

use glam::{Quat, Vec3};
use sf_anim::{
    Clip, ClipRules, ContactRules, CookSettings, Curve, Event, LocalPose, Looping, RootMotionMode, RootMotionRules, Skeleton, SkeletonRules, SourceClip,
    SourceJoint, SourceTrack, SyncMarker, Trs, cook_clip,
};
use sf_bake::ProductKey;

use crate::asset::{GraphAsset, parse_graph};
use crate::style::{ClassName, StyleTable};
use crate::{Binding, CompileSource, CompiledGraph, GraphInstance, GraphOutput, StyleSel, UpdateInputs, compile, compile_styled};

const GRAPH: &str = r#"(
    params: [(name: "speed", kind: Float(1.2)), (name: "w", kind: Float(0.0))],
    nodes: {
        "move": BlendSpace1D(x: "speed", samples: [(clip: "loco.walk.cycle", at: 1.2), (clip: "loco.jog.cycle", at: 3.0)]),
        "idle_clip": Clip(clip: "idle"),
    },
    layers: [(node: "move"), (node: "idle_clip", weight: "w")],
)"#;

const TABLE: &str = r#"(
    base_prefix: "loco.",
    default_class: "neutral",
    classes: {
        "neutral": (prefix: None),
        "old":   (prefix: Some("loco.old."), fallback: Some("neutral")),
        "drunk": (prefix: Some("loco.drunk."), fallback: Identity),
        "limp":  (prefix: Some("loco.limp."), fallback: Identity),
        "mark":  (prefix: Some("loco.mark."), fallback: Identity),
    },
)"#;

/// Reference, duration, hips swing (rad, so every clip has its own pose),
/// point events and sync markers of every clip the source holds.
type ClipDef = (&'static str, f32, f32, &'static [(f32, &'static str)], &'static [(f32, &'static str)]);
const CLIPS: [ClipDef; 8] = [
    ("loco.walk.cycle", 1.0, 0.2, &[(0.25, "base_step")], &[(0.0, "down.l"), (0.5, "down.r")]),
    ("loco.jog.cycle", 0.6, 0.5, &[], &[]),
    ("idle", 2.0, 0.1, &[], &[]),
    ("loco.old.walk.cycle", 1.3, 0.8, &[(0.65, "old_step")], &[]),
    ("loco.old.jog.cycle", 0.8, 1.1, &[], &[]),
    ("loco.drunk.walk.cycle", 1.4, 1.4, &[], &[]),
    ("loco.limp.jog.cycle", 0.7, 1.7, &[], &[]),
    ("loco.mark.walk.cycle", 1.3, 2.0, &[], &[(0.0, "down.l"), (0.9, "down.r")]),
];

const DT: f32 = 1.0 / 60.0;

fn skeleton() -> Skeleton {
    let joints = [
        SourceJoint { name: "root".into(), parent: None, rest: Trs::IDENTITY },
        SourceJoint { name: "hips".into(), parent: Some(0), rest: Trs::from_translation(Vec3::Y) },
    ];
    let rules = SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() };
    Skeleton::with_rules(&joints, &rules).unwrap().0
}

fn clip(s: &Skeleton, &(name, d, swing, events, markers): &ClipDef) -> Clip {
    let rotation = Curve::linear(vec![0.0, d], vec![Quat::IDENTITY, Quat::from_rotation_x(swing)]);
    let src = SourceClip {
        name: name.into(),
        duration: d,
        tracks: vec![SourceTrack { joint: "hips".into(), rotation: Some(rotation), ..SourceTrack::default() }],
        events: events.iter().map(|&(t, e)| Event::point(t, e)).collect(),
        markers: markers.iter().map(|&(time, m)| SyncMarker { time, name: m.into() }).collect(),
        ..SourceClip::default()
    };
    let rules = ClipRules {
        looping: Looping::Whole,
        root_motion: RootMotionRules { mode: RootMotionMode::Ignore, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    };
    cook_clip(s, &src, &rules, &CookSettings::default()).unwrap()
}

struct Source(HashMap<String, Clip>);

impl CompileSource for Source {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        let c = self.0.get(reference).cloned().ok_or_else(|| format!("no clip {reference}"))?;
        Ok((ProductKey::new("anim.cook", reference), c))
    }
    fn available(&mut self, reference: &str) -> Result<bool, String> {
        Ok(self.0.contains_key(reference))
    }
    fn subgraph(&mut self, _path: &str) -> Result<GraphAsset, String> {
        Err("no subgraphs".into())
    }
}

struct Rig {
    skeleton: Skeleton,
    graph: CompiledGraph,
    clips: Vec<Arc<Clip>>,
}

/// The test graph, compiled with the style table or without one.
fn rig(styled: bool) -> Rig {
    let skeleton = skeleton();
    let mut source = Source(CLIPS.iter().map(|c| (c.0.to_owned(), clip(&skeleton, c))).collect());
    let asset = parse_graph(GRAPH).unwrap();
    let graph = if styled {
        compile_styled("test", &asset, &skeleton, &mut source, Some(&StyleTable::parse(TABLE).unwrap())).unwrap()
    } else {
        compile("test", &asset, &skeleton, &mut source).unwrap()
    };
    let clips = graph.clips.iter().map(|c| Arc::new(source.0[&c.reference].clone())).collect();
    Rig { skeleton, graph, clips }
}

impl Rig {
    fn binding(&self) -> Binding<'_> {
        Binding::new(&self.graph, &self.skeleton, &self.clips).unwrap()
    }
    fn slot(&self, reference: &str) -> usize {
        self.graph.clips.iter().position(|c| c.reference == reference).unwrap_or_else(|| panic!("no clip slot {reference}"))
    }
    fn class(&self, name: &str) -> u16 {
        let i = self.graph.styles.classes.iter().position(|c| c.name == ClassName::new(name)).unwrap_or_else(|| panic!("no class {name}"));
        u16::try_from(i).unwrap()
    }
    fn sel(&self, active: &str, identity: &str) -> StyleSel {
        StyleSel { active: self.class(active), identity: self.class(identity) }
    }
    /// An instance at `speed` (and idle layer weight `w`), stepped `n` updates.
    fn run(&self, speed: f32, w: f32, n: usize) -> (GraphInstance, GraphOutput) {
        let b = self.binding();
        let mut inst = GraphInstance::new(&b);
        inst.params.set_float(self.graph.param("speed").unwrap(), speed);
        inst.params.set_float(self.graph.param("w").unwrap(), w);
        let mut out = GraphOutput::new(&self.graph);
        for _ in 0..n {
            self.step(&mut inst, &mut out);
        }
        (inst, out)
    }
    fn step(&self, inst: &mut GraphInstance, out: &mut GraphOutput) {
        inst.update(&self.binding(), DT, 0, &UpdateInputs::default(), out);
    }
    /// `(time, weight)` of a lane of node `node`.
    fn lane(&self, inst: &GraphInstance, node: &str, lane: usize) -> (f32, f32) {
        inst.player(self.graph.node(node).unwrap()).unwrap().0[lane]
    }
    fn sample(&self, reference: &str, time: f32) -> LocalPose {
        let mut pose = LocalPose::new(self.skeleton.len());
        self.clips[self.slot(reference)].sample(time, &mut pose, None);
        pose
    }
}

fn same(a: &LocalPose, b: &LocalPose) -> bool {
    (0..a.len()).all(|j| {
        let (x, y) = (a.get(j), b.get(j));
        (x.rotation.dot(y.rotation).abs() - 1.0).abs() < 1e-6 && x.translation.distance(y.translation) < 1e-5
    })
}

/// The hips rotation step between two poses (rad).
fn hips_step(a: &LocalPose, b: &LocalPose) -> f32 {
    a.get(1).rotation.angle_between(b.get(1).rotation)
}

/// After a switch with `blend_s = 0` and one update, the pose is `reference`
/// sampled at the lane's playhead (and not the base clip's).
fn assert_plays(r: &Rig, inst: &GraphInstance, out: &GraphOutput, lane: usize, reference: &str, base: &str) {
    let (t, w) = r.lane(inst, "move", lane);
    assert_eq!(w, 1.0, "lane {lane} carries the whole blend space");
    assert!(same(&out.pose, &r.sample(reference, t)), "lane {lane} plays {reference} at {t}");
    if reference != base {
        assert!(!same(&r.sample(reference, t), &r.sample(base, t)), "the fixture tells {reference} from {base}");
    }
}

#[test]
fn only_remapped_clips_swap() {
    let r = rig(true);
    let b = r.binding();
    // Walk only under `old`: the old walk clip at the carried playhead.
    let (mut inst, mut out) = r.run(1.2, 0.0, 20);
    inst.set_style(&b, r.sel("old", "neutral"), 0.0);
    r.step(&mut inst, &mut out);
    assert_plays(&r, &inst, &out, 0, "loco.old.walk.cycle", "loco.walk.cycle");
    // The idle layer's clip never remaps.
    inst.params.set_float(r.graph.param("w").unwrap(), 1.0);
    r.step(&mut inst, &mut out);
    let (t, _) = r.lane(&inst, "idle_clip", 0);
    assert!(same(&out.pose, &r.sample("idle", t)), "idle plays its base clip");
    // Jog only under `drunk` (no jog clip), identity `neutral`: the base jog.
    let (mut inst, mut out) = r.run(3.0, 0.0, 20);
    inst.set_style(&b, r.sel("drunk", "neutral"), 0.0);
    r.step(&mut inst, &mut out);
    assert_plays(&r, &inst, &out, 1, "loco.jog.cycle", "loco.jog.cycle");
    // ... while its walk is the drunk walk.
    let (mut inst, mut out) = r.run(1.2, 0.0, 20);
    inst.set_style(&b, r.sel("drunk", "neutral"), 0.0);
    r.step(&mut inst, &mut out);
    assert_plays(&r, &inst, &out, 0, "loco.drunk.walk.cycle", "loco.walk.cycle");
}

#[test]
fn lookup_runs_active_then_identity_then_base() {
    let r = rig(true);
    let b = r.binding();
    let cases = [
        (r.sel("drunk", "old"), "loco.old.jog.cycle"),
        (r.sel("neutral", "neutral"), "loco.jog.cycle"),
        (StyleSel { active: 99, identity: 99 }, "loco.jog.cycle"),
        (StyleSel { active: 99, identity: r.class("old") }, "loco.old.jog.cycle"),
    ];
    for (sel, want) in cases {
        let (mut inst, mut out) = r.run(3.0, 0.0, 20);
        inst.set_style(&b, sel, 0.0);
        assert_eq!(inst.style(), Some(sel));
        r.step(&mut inst, &mut out);
        assert_plays(&r, &inst, &out, 1, want, "loco.jog.cycle");
    }
}

#[test]
fn a_switched_lane_keeps_its_phase() {
    let r = rig(true);
    let b = r.binding();
    // No markers on the class clip: the normalized loop phase (0.4 of 1.0 s -> 0.4 of 1.3 s).
    let (mut inst, _) = r.run(1.2, 0.0, 24);
    let (t, _) = r.lane(&inst, "move", 0);
    assert!((t - 0.4).abs() < 1e-3, "walk at {t}");
    inst.set_style(&b, r.sel("old", "neutral"), 0.0);
    let (t2, _) = r.lane(&inst, "move", 0);
    assert!((t2 - t * 1.3).abs() < 1e-4, "old walk at {t2}, want {}", t * 1.3);
    // Markers on both clips: the same fraction between the same markers
    // (0.4 between down.l 0.0 and down.r 0.5 -> between 0.0 and 0.9).
    let (mut inst, _) = r.run(1.2, 0.0, 24);
    let (t, _) = r.lane(&inst, "move", 0);
    inst.set_style(&b, r.sel("mark", "neutral"), 0.0);
    let (t2, _) = r.lane(&inst, "move", 0);
    assert!((t2 - t / 0.5 * 0.9).abs() < 1e-4, "mark walk at {t2}, want {}", t / 0.5 * 0.9);
}

#[test]
fn a_switch_on_a_playing_lane_inertializes() {
    let r = rig(true);
    let b = r.binding();
    // Playing walk switches: a blend, continuous in pose.
    let (mut inst, mut out) = r.run(1.2, 0.0, 20);
    let before = out.pose.clone();
    r.step(&mut inst, &mut out);
    let step0 = hips_step(&before, &out.pose);
    let last = out.pose.clone();
    inst.set_style(&b, r.sel("old", "neutral"), 0.3);
    r.step(&mut inst, &mut out);
    assert!(inst.inertializing());
    let blended = hips_step(&last, &out.pose);
    // The same switch as a cut, for scale.
    let (mut cut, mut cut_out) = r.run(1.2, 0.0, 21);
    let cut_last = cut_out.pose.clone();
    cut.set_style(&b, r.sel("old", "neutral"), 0.0);
    r.step(&mut cut, &mut cut_out);
    assert!(!cut.inertializing(), "blend_s 0 cuts");
    let jump = hips_step(&cut_last, &cut_out.pose);
    assert!(blended < 3.0 * step0 + 1e-4 && jump > 10.0 * blended, "per-update step {step0}, blended {blended}, cut {jump}");
    // The same selection again, or one resolving to the same clips: nothing.
    for _ in 0..40 {
        r.step(&mut inst, &mut out);
    }
    assert!(!inst.inertializing());
    let (t, _) = r.lane(&inst, "move", 0);
    inst.set_style(&b, r.sel("old", "neutral"), 0.3);
    inst.set_style(&b, r.sel("old", "drunk"), 0.3);
    assert_eq!(r.lane(&inst, "move", 0).0, t, "no time change");
    r.step(&mut inst, &mut out);
    assert!(!inst.inertializing());
    // Only a lane that isn't playing changes (walk plays, `limp` remaps jog): a cut.
    let (mut inst, mut out) = r.run(1.2, 0.0, 20);
    assert_eq!(r.lane(&inst, "move", 1).1, 0.0, "jog isn't playing");
    inst.set_style(&b, r.sel("limp", "neutral"), 0.3);
    r.step(&mut inst, &mut out);
    assert!(!inst.inertializing());
}

#[test]
fn a_switched_lane_fires_the_class_clip_events() {
    let r = rig(true);
    let b = r.binding();
    let (base_step, old_step) = (r.graph.event("base_step").unwrap(), r.graph.event("old_step").unwrap());
    let old_walk = u16::try_from(r.slot("loco.old.walk.cycle")).unwrap();
    // 0.45 s of the base walk: its event (0.25) fired once.
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&r.graph);
    let mut base_fired = 0;
    for _ in 0..27 {
        r.step(&mut inst, &mut out);
        base_fired += out.events.iter().filter(|e| e.id == base_step).count();
    }
    assert_eq!(base_fired, 1);
    // Three cycles of the old walk (1.3 s each) after the switch: three of its events, none of the base's.
    inst.set_style(&b, r.sel("old", "neutral"), 0.2);
    let (mut olds, mut bases) = (0, 0);
    for _ in 0..234 {
        r.step(&mut inst, &mut out);
        for e in &out.events {
            if e.id == old_step {
                assert_eq!(e.clip, old_walk, "reports the class slot");
                olds += 1;
            }
            bases += usize::from(e.id == base_step);
        }
    }
    assert_eq!((olds, bases), (3, 0));
}

#[test]
fn a_graph_without_classes_ignores_the_switch() {
    let r = rig(false);
    let b = r.binding();
    assert!(r.graph.styles.is_empty());
    let (mut a, mut out_a) = r.run(1.2, 0.0, 10);
    let (mut c, mut out_c) = r.run(1.2, 0.0, 10);
    let sel = StyleSel { active: 1, identity: 0 };
    assert_eq!(c.style(), None);
    c.set_style(&b, sel, 0.3);
    assert_eq!(c.style(), Some(sel));
    for k in 0..30 {
        r.step(&mut a, &mut out_a);
        r.step(&mut c, &mut out_c);
        assert_eq!(out_a.pose, out_c.pose, "update {k}");
        assert!(!c.inertializing());
    }
}
