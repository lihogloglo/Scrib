# Dynamic GI for Scrib: what 2024-2026 shipped titles do (research, 2026-11-01)

Second pass. The first pass (`gi-2026-11-01.md`; its sections 2-4 are reused, its recommendation is not) proposed
baked irradiance volumes. The owner rejected that: torches are carried through small lamp-lit interiors, the sun moves,
weather changes. This document asks what shipped open-world titles with those constraints do, and what that means for
an RTX 4060 laptop (RT hardware, ray queries through wgpu/Bevy), High preset, 1440p + DLSS Quality (about 960p
internal), <= 11 ms. Evidence grades: **[P]** primary (talk slides, engine docs), **[S]** secondary (press or Digital
Foundry coverage as quoted), **[I]** inference. Numbers marked (est.) are my extrapolations (method in section 2).

## 1. Shipped-title survey

| Title / engine | GI technique | RT HW needed | Fallback | Dynamic lights / sun | Published cost | Grade |
|---|---|---|---|---|---|---|
| **Assassin's Creed Shadows** (Anvil) | Per-pixel ray-traced diffuse GI: 1 ray/px at quarter res, screen-space march first, then BVH; secondary hits read a **cascaded DDGI-style probe volume** (5 cascades 16x16x8, 2-32 m spacing, ~10k probes, 73 MB) updated by rays; radiance, irradiance and visibility in octahedral atlases | yes for RTGI | **Baked** GI (33 key frames for time of day, 5 days bake, ~10 GB) on Series S and PS5 Performance; "shipping two GI systems is time-consuming" | Full: ToD (11 keyframes), 14 weather states, local lights evaluated at hits, players toggle lamps (probe G-buffer cached so relighting needs no re-trace); sun shadow per probe via shadow map or ray | RTX 4070, 1440p internal: probe update 512 probes/frame ~1 ms serial; diffuse GI 2.65 ms (quarter-res trace, incl. denoise); specular 1.6-3.2 ms. Series X diffuse 4.98 ms, PS5 7.1 ms | [P] SIGGRAPH 2025 |
| **Indiana Jones**, **Doom: The Dark Ages** (idTech 7/8) | "Fast as Hell" (Sousa): hardware RT as a *visibility tool*. 6 cascaded irradiance volumes 16^3 (+ hand-placed local 10^3), 64/32/16 visibility rays per probe, **world radiance cache** (spatial hash, 25 cm cells, ~20k entries shaded per frame), final gather 1 ray/px at 1/2 or 1/4 res reading caches only (zero shading), 2-band SH output, bilateral denoise + temporal + upscale, directional occlusion composite | **yes, mandatory** (min RTX 2060 Super / RX 6600) | none; lightmaps dropped (Eternal bakes 4-10 h per level, ~1.4 TB network storage) | Full; world-space cascaded **light grid** (16^3 x 8 cascades, 4k lights) shades the RT world | **Total GI 1.4 ms async / 1.9 ms serial (Series X, 1440p); 1.7 / 2.1 ms (Series S, 900p)**; irradiance volume update 0.08 ms; 64 MB atlas | [P] SIGGRAPH 2025 |
| **Star Wars Outlaws / Avatar** (Snowdrop) | Per-pixel RT GI plus a probe system as cache for GI and reflections (grid and octree placement), screen-space tracing first, several denoisers | yes on consoles | not found | Dynamic ("opening a door brightens an interior") | none public | [S] GDC 2024 abstract (Kuenlin); AC Shadows slides cite Snowdrop as a sibling design |
| **Crimson Desert** (BlackSpace) | Digital Foundry: ray-traced diffuse GI "per-pixel rather than probe-only", indoors and out, ToD and weather; RT "fundamental to the lighting on all platforms" | yes per DF; official minimum GPU is GTX 1060 / RX 5500 XT, mechanism unclear | unknown [I]: a non-RT or reduced path must exist for GTX 1060 | ToD and weather; local-light handling not stated | 7900 XTX native 4K ~60 fps Ultra (DF); DLSS Ray Reconstruction ~14% (5080, 4K Perf), FSR Ray Regeneration ~24% (9070 XT); default RTGI reportedly ~1/16 ray/px (weak) | [S] DF via PCGamesN/ResetEra. **No GDC/SIGGRAPH slides found** |
| **Kingdom Come: Deliverance 2** (CryEngine) | **SVOGI** voxel cone tracing, no RT hardware. CE5 doc: mode 0 voxelizes opacity only and samples bounce from (reflective) shadow maps: fully dynamic, 16 MB, single bounce, noisier for small point lights. Modes 1-2: radiance voxelized and propagated, multi-bounce, smoother, but big semi-static bounce lights cannot move freely | no | n/a (it is the software path) | Sun free in mode 0; dynamic objects bounce only near voxelized static geometry; 1-2 frame delay | CE5 doc: Xbox One 3-4 ms, average PC 2-3 ms (AO + sun bounce, no point lights, low-spec mode); AO only < 2 ms on Xbox One | [P] CE5 doc; [I] KCD2 uses the same modes ("improved SVOGI" per Warhorse via GameGPU) |
| **Avowed, Oblivion Remastered, other UE5** | **Lumen**: software (SDF) default, optional hardware RT (Avowed HWRT ~12-14% slower than software per a PCGH-linked guide). Oblivion Remastered, the closest genre match, ships Lumen with torches in interiors; DF: "dire performance problems", disabling hardware Lumen eases CPU hitching; community: HWRT -40% on a 4060 | optional | software Lumen | Full | Epic guide: 8 ms (30 fps) / 4 ms (60 fps) for GI + reflections at console 1080p; forum 2.1-2.6 ms GI only | [P] Epic budgets; rest [S], weak |
| **Metro Exodus Enhanced** (4A) | DDGI-like: rays sampled and cached each frame into a **world-space grid of probes**, infinite bounces; NVIDIA says it shares tech with RTXGI | yes | none | every shadowed light feeds GI (sun only before) | ~45 fps at 1080p on minimum RT spec (Shacknews) | [S] |
| **Cyberpunk 2077 2.x** | RT Overdrive / path tracing: RTXDI direct + path-traced indirect, DLSS Ray Reconstruction | yes (RTX 40 / 3090 class) | raster modes | Full | very heavy, DLSS 3 needed | [S] NVIDIA, DF |
| **Skyrim / Starfield** (Bethesda) | Skyrim: CK cell lighting is a six-colour ambient cube plus a bounce light. Starfield: Howard says "real-time GI"; technique unconfirmed | no | n/a | Starfield dynamic | n/a | [S]; probes vs voxels is **[I]**, unknown |
| **Elden Ring Nightreign / SotE**, Alan Wake 2 | Nothing reliable found (Nightreign has a "Global Illumination Quality" setting; ER's 2023 RT patch is AO and shadows). Not researched further | n/a | n/a | n/a | n/a | unknown |

What the survey says, ignoring marketing:

1. Every 2024-2026 title that wants **moving sun, weather and carried or toggled lights without a bake pipeline** went to
   *ray-traced probe caches* (AC Shadows, idTech 8, Snowdrop, Metro EE). The non-RT dynamic answers are CryEngine
   SVOGI (KCD2: 2-3 ms on PC, noisy for small point lights) and software Lumen (heavy).
2. The pattern is not "probes or per-pixel": it is a **probe volume updated by ray queries that doubles as the radiance
   cache**, optionally followed by a cheap per-pixel final gather. AC Shadows and idTech 8 converged on it independently.
3. Tracing is cheap, shading is not. idTech 8 traces 16-64 rays per probe per frame and spends its budget on lighting
   hit points, amortised through a hashed radiance cache.
4. Fallbacks: AC Shadows kept a **baked** system for weaker consoles and calls the dual-system upkeep a cost; id dropped
   bakes and requires RT; Crimson Desert lists a GTX 1060 minimum (mechanism unknown).
5. Leaks are never solved. Ubisoft's conclusion: "probe interpolation always leaks", mitigated with Chebyshev
   visibility, probe relocation, back-face probe disabling, conservative shadow filtering, probe AO + RTAO + GTAO.

## 2. Technique comparison for our case

Scaling used for (est.) values: the RTX 4060 laptop (115 W) has roughly half the FP32 rate and memory bandwidth of the
RTX 4070 desktop Ubisoft measured, about 1.2x Series X FP32, about 3x Series S, with stronger RT than those consoles.
960p internal is 0.44x the pixels of 1440p. Pixel-bound passes scale with pixels, probe passes do not.

### 2.1 DDGI / RTXGI-style probe volumes updated with ray queries (primary candidate)

- **Cost (est.):** probe update + sampling 0.6-1.1 ms at 960p. Basis: AC Shadows 512 probes/frame ~1 ms on a 4070 [P]
  (includes G-buffer, sun shadows, relight, convolution, relocation); idTech 8 irradiance volume update 0.08 ms and world
  sampling ~0.3 ms [P]. Ours is simpler at first (no G-buffer cache) but the 4060 is ~2x slower than a 4070. Sampling 8
  probes x (irradiance + visibility) per pixel adds 0.2-0.3 ms (est.).
- **Probe count:** AC ~10k probes (5 cascades) for 512 m; idTech 6 x 4096 (12^3 on low spec). An interior needs one
  cell-fitted grid, e.g. 30 x 10 x 30 m at 1 m = 9k probes, 512-1024 updated per frame, full refresh in 0.15-0.3 s at
  60 fps. Exterior: camera-following cascades as in AC.
- **RT hardware:** yes (ray queries). **Dynamic lights:** yes, lights evaluated at ray hits. **Moving sun:** probes
  relight each cycle; AC rescales non-updated probes by a per-probe luminance ratio to hide lag [P]. **Carried torch:**
  probes inside a dynamic light's radius get update priority (our addition; idTech and AC use round robin plus cached
  G-buffers and I found no torch-specific rule).
- **Leaks:** Chebyshev visibility (mean depth, depth^2) fixes most; thin walls leak when probe spacing exceeds wall
  thickness. Mitigations in section 1 point 5. Interiors being **separate worldspaces with their own TLAS** removes the
  exterior-into-interior leak the first pass feared (probes ignoring `RenderLayers`): our probes are not Bevy light probes.
- **Streaming per cell:** probes are runtime state, not a per-cell product; only geometry enters the TLAS as cells
  stream. Mods that add cells work with no bake. **Bake:** none.
- **Bevy 0.20 fit:** `bevy_solari::scene::RaytracingScenePlugin` already builds BLAS/TLAS and bindless materials and
  exposes `trace_ray` / `resolve_ray_hit_full` as WESL libraries; `SolariLightingPlugin` (not the scene plugin) forces
  the deferred path, so a custom pass can stay forward (**verify**: `realtime/mod.rs:62`). We would write: the probe
  update compute (ray query, hit shading, light list, octahedral irradiance + visibility atlases, hysteresis,
  relocation), a sampling function patched into vendored `bevy_pbr` at the `IrradianceVolume` priority slot
  (`pbr_functions.wesl`: lightmap > volume > env map), cascade management. Solari 0.20 sees directional lights, emissive
  meshes and the env map only; `DECISIONS.md` (2026-10-11) already plans point-light proxies as emissive spheres for RT,
  and our own light list sidesteps the limit.
- **Licence:** RTXGI-DDGI and RTXGI 2.0 carry an NVIDIA custom licence (GitHub API: NOASSERTION), not MIT, GPL
  compatibility unverified. **Write from the JCGT paper and the AC/idTech slides; do not copy the SDK.**

### 2.2 World radiance cache plus 1 ray/pixel final gather (idTech 8, Solari world cache)

- **Cost:** published 1.4-2.1 ms total on Series S/X at 900p-1440p [P]; est. 1.0-1.6 ms at 960p on the 4060 laptop
  (similar FP32 to Series X, 0.44x pixels for the ~60% that is pixel-bound), and this includes the probe tier.
- **Why consider:** removes probe-resolution detail loss and most leaks (hits read an exact-position cache), gives
  normal-mapped 2-band SH. **Why not first:** needs a denoiser and upscaler (idTech wrote bilateral + temporal +
  upscale; Bevy's Solari leans on DLSS Ray Reconstruction), adds noise risk, and still needs the probe tier as the miss
  fallback ("last cache: fall back to irradiance volumes").
- **Bevy fit:** Solari's `world_cache_*.wesl` is a spatial-hash radiance cache (2^20 entries, 15 cm base cells, LOD, 40k
  stochastic cell updates per frame). JMS55 measures 0.47 ms for 40k cell updates in Bistro (GPU unstated) and 2.65 ms
  to update every cell [S, blog 2026-04-12]; he says he is "not fully happy" with it and plans a world-space light grid.
  Reusing or upstreaming into it is the natural Phase 3.

### 2.3 Voxel cone tracing (SVOGI, VXGI)

- **Cost:** CE5 doc 2-3 ms average PC, 3-4 ms Xbox One, without point lights [P]; est. 2-3 ms on the 4060 at 960p.
- No RT hardware; sun and sky fully dynamic. Small local lights give noisy bounce in mode 0; modes 1-2 cannot move big
  lights. Doc limits: leaking, ghosting, noise; teleports need up to seconds to settle (door teleports between
  worldspaces would hit this). Voxels (tens of cm) are thicker than Morrowind's thin walls: leaks higher than 2.1.
- Streaming per cell: voxelize on load (raster, no bake). **Bevy fit:** nothing exists; write voxelization, clipmap,
  cone trace. **Licence:** NVIDIA VXGI proprietary; CryEngine code not reusable.
- **Not recommended:** worse than 2.1 on RT hardware; KCD2's choice is explained by shipping on non-RT consoles.

### 2.4 Screen-space GI (SSGI, visibility bitmask) as a supplement

- Cost ~1-2 ms at 1080p class (Motrix-Sim docs, RTX 5060 Ti [S]; no 4060 figure found; est. 0.8-1.5 ms at 960p).
- On-screen only: no light from behind walls or from an off-screen torch, edge flicker. Valid as the near-field detail
  layer (AC traces screen-space first; idTech composites directional occlusion for "high frequency details we miss") and
  as the cheapest non-RT fallback. Bevy: only a stale community crate (`bevy_ridiculous_ssgi`); we would write it.
  Not a GI system by itself for lamp-lit interiors.

### 2.5 Radiance cascades 3D, surfel GI

- **Radiance cascades 3D:** research stage (Split RC paper 2026-07: 11.5 ms on a 3080 laptop for pipeline stages, from the
  first pass); no Bevy crate. Watch only.
- **Surfel GI (EA SEED GIBS):** ships in EA Sports College Football 25 (stadiums, ToD, no precompute) [S]; I found no
  ms numbers. idTech evaluated surfels (Halen 2021) and picked probes plus a hashed cache for simplicity. More machinery
  than probes for the same result. Not recommended.

### 2.6 Lumen-style software RT and SDF/brick GI (non-RT candidates)

- **Lumen:** too heavy (budgets above) and tied to UE.
- **AMD Brixelizer GI:** compute only, no RT hardware, based on AMD GI-1.0 (screen probes + world irradiance cache) [P,
  GDC 2024]. 1.7 ms RX 7600 XT at 1080p, 2.2 ms RTX 4080 at 4K; ~200 MB at 1080p; radiance cache fed from screen space,
  so changes show only once on screen; MIT per the slide (repo licence not re-verified); HLSL/GLSL; the engine must feed
  geometry into SDF bricks every frame.
- **Godot SDFGI** (MIT, GLSL): cascaded SDF voxels + probes; docs call it demanding, need thick walls, "Use Occlusion"
  costs more, dynamic lights fine, dynamic occluders not [P]. Structurally it is 2.1 with a voxel tracer instead of ray
  queries. Role for us: the **non-RT tracer backend** (section 3).

### 2.7 Summary

| Technique | Cost 4060 @960p (est.) | RT HW | Torch / sun | Thin-wall leaks | Bake | Bevy fit |
|---|---|---|---|---|---|---|
| Ray-query DDGI cascades (Tier A) | 0.6-1.1 ms | yes | yes / yes | medium (visibility + AO) | none | scene plugin + own passes |
| + radiance cache + final gather (Tier B) | 1.0-1.6 ms total | yes | yes / yes | low | none | Solari world cache as base |
| SVOGI | 2-3 ms | no | weak for small lights / yes | high | none | write all |
| SSGI | 0.8-1.5 ms | no | on-screen only | n/a | none | write |
| Brixelizer / SDFGI class | 1.5-2.5 ms | no | yes / yes | high | none | port (MIT) |
| First pass: baked volumes | ~0.2 ms | no | no carried-torch bounce / sky via vis | medium | minutes | `IrradianceVolume` as is |

## 3. Recommendation

**Primary (High on RT hardware): cascaded probe volumes updated by ray queries, in the AC Shadows / idTech 8 pattern,
built in two steps.**

- **Tier A (High, 0.6-1.1 ms est.):** DDGI-style octahedral irradiance + visibility atlases. Interiors: one cell-fitted
  grid at about 1 m (camera cascades for huge interiors). Exteriors: 3-5 camera-following cascades, spacing roughly
  2-32 m. 32-64 rays per probe, 512-1024 probes per frame, hysteresis tuned for torch lag. Lights: our own world-space
  light list (torches, lamps, sun) evaluated at hits, so Solari's missing point lights do not matter. Sky: miss rays
  sample the atmosphere env map (as Solari does). Sun shadow: a shadow ray. Multi-bounce by reading last frame's
  probes. GTAO stays on indirect; per-probe back-face disable and relocation.
- **Tier B (High+, about +0.5 ms est.):** hashed radiance cache and a quarter-res final gather, only if Tier A leaks or
  looks flat in the owner's visual check. Start from Solari's world cache; consider upstreaming point-light sampling.
- **Keep from the first pass:** stock Bevy where possible, a vendored `bevy_pbr` patch for the lookup (no render-world
  injection), AO touches indirect only, contact shadows for direct.

**Fallback for non-RT GPUs:** do not build a second GI system now. Phase 1: current ambient + GTAO (+ bent normals),
zero cost. Phase 2, only if non-RT hardware matters: a **tracer backend swap** under the same probe atlas, sampling and
light code, using a voxel/SDF DDA tracer in the manner of Godot SDFGI (MIT) or Brixelizer GI (MIT, 1.7 ms on a 7600 XT
at 1080p), est. 1.5-2.5 ms. That keeps one look, which Ubisoft names as the costly part of its dual setup. The first
pass's baked sky visibility + ambient cube remains a valid cheap fallback if the owner prefers zero runtime cost over a
second tracer, but it is a second product with a bake step and a different look.

**Phase order**
1. **Spike (one slice):** one interior, one hand-written ray-query probe pass on top of `RaytracingScenePlugin`: 16x8x16
   probes at 1 m (2k probes), 64 rays, 512 probes/frame, a torch and the sun from a light buffer, sampled through a
   patched `IrradianceVolume` slot. Print ms per pass on the 4060 at High. In the same room also record Solari GI with
   ReSTIR off (its world cache already works with the emissive-sphere proxies), as a free comparison. Verify: the
   forward renderer survives without `SolariLightingPlugin`; `ExtendedMaterial` meshes yield albedo in hits; alpha-tested
   leaves; behaviour with no RT feature (graceful off).
2. Interiors: per-worldspace TLAS, probe relocation, dynamic-light priority, door-teleport reset.
3. Exteriors: cascades, sky miss, terrain RT proxies (already planned for Ultra), BLAS streaming per cell.
4. Tier B and the non-RT backend only on the owner's go.

**Cost per preset (est., replaced by spike numbers):** Ultra with Solari unchanged (separate budget). High: Tier A,
0.6-1.1 ms. Medium: Tier A at half update rate and fewer cascades, 0.3-0.6 ms. Low / non-RT: ambient + GTAO.

**What the first pass got right:** the constraints; `IrradianceVolume` has no visibility and ignores `RenderLayers`;
GTAO only touches indirect light; no reusable crate exists; Lumen and radiance cascades are too heavy; baked sky
visibility is the correct trick *if* baking. Solari is not the High path today (no point lights, forces deferred, needs
DLSS Ray Reconstruction). **What it got wrong:** it treated ray queries as Ultra-only and priced RT GI from the 2019
DDGI paper (6 ms), whereas AC Shadows and idTech 8 publish sub-2 ms RT probe/cache systems on weaker hardware than
ours; it chose a bake that cannot express a carried torch or a lamp a mod toggles; and it added a per-cell bake product
and fallback logic the dynamic design makes unnecessary.

## 4. Risks and open questions

- **wgpu ray queries are `EXPERIMENTAL_RAY_QUERY`** (wgpu 29 in the registry): no RT pipelines, API may shift;
  Vulkan/DX12/Metal only. Same risk as Ultra.
- **VRAM:** the 4060 laptop has 8 GB. AC uses ~320 MB BLAS/TLAS in an urban scene and 73 MB of probes; idTech a 64 MB
  atlas. Morrowind exteriors need simplified RT meshes (terrain proxies are in the terrain spec); budget per preset.
- **BLAS build cost while streaming cells** and per-frame TLAS rebuild (AC does it async). Solari 0.20 made scene
  management retained, which helps. Measure on the Balmora walk.
- **Materials:** Solari's binder reads `StandardMaterial` assets; our meshes use `ExtendedMaterial<StandardMaterial,_>`
  and custom materials. Alpha-tested foliage, common in Morrowind, needs opacity handling (AC bakes opacity per mesh).
  Skinned characters are not in Solari's scene (known gap); acceptable, AC excludes them too.
- **Torch lag and flicker** from hysteresis without a per-pixel denoiser; may need Tier B or a faster-updating
  near-torch cascade.
- **Thin walls:** Morrowind tents and modular kits may be thinner than 1 m probe spacing; visibility + relocation + GTAO
  reduce leaks, Ubisoft says they never vanish.
- **Licence:** RTXGI is not MIT, write from the paper. Brixelizer's MIT is stated on a slide; check the repo before any
  port.
- **Evidence gaps:** Crimson Desert has no primary technical source found (DF describes the output; "probe-free" is DF's
  inference). KCD2 SVOGI is inferred from CE5 docs and developer statements. Starfield, Elden Ring, Alan Wake 2 not
  established. Every 4060 figure is an extrapolation; the spike replaces them.

## 5. Sources

- Ubisoft Montreal, "Ray tracing the world of Assassin's Creed Shadows", SIGGRAPH 2025 Advances course:
  https://advances.realtimerendering.com/s2025/content/Advances%202025%20-%20Raytracing%20the%20world%20of%20Assassin's%20Creed%20Shadows.pdf
- Tiago Sousa, "Fast as Hell: idTech 8 Global Illumination", SIGGRAPH 2025:
  https://www.advances.realtimerendering.com/s2025/content/SOUSA_SIGGRAPH_2025_Final.pdf (course page
  https://advances.realtimerendering.com/s2025/)
- AMD, "GI with FidelityFX Brixelizer", GDC 2024: https://gpuopen.com/download/GDC2024_GI_with_AMD_FidelityFX_Brixelizer.pdf
- Kuenlin, "Raytracing in Snowdrop: An Optimized Lighting Pipeline for Consoles", GDC 2024:
  https://gdcvault.com/play/1034763/Advanced-Graphics-Summit-Raytracing-in ; Massive on Snowdrop RT:
  https://massive.se/article/snowdrops-ray-tracing-shines-a-light-on-pandora
- CryEngine 5 SVOGI doc: https://cryengine.com/docs/static/engines/cryengine-5/categories/23756816/pages/25535599 ;
  KCD2: https://www.dsogaming.com/pc-performance-analyses/kingdom-come-deliverance-2-pc-performance-analysis/ ,
  https://en.gamegpu.com/news/igry/kingdom-come-deliverance-2-ispolzuet-uluchshennuyu-versiyu-cryengine-i-podderzhivaet-nvidia-dlss-i-amd-fsr
- Crimson Desert: DF as quoted at
  https://www.resetera.com/threads/df-crimson-deserts-blackspace-engine-rich-on-rt-without-compromising-performance.1449508/ ,
  https://www.pcgamesn.com/crimson-desert/pc-ray-tracing-upgrade ; minimum spec
  https://www.tweaktown.com/news/110416/crimson-desert-pc-specs-show-that-it-will-run-on-a-geforce-gtx-1060/index.html
- Doom TDA / Indiana Jones: https://www.xda-developers.com/doom-dark-ages-id-tech-hands-on/ ,
  https://wccftech.com/doom-the-dark-ages-reduced-space-turnaround-rtgi/
- Oblivion Remastered: https://wccftech.com/oblivion-remastered-is-an-impressive-remaster-with-dire-performance-problems-says-digital-foundry ;
  Avowed: https://www.sportskeeda.com/gaming-tech/avowed-ray-tracing-on-vs-off-visual-comparison-performance-impact
- Metro Exodus Enhanced: https://www.nvidia.com/en-us/geforce/news/metro-exodus-pc-enhanced-edition-ray-tracing-dlss/ ;
  Cyberpunk Overdrive: https://www.nvidia.com/en-gb/geforce/news/cyberpunk-2077-ray-tracing-overdrive-update-launches-april-11
- Starfield: https://gamesradar.com/starfields-lighting-will-adapt-to-the-planet-youre-on ; Skyrim CK lighting:
  https://skyrimck.uesp.net/wiki/Lighting_Template
- EA SEED GIBS: https://www.ea.com/seed/news/siggraph21-global-illumination-surfels ; Godot SDFGI:
  https://docs.godotengine.org/en/stable/tutorials/3d/global_illumination/using_sdfgi.html
- NVIDIA RTXGI: https://github.com/NVIDIA-RTX/RTXGI , https://github.com/NVIDIAGameWorks/RTXGI-DDGI ; Epic Lumen performance
  guide: https://dev.epicgames.com/documentation/unreal-engine/lumen-performance-guide-for-unreal-engine
- Solari world cache numbers: https://jms55.github.io/posts/2026-04-12-solari-bevy-0-19 ; Bevy 0.20: https://bevy.org/news/bevy-0-20/
- Local: `bevy_solari-0.20.0/src/{lib.rs,scene/mod.rs,realtime/mod.rs,realtime/world_cache_*.wesl}`;
  `superfargoth/docs/DECISIONS.md` (2026-10-11 sky and render path); first pass `gi-2026-11-01.md`.
