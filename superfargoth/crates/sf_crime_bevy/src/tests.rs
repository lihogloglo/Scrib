//! `LW4b2` headless tests: the plugin installs the tunables' `StandardDetection`, a tunable change
//! swaps it, and the def defaults are the struct defaults.

use sf_crime::{DetectIn, Stance, VisibilityFactors};
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use super::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, CrimeStealthPlugin));
    app
}

/// A crouched, still target in full light, straight in front of the observer.
fn crouched_lit() -> DetectIn {
    DetectIn {
        factors: VisibilityFactors { light: 1.0, stance: Stance::Crouched, speed_mps: 0.0, ..VisibilityFactors::default() },
        ..DetectIn::default()
    }
}

fn installed_concealment(app: &App) -> f32 {
    app.world().resource::<AiDetectionRules>().0.concealment(&crouched_lit())
}

#[test]
fn the_plugin_installs_the_default_rules() {
    let mut app = app();
    app.update();
    let expected = StandardDetection::default().concealment(&crouched_lit());
    assert!((installed_concealment(&app) - expected).abs() < 1e-6);
    assert!((expected - 30.0).abs() < 1e-6, "crouched in full light = crouched_concealment");
}

#[test]
fn a_tunable_change_swaps_the_installed_rules() {
    let mut app = app();
    app.update();
    let before = app.world().resource::<AiDetectionRules>().0.clone();
    app.world_mut().write_message(SetTunable::new(CROUCHED_CONCEALMENT.name(), "60", SetBy::Console));
    app.update();
    let config = StandardDetectionConfig { crouched_concealment: 60.0, ..StandardDetectionConfig::default() };
    let expected = StandardDetection::new(config).concealment(&crouched_lit());
    assert!((installed_concealment(&app) - expected).abs() < 1e-6);
    assert!((expected - 60.0).abs() < 1e-6);
    assert_eq!(app.world().resource::<CrimeStealthSettings>().0, config);
    assert!(!Arc::ptr_eq(&before, &app.world().resource::<AiDetectionRules>().0));
    // An idle frame swaps nothing.
    let after = app.world().resource::<AiDetectionRules>().0.clone();
    app.update();
    assert!(Arc::ptr_eq(&after, &app.world().resource::<AiDetectionRules>().0));
}

#[test]
fn rules_installed_by_someone_else_are_kept() {
    let mut app = app();
    app.update();
    let theirs: Arc<dyn DetectionRules> = Arc::new(StandardDetection::new(StandardDetectionConfig {
        crouched_concealment: 5.0,
        ..StandardDetectionConfig::default()
    }));
    app.insert_resource(AiDetectionRules(theirs.clone()));
    app.world_mut().write_message(SetTunable::new(CROUCHED_CONCEALMENT.name(), "60", SetBy::Console));
    app.update();
    assert!(Arc::ptr_eq(&theirs, &app.world().resource::<AiDetectionRules>().0));
}

#[test]
fn def_defaults_are_the_struct_defaults() {
    let d = StandardDetectionConfig::default();
    let pairs = [
        (&LIGHT_CONCEALMENT, d.light_concealment),
        (&CROUCHED_CONCEALMENT, d.crouched_concealment),
        (&PRONE_CONCEALMENT, d.prone_concealment),
        (&SPEED_FREE_MPS, d.speed_free_mps),
        (&SPEED_REVEAL_PER_MPS, d.speed_reveal_per_mps),
        (&SPEED_REVEAL_MAX, d.speed_reveal_max),
        (&FACING_FRONT_RAD, d.facing_front_rad),
        (&BEHIND_CONCEALMENT, d.behind_concealment),
        (&MAX_CONCEALMENT, d.max_concealment),
    ];
    assert_eq!(pairs.len(), TUNABLES.len(), "one def per config field");
    for (def, field) in pairs {
        assert!((def.default_value() - field).abs() < f32::EPSILON, "{}", def.name());
    }
    // Defaults bound through the plugin give the struct back.
    let mut app = app();
    app.update();
    assert_eq!(app.world().resource::<CrimeStealthSettings>().0, d);
}
