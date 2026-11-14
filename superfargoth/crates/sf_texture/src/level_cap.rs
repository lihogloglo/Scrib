//! Load-time level-0 size cap for KTX2 textures (textures.md §6 "Pool per preset", slice S1).
//!
//! A KTX2 file carries its whole mip chain; under a cap the loader starts at the first level whose
//! larger side fits, so the top levels are never read nor uploaded (UE's per-group `MaxLODSize`,
//! Unity's `globalTextureMipmapLimit`). The kept levels are byte-identical to the file's: only the
//! header (level-0 size, level count), the level index and the data offsets are rewritten.
//!
//! Rules:
//! - Block-compressed formats never start at a level that isn't a whole number of blocks on both
//!   sides (wgpu needs a block-aligned level 0): the start backs off toward level 0 until it is.
//! - The last level is never dropped, and a file the cap can't trim safely is served untouched:
//!   unknown `vkFormat` (`VK_FORMAT_UNDEFINED`, `BasisLZ`/UASTC), `BasisLZ` supercompression (its
//!   global data indexes every level), 3D textures, `levelCount` 0 (no chain).
//! - Per-level supercompression (Zstandard, zlib) trims like raw data: each level is its own stream.
//! - KTX2 stores levels smallest first, so the top levels sit at the end of the file: the loader
//!   reads only up to [`CapPlan::read_len`].
//!
//! Colour space is untouched: the trimmed file keeps its `vkFormat` and DFD.

use std::borrow::Cow;

use ktx2::{Header, LevelIndex, SupercompressionScheme, dfd};

/// Bytes of the fixed KTX2 header (identifier + fields + index).
pub const HEADER_LEN: usize = Header::LENGTH;

/// A KTX2 file the cap can't read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapError {
    /// Shorter than the header, the level index or a section it points to.
    Truncated,
    /// Not a KTX2 header (identifier or field values).
    Header(String),
    /// The level index or a section offset points inside the header or the level index.
    Layout,
}

impl std::fmt::Display for CapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => f.write_str("ktx2 cap: file truncated"),
            Self::Header(e) => write!(f, "ktx2 cap: bad header: {e}"),
            Self::Layout => f.write_str("ktx2 cap: sections overlap the level index"),
        }
    }
}

impl std::error::Error for CapError {}

/// Why a file is served whole whatever the cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uncappable {
    /// `vkFormat` undefined or without a known texel block (Basis, UASTC, vendor formats).
    UnknownBlock,
    /// `BasisLZ`: its supercompression global data describes every level.
    BasisLz,
    /// A 3D texture (depth halves per level too).
    Volume,
    /// `levelCount` 0: the file has one level and asks the loader to generate the rest.
    NoChain,
}

/// The header and level index of a KTX2 file: enough to plan a cap.
#[derive(Clone, Debug)]
pub struct Layout {
    pub header: Header,
    /// Level 0 first.
    pub levels: Vec<LevelIndex>,
    /// Texel block size `(width, height)` and bytes per block, `Err` when the file can't be capped.
    pub block: Result<((u32, u32), u32), Uncappable>,
}

/// What a cap does to one file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapPlan {
    /// Top levels dropped (0: the file is served as is).
    pub skip: u32,
    /// Level-0 size after the cap.
    pub width: u32,
    pub height: u32,
    /// Level count after the cap.
    pub levels: u32,
    /// GPU bytes (uncompressed level lengths) of the levels kept and of the levels dropped.
    pub bytes_kept: u64,
    pub bytes_skipped: u64,
    /// File bytes the trimmed file is built from (a prefix of the file); `None` when `skip` is 0
    /// (the whole file is needed).
    pub read_len: Option<u64>,
    /// Set when the cap wanted to trim but the file can't be.
    pub uncappable: Option<Uncappable>,
}

/// Bytes of the level index following the header, read from the header's `levelCount`.
///
/// # Errors
/// [`CapError::Header`] when `head` isn't a KTX2 header.
pub fn level_index_len(head: &[u8; HEADER_LEN]) -> Result<usize, CapError> {
    let header = Header::from_bytes(head).map_err(|e| CapError::Header(format!("{e:?}")))?;
    Ok(LevelIndex::LENGTH * header.level_count.max(1) as usize)
}

impl Layout {
    /// Parses the header and level index from the start of a KTX2 file (`head` holds at least
    /// [`HEADER_LEN`] + [`level_index_len`] bytes; more is ignored).
    ///
    /// # Errors
    /// [`CapError::Truncated`] or [`CapError::Header`].
    pub fn parse(head: &[u8]) -> Result<Self, CapError> {
        let fixed: &[u8; HEADER_LEN] = head.get(..HEADER_LEN).and_then(|h| h.try_into().ok()).ok_or(CapError::Truncated)?;
        let header = Header::from_bytes(fixed).map_err(|e| CapError::Header(format!("{e:?}")))?;
        let count = header.level_count.max(1) as usize;
        let index = head.get(HEADER_LEN..HEADER_LEN + LevelIndex::LENGTH * count).ok_or(CapError::Truncated)?;
        let levels = index.as_chunks::<{ LevelIndex::LENGTH }>().0.iter().map(LevelIndex::from_bytes).collect();
        let block = if header.level_count == 0 {
            Err(Uncappable::NoChain)
        } else if header.pixel_depth > 1 {
            Err(Uncappable::Volume)
        } else if header.supercompression_scheme == Some(SupercompressionScheme::BasisLZ) {
            Err(Uncappable::BasisLz)
        } else {
            header
                .format
                .and_then(|f| dfd::Basic::from_format(f).ok())
                .map(|(basic, _)| {
                    let [w, h, ..] = basic.texel_block_dimensions;
                    ((u32::from(w.get()), u32::from(h.get())), u32::from(basic.bytes_planes[0]))
                })
                .ok_or(Uncappable::UnknownBlock)
        };
        Ok(Self { header, levels, block })
    }

    /// The plan under `cap` (largest side of level 0 in texels; `None`: no cap).
    pub fn plan(&self, cap: Option<u32>) -> CapPlan {
        let (width, height) = (self.header.pixel_width, self.header.pixel_height);
        let gpu = |l: &LevelIndex| l.uncompressed_byte_length;
        let total: u64 = self.levels.iter().map(gpu).sum();
        let whole = CapPlan {
            skip: 0,
            width,
            height,
            levels: self.header.level_count,
            bytes_kept: total,
            bytes_skipped: 0,
            read_len: None,
            uncappable: None,
        };
        let Some(cap) = cap else { return whole };
        if width.max(height) <= cap {
            return whole;
        }
        let ((bw, bh), _) = match self.block {
            Ok(b) => b,
            Err(reason) => return CapPlan { uncappable: Some(reason), ..whole },
        };
        let skip = first_level(width, height, self.header.level_count, (bw, bh), cap);
        if skip == 0 {
            return whole;
        }
        let kept = &self.levels[skip as usize..];
        let read_len = kept
            .iter()
            .map(|l| l.byte_offset + l.byte_length)
            .chain([self.meta_end()])
            .max()
            .unwrap_or(0);
        CapPlan {
            skip,
            width: level_size(width, skip),
            height: if height == 0 { 0 } else { level_size(height, skip) },
            levels: self.header.level_count - skip,
            bytes_kept: kept.iter().map(gpu).sum(),
            bytes_skipped: self.levels[..skip as usize].iter().map(gpu).sum(),
            read_len: Some(read_len),
            uncappable: None,
        }
    }

    /// Bytes from the end of the level index to the end of the DFD, KVD and SGD: copied verbatim.
    fn meta_start(&self) -> u64 {
        (HEADER_LEN + LevelIndex::LENGTH * self.levels.len()) as u64
    }

    fn meta_end(&self) -> u64 {
        let i = &self.header.index;
        let dfd = u64::from(i.dfd_byte_offset) + u64::from(i.dfd_byte_length);
        let kvd = if i.kvd_byte_length == 0 { 0 } else { u64::from(i.kvd_byte_offset) + u64::from(i.kvd_byte_length) };
        let sgd = if i.sgd_byte_length == 0 { 0 } else { i.sgd_byte_offset + i.sgd_byte_length };
        dfd.max(kvd).max(sgd).max(self.meta_start())
    }
}

/// Size of a level's side (never below 1).
pub const fn level_size(size: u32, level: u32) -> u32 {
    let s = size >> level;
    if s == 0 { 1 } else { s }
}

/// First level kept under `cap`: the first whose larger side is at most `cap`, never past the last
/// level, then backed off toward 0 until both sides are whole blocks of `block` texels.
pub fn first_level(width: u32, height: u32, levels: u32, block: (u32, u32), cap: u32) -> u32 {
    let last = levels.saturating_sub(1);
    let mut k = 0;
    while k < last && level_size(width, k).max(level_size(height.max(1), k)) > cap {
        k += 1;
    }
    let aligned = |k: u32| {
        level_size(width, k).is_multiple_of(block.0) && (height == 0 || level_size(height, k).is_multiple_of(block.1))
    };
    while k > 0 && !aligned(k) {
        k -= 1;
    }
    k
}

/// The KTX2 file of `plan` built from `prefix` (the file's first `plan.read_len` bytes or more):
/// header and level index rewritten, DFD/KVD/SGD and the kept levels' bytes copied unchanged,
/// each level at the alignment KTX2 requires (`lcm(4, block bytes)`, 1 when supercompressed).
///
/// # Errors
/// [`CapError::Truncated`] when `prefix` is short, [`CapError::Layout`] when a section offset
/// points inside the header or the level index.
pub fn trim(prefix: &[u8], layout: &Layout, plan: &CapPlan) -> Result<Vec<u8>, CapError> {
    let skip = plan.skip as usize;
    let block_bytes = layout.block.map_or(1, |(_, b)| b);
    let align = if layout.header.supercompression_scheme.is_some() { 1 } else { lcm(4, u64::from(block_bytes.max(1))) };
    let delta = (LevelIndex::LENGTH * skip) as u64;
    let (meta_start, meta_end) = (layout.meta_start(), layout.meta_end());
    let index = &layout.header.index;
    if u64::from(index.dfd_byte_offset) < meta_start
        || (index.kvd_byte_length > 0 && u64::from(index.kvd_byte_offset) < meta_start)
        || (index.sgd_byte_length > 0 && index.sgd_byte_offset < meta_start)
    {
        return Err(CapError::Layout);
    }
    let range = |start: u64, len: u64| -> Result<&[u8], CapError> {
        let s = usize::try_from(start).map_err(|_| CapError::Truncated)?;
        let e = usize::try_from(start + len).map_err(|_| CapError::Truncated)?;
        prefix.get(s..e).ok_or(CapError::Truncated)
    };

    let mut header = layout.header;
    header.pixel_width = plan.width;
    header.pixel_height = plan.height;
    header.level_count = plan.levels;
    header.index.dfd_byte_offset -= u32::try_from(delta).map_err(|_| CapError::Layout)?;
    if header.index.kvd_byte_length > 0 {
        header.index.kvd_byte_offset -= u32::try_from(delta).map_err(|_| CapError::Layout)?;
    }
    if header.index.sgd_byte_length > 0 {
        header.index.sgd_byte_offset -= delta;
    }

    let kept = &layout.levels[skip..];
    let mut new_index = vec![LevelIndex { byte_offset: 0, byte_length: 0, uncompressed_byte_length: 0 }; kept.len()];
    let mut cursor = meta_end - delta;
    let mut data: Vec<(u64, &[u8])> = Vec::with_capacity(kept.len());
    // Smallest level first, as KTX2 stores them.
    for (i, level) in kept.iter().enumerate().rev() {
        cursor = cursor.next_multiple_of(align);
        new_index[i] = LevelIndex { byte_offset: cursor, ..*level };
        data.push((cursor, range(level.byte_offset, level.byte_length)?));
        cursor += level.byte_length;
    }

    let mut out = Vec::with_capacity(usize::try_from(cursor).unwrap_or(0));
    out.extend_from_slice(&header.as_bytes());
    for entry in &new_index {
        out.extend_from_slice(&entry.as_bytes());
    }
    out.extend_from_slice(range(meta_start, meta_end - meta_start)?);
    for (offset, bytes) in data {
        out.resize(usize::try_from(offset).map_err(|_| CapError::Truncated)?, 0);
        out.extend_from_slice(bytes);
    }
    Ok(out)
}

/// [`Layout::parse`] + [`Layout::plan`] + [`trim`] over a whole file in memory: the file itself
/// (borrowed) when nothing is trimmed.
///
/// # Errors
/// As [`Layout::parse`] and [`trim`].
pub fn cap_ktx2(bytes: &[u8], cap: Option<u32>) -> Result<(Cow<'_, [u8]>, CapPlan), CapError> {
    let layout = Layout::parse(bytes)?;
    let plan = layout.plan(cap);
    if plan.skip == 0 {
        return Ok((Cow::Borrowed(bytes), plan));
    }
    Ok((Cow::Owned(trim(bytes, &layout, &plan)?), plan))
}

const fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[cfg(test)]
mod tests;
