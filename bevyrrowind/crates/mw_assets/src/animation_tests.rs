//! Material controller gate (CW-6): exact counts of material writes and values
//! against the ungated path, headless (no render world; `ViewVisibility` set by hand).

use std::time::Duration;

use bevy::time::TimeUpdateStrategy;
use mw_nif::anim::{Curve, Cycle, Timing, UvTrack};

use super::*;

/// `AssetEvent::Modified` count for `StandardMaterial` (one per `get_mut`).
#[derive(Resource, Default)]
struct Modified(usize);

fn count_modified(mut events: MessageReader<AssetEvent<StandardMaterial>>, mut modified: ResMut<Modified>) {
    modified.0 += events.read().filter(|e| matches!(e, AssetEvent::Modified { .. })).count();
}

/// A headless app stepping the clock by `frame_ms` per update.
fn app(gated: bool, frame_ms: u64) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<StandardMaterial>()
        .init_asset::<NifAnimation>()
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(frame_ms)))
        .init_resource::<Modified>()
        .add_systems(Update, animate_materials)
        .add_systems(Last, count_modified);
    if gated {
        add_material_gate(&mut app);
    }
    app
}

/// A one-second looping U scroll (offset 0 -> 1).
fn scroll() -> Controller {
    Controller {
        timing: Timing { frequency: 1.0, phase: 0.0, start: 0.0, stop: 1.0, cycle: Cycle::Loop },
        auto_play: true,
        random_start: false,
        target: Target::Uv { material: 0, track: UvTrack { offset_u: Curve::linear([(0.0, 0.0), (1.0, 1.0)]), ..default() } },
    }
}

/// A constant alpha of 0.5.
fn constant_alpha() -> Controller {
    Controller {
        timing: Timing { frequency: 1.0, phase: 0.0, start: 0.0, stop: 1.0, cycle: Cycle::Loop },
        auto_play: true,
        random_start: false,
        target: Target::Alpha { material: 0, curve: Curve::linear([(0.0, 0.5), (1.0, 0.5)]) },
    }
}

/// One model: `controller` on a fresh material; returns the material.
fn model(app: &mut App, controller: Controller) -> Handle<StandardMaterial> {
    let material = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
    let animation = NifAnimation { controllers: vec![controller], particles: Vec::new(), materials: vec![material.clone()] };
    let animation = app.world_mut().resource_mut::<Assets<NifAnimation>>().add(animation);
    // The instance root keeps the animation alive, as in the game.
    app.world_mut().spawn(ModelAnimation(animation));
    material
}

/// A mesh entity drawing `material`, as a NIF node spawns it (with the marker).
fn user(app: &mut App, material: &Handle<StandardMaterial>, visible: bool) -> Entity {
    let visibility = if visible { ViewVisibility::VISIBLE } else { ViewVisibility::HIDDEN };
    app.world_mut().spawn((MeshMaterial3d(material.clone()), AnimatedMaterialUser(material.clone()), visibility)).id()
}

fn set_visible(app: &mut App, entity: Entity, visible: bool) {
    *app.world_mut().get_mut::<ViewVisibility>(entity).unwrap() = if visible { ViewVisibility::VISIBLE } else { ViewVisibility::HIDDEN };
}

fn uv(app: &App, material: &Handle<StandardMaterial>) -> Affine2 {
    app.world().resource::<Assets<StandardMaterial>>().get(material).unwrap().uv_transform
}

fn modified(app: &App) -> usize {
    app.world().resource::<Modified>().0
}

#[test]
fn no_visible_user_means_no_material_writes() {
    let mut app = app(true, 40);
    let material = model(&mut app, scroll());
    user(&mut app, &material, false);
    for _ in 0..30 {
        app.update();
    }
    assert_eq!(modified(&app), 0);
    assert_eq!(uv(&app, &material), Affine2::IDENTITY);
}

#[test]
fn visible_user_matches_the_ungated_path() {
    // 10 ms frames: steps change every 3-4 frames, so same-step frames run too.
    let (mut gated, mut plain) = (app(true, 10), app(false, 10));
    let (a, b) = (model(&mut gated, scroll()), model(&mut plain, scroll()));
    user(&mut gated, &a, true);
    for frame in 0..60 {
        gated.update();
        plain.update();
        assert!(uv(&gated, &a).abs_diff_eq(uv(&plain, &b), 1e-6), "frame {frame}");
    }
    assert!(modified(&gated) > 0);
    assert_eq!(modified(&gated), modified(&plain));
}

#[test]
fn becoming_visible_gets_the_current_value_at_once() {
    let (mut gated, mut plain) = (app(true, 10), app(false, 10));
    let (a, b) = (model(&mut gated, scroll()), model(&mut plain, scroll()));
    let entity = user(&mut gated, &a, false);
    for _ in 0..47 {
        gated.update();
        plain.update();
    }
    assert_eq!(modified(&gated), 0);
    assert!(!uv(&gated, &a).abs_diff_eq(uv(&plain, &b), 1e-6), "the scroll moved while hidden");
    // The first update that sees it visible writes the step value (the same-step case is the next test).
    set_visible(&mut gated, entity, true);
    gated.update();
    plain.update();
    assert!(uv(&gated, &a).abs_diff_eq(uv(&plain, &b), 1e-6));
    assert_eq!(modified(&gated), 1);
}

#[test]
fn shown_between_steps_catches_up_without_waiting_for_the_next_step() {
    // 1 ms frames: dozens of frames share a 30 Hz step.
    let (mut gated, mut plain) = (app(true, 1), app(false, 1));
    let (a, b) = (model(&mut gated, scroll()), model(&mut plain, scroll()));
    let entity = user(&mut gated, &a, false);
    for _ in 0..50 {
        gated.update();
        plain.update();
    }
    let step = |app: &App| (app.world().resource::<Time>().elapsed_secs() * MATERIAL_ANIM_HZ).floor();
    let before = step(&gated);
    set_visible(&mut gated, entity, true);
    gated.update();
    plain.update();
    assert!((step(&gated) - before).abs() < 0.5, "the catch-up frame stays in the step");
    assert!(uv(&gated, &a).abs_diff_eq(uv(&plain, &b), 1e-6));
}

#[test]
fn unchanged_value_is_written_once() {
    for gated in [false, true] {
        let mut app = app(gated, 40);
        let material = model(&mut app, constant_alpha());
        user(&mut app, &material, true);
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(modified(&app), 1, "gated {gated}");
        let alpha = app.world().resource::<Assets<StandardMaterial>>().get(&material).unwrap().base_color.alpha();
        assert!((alpha - 0.5).abs() < 1e-6);
    }
}

#[test]
fn users_follow_spawn_swap_and_despawn() {
    let mut app = app(true, 40);
    let material = model(&mut app, scroll());
    // Untracked (drawn by something we don't see): animates as without the gate.
    app.update();
    app.update();
    assert!(modified(&app) > 0);
    let entity = user(&mut app, &material, true);
    let id = material.id();
    assert_eq!(app.world().resource::<MaterialUsers>().users(id).map(<[Entity]>::len), Some(2), "mesh material + marker");
    // A fogged copy replaces the mesh material; the marker keeps the entity a user.
    let other = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
    app.world_mut().entity_mut(entity).insert(MeshMaterial3d(other.clone()));
    assert_eq!(app.world().resource::<MaterialUsers>().users(id), Some(&[entity][..]));
    assert_eq!(app.world().resource::<MaterialUsers>().users(other.id()), Some(&[entity][..]));
    // Despawned: tracked with no user, so skipped.
    app.world_mut().entity_mut(entity).despawn();
    assert_eq!(app.world().resource::<MaterialUsers>().users(id), Some(&[][..]));
    let before = modified(&app);
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(modified(&app), before);
}

#[test]
fn only_materials_of_played_controllers_are_marked() {
    let mut idle = scroll();
    idle.auto_play = false;
    let mut on_second = constant_alpha();
    on_second.target = Target::Alpha { material: 1, curve: Curve::linear([(0.0, 0.5)]) };
    assert_eq!(animated_materials(&[idle, on_second], 3), vec![false, true, false]);
    assert_eq!(animated_materials(&[scroll()], 0), Vec::<bool>::new(), "out-of-range index ignored");
}

/// CW-6 live (`mw.matctrl.gate`): off, nothing is tracked and every step writes; turned on with
/// entities already spawned, their materials are tracked from then on (an invisible user's material
/// is skipped); off again, the next step writes it.
#[test]
fn gate_turns_on_and_off_live() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), sf_tunables_bevy::TunablesPlugin))
        .init_asset::<StandardMaterial>()
        .init_asset::<NifAnimation>()
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(40)))
        .init_resource::<Modified>()
        .add_systems(Update, animate_materials)
        .add_systems(Last, count_modified);
    add_live_gate(&mut app);
    let material = model(&mut app, scroll());
    user(&mut app, &material, false);
    app.update();
    assert!(app.world().get_resource::<MaterialUsers>().is_none(), "off: no tracking");
    let set = |app: &mut App, on: &str| {
        app.world_mut().write_message(sf_tunables_bevy::SetTunable::new("mw.matctrl.gate", on, SetBy::Console));
        app.update();
    };
    set(&mut app, "on");
    assert_eq!(app.world().resource::<MaterialUsers>().users(material.id()).map(<[Entity]>::len), Some(2), "spawned users found");
    app.world_mut().resource_mut::<Modified>().0 = 0;
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(app.world().resource::<Modified>().0, 0, "invisible user: skipped");
    set(&mut app, "off");
    for _ in 0..3 {
        app.update();
    }
    assert!(app.world().resource::<Modified>().0 > 0, "off: animates again");
}
