# ARCHITECTURE section 7: original feature sketches

Moved verbatim from [ARCHITECTURE.md](../ARCHITECTURE.md) section 7 (doc cleanup S14, 2026-10-26). Current designs live in the specs.

### 7.2 Ocean and water

- **Surface:** Tessendorf FFT with several cascades in WGSL compute
  (concepts from GodotOceanWaves, which is MIT), on a camera-centered clipmap
  mesh. Jacobian foam, subsurface scattering, refraction, screen-space
  reflections.
- **Shoreline:** a distance field baked from terrain against the water
  plane drives wave damping, shore waves, foam lines, and wetness (lessons
  from Godotwind's `shore_overhaul.md` and the ocean audits).
- **Underwater:** absorption (Beer-Lambert), caustics, and correct handling
  of a camera split across the waterline.
- **Buoyancy:** physics needs heights on the CPU. Evaluate the
  lowest-frequency cascade on the CPU (cheap, deterministic) rather than
  reading back from the GPU. Bodies sample a few points per hull.
- **Later:** rivers with flowmaps; lakes at arbitrary heights (Morrowind has
  only one water plane, the framework won't).
- **Specified 2026-10-13** in [specs/water.md](../specs/water.md) (M6): bevy-aqua
  evaluated and ported rather than adopted; SSPR instead of SSR for water
  reflections (the exterior camera has no deferred prepass); rivers as baked
  flow; pontoon buoyancy (§10 2026-10-13 m6-spec).

### 7.3 Rigid-body physics

- **Rapier** (`rapier3d` behind `sf_physics`; was Avian, changed 2026-10-12, §10), on a fixed timestep with transform interpolation (the lesson
  of Godotwind's "carry vibration" bug).
- **Static world:** colliders baked from NIF collision nodes when present,
  otherwise from convex decomposition at bake time. Tile-level heightfields
  for terrain.
- **Dynamic world:** loose items (misc, weapons, armor, ingredients) are
  real rigid bodies once disturbed. They start asleep so placed scenes
  stay authored. Physics-driven carry and throw.
- **Ragdolls:** death and knockdown hand off from animation to a physics
  ragdoll and back via blended get-up.
- **Streaming:** bodies in unloading cells freeze and serialize their
  transforms. Nothing simulates outside the active radius.
- **Buoyancy** via §7.2 queries.

### 7.4 Terrain deformation

Two layers, because they have different lifetimes and costs:

1. **Transient surface deformation** (snow, mud, ash, sand trails): GPU
   stamp render target in a ring buffer around the camera, with time-based
   recovery. It affects terrain shading and displacement plus grass bending.
   Render-only, no collision changes. Nearby trails persist briefly in saves.
   This is the Godotwind RTT design, generalized.
2. **Persistent height edits** (craters, digging, landslides, construction):
   sparse per-tile delta heightfields stored in the save overlay. An edit
   rebuilds that tile's physics heightfield, navmesh tile, grass masks, and
   GPU heightmap region.

Both live in `sf_terrain` as a deformation stack that every consumer
(rendering, physics, navigation, foliage) reads, so they never disagree.

### 7.5 Lighting and rendering

- **Solari** is the high-end path; **raster** (clustered forward, cascaded
  shadows, contact shadows, SSR, volumetric fog) is the required fallback.
  It covers non-RT GPUs, and Solari is still experimental.
- **Solari gap:** no point or spot lights yet, and Morrowind interiors are
  lit almost entirely by point lights. Interim fix: each light gets an
  emissive proxy mesh, which Solari handles, while raster lights stay for the
  fallback path. Drop the proxies when Solari ships punctual lights.
- **Bindless** comes free from Bevy's material system. Material dedup
  (a big Godotwind win) is natural there.
- **Missing in Bevy: texture streaming.** Vanilla Morrowind fits in VRAM;
  HD texture packs won't. Plan a mip-residency system (load low mips, stream
  higher mips by screen-space demand) once HD packs are being tested.
- **Old assets:** 2002 diffuse-only textures look flat under PBR. The
  adapter generates conservative normal and roughness maps at bake time, and
  HD replacer mods slot in via §6.1.

### 7.6 Animation

Morrowind's `.kf` files and text keys convert into Bevy `AnimationClip`s
plus animation events. On top of that: a locomotion layer (blend spaces), an
upper-body action layer (masks), additive hit reactions, and an IK pass
(foot placement on terrain and stairs, look-at, hand IK for doors and
levers). The framework never assumes Bip01 naming; skeleton conventions
come from the adapter.

