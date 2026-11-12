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
- **Night sky.** Stars, textured moons and sun glare are drawn before the cloud composite (`night.rs`); the moon
  light is separate from the drawn disks. Raster or Solari path switch: `path.rs`; DLSS/TAA: `upscale.rs`.
- **Clouds and fog.** `CloudsPlugin` (`sf_atmos_bevy/src/lib.rs:229`): a half-res raymarch lit through the atmosphere
  LUTs, temporal resolve, depth-aware composite after Bevy's `render_sky` and before transparents
  (`render.rs:53`-`:67`). Cloud shadows: a 1024^2 toroidal map given to `CloudShadowLight` as a
  `DirectionalLightTexture` (module doc `lib.rs:1`). `Fog` (`:112`): analytic height fog and ground banks in the same
  node, plus a froxel volume near the camera (god rays, shadowed and point-lit fog; `render.rs:1069`) that reads the
  terrain's far shadow heights (`FogShadowHeights` `:129`). Fog fill adds the camera's ambient (`view_ambient` `:457`).
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

`--off clouds|fog|froxels|godrays|fogdither|fogjitter|foghistory|transfog|envslice|skymotion|surface`
(`mw_game_core/src/perf.rs:135`); live `flip ambient_specular`, console `surface set|dump|clear`, weather overrides
(`WeatherOverrides`). GPU spans `clouds_shadow|trace|resolve|composite`, Bevy `atmosphere_luts`; `MediumStats`
(`rebuilds`, `scale_updates`), `WeatherStats`.

## Known limits

- `atmosphere_luts` rose 0.36 -> 0.84-0.93 ms with P1 (per-sample term sums); P1b restores stock single fetches, expected
  back near 0.36 ms, not yet measured ([ledger](../../ledger/sky-weather.md)).
- Local storms seen from outside look like a wall with a shimmering border (WE-10); rain and clouds do not draw over
  distant water (WE-7); no visible god rays at sunrise in fog (FG-10); weather fog vanishes at night (FG-8, WE-9 causes 1 and 3).
- A weather medium change moves the sky through the GPU LUTs (no CPU bake); a model change still bakes on the render thread.

## Specs

[sky-lighting](../../../superfargoth/docs/specs/sky-lighting.md), [weather](../../../superfargoth/docs/specs/weather.md),
[clouds](../../../superfargoth/docs/specs/weather/clouds.md), `superfargoth/docs/specs/surface-weather.md`.
