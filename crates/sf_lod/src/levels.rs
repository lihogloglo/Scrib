//! Handoff distances, error budgets and the lead-band visibility ranges
//! (far-field.md, "Lead bands").
//!
//! Every representation is drawn through a distance range with dithered
//! fade bands (Bevy's `VisibilityRange`, mirrored here Bevy-free as
//! [`FadeRange`]). A node starts fading in early enough that it is fully
//! drawn before any of its children starts fading out, so no camera
//! position ever sees a hole; [`coverage`] checks that property.

use std::ops::Range;

use sf_model::LodScreen;

use crate::node::{FIRST_LEVEL, LAST_LEVEL};

/// A distance range with fade bands, measured from an entity's origin.
/// Same semantics as Bevy's `VisibilityRange`: fully drawn between
/// `start.end` and `end.start`, fading in over `start`, out over `end`.
#[derive(Clone, Debug, PartialEq)]
pub struct FadeRange {
    pub start: Range<f32>,
    pub end: Range<f32>,
}

/// "Never ends": far enough for any worldspace, finite so GPU math stays
/// well defined.
pub const FOREVER: f32 = 1.0e9;

impl FadeRange {
    pub const ALWAYS: Self = Self { start: 0.0..0.0, end: FOREVER..FOREVER };

    /// The drawn fraction at a distance: 1 fully drawn, 0 culled, between
    /// while dithering. Bevy quantizes the same ramp to 16 levels.
    pub fn fraction(&self, d: f32) -> f32 {
        if d < self.start.start || d >= self.end.end {
            return 0.0;
        }
        let ramp = |r: &Range<f32>| if r.end > r.start { (d - r.start) / (r.end - r.start) } else { 1.0 };
        if d < self.start.end {
            ramp(&self.start).clamp(0.0, 1.0)
        } else if d >= self.end.start {
            (1.0 - ramp(&self.end)).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    /// Fully drawn (no dither) at `d`.
    pub fn full(&self, d: f32) -> bool {
        d >= self.start.end && d < self.end.start
    }
}

/// Handoff distances and the screen they are tuned for. Part of the bake
/// key (budgets come from it); runtime presets scale it with `bias`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FarLevels {
    /// `d1`, `d2`, `d3`: L0 → L1, L1 → L2, L2 → L3, in meters.
    pub handoff: [f32; 3],
    /// Fade band width as a fraction of each handoff.
    pub band: f32,
    /// Pixel-error screen (bake.md's reference: 1440p, 70°, 1 px).
    pub screen: LodScreen,
    /// Objects whose bounds *diameter* projects under this many pixels where
    /// a level is fully drawn leave that level ([`Self::min_radius`]).
    pub min_pixels: f32,
}

impl Default for FarLevels {
    /// Starting values, to be calibrated by fly-throughs (far-field.md).
    fn default() -> Self {
        Self { handoff: [400.0, 2000.0, 8000.0], band: 0.1, screen: LodScreen::default(), min_pixels: 2.0 }
    }
}

impl FarLevels {
    /// Handoff distance into `level` (1..=3), scaled by the screen bias.
    pub fn handoff(&self, level: u8) -> f32 {
        self.handoff[usize::from(level.clamp(FIRST_LEVEL, LAST_LEVEL) - 1)] * self.screen.bias
    }

    /// Fade band width at the handoff into `level`.
    pub fn band_width(&self, level: u8) -> f32 {
        self.handoff(level) * self.band
    }

    /// Where a node of `level` whose farthest child origin is `lead` meters
    /// from its center must be fully drawn: before any child starts fading
    /// out (`d_child >= handoff` implies `d_node >= handoff - lead`).
    pub fn full_from(&self, level: u8, lead: f32) -> f32 {
        (self.handoff(level) - lead).max(0.0)
    }

    /// The distance error budgets are computed at: [`Self::full_from`], but
    /// never closer than a quarter of the handoff (a pathological lead would
    /// otherwise ask for a zero budget; coverage is unaffected, only the
    /// error shown in the overlap band grows).
    fn budget_distance(&self, level: u8, lead: f32) -> f32 {
        self.full_from(level, lead).max(self.handoff(level) * 0.25)
    }

    /// Meters per pixel at distance `d` on the reference screen.
    pub fn pixel_size(&self, d: f32) -> f32 {
        d * 2.0 * (self.screen.fov_y * 0.5).tan() / self.screen.height_px
    }

    /// The geometric error a node of `level` may have: one pixel (times the
    /// screen's pixel error) where it is first fully drawn.
    pub fn error_budget(&self, level: u8, lead: f32) -> f32 {
        self.pixel_size(self.budget_distance(level, lead)) * self.screen.pixel_error
    }

    /// Smallest bounds radius an object keeps in `level`.
    pub fn min_radius(&self, level: u8, lead: f32) -> f32 {
        self.pixel_size(self.budget_distance(level, lead)) * self.min_pixels * 0.5
    }

    /// The range of an object (L0) covered by the far field: its LOD chain
    /// ends at `d1`.
    pub fn object_end(&self) -> f32 {
        self.handoff(1)
    }

    /// The range of a node of `level` with lead offset `lead`.
    /// `forced`: some child it covers isn't resident, so it is drawn fully
    /// from distance 0 (the no-hole fallback).
    pub fn node_range(&self, level: u8, lead: f32, forced: bool) -> FadeRange {
        let start = if forced {
            0.0..0.0
        } else {
            let full = self.full_from(level, lead);
            (full - self.band_width(level)).max(0.0)..full
        };
        let end = if level < LAST_LEVEL {
            let d = self.handoff(level + 1);
            d..d + self.band_width(level + 1)
        } else {
            FOREVER..FOREVER
        };
        FadeRange { start, end }
    }

    /// Whether a node is in or below its fade-in band at distance `d` from
    /// its center: the region where its children must be drawn.
    pub fn needs_children(&self, level: u8, lead: f32, d: f32) -> bool {
        let r = self.node_range(level, lead, false);
        d < r.start.end
    }

    /// The no-hole fallback (3D Tiles / UE: keep the parent until its
    /// children arrive): a node is forced fully drawn while the camera is
    /// where its children should be drawn and any of them isn't resident.
    pub fn forced(&self, level: u8, lead: f32, d: f32, children_resident: bool) -> bool {
        !children_resident && self.needs_children(level, lead, d)
    }
}

/// An element of a coverage check: something drawn at a position with a
/// range (one link in an object's chain of representations).
#[derive(Clone, Debug)]
pub struct Drawn {
    /// Origin the range is measured from.
    pub origin: glam::DVec3,
    pub range: FadeRange,
    /// Whether the entity exists (resident).
    pub resident: bool,
}

/// Whether, seen from `camera`, at least one representation in `chain`
/// (an object, its L1, L2, L3 nodes) is fully drawn: no hole.
pub fn coverage(camera: glam::DVec3, chain: &[Drawn]) -> bool {
    chain.iter().any(|e| {
        #[allow(clippy::cast_possible_truncation)]
        let d = e.origin.distance(camera) as f32;
        e.resident && e.range.full(d)
    })
}
