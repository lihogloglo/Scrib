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

## T2b as built (2026-11-11; moved from textures.md §9 at T3)

**T2b as built** (2026-11-11): `sf_texture::remap`: `ChannelMap([Src; 4])` (output ← `Channel(c)` / `Inverted(c)` /
`Unused` = keep the cheapest: colour keeps its own channel, alpha drops where it can), `then` (composition),
`apply_rgba` (same map on decoded texels for the cook path), `plan(format, map) -> Remap | RemapError`, `Remap::level`.
**Exact on the blocks**: BC1 / BC2-BC3 colour R↔B swap and any-channel complement (565 fields; endpoint order
restored, indices remapped), BC2/BC3 → BC1 (alpha dropped, four-colour order forced), BC3 alpha → BC4 and BC5 half →
BC4 (± complement), BC4/BC5 complement and half swap, BC7 any RGB permutation (fields + rotation). **Fallbacks**
(`CookReason::NotBlockExact`): any BC7 complement (shared p-bits; integer interpolation off by one at ties), a BC1
three-colour block using transparent black under a complement (`PunchThroughInvert`); moves across widths/kinds →
`FormatForRole`. `rewrap` now runs every role through its canonical map (`canonical(role)`: Down normal → G
complemented; normal alpha unused, so BC2/BC3 normals become BC1; mask channel → R) and `rewrap_mapped(bytes, layout,
role)` composes a pack layout first; `Rewrapped.remapped` flags it. `sf_texture::profile`: `PackProfile { name,
suffixes: [SuffixRule { suffix, role: PackRole, layout }], normal_y, height_neutral, quirks: [Quirk] }` (serde;
`Quirk` has no variant yet), `rule_for(stem)` (longest suffix), `companions(base)` (lookup order), `apply(profile,
rule, bytes, alpha) -> [Product { slot, map, role, result }]` (`_nh` → Normal + Depth). Exactness tests:
integer reference decoders (BC1 ×6, BC4 ×35) on random blocks; BC7 bit-exact vs `block_compression`,
8 modes × 6 perms. Instances: `mw_bake::texture_profiles`.

## T4b as built (2026-11-11; runtime material, T4 part 2 of 3)

`mw_assets::Surface.normal_texture` (companion file); `to_material`'s texture callback takes an sRGB flag
(`image_settings_clamped(.., srgb)`: Bevy's `is_srgb` overrides the KTX2/DDS format, so normals load
`false`), sets `normal_map_texture` on lit surfaces with the base clamp, `flip_normal_map_y` false (cook wrote
OpenGL Y; BC5 gets Bevy's two-component Z). Cooked path: `cooked::NormalCompanions` (`read_companions` once
per VFS, cached in `CookedMaterials`, key `mw_bake::cook::companion_of`: VfsPath-normalised stem),
`cooked_material_with` / `material_mapper(vfs)` for streamed statics; plain `cooked_material` and the runtime
NIF path take no normal map (no tangents yet). Inert until part 3 (tangents in `mesh.cook`). Test
`normal_map_iff_the_base_texture_has_a_companion`. Anisotropic filtering: preset lives in `mw_game_core`
perf.rs (other session's scope), not done.

## T3 as built (2026-11-11; moved from textures.md §9 at T4)

Adapter `mw_bake::cook`: three steps, not two. `texture.cook` (per winner, key = file +
role + quality; no LZ4) stores the served KTX2 **followed by** its `CookMeta` (postcard + `u32` length; the layer cuts
it off on read, a `truncate`); `texture.cook.meta` (per winner) holds the meta alone, so `texture.cook.index` reads a
few bytes per texture and an incremental bake re-reads only changed products. `CookPath`: **in place** (BCn DDS/KTX2
whose format suits the role, full chain, no remap: nothing copied, the shipped file loads; the entry keeps the level
table and the DDS payload offset), **rewrapped** (levels filled or remapped), **encoded** (TGA/BMP/PNG/uncompressed:
BC7 sRGB Basic), or skipped (`CookSkip`: ambiguous stem, sky/UI, unaligned, unsupported, supercompressed,
undecodable). Morrowind roles are all colour (`job_for`): blend/effect/glow/detail keep alpha, alpha-tested uses keep
coverage (a candidate tests only if its alpha has holes), opaque albedo drops alpha. `CookedTextureLayer` replaces
`DelightLayer` (one top layer, label `cooked textures (bake)`, de-light entries first). Bake stats line:
`texture.cook: N textures (in place a, rewrapped b [filled, remapped], encoded c) ... by alpha, by format, VRAM as
shipped -> cooked, served from the bake, skipped by reason, built, CPU-s`. Not yet: `--off texcook` (`mount_layers`
takes the switch; `Part::TexCook` + two `main.rs` calls pending), per-level zstd (S1 loader), opaque BC3 → BC1
(adapter §5), per-format ms/MPix in the line.

## T4 as built (2026-11-11; parts 1 and 3; part 2 is "T4b" above)

Part 1 (bake, adapter `mw_bake::cook` + `delight`): `normal_companion(profile, reference, resolve)` is the one
companion lookup (profile order `_nh`, `_n`, `_nm`; `<stem><suffix>.tga` through `TEXTURE_ALTERNATES`);
`plan_normals` adds a `CookJob::normal` per sole-variant companion of an albedo use (ambiguous: `CookSkip::Ambiguous`);
BC1/BC3/BC5/BC7 sources rewrap through the exact G complement, others decode + cook to BC5 UNORM, all linear.
The layer serves `cooked/texture_normals.postcard` (base stem -> companion) with current companions only. De-light
skips a texture only when a `_n/_nh/_spec` comes from the albedo's own VFS source. Bake line: "N normal companions".

Part 3 (tangents, framework `sf_model`): `tangent.rs` runs `bevy_mikktspace` (Bevy's byte-identical port of the
reference MikkTSpace, Bevy-free, Zlib/MIT/Apache) per triangle corner, un-indexes the group, and the weld merges
vertex + tangent back, so mirrored-UV seams split (test: a 17x17 grid mirrored at its middle column gains exactly
17 vertices). Degenerate corners get a unit tangent perpendicular to the normal. Tangents ride the vertex index
through simplification (not a simplifier attribute: meshopt treats the split as a seam) and the per-level fetch
remap (`cook::fetch_order`, shared by static and skinned). `CookedSubmesh.tangents` (meshopt-encoded `[f32; 4]`,
empty = none); `.sfmesh` v4, MIN 4 (v1-v3 retired, everything re-cooks once), `KEY_FORMAT` 3. Adapter hook
`steps::TangentRule { version, wants(&Material, &mut RunCx) }` on `mesh.cook`/`mesh.rest`/`mesh.rig` (version in
the key); `mw_bake::cook::tangent_rule()` = lit material whose base texture has a companion, each candidate through
`RunCx::file_exists`, so a pack adding a normal map re-cooks only models using that base (trace test
`tangent_rule_dependencies_recook_only_affected_models`). Loaders: `sf_model_bevy` inserts `ATTRIBUTE_TANGENT`
(static, skinned, RT mesh). Streamed statics use `mw_assets::cooked::material_mapper`. Gaps: `sf_character`
assembly (`AssembledMesh`) and `sf_scene`'s `scene.chains` concatenation drop tangents (actors keep
`cooked_material`); morph targets carry no tangent deltas; no `--off normals` A/B (needs `main.rs`/`perf.rs`).
