//! Plays NIF controllers: material animation per model asset, node
//! animation (keyframes, visibility) per instance. Sampling lives in
//! [`mw_nif::anim`]; this only writes the results into Bevy.

use bevy::math::Affine2;
use bevy::prelude::*;
use mw_nif::anim::{ColorSlot, Controller, Target};
use mw_nif::particles::ParticleSystem;

/// Everything animated in one model, shared by its instances.
#[derive(Asset, TypePath)]
pub struct NifAnimation {
    pub controllers: Vec<Controller>,
    pub particles: Vec<ParticleSystem>,
    /// The model's materials, by [`mw_nif::Model::materials`] index.
    pub materials: Vec<Handle<StandardMaterial>>,
}

/// On a model instance's root: the model's animation data.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ModelAnimation(pub Handle<NifAnimation>);

/// On a node entity: the controllers (indices into
/// [`NifAnimation::controllers`]) that move or hide it.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct NodeAnimation {
    pub animation: Handle<NifAnimation>,
    pub controllers: Vec<u32>,
}

/// Per-instance time offset, so copies of a model don't move in lockstep.
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimationPhase(pub f32);

/// Sample rate for shared NIF material controllers (UV scroll, alpha/color
/// pulse). Stepping at 30 Hz matches Morrowind's authoring rate while cutting
/// render-world `PreparedMaterial` bind-group and specialization churn by 3x
/// at 90 fps.
const MATERIAL_ANIM_HZ: f32 = 30.0;

/// Material controllers. Materials are shared by every instance of a model,
/// so they animate once per model, on the global clock.
pub(crate) fn animate_materials(
    time: Res<Time>,
    mut anim_events: MessageReader<AssetEvent<NifAnimation>>,
    animations: Res<Assets<NifAnimation>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut last_step: Local<Option<u32>>,
) {
    let elapsed = time.elapsed_secs().max(0.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let step = (elapsed * MATERIAL_ANIM_HZ).floor() as u32;
    let added_or_modified = anim_events
        .read()
        .any(|e| matches!(e, AssetEvent::Added { .. } | AssetEvent::Modified { .. } | AssetEvent::LoadedWithDependencies { .. }));
    if !added_or_modified && *last_step == Some(step) {
        return;
    }
    *last_step = Some(step);
    #[allow(clippy::cast_precision_loss)]
    let now = step as f32 / MATERIAL_ANIM_HZ;
    for (_, animation) in animations.iter() {
        for controller in animation.controllers.iter().filter(|c| c.auto_play) {
            let t = controller.timing.key_time(now);
            match &controller.target {
                Target::Alpha { material, curve } => {
                    let Some(alpha) = curve.sample(t).map(|a| a.clamp(0.0, 1.0)) else { continue };
                    let Some(handle) = animation.materials.get(*material) else { continue };
                    let Some(cur) = materials.get(handle) else { continue };
                    if (cur.base_color.alpha() - alpha).abs() > 1e-3
                        && let Some(mut m) = materials.get_mut(handle)
                    {
                        m.base_color.set_alpha(alpha);
                    }
                }
                Target::MaterialColor { material, slot, curve } => {
                    if matches!(slot, ColorSlot::Ambient | ColorSlot::Specular) {
                        continue;
                    }
                    let Some(c) = curve.sample(t) else { continue };
                    let Some(handle) = animation.materials.get(*material) else { continue };
                    let Some(cur) = materials.get(handle) else { continue };
                    match slot {
                        ColorSlot::Diffuse => {
                            let srgba = cur.base_color.to_srgba();
                            if ((srgba.red - c.x).abs() > 1e-3 || (srgba.green - c.y).abs() > 1e-3 || (srgba.blue - c.z).abs() > 1e-3)
                                && let Some(mut m) = materials.get_mut(handle)
                            {
                                m.base_color = Color::srgba(c.x, c.y, c.z, srgba.alpha);
                            }
                        }
                        ColorSlot::Emissive => {
                            let linear = Color::srgb(c.x, c.y, c.z).to_linear();
                            let target = linear * crate::emissive_nits();
                            let dirty = (cur.emissive.red - target.red).abs() > 1e-2
                                || (cur.emissive.green - target.green).abs() > 1e-2
                                || (cur.emissive.blue - target.blue).abs() > 1e-2
                                || (cur.emissive_exposure_weight - 1.0).abs() > 1e-4;
                            if dirty && let Some(mut m) = materials.get_mut(handle) {
                                crate::set_emissive(&mut m, linear);
                            }
                        }
                        ColorSlot::Ambient | ColorSlot::Specular => {}
                    }
                }
                Target::Uv { material, track } => {
                    let Some(handle) = animation.materials.get(*material) else { continue };
                    let Some(cur) = materials.get(handle) else { continue };
                    let ([u, v], [su, sv]) = track.sample(t);
                    // Tiling scales about the texture center, then offsets.
                    let scale = Vec2::new(su, sv);
                    let next = Affine2::from_scale_angle_translation(scale, 0.0, Vec2::splat(0.5) - scale * 0.5 + Vec2::new(u, v));
                    if !cur.uv_transform.abs_diff_eq(next, 1e-4)
                        && let Some(mut m) = materials.get_mut(handle)
                    {
                        m.uv_transform = next;
                    }
                }
                Target::Transform { .. } | Target::Visibility { .. } => {}
            }
        }
    }
}

/// Gives each newly spawned animated node its instance's phase. Nodes of
/// one instance share a phase (a machine's parts stay in step).
pub(crate) fn assign_phases(
    mut commands: Commands,
    added: Query<(Entity, &NodeAnimation), Without<AnimationPhase>>,
    parents: Query<&ChildOf>,
    animations: Res<Assets<NifAnimation>>,
) {
    for (entity, node) in &added {
        let Some(animation) = animations.get(&node.animation) else { continue };
        let random = node.controllers.iter().filter_map(|&i| animation.controllers.get(i as usize)).any(|c| c.random_start);
        let phase = if random {
            let root = parents.iter_ancestors(entity).last().unwrap_or(entity);
            seed_unit(root.to_bits()) * 600.0
        } else {
            0.0
        };
        commands.entity(entity).try_insert(AnimationPhase(phase));
    }
}

/// Keyframe and visibility controllers, per instance.
pub(crate) fn animate_nodes(
    time: Res<Time>,
    animations: Res<Assets<NifAnimation>>,
    mut nodes: Query<(&NodeAnimation, &AnimationPhase, &mut Transform, &mut Visibility)>,
) {
    let now = time.elapsed_secs();
    for (node, phase, mut transform, mut visibility) in &mut nodes {
        let Some(animation) = animations.get(&node.animation) else { continue };
        for &index in &node.controllers {
            let Some(controller) = animation.controllers.get(index as usize).filter(|c| c.auto_play) else { continue };
            let t = controller.timing.key_time(now + phase.0);
            match &controller.target {
                Target::Transform { track, .. } => {
                    if let Some(p) = track.translation.as_ref().and_then(|c| c.sample(t)) {
                        let next = Vec3::from_array(p.to_array());
                        if !transform.translation.abs_diff_eq(next, 1e-5) {
                            transform.translation = next;
                        }
                    }
                    if let Some(q) = track.rotation.as_ref().and_then(|r| r.sample(t)) {
                        let next = Quat::from_array(q.to_array());
                        if !transform.rotation.abs_diff_eq(next, 1e-5) {
                            transform.rotation = next;
                        }
                    }
                    if let Some(s) = track.scale.as_ref().and_then(|c| c.sample(t)) {
                        let next = Vec3::splat(s);
                        if !transform.scale.abs_diff_eq(next, 1e-5) {
                            transform.scale = next;
                        }
                    }
                }
                Target::Visibility { keys, .. } => {
                    let shown = keys.sample(t).unwrap_or(true);
                    visibility.set_if_neq(if shown { Visibility::Inherited } else { Visibility::Hidden });
                }
                _ => {}
            }
        }
    }
}

/// A stable pseudo-random value in `[0, 1)` for a seed.
pub(crate) fn seed_unit(seed: u64) -> f32 {
    let mut h = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    h = (h ^ (h >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    h ^= h >> 31;
    #[allow(clippy::cast_precision_loss)]
    let unit = (h >> 40) as f32 / (1u64 << 24) as f32;
    unit
}
