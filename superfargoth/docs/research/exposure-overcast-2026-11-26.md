# Overcast darker than night: exposure and eye adaptation (research, 2026-11-26)

Research only (research-1). No builds, no measurements: every luminance below is **calculated from code constants** with the stated
assumptions; the first slice (X0) prints the real numbers headless. Unverified claims are marked (U).

## 1. Summary

- **Cause.** Camera EV100 is a function of the sun's elevation only (`sf_sky/src/sky.rs:268`, curve `sky.rs:77-86`). Weather never touches it.
  Overcast removes 70-95 % of the direct sun (`Weather::direct_light_factor`, `sf_sky/src/weather.rs:108`, x the cloud-shadow cookie,
  `cloud_shadow.wesl:60`) while EV stays at the daytime 12-14. Night is the opposite: EV is lifted to the 4.5 floor
  (`mw_assets/src/lights.rs:85`) and a 4-nit ambient floor + 31 lx moon are tuned for that EV. Day: light budget down 3-4 stops, eye unchanged.
  Night: light budget tiny, eye opened ~9 stops past the day value and hand-lit. So a heavy overcast day can read at or below the night frame.
- A second, weather-independent hole: the **twilight valley** (sun -8 to 0 deg). Ambient floor fades to 0 by -4 deg
  (`lights.rs:149`) but EV is still the schedule (5.5 at -6 deg, 7 at -4 deg, `sky.rs:80-81`), so dusk/dawn is darker than deep night (section 2.3).
- **Canonical technique.** Auto exposure (histogram, speed up/down, Min/Max EV100) + an exposure compensation curve over scene EV (UE, HDRP
  Curve Mapping) + per-time/weather artistic clamps + (UE5.1+) local exposure. We already specced it (`exposure-tonemap.md` E3/E4/E5) but only the night floor (E1) shipped.
- **Top fixes.** (1) Weather-aware analytic EV compensation, 0 ms CPU, no history issue, a few lines of Bevy-free maths in `sf_sky`.
  (2) Fix the twilight valley (floor fade tied to EV). (3) Full histogram auto mode (E3/E4/E5), ~0.1-0.3 ms GPU (U, to measure).
  (4) Tonemapper/local-contrast grade, ~0.05 ms.
- **Owner questions:** how much brighter overcast may read than "physically right" (recommend: keep a 1.5-stop gap below clear), and is a lamp-lit overcast town
  allowed to glow more (recommend yes).

## 2. What we do today

### 2.1 Chain (path:line)

| Term | Value / rule | Where |
|---|---|---|
| Sun above atmosphere | 130 000 lx | `sf_sky/src/sky.rs:67` |
| Cloud dimming of sun and moon | `1 - cover x overcast_dimming`; dimming default 0.8, **no weather data overrides it** (no `.ron`/`.toml` mentions it), so Overcast, Foggy, Storm all use 0.8 and differ only by `cover` | `weather.rs:54,108`; `sf_weather/src/frame.rs:337,420`; `sf_weather/src/records.rs:204` |
| Cloud shadow cookie on the sun | `lit = mix(1, exp(-od), shadow_strength 0.9)`; edge/average `1 - mean_cover x 0.9 x 0.6` | `sf_atmos_bevy/src/shaders/cloud_shadow.wesl:60-62`; `sf_atmos/src/settings.rs:274` |
| Sky fill / ambient by day | atmosphere env map (clear-sky LUT, **clouds are not in the env map**, ledger 2026-11-12 line) + env refresh every 8 frames; flat ambient curve is 0 by day | `sky.rs:93` (`-12: 0.03, -4: 0`), game replaces with 4 -> 0 at `lights.rs:149` |
| Weather ambient tint (Morrowind.ini) | colour only (luminance 1 = floor brightness unchanged), applies to the night floor | `sf_sky/src/weather.rs:36-40`, `mw_weather/src/ambient.rs` |
| EV100 | `exposure_range.clamp(schedule(sun_deg))`, schedule -18:0.5, -12:2, -6:5.5, -2:8.5, 2:10.5, 10:12.5, 25:13.5, 45:14 | `sky.rs:77-86,268` |
| Exposure floor | 4.5 (was 3.5), tunable `exposure.min_ev100` | `lights.rs:85,142-148` |
| Night ambient | 4 nits, tunable `sky.night.ambient_nits`, fades out -12 -> -4 deg | `lights.rs:94,149` |
| Moon | 2.5 x pi x 4 = 31.4 lx, dimmed by clouds like the sun (0.10-0.24x overcast, `sky-lighting-as-built.md:109`) | `lights.rs:153-157` |
| Written to camera | `Exposure{ev100}` on sky cameras; interiors EV 7 | `sf_sky_bevy/src/lib.rs:654,712,800` |
| Eye adaptation | **none** (AutoExposure removed with the portal spike, `exposure-tonemap.md:33`) | - |
| Tonemapper | Bevy default TonyMcMapface, bloom NATURAL | `exposure-tonemap.md:36` |

Bevy exposure: `scene_value = nits / (1.2 x 2^EV100)`.

### 2.2 Numbers (calculated; mid-grey ground, albedo 0.3, facing up; L = 0.3/pi x E)

Assumptions: atmospheric transmittance of the sun T = 0.75 at 30 deg, 0.65 at 15 deg; clear-sky hemispherical fill 15 klx at 30 deg, 10 klx at 15 deg
(typical; the real value is what the vendored LUT gives: measure in X0); overcast cover 1 (factor 0.2), cookie 0.3 (dense deck, range 0.1-0.5); the env-map fill is
**not dimmed by clouds** (best case for overcast; if the env map is dimmed it is worse). "Scene" = linear value before tonemapping.

| Case | Sun E (lx) | Fill E (lx) | EV100 | 1.0 = nits | Mid-grey nits | Scene value |
|---|---|---|---|---|---|---|
| Clear day, sun 30 deg | 130k x 0.5 x 0.75 = 49k | 15k | 13.6 | 14.9k | 6.1k | **0.41** |
| Overcast day, sun 30 deg | 49k x 0.2 x 0.3 = 2.9k | 15k | 13.6 | 14.9k | 1.7k | **0.11** (-1.9 stops vs clear) |
| Storm/very dense (cookie 0.1, fill halved by haze/rain/fog) | 1k | 7k | 13.6 | 14.9k | 0.76k | **0.05** (-3 stops) |
| Overcast, sun 15 deg | 33.6k x 0.65 x 0.2 x 0.3 = 1.3k | 10k | 12.8 | 8.7k | 1.1k | **0.12** |
| Overcast, sun 8 deg, T 0.5 | 18k x 0.5 x 0.06 = 0.5k | 3k | ~12.1 | 5.1k | 0.33k | **0.065** |
| Clear night, moon up (4 nit ambient, 31 lx moon at ~60 deg N.L 0.87) | 27 | ambient 4 nits (diffuse = albedo x 4 = 1.2 nit) | 4.5 | 27.2 | 1.2 + 2.6 = 3.8 | **0.14** |
| Clear night, no moon | - | 1.2 nit | 4.5 | 27.2 | 1.2 | **0.044** (+ sky glow in env map, unmeasured) |
| Overcast night (moon x0.2, ambient tint luminance 1; vanilla overcast nights are brighter, `ambient.rs:388`) | 5 | 1.2 nit | 4.5 | 27.2 | 1.8 | **0.066** |

So a heavy overcast day (0.05-0.12 scene, before the tonemapper's toe) lands in the same band as a moonlit night (0.044-0.14), and **below** a moonlit night
once the storm term and fog stack. After the TonyMcMapface toe (it is contrasty in the low end) 0.05 reads near black while night, which is also lit by lamps, sky glow
and a light-sky env map, reads as "blue and readable". The owner sees the day as very dark because the eye is not allowed to open: EV is 13.6 where an eye
(or UE's auto exposure) would sit near 11.5-12 (+2 stops = 4x brighter, scene 0.11 -> 0.44, same as clear).

### 2.3 Twilight valley (weather-independent)

| Sun deg | EV100 (`sky.rs`) | Ambient (nits) | 1.0 = nits | Ambient contribution | Note |
|---|---|---|---|---|---|
| -12 | 2.0 -> clamped 4.5 | 4 | 27.2 | 0.044 | night |
| -8 | 4.0 -> 4.5 | 2 | 27.2 | 0.022 | floor still holds EV |
| -6 | 5.5 | 1 | 54 | 0.0055 | EV leaves the floor, ambient 1/4 |
| -4 | 7.0 | 0 | 150 | 0 (only twilight sky fill, ~0.001-0.005) | **darkest point of the whole day** |

EV rises faster than the ambient floor fades: dusk/dawn at -6..-3 deg is 10-30x darker than deep night (0.044). With overcast on top, a dusk under cloud is the "very dark" frame.
This is a bug in the coupling of two independently tuned curves, not a physical effect.

### 2.4 Exact cause

1. EV ignores weather (`sky.rs:268`). Dimming is applied to light only (`weather.rs:108`, cookie), so the scene gets 2-3 stops darker at the same EV.
2. Night is rescued by two terms that have no day equivalent: EV floor 4.5 and a 4-nit ambient floor (`lights.rs:85,94`). They were tuned to be readable, never tied to the day.
3. The weather ambient tint carries no brightness (`weather.rs:36`), so OpenMW's brighter overcast ambient (Morrowind.ini) is not in the day lighting at all.
4. The twilight crossover (2.3).

## 3. Bevy 0.20 native and crates

Read in `bevy_post_process-0.20.0/src/auto_exposure/`:

- `AutoExposure` (`settings.rs:31`): 64-bin luminance histogram (`auto_exposure.wesl:78`, bins 1..63, bin 0 = below range ignored), `range` -8..8 EV (default), `filter` 0.10..0.90
  percentile clip, `speed_brighten` / `speed_darken` stops per second (defaults 3 / 1), `exponential_transition_distance` 1.5, `metering_mask: Handle<Image>` (red channel weight),
  `compensation_curve: Handle<AutoExposureCompensationCurve>` (256-texel 1D LUT over metered log2 luminance, min/max configurable, extend modes).
- Pipeline: two compute passes (histogram, then mean + adapt) per view, `queue_view_auto_exposure_pipelines` keyed per view entity, state in per-view buffer.
- **It does not change `Exposure`.** The adapted value is added to `view.color_grading.exposure` (`auto_exposure.wesl:193`) and applied in the tonemapping shader, i.e. after
  lighting and bloom. Everything that reads `view.exposure` (our 15 shaders, `exposure-tonemap.md:41`; the vendored atmosphere; particle floor gain) does not see it.
  This is why spec E3 plans a vendored meter-only patch + readback, then writes the adapted EV into `Exposure` (UE's pre-exposure model, correct for TAA/DLSS).
- Meters to mid-grey: a night meters up to grey unless the compensation curve pulls it down (the "grey-night problem", `sky-lighting.md:43`). Curve = the fix, as in UE.
- Cost: not measured here; 2 small dispatches at full-res read (histogram is bandwidth bound), expect ~0.1-0.3 ms at 1440p (U; DLSS Quality renders at 960p, so metering before upscale is ~0.05-0.15 ms). Measure in E3.
- DLSS/TAA: the plugin receives a pre-exposure value (`sf_dlss/src/lib.rs:596`); a continuously changing EV needs the exposure ratio applied to history (E5).
- Tonemapping in `bevy_core_pipeline-0.20.0/src/tonemapping`: ten operators incl. Reinhard, AcesFitted, AgX, SomewhatBoringDisplayTransform, TonyMcMapface, BlenderFilmic; `ColorGrading` (exposure, gamma, saturation, shadows/midtones/highlights). **No local exposure, no auto-gain on tonemap.** Crates: no maintained Bevy crate for local exposure found (U, not searched exhaustively); the sibling ones I know (bevy_mod_... post process) are stylization.

## 4. Industry

| Who | Technique | Source |
|---|---|---|
| **UE5** | Auto exposure "Histogram" (Low/High percent 80/98.3, Min/Max EV100, Speed Up 3 / Down 1 stops/s), "Basic" mode, **Exposure Compensation Curve** (compensation by scene EV100), global/post-process exposure compensation, metering mask, pre-exposure so lighting is written exposed; since 5.1 **Local Exposure** (highlight/shadow contrast, blur radius) compresses overcast/backlit scenes without moving global EV. Weather/time-of-day dims light physically; Min/Max EV clamps per volume are the artistic lever. | UE docs "Auto Exposure" |
| **HDRP/Unity** | Exposure override: Fixed, Automatic (histogram), Curve Mapping (EV = f(scene EV), the same idea as our proposed analytic curve), Use Physical Camera; Limit Min/Max, compensation, adaptation Progressive/Fixed speed. | HDRP Exposure docs |
| **Frostbite** | Physical units (lux, nits), EV100 exposure; auto exposure with artist compensation; lights in photometric units so exposure can be moved without relighting. | Lagarde & de Rousiers 2014 "Moving Frostbite to PBR" (exposure section) |
| **Hillaire sky** | Physically based sky in luminance; assumes the app picks exposure; sun 120 klx vs sky 10-20 klx; says nothing about artistic clamps. | Hillaire EGSR 2020 |
| **RDR2** | Bauer's talk covers atmosphere, clouds and light, aimed at readable moods; I found no verifiable public numbers for exposure handling (U). Widely reported behaviour: weather/time-of-day set artistic exposure and a lifted shadow floor so storms stay readable. | Advances in RT Rendering 2019 (index) |
| **Horizon / Decima** | Schneider's cloud lighting uses an ambient that does not collapse under dense cloud (in-scatter term, "ambient + powder"); exposure specifics not public in what I could verify (U). | Schneider SIGGRAPH 2015/2017 |
| **Ghost of Tsushima** | Patry's rendering talks (shading/tonemapping) stress a filmic curve with controlled shadows; exposure per weather unverified (U). | Advances in RT Rendering (Patry) |
| **Cyberpunk 2077** | No primary public exposure talk found; games of this class ship auto exposure + per-weather curves (U). | - |
| Real light | Sunny 100-130 klx, thin overcast 20-50 klx, heavy overcast 1-10 klx (storm ~1-5 klx), civil twilight ~3-10 lx, full moon 0.1-0.3 lx, starlight 0.001 lx. Day-to-night ratio 10^6 (20 stops); human eye + display compress it with ~14 stops of adaptation. | standard photometry (CIE/IES tables) |

**Canonical solution:** the scene is lit in photometric units; the displayed frame is made readable by (a) auto exposure with EV100 clamps (b) a compensation curve that keeps
night dark and overcast not black (c) artistic per-weather/time-of-day bias (d) filmic tonemapper toe and (e) from UE5.1 local exposure for contrast. No shipped game keeps one fixed daytime EV across weather.

## 5. Why ours differs

1. We have a deterministic schedule only (sun elevation), no scene feedback and no weather term: overcast and storm get the clear-day EV.
2. The night is hand-lit (ambient 4 nits, EV floor), the day is physical: two different calibrations, no crossover (2.3).
3. Direct light is dimmed twice (0.2 x cookie 0.1-0.5) but fill is not, and the weather's OpenMW ambient brightness is discarded, so the ratio sun:fill is unrealistic and
   the overcast frame is dim *and* flat.
4. The deterministic schedule was chosen because AutoExposure meters grey nights (`sky-lighting.md:43`); true for Bevy's curve-less use, solved in UE/HDRP by the compensation curve (spec E4 has it).
5. No local exposure: neither Bevy nor us, so global EV must carry everything.
6. Our look-pick (owner) prefers lit towns: the lamp rule (lamps read 2^(7 - EV)) means opening EV in overcast also brightens lamps and interiors' spill: a plus.

## 6. Ranked fixes

Frame target <= 8.3 ms median, <= 11 ms p99 (RTX 4060 laptop, High, 1440p + DLSS Quality). Costs are estimates (U) unless stated.

| # | Fix | ms (GPU / CPU) | Changes | Risk |
|---|---|---|---|---|
| 1 | **Weather exposure compensation (analytic).** `ev = schedule(sun) + k x log2(clear_budget / weather_budget)` with budget = `sun_E x direct_factor x mean_cookie + fill`, k tunable (~0.6-0.8), clamped to +2.5 EV and by `exposure.max_ev100`; eased in time (1-3 stops/s) | 0 / <0.01 | `exposure-tonemap.md` new "Weather compensation" + E4 base; `sf_sky::SkySettings::evaluate` (pure maths, tested); tunables `exposure.weather.k`, `.max_boost_ev`, `.ease_speed`; game values in `mw_assets/lights.rs` | low: lamps/particles in exposed units move 2^k (documented, RO-23 already uses exposed units) |
| 2 | **Twilight valley.** Tie ambient floor fade to EV: `ambient(-4..-12)` becomes `floor_nits x 2^(EV_floor_ref - EV)` blend, or hold the EV schedule at the floor until -2 deg (e.g. new key points) | 0 / 0 | `sky.rs:77-86` schedule keys + `lights.rs:149` curve; sky-lighting.md "Exposure" | low; changes dusk/dawn look: owner check |
| 3 | **Weather-aware ambient floor by day.** Day ambient = cloud fill (`sky_up`/cover) so overcast keeps a floor (OpenMW overcast ambient brightness as luminance, ports `ambient.rs` data) | 0 / 0 | `SkySettings::ambient`, weather ambient tint luminance != 1 | medium: shadows lose contrast (sun gate, `sun_gate.rs`, already handles) |
| 4 | **Histogram auto mode** (E3 meter-only readback + E4 adaptation + E5 history) as optional `exposure.mode auto` | 0.1-0.3 GPU / 0.05 CPU | `exposure-tonemap.md` E3-E5, vendored `bevy_post_process` | medium: pumping with DLSS, perf suite must force schedule |
| 5 | **Local exposure / shadow lift** (post, UE5.1 style: blur-based gain on shadows/highlights) | 0.2-0.4 GPU | new spec section; after #1 judge need | medium |
| 6 | **Tonemapper toe** (EX-2: AgX / TonyMcMapface vs PBR Neutral) | ~0 | `render.tonemap` tunable, E2 | owner taste |

Ranking by cost-benefit: 1 > 2 > 3 > 4 > 6 > 5. #1+#2 close the owner's symptom with zero ms.

## 7. Proposed slices (one deliverable + one test each; tunable = unit/range/source)

- **X0 measure (headless, no render).** Test `#[ignore = "measurement"]` in `mw_weather`/`sf_sky` printing sun E, fill E (sky-view/CPU `sky_fill`, `sky.rs:~240`), EV, mid-grey scene value for clear / overcast / storm at sun 30/15/8/-4/-8/-12 deg. Output: the real table replacing section 2.2. Test: finite, ordered values.
- **X1 twilight valley fix.** New key points (schedule or ambient curve) so the scene value at -4 deg is >= the deep-night value. Tunables stay `exposure.min_ev100`, `sky.night.ambient_nits` plus `exposure.twilight.hold_deg` (deg, -6..0, Guess). Test: mid-grey value is monotone non-dark across -12..0 (exact assert on the CPU model).
- **X2 weather EV compensation.** `sf_sky::ExposureWeather { k, max_boost_ev, ease_speed }` in `SkySettings`, pure fn `weather_ev_bias(&Weather, sun_E, fill_E)`. Tunables `exposure.weather.k` (0..1, Guess 0.7), `.max_boost_ev` (0..4 EV, Guess 2.5), `.ease_speed` (EV/s, 0.2..3, UE speeds). Game default in `bevyrrowind/crates/mw_assets/src/lights.rs`. Tests: clear -> bias 0; cover 1 + cookie -> bias in (0, max]; exact value at known inputs; ramp continuity.
- **X3 day ambient floor under cloud.** `sky.cloud.ambient_nits` curve from `Weather::cloud_cover`; Morrowind ini brightness mapped in `mw_weather/src/ambient.rs`. Test: `ambient.rs` ratio test extended to day hours.
- **E3/E4/E5 (existing spec slices)** only if X1+X2 leave the owner wanting adaptation (interiors <-> exteriors transitions).
- Owner check batch (one sitting): Balmora overcast/storm 15:00 and 18:30, dusk -4 deg clear, night 03:30, on `--exposure` modes; visual-check entries `EX-4..EX-6`.

## 8. Owner questions (taste/scope)

1. How much should overcast stay darker than clear (physically 3 stops; our default after X2 is ~1.5)? Recommend: keep 1.5 stops below clear so the drama is still there.
2. Should storms stay dramatically dark (keep a darker k for Thunderstorm/Blight)? Recommend: yes, k lower for storm presets (0.4) than Overcast (0.7).
3. Lamp/torch glow rises 2^k in overcast exposed units; lit towns at storm are brighter than at clear night. Accept? Recommend: yes (matches your lit-towns rule).
4. Do you want live eye adaptation (E3-E5) at all after X1/X2, or keep a deterministic look? Recommend: defer until X1/X2 checked; interior transitions are the only strong case.

## 9. Sources

Code read: `superfargoth/crates/sf_sky/src/sky.rs`, `weather.rs`; `sf_weather/src/frame.rs`, `records.rs`; `sf_sky_bevy/src/lib.rs`; `sf_atmos_bevy/src/shaders/cloud_shadow.wesl`; `sf_atmos/src/settings.rs`;
`bevyrrowind/crates/mw_assets/src/lights.rs`; specs `exposure-tonemap.md`, `sky-lighting.md`, `sky-lighting-as-built.md`; wiki `engine/sky-atmosphere-weather.md`, `engine/lights.md`; Bevy 0.20 `bevy_post_process-0.20.0/src/auto_exposure/{settings.rs,auto_exposure.wesl}`, `bevy_core_pipeline-0.20.0/src/tonemapping`.

External (recalled, not re-fetched in this session; verify before quoting numbers):
- UE Auto Exposure: https://dev.epicgames.com/documentation/en-us/unreal-engine/auto-exposure-in-unreal-engine
- Unity HDRP Exposure: https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@17.0/manual/Override-Exposure.html
- Lagarde, de Rousiers, "Moving Frostbite to PBR": https://seblagarde.files.wordpress.com/2015/07/course_notes_moving_frostbite_to_pbr_v32.pdf
- Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique": https://sebh.github.io/publications/egsr2020.pdf
- Bauer, "Creating the Atmospheric World of Red Dead Redemption 2" (SIGGRAPH 2019 Advances): https://advances.realtimerendering.com/s2019/index.htm
- Schneider, "Nubis" / Real-time Volumetric Cloudscapes (Advances 2015, 2017): https://advances.realtimerendering.com
- Patry, Ghost of Tsushima talks (Advances 2020/2021): https://advances.realtimerendering.com
