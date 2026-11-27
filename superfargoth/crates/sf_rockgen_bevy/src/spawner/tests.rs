//! RG-P3: `RockSpawner` in a headless `World` (exact: part counts, shared handles, ranges, culling tags).

use std::collections::HashSet;
use std::ops::Range;

use bevy_asset::{AssetId, Assets, Handle};
use bevy_camera::visibility::{NoCpuCulling, RenderLayers, VisibilityRange};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_transform::components::Transform;
use sf_coords::{WorldPos, WorldTransform};
use sf_rockgen::placed::{Libraries, model_id};
use sf_rockgen::{Aabb, Library, RockLods, RockMesh, Variant};
use sf_vfs::VfsPath;
use sf_world::{IndexedPlacement, Level, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{EndInputs, LoadedModel, PlacementSpawner, SpawnCx, StaticGpuCulling, shared_range};

use super::{RockMaterialSet, RockSpawner};
use crate::{RockLodSettings, lod_ranges};

const LEVELS: usize = 3;

fn tetra(s: f32) -> RockMesh {
    RockMesh {
        positions: vec![[0.0, 0.0, 0.0], [s, 0.0, 0.0], [0.0, s, 0.0], [0.0, 0.0, s * 0.5]],
        normals: vec![[0.0, 0.0, 1.0]; 4],
        indices: vec![0, 2, 1, 0, 1, 3, 0, 3, 2, 1, 2, 3],
    }
}

fn variant(seed: u64) -> Variant {
    #[allow(clippy::cast_precision_loss)] // small seeds
    let s = 1.0 + seed as f32 * 0.1;
    let lods = (0..LEVELS).map(|_| tetra(s)).collect();
    Variant { seed, lods: RockLods { lods, bounds: Aabb { min: [0.0; 3], max: [s, s, s * 0.5] }, hash: seed }, hull: None }
}

/// Two classes: `block` (2 variants, reference size 2 m) and `talus` (1 variant, 1 m).
fn libraries() -> Libraries {
    Libraries::new([
        ("block".to_owned(), Library { key: 1, size_m: 2.0, variants: vec![variant(1), variant(2)] }),
        ("talus".to_owned(), Library { key: 2, size_m: 1.0, variants: vec![variant(3)] }),
    ])
}

fn settings() -> RockLodSettings {
    RockLodSettings { lod1_distance: 10.0, growth: 2.0, fade: 0.1 }
}

fn spawner() -> RockSpawner {
    let mut meshes = Assets::<Mesh>::default();
    let materials = RockMaterialSet { default: vec![Handle::default()], ..RockMaterialSet::default() };
    RockSpawner::new(&libraries(), &mut meshes, &materials, settings())
}

fn rock(key: &[u8], model: &str) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id: PlacementId::from_key("test", key),
            base: RecordRef::new("rockgen", "block"),
            model: Some(VfsPath::new(model).unwrap()),
            transform: WorldTransform::from_pos(WorldPos::ORIGIN),
            class: PlacementClass::Custom(3),
            flags: PlacementFlags::empty(),
        },
        radius: 1.0,
    }
}

/// Spawns `rock` under a fresh parent with scale 1.5 and the given end; returns the parent.
fn spawn(world: &mut World, spawner: &RockSpawner, rock: &IndexedPlacement, max_distance: f32, end_band: f32) -> (Entity, u32) {
    let parent = world.spawn(Transform::IDENTITY).id();
    let layers = RenderLayers::default();
    let lod = sf_model::LodScreen::default();
    let model: Option<&LoadedModel> = None;
    let cx = SpawnCx {
        worldspace: &WorldspaceId::new("test"),
        cell: IVec3::ZERO,
        level: Level::Near,
        parent,
        layers: &layers,
        local: Transform::from_scale(Vec3::splat(1.5)),
        model,
        max_distance,
        end_band,
        end: EndInputs::fixed(max_distance, end_band),
        lod: &lod,
    };
    let n = spawner.spawn(world, &cx, rock);
    (parent, n)
}

/// The parts under `parent`, LOD order (ranges start farther per level): (mesh id, range, gpu-culled).
fn parts(world: &mut World, parent: Entity) -> Vec<(AssetId<Mesh>, VisibilityRange, bool)> {
    let mut q = world.query::<(&ChildOf, &Mesh3d, &VisibilityRange, Has<NoCpuCulling>)>();
    let mut out: Vec<_> =
        q.iter(world).filter(|(c, ..)| c.parent() == parent).map(|(_, m, r, g)| (m.0.id(), r.clone(), g)).collect();
    out.sort_by(|a, b| a.1.start_margin.start.total_cmp(&b.1.start_margin.start));
    out
}

/// A range's bounds (`VisibilityRange` is not `Debug`).
fn bounds(r: &VisibilityRange) -> (Range<f32>, Range<f32>, bool) {
    (r.start_margin.clone(), r.end_margin.clone(), r.use_aabb)
}

#[test]
fn a_rock_spawns_one_part_per_lod_with_shared_meshes_per_variant() {
    let spawner = spawner();
    assert_eq!(spawner.mesh_count(), 3 * LEVELS);
    let mut world = World::new();
    let (first, count) = spawn(&mut world, &spawner, &rock(b"a", &model_id("block", 1)), 500.0, 0.0);
    assert_eq!(count as usize, LEVELS);
    let (twin, _) = spawn(&mut world, &spawner, &rock(b"b", &model_id("block", 1)), 500.0, 0.0);
    let (talus, _) = spawn(&mut world, &spawner, &rock(b"c", &model_id("talus", 0)), 500.0, 0.0);
    let (other, _) = spawn(&mut world, &spawner, &rock(b"d", &model_id("block", 0)), 500.0, 0.0);
    let ids = |world: &mut World, e| parts(world, e).into_iter().map(|p| p.0).collect::<Vec<_>>();
    let first = ids(&mut world, first);
    let (twin, talus, other) = (ids(&mut world, twin), ids(&mut world, talus), ids(&mut world, other));
    assert_eq!(first.len(), LEVELS);
    assert_eq!(first, twin, "two rocks of one variant share every mesh handle");
    let set = |v: &[AssetId<Mesh>]| v.iter().copied().collect::<HashSet<_>>();
    assert!(set(&first).is_disjoint(&set(&talus)), "two classes share no mesh");
    assert!(set(&first).is_disjoint(&set(&other)), "two variants share no mesh");
}

#[test]
fn ranges_follow_the_rock_law_and_end_at_the_placement_end() {
    let spawner = spawner();
    // block reference 2 m x scale 1.5.
    let law = lod_ranges(3.0, LEVELS, &settings());
    for band in [0.0, 40.0] {
        let mut world = World::new();
        let (parent, n) = spawn(&mut world, &spawner, &rock(b"a", &model_id("block", 0)), 500.0, band);
        let got = parts(&mut world, parent);
        assert_eq!(n as usize, LEVELS);
        for (k, (_, range, _)) in got.iter().enumerate() {
            let mut want = law[k].clone();
            if k + 1 == LEVELS {
                want.end_margin = 500.0..500.0 + band;
            }
            // The snap of the expected input, never a re-snap of the output.
            assert_eq!(bounds(range), bounds(&shared_range(want)), "level {k}, band {band}");
        }
        let last = &got[LEVELS - 1].1;
        assert_eq!(last.end_margin, shared_range(VisibilityRange { start_margin: 0.0..0.0, end_margin: 500.0..500.0 + band, use_aabb: false }).end_margin);
    }
    // An end before LOD2's switch (60 m, fade from 54 m): LOD2 is dropped and LOD1 ends there.
    let mut world = World::new();
    let (parent, n) = spawn(&mut world, &spawner, &rock(b"a", &model_id("block", 0)), 40.0, 0.0);
    assert_eq!(n, 2);
    let got = parts(&mut world, parent);
    let mut want = law[1].clone();
    want.end_margin = 40.0..40.0;
    assert_eq!(bounds(&got[1].1), bounds(&shared_range(want)));
}

#[test]
fn gpu_culling_tags_iff_on_and_unknown_ids_spawn_nothing() {
    let spawner = spawner();
    for (on, resource) in [(true, Some(true)), (false, Some(false)), (false, None)] {
        let mut world = World::new();
        if let Some(r) = resource {
            world.insert_resource(StaticGpuCulling(r));
        }
        let (parent, _) = spawn(&mut world, &spawner, &rock(b"a", &model_id("talus", 0)), 500.0, 0.0);
        let got = parts(&mut world, parent);
        assert_eq!(got.len(), LEVELS);
        assert!(got.iter().all(|p| p.2 == on), "NoCpuCulling iff StaticGpuCulling(true): {resource:?}");
    }
    let mut world = World::new();
    for bad in [model_id("talus", 1), model_id("scoria", 0), "meshes/rock.nif".to_owned()] {
        assert_eq!(spawn(&mut world, &spawner, &rock(b"x", &bad), 500.0, 0.0).1, 0, "{bad}");
    }
    let mut none = rock(b"y", &model_id("talus", 0));
    none.placement.model = None;
    assert_eq!(spawn(&mut world, &spawner, &none, 500.0, 0.0).1, 0);
    assert_eq!(world.query::<&Mesh3d>().iter(&world).count(), 0);
    assert!(!spawner.needs_model(&rock(b"a", &model_id("talus", 0))));
}
