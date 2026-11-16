//! Plays NIF controllers: material animation per model asset, node
//! animation (keyframes, visibility) per instance. Sampling lives in
//! [`mw_nif::anim`]; this only writes the results into Bevy.

use bevy::math::Affine2;
use bevy::platform::collections::{HashMap, HashSet};
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

/// `1`: material controllers skip materials no visible entity draws (ROADMAP 1.25 CW-6). Opt-in until its
/// interleaved A/B (perf arm `--env BEVYRROWIND_MATCTRL_GATE=1`).
const ENV_MATCTRL_GATE: &str = "BEVYRROWIND_MATCTRL_GATE";

/// Whether `BEVYRROWIND_MATCTRL_GATE=1` is set.
pub(crate) fn gate_from_env() -> bool {
    std::env::var(ENV_MATCTRL_GATE).is_ok_and(|v| v.trim() == "1")
}

/// On a NIF mesh node whose material a material controller drives: that
/// material. It outlives a later material swap on the entity (blended
/// surfaces trade their `MeshMaterial3d<StandardMaterial>` for a fogged
/// copy that mirrors the source), so the entity still counts as a user.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AnimatedMaterialUser(pub Handle<StandardMaterial>);

/// Visibility gate of the material controllers (`BEVYRROWIND_MATCTRL_GATE=1`): the
/// entities drawing each `StandardMaterial`, kept on spawn and despawn by observers
/// (holders of `MeshMaterial3d<StandardMaterial>` and of [`AnimatedMaterialUser`]),
/// never by a per-frame scan. Present only while the gate is on.
///
/// A material never seen on an entity is drawn by something we don't track:
/// it animates as without the gate. A tracked material with no user left is
/// drawn by nobody and is skipped.
#[derive(Resource, Default, Debug)]
pub struct MaterialUsers {
    users: HashMap<AssetId<StandardMaterial>, Vec<Entity>>,
}

impl MaterialUsers {
    fn add(&mut self, id: AssetId<StandardMaterial>, entity: Entity) {
        self.users.entry(id).or_default().push(entity);
    }

    /// Drops one registration (an entity with both a marker and the material holds two).
    fn remove(&mut self, id: AssetId<StandardMaterial>, entity: Entity) {
        if let Some(list) = self.users.get_mut(&id)
            && let Some(i) = list.iter().position(|&e| e == entity)
        {
            list.swap_remove(i);
        }
    }

    /// Entities registered as drawing `id` (`None`: never tracked).
    pub fn users(&self, id: AssetId<StandardMaterial>) -> Option<&[Entity]> {
        self.users.get(&id).map(Vec::as_slice)
    }

    /// Whether `id` must be animated now: untracked, or drawn by an entity that
    /// was visible in a view last frame.
    fn wanted(&self, id: AssetId<StandardMaterial>, visibility: &Query<&ViewVisibility>) -> bool {
        self.users.get(&id).is_none_or(|list| list.iter().any(|&e| visibility.get(e).is_ok_and(|v| v.get())))
    }
}

/// Turns the visibility gate on: [`MaterialUsers`] and the observers that keep it.
pub(crate) fn add_material_gate(app: &mut App) {
    app.init_resource::<MaterialUsers>()
        .add_observer(|insert: On<Insert<MeshMaterial3d<StandardMaterial>>>, q: Query<&MeshMaterial3d<StandardMaterial>>, mut users: ResMut<MaterialUsers>| {
            if let Ok(m) = q.get(insert.entity) {
                users.add(m.id(), insert.entity);
            }
        })
        .add_observer(|discard: On<Discard<MeshMaterial3d<StandardMaterial>>>, q: Query<&MeshMaterial3d<StandardMaterial>>, mut users: ResMut<MaterialUsers>| {
            if let Ok(m) = q.get(discard.entity) {
                users.remove(m.id(), discard.entity);
            }
        })
        .add_observer(|insert: On<Insert<AnimatedMaterialUser>>, q: Query<&AnimatedMaterialUser>, mut users: ResMut<MaterialUsers>| {
            if let Ok(m) = q.get(insert.entity) {
                users.add(m.0.id(), insert.entity);
            }
        })
        .add_observer(|discard: On<Discard<AnimatedMaterialUser>>, q: Query<&AnimatedMaterialUser>, mut users: ResMut<MaterialUsers>| {
            if let Ok(m) = q.get(discard.entity) {
                users.remove(m.0.id(), discard.entity);
            }
        });
}

/// Material controllers skipped by the gate at the current step, retried every
/// frame until a user shows (`(animation, controller index)`).
#[derive(Default)]
pub(crate) struct MaterialAnimState {
    last_step: Option<u32>,
    stale: HashSet<(AssetId<NifAnimation>, usize)>,
}

/// The material a material controller drives (`None` for node controllers
/// and slots we don't render).
fn controlled_material<'a>(animation: &'a NifAnimation, controller: &Controller) -> Option<&'a Handle<StandardMaterial>> {
    material_index(&controller.target).and_then(|i| animation.materials.get(i))
}

/// The model material index a material controller target drives.
fn material_index(target: &Target) -> Option<usize> {
    match target {
        Target::Alpha { material, .. }
        | Target::Uv { material, .. }
        | Target::MaterialColor { material, slot: ColorSlot::Diffuse | ColorSlot::Emissive, .. } => Some(*material),
        Target::MaterialColor { .. } | Target::Transform { .. } | Target::Visibility { .. } => None,
    }
}

/// Per model material: whether a played controller drives it (its mesh nodes get an
/// [`AnimatedMaterialUser`]).
pub(crate) fn animated_materials(controllers: &[Controller], count: usize) -> Vec<bool> {
    let mut animated = vec![false; count];
    for index in controllers.iter().filter(|c| c.auto_play).filter_map(|c| material_index(&c.target)) {
        if let Some(slot) = animated.get_mut(index) {
            *slot = true;
        }
    }
    animated
}

/// Material controllers. Materials are shared by every instance of a model,
/// so they animate once per model, on the global clock: a pure function of
/// the step time, so a material the gate skipped gets the right value the
/// moment it is wanted again. Each write is an asset change (render-world
/// re-prepare, bind-group slot rewrite), so the asset is compared first and
/// written only when the value differs.
///
/// With [`MaterialUsers`] (the gate): at each step, materials no visible entity
/// draws are skipped and remembered; between steps only those are retried, so
/// one coming into view gets its step value on the next frame (`ViewVisibility`
/// is decided in `PostUpdate`, after the asset events this frame's writes need).
pub(crate) fn animate_materials(
    time: Res<Time>,
    mut anim_events: MessageReader<AssetEvent<NifAnimation>>,
    animations: Res<Assets<NifAnimation>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gate: Option<Res<MaterialUsers>>,
    visibility: Query<&ViewVisibility>,
    mut state: Local<MaterialAnimState>,
) {
    let elapsed = time.elapsed_secs().max(0.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let step = (elapsed * MATERIAL_ANIM_HZ).floor() as u32;
    let added_or_modified = anim_events
        .read()
        .any(|e| matches!(e, AssetEvent::Added { .. } | AssetEvent::Modified { .. } | AssetEvent::LoadedWithDependencies { .. }));
    let full = added_or_modified || state.last_step != Some(step);
    if !full && (gate.is_none() || state.stale.is_empty()) {
        return;
    }
    state.last_step = Some(step);
    #[allow(clippy::cast_precision_loss)]
    let now = step as f32 / MATERIAL_ANIM_HZ;
    let wanted = |handle: &Handle<StandardMaterial>| gate.as_ref().is_none_or(|g| g.wanted(handle.id(), &visibility));
    if full {
        state.stale.clear();
        for (id, animation) in animations.iter() {
            for (index, controller) in animation.controllers.iter().enumerate().filter(|(_, c)| c.auto_play) {
                let Some(handle) = controlled_material(animation, controller) else { continue };
                if wanted(handle) {
                    apply_material_controller(controller, handle, controller.timing.key_time(now), &mut materials);
                } else {
                    state.stale.insert((id, index));
                }
            }
        }
    } else {
        state.stale.retain(|&(id, index)| {
            let Some(animation) = animations.get(id) else { return false };
            let Some(controller) = animation.controllers.get(index) else { return false };
            let Some(handle) = controlled_material(animation, controller) else { return false };
            if !wanted(handle) {
                return true;
            }
            apply_material_controller(controller, handle, controller.timing.key_time(now), &mut materials);
            false
        });
    }
}

/// Writes one material controller's value at key time `t` into its material,
/// only when it differs from what the asset holds (`get` first, `get_mut` on change).
fn apply_material_controller(controller: &Controller, handle: &Handle<StandardMaterial>, t: f32, materials: &mut Assets<StandardMaterial>) {
    let Some(cur) = materials.get(handle) else { return };
    match &controller.target {
        Target::Alpha { curve, .. } => {
            let Some(alpha) = curve.sample(t).map(|a| a.clamp(0.0, 1.0)) else { return };
            if (cur.base_color.alpha() - alpha).abs() > 1e-3
                && let Some(mut m) = materials.get_mut(handle)
            {
                m.base_color.set_alpha(alpha);
            }
        }
        Target::MaterialColor { slot, curve, .. } => {
            let Some(c) = curve.sample(t) else { return };
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
        Target::Uv { track, .. } => {
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

#[cfg(test)]
#[path = "animation_tests.rs"]
mod tests;
