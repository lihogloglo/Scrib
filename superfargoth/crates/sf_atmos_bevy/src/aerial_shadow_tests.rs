//! AS (`docs/specs/aerial-shadow.md`, vendored `bevy_pbr` patch "own, AS: aerial shadow"): the
//! terrain's shadow-height map reaches every atmosphere camera as an `AtmosphereHeightShadow` when
//! it changes and is gone when there is none or `sky.aerial.shadow` is off; the render world gets
//! its uniform only while the camera has one (absent = the stock aerial view LUT). No GPU: the
//! shader side is `sf_sky_bevy` `shader_tests.rs`.

use bevy::MinimalPlugins;
use bevy_app::App;
use bevy_asset::Handle;
use bevy_camera::Camera3d;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_light::DirectionalLight;
use bevy_math::{Quat, Vec3, Vec4};
use bevy_pbr::{AtmosphereHeightShadow, AtmosphereHeightShadowImage, AtmosphereSettings, GpuAtmosphereHeightShadow};
use bevy_render::sync_world::RenderEntity;
use bevy_render::MainWorld;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use super::*;

const PARAMS: Vec4 = Vec4::new(-4608.0, -4608.0, 1.0 / 9216.0, 12.0);

fn heights(params: Vec4) -> FogShadowHeights {
    FogShadowHeights { image: Some(Handle::<Image>::default()), params }
}

/// Logic: none without a map, without an extent, without the light or with the switch off;
/// otherwise the map, its params, the light and the shared far-shadow rule, exactly.
#[test]
fn the_wanted_shadow_copies_the_map_the_light_and_the_rule() {
    let shape = FarShadowShape::default();
    let light = Some(Vec3::new(0.0, 0.2, 0.98).normalize());
    assert!(wanted_height_shadow(&FogShadowHeights::default(), shape, true, light).is_none(), "no map");
    assert!(wanted_height_shadow(&heights(PARAMS.with_z(0.0)), shape, true, light).is_none(), "no extent yet");
    assert!(wanted_height_shadow(&heights(PARAMS), shape, true, None).is_none(), "no shadow light");
    assert!(wanted_height_shadow(&heights(PARAMS), shape, false, light).is_none(), "sky.aerial.shadow off");
    let s = wanted_height_shadow(&heights(PARAMS), shape, true, light).expect("a shadow");
    assert_eq!(s.params, PARAMS);
    assert_eq!(Some(s.light_dir), light);
    assert_eq!(
        (s.penumbra_per_m, s.penumbra_min, s.bias, s.bias_per_m, s.fade_start),
        (0.012, 2.0, 6.0, 0.0015, 0.85),
        "the froxels' old consts are the defaults (froxel look unchanged)"
    );
}

/// An app with the wiring, the tunables, a sun with the cloud shadow marker and one atmosphere
/// camera (no renderer).
fn app() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TunablesPlugin)).init_resource::<FogShadowHeights>();
    aerial_shadow::build(&mut app);
    let sun = Transform::from_rotation(Quat::from_rotation_x(-0.2));
    app.world_mut().spawn((DirectionalLight::default(), CloudShadowLight, GlobalTransform::from(sun)));
    let camera = app.world_mut().spawn((Camera3d::default(), AtmosphereSettings::default(), GlobalTransform::default())).id();
    app.init_resource::<Writes>().add_systems(bevy_app::Last, count_writes);
    app.finish();
    app.cleanup();
    // Off by default (FG-12 verdict); these tests check the switched-on path.
    app.world_mut().write_message(SetTunable::new("sky.aerial.shadow", "true", SetBy::Console));
    (app, camera)
}

/// Frames x cameras whose `AtmosphereHeightShadow` was written (inserted or changed).
#[derive(Resource, Default)]
struct Writes(usize);

fn count_writes(changed: Query<(), Changed<AtmosphereHeightShadow>>, mut writes: ResMut<Writes>) {
    writes.0 += changed.iter().count();
}

fn shadow(app: &App, camera: Entity) -> Option<AtmosphereHeightShadow> {
    app.world().get::<AtmosphereHeightShadow>(camera).cloned()
}

/// Logic: the camera gets the component once the map has an extent, follows a params change (the
/// floating origin), a rule change (`sky.far_shadow.*`) and loses it with the switch off or the map
/// gone; an unchanged frame writes nothing.
#[test]
#[expect(clippy::float_cmp, reason = "exact: copied values")]
fn the_atmosphere_camera_follows_the_shadow_heights() {
    let (mut app, camera) = app();
    app.update();
    assert!(shadow(&app, camera).is_none(), "no map yet");

    *app.world_mut().resource_mut::<FogShadowHeights>() = heights(PARAMS);
    app.update();
    let s = shadow(&app, camera).expect("copied");
    assert_eq!(s.params, PARAMS);
    let sun_back = (Quat::from_rotation_x(-0.2) * Vec3::Z).normalize();
    assert!(s.light_dir.dot(sun_back) > 0.999_99, "toward the cloud shadow light: {:?}", s.light_dir);

    assert_eq!(app.world().resource::<Writes>().0, 1, "inserted once");
    app.update();
    assert_eq!(app.world().resource::<Writes>().0, 1, "unchanged = not written");

    let moved = PARAMS + Vec4::new(117.0, -50.0, 0.0, 0.0);
    app.world_mut().resource_mut::<FogShadowHeights>().params = moved;
    app.update();
    assert_eq!(shadow(&app, camera).expect("kept").params, moved);

    app.world_mut().write_message(SetTunable::new("sky.far_shadow.bias_m", "9", SetBy::Console));
    app.update();
    assert_eq!(shadow(&app, camera).expect("kept").bias, 9.0);

    app.world_mut().write_message(SetTunable::new("sky.aerial.shadow", "false", SetBy::Console));
    app.update();
    assert!(shadow(&app, camera).is_none(), "sky.aerial.shadow off = stock");
    app.world_mut().write_message(SetTunable::new("sky.aerial.shadow", "true", SetBy::Console));
    app.update();
    assert!(shadow(&app, camera).is_some(), "back on");

    app.world_mut().resource_mut::<FogShadowHeights>().params = moved.with_z(0.0);
    app.update();
    assert!(shadow(&app, camera).is_none(), "a map with no extent is none");
}

/// Logic: `extract_atmosphere` gives the render view the height shadow's uniform and image while
/// the camera has one with an extent, and removes them (and their uniform index) otherwise, so the
/// node falls back to the stock aerial view LUT.
#[test]
fn extraction_carries_the_height_shadow_only_while_present() {
    let mut main = World::new();
    let mut render = World::new();
    render.init_resource::<MainWorld>();
    let extract = render.register_system(bevy_pbr::extract_atmosphere);
    let render_camera = render.spawn_empty().id();
    let s = wanted_height_shadow(&heights(PARAMS), FarShadowShape::default(), true, Some(Vec3::Y)).expect("a shadow");
    let camera = main
        .spawn((Camera3d::default(), AtmosphereSettings::default(), GlobalTransform::default(), RenderEntity::from(render_camera), s))
        .id();
    let run = |main: &mut World, render: &mut World| {
        std::mem::swap(main, &mut *render.resource_mut::<MainWorld>());
        render.run_system(extract).expect("extract runs");
        render.flush();
        std::mem::swap(main, &mut *render.resource_mut::<MainWorld>());
        main.increment_change_tick();
    };

    run(&mut main, &mut render);
    let gpu = render.get::<GpuAtmosphereHeightShadow>(render_camera).expect("extracted");
    assert_eq!((gpu.params, gpu.light_dir, gpu.bias, gpu.fade_start), (PARAMS, Vec3::Y, 6.0, 0.85));
    assert!(render.get::<AtmosphereHeightShadowImage>(render_camera).is_some(), "its image");

    main.get_mut::<AtmosphereHeightShadow>(camera).expect("present").params.z = 0.0;
    run(&mut main, &mut render);
    assert!(render.get::<GpuAtmosphereHeightShadow>(render_camera).is_none(), "no extent = stock");

    main.get_mut::<AtmosphereHeightShadow>(camera).expect("present").params = PARAMS;
    run(&mut main, &mut render);
    assert!(render.get::<GpuAtmosphereHeightShadow>(render_camera).is_some(), "back");

    main.entity_mut(camera).remove::<AtmosphereHeightShadow>();
    run(&mut main, &mut render);
    assert!(render.get::<GpuAtmosphereHeightShadow>(render_camera).is_none(), "removed = stock");
    assert!(render.get::<AtmosphereHeightShadowImage>(render_camera).is_none());
}
