//! The fog's ambient input (WE-9): the fog's fill takes the camera's ambient light, the very
//! value that lights the surfaces, not a copy of it.

use bevy_app::App;
use bevy_camera::Camera;
use bevy::color::Color;
use bevy_ecs::system::RunSystemOnce;
use bevy_light::{AmbientLight, GlobalAmbientLight};
use bevy_math::Vec3;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use sf_atmos::{CloudSettings, CloudTemporal, FogSettings, FroxelSettings, VIEW_AMBIENT_IN_FOG};
use sf_sky_bevy::sf_sky::SkyState;
use sf_sky_bevy::SkyRenderOptions;

use super::*;

/// An app with [`drive`]'s resources and nothing else (no renderer).
fn app(fog_scale: f32) -> App {
    let mut app = App::new();
    app.init_resource::<Time>()
        .insert_resource(Clouds { settings: CloudSettings::default(), enabled: false, shadows: false, glows: [Vec4::ZERO; GLOWS], temporal: CloudTemporal::default() })
        .insert_resource(Fog { settings: FogSettings::default(), enabled: true, froxels: Some(FroxelSettings::high()), view_ambient: fog_scale })
        .init_resource::<CloudsOrigin>()
        .init_resource::<CloudImages>()
        .init_resource::<Scroll>()
        .init_resource::<Slicer>()
        .init_resource::<Drift>()
        .init_resource::<CloudFrame>()
        .init_resource::<FogBankEdges>()
        .init_resource::<FogCloudSplit>()
        .init_resource::<CloudEdgeStable>()
        .init_resource::<CumulonimbusShape>()
        .init_resource::<FarStormShape>()
        .init_resource::<FogShadowHeights>()
        .init_resource::<FarShadowShape>()
        .init_resource::<FroxelVolume>()
        .init_resource::<FroxelsEnabled>();
    app
}

fn fog_ambient(app: &mut App) -> Vec3 {
    app.world_mut().run_system_once(drive).unwrap();
    app.world().resource::<CloudFrame>().fog_ambient
}

/// Logic: at night the fog's ambient input is exactly the surfaces' ambient, the camera's
/// `AmbientLight` as `sf_sky_bevy` writes it (the night floor × the weather's tint), and not the
/// leftover global ambient; the A/B's 0 takes it out.
#[test]
fn fog_ambient_is_the_surfaces_ambient() {
    // The sky at night: a 4-nit floor tinted grey-blue by the weather (`SkyState::ambient`,
    // `ambient_tint`), the colour as `sf_sky_bevy`'s `apply_cameras` sets it on the camera.
    let night = SkyState { ambient: 4.0, ambient_tint: Some(Vec3::new(0.8, 0.95, 1.2)), ..SkyState::default() };
    let surfaces = AmbientLight { color: sf_sky_bevy::ambient_color(&night, &SkyRenderOptions::default()), brightness: night.ambient, affects_lightmapped_meshes: true };
    let c = surfaces.color.to_linear();
    let expected = Vec3::new(c.red, c.green, c.blue) * surfaces.brightness;

    let mut app = app(VIEW_AMBIENT_IN_FOG);
    // A global ambient the camera's own overrides (an interior's, say): ignored, as Bevy does.
    app.insert_resource(GlobalAmbientLight { color: Color::WHITE, brightness: 300.0, ..GlobalAmbientLight::default() });
    let camera = app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default(), surfaces.clone())).id();
    assert_eq!(fog_ambient(&mut app), expected);
    assert_eq!(view_ambient(Some(&surfaces), None), expected);

    // No ambient on the camera: the global one lights the surfaces, and the fog.
    app.world_mut().entity_mut(camera).remove::<AmbientLight>();
    assert_eq!(fog_ambient(&mut app), Vec3::splat(300.0));

    // The A/B off (`flip fog_night_light`): the sky-only fill.
    app.world_mut().resource_mut::<Fog>().view_ambient = 0.0;
    assert_eq!(fog_ambient(&mut app), Vec3::ZERO);
}

/// Logic: the depth-aware fog bank (FOG-BANK) is on by default and its switch reaches the
/// render world's frame (`--off fogbank` sets it false).
#[test]
fn fog_bank_edges_reach_the_frame() {
    let mut app = app(VIEW_AMBIENT_IN_FOG);
    app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default()));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(app.world().resource::<FogBankEdges>().0);
    assert!(app.world().resource::<CloudFrame>().fog_bank_edges);
    app.insert_resource(FogBankEdges(false));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(!app.world().resource::<CloudFrame>().fog_bank_edges);
}

/// Logic: the fog split at the cloud's depth (WE12-FIX) is on by default and its switch reaches
/// the render world's frame (`--off cloudfog` sets it false).
#[test]
fn fog_cloud_split_reaches_the_frame() {
    let mut app = app(VIEW_AMBIENT_IN_FOG);
    app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default()));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(app.world().resource::<FogCloudSplit>().0);
    assert!(app.world().resource::<CloudFrame>().fog_cloud_split);
    app.insert_resource(FogCloudSplit(false));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(!app.world().resource::<CloudFrame>().fog_cloud_split);
}

/// Logic: stable cloud silhouettes (S1) are on by default and the switch reaches the render
/// world's frame (`--off cloudedge` sets it false).
#[test]
fn cloud_edge_stable_reaches_the_frame() {
    let mut app = app(VIEW_AMBIENT_IN_FOG);
    app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default()));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(app.world().resource::<CloudEdgeStable>().0);
    assert!(app.world().resource::<CloudFrame>().cloud_edge_stable);
    app.insert_resource(CloudEdgeStable(false));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(!app.world().resource::<CloudFrame>().cloud_edge_stable);
}

/// The storm shape (CW-1) is on by default and reaches the render world's frame with the game's
/// metres (`--off stormshape` turns it off).
#[test]
fn storm_shape_reaches_the_frame() {
    let mut app = app(VIEW_AMBIENT_IN_FOG);
    app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default()));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(app.world().resource::<CloudFrame>().storm_shape.enabled);
    let game = sf_atmos::StormShape { enabled: false, edge_m: 40.0, ..sf_atmos::StormShape::default() };
    app.insert_resource(FarStormShape(game));
    app.world_mut().run_system_once(drive).unwrap();
    assert_eq!(app.world().resource::<CloudFrame>().storm_shape, game);
}

/// Logic: the storm cloud shape (S3) is on by default and the switch reaches the render world's
/// frame (`--off cbshape` sets it false).
#[test]
fn cb_shape_reaches_the_frame() {
    let mut app = app(VIEW_AMBIENT_IN_FOG);
    app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default()));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(app.world().resource::<CumulonimbusShape>().0);
    assert!(app.world().resource::<CloudFrame>().cb_shape);
    app.insert_resource(CumulonimbusShape(false));
    app.world_mut().run_system_once(drive).unwrap();
    assert!(!app.world().resource::<CloudFrame>().cb_shape);
}

/// [`app`] with the tunables, the froxel switch ([`add_froxel_switch`], as `CloudsPlugin` adds it), a
/// camera, the noise in place and what [`transparent::update_volume`] needs (no renderer).
fn switch_app() -> App {
    let mut app = app(VIEW_AMBIENT_IN_FOG);
    app.add_plugins((bevy::MinimalPlugins, sf_tunables_bevy::TunablesPlugin));
    add_froxel_switch(&mut app);
    app.init_resource::<Assets<Image>>()
        .init_resource::<Assets<FroxelFogMaterial>>()
        .insert_resource(sf_render_order_bevy::TranslucentFog { volume: Handle::default(), params: sf_render_order_bevy::NO_FOG });
    app.world_mut().resource_mut::<CloudImages>().noise =
        Some(NoiseImages { shape: Handle::default(), detail: Handle::default(), weather: Handle::default(), blue_noise: Handle::default() });
    app.world_mut().spawn((CloudCamera, Camera::default(), GlobalTransform::default()));
    app
}

/// Sets `sky.froxels.enabled` at the console layer, then runs the frame's main-world side.
fn set_froxels(app: &mut App, on: bool) {
    app.world_mut().write_message(sf_tunables_bevy::SetTunable::new("sky.froxels.enabled", if on { "true" } else { "false" }, sf_tunables_bevy::SetBy::Console));
    app.update();
    app.world_mut().run_system_once(transparent::update_volume).unwrap();
    app.world_mut().run_system_once(drive).unwrap();
}

/// FS-L2: `sky.froxels.enabled` off at the console layer takes the froxels out of the render world's
/// frame (no grid, no lamps lit into it) and shrinks the shared volume to its unwritten stand-in, while
/// the analytic fog stays on; on brings the grid and the volume back. The render world's skip (no
/// inject/integrate dispatch when the frame has no grid) needs a GPU: its history rule is
/// [`froxels_history_restarts_after_the_switch`].
#[test]
#[allow(clippy::float_cmp)] // `params.w` is a 0/1 flag
fn froxels_switch_off_keeps_the_fog() {
    let mut app = switch_app();
    let high = FroxelSettings::high();
    app.world_mut().spawn((PointLight { range: 20.0, ..PointLight::default() }, GlobalTransform::from_translation(Vec3::new(0.0, 0.0, -5.0)), ViewVisibility::VISIBLE));
    app.world_mut().resource_mut::<Fog>().settings.height.density = 0.01;
    assert!(app.world().resource::<Fog>().settings.any(), "the test fog draws");
    set_froxels(&mut app, true);
    let store = app.world().resource::<sf_tunables_bevy::Tunables>();
    assert!(store.0.id("sky.froxels.enabled").is_some(), "registered");
    assert!(TUNABLES.iter().any(|d| d.name() == "sky.froxels.enabled"), "in the catalogue");
    assert_eq!(FROXELS.name(), "sky.froxels.enabled");
    let frame = app.world().resource::<CloudFrame>();
    assert_eq!(frame.froxels, Some(high));
    assert_eq!(frame.fog_lights.len(), 1, "the lamp lit into the volume");
    assert!(frame.fog.is_some());
    assert_eq!(app.world().resource::<FroxelVolume>().size, high.size);
    assert_eq!(app.world().resource::<FroxelVolume>().params.w, 1.0, "written");

    set_froxels(&mut app, false);
    assert!(!app.world().resource::<FroxelsEnabled>().0);
    let frame = app.world().resource::<CloudFrame>();
    assert_eq!(frame.froxels, None, "no froxel passes");
    assert!(frame.fog_lights.is_empty(), "no lamps lit into the volume");
    assert!(frame.fog.is_some(), "the analytic fog keeps drawing");
    let volume = app.world().resource::<FroxelVolume>();
    assert_eq!((volume.size, volume.params.w), ([1, 1, 1], 0.0), "transparents read the unwritten stand-in");
    assert_eq!(app.world().resource::<Fog>().froxels, Some(high), "the settings survive the switch");

    set_froxels(&mut app, true);
    assert_eq!(app.world().resource::<CloudFrame>().froxels, Some(high));
    let volume = app.world().resource::<FroxelVolume>();
    assert_eq!((volume.size, volume.params.w), (high.size, 1.0));
}

/// FS-L2: the render world records the grid each frame (`History::froxels`, `None` while the switch
/// is off), so the first frame back blends no stale history; the next one does.
#[test]
fn froxels_history_restarts_after_the_switch() {
    let high = Some(FroxelSettings::high());
    let mut recorded = None;
    let mut valid = Vec::new();
    for (frame, settings) in [(0, high), (1, high), (2, None), (3, high), (4, high)] {
        valid.push(render::froxel_history_valid(settings, recorded, frame));
        recorded = settings.map(|f| f.size);
    }
    assert_eq!(valid, [false, true, false, false, true]);
}

/// FS-2b: the switches `CloudsPlugin` registers (froxels, sun visibility, light shafts, aerial shadow)
/// are in `FeatureSwitches` after its build functions ran, and one follows a console-layer set.
#[test]
fn feature_switches_registered() {
    let mut app = App::new();
    app.add_plugins((bevy::MinimalPlugins, sf_tunables_bevy::TunablesPlugin)).init_resource::<FogShadowHeights>();
    add_froxel_switch(&mut app);
    aerial_shadow::build(&mut app);
    shafts::build(&mut app);
    sun_visibility::build(&mut app);
    app.finish();
    app.cleanup();
    app.update();
    let names = ["sky.froxels.enabled", "sky.sun_vis.enabled", "sky.shafts.enabled", "sky.aerial.shadow"];
    let running = |app: &App, name: &str| {
        let switches = app.world().resource::<sf_tunables_bevy::FeatureSwitches>();
        assert!(switches.get(name).is_some(), "{name} registered");
        switches.running(&app.world().resource::<sf_tunables_bevy::Tunables>().0, name)
    };
    let defaults: Vec<bool> = names.iter().map(|n| running(&app, n)).collect();
    assert_eq!(defaults, [true, true, false, false], "on as their tunables' defaults");
    app.world_mut().write_message(sf_tunables_bevy::SetTunable::new("sky.sun_vis.enabled", "false", sf_tunables_bevy::SetBy::Console));
    app.update();
    assert!(!running(&app, "sky.sun_vis.enabled"), "off at the console layer");
    assert!(running(&app, "sky.froxels.enabled"), "the others untouched");
}
