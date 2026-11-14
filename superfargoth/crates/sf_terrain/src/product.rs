//! Bake products (terrain.md, bake.md "M2 steps") and their byte formats.
//!
//! Every product is postcard behind a 4-byte magic and a format version.
//! Heights use a gradient predictor (`left + up − up_left`) with zigzag
//! varints, so smooth terrain stores about one byte per sample before the
//! store's LZ4 pass.

use serde::{Deserialize, Serialize};

use crate::layers::{LayerId, LayerMap, TintMap};
use crate::pyramid::{BlockBounds, EncodedHeights};
use crate::tile::{HeightTile, LAYER_SAMPLES, TILE_SAMPLES, TileCoord};

/// Product decode errors.
#[derive(Debug, thiserror::Error)]
pub enum ProductError {
    #[error("not a {0} product (bad magic)")]
    Magic(&'static str),
    #[error("{what} product format {got}, expected {expected}")]
    Format { what: &'static str, got: u32, expected: u32 },
    #[error("corrupt product: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("corrupt product: {0}")]
    Invalid(&'static str),
}

/// The global id of a `TerrainLayer` record: a stable 32-bit hash of its
/// id (FNV-1a), so tiles depend only on their own sources, never on which
/// other layers exist. The layer table maps ids to texture array slices
/// and rejects collisions.
pub fn layer_id(record: &str) -> LayerId {
    let mut h: u32 = 0x811c_9dc5;
    for b in record.bytes() {
        h ^= u32::from(b);
        h = h.wrapping_mul(0x0100_0193);
    }
    LayerId(h)
}

fn encode<T: Serialize>(magic: [u8; 4], format: u32, value: &T) -> Vec<u8> {
    let mut out = magic.to_vec();
    out.extend_from_slice(&format.to_le_bytes());
    postcard::to_extend(value, out).expect("serializing to a Vec can't fail")
}

fn decode<'a, T: Deserialize<'a>>(
    what: &'static str,
    magic: [u8; 4],
    format: u32,
    bytes: &'a [u8],
) -> Result<T, ProductError> {
    if bytes.len() < 8 || &bytes[..4] != magic.as_slice() {
        return Err(ProductError::Magic(what));
    }
    let got = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
    if got != format {
        return Err(ProductError::Format { what, got, expected: format });
    }
    Ok(postcard::from_bytes(&bytes[8..])?)
}

/// One canonical tile (`terrain.tile` at level 0, `terrain.pyramid` above).
#[derive(Clone, Debug, PartialEq)]
pub struct TileProduct {
    pub coord: TileCoord,
    pub heights: EncodedHeights,
    /// Per-block bounds of this tile *and every finer level under it*
    /// (merged up the pyramid), so CDLOD can test a node against any
    /// resident level and stay conservative.
    pub bounds: BlockBounds,
    pub layers: Option<LayerMap>,
    pub tint: Option<TintMap>,
}

#[derive(Serialize, Deserialize)]
enum HeightsWire {
    U16 { base: f32, step: f32, residuals: Vec<u32>, border: Vec<f32> },
    F32(Vec<f32>),
}

#[derive(Serialize, Deserialize)]
struct TileWire {
    coord: TileCoord,
    heights: HeightsWire,
    bounds: BlockBounds,
    layers: Option<LayerMap>,
    tint: Option<TintMap>,
}

fn zigzag(v: i32) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let z = ((v << 1) ^ (v >> 31)) as u32;
    z
}

fn unzigzag(z: u32) -> i32 {
    #[allow(clippy::cast_possible_wrap)]
    let v = ((z >> 1) as i32) ^ -((z & 1) as i32);
    v
}

/// Gradient prediction of sample `k` of a 513² grid from decoded values.
fn predict(v: &[i32], k: usize) -> i32 {
    let (i, j) = (k % TILE_SAMPLES, k / TILE_SAMPLES);
    match (i, j) {
        (0, 0) => 0,
        (_, 0) => v[k - 1],
        (0, _) => v[k - TILE_SAMPLES],
        _ => v[k - 1] + v[k - TILE_SAMPLES] - v[k - TILE_SAMPLES - 1],
    }
}

impl TileProduct {
    const MAGIC: [u8; 4] = *b"SFTT";
    pub const FORMAT: u32 = 1;

    pub fn to_bytes(&self) -> Vec<u8> {
        let heights = match &self.heights {
            EncodedHeights::F32(h) => HeightsWire::F32(h.clone()),
            EncodedHeights::U16 { base, step, values, border } => {
                let v: Vec<i32> = values.iter().map(|&q| i32::from(q)).collect();
                let residuals = (0..v.len()).map(|k| zigzag(v[k] - predict(&v, k))).collect();
                HeightsWire::U16 { base: *base, step: *step, residuals, border: border.clone() }
            }
        };
        let wire = TileWire {
            coord: self.coord,
            heights,
            bounds: self.bounds.clone(),
            layers: self.layers.clone(),
            tint: self.tint.clone(),
        };
        encode(Self::MAGIC, Self::FORMAT, &wire)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProductError> {
        let wire: TileWire = decode("terrain tile", Self::MAGIC, Self::FORMAT, bytes)?;
        let heights = match wire.heights {
            HeightsWire::F32(h) => EncodedHeights::F32(h),
            HeightsWire::U16 { base, step, residuals, border } => {
                if residuals.len() != TILE_SAMPLES * TILE_SAMPLES {
                    return Err(ProductError::Invalid("height count"));
                }
                let mut v = vec![0i32; residuals.len()];
                for k in 0..v.len() {
                    v[k] = predict(&v, k) + unzigzag(residuals[k]);
                }
                let values = v.into_iter().map(|q| u16::try_from(q).map_err(|_| ProductError::Invalid("height range")));
                EncodedHeights::U16 { base, step, values: values.collect::<Result<_, _>>()?, border }
            }
        };
        Ok(Self { coord: wire.coord, heights, bounds: wire.bounds, layers: wire.layers, tint: wire.tint })
    }

    /// Decoded heights.
    pub fn height_tile(&self) -> HeightTile {
        self.heights.decode(self.coord)
    }
}

/// Macro albedo of one tile (`terrain.macro`): 257² sRGB RGBA8 samples at
/// the layer map's spacing; the far-terrain color in one fetch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacroProduct {
    pub coord: TileCoord,
    pub rgba: Vec<[u8; 4]>,
}

impl MacroProduct {
    const MAGIC: [u8; 4] = *b"SFTM";
    pub const FORMAT: u32 = 1;

    pub fn to_bytes(&self) -> Vec<u8> {
        encode(Self::MAGIC, Self::FORMAT, self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProductError> {
        let m: Self = decode("terrain macro", Self::MAGIC, Self::FORMAT, bytes)?;
        if m.rgba.len() != LAYER_SAMPLES * LAYER_SAMPLES {
            return Err(ProductError::Invalid("macro sample count"));
        }
        Ok(m)
    }
}

/// One layer's runtime parameters (`terrain.layer_table`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerInfo {
    /// The `TerrainLayer` record id.
    pub record: String,
    pub id: LayerId,
    /// Texture repeat in meters, rounded so that 1024 m is a whole number
    /// of repeats (world-coordinates.md: patterns use the period-1024 phase).
    pub tiling_m: f32,
    pub triplanar: bool,
    pub displacement_m: f32,
    /// Mean albedo, linear RGB, and mean height (alpha).
    pub mean_albedo: [f32; 3],
    pub mean_height: f32,
    /// Larger edge of the albedo source, texels: `terrain.layers` never upsamples the arrays
    /// past the largest one (T6).
    pub source_edge: u32,
}

/// Every terrain layer, in texture array order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LayerTable {
    pub layers: Vec<LayerInfo>,
    /// The layer where no source names one (and outside the sources).
    pub fill_layer: Option<LayerId>,
    /// The height where no source has samples, meters.
    pub fill_height: f32,
}

impl LayerTable {
    const MAGIC: [u8; 4] = *b"SFTL";
    pub const FORMAT: u32 = 2;

    pub fn to_bytes(&self) -> Vec<u8> {
        encode(Self::MAGIC, Self::FORMAT, self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProductError> {
        decode("terrain layer table", Self::MAGIC, Self::FORMAT, bytes)
    }

    /// Array slice of a layer id.
    pub fn slice_of(&self, id: LayerId) -> Option<usize> {
        self.layers.iter().position(|l| l.id == id)
    }
}

/// The layers' textures as block-compressed array data (`terrain.layers`, T6): three arrays,
/// every slice `size`² with a full mip chain, layer-major (slice 0's levels, then slice 1's, …),
/// the layout GPU uploads take. All three use 16-byte 4×4 blocks (levels under 4² take one
/// block per row and column, wgpu's physical size), so they share every offset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerArrays {
    pub table: LayerTable,
    pub size: u32,
    pub mips: u32,
    /// BC7 sRGB: albedo, linear height in alpha.
    pub albedo_height: Vec<u8>,
    /// BC5: tangent-space normal X and Y (+Y = +V, texture rows downward: a canonical +Y-up
    /// map has G complemented at bake; Z reconstructed in the shader).
    pub normal: Vec<u8>,
    /// BC5: ambient occlusion (R), perceptual roughness (G).
    pub ao_roughness: Vec<u8>,
}

/// [`LayerArrays`] borrowing its data from the product bytes (same wire form): a capped load
/// copies only the levels it keeps.
#[derive(Serialize, Deserialize)]
struct LayerArraysRef<'a> {
    table: LayerTable,
    size: u32,
    mips: u32,
    albedo_height: &'a [u8],
    normal: &'a [u8],
    ao_roughness: &'a [u8],
}

impl LayerArrays {
    const MAGIC: [u8; 4] = *b"SFTA";
    pub const FORMAT: u32 = 3;
    /// Bytes of one 4×4 block in every array (BC7 and BC5 alike).
    pub const BLOCK_BYTES: usize = 16;

    pub fn to_bytes(&self) -> Vec<u8> {
        let view = LayerArraysRef {
            table: self.table.clone(),
            size: self.size,
            mips: self.mips,
            albedo_height: &self.albedo_height,
            normal: &self.normal,
            ao_roughness: &self.ao_roughness,
        };
        encode(Self::MAGIC, Self::FORMAT, &view)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProductError> {
        Self::from_bytes_capped(bytes, &LayerCap::default())
    }

    /// Decodes the arrays keeping only the levels `cap` allows ([`LayerCap::skip`]): the top
    /// levels of every slice are never copied.
    pub fn from_bytes_capped(bytes: &[u8], cap: &LayerCap) -> Result<Self, ProductError> {
        let v: LayerArraysRef<'_> = decode("terrain layer arrays", Self::MAGIC, Self::FORMAT, bytes)?;
        let layers = v.table.layers.len().max(1);
        let slice = Self::slice_len(v.size, v.mips);
        if v.mips == 0 || v.mips > 32 || [v.albedo_height, v.normal, v.ao_roughness].iter().any(|a| a.len() != slice * layers) {
            return Err(ProductError::Invalid("layer array length"));
        }
        let skip = cap.skip(v.size, v.mips, layers);
        let head = (0..skip).map(|m| Self::level_len(v.size, m)).sum::<usize>();
        let keep = slice - head;
        let cut = |a: &[u8]| -> Vec<u8> {
            let mut out = Vec::with_capacity(keep * layers);
            for s in a.chunks_exact(slice) {
                out.extend_from_slice(&s[head..]);
            }
            out
        };
        Ok(Self {
            size: v.size >> skip,
            mips: v.mips - skip,
            albedo_height: cut(v.albedo_height),
            normal: cut(v.normal),
            ao_roughness: cut(v.ao_roughness),
            table: v.table,
        })
    }

    /// Bytes of level `mip` of one slice whose level 0 is `size`².
    pub fn level_len(size: u32, mip: u32) -> usize {
        let d = (size >> mip).max(1).div_ceil(4) as usize;
        d * d * Self::BLOCK_BYTES
    }

    /// Bytes of one slice (every level) of one array.
    pub fn slice_len(size: u32, mips: u32) -> usize {
        (0..mips).map(|m| Self::level_len(size, m)).sum()
    }

    /// Bytes of all three arrays: what the GPU holds.
    pub fn gpu_bytes(&self) -> usize {
        self.albedo_height.len() + self.normal.len() + self.ao_roughness.len()
    }
}

/// Which top levels of [`LayerArrays`] a load drops: the per-preset texture cap and a memory
/// budget (terrain.md "Layer arrays", textures.md §9 T6).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayerCap {
    /// Largest level-0 edge kept, texels (`None`: no cap).
    pub max_edge: Option<u32>,
    /// Largest total of the three arrays kept, bytes (`None`: no budget).
    pub budget: Option<usize>,
}

impl LayerCap {
    /// Smallest level 0 the cap ever leaves: one whole block (wgpu needs a block-compressed
    /// level 0 in whole blocks).
    pub const MIN_EDGE: u32 = 4;

    /// How many top levels to drop from arrays of `layers` slices, `size`², `mips` levels:
    /// until level 0 is at most `max_edge` and the arrays fit `budget`, never below
    /// [`Self::MIN_EDGE`] nor the last level.
    pub fn skip(&self, size: u32, mips: u32, layers: usize) -> u32 {
        let fits = |skip: u32| {
            let edge = size >> skip;
            let bytes = 3 * layers * LayerArrays::slice_len(edge, mips - skip);
            self.max_edge.is_none_or(|m| edge <= m) && self.budget.is_none_or(|b| bytes <= b)
        };
        let mut skip = 0;
        while !fits(skip) && skip + 1 < mips && (size >> (skip + 1)) >= Self::MIN_EDGE {
            skip += 1;
        }
        skip
    }
}

/// Static ray-tracing proxy of one tile (`terrain.rt_proxy`): every
/// `stride`-th sample, triangulated like the canonical tiles.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RtProxy {
    pub coord: TileCoord,
    pub stride: u32,
    /// Samples per edge.
    pub samples: u32,
    pub heights: Vec<f32>,
}

impl RtProxy {
    const MAGIC: [u8; 4] = *b"SFTR";
    pub const FORMAT: u32 = 1;

    pub fn to_bytes(&self) -> Vec<u8> {
        encode(Self::MAGIC, Self::FORMAT, self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProductError> {
        decode("terrain rt proxy", Self::MAGIC, Self::FORMAT, bytes)
    }

    /// Mesh data relative to the tile's origin (`TileCoord::origin`; heights
    /// stay absolute Y): positions, smooth normals, and triangle indices
    /// split along the canonical diagonal, counter-clockwise seen from +Y.
    pub fn mesh(&self) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
        let n = self.samples as usize;
        #[allow(clippy::cast_precision_loss)]
        let step = self.coord.spacing() * self.stride as f32;
        let h = |i: usize, j: usize| self.heights[j.min(n - 1) * n + i.min(n - 1)];
        let mut positions = Vec::with_capacity(n * n);
        let mut normals = Vec::with_capacity(n * n);
        for j in 0..n {
            for i in 0..n {
                #[allow(clippy::cast_precision_loss)]
                positions.push([i as f32 * step, h(i, j), j as f32 * step]);
                let dx = h(i + 1, j) - h(i.saturating_sub(1), j);
                let dz = h(i, j + 1) - h(i, j.saturating_sub(1));
                let span = |a: usize| if a == 0 || a == n - 1 { step } else { 2.0 * step };
                let (sx, sz) = (dx / span(i), dz / span(j));
                let len = (sx * sx + sz * sz + 1.0).sqrt();
                normals.push([-sx / len, 1.0 / len, -sz / len]);
            }
        }
        let mut indices = Vec::with_capacity((n - 1) * (n - 1) * 6);
        let v = |i: usize, j: usize| u32::try_from(j * n + i).expect("small mesh");
        for j in 0..n - 1 {
            for i in 0..n - 1 {
                // Main diagonal (i, j)-(i+1, j+1); +Z is "down" in i/j order, so CCW from +Y is 00, 11, 10.
                indices.extend_from_slice(&[v(i, j), v(i + 1, j + 1), v(i + 1, j)]);
                indices.extend_from_slice(&[v(i, j), v(i, j + 1), v(i + 1, j + 1)]);
            }
        }
        (positions, normals, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_bytes_round_trip() {
        let coord = TileCoord::new(2, -3, 7);
        let mut tile = HeightTile::flat(coord, 0.0);
        for (k, h) in tile.heights.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let x = (k % TILE_SAMPLES) as f32;
            #[allow(clippy::cast_precision_loss)]
            let z = (k / TILE_SAMPLES) as f32;
            *h = 40.0 * (x * 0.03).sin() + 0.1 * z;
        }
        let p = TileProduct {
            coord,
            heights: EncodedHeights::encode(&tile),
            bounds: BlockBounds::of(&tile),
            layers: None,
            tint: None,
        };
        let bytes = p.to_bytes();
        assert!(bytes.len() < TILE_SAMPLES * TILE_SAMPLES * 2, "{} bytes", bytes.len());
        assert_eq!(TileProduct::from_bytes(&bytes).unwrap(), p);
        assert!(MacroProduct::from_bytes(&bytes).is_err());
    }

    #[test]
    fn layer_ids_are_stable() {
        assert_eq!(layer_id("grass"), layer_id("grass"));
        assert_ne!(layer_id("grass"), layer_id("rock"));
        assert_eq!(layer_id(""), LayerId(0x811c_9dc5));
    }
}
