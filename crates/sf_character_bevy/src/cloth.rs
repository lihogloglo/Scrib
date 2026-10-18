//! Cloth parts on characters (physics.md "Cloth", M4 P10): a robe or skirt
//! the adapter's [`CharacterContent::cloth_spec`] names is cooked
//! (`sf_character::cook_cloth`), rebound onto its lattice
//! (`sf_character::cloth_decoded`) and assembled like any part; its lattice
//! joints are appended to the outfit palette as entities under the body
//! (bind: the particle's rest position, unrotated). The animator's graph
//! runs the `ClothLattice` op (`GraphInstance::set_cloth`) after the
//! procedural ops, and [`write_cloth_joints`] poses those entities from the
//! op's positions and rotations.
//!
//! **Scaffold `ClothCookAtLoad`:** the cook runs on the build task (once
//! per part and skeleton, cached; ~9 ms median) instead of the `cloth.cook`
//! bake step. Removal path: a `cloth.cook` product per (part, skeleton) in
//! the bake (`ClothCook` is serde) read through the content, as
//! `ragdoll.cook` is.

use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec3};
use bevy_transform::components::Transform;
use sf_anim_bevy::{AnimAsset, Animator, ClothBinding};
use sf_character::{BodyPart, CharacterRecipe, ClothSpec, DecodedPart, SkinSurface, cloth_decoded, cook_cloth};
use sf_model::CookedSkinned;
use sf_records::RecordId;

use crate::CharacterContent;

/// Lattice joint names in the outfit palette: `cloth<k>#<particle>`, `k`
/// the cloth's index in the outfit.
pub(crate) const CLOTH_JOINT: &str = "cloth";

pub(crate) fn joint_name(cloth: usize, particle: usize) -> String {
    format!("{CLOTH_JOINT}{cloth}#{particle}")
}

/// `(cloth, particle)` of a lattice joint name.
pub(crate) fn parse_joint(name: &str) -> Option<(usize, usize)> {
    let (k, i) = name.strip_prefix(CLOTH_JOINT)?.split_once('#')?;
    Some((k.parse().ok()?, i.parse().ok()?))
}

/// One cooked cloth part for one skeleton (cached by the build tasks).
pub(crate) struct ClothPart {
    pub binding: Arc<ClothBinding>,
    /// Lattice joints named `cloth0#i` (renamed per outfit).
    pub decoded: DecodedPart,
}

/// Cooks a cloth part on the recipe's skeleton (`ClothCookAtLoad`).
pub(crate) fn cook_part(
    content: &dyn CharacterContent,
    recipe: &CharacterRecipe,
    animation: &AnimAsset,
    id: &RecordId,
    cooked: &CookedSkinned,
    part: &BodyPart,
    spec: &ClothSpec,
) -> Result<ClothPart, String> {
    let skeleton = &animation.skeleton;
    let rest = skeleton.rest_model();
    let joint = |n: &str| skeleton.find(n).and_then(|j| u16::try_from(j).ok());
    let surface = SkinSurface::from_cooked(cooked, 0, Some(part), joint, &rest).map_err(|e| e.to_string())?;
    let attach = joint(&spec.attach).ok_or_else(|| format!("no joint {}", spec.attach))?;
    let below = rest[usize::from(attach)].translation.y + spec.attach_offset;
    let colliders = content.cloth_colliders(recipe, animation, below);
    let cook = cook_cloth(&surface, &rest, attach, spec, colliders).map_err(|e| e.to_string())?;
    let decoded = DecodedPart::decode(cooked, 0, Some(part)).map_err(|e| e.to_string())?;
    let decoded = cloth_decoded(&cook, decoded, &surface, skeleton.names(), &rest, |i| joint_name(0, i)).map_err(|e| e.to_string())?;
    let skin: Vec<([u16; 4], [f32; 4])> = cook.targets.iter().map(|s| (s.joints, s.weights)).collect();
    let binding = ClothBinding::new(cook.lattice, spec.params, attach, cook.offsets, &skin, &rest, content.cloth_leg_follow());
    log::debug!("cloth {id}: {} particles, {} on the lattice, {} filled", binding.lattice.particles.len(), cook.rebind.on_lattice, cook.filled);
    Ok(ClothPart { binding: Arc::new(binding), decoded })
}

/// The part's assembly copy with its lattice joints named for cloth `k` of the outfit.
pub(crate) fn renamed(part: &ClothPart, k: usize) -> DecodedPart {
    let mut d = part.decoded.clone();
    if k != 0 {
        for j in &mut d.joints {
            if let Some((_, i)) = parse_joint(&j.name) {
                j.name = joint_name(k, i);
            }
        }
    }
    d
}

/// A lattice joint entity (child of the body): which cloth of the body's
/// animator and which particle it shows.
#[derive(Component, Clone, Copy, Debug)]
pub struct ClothJoint {
    pub body: Entity,
    pub cloth: u16,
    pub particle: u16,
}

/// Bindings waiting for the body's animator (inserted with the outfit).
#[derive(Component)]
pub(crate) struct ClothPending(pub Vec<Arc<ClothBinding>>);

/// Cloth on or off (A/B view: off shows the lattice on its animated
/// targets, i.e. a skinned robe that rides the pelvis and follows the legs).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClothSettings {
    pub simulate: bool,
}

impl Default for ClothSettings {
    fn default() -> Self {
        Self { simulate: true }
    }
}

/// Cloth counters, refreshed every frame.
#[derive(Resource, Clone, Debug, Default)]
pub struct ClothStats {
    /// Cloth parts bound, and those simulated this frame.
    pub parts: usize,
    pub simulated: usize,
    /// The `ClothLattice` op's time this frame summed over parts, µs.
    pub op_us: f32,
}

pub(crate) fn attach_cloth(mut commands: Commands, settings: Res<ClothSettings>, mut pending: Query<(Entity, &mut Animator, &ClothPending)>) {
    for (entity, mut animator, cloth) in &mut pending {
        animator.instance.set_cloth(cloth.0.clone());
        for run in animator.instance.cloth_mut() {
            run.enabled = settings.simulate;
        }
        commands.entity(entity).remove::<ClothPending>();
    }
}

pub(crate) fn apply_cloth_settings(settings: Res<ClothSettings>, mut animators: Query<&mut Animator>) {
    if !settings.is_changed() || settings.is_added() {
        return;
    }
    for mut animator in &mut animators {
        if animator.instance.cloth().is_empty() {
            continue;
        }
        for run in animator.instance.cloth_mut() {
            run.enabled = settings.simulate;
        }
    }
}

/// Poses the lattice joint entities from their animator's `ClothLattice` op.
pub(crate) fn write_cloth_joints(animators: Query<&Animator>, mut joints: Query<(&ClothJoint, &mut Transform)>, mut stats: ResMut<ClothStats>) {
    for (joint, mut transform) in &mut joints {
        let Ok(animator) = animators.get(joint.body) else { continue };
        if !animator.posed() {
            continue;
        }
        let Some(run) = animator.instance.cloth().get(usize::from(joint.cloth)) else { continue };
        let i = usize::from(joint.particle);
        let (Some(&p), Some(&r)) = (run.positions().get(i), run.rotations().get(i)) else { continue };
        let t = Transform { translation: p, rotation: r, scale: Vec3::ONE };
        if *transform != t {
            *transform = t;
        }
    }
    let mut s = ClothStats::default();
    for animator in &animators {
        for run in animator.instance.cloth() {
            s.parts += 1;
            s.simulated += usize::from(run.simulated);
            s.op_us += run.last_us;
        }
    }
    *stats = s;
}

/// A lattice joint at its rest position (before the first posed update).
pub(crate) fn rest_transform(binding: &ClothBinding, particle: usize) -> Transform {
    let p = binding.lattice.particles.get(particle).map_or(Vec3::ZERO, |p| p.rest);
    Transform { translation: p, rotation: Quat::IDENTITY, scale: Vec3::ONE }
}
