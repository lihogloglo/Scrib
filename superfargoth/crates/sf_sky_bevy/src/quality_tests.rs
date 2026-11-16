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

/// T5a: the prepasses and MSAA follow their consumers live. Contact shadows + GTAO on a view without
/// a temporal resolve add the depth, normal and motion-vector prepasses and drop MSAA; turning both
/// off gives the view back exactly as a start without them (no prepass, its MSAA); on restores them.
/// A temporal resolve keeps depth + motion vectors (and MSAA off) without them, and its own way back
/// to `off` (`upscale.rs` snapshot) ends as a plain view again.
#[test]
fn prepasses_follow_contact_shadows_gtao_and_the_temporal_resolve() {
    use bevy_anti_alias::taa::TemporalAntiAliasing;
    use bevy_core_pipeline::prepass::{MotionVectorPrepass, NormalPrepass};
    use bevy_render::view::Msaa;

    let mut app = app();
    app.insert_resource(crate::ActiveRenderPath(sf_sky::RenderPath::Raster))
        .configure_sets(bevy_app::Update, (crate::SkySystems::RenderPath, crate::path::PathSystems));
    crate::upscale::build(&mut app);
    let camera = app.world_mut().spawn((crate::RenderPathCamera, ViewQualityCamera)).id();
    let read = |app: &App| {
        let w = app.world();
        (
            w.get::<DepthPrepass>(camera).is_some(),
            w.get::<NormalPrepass>(camera).is_some(),
            w.get::<MotionVectorPrepass>(camera).is_some(),
            w.get::<Msaa>(camera).copied(),
        )
    };
    app.update();
    assert_eq!(read(&app), (false, false, false, Some(Msaa::Sample4)), "plain view: no prepass, MSAA");
    set(&mut app, "shadow.contact", "on");
    set(&mut app, "render.ssao.quality", "medium");
    assert_eq!(read(&app), (true, true, true, Some(Msaa::Off)), "contact + GTAO");
    set(&mut app, "render.ssao.quality", "off");
    assert_eq!(read(&app), (true, false, true, Some(Msaa::Sample4)), "contact alone: depth + motion vectors, MSAA back");
    set(&mut app, "shadow.contact", "off");
    assert_eq!(read(&app), (false, false, false, Some(Msaa::Sample4)), "both off: as a start without them");
    set(&mut app, "shadow.contact", "on");
    set(&mut app, "render.ssao.quality", "high");
    assert_eq!(read(&app), (true, true, true, Some(Msaa::Off)), "back on");
    // A temporal resolve through its tunable: it keeps depth + motion vectors without the effects.
    set(&mut app, "render.upscaling", "taa");
    set(&mut app, "shadow.contact", "off");
    set(&mut app, "render.ssao.quality", "off");
    assert!(app.world().get::<TemporalAntiAliasing>(camera).is_some(), "the binding reached the camera");
    assert_eq!(read(&app), (true, false, true, Some(Msaa::Off)), "TAA alone");
    set(&mut app, "render.upscaling", "off");
    assert!(app.world().get::<TemporalAntiAliasing>(camera).is_none());
    assert_eq!(read(&app), (false, false, false, Some(Msaa::Sample4)), "off again: plain view");
}

/// A host's own prepasses ([`crate::ViewPrepassBase`]) stay whatever the effects do.
#[test]
fn a_views_base_prepasses_stay() {
    use bevy_core_pipeline::prepass::MotionVectorPrepass;
    let mut app = app();
    let base = crate::ViewPrepassBase { depth: true, motion_vectors: true, ..Default::default() };
    let camera = app.world_mut().spawn((ViewQualityCamera, base)).id();
    app.update();
    set(&mut app, "shadow.contact", "on");
    set(&mut app, "shadow.contact", "off");
    let w = app.world();
    assert!(w.get::<DepthPrepass>(camera).is_some() && w.get::<MotionVectorPrepass>(camera).is_some());
}
