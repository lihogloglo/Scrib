# OpenMW Clockwork: code and shader read (2026-10-25)

Research note, slice `clockwork`. Nothing built or run. Supersedes the "Clockwork behaviour, summarised (docs only)"
sections of [borrow-audit-2026-10.md](borrow-audit-2026-10.md) and the behaviour-only note in
[shadows/README.md](shadows/README.md) section 2: this time the C++ and the GLSL were read.

**Source.** `https://gitlab.com/OursCodeur/openmw.git`, branch `clockwork-alpha`, **`c5e5a9a6362c35512823884753d2d5284af204fa`**
("Upgrade resolution, live sliders, GI glide and UI fixes", 2026-10-19, newest of the fork; the `gpu-*` branches are
older feature branches from Sep 2026 that fed it; upstream `master` is `71fc0a4`, 2026-10-05, without Clockwork).
Shallow clone: `C:\cargo-target\refs\openmw-clockwork` (one commit deep; paths below are relative to it).
Author of the fork: OursCodeur (Thomas Portal). Licence: OpenMW's GPL-3.0 (`LICENSE`); the files we would port are GPL-3.0
with no per-file header, so provenance = file + commit above. Two third-party items inside it need their own check before a
code port: `lib/core/tonemap.glsl` (curve of `exposure.omwfx` 1.3 by wareya and Hrnchamd) and
`compatibility/water.frag` (derived from martinsh's "Blender GLSL Water"); neither is in the ranked list as code.

Size: `apps/openmw/mwrender/clockwork/**` 37.6k lines C++ (Vulkan renderer: `frame/framedriver.cpp` 6.2k,
`scene/gpuscenevk.cpp` 6.3k, `frame/gpuscenevkframe.cpp` 7.1k) + `apps/openmw/mwrender/gpuscene*.cpp` + about 20k lines of
GLSL under `files/shaders/{compatibility,core,lib,clockwork}`. The renderer files carry almost no prose; the shaders' short
comments and `docs/source/reference/modding/settings/*.rst` are the documentation.

## Verdict

Clockwork is **a GPU-driven Vulkan forward renderer for Morrowind's content, not a modern look**: the look is still
Morrowind's (display-space additive Blinn-Phong/vertex-colour lighting, OpenMW's textured sky and clouds, martinsh-style
planar-reflection water, OpenMW's terrain quadtree, MSAA only: **no TAA, no upscaler, no colour grading**; its tonemap is
one `exposure.omwfx` curve plus a 4-tap bloom). We are well ahead on sky, clouds, water, terrain, far field, anti-aliasing and
the PBR base. Where it is better than us is **inside the Morrowind-specific lighting/shadow problems we have open**:
(1) a sun-shadow filter that works in **world units** (fixes our H2 "penumbra jumps between cascades"), 8 blocker taps +
8/16 filter taps, small enough to port; (2) a **persistent GPU instance table with per-view compute culling** (frustum,
per-instance size reach with dissolve, frame-late Hi-Z occlusion, indirect draws, bindless materials) for every view including
sun cascades and lamp cubes: the one existing proof of the architecture H1 points at, but with **no published frame times**
and on OSG/Vulkan, so it is a design reference, not a ready fix; (3) a **K-nearest-layer order-independent transparency**
buffer with a haze tail, relevant to H3's smoke/ash/translucent cases; (4) a complete, cheap **rain/dust/snow surface
model** keyed to a top-down shelter map, plus rain-ring/crown functions we can port nearly verbatim; (5) a mature set of local
light shadow rules (32-bit cubes, selection by contribution including off-screen lights, 30 Hz moving-object overlay,
source-radius PCSS, actor slot shadows with floor occluders) that mostly confirm our own lamp-cache design and give us the
softness and fixture rules we lack. Its published GPU costs are small (AO 0.10 ms, fog 0.24 ms, 47 cube lights 0.13 ms, all
RTX 4090 at 1080p); expect 4-5x on the 4060 Laptop (estimate). No bounce-GI, horizon-map-AO or cloud number exists. **Take
techniques and a few small functions; do not port the renderer.**

Their numbers are the settings docs' (`docs/source/reference/modding/settings/*.rst`); no benchmark script or table in the
repo gives draw-call counts or frame times (`scripts/HOWTO-benchmark.md` is OpenMW's generic stats tool).

## How it is built (short)

- **Frame**: `frame/framedriver.cpp` records a Vulkan frame; `scene/gpuscenevk.cpp` owns buffers and compute dispatches;
  `components/vulkan/` is the device layer. Classic OSG scene graph still drives loading; objects become *templates* and
  *instances* (`gpuscene.hpp` `GpuTemplate`, `Instance`, `createChunk`) in GPU records. Each "upgrade" has a master switch whose
  false state reproduces OpenGL; every feature at 0 "compiles nothing" (shader permutations by `@define`, SPIR-V cache).
- **Culling/draw** (`files/shaders/compatibility/gpuscene_cull.comp` 452 lines, `gpuscene_commands.comp`, `gpuscene_slabs.comp`,
  `gpuscene_slabpack.comp`, `gpuscene_orderedstream.comp`, `lib/core/gpuscene_occlusion.glsl`, `gpuscene_downsample.comp`): one row
  per instance, one *reason* per view (dropped by frustum, small feature, occlusion, night/day switch...). Hi-Z pyramid from view 0,
  "seed draws occluders, resolve tests the rest", one frame late; sun cascade casters test against view 0's pyramid; per-instance
  size rule from the eye each frame (`span = reach / distance`, drop below 1, dissolve over the last fifth); births, night/day
  switch children and chunk swaps cross-fade (dissolve, no pop); blended buckets sort on the GPU into log-spaced depth *slabs*
  (median key per command). Materials are bindless (`material_fragment_bindless.glsl`, `gpuscenetextures.cpp`).
- **Lighting**: forward, all lights through clusters (`core/lighting/{cull,bound,slots}.comp`); the cluster cull keeps the highest
  `brightness x projected area` per cluster and fades the ones near the cut (`cull.comp`), a "bound" pass uses the frame's depth to
  tighten long lists, a "slots" pass handles actor contact capsules. Light adds in display space (as vanilla), reach = 1.0 x radius,
  physical falloff capped at 1.53, carried lights 8x (settings docs).
- **Order-independent transparency** (`lib/core/layers.glsl`, `layerpack.glsl`, `layerfront.glsl`, `clockwork/layers.frag`): each pixel
  keeps its nearest K blended fragments as packed 64-bit words (depth key, coverage, colour) with 64-bit atomics (optional pixel
  interlock), overflow is summed as *haze behind the stack* (optical depth), 16x16-pixel block flags skip empty composites, MSAA
  partial layers composite per sample.

## Ranked table

Costs are Clockwork's (RTX 4090, stated resolution) or "none published". "Our state" is from the specs named in the task.
Effort for Bevy 0.20/WGSL: S under 0.5 d, M 1-2 d, L 3+ d. Phases are ROADMAP's (0-4). "Code" = port the function with
provenance; "technique" = re-implement from the description.

| # | Item | Clockwork files @ `c5e5a9a6` | Technique | Their cost | Our state | Where it goes | Effort | Port | Phase |
|---|---|---|---|---|---|---|---|---|---|
| 1 | **Sun shadow penumbra in world units** (H2) | `lib/light/sunsoft.glsl` (132 lines), `sunblockers.glsl`; settings `sun shadow softness`, `sun shadow weather`; `gpuscene_worldsoft.comp` (screen-space share) | 8-tap blocker search (golden-angle disc, per-pixel IGN rotation), weighted blocker height (squared weights, tallest blockers win), receiver-plane depth slope from `across/down` so taps do not self-shadow, penumbra = sun disc (0.0087 rad) x caster height capped at 30 m, same width in world units in every cascade, minimum 1.5 texels, 8 or 16 filter taps, widened by haze/cloud, shadow fades to 60 % under overcast | none published | Bevy directional PCSS in cascade texels: penumbra jumps between cascades (H2 symptom) | framework: vendored `bevy_pbr` shadow-filter patch (same place as `superfargoth-shadow-early-out-lamp-filter.diff`); weather widening from `sf_weather` | M | code (GLSL to WGSL; keep the plane-slope and clamp logic) + the weather widening | 1 (ROADMAP 1.4/1.12) |
| 2 | **Persistent GPU instance table, compute culling per view, indirect draws** (H1) | `compatibility/gpuscene_cull.comp`, `gpuscene_commands.comp`, `lib/core/gpuscene_types.glsl`, `instance.glsl`; `mwrender/gpuscene.cpp` (3.4k), `gpuscenedraw.cpp`, `clockwork/scene/gpuscenevk.cpp` | records uploaded once per chunk (`createChunk`, rebake in place); one compute dispatch per view fills per-view visibility + indirect commands; reason codes for debugging; sun cascades and cube faces cull from the same table; bindless textures remove per-material bind groups | **none published** (no draw-call or ms figures in repo) | Bevy GPU preprocessing but CPU-side per-view specialize/queue/batch/encode; `NoCpuCulling` tried: 81 ms; H1's hypothesis is cost = batches x views | framework; needs a vendored Bevy patch or custom pass; design reference for the H1 "first step" (count batches/work items per view) | L (XL) | technique only (OSG/Vulkan structure does not transfer) | 1 (ROADMAP 1.2/1.9), after the H1 measurement |
| 3 | **Per-instance reach rule with dissolve; bake-time reach heuristic** (H1, far field) | `gpuscene_cull.comp` lines ~330-350 (`staticReach`, `fadeReachSpan`, `fadeBornSeconds`); `objectpaging.cpp:641` `pagingMinSize` (merge-benefit and cost multiplier); settings `object paging min size*` | reach baked per instance into `material.w`; cull drops when `reach/distance < 1`, dithered fade over the last fifth; reach heuristic = min size scaled by `share = (1-cost)*mergeFactor + cost`, `cost = min(1, mergeCost*mult/mergeBenefit)` | none published | Bevy `VisibilityRange` + dither exists; reach policy per object is our bake (far-field.md, MGE note rows 1-2 use a bake-time size threshold) | framework bake (`sf_lod::bake`) gets the cost-aware reach formula; Morrowind default constants (min size 0.01, merge factor 250, 0.3, cost 25) go to bevyrrowind data | S | code (the 6-line formula, upstream OpenMW too) + technique (per-frame test) | 1 |
| 4 | **K-nearest-layer OIT with haze tail** (H3: RO-B smoke, WE-5 ash, RO-6 translucents) | `lib/core/layers.glsl`, `layerpack.glsl`, `layerfront.glsl`, `clockwork/layers.frag`, `waterside.glsl`; `compatibility/gpuscene_layerlight.comp` (sun/fog share per layer) | per-pixel slots as 64-bit packed words (depth key, coverage, E5B9G9R9-style colour), atomic insert, shed layers accumulate as optical-depth haze, block flags, water-plane split (two rounds), layers lit by sun + fog volume | none published | Bevy `OrderIndependentTransparency` exists but our custom passes (clouds, froxels, precip, particles, water snapshot) have implicit order (H3) | framework `render-order.md`: one OIT composite pass with a written position in the graph; needs 64-bit atomics in wgpu (check feature support) or a 32-bit variant | L | technique (haze tail and block flags are the ideas worth keeping) | 1/4 (after the order contract) |
| 5 | **Surface weather: wet / puddle / dust / snow with a shelter map** | `lib/light/surface.glsl` (624), `compatibility/puddles.frag`, `terrain.frag`, `objects.frag`; `clockwork/scene/surfaceweather.{cpp,hpp}`; settings `surface weather`, `surface drying time` | state = few scalars per weather *region* advanced in game minutes (not saved; drying 120 min, snow 3x, dust 4x); exposure from a top-down "highest surface" map read with `textureGather` over a tent, plus leaf shelter on a B-spline lattice (`roofLeaves`); walls get world-plane streaks; rain crowns on two offset grids (`surfaceCrowns`), rings via `rain_ripples.glsl`; ash/blight dust uses the region's ground textures; puddles on the colour alpha; optional screen-space puddle reflection (`puddles.frag`) | none published | weather.md "Surface state" is the heavy design (clipmap, persistent grid); borrow-audit proposes 2 scalars + our occlusion map | framework `WorldMaterial` hook for wet/puddle/snow/dust and rings; **ash/blight region textures and Morrowind weather names -> bevyrrowind** | M | code for `surfaceCrowns`, `surfaceNoise`, `surfaceExposure` shape (WGSL); technique for the rest | 4 (SU1/SU2) |
| 6 | **Local-light shadow rules** (H2: SH-13 grain, thin casters, lamp reach) | `lib/light/worldshadows.glsl`, `worldfilter.glsl`, `cubeface.glsl`, `localshadows.glsl`, `gpuscene_worldroom.comp`; `sceneutil/worldshadows.hpp`; settings `local world shadow*`, `local shadow source radius`, `light fixture shadows`, `local shadow strength` | cube per light, **32-bit depth**, 512 px faces (1024 for moving lights), redrawn only when a caster near it changes; slots chosen by contribution at the camera **including off-screen lights**, hysteresis ("keeps its cube until a much stronger one arrives"), crossfade on takeover; moving objects redrawn at 30 Hz over a copy of the still depth; source-radius PCSS (8 units fixed, 2 carried, meshless lights larger), "hard at the caster's foot"; fixture mesh shades its own light with a size-based fade; shadow subtracts a second time (`local shadow strength`) so a dominant light's shadow stays dark | 8 lights 0.24 ms tomb / 0.06 ms room, 47 lights +0.13 ms, 74 MB (4090, 1080p, 256 px) | lamp cache + vendored #26149/#26161 + 2x2 PCF; 1024 cube array; 0.35 ms CPU per face view; "lamp shadows short-reaching, grainy" | framework `sf_shadow_cache_bevy`: port the **filter** (`worldfilter.glsl`: mean blocker, grazing-slope limit, source radius) into the existing lamp-filter patch; fixture shade and strength rules -> bevyrrowind (NIF fixture detection) | M (filter), L (own cube atlas) | code (filter) + technique (rules); atlas: skip while stock Bevy is the decision | 1 |
| 7 | **Distant terrain + object shadows with incremental redraw** (H2 SH-11) | `clockwork/scene/terrainhorizon.{cpp,hpp}`, `lib/light/distantshadows.glsl`; settings `distant terrain shadows`, `distant object shadows`, `distant terrain shadow quality` | CPU threads bake per cell a horizon height for 16 sun directions (southern half) at 128-unit spacing (136 KB/cell, ~27 ms/cell/thread; low: 8 dirs, 256 u, 3.5 ms), 3D texture; lifted receivers see past near ridges; plus one 4096 px static-only sun map to 81920 units, redrawn **one ninth per frame**, only when the sun moved 0.02 deg / the camera 1/16 of reach / 0.5 s after new objects, previous map read meanwhile, crossfade on swap | terrain horizon < 30 us main pass (2560x1440, 4090); object map 0.05 ms average, 0.1-1.1 ms per ninth; 128 MB | terrain shadow-height map built (weather.md "Distant shadows"); the far static sun map was **deleted** (reset A2) because of flashes | framework; compare their redraw triggers and crossfade with our terrain map before re-adding any far object map | S (compare), M (rebuild) | technique | 1 (SH-11) |
| 8 | **AO with sun and lamp contact rays; actor capsules** | `lib/light/screengather.glsl`, `ambientocclusion.glsl`, `occlusionblur.glsl`, `capsules.glsl`, `gpuscene_bounce.comp` (occlusion rows), `core/lighting/slots.comp` | half-res slice-marching occlusion on ambient only (strength 0.8), same pass marches short rays toward the sun and the strongest lamps (contact shadows), depth-aware blur, 16 nearest actors darken ambient under them via 3 capsules (cylinder + floor snap, 0.25 s fade-in) | AO 0.10 ms (1080p, 4090; 0.04 Vivec); capsules 0.04 ms with 400 actors | Bevy GTAO (Msaa off, motion prepass) + sun/moon contact shadows 0.3 m; lamps no contact | framework (capsule AO term in ambient); lamp contact rays optional | S (capsules) / M (lamp rays) | technique | 3 (actors), 1 (image quality) |
| 9 | **Fog volume: local-light halos, wisps, storm curtains** (WE-5) | `compatibility/gpuscene_fogfill.comp` (737), `gpuscene_fogintegrate.comp`, `gpuscene_fogspread.comp`, `lib/light/fogvolume.glsl`, `fog.rst` | froxel 160x90x64 (medium), sun by cascades, local lights scatter their own halo with falloff and are shadowed by their cubes, "spread" by fog in front, interior keeps 15 % of the glow, height term (e per 1024 u, capped 4x), 3 noise bands of wisps (up to 30 %, 90 % in storms, stretched 4x across wind, 1200 u/s), storm in-scatter colour (ash brown-grey, blight red), sky-light fill | 0.24 ms (4090, 1080p, high), 0.21 ms tomb with 50 lights | froxels built (+0.16 ms measured, 4060, 1440p), god rays, ground fog banks | framework (storm wisps/curtains, light halos); ash/blight colours -> bevyrrowind weather records | S-M | technique | 4 |
| 10 | **Aerial perspective constants** | `lib/light/aerial.glsl` (136), `fog.rst` | two scale heights (molecules 8 km, aerosol 1.2 km) from water level, 0.1 perceived m per unit, per-weather visibility 40/25/15/10 km, 5 % contrast, denser of air vs fog (no kink), haze colour = rendered sky in that direction, sun-lobe warm tint | folded into the fog numbers above | height fog + aerial in weather.md | tune our atmosphere with these as a Morrowind data table; the rest we have | bevyrrowind data (visibility per weather, 0.1 m/unit) | S | data + technique | 4 |
| 11 | **Specular antialiasing** | `lib/light/specularaa.glsl` (Tokuyoshi-Kaplanyan 2019, `shaders.rst`) | widens the NDF where the normal varies within a pixel and lowers the peak; ~40 lines | 0.06 ms (4090, 1440p Vivec) | not in any spec (grep: only this audit mentions it); unknown whether Bevy 0.20 PBR does it | framework material extension | S | code (check Bevy first) | 1/4 |
| 12 | **Cheap light-return ("indirect light" fallback) and interior tuning values** | `shaders.rst` `indirect light`, `interior fill` 1.35, `minimum interior brightness` 0.08, `physical falloff cap` 1.53, `carried light reach multiplier` 8; `lib/light/clamp.glsl` | lights without a cube return 0.5 x of their level, spread evenly across reach, 1/3 of reach away, AO-dimmed, fading to zero at the edge; the full path traces a voxel grid injected from the cubes | none published | interior ambient via LightCalibration + ambient; no light-return | bevyrrowind interior look (values) ; framework optional "room fill" term | S | data + technique | 2 (playable slice interiors) |
| 13 | **Voxel + world-point GI (bounce)** | `gpuscene_voxelinject.comp` (677), `gpuscene_bounce.comp` (977), `lib/light/voxelgrid.glsl`, `gialbedo.glsl` | cube depth maps injected into a 2-level voxel grid (light, normal, place per voxel); world points hashed in a table trace voxel rays, a-trous denoise; sun map for outdoor bounce; "glide" while the grid scrolls | none published (quarter-res gather) | Solari RT on Ultra; raster has none | skip (Solari covers it); revisit only if raster interiors look flat after row 12 | L | skip | - |
| 14 | **Cloud shadows, hemispheric sky light, backlit foliage, sky visibility** | `lib/light/weather.glsl` (169), `sun.glsl`, `groundcover.frag` | cloud texture projected along the sun (1 fetch); ambient linear in normal (up +20 %); leaf translucency toward the eye; ambient dimmed by horizon map under overcast | none published | cloud shadow map built (render-order row 0c), sky-lighting.md implemented | skip all but **backlit foliage** (Morrowind flora-name rule -> bevyrrowind) and overcast sky visibility from our terrain height map (S) | S | technique | 4 |
| 15 | **Rain ripple tiers, caustics, underwater shafts** | `lib/water/rain_ripples.glsl` (128), `caustics.glsl` (87), `water.frag` | 2 or 5 ripple grids by tier, ring-only ripples; Voronoi-line caustics from two warped moving point networks, using the refracted sun position; shafts shaded by sun cascades | none published | FFT ocean + SSPR designed; caustics designed (water.md bake step `water.caustics`) | framework water; ripple tiers as quality levels | S | code for the ripple grids (not `water.frag`) | 4 / M6 water |
| 16 | **Frame hitch attribution and capture** | `components/diagnostics/*`, `clockwork/frame/framepacing.hpp`, `diagnostics.rst` | hitch = frame > 2x median of last 600 or > 33 ms, labelled with the pass that owned it (`gpu.shadow`); F7 snapshot / F8 trace to one `.omwcap` (settings, log tail, frame, PNG text) | CPU only | `sf_diag` budget gates in progress; borrow-audit row 9 (Mudcrab) | framework `sf_diag` | M | technique | 1 |

Rows not listed because they are **not better than ours**: sky/clouds (`sky.frag` is OpenMW's textured sky plus lightning
channel and storm-dust sun disc), terrain (`terrain.frag` is quadtree + composite maps + geomorph), water base (planar RTT 512,
`reflection detail` 0-5), tonemap/bloom/auto-exposure (`tonemap.glsl`, `postbloom.frag`, `postluminance.frag`: one curve, 4-tap
bloom with soft knee and cap 8, log-luminance adapt), anti-aliasing (MSAA only; `alpha-to-coverage` not applied under Vulkan),
no TAA/DLSS/FSR/XeSS anywhere (grep of `files/`, `clockwork/`, settings docs finds none), no colour grading.

## Per-question answers

- **Lighting model**: Morrowind's vertex-colour/material modes, Blinn-Phong in display space, clustered, 1.0 x radius reach,
  physical falloff capped, per-pixel only when forced. We are PBR and linear; do not copy. Take the *rules* (carried light x8,
  negative lights, reach), which are Morrowind-specific -> bevyrrowind `LightCalibration`.
- **Shadows**: 3 cascades x 2048^2 to 16384 units (about 230 m), practical split lambda 0.5, fitted to casters/receivers in view,
  polygon offset + 1-unit normal offset; our 3 cascades reach ~150 m on High. See rows 1, 6, 7.
- **AO**: row 8. **GI/ambient**: rows 12-14. **Sky/atmosphere/clouds**: classic; only aerial constants (row 10) and storm
  fog (row 9). **Water**: classic; rows 5, 15. **Wetness/puddles/snow**: row 5. **Post**: nothing to take.
- **LOD/paging/batching** against `bevyrrowind/docs/research/mge-xe-code-2026-10-25.md`: Clockwork keeps OpenMW's object paging
  (chunk the world, decide per object by size, cost-aware merge benefit) **but no longer merges geometry**: a chunk is a list of
  GPU instance records, culled and drawn indirectly. That is the opposite trade to the Greatness7 fork (merge + atlas, fewer
  entities) and the same *selection* idea as stock MGE XE (size threshold, quadtree); it removes the per-draw CPU cost by moving
  visibility to the GPU instead of reducing draw count. For H1 this is the second path: if the per-view Bevy cost really scales with
  batches x views, then either fewer batches (merge + atlas, MGE fork) or a single GPU-resident table with few views (Clockwork)
  fixes it; Clockwork's own evidence is a working renderer, not a number. Horizon culling (MGE fork, ~41 % fps in dense areas) is
  **absent** in Clockwork (it has Hi-Z instead); the two are complementary.
- **H-problem flags**: H1 rows 2, 3; H2 rows 1, 6, 7 (+ 8 for SH-20 actor contact); H3 row 4 (and the shed-layer haze idea for
  ash/smoke), row 9 for WE-5 colour.
- **Generic (framework) vs Morrowind-specific (bevyrrowind)**: framework: 1, 2, 3 (formula), 4, 6 (filter), 7, 8, 9 (volume), 11,
  15, 16. bevyrrowind: weather constants and ash/blight textures (5, 9, 10), fixture/flora-name detection (6, 14), interior and
  light tuning values (12), per-weather visibility table (10).

## Recommended order (for the orchestrator)

1. Row 1 (world-unit PCSS): contained, directly answers the H2 symptom owner reported; M.
2. Row 6 filter + row 7 trigger/crossfade comparison: finish H2 without owning an atlas.
3. Row 3 formula: free; check it against the far-field bake before the H1 batching work.
4. Row 2: read `gpuscene_cull.comp` + `gpuscene.hpp` again when the H1 trace is in; decide merge+atlas vs GPU table with the numbers.
5. Row 4 once `render-order.md` has its written contract (H3).
6. Row 5, 8, 9, 11, 12 as Phase 3/4 slices.

## Caveats

- No Clockwork frame times at all for the culling/draw path or the GI; every cost above is a settings-doc figure on an RTX 4090.
- The branch is "alpha" and rebases often (`1dba3a92` rebase on 2026-10-18); re-check the commit before porting.
- I did not build or run it. Reading was targeted: `gpuscene_cull.comp`, `layers*.glsl`, `surface.glsl`, `sunsoft.glsl`,
  shader headers of the light/fog/GI compute passes, settings docs, `objectpaging.cpp` and the scene/frame headers; the large
  `.cpp` files (framedriver, gpuscenevk*) were only skimmed for structure.
