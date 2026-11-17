# Exposure and tonemapping (eye adaptation, night darkness, image look)

Status: spec (2026-11-17); E1 and E1b built 2026-11-17 (awaiting owner, EX-1). ROADMAP 1.26. Visual checks `EX-`. Wiki: [lights](../../../docs/wiki/engine/lights.md)
"Night brightness and ambient"; ledger [sky-weather](../../../docs/ledger/sky-weather.md).

<!-- agent-summary -->
> **Agent summary (2026-11-17).** Three owner asks in one system: nights darker (owner 2026-11-17: "day for night"),
> eye adaptation, and a look where sunlit patches nearly burn out (Crimson Desert reference). Exposure becomes
> **meter -> adapt -> clamp -> camera `Exposure`** (pre-exposure, the UE/Frostbite pattern): a GPU luminance histogram
> (Bevy's `AutoExposure` shader, metering only), read back, adapted on the CPU, written as the camera's EV100 so every
> shader that reads `view.exposure` stays right. The sun schedule stays as a mode for benchmarks and screenshots. Every value
> is a tunable (console, `--set`, BRP), and **every one has a row on a new options page, Image** (owner 2026-11-17).
> Read next: "Design" to build, "Tunables" for names and defaults, "Slices" for order.
<!-- /agent-summary -->

## Goal

Owner, 2026-11-17: (1) the night reads too bright, so Secunda barely shows; (2) eye adaptation; (3) the look of
sunlit patches that are almost blown out, as in Crimson Desert. All of this system's values must be easy to tweak,
in the options menu and the console.

Non-goals: local exposure (UE 5.1's bilateral-grid tonemapping: not in Bevy 0.20, a later wish), ray-traced GI (the
Ultra tier's job; the raster look must stand on its own), per-weather grading (weather keeps its own colours).

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

## Reuse check (CONTRIBUTING rule 0/2)

- **Bevy 0.20 `AutoExposure`** (`bevy_post_process-0.20.0/src/auto_exposure/`): 64-bin histogram of the HDR target,
  percentile filter (default 10-90 %), `speed_brighten` 3 / `speed_darken` 1 stops/s with an exponential section
  (1.5 stops), metering mask (red channel, 16 levels), compensation curve (log luminance -> stops, 256-entry LUT).
  The right metering, but as shipped it doesn't fit: (a) its correction is applied only in the tonemapping pass, so
  the 15 shaders above see the un-adapted `view.exposure` (torches, particles and clouds would drift by the adapted
  amount); (b) no output clamp (min/max EV: our workaround is the curve, `clamp_compensation`); (c) the value lives in a
  render-world buffer, so the CPU (logs, F3 panel, perf determinism, net presentation) can't see it. **Use:** its
  histogram shader, metering only, read back (slice E3, a small vendored patch: rule 4).
- **Bevy `Tonemapping`** (`bevy_render-0.20.0/src/view/mod.rs`): None, Linear, Reinhard, ReinhardLuminance,
  AcesFitted, AgX, SomewhatBoringDisplayTransform, TonyMcMapface (default), BlenderFilmic, KhronosPbrNeutral;
  `tonemapping_luts` is on in our features. **Bevy `ColorGrading`**: global (exposure, temperature, tint, hue,
  post-saturation, midtones range) and per section (shadows, midtones, highlights: saturation, contrast, gamma,
  gain, lift). **`Bloom`**: intensity, prefilter threshold, composite mode. All used as-is, no new code.
- **Crates:** `bevy_mod_auto_exposure` 0.2 targets Bevy 0.13 (March 2024), no newer release: stale. Nothing else
  for 0.20 (search 2026-11-17).
- **Ours (thin):** the CPU adaptation (a few dozen lines of maths, Bevy-free in `sf_sky`), the readback glue, the
  options page records.

## Industry comparison

| Engine | Exposure | Ours |
|---|---|---|
| UE 5 | Histogram auto exposure (64 bins, low/high percent 10/90), Speed Up 3 / Speed Down 1 stops/s, Min/Max EV100, exposure compensation (setting + curve by average scene EV100), pre-exposure (rendering uses last frame's adapted exposure), metering mask, local exposure 5.1+, reset on camera cut ([UE auto exposure][ue-ae]) | **match** after E4 (same model and defaults); local exposure gap (Bevy) |
| Unity HDRP | Automatic / automatic histogram / curve mapping / fixed / physical camera, limits min/max, adaptation speeds ([Unity][unity-ae]) | match |
| Frostbite | Pre-exposure introduced with the move to physical units (Lagarde and de Rousiers, "Moving Frostbite to PBR", 2014): lights stay physical, the frame is rendered pre-exposed | match (the reason for "write the camera `Exposure`") |
| Crimson Desert | No developer talk; the burned-sunspot look reads as ray-traced GI keeping shade dark + exposure set for the shade + a tonemapper that lets highlights clip ([industry page](../../../docs/wiki/industry/crimson-desert.md)) | raster gets the exposure and tonemapper parts; dark shade comes from probe GI/AO (Solari on Ultra) |
| Bethesda / OpenMW | LDR lighting, no physical exposure; eye adaptation as a post effect (MGE XE) | the Classic preset may keep `schedule` |

[ue-ae]: https://dev.epicgames.com/documentation/en-us/unreal-engine/auto-exposure-in-unreal-engine
[unity-ae]: https://docs.unity3d.com/Packages/com.unity.postprocessing@3.3/manual/Auto-Exposure.html

Decided from the standard (no owner question): histogram metering, pre-exposure, separate up/down speeds, reset on cut,
min/max EV clamp, benchmarks ignore adaptation.

## Design

### Exposure modes (`exposure.mode`)

- `schedule`: today's sun curve (kept: deterministic). Forced for `perf suite`, `--screenshot` and baselines, the way
  benchmark runs already ignore user overrides (tunables spec).
- `auto`: eye adaptation (below). Interactive default once EX-3 is signed off; until then `schedule`.
- `fixed`: `exposure.fixed_ev100`, for checks and photo-style debugging.

### Auto: meter -> adapt -> clamp -> camera `Exposure`

1. **Meter (GPU).** Bevy's histogram pass over the pre-exposed HDR target, mask = `exposure.meter.center_weight`
   (0 uniform, 1 strong centre weighting; a generated mask image), percentiles `exposure.meter.low_pct` /
   `high_pct`. Output: the average log2 luminance of the kept bins. Vendored patch to `bevy_post_process` (smallest
   form, E3): a `meter_only` switch that skips the tonemapping correction, and the result copied into a buffer
   that `bevy_render::gpu_readback::Readback` maps. Scene EV = metered + the EV the frame was rendered at (known on
   the CPU, so the 1-3 frame readback latency is exact, not estimated).
2. **Target (CPU, Bevy-free, `sf_sky::exposure`).** `target = scene_ev + compensation(scene_ev) + exposure.bias_ev`.
   Compensation is a curve over scene EV with five fixed anchors (`exposure.comp.ev_m2`, `_2`, `_6`, `_10`, `_14`:
   stops at scene EV -2, 2, 6, 10, 14, linear between, flat outside). This is how dark stays dark: a negative
   compensation at low scene EV keeps a night from metering up to grey (UE's curve; Bevy's grey-night problem,
   sky-lighting.md "Exposure"). Default anchors reproduce today's schedule at its own key points (calibrated by a
   test against `SkySettings::exposure`), so switching modes doesn't change the look, only adds adaptation.
3. **Adapt.** Toward the target at `exposure.speed_up` / `exposure.speed_down` stops/s (camera EV rising = adapting to
   brighter; up 3, down 1, UE/Bevy defaults), exponential within `exposure.exp_distance` stops. Frame-time based on
   `Time<Real>` (pausing the game doesn't freeze the eye; the pause menu shows the world).
4. **Clamp.** `exposure.min_ev100` (today's floor, 3.5, the game's base mod value) to `exposure.max_ev100`.
   The floor stays the lights' guard (lamps read at most 2^(7 - floor) their calibration).
5. **Apply.** Written to the camera's `Exposure.ev100` (and the sky camera's, as now); every shader stays right.
   Cuts reset to the target with no adaptation (cell load, teleport, fast travel, save load, `exposure.mode` change;
   `exposure.reset_on_cut`).

Interiors meter like exteriors in `auto` (a dark cave opens down to the floor; a lit tavern settles near 7). Net:
everything is `Presentation` (each peer its own eye).

**Temporal history.** Exposure changing frame to frame shifts the pre-exposed history of TAA and DLSS. Standard fix:
scale history by the exposure ratio (UE), and give DLSS the pre-exposure value (NGX `InPreExposure`: the plugin
already passes one, `sf_dlss/src/lib.rs:596`; check what the app feeds it, and the `AutoExposure` creation flag at
`:176`). E5 diagnoses it first (does a fast change smear or flash?) before fixing.

### Night darkness

The two calibration values become tunables, live (the ambient curve and the floor are re-applied to `SkySettings`
on change): `sky.night.ambient_nits` (4) and `exposure.min_ev100` (3.5); moon and sky glow keep following the
ambient. Launch flags stay as aliases. The new default is the owner's pick from EX-1 (`Source::Owner(date)`).

### Look: tonemapper, grade, bloom

`render.tonemap` (choice of Bevy's ten, default `tony_mcmapface` until EX-2), a short grade on Bevy `ColorGrading`
(global post-saturation, temperature; per section: highlights gain and contrast, shadows lift and contrast), and the
existing bloom tunables plus `render.bloom.threshold`. All live: component swaps and uniforms; a tonemapper switch
compiles its pipeline on first use (logged; pipeline warm-up covers the default only).

### Options page "Image" (owner 2026-11-17: every value has a row)

A new `ui.setting` page `image` (`scope = "Machine"`, like Graphics), one row per tunable below, bound by key exactly
as `graphics_page` binds the preset drivers (`mw_exterior_bevy/src/graphics_page.rs`); floats are sliders
(`SettingKind::Float`, `log_scale` off), choices are choice rows. Two sections: **Image** (Brightness = bias,
Eye adaptation = mode, Adaptation speed, Tone mapping, Bloom, Night darkness) and **Advanced** (all the rest).
A console `set` overrides for the run and is never saved (as Graphics). This answers tunables open question 1 for
this system only: these tunables are `Scope::Player`.

## Tunables

All `Net::Presentation`, `Lifetime::Live`, group `Post` unless noted. Framework names; the game's base mod sets
the Morrowind values (marked *game*).

| Name | Default | Range, unit | Source | Row |
|---|---|---|---|---|
| `exposure.mode` | `schedule` | schedule / auto / fixed | Owner 2026-11-17 (auto after EX-3) | Eye adaptation |
| `exposure.bias_ev` | 0 | -3..3 stops | Industry (UE Exposure Compensation; game "Brightness") | Brightness |
| `exposure.speed_up` | 3 | 0.1..20 stops/s | BevyDefault `speed_brighten`; UE Speed Up | Adaptation speed (scales both) |
| `exposure.speed_down` | 1 | 0.1..20 stops/s | BevyDefault `speed_darken`; UE Speed Down | Advanced |
| `exposure.exp_distance` | 1.5 | 0..5 stops | BevyDefault `exponential_transition_distance` | Advanced |
| `exposure.min_ev100` | -10 (*game* 3.5) | -10..16 EV100 | Owner 2026-10-13 option A | Night darkness (with ambient) |
| `exposure.max_ev100` | 20 (*game* 16) | -10..20 EV100 | Guess (above noon 14) | Advanced |
| `exposure.fixed_ev100` | 7 | -10..20 EV100 | Reference (`SCENE_EV100`) | Advanced |
| `exposure.meter.low_pct`, `high_pct` | 10, 90 | 0..100 % | BevyDefault `filter`; UE Low/High Percent | Advanced |
| `exposure.meter.center_weight` | 0.5 | 0..1 ratio | Guess (UE centre-weighted metering) | Advanced |
| `exposure.comp.ev_m2` .. `ev_14` (5) | from the schedule (E4 test) | -6..6 stops | Measured (fit to `SkySettings::exposure`) | Advanced |
| `exposure.reset_on_cut` | on | bool | Industry (UE camera-cut reset) | Advanced |
| `sky.night.ambient_nits` | 0.03 (*game* 4, then EX-1) | 0..20 nits | Owner 2026-10-19 (LT-1) | Night darkness |
| `sky.moon.light_floor_ratio` | 2.5 | 0..10 ratio | Reference (`FULL_MOONLIGHT_FLOOR_RATIO`, artistic) | Advanced |
| `sky.night.glow_ratio` | `NIGHT_SKY_GLOW_RATIO` | 0..5 ratio | as declared today | Advanced |
| `render.tonemap` | `tony_mcmapface` | Bevy's ten | BevyDefault `Tonemapping` | Tone mapping |
| `render.grade.saturation` | 1 | 0..2 ratio | BevyDefault `ColorGradingGlobal::post_saturation` | Advanced |
| `render.grade.temperature` | 0 | -1..1 | BevyDefault | Advanced |
| `render.grade.highlights_gain`, `_contrast` | 1, 1 | 0..2 ratio | BevyDefault `ColorGradingSection` | Advanced |
| `render.grade.shadows_lift`, `_contrast` | 0, 1 | -0.5..0.5, 0..2 | BevyDefault | Advanced |
| `render.bloom`, `.intensity` (exist) | per preset | | as declared | Bloom |
| `render.bloom.threshold` | 0 | 0..10 (scene units) | BevyDefault `BloomPrefilter::threshold` | Advanced |

"Night darkness" is one player slider driving `sky.night.ambient_nits` (log scale over 0.5..8); the floor has its own
Advanced row.

## Slices

One deliverable plus its test each; each behind its switch; values logged in the ledger line whatever the result.

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
- **E2 look tunables + Image page.** `render.tonemap`, grade, bloom threshold, the `image` page with every row of
  the table (rows for E4 values hidden until E4). Test: each row's key is a declared tunable with its range;
  picking a row sets the `User` layer (as `graphics_page` tests). Owner check EX-2: noon street and forest edge,
  tonemappers cycled from the console.
- **E3 metering.** Vendored `bevy_post_process` meter-only patch + readback -> `ExposureMeter { scene_ev, frame }`,
  shown in the F3 panel; behaviour unchanged. Test: synthetic scene of a uniform emissive plane at a known
  luminance meters within one histogram bin (exact range). GPU timer on the pass.
- **E4 adaptation.** `exposure.mode auto`: target, curve, speeds, clamp, cut reset (Bevy-free maths in `sf_sky`, exact
  tests: step response times, clamp, the default curve reproducing the schedule within 0.25 EV at its keys). Owner
  check EX-3: Balmora noon from a shaded alley into the square; night from a torch-lit street into the dark; a cave
  mouth by day; a cell load (no adaptation ramp after a cut).
- **E5 temporal history.** Diagnose TAA/DLSS under a fast exposure change; fix as the standard says (history rescale,
  DLSS pre-exposure). Owner check in the EX-3 scene.
- **E6 cost.** One untraced interleaved A/B, `schedule` vs `auto` (expected < 0.1 ms GPU, ~0 main), ledger perf line.
  Then `auto` becomes the interactive default.

## Risks

- **Grey nights** if the curve is wrong: the E4 test pins the default curve to the schedule; the floor guards lamps.
- **Readback latency** of 1-3 frames: harmless at 1-3 stops/s, and the EV each metered frame was rendered at is
  known exactly.
- **Exposure-relative shaders** (particles, self-lit torches, clouds, fog): correct by construction under
  pre-exposure; check EX-3 that torches and smoke don't pump as the eye adapts.
- **Tonemapper change shifts every look check** signed off so far: EX-2 picks once; no per-scene tonemapping.

## Open questions (owner)

Taste only, answered by the checks: the night darkness level (EX-1), the tonemapper (EX-2), adaptation on by
default and its speed (EX-3). Everything else above is decided from the industry standard.
