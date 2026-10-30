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

---

## Open checks index

One line per open ID: `ID | status | file | title`. Entries live in [`visual-checks/`](visual-checks/); closed ones in [`archive/visual-checks-closed.md`](archive/visual-checks-closed.md). Add a new entry to the right system file and a line here.

### Awaiting owner (37)

SH-11 | confirmed (2026-10-30) | [shadows](visual-checks/shadows.md) | Distant flashes while moving (PostUpdate far-shadow ordering + world-unit PCSS)
SH-20 | partial (2026-10-30: smooth now; which lamps cast looks random, player and torches cast none) | [shadows](visual-checks/shadows.md) | Character lamp shadows update per frame within 16 m (`--off movershadows` A/B)
LM-2 | partial (2026-10-30: leaves OK, water still too bright at night) | [lights](visual-checks/lights.md) | NetImmerse vertex lighting mode, clamp modes, and 16-bit DDS cutouts
FF-2 | rejected (2026-10-30: all green to the horizon, L2/L3 never shown) | [rendering](visual-checks/rendering.md) | Far-field L1/L2/L3 F5 debug tint on proxies and impostors
DL-5 | awaiting owner | [rendering](visual-checks/rendering.md) | Interior `--rt --off denoiser` runs without `sky_motion` validation crash
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
AL-11 | partial (2026-10-30: brace shows; slides sideways on a straight hit; trail pose should be hand ahead, thumb down) | [anim-lab](visual-checks/anim-lab.md) | Wall brace, player (palms on the 3 m wall, side-wall trail, off hand only with the sword)
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
DE-1 | confirmed with follow-up (2026-10-27: floor slide fixed, push in F11 only; a crouched landing sinks ~1.5 s, later) | [anim-lab](visual-checks/anim-lab.md) | Delirium takeover: click-to-push, catch steps, blend-out to the idle, fall + roll-over + get-up
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
MM-1 | rejected (2026-10-30: wants title art/video, not the live world) | [player-ui](visual-checks/player-ui.md) | Release start: main menu over Seyda Neen, New game

### Partial (9)

SH-15 | partial | [shadows](visual-checks/shadows.md) | 8 cached lamp shadows
DR-1 | partial | [streaming](visual-checks/streaming.md) | Through a door into an interior
DR-2 | partial | [streaming](visual-checks/streaming.md) | Back out
LT-2 | partial | [lights](visual-checks/lights.md) | Dark interior keeps its lights
PH-7 | partial | [physics](visual-checks/physics.md) | Item body shapes
PH-9 | partial | [physics](visual-checks/physics.md) | knockdown and get-up
WA-1 | partial (2026-10-30: moiré fixed; no rain or wake ripples; shore/whitecaps Phase 4) | [terrain-water](visual-checks/terrain-water.md) | Open sea, W2 FFT ocean
IV-1 | partial | [player-ui](visual-checks/player-ui.md) | Take items and Fargoth's ring; inventory layout, hover, drop, paper doll missing
IV-2 | confirmed with follow-up (2026-10-30: icons only on hover, E should close) | [player-ui](visual-checks/player-ui.md) | Container window: the ring barrel outside the Census office (ring), move both ways, Take all closes
CO-1 | confirmed (2026-10-30) | [player-ui](visual-checks/player-ui.md) | Console: exterior COC, paste, Down/Enter on suggestions (2026-10-26)

### Confirmed with follow-up (5)

DL-3 | confirmed with follow-up | [rendering](visual-checks/rendering.md) | Ray-traced Census Office with Ray Reconstruction
WA-2 | confirmed with follow-up | [terrain-water](visual-checks/terrain-water.md) | Shore and region optics, W3
RO-A | confirmed with follow-up | [rendering](visual-checks/rendering.md) | Translucents cast no shadows; distant lights and rai...
DJ-1 | confirmed with follow-up | [player-ui](visual-checks/player-ui.md) | Talk to Fargoth, Sellus greeting; resizable window wished
SV-1 | confirmed with follow-up | [player-ui](visual-checks/player-ui.md) | Quicksave / quickload; load sometimes spawns below the floor

### Known bug (4)

SH-13 | known bug | [shadows](visual-checks/shadows.md) | Point-light (lamp) shadows
SH-14 | known bug | [shadows](visual-checks/shadows.md) | Contact-shadow bands at object edges
GF-1 | known bug | [rendering](visual-checks/rendering.md) | Ghostfence appears complete, never late
DL-4 | known bug | [rendering](visual-checks/rendering.md) | Rug-edge black bands under RT
RO-11 | known bug | [rendering](visual-checks/rendering.md) | Black ground bands that move with the camera (2026-10-30)
RO-12 | known bug | [rendering](visual-checks/rendering.md) | Bright outline around characters (2026-10-30)
PH-12 | known bug | [physics](visual-checks/physics.md) | Walkable hole in the Census Office floor (again 2026-10-30)
PH-13 | known bug | [physics](visual-checks/physics.md) | Player and NPCs drop onto the ground when a cell loads (2026-10-30)
CB-3 | known bug | [combat](visual-checks/combat.md) | F never draws a sword (2026-10-30)
PL-6 | awaiting owner (2026-10-30: hard-landing lock removed until fall damage) | [player-ui](visual-checks/player-ui.md) | Recovery delay after jumping down from a high box (2026-10-30)
RO-13 | known bug | [rendering](visual-checks/rendering.md) | F7 wireframe drawn away from the model (2026-10-30)
WE-6 | known bug | [weather-sky](visual-checks/weather-sky.md) | Rain turns grainy while the camera moves (2026-10-30)
WE-7 | known bug | [weather-sky](visual-checks/weather-sky.md) | Rain curtain stops in front of distant water (2026-10-30)
FG-7 | known bug | [weather-sky](visual-checks/weather-sky.md) | Blocky crenellated edges on distant land in fog (2026-10-30)
PH-14 | known bug | [physics](visual-checks/physics.md) | Dev kill key I kills the player in third person and opens the inventory (2026-10-30)
AU-6 | known bug | [audio](visual-checks/audio.md) | Third-person listener in the character head, not the camera (2026-10-30)
FG-8 | known bug | [weather-sky](visual-checks/weather-sky.md) | Weather fog vanishes at night (2026-10-30)

### Rejected (2)

SH-9 | rejected | [shadows](visual-checks/shadows.md) | Many shadowed lights indoors
LT-3 | rejected | [lights](visual-checks/lights.md) | Point-light shadows, torch-bearers

### Inconclusive (3)

SH-16 | inconclusive | [shadows](visual-checks/shadows.md) | Shadow reset A1
SH-19 | inconclusive | [shadows](visual-checks/shadows.md) | Shadow reset B2
FG-1 | inconclusive | [weather-sky](visual-checks/weather-sky.md) | Volumetric fog black bands on fast turns

### Deferred (22)

DL-2 | deferred | [rendering](visual-checks/rendering.md) | Super Resolution at 1440p
SK-2 | deferred | [weather-sky](visual-checks/weather-sky.md) | Night and moon
ST-2 | deferred | [streaming](visual-checks/streaming.md) | No pop at origin shifts in flight
ST-3 | deferred | [streaming](visual-checks/streaming.md) | Paced model loads arrive in time
SC-2 | deferred | [streaming](visual-checks/streaming.md) | The cutoff fades, never pops
SN-2 | deferred | [characters](visual-checks/characters.md) | Guar from its cooked skinned product
SN-3 | deferred | [characters](visual-checks/characters.md) | Dark elf head morphs
CH-1 | deferred | [characters](visual-checks/characters.md) | The M3 lineup
CH-3 | deferred | [characters](visual-checks/characters.md) | Balmora street with NPCs standing in place
CH-4 | deferred | [characters](visual-checks/characters.md) | Census and Excise Office staff
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
