//! Per-face culling of point shadow cube maps (ROADMAP 1.25 CW-3b; vendored `bevy_pbr` patch
//! "own: point shadow face culling", `render/point_face_cull.rs`): the face-vs-camera test, the
//! reveal rule, and the margin tunable reaching the renderer's settings.

use bevy::MinimalPlugins;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy_app::App;
use bevy_light::atmosphere::ScatteringMedium;
use bevy_math::{Mat4, Vec3};
use bevy_pbr::{
    ALL_POINT_SHADOW_FACES, PointShadowFaceCull, PointShadowFaceCullTracker, point_shadow_faces_seen,
    revealed_face_must_draw,
};
use bevy_render::sync_world::MainEntity;
use bevy_render::view::RetainedViewEntity;
use bevy_shape::ViewFrustum;
use bevy_transform::components::Transform;
use sf_sky::SkySettings;
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use crate::*;

// Face bits in `CUBE_MAP_FACES` order.
const POS_X: u8 = 1 << 0;
const NEG_X: u8 = 1 << 1;
const POS_Y: u8 = 1 << 2;
const NEG_Y: u8 = 1 << 3;
const NEG_Z: u8 = 1 << 4;
const POS_Z: u8 = 1 << 5;

const RANGE: f32 = 10.0;

/// A camera frustum built as Bevy builds a perspective camera's (`PerspectiveProjection`:
/// infinite reverse-Z, far plane at `far`): at `pos`, looking along `dir`, vertical `fov_deg`.
fn camera(pos: Vec3, dir: Vec3, fov_deg: f32, aspect: f32) -> ViewFrustum {
    let up = if dir.normalize().cross(Vec3::Y).length_squared() < 1e-6 { Vec3::Z } else { Vec3::Y };
    let view = Transform::from_translation(pos).looking_to(dir, up);
    let clip_from_view = bevy_math::proj::perspective_infinite_reverse(fov_deg.to_radians(), aspect, 0.1);
    let clip_from_world = clip_from_view * Mat4::from(view.compute_affine()).inverse();
    ViewFrustum::from_clip_from_world_custom_far(&clip_from_world, &pos, &view.back().as_vec3(), 1000.0)
}

fn seen(frusta: &[ViewFrustum], margin_deg: f32) -> u8 {
    point_shadow_faces_seen(Vec3::ZERO, RANGE, margin_deg, frusta)
}

#[test]
fn camera_looking_away_culls_the_faces_behind_it() {
    // Inside the light's sphere, 5 m out along +Z, looking further out: only the +Z face holds
    // receivers in view.
    let frustum = camera(Vec3::new(0.0, 0.0, 5.0), Vec3::Z, 60.0, 16.0 / 9.0);
    assert_eq!(seen(&[frustum], 2.0), POS_Z);
    // Outside the sphere, looking away from it: nothing.
    let away = camera(Vec3::new(0.0, 0.0, 20.0), Vec3::Z, 60.0, 16.0 / 9.0);
    assert_eq!(seen(&[away], 2.0), 0);
}

#[test]
fn camera_inside_the_sphere_keeps_the_faces_covering_its_view() {
    // Near the light, looking diagonally between +X and +Z with a 90 deg view: both of those
    // faces are seen, the two behind (-X, -Z) are not.
    let dir = Vec3::new(1.0, 0.0, 1.0).normalize();
    let mask = seen(&[camera(Vec3::new(2.0, 0.0, 2.0), dir, 90.0, 1.0)], 2.0);
    assert_eq!(mask & (POS_X | POS_Z), POS_X | POS_Z, "mask {mask:06b}");
    assert_eq!(mask & (NEG_X | NEG_Z), 0, "mask {mask:06b}");
    // At the light itself every face its view covers is kept.
    let at_light = seen(&[camera(Vec3::new(0.0, 0.0, 0.0), Vec3::X, 60.0, 1.0)], 2.0);
    assert_eq!(at_light & POS_X, POS_X, "mask {at_light:06b}");
}

#[test]
fn camera_seeing_the_whole_sphere_keeps_every_face() {
    let frustum = camera(Vec3::new(0.0, 0.0, 30.0), Vec3::NEG_Z, 60.0, 16.0 / 9.0);
    assert_eq!(seen(&[frustum], 2.0), ALL_POINT_SHADOW_FACES);
    // No camera known: nothing is culled.
    assert_eq!(seen(&[], 2.0), ALL_POINT_SHADOW_FACES);
}

#[test]
fn seam_margin_keeps_the_neighbour_of_a_visible_face_near_its_edge() {
    // A thin column of view straight down onto receivers 1.5 deg inside the +X face from its
    // seam with +Z (43.5 deg from +X towards +Z, 6 m out): +X is seen; +Z only through the
    // margin (filter taps and the normal-offset bias may cross the seam).
    let a = 43.5_f32.to_radians();
    let receiver = Vec3::new(a.cos(), 0.0, a.sin()) * 6.0;
    let column = camera(receiver + Vec3::Y * 6.0, Vec3::NEG_Y, 0.5, 1.0);
    let without = seen(&[column], 0.0);
    let with = seen(&[column], 2.0);
    assert_eq!(without & POS_X, POS_X, "mask {without:06b}");
    assert_eq!(without & POS_Z, 0, "no margin: +Z culled, mask {without:06b}");
    assert_eq!(with & POS_Z, POS_Z, "2 deg margin: +Z kept, mask {with:06b}");
    // The column reaches above and below the light's equator: the vertical faces are seen too.
    assert_eq!(with & (POS_Y | NEG_Y), POS_Y | NEG_Y, "mask {with:06b}");
}

#[test]
fn several_cameras_union_their_faces() {
    let up = camera(Vec3::new(0.0, 5.0, 0.0), Vec3::Y, 60.0, 1.0);
    let down = camera(Vec3::new(0.0, -5.0, 0.0), Vec3::NEG_Y, 60.0, 1.0);
    assert_eq!(seen(&[up], 2.0), POS_Y);
    assert_eq!(seen(&[up, down], 2.0), POS_Y | NEG_Y);
}

#[test]
fn a_revealed_dirty_face_is_drawn_at_once() {
    let face = RetainedViewEntity::new(MainEntity::from(Entity::from_raw_u32(7).expect("entity")), None, 4);
    let mut tracker = PointShadowFaceCullTracker::default();
    // Visible, then culled for two frames, then back.
    assert!(!tracker.record(face, false));
    tracker.end_frame();
    assert!(!tracker.record(face, true));
    tracker.end_frame();
    assert!(!tracker.record(face, true));
    tracker.end_frame();
    let revealed = tracker.record(face, false);
    tracker.end_frame();
    assert!(revealed, "back in view after being culled");
    assert!(revealed_face_must_draw(revealed, true), "dirty: redrawn before it is sampled");
    assert!(!revealed_face_must_draw(revealed, false), "clean: its kept map is still right");
    // The next frame it is an ordinary visible face again (the refresh budgets apply).
    assert!(!tracker.record(face, false));
}

#[test]
fn margin_tunable_reaches_the_renderer_settings() {
    let settings = SkySettings::default();
    let clock = GameClock::at(172.0, 12.0, settings.calendar.day_ms());
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), TunablesPlugin))
        .init_asset::<ScatteringMedium>()
        .insert_resource(PointShadowFaceCull { enabled: true, ..Default::default() })
        .add_plugins(SkyPlugin { settings, clock, render_path: RenderPath::RayTraced });
    app.finish();
    app.cleanup();
    app.update();
    let cull = *app.world().resource::<PointShadowFaceCull>();
    assert!(cull.enabled, "the game's switch is kept");
    assert!((cull.margin_deg - 2.0).abs() < 1e-6, "default margin");
    app.world_mut().write_message(SetTunable::new("shadow.lamp.face_cull.margin_deg", "5", SetBy::CommandLine));
    app.update();
    assert!((app.world().resource::<PointShadowFaceCull>().margin_deg - 5.0).abs() < 1e-6);
}
