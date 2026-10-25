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
//! Creature emitters switched by visibility controllers (`hide:<node>` clip
//! curves, `mw_anim::effects`) on nodes between the joint and the particle
//! node are gated here; a curve on a joint hides the emitter with its joint
//! (`actor_fx`, scale zero). Each creature system emits while its emission
//! curve (`emit:<particle node>`) says its window is open at the current
//! animation time ([`ActorEmitter`]): Dagoth Ur's hands in his casts, the
//! dremora's dust in his deaths, the flame atronach's body always.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use mw_anim::effects::Effect;
use mw_assets::{ActorEmitter, LiveEmitter, LiveNode, LiveSystem, NifAnimation, ParticleEmitter, ParticleRange};
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
    /// Per system whose emitter or array nodes hang from other joints: where
    /// they are, so they emit from where the pose puts them (a flame
    /// atronach's hands, its whole body), not from their rest place.
    pub live: Vec<LiveAnchor>,
}

/// A node as `(joint, offset in the joint's space)`, if it hangs from one.
pub type JointAt = Option<(String, Affine3A)>;

/// A system's moving emitter nodes, as `(joint, offset in the joint's space)`.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveAnchor {
    pub system: u32,
    /// The emitter node, when it hangs from another joint than the particle node.
    pub emitter: JointAt,
    /// Per `ParticleSystem::targets` entry: the node and its parent.
    pub targets: Vec<(JointAt, JointAt)>,
}

/// The node's nearest joint at or above it (`is_joint`), the node's rest
/// transform in that joint's space and the names below the joint (top
/// first). The root counts only if it is a joint.
fn anchor_of(
    model: &mw_nif::Model,
    node: usize,
    is_joint: &impl Fn(&str) -> bool,
    inclusive: bool,
) -> (Option<String>, Affine3A, Vec<String>) {
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
            let jointed = |i: usize| {
                let (j, offset, _) = anchor_of(model, i, &is_joint, true);
                j.map(|j| (j, offset))
            };
            let live = systems
                .iter()
                .map(|&s| {
                    let system = &model.particles[s as usize];
                    let emitter =
                        (system.emitter != node).then(|| jointed(system.emitter)).flatten().filter(|(j, _)| Some(j) != joint.as_ref());
                    let targets = system.targets.iter().map(|t| (jointed(t.node), t.parent.and_then(jointed))).collect();
                    LiveAnchor { system: s, emitter, targets }
                })
                .filter(|l| l.emitter.is_some() || !l.targets.is_empty())
                .collect();
            ParticleAnchor { joint, transform, path, visible: model.nodes[node].visible, systems, live }
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
    /// `(emitter, [(visibility curve index, show)])`: hidden while any says so
    /// (`mw_anim::effects::hidden`; `show` marks a `show:` curve).
    gates: Vec<(Entity, Vec<(u16, bool)>)>,
    /// `(emitter, [(system, emission curve index)])`: a system emits while
    /// its curve is at least 0.5.
    emission: Vec<(Entity, Vec<(u32, u16)>)>,
}

/// Per system of `anchor`, the index of its emission curve among `curves`
/// (the animation graph's), if the clips carry one; `names` from
/// [`mw_anim::effects::emission_names`] of the same model.
fn emission_curves(anchor: &ParticleAnchor, names: &[String], curves: &[String]) -> Vec<(u32, u16)> {
    anchor
        .systems
        .iter()
        .filter_map(|&s| {
            let name = names.get(s as usize)?;
            let curve = curves.iter().position(|c| c.eq_ignore_ascii_case(name))?;
            Some((s, u16::try_from(curve).ok()?))
        })
        .collect()
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
        ActorEmitter::default(),
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
            commands.entity(prop.entity).try_insert(PropParticles);
            let path = prop.item.model.as_str();
            let Some(nif) = models.nif(path) else {
                continue;
            };
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
        let Ok((animator, binding)) = bodies.get(body.body) else {
            continue;
        };
        let mut out = BodyParticles { body: Some(body.body), ..BodyParticles::default() };
        if let Some(nif) = models.nif(&body.skeleton) {
            let skeleton = &animator.asset.skeleton;
            let vis: Vec<(u16, &str, bool)> = animator
                .asset
                .graph
                .curves
                .iter()
                .enumerate()
                .filter_map(|(i, name)| match mw_anim::effects::parse(name) {
                    Some(Effect::Hide { node }) => Some((u16::try_from(i).ok()?, node, false)),
                    Some(Effect::Show { node }) => Some((u16::try_from(i).ok()?, node, true)),
                    _ => None,
                })
                .collect();
            let names = mw_anim::effects::emission_names(&nif);
            for anchor in particle_anchors(&nif, |name| binding.find(skeleton, name).is_some()) {
                let Some(joint) = anchor.joint.as_deref().and_then(|j| binding.find(skeleton, j)) else {
                    continue;
                };
                let gates: Vec<(u16, bool)> =
                    vis.iter().filter(|(_, node, _)| anchor.path.iter().any(|p| p.eq_ignore_ascii_case(node))).map(|(i, _, show)| (*i, *show)).collect();
                if !anchor.visible && gates.is_empty() {
                    continue; // hidden in the file, nothing shows it
                }
                debug!(
                    "  {}: joint {:?}, path {:?}, at {:?}, systems {:?}",
                    body.skeleton, anchor.joint, anchor.path, anchor.transform.translation, anchor.systems
                );
                visible_chain(&mut commands, &joints, joint, body.body);
                let emission = emission_curves(&anchor, &names, &animator.asset.graph.curves);
                // Gated from the start: not emitting until the first pose says so.
                let gate = ActorEmitter { emitting: emission.iter().map(|&(s, _)| (s, false)).collect() };
                let entity = commands.spawn((emitter(animation(&server, &body.skeleton), &anchor), ChildOf(joint))).insert(gate).id();
                let live = |at: &JointAt| {
                    let (name, offset) = at.as_ref()?;
                    binding.find(skeleton, name).map(|node| LiveNode { node, offset: *offset })
                };
                let systems: Vec<LiveSystem> = anchor
                    .live
                    .iter()
                    .map(|l| LiveSystem {
                        system: l.system,
                        emitter: live(&l.emitter),
                        targets: l.targets.iter().map(|(node, parent)| (live(node), live(parent))).collect(),
                    })
                    .collect();
                debug!(
                    "    live systems {:?}",
                    systems.iter().map(|s| (s.system, s.emitter.is_some(), s.targets.len())).collect::<Vec<_>>()
                );
                if !systems.is_empty() {
                    commands.entity(entity).try_insert(LiveEmitter { systems });
                }
                out.gates.push((entity, gates));
                out.emission.push((entity, emission));
            }
            debug!(
                "actor particles {root}: {} emitters, {} systems on emission curves",
                out.gates.len(),
                out.emission.iter().map(|e| e.1.len()).sum::<usize>()
            );
        }
        match done {
            Some(mut d) => *d = out,
            None => {
                commands.entity(root).try_insert(out);
            }
        }
    }
}

/// Gives the joints from `joint` up to `body` visibility components, so an
/// emitter under them inherits the character's visibility.
fn visible_chain(commands: &mut Commands, joints: &Query<(Option<&ChildOf>, Has<Visibility>)>, joint: Entity, body: Entity) {
    let mut at = joint;
    while at != body {
        let Ok((parent, has)) = joints.get(at) else {
            return;
        };
        if !has {
            commands.entity(at).try_insert(Visibility::Inherited);
        }
        let Some(parent) = parent else { return };
        at = parent.parent();
    }
}

/// Shows each gated creature emitter while its visibility curves say so, and
/// lets each system emit while its emission curve does.
fn gate_particles(roots: Query<&BodyParticles>, animators: Query<&Animator>, mut emitters: Query<(&mut Visibility, &mut ActorEmitter)>) {
    for particles in &roots {
        let Some(Ok(animator)) = particles.body.map(|b| animators.get(b)) else {
            continue;
        };
        if !animator.posed() {
            continue;
        }
        let curves = &animator.output.curves;
        for (entity, gates) in particles.gates.iter().filter(|(_, g)| !g.is_empty()) {
            let shown = gates.iter().all(|&(i, show)| {
                let effect = if show { Effect::Show { node: "" } } else { Effect::Hide { node: "" } };
                curves.get(usize::from(i)).is_none_or(|&v| mw_anim::effects::hidden(&effect, v) != Some(true))
            });
            if let Ok((mut visibility, _)) = emitters.get_mut(*entity) {
                visibility.set_if_neq(if shown { Visibility::Inherited } else { Visibility::Hidden });
            }
        }
        for (entity, systems) in particles.emission.iter().filter(|(_, s)| !s.is_empty()) {
            let Ok((_, mut gate)) = emitters.get_mut(*entity) else { continue };
            let emitting = systems.iter().map(|&(s, i)| (s, curves.get(usize::from(i)).is_some_and(|v| *v >= 0.5)));
            if !gate.emitting.iter().copied().eq(emitting.clone()) {
                gate.emitting = emitting.collect();
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
    fn array_emitter_nodes_anchor_at_their_joints() {
        use mw_nif::particles::{EmitFrom, EmitTarget, ParticleSystem};
        let target = |node, parent| EmitTarget {
            node,
            parent: Some(parent),
            transform: mw_nif::glam::Affine3A::IDENTITY,
            parent_origin: mw_nif::glam::Vec3::ZERO,
        };
        let system = ParticleSystem {
            node: 4,
            emitter: 1,
            emit_from: EmitFrom::Nodes { along_bones: true },
            targets: vec![target(1, 0), target(2, 1), target(3, 2)],
            ..Default::default()
        };
        let model = mw_nif::Model {
            nodes: vec![
                node("Bip01", None, 0.0, NodeKind::Node),
                node("Bip01 Pelvis", Some(0), 1.0, NodeKind::Node),
                node("Bip01 Spine", Some(1), 0.5, NodeKind::Node),
                node("Helper", Some(2), 0.25, NodeKind::Node),
                node("Fire", Some(0), 0.0, NodeKind::Particles),
            ],
            particles: vec![system],
            ..Default::default()
        };
        let joints = ["Bip01", "Bip01 Pelvis", "Bip01 Spine"];
        let anchors = particle_anchors(&model, |n| joints.contains(&n));
        let live = &anchors[0].live;
        assert_eq!(live.len(), 1);
        assert_eq!(live[0].emitter.as_ref().map(|(j, _)| j.as_str()), Some("Bip01 Pelvis"), "emitter on another joint");
        let names: Vec<_> =
            live[0].targets.iter().map(|(n, p)| (n.as_ref().map(|j| j.0.as_str()), p.as_ref().map(|j| j.0.as_str()))).collect();
        assert_eq!(
            names,
            [
                (Some("Bip01 Pelvis"), Some("Bip01")),
                (Some("Bip01 Spine"), Some("Bip01 Pelvis")),
                (Some("Bip01 Spine"), Some("Bip01 Spine"))
            ]
        );
        let helper = live[0].targets[2].0.as_ref().unwrap().1;
        assert!((helper.translation - Vec3A::new(0.0, 0.25, 0.0)).length() < 1e-6, "a non-joint node rides its joint at its offset");
    }

    #[test]
    fn systems_map_to_their_emission_curves() {
        let anchor = ParticleAnchor {
            joint: None,
            transform: Affine3A::IDENTITY,
            path: Vec::new(),
            visible: true,
            systems: vec![0, 2],
            live: Vec::new(),
        };
        let names = ["emit:PArray", "emit:Body", "emit:PArray#1"].map(String::from);
        let curves = ["Tri Head#1", "emit:parray#1", "vis:Body", "emit:PArray"].map(String::from);
        assert_eq!(emission_curves(&anchor, &names, &curves), [(0, 3), (2, 1)]);
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
            live: Vec::new(),
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
