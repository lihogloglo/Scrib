//! Wind scroll, shape evolution and floating-origin phase for the cloud
//! textures.
//!
//! Shaders see origin-relative positions. A texture with tile `T` is read
//! at `rel / T + offset`, where `offset = frac((origin − wind_travel) / T)`
//! is computed here in f64. Each texture keeps its **own** offset, wrapped
//! in its own tile (Megalophobia's rule: one shared offset rescaled per
//! texture wraps them at different moments, and coverage jumps). The wind
//! travel itself is wrapped at 16 384 m, a multiple of every tile, so the
//! wrap is invisible for all of them.
//!
//! **Evolution** (clouds form, grow and dissolve instead of sliding by as
//! frozen stamps): the cloud field is two octaves of the weather map that
//! drift at slightly different velocities (`evolve` of the wind apart), as in
//! advected-noise weather maps (Schneider 2017), and the billows rise
//! through the shape volume (its vertical axis tiles, so a `rise` phase
//! scrolls it upward).

use glam::Vec2;

use crate::settings::CloudLook;

/// The common period every tile divides (the floating origin's phase period).
pub const SCROLL_PERIOD: f64 = 16_384.0;

/// The high layer's speed relative to the low layer's wind (winds grow
/// with altitude; the parallax between the layers reads as depth).
pub const CIRRUS_WIND: f32 = 2.0;

/// How fast the billows rise through the shape volume, in shape tiles per
/// meter of relative (evolution) drift.
const RISE_RATE: f64 = 0.5;

/// Accumulated travel (world X, Z), meters, each wrapped at [`SCROLL_PERIOD`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CloudScroll {
    /// The cloud layer (weather map, shape, detail).
    pub travel: [f64; 2],
    /// The weather map's second, slower octave (evolution).
    pub evolve: [f64; 2],
    /// The high (cirrus) layer.
    pub cirrus: [f64; 2],
    /// Upward phase of the shape volume, in its tiles, 0..1.
    pub rise: f64,
}

/// Per-texture offsets, each in units of its own tile, in 0..1.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TileOffsets {
    pub weather: Vec2,
    /// The weather map's second octave, sampled transposed (`uv.yx`), so
    /// this offset is already (z, x).
    pub weather2: Vec2,
    pub shape: Vec2,
    pub detail: Vec2,
    pub cirrus: Vec2,
    /// Vertical phase of the shape volume, 0..1.
    pub rise: f32,
}

fn wrap(v: f64) -> f64 {
    v.rem_euclid(SCROLL_PERIOD)
}

impl CloudScroll {
    /// Advances by `wind` (m/s) over `dt` seconds. `evolve` is
    /// [`CloudLook::evolve`]: the second weather octave drifts that fraction
    /// of the wind slower and a little sideways, so the field re-forms at
    /// about `evolve × |wind|`; the billows rise with it.
    pub fn advance(&mut self, wind: Vec2, evolve: f32, dt: f64, shape_tile: f32) {
        let (wx, wz) = (f64::from(wind.x), f64::from(wind.y));
        let e = f64::from(evolve.clamp(0.0, 1.0));
        self.travel = [wrap(self.travel[0] + wx * dt), wrap(self.travel[1] + wz * dt)];
        // (1 − e)·wind + e/2·perp(wind): relative drift ≈ 1.1·e·|wind|.
        let (ex, ez) = ((1.0 - e) * wx - 0.5 * e * wz, (1.0 - e) * wz + 0.5 * e * wx);
        self.evolve = [wrap(self.evolve[0] + ex * dt), wrap(self.evolve[1] + ez * dt)];
        let c = f64::from(CIRRUS_WIND);
        self.cirrus = [wrap(self.cirrus[0] + c * wx * dt), wrap(self.cirrus[1] + c * wz * dt)];
        let speed = wx.hypot(wz);
        self.rise = (self.rise + RISE_RATE * e * speed * dt / f64::from(shape_tile.abs().max(1.0))).rem_euclid(1.0);
    }

    /// Offsets for an origin at world `(x, z)` meters (the absolute position
    /// that origin-relative coordinates are measured from).
    pub fn offsets(&self, origin_xz: [f64; 2], look: &CloudLook) -> TileOffsets {
        let off = |tile: f32, travel: [f64; 2]| {
            let t = f64::from(tile.abs().max(1e-3));
            #[allow(clippy::cast_possible_truncation)]
            let f = |o: f64, w: f64| ((o - w) / t).rem_euclid(1.0) as f32;
            Vec2::new(f(origin_xz[0], travel[0]), f(origin_xz[1], travel[1]))
        };
        #[allow(clippy::cast_possible_truncation)]
        TileOffsets {
            weather: off(look.weather_tile, self.travel),
            weather2: { let o = off(look.weather_tile, self.evolve); Vec2::new(o.y, o.x) },
            shape: off(look.shape_tile, self.travel),
            detail: off(look.detail_tile, self.travel),
            cirrus: off(look.cirrus_tile, self.cirrus),
            rise: self.rise as f32,
        }
    }
}
