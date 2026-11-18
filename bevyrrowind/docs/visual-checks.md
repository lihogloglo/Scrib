# Visual checks

The owner's eyes judge rendering work (CONTRIBUTING.md "Tests and visual checks"). Agents take a few
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
   | F3 | perf panel: off, compact, full (frame, main/render/GPU, bound by; CW-17) |
   | Shift+F3 | terrain far shadows on/off (shadows past the cascades) |
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

---

## Open checks index

One line per open ID: `ID | status | file | title`. Entries live in [`visual-checks/`](visual-checks/); closed ones in [`archive/visual-checks-closed.md`](archive/visual-checks-closed.md). Add a new entry to the right system file and a line here.

### Awaiting owner (56)
WG-V | awaiting owner | [worldgen](visual-checks/worldgen.md) | 3D worldgen viewer `worldgen-view`, S2 arms A0-A3 (B0-B3 next) (2026-11-18)
WG-S2 | partial (owner 2026-11-18: parallel "motherboard" channels in A1/A2; wants 3D) | [worldgen](visual-checks/worldgen.md) | 10x Vvardenfell erosion look test: hillshade sheets A0-A3 in `target/worldgen/s2/` (images, no game run) (2026-11-18)

EX-1 | awaiting owner (fix E1+E1b built 2026-11-17: tune `sky.moon.sky_scatter`, `sky.night.glow_ratio` live; floor 4.5 default) | [weather-sky](visual-checks/weather-sky.md) | Night darkness, three launch-flag arms vs today (`view "Balmora" --day 173 --hour 3.5 --weather clear`; `--night-ambient 2`, `--exposure-floor 4.5`, both halfway)
WA-10 | partial (2026-11-17 recheck: rings + no rain underwater pass; Bitter Coast must stay green, only visibility was wrong; storm sea still meek, no crest foam/spray (Sea of Thieves session); U4 waterline open) | [terrain-water](visual-checks/terrain-water.md) | Storm sea + underwater + rain rings (W-SEASTATE, U3, R1): `scene wa-seyda-noon --weather thunderstorm` (2026-11-17)
WA-9 | failed (2026-11-17: no surf at Seyda Neen, foam painted + wrong scale, no wet sand) | [terrain-water](visual-checks/terrain-water.md) | Surf: breaker foam + breaking shape (SF-1, SF-2): `scene wa-seyda-noon --water-wind 14` / `30`; A/B `--off surffoam,surfskew` (2026-11-17)
WA-8 | awaiting owner | [terrain-water](visual-checks/terrain-water.md) | Shore foam follows the wave energy, none on calm inland water (W-FOAM-GATE): `view "Balmora"` Odai, `scene wa-seyda-noon` beach (2026-11-17)
WA-7 | confirmed with follow-up (owner 2026-11-17: no regression, no visible change; F3 GPU numbers missing) | [terrain-water](visual-checks/terrain-water.md) | Water chain gated by visible water (CW-10a): release, `view "Balmora"` street then the river bank at noon, `set water.gate.visible on`, F3 GPU water passes before/after; turning toward/away from water: no pop, reflections valid (2026-11-17)
WA-6 | partial (2026-11-13: storm sea far too tame; moving white shore band in rivers/lakes, still seen 2026-11-15) | [terrain-water](visual-checks/terrain-water.md) | The sea follows the weather (W-WIND): `scene wa-seyda-noon`, BRP weather thunderstorm and back to clear (`time_scale` 120); smooth roughening and calming, no pop or re-roll; A/B `--off seawind` (2026-11-13)
WE-15 | partial (2026-11-13: clouds still goofy; bigger shape pass later) | [weather-sky](visual-checks/weather-sky.md) | Storm cloud shape (S3): `scene we3-ridge` 14:00, BRP thunderstorm over Ascadian Isles, look E/SE; domed tapering cells, anvil shelves, ragged base, sloped edge, not an extruded footprint; A/B `--off cbshape` (2026-11-13)
WA-5 | partial (2026-11-15: shore waves visible but faint bulges, crests do not bend; open sea small at 14 m/s; contact foam bad; FPS very low, unmeasured) | [terrain-water](visual-checks/terrain-water.md) | Gerstner shore waves over the real bed slope: `scene wa-seyda-noon`, `--water-wind 14`, `30`, `scene wa-bittercoast`; A/B `--off shoredepth` / `--off shorewaves`
WE-13 | confirmed with follow-up (2026-11-13: smooth rain fade, curtain visible, shimmer behind it much better, some left) | [weather-sky](visual-checks/weather-sky.md) | Local storm seen from outside: storm cell under its clouds, narrow ragged border, no ground-to-sky wall (CW-1 + S2; A/B `--off stormshape`)
SH-20 | partial (2026-10-30: smooth now; which lamps cast looks random, player and torches cast none; fixes built 2026-11-01: SH-21); owner 2026-11-15: shadow code changed since, may be stale, re-check | [shadows](visual-checks/shadows.md) | Character lamp shadows update per frame within 16 m (`--off movershadows` A/B)
SH-21 | partial (2026-11-01: torches cast; flicker, cut faces, choppy: SH-20b in progress); owner 2026-11-15: shadow code changed since, may be stale, re-check | [shadows](visual-checks/shadows.md) | Lamp and torch shadows on the player and torch-bearers; shadowed-lamp pick by view visibility (2026-11-01)
SH-22 | known bug; owner 2026-11-15: shadow code changed since, may be stale, re-check | [shadows](visual-checks/shadows.md) | Exterior fps halves after leaving the Census office, not the GI (2026-11-02)
SH-23 | awaiting owner | [shadows](visual-checks/shadows.md) | Sun shadow early-out: Balmora noon, dusk, night, leaf light-through and contact shadows identical before/after (2026-11-09)
SH-24 | awaiting owner | [shadows](visual-checks/shadows.md) | Balmora at night: statics still cast into cached lamp shadows with GPU-culled statics, A/B `--off cpuculltag` (perf-push P3, 2026-11-12)
SH-25 | confirmed | [shadows](visual-checks/shadows.md) | Balmora street at night (`scene lamp-turn`): lamp shadows already there after a 180° camera turn and when walking toward lamps, no fade-in; note NPC shadow flicker among lamps (lamp-select, 2026-11-15)
SH-26 | confirmed 2026-11-16 | [shadows](visual-checks/shadows.md) | Balmora street 16:00 (`scene cascade-hold`): far sun cascades held between redraws; no popping, swimming or edge gaps when walking and turning; far NPC shadows may step (orch-11-15c, 2026-11-15)
SH-27 | bug, logged | [shadows](visual-checks/shadows.md) | Night, Balmora: lamp shadows on some buildings appear only within ~10 m and fade in (budget of 8 maps on High, likely; owner 2026-11-16)
SH-28 | confirmed 2026-11-16 (owner: "seems good", noon; dusk fade not separately checked; opt-in CW-3a, perf not yet measured) | [shadows](visual-checks/shadows.md) | Lamp shadow day gate: noon lamps under awnings, dusk fade-in, night unchanged (`BEVYRROWIND_LAMP_DAY_GATE=1`)
SH-29 | confirmed 2026-11-16 (owner: "seems all good"; opt-in CW-3b, perf not yet measured) | [shadows](visual-checks/shadows.md) | Point shadow face culling: no pop-in when spinning near a lamp, no seam gaps (`BEVYRROWIND_POINT_FACE_CULL=1`)
SH-30 | confirmed 2026-11-16 (owner: "everything looks right": rain 16:30 -> dusk fade -> night moon; opt-in CW-8, perf not yet measured) | [shadows](visual-checks/shadows.md) | Sun/moon shadow gate: dusk-rain fade, ashstorm, overcast noon look, dim moon (`BEVYRROWIND_SUN_SHADOW_GATE=1`)
SH-31 | confirmed with follow-up (owner 2026-11-17: Census + Balmora, no flicker, same on/off, face views drop; Balmora crash fixed same day); open: lamp shadows fade in too close | [shadows](visual-checks/shadows.md) | Lamp shadow cache: Census Office and Balmora 22:00, `set shadow.lamp.cache.enabled on`: same shadows as off when settled, crisp NPC shadows near lamps, no flicker or seam gaps, lamp face views near 0 when still
GI-1 | partial (2026-11-02: runs clean, but `flip gi` barely changes the room) | [gi](visual-checks/gi.md) | Dynamic probe GI in the Census office (`--off gi` A/B)
GI-2 | partial (2026-11-02: no artifacts seen; effect too weak to judge leaks) | [gi](visual-checks/gi.md) | DDGI visibility-weighted probe sampling: no leaks through the office floor/walls, bounce in corners (`--off gi` A/B)
GI-lampbody | awaiting owner | [gi](visual-checks/gi.md) | Lamp bodies don't light twice at GI hits (less hot bounce right around lamps, room fill kept) and interior ambient floor 0.1 (corners not pitch black) (`flip gi` A/B)
PH-16 | confirmed for console kill (2026-11-05); hit deaths unseen until C6 | [physics](visual-checks/physics.md) | A killed human NPC falls and stays dead under Delirium (console `kill`); beasts and creatures keep the clip + limp ragdoll
LT-reach | rejected 2026-11-05 (default back to 4 x) | [lights](visual-checks/lights.md) | Lamp reach follows shadow state: no light through walls, no ring or pop at pool edges, GI visible next to lamps (`flip gi` A/B; old reach `--range-unshadowed 4`)
FF-2 | rejected (2026-10-30: all green to the horizon, L2/L3 never shown) | [rendering](visual-checks/rendering.md) | Far-field L1/L2/L3 F5 debug tint on proxies and impostors
DL-5 | awaiting owner | [rendering](visual-checks/rendering.md) | Interior `--rt --off denoiser` runs without `sky_motion` validation crash
RO-16 | 2026-11-06: ground better (fix 1, N1), moonlight OK (N3); night too lit (closer to OpenMW), Secunda not visible; fix 2, N2 unconfirmed; noon not checked | [rendering](visual-checks/rendering.md) | Terrain "veil" at grazing angles, blue-grey at night: name the console switch that lifts it (`scene ro16-terrain-veil`)
RO-17 | parked (owner 2026-11-06) | [rendering](visual-checks/rendering.md) | Tree leaves reflect as one flat panel in the light colour, leaf texture lost (2026-11-05)
TX-lab | confirmed with follow-up (2026-11-15: N and M work well; lab lamp too bright, only visible at night) | [rendering](visual-checks/rendering.md) | Texture lab: swatches + cooked statics under low sun and orbiting lamps, N / M flip normal / ORM maps live (also `flip normalmaps` / `flip orm` anywhere); info panel says per swatch which maps exist, are bound, their format and tangents (`scene level-texture-lab`)
TX-5 | confirmed with follow-up (2026-11-15: depth works; POM swims on some rocks at grazing angles, fix later) | [rendering](visual-checks/rendering.md) | Parallax occlusion on statics from `_nh` height: depth in cobbles / bricks at grazing angles, no swimming, no smearing at silhouettes; L in the lab / `flip parallax` A/B (`scene level-texture-lab --lab-textures ...`, Balmora; re-bake with packs)
TX-6 | confirmed | [rendering](visual-checks/rendering.md) | Balmora 08:00 and night: statics shade the same with `--off flatnormal` vs default (tangents everywhere + shared flat normal map, fix B2, 2026-11-15)
SS-matclass | awaiting owner | [rendering](visual-checks/rendering.md) | Static material classes: every static looks as with `--off matclass` (textures, cutout edges, two-sided leaves, glow) in Balmora and Seyda Neen
SC-scene | owner: matches sc-noscene (2026-11-11); floating foliage + shadow flicker are not from --scene; 883 batch-full warnings; perf: ~2-3 ms SLOWER on (batch overflow; re-measured 2026-11-17 on the walk-once base: walk parity, standing -0.22 ms, ledger statics); on by default since owner play 2026-11-17 (PP-5) | [rendering](visual-checks/rendering.md) | Static scene (`--scene`): every static present, no double draw, nothing jumps after a 256 m walk; doors/lights stay entities
SC-lamps | awaiting owner (static scene on by default since 2026-11-17: lamp shadows of statics at night not yet checked on their own) | [rendering](visual-checks/rendering.md) | Lamp shadows of `--scene` statics at night: present, steady, recapture after a new cell (scene sc-scene-night)
RO-B | confirmed by owner 2026-11-16 (fix smoke-2: volume lighting, sum-of-lights saturation, exposure-relative fully lit) | [rendering](visual-checks/rendering.md) | Lit smoke shading + dynamic plume Aabb sort over sky and objects
PH-11 | partial (2026-10-30: still a walkable hole in the office floor, PH-12) | [physics](visual-checks/physics.md) | Physics fixes: no falls on load, still clutter, steady carry
WE-5 | confirmed with follow-up (2026-10-30: shelter too generous, cap to a few metres; clouds maybe high) | [weather-sky](visual-checks/weather-sky.md) | Ash/blight storm ignores terrain and objects, wrong ...
AL-2 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Walk loop, turns
AL-3 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Ramps and stairs
AL-5 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Props push
AL-7 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Idles, reactions, other rigs
AL-8 | confirmed with follow-up (2026-10-27: camera solved; hands on the lip + clipping: B2b next) | [anim-lab](visual-checks/anim-lab.md) | Waist mantle, player (climb clip on the body)
AN-7 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Capsule steady under a walker
AN-3 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Footsteps on the feet
AN-2 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Walkers stop within a step
AN-8 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Creature capsules fit the creature
AN-9 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Planted foot holds through the heel roll
AN-10, AN-13, AN-14 | retired (2026-10-30: superseded by delirium) | [anim-lab](visual-checks/anim-lab.md) | Get-ups
DE-1 | confirmed with follow-up (2026-10-27: floor slide fixed, push in F11 only; a crouched landing sinks ~1.5 s, later) | [anim-lab](visual-checks/anim-lab.md) | Delirium takeover: click-to-push, catch steps, blend-out to the idle, fall + roll-over + get-up
AL-TRUNK | confirmed in the browser 2026-11-12 (arms clear the chest, trunk upright); in-game mantle/combat clips not yet looked at | [anim-lab](visual-checks/anim-lab.md) | Matched retarget pose: retargeted clips keep Morrowind's trunk curvature (upright vs before), mantle/combat no worse
AU-3 | partial (2026-10-30: greets only very close; NPCs don't stop or turn) | [audio](visual-checks/audio.md) | NPC greetings, idle chatter, lip-sync level
AU-5 | rejected (2026-10-30: no swing, no sound, prompt only) | [audio](visual-checks/audio.md) | Swinging-door sounds
IN-1 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Keymap controls, rebind, gamepad, sneak hold vs toggle
PM-1 | confirmed with follow-up (2026-10-30: mouse sliders, mute icon, Main menu button) | [player-ui](visual-checks/player-ui.md) | Escape menu and options (volume live and saved)
SV-2 | confirmed with follow-up (2026-10-30: thumbnails missing) | [player-ui](visual-checks/player-ui.md) | Save/load screens from the pause menu (save, overwrite, load, delete)
MM-1 | confirmed with follow-up (2026-11-11: flow works; look crude, text over grey boxes, not Morrowind/OpenMW) | [player-ui](visual-checks/player-ui.md) | Release start: intro logos, main menu with no world behind it, New game
OP-1 | confirmed with follow-up (2026-11-11: works; a plain list, needs real widget art); S2b1 description pane awaiting owner | [player-ui](visual-checks/player-ui.md) | Options rows: mouse-drag sliders, checkboxes (S2a, PM-1 follow-up)
LK-1 | owner: issues (submenus overlap) | [player-ui](visual-checks/player-ui.md) | Look pass L1: OpenMW frames on windows, boxes, framed buttons
LK-2 | owner: issues (Continue style) | [player-ui](visual-checks/player-ui.md) | Look pass L2: title menu as OpenMW image buttons, version text
FG-10 | known bug | [weather-sky](visual-checks/weather-sky.md) | No visible god rays at sunrise in fog (2026-11-02)
AI-1 | awaiting owner | [ai](visual-checks/ai.md) | AI debug overlay (`ai.select`, `ai.trace`, plan gizmos) on a synthetic trace, example `ai_overlay` (2026-11-09)
NV-1 | confirmed by owner 2026-11-18 (Balmora: `nav.stats` good, `nav.draw tiles` streams and follows terrain after detail-mesh + terrain-origin fixes) | [ai](visual-checks/ai.md) | Baked navmesh tiles over Seyda Neen (`nav.draw tiles`, `nav.stats`), slice NV4c (2026-11-17)
NV-2 | confirmed by owner 2026-11-18 (Balmora) | [ai](visual-checks/ai.md) | NPCs walk smoothly on the fixed tick at low and high fps; wake, knockdown, ball hits without pops, slice NV7a (2026-11-17)
AI-2 | confirmed by owner 2026-11-18 (Balmora: no one frozen, follow drawn paths, log OK) | [ai](visual-checks/ai.md) | Balmora wanderers plan HTN wander/idle and walk navmesh paths, pathgrid only as fallback, slice MA3 (2026-11-18)
RG-1 | awaiting owner | [streaming](visual-checks/streaming.md) | Nothing pops in after a door fade or the loading screen lifts (reveal gate W1; `reveal:` lines)
DR-5 | confirmed with follow-up (owner 2026-11-17: door in/out fine; F3 drop not read) | [streaming](visual-checks/streaming.md) | Interiors park the exterior (`set stream.interior.disable_exterior on`): door in and out unchanged, F3 entities/main ms drop indoors
NS-3 | confirmed with follow-up (owner 2026-11-17: moonrise, per-moon glows; open: glow behind the disk, black inner circle on Masser) | [weather-sky](visual-checks/weather-sky.md) | Masser and Secunda
SW-8 | confirmed with follow-up (owner 2026-11-17: trail fixed after the viewport fix; crouch/NPC/1P not reported; small creatures leave no trace) | [weather-sky](visual-checks/weather-sky.md) | Legs and bodies carve deep snow (depth view from below): Skaal Village `surface set snow 1`, wade, crouch, NPC passing, first person

### Partial (9)

SH-15 | partial | [shadows](visual-checks/shadows.md) | 8 cached lamp shadows
DR-1 | partial | [streaming](visual-checks/streaming.md) | Through a door into an interior
DR-2 | partial | [streaming](visual-checks/streaming.md) | Back out
LT-2 | partial | [lights](visual-checks/lights.md) | Dark interior keeps its lights
PH-7 | partial | [physics](visual-checks/physics.md) | Item body shapes
PH-9 | partial | [physics](visual-checks/physics.md) | knockdown and get-up
WA-1 | partial (2026-11-12: still flat at the shore (normals scaled with displacement), bad foam, no rain or wake ripples; Phase 4) | [terrain-water](visual-checks/terrain-water.md) | Open sea, W2 FFT ocean
IV-1 | partial | [player-ui](visual-checks/player-ui.md) | Take items and Fargoth's ring; inventory layout, hover, drop, paper doll missing
IV-2 | confirmed with follow-up (2026-10-30: icons only on hover, E should close) | [player-ui](visual-checks/player-ui.md) | Container window: the ring barrel outside the Census office (ring), move both ways, Take all closes

### Confirmed with follow-up (8)

DL-3 | confirmed with follow-up | [rendering](visual-checks/rendering.md) | Ray-traced Census Office with Ray Reconstruction
WA-2 | confirmed with follow-up | [terrain-water](visual-checks/terrain-water.md) | Shore and region optics, W3
RO-A | confirmed with follow-up | [rendering](visual-checks/rendering.md) | Translucents cast no shadows; distant lights and rai...
DJ-1 | confirmed with follow-up | [player-ui](visual-checks/player-ui.md) | Talk to Fargoth, Sellus greeting; resizable window wished
DJ-2 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Dialogue action buttons, Persuasion submenu modal (written 2026-11-08, untested)
PUI-SV3 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Trade screen over the dialogue: Barter with Arrille, stage, haggle, accept, cancel (TR2)
PUI-SV4 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Training list over the dialogue: top 3 skills, prices, refusals, buy +1 and 2 h pass, Back (A3)
SV-1 | confirmed with follow-up | [player-ui](visual-checks/player-ui.md) | Quicksave / quickload; load sometimes spawns below the floor

### Known bug (29)
PL-9 | known bug (owner 2026-11-15, not diagnosed) | [player-ui](visual-checks/player-ui.md) | Player run looks sideways: the character runs on a diagonal to its travel
RO-23 | confirmed by owner 2026-11-16 (fix smoke-2: self_lit in exposed units) | [rendering](visual-checks/rendering.md) | Carried-torch handles unnaturally bright at night (fully lit in nits at EV 7, night floor EV 3.5 = ~11x)

SH-13 | known bug; owner 2026-11-15: shadow code changed since, may be stale, re-check | [shadows](visual-checks/shadows.md) | Point-light (lamp) shadows
SH-14 | known bug; owner 2026-11-15: shadow code changed since, may be stale, re-check | [shadows](visual-checks/shadows.md) | Contact-shadow bands at object edges
GF-1 | known bug (owner 2026-11-15, repeated: fence panels pop with distance; no far copy, B5) | [rendering](visual-checks/rendering.md) | Ghostfence appears complete, never late
DL-4 | known bug | [rendering](visual-checks/rendering.md) | Rug-edge black bands under RT
RO-18 | known bug (owner 2026-11-15: not reproduced; owner retries flying high) | [rendering](visual-checks/rendering.md) | Grass and foliage floating in the sky, swapping as a batch every ~200 m (2026-11-11)
RO-19 | known bug; owner 2026-11-15: shadow code changed since, may be stale, re-check | [rendering](visual-checks/rendering.md) | Very short shadow flickers in Balmora (2026-11-11)
RO-22 | awaiting owner | [rendering](visual-checks/rendering.md) | Near/far handoff band: no gaps or doubles with GPU-culled statics, A/B `--off cpuculltag` (perf-push P3, 2026-11-12)
PH-12 | known bug | [physics](visual-checks/physics.md) | Walkable hole in the Census Office floor (again 2026-10-30)
PH-13 | known bug | [physics](visual-checks/physics.md) | Player and NPCs drop onto the ground when a cell loads (2026-10-30)
CB-3b | partial 2026-11-12 (held guard loop jumps; stamina refills too fast to test the break; regenerate clips) | [combat](visual-checks/combat.md) | Guard (hold right mouse / L1) in third and first person: front hits guarded (no reaction, stamina dip), back hits not, guard break at 0 stamina; body follows the view in first person (scene level-sparring, 2026-11-10)
CB-5 | awaiting owner | [combat](visual-checks/combat.md) | First-person melee: swings where you look, soft aim bends onto the dummy, no lock in first person (scene level-arena, 2026-11-10)
CB-5b | awaiting owner | [combat](visual-checks/combat.md) | First-person attack look limits: the view stops ~45 deg off the body mid-swing, released at recovery; view model on and off (scene level-arena, 2026-11-10)
PL-6 | awaiting owner (2026-10-30: hard-landing lock removed until fall damage) | [player-ui](visual-checks/player-ui.md) | Recovery delay after jumping down from a high box (2026-10-30)
PL-7 | partial (owner 2026-11-11 re-test: feet on the ground OK; stop pops a leg back + hip rise, arms point back on slopes, head too tilted; locomotion rethink) | [player-ui](visual-checks/player-ui.md) | Player foot placement on slopes and stairs, K A/B (P-IK1, 2026-11-11)
WALK-0 | awaiting owner | [player-ui](visual-checks/player-ui.md) | A/B ladder on the player's walk: speed_match, stride_warp, foot_placement, lean, side_lanes off one at a time (3.4c A0)
FP-1 | awaiting owner | [player-ui](visual-checks/player-ui.md) | First person shows body, arms and weapon with the head hidden (2026-11-10)
FP-1b | awaiting owner | [player-ui](visual-checks/player-ui.md) | First person: the player's shadow keeps its head under sun, lamps and a torch, raster and RT (2026-11-10)
FP-2 | awaiting owner | [player-ui](visual-checks/player-ui.md) | First-person view model (`flip viewmodel`): weapon and arms at the body FOV, never in walls, lit like the world, no DLSS/TAA smear; shader-compile log line (2026-11-10)
PP-1 | confirmed, follow-up | [rendering](visual-checks/rendering.md) | F3 perf panel (CW-17, release build): numbers refresh, bound-by matches the feel, flipping a CW switch moves them (2026-11-16)
PP-2 | confirmed, follow-up | [rendering](visual-checks/rendering.md) | Graphics presets live: Shift+F4 cycles extremely_low..ultra in Balmora night and day, F3 follows, custom after a hand change (T5b, 2026-11-16)
PP-3 | confirmed | [rendering](visual-checks/rendering.md) | F3 perf panel graphs (CW-17b, release, Balmora 22:00): frame-time graph coloured by bound with 8.3/11 ms lines, thread bars, full: pass/stage bars + sparklines (2026-11-16)
PP-4 | blocked (menu bugs, 2.2b) | [rendering](visual-checks/rendering.md) | Graphics page: Esc > Options > Graphics in Balmora 22:00, overall + group rows live behind the menu, Custom, restart badges, saved across a restart (T5d, 2026-11-16)
PP-5 | confirmed 2026-11-17 (owner play: "very smoothly and very fast", 110 fps; every gate on by default, lamp cache off) | [rendering](visual-checks/rendering.md) | CW gates on by default: play with `--tunables tunables/cw-gates-on.toml`, F3 open, flip each gate in the console; verdict per gate (2026-11-17)
RO-13 | known bug | [rendering](visual-checks/rendering.md) | F7 wireframe drawn away from the model (2026-10-30)
WE-6 | known bug | [weather-sky](visual-checks/weather-sky.md) | Rain turns grainy while the camera moves (2026-10-30)
WE-7 | known bug | [weather-sky](visual-checks/weather-sky.md) | Rain curtain stops in front of distant water (2026-10-30)
SW-7 | confirmed with follow-up (owner 2026-11-15: night puffs good, "much better", drift with the wind liked; look rework still open) | [weather-sky](visual-checks/weather-sky.md) | Kick-up puffs (W7): Skaal Village `surface set snow 1` walk/run/sprint: white-blue powder puffs at each foot-down, bigger when running, lit like the snow (shadow, night); Ghostgate `surface set ash 1`: grey-brown puffs drift downwind; `--off kickup`
SW-6 | confirmed with follow-up (owner 2026-11-17: ramp good); open: shin trails, ash depth, SW-6b drip lines | [weather-sky](visual-checks/weather-sky.md) | Knee-deep snow + footprints (W6a/W6b): Skaal Village `surface set snow 1`, walk: snow raised 0.45 m near the camera, easing out by ~43 m, feet sunk, prints cut to the ground with lips, no smear with DLSS; live `off trails` / `off cover`
SW-5 | confirmed (owner 2026-11-17: shelter bug gone after the shelter-audit fix); ash not rechecked | [weather-sky](visual-checks/weather-sky.md) | Snow and ash cover: Ghostgate `surface set ash 1`, Skaal Village `surface set snow 1`; flat tops covered, walls/slopes bare, patchy at 0.4, none under roofs or on characters, snow glints; A/B `--off cover`
AU-6 | known bug | [audio](visual-checks/audio.md) | Third-person listener in the character head, not the camera (2026-10-30)
FG-8 | known bug | [weather-sky](visual-checks/weather-sky.md) | Weather fog vanishes at night (2026-10-30)

### Rejected (2)

SH-9 | rejected | [shadows](visual-checks/shadows.md) | Many shadowed lights indoors
LT-3 | rejected | [lights](visual-checks/lights.md) | Point-light shadows, torch-bearers

### Inconclusive (2)

SH-16 | confirmed with follow-up (2026-11-15: GTAO works, barely visible; AO only darkens indirect light, grows with GI) | [shadows](visual-checks/shadows.md) | Shadow reset A1
SH-19 | inconclusive | [shadows](visual-checks/shadows.md) | Shadow reset B2

### Deferred (23)

DL-2 | deferred | [rendering](visual-checks/rendering.md) | Super Resolution at 1440p
SK-2 | deferred | [weather-sky](visual-checks/weather-sky.md) | Night and moon
ST-3 | deferred | [streaming](visual-checks/streaming.md) | Paced model loads arrive in time
ST-4 | confirmed 2026-11-16 (owner: "no popping whatsoever, even when flying fast"; doors fine; opt-in CW-2, perf not yet measured) | [streaming](visual-checks/streaming.md) | Selection gates: no terrain cracks, far-field holes or late cells (`BEVYRROWIND_SELECT_GATE=1`)
SC-2 | deferred | [streaming](visual-checks/streaming.md) | The cutoff fades, never pops
SC-foliage | owner: OK in Balmora (2026-11-11) | [rendering](visual-checks/rendering.md) | Cutout candidates (foliage, fences, grates) drawn from the static table, and no candidate that should fade softly gaining hard edges
SN-2 | deferred | [characters](visual-checks/characters.md) | Guar from its cooked skinned product
SN-3 | deferred | [characters](visual-checks/characters.md) | Dark elf head morphs
CH-1 | deferred | [characters](visual-checks/characters.md) | The M3 lineup
CH-3 | deferred | [characters](visual-checks/characters.md) | Balmora street with NPCs standing in place
CH-4 | deferred | [characters](visual-checks/characters.md) | Census and Excise Office staff
CH-5 | confirmed 2026-11-16 (owner: "everything seems fine, no smear, no issues"; opt-in CW-7, perf not yet measured) | [characters](visual-checks/characters.md) | Skip unchanged skins: poses correct, no DLSS smear on resume (`BEVYRROWIND_SKIN_SKIP_UNCHANGED=1`)
SB-1 | deferred | [character-gear](visual-checks/character-gear.md) | Lineup close-up at elbows and knees, walking
SB-2 | deferred | [character-gear](visual-checks/character-gear.md) | Armored NPC
SB-3 | deferred | [character-gear](visual-checks/character-gear.md) | Beast tails with greaves
SB-5 | deferred | [character-gear](visual-checks/character-gear.md) | Walkers in Balmora
HI-3 | deferred | [character-gear](visual-checks/character-gear.md) | Torch-bearer at night
AP-2 | deferred | [character-gear](visual-checks/character-gear.md) | Flame atronach and ancestor ghost particles
AP-3 | deferred | [character-gear](visual-checks/character-gear.md) | Archers hold bows
PH-10 | deferred | [physics](visual-checks/physics.md) | kinematic hit reactions and tail spring bones
PH-4 | deferred | [physics](visual-checks/physics.md) | Carry, drag and throw
NS-2 | deferred | [weather-sky](visual-checks/weather-sky.md) | Stars fade in at dusk
NS-4 | deferred | [weather-sky](visual-checks/weather-sky.md) | No red streaks at night
WE-10 | fix 1 (history) failed 2026-11-06: shimmer stays, storm not over water; owner 2026-11-12: still a lot of shimmer on everything behind fog and clouds | [weather-sky](visual-checks/weather-sky.md) | Local storm seen from outside: cloud curtain and noisy border (`scene we3-ridge` + BRP region force)
WE-9 | fog fix confirmed 2026-11-06; causes 1 (hour keys, re-seen) and 3 open | [weather-sky](visual-checks/weather-sky.md) | Forced fog and rain seem to go away at night: hour keys wrap to the same day (live force undone), fog unlit by the night ambient floor, rain drops unlit at night
