//! A synthetic biped walking through the plugin in a headless `App`.

use std::sync::Arc;
use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use sf_anim::fixtures::{biped, gait};
use sf_anim::{
    ClipRules, CookSettings, Event, Looping, RootMotionMode, RootMotionRules, cook_clip,
};
use sf_anim_graph::{CompileSource, compile_str};
use sf_bake::ProductKey;

use crate::{
    AnimAsset, AnimEvent, AnimJoint, AnimLod, AnimNotify, AnimRootMotion, AnimStats, AnimTier,
    AnimateWhilePaused, Animator, PhysicsPose, SfAnimPlugin, spawn_rig,
};

struct Walk(sf_anim::Clip);

impl CompileSource for Walk {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, sf_anim::Clip), String> {
        Ok((ProductKey::new("anim.cook", reference), self.0.clone()))
    }
    fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
        Err("none".into())
    }
}

fn walk_asset() -> Arc<AnimAsset> {
    let skeleton = biped();
    let mut src = gait(
        &skeleton,
        "walk",
        1.0,
        1.4,
        0.5,
        0.6,
        &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)],
    );
    src.events.push(Event {
        payload: "left".into(),
        ..Event::point(0.25, "step")
    });
    let rules = ClipRules {
        looping: Looping::Whole,
        root_motion: RootMotionRules {
            mode: RootMotionMode::Drive,
            ..RootMotionRules::default()
        },
        ..ClipRules::default()
    };
    let clip = cook_clip(&skeleton, &src, &rules, &CookSettings::default()).unwrap();
    let ron = r#"(nodes: {"walk": Clip(clip: "walk")}, layers: [(node: "walk")])"#;
    let graph = compile_str("walk", ron, &skeleton, &mut Walk(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    Arc::new(AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap())
}

#[derive(Resource, Default)]
struct Seen {
    messages: usize,
    /// "step" messages carrying their clip event's payload.
    left_steps: usize,
    observed: usize,
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, SfAnimPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1.0 / 60.0,
        )))
        .init_resource::<Seen>()
        .add_systems(
            PostUpdate,
            (|mut r: MessageReader<AnimEvent>, mut seen: ResMut<Seen>| {
                for e in r.read() {
                    seen.messages += 1;
                    seen.left_steps += usize::from(&*e.name == "step" && e.payload.as_deref() == Some("left"));
                }
            })
                .after(crate::AnimSystems::Apply),
        );
    app
}

fn spawn(app: &mut App, asset: &Arc<AnimAsset>, tier: AnimTier) -> Entity {
    let world = app.world_mut();
    let character = world
        .spawn((
            Transform::IDENTITY,
            Animator::new(asset.clone()),
            AnimLod::forced(tier),
            AnimRootMotion {
                apply: true,
                ..default()
            },
        ))
        .observe(|_: On<AnimNotify>, mut seen: ResMut<Seen>| seen.observed += 1)
        .id();
    let mut commands = world.commands();
    spawn_rig(&mut commands, character, &asset.skeleton);
    world.flush();
    character
}

#[test]
fn walk_writes_bones_root_motion_and_events() {
    let asset = walk_asset();
    let mut app = app();
    let walker = spawn(&mut app, &asset, AnimTier::A0);
    let frozen = spawn(&mut app, &asset, AnimTier::A3);
    let thigh = asset.skeleton.find("thigh_l").unwrap();
    let bone = |app: &mut App, who: Entity| {
        let mut q = app.world_mut().query::<(&AnimJoint, &Transform)>();
        q.iter(app.world())
            .find(|(j, _)| j.character == who && usize::from(j.joint) == thigh)
            .map(|(_, t)| *t)
            .unwrap()
    };
    let rest = bone(&mut app, walker);
    let mut moved = 0;
    for _ in 0..90 {
        app.update();
        if bone(&mut app, walker).rotation.angle_between(rest.rotation) > 0.05 {
            moved += 1;
        }
    }
    assert!(
        moved > 30,
        "the walking thigh swings ({moved} of 90 frames)"
    );
    assert_eq!(bone(&mut app, frozen), rest, "A3 never poses");
    let z = |app: &App, e: Entity| app.world().get::<Transform>(e).unwrap().translation.z;
    let (zw, zf) = (z(&app, walker), z(&app, frozen));
    assert!(
        (zw - 1.4 * 1.5).abs() < 0.15,
        "1.5 s at 1.4 m/s of Drive root motion: {zw}"
    );
    assert!(
        (zf - zw).abs() < 1e-3,
        "A3 still advances root motion: {zf} vs {zw}"
    );
    let seen = app.world().resource::<Seen>();
    // One "step" per cycle per character, plus the gait's own events.
    assert!(seen.messages >= 2 * 2, "{}", seen.messages);
    assert!(seen.left_steps >= 2, "payloads ride along: {}", seen.left_steps);
    assert_eq!(
        seen.messages, seen.observed,
        "every message is also observed"
    );
    let stats = app.world().resource::<AnimStats>();
    assert_eq!(stats.per_tier, [1, 0, 0, 1]);
    assert_eq!(stats.evaluated, 2);
}

#[test]
fn a2_updates_every_other_frame() {
    let asset = walk_asset();
    let mut app = app();
    spawn(&mut app, &asset, AnimTier::A2);
    let mut evaluated = 0;
    for _ in 0..20 {
        app.update();
        evaluated += app.world().resource::<AnimStats>().evaluated;
    }
    assert_eq!(evaluated, 10);
}

#[derive(Resource, Default)]
struct Runs {
    probe: usize,
    evaluate: usize,
}

fn bones(app: &mut App, who: Entity) -> Vec<(u16, Transform)> {
    let mut q = app.world_mut().query::<(&AnimJoint, &Transform)>();
    let mut v: Vec<_> = q
        .iter(app.world())
        .filter(|(j, _)| j.character == who)
        .map(|(j, t)| (j.joint, *t))
        .collect();
    v.sort_by_key(|(j, _)| *j);
    v
}

/// ROADMAP 1.18 (P-IK2): a paused `Time<Virtual>` stops the Probe, Evaluate and Apply stage;
/// `AnimateWhilePaused` keeps a preview going on real time; resuming doesn't pop.
#[test]
fn pause_holds_the_pose_and_resume_does_not_pop() {
    let asset = walk_asset();
    let mut app = app();
    app.init_resource::<Runs>()
        .add_systems(
            PostUpdate,
            (|mut r: ResMut<Runs>| r.probe += 1).in_set(crate::AnimSystems::Probe),
        )
        .add_systems(
            PostUpdate,
            (|mut r: ResMut<Runs>| r.evaluate += 1).in_set(crate::AnimSystems::Evaluate),
        );
    let walker = spawn(&mut app, &asset, AnimTier::A0);
    let preview = spawn(&mut app, &asset, AnimTier::A0);
    app.world_mut().entity_mut(preview).insert(AnimateWhilePaused);
    let thigh = u16::try_from(asset.skeleton.find("thigh_l").unwrap()).unwrap();
    let thigh_of = |b: &[(u16, Transform)]| b.iter().find(|(j, _)| *j == thigh).unwrap().1;
    for _ in 0..20 {
        app.update();
    }
    let before = bones(&mut app, walker);
    let root_before = *app.world().get::<Transform>(walker).unwrap();
    let runs_before = (app.world().resource::<Runs>().probe, app.world().resource::<Runs>().evaluate);
    assert_eq!(runs_before, (20, 20));

    app.world_mut().resource_mut::<Time<Virtual>>().pause();
    let mut preview_moved = 0;
    let mut last_preview = thigh_of(&bones(&mut app, preview));
    for _ in 0..10 {
        app.update();
        assert_eq!(bones(&mut app, walker), before, "paused bones don't change");
        assert_eq!(*app.world().get::<Transform>(walker).unwrap(), root_before, "no root motion");
        let a = app.world().get::<Animator>(walker).unwrap();
        assert!(!a.updated && !a.posed(), "the paused walker isn't updated");
        let p = thigh_of(&bones(&mut app, preview));
        preview_moved += usize::from(p != last_preview);
        last_preview = p;
        assert_eq!(app.world().resource::<AnimStats>().evaluated, 1, "only the preview");
    }
    assert_eq!(preview_moved, 10, "the opted-in preview animates on real time");
    let runs = app.world().resource::<Runs>();
    assert_eq!(runs.probe, 20, "no Probe feed while paused");
    assert_eq!(runs.evaluate, 30, "Evaluate runs for the preview");

    // Nothing opted in: the Evaluate and Apply sets are skipped outright.
    app.world_mut().entity_mut(preview).remove::<AnimateWhilePaused>();
    let held = bones(&mut app, preview);
    for _ in 0..5 {
        app.update();
        assert_eq!(bones(&mut app, preview), held);
        assert_eq!(bones(&mut app, walker), before);
    }
    let runs = app.world().resource::<Runs>();
    assert_eq!((runs.probe, runs.evaluate), (20, 30), "Probe and Evaluate skipped");
    let stats = app.world().resource::<AnimStats>();
    assert_eq!((stats.evaluated, stats.bones_written, stats.events), (0, 0, 0));
    let a = app.world().get::<Animator>(preview).unwrap();
    assert!(!a.updated && !a.posed(), "flags cleared when the sets skip");

    app.world_mut().resource_mut::<Time<Virtual>>().unpause();
    app.update();
    let runs = app.world().resource::<Runs>();
    assert_eq!((runs.probe, runs.evaluate), (21, 31), "resumed");
    assert!(app.world().get::<Animator>(walker).unwrap().posed());
    let after = bones(&mut app, walker);
    // One 1/60 s step of a 1 s walk cycle from the held pose, not a jump (sanity bound).
    let turn = thigh_of(&after).rotation.angle_between(thigh_of(&before).rotation);
    assert!(turn > 0.0 && turn < 0.15, "first resumed frame continues the pose: {turn} rad");
    let step = app.world().get::<Transform>(walker).unwrap().translation - root_before.translation;
    assert!(step.length() < 0.05, "one frame of root motion, no catch-up: {step}");
}

/// Bones whose `Transform` changed this frame, as `(character, joint)` (change ticks since this
/// system's last run, i.e. the previous frame).
#[derive(Resource, Default)]
struct ChangedBones(Vec<(Entity, u16)>);

fn record_changed(bones: Query<(&AnimJoint, Ref<Transform>)>, mut out: ResMut<ChangedBones>) {
    out.0 = bones
        .iter()
        .filter(|(_, t)| t.is_changed())
        .map(|(j, _)| (j.character, j.joint))
        .collect();
}

/// Slice P6: `write_bones` visits characters, not bones. A posed character's bones hold exactly
/// the evaluated local pose (the values the per-bone pass wrote); a frozen character's bones are
/// never touched, so change detection (and transform propagation) stays quiet for them; a
/// physics pose still drives its fully physical joints on a frame the graph doesn't pose.
#[test]
fn write_bones_writes_posed_characters_only() {
    let asset = walk_asset();
    let mut app = app();
    app.init_resource::<ChangedBones>().add_systems(
        PostUpdate,
        record_changed
            .after(crate::AnimSystems::Apply)
            .before(TransformSystems::Propagate),
    );
    let walker = spawn(&mut app, &asset, AnimTier::A0);
    let frozen = spawn(&mut app, &asset, AnimTier::A3);
    let ragdoll = spawn(&mut app, &asset, AnimTier::A3);
    let thigh = asset.skeleton.find("thigh_l").unwrap();
    let mut pose = asset.skeleton.rest().to_vec();
    pose[thigh].rotation = Quat::from_rotation_x(0.5) * pose[thigh].rotation;
    app.world_mut().entity_mut(ragdoll).insert(PhysicsPose {
        pose: pose.clone(),
        weight: 1.0,
        weights: Vec::new(),
    });
    let joints = asset.skeleton.len();
    for frame in 0..10 {
        app.update();
        let a = app.world().get::<Animator>(walker).unwrap();
        assert!(a.posed());
        let expected: Vec<(u16, Transform)> = (0..joints)
            .map(|j| (u16::try_from(j).unwrap(), crate::trs_transform(a.output.pose.get(j))))
            .collect();
        assert_eq!(bones(&mut app, walker), expected, "frame {frame}: the evaluated pose");
        if frame == 0 {
            continue; // Spawned bones read as changed on the recorder's first run.
        }
        let changed = &app.world().resource::<ChangedBones>().0;
        assert!(
            changed.iter().any(|(c, _)| *c == walker),
            "frame {frame}: the walking character's bones change"
        );
        assert!(
            !changed.iter().any(|(c, _)| *c == frozen),
            "frame {frame}: no frozen bone is marked changed"
        );
        assert!(!app.world().get::<Animator>(frozen).unwrap().posed());
    }
    let rest = asset.skeleton.rest();
    let frozen_bones = bones(&mut app, frozen);
    assert!(
        frozen_bones.iter().all(|(j, t)| *t == crate::trs_transform(rest[usize::from(*j)])),
        "the frozen character keeps its rest pose"
    );
    let held = bones(&mut app, ragdoll)[thigh].1;
    let want = crate::trs_transform(pose[thigh]);
    assert!(
        held.rotation.angle_between(want.rotation) < 1e-4
            && held.translation.distance(want.translation) < 1e-4,
        "a fully physical joint follows the physics pose: {held:?} vs {want:?}"
    );
}
