//! The sun and moon shadow gate on the sky's lights (`sun_gate.rs`, ROADMAP 1.25 CW-8). Headless:
//! the render path resolves to raster, so the elevation rule's shadow maps are on by day.

#![allow(clippy::float_cmp, reason = "strengths are exact at the band's ends")]

use bevy::MinimalPlugins;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_light::atmosphere::ScatteringMedium;
use bevy_pbr::DirectionalLightShadowFade;
use sf_sky::sun_gate::SunShadowGate;
use sf_sky::{KeyCurve, SkySettings, Weather};

use crate::*;

/// A paused sky at `hour` of `day`, a sky camera, the gate as given (`None`: the plugin's
/// default, off), 100 ms frames.
fn gate_app(day: f64, hour: f64, settings: SkySettings, gate: Option<SunShadowGate>) -> App {
    let mut clock = GameClock::at(day, hour, settings.calendar.day_ms());
    clock.paused = true;
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<ScatteringMedium>()
        .add_plugins(SkyPlugin { settings, clock, render_path: RenderPath::RayTraced });
    app.finish();
    app.cleanup();
    if let Some(gate) = gate {
        app.insert_resource(SunShadowGating(gate));
    }
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100)));
    app.world_mut().spawn(SkyCamera::default());
    app
}

#[expect(clippy::unnecessary_wraps, reason = "reads as the `gate` argument of `gate_app`")]
fn on(fade_seconds: f32) -> Option<SunShadowGate> {
    Some(SunShadowGate { enabled: true, fade_seconds, ..SunShadowGate::default() })
}

/// The gate switched off (it is on by default since 2026-11-17).
#[expect(clippy::unnecessary_wraps, reason = "reads as the `gate` argument of `gate_app`")]
fn off() -> Option<SunShadowGate> {
    Some(SunShadowGate { enabled: false, ..SunShadowGate::default() })
}

/// `(shadow maps on, fade component)` of the light with marker `M`.
fn light<M: Component>(app: &mut App) -> (bool, Option<f32>) {
    let mut q = app.world_mut().query_filtered::<(&DirectionalLight, Option<&DirectionalLightShadowFade>), With<M>>();
    let (l, fade) = q.single(app.world()).expect("one light");
    (l.shadow_maps_enabled, fade.map(|f| f.0))
}

fn has_gate_state<M: Component>(app: &mut App) -> bool {
    let mut q = app.world_mut().query_filtered::<Has<ShadowGateState>, With<M>>();
    q.single(app.world()).expect("one light")
}

/// Switch off (the default): no gate component, maps exactly as the elevation rule says,
/// whatever the weather; stats stay zero.
#[test]
fn gate_off_is_todays_behaviour() {
    for weather in [Weather::default(), Weather { haze: 80.0, cloud_cover: 1.0, ..Weather::default() }] {
        let mut app = gate_app(172.0, 12.0, SkySettings::default(), off());
        app.world_mut().resource_mut::<WeatherState>().0 = weather;
        app.world_mut().resource_mut::<SunLightOcclusion>().0 = 0.0;
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(light::<SunLight>(&mut app), (true, None));
        assert!(!has_gate_state::<SunLight>(&mut app) && !has_gate_state::<MoonLight>(&mut app));
        assert_eq!(*app.world().resource::<SunShadowGateStats>(), SunShadowGateStats::default());
    }
}

/// Clear noon with the gate on: cascades on at full strength from the first frames.
#[test]
fn clear_noon_keeps_cascades() {
    let mut app = gate_app(172.0, 12.0, SkySettings::default(), on(3.0));
    for _ in 0..5 {
        app.update();
        let (maps, _) = light::<SunLight>(&mut app);
        assert!(maps);
    }
    assert_eq!(light::<SunLight>(&mut app), (true, Some(1.0)));
    let stats = *app.world().resource::<SunShadowGateStats>();
    assert!(stats.sun_contrast > 0.5 && stats.sun_strength == 1.0, "{stats:?}");
}

/// The sun's direct light drowned (a shadow texture that blocks it, heavy haze): the strength
/// fades out over the fade time, monotonically, the maps stay on until it reaches 0, then go
/// off; the sky clearing again brings them back, fading in. Live switch off: back to today.
#[test]
fn drowned_sun_fades_out_drops_its_maps_and_fades_back() {
    let mut app = gate_app(172.0, 12.0, SkySettings::default(), on(1.0));
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(light::<SunLight>(&mut app), (true, Some(1.0)));
    app.world_mut().resource_mut::<WeatherState>().0 = Weather { haze: 48.0, cloud_cover: 1.0, ..Weather::default() };
    app.world_mut().resource_mut::<SunLightOcclusion>().0 = 0.0;
    let mut previous = 1.0;
    let mut frames_on = 0;
    for _ in 0..20 {
        app.update();
        let (maps, fade) = light::<SunLight>(&mut app);
        let strength = app.world().resource::<SunShadowGateStats>().sun_strength;
        assert!(strength <= previous + 1e-6, "monotonic");
        assert_eq!(maps, strength > 0.0, "maps follow the strength");
        if maps {
            assert_eq!(fade, Some(strength));
            frames_on += 1;
        }
        previous = strength;
    }
    // 1 s at 100 ms frames: not in one frame, gone within the fade (+ a frame of lag).
    assert!((8..=11).contains(&frames_on), "faded over {frames_on} frames");
    assert!(!light::<SunLight>(&mut app).0);

    app.world_mut().resource_mut::<WeatherState>().0 = Weather::default();
    app.world_mut().resource_mut::<SunLightOcclusion>().0 = 1.0;
    let mut strengths = Vec::new();
    for _ in 0..15 {
        app.update();
        strengths.push(app.world().resource::<SunShadowGateStats>().sun_strength);
    }
    let first_on = strengths.iter().position(|s| *s > 0.0).expect("back on");
    assert!(strengths[first_on] <= 0.2, "fades in, no pop: {strengths:?}");
    assert!(strengths.windows(2).all(|p| p[1] >= p[0]));
    assert_eq!(light::<SunLight>(&mut app), (true, Some(1.0)));

    // Live switch off: components gone, maps per the elevation rule.
    app.world_mut().resource_mut::<SunShadowGating>().0.enabled = false;
    app.world_mut().resource_mut::<WeatherState>().0 = Weather { haze: 48.0, cloud_cover: 1.0, ..Weather::default() };
    app.world_mut().resource_mut::<SunLightOcclusion>().0 = 0.0;
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(light::<SunLight>(&mut app), (true, None));
    assert!(!has_gate_state::<SunLight>(&mut app));
}

/// Night, full moon: a bright ambient floor (10 nits, ~31 lux) drowns the ~1 lux moonlight; the gate gives
/// the moon no cascades, while the elevation rule alone does.
#[test]
fn dim_moon_at_night_gets_no_cascades() {
    let settings = SkySettings { ambient: KeyCurve::new(vec![(-12.0, 10.0), (-4.0, 0.0)]), ..SkySettings::default() };
    // Day 162 is a full moon; at 0:30 the sun is well below the horizon (`tests.rs`).
    let mut plain = gate_app(162.0, 0.5, settings.clone(), off());
    plain.update();
    assert!(light::<MoonLight>(&mut plain).0, "the elevation rule casts");
    let mut gated = gate_app(162.0, 0.5, settings, on(3.0));
    for _ in 0..3 {
        gated.update();
    }
    let stats = *gated.world().resource::<SunShadowGateStats>();
    assert!(stats.moon_contrast < 0.05, "{stats:?}");
    assert!(!light::<MoonLight>(&mut gated).0);
    assert!(!light::<SunLight>(&mut gated).0);
}
