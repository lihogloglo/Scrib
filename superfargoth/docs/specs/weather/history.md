# Weather: history and superseded material

Moved verbatim from [weather.md](../weather.md) (doc cleanup S5a, 2026-10-25): research summary, reuse decision, Megalophobia scavenge, proposed changes to other specs, and the WE4 as-built logs.

## Research summary (2026-10-11)

Reuse order (CLAUDE.md rule 8): Bevy, then crates, then our own. The sources come first, then what
each one means for us.

**Clouds.**
- Schneider and Vos, *The Real-time Volumetric Cloudscapes of Horizon Zero Dawn*, SIGGRAPH 2015
  Advances ([pdf](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf)).
  This is the canonical density model: a Perlin-Worley base shape, Worley detail erosion, a height
  gradient per cloud type, and a weather map (coverage, type, precipitation). Clouds are raymarched in
  a spherical shell over a flat world. They are drawn at quarter resolution, and 1/16 of the pixels
  update each frame with temporal reprojection.
- Schneider, *Nubis: Authoring Real-Time Volumetric Cloudscapes with the Decima Engine*, SIGGRAPH 2017
  Advances ([pdf](https://advances.realtimerendering.com/s2017/Nubis%20-%20Authoring%20Realtime%20Volumetric%20Cloudscapes%20with%20the%20Decima%20Engine%20-%20Final.pdf)).
  Authoring comes from the weather map and type profiles, plus the cloud shadow and lighting
  approximations (Beer-powder, multiple-scattering octaves).
- Schneider, *Nubis, Evolved*, SIGGRAPH 2022
  ([pdf](https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf)),
  and *Nubis³*, SIGGRAPH 2023
  ([pdf](https://advances.realtimerendering.com/s2023/Nubis%20Cubed%20(Advances%202023).pdf)).
  These cover clouds you can fly through, compressed voxel clouds with SDF-accelerated marching, and
  0.2 to 5 ms on PS5 depending on view. We take the 2015/2017 model for the global deck. Voxel clouds
  are an option for one hero volume (the Red Mountain plume), not the whole sky.
- Hillaire, *Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite*, SIGGRAPH 2016 PBS
  course ([page](https://www.ea.com/frostbite/news/physically-based-sky-atmosphere-and-cloud-rendering)).
  It gives the energy-conserving analytic step integral (no banding at low step counts), the
  multiple-scattering octave approximation, and how clouds integrate with the atmosphere LUTs.
- Bauer, *Creating the Atmospheric World of Red Dead Redemption 2: A Complete and Integrated Solution*,
  SIGGRAPH 2019 Advances course (ACM SIGGRAPH 2019 Courses). Bauer presents sky, clouds, fog and
  volumetrics as **one integrated system** that a weather state drives, rather than separate effects.
  This is the "RDR2 look" the owner wants. We could only confirm the talk's outline, not its slides.
  The parts we rely on are also in the Nubis and Frostbite sources: the multiple-scattering octaves,
  low-resolution temporal clouds, and froxel fog with a separate far raymarch. Our structure follows
  the talk's thesis, not its implementation.
- UE5 *Volumetric Clouds*, *Sky Atmosphere*, *Exponential Height Fog* and *Volumetric Fog*
  (dev.epicgames.com docs). This is the same split we use: the sky LUTs (Hillaire 2020), a cloud
  component raymarched at reduced resolution with temporal reprojection, cloud shadows on the
  directional light and in the fog, analytic height fog for the far range, and froxel fog for the near
  range.

**Fog and god rays.**
- Wronski, *Volumetric Fog*, SIGGRAPH 2014 Advances (Assassin's Creed IV). Hillaire, *Towards Unified
  and Physically-Based Volumetric Lighting in Frostbite*, SIGGRAPH 2015 Advances
  ([page](https://www.ea.com/frostbite/news/physically-based-unified-volumetric-rendering-in-frostbite)).
  Both describe a camera-aligned froxel grid: density and lighting per froxel (sun with shadow maps,
  local lights, ambient), then front-to-back integration and temporal reprojection. Opaque and
  transparent surfaces then sample the result. UE's Volumetric Fog uses the same technique. God rays
  are the shadowed part of the in-scattering, not a screen-space hack.
- Hillaire, *A Scalable and Production Ready Sky and Atmosphere Rendering Technique*, EGSR 2020
  ([pdf](https://sebh.github.io/publications/egsr2020.pdf)). Bevy 0.20's `Atmosphere` implements this,
  including the aerial-view LUT.

**Wind.**
- Rockenbeck, *Blowing from the West: Simulating Wind in Ghost of Tsushima*, GDC 2021
  ([vault](https://gdcvault.com/play/1027124/Blowing-from-the-West-Simulating)). One wind system drives
  thousands of cloth pieces, hundreds of thousands of particles and millions of grass blades, and the
  wind is a gameplay signal. We have the abstract only. The design point we take: one field, many
  consumers. Wohllaib, *Procedural Grass in Ghost of Tsushima*, GDC 2021
  ([vault](https://gdcvault.com/play/1027033/Advanced-Graphics-Summit-Procedural-Grass)): per-blade
  Bézier bend from the wind, already ARCHITECTURE §7.1.
- SpeedTree wind (Global, Branch, Leaf levels; SpeedTree Modeler docs) and UE *Pivot Painter Tool 2.0*
  (docs). Tree motion is hierarchical: the trunk bends, branches sway around their pivots, and leaves
  flutter. Pivots, axes and hierarchy levels are baked into vertex data or textures offline, and a
  vertex shader evaluates them. Morrowind trees carry no such data, so a bake step derives it.

**Rain, wetness, puddles.**
- Tatarchuk, *Artist-Directable Real-Time Rain Rendering in City Environments*, SIGGRAPH 2006 course
  (ATI ToyShop). Rain is streak particles in camera-relative layers, with occlusion from a top-down
  depth map, splashes and ripples. The industry still does it this way (RDR2, Watch Dogs, UE Niagara
  rain).
- Lagarde, the "Water drop" series, 2012-2013 (seblagarde.wordpress.com): 1 *Observe rainy world*,
  2a/2b *Dynamic rain and its effects*, 3a *Physically based wet surfaces*, 3b *Other wet surfaces*.
  Porous materials darken (albedo²-like) and get smoother. Puddles are a water layer
  (F0 ≈ 0.02, roughness ≈ 0, flat normal, ripple normals). The puddle level follows the height map,
  and accumulation and drying are separate rates. Remember Me used a rain occlusion map.

**Distant shadows.**
- UE: cascaded shadow maps out to *Dynamic Shadow Distance*, *Far Shadow Cascades* for selected
  casters, then *Distance Field Shadows* out to *Distance Field Shadow Distance*
  ([docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-field-soft-shadows-in-unreal-engine)).
  UE5 adds Virtual Shadow Maps. The point is that shadows beyond the cascades come from a cheaper,
  coarser representation, never from more cascades.
- Horizon mapping: Max, *Horizon Mapping: Shadows for Bump-Mapped Surfaces* (1988). Sloan and Cohen,
  *Interactive Horizon Mapping* (EGWR 2000). For a heightfield and a single sun direction, the
  horizon reduces to one "shadow height" per texel, which any receiver can compare against. Terrain
  engines and flight simulators use this for far terrain shadows.
- Cloud shadows: Nubis 2017 (a 2D shadow map from the weather map), UE Volumetric Clouds (cloud
  shadows on the directional light and in the fog).

**Bevy 0.20 (read in `~/.cargo/registry/src/*/bevy_*-0.20.0`).**
- `Atmosphere` + `AtmosphereSettings`: transmittance, multiscattering, sky-view and **aerial-view**
  LUTs. The `render_sky` pass runs after opaque and reads the main depth texture. For geometry pixels it
  composites aerial-view in-scattering and segment transmittance (`aerial_view_lut_max_distance`
  32 km, 32³ by default). **Aerial perspective already applies to everything**, far field included,
  with no depth prepass needed. Its slices are ~1 km deep, so it is haze, not near fog.
- `DistanceFog` (`Linear`, `Exponential`, `ExponentialSquared`, `Atmospheric`): applied per material,
  with **no height falloff**.
- `VolumetricFog` (camera) + `VolumetricLight` (directional/point/spot) + `FogVolume` (box, optional
  3D density texture with scroll offset). It raymarches per pixel per volume, with single scattering
  and 64 steps by default, and reads the main depth texture. Megalophobia measured its limits on large
  boxes (below): the sun term dims with box size, edges are hard, there is a box-intersection bug
  (#22574), and each volume costs one full-screen pass. It is not a froxel volume, has no temporal
  integration, and transparent surfaces can't sample it.
- `DirectionalLightTexture` (feature `pbr_light_textures`, through clustered decals): it multiplies a
  directional light by an R-channel texture projected in light space, tiled or single, for every PBR
  material. This is the cheapest correct hook for **cloud shadows** on every material, ours and
  `StandardMaterial` alike.
- Shadows: at most 4 cascades per light (`MAX_CASCADES_PER_LIGHT`). Past the last cascade's far bound
  `fetch_directional_shadow` returns 1.0, which means **lit, with a hard cut and no fade**
  (`shadows.wesl`). Overlap blending happens only between cascades.
- `ScreenSpaceReflections` requires `DepthPrepass` + `DeferredPrepass`. No height fog, clouds,
  precipitation, wind or GPU particles in Bevy.
- `pbr_lighting::directional_light` and `shadows::fetch_directional_shadow` are public WESL functions.
  We use them for the far-shadow correction (below).

**Crates (crates.io, 2026-10-11).**

| Crate | Latest | Bevy | License | Verdict |
|---|---|---|---|---|
| `bevy_weather` | 0.2.0 | 0.19.1 | MIT/Apache | **Reference.** Full weather plugin: curved cloud shell, CPU cloud shadows as `DirectionalLightTexture` (1.5-20 ms CPU, amortized), `DistanceFog` + fog volumes, GPU rain and snow, a pure-function weather driver (clock + seed). One maintainer, 10 commits, 1 star. Not on 0.20. We'd replace its fog, shadows and coordinates anyway. |
| `bevy_fast_clouds` | 0.19.0 | 0.19 | MIT/Apache | Wavelet cloud volumes, 18 recent downloads. Not Nubis, not on 0.20. Reference only. |
| `bevy-volumetric-clouds` | 0.2.0 | 0.18 | MIT | HZD-style skybox clouds at a fixed 1080p target, **ignores the depth buffer**, no atmosphere integration. Rejected. |
| `bevy_hanabi` | 0.19.0 (main: 0.20.0-dev, still Bevy 0.19) | 0.19 | MIT/Apache | GPU particles. Not on 0.20 yet. Precipitation doesn't need a particle simulator (below). Revisit for splashes and spell VFX when it ports. |
| `bevy_enoki` | 0.8.0 | **0.20** | MIT | 2D sprite particles only. Rejected for 3D. |
| `bevy_firework` | 0.10.0 | 0.19 | MIT/Apache | CPU particles. Wrong scale for rain. |
| `bevy_feronia` | 0.8.4 | 0.18 | MIT/Apache | Foliage scattering + wind shaders. Reference for grass wind, stale Bevy. |

## Reuse decision

| Need | Bevy | Crate | Ours, and why |
|---|---|---|---|
| Sky, sun, aerial perspective | **`Atmosphere` (used)** | n/a | Weather drives its `haze` and medium terms (sky-lighting.md). |
| Clouds | none | `bevy_weather` (0.19, bus factor 1, CPU shadows, no floating origin) | **Ours** in `sf_atmos_bevy`. We port the owner's Megalophobia deck math (below), on a shell with temporal reprojection and the atmosphere LUTs. |
| Cloud shadows on surfaces | **`DirectionalLightTexture` (used)** | n/a | We generate the texture on the GPU (a few µs, vs bevy_weather's ms on the CPU). |
| Near fog + god rays | `VolumetricFog` (prototype only, near-term step N3) | none | **Ours: froxel fog.** Bevy's raymarch is per pixel per box, has no temporal reprojection, transparent surfaces can't sample it, and it has known large-volume artifacts. |
| Height fog, far fog | `DistanceFog` (no height) | none | **Ours**: analytic exponential height fog (UE formula), plus a low-resolution far raymarch shared with the cloud pass. |
| Wind | none | `bevy_feronia` (grass only, 0.18) | **Ours** (`sf_wind`): CPU and GPU twins of one deterministic field. |
| Precipitation | none | `bevy_hanabi` (not on 0.20) | **Ours**: closed-form instanced streaks in camera-wrapped volumes. No simulation, so no particle crate is needed. |
| Wetness, puddles, snow, ash | none | none | **Ours** (`sf_surface`): surface-state clipmap + the shader hook terrain already calls. |
| Distant shadows | CSM (4 cascades max, hard end) | none | CSM retuned + **ours**: a terrain shadow-height map and a far-shadow correction in our materials. |

## Megalophobia (the owner's Bevy project): what to scavenge

`D:\Gamedev\Megalophobia`, Bevy **0.19**, author: the owner (all 10 commits by `lihogloglo`). No
LICENSE file. `THIRD_PARTY_NOTICES.md` lists only `src/gen/erosion.rs` (MPL-2.0, Rune Skovbo
Johansen / Luke Mitchell), CC BY greeble meshes and NASA imagery. None of these are in the files below,
and none of them may move. The files below carry no third-party markers. They cite papers (Schneider,
Hillaire, Wrenninge), not code.

| File | What it is | Take | Change for us |
|---|---|---|---|
| `src/clouds.rs`, `assets/shaders/cloud_deck.wgsl`, `cloud_density.wgsl` | Nubis density (Perlin-Worley + weather map + Worley erosion, height-fraction sampling), Hillaire analytic step integral, multiple-scattering octaves, Beer-powder, two-lobe HG phase, CPU tiling noise generators (Perlin lattice, tiling Worley), per-texture scroll wrapping | **Density model, lighting math, noise generators, scroll rule** ("each texture wraps in its own tile": the bug that taught it is documented) | Slab → spherical shell. Full-res transparent mesh material → quarter-res render-graph pass with temporal reprojection. Fixed sky/ground tints → atmosphere LUTs (sun transmittance, sky ambient, aerial perspective). `#ifdef DEPTH_PREPASS` depth clamp → main depth texture (exteriors have no prepass, far-field.md). Static dither → per-frame blue noise (TAA/DLSS). Startup noise bake → `atmos.noise` bake step. Coordinates → origin-relative + `WorldOrigin` phase. |
| `src/cloud_shadow.rs`, `cloud_shadow.wgsl` | A 60 km alpha-masked quad drawn into the sun's cascades so fog shafts see clouds | **The out-of-range-is-lit rule** and the dithered-threshold penumbra idea | Replaced by a 2D cloud shadow map (`DirectionalLightTexture` + fog/cloud sampling). Our cascades end at ~500 m, and a quad in every cascade costs a shadow pass per cascade. |
| `src/ground_fog.rs`, `ground_fog.wgsl` | Raymarched slab: warped, advected bank noise over an exponential profile + a haze column shadow-sampled for shafts | **The bank density function** | Becomes a density term of the froxel volume and of the far raymarch, not its own pass. |
| `assets/shaders/grass_wind.wgsl` (+ `grass.rs` notes) | Two-octave curl-noise gust field advected downwind, gust envelope with calm bias, swirl capped so blades never lean upwind, `time_prev` for motion vectors, one shared function for forward/prepass/shadow | **The gust field, nearly as is** | WGSL + Rust twin in `sf_wind`. Origin phase instead of a camera focus point. Uniform from `sf_wind_bevy`. |
| `src/volumetrics.rs` | `FogVolume` setup + tiling 3D noise | Lessons only (why `FogVolume` fails at scale) | n/a |

**Measured cost there** (RTX 4060 Laptop, dev build, window resolution not recorded, TRACKER.md
session 11): cloud deck **5.8 ms** (full resolution, 64 + 6 steps, 3 octaves), ground fog slab
**2.5 ms**. That is the reason for our quarter-resolution, temporally amortized design. With the same
math the expected cost is 0.5 to 1 ms at 1440p (budgets below).

**Provenance gate:** before any of this code enters superfargoth, the owner states in writing (a
§10 entry) that they license these Megalophobia files as MIT OR Apache-2.0. Ports are rewritten for
0.20/WESL anyway, but they are derived works of those files.

## Proposed changes to sky-lighting.md (owned by the dlss session; apply after it finishes)

1. **Shadows** paragraph: "4 cascades to `s_max` = far-field `d1·1.1` + margin (default 500 m), first
   bound 10 m; shadow map 4096 high / 2048 low; past the CSM, far shadows come from weather.md's
   shadow-height map". `SkyRenderOptions::cascades` default changes accordingly, or the app derives it
   from `FarFieldSettings`.
2. **Data model**: `Wind` moves to `sf_wind` (re-exported from `sf_sky`, so callers don't change).
   "M6 swaps its analytic gusts for a sampled field" now points to weather.md.
3. **Weather hook** (applied in code by F1, 2026-10-13): `Weather` gains `dust: f32` (absorbing aerosol term scale, N2). `haze` is driven by
   `sf_weather`.
4. **Interfaces**: the sun/moon entities get `DirectionalLightTexture` (cloud shadows) and, optionally,
   `VolumetricLight` (prototype N3). `SkyPlugin` must keep them when re-spawning lights. The light's
   `Transform` is set by `sf_atmos_bevy` (camera-centered).
5. **Non-goals**: "clouds, weather presets and blending, fog, precipitation, surface wetness → weather.md".
   Stars and moon phases (open question 4) stay with sky-lighting.
6. **Raster path**: note that the exterior camera has no depth prepass (far-field.md pitfall 2), which
   matters for SSR on wet surfaces (weather.md open question 4).

## Proposed changes to other specs

- **far-field.md**: `lod.policy`/`lod.l1` fall back to `mesh.rest` (B2). Proxy and impostor materials
  call the far-shadow correction and the surface hook (wetness only). Impostor atlas header gains wind
  stiffness. "Shadows: L1 casts (far cascades)" → "L1 casts in the CSM's last band; past the CSM,
  terrain shadows come from the shadow-height map".
- **streaming.md**: `PlacementSpawner::prepare` (B3). `MaterialMapper` maps to `WorldMaterial` (surface
  hook) at M6.
- **terrain.md**: `surface_state.wesl` moves to `sf_surface_bevy` (the hook signature stays). The
  terrain material adds the far-shadow correction. The new bake step `terrain.skyvis` (tier 4,
  optional). The clipmap is bound for the shadow-height compute.
- **bake.md / bake-adapter.md**: products `mesh.rest` and `wind.paint` (a per-LOD vertex stream appended
  at load).

### WE4a as built (we4a, 2026-10-19): `sf_weather::WeatherMap` (CPU side)

**Reuse check (rule 8).** Bevy has no spatial weather; the map is ~400 lines over `sf_weather`'s own `evaluate` and
`border_blend`, so it lives in that crate (module `map`, no new deps: `glam`, `smallvec`, `sf_atmos::fog`).

- **`ClimateRaster`** (world X, Z origin, square size, `width × height` of `Option<ClimateIx>`, band): the adapter's
  region raster. Unowned squares (sea) take the **nearest** owned square's climate (owner decision 1): Euclidean
  dilation by vector propagation (Danielsson 1980: a BFS that carries each square's source square and relaxes on a
  closer one; ties to the lower climate index, then the lower source), so two regions meet at sea on their Voronoi
  border and blend there like on land. `site_at(p)` = own square's climate + the nearest other climate among the squares
  within half the band (point-to-square distance), through `border_blend`: the same rule as the app's `ClimateMap::border_at`
  (one ring when the band ≤ 2 squares), generalized to any band. Past the raster: clamped to the edge squares.
  Deviation from Design §2: no stored per-square `(neighbor, distance)`; the distance is exact per query from the
  neighborhood (WE4b's resolve does the same 3 × 3 search per texel, trivial at 65 k threads).
- **`ClimateRow`** (the table): each raster climate evaluated **once at weight 1** (`update_table`; lightning window
  empty: strikes stay the camera frame's) into 31 continuous channels (`row::*`: cloud genus terms + cloud speed, dimming,
  haze, dust, ground wind, gustiness, height fog + bank terms, total precipitation, precipitation extinction, lightning
  rate, surface rates) and the heaviest precipitation's `look`. `ClimateRow::visibility_m(alt)` recomputes the frame's
  Koschmieder visibility from the blended terms (nonlinear, so not blended itself).
- **`WeatherMap`** (`new(raster, size, texel_m)`, default `MAP_SIZE` 256): `sample(p)` / `sample_site(site)` = the
  weight-normalized sum of rows (the CPU twin of the resolve); `update(book, state, hours, hour_of_day, camera)` refreshes
  the table, then the **world-anchored toroidal window**: world texel `t = floor(p / texel_m)` (absolute f64 position,
  so floating-origin shifts move nothing) stored at `t mod size`, window = camera texel snapped to 1/8-map steps − size/2.
  A full resolve when the table moved past `MAP_EPSILON` (1e-3, relative over 1) since the last resolve, else only the
  strips entering the window on a recenter. `MapTexel` = the spec's A (coverage, genus, wetness, storm), B (base, thickness,
  density, erosion), C (cirrus, precipitation, **height-fog density**, **overcast dimming = darkness**) + `look`.
  Deviation: C carries absolute fog density and dimming instead of "fog scale" and the look byte (the look is a separate
  field); WE4b decides the packing. **Uniform skip**: when every row's map channels agree (all regions Clear: their
  winds differ, the map channels don't), `update` writes nothing, `texel_at` is `None` and `uniform_row()` stands for the map.
- **Near = far:** the camera's site is meant to come from `raster.site_at(camera)` (WE4c replaces the app's
  `ClimateMap::border_at` with it); `CurrentWeather.frame` = `evaluate(site)` and `map.sample(camera)` agree by construction.
- **Verified** (`cargo test -p sf_weather`, 5 new, 17 total): `raster_fills_sea_with_the_nearest_region` (islands + sea:
  sea columns split at the midpoint, ½/½ at the sea border, edge clamp, empty raster → `None`);
  `map_at_the_camera_is_the_current_weather` (2 m walk across a 200 m band, two climates mid-transition: all 31 channels
  of `sample` = `ClimateRow::from_frame(evaluate(site_at))` within 1e-5 relative, same look, visibility within 1e-4, no
  coverage/precipitation step ≥ 0.02, resolved texel = sample at its center); `map_is_a_pure_function_of_seed_overrides_clock_raster`;
  `uniform_day_skips_and_recentering_writes_only_new_strips` (clear + clear: 0 texels; storm: 32² once; same inputs: nothing;
  one step east: 4 × 32 texels, every texel = its world sample); `map_cost`.
- **Cost** (release, 16 climates, 64 × 64 raster): **table 0.0034 ms** (estimate ≤ 0.02 incl. upload; dev opt-level 1:
  0.0053 ms). CPU resolve of the full 256² window 3.5 ms release (5.4 ms dev): fine for tests and a one-off, not per frame;
  the per-frame resolve is WE4b's GPU pass (≤ 0.01 ms estimate). Recentering strips (256 × 32) would be ~0.4 ms on the CPU.

**Next (WE4b, ordered):** (1) `sf_atmos_bevy` resource with the raster (R16Uint own-climate texture, uploaded once)
and the table (storage buffer of rows, ≤ 64 × 31 f32, re-uploaded when `MapUpdate.table_changed`); (2) WGSL resolve
compute pass, twin of `ClimateRaster::site_at` + `WeatherMap::blend` (3 × 3 search, `border_blend`), writing A/B/C
textures at `storage_index`, full on table change, strips on recenter, skipped when uniform; a GPU-vs-CPU texel test
(readback ≤ 1e-3) like `sf_wind`'s twin; (3) `weather_sample(p)` in the cloud trace reads A (and B) by world texel with
the origin phase, uniform fast-path flag keeps today's scalars; (4) envelope shell (max base + thickness over the table)
and per-sample local base/thickness; (5) cloud shadow map reads the same map; (6) `clouds` example `--split clear,storm`
+ GPU timer A/B; (7) log the "no written weather map" reversal in ARCHITECTURE §10.

### WE4b as built, steps 1-2 (we4b, 2026-10-19): GPU map, clouds and cloud shadow map read it

**Reuse check (rule 8).** Bevy has no spatial weather texture or cloud weather hook; the resolve is one ~100-line compute
shader over our own raster/table, so it is ours. The WGSL lives in the Bevy-free `sf_weather` (`MAP_WGSL`, `src/map.wgsl`)
so the twin test runs it in raw wgpu like `sf_wind`'s; `sf_atmos_bevy` now depends on `sf_weather` (Bevy-free, small).

- **CPU side** (`sf_weather::map`): `ClimateRaster::gpu_cells` (each square's table row index, `u32` storage buffer:
  deviation from "R16Uint texture", 16 KB for 64², simpler indexing), `WeatherMap::gpu_rows` (A, B, C per row),
  `gpu_geometry` → `MapGpuGeometry::params(window, rects)` (96-byte uniform; raster origin split into integer + fractional
  world texels so f32 precision doesn't depend on world size), `resolve_rects(window, size, from)` (≤ 2 strips; `None` =
  full), `set_cpu_texels(false)` (`update` only tracks table/window; the GPU owns the texels), `set_rows` + `update_window`
  (a table without a `WeatherBook`, for examples/tools).
- **Resolve** (`map.wgsl`, entry `resolve`, 8 × 8, dispatch `(size/8, size/8, rects)`): per texel the 3 × 3 (reach)
  point-to-square search, `border_weight`, blend of two rows, written to a `Rgba16Float` 2-D array (layers A, B, C;
  deviation: C is `Rgba16Float` too, it carries absolute fog density) at `t mod size`. Ties: lower distance, then lower
  row index (= lower `ClimateIx`, the CPU's order).
- **Bevy** (`sf_atmos_bevy::weather_map`): main-world `CloudWeatherMap::sync(&map, update)` (raster re-upload on new
  geometry, rows when the table was resolved anew); render world uploads, schedules full / strip / no resolve (a resolve
  that never ran, e.g. no camera, forces a full one next frame), Core3d pass `weather_map` (GPU timer) before
  `clouds_shadow`. Uniform day or no map: `map_on = 0`, the clouds keep today's uniform scalars.
- **Clouds** (`cloud_density.wesl`): `map_a(p)` = one bilinear fetch of layer A by world texel (`map_offset` = origin /
  texel − window in f64 on the CPU, `map_wrap` = window mod size, Repeat sampler, clamped to the window's edge texels);
  `weather_sample_a(p, a)` feeds `weather_at` with the map's coverage, genus, wetness (darkness) and storm instead of the
  uniforms. `CloudQuality::map_per_sample` (High/Ultra `true`): every density sample (trace, light march, shadow map);
  `false` (Medium/Low, owner decision): `ray_a` = 3 fetches along the in-layer segment (trace) or the shadow texel's
  light ray, lerped by the sample's fraction; the light march reuses the primary sample's value.
- **Verified**: `cargo test -p sf_weather --test map_gpu_twin` (RTX 4060 Laptop): 3 climates over sea, 64² map of 32 m,
  full resolve and a recenter (2 strips covering exactly the entering texels) match the CPU twin within **9.2e-4**
  (`MapTexel::max_diff`, f16 storage; gate 1e-3). `clouds --split clear,storm` (Clear west of x = 2.5 km, Storm east,
  400 m band): zenith clear with the storm deck beyond the border, ground lit on the clear side (shadow map reads the
  map); visual check **WE-4b** awaiting owner.
- **Cost** (1280 × 720, RTX 4060 Laptop, High, averaged GPU timers): resolve **0.017 ms** (full 256², only on change);
  `clouds_trace` uniform storm 0.255 ms vs split per sample 0.253 ms vs per ray 0.250 ms; `clouds_shadow` 0.019 / 0.019 /
  0.025 ms. The extra fetch is lost in the noise at this resolution; re-measure at 1440p in the app (WE4c).
- **Not yet** (next, in order): (3/4) envelope shell + local base/thickness/density/erosion from layer B (the shell is
  still the uniform `Clouds` weather's; the example uses the east climate's heights), plume anchor hook; WE4c app wiring
  (`site_at`, `sf_weather_bevy` drives `CloudWeatherMap`); cirrus stays uniform (layer C unused by the clouds).

### WE4b as built, steps 3-4 (we4b2, 2026-10-19): local layer heights and cirrus from the map

- **Envelope shell.** `CloudWeatherMap` keeps `envelope(rows)` = lowest base, highest top (base + thickness) and most cirrus
  over the table's rows; the render world grows the uniform (camera) layer to it (`Clouds.shell_base`, `shell_thickness`,
  `cirrus_max`). A texel is a convex blend of rows, so every local layer lies inside the shell. The trace and the cloud
  shadow map march the shell (the shadow map's span cap is 4 × the shell's thickness); off the map it is the uniform layer.
- **Local layer from B.** Each density sample reads layer B (base, thickness, density, erosion) next to A, by the same
  rule: per sample on High/Ultra (`map_layer(p, 1)`, one more bilinear fetch of the same texel), else `ray_b` = 3 fetches
  along the in-layer segment, lerped (Medium/Low). `height_fraction(p, b)` and `cloud_density(p, h, w, b, detailed)`
  use the local base/thickness (detail octave z, density, erosion); the uniform `Clouds.base/thickness/density/erosion`
  are now only the no-map / uniform-day layer. The light march keeps the primary sample's B (the border band, ≥ 400 m,
  is wider than the march's reach at the steps it takes); the trace step `ds` stays the camera layer's thickness / 20
  (clamped by the step counts), so near clouds keep their sampling under a tall neighbour. Shader helpers are
  generic per layer (`map_layer`, `uniform_layer`, `local_layer`, `ray_layer`, `ray_local`, `_a`/`_b` wrappers).
- **Cirrus from C.** The sheet stays one sphere at the camera layer's `cirrus_height` (one ray-sphere hit); its cover is
  layer C's cirrus at the hit point (any map mode: one fetch per ray), gated by `cirrus_max`. The shadow map's cirrus
  term reads it too (`cirrus_optical_depth` → `cirrus_hit`).
- **Example**: `clouds --split` now sets the plugin's uniform layer to the **west** (camera) climate, as the app will.
- **Verified** (1280 × 720, RTX 4060 Laptop, High, averaged GPU timers, `clouds`): `--preset storm` (uniform) trace
  **0.210 ms**, shadow 0.020; `--split clear,storm` per sample trace **0.239 ms** (+0.03, the storm deck now 8.9 km tall
  beyond the border while the clear side is a 700 m layer), shadow 0.019, resolve 0.018; `--per-ray` trace 0.249 ms
  (no saving at this resolution; the 6 per-ray fetches + lerp cost about what the per-sample fetch does), shadow 0.019.
  All inside the WE4 gate (+0.15 High). Shots: from the clear side the storm rises as a wall at the border band with its
  tops against blue sky (`--scale 0.3` to see the tops in frame), clear zenith, cirrus streaks; no seam line in the
  sky; the `shadow` view shows only soft patches on the clear-side ground (no band at the border). Visual check WE-4b
  covers it (owner).
- **Not done** (by scope): plume anchor hook; cirrus height per climate (one sheet); a per-ray B saving on Medium/Low
  (could reuse `ray_a`'s 3 points with a 2-layer fetch helper if Medium/Low trace time matters at 1440p).

### WE4c as built (we4c, 2026-10-19): the game runs the world weather map

- **`sf_weather_bevy::WorldWeatherMap`** (`map: WeatherMap`, `camera: Option<DVec2>`): built from the app's
  `ClimateRaster` with `MAP_SIZE` 256 and `MAP_TEXEL_M` 64 m (a quarter of the 256 m band; 16.4 km window), CPU texels
  off. System `update_map`, chained after `evaluate` in `WeatherSystems::Evaluate`: `map.update(book, state, hours,
  hour, camera)` then `CloudWeatherMap::sync`; counter `weather/map/ms`. `site_at` / `climates` helpers.
- **App** (bevyrrowind): `WeatherSetup::raster` = the region cells' bounding box + 4 sea squares, rows along +Z (south),
  unowned squares dilated to the nearest region; without region cells one square of the fallback climate. `weather_site`
  writes `map.camera` and takes the site from `site_at` (the old `ClimateMap::border_at` and its 2-square band cap are
  gone; the band is now `BORDER_BAND_M` 256 m). Live control forces every raster climate, or one region's
  (`bevyrrowind/set {"weather", "region"}`). `Quality::cloud_map_per_sample`: High/Ultra per sample, Medium/Low per ray.
- **Verified:** builds, clippy clean (our code); scene `we3-ridge` with Ashlands = ashstorm, Ascadian Isles =
  thunderstorm: per-region forces apply, the zenith stays clear, the forced regions get clouds.
- **Bug fixed (we4c2, 2026-10-19): distant decks cut short, grey columns.** Cause, from the numbers: (1) **past-the-edge
  rule**: the resolve clamped positions past the raster per axis (and the trace clamped past the 16 km window per axis),
  so each edge square stretched into a stripe of constant width out to the 50 km cloud reach. Morrowind's raster is the
  island (~6 km, 117 m cells) + 4 sea squares, so a region touching the edge narrowly (Ascadian Isles' dilated sea)
  became a ~1 km stripe running to the horizon, drawn as a tall column tapering into the distance (8.9 km storm deck).
  (2) **step length**: the shell (700-8900 m with ashstorm + thunderstorm) puts the camera inside it, so every grazing ray
  spans ~50 km and `ds` = span / `max_steps` ≈ 780 m (coarse 1.56 km): near decks drew as banded slabs, narrow ones were
  stepped over. Fix: past the edge, positions are **projected toward the raster's (window's) center onto its edge**
  (`ClimateRaster::site_at`, `map.wgsl`, `map_layer`): each edge square owns a wedge that widens outward (a coastal
  region's weather reaches the horizon in its direction; the twin test still matches). The trace step is
  **`ds = local thickness / 20 + g·t`** (distance-proportional, as Nubis/UE scale steps with view distance), `g` sized so
  `max_steps` coarse steps grow t from the start to `end`, capped at span / `min_steps`: a thin deck keeps 35 m steps, an
  8 km tower 400 m, far samples ~2 km. `clouds --split clear,storm` (1280 × 720, High): trace **0.215 ms** (was 0.239),
  deck to ~1° above the horizon. Game `we3-ridge` (`shots/we4c2/`): the Ascadian Isles storm is a wide wall reaching the
  horizon, clear zenith, no striations; the Ashlands deck ends ~3 km out because the region does (geometry: a 2-3 km region
  under a 700 m base can't fill the sky below ~10°). **Not done:** 1440p re-time (trace, map pass, High per sample vs
  Medium per ray; no GPU pass timers over BRP and the dev window opened at 720p); the window's own radial projection
  recenters with the window (32-texel steps), so the sky past 8 km can shift on a recenter (unchecked).

### WE4d as built (we4d, 2026-10-19): distant precipitation shafts and far fog by location

**Reuse check (rule 8).** Bevy has no participating-media rain or spatial fog; this is ~90 lines inside our own cloud
trace (Design §6/§7's option (a)), no new pass, no new texture besides one map layer.

- **Map layer D** (`sf_weather`): `MapTexel::d` = precipitation extinction (Σ intensity × `PrecipKind.extinction`, the
  number `visibility_m` uses) and its extinction-weighted tint (`PrecipKind.tint` rgb, premultiplied so rows blend
  linearly), all **per km** (f16 and `MAP_EPSILON` friendly). New row channels `PRECIP_TINT_R/G/B` (`row::LEN` 34);
  `MAP_LAYERS` = 4 (resolve, Bevy texture, `gpu_rows`, twin test: still ≤ 1e-3). The map's envelope gains a flag: some
  row precipitates or the rows' height-fog densities differ.
- **Trace** (`cloud_trace.wesl` `far_weather`): when `Clouds.far_weather` (per-sample map = High/Ultra, the new
  `CloudQuality::far_weather` on, the flag set), 8 quadratic steps from 32 m to min(depth, half the map window = 8 km).
  Per step, layers B/D (and C with fog on): **shaft** σ = max(D(x) − D(camera), 0) × the clouds' own weather noise
  coverage at the point the rain left the base (sheared downwind 0.35 m per m of fall: slanted curtains under the
  storm's cells) × a 150 m fade below the local base × `ramp(32 m, 128 m, t)`; lit by the key light through a column
  estimate of the deck above (coverage × density × thickness from the map: dark under the cell, bright at its sunlit
  edge) + the sky/bounce fill greyed and dimmed by the wet deck, × the tint. **Far fog** σ = max(C.z(x) − C.z(camera), 0)
  on the camera layer's height profile, lit like the fog (albedo, key light × phase, sky × ambient). Composited in
  front of the clouds with aerial perspective at its mean distance; the composite's camera fog then lies over it, so
  curtains fade into the far fog.
- **Deviation (near = far):** shafts add only the **difference** from the camera's precipitation, like §7's fog, not
  the full extinction from 128 m out. At the camera both terms are zero, so local rain (sf_precip particles, the
  camera's fog) looks as before and walking into a storm fades the curtain out exactly as the camera's weather fades in
  over the band (no pop, no double rain). Not drawn: a camera in rain looking at a drier region (can't subtract in a
  front layer), virga fraction and vertical fall streaks (no per-preset virga; noise cells only).
- **Verified:** `cargo test -p sf_weather` (18 + GPU twin with 4 layers), clippy pedantic clean. `clouds --split
  clear,storm --time 14` (example rows: wet decks rain 2e-3/m, grey-blue): an opaque dark grey-blue curtain under the
  storm from the ground to its base, a sunlit gap between cells with slanted edges, pillars on the clear side untouched
  (`--no-far` A/B, `--far-fog <m>` for an east fog). Game `we3-ridge`, Ashlands forced thunderstorm: grey rain veil
  over the far terrain on the storm side fading into the haze, clear side and zenith unchanged (one shot, owner check
  WE-3).
- **Cost, 2560 × 1440** (RTX 4060 Laptop, High, `clouds --size 2560x1440`, two runs each): trace with shafts
  **0.601 ms** vs `--no-far` **0.541 ms** = **+0.06** (12 steps: +0.07, so 8). **1440p re-time** (we4c2's open item):
  split per sample 0.54 ms (no shafts) vs `--per-ray` 0.558 ms vs uniform storm 0.512 ms; shadow 0.019-0.024;
  weather_map resolve 0.018 ms (on change). Per-ray still saves nothing at 1440p; the map costs ≈ +0.03 over uniform.

### WE4e as built (we4e, 2026-10-19): lightning on distant storms

- **Placement (`sf_weather::world_strikes`, pure):** a 2 km world grid within 16 km of the camera; per cell and 1/30 h bucket a
  hashed Poisson candidate count (cap rate spread over the disk so the disk carries the climates' `lightning_per_hour`), a hashed
  position in the cell and time, thinned by the rate of the climate that owns the spot (`ClimateRaster::site_at`, blend at the
  strike time). Pure in (seed, cell, bucket), independent of frame cuts: co-op peers agree with no messages. Test
  `world_strikes_are_deterministic_and_placed_in_stormy_cells`. When a `WorldWeatherMap` exists the frame's strikes come from it
  (`CurrentWeather::strike_pos` holds each world X, Z); azimuth, distance and the thunder delay (distance / 343) are from the camera.
- **Glow:** `Clouds::glows` (4 slots, render-space position at 35 % of the layer, intensity `flash_at × LIGHTNING_GLOW`) added as
  emission to the march's scattered light with a 2.5 km Lorentzian falloff (`cloud_trace.wesl`): the nearest-brightest 4 live
  strikes; the old uniform ambient gain remains only for camera-relative strikes. The flash light and rain brightening scale by
  `near_factor` (full within 3 km, none past 9 km), so a far strike lights its cloud only.
- **Cost:** 4 vec4 reads + a branch per lit cloud sample (zero-intensity slots skip the math); the placement loop is 81 cells ×
  1-2 buckets of hashing per frame. Not timed on the GPU.
- **Not done:** Nubis' stable mask so the temporal pass holds the flash (the glow rides the history blend today; flashes last
  ~0.3 s), the owner's visual sign-off (agent shots caught no strike), per-climate flash radius.
