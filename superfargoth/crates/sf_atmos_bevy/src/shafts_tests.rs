//! LSO (`docs/specs/aerial-shadow.md` "S3", vendored `bevy_pbr` patch "own, LSO: light shaft
//! occlusion"): the `sky.shafts.*` tunables reach [`LightShafts`], the sun follows the
//! [`CloudShadowLight`], each view's params come out of its projection exactly, and the render
//! world marks a camera for the passes only while the occlusion is on (no marker = no
//! `bevy_pbr::AtmosphereLightShafts` = the stock `render_sky`). No GPU: the shaders are checked in
//! `shader_tests.rs`, the formulas in `sf_atmos::shafts`.

use bevy::MinimalPlugins;
use bevy_app::App;
use bevy_camera::Camera3d;
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_math::{Mat4, Quat, Vec2, Vec3, Vec4};
use bevy_pbr::AtmosphereSettings;
use bevy_render::sync_world::RenderEntity;
use bevy_render::MainWorld;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use super::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TunablesPlugin));
    shafts::build(&mut app);
    app.finish();
    app.cleanup();
    app
}

/// Logic: the tunables' defaults (on), live sets, and the sun = the cloud shadow light's back.
#[test]
fn the_tunables_and_the_sun_reach_the_settings() {
    let mut app = app();
    app.update();
    assert_eq!(*app.world().resource::<LightShafts>(), LightShafts::default());
    assert!(app.world().resource::<LightShafts>().enabled, "on by default");
    assert_eq!(app.world().resource::<ShaftsSun>().0, None, "no sun yet");

    let rotation = Quat::from_rotation_x(-0.2);
    app.world_mut().spawn((DirectionalLight::default(), CloudShadowLight, GlobalTransform::from(Transform::from_rotation(rotation))));
    app.world_mut().write_message(SetTunable::new("sky.shafts.taps", "12", SetBy::Console));
    app.world_mut().write_message(SetTunable::new("sky.shafts.occlusion", "0.5", SetBy::Console));
    app.world_mut().write_message(SetTunable::new("sky.shafts.enabled", "false", SetBy::Console));
    app.update();
    let s = *app.world().resource::<LightShafts>();
    assert_eq!((s.taps, s.occlusion, s.enabled), (12, 0.5, false));
    let sun = app.world().resource::<ShaftsSun>().0.expect("the shadow light");
    assert!(sun.dot(rotation * Vec3::Z) > 0.999_99, "toward the light: {sun}");
}

fn projection() -> Mat4 {
    // Bevy's infinite reverse-Z perspective (`ExtractedView::clip_from_view`), 90° vertical, 16:9.
    let (f, aspect, near) = (1.0, 16.0 / 9.0, 0.1);
    Mat4::from_cols(
        Vec4::new(f / aspect, 0.0, 0.0, 0.0),
        Vec4::new(0.0, f, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, -1.0),
        Vec4::new(0.0, 0.0, near, 0.0),
    )
}

/// Logic: the sun ahead is on screen at full darkness; behind the camera the passes are off; half
/// way through the off-screen fade, half the darkness; too few taps or no length = no blur.
#[test]
fn view_params_follow_the_projection() {
    let s = LightShafts::default();
    let ahead = ShaftsViewParams::new(&s, projection(), Vec3::NEG_Z, 1.0);
    assert_eq!(ahead.sun_uv, Some(Vec2::splat(0.5)));
    assert!((ahead.darkness - s.occlusion).abs() < 1e-6 && ahead.active());
    assert_eq!((ahead.passes, ahead.taps), (s.passes, s.taps));

    let behind = ShaftsViewParams::new(&s, projection(), Vec3::Z, 1.0);
    assert_eq!((behind.sun_uv, behind.darkness, behind.active()), (None, 0.0, false));

    // 90° vertical fov: tan(θ) = 1.3 puts the sun 0.15 UV above the screen (half the 0.3 fade).
    let above = ShaftsViewParams::new(&s, projection(), Vec3::new(0.0, 1.3, -1.0).normalize(), 1.0);
    let uv = above.sun_uv.expect("in front");
    assert!((uv.y + 0.15).abs() < 1e-5, "{uv}");
    assert!((above.darkness - 0.5 * s.occlusion).abs() < 1e-5, "{}", above.darkness);

    let far_off = ShaftsViewParams::new(&s, projection(), Vec3::new(0.0, 5.0, -1.0).normalize(), 1.0);
    assert!(!far_off.active(), "past the fade");

    assert_eq!(ShaftsViewParams::new(&LightShafts { taps: 1, ..s }, projection(), Vec3::NEG_Z, 1.0).passes, 0);
    assert_eq!(ShaftsViewParams::new(&LightShafts { length: 0.0, ..s }, projection(), Vec3::NEG_Z, 1.0).passes, 0);
    assert_eq!(ShaftsViewParams::new(&LightShafts { passes: 99, ..s }, projection(), Vec3::NEG_Z, 1.0).passes, 6, "clamped");
}

/// Logic (S3b): the bloom adds `bloom_scale` with the sun on screen, fades with it off screen like
/// the occlusion, is off behind the camera, with `sky.shafts.bloom` off and at night (the planet
/// hides the sun), and is independent of the occlusion's strength.
#[test]
fn bloom_params_follow_the_sun_and_the_night_gate() {
    let s = LightShafts { bloom: true, ..LightShafts::default() };
    let ahead = ShaftsViewParams::new(&s, projection(), Vec3::NEG_Z, 1.0);
    assert!((ahead.bloom - s.bloom_scale).abs() < 1e-6 && ahead.bloom_active(), "{}", ahead.bloom);

    let above = ShaftsViewParams::new(&s, projection(), Vec3::new(0.0, 1.3, -1.0).normalize(), 1.0);
    assert!((above.bloom - 0.5 * s.bloom_scale).abs() < 1e-5, "half the off-screen fade: {}", above.bloom);

    let behind = ShaftsViewParams::new(&s, projection(), Vec3::Z, 1.0);
    assert!(!behind.bloom_active(), "behind the camera");

    let night = ShaftsViewParams::new(&s, projection(), Vec3::NEG_Z, 0.0);
    assert!(!night.bloom_active(), "the sun below the horizon: {}", night.bloom);
    assert!(night.active(), "the night gate is the bloom's only");
    let dusk = ShaftsViewParams::new(&s, projection(), Vec3::NEG_Z, 0.25);
    assert!((dusk.bloom - 0.25 * s.bloom_scale).abs() < 1e-6, "on the terminator: {}", dusk.bloom);

    let off = ShaftsViewParams::new(&LightShafts { bloom: false, ..s }, projection(), Vec3::NEG_Z, 1.0);
    assert!(!off.bloom_active() && off.active(), "sky.shafts.bloom off, the occlusion stays");
    let unmasked = ShaftsViewParams::new(&LightShafts { occlusion: 0.0, ..s }, projection(), Vec3::NEG_Z, 1.0);
    assert!(unmasked.bloom_active() && !unmasked.active(), "no occlusion, the bloom stays");
}

/// Logic: the bloom tunables' defaults (UE's light shaft bloom numbers) and live sets reach the
/// settings, the tint as three floats.
#[test]
fn the_bloom_tunables_reach_the_settings() {
    let mut app = app();
    app.update();
    let s = *app.world().resource::<LightShafts>();
    assert_eq!((s.bloom, s.bloom_scale, s.bloom_threshold, s.bloom_max, s.bloom_tint, s.bloom_radius), (false, 0.2, 8.0, 100.0, Vec3::ONE, 0.5));
    app.world_mut().write_message(SetTunable::new("sky.shafts.bloom_threshold", "2", SetBy::Console));
    app.world_mut().write_message(SetTunable::new("sky.shafts.bloom_tint", "1.0, 0.8, 0.6", SetBy::Console));
    app.world_mut().write_message(SetTunable::new("sky.shafts.bloom", "true", SetBy::Console));
    app.update();
    let s = *app.world().resource::<LightShafts>();
    assert_eq!((s.bloom, s.bloom_threshold, s.bloom_tint), (true, 2.0, Vec3::new(1.0, 0.8, 0.6)));
}

/// Every `sky.*` tunable this crate registers is in its `TUNABLES` list (the game's catalogue).
#[test]
fn every_registered_tunable_is_listed() {
    let app = app();
    let listed: Vec<&str> = TUNABLES.iter().map(|d| d.name()).collect();
    for name in ["sky.shafts.enabled", "sky.shafts.lobe_g", "sky.shafts.bloom", "sky.shafts.bloom_tint", "sky.shafts.bloom_radius"] {
        assert!(listed.contains(&name), "{name}");
    }
    let store = app.world().resource::<sf_tunables_bevy::Tunables>();
    for name in &listed {
        if name.starts_with("sky.shafts.") {
            assert!(store.0.id(name).is_some(), "{name} registered by shafts::build");
        }
    }
}

/// Wiring: the render camera is marked for the passes (and so gets the vendored occlusion
/// component) only while the occlusion is on, a sun exists and the camera has Bevy's atmosphere.
#[test]
fn extraction_marks_the_camera_only_while_enabled() {
    let mut main = World::new();
    main.insert_resource(LightShafts::default());
    main.insert_resource(ShaftsSun(Some(Vec3::Y)));
    let mut render = World::new();
    render.init_resource::<MainWorld>();
    let extract = render.register_system(shafts::extract);
    let render_camera = render.spawn_empty().id();
    let plain_render = render.spawn_empty().id();
    let camera = main.spawn((Camera3d::default(), AtmosphereSettings::default(), RenderEntity::from(render_camera))).id();
    main.spawn((Camera3d::default(), RenderEntity::from(plain_render)));
    let run = |main: &mut World, render: &mut World| {
        std::mem::swap(main, &mut *render.resource_mut::<MainWorld>());
        render.run_system(extract).expect("extract runs");
        render.flush();
        std::mem::swap(main, &mut *render.resource_mut::<MainWorld>());
    };

    run(&mut main, &mut render);
    assert!(render.get::<ShaftsCamera>(render_camera).is_some(), "on");
    assert!(render.get::<ShaftsCamera>(plain_render).is_none(), "no atmosphere: stock");

    main.resource_mut::<LightShafts>().enabled = false;
    run(&mut main, &mut render);
    assert!(render.get::<ShaftsCamera>(render_camera).is_none(), "sky.shafts.enabled off");

    main.resource_mut::<LightShafts>().enabled = true;
    run(&mut main, &mut render);
    assert!(render.get::<ShaftsCamera>(render_camera).is_some(), "back on");

    main.resource_mut::<ShaftsSun>().0 = None;
    run(&mut main, &mut render);
    assert!(render.get::<ShaftsCamera>(render_camera).is_none(), "no sun");

    main.resource_mut::<ShaftsSun>().0 = Some(Vec3::Y);
    main.entity_mut(camera).remove::<AtmosphereSettings>();
    run(&mut main, &mut render);
    assert!(render.get::<ShaftsCamera>(render_camera).is_none(), "the atmosphere removed");
}

/// Wiring (S3b): the bloom pass's marker ([`ShaftsBloomCamera`], which `bloom_pass`, its pipelines
/// and textures require) is on the render camera only while both `sky.shafts.enabled` and
/// `sky.shafts.bloom` are on.
#[test]
fn the_bloom_pass_runs_only_while_shafts_and_bloom_are_on() {
    let mut main = World::new();
    main.insert_resource(LightShafts::default());
    main.insert_resource(ShaftsSun(Some(Vec3::Y)));
    let mut render = World::new();
    render.init_resource::<MainWorld>();
    let extract = render.register_system(shafts::extract);
    let render_camera = render.spawn_empty().id();
    main.spawn((Camera3d::default(), AtmosphereSettings::default(), RenderEntity::from(render_camera)));
    let run = |main: &mut World, render: &mut World| {
        std::mem::swap(main, &mut *render.resource_mut::<MainWorld>());
        render.run_system(extract).expect("extract runs");
        render.flush();
        std::mem::swap(main, &mut *render.resource_mut::<MainWorld>());
        (render.get::<ShaftsCamera>(render_camera).is_some(), render.get::<ShaftsBloomCamera>(render_camera).is_some())
    };

    assert_eq!(run(&mut main, &mut render), (true, false), "the defaults: occlusion on, bloom off (FG-12 verdict)");
    main.resource_mut::<LightShafts>().bloom = true;
    assert_eq!(run(&mut main, &mut render), (true, true), "both on");
    main.resource_mut::<LightShafts>().bloom = false;
    assert_eq!(run(&mut main, &mut render), (true, false), "sky.shafts.bloom off: the occlusion only");
    main.resource_mut::<LightShafts>().bloom = true;
    assert_eq!(run(&mut main, &mut render), (true, true), "back on");
    main.resource_mut::<LightShafts>().enabled = false;
    assert_eq!(run(&mut main, &mut render), (false, false), "sky.shafts.enabled off (--off shafts): no bloom either");
}
