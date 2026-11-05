//! Bevy-free texture processing for bake steps.
//!
//! - [`delight`]: albedo de-lighting, which divides the low-frequency light painted into a
//!   texture back out of it at bake time (spec: `docs/specs/bake.md`, "De-lighting").
//! - [`ktx2_out`]: the baked texture's mip chain (gamma-correct, alpha-weighted box filter), BC7
//!   encode and KTX2 container.
//!
//! Inputs are decoded texels (RGBA8); decoding source container formats belongs to the bake step
//! that calls this crate.

pub mod delight;
pub mod ktx2_out;

pub use delight::{ShadingGrid, delight, shading_grid};
pub use ktx2_out::{
    Bc7Quality, MipLevel, bc7_ktx2, bc7_ktx2_cutout, encode_bc7, mip_chain, preserve_coverage,
    write_ktx2_bc7,
};
