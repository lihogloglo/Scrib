//! Headless tests (no GPU, no picture): the rings, the slot pool, the residency's slot layout and origin-shift
//! rewrite (against a CPU mirror of the batch buffers), the ground cache, and an App driving the viewer across
//! tiles. Plus an `#[ignore]` measurement of instances, bytes and scatter time per tile.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_math::DVec3;
use sf_groundcover::{
    ChannelRule, GroundSample, Groundcover, Overrides, RangeClass, RenderKind, RuleTable, ShadowClass, Span, Species,
    TileKey, WeightedSpecies,
};

use crate::residency::{SlotWrite, SlotWords, batch_index};
use crate::*;

fn species(id: &str, model: &str, footprint: f32, density: f32, shadow: ShadowClass) -> Species {
    Species {
        id: id.into(),
        render: RenderKind::Mesh { model: model.into(), lods: Vec::new() },
        footprint_m: footprint,
        density,
        slope_deg: Some(Span::new(0.0, 40.0)),
        altitude_m: None,
        water_m: None,
        scale: Span::new(0.8, 1.2),
        yaw_deg: Span::new(0.0, 360.0),
        tint: [[0.9, 1.0, 0.9], [1.0, 1.0, 0.8]],
        align_to_surface: Span::new(0.0, 0.5),
        wind: 1.0,
        trample: 1.0,
        z_offset_m: 0.0,
        shadow,
        range: RangeClass::Near,
    }
}

/// Two species on model "fern" (one casting), one on "flower", one blade species (not drawn here).
fn cover() -> Groundcover {
    let mut blade = species("grass", "unused", 0.5, 2.0, ShadowClass::None);
    blade.render = RenderKind::Blade { style: "meadow".into() };
    let all = vec![
        species("fern_a", "fern", 1.0, 0.4, ShadowClass::Near),
        species("fern_b", "fern", 2.0, 0.1, ShadowClass::None),
        species("flower", "flower", 1.0, 0.3, ShadowClass::None),
        blade,
    ];
    let rules = RuleTable {
        rules: vec![ChannelRule {
            channel: 0,
            species: all.iter().map(|s| WeightedSpecies { species: s.id.clone(), weight: 1.0 }).collect(),
        }],
    };
    Groundcover::new(all, &rules).expect("valid cover")
}

/// A gentle tilted plane, one channel.
fn plane(x: f64, z: f64, out: &mut GroundSample) {
    #[allow(clippy::cast_possible_truncation)]
    let h = (0.02 * x + 0.01 * z) as f32;
    out.height = h;
    out.normal = Vec3::new(-0.02, 1.0, -0.01).normalize();
    out.water_distance = f32::MAX;
    out.valid = true;
    out.channels.clear();
    out.channels.push(1.0);
}

fn settings() -> RenderSettings {
    RenderSettings { mesh_range_m: 40.0, hysteresis_m: 6.0, shadow_range_m: 20.0, fade_width_m: 8.0, ..RenderSettings::default() }
}

fn inputs(cover: &Arc<Groundcover>, settings: RenderSettings, species: Arc<[(sf_groundcover::SpeciesIndex, u8)]>) -> JobInputs {
    JobInputs { cover: cover.clone(), overrides: Arc::new(Overrides::none()), ground: Arc::new(FnGround(plane)), settings, species }
}

// ---------------------------------------------------------------------------------------------------------------
// Rings

/// The ring holds exactly the tiles whose nearest point is within range (brute force), and its job order is
/// nearest first, ties by key.
#[test]
fn ring_holds_exactly_the_tiles_within_range() {
    let s = RingSettings { tile_size_m: 16.0, range_m: 40.0, hysteresis_m: 6.0 };
    let p = [5.3, -7.9];
    let mut ring = TileRing::default();
    let delta = ring.update(p, &s, Measure::Nearest);
    let mut expect = BTreeSet::new();
    for z in -10..10 {
        for x in -10..10 {
            let k = TileKey::new(x, z);
            if nearest_distance(k, 16.0, p) <= 40.0 {
                expect.insert(k);
            }
        }
    }
    assert_eq!(ring.members(), &expect);
    assert_eq!(delta.entered.len(), expect.len());
    assert!(delta.left.is_empty());
    let d: Vec<f64> = delta.entered.iter().map(|&k| nearest_distance(k, 16.0, p)).collect();
    assert!(d.windows(2).all(|w| w[0] <= w[1]), "entered nearest first");
    assert_eq!(delta.entered, tiles_within(p, 16.0, 40.0), "same order as the job order");
}

/// A viewer pacing back and forth across a boundary (within the hysteresis) changes nothing after the first step.
#[test]
fn hysteresis_stops_thrash_at_a_boundary() {
    let s = RingSettings { tile_size_m: 16.0, range_m: 40.0, hysteresis_m: 6.0 };
    let mut ring = TileRing::default();
    // Tile x = 3 starts at 48 m; from x = 7.9 its nearest point is 40.1 m away (out), from 8.1 it's 39.9 (in).
    ring.update([7.9, 8.0], &s, Measure::Nearest);
    assert!(!ring.contains(TileKey::new(3, 0)));
    let first = ring.update([8.1, 8.0], &s, Measure::Nearest);
    assert!(first.entered.contains(&TileKey::new(3, 0)));
    // One pass over both positions (the far side's tiles enter once), then nothing changes.
    ring.update([7.0, 8.0], &s, Measure::Nearest);
    ring.update([8.5, 8.0], &s, Measure::Nearest);
    for i in 0..40 {
        let x = if i % 2 == 0 { 7.0 } else { 8.5 };
        let d = ring.update([x, 8.0], &s, Measure::Nearest);
        assert_eq!(d, RingDelta::default(), "step {i} at x = {x} changed the ring");
    }
    // Past the hysteresis it leaves.
    let d = ring.update([1.0, 8.0], &s, Measure::Nearest);
    assert!(d.left.contains(&TileKey::new(3, 0)));
}

/// The ring never holds more tiles than `max_ring_tiles`, wherever the viewer stands.
#[test]
fn max_ring_tiles_bounds_every_position() {
    let s = RingSettings { tile_size_m: 16.0, range_m: 60.0, hysteresis_m: 8.0 };
    let bound = max_ring_tiles(16.0, 60.0, 8.0) as usize;
    let mut worst = 0;
    let mut ring = TileRing::default();
    for i in 0..400 {
        let t = f64::from(i) * 0.37;
        let p = [t * 3.1 + (t * 1.7).sin() * 20.0, t * 2.3 + (t * 0.9).cos() * 25.0];
        ring.update(p, &s, Measure::Nearest);
        worst = worst.max(ring.len());
    }
    assert!(worst <= bound, "ring held {worst} > bound {bound}");
    println!("ring worst {worst}, bound {bound}");
}

// ---------------------------------------------------------------------------------------------------------------
// Slot pool

#[test]
fn pool_allocates_lowest_frees_and_reuses() {
    let mut pool = SlotPool::new(4, 100);
    assert_eq!(pool.capacity(), 400);
    let blocks: Vec<u32> = (0..4).map(|_| pool.allocate().expect("room")).collect();
    assert_eq!(blocks, [0, 1, 2, 3]);
    assert_eq!(pool.allocate(), None, "full");
    assert_eq!(pool.set_live(2, 30), 200..230);
    assert_eq!(pool.set_live(2, 10), 200..230, "shrinking writes zeros over the old tail");
    assert_eq!(pool.set_live(2, 500), 200..300, "clamped to the block");
    assert_eq!(pool.free(2), 200..300);
    assert_eq!(pool.free(0), 0..0, "an empty block frees nothing to zero");
    assert_eq!(pool.allocate(), Some(0), "lowest free first");
    assert_eq!(pool.allocate(), Some(2));
    assert_eq!(pool.live(2), 0, "a reused block starts empty");
}

// ---------------------------------------------------------------------------------------------------------------
// Residency (CPU mirror of the batch buffers)

/// The batch buffers as the GPU would hold them after the writes.
#[derive(Default, PartialEq, Debug)]
struct Mirror(BTreeMap<usize, Vec<SlotWords>>);

impl Mirror {
    fn open(&mut self, batch: usize, capacity: u32) {
        self.0.insert(batch, vec![[0; 16]; capacity as usize]);
    }

    fn apply(&mut self, writes: &[SlotWrite]) {
        for w in writes {
            let buf = self.0.get_mut(&w.batch).expect("write to an open batch");
            let start = w.first_slot as usize;
            buf[start..start + w.slots.len()].copy_from_slice(&w.slots);
        }
    }

    fn active(&self, batch: usize) -> usize {
        self.0.get(&batch).map_or(0, |b| b.iter().filter(|s| s[12] != 0).count())
    }
}

/// Scatters every resident tile synchronously, opens every batch, reconciles; returns the residency and mirror.
fn build(cover: &Arc<Groundcover>, settings: RenderSettings, viewer: [f64; 2], origin: DVec3) -> (Residency, Mirror) {
    let mut r = Residency::new(Plan::new(cover, &settings), settings);
    let mut mirror = Mirror::default();
    for b in 0..r.plan.batch_count() {
        if let Some((blocks, slots)) = r.open_batch(b) {
            mirror.open(b, blocks * slots);
        }
    }
    r.update_rings(viewer);
    let job = inputs(cover, settings, r.job_species());
    for k in r.next_jobs(viewer, &BTreeSet::new(), |_| true, usize::MAX) {
        assert!(r.insert(scatter_tile(&job, k)));
    }
    mirror.apply(&r.reconcile(origin));
    (r, mirror)
}

/// Every placed block is one contiguous run holding exactly its tile's instances, blocks never overlap, and the
/// mirror's active slots are the pools' live slots.
fn assert_layout(r: &Residency, mirror: &Mirror, origin: DVec3) {
    let size = r.settings.scatter.tile_size_m;
    let mut used: BTreeMap<usize, BTreeSet<u32>> = BTreeMap::new();
    for (&tile, data) in r.tiles() {
        for (sp, placed) in data.scatter.species.iter().zip(&data.placed) {
            let Some(p) = placed else {
                let waits = r.plan.species[sp.species.get()]
                    .is_none_or(|p| !r.is_open(batch_index(p.group, p.casts && r.is_casting(tile))));
                assert!(sp.instances.is_empty() || waits, "{tile:?} unplaced");
                continue;
            };
            assert!(used.entry(p.batch).or_default().insert(p.block), "block {p:?} placed twice");
            let pool = r.pool(p.batch).expect("open");
            let first = pool.first_slot(p.block) as usize;
            let buf = &mirror.0[&p.batch];
            for (i, inst) in sp.instances.iter().enumerate() {
                assert_eq!(buf[first + i], slot_words(inst, tile, size, origin), "{tile:?} slot {i}");
            }
            let end = first + pool.block_slots() as usize;
            assert!(buf[first + sp.instances.len()..end].iter().all(|s| s[12] == 0), "{tile:?}: active slot past its run");
        }
    }
    for b in 0..r.plan.batch_count() {
        let live = r.pool(b).map_or(0, SlotPool::live_slots);
        assert_eq!(mirror.active(b) as u64, live, "batch {b}");
    }
}

#[test]
fn plan_groups_species_by_model_and_sizes_blocks_from_the_grid() {
    let cover = Arc::new(cover());
    let s = settings();
    let plan = Plan::new(&cover, &s);
    assert_eq!(plan.groups.len(), 2);
    assert_eq!(plan.groups[0].model, "fern");
    assert_eq!(plan.groups[0].species.len(), 2);
    // fern_a: footprint 1 m on a 16 m tile = 256 cells; fern_b 64 cells.
    assert_eq!(plan.groups[0].block_slots, 256);
    assert!(plan.species[3].is_none(), "blade species are not drawn here");
    let fern_b = plan.species[1].expect("planned");
    assert_eq!((fern_b.group, fern_b.slot, fern_b.casts), (0, 1, false));
    assert!(plan.species[0].expect("planned").casts);
    assert_eq!(plan.batch_layout(batch_index(1, true)).0, 0, "flower never casts: no casting batch");
    let all = max_ring_tiles(16.0, 40.0, 6.0);
    assert_eq!(plan.groups[0].blocks[0], (all * 2).min(s.max_blocks_per_batch));
}

#[test]
fn slot_layout_is_valid_and_deterministic() {
    let cover = Arc::new(cover());
    let origin = DVec3::new(256.0, 0.0, -512.0);
    let (r, mirror) = build(&cover, settings(), [300.0, -470.0], origin);
    assert!(r.stats.scattered > 0 && r.stats.scattered == r.stats.resident);
    assert!(r.stats.casting > 0, "some tiles inside the shadow ring");
    assert_eq!(r.stats.unplaced, 0);
    assert_layout(&r, &mirror, origin);
    assert!(mirror.active(batch_index(0, true)) > 0, "near-shadow fern in the casting batch");
    let (_, again) = build(&cover, settings(), [300.0, -470.0], origin);
    assert_eq!(mirror, again, "same inputs, same buffers");
}

/// After an origin shift the rewritten buffers equal a fresh build at the new origin.
#[test]
fn origin_shift_rewrite_matches_a_fresh_build() {
    let cover = Arc::new(cover());
    let viewer = [1000.0, 2000.0];
    let (mut r, mut mirror) = build(&cover, settings(), viewer, DVec3::new(768.0, 0.0, 1792.0));
    let shifted = DVec3::new(1024.0, 256.0, 2048.0);
    let writes = r.reconcile(shifted);
    assert!(!writes.is_empty());
    mirror.apply(&writes);
    let (_, fresh) = build(&cover, settings(), viewer, shifted);
    assert_eq!(mirror, fresh);
    assert_layout(&r, &mirror, shifted);
    assert!(r.reconcile(shifted).is_empty(), "no shift, nothing to write");
}

/// Walking moves tiles out of the ring (their blocks zeroed) and casting tiles into the plain batch; the layout
/// stays valid at every step.
#[test]
fn walking_frees_and_moves_blocks() {
    let cover = Arc::new(cover());
    let s = settings();
    let origin = DVec3::ZERO;
    let (mut r, mut mirror) = build(&cover, s, [0.0, 0.0], origin);
    let job = inputs(&cover, s, r.job_species());
    for step in 1..=12 {
        let p = [f64::from(step) * 7.0, f64::from(step) * 3.0];
        r.update_rings(p);
        for k in r.next_jobs(p, &BTreeSet::new(), |_| true, usize::MAX) {
            r.insert(scatter_tile(&job, k));
        }
        mirror.apply(&r.reconcile(origin));
        assert_layout(&r, &mirror, origin);
        assert_eq!(r.stats.unplaced, 0, "step {step}: a batch ran out of blocks");
    }
}

/// A tight per-tile cap: the block is the cap, the scatter reports capped tiles.
#[test]
fn cap_sizes_blocks_and_is_reported() {
    let cover = Arc::new(cover());
    let mut s = settings();
    s.scatter.max_instances_per_tile = 10;
    let (r, mirror) = build(&cover, s, [0.0, 0.0], DVec3::ZERO);
    assert_eq!(r.plan.groups[0].block_slots, 10);
    assert!(r.stats.capped > 0);
    assert_layout(&r, &mirror, DVec3::ZERO);
}

/// A late model: nothing is written until its batch opens, then every resident tile lands at once.
#[test]
fn late_batch_gets_every_resident_tile() {
    let cover = Arc::new(cover());
    let s = settings();
    let mut r = Residency::new(Plan::new(&cover, &s), s);
    r.update_rings([0.0, 0.0]);
    let job = inputs(&cover, s, r.job_species());
    for k in r.next_jobs([0.0, 0.0], &BTreeSet::new(), |_| true, usize::MAX) {
        r.insert(scatter_tile(&job, k));
    }
    assert!(r.reconcile(DVec3::ZERO).is_empty(), "no batch open yet");
    let mut mirror = Mirror::default();
    let b = batch_index(1, false);
    let (blocks, slots) = r.open_batch(b).expect("flower batch");
    mirror.open(b, blocks * slots);
    mirror.apply(&r.reconcile(DVec3::ZERO));
    assert!(mirror.active(b) > 0);
    assert_layout(&r, &mirror, DVec3::ZERO);
}

/// On a plane the bilinear ground cache is exact: the cached scatter places the same instances.
#[test]
fn ground_cache_matches_direct_sampling_on_a_plane() {
    let cover = Arc::new(cover());
    let mut s = settings();
    let species = Plan::new(&cover, &s);
    let r = Residency::new(species, s);
    let tile = TileKey::new(3, -2);
    s.ground_cache_m = 0.0;
    let direct = scatter_tile(&inputs(&cover, s, r.job_species()), tile);
    s.ground_cache_m = 2.0;
    let cached = scatter_tile(&inputs(&cover, s, r.job_species()), tile);
    assert!(!direct.cached && cached.cached);
    for (a, b) in direct.species.iter().zip(&cached.species) {
        assert_eq!(a.instances.len(), b.instances.len());
        for (x, y) in a.instances.iter().zip(&b.instances) {
            assert_eq!((x.local[0], x.local[2], x.tag), (y.local[0], y.local[2], y.tag));
            assert!((x.local[1] - y.local[1]).abs() < 1.0e-4, "height {} vs {}", x.local[1], y.local[1]);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// App

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), GroundcoverPlugin));
    app.insert_resource(GroundcoverSource {
        cover: Arc::new(cover()),
        overrides: Arc::new(Overrides::none()),
        ground: Arc::new(FnGround(plane)),
    });
    let mut models = GroundcoverModels::default();
    for m in ["fern", "flower"] {
        models.0.insert(
            m.into(),
            GroundcoverModel { mesh: Handle::default(), material: StandardMaterial::default(), bend_height_m: Some(0.6) },
        );
    }
    app.insert_resource(models);
    app
}

/// Runs frames until no job is pending (or a timeout).
fn settle(app: &mut App) -> GroundcoverStats {
    let start = Instant::now();
    loop {
        app.update();
        let s = *app.world().resource::<GroundcoverStats>();
        if s.pending_jobs == 0 && s.residency.scattered == s.residency.resident && s.residency.resident > 0 {
            app.update();
            return *app.world().resource::<GroundcoverStats>();
        }
        assert!(start.elapsed() < Duration::from_secs(60), "jobs never finished: {s:?}");
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// The plugin follows the viewer: resident tiles = the ring, every tile scattered and placed, live slots = the
/// instances, batches per (model, casting); moving far replaces the whole ring and frees the old blocks.
#[test]
fn app_follows_the_viewer_across_tiles() {
    let mut app = app();
    let viewer = app.world_mut().spawn((GroundcoverViewer, GlobalTransform::from_translation(Vec3::new(5.0, 1.7, 5.0)))).id();
    let first = settle(&mut app);
    let range = RenderSettings::default().mesh_range_m;
    let expect = u32::try_from(tiles_within([5.0, 5.0], 16.0, f64::from(range)).len()).expect("small");
    assert_eq!(first.residency.resident, expect);
    assert_eq!(first.batches, 3, "fern plain + casting, flower plain");
    assert!(first.live_slots > 0 && first.live_slots <= first.capacity_slots);
    assert_eq!(first.residency.unplaced, 0);
    let batches = app.world_mut().query::<&GroundcoverBatch>().iter(app.world()).count();
    assert_eq!(batches, 3);
    let casting = app.world_mut().query_filtered::<&GroundcoverBatch, Without<NotShadowCaster>>().iter(app.world()).count();
    assert_eq!(casting, 1);

    // Walk 400 m east: an entirely new ring.
    app.world_mut().entity_mut(viewer).insert(GlobalTransform::from_translation(Vec3::new(405.0, 1.7, 5.0)));
    let second = settle(&mut app);
    assert_eq!(second.residency.resident, expect, "same ring size elsewhere (same sub-tile position)");
    assert_eq!(second.batches, 3, "batches are kept");
    let uploads = app.world().resource::<GroundcoverUploads>();
    assert!(!uploads.writes.is_empty(), "runs queued (no render world drains them here)");
}

/// Replacing the source rebuilds: old batch entities despawned, new ones spawned.
#[test]
fn replacing_the_source_rebuilds() {
    let mut app = app();
    app.world_mut().spawn((GroundcoverViewer, GlobalTransform::default()));
    settle(&mut app);
    let old: Vec<Entity> = app.world_mut().query_filtered::<Entity, With<GroundcoverBatch>>().iter(app.world()).collect();
    let src = app.world().resource::<GroundcoverSource>().clone();
    app.insert_resource(src);
    settle(&mut app);
    for e in old {
        assert!(app.world().get_entity(e).is_err(), "old batch {e} still alive");
    }
    assert_eq!(app.world_mut().query::<&GroundcoverBatch>().iter(app.world()).count(), 3);
}

/// The `groundcover/*` gauges carry the stats (set in `Last` when `DiagCounters` exists).
#[test]
fn gauges_publish_the_stats() {
    let mut app = app();
    app.init_resource::<DiagCounters>();
    app.world_mut().spawn((GroundcoverViewer, GlobalTransform::default()));
    let stats = settle(&mut app);
    let counters = app.world().resource::<DiagCounters>().0.clone();
    let mut seen = BTreeMap::new();
    counters.drain(|_, def, v| {
        if let Some(v) = v {
            seen.insert(def.name.clone(), v);
        }
    });
    assert_eq!(seen.get("groundcover/tiles/resident").copied(), Some(f64::from(stats.residency.resident)));
    #[allow(clippy::cast_precision_loss)]
    let live = stats.live_slots as f64;
    assert_eq!(seen.get("groundcover/slots/live").copied(), Some(live));
    assert!(seen.contains_key("groundcover/slots/capacity") && seen.contains_key("groundcover/scatter/us"));
}

// ---------------------------------------------------------------------------------------------------------------
// Measurement

/// Instances, bytes and scatter time per tile for a meadow-like set (prints; no limits).
#[test]
#[ignore = "measurement: instances, bytes and scatter us per tile"]
#[allow(clippy::cast_precision_loss)] // printed averages
fn measure_tile_cost() {
    let all = vec![
        species("fern", "fern", 0.75, 0.6, ShadowClass::Near),
        species("flower_a", "flower", 1.0, 0.25, ShadowClass::None),
        species("flower_b", "flower", 1.0, 0.25, ShadowClass::None),
        species("bush", "bush", 2.0, 0.08, ShadowClass::Near),
    ];
    let rules = RuleTable {
        rules: vec![ChannelRule {
            channel: 0,
            species: all.iter().map(|s| WeightedSpecies { species: s.id.clone(), weight: 1.0 }).collect(),
        }],
    };
    let cover = Arc::new(Groundcover::new(all, &rules).expect("valid"));
    for cache in [0.0, 0.5, 1.0] {
        let s = RenderSettings { ground_cache_m: cache, ..RenderSettings::default() };
        let r = Residency::new(Plan::new(&cover, &s), s);
        let job = inputs(&cover, s, r.job_species());
        let tiles: Vec<TileKey> = (0..64).map(|i| TileKey::new(i % 8, i / 8)).collect();
        let start = Instant::now();
        let mut instances = 0usize;
        for &k in &tiles {
            instances += scatter_tile(&job, k).species.iter().map(|s| s.instances.len()).sum::<usize>();
        }
        let us = start.elapsed().as_secs_f64() * 1e6 / tiles.len() as f64;
        let per_tile = instances as f64 / tiles.len() as f64;
        println!(
            "cache {cache} m: {per_tile:.0} instances/tile, {:.1} KB GPU + {:.1} KB CPU per tile, {us:.0} us/tile",
            per_tile * 64.0 / 1024.0,
            per_tile * 36.0 / 1024.0
        );
    }
    let s = RenderSettings::default();
    let plan = Plan::new(&cover, &s);
    for (gi, g) in plan.groups.iter().enumerate() {
        for casting in [false, true] {
            let (blocks, slots) = plan.batch_layout(batch_index(gi, casting));
            println!(
                "batch {} casting {casting}: {blocks} blocks x {slots} slots = {:.1} MB",
                g.model,
                f64::from(blocks * slots) * 64.0 / 1.0e6
            );
        }
    }
}
