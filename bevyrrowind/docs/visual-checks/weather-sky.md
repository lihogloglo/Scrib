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
- Owner 2026-10-30 (`scene summit-vivec --weather ashstorm`): occlusion works. Too much shelter: on rugged ground you are
  almost always counted as sheltered, so the storm rarely blows at you. Wanted: shelter only within a few metres of a
  vertical object (cap the lee distance; ROADMAP 1.12 "cap ash shelter distance"). Thunderstorm clouds may still be
  too high (owner unsure). Status: confirmed with follow-up.

### WE-6: Rain turns grainy while the camera moves (owner report)
- Kind: game location. Rain in some areas of the island (seen 2026-10-30 from `scene balmora`, earlier passes too).
- Look for: rain streaks clean while still; as soon as the camera moves, heavy grain where the rain is.
- Suspect (not tested): translucents have no motion vectors (ROADMAP 1.3, RO-7), so TAA/DLSS reprojects rain wrongly.
- Status: known bug: not diagnosed (owner pass 2026-10-30).

### WE-7: Rain curtain stops in front of distant water (owner report)
- Kind: game location. Rain with open water in view (seen 2026-10-30).
- Look for: the distant rain curtain doesn't draw over the water body, so it ends at the water's edge.
- Suspect (not tested): pass order/depth between the water snapshot and precip (hard-problems H3, `render-order.md`).
- Status: known bug: not diagnosed (owner pass 2026-10-30).

### WE-8: Rain leans with the wind, capped, turning slowly (rain-slant, weather/wind.md "Precipitation tilt as built")
- Kind: game location. Run: `.\bevyrrowind view "Balmora" --hour 12 --weather rain`, then `--weather thunderstorm`
  (free camera, look across a street at the rain against a wall and the sky; wait a minute or two).
- Look for: rain near vertical in light wind (about 9-15° in Rain, up to ~25° in a thunderstorm, never steeper);
  the lean direction turns slowly (over a minute or two, no swinging); trees bend the same way the rain leans;
  the rain-under-roofs shelter follows the lean. Ash storms still blow near flat (uncapped).
- A/B: console (backquote) `flip rain_slant_cap`: off = the old raw lean (21-44°, turns with every wind change).
- Status: confirmed (owner 2026-11-06: "rain slant works").

### WE-9: Forced fog and rain seem to go away at night (owner 2026-11-06, long-standing)
- Seen: forced Foggy is dense by day but "see super far" after ~21:00; a forced Thunderstorm stops at "23.50" and comes back at ~7:00.
- Cause 1 (rain stopping, all forces made with C/BRP): the `]`/`[` hour keys (`live_ab.rs` ~487) and BRP `set hour` (`remote.rs` ~191)
  call `SkyClock::set_hour`, which keeps the day: 23.5 + 0.5 is 0:00 of the *same* day, 24 game hours back, before the live force
  started (`force_live` starts at `now`), so the forecast takes over (rain, sound, wetting) until the clock passes the C press again.
  `--weather` at launch is not affected (forced from 1e4 h before). Test: `mw_weather/tests/night_weather.rs` (prints it).
- Cause 2 (fog): the fog's in-scatter is lit by the moon and the sky-view LUT only (`fog_light.wesl`), not by the night ambient floor
  that lights surfaces (`LightCalibration::night_ambient_nits`, sun -4° to -12°, ~20:00-21:15) under the opened night exposure: the
  fog turns into a black absorber with no veil, so lamps, far-light sprites and floor-lit ground read far. Fog densities rise at night.
- Cause 3 (rain look): `fx::precip_brightness` drops to 0.04 x (1 - 0.5 overcast) below sun -5.7°, not x exposure nor the night
  ambient: drops become faint dark streaks by night (the rain is still falling and wetting).
- Fix (fog, 2026-11-06): the fog's fill adds the camera's `AmbientLight` (the surfaces' night floor, Foggy-tinted) to the sky fill,
  near (froxels, composite) and far (cloud trace). Run `view "Balmora" --hour 22 --weather foggy` (also `--hour 12`, `--hour 1`):
  at 22:00 and 1:00 a soft grey-blue veil hides the distance like the noon fog (~50 m), lamps and far hills fade into it, no black
  wall, no glow; `flip fog_night_light` (console) = the old sky-only fill (black, see-through night fog). Too bright? Say so.
- Owner 2026-11-06 (RO-16 pass): forced snowstorm still switches just before midnight and comes back in the morning (cause 1).
- Status: fog fix confirmed (owner 2026-11-06, `--hour 22`: "better"); causes 1 and 3 open.

### WE-10: Local storm seen from outside: cloud curtain and noisy border (owner 2026-11-06)
- Kind: game location. Run: `.\bevyrrowind scene we3-ridge`, then BRP `bevyrrowind/set {"weather": "ashstorm", "region": "Ashlands Region", "transition_hours": 0}` (look north) or `"rain"` / `"thunderstorm"` in `"Ascadian Isles Region"` (look east/south-east).
- Seen: from outside, the storm is a wall from the deck down to the ground, cut by a straight vertical edge at the region border; the
  deck and the curtain's edges shimmer with dither noise on a still camera.
- Cause (research `bevyrrowind/docs/research/cloud-weather-edges-2026-11-06.md`): 256 m border band cuts a solid deck; far shafts fill the
  whole region with the in-storm extinction (ash opaque ~300 m in); shape noise in height-fraction space stretches vertically;
  8 far steps up to 1.9 km long and a 50/50 cloud history leave the jitter unconverged. Over the far sea the curtain stops (as WE-7).
- Look for, after the fix: the storm deck thins out over kilometres, rain hangs under the rain cells, no vertical wall, no shimmer.
- Fix 1 (history): ~16-sample cloud history, 1.5σ YCoCg variance clip, blue-noise jitter. Still camera on the ash (north) and
  rain (east) borders: deck and curtain stop shimmering within ~1 s; fast turns and walking leave no smear behind silhouettes
  or the deck edge; lightning still flashes at once. A/B: console `flip cloud_history` (off = the old 50/50 resolve).
- Owner 2026-11-06 (ash, north): shimmer still there with fix 1; the region storm's cloud/rain still not drawn over the water (WE-7).
- Status: fix 1 failed; slices 2-3 (far rain shaped by clouds, deck edges) wait for the owner.

### FG-7: Blocky crenellated edges on distant land in fog (owner report)
- Kind: game location. Distant landmasses with fog around or behind them (seen 2026-10-30, `scene balmora`).
- Look for: stair-stepped blocky shapes along land silhouettes, only where fog is rendered; present when still, worse
  when moving.
- Suspect (not tested): froxel fog sampled at low resolution without a depth-aware upsample at silhouettes.
- Status: known bug: not diagnosed (owner pass 2026-10-30).

### FG-8: Weather fog vanishes at night (owner report)
- Kind: game location. Foggy weathers (fog, overcast, ash), compare 20:00 and 01:00 (`--hour`).
- Seen 2026-10-30: in some weathers the fog seems to disappear at night.
- Suspect (not tested): fog in-scatter scales with sun/moon light, so at night it goes dark and reads as no fog;
  the extinction (distance hiding) should stay.
- Status: known bug: not diagnosed (owner pass 2026-10-30).

### FG-9: Vertical light pillar toward a low sun in fog (owner report)
- Kind: game location. `view "Balmora" --hour 6.25 --weather foggy` (any fog, sun near the horizon), face the sun.
- Seen 2026-11-02: instead of shafts from the sun, a vertical pillar of light in front of the camera, from below the
  ground into the sky, that follows the camera. `flip terrain_far_shadows` / `clouds` / `sun_shadows`: no change.
- Cause: the fog's sky fill was per view *azimuth* (sky at 0.05 rad elevation + zenith), added at every pitch: a low
  sun's aureole lit a vertical column. Fixed 2026-11-02 (fg9-fog): one isotropic fill, the sky dome's mean radiance
  (SH L0, as UE/Frostbite volumetric fog), for froxels and far fog alike (fog.md "clouds_fog_light").
- Run: `powershell -File tools/run_dev.ps1 view "Balmora" --hour 6.25 --weather foggy`, face the sun, pitch up/down.
- Look for: no pillar; the glow toward the sun is a round forward lobe (key light HG) that stays centred on the sun,
  not a column; fog brightness even all around (no seam at ~384 m, the froxel range); also noon and 19:00.
- Status: confirmed (owner 2026-11-02: no pillar). God rays still not visible: FG-10.

### FG-10: No visible god rays at sunrise in fog (owner report)
- Kind: game location. `view "Balmora" --hour 6.25 --weather foggy` (also `--weather clear`), face the low sun with
  buildings or the ridge between you and it.
- Seen 2026-11-02 (after the FG-9 fix): no shafts at all, though froxel fog with CSM, terrain and cloud shadow terms
  is built (`froxel_inject.wesl:156-165`, fog.md "Froxel fog as built"). Suspects (not tested): froxel slices too
  coarse at 160x90x64 / 384 m, in-scatter too weak against the fog's ambient, shadow term not reaching the froxels.
- Status: known bug: not diagnosed (owner pass 2026-11-02).

### SW-2: Surface weather shelter, night and trees (surface-weather.md W2, W2b)
- Kind: game location. `powershell -File bevyrrowind/tools/run_dev.ps1 view "Balmora" --hour 23 --weather clear` (night: no
  sun shadows); console (`` ` ``): `surface debug exposure`, `surface set wet 1`, later `surface debug cells`, `surface debug off`.
- Look for (exposure, night): open ground warm yellow; under roofs, eaves and the bridge arches deep blue (full shelter);
  under tree canopies and bushes only partly blue (about 60% held back, leafy gaps lighter); slopes not self-shadowed;
  walk 20 m: the tint redraws (no stale map from the last daylight), no pop, fades to yellow ~64 m out.
- With `surface debug off` + wet 1: ground under trees visibly wet but less than open ground; under roofs dry.
- Cells: grey 32 m squares, dark borders, fixed as you walk; blue after wet; purple only past ~2 km.
- A/B: same run with `--off shelter`: everything open (yellow), soaked everywhere. Repeat at `--hour 12`: same result.
- Owner 2026-11-05 (rain, noon): failed, all yellow even under Vivec's arches (W2b anchor views drew layer 0 only; statics
  sit on the worldspace layer, so `queue_shadows` dropped every caster). Status: confirmed (owner 2026-11-06, after the render-layer fix).

### SW-7: Kick-up puffs in snow and ash (surface-weather.md W7)
- Kind: game location. `powershell -File bevyrrowind/tools/run_dev.ps1 view "Skaal Village" --hour 12 --weather snow`, console
  `surface set snow 1`; walk, then run, then sprint over open snow (third person), look at the feet. Ash: `view "Ghostgate"
  --hour 12 --weather ashstorm`, `surface set ash 1`. Night: same at `--hour 23`. A/B: restart with `--off kickup`.
- Look for: each foot-down throws a small white-blue puff of powder (short arc, ~0.5 s, bigger when running); ash puffs are
  grey-brown, lighter, and drift downwind; NPCs walking nearby throw smaller puffs; nothing on bare ground (known: an overhang over snowy ground still puffs).
- Light: puffs match the snow around them (sunlit vs in a rock's shadow), no glowing puffs at night (moon + floor light), fade
  into fog. Soft where they meet the ground (no hard line). Puddle splashes not built. Too much / too little? Say so.
- Owner 2026-11-06 (snow): OK for now but crude, to be reworked; at night the puffs turn into near pure-white circles.
  Ash (same day): a bit too big and bright; same near-white glow at night.
  Status: accepted for now; rework, size/brightness (ash) and night look pending.

### SW-6: Knee-deep snow and footprints (surface-weather.md W6a prints + W6b raised geometry)
- Kind: game location. `powershell -File bevyrrowind/tools/run_dev.ps1 view "Skaal Village" --hour 12 --weather snow`,
  console `surface set snow 1`; walk (third person) over open snowy ground, look back. `surface debug trails`: prints blue,
  lips red, ring edge purple; `surface debug off`. Live A/B: `off trails` / `on trails`, `off cover` / `on cover`.
- Look for: the snow stands ~knee deep (0.45 m) over the ground within ~20 m, easing down to the ground by ~24 m (no
  visible step); feet, boots and NPC legs sink into it (the body stays on the ground, never lifted onto the snow); each
  print is a real hole cut to the ground with a lit lip; snow thins to nothing under roofs and on steep slopes.
- DLSS/TAA on: no smear, ghosting or trailing smudge behind the feet as prints appear; no shimmer at print walls.
- While it snows the prints refill (~1 min real time at 30× for 0.45 m); `--weather clear`: they stay.
- Watch for: rock/house bases buried 0.45 m (expected), terrain poking through floors at doorways, z-fighting at the
  ~20-24 m fade. Ash (`view "Ghostgate"`, `surface set ash 1`): only a 3 cm crust.
- Owner 2026-11-06 (snow): the raised ring's edge is visible, very close, following the player; frequent sudden "resets"
  where the whole snow look (and the prints) changes at once after some movement (big_space recentre?); at `snow 0.4` on
  paving the whole ground rises over game hours, not just the snowy patches, so feet sink into cobblestone; deep snow
  should leave continuous trails, not separate prints (prints for shallow snow / ash). Ash (same day): same sudden resets
  (where the ash lies jumps after some movement); crust could be a little deeper (not much). Status: failed, not fixed.
- Fix 2026-11-06 (resets only): shelter redraws lost casters outside the camera's and sun's frusta (cover and prints under
  them flipped at each redraw); the redraw now marks its casters visible. Check: walk 100+ m near houses (Skaal) and in
  Ghostgate, turn around, look for jumps. Status: resets awaiting owner; ring edge, paving, trails, ash depth still open.

### SW-5: Snow and ash cover (surface-weather.md W5)
- Kind: game location. Ash: `powershell -File bevyrrowind/tools/run_dev.ps1 view "Ghostgate" --hour 12 --weather ashstorm`,
  console `surface set ash 1` (full), `surface set ash 0.4` (thin), `surface clear`. Snow: `view "Skaal Village" --hour 12
  --weather snow`, `surface set snow 1` / `0.4`. P pauses the clock (cover melts or washes over game hours).
- Look for: ground, roofs and flat tops of statics covered in the snow/ash look; steep slopes and walls bare (~37-63° fade);
  at 0.4 patchy: hollows, dark grout and drift patches (~64 m) first, bumps poking through; soft edges, no tiling seams.
- None under roofs, arches or the Ghostgate passage; none on characters (skinned) or far proxies; ends ~2 km out (cell window).
- Snow: slight sparkle within a few metres (sparse glints that hold still as you stand, flash as you move; no crawling noise
  under DLSS). Normal detail from the land texture catches low sun. A/B: `--off cover` (or `surface set snow 0`).
- Owner 2026-11-06 (snow): roofs and ground covered, none under arches: good. Same sudden "resets" as SW-6 (whole look
  changes at once); ash: same resets (where the ash lies jumps). Status: coverage OK, resets open (see SW-6).
- Fix 2026-11-06: shelter redraws (every 16 m, or as casters stream in) lost every roof/rock outside the camera's and sun's
  frusta (Bevy drops meshes no view sees), so cover under them flipped at each redraw; the redraw now marks its casters
  visible. Glints and wall streaks also keep their pattern across an origin shift. Check: walk 100+ m around houses and
  turn around, look for jumps. Status: resets awaiting owner.

### SW-4: Puddles in rain and after (surface-weather.md W4)
- Kind: game location. `powershell -File bevyrrowind/tools/run_dev.ps1 view "Balmora" --hour 12 --weather rain`; console:
  `surface set wet 1`, `surface set puddle 1` (full), `surface set puddle 0.5` (drying), `surface clear`. C: next weather.
- Look for: puddles in street hollows and on flat paving/floors, edges following dark grout and low spots (soft);
  flat, mirror-like water reflecting the sky; none on slopes or walls, none under roofs/arches, smaller under trees.
- Rain rings while it rains (denser in heavier rain); still water at once when the rain stops (C to a clear weather).
- At puddle 0.5 the puddles shrink from their edges. No shimmer on distant puddles; no rings on characters or far proxies.
- A/B: same run with `--off puddles`: the W3 wet look only, no standing water. Low preset: no puddles.
- Re-check (fix 2026-11-05): no puddles on slopes (street ramps, hill roads, stair ramps, slope feet); level ground and
  flat paving/floors still hold water. Confirmed 2026-11-05 ("puddles are good now").
- Retune 2026-11-06 (owner: faster, a bit more lenient): slope ramp none from 6°, full from 3° (was 4°/2°); puddles start
  at wet 0.7 (was 0.9) and fill at 1.25/h × rain (was 0.5): full ~70 game min (~2.5 real min) from dry ground, was ~2.5 h. Look for: puddles
  within one shower, gentle street slopes hold some, ramps still dry. Status: confirmed (owner 2026-11-06).

### SW-3: Wetness after rain (surface-weather.md W3)
- Kind: game location. `powershell -File bevyrrowind/tools/run_dev.ps1 view "Seyda Neen" --hour 10 --weather clear`; console:
  `surface set wet 1` (soaked), `surface set wet 0.5` (drying), `surface clear` (dry). A/B: same run with `--off wet`.
- Look for: ground, roads, plaster and wood darker and a little richer in colour, glossier (sun and sky highlights);
  smooth stone and metal get a sharp water sheen; normal-map detail softens under the film (no faceting, no noise).
- Under roofs, eaves, the dock's deck and tree canopies stays dry (soft edge ~0.75 m); `surface debug exposure` now
  tints statics too. Walls: vertical streaks, less wet than the ground; undersides dry.
- At 0.5: patchy drying (metre-scale patches), no tiling visible. Characters stay dry (W8). Far proxies darken a bit.
- `surface debug porosity`: metal and polish black, stone, earth, plaster white. `--off wet`: exactly the dry look.
- Status: awaiting owner (built 2026-11-05).
