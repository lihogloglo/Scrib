//! Tunables T4 (`quality.rs`): a console set reaches the views' components and the sun light, live,
//! in a headless app (no render world).

use bevy_app::App;
use bevy_core_pipeline::prepass::DepthPrepass;
use bevy_light::{CascadeShadowConfig, DirectionalLight, ShadowFilteringMethod};
use bevy_pbr::{ContactShadows, ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use crate::{ShadowQualityPlugin, SkyRenderOptions, SunLight, SunShadowQuality, UpscalingSetting, ViewQualityCamera};
use sf_sky::Upscaling;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, ShadowQualityPlugin))
        .init_resource::<SkyRenderOptions>()
        .insert_resource(SunShadowQuality::default())
        .insert_resource(UpscalingSetting(Upscaling::Off));
    app
}

fn set(app: &mut App, name: &str, value: &str) {
    app.world_mut().write_message(SetTunable::new(name, value, SetBy::Console));
    app.update();
}

#[test]
fn view_quality_follows_its_tunables_both_ways() {
    let mut app = app();
    let camera = app.world_mut().spawn(ViewQualityCamera).id();
    app.update();
    let w = app.world();
    assert!(w.get::<ContactShadows>(camera).is_none() && w.get::<ScreenSpaceAmbientOcclusion>(camera).is_none(), "framework defaults: off");
    assert_eq!(w.get::<ShadowFilteringMethod>(camera), Some(&ShadowFilteringMethod::Gaussian), "auto without a temporal resolve");
    set(&mut app, "shadow.contact", "on");
    set(&mut app, "shadow.contact.steps", "8");
    set(&mut app, "render.ssao.quality", "high");
    set(&mut app, "shadow.filter", "hardware");
    let w = app.world();
    assert_eq!(w.get::<ContactShadows>(camera).map(|c| c.linear_steps), Some(8));
    assert!(w.get::<DepthPrepass>(camera).is_some(), "the prepass they march");
    assert_eq!(w.get::<ScreenSpaceAmbientOcclusion>(camera).map(|s| s.quality_level), Some(ScreenSpaceAmbientOcclusionQualityLevel::High));
    assert_eq!(w.get::<ShadowFilteringMethod>(camera), Some(&ShadowFilteringMethod::Hardware2x2));
    assert!(w.resource::<SkyRenderOptions>().sun_shadow.contact_shadows, "the sun casts them");
    set(&mut app, "shadow.contact", "off");
    set(&mut app, "render.ssao.quality", "off");
    set(&mut app, "shadow.filter", "auto");
    app.world_mut().resource_mut::<UpscalingSetting>().0 = Upscaling::Taa;
    app.update();
    let w = app.world();
    assert!(w.get::<ContactShadows>(camera).is_none() && w.get::<ScreenSpaceAmbientOcclusion>(camera).is_none(), "live off");
    assert_eq!(w.get::<ShadowFilteringMethod>(camera), Some(&ShadowFilteringMethod::Temporal), "auto under TAA");
    assert!(!w.resource::<SkyRenderOptions>().sun_shadow.contact_shadows);
}

#[test]
fn sun_light_follows_cascades_and_pcss() {
    let mut app = app();
    let sun = app.world_mut().spawn((SunLight, DirectionalLight::default(), CascadeShadowConfig::default())).id();
    app.update();
    assert_eq!(app.world().get::<CascadeShadowConfig>(sun).map(|c| c.bounds.len()), Some(4), "the default layout");
    set(&mut app, "shadow.sun.cascades", "2");
    set(&mut app, "shadow.sun.distance_m", "120");
    let bounds = app.world().get::<CascadeShadowConfig>(sun).map(|c| c.bounds.clone()).expect("cascades");
    assert_eq!(bounds.len(), 2);
    assert!((bounds[1] - 120.0).abs() < 1e-3, "{bounds:?}");
    set(&mut app, "shadow.sun.soft", "on");
    assert_eq!(app.world().resource::<SkyRenderOptions>().sun_shadow.soft_size, Some(10.0));
    set(&mut app, "shadow.sun.soft", "off");
    assert_eq!(app.world().resource::<SkyRenderOptions>().sun_shadow.soft_size, None);
}
