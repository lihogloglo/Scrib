# Surface weather and deformable ground: reuse and plan (research, 2026-11-05)

Owner ask: deformable ground, ash/snow accumulation after weather, puddles after rain; "Rafael shaders" and Clockwork
as references; are there ready-made Bevy solutions? Research only (web + our docs). Items marked **(verify)** come
from memory, not from a fetched source.

## 1. Summary

- **No Bevy crate or open project offers any of it** (accumulation, wetness/puddles, trail deformation). Searches for
  Bevy snow/footprint/wetness found none; Bevy's own `ssr` example (ExtendedMaterial water ripples) is the only
  related shipped code. Terrain crates (`bevy_mesh_terrain`, `bevy-clipmap`, the bevy_terrain thesis mirror) do
  geometry/splat only, none has deformation or weather, and some are on old Bevy. We build; what is reusable is
  *technique and shader math*, ported from Clockwork (GPL-3.0, compatible), `bevy_atmos` (MIT/Apache, already
  audited) and the GDC literature.
- **"Rafael shaders" = Rafael's OpenMW Shader Pack** (Nexus Morrowind mod 53667 per community posts, **verify**
  number and author): volumetric fog/mist, AO, PBR lighting, rain and "wetworld" (wet ground). It is OpenMW
  post-process/shader-pipeline GLSL, not compute and not deformation. Add-ons build on it: a Seasons-linked set with
  snow accumulation by normal only (its author admits no occlusion, so NPC faces get snowy) and rain wetness. The
  pack's licence is unknown (**verify**): visual reference only, never port. The "compute shader" memory is probably
  a conflation; Clockwork's surface weather is fragment-shader based (our Clockwork note, row 5).
- **Most of it is cheap** (material-layer blends driven by a few scalars plus our existing occlusion map). Only
  trail deformation is real work, and it needs no tessellation (section 4).

## 2. What exists to reuse

| Source | What | Licence / fit | Verdict |
|---|---|---|---|
| Clockwork (Bevy 0.19 fork of OpenMW, OursCodeur): `lib/light/surface.glsl`, `puddles.frag`, `surfaceweather.cpp` | wet/puddle/dust/snow keyed to a top-down shelter map (`textureGather` tent), leaf shelter lattice, wall streaks, rain crowns/rings, optional screen-space puddle reflection, per-region scalars advanced in game minutes, drying 120 min (snow 3x, dust 4x), **not saved** | GPL-3.0, portable to our GPL-3.0-or-later with file+commit citation and a `THIRD_PARTY_NOTICES.md` entry; GLSL, so a WGSL/WESL rewrite | **Port shapes** of `surfaceCrowns`, `surfaceNoise`, `surfaceExposure` (already rated in the Clockwork note) |
| `bevy_atmos` `wet.wgsl` (109 lines) | albedo x0.55, roughness to 0.18, noise puddles on up-facing pixels, puddle roughness 0.03 | MIT/Apache | Technique, rewrite onto our `WorldMaterial` hook |
| Bevy `examples/3d/ssr.rs` | `ExtendedMaterial` + normal-map ripple extension; SSR needs depth prepass | MIT/Apache | Pattern for the extension; SSR itself parked (exterior camera has no depth prepass; borrow-audit) |
| `bevy_mesh_terrain` (Bevy 0.14/0.15 era), `bevy-clipmap` 1.0.4 (Bevy 0.18, geometry clipmap, 360 horizon maps, huge on disk), bevy_terrain mirror | terrain meshing/splat | Old or mismatched Bevy; we own `sf_terrain` (CDLOD + deformation stack) | Nothing to adopt |
| `bevy_app_compute`, `bevy_easy_compute`, Bevy "game of life" compute example | compute helpers | We already have render-graph passes; a stamp pass is ~100 lines in our plugin | Not needed |
| Unity "Production Ready Weather" Shader Graph sample | puddles on flat up-facing surfaces; wet = darker, more saturated, smoother | Unity sample licence | Concept only |
| Lagarde, "Water drop 3a/3b: physically based wet surfaces" and "Observe Rainy World" (2013) **(verify links)** | wet albedo darkening by porosity, roughness drop, thin water film | article math | **Canonical wetness model**; cite in the spec |
| Godot_rain_shader (Lexpartizan), Godotwind/ECSwind | Lagarde wet albedo | anecdote tier per CONTRIBUTING | reference only |
| Batman: Arkham Origins deformable snow (Barre-Brisebois, GDC 2014, GDCVault 1020379) | RT height stamp + relief mapping (console) + tessellation (PC) | talk | **Canonical**; the console path is our model |
| Rise of the Tomb Raider deferred snow deformation (Sikachev et al., GPU Pro 7 **(verify)**) | trail map, raised lips, tessellation | book chapter | lip-shape reference |
| RDR2 snow/mud, God of War (2018) trails, Horizon Zero Dawn, Assassin's Creed snow/wet | shipped members of the same family; no formal public talks confirmed by search | n/a | industry-comparison citations **(verify)**, not sources |

## 3. Industry technique per feature

**Accumulation (snow, ash, dust).**
- Canonical: a **world-space top-down occlusion ("shelter") map** rendered from the precipitation direction (height of
  the highest surface), sampled per pixel: exposed if surface height >= stored height minus bias. Plus
  `up = saturate(dot(N, up_dir))` with a smoothstep slope threshold, noise to break edges, and **height-blend**
  (accumulation wins where the material's own height/AO map is low, so cracks fill first). Coverage animates over
  time. This is what Clockwork does and what the Seasons add-on lacks.
- Canopy shelter: Clockwork uses a B-spline leaf lattice; our occlusion map already includes canopy meshes.
- Ash differs from snow in colour/roughness/sparkle and falls lightly on walls too.

**Puddles.**
- Fill by a **porosity/height field**: mask = smoothstep over (water level - ground height detail), so water sits in
  low points of the terrain/material height map; flat normal, roughness ~0.03, F0 up, Fresnel against the sky probe;
  rain ripple rings (two offset grids, Clockwork `rain_ripples`). UE wetness and Watch Dogs follow Lagarde's film
  model **(verify)**. Reflection ladder: sky/probe (cheap, default) -> planar for big pools -> SSR (parked).
- Wetness on all materials: albedo darkening by (1 - porosity), roughness lerp to ~0.2, streaks on walls.

**Deformable snow/ash/mud.**
- Canonical (Batman AO, ROTTR, GoW, RDR2): a **camera-centred toroidal render-target ring buffer** of height deltas
  (about 0.05-0.1 m/texel over 30-60 m **(verify sizes)**) into which feet/bodies/wheels are **stamped** as
  instanced quads/capsules with min/max blending. The terrain shader samples it for vertex displacement (coarse) and
  relief mapping/POM (fine) and derives lip normals from its gradient. Slow decay/refill (snowfall adds back) by a
  small pass. Nothing persists outside the ring unless baked out.
- Tessellation is the PC option, not a requirement (section 4).

**Persistence.** Accumulation is **simulated state**: per-region or 32 m-grid scalars `wet, puddle, snow, ash`
advanced on the fixed weather tick, saved coarsely, deterministic from (weather history, time) so a co-op client
reconstructs it (net-ready: visuals are presentation; only footing/ice would be gameplay and is host-side).
Clockwork does not save it; our owner note (2026-10-21/22) says persist. Trail deformation in the ring is cosmetic and
unsaved; only *persistent edits* (digging, burial) use the sparse-delta path of `specs/terrain.md`.

## 4. Cost on the target (RTX 4060 laptop, 1440p DLSS Quality, <= 11 ms High)

| Part | Class | Notes |
|---|---|---|
| Wet/snow/ash material layer | cheap, ~0.1-0.3 ms **(estimate; measure)** | tiled noise textures, not 6 gradient-noise calls per pixel (borrow-audit); forward, no extra pass |
| Shelter map | cheap | 1-2k^2 R16F/R32F around the camera, **updated incrementally** (scrolled strips or one slice per frame); occlusion map already exists (WE-5) |
| Puddle reflection: sky/probe | ~free | SSR est. >3 ms at 1440p (atmos/borrow-audit); parked |
| Rain ripples | cheap | procedural normals on puddle pixels only |
| Stamp ring buffer + decay | cheap | a few hundred instanced quads + one ~1k^2 pass |
| Vertex displacement from trail map | cheap-moderate | CDLOD gives dense patches near camera; displace only the finest 2 LODs (~0.1 m resolution) |
| POM / relief on terrain | moderate | 4-8 steps, near field, distance fade; keep UVs stable under DLSS/TAA jitter |
| Hardware tessellation | **unavailable** | wgpu has no tessellation or mesh shaders |

**Instead of tessellation in Bevy/wgpu:** (1) displace vertices of the finest CDLOD grid from the trail texture
(vertex texture fetch), lips come from the map; (2) relief mapping/POM in the fragment for sub-vertex prints, the
Batman console path; (3) prints on terrain only, so no generic mesh subdivision; (4) later option: compute-shader
refinement of near terrain patches. DLSS/TAA: displaced geometry must also be displaced in the depth/motion-vector
prepass (same function) or ghosting and depth mismatches appear; the shadow pass needs it too, or accept flat
shadows for 5 cm prints.

## 5. Where it plugs into our code

- `sf_weather`: `WeatherFrame.surface` already carries `wet, puddle, snow, ash, drying` (frame.rs:341).
- Occlusion map from WE-5 is the shelter map source (`specs/weather/precipitation-surface.md`, "Surface state").
- Not built: `sf_surface`, `surface_state.wesl`, the `WorldMaterial` hook in `sf_model_bevy`, the persistent grid.
- Terrain: `specs/terrain.md` deformation stack (ARCHITECTURE 7.4). WISHLIST sections 5 and 7 hold the look brief
  (RDR2: coarse vertex depth + POM, raised lip, packed-snow/wet-mud inside, kick-up particles, refill while it
  snows, depth per surface type).
- Game side: Morrowind weather names and ash/blight region textures live in `bevyrrowind`; the framework stays
  generic. ROADMAP Phase 4 "Weather polish" (WE-5, SU1, SU2, PR2).

## 6. Recommendation and slice plan

Adopt no dependency; port technique. Build in `sf_*`, one deliverable per slice, each with a registered visual-check
scene (awaiting owner; no agent screenshots).

| Slice | Deliverable | Depends on | Reuse |
|---|---|---|---|
| W1 | Surface state: scalars (wet, puddle, snow, ash) with rise/dry rates, fixed tick, coarse save; exact-assertion tests | weather state machine | Clockwork rates (120 min dry, snow 3x, dust 4x) as start values |
| W2 | Shelter map exposed to materials, incremental update | W1, WE-5 | Clockwork `surfaceExposure` shape |
| W3 | `WorldMaterial` surface layer on all materials: wetness (Lagarde darkening, roughness, porosity), wall streaks | W1, W2, `sf_model_bevy` | Lagarde, atmos `wet.wgsl` |
| W4 | Puddles on terrain: height/low-point fill, ripples, sky/probe reflection | W3, terrain height/normal maps | Clockwork `surfaceNoise`/rings |
| W5 | Snow/ash accumulation: normal*up, height-blend, region ground textures for ash | W3, W2 | Clockwork dust path |
| W6 | Trail deformation: ring buffer, foot/body stamps, vertex displacement + POM, lips, refill, depth per surface type | W5, terrain deformation stack, prepass/motion-vector parity | Batman AO console path, ROTTR lips |
| W7 | Kick-up particles and splashes (PR2) | W6, bevy_hanabi crate batch | n/a |
| later | SSR puddles; persistent dig edits; character wetness (waterline) | depth prepass, water query API | Bevy ssr example |

Order logic: W1-W3 deliver "wet world after rain" (the owner's cheap-puddles-first decision); W4/W5 puddles and snow;
W6 is the only expensive/risky slice and goes last. Each ships independently.

## 7. Owner decisions needed

1. Confirm "adopt nothing, port technique", including porting Clockwork shapes under GPL with notices.
2. Persistence granularity: per-region scalars (cheap) vs 32 m grid (spec). Suggest 32 m grid for snow/ash, per-region
   for wetness.
3. Reflections: sky/probe only now with SSR parked until a depth prepass exists (suggested), or prioritise the prepass.
4. Trail scope: player + NPCs + creatures, terrain only (suggested); ring radius (~40 m) and resolution; displaced
   shadows or flat.
5. Accept the prepass/motion-vector displacement parity requirement for terrain before W6 (DLSS ghosting).
6. Ash: also on walls (lightly) or ground only; may ash and snow layers coexist.
7. Lit-look rule (2026-11-05): wetness never applies indoors; A/B wet albedo darkening in lit towns before shipping.

## Sources

- Rafael's OpenMW shader pack discussion: https://reddit.birdcat.cafe/r/Morrowind/comments/1fphlk0/openmw_rafaels_shader_pack_appreciation_post ;
  add-on requiring it: https://www.nexusmods.com/morrowind/mods/59485 ; related: https://www.nexusmods.com/morrowind/mods/59113 ,
  https://www.nexusmods.com/morrowind/mods/59475 ; OpenMW issue: https://gitlab.com/OpenMW/openmw/-/issues/8864
- Batman: Arkham Origins deformable snow (GDC 2014): https://gdcvault.com/play/1020379/Deformable-Snow-Rendering-in-Batman ;
  slides https://www.slideshare.net/colinbb/gdc2014-deformable-snow-rendering-in-batman-arkham-origins
- Bevy ssr example: https://docs.rs/bevy/0.17.1/src/ssr/ssr.rs.html
- bevy-clipmap https://docs.rs/bevy-clipmap ; bevy_mesh_terrain https://docs.rs/bevy_mesh_terrain ; bevy_terrain mirror
  https://github.com/RustWorks/bevy_terrain_map ; compute helpers https://docs.rs/crate/bevy_app_compute/latest
- Unity weather sample: https://docs.unity3d.com/Packages/com.unity.shadergraph%4017.0/manual/Shader-Graph-Sample-Production-Ready-Weather.html
- Godot rain shader: https://github.com/Lexpartizan/Godot_rain_shader ; wet puddle gist:
  https://gist.github.com/eviltak/e04b83ffdb91aa3d477bbfe0ca370da7
- Snow with compute shaders (AAU): https://vbn.aau.dk/da/publications/real-time-interactive-snow-simulation-using-compute-shaders-in-di
- Internal: `research/clockwork-code-2026-10-25.md` row 5, `research/borrow-audit-2026-10.md` "Rain puddles and wet
  surfaces", `WISHLIST.md` sections 5 and 7, `specs/weather/precipitation-surface.md`, `specs/terrain.md`.
