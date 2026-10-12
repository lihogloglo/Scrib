//! NIF particle systems on characters (nif-runtime.md "Particles on actors"):
//! the flame of a held torch, and the emitters inside creature models (flame
//! atronach, ghosts, ash creatures).
//!
//! The cooks flatten models to meshes and drop particle nodes, so the
//! particles come from the NIF itself: its `Animation` sub-asset (the same
//! data a placed fireplace uses) and, per particle node, its rest transform
//! relative to the nearest skeleton joint above it (a creature) or to the
//! model root (a held prop). An emitter entity at that transform under the
//! joint or prop follows the pose through transform propagation; the
//! simulation (`mw_assets::particles`) keeps world-space systems' particles
//! where they were emitted while the joint moves (`Simulation::carry`),
//! pauses off view, and hides beyond [`ACTOR_PARTICLE_RANGE_M`].
//!
//! Creature emitters switched by visibility controllers (`vis:<node>` clip
//! curves, `mw_anim::effects`) on nodes between the joint and the particle
//! node are gated here; a curve on a joint hides the emitter with its joint
//! (`actor_fx`, scale zero).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use mw_anim::effects::Effect;
use mw_assets::{ActorEmitter, LiveEmitter, NifAnimation, ParticleEmitter, ParticleRange};
use sf_anim_bevy::{AnimSystems, Animator, SkeletonBinding};
use sf_character_bevy::{CharacterBody, CharacterSystems, HeldProps};
use sf_vfs::{Vfs, VfsPath};

/// Beyond this camera distance actor particles hide and pause: a torch
/// flame is a few pixels there, and every simulated emitter re-uploads its
/// mesh each frame (streaming.md "Spikes").
pub const ACTOR_PARTICLE_RANGE_M: f32 = 60.0;

/// Where one particle node of a model goes.
#[derive(Clone, Debug, PartialEq)]
pub struct ParticleAnchor {
    /// The joint it hangs from (`None`: the model root).
    pub joint: Option<String>,
    /// The particle node's rest transform in the joint's (or root's) space.
    pub transform: Affine3A,
    /// Names of the nodes from below the joint down to the particle node:
    /// their visibility curves gate the emitter.
    pub path: Vec<String>,
    /// Whether the file shows it (else only a visibility curve does).
    pub visible: bool,
    /// Indices into the model's particle systems.
    pub systems: Vec<u32>,
    /// The emitter node (the first system's, when it isn't the particle
    /// node) anchored the same way, when that is a different joint: it then
    /// emits from where the pose puts it (a flame atronach's hands), not
    /// from its rest place.
    pub emitter: Option<(String, Affine3A)>,
}

/// The node's nearest joint at or above it (`is_joint`), the node's rest
/// transform in that joint's space and the names below the joint (top
/// first). The root counts only if it is a joint.
fn anchor_of(model: &mw_nif::Model, node: usize, is_joint: &impl Fn(&str) -> bool, inclusive: bool) -> (Option<String>, Affine3A, Vec<String>) {
    let local = |i: usize| {
        let t = &model.nodes[i].transform;
        Affine3A::from_scale_rotation_translation(Vec3::splat(t.scale), Quat::from_array(t.rotation), Vec3::from_array(t.translation))
    };
    if inclusive && is_joint(&model.nodes[node].name) {
        return (Some(model.nodes[node].name.clone()), Affine3A::IDENTITY, Vec::new());
    }
    let mut transform = local(node);
    let mut path = vec![model.nodes[node].name.clone()];
    let mut joint = None;
    let mut at = model.nodes[node].parent;
    while let Some(i) = at {
        let n = &model.nodes[i];
        if is_joint(&n.name) {
            joint = Some(n.name.clone());
            break;
        }
        if n.parent.is_none() {
            break; // a root that isn't a joint: the model root (its transform is dropped)
        }
        transform = local(i) * transform;
        path.push(n.name.clone());
        at = n.parent;
    }
    path.reverse();
    (joint, transform, path)
}

/// The particle nodes of `model`, each anchored at its nearest ancestor for
/// which `is_joint` holds (the root counts only if it is a joint).
pub fn particle_anchors(model: &mw_nif::Model, is_joint: impl Fn(&str) -> bool) -> Vec<ParticleAnchor> {
    let mut by_node: Vec<(usize, Vec<u32>)> = Vec::new();
    for (i, system) in model.particles.iter().enumerate() {
        let Ok(i) = u32::try_from(i) else { break };
        match by_node.iter_mut().find(|(n, _)| *n == system.node) {
            Some((_, systems)) => systems.push(i),
            None => by_node.push((system.node, vec![i])),
        }
    }
    by_node
        .into_iter()
        .map(|(node, systems)| {
            let (joint, transform, path) = anchor_of(model, node, &is_joint, false);
            let emitter_node = systems.first().map_or(node, |&s| model.particles[s as usize].emitter);
            let emitter = (emitter_node != node)
                .then(|| anchor_of(model, emitter_node, &is_joint, true))
                .and_then(|(j, offset, _)| j.filter(|j| Some(j) != joint.as_ref()).map(|j| (j, offset)));
            ParticleAnchor { joint, transform, path, visible: model.nodes[node].visible, systems, emitter }
        })
        .collect()
}

/// Parsed particle anchors per model (and per joint set: by skeleton model).
#[derive(Resource)]
struct ParticleModels {
    vfs: Arc<Vfs>,
    nifs: HashMap<String, Option<Arc<mw_nif::Model>>>,
}

impl ParticleModels {
    fn nif(&mut self, path: &str) -> Option<Arc<mw_nif::Model>> {
        let vfs = &self.vfs;
        self.nifs
            .entry(path.to_owned())
            .or_insert_with(|| {
                let model = VfsPath::new(path).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_nif::load(&b).ok())?;
                (!model.particles.is_empty()).then(|| Arc::new(model))
            })
            .clone()
    }
}

/// On a held prop whose particles were looked at.
#[derive(Component)]
struct PropParticles;

/// On a character root: the creature emitters spawned for its body.
#[derive(Component, Debug, Default)]
struct BodyParticles {
    body: Option<Entity>,
    /// `(emitter, visibility curve indices)`: hidden while any is below 0.5.
    gates: Vec<(Entity, Vec<u16>)>,
}

pub fn add(app: &mut App, vfs: Arc<Vfs>) {
    app.insert_resource(ParticleModels { vfs, nifs: HashMap::new() })
        .add_systems(Update, (prop_particles, body_particles).after(CharacterSystems))
        .add_systems(PostUpdate, gate_particles.after(AnimSystems::Apply).before(TransformSystems::Propagate));
}

/// The model's particle data: the `Animation` sub-asset of its NIF.
fn animation(server: &AssetServer, model: &str) -> Handle<NifAnimation> {
    server.load(format!("{model}#Animation"))
}

fn emitter(animation: Handle<NifAnimation>, anchor: &ParticleAnchor) -> impl Bundle {
    (
        Name::new(format!("particles {}", anchor.path.last().map_or("", String::as_str))),
        ParticleEmitter { animation, systems: anchor.systems.clone() },
        ActorEmitter,
        ParticleRange(ACTOR_PARTICLE_RANGE_M),
        Transform::from_matrix(Mat4::from(anchor.transform)),
        if anchor.visible { Visibility::Inherited } else { Visibility::Hidden },
    )
}

/// Particles on new held props (a torch's flame), anchored at the prop root.
fn prop_particles(
    mut commands: Commands,
    mut models: ResMut<ParticleModels>,
    server: Res<AssetServer>,
    roots: Query<&HeldProps, Changed<HeldProps>>,
    done: Query<(), With<PropParticles>>,
) {
    for props in &roots {
        for prop in props.props.iter().filter(|p| !done.contains(p.entity)) {
            commands.entity(prop.entity).insert(PropParticles);
            let path = prop.item.model.as_str();
            let Some(nif) = models.nif(path) else { continue };
            for anchor in particle_anchors(&nif, |_| false).iter().filter(|a| a.visible) {
                commands.spawn((emitter(animation(&server, path), anchor), ChildOf(prop.entity)));
            }
        }
    }
}

/// Particles inside creature models, under their joints, on a new body.
fn body_particles(
    mut commands: Commands,
    mut models: ResMut<ParticleModels>,
    server: Res<AssetServer>,
    mut roots: Query<(Entity, &CharacterBody, Option<&mut BodyParticles>)>,
    bodies: Query<(&Animator, &SkeletonBinding)>,
    joints: Query<(Option<&ChildOf>, Has<Visibility>)>,
) {
    for (root, body, done) in &mut roots {
        if done.as_ref().is_some_and(|d| d.body == Some(body.body)) {
            continue;
        }
        let Ok((animator, binding)) = bodies.get(body.body) else { continue };
        let mut out = BodyParticles { body: Some(body.body), gates: Vec::new() };
        if let Some(nif) = models.nif(&body.skeleton) {
            let skeleton = &animator.asset.skeleton;
            let vis: Vec<(u16, &str)> = animator
                .asset
                .graph
                .curves
                .iter()
                .enumerate()
                .filter_map(|(i, name)| match mw_anim::effects::parse(name) {
                    Some(Effect::Visibility { node }) => Some((u16::try_from(i).ok()?, node)),
                    _ => None,
                })
                .collect();
            for anchor in particle_anchors(&nif, |name| binding.find(skeleton, name).is_some()) {
                let Some(joint) = anchor.joint.as_deref().and_then(|j| binding.find(skeleton, j)) else { continue };
                let gates: Vec<u16> =
                    vis.iter().filter(|(_, node)| anchor.path.iter().any(|p| p.eq_ignore_ascii_case(node))).map(|(i, _)| *i).collect();
                if !anchor.visible && gates.is_empty() {
                    continue; // hidden in the file, nothing shows it
                }
                debug!("  {}: joint {:?}, path {:?}, at {:?}, systems {:?}", body.skeleton, anchor.joint, anchor.path, anchor.transform.translation, anchor.systems);
                debug!("    emitter {:?}", anchor.emitter.as_ref().map(|(j, o)| (j, o.translation)));
                visible_chain(&mut commands, &joints, joint, body.body);
                let entity = commands.spawn((emitter(animation(&server, &body.skeleton), &anchor), ChildOf(joint))).id();
                if let Some((name, offset)) = &anchor.emitter
                    && let Some(node) = binding.find(skeleton, name)
                {
                    commands.entity(entity).insert(LiveEmitter { node, offset: *offset });
                }
                out.gates.push((entity, gates));
            }
            debug!("actor particles {root}: {} emitters", out.gates.len());
        }
        match done {
            Some(mut d) => *d = out,
            None => {
                commands.entity(root).insert(out);
            }
        }
    }
}

/// Gives the joints from `joint` up to `body` visibility components, so an
/// emitter under them inherits the character's visibility.
fn visible_chain(commands: &mut Commands, joints: &Query<(Option<&ChildOf>, Has<Visibility>)>, joint: Entity, body: Entity) {
    let mut at = joint;
    while at != body {
        let Ok((parent, has)) = joints.get(at) else { return };
        if !has {
            commands.entity(at).insert(Visibility::Inherited);
        }
        let Some(parent) = parent else { return };
        at = parent.parent();
    }
}

/// Shows each gated creature emitter while its visibility curves say so.
fn gate_particles(roots: Query<&BodyParticles>, animators: Query<&Animator>, mut emitters: Query<&mut Visibility, With<ActorEmitter>>) {
    for particles in &roots {
        let Some(Ok(animator)) = particles.body.map(|b| animators.get(b)) else { continue };
        if !animator.posed() {
            continue;
        }
        let curves = &animator.output.curves;
        for (entity, gates) in particles.gates.iter().filter(|(_, g)| !g.is_empty()) {
            let shown = gates.iter().all(|&i| curves.get(usize::from(i)).is_none_or(|v| *v >= 0.5));
            if let Ok(mut visibility) = emitters.get_mut(*entity) {
                visibility.set_if_neq(if shown { Visibility::Inherited } else { Visibility::Hidden });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec3A;
    use mw_nif::coords::Trs;
    use mw_nif::{Node, NodeKind};

    fn node(name: &str, parent: Option<usize>, y: f32, kind: NodeKind) -> Node {
        let transform = Trs { translation: [0.0, y, 0.0], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 };
        Node { name: name.into(), parent, transform, kind, visible: true, mesh: None }
    }

    #[test]
    fn particle_nodes_anchor_at_their_nearest_joint_with_the_path_below_it() {
        let system = mw_nif::particles::ParticleSystem { node: 3, ..Default::default() };
        let model = mw_nif::Model {
            nodes: vec![
                node("Root", None, 5.0, NodeKind::Node),
                node("Bip01 Head", Some(0), 1.0, NodeKind::Node),
                node("Emitter", Some(1), 0.25, NodeKind::Node),
                node("Flame", Some(2), 0.5, NodeKind::Particles),
            ],
            particles: vec![system.clone(), system],
            ..Default::default()
        };

        let joints = ["Root", "Bip01 Head"];
        let anchors = particle_anchors(&model, |n| joints.contains(&n));
        assert_eq!(anchors.len(), 1, "one emitter per particle node");
        let a = &anchors[0];
        assert_eq!(a.joint.as_deref(), Some("Bip01 Head"));
        assert_eq!(a.path, ["Emitter", "Flame"]);
        assert_eq!(a.systems, [0, 1]);
        assert!((a.transform.translation - Vec3A::new(0.0, 0.75, 0.0)).length() < 1e-6, "rest offset below the joint");

        // A held prop: no joints, anchored at the model root (root transform dropped).
        let prop = particle_anchors(&model, |_| false);
        assert_eq!(prop[0].joint, None);
        assert!((prop[0].transform.translation - Vec3A::new(0.0, 1.75, 0.0)).length() < 1e-6);
    }

    #[test]
    fn emitter_follows_its_joint() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, TransformPlugin));
        let joint = app.world_mut().spawn((Transform::from_xyz(1.0, 2.0, 3.0), Visibility::Inherited)).id();
        let anchor = ParticleAnchor {
            joint: Some("Bip01 Head".into()),
            transform: Affine3A::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            path: vec!["Flame".into()],
            visible: true,
            systems: vec![0],
            emitter: None,
        };
        let emitter = app.world_mut().spawn((emitter(Handle::default(), &anchor), ChildOf(joint))).id();
        app.update();
        let at = |app: &App| app.world().get::<GlobalTransform>(emitter).unwrap().translation();
        assert!((at(&app) - Vec3::new(1.0, 2.5, 3.0)).length() < 1e-5);
        app.world_mut().get_mut::<Transform>(joint).unwrap().translation = Vec3::new(-4.0, 0.0, 0.0);
        app.update();
        assert!((at(&app) - Vec3::new(-4.0, 0.5, 0.0)).length() < 1e-5, "moves with the joint");
        assert!(app.world().get::<ActorEmitter>(emitter).is_some() && app.world().get::<ParticleRange>(emitter).is_some());
    }
}
