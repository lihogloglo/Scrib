# Library rocks in the game (RG-P research, 2026-11-27)

Question: how do the scatter's **library rocks** (`mw_worldgen` `flora_rocks.rs` `RockPlacement`: class, variant
hash, size-mark transform, sink share, regime; `library_rocks`, no consumer yet) reach the generated world as drawn,
LODed, culled and collidable instances, reusing what exists? Feeds the RG-P SPEC row (`rockgen.md` "Placement in the
game (RG-P)"). V = source read this session at `path:line`; U = known from talks, docs or summaries, not read in full.

Scale to plan for (WG-E3m, `docs/ledger/worldgen.md` 2026-11-27): x10s keeps 2.21 M rocks, 25 % pack models, so
~1.66 M library rocks over the island, 9-26 rocks/ha per geology unit. In a 1 km streaming radius (~314 ha) that is
~3-8 k library rocks, each up to `lod_levels` LOD entities.

## 1. What the game does today (V)

**Pack-model rocks are already placements.** `flora::extra` (`mw_worldgen/src/flora.rs:1292-1322`) returns
`("flora", placements(&run))` and `("rocks", rocks::rock_placements(&run))`; `rock_placements`
(`flora_rocks.rs:~360-375`) turns each `RockMesh::Model` rock into an `sf_world::Placement` copying its learned
reference (base, model, class, flags). Library rocks (`RockMesh::Class`) are dropped there: `library_rocks`
(`flora_rocks.rs:393-412`) is only called by a test.

**The generated world's statics stream from an in-memory index.** The app hands `WorldgenContent { placements,
extra }` (`apps/bevyrrowind/src/world.rs:314-315`) to `mw_exterior_bevy::worldgen` (`worldgen.rs:102-110`), which
indexes every placement of `placements` and every `extra` list with `build_pages` (`content_index`,
`worldgen.rs:142-155`). Bounds come from the bake's `model.info` per model path (`model_infos`, `worldgen.rs:118-139`);
a placement whose model has none is **indexed as a point** (`sf_world/src/bake.rs:398` comment). The scaffold note
(`worldgen.rs:10-13`) says the generated world turns the precombine, static scene and far field **off**: its statics
spawn one entity per placement.

**Spawning is a per-class hook.** `sf_world_bevy` `PlacementSpawners` (`spawn.rs:129-190`) maps
`PlacementClass` -> `Arc<dyn PlacementSpawner>`; `Custom(u16)` classes fall to `default` (the `StaticSpawner`) unless
the app `set`s one (`spawn.rs:142-146`). `PlacementClass::Custom` is documented "Content-pack defined"
(`sf_world/src/placement.rs:~66`). The trait (`spawn.rs:105-123`) has `spawn(world, cx, placement)`,
`needs_model(placement)` (default: `placement.model.is_some()`), `prepare` (an extra asset the reveal waits for),
`activate` / `deactivate` (R0). `spawn_placement` (`stream.rs:1700-1733`) builds `SpawnCx { local, model,
max_distance, end_band, lod, layers, parent, .. }` and calls the class's spawner.

**The static path is tied to `StandardMaterial` and the `mesh.cook` format.** A model loads as
`baked://mesh.cook/<model>.sfmesh` (`stream.rs:1465-1494`, `StreamingConfig` defaults `stream.rs:141-142`,
`sf_model_bevy/src/source.rs:115`) into `LoadedModel::Cooked(ModelTemplate)`, whose LOD parts are
`(Handle<Mesh>, Handle<StandardMaterial>)` (`spawn.rs:38-54`). Library rocks have no UVs and draw with
`RockMaterial = ExtendedMaterial<StandardMaterial, RockExt>` (world-axis triplanar, `sf_rockgen_bevy/src/material.rs:68`),
which the owner approved at RG-3/RG-4. Cooking library variants into `.sfmesh` would put them in the static path
but lose that material (or need a generic-material `ModelTemplate`, a framework refactor across `spawn.rs`,
`lod_live`, the precombine and the static scene).

**GPU culling is Static-class only.** `StaticSpawner::spawn` (`spawn.rs:309-313`) marks GPU-culled parts only for
`PlacementClass::Static` when `StaticGpuCulling::on(world)`; a custom spawner decides for itself.

**Collision is a model-path lookup.** `sf_physics_bevy` `PhysicsHook` wraps the spawners of the classes the app picks
(`stream.rs:505-516`); on activation it queues a `Fixed` collider whose shape comes from `PhysicsShapeSource(ShapeLoader)`,
`ShapeLoader = Arc<dyn Fn(&str) -> Option<Arc<StaticShape>>>` (model path -> shape, built once per model on the task
pool and shared, `stream.rs:47-53`, module doc `stream.rs:1-11`); inserts run under `insert_budget_ms`. `StaticShape` is
a parry `TriMesh` (`sf_controller/src/statics.rs:148-160`). The game installs the source and the hook in
`mw_walkers_bevy/src/physics.rs:109, 688`. A library `Hull` (`sf_rockgen/src/library.rs:93-99`: points + outward
triangles, quickhull) is a valid triangle mesh for it.

**The library is baked, unread.** `bake_rock_libraries` (`mw_boot_bevy/src/bake.rs:~285-297`) bakes one
`rockgen.library` product per class (`LibraryStep::from_classes(CLASSES, ..)`, keys per class); "No consumer yet (the
placement slice reads them)". `sf_rockgen::bake::decode` (`bake.rs:125`) gives a `Library` of `Variant { seed, lods:
RockLods, hull: Option<Hull> }` (`library.rs:101-110`), Bevy-free.

**The Bevy side exists for one lab.** `sf_rockgen_bevy` (`lib.rs:140-230`): `rock_mesh` (RockMesh -> Mesh),
`lod_ranges(size_m, levels, settings)` (one `VisibilityRange` per LOD, dithered crossfade, all LODs at one origin),
`lod_bundles(meshes, materials, ranges, tag)` -> `(Mesh3d, MeshMaterial3d<RockMaterial>, VisibilityRange, MeshTag,
Transform)` per LOD. The rock lab (`mw_exterior_bevy/src/level/rock_lab.rs:280-315`) builds `RockTextures::from_standard`
from its level's texture list and one material per LOD (`rock_material(.., lod, ..)`). No class -> texture-layer
adapter data exists yet (`scatter_species.toml` has none; `rockgen.md` "Materials": "which layer per rock type is
adapter data"; RG-A1).

**Memory.** `Placement` = id (u128) + `RecordRef` (two `Arc<str>`-backed fields) + `Option<VfsPath>` (`Arc<str>`,
`sf_vfs/src/path.rs:11`) + `WorldTransform` + class + flags (`sf_world/src/placement.rs:119-127`), ~130 B before shared
strings. 1.66 M library rocks as placements ~ 215 MB, beside the 20.8 M flora placements x10s already indexes
(WG-E2o.m): +8 % of what the index already holds.

## 2. Bevy 0.20 native (V unless marked)

- **Automatic instancing.** Entities that share a `Mesh` handle and a material handle batch into one instanced draw;
  GPU preprocessing builds `MeshUniform`s and indirect args in a compute pass, and batch sets with the same slab,
  pipeline and material bind group share one multi-draw indirect call (`docs/wiki/bevy/render-and-visibility.md` §5-6,
  `bevy_render/src/batching/gpu_preprocessing.rs:1052-1135`, `render_phase/mod.rs:109, 476`). `NoAutomaticBatching`
  opts out (`vendor/bevy_pbr/src/render/mesh.rs:1049-1053`, `should_batch`).
- **`MeshTag` does not split batches**: it is carried per instance in the GPU-building input
  (`vendor/bevy_pbr/src/render/mesh.rs:1261-1275`, `tag: Option<&MeshTag>` into the instance), so `lod_bundles`' per-rock
  texture offset keeps one batch per (variant LOD mesh, LOD material). (The batch-key derivation itself was not traced
  line by line: U.)
- **LOD = `VisibilityRange`** (`bevy_camera/src/visibility/range.rs:80`; wiki §2): one entity per LOD level, dithered
  crossfade, ranges deduplicated in the render world (`RenderVisibilityRanges`). GPU-culled meshes get their range
  tested on the GPU (wiki §6).
- **`NoCpuCulling`** moves frustum culling to the GPU for an entity (wiki §6; `many_cubes.rs:209-220`); the statics use
  it under `StaticGpuCulling`.
- Nothing in Bevy 0.20 is an "instance list" component (no ISM-like buffer of transforms behind one entity) outside
  our vendored `gpu_instance_batch` (`GpuBatchedMesh3d`, bevy#25978 port, used by the static scene, wiki
  `engine/statics-and-lod.md`). Entities with shared handles are the native path.

**Crates.** No crate survey beyond Bevy native (U): Bevy's automatic batching + MDI already instances shared
mesh + material, the static scene already has the retained-batch path, so an instancing crate would add a second
mechanism. Not searched further.

## 3. Industry (U unless marked)

- **Unreal PCG** (Epic docs "Using PCG generation modes", `UPCGStaticMeshSpawnerSettings`, ISM component page; U,
  summaries): the Static Mesh Spawner emits ISM/HISM components per partition grid cell (`Partition Grid Size` on the
  PCG world actor), either at edit time (baked into the level) or with *Generate at Runtime* around generation
  sources (radii per grid size, cleanup >= generate). Per-instance LOD on HISM (and newer ISM); Nanite projects use ISM.
  Rocks are one more mesh descriptor; the spawner chooses the mesh per point, the instance is (mesh, transform).
- **Horizon Zero Dawn** (van Muijden, GDC 2017 "GPU-Based Run-Time Procedural Placement", slides PDF; U, summaries):
  placement runs on the GPU at runtime from 2D maps (~4 MB/km²) and ecotopes, 500+ asset types, 100 k+ objects
  resident, ~250 µs GPU per frame while moving; rocks are among the placed assets. The output is instanced draws per
  asset; collision for placed objects is handled by the runtime as well (not read in detail).
- **Far Cry 5** (Carrier, GDC 2018 "Procedural World Generation of Far Cry 5"; U): Houdini tools generate placements
  offline into the world data; the engine streams them as ordinary instanced entities.
- **RDR2, Ghost of Tsushima, Cyberpunk 2077** (U): no public detail on rock instancing beyond instanced statics in
  the world streaming; Tsushima's talks cover grass and wind, not rocks.
- **OpenMW** (Morrowind behaviour): rocks are ordinary statics (`STAT` records, NIF meshes), one reference each; this
  is how our pack-model rocks already arrive.

Common pattern: placement data offline or near the player, **one instance = (mesh id, transform)** streamed with the
world's partition, drawn instanced per mesh, collision as a shared shape per mesh. Our scatter is offline (the
`flora.bin` / `rocks.bin` product), so the FC5 / UE edit-time pattern fits: the instances join the world's streamed
index, not a second streamer.

## 4. Options for RG-P

| Option | What | Keeps | Costs |
|---|---|---|---|
| **A. Cook into `mesh.cook`** | each library variant becomes an `.sfmesh` model; rocks are `Static` placements | everything of the static path (GPU culling, lod_live, scene, far field later) | loses `RockMaterial` (StandardMaterial needs UVs; rocks have none) unless `ModelTemplate` becomes material-generic: a framework refactor |
| **B. `Custom` class + rock spawner** (recommended) | rocks are placements of an adapter-chosen `PlacementClass::Custom(n)` whose `model` is a library id path (`rockgen/<class>/<variant>`); a `PlacementSpawner` in `sf_rockgen_bevy` resolves it to the decoded library's meshes + per-class `RockMaterial`s and spawns `lod_bundles`; `needs_model` false (no `mesh.cook` load) | the streamed index, pages, residency, R0 activation, physics hook (shape loader resolves the same id to the hull), the triplanar material, automatic instancing | the spawner owns its LOD chain (`lod_ranges`) and GPU-culling choice; `ModelInfo` for the id must come from the library (bounds x scale), else rocks index as points and get no size-cull end |
| **C. Own instance source** | a rock streamer beside `sf_world_bevy` (chunks, rings, its own physics activation) | full control | a second streamer, second residency and activation path: duplicates `sf_world_bevy` (CONTRIBUTING rule 2) |

**Recommendation: B.** It is the seam the framework already declares ("Content-pack defined" classes, per-class
spawner hooks), it matches the industry pattern (instances in the world's own streamed partition, one shared mesh and
shape per variant), and it keeps the approved material. A stays possible later for interiors or the static scene once
`ModelTemplate` takes other materials; it is not needed for a generated exterior, which has the scene and far field
off anyway (`worldgen.rs:10-13`).

## 5. Open points for the SPEC (decide from the standard, not owner questions)

1. **Where the variant, sink and bounds resolve.** `RockPlacement` carries a variant *hash* and a sink *share of the
   short axis*; both need the decoded library (variant count, variant bounds). Resolve once, Bevy-free, where the app
   builds `WorldgenContent` (it has the bake store): variant index = hash mod N, position lowered by
   `sink x short_axis x scale`, and a `ModelInfo` per id (variant bounds, `lod_distance` from the LOD settings) so the
   index pages and size culling see real extents. The spawner then reads no hash.
2. **Id scheme.** A `VfsPath`-shaped id (`rockgen/<class>/<n>`) never mounted in `sf_vfs` (the spawner's
   `needs_model` false; the physics `ShapeLoader` matches the prefix). Check `VfsPath` normalisation (case, separators)
   and that `model_infos` tolerates ids with no `model.info` (it does: indexed as a point, `worldgen.rs:126-131`).
3. **Base record.** `Placement::base` is required; a synthetic `RecordRef` per class (adapter kind) or the namespace.
4. **Culling.** With ~3-8 k rocks x LODs per radius, follow the statics: `NoCpuCulling` when `StaticGpuCulling::on`.
5. **Materials.** One `RockMaterial` per (class, LOD) from adapter texture layers (new `[rock_textures]` or per-class
   rows; RG-A1). Until then a default layer, logged.
6. **Budget group and collision.** Rocks are budget group 0 already (scatter); collision: hull per variant through
   the physics hook (PhysicsHook wraps the Custom class too), hull error `Hull::error_m` already bounded.
7. **Regime.** `regime` is unused by the draw until RG-9i (cover); carry it in the id or drop it until then.

Sources: Epic, "Using PCG generation modes in Unreal Engine"
(https://dev.epicgames.com/documentation/en-us/unreal-engine/using-pcg-generation-modes-in-unreal-engine, U);
Epic, "Instanced Static Mesh Component"
(https://dev.epicgames.com/documentation/unreal-engine/instanced-static-mesh-component-in-unreal-engine, U);
van Muijden, GDC 2017 (https://www.guerrilla-games.com/read/gpu-based-procedural-placement-in-horizon-zero-dawn, U);
80.lv summary (https://80.lv/articles/real-time-procedural-placement-in-horizon-zero-dawn/, U); Carrier, GDC 2018
(U, not fetched).
