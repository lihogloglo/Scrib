# Visual checks: weather-sky

Open visual checks: weather, fog, clouds, sky, night sky, haze (WE, FG, SK, NS). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Distant shadows (superfargoth `weather/distant-shadows.md`, "Near-term visual fixes", bug A)

The shadows session (2026-10-12) moved the sun's cascades to 504 m (2048² after shadows2; 4096² cost +1.4 ms)
and added a terrain far shadow past them. Screenshots: `shots\shadows\`
(`before_*`, `after_*`, `north_1830_on/off.png`). Cost measured +2.5 ms at
1440p (over budget; being cut).

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
- Owner 2026-11-15 (repeated, also RO-16 2026-11-06): the white moon (Secunda) is still never visible. Undeferred: known bug, cause not diagnosed.
- Fix 2026-11-17 (NS-3): cause = our invented moon path kept Secunda low (4.6° at 21:30 on days 0 and 168, never above 40°),
  behind Balmora's valley sides; the disk and its 16 textures were fine. Now OpenMW's moon model drives both moons (rise
  1.2 h later each day on a 24-day cycle, ~20 h arc, shown 15:00-07:00 and only once ~30-40° up; NW rise, overhead, SE
  set; OpenMW sizes 21°/9°). Secunda shows every night of the cycle, 4-13 h a night. Look for: `scene ns-moons-full`
  (day 168, 01:00): both full in the north-west, 70° and 52° up; `scene ns-moons-quarter` (day 162, 21:30): look
  straight up, both half-lit near the zenith; new game's first night: Masser from ~22:00 NNW, Secunda ~23:40 NW, each a
  plain star-blocking disk first, its texture fading in as it climbs. Days only advance with the clock running (P/T;
  `[`/`]` stay in the same day). Status: awaiting owner.
- Owner 2026-11-17 (Balmora from 22:00, clock running): Masser only appears ~02:30, already high; near the horizon its
  disk is invisible but its light (the halo at that spot) shows; no Secunda. Cause (code, not render order): the disk's
  opacity is OpenMW's `earlyMoonShadowAlpha` (`mw_weather/src/moons.rs:457`): hidden below `Fade End Angle` (Masser 40°,
  Secunda 30°), plain disk up to 50°; the moon's light and atmosphere halo (`sf_sky_bevy` moon light) ignore that alpha.
  Status: failed (disk/halo mismatch; Secunda not seen). Design question to the owner: moons from the horizon (modern)
  vs OpenMW's fade (classic preset). Owner 2026-11-17: visible from the horizon (DECISIONS.md); OpenMW fade only for Classic.
  Next slice: drop the horizon fade from the modern moon alpha (keep the daytime fade), check the halo and disk agree; and
  print Secunda's elevation/alpha/phase over the run's night (why it was not seen).
- Fix 2026-11-17 (NS-3b, `MoonHorizon::Rise`, dev build): moons show opaque and textured from their rise (0.5° edge ramp),
  dimmed and reddened by the atmosphere like their glow; daytime fade unchanged. Measured: on clock day 172 Secunda was below
  the horizon until ~02:25 (so not seen in the run above); the modern schedule differs from OpenMW's by hours on some nights
  (Masser ~55° at 22:00 that night instead of 8°). Check: `view "Balmora" --day 173 --hour 3.5 --weather clear`, clock
  paused (P), fly up with the free camera (F11) to clear the valley: Secunda low in the north-west (~10°), its disk and glow in
  the same place, a smaller, third-quarter moon; Masser near the zenith. Then let time run (T) ~1 h: Secunda climbs smoothly,
  never jumps or pops. Also `scene ns-moons-quarter` (Secunda high, half lit). Status: awaiting owner.
- Owner 2026-11-17 (NS-3b run, Balmora day 173 03:30): Secunda finally seen, very faint (the night reads too bright, a
  "day for night" look). Masser's glow is white, should take the texture's reddish colour; Secunda has no glow at all.
  Cause (code): the glow is Bevy's atmosphere scattering the single `MoonLight` (`sf_sky_bevy/src/lib.rs:501`), which
  points at the brightest moon only (`sf_sky/src/sky.rs:361` `moonlight`) and is coloured by the weather's moonlight hue,
  not the moon. Status: moonrise confirmed; glow failed (colour, Secunda); night brightness logged (separate).
- Fix 2026-11-17 (NS-3c, dev build): each moon draws its own small glow in the night pass (as the sun glare), hue = its phase
  texture's alpha-weighted average (a texture mod changes it), strength = its own light at this phase x transmittance
  (`sky.moon.glow.strength` 0.04, `sky.moon.glow.size` 0.5 disk radii: Masser ~4.4°, Secunda ~1.9°). Ground light, its
  colour and the atmosphere unchanged, so the wide white halo round the brightest moon remains (removing it needs a vendored
  flag that also drops all moonlit sky brightening: owner decision). Check: `view "Balmora" --day 173 --hour 3.5 --weather
  clear`, GPU timers on: a reddish glow hugging Masser, a smaller pale one round Secunda (low NW), both soft, no ring or
  hard edge, gone when a moon sets; the ground light's colour unchanged. Status: awaiting owner.
- Owner 2026-11-17 (NS-3c run): glows alright, reddish hue there; they sit behind the moons (weird, but pretty: keep for
  now, maybe fix later for realism); the white atmosphere halo round Masser is fine (no vendored flag). New, not fixed:
  Masser shows a black circle in its middle, smaller than the texture (size mismatch; source unknown, maybe the `occluder`
  0.85 disk or the glow edge vs the texture's disk). Status: glow confirmed with follow-up; black circle = known bug.

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
- Owner 2026-11-11: also clouds. Flying high, a cloud between the camera and the ocean does not show (same family, H3).
- Status: known bug: not diagnosed (owner pass 2026-10-30).

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

### FG-8: Weather fog vanishes at night (owner report)
- Kind: game location. Foggy weathers (fog, overcast, ash), compare 20:00 and 01:00 (`--hour`).
- Seen 2026-10-30: in some weathers the fog seems to disappear at night.
- Suspect (not tested): fog in-scatter scales with sun/moon light, so at night it goes dark and reads as no fog;
  the extinction (distance hiding) should stay.
- Status: known bug: not diagnosed (owner pass 2026-10-30).

### FG-10: No visible god rays at sunrise in fog (owner report)
- Kind: game location. `view "Balmora" --hour 6.25 --weather foggy` (also `--weather clear`), face the low sun with
  buildings or the ridge between you and it.
- Seen 2026-11-02 (after the FG-9 fix): no shafts at all, though froxel fog with CSM, terrain and cloud shadow terms
  is built (`froxel_inject.wesl:156-165`, fog.md "Froxel fog as built"). Suspects (not tested): froxel slices too
  coarse at 160x90x64 / 384 m, in-scatter too weak against the fog's ambient, shadow term not reaching the froxels.
- Status: known bug: not diagnosed (owner pass 2026-11-02).

### FG-12: Sun glow shows through terrain (owner report)
- Kind: game location (worldgen canyon, `view`, 18.09 h, High + DLSS). Look up a canyon at the low sun hidden behind the
  ridge: a round warm glow sits on the shadowed rock face, centred on the hidden sun (ShareX `bevyrrowind_VAW8Nx8GCT.jpg`).
- Not the sky glare: `night.wesl` draws it at the far plane, depth-tested. Candidates, all unshadowed forward-lobe
  in-scatter toward the sun: (1) analytic fog past the froxel range adds `key_seen * fog_phase` with no visibility
  (`cloud_composite.wesl:152`); (2) Bevy's aerial perspective (stock, no sun shadowing); (3) froxel shadow term ~1
  (`froxel_inject.wesl:221-226`), which would also explain FG-10. To split: live `flip fog`, then `--off froxels`.
- Owner 2026-11-22 (x10s, 18.09 h): glow stays with `flip fog` (our analytic + froxel fog off). Cause (2): Bevy's
  aerial perspective shadows the sun only by atmosphere transmittance and the planet (`vendor/bevy_pbr/src/atmosphere/
  functions.wesl:240`), never by terrain or shadow maps, so haze in front of shaded terrain glows with Mie forward scatter.
  Fix direction: shadow the aerial view LUT's single scatter (Hillaire 2020; UE5 "Cast Shadows on Atmosphere"), e.g.
  with the terrain far shadow map, as a vendored patch. FG-10 stays a separate question.
- Fix built 2026-11-22 (ROADMAP 1.27 S1): the aerial view LUT shadows the sun with the terrain far shadow map. Same spot
  and hour: the rock face in front of the hidden sun stays dark (no round glow); the sky above the ridge and sunlit
  slopes keep their haze. A/B: restart with `--off aerialshadow` (old glow). Limit: rock past ~3.9 km from the map centre
  still glows (map edge fade).
- Owner 2026-11-22 (AS on): "not very convinced": shafts blocky, move with the camera like big animated blocks; a halo
  is still there, also where it shouldn't be. Wants a classic god-ray shader. AS now off by default (`--set
  sky.aerial.shadow=true` = on); next: screen-space light shafts + occlusion (ROADMAP 1.27 S3, spec aerial-shadow.md).
- S3a built 2026-11-22 (light shaft occlusion): same spot; the halo in front of the hidden sun should be gone, with a
  clean edge along the ridge line, and haze lit from the side unchanged. No visible rays yet (S3b). A/B: console
  `set sky.shafts.enabled false`; knobs `sky.shafts.*` (e.g. `occlusion`, `length`).
- Owner 2026-11-22 (S3a, 18.79 h, ShareX `bevyrrowind_2bohrMdsQH.jpg`): "godrays are nice now", but a very dark disc
  surrounded by glow sits on the cliff where the sun hides. Cause (code): `render_sky` multiplies the whole aerial
  in-scatter (Rayleigh, multiple scattering, all of it) by `1 - 0.95 (1 - o) lobe`; o = 0 at the sun's spot, so all haze
  goes (dark hole), and the HG(0.8) lobe (~15 deg) is narrower than the real Mie halo, which stays outside it (ring).
  Fix S3c: the aerial LUT stores the sun's single scattering apart; occlusion subtracts only that (no lobe); fog masks
  only its HG lobe, not its isotropic part.
- S3b built 2026-11-22 (light shaft bloom: bright sky and cloud gaps near the sun blurred toward it and added): rays
  spilling over the ridge and through cloud gaps when the sun peeks out. A/B `set sky.shafts.bloom false` (owner
  already approved S3a's dark shafts alone; bloom off by default if preferred). No rays? lower `sky.shafts.bloom_threshold`
  to ~1-2. The S3a dark disc is still there (fix waits on the architecture review).
- Owner 2026-11-22 (S3b): bad visually: super blocky, shimmers constantly even with the sun still, redundant with
  S3a's shafts. Bloom off by default (`set sky.shafts.bloom true` = on). Parked.
- A1-A3 built 2026-11-22 (architecture review: one per-pixel sun visibility from the cascades, far map and cloud shadows
  masks only the sun's single scattering in Bevy's haze and our fog's sun term; LSO now off). Launch
  `owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10s" --hour 18.09`, same canyon. Look for: no
  glow and no dark disc or ring on the rock in front of the hidden sun; the haze there keeps its sky-blue fill (only the
  sun's warm forward light goes); same with the sun just off the top or side of the screen; shafts in the haze/fog
  where the ridge shadows the air; no blocks or crawl when turning. A/B: restart with `--off sunvis` (old glow) or
  console `set sky.sun_vis.enabled false` + `set sky.shafts.enabled true` (the LSO rays). Limit: ridges past ~5 km.
- Owner 2026-11-23 (A1-A3, x10s 18.09 h): "It works, well done"; asks whether it costs FPS (cost A/B running).
- Status: confirmed (look); cost pending.

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
- Owner 2026-11-06 (after the SW-5/6 fixes): puffs still very bright at night (snow and ash); night look is the priority fix.
- Fix 2026-11-15 (bugfix-11-15): night glow = `puff_light` lit puffs with the top-of-atmosphere sun (no transmittance; ~130k lux
  after sunset), phase floor 0.25. Puffs now use Bevy `apply_pbr_lighting` (fake sphere normal): atmosphere, cascades, lamps,
  ambient, GI. Ash `KickLook::ASH` 0.05->0.25 m at opacity 0.25 (was 0.07->0.38 at 0.4). Look for: Skaal night `surface set snow 1`
  and Ghostgate night `surface set ash 1`, walk/run: faint moonlit or lamp-tinted puffs, not white; by day, shaded far side.
  Still open (later slice): shape/noise, soft-particle fade scaled with size, forward-scatter phase, two layers. Status: awaiting owner.
- Owner 2026-11-15 (after the fix): night puffs good, "much better"; likes that they drift with the wind. Status: confirmed with
  follow-up (look rework later).

### SW-8: Legs and bodies carve deep snow (surface-weather.md "S1 deformer capture"; owner go 2026-11-16)
- Kind: game location. `view "Skaal Village" --hour 12 --weather snow`, console `surface set snow 1`, P to pause the clock.
- Look for: wading leaves one continuous trench the width of the legs (not a row of prints), deeper only where legs go
  below the snow; crouch: deeper; an NPC walking past carves its legs; a lying body carves its shape; first person: the
  legs carve, no stray arm-shaped hole; running fast: no gaps in the trench. Snow 0.4 (thin): carves only as deep as the
  snow is. Log: `surface/deform_*` counters non-zero, no "surface deform: no ground height hook" line.
- Known: the 16 m window follows the tagged actor nearest the camera (an NPC between camera and player can take it);
  first-person arms and held items do not carve; gap-fill prints also puff. No berm or spray yet (S2, S4).
- Owner 2026-11-17 (Skaal, `--weather snow` 12:00; snow cover command not logged): "very bad": the trail made by the
  character is not centred on it; patches of deformed snow keep appearing next to the character ("still": maybe the
  W6 foot-stamp trail too). Diagnosis queued (trail-offset: origin/height/focus hypotheses, end-to-end tests). Status: failed.
- Owner 2026-11-17, run 2 (`surface set snow 1`, 1P and 3P alike; `surface debug trails`): foot prints (blue) and lips
  (red) sit correctly under the feet; the capture's groove (green, ring B) has leg width and follows the walk but is carved
  a few metres to the right of the character, roughly constant offset. Lead: capture <-> ring mapping (sun_view depth
  view vs the ring's frame). Status: failed (diagnosis running).
- Fix 2026-11-17 (trail-offset): Bevy's shadow pass ignores the requested viewport, so the capture was drawn at 2x over the
  512² target; it now projects over the whole target. Owner 2026-11-17 (run 3): "the trail is fixed". Status: trail position
  confirmed; crouch, NPCs, lying bodies, first person not yet reported.
- Owner 2026-11-17 (run 4, Skaal `surface set snow 1`): some little creatures (owner: "skraelings", likely rieklings) leave
  no trace in the snow. Not diagnosed. Lead: the capture is an orthographic depth view from below, but it draws only meshes
  under a `Deformer`, and `tag_deformers` (`mw_exterior_bevy/src/trails.rs`) tags only animators whose graph has `legs`; a
  creature rig without a legs entry is never tagged. Fix direction: tag every actor (creature) body, not by leg graph.
  Status: known bug (creatures), logged.

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
- Fix 2026-11-06 (ring/paving): the raise faded linearly over 20-24 m (a crest following the camera); it now eases out
  smoothly from ~18 m to ground level at ~43 m, and stands only where the snow shows. Look for: `surface set snow 1`, no
  edge or ridge near the player, the snow eases down to the ground at the far edge with no line in the lighting; `surface
  set snow 0.4` (P, let hours pass) on Skaal's paving: paving stays at ground level, only the snowy patches stand raised.
  Status: awaiting owner (trails vs prints, ash depth still open).
- Owner 2026-11-06 (after both fixes): resets still there (canopy shelter pops in/out near trees, see SW-5). Still only
  separate footprints: knee-deep characters should plough a continuous trail with their shins. Status: failed (resets,
  trails); ring edge and paving not reported.
- Logged, not checked: the rain occlusion map (`sf_precip_bevy/src/occlusion.rs`) likely drops casters outside every
  view the same way the shelter map did (fixed in 30319aa).
- Fix 2026-11-15 (bugfix-11-15, raise only): the raise was coverage x amount x depth with coverage at the vertex (cobble/grout  heights averaged to their mean), so drift patches rose stones and all. Now raise = depth x saturate((amount - top)/(1 - top)),  top = the layer's 90th-percentile height (baked, terrain layer table v4) mixed with drift: snow fills the grout first (shading),  the ground rises only once the stones are buried. Look for: a terrain road at `surface set snow 0.4`: snow in the grout, stones  and feet at ground level; `snow 1`: the full knee-deep raise with prints. Status: raise awaiting owner (shelter resets: SW-5).
- Owner 2026-11-16 (Skaal `surface set snow 1`): raise maybe: better (only snow rises), but it reads as an extruded mesh, a
  vertical wall with jagged edges along the drift border; maybe raises too much. Wants a smooth ramp at the edge. Status: raise maybe.
- Fix 2026-11-16 (SW-6, smooth ramp): the raise threshold amplified tiny changes into a wall (old worst slope 47-72°); now
  a smooth, slope-limited field: edges ramp over ~4 m and never stand steeper than 30° (snow's angle of repose); over stony
  ground at full snow it rises only part of the way (gentle undulation, answers "maybe raises too much"). Look for: Skaal
  `surface set snow 1`: drift and road edges are soft ramps, no vertical wall or jagged steps; `snow 0.4` on paving: still
  flat (snow in the grout only). Known left: roof drip lines still steep (SW-6b queued). Status: awaiting owner.
- Owner 2026-11-17 (Skaal, same run as SW-5 run 3): "the snow ramp is good". Status: ramp confirmed. Still open: shin
  trails (SW-8), ash depth, SW-6b drip lines.

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
- Owner 2026-11-06 (after both fixes): still jumps. Lead: shelter under trees (canopy) pops in and out with distance: walking
  toward a tree in snow, the ground under its leaves suddenly turns bare and depressed; step back and it suddenly refills.
  Status: resets failed (not fixed; see SW-6).
- Fix 2026-11-16 (bugfix-11-15): the shelter photo drew far trees as their sparse LOD level chosen by camera distance (walk  closer -> full leaves -> ground bare at once), with a lagging centre and no blend. Now the caster level follows the map's own  centre (camera-independent), centres snap to world texels, and a world-anchored exposure history eases changes over ~2 s  (keeps across a big_space recentre). Precip occlusion: casters behind the camera kept, centre snapped. Also cover amount  from the geometric slope (texture edges no longer bare through full snow). Look for: Skaal `surface set snow 1`, walk from  60 m to 3 m toward a tree and back: snow under the leaves stays or fades, never snaps; no scene-wide reset after 16 m; same in  Ghostgate ash under roofs/arches; rain under trees/roofs steady as you approach or back off. Status: awaiting owner.
- Owner 2026-11-16 (Skaal): reject. Changes are now eased (vertices move up and down smoothly), but cover still follows
  the camera: walking up to a tree, the snow under it noticeably sinks. Shelter must not depend on viewer distance. Status: failed.
- Fix 2026-11-16 (SW-5b): the cover amount came from the ground's normal tilted by the snow raise, which fades in with camera
  distance (18-43 m), so the raised walls round a canopy lost cover and the shelter lookup slid under the tree as you came
  near. The amount now reads the undisplaced ground; the raise only lights. Look for: Skaal Village `surface set snow 1`, walk
  towards a tree from ~50 m: the snow under and around the canopy must not change as you approach; raised snow walls keep their
  cover (no bare walls). Known left: the raise height itself may still settle slightly near the camera (separate slice).
- Owner 2026-11-16 (SW-5b, Skaal): still the same issue. Owner asks for a console view of the surface render targets (shelter, exposure, trail) to see under the hood; also: deformation should take the whole leg up to the knee, not only the feet; missing niceties: rim bulge round the trench, particles (RDR2 as reference). Status: failed.
- Fix 2026-11-16 (SW-5c): the raised snow (~0.45 m) existed only within ~43 m of the camera (faded in from 43 to 18 m) and
  its bury tops were filtered by the LOD's vertex spacing, so open snow rose round a sheltered patch that stayed flat as you
  came near. Now every terrain LOD out to ~400 m displaces the same world-fixed snow height (fade 384-576 m, sub-pixel).
  Look for: Skaal Village `surface set snow 1`, P to pause; walk to a tree from ~60 m and back: the snow height round and
  under it stays the same the whole way (no rising, no sinking); open snow far off stands raised too (look at hill edges).
  A/B: console `flip raise` (F1 help, "Snow and ash" line) toggles the raise alone (off = flat cover, same look). Known
  left: past ~64 m (shelter map edge) the ground under far trees reads open and raised until it enters the map.
  Status: awaiting owner.
- Owner 2026-11-16 (tex viewer, Skaal snow): `surface.shelter` redraws (about half the rate of the other maps) change the whole map: trees gain or lose shelter at each redraw; `surface.exposure` shows red moving with the player and blue patches flashing then fading at each update. Cause: casters pick their LOD by the map centre, which jumps 16 m per redraw (SW-5d in progress).
- Fix 2026-11-17 (SW-5d + SW-5e): (d) the shelter, canopy and rain-occlusion maps draw every tree/roof at its finest level
  whatever the camera or map centre (vendored patch `RenderViewLodDistances`), so a redraw 16 m on gives the same texels;
  (e) a second, far shelter map out to ±512 m (1 m texels, a fixed coarser level of each caster), blended in at 56-64 m, so
  trees far away shelter the snow under them too; the raise now ends where shelter is known (360 m High, 270 m Low).
  Look for: Skaal Village `surface set snow 1`, P to pause; walk to a tree from ~300 m and back: the snow under and round
  it never rises or sinks on the way; `tex show surface.shelter` while walking 100 m: trees no longer gain/lose shelter
  at each redraw (the map just slides); `tex show surface.exposure`: no blue flashes. Also rain under far trees: ground
  under the canopy stays drier at any distance. Status: awaiting owner.
- Owner 2026-11-17 (run 2, Skaal `surface set snow 1`): some trees' sheltered area is offset from the tree, sometimes
  not; the offset can update suddenly (a flat pit beside the player under a tree in the shot). Likely the same root cause
  as SW-8's offset (both are sun_view depth views); SW-5e far level / near blend also suspect. Status: failed.
- 2026-11-17: shelter offsets handed to the owner's separate shelter session (not this one). Lead from a stopped diagnosis
  (`.sessions/_archive/shelter-offset.md`): rigid held props are shelter casters, so a redraw stamps a sheltered patch where
  the character stood.
- Fix 2026-11-17 (shelter-audit, measured, not yet seen): the capture itself was being skipped. Bevy's light extraction
  strips the shadow tables from our capture anchors every frame and we put them back in no fixed order, so about every
  other redraw (and most snow-trail captures) drew nothing while the game moved on to the new map centre: every sheltered
  patch then sat a recentre step (up to ~20 m) off its tree or roof until a later redraw. All earlier SW-5 fixes eased or
  re-levelled that symptom. Now ordered (`sun_view.rs`), with a log warning if a capture is ever dropped; character parts
  (heads, helmets, weapons, shields) no longer cast shelter, so no sheltered pit is left where someone stood. Numbers
  (`surface probe`): the area round the SW-5 video's tree reads identical from every map centre and view direction.
  Check: `view "Skaal Village" --hour 12 --weather snow`, `surface set snow 1`, run between the trees NE of the village
  (around world -2106, -3044) for a few minutes, in and out, turning: the bare patch under each tree stays under that
  tree, same shape near and far, no patch appears or leaves as you approach, none trails behind you or NPCs. Also SW-8
  again (trails continuous, no missing stretches). Known left: a patch may change shape slightly ~40-60 m out (near map
  taking over from the far map); the 2.4 s easing is still in (proposal to remove it now that captures agree: ask).
  Status: awaiting owner.
- Owner 2026-11-17 (run 3, Skaal `surface set snow 1`, dev build 02:37): "snow bug has disappeared". Status: confirmed
  (shelter offsets and resets gone). Open: ash in Ghostgate not rechecked; easing removal still to ask.

### WE-13: Local storm seen from outside (CW-1, distant-weather.md "CW-1 Storm shape"; follows WE-10)
- Kind: game location. `bash bevyrrowind/tools/owner_launch.sh scene we3-ridge`, then BRP `bevyrrowind/set {"weather":
  "thunderstorm", "region": "Ascadian Isles Region", "transition_hours": 0}` (look east/south-east) and `"ashstorm"` in
  `"Ashlands Region"` (look north); still camera, then pan slowly. A/B: relaunch with `--off stormshape` (old region column).
- Look for: a storm cell with a ragged, cloud-shaped rain curtain hanging under its thick clouds, thinner toward the
  ground, gaps where the deck is thin; a narrow soft border (~64 m, wavering) instead of a straight wall; no
  ground-to-sky pillar; no shimmer at the curtain's edge, still or panning.
- Must not: fog scenes FG-7, FG-11, WE-11, WE-12 changed (Bitter Coast fog from the ridge, fog over sea, deck from above);
  rain doubled or popping when walking into the storm.
- Status: partial (owner 2026-11-12, `scene we3-ridge`, Ascadian Isles thunderstorm forced over BRP): the rain curtain is
  barely visible; the cloud is still a cylinder (its base shape extruded straight up); a tiny shimmer exactly on the
  horizon line; walking in, the rain appears suddenly at a threshold (on/off), worse than before CW-1. A/B `--off stormshape` (old path): rain fades in smoothly when
  walking in (better); the shimmer is NOT at the storm edge but exactly on the horizon line where no fog is behind, in
  both paths; cloud extruded upward in both (not an exact cylinder); curtain hard to see and not reaching the cloud base
  in both; shimmer behind the curtain (old path, seen before too). CW-1 aimed at an edge flicker the owner does not see.
- Fix S2 (2026-11-13; distant-weather.md "S2 fixes as built"): the far edge is pre-filtered to the march's sample spacing
  and is the map's own 256 m band within ~128-256 m (the near FX's ramp); the curtain reaches the cloud's visible base;
  rain falls under ordinary thick storm cloud, not only cells. Same launch and BRP as above (Ascadian Isles thunderstorm),
  then walk slowly into the storm and back out. Look for: the rain fading in smoothly as you walk in, no on/off threshold,
  as smooth as `--off stormshape`; from outside, a curtain clearly visible and meeting the cloud base (no gap of sky
  between rain top and cloud); dry gaps still where the deck is thin. Status: confirmed with follow-up (owner 2026-11-13, we3-ridge thunderstorm: rain transition smooth walking in, the curtain is visible now, shimmer of objects behind the curtain much better with some left (later)).

### WE-15: Storm cloud shape (S3, clouds.md "Storm cloud shape"; follows WE-13 "cylinder")
- Scene: `scene we3-ridge` at 14:00, then BRP `bevyrrowind/set {"weather":"thunderstorm","region":"Ascadian Isles Region",
  "transition_hours":0}`; look east/south-east. Still camera, then a slow pan. A/B: relaunch with `--off cbshape` (old profile).
- Look for: towering, domed storm cells whose tops taper toward their rims, flat anvil shelves spreading near the top of
  the big cores, a ragged (wispy) base, sides that slope down at the region edge; not the footprint extruded straight up.
- Must not: fair or overcast decks changed (a calm region beside the storm, then `"weather":"cloudy"` / `"overcast"`);
  FG-7, WE-11, WE-12 looks changed (fog behind ridges, fog over sea, deck from above).
- Status: partial (owner 2026-11-13: shimmer better, but the clouds "still look goofy and a bit weird"; a bigger cloud-shape pass later).

## Exposure and tonemapping (superfargoth `exposure-tonemap.md`)

### EX-1: Night darkness, three arms (E0, launch flags only, no build)
- Game location: `view "Balmora" --day 173 --hour 3.5 --weather clear`, clock paused (P); look at the street, the
  hills and the sky (Secunda low NW, Masser high). Arms, one launch each: today; `--night-ambient 2` (ground, sky glow
  and moonlight 1 EV darker, lamps unchanged); `--exposure-floor 4.5` (everything 1 EV darker, lamps included);
  `--night-ambient 2.5 --exposure-floor 4` (between).
- Look for: which reads as night and not "day for night", while streets stay readable (LT-1, 1 nit, was too dark);
  lamp-lit walls not too dim (lit-look rule); Secunda easier to see. Status: awaiting owner.
- Owner 2026-11-17 (arms 1 and 2 seen, arm 3 cancelled: build running): `--exposure-floor 4.5` looked nice, but the sky
  stays a clear daytime blue. Suspects (code): the atmosphere's Rayleigh scattering of the artistic moonlight (31.4 lx,
  ~125x a real full moon, hue `Sun Night Color` 59,97,176) and the N2 airglow; split live with `flip moonlight` and
  `flip night_sky_glow`. Status: floor 4.5 kept as the candidate; blue sky open (spec E1b).
- Owner 2026-11-17 (arm 2, flips, 4 shots): the moonlight makes the sky blue (owner likes its blue on the land, not in
  the sky); the night sky glow makes the sky brighter than the land; both off: a dark grey sky with a pale horizon band.
  Next (E1b): moon lights the land at full strength but scatters less in the sky (per-light sky-scatter scale), glow
  ratio lowered until the sky sits under the land. Status: diagnosed, fix pending.
- Fix 2026-11-17 (E1 + E1b, dev build needed): floor 4.5 by default; live `set sky.moon.sky_scatter <0..1>` (0.15: moon
  lights the land fully, the sky a little; also shrinks the white halo round Masser), `set sky.night.glow_ratio <x>`
  (0.1 today; lower until the sky sits under the land), `set sky.night.ambient_nits`, `set exposure.min_ev100`.
  Check: same view, clock paused; land keeps its moonlit blue, sky deep and under the land, Secunda readable. Say the
  values you keep. Status: awaiting owner.

