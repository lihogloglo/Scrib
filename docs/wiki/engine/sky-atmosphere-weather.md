# Sky, atmosphere and weather

## What it does

One game clock drives a physically based sky (Bevy `Atmosphere`, sun, moon, stars, exposure, ambient floor). A
weather state (a pure function of records, seed, overrides, clock and the camera's climate) blends haze, dust, cloud
cover, fog, precipitation and wind, so changes glide. Volumetric clouds, fog and froxel god rays are our own passes.

## How it works

- **Sky plugin.** `SkyPlugin` (`superfargoth/crates/sf_sky_bevy/src/lib.rs:290`): `advance_clock` on `FixedUpdate`
  (`:515`), then `SkySystems::{Clock, RenderPath, Apply}` in `Update` (`:315`-`:331`). `evaluate_sky` fills `CurrentSky`;
  `apply_lights` writes sun and moon (`:689`), `apply_cameras` writes `Exposure`, `AmbientLight` and the airglow
  (`:712`). Cameras with `SkyCamera` get Bevy `AtmosphereSettings` and `AtmosphereEnvironmentMapLight` (`:602`); removed
  indoors. `place_atmosphere` keeps the planet under sea level after floating-origin shifts (`:588`).
- **Medium and weather scales.** `build_medium` (`:399`) bakes the density and phase LUTs (256^2,
  `SkyRenderOptions::medium_resolution` `:132`) only when the atmosphere model changes. Haze and dust are live
  `ScatteringMediumScales` (`update_medium` `:534`, steps 0.05/0.02/0.02, at most every 0.25 s,
  `MediumStats`). On a scale change the cached static pass recomputes once on the GPU: the combined medium LUTs (P1b
  `medium_lut.wesl`: unit falloff and phase x live coefficients, stock layout), then transmittance and multiscattering;
  every other pass samples the combined LUTs with stock single fetches.
- **Vendored `bevy_pbr` atmosphere patches** (`superfargoth/vendor/README.md`): `AtmosphereStaticLutCache` (`:23`),
  P1 live medium scales (`:24`, `vendor/bevy_pbr/src/medium.rs`), N2 airglow (`:29`: night sky glow in the sky-view LUT, so
  sky, env map, fog and clouds all see it), N1 ambient-specular toggle (`:27`), surface weather hook (`:26`).
- **Env map.** Bevy refilters the sky env map every frame (~0.7 ms a run); `EnvMapRefresh` keeps the result for N = 8
  frames (`sf_sky_bevy/src/env_refresh.rs:24`), so it costs ~0.09 ms a frame. Bevy timings appear only on frames a pass
  runs: the perf report shows `GPU ms/run` and `runs/frame` and sums the per-frame share (`sf_diag` `PassTiming`).
  A run still lands on one frame (a ~0.7 ms spike every 8th); spreading it over frames (UE-style) is not done.
- **Night sky.** Stars, textured moons and sun glare are drawn after the water and before the cloud composite (`night.rs`); the moon
  light is separate from the drawn disks. Raster or Solari path switch: `path.rs`; DLSS/TAA: `upscale.rs`.
- **Clouds and fog.** `CloudsPlugin` (`sf_atmos_bevy/src/lib.rs:229`): a half-res raymarch lit through the atmosphere
  LUTs, temporal resolve, depth-aware composite after the water (Bevy's transmissive pass) and `render_sky`, before
  transparents; the trace and composite read a depth that holds the water, so weather covers sea like land (W-ORDER,
  [render-order](../../../superfargoth/docs/specs/render-order.md) "Frame graph")
  (`render.rs:53`-`:67`). Cloud shadows: a 1024^2 toroidal map given to `CloudShadowLight` as a
  `DirectionalLightTexture` (module doc `lib.rs:1`). In a storm (map storm scalar > 0) the density samples the shape noise
  in world altitude, tops follow coverage (domed cells), an anvil shelf spreads above h 0.72, the base erodes ragged and
  cells shrink by threshold at the region edge; calm decks run the old functions (S3, `CumulonimbusShape`,
  `sf_atmos::combine_density_shaped`; `--off cbshape` = old profile). `Fog` (`:112`): analytic height fog and ground banks in the same
  node, plus a froxel volume near the camera (god rays, shadowed and point-lit fog; `render.rs:1069`) that reads the
  terrain's far shadow heights (`FogShadowHeights` `:129`). Fog fill adds the camera's ambient (`view_ambient` `:457`).
  Ground-fog banks: the half-res trace (`shaders/cloud_trace.wesl`, 6x6-window depth) records the bank ratio at the
  window's nearest and furthest depth as packed (distance, bank) pairs in `dist` G/A; the resolve keeps the far pair's
  history by open/cut population, the near pair's by near distance; the composite blends them by each pixel's
  log-depth with bilateral depth weights (FOG-BANK, `sf_atmos::bank_edges`, `FogBankEdges`; spec `weather/fog.md`).
  The clouds sit inside the fog column: the composite evaluates the fog again to the clouds' tap-weighted depth and
  puts only that part in front of them, the rest behind (`rgb = L_f + T_f·C + T_c·(L − L_f)`, UE5/Frostbite; WE12-FIX,
  `FogCloudSplit`, `sf_atmos::compose_fog_split`), and the trace orders the far weather by distance.
  Under the checkerboard the trace takes that window over the whole 2x2 block (plus the composite's 1-px bilinear reach)
  and classes it along the block's centre ray, so silhouette blocks keep one open/cut class across phases; a texel whose
  history held only the other population shows its same-population 3x3 mean, and the stored cloud distance accumulates
  like the colour (S1, `CloudEdgeStable`, `sf_atmos::temporal::depth_window`; `--off cloudedge` = old window, raw sample).
  Far rain (CW-1 + S2 fixes, `far_weather_shaped`, CPU twin `sf_atmos::storm`): map precipitation × the clouds' rain
  cores (from just under the coverage threshold) × a profile reaching up to the visible base, its region edge re-blended over
  `StormShape::edge_m` (64 m, ±48 m noise) from 4 map taps at ±band, widened to the sample spacing along the ray
  (`edge_half_width`) and back to the band within 128-256 m of the camera (the near FX hand-off),
  integrated in closed form over 16 fixed segments (no jitter: a smooth function of the ray).
  Froxel history (`shaders/froxel_inject.wesl`): per-froxel R3 jitter, history reprojected with last frame's
  clip-from-world and read at the froxel centre (off screen or past `far`: this frame alone), then clipped to the
  min/max box of this frame's 3x3 samples in the slice (Karis 2014 TAA clip; 16x16 workgroups write 14x14 tiles,
  the apron is the box) at weight 0.95 (`FroxelSettings::clamp`, `HISTORY_CLAMPED`; FOG-CLAMP, `--off fogclamp` =
  unclipped 0.9).
  Translucents use `FroxelFogMaterial` (`transparent.rs`, `fog_translucents.rs`).
- **Weather.** `WeatherPlugin` (`superfargoth/crates/sf_weather_bevy/src/lib.rs:184`): `evaluate` into
  `CurrentWeather` (`:223`), then `apply_sky` (`:318`), `apply_clouds` (`:354`), `apply_fog` (`:379`), `apply_wind` (`:427`),
  precipitation, lightning flash and sound cues (`fx.rs`). Distant weather: a world weather map mirrored to
  `CloudWeatherMap` (`:291`, `sf_atmos_bevy/src/weather_map.rs`). Surface weather (wet, puddles, snow, ash) steps in
  `surface.rs` and shades through the vendored SW patch.
- **Exposure and night.** Exposure range floor EV100 3.5 and the night ambient floor come from the game
  (`bevyrrowind/crates/mw_assets/src/lights.rs:93`); see [lights](lights.md).

## Bevy features used

`Atmosphere`, `ScatteringMedium`, `AtmosphereSettings`, `AtmosphereEnvironmentMapLight`, `SunDisk`,
`DirectionalLightTexture`, `Exposure`, `GeneratedEnvironmentMapLight`, STBN blue noise (vendored file) for cloud jitter.

## Switches and counters

`--off clouds|fog|froxels|godrays|fogdither|fogjitter|foghistory|fogbank|fogclamp|cloudfog|stormshape|cloudedge|cbshape|transfog|envslice|skymotion|surface`
(`mw_game_core/src/perf.rs:135`); live `flip ambient_specular`, console `surface set|dump|clear`, weather overrides
(`WeatherOverrides`). GPU spans `clouds_shadow|trace|resolve|composite`, Bevy `atmosphere_luts`; `MediumStats`
(`rebuilds`, `scale_updates`), `WeatherStats`.

## Known limits

- `atmosphere_luts` rose 0.36 -> 0.84-0.93 ms with P1 (per-sample term sums); P1b restores stock single fetches, expected
  back near 0.36 ms, not yet measured ([ledger](../../ledger/sky-weather.md)).
- Local storms seen from outside look like a wall with a shimmering border (WE-10); rain and clouds not drawing over
  distant water (WE-7, cause: water drawn after the weather) is fixed by W-ORDER, awaiting owner (WE-11); no visible god rays at sunrise in fog (FG-10); weather fog vanishes at night (FG-8, WE-9 causes 1 and 3).
- A weather medium change moves the sky through the GPU LUTs (no CPU bake); a model change still bakes on the render thread.

## Specs

[sky-lighting](../../../superfargoth/docs/specs/sky-lighting.md), [weather](../../../superfargoth/docs/specs/weather.md),
[clouds](../../../superfargoth/docs/specs/weather/clouds.md), `superfargoth/docs/specs/surface-weather.md`.
