//! Groundcover species: what one kind of grass, flower or small plant is and where it may grow
//! (spec "Data model": Skyrim GRAS fields, UE grass varieties, Horizon footprint/density). Metres and degrees
//! only; asset keys are opaque strings the renderer resolves.

use serde::{Deserialize, Serialize};

/// An inclusive `[min, max]` range.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub min: f32,
    pub max: f32,
}

impl Span {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    /// A single value.
    pub const fn exactly(v: f32) -> Self {
        Self { min: v, max: v }
    }

    #[inline]
    pub fn contains(self, v: f32) -> bool {
        v >= self.min && v <= self.max
    }

    /// `min` at `t = 0`, `max` at `t = 1`.
    #[inline]
    pub fn lerp(self, t: f32) -> f32 {
        self.min + (self.max - self.min) * t
    }

    /// Both ends finite and `min <= max`.
    pub fn is_valid(self) -> bool {
        self.min.is_finite() && self.max.is_finite() && self.min <= self.max
    }
}

/// How the renderer draws a species. Keys are opaque to this crate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderKind {
    /// Procedural blades (a blade style key, e.g. a `bevy_grass` style file).
    Blade { style: String },
    /// An instanced model (its asset key), with optional coarser LOD model keys, finest first.
    Mesh {
        model: String,
        #[serde(default)]
        lods: Vec<String>,
    },
}

/// Whether instances cast shadows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShadowClass {
    /// Never.
    #[default]
    None,
    /// Inside the near shadow range (a renderer tunable).
    Near,
}

/// The species' draw-distance class; the distances are renderer tunables per preset. Also the class an
/// explicit-placement region suppresses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RangeClass {
    #[default]
    Near,
    Mid,
}

/// One groundcover species (a record; the adapter or a mod fills it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Species {
    /// Record id, unique in a [`Groundcover`](crate::Groundcover). Hashed into every candidate seed, so renaming a
    /// species moves its instances.
    pub id: String,
    pub render: RenderKind,
    /// Spacing between instances (Horizon footprint), metres. The scatter grid's cell size: at most one instance per
    /// footprint cell, so density saturates at `1 / footprint_m²`.
    pub footprint_m: f32,
    /// Instances per m² at weight 1.
    pub density: f32,
    /// Allowed ground slope, degrees from horizontal; `None` = any.
    #[serde(default)]
    pub slope_deg: Option<Span>,
    /// Allowed ground height, metres; `None` = any.
    #[serde(default)]
    pub altitude_m: Option<Span>,
    /// Allowed signed distance to the water surface, metres (negative = submerged); `None` = any.
    #[serde(default)]
    pub water_m: Option<Span>,
    /// Uniform scale per instance.
    pub scale: Span,
    /// Yaw per instance, degrees.
    pub yaw_deg: Span,
    /// Tint range (linear RGB multipliers): the instance tag carries an 8-bit position between the two.
    pub tint: [[f32; 3]; 2],
    /// How far the instance's up axis follows the ground normal: 0 = world up, 1 = the normal.
    pub align_to_surface: Span,
    /// Wind response factor (renderer scales it).
    pub wind: f32,
    /// Trample response factor (renderer scales it).
    pub trample: f32,
    /// Height of the model's origin above the ground, metres at scale 1 (scaled with the instance): models whose
    /// geometry hangs below their origin (roots, a sunk base) stand on the ground instead of half buried. UE
    /// foliage types' Z offset. Default 0.
    #[serde(default)]
    pub z_offset_m: f32,
    #[serde(default)]
    pub shadow: ShadowClass,
    #[serde(default)]
    pub range: RangeClass,
}
