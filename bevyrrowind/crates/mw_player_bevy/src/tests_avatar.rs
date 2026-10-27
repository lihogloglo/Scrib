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
use crate::avatar::{AirDebounce, PlayerFeed};
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
        params: [(name: "moving", kind: Bool(false)), (name: "on_ground", kind: Bool(true)), (name: "jumping", kind: Bool(false))],
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
        let ticks = Some(((progress - 1.0 / 60.0).max(0.0), progress));
        avatar::traversal(&mut feed, &mut animator, &character, ticks, 1.0);
        animator.update(1.0 / 60.0, AnimTier::A0);
        let time = animator.instance.montage(slot).expect("the mantle plays").time;
        assert!((time - progress).abs() < 1e-3, "playhead {time} follows progress {progress}");
    }
    assert!(feed.traversal().is_some());
    character.output.warp = None;
    avatar::traversal(&mut feed, &mut animator, &character, None, 1.0);
    animator.update(1.0 / 60.0, AnimTier::A0);
    assert!(feed.traversal().is_none());
    assert!(animator.instance.montage(slot).is_none(), "back to locomotion");
}

#[test]
fn air_debounce_ignores_step_misses_and_reports_launches_and_falls() {
    const G: f32 = 9.8;
    for fps in [30.0_f32, 60.0, 144.0] {
        let dt = 1.0 / fps;
        // A step-down miss: a few frames without ground, barely moving down.
        let mut d = AirDebounce::default();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a few frames")]
        let n = ((0.05 * fps).round() as usize).max(1);
        let blip: Vec<_> = (0..n).map(|_| d.on_ground(false, false, -0.3, dt)).collect();
        assert!(blip.iter().all(|&g| g), "{fps} fps: a short miss stays grounded");
        assert!(d.on_ground(true, false, 0.0, dt));
        // Walking off a ledge: free fall, airborne once past the grace or the fall speed.
        let mut d = AirDebounce::default();
        let mut t = 0.0;
        while d.on_ground(false, false, -G * t, dt) {
            t += dt;
            assert!(t < 1.0, "{fps} fps: a fall leaves the ground");
        }
        println!("{fps} fps: unlaunched fall reported after {t:.3} s");
        assert!(!d.on_ground(false, false, -0.1, dt), "{fps} fps: airborne holds until touchdown");
        assert!(d.on_ground(true, false, -3.0, dt), "{fps} fps: touchdown reports grounded at once");
        // A launch leaves the ground on its first frame.
        let mut d = AirDebounce::default();
        assert!(!d.on_ground(false, true, 4.0, dt), "{fps} fps: a launch is airborne at once");
    }
}

/// One frame of the air feed: the mover's ground, what the graph got, the feet height.
#[derive(Clone, Copy, Debug)]
struct AirFrame {
    grounded: bool,
    on_ground: bool,
    jumping: bool,
    feet: Vec3,
}

/// The game's player feeding the test graph, teleported to `at` (else the level's start) and
/// settled for 0.5 s; then `secs` of frames with the stick at `stick`, pressing Jump for the
/// first 0.1 s when `jump`.
fn air_run(at: Option<Vec3>, stick: Vec2, jump: bool, secs: f32) -> Vec<AirFrame> {
    let (mut app, player) = app(60);
    app.add_systems(PostUpdate, avatar::feed);
    let animator = app.world_mut().spawn((Animator::new(asset()), Transform::default())).id();
    let mut feed = PlayerFeed::default();
    feed.animator = Some(animator);
    app.world_mut().entity_mut(player).insert((PlayerBody, feed));
    if let Some(at) = at {
        app.world_mut().get_mut::<ActionBody>(player).unwrap().teleport(WorldPos::from_f64(at.as_dvec3()), 0.0);
    }
    for _ in 0..30 {
        app.update();
    }
    let frame = |app: &App| {
        let body = app.world().get::<ActionBody>(player).unwrap();
        let a = app.world().get::<Animator>(animator).unwrap();
        let get = |name: &str| a.instance.params.get(a.param(name).expect("param"))[0] > 0.5;
        AirFrame {
            grounded: body.loco.grounded.is_some(),
            on_ground: get(mw_graphs::build::params::ON_GROUND),
            jumping: get(mw_graphs::build::params::JUMPING),
            feet: WorldPos::ORIGIN.delta_f32(body.loco.position),
        }
    };
    let stick_e = action(&mut app, PlayerAction::Stick);
    if stick != Vec2::ZERO {
        mock(&mut app, stick_e, stick);
    }
    let jump_e = action(&mut app, PlayerAction::Button(ActionClass::Jump));
    let mut frames = vec![frame(&app)];
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a few hundred frames")]
    let n = (secs * 60.0).round() as usize;
    for i in 0..n {
        if jump && i == 0 {
            mock(&mut app, jump_e, true);
        }
        if jump && i == 6 {
            app.world_mut().entity_mut(jump_e).remove::<ActionMock>();
        }
        app.update();
        frames.push(frame(&app));
    }
    frames
}

/// Down `level-flat-slopes`' steep stairs (0.25 m rises) and off its 2 m plateau, at the
/// default gait. Sanity: the stairs never leave the ground in the graph, the ledge falls (not a
/// take-off) and lands. Ground misses and the fall delay are printed.
#[test]
fn small_steps_stay_grounded_and_a_ledge_falls_without_take_off() {
    let stairs = air_run(Some(Vec3::new(15.0, 2.0, -6.5)), Vec2::Y, false, 2.0);
    let (first, last) = (stairs[0].feet, stairs[stairs.len() - 1].feet);
    let misses = stairs.iter().filter(|f| !f.grounded).count();
    println!(
        "steep stairs: {:.2} m down over {:.2} m in 2 s, {misses} frames without mover ground",
        first.y - last.y,
        last.z - first.z
    );
    assert!(first.y - last.y > 1.5, "went down the stairs: {first} -> {last}");
    assert!(stairs.iter().all(|f| f.on_ground && !f.jumping), "small steps stay grounded in the graph");

    let ledge = air_run(Some(Vec3::new(-9.0, 2.0, -6.5)), Vec2::Y, false, 2.0);
    let left = ledge.iter().position(|f| !f.grounded).expect("leaves the plateau");
    let fell = ledge.iter().position(|f| !f.on_ground).expect("falls in the graph");
    println!("2 m ledge: mover airborne at frame {left}, graph airborne at frame {fell}");
    assert!(ledge.iter().all(|f| !f.jumping), "a fall is not a jump");
    let end = ledge[ledge.len() - 1];
    assert!(end.on_ground && end.feet.y < 0.1, "lands on the floor: {end:?}");
}

/// A jump on flat ground: `jumping` and airborne on the launch frame, both clear at touchdown.
#[test]
fn a_jump_flags_jumping_with_the_launch_and_clears_on_touchdown() {
    let frames = air_run(None, Vec2::ZERO, true, 1.5);
    let launch = frames.iter().position(|f| !f.grounded).expect("the jump leaves the ground");
    let f = frames[launch];
    assert!(f.jumping && !f.on_ground, "launch frame: {f:?}");
    let land = launch + frames[launch..].iter().position(|f| f.grounded).expect("lands");
    println!("jump: launch frame {launch}, touchdown frame {land}");
    assert!(frames[launch..land].iter().all(|f| f.jumping && !f.on_ground), "JUMP, never FALL, in the air");
    assert!(frames[land..].iter().all(|f| f.on_ground && !f.jumping), "grounded after touchdown");
}
