//! Headless tests for the light exposure system (slice AI8d2): a lamp's exposure in and out of its
//! reach, darkness feeding the meter nothing, an occluder and a roof, a darkness-0 observer, tick
//! rate invariance, the light tunables' defaults. App setup shared with `sight_tests`.

// Exact assertions on purpose (same inputs, same maths, same bits).
#![allow(clippy::float_cmp)]

use glam::Vec3;
use sf_ai::{AwarenessBand, LightConfig, LightSource, PerceptTarget, PerceptionProfile, is_dark, source_contribution};
use sf_ids::ActorKey;

use super::*;
use crate::sight_tests::{CHEST, HEAD, PLAYER, Walls, app, every_tick, level, mover, observer, step, target, wpos};

const NPC: PerceptTarget = PerceptTarget::Actor(ActorKey(99));

fn exposure(app: &App, e: Entity) -> Option<f32> {
    app.world().get::<AiLightExposure>(e).map(|l| l.0)
}

fn lamp(at: Vec3, radius_m: f32, intensity: f32) -> LightSource {
    LightSource { pos: wpos(at), radius_m, intensity, on: true }
}

fn lights(app: &mut App, sky: f32, interior: bool, sources: Vec<LightSource>) {
    app.insert_resource(AiLightInputs { sky, interior, sources });
}

fn chest(feet: Vec3) -> Vec3 {
    feet + Vec3::Y * CHEST
}

#[test]
fn lamp_lights_its_target_and_not_one_out_of_its_reach() {
    let mut app = app(60.0);
    let (near_at, far_at) = (Vec3::new(0.0, 0.0, 10.0), Vec3::new(20.0, 0.0, 10.0));
    let l = lamp(chest(near_at) + Vec3::new(2.0, 0.0, 0.0), 5.0, 0.8);
    lights(&mut app, 0.0, true, vec![l]);
    let near = target(&mut app, near_at);
    let far = app.world_mut().spawn((Perceivable { target: NPC, chest_m: CHEST, head_m: HEAD }, mover(far_at, 0.0))).id();
    step(&mut app);
    let d = wpos(chest(near_at)).delta_f32(l.pos).length();
    let expected = source_contribution(&l, d, LightConfig::default().light_falloff_exp);
    assert!((expected - 0.8 * 0.84 * 0.84).abs() < 1e-6, "{expected}");
    assert_eq!(exposure(&app, near), Some(expected));
    assert_eq!(exposure(&app, far), Some(0.0));
    // Interior: no roof ray; one source ray for the near target only.
    assert_eq!(app.world().resource::<AiRays>().light_cast, 1);
}

#[test]
fn no_sky_and_no_source_is_dark_and_the_meter_gains_nothing() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    lights(&mut app, 0.0, false, Vec::new());
    let obs = observer(&mut app, 7, Vec3::ZERO);
    let t = target(&mut app, Vec3::new(0.0, 0.0, 10.0));
    for _ in 0..60 {
        step(&mut app);
    }
    assert_eq!(exposure(&app, t), Some(0.0));
    assert!(is_dark(0.0, &LightConfig::default()));
    assert_eq!(level(&app, obs), None);
}

#[test]
fn occluder_between_lamp_and_target_leaves_the_sky_term() {
    let mut app = app(60.0);
    let t_at = Vec3::new(0.0, 0.0, 10.0);
    // The lamp 3 m to +X, a wall between at x 1..1.5 (clear of the observer's line and the roof ray).
    lights(&mut app, 0.3, false, vec![lamp(chest(t_at) + Vec3::new(3.0, 0.0, 0.0), 6.0, 1.0)]);
    app.world_mut().resource_mut::<Walls>().0.push((Vec3::new(1.0, -5.0, 5.0), Vec3::new(1.5, 5.0, 15.0)));
    let t = target(&mut app, t_at);
    step(&mut app);
    assert_eq!(exposure(&app, t), Some(0.3));
    // Roof ray + one source ray.
    assert_eq!(app.world().resource::<AiRays>().light_cast, 2);
}

#[test]
fn roof_over_the_target_scales_the_sky_term() {
    let mut app = app(60.0);
    let t_at = Vec3::new(0.0, 0.0, 10.0);
    lights(&mut app, 0.8, false, Vec::new());
    app.world_mut().resource_mut::<Walls>().0.push((Vec3::new(-3.0, 4.0, 7.0), Vec3::new(3.0, 4.3, 13.0)));
    let t = target(&mut app, t_at);
    step(&mut app);
    assert_eq!(exposure(&app, t), Some(0.8 * LightConfig::default().roof_sky_mul));
    // Indoors the same roof casts nothing: the sky value is the ambient.
    app.world_mut().resource_mut::<AiLightInputs>().interior = true;
    step(&mut app);
    assert_eq!(exposure(&app, t), Some(0.8));
}

#[test]
fn darkness_zero_observer_gains_as_if_lit() {
    let run = |dark_world: bool| -> (Vec<Option<f32>>, Vec<Option<f32>>) {
        let mut app = app(60.0);
        every_tick(&mut app, 60.0);
        if dark_world {
            lights(&mut app, 0.0, false, Vec::new());
        }
        let owl = observer(&mut app, 7, Vec3::ZERO);
        app.world_mut().entity_mut(owl).insert(AiPerceptionProfile(PerceptionProfile { darkness: 0.0, ..PerceptionProfile::default() }));
        let plain = observer(&mut app, 8, Vec3::new(1.0, 0.0, 0.0));
        target(&mut app, Vec3::new(0.0, 0.0, 12.0));
        (0..40)
            .map(|_| {
                step(&mut app);
                (level(&app, owl), level(&app, plain))
            })
            .unzip()
    };
    let (owl_dark, plain_dark) = run(true);
    let (owl_lit, plain_lit) = run(false);
    assert_eq!(owl_dark, owl_lit);
    assert!(owl_dark.last().copied().flatten().is_some_and(|l| l > 0.0));
    assert!(plain_dark.iter().all(Option::is_none));
    assert!(plain_lit.last().copied().flatten().is_some_and(|l| l > 0.0));
}

#[test]
fn exposure_and_meter_match_at_30_and_144_hz_over_one_second() {
    let times = [0.25, 0.5, 0.75, 1.0];
    let run = |hz: f64| -> (Vec<f32>, Vec<AwarenessBand>, f32) {
        let mut app = app(hz);
        let t_at = Vec3::new(0.0, 0.0, 10.0);
        lights(&mut app, 0.1, false, vec![lamp(chest(t_at) + Vec3::new(2.0, 1.0, 0.0), 6.0, 0.7)]);
        let obs = observer(&mut app, 0xABCD, Vec3::ZERO);
        let t = target(&mut app, t_at);
        let mut ticks = 0u32;
        let (mut lights_at, mut bands) = (Vec::new(), Vec::new());
        for &s in &times {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let until = (s * hz).round() as u32;
            while ticks < until {
                step(&mut app);
                ticks += 1;
            }
            lights_at.push(exposure(&app, t).expect("sampled"));
            bands.push(app.world().get::<AiAwareness>(obs).unwrap().0.band(PLAYER));
        }
        (lights_at, bands, level(&app, obs).unwrap_or(0.0))
    };
    let (light_30, bands_30, level_30) = run(30.0);
    let (light_144, bands_144, level_144) = run(144.0);
    assert_eq!(light_30, light_144);
    assert!(light_30[0] > 0.1 && light_30[0] < 1.0, "partly lit: {}", light_30[0]);
    assert_eq!(bands_30, bands_144);
    assert!(bands_30[3] >= AwarenessBand::Suspicious, "{bands_30:?}");
    // The checks fall on whole ticks (3 at 30 Hz, 14 at 144 Hz per 0.1 s period): levels agree to
    // within that rounding.
    assert!((level_30 - level_144).abs() < 0.05, "{level_30} vs {level_144}");
}

#[test]
fn removing_the_inputs_drops_every_sample() {
    let mut app = app(60.0);
    lights(&mut app, 0.5, true, Vec::new());
    let t = target(&mut app, Vec3::ZERO);
    step(&mut app);
    assert_eq!(exposure(&app, t), Some(0.5));
    app.world_mut().remove_resource::<AiLightInputs>();
    step(&mut app);
    assert_eq!(exposure(&app, t), None);
    assert_eq!(app.world().resource::<AiRays>().light_cast, 0);
}

#[test]
fn light_tunable_defaults_match_the_core_defaults() {
    use crate::perception as p;
    let c = AiPerceptionSettings::default().light;
    assert_eq!(c, LightConfig::default());
    assert_eq!(
        [p::LIGHT_SEARCH_M, p::DARK_THRESHOLD, p::ROOF_SKY_MUL, p::LIGHT_FALLOFF_EXP].map(|d| d.default_value()),
        [c.light_search_m, c.dark_threshold, c.roof_sky_mul, c.light_falloff_exp]
    );
    assert_eq!(p::LIGHT_RAYS.default_value(), c.light_rays);
}

#[test]
fn a_mover_less_target_is_sampled_at_its_pose() {
    let mut app = app(60.0);
    let feet = Vec3::new(5.0, 0.0, 5.0);
    let l = lamp(chest(feet) + Vec3::new(0.0, 0.0, 1.0), 4.0, 0.5);
    lights(&mut app, 0.0, true, vec![l]);
    let t = app
        .world_mut()
        .spawn((
            Perceivable { target: PLAYER, chest_m: CHEST, head_m: HEAD },
            PerceivablePose { feet: wpos(feet), velocity: Vec3::ZERO },
        ))
        .id();
    step(&mut app);
    let d = wpos(chest(feet)).delta_f32(l.pos).length();
    assert_eq!(exposure(&app, t), Some(source_contribution(&l, d, LightConfig::default().light_falloff_exp)));
}
