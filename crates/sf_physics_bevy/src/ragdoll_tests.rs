//! R0 (PH-2): after write-back every ragdoll body's world pose matches its
//! owner bone's world transform, while the character frame is moved, yawed,
//! non-uniformly scaled and cell-rebased mid-fall on a slope, in both modes.

#![allow(clippy::cast_precision_loss)]

use std::sync::Arc;
use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, IVec3, Quat, Vec3};
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_anim::fixtures::{biped, gait};
use sf_anim::{ClipRules, CookSettings, Looping, RigHints, RootMotionMode, RootMotionRules, cook_clip, detect_profile};
use sf_anim_bevy::{AnimAsset, AnimLod, AnimRootMotion, AnimTier, Animator, PhysicsPose, SfAnimPlugin, SkeletonBinding, spawn_rig};
use sf_anim_graph::{CompileSource, compile_str};
use sf_bake::ProductKey;
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, Layer, ShapeRef};
use sf_ragdoll::{CookParams, RagdollDef, cook};
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, WorldspaceRoot, from_grid};

use super::{Ragdoll, RagdollMode, RagdollSource, RagdollStats, Ragdolls};
use crate::{Physics, PhysicsFocus, PhysicsPlugin};

struct Walk(sf_anim::Clip);

impl CompileSource for Walk {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, sf_anim::Clip), String> {
        Ok((ProductKey::new("anim.cook", reference), self.0.clone()))
    }
    fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
        Err("none".into())
    }
}

/// The fixture biped walking with root motion applied to its frame (so the
/// clip's root keeps moving under the ragdoll).
fn walk_asset() -> Arc<AnimAsset> {
    let skeleton = biped();
    let src = gait(&skeleton, "walk", 1.0, 1.4, 0.5, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
    let rules = ClipRules { looping: Looping::Whole, root_motion: RootMotionRules { mode: RootMotionMode::Drive, ..RootMotionRules::default() }, ..ClipRules::default() };
    let clip = cook_clip(&skeleton, &src, &rules, &CookSettings::default()).unwrap();
    let ron = r#"(nodes: {"walk": Clip(clip: "walk")}, layers: [(node: "walk")])"#;
    let graph = compile_str("walk", ron, &skeleton, &mut Walk(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    Arc::new(AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap())
}

struct Scene {
    app: App,
    ws: WorldspaceId,
    root: Entity,
    body: Entity,
    def: Arc<RagdollDef>,
}

fn scene(mode: impl FnOnce(WorldPos) -> RagdollMode) -> Scene {
    let ws = WorldspaceId::new("r0");
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default(), SfAnimPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())));
    let asset = walk_asset();
    let profile = detect_profile(&asset.skeleton, &RigHints::default()).unwrap();
    let def = Arc::new(cook(&asset.skeleton, &profile, &CookParams { total_mass: Some(75.0), ..CookParams::default() }).unwrap());
    let loader = def.clone();
    app.insert_resource(RagdollSource(Arc::new(move |_| Some(loader.clone()))));

    // A 20° slope whose top surface passes through `base`.
    let base = WorldPos::new(IVec3::new(3, 0, -2), Vec3::new(10.0, 5.0, 20.0));
    let tilt = Quat::from_rotation_z(20f32.to_radians());
    let slope = WorldTransform { pos: base.offset(tilt * Vec3::new(0.0, -0.5, 0.0)), rotation: tilt, scale: Vec3::ONE };
    app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::fixed(slope, ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC));

    // Worldspace root → character root (grid cell, the mover) → body (race
    // scale, own yaw, the animator), as `sf_character_bevy` builds it.
    let grid = app.world_mut().spawn(WorldspaceRoot { id: ws.clone(), layer: 0 }).id();
    let (cell, mut t) = sf_world_bevy::to_grid(base.offset(Vec3::new(0.0, 0.6, 0.0)));
    t.rotation = Quat::from_rotation_y(0.4);
    let root = app.world_mut().spawn((cell, t, ChildOf(grid))).id();
    let body_t = Transform { translation: Vec3::ZERO, rotation: Quat::from_rotation_y(0.3), scale: Vec3::new(0.95, 0.9, 0.95) };
    let body = app
        .world_mut()
        .spawn((body_t, Animator::new(asset.clone()), AnimLod::forced(AnimTier::A0), AnimRootMotion { apply: true, ..Default::default() }, ChildOf(root)))
        .id();
    let world = app.world_mut();
    let mut commands = world.commands();
    spawn_rig(&mut commands, body, &asset.skeleton);
    world.flush();
    let ragdoll = Ragdoll { skeleton: "biped".into(), delay: 0.1, mode: mode(base) };
    app.world_mut().entity_mut(body).insert(ragdoll);
    Scene { app, ws, root, body, def }
}

/// Largest position (m) and angle (°) error between each body and its owner bone.
fn errors(s: &mut Scene) -> Option<(f32, f32)> {
    let world = s.app.world();
    let live = world.resource::<Ragdolls>();
    let (_, _, instance) = live.iter().find(|(e, ..)| *e == s.body)?;
    let alpha = world.resource::<Time<Fixed>>().overstep_fraction();
    let w = world.resource::<Physics>().world(&s.ws)?;
    let (cell, root_t) = (world.get::<CellCoord>(s.root)?, world.get::<Transform>(s.root)?);
    let composed = root_t.mul_transform(*world.get::<Transform>(s.body)?);
    let origin = from_grid(cell, &composed);
    let frame = Affine3A::from_scale_rotation_translation(composed.scale, composed.rotation, Vec3::ZERO);
    let animator = world.get::<Animator>(s.body)?;
    let skeleton = &animator.asset.skeleton;
    let binding = world.get::<SkeletonBinding>(s.body)?;
    let mut model = vec![Affine3A::IDENTITY; skeleton.len()];
    for j in 0..skeleton.len() {
        let local = world.get::<Transform>(binding.joints[j])?.compute_affine();
        model[j] = skeleton.parent(j).map_or(frame, |p| model[p]) * local;
    }
    let (mut dp, mut da) = (0.0f32, 0.0f32);
    for (b, &id) in s.def.bodies.iter().zip(&instance.bodies) {
        let pose = w.pose_at(id, alpha)?;
        let bone = model[usize::from(b.joint)];
        let expected = bone.transform_point3(b.local.0);
        dp = dp.max(expected.distance(origin.delta_f32(pose.pos)));
        let (_, r, _) = bone.to_scale_rotation_translation();
        da = da.max((r * b.local.1).angle_between(pose.rotation).to_degrees());
    }
    Some((dp, da))
}

fn run(label: &str, mode: impl FnOnce(WorldPos) -> RagdollMode) {
    let mut s = scene(mode);
    let edge = from_grid(&CellCoord::new(0, 0, 0), &Transform::IDENTITY).delta_f32(from_grid(&CellCoord::new(1, 0, 0), &Transform::IDENTITY));
    let mut worst: Vec<(&str, f32, f32)> = Vec::new();
    let mut phase = "fall";
    let mut travel: (Option<WorldPos>, f32) = (None, 0.0);
    for frame in 0..100 {
        match frame {
            // The host moves and yaws the mover mid-fall.
            40 => {
                phase = "moved+yawed";
                let mut t = s.app.world_mut().get_mut::<Transform>(s.root).unwrap();
                t.translation += Vec3::new(0.3, 0.0, -0.2);
                t.rotation *= Quat::from_rotation_y(0.6);
            }
            // big_space recentres the mover one cell over (same place).
            55 => {
                phase = "cell-rebased";
                let root = s.root;
                let world = s.app.world_mut();
                let c = *world.get::<CellCoord>(root).unwrap();
                *world.get_mut::<CellCoord>(root).unwrap() = CellCoord::new(c.x + 1, c.y, c.z);
                world.get_mut::<Transform>(root).unwrap().translation -= edge;
            }
            // The physics world rebases its anchor.
            65 => {
                phase = "physics-rebased";
                s.app.world_mut().resource_mut::<PhysicsFocus>().0 = Some(WorldPos::new(IVec3::new(7, 0, -2), Vec3::ZERO));
            }
            _ => {}
        }
        s.app.update();
        let full = s.app.world().get::<PhysicsPose>(s.body).is_some_and(|p| p.weight >= 1.0);
        if !full {
            continue;
        }
        let Some((dp, da)) = errors(&mut s) else { continue };
        let world = s.app.world();
        let pelvis = world.resource::<Ragdolls>().iter().find(|(e, ..)| *e == s.body).map(|(_, _, i)| i.bodies[0]);
        if let Some(p) = pelvis.and_then(|id| world.resource::<Physics>().world(&s.ws)?.pose(id)) {
            let start = *travel.0.get_or_insert(p.pos);
            travel.1 = travel.1.max(start.delta_f32(p.pos).length());
        }
        match worst.iter_mut().find(|w| w.0 == phase) {
            Some(w) => {
                w.1 = w.1.max(dp);
                w.2 = w.2.max(da);
            }
            None => worst.push((phase, dp, da)),
        }
    }
    eprintln!("{label}: worst body-vs-bone error per phase (m, °): {worst:?}; pelvis travelled {:.2} m", travel.1);
    assert!(travel.1 > 0.3, "{label}: the ragdoll barely moved ({} m)", travel.1);
    assert!(worst.len() >= 3, "{label}: the ragdoll ended early: {worst:?}");
    for (phase, dp, da) in &worst {
        assert!(*dp <= 0.01 && *da <= 2.0, "{label} / {phase}: {dp:.3} m, {da:.2}° (gate 1 cm, 2°)");
    }
}

#[test]
fn bones_follow_bodies_death() {
    run("death", |_| RagdollMode::Death { crumple: false });
}

#[test]
fn bones_follow_bodies_knockdown() {
    run("knockdown", |base| RagdollMode::Knockdown {
        point: base.offset(Vec3::new(0.0, 1.6, 0.0)),
        impulse: Vec3::new(-60.0, 0.0, 0.0),
        get_up: None,
        get_up_flag: "down".into(),
    });
}

/// R2: the controller drives the knockdown: it braces along the set's fall
/// curves (at least their 0.8 s), settles at rest, and hands the character
/// back through the get-up.
#[test]
fn knockdown_gets_up_through_the_controller() {
    let mut s = scene(|base| RagdollMode::Knockdown {
        point: base.offset(Vec3::new(0.0, 1.6, 0.0)),
        impulse: Vec3::new(-60.0, 0.0, 0.0),
        get_up: None,
        get_up_flag: "down".into(),
    });
    let mut got_up_at = None;
    for frame in 0..900 {
        s.app.update();
        if got_up_at.is_none() && s.app.world().resource::<RagdollStats>().got_up > 0 {
            got_up_at = Some(frame);
            break;
        }
    }
    let frame = got_up_at.expect("the knocked-down character gets up");
    // 0.1 s clip delay + 0.8 s brace fade at the least.
    assert!(frame as f32 / 60.0 >= 0.85, "got up at frame {frame}");
    assert!(s.app.world().get::<Ragdoll>(s.body).is_none());
}

/// AN-14: killed while knocked down, the character stays dead: the host's
/// death request replaces the live knockdown, and no get-up follows.
#[test]
fn a_death_during_a_knockdown_never_gets_up() {
    let mut s = scene(|base| RagdollMode::Knockdown {
        point: base.offset(Vec3::new(0.0, 1.6, 0.0)),
        impulse: Vec3::new(-60.0, 0.0, 0.0),
        get_up: None,
        get_up_flag: "down".into(),
    });
    for _ in 0..30 {
        s.app.update();
    }
    assert_eq!(s.app.world().resource::<RagdollStats>().spawned, 1, "the knockdown is live");
    s.app.world_mut().entity_mut(s.body).insert(Ragdoll { skeleton: "biped".into(), delay: 0.1, mode: RagdollMode::Death { crumple: true } });
    for _ in 0..900 {
        s.app.update();
    }
    let stats = *s.app.world().resource::<RagdollStats>();
    assert_eq!((stats.got_up, stats.spawned, stats.live), (0, 1, 1), "{stats:?}");
    assert!(s.app.world().get::<Ragdoll>(s.body).is_some());
}

/// The pelvis bone's world position (relative to `base`) from the bone entities.
fn pelvis_world(s: &Scene, base: WorldPos) -> Option<Vec3> {
    let world = s.app.world();
    let (cell, root_t) = (world.get::<CellCoord>(s.root)?, world.get::<Transform>(s.root)?);
    let composed = root_t.mul_transform(*world.get::<Transform>(s.body)?);
    let origin = base.delta_f32(from_grid(cell, &composed));
    let frame = Affine3A::from_scale_rotation_translation(composed.scale, composed.rotation, origin);
    let animator = world.get::<Animator>(s.body)?;
    let skeleton = &animator.asset.skeleton;
    let binding = world.get::<SkeletonBinding>(s.body)?;
    let pelvis = usize::from(s.def.bodies[0].joint);
    let mut model = vec![Affine3A::IDENTITY; skeleton.len()];
    for j in 0..=pelvis {
        let local = world.get::<Transform>(binding.joints[j])?.compute_affine();
        model[j] = skeleton.parent(j).map_or(frame, |p| model[p]) * local;
    }
    Some(Vec3::from(model[pelvis].translation))
}

/// AN-4: a request whose ragdoll can't spawn (no ragdoll for the skeleton)
/// ends once, with `RagdollFailed` and the component removed; it retried
/// every `delay` forever and the host never learned of it.
#[test]
fn a_failed_handoff_ends_the_request_once() {
    let mut s = scene(|_| RagdollMode::Death { crumple: false });
    s.app.insert_resource(RagdollSource(Arc::new(|_| None)));
    let mut failed = 0;
    for _ in 0..60 {
        s.app.update();
        failed += s.app.world().resource::<Messages<super::RagdollFailed>>().iter_current_update_messages().count();
    }
    assert_eq!(s.app.world().resource::<RagdollStats>().failed, 1);
    assert_eq!(failed, 1);
    assert!(s.app.world().get::<Ragdoll>(s.body).is_none());
}

/// AN-5/AN-6: at the get-up the frame lands on the ground under the body
/// (where the capsule goes and the clip stands up) and the pelvis doesn't
/// jump across the hand-back.
#[test]
fn get_up_lands_on_the_ground_without_a_pelvis_jump() {
    let base = WorldPos::new(IVec3::new(3, 0, -2), Vec3::new(10.0, 5.0, 20.0));
    let mut s = scene(|base| RagdollMode::Knockdown { point: base.offset(Vec3::new(0.0, 1.6, 0.0)), impulse: Vec3::new(-60.0, 0.0, 0.0), get_up: None, get_up_flag: "down".into() });
    let mut before = None;
    // The fixture walk drives its frame by root motion: off, so only the hand-back moves the pelvis.
    s.app.world_mut().get_mut::<AnimRootMotion>(s.body).unwrap().apply = false;
    for _ in 0..900 {
        let last = pelvis_world(&s, base);
        s.app.update();
        if s.app.world().resource::<RagdollStats>().got_up > 0 {
            before = last;
            break;
        }
    }
    let before = before.expect("gets up");
    let after = pelvis_world(&s, base).unwrap();
    assert!(before.distance(after) < 0.03, "pelvis jumped {:.3} m at the hand-back", before.distance(after));
    // The slope (20° about Z through `base`): the frame sits on it.
    let world = s.app.world();
    let composed = world.get::<Transform>(s.root).unwrap().mul_transform(*world.get::<Transform>(s.body).unwrap());
    let frame = base.delta_f32(from_grid(world.get::<CellCoord>(s.root).unwrap(), &composed));
    let n = Quat::from_rotation_z(20f32.to_radians()) * Vec3::Y;
    let surface = -(n.x * frame.x + n.z * frame.z) / n.y;
    assert!((frame.y - surface).abs() < 0.02, "frame {:.3} m above the ground", frame.y - surface);
}
