# Pipeline warm-up (shader precompilation, reveal gate)

## What it does

Keeps pipeline compiles off the visible frames. While the screen is covered (the `Loading` phase, a door fade at
black) a reveal gate holds until the near meshes, their textures and their pipelines are ready, compiles run on
a wide pool, and a baked catalog of every (vertex layout, material, entity flags) class is precompiled ahead of
need at low priority. In play, compiles stay on Bevy's 4 async-compute threads.

## How it works

- **Reveal gate** `sf_world_bevy::RevealGate` (`superfargoth/crates/sf_world_bevy/src/reveal.rs:60`): terms cells,
  textures, assets, pipelines (draws skipped, shadow/prepass queues, `waiting_pipelines()`, the catalog), gates;
  named readiness sources in `RevealHolds` (`reveal.rs:124`, e.g. GI: `sf_gi_bevy::GiReadiness`,
  `superfargoth/crates/sf_gi_bevy/src/lib.rs:68`, bridged by `bevyrrowind/crates/mw_exterior_bevy/src/gi_volume.rs:37`).
  Cap 10 s after the cells (`reveal.rs:30`), with a `warn!` naming what was pending.
- **Compile pool** `superfargoth/crates/sf_world_bevy/src/compile_pool.rs:31`: all cores but 2 while the gate holds
  (`boost_while_holding`, `compile_pool.rs:93`; vendored `bevy_render` W2b hunk,
  `superfargoth/vendor/bevy_render/src/render_resource/pipeline_cache.rs:1187`). Shader composition runs outside
  the shader-cache lock (vendored `bevy_shader`, `superfargoth/vendor/bevy_shader/src/shader_cache.rs:210`;
  `pipeline_cache.rs:1112` `stage_modules`).
- **Precache** (vendored `bevy_pbr` patch PW, `superfargoth/vendor/bevy_pbr/src/precache.rs`): the draw path and
  the precache build keys with the same functions (`main_pass_mesh_key`, `precache.rs:149`, and siblings), so a
  precached pipeline is a hash hit for the later draw. `plan_specimen` (`precache.rs:377`) gives every key of a
  specimen per known view kind (camera main and prepass keys, both shadow templates, every subset of the
  previous-frame skin/morph bits). `PipelinePrecache` (`precache.rs:523`) queues specimens by tag;
  `precache_specimens_with` (`precache.rs:826`) admits them one pipeline at a time within `PrecacheLimit`
  (`precache.rs:482`), ordered after the draw specialisers (`precache_specimens_systems`, `precache.rs:802`).
  Main-world surface: `sf_world_bevy::Precache` (`superfargoth/crates/sf_world_bevy/src/precache.rs:182`,
  `request` / `status` by tag).
- **Low priority:** `precache_limit` (`compile_pool.rs:76`): while a reveal holds, all compile threads, half of
  them while a draw-path pipeline waits; 1 in play. Never unlimited: a later draw waits at most one task.
- **Catalog:** bake step `pipeline.catalog` (`superfargoth/crates/sf_model/src/catalog.rs:296` `load_catalog`;
  Morrowind enumeration `bevyrrowind/crates/mw_bake/src/catalog.rs`). `sf_world_bevy::CatalogWarmup`
  (`superfargoth/crates/sf_world_bevy/src/catalog_warm.rs:109`) loads one representative per entry through
  streaming's own material cache, flags from `entry_flags` (`catalog_warm.rs:220`), and requests tag `catalog`;
  the game starts it on `Loading` (`bevyrrowind/crates/mw_exterior_bevy/src/catalog_warm.rs:41`).

## Bevy features used

`PipelineCache`, `SpecializedMeshPipelines`, `AsyncComputeTaskPool`; Bevy 0.20 has no precache API
(bevy#10871), hence the vendored patches (`superfargoth/vendor/README.md` rows W2a, W2b, W3).

## Switches and counters

`--env BEVYRROWIND_COMPILE_THREADS=<n>` (0 = stock threads). Counters `pipelines/{compile,front,driver,wait,lock}_ms`,
`pipelines/after_reveal`, `draws/skipped_late`, `precache/{requested,hit,missed,pending_assets,throttled,outstanding,draw_waiting,limit}`.
Log lines: `reveal: <kind> held ...`, `reveal: <kind> +10.0 s: after_reveal ...`, `warmup: catalog N specimens ...`,
`pipelines: compiled ...` (`superfargoth/crates/sf_diag_bevy/src/pipeline_watch.rs`), `precache missed: ...` (a
`waiting 0` line is a true miss).

## Known limits

- The first reveal waits on the whole catalog (~250 pipelines where the census view uses ~107): census warm hold
  ~6.3 s against ~2.9 s without it, cold capped at 10 s ([perf ledger](../../ledger/perf.md), 2026-11-16).
  Planned: wait only for the shown set and leave the rest to the background (UE5 PSO precaching).
- The shader front end (composition + naga) is now the bulk of a warm hold; there is no module cache.
- A view key first seen after a specimen resolved (settings change, GI flip) does not re-plan it (W6).
- Measuring: use a private NVIDIA GLCache per exe (cold, then warm x4); the shared ~1 GB cache evicts.

## Specs

[pipeline-warmup](../../../superfargoth/docs/specs/pipeline-warmup.md) (§5 design, §8 slices and "as built").
