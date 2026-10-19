//! Headless glue tests: grid precision, origin phases, and a full stream
//! in and out through the real stages (cooked models from a bake store).

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::mesh::{Mesh3d, MeshPlugin};
use bevy::prelude::*;
use big_space::plugin::BigSpaceMinimalPlugins;
use big_space::prelude::FloatingOrigin;
use glam::DVec3;
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{CookSettings, Material as ModelMaterial, MeshData, Model, Node, cook};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_vfs::VfsPath;
use sf_world::{
    IndexParams, Level, MemoryIndex, ModelInfo, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef,
    WorldspaceId,
};

use super::*;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

#[test]
fn entities_a_millimeter_apart_a_thousand_km_out_stay_a_millimeter_apart() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, BigSpaceMinimalPlugins));
    let root = app.world_mut().spawn((big_space::prelude::BigSpace::default(), world_grid(), GlobalTransform::default())).id();
    let far = DVec3::new(1.0e6, 3.0, -1.0e6);
    let spawn = |app: &mut App, p: DVec3, extra: bool| {
        let (cell, transform) = to_grid(WorldPos::from_f64(p));
        let mut e = app.world_mut().spawn((cell, transform, ChildOf(root)));
        if extra {
            e.insert(FloatingOrigin);
        }
        e.id()
    };
    spawn(&mut app, far + DVec3::new(10.0, 0.0, 0.0), true);
    let a = spawn(&mut app, far, false);
    let b = spawn(&mut app, far + DVec3::new(0.001, 0.0, 0.0), false);
    app.update();
    let ga = app.world().get::<GlobalTransform>(a).unwrap().translation();
    let gb = app.world().get::<GlobalTransform>(b).unwrap().translation();
    assert!(((gb - ga).x - 0.001).abs() < 2e-6, "{ga} {gb}");
    assert!(ga.length() < 300.0, "origin-relative, not absolute: {ga}");
}

#[test]
fn grid_conversions_round_trip() {
    for p in [DVec3::ZERO, DVec3::new(127.9, -128.0, 5.0e5), DVec3::new(-3.0e7, 12.5, 7.0e6)] {
        let pos = WorldPos::from_f64(p);
        let (cell, transform) = to_grid(pos);
        assert_eq!(from_grid(&cell, &transform), pos);
    }
    // big_space lets moving entities overshoot their cell before recentering.
    let (cell, mut transform) = to_grid(WorldPos::from_f64(DVec3::new(100.0, 0.0, 0.0)));
    transform.translation.x += 50.0;
    assert_eq!(from_grid(&cell, &transform), WorldPos::from_f64(DVec3::new(150.0, 0.0, 0.0)));
}

#[test]
fn origin_phase_is_continuous_across_origin_moves() {
    let world = WorldPos::from_f64(DVec3::new(123_456.75, 3.5, -987_654.25));
    let mut seen = Vec::new();
    for origin_cell in [IVec3::new(482, 0, -3858), IVec3::new(483, 0, -3858), IVec3::new(480, 0, -3861)] {
        let origin = WorldOrigin::new(Some(ws()), origin_cell);
        let rel = coords::origin_relative(origin_cell, world);
        for period in PHASE_PERIODS {
            seen.push((period, origin.wrapped(rel, period)));
        }
    }
    for period in PHASE_PERIODS {
        let values: Vec<Vec3> = seen.iter().filter(|(p, _)| *p == period).map(|(_, v)| *v).collect();
        #[allow(clippy::cast_precision_loss)]
        let expected = world.to_f64().rem_euclid(DVec3::splat(period as f64)).as_vec3();
        for v in values {
            assert!((v - expected).abs().max_element() < 1e-3, "{v} vs {expected} (period {period})");
        }
    }
    // Exact integers: no drift at the far end of the i32 grid.
    assert_eq!(WorldOrigin::phase(IVec3::new(i32::MAX, i32::MIN, -1), 1024), Vec3::new(768.0, 0.0, 768.0));
}

// -- end to end ------------------------------------------------------------------

/// A flat 8 × 8 m grid, fine enough for a LOD chain.
fn plate() -> Model {
    let n = 24u32;
    let mut mesh = MeshData { normals: Some(Vec::new()), ..MeshData::default() };
    for z in 0..=n {
        for x in 0..=n {
            #[allow(clippy::cast_precision_loss)]
            let (fx, fz) = (x as f32 / n as f32, z as f32 / n as f32);
            mesh.positions.push([fx * 8.0 - 4.0, 0.3 * (fx * 9.0).sin() * (fz * 7.0).cos(), fz * 8.0 - 4.0]);
            mesh.normals.as_mut().unwrap().push([0.0, 1.0, 0.0]);
        }
    }
    let row = n + 1;
    for z in 0..n {
        for x in 0..n {
            let i = z * row + x;
            mesh.indices.extend([i, i + row, i + 1, i + 1, i + row, i + row + 1]);
        }
    }
    let mut node = Node::new("plate");
    node.mesh = Some(0);
    Model { nodes: vec![node], meshes: vec![mesh], materials: vec![ModelMaterial::default()] }
}

fn placements() -> Vec<Placement> {
    let mut out = Vec::new();
    for i in 0..40 {
        for j in 0..40 {
            let key = format!("{i}/{j}");
            let model = if (i + j) % 7 == 0 { "missing.mdl" } else { "plate.mdl" };
            out.push(Placement {
                id: PlacementId::from_key("test", key.as_bytes()),
                base: RecordRef::new("thing", key.as_str()),
                model: Some(VfsPath::new(model).unwrap()),
                transform: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(
                    f64::from(i) * 40.0 - 800.0,
                    0.0,
                    f64::from(j) * 40.0 - 800.0,
                ))),
                class: if (i * j) % 11 == 3 { PlacementClass::Marker } else { PlacementClass::Static },
                flags: PlacementFlags::empty(),
            });
        }
    }
    out
}

fn streaming_app(dir: &std::path::Path) -> App {
    let store = Store::open(dir).unwrap();
    let cooked = cook(&plate(), &CookSettings::default()).unwrap().to_bytes();
    let mut manifest = Manifest::default();
    manifest.products.insert(ProductKey::new("mesh.cook", "plate.mdl"), store.put(&cooked, true).unwrap());

    let models = |p: &VfsPath| {
        (p.as_str() != "nothing").then_some(ModelInfo { min: Vec3::new(-4.0, -0.3, -4.0), max: Vec3::new(4.0, 0.3, 4.0), bytes: 50_000, lod_distance: 0.0 })
    };
    let (pages, _) = sf_world::build_pages(&IndexParams::default(), placements(), &models);
    let directory = sf_world::build_directory(ws(), &IndexParams::default(), &pages);
    let mut index = MemoryIndex::new();
    index.insert(&sf_world::WorldIndex { directory, pages, stats: sf_world::IndexStats::default() });

    let mut app = App::new();
    app.register_asset_source(
        BAKED_SOURCE,
        baked_source(store, BakedManifest::new(manifest), BTreeMap::from([("mesh.cook".to_owned(), "sfmesh".to_owned())])),
    );
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .add_plugins(WorldStreamingPlugin::default())
        .insert_resource(WorldIndexSource(Arc::new(index)))
        .insert_resource(ActiveWorldspace(Some(ws())));
    app.finish();
    app.cleanup();
    app
}

/// Runs frames until streaming is idle (async IO and asset loads need real time).
fn settle(app: &mut App) -> usize {
    for frame in 0..5000 {
        app.update();
        if frame > 3 && app.world().resource::<Streaming>().is_idle() {
            return frame;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("streaming never went idle");
}

fn count<F: bevy::ecs::query::QueryFilter>(app: &mut App) -> usize {
    app.world_mut().query_filtered::<(), F>().iter(app.world()).count()
}

#[test]
fn streams_in_around_an_anchor_and_out_when_it_leaves() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = streaming_app(dir.path());
    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &ws()).root;
    let (cell, transform) = to_grid(WorldPos::from_f64(DVec3::new(0.0, 2.0, 0.0)));
    let camera = app
        .world_mut()
        .spawn((cell, transform, StreamingAnchor::visual(), FloatingOrigin, ChildOf(root)))
        .id();
    app.world_mut().spawn((to_grid(WorldPos::ORIGIN).0, Transform::IDENTITY, StreamingAnchor::gameplay(), ChildOf(root)));
    app.world_mut().resource_mut::<Streaming>().begin_burst();
    settle(&mut app);

    let streaming = app.world().resource::<Streaming>();
    let stats = streaming.stats();
    let resident: Vec<(IVec3, Level)> = streaming.scheduler().resident(&ws()).collect();
    // 1.6 km square of content, R2 at 3 km: every cell is resident; R1
    // (768 m) covers all but the corners.
    assert_eq!(resident.len(), streaming.scheduler().directory(&ws()).unwrap().cells.len());
    assert!(resident.iter().filter(|(_, l)| *l >= Level::Near).count() > resident.len() * 3 / 4);
    assert!(resident.iter().any(|(_, l)| *l == Level::Active));
    assert_eq!(stats.resident_cells, resident.len());
    assert!(stats.uncooked > 0, "missing.mdl falls back (no uncooked hook: nothing spawned)");
    assert_eq!(count::<With<StreamCell>>(&mut app), resident.len());
    let parts = count::<With<Mesh3d>>(&mut app);
    assert!(parts > 1000, "{parts} LOD parts");
    // Every part has an Aabb (no CPU mesh needed) and a visibility range.
    assert_eq!(count::<(With<Mesh3d>, With<bevy::camera::primitives::Aabb>, With<bevy::camera::visibility::VisibilityRange>)>(&mut app), parts);
    // All groups were revealed.
    let hidden = app
        .world_mut()
        .query::<(&Visibility, &ChildOf)>()
        .iter(app.world())
        .filter(|(v, _)| **v == Visibility::Hidden)
        .count();
    assert_eq!(hidden, 0);

    // Fly 50 km away: everything unloads, models are released.
    app.world_mut()
        .entity_mut(camera)
        .insert(to_grid(WorldPos::from_f64(DVec3::new(50_000.0, 2.0, 0.0))));
    let gameplay: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, (With<StreamingAnchor>, Without<FloatingOrigin>)>()
        .iter(app.world())
        .collect();
    for e in gameplay {
        app.world_mut().entity_mut(e).despawn();
    }
    settle(&mut app);
    assert_eq!(count::<With<StreamCell>>(&mut app), 0);
    assert_eq!(count::<With<Mesh3d>>(&mut app), 0);
    let stats = app.world().resource::<Streaming>().stats();
    assert_eq!(stats.models_loaded, 0);
    assert_eq!(stats.resident_cells, 0);
}

/// Meshes spawned later under a `SubtreeRange` root (a runtime-imported
/// model's scene) get the placement's range, pushed out by their offset.
#[test]
fn subtree_ranges_reach_late_meshes() {
    use bevy::camera::visibility::VisibilityRange;
    use bevy::ecs::system::RunSystemOnce;

    let mut world = World::new();
    let root = world
        .spawn((Transform::from_scale(Vec3::splat(2.0)), SubtreeRange { end: 100.0, band: 10.0 }))
        .id();
    let node = world.spawn((Transform::from_xyz(3.0, 0.0, 0.0), ChildOf(root))).id();
    let mesh = world.spawn((Transform::from_xyz(0.0, 4.0, 0.0), ChildOf(node), Mesh3d::default())).id();
    let loose = world.spawn((Transform::default(), Mesh3d::default())).id();
    world.run_system_once(apply_subtree_ranges).expect("runs");
    // Offset |(3, 4, 0)| × 2 = 10 m: fade 110..120 m, snapped outward onto the shared grid.
    let range = world.get::<VisibilityRange>(mesh).expect("range");
    let near = |v: f32, want: f32| v >= want && v < want * 1.022;
    assert!(near(range.end_margin.start, 110.0) && near(range.end_margin.end, 120.0), "{:?}", range.end_margin);
    assert!(world.get::<VisibilityRange>(loose).is_none(), "meshes outside a subtree keep no range");
}

/// Size-based draw distance (industry-audit.md J1): small placements end at
/// their screen-size distance with a fade; exempt classes, flagged and
/// proxy-kept placements keep their ring or handoff.
#[test]
fn size_culling_shortens_small_placements_only() {
    let lod = sf_model::LodScreen::default(); // High: 2 px, 24 m floor.
    let at = |class, flags, radius| sf_world::IndexedPlacement {
        placement: Placement {
            id: PlacementId::from_key("test", b"p"),
            base: RecordRef::new("thing", "p"),
            model: Some(VfsPath::new("cup.mdl").unwrap()),
            transform: WorldTransform::from_pos(WorldPos::from_f64(DVec3::ZERO)),
            class,
            flags,
        },
        radius,
    };
    let none = PlacementFlags::empty();
    // A 6 cm cup in a 648 m ring: ~61.7 m, fading over the last 10 %.
    let (end, band) = size_culled_end(&lod, &at(PlacementClass::Dynamic, none, 0.06), 648.0, 0.0, None);
    assert!((end + band - 61.7).abs() < 0.1 && (band - 6.17).abs() < 0.01, "{end} {band}");
    // Big enough: unchanged, the handoff band too.
    assert_eq!(size_culled_end(&lod, &at(PlacementClass::Static, none, 2.0), 400.0, 40.0, Some(0.3)), (400.0, 40.0));
    // Exempt: actors, doors, the flag, unknown bounds, proxy-kept.
    for (class, flags, radius, kept) in [
        (PlacementClass::Actor, none, 0.06, None),
        (PlacementClass::Door, none, 0.06, Some(1.0)),
        (PlacementClass::Dynamic, PlacementFlags::NO_SIZE_CULL, 0.06, None),
        (PlacementClass::Static, none, 0.0, None),
        (PlacementClass::Static, none, 0.35, Some(0.34)),
    ] {
        assert_eq!(size_culled_end(&lod, &at(class, flags, radius), 400.0, 40.0, kept), (400.0, 40.0), "{class:?}");
    }
    // Not kept by its proxy: culled before the handoff, no lead band.
    let (end, band) = size_culled_end(&lod, &at(PlacementClass::Static, none, 0.2), 400.0, 40.0, Some(0.34));
    assert!(end + band < 400.0 && band > 0.0);
    // Off.
    let off = sf_model::LodScreen { min_pixels: 0.0, ..lod };
    assert_eq!(size_culled_end(&off, &at(PlacementClass::Dynamic, none, 0.06), 648.0, 0.0, None), (648.0, 0.0));
}

/// Per-instance ranges snap onto a shared grid (streaming.md, "Visibility range
/// slots"): few distinct ranges, bounds never pulled in, matched crossfades kept.
#[test]
fn shared_ranges_collapse_per_instance_values() {
    use std::collections::HashSet;
    let lod = sf_model::LodScreen::default();
    let mut distinct = HashSet::new();
    // 20k placements with distinct scales: errors scale per instance.
    for i in 0..20_000u32 {
        #[allow(clippy::cast_precision_loss)]
        let scale = 0.7 + i as f32 * 3.1e-5;
        let errors = [0.0, 0.01 * scale, 0.04 * scale];
        let ranges = chain_ranges(&errors, &lod, 400.0 + scale, 40.0);
        let vis: Vec<_> = ranges.iter().filter_map(visibility_range).collect();
        for (a, b) in vis.iter().zip(vis.iter().skip(1)) {
            assert_eq!(a.end_margin, b.start_margin, "matched crossfade survives rounding");
        }
        for (r, v) in ranges.iter().zip(ranges.iter().map(visibility_range)) {
            let Some(v) = v else { continue };
            assert!(v.end_margin.end >= r.end, "bounds only move outward");
            assert_eq!(v.start_margin.start == 0.0, r.start - r.start_margin * 0.5 <= 0.0);
            distinct.insert([v.start_margin.start, v.start_margin.end, v.end_margin.start, v.end_margin.end].map(f32::to_bits));
        }
    }
    assert!(distinct.len() < 2_000, "{} distinct ranges", distinct.len());
    let q = shared_range(bevy::camera::visibility::VisibilityRange {
        start_margin: 0.0..0.0,
        end_margin: 100.0..110.0,
        use_aabb: false,
    });
    assert!(q.end_margin.start >= 100.0 && q.end_margin.start < 100.0 * 1.022);
}
