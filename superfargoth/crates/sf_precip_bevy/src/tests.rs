#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn drop_mesh_has_four_vertices_per_drop_and_even_ranks() {
    let mesh = drop_mesh(1000, 1);
    assert_eq!(mesh.count_vertices(), 4000);
    let Some(bevy_mesh::VertexAttributeValues::Float32x4(drops)) = mesh.attribute(ATTRIBUTE_DROP) else {
        panic!("drop attribute");
    };
    // Ranks spread evenly: about 30 % of drops show at intensity 0.3.
    let shown = drops.iter().step_by(4).filter(|d| d[0] < 0.3).count();
    assert!((295..=305).contains(&shown), "{shown}");
}

#[test]
fn offset_wraps_and_follows_the_wind() {
    let slot = PrecipSlot { fall_speed: 7.0, ..PrecipSlot::default() };
    let v = velocity(&slot, Vec2::new(3.0, -1.0));
    assert_eq!(v, Vec3::new(3.0, -7.0, -1.0));
    let mut o = Vec3::ZERO;
    for _ in 0..1000 {
        o = advance(o, v, 1.0 / 60.0, 8.0);
        assert!(o.cmpge(Vec3::ZERO).all() && o.cmplt(Vec3::splat(8.0)).all(), "{o}");
    }
    // 1000 frames at 60 Hz: 50 m of x travel, 116.7 m of fall, wrapped.
    assert!((o.x - (50.0_f32).rem_euclid(8.0)).abs() < 1e-2, "{o}");
}

#[test]
fn layer_params_nest_and_light() {
    let flake = PrecipSlot {
        shape: PrecipShape::Flake,
        intensity: 1.5,
        flutter: 0.2,
        color: Vec4::new(1.0, 1.0, 1.0, 0.8),
        ..PrecipSlot::default()
    };
    let precip = Precip { brightness: 0.5, ..Precip::default() };
    let p0 = layer_params(&flake, 0, &precip, Vec3::ONE, 2.0);
    let p2 = layer_params(&flake, 2, &precip, Vec3::ONE, 2.0);
    assert_eq!(p0.volume, Vec4::new(8.0, 0.0, STREAK_S, 1.0));
    assert_eq!(p2.volume.y, 12.0, "far box skips the mid box");
    assert_eq!(p0.look.z, 1.0, "intensity clamped");
    assert_eq!(p0.color.truncate(), Vec3::splat(0.5));
    assert!(p2.color.w < p0.color.w, "far box more transparent");
    assert_eq!(p0.offset.w, 2.0);
    assert_eq!(p0.velocity.w, 0.2);
}

#[test]
fn occlusion_view_follows_the_travel_direction() {
    use super::occlusion::{along, travel, view_dir, MIN_ELEVATION};
    // Rain falling straight down: the old top-down map (eye above, depth 1 at the top).
    assert_eq!(view_dir(Vec3::new(0.0, -8.0, 0.0)), Vec3::NEG_Y);
    assert_eq!(view_dir(Vec3::ZERO), Vec3::NEG_Y);
    let center = Vec3::new(10.0, 5.0, -3.0);
    let (_, clip) = along(center, Vec3::NEG_Y);
    let roof = clip.project_point3(center + Vec3::Y * 4.0);
    let drop = clip.project_point3(center);
    assert!(roof.truncate().distance(drop.truncate()) < 1e-5 && roof.z > drop.z, "{roof} {drop}");
    // Slanted rain (8 m/s down, 3 m/s wind) keeps its own direction.
    let rain = view_dir(Vec3::new(3.0, -8.0, 0.0));
    assert!((rain - Vec3::new(3.0, -8.0, 0.0).normalize()).length() < 1e-6, "{rain}");
    // An ash storm (1 m/s down, 15 m/s wind along +x) blows near flat: the
    // view is lowered to MIN_ELEVATION and a wall upwind shelters the drop,
    // while the same wall straight above it would not.
    let ash = view_dir(Vec3::new(15.0, -1.0, 0.0));
    assert!((ash.y + MIN_ELEVATION.sin()).abs() < 1e-6 && ash.x > 0.9 && ash.z.abs() < 1e-6, "{ash}");
    let (_, clip) = along(center, ash);
    let drop = clip.project_point3(center);
    let wall = clip.project_point3(center - ash * 20.0);
    assert!(wall.truncate().distance(drop.truncate()) < 1e-4 && wall.z > drop.z, "{wall} {drop}");
    let above = clip.project_point3(center + Vec3::Y * 4.0);
    assert!(above.truncate().distance(drop.truncate()) > 0.05, "{above} {drop}");
    // The whole far box (64 m) fits the map's square and depth range.
    for corner in [Vec3::new(32.0, 32.0, 32.0), Vec3::new(-32.0, -32.0, 32.0), Vec3::new(32.0, -32.0, -32.0)] {
        for dir in [Vec3::NEG_Y, ash, rain] {
            let c = along(center, dir).1.project_point3(center + corner);
            assert!(c.x.abs() <= 1.0 && c.y.abs() <= 1.0 && c.z > 0.0 && c.z <= 1.0, "{dir} {corner} {c}");
        }
    }
    // The strongest falling slot steers the view.
    let ash_slot = PrecipSlot { fall_speed: 1.0, intensity: 0.9, ..PrecipSlot::default() };
    let rain_slot = PrecipSlot { fall_speed: 8.0, intensity: 0.2, wind_follow: 0.0, ..PrecipSlot::default() };
    let precip = Precip { slots: [Some(rain_slot), Some(ash_slot)], wind: Vec2::new(15.0, 0.0), ..Precip::default() };
    assert_eq!(travel(&precip), Some(ash));
    assert_eq!(travel(&Precip { enabled: false, ..precip }), None);
}

/// SW-6: the map centre snaps to the map's texels on the view's own axes (a stable cascade's
/// snap): a snapped centre stays, a move under half a texel keeps the centre and the projection,
/// a whole texel across the view moves a world point by exactly one texel.
#[test]
fn occlusion_centre_snaps_to_its_texels() {
    use super::occlusion::{along, snap_center, view_dir, MAP_SIZE, TEXEL_M};
    assert_eq!(TEXEL_M, 0.1875);
    for dir in [Vec3::NEG_Y, view_dir(Vec3::new(3.0, -8.0, 0.0)), view_dir(Vec3::new(15.0, -1.0, 4.0))] {
        let c = snap_center(Vec3::new(103.37, 21.9, -48.02), dir);
        assert_eq!(snap_center(c, dir), c, "{dir}: a snapped centre stays");
        for off in [Vec3::new(0.05, 0.0, 0.0), Vec3::new(0.0, -0.04, 0.03), Vec3::new(-0.03, 0.03, 0.03)] {
            let moved = snap_center(c + off, dir);
            assert_eq!(moved, c, "{dir} {off}");
            assert_eq!(along(moved, dir).1, along(c, dir).1, "{dir} {off}: same projection");
        }
        let (view, clip) = along(c, dir);
        let right = view.inverse().transform_vector3(Vec3::X);
        let next = snap_center(c + right * TEXEL_M, dir);
        let p = c + Vec3::new(5.0, 1.0, -3.0);
        let (a, b) = (clip.project_point3(p), along(next, dir).1.project_point3(p));
        #[allow(clippy::cast_precision_loss)]
        let texel_clip = 2.0 / MAP_SIZE as f32;
        assert!(((a.x - b.x).abs() - texel_clip).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4, "{dir}: {a} {b}");
    }
}

/// SW-6 (twin of `sf_surface_bevy`'s shelter test): an occlusion redraw makes its casters visible
/// (Bevy extracts only meshes some view sees: a roof behind the camera dropped out of the map),
/// only on redraw frames, and only a caster's finest LOD level casts (SW-5d).
#[test]
fn occlusion_redraw_makes_its_casters_visible() {
    use bevy_camera::visibility::{InheritedVisibility, ViewVisibility, VisibilityRange};
    use bevy_camera::{Camera, Camera3d};
    use bevy_light::DirectionalLight;
    use bevy_transform::components::GlobalTransform;
    let mut app = App::new();
    app.insert_resource(Assets::<bevy_image::Image>::default());
    let rain = PrecipSlot { intensity: 0.5, wind_follow: 0.0, ..PrecipSlot::default() };
    app.insert_resource(Precip { slots: [Some(rain), None], ..Precip::default() });
    occlusion::build(&mut app);
    app.world_mut().spawn(DirectionalLight { shadow_maps_enabled: true, ..Default::default() });
    let eye = Vec3::new(10.0, 2.0, -4.0);
    let cam = app.world_mut().spawn((Camera::default(), Camera3d::default(), GlobalTransform::from_translation(eye))).id();
    let caster = |app: &mut App, at: Vec3, range: Option<VisibilityRange>| {
        let e = app
            .world_mut()
            .spawn((Mesh3d(Handle::default()), GlobalTransform::from_translation(at), InheritedVisibility::VISIBLE, ViewVisibility::HIDDEN))
            .id();
        if let Some(r) = range {
            app.world_mut().entity_mut(e).insert(r);
        }
        e
    };
    // Behind and above the camera (no frustum holds it), and a tree's two levels 15 m away.
    let roof = caster(&mut app, eye + Vec3::new(-3.0, 4.0, 2.0), None);
    let near = caster(&mut app, eye + Vec3::new(15.0, 0.0, 0.0), Some(VisibilityRange::abrupt(0.0, 20.0)));
    let far = caster(&mut app, eye + Vec3::new(15.0, 0.0, 0.0), Some(VisibilityRange::abrupt(20.0, 200.0)));
    let visible = |app: &App, e| app.world().get::<ViewVisibility>(e).unwrap().get();
    app.update();
    let frame = app.world().resource::<occlusion::Frame>().clone();
    let req = frame.0.expect("first redraw");
    let mut ids: Vec<_> = req.casters.iter().map(|c| c.1.id()).collect();
    ids.sort_unstable();
    let mut expected = vec![roof, near];
    expected.sort_unstable();
    assert_eq!(ids, expected, "the finest level only");
    assert!(visible(&app, roof) && visible(&app, near) && !visible(&app, far));
    // A small move (under the 6 m recenter): no redraw, nothing marked.
    for e in [roof, near] {
        *app.world_mut().get_mut::<ViewVisibility>(e).unwrap() = ViewVisibility::HIDDEN;
    }
    *app.world_mut().get_mut::<GlobalTransform>(cam).unwrap() = GlobalTransform::from_translation(eye + Vec3::X * 3.0);
    app.update();
    assert!(app.world().resource::<occlusion::Frame>().0.is_none());
    assert!(!visible(&app, roof));
    assert_eq!(app.world().resource::<PrecipOcclusionStats>().redraws, 1);
}

/// SW-5d: the levels a fixed LOD distance picks (Bevy's `is_visible_at_all` at that distance),
/// for chains 0-30 / 30-120 / 120-400 m: 0 m the first, 150 m the third; no range always casts,
/// a proxy that starts beyond the distance never does.
#[test]
fn lod_casts_picks_the_level_at_the_fixed_distance() {
    use bevy_camera::visibility::VisibilityRange;
    let chain = [VisibilityRange::abrupt(0.0, 30.0), VisibilityRange::abrupt(30.0, 120.0), VisibilityRange::abrupt(120.0, 400.0)];
    let picks = |d: f32| chain.iter().map(|r| occlusion::lod_casts(Some(r), d)).collect::<Vec<_>>();
    assert_eq!(picks(occlusion::LOD_DISTANCE_M), [true, false, false]);
    assert_eq!(picks(150.0), [false, false, true]);
    assert!(occlusion::lod_casts(None, 0.0) && occlusion::lod_casts(None, 150.0));
    assert!(!occlusion::lod_casts(Some(&VisibilityRange::abrupt(400.0, 1.0e6)), occlusion::LOD_DISTANCE_M), "proxy only");
}

/// SW-5d: redraws centred 0, 16 and 48 m apart over one world draw the same casters: each
/// tree's finest level (0-30 m), never its 30-120 m level or its 400 m+ proxy, though the trees
/// stand 0-58 m from the centres (the SW-5 centre rule swapped levels between these redraws).
#[test]
fn occlusion_casters_are_the_finest_levels_for_any_centre() {
    use bevy_camera::visibility::{InheritedVisibility, ViewVisibility, VisibilityRange};
    use bevy_camera::{Camera, Camera3d};
    use bevy_light::DirectionalLight;
    use bevy_transform::components::GlobalTransform;
    let mut app = App::new();
    app.insert_resource(Assets::<bevy_image::Image>::default());
    let rain = PrecipSlot { intensity: 0.5, wind_follow: 0.0, ..PrecipSlot::default() };
    app.insert_resource(Precip { slots: [Some(rain), None], ..Precip::default() });
    occlusion::build(&mut app);
    app.world_mut().spawn(DirectionalLight { shadow_maps_enabled: true, ..Default::default() });
    let eye = Vec3::new(10.0, 2.0, -4.0);
    let cam = app.world_mut().spawn((Camera::default(), Camera3d::default(), GlobalTransform::from_translation(eye))).id();
    let spawn = |app: &mut App, at: Vec3, range: VisibilityRange| {
        app.world_mut()
            .spawn((Mesh3d(Handle::default()), GlobalTransform::from_translation(at), InheritedVisibility::VISIBLE, ViewVisibility::HIDDEN, range))
            .id()
    };
    let mut finest = Vec::new();
    for x in [-10.0, 0.0, 10.0, 20.0] {
        let at = eye + Vec3::new(x, 0.0, 3.0);
        finest.push(spawn(&mut app, at, VisibilityRange::abrupt(0.0, 30.0)));
        spawn(&mut app, at, VisibilityRange::abrupt(30.0, 120.0));
        spawn(&mut app, at, VisibilityRange::abrupt(400.0, 1.0e6));
    }
    finest.sort_unstable();
    let mut sets = Vec::new();
    for dx in [0.0, 16.0, 48.0] {
        *app.world_mut().get_mut::<GlobalTransform>(cam).unwrap() = GlobalTransform::from_translation(eye + Vec3::X * dx);
        app.update();
        let req = app.world().resource::<occlusion::Frame>().0.clone().expect("a move past the recenter redraws");
        let mut ids: Vec<_> = req.casters.iter().map(|c| c.1.id()).collect();
        ids.sort_unstable();
        sets.push(ids);
    }
    for (ids, dx) in sets.iter().zip([0, 16, 48]) {
        assert_eq!(ids, &finest, "centre {dx} m on: the finest levels only");
    }
}
