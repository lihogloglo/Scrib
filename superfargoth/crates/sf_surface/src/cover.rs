//! Snow and ash cover looks (surface-weather.md "Records", slice W5a): the [`SurfaceLayer`]
//! record kind ([`LAYER_KIND`], [`register_kinds`]) and the adapter's pick of one active layer
//! per channel per worldspace ([`ActiveLayers`]).
//!
//! A layer names its textures by VFS path strings (as `sf_terrain::bake::TerrainLayer` does);
//! the host reads and decodes them, `sf_surface_bevy` packs them into the cover layer array.
//! Adapters generate the records, mods patch them field by field (everything is a mod).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError, RecordId};

use crate::Worldspace;

/// Record kind of [`SurfaceLayer`].
pub const LAYER_KIND: &str = "surface.layer";

/// Registers the surface record kinds (serde, so record files and patches can write them).
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<SurfaceLayer>(LAYER_KIND)
}

/// The cover channels a [`SurfaceLayer`] can dress.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverChannel {
    Snow,
    Ash,
}

impl CoverChannel {
    /// In layer array order (slices `2 · index` and `2 · index + 1`).
    pub const ALL: [CoverChannel; 2] = [CoverChannel::Snow, CoverChannel::Ash];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn name(self) -> &'static str {
        match self {
            CoverChannel::Snow => "snow",
            CoverChannel::Ash => "ash",
        }
    }

    /// Visual depth of a full cover when the layer gives none (W6 trails): snow 0.15 m, ash 0.03 m.
    pub fn default_depth_m(self) -> f32 {
        match self {
            CoverChannel::Snow => 0.15,
            CoverChannel::Ash => 0.03,
        }
    }
}

/// A snow or ash look (surface-weather.md "Records"). Paths are VFS path strings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SurfaceLayer {
    pub channel: CoverChannel,
    /// sRGB albedo with height in alpha (255 everywhere = flat; the height blend then reduces to
    /// a coverage threshold).
    pub albedo_height: String,
    /// Tangent-space normal map, +Y up; flat when absent.
    #[serde(default)]
    pub normal: Option<String>,
    /// Ambient occlusion, roughness, metallic (metallic unused); `roughness` and AO 1 when absent.
    #[serde(default)]
    pub arm: Option<String>,
    /// Meters per texture repeat (world-aligned, top-down).
    #[serde(default = "default_tiling_m")]
    pub tiling_m: f32,
    /// Visual depth of a full cover in meters (W6); `None` = the channel's default.
    #[serde(default)]
    pub max_depth_m: Option<f32>,
    /// Perceptual roughness where no `arm` map gives one.
    #[serde(default = "default_roughness")]
    pub roughness: f32,
    /// Glint strength, 0..1 (fresh snow).
    #[serde(default)]
    pub sparkle: f32,
    /// Height-blend contrast against the base material (1 = the plain height compare; higher
    /// sharpens the edge).
    #[serde(default = "default_contrast")]
    pub contrast: f32,
}

fn default_tiling_m() -> f32 {
    4.0
}

fn default_roughness() -> f32 {
    0.8
}

fn default_contrast() -> f32 {
    1.0
}

impl SurfaceLayer {
    /// A layer with the defaults for everything but its channel and albedo.
    pub fn new(channel: CoverChannel, albedo_height: impl Into<String>) -> Self {
        Self {
            channel,
            albedo_height: albedo_height.into(),
            normal: None,
            arm: None,
            tiling_m: default_tiling_m(),
            max_depth_m: None,
            roughness: default_roughness(),
            sparkle: 0.0,
            contrast: default_contrast(),
        }
    }

    /// [`Self::max_depth_m`] or the channel's default.
    pub fn depth_m(&self) -> f32 {
        self.max_depth_m.unwrap_or_else(|| self.channel.default_depth_m())
    }
}

/// The adapter's pick: one [`SurfaceLayer`] record per channel per worldspace, `default` for
/// worldspaces it doesn't name. A pick whose record is missing, or dresses the other channel,
/// resolves to none (no cover drawn on that channel).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActiveLayers {
    pub default: [Option<RecordId>; 2],
    pub worldspaces: HashMap<Worldspace, [Option<RecordId>; 2]>,
}

impl ActiveLayers {
    /// The picked record id for `channel` in `ws`.
    pub fn pick(&self, ws: &Worldspace, channel: CoverChannel) -> Option<&RecordId> {
        self.worldspaces.get(ws).unwrap_or(&self.default)[channel.index()].as_ref()
    }

    /// The picked layer records of `ws`, in [`CoverChannel::ALL`] order.
    pub fn resolve<'a>(&self, db: &'a RecordDb, ws: &Worldspace) -> [Option<&'a SurfaceLayer>; 2] {
        CoverChannel::ALL.map(|channel| {
            let layer = db.get::<SurfaceLayer>(self.pick(ws, channel)?)?;
            (layer.channel == channel).then_some(layer)
        })
    }
}
