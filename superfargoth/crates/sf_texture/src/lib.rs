//! Bevy-free texture processing for bake steps.
//!
//! - [`delight`]: albedo de-lighting, which divides the low-frequency light painted into a
//!   texture back out of it at bake time (spec: `docs/specs/bake.md`, "De-lighting").
//! - [`ktx2_out`]: the baked texture's mip chain (gamma-correct, alpha-weighted box filter), BC7
//!   encode and the KTX2 container for every BC format this crate writes.
//! - [`cook`]: a texture cooked by its role (colour, normal, mask, packed) to BC7, BC5 or BC4 in
//!   KTX2 (spec: `docs/specs/textures.md` §5).
//! - [`rewrap`]: an already block-compressed DDS or KTX2 rewrapped into KTX2 without re-encoding,
//!   missing mip levels filled in its format (textures.md §5 "Rewrap rule").
//! - [`remap`]: block-exact channel remaps (Y flip, R/B swap, dropped alpha, split-out channel)
//!   run on the blocks of a rewrap (textures.md §7, slice T2b).
//! - [`profile`]: a texture pack's conventions as data, converted to the canonical layout
//!   at bake (textures.md §7).
//!
//! Inputs are decoded texels (RGBA8), or DDS/KTX2 bytes for [`rewrap`]; decoding other container
//! formats belongs to the bake step that calls this crate.

pub mod cook;
pub mod delight;
pub mod ktx2_out;
pub mod profile;
pub mod remap;
pub mod rewrap;

pub use cook::{
    AlphaUse, Channel, PlaneLevel, Role, SourceY, cook_levels, cook_rgba, encode_bc4, encode_bc5,
    format_for, linear_mip_chain, mask_mip_chain, normal_mip_chain,
};
pub use delight::{ShadingGrid, delight, shading_grid};
pub use ktx2_out::{
    Bc7Quality, BlockFormat, MipLevel, bc7_ktx2, bc7_ktx2_cutout, encode_bc7, level_len, mip_chain,
    preserve_coverage, write_ktx2, write_ktx2_bc7,
};
pub use profile::{PackProfile, PackRole, Product, Quirk, Slot, SuffixRule, apply, plan_products};
pub use remap::{ChannelMap, Remap, RemapError, Src, plan};
pub use rewrap::{
    BcSource, Container, CookReason, Rewrap, Rewrapped, SkipReason, canonical, decode_level,
    encode_level, read_source, rewrap, rewrap_mapped, target_format, with_srgb,
};
