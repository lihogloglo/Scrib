//! 2D shadow atlas allocator and policy (Bevy-free; Slice A2).
//!
//! - [`BuddyAtlas`] and [`AtlasRect`]: power-of-two 2D quadtree buddy allocator with automatic
//!   sibling coalescing (zero external fragmentation) and handover texel reservation.
//! - [`ShadowAtlasManager`]: manages separate static and live atlases per [`crate::lamp_cache::LampKey`],
//!   with empty-face omission, graceful tier step-down, moving-lamp live allocation, and per-face
//!   mover tile lifecycles.
//! - [`pick_tile_size`], [`guard_band_tan_half`], [`guard_band_inv_tan`], [`guard_band_fov_y`],
//!   and [`luminance_weight`]: pure tier-selection, guard-band projection, and perceptual-weight math.
//! - [`ShadowAtlasConfig`] and [`TUNABLES`]: runtime-configurable shadow atlas knobs (`shadow.lamp.atlas.*`
//!   and `shadow.lamp.contact_max`).

mod allocator;
mod policy;
mod tunables;

#[cfg(test)]
mod tests;

pub use allocator::{AtlasRect, BuddyAtlas};
pub use policy::{
    AtlasStats, AtlasTileRef, ShadowAtlasManager, guard_band_fov_y, guard_band_inv_tan,
    guard_band_tan_half, luminance_weight, pick_tile_size,
};
pub use tunables::{
    CONTACT_MAX, ENABLED, GUARD_TEXELS, HANDOVER_SPARES, LIVE_SIZE_PX, MAX_LAMPS,
    RESIZE_COOLDOWN_S, STATIC_SIZE_PX, ShadowAtlasConfig, TIER_256_M, TIER_512_M, TIER_HYSTERESIS,
    TUNABLES,
};
