//! Validated little-endian DTLR layers and the compressor boundary.
use crate::{Error, Workspace};
use alloc::vec::Vec;

/// Upstream layer magic (`DTLR`).
pub const LAYER_MAGIC: i32 = 0x44544c52;
/// Size of the padded upstream layer header.
pub const LAYER_HEADER_SIZE: usize = 56;

/// A compressor writes only into caller-provided, budgeted storage.
/// Implementations must return the actual byte count; decode must produce exactly
/// three grids. Codec choice is external metadata, as in upstream.
pub trait LayerCompressor {
    /// Maximum compressed size for an input of `size` bytes.
    fn max_compressed_size(&self, size: usize) -> Result<usize, Error>;
    /// Compress into `output`.
    fn compress(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Error>;
    /// Decompress into `output`.
    fn decompress(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Error>;
}
/// Uncompressed, upstream-compatible layers; no additional codec dependency.
#[derive(Clone, Copy, Debug, Default)]
pub struct RawCompressor;
impl LayerCompressor for RawCompressor {
    fn max_compressed_size(&self, size: usize) -> Result<usize, Error> {
        Ok(size)
    }
    fn compress(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Error> {
        if input.len() > output.len() {
            return Err(Error::BufferTooSmall);
        }
        output[..input.len()].copy_from_slice(input);
        Ok(input.len())
    }
    fn decompress(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Error> {
        self.compress(input, output)
    }
}
/// Upstream layer header, excluding magic/version and padding.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayerHeader {
    /// Tile x.
    pub tx: i32,
    /// Tile z.
    pub ty: i32,
    /// Layer index.
    pub tlayer: i32,
    /// World bounds minimum.
    pub bmin: [f32; 3],
    /// World bounds maximum.
    pub bmax: [f32; 3],
    /// Minimum quantized height.
    pub hmin: u16,
    /// Maximum quantized height.
    pub hmax: u16,
    /// Grid width.
    pub width: u8,
    /// Grid height.
    pub height: u8,
    /// Usable minimum x.
    pub minx: u8,
    /// Usable maximum x.
    pub maxx: u8,
    /// Usable minimum z.
    pub miny: u8,
    /// Usable maximum z.
    pub maxy: u8,
}
impl LayerHeader {
    /// Creates a header covering the full grid; fill bounds and coordinates before building.
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            maxx: width.saturating_sub(1),
            maxy: height.saturating_sub(1),
            ..Self::default()
        }
    }
    pub(crate) fn validate(&self) -> Result<usize, Error> {
        if self.width == 0
            || self.height == 0
            || self.minx > self.maxx
            || self.miny > self.maxy
            || self.maxx >= self.width
            || self.maxy >= self.height
            || self.hmin > self.hmax
            || self.tlayer < 0
            || !(0..3).all(|i| {
                self.bmin[i].is_finite() && self.bmax[i].is_finite() && self.bmin[i] <= self.bmax[i]
            })
        {
            return Err(Error::InvalidInput("layer header"));
        }
        Ok(usize::from(self.width) * usize::from(self.height))
    }
    /// Decodes and validates a header without allocating.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < LAYER_HEADER_SIZE {
            return Err(Error::InvalidInput("truncated layer header"));
        }
        let i32_at = |i| i32::from_le_bytes(data[i..i + 4].try_into().unwrap());
        if i32_at(0) != LAYER_MAGIC || i32_at(4) != 1 {
            return Err(Error::InvalidInput("layer magic/version"));
        }
        let f = |i| f32::from_bits(u32::from_le_bytes(data[i..i + 4].try_into().unwrap()));
        let h = Self {
            tx: i32_at(8),
            ty: i32_at(12),
            tlayer: i32_at(16),
            bmin: [f(20), f(24), f(28)],
            bmax: [f(32), f(36), f(40)],
            hmin: u16::from_le_bytes([data[44], data[45]]),
            hmax: u16::from_le_bytes([data[46], data[47]]),
            width: data[48],
            height: data[49],
            minx: data[50],
            maxx: data[51],
            miny: data[52],
            maxy: data[53],
        };
        h.validate()?;
        Ok(h)
    }
    fn encode(&self, out: &mut [u8]) {
        for (i, v) in [LAYER_MAGIC, 1, self.tx, self.ty, self.tlayer]
            .iter()
            .enumerate()
        {
            out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        for (i, v) in self.bmin.iter().chain(&self.bmax).enumerate() {
            out[20 + i * 4..24 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        out[44..46].copy_from_slice(&self.hmin.to_le_bytes());
        out[46..48].copy_from_slice(&self.hmax.to_le_bytes());
        out[48..54].copy_from_slice(&[
            self.width,
            self.height,
            self.minx,
            self.maxx,
            self.miny,
            self.maxy,
        ]);
    }
}
/// Swaps the byte order of a layer header in place (`dtTileCacheHeaderSwapEndian`).
///
/// The crate reads and writes layers little-endian; upstream's "native" order is
/// the host's, so this converts between little-endian and big-endian layers and is
/// its own inverse. It accepts a header whose magic and version are both correct in
/// one byte order or both in the other, then reverses the bytes of the magic,
/// version, tile coordinates, bounds and the two height limits. The one-byte
/// fields and the compressed grids that follow the header are not touched.
/// [`Error::InvalidInput`] when the magic or version fit neither byte order or
/// `data` is shorter than the 56-byte header.
pub fn tile_cache_header_swap_endian(data: &mut [u8]) -> Result<(), Error> {
    if data.len() < LAYER_HEADER_SIZE {
        return Err(Error::InvalidInput("truncated layer header"));
    }
    let magic = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let version = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let native = magic == LAYER_MAGIC && version == 1;
    let swapped = magic == LAYER_MAGIC.swap_bytes() && version == 1i32.swap_bytes();
    if !native && !swapped {
        return Err(Error::InvalidInput("layer magic/version"));
    }
    for w in 0..11 {
        let b = w * 4;
        data.swap(b, b + 3);
        data.swap(b + 1, b + 2);
    }
    data.swap(44, 45);
    data.swap(46, 47);
    Ok(())
}

/// A decoded layer. Private grids preserve the validation invariant.
#[derive(Debug)]
pub struct Layer {
    pub(crate) header: LayerHeader,
    pub(crate) heights: Vec<u8>,
    pub(crate) areas: Vec<u8>,
    pub(crate) cons: Vec<u8>,
    pub(crate) regs: Vec<u8>,
    pub(crate) reg_count: u8,
    pub(crate) regions_built: bool,
}
impl Layer {
    /// Header.
    pub fn header(&self) -> &LayerHeader {
        &self.header
    }
    /// Heights, including 255 for empty cells.
    pub fn heights(&self) -> &[u8] {
        &self.heights
    }
    /// Area ids (zero is unwalkable).
    pub fn areas(&self) -> &[u8] {
        &self.areas
    }
    /// Connections and portals.
    pub fn connections(&self) -> &[u8] {
        &self.cons
    }
    /// Region ids, 255 for empty cells.
    pub fn regions(&self) -> &[u8] {
        &self.regs
    }
    /// Number of built regions.
    pub fn region_count(&self) -> u8 {
        self.reg_count
    }
}
fn validate_grids(
    h: &LayerHeader,
    heights: &[u8],
    areas: &[u8],
    cons: &[u8],
    ws: &mut Workspace,
) -> Result<(), Error> {
    let n = h.validate()?;
    if heights.len() != n || areas.len() != n || cons.len() != n {
        return Err(Error::InvalidInput("layer grid lengths"));
    }
    ws.work(u64::try_from(n).map_err(|_| Error::SizeOverflow)?)?;
    let w = usize::from(h.width);
    let hh = usize::from(h.height);
    for i in 0..n {
        if areas[i] > 63 || (areas[i] != 0 && heights[i] == 255) {
            return Err(Error::InvalidInput("layer area/height"));
        }
        let x = i % w;
        let y = i / w;
        // Connections may point to cells made unwalkable by later marking, but never outside the grid.
        if (x == 0 && cons[i] & 1 != 0)
            || (y + 1 == hh && cons[i] & 2 != 0)
            || (x + 1 == w && cons[i] & 4 != 0)
            || (y == 0 && cons[i] & 8 != 0)
        {
            return Err(Error::InvalidInput("layer connection outside grid"));
        }
    }
    Ok(())
}
/// Builds a canonical layer, with zero padding and a pluggable compressor.
pub fn build_layer(
    ws: &mut Workspace,
    comp: &impl LayerCompressor,
    header: &LayerHeader,
    heights: &[u8],
    areas: &[u8],
    cons: &[u8],
) -> Result<Vec<u8>, Error> {
    ws.begin();
    validate_grids(header, heights, areas, cons, ws)?;
    let n = heights.len();
    let size = n * 3;
    let bound = comp.max_compressed_size(size)?;
    let mut out = ws.vec(
        bound
            .checked_add(LAYER_HEADER_SIZE)
            .ok_or(Error::SizeOverflow)?,
        0u8,
    )?;
    let mut raw = ws.vec(size, 0u8)?;
    raw[..n].copy_from_slice(heights);
    raw[n..n * 2].copy_from_slice(areas);
    raw[n * 2..].copy_from_slice(cons);
    ws.work(
        u64::try_from(size.checked_add(bound).ok_or(Error::SizeOverflow)?)
            .map_err(|_| Error::SizeOverflow)?,
    )?;
    header.encode(&mut out);
    let used = comp.compress(&raw, &mut out[LAYER_HEADER_SIZE..])?;
    if used > bound {
        return Err(Error::InvalidInput("compressor output size"));
    }
    out.truncate(LAYER_HEADER_SIZE + used);
    Ok(out)
}
/// Validates and decompresses exactly three grids; truncated or oversized output is rejected.
pub fn decompress_layer(
    ws: &mut Workspace,
    comp: &impl LayerCompressor,
    data: &[u8],
) -> Result<Layer, Error> {
    ws.begin();
    decode(ws, comp, data)
}
pub(crate) fn decode(
    ws: &mut Workspace,
    comp: &impl LayerCompressor,
    data: &[u8],
) -> Result<Layer, Error> {
    if data.len() > ws.limits().max_tile_bytes {
        return Err(Error::LimitExceeded);
    }
    let h = LayerHeader::decode(data)?;
    let n = h.validate()?;
    ws.work(u64::try_from(data.len() + n * 3).map_err(|_| Error::SizeOverflow)?)?;
    let mut raw = ws.vec(n * 3, 0u8)?;
    if comp.decompress(&data[LAYER_HEADER_SIZE..], &mut raw)? != n * 3 {
        return Err(Error::InvalidInput("decompressed layer size"));
    }
    validate_grids(&h, &raw[..n], &raw[n..n * 2], &raw[n * 2..], ws)?;
    let mut heights = ws.vec(n, 0)?;
    heights.copy_from_slice(&raw[..n]);
    let mut areas = ws.vec(n, 0)?;
    areas.copy_from_slice(&raw[n..n * 2]);
    let mut cons = ws.vec(n, 0)?;
    cons.copy_from_slice(&raw[n * 2..]);
    Ok(Layer {
        header: h,
        heights,
        areas,
        cons,
        regs: ws.vec(n, 255)?,
        reg_count: 0,
        regions_built: false,
    })
}
