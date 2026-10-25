//! LOD switch distances from cooked errors (bake.md, "Runtime distance").
//!
//! A level whose geometric error is `e` meters may be shown once it covers
//! less than `τ` pixels on screen: `d = e · H / (2 · tan(fov/2) · τ)`.

/// Screen parameters that turn errors into distances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LodScreen {
    /// Vertical resolution in pixels.
    pub height_px: f32,
    /// Vertical field of view in radians.
    pub fov_y: f32,
    /// Allowed error on screen, in pixels.
    pub pixel_error: f32,
    /// Global multiplier (quality presets, user setting).
    pub bias: f32,
    /// Crossfade band as a fraction of each switch distance.
    pub crossfade: f32,
    /// Size-based draw distance (industry-audit.md J1): an object is no
    /// longer drawn once its bounding sphere's diameter projects under this
    /// many reference pixels (an angle, like `pixel_error`: resolution and
    /// upscaling don't change what is drawn). 0 turns size culling off.
    pub min_pixels: f32,
    /// No object is size-culled closer than this, meters (not scaled by `bias`).
    pub min_draw_distance: f32,
}

impl Default for LodScreen {
    /// The reference: 1440p, 70° vertical FOV, 1 px; size culling at 2 px
    /// with a 24 m floor (the High preset).
    fn default() -> Self {
        Self { height_px: 1440.0, fov_y: 70f32.to_radians(), pixel_error: 1.0, bias: 1.0, crossfade: 0.1, min_pixels: 2.0, min_draw_distance: 24.0 }
    }
}

impl LodScreen {
    /// Distance from which an error of `error` meters is acceptable.
    pub fn distance(&self, error: f32) -> f32 {
        error * self.focal_px() / self.pixel_error * self.bias
    }

    /// Pixels per meter of size at one meter of distance: `H / (2 tan(fov/2))`.
    pub fn focal_px(&self) -> f32 {
        self.height_px / (2.0 * (self.fov_y * 0.5).tan())
    }

    /// Size-based draw distance of an object whose bounds radius is `radius`
    /// meters: where its diameter shrinks to [`Self::min_pixels`], times
    /// `bias`, never under [`Self::min_draw_distance`]. Infinite when size culling is off. Callers cap it with the
    /// ring or far-field handoff that ends the object anyway.
    pub fn draw_distance(&self, radius: f32) -> f32 {
        if self.min_pixels <= 0.0 {
            return f32::INFINITY;
        }
        (self.focal_px() * 2.0 * radius / self.min_pixels * self.bias).max(self.min_draw_distance)
    }
}

/// One level's visible range: `[start, end)` meters, with crossfade margins.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LodRange {
    pub start: f32,
    pub end: f32,
    /// Width of the fade band centered on each boundary (0 at the ends).
    pub start_margin: f32,
    pub end_margin: f32,
}

// Visible ranges for every level, from each level's error (LOD0 first, error 0). The last level
/// extends to `max_distance` (the streaming ring that holds the object).
/// Levels whose range collapses (an error too close to the previous one)
/// are dropped by giving them an empty range.
pub fn lod_ranges(errors: &[f32], screen: &LodScreen, max_distance: f32) -> Vec<LodRange> {
    let switch: Vec<f32> = errors.iter().skip(1).map(|&e| screen.distance(e).min(max_distance)).collect();
    let mut out = Vec::with_capacity(errors.len());
    let mut start = 0.0f32;
    for i in 0..errors.len() {
        let end = switch.get(i).copied().unwrap_or(max_distance).max(start);
        let margin = |d: f32| if d > 0.0 && d < max_distance { d * screen.crossfade } else { 0.0 };
        out.push(LodRange { start, end, start_margin: margin(start), end_margin: margin(end) });
        start = end;
    }
    out
}
