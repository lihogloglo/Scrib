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

## Lighting: night exposure floor, point-light shadows (superfargoth `sky-lighting.md` "Exposure range", "Local-light shadows"; `.sessions/lighting.md`)

Shots in `shots/lighting/` (`before_night.png` / `night_old.png`: the old night; `night2.png`: floor 3.5 + night ambient 1;
`cave_in1*.png`; `torch_stage.png`).

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
