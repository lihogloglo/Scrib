# Visual checks: closed

Confirmed, retired and decided checks, moved out of `../visual-checks.md` on 2026-10-19 (phase0).
Grep by ID; reopen an entry by moving it back.

## Far field (superfargoth `far-field.md`, "Measured on Morrowind")

Screenshots: `D:\Gamedev\Bevyrrowind\shots\farfield\` (`balmora_a*`,
`redmtn_view*`, `summit_vivec*`, `ebonheart_night*`, `handoff_forest*`).

### FF-1: Balmora horizon
- Kind: game location. Run: `.\bevyrrowind scene balmora`
- Look for: town and hills drawn out to the horizon; no holes, flicker or
  seams around the 400 m handoff; no streaming pop while you fly around.
- A/B: `.\bevyrrowind scene balmora --off farfield` (the old far ring: the
  horizon stops at ~1 km and the frame costs ~2.7x).
- Status: confirmed by owner 2026-10-19 (round 1, scene balmora): no holes, drawn to the horizon. Note: brief flashes in the distance while moving, likely shadows (logged as SH-11, Phase 1).
- Source: farfield-lod.

### FF-3: Red Mountain is solid
- Kind: game location. Run: `.\bevyrrowind scene redmtn-view`
- Look for: the mountain is one solid surface; distant objects sit on it.
- A/B: `--off farfield` shows the old see-through gaps (`redmtn_view_offfarfield.png`).
- Status: confirmed by owner 2026-10-19 (round 2, scene redmtn-view).
- Source: farfield-lod.

### FF-4: Summit to Vivec
- Kind: game location. Run: `.\bevyrrowind scene summit-vivec`
- Look for: Vivec, Balmora and the coast readable at 2+ km; no pop when panning.
- A/B: `--off farfield`.
- Status: confirmed by owner 2026-10-19 (round 3, scene summit-vivec): everything readable from afar, no pop on pan.
- Source: farfield-lod (camera added by the scenes session; the original shot's camera wasn't recorded).

### FF-5: Distant lights at night
- Kind: game location. Run: `.\bevyrrowind scene summit-vivec-night`, then
  `.\bevyrrowind scene ebonheart-night`.
- Look for: towns' lights as warm points/glows at the right places, not
  oversized; they fade in from dusk (press `[` a few times to 19:00, then T).
  Ebonheart: Vivec's cantons lit across the water, no flicker flying at them.
- A/B: `--off farfield`.
- Status: confirmed by owner 2026-10-19 (round 6, scene summit-vivec-night): cities look cool, no flicker or pop. Bug seen: GF-1 (Ghostgate far light sprites).
- Source: farfield-lod.

### FF-6: Impostor trees at the handoff
- Kind: game location. Run: `.\bevyrrowind scene handoff-forest`
- Look for: trees at 350-450 m keep their shape, size and brightness when
  they switch to impostors (F5 shows where: white to green).
- A/B: F5.
- Status: confirmed by owner 2026-10-19 (round 4, scene handoff-forest): hard to spot, the illusion works.
- Source: farfield-lod.

### FF-7: No pop in fast flight
- Kind: game location. Run: `.\bevyrrowind scene redmtn-view`, hold Shift+W
  (80 m/s) toward the mountain for ~20 s.
- Look for: nothing appears late or vanishes at the 400 m handoff; hitches are
  a perf issue (ROADMAP "Next up" 1), note them separately.
- Status: confirmed by owner 2026-10-19 (round 2) except the Ghostfence field, which vanishes at distance and pops back close (GF-1, known).
- Source: farfield-lod.

## Distant shadows (superfargoth `weather.md`, "Near-term visual fixes", bug A)

The shadows session (2026-10-12) moved the sun's cascades to 504 m (2048² after shadows2; 4096² cost +1.4 ms)
and added a terrain far shadow past them. Screenshots: `shots\shadows\`
(`before_*`, `after_*`, `north_1830_on/off.png`). Cost measured +2.5 ms at
1440p (over budget; being cut).

### SH-1: No shadow pop on approach
- Kind: game location. Run: `.\bevyrrowind scene shadow-flyin` and fly west
  into Balmora; also `scene shadow-approach-450` vs `scene shadow-approach-280`.
- Look for: building shadows present at every distance below 440 m; no
  shadow pop between 450 m and 280 m. Object shadows now fade out (dithered) over 430-504 m instead of ending on a line (shadows2; see SH-6).
- A/B: F2 (sun shadows off); `--off shadowtune` (old cascades: shadows end
  much closer and objects gain them as you approach).
- Status: retired 2026-10-19 (phase0 triage): superseded by SH-7/8/10 cascades; shadows redone in Phase 1.
- Source: shadows (check 3).

### SH-2: Near shadow quality
- Kind: game location. Run: `.\bevyrrowind scene shadow-flyin`, land in a street.
- Look for: crisp, stable shadows close up (first cascade 10 m, 2048²): no
  blockiness, shimmer or acne when walking.
- A/B: `--off shadowtune`.
- Status: retired 2026-10-19 (phase0 triage): superseded by SH-7/8/10; Phase 1.
- Source: shadows.

### SH-3: Terrain far shadows
- Kind: game location. Run: `.\bevyrrowind scene ridge-shadows-north` (the
  clearest), `scene ridge-shadows-evening`, `scene ridge-shadows-morning`.
- Look for: long ridge shadows on distant slopes; no straight shadow line
  across the terrain near 400-500 m; shadows move correctly with `[` / `]`.
- A/B: F3 live; `--off farshadows` or `--off farshadows,shadowtune` (pre-session state).
- Status: retired 2026-10-19 (phase0 triage): superseded; terrain far shadows rechecked in Phase 1.
- Source: shadows (check 2).

### SH-4: Balmora sunset shadows
- Kind: game location. Run: `.\bevyrrowind scene balmora-sunset-shadows` (1440p).
- Look for: hills past ~400 m carry shadows; no straight line near 400-500 m.
  Baseline `shots\shadows\before_balmora_1830.png`.
- A/B: F3; `--off farshadows,shadowtune`.
- Status: retired 2026-10-19 (phase0 triage): superseded; Phase 1.
- Source: shadows (check 1).

### SH-5: Far objects in terrain shadow
- Kind: game location. Run: `.\bevyrrowind scene balmora-sunset-shadows`.
- Look for: far trees and buildings in a shadowed valley are shadowed like the terrain around them.
- Status: retired 2026-10-19 (phase0 triage): superseded; Phase 1.

### SH-6: Dithered cascade end (shadows2)
- Kind: game location. Run: `.\bevyrrowind scene ridge-shadows-evening` and `scene balmora-sunset-shadows`;
  or `cargo dev view "Balmora" --hour 18.5 --resolution 2560x1440 --camera "450,+60,0,0,0,0"`.
- Look for: where long object/proxy shadows reach ~430-504 m from the camera they thin out through a
  fine 4x4 dither and hand over to the terrain shadow; no straight line, no visible dither crawl when
  the camera moves slowly; far proxies and impostor trees in shadowed valleys are dark (not glowing lit);
  no blotchy false shadows on lit flat ground far away (the 24 m shadow map; bias 6 m).
- A/B: `--off farshadows` (hard cut at 504 m, far field lit), `--off farshadows,shadowtune` (pre-session).
  F3 toggles only the terrain part live.
- Status: retired 2026-10-19 (phase0 triage): superseded by 250 m cascades (SH-8); Phase 1.
- Source: shadows2.
- Source: shadows (check 4).

### SH-7: Contact shadows and shadow bias (shadows F1 + F2)
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --camera "-143.5,4.4,1020.0,-153.5,2.4,1010.0" --hour 15 --weather clear`
  (street by the Census Office, NPCs at 5-15 m, houses at 20-30 m); then `view "Balmora" --camera "-280,+12,200,-295,+1,180" --hour 15 --weather clear`
  (pots under the arch ~25 m, NPCs at 30-40 m); then both at `--hour 7` (grazing sun).
- Look for: feet, pots, crates and column/wall bases darkened where they touch the ground at every distance
  (no floating objects); no dark halos beside thin objects or at depth edges; no shadow acne (stripes, speckle)
  on walls and ground at 07:00, and the 6a "blocky half-transparent outline" at grazing light gone or smaller;
  shadows not detached from their casters (peter-panning).
- A/B: `--off contactshadows` (contact shadows only); `--off shadowtune` (Bevy's 1.8-texel normal bias + old
  cascades); both together = before this slice. `--quality low` = Hardware2x2 filtering, no contact shadows.
- Status: **confirmed by owner 2026-10-19** (round 2 A/B vs all four off at Seyda Neen: both good, new maybe slightly better; differences subtle). Was: (shf12 saw only small differences at 15:00; GPU cost not yet benchmarked).
- Source: shf12 (shadows F1 + F2).

### SH-8: Sun cascades to 250 m (shadows F3a)
- Kind: game location. Run: `.\bevyrrowind view "Balmora" --camera "-280,+12,200,-295,+1,180" --hour 15 --weather clear`,
  then a street view down the Odai with houses at 30-150 m (fly with the free camera), then `view "Seyda Neen"
  --camera "-143.5,4.4,1020.0,-153.5,2.4,1010.0"`; all at `--hour 15` and `--hour 7`. Then fly slowly from ~400 m
  toward a Balmora building.
- Look for: shadows of NPCs, awnings, rails and crates at 30-150 m visibly sharper than with `--off shadowsplits`;
  no shimmer/crawl of shadow edges while the camera moves or turns (Bevy texel snapping); no visible line where
  cascades meet (~10, 27, 75 m); the cascade end at ~212-250 m fades (dithered) rather than cuts. Known gap until
  F3b (cached far map): objects past 250 m cast no shadows, and a shadow cast onto a building at ~250 m may end on a
  line there.
- A/B: `--off shadowsplits` (A1 4 cascades to 504 m); `--quality low` (3 cascades to 200 m).
- Status: **confirmed by owner 2026-10-19** (round 2 A/B vs all four off at Seyda Neen: both good, new maybe slightly better; differences subtle). Was: (shf3d 2026-10-19: far map past 250 m now drawn (log: casters/sets per redraw) and received by terrain + far proxies; NIF statics past 250 m still unshadowed. shf3b: the 15:00 vista does show sun shadows, backlit, and is identical with `--off shadowsplits`; shf3 agent shots at 15:00/07:30 were inconclusive: near view nearly identical, the
  Balmora vista was backlit; GPU ≈ 0, dev frame −3.5 ms at Seyda Neen).
- Source: shf3 (shadows F3 step a).

### SH-10: Soft contact-hardening sun shadows (shadows F5)
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --camera "-143.5,4.4,1020.0,-153.5,2.4,1010.0" --hour 15 --weather clear`
  (tree shadow across the street); compare `--off softshadows`, `--quality medium` (Gaussian PCF) and `--quality low`.
- Look for: shadow edges crisp where the shadow touches its caster (trunk base, wall foot, feet) and softer toward the far
  tip; no banding; faint dither noise is expected on raster (no TAA; clean with `--dlss`). Soft size is
  `SOFT_SHADOW_SIZE` in exterior.rs (texels, 10): too soft or too sharp? (6 barely visible, 14 fades tips strongly).
- Cost: within noise at Seyda Neen (22.6 vs 23.2 ms median, dev build).
- Status: **confirmed by owner 2026-10-19** (round 2 A/B vs all four off at Seyda Neen: both good, new maybe slightly better; differences subtle). Was: (shf5 saw the tree shadow tip fade softly at 14, near-identical at 6).
- Source: shf5 (shadows F5).

### SH-17: Shadow reset A2 (lamp maps captured during the door fade)
- Kind: game location. Run (release): `.\bevyrrowind scene door-census` at night (`--hour 22`): Space out to Seyda Neen,
  Space at the office door back in. The log line "door travel took ... s of it for gates" gives the extra black time.
- Look for: on arrival inside, the lamps' shadows are there from the first visible frame: no flicker or shadows
  switching on as the fade lifts (SH-15's "flicker at first"); the extra black time is short (~0.3-0.5 s, capped 1.5 s).
  Outdoors the first visit may hit the 1.5 s cap (shader compiles): any flicker left there is expected.
- Also (doorwalls fix): back inside, everything in view draws on the first frame: walls, floor, rug and the NPCs,
  no fog-colored holes or silhouettes (were: main-pass pipelines still compiling). Inside, the gate may hit the 1.5 s cap
  while the exterior's pipelines still compile (log `pipelines: ... compiling`).
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): "everything works".

### SH-18: Shadow reset B1 (vendored Bevy shadow patch under the lamp cache)
- Kind: game location. Run (release): `.\bevyrrowind scene door-census --hour 22`, then Balmora at night
  (`view Balmora --hour 22`, walk the streets by lamps and torch-bearers). A/B: `--off shadowcache` = patch alone.
- Look for: lamp shadows as in SH-15/17 (all 8 census lamps, NPCs casting while walking); no flicker; no stale
  shadows: a walker's shadow follows it and leaves nothing behind when it walks out of a lamp, a door that opens
  updates its shadow. Same with `--off shadowcache` (slower, should look identical).
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): "everything works".

## Ghostfence (superfargoth `weather.md`, bug B)

### NP-1: Lamps hand off to the far field without pop or double draw
- Kind: game location. Run: `.\bevyrrowind scene lamps-handoff`, then fly
  back (Shift+S) until the town is 400-600 m away; fly in again.
- Look for: lamp and torch models (runtime NIF path) dither out into their
  merged L1 copies around 400-460 m and back in when you return; no lamp is
  drawn twice (NIF + L1) past the handoff, none pops, none appears late after
  its cell reveals. 21 animated models (tiki torches, torches, braziers,
  candles, lanterns, fires) now have rest-pose far copies (`mesh.rest`).
- A/B: F5 (L1 green: the lamps should turn green past the handoff).
- Status: confirmed by owner 2026-10-19 (round 5, scene lamps-handoff): fade sleek, not noticeable, no pops (green tint not readable at night).
- Source: nifpop.

## DLSS (superfargoth `sky-lighting.md`; `.sessions/dlss.md`)

Screenshots: `D:\Gamedev\Bevyrrowind\shots\dlss\`.

### DL-1: Super Resolution at 720p in motion
- Kind: game location. Run: `.\bevyrrowind scene dlss-run`, run forward
  along the street (Shift+W) and pan across foliage.
- Look for: smear or ghosting on alpha-tested leaves (DLSS quality renders
  480p internally); compare F4 off vs quality vs performance.
- A/B: F4.
- Status: decided 2026-10-12 (--dlss auto).
- Source: dlss.

### TA-1: TAA as the base temporal resolve (ROADMAP 1.3)
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --hour 11 --weather foggy --dlss taa`, fly
  around; then `--weather ashstorm` far away (WE-2 grain) and Balmora at a grazing sun (SH-12 edge shimmer).
- Look for: fog, far bad weather and soft shadow edges smooth instead of grainy; no ghost trails behind moving
  actors, rain or the ship's sail (translucents write no motion vectors yet, RO-6); not blurrier than DLSS Q.
- A/B: F4 cycles off (MSAA) / taa / DLSS modes. Agent shots: `shots/p1-aa-off.png`, `shots/p1-aa-taa.png`.
  Cost (1440p native, Balmora): TAA 15.4 ms vs MSAA 18.5 (TAA pass 0.39 ms; no MSAA saves more).
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): "very good". Default: DLSS auto where available, else TAA; Low: MSAA.

## Terrain (superfargoth `terrain.md`, "Verification")

### TR-1: No cracks or popping
- Kind: game location. Run: `.\bevyrrowind scene terrain-valley` and
  `.\bevyrrowind scene terrain-coast`; fly low and fast.
- Look for: no cracks between patches, no visible LOD pop (F6 shows LOD
  colors morphing smoothly, F7 the triangle edges); coast stable against
  the sea plane.
- A/B: F6, F7.
- Status: confirmed by owner 2026-10-19 (round 7, scene terrain-valley): no popping, no cracks.
- Source: terrain session (2026-10-11).

### TR-2: Splat blending
- Kind: game location. Run: `.\bevyrrowind scene terrain-valley`, look at
  grass/rock/road transitions from 5-50 m.
- Look for: soft, natural boundaries; no texel grid or diamond pattern.
- Status: confirmed by owner 2026-10-19 (round 7): ground texture transitions good.
- Source: terrain session.

## Clouds (superfargoth `weather.md`, "Clouds"; `.sessions/clouds.md`, `.sessions/clouds2.md`)

Volumetric clouds are on by default in exteriors. `--clouds <name>` picks a genus (clear,
fair, cumulus, stratocumulus, altocumulus, altostratus, cirrus, overcast, storm) or a
Morrowind weather (cloudy, foggy, rain, thunderstorm, ashstorm, blight, snow, blizzard;
each with its `Cloud Speed`); default fair. `--off clouds` drops them (the A/B). In the
viewer, F10 toggles them and C cycles the nine genera live. Hours: `[` `]` or `--hour`.
Synthetic scene (no game data): `cargo run --release -p sf_atmos_bevy --example clouds --
--preset <genus> --scale 0.5 --view sky|sun|shadow|up` in superfargoth.

S1 rework (2026-10-13): cloud size and height both at half real scale (fair cumulus a few
hundred meters wide, base 600 m), clouds move (Morrowind `Cloud Speed` x 10 m/s: cloudy
20 m/s) and re-form, cloud shadows now actually reach the ground (the light texture was
silently disabled), and the sun is fully hidden behind thick cloud.

### CL-1: Cloud genera and motion
- Kind: game location. Run: `.\bevyrrowind scene clouds-cumulus`, `clouds-stratocumulus`,
  `clouds-altocumulus`, `clouds-altostratus`, `clouds-cirrus`, `clouds-ashstorm`,
  `clouds-redmtn-noon` (fair), `clouds-redmtn-sunset`; in any of them C cycles the genera.
- Look for: nine distinct skies. Fair: small separate heaps, about a dozen across the
  island (not one cloud over all of it). Cumulus: fewer, taller towers. Stratocumulus: a low
  lumpy deck with gaps. Altocumulus: many small cloudlets high up. Altostratus: a thin even
  sheet, the sun a smudge. Cirrus: high white streaks along the wind. Overcast: a closed
  grey deck. Storm: dark towers with breaks and a high veil. Motion: in real time (no
  T time-lapse) clouds visibly drift (a cloud crosses the sky in about a minute at cloudy
  speed; ashstorm several times faster) and change shape while they move; cirrus moves
  faster than the low clouds (parallax). Sunset: warm undersides, brighter tops.
- A/B: F10 (or `--off clouds`): the sky without clouds.
- Calibrate: `CLOUD_WORLD_SCALE` 0.5 (sizes and heights), `CLOUD_SPEED_MPS` 10 (exterior.rs).
- Known: lumpy/soft tops rather than crisp cauliflower billows (cumulus towers read as soft
  vertical columns); no anvils on storms; weather does not change by itself yet (M6). Fixed
  (clouds3), re-check: the faint horizontal banding in thick decks (step count jumped with the
  ray span); cumulus coverage lowered to 0.34 (agent shots 2026-10-13: from the scene camera
  the near bases still fill the top of the view; judge whether there is enough sky between towers).
- Status: retired 2026-10-19 (phase0 triage): clouds now weather-driven (WE-0/1/3 confirmed); recheck in Phase 4.
- Source: clouds, clouds2.

### CL-2: Clouds composite with atmosphere, far field, terrain and the sun
- Kind: game location. Run: `.\bevyrrowind scene clouds-sun-behind` (also
  `--clouds overcast`), `.\bevyrrowind scene clouds-redmtn-sunset`.
- Look for: where thick cloud covers the sun, the disk and its bloom are gone completely
  (no disk drawn over a cloud); at thin cloud edges the sun burns through with a bright
  rim. Distant clouds fade into the same haze as the far mountains; Red Mountain and
  far-field proxies cut sharply into clouds behind them; clouds never draw on the sea or
  below the horizon.
- Agent check (clouds3, release): stratocumulus at 17:30 hides the sun completely, no disk or
  bloom; with `--clouds fair` the disk shows in a gap. Also look whether a dithered checker
  pattern on a dead tree left of center (Seyda Neen camera) goes away with F10 (it showed in
  both shots; probably foliage alpha, not clouds).
- Status: retired 2026-10-19 (phase0 triage): superseded by WE3/WE4; Phase 4.
- Source: clouds, clouds2.

### CL-3: Cloud shadows on the ground
- Kind: game location. Run: `.\bevyrrowind scene clouds-shadow-edge-noon`, then
  `.\bevyrrowind scene clouds-redmtn-noon`.
- Look for: dark patches under the clouds with a visible edge (a blur of tens of meters,
  not hundreds), drifting with the clouds over terrain, buildings and far field. Standing
  under a cloud edge at noon, the edge line is visible on the ground nearby. Under a cloud
  the ground loses its direct sun (stays lit by the sky, so not black). F10 off removes them.
  Walking or flying, shadows stay fixed to the clouds (they never swim or step with the
  camera), and no straight seam line runs through a shadow edge (the map refreshes one band of
  rows per frame; a seam would mean a band lagging).
- Agent check (clouds3, release): shadow patches with edges on the hills east of Balmora at
  noon, gone with `--off clouds`.
- Known: past ~8 km the map fades to the average cover; no moon shadows. Fast decks (ashstorm)
  may lag a band by up to a texel (16 m).
- Status: retired 2026-10-19 (phase0 triage): superseded by WE3/WE4; cloud shadows recheck in Phase 4.
  Bevy ignored the light texture without the `pbr_clustered_decals` feature).
- Source: clouds, clouds2.

### CL-4: Night clouds
- Kind: game location. Run: `.\bevyrrowind scene clouds-redmtn-night`
- Look for: faint moonlit clouds that occlude the night sky; no glowing grey sheet;
  cirrus faint, not glowing.
- Status: retired 2026-10-19 (phase0 triage): superseded; Phase 4.
- Source: clouds.

### CL-5: Overcast and storm
- Kind: game location. Run: `.\bevyrrowind scene clouds-bittercoast-overcast`,
  `.\bevyrrowind scene clouds-seyda-storm`.
- Look for: overcast: a closed flat grey deck, the ground evenly dimmed (no sun shadows).
  Storm: tall separate cells a couple of km wide with dark wet bases, breaks between them,
  dark shadow patches under the cells. Known: no rain, fog, anvil or lightning yet (M6).
- Status: retired 2026-10-19 (phase0 triage): superseded; Phase 4.
- Source: clouds, clouds2.

### CL-6: Stable in flight (floating origin, temporal)
- Kind: game location. Run: `.\bevyrrowind scene clouds-flight`, hold Shift+W.
- Look for: clouds and cloud shadows stay put in the world (apart from their own drift)
  while the floating origin moves every 256 m (no jumps); the cloud shadow map is now toroidal
  (anchored in 2 km world cells; a full redraw when the anchor moves), so crossing a 2 km line
  must not make the ground shadows jump either; turning the camera fast leaves no
  ghost trails; the mountain silhouette has no smear. Re-check now that clouds move.
- Status: retired 2026-10-19 (phase0 triage): superseded; Phase 4.
- Source: clouds, clouds2.

## Streaming spikes (superfargoth `streaming.md`, "Spikes"; `.sessions/spikes.md`)

### ST-1: Particles pause out of view
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen, Census and Excise Office" --camera "-0.57,4.6,2.0,-0.57,2.6,6.74"`
  (fireplace), then an exterior with fires or smoke (`.\bevyrrowind scene lamps-handoff`).
- Changed: an emitter outside the view frustum (12 m margin) or past its visibility
  range no longer simulates or rewrites its mesh; it resumes where it stopped when it
  comes back into view.
- Look for: turn away from the fire and back (mouse): flames and sparks keep moving
  at once, no frozen or mis-facing quads for a frame, no burst of particles on return.
  Fires at the screen edge still animate.
- A/B: none at runtime; compare with an older `play/` exe if in doubt.
- Status: confirmed by owner 2026-10-19 (round 5): fires animate correctly.
- Source: spikes.

### ST-4: Benchmark offscreen render size (`BEVYRROWIND_RENDER_SIZE`)
- Kind: game location (benchmark switch, not a game feature). Run: `$env:BEVYRROWIND_RENDER_SIZE="640x360";
  .\bevyrrowind view "Seyda Neen"`, then with `"2560x1440"`.
- Changed: the exterior camera renders into an offscreen image of that size, shown scaled to the window by a
  second (2D) camera and a full-window UI image, so benchmarks vary pixel count and window independently.
- Look for: the whole window filled, right aspect, soft at 640x360 and sharp at 2560x1440; colors and
  brightness as without the switch (no double tonemapping, no sRGB darkening); UI overlays (F1) still drawn.
- Sanity screenshot (spikes4, 640x360 in a 1280x720 window): scene fills the window, colors normal.
- Status: retired 2026-10-19 (phase0 triage): tooling, not visual.
- Source: spikes3/spikes4.

## Size-based draw distance (superfargoth `industry-audit.md` J1; `.sessions/sizecull.md`)

### SC-1: High preset culls nothing you can see
- Kind: game location. Run: `.\bevyrrowind scene sizecull-balmora`
- Look for: the town and hills look exactly as with the A/B; objects that
  go are ~2 px wide when they fade; no holes before the 400 m far-field
  handoff (proxy-kept objects stay to `d1`).
- A/B: `.\bevyrrowind scene sizecull-balmora --off sizecull`.
- Status: confirmed by owner 2026-10-19 (checked in scene balmora at High).
- Source: sizecull.

## Skinned models (superfargoth `bake.md`, "Bevy mapping"; `.sessions/skinloader.md`)

`Cooked::Skinned` bake products rendered by Bevy's own skinning and morphs
(`sf_model_bevy::spawn_skinned`), bound by joint name to a skeleton. Bind
pose only: no animation runtime yet.

### SN-1: Cooked character vs. bevy_gltf (`sf_model_bevy`)
- Kind: synthetic. Run from `superfargoth`: `cargo run -p sf_model_bevy --example skinned -- "D:\Gamedev\Godotwind\godotwind\assets\characters\quaternius\UAL2_Standard.glb"`
  (CC0 Quaternius UAL2 mannequin, 65 joints; any skinned .glb works).
- Look for: two identical mannequins in T-pose; left is the glTF imported
  and cooked by `sf_model` and loaded through `SfMeshLoader`, right is the
  same file through `bevy_gltf`. Same proportions, joints, shading (left
  uses flat material colors).
- A/B: B bends every joint of the left skeleton 30° (the skin must follow
  its joints without tearing; B again restores), M cycles morph targets
  when the model has any (UAL2 has none).
- Status: retired 2026-10-19 (phase0 triage): agent-verified 2026-10-12; system in game.
- Source: skinloader.

## Character animation (superfargoth `animation.md`, "Bevy integration"; `.sessions/animbevy.md`)

`sf_anim_bevy`: the Bevy-free graph runtime evaluated per character
(`par_iter`), local pose written back to rig bone entities from
`spawn_rig`, skins bound to those bones by name. Gizmos draw one green line
per bone (yellow/orange/red at LOD A1/A2/A3).

### AN-1: glTF character through a 2D blend space (`sf_anim_bevy`)
- Kind: synthetic. Run from `superfargoth`: `cargo run -p sf_anim_bevy --features debug --example blendspace -- "D:\Gamedev\Godotwind\godotwind\assets\characters\quaternius\UAL1.glb"`
  (CC0 Quaternius UAL1: 66 joints; samples Idle, Walk, Jog Fwd/Bwd/Left/Right, Sprint).
- Look for: the mannequin idles; W/S/A/D blend smoothly into walk, jog
  back and strafes, diagonals mix two clips without foot pops; Shift+W
  sprints. The skeleton lines (G) stay inside the body.
- A/B: G gizmos; 1-4 force A0-A3 (A3 freezes the pose, A2 updates every
  other frame), 0 automatic; T cycles 1 / 25 / 100 copies (log line every
  2 s: tiers, evaluate µs, bones written, events).
- Status: retired 2026-10-19 (phase0 triage): old synthetic example.
- Source: animbevy.

### AN-2: Guar playing its baked creature graph
- Kind: game model. Run: `.\bevyrrowind scene anim-guar`.
- Look for: the guar breathes in its idle (Idle group), skeleton lines on the
  body. Arrows walk it in place (forward/back/left/right states), R runs,
  Q/E turn, H hit, J knockdown, X death, F fidget. Events (`sound_gen`,
  footsteps) are logged in the console.
- A/B: G gizmos, L cycles the LOD tier (auto, A0-A3).
- Status: retired 2026-10-19 (phase0 triage): animation rewrite in Phase 3.
- Source: animbevy.

### AN-3: Dark elf body on base_anim, humanoid graph
- Kind: game model. Run: `.\bevyrrowind scene anim-body`.
- Look for: the dark elf skins (chest, hands) on `base_anim.nif` idling;
  arrows walk (direction picks the walk forward/back/left/right group), R
  runs, C sneaks, Z swims, N cycles stances 0-3 (hand-to-hand, weapon,
  spell…), F fidget, H hit, J knockdown, X death. Every state should play
  without the skin tearing.
- A/B: G gizmos, L LOD tier.
- Status: retired 2026-10-19 (phase0 triage): animation rewrite in Phase 3.
- Source: animbevy.

## Characters (superfargoth `character.md`, "Assembly at runtime"; `.sessions/charbevy.md`)

`sf_character_bevy`: `mw_actors` recipes resolved and merged per material
off the main thread (`sf_character::assemble`), one `SkinnedMesh` per
material (+ one per morphing head) on an `sf_anim_bevy` rig playing the
baked graph's idle; identical looks share one cached outfit. Expected looks
(vanilla Morrowind / OpenMW from memory, not run): guards in Imperial
steel/templar plates, Ajira in a common robe, Huleeya in light armor with
tail, Fargoth's naked body with a loincloth, guar and cliff racer in their
idles.

### CH-1b: Clothed legs closed from hip to shoe (feetfix)
- Kind: game models. Run: `.\bevyrrowind character fargoth fargoth:naked --camera "0.3,1.2,2.4,0,0.8,0"`.
- Look for: clothed Fargoth's pants run unbroken from the hip to the shoes
  (rigid pants upper leg, knee and ankle parts plus shoes), no gap at the
  shins, nothing under the floor; the naked Fargoth's neck, groin and knees
  sit as before. Rigid parts now keep their model's root transform under
  their bone (the pants ankle part hangs 37 cm up the shin through it).
- Status: retired 2026-10-19 (phase0 triage): superseded by SB-6 (confirmed).
  seams (2026-10-12): diagnosed as shading steps at overlapping pieces (fixed in smooth mode, SB-6) plus
  per-piece texture colour steps and authored rims (art; options in smooth-bodies.md "Seam shading").
  Awaiting owner on SB-6; the legs-closed part of this check is unchanged.
  shows the gap, after.png full lineup, after_close.png closed legs).

### CH-2: Outfit swap without a pop
- Kind: game models. Run: `.\bevyrrowind character "@imperial,guard" ajira`, press O
  repeatedly.
- Look for: the outfit changes in one frame: never a frame with no body or
  with both outfits; the idle keeps playing (the rig survives the swap).
- Status: retired 2026-10-19 (phase0 triage): covered by headless test.
- Source: charbevy.

## Smooth bodies (superfargoth `smooth-bodies.md`; `.sessions/smoothlive.md`)

Segmented bodies (rigid limb pieces), armor and clothing are smoothed per
outfit at assembly, off the main thread (`sf_character::smooth_outfit`):
seams welded, weights blended across joints, then a per-seam guard over
probe poses (elbows/knees 90°, shoulders raised, walk/run frames) puts back
the authored weights wherever a seam would open wider than vanilla. Heads
(morphs) and hair stay as authored. On by default; `--bodies vanilla` starts
segmented, **B** toggles live (every outfit rebuilds through the no-pop swap;
both looks stay cached). In `character`: **1** walks in place, **2** runs.
The lineup log prints "smooth bodies: N outfits, parts rewritten, guard
reverted, ms per outfit (indicative)".

Known remaining failures (survey, `smooth-bodies.md` "Survey rerun and
guard"), so look there first: **tail root ↔ groin / greaves pelvis opens
44-66 mm** in run frames (naked Argonians 46 mm, iron/daedric on Argonians);
**pauldron ↔ upper-arm pieces 13-22 mm** in elbow and run poses; **guard
cascades**: reverting one seam reopens a neighbor up to its vanilla width
(iron on an Imperial female 8 → 18 mm, daedric on an Argonian 33 → 66 mm).
None of these is wider than vanilla.

### SB-4: Toggle without a pop
- Kind: game models. Run: SB-1's lineup, press B repeatedly (and O, the
  outfit swap, in between).
- Look for: every character changes look in one frame: never a frame with
  no body or with two bodies; the walk keeps playing (the rig and animator
  survive). The first press per look builds it (a short wait), later presses
  reuse the cache.
- Status: retired 2026-10-19 (phase0 triage): covered by headless test.
  `smooth_and_vanilla_bodies_are_separate_outfits_and_the_toggle_never_pops`).
- Source: smoothlive.

### SB-6: Seam shading at neck, elbow, wrist, knee and ankle (seams)
- Kind: game models. Run: `.\bevyrrowind character fargoth:naked "@dark elf,,f" --camera "0.55,1.2,0.35,0.2,1.15,0"`
  (elbow), `--camera "0.15,0.55,0.55,0.08,0.5,0"` (knees), `--camera "0.12,1.5,0.45,0,1.42,0"` (neck), then
  free camera to the wrists and ankles; add `--walk` for the moving pass.
- Look for, smooth (default) vs vanilla (B): with B on, no bright or dark *line* in the lighting where
  two pieces meet (shin line below the knee, wrist, the highlight on the upper arm's rim at the elbow,
  the chin and neck-to-chest join): the shading runs on across the join. B off brings the lines back.
  What stays in both modes and is art, not this fix: colour steps where pieces use different textures
  (the knee bulb lighter than the thigh, the neck darker than the chest, hands lighter than wrists) and
  the zigzag rims and knee bulb silhouette. Tell us if those colour steps are what you meant by
  "seams": options in smooth-bodies.md "Seam shading" (seam colour levelling needs your go).
- Shots (bind pose, smooth after the fix): `shots/seams/{knee,elbow,neck}_smooth_n.png`; before:
  `shots/seams/{knee,elbow,neck}_vanilla.png`, `knee_smooth.png` (weld only).
- Status: confirmed by owner (2026-10-13).
- Source: seams.

## Node-animated creatures (superfargoth `character.md`, "Node-animated creatures"; `.sessions/nodecreatures.md`)

Creature bodies are cooked for their rig (`mesh.rig`, `sf_model::cook_rig`):
rigid meshes become single-joint parts of their transform node, so
creatures animated per node (scrib, kwama forager, shalk, bonelord,
centurion sphere) and the rigid parts of mixed
creatures (wolf, bear and goblin heads, kwama antennae, storm atronach
rocks) follow the same rig and graph as skinned bodies.

### NC-1: Node-animated lineup (`bevyrrowind character`)
- Kind: game models. Run: `.\bevyrrowind character scrib "kwama forager" shalk bonelord centurion_sphere atronach_storm`.
- Look for: all six whole and idling: the scrib's legs and mandibles on its
  body, the shalk's legs moving, the bonelord's hood/robe on the skeleton,
  the sphere centurion's ball, arms, blade and shield attached and moving,
  the storm atronach's rocks orbiting its body (they used to sit at the
  root). The kwama forager is one morph-animated mesh: it stays still
  (morph curves are not played yet). Mixed creatures' heads (wolves, bears,
  goblins, daedroth) were bound to the root before and now ride their neck:
  check one in the world (CH-3 style) when you meet it.
- Root motion (rootmotion, after the owner note): press 1 (walk), then 2
  (run). All six must step **in place** on their marks: none slides
  forward and snaps back at the loop (scrib, forager, shalk, bonelord and
  sphere used to: their travel sat on `Root Bone`, now extracted like
  `Bip01`). Press 3: everyone walks off forward at its own clip speed
  (scrib ~0.6 m/s, shalk ~0.9, bonelord ~1.8, sphere ~2.4, atronach ~0.5),
  feet roughly planted, no loop snap; 3 again puts them back on their
  marks. Same rule for any creature: `character "cliff racer" dreugh
  netch_bull slaughterfish_small mudcrab` (cliff racer and dreugh fly or
  swim off fast when 3 applies: their clips are authored that way).
- Status: confirmed by owner (2026-10-13).
  fix: looks right (all whole; log: 0 missing parts, 0 missing joints),
  but some creatures walked in place while others actually moved.
- Source: nodecreatures, rootmotion.

### NC-2: A scrib wandering in the world
- Kind: game location. Run: `.\bevyrrowind scene nc-scrib`.
- Look for: see the scene's `look_for`. The placement is cell (-4,-3); the
  camera is 12 m above where it was placed, so it may have walked off.
- Status: retired 2026-10-19 (phase0 triage): Phase 3.
  frame, camera too low; walkers log: 83 movers, 55 wandering, all grounded).
- Source: nodecreatures.

## Walkers (M3 demo: NPCs walk the streets)

`sf_controller_bevy` (ORCA avoidance → kinematic mover → animation feed) on the characters
`sf_character_bevy` spawns, driven by `mw_ai`'s `PathWalkScaffold` (app `walkers.rs`). Keys in
the streamed view: J mover gizmos, K foot placement on/off. `--off walkers` turns it all off
(CH-3 then shows everyone standing).

### WK-1: Balmora wanderers walk their pathgrids
- Kind: game location. Run: `.\bevyrrowind scene wk-balmora`.
- Look for: `AI_W` wanderers walk street pathgrid edges at walking pace, stop and idle, sometimes
  fidget (Idle2-9); the walk cycle's stride matches the ground speed (SpeedMatch, no skating);
  turns are smooth; non-wanderers stay at their posts; nobody sinks, floats or walks through walls.
- Status: confirmed by owner 2026-10-19 (round 10, scene wk-balmora).
  log: 472-478 movers in the near rings, 65-75 wanderers, 30-45 moving, all but 5 grounded).
  animfix (2026-10-12) changed what to expect: the walk cycle now follows the *measured* speed
  (distance actually travelled), playback 0.5-1.5x plus stride warping; a walker blocked by
  another or a wall stands (idle) instead of walking on the spot; slowing to a stop the stride
  shortens, then idle below 0.25 m/s; a standing NPC that turns plays its turn clip. Look for:
  each planted foot stays put on the street while the body passes over it.
- Source: walkers, animfix.

### WK-2: Walkers avoid each other and the camera
- Kind: game location. Run: `.\bevyrrowind scene wk-balmora`, fly down to street level and stand
  in a walker's path.
- Look for: walkers passing each other keep a gap (no overlap, no jitter); a walker heading at the
  camera steps around it (the camera never gets pushed: it is non-reactive, as the player will be).
- Status: confirmed by owner 2026-10-19 (round 10).
  `walkers_avoid_a_standing_obstacle_that_never_moves`).
- Source: walkers.

### WK-3: Narrow crossing
- Kind: game location. Run: `.\bevyrrowind scene wk-crossing`.
- Look for: walkers meeting on the bridge or its stairs: one yields, both continue; no permanent
  deadlock. Vanilla wanderers are sparse, so a meeting may take a minute; the 4 + 4 doorway swap is
  covered headless in `sf_controller` (`avoid` tests, 13.2 s).
- Status: retired 2026-10-19 (phase0 triage): locomotion rewrite in Phase 3.
- Source: walkers.

### WK-4: Feet on stairs and slopes
- Kind: game location. Run: `.\bevyrrowind scene wk-stairs`; wait for a walker on the stairs.
- Look for: the body rises step by step without pops (the mover smooths the height), each foot on
  its step, knees bent; on the sloped street both feet touch. K toggles foot placement for A/B.
  Scaled races (Bosmer, Altmer): feet land on their steps too (walkers2: probe origins and samples
  now use the body scale; before, a few cm off).
- Status: retired 2026-10-19 (phase0 triage): locomotion rewrite in Phase 3.
  other does a reduced stride, the body slides forward. Cause found: a path into ground too steep to climb (or a wall) at a
  shallow angle makes the mover slide along the contour at ~1/4 speed while the walker kept facing into the slope, so the graph
  got a slow sideways walk (WalkLeft/Right blend, whose contact curves barely plant one foot). Fix: walkers face the motion
  the surface leaves (`Locomotion::deflect`, orient to movement). Re-check: where a walker meets a steep bank or a wall
  edge it turns along it and walks forward (both feet plant, stride shrinks with speed), or stands facing it when head-on;
  no sideways skating with one leg dragged. Log: `walkers strides since start: N stances, S skated, F legs without a plant`
  (S, F should stay a few % of N; `BEVYRROWIND_STRIDE_TRACE=1` logs each case). Known open: some walkers on flat ground
  walk with frozen contacts (one leg 0, one 1, see .sessions/slopefix.md); strafe clips (WalkLeft/Right) plant almost
  never (cook contact detection).
  without a walker on them; foot placement compiled into `mw_humanoid` and probing, pose not yet seen
  close up: the owner judges it live). animfix: on stairs the pelvis drops at most 0.4 of a leg
  and a sample more than half a leg away (a ledge, a probe past the step) is ignored, so a foot may
  hover at a step edge rather than the body sinking; the log counts it (`walkers feet: ...`).
- Source: walkers, animfix.

### WK-7: No blow-ups (legs far from the body, pelvis on the ground)
- Kind: game location. Run: `.\bevyrrowind scene wk-balmora`, then fly around Balmora for a few
  minutes: down among walkers, away past 250 m and back (dormant → active), through the crowd near
  the bridge. Also `.\bevyrrowind scene sb-balmora`.
- Look for: no character with its legs stretched away and the pelvis on the ground, sliding (the
  bug the owner saw). Every 5 s the log prints `walkers feet: N blocked; IK guards since start:
  ... unplanted, ... reach clamps, ... pelvis clamps, ... teleports, ... non-finite`; a few
  unplants per minute are normal, non-finite should stay 0.
- Root cause fixed (animfix 2026-10-12): a planted foot's lock drifted beyond the leg's reach when
  the body moved faster than the clip (or slid while idle), and the reach cap pulled the pelvis to
  the lock's height. Headless regression test reproduces it with the guards off.
- Status: confirmed by owner (2026-10-12): no blow-ups, pelvis never on the ground.
  non-finite, 65 unplants; stairs 120 s: 166 pelvis clamps bounded at 0.4 leg).
- Source: animfix.

### WK-5: Interior wanderers (Eight Plates)
- Kind: game location. Run: `.\bevyrrowind scene wk-interior` (Balmora, Eight Plates: 4 AI_W
  wanderers with pathgrid nodes in range).
- Look for: patrons walk the interior pathgrid between tables and the bar, at walking pace, feet on
  the floor (not on the exterior terrain height, not sunk), around furniture rather than through
  it; they stop, idle, fidget. J: mover gizmos (green ring = grounded). Nobody walks through walls.
- Status: retired 2026-10-19 (phase0 triage): Phase 3.
  patron standing on the corridor floor; log: 5 movers active indoors, all grounded, 83 prefetched
  exterior actors dormant).
- Source: walkers2.

### WK-6: Processing range (no walking without collision)
- Kind: game location. Run: `.\bevyrrowind scene wk-balmora --rings`, then fly up and away from
  Balmora (> 250 m) and back.
- Look for: in the log, `walkers: N movers + M dormant`: M grows as you leave (actors outside the
  R0 cells park where they stand, in idle, never walking through buildings), and walkers resume
  in place when you come back. Distant NPCs seen from the air stand still (low animation tier).
- Status: retired 2026-10-19 (phase0 triage): covered by headless tests; Phase 3.
- Source: walkers2.

## Object-terrain blending (superfargoth `terrain.md`, "Object-terrain blending (as built)")

Rocks, cliffs, tree roots and building bases take the ground's own surface
(the same splat layers, tint and macro variation the terrain shows) over a
band above the terrain: 0.6 m rocks, 0.4 m buildings, 0.3 m trees. The edge
is broken up by the layers' height maps. Raster only (`--rt` keeps the old
seam). Agent screenshots: `D:\Gamedev\Bevyrrowind\shots\terrainblend\`
(`tb1b_on`/`tb1b_off`: first version, top-down projection, showed vertical
streaks on wall bases; `tb1c_on`: the shipped triplanar version, not viewed
by the agent: the screenshot cap).

## Synthetic scenes (framework examples)

Run from `D:\Gamedev\Bevyrrowind\superfargoth`; they bake their data on
first run (cached afterwards). Each isolates one system on generated
content, so a bug seen here isn't Morrowind content.

### SY-1: Far field on a synthetic world (`sf_lod_bevy`)
- Run: `cargo run --release -p sf_lod_bevy --example far_field -- [--tint] [--night] [--no-far]`
- Isolates: HLOD levels, tree impostors and light sprites on 16 x 16 km with
  30 towns and ~80k placements. Keys: WASD/QE, right mouse look, Shift x10,
  F2 level tint.
- Look for: as FF-1/FF-2/FF-6, without terrain or Morrowind assets.
- Status: retired 2026-10-19 (phase0 triage): old synthetic example; system seen in game.

### SY-2: Sky, sun and moon (`sf_sky_bevy`)
- Run: `cargo run -p sf_sky_bevy --features solari --example sky`
- Isolates: atmosphere, sun/moon by time of day, raster vs ray-traced switch.
  Keys: R raster/RT, Space pause, Up/Down time speed, 1-5 jump to 6:00,
  12:00, 19:30, 21:00, 0:00.
- Look for: as SK-1/SK-2; RT vs raster brightness match.
- Status: retired 2026-10-19 (phase0 triage): old synthetic example.

### SY-3: Terrain on a procedural world (`sf_terrain_bevy`)
- Run: `cargo run -p sf_terrain_bevy --example terrain --features bevy/dynamic_linking -- [--lod] [--wire] [--hour H]`
- Isolates: clipmap terrain LOD, morphing, splats, sun on fBm mountains.
  Keys: WASD/QE (Shift x5), right mouse look, wheel speed, L LOD colors,
  F wireframe, P freeze selection, T +1 h.
- Look for: as TR-1/TR-2 (and SH-3 once far shadows land in the example).
- Status: retired 2026-10-19 (phase0 triage): old synthetic example.

### SY-4: Streaming (`sf_world_bevy`)
- Run: `cargo run -p sf_world_bevy --example stream --features bevy/dynamic_linking -- [--rings]`
- Isolates: placement streaming over a 20 x 20 km world (~100k placements),
  a far-precision cluster 500 km out, an interior behind a door. Keys:
  WASD/QE, right mouse look, Shift x10, Ctrl x0.1, F1 rings.
- Look for: no pop-in in front of the camera; walking into the house door
  east of the start crosses into the interior and back.
- Status: retired 2026-10-19 (phase0 triage): old synthetic example.

Non-visual: `cargo run -p sf_terrain --features bake --release --example
bake_synthetic` (terrain bake timings) and `--example bake_throughput`.

### PT-1: Seamless door portal (M8 spike)
- Status: retired 2026-10-19 (phase0 triage): portals parked until M9.
- Kind: game location. Run: `.\bevyrrowind scene portal-census` (outside) and
  `.\bevyrrowind scene portal-census-in` (inside); both add `--portals`.
- Look for: from the street, the Census Office's interior through the open
  door (its warm light and exposure, no sky, no wall behind the opening);
  from inside, Seyda Neen under the sky through the exit door, with the
  office itself not sunlit. Walk through (W, mouse look) both ways: no fade,
  no black or wall flash at the door, heading and speed kept. The bigger
  door swings open or hides; the smaller hides.
- Known (spike): a ~0.3 m strip at the bottom of the opening seen from the
  street shows the interior's fog color; each side keeps its own exposure
  (the street isn't blown out from inside); moving makes the opening a bit
  soft/ghosty under DLSS (its motion vectors are the quad's); the opening
  can show street around the small exterior door from inside (1.27 size
  ratio). Other doors also get portals but only one opens at a time (the
  one you look at).

> PT-1 owner pass (2026-10-12): "a good start"; gap/void, white flash,
> per-space exposure and no way back out reported. Fixed in PT-2..PT-5
> (superfargoth portals.md "Owner findings and fixes").

### PT-2: Portal opening aligned and grounded (Census Office)
- Status: retired 2026-10-19 (phase0 triage): portals parked until M9.
- Kind: game location. Run: `.\bevyrrowind scene portal-noon` (street) and
  `.\bevyrrowind scene portal-noon-in` (inside).
- Look for: the opening is the same size from both sides (1.64 x 2.81 m,
  the smaller door's), standing on the floor: no fog-colored strip at its
  bottom, no street margin around it from inside; moving sideways, its
  edges stay on the door frame. Inside, the bigger office door stays
  closed around the (smaller) opening: no blue-grey void ring.
- Known: in the first 0.13-0.26 m after arriving, turning round shows the
  far side's back-of-wall (the return opening is hidden in that band).

### PT-3: Viewer exposure, noon and sunset (Census Office)
- Status: retired 2026-10-19 (phase0 triage): portals parked until M9.
- Kind: game location. Run: `portal-noon`, `portal-noon-in`,
  `portal-sunset`, `portal-sunset-in`. A/B: add `--scheduled-exposure`
  (the spike's per-space exposure).
- Look for: noon, from the street the office is dark through the door;
  from inside the street is blown out; crossing either way: no white
  flash, the eye adapts over ~1-1.5 s. Sunset: both sides close in
  brightness. The street itself looks as before (EV100 ~14 at noon) and
  the office as before (EV100 7) once adapted.

### PT-4: Viewer exposure at night (Census Office)
- Status: retired 2026-10-19 (phase0 triage): portals parked until M9.
- Kind: game location. Run: `portal-night`, `portal-night-in`.
- Look for: from the street the office glows bright and warm; the street
  stays dark (not lifted to grey); from inside the street is dark through
  the door; crossing adapts smoothly.

### PT-5: Walk through and back (Census Office)
- Status: retired 2026-10-19 (phase0 triage): portals parked until M9.
- Kind: game location. Run: `.\bevyrrowind scene portal-walk` (scripted:
  in 4 m, back out 4 m backwards, twice), then fly yourself.
- Look for: four crossings with no flash or wall; afterwards, by hand:
  walk in and straight back out (S, backwards) at once, strafe through
  (A/D), turn round inside and walk out: every crossing works both ways.

> portal3 (2026-10-12): owner reported no terrain from inside, a flash frame at the crossing, exposure "too
> intense". Fixed in PT-6; PT-2..PT-5 now run with the softer adaptation (strength 0.7, 3/2 EV/s).
> Seamless portals are now an optional setting, off by default (`--portals` or `BEVYRROWIND_PORTALS=1`;
> the scenes add the flag), parked until M8.

### PT-6: Terrain from inside, no crossing flash, exposure strength (Census Office)
- Status: retired 2026-10-19 (phase0 triage): portals parked until M9.
- Kind: game location. Run: `portal-noon-in` (terrain: look out at the hills
  beyond the street), `portal-walk`, `portal-noon`, `portal-night`, each with
  `play/bevyrrowind.exe` (Solari/DLSS) and `play/bevyrrowind-raster.exe`.
- Look for: from inside, terrain (ground, hills) through the door, not
  floating statics over void; crossing either way, no single bright or dark
  frame (the log's `portal luma` lines show the presented frames' brightness
  around each crossing: no lone spike). Exposure: press `-` / `=` to change
  the adaptation strength (0.1 steps, printed with the resulting night /
  interior / noon EV100; F1 shows it); 0.7 is the default, 1.0 was portal2's.
  Tell us the value you like.
- Known (parked for M8): walking in, the office is ~1 EV darker through the
  door than once inside (a step at the crossing, not a flash); walking out
  at noon the street blows out and adapts (physical); the opening is soft
  while moving (motion vectors); turning round just past the door shows
  void (thin portal band).

## Actor morphs and faces (superfargoth `character.md`, "Morph playback"; `.sessions/actorfx.md`)

| ID | Scene | What to look for | Status |
|---|---|---|---|
| AF-1 | `cargo dev run -- character "kwama forager" fargoth` (or `cargo run -p bevyrrowind --features dev -- character "kwama forager" fargoth`) | The forager's body swells and squeezes with its idle (geometry morph from its KF clip), no vertex explosions; log `morphs: 9 of 9 targets follow animation curves` with `RUST_LOG=sf_character_bevy=debug` | awaiting owner |
| AF-2 | same lineup, fly close to Fargoth's face | Eyes blink every 2-8 s; press `Y`: the mouth flaps (fake loudness), `Y` again stops; no blink while talking | awaiting owner |
| AF-3 | `character "kwama forager"` placed in an egg mine (any LEVC kwama) via `view` | Leveled kwama placements now wander like CREA ones (walkers report: wanderers > 0 in egg mines) | awaiting owner |
| AF-4 | `cargo dev character ancestor_ghost Heart_Akulakhan dremora clannfear` (shot: `shots/actorfx2/lineup.png`) | Heart of Akulakhan: fire, frost, shock and magicka shields scroll (UV) and blink in and out with its idles; log `actor fx ...: 5 hidden joints, 3 materials, 6 billboards` with `RUST_LOG=bevyrrowind::actor_fx=debug` | awaiting owner |
| AF-5 | same lineup, orbit the heart (right mouse) | The ray sheets around the heart (6 billboard parts) keep facing the camera from every side while the heart beats; nothing sits at the heart's root | awaiting owner |
| AF-6 | same lineup | Ancestor ghost is see-through and its robe alpha follows its idle (not a fixed fade); dremora and clannfear look as before (their effects only play in death / attack groups) | awaiting owner |
| AF-7 | `cargo dev character dremora dremora_lord clannfear --camera 0.6,1.4,3.6,0,1.0,0` (dremora fix, `.sessions/dremora.md`; shots/dremora/before.png, after.png) | Both dremora wear their daedric armor while idling: shoulder pauldrons, upper-arm plates ("biceps"), forearm braces, thigh greaves, calf boots and shoes (before: only the leather body suit). Press X (death): the armor stays on through the fall until the clip's end, then hides as authored | confirmed by owner (2026-10-13) |

## Held items (superfargoth `character.md`, "Held items at runtime"; `.sessions/heldrender.md`)

Weapons, shields and torches as rigid props on `Weapon Bone` / `Shield Bone`
(`sf_character_bevy::held`), mounted by readiness (vanilla: no sheathed
weapons), torches lit when dark (placed-light calibration + flicker, the 12
nearest lit), the graph's weapon stance fed from readiness + weapon type.
Compare with OpenMW (same NPCs, same hour).

### HI-1: Guards with weapons drawn, stance layer
- Kind: game models. Run: `.\bevyrrowind character "@imperial,guard" "volene llervu" "@dark elf,guard" "@redguard,guard" "@nord,guard" --drawn`.
- Look for: each guard's weapon in the right hand, gripped at the hilt (blade
  out of the fist, not through the forearm), the Nord's battleaxe two-handed
  pose; arms in the weapon-ready idle (stance `1h`/`2c`), not the relaxed idle.
  R sheathes: weapons vanish, arms drop to the plain idle; R again draws.
  Check the 2 `BoneOffset` models (−90° about X) if one shows up.
- Status: confirmed by owner (2026-10-12): weapons gripped right.
  with `--night`: swords upright in hand, shields on arms, battleaxe held).
- Source: heldrender.

### HI-2: Redoran with shield
- Kind: game models. Run: `.\bevyrrowind character "volene llervu" "@dark elf,guard"` (R toggles).
- Look for: the chitin shield on the left forearm, face outward, not
  intersecting the arm; it stays while weapons are drawn (one-handed weapon)
  and disappears for a two-hander drawn.
- Status: confirmed by owner (2026-10-12): shield and weapons look right.
  `character @imperial,guard --camera 1.2,1.4,1.6,0,1.0,0`): the Imperial guard's tower shield sits
  rotated about 90°, long side horizontal across the front of the thighs, instead of upright on the
  left forearm. Compare with OpenMW; check the shield part's root transform under `Shield Bone`.
- Source: heldrender.

### HI-4: Balmora at night, torch-bearing walkers (`scene hi-balmora-night`)
- Kind: game location. Run: `cargo dev scene hi-balmora-night` (21:30, clear).
- Look for: walking guards/commoners carry torches whose light moves with
  them along walls and the street; no more than 12 torches lit at once
  (the nearest); no weapons shown; bows are absent (skinned models: known gap).
- Status: retired 2026-10-19 (phase0 triage): walk cycle, Phase 3.
  Fixed: the `Torch` group now loops on the left arm (`mw_graphs` `torch_layer`, `left_arm` mask) while a torch
  shows; the carried light is dimmed to read "fully lit" at a third of its radius (the original's clamp edge).
  Look for: walkers hold the torch up in a bent left arm (not straight ahead) while legs walk; the pool under
  each walker is warm, about 2-3 m wide, no white blow-out on the walker. Known, not torchfix: placed lanterns
  still blow walls out at night (lights calibrated at EV 7, night exposure ~EV 1: sky-lighting follow-up); a crate by
  the door left of the camera rendered black in the agent shot (`shots/torchfix/hi4_balmora.png` vs
  `hi4_before.png`): check whether it is lit.
- Source: heldrender.

## Particles on actors, bows, carried-light near field (nif-runtime.md "Particles on actors"; `.sessions/actorparticles.md`)

Torch flames and creature particles are NIF emitters anchored at a skeleton
joint or the prop root (world-space particles trail a moving joint); bows are
skinned props on their own small skeleton (rest pose); the carried torch
light keeps 0.7 m off the carrier, dimmed to read "fully lit" at a third of its radius (torchfix). Compare with OpenMW.

### AP-1: Torch-bearer at night: flame and light, not overexposed
- Kind: game models. Run: `.\bevyrrowind character "imperial guard" "@wood elf,archer" --night --drawn` (N toggles night).
- Look for: a flame (additive particles) at the torch's burning end; it
  trails when the arm moves (walk with 1); the guard's armor near the torch is
  bright but keeps its texture (before: blown out white), the floor pool is
  warm and wide. Torch orientation in the hand is HI-3's open question.
- Status: confirmed by owner (2026-10-13).
  centimeters from the torch head (inverse-square, no near clamp in Bevy: ~100x "fully lit"), and NIF emissive
  (the flame) was never exposed (Bevy 0.20 `emissive_exposure_weight` 0: ~150x). Now the torch model is
  `mw_assets::self_lit` (its texture at "fully lit", like the original's saturated torch) and emissive is
  exposed (`set_emissive`). Look for: the torch's wood and iron textures visible, not white; an orange flame
  with grey smoke at the top; torch held upright in the bent left arm; the guard's chest warm but textured;
  the floor pool smaller and dimmer than before (matches the original under the torch, falls off faster).
  Agent shot: `shots/torchfix/ap1_night.png`. Also check any glowing NIF (glow maps, candle flames) in an
  interior: emissive is now 1x "fully lit" at EV 7 instead of ~150x.
- Source: actorparticles.

### PA-1: Atronach body flames cover the skeleton
- Kind: game models. Run: `.\bevyrrowind character atronach_flame atronach_frost atronach_storm --night`
  (`RUST_LOG=bevyrrowind::actor_particles=debug` logs `live systems [(index, emitter, targets)]`).
- Look for: the flame atronach burns all over its body and limbs (array
  emitter: a random node of `Bip01 Pelvis`'s 51-node subtree per particle,
  started on the bone between node and parent), not one glow at the pelvis;
  the flames follow the idle pose; frost and storm atronachs likewise
  (frost: 63 nodes, storm: 48). The body flames never pause, not even at
  the end of `Idle3` (whole-range window, nif-runtime.md "Emission clocks").
  Moment-keyed effects (hand flames, Dagoth Ur's hands, dremora death dust,
  ghost sprays) follow the animation: PA-3.
- Status: confirmed by owner (2026-10-12).
  atronach glows over chest, arms, legs and feet (before: pelvis only), frost atronach mist
  over body and hands, storm atronach faint.
- Source: particles2. Re-shot after particles3 (`emit:` curves):
  `shots/particles3/lineup_night.png` (flame and frost atronachs still burn
  and mist; Dagoth Ur and the dremora idle with no particles, as they should).

### PA-2: Placed machine steam bursts on the controller clock
- Kind: model viewer. Run: `.\bevyrrowind view l\light_om_brazier.nif --model`, then
  `.\bevyrrowind view f\furn_lavacave_pool00.nif --model`.
- Look for: the brazier's fire and the lava pool's spray rise from their
  keyframed emitter nodes as those move (live emitter nodes, `NifNode`), not
  from the nodes' rest places; emission follows the model's controller clock
  (windowed systems burst in step with the model's motion; copies with random
  start differ in phase, parts of one copy stay in step).
- Status: confirmed by owner (2026-10-12): brazier and lava pool.
  (`i\act_sotha_fabsteam.nif`, steam windowed 1 s of 2 s) shows only a box: an
  activator whose controllers don't auto-play, so its steam waits for gameplay.
- Source: particles2.

### PA-3: Creature particles keyed to a moment of the animation
- Kind: game models. Run: `.\bevyrrowind character dagoth_ur_1 atronach_flame dremora ancestor_ghost --night`
  (`RUST_LOG=bevyrrowind::actor_particles=debug` logs `actor particles ...: N emitters, N systems on
  emission curves`: 8 for Dagoth Ur, 4 for the flame atronach, 1 for the dremora, 4 for the ghost).
- Keys (lineup, particles3): `F` plays the next fidget idle (`Idle2`, then
  `Idle3`, `Idle4`, again `Idle2`; the log says which), `X` toggles `dead`
  (`Death1`, held on its last frame; `X` again gets up).
- Look for (emission follows the creature's animation clock, `emit:` clip curves):
  - While they idle: the flame atronach's body burns (also at the end of
    `Idle3`); Dagoth Ur, the dremora and the ghost show no particles.
  - First `F` (`Idle2`): lavender-blue particles spray from Dagoth Ur's right hand for about
    1.7 s of the 3.7 s idle, then stops; nothing on the others.
  - `X` (`Death1`): the dremora's dust bursts over his whole body for about
    1.7 s of the 2.7 s fall; the ghost's sprays burst for about 0.7 s; Dagoth
    Ur's body flashes for 0.8 s; the flame atronach keeps burning as it
    falls. After the burst, particles fade over their life; nothing restarts
    while the body lies still.
  - Not reachable here: the flame atronach's hand flames (`Weapon`) belong to
    `Attack1`-`3` and Dagoth Ur's hand bursts to `SpellCast` (no attack or
    cast key yet: check in game when combat lands).
- Status: retired 2026-10-19 (phase0 triage): Phase 3.
  not fire, and emits no light. Checked against the data (`r/dagothr.nif`, `r/xdagothr.nif`): ours matches.
  The 8 systems have no colour modifier (white, additive) and take their colour from their textures:
  right hand `tx_dagoth_missalpha` x2 (lavender-blue, mean RGB 0.60/0.52/0.75) and `tx_dagoth_targetalpha`
  (orange, 0.77/0.47/0.34), left hand `missalpha` and `tx_dagoth_touchalpha` (cyan, 0.46/0.81/0.82),
  pelvis `tx_dagoth_slefalpha` (violet) and `tx_dagoth_bodyalpha` x2 (red). The `Idle2` spray is the big
  right-hand `missalpha` system (64 particles), so bluish is right; "fire" above was a guess. The NIF has
  no `NiPointLight` or `AttachLight` node, and creatures carry no light record: there is no light to
  spawn. Look for: the `Idle2` spray reads lavender-blue like OpenMW's; a warm glow, if the original shows
  one, would come from spell-cast VFX (`SpellCast` + effect visuals), not this model.
- Source: particles3.

### PA-4: GPU-drawn particles look as before (ROADMAP 1.2, particles1)
- Kind: game location. Run: `.\bevyrrowind scene particles-census-fire` (fireplace), then
  `.\bevyrrowind scene hi-balmora-night` (torch flames on walkers, braziers) and `.\bevyrrowind character atronach_flame --night`.
- Changed: quads are placed and turned to the camera in the vertex shader from one shared particle buffer
  (`mw_assets` particles.rs, `shaders/particles.wesl`) instead of a mesh rewritten per emitter per frame.
- Look for: flames, sparks and smoke as before (size, colour, additive glow, soft blended smoke, spin); quads
  face the camera from any angle; torch flames still trail a moving hand; no flicker, no stray quads at the
  fire's base, none vanishing near the screen edge; fog dims far fires as before.
- A/B: the previous exe (none at runtime; `--off particles` removes them).
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): GPU particles fine. (Fire/torch smoke vs sky draw order is a separate bug: RO-B.) Source: particles1.

## Lighting: night exposure floor, point-light shadows (superfargoth `sky-lighting.md` "Exposure range", "Local-light shadows"; `.sessions/lighting.md`)

Shots in `shots/lighting/` (`before_night.png` / `night_old.png`: the old night; `night2.png`: floor 3.5 + night ambient 1;
`cave_in1*.png`; `torch_stage.png`).

### LT-1: Night exposure floor + night ambient (`scene lt-night-floor`, `scene lamps-handoff`)
- Kind: game location. Run: `cargo dev scene lt-night-floor --hour 22`, then `cargo dev scene lamps-handoff` (22:00,
  where the owner rejected round 5). Retune 2026-10-21 (renderfix1): floor EV100 3.5 kept (lamps not blown), night
  ambient 1 -> 4 nits (x the sky's blue ambient colour): moonlit stone ~2 EV under mid grey instead of ~4.
- Look for: facades, street, stairs and far hills between the lamp pools readable in a cool moonlit blue, lamp pools
  warm, lantern walls textured (no white patch); dark, not grey or washed out. A/B: `--night-ambient 1` (the rejected
  night), `--night-ambient 2` / `6` to bracket. Agent pair: `shots/renderfix1/night_amb1.png`, `night_amb4.png`.
- Tune: `--exposure-floor <EV100>` (higher = darker night, dimmer lamps), `--night-ambient <nits>`.
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): night ambient 4 nits "seems okay" (floor EV100 3.5, ambient 4 nits).

## Physics (superfargoth `physics.md`, bevyrrowind `physics-adapter.md`; `.sessions/physics-p2.md`)

PH-3 is specified in `docs/specs/physics-adapter.md` and lands with task P10; PH-2 (first half, P7) and PH-4 (carry) are below.

### PH-1: Knock over a table's contents (`V` throws a ball)
- Kind: game location. Run: `.\bevyrrowind scene ph-tradehouse-items` (Arrille's Tradehouse) or any `view`; aim at the
  counter clutter and press `V`: a 3 kg ball (red, 12 cm radius) flies from the camera at 12 m/s (8 newest kept).
  `G` shows the shapes (orange items, cyan statics).
- Look for: before anything is hit, no item moves (they start asleep exactly as authored); hit items topple, roll and
  slide, then rest within about 3 s; bottles roll and stop (rolling resistance stand-in), plates slide; nothing falls
  through the counter or floor; no jitter at rest; carryable lanterns and items `items.check` flagged unsupported stay
  put (held until grabbed, P6). Shelf clutter rests on the shelves (bookshelves and cabinets whose authored collision is
  one box simulate with their render mesh, `support.cook`): a ball into a bookshelf knocks books off the shelf, they
  do not drop inside it. Wall-mounted weapons and shields stay put (held).
- Status: confirmed by owner 2026-10-19 (round 11, scene ph-tradehouse-items), with: some items act as if infinitely heavy and never budge (candles: lights kept static?). Phase 3 physics follow-ups.
  taken: `V` needs a key press). Known: a few items (about 12 in the Tradehouse) never fall asleep after load
  (`physics: awake item ...` log lines), see ROADMAP M4 P4.
- Source: physics-p4, physics-p4b.

### PH-5: Moved items stay where they fell
- Kind: game location. Run: as PH-1 in an exterior with clutter (e.g. `view "Seyda Neen"`, the tradehouse porch or
  market stalls): knock items over with `V`, fly away until their cell leaves R0 and unloads (watch `physics/items`
  drop in F3 counters), fly back.
- Look for: every knocked item is where it came to rest, same orientation; nothing snaps back to its authored spot.
  Within a session only: the `PlacementOverlay` holds the poses (saves are M10).
- Status: retired 2026-10-19 (phase0 triage): covered by headless test.
  needs key presses (`V`, flying), so no agent has driven it.
- Source: physics-p4.

### PH-8: Characters on physics (capsules, pushes, slope sliding)
- Kind: game location. Run: `.\bevyrrowind scene wk-balmora` (walkers on the physics ground; `--off physics` is the
  `StaticWorld` A/B), `G` for the colliders. Throw balls at walkers with `V`; for pushes, `scene ph-tradehouse-items`
  and wait for Arrille or a customer to walk through the clutter, or throw a ball onto a walker's path.
- Look for: walkers walk, step and stand exactly as with `--off physics` (no floating, sinking or new stalls on stairs,
  bridges, slopes); a ball thrown at a walker bounces off its body (it never passes through) and the walker isn't
  moved by it (no reactions yet: `ImpactOnCharacter` has no consumer before P8); light items in a walker's path are
  pushed aside at walking speed, heavy crates/barrels block, and nothing is launched; NPCs on ground too steep to
  stand on slide down it and stop at the bottom. The `physics:` log line counts capsules, pushes and impacts.
- Status: confirmed by owner 2026-10-19 (round 10): balls hit walkers, walkers walk as before.
  0 pushes, 0 impacts: nobody threw anything).
- Source: physics-p5.

### PH-6: Collider debug view (statics + terrain heightfields line up)
- Kind: game locations. Run: `.\bevyrrowind scene ph-balmora-colliders` and `.\bevyrrowind scene ph-census-colliders`
  (or any `view` with `--physics-debug`; `G` toggles the wireframes at runtime). Lines are drawn within 40 m of the
  camera: R0 statics, doors and placed items cyan (`Fixed` until P4), terrain heightfield chunks green (64 m, 0.5 m).
- Look for:
  - Balmora: cyan edges sit on the walls, stairs, bridge and canal walls they belong to (no offset, no rotation, no
    wrong scale on scaled rocks and buildings); the green grid lies on the rendered ground everywhere, including under
    buildings' foundations and on slopes; fly around: colliders appear before you get near (R0 margin), nothing missing.
  - Census Office: cyan on every room, counter and shelf; no green; walking through doors to the exterior and back keeps
    the right set (each worldspace is its own physics world).
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): "all good".
- Source: physics-p2.

### LT-4: Lamp light cut along block edges, view-dependent (owner report; ROADMAP 1.12 calls it "LT-2") (confirmed by owner 2026-10-30)
- Kind: game location. Run: `.\bevyrrowind scene lt-night-floor --hour 22`, Balmora canal by the lantern arch, then
  turn the camera / walk along the staircase.
- Seen by owner 2026-10-21 (bw-owner10, SK-1 time-lapse): lamp light is cut along screen-aligned block edges on the
  staircases and changes with the view ("like seeing the camera's cone").
- Suspects: clustered light assignment (Bevy cluster z-slices / limits, far-plane or range cull), shadowed-lamp budget
  (High = 8) chosen by view, light frustum culling vs range sphere, far light sprites.
- Owner A/B 2026-10-21 (release, same spot): blocks appear ONLY with F4 = DLSS quality; F4 = taa (native) is clean.
  Trigger is DLSS upscaling: lead is the cluster grid sized for one resolution and indexed at the other.
- Cause (lightclusters 2026-10-21, confirmed by source + release A/B shots `shots/lightclusters/`): Bevy 0.20 bug.
  `clustered_forward.wesl` `view_fragment_cluster_index` multiplies a render-res `frag_coord` by `cluster_factors`
  sized for the output viewport, so under `MainPassResolutionOverride` (DLSS) pixels read the grid's top-left part.
- Fix (render-opt 2026-10-28): applied upstream `bevy#26236` (`#26235`, `#26237`) in vendored `bevy_pbr` / `bevy_core_pipeline` so `clustered_forward.wesl`, `transmission.wesl`, and OIT normalize by `view.main_pass_viewport.zw` under DLSS.
- Status: confirmed by owner 2026-10-30.

## Night sky and sun disk (superfargoth `sky-lighting.md` "Night sky and drawn bodies"; `.sessions/nightsky.md`)

### NS-1: Sun disk size and glare (`scene ns-noon-sun`, `scene ns-sunset-sun`)
- Kind: game location. The drawn sun is 2.5° (was the physical 0.53°; `--sun-size <deg>` A/B), with a glare halo
  drawn into the sky before the clouds.
- Look for: noon: a round white disk about 5x its old width with a soft halo a few disk widths wide, no ring or square
  edge; `--clouds fair`: clouds crossing the sun hide disk and halo (no sun "in front of" a cloud). Sunset: a big
  low orange disk with a warm halo (`--hour 19.5`: setting behind the hills). Too big / too small / halo too strong: say
  which (`--sun-size`).
- Status: **confirmed by owner 2026-10-19** (round 1, Seyda Neen noon). Sanity screenshots 2026-10-13 (nightsky, `shots/nightsky/ns-noon-sun.png`, `ns-sunset-sun.png`):
  disk ~2.5°, halo subtle; cirrus in front of the sun covers it.
- Source: nightsky.

## Haze and dust (superfargoth `weather.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

### FG-1: Haze and dust per weather (`scene fg-redmtn-clear`, `fg-redmtn-foggy`, `fg-redmtn-ashstorm`, `fg-redmtn-ashstorm-sunset`, `fg-bittercoast-morning`)
- Kind: game location. Owner picks the defaults: the clear-day haze (20 now; try `--haze 10`, `--haze 40`), the ash
  storm's dust (4 now; `--dust 2`, `--dust 8`) and whether blight's red is strong enough (`--weather blight`).
- Look for: clear: far hills sit back slightly in a pale haze, blue sky overhead (A/B `--haze 1` is the old look);
  foggy: milky grey-white sky, the mountain veiled but readable; ash storm: grey-brown sky and veil, dimmer warmer
  sun; blight: rusty orange cast; sunset: warm brown dusk without bands; Bitter Coast 6:30 foggy: humid milky dawn,
  distant islands fading (morning curve x1.5, back to the weather's value by 10:00).
- Known issue: at noon with dust (and faintly on clear days) a thin purple/white band sits on the sea horizon left
  and right of the mountain: the seam between Bevy's sky-view LUT (16 samples on grazing rays through the dense low
  layer) and the aerial-perspective LUT on the far sea plane. Not seen at sunset. F2's far fog covers the horizon; a
  quick A/B for F2 is `AtmosphereSettings::sky_view_lut_samples` 16 → 32-48.
- Status: retired 2026-10-19 (phase0 triage): superseded by weather-driven fog (WE-0/1, FG-5/6 confirmed); Phase 4.
  at 10:00 (foggy and blight from the first pass: blight then had dust 6 at 1 km scale height) and
  `redmtn_{clear,foggy,ashstorm,blight}_sunset.png` at 19:00): looks as described, with the band above.
- Source: m6-f1.

### FG-2: Height fog, ground-fog banks, far fog (`scene fg-bittercoast-morning`, `fg-bittercoast-ridge`, `fg-balmora-foggy-noon`, `fg-redmtn-ashstorm`, `fg-night-fog`)
- Kind: game location. M6 F2 (weather.md "Fog as built"). A/B everywhere: `--off fog` (F2 off, F1 haze stays),
  `--fog 0.5` / `--fog 2` (density), `--off clouds` (fog alone).
- Look for: Bitter Coast 6:30 foggy, low (`fg-bittercoast-morning`): white swamp morning, near trees crisp, ground
  and water milky within ~100 m; from 160 m up (`fg-bittercoast-ridge`): banks lying on the swamp and the water
  with darker gaps where the water shows, continuing toward the horizon; hills rising out of it. Balmora foggy noon:
  milky valley floor, the mountain a pale outline, fog colour meeting the horizon without a band. Ash storm (10:00):
  thick grey-brown, visibility a few hundred meters. Night (23:00 foggy): dim, never brighter than the sky, lights
  and moons dimmed by it; readable through the exposure floor.
- Also check: no halos or blocky edges where trees, roofs or far-field proxies meet fog (the banks are traced at
  reduced resolution, the fog amount is per pixel); no swimming while the camera moves; DLSS (`--dlss quality`) no
  smear on bank edges; the sea-horizon seam from FG-1 gone (sky-view LUT 32 samples) or hidden by the fog.
- Owner picks: bank density/height (0.03 /m, 12 m; `--fog`), how much banks stay by day in foggy weather (a third),
  the ash storm's visibility (450 m at sea level) and whether clear mornings in swamps want banks (waits for
  climates, WE2/WE3).
- Known: transparents (particles, glass) are not fogged until F3; the sea scaffold is now opaque (the composite
  fogs by depth), so shallow seabed no longer shows through.
- Status: retired 2026-10-19 (phase0 triage): superseded; Phase 4.
  (one clear gap shows; most of the view is in banks plus haze 90), `fg-balmora-foggy-noon.png`,
  `fg-redmtn-ashstorm.png`, `fg-night-fog.png`.
- Source: m6-f2.

### FG-3: Froxel fog and god rays (`scene fg-balmora-sunrise-rays`, `fg-balmora-sunrise-foggy`, `fg-balmora-ridge-rays`, `fg-bittercoast-morning`, `fg-redmtn-ashstorm`)
- Kind: game location. M6 F3 (superfargoth weather.md "Froxel fog as built"). A/B everywhere: `--off froxels` (alias
  `godrays`: F2's analytic fog only, the Low preset), `--off farshadows` (no ridge shadows in the fog past the cascades).
- Look for: Balmora 6:15 looking east: haze in the shade of walls and the ridge darker than with `--off froxels` (the
  analytic fog lights everything as if in sun), shafts where the sun gets between roofs and trees; foggy dawn: thick
  warm fog toward the sun cut by shadow shafts, greyer away from it; ridge view with `--fog 3`: valley haze darker
  under ridges and cloud shadows. Bitter Coast morning and the ash storm look as in FG-2 (same density terms), with no
  seam at ~384 m where the froxels hand over to the analytic layers.
- Also check: no flicker or crawling bands when still (Halton jitter + 90 % history), no smear when turning or in
  flight (60 m/s), no light leaking through walls at froxel resolution (16 px tiles).
- Owner picks: clear/fair dawn haze (40 km / 30 km visibility, x1.5 at dawn: shafts are subtle on clear mornings; try
  `--fog 3`), history weight, whether High's 384 m reach is enough.
- Status: retired 2026-10-19 (phase0 triage): superseded; god rays recheck in Phase 4.
  `-off.png` (shadowed haze darker with froxels; shafts subtle at the default clear-day haze), `fg-bittercoast-morning.png`,
  `fg-redmtn-ashstorm.png`, `balmora_noon.png` (no seam).
- Source: m6-f3b.

### FG-4: Lanterns in fog, fogged transparents (`scene fg-night-lanterns`; example `clouds --view glass`)
- Kind: game location + synthetic. The 32 nearest point lights scatter in the froxels (unshadowed); transparent
  surfaces using `FroxelFogMaterial` sample the same volume.
- Look for: warm halos around lanterns and torch carriers, fading with distance; halos don't pop when a light drops
  out of the 32; example (`cargo run --release -p sf_atmos_bevy --example clouds -- --view glass --fog 400`, add
  `--time 23` for night): glass panes fade with distance like the stone panes beside them.
- Known: the street in `fg-night-lanterns` reads strongly orange (many lights × foggy density: owner picks whether
  the fog's point-light scattering needs a scale, as UE's per-light volumetric scattering intensity); halos are not
  shadowed (light through walls); the app's own NIF glass and particles are not yet `FroxelFogMaterial` (unfogged).
- Status: retired 2026-10-19 (phase0 triage): superseded (RO-A, FG-5); Phase 4.
  `ex_glass_day.png`, `ex_glass_night.png`.
- Source: m6-f3b.

### FG-5: Blended surfaces in fog (`view "Seyda Neen" --hour 11 --weather foggy`; `scene ghostfence-far --weather foggy`)
- Kind: game location. Blended and additive NIF surfaces (tree leaves, the Ghostfence field, glows, particles) draw
  after the fog composite and now fog themselves from the froxels (fogfix 2026-10-19).
- Look for: Seyda Neen's palms and far trees fade into the fog like their trunks and the ground (no dark leaf
  silhouettes over white fog at any distance); fly toward the Ghostfence and Red Mountain: the field fades in with
  distance. A/B: `--off transfog` (the old look). Also judge whether near leaves read too dark in fog (lit colour
  under the dim foggy key light; not a fog term).
- Status: **confirmed by owner 2026-10-19** (foliage no longer over the fog). Sanity: before/after shots in fogfix (old exe: leaves unfogged at all distances).
- Source: fogfix.

### FG-6: Fog without the pulse (`view "Seyda Neen" --hour 11 --weather foggy`, also `--hour 6.5`; T x20)
- Kind: game location. The froxel fog no longer pulses: per-froxel jitter, history at froxel centres (shimmer
  2026-10-19, weather.md "Temporal sampling").
- Look for: stand still (P pauses the clock): the fog, shafts and the ground under it hold steady, no breathing or
  flicker a few times a second; with T x20 the light changes smoothly as the sun moves (that drift is real). A/B
  `--off fogdither` (the old pulsing fog: watch near shadowed buildings/trees in the fog), `--off fogjitter`
  (no temporal sampling: still, but shadow edges in the fog go blocky). Also check for new fine grain in the fog.
- Status: **confirmed by owner 2026-10-19** (round 1 weather pass).
- Source: shimmer.

### WA-8: Coast flight across origin shifts (`scene wa-vivec-waterfront`, fly fast along the coast)

Status: confirmed by owner 2026-10-19 (round 12, scene wa-vivec-waterfront): no jump in the pattern.
256 m); with `--dlss`, no wave smear (velocity redraw). The phase is exact by construction (integer cells, W1 test);
not yet flown in the app.

### WA-6: Interior water (`scene wa-interior-pool`: Hlormaren, Sewers, level −2.77 m)

Status: **confirmed by owner (2026-10-18)** (W3, w3fin; black pool fixed by w3pool). Shot: `shots/w3pool/wa-interior-pool.png`
(before: `shots/w3/wa-interior-pool.png`).
The interior's water is a `WaterSurface` pool at the cell's `WHGT` (all 216 visible `HAS_WATER` interiors get one;
spawned on entering, despawned on leaving). The pool used to draw **opaque near-black**: interiors have no
`CloudCamera`, so the froxel volume was never written, yet the water still applied it (transmittance 0). Now the
volume is flagged off without an active cloud camera. Look for: flat dark water with ripples only, the channel
bottom visible through it (as with the A/B `--off froxels`, which never had the bug), level steady while moving; also
walk out of a door and back in: no black flash on the water, and exterior glass/water fog unchanged.

### WD-0: Wind field, synthetic (`cargo run -p sf_wind_bevy --example wind` in `../superfargoth`)

Status: **confirmed by owner** 2026-10-18 ("wind seems great"; WD1, wd1b). Shot: `shots/wd1b/wind-burst.png` (`-- --screenshot <png> --frames
120 --burst 100`). Top-down 315 m square: white-to-orange arrows are the CPU twin (`WindField::sample`, 2 m above
ground, length = 1 s of travel), the ground colour is the GPU field (`sf_wind_bevy::wind::wind_sample`: calm dark
blue, mean speed green, twice the mean yellow). Look for: arrows and colours agree everywhere (longer arrows on
greener/yellower ground); gust patches drift downwind (+x, slightly +z), form, swirl and fade, never a frozen or
scrolling-texture look; no arrow ever points upwind; Space fires a radial burst (yellow disc, arrows pointing out,
a calm ring on its upwind edge where it cancels the wind) that grows in 0.05 s and fades over ~2.5 s; V a vortex
(arrows circling); Up/Down change speed without the pattern jumping; G cycles gustiness 0 / 0.3 / 0.6 (0 = uniform
green). Known: no consumer in the game samples the GPU field yet (grass, trees, rain lanes); in game only the fog and
cloud drift direction follow it.

### UI-1: Gamepad-only navigation, synthetic (`cargo run -p sf_ui_bevy --example ui_nav` in `../superfargoth`)

Status: **confirmed by owner 2026-10-19** (gamepad works; not Morrowind-themed yet, U3; M5 U1, u1b, 2026-10-18). Shot: `shots/u1b/ui_nav_modal.png` (`-- --autotest <png>`: a
synthetic gamepad runs 40 checked steps and exits 0). Plug in a real pad and use only it. Look for: Start opens the
menu (focus on Inventory, yellow border); d-pad and left stick move focus between the six buttons, holding a direction
repeats after a short delay; A on Quit opens the modal with focus on No; inside the modal Up/Down never move focus to
the menu behind it; B closes the modal and focus returns to Quit; B again closes the menu; the bottom-left line
switches between `Kbm` and `Gamepad` labels (Esc/Enter vs Menu/A) the moment you touch the other device, and never
flickers while a pad button is held and the mouse moves. Known: plain placeholder styling (themes are U2/U3), Xbox
labels for every pad (per-family glyph icons are U3).

### UI-2: Mod stylesheets and hot reload, synthetic (`cargo run -p sf_ui_css --example ui_css` in `../superfargoth`)

Status: **confirmed by owner 2026-10-19** (M5 U2, u2b, 2026-10-18). Shots: `shots/u2b/ui_css_before.png` (green buttons),
`shots/u2b/ui_css_after.png` (red, 0.33 s after the sheet was rewritten on disk; `-- --autotest <dir>` works on a temp
copy and exits 0). A menu styled only by three mod sheets in load order: `base_ui` (layout, tokens, focus ring),
`bright_buttons` (later, its button colour wins), `broken_mod` (one valid HUD rule, one invalid value). While it runs,
open `crates/sf_ui_css/examples/assets/ui/bright_buttons/style.css`, change `#2e7d32` to any colour and save. Look for:
all six buttons recolour within a second with a short fade, no restart, no flash of unstyled UI; the focused button
keeps its yellow ring; a typo in that sheet shows up in the bottom-left line as `bright_buttons (...)` and the last
good look stays, fixing it clears the line; touch a gamepad: the line says `Gamepad` and the focus ring gets thicker
with a lighter fill (`[input=gamepad]`); the bottom-left line always lists `broken_mod`, and its valid rule still
tints the HUD text blue (dimmed by the menu backdrop). Revert the colour afterwards (the example edits the real file).
Known: placeholder look (game themes are U3); flair logs each broken sheet's warning twice.

### TS-1: Synthetic test levels, greybox (`scene level-flat-slopes`, `scene level-ledges`; `--camera <name>` picks a level camera)

Status: **confirmed by owner 2026-10-19** (TS1 part 1, ts1a, 2026-10-18). Shots: `shots/ts1/level-flat-slopes.png`,
`shots/ts1/level-ledges.png` (with `--physics-debug`: collider wireframes on every box), `shots/ts1/level-flat-slopes-balls.png`
(`ramps` camera, overlay on, a thrown ball resting on the floor). Levels are `levels/*.ron` (`docs/specs/test-levels.md`),
opened in the normal viewer in their own worldspace: sky at 10:00, clouds, fog, physics, F1 keys and BRP as anywhere
else; no terrain, sea or Vvardenfell. Ready ~2-3 s after start. Look for: a 1 m grid on every surface (scale reads at a
glance, no shimmer at grazing angles), sun shadows from every block; flat-slopes: ramps at 10/20/30/40° (green, yellow,
orange, red) up to a 2 m grey plateau, three stair flights (blue comfortable, purple standard, red steep) with landings;
ledges: the height row 0.3-2.5 m, thin vs deep blocks, platforms with 1-4 m gaps, the beam, the wall. Fly (WASD) over
them; `V` throws a ball that bounces off and rests on the greybox (colliders = the visible triangles); `G` draws the
colliders. Known: no walking player yet (fly camera only), floor ends at 40 m into the fog.
**Owner 2026-10-18: FAILED** (black shapes flash for a frame or two while the camera moves; gone with `--off terrain`,
unchanged with F2/F3/F10 and `--off froxels`: Vvardenfell's terrain streams into the level worldspace). Fix handed to ts1b.
**ts1b fix (2026-10-18), awaiting owner:** a level worldspace never gets `TerrainCamera` (only the exterior has land:
`exterior.rs` setup and `doors::worldspace_look`), so no terrain patch streams or draws in a level. Re-check: fly fast
(Shift) over `level-ledges`; no black shapes.

### TS-2: Level actors, props and walk mode (`scene level-robe-walk`; also `U` in any `level-*` scene)

Status: **confirmed by owner 2026-10-19** (TS1 part 2, ts1b, 2026-10-18; walk mode and props agent-checked by ts1c 2026-10-19: W/Shift moves the player, follow camera, V shoves the crates). Three robed NPCs on scripted paths (`levels/robe-walk.ron`):
camera `priest`: the Balmora temple priest in his own robe walks a flat 6 × 8 m loop, stops 3 s at its south-west
corner and fidgets; camera `course`: Fargoth (common robe) walks and Elone (extravagant robe, run gait, 4 m/s) runs back
and forth over a 15° ramp, a 1 m plateau and stairs (5 × 0.20 m) down the far side, waiting at the ends; camera
`props`: a table and three stacked crates (dynamic bodies): `V` balls shove them (aim close: a 3 kg ball moves a 15 kg crate, the stack slides apart rather than toppling). `U` toggles walk mode: the
`player` character at the `start` spawn walks with WASD (Shift runs) in the camera's heading, the free camera follows
3.5 m behind (mouse look as usual). Look for: dressed characters (no T-pose, no missing parts) whose feet stay on the
ramp, plateau and steps (no sinking or floating), walk/run playback matched to speed, turns at the path ends; crates
tumble and rest on the floor; walking the player up the `ledges` boxes steps only onto the 0.3 m one. Cloth (PH-3) is
p10c's: once it is wired, the robes should swing with the stride here. Known: no camera collision in walk mode, no
jump, the table is a single box (no legs).

### WE-0: Weather drives sky haze and wind (`view "Seyda Neen" --hour 11 --weather clear`, then `--weather overcast`)

Status: **confirmed by owner 2026-10-19** (round 1 weather pass; M6 WE3 part 1, we3a). Shots: `target/we3a/{clear,overcast,natural}.png`
(`--screenshot`). Look for: `--weather clear` matches the old `--clouds clear` (blue sky, thin haze, mountains sitting
back but sharp); `--weather overcast` matches the old `--clouds overcast` (grey, hazy, 8 km fog); no flag prints
`weather <Type>` for the start cell's forecast and shows that weather's clouds/fog. Since part 2a (we3b1) clouds and
fog follow the live weather too; C and T (see WE-1) change the weather live. No rain, lightning or border blend yet.

### WE-1: Live weather transition, clouds lead and fog follows (`view "Seyda Neen" --hour 11 --weather clear`)

Status: **confirmed by owner 2026-10-19** (round 1 weather pass, incl. rain over the sea; nit for later polish: blight
particles look a bit odd; M6 WE3 part 2a, we3b1). Shots: `target/we3b1/{a_clear,b_mid,c_overcast}.png`
(BRP, clock paused, hour stepped 11.0 / 11.35 / 12.1 after `set {weather: "overcast", transition_hours: 1}`).
How: press **C** to force the next weather (clear → cloudy → foggy → overcast → rain → ...) over the region's
transition (~0.2-0.6 game h), **T** for x4 game time so a transition takes ~10-25 s (again for x20, again for x1).
BRP: `bevyrrowind/set {"weather": "overcast", "transition_hours": 1}`, `"auto"` back to the forecast;
`bevyrrowind/status` shows `weather.clouds`/`weather.fog` progress. Look for: the clouds build and the sky dims first
(clouds 0-60 % of the transition), the fog thickens after (40-100 %); no pop in cloud positions, cover, fog or
light at any point, including when the cloud speed changes (ash storm, blight are fast); ash/blight fog turns
grey-brown/red. Known gaps (after part 2b, we3b2): rain falls under roofs and trees outdoors (no occlusion map), no splashes, streaks
unfogged, no thunder/rain audio (cue messages only), lightning lights the clouds uniformly (ambient glow, no in-cloud
flash at the strike). Also check: rain (`set {"weather": "rain"}`) slants with the wind; ash storm shows blowing
grey-brown streaks; thunderstorm flashes light the land and clouds (`time_scale` 600 for more strikes). Shots
`target/we3b2/{a_rain,c_ashstorm2}.png`.
Rain over the sea (rainwater, 2026-10-19, **confirmed by owner 2026-10-19**): `set {"weather": "rain", "camera": "-60,8,1290,-60,-4,1320"}`
(looking down at the sea off Seyda Neen): streaks show over the water as over the rocks at every pitch; water
shading/refraction unchanged. Was: rain hidden over the ocean when looking down (Transparent3d sort; water.md
"Render order"). Shots `target/rainwater/{a_sea_down,b_open_sea_down}.png`.

### WE-2: Region border blend, Ascadian Isles → Ashlands (`view "Pelagiad" --hour 12`, fly north toward the Ashlands)

Status: confirmed by owner 2026-10-19 (round 13, Pelagiad north): crossfade works, no snap, very nice. Seen from outside, the bad-weather volume shows animated grain (same as WE-3; Phase 1 temporal resolve).
unit-level: `sf_weather::border_blend`; app `ClimateMap::border_at`). How: force a clear Ascadian Isles and an ash storm
in the Ashlands is not possible with one global force, so use the forecast (`set {"weather": "auto"}`) or step the hour
until the two regions differ, then fly across the border. Look for: haze, clouds, fog and wind crossfade over ~230 m
around the border (half each side), no pop when the camera crosses a cell edge; precipitation intensity fades in, not on.
### WE-4b: Two climates on the world weather map (`cargo run --release -p sf_atmos_bevy --example clouds -- --split clear,storm`, in superfargoth)

Status: retired 2026-10-19 (phase0 triage): superseded by WE-3 in game (confirmed). Was: (M6 WE4b, we4b, 2026-10-19). Synthetic scene: west of x = 2.5 km is Clear, east is Storm
(400 m band); the camera stands on the clear side looking +X. Also `--view up` (zenith: clear sky, storm deck at the top
edge = east) and `--view shadow` (ground lit on the clear side, dark beyond the border), `--per-ray` for the Medium/Low
lookup. Look for: the storm deck starts at the border, not overhead; no seam or hard line at the band; the cloud shadow
edge on the ground matches the deck edge. Known: the shell height is still the storm's everywhere (envelope = WE4b step 4).

### LA-1: Near foliage stays drawn after a long flight (`view "Seyda Neen" --hour 11`)

Status: **confirmed by owner 2026-10-19** (foliage OK in recent runs; lodaudit, 2026-10-19). Not screenshotted after the fix (the agent's live repro never
caught a vanishing plant: CPU visibility kept every near placement visible). How: fly around for several minutes
(several km, F8 rings on if useful), come back to the swamp trees east of Seyda Neen and fly into them from ~60 m to
~2 m. Look for: no tree, bush or plant vanishing as the camera closes in (it may dither while crossing a LOD switch,
then stay solid). Check the log for Bevy's "More than 65535 distinct `VisibilityRange`s" warning (should never
appear now). If plants still vanish, note which (tree vs bush, leaves only vs whole plant) and try `--off transfog`
(fogfix's blended-material swap) and the leaf alpha-test change (leafmask) to separate the causes.

### WE-3: Neighbouring regions' weather seen from afar (`scene we3-ridge`, then BRP region forces)

WE4d (we4d, 2026-10-19), High/Ultra: under a forced rain/thunderstorm region look for grey rain curtains under the
deck, fading into the far fog, nothing over the clear side; walking in, the curtain fades as the near rain starts
(no double rain). A forced foggy region (e.g. Bitter Coast) shows its extra fog from a dry ridge.

Status: **confirmed by owner 2026-10-19: distant weather works, but grainy (many dots); same grain seen in soft shadows. Suspect: no temporal pass (TAA) when DLSS is off, dev build has no dlss; verify with a dlss build, fix = Bevy TAA fallback** (M6 WE4c, we4c, 2026-10-19; bug fixed we4c2). Camera on the ridge NE of Balmora (West Gash, forced
Clear) at 14:00. Force neighbours over BRP (port in the title): `bevyrrowind/set {"weather": "ashstorm", "region":
"Ashlands Region", "transition_hours": 0}` and `{"weather": "thunderstorm", "region": "Ascadian Isles Region",
"transition_hours": 0}`; look north (Ashlands, Red Mountain) and south-east (Ascadian Isles, Vivec). Look for: clear sky
overhead; a dark deck over each forced region only, starting at its border, continuing to the horizon; no seam at the
band; walking toward a border (400/200/100/0 m), no pop. Agent shots (`shots/we4c/`): the forced regions do get clouds
and the zenith stays clear, but the decks stop short of the horizon and hang as grey vertical columns near the camera
(bug, see weather.md "WE4c as built"). Fixed (we4c2, `shots/we4c2/`): the Ascadian Isles storm is a wall reaching the
horizon, zenith clear, no columns; the Ashlands deck ends ~3 km out where the region does (small regions under real-scale
decks can't fill the low sky). Also check: no jump in the far sky when the map recenters (walk 2 km).

WE4e (we4e): time-lapse the forced Ashlands thunderstorm (BRP `time_scale`); distant flashes should light the far deck only (not the ridge or rain); thunder follows by distance / 343. Agent shots (`shots/we4e/`) caught no strike: **awaiting owner**.

PR-1 (rainocc, **confirmed by owner 2026-10-19**, round 1 weather pass): Seyda Neen in rain (`set {"weather":"rain"}`, F1 view): stand under a roof overhang and under a palm: no streaks beneath, rain continues outside the edge (about 6 m recenter is invisible). A/B: `PrecipOcclusionConfig(false)`. Agent saw only open-sky rain (shots target/rainocc_*.png), not yet under a roof.

## Door links (superfargoth `streaming.md`, "Worldspaces at runtime"; `.sessions/interiors.md`)

### DR-4: Door travel polish and crash (owner report)
- Kind: game location. Run: `.\bevyrrowind scene door-balmora-guild`, several round trips (Space at the doors).
- Seen by owner 2026-10-19 (round 8):
  - during the load, big rough shapes appear first and real models replace them (likely the far-field L1 copies,
    shown until the near ring's models arrive: the fade lifts before the near ring is ready);
  - the fade isn't black;
  - sometimes you arrive in an interior that's still loading (objects appear in front of you);
  - no "press Space" prompt on doors (expected: HUD, Phase 2);
  - a few round trips ended in a **crash** (release exe, 10:55:47 UTC, leaving the bookseller to @exterior): Bevy's
    default command error handler panicked on a command to a despawned entity (30782v1). Same class as the walkers.rs
    `Dormant` crash fixed earlier today.
- Fixed (phase1, 2026-10-19): the fade is a full-window UI node (after post-processing: truly black); it holds at
  black until the arrival's near cells are spawned and the meshes within 48 m have their textures (10 s cap); ~100
  commands on streamed entities use `try_*`; release builds log failed commands instead of panicking. Agent run:
  4 crossings, 0.70–0.90 s each (was 0.64), the log line now says how long it held at black.
- Look for: a black fade (no fogged grey); on arrival no rough shapes, no objects popping in front of you; a long trip
  (bookseller, wait > 5 s inside so the town unloads) still arrives complete. No crash over many round trips.
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): "perfect". (On-door "press Space" prompt: Phase 2.)

### DR-3: Interiors open in the streamed view
- Kind: game location. Run: `.\bevyrrowind scene door-census` (or any
  `view "<interior>"`), press Space at the door in front of you, then Space
  at the door outside.
- Look for: the Census Office in the streamed view (doors work, the window
  title names the worldspace), lit like DL-3's raster baseline; no sea plane
  or sky inside. Out: Seyda Neen with sky and terrain (the first trip out of a
  start interior takes ~1 s: only the door's R0 was prefetched). Back in
  through the office's other door: 0.65 s. Known: that door's marker sits
  low, so the camera arrives near floor level (fly up).
- Old viewer: `--single-cell` (and `--rt`, `--no-baked`, `--skinned`, which
  the streamed view doesn't support yet) opens the single-cell viewer as
  before; `census` and `census-rt` use it.
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): door-census round trips worked.
- Source: doorfix.

## Sky (superfargoth `sky-lighting.md`)

### SK-1: Sunset and sunrise
- Kind: game location. Run: `.\bevyrrowind scene sky-sunset` (west, 19:30)
  and `.\bevyrrowind scene sky-dawn` (east, 6:00); T for a time-lapse.
- Look for: warm low sun, smooth gradient (no banding), the town not black,
  no exposure jump as the sun crosses the horizon.
- Known: the ray-traced path darkens and reddens the sky at low sun
  (open question 1); judge raster.
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): sunset and sunrise fine.
- Source: sky-lighting / dlss.

## Audio, foliage, precombine (owner pass 2026-10-21; AU: audio-pass, LM: leafshadow/renderfix1, PC: precombine3)

### PC-1: Precombined near cells look the same as separate objects (ROADMAP 1.9, superfargoth precombine.md)
- Kind: game location. Run: `.\bevyrrowind scene balmora`, then walk the streets at eye height; A/B with `--off precombine`
  (one entity per object, the old way). Also `scene seyda-neen` (docks, small clutter).
- Look for: nothing missing or doubled; pots, crates, baskets and other small clutter appear and disappear at about
  the same distances as before; no popping when buildings switch detail; shadows unchanged; no hole or double image
  where the far field takes over (~400 m). A script-disabled static (e.g. a quest rock) must still disappear.
- Status: confirmed by owner 2026-10-21 (pass bw-owner10, release build): precombine stays on by default.

### AU-1: Weather sound (`view "Seyda Neen" --weather rain`, then C to step to thunderstorm)
Status: **confirmed by owner 2026-10-21 (pass bw-owner10, release build)**: indoor fade confirmed (rain fades in/out at doors); outdoors confirmed 2026-10-19. Earlier notes below.
Earlier: **confirmed outdoors 2026-10-19; indoor fix** (phase1): an interior (no sky in its cell flags) ducks the weather bus to silence over 0.5 s and draws no region ambience; log `audio: listener indoors/outdoors`. Listen: `scene door-balmora-guild` with `--weather rain`, walk in (rain gone), out (rain back); also the Census Office and a cave. Was: weather/region ambience kept playing indoors (round 1 weather pass; M6 WE3 audio; agents cannot hear, the log lines `audio: loop start/stop`,
`audio: one-shot` were checked headless). Look/listen for: rain loop fades in over ~2 s and out when the weather
changes (C key), the thunderstorm adds its loop, thunder follows each flash after distance / 343 s, region ambience
(birds, wind) draws now and then on a clear day. Levels sane against each other; mp3 and wav both play.

### AU-2: World sounds and music (`cargo dev scene door-balmora-guild`, then a cave: `cargo dev view "Addamasartus"`)
Status: **confirmed by owner 2026-10-21 (pass bw-owner10, release build)**: "all sounds fine", cave sounds good. Leftovers (volume mixer, NPC voices, impact sounds) are in ROADMAP 2.2/2.6. Earlier notes below.
Earlier: **awaiting owner** (audio-pass slice 2, ROADMAP 2.6; agents cannot hear: the log has `audio: playlist`,
`audio: music`, `audio: cue`, `audio: N loop emitters loaded, M audible`). Listen for: explore music starts within
a few seconds and the next track crossfades in when one ends (battle music needs Phase 3 combat; nothing triggers
it yet). Walk up to a street lamp or brazier with a fire: its crackle grows as you approach and fades out by ~11 m,
panned left/right as you turn. Activate the guild door: the door sound plays from the door's side and carries
through the load. In the cave, the drip and wind loops come from their spots. Footsteps: NPC walkers within 15 m
step in time with their gait (bare-foot sounds until equipment is wired); none while flying the free camera.

### LM-1: Leaves as alpha-tested cutouts (`view "Seyda Neen" --hour 11 --weather foggy`, then `--weather clear`; forest: fly west into the Bitter Coast)
Status: **confirmed by owner 2026-10-21 (pass bw-owner10, release build)**: "leaf shadows are in". Side findings carried in LM-2. History below.
Fix (renderfix1, 2026-10-21; no re-bake, load-time): cause confirmed with `cargo run -p mw_bake --example foliage_report`: plain-blend foliage is promoted only if `textures::alpha_is_cutout` reads holes; its "soft <= 1/4 of covered" rule rejected thin-twig atlases (`tx_bc_leaves_02` of `flora_bc_tree_01`: 45% holes, 32% solid, 23% soft; also `tx_leaves_07`, `tx_branches_01/02`, roobrush, wickwheat, fire fern, bc moss), so those trees stayed Blend (no shadow caster). Now also a cutout: >= 10% holes and soft <= 1.25x solid; glass, waterfalls, crystals, chalk/glow decals, lava/oil crust still blend. New cutouts to eyeball: `tx_stain_pueblo00`, `tx_scroll_fleur`, awning thatch, woven rope. Shots: `shots/renderfix1/lm1_fix.png`, `lm1_close.png` (Bitter Coast, all canopies drawn as cutouts). Look for: dappled shadows under the big Bitter Coast trees too. Side finding: with `--off leafmask` (`lm1_off.png`) blended leaves vanish entirely at ~100 m+, so the A/B no longer shows "blended as before"; logged, not fixed.
Earlier: **partial (owner 2026-10-19, round 2, release exe with the v4 bake): some trees' foliage casts shadows, other trees' doesn't**, so not a global regression (the earlier 'none anywhere' was likely the stale v3 bake). Open: which materials stay blended (load-time promotion path?). Phase 1. Earlier: **rejected 2026-10-19: no foliage shadows anywhere** (owner, round 2, clear sky; fog and crisp edges OK in round 1; leafmask). Agent shots: foggy Seyda Neen (leaves fog like the trunks), clear
Cause found (leafshadow): stale bake, leaves cooked under importer v3 before the cutout code existed, so still Blend -> fog swap casts no shadows. Fix (uncommitted): NifImporter::VERSION 4 + re-bake. Recheck with the v4 exe: dappled leaf shadows under trees at 11:00 clear.
Recheck 2026-10-19 (owner, v4 release exe after re-bake): STILL no foliage shadows at all. Stale bake was not (or not the only) cause. Untested leads: most MW foliage is plain blend without alpha test (Cutout::Candidate), promoted at load only if the texture alpha reads as holes: check that the load-time check runs (compressed DDS alpha, texture not loaded yet when the material is built), else leaves stay Blend and the fog swap drops their shadows.
Seyda Neen (tree shadow on the island and the water). A/B: `--off leafmask` (leaves drawn blended as before: no depth,
no shadows, no motion vectors). Look for: crisp-edged leaf cards fogged exactly like trunks and terrain at every distance
(no leaves showing through fog); dappled leaf shadows on the ground under trees at 11:00 clear (with `--off leafmask`:
trunk and branch shadows only); no smearing behind foliage with TAA/DLSS (F4); distant trees not thinning to bare twigs
(cutoff 0.5, no coverage-preserving mips yet: if they thin, note at what distance). Glass, potions, Ghostfence, glows,
smoke and ghosts must still look translucent (if one became a hard-edged cutout, note the object).

## History of entries kept open (moved out of ../visual-checks.md 2026-10-21)

### FG-1 history: volumetric fog black bands on fast turns
- Seen by owner 2026-10-20: dark vertical bands appear in the lit fog during fast yaw (owner shot `Desktop/shadow fog.jpg`:
  giant-mushroom/cave area, 21:15, fog). Suspects then (unverified): the froxel volume's temporal reprojection or history
  rejection on rotation, or the froxel light/shadow lookup at history edges (`sf_atmos_bevy` froxel_inject /
  froxel_integrate). A/B tried: `--off froxels`, `--off fogjitter`, `--off foghistory`.
- Agent repro 2026-10-21 (renderfix1, `view "Sadrith Mora" --hour 21.45 --weather foggy`, `shots/renderfix1/spin.sh`):
  the vertical glow edge seen there is static, NOT the owner's bug: same still vs mid-turn, TAA vs F4 off, sun shadows
  on/off. It is the tower lamps' range silhouette (column of 428k lm lights, range 29.3 m: bright right up to the
  Bevy-style window). The owner's transient rectangle bars were not reproduced; froxel history not ruled out.
- Owner pass 2026-10-21: not reproduced at Sadrith Mora 21:15 foggy; sun cascades changed in shadow reset A2/B3.

### WE-5 history: renderfix1 diagnosis 2026-10-21 (hypothesis only, superseded by the owner's cause)
- Diagnosis (renderfix1 2026-10-21, hypothesis only, no code): the ash/blight streaks (`sf_precip_bevy`, `PrecipMaterial`
  Blend, Transparent3d Overlay band) are depth-tested (stock Blend pipeline, `specialize` keeps Bevy's depth state) and
  live in boxes of at most 64 m around the camera, so they can't paint over terrain beyond ~32 m; near ones that cross
  the camera look "over" rocks but are in front (`shots/renderfix1/we5_ash.png`). Suspects: the ash look's dense height
  fog (visibility 450 m, fog height 500 m, sky dust 4 in `mw_weather` visual.rs) or the Stratocumulus deck, both
  composited by `sf_atmos_bevy` (FG-1 lane), e.g. sky-pixel fog/cloud on terrain silhouettes or depth mismatch. Next experiment: owner's view (F9 copy) with
  `--weather ashstorm`, A/B in one sitting `--off clouds`, `--off fog`, `--off froxels`; whichever removes the overdraw owns it.

## Moved from visual-checks.md on 2026-10-25 (vchecks-split)

## Distant shadows (superfargoth `weather.md`, "Near-term visual fixes", bug A)

### SH-12: Grazing-light edge shimmer (owner report, repeated)
- Kind: game location, any exterior with low sun (also Next up 6a "Grazing-light artefact", 2026-10-13).
- Seen by owner 2026-10-19 (round 2, again): object edges under grazing light shimmer and don't look solid.
  Suspects: no temporal AA on the raster path (aliasing of shadow terminators and alpha edges), normal-offset
  bias, dithered cascade end.
- Cause (owner A/B 2026-10-19, round 15, Balmora 17:30 toward the sun): **the clouds** (gone with `--off clouds`;
  still there with `--off softshadows` and `--off godrays`). The half/quarter-res checkerboarded cloud trace and its
  temporal resolve leak bright forward-scattered sky onto silhouettes (blocky fringe inside the meshes). A first
  depth-aware resolve/composite (uncommitted) removed the inner fringe but drew a stippled halo in the sky outside
  silhouettes, and the sky seems to wobble as the camera moves (owner: `Desktop/outline.jpg`). Research under way:
  `superfargoth/docs/research/clouds-upsampling.md`. (The Temporal shadow filter with TAA/DLSS stays: right pairing.)
- Fixed (superfargoth 8d8cfb4): trace marches to the furthest depth of a 6x6 window and stores the cut depth;
  resolve and composite match taps by population (open sky vs cut at the same depth); jittered history matrix.
- Status: **confirmed by owner 2026-10-19** (round 15: fringe, halo and sky wobble gone).

## Ghostfence (superfargoth `weather.md`, bug B)

### GF-2: Ghostfence energy field look
- Kind: game location. Run: `.\bevyrrowind scene ghostfence-near`
- Look for: a translucent blue barrier by day (`--weather clear`), a blue glow at night (`--hour 23`), never white.
- Diagnosed 2026-10-25 (ghostfence, no code change). Confirmed from the NIF: no vertex colours, no NiVertexColor/Stencil/ZBuffer
  property, alpha flags 0x00ED (sorted SRC/INV_SRC); each layer is two-faced geometry, so back-face culling is right.
  Tex alpha (DXT3, mean 0.65) × 0.5 gives 0.325 per layer, 69 % for three (not 87.5 %). The material mapping is faithful.
  Day 2026-10-25: translucent pale blue, not white. Fence-region mean 124,138,160 vs 167,183,205 in the 10-12 shot, which is stale.
  Night (confirmed): emissive 0.3 is exposure-weighted, so at the EV 3.5 floor it is ~3.4× "fully lit" with
  no Morrowind `min(1, …)` clamp. 17 % of the field is brighter than its texel colour, with white cores.
  Day hypothesis: Morrowind blends in gamma space and we blend in linear HDR, which hides detail behind pale layers (~6×).
- A/B: `--hour 23` vs 14, `--off envmap` (lowers the field and the terrain alike; not the cause).
- Status: **confirmed by owner 2026-10-25** ("looks amazing from up close", day and night): no emissive or blend change. Distance pop is GF-1.
- Source: farfield-exterior / weather-spec / nifpop.

## Physics (superfargoth `physics.md`, bevyrrowind `physics-adapter.md`; `.sessions/physics-p2.md`)

### PH-3 (P10): Robes and skirts drawn as authored (legs always covered)

Status: **confirmed by owner 2026-10-19** (Morrowind-style robes fine; rewritten p10s, 2026-10-19). V1 (simulated lattice on the girdle) FAILED, owner
2026-10-18: legs went missing when a foot stepped past the robe, and the robe didn't follow the legs. Owner decision
2026-10-19: full robe simulation is overkill, so robes and skirts now use their **authored thigh/calf skin** by
default, as in vanilla (physics-adapter.md "Cloth"; research `../superfargoth/docs/research/cloth/long-robes.md`).
The V1 simulation is the opt-in `--cloth robes` (then **H** toggles simulated ↔ kinematic as before).
Agent sanity run (p10s, BRP, dev build): both robes covered the legs at walk and at run, no `cloth:` log line.
- Kind: synthetic level. `.\bevyrrowind scene level-robe-walk` (TS-2: the temple priest's loop, Fargoth walking and
  Elone running over a ramp and stairs; cameras `priest`, `course`).
- Kind: lineup (no scene: `character` isn't a `view`). `.\bevyrrowind character "fargoth+common_robe_01" "@dark
  elf,,f+common_robe_02" --walk` (dev: `tools/run_dev.ps1 character ...`). Keys: **1** walk on/off, **2** run (again:
  walk), B smooth bodies. Optional A/B: add `--cloth robes` for the V1 simulated robes.
- Kind: game location. `.\bevyrrowind scene ph3-balmora-temple` (the priests in their own robes; fly down to them).
- Look for: **legs always covered**: no missing or floating leg at a walk, a run, on the ramp and stairs, from the
  side and behind; **the robe follows the stride** (its lower part moves with the thighs and calves, like the original
  game); no seam or gap at the waist; nothing stretched or inside-out. No hem swing is expected (that is S2, parked).
- Source: p10s (V1: p10c, p10d; shots `shots/p10d/`).

## Terrain (superfargoth `terrain.md`)

### TR-3: Terrain looks glossy, not rough enough (owner report)
- Kind: game location. Run: any outdoor view in sun, e.g. `.\bevyrrowind scene balmora` at 11:00, look at bare ground
  and grass at a grazing sun angle; A/B `--off envmap`.
- Seen by owner 2026-10-21 (bw-owner10, during AU-2): the terrain reads glossy / not rough, like wet ground.
- Check: the terrain material's roughness and specular (perceptual roughness near 1, no env-map or sun specular on
  ground), and whether the terrain blend/normal-map path lowers roughness.
- Status: confirmed by owner 2026-10-25 ("terrain is ok"; lanes `terrain-gloss` + `terrain-gloss-fix`; ROADMAP 1.12).
- Not the cause: roughness 0.85 far / 0.90 near (`surface.wesl:157`), metallic 0, F0 0.04, no wetness.
- Cause: terrain set only `diffuse_occlusion = s.ao` (Morrowind AO = 1): no SSAO, `specular_occlusion` stuck at 1.0.
- Fix: `terrain.wesl` now does StandardMaterial's occlusion (SSAO multibounce on diffuse, Lagarde specular occlusion
  from min(layer AO, SSAO)); the SSAO def reaches the terrain via the mesh view key, no Rust change.
- Seen by agent (`shots/terrain-gloss/b_on` vs `fix_on`, default 150 m camera): almost no change (mean luma -0.1),
  only a few creases darker. `--off envmap` (b_off) also drops env *diffuse*, so the "veil" may not be specular at all.
- Look for: at ground level in a valley, creases/slope feet less milky with SSAO on; A/B `--off ssao`. If open
  slopes still read wet, next is a roughness clamp (>= 0.95, owner taste call) or a specular-only env A/B.

## Animation lab (`levels/anim-lab.ron`, test-levels.md; `.sessions/anim-refactor.md`, slice 1)

### AN-19 Fargoth's walk has no per-step jolt (owner OK 2026-10-22, with rate_share 0.3)
Scene `level-anim-loop`, camera `loop-close` (Fargoth on the 12 m square, 1.4 m/s). Needs re-bake (`anim.cook` v8).
Look for: each planted foot stays put until it rolls off and lifts smoothly, no little snap forward as it leaves the
ground; the hips no longer swivel the whole body left/right each step; the body may surge a touch per step (as authored).
Compare a beast walker (Ajira) and Elone. Was: the male clip's root carried the hips' ±17° yaw and its feet skated
unevenly, so the lock let go of 12-19 cm each stance. Headless: `slope_walk walk_clip_stance_feet_keep_pace_with_the_body`.

### AN-21 Walkers shorten their steps uphill and lean into the slope (owner OK 2026-10-22 after "stronger": lean gains doubled, 30 deg +5/-2.4 deg)
Scene `level-anim-stairs`, camera `stairs` (lab ramps 10/20/30° and stairs). Needs re-bake (graph RON, graph format 6).
Look for: uphill, shorter quicker steps (about a quarter shorter at 30°); downhill, slightly longer ones (about +14%);
the upper body tipped a few degrees forward going up, a touch back going down, easing in over ~0.3 s, never a jolt;
feet still flat on the ramp. Flat ground looks exactly as before. Too much or too little: knobs in `mw_graphs`
build.rs (`StrideWarp grade up/down`, `Lean grade up/down/max`). Headless: `slope_walk walkers_shorten_their_steps_uphill_and_lean_into_the_slope`.

### AN-18 Ragdoll torso doesn't spin (confirmed by owner 2026-10-22 with AN-20; supersedes AN-11)
Scene `level-anim-idles` (camera `idles-close`): throw balls (`V`) at several dummies, then press `I` on one to kill it. Look for:
the torso stays one piece through the fall (no pelvis whirling, no back twisted round or folded past ~45 degrees over the whole
spine), the body comes to rest without a twisted waist, and arms, legs, hands, feet and head swing less loosely than before.
Re-bake first (ragdoll.cook v3). Known: limb cones are still symmetric (no separate forward/back hip or shoulder range yet).

### AN-11 Ragdoll spine bends like a spine (superseded by AN-18, confirmed 2026-10-22)
Scene `level-anim-idles` (camera `idles-close`): throw balls (`V`) at several dummies and press `I` on one to kill it. Look for:
a falling or dead body keeps a believable back (a gentle curve, 15 degrees of bend and twist per spine segment) and no longer
folds in half or corkscrews. Re-bake first (ragdoll.cook v2). Known trade-off: a hard tumble down stairs stretches a joint
up to 1.8 cm for a moment (gate relaxed from 1 to 2.5 cm).

### AN-16 Walkers take human-length steps at 1.4 m/s (owner OK 2026-10-22: "much better", then rate_share 0.5 -> 0.3)
Scene `level-anim-loop`, camera `loop-close` (Fargoth on the 12 m square, 1.4 m/s). Needs re-bake (`anim.cook` v7, graph RON).
Look for: a brisk cadence (about 1.1-1.2 s per two steps) with moderate steps, not slow lunging strides; legs not locked
straight behind the body. Was: the MW male walk (made for 2.1 m/s) played at 0.64x, 1.1 m steps, 76 steps/min.
Headless: `slope_walk walkers_keep_a_human_stride_and_plant_once_per_step` (male 1.66, Fargoth 1.77 leg lengths/cycle, was 2.21).

### AN-17 Elone's walk is smooth, no jolt per step (confirmed by owner 2026-10-22: "perfect")
Scene `level-anim-props`, camera `props-close` (Elone by the orange boxes, 1.4 m/s). Needs re-bake (`anim.cook` v7).
Look for: each foot plants once and stays until it rolls off; no twitch or snap of the left foot mid stance. Was: the
female walk's left contact dropped for one frame at the loop seam, so the lock released and re-planted every left step.

### AN-20: hit bodies loose, joints still in limits (confirmed by owner 2026-10-22: "good"; ragdoll.cook v5; limp_drop/stairs stretch gates 0.4-0.03 cm over, logged)
Scene `level-anim-idles`, camera `idles-close`. Knock an NPC down with the ball, then kill one limp. Check: arms and
head swing and flop as they fall (no wooden plank, no frozen arms), hips fold forward far but barely backward, shoulders
reach forward more than back, no elbow/knee bending backward, no spinning torso. Compare a stagger: brief give, quick recovery.

## Closed 2026-11-15 from visual-checks/weather-sky.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### WE-14: Stable cloud silhouettes (S1, clouds.md "Stable silhouette history"; follows WE-10, WE-13 horizon shimmer)
- Scene: `scene we3-ridge` at 14:00, clear-to-cloudy weather; then Seyda Neen rooftops against cloud. Still camera, then a
  slow pan. A/B: relaunch with `--off cloudedge` (old trace window on the traced pixel, raw sample on a history miss).
- Look at: the horizon line where no fog is behind it, and any ridge, rooftop or tree silhouette against cloud.
- Look for: no shimmering 1-2 px strip along the horizon or along silhouettes, still or panning; clouds near silhouettes
  as calm as the open sky.
- Must not: a halo or sky bleeding past silhouettes (bright cloud/sky fringe on the land side); FG-7 (fog behind ridges)
  or WE-12 (deck from above, fog split) looking changed.
- Status: confirmed (owner 2026-11-13, `scene we3-ridge` 14:00: "no shimmering whatsoever"; `--off cloudedge` A/B not run).

WE-14 | confirmed (2026-11-13: "no shimmering whatsoever") | [weather-sky](../visual-checks/weather-sky.md) | Stable cloud silhouettes (S1): `scene we3-ridge` 14:00 + Seyda Neen, horizon line with no fog behind and silhouettes against cloud, still then slow pan; A/B `--off cloudedge` (2026-11-13)

### FG-11: Froxel fog stays smooth over near objects (FOG-CLAMP, part of WE-10 shimmer)
- Kind: game location. Fix FOG-CLAMP (2026-11-12): froxel history clipped to this frame's 3x3 neighbourhood,
  weight 0.9 -> 0.95 (fog.md "Froxel history clip as built").
- Run: `bash bevyrrowind/tools/owner_launch.sh scene balmora --weather foggy --hour 6.5`; hold still 5 s on houses and
  hills 50-300 m away, then pan slowly, turn fast, and watch NPCs walking past. A/B: same with `--off fogclamp`.
- Look for: fog over near buildings and land smooth, no crawling grain still or panning; no ghost trails behind
  moving NPCs or their shadows, none on fast turns; no black bands (FG-1); fog density/colour as before.
  `--off fogclamp` should crawl more when still.
- Status: confirmed (owner 2026-11-12, `scene balmora --weather foggy --hour 6.5`: "the fog is glorious"; `--off fogclamp` A/B not run).

FG-11 | confirmed (2026-11-12: "the fog is glorious") | [weather-sky](../visual-checks/weather-sky.md) | Froxel fog smooth over near objects, still and panning; no trails or black bands (FOG-CLAMP, A/B `--off fogclamp`, 2026-11-12)

### WE-11: Weather covers water like land (W-ORDER, render-order.md "Frame graph"; fixes WE-7, WE-10 over sea)
- Kind: game location. `bash bevyrrowind/tools/owner_launch.sh view "Seyda Neen" --hour 9 --weather foggy`, then
  `--weather rain` and `--weather thunderstorm`; face the open sea (south-west, off the dock), still camera, then pan.
  High-up clouds: same `--weather rain`, free camera up ~300 m over the sea. A/B per effect: `--off fog`, `--off froxels`, `--off clouds`.
- Look for: fog thickening over the far sea as over land (no clear band at the horizon); the far rain curtain and storm
  continuing over water instead of stopping at the shore; from high up, a cloud between camera and ocean shows.
- Must not: water looking double-fogged (milkier/greyer than the land beside it); a halo or seam at the shoreline;
  refraction of the seabed and shore foam changed; stars drawn over water at night (`--hour 23 --weather clear`).
- Status: confirmed (owner 2026-11-12: fog over the water (`scene fg-bittercoast-ridge`); thunderstorm at Seyda Neen: fog,
  rain and lightning flashes carry on over the sea; clouds right seen from up at sea). Follow-up wish (later): show the
  lightning bolt itself, not only the flash.

WE-11 | confirmed (2026-11-12: fog, rain, storm over sea; clouds at sea; wish: visible lightning bolt) | [weather-sky](../visual-checks/weather-sky.md) | Weather covers water like land (W-ORDER): fog/rain/storm over the far sea, cloud over the ocean from high, no double fog or shore halo (2026-11-12)

### WE-12: From very high, the lower part of the clouds disappears (owner 2026-11-12)
- Kind: game location. Seen in `scene fg-bittercoast-ridge` after flying far above the cloud deck (after W-ORDER).
- Seen: high enough, the bottom of the clouds vanishes, as if clouds draw only against the sky. Re-seen 2026-11-12
  flying up into the cloud layer and trying to enter the clouds: their bottom part is missing. Clouds seen from up at sea
  (below the deck) are fine, so it is not WE-11's high-cloud item. Owner's precise case: deck at ~500 m, camera at
  ~600 m looking down: the cloud between the camera and the terrain/fog/objects behind it is not drawn; from below it
  is perfect. Flying into clouds is not a game feature, but very low clouds (and a volcano smoke plume, WISHLIST §5)
  would hit the same path.
- Suspect (not tested): the cloud march assumes a camera below or inside the deck, or stops at geometry depth when looking
  down, or at the deck's entry point.
- Cause (2026-11-12, WE12-DIAG, code): the composite applies the fog of the pixel's whole column over the cloud, so
  the dense low fog and banks *behind* the cloud hide it from above (`cloud_composite.wesl:112-135`). Fix: WE12-FIX.
- Fix WE12-FIX (2026-11-12): only the fog in front of the clouds dims them, the rest lies behind (UE5/Frostbite);
  far rain/fog under the deck now draws behind it when seen from above. Run
  `bash bevyrrowind/tools/owner_launch.sh scene fg-bittercoast-ridge`, fly ~100 m above the deck and look down at the
  cloud tops against the foggy ground, then fly down into the deck. Look for: the deck's full body visible over the
  terrain/fog from above, and its bottom still drawn when inside. A/B `--off cloudfog` (old: the bottom vanishes).
  Must not: the at-sea deck seen from below changed (horizon clouds may be slightly clearer: less fog past them,
  expected), the FG-11/FG-7 fog looks changed, a bright or dark halo at cloud silhouettes against terrain.
- Status: confirmed (owner 2026-11-12, `scene fg-bittercoast-ridge`: "works perfectly"; `--off cloudfog` A/B not run).

WE-12 | confirmed (2026-11-12: "works perfectly") | [weather-sky](../visual-checks/weather-sky.md) | Deck from above/inside: fog split at the clouds' depth (WE12-FIX); `scene fg-bittercoast-ridge`, fly ~100 m over the deck, look down, then into it; A/B `--off cloudfog` (2026-11-12)

### FG-7: Blocky crenellated edges on distant land in fog (owner report)
- Kind: game location. Seen 2026-10-30 (`scene balmora`): stair-stepped land silhouettes in fog, worse moving.
- Cause (2026-11-12, code): the half-res bank modulation is marched to the furthest depth of each trace window, so
  land pixels at a silhouette took the sky's value; open/cut flips under jitter dropped history (grid steps). Not froxels.
- Fix FOG-BANK (2026-11-12): bank at the window's nearest and furthest depth, blended per pixel by depth (fog.md
  "Depth-aware bank"). Run: `bash bevyrrowind/tools/owner_launch.sh scene fg-bittercoast-ridge` (6:30, foggy, 160 m up),
  then `scene balmora --weather foggy --hour 6.5`; still, then pan and walk. A/B: same with `--off fogbank`.
- Look for: land and hill silhouettes in fog smooth, no stair steps still or moving, no bright/dark halo along them,
  fog density and bank shapes unchanged elsewhere. `--off fogbank` should show the old steps.
- Status: confirmed (owner 2026-11-12, `scene fg-bittercoast-ridge`: blockiness gone).

FG-7 | confirmed (2026-11-12: blockiness gone) | [weather-sky](../visual-checks/weather-sky.md) | Blocky edges on distant land in fog: FOG-BANK fix, A/B `--off fogbank` (2026-11-12)

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

FG-9 | confirmed (2026-11-02: no pillar; god rays still not visible, FG-10) | [weather-sky](../visual-checks/weather-sky.md) | Vertical light pillar toward a low sun in fog: isotropic sky fill (fix built 2026-11-02)

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

SW-4 | confirmed (2026-11-06 retune: faster fill, 6° slope) | [weather-sky](../visual-checks/weather-sky.md) | Puddles, Balmora in rain: `surface set puddle 1`, flat sky-mirror water in hollows and grout, rings while raining that stop with the rain, none under roofs; A/B `--off puddles`

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

SW-3 | confirmed (owner 2026-11-05: works great, dries in a satisfying pattern; ground look a bit weird, see terrain roughness bug) | [weather-sky](../visual-checks/weather-sky.md) | Wetness after rain, Seyda Neen: `surface set wet 1`, darker/glossier ground and walls, streaks, dry under roofs; A/B `--off wet`

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

SW-2 | confirmed (2026-11-06, after the render-layer fix) | [weather-sky](../visual-checks/weather-sky.md) | Surface weather shelter at night (own top-down pass): roofs full, trees partial (`surface debug exposure`), 32 m cells, Balmora 23:00

### WE-8: Rain leans with the wind, capped, turning slowly (rain-slant, weather/wind.md "Precipitation tilt as built")
- Kind: game location. Run: `.\bevyrrowind view "Balmora" --hour 12 --weather rain`, then `--weather thunderstorm`
  (free camera, look across a street at the rain against a wall and the sky; wait a minute or two).
- Look for: rain near vertical in light wind (about 9-15° in Rain, up to ~25° in a thunderstorm, never steeper);
  the lean direction turns slowly (over a minute or two, no swinging); trees bend the same way the rain leans;
  the rain-under-roofs shelter follows the lean. Ash storms still blow near flat (uncapped).
- A/B: console (backquote) `flip rain_slant_cap`: off = the old raw lean (21-44°, turns with every wind change).
- Status: confirmed (owner 2026-11-06: "rain slant works").

WE-8 | confirmed (2026-11-06) | [weather-sky](../visual-checks/weather-sky.md) | Rain leans with the wind, capped (Rain ~9-15°, thunderstorm ≤ ~25°), direction turns slowly, trees agree; `view "Balmora" --hour 12 --weather rain`, A/B `flip rain_slant_cap`

## Closed 2026-11-15 from visual-checks/terrain-water.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### WA-4: Wavelets at the shore (W-SHORE; `scene wa-seyda-noon`, `wa-bittercoast`; A/B `--off shorenormals`)

Status: confirmed (owner 2026-11-12, `scene wa-seyda-noon`: "that's perfect"; `--off shorenormals` A/B not run). Kind: real game locations, High preset, the scenes' own time and weather.
Launch: `bash bevyrrowind/tools/owner_launch.sh scene wa-seyda-noon`, then `scene wa-bittercoast`; A/B: the same with
`--off shorenormals` (the old look: the normal flattens with the displacement). Fixes WA-1 (3) "100 % flat at the shore".
Look for: small wind ripples glinting and moving right up to the waterline on the Seyda Neen beach and the Bitter Coast
banks; the swell (big slow waves) still dying out over the last tens of metres, the surface itself not bobbing at the
sand; no flat mirror strip along the shore. Bad: shimmer, sparkle noise or moiré on the near-shore or far water; open-sea
chop on the beach; a visible line where the ripples start. Foam and reflections should look unchanged.

WA-4 | confirmed (2026-11-12: "that's perfect") | [terrain-water](../visual-checks/terrain-water.md) | Wavelets at the shore (W-SHORE): ripples glint to the waterline, swell still dies on beaches, no flat strip, no moiré; A/B `--off shorenormals` (2026-11-12)

### WA-3: Local reflections, SSPR (W-SSPR; `scene wa-vivec-waterfront` 15:00, `ebonheart-night` 22:00, `wa-seyda-noon`, `wa-night-moons` 23:30; A/B `--off waterssr`)

Status: confirmed (owner 2026-11-12: Seyda Neen thunderstorm 9:00 "very nice"; `ebonheart-night` 22:00: "the water is
correct now", no longer brighter than the land (LM-2), "very nice"). Open: owner unsure emissive surfaces (lit windows)
show in the reflection; `--off waterssr` A/B not run. Launch: `bash bevyrrowind/tools/owner_launch.sh scene wa-vivec-waterfront`, then
`scene ebonheart-night`; A/B: the same with `--off waterssr` (environment map only, the old look). High preset.
Look for: Vivec's cantons, Ebonheart's walls and docks, ships and the dark banks mirrored in the water below them and
wobbling with the waves (vertical streaks on rougher far water, not a hard mirror); at 22:00 lit windows, lamps and light
sprites reflected (the sprites are translucent: not in the reflection, known), water near dark banks dark, not airglow-bright.
Bad: smeared streaks or stretched copies at screen edges; a hard cut where a reflection leaves the screen (it should fade
to the sky reflection); flicker or crawling when still or panning; anything under the water (seabed, hull bottoms)
mirrored; reflections on a lake or pool at another level than the nearest one. Known: reflections are unfogged (the fog
over the water covers them), off-screen and occluded things fall back to the sky.

WA-3 | confirmed (2026-11-12: day and night "very nice", night water no longer too bright; emissive in reflection unsure) | [terrain-water](../visual-checks/terrain-water.md) | Water mirrors banks, buildings, ships and lit windows, wobbling with waves; edge fade, no flicker, nothing below the water (W-SSPR, A/B `--off waterssr`, 2026-11-12)

## Closed 2026-11-15 from visual-checks/shadows.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### SH-11: Distant flashes while moving (owner report)
- Kind: game location. Run: `.\bevyrrowind scene balmora`, fly around.
- Seen by owner 2026-10-19 (round 1): brief flashes in the distance whenever the camera moves; they look like shadows.
- Fix (render-opt 2026-10-28): ordered `far_map::update_far_map` before `update_terrain` and moved `fog_shadow_heights`, `far_shadow_input`, and `FarFieldSystems::FarShadow` to `PostUpdate` after `TerrainSystems::Update` so shifted terrain origins never render 1 stale frame; also replaced Bevy cascade-NDC PCSS with world-unit directional PCSS (OpenMW Clockwork adaptation).
- Status: awaiting owner (2026-10-28).
- Owner 2026-10-30 (scene balmora, flying): the distant shadow-like flashes are gone. Status: confirmed.

SH-11 | confirmed (2026-10-30) | [shadows](../visual-checks/shadows.md) | Distant flashes while moving (PostUpdate far-shadow ordering + world-unit PCSS)

## Closed 2026-11-15 from visual-checks/gi.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### GI-3: GI replaces the flat ambient inside its volume, Census office
- Kind: game location, High preset, raster. Run (dev exe): `powershell -File bevyrrowind/tools/run_dev.ps1 view "Seyda Neen, Census and Excise Office"`;
  A/B with `flip gi` in the console (on: probe bounce, no cell ambient inside the volume; off: today's cell ambient).
  Log: `gi: volume traced, sampled by the PBR shader (DDGI probes)`; no `naga`/`wgsl`/validation error.
- Look for: with GI on, rooms read darker in unlit corners and warmer near lamps than before (GI-1/2 runs); no hard
  box edge visible inside the office; nothing black or flickering. If corners go too dark, say so: the game's
  `INTERIOR_AMBIENT_SCALE` (0) can take a small floor. Known limit: dark probe patches (no relocation yet).
- Tune live: `render.gi_intensity 2` (try 1-4), `render.gi_ambient 0.1`; report the value you like.
- Status: confirmed (owner 2026-11-02: looks good; defaults kept, `render.gi_intensity` 1, `render.gi_ambient` 0).

GI-3 | confirmed (2026-11-02: looks good; defaults kept, gain 1, ambient 0) | [gi](../visual-checks/gi.md) | GI replaces the flat cell ambient inside its volume: darker unlit corners, warmer near lamps, no box edge in the office (`flip gi` A/B)

## Closed 2026-11-15 from visual-checks/physics.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### PH-15: Knockdowns of human NPCs are Delirium falls (owner 2026-10-30/11-05: the old ragdoll still played)
- Kind: level + game location, default build (`delirium` on). `cargo build -p bevyrrowind --features dev`, then
  `powershell -File tools/run_dev.ps1 scene level-anim-idles` (Llathyno, idle-5: 25 N·s every 12 s; `level-anim-drop`: 10 s).
- Also: F11 free camera, `V` balls (15 N·s at full throw) at a Dunmer/Breton walker, then at Ajira, Huleeya and the guar.
- Look for: a human NPC hit goes down under Delirium (`catchFall`: hands out, no pop from the clip), lies still, rolls
  if needed, gets up and goes back to its idle or path. Ajira/Huleeya/guar keep the old clip + powered-ragdoll knockdown.
- Log: `knockdown: <e> taken over by the physics layer`, `delirium: took over ... with catchFall`, `handed back (Fallen)`;
  beasts/creatures `ragdoll: knocked down`. A/B: `--no-default-features` build = old ragdoll for everyone.
- Status: awaiting owner (delirium-knockdown, 2026-11-05).
- Fix 2026-11-05 (delirium-hits; owner: "push them in the direction, as the click did"): hits are pushes with the balancer
  on, no forced fall: a `V` ball (up to 36 N·s head-on) staggers the NPC 0.3-0.5 m along the ball's path with a catch
  step, then it blends back to its idle; a second ball on a staggering or lying NPC pushes it again. Only 100+ N·s
  falls for sure (after 0.3 s of push). Log: `hit: <e> taken by the physics layer`, `took over ... with bodyBalance`,
  `recovered, blending out`. Same feel as the F11 click (Ctrl 20 / plain 45 N·s). Status: awaiting owner.
- Owner 2026-11-05: confirmed (hits push along their direction). Follow-ups: fall threshold should scale with the
  character's strength (stats); bodies sink into bent knees on takeover and rise at hand-back; balls pass through arms
  outside the capsule (hurt shapes, C4). Status: confirmed with follow-ups.
- Fix 2026-11-05 (knee-sink): the balancer now stands at the takeover pose's leg extension. Delirium counted the actor's
  scale twice (x0.9 stood 9 cm low, 45 deg more knee; x0.95 5 cm; x1.0 1 cm), now 0.4-0.8 cm and 5-10 deg at every scale.
  Check: `scene level-anim-idles`, F11, small `V` balls (or a Ctrl click) at Fargoth, Llathyno and a Breton/Dunmer woman:
  no knee sink as Delirium takes over, no rise as it blends back to the idle. Status: awaiting owner.
- Fix 2026-11-05 (shapes-s3, character-shapes S3): a held body has kinematic proxies (Delirium's 21 parts) that balls hit.
  Check: `scene level-anim-idles`, F11, rapid `V` balls at a staggering, then a lying NPC: each ball bounces off the limb
  it hits and pushes the body again (no ball through it); after the hand-back balls hit the capsule as before. Log:
  `hit: <e> taken by the physics layer (... N·s, held)` once per ball. Status: awaiting owner.

PH-15 | confirmed (2026-11-05: knee sink fixed; rapid balls push held bodies each time, S3) | [physics](../visual-checks/physics.md) | Knockdowns of human NPCs are Delirium falls + get-ups (V ball, `@impact`); beasts and creatures keep the old ragdoll

### PH-2 (first half, P7): a dead guard ragdolls on the Balmora stairs
- Note 2026-10-25: `sf_ragdoll` tests `limp_drop` and `settles_on_stairs` fail on their stretch limit (pre-existing).
  After this check they become sanity guards + printed measurements (CONTRIBUTING.md "Tests and visual checks").
- Kind: game location. Run: `.\bevyrrowind view "Balmora"` (release, physics on), fly (free camera) to a guard on
  the stone stairs (the High Town steps up to the Council Club / Hlaalu manor), put the crosshair on him and press
  `I`. He plays his death clip for 0.4 s, then goes limp. Then hold LMB on a hand or foot to drag the body (a 75 kg
  corpse is a drag at the default Strength 50: the camera slows), release to drop.
- What to look for: the clip-to-ragdoll cut is not a pop (velocities carry over); the body tumbles or slumps down the
  steps without exploding, jittering or sinking into them, then rests within a few seconds; limbs bend the natural
  way (knees back, elbows forward) and stay attached (the log's `ragdoll: joint stretch peak` stays under 1 cm);
  the dragged corpse follows by the grabbed limb. Creatures too (a guar, a cliff racer: `I` on them).
- Press F7 + G to see the skinned-mesh wireframe (yellow) over the ragdoll capsules (orange) and joints (magenta):
  the mesh must follow the bodies (heads are not drawn: morph-target meshes crash Bevy's wireframe pass). Both keys are in the F1 overlay.
- Known (P8): no powered crumple yet (the clip cuts to limp); the character root stays where it died (a corpse
  dragged far can be culled early); corpses don't persist across cell unloads; creature masses are volume
  estimates (a guar is 1 t: too heavy to drag).
- Agent run 2026-10-18 (p9fin, release, `scene wk-balmora`, no keyboard):
  `$env:BEVYRROWIND_AUTO="kill_nearest@3,shot:shots/p9/kill.png@4.5,exit@8"`. Log: `ragdoll: killed ...
  (meshes/base_anim.nif)`, 1 spawned, 0 failed; joint stretch peak 1.4–2.1 cm (over the 1 cm target: open, P8b).
  Sanity shot `shots/p9/kill.png`. Not on the stairs (wk-balmora's camera); creatures not run.
- **Lead (owner 2026-10-19, round 11, with `G` collider view):** the ragdoll's physics capsules behave normally when hit; it's the skinned mesh that doesn't follow them. So the solver is fine and the bug is the pose write-back (bodies → bones: bone/body offset, local vs world frame, or the floating-origin offset), which would explain sinking, floating, spinning and the odd joint angles.
- Earlier status: rejected by owner 2026-10-19 (round 10): ragdolls very bad: most bodies sink into the ground, float and turn around; joints look unconstrained (unnatural angles); they don't explode. Drag doesn't match the expected Oblivion-style hold/drag UX (later). Phase 3.
- **R0 fix (anim-refactor, 2026-10-22):** the clip's root still placed the mesh (joints above the pelvis body had physics
  weight 0, and root motion kept moving the frame); non-uniform race build scale skewed lying limbs; foot placement
  bent ragdoll legs. Headless test now holds every bone on its body within 0.1 mm while the frame moves, yaws and
  rebases (`sf_physics_bevy` `bones_follow_bodies_*`). Not checked live by the agent.
- Re-check: `scene level-anim-drop` (AL-6), press `G`: the mesh sits inside the capsules during the fall, the knockdown
  and at rest (no sinking, floating or spinning; limbs follow their capsules); `I` on the walker, then on a Balmora
  guard on the stairs as above. Expect a slight width change at death on races whose build ≠ height (uniform scale
  while ragdolled, restored at get-up). Status: **awaiting owner**.
- Owner 2026-10-30: no explosion and the death clip plays, but this clip-then-limp ragdoll path is deprecated: deaths
  and knockdowns use the delirium system from now on, so this check is retired. Dragging is too hard and works badly
  (later). Bug seen: `I` in third person kills the player (the crosshair ray hits the player) and also opens the
  inventory (key clash): logged as PH-14. Status: retired (superseded by delirium).

PH-2 | retired (2026-10-30: clip+limp ragdoll deprecated, delirium from now on) | [physics](../visual-checks/physics.md) | a dead guard ragdolls on the Balmora stairs

### PH-14: Dev kill key `I` kills the player in third person and opens the inventory (owner report)
- Kind: game location, dev build. Third person, crosshair on an NPC, `I`.
- Seen 2026-10-30: the player dies (the crosshair ray starts behind the player and hits its own body first) and the
  inventory opens too (`I` is also the inventory key).
- Fixed 2026-11-05 (delirium-death): the `I` key is gone; the console's `kill` replaces it and only targets walkers
  (never the player or the training dummy). Status: fixed, awaiting owner (check with PH-16).

PH-14 | confirmed (2026-11-05) | [physics](../visual-checks/physics.md) | Dev kill key I kills the player in third person and opens the inventory (2026-10-30)

## Closed 2026-11-15 from visual-checks/lights.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### LM-2: Leaf side findings (from LM-1, renderfix1)
- With `--off leafmask`, blended leaves vanish entirely at ~100 m+, so the A/B no longer shows "blended as before".
- Leaves (and water) look bright white at night (`shots/renderfix1/night_amb4.png`, Balmora): not traced; hypothesis
  is still-Blend leaves on the forward FroxelFog path.
- 4 textures could not be decoded, so they are never promoted to cutouts: `poxyclouds2`, `tx_banner_temple_04`,
  `tx_eggmine_banner*`, `tx_rope_net_01`.
- Fix (render-opt 2026-10-28): propagated NetImmerse `NiVertexColorProperty::LightingMode` (`Emissive` vs `EmissiveAmbientDiffuse`) and `NiTexturingProperty::ClampMode` through `mw_nif`, `sf_model`, `mw_bake`, and `mw_assets`, and added 16-bit `A1R5G5B5`/`A4R4G4B4`/`B5G6R5` DDS decoding in `mw_bake/src/textures.rs` for cutout promotion. Re-bake `mesh.cook` / `tex.meta` if using an older bake store; check `view Balmora --hour 22`.
- Status: awaiting owner (2026-10-28).
- Owner 2026-10-30 (`scene hi-balmora-night`): leaves look okay; the water is still brighter than the rest at night (not
  a moon effect). Status: partial (water at night open). 2026-11-12: water at night correct after W-SSPR (owner, `ebonheart-night`, WA-3).

LM-2 | confirmed (2026-11-12: leaves OK; night water correct after W-SSPR) | [lights](../visual-checks/lights.md) | NetImmerse vertex lighting mode, clamp modes, and 16-bit DDS cutouts

## Closed 2026-11-15 from visual-checks/rendering.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### RO-14: Bloom on the game camera (2026-11-01)
- Kind: game location, dev or release build, High preset. Runs: (1) `view "Balmora" --hour 18.5`, face the sun
  (west); (2) `view "Balmora" --hour 23`, look at the street lamps and braziers; (3) `scene census`, the candles.
- Look for: a soft glow around the sun disk, lamp and candle flames and fire only; lit walls and sunlit ground keep
  crisp edges, no milky haze over the whole image; no pulsing when the camera stops (DLSS/TAA run before bloom).
- A/B: `--off bloom` (or console `render.bloom 0`; retune live, e.g. `render.bloom 0.2`; default 0.1).
- Status: confirmed (owner 2026-11-02, sun at dusk; night lamps and candles not checked). Source: visual-niceties.

RO-14 | confirmed (2026-11-02: sun at dusk; lamps and candles runs not checked) | [rendering](../visual-checks/rendering.md) | Bloom on the game camera: sun, lamps, candles glow, lit walls stay crisp (`--off bloom` A/B)

### RO-15: Light through leaves (foliage diffuse transmission, 2026-11-01)
- Kind: game location. Runs: `view "Seyda Neen" --hour 7` looking east and `view "Balmora" --hour 18.5` looking
  west, from under or behind trees so the sun sits behind the canopy; then turn around (sun at your back).
- Look for: backlit leaves glow warm/green-gold, leaves shadowed by the trunk or other trees stay darker; front-lit
  leaves look as before or a touch darker (half their direct light goes through); no bright rims or speckle on
  cutout edges; far impostor trees are unchanged (not covered).
- A/B: `--off foliagetransmission` (startup only; value `mw_assets::LEAF_DIFFUSE_TRANSMISSION` 0.5 (owner 2026-11-02, was 0.3), thickness 0.05 m).
- Status: confirmed (owner 2026-11-02 at 0.5; 1.0 tried and liked). Source: visual-niceties.

RO-15 | confirmed (2026-11-02: 0.3 too faint, 1.0 liked, kept 0.5) | [rendering](../visual-checks/rendering.md) | Light through leaves: backlit canopy glows, front-lit unchanged (`--off foliagetransmission` A/B)

### TX-delight: De-lit albedo textures (bake layer, 2026-11-05)
- Kind: game location. Needs a current `bevyrrowind bake` (start log: `delight: layer mounted, N textures`).
  Runs: `view "Seyda Neen, Census and Excise Office"` and `view "Balmora" --hour 14` facing a lit house wall;
  A/B by restarting each with `--off delight` (log `delight: off`).
- Look for: corners and edges no longer double-dark under real light; walls not flat or cartoony; no colour shift;
  no blocky BC7 artifacts on gradients; cutout foliage (hedges, tree cards) not thinner at distance (coverage mips).
- Strength 0.5 (owner 2026-11-05) is baked in: a new strength needs a re-bake.
- Status: confirmed 2026-11-13 (owner: "textures seem great"). Source: rtx-borrow D-2b.

TX-delight | confirmed 2026-11-13 (owner: "textures seem great") | [rendering](../visual-checks/rendering.md) | De-lit albedo textures: no double-dark corners, no colour shift or BC7 blocks, cutouts keep coverage (`--off delight` A/B)

### TX-2: Terrain layer normals + ARM from packs (textures T5, 2026-11-13)
- Kind: game location. Needs a fresh `bevyrrowind bake` with the packs mounted (`mvr-pbr`, `tamrielic-pbr-west-gash`;
  bake line `terrain layers: N, n with a normal map, a with ARM`, n > 0; `terrain.layers` re-runs, step v3).
- Runs: `scene tx2-terrain-normals` (Balmora street, RO-16 camera, 07:00 low sun), `scene tx2-west-gash` (hillside west
  of Balmora, sun to the right), each also with `--hour 18`. No `--off` A/B for terrain normals yet.
- Look for: ground relief catching the low sun on the side facing it; bumps not inverted (the lit side faces the sun);
  no shimmer or sparkle at 10-50 m (arrays are still 256², T6 raises them); rock darker in cracks, grass matte, rock
  not glossy; and whether RO-16's even resin-like sheen breaks up.
- T6 (2026-11-13): arrays now BC7/BC5 up to 2048² (High/Medium load 1024² under the budget; log `terrain layers: N x S², X MiB`):
  look for sharper ground at 2-20 m, no blockiness or colour banding (BC7), no shimmer at distance, normals still right.
- Status: CONFIRMED owner 2026-11-13 (tx2-terrain-normals, after T5+T6 re-bake): "works well"; asked for parallax on the ground too (tx-tpom, queued). Source: textures T5 (`.sessions/tx-t5.md`), T6 (`.sessions/tx-t6.md`).

TX-2 | confirmed 2026-11-13 | [rendering](../visual-checks/rendering.md) | Terrain layer normals + ARM from packs: ground relief under low sun, no inverted bumps, no shimmer; does the RO-16 sheen break up (`scene tx2-terrain-normals`, `scene tx2-west-gash`; re-bake with packs)

### RO-11: Black bands on the ground that move with the camera (owner report)
- Kind: game location. Seen 2026-10-30 from the release start (Seyda Neen, New game, High defaults), walking outside.
- Look for: a pattern of dark bands on the terrain that slides as the camera turns or moves.
- Suspects (not tested): SH-14 contact-shadow bands (`--off contactshadows`), FG-1 fog history (`--off foghistory`), GTAO
  under TAA/DLSS (console `ssao`). First step: those three A/Bs at the same spot.
- Status: known bug: cause not diagnosed (owner pass 2026-10-30).
- Owner 2026-10-30: the same shadow-band look also shows in the anim-lab test levels (flat synthetic boxes, no terrain),
  so it isn't terrain-specific: a good place to A/B it.
- Owner 2026-10-30, `scene wk-balmora` (release copy, `--resolution 2560x1440`): bands still there but weaker. Clue: the
  release start (no subcommand) may run different settings (DLSS/preset) than `scene` runs; diff them first.
- Owner 2026-10-30: the bands look like a regression from the very recent shadow work (2026-10-28: d89eaa0 world-unit sun
  PCSS, 39f6508 shadow/LUT changes). First step: A/B or bisect those two commits.
- Audit 2026-11-01 (`superfargoth/docs/research/shadows/2026-11-01-audit.md` §3): release start = `view` defaults, no settings
  diff. Leads: GTAO at output resolution under DLSS (bevy#26234, not vendored), contact shadows marching full-size depth under
  DLSS (`pbr_functions.wesl:308`), aerial LUT samples 10 → 6 (39f6508). A/B: `--dlss taa`, `--off contactshadows`, `ssao`.
- Built 2026-11-01 (shadow-fixes; spec `shadows.md` "Shadow fixes as built"): contact-shadow march in main-pass pixel space
  under DLSS, full receiver slope + half-texel margin on the sun's far 4-tap and PCSS paths, bevy#26234 GTAO at the
  main-pass resolution. Owner copy `%TEMP%\bw-owner-2026-11-01`: `01-new-game.cmd` (fixed), then if bands remain
  `11-…-no-contactshadows`, `12-…-no-softshadows`, `13-…-taa`, `14-…-no-ssao` (same start, one part off each).
- Owner 2026-11-01 (`01-new-game`, `11-…-no-contactshadows`): no ground bands in either. Status: **confirmed** (which of the
  three fixes did it is not isolated; all three stay).

RO-11 | confirmed (owner 2026-11-01: gone) | [rendering](../visual-checks/rendering.md) | Black ground bands that move with the camera (2026-10-30)

### RO-20: Far terrain shadows keep following after the far-map upload fix (perf-push P2, 2026-11-12)
- Why: the far shadow map upload no longer touches every terrain, proxy and impostor material to "re-bind" (Bevy writes the
  same texture in place, so bind groups stay valid); risk is a stale far map if that reasoning is wrong.
- Kind: game location. Run: `scene balmora-sunset-shadows` (Balmora 18:30), then fly (W, Shift) 2-3 km in one direction.
- Look for: hills past ~400 m keep shadows that match the low sun as you go; after ~1 km of travel the far shadows around
  you refresh (no shadows left hanging where you were, no unshadowed hills ahead). A/B: `--off farshadows` drops them.
- Status: **confirmed 2026-11-12** (owner, clear weather): hills keep shadows, they update as you approach. New, logged: subtle shadow swimming on hills while approaching (cause unknown: far-map recentre texel shift or cascade handoff are the suspects; not chased).

RO-20 | confirmed 2026-11-12 (subtle shadow swimming on hills while approaching, logged) | [rendering](../visual-checks/rendering.md) | Far terrain shadows keep following the player after the far-map upload stopped re-binding materials (2026-11-12)

### RO-21: Haze and ash change smoothly with no medium re-bake (perf-push P1, 2026-11-12)
- Why: weather no longer re-bakes the sky medium on the CPU (a ~4.3 ms render-thread hitch about every 1.3 s); haze,
  dust and dust tint are now live scales in the atmosphere uniform (vendored `bevy_pbr` patch P1). Same look by design.
- Kind: game location. Run: `scene fg-redmtn-clear`, press P (clock on) and T (time-lapse), then C until foggy, then
  ash storm. Then `scene fg-redmtn-foggy` and `scene fg-redmtn-ashstorm` as still references.
- Look for: haze thickens and the grey-brown ash tint arrives smoothly, no step, flash or pop in sky or far hills; the
  still scenes look as before (foggy milky white, ash storm grey-brown with a dimmer, warmer sun).
- Status: **confirmed 2026-11-12** (owner): all weather transitions smooth. Owner wish: an even faster time-lapse preset.

RO-21 | confirmed 2026-11-12 | [rendering](../visual-checks/rendering.md) | Haze and ash tint change smoothly in weather transitions, same look as before, with the medium no longer re-baked on the CPU (perf-push P1, 2026-11-12)

## Closed 2026-11-15 from visual-checks/combat.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### CB-1: Light chain and roll on the dummy (C1b)
- `.\bevyrrowind scene level-arena` (or `scene level-arena --camera chain`). Keys: F readies / sheathes the weapon (an attack press while
  sheathed draws instead); LMB or pad right bumper attacks (four presses = the light chain); Shift or pad East: tap rolls (on release), hold
  sprints; O toggles the combat gizmos (hurt capsules green, cyan in i-frames; blade yellow, red while a hit window is open).
- Look for: hits land when the blade arrives at the dummy (not before); hit-stop reads as impact, not a stall; the dummy's reaction matches the
  side; the stamina (green) bar drains per swing and refills after; the roll moves ~2 m with no slide at its end; sprint stops when winded.
- Tune in `bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron` (windows, hit timings, hit-stop, costs, weapon segment) and the slot clips
  in `mw_graphs/data/animation/sources.ron` (re-bake).
- Known gaps: no weapon mesh (the stance is gameplay only); medium and large reactions share the knockback clip at full travel (no
  per-tier root scale yet); no pad default for ready weapon; the dummy is talkable (dialogue picks up every NPC character).
- Agent live check (2026-10-27): dummies stand on the floor, light1 lands Small front hits (100 -> 90 -> 80), light3's second hit lands
  Medium; gizmos draw capsules and the blade at the hand. Roll and sprint not confirmed live (BRP input).
- Owner 2026-10-27: animations work well. Bugs: (1) holding Shift never sprints, every press rolls; (2) the chain walks the player
  forward too far (UAL2 attacks carry 0.8-1.6 m root; ER steps ~0.3-0.5 m: root scale per move + capsule collision); (3) mouse
  not captured in 3rd person (windowed: cursor shows, a click lands in another window).
- Fixed 2026-10-27 (afternoon): hold sprint (a stray unbound Sprint release cancelled the shared Shift / pad B press, `sf_action`
  `InputSampler::release`); cursor confined on Windows; light chain ~0.4 m per swing (per-move `root_scale` in `sword_1h.ron`); body stays
  on the capsule through the chain (`sf_anim` cook `start_anchor`: UAL2 split-take clips kept a ~0.95 m pelvis offset); player vs dummy
  collision (`separate_moved`: the mover stops/slides, the dummy is never shoved). Owner confirmed all live (copy c, arena).
- Status: confirmed. Owner 2026-10-27 evening (copy d): the 4-hit chain (UAL2 `Sword_Light_A-D`, combat.md "C1b" table) and M1 (cursor
  stays hidden on a left click and in F11) confirmed.

CB-1 | confirmed (2026-10-27: sprint hold, chain step, mouse capture, dummy collision fixed; evening: 4-hit chain and M1 cursor confirmed) | [combat](../visual-checks/combat.md) | Light chain and roll on the training dummy (scene level-arena)

### CB-2: Lock-on, locked facing and side rolls (C1c parts 1-4)
- `.\bevyrrowind scene level-arena`. Q (or middle mouse, pad R3): lock the dummy in front; Q again releases; Q with nothing in the
  15 m / 35 deg cone recentres the camera. Then walk away past 20 m (breaks), mantle the 1 m ledge (breaks, re-offered 1 s after landing).
- Look for: the camera frames the target; a small white dot (dark rim) sits on the locked dummy's chest, gone when unlocked or off screen;
  locked, the body keeps facing the dummy: A/D circle it, S backs away facing it, standing still turns to it, sprint faces its run;
  locked Shift tap + A/D = a side dodge (~2 m around it, no pause at its end); W/S + tap = the forward roll squared to the stick.
- Known gaps: locomotion still plays the forward cycles (strafe clips C1c-2b); no back roll / backstep clip; no target switching (C4).
- Owner 2026-10-27 (copy d): lock, release, recentre and locked facing confirmed ("main logic solid"); asked for the marker and faster
  side rolls (both built after, 2709b71, ddc252d). Status: lock + facing confirmed; marker and side rolls awaiting owner.
- Owner 2026-10-30 (dev3 copy): marker, side rolls and the rest work great. Status: confirmed.

CB-2 | confirmed (2026-10-30) | [combat](../visual-checks/combat.md) | Lock-on, locked facing and side rolls around the dummy (scene level-arena)

### CB-3: F never draws a sword (owner report)
- Kind: synthetic. `scene level-arena` or any anim-lab level, player (F11).
- Seen 2026-10-30: F (ready weapon, `combat.rs` `ready_weapon`) never shows a drawn sword, and never did. Unknown whether
  `Stance` toggles without a visible weapon or the action never fires.
- Fixed 2026-11-05 (stance-visible): F did toggle `Stance`, but nothing showed it: the avatar never got
  `CharacterReadiness`, and the `player` record holds no weapon. Now the avatar follows the stance, levels `arena` and
  `anim-lab` equip `iron longsword` (`player_held`), and a sheathed click or right click does nothing (no implicit draw).
- Run: `cargo dev scene level-arena` (F11: player). Look for: no sword in hand at start; LMB / RMB do nothing; F: the
  sword appears in the right hand and the idle / run become the one-handed set; attacks swing it, the blade follows the
  hand; F again: the sword vanishes. Real cells: the player has no weapon yet (no equip).
- 2026-11-05 (draw-clips): F plays Morrowind's one-handed draw / sheathe (`WeaponOneHand` equip / unequip) on the
  upper body; the sword appears in the hand at the clip's attach key and vanishes at its detach key. Look for: walking
  or running while drawing keeps the legs going (upper body only); the sword shows as the hand reaches it (not at the
  press, not after the clip); LMB during the draw does nothing, LMB after it swings; F again: hand goes back, sword
  vanishes mid-clip, LMB refused from the press. Log line: `combat: draw .. s (attach at ..)`. No sheath model (original).
- Status: confirmed 2026-11-05 (owner: "the sheathe thing works well"). Follow-up: the sword attack clips (Quaternius UAL Sword_Light_*) are poor; generate new ones (3.8 clip factory).

CB-3 | confirmed (2026-11-05; follow-up: new sword attack clips) | [combat](../visual-checks/combat.md) | F draws a visible sword; sheathed clicks do nothing; draw / sheathe clips, sword at the attach key (2026-11-05)

### CB-3a: Sparring dummy strikes; player reactions in both views (C2a, 2026-11-10)
- `cargo dev scene level-sparring` (cameras `behind`, `side`), F11 player, F to draw. The sparring dummy (Fargoth, sword drawn) stands
  3 m ahead and swings light1 every 2.5 s while you are within 2 m (`sparring: the player: hit ...` in the log); `dummy` 6 m left
  stands still. Run third person, then Tab to first person (view model off, then `on viewmodel`).
- Feel for: step in: a steady beat you can learn; each hit lands as the blade arrives, hit-stop on both, and your body plays the
  reaction for the side it came from (turn your back, side-on); health -10 per hit; at 0 you refill (non-lethal bout). Step back past
  2 m: it never swings. Roll through its swing: no hit. First person: the view stays free in a reaction (no wall), the arms follow
  the flinch, no camera kick yet (C4 shake).
- Tune: `mw_combat/data/sparring.ron` (first, period, reach); reaction slots in `mw_graphs/data/animation/sources.ron`.
- Known gaps: Medium/Large reactions front only, one clip; no guard (C2b) or knockdown (C2c).
- Status: confirmed 2026-11-12 ("all good"). Logged: the roll starts late (Shift tap waits to tell a tap from a sprint hold), hard to time
  through a swing; first person looking down shows the inside of the body (chest clipping the eye).

CB-3a | confirmed 2026-11-12 (logged: late roll start, first-person chest clipping looking down) | [combat](../visual-checks/combat.md) | Sparring dummy strikes on a beat; player hit reactions by side in third and first person, free look in a reaction (scene level-sparring, 2026-11-10)

## Closed 2026-11-15 from visual-checks/anim-lab.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### AL-1: Animation lab overview
- `.\bevyrrowind scene level-anim-overview`. Look for: six areas apart (ledges + props near the origin, loop south, slopes/stairs west-north,
  drops north-east, idles south-west), nothing sunk or missing, every NPC dressed. Status: awaiting owner.
- Owner 2026-10-30: works perfectly. Status: confirmed.

AL-1 | confirmed (2026-10-30) | [anim-lab](../visual-checks/anim-lab.md) | Animation lab overview

### AL-4: Traversal heights (future vault / mantle / climb)
- `scene level-anim-ledges` (cameras `ledges`, `ledges-close`); F11 for the player, walk into each box and the 3 m wall.
- Look for: the capsule stops at each ledge and the body does not clip into the box (no vault, mantle or climb exists yet: this is the
  baseline for them); heights readable against the 1 m grid. Status: awaiting owner.
- 2026-10-30: retired (written before mantles existed; AL-10 and AL-11 cover traversal now). Status: retired.

AL-4 | retired (2026-10-30: superseded by AL-10/AL-11) | [anim-lab](../visual-checks/anim-lab.md) | Traversal heights

### AL-6: Falls and ragdoll (PH-2, PH-9)
- `scene level-anim-drop` (cameras `drop`, `drop-side`, `drop-close`); `G` shows collider wireframes, console `kill` kills the NPC under the crosshair, `V` throws.
- Look for: the walker stepping off the 3 m edge (a fall animation or a snap? it must not freeze), the dummy knocked down every 10 s
  (`@impact` 25 N*s: ragdoll, then get-up), with `G` the capsules following the mesh, no pop into the ragdoll, no spin on the 15 deg slope.
  No 6 m fall exists for actors (no way to place one on the platform): throw or fly. Status: awaiting owner.
- 2026-10-30: retired with PH-2 (owner: the clip+limp ragdoll is deprecated, delirium from now on). Status: retired.

AL-6 | retired (2026-10-30: superseded by delirium) | [anim-lab](../visual-checks/anim-lab.md) | Falls and ragdoll

### AL-10: High mantle, player (action.md B2 high mantle)
- Needs a bake after 2026-10-27 (`traverse.mantle.high` on UAL2 `ClimbUp_2m`). `scene level-anim-ledges`, F11; walk W into a box, W+Space.
- Look for: the orange h1.60 and h2.00 boxes play the high climb (hands above the head, body over the lip onto the top); the yellow
  h1.20 box keeps the waist mantle; the red h2.50 box and the 3.5 m wall refuse into a plain jump; Ctrl + Space never mantles.
- Owner 2026-10-27 (copy d): works, hands go above the head. Issues: the hands aim at the box centre, not the lip (B2b-2 hand IK, after
  the B2b-2a contacts); hard to trigger: the body slides along the face and doesn't face it (B2-T approach tolerance + yaw align, in
  progress).
- Owner 2026-10-30, after B2-T (99f6466) and B2b-2b/2c (130bf38, e5224f2): the mantle triggers and works; the hands reach the lip
  but the hand IK needs tuning. Status: confirmed (hand IK tuning logged on ROADMAP 3.1).

AL-10 | confirmed (2026-10-30: triggers and works; lip hand IK needs tuning, ROADMAP 3.1) | [anim-lab](../visual-checks/anim-lab.md) | High mantle, player (ClimbUp_2m on the 1.6/2.0 m boxes)

### AL-11: Wall brace, player (action.md B2c)
- No bake needed (runtime feed only). `scene level-anim-ledges`, F11; the grey 3 m wall behind the boxes (14 m long, z -7).
- Run (W) straight into the wall: both hands rise before the impact and plant on the face, arms slightly bent, palms on
  the wall (not floating more than ~2 cm, no fingers or forearm through it); release W: the hands let go over ~0.5 s.
- Walk along the wall about 0.5 m off it (turn parallel, W): only the wall-side hand trails lightly on the face, behind
  the shoulder; walking away: it lets go. Standing still: no brace. Near the wall's end the hand drops past the edge.
- F (sword drawn): only the left (off) hand braces. Mantles: the lip grip still wins (AL-10 unchanged).
- Known: the hand keeps its animated rotation (no palm frame yet), so fingers may angle oddly. Status: awaiting owner.
- Owner 2026-10-30 (release copy, 13:08 build): fails. Running into the wall, the character starts walking sideways along it,
  doesn't stop, and the hands never rise. F never draws a sword (never did), so the drawn case is untested.
- Fix 2026-10-30 (wall-brace-fix): the brace never ran (its gate waited for "no action slot", but plain locomotion always
  has one); now gated on no Clip/Warp move. Live dev build: both weights reach 1 at the wall, wrists on the face. The sideways
  walk is older and not the brace: pushing W at an angle into a wall, the mover keeps the along-wall part (collide-and-slide)
  while the body faces the stick, so the graph plays a side step (bug logged here; the fix is a mover/facing slice). Run in straight (camera
  square to the wall) to judge the hands. Needs a new exe copy, no bake. Status: awaiting owner.
- Owner 2026-10-30 (dev3 copy, after the gate fix): the brace now shows, but (1) running straight at the wall, the body
  starts moving sideways as soon as it hits: wanted a grazing-angle rule: below a threshold angle to the wall's
  normal the mover stops instead of sliding (mover `sf_controller` `mover.rs`; B2-T noted the same); (2) the side-wall
  trail pose is wrong: people feeling a wall reach the hand ahead of them, palm on the wall, thumb pointing down
  (needs the palm frame from the wall normal, the known rotation gap). Status: partial.
- Owner 2026-10-30: the wall brace needs a big fix; find where the sideways slide comes from first (diagnosis slice),
  then fix the stop and the trail pose.
- Fix 2026-11-05 (brace-stop), point (1) only: collide-and-slide kept the along-wall part of any push and the facing turned
  along it. Now a wall met within 30° of head-on stops the mover (`MoverParams::wall_stop_deg`, Godot's
  `wall_min_slide_angle`); past 30° it still slides. Check: W straight or up to ~25° off at the wall: the body stops and
  stays facing the push, no side step, hands brace; ~45° and shallower: it slides along as before. Trail pose (2) not
  touched yet. Needs a new exe copy, no bake. Status: awaiting owner.
- Fix 2026-11-05 (brace-palm), point (2): the hand IK now takes a palm rotation from the wall normal (rig palm axes from the
  Bip01 fingers). Check: walk W along the wall ~0.5 m off it: the wall-side hand reaches ahead of the shoulder, palm flat on
  the face, fingers forward and a little up, thumb down; run straight in: both palms flat on the face, fingers up, elbows a
  little bent. Look for wrist skin twisting (no forearm twist bones) and fingers through the face. Needs a new exe copy and
  the bake (done 2026-11-05). Status: awaiting owner.
- Owner 2026-11-05: wall stop good, "works and it's satisfying". Nitpick: head-on hands closer together, just in front of the
  bust (not out to the sides), and at chest height (a bit higher than now). Status: confirmed with follow-up (hand spacing/height).
- Fix 2026-11-05 (brace-chest): head-on wrists in front of the chest, 0.55 of the shoulders' spacing apart (Bip01 ~0.17 m;
  were out past the shoulders, 0.5-0.9 m) at 0.92 of shoulder height (~12 cm under it; were 10-30 cm lower, dropping as the
  body got closer), the same at any distance; side-wall trail and wall stop unchanged. Check: `scene level-anim-ledges`, F11,
  run W straight into the grey 3 m wall: hands close together just in front of the chest, palms flat, fingers up, elbows bent
  (more when close), hands not touching each other. Needs a new exe copy, no bake. Status: awaiting owner.

AL-11 | confirmed (2026-11-05: chest brace good, spacing widened a little 0.55 → 0.68) | [anim-lab](../visual-checks/anim-lab.md) | Wall brace, player (palms on the 3 m wall, side-wall trail, off hand only with the sword)

### AL-12: Vault over thin obstacles, player (action.md B2 vault)
- Re-baked 2026-11-05 (slot `traverse.vault` = UAL2 `SafetyVault`). `scene level-anim-ledges`, camera `vaults` (or F11);
  boxes 0.4 m deep east of the ledges: yellow 1.0 m, green 0.85 m, white 0.6 m. Then `cargo dev view "Seyda Neen"`: crates,
  barrels, the dock railing.
- Run (W) + Space ~1 m from the 1.0 / 0.85 m boxes: a one-hand side vault, the right palm on the near edge of the top (not
  floating, not through it), legs clear of the box, lands on the far side and keeps running. The 0.6 m box (under the band,
  0.81-1.26 m) is a plain jump; deep boxes (the ledges) still mantle; a box with a wall right behind mantles instead.
- Watch for: the takeoff foot dipping into the floor and a camera bob on the 0.85 m box (the body is lowered up to ~0.45 m
  to the obstacle), a stretched or floating landing when the run-out is short. Status: awaiting owner.
- 2026-11-05 trigger fix: the probe saw both boxes; the warp refused (`MaxCorrection`) at an angle (hand target sideways of
  the body) and within ~0.3 m of the face. Now the lip is re-probed square-on and a close takeoff brakes into the plant.
  Run at the boxes at an angle and right up to them, then Space: vaults (log `traversal press: ... refused []`); look for
  a brief stall before the hand plants on close presses. Status: awaiting owner.

AL-12 | confirmed (2026-11-05: vault works; a small hiccup before the vault, not smooth or fast yet: tuning) | [anim-lab](../visual-checks/anim-lab.md) | Vault over thin obstacles, player (SafetyVault, one hand on the top; anim-lab vault boxes, Seyda Neen crates and railing)

### AL-9: Gait bands, pelvis and feet (pose audit S3, feed audit F4)
- Humanoid op data 2026-10-26: `StrideWarp` max 1.6 -> 1.2, `SpeedMatch` rate_share 0.3 -> 0.5, foot IK gated on `on_ground`.
- `scene level-anim-loop`: NPCs walk on flat ground. Then `scene level-anim-overview`, F11: player walk, run and sprint.
- Look for: pelvis steady, no bob at step frequency; no deep crouch when running or sprinting (was ~95 mm at 4.5-7 m/s, now
  under 1 mm headless); feet don't slide; feet let go of the ground in a jump. Sprint plays the run cycle at 2.2x
  (4.75 steps/s): if it looks frantic, a sprint cycle is the fix, not a longer stride. Run strafes still skate (clip issue).
  Numbers: `cargo test -p mw_graphs --test it slope_walk::gait_bands -- --ignored --nocapture`. Status: confirmed (owner 2026-10-27: "so much better"; movement feel follow-up under PL-2).

AL-9 | confirmed | [anim-lab](../visual-checks/anim-lab.md) | Gait bands: pelvis steady, no sprint crouch, feet planted

### AN-1: Planted feet stay put on distant walkers (slice R3-1)
- `scene level-anim-overview` (camera `overview`, all walkers at A2 LOD there), then `level-anim-props`/`loop`.
- Look for: on the props walker (beside the orange crates) and the loop walker, the standing foot doesn't drift with
  the body during its stance, far and near. Was: A2 updates got half the body motion. Status: awaiting owner.
- Owner 2026-10-30 (overview): works perfectly. Status: confirmed.

AN-1 | confirmed (2026-10-30) | [anim-lab](../visual-checks/anim-lab.md) | Planted feet stay put on distant walkers

### AN-4 Knockdowns always come back
Scene `level-anim-overview` (or `level-anim-drop`), G on. Throw `V` balls at walkers, the knockdown dummy, the guar
and the mudcrab, hard and soft. Look for: every knocked-down character gets up within ~6 s, then has its capsule
(G) and walks/idles again; a creature without a ragdoll logs "knockdown cancelled" and keeps its capsule. Never a
character frozen lying or standing without a capsule.
- 2026-10-30: retired with PH-2 (delirium replaces knockdowns). Status: retired.

AN-4 | retired (2026-10-30: superseded by delirium) | [anim-lab](../visual-checks/anim-lab.md) | Knockdowns always come back

### AN-5 Get-up starts from the lying pose
Scene `level-anim-overview`, the `@impact` knockdown dummy (idles row, every 12 s). Look for: at the moment the
ragdoll hands back, no snap: the limbs flow from where they lie into the stand-up over ~0.3 s, the pelvis doesn't
jump, and the body doesn't rise or sink to meet the ground.
- 2026-10-30: retired with PH-2 (delirium replaces knockdowns). Status: retired.

AN-5 | retired (2026-10-30: superseded by delirium) | [anim-lab](../visual-checks/anim-lab.md) | Get-up starts from the lying pose

### AN-6 Get-up ends on the capsule
Same scene, G on. Look for: when the get-up starts the capsule appears under the lying pelvis (not ~1 m away), the
character stands up onto it, and nothing slides when the clip ends and idle takes over.
- 2026-10-30: retired with PH-2 (delirium replaces knockdowns). Status: retired.

AN-6 | retired (2026-10-30: superseded by delirium) | [anim-lab](../visual-checks/anim-lab.md) | Get-up ends on the capsule

### AN-21 Slope trunk lean, retuned (reopened 2026-11-05, awaiting owner; first pass archived in visual-checks-closed.md)
Owner 2026-11-05: uphill not enough forward lean, downhill "maybe sliiiightly too much". Retuned (`mw_graphs` build.rs `Lean`):
up 0.4 -> 0.8, down 0.12 -> 0.075, cap 12 -> 26°, spread over Spine1/Spine2/neck (0.45/0.4/0.15; was chest 0.7 + neck 0.3).
Measured walk lean vs flat at 10/20/30°: up +2.8/+5.7/+8.4 -> +5.2/+10.7/+16.5°, down -0.8/-1.7/-2.6 -> -0.5/-1.0/-1.5°
(run about 3/4 of that). Re-baked 2026-11-05. `scene level-anim-stairs`, camera `slopes` (runner on 10°, walkers on 20/30°).
Look for: going up the 20/30° ramps the back clearly bends forward (bent spine, head following, not a craned neck); going
down a barely-there lean back; eases in over ~0.3 s, no jolt; flat walking unchanged. Numbers: `slope_walk slope_lean_table`.

AN-21 | confirmed (2026-11-05: slope lean works) | [anim-lab](../visual-checks/anim-lab.md) | Slope trunk lean

### LOCO-2 Generated locomotion batch 1 (ROADMAP 3.4c B3b, awaiting owner)
- Run (from `bevyrrowind/`): `cargo dev clip-preview --browse "../animation packs/generated/loco1"` (Up/Down = clip group, Left/Right
  = sample; 228 Kimodo clips, 38 groups; `*.free` = same prompt without the speed path). Recipe `tools/motiongen/recipes/loco-batch1.toml`.
- Look for: a natural gait at the right speed (walk 1.6, jog 4.5, sprint 7.0 m/s); feet not skating or hovering (measured: path-forced
  run/sprint cycles skate 36-414 cm/s and lose ground contact for 0.4-2.1 s, walk 9-62; the `.free` takes are slow, 2.6 / 3.1 m/s, but
  plant); starts and stops plant cleanly (no slide into the stand); pivots and turns turn the way named (some pivot/turn samples turn
  right or not at all: mirror or reject); one consistent style across cycles and transitions. Rate in the browser: Y keep, N reject,
  M maybe (each advances), Bksp clear, U next unrated; saved to `loco1/verdicts.toml` (overlay shows group tallies). Numbers: `16-gait-card` "As built (B3b)". Status: awaiting owner.

LOCO-2 | rated 2026-11-12 (229 takes: 38 keep, 61 maybe, 130 reject; verdicts.toml) | [anim-lab](../visual-checks/anim-lab.md) | Generated locomotion batch 1 (Kimodo): gait, speed, skate, clean starts/stops, one style; pick keepers

### LOOP-1 Loop-cut cycles in the clip browser (loco-coord 2026-11-12, awaiting owner)
- Run: `bash bevyrrowind/tools/owner_launch.sh clip-preview --browse "animation packs/generated/loco1"`; Up/Down to a
  `loco.walk.cycle.*` / `loco.run.cycle.*` / `loco.sprint.cycle.*` group. Cycle takes now cook through the loop cutter and loop in
  the slot (no re-blend); the overlay's loop line gives the window, strides and seam errors (pose, foot, speed; hard-cut values in
  brackets). L toggles cut loop vs raw take (A/B). A cut loop walks on: camera and floor follow (the floor steps by whole 2 m checker periods, so it reads as endless).
- Look for: walks loop with no hitch, foot pop or speed surge at the seam (measured residue 0.2-0.6 cm/frame); runs mostly clean
  (0.6-3.2); sprints expected to show the seam (4.9-6.3, poor sources); the slow heading drift per loop is the take's own (±4 deg).
  Say which cut loops are good enough to give slots. Status: confirmed 2026-11-12 (owner: "this is working"; no per-clip pick).

LOOP-1 | confirmed 2026-11-12 (owner: "working"; no per-clip pick, seam numbers decide) | [anim-lab](../visual-checks/anim-lab.md) | Loop-cut cycles in the clip browser: seam hitch/foot pop/speed surge per gait, L = raw take A/B

## Closed 2026-11-15 from visual-checks/audio.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### AU-4: Footsteps by boots (`cargo dev scene door-balmora-guild`)
- Walkers in heavy boots (guards) step heavy, light/medium armour boots light/medium, shoes and bare feet bare.
- Owner 2026-10-30 (`scene door-balmora-guild`): footsteps by boots work. Status: confirmed.

AU-4 | confirmed (2026-10-30) | [audio](../visual-checks/audio.md) | Footsteps by boots

## Closed 2026-11-15 from visual-checks/player-ui.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### PL-1: Third-person walk and the use prompt (`cargo dev view "Seyda Neen"`, 10:00)
- Starts in player mode (the dressed `player` body, PL-2; the capsule is drawn only with characters off). WASD, Shift, Space jump, Tab 1st/3rd, wheel zoom; walk to the Census and
  Excise Office door, then to an NPC (Fargoth by the lighthouse path).
- Look for: smooth walk and camera (no jitter, camera pushes in against walls); under the centre dot the prompt reads
  `[E] Open  <door>` / `[E] Talk  <npc>` / `[E] Take  <item>`, it clears when you look away or step past ~2.7 m;
  E on the door travels, the body arrives at the marker facing out.
- Fixed 2026-10-25 (player3), check: focus is now a physics ray plus actor capsules, so in the Census and Excise Office
  the dot on an NPC's chest reads `[E] Talk  <npc>` (was `Take  Broom`); walls block items behind them.
  `cargo dev view "Seyda Neen, Census and Excise Office"` starts on the floor at the door's arrival marker, facing in
  (was: on the roof, grey void). E does nothing while the journal or dialogue is open.
- Owner 2026-10-30 (release start, New game): walk, camera and use prompt fine. Status: confirmed.

PL-1 | confirmed (2026-10-30) | [player-ui](../visual-checks/player-ui.md) | Third-person walk and the use prompt

### PL-2: Animated player body (action.md A2 slice 2; `cargo dev view "Seyda Neen"`, 10:00)
- Starts in player mode; Tab to third person. Walk (W, stick), run (Shift sprint), Ctrl sneak, Space jump, turn on the spot.
- Look for: the `player` record's character (race, sex, head, hair, clothes) in place of the capsule; walk, run, sneak
  cycles matching the ground speed (no skating), jump/fall clip off the ground and landing back into locomotion, no
  jitter at any frame rate (the body follows the fixed-tick capsule, interpolated, through the correction
  spring). Tab to first person hides the body (no arms yet). Shot: `shots/player-body-3-walk.png` (indoors, 3rd person).
- Status: confirmed with follow-up. Owner 2026-10-27 (after the IK audit fixes): much better, pelvis and jump fixed. Open:
  movement feel has too much inertia ("like driving a car"); never designed on purpose. Follow-up: PL-4.
- Owner 2026-10-27 (arena): switching third -> first person can leave the view separated from the capsule (bug, open);
  first-person arms not built yet (known).
- Fixed 2026-10-27 (afternoon): the split was the body leaving its capsule, not the camera: Delirium click-to-push hit the
  player's own body on a left-click attack (now F11 only, never action characters) plus the UAL2 split-take clip offset
  (cook `start_anchor`). Owner confirmed the view switch live. Status: confirmed (first-person arms still not built).

PL-2 | confirmed (2026-10-27: view/capsule split fixed: self-push + clip offset) | [player-ui](../visual-checks/player-ui.md) | Animated player body (dressed `player` record, humanoid graph)

### PL-3: Jump take-off held in the air, landing on touchdown (`cargo dev scene level-anim-ledges`, F11 player)
- Humanoid graph B2: `jump` plays `Jump@head` and holds it while airborne; touchdown plays `Jump@tail` (`land`), then
  locomotion; running cuts the landing into the run.
- Jump in place, then running. Look for: one tuck at take-off, held in the air, knees absorb on touchdown (land
  sound with it); no second leg lift mid-air.
- B3b feed: walking off a ledge (here or in anim-lab) shows the fall hold (no take-off tuck) after ~0.1 s, then lands;
  running down stairs or over small steps shows no fall pose at all.
- Status: confirmed (owner 2026-10-27: jump feels correct).

PL-3 | confirmed | [player-ui](../visual-checks/player-ui.md) | Jump: take-off held in the air, landing on touchdown (graph B2)

### PL-4: Movement feel: starts, stops, reversals (`cargo dev scene level-anim-overview`, F11 player, Tab 3rd person)
- Player-only motion tuning (Elden-Ring-style): fast speed ramp, faster brake, facing capped at 720/1080/480 deg/s
  (walk/run/sprint) with the motion following the facing, so a reversal turns at speed. NPCs keep their 0.27 s springs.
- Try: run (W), release (stop), W then S (reversal), W then D (90 deg), Shift sprint turns (heavier), tap a direction
  from standing. Look for: responsive, no car-like slide; feet not skating at starts/stops; no pop entering run.
- Numbers (`cargo test -p mw_player_bevy measure_player_feel -- --ignored --nocapture`), before -> after: 90 % speed
  0.77 -> 0.20 s; stop from run 1.12 s / 1.73 m -> 0.22 s / 0.32 m; 180 reversal 1.08 s (speed dips to 0.16 m/s) ->
  0.17 s (keeps 4.5 m/s); 90 deg turn 0.90 -> 0.08 s.
- Tweak: `crates/mw_player_bevy/src/feel.rs` (half-lives, turn rates, feed smoothing); gait blends `mw_graphs` build.rs
  `GAIT_BLEND` 0.15 s / `DIRECTION_SMOOTHING` 0.05 s (shared with NPCs, re-bless). Status: confirmed with follow-up
  (owner 2026-10-27: "much much better, about right"; jump air time is long and floaty). Wanted later: run + Ctrl slide.
  Jump fix 2026-10-27: player gravity 9.81 -> 18 m/s² (`feel.rs` `GRAVITY`; height still 1 m): air 0.90 -> 0.67 s. Confirmed (owner 2026-10-27: "jump feels better").

PL-4 | confirmed (2026-10-27: "about right"; jump fixed with gravity 18) | [player-ui](../visual-checks/player-ui.md) | Movement feel: starts, stops, reversals (player feel tuning)

### HD-1: Minimal HUD: bars, compass, crosshair, use prompt (`cargo dev view "Seyda Neen, Census and Excise Office"`, player mode)
- Bottom left: three thin framed bars, red health, blue magicka, green fatigue (full: the `player` record's 35/50/200),
  no labels, Morrowind-sized. Bottom right: heading (`N 000` ... `NW 315`) follows the mouse look; north matches the map.
- A small dot at the screen centre, in first and third person (Tab): what it covers is what E uses.
- Look at a door or NPC: `E Open <name>` / `E Talk <name>` under the dot; rebind Use (F10) and the glyph follows;
  a creature shows its name only, no key.
- Esc (pause), talk (dialogue), J (journal), inventory, the console: the HUD hides; it returns on close. F11 (free camera) hides it.
- Known: bars are static (no damage or regeneration yet); the compass is a heading label, no local map.
- Owner 2026-10-30: HUD fine. Status: confirmed.

HD-1 | confirmed (2026-10-30) | [player-ui](../visual-checks/player-ui.md) | Minimal HUD: bars, compass, crosshair, use prompt

### HD-2: Bars follow damage and regen (stats S5: the bars are the player's stat sheet)
- Build `cargo build -p bevyrrowind --features dev`; run `powershell -File bevyrrowind/tools/run_dev.ps1 scene level-arena`
  (F11 if the free camera is on). Synthetic: the player 3 m from a training dummy.
- Magicka bar: full at the player's real maximum (it was a static record copy); health and fatigue full.
- Shift tap (roll) or F then LMB (attacks): the green fatigue bar drops at each roll or swing, then refills smoothly a
  moment after; it never drops on its own while standing still, and refills no faster than before S5.
- Hit the dummy: the log reads `training dummy: hit ... health h/100` falling per hit; at 0 it refills to 100.
- Known: nothing hurts the player yet (the health bar's drop needs an attacker: C2 sparring dummy). Owner 2026-11-05: magicka at max, fatigue drops and refills, health bar still (as expected); dummy health "seems to go down", to re-confirm. Status: confirmed except the dummy log line.

HD-2 | confirmed (2026-11-05: bars ok; dummy health log to re-confirm) | [player-ui](../visual-checks/player-ui.md) | Bars follow damage and regen (stats S5: bars from the player's sheet)

### CO-1: Live SSAO and A/B switches from the console
- Kind: game location. Build `cargo build --release -p bevyrrowind --features dev` (no `solari` needed), run
  `.\bevyrrowind view "Seyda Neen, Census and Excise Office"` (also SH-16's Balmora spot). Backquote (key left of 1,
  `²` on AZERTY) opens/closes the console; F1 stays the A/B help.
- Type `off ssao` / `on ssao` (or `toggle render.ssao`, `render.ssao off`): room corners and under-table darkening
  go and come back the same frame, no hitch beyond one pipeline compile. `flip sun_shadows`, `off clouds`,
  `on wireframe`, `flip rings`: same as F2/Shift+C/F7/F8, overlay line updates. `off froxels` says "startup only,
  relaunch with --off froxels". `help on` lists the live names; Tab completes command and variable names. Keys don't reach the game while open.
- Scope: GI dropped (owner 2026-10-25: raster GI not built yet, Phase 4). SSAO off keeps the prepasses (image A/B,
  not cost; cost = a `--off ssao` run). Unblocks SH-16 (AO) re-check.
- Owner 2026-10-25: partial. COC works for interiors only (`coc balmora` fails: exterior lookup, console slice 3);
  can't paste; wants Down to select a suggestion and Enter to accept it. No upscaling/DLSS console switch exists yet.
- 2026-10-26 (console slice 3), re-check: `coc balmora`, `coc "seyda neen"`, `coc "bitter coast region"`, `coc -2 -9`
  fade to the exterior square's centre on the ground (no fall); `coc "seyda neen, census and excise office"` still
  goes inside. Type `re`: Down/Up move the `>` mark, Enter puts the candidate in the line (a second Enter runs it),
  Escape closes the popup first. Ctrl+V and Shift+Insert paste at the cursor (several lines join with `; `).
- Owner 2026-10-30: `off/on ssao` works, every coc (interior and exterior) works, Down/Enter suggestions and paste work.
  Status: confirmed.

CO-1 | confirmed (2026-10-30) | [player-ui](../visual-checks/player-ui.md) | Console: exterior COC, paste, Down/Enter on suggestions (2026-10-26)

### WALK-1: speed gait set, walk / jog / sprint on the flat (3.4c A3, 2026-11-12)
- Kind: synthetic. Run `bevyrrowind bake` first (new gait slots, graph format 11). `cargo dev scene level-flat-slopes`, 10:00,
  camera `overview`; F11 player, Tab third person, orbit to the side. Free movement now plays generated loops (Kimodo
  loco1: walk `walk.cycle.free` 05, jog `run.cycle.3` 04, sprint `sprint.cycle.1` 05, loop-cut at the bake).
- Look for: walk at a light stick push and full (analog: cadence stays natural, steps shorten at a slow push), jog (run),
  sprint: no foot skating on the flat, no pop of the feet when switching walk <-> run <-> sprint (the foot phase now carries
  over), no hitch at the loop seam. Sprint is the "maybe" take (a retimed free sprint is the next A/B). Locked on: backpedal
  and sidesteps still play the Morrowind sets. The WALK-0 `off ...` ladder still works (`off speed_match` = clips at rate 1).
- Status: confirmed 2026-11-13 after the walk swap (owner: "I think it's alright"; the diagonal look came from the centre dot on the off-centre shoulder framing: dot now first person only). Was failed (owner 2026-11-12): "the character walks sideways (not in a straight line, more diagonal)"; sprint "looks okay but the animation itself is a bit goofy (legs wide apart)". Suspect: take heading offset vs travel, hidden in the browser (Drive turns the actor) but not in-game (Match); diagnosis 2026-11-12 (`mw_bake/tests/gait_heading.rs`): no heading offset (walk < 1 deg cooked and in-graph); walk.free.05 is wide (22.8 cm feet, toe-out 10/-12) and Match drops its 8.7 deg pelvis yaw swing. Owner: pick another clip. Re-check with walk.cycle.1.05 (path-guided, 1.62 m/s, pelvis-travel 0.5 deg).

WALK-1 | confirmed 2026-11-13 (walk.cycle.1.05; centre dot now first person only; sprint style still goofy: sprint A/B next) | [player-ui](../visual-checks/player-ui.md) | Speed gait set: generated walk / jog / sprint loops on speed, foot phase kept on gait switches, no skating (3.4c A3)

### PL-9: A planted foot lets go on sharp turns (P-IK5, 2026-11-11)
- Kind: synthetic. `cargo dev scene level-flat-slopes` (cameras `ramps`), F11 player, Tab third person, 12:00. K is the
  A/B (foot placement on/off, walkers and player).
- Before: a foot planted when the stick turned stayed locked in the world while the body spun (720 deg/s walk turn)
  until it drifted 0.3 of the leg's reach: legs spread or crossed, the planted foot up to 34 cm off the clip.
- Do sharp 90° and 180° turns at walk and at run, on the flat and on the 20° and 30° ramps. Look for: the planted foot
  lets go after about 20° of turn and steps with the clip (no spread or crossed legs), with no pop or snap as it lets
  go; straight walking and gentle curves keep their planted feet as before. K off: the plain clip, for comparison.
- Status: confirmed (owner 2026-11-11, level-anim-stairs: "seems ok").

PL-9 | confirmed (owner 2026-11-11: "seems ok") | [player-ui](../visual-checks/player-ui.md) | Sharp 90/180° turns at walk and run on flat and ramps: the planted foot lets go instead of the legs spreading, K A/B (P-IK5, 2026-11-11)

## Closed 2026-11-15 from visual-checks/streaming.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### ST-2: No pop at origin shifts in flight
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground`
  (60 m/s), watch the whole flight.
- Changed: at a floating-origin shift (every 256 m) terrain patches move in place
  instead of respawning, terrain clipmap uploads wait two frames, and object
  streaming (spawn, reveal, despawn) holds for up to 6 frames around the shift.
- Look for: no terrain flicker or one-frame holes when crossing 256 m borders; no
  objects arriving visibly late (pop-in in front of the camera) compared with before.
- A/B: none at runtime (`StreamingConfig::origin_shift_guard_s = 0` in code).
- Owner 2026-11-12 (Bitter Coast flight, also seen before): a little flicker in distant terrain every ~200 m of travel.
  Owner suspects distant shadows mishandling the origin shift (every 256 m); not diagnosed, check later.
  Cause (FLICKER-DIAG + owner bisect 2026-11-12): DLSS/TAA history is reset at every origin shift (`sync_origin`);
  seen on terrain and water, never when still, gone with F4 Off.
- Fix FLICKER-1 (2026-11-12): shifts keep TAA/DLSS history, motion vectors rebased by the origin delta (`OriginRebase`).
  Launch `bash bevyrrowind/tools/owner_launch.sh view "Seyda Neen" --hour 10`, fly west over the Bitter Coast high and
  fast. Look for: no periodic pop on distant terrain, shadows or water. Must not: smears or ghost trails for one frame at
  shifts. A/B `--off shiftrebase` brings the pop back. Log: `origin shift A -> B: rebase ... (history kept)` per shift.
  Water's own motion vectors get the rebase after W-GERSTNER lands (until then water may still pop/smear at shifts).
- Status: confirmed (owner 2026-11-12, Seyda Neen 10:00 flight west: "the flicker is gone"; `--off shiftrebase` A/B not run).
- Source: spikes.

ST-2 | confirmed (2026-11-12: periodic flicker gone, FLICKER-1) | [streaming](../visual-checks/streaming.md) | No pop at origin shifts in flight

## Closed 2026-11-15 from visual-checks/characters.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### CH-5: Busy Balmora street, NPCs deform with shared joint blocks (perf-push P5, 2026-11-12)
- Why: an NPC's meshes (one per material, + face) now share ONE joint block in the skin buffer (vendored `bevy_pbr`
  patch P5); matrices are identical by design, so nothing should change.
- Kind: game location. Run: `scene char-balmora` (walkers OFF in this scene: `--off walkers`, NPCs only idle), DLSS Quality; fly down to a crowded street, then walk
  past NPCs and turn the camera fast.
- Look for: NPCs idle and walk normally; clothes, armour, robes and heads stay on the body (no torn seams, no part
  stuck in a T-pose or at the origin); held weapons/torches follow the hand; no smearing or ghosting on moving
  limbs under DLSS (motion vectors). NPCs that walk into view look right on their first frame.
- Status: **confirmed 2026-11-12** (owner): NPCs deform fine. New, logged: idle NPCs float slightly above the ground (scene checklist says no floating); not yet known whether today's P5/P6 or older.

CH-5 | confirmed 2026-11-12 (new: idle NPCs float slightly above ground, logged) | [characters](../visual-checks/characters.md) | Busy Balmora street: NPCs deform normally with shared skin joint blocks, no DLSS smear (perf-push P5, 2026-11-12)

## Closed 2026-11-15 from visual-checks/player-ui.md (owner-pass-11-15)

### PL-8: Nothing animates in the pause menu (P-IK2, 2026-11-11)
- Kind: synthetic. `cargo dev scene level-anim-stairs` (walkers on the stairs), then `cargo dev scene level-flat-slopes`,
  12:00. F11 player, Tab third person; stand on a tread and next to a wall (hand brace), walk, then Esc (pause menu).
- Before: the animation stage kept running at dt 0 while paused (a walker's foot dropped ~14 cm for a frame on stairs,
  the player's wall brace re-probed every frame).
- Look for: while the menu is open nothing in the world moves (walker feet, hips, hands, the player's hands on the
  wall); close it: walking resumes from the held pose with no pop or snap on the first frame. Status: awaiting owner.
- Owner 2026-11-11 (stairs scene): pause not tested. Other IK still wrong there: a leg still snaps into place, steps too
  wide going up or down stairs, arms still behind the back. Logged, not fixed (see PL-7, locomotion rethink).
- Owner 2026-11-15 (level-anim-stairs): confirmed, nothing moves with the menu open, resumes normally.

PL-8 | awaiting owner (pause not tested 2026-11-11; stairs IK issues logged: leg snap, wide steps on stairs, arms behind back) | [player-ui](../visual-checks/player-ui.md) | Pause menu freezes all animation (walkers on stairs, the player at a wall), no pop on resume (P-IK2, 2026-11-11)

## Closed 2026-11-15 from visual-checks/weather-sky.md (owner-pass-11-15)

### CK-1: One game clock drives sky, weather, lamps and saves (clock C1+C2, ROADMAP 1.17, 2026-11-15)
- Kind: game location. `scene balmora-sunset-shadows` (18:30, opens with the clock paused).
- Do: P (clock runs), then T (time-lapse x600) through dusk to night, T back to x30, P to pause; Esc menu; if the
  pause menu offers Save, save, move the hour with `]`, load.
- Look for: the sun sinks smoothly with no jump when the clock starts or stops; lamps and windows come on at dusk;
  stars come up; P and the Esc menu both stop the sky dead; a load brings back the saved hour and sky, no jump after.
- Status: confirmed 2026-11-15 (owner: "it works"; remarks logged on 1.13b, TX-5, WA-6, NS-3, GF-1 and the weather polish line).

CK-1 | awaiting owner | [weather-sky](../visual-checks/weather-sky.md) | One game clock drives sky, weather, lamps and saves: `scene balmora-sunset-shadows`, P / T / Esc, save and load (clock C2, 2026-11-15)

## Closed 2026-11-15 from visual-checks/rendering.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### TX-1: Normal maps from packs (textures T4, 2026-11-11)
- Kind: game location. Needs a normal-map pack installed as a mod (MVR PBR v2 Beta or Tamrielic PBR West Gash; census in
  `textures-adapter.md` §7) and a fresh `bevyrrowind bake` (line `texture.cook: ... N normal companions`, N > 0).
- Runs: `view "Balmora" --hour 7` and `--hour 18` (low sun raking across house walls and paving), then
  `view "Seyda Neen, Census and Excise Office"` (lamp light on walls). A/B: console `flip normalmaps` (or start with `--off normalmaps`; lab key N).
- Look for: relief that moves with the light on pack-textured walls/floors; bumps not inverted (lit side faces the sun);
  no shimmer or sparkle at distance; no hard seams where a texture is mirrored (symmetric trims, statues).
- Not covered: NPC/creature bodies (assembly carries no tangents yet), terrain (T5/T6).
- Status: awaiting owner. 2026-11-13: "textures seem great", but that run had no pack mounted (`texture.cook: ... 511 cooked`); the packs are now mods `mvr-pbr` and `tamrielic-pbr-west-gash` (`<user data>/mods`, profile `wareya-pbr`): re-run with TX-4. Source: textures T4.
- 2026-11-13 (tx-tangent): cooked tangents had the wrong handedness (green inverted on every cooked static, mirrored
  rigid parts too); fixed and `mesh.cook`/`mesh.rest`/`mesh.rig` keys bumped, so the owner must re-check "bumps not
  inverted" after a fresh `bevyrrowind bake` (every cooked mesh re-cooks once). Earlier TX-1 verdicts predate the fix.
- Owner 2026-11-13 (view Balmora 12:00, full re-bake, packs mounted): normal maps on "obviously better" (confirmed on buildings/paving); landscape ROCK statics look wrong (matches `materials.live: ... 2602 normal map without tangents`; tx-tanfix queued); `flip normalmaps` does not reach terrain (terrain material not in map_ab; queued).
- Owner 2026-11-15 (Balmora 08:00 after tx-tanfix, log `0 normal map without tangents`, was 2602): no shimmer on normal maps; rock look pending.
- Owner 2026-11-15: landscape rocks "look right now". Status: confirmed.

TX-1 | confirmed 2026-11-15 (rocks right after tx-tanfix, no shimmer) | [rendering](../visual-checks/rendering.md) | Normal maps from packs: relief under low sun / lamp light, no inverted bumps, no shimmer, no mirrored-UV seams (pack installed + re-bake; no A/B yet)

## Closed 2026-11-15 from visual-checks/rendering.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### TX-4: PBR `_spec` ORM on statics (textures TS-2, 2026-11-13)
- Kind: game location. Needs a fresh `bevyrrowind bake` with the packs mounted (log `texture profile: mvr-pbr -> wareya-pbr`,
  no `texture profile warning`). Runs: `view "Balmora" --hour 12` (stone, wood, metal trim), then an armourer's interior
  (e.g. `view "Balmora, Meldor: Armorer"`) for displayed armour and weapons. A/B: console `flip orm` (or `--off orm`; lab key M).
- Look for: metal reads as metal (dark diffuse, coloured reflections); wood and stone stay matte, not plastic; crevices
  darker under ambient (AO); no mirror-gloss everywhere (= a pack on the wrong profile); displayed armour the same up close
  and a few metres away (matte close, metallic far = MVR metal-mip defect, repair quirk still off).
- Not covered: worn armour/bodies on NPCs (actors bind no ORM yet), terrain.
- Status: awaiting owner. Source: TS-2.
- Owner 2026-11-15 (Balmora 12:00, `flip orm`): ORM looks fine outside. The run then crashed walking near a house (render-layer exhaustion, ROADMAP 1.1, not texture-related); armorer half re-launched as `view "Balmora, Meldor: Armorer"`.
- Owner 2026-11-15 (Meldor: Armorer, `flip orm`): "something happens", but expected-metal items (iron helmet) barely change. Diagnosis (orchestrator, top-mip decode of the pack files): our path is right (factors 1.0, R metal -> B), but MVR PBR v2 Beta has metalness 0 on every pixel in 3384 of its 3390 `_spec` files (iron helm00_00/01/03, steel battleaxe and greaves, `tx_dwrv_metal00` all R = 0; only 6 sword grip/quillon/club textures carry metal). So ORM only changes roughness + AO; pack metal is absent. Comparison: `TX_W_Broadsword_grip`, `tx_w_claymore_grip`, `TX_W_club_daedric_00`. Status: pipeline confirmed; metal look waits on owner (another pack, or a bake-time metal profile).
- Owner 2026-11-15: "fine, move on": a pack without metal data is the pack's problem; what matters is that the pipeline offers it. Status: confirmed.

TX-4 | confirmed 2026-11-15 (pipeline OK; MVR pack carries metal in 6 of 3390 `_spec` files, accepted) | [rendering](../visual-checks/rendering.md) | PBR `_spec` ORM on statics: metal reads as metal, wood/stone matte, AO in crevices, no mirror-gloss everywhere (TS-2; packs on `wareya-pbr`)

## Closed 2026-11-15 from visual-checks/rendering.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### TX-3a: Load-time texture size cap (textures S1, 2026-11-13)
- Kind: game location, A/B by restart. Needs the 2K packs mounted (`mvr-pbr`, `tamrielic-pbr-west-gash`) and a current bake.
  Runs: `view "Balmora" --hour 12` (High: cap 2048 = packs at full res), then the same with `--texture-cap 512` (Low's cap,
  nothing else changed), optionally `--quality low`. Stand close (1-2 m) to a pack-textured wall and the paving.
- Look for: High looks as before (full res); 512 visibly softer up close, same at distance; no texture missing, black, pink
  or blocky-garbled, normal maps still bumping the right way, no sampler/tiling change.
- Log: `texture.cap: 2048 px, N ktx2 loaded, M capped, ...` (High: M ≈ only >2K files; 512: M large, MiB skipped > 0); one
  expected startup `warn!` `Duplicate AssetLoader preregistered ... ["ktx2"]`, no `ktx2 cap:` load errors.
- Status: awaiting owner. Source: textures S1.
- Owner 2026-11-15 (Balmora 12:00, High then `--texture-cap 512`): "that's good". Log High: `texture.cap: 2048 px, 1114 ktx2 loaded, 8 capped, 692.2 MiB loaded, 18.0 MiB skipped`; 512: `texture.cap: 512 px, 1803 ktx2 loaded, 1067 capped, 0 uncappable, 198.6 MiB loaded, 762.4 MiB skipped`. Status: confirmed.

TX-3a | confirmed 2026-11-15 | [rendering](../visual-checks/rendering.md) | Load-time texture size cap: High = full res, `--texture-cap 512` visibly softer up close, nothing missing/black/pink (S1; `texture.cap:` line)

## Closed 2026-11-15 from visual-checks/weather-sky.md (owner verdicts up to 2026-11-13; index line kept under each entry)

### FG-1: Volumetric fog black bands on fast turns (owner report)
- Kind: game location: fog lit by lamps at night (owner shot `Desktop/shadow fog.jpg`: giant-mushroom/cave area,
  21:15, fog). Turn the camera fast left or right.
- Seen by owner 2026-10-20: dark vertical bands in the lit fog during fast yaw. Repro history (renderfix1, froxel
  history, lamp-range edge, sun cascades) is in `archive/visual-checks-closed.md`, "FG-1 history".
- Status: not reproduced 2026-10-21 (owner, Sadrith Mora 21:15 foggy): fog code unchanged since the report; the sun
  cascades changed in shadow reset A2/B3, likely the fix. Reopen if seen.
- Owner 2026-11-15 (`view "Sadrith Mora" --hour 21.25 --weather foggy`, fast turns in lamp-lit fog): no black bands. Status: confirmed fixed (closed).

FG-1 | confirmed 2026-11-15 (no bands in Sadrith Mora fog at night) | [weather-sky](../visual-checks/weather-sky.md) | Volumetric fog black bands on fast turns

## Closed 2026-11-17 from visual-checks/rendering.md

### RO-12: Bright outline around characters (owner report)
- Kind: game location. Seen 2026-10-30 from the release start (Seyda Neen), on NPCs and the player.
- Look for: a thin halo, brighter than the surroundings, along character silhouettes.
- Suspects (not tested): TAA/DLSS disocclusion on skinned meshes (motion vectors), GTAO or contact-shadow edge, fog
  froxel edge. First step: F4 (temporal mode) and `--off` A/Bs at the same spot.
- Status: known bug: cause not diagnosed (owner pass 2026-10-30).
- Owner 2026-10-30, `scene wk-balmora`: no outline around characters there; seen only from the release start (no
  subcommand). Clue: compare its settings (DLSS mode, preset, resolution) with `scene` runs.
- Audit 2026-11-01: no settings diff (content: Seyda Neen haze). Lead: half-res cloud/bank composite gives thin foreground
  objects the background's bank fog (`cloud_composite.wesl:44-87`, `d.y` not depth-clipped); secondary GTAO edge under DLSS.
- Owner 2026-11-17: fixed for a while (not seen since well before 2026-11-15). Status: confirmed (closed).

RO-12 | confirmed 2026-11-17 (owner: fixed for a while) | [rendering](../visual-checks/rendering.md) | Bright outline around characters (2026-10-30)

### SK-HB: Coloured bands at the horizon (atmosphere transmittance, 2026-11-19)
- Game location: `view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10"` (owner shot 2026-11-19 11:50, Cloudy, toward
  the summit), then any Morrowind exterior with a far view (e.g. `view "Seyda Neen"` looking out to sea and the hills).
- Look for: no thin purple, cyan or yellow lines across far mountains at eye level, in Cloudy, Clear, Overcast, Rain
  and Blizzard (weather key); far terrain still fades into the haze as before (the fix changes no look on purpose,
  only removes the broken band). Status: confirmed by the owner 2026-11-19 ("fixed").

### WE-16: Windy, a dry gale on grass (weather-adapter.md "Extra presets")
- Game location: `scene we-windy` (Grazelands 11,8 at 14:00, blades on, `--weather windy`): Cloudy's sky and light,
  16-20 m/s wind at 10 m, gusts to 1.4x in ~120 m cells, clouds at 60 m/s, no rain. A/B on the same sky: BRP `set
  {"weather": "cloudy"}` (~4 m/s) and back `{"weather": "windy"}`.
- Look for: blades and clumps pressed hard downwind and streaming; gust fronts sweeping the plain as waves, lulls where
  they half stand up; trees thrashing; clouds racing; nothing leaning upwind, no popping, no blades through the ground.
  Values live in `mw_mods/data/weather/extra-presets.ron` (say what to change: speed, gustiness, gust size).
  Owner 2026-11-24: "it's very good". Status: confirmed (closed).

WE-16 | confirmed 2026-11-24 (owner: "very good") | [weather-sky](../visual-checks/weather-sky.md) | Windy preset, a dry gale on grass: `scene we-windy` (Grazelands blades, 14:00); blades pressed downwind, gust waves across the plain, trees thrashing, fast clouds; A/B BRP set weather cloudy/windy (2026-11-24)
