//! Static collision for `StaticProbe`: the Morrowind rules of the
//! framework's `collision.cook` step (`superfargoth/docs/specs/locomotion.md`,
//! "Ground probes"; spec: `docs/specs/actors-adapter.md`, "Static collision").
//!
//! Per model, in this order (`OpenMW`'s `BulletNifLoader` semantics, except
//! that the fallback is a simplified render LOD rather than every triangle):
//!
//! 1. Skinned models (actors, body parts): none. Actors collide as
//!    capsules, not as static geometry.
//! 2. An `NC…` string marker on the root (`NCO`, `NCC`): none for actors.
//! 3. A `RootCollisionNode`: its subtree's triangles, whatever their
//!    visibility, minus parts that keyframes move. An empty one means
//!    camera-only collision in the original: none for actors.
//! 4. Editor marker models (`MRK`) without one: none.
//! 5. Models `mesh.cook` skips (animated parts): their render geometry in
//!    the rest pose, minus moving, camera-facing and particle parts.
//! 6. Everything else: the `mesh.cook` product's LOD1 (the framework
//!    default, [`CollisionSettings::lod`]).
//!
//! Cases 1 to 5 are [`CollisionRules::authored`] results (an empty mesh for
//! "none"); case 6 returns `None` and the step derives it.

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
    /// NIF render geometry in the rest pose (models `mesh.cook` skips).
    RestPose,
    /// Render LOD of the `mesh.cook` product.
    RenderLod,
    None(NoCollision),
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Self::Authored => "authored (RootCollisionNode)",
            Self::RestPose => "rest-pose render geometry (animated)",
            Self::RenderLod => "render LOD1 (fallback)",
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

/// The model's collision source, and its mesh unless the step derives it
/// from the render LOD ([`Source::RenderLod`] gives `None`; "none" sources
/// give an empty mesh).
pub fn collision_of(nif: &mw_nif::Model, weld: f32) -> (Source, Option<CollisionMesh>) {
    let none = |why| (Source::None(why), Some(CollisionMesh::default()));
    if nif.had_skin {
        return none(NoCollision::Actor);
    }
    if nif.no_collision {
        return none(NoCollision::Marker);
    }
    let world = mw_nif::world_transforms(nif);
    let moving = moving_nodes(nif);
    let build = |keep: &dyn Fn(usize, &mw_nif::Mesh) -> bool| {
        let mut builder = CollisionBuilder::new(weld);
        for (i, node) in nif.nodes.iter().enumerate() {
            if let Some(mesh) = node.mesh.and_then(|m| nif.meshes.get(m))
                && !moving[i]
                && keep(i, mesh)
            {
                let transform = glam::Affine3A::from_cols_array(&world[i].to_cols_array());
                builder.add(&mesh.positions, &mesh.indices, transform, SurfaceId::DEFAULT);
            }
        }
        builder.finish()
    };

    if nif.nodes.iter().any(|n| n.kind == NodeKind::Collision) {
        let mesh = build(&|_, mesh| mesh.collision);
        return if mesh.is_empty() { none(NoCollision::EmptyCollisionNode) } else { (Source::Authored, Some(mesh)) };
    }
    if nif.marker {
        return none(NoCollision::EditorMarker);
    }
    if dynamic_parts(nif).is_empty() {
        return (Source::RenderLod, None);
    }
    let mut facing: Vec<bool> =
        nif.nodes.iter().map(|n| matches!(n.kind, NodeKind::Billboard | NodeKind::Particles)).collect();
    inherit(nif, &mut facing);
    let mesh = build(&|i, mesh| nif.nodes[i].visible && !mesh.collision && !facing[i]);
    (Source::RestPose, Some(mesh))
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
    /// Bump when [`collision_of`]'s output changes.
    pub const VERSION: u32 = 1;
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
        Ok(collision_of(&nif, self.weld).1)
    }

    fn version(&self) -> u32 {
        Self::VERSION
    }
}

/// Registers `collision.cook` with the Morrowind rules.
pub fn register(baker: &mut Baker) {
    let rules = MwCollisionRules::default();
    let settings = CollisionSettings { weld: rules.weld, ..CollisionSettings::default() };
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
        let mesh = mesh.unwrap();
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
        assert_eq!(collision_of(&nif, 1e-3), (Source::None(NoCollision::Marker), Some(CollisionMesh::default())));

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
        let (source, mesh) = collision_of(&nif, 1e-3);
        assert_eq!(source, Source::None(NoCollision::EditorMarker));
        assert!(mesh.unwrap().is_empty());
    }

    #[test]
    fn static_models_fall_back_to_the_render_lod() {
        assert_eq!(collision_of(&render_only(), 1e-3), (Source::RenderLod, None));
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
        let mesh = mesh.unwrap();
        assert_eq!(mesh.triangles.len(), 2, "only the static render quad");
        let extent = mesh.positions.iter().map(|p| Vec3::from_array(*p).length()).fold(0.0, f32::max);
        assert!((extent - 8.0_f32.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn only_nifs_are_ruled() {
        assert!(is_nif("meshes/x/ex_rock.NIF") && !is_nif("meshes/mod/rock.glb"));
    }
}
