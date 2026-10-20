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
says what to expect (memory: owner-visual-confirmation). The old `play\` folder is gone. Running it yourself:

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
   particles`, `--off farshadows`, `--off shadowtune` (pre-2026-10-12
   cascades), `--off denoiser`, `--lod-tint`, `--far-tint` (initial state
   of F5), `--terrain-lod`, `--wire`, `--rt`, `--prepass`, `--resolution
   WxH`. Several `--off` parts go comma-separated: `--off farshadows,shadowtune`.
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

### FG-1: Volumetric fog black bands on fast turns (owner report)
- Kind: game location: fog lit by lamps at night (owner shot `Desktop/shadow fog.jpg`: giant-mushroom/cave area,
  21:15, fog). Turn the camera fast left or right.
- Seen by owner 2026-10-20: dark vertical bands appear in the lit fog during fast yaw. Suspects (unverified): the
  froxel volume's temporal reprojection or history rejection on rotation, or the froxel light/shadow lookup at
  history edges (`sf_atmos_bevy` froxel_inject / froxel_integrate). A/B to try: `--off froxels`, `--off fogjitter`.
- Status: known bug: Phase 1 (1.10).


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
- Look for: the field should read as a glowing translucent barrier. Today it
  is an opaque flat white/blue-white wall (`shots\scenes\ghostfence_near.png`,
  unchanged in `shots\nifpop\ghostfence_near.png`).
- Investigated (nifpop): the NIF material maps faithfully (`tx_gg_fence_01/02`,
  non-additive blend, alpha 0.5, emissive 0.3); three stacked 0.5 planes give
  87.5 % coverage of a pale texture under an unclamped sun. Candidates, not
  yet changed (weather.md "Status"): vertex-color alpha replacing material
  alpha in `AmbientDiffuse` mode, NIF light clamping, env-map specular on
  blended planes. Owner: say whether the original reads more translucent or
  more glowing; that picks the fix.
- A/B: `[`/`]` hour (does it glow at night?), `--off envmap`.
- Status: awaiting owner (next pass; small leftover).
- Source: farfield-exterior / weather-spec / nifpop.

## DLSS (superfargoth `sky-lighting.md`; `.sessions/dlss.md`)

Screenshots: `D:\Gamedev\Bevyrrowind\shots\dlss\`.

### TA-1: TAA as the base temporal resolve (ROADMAP 1.3)
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --hour 11 --weather foggy --dlss taa`, fly
  around; then `--weather ashstorm` far away (WE-2 grain) and Balmora at a grazing sun (SH-12 edge shimmer).
- Look for: fog, far bad weather and soft shadow edges smooth instead of grainy; no ghost trails behind moving
  actors, rain or the ship's sail (translucents write no motion vectors yet, RO-6); not blurrier than DLSS Q.
- A/B: F4 cycles off (MSAA) / taa / DLSS modes. Agent shots: `shots/p1-aa-off.png`, `shots/p1-aa-taa.png`.
  Cost (1440p native, Balmora): TAA 15.4 ms vs MSAA 18.5 (TAA pass 0.39 ms; no MSAA saves more).
- Status: awaiting owner (phase1, 2026-10-19). Default now: DLSS auto where available, else TAA; Low: MSAA.

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

## Sky (superfargoth `sky-lighting.md`)

### SK-1: Sunset and sunrise
- Kind: game location. Run: `.\bevyrrowind scene sky-sunset` (west, 19:30)
  and `.\bevyrrowind scene sky-dawn` (east, 6:00); T for a time-lapse.
- Look for: warm low sun, smooth gradient (no banding), the town not black,
  no exposure jump as the sun crosses the horizon.
- Known: the ray-traced path darkens and reddens the sky at low sun
  (open question 1); judge raster.
- Status: awaiting owner (next pass; small leftover).
- Source: sky-lighting / dlss.

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
viewer, F10 toggles them and C cycles the nine genera live. Hours: `[` `]` or `--hour`.
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
- Status: awaiting owner (prompt on doors: Phase 2).

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
- Status: awaiting owner (next pass; small leftover).
- Source: doorfix.

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
the streamed view: J mover gizmos, K foot placement on/off. `--off walkers` turns it all off
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

### PA-4: GPU-drawn particles look as before (ROADMAP 1.2, particles1)
- Kind: game location. Run: `.\bevyrrowind scene particles-census-fire` (fireplace), then
  `.\bevyrrowind scene hi-balmora-night` (torch flames on walkers, braziers) and `.\bevyrrowind character atronach_flame --night`.
- Changed: quads are placed and turned to the camera in the vertex shader from one shared particle buffer
  (`mw_assets` particles.rs, `shaders/particles.wesl`) instead of a mesh rewritten per emitter per frame.
- Look for: flames, sparks and smoke as before (size, colour, additive glow, soft blended smoke, spin); quads
  face the camera from any angle; torch flames still trail a moving hand; no flicker, no stray quads at the
  fire's base, none vanishing near the screen edge; fog dims far fires as before.
- A/B: the previous exe (none at runtime; `--off particles` removes them).
- Status: awaiting owner. Source: particles1.

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

### LT-1: Night exposure floor (`scene lt-night-floor`)
- Kind: game location. Run: `cargo dev scene lt-night-floor` (21:30, clear, sun -13.5 deg).
- Look for: the plaster around the door lantern (left) is bright but textured, not a white patch; lit pools under
  the lamps; facades, stairs and the far houses between the pools dark but visible (night ambient), not pure black;
  the night does not look grey or washed out. A/B: `--exposure-floor=-10 --night-ambient 0.03` (the old night).
- Measured (1280x720, mean sRGB luma / pixels > 235 / pixels < 6): old night EV100 1.62: 72.1 / 1.63 % / 28.7 %;
  floor 3.5 + ambient 0.4: 38.6 / 0.26 % / 34.9 %; floor 3.5 + ambient 1.0 (default): 38.4 / 0.26 % / 26.9 %.
  With `--portals` (viewer auto exposure, curve clamped to the floor): 40.1 / 0.22 % / 37.7 % (ambient 0.4).
- Tune: `--exposure-floor <EV100>` (higher = darker night, dimmer lamps), `--night-ambient <nits>`.
- Status: rejected by owner 2026-10-19 (round 5, seen in scene lamps-handoff at 22:00, not lt-night-floor): night is far too dark, you can't see anything. Phase 1 (exposure floor / night ambient retune).

### LT-2: Dark interior keeps its lights (`scene lt-cave`)
- Kind: game location. Run: `cargo dev scene lt-cave` (Addamasartus, mine tunnel).
- Look for: rock texture readable in the torch pools, the far tunnel dark but not black; the wooden beam and post
  shadow the rock (point-light shadows). A/B `--portals`: auto exposure meters this cave at the same level with or
  without the floor (luma 50.8 vs 50.4): the floor never darkens it.
- Status: partial, owner 2026-10-19 (round 9, scene lt-cave): lighting OK (rock readable, far tunnel dark not black); beam/post shadows only when very close, and very grainy (SH-13).

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
- Known (P8): no powered crumple yet (the clip cuts to limp); the character root stays where it died (a corpse
  dragged far can be culled early); corpses don't persist across cell unloads; creature masses are volume
  estimates (a guar is 1 t: too heavy to drag).
- Agent run 2026-10-18 (p9fin, release, `scene wk-balmora`, no keyboard):
  `$env:BEVYRROWIND_AUTO="kill_nearest@3,shot:shots/p9/kill.png@4.5,exit@8"`. Log: `ragdoll: killed ...
  (meshes/base_anim.nif)`, 1 spawned, 0 failed; joint stretch peak 1.4–2.1 cm (over the 1 cm target: open, P8b).
  Sanity shot `shots/p9/kill.png`. Not on the stairs (wk-balmora's camera); creatures not run.
- **Lead (owner 2026-10-19, round 11, with `G` collider view):** the ragdoll's physics capsules behave normally when hit; it's the skinned mesh that doesn't follow them. So the solver is fine and the bug is the pose write-back (bodies → bones: bone/body offset, local vs world frame, or the floating-origin offset), which would explain sinking, floating, spinning and the odd joint angles.
- Status: rejected by owner 2026-10-19 (round 10): ragdolls very bad: most bodies sink into the ground, float and turn around; joints look unconstrained (unnatural angles); they don't explode. Drag doesn't match the expected Oblivion-style hold/drag UX (later). Phase 3.

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
- Status: awaiting owner (next pass; small leftover).
- Source: physics-p2.

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

### AU-1: Weather sound (`view "Seyda Neen" --weather rain`, then C to step to thunderstorm)

Status: **confirmed outdoors 2026-10-19; indoor fix awaiting owner** (phase1): an interior (no sky in its cell flags) ducks the weather bus to silence over 0.5 s and draws no region ambience; log `audio: listener indoors/outdoors`. Listen: `scene door-balmora-guild` with `--weather rain`, walk in (rain gone), out (rain back); also the Census Office and a cave. Was: weather/region ambience kept playing indoors (round 1 weather pass; M6 WE3 audio; agents cannot hear, the log lines `audio: loop start/stop`,
`audio: one-shot` were checked headless). Look/listen for: rain loop fades in over ~2 s and out when the weather
changes (C key), the thunderstorm adds its loop, thunder follows each flash after distance / 343 s, region ambience
(birds, wind) draws now and then on a clear day. Levels sane against each other; mp3 and wav both play.



### LM-1: Leaves as alpha-tested cutouts (`view "Seyda Neen" --hour 11 --weather foggy`, then `--weather clear`; forest: fly west into the Bitter Coast)

Status: **partial (owner 2026-10-19, round 2, release exe with the v4 bake): some trees' foliage casts shadows, other trees' doesn't**, so not a global regression (the earlier 'none anywhere' was likely the stale v3 bake). Open: which materials stay blended (load-time promotion path?). Phase 1. Earlier: **rejected 2026-10-19: no foliage shadows anywhere** (owner, round 2, clear sky; fog and crisp edges OK in round 1; leafmask). Agent shots: foggy Seyda Neen (leaves fog like the trunks), clear
Cause found (leafshadow): stale bake, leaves cooked under importer v3 before the cutout code existed, so still Blend -> fog swap casts no shadows. Fix (uncommitted): NifImporter::VERSION 4 + re-bake. Recheck with the v4 exe: dappled leaf shadows under trees at 11:00 clear.
Recheck 2026-10-19 (owner, v4 release exe after re-bake): STILL no foliage shadows at all. Stale bake was not (or not the only) cause. Untested leads: most MW foliage is plain blend without alpha test (Cutout::Candidate), promoted at load only if the texture alpha reads as holes: check that the load-time check runs (compressed DDS alpha, texture not loaded yet when the material is built), else leaves stay Blend and the fog swap drops their shadows.
Seyda Neen (tree shadow on the island and the water). A/B: `--off leafmask` (leaves drawn blended as before: no depth,
no shadows, no motion vectors). Look for: crisp-edged leaf cards fogged exactly like trunks and terrain at every distance
(no leaves showing through fog); dappled leaf shadows on the ground under trees at 11:00 clear (with `--off leafmask`:
trunk and branch shadows only); no smearing behind foliage with TAA/DLSS (F4); distant trees not thinning to bare twigs
(cutoff 0.5, no coverage-preserving mips yet: if they thin, note at what distance). Glass, potions, Ghostfence, glows,
smoke and ghosts must still look translucent (if one became a hard-edged cutout, note the object).

### RO-A: Translucents cast no shadows; distant lights and rain fade into fog (render-order RO-1/2/3, rofix1)
Status: **step 1 confirmed by owner 2026-10-19** (no flame/glow shadow blobs in Balmora); steps 2-4 not checked. Agent check: dev build renders, shaders compile, rain at night visible (`view "Seyda Neen"`, hour 22, rain); fading not judged.
1. `view "Balmora" --hour 11 --weather clear`: torch flames, glow cards and glass cast no shadow on sunlit walls/ground; trees (cutout leaves) and solid props still do. Compare `--off transfog` (old: flames/glows cast near-solid shadows).
2. `view "Seyda Neen" --hour 22 --weather foggy`, fly ~1-3 km out: the far town's light points dim with distance and vanish into the fog instead of staying full-bright.
3. `view "Seyda Neen" --hour 15 --weather rain` (then `foggy` + rain via C): far rain streaks fade into the fog colour; near streaks unchanged.
4. Ghostfence (`view "Ghostgate" --hour 12`): the field casts no shadow.

### WE-5: Ash/blight storm ignores terrain and objects, wrong direction (owner report)
- Kind: game location. Run: `.\bevyrrowind scene summit-vivec` (or any ashstorm/blight weather near Red Mountain).
- Seen by owner 2026-10-19 (round 3): the blight storm (maybe other weather effects too) is not occluded by terrain or
  objects: it draws over them. Also the ash storm should blow from Red Mountain (Morrowind storms have a source;
  we2b already derives the summit storm source from LAND), which it doesn't seem to.
- Also (owner 2026-10-19, round 6): in some weathers the clouds extend very high into the sky (deck base/thickness, WE4b local heights?).
- Status: known bug: occlusion in Phase 1 (render correctness), storm direction in Phase 4.


### PC-1: Precombined near cells look the same as separate objects (ROADMAP 1.9, superfargoth precombine.md)
- Kind: game location. Run: `.\bevyrrowind scene balmora`, then walk the streets at eye height; A/B with `--off precombine`
  (one entity per object, the old way). Also `scene seyda-neen` (docks, small clutter).
- Look for: nothing missing or doubled; pots, crates, baskets and other small clutter appear and disappear at about
  the same distances as before; no popping when buildings switch detail; shadows unchanged; no hole or double image
  where the far field takes over (~400 m). A script-disabled static (e.g. a quest rock) must still disappear.
- Status: **awaiting owner** (2026-10-20, precombine3; not seen on screen by the agent yet).
