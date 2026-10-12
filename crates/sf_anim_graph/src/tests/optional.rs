//! Optional clips (animation.md "Optional clips"): fallbacks, drops,
//! diagnostics, provenance, and invalidation when a source appears.

use super::*;
use crate::steps::{ANIMGRAPH_COMPILE, AnimGraphCompileStep, GraphRules, graph_subject};

/// The test clip table, where only some bring-your-own sources exist, and
/// some of those may not be shared.
struct Byo {
    inner: Source,
    private: Vec<&'static str>,
}

impl CompileSource for Byo {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        self.inner.clip(reference)
    }
    fn subgraph(&mut self, path: &str) -> Result<asset::GraphAsset, String> {
        self.inner.subgraph(path)
    }
    fn available(&mut self, reference: &str) -> Result<bool, String> {
        Ok(self.inner.clips.contains_key(reference))
    }
    fn provenance(&mut self, reference: &str, _product: &ProductKey) -> Provenance {
        if self.private.contains(&reference) {
            Provenance { source: "mocap".into(), license: "CC-BY-NC-ND-4.0".into(), redistributable: false }
        } else {
            Provenance::default()
        }
    }
}

const BYO_GRAPH: &str = r#"(
    params: [(name: "speed", kind: Float(1.2)), (name: "go", kind: Bool(false)), (name: "fall", kind: Trigger)],
    clips: {
        "idle_kf": "idle",
        "idle_any": (source: "byo_idle", optional: true, fallback: Clip("idle_kf")),
        "walk_any": (source: "byo_walk", optional: true, fallback: Clip("walk")),
        "run_any": (source: "byo_run", optional: true),
        "getup": (source: "byo_getup", optional: true, fallback: Drop),
    },
    nodes: {
        "sm": StateMachine(
            states: [(name: "idle", node: "idle_clip"), (name: "move", node: "move"), (name: "getup", node: "getup_clip")],
            transitions: [
                (from: "idle", to: "move", when: [IsTrue("go")]),
                (from: "move", to: "idle", when: [IsFalse("go")]),
                (from: "*", to: "getup", when: [Trigger("fall")], blend: Inertialize(0.2)),
                (from: "getup", to: "idle", when: [Finished]),
            ],
        ),
        "idle_clip": Clip(clip: "idle_any"),
        "getup_clip": Clip(clip: "getup"),
        "move": BlendSpace1D(x: "speed", samples: [(clip: "walk_any", at: 1.2), (clip: "run_any", at: 3.6)], sync: (group: "feet")),
    },
    layers: [(node: "sm"), (node: "getup_clip", weight: 0.5)],
)"#;

fn byo(s: &Skeleton, present: bool) -> Byo {
    let mut inner = source(s);
    if present {
        let mut getup = attack(s);
        getup.name = "byo_getup".into();
        for c in [cycle(s, "byo_walk", 1.1, 1.3, 0.5, 0.5), cycle(s, "byo_run", 0.7, 3.5, 0.9, 0.2), getup] {
            inner.clips.insert(c.name.clone(), c);
        }
    }
    Byo { inner, private: vec!["byo_getup"] }
}

fn machine(g: &CompiledGraph) -> &Machine {
    g.nodes.iter().find_map(|n| if let NodeKind::StateMachine(m) = &n.kind { Some(m) } else { None }).unwrap()
}

fn lanes(g: &CompiledGraph, node: &str) -> usize {
    match &g.nodes[g.node(node).unwrap() as usize].kind {
        NodeKind::Player(p) => p.lanes.len(),
        k => panic!("{node} is {k:?}"),
    }
}

/// Every node reference and subtree range stays inside the node table.
fn check_layout(g: &CompiledGraph) {
    let n = u32::try_from(g.nodes.len()).unwrap();
    assert!(g.root < n);
    for (i, node) in g.nodes.iter().enumerate() {
        assert!(node.subtree_end as usize > i && node.subtree_end <= n, "{}: subtree end {}", node.name, node.subtree_end);
        let children: Vec<u32> = match &node.kind {
            NodeKind::Blend { a, b, .. } => vec![*a, *b],
            NodeKind::Additive { base, additive, .. } => vec![*base, *additive],
            NodeKind::Layers { base, layers } => std::iter::once(*base).chain(layers.iter().map(|l| l.node)).collect(),
            NodeKind::StateMachine(m) => m.states.iter().map(|s| s.node).collect(),
            NodeKind::RateScale { input, .. } | NodeKind::Mirror { input, .. } => vec![*input],
            _ => Vec::new(),
        };
        for c in children {
            assert!(c as usize > i && c < node.subtree_end, "{}: child {c} outside its subtree", node.name);
        }
    }
}

fn run(g: &CompiledGraph, s: &Skeleton, src: &Byo) {
    let clips: Vec<Arc<Clip>> = g.clips.iter().map(|c| Arc::new(src.inner.clips[&c.reference].clone())).collect();
    let b = Binding::new(g, s, &clips).unwrap();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(g);
    inst.params.set_bool(g.param("go").unwrap(), true);
    inst.params.fire(g.param("fall").unwrap());
    for _ in 0..90 {
        inst.update(&b, 1.0 / 60.0, 0, &UpdateInputs::default(), &mut out);
    }
}

#[test]
fn missing_optional_clips_fall_back_or_drop_cleanly() {
    let s = biped();
    let mut src = byo(&s, false);
    let g = compile_str("byo", BYO_GRAPH, &s, &mut src).unwrap();

    // Fallbacks: the graph's own clips, recorded as such.
    let walk = g.resolution("walk_any").unwrap();
    assert_eq!(walk.missing, ["byo_walk"]);
    assert_eq!(g.clips[usize::from(walk.slot.unwrap())].reference, "walk");
    let idle = g.resolution("idle_any").unwrap();
    assert!(idle.fell_back());
    assert_eq!(g.clips[usize::from(idle.slot.unwrap())].reference, "idle");
    // Drops: the run sample leaves the space, the get-up state and its
    // transitions leave the machine, and its layer leaves the root.
    assert_eq!(g.resolution("run_any").unwrap().slot, None);
    assert_eq!(g.resolution("getup").unwrap().slot, None);
    assert_eq!(lanes(&g, "move"), 1);
    let m = machine(&g);
    assert_eq!(m.states.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["idle", "move"]);
    assert_eq!(m.transitions.len(), 2);
    assert!(m.transitions.iter().all(|t| usize::from(t.to) < 2 && t.from.is_none_or(|f| usize::from(f) < 2)));
    let NodeKind::Layers { layers, .. } = &g.nodes[g.root as usize].kind else { panic!("root") };
    assert!(layers.is_empty());
    assert!(g.node("getup_clip").is_none());
    check_layout(&g);
    assert_eq!(g.non_redistributable().count(), 0);
    run(&g, &s, &src);

    // With every source present nothing falls back or drops.
    let mut full = byo(&s, true);
    let g = compile_str("byo", BYO_GRAPH, &s, &mut full).unwrap();
    assert_eq!(g.resolutions.iter().filter(|r| r.name != "idle_any" && r.fell_back()).count(), 0);
    assert_eq!(lanes(&g, "move"), 2);
    assert_eq!(machine(&g).states.len(), 3);
    assert_eq!(machine(&g).transitions.len(), 4);
    check_layout(&g);
    run(&g, &s, &full);

    // Provenance flags the slot that may not be shared, for export guards.
    let private: Vec<_> = g.non_redistributable().collect();
    assert_eq!(private.len(), 1);
    assert_eq!(private[0].reference, "byo_getup");
    assert_eq!(private[0].provenance.license, "CC-BY-NC-ND-4.0");
    let back = CompiledGraph::from_bytes(&g.to_bytes()).unwrap();
    assert_eq!(back.resolutions, g.resolutions);
    assert_eq!(back.non_redistributable().count(), 1);
}

#[test]
fn optional_clip_diagnostics() {
    let s = biped();
    let err = |ron: &str| compile_str("bad", ron, &s, &mut byo(&s, false)).unwrap_err().message;
    let one = |clips: &str, node: &str| format!("(clips: {{{clips}}}, nodes: {{\"n\": {node}}}, layers: [(node: \"n\")])");

    // A missing required clip is an error, also as the end of a fallback chain.
    assert!(err(&one(r#""w": "nope""#, r#"Clip(clip: "w")"#)).contains("no clip nope"));
    let e = err(&one(r#""w": (source: "byo_walk", optional: true, fallback: Clip("nope"))"#, r#"Clip(clip: "w")"#));
    assert!(e.contains("nope") && e.contains("required fallback of \"w\""), "{e}");
    // Fallback cycles, a fallback on a required clip, unknown fields.
    let e = err(&one(
        r#""a": (source: "x", optional: true, fallback: Clip("b")), "b": (source: "y", optional: true, fallback: Clip("a"))"#,
        r#"Clip(clip: "a")"#,
    ));
    assert!(e.contains("fallback cycle"), "{e}");
    assert!(err(&one(r#""w": (source: "walk", fallback: Drop)"#, r#"Clip(clip: "w")"#)).contains("needs `optional: true`"));
    assert!(err(&one(r#""w": (source: "walk", optonal: true)"#, r#"Clip(clip: "w")"#)).contains("optonal"));
    // Nothing left to play, or a dropped initial state.
    assert!(err(&one(r#""w": (source: "byo_walk", optional: true)"#, r#"Clip(clip: "w")"#)).contains("nothing to play"));
    let sm = r#"StateMachine(states: [(name: "a", node: "x"), (name: "b", node: "y")])"#;
    let e = err(&format!(
        r#"(clips: {{"g": (source: "byo_getup", optional: true)}}, nodes: {{"n": {sm}, "x": Clip(clip: "g"), "y": Clip(clip: "idle")}}, layers: [(node: "n")])"#
    ));
    assert!(e.contains("initial state dropped"), "{e}");
    // A transition naming a dropped state goes; one naming an unknown state is still an error.
    let e = err(
        r#"(clips: {"g": (source: "byo_getup", optional: true)}, nodes: {"n": StateMachine(states: [(name: "a", node: "y"), (name: "b", node: "x")],
            transitions: [(from: "a", to: "b"), (from: "a", to: "c")]), "x": Clip(clip: "g"), "y": Clip(clip: "idle")}, layers: [(node: "n")])"#,
    );
    assert!(e.contains("unknown state \"c\""), "{e}");
    // One side of a blend dropped: the other plays alone through a pass-through.
    let ok = one(r#""r": (source: "byo_run", optional: true)"#, r#"Blend(a: "w", b: "r", alpha: 0.5)"#)
        .replace(r#""n": Blend"#, r#""w": Clip(clip: "walk"), "r": Clip(clip: "r"), "n": Blend"#);
    let g = compile_str("ok", &ok, &s, &mut byo(&s, false)).unwrap();
    assert!(matches!(g.nodes[g.node("n").unwrap() as usize].kind, NodeKind::RateScale { .. }));
    check_layout(&g);
}

/// Marks `byo/` sources as private.
struct ByoRules;

impl GraphRules for ByoRules {
    fn provenance(&self, _graph: &str, reference: &str, _product: &ProductKey) -> Provenance {
        if reference.starts_with("byo/") {
            Provenance { source: "byo".into(), license: "CC-BY-NC-4.0".into(), redistributable: false }
        } else {
            Provenance::default()
        }
    }
}

#[test]
fn adding_an_optional_source_recompiles_the_graph() {
    let s = biped();
    let skel = "chars/hero.glb";
    let products: Arc<RwLock<HashMap<String, Vec<u8>>>> = Arc::default();
    let anims: Arc<RwLock<HashMap<String, Vec<u8>>>> = Arc::default();
    products.write().unwrap().insert(skel.into(), s.to_bytes());
    anims.write().unwrap().insert(sf_anim::steps::anim_subject(skel, "anims/loco.glb", "walk"), cycle(&s, "walk", 1.0, 1.2, 0.5, 0.5).to_bytes());
    let files = Arc::new(Files::default());
    let graph = r#"(clips: {"walk": "anims/loco.glb#walk", "walk_byo": (source: "byo/ual.glb#Walk_Loop", optional: true, fallback: Clip("walk"))},
        nodes: {"w": Clip(clip: "walk_byo")}, layers: [(node: "w")])"#;
    files.0.write().unwrap().insert("graphs/hero.animgraph.ron".into(), graph.as_bytes().to_vec());
    let dir = tempfile::tempdir().unwrap();
    let session = || {
        let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files.clone());
        baker.register(Stub(sf_anim::steps::SKELETON_COOK, products.clone())).register(Stub(sf_anim::steps::ANIM_COOK, anims.clone()));
        AnimGraphCompileStep::register(Arc::new(ByoRules), &mut baker);
        baker
    };
    let key = ProductKey::new(ANIMGRAPH_COMPILE, graph_subject("graphs/hero.animgraph.ron", skel));
    let load = |baker: &Baker| CompiledGraph::from_bytes(&baker.store().get(&baker.product(&key).unwrap()).unwrap()).unwrap();

    // The source is missing: the graph falls back to the cooked walk.
    let first = session();
    assert_eq!(first.bake(std::slice::from_ref(&key)).failures().count(), 0);
    let g = load(&first);
    assert_eq!(g.resolution("walk_byo").unwrap().missing, ["byo/ual.glb#Walk_Loop"]);
    assert_eq!(g.clips[0].reference, "anims/loco.glb#walk");
    assert_eq!(g.non_redistributable().count(), 0);

    // Nothing changed: a warm session builds nothing.
    let warm = session();
    assert_eq!(warm.bake(std::slice::from_ref(&key)).failures().count(), 0);
    assert_eq!(warm.step_stats()[ANIMGRAPH_COMPILE].built, 0);

    // The user adds the library: the graph recompiles onto it.
    files.0.write().unwrap().insert("byo/ual.glb".into(), b"glb".to_vec());
    anims
        .write()
        .unwrap()
        .insert(sf_anim::steps::anim_subject(skel, "byo/ual.glb", "Walk_Loop"), cycle(&s, "Walk_Loop", 1.1, 1.3, 0.5, 0.5).to_bytes());
    let added = session();
    assert_eq!(added.bake(std::slice::from_ref(&key)).failures().count(), 0);
    assert_eq!(added.step_stats()[ANIMGRAPH_COMPILE].built, 1);
    let g = load(&added);
    assert!(!g.resolution("walk_byo").unwrap().fell_back());
    assert_eq!(g.clips.len(), 1);
    assert!((g.clips[0].duration - 1.1).abs() < 1e-6);
    let private: Vec<_> = g.non_redistributable().collect();
    assert_eq!(private.len(), 1);
    assert_eq!(private[0].provenance.source, "byo");

    // Removing it again falls back again.
    files.0.write().unwrap().remove("byo/ual.glb");
    let removed = session();
    assert_eq!(removed.bake(std::slice::from_ref(&key)).failures().count(), 0);
    assert!(load(&removed).resolution("walk_byo").unwrap().fell_back());
}
