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
Status: **owner screenshot 2026-11-17 (Balmora, `nav.draw tiles`): roofs, bridges, stairs and canal walks drawn; most terrain
ground between houses shows no lines.** Cause (code): the draw uses Detour polygon vertices only (`sf_nav_recast` `tile_geometry`,
5 cm lift), which cut under bumpy terrain and are hidden by depth; the detail mesh (built, `build.rs:166`) is not drawn. Fixed
the same day (outlines follow the detail mesh, may float up to ~0.3 m over bumps): **awaiting re-check**. Roofs: Recast keeps every walkable
surface (unreachable islands are standard; pruning from pathgrid seeds is a possible later option).
1. Type `nav.draw tiles`: humanoid tile outlines (19.2 m tiles) out to ~256 m around you, walls brighter; tiles fill in
   over a second or two (4 per frame). Walk or fly ~100 m: new tiles appear ahead, far ones drop behind.
2. Type `nav.stats`: `nav: N tiles (class 0: N) ...; ring: 0 pending, E empty, U unbaked, 0 failed, L loaded, D dropped`.
3. Look for: outlines lie on the ground (not floating or offset by a cell), cover streets and the dock, stop at house
   walls and water; nothing over rooftops you can't reach. `nav.draw off` hides them.

### NV-2: NPCs walk smoothly on the fixed tick (`owner_launch.sh view "Balmora" --hour 12`, slice NV7a)
Status: **awaiting owner** (built and headless-tested 2026-11-17). NPC movement moved from frame rate to the 60 Hz tick
with interpolation, and their physics capsules now follow the tick pose (the old AN-7 lead was deleted).
1. Watch street walkers uncapped (high fps), then cap to 30 fps if your build has the cap key/flag: no judder, stepping
   or foot sliding at either rate; turns stay smooth.
2. Walk away until NPCs freeze (processing range) and come back: no pop or slide when they wake.
3. Knock one down / ragdoll it and let it get up; throw a ball at a walker: capsule sits on the body, no snap.
4. Fly the camera into a walker's path: it still steps around the camera.
