# Visual checks: weather-sky

Open visual checks: weather, fog, clouds, sky, night sky, haze (WE, FG, SK, NS). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Distant shadows (superfargoth `weather/distant-shadows.md`, "Near-term visual fixes", bug A)

The shadows session (2026-10-12) moved the sun's cascades to 504 m (2048² after shadows2; 4096² cost +1.4 ms)
and added a terrain far shadow past them. Screenshots: `shots\shadows\`
(`before_*`, `after_*`, `north_1830_on/off.png`). Cost measured +2.5 ms at
1440p (over budget; being cut).

### FG-1: Volumetric fog black bands on fast turns (owner report)
- Kind: game location: fog lit by lamps at night (owner shot `Desktop/shadow fog.jpg`: giant-mushroom/cave area,
  21:15, fog). Turn the camera fast left or right.
- Seen by owner 2026-10-20: dark vertical bands in the lit fog during fast yaw. Repro history (renderfix1, froxel
  history, lamp-range edge, sun cascades) is in `archive/visual-checks-closed.md`, "FG-1 history".
- Status: not reproduced 2026-10-21 (owner, Sadrith Mora 21:15 foggy): fog code unchanged since the report; the sun
  cascades changed in shadow reset A2/B3, likely the fix. Reopen if seen.

## Sky (superfargoth `sky-lighting.md`)

### SK-2: Night and moon
- Kind: game location. Run: `.\bevyrrowind scene sky-night`
- Look for: moonlit, readable night, not grey-washed; town lights carry it.
  Known: no stars (M6), the moon disk has no phase shape.
- Status: deferred 2026-10-19 (phase0): recheck after the Phase 1 night brightness fix (LT-1 rejected).
- Source: sky-lighting.

## Clouds (superfargoth `weather/clouds.md`, "Clouds"; `.sessions/clouds.md`, `.sessions/clouds2.md`)

Volumetric clouds are on by default in exteriors. `--clouds <name>` picks a genus (clear,
fair, cumulus, stratocumulus, altocumulus, altostratus, cirrus, overcast, storm) or a
Morrowind weather (cloudy, foggy, rain, thunderstorm, ashstorm, blight, snow, blizzard;
each with its `Cloud Speed`); default fair. `--off clouds` drops them (the A/B). In the
viewer, Shift+C toggles them (F10 until 2026-10-25) and C cycles the nine genera live. Hours: `[` `]` or `--hour`.
Synthetic scene (no game data): `cargo run --release -p sf_atmos_bevy --example clouds --
--preset <genus> --scale 0.5 --view sky|sun|shadow|up` in superfargoth.

S1 rework (2026-10-13): cloud size and height both at half real scale (fair cumulus a few
hundred meters wide, base 600 m), clouds move (Morrowind `Cloud Speed` x 10 m/s: cloudy
20 m/s) and re-form, cloud shadows now actually reach the ground (the light texture was
silently disabled), and the sun is fully hidden behind thick cloud.

## Night sky and sun disk (superfargoth `sky-lighting.md` "Night sky and drawn bodies"; `.sessions/nightsky.md`)

### NS-2: Stars fade in at dusk (`scene ns-dusk-stars`)
- Kind: game location. Procedural star field (framework), hidden by the sky's brightness, the atmosphere near the
  horizon and clouds, plus a fade from sun −4° to −14°.
- Look for: `--hour 20`: no stars; `20.5`: a few bright stars only in the dark part of the sky (east, high); `21.5`:
  a full field of mostly faint stars, warm and blue tints, fewer and dimmer toward the horizon, a slight twinkle
  low down; `--clouds fair`: no stars through clouds. Stars look like points, not squares or blurs.
- Status: deferred 2026-10-19 (phase0): recheck after the Phase 1 night brightness fix (LT-1 rejected).
- Source: nightsky.

### NS-3: Masser and Secunda (`scene ns-moons-full`, `scene ns-moons-quarter`, `--day 159` crescent)
- Kind: game location (Morrowind's `tx_masser_*` / `tx_secunda_*` phase textures, `[Moons]` sizes and tilts).
- Look for: Masser large (~8° disk), Secunda smaller (~3.4°); the phase texture matches the day (168 full, 162 first
  quarter, 159 crescent; both moons share the phase); no stars inside a moon's disk (dark side included) and no
  dark ring of missing stars around it (the `occluder` margin, 0.85 of the texture); moons move east to west over
  the night and rise about an hour later each day; by day they are invisible or faint, not dark disks.
- Status: deferred 2026-10-19 (phase0): recheck after the Phase 1 night brightness fix (LT-1 rejected).
  texture at day 168, Secunda half-lit at 162; at dusk (`ns-dusk-stars`) Masser shows pale against the twilight.
- Source: nightsky.

### NS-4: No red streaks at night (`scene ns-night-clear`, `scene lt-torch-shadows`)
- Kind: game location. Bug: with `--clouds clear` red/pink streaks crossed every Balmora night sky. Cause: the cirrus
  layer was lit by the set sun (the cloud key light was picked by raw illuminance, and the raster path keeps the sun's
  unattenuated value after sunset) through Bevy's transmittance LUT, which clamps below-horizon rays to the horizon's
  red instead of 0. Fixed: key light by its light at the camera, and the planet's shadow at every cloud sample.
- Look for: no red or pink anywhere in the night sky; thin cirrus shows as faint dark or moon-grey wisps; sunset
  clouds still glow orange while the sun is up (`--hour 19.4 --clouds fair`).
- Status: deferred 2026-10-19 (phase0): recheck after the Phase 1 night brightness fix (LT-1 rejected).
  cirrus now reads as dark wisps (it hides the stars and Bevy's faintly lit twilight sky behind it): say if that is
  objectionable (fix: moonlit cirrus fill, or fading the high layer at night).
- Source: nightsky.

## Haze and dust (superfargoth `weather/distant-shadows.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

### WE-5: Ash/blight storm ignores terrain and objects, wrong direction (owner report)
- Kind: game location. Run: `.\bevyrrowind scene summit-vivec` (or any ashstorm/blight weather near Red Mountain).
- Seen by owner 2026-10-19 (round 3): the blight storm (maybe other weather effects too) is not occluded by terrain or
  objects: it draws over them. Also the ash storm should blow from Red Mountain (Morrowind storms have a source;
  we2b already derives the summit storm source from LAND), which it doesn't seem to.
- Also (owner 2026-10-19, round 6): in some weathers the clouds extend very high into the sky (deck base/thickness, WE4b local heights?).
- Status: occlusion part fixed, awaiting owner (storm-occlusion 2026-10-25). Cause (owner 2026-10-21, confirmed in code):
  the map always looked straight down and terrain was opted out, so nothing upwind sheltered near-flat ash (fall 1 m/s).
  Fix (`sf_precip_bevy` occlusion.rs): the map looks along the strongest kind's travel direction (never flatter than
  15°, lee ≈ 3.7 obstacle heights), eye 100 m upwind; terrain is an occluder again; rain stays ~top-down.
  Check: `.\bevyrrowind scene wk-balmora --weather ashstorm` (A/B `--off precipocc`): street in the lee of houses
  and hills shows little/no ash, open ground and the `fg-redmtn-ashstorm` hillside (70 m up) still streak. Rain under
  roofs unchanged (`--weather rain`). Tall clouds fixed 2026-10-28 (render-opt: Morrowind storm cloud deck thickness capped to 1.6 km in `mw_weather/src/visual.rs`, plus dry-weather `PrecipOcclusion` skip), awaiting owner. Storm direction from Red Mountain: Phase 4.
