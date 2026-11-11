# Texture compression, mips and streaming: research (2026-11-11)

Session `textures` (research + spec, no code). Feeds `superfargoth/docs/specs/textures.md` and
`bevyrrowind/docs/specs/textures-adapter.md`. Owner question answered here: "Bevy has native texture streaming,
no?" **No.** Evidence in §1. Items marked *(unverified)* were not confirmed from source or a primary doc.

## 1. What Bevy 0.20 has (read from source: cargo registry `bevy_image-0.20.0`, vendored `bevy_render`, `bevy_pbr`, `bevy_core_pipeline`)

| Piece | Bevy 0.20 | Evidence |
|---|---|---|
| Texture / mip streaming, residency under a budget | **none** | No streaming, residency or budget code in `bevy_image`, `bevy_render/src/texture`, `bevy_pbr`. `GpuImage::prepare_asset` (`vendor/bevy_render/src/texture/gpu_image.rs:114-235`) uploads the whole `Image` (`create_texture_with_data`, every level at once) or, when the descriptor is unchanged and the texture has `COPY_DST`, rewrites it in place with **one** `write_texture` sized for mip 0 only (upstream bug, fixed after 0.20 by [bevy#26320](https://github.com/bevyengine/bevy/pull/26320), merged 2026-10-27, no milestone). No partial-mip upload API. |
| Virtual texturing | **none** | Open feature request [bevy#25986](https://github.com/bevyengine/bevy/issues/25986) "Software Virtual Texturing" (2026-10, prototype by the author, no PR, 0 comments). wgpu/WebGPU have no sparse (tiled) resources, so hardware VT is out (stated in the issue). |
| Per-frame upload throttle | `RenderAssetBytesPerFrame` / `RenderAssetBytesPerFrameLimiter` (`vendor/bevy_render/src/render_asset.rs:398, 492`) | Whole-asset granularity. **We keep it off**: in 0.20 a mesh instance collected while its upload is deferred caches a stale slot (streaming.md "Upload", far-field.md "Bevy 0.20 pitfalls"). |
| Mip generation | CPU: none at load (TGA/PNG load with 1 level). GPU: `bevy_core_pipeline::mip_generation` (FidelityFX SPD) for storage-capable **uncompressed** formats only (`DownsamplePipeline::supports_texture_format`) | Open issue [bevy#18037](https://github.com/bevyengine/bevy/issues/18037) "Runtime mipmap generation"; maintainers point users at the asset processor. Useless for BCn. |
| Offline compression | `CompressedImageSaver` (asset processor; `bevy_image/src/compressed_image_saver/`): features `compressed_image_saver` (via `ctt` 0.5, C++ compiler) or `compressed_image_saver_universal` (`basis-universal`) | Format table: 1-ch → BC4, 2-ch → BC5, `is_normal_map` → BC5, LDR RGBA → BC7, full mips. Runs inside Bevy's `AssetProcessor` (writes `imported_assets/`), not inside our content-addressed bake, and needs one input file per asset path: doesn't fit "everything is a mod" through `sf_vfs` + `sf_bake`. |
| Containers | KTX2 (`ktx2` + zstd via `zstd_rust`), DDS (`ddsfile`, BC1-7 incl. DX10 header; BCn stays compressed), TGA/PNG/BMP (decoded to RGBA8) | `bevy_image/src/ktx2.rs`, `dds.rs`. The KTX2 loader reads the whole file (issue [bevy#22676](https://github.com/bevyengine/bevy/issues/22676), selective layer loading: maintainers prefer one file per texture). |
| Two-channel normal maps | **yes**: `StandardMaterial` sets `TWO_COMPONENT_NORMAL_MAP` for `Rg8Unorm`, `Rg16Unorm`, `Bc5RgUnorm`, `EacRg11Unorm`, ASTC (`vendor/bevy_pbr/src/pbr_material.rs:1163-1187`); shader reconstructs `z = sqrt(1 - x² - y²)` (`render/pbr_functions.wesl:191-194`) | Uniform flag, no pipeline key split. |
| Normal-map Y convention | Bevy (glTF) = OpenGL/+Y. `flip_normal_map_y` for DirectX maps (`pbr_material.rs:419-421`, uniform flag) | |
| Normal maps need tangents | `ATTRIBUTE_TANGENT` (MikkTSpace via `Mesh::generate_tangents`). Our cooked meshes carry tangents only on the Solari path (`sf_model_bevy/src/loader.rs:298-338`) | `matclass.rs:20-22` notes it. |
| Parallax | `depth_map` + `parallax_depth_scale` (POM/relief), costly | `pbr_material.rs:700-742`. |
| Sampler LOD / bias | `ImageSamplerDescriptor { lod_min_clamp, lod_max_clamp, mipmap_filter, anisotropy_clamp }` per image; view-level `MipBias` (TAA/DLSS) | `bevy_image/src/image.rs:860-965`. Our model textures use `ImageSamplerDescriptor::linear()` (anisotropy 1) in `mw_assets::image_settings_clamped`. |
| Re-binding after an image reallocates | **not handled** | Materials re-prepare only on their own `AssetEvent` (`vendor/bevy_render/src/erased_render_asset.rs:300-345`); `bevy_pbr`/`bevy_material` hold no `AssetEvent<Image>` reader. An `Image` whose descriptor changes gets a new `wgpu::Texture` and view; a material prepared earlier keeps the old view until the material itself is modified. (Read from source; no upstream issue found, search *(unverified: GitHub search returned nothing, maybe rate-limited)*.) |
| VRAM accounting | none in Bevy; wgpu 30 has `Device::generate_allocator_report` and `get_internal_counters` (`wgpu-30.0.1/src/api/device.rs:620-630`) | Backend-dependent *(unverified on DX12/Vulkan output detail)*. |

Conclusion: Bevy 0.20 gives containers, BCn upload, two-channel normal reconstruction and an offline saver.
Streaming (choosing and changing resident mips) is engine work, as WISHLIST §10 already says.

## 2. Ecosystem crates (crates.io API 2026-11-11; licences against `deny.toml`: MIT/Apache/BSD/Zlib OK)

| Crate | Version (updated) | Licence | What | Verdict |
|---|---|---|---|---|
| `intel_tex_2` | 0.5.0 (2025-07) | MIT/Apache-2.0 | ISPC kernels: BC1, BC3, BC4, BC5, BC6H, BC7, ETC1, ASTC (CPU, prebuilt ISPC objects) | **adopt** (already ours, `sf_texture::ktx2_out`): BC7 + new BC4/BC5 (`bc4::compress_blocks_into`, `bc5::compress_blocks_into`) |
| `block_compression` | 0.11.0 (2026-10-24) | MIT | WGSL port of the ISPC kernels, BC1-7 incl. BC6H; GPU (wgpu **30** = Bevy 0.20's) plus CPU `encode::compress_rgba8` and `decode` | **adopt**: CPU decode for tests and missing-mip fill (already a dev-dep); GPU BC7 as the bake lever for big uncompressed packs (measure first; DX12 BC7 pipeline compile is slow, crate docs) |
| `ktx2` | 0.5.0 (2026-04) | Apache-2.0 | KTX2 reader (+ our writer in `ktx2_out`) | **adopt** (in tree) |
| `ddsfile` | 0.6.0 (2026-04) | MIT | DDS parse/compose, legacy FourCC `ATI1`→BC4, `ATI2`→BC5 (`format/dxgi.rs:624-633`) | **adopt** (in tree) for DDS → KTX2 rewrap |
| `texpresso` | 2.0.2 (2025-05) | MIT | pure-Rust BC1-5 encode/decode | keep for the existing BC1-3 decode; no BC7 → not the encoder |
| `bcdec_rs` | 0.2.1 | MIT | BC1-7 decode | alternative to `block_compression::decode`; not needed |
| `image_dds` | 0.7.2 (2025-03) | MIT | DDS ↔ image with BCn (wraps intel_tex/bcdec) | reject: duplicates what we have |
| `ctt` | 0.6.1 (2026-10-28) | MIT/Apache/Zlib | wraps bc7enc-rdo, Compressonator, ISPC, astcenc; mip filters incl. alpha cutoff | reject for now: needs a C++ toolchain on every build machine (compile-time constraint), and we already have ISPC; revisit if BC7 quality from ISPC "basic" is judged short |
| `basis-universal` | 0.3.1 (2023-11) | MIT/Apache | UASTC/ETC1S supercompression + transcode | reject: we ship desktop BCn only; transcode costs load time, and stale (2023) |
| `bevy_mod_mipmap_generator` | 0.2.0 (2026-08) | MIT/Apache | CPU mips + optional BC7 at runtime | reject: Bevy 0.19, runtime work ("nothing bakes during play") |
| Bevy texture streaming / VT crates | none found | | crates.io search "texture streaming", "virtual texture", "mip streaming", "bevy texture" | nothing to adopt; `martensite-assets` mentions "dynamic texture streaming" but is not Bevy *(unverified scope)* |

## 3. Industry practice

**Formats (what modern PC games ship).** BC7 for colour (sRGB) and colour+alpha, BC1 where size beats quality
(far, opaque), BC5 for tangent-space normals (two channels, Z reconstructed), BC4 for single masks (roughness, AO,
height), BC6H for HDR. Reference: Nathan Reed, [Understanding BCn texture compression formats](https://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/)
(2012); Microsoft [Texture block compression in Direct3D 11](https://learn.microsoft.com/en-us/windows/win32/direct3d11/texture-block-compression-in-direct3d-11).
Same as Bevy's own `CompressedImageSaver` table (§1). Skyrim SE moved to D3D11 and accepts BC7 *(unverified primary
source)*; Morrowind content is DXT1/DXT3/DXT5 DDS plus some TGA (MGE XE research note).

**Mips.** Box or Kaiser filter in linear space for colour; renormalise normals per level (and fold lost length
into roughness: Toksvig / "LEAN"-lite, used by UE as "composite texture" and by Frostbite *(roughness folding:
later, not v1)*); alpha-tested textures keep coverage per level (Castaño, "Computing alpha mipmaps", 2010; our
`preserve_coverage` already does it).

**Streaming.**
- **UE texture streaming** ([overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/texture-streaming-overview-for-unreal-engine),
  [build data](https://dev.epicgames.com/documentation/en-us/unreal-engine/building-texture-streaming-data-in-unreal-engine)):
  each primitive lists its textures with world bounds and texel world size (mesh UV density built offline, material
  texture scales built per material); the streamer computes the resolution for ~1 texel per pixel, plans against the
  pool, and when over budget "drops one mip at a time", off-screen and least-recently-seen first; visible mips load
  first; work runs on an async thread with a temporary in-flight pool. Missing build data → UV0, scale 1,
  conservative heuristics. On APIs without sparse textures UE reallocates the texture with the new mip count and
  copies the resident levels *(unverified detail: `RHIAsyncReallocateTexture2D` in UE4 source)*.
- **Unity mipmap streaming** ([manual](https://docs.unity3d.com/2022.3/Documentation/Manual/TextureStreaming.html)):
  a `Memory Budget`; levels from renderer bounds plus mesh UV distribution metrics computed at mesh import; `Max
  Level Reduction` (top mips skipped on first load, cap on later drops, beats the budget); per-texture `Priority`
  acting as a mip bias; non-streaming textures always full; debug view colours reduced / starved / not streaming.
- **Virtual texturing**: id Tech 5 MegaTexture (Rage, 2011), Far Cry 4 adaptive VT (GDC 2015), Frostbite terrain
  VT, UE runtime/streaming VT. Feedback pass + page table + physical cache. Pays off for unique-texel terrain and
  huge atlases. Our terrain is tiled layers blended in shader (terrain.md), not unique texels, so VT buys little now.
- **Bethesda / OpenMW**: no mip streaming; whole textures load with their cells and drop with them; HD packs ride
  on VRAM. That is also us today (Bevy handles drop with the cell).

**Containers built for streaming.** KTX 2.0 stores level data smallest-first so a reader can stop early, and zstd
supercompression is per level ([KTX 2.0 spec](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html), "Level
Index", "Mip Level Array" *(order recalled from the spec, re-check when building S1)*).

## 4. Morrowind normal-map packs

- **OpenMW** ([texture basics](https://openmw.readthedocs.io/en/latest/reference/modding/texture-modding/texture-basics.html)),
  `[Shaders]` settings, all off by default: `auto use object normal maps`, `auto use object specular maps`,
  `auto use terrain normal maps`, `auto use terrain specular maps`; `normal map pattern = _n`
  ("Also used for terrain normal maps"), `normal height map pattern = _nh` (alpha = parallax height),
  `specular map pattern = _spec` (RGB colour, alpha shininess 0-255), `terrain specular map pattern =
  _diffusespec` (specular in the layer texture's alpha). Quote: "OpenMW uses the DX format for normal maps, and not
  the OpenGL format as one otherwise might expect." Two-channel ("red-green") formats are supported, Z
  reconstructed (so no parallax with them).
- **Older packs** ([converting bump-mapped mods](https://openmw.readthedocs.io/en/latest/reference/modding/texture-modding/convert-bump-mapped-mods.html)):
  `_nm.dds` names (rename to `_n`), `_reflection.dds` (rename to `_spec`), NIF `NiTextureEffect` / bump slots to
  drop; MGE XE's normal maps were an abandoned experimental PBR build keyed by text files (forum hearsay
  *(unverified)*).
- **Our code today**: `mw_nif::Material` reads only the base and glow slots (`mw_nif/src/material.rs:249-281`, the
  bump slot is parsed but unused); `mw_assets::standard_material` never sets `normal_map_texture`; every model
  texture loads with `is_srgb = true` (`mw_assets/src/lib.rs:498-510`), which would be wrong for a normal map;
  `mw_bake::delight` already knows the `_n`/`_nh`/`_spec` companions and **skips de-lighting any albedo that has
  one** (`delight.rs:29, 68, 239-246`), so a normals-only pack would silently turn de-lighting off for the vanilla
  albedos it covers. Terrain: `TerrainLayer.normal` exists (terrain.md) but the adapter sets none
  (world-adapter.md), and the layer arrays are RGBA8 (`sf_terrain_bevy/src/lib.rs:418-419`) although terrain.md
  says BC7/BC5.

## 5. Costs we know

- BC7 via ISPC: 318 ms per 512² "basic", 128 ms "fast" (bake.md levers). 3521 de-lit textures: 34 s wall, 395 CPU-s.
- BC4/BC5 via ISPC: much faster than BC7 (simpler mode search) *(unverified number; T1 prints it)*.
- 2048² BC7 or BC5 with mips: 5.33 MiB; RGBA8: 21.3 MiB. 1024²: 1.33 MiB vs 5.33 MiB.
- An HD pack of 3000 albedo + normal at 2K, all resident: ~32 GiB as BCn: streaming (or a size cap) is mandatory;
  with cell residency (~600 textures live) and a 1K cap: ~1.6 GiB.
- Re-encoding 3000 uncompressed 2K albedos to BC7 "basic" on the CPU: ~15000 CPU-s (~17 min on 15 threads): over the
  5 min ceiling; already-BCn DDS rewraps in milliseconds. Vanilla census (format mix, how much is TGA) is unmeasured:
  slice T0.
