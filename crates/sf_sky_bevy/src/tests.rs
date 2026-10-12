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

#[test]
fn a_fading_shadow_splits_the_light_without_changing_its_power() {
    let budget = PointShadowBudget { max: 1, fade_time: 1.0, ..PointShadowBudget::default() };
    let (mut app, lights) = budget_app(budget);
    for _ in 0..3 {
        app.update();
    }
    let (on, part) = shadowed(&app, lights[0]);
    let slot = *app.world().get::<ShadowSlot>(lights[0]).expect("slot");
    assert!(on && slot.held && slot.fade > 0.0 && slot.fade < 1.0);
    let mut twins = app.world_mut().query_filtered::<(&bevy_light::PointLight, &bevy_camera::visibility::Visibility), With<ShadowTwin>>();
    let rest: f32 = twins
        .iter(app.world())
        .filter(|(_, v)| **v != bevy_camera::visibility::Visibility::Hidden)
        .map(|(l, _)| {
            assert!(!l.shadow_maps_enabled);
            l.intensity
        })
        .sum();
    assert!((part + rest - 1000.0).abs() < 1e-2, "{part} + {rest}");
    // Only one map at a time: the second light waits.
    assert!(!shadowed(&app, lights[1]).0);
}
