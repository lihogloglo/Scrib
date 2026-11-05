//! Headless plugin tests (no renderer: the render path resolves to raster).

use bevy::MinimalPlugins;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy_app::App;
use bevy_camera::Exposure;
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_light::atmosphere::ScatteringMedium;
use bevy_pbr::AtmosphereSettings;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::*;

fn app(hour: f64) -> App {
    let settings = SkySettings::default();
    let mut clock = SkyClock::at(172.0, hour, &settings.calendar);
    clock.paused = true;
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<ScatteringMedium>()
        .add_plugins(SkyPlugin {
            settings,
            clock,
            render_path: RenderPath::RayTraced,
        });
    app.finish();
    app.cleanup();
    app
}

fn sun(app: &mut App) -> (DirectionalLight, Transform) {
    let mut q = app
        .world_mut()
        .query_filtered::<(&DirectionalLight, &Transform), With<SunLight>>();
    let (light, transform) = q.single(app.world()).expect("one sun");
    (*light, *transform)
}

#[test]
fn falls_back_to_raster_without_a_renderer() {
    let mut app = app(12.0);
    app.update();
    assert!(!app.world().resource::<RayTracingSupport>().0);
    assert_eq!(
        app.world().resource::<ActiveRenderPath>().0,
        RenderPath::Raster
    );
}

#[test]
fn drives_sun_and_camera_from_the_clock() {
    let mut app = app(12.0);
    let camera = app.world_mut().spawn(SkyCamera::default()).id();
    app.update();
    app.update();

    let sky = app.world().resource::<CurrentSky>().0;
    assert!(sky.sun.elevation > 1.0);
    let (light, transform) = sun(&mut app);
    // Raster: unattenuated sun with shadows, shining along −direction.
    assert!((light.illuminance - sky.sun.illuminance).abs() < 1e-3);
    assert!(light.shadow_maps_enabled);
    assert!((transform.forward().as_vec3() + sky.sun.direction).length() < 1e-4);

    let world = app.world();
    assert!(world.get::<AtmosphereSettings>(camera).is_some());
    let ev = world.get::<Exposure>(camera).expect("exposure").ev100;
    assert!((ev - sky.ev100).abs() < 1e-4);

    // Leaving the sky (an interior): atmosphere off, sun dark.
    app.world_mut().entity_mut(camera).remove::<SkyCamera>();
    app.update();
    assert!(app.world().get::<AtmosphereSettings>(camera).is_none());
    let (light, _) = sun(&mut app);
    assert!(light.illuminance <= 0.0 && !light.shadow_maps_enabled);
}

#[test]
fn moon_takes_over_shadows_at_night() {
    // Day 162 is a full moon; at 0:30 the sun is well below the horizon.
    let mut app = app(0.5);
    app.world_mut().resource_mut::<SkyTime>().0 =
        SkyClock::at(162.0, 0.5, &SkySettings::default().calendar);
    app.world_mut().spawn(SkyCamera::default());
    app.update();
    let (sun_light, _) = sun(&mut app);
    assert!(!sun_light.shadow_maps_enabled);
    let mut q = app
        .world_mut()
        .query_filtered::<&DirectionalLight, With<MoonLight>>();
    let moon = q.single(app.world()).expect("one moon");
    assert!(moon.shadow_maps_enabled && moon.illuminance > 0.0);
}

fn budget_app(budget: PointShadowBudget) -> (App, Vec<Entity>) {
    let mut app = app(0.0);
    app.insert_resource(budget);
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100)));
    app.world_mut().spawn((Camera3d::default(), GlobalTransform::IDENTITY));
    let lights = [2.0, 5.0, 8.0, 30.0]
        .map(|z| {
            app.world_mut()
                .spawn((
                    bevy_light::PointLight { intensity: 1000.0, ..Default::default() },
                    BudgetedShadow::default(),
                    NoLightProxy,
                    GlobalTransform::from_xyz(0.0, 0.0, z),
                    // No visibility propagation in these tests.
                    bevy_camera::visibility::InheritedVisibility::VISIBLE,
                    // Nor visibility checks: every light is in view unless a test hides it.
                    bevy_camera::visibility::ViewVisibility::VISIBLE,
                ))
                .id()
        })
        .to_vec();
    (app, lights)
}

fn shadowed(app: &App, e: Entity) -> (bool, f32) {
    let l = app.world().get::<bevy_light::PointLight>(e).expect("light");
    (l.shadow_maps_enabled, l.intensity)
}

#[test]
fn point_shadow_budget_shadows_the_nearest_within_range() {
    let budget = PointShadowBudget { max: 2, fade_time: 0.0, ..PointShadowBudget::default() };
    let (mut app, lights) = budget_app(budget);
    app.update();
    app.update();
    let on: Vec<bool> = lights.iter().map(|e| shadowed(&app, *e).0).collect();
    assert_eq!(on, [true, true, false, false]);
    // The light past `fade_end` never shadows, even with a free slot.
    app.world_mut().resource_mut::<PointShadowBudget>().max = 4;
    app.update();
    app.update();
    let on: Vec<bool> = lights.iter().map(|e| shadowed(&app, *e).0).collect();
    assert_eq!(on, [true, true, true, false]);
    // Budget 0: nothing shadows.
    app.world_mut().resource_mut::<PointShadowBudget>().max = 0;
    app.update();
    assert!(lights.iter().all(|e| !shadowed(&app, *e).0));
}

/// A light no view can see (out of every frustum, or on render layers no
/// camera draws: another worldspace) never takes a slot, however near; a
/// holder that stops being visible fades out and frees its slot.
#[test]
fn point_shadow_budget_skips_lights_no_view_sees() {
    use bevy_camera::visibility::ViewVisibility;
    let budget = PointShadowBudget { max: 1, fade_time: 0.0, ..PointShadowBudget::default() };
    let (mut app, lights) = budget_app(budget);
    app.world_mut().entity_mut(lights[0]).insert(ViewVisibility::HIDDEN);
    app.update();
    app.update();
    let on: Vec<bool> = lights.iter().map(|e| shadowed(&app, *e).0).collect();
    assert_eq!(on, [false, true, false, false]);
    // The holder leaves every view: its slot goes to the next visible light.
    app.world_mut().entity_mut(lights[1]).insert(ViewVisibility::HIDDEN);
    app.update();
    app.update();
    let on: Vec<bool> = lights.iter().map(|e| shadowed(&app, *e).0).collect();
    assert_eq!(on, [false, false, true, false]);
    assert!(!app.world().get::<ShadowSlot>(lights[1]).expect("slot").held);
}

#[test]
fn a_fading_shadow_splits_the_light_without_changing_its_power() {
    let budget = PointShadowBudget { max: 1, fade_time: 1.0, ..PointShadowBudget::default() };
    let (mut app, lights) = budget_app(budget);
    for _ in 0..3 {
        app.update();
    }
    let (on, intensity) = shadowed(&app, lights[0]);
    let slot = *app.world().get::<ShadowSlot>(lights[0]).expect("slot");
    let fade = app
        .world()
        .get::<bevy_pbr::PointLightShadowFade>(lights[0])
        .expect("shadow fade");
    assert!(on && slot.held && slot.fade > 0.0 && slot.fade < 1.0);
    assert!((fade.0 - slot.fade).abs() < 1e-5);
    assert!((intensity - 1000.0).abs() < 1e-2, "{intensity}");
    // Only one map at a time: the second light waits.
    assert!(!shadowed(&app, lights[1]).0);
}

/// A light with a [`ShadowReach`] has its short range without a slot, its
/// long range at full shadow, and the shadow fade's blend in between, both
/// ways (spec lights.md "Reach follows shadow state"). A light without one
/// keeps the range it was given.
#[test]
fn reach_follows_the_shadow_fade() {
    let reach = ShadowReach { unshadowed: 3.0, shadowed: 8.0 };
    assert!((reach.range(0.0) - 3.0).abs() < 1e-6 && (reach.range(1.0) - 8.0).abs() < 1e-6);
    assert!((reach.range(0.25) - 4.25).abs() < 1e-6 && (reach.range(2.0) - 8.0).abs() < 1e-6);

    let budget = PointShadowBudget { max: 1, fade_time: 1.0, ..PointShadowBudget::default() };
    let (mut app, lights) = budget_app(budget);
    for e in &lights[..3] {
        app.world_mut().entity_mut(*e).insert(reach);
    }
    let range = |app: &App, e: Entity| app.world().get::<bevy_light::PointLight>(e).expect("light").range;
    let fade = |app: &App, e: Entity| app.world().get::<ShadowSlot>(e).expect("slot").fade;
    let untouched = bevy_light::PointLight::default().range;

    // Fading in: the holder's range is the blend at its shadow fade.
    for _ in 0..3 {
        app.update();
    }
    let f = fade(&app, lights[0]);
    assert!(f > 0.0 && f < 1.0, "{f}");
    let r = range(&app, lights[0]);
    assert!((r - reach.range(f)).abs() < 1e-4 && r > 3.0 && r < 8.0, "{r} at fade {f}");
    // No slot: the short reach; no `ShadowReach`: unchanged.
    assert!((range(&app, lights[1]) - 3.0).abs() < 1e-4);
    assert!((range(&app, lights[2]) - 3.0).abs() < 1e-4);
    assert!((range(&app, lights[3]) - untouched).abs() < 1e-6);

    // Full shadow: the long reach.
    for _ in 0..15 {
        app.update();
    }
    assert!((fade(&app, lights[0]) - 1.0).abs() < 1e-6);
    assert!((range(&app, lights[0]) - 8.0).abs() < 1e-4);

    // Losing the slot (budget 0): back down with the fade-out, then short.
    app.world_mut().resource_mut::<PointShadowBudget>().max = 0;
    for _ in 0..3 {
        app.update();
    }
    let f = fade(&app, lights[0]);
    assert!(f > 0.0 && f < 1.0, "{f}");
    assert!((range(&app, lights[0]) - reach.range(f)).abs() < 1e-4);
    for _ in 0..15 {
        app.update();
    }
    assert!(!app.world().get::<ShadowSlot>(lights[0]).expect("slot").held);
    assert!((range(&app, lights[0]) - 3.0).abs() < 1e-4);
    assert!((range(&app, lights[3]) - untouched).abs() < 1e-6);
}

#[test]
fn medium_rebuilds_on_weather_steps_rate_limited() {
    let mut app = app(12.0);
    app.update();
    let rebuilds = |app: &App| app.world().resource::<MediumStats>().rebuilds;
    let first = rebuilds(&app);
    assert_eq!(first, 1, "built once at startup");
    // Below a step: no rebuild.
    app.world_mut().resource_mut::<WeatherState>().0.haze += 0.01;
    app.update();
    assert_eq!(rebuilds(&app), first);
    // A dust storm arrives, but the last build is too recent.
    app.world_mut().resource_mut::<SkyRenderOptions>().medium_min_interval = 1.0e6;
    app.world_mut().resource_mut::<WeatherState>().0.dust = 4.0;
    app.update();
    assert_eq!(rebuilds(&app), first);
    // Once the interval allows it, one rebuild with the new dust.
    app.world_mut().resource_mut::<SkyRenderOptions>().medium_min_interval = 0.0;
    app.update();
    app.update();
    assert_eq!(rebuilds(&app), first + 1);
    let stats = *app.world().resource::<MediumStats>();
    assert!((stats.built.dust - 4.0).abs() < 1e-6);
    // The dust term is in the GPU medium only while there is dust.
    let terms = |app: &App| {
        let handle = app.world().resource::<MediumState>().handle.clone();
        app.world().resource::<Assets<ScatteringMedium>>().get(&handle).expect("medium").terms.len()
    };
    assert_eq!(terms(&app), 4);
    app.world_mut().resource_mut::<WeatherState>().0.dust = 0.0;
    app.update();
    assert_eq!(terms(&app), 3);
}

/// The CPU half of a medium rebuild: the LUT bake Bevy's
/// `GpuScatteringMedium::prepare_asset` runs on the render thread (same
/// loops, public sampling API), for the default 256² resolution.
/// `cargo test --release -p sf_sky_bevy medium_rebuild_cost -- --ignored --nocapture`
#[test]
#[ignore = "timing; run in release"]
#[expect(clippy::cast_precision_loss, reason = "LUT indices")]
fn medium_rebuild_cost() {
    use bevy_color::ColorToComponents;
    use bevy_math::Vec4;
    let model = sf_sky::AtmosphereModel::earth();
    for (label, scales) in [
        ("clear", MediumScales::haze(20.0)),
        ("dust", MediumScales { dust: 4.0, ..MediumScales::haze(24.0) }),
    ] {
        let runs = 20;
        let start = std::time::Instant::now();
        let mut sink = 0.0;
        for _ in 0..runs {
            let medium = build_medium(&model, &scales, 256);
            let (nf, np) = (medium.falloff_resolution, medium.phase_resolution);
            let mut density = Vec4::ZERO;
            for i in 0..nf {
                let f = (i as f32 + 0.5) / nf as f32;
                for t in &medium.terms {
                    density += (t.absorption + t.scattering).extend(0.0) * t.falloff.sample(f);
                }
            }
            let mut scattering = Vec4::ZERO;
            for raw in 0..nf * np {
                let f = ((raw % np) as f32 + 0.5) / nf as f32;
                let s = 2.0 * ((raw / np) as f32 + 0.5) / np as f32 - 1.0;
                let neg_l_dot_v = s.signum() * (1.0 - (1.0 - s.abs()).powf(2.0));
                for t in &medium.terms {
                    if let Some(p) = t.phase.sample(neg_l_dot_v) {
                        scattering += (t.scattering * p.to_vec3() * t.falloff.sample(f)).extend(0.0);
                    }
                }
            }
            sink += density.x + scattering.x;
        }
        let ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(runs);
        println!("medium rebuild CPU ({label}, {} terms): {ms:.2} ms (sink {sink:e})", build_medium(&model, &scales, 256).terms.len());
    }
}

#[test]
fn practical_split_cascades_and_texel_sizes() {
    use crate::{cascade_texel_sizes, CascadeSplits};
    let splits = CascadeSplits {
        num_cascades: 4,
        minimum_distance: 0.1,
        split_near: 1.0,
        maximum_distance: 250.0,
        lambda: 0.9,
        overlap_proportion: 0.2,
    };
    let config = splits.build();
    assert_eq!(config.bounds.len(), 4);
    assert!((config.bounds[3] - 250.0).abs() < 1e-3);
    assert!(config.bounds.windows(2).all(|w| w[0] < w[1]));
    // Lambda 1 is the pure log split, 0 the uniform one.
    let log = CascadeSplits { lambda: 1.0, ..splits }.bounds();
    assert!((log[1] - 250.0_f32.sqrt()).abs() < 1e-2);
    let uniform = CascadeSplits { lambda: 0.0, ..splits }.bounds();
    assert!((uniform[1] - 125.5).abs() < 1e-3);
    // Texel sizes at 70 deg vertical FOV, 16:9, 2048² (printed for the spec).
    let texels = cascade_texel_sizes(&config, 70f32.to_radians(), 16.0 / 9.0, 2048);
    let old = bevy_light::CascadeShadowConfigBuilder {
        num_cascades: 4,
        minimum_distance: 0.1,
        first_cascade_far_bound: 10.0,
        maximum_distance: 504.0,
        overlap_proportion: 0.2,
    }
    .build();
    let old_texels = cascade_texel_sizes(&old, 70f32.to_radians(), 16.0 / 9.0, 2048);
    println!("new bounds {:?} texels {texels:?}", config.bounds);
    println!("old bounds {:?} texels {old_texels:?}", old.bounds);
    assert!(texels[3] < old_texels[3] * 0.6);
}

#[test]
fn shadow_importance_prefers_strong_near_lights() {
    // Same light: nearer ranks higher; inside 1 m no further gain.
    assert!(shadow_importance(8.0, 3.0, 1.0) > shadow_importance(8.0, 6.0, 1.0));
    assert!((shadow_importance(8.0, 0.5, 1.0) - shadow_importance(8.0, 1.0, 1.0)).abs() < 1e-6);
    // A brazier (range 16 m) at 10 m outranks a candle (range 2 m) at 3 m.
    assert!(shadow_importance(16.0, 10.0, 1.0) > shadow_importance(2.0, 3.0, 1.0));
    // Out of range it falls off by coverage too (continuous at d = range).
    assert!((shadow_importance(4.0, 4.0, 1.0) - 1.0).abs() < 1e-6);
    assert!(shadow_importance(4.0, 8.0, 1.0) < shadow_importance(4.0, 4.0, 1.0) / 4.0 + 1e-6);
    // Priority 4 ranks like half the distance (inside range).
    assert!((shadow_importance(20.0, 8.0, 4.0) - shadow_importance(20.0, 4.0, 1.0)).abs() < 1e-3);
}

#[test]
fn ray_tracing_follows_the_cameras_worldspace_layer() {
    use bevy_camera::visibility::RenderLayers;
    use crate::path::{camera_layers, traced_by};
    // A camera in worldspace layer 2 (plus the shared layer 0).
    let interior = RenderLayers::from_layers(&[0, 2]);
    let view = camera_layers([Some(&interior)].into_iter());
    assert!(traced_by(&view, Some(&RenderLayers::layer(2))));
    assert!(!traced_by(&view, Some(&RenderLayers::layer(1))), "another worldspace stays out");
    assert!(traced_by(&view, None), "unlayered meshes are layer 0");
    // No camera: nothing is traced.
    assert!(!traced_by(&camera_layers(std::iter::empty()), None));
    // A camera without layers sees layer 0 only.
    let default_view = camera_layers([None].into_iter());
    assert!(traced_by(&default_view, None) && !traced_by(&default_view, Some(&RenderLayers::layer(2))));
}
