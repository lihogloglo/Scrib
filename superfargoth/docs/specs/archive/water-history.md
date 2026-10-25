# Water: as-built history

Moved out of [water.md](../water.md) on 2026-10-26 (doc budget, S11). Append-only; grep it.

## Research summary (2026-10-13)

Reuse order (CLAUDE.md rule 8): Bevy, then crates, then our own.

**Bevy 0.20** (read in `~/.cargo/registry/src/*/bevy_*-0.20.0`):
- No water. `StandardMaterial` has screen-space **transmission** (`specular_transmission`, `ior`,
  `thickness`, attenuation): the `Transmissive3d` phase snapshots the opaque colour and draws
  transmissive meshes. It samples colour only (no scene depth without a depth prepass), and its system
  is ordered only "after opaque, before transparent", the same as the atmosphere's `render_sky`, so
  whether the snapshot contains the sky and aerial perspective is unspecified. It writes no motion
  vectors. Not enough for depth colour, shore fades or DLSS, but the mechanism (snapshot, then draw) is
  the one we use.
- `ScreenSpaceReflections` needs `DepthPrepass` + `DeferredPrepass`; the exterior camera has neither
  (far-field.md pitfall 2; weather.md open question 4).
- What we do reuse: the `Atmosphere` aerial-view LUT and environment map (sky reflection), the PBR
  lighting functions (GGX sun specular, clustered point lights for torches reflecting at night,
  `fetch_directional_shadow`), `DirectionalLightTexture` cloud shadows, the core-3d schedule (our
  passes are systems ordered `after(render_sky)`, `before(main_transparent_pass_3d)`), and the main
  depth texture the atmosphere already makes bindable (`configure_camera_depth_usages`).
- `bevy_solari` 0.20: BLAS geometry is flagged `OPAQUE`; specular/mirror lobes are traced
  (`initial_path.wesl`). Water can't be a transparent RT surface; on the RT path it stays a raster
  layer drawn over Solari's output (below).

**Crates (crates.io and GitHub, 2026-10-13).**

| Crate | Latest | Bevy | License | State | Verdict |
|---|---|---|---|---|---|
| `bevy-aqua` | 0.1.3 (2026-08-31) | 0.19 | MIT OR Apache-2.0 | 167 commits, last 2026-09-23, 29 stars, 98 downloads, one maintainer (+1 contributor), "AI-assisted development" disclosed; no Bevy 0.20 branch (0.20 shipped 2026-10-08) | **Reference and code source, not a dependency** (below) |
| `bevy_water` | 0.18.1 (2026-02-02) | 0.18 | MIT OR Apache-2.0 | dynamic ocean material (sum-of-waves in a `StandardMaterial` extension), one level | Reference only: no FFT, no bodies, two Bevy versions behind |
| `rustfft` | 6.x | n/a | MIT OR Apache-2.0 | maintained, the Rust FFT | **Use** in `sf_water` for the CPU twin (check the version at W1) |
| `bevy_hanabi` | 0.19 line | 0.19 | MIT/Apache | not on 0.20 | Not needed (spray is ours or later) |

**`bevy-aqua` evaluation** (README, repo, crates.io, 2026-10-13). What it has: five camera-centred
displacement cascades with LOD blending; analytic (Crest-style) or Tessendorf FFT waves from sea
state, wind and fetch; shallow-water attenuation; Beer-Lambert transmission, refraction, planar mirror
or cubemap reflections, optional SSR (needs a depth prepass); persistent whitecaps and shoreline foam;
bounded ponds, lakes and river corridors with per-body optics (rivers analytic, others flat);
procedural caustics; GPU `WaveQuery` (256 probes per frame, ~1 frame readback latency); optional
`bevy_hanabi` spray; opt-in underwater (homogeneous medium, no multi-camera, waterline not closed).
Inputs: one `Ocean` resource (one sea level), one global `BedHeightMap` image (replaced by handle,
no in-place edits), bodies must be flat and should not move (moving rebuilds shore fields).

Against our constraints:

| Our constraint | bevy-aqua 0.1.3 | Cost to make it fit |
|---|---|---|
| Bevy 0.20 | 0.19 | port (render API moved to schedule-based core-3d in 0.20; WESL) |
| Floating origin (`big_space`, 256 m cells) | world XZ in `f32`, no origin phase | rewrite every world-space sample (cascades, foam, bodies, queries) |
| Streamed terrain (toroidal height clipmap) | one global `BedHeightMap` | a window copy per camera step, or sample our clipmap |
| CPU, synchronous, deterministic query (§10) | GPU readback, ~1 frame late, 256/frame | write the CPU twin ourselves |
| No depth prepass on the exterior camera | refraction/SSR expect one | restructure to a snapshot pass |
| Planar reflections | second camera re-rendering `ReflectedInWater` roots | re-renders the streamed world (draw cost ×~2); replace |
| Many worldspaces (interiors with their own water) | one `Ocean` resource | multi-worldspace bodies |
| Spray | `bevy_hanabi` (not on 0.20) | drop |

Most of its integration surface would be rewritten; what transfers unchanged is the wave model
(spectrum, FFT, cascade blending, foam accumulation, shallow attenuation) and its optics shader math.
**Decision: build `sf_water` ourselves and port bevy-aqua's wave/foam/optics code where it fits
(MIT OR Apache-2.0: allowed, keep its copyright notice in the ported files and in
`THIRD_PARTY_NOTICES`).** GodotOceanWaves (MIT, the source ARCHITECTURE §7.2 named) is the second
reference for the spectrum and cascades. Revisit trigger: if bevy-aqua ships on Bevy 0.20 with
origin-relative coordinates and a CPU query, re-evaluate it for the ocean surface only (log it in §10).

**Papers and engines** (the canonical techniques):
- Tessendorf, *Simulating Ocean Water*, SIGGRAPH course notes 2001: FFT ocean from a statistical
  spectrum, horizontal "choppy" displacement, the Jacobian of the displacement as the foam signal.
- Horvath, *Empirical Directional Wave Spectra for Computer Graphics*, DigiPro 2015: JONSWAP with
  fetch, TMA shallow-water correction, Donelan-Banner/Hasselmann directional spreading, swell. What
  GodotOceanWaves and most current game oceans implement.
- Bruneton, Neyret, Holzschuch, *Real-time Realistic Ocean Lighting using Seamless Transitions from
  Geometry to BRDF*, Eurographics 2010: far ocean shading from the wave slope variance (roughness
  grows with distance instead of aliasing sparkles).
- Dupuy and Bruneton, *Real-time Animation and Rendering of Ocean Whitecaps*, SIGGRAPH Asia 2012
  technical brief: whitecap coverage from the Jacobian, accumulated with decay.
- Losasso and Hoppe, *Geometry Clipmaps*, SIGGRAPH 2004; Strugar, *Continuous Distance-Dependent
  Level of Detail* (CDLOD), 2009: camera-centred nested grids with vertex morphing (our terrain uses
  the same family).
- Bowles, *Crest: Novel Ocean Rendering Techniques in an Open Source Framework*, SIGGRAPH 2017
  Advances, and Crest (Unity, MIT): LOD data cascades for shape, foam, dynamic waves and sea-floor
  depth; queries by GPU readback, later also baked FFT data for CPU queries.
- Unity HDRP Water System (2022+): up to three bands (swell, chop/agitation, ripples), pools use
  ripples only, rivers ripples + chop; script queries by GPU readback by default with a **CPU
  simulation** option ("full CPU simulation", "evaluate ripples" off by default) and an iterative
  search for the displaced height. Same split as ours.
- Unreal Engine Water plugin (UE 4.26+/5): water body actors (Ocean, Lake, River, Custom) on splines,
  a water zone that renders all bodies into a water info texture (height, depth, velocity), the
  Single Layer Water shading model (water drawn in its own pass after opaque, reading scene colour and
  depth: absorption, scattering, refraction), Gerstner "Water Waves" assets, and a Buoyancy component
  that approximates a hull with **pontoon spheres** (the docs' example: four pontoons on a cube).
  `GetWaterHeight` returns height, depth, normal and velocity: the same query contract as ours.
- Vlachos, *Water Flow in Portal 2*, SIGGRAPH 2010: flow maps with two-phase normal advection.
- Kerner, *Water interaction model for boats in video games* (Gamasutra 2015): per-triangle hull
  clipping with hydrostatic and drag forces; the accurate alternative to pontoons.
- Tessendorf, *Interactive Water Surfaces* (iWave), Game Programming Gems 4, 2004: a small local
  wave-equation grid for ripples and wakes (Crest's "dynamic waves" is the same idea).
- Cichocki, *Optimized pixel-projected reflections for planar reflectors*, SIGGRAPH 2017 Advances:
  screen-space planar reflections (SSPR) in one compute pass for a known plane.
- Guardado and Sánchez-Crespo, *Rendering Water Caustics*, GPU Gems 2004: caustics from refracting
  light through the wave normals onto a plane; games ship a tiling animated caustics texture.
- Barnes, Lehman and Mulla, *Priority-Flood*, Computers & Geosciences 2014 (depression filling,
  watershed labels); O'Callaghan and Mark 1984 (D8 flow accumulation); Génevaux et al., *Terrain
  Generation Using Procedural Models Based on Hydrology*, SIGGRAPH 2013 (river networks from
  drainage); Leopold and Maddock 1953 (hydraulic geometry: width grows with discharge^0.5).
- Felzenszwalb and Huttenlocher, *Distance Transforms of Sampled Functions*, 2012: exact linear-time
  Euclidean distance transform (shore distance at bake).

## As built (W1, 2026-10-13, m6-w1)

`crates/sf_water` (Bevy-free; deps `glam`, `rustfft` 6.4, `serde`, `sf_coords`, `sf_records`, `sf_world`,
`sf_terrain`). No bake steps and no buoyancy math yet (W3, W6, W7).

- `records`: `WaterBody` (`sf.water_body`), `WaterKind`, `WaterShape` (Infinite / Polygon with holes / Ribbon
  of `RiverPoint` / Bounds), `WaveTerms` (first cascade + amplitude; per-kind defaults), `WaterOptics`
  (`sf.water_optics`, physical defaults for clear water, `visibility()`), `WaterBody::from_worldspace`
  (`sea_level` → Ocean/Infinite with terrain, Pool/Bounds without).
- `spectrum`: `SpectrumParams` (wind, fetch, depth, amplitude, choppiness) with JONSWAP + TMA (Kitaigorodskii,
  Horvath's approximation) + Donelan-Banner spreading + finite-depth dispersion; `Cascades` (power-of-two
  tiles, band boundary `size(i+1)/split`, split 2); `H0::generate` (deterministic in seed, cascade, params; noise
  keyed by **wave vector**, so any two grid sizes share every coefficient), `H0::blend` (weather mix),
  `band_variance` (analytic). Swell (Horvath's second spectrum) is not in yet. `WindInput` is the named scaffold
  for `sf_wind::WindState` (WD1): swap the field type when WD1 lands.
- `twin`: `CascadeTwin` (per-step time evolution of the active bins only, 4 packed complex 2D inverse FFTs
  giving height, height rate, Dx, Dz, their rates and slopes; Catmull-Rom sampling), `WaveTwin` (1-4 cascades
  summed, displaced-height inversion with 3 fixed-point iterations), `tile_coords(WorldPos, size)` (exact
  integer cell phase: the render origin never enters), `origin_phase(origin_cell, size)` (what the GPU adds
  to camera-relative XZ), `reference_height` (direct sum, for tests and goldens).
- `query`: `WaterQuery` (`sample`, `sample_many`, `immersion`), `WaterWorld` (per worldspace, `step(t)` evaluates
  every twin), `WorldspaceWater` (bodies, 2D median-split BVH over finite bodies + unbounded bodies by priority,
  optional `BedHeights` (impl for `TerrainHeights`), optional `WaveTwin`). The ocean is dry where the bed is above
  the displaced surface; depth is `f32::INFINITY` where no bed is known (interiors, unstreamed terrain).
- Hooks: `WaterSource` (bodies, `river_policy`, `ocean_spectrum`), `DefaultWaterSource`, `WaterOpticsSource`,
  `RiverPolicy`.

**Differences from the text above** (all logged in ARCHITECTURE §10): the query has no `t` argument (it reads the
last `step`); bands split at `size/2`; cubic twin sampling; physical amplitudes (bevy-aqua normalises every sea
state to a fixed RMS height; we don't).

**Tests** (`cargo test -p sf_water`, 11): spectrum energy vs the analytic JONSWAP integral within 3 % (realised
variance within 15 %, Donelan-Banner integrates to 1 ± 1e-3; Hs vs the JONSWAP fetch law within 40 %: the peak
and energy fits are separate regressions, ~30 % apart); cascades power-of-two and gap-free; determinism (same
seed → same `h0` bits; twin state a function of `t` only); 64² and 256² grids share every coefficient and the
whole band; twin vs direct DFT: 2.4e-7 m on nodes, **2.4 mm** between nodes and vs the 256² grid (Hs 1.9 m; was
7.7 cm bilinear with `/4` bands); inversion: 3 iterations within 2.4 mm of 40 (choppiness 1, 18 m/s, two
cascades), converged fit 2.6e-6 m; origin shifts and far cells (cell 100 002) give bit-identical samples, the GPU
formula lands within 1e-3 m of the CPU's tile coordinates for any origin; weather blend continuous; BVH priority
(ocean, lake above the sea, pool inside the lake, 60 ponds, a river with height and flow by segment), dry land
cuts the ocean, immersion; record JSON round trip.

**Cost** (`cargo run --release -p sf_water --example water_bench`, 4060 Laptop machine, 201 bodies): twin step
**0.027 ms** for cascade 0 (192 active bins; budget 0.05), 0.055 ms with cascade 1 (376 bins: High's "rough sea"
option is 10 % over, so evaluate cascade 1 every other step or accept it); sample **0.25 µs** (budget 2), 0.43
µs with two cascades.

**W2 handoff (ocean surface + water pass)**:
1. GPU spectrum: upload `H0::generate(params, cascades, i, 256 or 128, seed)` per cascade (RG32F per texel, layout
   `iz * n + ix`, signed frequency mod n). Use the **same seed and `Cascades`** as the twin: the twin is then the
   GPU's cascade 0 by construction. Time evolution on the GPU must match `CascadeTwin::evaluate`: `ω` from
   `SpectrumParams::omega` (finite depth if set), `h̃ = h0·e^{iωt} + conj(h0(−k))·e^{−iωt}`, `D̃ = λ·i·k̂·h̃` (sign:
   crests sharpen). The CPU reduces `ω·t` mod 2π in f64; the GPU has no f64, so W2 adds Tessendorf's repeat
   period `T` (quantise `ω` to multiples of 2π/T in `CascadeTwin::new` too, upload `t mod T`) and keeps the
   twin test green.
2. Sampling position: `fract((origin_phase(origin_cell, size) + rel_xz) / size)`, origin cell from the same
   place `sync_origin` reads (water-adapter.md "What replaces `SeaPlaneScaffold`").
3. Golden: capture GPU displacement at a few points from `water_ocean` and compare with
   `WaveTwin::sample` (target 1 cm; the twin is 2.4 mm from the exact field).
4. Weather: blend with `H0::blend` (CPU) and re-upload when `w` moves 2 %; rebuild the twin from the same blended
   `h0` (`WaveTwin::from_h0`).
5. App wiring (owner of `exterior.rs`): `WaterWorld` as a resource, `step(t)` in `FixedUpdate` before physics,
   one `WorldspaceWater` per active worldspace from `mw_water::MwWaterSource`, bed = the resident
   `TerrainHeights`.

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Ocean waves | Tessendorf FFT cascades (Crest, HDRP 3 bands, Sea of Thieves, Atlas, bevy-aqua); Gerstner sums (UE Water Waves, bevy_water) | FFT cascades, JONSWAP/TMA (Horvath 2015) | **match** (FFT branch; Gerstner kept only as Low fallback if FFT is over budget) |
| Wave LOD | camera-centred LOD cascades / clipmaps (Crest, aqua, UE water quadtree) | geometry clipmap + CDLOD morph, horizon skirt | **match** |
| Far ocean shading | slope-variance BRDF transition (Bruneton 2010) | same | **match** |
| Whitecaps / foam | Jacobian with persistent decay (Tessendorf, Dupuy-Bruneton, Crest, HDRP) | same | **match** |
| Water bodies at any height | UE water bodies (ocean/lake/river/custom) + water zone info texture; HDRP surfaces (ocean/river/pool); Crest water bodies | records + water-level clipmap | **match** |
| Water shading pass | UE Single Layer Water (after opaque, scene colour + depth); HDRP water pass | snapshot + water draw | **match** |
| Reflections | SSR + probes (UE, HDRP), planar (older; aqua), RT (UE Lumen, HDRP RTR) | SSPR + camera-centred dynamic probe + env map; RT later | **match** (SSPR instead of SSR because our exterior has no deferred prepass) |
| Refraction, absorption | screen-space refraction + Beer-Lambert + scattering (all) | same | **match** |
| Caustics | projected animated texture / light function (UE, Crest), RT caustics (HDRP option) | baked atlas in the surface hook | **match** (no RT caustics) |
| Underwater | volumetric underwater fog + waterline meniscus (Crest, HDRP underwater volume, UE post) | froxel water medium (High) / analytic, meniscus | **match** |
| Shoreline | sea-floor depth (Crest), distance to shore (UE water info), shore foam | baked shore EDT + live depth, openness | **match** |
| Rivers | authored splines with flow (UE, HDRP rivers, Portal 2 flow maps) | authored or baked from hydrology | **differ** (games author rivers; we derive flow because the first content pack has none: a bake step, not runtime simulation) |
| Gameplay queries | GPU readback (Crest, HDRP default, aqua) or CPU simulation (HDRP CPU option, Crest baked FFT, UE CPU Gerstner) | CPU twin of the long cascades | **match** (CPU path, deterministic) |
| Buoyancy | pontoons (UE Buoyancy component, Crest), hull triangle clipping (Kerner 2015, many boat games) | pontoons default, hull clipping per body | **match** |
| Interaction ripples | local wave-equation sims (iWave, Crest dynamic waves, HDRP deformers) | iWave grid around the camera | **match** |
| Temporal AA on water | motion vectors from displacement, reactive masks (UE translucency velocity, FSR reactive) | velocity redraw + reactive mask (weather.md question 9) | **gap until the DLSS bias patch** (P1 with precipitation) |
| Fluid simulation (flooding, flowing over terrain) | rare in open worlds (shallow-water sims in UE Fluid Flow, Cities: Skylines) | none | **differ** (non-goal) |

**Sources** (checked 2026-10-13): bevy-aqua README and repo (https://github.com/sayhisam1/bevy-aqua, commits
API, branches), crates.io `bevy-aqua`, `bevy_water`; UE Buoyancy component docs
(https://dev.epicgames.com/documentation/en-us/unreal-engine/water-buoyancy-component-in-unreal-engine)
and Water System overview; Unity forum thread on HDRP water CPU simulation
(https://discussions.unity.com/t/water-level-in-water-system/1688088) and the HDRP water blog
(https://unity.com/blog/engine-platform/new-hdrp-water-system-in-2022-lts-and-2023-1); Bevy 0.20 and
bevy_solari 0.20 sources in the local registry. Papers as cited above (from the literature, not
re-read this session); UE water info texture and Single Layer Water details from the UE docs as
remembered: mark unverified on detail.

## As built (W2, 2026-10-13, m6-w2)

`crates/sf_water_bevy` (thin Rust, WESL; minimal bevy_* deps + `sf_water`). `WaterPlugin`; host inputs `OceanSpectrum`
(params, cascades, seed: the twin's), `WaterFrame` (time, origin cell, render-space level), `WaterLook`
(`from_optics`), `WaterFogInput` (froxel volume), `ClipmapSettings`; markers `WaterCamera`, `OceanSurface`;
`WaterSystems::Drive` (PostUpdate). `sf_water`: `REPEAT_PERIOD` 256 s, `omega_repeat`/`repeat_cycles` (the twin now
evolves with them), `H0::gpu_texels`; test `waves_repeat_after_the_repeat_period`.
- **FFT** (`water_fft.wesl`): 4 × 256² every frame: spectrum (4 packed complex signals: h, Dx, Dz, slopes, Jxx, Jzz,
  Jxz), radix-2 inverse FFT rows then columns in workgroup memory (one line per workgroup), assemble into two RGBA16F
  arrays (Dx,h,Dz,foam / sx,sz,Jxx,Jzz) + persistent Jacobian foam (storage buffer with decay). Phase
  `2π·fract(cycles·(t mod T)/T)`.
- **Golden** (`cargo run --release -p sf_water_bevy --example water_ocean -- --golden`, 14 m/s, t 37.5 s): GPU cascade 0
  vs `CascadeTwin` on the 64² nodes **max 1.9 mm, rms 0.2 mm** (Hs ≈ 2 m; includes f16 storage). Exits 0/1.
- **Clipmap**: one static mesh (7 rings × 128², 0.25 m, hole 30 cells, skirt to 32 km); the vertex shader snaps each
  ring to 2× its spacing around the camera and CDLOD-morphs its outer 12 cells; a ring discards where the finer one
  covers it, one fine cell short of the edge (there both are the same surface: no seams). Displacement fades per
  cascade with distance (10-16 × its shortest wavelength) and to 0 at 900 m; normals fade per cascade by pixel
  footprint, the lost variance goes into roughness (Bruneton-lite; no mips yet).
- **Water pass**: snapshot (colour copy + depth resolve to R32F, fullscreen, ordered after everything that runs before
  `main_transparent_pass_3d`, i.e. `render_sky` and the `sf_atmos` composite) + a compute copy of Bevy's aerial-view
  LUT; water draw = `ExtendedMaterial<StandardMaterial, WaterExt>` in the transparent phase, depth-writing, cull none
  (open question 1 decided: keeps Bevy's lights, shadows and environment map with the least code); refraction with
  in-front rejection, Beer-Lambert + single-scattering body colour, Bevy PBR with F0 0.02, whitecaps (cascades 0-1),
  contact foam (vertical depth below 0.35 m × contact), then aerial perspective (LUT in-scatter, analytic air
  extinction) and froxel fog. **Velocity**: after the transparent pass the clipmap is redrawn procedurally into the
  prepass motion-vector texture where the water is nearest (current vs previous displacement texture and view); only
  when the camera has `MotionVectorPrepass` (DLSS, `--prepass`).
- **GPU cost** (example, 1600×900, 4060 Laptop, **shared machine**: other agents' builds were running): `water_fft`
  0.22 ms (High budget 0.15: over; amortise the 256 m cascade every 2nd frame, or 3 cascades on Medium),
  `water_snapshot` 0.04 ms (budget 0.1), transparent pass holding the water draw 0.53 ms (draw budget 0.5). Velocity not
  timed (the example has no prepass). Not yet re-measured in the app at 1440p.
- **Handoff (W3/W4 and W2 leftovers)**: shore attenuation + `water.shore` (W3; waves run up beaches unattenuated now),
  optics tuning (W1 placeholders read milky turquoise), SSPR + dynamic probe (W4: environment map only now; the
  material's PBR call is where SSPR plugs in), underwater (W5; back faces flip the normal only), FFT amortisation and
  the Low 128² path, normal mips, F2 analytic height fog on water when froxels are off (water gets aerial + froxels
  only), weather `H0::blend` re-upload (WE3; `upload_spectrum` regenerates on any `OceanSpectrum` change, on the main
  thread, tens of ms), pipeline warm-up (the FFT pipelines take ~1-2 s to compile on DX12; the water is flat until
  then), in-app DLSS smear check and fast flight across origin shifts (WA-8).

## As built (W3, 2026-10-13 / 2026-10-18, m6-w3 + w3fin)

- **Shore** (`sf_water::shore`): `WetMask` → `ShoreField::bake`: exact EDT (Felzenszwalb-Huttenlocher) to a signed
  shore distance (± `range` 256 m), 4-connected components, openness = exp(−geodesic / `open_falloff`) through water
  from the open region (the edge-touching component, ≥ `open_core` from shore; Dial 5-7 chamfer), enclosed water
  `enclosed_max` × inscribed diameter / `enclosed_fetch`; land texels take the wettest neighbour. `sample`,
  `swell_scale` / `chop_scale` (GPU twins in `water_common.wesl` `shore_scales`), `encode_rg8` (sqrt-encoded distance).
  The shore distance is the GPU's depth proxy (the water material has no bed heights). `WorldspaceWater::set_shore`:
  the CPU query attenuates the twin's swell the same way. Host input `WaterShoreInput`.
- **Shader**: shore attenuation inside `displaced(.., scales)` (vertex and velocity share it), fragments of sea under
  land discarded (shore distance < −2 m), shore foam band 0-6 m travelling shoreward, bounds discard for bounded
  bodies, region optics blended from an R8Uint cell map + an 8-entry look table (`WaterRegionsInput`, `MAX_LOOKS`),
  analytic height fog when the froxels are off (`WaterFogInput.height` / `height_color`, the `FogLayer` closed form).
- **FFT**: two fused dispatches (`fft_rows` = spectrum + row FFT, `fft_cols` = column FFT + assemble/foam), runtime
  n / log n, `skip` cascade, per-cascade foam dt. `WaterQuality`: Low 2 × 128², Medium 3 × 128², High 4 × 256² with
  the two swell cascades alternating (3 per frame), Ultra 4 × 256²; `size_waves` recreates the images on a tier change.
  Slope mips (6 levels, `water_mips.wesl`), sampled at the pixel footprint's lod, lost variance → roughness;
  displacement foam fades at lod 2-5. Pipelines warm up at startup (FFT, mips, aerial, snapshot, velocity).
- **Bodies**: `WaterSurface { level, bounds, swell, chop, look }` (+ `WaterSurface::pool`) draws with the ocean's
  material and cascades in its own entity's render layers; the FFT and snapshot run while any surface is shown.
- **GPU** (example `water_ocean`, 1600 × 900, 4060 Laptop, **shared machine**): `water_fft` 0.139 ms (was 0.22; High
  budget 0.15 met) + `water_mips` 0.023 ms; snapshot 0.043 ms; transparent pass 0.54 ms. Golden GPU vs twin: max 1.0 mm,
  rms 0.1 mm. In-app 1440p counters not yet measured (no quiet machine on 2026-10-18).
- Interior pools drew opaque black (WA-6; w3pool 2026-10-18): not the snapshot (`--off froxels` drew them right) but
  the froxel fog. Interiors drop `CloudCamera`, so nothing writes `FroxelVolume`, yet its `params.w` stayed 1 and the
  water (like any `FroxelFogMaterial`) applied the unwritten volume (transmittance 0). Fixed in `sf_atmos_bevy`:
  `params.w` is 1 only while an active `CloudCamera` writes the volume (the industry rule: a translucent reads the
  froxel volume only for views that integrated it).
- Open (handoff): DLSS velocity check and WA-8 flight; a GPU counter for discarded sea-under-land fragments; the `water_bodies` example (lake + shore
  on the island). W4 (SSPR) plugs in where `apply_pbr_lighting` supplies the environment reflection; the dominant plane
  is `WaterFrame.level` or a `WaterSurface.level`.
