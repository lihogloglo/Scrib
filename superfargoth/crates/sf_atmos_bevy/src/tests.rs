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
