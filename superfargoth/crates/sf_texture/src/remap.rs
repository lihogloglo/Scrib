//! Block-exact channel remaps: a block-compressed texture's channels reordered, complemented,
//! dropped or split out **on the blocks themselves**, without decoding or re-encoding
//! (textures.md §7 "pack profiles", slice T2b).
//!
//! A pack ships its data in its own layout (a DirectX normal's green points down, a `_spec` map
//! stores metal in red); the runtime wants one canonical layout (Bevy's: +Y normals, glTF ORM,
//! depth = 1 − height). A [`ChannelMap`] says where each output channel comes from; [`plan`]
//! finds the block-level operation that produces it, or says why none exists, and
//! [`Remap::level`] runs it. The rule kept: no second lossy pass when only the layout changes.
//!
//! **Operations** (each output decodes to the transformed decode of its input):
//! - **BC1, BC2/BC3 colour block.** Endpoints are RGB 5:6:5 and the palette is interpolated per
//!   channel, so swapping R and B (equal widths) and complementing a field (`x → 2ⁿ−1−x`, whose
//!   bit-replicated 8-bit expansion is exactly `255 − x`) commute with decoding. The swap or
//!   complement can flip the endpoints' order, which in BC1 selects the mode (`c0 > c1`: four
//!   colours; else three plus transparent black): the endpoints are swapped back and the indices
//!   remapped (four-colour `0↔1, 2↔3`; three-colour `0↔1`). G can't move to R or B (6 bits vs
//!   5). A three-colour block whose texels use index 3 (transparent black) can't be complemented
//!   (black has no complement in that mode): [`RemapError::PunchThroughInvert`].
//! - **Drop alpha: BC2/BC3 → BC1.** The colour block of BC2/BC3 always decodes in four-colour
//!   mode; forcing `c0 > c1` (swap + index remap, or all indices 0 when `c0 == c1`) makes it a
//!   BC1 block with the same colours. Halves the size.
//! - **BC4 complement** (a BC3 alpha block, a BC5 half): `e → 255 − e` on both endpoints, which
//!   reverses their order, so they swap and the indices remap (eight-value mode `0↔1, 2↔7, 3↔6,
//!   4↔5`; six-value mode `0↔1, 2↔5, 3↔4` and the constants `0↔255`, indices `6↔7`). BC2's
//!   explicit 4-bit alpha complements as `x ^ 0xF`.
//! - **Split out a channel**: a BC3 alpha block *is* a BC4 block (same encoding), and a BC5
//!   half is one; so `_nh` height (DXT5 alpha) becomes a BC4 depth map, complemented, with no
//!   re-encode. The colour block of the same file becomes the BC1 normal.
//! - **BC7 channel permutation.** Every mode stores the R, G and B endpoint fields at the same
//!   width, one after another, with the same weights for each channel: permuting the fields
//!   permutes the decode exactly. Modes 4/5's rotation (swap A with R, G or B after decoding)
//!   maps through the permutation (an R↔B swap maps rotation 1 ↔ 3).
//!
//! **Not block-exact** ([`RemapError`], the caller decodes and cooks): complementing any BC7
//! channel (the p-bit of modes 0, 1, 3, 6 and 7 is shared by all channels of an endpoint, so one
//! channel's field can't be complemented alone, and BC7's normative integer interpolation
//! `((64 − w)·e0 + w·e1 + 32) >> 6` is off by one under complement when the sum is a multiple of
//! 64 plus 32), moving data across channels of different widths or kinds (G into R in BC1,
//! colour into a BC4, alpha into BC7 colour), and the punch-through case above.
//!
//! Exactness of the interpolated BC1/BC4 palette entries holds for the formats' definitions
//! (real-valued `⅔·c0 + ⅓·c1`, D3D's tolerance-based decode); integer decoders that floor
//! (libsquish) differ by at most 1/255 on those entries, as they already do from the GPU.

use serde::{Deserialize, Serialize};

use crate::cook::Channel;
use crate::ktx2_out::BlockFormat;
use crate::rewrap::with_srgb;

/// Where one output channel's values come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Src {
    /// The output doesn't use this channel: the remap keeps whatever is cheapest (a colour
    /// channel keeps the source channel in its position, alpha is dropped where the format can,
    /// BC2/BC3 → BC1).
    Unused,
    /// This source channel, as is.
    Channel(Channel),
    /// This source channel complemented (`255 − x`): height → depth, gloss → roughness, a
    /// DirectX normal's green.
    Inverted(Channel),
}

impl Src {
    const fn resolve(self) -> Option<(Channel, bool)> {
        match self {
            Self::Unused => None,
            Self::Channel(channel) => Some((channel, false)),
            Self::Inverted(channel) => Some((channel, true)),
        }
    }

    const fn inverted(self) -> Self {
        match self {
            Self::Unused => Self::Unused,
            Self::Channel(channel) => Self::Inverted(channel),
            Self::Inverted(channel) => Self::Channel(channel),
        }
    }
}

/// Output channel (R, G, B, A) ← source channel: a texture's layout relative to another.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChannelMap(pub [Src; 4]);

impl ChannelMap {
    /// Every channel as it is.
    pub const IDENTITY: Self = Self([
        Src::Channel(Channel::R),
        Src::Channel(Channel::G),
        Src::Channel(Channel::B),
        Src::Channel(Channel::A),
    ]);

    /// This map, then `next` applied to its output: `next`'s output channel `o` takes this
    /// map's channel `next[o]` (complemented again if `next` inverts it).
    #[must_use]
    pub fn then(self, next: Self) -> Self {
        Self(next.0.map(|src| match src {
            Src::Unused => Src::Unused,
            Src::Channel(channel) => self.0[channel_index(channel)],
            Src::Inverted(channel) => self.0[channel_index(channel)].inverted(),
        }))
    }

    /// The same map on decoded RGBA8 texels, for the cook path (decode, map, cook): an unused
    /// colour channel keeps its value, an unused alpha becomes 255.
    ///
    /// # Panics
    /// If `rgba.len()` isn't a multiple of 4.
    pub fn apply_rgba(&self, rgba: &mut [u8]) {
        let (texels, rest) = rgba.as_chunks_mut::<4>();
        assert!(rest.is_empty(), "RGBA8 texels");
        for texel in texels {
            let source = *texel;
            for (out, src) in self.0.iter().enumerate() {
                texel[out] = match src.resolve() {
                    None if out == 3 => u8::MAX,
                    None => source[out],
                    Some((channel, false)) => source[channel_index(channel)],
                    Some((channel, true)) => u8::MAX - source[channel_index(channel)],
                };
            }
        }
    }
}

const fn channel_index(channel: Channel) -> usize {
    match channel {
        Channel::R => 0,
        Channel::G => 1,
        Channel::B => 2,
        Channel::A => 3,
    }
}

/// Why a map has no block-exact operation for a format (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RemapError {
    /// The format can't hold the requested layout without re-encoding: a channel moved across
    /// widths or kinds (G into R in BC1, colour into BC4, alpha into BC7 colour), a channel the
    /// format doesn't store, or BC2 alpha as a single channel.
    Unrepresentable,
    /// A complement in BC7: shared p-bits and the integer interpolation's rounding make it
    /// inexact (module docs). Decode + cook instead.
    Bc7Invert,
    /// A BC1 three-colour block uses transparent black (index 3) and a channel is complemented.
    PunchThroughInvert,
}

/// A BC1-style colour block operation: output channel `o` takes source channel `src[o]`
/// (0 R, 1 G, 2 B), complemented where `invert[o]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ColourOp {
    src: [usize; 3],
    invert: [bool; 3],
}

impl ColourOp {
    const IDENTITY: Self = Self {
        src: [0, 1, 2],
        invert: [false; 3],
    };

    fn inverts(self) -> bool {
        self.invert.contains(&true)
    }

    /// One 5:6:5 endpoint.
    fn apply565(self, color: u16) -> u16 {
        let fields = [color >> 11, (color >> 5) & 0x3F, color & 0x1F];
        let mut out = [0u16; 3];
        for (channel, value) in out.iter_mut().enumerate() {
            let max = if channel == 1 { 0x3F } else { 0x1F };
            let field = fields[self.src[channel]];
            *value = if self.invert[channel] {
                max - field
            } else {
                field
            };
        }
        (out[0] << 11) | (out[1] << 5) | out[2]
    }
}

/// What [`Remap::level`] does to every block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Identity,
    /// BC1 → BC1, the block's mode kept.
    Bc1(ColourOp),
    /// BC2 → BC2: colour block (four-colour), explicit alpha complemented or not.
    Bc2 {
        colour: ColourOp,
        invert_alpha: bool,
    },
    /// BC3 → BC3: colour block (four-colour), alpha block complemented or not.
    Bc3 {
        colour: ColourOp,
        invert_alpha: bool,
    },
    /// BC2/BC3 → BC1: the colour block, forced to four-colour order.
    DropAlpha(ColourOp),
    /// BC3 → BC4: the alpha block.
    AlphaToBc4 {
        invert: bool,
    },
    /// BC4 → BC4 complemented.
    Bc4Invert,
    /// BC5 → BC5: output half `o` is source half `src[o]`, complemented where `invert[o]`.
    Bc5 {
        src: [usize; 2],
        invert: [bool; 2],
    },
    /// BC5 → BC4: one half.
    Bc5ToBc4 {
        half: usize,
        invert: bool,
    },
    /// BC7 → BC7: output field `o` is source field `perm[o]`.
    Bc7 {
        perm: [usize; 3],
    },
}

/// A planned remap of one texture: the operation and the format it produces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Remap {
    op: Op,
    input: BlockFormat,
    /// The output's block format, linear (UNORM) variant; the caller picks the colour space.
    pub output: BlockFormat,
}

/// The block-exact operation that turns `format` blocks into `map`'s layout (module docs).
///
/// # Errors
/// [`RemapError::Unrepresentable`] or [`RemapError::Bc7Invert`] when there is none. The
/// data-dependent [`RemapError::PunchThroughInvert`] comes from [`Remap::level`].
pub fn plan(format: BlockFormat, map: &ChannelMap) -> Result<Remap, RemapError> {
    use BlockFormat as F;
    let input = with_srgb(format, false);
    let [r, g, b, a] = map.0.map(Src::resolve);
    let (op, output) = match input {
        F::Bc1 | F::Bc1Srgb => {
            if !matches!(a, None | Some((Channel::A, false))) {
                return Err(RemapError::Unrepresentable);
            }
            match colour_op([r, g, b])? {
                ColourOp::IDENTITY => (Op::Identity, F::Bc1),
                colour => (Op::Bc1(colour), F::Bc1),
            }
        }
        F::Bc2 | F::Bc2Srgb | F::Bc3 | F::Bc3Srgb => match (r, g, b, a) {
            (Some((Channel::A, invert)), None, None, None) if input == F::Bc3 => {
                (Op::AlphaToBc4 { invert }, F::Bc4)
            }
            (.., None) => (Op::DropAlpha(colour_op([r, g, b])?), F::Bc1),
            (.., Some((Channel::A, invert_alpha))) => {
                let colour = colour_op([r, g, b])?;
                let op = match (colour, invert_alpha, input) {
                    (ColourOp::IDENTITY, false, _) => Op::Identity,
                    (_, _, F::Bc2) => Op::Bc2 {
                        colour,
                        invert_alpha,
                    },
                    _ => Op::Bc3 {
                        colour,
                        invert_alpha,
                    },
                };
                (op, input)
            }
            _ => return Err(RemapError::Unrepresentable),
        },
        F::Bc4 => match (r, g, b, a) {
            (Some((Channel::R, false)), None, None, None) => (Op::Identity, F::Bc4),
            (Some((Channel::R, true)), None, None, None) => (Op::Bc4Invert, F::Bc4),
            _ => return Err(RemapError::Unrepresentable),
        },
        F::Bc5 => {
            if b.is_some() || a.is_some() {
                return Err(RemapError::Unrepresentable);
            }
            let half = |src: Option<(Channel, bool)>, own: usize| match src {
                None => Ok((own, false)),
                Some((Channel::R, invert)) => Ok((0, invert)),
                Some((Channel::G, invert)) => Ok((1, invert)),
                Some(_) => Err(RemapError::Unrepresentable),
            };
            if r.is_some() && g.is_none() {
                let (half, invert) = half(r, 0)?;
                (Op::Bc5ToBc4 { half, invert }, F::Bc4)
            } else {
                let (r, g) = (half(r, 0)?, half(g, 1)?);
                let (src, invert) = ([r.0, g.0], [r.1, g.1]);
                if src == [0, 1] && invert == [false; 2] {
                    (Op::Identity, F::Bc5)
                } else {
                    (Op::Bc5 { src, invert }, F::Bc5)
                }
            }
        }
        F::Bc7 | F::Bc7Srgb => {
            if [r, g, b, a].iter().flatten().any(|&(_, invert)| invert) {
                return Err(RemapError::Bc7Invert);
            }
            if !matches!(a, None | Some((Channel::A, false))) {
                return Err(RemapError::Unrepresentable);
            }
            let perm = bc7_permutation([r, g, b])?;
            if perm == [0, 1, 2] {
                (Op::Identity, F::Bc7)
            } else {
                (Op::Bc7 { perm }, F::Bc7)
            }
        }
    };
    Ok(Remap { op, input, output })
}

/// The colour block op for R, G, B sources (module docs: R/B fields 5 bits, G 6 bits).
fn colour_op(rgb: [Option<(Channel, bool)>; 3]) -> Result<ColourOp, RemapError> {
    let mut op = ColourOp::IDENTITY;
    for (out, src) in rgb.into_iter().enumerate() {
        let Some((channel, invert)) = src else {
            continue;
        };
        let fits = match out {
            1 => channel == Channel::G,
            _ => matches!(channel, Channel::R | Channel::B),
        };
        if !fits {
            return Err(RemapError::Unrepresentable);
        }
        op.src[out] = channel_index(channel);
        op.invert[out] = invert;
    }
    Ok(op)
}

/// A permutation of BC7's R, G, B fields: unused outputs take the remaining channels in order.
fn bc7_permutation(rgb: [Option<(Channel, bool)>; 3]) -> Result<[usize; 3], RemapError> {
    let mut perm = [usize::MAX; 3];
    for (out, src) in rgb.iter().enumerate() {
        if let Some((channel, _)) = src {
            let index = channel_index(*channel);
            if index > 2 || perm.contains(&index) {
                return Err(RemapError::Unrepresentable);
            }
            perm[out] = index;
        }
    }
    for out in 0..3 {
        if perm[out] == usize::MAX {
            let free = (0..3)
                .find(|index| !perm.contains(index))
                .expect("three slots, three channels");
            perm[out] = free;
        }
    }
    Ok(perm)
}

impl Remap {
    /// The blocks come out unchanged (only the container changes).
    pub const fn is_identity(&self) -> bool {
        matches!(self.op, Op::Identity)
    }

    /// One level's blocks remapped.
    ///
    /// # Errors
    /// [`RemapError::PunchThroughInvert`] (module docs).
    ///
    /// # Panics
    /// If `blocks.len()` isn't a whole number of the input format's blocks.
    pub fn level(&self, blocks: &[u8]) -> Result<Vec<u8>, RemapError> {
        let size = self.input.block_bytes();
        assert_eq!(blocks.len() % size, 0, "whole {:?} blocks", self.input);
        let count = blocks.len() / size;
        let mut out = Vec::with_capacity(count * self.output.block_bytes());
        for block in blocks.chunks_exact(size) {
            match self.op {
                Op::Identity => out.extend_from_slice(block),
                Op::Bc1(colour) => {
                    out.extend_from_slice(&colour_block(half8(block, 0), colour, false)?);
                }
                Op::Bc2 {
                    colour,
                    invert_alpha,
                } => {
                    let alpha = half8(block, 0);
                    out.extend(alpha.map(|byte| if invert_alpha { !byte } else { byte }));
                    out.extend_from_slice(&colour_block(half8(block, 1), colour, true)?);
                }
                Op::Bc3 {
                    colour,
                    invert_alpha,
                } => {
                    out.extend_from_slice(&bc4_block(half8(block, 0), invert_alpha));
                    out.extend_from_slice(&colour_block(half8(block, 1), colour, true)?);
                }
                Op::DropAlpha(colour) => {
                    out.extend_from_slice(&colour_block(half8(block, 1), colour, true)?);
                }
                Op::AlphaToBc4 { invert } => {
                    out.extend_from_slice(&bc4_block(half8(block, 0), invert));
                }
                Op::Bc4Invert => out.extend_from_slice(&bc4_block(half8(block, 0), true)),
                Op::Bc5 { src, invert } => {
                    for (half, invert) in src.into_iter().zip(invert) {
                        out.extend_from_slice(&bc4_block(half8(block, half), invert));
                    }
                }
                Op::Bc5ToBc4 { half, invert } => {
                    out.extend_from_slice(&bc4_block(half8(block, half), invert));
                }
                Op::Bc7 { perm } => {
                    let block: [u8; 16] = block.try_into().expect("16-byte BC7 block");
                    out.extend_from_slice(&bc7_block(block, perm));
                }
            }
        }
        Ok(out)
    }
}

/// The `index`th 8-byte half of a block.
fn half8(block: &[u8], index: usize) -> [u8; 8] {
    block[index * 8..][..8]
        .try_into()
        .expect("an 8-byte half block")
}

/// A BC1 colour block (or a BC2/BC3 colour block when `four`, always four-colour) with `op`
/// applied, endpoints reordered to keep the decode mode (module docs). A `four` source comes out
/// in BC1 four-colour order, so it is also a valid BC1 block.
fn colour_block(block: [u8; 8], op: ColourOp, four: bool) -> Result<[u8; 8], RemapError> {
    let c0 = u16::from_le_bytes([block[0], block[1]]);
    let c1 = u16::from_le_bytes([block[2], block[3]]);
    let mut indices = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
    let three = !four && c0 <= c1;
    if three && op.inverts() && indices & (indices >> 1) & 0x5555_5555 != 0 {
        return Err(RemapError::PunchThroughInvert);
    }
    let (mut t0, mut t1) = (op.apply565(c0), op.apply565(c1));
    if three {
        if t0 > t1 {
            (t0, t1) = (t1, t0);
            // 0 <-> 1 where the high bit is clear; 2 and 3 stay.
            indices ^= (!indices >> 1) & 0x5555_5555;
        }
    } else if t0 < t1 {
        (t0, t1) = (t1, t0);
        indices ^= 0x5555_5555;
    } else if t0 == t1 {
        // Every palette entry is the same colour; index 0 also reads it in three-colour mode.
        indices = 0;
    }
    let mut out = [0; 8];
    out[..2].copy_from_slice(&t0.to_le_bytes());
    out[2..4].copy_from_slice(&t1.to_le_bytes());
    out[4..].copy_from_slice(&indices.to_le_bytes());
    Ok(out)
}

/// A BC4 block (BC3 alpha block, BC5 half), complemented when `invert` (module docs).
fn bc4_block(block: [u8; 8], invert: bool) -> [u8; 8] {
    const EIGHT: [u64; 8] = [1, 0, 7, 6, 5, 4, 3, 2];
    const SIX: [u64; 8] = [1, 0, 5, 4, 3, 2, 7, 6];
    if !invert {
        return block;
    }
    let (a0, a1) = (block[0], block[1]);
    let table = if a0 > a1 { EIGHT } else { SIX };
    let mut bits = [0u8; 8];
    bits[..6].copy_from_slice(&block[2..]);
    let indices = u64::from_le_bytes(bits);
    let mut remapped = 0u64;
    for texel in 0..16 {
        let index = (indices >> (3 * texel)) & 7;
        remapped |= table[usize::try_from(index).expect("3 bits")] << (3 * texel);
    }
    let mut out = [0; 8];
    out[0] = u8::MAX - a1;
    out[1] = u8::MAX - a0;
    out[2..].copy_from_slice(&remapped.to_le_bytes()[..6]);
    out
}

/// BC7 mode layout: (subsets, partition bits, rotation bits, index-selection bits, colour bits
/// per endpoint channel), from the BC7 format reference (Khronos Data Format spec §21).
const BC7_MODES: [(u32, u32, u32, u32, u32); 8] = [
    (3, 4, 0, 0, 4),
    (2, 6, 0, 0, 6),
    (3, 6, 0, 0, 5),
    (2, 6, 0, 0, 7),
    (1, 0, 2, 1, 5),
    (1, 0, 2, 0, 7),
    (1, 0, 0, 0, 7),
    (2, 6, 0, 0, 5),
];

/// A BC7 block with its R, G, B endpoint fields permuted (output field `o` = source field
/// `perm[o]`) and the rotation mapped through the permutation. Reserved mode 8 (first byte 0)
/// decodes to zero in every channel and is kept.
fn bc7_block(block: [u8; 16], perm: [usize; 3]) -> [u8; 16] {
    let mode = block[0].trailing_zeros();
    let Some(&(subsets, partition, rotation, selection, colour)) =
        BC7_MODES.get(usize::try_from(mode).expect("mode < 9"))
    else {
        return block;
    };
    let bits = u128::from_le_bytes(block);
    let start = mode + 1 + partition + rotation + selection;
    let len = 2 * subsets * colour;
    let mask = (1u128 << len) - 1;
    let fields = [0u32, 1, 2].map(|channel| (bits >> (start + channel * len)) & mask);
    let mut out = bits & !(((1u128 << (3 * len)) - 1) << start);
    for (channel, &src) in perm.iter().enumerate() {
        let index = u32::try_from(channel).expect("channel < 3");
        out |= fields[src] << (start + index * len);
    }
    if rotation > 0 {
        let shift = mode + 1;
        let swapped = (bits >> shift) & 3;
        if swapped != 0 {
            let src = usize::try_from(swapped - 1).expect("2 bits");
            let to = perm.iter().position(|&p| p == src).expect("a permutation");
            let to = u128::try_from(to + 1).expect("rotation < 4");
            out = (out & !(3u128 << shift)) | (to << shift);
        }
    }
    out.to_le_bytes()
}

/// Rebuilds `channel` of level 0 from level 1 when level 0 holds none of it and level 1 does
/// (a pack-defect repair, [`crate::Quirk::FillEmptyLevel0`]): the rest of the chain and every
/// other channel stay as they are. `levels`: the texture's levels in `format`, level 0 first,
/// `width` x `height` at level 0. True when level 0 changed.
///
/// BC1 only, R, G or B (the defect seen so far ships BC1; other formats are left as shipped):
/// a BC1 block's texels all share one index set across channels, so one channel can't vary
/// within a block without re-encoding it. Level 0 is "empty" when that field is 0 in both
/// endpoints of every block (every texel decodes to 0 then); each block then takes the mean of
/// the level-1 texels it covers (2x2), quantized to the field's width and written to both
/// endpoints: block-constant, so the rebuilt channel has level 2's resolution. Writing the same
/// value to a field both endpoints share keeps their order, so the block's mode and its other
/// channels decode exactly as before.
pub fn fill_empty_level0(format: BlockFormat, levels: &mut [Vec<u8>], width: u32, height: u32, channel: Channel) -> bool {
    let (shift, bits) = match channel {
        Channel::R => (11, 5),
        Channel::G => (5, 6),
        Channel::B => (0, 5),
        Channel::A => return false,
    };
    if with_srgb(format, false) != BlockFormat::Bc1 || levels.len() < 2 || width < 4 || height < 4 {
        return false;
    }
    let mask: u16 = ((1u16 << bits) - 1) << shift;
    let endpoint = |block: &[u8], at: usize| u16::from_le_bytes([block[at], block[at + 1]]);
    if levels[0].as_chunks::<8>().0.iter().any(|b| (endpoint(b, 0) | endpoint(b, 2)) & mask != 0) {
        return false;
    }
    let (w1, h1) = ((width / 2).max(1), (height / 2).max(1));
    if levels[1].len() != crate::ktx2_out::level_len(format, w1, h1) {
        return false;
    }
    let texels = crate::rewrap::decode_level(format, &levels[1], w1, h1);
    let at = channel_index(channel);
    let value = |x: u32, y: u32| u32::from(texels[(y.min(h1 - 1) * w1 + x.min(w1 - 1)) as usize * 4 + at]);
    if (0..w1 * h1).all(|i| texels[i as usize * 4 + at] == 0) {
        return false;
    }
    let blocks_x = width.div_ceil(4);
    let max = (1u32 << bits) - 1;
    for (i, block) in levels[0].as_chunks_mut::<8>().0.iter_mut().enumerate() {
        let i = u32::try_from(i).unwrap_or(u32::MAX);
        let (x, y) = ((i % blocks_x) * 2, (i / blocks_x) * 2);
        let mean = (value(x, y) + value(x + 1, y) + value(x, y + 1) + value(x + 1, y + 1) + 2) / 4;
        let field = u16::try_from((mean * max + 127) / 255).unwrap_or(0) << shift;
        for at in [0, 2] {
            let bytes = ((endpoint(block, at) & !mask) | field).to_le_bytes();
            block[at..at + 2].copy_from_slice(&bytes);
        }
    }
    true
}

#[cfg(test)]
mod tests;
