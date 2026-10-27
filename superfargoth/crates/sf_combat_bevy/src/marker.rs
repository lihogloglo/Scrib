//! Lock-on marker (combat.md "Lock-on and camera"): the aim cursor moved onto the locked target's
//! aim point, as Elden Ring's lock reticle. Screen space, constant pixel size, in third and first
//! person.
//!
//! - **What.** One Bevy UI node ([`LockMarker`], spawned hidden at startup). Each frame the first
//!   active [`PlayerCamera`] whose followed character has a `LockOn` places it on that target's
//!   [`Lockable::aim_height`] above the target's root, and targets that camera
//!   (`UiTargetCamera`), so the pixel position is in that camera's viewport.
//! - **Look.** [`LockMarkerStyle`]: by default a small white dot with a thin dark rim. A host with
//!   its own screen-centre crosshair tags that node [`LockMarkerTemplate`] and the marker copies
//!   its look (size, fill, rim, corner radius, as themed), so one cursor reads on screen and it
//!   "moves onto the target".
//! - **When.** `PostUpdate` after transform propagation (`big_space` included) and the camera
//!   update, before the UI layout: the camera and the target's `GlobalTransform`s are the same
//!   frame's (both relative to the floating origin), so the dot doesn't lag a frame.
//! - **Hidden** without a lock, and while the point is behind the camera or off screen
//!   ([`project_marker`], Bevy-free). [`LockMarkerShown`] says whether it is shown, for a host HUD
//!   that hides its centre crosshair meanwhile (read next frame: the HUD runs in `Update`).

use bevy_app::{App, Plugin, PostUpdate, Startup};
use bevy_camera::visibility::Visibility;
use bevy_camera::{Camera, CameraUpdateSystems};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_math::{Mat4, Vec2, Vec3};
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;
use bevy_ui::{BackgroundColor, BorderColor, BorderRadius, Node, PositionType, UiRect, UiSystems, UiTargetCamera, Val};
use sf_camera_bevy::PlayerCamera;

use crate::lock::{LockOn, Lockable};

/// The default marker's diameter, rim included, logical px.
pub const MARKER_SIZE: f32 = 8.0;
/// The default dark rim's width, logical px (contrast on bright skies and sand).
pub const MARKER_RIM: f32 = 1.0;

/// The lock-on marker node (one, [`LockMarkerPlugin`]).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LockMarker;

/// On a host UI node (its crosshair) whose look the marker copies into [`LockMarkerStyle`]
/// (when it has a pixel width; one such node).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LockMarkerTemplate;

/// Whether the marker is on screen this frame (written in `PostUpdate` by [`LockMarkerPlugin`]).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LockMarkerShown(pub bool);

/// The marker's look. Hosts set it (e.g. to their crosshair's); applied when it changes.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LockMarkerStyle {
    /// Width and height, rim included, logical px.
    pub size: f32,
    pub fill: Color,
    /// Rim width, logical px (0: none).
    pub rim: f32,
    pub rim_color: Color,
    pub radius: BorderRadius,
}

impl Default for LockMarkerStyle {
    fn default() -> Self {
        Self {
            size: MARKER_SIZE,
            fill: Color::srgba(1.0, 1.0, 1.0, 0.85),
            rim: MARKER_RIM,
            rim_color: Color::srgba(0.0, 0.0, 0.0, 0.55),
            radius: BorderRadius::MAX,
        }
    }
}

/// The lock-on marker. Needs the camera's `LockOn` mirror (`SfLockOnPlugin`) and the host's UI.
pub struct LockMarkerPlugin;

impl Plugin for LockMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LockMarkerShown>()
            .init_resource::<LockMarkerStyle>()
            .add_systems(Startup, spawn_marker)
            .add_systems(
                PostUpdate,
                (follow_template, style_marker.run_if(resource_changed::<LockMarkerStyle>), place_marker)
                    .chain()
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
    // The look comes from `style_marker` (its first run sees the style as changed).
    commands.spawn((
        LockMarker,
        Node { position_type: PositionType::Absolute, ..Node::default() },
        BackgroundColor::default(),
        BorderColor::default(),
        Visibility::Hidden,
    ));
}

/// The [`LockMarkerTemplate`] node's look into [`LockMarkerStyle`] (when it has a pixel width).
type TemplateQuery = (&'static Node, Option<&'static BackgroundColor>, Option<&'static BorderColor>);

fn follow_template(
    templates: Query<TemplateQuery, With<LockMarkerTemplate>>,
    mut style: ResMut<LockMarkerStyle>,
) {
    let Some((node, fill, rim)) = templates.iter().next() else { return };
    let Val::Px(size) = node.width else { return };
    let rim_width = if let Val::Px(w) = node.border.left { w } else { 0.0 };
    style.set_if_neq(LockMarkerStyle {
        size,
        fill: fill.map_or(Color::NONE, |c| c.0),
        rim: rim_width,
        rim_color: rim.map_or(Color::NONE, |c| c.top),
        radius: node.border_radius,
    });
}

/// [`LockMarkerStyle`] onto the marker node.
fn style_marker(
    style: Res<LockMarkerStyle>,
    mut markers: Query<(&mut Node, &mut BackgroundColor, &mut BorderColor), With<LockMarker>>,
) {
    for (mut node, mut fill, mut rim) in &mut markers {
        node.width = Val::Px(style.size);
        node.height = Val::Px(style.size);
        node.border = UiRect::all(Val::Px(style.rim));
        node.border_radius = style.radius;
        fill.0 = style.fill;
        *rim = BorderColor::all(style.rim_color);
    }
}

type MarkerQuery = (Entity, &'static mut Node, &'static mut Visibility, Option<&'static UiTargetCamera>);

/// Each frame: the marker on the first player camera's locked target, or hidden (module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn place_marker(
    mut commands: Commands,
    style: Res<LockMarkerStyle>,
    cameras: Query<(Entity, &Camera, &GlobalTransform, &PlayerCamera)>,
    owners: Query<&LockOn>,
    targets: Query<(&GlobalTransform, &Lockable)>,
    mut markers: Query<MarkerQuery, With<LockMarker>>,
    mut shown: ResMut<LockMarkerShown>,
) {
    let placed = cameras.iter().filter(|c| c.1.is_active).find_map(|(e, camera, view, player)| {
        let (target, lockable) = targets.get(owners.get(player.follow).ok()?.0).ok()?;
        let aim = target.translation() + Vec3::Y * lockable.aim_height;
        let view_point = view.affine().inverse().transform_point3(aim);
        project_marker(camera.clip_from_view(), view_point, camera.logical_viewport_size()?).map(|p| (e, p))
    });
    let mut any = false;
    for (marker, mut node, mut visibility, target_camera) in &mut markers {
        let Some((camera, at)) = placed else {
            visibility.set_if_neq(Visibility::Hidden);
            continue;
        };
        let half = style.size / 2.0;
        let (left, top) = (Val::Px(at.x - half), Val::Px(at.y - half));
        if node.left != left || node.top != top {
            node.left = left;
            node.top = top;
        }
        visibility.set_if_neq(Visibility::Inherited);
        any = true;
        if target_camera.map(|t| t.0) != Some(camera) {
            commands.entity(marker).insert(UiTargetCamera(camera));
        }
    }
    shown.set_if_neq(LockMarkerShown(any));
}
