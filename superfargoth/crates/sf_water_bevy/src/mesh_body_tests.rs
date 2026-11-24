//! Mesh water bodies ([`crate::WaterMeshSurface`]): the plugin's systems give a host mesh the water
//! material on the mesh-body path, leave it to Bevy's frustum culling, and count it as a shown body
//! for the visibility gate and the SSPR plane; the ocean keeps the clipmap path.
#![allow(clippy::float_cmp, deprecated)] // exact: copied levels and box corners; the frustum as gate_tests builds it

use bevy_asset::{Assets, RenderAssetUsages};
use bevy_camera::primitives::{Aabb, Frustum};
use bevy_camera::visibility::{InheritedVisibility, NoFrustumCulling};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy_pbr::MeshMaterial3d;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::{
    gate, render, world_box, ClipmapSettings, OceanSpectrum, OceanSurface, WaterBody, WaterCamera, WaterFrame, WaterGate, WaterGateSettings, WaterLook, WaterMaterial, WaterMeshSurface, WaterQuality,
    WaterRain, WaterRainSettings, WaterReflections, WaterRegionsInput, WaterShoreInput, WaterSurfFoam,
};

/// A ribbon `length` m along +X, `width` m wide, dropping `drop` m downstream (a flat lake polygon
/// at `drop` 0), in two-metre segments, at local height `y`.
fn ribbon(length: f32, width: f32, y: f32, drop: f32) -> Mesh {
    let segments = 8u32;
    let mut positions = Vec::new();
    let mut flow = Vec::new();
    #[allow(clippy::cast_precision_loss)]
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        for z in [-0.5 * width, 0.5 * width] {
            positions.push([t * length, y - t * drop, z]);
            flow.push([1.0, 0.0]);
        }
    }
    let mut indices = Vec::new();
    for i in 0..segments {
        let a = 2 * i;
        indices.extend_from_slice(&[a, a + 1, a + 2, a + 2, a + 1, a + 3]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(crate::mesh::ATTRIBUTE_WATER_FLOW, flow);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// A camera at `eye` looking along `dir` (as `gate_tests`).
fn frustum(eye: Vec3, dir: Vec3) -> Frustum {
    let up = if dir.normalize().y.abs() > 0.99 { Vec3::Z } else { Vec3::Y };
    let view = Mat4::look_to_rh(eye, dir, up);
    let proj = Mat4::perspective_infinite_reverse_rh(60f32.to_radians(), 16.0 / 9.0, 0.1);
    Frustum(<<Frustum as std::ops::Deref>::Target>::from_clip_from_world(&(proj * view)))
}

/// The plugin's resources and its attach, gate and drive systems in their order, without a renderer.
fn world() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<Assets<Image>>();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<WaterMaterial>>();
    world.init_resource::<WaterQuality>();
    world.init_resource::<OceanSpectrum>();
    world.init_resource::<WaterLook>();
    world.init_resource::<WaterShoreInput>();
    world.init_resource::<WaterRegionsInput>();
    world.init_resource::<WaterReflections>();
    world.init_resource::<WaterSurfFoam>();
    world.init_resource::<WaterRain>();
    world.init_resource::<WaterRainSettings>();
    world.init_resource::<crate::WaterFlowSettings>();
    world.init_resource::<ClipmapSettings>();
    world.init_resource::<gate::WetTilesCache>();
    world.init_resource::<render::WaterRender>();
    world.insert_resource(WaterGateSettings { enabled: true, ..WaterGateSettings::default() });
    // No ocean this frame (an inland view): only the bodies.
    world.insert_resource(WaterFrame { enabled: false, ..WaterFrame::default() });
    let mut startup = Schedule::default();
    startup.add_systems(crate::create_images);
    startup.run(&mut world);
    let mut schedule = Schedule::default();
    schedule.add_systems((crate::attach_surface, crate::attach_mesh_surface, gate::update_tiles, gate::gate_views, crate::drive).chain());
    (world, schedule)
}

/// A lake polygon at 40 m: the water material on the mesh path, the host's mesh, an `Aabb` and no
/// `NoFrustumCulling`; the gate's box and the SSPR plane are the lake's; the body's params are
/// flat (no swell) at its level, and the velocity redraw (a clipmap) stays off with only mesh bodies.
#[test]
fn a_mesh_body_is_drawn_culled_and_seen() {
    let (mut world, mut schedule) = world();
    let lake = world.resource_mut::<Assets<Mesh>>().add(ribbon(16.0, 10.0, 0.0, 0.0));
    let at = Transform::from_xyz(100.0, 40.0, 50.0);
    let body = world.spawn((WaterMeshSurface::still(lake.clone(), WaterLook::default()), at, GlobalTransform::from(at), InheritedVisibility::VISIBLE)).id();
    let ocean = world.spawn((OceanSurface, bevy_camera::visibility::Visibility::Inherited, InheritedVisibility::VISIBLE)).id();
    let eye = Vec3::new(108.0, 80.0, 50.0);
    let camera = world.spawn((WaterCamera, GlobalTransform::from_translation(eye), frustum(eye, Vec3::NEG_Y))).id();
    schedule.run(&mut world);

    let e = world.entity(body);
    assert_eq!(e.get::<Mesh3d>().map(|m| m.0.clone()), Some(lake.clone()), "the host's mesh");
    assert!(e.get::<Aabb>().is_some(), "no Aabb: not frustum-culled");
    assert!(!e.contains::<NoFrustumCulling>(), "a mesh body is culled like any mesh");
    let handle = e.get::<MeshMaterial3d<WaterMaterial>>().expect("the water material").0.clone();
    let material = world.resource::<Assets<WaterMaterial>>().get(&handle).expect("material asset");
    assert!(material.extension.mesh, "the mesh-body vertex path");
    let p = material.extension.params;
    assert_eq!(p.clip.w, 40.0, "the body's level");
    assert_eq!(p.waves.x, 0.0, "no swell on a mesh body");
    assert!(p.waves.y > 0.0, "the chop's ripples");
    assert_eq!(p.waves.z, 0.0, "no rectangle clip");

    // The ocean keeps the clipmap (camera-centred, never culled).
    let o = world.entity(ocean);
    assert!(o.contains::<NoFrustumCulling>());
    let oh = o.get::<MeshMaterial3d<WaterMaterial>>().expect("ocean material").0.clone();
    assert!(!world.resource::<Assets<WaterMaterial>>().get(&oh).expect("asset").extension.mesh);

    let gate = world.get::<WaterGate>(camera).expect("gated camera");
    assert!(gate.in_frustum() && !gate.forced, "{gate:?}");
    assert_eq!(gate.boxes.len(), 1, "{:?}", gate.boxes);
    let [lo, hi] = gate.boxes[0];
    assert!(lo.x <= 100.0 && hi.x >= 116.0 && lo.z <= 45.0 && hi.z >= 55.0 && lo.y < 40.0 && hi.y > 40.0, "{lo} {hi}");

    let state = world.resource::<render::WaterRender>();
    assert!(state.active, "a shown body keeps the water passes on");
    assert!(!state.clipmap, "no clipmap to redraw for motion vectors");
    assert_eq!(state.sspr.map(|s| s.level), Some(40.0), "SSPR mirrors the lake");
}

/// A mesh body out of view (looking away) gives the gate no box; a hidden one is not a body at all.
#[test]
fn hidden_or_unseen_mesh_bodies_close_the_gate() {
    let (mut world, mut schedule) = world();
    let lake = world.resource_mut::<Assets<Mesh>>().add(ribbon(16.0, 10.0, 0.0, 0.0));
    let at = Transform::from_xyz(100.0, 40.0, 50.0);
    let body = world.spawn((WaterMeshSurface::still(lake, WaterLook::default()), at, GlobalTransform::from(at), InheritedVisibility::VISIBLE)).id();
    let eye = Vec3::new(108.0, 80.0, 50.0);
    let camera = world.spawn((WaterCamera, GlobalTransform::from_translation(eye), frustum(eye, Vec3::Y))).id();
    schedule.run(&mut world);
    assert!(!world.get::<WaterGate>(camera).expect("gated").in_frustum(), "looking at the sky");
    world.entity_mut(body).insert(InheritedVisibility::HIDDEN);
    world.entity_mut(camera).insert(frustum(eye, Vec3::NEG_Y));
    schedule.run(&mut world);
    assert!(!world.get::<WaterGate>(camera).expect("gated").in_frustum(), "a hidden body is no water");
    assert!(!world.resource::<render::WaterRender>().active);
}

/// A sloped river ribbon under a rotated, moved transform: its render-space box is the mesh's
/// through the transform, its level the box's middle; swapping the mesh swaps `Mesh3d`.
#[test]
fn a_river_ribbon_slopes_and_swaps_its_mesh() {
    let (mut world, mut schedule) = world();
    let river = world.resource_mut::<Assets<Mesh>>().add(ribbon(32.0, 6.0, 4.0, 4.0));
    let at = Transform::from_xyz(10.0, 20.0, 30.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
    let body = world.spawn((WaterMeshSurface::still(river, WaterLook::default()), at, GlobalTransform::from(at), InheritedVisibility::VISIBLE)).id();
    schedule.run(&mut world);
    let aabb = *world.get::<Aabb>(body).expect("aabb");
    let [lo, hi] = world_box(&aabb, &GlobalTransform::from(at));
    // Local x 0..32 turns to world -z; local z ±3 to world x; heights 20 + (0..4).
    assert!((lo - Vec3::new(7.0, 20.0, -2.0)).length() < 1e-4 && (hi - Vec3::new(13.0, 24.0, 30.0)).length() < 1e-4, "{lo} {hi}");
    let surface = world.get::<WaterMeshSurface>(body).expect("body").clone();
    let placed = (world.get::<WaterMeshSurface>(body), Some((&aabb, &GlobalTransform::from(at))));
    match WaterBody::of(None, false, placed) {
        WaterBody::Mesh(w, _) => assert_eq!(w.level, 22.0),
        other => panic!("{other:?}"),
    }
    // A new mesh from the host (a rebuilt ribbon).
    let next = world.resource_mut::<Assets<Mesh>>().add(ribbon(40.0, 6.0, 4.0, 4.0));
    world.entity_mut(body).insert(WaterMeshSurface { mesh: next.clone(), ..surface });
    schedule.run(&mut world);
    assert_eq!(world.get::<Mesh3d>(body).map(|m| m.0.clone()), Some(next));
}
