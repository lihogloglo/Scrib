//! `surface probe` headless (no render app, so no readback of its own): the request waits for a
//! frame whose maps and clips agree, spawns one readback per held map, and the bytes a test
//! hands back as the render world would (`ReadbackComplete`) print as world-anchored grids.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, clippy::float_cmp)]

use bevy_app::App;
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, Vec3};
use bevy_render::gpu_readback::ReadbackComplete;
use bevy_transform::components::GlobalTransform;
use sf_surface::Channel;
use sf_surface::probe::map_center;
use sf_surface::view::{NEAR_LEVEL, SHELTER_SIZE};

use crate::probe::{PROBE_TIMEOUT_FRAMES, ProbeState};
use crate::shelter::ShelterPending;
use crate::tests::app;
use crate::{ProbeStatus, SurfaceOrigin, SurfaceProbe, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera};

const SIZE: usize = SHELTER_SIZE as usize;

fn reading(app: &App) -> Vec<Entity> {
    app.world().resource::<ProbeState>().reading()
}

fn status(app: &App) -> ProbeStatus {
    app.world().resource::<SurfaceProbe>().status()
}

fn lines(app: &App) -> Vec<String> {
    app.world().resource::<SurfaceProbe>().lines().to_vec()
}

/// Updates until the probe spawned its readbacks; returns them and the frames it took.
fn until_reading(app: &mut App) -> (Vec<Entity>, u32) {
    for frame in 1..=10 {
        app.update();
        let entities = reading(app);
        if !entities.is_empty() {
            return (entities, frame);
        }
    }
    panic!("no readback spawned: {:?}", lines(app));
}

/// Hands a map's depths back as the render world does.
fn deliver(app: &mut App, entity: Entity, depth: &[f32]) {
    let data = depth.iter().flat_map(|d| d.to_le_bytes()).collect();
    app.world_mut().trigger(ReadbackComplete { entity, data });
}

/// A near map with one caster: every texel whose centre lies in world x 16.5..17, z 20..20.5,
/// top at world y = 9, for a map centred at `center`.
fn near_with_caster(center: DVec3) -> Vec<f32> {
    let level = NEAR_LEVEL;
    let texel = f64::from(level.texel_m());
    let half = f64::from(level.half_m);
    let mut depth = vec![0.0; SIZE * SIZE];
    let d = ((9.0 - (center.y - f64::from(level.down_m))) / f64::from(level.depth_m())) as f32;
    for r in 0..SIZE {
        let z = center.z - half + (r as f64 + 0.5) * texel;
        for c in 0..SIZE {
            let x = center.x - half + (c as f64 + 0.5) * texel;
            if (16.5..17.0).contains(&x) && (20.0..20.5).contains(&z) {
                depth[r * SIZE + c] = d;
            }
        }
    }
    depth
}

fn near_center(app: &App) -> DVec3 {
    let clip = app.world().resource::<SurfaceUniform>().shelter_clip_from_world;
    map_center(&NEAR_LEVEL, clip, app.world().resource::<SurfaceOrigin>().0.to_f64())
}

#[test]
#[allow(clippy::too_many_lines)]
fn probe_pairs_each_map_with_its_clip_and_prints_world_grids() {
    let mut app = app();
    app.update();
    // No camera, dry: no map to read, and the probe says so at once.
    app.world_mut().resource_mut::<SurfaceProbe>().request(None, 2.0);
    assert_eq!(status(&app), ProbeStatus::Pending);
    app.update();
    assert_eq!(status(&app), ProbeStatus::Done);
    let out = lines(&app);
    assert_eq!(out.len(), 1);
    assert!(out[0].starts_with("surface probe: no shelter map held"), "{out:?}");

    // A camera at world (16, 5, 20) (the origin is one 256 m cell east) and a wet layer: the
    // maps are drawn on the next frame. A probe asked now must not read on that frame (the
    // uniform still holds the old clip, here none), only once the uniform took the redraw's.
    let camera = app
        .world_mut()
        .spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(-240.0, 5.0, 20.0))))
        .id();
    sf_weather_bevy::surface_set(app.world_mut(), Channel::Wet, 0.5, "land").unwrap();
    app.world_mut().resource_mut::<SurfaceProbe>().request(None, 2.0);
    app.update();
    assert_eq!(*app.world().resource::<ShelterPending>(), ShelterPending { near: true, far: true }, "redraws requested this frame");
    assert!(reading(&app).is_empty(), "no readback on a redraw's frame");
    assert_eq!(status(&app), ProbeStatus::Pending);
    let (entities, _) = until_reading(&mut app);
    assert_eq!(*app.world().resource::<ShelterPending>(), ShelterPending::default());
    assert_eq!(entities.len(), 4, "near solid, near leaves, far solid, far leaves");
    let u = *app.world().resource::<SurfaceUniform>();
    assert!(u.shelter.x > 0.0 && u.far_shelter.x > 0.0, "both levels hold a map");
    let center = near_center(&app);
    // X and Z on the world texel grid exactly; Y as the clip's `f32`s hold it.
    assert_eq!((center.x, center.z), (16.0, 20.0));
    assert!((center.y - 5.0).abs() < 1e-3, "{center:?}");

    // The bytes come back (in any order, over several frames): the grids print when all did.
    let empty = vec![0.0; SIZE * SIZE];
    deliver(&mut app, entities[3], &empty);
    deliver(&mut app, entities[0], &near_with_caster(center));
    app.update();
    assert_eq!(status(&app), ProbeStatus::Pending);
    assert_eq!(reading(&app), [entities[1], entities[2]]);
    deliver(&mut app, entities[1], &empty);
    deliver(&mut app, entities[2], &empty);
    app.update();
    assert_eq!(status(&app), ProbeStatus::Done);
    assert!(reading(&app).is_empty());
    assert!(entities.iter().all(|e| app.world().get_entity(*e).is_err()), "the readback entities are gone");
    let out = lines(&app);
    assert!(out.iter().all(|l| l.starts_with("surface probe: ")), "{out:#?}");
    assert!(out[0].starts_with("surface probe: window +-2 m around x 16.00 z 20.00, camera x 16.00 y 5.00 z 20.00, redraws so far near 1 far 1"), "{}", out[0]);
    assert!(out[1].starts_with("surface probe: legend:"), "{}", out[1]);
    // Four grids: near at 0.5 m cells (8x8 over +-2 m), far at 1 m cells (4x4), each after its
    // header; the caster (top 4 m over the camera: `2`) in its world cell of the near solid map.
    assert_eq!(out.len(), 2 + 4 + 8 + 8 + 4 + 4);
    assert_eq!(
        out[2],
        "surface probe: near solid: first cell x 14.25 z 18.25, cell 0.5 m, 8x8 cells, map centre x 16.000 z 20.000 y 5.000, \
         camera x 16.00 y 5.00 z 20.00, 16 caster texels of 1048576"
    );
    let cells = |l: &String| l.rsplit(' ').next().unwrap().to_owned();
    let near_solid: Vec<String> = out[3..11].iter().map(cells).collect();
    assert_eq!(near_solid, ["........", "........", "........", "........", ".....2..", "........", "........", "........"]);
    assert_eq!(out[7], "surface probe:      20.25 .....2..");
    assert!(out[11].starts_with("surface probe: near leaves: first cell x 14.25 z 18.25, cell 0.5 m, 8x8 cells"), "{}", out[11]);
    assert!(out[12..20].iter().all(|l| cells(l) == "........"));
    assert!(out[20].starts_with("surface probe: far solid: first cell x 14.50 z 18.50, cell 1 m, 4x4 cells, map centre x 16.000 z 20.000 y 5.000"), "{}", out[20]);
    assert!(out[21..25].iter().all(|l| cells(l) == "...."));
    assert!(out[25].starts_with("surface probe: far leaves: "), "{}", out[25]);

    // Steady frames: a second probe (a named window this time) reads at once. The camera then
    // moves 20 m, the near level redraws before the bytes are back: that result is thrown away
    // and the maps are read again once the uniform took the new clip; the same world caster then
    // prints in the same cells from the moved map.
    app.world_mut().resource_mut::<SurfaceProbe>().request(Some(bevy_math::DVec2::new(16.0, 20.0)), 2.0);
    let (first, frames) = until_reading(&mut app);
    assert_eq!(frames, 1);
    let redraws = app.world().resource::<SurfaceStats>().shelter_redraws;
    *app.world_mut().get_mut::<GlobalTransform>(camera).unwrap() = GlobalTransform::from_translation(Vec3::new(-220.0, 8.0, 28.0));
    app.update();
    assert_eq!(app.world().resource::<SurfaceStats>().shelter_redraws, redraws + 1);
    let stale = near_with_caster(center);
    deliver(&mut app, first[0], &stale);
    for e in &first[1..] {
        deliver(&mut app, *e, &empty);
    }
    let (second, _) = until_reading(&mut app);
    assert_eq!(status(&app), ProbeStatus::Pending, "the overtaken result printed nothing");
    assert!(second.iter().all(|e| !first.contains(e)));
    assert_eq!(*app.world().resource::<ShelterPending>(), ShelterPending::default());
    let moved = near_center(&app);
    assert_eq!((moved.x, moved.z), (36.0, 28.0));
    assert!((moved.y - 8.0).abs() < 1e-3, "{moved:?}");
    deliver(&mut app, second[0], &near_with_caster(moved));
    for e in &second[1..] {
        deliver(&mut app, *e, &empty);
    }
    app.update();
    assert_eq!(status(&app), ProbeStatus::Done);
    let again = lines(&app);
    assert!(again[2].contains("map centre x 36.000 z 28.000 y 8.000, camera x 36.00 y 8.00 z 28.00"), "{}", again[2]);
    // The camera rose 3 m: the same top is 1 m over it now (`1`), in the same world cell.
    let near_solid: Vec<String> = again[3..11].iter().map(cells).collect();
    assert_eq!(near_solid, ["........", "........", "........", "........", ".....1..", "........", "........", "........"]);
    assert_eq!(again[3..11].iter().map(|l| &l[..25]).collect::<Vec<_>>(), out[3..11].iter().map(|l| &l[..25]).collect::<Vec<_>>(), "same rows");

    // A readback that never comes back (as here, with no render app): the probe gives up.
    app.world_mut().resource_mut::<SurfaceProbe>().request(None, 2.0);
    let (lost, _) = until_reading(&mut app);
    for _ in 0..PROBE_TIMEOUT_FRAMES {
        app.update();
    }
    assert_eq!(status(&app), ProbeStatus::Done);
    let out = lines(&app);
    assert_eq!(out.len(), 1);
    assert!(out[0].starts_with(&format!("surface probe: gave up after {PROBE_TIMEOUT_FRAMES} frames: 0 of 4 readbacks came back")), "{out:?}");
    assert!(lost.iter().all(|e| app.world().get_entity(*e).is_err()));
}
