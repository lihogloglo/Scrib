//! The point-shadow budget's contrast gate (`ShadowContrastGate`, `shadow_budget.rs`): lights whose
//! shadow would be imperceptible against the sky's light get no shadow map.

use bevy::MinimalPlugins;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy_app::App;
use bevy_camera::Camera3d;
use bevy_ecs::prelude::*;
use bevy_light::atmosphere::ScatteringMedium;
use bevy_transform::components::GlobalTransform;
use sf_sky::{SkyClock, SkySettings, SkyState, Weather};

use crate::shadow_budget::{DAY_GATE, DAY_GATE_CONTRAST};
use crate::*;

/// A sky at `hour` of a summer day (paused), a budget, a viewer at the origin (outdoors: a
/// `SkyCamera`, or not) and four 1000 lm lights at 2, 5, 8 and 30 m.
fn gate_app(hour: f64, budget: PointShadowBudget, outdoors: bool) -> (App, Vec<Entity>) {
    let settings = SkySettings::default();
    let mut clock = GameClock::at(172.0, hour, settings.calendar.day_ms());
    clock.paused = true;
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<ScatteringMedium>()
        .add_plugins(SkyPlugin { settings, clock, render_path: RenderPath::RayTraced });
    app.finish();
    app.cleanup();
    app.insert_resource(budget);
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100)));
    let camera = app.world_mut().spawn((Camera3d::default(), GlobalTransform::IDENTITY)).id();
    if outdoors {
        app.world_mut().entity_mut(camera).insert(SkyCamera::default());
    }
    let lights = [2.0, 5.0, 8.0, 30.0]
        .map(|z| {
            app.world_mut()
                .spawn((
                    bevy_light::PointLight { intensity: 1000.0, ..Default::default() },
                    BudgetedShadow::default(),
                    NoLightProxy,
                    GlobalTransform::from_xyz(0.0, 0.0, z),
                    bevy_camera::visibility::InheritedVisibility::VISIBLE,
                    bevy_camera::visibility::ViewVisibility::VISIBLE,
                ))
                .id()
        })
        .to_vec();
    (app, lights)
}

/// `(shadow map on, shadow fade)` per light.
fn state(app: &App, lights: &[Entity]) -> Vec<(bool, f32)> {
    lights
        .iter()
        .map(|e| {
            let on = app.world().get::<bevy_light::PointLight>(*e).expect("light").shadow_maps_enabled;
            (on, app.world().get::<ShadowSlot>(*e).map_or(0.0, |s| s.fade))
        })
        .collect()
}

fn run(hour: f64, gate: bool, outdoors: bool) -> Vec<(bool, f32)> {
    let contrast_gate = ShadowContrastGate { enabled: gate, ..ShadowContrastGate::default() };
    let budget = PointShadowBudget { max: 2, fade_time: 0.0, contrast_gate, ..PointShadowBudget::default() };
    let (mut app, lights) = gate_app(hour, budget, outdoors);
    for _ in 0..4 {
        app.update();
    }
    state(&app, &lights)
}

fn sky_at(hour: f64) -> SkyState {
    let settings = SkySettings::default();
    settings.evaluate(&SkyClock::at(172.0, hour, &settings.calendar), &Weather::default())
}

/// The gate's weight of one of [`gate_app`]'s lights (not holding a slot) at `hour`.
fn weight_at(hour: f64) -> f32 {
    let gate = ShadowContrastGate::default();
    let sky = sky_at(hour);
    let lux = gate.receiver_lux(1000.0, bevy_light::PointLight::default().range);
    gate.weight(ShadowContrastGate::contrast(lux, sky_reference_lux(&sky, sky.ambient)), false, 0.8)
}

#[test]
fn gate_is_on_by_default() {
    assert!(PointShadowBudget::default().contrast_gate.enabled);
    assert!(DAY_GATE.default_value(), "on by default (owner 2026-11-17)");
    let [off, full] = DAY_GATE_CONTRAST.default_value();
    assert!(0.0 < off && off < full && full < 1.0);
    let mut t = sf_tunables::Tunables::default();
    t.register_all(SHADOW_BUDGET_TUNABLES).expect("the gate's tunables register");
}

/// Weight 0 at or below the off contrast, 1 at or above the full one, linear between; a holder's
/// contrast counts `1 / hysteresis` times, so it keeps its map a little below the threshold.
#[test]
fn gate_weight_band_and_hysteresis() {
    let gate = ShadowContrastGate { enabled: true, receiver_distance: 2.0, contrast_off: 0.05, contrast_full: 0.15 };
    assert!(gate.weight(0.05, false, 0.8).abs() < 1e-6);
    assert!((gate.weight(0.10, false, 0.8) - 0.5).abs() < 1e-5);
    assert!((gate.weight(0.15, false, 0.8) - 1.0).abs() < 1e-6);
    assert!((gate.weight(0.9, true, 0.8) - 1.0).abs() < 1e-6);
    // 0.045: below the threshold for a newcomer, above it (0.056) for a holder.
    assert!(gate.weight(0.045, false, 0.8).abs() < 1e-6);
    assert!(gate.weight(0.045, true, 0.8) > 0.0);
    // Contrast: the light's share; 1 in the dark.
    assert!((ShadowContrastGate::contrast(1.0, 3.0) - 0.25).abs() < 1e-6);
    assert!((ShadowContrastGate::contrast(0.0, 0.0) - 1.0).abs() < 1e-6);
    // Receiver illuminance: Φ / 4πd² inside the range window, and never 0 for a short light
    // (the receiver moves in to half its range).
    let lux = gate.receiver_lux(4.0 * std::f32::consts::PI * 400.0, 100.0);
    assert!((lux - 100.0).abs() < 0.1, "{lux}");
    assert!(gate.receiver_lux(1000.0, 1.0) > 0.0);
}

/// Noon outdoors: the sun drowns 1000 lm lights, none holds a map (the ungated budget shadows two).
#[test]
fn noon_outdoors_shadows_no_light() {
    let gated = run(12.0, true, true);
    assert!(gated.iter().all(|(on, fade)| !on && *fade == 0.0), "{gated:?}");
    let base = run(12.0, false, true);
    assert_eq!(base.iter().filter(|(on, _)| *on).count(), 2, "{base:?}");
}

/// Night: the gate changes nothing (same maps, same fades).
#[test]
fn night_selection_is_unchanged() {
    assert!((weight_at(0.0) - 1.0).abs() < 1e-6);
    let gated = run(0.0, true, true);
    assert_eq!(gated, run(0.0, false, true));
    assert_eq!(gated.iter().filter(|(on, _)| *on).count(), 2);
}

/// Interiors (no `SkyCamera` viewer) have no sun: the gate stays out, even at noon.
#[test]
fn interior_is_unchanged() {
    let gated = run(12.0, true, false);
    assert_eq!(gated, run(12.0, false, false));
    assert_eq!(gated.iter().filter(|(on, _)| *on).count(), 2);
}

/// Dusk: as the sun sets the weight rises from 0 to 1 continuously (sampled every 10 game
/// seconds, a third of a real second at a 30x clock, no step above 0.05), never back down; in the
/// band a light holds a map at part strength. The direct sun drops fast at the horizon, so the
/// band is a few game minutes wide (printed); the budget's timed fade smooths it further.
#[test]
fn dusk_fades_smoothly() {
    let (mut prev, mut biggest, mut band) = (weight_at(16.0), 0.0f32, None);
    let (mut first, mut last) = (None, None);
    let mut seconds = 16.0 * 3600.0;
    while seconds <= 22.0 * 3600.0 {
        let hour = seconds / 3600.0;
        let w = weight_at(hour);
        assert!(w + 1e-6 >= prev, "weight fell at {hour:.3} h: {prev} -> {w}");
        biggest = biggest.max(w - prev);
        if band.is_none() && w > 0.2 && w < 0.8 {
            band = Some(hour);
        }
        if w > 0.0 {
            first.get_or_insert(hour);
        }
        if w < 1.0 {
            last = Some(hour);
        }
        prev = w;
        seconds += 10.0;
    }
    assert!(weight_at(16.0) < 1e-6 && (prev - 1.0).abs() < 1e-6, "noon side 0, night side 1");
    assert!(biggest < 0.05, "largest step per 10 game seconds {biggest}");
    eprintln!("dusk fade band: {:.3} h to {:.3} h", first.unwrap_or(0.0), last.unwrap_or(0.0));
    let hour = band.expect("a dusk minute in the fade band");
    let lights = run(hour, true, true);
    let (on, fade) = lights[0];
    assert!(on && fade > 0.0 && fade < 1.0, "at {hour:.2} h: {lights:?}");
}
