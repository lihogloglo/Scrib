//! FS-2d: every live feature `Switch` is a feature switch, kept in step with its tunable.

use super::*;
use bevy_ecs::system::SystemState;
use bevy_math::Vec4;
use sf_console_bevy::{ConsolePlugin, ConsoleSubmit};
use sf_tunables_bevy::FeatureSwitches;

/// The live switches that are no feature: pure dev visualisers and the panels.
const NOT_FEATURES: [Switch; 6] = [Switch::FarTint, Switch::TerrainLodColors, Switch::Wireframe, Switch::Rings, Switch::PerfPanel, Switch::FeaturesPanel];
/// `render.gi` is a console variable (`console.rs`): a stand-in under its name.
const GI: Def<bool> = Def::new(crate::console::GI_VAR, false);

/// Every feature `Switch`: its feature switch, effect and category (exact).
#[test]
fn every_feature_switch_is_registered_with_its_effect_and_category() {
    use Category::{Debug, Effects, Lighting, Shadows, Sky, Textures};
    use Effect::{Hides, Skips};
    let want: [(Switch, &str, Effect, Category); 21] = [
        (Switch::SunShadows, "shadow.sun.enabled", Skips, Shadows),
        (Switch::TerrainFarShadows, "shadow.terrain_far.enabled", Skips, Shadows),
        (Switch::Clouds, "sky.clouds.enabled", Skips, Sky),
        (Switch::TerrainSpecular, "terrain.specular.enabled", Hides, Debug),
        (Switch::TerrainEnvSpecular, "terrain.env_specular.enabled", Hides, Debug),
        (Switch::TerrainIndirect, "terrain.indirect.enabled", Hides, Debug),
        (Switch::TerrainTint, "terrain.tint.enabled", Hides, Debug),
        (Switch::Surface, "surface.enabled", Hides, Effects),
        (Switch::Fog, "sky.fog.enabled", Skips, Sky),
        (Switch::FogNightLight, "sky.fog.night_light.enabled", Hides, Sky),
        (Switch::WeatherAmbient, "sky.weather_ambient.enabled", Hides, Lighting),
        (Switch::AmbientSpecular, "render.ambient_specular.enabled", Hides, Lighting),
        (Switch::CloudHistory, "sky.clouds.history.enabled", Hides, Sky),
        (Switch::NightSkyGlow, "sky.night.glow.enabled", Hides, Sky),
        (Switch::Moonlight, "sky.moonlight.enabled", Hides, Lighting),
        (Switch::Cover, "surface.cover.enabled", Hides, Effects),
        (Switch::Trails, "surface.trails.enabled", Hides, Effects),
        (Switch::Raise, "surface.raise.enabled", Hides, Effects),
        (Switch::NormalMaps, "texture.normal_maps.enabled", Skips, Textures),
        (Switch::Orm, "texture.orm.enabled", Skips, Textures),
        (Switch::Parallax, "texture.parallax.enabled", Skips, Textures),
    ];
    for switch in Switch::ALL {
        assert_ne!(live_of(switch).is_some(), NOT_FEATURES.contains(&switch), "{}: a feature or a visualiser, not both", switch.name());
    }
    let mut app = App::new();
    register(&mut app);
    app.register_tunables(&[&GI]).add_feature_switch(&crate::console::GI_SWITCH);
    let switches = app.world().resource::<FeatureSwitches>();
    assert_eq!(LIVE.len(), want.len());
    for (live, (switch, name, effect, category)) in LIVE.iter().zip(want) {
        assert_eq!(live.switch, switch, "LIVE in Switch::ALL order");
        let registered = switches.get(name).unwrap_or_else(|| panic!("{name} registered"));
        assert_eq!((registered.effect_of(), registered.category_of()), (effect, category), "{name}");
        assert_eq!(live.def.name(), name);
        assert_eq!(live.def.meta().lifetime, Lifetime::Live, "{name}");
    }
    let gi = switches.get("render.gi").expect("GI switch");
    assert_eq!((gi.effect_of(), gi.category_of()), (Skips, Lighting));
    assert_eq!(switches.len(), want.len() + 1);
}

fn clouds() -> sf_atmos_bevy::Clouds {
    sf_atmos_bevy::Clouds {
        settings: sf_atmos_bevy::sf_atmos::CloudSettings::default(),
        enabled: true,
        shadows: false,
        glows: [Vec4::ZERO; sf_atmos_bevy::GLOWS],
        temporal: sf_atmos_bevy::sf_atmos::CloudTemporal::default(),
    }
}

/// The console and tunables, `LiveAb` and the live switches (no keys: they need a window).
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin, ConsolePlugin))
        .init_resource::<LiveAb>()
        .add_plugins(LiveSwitchesPlugin);
    app.update();
    app
}

fn value(app: &App, name: &str) -> (bool, SetBy) {
    let t = app.world().resource::<Tunables>();
    let id = t.id(name).expect("registered");
    (t.flag(id), t.var(id).set_by())
}

fn run(app: &mut App, line: &str) {
    app.world_mut().write_message(ConsoleSubmit::exec(line));
    app.update();
    app.update();
}

/// What an F-key does: `Switch::set` on the live resources.
fn key(app: &mut App, switch: Switch) {
    let mut state: SystemState<(ResMut<LiveAb>, Switches)> = SystemState::new(app.world_mut());
    let (mut ab, mut s) = state.get_mut(app.world_mut()).expect("params");
    let _ = switch.set(&mut ab, &mut s, None);
    state.apply(app.world_mut());
    app.update();
}

/// `toggle <tunable>` (the Features panel's path) applies the switch; an F-key flip writes the
/// tunable at the console layer, and the tunable brings it back.
#[test]
fn a_tunable_flip_applies_the_switch_and_a_key_flip_writes_the_console_layer() {
    let mut app = app();
    app.insert_resource(clouds());
    app.update();
    assert_eq!(value(&app, "sky.clouds.enabled"), (true, SetBy::Default), "in step: nothing written");

    run(&mut app, "toggle sky.clouds.enabled");
    assert_eq!(value(&app, "sky.clouds.enabled"), (false, SetBy::Console));
    assert!(!app.world().resource::<sf_atmos_bevy::Clouds>().enabled, "the tunable applied the switch");

    key(&mut app, Switch::Clouds);
    assert!(app.world().resource::<sf_atmos_bevy::Clouds>().enabled);
    assert_eq!(value(&app, "sky.clouds.enabled"), (true, SetBy::Console), "the key wrote the console layer");

    key(&mut app, Switch::SunShadows);
    assert_eq!(value(&app, "shadow.sun.enabled"), (false, SetBy::Console));
    run(&mut app, "toggle shadow.sun.enabled");
    let mut state: SystemState<(Res<LiveAb>, Switches)> = SystemState::new(app.world_mut());
    let (ab, s) = state.get_mut(app.world_mut()).expect("params");
    assert_eq!(Switch::SunShadows.get(&ab, &s), Some(true), "back on from the tunable");
    assert_eq!(value(&app, "shadow.sun.enabled"), (true, SetBy::Console));
}

/// A resource that appears (start, door travel) takes a value some layer chose; an untouched
/// default takes the game's start state at the code layer.
#[test]
fn an_appearing_resource_takes_the_chosen_value_or_seeds_the_tunable() {
    let mut app = app();
    run(&mut app, "toggle surface.enabled");
    assert_eq!(value(&app, "surface.enabled"), (false, SetBy::Console), "no surface yet: the value waits");
    app.init_resource::<sf_surface_bevy::SurfaceSettings>();
    app.update();
    assert!(!app.world().resource::<sf_surface_bevy::SurfaceSettings>().layer, "the console's off applies");

    app.insert_resource(mw_assets::MaterialMaps::new(false, true, true));
    app.update();
    assert_eq!(value(&app, "texture.normal_maps.enabled"), (false, SetBy::Code), "the game's start state");
    assert_eq!(value(&app, "texture.orm.enabled"), (true, SetBy::Default));
}
