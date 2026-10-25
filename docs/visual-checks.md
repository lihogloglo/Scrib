# Visual checks

The owner's eyes judge rendering work (CLAUDE.md rule 5). Agents take a few
sanity screenshots, then add an entry here with status **awaiting owner**
and move on. The owner confirms in batched passes.

This file holds **open** checks only (awaiting, partial, rejected, known bug, deferred). Closed ones (confirmed,
retired, decided) move to [`archive/visual-checks-closed.md`](archive/visual-checks-closed.md). Keep entries short:
run line, what to look for, status. History goes in the spec, not here.

## How to do a visual pass

Since 2026-10-19 the session runs the pass: it copies a current release exe (`target/release`, built with
`--features solari,dlss`; check the build's features first) plus the DLSS DLLs and `scenes.ron` into
`%TEMP%\bw-owner<N>`, so repo builds never block it, then launches **one window per check** for the owner and
says what to expect (memory: owner-visual-confirmation). The session launches every window and every A/B
variant (`--off ...`, `--dlss ...`) itself; the owner never types commands or relaunches with flags (owner
2026-10-21). The old `play\` folder is gone. Running it yourself:

1. **Open a terminal in the newest `%TEMP%\bw-owner<N>` folder.**
2. **List the scenes:** `.\bevyrrowind scene --list`.
3. **Open one:** `.\bevyrrowind scene <name>` (a unique prefix works:
   `.\bevyrrowind scene ghostfence-f`). The console prints the equivalent
   `view` command line and what to look for; the window shows the same in
   its top-left overlay. Add any `view` flag after the name for an A/B run,
   e.g. `.\bevyrrowind scene balmora --off farfield` or
   `.\bevyrrowind scene dlss-run --dlss performance`.
4. **Look, fly, flip.** A scene opens at its camera and hour with the clock
   paused. Keys:

   | key | does |
   |---|---|
   | WASD, Q/E, mouse | fly (free camera; Shift runs at 80 m/s) |
   | F1 | show/hide the help overlay |
   | F2 | sun shadows on/off |
   | F3 | terrain far shadows on/off (shadows past the cascades) |
   | F4 | DLSS: off, quality, balanced, performance, DLAA (exe built with `dlss`; on by default as `auto` since 2026-10-12, start with `--dlss off` for native) |
   | F5 | far-field level tint (L1 green, L2 blue, L3 purple) |
   | F6 / F7 | terrain LOD colors / terrain wireframe |
   | F8 | streaming rings overlay |
   | `[` / `]` | hour -/+ 0.5 (Shift: 0.1) |
   | P | pause/resume the clock |
   | T | time-lapse (x600; again for normal x30) |
   | F9 | print the camera as `--camera "..." --hour h` and copy it to the clipboard |
   | F12 | screenshot to `play\shots\<scene>-<time>.png` |

   Not switchable live (decided at startup, compare two runs): `--off
   farfield` (alias `lod`), `--off terrain`, `--off sky`, `--off envmap`,
   `--off near`, `--off far`, `--off nif`, `--off lights`, `--off
   particles`, `--off farshadows` (terrain far shadow), `--off denoiser`,
   `--lod-tint`, `--far-tint` (initial state of F5), `--terrain-lod`,
   `--wire`, `--rt`, `--prepass`, `--resolution WxH`. Several `--off` parts go comma-separated: `--off farshadows,lights`.
5. **Mark the result** in this file: change the entry's `Status:` line to
   `confirmed` or `rejected: <what you saw>`, optionally with a camera from
   F9 or a screenshot from F12. Agents pick up rejected entries.

Scenes live in `bevyrrowind/scenes.ron` (name, cell, camera, hour, flags,
what to look for). The exe reads `scenes.ron` from the current folder first,
then next to the exe, then its built-in copy, so a scene can be tweaked in
`play\scenes.ron` without a rebuild. From the repo: `cargo dev scene <name>`
(Bevy dynamically linked), and `cargo run -p bevyrrowind --features
dev,solari,dlss -- scene <name>` for the RT/DLSS scenes.

Synthetic scenes (framework examples) isolate one system on generated data;
they run from `D:\Gamedev\Bevyrrowind\superfargoth` with cargo (no exe in
`play`). See "Synthetic scenes" at the end.

### Entry format

`ID: title` — kind (game location / synthetic), launch command, what to look
for, A/B, status, source session. Status is one of `awaiting owner`,
`confirmed`, `rejected: <note>`, or `known bug: <fix queued>` (the check
shows a bug that's already understood; confirm the bug or skip).

---

## Far field (superfargoth `far-field.md`, "Measured on Morrowind")

Screenshots: `D:\Gamedev\Bevyrrowind\shots\farfield\` (`balmora_a*`,
`redmtn_view*`, `summit_vivec*`, `ebonheart_night*`, `handoff_forest*`).

### FF-2: Far-field levels
- Kind: game location. Run: `.\bevyrrowind scene balmora`, press F5.
- Look for: L1 green from ~400 m, L2 blue from ~2 km, L3 purple beyond; no
  untinted gap and no patch that flickers between levels while flying.
- A/B: F5 on/off.
- Status: partial, owner 2026-10-19 (rounds 1 and 3, Balmora and the summit): F5 shows only L1 green past ~400 m, never L2 blue or L3 purple. The run log says L2/L3 exist and are resident (1623 L1, 25 L2, 4 L3 nodes, 79.7 MiB), so either the tint misses L2/L3 materials or L2/L3 are never selected inside the view distance. Phase 1.
- Source: farfield-lod.

## Distant shadows (superfargoth `weather.md`, "Near-term visual fixes", bug A)

The shadows session (2026-10-12) moved the sun's cascades to 504 m (2048² after shadows2; 4096² cost +1.4 ms)
and added a terrain far shadow past them. Screenshots: `shots\shadows\`
(`before_*`, `after_*`, `north_1830_on/off.png`). Cost measured +2.5 ms at
1440p (over budget; being cut).

### SH-9: Many shadowed lights indoors (shadows F6)
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen, Census and Excise Office" --camera "1.0,4.2,5.5,2.7,3.0,-4.0" --point-shadows 8`
  (desk, chair, chandeliers, two NPCs at 6-10 m); compare `--point-shadows 1` (High today) and `--point-shadows 4` (Ultra).
  Also `view "Balmora, Council Club"` walking past the braziers.
- Look for: with 8, the desk, chair legs and NPCs cast shadows from several lamps at once; chandeliers throw patterns on
  the ceiling and floor; no light leaks through walls into the next room. Walking or turning the camera: no shadow pops
  (the shadowed set is picked by importance incl. lights behind you, with hysteresis and a 0.3 s crossfade);
  candle flicker doesn't make shadows jump between lights.
- Cost: uncached Bevy maps, ~1.9 ms per light (census bench: 4.8 / 6.6 / 12.2 / 21.7 ms for 0 / 1 / 4 / 8), so the
  presets are unchanged until cached maps (superfargoth `docs/specs/shadows.md` slice 2). Owner: is 8 the look you want?
- Status: **rejected 2026-10-19 (owner, 8 shadows):** some shadows flicker, e.g. the thin cable holding a chandelier (thin casters in the cube maps; check cache recapture + candle flicker). Was: (shf6 saw 1 vs 8 at the camera above: 8 adds table/chair/chandelier shadows, clearly moodier).
  Since shf6b (slice 2) maps come from a cache (`--off shadowcache` for Bevy's own): also look for NPC shadows
  following them, no shadow left behind when a door opens or an item is moved, no 1-3 frame flicker of actor
  shadows when a light recaptures. Not yet seen by an agent (no screenshots).

- Source: shf6 (shadows F6 slice 1).

### SH-11: Distant flashes while moving (owner report)
- Kind: game location. Run: `.\bevyrrowind scene balmora`, fly around.
- Seen by owner 2026-10-19 (round 1): brief flashes in the distance whenever the camera moves; they look like shadows.
  Suspects (unverified): cascade refit / texel snap at the 250 m end, the terrain far shadow map swapping on its
  async refresh, the dithered cascade end.
- Status: known bug: Phase 1 shadow pass.
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
- Fixed (superfargoth c31e397): trace marches to the furthest depth of a 6x6 window and stores the cut depth;
  resolve and composite match taps by population (open sky vs cut at the same depth); jittered history matrix.
- Status: **confirmed by owner 2026-10-19** (round 15: fringe, halo and sky wobble gone).

### SH-13: Point-light (lamp) shadows (owner report)
- Kind: game location. Run: `.\bevyrrowind scene lamps-handoff` (Balmora, 22:00).
- Seen by owner 2026-10-19 (round 5): lamp shadows only visible a few metres from the lamp; some have a grainy,
  dithered pattern; walking guards cast a shadow that isn't their outline; some shadows flicker.
  Suspects (unverified): budget of 1 shadowed light at High with a 15-25 m fade (dithered fade = the grain),
  `shadow_importance` switching which light casts (flicker), skinned casters or 1024² cube faces + bias (guards).
- Owner 2026-10-19 (round 9, lt-cave): grain varies, some lamp shadows are clean and others very grainy (per-light: budget fade dither vs. the fully shadowed light?).
- Owner 2026-10-20: shadows overall "only showing when super close from the object". Note: since 2026-10-19 the
  app no longer adds the parked cube-cache plugin, so the one shadowed lamp redraws its 6 faces every frame.
- Status: known bug: Phase 1 shadow pass (1.4).

### SH-14: Contact-shadow bands at object edges (owner report)
- Kind: game location. Run: `.\bevyrrowind view Balmora --camera "-280,+12,200,-295,+1,180" --hour 15 --weather clear`,
  turn the camera; A/B `--off contactshadows`.
- Seen by owner 2026-10-20: thin shadow bands around object edges that move with the camera angle; gone with
  `--off contactshadows` (confirmed cause). Ours march 0.5 m with 0.1 m thickness (Bevy default 0.3 m), Medium 8 /
  High 16 steps (`exterior.rs` `Quality::contact_shadows`). Fix direction: shorter rays, or contact shadows for
  close contact only, as engines use them; then recheck SH-7's grounding.
- Status: known bug: Phase 1 shadow pass (1.4).

### SH-15: 8 cached lamp shadows (shadows v2 V1, shadows2)
- Kind: game location. Run (release): `.\bevyrrowind view "Seyda Neen, Census and Excise Office"`, walk up to the
  desk and the NPCs; then `.\bevyrrowind scene lamps-handoff` (Balmora 22:00), walk the street past torch-bearers.
  A/B: `--point-shadows 1` (old High), `--off shadowcache` (Bevy redraws every face).
- Look for: lamp shadows from several lamps at once, out to ~30-40 m (was a few metres); no flicker when walking
  close to a lamp or when NPCs walk by; NPC shadows follow them without 1-frame gaps; no shadow left behind
  where an object streamed out (≤ 2 s is by design); no shadow "swimming" with a carried torch.
- Owner decides: High's lamp count (8 now; walk ~+2 ms vs 0 lamps, perf.md "Shadows v2").
- Status: partial (owner 2026-10-21, census, 8 lamps): flicker at first, stable once the maps are captured; shadows
  pop in a few metres ahead (end of the hall gets its shadows only when walking through the door): shadowed-set
  changes are visible. Goes to slice A/B (budget + light lab).

### SH-16: Shadow reset A1 (lamp contact off, sun out of interiors, GTAO)
- Kind: game location. Run (release): `.\bevyrrowind view "Seyda Neen, Census and Excise Office"`; `.\bevyrrowind
  scene lamps-handoff` (Balmora 22:00, walk the street); `.\bevyrrowind view Balmora --camera "-280,+12,200,-295,+1,180"
  --hour 15 --weather clear` (SH-14's spot), turn the camera. A/B `--off ssao`, env `BEVYRROWIND_LAMP_CONTACT=1`.
- Look for: no thin view-dependent bands at object edges near lamps (SH-14), the sun's own contact shadow short
  (0.3 m); soft darkening in room corners, under tables/crates and where walls meet the floor, without dark halos
  around NPCs or a grainy crawl under TAA/DLSS; indoors nothing lit or shadowed from the sun's direction; at night
  torch-bearers' own lights cast moving shadows (within High's 8); outdoors sun shadows, cloud shadows and rain
  occlusion under roofs as before.
- Status: inconclusive 2026-10-21 (bw-owner10): owner can't judge AO / contact shadows without live on/off toggles
  (and is unsure contact shadows work at all); needs the dev console (ROADMAP 1.13, queued).

### SH-19: Shadow reset B2 (lamp shadows on 2x2 hardware PCF)
- Kind: game location. Run (release): `.\bevyrrowind view "Seyda Neen, Census and Excise Office" --camera
  "1.0,4.2,5.0,1.0,3.6,-6.0"`, then walk to the desk and the NPCs; also `view Balmora --hour 22` by lamps.
  A/B: `--off lampfilter` (lamps back on the preset's Temporal filter, the old look).
- Look for: lamp shadow edges (table legs, chandeliers, NPCs on the floor) as soft as before under TAA, no stair-step
  aliasing or crawling/shimmer on the edges while walking; the sun's shadows outdoors unchanged. Agent screenshot
  pair at that camera looked identical at full-screen scale; motion is the open question.
- Owner decides: keep 2x2 for lamps (-0.1 ms GPU per shadowed lamp at 1440p) or `--off lampfilter` as default.
- Status: inconclusive 2026-10-21 (bw-owner10): owner can't judge the lamp filter without live on/off toggles; needs
  the dev console (ROADMAP 1.13, queued).

### SH-20: Character shadows update very infrequently (owner report)
- Kind: game location. Run: `.\bevyrrowind scene lamps-handoff` (Balmora 22:00) and a daytime street with walkers;
  watch the shadow of a walking NPC.
- Seen by owner 2026-10-21 (bw-owner10, SH-17/18 pass): NPC shadows move in coarse steps, so the shadow animation
  looks choppy while the NPC itself is smooth.
- Cause (charshadows 2026-10-21, confirmed in code, not yet at runtime): lamp shadows only. Shadow reset B3's skip
  rule in vendored `bevy_pbr` light.rs hashes a skinned caster by its root on a ≥ 20 cm grid (joints ignored) and
  refreshes ≤ 2 faces/frame, so a walker's lamp shadow redraws ~5-8 Hz and an idler's never animates. Not the cube
  cache (`POINT_SHADOW_CACHE` off since B3, so `--off shadowcache` is a no-op) nor the anim budget.
- Fix parked, unbuilt: `superfargoth/vendor/parked/SH-20-*.diff` (redraw a face when its casters move/deform,
  outside the budget; `--off movershadows`). Needs build, A/B, perf (census, balmora_walk_night ≤ 11 ms).
- Status: known bug, on the rendering-bugs list (ROADMAP 1.12).

### FG-1: Volumetric fog black bands on fast turns (owner report)
- Kind: game location: fog lit by lamps at night (owner shot `Desktop/shadow fog.jpg`: giant-mushroom/cave area,
  21:15, fog). Turn the camera fast left or right.
- Seen by owner 2026-10-20: dark vertical bands in the lit fog during fast yaw. Repro history (renderfix1, froxel
  history, lamp-range edge, sun cascades) is in `archive/visual-checks-closed.md`, "FG-1 history".
- Status: not reproduced 2026-10-21 (owner, Sadrith Mora 21:15 foggy): fog code unchanged since the report; the sun
  cascades changed in shadow reset A2/B3, likely the fix. Reopen if seen.


## Ghostfence (superfargoth `weather.md`, bug B)

### GF-1: Ghostfence appears complete, never late
- Kind: game location. Run: `.\bevyrrowind scene ghostfence-far` and fly
  toward Ghostgate (the camera starts ~1.9 km south-west, 120 m up, facing it).
- Look for: the fence segments appear at their ring (~520-650 m) already
  loaded: no frames where the pylons stand without the field after a cell
  group reveals (B3 preload). Not fixed yet: the field is still invisible
  past the ring and appears there without a fade. Its segments are
  activators made only of blended parts, so they have no far copy (weather.md
  B5: blended far proxies + scriptless activators).
- A/B: F5 (pylons tint green at L1; fence segments stay untinted). Compare
  `play\` before this session's exe if kept: there the field showed up a few
  frames after its cell.
- Also (owner 2026-10-19, round 6, from the summit at night): round light spots sit inside the fence "wall": they are the far-distance light sprites of the fence's blue lights, drawn while the field itself has no far copy. Not vanilla-looking.
- Status: known bug, seen by owner 2026-10-19 (round 2): the fence field disappears entirely at a distance and pops back in close. Cause known: blended-only activators have no far copy (weather.md B5); Phase 1 or later.
- Source: weather-spec / farfield / nifpop.

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

## DLSS (superfargoth `sky-lighting.md`; `.sessions/dlss.md`)

Screenshots: `D:\Gamedev\Bevyrrowind\shots\dlss\`.

### DL-2: Super Resolution at 1440p
- Kind: game location. Run: `.\bevyrrowind scene dlss-balmora-1440`
- Look for: as sharp as native (F4 to off) when still and when flying;
  frame time drops ~12 to ~9 ms.
- A/B: F4.
- Status: deferred 2026-10-19 (phase0): recheck with a solari,dlss build when the RT role is decided (Phase 1).
- Source: dlss.

### DL-3: Ray-traced Census Office with Ray Reconstruction
- Kind: game location. Run: `.\bevyrrowind scene census-rt` (raster
  baseline: `.\bevyrrowind scene census`).
- Look for: clean, stable bounce lighting without crawling noise; brightness
  close to raster.
- A/B: `.\bevyrrowind scene census-rt --off denoiser` (noisy); F4 changes the
  RR quality.
- Status: confirmed by owner 2026-10-19 (round 14, solari,dlss build): the Census Office looks nice. Feels very smooth (owner). Gaps: no characters (`--rt` still opens the single-cell viewer, which spawns no actors, and Solari 0.20 traces static BLASes only: skinned meshes need a compute skin cache). Phase 1.10.
- Source: dlss.

### DL-4: Rug-edge black bands under RT
- Kind: game location. Run: `.\bevyrrowind scene census-rt`, look at the floor where rugs end.
- Look for: black bands along rug edges, with and without the denoiser.
  Believed to be content (rug geometry), not DLSS.
- A/B: `--off denoiser`.
- Status: known bug, owner 2026-10-19 (round 14): black bands at the edges of rugs, and also a table, a bench and a chair. Cause not investigated. Suspects: objects touching or interpenetrating the floor (contact gaps fully occluded, ray-origin offset too small), double-sided or flipped faces, then bad normals. First test: same camera in raster; if the bands are RT-only, it's the RT scene, not the meshes. Phase 1.10.
- Source: dlss.

### DL-5: Interior `--rt` without Ray Reconstruction quits (wgpu validation)
- Kind: game location. Run: `view "Seyda Neen, Census and Excise Office" --rt --off denoiser` (or `--rt` with no
  DLSS plugin package, which falls back to TAA).
- Look for: the app quits at the first frame: `Device::create_bind_group 'sky_motion'`: the view depth texture has
  `COPY_SRC | RENDER_ATTACHMENT`, no `TEXTURE_BINDING`. With RR on it runs (the RR path makes depth bindable).
- Status: known bug, found by dlss-plugin-2 2026-10-25; not caused by the DLSS plugin split (same with the plugin
  absent). Suspect: superfargoth `sf_render_order_bevy/src/sky_motion.rs` `prepare` binds depth on any single-sampled
  view with motion vectors; interiors have no atmosphere/water camera to add `TEXTURE_BINDING`. Fix candidates: skip
  views whose depth texture lacks the usage, or add the usage like `sf_water_bevy::depth_usages`. Not fixed (owner:
  log bugs, don't chase); blocks the DL-3 noisy A/B. Also: the RR Census shot needs `--settle 6` (2 s = black frame,
  RT warm-up).
- Source: dlss-plugin-2.

## Sky (superfargoth `sky-lighting.md`)

### SK-2: Night and moon
- Kind: game location. Run: `.\bevyrrowind scene sky-night`
- Look for: moonlit, readable night, not grey-washed; town lights carry it.
  Known: no stars (M6), the moon disk has no phase shape.
- Status: deferred 2026-10-19 (phase0): recheck after the Phase 1 night brightness fix (LT-1 rejected).
- Source: sky-lighting.

## Clouds (superfargoth `weather.md`, "Clouds"; `.sessions/clouds.md`, `.sessions/clouds2.md`)

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

## Streaming spikes (superfargoth `streaming.md`, "Spikes"; `.sessions/spikes.md`)

### ST-2: No pop at origin shifts in flight
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground`
  (60 m/s), watch the whole flight.
- Changed: at a floating-origin shift (every 256 m) terrain patches move in place
  instead of respawning, terrain clipmap uploads wait two frames, and object
  streaming (spawn, reveal, despawn) holds for up to 6 frames around the shift.
- Look for: no terrain flicker or one-frame holes when crossing 256 m borders; no
  objects arriving visibly late (pop-in in front of the camera) compared with before.
- A/B: none at runtime (`StreamingConfig::origin_shift_guard_s = 0` in code).
- Status: deferred 2026-10-19 (phase0): recheck in the Phase 1 perf pass.
- Source: spikes.

### ST-3: Paced model loads arrive in time
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground`
  (60 m/s), then the 7 m/s run (`--bench benchmarks/exterior_run.json`).
- Changed: models load only while the meshes of the models already loading stay under 64
  (`StreamingConfig::max_loading_meshes`, estimated from a running mean of meshes per model; never
  capped in burst mode); the rest queue, so a cell's objects can be revealed a few frames later than before.
- Look for: no objects popping in near the camera (inside ~100 m) that didn't before;
  far objects fading in at their usual distance.
- A/B: `$env:BEVYRROWIND_LOADING_MESHES=0` before the run restores unpaced loads.
- Status: deferred 2026-10-19 (phase0): recheck in the Phase 1 perf pass.
- Source: spikes2. Renamed by spikes4 (the budget counts meshes, not models).

## Size-based draw distance (superfargoth `industry-audit.md` J1; `.sessions/sizecull.md`)

### SC-2: The cutoff fades, never pops (Low preset)
- Kind: game location. Run: `.\bevyrrowind scene sizecull-balmora-low`, then
  back away from a market stall or a pile of crates (S, or Shift+S).
- Look for: small props dither out over the last 10 % of their draw distance
  (a 6 cm cup ~23 m, a 45 cm barrel ~170 m, at Low); nothing pops; buildings,
  doors and NPCs never go.
- A/B: add `--off sizecull` (everything to its ring or the 300 m handoff).
- Status: deferred 2026-10-19 (phase0): recheck in the Phase 1 perf pass.
- Source: sizecull.

## Door links (superfargoth `streaming.md`, "Worldspaces at runtime"; `.sessions/interiors.md`)

### DR-1: Through a door into an interior
- Kind: game location. Run: `.\bevyrrowind scene door-balmora-guild`, press
  Space (Morrowind's activate key) facing the door, within 4 m.
- Look for: a short fade (0.3 s out, 0.3 s in; longer only if the interior
  wasn't prefetched); you stand at the arrival marker inside the guild hall,
  level, facing into the room (the marker's heading); the cell's ambient light
  and dark clear color, no sky, sun, clouds, terrain or sea showing through;
  the window title names the interior. `door-balmora-guild-auto` does the
  same by itself.
- Mouse after arrival (fixed 2026-10-12, doorfix): move the mouse right after
  the fade lifts, and also keep moving it during the fade. The view must turn
  smoothly from where you arrived facing, never snap back to the heading you
  had before the door. Before the fix, Bevy's `FreeCamera` kept its own
  yaw/pitch, so the first mouse motion jumped to the pre-door direction (the
  "sudden turn", and the "wrong facing" when the mouse moved during the fade).
- Height: the camera arrives 0.5 m above the marker (scaffold until the
  character controller grounds the player). Most markers float ~1.2 m above
  the sill, so that's eye height; a marker authored low can still leave the
  camera near the floor (the Census Office's other door, DR-3).
- Status: partial, owner 2026-10-19 (round 8, several round trips): travel works, but see DR-4 (proxy shapes first, fade not black, arriving while the interior still loads, crash).
- Source: interiors, doorfix.

### DR-2: Back out
- Kind: game location. Run: as DR-1, then turn to the door you came in by and
  press Space. Hands-off: `.\bevyrrowind view Balmora --camera "-316.6,10.4,200.5,-312.6,10.0,200.35" --hour 10 --use-door 2`.
- Look for: you arrive in front of the guild door outside, facing away from
  it; sky, clouds, terrain and the town are back at once; lighting and
  exposure match the start; mouse look as in DR-1.
- What changed (doorfix): the exterior you left stays resident (hidden) while
  you stand within 32 m of a door back to it, and for 5 s after leaving it
  (`SchedulerConfig::linger_secs`), so the way out no longer reloads the near
  ring under the fade. Measured: out 0.64 s (was 1.02 s), in 0.63 s. Also try
  staying inside a minute near the door, then leaving: still fast. Walk more
  than 32 m from the door in a large interior and wait 5 s: the exterior
  streams out, and leaving takes about 1 s again.
- Status: partial, owner 2026-10-19 (round 8): works, see DR-4.
- Source: interiors, doorfix.

## Skinned models (superfargoth `bake.md`, "Bevy mapping"; `.sessions/skinloader.md`)

`Cooked::Skinned` bake products rendered by Bevy's own skinning and morphs
(`sf_model_bevy::spawn_skinned`), bound by joint name to a skeleton. Bind
pose only: no animation runtime yet.

### SN-2: Guar from its cooked skinned product
- Kind: game model. Run: `.\bevyrrowind scene skin-guar` (or `view r\guar.nif --model --skinned`).
- Look for: a whole, textured guar (16 meshes, 48 joints) at the NIF's rest
  pose: no stretched triangles, no parts at the origin, legs and claws in place.
- A/B: K flips to the NIF runtime meshes; nothing should move or change
  (rest pose = bind pose for vanilla skins).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: skinloader.

### SN-3: Dark elf head morphs
- Kind: game model. Run: `.\bevyrrowind scene skin-head`.
- Look for: the head matches the NIF (K). M cycles its two geometry morph
  targets (`#1`, `#2`) at weight 1, then none: the face deforms smoothly
  (Bevy morph targets, gathered per mesh from the model-wide weights).
- A/B: K (NIF meshes), M (morph).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: skinloader.

## Character animation (superfargoth `animation.md`, "Bevy integration"; `.sessions/animbevy.md`)

`sf_anim_bevy`: the Bevy-free graph runtime evaluated per character
(`par_iter`), local pose written back to rig bone entities from
`spawn_rig`, skins bound to those bones by name. Gizmos draw one green line
per bone (yellow/orange/red at LOD A1/A2/A3).

## Characters (superfargoth `character.md`, "Assembly at runtime"; `.sessions/charbevy.md`)

`sf_character_bevy`: `mw_actors` recipes resolved and merged per material
off the main thread (`sf_character::assemble`), one `SkinnedMesh` per
material (+ one per morphing head) on an `sf_anim_bevy` rig playing the
baked graph's idle; identical looks share one cached outfit. Expected looks
(vanilla Morrowind / OpenMW from memory, not run): guards in Imperial
steel/templar plates, Ajira in a common robe, Huleeya in light armor with
tail, Fargoth's naked body with a loincloth, guar and cliff racer in their
idles.

### CH-1: The M3 lineup (`bevyrrowind character`)
- Update 2026-10-22 (anim-refactor slice 3): characters now start their idles at their own phase and rate (+-5 %), so same-record NPCs no
  longer breathe in lockstep; the isolated check is `scene level-anim-crowd` (AL-7). Status: awaiting owner there.
- Kind: game models. Run: `.\bevyrrowind character` (Imperial guard, Ajira,
  Huleeya, a female dark elf, naked Fargoth, guar, cliff racer), or name
  ids: `.\bevyrrowind character "caius cosades" "sellus gravius:naked"`.
- Look for: everyone dressed and idling (breathing, no T-pose), heads with
  hair, hands and feet present, mirrored parts (left/right pauldrons,
  gauntlets, boots) on the correct sides and not inside-out, beast feet and
  tails on Ajira/Huleeya. The log lists each character's skinned meshes
  (merged per material + morphing head) and the cache.
- A/B: O swaps every NPC between own equipment and naked (CH-2), F fidget,
  G skeleton gizmos.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  posed; 13-19 meshes per NPC = their material count + 1 head).
- Source: charbevy.

### CH-3: Balmora street with NPCs standing in place
- Update 2026-10-22 (anim-refactor slice 3): idle phase, rate and AI fidget choice (never the same Idle2-9 twice in a row) vary per NPC;
  check no two neighbours breathe in sync (also `scene level-anim-crowd`). Status: awaiting owner.
- Kind: game location. Run: `.\bevyrrowind scene char-balmora`.
- Look for (with `--off walkers`; otherwise wanderers walk, WK-1): NPCs on the streets and bridges dressed and idling, feet on
  the ground, facing their placed direction (vanilla faces -Z; we add the
  half turn). Leveled creatures show the list's lowest entry (scaffold
  `leveled pick`). Fly down for a close look.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  standing in the street).
- Source: charbevy.

### CH-4: Census and Excise Office staff
- Kind: game location. Run: `.\bevyrrowind scene char-census`.
- Look for: Sellus Gravius and the officers dressed (Imperial uniforms) at
  their posts, idling; no part floating off a joint.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  the staff now stand on the floor (fallfix confirmed; before, they fell through at load). Their shadows only
  show with the camera very close, as for every object (interior point-light shadows: 1 light, faded out over
  15-25 m); owner asks for a general shadows pass (ROADMAP backlog).
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

### SB-1: Lineup close-up at elbows and knees, walking
- Kind: game models. Run: `.\bevyrrowind character "@imperial,guard" ajira huleeya "@dark elf,,f" fargoth "@nord,+iron_cuirass+iron_greaves+iron_pauldron_left+iron_pauldron_right+iron boots+iron_gauntlet_left+iron_gauntlet_right" "@high elf,+common_robe_01+common_shoes_01" --walk`
  (Imperial guard, Khajiit, Argonian, female dark elf, Fargoth in his pants,
  a Nord in iron, a High elf in a robe). Fly close (free camera).
- Look for: elbows, knees, wrists, ankles and the neck bend without cracks,
  see-through slits or pieces sliding over each other; no piece is
  stretched or collapsed (a stretched plate is the step blending too far).
  Press B (vanilla): the classic cracks open at the same joints; B again.
  2 runs (larger bends).
  Since animfix (2026-10-12) the lineup is a treadmill: the graph is told the
  cycle's own speed, so walk (1) and run (2) play at their natural cadence
  (before: 0.75x, slow-motion), and the feet play the clip (no foot locks
  without ground: before, a planted foot stuck forward while the clip swept).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive, animfix.

### SB-2: Armored NPC
- Kind: game models. Run: `.\bevyrrowind character "@nord,+iron_cuirass+iron_greaves+iron_pauldron_left+iron_pauldron_right+iron boots+iron_gauntlet_left+iron_gauntlet_right" "@imperial,guard" --walk --camera "0.8,1.5,1.6,0,1.1,0"`.
- Look for: cuirass ↔ greaves, greaves ↔ knee guard ↔ boots and gauntlet ↔
  bare forearm stay closed as the legs and arms swing; plates stay rigid
  in their middle. Pauldrons are the known weak spot (13-22 mm opening at
  the shoulder ↔ upper arm, vanilla 68-99 mm). In Balmora (`scene sb-balmora`)
  the guards walk.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive.

### SB-3: Beast tails with greaves
- Kind: game models. Run: `.\bevyrrowind character huleeya "huleeya+iron_cuirass+iron_greaves" "ajira+netch_leather_greaves+netch_leather_cuirass" "huleeya:naked" --walk --camera "0.5,1.0,-2.2,0,0.8,0"`
  (camera behind them; 2 runs).
- Look for: the tail root against the greaves' pelvis piece and against the
  naked groin. **Known failure**: this seam still opens 44-66 mm in run
  frames (vanilla wider); the rest of the tail and legs should be closed.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive.

### SB-5: Walkers in Balmora
- Kind: game location. Run: `.\bevyrrowind scene sb-balmora` (`--bodies vanilla` to start segmented).
- Look for: walking NPCs' elbows and knees without cracks; press B for
  vanilla and back: no NPC disappears for a frame, no hitch beyond the first
  build of each look. Since animfix: feet planted while walking (no sliding),
  no NPC with legs stretched behind and the pelvis on the ground (WK-7).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive, animfix.

## Node-animated creatures (superfargoth `character.md`, "Node-animated creatures"; `.sessions/nodecreatures.md`)

Creature bodies are cooked for their rig (`mesh.rig`, `sf_model::cook_rig`):
rigid meshes become single-joint parts of their transform node, so
creatures animated per node (scrib, kwama forager, shalk, bonelord,
centurion sphere) and the rigid parts of mixed
creatures (wolf, bear and goblin heads, kwama antennae, storm atronach
rocks) follow the same rig and graph as skinned bodies.

## Walkers (M3 demo: NPCs walk the streets)

`sf_controller_bevy` (ORCA avoidance → kinematic mover → animation feed) on the characters
`sf_character_bevy` spawns, driven by `mw_ai`'s `PathWalkScaffold` (app `walkers.rs`). Keys in
the streamed view: Shift+J mover gizmos, K foot placement on/off. `--off walkers` turns it all off
(CH-3 then shows everyone standing).

## Synthetic scenes (framework examples)

Run from `D:\Gamedev\Bevyrrowind\superfargoth`; they bake their data on
first run (cached afterwards). Each isolates one system on generated
content, so a bug seen here isn't Morrowind content.

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

### HI-3: Torch-bearer at night
- Kind: game models. Run: `.\bevyrrowind character "imperial guard" "volene llervu" "@imperial,guard" --night` (N toggles day/night).
- Look for: the Imperial guard holds a torch in the left hand (its shield
  replaced, as out of combat in vanilla); warm flickering light on the floor
  and on the neighbours; by day (N) the torch goes and the shield comes back
  (props swap, outfit meshes stay). No flame yet (particles not rendered).
  The carrier itself looks overexposed near the torch (inverse-square near
  field; calibration follow-up).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  torch in hand, orange light pool on the floor and the lineup).
- Source: heldrender.

## Particles on actors, bows, carried-light near field (nif-runtime.md "Particles on actors"; `.sessions/actorparticles.md`)

Torch flames and creature particles are NIF emitters anchored at a skeleton
joint or the prop root (world-space particles trail a moving joint); bows are
skinned props on their own small skeleton (rest pose); the carried torch
light keeps 0.7 m off the carrier, dimmed to read "fully lit" at a third of its radius (torchfix). Compare with OpenMW.

### AP-2: Flame atronach and ancestor ghost particles
- Kind: game models. Run: `.\bevyrrowind character atronach_flame ancestor_ghost --night`;
  `RUST_LOG=bevyrrowind::actor_particles=debug` logs each emitter's joint and live emitter joint.
- Look for: the atronach's fire (4 emitters: one from `Bip01 Pelvis`, three
  from the `Weapon` hand joint) burning on the body and following it while it
  idles; the ghost's mist sprays (4 emitters at its root) under it. Particles
  hide beyond 60 m and pause off view.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  shows only the pelvis glow; the hand emitters may sit outside their emission
  window (they run on a free clock, not the animation time) or need
  `NiBSPArrayController` vertex emission: check against OpenMW. particles2:
  the hand flames are moment-keyed (cast) and wait for `emit:` curves; the
  body flames now cover the skeleton (PA-1). particles3: the hand flames
  emit in `Attack1`-`3` only, the ghost's sprays in its deaths only, so an
  idling lineup shows neither (PA-3).
- Source: actorparticles.

### AP-3: Archers hold bows
- Kind: game models. Run: `.\bevyrrowind character "imperial archer" "@wood elf,archer" "@dark elf,archer" --drawn`.
- Look for: a curved bow in hand (long bow, chitin short bow, steel longbow:
  skinned props, 1 joint each), not missing; held still at rest (no string
  pull yet). Log with `RUST_LOG=sf_character_bevy=debug`: `... is a skinned prop`.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  the Imperial archer's curved bow).
- Source: actorparticles.

## Lighting: night exposure floor, point-light shadows (superfargoth `sky-lighting.md` "Exposure range", "Local-light shadows"; `.sessions/lighting.md`)

Shots in `shots/lighting/` (`before_night.png` / `night_old.png`: the old night; `night2.png`: floor 3.5 + night ambient 1;
`cave_in1*.png`; `torch_stage.png`).

### LT-2: Dark interior keeps its lights (`scene lt-cave`)
- Kind: game location. Run: `cargo dev scene lt-cave` (Addamasartus, mine tunnel).
- Look for: rock texture readable in the torch pools, the far tunnel dark but not black; the wooden beam and post
  shadow the rock (point-light shadows). A/B `--portals`: auto exposure meters this cave at the same level with or
  without the floor (luma 50.8 vs 50.4): the floor never darkens it.
- Status: partial, owner 2026-10-19 (round 9, scene lt-cave): lighting OK (rock readable, far tunnel dark not black); beam/post shadows only when very close, and very grainy (SH-13).

### LT-4: Lamp light cut along block edges, view-dependent (owner report; ROADMAP 1.12 calls it "LT-2")
- Kind: game location. Run: `.\bevyrrowind scene lt-night-floor --hour 22`, Balmora canal by the lantern arch, then
  turn the camera / walk along the staircase.
- Seen by owner 2026-10-21 (bw-owner10, SK-1 time-lapse): lamp light is cut along screen-aligned block edges on the
  staircases and changes with the view ("like seeing the camera's cone"). Evidence: `Desktop/jagged.jpg`
  and `jagged.mp4`. Static, so an agent can repro by screenshot.
- Suspects: clustered light assignment (Bevy cluster z-slices / limits, far-plane or range cull), shadowed-lamp budget
  (High = 8) chosen by view, light frustum culling vs range sphere, far light sprites.
- Owner A/B 2026-10-21 (release, same spot): blocks appear ONLY with F4 = DLSS quality; F4 = taa (native) is clean.
  Trigger is DLSS upscaling: lead is the cluster grid sized for one resolution and indexed at the other.
- Cause (lightclusters 2026-10-21, confirmed by source + release A/B shots `shots/lightclusters/`): Bevy 0.20 bug.
  `clustered_forward.wesl` `view_fragment_cluster_index` multiplies a render-res `frag_coord` by `cluster_factors`
  sized for the output viewport, so under `MainPassResolutionOverride` (DLSS) pixels read the grid's top-left part.
- Fix parked, unbuilt: `superfargoth/vendor/parked/LT-4-cluster-main-pass-viewport.diff` (rescale by
  `viewport / main_pass_viewport`, exactly 1.0 without DLSS). File an upstream Bevy issue.
- Status: known bug, on the rendering-bugs list (ROADMAP 1.12).

### LT-3: Point-light shadows, torch-bearers (`scene lt-torch-shadows`; stage: `character "imperial guard" --night`)
- Kind: game location + game models.
- Look for: railings, crates and pillars near lamps throw shadows on the street and walls; a torch-bearer within
  ~15 m throws a shadow on the ground (and on a wall beside them) that moves with the torch; walkers passing ~20 m
  lose their shadow by fading, not popping; lantern cages do not stripe the walls (0.25 m shadow near plane). On the
  stage (`.\bevyrrowind character "imperial guard" --night`): a long shadow of the guard on the floor away from the torch.
- A/B: `--point-shadows 0` / `2` / `4`; default by `--quality` (Low 0, Medium 0, High 1, Ultra 4). Cost (1440p, RTX 4060 laptop,
  median frame, N = 0/1/2/4/8): 9.4 / 10.2 / 13.3 / 13.8 / 13.6 ms; render-thread CPU for the casters in each
  light's range is most of it (GPU +0.35 to +1.6 ms). With the High default only the nearest light (a torch
  within 2x the nearest lamp's distance) shadows; `--point-shadows 4` shows the lamps' railings and crates too.
- Status: rejected by owner 2026-10-19 (round 5, Balmora lamps at 22:00): see SH-13.

## Physics (superfargoth `physics.md`, bevyrrowind `physics-adapter.md`; `.sessions/physics-p2.md`)

PH-3 is specified in `docs/specs/physics-adapter.md` and lands with task P10; PH-2 (first half, P7) and PH-4 (carry) are below.

### PH-2 (first half, P7): a dead guard ragdolls on the Balmora stairs
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

### PH-7: Item body shapes (`body.cook`)
- Kind: game location. Run: `.\bevyrrowind scene ph-tradehouse-items` (Arrille's Tradehouse with `--physics-debug`;
  `G` toggles). Placed items (MISC, WEAP, ARMO, CLOT, BOOK, ...) draw orange: their `body.cook` shape (a box, capsule,
  cylinder or sphere when one fits within 15 % of the hull, else one hull of at most 32 vertices, or up to 8 hulls for
  concave closed models). Statics stay cyan. Since P4 items are sleeping dynamic bodies (held ones `Fixed`).
- Look for: every item on the counter and shelves has an orange shape hugging it (bottles as cylinders/capsules, books
  and plates as flat boxes, weapons as thin boxes or capsules along the blade); no shape offset, rotated or scaled
  away from its item; no item without a shape; nothing grossly bigger than its item.
  Bookshelves and cabinets that hold items draw their render mesh in cyan (`support.cook`), not a box.
- Status: partial, owner 2026-10-19 (round 11): shapes very good almost everywhere; some items have none (the ceiling candle holder, a small box on Arrille's desk: lights/containers not cooked?). Phase 3.
  clutter in front, Arrille behind his counter at the right; sanity screenshot `shots/physics/ph7_tradehouse_a.png`
  shows orange shapes on the book, staff and shelf items, lined up. `SF_ITEMS_CELL="<cell>"` (+ `SF_ITEMS_NEAR="x,y,z"`
  for the statics around a point) on `cargo run --release -p mw_bake --example body_report` lists a cell's items with
  their `items.check` result.
- Source: physics-p3, physics-p4b.

### PH-9 (P8): knockdown and get-up

Status: **partial, owner 2026-10-19 (round 10): walkers fall and stand back up, but the fall animation is terrible (basically none). Phase 3.** Earlier: agent-run 2026-10-18 (p9fin): impacts ≥ 15 N·s (`KNOCKDOWN_IMPULSE`) knock a walker
down; the get-up root comes from the `Knockout@tail` clip's first frame (`clip_root_body`; the `LYING_REST_GET_UP`
scaffold only for skeletons without that clip, which logs a line).
Agent run (release, no keyboard): `$env:BEVYRROWIND_AUTO="ball_at_nearest@3,shot:shots/p9/knock.png@4.3,shot:shots/p9/getup.png@8,exit@10"; .\bevyrrowind scene wk-balmora`.
Log: `ragdoll: knocked down ... (25–36 N·s)` for the 12 m/s ball, `got up (heading ...)` 2.2–2.5 s later, no
scaffold line. Sanity shots `shots/p9/knock.png` (lying on the street, ball beside), `shots/p9/getup.png` (standing,
feet under it). A second knockdown right after a get-up logged a 5.6 cm joint stretch peak (open, P8b).
Scene: Balmora, a walker on the Temple stairs; free camera 6 m away; press `V` to throw the 3 kg ball at the walker.
What to look for: the walker braces and falls with the hit (no pop from the animated pose into the ragdoll), lies
still within ~2 s, then gets up from where it lies with its feet under it (root placed under the pelvis); face-down
bodies roll over during the get-up blend (the `ROLL_TO_BACK` scaffold); no joint stretch, no spin of the root.

### PH-10 (P9): kinematic hit reactions and tail spring bones

Owner batch 2026-10-18: "didn't work". The window was closed ~13 s in, before the first ball (the `@` times count
from the settled start, ~13 s after launch in release). Orchestrator re-run (`ball_at_nearest:2@4,...`): the balls
fly, but the first 2 m/s ball knocked its walker down at **72 N·s** ~1.9 s after the throw (likely the walker
walking into the ball on the street), and neither soft ball logged a `hit react`. Bug, back to physics (P9).

Fix 2026-10-18 (p9soft): impacts are now the incoming body's momentum at first contact (`PhysicsEvent::Hit`,
`sf_physics::impact_impulse`: `m · min(v_by·n, closing speed)` from rapier's pre-solve `CollisionEvent::Started`),
not the solver's contact force (a walker pressing a resting 3 kg ball into a wall measured 423 N·s that way; now no
hit). Second cause, found by test: the character push zone (wider than the capsule) batted incoming soft balls away
before they touched (no hit at all) and braked balls from behind; pushes now skip trailing-side and incoming
(> 0.25 m/s) bodies. `impact_threshold` 5 → 1 N·s. Tests: sf_physics `character_tests` (soft hit 6.0 N·s, chasing
ball = closing speed, approaching walker, walking into a resting ball = no hit). Agent re-run (release, the
orchestrator's `ball_at_nearest:2@4,...,ball_at_nearest@16` run): `hit react: 3915v0 (6.4 N·s, strength 0.43)`
(second soft ball), `knocked down 3915v0 (35 N·s)` (12 m/s), `got up`; no knockdown from a ball on the street.
Open: the first soft throw at a newly framed walker fell short of the chest (sanity shot: ball in the air at knee
height in front, no contact; an aim issue in the `BEVYRROWIND_AUTO` throw, not detection; it hit in the previous
run, 2.6 N·s); the knocked-down ragdoll tumbling into a neighbour gave it ~10 small `hit react`s (1–5 N·s) in
1.5 s (real contacts; no per-character cooldown yet); hit strength tuning not done (owner: don't polish).
Awaiting owner.

Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
strength 0.40)`, sanity shot `shots/p9/hit.png` at the hit; the flinch itself needs eyes on the live run; tails not run).
- Kind: game location. Run (release): `$env:BEVYRROWIND_AUTO="ball_at_nearest:2@3,shot:shots/p9/hit.png@3.88,ball_at_nearest:2@6,exit@9"; .\bevyrrowind scene wk-balmora`
  (a new target is framed first and the ball leaves 0.5 s later; a 3 kg ball at 2 m/s released 0.6 m from the
  chest, about 6 N·s, under the 15 N·s knockdown; lobbed from 4 m any ball lands at ≥ 6 m/s, 18 N·s, a knockdown;
  the `V` key always throws at 12 m/s, a knockdown). Hits under 1 N·s (`CharacterPhysics::impact_threshold`) don't
  reach the graph at all. Tails: `.\bevyrrowind view "Seyda Neen"` near the guars
  south of the lighthouse road, or `character --walk` on an Argonian/Khajiit.
- What to look for: the hit walker's torso and head flinch away from the ball (peak ~0.09 s after the hit, back on
  the animation within ~0.5 s; no pop, no lasting offset, feet stay planted); log `hit react: ... strength`. A guar's
  or beast race's tail trails a little when it starts, stops or turns and settles without jitter or drift; at
  `BEVYRROWIND_FPS_CAP=30` and uncapped it looks the same.
- Known: hit strength is impulse / 15 N·s (`HIT_REACT_IMPULSE` 1 N·s minimum); no additive directional hit clips yet.

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

### PH-4: Carry, drag and throw (dev camera; corpse drag waits for P7)
- Kind: game location. Run: `.\bevyrrowind scene ph-tradehouse-items` (Arrille's Tradehouse; or any `view`). The
  free camera is the holder (no player yet), Strength 50 (`--env BEVYRROWIND_STRENGTH=30` / `=100` to compare). Keys:
  **hold the left mouse button** on an item under the screen center (within 2.5 m) to grab it, release to drop; while
  holding, **`V`** throws it (instead of a ball), **`[` / `]`** turn it 15°. Frame rate:
  `--env BEVYRROWIND_FPS_CAP=30` (or 60, 144; for 144 add `--env BEVYRROWIND_PRESENT=mailbox`). The log prints
  `carry: Grabbed { mode, mass }`, `TooHeavy`, `Thrown { speed }`, `Broke`.
- Look for: a bottle, bowl or book lifts into view and stays put relative to the view while you fly and turn, at 30,
  60 and 144 fps (no vibration or shimmer; it may lag a little on fast turns and settle); it pushes other clutter
  aside but never you. Throws arc and knock things over (S 50: 6 N·s, a 0.5 kg bottle leaves at the 15 m/s cap).
  Heavy items (over 15 kg at S 50: crates, sacks, barrels with a body) drag: they stay on the floor, slide or tip
  toward you, the camera slows (`1 − m / (2·drag mass)`); a dragged object is released by `V`, not thrown. Over
  120 kg (S 50; 88 kg at S 30) the log says `TooHeavy` and nothing moves. At S 30 a 12 kg item drags where S 50
  carries it. Pulling a held item through a wall for 0.3 s drops it (`Broke`). Dead bodies by a limb: P7.
- Automated (sf_physics `carry_tests`): view-space spread of a carried item while walking a 1.4 m circle at 1 rad/s:
  0.008 / 0.010 / 0.044 mm at 30 / 60 / 144 fps (gate 1 mm); a 0.3 kg throw leaves at 15.00 m/s (cap, within 1 %);
  a 6 m/s throw from 1.5 m lands 3.320 m out (ballistic 3.333); carry / drag / too heavy at the S 30 and S 100
  limits; a 40 kg crate dragged 2.3 m in 3 s; an 11 kg item on a 40 N grip breaks away; the holder's capsule never
  touches what it holds.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: physics-p6.

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

## Haze and dust (superfargoth `weather.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

### WA-1: Open sea, W2 FFT ocean (`scene wa-seyda-noon`, `wa-vivec-waterfront`, `wa-bittercoast`, `wa-sunset-sea`, `wa-night-moons`; A/B `--water scaffold`, `--water off`, `--water-wind 2` / `14`; example `water_ocean`)

Status: partial, owner 2026-10-19 (round 12): one sea, no seams, nice transparency and glitter. Issues: (1) moiré in the distance (Phase 1, TAA/normal mips); (2) shore foam band artificial, near pure white, fixed width, 'offendingly ugly'; (3) shore damping wrong: water goes 100 % flat in the last centimetres; want swash (water moving up and down at the shore), wavelets over the first ~20 m rising gradually to full open-sea waves; (4) unsure wave height follows the weather (spec: h0 blend from WindState; verify); (5) no whitecaps on wave crests (spec: Jacobian foam, not visible); (6) owner asks for crest subsurface scattering (light through wave tips). (2)-(6) Phase 4.
Look for: waves read as one sea (swell + chop) and move; sand visible through shallow water, colour deepening
offshore; a thin contact-foam line where water meets sand, rocks, piers and hulls; sun glitter widening with distance,
no sparkle noise; far water hazed like the land (aerial perspective, morning froxel fog); no square seams, cracks or
steps (clipmap rings) and no gap at the horizon; a sunset path on the water; at night moon glints, sea not black.
Known: reflections are the environment map only (SSPR/probe at W4), no shore wave damping yet (W3), the water may read
too milky turquoise (W1 placeholder optics, tuned at W3), whitecaps at storm winds untuned.

### WA-2: Shore and region optics, W3 (`scene wa-seyda-noon`, `wa-bittercoast`, `wa-vivec-waterfront`, `wa-sunset-sea`, `wa-night-moons`)

Status: **region colour confirmed by owner (2026-10-18)**: muddy water at Seyda Neen, bluish toward Suran.
**Unfinished (owner 2026-10-18):** at `wa-vivec-waterfront` the shore foam is a uniform white band everywhere
(not thin and broken); at `wa-sunset-sea` the swell is very small, even with `--water-wind 14` and `20`. Parked:
no more work on these now (owner); pick up with the foam/shore and weather-driven waves work (SU1, WE3). Shots: `shots/w3/` (compare with `shots/w2/`).
Addresses the owner's 6a notes: water no longer milky turquoise (body colour sized like real backscatter, 2-5 %);
colour by region from the cell map (Seyda Neen / Bitter Coast swamp olive-brown, Ascadian lagoons teal, open Inner Sea
deep blue, ash grey-brown, the north steel blue; ≈ 117 m blend between cells); shore foam per region and fainter; waves
calm toward shores and in enclosed water (`water.shore`: signed shore distance + openness, baked at start in ~0.5 s).
Look for: the hue changes between regions without a visible seam; swamp water murky and calm near Seyda Neen; the
shore line is a thin broken foam band, not a white outline; swell dies on beaches, open sea keeps it; no water drawn
under land. Known: from above (`wa-bittercoast`) the swamp reads grey (sky reflection dominates at that angle; SSPR at
W4); at night the water reads lighter than the land (env-map reflection, W4); the shore foam band shape is still the
generic travelling band (wet sand band waits for SU1); waves by weather are WE3.

### RO-A: Translucents cast no shadows; distant lights and rain fade into fog (render-order RO-1/2/3, rofix1)
Status: **step 1 confirmed by owner 2026-10-19** (no flame/glow shadow blobs in Balmora); steps 2-4 not checked. Agent check: dev build renders, shaders compile, rain at night visible (`view "Seyda Neen"`, hour 22, rain); fading not judged.
1. `view "Balmora" --hour 11 --weather clear`: torch flames, glow cards and glass cast no shadow on sunlit walls/ground; trees (cutout leaves) and solid props still do. Compare `--off transfog` (old: flames/glows cast near-solid shadows).
2. `view "Seyda Neen" --hour 22 --weather foggy`, fly ~1-3 km out: the far town's light points dim with distance and vanish into the fog instead of staying full-bright.
3. `view "Seyda Neen" --hour 15 --weather rain` (then `foggy` + rain via C): far rain streaks fade into the fog colour; near streaks unchanged.
4. Ghostfence (`view "Ghostgate" --hour 12`): the field casts no shadow.

### RO-B: Fire/torch smoke not drawn in front of the sky, or not hiding objects behind it (owner report)
- Kind: game location. Run: `.\bevyrrowind scene hi-balmora-night` and `.\bevyrrowind scene particles-census-fire`;
  look at torch/brazier smoke with the sky (and then a wall or NPC) behind it.
- Seen by owner 2026-10-21 (bw-owner10, AU-2 and PA-4): smoke sometimes goes missing in front of the sky and/or does
  not hide the objects behind it properly. Render order (translucent sort vs sky/clouds/fog passes), Phase 1. Not the
  GPU particle path itself (PA-4 confirmed). ROADMAP 1.12 calls it "RO-8" (render-order.md RO-8 is a different, fixed item).
- Ruled out (smokeorder 2026-10-21, by code): sky, stars, cloud/fog composite all draw before the transparent pass.
  Open: H1 particles sort by emitter origin (no bounds, `NoFrustumCulling`), so smoke metres up sorts as the flame
  (RO-10); H2 particles always lit (`particles.wesl` ignores unlit): dark smoke on dark sky; H3 TAA, no smoke MVs.
- Status: known bug, awaiting diagnosis; next: one A/B run at an exterior fire against the sky (F4 off, `--off
  skymotion`, `--off transfog`), hours 12 and 21.5 (`.sessions/smokeorder.md`).

### LM-2: Leaf side findings (from LM-1, renderfix1)
- With `--off leafmask`, blended leaves vanish entirely at ~100 m+, so the A/B no longer shows "blended as before".
- Leaves (and water) look bright white at night (`shots/renderfix1/night_amb4.png`, Balmora): not traced; hypothesis
  is still-Blend leaves on the forward FroxelFog path.
- 4 textures could not be decoded, so they are never promoted to cutouts: `poxyclouds2`, `tx_banner_temple_04`,
  `tx_eggmine_banner*`, `tx_rope_net_01`.
- Status: known bug, logged by renderfix1 2026-10-21; not scheduled.

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
  roofs unchanged (`--weather rain`). Storm direction from Red Mountain: Phase 4; tall clouds: open.

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

One greybox for the animation checks, six areas apart. Open the scenes below (or `scene level-anim-overview --camera <name>`; cameras are in the
level file). NPCs only run their scripts near the camera (processing range), so fly to an area first. All entries: kind synthetic level, status
awaiting owner. Known issue ids this feeds: SN-2/3 (guar, beasts), CH-1/3/4 (dressed NPCs), SB-1/2/3/5 (spring bones, tails), HI-3, AP-2/3 (not
here), PH-2/9/10 (ragdoll, knockdown, hit reaction).

### AL-1: Animation lab overview
- `.\bevyrrowind scene level-anim-overview`. Look for: six areas apart (ledges + props near the origin, loop south, slopes/stairs west-north,
  drops north-east, idles south-west), nothing sunk or missing, every NPC dressed. Status: awaiting owner.

### AL-2: Walk loop, turns (SB-1, CH-1)
- `scene level-anim-loop` (cameras `loop`, `loop-close`). Fargoth walks a 12 m square, Elone runs a 24 m square.
- Look for: foot sliding or skating at the four corners, a smooth turn (no snap, body leads), cycle speed matching the move speed at walk and run,
  spring bones and tails settling after turns. Status: awaiting owner.
- Slice 4b (headless, 2026-10-22): sidestep and backpedal no longer slide (walk right 30, run back 55, run strafes 32-36 cm per stance,
  now 0). Strafe (A/D) and back-pedal (S) in a free-walk `view`, at walk and run: feet planted, no skating. The price: speed matching
  now plays strafe clips up to 2.2x (was 1.5x), so a run-strafe has a brisk cadence. If it looks frantic, lower SpeedMatch `max` in
  `mw_humanoid.animgraph.ron` (and `build.rs`) and accept some slide. Status: awaiting owner.

### AL-3: Ramps and stairs (feet on slopes, SB-1)
- `scene level-anim-stairs` (cameras `slopes`, `stairs`, `stairs-close`). Runner on 10 deg, walkers on 20 and 30 deg and on 0.15 m x 12 and
  0.25 m x 8 stairs, all to a 2 m plateau and back.
- Look for: feet on the surface (no floating or sinking, no knees through treads), lean with the slope, no pelvis jitter on the steps, a
  steep (30 deg, 0.25 m) climb still plausible. K toggles foot placement (A/B).
- Slice 5 (headless, 2026-10-22): ramps within 2 cm (toe no longer sinks on 30 deg); comfy stairs going up: toes lift onto the next
  tread (was 16 cm into it, now about 5; heel may hang over the nosing). Known bad: the steep 0.25 m flight at walk speed (feet sink or
  float up to ~0.6 m; needs shorter steps, not IK). Status: awaiting owner.

### AL-4: Traversal heights (future vault / mantle / climb)
- `scene level-anim-ledges` (cameras `ledges`, `ledges-close`); F11 for the player, walk into each box and the 3 m wall.
- Look for: the capsule stops at each ledge and the body does not clip into the box (no vault, mantle or climb exists yet: this is the
  baseline for them); heights readable against the 1 m grid. Status: awaiting owner.

### AL-5: Props push (PH-1, physics-adapter.md)
- `scene level-anim-props` (cameras `props`, `props-close`); F11 (player under the camera), push crates, boxes and the table; `V` throws.
- Look for: the NPC brushing the crate row without sliding through it, light boxes shoved with a lean or step (not a stuck walk cycle), the
  300 kg block unmoved, crates not jittering. Status: awaiting owner.

### AL-6: Falls and ragdoll (PH-2, PH-9)
- `scene level-anim-drop` (cameras `drop`, `drop-side`, `drop-close`); `G` shows collider wireframes, `I` kills the NPC under the crosshair, `V` throws.
- Look for: the walker stepping off the 3 m edge (a fall animation or a snap? it must not freeze), the dummy knocked down every 10 s
  (`@impact` 25 N*s: ragdoll, then get-up), with `G` the capsules following the mesh, no pop into the ragdoll, no spin on the 15 deg slope.
  No 6 m fall exists for actors (no way to place one on the platform): throw or fly. Status: awaiting owner.

### AL-7: Idles, reactions, other rigs (SN-2/3, CH-3, SB-3, PH-10)
- `scene level-anim-idles` (cameras `idles`, `idles-close`). Left to right: Fargoth, a Khajiit (Ajira), an Argonian (Huleeya) fidgeting with
  different idle variants; Elone hit-reacting every 4 s; a dummy knocked down every 12 s; a guar; a mudcrab.
- Look for: distinct fidget clips (no T-pose, no sliding), flinch and recovery on the hit, Khajiit and Argonian legs and tails bending
  correctly, creatures standing idle (no graph fidget for them yet). Status: awaiting owner.
- Variety (CH-1/CH-3 crowd): `scene level-anim-crowd` (camera `idles-crowd`): four identical Fargoths, same idle and fidget set. No two neighbours
  breathe in sync, fidgets begin at different times and vary, none repeats back to back (`@fidget`: Idle2/3/4 at 50/30/20, every 3-8 s).
  Status: awaiting owner.

### AN-7: Capsule steady under a walker (owner pass 2026-10-22, slice R3-1)
- `scene level-anim-loop` (camera `loop-close`), `G` for collider wireframes.
- Look for: the orange capsule glides with the walker and the runner, a constant hair behind them (one physics
  step, ~2 cm walking, ~7 cm running), no shaking or stutter. Was: target from the last frame's feet, beating against
  the 60 Hz step. Status: awaiting owner.

### AN-1: Planted feet stay put on distant walkers (slice R3-1)
- `scene level-anim-overview` (camera `overview`, all walkers at A2 LOD there), then `level-anim-props`/`loop`.
- Look for: on the props walker (beside the orange crates) and the loop walker, the standing foot doesn't drift with
  the body during its stance, far and near. Was: A2 updates got half the body motion. Status: awaiting owner.

### AN-3: Footsteps on the feet (slice R3-1)
- `scene level-anim-loop` (camera `loop-close`), sound on; then `level-anim-props` for the walker's stops.
- Look for: each step sound as a foot lands (left/right alternate), at walk and run, none while standing. Sounds come
  from the clips' `SoundGen` keys now (were one per 0.7 m of travel). Status: awaiting owner.

### AN-2: Walkers stop within a step (slice R3-2)
- `scene level-anim-stairs` (camera `stairs`): the ramp/stair walkers and the runner turn round at each end after
  a 1 s wait; `level-anim-overview` shows the same area from afar.
- Look for: a walker slows over its last step (about 0.5 m, under 1 s) and stands, the runner over 2-3 steps
  (about 2 m, 1.5 s); no long glide, no slow-motion walk at the end, idle blends in as it settles. Was: braking
  from 2 m out plus a 1.5 s creep at 0.25 m/s (3 s). Status: awaiting owner.

### AN-4 Knockdowns always come back (awaiting owner)
Scene `level-anim-overview` (or `level-anim-drop`), G on. Throw `V` balls at walkers, the knockdown dummy, the guar
and the mudcrab, hard and soft. Look for: every knocked-down character gets up within ~6 s, then has its capsule
(G) and walks/idles again; a creature without a ragdoll logs "knockdown cancelled" and keeps its capsule. Never a
character frozen lying or standing without a capsule.

### AN-5 Get-up starts from the lying pose (awaiting owner)
Scene `level-anim-overview`, the `@impact` knockdown dummy (idles row, every 12 s). Look for: at the moment the
ragdoll hands back, no snap: the limbs flow from where they lie into the stand-up over ~0.3 s, the pelvis doesn't
jump, and the body doesn't rise or sink to meet the ground.

### AN-6 Get-up ends on the capsule (awaiting owner)
Same scene, G on. Look for: when the get-up starts the capsule appears under the lying pelvis (not ~1 m away), the
character stands up onto it, and nothing slides when the clip ends and idle takes over.

### AN-8 Creature capsules fit the creature (awaiting owner)
Scene `level-anim-idles` (or `-overview`, camera `idles-close`), F7 + G on. Look for: the mudcrab and the guar (last two
on the row) have a physics shape over their whole body (an upright cylinder, orange, as wide as the shell or body and as
high as the back), not the old 0.4 m column inside the shell; people keep their capsule. Throw a ball (`V`) at the crab:
it should hit the shell's outline. Known: the shape is round, so a long body (guar) is covered with some slack at the sides.

### AN-9 Planted foot holds through the heel roll (awaiting owner)
Scene `level-anim-loop` (camera `loop-close`). Look for: a planted foot stays put from heel-strike until the toe leaves the
ground; at weight transfer (the other foot landing) the heel rises and the foot rolls over a fixed toe, with no forward
slide. Any correction happens while the foot is in the air. Fix: cooked `roll.<foot>` curves (heel-off to toe-off) pin the toe.
Note (run 2): the male/female skeletons have no toe bone (only the beast races have `Toe0`); the cook and compiler use the foot's
socket child as the toe, so the roll works on humans too. Gated: `slope_walk` forward walk toe slide 2 cm per heel-off (14-16 cm
without the roll curves). Strafes not gated (lead: walk right 7/15 cm, likely swing-foot skim, check in the scene).

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

### AN-10, AN-13, AN-14 Get-ups (awaiting owner)
Scene `level-anim-idles`, camera `idles-close`. AN-13: throw a ball (`V`) at the guar: it falls, gets up once and stands
idle (it used to loop its knockout clip forever). AN-10: knock dummies down until one lands face down: it rolls onto its back
while rising, with no fast spin about the vertical. AN-14: knock someone down, press `I` on them while down: they stay dead.

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

## Sound: NPC voices, boots, swinging doors (superfargoth `audio.md`; ROADMAP 2.6, session voices)

Dev builds only (`cargo dev ...`): the glue is behind the `dev` feature until these pass. Agents cannot hear; the
synthetic and app tests cover selection, timing and routing (`mw_audio` voice tests, `audio::tests`). Run
`RUST_LOG=bevyrrowind::audio=debug` to see each line's subtitle (`audio: <npc> says Hello: ...`).

### AU-3: NPC greetings, idle chatter, lip-sync level (`cargo dev view "Seyda Neen, Census and Excise Office"`)
- Fly the camera up to a standing NPC: within ~2-3 m (AI Hello x 6 units) they say a Hello line matching their
  race and sex, from their position, once; stay close: no repeat; back off past ~7 m and return: greets again.
- Stand among NPCs a few minutes: now and then an idle line (about one per NPC every ~3 minutes on average).
- Music dips (~8 dB) under speech and comes back over ~1 s. Never two lines at once from one NPC.
- Known gaps: no line-of-sight test (voices reach through walls), quest/local filters see stage 0 and no locals.
- Status: awaiting owner.

### AU-4: Footsteps by boots (`cargo dev scene door-balmora-guild`)
- Walkers in heavy boots (guards) step heavy, light/medium armour boots light/medium, shoes and bare feet bare.
- Status: awaiting owner.

### AU-5: Swinging-door sounds (`cargo dev view "Seyda Neen, Census and Excise Office"`, any inner door)
- Activate a non-load door: its open sound at the door; again: its close sound. The door itself does not move
  yet (the swing is a doors.rs follow-up), so the open/close order is the only state.
- Status: awaiting owner.

## Player (ROADMAP 2.1; `apps/bevyrrowind/src/player.rs`)

### PL-1: Third-person walk and the use prompt (`cargo dev view "Seyda Neen"`, 10:00)
- Starts in player mode (capsule scaffold). WASD, Shift, Space jump, Tab 1st/3rd, wheel zoom; walk to the Census and
  Excise Office door, then to an NPC (Fargoth by the lighthouse path).
- Look for: smooth walk and camera (no jitter, camera pushes in against walls); under the centre dot the prompt reads
  `[E] Open  <door>` / `[E] Talk  <npc>` / `[E] Take  <item>`, it clears when you look away or step past ~2.7 m;
  E on the door travels, the body arrives at the marker facing out.
- Fixed 2026-10-25 (player3), check: focus is now a physics ray plus actor capsules, so in the Census and Excise Office
  the dot on an NPC's chest reads `[E] Talk  <npc>` (was `Take  Broom`); walls block items behind them.
  `cargo dev view "Seyda Neen, Census and Excise Office"` starts on the floor at the door's arrival marker, facing in
  (was: on the roof, grey void). E does nothing while the journal or dialogue is open.
- Status: awaiting owner.

### IV-1: Take items and Fargoth's ring (`cargo dev view "Seyda Neen, Census and Excise Office"`) (awaiting owner)
- E on a loose item (bottle, plate, book): it vanishes at once and stays gone after leaving and re-entering the
  office (overlay removal); log line `take: <id> (now N)`. Doors, NPCs and statics are unchanged.
- E on the barrel by the desk (`chargen barrel fatigue`): log `take all from ... [(ring_keley, 1), ...]`; the barrel is
  now solid (containers got collision). Then talk to Fargoth outside, topic `ring`, "Yes": quest Fargoth's Ring
  finishes (J shows it), the ring leaves your inventory. Shift+U (dev ring) still works.
- Known: no container window yet (the barrel gives everything), no theft for owned items, no weight.
- I (pad Y / Triangle, rebindable on F10's gameplay rows) opens the inventory: record names, counts, DDS icons, tabs All/Weapon/
  Apparel/Magic/Misc; after the barrel it lists Fargoth's Ring. I or Esc closes; you can't walk or press E while open.
  Look for: icons drawn (not blank squares). Read-only: no equip/drop yet. Shot: `--open-ui inventory` (dev sample items).

### SV-1: Quicksave / quickload, save, quit, load (`cargo dev view "Seyda Neen, Census and Excise Office"`) (awaiting owner)
- In player mode take the barrel by the desk (ring) and a loose bottle, step into the middle of the room, Tab to first
  person, F5: toast "Quicksaved" (log `saved ...\saves\single\<world>\000001-quick.sfsave`).
- Walk elsewhere, take another item, F9: fade, you stand where you saved facing the same way, first person, toast
  "Loaded"; I lists the ring; the barrel gives nothing more; the first bottle stays gone.
- Go outside (door), F9: fade back into the office at the saved spot (cross-worldspace load).
- Quit, start again, F9 straight away: same spot and inventory (newest quicksave of any world).
- Load correctness: F5, take a bottle and knock a crate or basket across the floor, open the inventory (or talk to
  someone), F9: the menu closes, the bottle is back once (no double), the crate lies where it was at the save; you can
  walk and look right away. Jump, F5 mid-air, F9: you land as from the jump.
- Known: F5/F9 rebind on F10 (gameplay rows), no gamepad default. Dev keys moved: far tint Shift+F6, copy camera Shift+F12.

### IN-1: Keymap controls, rebind, gamepad, sneak hold vs toggle (`cargo dev view "Seyda Neen"`, 10:00)
- F10 opens the keybinds screen (UI + gameplay rows). Rebind Jump (keyboard slot) to J, close with F10 or Esc; J jumps,
  Space doesn't. Quit, start again: J still jumps (saved in `profiles/<profile>.toml`, `[bindings.gameplay]`).
- Gamepad (Xbox layout): left stick walks (dead zone, no drift), right stick looks at a steady rate, A jumps, X uses
  (door / NPC prompt), L3 sprints (held), B sneaks while held, R3 switches first/third person.
- Ctrl / B: sneak only while held (default). On the Sneak row press the Hold cell (reads Toggle): one press sneaks,
  the next stands; restart keeps Toggle. Sprint row works the same. Look for: no stuck sneak after closing the menu.
- Labels (uipolish, awaiting owner): rows read Move Forward / Move Back / Move Left / Move Right, Jump, Run, Sneak,
  Activate, Toggle POV, Inventory, Camera Zoom, Look; menu rows Select, Back, Menu Up, Previous Tab... F10 no longer
  toggles clouds (Shift+C does, or console `flip clouds`).
- Status: menu works (owner 2026-10-25); labels fixed (above). Rebind persistence, gamepad, hold/toggle not reported yet.

## Dialogue and journal (superfargoth `journal-ui.md`, `ui.md`; ROADMAP 2.4 / 2.2; `.sessions/dialogue2.md`)

### DJ-1: Talk to Fargoth on E, ring flow, journal on J (`cargo dev view "Seyda Neen"`)
- Walk (third person) to Fargoth outside the Census office; face him: E opens the dialogue window;
  WASD and mouse look stop while it is open. Ask "ring", answer yes: no ring, disposition drops.
- Shift+U (dev: gives the ring), ask "ring" again, yes: he thanks you; Goodbye or Esc closes; controls return.
- J: the journal opens at the newest spread with both Fargoth's Ring entries (Day 1 headings); quest list (Finished tab) shows
  "Fargoth's Ring" (finished, new marker); topic index R tab has "ring" with his lines. J or Esc closes it.
- Theme (U3 mwtheme): Mystic Cards font, ini palette: speaker/headings pale gold, text tan, links blue, answers
  dark red, topics tan; black translucent panels in bronze borders. Message boxes (`--open-ui message`): framed.
- uipolish (awaiting owner): no bright outline on any button with the mouse; every button (topics, answers, Goodbye,
  journal tabs/rows/< >, Yes/No, keybinds cells) turns lighter on hover and near-white while pressed; with
  arrows/pad the focused one shows the hover colour. Journal: both pages' text inside the parchment (no text
  on the covers or past the bottom), `< 1/2 >` centred on the book's bottom edge, index panel beside the book.
- dialogue3 (awaiting owner): dialogue window and journal open centred. In the Census office (new game, no chargen):
  the guard lists latest rumors/little advice/little secret, Sellus Gravius Caius Cosades + my trade; Socucius only
  imperial legion once a greeting named it (his only topics in the data). Shot: `--open-ui dialogue`. sellus:
  Sellus Gravius greets "I am Sellus Gravius. My background is not important...", not "Where are your papers?".
- Known gaps: frames are flat CSS borders, not the 8-piece menu textures; no HUD; day headings placeholders;
  seen markers not saved.
- Status: flow confirmed by owner 2026-10-25 (E talk, Shift+U ring, journal updates); journal layout, button
  outlines and hover fixed by uipolish, awaiting owner.

## Dev console (superfargoth `console.md`; ROADMAP 1.13; `.sessions/console.md`)

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
- Status: awaiting owner.
