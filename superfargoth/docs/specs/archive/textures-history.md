# textures.md history

Moved out of `textures.md` §9 to keep the spec under its budget. Newest last.

## T1 plan (2026-11-11, built as planned; as built in textures.md §9)

**T1 in detail** (first build slice, Bevy-free, no new dependency):
- `superfargoth/crates/sf_texture/src/ktx2_out.rs`: generalise `write_ktx2_bc7(width, height, srgb, levels)`
  (line ~359) into `pub fn write_ktx2(format: BlockFormat, width, height, levels: &[Vec<u8>]) -> Vec<u8>` with
  `BlockFormat { Bc1, Bc1Srgb, Bc2, Bc2Srgb, Bc3, Bc3Srgb, Bc4, Bc5, Bc7, Bc7Srgb }` → vkFormat 133/134 (BC1 RGBA), 135/136,
  137/138, 139, 141, 145/146 and block bytes 8/16; keep `write_ktx2_bc7` as a one-line wrapper so `mw_bake::delight`
  and its products don't change (no `VERSION` bump). Generalise `bc7_level_len` to `level_len(format, w, h)`.
- NEW `superfargoth/crates/sf_texture/src/cook.rs`: `pub enum Role { Color { alpha: AlphaUse }, Normal { source_y:
  SourceY }, Mask }`, `pub enum AlphaUse { None, Test(f32), Blend }`, `pub enum SourceY { Up, Down }`;
  `pub fn format_for(role, has_alpha) -> BlockFormat`; `pub fn normal_mip_chain(rgba, w, h, source_y) -> Vec<MipLevel>`
  (level 0: G inverted for `Down`; each level box-filters decoded vectors `2c/255 − 1`, renormalises, re-encodes;
  B = 255 kept for viewers); `pub fn encode_bc4(r, w, h)`, `pub fn encode_bc5(rg, w, h)` (`intel_tex_2::bc4/bc5`);
  `pub fn cook_rgba(role, rgba, w, h, quality) -> Vec<u8>` (KTX2 bytes; `Color` reuses `mip_chain` +
  `preserve_coverage` + `encode_bc7`).
- NEW `superfargoth/crates/sf_texture/src/cook/tests.rs`; `lib.rs` re-exports; Cargo.toml description line.
- Tests (exact): KTX2 header vkFormat and level count per format; every level's length = blocks × 8/16; flat
  normal (128,128,255) stays (128,128) at every level; a random normal field has unit length within 2/255 at every
  level before encoding; `Down` inverts G at level 0 exactly; `write_ktx2_bc7` output byte-identical to
  `write_ktx2(Bc7Srgb, ..)`; BC5 round trip through `block_compression::decode` within a sanity bound. One
  `#[ignore = "measurement: ..."]` test prints ms per MPix for BC7 basic/fast, BC5, BC4.
- Verify: `bash bevyrrowind/tools/cargo_turn.sh test -p sf_texture` then clippy (pedantic, `-D warnings`) on the
  crate. No game launch; nothing visual.

## T1 as built (2026-11-11; moved from textures.md §9 at T2b)

**T1 as built** (2026-11-11): `ktx2_out::{BlockFormat, write_ktx2, level_len}` (level offsets aligned to the block
size, 8 or 16; `write_ktx2_bc7` a wrapper, byte-identical by golden test, de-light `VERSION` unchanged). `cook`:
`Role { Color { alpha }, Normal { source_y }, Mask { channel, invert }, Packed }`, `Role::height(channel)`;
`format_for(role)` (alpha lives in the role); `normal_mip_chain` rebuilds Z from XY (what BC5 shows; source blue
ignored), keeps level 0 XY bytes, stores Z in blue; `mask_mip_chain`, `linear_mip_chain`, `encode_bc4/bc5`,
`cook_levels` (payload for T3) + `cook_rgba`. `AlphaUse::None` cooks alpha 255. `_nh` = `Normal` + `height(A)`.
15 tests + 1 measurement in `cook/tests.rs` (BC4/BC5 checked with a test-side decoder: `block_compression` is `bc7`-only here).

## T2 as built (2026-11-11; moved from textures.md §9 at T2b)

**T2 as built** (2026-11-11): `sf_texture::rewrap(bytes, role) -> Rewrap { Rewrapped | Cook(reason) | Skip(reason) }`,
`read_source`, `target_format`, `decode_level`, `encode_level` (the "encode this level in format F" dispatcher).
DDS headers via `ddsfile` (level sizes from blocks, never pitch fields; `dwMipMapCount` read even without its flag,
as DirectXTex's loader), KTX2 via `ktx2`. **Colour space follows the role** (colour sRGB, the rest UNORM), not the
file's DXGI flag: Bevy's own DDS loader does the same and many writers label sRGB albedo UNORM. Cook outcomes:
not BCn, format unfit for the role (layout changes: T2b below). Skips: unaligned level 0,
cubemap/volume/array, BC6H, signed BC4/BC5, Basis, supercompressed KTX2. Missing levels fill **in the source
format** with the role's mip filter, so shipped levels stay untouched and no whole-file re-encode is ever needed:
ISPC for BC1 opaque/BC3/BC4/BC5/BC7, `texpresso` (libsquish port, in tree) for BC1 punch-through and BC2; decode
`texpresso` BC1-5 + `block_compression` BC7 (both now normal deps; `ddsfile`/`texpresso` added as workspace deps).
`Rewrapped.as_shipped` marks a full-chain KTX2 already in the target format (index as-is). 12 tests in
`rewrap/tests.rs`; measurement `mw_bake/tests/rewrap_real_install.rs`.
