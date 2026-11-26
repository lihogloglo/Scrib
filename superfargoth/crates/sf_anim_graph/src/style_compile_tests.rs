//! Tests of `compile_styled` (AN14b1): per-class clip remap and blend-sample
//! speeds in `CompiledGraph.styles` (animation-style.md "Class remap").
#![allow(clippy::float_cmp)]

use std::collections::HashMap;

use glam::{Quat, Vec3};
use sf_anim::{
    Clip, ClipRules, ContactRules, CookSettings, Curve, FormatError, Looping, RootMotionMode, RootMotionRules, Skeleton, SkeletonRules, SourceClip, SourceJoint,
    SourceTrack, Trs, cook_clip,
};
use sf_bake::ProductKey;

use crate::asset::{GraphAsset, parse_graph};
use crate::style::{ClassName, StyleTable};
use crate::{CompileSource, CompiledClass, CompiledGraph, GRAPH_FORMAT, compile, compile_styled};

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
        "old":   (prefix: Some("loco.old."), fallback: Some("neutral"), speeds: { "walk": 1.15, "jog": 2.6 }),
        "drunk": (prefix: Some("loco.drunk."), fallback: Identity, speeds: { "walk": 1.2, "jog": 3.2 }),
        "frail": (prefix: Some("loco.frail."), fallback: Some("old")),
    },
)"#;

/// Reference and duration of every clip the source holds.
const CLIPS: [(&str, f32); 7] = [
    ("loco.walk.cycle", 1.0),
    ("loco.jog.cycle", 0.6),
    ("idle", 2.0),
    ("loco.old.walk.cycle", 1.3),
    ("loco.old.jog.cycle", 0.8),
    ("loco.drunk.walk.cycle", 1.4),
    ("loco.frail.walk.cycle", 1.5),
];

fn skeleton() -> Skeleton {
    let joints = [
        SourceJoint { name: "root".into(), parent: None, rest: Trs::IDENTITY },
        SourceJoint { name: "hips".into(), parent: Some(0), rest: Trs::from_translation(Vec3::Y) },
    ];
    let rules = SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() };
    Skeleton::with_rules(&joints, &rules).unwrap().0
}

fn clip(s: &Skeleton, name: &str, d: f32) -> Clip {
    let rotation = Curve::linear(vec![0.0, d], vec![Quat::IDENTITY, Quat::from_rotation_x(0.2)]);
    let src = SourceClip {
        name: name.into(),
        duration: d,
        tracks: vec![SourceTrack { joint: "hips".into(), rotation: Some(rotation), ..SourceTrack::default() }],
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

/// In-memory clips; a reference is available when the source holds it.
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

struct Fixture {
    skeleton: Skeleton,
    source: Source,
    asset: GraphAsset,
    table: StyleTable,
}

fn fixture() -> Fixture {
    let skeleton = skeleton();
    let source = Source(CLIPS.iter().map(|&(name, d)| (name.to_owned(), clip(&skeleton, name, d))).collect());
    Fixture { skeleton, source, asset: parse_graph(GRAPH).unwrap(), table: StyleTable::parse(TABLE).unwrap() }
}

impl Fixture {
    fn plain(&mut self) -> CompiledGraph {
        compile("test", &self.asset, &self.skeleton, &mut self.source).unwrap()
    }
    fn styled(&mut self) -> CompiledGraph {
        compile_styled("test", &self.asset, &self.skeleton, &mut self.source, Some(&self.table)).unwrap()
    }
}

fn slot(g: &CompiledGraph, reference: &str) -> usize {
    g.clips.iter().position(|c| c.reference == reference).unwrap_or_else(|| panic!("no clip slot {reference}"))
}

fn class<'g>(g: &'g CompiledGraph, name: &str) -> &'g CompiledClass {
    g.styles.classes.iter().find(|c| c.name == ClassName::new(name)).unwrap_or_else(|| panic!("no class {name}"))
}

/// The class clip reference for each base slot (`None` = no remap).
fn remapped<'g>(g: &'g CompiledGraph, c: &CompiledClass) -> Vec<Option<&'g str>> {
    c.remap.iter().map(|&s| (s != u16::MAX).then(|| g.clips[usize::from(s)].reference.as_str())).collect()
}

#[test]
fn classes_remap_base_clips_to_appended_class_clips() {
    let mut f = fixture();
    let plain = f.plain();
    let g = f.styled();
    let base = plain.clips.len();
    assert_eq!(base, 3);
    // Base slots never move: the plain graph's clips are the styled graph's first ones.
    assert_eq!(g.clips[..base], plain.clips[..]);
    assert_eq!(g.clips.len(), base + 4, "drunk walk, frail walk, old walk, old jog: one slot each");
    let (walk, jog, idle) = (slot(&g, "loco.walk.cycle"), slot(&g, "loco.jog.cycle"), slot(&g, "idle"));
    assert!(walk < base && jog < base && idle < base);
    // Classes in table order (sorted by name).
    let names: Vec<&str> = g.styles.classes.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["drunk", "frail", "neutral", "old"]);
    for c in &g.styles.classes {
        assert_eq!(c.remap.len(), base, "class {}", c.name);
        assert_eq!(c.remap[idle], u16::MAX, "class {}: the non-loco clip never remaps", c.name);
    }
    let old = remapped(&g, class(&g, "old"));
    assert_eq!(old[walk], Some("loco.old.walk.cycle"));
    assert_eq!(old[jog], Some("loco.old.jog.cycle"));
    let drunk = remapped(&g, class(&g, "drunk"));
    assert_eq!(drunk[walk], Some("loco.drunk.walk.cycle"));
    assert!(class(&g, "neutral").remap.iter().all(|&s| s == u16::MAX), "no prefix: all none");
    // Appended slots keep their clip's facts.
    for (reference, d) in [("loco.old.walk.cycle", 1.3), ("loco.old.jog.cycle", 0.8), ("loco.drunk.walk.cycle", 1.4), ("loco.frail.walk.cycle", 1.5)] {
        let s = slot(&g, reference);
        assert!(s >= base, "{reference} is appended");
        assert!((g.clips[s].duration - d).abs() < 1e-5, "{reference}: duration {}", g.clips[s].duration);
    }
    assert_eq!(g.clip_events.len(), g.clips.len());
    assert_eq!(g.clip_curves.len(), g.clips.len());
    assert_eq!(g.warp_windows.len(), g.clips.len());
}

#[test]
fn walk_only_classes_fall_back_per_slot() {
    let mut f = fixture();
    let g = f.styled();
    let (walk, jog) = (slot(&g, "loco.walk.cycle"), slot(&g, "loco.jog.cycle"));
    // Identity fallback: the compiler stores none, the runtime tries the identity class next.
    let drunk = class(&g, "drunk");
    assert_eq!(drunk.remap[jog], u16::MAX);
    // Class fallback: frail has only a walk clip and takes old's jog slot.
    let (frail, old) = (class(&g, "frail"), class(&g, "old"));
    assert_eq!(g.clips[usize::from(frail.remap[walk])].reference, "loco.frail.walk.cycle");
    assert_ne!(old.remap[jog], u16::MAX);
    assert_eq!(frail.remap[jog], old.remap[jog]);
}

#[test]
fn class_speeds_sit_on_remapped_blend_samples() {
    let mut f = fixture();
    let g = f.styled();
    let node = g.node("move").unwrap();
    assert_eq!(&*class(&g, "old").at, &[(node, 0, 1.15), (node, 1, 2.6)]);
    assert_eq!(&*class(&g, "drunk").at, &[(node, 0, 1.2)], "only walk is remapped");
    assert!(class(&g, "frail").at.is_empty(), "no speeds: the samples keep the graph's positions");
    assert!(class(&g, "neutral").at.is_empty());
}

#[test]
fn without_a_table_the_graph_is_unchanged_and_old_blobs_are_refused() {
    let mut f = fixture();
    let plain = f.plain();
    let none = compile_styled("test", &f.asset, &f.skeleton, &mut f.source, None).unwrap();
    assert!(plain.styles.is_empty() && none.styles.is_empty());
    let bytes = plain.to_bytes();
    assert_eq!(bytes, none.to_bytes());
    let back = CompiledGraph::from_bytes(&bytes).unwrap();
    assert_eq!(back.to_bytes(), bytes);
    let mut old = bytes.clone();
    old[4..8].copy_from_slice(&16u32.to_le_bytes());
    match CompiledGraph::from_bytes(&old) {
        Err(FormatError::Version { found: 16, expected }) => assert_eq!(expected, GRAPH_FORMAT),
        other => panic!("expected a version error, got {:?}", other.map(|g| g.name)),
    }
}

#[test]
fn styles_carry_the_table_digest_and_round_trip() {
    let mut f = fixture();
    let g = f.styled();
    assert_ne!(f.table.digest, 0);
    assert_eq!(g.styles.table_digest, f.table.digest);
    let back = CompiledGraph::from_bytes(&g.to_bytes()).unwrap();
    assert_eq!(back.styles, g.styles);
}
