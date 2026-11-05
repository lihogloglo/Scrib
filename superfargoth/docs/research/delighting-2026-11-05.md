# Texture de-lighting for pre-lit Morrowind albedo (research, 2026-11-05)

Research only; nothing built. Question: vanilla Morrowind textures carry painted light (dark corners, baked AO), and we now
add shadows, SSAO/contact shadows, probe GI (`superfargoth/docs/specs/gi.md`) and Solari on Ultra. Should we de-light, and how?

## 1. Summary and recommendation

- **Yes, but as a mild, bounded, bake-time step, and Ultra (Solari / probe GI) benefits most.** The only shipped renderer that
  does this for Morrowind (openmw-rtx) divides out a coarse luminance estimate, normalised to mean 1 and clamped to 0.5..2.0.
  Their own comments say it removes slow gradients only and flattens genuinely graded albedo (Retinex failure mode).
- **Raster High (shadows + SSAO + probe GI):** useful but optional. Double darkening in corners is real, but the painted
  light is also part of the art style, and a mild de-light weakens that. Ship it as a profile switch (`delight` strength),
  default off until the owner signs off the test scene (section 5).
- **Solari Ultra / probe GI hits:** recommended on. Solari's binder reads the `StandardMaterial` base colour only
  (`superfargoth/docs/specs/gi.md:177`), and `.sessions/gi-weak.md` measured Morrowind linear albedo at 0.12-0.2, so each
  bounce keeps only rho/(1-rho) = 0.14-0.25 of the light. Painted darkness lowers the average albedo further.
- **Chosen design: option (a), the bake writes de-lit texels into a cooked texture** (zero runtime cost, helps raster,
  probe GI and Solari identically, no shader changes, mods opt out through VFS priority). (b) shader divide only helps
  surfaces we own the shader for, not Solari's binder. (c) hit-only is incomplete for the same reason.
- Caveat: today **no texture cooking exists** (section 4), so (a) needs a new texture bake product. That is the real cost
  and is already planned as M7 ("PBR generation at bake time ... albedo de-lighting", `superfargoth/docs/WISHLIST.md:227`).
- Do not expect de-lighting to fix weak bounce alone: the 0.12-0.2 albedo is mostly the intended dark-brown palette. A
  separate, explicit albedo-lift or exposure decision is an owner question (section 5).

## 2. How openmw-rtx does it

Repo `github.com/xorza/openmw-rtx`, pinned `f2f8b2d5b2cb142d1b454b3bb1815f4552980562` (GPLv3, ports allowed with
citation + `THIRD_PARTY_NOTICES.md`). Paths below are relative to that commit.

**Algorithm** (`components/rtx/image/shadingmap.hpp/.cpp`, host reference; `components/rtxvulkan/shaders/texture/shadingmap.comp`
plus `shadingsum.comp`, device version run as the texture uploads):
1. Read the largest mip once. BC1/DXT blocks are read through their 4-entry palette and the indices that chose each entry
   (a block's sum is its palette weighted by index counts), loose formats texel by texel. sRGB formats are converted to linear
   first (`linearOf`), then luminance with Rec.709 weights (`shadingmap.cpp`, `LUMINANCE_WEIGHTS`).
2. **Alpha-weighted**: every texel's contribution and the cell weight are multiplied by alpha, in every format (BC1 punch-through
   holes have alpha 0). Reason in `shadingmap.h` (`ShadingSum`): a hole texel is black and "darkened every cell round a leaf's
   holes, which the delighting then brightened the leaves against". A fully transparent cutout returns the neutral map.
3. Accumulate into a **32 x 32 grid** (`SHADING_EXTENT = 32`, `components/rtx/shaders/scene.h:105`): "Coarse on purpose:
   painted lighting varies slowly ... painted detail does not". Textures smaller than the grid leave empty cells; those are filled
   with the mean of the sampled cells, and a texture with no sampled cells is neutral (no correction).
4. **Three 3x3 box-blur passes** (`SHADING_BLUR_PASSES = 3`, `shadingmap.h`), wrapping around axes where the texture repeats and
   clamping on axes where it clamps (`mWrap`), because Morrowind textures tile and a clamped blur would invent a seam; clamped
   textures (banners, flames) would otherwise divide real albedo by the opposite edge's light. About 11 texels of blur at 256 px.
5. **Normalise to mean 1** ("a redistribution rather than a dimmer": moves light from where the texture has it to where it does
   not, total unchanged), then **clamp to [0.5, 2.0]** (`SHADING_FLOOR/CEILING`, `components/rtx/shaders/look.h:362-363`).
   A black texture (mean 0) gets the neutral map. Stored on the device as R16 unorm over [0.5, 2.0] so that 1.0 is exactly
   21845/65535.
6. Test (`apps/components_tests/rtx/image/shadingmap.cpp`): a 3:1 cosine gradient (60..180 linear) comes back as 0.519..1.481
   (ratio 2.854, i.e. what three blurs keep, 96.2% of amplitude). Other tests: a checkerboard is left alone, cutout holes weigh
   nothing, clamped axes, tiny textures neutral, black neutral, BGRA vs RGBA channel order.

**Where it is divided out** (`components/rtxvulkan/shaders/lib/ground.glsl:52-86`, `delitTexel`; called from
`texturing.glsl:362` `sampleAlbedo` for every path-traced hit, and from `layerTexel` for every terrain layer):
```
delit = texel / mix(1.0, paintedLight(slot, uv), delight);   // delight in [0,1], run-wide, CLI --delight
return delit / max(brightest(delit), 1.0);                  // hold reflectance <= 1, scale all 3 channels (colour kept)
```
- It applies to **albedo reads of the primary surface and of bounces alike** (their trace is the renderer; they do not have a
  raster primary). The near-field ground "bounce reads the composite only where the frame divides out the painted light the bake
  did" (`traversal.glsl:1351`): a baked ground composite must have been baked at the same delight value, else it falls back to
  summing layers.
- `--delight` is a continuous 0..1 strength, "Zero is the A/B that says what it did" (`apps/rtxtool/options.cpp:364-366`). I
  could not find the shipped default value in the files read.
- The factor is a grey, bilinear, **wrapping** lookup (`paintedLight`, sampler does wrap and blend; decode after filter is exact
  because it is affine).
- **Skipped for**: authored (PBR replacer) ground layers, `if (authored) return texel;` in `layerTexel`; emissive maps ("light
  rather than a surface and must keep what it was painted with"); the cutout mask (alpha unaffected). `docs/rtx/architecture.md:23`:
  "a PBR replacer's companion maps reach the trace, and a vanilla scene draws the same whether or not" they can be read.
  `_n`, `_nh`, `_spec` companions are data slots for the material (architecture.md:267-268). For ordinary statics I did not
  trace how a replacer diffuse opts out per object (only per ground layer was verified).
- **Untextured surfaces**: albedo is white (`UNTEXTURED_ALBEDO`, look.h) and untextured shapes must get the neutral map; the
  class comment says a missing map once made "every untextured surface ... divided by two".
- Vertex colours: not found in the de-light path; I did not verify.
- **Honest limits** they state: occlusion in a corner or a rim highlight a few texels wide is not removed; slowly varying true
  albedo (a wall painted darker in the lower half) is flattened; this is Retinex's smoothness assumption
  (`ground.glsl` comment on `delitTexel`).

## 3. Industry comparison

| Technique | Who | Cost | Quality | Pitfalls |
|---|---|---|---|---|
| Coarse luminance-grid divide, mean-normalised, clamped | openmw-rtx (above) | Texture-upload time, GPU compute; negligible | Removes only slow gradients; safe by construction (bounded 0.5..2) | Under-corrects tight AO; over-flattens graded albedo; needs per-texture wrap awareness |
| Tool-assisted de-lighting from AO + normal + bent-normal maps (needs the lit texture plus baked maps) | Unity De-Lighting Tool, 2017 experimental ([80.lv](https://80.lv/articles/photogrammetry-de-lighting-tool-for-unity), [CG Channel](https://www.cgchannel.com/2017/07/download-unity-labs-free-texture-de-lighting-tool/), [Sketchfab guide](https://sketchfab.com/blogs/community/unity-photogrammetry-de-lighting-workflow)) | Offline, per asset; needs geometry-derived maps | Good for scanned objects with real geometry | Needs the high-poly or maps; Morrowind has none of these for its tiling textures |
| Learned albedo generation from diffuse (upscale, normal, roughness, height; some models trained to remove baked lighting) | RTX Remix community: PBRify_Remix ([repo](https://github.com/Kim2091/PBRify_Remix)), PBRFusion used on Painkiller RTX ([NVIDIA blog](https://developer.nvidia.com/blog/how-painkiller-rtx-uses-generative-ai-to-modernize-game-assets-at-scale)) | Offline GPU batch | Reasonable baseline; the Painkiller team still hand-authored metals, glass and skin and said atlases confuse the models | Model licence and size; non-deterministic; not a clean-room generic algorithm; the source states the baked-lighting removal motivation, but I could not confirm PBRFusion itself does it |
| Hand/ML-processed "ambiently lit" Morrowind retextures | Nexus mod based on Vurt's Morrowind Visual Resurgence ([Nexus 57592](https://www.nexusmods.com/morrowind/mods/57592)) | Community labour | Author says it is not a proper albedo, and that without parallax self-shadowing some baked lighting must stay or things look cartoony | Evidence that complete removal reads wrong |
| Sidecar normal maps by naming (`_n`) for vanilla textures | OpenMW ([docs](https://openmw.readthedocs.io/en/openmw-0.41.0/openmw-mods/convert_bump_mapped_mods.html)); Rafael's PBR shaders need the diffuse used as albedo | None at runtime | Adds relief, does not de-light | PBR packs assume the diffuse is albedo |
| Intrinsic image decomposition (Retinex lineage; modern learned: SAIL arXiv [2505.19751](https://arxiv.org/abs/2505.19751), Ordinal Shading [2311.12792](https://arxiv.org/abs/2311.12792)) | Research | Heavy (diffusion models) | State of the art on photos | Trained on real photos; tiling game textures are out of distribution; overkill and non-deterministic for a bake |

What is canonical: for scanned assets, geometry-aware de-lighting (AO/bent normal); for legacy game art, a learned or manual
pass. What is cheap: the openmw-rtx grid divide. What over-corrects: anything following texel-scale detail (their comment: "the
over-correction that flattens a texture into a colour"). I found no talk or paper (Ubisoft, EA, DICE) specific to
Morrowind-style tiling textures; photogrammetry de-lighting is a different problem, so treat it as adjacent, not direct evidence.

## 4. Our architecture and design options

**How textures reach the GPU today** (grep): no texture cooking. `mw_assets` resolves a base-colour texture reference via
the VFS with `TEXTURE_ALTERNATES = ["ktx2","dds","png"]` (`bevyrrowind/crates/mw_assets/src/lib.rs:41-44`) and Bevy's own
`dds`/`tga`/`bmp` loaders upload BCn compressed at runtime. `mw_bake::textures::decode_rgba` (`bevyrrowind/crates/mw_bake/src/textures.rs:21`)
decodes DDS (via `ddsfile` + `texpresso`) to RGBA8 on the CPU, today only for terrain layer arrays and the cutout test
(`mw_assets/src/lib.rs:359-373`, a flagged scaffold). `bake-adapter.md` lists texture baking as M7 (line 17). Because
`ktx2` is first in the alternates list, a baked KTX2 placed in a cache VFS layer already wins with no loader changes.

### (a) Bake writes de-lit texels (recommended)
- Generic algorithm in a new Bevy-free module, e.g. `sf_bake` (or `sf_model`): `delight_grid(rgba, wrap, alpha) -> [f32; N*N]`
  and `apply(rgba, grid, strength)`. Port of `shadingmap.cpp` steps 1-5 with citation. Nothing names Morrowind.
- Adapter hook (`mw_bake`): enumerates textures from the VFS, passes tile/clamp flags (from NIF `NiTexturingProperty` clamp
  mode, already carried as `(clamp_s, clamp_t)`, `mw_assets/src/lib.rs:441`) and the **exclusion policy** (skip if a
  `_n`/`_nh`/`_spec` companion or an `authored` PBR marker exists, skip emissive/glow slots, skip UI/sky/particle textures).
  Texture role is known only per material, so the bake needs a texture -> role table from the model bake (a texture used as
  both albedo and glow is skipped).
- Output: de-lit RGBA with mips regenerated, written as KTX2 (BC7 or BC1/BC3 re-encode) into the profile cache; the VFS cache
  layer sits below mods so any mod PBR replacer wins automatically (VFS priority, "everything is a mod"). Original kept
  untouched for the A/B and for the classic preset.
- Cost estimate (not measured): a vanilla install is a few thousand textures, mostly 128-512 px. CPU grid + divide is
  microseconds-to-milliseconds per texture; the dominant cost is BC re-encode. A fast CPU BC7 encoder across all cores is
  expected to take tens of seconds to a few minutes total, inside the "one-time step under a few minutes" rule. Measure before
  committing (`#[ignore = "measurement: ..."]` test). Encoder crate choice needs its own reuse check (licence, maintenance).
- Memory: same as today if re-encoded to the same BC format; BC7 is 2x BC1. Re-encoding BC1 -> BC7 adds a second lossy
  generation; mitigate by decoding exactly then encoding at high quality, or keep BC1/BC3 for fidelity.
- Strength is baked in (a profile product `texture.delight` with strength in its key), so changing strength re-bakes the
  texture product; no runtime knob. One more reason for a fixed, owner-signed value.
- Cons: bake knob not live; cost of a new product; recolours decals/atlases wrongly if the role table is incomplete.

### (b) Per-texture factor grid + shader divide (their way)
- Bake stores a 32x32 R16 grid sidecar per texture (2 KB each); a custom material extension (Bevy `ExtendedMaterial` on
  `StandardMaterial`) divides in the fragment shader; a live strength slider is possible.
- Touches: `mw_assets` materials, a new shader, the vendored `bevy_pbr` binder path for any custom hit shading.
- Problem: **Solari's binder reads `StandardMaterial` base colour only** (`gi.md:177`) and our probe GI hit shading uses
  albedo from the same path, so the divide would have to be duplicated in Solari's binder (a patch to vendored Bevy) and in
  `sf_gi_bevy` hit shading. This is the openmw-rtx situation only because their entire renderer is theirs. Extra texture
  fetch per fragment, plus a second binding per material. Not recommended.

### (c) Only at GI/RT hits
- Primary surface keeps the painted look; bounce uses de-lit albedo. Appealing for art preservation, and cheap if we sample a
  proxy albedo at hits. But Solari's hits come from the same `StandardMaterial` binding as the primary, so it would need a
  second texture handle per material and a binder patch. For `sf_gi` probe hits, a per-material de-lit proxy (the cooked
  texture from (a) bound only in the GI path) is possible later. Treat as an extension of (a), not an alternative: the cooked
  de-lit texture is the prerequisite either way.

### Framework / game split
Generic algorithm, grid parameters and wrap-aware blur: `superfargoth` (`sf_bake` step). Which textures, tile/clamp flags,
companions and roles: `bevyrrowind/crates/mw_bake` adapter. Strength and on/off: a bake profile setting.

## 5. Open questions for the owner, and a proposed visual check

1. Default on or off for raster High (art-style vs double-darkening)? Proposed: off until signed off, on for Ultra.
2. Strength: openmw-rtx uses a continuous 0..1; bake one value (suggest start at 0.5-0.75, with the 0.5..2.0 clamp and the
   `/max(brightest,1)` guard) or ship two caches?
3. Is the weak GI bounce mainly albedo? If yes, is a separate albedo lift for GI hits acceptable (a hack, needs a named
   removal path), or do we accept dark brown?
4. Approve a new texture bake product (M7 pulled forward) and a BC7 encoder dependency (batched crate update rule applies)?
5. Terrain layer textures and the ground composite must use the same delight value as models (openmw-rtx binds them together,
   `traversal.glsl:1351`).

**Test scene (awaiting owner, to be registered in `bevyrrowind/docs/visual-checks/`)**: Census office interior (GI volume
already there) and a Balmora wall/plaster street corner at noon and at dusk, flags: GI on, `--rt` for the Solari pair, A/B of
original vs de-lit cache. Look for: (1) corners and wall feet no longer doubly dark under SSAO and contact shadows;
(2) plaster, brick and wood still read as their material, not flat colour (flattening is the failure mode); (3) leaf cutouts
without bright or dark fringes round holes; (4) tiling seams on repeated walls; (5) banners, signs and glow textures
unchanged; (6) bounce colour bleed visibly stronger in the interior. Agents do not judge this from screenshots.
