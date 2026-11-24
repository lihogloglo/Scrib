//! The deformer capture (S1a), headless: the request carries only [`Deformer`] meshes (skinned
//! and held items included, none of the shelter's casters), marks them visible, hands its
//! placement to the next frame's ring pass, lists sweep gaps; the ground hook and its fallback.
#![allow(clippy::float_cmp)]

use std::sync::Arc;

use bevy_app::App;
use bevy_asset::Handle;
use bevy_camera::Camera;
use bevy_camera::visibility::{InheritedVisibility, ViewVisibility};
use bevy_ecs::prelude::*;
use bevy_math::{DVec2, Vec2, Vec3, Vec4};
use bevy_mesh::Mesh3d;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;

use crate::deform::{DeformerFrame, ground_grid};
use crate::tests::app;
use crate::*;

fn mesh(app: &mut App, at: Vec3) -> Entity {
    app.world_mut()
        .spawn((Mesh3d(Handle::default()), GlobalTransform::from_translation(at), InheritedVisibility::VISIBLE, ViewVisibility::HIDDEN))
        .id()
}

/// Snow on the cover cell under render position `at` (the capture's gate).
fn snow_under(app: &mut App, at: Vec3) {
    let world = app.world().resource::<SurfaceOrigin>().0.offset(at).to_f64();
    let mut s = app.world_mut().resource_mut::<WorldSurface>();
    let ws = s.grid().expect("the map's cell grid").worldspace().clone();
    s.state.set_cell(&ws, sf_surface::cell_of(DVec2::new(world.x, world.z)), sf_surface::Channel::Snow, 1.0);
    s.touch();
}

fn casters(app: &App) -> Option<Vec<Entity>> {
    let req = app.world().resource::<DeformerFrame>().0.clone()?;
    let mut v: Vec<Entity> = req.casters.iter().map(|c| c.1.id()).collect();
    v.sort();
    Some(v)
}

#[test]
fn capture_draws_only_deformer_meshes() {
    let mut app = app();
    app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(0.0, 2.0, 3.0))));
    let root = app.world_mut().spawn((Deformer { width_m: 0.15 }, GlobalTransform::from_translation(Vec3::new(1.0, 0.0, 1.0)))).id();
    let body = mesh(&mut app, Vec3::new(1.0, 0.9, 1.0));
    app.world_mut().entity_mut(body).insert((SkinnedMesh::default(), ChildOf(root)));
    let item = mesh(&mut app, Vec3::new(1.4, 0.8, 1.0));
    let hand = app.world_mut().spawn((GlobalTransform::default(), ChildOf(root))).id();
    app.world_mut().entity_mut(item).insert(ChildOf(hand));
    // A rock beside the actor (a shelter caster) and an actor out of range.
    let _rock = mesh(&mut app, Vec3::new(1.5, 0.2, 1.0));
    let far = app.world_mut().spawn((Deformer { width_m: 0.15 }, GlobalTransform::from_translation(Vec3::new(30.0, 0.0, 0.0)))).id();
    let far_body = mesh(&mut app, Vec3::new(30.0, 0.9, 0.0));
    app.world_mut().entity_mut(far_body).insert(ChildOf(far));
    app.update();
    assert_eq!(casters(&app), None, "no cover look: trails off, no capture");
    app.world_mut().resource_mut::<SurfaceUniform>().cover[0] = Vec4::new(0.25, 0.4, 1.0, 1.0);
    app.update();
    assert_eq!(casters(&app), None, "a cover look but no snow on the ground: no capture (S1b gate)");
    snow_under(&mut app, Vec3::new(1.0, 0.0, 1.0));
    app.update();
    let mut want = vec![body, item];
    want.sort();
    assert_eq!(casters(&app), Some(want), "the actor's skinned body and held item; not the rock, not the far actor");
    let visible = |app: &App, e| app.world().get::<ViewVisibility>(e).unwrap().get();
    assert!(visible(&app, body) && visible(&app, item), "extracted although no view sees them");
    // (The rock and the far body are the shelter's casters once snow lies: its view marks them, not the capture.)
    let req = app.world().resource::<DeformerFrame>().0.clone().unwrap();
    assert_eq!((req.size, req.subview), (sf_surface::deform::MAX_CAPTURE_SIZE, DEFORM_SUBVIEW), "the request names the whole target: the shadow pass draws over all of it");
    assert_eq!(stats_of(&app).deform_casters, 2);
    // This frame's ring pass merges last frame's capture.
    assert!(app.world().resource::<TrailFrame>().capture.is_none());
    app.update();
    let (cap, _) = app.world().resource::<TrailFrame>().capture.clone().expect("last frame's capture");
    assert_eq!(cap.size, 256);
    // Window around the player (the deformer root nearest the camera), on whole texels.
    let half = 128.0 * 0.0625;
    assert!((cap.corner + Vec2::splat(half) - Vec2::new(1.0, 1.0)).abs().max_element() <= 0.0625, "{:?}", cap.corner);
    // No ground hook: a plane through the player's root (y 0), slab under it to the cover's top.
    assert_eq!(cap.ground.range(), (0.0, 0.0));
    assert!((cap.near_y + 0.25).abs() < 1e-6);
    assert!((cap.near_y + cap.range_m - 0.4 * 1.25).abs() < 1e-5);
    // Sweep gap: the root jumps 1 m in a frame (0.15 m wide).
    *app.world_mut().get_mut::<GlobalTransform>(root).unwrap() = GlobalTransform::from_translation(Vec3::new(2.0, 0.0, 1.0));
    app.update();
    let gaps = app.world().resource::<DeformGaps>().0.clone();
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].root, root);
    app.update();
    assert!(app.world().resource::<DeformGaps>().0.is_empty(), "standing still");
    // `NoDeform` (a view-model copy): left out.
    app.world_mut().entity_mut(item).insert(NoDeform);
    app.update();
    assert_eq!(casters(&app), Some(vec![body]));
    // Off: no capture, nothing merged next frame.
    app.world_mut().resource_mut::<DeformSettings>().on = false;
    app.update();
    assert_eq!(casters(&app), None);
    app.update();
    assert!(app.world().resource::<TrailFrame>().capture.is_none());
}

/// GC-T2b: a press demand runs the capture with no cover look and no snow under the window, its
/// slab reaching the press height at ring depth 0, and hands the press height to the ring pass;
/// without a demand nor cover nothing is captured.
#[test]
fn a_press_demand_runs_the_capture_without_cover() {
    use crate::deform::capture_wanted;
    assert!(capture_wanted(0.5, || false), "demand, no cover");
    assert!(capture_wanted(0.0, || true), "cover, no demand");
    assert!(!capture_wanted(0.0, || false));
    assert!(capture_wanted(0.5, || panic!("the cover lookup is skipped on a demand")));
    let mut app = app();
    app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(0.0, 2.0, 3.0))));
    let root = app.world_mut().spawn((Deformer { width_m: 0.15 }, GlobalTransform::from_translation(Vec3::new(1.0, 0.0, 1.0)))).id();
    let body = mesh(&mut app, Vec3::new(1.0, 0.9, 1.0));
    app.world_mut().entity_mut(body).insert(ChildOf(root));
    app.update();
    assert_eq!(casters(&app), None, "no demand, no cover look: no capture");
    app.world_mut().resource_mut::<TrailDemand>().press = true;
    app.update();
    assert_eq!(casters(&app), Some(vec![body]), "a press demand: the capture runs over bare ground");
    app.update();
    let (cap, _) = app.world().resource::<TrailFrame>().capture.clone().expect("last frame's capture");
    let height = app.world().resource::<TrailPressSettings>().height_m;
    assert_eq!(cap.press_m, height);
    assert_eq!(cap.spread_m, app.world().resource::<TrailPressSettings>().spread_m, "GC-T3: the spread handed over");
    assert_eq!(app.world().resource::<TrailFrame>().depth_m, 0.0, "press only: ring depth 0");
    assert!((cap.near_y + 0.25).abs() < 1e-6);
    assert!(cap.range_m >= 0.25 + height - 1e-5 && (cap.near_y + cap.range_m - height).abs() < 1e-5, "slab to the press height: {cap:?}");
    // Demand withdrawn: no capture again.
    app.world_mut().resource_mut::<TrailDemand>().press = false;
    app.update();
    assert_eq!(casters(&app), None);
}

fn stats_of(app: &App) -> SurfaceStats {
    *app.world().resource::<SurfaceStats>()
}

#[test]
fn ground_grid_reads_the_hook_in_render_space() {
    let origin = WorldPos::from_f64(bevy_math::DVec3::new(100.0, 10.0, -50.0));
    let slope = DeformGround(Arc::new(|p: DVec2| (p.x > 0.0).then_some(20.0 + 0.5 * p.x)));
    let (g, fell) = ground_grid(Some(&slope), origin, DVec2::new(102.0, -48.0), 4.0, 0.5, -3.0);
    assert!(!fell);
    assert_eq!(g.edge, 11);
    // Render (2.5, 2.5) is world (102.5, ...): 20 + 51.25 - 10.
    assert!((g.at(Vec2::new(2.5, 2.5)) - 61.25).abs() < 1e-3, "{}", g.at(Vec2::new(2.5, 2.5)));
    let (g, fell) = ground_grid(Some(&slope), origin, DVec2::new(-1.0, 0.0), 4.0, 0.5, -3.0);
    assert!(fell, "unknown west of x = 0");
    assert_eq!(g.range().0, -3.0);
    let (g, fell) = ground_grid(None, origin, DVec2::ZERO, 4.0, 0.5, -3.0);
    assert!(fell);
    assert_eq!(g.range(), (-3.0, -3.0));
}

/// trail-offset (owner 2026-11-17: prints under the feet, the capture's groove a few meters
/// beside the player), through the systems with a floating origin off in X and Y and one
/// recentre: a foot stamp prints the ring texel holding the foot; the capture's request matrix
/// (what `sun_view` hands the shadow pass, drawn over the whole target) puts a leg's world point
/// in the texel the ring pass reads for that world point (`Capture::texel_of`, `trail.wgsl`
/// `captured`); after a recentre the merge drops one frame and the window stays on the world.
#[test]
#[allow(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::manual_midpoint
)]
fn stamps_and_capture_land_under_the_actor_across_a_recentre() {
    use bevy_math::{IVec2, IVec3, Mat4};
    use sf_surface::deform::MAX_CAPTURE_SIZE;
    use sf_surface::trail::{FOOT_HALF_M, RING_SIZE, RING_TEXEL_M, TrailPass};
    let mut app = app();
    app.world_mut().resource_mut::<SurfaceOrigin>().0 = WorldPos::new(IVec3::new(1, 1, 0), Vec3::ZERO);
    let focus = Vec3::new(37.3, -211.6, 100.7);
    let cam = app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(focus + Vec3::new(0.0, 1.6, 3.0)))).id();
    let root = app.world_mut().spawn((Deformer { width_m: 0.15 }, GlobalTransform::from_translation(focus))).id();
    let leg = mesh(&mut app, focus + Vec3::new(-0.1, 0.3, 0.1));
    app.world_mut().entity_mut(leg).insert(ChildOf(root));
    app.world_mut().resource_mut::<SurfaceUniform>().cover[0] = Vec4::new(0.25, 0.4, 1.0, 1.0);
    app.update();
    snow_under(&mut app, focus);
    app.update();
    let origin = app.world().resource::<SurfaceOrigin>().0;
    let foot = focus + Vec3::new(0.12, 0.0, -0.2);
    app.world_mut().write_message(TrailStamp { at: origin.offset(foot), forward: Vec2::Y, half_m: FOOT_HALF_M, depth: 1.0 });
    app.update();
    let n = RING_SIZE;
    let ring_addr = |w: bevy_math::DVec3| (DVec2::new(w.x, w.z) / RING_TEXEL_M).floor().as_ivec2().rem_euclid(IVec2::splat(n));
    // The foot print: the deepest ring texel holds the foot's world point.
    let f = app.world().resource::<TrailFrame>().clone();
    assert_eq!(f.stamps.len(), 1);
    let pass = TrailPass { corner: f.corner, depth_m: f.depth_m, new_min: f.new_min, old_min: None, refill_m: 0.0, stamps: f.stamps.clone(), capture: None, press: sf_surface::trail::Press::default() };
    let r = pass.run(&vec![[0.0; 2]; (n * n) as usize]);
    // The print's texels (the ring wraps far from them) centre on the foot's texel, which is
    // printed to the full depth.
    let printed: Vec<IVec2> = (0..n * n).filter(|&i| r[i as usize][0] < 0.0).map(|i| IVec2::new(i % n, i / n)).collect();
    assert!(!printed.is_empty());
    let mid = printed.iter().map(|a| a.as_vec2() + 0.5).sum::<Vec2>() / printed.len() as f32;
    let at = ring_addr(origin.offset(foot).to_f64());
    assert!((mid - (at.as_vec2() + 0.5)).abs().max_element() <= 1.0, "the print centres on {mid}, the foot's texel is {at}");
    assert!((r[(at.y * n + at.x) as usize][0] + f.depth_m).abs() < 1e-4, "printed to the ground under the foot");
    // The capture: drawn this frame, merged next frame.
    let req = app.world().resource::<DeformerFrame>().0.clone().expect("a capture");
    app.update();
    let (cap, _) = app.world().resource::<TrailFrame>().capture.clone().expect("last frame's capture");
    // The shadow pass draws over the whole attachment (no viewport), whatever the request says.
    let e = MAX_CAPTURE_SIZE;
    assert_eq!(req.size, e, "the request names the target edge");
    let raster = |clip: Mat4, p: Vec3| {
        let ndc = clip.project_point3(p);
        Vec2::new((ndc.x + 1.0) * 0.5 * e as f32, (1.0 - ndc.y) * 0.5 * e as f32).floor().as_uvec2()
    };
    let f = app.world().resource::<TrailFrame>().clone();
    for p in [focus + Vec3::new(-0.1, 0.3, 0.1), focus + Vec3::new(0.2, 0.05, -0.3), focus + Vec3::new(-0.4, 0.1, -0.05)] {
        let drawn = raster(req.clip_from_world, p);
        assert_eq!(Some(drawn), cap.texel_of(Vec2::new(p.x, p.z)), "drawn where the ring reads it ({p})");
        // The ring texel holding this world point reads that same capture texel at its centre.
        let a = ring_addr(origin.offset(p).to_f64());
        let rel = (a - f.new_min).rem_euclid(IVec2::splat(n)).as_vec2() + 0.5;
        let q = f.corner + rel * RING_TEXEL_M as f32;
        assert_eq!(cap.texel_of(q), Some(drawn), "the ring texel under {p} reads the capture's texel under it");
    }
    // Recentre: render space moves a cell; the actor stays put in the world.
    let corner_world = |app: &App, c: Vec2| {
        let o = app.world().resource::<SurfaceOrigin>().0.to_f64();
        DVec2::new(f64::from(c.x) + o.x, f64::from(c.y) + o.z)
    };
    let before = corner_world(&app, cap.corner);
    app.world_mut().resource_mut::<SurfaceOrigin>().0 = WorldPos::new(IVec3::new(2, 1, 0), Vec3::ZERO);
    let shift = Vec3::new(-sf_coords::GRID_CELL, 0.0, 0.0);
    for (e, at) in [(cam, focus + Vec3::new(0.0, 1.6, 3.0)), (root, focus), (leg, focus + Vec3::new(-0.1, 0.3, 0.1))] {
        *app.world_mut().get_mut::<GlobalTransform>(e).unwrap() = GlobalTransform::from_translation(at + shift);
    }
    app.update();
    assert!(app.world().resource::<TrailFrame>().capture.is_none(), "a capture placed in the old render space is dropped");
    app.update();
    let (cap, _) = app.world().resource::<TrailFrame>().capture.clone().expect("captured again");
    assert_eq!(corner_world(&app, cap.corner), before, "the window holds its world texels");
}
