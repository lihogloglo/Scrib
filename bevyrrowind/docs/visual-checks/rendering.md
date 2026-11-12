# Visual checks: rendering

Open visual checks: far field, ghostfence, DLSS, render order, synthetic scenes (FF, GF, DL, RO). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Far field (superfargoth `far-field.md`, "Measured on Morrowind")

Screenshots: `D:\Gamedev\Bevyrrowind\shots\farfield\` (`balmora_a*`,
`redmtn_view*`, `summit_vivec*`, `ebonheart_night*`, `handoff_forest*`).

### FF-2: Far-field levels
- Kind: game location. Run: `.\bevyrrowind scene balmora`, press F5.
- Look for: L1 green from ~400 m, L2 blue from ~2 km, L3 purple beyond; no
  untinted gap and no patch that flickers between levels while flying.
- A/B: F5 on/off.
- Status: awaiting owner (render-opt 2026-10-28: `ImpostorSet` now holds per-level `[Handle<ImpostorMaterial>; 3]` sharing GPU texture arrays plus an emissive debug boost so F5 tints L1 green, L2 blue, and L3 purple cleanly through aerial haze; check `.\bevyrrowind scene summit-vivec --far-tint`).
- Source: farfield-lod.
- Owner 2026-10-30 (Shift+F6; F5 is quicksave now): every distant asset is green up to the horizon; no blue or purple
  seen, so L2/L3 still look never selected (hard-problems.md second tier). Status: rejected.

## Ghostfence (superfargoth `weather/distant-shadows.md`, bug B)

### GF-1: Ghostfence appears complete, never late
- Kind: game location. Run: `.\bevyrrowind scene ghostfence-far` and fly
  toward Ghostgate (the camera starts ~1.9 km south-west, 120 m up, facing it).
- Look for: the fence segments appear at their ring (~520-650 m) already
  loaded: no frames where the pylons stand without the field after a cell
  group reveals (B3 preload). Not fixed yet: the field is still invisible
  past the ring and appears there without a fade. Its segments are
  activators made only of blended parts, so they have no far copy (weather/distant-shadows.md
  B5: blended far proxies + scriptless activators).
- A/B: F5 (pylons tint green at L1; fence segments stay untinted). Compare
  `play\` before this session's exe if kept: there the field showed up a few
  frames after its cell.
- Also (owner 2026-10-19, round 6, from the summit at night): round light spots sit inside the fence "wall": they are the far-distance light sprites of the fence's blue lights, drawn while the field itself has no far copy. Not vanilla-looking.
- Status: known bug, seen by owner 2026-10-19 (round 2): the fence field disappears entirely at a distance and pops back in close. Cause known: blended-only activators have no far copy (weather/distant-shadows.md B5); Phase 1 or later.
- Source: weather-spec / farfield / nifpop.

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
- Status: awaiting owner (render-opt 2026-10-28: guarded `sf_render_order_bevy/src/sky_motion.rs` `prepare` on `TextureUsages::TEXTURE_BINDING` so `view "Seyda Neen, Census and Excise Office" --rt --off denoiser` runs without wgpu validation panic).
- Source: dlss-plugin-2.

## Synthetic scenes (framework examples)

Run from `D:\Gamedev\Bevyrrowind\superfargoth`; they bake their data on
first run (cached afterwards). Each isolates one system on generated
content, so a bug seen here isn't Morrowind content.

## Haze and dust (superfargoth `weather/distant-shadows.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

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
- Status: awaiting owner (render-opt 2026-10-28: added lit smoke shading via `p.misc.z` in `particles.wesl` so dark smoke catches ambient/froxel/point light against dark skies, replaced emitter-origin `NoFrustumCulling` sort with dynamic conservative plume `Aabb` in `mw_assets/src/particles.rs`, and swept `RO-6` DLSS `main_pass_viewport` UVs across clouds/water/precip/sky_motion).
- Owner 2026-10-30 (`scene hi-balmora-night`, dev copy): torch smoke now draws over everything, but its blending looks
  wrong, like the Ghostfence (GF-1): it seems not to show over darker things. Hard to describe; looks like a blend-mode
  issue (additive vs alpha?). Status: partial.

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

### RO-13: F7 wireframe drawn away from the model (owner report)
- Kind: synthetic. Any anim-lab level, dev build, F7 (wireframe over skinned meshes; `mw_devtools_bevy/src/live_ab.rs`).
- Seen 2026-10-30: the wireframe seems to exist but shows at a different place than the character it belongs to.
- Suspect (not tested): the wireframe copy isn't placed in the same big_space grid cell / transform as the skinned mesh.
- Status: known bug: not diagnosed (owner pass 2026-10-30).
- Owner 2026-10-30 (Balmora, release copy): in the game F7 shows the wireframe on the terrain only, none on characters.

### RO-14: Bloom on the game camera (2026-11-01)
- Kind: game location, dev or release build, High preset. Runs: (1) `view "Balmora" --hour 18.5`, face the sun
  (west); (2) `view "Balmora" --hour 23`, look at the street lamps and braziers; (3) `scene census`, the candles.
- Look for: a soft glow around the sun disk, lamp and candle flames and fire only; lit walls and sunlit ground keep
  crisp edges, no milky haze over the whole image; no pulsing when the camera stops (DLSS/TAA run before bloom).
- A/B: `--off bloom` (or console `render.bloom 0`; retune live, e.g. `render.bloom 0.2`; default 0.1).
- Status: confirmed (owner 2026-11-02, sun at dusk; night lamps and candles not checked). Source: visual-niceties.

### RO-15: Light through leaves (foliage diffuse transmission, 2026-11-01)
- Kind: game location. Runs: `view "Seyda Neen" --hour 7` looking east and `view "Balmora" --hour 18.5` looking
  west, from under or behind trees so the sun sits behind the canopy; then turn around (sun at your back).
- Look for: backlit leaves glow warm/green-gold, leaves shadowed by the trunk or other trees stay darker; front-lit
  leaves look as before or a touch darker (half their direct light goes through); no bright rims or speckle on
  cutout edges; far impostor trees are unchanged (not covered).
- A/B: `--off foliagetransmission` (startup only; value `mw_assets::LEAF_DIFFUSE_TRANSMISSION` 0.5 (owner 2026-11-02, was 0.3), thickness 0.05 m).
- Status: confirmed (owner 2026-11-02 at 0.5; 1.0 tried and liked). Source: visual-niceties.

### RO-16: Terrain "veil", as if seen through resin (owner report 2026-11-05; was ROADMAP 1.12 "RO-13")
- Kind: game location, dev build. Run: `powershell -File tools/run_dev.ps1 scene ro16-terrain-veil` (Balmora street, 12:00, clear), then again with `--hour 23`. Seen: strongest at grazing angles, ground takes the sky/env colour, night = smooth blue-grey "clay" sheet, silhouettes lighter, rocks/trees less hit.
- Console (backquote), one at a time, each followed by the same line again to restore: `flip terrain_env_specular` (env-map specular only), `flip terrain_specular` (every specular lobe), `flip ambient` (the sky's flat 4-nit blue night ambient, all objects), `flip terrain_indirect` (env map + ambient + GI on the terrain), `render.bloom 0` (back: `render.bloom 0.1`), `flip fog`, `flip surface`, `flip terrain_tint` (raw layers). F1 shows their state.
- If it's the culprit: env_specular fixes noon grazing/shade; terrain_specular also fixes night; ambient (night only) makes the ground darker and its own colour again, no blue; indirect = env or ambient (ask which of the two above); bloom/fog lift the whole image, not only the terrain; surface/tint should change nothing on dry ground.
- Owner 2026-11-05, night: `ambient` lifts it (ground dark, no blue); `terrain_specular` removes the blue reflection and keeps the ground visible; `terrain_env_specular` no visible change; the rest irrelevant. Cause at night: the flat blue ambient (its specular lobe most). Noon not checked yet. Status: night diagnosed, fix pending. Toggles are RO-16 scaffolds (`TerrainDebug` bits 8/16/32/64, `Switch::Terrain*`, console `ambient`): remove with the fix.
- Fix 1 (terrain reflectance 0.3, was 0.5; console `terrain.reflectance`): look for the blue night film mostly gone, sheen in sun and rain kept; try 0.2-0.4 live. Status: awaiting owner.
- Fix 2 (night ambient from weather, `flip weather_ambient`): look for no blue cast at night; Clear vs Overcast vs Ash tint; brightness unchanged. Weather via C (next weather), `--hour 23`. Vanilla Clear nights stay slightly blue (Morrowind's own colour); Overcast/Rain greyer. Status: awaiting owner.
- N1 (flat ambient diffuse only, 2026-11-06; `flip ambient_specular`, on = old): `--hour 23`, look for terrain, walls and roofs keeping their dark readable colour with no blue sheen at grazing angles (silhouettes no longer lighter); lamps' highlights unchanged; noon identical. Also an interior (`view "Seyda Neen, Census and Excise Office"`): no sheen, not darker overall. Status: awaiting owner.
- N2 (night sky glow, 2026-11-06; `flip night_sky_glow`, off = black sky): `--hour 23`, look for a dark sky brighter toward the horizon (not a twilight; vanilla Clear "Sky Night Color" is near grey: say if it should take the bluer ambient hue), thin clouds darker than the sky not distracting, stars still visible, overcast = dim grey-blue cloud not black, wet/smooth surfaces reflecting that sky faintly; C cycles weather (Blight: reddish sky). Noon identical. Status: awaiting owner.
- N3 (the moons light the world, 2026-11-06; `flip moonlight`, off = the old weak moon): `--hour 23` (half Masser), `--day 167 --hour 21.5` (both full, low), `--day 179 --hour 23` (new: floor only). Look for walls and ground facing the moon visibly lighter and bluer (`Sun Night Color`) than the shadowed side, soft moon shadows near the camera (2 cascades to 120 m), no part darker than with `flip moonlight` off; overcast = faint. Risk: the moon also lights the sky (a blue-grey moonlit sky, brighter than N2 alone): say if it reads as dusk. Status: awaiting owner.
- Owner 2026-11-06 (`--hour 23`, clear): ground better (fix 1 / N1 OK), soft moonlight OK (N3); the night is too lit ("we can see very
  well"): wants values closer to OpenMW (knobs `--night-ambient` 4 nits, `--exposure-floor` 3.5; 1 nit was "too dark" 2026-10-19).
  Secunda (white moon) not visible: hidden behind Masser or not drawn, not diagnosed. Status: night brightness + Secunda open.

### TX-delight: De-lit albedo textures (bake layer, 2026-11-05)
- Kind: game location. Needs a current `bevyrrowind bake` (start log: `delight: layer mounted, N textures`).
  Runs: `view "Seyda Neen, Census and Excise Office"` and `view "Balmora" --hour 14` facing a lit house wall;
  A/B by restarting each with `--off delight` (log `delight: off`).
- Look for: corners and edges no longer double-dark under real light; walls not flat or cartoony; no colour shift;
  no blocky BC7 artifacts on gradients; cutout foliage (hedges, tree cards) not thinner at distance (coverage mips).
- Strength 0.5 (owner 2026-11-05) is baked in: a new strength needs a re-bake.
- Status: awaiting owner. Source: rtx-borrow D-2b.

### TX-1: Normal maps from packs (textures T4, 2026-11-11)
- Kind: game location. Needs a normal-map pack installed as a mod (MVR PBR v2 Beta or Tamrielic PBR West Gash; census in
  `textures-adapter.md` §7) and a fresh `bevyrrowind bake` (line `texture.cook: ... N normal companions`, N > 0).
- Runs: `view "Balmora" --hour 7` and `--hour 18` (low sun raking across house walls and paving), then
  `view "Seyda Neen, Census and Excise Office"` (lamp light on walls). No `--off normals` A/B yet (needs main.rs/perf.rs).
- Look for: relief that moves with the light on pack-textured walls/floors; bumps not inverted (lit side faces the sun);
  no shimmer or sparkle at distance; no hard seams where a texture is mirrored (symmetric trims, statues).
- Not covered: NPC/creature bodies (assembly carries no tangents yet), terrain (T5/T6).
- Status: awaiting owner. Source: textures T4.

### SS-matclass: Static material classes look unchanged (static-scene S2, 2026-11-08)
- Kind: game location, A/B. Runs: `view "Balmora" --hour 14` and `view "Seyda Neen" --hour 14`, each default and
  restarted with `--off matclass` (old materials, one sampler per image). Default should look identical.
- Look for any surface that differs between the two: missing or wrong textures (sampler sharing), clamped vs tiled
  edges on signs and banners, cutout edges (fences, hedges, tree cards) thicker or thinner, two-sided leaves and
  banners losing their back face, leaf light-through (RO-15) still on, decals or glowing windows unchanged.
- Gauges in a perf run (`statics/material_pipelines`, `statics/material_bind_groups`, `statics/bindless`): perf.md
  "Static material classes (S2)". If anything changed, run with `--off matclass` and report the surface.
- Status: awaiting owner.

### RO-17: Tree leaves reflect as one flat panel (owner report 2026-11-05)
- Kind: game location. Seen in `scene ro16-terrain-veil` (Balmora, 23:00 clear): a whole leaf card reflects as one panel in the light's colour, the leaf texture lost, no micro detail.
- Suspects (not diagnosed): flat cards with no normal map catch one uniform specular (statics reflectance 0.3, roughness 0.85, `mw_assets/src/lib.rs:453`); the flat-ambient specular lobe (same as RO-16 night); foliage transmission (RO-15) lighting the card evenly (A/B `--off foliagetransmission`).
- Status: parked (owner 2026-11-06: later).

### SC-foliage: Cutout candidates drawn from the static table (static-scene S1b, 2026-11-09)
- Why: an authored blend marked `nif.blend = "cutout_candidate"` draws alpha-tested at load when its texture's
  alpha has holes, and blended when it does not. The bake now reaches the same verdict from the texture bake's
  per-texture alpha class (`texture.alpha_class` → `mw_bake::scene::runtime_alpha`), so foliage, fences and
  grates enter the GPU table while glass and gauze stay sorted entities. A misclassified texture would show up
  as one surface drawn with the wrong edges.
- Kind: game location, A/B. Needs the S3b `--scene` path and a current `bevyrrowind bake` (start log:
  `alpha classes: N cutout candidates`). Runs: `view "Seyda Neen" --hour 14` and `view "Balmora" --hour 14`,
  each default and restarted with `--scene`.
- Look for: no missing vegetation, hedges, fences, grates or tree cards with `--scene` on; and no candidate
  surface with hard cutout edges where it used to fade softly (glass, gauze, cobwebs, smoke cards) — name the
  surface, which names the misclassified texture (`textures::alpha_is_cutout` thresholds).
- Status: owner 2026-11-11 (Balmora): no missing cutouts seen. Source: static-scene scene-fixes, defects 1 + 5.

### SC-scene: Statics drawn by the static scene (static-scene S3b C2, 2026-11-11)
- Why: `--scene` draws exterior statics from the baked GPU table (one batch per material/slab, slots retained) instead
  of entities + precombine. First time it runs in the game: anything wrong shows as missing, doubled or misplaced statics.
- Kind: game location, A/B. Needs a current `bevyrrowind bake` (log: `static scene: N cell tables`, `N chains`).
  Runs: `view "Balmora" --hour 14`, default then restarted with `--scene`; then walk/fly ≥ 300 m in one direction with `--scene`.
- Look for: every building, rock, fence and tree present with `--scene` (none missing, none flickering or z-fighting
  as a double draw); after crossing ~256 m nothing jumps or drifts (origin shift, O1); doors, containers, lights and
  scripted objects still entities (interactable); cutouts as SC-foliage. Log: no `sf_scene_bevy` errors, no `is full` warning.
- Status: owner 2026-11-11: "seems perfect": nothing missing or popping; a subtle flicker now and then, maybe shadows or fog
  (unattributed: compare in sc-noscene); felt faster, no FPS counter in the overlay to tell. Log: 883 `batch ... @exterior is
  full (4096 slots)` warnings (the overflow spawns as entities, so the batch is undersized for Balmora, and the warn-once
  doesn't hold). Logged, not fixed.
- Owner 2026-11-11, later in the same run: **foliage floating in the sky** when flying up; grass/foliage only, appearing and
  disappearing as a batch every ~200 m. Check sc-noscene (A/B) to tell if `--scene` causes it. Frame rate from the log (physics
  line, frames per 10 s): ~49-62, mostly 60 (vsync cap?); the stairs scene ran ~121 in the same build.
- A/B (sc-noscene, owner 2026-11-11): "exactly the same things", the short shadow flickers included, so the flicker and
  the floating foliage are not from `--scene` (log them as their own bugs). Frames per 10 s off: 90, 72, 69 (35 s run,
  different spots) vs on ~49-62: no faster on this evidence; the batch overflow above may explain it. Perf suite pending.
- Perf A/B 2026-11-11 (profiling exe, 1440p, DLSS Quality loaded, 3 runs each; benchmarks/runs/scene_off, scene_on):
  balmora_walk median 11.4-12.3 ms off vs 14.4-14.9 ms on (+~2.7 ms); balmora 9.3 ms off vs 11.2-11.8 ms on (+~2.1 ms).
  GPU busy 77-96% off vs 63-88% on (more CPU-bound). The on runs log 81 `batch ... is full (4096 slots)` warnings
  (overflow spawns as entities, so both paths run at once). Audit 2026-11-11 (traced, incl. 32k slots, no overflow:
  still +1.8 ms): per-frame O(ranges) CPU in the GS port + per-frame `extract_chain_slabs`; capacity is secondary.
  `superfargoth/docs/research/static-scene-perf-audit-2026-11-11.md`.

### SC-lamps: Lamp shadows of statics drawn by the static scene (static-scene fix 5, 2026-11-12)
- Why: the point-light shadow cache only knew `Mesh3d` casters; scene batches are now static casters (capture after the
  batch visibility, `ShadowCastersChanged` from cell stream-in/out and promotion recaptures nearby lamps).
- Kind: game location, A/B. Run: `scene sc-scene-night` (Balmora 22:00, `--scene`); A/B: `scene sc-noscene` then `[`/`]` to night.
- Look for: lamps cast shadows of buildings and walls, steady (no flicker or popping); walk into a new cell: its statics cast
  lamp shadows within ~2 s; shadows of cells left behind clear. Same shadows as without `--scene`.
- Status: awaiting owner.

### RO-18: Grass and foliage floating in the sky (owner report 2026-11-11)
- Seen in `scene sc-scene` and `scene sc-noscene` (Balmora 14:00), so not the static scene: fly up high and grass/foliage
  hangs in the air; it disappears and reappears as a batch every ~200 m of travel (grass or cell streaming, unknown).
- 2026-11-11 later (scene-audit): not reproduced. A scripted climb to 700 m and the owner's own flight (to 595 m, coast and
  open water, BRP capture of every new mesh) showed nothing floating; vertical origin shifts move all statics correctly.
  Intermittent. Capture tool: `node bevyrrowind/tools/brp_capture_meshes.mjs <out.jsonl> [port]`; rerun when it shows again.
- Status: known bug, not diagnosed. Next step: a diagnosis slice (which system spawns it: groundcover, statics, streaming).

### RO-19: Very short shadow flickers here and there (owner report 2026-11-11)
- Balmora 14:00, same in sc-scene and sc-noscene: brief, subtle flickers, the owner thinks shadows (maybe fog).
- Status: known bug, not diagnosed (see the shadows checks SH-* for related cases).
