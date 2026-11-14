# Textures: Morrowind adapter

> Status 2026-11-11: **design**; T0 packs census done, T2b remaps + pack profiles built (§7). Framework spec: `superfargoth/docs/specs/textures.md` (roles, cook, residency,
> slices T0-T8, S1-S4). Research: `superfargoth/docs/research/texture-compression-streaming-2026-11-11.md`.
> This file holds only what is Morrowind's: role mapping, companion names, conventions, the de-light rule, and the
> PBR pack census (§7; evidence in `superfargoth/docs/research/pbr-packs-census-2026-11-11.md`).

## 1. Roles from `texture.roles`

`mw_bake::delight::roles_of` already lists every NIF texture slot with its `Role` (`delight.rs:72-146`). The cook
maps them to `sf_texture::cook::Role`:

| `mw_bake::delight::Role` | Cook role | Note |
|---|---|---|
| `Albedo` (de-lit) | none: the de-light product is the cooked texture | `texture.delight`, unchanged |
| `Albedo` (de-light skipped: companion, multi-variant, already KTX2, small) | `Color { alpha }` from the base slot's cutoff / blend | alpha `Test(cutoff)` when `alpha_class` says cutout |
| `Glow`, `Emissive` | `Color { alpha: None }` (sRGB) | glow maps feed `emissive_texture` |
| `Detail` (detail/dark/decal slots) | `Color` | Morrowind's decal slots blend in sRGB |
| `Effect`, `Translucent` | `Color { alpha: Blend }` | particles, glass |
| sky (`is_sky_or_ui` sky names) | `Color` | |
| UI, splash, cursors, `Icons/`, `Bookart/` | `Ui` (untouched) | sampled 1:1 by `sf_ui` |
| `_n` companion | `Normal { source_y: Down }` | §2 |
| `_nh` companion | `NormalHeight` (normal + Bevy `depth_map`) | §7; `_nh` wins over `_n` (OpenMW order) |
| `_spec` | `Packed` (glTF ORM after an R↔B swap) | §7: R metal, G roughness, B AO, A 1 − subsurface (`wareya-pbr`); `openmw` sources: specular map converted to ORM, §10 |
| `_diffusespec` (terrain) | terrain layer: RGB albedo, A roughness | §7, T5/T6 |

NIF bump-map slots (`NiTexturingProperty` `BumpMap`, parsed in `mw_nif/src/material.rs:249-253`) and
`NiTextureEffect` stay ignored, as in OpenMW (its "convert bump-mapped mods" guide drops them).

## 2. Companions (normal-map packs)

- **Names** (OpenMW `[Shaders]` defaults, `texture-basics.html`): for `textures/foo.dds` the normal map is
  `textures/foo_n.<ext>`, height-normal `foo_nh`, specular `foo_spec`; terrain uses the same `_n` (OpenMW: "Also
  used for terrain normal maps") and `_diffusespec`. We also accept `_nm` (older packs; OpenMW asks users to rename)
  after `_n` and `_nh`, in that order. Extensions through `TEXTURE_ALTERNATES` (`ktx2`, `dds`, `png`) plus `tga`.
- **Lookup**: `mw_bake` resolves companions while planning (the companion's existence and fingerprint are cook and
  `mesh.cook` inputs); the runtime reads the cooked index, never probes the VFS per material. Before T3 lands, T4's
  loader probe `vfs.resolve_preferring(stem + "_n")` at material build is the scaffold (named, removed by T3).
- **Convention**: `source_y = Down` (OpenMW: "OpenMW uses the DX format for normal maps"), so cook inverts G. A
  pack authored OpenGL-style sets `normal_y = "up"` in its `mod.toml` `[texture]` (or `textures.toml`, §8).
- **Colour space**: linear. The as-shipped path sets `is_srgb = false` for the normal slot (today
  `mw_assets::image_settings_clamped` forces sRGB for every model texture, `mw_assets/src/lib.rs:498-510`).

## 3. De-light rule with companions (changes `plan_jobs`)

Today any `_n`/`_nh`/`_spec` companion skips de-lighting its albedo ("a PBR replacer: leave it as authored",
`delight.rs:29`). A **normals-only** pack over vanilla diffuse would then silently stop de-lighting those vanilla
albedos. Proposed (owner question 2): skip only when the companion's winning file and the albedo's winning file
come from the **same VFS source** (a pack that ships both was authored together, so its albedo is meant for normal
mapping); otherwise de-light as usual. T4 changes `Skip::Companion` accordingly and bumps the de-light index
`VERSION`; test: a vanilla albedo plus a companion from another source is de-lit, same source is skipped.

## 4. Where the code goes (by slice)

| Slice | Files |
|---|---|
| T0 census | `bevyrrowind/crates/mw_bake/src/census.rs` (texture pass beside the NIF census) |
| T3 cook | NEW `bevyrrowind/crates/mw_bake/src/cook.rs` (steps `texture.cook`, `texture.cook.index`), `mw_bake/src/delight.rs` (`DelightLayer` → `CookedTextureLayer` serving both indices, de-light first), `mw_boot_bevy::bake` (call order: after de-light) |
| T4 normals | `bevyrrowind/crates/mw_assets/src/lib.rs` (`Surface` gains `normal_texture`; `to_material` sets `normal_map_texture` with a linear clamped/repeat sampler), `mw_bake` `mesh.cook` adapter (tangents iff the material has a normal companion; `bake-mesh-cook.md`), `delight.rs` (§3) |
| T5 terrain | `world-adapter.md` `TerrainLayer` mapping: `normal = companion of the LTEX texture` |

## 5. Numbers (to fill from T0)

Vanilla + Tribunal + Bloodmoon: 3521 albedos de-lit (bake.md). Unmeasured: the other referenced textures, their
format mix (DXT1/DXT3/DXT5 DDS vs TGA vs uncompressed DDS), level counts, VRAM as shipped vs cooked, and the same
for the owner's normal-map pack once it arrives. The owner's PBR packs are measured in §7.

## 6. Visual checks (registered when T4/T5/S4 build; owner signs off)

- **TX-1** normal maps on statics: Balmora, noon and 21:00, with the pack installed, `--off texcook` A/B; look for
  bumps lit from the sun side (not inverted: Y convention), no sRGB washout (flat, low-contrast relief), no seams on
  mirrored UVs (tangents).
- **TX-2** terrain normals: Ascadian Isles road edges at 08:00 low sun, with/without the pack.
- **TX-3** residency: run Seyda Neen → Balmora on High with an HD pack; look for any texture sharpening within a few
  metres of the camera, and blurred textures that stay blurred near you.

## 7. PBR packs (census 2026-11-11, slice T0)

Packs: MVR PBR v2 Beta (Nexus 57592, 10 769 DDS, 13.0 GB), Tamrielic PBR West Gash 2K (60568, 42 DDS, 339 MiB),
Parallax Replacement v1.2 (58303, an OpenMW 0.51 POM shader: technique reference only, no licence notice, no code
ported). They target Wareya's PBR shaders for OpenMW (github.com/wareya/OpenMW-PBR, GPL-3.0, `cab24e7a`); the
channel layout below is that shader's (`lighting_pbr.glsl` `specMapToPBR`, `terrain.frag`) and the pixel statistics
confirm it. Method, per-suffix tables and evidence: `superfargoth/docs/research/pbr-packs-census-2026-11-11.md`.

| Suffix | Role | Channels (as shipped) | Bevy `StandardMaterial` slot | Cook path |
|---|---|---|---|---|
| none (diffuse) | `Color` (sRGB) | RGB albedo, A cutout/blend or unused 255 | `base_color_texture` | **rewrap** (BC1/2/3 as shipped, BC7 UNORM → BC7 sRGB vkFormat, same bytes); de-light skipped (same source, §3); truncated foliage chains filled (T2) |
| `_n` | `Normal { source_y: Down }` | RGB unit normal (BC1, 3 ATI2, 2 DXT5), DirectX | `normal_map_texture` (3-channel; BC5 sources 2-channel) | **rewrap + exact G complement** (below); never re-encoded to BC5 (doubles BC1's VRAM for no gain) |
| `_nh` (wins over `_n`) | `NormalHeight` | RGB normal DirectX, A height (white = high, neutral 0.5) | `normal_map_texture` + `depth_map` | DXT5 **split, block-exact**: colour block → BC1 normal (G complemented), alpha block → BC4 depth (`1 − h`, complemented); bytes unchanged. Uncompressed or BC7 sources (TPBR 10): decode + T1 cook (BC5 + BC4) |
| `_spec` | `Packed` | R metallic, G perceptual roughness, B AO, A 1 − subsurface | one image as both `metallic_roughness_texture` (G rough, B metal) and `occlusion_texture` (R) | **rewrap + exact R↔B swap** → glTF ORM; BC1 stays BC1, DXT5 drops its alpha block → BC1 (A unused: Bevy has no skin SSS input), BC7 swaps fields + rotation 1↔3 |
| `_diffusespec` | terrain layer | RGB = the pack's diffuse (mean abs diff < 2/255), A roughness | terrain layer arrays (`TerrainLayer`, T5/T6), not `StandardMaterial` | rewrap into the layer array (RGBA kept: albedo + roughness, one fetch) |

Material factors with a `_spec`: `metallic = 1.0`, `perceptual_roughness = 1.0` (Bevy multiplies factor × texture),
`reflectance = 0.5` (F0 0.04, as Wareya's `f0_scalar`). Without one, today's defaults stand. Textures are linear
except the diffuse (`is_srgb = false` for normal, ORM and depth).

**`_nh` height → parallax (built 2026-11-13, tx-pom; textures.md §9 "T4d as built").** The winning normal companion read
as `NormalHeight` by its own source's profile also cooks `texture.cook.depth`: the DXT5 alpha block split out as BC4,
complemented (bytes unchanged otherwise), BC7 / uncompressed decoded + BC4; capped at 1024² (top levels dropped);
served at `<nh stem>_depth.ktx2`, listed in `cooked/texture_depths.postcard`. Material: `depth_map` +
`parallax_depth_scale` 0.04 (OpenMW's `PARALLAX_SCALE` for these maps), method and layers per preset (Low none,
Medium Occlusion 16, High Relief 5 / 16, Ultra Relief 8 / 32); `--off parallax`, `flip parallax`. The profile's
`height_neutral` 0.5 is not applied (Bevy has no bias: the polygon is the top). Vanilla `_nh` (openmw profile) too.

**Conventions, measured.** Y is **DirectX** in both packs: for `_nh`, corr(R, ∂h/∂x) −0.84 and corr(G, ∂h/∂row)
−0.83 (MVR, 40/40 files; TPBR −0.70/−0.70, 10/10): same sign, so +Y points down the image; `_n` against diffuse
luminance agrees (41/43). Height: alpha, white = high (the R correlation's sign), per-file mean 128 in every MVR
sample = **neutral 0.5** (the POM shader's `PARALLAX_NEUTRAL 0.5`). Roughness, not smoothness: wood 255, metal trim
150-200, matching `PBR_MAT_ROUGHNESS_INVERTED 0`. Not OpenMW's vanilla `_spec` (RGB colour + A shininess).

**Block-exact remaps** (T2b, **built 2026-11-11**: `sf_texture::remap` + `profile`; profile instances `openmw` (default) and
`wareya-pbr` in `mw_bake/data/texture_profiles.ron`, read by `mw_bake::texture_profiles`; BC7 Y flip falls back to
decode + cook, see textures.md "T2b as built"): BC1/BC3 colour endpoints store R and B
at 5 bits each and G at 6, and the palette is per channel, so swapping R and B or complementing G (`g → 63 − g`,
whose 8-bit expansion is `255 − x`) on both endpoints is lossless; when the swap flips `c0 > c1` (the BC1 mode bit),
swap the endpoints and remap indices (4-colour 0↔1, 2↔3; 3-colour 0↔1). BC4/BC5 complement: `e → 255 − e`, swap
endpoints, remap (8-value 0↔1, 2↔7, 3↔6, 4↔5; 6-value mode also swaps the 0/255 constants). A DXT5 alpha block
*is* a BC4 block; a DXT5 colour block becomes BC1 after forcing `c0 > c1` (equal endpoints: all indices 0). BC7
stores R, G, B endpoint fields at equal widths in every mode: swap the R and B fields, map rotation 1 ↔ 3. Test:
decode(remapped) equals channel-permuted decode(source) on random blocks of every mode: endpoints exact,
interpolants within 1/255 (decoder rounding). This keeps
the standard rule "no second lossy pass unless the layout must change": here the layout changes without one.

**Numbers.** Vanilla BSAs: 4 783 texture names. MVR replaces 3 332 of them (3 329 upscaled; vanilla originals
96 MiB) and brings 53 new names; every companion's diffuse is in the pack, so de-light skips all 3 385 diffuses
(§3 "same source"). Formats: MVR diffuse DXT5 1 869 / DXT1 1 371 / DXT3 106, `_spec` DXT1 3 204 / DXT5 186, `_n`
DXT1 2 639, `_nh` DXT5 1 185, `_diffusespec` DXT5 165 (all 4096²); full chains except 104 foliage diffuses; 415
bases ship both `_n` and `_nh` (the `_n` is then unused). TPBR: BC7 UNORM except 7 uncompressed `_nh` and 1 DXT5.
Resident if all loaded, by cap 512/1024/2048/none: MVR 1.7/5.5/9.8/12.7 GiB, TPBR 21/83/331/339 MiB: residency
(S3/S4) is what fits a pool, the cap only bounds one texture. 26/30 sampled DXT5 diffuses are fully opaque (vanilla
DXT1): opaque BC3 → BC1 is block-exact and halves them (T3 option, counted in the bake stats).

**Budget.** Cold cook of MVR is IO-bound: 13 GB read at ~390 MB/s ≈ 35 s; remaps cost nothing measurable;
per-level zstd-3 (adopted for BC1-BC5 products: 51-81 % of raw, clears the 15 % bar; BC7 at 78-92 % stays raw)
~100 CPU-s, ~15 s on 8 threads; TPBR's 10 decoded `_nh` ≈ 42 MPix × 36 ms ≈ 1.5 s. About **1 min** total, inside
the 3 min target. Re-encoding MVR's diffuses to BC7 instead would be ~5 700 MPix × 1 255 ms ≈ 2 h CPU: rewrap is
mandatory, not an optimisation. Disk: a full KTX2 copy is ~8.5 GB with zstd against 28 GB free on the owner's C:
(bake store); T3 should index untransformed DDS in place (level offsets into the pack file, "authored beats
derived" extended to DDS) so only remapped textures (`_n`, `_nh`, `_spec`: 5.4 GB raw, ~3.8 GB zstd) are copied.

**Parallax** (decided 2026-11-11: Bevy native, compared against the owner's shader; research note §5). Start with
`ParallaxMappingMethod::Relief { max_steps: 5 }`, `max_parallax_layer_count` 32 on High/Ultra, 16 Medium, off on
Low, `parallax_depth_scale` 0.04 (the shader's UV scale). Bevy lacks the neutral offset (mid-grey reads as recessed),
distance fade, mip-filtered depth sampling (it samples level 0) and self-shadow: one later vendored `bevy_pbr` patch
(`own: parallax LOD`), its own slice after TX-1. Terrain parallax belongs to T5's terrain shader.

**Pack defect (owner question 7, textures.md §11).** MVR's metal channel is empty at level 0 in 3 160 of 3 204 DXT1
`_spec`; its `MetalMipmapsHotfix.zip` (422 armour/weapon `_spec`, not yet installed) adds metal to mips 1+ only, so
armour turns metallic a few metres away and matte up close. Options: as shipped, or the cook rebuilds level 0's R
from level 1 where level 0 has none and level 1 has some (proposed; blurrier metal mask, consistent at every
distance), or ignore the pack's metal. Report upstream either way.

## 8. Texture sources and profiles (TS-1a, 2026-11-13)

Owner 2026-11-13: read textures from any source, not just the packs linked so far. Each **VFS source** gets its own
pack profile (§7), chosen by data; the framework still names no pack.

**Reuse check.** Bevy 0.20 has nothing per pack: `bevy_image` `ImageLoaderSettings` (`image_loader.rs:134`) is
per asset (`format`, `texture_format`, `is_srgb`, `sampler`, `array_layout`), with no green flip, suffix or
companion notion, and our bake serves cooked KTX2 anyway. Crates: none (a profile is a few lines of data;
`sf_texture::PackProfile` already models it).

**Industry comparison.** UE5: per texture asset, "Flip Green Channel" plus compression setting `Normalmap` (BC5),
set by the importer (often from a `_N` suffix rule in import presets). Unity: per texture importer "Texture Type:
Normal map", presets matched by folder or name (`AssetPostprocessor`). OpenMW: one global set in `settings.cfg`
`[Shaders]` (`normal map pattern = _n`, `normal height map pattern = _nh`, `specular map pattern = _spec`,
`terrain specular map pattern = _diffusespec`), DirectX Y only. Bethesda (Skyrim/FO4): fixed suffixes `_n`, `_m`,
`_s`, `_g`, DirectX Y. So the standard is per-asset settings defaulted by a per-source naming rule; OpenMW's
global patterns are the weakest form (one convention for every pack). Ours: per-source profile at bake, which is
UE's import-preset model applied to mod folders.

**Design** (`mw_bake/src/texture_profiles.rs` `TextureSources`):
- **Choosing a profile.** A mod's `mod.toml` `[texture]` (`sf_mods::TextureSection`): `profile = "<name>"` (a name
  from `data/texture_profiles.ron`), `normal_y = "up" | "down"` overriding the profile's, `suffixes = { "_normal" =
  "_n" }` renaming a companion (the pack's suffix follows that profile rule's role and layout, looked up right after
  it). Manifest-less sources (loose `Data Files`, the ini archives, the builtin mod: all synthesized by `mw_mods`)
  read the user's `<user data>/textures.toml`, `[<mod id>]` tables of the same fields (e.g. `[morrowind-data]`);
  ids of folder mods there are ignored (they carry their own `mod.toml`). No entry: `openmw`. Unknown names and
  bad renames are bake log warnings (`texture profile warning: ...`) and fall back; assigned sources are logged
  (`texture profile: <source label> -> <profile>`).
- **Granularity is the mod.** All layers of one mod share its profile. Several packs dumped into one loose
  `Data Files` are one source with one profile: put a pack of another convention in `<user data>/mods/<pack>/`
  with a `mod.toml` (a folder there without one doesn't load: `NoManifest` warning).
- **Companion lookup uses the companion's source.** For base `foo`, the candidates are the union of every
  profile's normal suffixes (default's first: `_nh`, `_n`, `_nm`, OpenMW's global order, then the new ones); the
  first `foo<suffix>` that resolves and whose **winning file's own source** profile has that suffix as a normal
  rule wins, read with that profile's layout and Y. Why not the albedo's source: the common case is a normals-only
  pack over vanilla diffuse (BSA), and only the pack knows its convention. De-light's §3 rule is unaffected: it
  asks for companions in the albedo's own source, so one profile (that source's) covers both files; its companion
  list is `_n`/`_nh`/`_spec` plus that profile's suffixes.
- **Keys.** The cook job (layout, `source_y`) is the `texture.cook` `role` param, so a profile change re-cooks only
  that source's companions. `mesh.cook`'s tangent rule asks the suffix union (a superset of the cook's companions:
  extra tangents are harmless, missing ones shade wrong); its version is 1 while the union is the default's and a
  hash of it otherwise, so only a new suffix re-cooks meshes. The runtime layer checks fingerprints, not profiles:
  after changing `[texture]`, re-bake (the profile fingerprint already changes, the manifest is hashed).
- **Roles beyond normals.** `TextureSources::rule_for(stem, source)` routes `_spec` to ORM for a `wareya-pbr`
  source; materials don't bind ORM yet (separate slice).
- **Not built:** a per-folder rule inside one source; a Y-convention census warning at bake (§7 method).

## 9. Source formats (TS-1b, 2026-11-13)

What the bake reads from any source (a BSA, the loose `Data Files`, a mod folder: one VFS). Two readers:
`sf_texture::rewrap` (block-compressed files, kept as shipped; textures.md §5) and `mw_bake::textures::decode_rgba`
(everything the cook's decode path, de-light, terrain and far bakes decode). The decoder reads every format the rewrap
reads, so a block file the rewrap can't keep (a BC7 normal whose Y can't be flipped on the blocks, BC4/BC5 used as
colour) cooks instead of being `Undecodable`, as it was until TS-1b for BC4/BC5/BC7 and any KTX2.

**Reuse check.** Bevy 0.20 `bevy_image` (`dds.rs`, `ktx2.rs`): upload only, no CPU decode; it rejects most legacy
uncompressed DDS (`X8R8G8B8`, `R5G6B5`, `A1R5G5B5`, `A4R4G4B4`, `R8G8B8`) and maps `L8`/`A8L8`/`L16` to integer
formats, so a file the bake doesn't cook often can't load raw either: decode everything we can. `ddsfile` 0.6 parses
headers, but drops `DDPF_PALETTEINDEXED8` and matches a fixed mask list (and writes luminance as `DDPF_RGB`): legacy
masks are read from the raw header. `ktx2` 0.5: header and levels. Blocks: `sf_texture::decode_level` (`texpresso`
BC1-BC5, `block_compression` BC7). `image` 0.25: TGA (RLE, colour-mapped, greyscale), BMP (palettized, RLE), PNG.
JPEG: no decoder in the tree, and `image`'s `jpeg` feature changes Bevy's feature closure (a workspace rebuild +
hakari); Bevy's loader has no `jpeg` either: deferred to the crate-update batch.

**Industry comparison.** Morrowind reads DDS (DXT1/3/5, uncompressed), TGA, BMP; OpenMW reads through OSG plugins (DDS
with DXTn, RGTC and mask-matched uncompressed incl. luminance/alpha; TGA, BMP, PNG, JPEG). `DirectXTex`/`texconv` reads
every legacy D3D9 layout (P8 and luminance expanded) and every DXGI format, and writes BC1-BC7 and DXGI uncompressed:
modern packs come from it, NVTT or Compressonator. UE5 and Unity import DDS/TGA/PNG/BMP/JPEG/PSD/EXR and re-encode. We
read texconv's legacy set, its DXGI UNORM formats and KTX2; gaps vs UE/Unity are JPEG, PSD and HDR/EXR (no Morrowind
material input).

| Source format | Read by | Outcome |
|---|---|---|
| DDS legacy `DXT1`, `DXT2/3`, `DXT4/5`, `ATI1`/`BC4U`, `ATI2`/`BC5U`; DDS DX10 BC1-BC5, BC7 (UNORM, sRGB, TYPELESS); KTX2 BC1-BC5, BC7 | rewrap | kept (rewrapped, levels filled; full chain in the role's format: in place), else decoded + cooked |
| DDS legacy uncompressed, any RGB / luminance / alpha mask layout of 8-32 bits (`A8R8G8B8`, `X8R8G8B8`, `A8B8G8R8`, `R8G8B8`, `R5G6B5`, `X1R5G5B5`, `A1R5G5B5`, `A4R4G4B4`, `X4R4G4B4`, `A8R3G3B2`, `L8`, `A8L8`, `A4L4`, `A8`, `L16`, `G16R16`, 10:10:10:2); `FourCC` 36 (`A16B16G16R16`); P8 | decoder | cooked |
| DDS DX10 `R8G8B8A8`, `B8G8R8A8`, `B8G8R8X8`, `R8G8`, `R8`, `A8`, `R16G16B16A16`/`R16G16`/`R16` UNORM, `R10G10B10A2`, `B5G6R5`, `B5G5R5A1`, `B4G4R4A4` | decoder | cooked |
| KTX2 `R8G8B8A8`, `A8B8G8R8` pack, `B8G8R8A8`, `R8G8B8`, `B8G8R8`, `R8G8`, `R8` (UNORM, sRGB), `R16*` UNORM, `A2B10G10R10`, `R5G6B5`, `A1R5G5B5`, `B4G4R4A4` | decoder | cooked |
| TGA (true colour, RLE, colour-mapped, greyscale), BMP (1-32 bit, palettized, RLE), PNG | decoder (`image`) | cooked |
| signed BC4/BC5, BC6H, float, integer, YUV, ETC/ASTC, Basis; cubemap, volume, array | skip | served as shipped (`Unsupported` / `Undecodable`): not Morrowind material inputs (OpenMW samples normals UNORM; no HDR slot); Bevy loads BC6H and cubemaps as shipped |
| KTX2 zstd/zlib supercompressed | skip | served as shipped (`Supercompressed`; Bevy reads zstd) |
| JPEG | skip | `Undecodable` (deferred, above) |

- **The bytes decide**: a `DDS ` or KTX2 signature is that container whatever the name; PNG/BMP by signature; TGA by
  extension. Level sizes come from the format, never the pitch field; rows tightly packed (the DDS/KTX2 references;
  texconv's `-dword` legacy DWORD-aligned 24-bit files are not detected); `dwMipMapCount` 0 reads as 1 (rewrap).
- **Channels as D3D9/GL sample them**: missing colour 0, missing alpha 1, luminance replicated to RGB (OSG and the
  original engine read `L8` as grey; `DirectXTex` maps it to `R8` unless told to expand: we follow the engines the
  content was made for), `A8` = (0, 0, 0, a), DXGI `R8`/`R8G8` red / red-green.
- **Level 0 not a whole number of 4x4 blocks** (wgpu and D3D11 refuse it for BC). Standard: keep such textures
  uncompressed (Unity's fallback when it can't block-compress a size); no resize (changes texel addressing, needs a
  filter per role) and no padding (wrapped UVs would sample the pad). The decoder crops partial blocks, so the cook
  can do it; today it's still `CookSkip::Unaligned`, served as shipped (fine for TGA/PNG, a raw unaligned BC DDS fails
  to load). Follow-up in `cook.rs`: encode them as RGBA8 KTX2.
- **Sources** (`mw_bake/tests/texture_sources.rs`): every container above cooks the same from the BSA, the loose folder
  or a mod above both; the highest source's file wins; a normal companion from another source than its albedo is
  found and cooked. One stem in two extensions across sources (BSA `x.dds`, mod `x.tga`) is `Ambiguous`, served as
  shipped: a `.dds` and a `.tga` reference resolve to different files. A `.bmp` reference finds only `.bmp`
  (`TEXTURE_ALTERNATES` has no `bmp`, as the original engine). Per-format exact texels:
  `mw_bake/src/textures/tests.rs`.

## 10. `_spec` maps into materials (TS-2, 2026-11-13)

Owner 2026-11-13 after TX-1: PBR `_spec` into materials. Decided 2026-11-11 (textures.md §11 q5): "`_spec` mapped to PBR".

**Reuse check.** Bevy 0.20 `StandardMaterial` (`vendor/bevy_pbr/src/pbr_material.rs:152-174,423-446`): one image as both
`metallic_roughness_texture` (G roughness, B metallic) and `occlusion_texture` (R) is its documented glTF setup; factors
multiply the texture, so both go to 1.0. Per-texel specular colour/intensity (`specular_tint_texture`,
`specular_texture`) sit behind the `pbr_specular_textures` feature, off in our Bevy: enabling it rebuilds Bevy for every
session (CONTRIBUTING "Building"), so not here; candidate for the crate-update batch. Crates: none (the conversion is a
256-entry table).

**Industry comparison.** Packed ORM: glTF 2.0 core, UE (`_ORM`/`_ARM` with sRGB off), Unity HDRP mask map: linear, one
fetch. Legacy specular/gloss: UE and Unity have no runtime specular-Phong path; importers and converters turn
gloss/shininess into roughness at import (Unity keeps a "Specular setup" shader, UE converts). The standard mapping of a
Blinn-Phong exponent to microfacet roughness is Walter et al. 2007 (Phong ↔ Beckmann `α = √(2/(n+2))`, GGX alike;
Karis, "Specular BRDF Reference"), perceptual roughness `r = √α`. OpenMW's specular map: RGB specular colour, A
shininess (`objects.frag`: `shininess = specTex.a * 255.0`, Blinn-Phong); `[Shaders] specular map pattern = _spec`.

**Design.**
- `_spec` follows its own VFS source's profile (§8): `wareya-pbr` → `PackRole::Orm` (R↔B swap, rewrapped block-exactly:
  BC1 stays BC1, DXT5 drops its alpha to BC1, BC7 swaps fields; else decode + BC7 UNORM); `openmw` (default) →
  `PackRole::SpecularShininess`: decoded, `G = round(255·(2/(255·A+2))^¼)`, R 255, B 0, then box mips and **BC1
  linear** (one varying channel, 6 bits). The colour is dropped (no Bevy input without the feature above): vanilla-style
  spec packs give roughness, not tint.
- Plan: `plan_orms` (as `plan_normals`, albedo uses only, sole variant else `Ambiguous`), ORM jobs keyed by an `orm`
  param (`texture.cook` VERSION 3, index 3); `CookIndex::orms` base stem → `OrmCompanion { file, occlusion }`, served as
  `cooked/texture_orms.postcard` only while current (stale: none, not metal in the wrong channel).
- Material (`mw_assets` `Surface::to_material`, statics and every cooked-model path through `TextureCompanions`):
  ORM bound linear with the base slot's clamp; `occlusion_texture` only for a packed ORM (a converted spec map's R is 1,
  no fetch); `metallic` 1, `perceptual_roughness` 1, `reflectance` 0.5 (F0 0.04, Wareya's `f0_scalar`). No ORM: the
  material is unchanged (0 / 0.85 / 0.3). Unlit: none. Actors: not yet (`cooked_material` passes no companions).
- Pack defect (§7, owner question 7, pending): `Quirk::FillEmptyLevel0 { role: Orm, channel: B }` in a profile's
  `quirks` (off in the shipped ron): level 0's metal rebuilt from level 1 when level 0 has none and level 1 some, BC1
  only, block-constant (BC1 shares one index set across channels; the rebuilt mask has level 2's resolution), other
  channels bit-exact. Counted `level 0 rebuilt` in the `texture.cook:` line.

**Numbers.** Exact roughness at A = 0, 2, 14, 126, 255: 255, 214, 152, 90, 76 (`r` 1.0 … 0.297). MVR `_spec`: 3 390
files, all → BC1 by rewrap (cost: IO, §7 budget); TPBR BC7 → BC7 by field swap. Converted vanilla spec: decode + BC1
encode (ISPC, cheap; no census of such packs yet). Tests: `mw_bake/src/cook/tests.rs` (`plan_orms_…`, `orm_companions_…`,
`level0_metal_…`), `sf_texture` `profile/tests.rs`, `remap/tests.rs`, `cook/tests.rs`, `mw_assets/src/cooked.rs`.
