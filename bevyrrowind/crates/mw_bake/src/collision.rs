//! Static collision for `StaticProbe`: the Morrowind rules of the
//! framework's `collision.cook` step (`superfargoth/docs/specs/locomotion.md`,
//! "Ground probes"; spec: `docs/specs/actors-adapter.md`, "Static collision").
//!
//! Per model, in this order (`OpenMW`'s `BulletNifLoader` semantics, ported
//! from `components/nifbullet/bulletnifloader.cpp` at `OpenMW` commit
//! 4e718b50, lines 110-326). Which nodes take part is [`select`]:
//!
//! 1. Skinned models (actors, body parts): none. Actors collide as
//!    capsules, not as static geometry (nor as the `"Bounding Box"` node's
//!    box, which `OpenMW` keeps for actors only).
//! 2. An `NC…` string marker on the root (`NCO`, `NCC`): none for actors.
//! 3. A root whose last direct-child `RootCollisionNode` is empty:
//!    camera-only collision in the original, none for actors.
//! 4. A root with a non-empty one: that subtree's triangles, whatever their
//!    visibility, minus parts that keyframes move. Other (extra or nested)
//!    collision nodes are plain geometry.
//! 5. Models `mesh.cook` skips (animated parts) and editor marker models
//!    (`MRK`): their NIF geometry in the rest pose, hidden nodes included,
//!    minus moving, camera-facing and particle parts. `MRK` drops only
//!    `Tri EditorMarker…` shapes; a model of nothing else gets none.
//! 6. Everything else (static, no `RootCollisionNode`): every NIF triangle,
//!    hidden nodes included, as `OpenMW` does. Not a render LOD: `mesh.cook`
//!    drops hidden nodes, and its simplified LOD1 moved borders and dropped
//!    triangles (holes; physics-fix-plan.md slice 7). Same build as case 5.
//!
//! Everywhere, a switch node contributes its first child only. All cases are
//! [`CollisionRules::authored`] results (an empty mesh for "none"); other
//! model formats (`.glb`) return `None` and the step derives collision from
//! the render LOD0 ([`register`]).

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use mw_nif::NodeKind;
use mw_nif::anim::Target;
use rayon::prelude::*;
use sf_bake::{BakeError, Baker, Inputs, Manifest, PlanCx, ProductKey, RunCx, StepStats, Store};
use sf_controller::steps::{COLLISION_COOK, CollisionCookStep, CollisionRules, CollisionSettings};
use sf_controller::{CollisionBuilder, CollisionMesh, SurfaceId};

use crate::ModelUse;
use crate::importer::dynamic_parts;

/// Why a model has no static collision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoCollision {
    /// Skinned geometry: an actor or body part.
    Actor,
    /// `NC…` string marker on the root.
    Marker,
    /// `RootCollisionNode` without (static) geometry: camera-only.
    EmptyCollisionNode,
    /// Editor marker model (`MRK`) without a collision node.
    EditorMarker,
}

/// Where a model's static collision comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Source {
    /// `RootCollisionNode` subtree.
    Authored,
    /// NIF geometry in the rest pose (models `mesh.cook` skips, and `MRK`
    /// models).
    RestPose,
    /// Every NIF triangle of a static model without a `RootCollisionNode`.
    AllGeometry,
    /// Render LOD of the `mesh.cook` product (non-NIF models).
    RenderLod,
    None(NoCollision),
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Self::Authored => "authored (RootCollisionNode)",
            Self::RestPose => "rest-pose NIF geometry (animated, MRK)",
            Self::AllGeometry => "all NIF geometry (fallback)",
            Self::RenderLod => "render LOD0 (non-NIF)",
            Self::None(NoCollision::Actor) => "none: skinned (actor)",
            Self::None(NoCollision::Marker) => "none: NC marker",
            Self::None(NoCollision::EmptyCollisionNode) => "none: empty RootCollisionNode",
            Self::None(NoCollision::EditorMarker) => "none: editor marker (MRK)",
        }
    }
}

/// Per node: whether keyframes move it (itself or an ancestor).
fn moving_nodes(nif: &mw_nif::Model) -> Vec<bool> {
    let mut moving = vec![false; nif.nodes.len()];
    for controller in &nif.controllers {
        if let Target::Transform { node, .. } = controller.target
            && let Some(m) = moving.get_mut(node)
        {
            *m = true;
        }
    }
    inherit(nif, &mut moving);
    moving
}

/// ORs each flag into the node's subtree (parents precede children).
fn inherit(nif: &mw_nif::Model, flags: &mut [bool]) {
    for (i, node) in nif.nodes.iter().enumerate() {
        if let Some(p) = node.parent
            && flags[p]
        {
            flags[i] = true;
        }
    }
}

/// Which nodes `OpenMW`'s collision walk takes, per [`select`].
#[derive(Debug)]
struct Selection {
    /// Per node: geometry here goes into the physical shape.
    collides: Vec<bool>,
    /// Some root picked a non-empty `RootCollisionNode`.
    authored: bool,
    /// Some root picked an empty one: the whole shape is camera-only.
    camera_only: bool,
}

/// `OpenMW`'s node selection (`bulletnifloader.cpp` (4e718b50): `handleRoot`
/// 137-180, `handleNode` 185-262; `NiNode::findRootCollisionNode`,
/// `components/nif/node.cpp:209`). Visibility plays no part: the loader has
/// no hidden-node check, so hidden nodes collide like any other.
fn select(nif: &mw_nif::Model) -> Selection {
    let n = nif.nodes.len();
    // Per parent (parents precede children, siblings keep their order): its
    // first child, and its LAST `RootCollisionNode` child, since
    // `findRootCollisionNode` searches the children in reverse ("yes, this
    // search needs to be reversed"). Only a root's direct children count
    // (the recursive search needs an `RCN` string marker, which vanilla
    // doesn't use and we don't read).
    let mut first_child = vec![None; n];
    let mut last_rcn = vec![None; n];
    let mut root_of = vec![0; n];
    for (i, node) in nif.nodes.iter().enumerate() {
        match node.parent {
            Some(p) => {
                first_child[p].get_or_insert(i);
                if node.kind == NodeKind::Collision {
                    last_rcn[p] = Some(i);
                }
                root_of[i] = root_of[p];
            }
            None => root_of[i] = i,
        }
    }
    let mut out = Selection { collides: vec![false; n], authored: false, camera_only: false };
    let mut skipped = vec![false; n];
    for (i, node) in nif.nodes.iter().enumerate() {
        let Some(p) = node.parent else {
            // handleRoot (163-179): no RootCollisionNode => every geometry
            // collides; an empty one (no children) => render geometry as
            // camera-only collision; else collision starts at that node.
            match last_rcn[i] {
                None => out.collides[i] = true,
                Some(rcn) if first_child[rcn].is_none() => out.camera_only = true,
                Some(_) => out.authored = true,
            }
            continue;
        };
        // handleNode (258-261): a NiSwitchNode (or NiFltAnimationNode) walks
        // its first child only, whichever child renders.
        skipped[i] = skipped[p] || (matches!(nif.nodes[p].kind, NodeKind::Switch { .. }) && first_child[p] != Some(i));
        // handleNode (214-232): the chosen RootCollisionNode turns collision
        // on for its subtree; extra or nested ones are plain geometry.
        let chosen = last_rcn[root_of[i]] == Some(i);
        out.collides[i] = out.collides[p] || chosen;
    }
    for (c, s) in out.collides.iter_mut().zip(skipped) {
        *c &= !s;
    }
    out
}

/// A Morrowind editor marker shape, which `MRK` models don't collide with
/// (`bulletnifloader.cpp` (4e718b50) line 274: case-insensitive prefix).
fn is_editor_marker(name: &str) -> bool {
    name.get(.."tri editormarker".len()).is_some_and(|p| p.eq_ignore_ascii_case("tri editormarker"))
}

/// The model's collision source and mesh ("none" sources give an empty
/// mesh).
pub fn collision_of(nif: &mw_nif::Model, weld: f32) -> (Source, CollisionMesh) {
    let none = |why| (Source::None(why), CollisionMesh::default());
    if nif.had_skin {
        return none(NoCollision::Actor);
    }
    // `NC…` makes the whole shape visual-only (`bulletnifloader.cpp`
    // (4e718b50) 150-160), RootCollisionNode or not.
    if nif.no_collision {
        return none(NoCollision::Marker);
    }
    let selection = select(nif);
    if selection.camera_only {
        return none(NoCollision::EmptyCollisionNode);
    }
    let world = mw_nif::world_transforms(nif);
    let moving = moving_nodes(nif);
    // `MRK` drops only `Tri EditorMarker…` shapes (line 274), everywhere.
    let keep = |i: usize| selection.collides[i] && !moving[i] && !(nif.marker && is_editor_marker(&nif.nodes[i].name));
    let build = |extra: &dyn Fn(usize) -> bool| {
        let mut builder = CollisionBuilder::new(weld);
        for (i, node) in nif.nodes.iter().enumerate() {
            if let Some(mesh) = node.mesh.and_then(|m| nif.meshes.get(m))
                && keep(i)
                && extra(i)
            {
                let transform = glam::Affine3A::from_cols_array(&world[i].to_cols_array());
                builder.add(&mesh.positions, &mesh.indices, transform, SurfaceId::DEFAULT);
            }
        }
        builder.finish()
    };

    if selection.authored {
        let mesh = build(&|_| true);
        return if mesh.is_empty() { none(NoCollision::EmptyCollisionNode) } else { (Source::Authored, mesh) };
    }
    // No `RootCollisionNode`: every triangle, hidden or not (a static model
    // has no moving or facing parts, so the filters below are no-ops there).
    let mut facing: Vec<bool> =
        nif.nodes.iter().map(|n| matches!(n.kind, NodeKind::Billboard | NodeKind::Particles)).collect();
    inherit(nif, &mut facing);
    let mesh = build(&|i| !facing[i]);
    if nif.marker && mesh.is_empty() {
        return none(NoCollision::EditorMarker);
    }
    let source = if nif.marker || !dynamic_parts(nif).is_empty() { Source::RestPose } else { Source::AllGeometry };
    (source, mesh)
}

fn is_nif(subject: &str) -> bool {
    subject.rsplit('.').next().is_some_and(|ext| ext.eq_ignore_ascii_case("nif"))
}

/// The Morrowind [`CollisionRules`] (module docs). Other model formats a
/// mod points at (`.glb`) take the framework default.
#[derive(Clone, Copy, Debug)]
pub struct MwCollisionRules {
    /// Weld grid for authored and rest-pose geometry, meters.
    pub weld: f32,
}

impl MwCollisionRules {
    /// Bump when [`collision_of`]'s output changes. 2: `OpenMW` selection
    /// rules (last `RootCollisionNode`, switch first child, hidden nodes, `MRK`).
    /// 3: the static fallback is every NIF triangle, not the render LOD1.
    pub const VERSION: u32 = 3;
}

impl Default for MwCollisionRules {
    fn default() -> Self {
        Self { weld: CollisionSettings::default().weld }
    }
}

impl CollisionRules for MwCollisionRules {
    fn plan(&self, subject: &str, cx: &mut PlanCx) {
        if is_nif(subject) {
            cx.file(subject);
        }
    }

    fn authored(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Option<CollisionMesh>, BakeError> {
        if !is_nif(subject) {
            return Ok(None);
        }
        let bytes = cx.read_file(subject)?;
        let nif = mw_nif::load(&bytes).map_err(|e| BakeError::step(&ProductKey::new(COLLISION_COOK, subject), e))?;
        Ok(Some(collision_of(&nif, self.weld).1))
    }

    fn version(&self) -> u32 {
        Self::VERSION
    }
}

/// Registers `collision.cook` with the Morrowind rules. Models the rules
/// don't cover (`.glb`) collide with their full render LOD0 too.
pub fn register(baker: &mut Baker) {
    let rules = MwCollisionRules::default();
    let settings = CollisionSettings { lod: 0, weld: rules.weld };
    CollisionCookStep::register(Arc::new(rules), settings, baker);
}

/// `collision.cook` of every model.
pub fn collision_keys(models: &[String]) -> Vec<ProductKey> {
    models.iter().map(|m| ProductKey::new(COLLISION_COOK, m.as_str())).collect()
}

/// Models, placements and triangles of one [`Source`].
#[derive(Clone, Debug, Default)]
pub struct SourceStats {
    pub models: u32,
    pub placements: u32,
    /// Models whose mesh came out empty.
    pub empty: u32,
    pub triangles: u64,
    /// The most placed model paths (with their placements), for spot checks.
    pub examples: Vec<String>,
}

/// What `collision.cook` produced over a model set.
#[derive(Clone, Debug, Default)]
pub struct CollisionReport {
    pub by_source: BTreeMap<Source, SourceStats>,
    /// Models without a product or whose NIF failed to load.
    pub failed: Vec<(String, String)>,
    /// The step's counters and the bake's wall time, when known.
    pub step: Option<(StepStats, Duration)>,
}

/// Classifies every model (parsing its NIF again) and reads its
/// `collision.cook` product from the manifest.
pub fn report(
    store: &Store,
    manifest: &Manifest,
    inputs: &dyn Inputs,
    models: &BTreeMap<String, ModelUse>,
) -> CollisionReport {
    let rows: Vec<_> = models
        .par_iter()
        .map(|(path, model_use)| -> Result<(Source, u32, u64), (String, String)> {
            let fail = |e: &dyn fmt::Display| (path.clone(), e.to_string());
            let source = if is_nif(path) {
                let bytes = inputs.read_file(path).map_err(|e| fail(&e))?;
                collision_of(&mw_nif::load(&bytes).map_err(|e| fail(&e))?, MwCollisionRules::default().weld).0
            } else {
                Source::RenderLod
            };
            let digest = manifest.get(COLLISION_COOK, path).ok_or_else(|| fail(&"no collision.cook product"))?;
            let bytes = store.get(&digest).map_err(|e| fail(&e))?;
            let mesh = CollisionMesh::from_bytes(&bytes).map_err(|e| fail(&e))?;
            Ok((source, model_use.placements, mesh.triangles.len() as u64))
        })
        .collect();
    let mut out = CollisionReport::default();
    let mut ranked: BTreeMap<Source, Vec<(u32, &String)>> = BTreeMap::new();
    for (row, path) in rows.into_iter().zip(models.keys()) {
        match row {
            Ok((source, placements, triangles)) => {
                let stats = out.by_source.entry(source).or_default();
                stats.models += 1;
                stats.placements += placements;
                stats.triangles += triangles;
                stats.empty += u32::from(triangles == 0);
                ranked.entry(source).or_default().push((placements, path));
            }
            Err(failure) => out.failed.push(failure),
        }
    }
    // The most placed models of each source.
    for (source, mut paths) in ranked {
        paths.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        out.by_source.entry(source).or_default().examples =
            paths.into_iter().take(3).map(|(n, p)| format!("{p} ({n})")).collect();
    }
    out
}

impl fmt::Display for CollisionReport {
    #[allow(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let models: u32 = self.by_source.values().map(|s| s.models).sum();
        let triangles: u64 = self.by_source.values().map(|s| s.triangles).sum();
        writeln!(f, "collision.cook: {models} models, {triangles} triangles, {} failed", self.failed.len())?;
        for (source, s) in &self.by_source {
            let mean = if s.models > s.empty { s.triangles as f64 / f64::from(s.models - s.empty) } else { 0.0 };
            writeln!(
                f,
                "  {:40} {:5} models ({:4} empty) {:7} placements {:9} tris (mean {mean:6.0})  e.g. {}",
                source.label(),
                s.models,
                s.empty,
                s.placements,
                s.triangles,
                s.examples.join(", ")
            )?;
        }
        if let Some((stats, wall)) = &self.step {
            writeln!(
                f,
                "  step: built {} hit {} failed {}, run {:.2} s summed over threads, wall {wall:.2?}, {:.1} MiB written",
                stats.built,
                stats.hits,
                stats.failed,
                stats.run_time.as_secs_f64(),
                stats.bytes_written as f64 / 1_048_576.0
            )?;
        }
        for (path, error) in self.failed.iter().take(5) {
            writeln!(f, "  failed: {path}: {error}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mw_nif::anim::{Controller, Timing, TransformTrack};
    use mw_nif::coords::Trs;
    use mw_nif::glam::Vec3;

    use super::*;

    fn node(name: &str, parent: Option<usize>, kind: NodeKind, mesh: Option<usize>) -> mw_nif::Node {
        mw_nif::Node { name: name.into(), parent, transform: Trs::IDENTITY, kind, visible: true, mesh }
    }

    fn quad(size: f32, collision: bool) -> mw_nif::Mesh {
        mw_nif::Mesh {
            positions: vec![[0.0, 0.0, 0.0], [size, 0.0, 0.0], [size, size, 0.0], [0.0, size, 0.0]],
            indices: vec![0, 1, 2, 0, 2, 3],
            collision,
            ..mw_nif::Mesh::default()
        }
    }

    /// Root with a render quad (2 m) and a collision node holding a 1 m
    /// quad, offset 5 m up.
    fn with_collision_node() -> mw_nif::Model {
        let mut nif = mw_nif::Model {
            nodes: vec![
                node("root", None, NodeKind::Node, None),
                node("render", Some(0), NodeKind::Geometry, Some(0)),
                node("collision", Some(0), NodeKind::Collision, None),
                node("collider", Some(2), NodeKind::Geometry, Some(1)),
            ],
            meshes: vec![quad(2.0, false), quad(1.0, true)],
            materials: vec![mw_nif::Material::default()],
            ..mw_nif::Model::default()
        };
        nif.nodes[2].transform.translation = [0.0, 5.0, 0.0];
        nif.nodes[3].visible = false;
        nif
    }

    fn render_only() -> mw_nif::Model {
        let mut nif = with_collision_node();
        nif.nodes.truncate(2);
        nif
    }

    fn moves(node: usize) -> Controller {
        Controller {
            timing: Timing::default(),
            auto_play: true,
            random_start: false,
            target: Target::Transform { node, track: TransformTrack::default() },
        }
    }

    #[test]
    fn collision_nodes_win_over_render_geometry() {
        let (source, mesh) = collision_of(&with_collision_node(), 1e-3);
        assert_eq!(source, Source::Authored);
        assert_eq!(mesh.triangles.len(), 2);
        let max_y = mesh.positions.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
        assert!((max_y - 6.0).abs() < 1e-5, "collision node transform applied: {max_y}");

        // Editor markers keep their authored collision.
        let mut nif = with_collision_node();
        nif.marker = true;
        assert_eq!(collision_of(&nif, 1e-3).0, Source::Authored);
    }

    #[test]
    fn no_collision_cases() {
        let mut nif = with_collision_node();
        nif.no_collision = true;
        assert_eq!(collision_of(&nif, 1e-3), (Source::None(NoCollision::Marker), CollisionMesh::default()));

        let mut nif = with_collision_node();
        nif.had_skin = true;
        assert_eq!(collision_of(&nif, 1e-3).0, Source::None(NoCollision::Actor));

        let mut nif = with_collision_node();
        nif.nodes.truncate(3);
        assert_eq!(collision_of(&nif, 1e-3).0, Source::None(NoCollision::EmptyCollisionNode));
        // Keyframed collision isn't static either.
        let mut nif = with_collision_node();
        nif.controllers.push(moves(2));
        assert_eq!(collision_of(&nif, 1e-3).0, Source::None(NoCollision::EmptyCollisionNode));

        let mut nif = render_only();
        nif.marker = true;
        nif.nodes[1].name = "Tri EditorMarker".into();
        let (source, mesh) = collision_of(&nif, 1e-3);
        assert_eq!(source, Source::None(NoCollision::EditorMarker));
        assert!(mesh.is_empty());
    }

    /// A `cells`×`cells` floor of 1 m quads, gently bumped (what a render
    /// simplifier would merge).
    fn floor(cells: u16) -> mw_nif::Mesh {
        let positions = (0..=cells)
            .flat_map(|z| (0..=cells).map(move |x| (f32::from(x), f32::from(z))))
            .map(|(x, z)| [x, 0.01 * (x * 1.3 + z * 0.7).sin(), z])
            .collect();
        let (cells, side) = (u32::from(cells), u32::from(cells) + 1);
        let mut indices = Vec::new();
        for z in 0..cells {
            for x in 0..cells {
                let i = z * side + x;
                indices.extend([i, i + side, i + 1, i + 1, i + side, i + side + 1]);
            }
        }
        mw_nif::Mesh { positions, indices, ..mw_nif::Mesh::default() }
    }

    #[test]
    fn static_models_without_a_collision_node_keep_every_triangle() {
        // Slice 7: no render LOD; every LOD0 floor triangle, hidden nodes too.
        let mut nif = model(
            vec![
                node("root", None, NodeKind::Node, None),
                node("floor", Some(0), NodeKind::Geometry, Some(0)),
                node("hidden", Some(0), NodeKind::Geometry, Some(1)),
            ],
            vec![floor(8), quad(1.0, false)],
        );
        nif.nodes[2].transform.translation = [0.0, 3.0, 0.0];
        nif.nodes[2].visible = false;
        let render: usize = nif.nodes.iter().filter_map(|n| n.mesh).map(|m| nif.meshes[m].indices.len() / 3).sum();
        assert_eq!(render, 8 * 8 * 2 + 2);
        let (source, mesh) = collision_of(&nif, 1e-3);
        assert_eq!(source, Source::AllGeometry);
        assert_eq!(mesh.triangles.len(), render, "every floor triangle plus the hidden quad");
        let top = mesh.positions.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
        assert!((top - 4.0).abs() < 1e-5, "the hidden (upright, 3 m up) quad collides: {top}");
    }

    #[test]
    fn animated_models_use_their_rest_pose_without_moving_or_facing_parts() {
        let mut nif = render_only();
        nif.nodes.push(node("banner", Some(0), NodeKind::Geometry, Some(1)));
        nif.nodes.push(node("flare", Some(0), NodeKind::Billboard, None));
        nif.nodes.push(node("glow", Some(3), NodeKind::Geometry, Some(2)));
        nif.nodes.push(node("hidden", Some(0), NodeKind::Geometry, Some(3)));
        nif.meshes.extend([quad(1.0, false), quad(3.0, false), quad(4.0, false)]);
        nif.meshes[1].collision = false;
        nif.nodes[5].visible = false;
        nif.controllers.push(moves(2));
        let (source, mesh) = collision_of(&nif, 1e-3);
        assert_eq!(source, Source::RestPose);
        // Rule 8: the hidden quad collides (no hidden check in `OpenMW`).
        assert_eq!(mesh.triangles.len(), 4, "the static render quad and the hidden one");
        assert!((extent(&mesh) - 18.0_f32.sqrt()).abs() < 1e-5, "the 3 m hidden quad");
    }

    fn extent(mesh: &CollisionMesh) -> f32 {
        mesh.positions.iter().map(|p| Vec3::from_array(*p).length()).fold(0.0, f32::max)
    }

    fn model(nodes: Vec<mw_nif::Node>, meshes: Vec<mw_nif::Mesh>) -> mw_nif::Model {
        mw_nif::Model { nodes, meshes, materials: vec![mw_nif::Material::default()], ..mw_nif::Model::default() }
    }

    /// A render quad plus a keyframed one: the NIF (rest-pose) path.
    fn animated(mut nodes: Vec<mw_nif::Node>, mut meshes: Vec<mw_nif::Mesh>) -> mw_nif::Model {
        let moving = nodes.len();
        nodes.push(node("banner", Some(0), NodeKind::Geometry, Some(meshes.len())));
        meshes.push(quad(9.0, false));
        let mut nif = model(nodes, meshes);
        nif.controllers.push(moves(moving));
        nif
    }

    // Rules from `OpenMW` `components/nifbullet/bulletnifloader.cpp` (4e718b50).

    #[test]
    fn rule1_the_last_root_collision_node_wins() {
        let nif = model(
            vec![
                node("root", None, NodeKind::Node, None),
                node("rcn a", Some(0), NodeKind::Collision, None),
                node("a", Some(1), NodeKind::Geometry, Some(0)),
                node("rcn b", Some(0), NodeKind::Collision, None),
                node("b", Some(3), NodeKind::Geometry, Some(1)),
            ],
            vec![quad(3.0, true), quad(1.0, true)],
        );
        let (source, mesh) = collision_of(&nif, 1e-3);
        assert_eq!(source, Source::Authored);
        assert_eq!(mesh.triangles.len(), 2, "only the last collision node");
        assert!((extent(&mesh) - 2.0_f32.sqrt()).abs() < 1e-5);

        // An empty last one wins too: camera-only.
        let mut nif = nif;
        nif.nodes.truncate(4);
        assert_eq!(collision_of(&nif, 1e-3).0, Source::None(NoCollision::EmptyCollisionNode));
    }

    #[test]
    fn rule2_without_a_root_collision_node_everything_collides() {
        assert_eq!(collision_of(&render_only(), 1e-3).0, Source::AllGeometry);
        // A collision node below the root's children is plain geometry.
        let nodes = vec![
            node("root", None, NodeKind::Node, None),
            node("render", Some(0), NodeKind::Geometry, Some(0)),
            node("group", Some(0), NodeKind::Node, None),
            node("nested rcn", Some(2), NodeKind::Collision, None),
            node("collider", Some(3), NodeKind::Geometry, Some(1)),
        ];
        let meshes = vec![quad(1.0, false), quad(2.0, true)];
        let (source, mesh) = collision_of(&model(nodes.clone(), meshes.clone()), 1e-3);
        assert_eq!(source, Source::AllGeometry);
        assert_eq!(mesh.triangles.len(), 4, "render and nested-collision quads");
        let (source, mesh) = collision_of(&animated(nodes, meshes), 1e-3);
        assert_eq!(source, Source::RestPose);
        assert_eq!(mesh.triangles.len(), 4, "render and nested-collision quads");
    }

    #[test]
    fn rule3_an_empty_root_collision_node_is_camera_only() {
        let mut nif = with_collision_node();
        nif.nodes.truncate(3);
        assert_eq!(
            collision_of(&nif, 1e-3),
            (Source::None(NoCollision::EmptyCollisionNode), CollisionMesh::default())
        );
    }

    #[test]
    fn rule4_mrk_drops_only_tri_editor_marker_shapes() {
        let nodes = vec![
            node("root", None, NodeKind::Node, None),
            node("render", Some(0), NodeKind::Geometry, Some(0)),
            node("Tri EditorMarker 01", Some(0), NodeKind::Geometry, Some(1)),
        ];
        let meshes = vec![quad(1.0, false), quad(5.0, false)];
        let mut nif = model(nodes, meshes);
        nif.marker = true;
        let (source, mesh) = collision_of(&nif, 1e-3);
        assert_eq!(source, Source::RestPose);
        assert_eq!(mesh.triangles.len(), 2);
        assert!((extent(&mesh) - 2.0_f32.sqrt()).abs() < 1e-5, "the marker shape is gone");
        // Nothing but marker shapes: no collision.
        nif.nodes.remove(1);
        nif.nodes[1].mesh = Some(1);
        assert_eq!(collision_of(&nif, 1e-3).0, Source::None(NoCollision::EditorMarker));

        // Under a collision node too, and only with `MRK`.
        let mut nif = with_collision_node();
        nif.nodes.push(node("tri editormarker", Some(2), NodeKind::Geometry, Some(0)));
        assert_eq!(collision_of(&nif, 1e-3).1.triangles.len(), 4);
        nif.marker = true;
        assert_eq!(collision_of(&nif, 1e-3).1.triangles.len(), 2);
        assert!(is_editor_marker("TRI EDITORMARKER") && !is_editor_marker("EditorMarker") && !is_editor_marker("tri"));
    }

    #[test]
    fn rule5_nc_markers_turn_collision_off() {
        for value in ["NC", "NCO", "NCC", "nc", "Ncc"] {
            assert!(mw_nif::no_collision_marker(value), "{value}");
        }
        for value in ["MRK", "N", "", "RCN"] {
            assert!(!mw_nif::no_collision_marker(value), "{value}");
        }
        // Even over a collision node.
        let mut nif = with_collision_node();
        nif.no_collision = true;
        assert_eq!(collision_of(&nif, 1e-3), (Source::None(NoCollision::Marker), CollisionMesh::default()));
    }

    #[test]
    fn rule6_switch_nodes_collide_with_their_first_child_only() {
        let switch = |under: usize, at: usize| {
            vec![
                node("switch", Some(under), NodeKind::Switch { active: 1 }, None),
                node("first", Some(at), NodeKind::Geometry, Some(0)),
                node("active", Some(at), NodeKind::Geometry, Some(1)),
            ]
        };
        let meshes = vec![quad(1.0, true), quad(3.0, true)];
        let mut nodes = vec![node("root", None, NodeKind::Node, None), node("rcn", Some(0), NodeKind::Collision, None)];
        nodes.extend(switch(1, 2));
        let (source, mesh) = collision_of(&model(nodes, meshes.clone()), 1e-3);
        assert_eq!(source, Source::Authored);
        assert!((extent(&mesh) - 2.0_f32.sqrt()).abs() < 1e-5, "first child, not the active one");

        let mut nodes = vec![node("root", None, NodeKind::Node, None)];
        nodes.extend(switch(0, 1));
        let (source, mesh) = collision_of(&animated(nodes, meshes), 1e-3);
        assert_eq!(source, Source::RestPose);
        assert_eq!(mesh.triangles.len(), 2);
        assert!((extent(&mesh) - 2.0_f32.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn rule7_a_bounding_box_node_is_not_static_collision_data() {
        // `OpenMW` reads it for the actor box only; its subtree collides as
        // any other, and skinned (actor) models get no static collision.
        let mut nif = with_collision_node();
        nif.nodes.push(node("Bounding Box", Some(2), NodeKind::Node, None));
        nif.nodes.push(node("inside", Some(4), NodeKind::Geometry, Some(0)));
        assert_eq!(collision_of(&nif, 1e-3).1.triangles.len(), 4);
        let mut nif = render_only();
        nif.nodes.push(node("Bounding Box", Some(0), NodeKind::Node, None));
        assert_eq!(collision_of(&nif, 1e-3).0, Source::AllGeometry);
        nif.had_skin = true;
        assert_eq!(collision_of(&nif, 1e-3).0, Source::None(NoCollision::Actor));
    }

    #[test]
    fn rule8_hidden_nodes_collide() {
        // Hidden authored collider (`with_collision_node` hides it).
        assert!(!with_collision_node().nodes[3].visible);
        assert_eq!(collision_of(&with_collision_node(), 1e-3).0, Source::Authored);
        // Hidden render geometry on the NIF path.
        let mut nif = animated(
            vec![node("root", None, NodeKind::Node, None), node("hidden", Some(0), NodeKind::Geometry, Some(0))],
            vec![quad(1.0, false)],
        );
        nif.nodes[1].visible = false;
        assert_eq!(collision_of(&nif, 1e-3).1.triangles.len(), 2);
    }

    #[test]
    fn only_nifs_are_ruled() {
        assert!(is_nif("meshes/x/ex_rock.NIF") && !is_nif("meshes/mod/rock.glb"));
    }
}
