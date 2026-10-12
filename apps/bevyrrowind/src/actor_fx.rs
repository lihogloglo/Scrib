//! NIF effects on dressed characters (nif-runtime.md "Actor parts"):
//! visibility and material controllers of actor models, and billboard nodes
//! inside a rig.
//!
//! - **Controllers** come as clip curves (`mw_anim::effects`): the graph
//!   blends them with the pose, and after `AnimSystems::Apply` this module
//!   writes them out. `vis:<node>` hides a joint's subtree by scaling the
//!   joint to zero (the rigid parts bound to it and to its descendants
//!   vanish; Unreal's `HideBoneByName` does the same), and the next pose
//!   restores it. `mat:<material>:<channel>` drives the character's own
//!   copy of that outfit material (outfits share materials across
//!   characters, effects run on each character's animation time).
//! - **Billboards** (`mw_anim::rig::billboard_joints`): after the pose,
//!   each billboard joint turns so its Morrowind axes align with the view
//!   (`mw_assets::billboard`), whatever the rig above it does.
//!
//! Not covered (counted in `mw_bake/tests/actor_fx_real_install.rs`):
//! visibility of skinned geometry nodes merged into an outfit mesh, and
//! particle systems on creatures (`actor_particles`).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::math::Affine2;
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use mw_anim::effects::{Channel, Effect};
use sf_anim_bevy::{AnimSystems, Animator, SkeletonBinding};
use sf_character_bevy::{CharacterBody, CharacterSystems};
use sf_vfs::{Vfs, VfsPath};

/// Billboard joints by skeleton model (parsed once per model).
#[derive(Resource)]
pub struct ActorFxModels {
    vfs: Arc<Vfs>,
    billboards: HashMap<String, Arc<Vec<String>>>,
}

impl ActorFxModels {
    fn billboards(&mut self, skeleton: &str) -> Arc<Vec<String>> {
        let vfs = &self.vfs;
        Arc::clone(self.billboards.entry(skeleton.to_owned()).or_insert_with(|| {
            let model = VfsPath::new(skeleton).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_nif::load(&b).ok());
            Arc::new(model.map(|m| mw_anim::rig::billboard_joints(&m)).unwrap_or_default())
        }))
    }
}

/// One animated outfit material of a character.
#[derive(Debug)]
struct FxMaterial {
    /// The character's own copy.
    handle: Handle<StandardMaterial>,
    /// `(curve index, channel)`.
    channels: Vec<(u16, Channel)>,
    /// Values last written, per channel.
    last: Vec<f32>,
}

/// On a character root: its effect curves bound to joints and materials.
#[derive(Component, Debug, Default)]
pub struct ActorFx {
    /// The body entity (rig parent; `CharacterBody::body`).
    body: Option<Entity>,
    /// The mesh entities and animation asset this was bound for.
    meshes: Vec<Entity>,
    asset: usize,
    /// `(curve index, joint)`: hidden while the curve is below 0.5.
    joints: Vec<(u16, Entity)>,
    materials: Vec<FxMaterial>,
    /// Joints turned to face the camera.
    billboards: Vec<Entity>,
    /// Curve values (graph curve order); the test seam between the animator
    /// and the write-out.
    values: Vec<f32>,
}

impl ActorFx {
    fn is_empty(&self) -> bool {
        self.joints.is_empty() && self.materials.is_empty() && self.billboards.is_empty()
    }
}

/// The character has no effects for its current outfit and animation.
#[derive(Component)]
struct NoActorFx {
    meshes: Vec<Entity>,
}

pub fn add(app: &mut App, vfs: Arc<Vfs>) {
    app.insert_resource(ActorFxModels { vfs, billboards: HashMap::new() })
        .add_systems(Update, bind.after(CharacterSystems))
        .add_systems(PostUpdate, (read_curves, apply_fx).chain().after(AnimSystems::Apply).before(TransformSystems::Propagate));
}

/// Binds effects on characters whose outfit or rig changed.
#[allow(clippy::type_complexity)]
fn bind(
    mut commands: Commands,
    mut models: ResMut<ActorFxModels>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    roots: Query<(Entity, &CharacterBody, Option<&ActorFx>, Option<&NoActorFx>)>,
    bodies: Query<(&Animator, &SkeletonBinding)>,
) {
    for (entity, body, fx, none) in &roots {
        let Ok((animator, binding)) = bodies.get(body.body) else { continue };
        let asset = Arc::as_ptr(&animator.asset) as usize;
        if fx.is_some_and(|f| f.meshes == body.meshes && f.asset == asset) || none.is_some_and(|n| n.meshes == body.meshes) {
            continue;
        }
        let skeleton = &animator.asset.skeleton;
        let mut out = ActorFx { body: Some(body.body), meshes: body.meshes.clone(), asset, ..ActorFx::default() };
        let mut by_material: HashMap<&str, Vec<(u16, Channel)>> = HashMap::new();
        for (i, name) in animator.asset.graph.curves.iter().enumerate() {
            let Ok(i) = u16::try_from(i) else { break };
            match mw_anim::effects::parse(name) {
                Some(Effect::Visibility { node }) => {
                    // Nodes that aren't joints (skinned geometry merged into an outfit mesh) stay shown.
                    if let Some(joint) = binding.find(skeleton, node) {
                        out.joints.push((i, joint));
                    }
                }
                Some(Effect::Material { material, channel }) => by_material.entry(material).or_default().push((i, channel)),
                Some(Effect::Emission { .. }) | None => {} // emission: actor_particles
            }
        }
        for (material, channels) in by_material {
            let mut copy = None;
            for (m, &mesh) in body.outfit.meshes.iter().zip(&body.meshes) {
                if !m.material_name.eq_ignore_ascii_case(material) {
                    continue;
                }
                let handle = copy.get_or_insert_with(|| {
                    let source = materials.get(&m.material).cloned().unwrap_or_default();
                    materials.add(source)
                });
                commands.entity(mesh).insert(MeshMaterial3d(handle.clone()));
            }
            if let Some(handle) = copy {
                let last = vec![f32::NAN; channels.len()];
                out.materials.push(FxMaterial { handle, channels, last });
            }
        }
        out.billboards = models.billboards(&body.skeleton).iter().filter_map(|name| binding.find(skeleton, name)).collect();
        if out.is_empty() {
            commands.entity(entity).remove::<ActorFx>().insert(NoActorFx { meshes: body.meshes.clone() });
        } else {
            debug!(
                "actor fx {entity}: {} hidden joints, {} materials, {} billboards",
                out.joints.len(),
                out.materials.len(),
                out.billboards.len()
            );
            commands.entity(entity).remove::<NoActorFx>().insert(out);
        }
    }
}

/// The animator's blended curve values into [`ActorFx::values`].
fn read_curves(mut roots: Query<&mut ActorFx>, animators: Query<&Animator>) {
    for mut fx in &mut roots {
        let Some(Ok(animator)) = fx.body.map(|b| animators.get(b)) else { continue };
        if animator.posed() && fx.values != animator.output.curves {
            fx.values.clone_from(&animator.output.curves);
        }
    }
}

/// Writes visibility, materials and billboards (after the pose, before
/// transform propagation).
#[allow(clippy::needless_pass_by_value)]
fn apply_fx(
    mut roots: Query<&mut ActorFx>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform, Without<Camera3d>>,
    parents: Query<&ChildOf>,
    globals: Query<&GlobalTransform>,
    camera: Query<&Transform, With<Camera3d>>,
) {
    let camera = camera.single().ok().map(|c| mw_assets::billboard::billboard_rotation(c.rotation));
    for mut fx in &mut roots {
        let fx = &mut *fx;
        let value = |i: u16| fx.values.get(usize::from(i)).copied();
        for &(curve, joint) in &fx.joints {
            if value(curve).is_some_and(|v| v < 0.5)
                && let Ok(mut t) = transforms.get_mut(joint)
                && t.scale != Vec3::ZERO
            {
                t.scale = Vec3::ZERO;
            }
        }
        for m in &mut fx.materials {
            let now: Vec<f32> = m.channels.iter().map(|&(c, _)| value(c).unwrap_or(f32::NAN)).collect();
            if now.iter().zip(&m.last).all(|(a, b)| a.to_bits() == b.to_bits()) {
                continue;
            }
            if let Some(mut material) = materials.get_mut(&m.handle) {
                apply_material(&mut material, m.channels.iter().map(|&(_, c)| c).zip(now.iter().copied()));
            }
            m.last = now;
        }
        let Some(view) = camera else { continue };
        for &joint in &fx.billboards {
            // The parent's world rotation from this frame's local poses up to
            // the body (last frame's global there: the placement).
            let mut parent = Quat::IDENTITY;
            let mut at = parents.get(joint).map(ChildOf::parent).ok();
            while let Some(e) = at {
                if Some(e) == fx.body {
                    parent = globals.get(e).map_or(Quat::IDENTITY, |g| g.compute_transform().rotation) * parent;
                    break;
                }
                parent = transforms.get(e).map_or(Quat::IDENTITY, |t| t.rotation) * parent;
                at = parents.get(e).map(ChildOf::parent).ok();
            }
            if let Ok(mut t) = transforms.get_mut(joint) {
                t.rotation = parent.inverse() * view;
            }
        }
    }
}

/// Material channel values (NIF units) onto a material. Untouched UV
/// channels keep offset 0 and tiling 1; NaN values are skipped.
pub fn apply_material(m: &mut StandardMaterial, values: impl Iterator<Item = (Channel, f32)>) {
    let mut base = m.base_color.to_srgba();
    let mut emissive: Option<[f32; 3]> = None;
    let mut uv: Option<[f32; 4]> = None;
    for (channel, v) in values.filter(|(_, v)| !v.is_nan()) {
        match channel {
            Channel::Alpha => base.alpha = v.clamp(0.0, 1.0),
            Channel::Diffuse(0) => base.red = v,
            Channel::Diffuse(1) => base.green = v,
            Channel::Diffuse(_) => base.blue = v,
            Channel::Emissive(i) => emissive.get_or_insert([0.0; 3])[i.min(2)] = v,
            Channel::UvOffset(i) => uv.get_or_insert([0.0, 0.0, 1.0, 1.0])[i.min(1)] = v,
            Channel::UvTiling(i) => uv.get_or_insert([0.0, 0.0, 1.0, 1.0])[2 + i.min(1)] = v,
        }
    }
    m.base_color = base.into();
    if let Some([r, g, b]) = emissive {
        // As `mw_assets` maps NIF emissive: glowing through the base texture.
        if m.emissive_texture.is_none() {
            m.emissive_texture.clone_from(&m.base_color_texture);
        }
        m.emissive = Color::srgb(r, g, b).to_linear() * mw_assets::emissive_nits();
    }
    if let Some([u, v, su, sv]) = uv {
        // Tiling scales about the texture center, then offsets (as `mw_assets`).
        let scale = Vec2::new(su, sv);
        m.uv_transform = Affine2::from_scale_angle_translation(scale, 0.0, Vec2::splat(0.5) - scale * 0.5 + Vec2::new(u, v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_channels_drive_alpha_color_and_uv() {
        let mut m = StandardMaterial::default();
        apply_material(&mut m, [(Channel::Alpha, 0.25), (Channel::Diffuse(0), 0.5), (Channel::UvOffset(1), 0.25)].into_iter());
        let c = m.base_color.to_srgba();
        assert!((c.alpha - 0.25).abs() < 1e-6 && (c.red - 0.5).abs() < 1e-6 && (c.green - 1.0).abs() < 1e-6);
        assert!((m.uv_transform.translation - Vec2::new(0.0, 0.25)).length() < 1e-6);
    }

    /// A character-shaped hierarchy: body (placed) -> spinning joint ->
    /// billboard joint, plus a hidden joint and an animated material.
    #[test]
    fn effects_follow_curves_and_billboards_face_the_camera_while_the_rig_moves() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), TransformPlugin))
            .init_asset::<StandardMaterial>()
            .add_systems(PostUpdate, apply_fx.before(TransformSystems::Propagate));
        let camera_rotation = Quat::from_rotation_y(0.6) * Quat::from_rotation_x(-0.2);
        app.world_mut().spawn((Camera3d::default(), Transform::from_rotation(camera_rotation)));
        let world = app.world_mut();
        let material = world.resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
        let body = world.spawn(Transform::from_rotation(Quat::from_rotation_y(1.0))).id();
        let spin = world.spawn((Transform::default(), ChildOf(body))).id();
        let billboard = world.spawn((Transform::default(), ChildOf(spin))).id();
        let hidden = world.spawn((Transform::default(), ChildOf(body))).id();
        let fx = ActorFx {
            joints: vec![(0, hidden)],
            materials: vec![FxMaterial { handle: material.clone(), channels: vec![(1, Channel::Alpha)], last: vec![f32::NAN] }],
            body: Some(body),
            billboards: vec![billboard],
            values: vec![1.0, 1.0],
            ..ActorFx::default()
        };
        let root = world.spawn(fx).id();

        let expected = mw_assets::billboard::billboard_rotation(camera_rotation);
        for (step, (spin_angle, shown, alpha)) in [(0.0, 1.0, 1.0), (0.9, 0.0, 0.3), (2.1, 1.0, 0.6)].into_iter().enumerate() {
            // The "animation": the parent joint turns, the curves move.
            app.world_mut().get_mut::<Transform>(spin).unwrap().rotation = Quat::from_rotation_z(spin_angle);
            app.world_mut().get_mut::<ActorFx>(root).unwrap().values = vec![shown, alpha];
            if shown > 0.5 {
                app.world_mut().get_mut::<Transform>(hidden).unwrap().scale = Vec3::ONE; // the pose's write
            }
            app.update();
            app.update(); // the body's global settles (placement: last frame's)
            let world = app.world();
            let q = world.get::<GlobalTransform>(billboard).unwrap().compute_transform().rotation;
            assert!(q.angle_between(expected) < 1e-3, "step {step}: billboard off by {} rad", q.angle_between(expected));
            let scale = world.get::<Transform>(hidden).unwrap().scale;
            assert_eq!(scale == Vec3::ZERO, shown < 0.5, "step {step}: visibility");
            let a = world.resource::<Assets<StandardMaterial>>().get(&material).unwrap().base_color.alpha();
            assert!((a - alpha).abs() < 1e-6, "step {step}: alpha {a}");
        }
    }
}
