//! `body.cook` fits and the `items.check` settle test on fixtures.
#![allow(clippy::many_single_char_names)]

use std::f32::consts::TAU;
use std::sync::Arc;

use glam::Vec3;

use crate::cook::{BodyShape, BodySource, FitSettings, PartShape, fit_authored, fit_render};

/// A closed box mesh centered at `c`.
fn box_mesh(c: Vec3, h: Vec3) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let p: Vec<Vec3> = (0..8)
        .map(|i| c + h * Vec3::new(if i & 1 == 0 { -1.0 } else { 1.0 }, if i & 2 == 0 { -1.0 } else { 1.0 }, if i & 4 == 0 { -1.0 } else { 1.0 }))
        .collect();
    let t = vec![
        [0, 2, 1], [1, 2, 3], [4, 5, 6], [5, 7, 6], [0, 1, 4], [1, 5, 4],
        [2, 6, 3], [3, 6, 7], [0, 4, 2], [2, 4, 6], [1, 3, 5], [3, 7, 5],
    ];
    (p, t)
}

/// A closed prism over the polygon `poly` (XZ), from Y 0 to `h`.
fn prism(poly: &[(f32, f32)], h: f32) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let n = u32::try_from(poly.len()).unwrap();
    let mut p: Vec<Vec3> = poly.iter().map(|&(x, z)| Vec3::new(x, h, z)).collect();
    p.extend(poly.iter().map(|&(x, z)| Vec3::new(x, 0.0, z)));
    let mut t = Vec::new();
    for i in 1..n - 1 {
        t.push([0, i, i + 1]);
        t.push([n, n + i + 1, n + i]);
    }
    for i in 0..n {
        let j = (i + 1) % n;
        t.push([j, i, n + i]);
        t.push([j, n + i, n + j]);
    }
    (p, t)
}

fn disc(r: f32, segments: u32) -> Vec<(f32, f32)> {
    #[allow(clippy::cast_precision_loss)]
    (0..segments).map(|i| (i as f32 / segments as f32 * TAU).sin_cos()).map(|(s, c)| (r * c, r * s)).collect()
}

#[test]
fn boxes_fit_boxes_with_exact_mass_properties() {
    let (p, t) = box_mesh(Vec3::new(0.0, 0.1, 0.0), Vec3::new(0.2, 0.1, 0.05));
    let body = fit_render(&p, &t, &FitSettings::default()).unwrap();
    assert_eq!(body.source, BodySource::Primitive);
    assert!(matches!(body.parts[0].shape, PartShape::Box(_)), "{:?}", body.parts);
    assert!((body.volume - 0.4 * 0.2 * 0.1).abs() < 1e-5, "{}", body.volume);
    assert!(Vec3::from_array(body.center_of_mass).distance(Vec3::new(0.0, 0.1, 0.0)) < 1e-4);
    // Unit inertia of a box: (b² + c²) / 3 per half extents, in some order.
    let mut got = body.unit_inertia;
    got.sort_by(f32::total_cmp);
    let mut want = [(0.01f32 + 0.0025) / 3.0, (0.04 + 0.0025) / 3.0, (0.04 + 0.01) / 3.0];
    want.sort_by(f32::total_cmp);
    for (g, w) in got.iter().zip(want) {
        assert!((g - w).abs() < 1e-5, "{got:?} vs {want:?}");
    }
    // Codec round trip.
    assert_eq!(BodyShape::from_bytes(&body.to_bytes()).unwrap(), body);
    assert!(BodyShape::from_bytes(b"SFBDxxxx").is_err());
}

#[test]
fn round_models_fit_cylinders_or_capsules() {
    let (p, t) = prism(&disc(0.04, 24), 0.3);
    let body = fit_render(&p, &t, &FitSettings::default()).unwrap();
    assert_eq!(body.source, BodySource::Primitive);
    assert!(matches!(body.parts[0].shape, PartShape::Cylinder { .. } | PartShape::Capsule { .. }), "{:?}", body.parts[0]);
    // A bottle lying on its side stays a round shape.
    let lying: Vec<Vec3> = p.iter().map(|v| Vec3::new(v.y, v.x, v.z)).collect();
    let body = fit_render(&lying, &t, &FitSettings::default()).unwrap();
    assert!(matches!(body.parts[0].shape, PartShape::Cylinder { .. } | PartShape::Capsule { .. }));
}

#[test]
fn flat_models_get_the_minimum_thickness() {
    let p = vec![Vec3::ZERO, Vec3::new(0.2, 0.0, 0.0), Vec3::new(0.2, 0.0, 0.3), Vec3::new(0.0, 0.0, 0.3)];
    let body = fit_render(&p, &[[0, 1, 2], [0, 2, 3]], &FitSettings::default()).unwrap();
    assert_eq!(body.source, BodySource::Thin);
    let extent = Vec3::from_array(body.aabb[1]) - Vec3::from_array(body.aabb[0]);
    assert!((extent.min_element() - 0.01).abs() < 1e-4, "{extent}");
}

#[test]
fn open_models_get_one_capped_hull() {
    // A cone without its base: open, and no primitive within 15%.
    let ring = disc(0.1, 48);
    let mut p: Vec<Vec3> = ring.iter().map(|&(x, z)| Vec3::new(x, 0.0, z)).collect();
    p.push(Vec3::new(0.0, 0.3, 0.0));
    let t: Vec<[u32; 3]> = (0..48).map(|i| [48, i, (i + 1) % 48]).collect();
    let body = fit_render(&p, &t, &FitSettings::default()).unwrap();
    assert_eq!(body.source, BodySource::Hull);
    let verts = body.parts[0].hull_vertices();
    assert!((8..=32).contains(&verts), "{verts}");
}

#[test]
fn concave_closed_models_decompose() {
    // A thin L: the hull is far bigger than the solid.
    let l = [(0.0, 0.0), (1.0, 0.0), (1.0, 0.1), (0.1, 0.1), (0.1, 1.0), (0.0, 1.0)];
    let (p, t) = prism(&l, 0.1);
    let body = fit_render(&p, &t, &FitSettings::default()).unwrap();
    assert_eq!(body.source, BodySource::Decomposition);
    assert!((2..=8).contains(&body.parts.len()), "{}", body.parts.len());
    assert!(body.parts.iter().all(|p| p.hull_vertices() <= 32));
    // The pieces cover roughly the L's volume (0.019 m³), far below its hull's.
    assert!(body.volume > 0.012 && body.volume < 0.04, "{}", body.volume);
}

#[test]
fn authored_parts_hull_each() {
    let (a, _) = box_mesh(Vec3::ZERO, Vec3::splat(0.1));
    let (b, _) = box_mesh(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(0.1));
    let body = fit_authored(&[a, b, Vec::new()], &FitSettings::default()).unwrap();
    assert_eq!(body.source, BodySource::Authored);
    assert_eq!(body.parts.len(), 2);
    assert!((body.volume - 2.0 * 0.008).abs() < 1e-4);
    assert!(fit_authored(&[], &FitSettings::default()).is_err());
}

#[cfg(feature = "bake")]
mod settle {
    use sf_coords::{WorldPos, WorldTransform};

    use super::*;
    use crate::steps::{CheckSettings, Hold, SettleItem, settle};
    use crate::{BodyDesc, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

    fn cube(h: f32) -> Arc<BodyShape> {
        let (p, t) = box_mesh(Vec3::ZERO, Vec3::splat(h));
        Arc::new(fit_render(&p, &t, &FitSettings::default()).unwrap())
    }

    #[test]
    fn unsupported_items_hold_and_resting_items_dont() {
        let at = |x: f32, y: f32| WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(x, y, 0.0)), ..WorldTransform::IDENTITY };
        let mut world = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
        // A table: top at Y 1, from X -1 to 1.
        world.insert(&BodyDesc::fixed(at(0.0, 0.95), ShapeRef::Box(Vec3::new(1.0, 0.05, 1.0)), Layer::STATIC));
        let c = cube(0.05);
        let item = |id: u128, x: f32, y: f32| SettleItem { id, pose: at(x, y), shape: Some(c.clone()), always_hold: false };
        let items = vec![
            item(0, 0.0, 1.05),  // on the table
            item(1, 0.3, 1.45),  // 40 cm above it
            item(2, 3.0, 1.05),  // beside it, over nothing
            item(3, -0.3, 1.03), // 2 cm into it
            item(4, 0.0, 1.15),  // stacked on item 0
            SettleItem { always_hold: true, ..item(5, 0.6, 1.05) },
            SettleItem { shape: None, ..item(6, 0.8, 1.05) },
        ];
        let checks = settle(&mut world, &items, &CheckSettings::default());
        let holds: Vec<Hold> = checks.iter().map(|c| c.hold).collect();
        assert_eq!(
            holds,
            [Hold::No, Hold::Unsupported, Hold::Unsupported, Hold::No, Hold::No, Hold::Rules, Hold::NoShape],
            "{checks:#?}"
        );
        assert!(checks[0].drop < 0.005 && checks[0].overlap() < 0.005, "{:?}", checks[0]);
        assert!(checks[1].drop > 0.3 && checks[2].drop > 0.5);
        assert!(checks[3].rise > 0.01 && checks[3].overlap() < 0.05, "{:?}", checks[3]);
    }
}

/// Bodies inserted awake fall at once, primitives and `body.cook` shapes alike.
#[test]
fn awake_bodies_fall() {
    use sf_coords::{WorldPos, WorldTransform};

    use crate::{BodyDesc, BodyKind, Layer, PhysicsParams, PhysicsWorld, ShapeRef};
    let (p, t) = box_mesh(Vec3::ZERO, Vec3::splat(0.05));
    let body = Arc::new(fit_render(&p, &t, &FitSettings::default()).unwrap());
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let at = |x: f32| WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(x, 5.0, 0.0)), ..WorldTransform::IDENTITY };
    let a = w.insert(&BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(0.0), ShapeRef::Box(Vec3::splat(0.05)), Layer::PROP));
    let b = w.insert(&BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(1.0), ShapeRef::Body(body), Layer::PROP));
    w.refresh_queries();
    for _ in 0..30 {
        w.step();
    }
    for id in [a, b] {
        let y = w.pose(id).unwrap().pos.local.y;
        assert!(y < 4.0, "{id:?} at {y}: {:?}", w.velocity(id));
    }
}
