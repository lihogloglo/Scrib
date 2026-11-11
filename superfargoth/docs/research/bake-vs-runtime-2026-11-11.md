# Bake vs runtime: who bakes what, when (2026-11-11)

Research for `specs/bake.md` "Inventory" and "What bakes when" (session bake-policy, owner question 2026-11-11: "a
proper design for the things that are baked when first launching the game and the things done at runtime"). One
web-search pass (no pages fetched in full); "unverified" marks claims from memory or a single forum post.

## Findings

**Bethesda Creation Kit: precombines, previs, navmesh.**
- A mod that edits a precombined reference (an identical-to-master record or any edited REFR) disables precombines
  and previs for that cell, and per community docs the 3x3 block around it; the visible symptom is an FPS loss in
  that area. One forum post says the invalidation is deliberate so an edit doesn't force a rebuild of everything
  (community explanation, not official). https://forums.nexusmods.com/topic/5446017-precombined-meshes-what-destroys-them
- PRP (Previsibines Repair Pack) regenerates the game's precombines and previs, cuts draw calls, fixes placement
  errors, loads before cell-editing mods, near the end of the load order, with patches that regenerate previs for
  other mods. https://www.nexusmods.com/fallout4/mods/46403
- Precombines, previs and navmesh are generated in the CK by developers or mod authors and shipped in plugins;
  navmesh edits from two mods conflict (deleted-navmesh crashes are the classic case). Unverified (from memory).

**xLODGen / DynDOLOD / TexGen (Skyrim, FO4).**
- Re-run DynDOLOD after installing, removing or updating mods with LOD assets or that change towns or areas; finalise
  the load order first, then TexGen, then DynDOLOD. https://stepmodifications.org/forum/topic/9817-when-to-re-run-dyndolods-script
- Outputs are mods placed last (TexGen output at the end; DynDOLOD.esm the last master); patchers sort after it.
  https://dionysist.substack.com/p/how-to-use-dyndolod-texgen-and-xlodgen ,
  https://stepmodifications.org/forum/topic/13726-wrye-bash-discovered-load-order-problem-with-dyndolod/
- Run times (minutes to hours on big lists) and the object/terrain/tree LOD split: unverified (dyndolod.info).

**Unreal Engine.**
- Derived Data Cache is hierarchical (local, shared, cloud); since UE 5.4 the local DDC defaults to a Zen store; Unreal
  Cloud DDC replicates across regions. https://dev.epicgames.com/documentation/unreal-engine/using-derived-data-cache-in-unreal-engine ,
  https://dev.epicgames.com/documentation/unreal-engine/set-up-zen-storage-server-as-shared-ddc-for-unreal-engine
- Keys from input hashes plus per-format version GUIDs: unverified this pass (J9 2026-10-12 cited the same, also from
  memory). A forum report of shared-DDC misses blames non-deterministic key parts (unverified):
  https://forums.unrealengine.com/t/texture-visibility-issues-in-ue-when-using-a-shared-ddc-cache/2653843
- PSO precaching compiles pipelines for content as it loads; bundled PSO caches are recorded by playing a build and
  shipped; Epic calls bundled caches the best practice until 5.2, and the two combine; precaching does not cover global
  shaders as of 5.5. https://www.unrealengine.com/tech-blog/game-engines-and-shader-stuttering-unreal-engines-solution-to-the-problem ,
  https://dev.epicgames.com/documentation/en-us/unreal-engine/pso-precaching-for-unreal-engine

**Unity.**
- Asset Database v2: `Library/SourceAssetDB` (sources and hashes) and `Library/ArtifactDB` (import results and their
  dependencies). https://docs.unity3d.com/Manual/asset-database-contents.html
- The legacy Cache Server keyed imports by MD5 of the source and `.meta` plus importer and postprocessor versions;
  pipeline v2 uses Unity Accelerator (its keying not verified). https://docs.unity3d.com/es/Manual/CacheServer.html
- Unity 6 PSO tracing: `GraphicsStateCollection` traces in development builds, saves a `.graphicsstate` asset,
  `WarmUp` / `WarmUpProgressively` at load; experimental; no compute shaders.
  https://docs.unity3d.com/6/Documentation/Manual/shader-pso-trace.html

**OpenMW.**
- `openmw-navmeshtool` pre-generates navmeshes into a SQLite `navmesh.db`; the docs warn large setups take a long
  time; generation stops at the DB size limit ("max navmeshdb file size"). No vanilla timing found.
  https://modding-openmw.com/tips/navmeshtool/ , https://modding-openmw.com/mods/generate-a-navmesh-cache
- Without the DB, navmesh tiles are generated at runtime on worker threads (consistent with the sources; the tool
  exists to avoid loading stutters). Settings: https://openmw-zack.readthedocs.io/en/latest/reference/modding/settings/navigator.html
- Object paging (`[Terrain] object paging`, `... active grid`, `... min size`, merge factors), OpenMW ≥ 0.47, built at
  runtime and cached in memory (the in-memory part unverified).
  https://openmw-zack.readthedocs.io/en/latest/reference/modding/settings/terrain.html
- A separate simplified-mesh tool builds its DB in about 2 minutes on one author's machine (merge request):
  https://gitlab.com/OpenMW/openmw/-/merge_requests/1287
- OpenMW developers note that collecting mesh data can be slower than rasterising it (nav bake cost lies in input
  gathering, as our own bake's time outside `Step::run`). https://git.cijber.net/mirror/openmw/src/commit/d30f0f11b4c5a70b7c6cae4ff424ccce0eb1aa29/components/detournavigator/makenavmesh.cpp

**Recast/Detour.** No published ms-per-tile figure; Recast exposes per-stage timers (rasterise, compact, regions,
contours, polymesh), so NV0/NV4 measure our own. https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/Navmesh/rcTimerLabel

**Content-addressed build caches (from memory).** Bazel keys an action by its inputs, command line and environment;
ccache/sccache hash preprocessed source, compiler identity/version and flags; a Nix store path hashes its
derivation's inputs. Same shape as our key (step id + `VERSION` + params + input fingerprints).

**Shader pre-caching.** Steam Shader Pre-Caching (since 2017) downloads precompiled shaders for the user's GPU and
driver, built on Fossilize (Vulkan). https://gamingonlinux.com/2020/05/valve-improve-linux-vulkan-shader-pre-caching
First-launch shader compiles on PC are reported from minutes to about an hour (The Last of Us Part I; anecdotal); a
Dragon Age: The Veilguard user reports over an hour on first load and 1-2 minutes after.
https://forums.ea.com/discussions/dragon-age-the-veilguard-discussion-en/poor-performance---fps-sutteringlag-etc/7399739/replies/7399743

**Mod-managed games with a rebuild step.** No documented example besides the Bethesda LOD/previs tools above.

## What we take

- The Bethesda model ships generated data with each mod, so mods conflict over it and users run fixers (PRP,
  DynDOLOD) last. Our per-load-order bake on the player's machine removes that class of conflict: the start check is
  "re-run DynDOLOD", automated and incremental.
- UE/Unity/Bazel confirm the keying (inputs + tool version) and the failure mode (a tool change without a version
  bump): golden digests per step (bake.md gap 7).
- OpenMW proves runtime navmesh generation is workable but stutters; its prebuilt DB is our baked `nav.tiles`.
- Shader compile screens of minutes to an hour are what players dislike; the warm-up gets a fixed slot (20 s) in the
  3 min budget, and a pipeline cache keyed per machine (pipeline-warmup.md).

## Comparison table (as in bake.md)

| Engine / tool | What it bakes, when | Mod change | Ours |
|---|---|---|---|
| Bethesda Creation Kit (FO4, Starfield) | precombined meshes + previs, navmesh, LOD baked by developers and mod authors, shipped in plugins | a mod editing a precombined reference disables precombines and previs for its cell (3x3 per community docs); PRP rebuilds them; navmesh edits conflict between mods | the player's machine rebakes per load order, so no mod ships generated data and nothing conflicts: **differ by design** |
| xLODGen / DynDOLOD / TexGen (Skyrim, FO4) | user-run LOD generators after finalising the mod list; output is a mod placed last | the user must re-run after mods that add LOD assets or change towns | the same idea, automatic: the start check is the "re-run DynDOLOD" step, incremental by traces: **match, automated** |
| OpenMW | no offline bake; scene graphs built from NIF at load, object paging and distant terrain at runtime; navmesh generated at runtime, optional `openmw-navmeshtool` prebuild into `navmesh.db` (SQLite; "takes long" on large mod setups) | navmesh DB entries keyed by tile input, regenerated on change | we bake more (meshes, far field, textures) because modern rendering needs it, within one budget: **differ** |
| Unreal DDC + cook | developer-side derived data (shaders, textures, mesh builds) cached locally/shared/Zen, keyed by input hashes and version GUIDs; cooked for shipping; PSO precaching at load (5.1/5.2+) and bundled PSO caches | n/a (developer pipeline) | same keying model; our "cook" runs on the player's machine: **match** |
| Unity Asset Database v2 | import artifacts keyed by source + importer version + settings in `Library/`; Accelerator shares them; PSO tracing `GraphicsStateCollection` warm-up (Unity 6) | reimport of changed assets only | **match** |
| Steam shader pre-caching, "compiling shaders" screens | Fossilize/Vulkan pipeline caches; first-launch shader compiles reported from minutes to an hour (TLOU Part I, anecdotal) | driver update recompiles | warm-up reserved in the 3 min budget |
| Bazel / ccache / sccache / Nix | action key = hash of inputs + tool identity and version | rebuild what changed | same (step id + `VERSION` + inputs) |

Sources: `../research/bake-vs-runtime-2026-11-11.md`. Earlier pipeline comparison (J9, 2026-10-12):
