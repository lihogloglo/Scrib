//! Unit tests for the Bevy-free 2D shadow atlas allocator, policy, and tunables.

use std::collections::HashSet;
use std::f32::consts::FRAC_PI_2;

use sf_tunables::Tunables;

use crate::lamp_cache::LampKey;

use super::*;

fn key(id: u64) -> LampKey {
    LampKey(id)
}

fn rects_overlap(a: AtlasRect, b: AtlasRect) -> bool {
    let ax1 = u32::from(a.x) + u32::from(a.size);
    let ay1 = u32::from(a.y) + u32::from(a.size);
    let bx1 = u32::from(b.x) + u32::from(b.size);
    let by1 = u32::from(b.y) + u32::from(b.size);
    u32::from(a.x) < bx1 && u32::from(b.x) < ax1 && u32::from(a.y) < by1 && u32::from(b.y) < ay1
}

#[test]
fn atlas_rect_uv_and_array() {
    let r = AtlasRect {
        x: 512,
        y: 1024,
        size: 256,
    };
    assert_eq!(r.to_array(), [512, 1024, 256]);
    let (origin, scale) = r.uv_rect(4096);
    assert!((origin[0] - 0.125).abs() < 1e-6);
    assert!((origin[1] - 0.25).abs() < 1e-6);
    assert!((scale - 0.0625).abs() < 1e-6);
}

#[test]
fn buddy_alloc_split_coalesce_and_zero_fragmentation_under_churn() {
    let mut atlas = BuddyAtlas::new(1024, 128, 512);
    assert_eq!(atlas.atlas_size(), 1024);
    assert_eq!(atlas.min_tile(), 128);
    assert_eq!(atlas.max_tile(), 512);
    assert_eq!(atlas.total_texels(), 1024 * 1024);
    assert_eq!(atlas.used_texels(), 0);
    assert_eq!(atlas.free_texels(), 1024 * 1024);
    assert_eq!(atlas.alloc_count(), 0);
    assert!(atlas.occupancy().abs() < 1e-6);

    // Invalid sizes return None without altering state.
    assert_eq!(atlas.alloc(64), None);
    assert_eq!(atlas.alloc(1024), None);
    assert_eq!(atlas.alloc(300), None);

    // Allocate a mix of 512, 256, and 128 tiles filling the entire 1024x1024 atlas:
    // 1x512 (262_144) + 5x256 (327_680) + 28x128 (458_752) = 1_048_576 texels.
    let mut live = Vec::new();
    live.push(atlas.alloc(512).unwrap());
    for _ in 0..5 {
        live.push(atlas.alloc(256).unwrap());
    }
    for _ in 0..28 {
        live.push(atlas.alloc(128).unwrap());
    }
    assert_eq!(atlas.used_texels(), atlas.total_texels());
    assert_eq!(atlas.free_texels(), 0);
    assert!((atlas.occupancy() - 1.0).abs() < 1e-6);
    assert_eq!(atlas.alloc(128), None);

    // Verify no allocated rectangles overlap and all are in bounds and aligned.
    for (i, &a) in live.iter().enumerate() {
        assert!(u32::from(a.x) + u32::from(a.size) <= 1024);
        assert!(u32::from(a.y) + u32::from(a.size) <= 1024);
        assert_eq!(a.x % a.size, 0);
        assert_eq!(a.y % a.size, 0);
        for &b in &live[i + 1..] {
            assert!(!rects_overlap(a, b), "overlap between {a:?} and {b:?}");
        }
    }

    // Heavy churn: repeatedly free half, reallocate mixed sizes, then free everything.
    for round in 0..8 {
        let mut freed = Vec::new();
        let mut kept = Vec::new();
        for (i, rect) in live.drain(..).enumerate() {
            if (i + round) % 2 == 0 {
                assert!(atlas.free(rect));
                // Double free returns false.
                assert!(!atlas.free(rect));
                freed.push(rect);
            } else {
                kept.push(rect);
            }
        }
        live = kept;
        // Re-fill freed space with 128px and 256px tiles.
        while let Some(r) = atlas.alloc(256) {
            live.push(r);
        }
        while let Some(r) = atlas.alloc(128) {
            live.push(r);
        }
        assert_eq!(atlas.free_texels(), 0);
    }

    // Free every tile -> coalesces back to root -> all four 512x512 tiles succeed again!
    for rect in live.drain(..) {
        assert!(atlas.free(rect));
    }
    assert_eq!(atlas.used_texels(), 0);
    assert_eq!(atlas.alloc_count(), 0);
    assert!(atlas.occupancy().abs() < 1e-6);

    let mut quads = HashSet::new();
    for _ in 0..4 {
        let r = atlas
            .alloc(512)
            .expect("full 512x512 tile must fit after full coalesce");
        assert!(quads.insert(r));
    }
    assert_eq!(atlas.alloc(128), None);

    atlas.clear();
    assert_eq!(atlas.used_texels(), 0);
    assert_eq!(atlas.alloc_count(), 0);
    assert!(atlas.alloc(512).is_some());
}

#[test]
fn buddy_prefers_already_split_exact_slot_over_splitting_larger_block() {
    let mut atlas = BuddyAtlas::new(1024, 128, 512);
    // Allocating one 128 splits (0,0,512) -> (0,0,256) -> four 128s inside (0,0)..(256,256).
    let a = atlas.alloc(128).unwrap();
    let b = atlas.alloc(128).unwrap();
    let c = atlas.alloc(128).unwrap();
    let d = atlas.alloc(128).unwrap();
    // All 4 must pack into the same 256x256 quadrant so three 512x512 blocks remain unsplit!
    for r in [a, b, c, d] {
        assert!(r.x < 256 && r.y < 256, "unexpected tile outside first 256 quadrant: {r:?}");
    }
    // The other three 512x512 quadrants must all still be intact.
    assert!(atlas.alloc(512).is_some());
    assert!(atlas.alloc(512).is_some());
    assert!(atlas.alloc(512).is_some());
    assert!(atlas.alloc(512).is_none());
}

#[test]
fn buddy_handover_reserve_is_respected_by_alloc_and_usable_by_handover() {
    // 1024x1024 = 16 tiles of 256x256. Reserve 4 tiles of 256x256.
    let reserve = 4 * 256 * 256;
    let mut atlas = BuddyAtlas::new(1024, 128, 512).with_reserve_texels(reserve);
    assert_eq!(atlas.reserved_texels(), reserve);

    // Regular alloc gets 12 tiles of 256x256, then stops at the reserve boundary.
    for _ in 0..12 {
        assert!(atlas.alloc(256).is_some());
    }
    assert_eq!(atlas.free_texels(), reserve);
    assert_eq!(atlas.alloc(256), None);
    assert_eq!(atlas.alloc(128), None);

    // Handover alloc can consume all 4 reserved 256x256 tiles.
    let mut handover_rects = Vec::new();
    for _ in 0..4 {
        handover_rects.push(atlas.alloc_handover(256).expect("handover uses reserve"));
    }
    assert_eq!(atlas.free_texels(), 0);
    assert_eq!(atlas.alloc_handover(128), None);

    // Lowering the reserve via set_reserve_texels allows regular alloc again after freeing.
    assert!(atlas.free(handover_rects.pop().unwrap()));
    assert_eq!(atlas.alloc(256), None);
    atlas.set_reserve_texels(0);
    assert!(atlas.alloc(256).is_some());
}

#[test]
fn tier_hysteresis_cooldown_and_priority() {
    let cfg = ShadowAtlasConfig::default();
    // Fresh admission (current = None): no hysteresis.
    assert_eq!(pick_tile_size(9.9, 1.0, None, 0.0, &cfg), 512);
    assert_eq!(pick_tile_size(10.0, 1.0, None, 0.0, &cfg), 512);
    assert_eq!(pick_tile_size(10.1, 1.0, None, 0.0, &cfg), 256);
    assert_eq!(pick_tile_size(24.0, 1.0, None, 0.0, &cfg), 256);
    assert_eq!(pick_tile_size(24.1, 1.0, None, 0.0, &cfg), 128);

    // Priority > 1.5 halves effective distance (carried torch at 18 m -> eff 9 m -> 512).
    assert_eq!(pick_tile_size(18.0, 2.0, None, 0.0, &cfg), 512);
    assert_eq!(pick_tile_size(18.0, 1.5, None, 0.0, &cfg), 256);

    // Cooldown prevents any resize while since_resize_s < resize_cooldown_s (2.0 s).
    assert_eq!(pick_tile_size(50.0, 1.0, Some(512), 1.99, &cfg), 512);
    assert_eq!(pick_tile_size(1.0, 1.0, Some(128), 0.5, &cfg), 128);

    // Past cooldown: hysteresis (1.5x) prevents oscillation across 10 m and 24 m.
    // 512 stays 512 up to 10.0 * 1.5 = 15.0 m, then downgrades to 256.
    assert_eq!(pick_tile_size(12.0, 1.0, Some(512), 2.5, &cfg), 512);
    assert_eq!(pick_tile_size(15.0, 1.0, Some(512), 2.5, &cfg), 512);
    assert_eq!(pick_tile_size(15.1, 1.0, Some(512), 2.5, &cfg), 256);

    // 256 stays 256 between 10.0 m and 24.0 * 1.5 = 36.0 m; upgrades at <= 10.0 m.
    assert_eq!(pick_tile_size(10.1, 1.0, Some(256), 2.5, &cfg), 256);
    assert_eq!(pick_tile_size(36.0, 1.0, Some(256), 2.5, &cfg), 256);
    assert_eq!(pick_tile_size(36.1, 1.0, Some(256), 2.5, &cfg), 128);
    assert_eq!(pick_tile_size(10.0, 1.0, Some(256), 2.5, &cfg), 512);

    // 128 upgrades to 256 at <= 24.0 m and directly to 512 at <= 10.0 m.
    assert_eq!(pick_tile_size(24.1, 1.0, Some(128), 2.5, &cfg), 128);
    assert_eq!(pick_tile_size(24.0, 1.0, Some(128), 2.5, &cfg), 256);
    assert_eq!(pick_tile_size(9.0, 1.0, Some(128), 2.5, &cfg), 512);
}

#[test]
fn empty_face_omission_and_sync_static_faces() {
    let mut mgr = ShadowAtlasManager::new(ShadowAtlasConfig::default());
    let granted = mgr
        .admit_static(key(1), 0b00_1101, 256, 0.0, false)
        .expect("admit static");
    assert_eq!(granted, 256);
    assert!(mgr.contains(key(1)));

    let (tile_size, tiles) = mgr.lamp_tiles(key(1)).unwrap();
    assert_eq!(tile_size, 256);
    assert!(matches!(tiles[0], AtlasTileRef::Static(_)));
    assert_eq!(tiles[1], AtlasTileRef::Empty);
    assert!(matches!(tiles[2], AtlasTileRef::Static(_)));
    assert!(matches!(tiles[3], AtlasTileRef::Static(_)));
    assert_eq!(tiles[4], AtlasTileRef::Empty);
    assert_eq!(tiles[5], AtlasTileRef::Empty);

    let stats = mgr.stats();
    assert_eq!(stats.lamps, 1);
    assert_eq!(stats.static_tiles, 3);
    assert_eq!(stats.live_tiles, 0);
    assert_eq!(stats.empty_faces, 3);

    // Change static faces from 0b00_1101 to 0b01_0101 (frees face 3, allocates face 4).
    let newly_alloc = mgr.sync_static_faces(key(1), 0b01_0101);
    assert_eq!(newly_alloc, 0b01_0000);
    let rects = mgr.lamp_static_rects(key(1)).unwrap();
    assert!(rects[0].is_some());
    assert!(rects[1].is_none());
    assert!(rects[2].is_some());
    assert!(rects[3].is_none());
    assert!(rects[4].is_some());
    assert!(rects[5].is_none());
    assert_eq!(mgr.stats().static_tiles, 3);

    // Clearing all static faces frees every static tile.
    assert_eq!(mgr.sync_static_faces(key(1), 0), 0);
    assert_eq!(mgr.stats().static_tiles, 0);
    assert_eq!(mgr.stats().empty_faces, 6);
}

#[test]
fn moving_lamp_uses_only_live_atlas_and_frees_on_remove() {
    let mut mgr = ShadowAtlasManager::new(ShadowAtlasConfig::default());
    let granted = mgr.admit_moving(key(42), 256, 1.0).expect("admit moving");
    assert_eq!(granted, 256);

    let stats = mgr.stats();
    assert_eq!(stats.lamps, 1);
    assert_eq!(stats.static_tiles, 0);
    assert_eq!(stats.static_used_texels, 0);
    assert_eq!(stats.live_tiles, 6);
    assert_eq!(stats.live_used_texels, 6 * 256 * 256);
    assert_eq!(stats.empty_faces, 0);

    let (_, tiles) = mgr.lamp_tiles(key(42)).unwrap();
    for tile in tiles {
        assert!(matches!(
            tile,
            AtlasTileRef::Live {
                static_rect: None,
                ..
            }
        ));
    }

    // sync_live_faces on a moving lamp keeps all 6 live tiles untouched.
    let before = mgr.lamp_live_rects(key(42)).unwrap();
    assert_eq!(mgr.sync_live_faces(key(42), 0), 0b11_1111);
    assert_eq!(mgr.lamp_live_rects(key(42)).unwrap(), before);

    assert!(mgr.remove(key(42)));
    assert!(!mgr.remove(key(42)));
    assert!(!mgr.contains(key(42)));
    assert_eq!(mgr.stats().live_tiles, 0);
    assert_eq!(mgr.stats().live_used_texels, 0);
}

#[test]
fn live_mover_tile_lifecycle_persists_and_frees_promptly() {
    let mut mgr = ShadowAtlasManager::new(ShadowAtlasConfig::default());
    mgr.admit_static(key(7), 0b11_1111, 256, 0.0, false)
        .expect("admit static");

    // Mover enters faces 1 and 4.
    let valid = mgr.sync_live_faces(key(7), 0b01_0010);
    assert_eq!(valid, 0b01_0010);
    assert_eq!(mgr.stats().live_tiles, 2);

    let initial_live = mgr.lamp_live_rects(key(7)).unwrap();
    assert!(initial_live[1].is_some());
    assert!(initial_live[4].is_some());

    // Mover stays across subsequent frames (Half/Far rate tiers): exact same rects persist.
    assert_eq!(mgr.sync_live_faces(key(7), 0b01_0010), 0b01_0010);
    assert_eq!(mgr.lamp_live_rects(key(7)).unwrap(), initial_live);

    // Check AtlasTileRef combines static + live on mover faces and static-only elsewhere.
    let (_, tiles) = mgr.lamp_tiles(key(7)).unwrap();
    assert!(matches!(tiles[0], AtlasTileRef::Static(_)));
    assert!(matches!(
        tiles[1],
        AtlasTileRef::Live {
            static_rect: Some(_),
            ..
        }
    ));

    // Mover leaves face 1, stays on face 4: face 1 freed immediately, face 4 preserved.
    assert_eq!(mgr.sync_live_faces(key(7), 0b01_0000), 0b01_0000);
    let after_partial = mgr.lamp_live_rects(key(7)).unwrap();
    assert!(after_partial[1].is_none());
    assert_eq!(after_partial[4], initial_live[4]);
    assert_eq!(mgr.stats().live_tiles, 1);

    // Mover leaves completely: all live tiles freed immediately.
    assert_eq!(mgr.sync_live_faces(key(7), 0), 0);
    assert_eq!(mgr.stats().live_tiles, 0);
    assert_eq!(mgr.stats().live_used_texels, 0);
}

#[test]
fn graceful_step_down_and_all_or_nothing_rollback_when_full() {
    // 1024x1024 static atlas, 0 handover spares = 4 slots of 512, 16 slots of 256, 64 slots of 128.
    let cfg = ShadowAtlasConfig {
        static_size_px: 1024,
        live_size_px: 512,
        handover_spares: 0,
        ..ShadowAtlasConfig::default()
    };
    let mut mgr = ShadowAtlasManager::new(cfg);

    // Lamp 1 asks for 6 faces at 512: 6 * 512^2 > 1024^2, so it steps down to 256 (6 * 256^2).
    assert_eq!(
        mgr.admit_static(key(1), 0b11_1111, 512, 0.0, false),
        Some(256)
    );
    // Lamp 2 also gets 6 faces at 256 (total 12 of 16 256-slots used; 4 256-slots = 16 128-slots left).
    assert_eq!(
        mgr.admit_static(key(2), 0b11_1111, 512, 0.0, false),
        Some(256)
    );
    // Lamp 3 needs 6 faces: only 4 256-slots remain, so it steps down to 128 (6 of 16 128-slots).
    assert_eq!(
        mgr.admit_static(key(3), 0b11_1111, 256, 0.0, false),
        Some(128)
    );
    // Lamp 4 gets 6 faces at 128 (10 -> 4 128-slots remain).
    assert_eq!(
        mgr.admit_static(key(4), 0b11_1111, 256, 0.0, false),
        Some(128)
    );
    let used_before_fail = mgr.stats().static_used_texels;
    let tiles_before_fail = mgr.stats().static_tiles;

    // Lamp 5 asks for 6 faces when only 4 128-slots remain: must fail with all-or-nothing rollback.
    assert_eq!(mgr.admit_static(key(5), 0b11_1111, 128, 0.0, false), None);
    assert!(!mgr.contains(key(5)));
    assert_eq!(mgr.stats().static_used_texels, used_before_fail);
    assert_eq!(mgr.stats().static_tiles, tiles_before_fail);
    assert_eq!(mgr.stats().alloc_failures, 1);

    // Same all-or-nothing check on live atlas (512x512 = 16 slots of 128):
    // 2 moving lamps take 12 slots of 128; 3rd moving lamp needs 6 but only 4 remain -> rollback.
    assert_eq!(mgr.admit_moving(key(10), 256, 0.0), Some(128));
    assert_eq!(mgr.admit_moving(key(11), 256, 0.0), Some(128));
    let live_used_before = mgr.stats().live_used_texels;
    assert_eq!(mgr.admit_moving(key(12), 128, 0.0), None);
    assert_eq!(mgr.stats().live_used_texels, live_used_before);
    assert_eq!(mgr.stats().alloc_failures, 2);
}

#[test]
fn maybe_resize_and_reconfigure() {
    let mut mgr = ShadowAtlasManager::new(ShadowAtlasConfig::default());
    mgr.admit_static(key(1), 0b00_0111, 256, 0.0, false)
        .expect("admit");

    // Within cooldown: no resize.
    assert_eq!(mgr.maybe_resize(key(1), 5.0, 1.0, 1.0), None);

    // Past cooldown and within 512 threshold: upgrades 256 -> 512.
    assert_eq!(
        mgr.maybe_resize(key(1), 5.0, 1.0, 3.0),
        Some((512, 0b00_0111))
    );
    assert_eq!(mgr.stats().resizes, 1);
    // The old 256 rects stay allocated (retiring) until the new ones are drawn.
    assert_eq!(mgr.stats().static_used_texels, 3 * 512 * 512 + 3 * 256 * 256);
    assert_eq!(mgr.release_retiring(key(1), 0b00_0111), 0b00_0111);
    assert_eq!(mgr.stats().static_used_texels, 3 * 512 * 512);

    // Reconfigure without changing dimensions preserves lamps.
    let mut cfg = ShadowAtlasConfig {
        tier_512_m: 12.0,
        ..ShadowAtlasConfig::default()
    };
    assert!(!mgr.reconfigure(cfg));
    assert!(mgr.contains(key(1)));

    // Reconfigure changing atlas dimensions clears lamps and returns true.
    cfg.static_size_px = 2048;
    assert!(mgr.reconfigure(cfg));
    assert!(!mgr.contains(key(1)));
    assert_eq!(mgr.static_atlas().atlas_size(), 2048);
    assert_eq!(mgr.stats().lamps, 0);
}

#[test]
fn manager_handover_uses_reserved_spares_and_resize_failure_keeps_old_tiles() {
    // 1024x1024 = 16 tiles of 256. 1 handover spare reserves 6 tiles of 256, leaving 10 for regular.
    let cfg = ShadowAtlasConfig {
        static_size_px: 1024,
        live_size_px: 512,
        handover_spares: 1,
        ..ShadowAtlasConfig::default()
    };
    let mut mgr = ShadowAtlasManager::new(cfg);

    // First regular lamp gets 6 faces at 256 (4 regular 256-slots left).
    assert_eq!(
        mgr.admit_static(key(1), 0b11_1111, 256, 0.0, false),
        Some(256)
    );
    // Second regular lamp asking for 6 faces at 256 cannot dip into the 6 reserved 256-slots, so
    // it steps down to 128 (6 * 128^2 = 1.5 256-slots; 2.5 regular 256-slots left).
    assert_eq!(
        mgr.admit_static(key(2), 0b11_1111, 256, 0.0, false),
        Some(128)
    );
    // A handover lamp CAN dip into the reserved 6 x 256-slots and gets full 256!
    assert_eq!(
        mgr.admit_static(key(3), 0b11_1111, 256, 0.0, true),
        Some(256)
    );

    // Lamp 2 trying to upgrade to 512 fails because the atlas cannot fit 6x512; old rects stay!
    let old_rects = mgr.lamp_static_rects(key(2)).unwrap();
    assert_eq!(mgr.maybe_resize(key(2), 1.0, 1.0, 5.0), None);
    assert_eq!(mgr.lamp_static_rects(key(2)).unwrap(), old_rects);
}

#[test]
fn live_mover_steps_down_when_live_atlas_is_tight() {
    // 512x512 live atlas = 4 quadrants of 256x256 (16 slots of 128x128).
    let cfg = ShadowAtlasConfig {
        static_size_px: 1024,
        live_size_px: 512,
        handover_spares: 0,
        ..ShadowAtlasConfig::default()
    };
    let mut mgr = ShadowAtlasManager::new(cfg);
    // Lamp 2 at 128px uses 1 live slot of 128x128, splitting one 256 quadrant into 4x128 (3x128 free,
    // plus 3 whole 256x256 quadrants free).
    mgr.admit_static(key(2), 0b00_0001, 128, 0.0, false)
        .expect("admit 128 static");
    assert_eq!(mgr.sync_live_faces(key(2), 0b00_0001), 0b00_0001);

    // Lamp 1 at 256px activates 6 mover faces: first 3 get 256x256, remaining 3 step down to 128x128.
    mgr.admit_static(key(1), 0b11_1111, 256, 0.0, false)
        .expect("admit 256 static");
    let valid = mgr.sync_live_faces(key(1), 0b11_1111);
    assert_eq!(valid, 0b11_1111);
    let live_rects = mgr.lamp_live_rects(key(1)).unwrap();
    assert_eq!(live_rects[0].unwrap().size, 256);
    assert_eq!(live_rects[1].unwrap().size, 256);
    assert_eq!(live_rects[2].unwrap().size, 256);
    assert_eq!(live_rects[3].unwrap().size, 128);
    assert_eq!(live_rects[4].unwrap().size, 128);
    assert_eq!(live_rects[5].unwrap().size, 128);
}

#[test]
fn guard_band_math_and_luminance_weight() {
    // Zero guard band -> exact 90 degree cube face.
    assert!((guard_band_tan_half(256, 0.0) - 1.0).abs() < 1e-6);
    assert!((guard_band_inv_tan(256, 0.0) - 1.0).abs() < 1e-6);
    assert!((guard_band_fov_y(256, 0.0) - FRAC_PI_2).abs() < 1e-6);

    // 2 texel guard band on 256 px tile -> inner = 252, tan_half = 256/252.
    let tan_h = guard_band_tan_half(256, 2.0);
    let inv_t = guard_band_inv_tan(256, 2.0);
    let fov = guard_band_fov_y(256, 2.0);
    assert!((tan_h - (256.0 / 252.0)).abs() < 1e-6);
    assert!((inv_t - (252.0 / 256.0)).abs() < 1e-6);
    assert!((tan_h * inv_t - 1.0).abs() < 1e-6);
    assert!(fov > FRAC_PI_2);
    // Smaller tile needs proportionally wider FOV for the same texel guard band.
    assert!(guard_band_fov_y(128, 2.0) > fov);
    assert!(guard_band_fov_y(512, 2.0) < fov);

    // Luminance weights: bright brazier > standard lamp > dim candle > 0.
    let brazier = luminance_weight(4000.0, 1.0);
    let lamp = luminance_weight(1000.0, 1.0);
    let candle = luminance_weight(120.0, 1.0);
    let zero = luminance_weight(0.0, 0.0);
    assert!((brazier - 2.0).abs() < 1e-6);
    assert!((lamp - 1.0).abs() < 1e-6);
    assert!((candle - 0.12_f32.sqrt()).abs() < 1e-6);
    assert!(brazier > lamp && lamp > candle && candle > zero && zero > 0.0);
}

#[test]
fn tunables_register_cleanly_and_config_clamping() {
    let mut reg = Tunables::default();
    reg.register_all(TUNABLES)
        .expect("shadow_atlas::TUNABLES register with no duplicates");
    // Also verify no name collisions with lamp_cache::TUNABLES.
    reg.register_all(crate::lamp_cache::TUNABLES)
        .expect("shadow_atlas::TUNABLES do not collide with lamp_cache::TUNABLES");

    let cfg = ShadowAtlasConfig::from_tunables(&reg).expect("read config from tunables");
    assert_eq!(cfg, ShadowAtlasConfig::default());
    assert_eq!(cfg.clamped_static_size(), 4096);
    assert_eq!(cfg.clamped_live_size(), 2048);

    // Power-of-two clamping checks.
    let mut weird = cfg;
    weird.static_size_px = 500;
    assert_eq!(weird.clamped_static_size(), 1024);
    weird.static_size_px = 3000;
    assert_eq!(weird.clamped_static_size(), 2048);
    weird.static_size_px = 3500;
    assert_eq!(weird.clamped_static_size(), 4096);
    weird.static_size_px = 16000;
    assert_eq!(weird.clamped_static_size(), 8192);

    weird.live_size_px = 256;
    assert_eq!(weird.clamped_live_size(), 512);
    weird.live_size_px = 1500;
    assert_eq!(weird.clamped_live_size(), 1024);
    weird.live_size_px = 1600;
    assert_eq!(weird.clamped_live_size(), 2048);
    weird.live_size_px = 8192;
    assert_eq!(weird.clamped_live_size(), 4096);
}

/// SH-33 (2026-11-20): a tier resize keeps each face's old rect (its last captured depth) until the
/// host reports the new rect drawn, so the shader never samples an undrawn rect nor loses the face.
#[test]
fn resize_keeps_the_old_rect_until_the_new_one_is_drawn() {
    let mut mgr = ShadowAtlasManager::new(ShadowAtlasConfig::default());
    mgr.admit_static(key(1), 0b00_0011, 256, 0.0, false).expect("admit");
    let first = mgr.lamp_static_rects(key(1)).unwrap();

    // 256 -> 512: new rects assigned, the old ones retiring and never handed out meanwhile.
    assert_eq!(mgr.maybe_resize(key(1), 5.0, 1.0, 3.0), Some((512, 0b00_0011)));
    assert_eq!(mgr.lamp_retiring_static(key(1)).unwrap(), first);
    let second = mgr.lamp_static_rects(key(1)).unwrap();
    for (old, new) in first.iter().zip(second.iter()) {
        if let (Some(old), Some(new)) = (old, new) {
            assert!(!rects_overlap(*old, *new), "old and new rects are both live");
        }
    }
    mgr.admit_static(key(2), 0b11_1111, 512, 0.0, false).expect("other lamp");
    for r in mgr.lamp_static_rects(key(2)).unwrap().into_iter().flatten() {
        for old in first.into_iter().flatten() {
            assert!(!rects_overlap(r, old), "a retiring rect was reallocated");
        }
    }

    // Resized again before its new rects were drawn: the never-drawn 512s are freed, the drawn
    // 256s stay the ones to sample.
    assert_eq!(mgr.maybe_resize(key(1), 20.0, 1.0, 6.0), Some((256, 0b00_0011)));
    assert_eq!(mgr.lamp_retiring_static(key(1)).unwrap(), first);

    // Face 0 drawn: only its retiring rect is released.
    assert_eq!(mgr.release_retiring(key(1), 0b00_0001), 0b00_0001);
    let retiring = mgr.lamp_retiring_static(key(1)).unwrap();
    assert_eq!((retiring[0], retiring[1]), (None, first[1]));

    // Removing the lamp frees its retiring rects too.
    let used = mgr.stats().static_used_texels;
    assert!(mgr.remove(key(1)));
    assert_eq!(mgr.stats().static_used_texels, used - 2 * 256 * 256 - 256 * 256);
}
