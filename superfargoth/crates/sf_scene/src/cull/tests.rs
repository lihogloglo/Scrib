#![allow(clippy::cast_precision_loss)]

use glam::{IVec3, Vec2, Vec3};

use super::*;
use crate::material::MaterialClass;
use crate::table::{LodChain, LodLevel, Sphere, StaticCellTable};

const FOV: f32 = std::f32::consts::FRAC_PI_3;

fn frustum(eye: Vec3, forward: Vec3) -> Frustum {
    Frustum::perspective(eye, forward, Vec3::Y, FOV, 16.0 / 9.0, 0.1, 1000.0)
}

fn view(eye: Vec3, forward: Vec3) -> SceneView {
    SceneView {
        eye,
        frustum: frustum(eye, forward),
        layers: 1,
        projection: SceneProjection::Perspective { pixels_per_radian: pixels_per_radian(1440.0, FOV) },
        lod_px: 1.0,
        size_cull_px: 1.0,
        horizon: false,
    }
}

/// The view's pixels per radian (every [`view`] is perspective).
fn ppr_of(v: &SceneView) -> f32 {
    match v.projection {
        SceneProjection::Perspective { pixels_per_radian } => pixels_per_radian,
        SceneProjection::Orthographic { .. } => unreachable!("the test views are perspective"),
    }
}

/// A sun-cascade-like orthographic view: a `half_extent` metre box of `map_px` texels, looking
/// down −Z from `center`.
fn ortho_view(center: Vec3, map_px: f32, half_extent: f32) -> SceneView {
    SceneView {
        eye: center,
        frustum: Frustum::orthographic(center, Vec3::NEG_Z, Vec3::Y, half_extent, half_extent, -half_extent, half_extent),
        layers: 1,
        projection: SceneProjection::Orthographic { texels_per_metre: texels_per_metre(map_px, 2.0 * half_extent) },
        lod_px: 1.0,
        size_cull_px: 1.0,
        horizon: false,
    }
}

fn inst(placement: u32, center: Vec3, radius: f32) -> StaticInstance {
    StaticInstance {
        placement,
        transform: [[1.0, 0.0, 0.0, center.x], [0.0, 1.0, 0.0, center.y], [0.0, 0.0, 1.0, center.z]],
        sphere: Sphere { center, radius },
        chain: 0,
        material_override: 0,
        flags: InstanceFlags::empty(),
    }
}

#[test]
fn pixels_per_radian_matches_the_lod_screen_formula() {
    // sf_model LodScreen::default(): 1440 px, 70°.
    let ppr = pixels_per_radian(1440.0, 70f32.to_radians());
    assert!((ppr - 1440.0 / (2.0 * 35f32.to_radians().tan())).abs() < 1e-3);
    assert!((pixels_per_radian(1080.0, FOV) - 1080.0 / (2.0 * (FOV / 2.0).tan())).abs() < 1e-3);
}

#[test]
fn presets_are_the_spec_table() {
    let lod: Vec<f32> = ScenePreset::ALL.iter().map(|p| p.lod_px()).collect();
    assert_eq!(lod, [0.5, 1.0, 2.0, 4.0]);
    // The sub-pixel cutoff is per view: a preset's number is the main view's, and a shadow view
    // gets 0 (off) until a per-cascade caster rule is measured (`ScenePreset::size_cull_px`).
    let main: Vec<f32> = ScenePreset::ALL.iter().map(|p| p.size_cull_px(ViewKind::Main)).collect();
    assert_eq!(main, lod);
    let shadow: Vec<f32> = ScenePreset::ALL.iter().map(|p| p.size_cull_px(ViewKind::Shadow)).collect();
    assert_eq!(shadow, [0.0; 4]);
    let v = view(Vec3::ZERO, Vec3::NEG_Z).with_preset(ScenePreset::Low, ViewKind::Main);
    assert_eq!((v.lod_px, v.size_cull_px), (4.0, 4.0));
    // The LOD knob still applies in a cascade: only the size cull defaults off.
    let cascade = ortho_view(Vec3::ZERO, 2048.0, 64.0).with_preset(ScenePreset::Low, ViewKind::Shadow);
    assert_eq!((cascade.lod_px, cascade.size_cull_px), (4.0, 0.0));
}

/// The sub-pixel cull is off by default in shadow views, in both of their projections (sun
/// cascades are orthographic, lamp cube faces perspective), and `0` is what turns it off. The
/// rule itself is untouched: the same preset still culls in a main view, and setting
/// `size_cull_px` on a shadow view turns it back on for the owner's visual check.
#[test]
fn shadow_views_keep_every_caster_by_default() {
    let speck = inst(1, Vec3::new(0.0, 0.0, -10.0), 1e-4);
    let far = inst(1, Vec3::new(0.0, 0.0, -900.0), 0.1);
    let cascade = ortho_view(Vec3::ZERO, 2048.0, 64.0).with_preset(ScenePreset::Low, ViewKind::Shadow);
    assert_eq!(cull_instance(&cascade, &speck, Vec3::ZERO, [0.0]), Verdict::Draw { level: 0 });
    let face = view(Vec3::ZERO, Vec3::NEG_Z).with_preset(ScenePreset::Low, ViewKind::Shadow);
    assert_eq!(cull_instance(&face, &far, Vec3::ZERO, [0.0]), Verdict::Draw { level: 0 });
    // The same preset in a main view still drops both: only the default changed.
    let main = view(Vec3::ZERO, Vec3::NEG_Z).with_preset(ScenePreset::Low, ViewKind::Main);
    assert_eq!(cull_instance(&main, &far, Vec3::ZERO, [0.0]), Verdict::Culled(CullReason::SubPixel));
    let turned_on = SceneView { size_cull_px: ScenePreset::Low.size_cull_px(ViewKind::Main), ..cascade };
    assert_eq!(cull_instance(&turned_on, &speck, Vec3::ZERO, [0.0]), Verdict::Culled(CullReason::SubPixel));
}

#[test]
fn lod_pick_is_monotone_and_never_out_of_range() {
    let errors = [0.0, 0.01, 0.05, 0.2];
    let ppr = pixels_per_radian(1440.0, FOV);
    let mut prev = 0;
    for step in 0..2000 {
        #[allow(clippy::cast_precision_loss)]
        let d = step as f32 * 0.5;
        let level = pick_level(errors, d, ppr, 1.0).unwrap();
        assert!(level < errors.len());
        assert!(level >= prev, "distance {d}: level {level} after {prev}");
        prev = level;
    }
    assert_eq!(prev, 3);
    assert_eq!(pick_level(errors, 0.0, ppr, 1.0), Some(0));
    // The switch is exactly where e · ppr / lod_px = d.
    let switch = 0.05 * ppr;
    assert_eq!(pick_level(errors, switch * 1.001, ppr, 1.0), Some(2));
    assert_eq!(pick_level(errors, switch * 0.999, ppr, 1.0), Some(1));
    // A chain with no level has no pick; one level is always level 0.
    assert_eq!(pick_level([0.0f32; 0], 1e9, ppr, 1.0), None);
    assert_eq!(pick_level([5.0], 0.0, ppr, 1.0), Some(0));
    // Non-monotone errors still give an existing level, the coarsest passing one.
    assert_eq!(pick_level([0.0, 0.5, 0.01], 20.0, ppr, 1.0), Some(2));
}

#[test]
fn lod_pick_scales_with_resolution_fov_and_preset() {
    let errors = [0.0, 0.1];
    let switch = |ppr: f32, lod_px: f32| {
        // Smallest distance (to 1 cm) picking level 1.
        (0..1_000_000).map(|cm| cm as f32 / 100.0).find(|&d| pick_level(errors, d, ppr, lod_px) == Some(1)).unwrap()
    };
    let base = switch(pixels_per_radian(1440.0, FOV), 1.0);
    let double_res = switch(pixels_per_radian(2880.0, FOV), 1.0);
    let narrow = switch(pixels_per_radian(1440.0, FOV / 2.0), 1.0);
    let low = switch(pixels_per_radian(1440.0, FOV), ScenePreset::Low.lod_px());
    assert!((double_res / base - 2.0).abs() < 1e-3, "{double_res} vs {base}");
    let fov_ratio = (FOV / 2.0).tan() / (FOV / 4.0).tan();
    assert!((narrow / base - fov_ratio).abs() < 1e-3);
    assert!((low / base - 0.25).abs() < 1e-3);
}

#[test]
fn frustum_sphere_and_box() {
    let f = frustum(Vec3::new(0.0, 1.0, 0.0), Vec3::NEG_Z);
    let eye = Vec3::new(0.0, 1.0, 0.0);
    assert!(!f.sphere_outside(eye + Vec3::new(0.0, 0.0, -10.0), 1.0));
    assert!(f.sphere_outside(eye + Vec3::new(0.0, 0.0, 10.0), 1.0), "behind");
    assert!(f.sphere_outside(eye + Vec3::new(0.0, 0.0, -1002.0), 1.0), "past far");
    assert!(f.sphere_outside(eye + Vec3::new(0.0, 0.0, -0.01), 0.05), "before near");
    // Left edge at depth 10: x = −10 · tan(fov/2) · aspect.
    let edge = 10.0 * (FOV / 2.0).tan() * 16.0 / 9.0;
    assert!(!f.sphere_outside(eye + Vec3::new(-edge - 0.9, 0.0, -10.0), 1.0), "straddles the left plane");
    assert!(f.sphere_outside(eye + Vec3::new(-edge - 1.5, 0.0, -10.0), 1.0));
    // Top edge: y = 10 · tan(fov/2).
    let top = 10.0 * (FOV / 2.0).tan();
    assert!(f.sphere_outside(eye + Vec3::new(0.0, top + 1.5, -10.0), 1.0));
    assert!(!f.sphere_outside(eye + Vec3::new(0.0, top + 0.9, -10.0), 1.0));
    let boxed = |min: Vec3, max: Vec3| f.aabb_outside(&Aabb { min: eye + min, max: eye + max });
    assert!(!boxed(Vec3::new(-1.0, -1.0, -20.0), Vec3::new(1.0, 1.0, -10.0)));
    assert!(boxed(Vec3::new(-1.0, -1.0, 5.0), Vec3::new(1.0, 1.0, 10.0)));
    // A box around the eye is never outside.
    assert!(!boxed(Vec3::splat(-1.0), Vec3::splat(1.0)));
    // Infinite far plane: nothing is past it.
    let inf = Frustum::perspective(eye, Vec3::NEG_Z, Vec3::Y, FOV, 1.0, 0.1, f32::INFINITY);
    assert!(!inf.sphere_outside(eye + Vec3::new(0.0, 0.0, -1e7), 1.0));
}

#[test]
fn sub_pixel_cull_at_the_threshold() {
    let v = view(Vec3::ZERO, Vec3::NEG_Z);
    let r = 0.1;
    // Projected radius ppr · r / √(d² − r²) equals 1 px at d = r √(ppr² + 1).
    let d_px = r * (ppr_of(&v).powi(2) + 1.0).sqrt();
    let at = |d: f32| cull_instance(&v, &inst(1, Vec3::new(0.0, 0.0, -d), r), Vec3::ZERO, [0.0]);
    assert_eq!(at(d_px * 0.99), Verdict::Draw { level: 0 });
    assert_eq!(at(d_px * 1.01), Verdict::Culled(CullReason::SubPixel));
    // Off, nothing is size-culled; the eye inside a sphere never is.
    let off = SceneView { size_cull_px: 0.0, ..v };
    assert_eq!(cull_instance(&off, &inst(1, Vec3::new(0.0, 0.0, -900.0), r), Vec3::ZERO, [0.0]), Verdict::Draw { level: 0 });
    assert_eq!(cull_instance(&v, &inst(1, Vec3::new(0.0, 0.0, -0.5), 1.0), Vec3::ZERO, [0.0]), Verdict::Draw { level: 0 });
    // Coarser presets cull nearer.
    let low = v.with_preset(ScenePreset::Low, ViewKind::Main);
    assert_eq!(cull_instance(&low, &inst(1, Vec3::new(0.0, 0.0, -d_px * 0.3), r), Vec3::ZERO, [0.0]), Verdict::Culled(CullReason::SubPixel));
}

/// Distance from the eye to the sphere surface (to 1 cm) at which `instance`'s pick first
/// reaches level 1 of `errors`, looking down −Z with the sub-pixel cull off.
fn lod1_switch(instance: StaticInstance, errors: [f32; 2]) -> f32 {
    let v = SceneView { size_cull_px: 0.0, ..view(Vec3::ZERO, Vec3::NEG_Z) };
    let r = instance.sphere.radius;
    (0..2_000_000)
        .map(|cm| cm as f32 / 100.0)
        .find(|&d| {
            let at = StaticInstance { sphere: Sphere { center: Vec3::new(0.0, 0.0, -(d + r)), radius: r }, ..instance };
            cull_instance(&v, &at, Vec3::ZERO, errors) == Verdict::Draw { level: 1 }
        })
        .unwrap()
}

#[test]
fn lod_error_follows_the_instance_scale() {
    // Level errors are model-space: a placement scaled 2× deviates 2× as many metres, so it must
    // be twice as far before the coarser level is acceptable (never nearer).
    let errors = [0.0, 0.05];
    let unit = inst(1, Vec3::ZERO, 0.5);
    let scaled = |s: Vec3| StaticInstance {
        transform: [[s.x, 0.0, 0.0, 0.0], [0.0, s.y, 0.0, 0.0], [0.0, 0.0, s.z, 0.0]],
        ..unit
    };
    let base = lod1_switch(unit, errors);
    let expected = 0.05 * pixels_per_radian(1440.0, FOV);
    assert!((base - expected).abs() < 0.02, "{base} vs {expected}");
    let double = lod1_switch(scaled(Vec3::splat(2.0)), errors);
    assert!((double / base - 2.0).abs() < 1e-3, "{double} vs {base}");
    let half = lod1_switch(scaled(Vec3::splat(0.5)), errors);
    assert!((half / base - 0.5).abs() < 1e-3, "{half} vs {base}");
    // Non-uniform scale: the largest axis decides (conservative), rotation does not matter.
    let stretched = lod1_switch(scaled(Vec3::new(1.0, 3.0, 1.0)), errors);
    assert!((stretched / base - 3.0).abs() < 1e-3);
    let rotated = glam::Affine3A::from_scale_rotation_translation(Vec3::splat(2.0), glam::Quat::from_rotation_y(1.1), Vec3::ZERO);
    let turned = lod1_switch(StaticInstance { transform: StaticInstance::rows(&rotated), ..unit }, errors);
    assert!((turned / double - 1.0).abs() < 1e-3);
    // At the unit switch distance the 2× instance still draws full detail.
    let v = SceneView { size_cull_px: 0.0, ..view(Vec3::ZERO, Vec3::NEG_Z) };
    let at = |i: StaticInstance| StaticInstance { sphere: Sphere { center: Vec3::new(0.0, 0.0, -(base * 1.01 + 0.5)), radius: 0.5 }, ..i };
    assert_eq!(cull_instance(&v, &at(unit), Vec3::ZERO, errors), Verdict::Draw { level: 1 });
    assert_eq!(cull_instance(&v, &at(scaled(Vec3::splat(2.0))), Vec3::ZERO, errors), Verdict::Draw { level: 0 });
}

#[test]
fn active_comes_first() {
    let v = view(Vec3::ZERO, Vec3::NEG_Z);
    let behind = inst(1, Vec3::new(0.0, 0.0, 50.0), 1.0);
    let inactive = StaticInstance { flags: InstanceFlags::INACTIVE, ..behind };
    assert_eq!(cull_instance(&v, &inactive, Vec3::ZERO, [0.0]), Verdict::Culled(CullReason::Inactive));
    assert_eq!(cull_instance(&v, &behind, Vec3::ZERO, [0.0]), Verdict::Culled(CullReason::Frustum));
    // The offset places the cell: the same instance moved in front is drawn.
    assert_eq!(cull_instance(&v, &behind, Vec3::new(0.0, 0.0, -100.0), [0.0]), Verdict::Draw { level: 0 });
}

/// 20 × 20 cells of 10 m, flat land at 0 with a ridge (floor 50 m) along grid column x = 10.
fn ridge() -> HorizonGrid {
    let mut g = HorizonGrid::new(Vec2::ZERO, 10.0, 20, 20);
    for z in 0..20 {
        for x in 0..20 {
            g.set(x, z, if x == 10 { 50.0 } else { 0.0 });
        }
    }
    g
}

fn cell_box(x: f32, top: f32) -> Aabb {
    Aabb { min: Vec3::new(x, 0.0, 0.0), max: Vec3::new(x + 10.0, top, 10.0) }
}

#[test]
fn horizon_culls_behind_the_ridge_and_keeps_above_it() {
    let g = ridge();
    let eye = Vec3::new(5.0, 2.0, 5.0);
    assert!(g.land_hides(eye, &cell_box(150.0, 20.0)), "low cell behind the ridge");
    assert!(!g.land_hides(eye, &cell_box(150.0, 200.0)), "tall cell above the ridge");
    assert!(!g.land_hides(eye, &cell_box(60.0, 20.0)), "in front of the ridge");
    // From high up the ridge hides nothing.
    assert!(!g.land_hides(Vec3::new(5.0, 300.0, 5.0), &cell_box(150.0, 20.0)));
    // Within min_cells (4 cells = 40 m), never culled, even behind a wall.
    let mut wall = g.clone();
    for z in 0..20 {
        wall.set(2, z, 1000.0);
    }
    assert!(!wall.land_hides(eye, &cell_box(35.0, 1.0)));
    assert!(wall.land_hides(eye, &cell_box(45.0, 1.0)));
    // An eye under its own cell's floor culls nothing (a cave below the heightfield).
    assert!(!g.land_hides(Vec3::new(5.0, -1.0, 5.0), &cell_box(150.0, 20.0)));
    // Outside the grid there is no land: nothing blocks.
    assert!(!g.land_hides(Vec3::new(-100.0, 2.0, -100.0), &cell_box(-200.0, 1.0)));
}

#[test]
fn horizon_is_conservative_at_the_margin() {
    // Nearest top sample x = 150 crosses the ridge's near face (x = 100) at t = 95 / 145. A box
    // top of 74 m clears the 50 m floor there only once raised by the 3 m margin.
    let mut g = ridge();
    let eye = Vec3::new(5.0, 2.0, 5.0);
    let at_ridge = |top: f32| 2.0 + (top - 2.0) * 95.0 / 145.0;
    assert!(at_ridge(74.0) < 50.0 && at_ridge(77.0) > 50.0);
    assert!(!g.land_hides(eye, &cell_box(150.0, 74.0)), "kept thanks to the margin");
    g.params.margin = 0.0;
    assert!(g.land_hides(eye, &cell_box(150.0, 74.0)), "culled without it");
    // One of the nine samples seeing past is enough: open the ridge's first row (z ∈ [0, 10]).
    g.set(10, 0, 0.0);
    let wide = Aabb { min: Vec3::new(150.0, 0.0, 0.0), max: Vec3::new(160.0, 20.0, 30.0) };
    assert!(!g.land_hides(eye, &wide), "the z = 0 samples see through the gap");
    let shifted = Aabb { min: Vec3::new(150.0, 0.0, 15.0), max: Vec3::new(160.0, 20.0, 30.0) };
    assert!(g.land_hides(eye, &shifted), "every sample crosses the ridge past the gap");
}

#[test]
fn horizon_walk_follows_diagonals() {
    // A single blocking cell on the diagonal hides a target straight behind it only.
    let mut g = HorizonGrid::new(Vec2::ZERO, 10.0, 20, 20);
    for z in 0..20 {
        for x in 0..20 {
            g.set(x, z, 0.0);
        }
    }
    for k in 0..=1 {
        g.set(8 + k, 8, 100.0);
        g.set(8, 8 + k, 100.0);
        g.set(9, 9, 100.0);
    }
    let eye = Vec3::new(5.0, 1.0, 5.0);
    let behind = Aabb { min: Vec3::new(150.0, 0.0, 150.0), max: Vec3::new(152.0, 2.0, 152.0) };
    let aside = Aabb { min: Vec3::new(150.0, 0.0, 10.0), max: Vec3::new(152.0, 2.0, 12.0) };
    assert!(g.land_hides(eye, &behind));
    assert!(!g.land_hides(eye, &aside));
}

fn cell_table() -> StaticCellTable {
    // Origin cell (0, 0, −1): 256 m along −Z. Instances relative to its centre.
    let local = |z: f32| Vec3::new(0.0, 1.0, z + 256.0);
    StaticCellTable {
        origin_cell: IVec3::new(0, 0, -1),
        aabb: Aabb { min: Vec3::new(-128.0, -10.0, -128.0), max: Vec3::new(128.0, 30.0, 128.0) },
        land_min: 0.0,
        layers: 1,
        instances: vec![
            inst(1, local(-20.0), 1.0),                                   // near: level 0
            inst(2, local(-120.0), 1.0),                                  // far: coarser
            inst(4, local(-20.0), 1.0),                                   // promoted
            inst(5, local(-200.0), 0.001),                                // sub-pixel
            inst(6, Vec3::new(-127.0, 1.0, 236.0), 1.0),                  // left of the frustum
        ],
        chains: vec![LodChain {
            mesh: 9,
            material: 0x77,
            material_class: MaterialClass::Masked,
            levels: vec![
                LodLevel { first_index: 0, index_count: 600, error: 0.0 },
                LodLevel { first_index: 600, index_count: 300, error: 0.01 },
                LodLevel { first_index: 900, index_count: 90, error: 0.1 },
            ],
        }],
    }
}

#[test]
fn cull_cell_emits_the_expected_work_items() {
    let bytes = cell_table().write().unwrap();
    let table = StaticCellTableView::parse(&bytes).unwrap();
    let offset = crate::cell_offset(table.origin_cell(), sf_coords::WorldPos::ORIGIN);
    assert_eq!(offset, Vec3::new(0.0, 0.0, -256.0));
    let v = view(Vec3::new(0.0, 1.0, 0.0), Vec3::NEG_Z);
    let mut promotions = Promotions::new();
    promotions.promote(4);
    let mut out = Vec::new();
    let stats = cull_cell(&v, 7, &table, offset, None, &promotions, &mut out);
    let items: Vec<_> = out.iter().map(|w| (w.cell, w.instance, w.placement, w.material_class, w.chain, w.level)).collect();
    // ppr ≈ 1247: level 1 (1 cm) from ~12.5 m, level 2 (10 cm) from ~125 m (surface distance).
    assert_eq!(items, [(7, 0, 1, 2, 0, 1), (7, 1, 2, 2, 0, 1)]);
    assert_eq!(stats, CullStats { drawn: 2, layer: 0, inactive: 1, frustum: 1, horizon: 0, sub_pixel: 1 });
    // From farther back the far one drops to the coarsest level.
    out.clear();
    let back = SceneView { eye: Vec3::new(0.0, 1.0, 30.0), frustum: frustum(Vec3::new(0.0, 1.0, 30.0), Vec3::NEG_Z), ..v };
    cull_cell(&back, 7, &table, offset, None, &promotions, &mut out);
    assert_eq!(out.iter().find(|w| w.placement == 2).map(|w| w.level), Some(2));
    // Looking away culls the whole cell on its box.
    out.clear();
    let away = SceneView { frustum: frustum(v.eye, Vec3::Z), ..v };
    let stats = cull_cell(&away, 7, &table, offset + Vec3::new(0.0, 0.0, -200.0), None, &promotions, &mut out);
    assert!(out.is_empty());
    assert_eq!(stats.frustum, 5);
    // Layers are per table: a view without the table's layer culls the whole cell first.
    let other_layer = SceneView { layers: 2, ..away };
    let stats = cull_cell(&other_layer, 7, &table, offset, None, &promotions, &mut out);
    assert!(out.is_empty());
    assert_eq!(stats, CullStats { layer: 5, ..CullStats::default() });
}

#[test]
fn cull_cell_applies_the_horizon_only_when_asked() {
    let bytes = cell_table().write().unwrap();
    let table = StaticCellTableView::parse(&bytes).unwrap();
    // Cell centred at (0, 0, −256), box top 30 m; a 500 m wall at z ∈ [−80, −60] across the grid.
    let mut g = HorizonGrid::new(Vec2::new(-400.0, -600.0), 20.0, 40, 40);
    for z in 0..40 {
        for x in 0..40 {
            g.set(x, z, if z == 26 { 500.0 } else { 0.0 });
        }
    }
    let offset = Vec3::new(0.0, 0.0, -256.0);
    let mut v = view(Vec3::new(0.0, 1.0, 0.0), Vec3::NEG_Z);
    let mut out = Vec::new();
    let promotions = Promotions::new();
    assert_eq!(cull_cell(&v, 0, &table, offset, Some(&g), &promotions, &mut out).horizon, 0, "off by default");
    v.horizon = true;
    out.clear();
    let stats = cull_cell(&v, 0, &table, offset, Some(&g), &promotions, &mut out);
    assert_eq!((stats.horizon, out.len()), (5, 0));
}

#[test]
fn texels_per_metre_is_the_maps_surface_density() {
    // A 2048-texel cascade over 128 m: 16 texels per metre, wherever in the cascade.
    assert!((texels_per_metre(2048.0, 128.0) - 16.0).abs() < 1e-6);
    assert!((texels_per_metre(1024.0, 512.0) - 2.0).abs() < 1e-6);
}

#[test]
fn ortho_pick_has_no_distance_term() {
    let errors = [0.0, 0.05, 0.2];
    let tpm = texels_per_metre(2048.0, 128.0); // 16 texels/m: a level passes while e ≤ 1/16 m.
    assert_eq!(pick_level_ortho(errors, tpm, 1.0), Some(1));
    // Coarser allowance, coarser level; a tighter one falls back to level 0.
    assert_eq!(pick_level_ortho(errors, tpm, 4.0), Some(2));
    assert_eq!(pick_level_ortho(errors, tpm, 0.1), Some(0));
    // A denser map picks finer, a sparser one coarser, at the same lod_px.
    assert_eq!(pick_level_ortho(errors, texels_per_metre(8192.0, 128.0), 1.0), Some(0));
    assert_eq!(pick_level_ortho(errors, texels_per_metre(256.0, 128.0), 1.0), Some(2));
    // Exactly at the threshold the level is taken (`≤`), just past it is not.
    assert_eq!(pick_level_ortho([0.0, 1.0 / 16.0], tpm, 1.0), Some(1));
    assert_eq!(pick_level_ortho([0.0, 1.0 / 16.0 + 1e-4], tpm, 1.0), Some(0));
    // As `pick_level`: no levels, no pick; one level is always level 0.
    assert_eq!(pick_level_ortho([0.0f32; 0], tpm, 1.0), None);
    assert_eq!(pick_level_ortho([5.0], tpm, 1.0), Some(0));
    // `SceneProjection` dispatches to the two forms.
    let p = SceneProjection::Perspective { pixels_per_radian: pixels_per_radian(1440.0, FOV) };
    let o = SceneProjection::Orthographic { texels_per_metre: tpm };
    assert_eq!(p.pick_level(errors, 20.0, 1.0), pick_level(errors, 20.0, pixels_per_radian(1440.0, FOV), 1.0));
    assert_eq!(o.pick_level(errors, 20.0, 1.0), Some(1));
    assert_eq!(o.pick_level(errors, 1e6, 1.0), Some(1), "distance must not matter");
}

#[test]
fn orthographic_frustum_is_the_cascade_box() {
    // Looking down −Z from the origin: the cube [−64, 64]³ (near −64, far 64 along forward).
    let f = Frustum::orthographic(Vec3::ZERO, Vec3::NEG_Z, Vec3::Y, 64.0, 64.0, -64.0, 64.0);
    assert!(!f.sphere_outside(Vec3::new(0.0, 0.0, -10.0), 1.0));
    assert!(!f.sphere_outside(Vec3::new(0.0, 0.0, 60.0), 1.0), "behind the centre, still in the box");
    assert!(!f.sphere_outside(Vec3::new(0.0, 0.0, -64.5), 1.0), "straddles the far face");
    assert!(f.sphere_outside(Vec3::new(0.0, 0.0, -70.0), 1.0), "past the far face");
    assert!(f.sphere_outside(Vec3::new(0.0, 0.0, 70.0), 1.0), "past the near face");
    // Unlike a perspective frustum the sides do not widen with distance.
    for z in [-10.0, -60.0] {
        assert!(!f.sphere_outside(Vec3::new(63.0, 0.0, z), 1.0), "inside the right face at {z}");
        assert!(f.sphere_outside(Vec3::new(70.0, 0.0, z), 1.0), "past the right face at {z}");
        assert!(f.sphere_outside(Vec3::new(0.0, -70.0, z), 1.0), "under the bottom face at {z}");
        assert!(!f.sphere_outside(Vec3::new(0.0, 64.5, z), 1.0), "straddles the top face at {z}");
    }
    // A box test agrees, and an infinite far face never culls.
    assert!(!f.aabb_outside(&Aabb { min: Vec3::splat(-1.0), max: Vec3::splat(1.0) }));
    assert!(f.aabb_outside(&Aabb { min: Vec3::new(65.0, 0.0, 0.0), max: Vec3::new(80.0, 1.0, 1.0) }));
    let inf = Frustum::orthographic(Vec3::ZERO, Vec3::NEG_Z, Vec3::Y, 64.0, 64.0, -64.0, f32::INFINITY);
    assert!(!inf.sphere_outside(Vec3::new(0.0, 0.0, -1e7), 1.0));
}

#[test]
fn ortho_cull_picks_one_level_per_chain_whatever_the_distance() {
    // A sun cascade: §4 step 5's orthographic form has no distance term, so the same chain at
    // the near and far end of the cascade takes the same level. With the perspective form
    // (what a cascade used before `SceneProjection`) it would pick main-view levels.
    let v = ortho_view(Vec3::ZERO, 2048.0, 64.0); // 16 texels/m, lod_px 1 → a level needs e ≤ 1/16.
    let errors = [0.0, 0.05, 0.2];
    let at = |z: f32| cull_instance(&v, &inst(1, Vec3::new(0.0, 0.0, z), 1.0), Vec3::ZERO, errors);
    assert_eq!(at(-2.0), Verdict::Draw { level: 1 });
    assert_eq!(at(-60.0), Verdict::Draw { level: 1 });
    assert_eq!(at(60.0), Verdict::Draw { level: 1 }, "behind the cascade centre");
    // A sparser cascade takes a coarser level, again everywhere in it.
    let far = ortho_view(Vec3::ZERO, 256.0, 64.0);
    let at_far = |z: f32| cull_instance(&far, &inst(1, Vec3::new(0.0, 0.0, z), 1.0), Vec3::ZERO, errors);
    assert_eq!(at_far(-2.0), Verdict::Draw { level: 2 });
    assert_eq!(at_far(-60.0), Verdict::Draw { level: 2 });
    // Model-space errors still scale with the instance, in this form too.
    let scaled = StaticInstance {
        transform: [[2.0, 0.0, 0.0, 0.0], [0.0, 2.0, 0.0, 0.0], [0.0, 0.0, 2.0, -20.0]],
        sphere: Sphere { center: Vec3::new(0.0, 0.0, -20.0), radius: 2.0 },
        ..inst(1, Vec3::ZERO, 1.0)
    };
    assert_eq!(cull_instance(&v, &scaled, Vec3::ZERO, errors), Verdict::Draw { level: 0 }, "0.05 × 2 > 1/16 m");
    // Outside the box it is a frustum cull, not a LOD pick.
    assert_eq!(at(-70.0), Verdict::Culled(CullReason::Frustum));
    assert_eq!(cull_instance(&v, &inst(1, Vec3::new(70.0, 0.0, -10.0), 1.0), Vec3::ZERO, errors), Verdict::Culled(CullReason::Frustum));
}

#[test]
fn ortho_sub_pixel_cull_is_in_map_texels() {
    // 16 texels/m, size_cull_px 1: a caster under 1/16 m radius covers under a texel and goes.
    let v = ortho_view(Vec3::ZERO, 2048.0, 64.0);
    let at = |r: f32, z: f32| cull_instance(&v, &inst(1, Vec3::new(0.0, 0.0, z), r), Vec3::ZERO, [0.0]);
    assert_eq!(at(0.1, -10.0), Verdict::Draw { level: 0 });
    assert_eq!(at(0.05, -10.0), Verdict::Culled(CullReason::SubPixel));
    // No distance term here either: the same radius decides at both ends of the cascade.
    assert_eq!(at(0.05, -60.0), Verdict::Culled(CullReason::SubPixel));
    assert_eq!(at(0.1, -60.0), Verdict::Draw { level: 0 });
    // A sparser cascade drops more: the same caster is sub-texel there.
    let far = ortho_view(Vec3::ZERO, 256.0, 64.0);
    assert_eq!(cull_instance(&far, &inst(1, Vec3::new(0.0, 0.0, -10.0), 0.1), Vec3::ZERO, [0.0]), Verdict::Culled(CullReason::SubPixel));
    // 0 turns it off for a view that wants every caster.
    let every = SceneView { size_cull_px: 0.0, ..v };
    assert_eq!(cull_instance(&every, &inst(1, Vec3::new(0.0, 0.0, -10.0), 1e-4), Vec3::ZERO, [0.0]), Verdict::Draw { level: 0 });
}

/// An orthographic view has no eye point, so `horizon` on one cannot mean anything: the test
/// marches from `SceneView::eye`. A debug build says so instead of culling cells off a
/// meaningless value (release builds carry on: this must never take the game down).
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "orthographic view")]
fn the_horizon_on_an_orthographic_view_is_a_caller_bug() {
    let bytes = cell_table().write().unwrap();
    let table = StaticCellTableView::parse(&bytes).unwrap();
    let v = SceneView { horizon: true, ..ortho_view(Vec3::ZERO, 2048.0, 64.0) };
    cull_cell(&v, 0, &table, Vec3::ZERO, None, &Promotions::new(), &mut Vec::new());
}

/// The same view without the horizon is fine, and so is a perspective view with it: the assert
/// is about the pair, not about either field.
#[test]
fn an_orthographic_view_culls_without_the_horizon() {
    let bytes = cell_table().write().unwrap();
    let table = StaticCellTableView::parse(&bytes).unwrap();
    let ortho = ortho_view(Vec3::new(0.0, 0.0, -256.0), 2048.0, 256.0);
    assert!(!ortho.horizon);
    let mut out = Vec::new();
    let stats = cull_cell(&ortho, 0, &table, Vec3::new(0.0, 0.0, -256.0), None, &Promotions::new(), &mut out);
    assert_eq!(stats.drawn + stats.frustum + stats.sub_pixel + stats.inactive, 5);
}
