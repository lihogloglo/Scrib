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
use sf_controller::CharacterFrame;
use sf_controller_bevy::ControllerSettings;

use super::*;
use crate::avatar::{AirDebounce, AvatarSlots, PlayerFeed};
use crate::tests::{Statics, action, app, mock};

/// Every clip reference cooks the same short walk cycle.
struct Clips(Clip);

impl CompileSource for Clips {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
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
        clips: {"walk": (source: "walk"), "traverse.mantle.waist": (source: "climb"),
                "attack.sword_1h.light1": (source: "a1"), "attack.sword_1h.light1_rec": (source: "a1r")},
        nodes: {"walk": Clip(clip: "walk"), "full_slot": Slot(slot: "full", input: "walk", group: None, mask: None)},
        layers: [(node: "full_slot")],
        slot_clips: ["traverse.mantle.waist", "attack.sword_1h.light1", "attack.sword_1h.light1_rec"],
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
        .add_systems(PostUpdate, avatar::feed::<Statics>);
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
        character.output.warp = Some(WarpOutput { root: RootPose::IDENTITY, progress, settle: false });
        let ticks = Some(((progress - 1.0 / 60.0).max(0.0), progress));
        avatar::traversal(&mut feed, &mut animator, &character, &AvatarSlots::default(), ticks, 1.0);
        animator.update(1.0 / 60.0, AnimTier::A0);
        let time = animator.instance.montage(slot).expect("the mantle plays").time;
        assert!((time - progress).abs() < 1e-3, "playhead {time} follows progress {progress}");
    }
    assert!(feed.traversal().is_some());
    character.output.warp = None;
    avatar::traversal(&mut feed, &mut animator, &character, &AvatarSlots::default(), None, 1.0);
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
    app.add_systems(PostUpdate, avatar::feed::<Statics>);
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

/// A Clip move's chain (attack, then recovery): each slot plays from the chain progress minus
/// the clips before it, and the montage hands back when the move ends.
#[test]
fn a_clip_chain_plays_each_slot_from_its_own_start() {
    use sf_action::ClipOutput;
    let mut animator = Animator::new(asset());
    let slot = animator.asset.graph.slot("full").expect("full slot");
    let slots = AvatarSlots::new(
        ["attack.sword_1h.light1".to_owned(), "attack.sword_1h.light1_rec".to_owned()],
        [("attack.sword_1h.light1".to_owned(), 0.1)],
    );
    let mut feed = PlayerFeed::default();
    let mut character = ActionCharacter::new(PLAYER_CHARACTER, ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    character.output.rule = RuleKey::of("sword_1h.light1");
    for tick in 0..12u32 {
        #[allow(clippy::cast_precision_loss, reason = "small")]
        let progress = tick as f32 / 60.0;
        let (key, chain, local) =
            if progress < 0.1 { ("attack.sword_1h.light1", 0, progress) } else { ("attack.sword_1h.light1_rec", 1, progress - 0.1) };
        character.output.tick = Tick(tick);
        character.output.slot = Some(SlotKey::of(key));
        character.output.chain = chain;
        character.output.clip = Some(ClipOutput { delta: Vec3::ZERO, yaw: 0.0, progress });
        avatar::traversal(&mut feed, &mut animator, &character, &slots, Some((progress, progress)), 1.0);
        animator.update(1.0 / 60.0, AnimTier::A0);
        let time = animator.instance.montage(slot).expect("the attack plays").time;
        assert!((time - local).abs() < 1e-3, "{key}: playhead {time} vs {local}");
        assert_eq!(feed.traversal(), Some(SlotKey::of(key)));
    }
    character.output.clip = None;
    avatar::traversal(&mut feed, &mut animator, &character, &slots, None, 1.0);
    animator.update(1.0 / 60.0, AnimTier::A0);
    assert!(feed.traversal().is_none());
}

/// Regression (AL-11, owner 2026-10-30): the wall brace never ran in game because its gate
/// required `ActionOutput::slot` to be `None`, and the base locomotion rule always carries its
/// `loco.move` slot. Walking on the real player rules keeps the brace live; a Clip move or a
/// warp turns it off.
#[test]
fn the_wall_brace_runs_in_plain_locomotion_and_not_under_a_clip_move() {
    use sf_action::ClipOutput;
    let (mut app, player) = app(60);
    let stick = action(&mut app, PlayerAction::Stick);
    mock(&mut app, stick, Vec2::Y);
    for _ in 0..60 {
        app.update();
    }
    let body = app.world().get::<ActionBody>(player).unwrap().clone();
    let mut character = app.world().get::<ActionCharacter>(player).unwrap().clone();
    assert!(body.loco.grounded.is_some(), "walking on the flat start");
    assert!(character.output.slot.is_some(), "the locomotion rule has a slot (the old gate's mistake)");
    assert!(grip::brace_active(&body, &character), "plain locomotion: the brace may run");
    character.output.clip = Some(ClipOutput { delta: Vec3::ZERO, yaw: 0.0, progress: 0.1 });
    assert!(!grip::brace_active(&body, &character), "a Clip move owns the arms");
    character.output.clip = None;
    character.output.warp = Some(WarpOutput { root: RootPose::IDENTITY, progress: 0.1, settle: false });
    assert!(!grip::brace_active(&body, &character), "a warp (mantle) owns the arms");
}

/// FP-1 (superfargoth first-person.md): the camera's view lays the avatar out by role through the
/// real systems (camera rig, `first_person`, the character layer sync), headless: third person
/// draws every part on the avatar's layers, first person moves the head to the shadow-only layer and keeps body,
/// arms and held items, and switching back restores them; with the view model on (FP-2) arms and
/// held items go shadow only and their view copies take the avatar's layers. No rendering is judged.
#[test]
fn switching_first_and_third_person_relayers_the_avatar_by_role() {
    use bevy_camera::visibility::RenderLayers;
    use sf_camera::ViewChoice;
    use sf_camera_bevy::{PlayerCamera, SfCameraPlugin};
    use sf_character_bevy::{PartRole, RoleLayers, SfCharacterPlugin, ShadowLayers, ViewCopy, ViewRole};
    let keymap = mw_controls_bevy::keymap();
    let (mut app, player) = tests::app_setup(60, &keymap, |app| {
        app.add_plugins((AssetPlugin::default(), SfCameraPlugin::<Statics>::default(), SfCharacterPlugin))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_asset::<bevy_mesh::skinning::SkinnedMeshInverseBindposes>();
        avatar::add_first_person(app);
    });
    // The worldspace's content and shadow-only layers (`WorldspaceLayers`, as the worldspace push writes them).
    let (ws, shadow) = (RenderLayers::layer(1), RenderLayers::layer(2));
    let avatar = app.world_mut().spawn((avatar::avatar_bundle(), ws.clone(), ShadowLayers(shadow.clone()), ChildOf(player))).id();
    let parts = ViewRole::ALL.map(|role| app.world_mut().spawn((PartRole(role), Mesh3d::default(), ChildOf(avatar))).id());
    let cam = app.world_mut().spawn((PlayerCamera::new(player), Transform::default())).id();
    let layers = |app: &App| parts.map(|p| app.world().get::<RenderLayers>(p).cloned().unwrap_or_default());
    let shown = |app: &App| *app.world().get::<Visibility>(avatar).unwrap();
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(app.world().get::<PlayerCamera>(cam).unwrap().pose().map(|p| p.view), Some(ViewChoice::ThirdPerson));
    assert_eq!(layers(&app), [ws.clone(), ws.clone(), ws.clone(), ws.clone()], "third person: body, head, arms, held");
    assert_eq!(shown(&app), Visibility::Inherited);

    app.world_mut().get_mut::<PlayerCamera>(cam).unwrap().set_view(ViewChoice::FirstPerson);
    app.update();
    assert_eq!(app.world().get::<RoleLayers>(avatar), Some(&RoleLayers::first_person(false)));
    assert_eq!(layers(&app), [ws.clone(), shadow.clone(), ws.clone(), ws.clone()], "first person: the head shadow-only, the rest drawn");
    assert_eq!(shown(&app), Visibility::Inherited, "the avatar is no longer hidden whole in first person");
    let copies = |app: &mut App| {
        let mut q = app.world_mut().query::<(&ViewCopy, &RenderLayers)>();
        let mut v: Vec<_> = q.iter(app.world()).map(|(c, l)| (c.source, l.clone())).collect();
        v.sort_by_key(|(e, _)| *e);
        v
    };
    assert!(copies(&mut app).is_empty(), "no view copies while the view model is off (the default)");

    app.world_mut().get_mut::<PlayerCamera>(cam).unwrap().settings.view_model = true;
    app.update();
    assert_eq!(app.world().get::<RoleLayers>(avatar), Some(&RoleLayers::first_person(true)));
    assert_eq!(layers(&app), [ws.clone(), shadow.clone(), shadow.clone(), shadow], "view model: head, arms and held shadow-only");
    let mut want = vec![(parts[2], ws.clone()), (parts[3], ws.clone())];
    want.sort_by_key(|(e, _)| *e);
    assert_eq!(copies(&mut app), want, "arms and held drawn as view copies on the avatar's layers");

    app.world_mut().get_mut::<PlayerCamera>(cam).unwrap().set_view(ViewChoice::ThirdPerson);
    app.update();
    assert_eq!(layers(&app), [ws.clone(), ws.clone(), ws.clone(), ws]);
    assert!(copies(&mut app).is_empty(), "third person: copies gone");
}

/// FP-2b: with the view model on, the arm part's view copy draws the view-model material derived
/// from the part's material, carrying the camera's projection (`fov_world` to `fov_body`, the
/// depth slice), and a settings change on the camera reaches it. Headless; no rendering judged.
#[test]
fn view_copy_material_follows_the_camera_settings() {
    use sf_camera::{ViewChoice, ViewModelProjection};
    use sf_camera_bevy::{PlayerCamera, SfCameraPlugin};
    use sf_character_bevy::{PartRole, SfCharacterPlugin, ViewCopy, ViewModel, ViewModelMaterial, ViewModelPlugin, ViewModelSettings, ViewRole};
    let keymap = mw_controls_bevy::keymap();
    let (mut app, player) = tests::app_setup(60, &keymap, |app| {
        app.add_plugins((AssetPlugin::default(), SfCameraPlugin::<Statics>::default(), SfCharacterPlugin))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_asset::<bevy_mesh::skinning::SkinnedMeshInverseBindposes>()
            .add_plugins(ViewModelPlugin);
        avatar::add_first_person(app);
    });
    let avatar = app.world_mut().spawn((avatar::avatar_bundle(), ChildOf(player))).id();
    let skin = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
    let arm = app.world_mut().spawn((PartRole(ViewRole::Arms), Mesh3d::default(), MeshMaterial3d(skin.clone()), ChildOf(avatar))).id();
    let cam = app.world_mut().spawn((PlayerCamera::new(player), Transform::default())).id();
    {
        let mut c = app.world_mut().get_mut::<PlayerCamera>(cam).unwrap();
        c.set_view(ViewChoice::FirstPerson);
        c.settings.view_model = true;
    }
    for _ in 0..3 {
        app.update();
    }
    let drawn = |app: &mut App| -> ViewModel {
        let mut q = app.world_mut().query::<(&ViewCopy, &MeshMaterial3d<ViewModelMaterial>)>();
        let (copy, material) = q.single(app.world()).expect("one view copy, with the view-model material");
        assert_eq!(copy.source, arm);
        let m = app.world().resource::<Assets<ViewModelMaterial>>().get(&material.0).expect("derived material");
        m.extension
    };
    let want = |settings: &sf_camera::ComfortSettings| {
        let p: ViewModelProjection = settings.view_model_projection();
        ViewModel { fov_scale: p.k, depth: p.depth }
    };
    let settings = app.world().get::<PlayerCamera>(cam).unwrap().settings;
    assert!(want(&settings).fov_scale > 1.0, "fov_body narrower than fov_world by default");
    assert_eq!(drawn(&mut app), want(&settings));
    let resource = *app.world().resource::<ViewModelSettings>();
    assert_eq!(ViewModel::from(resource), want(&settings));

    // The camera's settings change (a wider body FOV, a thinner slice): the material follows.
    {
        let mut c = app.world_mut().get_mut::<PlayerCamera>(cam).unwrap();
        c.settings.fov_body = 40.0;
        c.settings.view_model_depth = 0.5;
    }
    app.update();
    let settings = app.world().get::<PlayerCamera>(cam).unwrap().settings;
    assert_eq!(drawn(&mut app), want(&settings));
    assert!((drawn(&mut app).depth - 0.5).abs() < f32::EPSILON);
}

/// The view rule itself: first person by role, third person hides the whole avatar only past the
/// collision fade threshold.
#[test]
fn avatar_view_per_camera_view_and_fade() {
    use avatar::avatar_view;
    use sf_camera::ViewChoice;
    use sf_character_bevy::RoleLayers;
    for vm in [false, true] {
        assert_eq!(avatar_view(None, vm), (Visibility::Inherited, RoleLayers::world()));
        assert_eq!(avatar_view(Some((ViewChoice::FirstPerson, 1.0)), vm), (Visibility::Inherited, RoleLayers::first_person(vm)));
        assert_eq!(avatar_view(Some((ViewChoice::ThirdPerson, 0.2)), vm), (Visibility::Inherited, RoleLayers::world()));
        assert_eq!(avatar_view(Some((ViewChoice::ThirdPerson, 0.9)), vm), (Visibility::Hidden, RoleLayers::world()));
    }
}

/// [`asset`]'s walk with two legs and a `FootPlacement` op (pelvis drop, foot lock).
fn ik_asset() -> Arc<AnimAsset> {
    let skeleton = biped();
    let src = gait(&skeleton, "walk", 1.0, 1.4, 0.5, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
    let rules = ClipRules { looping: Looping::Whole, ..ClipRules::default() };
    let clip = cook_clip(&skeleton, &src, &rules, &CookSettings::default()).unwrap();
    let ron = r#"(
        clips: {"walk": (source: "walk")},
        nodes: {"walk": Clip(clip: "walk")},
        layers: [(node: "walk")],
        legs: [
            (name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l")),
            (name: "r", hip: Joint("thigh_r"), knee: Joint("shin_r"), foot: Joint("foot_r")),
        ],
        procedural: [FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"])],
    )"#;
    let graph = compile_str("avatar-ik", ron, &skeleton, &mut Clips(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    Arc::new(AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap())
}

/// One frame of [`probe_walk`]: the drawn root, the graph's ground samples and output.
struct ProbeFrame {
    root: Vec3,
    /// The samples in the world (through the drawn frame), `None` where missing.
    samples: Vec<Option<(Vec3, Vec3)>>,
    weight: f32,
    grade: f32,
    /// World height of each foot joint minus the ramp/floor height under it.
    foot_above: [f32; 2],
    finite: bool,
}

/// `level-flat-slopes`' 20° ramp: rises toward -z from z = 0.493 (2 m over 5.495 m), 4 m wide at x = -12.
fn ramp_20_height(p: Vec3) -> f32 {
    let (start, run) = (-2.253 + 5.495 / 2.0, 5.495);
    if (p.x + 12.0).abs() > 2.0 {
        return 0.0;
    }
    ((start - p.z) / run).clamp(0.0, 1.0) * 2.0
}

/// The game's player feed (`avatar::feed` over the level's statics) on a graph with foot
/// placement, walking north from `at` with `settings`, the graph updated each frame after the
/// feed (as `AnimSystems::Evaluate`).
fn probe_walk(at: Vec3, settings: ControllerSettings, secs: f32) -> Vec<ProbeFrame> {
    let (mut app, player) = app(60);
    app.insert_resource(settings).add_systems(PostUpdate, avatar::feed::<Statics>);
    let animator = app.world_mut().spawn((Animator::new(ik_asset()), Transform::default())).id();
    let mut feed = PlayerFeed::default();
    feed.animator = Some(animator);
    app.world_mut().entity_mut(player).insert((PlayerBody, feed));
    app.world_mut().get_mut::<ActionBody>(player).unwrap().teleport(WorldPos::from_f64(at.as_dvec3()), std::f32::consts::PI);
    let stick = action(&mut app, PlayerAction::Stick);
    mock(&mut app, stick, Vec2::NEG_Y);
    let skeleton = ik_asset().skeleton.clone();
    let foot_joints = [skeleton.find("foot_l").unwrap(), skeleton.find("foot_r").unwrap()];
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a few hundred frames")]
    let n = (secs * 60.0).round() as usize;
    let mut frames = Vec::with_capacity(n);
    for _ in 0..n {
        app.update();
        let (body, t) = (app.world().get::<ActionBody>(player).unwrap(), app.world().get::<Transform>(player).unwrap());
        let drawn = CharacterFrame::new(body.frame_origin.offset(t.translation), Mover::yaw_of(t.rotation));
        let mut a = app.world_mut().get_mut::<Animator>(animator).unwrap();
        a.update(1.0 / 60.0, AnimTier::A0);
        a.drive.moved = sf_anim::RootDelta::ZERO;
        let world = |p: Vec3| WorldPos::ORIGIN.delta_f32(drawn.to_world(p));
        let samples = a.drive.ground.iter().map(|s| s.map(|s| (world(s.point), drawn.rotation() * s.normal))).collect();
        let foot_above = foot_joints.map(|j| {
            let p = world(a.output.model.translation(j));
            p.y - ramp_20_height(p)
        });
        let finite = a.output.model.mats.iter().all(bevy_math::Affine3A::is_finite);
        frames.push(ProbeFrame {
            root: world(Vec3::ZERO),
            samples,
            weight: a.output.feet.weight_max,
            grade: a.output.grade,
            foot_above,
            finite,
        });
    }
    frames
}

/// P-IK1 (ik-feel-diagnosis 2026-11-11, cause 1): the player's graph gets ground samples from
/// the level's statics under its legs, in the drawn frame. Exact on the data path: every sample
/// arrives and lies on the ramp's surface with its normal; foot placement turns on and the
/// grade goes positive uphill. Sanity on the pose: finite, feet not through the ramp.
/// `K` (`foot_probes` off) drops them all.
#[test]
fn the_player_probes_the_ground_under_its_feet() {
    let frames = probe_walk(Vec3::new(-12.0, 0.0, 2.5), ControllerSettings::default(), 3.0);
    // After the first frames, every leg is probed (short of the plateau's far edge, z = -9, where the
    // leading foot's probe misses the 2 m drop).
    let missing: Vec<(usize, Vec3, usize)> = frames
        .iter()
        .enumerate()
        .filter(|(_, f)| f.root.z > -8.0 && (f.samples.len() != 2 || f.samples.iter().any(Option::is_none)))
        .map(|(i, f)| (i, f.root, f.samples.iter().flatten().count()))
        .collect();
    assert!(missing.iter().all(|m| m.0 < 2), "every leg sampled after the first frames: missing at {missing:?}");
    let ramp: Vec<&ProbeFrame> = frames.iter().filter(|f| f.root.y > 0.3 && f.root.y < 1.7).collect();
    assert!(ramp.len() > 30, "walked up the 20° ramp: {} frames on it", ramp.len());
    let (tilt, cos20) = (20f32.to_radians().sin(), 20f32.to_radians().cos());
    for f in &ramp {
        for (p, n) in f.samples.iter().flatten() {
            if p.z < 0.3 && p.z > -4.8 {
                assert!((p.y - ramp_20_height(*p)).abs() < 1e-3, "sample on the ramp: {p}");
                assert!((n.y - cos20).abs() < 1e-3 && (n.z - tilt).abs() < 1e-3, "ramp normal (faces +z, down the slope): {n}");
            }
        }
    }
    let weight = ramp.iter().map(|f| f.weight).fold(0.0f32, f32::max);
    #[allow(clippy::cast_precision_loss, reason = "a few hundred frames")]
    let grade = ramp.iter().map(|f| f.grade).sum::<f32>() / ramp.len() as f32;
    let sink = ramp.iter().flat_map(|f| f.foot_above).fold(f32::INFINITY, f32::min);
    println!("20° ramp: {} frames, foot placement weight max {weight:.2}, mean grade {grade:.2}, lowest foot joint {sink:+.3} m", ramp.len());
    assert!(weight > 0.0, "foot placement on: {weight}");
    assert!(grade > 0.0, "uphill grade: {grade}");
    assert!(frames.iter().all(|f| f.finite), "a finite pose");
    assert!(sink > -0.05, "no foot joint through the ramp: {sink}");

    let off = probe_walk(Vec3::new(-12.0, 0.0, 2.5), ControllerSettings { foot_probes: false, ..ControllerSettings::default() }, 1.0);
    assert!(off.iter().all(|f| f.samples.iter().all(Option::is_none)), "K off: no samples");
    assert!(off.iter().all(|f| f.weight.abs() < f32::EPSILON), "K off: foot placement holds the animated pose");
}
