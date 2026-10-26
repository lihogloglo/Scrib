//! The player avatar (action.md A2 slice 2): spawn, locomotion feed, traversal slot. Sanity only.

use std::sync::Arc;
use std::time::Duration;

use bevy_time::TimeUpdateStrategy;
use sf_action::{SlotKey, Tick, WarpOutput};
use sf_anim::fixtures::{biped, gait};
use sf_anim::{ClipRules, CookSettings, Looping, RootPose, cook_clip};
use sf_anim_bevy::{AnimAsset, AnimTier, Animator};
use sf_anim_graph::{CompileSource, compile_str};
use sf_bake::ProductKey;

use super::*;
use crate::avatar::PlayerFeed;
use crate::tests::{action, app, mock};

/// Every clip reference cooks the same short walk cycle.
struct Clips(sf_anim::Clip);

impl CompileSource for Clips {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, sf_anim::Clip), String> {
        Ok((ProductKey::new("anim.cook", reference), self.0.clone()))
    }
    fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
        Err("none".into())
    }
}

/// A graph with the humanoid's montage slot and the waist mantle as a slot clip.
fn asset() -> Arc<AnimAsset> {
    let skeleton = biped();
    let src = gait(&skeleton, "walk", 1.0, 1.4, 0.5, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
    let rules = ClipRules { looping: Looping::Whole, ..ClipRules::default() };
    let clip = cook_clip(&skeleton, &src, &rules, &CookSettings::default()).unwrap();
    let ron = r#"(
        params: [(name: "moving", kind: Bool(false)), (name: "on_ground", kind: Bool(true))],
        clips: {"walk": (source: "walk"), "traverse.mantle.waist": (source: "climb")},
        nodes: {"walk": Clip(clip: "walk"), "full_slot": Slot(slot: "full", input: "walk", group: None, mask: None)},
        layers: [(node: "full_slot")],
        slot_clips: ["traverse.mantle.waist"],
    )"#;
    let graph = compile_str("avatar", ron, &skeleton, &mut Clips(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    Arc::new(AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap())
}

#[test]
fn dressed_spawn_puts_the_player_record_under_the_body_and_no_capsule_mesh() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let visual = PlayerVisual { capsule: Handle::default(), nose: Handle::default(), material: Handle::default() };
    let keymap = mw_controls_bevy::keymap();
    let body = {
        let mut commands = world.commands();
        spawn_body(&mut commands, &visual, &keymap, (WorldPos::ORIGIN, 0.0), root, true)
    };
    world.flush();
    let mut q = world.query_filtered::<(&ChildOf, &sf_character_bevy::Character), With<mw_game_core::player::PlayerAvatar>>();
    let avatars: Vec<_> = q.iter(&world).filter(|(p, _)| p.parent() == body).collect();
    assert_eq!(avatars.len(), 1, "one avatar under the body");
    let sf_character_bevy::Character::Record { id, .. } = avatars[0].1 else { panic!("a record character") };
    assert_eq!(id.as_str(), "player");
    let mut meshes = world.query::<(&ChildOf, &Mesh3d)>();
    assert_eq!(meshes.iter(&world).filter(|(p, _)| p.parent() == body).count(), 0, "no capsule mesh");
    assert!(world.get::<PlayerFeed>(body).is_some());
}

#[test]
fn walking_feeds_the_graph_motion() {
    let (mut app, player) = app(60);
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .add_systems(PostUpdate, avatar::feed);
    let animator = app.world_mut().spawn((Animator::new(asset()), Transform::default())).id();
    let mut feed = PlayerFeed::default();
    feed.animator = Some(animator);
    app.world_mut().entity_mut(player).insert((PlayerBody, feed));
    let stick = action(&mut app, PlayerAction::Stick);
    mock(&mut app, stick, Vec2::Y);
    for _ in 0..60 {
        app.update();
    }
    let a = app.world().get::<Animator>(animator).unwrap();
    let v = a.drive.velocity;
    println!("drive velocity after 1 s walking: {v:?}");
    assert!(v.is_finite() && v.z > 0.5, "moving forward in the character frame: {v:?}");
}

#[test]
fn mantle_progress_plays_the_slot_then_hands_back() {
    let mut animator = Animator::new(asset());
    let slot = animator.asset.graph.slot("full").expect("full slot");
    let mut feed = PlayerFeed::default();
    let mut character = ActionCharacter::new(PLAYER_CHARACTER, ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    character.output.slot = Some(SlotKey::of("traverse.mantle.waist"));
    for tick in 0..12u32 {
        #[allow(clippy::cast_precision_loss, reason = "small")]
        let progress = tick as f32 / 60.0;
        character.output.tick = Tick(tick);
        character.output.warp = Some(WarpOutput { root: RootPose::IDENTITY, progress });
        avatar::traversal(&mut feed, &mut animator, &character, 1.0);
        animator.update(1.0 / 60.0, AnimTier::A0);
        let time = animator.instance.montage(slot).expect("the mantle plays").time;
        assert!((time - progress).abs() < 1e-3, "playhead {time} follows progress {progress}");
    }
    assert!(feed.traversal().is_some());
    character.output.warp = None;
    avatar::traversal(&mut feed, &mut animator, &character, 1.0);
    animator.update(1.0 / 60.0, AnimTier::A0);
    assert!(feed.traversal().is_none());
    assert!(animator.instance.montage(slot).is_none(), "back to locomotion");
}
