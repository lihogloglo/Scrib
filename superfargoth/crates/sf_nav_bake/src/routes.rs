//! The route bake (nav.md "Route graph" "Bake", NV12c1): step **`nav.routes`**, subject
//! worldspace, over the adapter's [`RouteSource`]. Bevy-free and game-agnostic.
//!
//! The source gives one [`RouteInput`] per worldspace (generic nodes, edges, kinds and door ends;
//! `None` = the space has no route graph). Its reads go through the step's [`RunCx`], so they are
//! the product's dynamic dependencies (as `nav.tiles` with `NavGeometrySource`): a mod that edits
//! one source re-bakes that space only. Adapter values that shape the input go in as plan
//! parameters through [`RouteSource::plan`]. The step feeds the input to [`RouteGraphBuilder`]
//! (order-free, so the source's iteration order never shows) with [`RouteBuildRules`] bound from
//! [`ROUTE_SIMPLIFY_M`] and [`ROUTE_DOOR_CONNECT_M`] (`Lifetime::Bake`), and stores
//! [`RouteGraph::encode`] (prefixed by [`ROUTE_GRAPH_FORMAT`]). A space without input stores zero
//! bytes (the store's one empty object); [`decode_routes`] reads it back as `None`.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_coords::WorldPos;
use sf_nav::{
    EdgeKind, LinkSide, ROUTE_GRAPH_FORMAT, RouteBuildRules, RouteBuildStats, RouteGraph,
    RouteGraphBuilder, RouteNodeKind,
};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Tunables, Unit, def};
use sf_world::{LinkId, WorldspaceId};

/// Step id.
pub const ROUTES: &str = "nav.routes";

/// `ai.nav.route.simplify_m`: [`RouteBuildRules::simplify_m`].
pub const ROUTE_SIMPLIFY_M: Def<f64> = def!("ai.nav.route.simplify_m" = 0.25_f64)
    .range(0.0, 5.0)
    .unit(Unit::Meters)
    .help("Douglas-Peucker tolerance of the route graph's polylines (0 keeps every point)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

/// `ai.nav.route.door_connect_m`: [`RouteBuildRules::door_connect_m`].
pub const ROUTE_DOOR_CONNECT_M: Def<f64> = def!("ai.nav.route.door_connect_m" = 60.0_f64)
    .range(0.0, 500.0)
    .unit(Unit::Meters)
    .help("Farthest a door approach with no edge of its own joins the nearest route edge; beyond, it stays isolated")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

/// The route bake's tunables, for the host to register.
pub const TUNABLES: &[&dyn AnyDef] = &[&ROUTE_SIMPLIFY_M, &ROUTE_DOOR_CONNECT_M];

/// The build rules from `t`: each registered tunable's effective value, else its default.
pub fn route_build_rules(t: &Tunables) -> RouteBuildRules {
    let read = |d: &Def<f64>| t.id_of(d).map_or(d.default_value(), |id| t.read(id));
    RouteBuildRules {
        simplify_m: read(&ROUTE_SIMPLIFY_M),
        door_connect_m: read(&ROUTE_DOOR_CONNECT_M),
    }
}

/// The defaults of [`ROUTE_SIMPLIFY_M`] and [`ROUTE_DOOR_CONNECT_M`].
pub fn default_route_build_rules() -> RouteBuildRules {
    RouteBuildRules {
        simplify_m: ROUTE_SIMPLIFY_M.default_value(),
        door_connect_m: ROUTE_DOOR_CONNECT_M.default_value(),
    }
}

/// One input node: a position and a kind. `Junction` and `End` are hints (the builder decides
/// them from the final degree); `Anchor(tag)` is a settlement or landmark; `Door` is a door end an
/// edge of the input already reaches (an edge-less one goes in [`RouteInput::doors`]).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteInputNode {
    pub pos: WorldPos,
    pub kind: RouteNodeKind,
}

/// One input edge between two nodes (indices into [`RouteInput::nodes`]) through `interior`
/// points. `cost_m: None` = the polyline's length (walk metres; a later bake pass measures it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteInputEdge {
    pub a: u32,
    pub b: u32,
    pub kind: EdgeKind,
    pub two_way: bool,
    pub interior: Vec<WorldPos>,
    pub cost_m: Option<f32>,
}

/// A door link end's approach point with no edge of its own: the builder joins it to the nearest
/// edge within `door_connect_m` (a `Connector`). Its node id hashes (`link`, `side`), not `pos`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteDoorEnd {
    pub link: LinkId,
    pub side: LinkSide,
    pub pos: WorldPos,
}

/// The adapter's route input for one worldspace. Order never matters (the builder is
/// order-free); equal node keys merge.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RouteInput {
    pub nodes: Vec<RouteInputNode>,
    pub edges: Vec<RouteInputEdge>,
    pub doors: Vec<RouteDoorEnd>,
}

impl RouteInput {
    /// Adds a node; returns its index for [`edge`](Self::edge).
    pub fn node(&mut self, pos: WorldPos, kind: RouteNodeKind) -> u32 {
        let i = u32::try_from(self.nodes.len()).expect("route input over 2^32 nodes");
        self.nodes.push(RouteInputNode { pos, kind });
        i
    }

    /// Adds an edge between node indices `a` and `b`.
    pub fn edge(
        &mut self,
        a: u32,
        b: u32,
        kind: EdgeKind,
        two_way: bool,
        interior: Vec<WorldPos>,
        cost_m: Option<f32>,
    ) {
        self.edges.push(RouteInputEdge {
            a,
            b,
            kind,
            two_way,
            interior,
            cost_m,
        });
    }

    /// Adds an edge-less door end.
    pub fn door(&mut self, link: LinkId, side: LinkSide, pos: WorldPos) {
        self.doors.push(RouteDoorEnd { link, side, pos });
    }

    /// The builder over this input (`Err` names an edge whose end index is out of range).
    pub fn builder(&self, ws: &WorldspaceId) -> Result<RouteGraphBuilder, String> {
        let mut b = RouteGraphBuilder::new(ws.clone());
        let ids: Vec<_> = self
            .nodes
            .iter()
            .map(|n| b.add_node(n.pos, n.kind.clone()))
            .collect();
        for (i, e) in self.edges.iter().enumerate() {
            let end = |n: u32| {
                ids.get(n as usize).copied().ok_or_else(|| {
                    format!("edge {i}: node {n} out of range ({} nodes)", ids.len())
                })
            };
            b.add_edge(end(e.a)?, end(e.b)?, e.kind, e.two_way, &e.interior, e.cost_m);
        }
        for d in &self.doors {
            b.add_door(d.link, d.side, d.pos);
        }
        Ok(b)
    }

    /// Builds the graph as the step does (measurements, adapter tests).
    pub fn build(
        &self,
        ws: &WorldspaceId,
        rules: &RouteBuildRules,
    ) -> Result<(RouteGraph, RouteBuildStats), String> {
        Ok(self.builder(ws)?.build(rules))
    }
}

/// The adapter's route source (nav.md "Route graph" "Bake"). Every read that shapes the input
/// goes through `cx` (recorded as a dynamic dependency); the input must be a function of those
/// reads and the plan parameters alone.
pub trait RouteSource: Send + Sync {
    /// Plan parameters of the space's input (adapter tunables that shape it, e.g. a join
    /// distance): a change re-bakes the space. Default: none.
    fn plan(&self, _ws: &WorldspaceId, _cx: &mut PlanCx) -> Result<(), BakeError> {
        Ok(())
    }

    /// The space's input; `None` = no route graph for it (zero-byte product).
    fn route_input(
        &self,
        cx: &mut RunCx<'_>,
        ws: &WorldspaceId,
    ) -> Result<Option<RouteInput>, BakeError>;
}

struct Shared {
    source: Arc<dyn RouteSource>,
    rules: RouteBuildRules,
}

/// The route bake: registers `nav.routes` on a [`Baker`] and names its products.
#[derive(Clone)]
pub struct RouteBake(Arc<Shared>);

impl RouteBake {
    /// `rules` usually from [`route_build_rules`].
    pub fn new(source: Arc<dyn RouteSource>, rules: RouteBuildRules) -> Self {
        Self(Arc::new(Shared { source, rules }))
    }

    pub fn rules(&self) -> &RouteBuildRules {
        &self.0.rules
    }

    pub fn register(&self, baker: &mut Baker) {
        baker.register(RoutesStep(self.0.clone()));
    }

    /// One `nav.routes` product per worldspace.
    pub fn products(&self, worldspaces: &[WorldspaceId]) -> Vec<ProductKey> {
        worldspaces
            .iter()
            .map(|ws| ProductKey::new(ROUTES, ws.as_str()))
            .collect()
    }
}

/// A `nav.routes` product: `None` for a space without input (zero bytes); a wrong
/// [`ROUTE_GRAPH_FORMAT`] or bad bytes is an error (re-bake).
pub fn decode_routes(bytes: &[u8]) -> Result<Option<RouteGraph>, BakeError> {
    if bytes.is_empty() {
        return Ok(None);
    }
    match RouteGraph::decode(bytes) {
        Ok(Some(g)) => Ok(Some(g)),
        Ok(None) => Err(BakeError::Other(format!(
            "{ROUTES}: route graph of another format (want {ROUTE_GRAPH_FORMAT}); re-bake"
        ))),
        Err(e) => Err(BakeError::Other(format!("{ROUTES}: {e}"))),
    }
}

struct RoutesStep(Arc<Shared>);

impl Step for RoutesStep {
    fn id(&self) -> &'static str {
        ROUTES
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let r = &self.0.rules;
        let mut rules = Vec::with_capacity(16);
        rules.extend_from_slice(&r.simplify_m.to_le_bytes());
        rules.extend_from_slice(&r.door_connect_m.to_le_bytes());
        cx.param("rules", rules)
            .param("format", ROUTE_GRAPH_FORMAT.to_le_bytes().to_vec());
        self.0.source.plan(&WorldspaceId::new(subject), cx)
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let ws = WorldspaceId::new(subject);
        let fail = |m: String| BakeError::step(&ProductKey::new(ROUTES, subject), m);
        let Some(input) = self.0.source.route_input(cx, &ws)? else {
            return Ok(Vec::new());
        };
        let (graph, _) = input.build(&ws, &self.0.rules).map_err(fail)?;
        graph.encode().map_err(|e| fail(e.to_string()))
    }
}
