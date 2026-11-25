//! Headless tests for sight visibility from crime's rules (slice `LW4b1`): upright, still and in
//! front gains exactly as the light-only sight did, a crouched target in half light gains less, a
//! custom rule of 100 concealment hides the target, a game's rules survive the plugin. App setup shared with `sight_tests`.

// Exact assertions on purpose (same inputs, same maths, same bits).
#![allow(clippy::float_cmp)]

use std::sync::Arc;

use glam::Vec3;
use sf_ai::{Stance, apply_darkness};
use sf_crime::{DetectIn, DetectionRules};

use super::*;
use crate::sight_tests::{Walls, app, every_tick, expected_gain_seen, level, observer, step, target};

const OBS_AT: Vec3 = Vec3::ZERO;
const TGT_AT: Vec3 = Vec3::new(0.0, 0.0, 12.0);
const TICKS: usize = 40;

/// Interior ambient `sky`, no source: every target's exposure is `sky`.
fn half_light(app: &mut App) {
    app.insert_resource(AiLightInputs { sky: 0.5, interior: true, sources: Vec::new() });
}

/// The observer's awareness level per tick over [`TICKS`] ticks, with the target's look.
fn levels(look: Option<AiVisibilityFactors>, rules: Option<AiDetectionRules>) -> Vec<Option<f32>> {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    half_light(&mut app);
    if let Some(r) = rules {
        app.insert_resource(r);
    }
    let obs = observer(&mut app, 7, OBS_AT);
    let t = target(&mut app, TGT_AT);
    if let Some(l) = look {
        app.world_mut().entity_mut(t).insert(l);
    }
    (0..TICKS)
        .map(|_| {
            step(&mut app);
            level(&app, obs)
        })
        .collect()
}

#[test]
fn upright_still_in_front_gains_exactly_the_light_only_visibility() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    half_light(&mut app);
    let obs = observer(&mut app, 7, OBS_AT);
    let t = target(&mut app, TGT_AT);
    step(&mut app); // the fresh timer starts counting; the light is sampled
    assert_eq!(app.world().get::<AiLightExposure>(t).map(|l| l.0), Some(0.5));
    // The AI8d2 light-only visibility: the exposure through the observer's darkness (default 1).
    let light_only = apply_darkness(0.5, 1.0);
    let mut expected = 0.0f32;
    for _ in 0..TICKS {
        let g = expected_gain_seen(&app, obs, OBS_AT, TGT_AT, light_only).expect("in range and cone");
        expected = (expected + g).min(1.0);
        step(&mut app);
        assert_eq!(level(&app, obs), Some(expected));
    }
    assert!(expected > 0.0);
}

#[test]
fn crouched_in_half_light_gains_less_than_upright() {
    let upright = levels(None, None);
    let crouched = levels(Some(AiVisibilityFactors { stance: Stance::Crouched, ..AiVisibilityFactors::default() }), None);
    let (u, c) = (upright.last().copied().flatten(), crouched.last().copied().flatten());
    let (Some(u), Some(c)) = (u, c) else { panic!("both seen: {u:?} {c:?}") };
    assert!(c > 0.0 && c < u, "crouched {c} vs upright {u}");
    // Absent component = an explicit default one.
    assert_eq!(levels(Some(AiVisibilityFactors::default()), None), upright);
}

/// Rules that hide every target fully.
struct Hidden;

impl DetectionRules for Hidden {
    fn concealment(&self, _: &DetectIn) -> f32 {
        100.0
    }
}

#[test]
fn custom_rules_of_full_concealment_hide_the_target() {
    let hidden = levels(None, Some(AiDetectionRules(Arc::new(Hidden))));
    assert!(hidden.iter().all(Option::is_none), "{hidden:?}");
    // The same scene with the default rules sees.
    assert!(levels(None, None).last().copied().flatten().is_some_and(|l| l > 0.0));
}

#[test]
fn rules_inserted_before_the_plugin_are_kept() {
    let rules: Arc<dyn DetectionRules> = Arc::new(Hidden);
    let mut app = App::new();
    app.insert_resource(AiDetectionRules(Arc::clone(&rules)));
    app.init_resource::<Walls>();
    app.add_plugins(AiSightPlugin::<Walls>::default());
    assert!(Arc::ptr_eq(&app.world().resource::<AiDetectionRules>().0, &rules));
}
