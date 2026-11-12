# Bevy 0.20: schedules, render sub-app, pipelining, transforms, big_space

Facts only, from the stock 0.20.0 source (`~/.cargo/registry/src/*/<crate>-0.20.0/`) and `superfargoth/vendor/big_space`.
Paths are `<crate>/src/...:line`.

## 1. Main schedules

- `Main` runs, in order: `First`, `PreUpdate`, `RunFixedMainLoop`, `Update`, `SpawnScene`, `PostUpdate`, `Last`
  (`bevy_app/src/main_schedule.rs:225-232`); startup: `PreStartup`, `Startup`, `PostStartup` (:233).
  `FixedMain` runs `FixedFirst`, `FixedPreUpdate`, `FixedUpdate`, `FixedPostUpdate`, `FixedLast`
  (`main_schedule.rs:355-364`) zero or more times inside `RunFixedMainLoop`.
- `PostUpdate` holds, among others: `AssetEventSystems` (`bevy_asset/src/lib.rs:679`), animation
  (`bevy_animation/src/lib.rs:1322`), `TransformSystems::Propagate`, visibility and culling
  ([render-and-visibility.md](render-and-visibility.md)).

## 2. Render sub-app and pipelining

- `RenderApp` is a `SubApp` built with `ExtractPlugin` (`bevy_render/src/lib.rs:395-400`;
  `bevy_extract/src/extract_plugin.rs`). Each frame: `entity_sync` (main-to-render entity map), then
  `ExtractSchedule` with access to `MainWorld` (`extract_plugin.rs:83-92`). `ExtractSchedule` does not apply
  commands; `ExtractCommands` applies them at the start of the render schedule (`extract_plugin.rs:60-77`).
- `Render` schedule order (`bevy_render/src/lib.rs:327-370`, sets in `RenderSystems`, :186-239):
  `ExtractCommands` -> `PrepareAssets` -> `PrepareMeshes` -> `CreateViews` -> `Specialize` -> `PrepareViews` ->
  `Queue` (`QueueMeshes`, `QueueSweep`; after `prepare_assets::<RenderMesh>`) -> `PhaseSort` -> `Prepare`
  (`PrepareResources`, `...BatchPhases`, `...WritePhaseBuffers`, `...CollectPhaseBuffers`, `...Flush`,
  `PrepareBindGroups`) -> `Render` -> `Cleanup` -> `PostCleanup`. Chains are `chain_weak`.
- `PipelinedRenderingPlugin` (`bevy_render/src/pipelined_rendering.rs:143`): the render app lives on its own thread;
  frame N renders while frame N+1 simulates. The main thread does `sync` + `extract` (blocking both), then the main
  schedule runs while the render thread runs commands + `Render`; extract again waits for both
  (`pipelined_rendering.rs:105-138`). Hence extract is serial cost on the critical path.
- Render-world asset and material preparation: [render-assets-and-materials.md](render-assets-and-materials.md).

## 3. Transform propagation

- `TransformPlugin` (`bevy_transform/src/plugins.rs:20`): in `PostUpdate` (and `PostStartup`), chained in
  `TransformSystems::Propagate`: `mark_dirty_trees`, `propagate_parent_transforms`, `sync_simple_transforms`
  (:26-47). `bevy_animation` runs before this set (`bevy_animation/src/lib.rs:1340`).
- Static transform optimisation exists: `StaticTransformOptimizations` resource, default `Enabled`
  (`bevy_transform/src/systems.rs:89-102`). `mark_dirty_trees` (:111) walks up from every entity with
  `Changed<Transform>` / `Changed<ChildOf>` / `Added<GlobalTransform>` (and orphaned) setting `TransformTreeChanged`
  on each ancestor (:111-160, parallel version with atomics). Propagation skips a child whose
  `TransformTreeChanged` and parent `GlobalTransform` are both unchanged (:710-715).
- Changed entities still pay: `set_if_neq` on `GlobalTransform` (:719); `sync_simple_transforms` queries
  `Changed<Transform>` on parentless childless entities (:42-52). `Changed<T>` filters are table scans over
  matching tables (comment at :273: parallelised for that reason).

## 4. Change detection costs (what the sources say)

- Query filters `Changed<T>` / `Added<T>` check a per-component tick per entity per table; the cost is
  O(entities in matching tables) each run, independent of how many changed (`bevy_transform/src/systems.rs:273`
  comment; `bevy_pbr/src/render/mesh.rs:2055` mesh extraction filter scan, see
  [render-and-visibility.md](render-and-visibility.md) section 4).
- `bypass_change_detection()` avoids marking; `set_if_neq` marks only on a different value.

## 5. big_space (vendored `superfargoth/vendor/big_space`, 0.14.0-rc.0)

- Model: `BigSpace` root + nested `Grid { cell_edge_length (default 2000), maximum_distance_from_origin =
  edge/2 + switching_threshold }` (`grid/mod.rs:33-56`); entities carry `CellCoord {x,y,z}` + `Transform`
  (`grid/cell.rs:25-35`); exactly one `FloatingOrigin` per `BigSpace` (`floating_origins.rs:20,61`).
  `TransformPlugin` must be disabled (`plugin.rs:134-139`).
- Systems in `PostUpdate`, in `TransformSystems::Propagate` (`plugin.rs:146-194`): `recenter_large_transforms`
  (`BigSpaceSystems::RecenterLargeTransforms`: for `Changed<Transform>` non-`Stationary` entities whose
  translation exceeds the grid limit, move to the right cell, `grid/cell.rs:82-112`) and `find_floating_origin`;
  then `LocalFloatingOrigin::compute_all` (every grid learns where the origin is relative to it,
  `grid/local_origin.rs:439`); then `propagate_root_grids` and `propagate_high_precision_channeled`
  (`CellCoord` entities, `GlobalTransform` relative to the origin, `grid/propagation.rs:26,123`); then
  `propagate_low_precision` for plain `Transform` subtrees under cell entities (:318).
- When the floating origin changes cell, every entity in that grid needs a new `GlobalTransform`
  (`local_origin.rs:144-150`, `is_local_origin_unchanged`).
- Pruning: `GridDirtyTick` per `Grid` (`stationary.rs:75-90`) records the last tick any non-`Stationary` entity
  in its subtree changed; clean subtrees are skipped (`propagation.rs:79,213`). `Stationary` entities are
  excluded from recentering and hashing (`stationary.rs:47`).
- Our patch (README row "dirty-tree pruning"): `plugin.rs:185-194` adds Bevy's `mark_dirty_trees` and
  `StaticTransformOptimizations` to the schedule, and `propagation.rs:439-445` returns early for unflagged
  low-precision subtrees under an unchanged parent.

Scrib uses this in [streaming](../engine/streaming.md) and [frame and views](../engine/frame-and-views.md).
