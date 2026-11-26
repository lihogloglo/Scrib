# Exposure and tonemapping (eye adaptation, night darkness, image look)

Status: spec (2026-11-17; X0-X3 added 2026-11-26); E1 and E1b built 2026-11-17 (awaiting owner, EX-1). ROADMAP 1.26. Visual checks `EX-`. Wiki: [lights](../../../docs/wiki/engine/lights.md)
"Night brightness and ambient"; ledger [sky-weather](../../../docs/ledger/sky-weather.md).

<!-- agent-summary -->
> **Agent summary (2026-11-17).** Three owner asks in one system: nights darker (owner 2026-11-17: "day for night"),
> eye adaptation, and a look where sunlit patches nearly burn out (Crimson Desert reference). Exposure becomes
> **meter -> adapt -> clamp -> camera `Exposure`** (pre-exposure, the UE/Frostbite pattern): a GPU luminance histogram
> (Bevy's `AutoExposure` shader, metering only), read back, adapted on the CPU, written as the camera's EV100 so every
> shader that reads `view.exposure` stays right. The sun schedule stays as a mode for benchmarks and screenshots. Every value
> is a tunable (console, `--set`, BRP), and **every one has a row on a new options page, Image** (owner 2026-11-17).
> **2026-11-26:** overcast read darker than night (EV ignores weather, twilight valley): X0-X3 add an analytic weather
> EV bias, a twilight hold of the night floor and a cloud ambient floor, before eye adaptation (E3-E5, deferred).
> Read next: "Design" to build, "Tunables" for names and defaults, "Slices" for order.
<!-- /agent-summary -->

## Goal

Owner, 2026-11-17: (1) the night reads too bright, so Secunda barely shows; (2) eye adaptation; (3) the look of
sunlit patches that are almost blown out, as in Crimson Desert. All of this system's values must be easy to tweak,
in the options menu and the console.

Non-goals: local exposure (UE 5.1's bilateral-grid tonemapping: not in Bevy 0.20, a later wish), ray-traced GI (the
Ultra tier's job; the raster look must stand on its own), per-weather grading (weather keeps its own colours).

## Today (as built; the 2026-11-17 text is in [history](archive/exposure-tonemap-history.md))

- **Exposure is a schedule of the sun's elevation only** (keys `sf_sky/src/sky.rs:77-86`, -18° 0.5 to 45° 14), applied
  at `sky.rs:268` through `ExposureRange::clamp` (`:115`), written to the sky cameras' `Exposure`
  (`sf_sky_bevy/src/lib.rs:700`, `:846`). Weather never enters it. Interiors sit at `SCENE_EV100` 7. No eye adaptation (`AutoExposure` went with the portal spike, `c3729b1b`; left over:
  `ExposureRange::clamp_compensation`, `sky.rs:123`).
- **Weather dims direct light only**: `Weather::direct_light_factor` (`sf_sky/src/weather.rs:108`) = 1 - cover x
  `overcast_dimming` (0.8 for every preset: no record sets it, `sf_weather/src/records.rs:204`), then the cloud-shadow
  cookie `mix(1, exp(-od), shadow_strength 0.9)` (`sf_atmos_bevy/src/shaders/cloud_shadow.wesl:60`; its mean
  `1 - mean_cover x 0.9 x 0.6`, `:62`). The sky fill is the atmosphere env map from the clear-sky LUT (clouds are not in
  it); its CPU model `SkySettings::sky_fill` (`sky.rs:282`, single scattering, no clouds, errs low) feeds the sun gate.
- **The night is hand-calibrated** (`LightCalibration`, `mw_assets/src/lights.rs`): exposure floor `exposure.min_ev100`
  4.5 (`:85`), ambient floor `sky.night.ambient_nits` 4 (`:94`) fading from -12° to 0 at -4° (`apply_sky`, `:149`),
  moonlight `sky.moon.light_floor_ratio` 2.5 x pi x floor = 31.4 lx (`:155`). The day has no floor (framework curve
  `sky.rs:88`).
- **The weather's ambient tint carries no brightness** (`weather.rs:36-40`, luminance 1): `WEATHER_AMBIENT_BRIGHTNESS`
  = 0 (`mw_weather/src/ambient.rs:44`, a bare const) drops Morrowind.ini's per-weather ratio, and the tint reaches only
  the night floor.
- **Tonemapper** Bevy's default `TonyMcMapface`; bloom `Bloom::NATURAL` (`render.bloom`, `.intensity`). **15 shaders read
  `view.exposure`** (clouds, froxel fog, night sky, water, terrain, surface weather, particles, vendored atmosphere and
  deferred; RO-23, particle floor gain): whatever drives exposure must drive `view.exposure`.

**Why overcast reads darker than night** (research `exposure-overcast-2026-11-26.md` §2.2-2.4, calculated; X0
measures). Mid-grey ground, scene value = nits / (1.2 x 2^EV100): clear 30° 0.41; overcast 30° 0.11 (-1.9 stops); storm
0.05; moonlit night 0.14, moonless 0.044. (1) EV ignores the weather while clouds dim the light 2-3 stops. (2) Only the
night has floors (EV, ambient). (3) The ini's weather ambient brightness is dropped and the fill under cloud not lifted.
(4) **Twilight valley**: EV reaches 7 at -4° (`sky.rs:80-81`) as the ambient floor reaches 0 (`lights.rs:149`): scene
~0.001-0.005, 10-30x under deep night, the darkest frame of the day.

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
- **X1-X3 (2026-11-26):** Bevy 0.20 has no weather- or budget-driven exposure (only the metered `AutoExposure`, E3) and
  no local exposure; no crate does either. Ours: a few lines of Bevy-free maths in `sf_sky` (research §3).
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

**Weather and twilight** (research 2026-11-26 §4; (U) = no verifiable public numbers):

| Who | Technique | Ours |
|---|---|---|
| UE 5 | Weather and time of day dim the light physically; the frame stays readable through the Exposure Compensation Curve (by scene EV100) and Min/Max EV100 per post-process volume ([UE auto exposure][ue-ae]) | **match** in effect: X2 is that curve over the modelled light budget (E4 the metered one); per-volume clamps gap (one global range) |
| UE 5.1+ | Local exposure (highlight/shadow contrast, blur radius) compresses overcast and backlit frames without moving global EV | gap: not in Bevy 0.20 (non-goal; judged after X2) |
| Unity HDRP | Exposure "Curve Mapping": EV = f(scene EV), limits, compensation | match (X2) |
| Frostbite | Photometric lights, EV100, auto exposure with artist compensation; exposure moves without relighting | match |
| RDR2 (Bauer, Advances 2019) | Per weather / time-of-day artistic exposure and a lifted shadow floor so storms stay readable (U) | match in kind: k per weather (X2), cloud floor (X3) |
| Horizon / Decima (Schneider 2015/2017) | Cloud ambient that does not collapse under a dense deck (ambient in-scatter); exposure not public (U) | match after X3 |
| Ghost of Tsushima, Cyberpunk 2077 | Filmic toe with controlled shadows; per-weather exposure curves (U) | toe = E2's tonemapper pick |
| Real light | Sunny 100-130 klx, heavy overcast 1-10 klx, civil twilight 3-10 lx, full moon 0.1-0.3 lx: ~20 stops, compressed to ~14 by eye and display | overcast kept 1.5 stops under clear (owner), physically 2-4 |

No shipped game keeps one daytime EV across weather. Dusk darker than the night is a coupling bug of two curves, not
a technique (research §2.3): X1.

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

### Weather and twilight compensation (X1-X3)

Three analytic terms on the schedule, Bevy-free maths in `sf_sky` (0 ms GPU): the analytic form of UE's compensation
curve / HDRP Curve Mapping over a modelled light budget. `schedule` mode includes them (still deterministic: benchmarks
and screenshots keep them). In `auto` (E4) the meter sees the weather, so the weather bias is off there (no double
count); the twilight hold and the cloud ambient are lighting and stay.

**Light budget** (`sf_sky::LightBudget`, built by `SkySettings::light_budget(e, &Weather, shadow_mean)`), lux on level
ground at sun elevation e:
- S = `sun_illuminance` x luminance(transmittance(e)) x max(sin e, 0): direct sun under a clear sky.
- F = `sky_fill(sun_illuminance, e, weather)`: the fill, undimmed by cloud (the env map has no clouds; if they enter it,
  F takes the renderer's factor).
- A_n = night floor after X1, A_c = cloud ambient (X3), nits; a Lambert face under uniform radiance L gets pi x L lux.
- B_clear = S + F + pi A_n; B_w = S x d x c + F + pi (A_n + A_c), with d = `Weather::direct_light_factor()` and c the
  cloud-shadow mean: `sf_sky_bevy::CloudShadowMean(f32)` (1 when absent), written by `sf_atmos_bevy` with the cookie's own
  expression `1 - mean_cover x shadow_strength x 0.6` (`cloud_shadow.wesl:62`), so `sf_sky` stays free of `sf_atmos`.

**Weather bias (X2).** `weather_ev_bias(&ExposureWeather, &Weather, &LightBudget) -> f32`, pure:
- deficit D = log2(B_clear / B_w), 0 when B_w >= B_clear or B_clear <= 0;
- bias = min(k_w x D, `max_boost_ev`), k_w = clamp(`exposure.weather.k` x `Weather::exposure_k_scale`, 0, 1);
- camera EV100 = `exposure_range.clamp(schedule(e) - bias_eased)` (lower EV = the eye opens).

`exposure_k_scale` (ratio, default 1) is a preset value (`SkyTerms::exposure_k_scale`) blended linearly like the other
`Weather` terms. Morrowind: Thunderstorm, Ashstorm, Blight and Blizzard take `exposure.weather.storm_scale` (0.6: storms
stay darker, owner 2026-11-26), every other preset 1 (`mw_weather`, the storm presets of `lib.rs:418-423`). Default k: X0
measures D for Overcast at sun 30° and sets k = 1 - 1.5 / D, so Overcast ends ~1.5 stops under Clear (owner 2026-11-26);
Guess 0.5 until then (calculated D 1.7-3). If D <= 1.5, k = 0 and the darkness is the fill and the toe (X3, E2), not
the EV: X0's report says which. Max boost 2.5 stops, inside `exposure.max_ev100`. Below the horizon S = F = A_c = 0, so
D = 0 at night with no extra ramp; near the horizon F dominates and D fades smoothly. Eased in `sf_sky_bevy`: the applied
bias moves toward the target at <= `exposure.weather.ease_speed` stops/s (`Time<Real>`), snaps on a cut (the cut list of
E4). The bias follows the mean cover, not the cloud patch over the camera (WE-17's passing-shadow drop is a meter or
local-exposure matter). Lamps and emissives read 2^bias brighter (lamp rule 2^(7 - EV)): accepted (owner 2026-11-26,
lit towns).

**Twilight hold (X1).** The night floor keeps its on-screen value until the sky takes over: `SkySettings::twilight:
TwilightHold { hold_deg, end_deg }`; `evaluate` returns A_n(e) = `ambient`(e) x 2^(EV_s(e) - `min_ev100`) x w(e), EV_s the
clamped schedule, w = 1 below `exposure.twilight.hold_deg` (-2°), linear to 0 at `exposure.twilight.end_deg` (2°). The
game's floor curve becomes flat at `sky.night.ambient_nits` (`apply_sky`, `lights.rs:149`; today -12° floor -> -4° 0).
At -4° that is 22.6 nits at EV 7 (scene 0.044, the night's) instead of 0. Moving the EV schedule instead would also lift
the sunset sky, sun disk and lamps by up to 4 stops at dusk; the hold changes only the floor. The night glow reads
`SkyState::ambient` (`sf_sky_bevy/src/lib.rs:897`), so it holds too (check under the twilight sky, EX-5), and so does the
sun gate's floor (`lib.rs:799`).

**Cloud ambient (X3).** The deck's own diffuse light, which real overcast keeps (the in-scatter Decima keeps under dense
cloud): A_c = `sky.cloud.ambient_nits` x cover x (S / `sun_illuminance`) x m_w, so it follows the sun and is 0 at night.
m_w = `Weather::cloud_ambient` (ratio, default 1), blended from `SkyTerms::cloud_ambient_by_hour`: Morrowind maps it in
`mw_weather/src/ambient.rs` with `tint_at`'s ratio code, (luminance of the weather's `Ambient` colour at the hour /
Overcast's)^`sky.cloud.ini_brightness` (Overcast is the reference: Clear has no deck, its cover is 0). Tinted with the
weather's ambient tint. New `SkyState::cloud_ambient` (nits): `sf_sky_bevy` adds it to `AmbientLight` and the sun gate's
floor, never to the night glow. A_c enters B_w, so X2's bias shrinks once X3 lands (X0 re-run, k reset).

### Options page "Image" (owner 2026-11-17: every value has a row)

A new `ui.setting` page `image` (`scope = "Machine"`, like Graphics), one row per tunable below, bound by key exactly
as `graphics_page` binds the preset drivers (`mw_exterior_bevy/src/graphics_page.rs`); floats are sliders
(`SettingKind::Float`, `log_scale` off), choices are choice rows. Two sections: **Image** (Brightness = bias,
Eye adaptation = mode, Adaptation speed, Tone mapping, Bloom, Night darkness, Cloudy-day brightness) and **Advanced** (all the rest).
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
| `exposure.min_ev100` | -10 (*game* 4.5, EX-1) | -10..16 EV100 | Owner 2026-10-13 option A | Night darkness (with ambient) |
| `exposure.max_ev100` | 20 (*game* 16) | -10..20 EV100 | Guess (above noon 14) | Advanced |
| `exposure.fixed_ev100` | 7 | -10..20 EV100 | Reference (`SCENE_EV100`) | Advanced |
| `exposure.meter.low_pct`, `high_pct` | 10, 90 | 0..100 % | BevyDefault `filter`; UE Low/High Percent | Advanced |
| `exposure.meter.center_weight` | 0.5 | 0..1 ratio | Guess (UE centre-weighted metering) | Advanced |
| `exposure.comp.ev_m2` .. `ev_14` (5) | from the schedule (E4 test) | -6..6 stops | Measured (fit to `SkySettings::exposure`) | Advanced |
| `exposure.reset_on_cut` | on | bool | Industry (UE camera-cut reset) | Advanced |
| `sky.night.ambient_nits` | 0.03 (*game* 4, then EX-1) | 0..20 nits | Owner 2026-10-19 (LT-1) | Night darkness |
| `sky.moon.light_floor_ratio` | 2.5 | 0..10 ratio | Reference (`FULL_MOONLIGHT_FLOOR_RATIO`, artistic) | Advanced |
| `sky.night.glow_ratio` | `NIGHT_SKY_GLOW_RATIO` | 0..5 ratio | as declared today | Advanced |
| `exposure.weather.k` | 0.5, then X0's value | 0..1 ratio | Guess; Measured in X0 for Overcast 1.5 stops under Clear (owner 2026-11-26) | Cloudy-day brightness |
| `exposure.weather.max_boost_ev` | 2.5 | 0..4 stops | Guess (research 2026-11-26 §6) | Advanced |
| `exposure.weather.ease_speed` | 1 | 0.2..3 stops/s | Industry (UE Speed Down 1) | Advanced |
| `exposure.weather.storm_scale` (*game*) | 0.6 | 0..1 ratio | Owner 2026-11-26 (storms darker); value Guess (research 0.4 / 0.7) | Advanced |
| `exposure.twilight.hold_deg` | -2 | -6..0 deg | Guess (X1 test must pass) | Advanced |
| `exposure.twilight.end_deg` | 2 | 0..6 deg | Guess (X1 test must pass) | Advanced |
| `sky.cloud.ambient_nits` | 1500 | 0..5000 nits | Guess (CIE overcast sky: 1-10 klx under heavy overcast, E = pi L) | Advanced |
| `sky.cloud.ini_brightness` (*game*) | 1 | 0..2 exponent | Reference (Morrowind.ini's full ratio over Overcast) | Advanced |
| `render.tonemap` | `tony_mcmapface` | Bevy's ten | BevyDefault `Tonemapping` | Tone mapping |
| `render.grade.saturation` | 1 | 0..2 ratio | BevyDefault `ColorGradingGlobal::post_saturation` | Advanced |
| `render.grade.temperature` | 0 | -1..1 | BevyDefault | Advanced |
| `render.grade.highlights_gain`, `_contrast` | 1, 1 | 0..2 ratio | BevyDefault `ColorGradingSection` | Advanced |
| `render.grade.shadows_lift`, `_contrast` | 0, 1 | -0.5..0.5, 0..2 | BevyDefault | Advanced |
| `render.bloom`, `.intensity` (exist) | per preset | | as declared | Bloom |
| `render.bloom.threshold` | 0 | 0..10 (scene units) | BevyDefault `BloomPrefilter::threshold` | Advanced |

"Night darkness" is one player slider driving `sky.night.ambient_nits` (log scale over 0.5..8); the floor has its own
Advanced row. The X1-X3 framework tunables are declared in `sf_sky` (as `sky.night.glow_ratio`), the *game* ones in
`mw_weather`; Morrowind values that differ from the framework's go through `LightCalibration::apply_sky`
(`mw_assets/src/lights.rs:147`), as E1's.

## Slices

One deliverable plus its test each; each behind its switch; values logged in the ledger line whatever the result.

- **E0 night A/B, E1 night tunables, E1b night sky colour: built 2026-11-17** (awaiting owner, EX-1): `exposure.min_ev100`
  4.5, live night tunables declared game-side in `mw_assets/src/lights.rs`, vendored `DirectionalLightAtmosphereScatter`
  with `sky.moon.sky_scatter` 0.15. Slice text, diagnosis and as-built: [history](archive/exposure-tonemap-history.md).
- **X0 light-budget measurement (headless, no render).** New `bevyrrowind/crates/mw_weather/tests/exposure_weather.rs`,
  `#[ignore = "measurement: light budget and mid-grey scene value by weather and sun"]`: for Clear, Overcast, Thunderstorm
  and Ashstorm (game presets from `WeatherBook`, vanilla ini fallback) at sun 30, 15, 8, -4, -8, -12°, evaluated at the
  elevation (not the clock): S, F (`sky_fill`), d, c, night and cloud ambient (nits), EV100 (schedule, then with each X
  term once built), mid-grey scene value (albedo 0.3, facing up, the renderer's ambient colour), stops vs Clear, the
  deficit D and the k leaving Overcast 1.5 stops under Clear at 30°. Calibration copied as `print_moonlight` does
  (`tests/night_weather.rs:243`). Test (not ignored): every value finite and >= 0; at sun >= 8°, Clear >= Overcast >=
  Thunderstorm scene value; S and F rise with elevation. The output replaces the research table (ledger line); re-run
  after X2 (sets k) and X3 (resets it).
- **X1 twilight hold.** `TwilightHold` in `SkySettings`, the floor scaled in `evaluate` (Design), `apply_sky`'s curve flat,
  `exposure.twilight.hold_deg` / `.end_deg` live. Test (`sf_sky`, exact on the CPU model, Clear, moonless, game
  calibration): mid-grey scene value at every 0.5° from -12° to +2° >= its -12° value; at -12° and below unchanged
  (exact). Owner check EX-5.
- **X2 weather EV compensation.** `sf_sky::ExposureWeather { k, max_boost_ev, ease_speed }` in `SkySettings`,
  `LightBudget`, pure `weather_ev_bias`; `Weather::exposure_k_scale` + `SkyTerms` field; `CloudShadowMean` from
  `sf_atmos_bevy`; the ease in `sf_sky_bevy`; `exposure.weather.storm_scale` on the four storm presets (`mw_weather`); k's
  default from X0. Tests (exact): Clear -> 0; cover 1 with c < 1 -> bias in (0, max]; a known budget -> its value; 0 below
  the horizon; storm scale lowers it; the eased bias is continuous, reaches the target in |delta| / speed s and snaps on
  a cut. Owner checks EX-4, EX-6.
- **X3 cloud ambient.** `sky.cloud.ambient_nits`, `SkyState::cloud_ambient`, `Weather::cloud_ambient` from
  `SkyTerms::cloud_ambient_by_hour` (mapped in `mw_weather/src/ambient.rs`, `sky.cloud.ini_brightness`); `sf_sky_bevy`
  adds it to `AmbientLight` and the sun gate floor, not the glow. Test: the `ambient.rs` ratio test
  (`tint_curves_carry_the_chroma_at_full_brightness`, `:363`) extended to day hours: each weather's day ratio over
  Overcast's = the ini luminance ratio ^ exponent (exact), cover 0 -> 0 nits; then X0 re-run. Owner check EX-4.
- **Owner sitting after X1-X3:** EX-4, EX-5, EX-6 in one batch (`visual-checks/weather-sky.md`); WE-17 rechecked there.
- **E2 look tunables + Image page.** `render.tonemap`, grade, bloom threshold, the `image` page with every row of
  the table (rows for E4 values hidden until E4). Test: each row's key is a declared tunable with its range;
  picking a row sets the `User` layer (as `graphics_page` tests). Owner check EX-2: noon street and forest edge,
  tonemappers cycled from the console.
- **E3-E5 deferred (owner 2026-11-26):** live eye adaptation waits until X1 and X2 are built and checked (EX-4..EX-6);
  the owner then decides between it and the deterministic look (interior <-> exterior transitions are the strong case).
  E6 follows E4.
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
- **Small deficit:** if X0 measures Overcast D <= 1.5 stops, X2 does nothing by default and the dark frame is the fill
  and the toe (X3, E2); X0's report says which before X2's default is set.
- **Twilight side effects:** the held floor also holds the night glow and the sun gate's floor through dusk (EX-5).

## Open questions (owner)

Taste only, answered by the checks: the night darkness level (EX-1), the tonemapper (EX-2), adaptation on by
default and its speed (EX-3). Everything else above is decided from the industry standard.

Answered (owner via meta-3, 2026-11-26; research `exposure-overcast-2026-11-26.md` §8): (1) overcast stays ~1.5 stops
under clear by default, as a live tunable (`exposure.weather.k`); (2) storms (Thunderstorm, Ashstorm, Blight, Blizzard)
keep a lower compensation and stay darker (`exposure.weather.storm_scale`); (3) lamps reading brighter under overcast
are accepted (lit-towns rule); (4) live eye adaptation (E3-E5) waits until X1 and X2 are built and checked.
