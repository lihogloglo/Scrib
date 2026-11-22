//! A2 (`docs/specs/aerial-shadow.md` "Architecture review"): the `sky.sun_vis.*` tunables reach
//! [`SunVisibility`], the per-view values come out of the settings exactly, the aerial hook masks
//! the sun with no lobe, and the render world marks a camera for the pass only while it is on with
//! a sun and Bevy's atmosphere. No GPU: the shaders are checked in `shader_tests.rs`, the march in
//! `sf_atmos::sun_visibility`.

use bevy::MinimalPlugins;
use bevy_app::App;
use bevy_camera::Camera3d;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_pbr::AtmosphereSettings;
use bevy_render::sync_world::RenderEntity;
use bevy_render::MainWorld;
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use super::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TunablesPlugin));
    sun_visibility::build(&mut app);
    app.finish();
    app.cleanup();
    app
}

/// Logic: the defaults (on, 16 samples, 5 km, history 0.9), live sets, every tunable listed.
#[test]
fn the_tunables_reach_the_settings() {
    let mut app = app();
    app.update();
    let s = *app.world().resource::<SunVisibility>();
    assert_eq!(s, SunVisibility { enabled: true, samples: 16, max_distance_m: 5000.0, history: 0.9 });
    for (name, value) in [("sky.sun_vis.samples", "24"), ("sky.sun_vis.max_distance_m", "3000"), ("sky.sun_vis.history", "0.5"), ("sky.sun_vis.enabled", "false")] {
        app.world_mut().write_message(SetTunable::new(name, value, SetBy::Console));
    }
    app.update();
    assert_eq!(*app.world().resource::<SunVisibility>(), SunVisibility { enabled: false, samples: 24, max_distance_m: 3000.0, history: 0.5 });
    let listed: Vec<&str> = TUNABLES.iter().map(|d| d.name()).collect();
    let store = app.world().resource::<sf_tunables_bevy::Tunables>();
    for name in ["sky.sun_vis.enabled", "sky.sun_vis.samples", "sky.sun_vis.max_distance_m", "sky.sun_vis.history"] {
        assert!(listed.contains(&name), "{name} in TUNABLES");
        assert!(store.0.id(name).is_some(), "{name} registered");
    }
}

/// Logic: the per-view values: samples clamped to the range, the sky's end the fog's sky distance
/// but at least the reach, the fog's weights from the froxels' far (0 without froxels).
#[test]
fn view_params_follow_the_settings() {
    let s = SunVisibility::default();
    let p = SunVisViewParams::new(&s, Some(20_000.0), Some(384.0));
    assert_eq!(p, SunVisViewParams { samples: 16, max_distance: 5000.0, sky_end: 20_000.0, fog_start: 384.0, history: 0.9 });
    let p = SunVisViewParams::new(&SunVisibility { samples: 0, history: 2.0, ..s }, Some(1000.0), None);
    assert_eq!((p.samples, p.sky_end, p.fog_start, p.history), (1, 5000.0, 0.0, 1.0));
    assert_eq!(SunVisViewParams::new(&SunVisibility { samples: 500, ..s }, None, None).samples, 64);
}

/// Logic: the aerial perspective's hook: the sun, full darkness, `lobe_g` 0 (`render_sky`'s lobe
/// is then exactly 1); nothing masked until the pass writes its result.
#[test]
#[expect(clippy::float_cmp, reason = "exact: copied values and a closed-form 1")]
fn the_aerial_hook_masks_the_sun_with_no_lobe() {
    let sun = Vec3::new(0.0, 0.2, -1.0).normalize();
    let on = aerial_hook(sun, true);
    assert_eq!((on.light_dir, on.darkness, on.lobe_g), (sun, 1.0, 0.0));
    assert_eq!(aerial_hook(sun, false).darkness, 0.0);
    // `lobe_g` 0: the occlusion's lobe is 1 at every angle (`sf_atmos::shafts::lobe`).
    for cos in [-1.0, 0.0, 0.5, 1.0] {
        assert_eq!(sf_atmos::shafts::lobe(on.lobe_g, cos), 1.0);
    }
}

/// Wiring: the render camera gets the pass only while it is on, a sun exists and the camera has
/// Bevy's atmosphere.
#[test]
fn extraction_marks_the_camera_only_while_enabled() {
    let mut main = World::new();
    main.insert_resource(SunVisibility::default());
    main.insert_resource(ShaftsSun(Some(Vec3::Y)));
    let mut render = World::new();
    render.init_resource::<MainWorld>();
    render.init_resource::<sun_visibility::SunVisHistory>();
    let extract = render.register_system(sun_visibility::extract);
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
    assert!(render.get::<SunVisCamera>(render_camera).is_some(), "on by default");
    assert!(render.get::<SunVisCamera>(plain_render).is_none(), "no atmosphere: stock");

    main.resource_mut::<SunVisibility>().enabled = false;
    run(&mut main, &mut render);
    assert!(render.get::<SunVisCamera>(render_camera).is_none(), "sky.sun_vis.enabled off (--off sunvis)");

    main.resource_mut::<SunVisibility>().enabled = true;
    run(&mut main, &mut render);
    assert!(render.get::<SunVisCamera>(render_camera).is_some(), "back on");

    main.resource_mut::<ShaftsSun>().0 = None;
    run(&mut main, &mut render);
    assert!(render.get::<SunVisCamera>(render_camera).is_none(), "no sun");

    main.resource_mut::<ShaftsSun>().0 = Some(Vec3::Y);
    main.entity_mut(camera).remove::<AtmosphereSettings>();
    run(&mut main, &mut render);
    assert!(render.get::<SunVisCamera>(render_camera).is_none(), "the atmosphere removed");
}
