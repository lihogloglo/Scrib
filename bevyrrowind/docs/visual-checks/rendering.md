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

### RO-12: Bright outline around characters (owner report)
- Kind: game location. Seen 2026-10-30 from the release start (Seyda Neen), on NPCs and the player.
- Look for: a thin halo, brighter than the surroundings, along character silhouettes.
- Suspects (not tested): TAA/DLSS disocclusion on skinned meshes (motion vectors), GTAO or contact-shadow edge, fog
  froxel edge. First step: F4 (temporal mode) and `--off` A/Bs at the same spot.
- Status: known bug: cause not diagnosed (owner pass 2026-10-30).
- Owner 2026-10-30, `scene wk-balmora`: no outline around characters there; seen only from the release start (no
  subcommand). Clue: compare its settings (DLSS mode, preset, resolution) with `scene` runs.

### RO-13: F7 wireframe drawn away from the model (owner report)
- Kind: synthetic. Any anim-lab level, dev build, F7 (wireframe over skinned meshes; `mw_devtools_bevy/src/live_ab.rs`).
- Seen 2026-10-30: the wireframe seems to exist but shows at a different place than the character it belongs to.
- Suspect (not tested): the wireframe copy isn't placed in the same big_space grid cell / transform as the skinned mesh.
- Status: known bug: not diagnosed (owner pass 2026-10-30).
- Owner 2026-10-30 (Balmora, release copy): in the game F7 shows the wireframe on the terrain only, none on characters.
