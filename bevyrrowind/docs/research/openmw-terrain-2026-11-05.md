# OpenMW terrain rendering (research, 2026-11-05)

Source: github.com/OpenMW/openmw, master at `d0c0fc0e600703f979e110caf92cf869b6552114` (2026-11-05).
Method: the shader/C++ files were read through a summarising fetch, so the notes below are paraphrases, not
verbatim reads. Items marked (unverified) come from memory or were not visible. Nothing was copied (GPL clean room).

Files: `files/shaders/compatibility/terrain.frag`, `terrain.vert`, `terrain_composite.*`, `fog.glsl`, `normals.glsl`,
`files/shaders/lib/light/{util,clamp,bindings}.glsl`, `components/terrain/material.cpp`,
`components/esmterrain/storage.cpp`, `files/settings-default.cfg`, `apps/openmw/mwrender/renderingmanager.cpp`.

## 1. Shading model
- Plain Blinn-Phong style, the fixed-function equivalent, not PBR. Final colour:
  `lighting = diffuseLight*diffuseColor + ambientLight*ambientColor (+emission)`, then `clampLighting`;
  `out = texColor * lighting + specularColor * specularLight`; then fog.
- Per-vertex lighting by default (terrain.vert computes diffuse/ambient/specular per vertex, blended with the shadow
  ratio in the fragment shader). Per-pixel lighting only when a normal map, a specular map, or
  `force per pixel lighting` (default false) is active (`PER_PIXEL_LIGHTING` define).
- Vertex colours (the ESM VCLR land colours) are a material input (`colorMode` uniform = 2) that modulates
  diffuse and ambient. This is the "baked" terrain tint of vanilla.
- Specular: `lib/light/util.glsl` computes `pow(clamp(dot(N,H),0,1), shininess)` only when `N.L > 0`, using
  the light's specular colour. The terrain pass sets no `osg::Material` (material.cpp); the default specular is
  effectively zero, so by default terrain has NO specular and NO environment reflection. There is no IBL anywhere.
- Specular turns on only through `auto use terrain specular maps` (default false). It looks for a
  `_diffusespec` file (pattern configurable) that replaces the diffuse texture, with the specular multiplier in
  its alpha: `specularColor = vec3(diffuse.a)`, shininess hard-coded to 128 (marked TODO in the shader).
  Detection is by file name, not by inspecting diffuse alpha.
- Diffuse texture alpha is never used as opacity.

## 2. Layer blending
- One render pass per layer (`material.cpp`), no single-pass multi-layer path. Layer 0: blend `SRC_ALPHA, ZERO`,
  depth LEQUAL, bin 0. Later layers: `SRC_ALPHA, ONE`, depth EQUAL, bin 1. Each layer's pass multiplies its output
  alpha by its blend-map alpha, so the blend is additive-weighted over the base. Texture units: 0 diffuse,
  1 blend map, 2 normal map. Without blend maps (single-layer chunk) no blending is enabled.
- Blend maps (`storage.cpp`): one 8-bit alpha image per unique texture per chunk, taken per land-texture sample
  (the 16x16 texture grid per cell), then upscaled 2x with nearest sampling. A texture-matrix "nudge" of
  1/scale/4 shifts the map "to look like vanilla" (visible seams unless the map is doubled). TES4/5 land uses
  per-quadrant opacities. There is no height blending or contrast blending. A `terrain_composite` shader exists
  for distant-LOD composite maps (baked per chunk, only considers blend-map presence).
- Net effect: hard-edged texel blends upscaled by the filter, unlit-flat look, very vanilla.

## 3. Normal maps and parallax
- Opt-in: `auto use terrain normal maps` (default false). Naming: for `foo.dds` the loader first tries the height
  variant (`foo_nh.dds`, suffix configurable), which enables parallax (height in the normal map alpha), else
  `foo_n.dds` (no parallax). TES5-style sets hard-disable parallax (alpha holds specular).
- Normal decode `n = tex*2-1`; optional Z reconstruction `sqrt(1-x²-y²)` for RG formats (which also disables
  parallax). Tangent frame is built in terrain.vert and multiplied into the normal matrix (`normals.glsl`).
- Without a normal map: the interpolated geometry normal is used, with per-vertex lighting.

## 4. Ambient, night, fog, colour space
- Ambient and sun colours come from the ESM weather record, interpolated by time of day (sunrise/day/sunset/
  night); the "sun" light's diffuse/specular are set from them, ambient is applied as the light-model ambient
  (`configureAmbient` and `updateAmbient` in renderingmanager.cpp, which also adds a night-eye boost). At night
  the sun light is the weather's dim night colour; the moons do not light the world (unverified, from memory).
  Interiors use the cell ambient, with `minimum interior brightness` 0.08 as a floor, and a fixed light direction.
- No colour-space conversion anywhere in the terrain shader or fog. All math is in the texture/framebuffer
  space (sRGB values treated as linear), exactly like vanilla's fixed-function pipeline. `lighting method` is a
  legacy/shaders choice, not a gamma choice. `clampLighting` clamps to 0..1 when the clamp define is set
  (default; keeps vanilla look), else just prevents negatives.
- Fog (`fog.glsl`): `color*scale + offset`, fog colour from weather. Default is linear in planar view depth
  (`radial fog` default false; true = Euclidean distance, view-angle independent). `exponential fog` default false.
  `sky blending` default false (start 0.8): fades toward the sampled sky colour near the far plane, quadratic.
  Applied last, after lighting. Underwater tint uses visibility 2500.

## 5. Vanilla reference
- Fixed function: texture * (vertex-lit colour), vertex colours from VCLR, no specular on terrain, no normal
  maps, linear fog in gamma space, per-vertex ambient+sun. Terrain texture detail is flat and saturated.
- Terrain normal-map mods (e.g. Normal Maps for Morrowind, TR terrain packs) just ship `_n`/`_nh` files beside
  the textures; they only appear in OpenMW once `auto use terrain normal maps` is on (and `_diffusespec` for
  gloss). Results stay matte: diffuse N.L shading gains relief, a small sun-only specular sheen at most. No
  environment reflections ever appear on land.

## 6. Comparison and veil candidates (PBR + IBL terrain vs OpenMW)
| Topic | OpenMW | StandardMaterial-like |
|---|---|---|
| Specular | none unless `_diffusespec`; sun only, Blinn-Phong | GGX plus environment specular for every light and for IBL |
| Environment | no IBL; ambient is a flat colour | diffuse + specular IBL (probe/sky), Fresnel `F0 + (1-F0)(1-N.V)^5` |
| Albedo | texture * vertex colour, in sRGB space | linear albedo; tone mapping applied after |
| Fog | linear/optional radial, weather colour | usually exp or atmosphere scattering |
| Blend | hard texel blend, no height blend | any |

Veil mechanisms from the comparison:
1. Fresnel on IBL specular rises toward 1 at grazing angles even for F0 0.04. The "roughness-aware" Fresnel
   (Fdez-Aguera/Karis split-sum) caps it, but with roughness 0.85-0.9 the specular lobe samples a very blurred
   sky/probe: low-contrast, sky-tinted light added on top of the albedo. That matches "strongest at grazing,
   ground takes the surroundings' tint". OpenMW adds nothing here.
2. Rough dielectrics under IBL also gain energy from multiple scattering/specular occlusion not being applied.
   No specular occlusion (AO/cavity) on terrain means the sky reflection fills crevices too.
3. Without normal maps, the geometry normal tilts little, so the whole field has the same N.V and the same veil
   in one direction. OpenMW's per-vertex model has no view-dependent term at all except sun specular (off).
4. Aerial perspective / radial fog plus sky blend: OpenMW's default fog is planar-depth linear, which varies
   little with angle. An angle-dependent atmosphere or radial fog tint at grazing angles adds a similar veil.
5. Diagnostic suggestions (not done): disable env-map specular on terrain (or `reflectance` 0 / specular
   occlusion to 0) and compare; check fog start; compare ambient colour/tone-map saturation to the ESM weather values.
- OpenMW-like reference look = diffuse-only terrain with ESM ambient and vertex colour. If the target is
  "classic" parity, terrain specular should be zero by default and enabled only for `_diffusespec`-style assets.
