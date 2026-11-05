# Sky and lighting: as built (history)

As-built notes for `sky-lighting.md` (the design), split off when it passed the 25 KB spec budget.

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
