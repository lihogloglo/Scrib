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
        .init_resource::<FroxelVolume>();
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
