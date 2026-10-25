# Sky and lighting (M2 slice)

Status: implemented (2026-10-11)
Crates: `sf_sky` (Bevy-free core), `sf_sky_bevy` (plugin; cargo feature `solari`)

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: implemented (2026-10-11); ROADMAP 1.3 (TAA/DLSS), 1.5 (GTAO), 1.7 (night checks SK-2, NS-2..4) in progress. Built: Bevy atmosphere sky, sun and moon from clock/season/latitude, scheduled exposure, night sky, raster or Solari render path with raster fallback, DLSS Ray Reconstruction.
> Crates / key types: `sf_sky` (Bevy-free: `SkySettings`, `SkyClock`, `SkyState`), `sf_sky_bevy` (`SkyPlugin`, `SkyCamera`, `RenderPath`; feature `solari`).
> Decisions: own `AtmosphereModel` medium, not Bevy's `ScatteringMedium::earth` ("Canonical technique"); sun 130 000 lux, per-pixel transmittance; EV100 schedule, not `AutoExposure`. Open: Solari sky double attenuation needs a `bevy_solari` patch ("Open questions" 1).
> Read next: "Interfaces" for plugin use; "Night sky and drawn bodies"; "Data model".
<!-- /agent-summary -->

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
- **Exposure range** (lighting job 2026-10-13, owner option A):
  `SkySettings::exposure_range` (`ExposureRange { min_ev100, max_ev100 }`,
  UE's auto-exposure Min/Max EV100) clamps the camera EV100 whatever
  drives it: the schedule clamps its sample, histogram auto exposure gets
  the clamp baked into its compensation curve
  (`ExposureRange::clamp_compensation`: Bevy's `AutoExposure` has a
  metering range but no output clamp, so the curve is sampled every 0.5 EV
  and limited to `x + base - clamp(base - (c - x))`). The framework default
  is wide (-10..20, no clamp). Why a floor: local lights are calibrated
  once for the interior EV100 (7); a night exterior opening to EV 0.5-2
  showed them 30-90x brighter than calibrated and blew out walls near
  lamps. Scaling lights by exposure (the alternative) was rejected: lights
  stay physical and the camera stops opening. The game sets the floor
  (Bevyrrowind: `LightCalibration::exposure_floor_ev100`, 3.5, so lights
  read at most 2^3.5 = 11x their calibration) and pays for night
  readability with a brighter night ambient floor (`ambient` curve, 4
  nits below -12°) instead of a wide-open camera, as games do (an
  exaggerated moon/sky light at a clamped exposure). Interiors keep EV100
  7 on the schedule; under viewer auto exposure they meter at or above
  the floor (a dark cave stops at 3.5 instead of ~2.6).
- **Shadows**: one shadow-casting directional light at a time (sun above
  −1°, else the moon above 3°), 4 cascades to 300 m by default; the exterior app derives
  504 m from the far-field handoff (2048², first bound 10 m), and past it terrain, proxies and
  impostors take the terrain far shadow through `directional_shadow_factor` with a dithered
  cascade end (weather/distant-shadows.md "Distant shadows", as built 2026-10-12).
- **Local-light shadows** (`shadow_budget.rs`, lighting job 2026-10-13):
  point lights marked `BudgetedShadow { priority }` compete for
  `PointShadowBudget::max` cube shadow maps (Bevy's
  `PointLight::shadow_maps_enabled`, faces `PointLightShadowMap::size`).
  Ranking: `shadow_importance` (strength at the viewer × screen coverage ×
  priority, lights out of view included; [shadows.md](shadows.md)); a holder ranks 1/0.8 better (hysteresis). Strength
  fades with distance (full to `fade_start` 15 m, none past `fade_end` 25 m)
  and over time (`fade_time` 0.3 s) when a light gains or loses its slot;
  a newcomer waits for a free slot, so at most `max` maps exist. Bevy's
  point light has no shadow strength, so a fading light is split: the
  shadowed light carries `fade` of the power and an unshadowed child twin
  (`ShadowTwin`, created on the first slot, hidden otherwise) the rest
  (`I (1 - s + s v) = s I v + (1 - s) I`), applied in `PostUpdate` and
  undone in `First` so game code (flicker) keeps writing full power.
  Industry: Unity HDRP (max shadows on screen + per-light shadow fade
  distance), UE (per-light shadow fade / resolution fade), Bethesda
  (a small cap of shadowed lights); OpenMW has no point-light shadows
  (comparisons from engine docs as remembered, not re-read). Casters
  nearer a light than its `shadow_map_near_z` don't shadow it (the game
  uses 0.25 m so a lantern's cage doesn't stripe the room). Costs are in
  the game's presets (Bevyrrowind `Quality::point_shadows`).
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
  no FSR/XeSS.
- **Temporal resolve as the base** (ROADMAP 1.3, 2026-10-19): every preset
  from Medium up resolves temporally, as UE5 (TSR), Frostbite and Decima
  do: stochastic effects (soft shadows, dithered LOD fades, fog and cloud
  noise) are built to converge over frames, and without a resolve they read
  as grain (WE-2/WE-3, SH-12, WA-1). `Upscaling::Taa` is Bevy's TAA at native
  resolution, the vendor-neutral path and DLSS's fallback; `Off` (MSAA) is
  the Low preset only.

Reuse check (2026-10-11): no maintained sky crate fits. `bevy_weather` 0.2
(MIT/Apache, Bevy 0.19, one maintainer) wraps the same Bevy atmosphere and
adds clouds, stars and moon phases: a reference for M6. `bevy_sky_gradient`
0.4 (0.19) is stylized, not physical. `bevy-volumetric-clouds` 0.2 (0.18)
and `bevy_fast_clouds` (0.19) are M6 cloud candidates.
`kj-bevy-realistic-sun` targets 0.20 but has a non-standard license. The
sun math is ~150 lines with tests, so we own it.


## Night sky and drawn bodies (S2/S3, nightsky 2026-10-13)

What is *lit* stays physical; what is *drawn* is artistic where games are.

- **Sun disk** (`SunDiskLook`): the drawn disk has its own angular size
  (default 2°, Morrowind 2.5°, `--sun-size`), separate from the light's
  physical 0.53° (`sun_angular_size`: horizon fade, soft shadows). Bevy's
  `SunDisk` divides the light by the disk's solid angle, so a bigger disk
  keeps the same total light: still ~10⁸ nits, far above white, so it
  saturates, blooms and is hidden by cloud transmittance in the composite.
  Glare (`glare`, share of the sun's light in an exponential halo of
  `glare_size`) is drawn in the night pass, before the clouds, so clouds
  Built (sunglare 2026-10-19): the weather scales the glare with `sf_sky::Weather::sun_glare` (`[0, 1]`, default 1; `SkyTerms::sun_glare` per preset, blended on the SKY channel with its lead/lag like haze, `WeatherFrame.sky`, applied by `sf_weather_bevy::apply_sky`); the night pass multiplies `glare` by it (no `SkySettings` rebuild, nothing pops). The disk itself dims through cloud transmittance and `direct_light_factor`.
  hide it. Industry: UE's `SkyAtmosphere` draws the light's source angle
  (physical by default, artists raise it) plus bloom/lens flare; Frostbite
  (Hillaire 2016/2020) keeps a physical disk and relies on bloom; RDR2 and
  most open worlds draw a large disk with a strong halo. We do the latter
  with the disk energy-conserving.
- **Moons** (`Moon`, any number, `MAX_MOONS` = 4 drawn): each has its own
  phase cycle (`phase_period_days`, `phase_at_epoch`, optional
  `phase_steps` for one texture per phase), path (`MoonPath::Synodic`:
  physical, position follows phase; `MoonPath::Circuit`: its own circuit
  period, tilt and declination for fantasy skies), size, tint, full-moon
  illuminance and phase textures (asset paths; none: a shaded sphere lit
  by the phase). `SkySettings::moons` empty = one Earth moon from the orbit.
  Lighting: one moonlight (`SkyState::moon`) toward the brightest moon at the
  ground, with the illuminance that makes its ground light the sum of all
  moons (Lambert phase law each): one shadow caster, as games do.
- **Stars** (`StarField`): procedural, three cube-map hash layers (few
  bright, many faint, warm-to-blue tints, scintillation near the horizon).
  Each star is a Gaussian about a pixel wide whose integral is its
  illuminance (`brightest_lux`, Sirius-class 2e-5 lx), so brightness does
  not depend on resolution. Hidden physically by the sky's own brightness
  and the atmosphere's extinction (per-elevation transmittance table), plus
  an artistic `visibility` curve over sun elevation (−4° → −14°).
- **Render** (`sf_sky_bevy::night`): one fullscreen pass at the far plane,
  depth-tested (`GreaterEqual` in reverse Z) so only sky pixels run, additive
  onto Bevy's sky after `render_sky` and before the transparent pass; the
  cloud composite (ordered after it by the same `SkyOrderScaffold`
  discovery; add `SkyPlugin` before cloud plugins) then dims and hides
  everything behind clouds. The moons' disks hide stars and farther moons
  (`occluder`, the unlit side too). The moons' `SunDisk`s are zero-intensity
  (the lights still scatter in the atmosphere).
- **Why our own pass**: Bevy 0.20's atmosphere draws only uniform `SunDisk`s
  (no texture, no phase, no stars); a transparent mesh would draw over the
  clouds and an opaque one would replace the in-scattered sky in front of it
  (a dark moon by day). No maintained crate draws stars/moons into Bevy's
  atmosphere (`bevy_sky_gradient` is a gradient-sky replacement).
- **Cost** (1440p, RTX 4060 laptop, sky-filled view): night pass 0.26 ms at night (stars + full Masser; shared machine), 0.08 ms at noon (stars off). The three star layers are most of it; a cheaper faint layer is the lever if needed.

Moved to [history](archive/sky-lighting-history.md#environment-map-refresh-lights1-2026-10-20).

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
  `Off` (MSAA), `Taa` or `Dlss(UpscaleQuality)` (falls back to `Taa`), quality `auto`, `native` (DLAA),
  `quality`, `balanced`, `performance`, `ultra-performance`; serde, so a
  settings file holds it), `ActiveUpscaling` (running), `UpscalingSupport`
  (SR / RR available), `TemporalHistory { reset }` (camera cuts, teleports,
  origin shifts, door travel). On raster, `RenderPathCamera`s get
  `TemporalAntiAliasing` or `Dlss<SuperResolution>` plus what it requires (jitter, mip bias, depth + motion-vector prepass)
  and `Msaa::Off`; a snapshot restores the camera when it is turned off at
  runtime. On the ray-traced path the quality goes to
  `Dlss<RayReconstruction>` (`Auto` when upscaling is off). ReSTIR is
  `SkyRenderOptions::ray_tracing_restir: Option<bool>`: `None` = on without
  a denoiser, off under Ray Reconstruction (Bevy's advice; same image,
  1.4 vs 3.5 ms of lighting in the Census Office at 1440p).
- **DLSS plugin package** (dlss-plugin-1/2, 2026-10-25). Why: both repos are
  GPL-3.0-or-later and the §7 permission (`LICENSE-EXCEPTION.md`) covers only
  our code, so the game binary holds no NVIDIA code. `dlss_wgpu` is replaced
  (`[patch.crates-io]` in both workspaces) by `vendor/dlss_wgpu`, a facade
  with the 6.0.0 API and no NVIDIA headers or libraries, so `dlss` builds
  need no SDK env. It loads `sf_dlss.dll` (`plugins/sf_dlss`, own workspace,
  links NGX; versioned C ABI in `vendor/dlss_wgpu/src/abi.rs`) at runtime from
  `$SF_DLSS_PLUGIN_DIR`, else `<exe dir>/dlss/`. Package layout: `dlss/` =
  `sf_dlss.dll`, `nvngx_dlss.dll` (SR), `nvngx_dlssd.dll` (RR),
  `NVIDIA-DLSS-LICENSE.txt`, `README.txt`; shipped separately, never in a
  game release. Build it with `bash plugins/sf_dlss/build-package.sh`
  (needs `../sdk`: `bash ../sdk/fetch.sh`, the ~140 MB non-redistributable
  subset; output `target/sf_dlss/package/dlss/`); Bevyrrowind's `build.rs`
  copies it to `target/<profile>/dlss/` for dev runs. Fallback: no package
  or ABI mismatch logs one info line, `UpscalingSupport` is all false and
  `upscale.rs` warns "DLSS plugin package missing" and runs TAA;
  `dlss_wgpu::plugin_available()` tells that apart from an unsupported GPU.
  Parity (2026-10-25): see Verification.
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
- DLSS plugin package (2026-10-25, dlss-plugin-2, release `solari,dlss`, driver 617.42, 1440p, 3
  interleaved runs): SR quality pass 2.24-2.30 ms (Balmora; 2.1 on 10-12/10-19), RR auto pass
  5.45-5.60 ms (Census Office `--rt`; 5.2 on 10-12); the facade passes the same NGX create/eval
  parameters, the extra FFI call is CPU-side (pass CPU 0.20 / 0.24 ms). Release exe: 0
  `NVSDK_NGX`/`nvsdk_ngx` strings. Screenshots `shots/dlss2/`. `--rt --off denoiser` in interiors
  quits on a `sky_motion` validation error (Bevyrrowind visual check DL-5; not the plugin).

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
4. ~~Night sky has no stars (M6), and the moon disk has no phase shape.~~ Done (S3, "Night sky and drawn bodies"); open: a star catalogue or a game's own star texture (adapter hook), lunar eclipses, moonlit-cloud silver lining from the moon pass.
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

Moved to [history](archive/sky-lighting-history.md#industry-comparison).
