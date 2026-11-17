//! E1b (`docs/specs/exposure-tonemap.md`; vendored `bevy_pbr` patch "own, E1b: directional
//! atmosphere scatter"): a directional light's atmosphere scatter scale reaches the render world
//! (absent = 1, clamped to 0..1, re-extracted when it changes), and `sky.moon.sky_scatter` drives
//! the moon's, live. No GPU: the shader side is `shader_tests.rs`.

use bevy::MinimalPlugins;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy_app::App;
use bevy_camera::visibility::ViewVisibility;
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_light::atmosphere::ScatteringMedium;
use bevy_pbr::{DirectionalLightAtmosphereScatter, ExtractedDirectionalLight, atmosphere_scatter_ratio};
use bevy_transform::components::GlobalTransform;
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use crate::lamp_cache_render_tests::RenderExtract;
use crate::*;

#[test]
#[expect(clippy::float_cmp, reason = "exact: clamps and the stock value")]
fn the_ratio_is_one_when_absent_and_clamped() {
    let ratio = |s: f32| atmosphere_scatter_ratio(Some(&DirectionalLightAtmosphereScatter(s)));
    assert_eq!(atmosphere_scatter_ratio(None), 1.0);
    assert_eq!(DirectionalLightAtmosphereScatter::default().0, 1.0);
    assert_eq!(ratio(0.15), 0.15);
    assert_eq!(ratio(0.0), 0.0);
    assert_eq!(ratio(-1.0), 0.0);
    assert_eq!(ratio(2.0), 1.0);
    assert_eq!(ratio(f32::NAN), 1.0);
}

/// `extract_lights` carries the component onto `ExtractedDirectionalLight` (the GPU light copies
/// that field in `prepare_lights`), and a change alone re-extracts the light.
#[test]
#[expect(clippy::float_cmp, reason = "exact: copied values")]
fn extraction_carries_the_scatter_scale() {
    let mut main = World::new();
    let stock = main.spawn((DirectionalLight::default(), GlobalTransform::IDENTITY, ViewVisibility::VISIBLE)).id();
    let moon = main
        .spawn((
            DirectionalLight::default(),
            GlobalTransform::IDENTITY,
            ViewVisibility::VISIBLE,
            DirectionalLightAtmosphereScatter(0.15),
        ))
        .id();
    let mut extract = RenderExtract::new();
    let render_stock = extract.link(&mut main, stock);
    let render_moon = extract.link(&mut main, moon);
    let scatter = |extract: &RenderExtract, e: Entity| extract.world().get::<ExtractedDirectionalLight>(e).expect("extracted").atmosphere_scatter;

    extract.extract(&mut main);
    assert_eq!(scatter(&extract, render_stock), 1.0, "absent = stock");
    assert_eq!(scatter(&extract, render_moon), 0.15);

    main.get_mut::<DirectionalLightAtmosphereScatter>(moon).expect("scatter").0 = 3.0;
    extract.extract(&mut main);
    assert_eq!(scatter(&extract, render_moon), 1.0, "clamped, and re-extracted on its own change");
}

#[test]
#[expect(clippy::float_cmp, reason = "exact: the tunable's value")]
fn the_moon_scatter_follows_its_tunable() {
    let settings = SkySettings::default();
    let mut clock = GameClock::at(172.0, 0.5, settings.calendar.day_ms());
    clock.paused = true;
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), TunablesPlugin))
        .init_asset::<ScatteringMedium>()
        .add_plugins(SkyPlugin { settings, clock, render_path: RenderPath::RayTraced });
    app.finish();
    app.cleanup();
    app.update();
    let moon = |app: &mut App| {
        let mut q = app.world_mut().query_filtered::<&DirectionalLightAtmosphereScatter, With<MoonLight>>();
        q.single(app.world()).expect("one moon").0
    };
    assert_eq!(moon(&mut app), MOON_SKY_SCATTER.default_value());
    let mut suns = app.world_mut().query_filtered::<(), (With<SunLight>, With<DirectionalLightAtmosphereScatter>)>();
    assert_eq!(suns.iter(app.world()).count(), 0, "the sun scatters fully (stock)");

    app.world_mut().write_message(SetTunable::new("sky.moon.sky_scatter", "0.4", SetBy::Console));
    app.update();
    assert_eq!(moon(&mut app), 0.4);
    app.world_mut().write_message(SetTunable::new("sky.moon.sky_scatter", "1", SetBy::Console));
    app.update();
    assert_eq!(moon(&mut app), 1.0);
}
