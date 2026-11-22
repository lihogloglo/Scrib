# Groundcover: industry standard and Bevy crates (research, 2026-11-22)

Research for the parked ROADMAP item "Groundcover" (owner 2026-11-22: procedural in the long run for the worldgen
suite, with a way to say "this species here"; don't reinvent). Two parts: web research on shipped games, and a
source read of the Bevy crates (shallow clones, nothing built). Unverified = secondhand or not in a primary source.

## 1. What shipped games do

**One placement model, several render kinds.** A *species* is a data record (render kind, spacing, slope/height/
water constraints); its density is a function of terrain-layer or biome weights; instances are not stored but
scattered at runtime per tile around the camera (GPU, jittered grid), culled and drawn instanced; designer control
is painted density maps, exclusion volumes/splines and priorities on top.

- **UE5 Landscape Grass Type:** a `LandscapeGrassType` = list of varieties (mesh, density, jittered grid, rotation,
  per-variety cull distances, shadow flags), driven by a landscape paint-layer weight in the landscape material.
  Texture-driven, so cheap at huge counts. [Grass quick start](https://dev.epicgames.com/documentation/en-us/unreal-engine/grass-quick-start-in-unreal-engine),
  [API](https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/Landscape/ULandscapeGrassType).
  Procedural Foliage Spawner (CPU, species competition, stored instances) is the other, heavier path (forum, unverified).
- **UE5 PCG Biome Core (production-ready 5.7):** a biome (volume, spline or texture) has a priority and a weighted
  asset list; generators emit points with `$Density`; global pass resolves priority and overlaps, spawns ISMs.
  Overrides: `BiomeExclusion` volumes/splines, `BiomePath`, per-asset overrides, injected points.
  [Epic reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-pcg-biome-core-and-sample-plugins-reference-guide-in-unreal-engine).
- **Horizon Zero Dawn (Decima, GDC 2017, van Muijden):** artist node graphs output one 2D density map per asset,
  multiplied by road/water/object masks; footprint per asset (undergrowth ~1 m, trees ~6 m); 64x64 evaluation grid per
  block (32 m grass, 128 m trees); world data ~4 MB/km², generated then paintable, painted maps override; went from
  baked to fully runtime; ~250 µs GPU for placement; 500+ asset types.
  [Slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf).
- **Ghost of Tsushima (GDC 2021, slides not read; secondhand, unverified):** compute shader per tile: grid + jitter,
  grass type and height from terrain textures, frustum/distance/occlusion cull; procedural Bezier blades (15/7 verts),
  far tiles 2x larger at the same count, farthest = a texture on the terrain; Voronoi clumps vary height/colour/lean.
  **Flowers and pampas are separate artist meshes, GPU-instanced.** Wind = global noise + per-blade sine; displacement
  buffer for trampling; shadows from raised-terrain imposters + screen-space shadows. ~2.5 ms (secondhand).
  [write-up](https://tigerabrodi.blog/grass-in-ghost-of-tsushima).
- **Skyrim (Bethesda):** a land texture (LTEX) lists grass records (GRAS): model, density, min/max slope, distance
  from water, position/height/colour ranges, fit-to-slope. Same model as UE grass types.
  [UESP GRAS](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/GRAS).
- **OpenMW groundcover (Morrowind mods):** `groundcover=` plugins are ordinary static references from ESPs, drawn
  instanced with a wind shader, density and distance settings (default 6144 units). Hand-placed meshes, no species
  model. Morrowind has no GRAS-like record. [Docs](https://openmw-vr.readthedocs.io/en/latest/reference/modding/settings/groundcover.html).
- **Unity terrain details:** textured quads or instanced meshes, painted per-detail density maps.
- **Far Cry 5 (GDC 2018):** Houdini biome tools, species competing per environmental conditions (secondhand).
- RDR2, Cyberpunk: nothing citable found.

**Blades vs meshes.** Tsushima is the clear case of both in one game: procedural blades for the carpet (no mesh asset,
geometry built in the shader), instanced artist meshes for flowers and hero plants. UE, Unity and Horizon place meshes
only. Placement is shared; the render kind is a per-species property.

## 2. Bevy crates (source read)

| | bevy_grass 0.20.6 | bevy_feronia 0.9 | bevy_meadow 0.1 | bevy_foliage_tool 0.16.2 |
|---|---|---|---|---|
| Bevy | 0.20 | 0.19 | Bevy `main` (git) | 0.16 |
| Licence | MIT | MIT OR Apache-2.0 | MIT | MIT |
| Last commit | 2026-11-08 | 2026-08-20 | 2026-10-28 | 2025-08-12 |
| Several species | up to 16 per style, share of field density only; one field per species for own density | one layer per species, own density | one blade type per biome variant, one active, 100-200 MB VRAM each | one layer per mesh |
| Custom meshes | no (non-goal) | yes, with LODs | no | yes |
| Placement | GPU, hash per 16 m chunk, nothing stored | CPU per chunk, GPU cull TODO | CPU Poisson patches, GPU blades | CPU painted |
| Motion vectors | exact | yes | single-sample only | not checked |
| Tests | ~308 + GPU/golden | ~37 | ~82 | 0 |
| Readiness | pre-release, polished, active | "early, not for production" | early, tracks Bevy main | dead |

**bevy_grass** (`src/style.rs:19` `MAX_SPECIES = 16`; `docs/styles.md:208-222` "a species cannot have its own
density"; `docs/roadmap.md` non-goals: splatmaps, blending styles, scattering arbitrary meshes). Each `GrassField` has
its own density mask (R = density, G = height; `src/api.rs:52-66`) or `with_density_from_fn`, overrides (tint,
density/height/wind scale), runtime edits (`GrassEdit`, `GrassBrush`: clear, paint, mow). Fields overlap by design.
Ground: plane, heightmap (≤ 4096 texels per side), baked mesh or `with_ground_from_fn`. 100 m radius at High, 3 LODs,
`FarGrassMaterial` past the last blade, 4M instance cap per camera, 3 indirect draws per camera. External wind
(`GrassWindFrame`), trampling, directional shadow (Off/Near/Full), deferred + prepass. Minimal Bevy features (check
against hakari). Not checked: per-field cost of many overlapping fields, floating-origin behaviour (transform-based,
probably fine).

**So "one species with bevy_grass"**: yes, a field with one style = one blade species; N species at their own densities =
N overlapping fields, each with its mask. It only draws blade-like plants (grass, fern-like and flower-like blade
presets), never a Morrowind model.

## 3. What this means for us

The industry pattern maps onto our stack as three layers:

1. **Species and rules (ours, Bevy-free, data):** a species record (render kind blade|mesh, model or blade preset,
   spacing, slope/height/water constraints, colour/scale ranges) and a rule table "material/biome field → species +
   density", like Skyrim's LTEX→GRAS and UE's grass types. For Morrowind the adapter ships that table as data (Morrowind
   has no GRAS), keyed on LTEX; worldgen reads the same table from its material fields (WG-N3). Overrides on top: painted
   density, exclusion (roads, buildings, water), "only this species here" regions, priorities (UE PCG, Horizon).
   Nothing found on crates.io does this layer; it is small and engine-agnostic.
2. **Blade renderer:** `bevy_grass`, one field per blade species per tile, density from our rules through
   `with_density_from_fn` or baked masks, wind bridged from `sf_wind`.
3. **Mesh renderer:** runtime GPU scatter of species models into instanced draws (Tsushima's flowers, UE grass types,
   Horizon). Candidates: our own on Bevy's GPU instancing / the vendored `GpuInstanceBatchPlugin` the static scene
   already uses (check whether a compute pass can fill its slots), or port ideas from `bevy_feronia` (0.19, CPU
   scatter). Morrowind `groundcover=` mods feed this path as explicit placements; their models can also be species in
   the rule table.

Open for the spec (G0): cost of overlapping bevy_grass fields on the 4060 at High 1440p + DLSS Q; tiling and streaming
of fields over a multi-km world (4096-texel cap); whether the mesh path reuses the static scene's batches; budget split
(Horizon placement 0.25 ms; Tsushima ~2.5 ms secondhand; target well under 1 ms on High); shadow range; Solari
(bevy_meadow is the only crate with RT casters).
