# Wishlist → design

The target, in the user's words: *"Morrowind if Bethesda made it in 2026",
modernized like Godotwind, but very, very fast. Crimson Desert level.*

This file turns each wish into a system, the canonical technique it calls
for, and the decisions it forces **early** (things that change data models
and are expensive to retrofit). Detailed designs go into specs
(`docs/specs/`) before each system is built. Milestones: ARCHITECTURE.md §9.

**Calibration.** Crimson Desert is the work of a large studio over many
years, mostly in content. The engine features below are achievable; matching
its *content* density is an art problem we attack with procedural generation,
bake-time tooling, and mods, not code alone. The bar we hold ourselves to is
its technology and feel, not its asset count.

**Reference policy: clean room.** Game mechanics, timings, and formulas
aren't copyrightable; code and asset data are. When we study another game
(Dark Souls 3 for combat), we use a clean-room split:

1. **Analysis sessions** observe play, read the game's data files from the
   user's own copy (DS3 keeps most combat behavior in TAE animation events
   and param tables, documented by the modding community), and use REA on
   the executable only for behavior that lives in code (lock-on, input
   buffering, hit sweeps, stagger states). They produce **behavior specs**
   in `docs/specs/combat/`: mechanics, state machines, timing windows,
   formulas, value ranges. **Never** code, pseudocode, decompiled
   identifiers or structures, or raw extracted tables.
2. **Implementation sessions** read only those specs, never analysis notes
   or decompiled output.
3. **Our numbers are our own**: tuned from the spec's ranges, never whole
   copied tables.
4. **Analysis notes never enter a repo.** They stay in a local scratch
   folder.

The clean room protects what we ship; it doesn't make the analysis itself
lawful. Decompiling for non-interoperability purposes and bypassing
anti-tamper protection carry legal risk that depends on jurisdiction and
the game's EULA, and that's the project owner's call. Morrowind.exe analysis
for our own content pack follows the same split.

---

## 0. Performance is a feature

"Very, very fast" needs numbers, or it can't be tested. Proposed targets
(adjust as hardware data comes in):

| Scenario | Target |
|---|---|
| Raster path, 1440p, RTX 3070-class | 120 fps in exteriors, 144+ in interiors |
| Solari path, 1440p, RTX 4070-class + DLSS | 60 fps |
| Worst frame while streaming (cell crossing, fast travel) | < 1.5× median frame time, no frame > 33 ms |
| Cold start to playable | < 10 s after first bake |

How: GPU-driven rendering and bindless (Bevy-native), aggressive LOD/HLOD/
impostors (§1), texture streaming (§10), frame-budgeted streaming (§2), and
the benchmark harness (`sf_diag`) gating every milestone: a regression is a
bug.

## 1. LODs, HLOD, impostors

- **Per-object LODs** generated at bake time with meshoptimizer
  (simplification + vertex cache/overdraw optimization). Bevy picks LODs by
  screen size (`VisibilityRange`).
- **HLOD**: static geometry merged per cell cluster (1×1, 2×2, 4×4 cells),
  materials merged into atlases or array textures, simplified per level.
  Baked, streamed as one draw per cluster per level.
- **Impostors**: octahedral impostors for far trees and landmarks (proven in
  Godotwind), rendered as GPU-instanced quads.
- **Occlusion culling**: Bevy's GPU occlusion culling (two-phase HZB) in dense
  cities.
- Distance tiers come from the streaming system (§2), not per-system guesses.

*Early decision:* the bake pipeline (`sf_bake`) is a first-class tool with
content-hash caching, per mod set. Every expensive derived asset (LODs,
HLODs, impostors, navmesh, terrain tiles, PBR maps) goes through it.

## 2. Huge worlds: streaming and coordinates

"Let's make Tamriel to scale" means hundreds of kilometers. 32-bit floats
lose centimeter precision around 100 km, and physics jitters well before.

- **Coordinates: grid cells + floating origin** (`big_space`: integer grid
  cell + local `Transform`, MIT/Apache, Bevy 0.19, typically ported to new
  Bevy within weeks). Positions in saves and records are (cell, local),
  never a raw f32 world position.
- **Streaming**: world partition into streaming cells independent of source
  "cells". Rings of detail: full gameplay → static visuals → HLOD →
  impostors/terrain-only. Priority by distance, view direction, and
  velocity (prefetch along travel). Frame budgets per phase (spawn, upload,
  physics bake).
- **Terrain at scale**: chunked heightfield with a GPU clipmap/CDLOD
  renderer, so view distance is independent of loaded gameplay cells.

*Early decision (needed in M2):* adopt grid coordinates from the first
exterior commit. Retrofitting coordinates later touches every system.

## 3. Terrain: higher detail, erosion

- Source heightmaps (Morrowind: 65×65 per 117 m cell, ~1.8 m spacing) are
  **upsampled at bake time** to ~0.25–0.5 m, then refined with **hydraulic +
  thermal erosion** (GPU, at bake time; Godotwind's erosion as reference)
  constrained to keep placed objects grounded.
- Runtime: clipmap terrain, height-blended splatting over array textures,
  triplanar on cliffs, macro variation, detail normal maps, optional
  displacement close to the camera.
- Terrain is a stack: base height + erosion detail + persistent edits
  (§7) + transient deformation (§7). Every consumer (render, physics,
  navmesh, grass) reads the same stack.

## 4. Water: ocean, water at any height, rivers

- **One water model**: water bodies with types (ocean, lake, river, pool),
  each with a surface definition, priority, and optics; all sampled through
  one query API (height, normal, velocity, depth, body id) shared by
  rendering, buoyancy, swimming, wetness, and underwater effects. This is the
  Godotwind `WaterSurfaceState` contract, which was the right part.
- **Ocean**: FFT cascades (Tessendorf) on a camera-centered clipmap, foam,
  SSS, refraction, shore interaction, underwater. **Evaluate `bevy-aqua`
  first** (FFT + Crest-style waves, cascades, shoreline foam, lakes and
  river corridors at their own heights, GPU wave queries for buoyancy; Bevy
  0.19, early 0.1.x). Adopt or fork if it holds up; build our own otherwise.
  Avoid Godotwind's pain points: one owner per water concern, buoyancy from
  a CPU evaluation of the low-frequency cascades (deterministic, no GPU
  readback stalls), no per-frame CPU rebuild of water-coverage atlases.
- **Lakes and pools at any height**: polygon-bounded bodies with their own
  level and optics (Morrowind only has sea level; the framework doesn't).
- **Rivers that run to the sea**: generated at bake time from terrain
  hydrology (flow accumulation over the eroded heightfield, Godotwind's
  hydrology as reference) as spline/ribbon water bodies with flow maps,
  ending in the ocean. Shader-advected normals and foam; no runtime fluid sim.

## 5. Weather and surfaces: wetness, puddles, snow, ash

One **surface state** system, not one hack per effect:

- World-space accumulation maps around the camera (wetness, puddle level,
  snow depth, ash depth) driven by weather, time, and temperature, and
  persisted coarsely for the whole world.
- **Occlusion**: a top-down depth map from the precipitation direction, so
  nothing gets wet or snowy under roofs and trees.
- Materials respond through a shared shader layer: wetness darkens albedo
  and lowers roughness; puddles fill height-field low points (from
  normal/height maps); snow and ash accumulate on upward-facing surfaces with
  height blending and displace terrain via the deformation stack (§7).
- Characters carry their own wetness (rain exposure, water immersion
  height from the water query API): wading in a river wets legs up to the
  waterline, then dries over time.
- Weather state machine with blended presets (rain, storm, snow, ash storm,
  blight) driving clouds, fog, particles, wind, and accumulation.

## 6. Seamless interior/exterior transitions

Morrowind interiors are separate spaces, usually larger than the building
outside, with their own ambient light and fog.

- **Portal-linked worldspaces**: an interior keeps its own space; its door is
  linked to the exterior door by a transform. The door frame is a portal:
  each side renders the other through it (render-to-texture or a stencil
  pass), so you can look in from the street and out from inside.
- **Crossing**: when the player passes the portal plane, they're
  re-parented into the other space through the link transform; no load
  screen, no fade. Both spaces are streamed in around a door before it
  matters.
- **Environment blending**: ambient, fog, exposure, and reverb live in
  environment volumes that interpolate across the threshold instead of
  switching.
- Size mismatch ("bigger on the inside") is handled naturally: the spaces
  never overlap.
- Exterior visibility from inside (windows) uses the same portal mechanism,
  with authoring hooks for mods that add real window portals.

*Early decision (needed in M2):* streaming works per worldspace, with
door links as first-class records, not per-cell teleports.

## 7. Physics: rigid bodies, ragdolls, cloth, deformation

- **Rigid bodies** (Avian/Rapier): loose items physical once disturbed,
  carry/throw, buoyancy from §4.
- **Ragdolls**: generated per skeleton from bone shapes and joint limits;
  powered (active) ragdolls for hit reactions and get-up blending.
- **Cloth**: XPBD cloth on GPU (or CPU for the player) for capes, robes,
  banners. Simulated proxy meshes skin the render mesh; colliders from the
  ragdoll capsules. Morrowind robes and skirts get proxy meshes at bake time.
- **Terrain deformation**: transient trails (GPU stamp ring buffer) and
  persistent height edits (sparse deltas in saves), as in ARCHITECTURE §7.4.
  Look (owner 2026-10-25, RDR2 reference): trail map drives coarse vertex depth (dense CDLOD levels) plus
  parallax occlusion for crisp prints; a pushed-up lip with its own normals around each print; packed-snow /
  wet-mud material inside; kick-up particles (snow puffs, ash, mud) on footfalls; prints refill while it snows.
  Depth per surface type (powder, ash, mud, none on stone).

## 8. Animation

- Layered animation graph: locomotion blend spaces, upper-body masks,
  additive layers (breathing, hit reactions), root motion.
- **Motion matching** for locomotion once we have a mocap library (Godotwind
  already explored a universal animation library and retargeting); blend
  trees until then.
- IK: two-bone (limbs), foot placement with pelvis adjust, look-at/aim,
  hand IK for interactions and weapons.
- Procedural layers: lean into turns, footstep-driven secondary motion,
  physics-driven hair and cloth (§7).
- Retargeting: Morrowind's Bip01 skeleton maps to a standard humanoid rig,
  so modern animation sets apply to vanilla bodies.

## 9. Combat, inspired by Dark Souls 3

- **Animation-driven**: every action is an animation with authored windows
  (startup, active hitbox frames, recovery, cancel windows, i-frames) stored
  as animation events/tracks.
- **Resources**: stamina for attacks, rolls, and blocks; **poise**
  (hyperarmor during heavy swings, stagger on break); HP. Morrowind's
  fatigue maps onto stamina naturally.
- **Hit detection**: weapon sweep volumes per active frame (swept capsules
  between frames), hitstop, directional hit reactions, parry → riposte,
  backstab windows, guard break.
- **Movement**: lock-on, dodge roll with i-frames, sprint, jump attacks;
  camera tuned for melee.
- **Data-driven movesets** per weapon class, so mods can add weapons and
  move sets without code. Morrowind's stats drive damage/scaling formulas
  through the rules plugin; no more invisible dice rolls on a hit that
  visibly connected.

## 10. Textures: PBR and streaming

- **PBR generation at bake time** for legacy textures: albedo de-lighting,
  height/normal from luminance with per-material tuning, roughness and
  metalness from material class (wood, metal, stone, cloth). Replacement
  packs override through the VFS (§6.1 of ARCHITECTURE).
- **Texture streaming**: block-compressed (BCn) textures with mip residency
  driven by screen-space demand under a VRAM budget; virtual texturing for
  terrain. Bevy has neither yet; this is our engine work, and a good
  candidate to upstream.

## 11. Foliage look (later; owner 2026-10-19, wishlist only)

- **Light through leaves**: the owner asked for "SSR in the leaves", read as
  subsurface scattering / diffuse transmission (sun glowing through foliage
  from behind); confirm before designing. Check Bevy's
  `StandardMaterial::diffuse_transmission` first.

## 12. Water look (later; owner 2026-10-19, round 12 of the visual pass)

- **Crest subsurface scattering**: light glowing through thin wave tips
  (the usual ocean "SSS" term on top of water.md's volume scattering).
- **Shore that lives**: swash (water running up and down the beach), wavelets
  over the first ~20 m growing gradually into open-sea waves, never a flat band;
  foam with texture, breakup and varying width, not a fixed white line.
- **Whitecaps** on crests (water.md: Jacobian foam), scaled by wind/weather.

## 13. Object-terrain blending (later; owner 2026-10-20, eye candy, low priority)

- Rocks, roots and foundations take on the terrain's albedo/normal where they meet the ground, the way the
  Skyrim SE "terrain blending" mods do (nexusmods.com/skyrimspecialedition/mods/147616), not a darkened band.
- The first attempt (`sf_terrain_bevy::ground_blend`) only darkened the base and cost 10-13 ms of render thread
  on the Balmora walk (non-batched per-part materials); removed 2026-10-20. Look for a Bevy crate first.

---

## Early decisions this list forces

These change data models, so they're settled before or during M2:

1. **Grid coordinates** (§2) for every world position, from the first
   exterior commit.
2. **Worldspaces + door links** (§6) as the streaming unit; interiors are
   worldspaces, not special cases.
3. **Terrain as a layered stack** (§3, §7) that every consumer reads.
4. **One water query API** (§4) before any water-dependent feature.
5. **Surface state** (§5) as a shared material layer, so every material
   gets wetness/snow/ash from day one instead of retrofits.
6. **Bake pipeline** (§1) with content-hash caching as the home of all
   derived data.
7. **Performance targets** (§0) wired into `sf_diag` and checked per milestone.
