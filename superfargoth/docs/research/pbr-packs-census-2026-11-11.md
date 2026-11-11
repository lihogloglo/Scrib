# PBR packs census (ROADMAP 1.20 slice T0, 2026-11-11)

Measurement of the three packs the owner put in `textures PBR/` (git-ignored, never committed). Results and the
decisions they drive live in `bevyrrowind/docs/specs/textures-adapter.md` §7; this note keeps method and evidence.
Scripts lived in the session scratchpad (not the repo): a DDS header walker, a BSA name lister, and a channel
statistics pass with BC1/BC3/BC5/BC7 decoders (BC7 partition tables read at runtime from `bcdec_rs` 0.2.1).

## 1. Packs

| Pack (Nexus id) | Files | MiB | What it is |
|---|---|---|---|
| MVR PBR v2 Beta (57592) | 10 769 DDS + `MetalMipmapsHotfix.zip` (422 `_spec`) | 13 018 | Whole-game PBR set: diffuse + `_spec` + `_n` or `_nh` (+ `_diffusespec` for 165 land textures), flat `textures/` |
| Tamrielic PBR, West Gash 2K (60568) | 42 DDS | 339 | Ten West Gash landscape textures (+ `_land_default`, one grass atlas): diffuse, `_spec`, `_nh`, `_diffusespec` |
| Parallax Replacement v1.2 (58303) | `parallax.glsl` | | OpenMW 0.51 POM shader for the above (no licence notice: technique reference only, no code ported) |

No readme ships in any folder; Nexus pages answer 403 to automated fetches. The packs target **Wareya's PBR shaders
for OpenMW** (github.com/wareya/OpenMW-PBR, GPL-3.0, commit `cab24e7a`, 2026-07-14): the POM shader names its TBN
modes "from Wareya's PBR Shaders" and "Rafael's original 4-tap radial" AO; the pixel statistics below match Wareya's
channel layout exactly.

## 2. Header census (every file)

MVR (formats are legacy FourCC; every chain is full except 104 foliage diffuses, below):

| Suffix | n | MiB | Formats | Sizes (larger side) |
|---|---|---|---|---|
| diffuse | 3 346 (+39 vanilla names ending `_g`/`_h`/`_metal`: greaves, helms, "rusty_metal") | 4 101 | DXT5 1 869, DXT1 1 371, DXT3 106 | 1024: 1 984, 2048: 806, 4096: 19, rest ≤ 512 |
| `_spec` | 3 390 | 921 | DXT1 3 204, DXT5 186 | 512: 2 146, 1024: 874, 2048: 40 |
| `_n` | 2 644 | 1 235 | DXT1 2 639, ATI2 3, DXT5 2 | 1024: 1 444, 2048: 246, 4096: 9 |
| `_nh` | 1 185 | 3 216 | DXT5 all | 1024: 611, 2048: 515, 4096: 26; one 1×1 (`Tx_BM_rock_01_nh`) |
| `_diffusespec` | 165 | 3 520 | DXT5 all | 4096 all |

- **Vanilla coverage**: the three BSAs hold 4 783 texture names (DXT1 3 836, DXT3 857, uncompressed 90). MVR
  replaces **3 332** of them (3 329 upscaled; the vanilla originals total 96 MiB), plus 53 new names (atlases,
  `pc_*`/`tr_*` assets of other mods). Every companion's diffuse ships in the same pack (2 636/2 644 `_n`, 1 169/1 185
  `_nh`, all `_spec`, all `_diffusespec`), so the adapter's "same source" rule skips de-light for all of them.
- 415 bases have both `_n` and `_nh`: OpenMW takes `_nh` first, `_n` only when no `_nh` exists
  (`components/shader/shadervisitor.cpp` ~340-349, OpenMW commit `97722d48`); we do the same.
- 104 diffuses (grass, ferns, leaves, lilypads) ship truncated chains (3, 8 or 10 levels of 11-12): alpha-tested
  foliage; T2's mip fill completes them (the cook's `preserve_coverage` path, textures.md §5).
- 3 537 textures are non-square; no NPOT, no trailing bytes, no cube maps.

Tamrielic PBR: diffuse 11 (BC7 UNORM via DX10, 2048², one 4096²), `_spec` 11 (BC7 10, DXT5 1), `_diffusespec` 10
(BC7), `_nh` 10 (**7 uncompressed** B8G8R8A8 2048², 3 BC7). All ten diffuses are vanilla landscape names (DXT1 256²).

## 3. Channel statistics (sample, level ≤ 512 decoded, means over files)

| Suffix | R | G | B | A |
|---|---|---|---|---|
| `_spec` MVR (all 3 388 at 64², 50 at 512²) | 0 in 3 160 files; bimodal 0/255 (sd ~100) in the ~100 with metal (dwemer armour, buckles, steel shoes) | 207 (sd 12, per file 50-255): mid-high | 248, 85 % ≥ 239 | 255 except 156 files; Dunmer heads 26 |
| `_spec` TPBR | 0 | 197 | 218 | 233 |
| `_n` MVR | 118 | 119 | 245 | 255 (unused) |
| `_nh` MVR / TPBR | 120 / 129 | 122 / 128 | 243 / 238 | 128 / 115 |
| `_diffusespec` MVR / TPBR | RGB = the pack's own diffuse (mean abs diff 0.9 / 2.0 of 255) | | | 209 / 201 |

Readings, against Wareya `lib/light/lighting_pbr.glsl` `specMapToPBR` (~line 900): `metallicity = specTex.r`,
`roughness = specTex.g` (perceptual, squared there; `PBR_MAT_ROUGHNESS_INVERTED 0` default), `ao = specTex.b`,
`sss = 1 - specTex.a`, `f0 = 0.04`. Terrain (`compatibility/terrain.frag` ~122) builds
`specularColor = vec4(0, diffuseTex.a, 1, 1)`: `_diffusespec` alpha is **roughness**, metal 0, AO 1. The stats fit:
R mostly 0 (non-metal world), G a plausible roughness, B a bright AO, A 255 (no subsurface) except skin. This is not
OpenMW's vanilla `_spec` (RGB specular colour, A shininess × 255), which would never give R ≡ 0 with G, B high.

**Y convention, measured.** For `_nh` the normal was correlated with the gradient of its own alpha: corr(R, ∂h/∂x)
−0.84 (MVR, 40/40 files negative), −0.70 (TPBR, 10/10), so alpha is height with white = high. corr(G, ∂h/∂row)
(rows run down the image) −0.83 / −0.70, **same sign as R**: +Y points down the image, i.e. **DirectX**. For `_n`
(no height) against diffuse luminance: R −0.48, G −0.48 (41/43 negative): DirectX too, as OpenMW expects.
Normals are unit length as stored (mean |n| 1.006 for `_n`, 0.98 `_nh`, 0.92 TPBR BC7): three-channel maps.

**Height neutral.** MVR `_nh` alpha per-file means are 128-129 for every sampled file: heights are normalised to
mid-grey. The POM shader states it: `PARALLAX_NEUTRAL 0.5 // Heightmap value that represents the geometric surface`.

**Metal data defect.** At level 0, 3 160 of 3 204 MVR DXT1 `_spec` have no block with a non-zero red endpoint; 38
have metal only in mips 1+, 6 in both. All 422 `MetalMipmapsHotfix.zip` files (same names, same formats and chain
lengths) have metal **only in mips 1+**, level 0 still 0 (e.g. `tx_a_adamantium_c_spec`: level 0 R 0 %, level 1
74 % of blocks). As shipped, armour turns metallic only beyond the first mip transition (a few metres).

**Opaque DXT5.** 26 of 30 sampled DXT5 diffuses have alpha 255 everywhere (vanilla was DXT1 for 1 665 of them):
a BC3 whose alpha is all 255 is exactly a BC1 (colour block reordered), half the bytes.

## 4. Compression and IO numbers

- zstd level 3 on the DDS payloads (30 files per suffix): MVR diffuse 59.5 % of raw, `_n` 80.9 %, `_nh` 72.6 %,
  `_spec` 51.4 %, `_diffusespec` 70.6 %; TPBR (BC7) 78-92 %. BCn-legacy products clear the spec's 15 % bar
  (textures.md §5 "Product"); BC7 mostly doesn't (8 % for colour). Compression ran at ~5-10 ms/MiB per core.
- Read throughput from D: 1.7 GB of `_nh` in 4.4 s (~390 MB/s, warm-ish cache).
- Resident bytes if every texture of the pack were loaded, by size cap (MiB): MVR 1 755 at 512², 5 590 at 1024²,
  10 069 at 2048², 13 018 uncapped; TPBR 21 / 83 / 331 / 339. Either pack fills any §6 pool many times over:
  residency (S3/S4) decides what is on the GPU; the cap only bounds a single texture.
- Disk: the owner's C: (bake store, `%LOCALAPPDATA%/bevyrrowind/bake`) had 28 GB free on 2026-11-11. A full KTX2
  copy of MVR is 13 GB raw, ~8.5 GB with zstd.

## 5. POM: the owner's shader vs Bevy 0.20

Owner's `parallax.glsl` (OpenMW 0.51): steep parallax, layer count `mix(32, 4, |V.z|)` (terrain and objects), then
5 binary-search refinements (relief); UV scale 0.04 (objects divided by `|V.z| + 0.3` bias, terrain by `max(|V.z|,
0.1)`); march starts offset by `PARALLAX_NEUTRAL 0.5` so relief goes both above and below the surface; three
distance tiers: full POM to 3 500-4 000 units (~50 m), iterative parallax (4 iterations) to 5 000-5 500, then one
iteration (plain offset); soft self-shadow (8 steps, strength 4) against the sun; terrain-only cavity AO from height
(4-tap radial, or a LOD-difference mode); TBN derived from screen derivatives and the triangle normal (seam-free
without vertex tangents); height read from the normal map's alpha with filtered (gradient) sampling.

Bevy 0.20 (`bevy_pbr/src/render/parallax_mapping.wesl`, `StandardMaterial` defaults): `depth_map` (`.r`, depth =
1 − height, 0 = the geometric surface), `parallax_depth_scale` 0.1, `max_parallax_layer_count` 16 with layers
`mix(max, 1, |V.z|)`, `ParallaxMappingMethod::Occlusion` (linear interpolation of the last step) or
`Relief { max_steps }` (default 5, binary search: the same technique as the shader's core). Missing against the
shader and the industry baseline (Tatarchuk, "Practical Parallax Occlusion Mapping", GDC 2006: dynamic sample count,
LOD fallback to normal mapping, soft self-shadows; UE's POM node: min/max steps, gradient sampling):
1. no neutral offset (Bevy's surface is height 1: mid-grey content reads as recessed, depth halves on average);
2. no distance fade or LOD fallback (only the view-angle layer count);
3. **level-0 sampling** (`textureSampleLevel(.., 0.0)`): no mip filtering on the depth map, so distant parallax
   aliases and reads full-size texels (a 2K map at 50 m costs cache misses per step);
4. no self-shadowing; 5. no terrain path (our terrain shader is our own).
Items 1-4 are one vendored `bevy_pbr` patch (`superfargoth patch (own: parallax LOD)`: neutral uniform, fade by view
depth to plain normal mapping, `textureSampleGrad`, optional sun self-shadow), offered upstream; item 5 joins T5.
