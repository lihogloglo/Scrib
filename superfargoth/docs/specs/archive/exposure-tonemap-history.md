# Exposure and tonemapping: history

Moved out of [exposure-tonemap.md](../exposure-tonemap.md) on 2026-11-26 (specs-4, X0-X3 slices), word for word. Headings are unchanged. exposure-tonemap.md and the code win where they differ.

## Today (as built, 2026-11-17)

- **Exposure is a schedule**: EV100 from the sun's elevation (`SkySettings::exposure`, `sf_sky/src/sky.rs:76`; -18° ->
  0.5 ... 45° -> 14), clamped by `ExposureRange` (`sky.rs:102`), written to the camera's `Exposure`
  (`sf_sky_bevy/src/lib.rs:654`, `:800`). Interiors sit at `SCENE_EV100` 7.
- **Night darkness = two calibration values** (`LightCalibration`, `mw_assets/src/lights.rs:81`): the exposure
  floor `exposure_floor_ev100` 3.5 (the night always sits on it: the schedule wants 0.5) and the ambient floor
  `night_ambient_nits` 4. The moonlight (`FULL_MOONLIGHT_FLOOR_RATIO` 2.5 x pi x floor, `lights.rs:77`) and the
  night sky's glow (`NightGlow::ratio` over the floor, `sf_sky/src/night.rs:438`) both scale with the floor, so
  lowering it darkens ground, sky and moonlight together; lamps and stars don't change. Launch flags
  `--night-ambient`, `--exposure-floor` (`apps/bevyrrowind/src/main.rs:1272`); not tunables, not live.
  History: 1 nit (LT-1) rejected 2026-10-19 as "can't see anything", before N3 moonlight existed (2026-11-06).
- **No eye adaptation.** `AutoExposure` went with the portal spike (`c3729b1b`); left over:
  `ExposureRange::clamp_compensation` (`sky.rs:123`) and a screenshot log line (`mw_environment_bevy/src/render.rs:331`).
- **Tonemapper never set**: Bevy's default `TonyMcMapface` (gentle, desaturates brights toward white). Bloom
  `Bloom::NATURAL`, `render.bloom` / `render.bloom.intensity` tunables (0.1 in presets, off on Low).
- **15 shaders read `view.exposure`** (clouds, froxel fog, night sky, water, terrain, surface weather, particles,
  vendored atmosphere and deferred): exposure-relative maths (RO-23 "fully lit in exposed units", particle floor
  gain `mw_assets/src/particles.rs:806`). Whatever drives exposure must drive `view.exposure`.

## Slices E0-E1b as written (2026-11-17)

- **E0 night A/B (no code).** Owner check EX-1 by launch flags: `--night-ambient 2`, `--exposure-floor 4.5`, and
  `--night-ambient 2.5 --exposure-floor 4`, against today, `view "Balmora" --day 173 --hour 3.5 --weather clear`.
  The pick becomes the default in E1.
- **E1 night tunables.** `LightCalibration`'s floor and ambient, the moon ratio and the sky glow ratio as live tunables
  (flags as aliases); EX-1's pick as the default. Test: setting the tunable changes `SkySettings` the next frame
  (exact), moonlight follows the ratio (exact).
- **E1b night sky colour** (EX-1, owner 2026-11-17: floor 4.5 nice, sky still "super clear blue"). **Diagnosed
  2026-11-17** (owner, live flips): the moonlight paints the sky blue (its blue on the land is wanted), and the glow
  lifts the sky above the land; so both fixes below, not either. Was: if the moonlight is the blue (expected: the
  atmosphere scatters the artistic 31.4 lx moon like a dim sun, so it paints a day sky), the standard fix is to let the
  moon light the ground without lighting the sky as strongly: UE lets a directional light opt out of the sky
  atmosphere (`Atmosphere Sun Light`); ours = a vendored per-light atmosphere-scatter scale in `bevy_pbr` (one
  float per directional light, absent = 1 = stock), tunable `sky.moon.sky_scatter` (Guess 0.15), which also
  shrinks the white halo of NS-3c. If the airglow is the blue: `sky.night.glow_ratio` and its hue. Either way, a
  night desaturation (rod vision, Kirk and O'Brien SIGGRAPH 2011, "Perceptually based tone mapping for low-light
  conditions") goes in E2 as `render.grade.night_desaturation` (shadows-section saturation by sun elevation).
  **E1 as built (2026-11-17):** `exposure.min_ev100` (4.5), `sky.night.ambient_nits`, `sky.moon.light_floor_ratio`
  declared game-side in `mw_assets/src/lights.rs` (framework names, game defaults, until `sf_sky_bevy` declares them
  and the base mod sets the values), `sky.night.glow_ratio` in `sf_sky/src/night.rs`; a `PreUpdate` system rewrites
  `SkyConfig` and `LightCalibration` on change; `--exposure-floor`/`--night-ambient` set the CommandLine layer.
  Exterior only (the viewers never applied the night calibration). **E1b as built:** vendored
  `DirectionalLightAtmosphereScatter(f32)` (`bevy_pbr` `render/light.rs`, WGSL `DirectionalLight.atmosphere_scatter`
  in the tail padding, 400 B unchanged), scaling single + multiple in-scattering in `sample_local_inscattering` (sky-view,
  aerial-view, raymarched sky, so the env map too) and the atmosphere's virtual-ground term; not transmittance, disks,
  surfaces or fog. `sky.moon.sky_scatter` (`sf_sky_bevy` `MOON_SKY_SCATTER`, 0.15 Guess) bound on `MoonLight`.
  Expect the night env-map ambient on the land to drop with the sky.
