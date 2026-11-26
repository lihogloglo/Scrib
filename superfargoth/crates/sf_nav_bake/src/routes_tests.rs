//! `nav.routes` (NV12c1): a fixture [`RouteSource`] over in-memory files, baked through the step.

use std::sync::Arc;

use glam::DVec3;
use sf_bake::{BakeError, Baker, Digest, PlanCx, ProductKey, RunCx, Store};
use sf_coords::WorldPos;
use sf_nav::{
    EdgeKind, LinkSide, ROUTE_GRAPH_FORMAT, RouteBuildRules, RouteGraph, RouteGraphBuilder,
    RouteNodeKind,
};
use sf_tunables::{SetBy, Tunables};
use sf_world::{LinkId, WorldspaceId};

use crate::routes::{ROUTE_SIMPLIFY_M, TUNABLES};
use crate::tests::{MemInputs, bake_ok, product, stats};
use crate::{
    ROUTES, RouteBake, RouteInput, RouteSource, decode_routes, default_route_build_rules,
    route_build_rules,
};

fn p(x: f64, z: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, z))
}

fn input_path(ws: &WorldspaceId) -> String {
    format!("{}/roads", ws.as_str())
}

/// Reads `<ws>/roads` (a postcard [`RouteInput`]) through `cx`; a space without the file has no
/// route graph. `join_m` stands for an adapter tunable that shapes the input (a plan parameter).
struct FileSource {
    join_m: u32,
}

impl RouteSource for FileSource {
    fn plan(&self, _ws: &WorldspaceId, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.param("join_m", self.join_m.to_le_bytes().to_vec());
        Ok(())
    }

    fn route_input(
        &self,
        cx: &mut RunCx<'_>,
        ws: &WorldspaceId,
    ) -> Result<Option<RouteInput>, BakeError> {
        let path = input_path(ws);
        if !cx.file_exists(&path)? {
            return Ok(None);
        }
        let input = postcard::from_bytes(&cx.read_file(&path)?)
            .map_err(|e| BakeError::Other(e.to_string()))?;
        Ok(Some(input))
    }
}

const LINK: LinkId = LinkId(0x00ab_cdef_0123_4567);

/// Two junctions joined by a polyline of three degree-2 nodes, spokes making them junctions, an
/// anchor at a spoke's end and one edge-less door end beside the polyline.
fn fixture() -> RouteInput {
    let mut r = RouteInput::default();
    let j1 = r.node(p(0.0, 0.0), RouteNodeKind::Junction);
    let j2 = r.node(p(100.0, 0.0), RouteNodeKind::Junction);
    let chain: Vec<u32> = [(25.0, 2.0), (50.0, -1.0), (75.0, 2.0)]
        .iter()
        .map(|&(x, z)| r.node(p(x, z), RouteNodeKind::Junction))
        .collect();
    let mut prev = j1;
    for &n in chain.iter().chain([&j2]) {
        r.edge(prev, n, EdgeKind::Path, true, Vec::new(), None);
        prev = n;
    }
    for (from, z) in [(j1, -30.0), (j1, 30.0), (j2, -30.0)] {
        let x = if from == j1 { 0.0 } else { 100.0 };
        let end = r.node(p(x, z), RouteNodeKind::End);
        r.edge(from, end, EdgeKind::Road, true, vec![p(x + 1.0, z / 2.0)], None);
    }
    let town = r.node(p(130.0, 0.0), RouteNodeKind::Anchor("town".into()));
    r.edge(j2, town, EdgeKind::Street, false, Vec::new(), Some(40.0));
    r.door(LINK, LinkSide::A, p(50.0, 10.0));
    r
}

/// The same graph through the builder's own calls (no `RouteInput`).
fn expected(ws: &WorldspaceId, rules: &RouteBuildRules) -> RouteGraph {
    let mut b = RouteGraphBuilder::new(ws.clone());
    let j1 = b.add_node(p(0.0, 0.0), RouteNodeKind::Junction);
    let j2 = b.add_node(p(100.0, 0.0), RouteNodeKind::Junction);
    let mut prev = j1;
    for (x, z) in [(25.0, 2.0), (50.0, -1.0), (75.0, 2.0)] {
        let n = b.add_node(p(x, z), RouteNodeKind::Junction);
        b.add_edge(prev, n, EdgeKind::Path, true, &[], None);
        prev = n;
    }
    b.add_edge(prev, j2, EdgeKind::Path, true, &[], None);
    for (from, x, z) in [(j1, 0.0, -30.0), (j1, 0.0, 30.0), (j2, 100.0, -30.0)] {
        let end = b.add_node(p(x, z), RouteNodeKind::End);
        b.add_edge(from, end, EdgeKind::Road, true, &[p(x + 1.0, z / 2.0)], None);
    }
    let town = b.add_node(p(130.0, 0.0), RouteNodeKind::Anchor("town".into()));
    b.add_edge(j2, town, EdgeKind::Street, false, &[], Some(40.0));
    b.add_door(LINK, LinkSide::A, p(50.0, 10.0));
    b.build(rules).0
}

/// `input` with its nodes, edges and doors in another order (edge ends remapped).
fn shuffled(input: &RouteInput) -> RouteInput {
    let n = input.nodes.len();
    // A fixed permutation: i -> (i * 5 + 3) mod n (5 is coprime with the fixture's 9 nodes).
    assert_ne!(n % 5, 0);
    let to = |i: u32| u32::try_from((i as usize * 5 + 3) % n).unwrap();
    let mut nodes = vec![None; n];
    for (i, node) in input.nodes.iter().enumerate() {
        nodes[to(u32::try_from(i).unwrap()) as usize] = Some(node.clone());
    }
    let mut edges: Vec<_> = input
        .edges
        .iter()
        .map(|e| {
            let mut e = e.clone();
            e.a = to(e.a);
            e.b = to(e.b);
            e
        })
        .collect();
    edges.reverse();
    let mut doors = input.doors.clone();
    doors.reverse();
    RouteInput {
        nodes: nodes.into_iter().map(Option::unwrap).collect(),
        edges,
        doors,
    }
}

struct Rig {
    inputs: Arc<MemInputs>,
    store_dir: tempfile::TempDir,
}

impl Rig {
    fn new() -> Self {
        let rig = Self {
            inputs: Arc::new(MemInputs::default()),
            store_dir: tempfile::tempdir().unwrap(),
        };
        rig.write(&main(), &fixture());
        rig.write(&other(), &other_input());
        rig
    }

    fn write(&self, ws: &WorldspaceId, input: &RouteInput) {
        self.inputs
            .put(input_path(ws), postcard::to_stdvec(input).unwrap());
    }

    /// A fresh bake session over the same store.
    fn baker(&self, join_m: u32, rules: RouteBuildRules) -> (Baker, RouteBake) {
        let mut baker = Baker::new(
            Store::open(self.store_dir.path()).unwrap(),
            self.inputs.clone(),
        );
        let bake = RouteBake::new(Arc::new(FileSource { join_m }), rules);
        bake.register(&mut baker);
        (baker, bake)
    }
}

fn main() -> WorldspaceId {
    WorldspaceId::new("main")
}

fn other() -> WorldspaceId {
    WorldspaceId::new("other/space")
}

fn other_input() -> RouteInput {
    let mut r = RouteInput::default();
    let a = r.node(p(0.0, 0.0), RouteNodeKind::End);
    let b = r.node(p(10.0, 0.0), RouteNodeKind::End);
    r.edge(a, b, EdgeKind::Road, true, Vec::new(), None);
    r
}

fn key(ws: &WorldspaceId) -> ProductKey {
    ProductKey::new(ROUTES, ws.as_str())
}

#[test]
fn baked_graph_equals_the_builder_and_ignores_input_order() {
    let rig = Rig::new();
    let rules = default_route_build_rules();
    let (baker, bake) = rig.baker(10, rules.clone());
    let keys = bake.products(&[main()]);
    assert_eq!(keys, vec![key(&main())]);
    bake_ok(&baker, &keys);
    let bytes = product(&baker, &key(&main()));
    let graph = decode_routes(&bytes).unwrap().unwrap();
    let want = expected(&main(), &rules);
    assert_eq!(graph, want);
    assert_eq!(bytes, want.encode().unwrap());
    // The door joined the polyline; the anchor survived.
    assert!(graph.anchor("town").is_some());
    assert!(graph.edges.iter().any(|e| e.kind == EdgeKind::Connector));

    // A second bake into a fresh store gives the same bytes; so does shuffled input.
    let again = Rig::new();
    let (b2, _) = again.baker(10, rules.clone());
    bake_ok(&b2, &keys);
    assert_eq!(product(&b2, &key(&main())), bytes);
    again.write(&main(), &shuffled(&fixture()));
    let (b3, _) = again.baker(10, rules);
    bake_ok(&b3, &keys);
    assert_eq!(stats(&b3, ROUTES).built, 1);
    assert_eq!(product(&b3, &key(&main())), bytes);
}

#[test]
fn source_reads_and_params_are_dependencies() {
    let rig = Rig::new();
    let rules = default_route_build_rules();
    let keys = [key(&main()), key(&other())];
    let first = bake_ok(&rig.baker(10, rules.clone()).0, &keys);

    // Unchanged: a full hit.
    let (b, _) = rig.baker(10, rules.clone());
    assert_eq!(bake_ok(&b, &keys), first);
    assert_eq!((stats(&b, ROUTES).built, stats(&b, ROUTES).hits), (0, 2));

    // Another space's read changes: only that space re-bakes.
    let mut o = other_input();
    o.nodes[1].pos = p(12.0, 0.0);
    rig.write(&other(), &o);
    let (b, _) = rig.baker(10, rules.clone());
    let after = bake_ok(&b, &keys);
    assert_eq!((stats(&b, ROUTES).built, stats(&b, ROUTES).hits), (1, 1));
    assert_eq!(after[&key(&main())], first[&key(&main())]);
    assert_ne!(after[&key(&other())], first[&key(&other())]);

    // A file nobody reads: nothing re-bakes.
    rig.inputs.put("main/unrelated".into(), vec![1, 2, 3]);
    let (b, _) = rig.baker(10, rules.clone());
    bake_ok(&b, &keys);
    assert_eq!(stats(&b, ROUTES).built, 0);

    // This space's read changes: it re-bakes, to the new graph.
    let mut m = fixture();
    m.nodes[0].pos = p(0.0, -5.0);
    rig.write(&main(), &m);
    let (b, _) = rig.baker(10, rules.clone());
    let after = bake_ok(&b, &keys);
    assert_eq!((stats(&b, ROUTES).built, stats(&b, ROUTES).hits), (1, 1));
    let bytes = product(&b, &key(&main()));
    assert_eq!(bytes, m.build(&main(), &rules).unwrap().0.encode().unwrap());
    assert_ne!(after[&key(&main())], first[&key(&main())]);

    // An adapter plan parameter and the bake rules re-bake every space.
    let (b, _) = rig.baker(11, rules.clone());
    bake_ok(&b, &keys);
    assert_eq!(stats(&b, ROUTES).built, 2);
    let coarser = RouteBuildRules { simplify_m: 1.0, ..rules };
    let (b, _) = rig.baker(11, coarser);
    bake_ok(&b, &keys);
    assert_eq!(stats(&b, ROUTES).built, 2);
}

#[test]
fn a_space_without_input_stores_nothing() {
    let rig = Rig::new();
    let (baker, bake) = rig.baker(10, default_route_build_rules());
    let void = WorldspaceId::new("void");
    let digests = bake_ok(&baker, &bake.products(std::slice::from_ref(&void)));
    assert_eq!(digests[&key(&void)], Digest::of(&[]));
    assert_eq!(decode_routes(&product(&baker, &key(&void))).unwrap(), None);
}

#[test]
fn product_format_prefix_and_wrong_version() {
    let rig = Rig::new();
    let (baker, _) = rig.baker(10, default_route_build_rules());
    bake_ok(&baker, &[key(&main())]);
    let bytes = product(&baker, &key(&main()));
    let prefix = postcard::to_stdvec(&ROUTE_GRAPH_FORMAT).unwrap();
    assert!(bytes.starts_with(&prefix));
    let mut wrong = postcard::to_stdvec(&(ROUTE_GRAPH_FORMAT + 1)).unwrap();
    wrong.extend_from_slice(&bytes[prefix.len()..]);
    assert_eq!(RouteGraph::decode(&wrong).unwrap(), None);
    assert!(decode_routes(&wrong).is_err());
    assert!(decode_routes(&bytes[..bytes.len() / 2]).is_err());
}

#[test]
fn bad_edge_index_fails_the_product() {
    let rig = Rig::new();
    let mut bad = fixture();
    bad.edges[0].b = 99;
    rig.write(&main(), &bad);
    let (baker, _) = rig.baker(10, default_route_build_rules());
    let report = baker.bake(&[key(&main())]);
    let failures: Vec<_> = report.failures().map(|(k, e)| (k.clone(), e.to_string())).collect();
    assert_eq!(failures.len(), 1);
    assert!(failures[0].1.contains("out of range"), "{}", failures[0].1);
}

#[test]
fn tunables_bind_the_build_rules() {
    assert_eq!(default_route_build_rules(), RouteBuildRules::default());
    let mut t = Tunables::default();
    assert_eq!(route_build_rules(&t), RouteBuildRules::default());
    t.register_all(TUNABLES).unwrap();
    let id = t.id_of(&ROUTE_SIMPLIFY_M).unwrap();
    t.write(id, 1.5, SetBy::User).unwrap();
    assert_eq!(
        route_build_rules(&t),
        RouteBuildRules { simplify_m: 1.5, ..RouteBuildRules::default() }
    );
}
