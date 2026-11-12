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
  [(id, ρ)] }` (ρ from `mesh.cook`'s per-submesh UV density, UE's "mesh UV density"). Resident cells contribute.
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
the change applies live, before it on the next load.

**Not in v1**: GPU-side copy of resident levels (we re-upload the lower levels: +1/3 of the top level's bytes,
no render-world code), terrain layer arrays (fixed size per preset, terrain.md), far-field atlases (fixed).

## 7. Part C: normal maps from packs

**Rule (owner 2026-11-11): the renderer is generic, never tuned to one pack.** Runtime materials use one canonical
layout (Bevy `StandardMaterial` slots: sRGB base colour, linear normal in Bevy's Y convention, glTF ORM, depth =
1 - height). Each pack's conventions (suffixes, channel order, Y, height neutral, known defects) are *data*: a pack
profile in the adapter or the mod's manifest, converted to the canonical layout at bake. No renderer code path or
shader constant exists for a specific pack; a pack without a profile falls back to the game's default convention.
1. **Discovery** is the adapter's (naming differs per game): for an albedo path it returns companion paths per role
   (`Normal`, `NormalHeight`, `Mask`), resolved through the VFS with the usual alternates, so a pack is just a mod.
2. **Colour space**: normal maps are linear. Cooked: `BC5_UNORM`. Served as shipped (no cook yet): the loader must
   set `is_srgb = false` for the normal slot (today every model texture loads sRGB).
3. **Y convention**: the framework's convention is Bevy's (+Y, OpenGL/glTF). The adapter declares the content's
   default `source_y`; a mod manifest may override it per mod (`texture.normal_y = "down"`); cook inverts G.
4. **Height and specular**: `_nh` alpha height maps to Bevy's `depth_map` (`1 − height`, parallax occlusion,
   costly per pixel) and a specular companion to Bevy's specular/roughness inputs: both behind owner question 5.
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
| T5 | Terrain normals: `TerrainLayer.normal` from companions | exact: layer record carries the path; visual check TX-2 |
| T6 | Terrain layer arrays BC7 + BC5 (terrain.md already specifies) | exact: array formats and sizes; VRAM line |
| T7 | Far-field atlases BC7 (bake.md gap 9; owner of far field coordinates) | exact: formats; VRAM delta printed |
| T8 | Measurement: GPU BC7 (`block_compression`) vs ISPC basic/fast, ms per MPix and PSNR on 50 vanilla textures | `#[ignore = "measurement: ..."]` printout |
| S1 | Our KTX2 loader (replaces Bevy's for `.ktx2`): per-preset size cap (top levels skipped), per-level zstd, counters | exact: a 2048² file under a 1024 cap yields 1024² with levels − 1; counters line |
| S2 | Rebind on reallocation: material reverse index, touch on `AssetEvent<Image>::Modified` | exact (no GPU): replacing an image marks every material using it modified, none else |
| S3 | `sf_texture::residency` planner + `texture.demand` bake product + UV density in `mesh.cook` | exact: wanted levels for fixed inputs, over-budget order, hysteresis, determinism |
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

## 10. Risks

- **Static-scene batches key on material bind groups** (static-scene B3b-2): S2's material touch re-prepares bind
  groups and can re-key batches. S2/S4 wait for the static-scene runtime to settle and agree the hook with its owner.
- Tangents on every normal-mapped submesh add 16 B per vertex there (only there).
- A pool too small for the preset starves textures in view: logged, never silent; the planner falls back to the
  global size cap.
- Mods shipping 3-channel BC1/BC3 normals with odd conventions: per-mod `texture.normal_y` override; visual check.
- KTX2 level order and per-level zstd are recalled from the spec (research §3): S1 re-checks before relying on it.

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
