# Sky and lighting (M2 slice)

Status: implemented (2026-10-11)
Crates: `sf_sky` (Bevy-free core), `sf_sky_bevy` (plugin; cargo feature `solari`)

## Goal

A physically based sky that drives scene lighting from a game clock: Bevy's
atmosphere, a sun and a moon placed by time of day, season and latitude,
camera exposure that follows them, and a runtime render-path setting
(raster or ray-traced with Bevy Solari) that falls back to raster when the
GPU can't ray trace. Punctual lights keep working on the ray-traced path
through emissive proxies. Everything is data (`SkySettings`), so a game
supplies its own calendar, latitude, atmosphere and curves.

Non-goals (M6, hooks below): clouds, weather presets and blending, fog,
precipitation, stars, moon phases on the disk, surface wetness.

## Canonical technique

- **Sky**: Hillaire 2020 LUT atmosphere (Bevy's `Atmosphere`), the industry
  default (UE5 SkyAtmosphere). The medium (Rayleigh, Mie, ozone) comes from
  our `AtmosphereModel`. Bevy 0.20's `ScatteringMedium::earth` swaps Mie
  scattering and absorption relative to the literature (same transmittance,
  a 9× dimmer sun halo), so we build the medium ourselves.
- **Sun and moon**: low-precision ephemeris (Meeus ch. 13/25 reduced to a
  circular orbit): declination from axial tilt and season, hour angle from
  local solar time, equatorial to horizontal. Declination is held per day
  (days are symmetric, ≤ 0.4°/day error). The moon lags the sun by its
  phase; brightness follows the Lambert phase law. Both are
  `DirectionalLight` + `SunDisk`; Bevy's atmosphere scatters every
  directional light, so moonlit skies come for free.
- **Physical units**: sun 130 000 lux above the atmosphere. The raster PBR
  shader applies atmosphere transmittance and horizon disk visibility per
  pixel, so the light stays unattenuated (twilight keeps working).
- **Exposure**: scheduled EV100 from a sun-elevation curve, not histogram
  auto exposure. Bevy's `AutoExposure` meters to mid-grey (nights turn
  grey) and lags; a schedule is deterministic for screenshots and
  benchmarks. Apps can opt out per camera and use `AutoExposure`.
  **Calibration check (2026-10-12, portals.md "Owner findings and fixes")**:
  sky and interiors share one physical light scale: noon EV100 ~14 (real
  sunlit 15), interiors EV100 7 (`LightCalibration`, real homes 5-7,
  offices 7-8), night ~0.5 (real full moon -3 to -2; the 1 lx moon is a
  readability choice). No constant changed. Seeing two worldspaces at once
  (portals) needs the viewer's exposure: base EV100 7 on every camera
  plus `AutoExposure` with a compensation curve that reproduces this
  schedule (night -4.5, interior -3.6, day -2.3 EV), opt-in through
  `SkyCamera { exposure: false }`; a new sky camera now starts at the
  scheduled EV100 (it got `Exposure::default()`, 9.7, for one frame).
- **Shadows**: one shadow-casting directional light at a time (sun above
  −1°, else the moon above 3°), 4 cascades to 300 m by default; the exterior app derives
  504 m from the far-field handoff (2048², first bound 10 m), and past it terrain, proxies and
  impostors take the terrain far shadow through `directional_shadow_factor` with a dithered
  cascade end (weather.md "Distant shadows", as built 2026-10-12).
- **Ray-traced path** (Bevy Solari 0.20, verified in source and on an RTX
  4060): lights from `DirectionalLight`, emissive `StandardMaterial` meshes
  and the camera's environment map (including `AtmosphereEnvironmentMapLight`).
  **No `PointLight`, `SpotLight` or `RectLight`** (upstream "near future").
  So ARCHITECTURE §7.5's interim stands, refined:
  - `PointLightProxy`: a sphere registered **only** with the ray tracer
    (`RaytracingMesh3d` without `Mesh3d`): it lights, shadows and reflects,
    but is never rasterized (no visible blob, zero raster cost). Luminance
    `Φ / (4π² r²)` gives exactly the point light's intensity from afar
    (raster and ray-traced midnight shots match). The raster light stays
    for forward-rendered transparents, with shadow maps off.
  - Solari's directional lights ignore atmospheric transmittance, so on
    that path the sun gets the CPU transmittance (`SkyLight::ground_light`).
- **Raster path** stays forward. The depth prepass (occlusion culling,
  ARCHITECTURE §10) is off in exteriors for now: Bevy 0.20's depth-only
  prepass ignores the `VisibilityRange` dither (far-field.md "Bevy 0.20
  pitfalls"). With a motion-vector prepass the same draws get a fragment
  stage that applies the dither, so DLSS (which needs depth + motion
  vectors) runs hole-free. Upstream fix for the depth-only prepass, two
  lines in Bevy 0.20: in `bevy_pbr` `prepass/mod.rs`, `fragment_required |=
  mesh_key.contains(MeshPipelineKey::VISIBILITY_RANGE_DITHER)`; in
  `pbr_prepass.wesl`, call `visibility_range_dither` in the `@else` (no
  `PREPASS_FRAGMENT`) fragment too. Material `depth_bias` already applies
  in the prepass. Solari forces deferred globally when its plugin
  loads (even on GPUs without ray queries); the switch owns
  `DefaultOpaqueRendererMethod` and re-prepares materials (a one-off hitch,
  like applying a graphics setting).
- **Upscaling** (2026-10-12): Bevy's DLSS (`bevy_anti_alias::dlss`, crate
  `dlss_wgpu` 6.0.0 on NVIDIA's DLSS SDK 310.9.1), the industry's default
  temporal upscaler on RTX (UE5/Unity/Frostbite ship it next to TSR/FSR).
  Super Resolution on the raster path, Ray Reconstruction (denoise +
  upscale) on the ray-traced path, one quality setting for both. Off on
  non-NVIDIA GPUs and in builds without the `dlss` feature; Bevy 0.20 has
  no FSR/XeSS, and its own TAA (`TemporalAntiAliasing`) is the obvious
  vendor-neutral fallback to wire later.

Reuse check (2026-10-11): no maintained sky crate fits. `bevy_weather` 0.2
(MIT/Apache, Bevy 0.19, one maintainer) wraps the same Bevy atmosphere and
adds clouds, stars and moon phases: a reference for M6. `bevy_sky_gradient`
0.4 (0.19) is stylized, not physical. `bevy-volumetric-clouds` 0.2 (0.18)
and `bevy_fast_clouds` (0.19) are M6 cloud candidates.
`kj-bevy-realistic-sun` targets 0.20 but has a non-standard license. The
sun math is ~150 lines with tests, so we own it.

## Data model (`sf_sky`)

- `Calendar { hours_per_day, days_per_year }`; `SkyClock { days: f64,
  time_scale, paused }`. Saves store `days`.
- `Orbit { latitude_deg, axial_tilt_deg, spring_equinox_day,
  north_heading_deg, lunar_month_days, moon_phase_at_epoch }`. Engine frame:
  Y up, north −Z (rotated by `north_heading_deg`), east +X.
- `AtmosphereModel { inner_radius, outer_radius, ground_albedo,
  terms: Vec<MediumTerm> }`, falloffs matching Bevy's normalized-altitude
  functions; `transmittance(altitude, elevation, haze, samples)`.
- `SkySettings` (serde, all fields defaulted): the above plus sun/moon
  illuminance and angular size, `exposure` and `ambient` `KeyCurve`s
  (piecewise-linear over sun elevation), shadow handover elevations.
- `SkySettings::evaluate(&SkyClock, &Weather) -> SkyState`: per light the
  direction, elevation, top-of-atmosphere illuminance, ground
  transmittance and shadow flag; plus EV100 and the ambient floor.
- `Weather { cloud_cover, overcast_dimming, haze, wind }` (M6 hook): cover
  dims direct light; haze scales aerosol density (the GPU medium is rebuilt
  when it moves by ≥ 0.02). `Wind::velocity_at(pos, t)` is the one wind
  API for grass, cloth, particles and clouds; M6 swaps its analytic gusts
  for a sampled field without changing callers. Positions passed in must
  be origin-stable (world-coordinates.md phase).
- `RenderPath { Raster, RayTraced }` with `resolve(supported)`;
  `sphere_light_luminance(lumens, radius)`.

## Interfaces (`sf_sky_bevy`)

- `SkyPlugin { settings, clock, render_path }`. Resources: `SkyConfig`,
  `SkyTime`, `WeatherState` (inputs); `CurrentSky` (output);
  `RenderPathSetting` (requested), `ActiveRenderPath` (running),
  `RayTracingSupport`; `SkyRenderOptions` (sea level, cascades, ambient
  tint, proxy defaults, auto-registration, ReSTIR).
- Spawns the `SkyAtmosphere`, `SunLight` and `MoonLight` entities unless the
  app already has them.
- `SkyCamera` on a 3D camera adds `AtmosphereSettings`, `Exposure`,
  `AtmosphereEnvironmentMapLight` and drives exposure and an
  `AmbientLight` floor. Removing it (interiors) removes the atmosphere
  components, and the sun and moon go dark when no sky camera is left.
- Ray-traced path (feature `solari`, adds `SolariPlugins`): snapshots and
  restores the camera components Solari changes (MSAA, prepasses,
  `OcclusionCulling`, main texture usages, ReSTIR double buffers); mirrors
  `Mesh3d` + `StandardMaterial` entities into `RaytracingMesh3d` unless
  `NoRayTracing`; proxies every `PointLight` unless `NoLightProxy`.
- Upscaling (`upscale.rs`; feature `dlss` for DLSS, which no longer
  implies `solari`): `UpscalingSetting(sf_sky::Upscaling)` (requested:
  `Off` or `Dlss(UpscaleQuality)`, quality `auto`, `native` (DLAA),
  `quality`, `balanced`, `performance`, `ultra-performance`; serde, so a
  settings file holds it), `ActiveUpscaling` (running), `UpscalingSupport`
  (SR / RR available), `TemporalHistory { reset }` (camera cuts, teleports,
  origin shifts). On raster, `RenderPathCamera`s get `Dlss<SuperResolution>`
  plus what it requires (jitter, mip bias, depth + motion-vector prepass)
  and `Msaa::Off`; a snapshot restores the camera when it is turned off at
  runtime. On the ray-traced path the quality goes to
  `Dlss<RayReconstruction>` (`Auto` when upscaling is off). ReSTIR is
  `SkyRenderOptions::ray_tracing_restir: Option<bool>`: `None` = on without
  a denoiser, off under Ray Reconstruction (Bevy's advice; same image,
  1.4 vs 3.5 ms of lighting in the Census Office at 1440p).
- Builds with `dlss` need `DLSS_SDK`, `VULKAN_SDK` and `LIBCLANG_PATH`;
  both repos' `.cargo/config.toml` point them at `../sdk` (outside the
  repos: the SDK may not be redistributed; `bash ../sdk/fetch.sh` fetches
  the ~140 MB subset with size checks). The game ships `nvngx_dlss.dll`
  (SR) and `nvngx_dlssd.dll` (RR) next to the executable (Bevyrrowind's
  `build.rs` copies them) plus NVIDIA's license notice (DLSS Programming
  Guide §9.5) in a distribution.
- Systems run in `Update` in `SkySystems::{Clock, RenderPath, Apply}`.
  `SkyRenderOptions::sea_level` is the hook for the floating origin
  (`sf_world_bevy` sets it when the origin cell moves vertically).

## Verification

- `cargo test -p sf_sky`: equinox and solstice noon elevations at several
  latitudes, polar day and night, sunrise east at 6:00, monotonic and
  symmetric day curve, longer summer days, moon phase geometry and
  brightness, transmittance (reddening, monotonic, zero below the horizon),
  disk visibility, curves, wind, serde round-trip.
- `cargo run -p sf_sky_bevy --features solari --example sky -- --screenshot
  <png> --time <h> [--path rt] [--day d] [--switch-at n]`: test scene with
  lamps. Checked 2026-10-11 on an RTX 4060 Laptop (Vulkan): raster 6:00,
  noon, 19:30 sunset, 20:12 and 21:00 twilight, midnight (new and full
  moon); ray-traced noon, sunset and midnight (proxies light the scene,
  brightness matches raster); runtime switch both ways.
- DLSS (2026-10-12, RTX 4060 Laptop, AC): `cargo test -p sf_sky` (upscaling
  parse/fallback/serde), `cargo test -p sf_sky_bevy` (reports `Off` without
  the feature); Bevyrrowind `view ... --rt [--off denoiser] [--dlss q]` and
  `view Balmora --dlss quality|performance|native [--prepass]`, screenshots
  in `D:/Gamedev/Bevyrrowind/shots/dlss/`. Balmora 1440p medians: native
  11.9 ms, DLSS quality 8.9 (SR pass 2.1 ms), performance 9.2 (SR 2.8 ms),
  DLAA 11.7, depth+MV prepass with occlusion culling 13.9.

## Open questions

1. **Sky double attenuation on the ray-traced path.** The sun's color is
   shared by Solari and the sky, so pre-attenuating it for Solari also
   darkens and reddens the sky at low sun. Checked 2026-10-12: no
   workaround inside our code (Solari reads every extracted
   `DirectionalLight` and ignores `RenderLayers`, so a Solari-only light
   can't be split from the sky's). Fix: a small `bevy_solari` patch, either
   sampling the atmosphere's transmittance LUT for directional lights
   (upstream-quality) or an extracted per-light tint the plugin fills with
   `SkyLight::ground_light` (smaller); vendored under `[patch.crates-io]`
   in both workspaces until upstream takes it. Not done yet.
2. **Denoising.** Done and verified 2026-10-12 (RTX 4060 Laptop, driver
   617.14, Vulkan): DLSS Ray Reconstruction denoises `--rt` (Census Office:
   `shots/dlss/census_hall_rt_rr.png` vs `_rt_noden.png`). Cost at 1440p:
   RR 5.2 ms (`auto`, ~performance), 8.1 ms (`quality`), 17 ms (`native`);
   frame 8.4 / 12.9 / 26.5 ms vs 15.4 ms undenoised with ReSTIR and 3.5 ms
   raster. Non-NVIDIA GPUs have no denoiser in 0.20.
3. **Content compatibility.** Solari takes only `StandardMaterial` and
   meshes with exactly position/normal/uv0/tangent and u32 indices
   (compressed attributes or vertex colors are skipped silently), and it
   ignores visibility. Resolved for cooked models in interiors
   (2026-10-11): an entity's `RayTracingMesh` replaces its `Mesh3d` for
   the ray tracer, the mirror registers visible entities only, and
   `sf_model_bevy` builds the variants at load time from one LOD
   (`ray_tracing_lod`), with the other levels marked `NoRayTracing`.
   Exteriors (checked 2026-10-12): `--rt` runs with RR, but the streamed
   world has no ray-tracing content yet, so it renders flat without
   shadows (`shots/dlss/balmora_rt_rr_untraced.png`). Needed, in order:
   `sf_world_bevy` templates loading with `ray_tracing_lod` and tagging
   one LOD per placement; `NoRayTracing` on far-field proxies
   (`sf_lod_bevy`); a terrain RT proxy (RT-only `StandardMaterial` meshes
   from `TerrainHeights` around the camera with macro albedo, like the
   light proxies; `sf_terrain_bevy`); then measure TLAS size and frame time
   in Balmora. Still open everywhere: NIF-path models, skinned meshes,
   alpha-masked and blended surfaces. Census Office RT shows black floor
   bands at rug edges with and without the denoiser (content, not DLSS).
   Fixture meshes that enclose a light (lanterns) occlude its proxy: the
   adapter should mark them `NoRayTracing` or the proxy radius must clear
   them.
4. Night sky has no stars (M6), and the moon disk has no phase shape.
5. Spot lights are not proxied (a sphere has no cone). Revisit when Solari
   gains punctual lights; then delete `proxy.rs`.

From J9 (2026-10-12, industry comparison below):

6. **Raster ambient occlusion and contact shadows (P1).** Bevy 0.20 ships
   both (`ScreenSpaceAmbientOcclusion`, based on Intel XeGTAO, and
   `ContactShadows`), but both need the depth prepass, which the exterior
   camera leaves off (dither pitfall). Every engine in the comparison
   ships GTAO/HBAO-class AO on its raster path; ARCHITECTURE §7.5 lists
   contact shadows as part of the fallback. Unblocked by audit job J5 (the
   two-line prepass fix); then add `ScreenSpaceAmbientOcclusion` +
   `ContactShadows` to `SkyCamera`'s raster set with a preset quality and
   measure (expected ~0.5–1 ms at 1440p, unmeasured).
7. **Raster indirect light for interiors (P2).** Raster interiors get a flat
   `AmbientLight` floor plus point lights; there's no bounce. Bevy 0.20
   has irradiance volumes (ambient cubes, Mitchell 2006) and reflection
   probes but no baker. Candidate: bake one irradiance volume per interior
   at bake time with Solari or a CPU path tracer (UE volumetric lightmaps /
   Unity APV-style), only for interiors (static lighting, no time of day).
   Blocked by: light probes ignore `RenderLayers` (portals.md), the bake
   budget (≤ a few minutes for all interiors), and Solari's content gaps
   (open question 3). Measure the visual gain on 3 interiors first.
8. **Bloom (P2, small).** The game cameras have no `Bloom` (the `sky`
   example uses `Bloom::NATURAL`). Universal in modern engines and it
   carries the sun glare, lamps and magic effects. Wire a preset setting;
   weather-adapter's "glare later" rides on it.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
Bevy features checked in the 0.20 source (`bevy_pbr`, `bevy_light`,
`bevy_post_process`). "Unverified" marks claims from memory not re-read
this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Physically based sky + aerial perspective | Hillaire 2020 LUTs: UE4.26+/UE5 SkyAtmosphere (transmittance, multi-scattering, sky-view and camera aerial-perspective froxel LUTs); Frostbite (Hillaire's origin, 2016 → 2020); Unity HDRP physically based sky (precomputed, Bruneton-style, unverified); RDR2 own atmosphere (Bauer, SIGGRAPH 2019 Advances, unverified) | Bevy `Atmosphere` (same LUT set incl. a 32 km aerial-view LUT), our medium | **match** |
| Sky as environment lighting | UE SkyLight real-time capture; Unity HDRP sky ambient probe | `AtmosphereEnvironmentMapLight` (filtered from the atmosphere) | **match** |
| Sun/moon from clock and latitude | UE SunPosition plugin (date, latitude); most open worlds use a hand-authored time-of-day curve instead | ephemeris + curves | **match** (more physical than most) |
| Exposure | UE: histogram auto exposure, physical camera, **exposure compensation curve** by average luminance, local exposure (5.1+); Unity HDRP: automatic histogram or fixed/physical, curve mapping; Bethesda/OpenMW: no physical exposure (LDR-era lighting, eye adaptation as a post effect) | scheduled EV100 by sun elevation; with portals, `AutoExposure` with a compensation curve (portals.md) | **match** (the portal2 setup is the UE pattern). **Local exposure** (UE 5.1, bilateral-grid tone mapping) is not in Bevy 0.20: helps interior-looking-out shots; noted, not a job |
| Tone mapping | UE filmic (ACES-fitted), Unity ACES/Neutral, Blender/Godot AgX | Bevy default `TonyMcMapface` | **match** (a curve choice, not a technique gap) |
| Shadows | UE virtual shadow maps (+ CSM fallback); Unity CSM + cached shadows; RAGE CSM + far shadow (unverified); Decima CSM + cached far cascades (unverified) | 4 CSM + terrain far shadow (weather.md) | **match** for CSM; VSM / cached far cascades = audit row 14 (P2) |
| Contact shadows | UE per-light contact shadows; Unity HDRP contact shadows; Bevy 0.20 `ContactShadows` (needs depth prepass) | **off** (prepass off in exteriors) | **GAP (P1)**: open question 6 |
| Ambient occlusion | UE SSAO/GTAO (+ Lumen short-range AO); Unity HDRP GTAO; Fallout 4 HBAO+, Skyrim SE SSAO (unverified); Bevy 0.20 SSAO = XeGTAO port (needs depth + normal prepass) | **none** on raster (Solari does it implicitly) | **GAP (P1)**: open question 6. Exteriors also lack large-scale sky occlusion: weather.md's terrain sky-visibility bake covers that |
| Indirect diffuse (GI) | UE5 Lumen (software/hardware RT), volumetric lightmaps; Unity Adaptive Probe Volumes with lighting scenarios blended at runtime (the time-of-day pattern), Enlighten; RAGE/Decima baked probes per time of day (unverified detail); Bethesda Skyrim/FO4: none (ambient per cell/region), Starfield real-time GI (unverified); OpenMW: per-cell ambient only | raster: ambient floor + sky env map; ray-traced: Solari ReSTIR GI | **match** on the high end (Solari = Lumen class). **Differ** on raster: we match Bethesda/OpenMW, not modern raster engines. Interior baked probes: **GAP (P2)**, open question 7. Exterior time-of-day probes: skipped (sky env map + sky-visibility bake is the cheap equivalent) |
| Reflections | UE Lumen reflections / SSR + reflection captures; Unity SSR + reflection probes | env map; SSR available in Bevy, not wired | **differ**: source content is mostly diffuse; revisit with PBR generation (M7) |
| Many-light culling | clustered/tiled forward or deferred everywhere; UE MegaLights (5.5, stochastic) | Bevy clustered forward; Solari for RT | **match** |
| Bloom / glare | universal | not on game cameras | **GAP (P2, small)**: open question 8 |
| Upscaling | DLSS/FSR/XeSS/TSR | DLSS (SR + RR); TAA fallback not wired | **match** on NVIDIA; vendor-neutral path open (above) |

**Sources** (checked 2026-10-12):
- Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique", EGSR 2020: https://sebh.github.io/publications/egsr2020.pdf
- UE, Sky Atmosphere component: https://dev.epicgames.com/documentation/en-us/unreal-engine/sky-atmosphere-component-in-unreal-engine
- UE, Auto exposure / local exposure (Eye Adaptation): https://dev.epicgames.com/documentation/en-us/unreal-engine/auto-exposure-in-unreal-engine (page not re-read; unverified detail)
- Unity, Adaptive Probe Volumes lighting scenarios and blending: https://docs.unity3d.com/Manual/urp/probevolumes-bakedifferentlightingsetups
- Unity, APV lighting panel: https://docs.unity3d.com/Manual/urp/probevolumes-lighting-panel-reference
- Intel XeGTAO (Bevy's SSAO source): https://github.com/GameTechDev/XeGTAO
- Bevy 0.20 source: `bevy_pbr/src/{ssao, contact_shadows.rs, light_probe/irradiance_volume.rs, atmosphere}` (irradiance volumes: "Bevy doesn't currently come with a way to bake"; points at Blender + `bevy-baked-gi`).
- Mitchell, "Shading in Valve's Source Engine", SIGGRAPH 2006 course (ambient cubes).
