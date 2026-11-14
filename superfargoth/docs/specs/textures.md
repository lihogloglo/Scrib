# Textures: cook, residency, normal maps

> Status 2026-11-11: **T1, T2, T2b built** (`sf_texture::cook`, `::rewrap`, `::remap`, `::profile`; §9 "as built"); the rest is design. De-light: bake.md "De-lighting". Research:
> [texture-compression-streaming-2026-11-11](../research/texture-compression-streaming-2026-11-11.md). Morrowind
> adapter: `bevyrrowind/docs/specs/textures-adapter.md`. Owner: "quite urgent" (a normal-map pack is coming).
> Read: §1, §5 for the bake step, §6 for streaming, §7 for normal maps, §9 for slices.

## 1. Goal and non-goals

- **Goal.** Every texture the renderer samples is block-compressed with a correct mip chain, cooked once in the
  setup bake through the VFS (mods override by winning the path); resident mips follow screen-space demand under a
  per-preset VRAM pool, so HD and normal-map packs fit an 8 GB laptop; normal maps from packs light statics,
  terrain and characters.
- **Never**: a texture visibly sharpening inside the reveal radius after a reveal (pipeline-warmup.md §5.4); a
  bake during play; render-world injection (CONTRIBUTING "Patch Bevy, don't inject").
- **Non-goals (v1)**: virtual texturing (terrain is tiled layers, not unique texels: §3), runtime GPU compression,
  PBR map generation from albedo (WISHLIST §10, M7), parallax and specular maps (§7.4, owner question).

## 2. Canonical technique

1. **Offline BCn per role** (UE/Unity texture compression settings, Bevy's own `CompressedImageSaver` table):
   colour → BC7 sRGB; colour+alpha → BC7 sRGB with coverage-preserving mips; normal → BC5 (X, Y; Z rebuilt in the
   shader); single mask → BC4; already block-compressed sources with a full chain are **rewrapped, not
   re-encoded** (a second lossy pass only loses quality and costs time).
2. **Mips with the right filter**: linear-space box filter for colour; per-level renormalised vectors for normals;
   coverage preservation for alpha tests (`sf_texture::preserve_coverage`, Castaño 2010).
3. **Mip residency under a pool** (UE texture streaming, Unity mipmap streaming): per texture a *wanted* level from
   the nearest demand's distance and the mesh's UV density (≈ one texel per pixel), a budget planner that drops one
   level at a time from the textures with the most slack, a per-frame upload budget, and a mip tail that never
   streams. On APIs without sparse textures (wgpu) a residency change **reallocates** the texture with the new level
   count and uploads it (UE's realloc path).
4. **Normal maps as companions** found by name next to the albedo (OpenMW's `_n` pattern), linear, converted to the
   engine's tangent convention at cook, tangents from MikkTSpace baked with the mesh.

## 3. Industry comparison

| Engine | Formats | Residency | Demand | Over budget | Us |
|---|---|---|---|---|---|
| UE5 | BC7/BC5/BC4/BC1, per texture group | streaming pool, mip tail never streams | primitive bounds + baked UV density + material scales, 1 texel/px | drop one mip at a time, off-screen and least-recently-seen first | **match** (§6), cell-cluster bounds instead of per-primitive |
| Unity | same | `Memory Budget`, `Max Level Reduction` | renderer bounds + mesh UV distribution metric (import time) | discard unused, then load lower | **match**; our S1 size cap = `Max Level Reduction` |
| id Tech 5-7, Far Cry 4, Frostbite terrain | BC + transcoded pages | virtual texturing, page cache | GPU feedback pass | lower pages | **not adopted**: no unique-texel content; revisit for terrain VT (WISHLIST §10) |
| OpenMW / Skyrim | DDS as shipped | whole textures with cells | none | none (VRAM decides) | **better**: cook + residency |
| Bevy 0.20 | uploads what loads; offline saver in the asset processor | none | none | none | engine work (§4) |

Gaps logged in `industry-audit.md` row 23 (residency) remain open until S4; roughness folding of normal-map
variance (Toksvig) and anisotropic filtering are §11 items.

## 4. Reuse check (CONTRIBUTING "How we decide" 0-2; detail in the research note §1-2)

| Piece | Bevy 0.20 offers | Crates (version, licence, Bevy 0.20 fit, maintenance) | Decision |
|---|---|---|---|
| BCn encode, CPU | `CompressedImageSaver` via `ctt` (C++ toolchain) or `basis-universal`, only inside `AssetProcessor` | `intel_tex_2` 0.5 (MIT/Apache, ISPC prebuilt, BC1/3/4/5/6H/7; in tree); `block_compression` 0.11 CPU path (MIT); `texpresso` 2.0 (MIT, no BC7); `ctt` 0.6 (MIT/Apache/Zlib, C++); `image_dds` 0.7 (MIT, wraps the same kernels) | **`intel_tex_2`** (already ours; adds BC4/BC5). The asset processor doesn't fit `sf_bake` (content-addressed, VFS winners, one plan) |
| BCn encode, GPU | none | `block_compression` 0.11 (MIT, wgpu 30 = ours, maintained 2026-10) | **adopt behind a measurement** (T8) as the lever for big uncompressed packs; owner OK'd GPU on the setup screen |
| BCn decode (missing-mip fill, tests) | none on CPU | `block_compression::decode` (in tree as dev-dep), `bcdec_rs`, `texpresso` (BC1-5, in tree) | `texpresso` BC1-5 (+ its BC1 punch-through / BC2 encode, which ISPC lacks), `block_compression::decode` BC7 (T2) |
| KTX2 / DDS IO | loaders for both; KTX2 reads the whole file | `ktx2` 0.5 (Apache), `ddsfile` 0.6 (MIT, legacy `ATI1`/`ATI2` mapped) | **reuse both**; writer is ours (`sf_texture::ktx2_out`) |
| Mip generation | GPU SPD for uncompressed storage formats only; none at load | `bevy_mod_mipmap_generator` 0.2 (Bevy 0.19, runtime) | **ours at bake** (`sf_texture::mip_chain`, extended for normals) |
| Normal-map handling | BC5/RG two-channel detection, `flip_normal_map_y`, MikkTSpace `generate_tangents` | `bevy_mikktspace` (Bevy-free, MIT/Apache) for bake-time tangents | **reuse Bevy's material path as-is**; tangents baked in `mesh.cook` |
| Streaming / residency | none (whole-image upload; `RenderAssetBytesPerFrame` is whole-asset and kept off) | none on crates.io for Bevy | **ours**: Bevy-free planner + thin plugin on public APIs (`Assets<Image>` replace, material touch) |
| Rebind after reallocation | not handled (materials re-prepare only on their own events) | none | **ours, main world** (S2): material reverse index + `Assets::get_mut` touch; offer upstream as an issue + PR, carry as a vendored patch only if the touch proves costly |
| In-place multi-mip update | buggy in 0.20 (mip 0 only) | upstream [bevy#26320](https://github.com/bevyengine/bevy/pull/26320) (merged after 0.20) | not needed by our realloc path; carry as a vendored patch only if a slice updates in place |
| Virtual texturing | none; [bevy#25986](https://github.com/bevyengine/bevy/issues/25986) open, no PR | none | **not now** (§3); watch #25986 |

## 5. Part A: `texture.cook` (bake step)

**Roles** (framework enum, adapter maps its slots to them):

| Role | Source | Output format | Mips | Notes |
|---|---|---|---|---|
| `Color { alpha: None }` | albedo, glow, detail | BC7 sRGB; BC1 sRGB kept as rewrap when the source is BC1 | box, linear space | de-light is a pre-pass for albedo (below) |
| `Color { alpha: Test(cutoff) }` | cutout albedo | BC7 sRGB | box + `preserve_coverage(cutoff)` | cutoff from the adapter (Morrowind: `texture.roles`) |
| `Color { alpha: Blend }` | translucent, effects | BC7 sRGB (alpha profile) | box, premultiply-free | |
| `Normal { source_y: Up \| Down }` | `_n` companions | BC5 unorm (R = X, G = Y, engine +Y convention) | decode to vectors, box filter, renormalise per level | `Down` (DirectX) sources get G inverted at cook, so the runtime never sets `flip_normal_map_y` |
| `NormalHeight` | `_nh` companions | BC5 normal + BC4 height | as `Normal`; height box | height is only cooked when §7.4 is approved |
| `Mask { channel, invert }` | roughness, metalness, AO, spec intensity, height (`invert`: Bevy depth), gloss (`invert`: roughness) | BC4 unorm | box | one channel of the source |
| `Packed` | ORM (R AO, G roughness, B metalness: glTF/Bevy layout), other packed masks | BC7 unorm | box, every channel alike (no alpha weight, no sRGB) | §11 decision 1 |
| `Hdr` | none today | BC6H | | reserved |
| `Ui` | menus, splash, cursors | untouched (as shipped) | | sampled 1:1, out of scope |

**Rewrap rule.** A source that is already BCn (DDS BC1-7, legacy `DXT1-5`, `ATI1`, `ATI2`; KTX2 BCn) is rewrapped
into KTX2 without re-encoding when its format suits the role (BC1/BC2/BC3/BC7 for colour, vanilla DXT3 included; BC5 or BC1/BC3 for normals,
since packs often ship three-channel BC1/BC3 normals: kept, Bevy reads RGB normals). Missing mip levels are filled:
decode the smallest shipped level, `mip_chain`, encode only the missing levels in the same format. A shipped
level-0 size that isn't a whole number of blocks is a skip (served as shipped, counted).

**Composition with de-light.** De-light stays its own step (`texture.delight`, bake.md): it decides which albedos
are de-lit and its product *is* the cooked texture for them. `texture.cook` plans every referenced texture the
de-light index doesn't cover. One runtime layer serves both (the de-light layer generalised, §5 "Serving"). A later
merge into one step is optional and changes no product.

**Data model** (bake.md "Data model"):
- Subject: the winning VFS path. Inputs: the file (fingerprint), the role set of the path (from the adapter's roles
  product; a path used in two roles cooks once per role, keyed by role), companion existence for `Normal` is the
  adapter's business, not cook's.
- Key = (step `texture.cook`, `VERSION`, subject, role, params `{ quality: Basic|Fast, source_y }`, input
  fingerprints). The de-light strength is **not** in the cook key.
- Product: KTX2 bytes, stored **without** the store's LZ4 (BCn compresses poorly with LZ4; KTX2 zstd per level is
  measured in T0 and adopted only if the ratio beats 15 %). A constant skip marker for unfit sources (counted by
  reason, as de-light does).
- `texture.cook.index` (one): per cooked texture `{ source, fingerprint, ktx2 digest, role, format, width, height,
  levels, level_bytes[] }`. The level table lets the streamer (§6) plan and read levels without opening files.
- **Authored beats derived**: a winner that is already a KTX2 with BCn and a full chain is indexed as-is (no product
  bytes, metadata only).

**Serving.** `CookedTextureLayer` (`sf_vfs` top layer, generalising `mw_bake::delight::DelightLayer`): serves
`<stem>.ktx2` for an index entry only while its source is still the winning file with the fingerprint the bake saw
and the only variant of its stem. A mod's newer file changes the winner or fingerprint, the stale entry is not
served, the mod's file loads as shipped, and the next start's incremental bake cooks it. `--off texcook` A/B.

**Budget.** Vanilla + expansions target **5 s** (mostly rewraps; T0 measures the mix). Per format, measured in T1
and printed: encode only, ms per MPix (T1, `measure_encode_per_format`, release): BC7 basic 936, fast 361, BC5 1.3, BC4 0.6; whole cook incl. mips: colour 1255, normal 24, mask 12; rewrap *(target
< 1 ms per file)*. Mod packs scale it: uncompressed 2K packs exceed the 5 min ceiling on CPU BC7 (research §5);
the levers, in order, are GPU BC7 (`block_compression`, T8), the Fast profile, and the setup screen's estimate.

## 6. Part B: residency (streaming)

**Units.** A *streamable* texture has level 0 over 512² (BC7 512² + chain = 341 KiB); smaller ones and every
texture's levels at or below 128² (the *mip tail*) are always resident. Vanilla Morrowind is mostly 256²-512²,
so vanilla runs with no streaming at all; residency engages for HD packs.

**Wanted level** (UE's texel-per-pixel rule), per texture, each update:
`px_per_m(d) = H_out / (2 · d · tan(fov_y / 2))`, `texels_per_m = ρ · W` (ρ = UV units per metre of the densest
demand using it, W = level-0 width), `wanted = clamp(floor(log2(texels_per_m / px_per_m(d))) + bias_preset, 0,
tail)`. `d` = distance from the camera to the demand's bounding sphere (0 inside), using the streaming scheduler's
effective distance `min(dist(now), dist(now + v·t))` (streaming.md "Priority") so upgrades run ahead of travel.
`H_out` is the output height (DLSS samples at output resolution with a negative view `MipBias`).

**Demand.** A framework interface, fed by whoever draws:
- *Static scene*: a baked `texture.demand` product per cell: clusters of ≤ 16 m radius, each `{ sphere, textures:
  [(id, ρ)] }` (ρ from `mesh.uv_density`, per-material UV density read from `mesh.cook`, UE's "mesh UV density"). Resident cells contribute.
- *Dynamic*: a `TextureDemand { textures, radius }` component on actors and items, set by their spawner.
- *Fallback* (no data): ρ = 1 UV per metre, distance to the owning cell: conservative (Unity/UE fallback).

**Planner** (`sf_texture::residency`, Bevy-free, deterministic, 4 Hz). Input: per texture (level bytes, wanted,
resident, last-wanted time, in-reveal-radius flag) and the pool size. Output: a target level per texture and an
ordered request list. Rules: target = wanted; while the sum exceeds the pool, raise by one level the texture with
the largest `(target − wanted, then least recently wanted, then bytes)` that is not in the reveal radius; then
textures in the radius, farthest first, logging `warn!` once per episode (the pool is too small for the preset).
Downgrades free memory before upgrades spend it. Hysteresis: an upgrade needs `wanted` stable for 0.5 s; a
downgrade waits 5 s (UE keeps unwanted mips cached while the pool allows).

**Upload.** The plugin (`sf_texture_bevy`) reads the needed levels from the store on the IO pool (the level table
gives offsets), builds a new `Image` (level-0 size = first resident level, `mip_level_count` = resident levels,
block-aligned start levels only) and **replaces the asset under its existing handle** (`Assets<Image>::insert`):
Bevy re-prepares a new `GpuImage` (`create_texture_with_data`). The old texture drops when no bind group holds it.
Then every material using that image is touched (S2 reverse index) so its bind group picks up the new view. Budget:
a class below cell loads inside streaming's GPU upload budget (8 MiB per frame, streaming.md "Budgets"), ×20 in
burst mode behind fades. Bevy's `RenderAssetBytesPerFrame` stays off.

**Reveal gate.** `RevealGate` (pipeline-warmup.md §5.4) gains one input: textures with a demand inside the reveal
radius whose resident level is coarser than wanted (counted, not drawn late). Doors and loads hold the fade until
it is 0 or the cap fires (`warn!` with the texture ids). During play, upgrades happen at distances where one level
is below a pixel's difference by construction; the `mip_deficit_visible` counter (§8) proves it stays at 0 while
walking and running.

**Pool per preset** (proposal, owner question 3): Low 1 GiB, Medium 1.5 GiB, High 2.5 GiB, Ultra 4 GiB, and a
load-time size cap (S1) Low 512², Medium 1024², High 2048², Ultra none. A preset change re-plans; with S2 built
the change applies live, before it on the next load. **Size cap decided (S1, 2026-11-13, industry standard)**: an
absolute cap on level 0's larger side, like UE's texture-group `MaxLODSize` per device profile (Unity's relative
`globalTextureMipmapLimit` / "Max Level Reduction" would halve vanilla 256² on Low too); values as proposed: High keeps 2K
packs whole (the content's authored size), Medium = one level off 2K (UE scalability `TextureQuality` 1, `MipBias` 1), Low
512² = the vanilla size. Ultra uncapped (4K packs). The pool sizes stay a proposal until S3/S4.

**Not in v1**: GPU-side copy of resident levels (we re-upload the lower levels: +1/3 of the top level's bytes,
no render-world code), terrain layer arrays (fixed size per preset, terrain.md), far-field atlases (fixed).

## 7. Part C: normal maps from packs

**Rule (owner 2026-11-11): the renderer is generic, never tuned to one pack.** Runtime materials use one canonical
layout (Bevy `StandardMaterial` slots: sRGB base colour, linear normal in Bevy's Y convention, glTF ORM, depth =
1 - height). Each pack's conventions (suffixes, channel order, Y, height neutral, known defects) are *data*: a pack
profile in the adapter or the mod's manifest, converted to the canonical layout at bake. No renderer code path or
shader constant exists for a specific pack; a pack without a profile falls back to the game's default convention.
The mod side is `sf_mods::TextureSection` (`mod.toml` `[texture]`: `profile` by the adapter's name, `normal_y`,
renamed `suffixes` via `PackProfile::alias_suffix`); the adapter resolves it per VFS source (Morrowind:
textures-adapter.md §8).
1. **Discovery** is the adapter's (naming differs per game): for an albedo path it returns companion paths per role
   (`Normal`, `NormalHeight`, `Mask`), resolved through the VFS with the usual alternates, so a pack is just a mod.
2. **Colour space**: normal maps are linear. Cooked: `BC5_UNORM`. Served as shipped (no cook yet): the loader must
   set `is_srgb = false` for the normal slot (today every model texture loads sRGB).
3. **Y convention**: the framework's convention is Bevy's (+Y, OpenGL/glTF). The adapter declares the content's
   default `source_y`; a mod manifest may override it per mod (`texture.normal_y = "down"`); cook inverts G.
4. **Height and specular**: `_nh` alpha height maps to Bevy's `depth_map` (`1 − height`, parallax occlusion,
   costly per pixel; owner q5: per preset, off on Low; built as T4d, §9) and a specular companion to Bevy's
   specular/roughness inputs. **POM checks** (T4d, 2026-11-13). Bevy 0.20 (vendored `bevy_pbr`): `depth_map` (white =
   deep), `parallax_depth_scale` (UV-relative), `max_parallax_layer_count`, `ParallaxMappingMethod::{Occlusion, Relief
   { max_steps }}` (Relief is a pipeline-key bit, Occlusion is not); `parallaxed_uv` (`parallax_mapping.wesl`) marches
   `mix(max, 1, |Vt.z|)` layers, sampling the depth map at level 0 always (`textureSampleLevel(.., 0.0)`), no distance
   fade; only the main pass parallaxes (`pbr_fragment.wesl:140`): the prepass (alpha discard, normal prepass), shadows
   and depth use the plain UV, no pixel-depth offset, silhouettes stay the mesh's; needs `VERTEX_TANGENTS`. Crates:
   none (POM is a shader; Bevy has it). Industry: UE's POM material function (min steps looking down, max at glancing
   angles, 32 the debug view's reference), Unity HDRP pixel displacement (min/max steps, fade by mip level), OpenMW
   0.4x single-step offset `h × 0.04 − 0.02` for the same `_nh` maps (`files/shaders/lib/material/parallax.glsl`,
   `4e718b5`). Gaps vs industry (no distance/mip fade, no depth offset): `industry-audit.md`.
5. **Materials**: statics and actors through the adapter's material builder (`normal_map_texture`, linear sampler,
   matclass `Split::NormalMap` already exists); terrain through `TerrainLayer.normal` (terrain.md; layer arrays
   become BC7 + BC5 in T6). A material with a normal map needs tangents: `mesh.cook` generates MikkTSpace tangents
   (`bevy_mikktspace`) for submeshes whose material has a normal companion; the companion's existence is a cook
   input, so adding a pack re-cooks only those meshes.

## 8. Measurement (log lines and counters only)

- Bake: `texture.cook: N cooked (bc7 a, bc5 b, bc4 c, rewrap d), skipped by reason, cold X s wall / Y CPU-s, per
  format ms/MPix` (stats, bake.md "Progress reporting").
- Runtime, every 5 s at `info!` behind `--diag textures`: resident bytes by format and in total vs pool, textures
  streamable / at wanted / starved, bytes uploaded this interval and the per-frame max, upload ms (main-thread
  `Image` build + insert), material touches, `mip_deficit_visible` (demands inside the view frustum with resident
  coarser than wanted). Plus wgpu `generate_allocator_report` totals where the backend provides them.
- A perf entry per preset (ms deltas) in `bevyrrowind/docs/perf.md` after S4.

## 9. Slices (one deliverable + its verify step each)

| # | Deliverable | Verify |
|---|---|---|
| **T1** | `sf_texture` cooks any role to KTX2 (below) | `cargo_turn.sh test -p sf_texture`; clippy pedantic |
| T0 | **Packs census done 2026-11-11**: suffix → slot → cook path, DirectX Y, height neutral 0.5, block-exact remaps (adapter §7; research `pbr-packs-census-2026-11-11.md`). Still open, folded into T3 stats: texture census (`mw_bake` census): per referenced texture role, container, format, size, levels, bytes, winning mod, companions; projected VRAM as shipped vs cooked | `#[ignore = "measurement: texture census"]` test printing totals; numbers into the adapter spec |
| T2 | `sf_texture::rewrap`: DDS/KTX2 BCn → KTX2 without re-encode, missing levels filled | exact: vkFormat per source format, level bytes identical to the DDS payload, filled levels' count and sizes |
| T2b | Block-exact remaps (`sf_texture::remap`) + generic pack profile (`sf_texture::profile`), profile instances in the adapter | exact: each op's output decodes to the mapped decode of its input; split products' levels; profile serde; suffix routing |
| T3 | `texture.cook` + `.index` steps and `CookedTextureLayer` (adapter: textures-adapter.md) | exact: role → format table, skip reasons, a changed winner re-cooks only that texture (trace test), stale entry not served; bake stats line |
| T4 | Normal maps on statics and actors: companions, linear sampler, tangents in `mesh.cook`, de-light companion rule (adapter) | exact: material gets `normal_map_texture` iff a companion exists, mesh has tangents iff its material has one, de-light still runs for normals-only packs; **visual check TX-1** for the owner |
| T4d | Parallax occlusion on statics from `_nh` height: BC4 depth companion at bake, `depth_map` + per-preset method in the material builder, `--off parallax` / `flip parallax` | exact: DXT5 split decodes `255 − alpha`, depth jobs per the companion's profile, index served / stale withheld, material gets `depth_map` iff a depth companion, preset → method, switch restores the material; **visual check TX-5** |
| T5 | Terrain normals: `TerrainLayer.normal` from companions | exact: layer record carries the path; visual check TX-2 |
| T6 | Terrain layer arrays BC7 + BC5 (terrain.md already specifies) | exact: array formats and sizes; VRAM line |
| T7 | Far-field atlases BC7 (bake.md gap 9; owner of far field coordinates) | exact: formats; VRAM delta printed |
| T8 | Measurement: GPU BC7 (`block_compression`) vs ISPC basic/fast, ms per MPix and PSNR on 50 vanilla textures | `#[ignore = "measurement: ..."]` printout |
| S1 | Our KTX2 loader (replaces Bevy's for `.ktx2`): per-preset size cap (top levels skipped), per-level zstd, counters | exact: a 2048² file under a 1024 cap yields 1024² with levels − 1; counters line |
| S2 | Rebind on reallocation: material reverse index, touch on `AssetEvent<Image>::Modified` | exact (no GPU): replacing an image marks every material using it modified, none else |
| S3a | `sf_texture::residency` planner (Bevy-free, deterministic) | exact: wanted levels for fixed inputs, tail/cap clamps, over-budget order, reveal radius last + one warn, hysteresis, determinism; measurement: time for 10k textures |
| S3b | `texture.demand` bake product + UV density (`mesh.uv_density`, reads `mesh.cook`) | exact: ρ of a known quad, clusters ≤ 16 m, products per cell |
| S4 | `sf_texture_bevy` streamer: demand → plan → IO → image replace under the upload budget; reveal-gate input | exact: plan executes within byte budget per frame in a headless app; **visual check TX-3**; perf entry |

**T1 as built** (2026-11-11): `ktx2_out::{BlockFormat, write_ktx2, level_len}`, `cook` (`Role`, `format_for`, mip chains,
`encode_bc4/bc5`, `cook_levels`); detail and the T1 plan: `archive/textures-history.md`.

**T2 as built** (2026-11-11): `sf_texture::rewrap(bytes, role) -> Rewrap { Rewrapped | Cook | Skip }` (DDS via `ddsfile`, KTX2
via `ktx2`; colour space follows the role; shipped levels unchanged, missing ones filled in the source format);
detail: `archive/textures-history.md`.

**T2b as built** (2026-11-11): `sf_texture::remap` (block-exact channel remaps, fallbacks `NotBlockExact` /
`FormatForRole`), `rewrap_mapped`, `sf_texture::profile` (`PackProfile`); detail: `archive/textures-history.md`.

**T3 as built** (2026-11-11): `texture.cook` + `.meta` + `.index` steps, paths in place / rewrapped / encoded / skipped,
one `CookedTextureLayer`, bake line `texture.cook:`; detail: `archive/textures-history.md`.

**T4 as built** (2026-11-11, three parts): bake-side companions (`mw_bake::cook::normal_companion`, BC5 / rewrapped
linear with the profile's Y flip, `cooked/texture_normals.postcard`), the runtime material (`cooked_material_with`), and
MikkTSpace tangents in `mesh.cook`/`mesh.rest`/`mesh.rig` (adapter hook `sf_model::steps::TangentRule`, `.sfmesh` v4).
Actors (sf_character assembly) don't carry tangents yet. Detail: `archive/textures-history.md` "T4".

**T4d as built** (2026-11-13, tx-pom; checks in §7 item 4). Bake: `mw_bake::cook::plan_normals` gives a winning
normal companion whose own source's profile reads it as `NormalHeight` (`_nh`, or a mod's alias of it) a `DepthJob`
(the rule's layout) and its bases a depth companion; step `texture.cook.depth` (v1, subject the `_nh` file, which
`texture.cook` still cooks as the normal) runs `cook_depth`: `rewrap_mapped(.., Role::height(A))` (DXT5: the alpha
block is the BC4 block, complemented, block-exact; BC7 / uncompressed: decode + `cook_levels` BC4), then
`level_cap::cap_ktx2` to `DEPTH_MAX_SIZE` 1024 (top levels dropped: Bevy samples the depth map at level 0 at any
distance, and its docs warn large depth maps cost disproportionately; height is low-frequency). `.depth.meta` (v1);
`texture.cook.index` v4 adds `depth_entries` + `depths` (base stem → served path `<nh stem>_depth.ktx2`);
`texture.cook` stays v3 (its products didn't change, so no re-cook). `CookedTextureLayer` serves current depth
maps and `cooked/texture_depths.postcard` (`read_depths`, `depth_of`), withholds stale ones with their companion;
`LayerStats.depths`. Lines: bake `texture.cook: ... N normal companions (D with depth); ...` (depth maps count in
textures / formats / VRAM, `built` includes them); runtime `texture.cook: layer mounted, ..., D depth companions`.
Material (`mw_assets`): `TextureCompanions::depth_of`, `Surface::depth_texture`; `to_material` binds the depth map
(linear, the base slot's clamp) with `map_ab::parallax()`, the preset's (`mw_game_core::Quality::parallax`: Low none,
not loaded; Medium `Occlusion` 16 layers; High `Relief { 5 }` 16 layers, Bevy's defaults; Ultra `Relief { 8 }` 32
layers, UE's reference count), `parallax_depth_scale` 0.04 for every pack (OpenMW's `PARALLAX_SCALE` for the same
maps on the same UVs; both packs author for that shader, so no per-pack value; Bevy's 0.1 "may look distorted"). Bevy
puts the polygon at depth 0 (the top), so a 0.5-neutral height sinks the mean surface by 0.02 UV (OpenMW centres it
with its −0.02 bias; Bevy has no bias input: accepted, `height_neutral` stays unused). Matclass: an `Occlusion`
depth map folds into its class (data only); `Relief` is a `Split::DepthMap` (key bit), and these materials already
split on `NormalMap`, so High adds no class. `sf_world_bevy::fade` waits for the depth map too. Switch:
`--off parallax`, console `flip parallax`, F1 "Textures" line, texture lab key **L** (P is the clock pause
everywhere); `MaterialMaps.parallax` strips `depth_map` and resets scale / method / layers to Bevy's defaults (the
key matches a material without a companion), restores them exactly. Lab panel: `depth: <file> <image> bound
<method> <layers> layers, scale <s>` or why none. Tests exact: `cook::tests::{dxt5_height_splits_into_a_complemented_
bc4_depth_map, depth_maps_are_capped_by_dropping_top_levels, depth_jobs_follow_the_companions_profile,
layer_publishes_current_depth_companions}`, `cooked::tests::depth_map_iff_the_base_texture_has_a_depth_companion`,
`map_ab::tests::{flipping_off_then_on_restores_identical_materials, parallax_settings_pick_bevys_method}`,
`exterior::tests::preset_picks_the_parallax_method`. **Cost** (no measurement yet; the owner A/B bench comes later,
`--off parallax`): per shaded pixel of a depth-mapped surface, `1 + layers` depth fetches (layers = 1 head-on up to
16/32 at grazing angles) + 1 (Occlusion) or + 5/8 (Relief), all at level 0 of a ≤ 1024² BC4. Unverified estimate (no measured source found; UE and HDRP document steps, not ms): commonly quoted figures put
16-32-step POM at roughly 0.3-1.5 ms at 1080p for full-screen coverage on mid-range GPUs (scales with covered
pixels × steps, worst on grazing ground); expect well under 1 ms at 1440p + DLSS Quality (internal 960p) in a Balmora
street where only `_nh` statics march. Not in: actors and terrain (T-POM, L2), distance fade (Bevy has none: gap 36).

**T5 checks** (2026-11-13). Bevy: no terrain layer system; `StandardMaterial` reads +Y-up normals with bitangent
`w·cross(N, T)`, and Bevy's `generate_tangents` negates MikkTSpace's sign (`bevy_mesh` `mikktspace.rs:126`), so the
bitangent is −dP/dv: an image's top on top-left-origin UVs. Crates: none fits (our terrain is `sf_terrain`). Industry:
UE Landscape layers blend full material attributes (base colour, normal, roughness) per layer; Unity `TerrainLayer`
carries diffuse + normal map + mask map (metal, AO, height, smoothness); Frostbite and Decima terrain layers carry
albedo, normal and roughness per material; OpenMW terrain reads `_n`/`_nh` and `_diffusespec`. All bind a normal and a
roughness source per layer, which T5 does.

**T5 as built** (2026-11-13): `mw_bake::world::layer_maps::companions` finds each `LTEX` texture's normal (cook's
`normal_companion`) and ARM (cook's `orm_companion`, else a `TerrainAlbedoRoughness` rule: `_diffusespec` alpha) with
the companion's own source profile; `Convert` (normal: layout + Y flip; ORM: layout; `openmw` `_spec`:
`specular_to_orm`; `_diffusespec`: roughness = A) runs in `LayerSource::decode`, which now takes the `LayerFile` role.
`sf_terrain::bake::normal_to_array_frame` turns canonical +Y-up normals into the arrays' +V frame (G complemented, Z
rebuilt) before `mip_chain`; `terrain.layers` v3. Net: a DirectX `_n` lands in the array with its shipped green.
Tests: `bake_tests::layer_normals_shade_the_relief_the_albedo_shows` (height ramp → canonical map → array → CPU mirror
of `surface.wesl` = the relief's geometric normal), `world::layer_maps::tests` (DX `_n`, per-source profile +
fingerprint, ORM / specular / `_diffusespec` → AO, roughness). Bake line `terrain layers: N, n with a normal map, a
with ARM`. Visual check TX-2.

**T6 checks + as built** (2026-11-13, tx-t6). Bevy 0.20: compressed `2d_array` images upload layer-major with levels
under a block at their physical size (wgpu `create_texture_with_data`); `CompressedImageFormatSupport` tells BC.
Industry: UE landscape layers are ordinary textures (BC1/BC7 colour, BC5 normal, packed masks) streamed per mip; Unity
terrain likewise; Frostbite/Decima/Far Cry use terrain virtual texturing for many layers. Arrays have no per-layer
streaming, so a budget stands in until residency. Formats: albedo + height **BC7 sRGB** (ISPC **Fast**: 2048² Basic
measured 1.8x slower per layer, bake budget; statics keep Basic), normal X/Y **BC5** (UE's normal-map format; Z
rebuilt), AO + roughness **BC5** (two independent channels; BC7 would share endpoints with no gain at the same 1 B/texel,
and BC5 encodes ~100x faster). Cost: 3 fetches per projection instead of 2 (top-down ≤ 4 layers: 8 → 12, steep
triplanar ≤ 36), each ~2.7x smaller (8 → 3 B per texel over the three arrays). `sf_terrain`: `encode_layer` (mip chain
then strips of 16 block rows in parallel; a 1×1 default is one block repeated), `LayerArrays` FORMAT 3 (three
arrays, 16-byte blocks, shared offsets; `level_len`, `slice_len`, `gpu_bytes`), `LayerCap { max_edge, budget }`
(`skip`: top levels dropped, never below 4²), `from_bytes_capped` / `BakedTerrain::layers_capped` (borrowed decode,
copies only kept levels), `LayerInfo::source_edge` (table FORMAT 2, step v3): edge = `min(max, pow2(largest
source))`; `terrain.layers` v4; `mw_bake` `LAYER_ARRAY_SIZE` 2048. Runtime: `TerrainSettings::layer_cap` (game:
preset cap, plus `DEFAULT_LAYER_BUDGET` 1 GiB unless uncapped), `LAYER_ARRAY_FORMATS`, binding 113
`ao_roughness_array`, log `terrain layers: N x S², X MiB (L levels)`; no BC: error + flat defaults. Measured (16
threads; `mw_bake` `terrain_layers_real_install`, `sf_terrain` `layer_encode_time`): 142 layers; vanilla sources →
256² arrays, 35.5 MiB, 0.8 s; all 142 at 2048² 63 s (albedo BC7 dominates; normal/ARM BC5 add little: synthetic
0.33-0.52 s/layer with all three maps). Memory at 2048² bake: Low 512² 142 MiB, Medium 1024² 568 MiB, High 1024² 568
MiB (budget), Ultra 2048² 2272 MiB (was RGBA8 256² 95 MiB). Known cost: the 2048² product is 2.2 GB on disk and read
whole at load (split by level band if load time shows). Tests exact: `bake_tests::{layer_arrays_are_block_compressed_
full_chains, missing_maps_keep_their_defaults, bc5_layer_normals_shade_the_relief_the_albedo_shows, layer_cap_drops_
exactly_the_top_levels}`, `sf_terrain_bevy` `layer_arrays_upload_block_compressed`. Visual check TX-2.

**S1 as built** (2026-11-13): `sf_texture::level_cap` (Bevy-free): `Layout::parse` (header + level index), `plan(cap)`
(`first_level`: first level whose larger side ≤ cap, never past the last, backed off toward 0 until both sides are whole
texel blocks, from the DFD of the `vkFormat`; served whole: `VK_FORMAT_UNDEFINED`, `BasisLZ`, 3D, `levelCount` 0), `trim`
(header size/level count and level index rewritten, DFD/KVD/SGD and kept level bytes copied, levels re-aligned to
`lcm(4, block bytes)`), `cap_ktx2`. `sf_texture_bevy` (new crate): `CappedKtx2Loader` for `.ktx2` (Settings =
`ImageLoaderSettings`, so callers are unchanged) reads the header and index, then only the prefix the kept levels need,
trims, and passes the bytes to Bevy's own `ImageLoader` (sRGB from the settings as before, sampler, usages, zstd);
`TextureCapPlugin { cap }` preregisters it in `build`, registers it in `finish` after Bevy's (last registered wins in
`bevy_asset` `loaders.rs` `find`); `TextureCapStats` counters (loaded, capped, uncappable, GPU bytes loaded/skipped) and the
line `texture.cap: ...` once per settled loading burst. Game: `Quality::texture_cap`, `--texture-cap <px>` (0 = none),
plugin in the exterior app (viewers uncapped). Tests exact (`level_cap/tests.rs`, `sf_texture_bevy/src/tests.rs`).
Known: one startup `warn!` "Duplicate AssetLoader preregistered ... ktx2": `bevy_image/ktx2` must stay on (`tonemapping_luts`
LUTs, STBN) and Bevy 0.20 has no loader-replace API; removal path: an upstream override API (issue to file). **Deferred**:
per-level zstd (the cook writes none; Bevy's reader already decompresses it and `trim` keeps per-level streams), DDS/TGA
uncapped (Bevy loaders; cooked layer serves KTX2 for referenced textures).

**S2 as built** (2026-11-13). Bevy 0.20 does not rebind by itself: `Assets<Image>::insert` on a live id queues
`Modified` for the image only (`bevy_asset` `assets.rs:383`), materials re-extract only on their own events
(`bevy_render` `erased_render_asset.rs:311-329`), nothing reads `AssetEvent<Image>` for materials, and `GpuImage` keeps
the old view only for an equal descriptor + `COPY_DST` (`gpu_image.rs:202-216`); a touched material's `prepare_material`
sees the changed view, frees and reallocates its binding (`material_bind_groups.rs:1136-1169, 2796-2810`).
`sf_texture_bevy::rebind`: `ImageRebindPlugin<M>` (any `Asset` whose image fields are `#[dependency]`, read through
`VisitAssetDependencies`; `StandardMaterial` yes, 0.20 `ExtendedMaterial` no: it doesn't visit `base`) keeps
`ImageUsers<M>` (image -> materials, material -> distinct images) from `AssetEvent<M>` (`Added`/`Modified` re-read,
`Removed` drops), and on `AssetEvent<Image>::Modified` touches (`get_mut` + `into_inner`) each user once per frame
(two slots, two replaced images or two events: one touch). `LoadedWithDependencies` is not a trigger (first loads fire
it too; a replacement already emits `Modified`). Runs in `PostUpdate` after `AssetEventSystems`, so the material
rebinds one frame after the image (Bevy's per-type `asset_events` are `pub(crate)`); for that frame the old bind group
keeps the old view and texture alive (no `destroy`), never an invalid binding. Static scene: no new hook, the touch is a
material `Modified`, which `sf_scene_bevy` `slabs.rs` `invalidate_chain_slabs` already re-asks (risk §10). Counters
`ImageRebindStats` (images replaced, materials touched), line `texture.rebind: N images replaced, M materials touched`
once per settled burst. Game: plugin for `StandardMaterial` in the exterior app. Tests exact
(`sf_texture_bevy/src/rebind_tests.rs`: exactly the users touched, once each; two slots / two events -> one touch;
removal and edits move the index). For S3: replace under the handle with `Assets::insert` (that is the `Modified`); a
descriptor-equal update with `COPY_DST` reuses the view (the touch then takes the data-only fast path); other material
types need their own `ImageRebindPlugin<M>` and `#[dependency]` image fields.

**S3a as built** (2026-11-13). `sf_texture::residency` (Bevy-free): `TextureLevels` (`new(w, h, block, level bytes,
cap first level)` or `from_layout(&Layout, cap)` from S1's header + level index; start levels = cap's first level to the
mip tail, block-aligned only; capped level 0 ≤ 512²: one start, not streamed), `Demand { distance, rho: Option, in_reveal_radius }`
(`effective_distance` = nearer of now and `now + v·t`), `View { h_out, fov_y, bias, fallback_rho }`, `Settings { pool_bytes,
upgrade_stable_s 0.5, downgrade_wait_s 5 }`, `Residency::plan(view, settings, inputs, now) -> Plan { targets, requests,
counters, pool_warning }`. Levels are the file's (0 = uncapped full size). Choices: `floor(log2)` read from the f64 exponent
(exact at powers of two); a texture wants its finest demand; no demand = its tail. Hysteresis state is the `Residency`
value the caller owns (one per view): an upgrade goes to the coarsest level wanted over the last 0.5 s of finer wants, a
downgrade to the finest over 5 s of coarser ones; first sight takes the wanted level at once (loads behind fades don't
wait); textures absent from an input lose their state. Pool order, the §6 rule with "target − wanted" read in resolution:
most levels finer than currently wanted first (levels kept by the 5 s hysteresis go before any wanted level, UE's
cached-unwanted-mips-first), so the loss then spreads one level at a time; then least recently wanted (last time a demand
wanted finer than the tail), then most bytes freed (UE drops the largest first); in the reveal radius only after, farthest
first, `pool_warning` (ids) on the first plan of an episode. Requests: downgrades by bytes freed, then upgrades (reveal
radius, largest visible deficit, nearest); every tie ends on the id (packed `u128` keys), so the output is independent of
input and map order. Counters: textures, streamable, bytes resident / wanted / target, `over_budget`, `limited_by_pool`,
`limited_in_radius`, `mip_deficit_visible` (in-radius demands whose texture is resident coarser than they want), up /
downgrades. Tests exact (`residency/tests.rs`, 12); `#[ignore = "measurement: ..."]` 10k textures × 4 demands, release,
owner laptop (noisy clocks): median 0.7-1.4 ms with the pool fitting, 1.5-1.7 ms with the pool at half the wanted bytes
and 41 % of textures in the reveal radius (stress); at 4 Hz that is ≤ 0.7 % of a core, on S4's worker, not the frame.
For S3b/S4: S3b supplies ρ per (cluster, texture) and the clusters; S4 builds `TextureLevels` once per texture
(`from_layout`), computes `Demand.distance` with `effective_distance` per cluster, runs `plan` at 4 Hz off the main
thread, executes `requests` in order under the upload budget (downgrades free memory first), logs `pool_warning` once as
`warn!`, feeds `mip_deficit_visible` to `RevealGate`, and maps file levels to the loaded image (level-0 = first resident
level). Pool sizes per preset stay the §6 proposal until S4 measures.

**S3b as built** (2026-11-13, tx-s3b). UV density: `sf_model::uv_density` (UE's mesh UV density, `FUVDensityAccumulator`:
per LOD0 triangle `d = sqrt(A / a)` m per UV weighted by `sqrt(A)`, lowest and highest 10 % of the weight dropped, `ρ = 1/d`;
areas ≤ 1e-8 ignored, no valid triangle = `None`), one value per material, step `mesh.uv_density` reading `mesh.cook`
(rule version in its key, so a rule change never re-cooks meshes). Demand: `sf_scene::demand` product `texture.demand`
per streaming cell (subject as `scene.cells`; magic `SFTD`, format 1): clusters (cell-relative sphere, radius ≤ 16 m; a
larger single placement is its own cluster) each with its textures by id and their densest ρ. Ids: FNV-1a 64 of the VFS
path without extension (bake hashes the material's reference, runtime the loaded `.ktx2`, same id). Clustering rule 1:
spheres sorted by (x, z, y, radius, index), each joins the cluster whose bounding-box sphere grows least within the
limit, else a new one (deterministic). Per placement: bounds circumsphere grown by the largest axis scale, ρ divided by
the smallest. Skipped: actors and markers (S4 gives spawned ones a dynamic demand), editor-only and starts-disabled.
`DemandSource` adapter (default: base colour); `mw_bake::demand::MwDemandSource` adds glow + the cook's normal / ORM / depth
companions (all UV 0; the cook index digest is its fingerprint, so a pack change re-runs only demand, density hits).
Boot bake runs it after the cook and prints `texture.demand: N cells, C clusters (O single placements over 16 m), T textures
per cluster, D distinct textures, E entries without UV density, in t`. Tests exact (`uv_density/tests.rs`,
`demand/tests.rs`, `demand/tests/bake.rs`). For S4: one `Demand` per (cluster, texture) with the eye-to-sphere distance.

## 10. Risks

- **Static-scene batches key on material bind groups** (static-scene B3b-2): S2's material touch re-prepares bind
  groups and can re-key batches. Handled (S2 as built): the touch is a material `Modified`, which the scene's
  `invalidate_chain_slabs` already re-asks; a burst of replacements re-keys those batches (S3 budgets replacements).
- Tangents on every normal-mapped submesh add 16 B per vertex there (only there).
- A pool too small for the preset starves textures in view: logged, never silent; the planner falls back to the
  global size cap.
- Mods shipping 3-channel BC1/BC3 normals with odd conventions: per-mod `texture.normal_y` override; visual check.
- KTX2 level order: checked in S1 against the `ktx2` crate and our writer (level index level 0 first, data smallest first,
  absolute offsets); per-level zstd not used yet (deferred, §9 S1).

## 11. Owner questions and related gaps

1. Which normal-map pack (name/link)? Its format (DDS BC5 / BC3 / BC1 or TGA), names (`_n`, `_nh`, `_nm`), and does it
   also replace the diffuse textures?
2. Normals-only pack on vanilla albedo: keep de-lighting that albedo (proposed; adapter §3)?
3. Texture pool and size cap per preset (§6 proposal)?
4. Uncompressed HD albedo packs blow the bake budget on CPU BC7: GPU BC7 on the setup screen (T8 measures), or the
   Fast profile?
5. `_nh` parallax (Bevy `depth_map`, costly) and `_spec` (Phong colour + shininess, mapped to PBR specular and
   roughness): include now or park?
6. Related, not in scope (log only): model samplers use anisotropy 1 (`mw_assets::image_settings_clamped`):
   anisotropic filtering per preset is the standard pairing with HD and normal maps. Roughness folding of normal
   variance (Toksvig) is a later quality item.
7. MVR PBR's metal channel is empty at level 0 (its hotfix fills mips 1+ only): ship as is, rebuild level 0 from level 1 at cook (proposed), or ignore its metal? (adapter §7). If fixed, it is a generic pack-profile quirk flag ("rebuild channel X at level 0 from level 1"), not MVR code.

Decided 2026-11-11 by industry standard (owner: don't ask what the industry answers; the owner is downloading PBR
packs and points at a POM shader for inspiration): (1) T0 censuses the owner's packs and sets the naming table; support
the OpenMW/MGE names plus the common PBR maps (roughness, metalness, AO, height, ORM-packed). (2) Yes, keep de-lighting
under a normals-only pack. (3) Pools and caps as §6 proposes. (4) GPU BC7 on the setup screen (owner allowed GPU use),
CPU Fast profile as fallback. (5) Include both: `_nh`/height through Bevy's native parallax (`depth_map`,
`ParallaxMappingMethod` Occlusion/Relief per preset, off on Low), compared against the owner's POM shader; `_spec`
mapped to PBR. (6) Anisotropic filtering per preset (Low 4x, Medium 8x, High/Ultra 16x) joins T4.
