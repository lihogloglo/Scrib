//! Tier selection, guard-band math, perceptual luminance weighting, and [`ShadowAtlasManager`].

use std::collections::BTreeMap;

use crate::lamp_cache::LampKey;

use super::allocator::{AtlasRect, BuddyAtlas};
use super::tunables::ShadowAtlasConfig;

/// Cube-map face count per point lamp.
const FACES: usize = 6; // tunables: const (layout: cube map faces)

/// Bitmask covering all 6 cube-map faces (`bits 0..6`).
const ALL_FACES_MASK: u8 = 0b0011_1111; // tunables: const (layout: 6 cube-map face bits)

/// Smallest supported tile resolution tier in pixels.
const TILE_TIER_128: u16 = 128; // tunables: const (layout: lowest atlas tile tier)

/// Medium tile resolution tier in pixels.
const TILE_TIER_256: u16 = 256; // tunables: const (layout: medium atlas tile tier)

/// Largest supported tile resolution tier in pixels.
const TILE_TIER_512: u16 = 512; // tunables: const (layout: highest atlas tile tier)

/// Nominal tile side length used to compute handover spare texel reservations.
const HANDOVER_TILE_PX: u64 = 256; // tunables: const (layout: handover reserve tile size)

/// Tangent of half the expanded face FOV that adds `guard_texels` of border padding on each edge
/// of a `tile_px × tile_px` tile, so PCF kernels never bleed into neighbouring atlas tiles.
#[allow(clippy::cast_precision_loss)]
pub fn guard_band_tan_half(tile_px: u32, guard_texels: f32) -> f32 {
    let tile = tile_px as f32;
    let inner = (tile - 2.0 * guard_texels.max(0.0)).max(1.0);
    tile / inner
}

/// Inverse of [`guard_band_tan_half`]: scales projected face coordinates inward so `[-1, 1]` maps
/// to the guard-band-inset inner region of the tile.
pub fn guard_band_inv_tan(tile_px: u32, guard_texels: f32) -> f32 {
    1.0 / guard_band_tan_half(tile_px, guard_texels)
}

/// Expanded cube-face field of view in radians for a `tile_px × tile_px` tile with `guard_texels`
/// of guard-band padding on every edge.
pub fn guard_band_fov_y(tile_px: u32, guard_texels: f32) -> f32 {
    2.0 * guard_band_tan_half(tile_px, guard_texels).atan()
}

/// Perceptual importance weight of a lamp from its steady luminous flux (`steady_intensity_lm`)
/// and linear RGB colour luminance (`color_luminance`).
///
/// Normalized so ~1000 lm white scores `1.0`, a 4000 lm brazier scores `2.0`, and a dim candle
/// scores `~0.35`; floored at `sqrt(0.05)` so dim lights never score zero.
pub fn luminance_weight(steady_intensity_lm: f32, color_luminance: f32) -> f32 {
    ((steady_intensity_lm.max(0.0) * color_luminance.max(0.0)) / 1000.0)
        .max(0.05)
        .sqrt()
}

/// Selects a lamp's tile resolution tier (`512`, `256`, or `128`) from viewer distance, priority,
/// current tier, elapsed seconds since last resize, and [`ShadowAtlasConfig`].
///
/// Hero / carried lights (`priority > 1.5`) halve their effective distance to bias toward sharper
/// tiles. Downgrades apply [`ShadowAtlasConfig::tier_hysteresis`], and any resize is gated by
/// [`ShadowAtlasConfig::resize_cooldown_s`].
pub fn pick_tile_size(
    distance_m: f32,
    priority: f32,
    current: Option<u16>,
    since_resize_s: f32,
    cfg: &ShadowAtlasConfig,
) -> u16 {
    let eff_d = if priority > 1.5 {
        distance_m * 0.5
    } else {
        distance_m
    };
    let target = if eff_d <= cfg.tier_512_m {
        TILE_TIER_512
    } else if eff_d <= cfg.tier_256_m {
        TILE_TIER_256
    } else {
        TILE_TIER_128
    };

    let Some(cur) = current else {
        return target;
    };
    if since_resize_s < cfg.resize_cooldown_s {
        return cur;
    }

    let h = cfg.tier_hysteresis.max(1.0);
    match cur {
        TILE_TIER_512 => {
            if eff_d <= cfg.tier_512_m * h {
                TILE_TIER_512
            } else if eff_d <= cfg.tier_256_m * h {
                TILE_TIER_256
            } else {
                TILE_TIER_128
            }
        }
        TILE_TIER_256 => {
            if eff_d <= cfg.tier_512_m {
                TILE_TIER_512
            } else if eff_d <= cfg.tier_256_m * h {
                TILE_TIER_256
            } else {
                TILE_TIER_128
            }
        }
        // TILE_TIER_128 (or any non-standard tier): upgrades immediately on the base thresholds.
        _ => target,
    }
}

/// How one cube face of a lamp is backed in the shadow atlases.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasTileRef {
    /// Face has neither static casters nor active movers (omitted from both atlases).
    Empty,
    /// Face is backed solely by a cached static tile in the static atlas.
    Static(AtlasRect),
    /// Face has an active live tile in the live atlas (optionally composited over `static_rect`).
    Live {
        /// Underlying cached static depth tile, if the face has static casters.
        static_rect: Option<AtlasRect>,
        /// Active tile in the live atlas holding mover or moving-lamp depth.
        live_rect: AtlasRect,
    },
}

/// Point-in-time occupancy and event counters for [`ShadowAtlasManager`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AtlasStats {
    /// Number of lamps currently tracked.
    pub lamps: u32,
    /// Active tile allocations in the static atlas.
    pub static_tiles: u32,
    /// Active tile allocations in the live atlas.
    pub live_tiles: u32,
    /// Faces across tracked lamps currently in [`AtlasTileRef::Empty`].
    pub empty_faces: u32,
    /// Allocated texels in the static atlas.
    pub static_used_texels: u64,
    /// Total texels in the static atlas.
    pub static_total_texels: u64,
    /// Allocated texels in the live atlas.
    pub live_used_texels: u64,
    /// Total texels in the live atlas.
    pub live_total_texels: u64,
    /// Cumulative allocation failures (including per-face live/static failures and failed resizes).
    pub alloc_failures: u32,
    /// Cumulative successful lamp tier resizes.
    pub resizes: u32,
}

#[derive(Clone, Debug)]
struct LampEntry {
    tile_size: u16,
    last_resize_s: f32,
    static_rects: [Option<AtlasRect>; FACES],
    live_rects: [Option<AtlasRect>; FACES],
    /// Pre-resize static rects still holding this lamp's last captured depth: sampled until the
    /// new rect is drawn, then released ([`ShadowAtlasManager::release_retiring`], SH-33).
    retiring_static: [Option<AtlasRect>; FACES],
    moving: bool,
    handover: bool,
}

/// Owns the static and live [`BuddyAtlas`] allocators and manages per-lamp tile assignments.
#[derive(Clone, Debug)]
pub struct ShadowAtlasManager {
    config: ShadowAtlasConfig,
    static_atlas: BuddyAtlas,
    live_atlas: BuddyAtlas,
    lamps: BTreeMap<LampKey, LampEntry>,
    alloc_failures: u32,
    resizes: u32,
    static_free_gen: u64,
}

impl ShadowAtlasManager {
    /// Creates a new manager with static and live buddy atlases sized from `config`.
    pub fn new(config: ShadowAtlasConfig) -> Self {
        let static_atlas =
            BuddyAtlas::new(config.clamped_static_size(), TILE_TIER_128, TILE_TIER_512)
                .with_reserve_texels(handover_reserve_texels(&config));
        let live_atlas = BuddyAtlas::new(config.clamped_live_size(), TILE_TIER_128, TILE_TIER_512);
        Self {
            config,
            static_atlas,
            live_atlas,
            lamps: BTreeMap::new(),
            alloc_failures: 0,
            resizes: 0,
            static_free_gen: 0,
        }
    }

    /// Monotonic generation bumped whenever static-atlas space is freed or reconfigured.
    /// Hosts can skip re-scanning un-admitted lamps while this counter has not advanced.
    pub fn static_free_gen(&self) -> u64 {
        self.static_free_gen
    }

    /// Applies an updated [`ShadowAtlasConfig`].
    ///
    /// Returns `true` if either atlas's power-of-two dimensions changed, in which case both
    /// atlases are rebuilt and all lamps are cleared for re-admission. When dimensions are
    /// unchanged, updates the static handover reserve and tier thresholds in place and returns
    /// `false`.
    pub fn reconfigure(&mut self, config: ShadowAtlasConfig) -> bool {
        let new_static = config.clamped_static_size();
        let new_live = config.clamped_live_size();
        let resized = new_static != self.static_atlas.atlas_size()
            || new_live != self.live_atlas.atlas_size();
        self.config = config;
        if resized {
            self.static_atlas = BuddyAtlas::new(new_static, TILE_TIER_128, TILE_TIER_512)
                .with_reserve_texels(handover_reserve_texels(&self.config));
            self.live_atlas = BuddyAtlas::new(new_live, TILE_TIER_128, TILE_TIER_512);
            self.lamps.clear();
            self.static_free_gen = self.static_free_gen.wrapping_add(1);
            true
        } else {
            let new_reserve = handover_reserve_texels(&self.config);
            if new_reserve != self.static_atlas.reserve_texels() {
                self.static_atlas.set_reserve_texels(new_reserve);
                self.static_free_gen = self.static_free_gen.wrapping_add(1);
            }
            false
        }
    }

    /// Admits a static lamp, allocating static tiles only for faces whose bit is set in
    /// `statics_mask` (empty-face omission).
    ///
    /// If `desired_tile` does not fit, steps down (`512 -> 256 -> 128`) before failing. Allocation
    /// is all-or-nothing (partial faces are rolled back on failure). If `key` was already tracked,
    /// its previous tiles are freed first.
    pub fn admit_static(
        &mut self,
        key: LampKey,
        statics_mask: u8,
        desired_tile: u16,
        now_s: f32,
        handover: bool,
    ) -> Option<u16> {
        self.remove(key);
        let mask = statics_mask & ALL_FACES_MASK;
        for &tier in step_down_tiers(desired_tile) {
            if let Some(static_rects) =
                try_alloc_mask(&mut self.static_atlas, mask, tier, handover)
            {
                self.lamps.insert(
                    key,
                    LampEntry {
                        tile_size: tier,
                        last_resize_s: now_s,
                        static_rects,
                        live_rects: [None; FACES],
                        retiring_static: [None; FACES],
                        moving: false,
                        handover,
                    },
                );
                return Some(tier);
            }
        }
        self.alloc_failures = self.alloc_failures.saturating_add(1);
        None
    }

    /// Admits a moving lamp (e.g. carried torch) directly into `live_atlas` across all 6 faces,
    /// consuming zero static-atlas texels.
    ///
    /// Steps down (`512 -> 256 -> 128`) if `desired_tile` does not fit, with all-or-nothing
    /// rollback on failure. If `key` was already tracked, its previous tiles are freed first.
    pub fn admit_moving(&mut self, key: LampKey, desired_tile: u16, now_s: f32) -> Option<u16> {
        self.remove(key);
        for &tier in step_down_tiers(desired_tile) {
            if let Some(live_rects) =
                try_alloc_mask(&mut self.live_atlas, ALL_FACES_MASK, tier, false)
            {
                self.lamps.insert(
                    key,
                    LampEntry {
                        tile_size: tier,
                        last_resize_s: now_s,
                        static_rects: [None; FACES],
                        live_rects,
                        retiring_static: [None; FACES],
                        moving: true,
                        handover: false,
                    },
                );
                return Some(tier);
            }
        }
        self.alloc_failures = self.alloc_failures.saturating_add(1);
        None
    }

    /// Frees every static and live tile owned by `key` and stops tracking it.
    pub fn remove(&mut self, key: LampKey) -> bool {
        let Some(entry) = self.lamps.remove(&key) else {
            return false;
        };
        let mut freed_static = false;
        for rect in entry.static_rects.into_iter().chain(entry.retiring_static).flatten() {
            self.static_atlas.free(rect);
            freed_static = true;
        }
        if freed_static {
            self.static_free_gen = self.static_free_gen.wrapping_add(1);
        }
        for rect in entry.live_rects.into_iter().flatten() {
            self.live_atlas.free(rect);
        }
        true
    }

    /// Reconciles a static lamp's static-atlas faces against `statics_mask`.
    ///
    /// Frees tiles for cleared faces first, then allocates tiles for newly populated faces.
    /// Returns the bitmask of newly allocated faces that need static capture.
    pub fn sync_static_faces(&mut self, key: LampKey, statics_mask: u8) -> u8 {
        let Some(entry) = self.lamps.get_mut(&key) else {
            return 0;
        };
        if entry.moving {
            return 0;
        }

        let mask = statics_mask & ALL_FACES_MASK;
        let mut freed_static = false;
        for (face, (slot, retiring)) in entry
            .static_rects
            .iter_mut()
            .zip(entry.retiring_static.iter_mut())
            .enumerate()
        {
            if (mask & (1 << face)) == 0 {
                for rect in [slot.take(), retiring.take()].into_iter().flatten() {
                    self.static_atlas.free(rect);
                    freed_static = true;
                }
            }
        }
        if freed_static {
            self.static_free_gen = self.static_free_gen.wrapping_add(1);
        }

        let mut newly_allocated = 0u8;
        for (face, slot) in entry.static_rects.iter_mut().enumerate() {
            if (mask & (1 << face)) == 0 || slot.is_some() {
                continue;
            }
            let alloc = if entry.handover {
                self.static_atlas.alloc_handover(entry.tile_size)
            } else {
                self.static_atlas.alloc(entry.tile_size)
            };
            if let Some(rect) = alloc {
                *slot = Some(rect);
                newly_allocated |= 1 << face;
            } else {
                self.alloc_failures = self.alloc_failures.saturating_add(1);
            }
        }
        newly_allocated
    }

    /// Evaluates [`pick_tile_size`] for `key` and resizes its existing faces if the tier changed.
    ///
    /// Allocates replacement tiles before freeing old ones so a full atlas leaves existing tiles
    /// intact. Returns `Some((new_tile_size, allocated_mask))` on a successful resize.
    pub fn maybe_resize(
        &mut self,
        key: LampKey,
        distance_m: f32,
        priority: f32,
        now_s: f32,
    ) -> Option<(u16, u8)> {
        let entry = self.lamps.get(&key)?;
        let since_resize_s = (now_s - entry.last_resize_s).max(0.0);
        let new_tier = pick_tile_size(
            distance_m,
            priority,
            Some(entry.tile_size),
            since_resize_s,
            &self.config,
        );
        if new_tier == entry.tile_size {
            return None;
        }

        let moving = entry.moving;
        let handover = entry.handover;
        let face_mask = if moving {
            rects_mask(&entry.live_rects)
        } else {
            rects_mask(&entry.static_rects)
        };

        if moving {
            let Some(new_live) = try_alloc_mask(&mut self.live_atlas, face_mask, new_tier, false)
            else {
                if let Some(entry) = self.lamps.get_mut(&key) {
                    entry.last_resize_s = now_s;
                }
                self.alloc_failures = self.alloc_failures.saturating_add(1);
                return None;
            };
            let entry = self.lamps.get_mut(&key)?;
            for old in entry.live_rects.into_iter().flatten() {
                self.live_atlas.free(old);
            }
            entry.live_rects = new_live;
            entry.tile_size = new_tier;
            entry.last_resize_s = now_s;
            self.resizes = self.resizes.saturating_add(1);
            Some((new_tier, face_mask))
        } else {
            let Some(new_static) =
                try_alloc_mask(&mut self.static_atlas, face_mask, new_tier, handover)
            else {
                if let Some(entry) = self.lamps.get_mut(&key) {
                    entry.last_resize_s = now_s;
                }
                self.alloc_failures = self.alloc_failures.saturating_add(1);
                return None;
            };
            let entry = self.lamps.get_mut(&key)?;
            // Keep each face's last drawn rect (retiring) until its replacement is drawn, so the
            // shader never samples an undrawn rect nor loses the face meanwhile (SH-33 flashing on
            // tier changes). A face resized again before that frees its never-drawn rect instead.
            let mut freed_static = false;
            for (old, retiring) in entry.static_rects.iter().zip(entry.retiring_static.iter_mut()) {
                let Some(old) = *old else { continue };
                if retiring.is_some() {
                    self.static_atlas.free(old);
                    freed_static = true;
                } else {
                    *retiring = Some(old);
                }
            }
            if freed_static {
                self.static_free_gen = self.static_free_gen.wrapping_add(1);
            }
            entry.static_rects = new_static;
            // Live (mover) tiles keep their size until the movers leave: re-allocating them here
            // left the face without mover depth until its next mover draw (SH-33).
            entry.tile_size = new_tier;
            entry.last_resize_s = now_s;
            self.resizes = self.resizes.saturating_add(1);
            Some((new_tier, face_mask))
        }
    }

    /// Reconciles a static lamp's active mover faces in `live_atlas` against `live_mask`.
    ///
    /// Frees live tiles for faces cleared in `live_mask`, keeps existing live tiles for faces that
    /// remain set (persisting across frames for Half/Far mover rate tiers), and allocates new live
    /// tiles (matching the lamp's `tile_size`, stepping down to `256`/`128` if tight) for newly
    /// active faces. Returns the bitmask of faces with a valid live tile (`<= live_mask`).
    ///
    /// For a moving lamp (`moving == true`), leaves all 6 live tiles untouched and returns
    /// `0b11_1111`.
    pub fn sync_live_faces(&mut self, key: LampKey, live_mask: u8) -> u8 {
        let Some(entry) = self.lamps.get_mut(&key) else {
            return 0;
        };
        if entry.moving {
            return ALL_FACES_MASK;
        }

        let mask = live_mask & ALL_FACES_MASK;
        for (face, slot) in entry.live_rects.iter_mut().enumerate() {
            if (mask & (1 << face)) == 0
                && let Some(rect) = slot.take()
            {
                self.live_atlas.free(rect);
            }
        }

        let mut valid_mask = 0u8;
        for (face, slot) in entry.live_rects.iter_mut().enumerate() {
            if (mask & (1 << face)) == 0 {
                continue;
            }
            if slot.is_some() {
                valid_mask |= 1 << face;
                continue;
            }
            let mut allocated = None;
            for &tier in step_down_tiers(entry.tile_size) {
                if let Some(rect) = self.live_atlas.alloc(tier) {
                    allocated = Some(rect);
                    break;
                }
            }
            if let Some(rect) = allocated {
                *slot = Some(rect);
                valid_mask |= 1 << face;
            } else {
                self.alloc_failures = self.alloc_failures.saturating_add(1);
            }
        }
        valid_mask
    }

    /// Returns a lamp's current nominal `tile_size` and per-face [`AtlasTileRef`] bindings.
    pub fn lamp_tiles(&self, key: LampKey) -> Option<(u16, [AtlasTileRef; 6])> {
        let entry = self.lamps.get(&key)?;
        let mut tiles = [AtlasTileRef::Empty; FACES];
        for (face, out) in tiles.iter_mut().enumerate() {
            *out = match (entry.static_rects[face], entry.live_rects[face]) {
                (static_rect, Some(live_rect)) => AtlasTileRef::Live {
                    static_rect,
                    live_rect,
                },
                (Some(static_rect), None) => AtlasTileRef::Static(static_rect),
                (None, None) => AtlasTileRef::Empty,
            };
        }
        Some((entry.tile_size, tiles))
    }

    /// Raw static-atlas rectangles per cube face for `key`.
    pub fn lamp_static_rects(&self, key: LampKey) -> Option<[Option<AtlasRect>; 6]> {
        self.lamps.get(&key).map(|entry| entry.static_rects)
    }

    /// Raw live-atlas rectangles per cube face for `key`.
    pub fn lamp_live_rects(&self, key: LampKey) -> Option<[Option<AtlasRect>; 6]> {
        self.lamps.get(&key).map(|entry| entry.live_rects)
    }

    /// Whether `key` is currently tracked by the manager.
    /// Pre-resize static rects still holding the lamp's last captured depth (sample these for
    /// faces whose new rect is not drawn yet).
    pub fn lamp_retiring_static(&self, key: LampKey) -> Option<[Option<AtlasRect>; FACES]> {
        self.lamps.get(&key).map(|e| e.retiring_static)
    }

    /// Frees the retiring rects of `faces` (their new rects were drawn). Returns the faces freed.
    pub fn release_retiring(&mut self, key: LampKey, faces: u8) -> u8 {
        let Some(entry) = self.lamps.get_mut(&key) else {
            return 0;
        };
        let mut freed = 0u8;
        for (face, retiring) in entry.retiring_static.iter_mut().enumerate() {
            if faces & (1 << face) != 0
                && let Some(rect) = retiring.take()
            {
                self.static_atlas.free(rect);
                freed |= 1 << face;
            }
        }
        if freed != 0 {
            self.static_free_gen = self.static_free_gen.wrapping_add(1);
        }
        freed
    }

    pub fn contains(&self, key: LampKey) -> bool {
        self.lamps.contains_key(&key)
    }

    /// Current policy configuration.
    pub fn config(&self) -> &ShadowAtlasConfig {
        &self.config
    }

    /// Read-only view of the underlying static buddy atlas.
    pub fn static_atlas(&self) -> &BuddyAtlas {
        &self.static_atlas
    }

    /// Read-only view of the underlying live buddy atlas.
    pub fn live_atlas(&self) -> &BuddyAtlas {
        &self.live_atlas
    }

    /// Snapshot of current occupancy, tile counts, and failure/resize counters.
    pub fn stats(&self) -> AtlasStats {
        let mut empty_faces = 0u32;
        for entry in self.lamps.values() {
            for face in 0..FACES {
                if entry.static_rects[face].is_none() && entry.live_rects[face].is_none() {
                    empty_faces += 1;
                }
            }
        }
        AtlasStats {
            lamps: u32::try_from(self.lamps.len()).unwrap_or(u32::MAX),
            static_tiles: u32::try_from(self.static_atlas.alloc_count()).unwrap_or(u32::MAX),
            live_tiles: u32::try_from(self.live_atlas.alloc_count()).unwrap_or(u32::MAX),
            empty_faces,
            static_used_texels: self.static_atlas.used_texels(),
            static_total_texels: self.static_atlas.total_texels(),
            live_used_texels: self.live_atlas.used_texels(),
            live_total_texels: self.live_atlas.total_texels(),
            alloc_failures: self.alloc_failures,
            resizes: self.resizes,
        }
    }
}

fn handover_reserve_texels(config: &ShadowAtlasConfig) -> u64 {
    u64::from(config.handover_spares) * (FACES as u64) * HANDOVER_TILE_PX * HANDOVER_TILE_PX
}

fn step_down_tiers(desired_tile: u16) -> &'static [u16] {
    if desired_tile >= TILE_TIER_512 {
        &[TILE_TIER_512, TILE_TIER_256, TILE_TIER_128]
    } else if desired_tile >= TILE_TIER_256 {
        &[TILE_TIER_256, TILE_TIER_128]
    } else {
        &[TILE_TIER_128]
    }
}

fn rects_mask(rects: &[Option<AtlasRect>; FACES]) -> u8 {
    let mut mask = 0u8;
    for (face, slot) in rects.iter().enumerate() {
        if slot.is_some() {
            mask |= 1 << face;
        }
    }
    mask
}

fn try_alloc_mask(
    atlas: &mut BuddyAtlas,
    mask: u8,
    tile_size: u16,
    handover: bool,
) -> Option<[Option<AtlasRect>; FACES]> {
    let mut rects = [None; FACES];
    for (face, slot) in rects.iter_mut().enumerate() {
        if (mask & (1 << face)) == 0 {
            continue;
        }
        let alloc = if handover {
            atlas.alloc_handover(tile_size)
        } else {
            atlas.alloc(tile_size)
        };
        if let Some(rect) = alloc {
            *slot = Some(rect);
        } else {
            for allocated in rects.into_iter().flatten() {
                atlas.free(allocated);
            }
            return None;
        }
    }
    Some(rects)
}
