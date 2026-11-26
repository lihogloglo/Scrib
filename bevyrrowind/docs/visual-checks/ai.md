# Visual checks: NPC AI (superfargoth `docs/specs/ai.md`, prefix `AI-`)

## Synthetic scenes (framework examples, no game data)

### AI-1: AI debug overlay on a synthetic trace (`cargo run -p sf_ai_bevy --example ai_overlay --features debug`, slice AI3)
Status: **awaiting owner** (built and headless-tested 2026-11-09; nothing decides in game before AI4/AI6, so the
trace is pushed at startup: the overlay shows only what the store holds). Backtick opens the console, WASD flies.
1. Click the left capsule, type `ai.select`: amber ring at its feet; above its head, facing the camera, four lines:
   `ai 000...a1 (watched)`, `#110 t2 plan kept_not_better`, `plan [p_wander] | paused`, `op wander running`.
2. `ai.select a2` (middle): red ring, `plan no_plan`, `plan -`, `op -`. `ai.select a3` (right): grey ring,
   `no decision traced yet`. `ai.trace` prints the steps (a1: two lines, `rej be_guard:patrol pre0`).
3. `ai.gizmos off` hides it, `ai.gizmos on` brings it back; `ai.select off` removes it. Look for: text readable
   from ~5 m, not mirrored, follows the camera when you fly round; nothing drawn on the unselected capsules.

## Game scenes

### NV-1: baked navmesh tiles drawn over Seyda Neen (`owner_launch.sh view "Seyda Neen" --hour 12`, slice NV4c)
Status: **confirmed by owner 2026-11-18 (Balmora: `nav.stats` good, `nav.draw tiles` shows tiles streaming and following the terrain).** Two earlier causes fixed: (1) 2026-11-17: outlines follow Detour's detail mesh instead of coarse
polygon edges; (2) 2026-11-18: `NavSpace::build` (`mw_bake/src/nav.rs:298`) listed level-0 terrain tiles 128 m off per axis
(`tx*256` instead of `TileCoord::origin()` = `tx*256-128`), leaving ~3/4 of exterior land without terrain polys; fixed and
exterior navmesh re-baked (80,892 tiles rebuilt). Roofs: Recast keeps every walkable
surface (unreachable islands are standard; pruning from pathgrid seeds is a possible later option).
1. Type `nav.draw tiles`: humanoid tile outlines (19.2 m tiles) out to ~256 m around you, walls brighter; tiles fill in
   over a second or two (4 per frame). Walk or fly ~100 m: new tiles appear ahead, far ones drop behind.
2. Type `nav.stats`: `nav: N tiles (class 0: N) ...; ring: 0 pending, E empty, U unbaked, 0 failed, L loaded, D dropped`.
3. Look for: outlines lie on the ground (not floating or offset by a cell), cover streets and the dock, stop at house
   walls and water; nothing over rooftops you can't reach. `nav.draw off` hides them.

### NV-2: NPCs walk smoothly on the fixed tick (`owner_launch.sh view "Balmora" --hour 12`, slice NV7a)
Status: **confirmed by owner 2026-11-18 (Balmora).** NPC movement moved from frame rate to the 60 Hz tick
with interpolation, and their physics capsules now follow the tick pose (the old AN-7 lead was deleted).
1. Watch street walkers uncapped (high fps), then cap to 30 fps if your build has the cap key/flag: no judder, stepping
   or foot sliding at either rate; turns stay smooth.
2. Walk away until NPCs freeze (processing range) and come back: no pop or slide when they wake.
3. Knock one down / ragdoll it and let it get up; throw a ball at a walker: capsule sits on the body, no snap.
4. Fly the camera into a walker's path: it still steps around the camera.

### AI-2: wanderers run the HTN wander/idle methods over the navmesh (`owner_launch.sh view "Balmora" --hour 12`, slice MA3)
Status: **confirmed by owner 2026-11-18 (Balmora): nobody frozen, NPCs follow the drawn paths, log line OK, `nav.draw paths` correct.** Rhythm (point 3) not commented. Ground wanderers now plan with `mw_ai::methods` (HTN over `sf_ai`) and walk
`sf_nav` navmesh paths (`NavCorridor`); the pathgrid is only a fallback (fliers, unbaked cells).
Log: `ai: N wanderers on navmesh, M on pathgrid fallback (since start: X navmesh walks arrived, ...)`: N most wanderers, X rising.
1. Street wanderers walk smooth paths to points near their spawn, stop, sometimes fidget, then walk again.
2. Nobody stands frozen for long, walks into walls, or leaves its area. `nav.draw paths` shows the paths.
3. Fidgets vary (never the same one twice in a row). Is the walk/pause/fidget rhythm right? (Plain pauses are 2-5 s,
   not OpenMW's per-tick re-roll; say if NPCs fidget too often or too rarely.)

### AI-3: perception gizmos and NPCs noticing the player (AI26 + AI8, 2026-11-25)
- Kind: game, dev build (`dev` now enables `sf_ai_bevy/debug`). `owner_launch.sh view "Seyda Neen" --hour 12`, then
  again at `--hour 23`. Third person (F11). Console: `ai.perception on`, `ai.select` + click an NPC.
- Look for: grey cones with ground arcs at each NPC; walking into a cone turns it yellow → orange → red with
  "<band> N%" rising; a sphere where it last saw you and "LKP ... s ago" growing behind cover; "search held" + ring
  while it searches (it walks to the spot, looks around, widens, gives up after ~30 s, then wanders); "light N%"
  over the player dropping in shade / at night (pitch dark = unseen, known until crime LW4); sprinting/landing
  leaves blue rings sized to the noise reach (run 12 m, sneak 2 m), fading in ~2 s; text never overlaps `ai.gizmos`.
- Log: none required. Status: awaiting owner (gameplay-roadmaps-5).

### AI-11: perception lab, controlled cases (AI47, owner request 2026-11-25)
- Kind: level, dev build. `owner_launch.sh scene level-perception` (overview) or `level-perception-{light,cover,
  hearing,search,impacts}` (one camera per area; hour 23, clear; gizmos on via `--set ai.perception=true`). F11 drops
  the player; Ctrl hold sneak, Shift hold sprint, V throws a ball.
- Light: walk each lane north to its guard: the lit lane's band rises early (~12 m post) and fast, the dark one late or
  never; "light N%" higher in the lit lane. Cover: walk the white line: band stays down behind the wall, rises in line
  with a gap; the LKP marker stays at the gap. Hearing: cross the 4/8/14 m stripes: run heard at 4 and 8 not 14,
  sneak never, sprint at 14, walk 4 only; rings show each step's reach; hearing alone tops out at Curious.
- Search (AI47b, 2026-11-25): the level now has a navmesh; the search guard should leave its post and walk around the boxes. Log: `nav: level tiles 60 in X ms`. Impacts make no noise yet (IN18).
- Status: owner 2026-11-25 "worked, too early to tell"; owner 2026-11-26 (after AI47b, AI8i, AI8h2, MA18): "all good": OK.

### AI-12: sneaking hides you from sight (MA18 + LW4, owner request 2026-11-25)
- Kind: level, dev build. `owner_launch.sh scene level-perception-light` (hour 23, clear, gizmos on). F11 drops the player.
- Dark lane (x -72, guard-dark): walk north to the guard upright, then again holding Ctrl. Sneaking: his meter fills
  clearly slower and he notices you much later (headless: 1.00 vs 0.24 at 90 ticks in 0.40 light). Lit lane (x -48):
  sneaking still slows it, less. Sprinting gives you away (fast movement reveals). Behind him you stay hidden longer.
- Weights are Guess tunables `crime.stealth.*`, live since LW4b2 (e.g. `--set crime.stealth.crouched_concealment=60`). Morrowind's sneak skill formula is MA5.
- Status: owner 2026-11-25 "seems to work but no guard is ever moving": Curious/Suspicious reactions never built (AI8h1/h2), search guard didn't search (AI47c diagnosis). AI8h2 + AI8i done 2026-11-25: re-check: a guard at Curious turns to face you, at Suspicious walks ~3 m toward where he saw you and stands facing it, then walks back to his post when the band drops.
- Verdict owner 2026-11-26: OK ("when they are suspicious they step toward me, look around and walk back"). Alerted guards stand still: expected, Alerted response = AI29 (warn, pursue) and combat (MA4d, MA10).

### AI-13: NPCs greet the player (MA4c2, 2026-11-26)
- Kind: game, dev build (voices on: they are dev-only). `owner_launch.sh view "Seyda Neen" --hour 12 --sound`, F11 drops
  the player (the free camera is never greeted). Wandering NPCs around the lighthouse and the docks.
- Proof log line: `greeting: N greeted (M without a voice)` (first greeting, then each time the total doubles; M > 0 is
  an NPC with no voiced Hello, e.g. most creatures).
- Q1: walk up to a wandering NPC (within ~2.5 m for a typical Hello 30): does it stop, turn its body to face you and say a
  greeting? yes/no
- Q2: stay near it, step around it within ~7 m: does it NOT greet a second time, and walk on after ~4 s? yes/no
- Q3: walk ~8 m away and come back: does it greet again? yes/no
- Q4: approach one from behind: does it turn around smoothly (no snap)? yes/no
- Status: awaiting owner.
