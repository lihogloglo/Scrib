# Sky and lighting: as built (history)

As-built notes for `sky-lighting.md` (the design), split off when it passed the 25 KB spec budget.

## Night sky glow tied to the night floor (night N2, 2026-11-06)

Why: after N1 the ground was lit by a 4-nit diffuse floor while the sky rendered black, so nothing in the sky
explained the light and the env map had nothing to reflect. Industry: OpenMW draws Morrowind.ini's per-weather
"Sky Night Color" (a dark colour in the ambient's family); UE's sky light captures the sky dome including its
night glow; physically the moonless sky is ~0.2 mcd/m² of airglow + starlight, brighter toward the horizon
(van Rhijn). Games raise it with the fill. Bevy 0.20's atmosphere has no emission term: vendored patch "own, N2".

- `sf_sky::NightGlow` (`SkySettings::night_glow`): zenith luminance = `NIGHT_SKY_GLOW_RATIO` (0.1; 0.15 read as dusk in a debug shot) × the floor's
  luminance (`SkyState::ambient` × `ambient_color` luminance: same -4° → -12° ramp); hue = `Weather::night_sky_tint`
  (luminance-normalised; else the floor's hue). Profile 1/sqrt(1 - k²sin²z), k² = 1 - 1/H², H =
  `NIGHT_SKY_HORIZON_RATIO` (3): zenith 1, horizon H. A ground hit sees albedo × 2H/(H+1) × zenith (closed form).
- `sf_sky_bevy::night_sky_radiance` writes `AtmosphereSettings::{airglow, airglow_horizon}` per sky camera
  (`apply_cameras`); `SkyRenderOptions::night_sky_glow` is the A/B. The patch adds `airglow(r, mu)` to every texel of
  the sky-view LUT (and the ray-marched sky), so it sits under the clouds and stars and everything reading the LUT
  sees it: the sky pass, `AtmosphereEnvironmentMapLight` (specular reflections and diffuse irradiance of the night
  sky, refreshed with the env map), cloud ambient (`sky_up`: an overcast night is dim cloud, not black) and the
  fog's sky fill. Stars are additive on top; the faintest now sink into the glow, as in a real sky.
- Numbers (`mw_weather` `print_night_sky_glow`, EV100 3.5): floor 2.6 nits at 23:30 → zenith 0.26 nits (exposed
  0.019), horizon 0.79 (0.058); at 21:00/3:00 (sun -10°) 0.20 / 0.61. The env map adds ~15 % of the floor to
  an upward face (no compensation of the flat floor yet). Vanilla hues: Clear B/R 1.23 (the ambient's is 1.60),
  Overcast 1.49, Foggy 1.92, Blight red.
- Game: `mw_weather::ambient` `Setting::Sky` (`Sky ... Color` + `Sky Pre/Post-Sunrise/Sunset Time`, OpenMW
  `addSetting("Sky")`, ca5e2576) → `SkyTerms::night_sky_tint_by_hour` → sky-channel blend → `Weather::night_sky_tint`.
- Tests: `sf_sky` `night_glow_ties_to_the_floor_and_rises_to_the_horizon`, `sf_weather`
  `night_sky_tint_follows_the_hour_and_the_sky_channel`, `sf_sky_bevy` `night_sky_glow_reaches_the_atmosphere_tied_to_the_floor`
  + `shader_tests` (sky-view LUT, sky pass ×4, env capture link; Rust/WGSL `AtmosphereSettings` sizes agree),
  `mw_weather` `sky_colours_and_fades_parse_and_drive_the_night_sky_hue`, `mw_devtools_bevy` console switch.

## Flat ambient lights diffuse only (night N1, 2026-11-06)

Why: Bevy's flat ambient (`GlobalAmbientLight` / a camera's `AmbientLight`) feeds both lobes in
`pbr_ambient.wesl` (diffuse `EnvBRDFApprox(albedo, F_AB(1, N·V))`, specular `F0·A + B` × `saturate(50·F0)`),
so the night floor reflected a uniform light the black sky does not contain: a bluish sheen on every
surface (Bevyrrowind RO-16; research `bevyrrowind/docs/research/night-lighting-2026-11-06.md`). Industry:
fixed-function GL, OpenMW and Skyrim treat ambient as diffuse-only; specular comes from a visible source
(env map / sky capture, lights). Bevy 0.20 has no switch (`AmbientLight` = colour, brightness,
`affects_lightmapped_meshes`), so a vendored patch (`superfargoth/vendor/README.md`, "own, N1").

- `bevy_pbr::AmbientLightSpecular(bool)` (main-world resource, default `true` = stock Bevy; extracted):
  `prepare_lights` sets `AmbientLightFlags::DIFFUSE_ONLY` (bit 1 of the view's `ambient_light_flags`)
  when it is `false`; `pbr_ambient.wesl` zeroes the ambient specular under that bit. A uniform bit, not a
  shader def: one pipeline, the flip is live and free. Every lit path calls `ambient_light` through
  `apply_pbr_lighting` (`StandardMaterial` forward and deferred, terrain, far proxies/impostors, surface
  weather); env-map, light-probe/GI and punctual specular are untouched.
- Day exteriors carry no flat ambient (the floor fades out by sun -4°), so daylight is unchanged; interiors
  (cell ambient) lose the ambient sheen too, as in OpenMW.
- Measured (`sf_sky_bevy` `flat_ambient_specular_share`, polynomial `F_AB`): at N·V 0.15, roughness 0.85
  the ambient specular was 3-13 % of the terrain's ambient (F0 0.0144, albedo 0.5-0.1) and 9-34 % of a
  default `StandardMaterial`'s (F0 0.04); 0 % now.
- Bevyrrowind sets it in `mw_environment_bevy::RenderSettings::insert`; A/B `flip ambient_specular`.
- Tests: `sf_sky_bevy` `shader_tests` (forward Low/High, lightmapped, deferred lighting link and validate
  with the gate; Rust bit = WGSL bit), `mw_environment_bevy` `render_settings_make_the_flat_ambient_diffuse_only`,
  `mw_devtools_bevy` `ambient_specular_switch_flips_the_pbr_resource`.

## Ambient tint from the weather (RO-16 fix 2, 2026-11-06)

Why: the night ambient floor (`SkySettings::ambient`, 4 nits in Bevyrrowind) had a fixed linear tint
(`SkyRenderOptions::ambient_color` (0.55, 0.65, 1.0)) whatever the weather or the visible night sky; the
owner's `flip ambient` test tied the blue-grey terrain "veil" to it (Bevyrrowind visual check RO-16).
Industry: OpenMW (and vanilla Morrowind) take the ambient colour from the weather, four colours over the
day blended across weather transitions; UE/Unity drive sky light colour from the sky capture or a
time-of-day gradient. The flat floor is our artistic stand-in, so it now takes the weather's hue.

- `sf_sky::Weather::ambient_tint: Option<Vec3>`: linear RGB, luminance relative to the renderer's
  default tint (1 = same brightness). `None` = no say; `Weather::lerp` keeps the other side's tint.
  `SkyState::ambient_tint` copies it.
- `sf_weather::SkyTerms::ambient_tint_by_hour: [KeyCurve; 3]` (R, G, B over the hour of day; any curve
  empty = no tint). `evaluate` averages the presets that have one with their sky-channel weights (0-60 %
  smoothstep window, weights sum to 1), so a blend of luminance-1 tints stays at luminance 1.
  `sf_weather_bevy::apply_sky` copies the frame's tint into `WeatherState`.
- `sf_sky_bevy::apply_cameras`: `AmbientLight.color` = `ambient_color(state, options)`: the tint ×
  Rec. 709 luminance of `SkyRenderOptions::ambient_color` (the reference), or that colour when the tint
  is `None` or `SkyRenderOptions::weather_ambient_tint` is off (A/B). Brightness stays `SkyState::ambient`.
  It now also rewrites the colour when only the colour changed (it used to update on brightness only).
- Game side (Bevyrrowind `mw_weather::ambient`, OpenMW port): `Morrowind.ini` ambient colours by time of
  day, decoded to linear and normalised to luminance 1 (chromaticity only; a game constant can let the
  weathers' relative brightness through, default 0).
- Tests: `sf_sky` `ambient_tint_blends_and_flows_into_the_sky_state`; `sf_weather`
  `ambient_tint_follows_the_hour_and_the_sky_channel`; `sf_sky_bevy`
  `weather_ambient_tint_reaches_the_camera_at_the_same_luminance`.

## Moonlight (night N3, 2026-11-06)

Why: the one moonlight (`SkyState::moon`) carried the drawn disks' photometric illuminance (≈ 1 lx in
all), ≈ 0.05 nit on the ground next to a 4-nit floor: invisible. Industry: UE lights the night with a
second directional light (atmosphere light index 1) whose intensity is set apart from the moon material;
Skyrim, Witcher 3 and RDR2 light nights with an exaggerated, phase- or weather-scaled moon plus a sky
fill. OpenMW keeps its sun light on at night, coloured `Sun Night Color`, from above (`weather.cpp`
ca5e2576 `WeatherManager::update`, `setSunColour(mSunColor, ...)`): Morrowind's moonlight.

- `sf_sky::Moonlight { full_lux, artistic }` on `SkySettings::moonlight`: when artistic, the light's
  illuminance is `full_lux / Σ Moon::illuminance` times the old sum (each moon's share, Lambert phase,
  `Weather::direct_light_factor` cloud dimming all kept); off = the disks' own values (A/B, default).
  `MoonState::illuminance` / `full_illuminance` (the drawn disks) are unchanged. The ambient floor never
  depends on it (new moon = floor only).
- `SkyLight::tint` (luminance 1; the sun's white): `Weather::moonlight_tint` (hue only, lerped like the
  other tints) else the brightest moon's tint. `sf_sky_bevy::write_light` sets it as the light colour on
  raster (the shader applies the atmosphere's transmittance; the old fixed bluish colour is gone).
- `SkyRenderOptions::moon_cascades` (`MOON_CASCADES`: 2 cascades, 120 m, bounds ≈ 16/120 m) and the
  sun's depth/normal bias for the moon light. One caster at a time, as before.
- Plumbing: `sf_weather::SkyTerms::moonlight_tint_by_hour` → frame blend → `sf_weather_bevy` copy.
  Game: `mw_weather::ambient::Setting::Sun` (vanilla `Sun ... Color` and `Sun` fades 0/0/1/1.25 in
  `ini.rs`), `mw_assets::FULL_MOONLIGHT_FLOOR_RATIO` 2.5 → 31.4 lx (`LightCalibration::apply_sky`).
- Measured (`mw_weather` `print_moonlight`, Clear / Overcast, stone 0.3, EV 3.5): lit face 1.11-1.31x
  the floor at full (both moons or full-ish Masser), 0.57x at half (scene default day 172), 0 at new;
  overcast 0.10-0.24x. Exposed: shadow 0.058, lit face 0.09-0.13. Clear hue (0.35, 0.95, 3.44).
- Known: Bevy's atmosphere scatters every directional light, so the moonlight also lights the sky
  (a faint blue moonlit sky on top of the N2 glow; honest, but brighter). If it reads as dusk: lower the
  ratio or a vendored per-light "no atmosphere" flag. Cloud shadows still follow the sun only
  (`CloudShadowLight`); Bevy has no angular size for directional specular, so the highlight's softness
  is the material's roughness.
- Tests: `sf_sky` `moonlight_is_full_lux_at_full_moon_and_follows_the_phase`,
  `moonlight_off_is_the_disks_own_light`, `the_ambient_floor_stays_whatever_the_moon`,
  `clouds_dim_the_moonlight_like_the_sun`, `moonlight_colour_is_the_weathers_hue_else_the_moons`;
  `sf_sky_bevy` `moon_light_follows_the_moonlight_setting`; `mw_weather`
  `sun_colours_and_fades_parse_and_drive_the_moonlight_hue`; `mw_assets`
  `apply_sky_sets_the_moonlight_from_the_floor`; `mw_devtools_bevy` `moonlight_switch_flips_the_sky_settings`.
