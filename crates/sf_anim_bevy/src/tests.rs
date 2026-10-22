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
    Animator, SfAnimPlugin, spawn_rig,
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
