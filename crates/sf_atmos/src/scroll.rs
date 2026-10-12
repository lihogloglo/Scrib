//! Wind scroll and floating-origin phase for the cloud textures.
//!
//! Shaders see origin-relative positions. A texture with tile `T` is read
//! at `rel / T + offset`, where `offset = frac((origin − wind_travel) / T)`
//! is computed here in f64. Each texture keeps its **own** offset, wrapped
//! in its own tile (Megalophobia's rule: one shared offset rescaled per
//! texture wraps them at different moments, and coverage jumps). The wind
//! travel itself is wrapped at 16 384 m, a multiple of every tile, so the
//! wrap is invisible for all of them.

use glam::Vec2;

use crate::settings::CloudLook;

/// The common period every tile divides (the floating origin's phase period).
pub const SCROLL_PERIOD: f64 = 16_384.0;

/// Accumulated wind travel (world X, Z), meters, wrapped at [`SCROLL_PERIOD`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CloudScroll {
    pub travel: [f64; 2],
}

/// Per-texture offsets, each in units of its own tile, in 0..1.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TileOffsets {
    pub weather: Vec2,
    pub shape: Vec2,
    pub detail: Vec2,
}

impl CloudScroll {
    /// Advances by `wind` (m/s) over `dt` seconds.
    pub fn advance(&mut self, wind: Vec2, dt: f64) {
        self.travel[0] = (self.travel[0] + f64::from(wind.x) * dt).rem_euclid(SCROLL_PERIOD);
        self.travel[1] = (self.travel[1] + f64::from(wind.y) * dt).rem_euclid(SCROLL_PERIOD);
    }

    /// Offsets for an origin at world `(x, z)` meters (the absolute position
    /// that origin-relative coordinates are measured from).
    pub fn offsets(&self, origin_xz: [f64; 2], look: &CloudLook) -> TileOffsets {
        let off = |tile: f32| {
            let t = f64::from(tile.abs().max(1e-3));
            #[allow(clippy::cast_possible_truncation)]
            let f = |o: f64, w: f64| ((o - w) / t).rem_euclid(1.0) as f32;
            Vec2::new(f(origin_xz[0], self.travel[0]), f(origin_xz[1], self.travel[1]))
        };
        TileOffsets { weather: off(look.weather_tile), shape: off(look.shape_tile), detail: off(look.detail_tile) }
    }
}
