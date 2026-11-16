//! Headless sanity for the main-world hold: Bevy's own cascade build, then ours.

use bevy_app::{App, Update};
use bevy_camera::{Camera, Projection};
use bevy_ecs::prelude::*;
use bevy_light::cascade::build_directional_light_cascades;
use bevy_light::{CascadeShadowConfig, Cascades, DirectionalLight, DirectionalLightShadowMap};
use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::components::{GlobalTransform, Transform};
use sf_sky::cascade_hold::{fit_cascade, CascadeHoldConfig, Redraw};

use super::{hold_far_cascades, CascadeHoldStats, HeldSunCascades, SunCascadeHold};
use crate::CascadeSplits;

fn app(config: CascadeHoldConfig) -> (App, Entity, Entity) {
    let mut app = App::new();
    app.insert_resource(DirectionalLightShadowMap { size: 2048 })
        .insert_resource(SunCascadeHold(config))
        .init_resource::<HeldSunCascades>()
        .init_resource::<CascadeHoldStats>()
        .add_systems(Update, (build_directional_light_cascades, hold_far_cascades).chain());
    let camera = app
        .world_mut()
        .spawn((Camera::default(), Projection::default(), GlobalTransform::from(Transform::from_xyz(100.0, 20.0, -300.0))))
        .id();
    let splits = CascadeSplits { num_cascades: 3, minimum_distance: 0.1, split_near: 1.0, maximum_distance: 205.0, lambda: 0.7, overlap_proportion: 0.2 };
    let light = app
        .world_mut()
        .spawn((
            DirectionalLight { shadow_maps_enabled: true, ..Default::default() },
            splits.build(),
            Cascades::default(),
            GlobalTransform::from(Transform::from_rotation(Quat::from_rotation_y(0.6) * Quat::from_rotation_x(-0.8))),
        ))
        .id();
    (app, camera, light)
}

fn matrices(app: &App, light: Entity, camera: Entity) -> Vec<Mat4> {
    app.world().get::<Cascades>(light).expect("cascades").cascades[&camera].iter().map(|c| c.clip_from_world).collect()
}

fn move_camera(app: &mut App, camera: Entity, by: Vec3) {
    let mut transform = app.world_mut().get_mut::<GlobalTransform>(camera).expect("camera");
    *transform = GlobalTransform::from(transform.compute_transform().with_translation(transform.translation() + by));
}

/// With nothing held, the port is Bevy's fit exactly (pad 0).
#[test]
fn unpadded_port_matches_bevy() {
    let (mut app, camera, light) = app(CascadeHoldConfig { enabled: false, ..CascadeHoldConfig::default() });
    app.update();
    let world = app.world();
    let config = world.get::<CascadeShadowConfig>(light).expect("config");
    let rotation = world.get::<GlobalTransform>(light).expect("light").rotation();
    let camera_matrix = world.get::<GlobalTransform>(camera).expect("camera").to_matrix();
    let projection = world.get::<Projection>(camera).expect("projection");
    let bevy = &world.get::<Cascades>(light).expect("cascades").cascades[&camera];
    let near = [config.minimum_distance, (1.0 - config.overlap_proportion) * config.bounds[0], (1.0 - config.overlap_proportion) * config.bounds[1]];
    for (index, cascade) in bevy.iter().enumerate() {
        let corners = projection.get_frustum_corners(-near[index], -config.bounds[index]);
        let ours = fit_cascade(&corners, 2048.0, Mat4::from_quat(rotation), camera_matrix, 0.0);
        assert_eq!(ours.clip_from_world, cascade.clip_from_world, "cascade {index}");
        assert_eq!(ours.world_from_cascade, cascade.world_from_cascade, "cascade {index}");
        assert_eq!(ours.texel_size.to_bits(), cascade.texel_size.to_bits(), "cascade {index}");
    }
    assert!(world.resource::<HeldSunCascades>().0.is_empty());
}

#[test]
fn far_cascades_hold_until_the_camera_moves() {
    let (mut app, camera, light) = app(CascadeHoldConfig::default());
    app.update();
    let first = matrices(&app, light, camera);
    assert_eq!(app.world().resource::<CascadeHoldStats>().redrawn[Redraw::First.index()], 2, "cascades 1 and 2");
    assert!(app.world().resource::<HeldSunCascades>().0.is_empty());

    move_camera(&mut app, camera, Vec3::new(3.0, 0.0, 0.0));
    app.update();
    let held = matrices(&app, light, camera);
    assert_ne!(held[0], first[0], "the nearest cascade follows the camera");
    assert_eq!(held[1..], first[1..], "the far cascades keep their fit");
    let mut listed = app.world().resource::<HeldSunCascades>().0.clone();
    listed.sort_by_key(|h| h.2);
    assert_eq!(listed, [(light, camera, 1), (light, camera, 2)]);

    move_camera(&mut app, camera, Vec3::new(1.5, 0.0, 0.0));
    app.update();
    assert_ne!(matrices(&app, light, camera)[1..], first[1..], "4.5 m from the draw: refit");
    let stats = *app.world().resource::<CascadeHoldStats>();
    assert_eq!((stats.held, stats.redrawn[Redraw::Moved.index()]), (2, 2));
    assert!(app.world().resource::<HeldSunCascades>().0.is_empty());
}

#[test]
fn switching_off_restores_bevy() {
    let (mut app, camera, light) = app(CascadeHoldConfig::default());
    app.update();
    app.update();
    assert_eq!(app.world().resource::<HeldSunCascades>().0.len(), 2);
    app.world_mut().resource_mut::<SunCascadeHold>().0.enabled = false;
    move_camera(&mut app, camera, Vec3::new(1.0, 0.0, 0.0));
    app.update();
    let (mut stock, stock_camera, stock_light) = self::app(CascadeHoldConfig { enabled: false, ..CascadeHoldConfig::default() });
    move_camera(&mut stock, stock_camera, Vec3::new(1.0, 0.0, 0.0));
    stock.update();
    assert_eq!(matrices(&app, light, camera), matrices(&stock, stock_light, stock_camera));
    assert!(app.world().resource::<HeldSunCascades>().0.is_empty());
}
