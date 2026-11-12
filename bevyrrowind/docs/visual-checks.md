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

---

## Open checks index

One line per open ID: `ID | status | file | title`. Entries live in [`visual-checks/`](visual-checks/); closed ones in [`archive/visual-checks-closed.md`](archive/visual-checks-closed.md). Add a new entry to the right system file and a line here.

### Awaiting owner (55)

WE-13 | awaiting owner | [weather-sky](visual-checks/weather-sky.md) | Local storm seen from outside: storm cell under its clouds, narrow ragged border, no ground-to-sky wall or edge shimmer (CW-1; A/B `--off stormshape`, 2026-11-12)
WA-4 | confirmed (2026-11-12: "that's perfect") | [terrain-water](visual-checks/terrain-water.md) | Wavelets at the shore (W-SHORE): ripples glint to the waterline, swell still dies on beaches, no flat strip, no moiré; A/B `--off shorenormals` (2026-11-12)
WA-3 | confirmed (2026-11-12: day and night "very nice", night water no longer too bright; emissive in reflection unsure) | [terrain-water](visual-checks/terrain-water.md) | Water mirrors banks, buildings, ships and lit windows, wobbling with waves; edge fade, no flicker, nothing below the water (W-SSPR, A/B `--off waterssr`, 2026-11-12)
FG-11 | confirmed (2026-11-12: "the fog is glorious") | [weather-sky](visual-checks/weather-sky.md) | Froxel fog smooth over near objects, still and panning; no trails or black bands (FOG-CLAMP, A/B `--off fogclamp`, 2026-11-12)
WE-11 | confirmed (2026-11-12: fog, rain, storm over sea; clouds at sea; wish: visible lightning bolt) | [weather-sky](visual-checks/weather-sky.md) | Weather covers water like land (W-ORDER): fog/rain/storm over the far sea, cloud over the ocean from high, no double fog or shore halo (2026-11-12)
WE-12 | confirmed (2026-11-12: "works perfectly") | [weather-sky](visual-checks/weather-sky.md) | Deck from above/inside: fog split at the clouds' depth (WE12-FIX); `scene fg-bittercoast-ridge`, fly ~100 m over the deck, look down, then into it; A/B `--off cloudfog` (2026-11-12)
FG-7 | confirmed (2026-11-12: blockiness gone) | [weather-sky](visual-checks/weather-sky.md) | Blocky edges on distant land in fog: FOG-BANK fix, A/B `--off fogbank` (2026-11-12)
SH-11 | confirmed (2026-10-30) | [shadows](visual-checks/shadows.md) | Distant flashes while moving (PostUpdate far-shadow ordering + world-unit PCSS)
SH-20 | partial (2026-10-30: smooth now; which lamps cast looks random, player and torches cast none; fixes built 2026-11-01: SH-21) | [shadows](visual-checks/shadows.md) | Character lamp shadows update per frame within 16 m (`--off movershadows` A/B)
SH-21 | partial (2026-11-01: torches cast; flicker, cut faces, choppy: SH-20b in progress) | [shadows](visual-checks/shadows.md) | Lamp and torch shadows on the player and torch-bearers; shadowed-lamp pick by view visibility (2026-11-01)
SH-22 | known bug | [shadows](visual-checks/shadows.md) | Exterior fps halves after leaving the Census office, not the GI (2026-11-02)
SH-23 | awaiting owner | [shadows](visual-checks/shadows.md) | Sun shadow early-out: Balmora noon, dusk, night, leaf light-through and contact shadows identical before/after (2026-11-09)
SH-24 | awaiting owner | [shadows](visual-checks/shadows.md) | Balmora at night: statics still cast into cached lamp shadows with GPU-culled statics, A/B `--off cpuculltag` (perf-push P3, 2026-11-12)
GI-1 | partial (2026-11-02: runs clean, but `flip gi` barely changes the room) | [gi](visual-checks/gi.md) | Dynamic probe GI in the Census office (`--off gi` A/B)
GI-2 | partial (2026-11-02: no artifacts seen; effect too weak to judge leaks) | [gi](visual-checks/gi.md) | DDGI visibility-weighted probe sampling: no leaks through the office floor/walls, bounce in corners (`--off gi` A/B)
GI-3 | confirmed (2026-11-02: looks good; defaults kept, gain 1, ambient 0) | [gi](visual-checks/gi.md) | GI replaces the flat cell ambient inside its volume: darker unlit corners, warmer near lamps, no box edge in the office (`flip gi` A/B)
GI-lampbody | awaiting owner | [gi](visual-checks/gi.md) | Lamp bodies don't light twice at GI hits (less hot bounce right around lamps, room fill kept) and interior ambient floor 0.1 (corners not pitch black) (`flip gi` A/B)
PH-15 | confirmed (2026-11-05: knee sink fixed; rapid balls push held bodies each time, S3) | [physics](visual-checks/physics.md) | Knockdowns of human NPCs are Delirium falls + get-ups (V ball, `@impact`); beasts and creatures keep the old ragdoll
PH-16 | confirmed for console kill (2026-11-05); hit deaths unseen until C6 | [physics](visual-checks/physics.md) | A killed human NPC falls and stays dead under Delirium (console `kill`); beasts and creatures keep the clip + limp ragdoll
LT-reach | rejected 2026-11-05 (default back to 4 x) | [lights](visual-checks/lights.md) | Lamp reach follows shadow state: no light through walls, no ring or pop at pool edges, GI visible next to lamps (`flip gi` A/B; old reach `--range-unshadowed 4`)
LM-2 | confirmed (2026-11-12: leaves OK; night water correct after W-SSPR) | [lights](visual-checks/lights.md) | NetImmerse vertex lighting mode, clamp modes, and 16-bit DDS cutouts
FF-2 | rejected (2026-10-30: all green to the horizon, L2/L3 never shown) | [rendering](visual-checks/rendering.md) | Far-field L1/L2/L3 F5 debug tint on proxies and impostors
DL-5 | awaiting owner | [rendering](visual-checks/rendering.md) | Interior `--rt --off denoiser` runs without `sky_motion` validation crash
RO-14 | confirmed (2026-11-02: sun at dusk; lamps and candles runs not checked) | [rendering](visual-checks/rendering.md) | Bloom on the game camera: sun, lamps, candles glow, lit walls stay crisp (`--off bloom` A/B)
RO-15 | confirmed (2026-11-02: 0.3 too faint, 1.0 liked, kept 0.5) | [rendering](visual-checks/rendering.md) | Light through leaves: backlit canopy glows, front-lit unchanged (`--off foliagetransmission` A/B)
RO-16 | 2026-11-06: ground better (fix 1, N1), moonlight OK (N3); night too lit (closer to OpenMW), Secunda not visible; fix 2, N2 unconfirmed; noon not checked | [rendering](visual-checks/rendering.md) | Terrain "veil" at grazing angles, blue-grey at night: name the console switch that lifts it (`scene ro16-terrain-veil`)
RO-17 | parked (owner 2026-11-06) | [rendering](visual-checks/rendering.md) | Tree leaves reflect as one flat panel in the light colour, leaf texture lost (2026-11-05)
TX-delight | awaiting owner | [rendering](visual-checks/rendering.md) | De-lit albedo textures: no double-dark corners, no colour shift or BC7 blocks, cutouts keep coverage (`--off delight` A/B)
TX-1 | awaiting owner | [rendering](visual-checks/rendering.md) | Normal maps from packs: relief under low sun / lamp light, no inverted bumps, no shimmer, no mirrored-UV seams (pack installed + re-bake; no A/B yet)
SS-matclass | awaiting owner | [rendering](visual-checks/rendering.md) | Static material classes: every static looks as with `--off matclass` (textures, cutout edges, two-sided leaves, glow) in Balmora and Seyda Neen
SC-scene | owner: matches sc-noscene (2026-11-11); floating foliage + shadow flicker are not from --scene; 883 batch-full warnings; perf: ~2-3 ms SLOWER on (batch overflow) | [rendering](visual-checks/rendering.md) | Static scene (`--scene`): every static present, no double draw, nothing jumps after a 256 m walk; doors/lights stay entities
SC-lamps | awaiting owner | [rendering](visual-checks/rendering.md) | Lamp shadows of `--scene` statics at night: present, steady, recapture after a new cell (scene sc-scene-night)
RO-B | partial (2026-10-30: draws in front now; odd blend, weak over dark things, like Ghostfence) | [rendering](visual-checks/rendering.md) | Lit smoke shading + dynamic plume Aabb sort over sky and objects
CB-1 | confirmed (2026-10-27: sprint hold, chain step, mouse capture, dummy collision fixed; evening: 4-hit chain and M1 cursor confirmed) | [combat](visual-checks/combat.md) | Light chain and roll on the training dummy (scene level-arena)
CB-2 | confirmed (2026-10-30) | [combat](visual-checks/combat.md) | Lock-on, locked facing and side rolls around the dummy (scene level-arena)
PH-2 | retired (2026-10-30: clip+limp ragdoll deprecated, delirium from now on) | [physics](visual-checks/physics.md) | a dead guard ragdolls on the Balmora stairs
PH-11 | partial (2026-10-30: still a walkable hole in the office floor, PH-12) | [physics](visual-checks/physics.md) | Physics fixes: no falls on load, still clutter, steady carry
WE-5 | confirmed with follow-up (2026-10-30: shelter too generous, cap to a few metres; clouds maybe high) | [weather-sky](visual-checks/weather-sky.md) | Ash/blight storm ignores terrain and objects, wrong ...
AL-1 | confirmed (2026-10-30) | [anim-lab](visual-checks/anim-lab.md) | Animation lab overview
AL-2 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Walk loop, turns
AL-3 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Ramps and stairs
AL-4 | retired (2026-10-30: superseded by AL-10/AL-11) | [anim-lab](visual-checks/anim-lab.md) | Traversal heights
AL-5 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Props push
AL-6 | retired (2026-10-30: superseded by delirium) | [anim-lab](visual-checks/anim-lab.md) | Falls and ragdoll
AL-7 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Idles, reactions, other rigs
AL-8 | confirmed with follow-up (2026-10-27: camera solved; hands on the lip + clipping: B2b next) | [anim-lab](visual-checks/anim-lab.md) | Waist mantle, player (climb clip on the body)
AL-10 | confirmed (2026-10-30: triggers and works; lip hand IK needs tuning, ROADMAP 3.1) | [anim-lab](visual-checks/anim-lab.md) | High mantle, player (ClimbUp_2m on the 1.6/2.0 m boxes)
AL-11 | confirmed (2026-11-05: chest brace good, spacing widened a little 0.55 → 0.68) | [anim-lab](visual-checks/anim-lab.md) | Wall brace, player (palms on the 3 m wall, side-wall trail, off hand only with the sword)
AL-12 | confirmed (2026-11-05: vault works; a small hiccup before the vault, not smooth or fast yet: tuning) | [anim-lab](visual-checks/anim-lab.md) | Vault over thin obstacles, player (SafetyVault, one hand on the top; anim-lab vault boxes, Seyda Neen crates and railing)
AL-9 | confirmed | [anim-lab](visual-checks/anim-lab.md) | Gait bands: pelvis steady, no sprint crouch, feet planted
AN-7 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Capsule steady under a walker
AN-1 | confirmed (2026-10-30) | [anim-lab](visual-checks/anim-lab.md) | Planted feet stay put on distant walkers
AN-3 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Footsteps on the feet
AN-2 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Walkers stop within a step
AN-4 | retired (2026-10-30: superseded by delirium) | [anim-lab](visual-checks/anim-lab.md) | Knockdowns always come back
AN-5 | retired (2026-10-30: superseded by delirium) | [anim-lab](visual-checks/anim-lab.md) | Get-up starts from the lying pose
AN-6 | retired (2026-10-30: superseded by delirium) | [anim-lab](visual-checks/anim-lab.md) | Get-up ends on the capsule
AN-8 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Creature capsules fit the creature
AN-9 | awaiting owner | [anim-lab](visual-checks/anim-lab.md) | Planted foot holds through the heel roll
AN-10, AN-13, AN-14 | retired (2026-10-30: superseded by delirium) | [anim-lab](visual-checks/anim-lab.md) | Get-ups
AN-21 | confirmed (2026-11-05: slope lean works) | [anim-lab](visual-checks/anim-lab.md) | Slope trunk lean
DE-1 | confirmed with follow-up (2026-10-27: floor slide fixed, push in F11 only; a crouched landing sinks ~1.5 s, later) | [anim-lab](visual-checks/anim-lab.md) | Delirium takeover: click-to-push, catch steps, blend-out to the idle, fall + roll-over + get-up
LOCO-2 | rated 2026-11-12 (229 takes: 38 keep, 61 maybe, 130 reject; verdicts.toml) | [anim-lab](visual-checks/anim-lab.md) | Generated locomotion batch 1 (Kimodo): gait, speed, skate, clean starts/stops, one style; pick keepers
AL-TRUNK | confirmed in the browser 2026-11-12 (arms clear the chest, trunk upright); in-game mantle/combat clips not yet looked at | [anim-lab](visual-checks/anim-lab.md) | Matched retarget pose: retargeted clips keep Morrowind's trunk curvature (upright vs before), mantle/combat no worse
LOOP-1 | confirmed 2026-11-12 (owner: "working"; no per-clip pick, seam numbers decide) | [anim-lab](visual-checks/anim-lab.md) | Loop-cut cycles in the clip browser: seam hitch/foot pop/speed surge per gait, L = raw take A/B
AU-3 | partial (2026-10-30: greets only very close; NPCs don't stop or turn) | [audio](visual-checks/audio.md) | NPC greetings, idle chatter, lip-sync level
AU-4 | confirmed (2026-10-30) | [audio](visual-checks/audio.md) | Footsteps by boots
AU-5 | rejected (2026-10-30: no swing, no sound, prompt only) | [audio](visual-checks/audio.md) | Swinging-door sounds
PL-1 | confirmed (2026-10-30) | [player-ui](visual-checks/player-ui.md) | Third-person walk and the use prompt
PL-2 | confirmed (2026-10-27: view/capsule split fixed: self-push + clip offset) | [player-ui](visual-checks/player-ui.md) | Animated player body (dressed `player` record, humanoid graph)
PL-3 | confirmed | [player-ui](visual-checks/player-ui.md) | Jump: take-off held in the air, landing on touchdown (graph B2)
PL-4 | confirmed (2026-10-27: "about right"; jump fixed with gravity 18) | [player-ui](visual-checks/player-ui.md) | Movement feel: starts, stops, reversals (player feel tuning)
IN-1 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Keymap controls, rebind, gamepad, sneak hold vs toggle
PM-1 | confirmed with follow-up (2026-10-30: mouse sliders, mute icon, Main menu button) | [player-ui](visual-checks/player-ui.md) | Escape menu and options (volume live and saved)
SV-2 | confirmed with follow-up (2026-10-30: thumbnails missing) | [player-ui](visual-checks/player-ui.md) | Save/load screens from the pause menu (save, overwrite, load, delete)
HD-1 | confirmed (2026-10-30) | [player-ui](visual-checks/player-ui.md) | Minimal HUD: bars, compass, crosshair, use prompt
HD-2 | confirmed (2026-11-05: bars ok; dummy health log to re-confirm) | [player-ui](visual-checks/player-ui.md) | Bars follow damage and regen (stats S5: bars from the player's sheet)
MM-1 | confirmed with follow-up (2026-11-11: flow works; look crude, text over grey boxes, not Morrowind/OpenMW) | [player-ui](visual-checks/player-ui.md) | Release start: intro logos, main menu with no world behind it, New game
OP-1 | confirmed with follow-up (2026-11-11: works; a plain list, needs real widget art); S2b1 description pane awaiting owner | [player-ui](visual-checks/player-ui.md) | Options rows: mouse-drag sliders, checkboxes (S2a, PM-1 follow-up)
LK-1 | owner: issues (submenus overlap) | [player-ui](visual-checks/player-ui.md) | Look pass L1: OpenMW frames on windows, boxes, framed buttons
LK-2 | owner: issues (Continue style) | [player-ui](visual-checks/player-ui.md) | Look pass L2: title menu as OpenMW image buttons, version text
FG-9 | confirmed (2026-11-02: no pillar; god rays still not visible, FG-10) | [weather-sky](visual-checks/weather-sky.md) | Vertical light pillar toward a low sun in fog: isotropic sky fill (fix built 2026-11-02)
FG-10 | known bug | [weather-sky](visual-checks/weather-sky.md) | No visible god rays at sunrise in fog (2026-11-02)
AI-1 | awaiting owner | [ai](visual-checks/ai.md) | AI debug overlay (`ai.select`, `ai.trace`, plan gizmos) on a synthetic trace, example `ai_overlay` (2026-11-09)
RG-1 | awaiting owner | [streaming](visual-checks/streaming.md) | Nothing pops in after a door fade or the loading screen lifts (reveal gate W1; `reveal:` lines)

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
CO-1 | confirmed (2026-10-30) | [player-ui](visual-checks/player-ui.md) | Console: exterior COC, paste, Down/Enter on suggestions (2026-10-26)

### Confirmed with follow-up (5)

DL-3 | confirmed with follow-up | [rendering](visual-checks/rendering.md) | Ray-traced Census Office with Ray Reconstruction
WA-2 | confirmed with follow-up | [terrain-water](visual-checks/terrain-water.md) | Shore and region optics, W3
RO-A | confirmed with follow-up | [rendering](visual-checks/rendering.md) | Translucents cast no shadows; distant lights and rai...
DJ-1 | confirmed with follow-up | [player-ui](visual-checks/player-ui.md) | Talk to Fargoth, Sellus greeting; resizable window wished
DJ-2 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Dialogue action buttons, Persuasion submenu modal (written 2026-11-08, untested)
PUI-SV3 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Trade screen over the dialogue: Barter with Arrille, stage, haggle, accept, cancel (TR2)
PUI-SV4 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Training list over the dialogue: top 3 skills, prices, refusals, buy +1 and 2 h pass, Back (A3)
SV-1 | confirmed with follow-up | [player-ui](visual-checks/player-ui.md) | Quicksave / quickload; load sometimes spawns below the floor

### Known bug (5)

SH-13 | known bug | [shadows](visual-checks/shadows.md) | Point-light (lamp) shadows
SH-14 | known bug | [shadows](visual-checks/shadows.md) | Contact-shadow bands at object edges
GF-1 | known bug | [rendering](visual-checks/rendering.md) | Ghostfence appears complete, never late
DL-4 | known bug | [rendering](visual-checks/rendering.md) | Rug-edge black bands under RT
RO-11 | confirmed (owner 2026-11-01: gone) | [rendering](visual-checks/rendering.md) | Black ground bands that move with the camera (2026-10-30)
RO-12 | known bug | [rendering](visual-checks/rendering.md) | Bright outline around characters (2026-10-30)
RO-18 | known bug | [rendering](visual-checks/rendering.md) | Grass and foliage floating in the sky, swapping as a batch every ~200 m (2026-11-11)
RO-19 | known bug | [rendering](visual-checks/rendering.md) | Very short shadow flickers in Balmora (2026-11-11)
RO-20 | confirmed 2026-11-12 (subtle shadow swimming on hills while approaching, logged) | [rendering](visual-checks/rendering.md) | Far terrain shadows keep following the player after the far-map upload stopped re-binding materials (2026-11-12)
RO-21 | confirmed 2026-11-12 | [rendering](visual-checks/rendering.md) | Haze and ash tint change smoothly in weather transitions, same look as before, with the medium no longer re-baked on the CPU (perf-push P1, 2026-11-12)
RO-22 | awaiting owner | [rendering](visual-checks/rendering.md) | Near/far handoff band: no gaps or doubles with GPU-culled statics, A/B `--off cpuculltag` (perf-push P3, 2026-11-12)
PH-12 | known bug | [physics](visual-checks/physics.md) | Walkable hole in the Census Office floor (again 2026-10-30)
PH-13 | known bug | [physics](visual-checks/physics.md) | Player and NPCs drop onto the ground when a cell loads (2026-10-30)
CB-3 | confirmed (2026-11-05; follow-up: new sword attack clips) | [combat](visual-checks/combat.md) | F draws a visible sword; sheathed clicks do nothing; draw / sheathe clips, sword at the attach key (2026-11-05)
CB-3a | confirmed 2026-11-12 (logged: late roll start, first-person chest clipping looking down) | [combat](visual-checks/combat.md) | Sparring dummy strikes on a beat; player hit reactions by side in third and first person, free look in a reaction (scene level-sparring, 2026-11-10)
CB-3b | partial 2026-11-12 (held guard loop jumps; stamina refills too fast to test the break; regenerate clips) | [combat](visual-checks/combat.md) | Guard (hold right mouse / L1) in third and first person: front hits guarded (no reaction, stamina dip), back hits not, guard break at 0 stamina; body follows the view in first person (scene level-sparring, 2026-11-10)
CB-5 | awaiting owner | [combat](visual-checks/combat.md) | First-person melee: swings where you look, soft aim bends onto the dummy, no lock in first person (scene level-arena, 2026-11-10)
CB-5b | awaiting owner | [combat](visual-checks/combat.md) | First-person attack look limits: the view stops ~45 deg off the body mid-swing, released at recovery; view model on and off (scene level-arena, 2026-11-10)
PL-6 | awaiting owner (2026-10-30: hard-landing lock removed until fall damage) | [player-ui](visual-checks/player-ui.md) | Recovery delay after jumping down from a high box (2026-10-30)
PL-7 | partial (owner 2026-11-11 re-test: feet on the ground OK; stop pops a leg back + hip rise, arms point back on slopes, head too tilted; locomotion rethink) | [player-ui](visual-checks/player-ui.md) | Player foot placement on slopes and stairs, K A/B (P-IK1, 2026-11-11)
PL-8 | awaiting owner (pause not tested 2026-11-11; stairs IK issues logged: leg snap, wide steps on stairs, arms behind back) | [player-ui](visual-checks/player-ui.md) | Pause menu freezes all animation (walkers on stairs, the player at a wall), no pop on resume (P-IK2, 2026-11-11)
WALK-0 | awaiting owner | [player-ui](visual-checks/player-ui.md) | A/B ladder on the player's walk: speed_match, stride_warp, foot_placement, lean, side_lanes off one at a time (3.4c A0)
WALK-1 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Speed gait set: generated walk / jog / sprint loops on speed, foot phase kept on gait switches, no skating (3.4c A3)
PL-9 | confirmed (owner 2026-11-11: "seems ok") | [player-ui](visual-checks/player-ui.md) | Sharp 90/180° turns at walk and run on flat and ramps: the planted foot lets go instead of the legs spreading, K A/B (P-IK5, 2026-11-11)
FP-1 | awaiting owner | [player-ui](visual-checks/player-ui.md) | First person shows body, arms and weapon with the head hidden (2026-11-10)
FP-1b | awaiting owner | [player-ui](visual-checks/player-ui.md) | First person: the player's shadow keeps its head under sun, lamps and a torch, raster and RT (2026-11-10)
FP-2 | awaiting owner | [player-ui](visual-checks/player-ui.md) | First-person view model (`flip viewmodel`): weapon and arms at the body FOV, never in walls, lit like the world, no DLSS/TAA smear; shader-compile log line (2026-11-10)
RO-13 | known bug | [rendering](visual-checks/rendering.md) | F7 wireframe drawn away from the model (2026-10-30)
WE-6 | known bug | [weather-sky](visual-checks/weather-sky.md) | Rain turns grainy while the camera moves (2026-10-30)
WE-7 | known bug | [weather-sky](visual-checks/weather-sky.md) | Rain curtain stops in front of distant water (2026-10-30)
SW-7 | OK for now 2026-11-06 (crude, rework; ash too big/bright; near-white at night) | [weather-sky](visual-checks/weather-sky.md) | Kick-up puffs (W7): Skaal Village `surface set snow 1` walk/run/sprint: white-blue powder puffs at each foot-down, bigger when running, lit like the snow (shadow, night); Ghostgate `surface set ash 1`: grey-brown puffs drift downwind; `--off kickup`
SW-6 | failed again 2026-11-06 after both fixes: resets remain (canopy shelter pops near trees), only prints, no shin trails; ring edge + paving fix not reported; ash a bit deeper | [weather-sky](visual-checks/weather-sky.md) | Knee-deep snow + footprints (W6a/W6b): Skaal Village `surface set snow 1`, walk: snow raised 0.45 m near the camera, easing out by ~43 m, feet sunk, prints cut to the ground with lips, no smear with DLSS; live `off trails` / `off cover`
SW-5 | coverage OK 2026-11-06; resets fix failed 2026-11-06 (tree canopy shelter pops in/out with distance) | [weather-sky](visual-checks/weather-sky.md) | Snow and ash cover: Ghostgate `surface set ash 1`, Skaal Village `surface set snow 1`; flat tops covered, walls/slopes bare, patchy at 0.4, none under roofs or on characters, snow glints; A/B `--off cover`
SW-4 | confirmed (2026-11-06 retune: faster fill, 6° slope) | [weather-sky](visual-checks/weather-sky.md) | Puddles, Balmora in rain: `surface set puddle 1`, flat sky-mirror water in hollows and grout, rings while raining that stop with the rain, none under roofs; A/B `--off puddles`
SW-3 | confirmed (owner 2026-11-05: works great, dries in a satisfying pattern; ground look a bit weird, see terrain roughness bug) | [weather-sky](visual-checks/weather-sky.md) | Wetness after rain, Seyda Neen: `surface set wet 1`, darker/glossier ground and walls, streaks, dry under roofs; A/B `--off wet`
SW-2 | confirmed (2026-11-06, after the render-layer fix) | [weather-sky](visual-checks/weather-sky.md) | Surface weather shelter at night (own top-down pass): roofs full, trees partial (`surface debug exposure`), 32 m cells, Balmora 23:00
PH-14 | confirmed (2026-11-05) | [physics](visual-checks/physics.md) | Dev kill key I kills the player in third person and opens the inventory (2026-10-30)
AU-6 | known bug | [audio](visual-checks/audio.md) | Third-person listener in the character head, not the camera (2026-10-30)
FG-8 | known bug | [weather-sky](visual-checks/weather-sky.md) | Weather fog vanishes at night (2026-10-30)

### Rejected (2)

SH-9 | rejected | [shadows](visual-checks/shadows.md) | Many shadowed lights indoors
LT-3 | rejected | [lights](visual-checks/lights.md) | Point-light shadows, torch-bearers

### Inconclusive (3)

SH-16 | awaiting owner (2026-11-01: contact-shadow flip shows nothing; isolating A/B in the entry) | [shadows](visual-checks/shadows.md) | Shadow reset A1
SH-19 | inconclusive | [shadows](visual-checks/shadows.md) | Shadow reset B2
FG-1 | inconclusive | [weather-sky](visual-checks/weather-sky.md) | Volumetric fog black bands on fast turns

### Deferred (22)

DL-2 | deferred | [rendering](visual-checks/rendering.md) | Super Resolution at 1440p
SK-2 | deferred | [weather-sky](visual-checks/weather-sky.md) | Night and moon
ST-2 | deferred (2026-11-12 owner: small flicker in distant terrain every ~200 m of travel; suspect far shadows at origin shifts) | [streaming](visual-checks/streaming.md) | No pop at origin shifts in flight
ST-3 | deferred | [streaming](visual-checks/streaming.md) | Paced model loads arrive in time
SC-2 | deferred | [streaming](visual-checks/streaming.md) | The cutoff fades, never pops
SC-foliage | owner: OK in Balmora (2026-11-11) | [rendering](visual-checks/rendering.md) | Cutout candidates (foliage, fences, grates) drawn from the static table, and no candidate that should fade softly gaining hard edges
SN-2 | deferred | [characters](visual-checks/characters.md) | Guar from its cooked skinned product
SN-3 | deferred | [characters](visual-checks/characters.md) | Dark elf head morphs
CH-1 | deferred | [characters](visual-checks/characters.md) | The M3 lineup
CH-3 | deferred | [characters](visual-checks/characters.md) | Balmora street with NPCs standing in place
CH-4 | deferred | [characters](visual-checks/characters.md) | Census and Excise Office staff
CH-5 | confirmed 2026-11-12 (new: idle NPCs float slightly above ground, logged) | [characters](visual-checks/characters.md) | Busy Balmora street: NPCs deform normally with shared skin joint blocks, no DLSS smear (perf-push P5, 2026-11-12)
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
NS-3 | deferred | [weather-sky](visual-checks/weather-sky.md) | Masser and Secunda
NS-4 | deferred | [weather-sky](visual-checks/weather-sky.md) | No red streaks at night
WE-8 | confirmed (2026-11-06) | [weather-sky](visual-checks/weather-sky.md) | Rain leans with the wind, capped (Rain ~9-15°, thunderstorm ≤ ~25°), direction turns slowly, trees agree; `view "Balmora" --hour 12 --weather rain`, A/B `flip rain_slant_cap`
WE-10 | fix 1 (history) failed 2026-11-06: shimmer stays, storm not over water; owner 2026-11-12: still a lot of shimmer on everything behind fog and clouds | [weather-sky](visual-checks/weather-sky.md) | Local storm seen from outside: cloud curtain and noisy border (`scene we3-ridge` + BRP region force)
WE-9 | fog fix confirmed 2026-11-06; causes 1 (hour keys, re-seen) and 3 open | [weather-sky](visual-checks/weather-sky.md) | Forced fog and rain seem to go away at night: hour keys wrap to the same day (live force undone), fog unlit by the night ambient floor, rain drops unlit at night
