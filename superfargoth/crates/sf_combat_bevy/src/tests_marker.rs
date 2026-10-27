//! Lock-on marker (combat.md "Lock-on and camera"): the projection decision, and headless that
//! the marker node is shown on the locked target and hidden when unlocked or behind the camera.

use bevy_app::App;
use bevy_camera::visibility::Visibility;
use bevy_camera::{Camera, CameraProjection, ComputedCameraValues, PerspectiveProjection, RenderTargetInfo};
use bevy_ecs::prelude::*;
use bevy_math::{Mat4, UVec2, Vec2, Vec3};
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_color::Color;
use bevy_ui::{BackgroundColor, Node, UiTargetCamera, Val};
use sf_camera_bevy::PlayerCamera;

use crate::lock::{LockOn, Lockable};
use crate::marker::{LockMarker, LockMarkerPlugin, LockMarkerShown, LockMarkerStyle, LockMarkerTemplate, MARKER_SIZE, project_marker};

const VIEW: Vec2 = Vec2::new(1280.0, 720.0);

fn projection() -> Mat4 {
    PerspectiveProjection { fov: 60f32.to_radians(), aspect_ratio: VIEW.x / VIEW.y, near: 0.1, ..PerspectiveProjection::default() }
        .get_clip_from_view()
}

#[test]
fn projects_ahead_and_refuses_behind_or_off_screen() {
    let p = project_marker(projection(), Vec3::new(0.0, 0.0, -5.0), VIEW).expect("straight ahead");
    assert!((p - VIEW / 2.0).length() < 1e-3, "centre: {p}");
    let up = project_marker(projection(), Vec3::new(0.0, 1.0, -5.0), VIEW).expect("above the centre");
    assert!(up.y < VIEW.y / 2.0 && (up.x - VIEW.x / 2.0).abs() < 1e-3, "screen y grows down: {up}");
    assert_eq!(project_marker(projection(), Vec3::new(0.0, 0.0, 5.0), VIEW), None, "behind");
    assert_eq!(project_marker(projection(), Vec3::ZERO, VIEW), None, "at the eye");
    assert_eq!(project_marker(projection(), Vec3::new(50.0, 0.0, -5.0), VIEW), None, "off screen");
}

struct Scene {
    app: App,
    camera: Entity,
    player: Entity,
    target: Entity,
}

/// A camera at the origin looking down -Z (as if rendered: its computed projection and size set),
/// following `player`, and a lockable `target` 5 m ahead, its aim point level with the eye.
fn scene() -> Scene {
    let mut app = App::new();
    app.add_plugins(LockMarkerPlugin);
    let target = app.world_mut().spawn((GlobalTransform::from_xyz(0.0, -1.3, -5.0), Lockable { aim_height: 1.3 })).id();
    let player = app.world_mut().spawn(Transform::default()).id();
    let camera = Camera {
        computed: ComputedCameraValues {
            clip_from_view: projection(),
            target_info: Some(RenderTargetInfo { physical_size: UVec2::new(1280, 720), scale_factor: 1.0 }),
            ..ComputedCameraValues::default()
        },
        ..Camera::default()
    };
    let camera = app.world_mut().spawn((camera, GlobalTransform::IDENTITY, PlayerCamera::new(player))).id();
    Scene { app, camera, player, target }
}

fn marker(app: &mut App) -> (Visibility, Val, Val, Option<Entity>) {
    let mut q = app.world_mut().query_filtered::<(&Visibility, &Node, Option<&UiTargetCamera>), With<LockMarker>>();
    let all: Vec<_> = q.iter(app.world()).map(|(v, n, t)| (*v, n.left, n.top, t.map(|t| t.0))).collect();
    assert_eq!(all.len(), 1, "one marker");
    all[0]
}

#[test]
fn marker_follows_the_locked_target_and_hides_otherwise() {
    let Scene { mut app, camera, player, target } = scene();
    app.update();
    assert_eq!(marker(&mut app).0, Visibility::Hidden, "no lock: hidden");
    assert!(!app.world().resource::<LockMarkerShown>().0);

    app.world_mut().entity_mut(player).insert(LockOn(target));
    app.update();
    let (visibility, left, top, on) = marker(&mut app);
    assert_eq!(visibility, Visibility::Inherited, "locked: shown");
    assert!(app.world().resource::<LockMarkerShown>().0, "shown, for the host HUD");
    assert_eq!((left, top), (Val::Px(640.0 - MARKER_SIZE / 2.0), Val::Px(360.0 - MARKER_SIZE / 2.0)), "centred on the aim point");
    assert_eq!(on, Some(camera), "drawn in the player camera's view");

    // The target steps right: the dot follows the same frame.
    app.world_mut().entity_mut(target).insert(GlobalTransform::from_xyz(1.0, -1.3, -5.0));
    app.update();
    let (visibility, left, ..) = marker(&mut app);
    assert_eq!(visibility, Visibility::Inherited);
    assert!(matches!(left, Val::Px(x) if x > 640.0), "moved right: {left:?}");

    app.world_mut().entity_mut(target).insert(GlobalTransform::from_xyz(0.0, -1.3, 5.0));
    app.update();
    assert_eq!(marker(&mut app).0, Visibility::Hidden, "behind the camera: hidden");

    app.world_mut().entity_mut(target).insert(GlobalTransform::from_xyz(0.0, -1.3, -5.0));
    app.update();
    assert_eq!(marker(&mut app).0, Visibility::Inherited, "back in view");
    app.world_mut().entity_mut(player).remove::<LockOn>();
    app.update();
    assert_eq!(marker(&mut app).0, Visibility::Hidden, "released: hidden");
    assert!(!app.world().resource::<LockMarkerShown>().0);
}

#[test]
fn marker_wears_the_template_nodes_look_and_centres_by_it() {
    let Scene { mut app, player, target, .. } = scene();
    let fill = Color::srgb(0.87, 0.79, 0.62);
    let template = Node { width: Val::Px(4.0), height: Val::Px(4.0), ..Node::default() };
    app.world_mut().spawn((LockMarkerTemplate, template, BackgroundColor(fill)));
    app.world_mut().entity_mut(player).insert(LockOn(target));
    app.update();
    let style = *app.world().resource::<LockMarkerStyle>();
    assert_eq!((style.size, style.fill, style.rim), (4.0, fill, 0.0), "the template's look");
    let mut q = app.world_mut().query_filtered::<(&Node, &BackgroundColor), With<LockMarker>>();
    let (node, marker_fill) = q.single(app.world()).expect("one marker");
    assert_eq!((node.width, node.height, marker_fill.0), (Val::Px(4.0), Val::Px(4.0), fill));
    assert_eq!((node.left, node.top), (Val::Px(638.0), Val::Px(358.0)), "centred by the template's size");
}
