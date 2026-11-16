//! Live view distance (tunables T5c, `lod_live`): a screen change re-ranges the resident LOD chains to
//! exactly what a fresh spawn under the new screen gives, without restreaming or touching anything
//! else, within its per-frame budget, while the streaming selection gate skips.

use bevy::camera::visibility::{NoCpuCulling, VisibilityRange};
use bevy::prelude::*;
use big_space::prelude::FloatingOrigin;
use glam::DVec3;
use sf_coords::WorldPos;
use sf_model::LodScreen;
use sf_world::{IndexParams, SchedulerConfig};

use super::tests::{settle, streaming_app_config, ws};
use super::*;

/// The camera and the gameplay anchor at the origin (as the end-to-end streaming test).
fn anchors(app: &mut App) {
    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &ws()).root;
    let (cell, transform) = to_grid(WorldPos::from_f64(DVec3::new(0.0, 2.0, 0.0)));
    app.world_mut().spawn((cell, transform, StreamingAnchor::visual(), FloatingOrigin, ChildOf(root)));
    app.world_mut().spawn((to_grid(WorldPos::ORIGIN).0, Transform::IDENTITY, StreamingAnchor::gameplay(), ChildOf(root)));
    app.world_mut().resource_mut::<Streaming>().begin_burst();
}

/// A streamed app under `config`, settled, transforms propagated.
fn streamed(config: StreamingConfig) -> (App, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let mut app = streaming_app_config(dir.path(), IndexParams::default().far_min_radius, config);
    anchors(&mut app);
    settle(&mut app);
    for _ in 0..3 {
        app.update();
    }
    (app, dir)
}

/// One LOD part: position (cm), level, range bounds (bits), GPU-culled.
type PartKey = (i64, i64, i64, u8, [u32; 4], bool);

/// Every LOD part, sorted: what two apps must agree on.
fn parts(app: &mut App) -> Vec<PartKey> {
    let mut q = app.world_mut().query::<(&GlobalTransform, &LodPart, &VisibilityRange, Has<NoCpuCulling>)>();
    #[allow(clippy::cast_possible_truncation)]
    let mut out: Vec<PartKey> = q
        .iter(app.world())
        .map(|(t, p, r, gpu)| {
            let a = (t.translation() * 100.0).round();
            let bounds = [r.start_margin.start, r.start_margin.end, r.end_margin.start, r.end_margin.end].map(f32::to_bits);
            (a.x as i64, a.y as i64, a.z as i64, p.level, bounds, gpu)
        })
        .collect();
    out.sort_unstable();
    out
}

/// Every streamed entity that is not a LOD part (cells, level groups, batches): none may change.
fn others(app: &mut App) -> Vec<Entity> {
    let mut q = app.world_mut().query_filtered::<Entity, (Or<(With<StreamCell>, With<ChildOf>)>, Without<LodPart>)>();
    let mut out: Vec<Entity> = q.iter(app.world()).collect();
    out.sort_unstable();
    out
}

/// LOD parts whose chain is not ranged for the current screen yet.
fn stale(app: &mut App) -> usize {
    let screen = app.world().resource::<Streaming>().config().lod;
    app.world_mut().query::<&LodPart>().iter(app.world()).filter(|p| *p.chain.screen() != screen).count()
}

#[test]
fn a_live_screen_change_re_ranges_resident_chains_as_a_fresh_spawn_would() {
    const BUDGET: usize = 256;
    let high = LodScreen::default();
    // Coarser levels sooner, no size culling: levels a High spawn skipped open.
    let low = LodScreen { bias: 0.25, min_pixels: 0.0, ..high };
    // Size culling ends the plates (r 5.7 m) near 270 m: far levels collapse.
    let culled = LodScreen { bias: 1.5, min_pixels: 64.0, min_draw_distance: 24.0, ..high };
    let config = StreamingConfig {
        gpu_cull_statics: true,
        lod: high,
        lod_live_budget: BUDGET,
        scheduler: SchedulerConfig { select_gate: SchedulerConfig::select_gate_quanta(true), ..SchedulerConfig::default() },
        ..StreamingConfig::default()
    };
    let (mut app, _dir) = streamed(config.clone());
    let at_high = parts(&mut app);
    assert!(at_high.len() > 1000, "{} LOD parts", at_high.len());
    let entities = others(&mut app);
    let start = app.world().resource::<Streaming>().stats();
    let mut previous = at_high.clone();
    for (screen, label) in [(low, "low"), (culled, "culled"), (high, "back to high")] {
        let (_, skips_before) = app.world().resource::<Streaming>().scheduler().gate_counts();
        let chains_before = app.world().resource::<Streaming>().stats().lod_live_chains;
        app.world_mut().resource_mut::<Streaming>().set_lod_screen(screen);
        let mut frames = 0u64;
        let mut last = chains_before;
        while stale(&mut app) > 0 {
            app.update();
            frames += 1;
            let chains = app.world().resource::<Streaming>().stats().lod_live_chains;
            assert!(chains - last <= BUDGET as u64, "{label}: {} chains in one frame", chains - last);
            last = chains;
            assert!(frames < 500, "{label}: never finished");
        }
        let done = last - chains_before;
        assert!(done > 0, "{label}: nothing re-ranged");
        assert!(frames <= done.div_ceil(BUDGET as u64) + 1, "{label}: {done} chains took {frames} frames");
        for _ in 0..3 {
            app.update();
        }
        // The streaming selection stayed gated (the camera never moved): the re-range is not behind it.
        let (_, skips) = app.world().resource::<Streaming>().scheduler().gate_counts();
        assert!(skips > skips_before, "{label}: the selection gate ran every frame");

        let (mut fresh, _fresh_dir) = streamed(StreamingConfig { lod: screen, ..config.clone() });
        let live = parts(&mut app);
        assert_eq!(live.len(), parts(&mut fresh).len(), "{label}: part count");
        assert!(live == parts(&mut fresh), "{label}: ranges differ from a fresh spawn");
        assert_ne!(live, previous, "{label}: the change changed nothing");
        previous = live;
        // Nothing restreamed: cells, groups and batches are the same entities, no spawn or despawn ran.
        assert_eq!(others(&mut app), entities, "{label}: a non-part entity changed");
        let stats = app.world().resource::<Streaming>().stats();
        assert_eq!(
            (stats.entities_spawned, stats.entities_despawned, stats.placements_spawned),
            (start.entities_spawned, start.entities_despawned, start.placements_spawned),
            "{label}: streaming spawned or despawned"
        );
    }
    let stats = app.world().resource::<Streaming>().stats();
    println!(
        "lod_live: {} chains, {} ranges changed, {} parts spawned, {} despawned",
        stats.lod_live_chains, stats.lod_live_ranges_changed, stats.lod_live_parts_spawned, stats.lod_live_parts_despawned
    );
    assert!(stats.lod_live_ranges_changed > 0);
    assert_eq!(parts(&mut app), at_high, "back to High: as first spawned");
}

/// A level a spawn skipped (empty range) opens when the screen gives it one, and a level that
/// collapses is despawned: the parts match a fresh spawn's level by level.
#[test]
fn levels_open_and_collapse_with_the_screen() {
    use bevy::ecs::system::RunSystemOnce;
    use sf_vfs::VfsPath;
    use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};

    // Switches at High (1028 px focal): 103 m, 411 m, 2056 m: the last level is past the 500 m ring.
    let lod = |error: f32| TemplateLod { error, parts: vec![(Handle::default(), Handle::default())] };
    let template = ModelTemplate {
        lods: vec![lod(0.0), lod(0.1), lod(0.4), lod(2.0)],
        aabb: bevy::camera::primitives::Aabb::from_min_max(Vec3::splat(-1.0), Vec3::splat(1.0)),
        ray_tracing: None,
    };
    let model = LoadedModel::Cooked(std::sync::Arc::new(template));
    let placement = IndexedPlacement {
        placement: Placement {
            id: PlacementId::from_key("test", b"p"),
            base: RecordRef::new("thing", "p"),
            model: Some(VfsPath::new("plate.mdl").unwrap()),
            transform: sf_coords::WorldTransform::from_pos(WorldPos::ORIGIN),
            class: PlacementClass::Static,
            flags: PlacementFlags::empty(),
        },
        radius: 1.0,
    };
    let layers = bevy::camera::visibility::RenderLayers::default();
    let end = EndInputs::of(&placement, 500.0, 0.0, None);
    // (level, range bounds) of every part.
    let levels = |world: &mut World| -> Vec<(u8, [u32; 4])> {
        let mut q = world.query::<(&LodPart, &VisibilityRange)>();
        let mut v: Vec<_> = q
            .iter(world)
            .map(|(p, r)| (p.level, [r.start_margin.start, r.start_margin.end, r.end_margin.start, r.end_margin.end].map(f32::to_bits)))
            .collect();
        v.sort_unstable();
        v
    };
    let spawn = |screen: LodScreen| -> World {
        let mut world = World::new();
        world.insert_resource(Streaming::new(StreamingConfig { lod: screen, ..StreamingConfig::default() }));
        let parent = world.spawn(Transform::IDENTITY).id();
        let r = end.resolve(&screen);
        let cx = SpawnCx {
            worldspace: &ws(),
            cell: IVec3::ZERO,
            level: sf_world::Level::Near,
            parent,
            layers: &layers,
            local: Transform::IDENTITY,
            model: Some(&model),
            max_distance: r.distance,
            end_band: r.band,
            end,
            lod: &screen,
        };
        StaticSpawner.spawn(&mut world, &cx, &placement);
        world
    };
    let high = LodScreen::default();
    let mut live = spawn(high);
    assert_eq!(levels(&mut live).iter().map(|l| l.0).collect::<Vec<_>>(), [0, 1, 2], "the last level is past the ring");
    // Bias 0.2, no size culling: switches 21, 82, 411 m, the last level opens. Size culling at 16 px ends the
    // placement at 116 m: levels 2 and 3 collapse.
    let low = LodScreen { bias: 0.2, min_pixels: 0.0, ..high };
    let culled = LodScreen { min_pixels: 16.0, ..high };
    for (screen, want) in [(low, vec![0, 1, 2, 3]), (culled, vec![0, 1]), (high, vec![0, 1, 2])] {
        live.resource_mut::<Streaming>().set_lod_screen(screen);
        live.run_system_once(lod_live::apply_lod_screen).unwrap();
        let got = levels(&mut live);
        assert_eq!(got.iter().map(|l| l.0).collect::<Vec<_>>(), want, "{screen:?}");
        assert_eq!(got, levels(&mut spawn(screen)), "{screen:?}: as a fresh spawn");
    }
    let stats = live.resource::<Streaming>().stats();
    assert_eq!((stats.lod_live_parts_spawned, stats.lod_live_parts_despawned), (2, 2), "opened 3, closed 2 and 3, reopened 2");
}

/// A precombine bucket ends at its farthest-reaching member: a member at the handoff adds no reach
/// unless size culling ends it first; a size-culled member reaches past its own end.
#[test]
fn bucket_ends_follow_the_screen() {
    let high = LodScreen::default();
    let handoff = EndInputs { end: 400.0, band: 40.0, cull_radius: Some(0.2), at_handoff: true };
    let ring = EndInputs { end: 648.0, band: 0.0, cull_radius: Some(0.05), at_handoff: false };
    let off = LodScreen { min_pixels: 0.0, ..high };
    // No size culling: the ring member (648 + reach) beats the handoff member (400, no reach).
    assert_eq!(bucket_end(&[handoff, ring], 3.0, &off), (651.0, 0.0));
    // High: both size-culled, each reaches 3 m past its own end; the larger radius wins.
    let (end, band) = bucket_end(&[handoff, ring], 3.0, &high);
    let h = handoff.resolve(&high);
    assert!(h.size_culled && (end - (h.distance + 3.0)).abs() < 1e-4 && (band - h.band).abs() < 1e-6, "{end} {band} vs {h:?}");
    // A higher bias pushes the culled ends out.
    let far = LodScreen { bias: 2.0, ..high };
    assert!(bucket_end(&[handoff, ring], 3.0, &far).0 > end);
    // Exempt members never move.
    let kept = EndInputs { cull_radius: None, ..handoff };
    assert_eq!(bucket_end(&[kept], 3.0, &high), bucket_end(&[kept], 3.0, &far));
}
