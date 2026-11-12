# Bevy open-world projects: how they stream and render (2026-11-12)

Method: `gh search repos` plus shallow clones in `C:\cargo-target\refs\` (code read there; paths below are relative
to each clone). Skipped as already surveyed: bevy_city / many_cubes / many_foxes, Mudcrab, Tiny Glade.
Headline: **no Bevy project we found publishes frame times for a large open world with many static meshes and
many NPCs.** Scrib's perf work is ahead of most of what is public; what we can borrow is mostly techniques.

| Project | Licence | Bevy | Last push | What it is | Mesh path |
|---|---|---|---|---|---|
| rust-gta-3 (imbevystar) | NOASSERTION (custom) | 0.20 | 2026-10-14 | GTA III reimplementation, 261k lines | entity per IPL instance, custom unlit material |
| evroon/bevy-open-world | GPL-3.0 | 0.19 | 2026-10-30 | quadtree terrain, OSM buildings, ADS-B, flight sim | one entity per chunk, merged meshes |
| mate-h/bevy_terrain | Apache/MIT | 0.20 | 2026-11-03 | streamed planetary/planar terrain (thesis-based) | custom render phase, GPU tile refinement, indirect draw |
| bevy_voxel_world (splashdust) | Apache/MIT | 0.20 | 2026-11-10 | chunked voxel world plugin, multi-LOD | entity per chunk, standard Mesh3d + material |
| riverbed (Inspirateur) | MIT | 0.20.1 | 2026-11-08 | Minecraft-like, 2 km render distance | entity per chunk face, own CPU culling |
| big_space (aevyrie) | Apache/MIT | 0.19 | 2026-10-07 | floating origin, integer grids | Transform propagation only, no rendering |
| bevy_meadow (pavlov-net) | MIT | main | 2026-10-28 | GPU-driven grass | compute cull + `draw_indexed_indirect`, optional mesh shaders |
| Ethertum | GPL-3.0 | 0.19.1 | 2026-10-22 | voxel multiplayer survival, renet, Solari feature | entity per chunk, own DDGI GI plugin |

Not useful for our question: HirundoDev/daggerfall-bevy-native (MIT, Bevy 0.20, 2.5D, data-fidelity work, no
streaming or culling code), dust-engine/dust (own Vulkan renderer using Bevy ECS only, own Bevy fork).
No Morrowind/OpenMW/Skyrim data port on Bevy besides Mudcrab was found.

## rust-gta-3 (closest to Scrib: an old game's data on Bevy)
- Streaming is the original game's: `crates/rust_gta_3_dev_arena/src/scene.rs:440` calls the re3-style
  `update_streaming_around_positions`, then `:490-535` builds a per-frame render list (`RendererPassPlan`) from
  every world entity (`:458-486` iterates all of them every frame on one thread) and diffs it against the existing Bevy entities.
- Spawn model: one Bevy entity per IPL instance (`ArenaWorldMapEntity`, `scene.rs:556`), LOD swapped by
  despawn and respawn when the distance band changes (`:527-545`). One extra entity per material submesh
  (`crates/rust_gta_3_game/src/renderer/BevyRender.rs:2024-2060`). Retired mirrors are kept hidden and reused:
  the comment at `:2037` says a mass despawn/spawn storm caused a visible flash on scene reset.
- Custom material only (not a custom pipeline): `RendererMaterials.rs:622-650` `impl Material for
  GtaUnlitMaterial` with `enable_shadows() = false`, `enable_prepass() = false`, WESL shaders in
  `assets/shaders/gta/`. Reason: GTA III is baked/unlit, no PBR. Shadows are projected blobs
  (`scene.rs:302` comment), no cascades.
- No frame times in README. Takeaway: a 1:1 game port runs fine because GTA III has few hundred visible objects;
  it does not tell us about Morrowind-scale towns.

## evroon/bevy-open-world
- `crates/bevy-terrain/src/quadtree.rs:152-156` CPU quadtree subdivided by distance (`should_subdivide`,
  `max_lod: 24` at `lib.rs:40`); each node becomes a chunk entity with elevation and raster images loaded by tile.
- `crates/bevy-osm`: OSM vector tiles (MVT) tessellated on the CPU into one mesh per fill/stroke
  (`mesh.rs:114,140`), so buildings are merged per chunk, not per building. Chunk cache on disk (`cache.rs`).
- Only perf tooling is an egui frame-time/chunks-loading plot (`performance.rs:14-50`; turns red above 300
  chunks loading). No numbers published. README says the crates are demos, not libraries.

## mate-h/bevy_terrain (fork of kurtkuehnert's, now Bevy 0.20)
- Most advanced renderer here. Not the Mesh3d path: its own `TerrainItem` phase and `queue_terrain`
  (`src/render/terrain_material.rs:461`, queued in `RenderSystems::QueueMeshes` at `:614`), a compute
  `tiling_prepass` that refines the tile tree on the GPU (`src/render/tiling_prepass.rs:239`), and one
  `draw_indirect` per view (`src/render/terrain_view_bind_group.rs:484`).
- Why: a terrain is one huge LOD-morphed grid, not many meshes; the standard mesh pipeline cannot do per-patch
  GPU LOD. It still plugs into Bevy PBR, the deferred G-buffer and motion vectors.
- Streaming: tile atlas pyramid (`terrain_data/gpu_tile_atlas.rs`), load by distance per LOD
  (`tile_tree.rs:314-316`), preprocess tool tiles GeoTIFFs offline.
- Shadows: own heightfield raymarched shadow map "RDR2-style" (`src/terrain_shadow.rs:1`, compute pass at
  `src/render/terrain_shadow.rs:331-363`), other materials sample it. Optional `big_space` floating origin
  (`src/floating_origin.rs:3`). No published frame times.

## bevy_voxel_world
- Entity per chunk with stock `Mesh3d` + `MeshMaterial3d` (`src/plugin.rs:47,178`); meshing on
  `AsyncComputeTaskPool` with a budget that keeps the pool busy only until next frame, 4+ jobs per thread
  (`src/pool_budget.rs:1-30`). Streaming policy is a pure function over camera snapshots
  (`src/streaming_policy.rs:1-30`), testable without a World; spawn/despawn strategies are configurable.
- LOD: chunk keys carry `lod_level`, 16 levels, each doubling chunk span (`src/chunk.rs:28-120`);
  `README.md:173` says the example streams 500 chunks out in six levels, and the user hides seams (skirts).
- Active in 2026 and tracks Bevy 0.20, so its streaming policy code is the best reusable reference. No perf numbers.

## riverbed
- Reaches 2 km render distance (README claim, no ms). Entity per (chunk, face) mesh
  (`crates/rb_render/src/mesh_draw.rs:73-109`), meshes built on a thread and sent by channel; LOD from distance
  `(dist/4).clamp(1, CHUNK_S1)` (`mesh_draw.rs:42`).
- Own CPU frustum culling that replaces Bevy's: chunks carry `NoFrustumCulling` and a parallel system sets
  `Visibility` from sphere + AABB tests plus back-face culling per face direction
  (`crates/rb_render/src/chunk_culling.rs:8-50`, `par_iter_mut`). The code does not state why; the likely
  reason is per-entity visibility cost at thousands of chunk entities (inference). Block textures sit in one
  array (`texture_array.rs:325`).

## big_space
- No rendering. Value for us: integer grid cells + `Transform` compatibility; `benches/benchmarks.rs:83`
  spawns 100,000 wide-hierarchy entities and times propagation. Relevant to floating origin cost, not draw cost.

## bevy_meadow (GPU-driven grass; Bevy main)
- Patches are ECS entities for gameplay but do not render; one render driver per variant. A compute kernel
  culls, LODs and compacts every blade per view, then a handful of `draw_indexed_indirect` calls
  (`src/compute.rs:1-21`, `src/meadow.wesl:8`). Optional mesh-shader path claimed 2x faster
  (README features; `Cargo.toml:27`). Built on `ExtendedMaterial<StandardMaterial>` so lighting, shadows
  and deferred stay Bevy's. Custom draw, standard material: same split as bevy_terrain.

## Ethertum
- Entity per chunk with own `TerrainMaterial/FoliageMaterial/LiquidMaterial` (`src/voxel/voxel_client.rs:11`);
  its own DDGI GI plugin (`src/client/ddgi/ddgi1_plugin.rs`) next to Bevy's Solari feature. Multiplayer via renet
  with host-side chunk systems (`src/net/netproc_*.rs`). Nothing on culling or frame times.

## What the successful ones have in common
- Terrain is never ordinary meshes: either a custom GPU-refined phase (bevy_terrain) or per-chunk meshes with
  distance LOD (voxel projects). All stream by distance rings with a per-frame job budget.
- Many small things are merged into per-chunk meshes (OSM buildings, voxel faces), not one entity each. The one
  entity-per-object project (rust-gta-3) works because its visible set is small and it reuses retired entities.
- When they bypass Bevy's pipeline it is for something that is "one big draw with GPU LOD" (terrain, grass),
  and they keep `StandardMaterial`/`ExtendedMaterial` so lighting, shadows and deferred still apply.
- Per-entity visibility cost shows up at thousands of entities: riverbed replaced Bevy's culling with its own
  parallel system on `NoFrustumCulling`; consistent with our own `NoCpuCulling` work.
- Streaming policy is separate from rendering and testable on plain data (bevy_voxel_world); meshing and tile
  decode run on the async task pool with a work budget tied to finished jobs per frame.
- Shadows are special-cased cheaply where the world is huge: heightfield raymarch for terrain, none for baked
  content (GTA). None of them use per-light shadow maps for towns.
- Nobody in public Bevy code handles hundreds of skinned NPCs; floating origin is solved by big_space or
  camera-relative tile math.

## Open
- No published ms for any of them; running bevy_voxel_world's `noise_terrain_lod` and bevy_terrain's
  `heightfield` example on the 4060 laptop would give comparable numbers (not done: research only).
- Not read: dust's renderer, atlas (Tarkov map viewer, may be wgpu-only), Foxtrot, bevy_terrain's tile loader
  internals, bevy_voxel_world's lighting.
- Reddit r/bevy, Discord showcase and "This Week in Bevy" were not searched (no web tools used); a Bevy MMO or
  town-crowd project may exist there.
