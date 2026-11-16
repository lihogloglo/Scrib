//! Exact tests of the lamp cache policy (spec `lamp-shadow-cache.md` S1 row).

use super::*;
use sf_tunables::Tunables;

const A: LampKey = LampKey(1);
const B: LampKey = LampKey(2);
const C: LampKey = LampKey(3);
const ALL: TileMask = TileMask::first(6);

fn mask(tiles: &[u8]) -> TileMask {
    let mut m = TileMask::NONE;
    for &t in tiles {
        m.insert(t);
    }
    m
}

fn cache_with(max_lamps: u32, config: LampCacheConfig) -> LampCache {
    LampCache::new(TileLayout::CUBE, max_lamps, config)
}

/// Plans and confirms every capture it issued (a host whose pipelines are all ready).
fn step(c: &mut LampCache, now: f32) -> FramePlan {
    let plan = c.plan(now).clone();
    for lp in &plan.lamps {
        for t in lp.capture.iter() {
            assert!(c.confirm_capture(lp.lamp, t));
        }
    }
    plan
}

/// Steps until no capture waits; returns the time after.
fn settle(c: &mut LampCache, mut now: f32) -> f32 {
    for _ in 0..64 {
        step(c, now);
        now += 0.016;
        if c.capture_queue().next().is_none() && c.lamps.values().all(Lamp::ready) {
            return now;
        }
    }
    panic!("captures never settled");
}

fn movers(c: &mut LampCache, lamp: LampKey, tiles: &[u8], distance_m: f32) {
    assert!(c.set_movers(
        lamp,
        MoverInput {
            tiles: mask(tiles),
            distance_m
        }
    ));
}

fn states(c: &LampCache, lamp: LampKey) -> Vec<FaceState> {
    (0..6).map(|t| c.face_state(lamp, t).unwrap()).collect()
}

#[test]
fn masks_and_layout() {
    assert_eq!(TileLayout::CUBE.all(), TileMask(0b11_1111));
    assert_eq!(TileLayout::new(0), None);
    assert_eq!(TileLayout::new(MAX_TILES + 1), None);
    assert_eq!(TileLayout::new(4).unwrap().all().count(), 4);
    assert_eq!(TileMask::first(MAX_TILES), TileMask(u8::MAX));
    assert_eq!(mask(&[1, 4]).iter().collect::<Vec<_>>(), vec![1, 4]);
    assert!(!mask(&[1]).contains(MAX_TILES));
}

#[test]
fn admission_queues_captures_then_all_clean() {
    let mut c = cache_with(4, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    assert_eq!(
        c.admit(A, LampTier::Full, ALL),
        Err(AdmitError::AlreadyCached)
    );
    assert_eq!(states(&c, A), vec![FaceState::Capture; 6]);

    // Two captures a frame; unconfirmed ones are drawn again.
    let p = c.plan(0.0).clone();
    assert_eq!(p.lamp(A).unwrap().capture, mask(&[0, 1]));
    assert_eq!(p.lamp(A).unwrap().views, mask(&[0, 1]));
    assert!(!p.lamp(A).unwrap().ready);
    assert_eq!((p.stats.captures, p.stats.queued), (2, 6));
    let p = c.plan(0.016).clone();
    assert_eq!(p.lamp(A).unwrap().capture, mask(&[0, 1]));

    assert!(c.confirm_capture(A, 0));
    assert!(c.confirm_capture(A, 1));
    assert!(!c.confirm_capture(A, 1), "only an issued capture confirms");
    assert!(
        !c.confirm_capture(A, 2),
        "a queued capture was not drawn yet"
    );
    assert_eq!(c.face_state(A, 0), Some(FaceState::Clean));
    assert_eq!(step(&mut c, 0.032).lamp(A).unwrap().capture, mask(&[2, 3]));
    assert_eq!(step(&mut c, 0.048).lamp(A).unwrap().capture, mask(&[4, 5]));
    assert_eq!(c.is_ready(A), Some(true));

    // A still scene: no capture, no copy, no view.
    let p = c.plan(0.064).clone();
    let lp = p.lamp(A).unwrap();
    assert!(lp.ready);
    assert_eq!(
        (lp.capture, lp.movers, lp.views),
        (TileMask::NONE, TileMask::NONE, TileMask::NONE)
    );
    assert!(p.copies.is_empty());
    assert_eq!(p.stats.clean_faces, 6);
    assert_eq!(p.stats.queued, 0);
}

#[test]
fn still_scene_of_many_lamps_ends_all_clean() {
    let mut c = cache_with(8, LampCacheConfig::default());
    for k in 0..8 {
        c.admit(LampKey(k), LampTier::Full, ALL).unwrap();
    }
    assert_eq!(
        c.admit(LampKey(9), LampTier::Full, ALL),
        Err(AdmitError::NoSlot)
    );
    let now = settle(&mut c, 0.0);
    let p = c.plan(now);
    assert_eq!(p.stats.lamps, 8);
    assert_eq!(p.stats.ready_lamps, 8);
    assert_eq!(p.stats.clean_faces, 48);
    assert!(p.lamps.iter().all(|l| l.views.is_empty()));
    assert!(p.copies.is_empty());
}

#[test]
fn tiles_without_statics_are_cleared_not_captured() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, mask(&[3])).unwrap();
    let p = step(&mut c, 0.0);
    assert_eq!(p.lamp(A).unwrap().capture, mask(&[3]));
    let cleared: Vec<u8> = p
        .copies
        .iter()
        .filter(|o| o.op == CopyOp::Clear)
        .map(|o| o.tile)
        .collect();
    assert_eq!(cleared, vec![0, 1, 2, 4, 5]);
    assert_eq!(c.is_ready(A), Some(true));

    // A mover in an empty tile restores from the cleared layer and takes no store layer.
    movers(&mut c, A, &[0], 5.0);
    let p = c.plan(0.1).clone();
    assert_eq!(p.lamp(A).unwrap().movers, mask(&[0]));
    assert!(p.copies.is_empty());
    movers(&mut c, A, &[0], 5.0);
    let p = c.plan(0.2).clone();
    assert_eq!(
        p.copies,
        vec![CopyOrder {
            lamp: A,
            tile: 0,
            op: CopyOp::Clear
        }]
    );
    assert_eq!(c.pool().in_use(), 0);
}

#[test]
fn mover_enters_draws_leaves_lingers_and_cleans() {
    let mut c = cache_with(2, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);

    // Enters tile 2: the clean live layer is saved, the mover drawn over it.
    movers(&mut c, A, &[2], 5.0);
    let p = c.plan(t).clone();
    let lp = p.lamp(A).unwrap();
    assert_eq!(
        (lp.movers, lp.views, lp.capture),
        (mask(&[2]), mask(&[2]), TileMask::NONE)
    );
    assert_eq!(
        p.copies,
        vec![CopyOrder {
            lamp: A,
            tile: 2,
            op: CopyOp::Save(StoreLayer(0))
        }]
    );
    assert_eq!(c.face_state(A, 2), Some(FaceState::Movers));
    assert_eq!(c.face_state(A, 3), Some(FaceState::Clean));
    assert_eq!((c.pool().in_use(), c.pool().pinned()), (1, 1));

    // Stays: restore, then draw.
    movers(&mut c, A, &[2], 5.0);
    let p = c.plan(t + 0.1).clone();
    assert_eq!(
        p.copies,
        vec![CopyOrder {
            lamp: A,
            tile: 2,
            op: CopyOp::Restore(StoreLayer(0))
        }]
    );
    assert_eq!(p.lamp(A).unwrap().movers, mask(&[2]));

    // Leaves: one last restore, then the view lingers without drawing.
    let p = c.plan(t + 0.2).clone();
    assert_eq!(
        p.copies,
        vec![CopyOrder {
            lamp: A,
            tile: 2,
            op: CopyOp::Restore(StoreLayer(0))
        }]
    );
    let lp = p.lamp(A).unwrap();
    assert_eq!((lp.movers, lp.views), (TileMask::NONE, mask(&[2])));
    assert_eq!(c.face_state(A, 2), Some(FaceState::Linger));
    assert_eq!(c.pool().pinned(), 0);
    let p = c.plan(t + 0.6).clone();
    assert!(p.copies.is_empty());
    assert_eq!(p.lamp(A).unwrap().views, mask(&[2]));

    // Linger over (0.5 s): clean, no view; the store copy is kept.
    let p = c.plan(t + 0.71).clone();
    assert_eq!(p.lamp(A).unwrap().views, TileMask::NONE);
    assert_eq!(c.face_state(A, 2), Some(FaceState::Clean));
    assert_eq!(c.store_layer(A, 2), Some(StoreLayer(0)));

    // Comes back: the kept copy is still valid, no save.
    movers(&mut c, A, &[2], 5.0);
    let p = c.plan(t + 0.8).clone();
    assert!(p.copies.is_empty());
    assert_eq!(p.lamp(A).unwrap().movers, mask(&[2]));
}

#[test]
fn mover_back_during_linger_draws_again() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    movers(&mut c, A, &[0, 1], 5.0);
    c.plan(t);
    c.plan(t + 0.1);
    assert_eq!(states(&c, A)[..2], [FaceState::Linger, FaceState::Linger]);
    movers(&mut c, A, &[1], 5.0);
    let p = c.plan(t + 0.2).clone();
    assert_eq!(p.lamp(A).unwrap().movers, mask(&[1]));
    assert_eq!(p.lamp(A).unwrap().views, mask(&[0, 1]));
    assert_eq!(states(&c, A)[..2], [FaceState::Linger, FaceState::Movers]);
}

#[test]
fn store_starved_lamp_draws_nothing_then_lru_layer_is_reclaimed() {
    // Two slots, one store layer each: a pool of two.
    let config = LampCacheConfig {
        store_faces_per_lamp: 1,
        ..LampCacheConfig::default()
    };
    let mut c = cache_with(2, config);
    c.admit(A, LampTier::Full, ALL).unwrap();
    c.admit(B, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    assert_eq!(c.pool().len(), 2);

    movers(&mut c, A, &[0], 5.0);
    c.plan(t);
    assert_eq!(c.store_layer(A, 0), Some(StoreLayer(0)));

    // B needs two layers, one is free: the whole lamp waits (no cut at a cube seam).
    movers(&mut c, A, &[0], 5.0);
    movers(&mut c, B, &[0, 1], 5.0);
    let p = c.plan(t + 0.1).clone();
    assert_eq!(p.lamp(B).unwrap().movers, TileMask::NONE);
    assert_eq!(
        p.lamp(B).unwrap().views,
        mask(&[0, 1]),
        "mover faces keep their views"
    );
    assert_eq!(p.stats.store_starved, 2);
    assert_eq!(c.face_state(B, 0), Some(FaceState::Movers));
    assert_eq!(c.pool().in_use(), 1);

    // A's mover leaves: its layer is unpinned and reclaimable; B takes both.
    movers(&mut c, B, &[0, 1], 5.0);
    let p = c.plan(t + 0.2).clone();
    assert_eq!(p.stats.store_starved, 0);
    assert_eq!(p.lamp(B).unwrap().movers, mask(&[0, 1]));
    assert_eq!(c.store_layer(A, 0), None, "evicted");
    let mut saved: Vec<_> = p
        .copies
        .iter()
        .filter(|o| o.lamp == B)
        .map(|o| o.op)
        .collect();
    saved.sort_by_key(|op| match op {
        CopyOp::Save(l) | CopyOp::Restore(l) => l.0,
        CopyOp::Clear => u32::MAX,
    });
    assert_eq!(
        saved,
        vec![CopyOp::Save(StoreLayer(0)), CopyOp::Save(StoreLayer(1))]
    );
    // A's restore was recorded before its layer went to B (copies run in order).
    let restore_a = p.copies.iter().position(|o| o.lamp == A).unwrap();
    let save_b = p.copies.iter().position(|o| o.lamp == B).unwrap();
    assert!(restore_a < save_b);
}

#[test]
fn pool_hands_out_free_first_then_least_recently_used() {
    let mut pool = StorePool::new(2);
    let (l0, v) = pool.take((A, 0), 1).unwrap();
    assert_eq!((l0, v), (StoreLayer(0), None));
    let (l1, _) = pool.take((B, 0), 2).unwrap();
    assert_eq!(l1, StoreLayer(1));
    // Full: the oldest unpinned layer of another lamp goes.
    assert_eq!(pool.take((C, 0), 3), Some((StoreLayer(0), Some((A, 0)))));
    pool.set_pinned(StoreLayer(1), true);
    assert_eq!(pool.available(LampKey(9)), 1);
    assert_eq!(
        pool.take((LampKey(9), 0), 4),
        Some((StoreLayer(0), Some((C, 0))))
    );
    // A lamp never evicts its own layers; pinned layers are never evicted.
    assert_eq!(pool.available(LampKey(9)), 0);
    assert_eq!(pool.take((LampKey(9), 1), 5), None);
    pool.release(StoreLayer(1));
    assert_eq!(pool.take((LampKey(9), 1), 6), Some((StoreLayer(1), None)));
    assert_eq!(pool.owner(StoreLayer(1)), Some((LampKey(9), 1)));
}

#[test]
fn per_frame_cap_takes_longest_waiting_lamps_whole() {
    let config = LampCacheConfig {
        max_mover_faces_per_frame: 3,
        ..LampCacheConfig::default()
    };
    let mut c = cache_with(3, config);
    for k in [A, B, C] {
        c.admit(k, LampTier::Full, ALL).unwrap();
    }
    let t = settle(&mut c, 0.0);
    let frame = |c: &mut LampCache, now: f32| {
        movers(c, A, &[0, 1], 5.0);
        movers(c, B, &[2, 3], 5.0);
        movers(c, C, &[4], 5.0);
        let p = c.plan(now).clone();
        let drawn: Vec<u32> = [A, B, C]
            .iter()
            .map(|&k| p.lamp(k).unwrap().movers.count())
            .collect();
        (drawn, p.stats.deferred_faces, p.stats.mover_draws)
    };
    // Nobody drew yet: key order. A (2) fits, B (2) would make 4 > 3, C (1) fits.
    assert_eq!(frame(&mut c, t), (vec![2, 0, 1], 2, 3));
    // B waited longest.
    assert_eq!(frame(&mut c, t + 0.1), (vec![0, 2, 1], 2, 3));
    // A drew longest ago.
    assert_eq!(frame(&mut c, t + 0.2), (vec![2, 0, 1], 2, 3));

    // A lamp bigger than the cap still draws when it is first.
    let config = LampCacheConfig {
        max_mover_faces_per_frame: 1,
        ..LampCacheConfig::default()
    };
    let mut c = cache_with(1, config);
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    movers(&mut c, A, &[0, 1, 2], 5.0);
    assert_eq!(c.plan(t).stats.mover_draws, 3);
}

#[test]
fn rate_tiers_by_distance() {
    let cfg = LampCacheConfig::default();
    assert_eq!(cfg.rate_tier(0.0), RateTier::Full);
    assert_eq!(cfg.rate_tier(16.0), RateTier::Full);
    assert_eq!(cfg.rate_tier(16.1), RateTier::Half);
    assert_eq!(cfg.rate_tier(30.0), RateTier::Half);
    assert_eq!(cfg.rate_tier(30.1), RateTier::Far);
    assert_eq!(
        [RateTier::Full, RateTier::Half, RateTier::Far].map(|r| cfg.interval_frames(r)),
        [1, 2, 4]
    );
}

/// Which of `lamps` drew movers in each of `frames` frames, all at `distance_m`.
fn draw_pattern(
    c: &mut LampCache,
    lamps: &[LampKey],
    distance_m: f32,
    frames: usize,
    t: f32,
) -> Vec<Vec<bool>> {
    (0..frames)
        .map(|i| {
            for &k in lamps {
                movers(c, k, &[0], distance_m);
            }
            #[allow(clippy::cast_precision_loss)]
            let p = c.plan(t + i as f32 * 0.016).clone();
            lamps
                .iter()
                .map(|&k| !p.lamp(k).unwrap().movers.is_empty())
                .collect()
        })
        .collect()
}

#[test]
fn half_rate_lamps_alternate_and_far_lamps_draw_every_fourth_frame() {
    let mut c = cache_with(2, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    c.admit(B, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    let pattern = draw_pattern(&mut c, &[A, B], 20.0, 7, t);
    // First frame: both new, both draw; then one each frame, each every second frame.
    assert_eq!(pattern[0], vec![true, true]);
    for (i, f) in pattern.iter().enumerate().skip(1) {
        assert_eq!(
            f.iter().filter(|&&d| d).count(),
            1,
            "frame {i}: {pattern:?}"
        );
        assert_ne!(f, &pattern[i - 1], "frame {i}: {pattern:?}");
    }

    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    let drew: Vec<bool> = draw_pattern(&mut c, &[A], 40.0, 13, t)
        .into_iter()
        .map(|f| f[0])
        .collect();
    let frames: Vec<usize> = drew
        .iter()
        .enumerate()
        .filter(|(_, d)| **d)
        .map(|(i, _)| i)
        .collect();
    let gaps: Vec<usize> = frames.windows(2).map(|w| w[1] - w[0]).collect();
    // The first draw is immediate (a new mover); then the lamp's phase, every fourth frame.
    assert_eq!(frames[0], 0);
    assert!(gaps[0] <= 4, "{frames:?}");
    assert_eq!(gaps[1..], vec![4; gaps.len() - 1][..], "{frames:?}");
    assert!(gaps.len() >= 2, "{frames:?}");

    // Near movers: every frame.
    let drew = draw_pattern(&mut c, &[A], 5.0, 4, t + 1.0);
    assert!(drew.iter().all(|f| f[0]));
}

#[test]
fn static_change_dirties_and_recaptures_after_the_interval() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    let effect = c.invalidate(
        A,
        Invalidation::StaticsChanged {
            touched: mask(&[1]),
            statics: ALL,
        },
    );
    assert_eq!(effect, InvalidationEffect::Recapture(mask(&[1])));
    assert_eq!(c.face_state(A, 1), Some(FaceState::Dirty));
    // The last capture was within 2 s: waits, the face stays as it is.
    let p = c.plan(t).clone();
    assert_eq!(p.lamp(A).unwrap().capture, TileMask::NONE);
    assert_eq!(p.stats.dirty_faces, 1);
    assert!(p.lamp(A).unwrap().ready);
    let p = step(&mut c, 2.5);
    assert_eq!(p.lamp(A).unwrap().capture, mask(&[1]));
    assert_eq!(c.face_state(A, 1), Some(FaceState::Clean));
    // The next change waits 2 s from this capture.
    c.invalidate(
        A,
        Invalidation::StaticsChanged {
            touched: mask(&[1]),
            statics: ALL,
        },
    );
    assert_eq!(c.plan(4.0).lamp(A).unwrap().capture, TileMask::NONE);
    assert_eq!(c.plan(4.5).lamp(A).unwrap().capture, mask(&[1]));
}

#[test]
fn static_change_during_an_issued_capture_redoes_it() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    c.plan(0.0);
    c.invalidate(
        A,
        Invalidation::StaticsChanged {
            touched: mask(&[0, 3]),
            statics: ALL,
        },
    );
    // Tile 0 was in flight: dirty once confirmed. Tile 3 was only queued: nothing to redo.
    assert!(c.confirm_capture(A, 0));
    assert_eq!(c.face_state(A, 0), Some(FaceState::Dirty));
    assert!(c.confirm_capture(A, 1));
    assert_eq!(c.face_state(A, 1), Some(FaceState::Clean));
}

#[test]
fn last_static_removed_clears_the_tile_and_frees_its_store() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    movers(&mut c, A, &[4], 5.0);
    c.plan(t);
    c.plan(t + 0.1);
    assert!(c.store_layer(A, 4).is_some());
    c.invalidate(
        A,
        Invalidation::StaticsChanged {
            touched: mask(&[4]),
            statics: mask(&[0, 1, 2, 3, 5]),
        },
    );
    let p = c.plan(3.0).clone();
    assert_eq!(p.lamp(A).unwrap().capture, TileMask::NONE);
    assert_eq!(
        p.copies,
        vec![CopyOrder {
            lamp: A,
            tile: 4,
            op: CopyOp::Clear
        }]
    );
    assert_eq!(c.store_layer(A, 4), None);
    assert_eq!(c.pool().in_use(), 0);
}

#[test]
fn recapture_of_a_mover_tile_skips_its_movers_until_confirmed() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    movers(&mut c, A, &[0, 1], 5.0);
    c.plan(t);
    c.invalidate(
        A,
        Invalidation::StaticsChanged {
            touched: mask(&[0]),
            statics: ALL,
        },
    );
    movers(&mut c, A, &[0, 1], 5.0);
    let p = c.plan(3.0).clone();
    let lp = p.lamp(A).unwrap();
    assert_eq!(
        (lp.capture, lp.movers, lp.views),
        (mask(&[0]), mask(&[1]), mask(&[0, 1]))
    );
    assert_eq!(c.store_layer(A, 0), None, "the stale copy is freed");
    assert!(c.confirm_capture(A, 0));
    // Next frame: tile 0 is clean, saved again and drawn with tile 1.
    movers(&mut c, A, &[0, 1], 5.0);
    let p = c.plan(3.1).clone();
    assert_eq!(p.lamp(A).unwrap().movers, mask(&[0, 1]));
    assert!(
        p.copies
            .iter()
            .any(|o| o.tile == 0 && matches!(o.op, CopyOp::Save(_)))
    );
}

#[test]
fn lamp_removal_causes_free_everything_and_readmission_captures_again() {
    for cause in [
        Invalidation::Despawned,
        Invalidation::SlotLost,
        Invalidation::Moved,
    ] {
        let mut c = cache_with(1, LampCacheConfig::default());
        c.admit(A, LampTier::Full, ALL).unwrap();
        c.admit(B, LampTier::StaticOnly, ALL).unwrap_err();
        let t = settle(&mut c, 0.0);
        movers(&mut c, A, &[0], 5.0);
        c.plan(t);
        assert_eq!(c.pool().in_use(), 1);
        assert_eq!(c.invalidate(A, cause), InvalidationEffect::Removed);
        assert_eq!(c.invalidate(A, cause), InvalidationEffect::Nothing);
        assert!(!c.contains(A));
        assert_eq!(c.pool().in_use(), 0);
        assert!(c.plan(t + 0.1).lamps.is_empty());

        c.admit(A, LampTier::Full, ALL).unwrap();
        assert_eq!(states(&c, A), vec![FaceState::Capture; 6]);
        assert_eq!(c.plan(t + 0.2).lamp(A).unwrap().capture, mask(&[0, 1]));
    }

    // Removal in the middle of the queue drops its captures.
    let mut c = cache_with(2, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    c.admit(B, LampTier::Full, ALL).unwrap();
    c.plan(0.0);
    c.remove(A);
    assert!(c.capture_queue().all(|(k, _)| k == B));
    assert_eq!(c.plan(0.1).lamp(B).unwrap().capture, mask(&[0, 1]));
}

#[test]
fn lamp_toggled_or_changed_does_nothing() {
    let mut c = cache_with(1, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    assert_eq!(
        c.invalidate(A, Invalidation::LightChanged),
        InvalidationEffect::Nothing
    );
    let p = c.plan(t);
    assert_eq!(p.stats.clean_faces, 6);
    assert!(p.copies.is_empty() && p.lamp(A).unwrap().views.is_empty());
}

#[test]
fn static_only_tier_never_draws_movers() {
    let mut c = cache_with(1, LampCacheConfig::default());
    assert_eq!(
        c.admit(A, LampTier::StaticOnly, ALL),
        Err(AdmitError::NoSlot),
        "off by default"
    );
    let config = LampCacheConfig {
        static_tier_slots: 1,
        ..LampCacheConfig::default()
    };
    let mut c = cache_with(1, config);
    c.admit(A, LampTier::StaticOnly, ALL).unwrap();
    c.admit(B, LampTier::Full, ALL).unwrap();
    assert_eq!(
        c.admit(C, LampTier::StaticOnly, ALL),
        Err(AdmitError::NoSlot)
    );
    let t = settle(&mut c, 0.0);
    movers(&mut c, A, &[0, 1], 5.0);
    let p = c.plan(t).clone();
    let lp = p.lamp(A).unwrap();
    assert_eq!((lp.movers, lp.views), (TileMask::NONE, TileMask::NONE));
    assert_eq!(states(&c, A), vec![FaceState::Clean; 6]);
    assert_eq!(c.pool().in_use(), 0);
}

#[test]
fn recapture_all_and_pool_resize() {
    let mut c = cache_with(2, LampCacheConfig::default());
    c.admit(A, LampTier::Full, ALL).unwrap();
    let t = settle(&mut c, 0.0);
    movers(&mut c, A, &[0], 5.0);
    c.plan(t);

    // A new pool size drops the copies; the tile drawn over recaptures at once.
    c.reconfigure(3, LampCacheConfig::default());
    assert_eq!(c.pool().len(), 9);
    assert_eq!(c.store_layer(A, 0), None);
    movers(&mut c, A, &[0], 5.0);
    let p = c.plan(t + 0.1).clone();
    assert_eq!(p.lamp(A).unwrap().capture, mask(&[0]));
    assert_eq!(p.lamp(A).unwrap().movers, TileMask::NONE);
    assert!(p.lamp(A).unwrap().ready);

    c.recapture_all();
    assert_eq!(c.is_ready(A), Some(false));
    assert_eq!(c.pool().in_use(), 0);
    assert_eq!(c.plan(t + 0.2).lamp(A).unwrap().capture, mask(&[0, 1]));
}

#[test]
fn caster_classes() {
    let cfg = LampCacheConfig::default();
    assert_eq!(
        cfg.classify(CasterMobility::Static, 0.0),
        CasterClass::Static
    );
    assert_eq!(
        cfg.classify(CasterMobility::Movable, 100.0),
        CasterClass::Mover
    );
    assert_eq!(cfg.classify(CasterMobility::Rigid, 0.5), CasterClass::Mover);
    assert_eq!(
        cfg.classify(CasterMobility::Rigid, 1.0),
        CasterClass::Static
    );
    assert_eq!(
        cfg.classify(CasterMobility::AnimatedStatic, 0.0),
        CasterClass::Static
    );
    let wind_moves = LampCacheConfig {
        animated_casters_move: true,
        ..cfg
    };
    assert_eq!(
        wind_moves.classify(CasterMobility::AnimatedStatic, 0.0),
        CasterClass::Mover
    );
}

#[test]
fn tunables_register_and_fill_the_config() {
    let mut t = Tunables::default();
    t.register_all(TUNABLES)
        .expect("every def has a range and a unique name");
    assert_eq!(
        LampCacheConfig::from_tunables(&t).unwrap(),
        LampCacheConfig::default()
    );
    let id = t.id_of(&MAX_MOVER_FACES_PER_FRAME).unwrap();
    t.write(id, 20, sf_tunables::SetBy::Console).unwrap();
    assert_eq!(
        LampCacheConfig::from_tunables(&t)
            .unwrap()
            .max_mover_faces_per_frame,
        20
    );
    assert!(STORE_FACES_PER_LAMP.bounds().unwrap().1 <= u32::from(MAX_TILES));
    assert!(LampCacheConfig::from_tunables(&Tunables::default()).is_err());
}
