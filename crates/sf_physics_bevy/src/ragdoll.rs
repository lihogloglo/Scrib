//! Limp ragdolls on characters (feature `ragdoll`; physics.md "Ragdolls and
//! the animation blend", P7).
//!
//! The host puts [`Ragdoll`] on a character's `Animator` entity (its model
//! frame) when it dies, after starting the death clip. While `delay` runs
//! the clip plays and the last two animated model poses are kept; then the
//! skeleton's [`RagdollDef`] (from [`RagdollSource`]) is spawned at the
//! animated pose with its velocities, and every frame the bodies' pose goes
//! to the animator as a [`PhysicsPose`] whose weight ramps to 1 over
//! [`RagdollConfig::fade`]. The ragdoll's bodies are `RAGDOLL` (limp), so
//! holders can drag them by a limb (`CarryParams::pick_layers`), and a grab
//! decides carry or drag from the whole ragdoll's mass. Removing the
//! component or despawning the entity removes the bodies.
//!
//! Not yet (P8 and later): powered crumple, sleeping-pose persistence
//! across deactivation, placed corpses, the root following the pelvis.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_math::Affine3A;
use bevy_time::{Fixed, Time};
use bevy_transform::components::Transform;
use sf_anim::Trs;
use sf_anim_bevy::{Animator, PhysicsPose};
use sf_coords::WorldTransform;
use sf_ragdoll::{RagdollDef, RagdollInstance, RagdollSettings, SpawnPose};
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{WorldspaceRoot, from_grid};

use crate::{Physics, PhysicsSystems};

/// The host's ragdoll definitions, by the skeleton subject it names in
/// [`Ragdoll::skeleton`] (usually the `ragdoll.cook` product).
#[derive(Resource, Clone)]
pub struct RagdollSource(pub RagdollLoader);

/// Loads a skeleton's ragdoll by subject.
pub type RagdollLoader = Arc<dyn Fn(&str) -> Option<Arc<RagdollDef>> + Send + Sync>;

#[derive(Resource, Clone, Debug)]
pub struct RagdollConfig {
    pub settings: RagdollSettings,
    /// Animation → physics blend time after the handoff, s.
    pub fade: f32,
}

impl Default for RagdollConfig {
    fn default() -> Self {
        Self { settings: RagdollSettings::default(), fade: 0.1 }
    }
}

/// Go limp: on a character's `Animator` entity.
#[derive(Component, Clone, Debug)]
pub struct Ragdoll {
    pub skeleton: String,
    /// Seconds of death clip before the handoff.
    pub delay: f32,
}

/// Live ragdolls and requests, by their `Animator` entity.
#[derive(Resource, Default)]
pub struct Ragdolls {
    pending: HashMap<Entity, Pending>,
    live: HashMap<Entity, Live>,
}

impl Ragdolls {
    /// Live ragdolls: (entity, worldspace, instance).
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &WorldspaceId, &RagdollInstance)> {
        self.live.iter().map(|(e, l)| (*e, &l.worldspace, &l.instance))
    }

    pub fn len(&self) -> usize {
        self.live.len()
    }

    pub fn is_empty(&self) -> bool {
        self.live.is_empty()
    }
}

struct Pending {
    left: f32,
    /// The last two animated model poses, with the frame time between them.
    poses: [Option<Vec<Affine3A>>; 2],
    dt: f32,
}

struct Live {
    worldspace: WorldspaceId,
    def: Arc<RagdollDef>,
    instance: RagdollInstance,
    frozen: Vec<Trs>,
    /// The model frame at the handoff (the root stays there).
    root: WorldTransform,
    weight: f32,
}

#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct RagdollStats {
    pub live: usize,
    pub awake: usize,
    /// Largest joint separation this frame, m.
    pub max_stretch: f32,
    pub spawned: usize,
    pub failed: usize,
}

pub(crate) fn add(app: &mut App) {
    app.init_resource::<RagdollConfig>()
        .init_resource::<Ragdolls>()
        .init_resource::<RagdollStats>()
        .add_systems(Update, (handoff, read_back).chain().after(PhysicsSystems::WriteBack));
}

type FrameQuery<'w, 's> = Query<'w, 's, (Option<&'static CellCoord>, &'static Transform, &'static ChildOf)>;

/// The entity's frame in its worldspace: composed `Transform`s up to the
/// nearest high-precision (`CellCoord`) ancestor. Scale: its Y (uniform).
fn world_frame(entity: Entity, spatial: &FrameQuery<'_, '_>, roots: &Query<&WorldspaceRoot>) -> Option<(WorldspaceId, WorldTransform, f32)> {
    let mut local = Transform::IDENTITY;
    let mut current = entity;
    for _ in 0..64 {
        let (cell, transform, parent) = spatial.get(current).ok()?;
        let composed = transform.mul_transform(local);
        if let Some(cell) = cell {
            let root = roots.get(parent.parent()).ok()?;
            let frame = WorldTransform { pos: from_grid(cell, &composed), rotation: composed.rotation, scale: bevy_math::Vec3::ONE };
            return Some((root.id.clone(), frame, composed.scale.y));
        }
        local = composed;
        current = parent.parent();
    }
    None
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn handoff(
    time: Res<Time>,
    config: Res<RagdollConfig>,
    source: Option<Res<RagdollSource>>,
    mut physics: ResMut<Physics>,
    mut ragdolls: ResMut<Ragdolls>,
    mut stats: ResMut<RagdollStats>,
    requests: Query<(Entity, &Ragdoll, &Animator)>,
    spatial: FrameQuery,
    roots: Query<&WorldspaceRoot>,
) {
    let dt = time.delta_secs();
    let Ragdolls { pending, live } = &mut *ragdolls;
    // Removed requests and despawned characters drop their bodies.
    pending.retain(|e, _| requests.contains(*e));
    live.retain(|e, l| {
        let keep = requests.contains(*e);
        if !keep && let Some(w) = physics.worlds.get_mut(&l.worldspace) {
            l.instance.remove(w);
        }
        keep
    });
    let Some(source) = source else { return };
    for (entity, request, animator) in &requests {
        if live.contains_key(&entity) {
            continue;
        }
        let p = pending.entry(entity).or_insert_with(|| Pending { left: request.delay, poses: [None, None], dt: 0.0 });
        if animator.posed() {
            p.poses = [p.poses[1].take(), Some(animator.output.model.mats.clone())];
            p.dt = dt;
        }
        p.left -= dt;
        if p.left > 0.0 {
            continue;
        }
        let p = pending.remove(&entity).expect("present");
        let skeleton = &animator.asset.skeleton;
        let model = p.poses[1].clone().unwrap_or_else(|| skeleton.rest_model());
        let (Some(def), Some((ws, root, scale))) = ((source.0)(&request.skeleton), world_frame(entity, &spatial, &roots)) else {
            stats.failed += 1;
            continue;
        };
        if def.owner.len() != skeleton.len() || model.len() != skeleton.len() {
            stats.failed += 1;
            continue;
        }
        let Some(world) = physics.worlds.get_mut(&ws) else {
            stats.failed += 1;
            continue;
        };
        let previous = p.poses[0].as_deref().map(|prev| (prev, p.dt));
        let pose = SpawnPose { root, scale, model: &model, previous, root_velocity: bevy_math::Vec3::ZERO };
        let instance = RagdollInstance::spawn(world, &def, &pose, &config.settings);
        let frozen = (0..skeleton.len()).map(|j| animator.output.pose.get(j)).collect();
        live.insert(entity, Live { worldspace: ws, def, instance, frozen, root, weight: 0.0 });
        stats.spawned += 1;
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn read_back(
    time: Res<Time>,
    fixed: Res<Time<Fixed>>,
    config: Res<RagdollConfig>,
    physics: Res<Physics>,
    mut ragdolls: ResMut<Ragdolls>,
    mut stats: ResMut<RagdollStats>,
    mut characters: Query<(&Animator, Option<&mut PhysicsPose>)>,
    mut commands: Commands,
) {
    let alpha = fixed.overstep_fraction();
    let dt = time.delta_secs();
    let (mut awake, mut stretch) = (0, 0.0f32);
    for (entity, l) in &mut ragdolls.live {
        let Some(w) = physics.worlds.get(&l.worldspace) else { continue };
        let Ok((animator, pose)) = characters.get_mut(*entity) else { continue };
        let asleep = l.instance.asleep(w);
        awake += usize::from(!asleep);
        stretch = stretch.max(l.instance.max_stretch(w));
        let fading = l.weight < 1.0;
        l.weight = if config.fade > 0.0 { (l.weight + dt / config.fade).min(1.0) } else { 1.0 };
        if asleep && !fading && pose.is_some() {
            continue;
        }
        let mut out = Vec::new();
        if !l.instance.local_pose(w, &l.def, &animator.asset.skeleton, &l.root, alpha, &l.frozen, &mut out) {
            continue;
        }
        match pose {
            Some(mut p) => {
                p.pose = out;
                p.weight = l.weight;
            }
            None => {
                commands.entity(*entity).insert(PhysicsPose { pose: out, weight: l.weight, weights: Vec::new() });
            }
        }
    }
    stats.live = ragdolls.live.len();
    stats.awake = awake;
    stats.max_stretch = stretch;
}
