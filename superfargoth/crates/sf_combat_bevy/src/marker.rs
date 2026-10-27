//! Lock-on marker (combat.md "Lock-on and camera"): a small white dot with a thin dark rim on the
//! locked target's aim point, as Elden Ring's lock reticle. Screen space, constant pixel size, in
//! third and first person.
//!
//! - **What.** One Bevy UI node ([`LockMarker`], spawned hidden at startup). Each frame the first
//!   active [`PlayerCamera`] whose followed character has a `LockOn` places it on that target's
//!   [`Lockable::aim_height`] above the target's root, and targets that camera
//!   (`UiTargetCamera`), so the pixel position is in that camera's viewport.
//! - **When.** `PostUpdate` after transform propagation (`big_space` included) and the camera
//!   update, before the UI layout: the camera and the target's `GlobalTransform`s are the same
//!   frame's (both relative to the floating origin), so the dot doesn't lag a frame.
//! - **Hidden** without a lock, and while the point is behind the camera or off screen
//!   ([`project_marker`], Bevy-free).

use bevy_app::{App, Plugin, PostUpdate, Startup};
use bevy_camera::visibility::Visibility;
use bevy_camera::{Camera, CameraUpdateSystems};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_math::{Mat4, Vec2, Vec3};
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;
use bevy_ui::{BorderColor, BorderRadius, Node, PositionType, UiRect, UiSystems, UiTargetCamera, Val};
use sf_camera_bevy::PlayerCamera;

use crate::lock::{LockOn, Lockable};

/// The marker's diameter, rim included, logical px.
pub const MARKER_SIZE: f32 = 8.0;
/// The dark rim's width, logical px (contrast on bright skies and sand).
pub const MARKER_RIM: f32 = 1.0;

/// The lock-on marker node (one, [`LockMarkerPlugin`]).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LockMarker;

/// The lock-on marker. Needs the camera's `LockOn` mirror (`SfLockOnPlugin`) and the host's UI.
pub struct LockMarkerPlugin;

impl Plugin for LockMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_marker).add_systems(
            PostUpdate,
            place_marker
                .after(TransformSystems::Propagate)
                .after(CameraUpdateSystems)
                .before(UiSystems::Prepare),
        );
    }
}

/// Where a camera-space point lands in a `viewport`-sized view (logical px, origin top left), or
/// `None` when it is behind the camera (cameras look down -Z) or outside the view.
#[must_use]
pub fn project_marker(clip_from_view: Mat4, view_point: Vec3, viewport: Vec2) -> Option<Vec2> {
    if view_point.z >= 0.0 || !view_point.is_finite() {
        return None;
    }
    let ndc = clip_from_view.project_point3(view_point);
    let on_screen = ndc.is_finite() && ndc.x.abs() <= 1.0 && ndc.y.abs() <= 1.0 && (0.0..=1.0).contains(&ndc.z);
    // NDC y is up, the UI's y is down.
    on_screen.then(|| (Vec2::new(ndc.x, -ndc.y) * 0.5 + 0.5) * viewport)
}

fn spawn_marker(mut commands: Commands) {
    commands.spawn((
        LockMarker,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(MARKER_SIZE),
            height: Val::Px(MARKER_SIZE),
            border: UiRect::all(Val::Px(MARKER_RIM)),
            border_radius: BorderRadius::MAX,
            ..Node::default()
        },
        bevy_ui::BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.85)),
        BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        Visibility::Hidden,
    ));
}

type MarkerQuery = (Entity, &'static mut Node, &'static mut Visibility, Option<&'static UiTargetCamera>);

/// Each frame: the marker on the first player camera's locked target, or hidden (module docs).
fn place_marker(
    mut commands: Commands,
    cameras: Query<(Entity, &Camera, &GlobalTransform, &PlayerCamera)>,
    owners: Query<&LockOn>,
    targets: Query<(&GlobalTransform, &Lockable)>,
    mut markers: Query<MarkerQuery, With<LockMarker>>,
) {
    let placed = cameras.iter().filter(|c| c.1.is_active).find_map(|(e, camera, view, player)| {
        let (target, lockable) = targets.get(owners.get(player.follow).ok()?.0).ok()?;
        let aim = target.translation() + Vec3::Y * lockable.aim_height;
        let view_point = view.affine().inverse().transform_point3(aim);
        project_marker(camera.clip_from_view(), view_point, camera.logical_viewport_size()?).map(|p| (e, p))
    });
    for (marker, mut node, mut visibility, target_camera) in &mut markers {
        let Some((camera, at)) = placed else {
            visibility.set_if_neq(Visibility::Hidden);
            continue;
        };
        let (left, top) = (Val::Px(at.x - MARKER_SIZE / 2.0), Val::Px(at.y - MARKER_SIZE / 2.0));
        if node.left != left || node.top != top {
            node.left = left;
            node.top = top;
        }
        visibility.set_if_neq(Visibility::Inherited);
        if target_camera.map(|t| t.0) != Some(camera) {
            commands.entity(marker).insert(UiTargetCamera(camera));
        }
    }
}
