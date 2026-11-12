# Exterior streaming and spawning

## What it does

Keeps cells around the camera resident at four levels (`Unloaded < Far < Near < Active`, `sf_world/src/rings.rs:11`), turns
each cell's placements into entities under a `big_space` grid, and removes them again. Fetch and model loading are async;
spawn, upload (reveal), activate and despawn run on the main thread under per-frame budgets. Framework: `sf_world_bevy`.
Game wiring: `mw_exterior_bevy`.

## How it works

**Frame loop.** `WorldStreamingPlugin` (`superfargoth/crates/sf_world_bevy/src/lib.rs:90`) adds `big_space` (`lib.rs:93`) and
runs `drive_streaming` in `Update` (`lib.rs:130`). `Streaming::drive` (`stream.rs:567`) per frame: builds `Anchor`s from
`StreamingAnchor` entities with smoothed velocity (`stream.rs:669`); ticks `sf_world::Scheduler` (`stream.rs:576`); starts
`Fetch` (index page, IO pool) and `Assets` (`stream.rs:1023`); polls models (`stream.rs:1108`, `max_loading_meshes` = 64);
marks assets done when models, far proxy and scene table are ready, with one settle frame before a spawn so mesh upload and
instance extraction do not coincide (`stream.rs:592-630`); then runs `Despawn, Deactivate, Spawn, Upload, Activate`
(`stream.rs:656`, `run_stage` `:956`) with ms budgets (bytes for `Upload`). Stages: `sf_world/src/scheduler.rs:67`. Level
changes publish `CellLoaded`/`CellUnloaded` (`stream.rs:980`).

**Origin-shift guard.** While the floating origin is within 0.05 s of travel of a grid-cell change, plus 2 frames after one,
the five main-thread stages wait, at most 6 frames in a row, never in burst mode (`stream.rs:54-58, 637, 702-709`): a shift
rewrites every `GlobalTransform` and Bevy re-collects all mesh instances.

**Floating origin.** One `BigSpace` root per worldspace, `Grid(256 m, 32 m)` (`coords.rs:25`; `sf_coords/src/lib.rs:23`);
`WorldPos` maps 1:1 to `CellCoord + Transform` (`coords.rs:38,48`). The exterior camera has `FloatingOrigin` and
`StreamingAnchor::visual()` (`mw_exterior_bevy/src/exterior.rs:804-807`); the player has `StreamingAnchor::gameplay()`
(`exterior.rs:872`). `TransformPlugin` is disabled (`exterior.rs:291`). A streamed cell is a `Stationary` entity with
`CellCoord` (`stream.rs:1250`). Shaders get world phases from the `WorldOrigin` uniform (`origin.rs:1-40`).

**Rings.** Defaults (`rings.rs:81`): active 192/256 m, near 768/896, far 3000/3250 (load/unload). With a far field,
`far_streaming_config` (`sf_lod_bevy/src/lib.rs:136`) derives near/far from the handoffs (`exterior.rs:269`);
`--near-ring/--far-ring` override (`exterior.rs:1276`).

**One placement into entities.** `spawn_item` (`stream.rs:1353`) spawns the far proxy, then the precombine buckets (see
`statics-and-lod.md`), then 16 placements per item into a hidden batch under a hidden group under the cell. A
`PlacementSpawner` is picked by class (`spawn.rs:123-186`): the game registers light, uncooked-NIF and actor spawners
(`exterior.rs:727-745`); statics use `StaticSpawner` (`spawn.rs:298`). `spawn_template_ranges` (`spawn.rs:333`) makes **one
entity per LOD level per mesh part**: `Mesh3d`, `MeshMaterial3d`, local `Transform`, `VisibilityRange`, worldspace
`RenderLayers`, `Aabb` + `NoAutoAabb`, `StaticPart`, `ChildOf(batch)` (`spawn.rs:340-352`); `NoCpuCulling` when GPU-culled
(`spawn.rs:364`); ray-tracing variants are not used in exteriors (`exterior.rs:1207`).

**LOD ranges.** `chain_ranges` (`spawn.rs:412`) maps per-level projected error to switch distances plus an end band;
`visibility_range` (`spawn.rs:431`) makes dithered `VisibilityRange` fades; `shared_range` (`spawn.rs:468`) rounds bounds up
to a log grid (32 steps per octave) because Bevy 0.20 gives each distinct range a never-freed GPU slot (65535 max). A
placement ends at the far handoff if far-eligible, else its ring's unload radius, shortened by size culling
(`Ends::of` `stream.rs:1756`, `size_culled_end` `spawn.rs:396`).

**Reveal.** A cell group stays `Visibility::Hidden` until `reveal` in the `Upload` stage (`stream.rs:1575`); `RevealGate`
(`reveal.rs`) is what loading and door fades wait on.

## Bevy features used

`big_space` + `FloatingOrigin`; `VisibilityRange`; `RenderLayers` (world plus shadow-only layer per worldspace,
`worldspace.rs:45`); `NoCpuCulling`. Bevy's `RenderAssetBytesPerFrame` limiter is off (`lib.rs:157`: a deferred upload left
stale instance offsets in 0.20).

## Switches and counters

- `--off near` (`statics`, `r1`), `--off far` (`r2`): spawners wrap and skip that level (`mw_devtools_bevy/src/perf.rs:259`).
  `--off nif`, `sizecull` (`exterior.rs:266`), `farfield` (`exterior.rs:217`), `actors`, `walkers`, `terrain`. Others:
  `statics-and-lod.md`. Env `BEVYRROWIND_LOADING_MESHES` (`exterior.rs:1278`).
- Counters: `stream/drive/ms`, `stream/tick/ms`, `stream/<stage>/{items,ms,queue}` (+`bytes` for upload; `stream.rs:972-1010`),
  `stream/{requests,resident_cells,models,model_queue,page_cache/bytes,oldest_request/s,latency/s,shift_guard/frames}`,
  `bigspace/*/ms` (`lib.rs:188`), `world/meshes`, `world/point_lights` (`exterior.rs:1185`). `StreamStats` (`stream.rs:245`).

## Known limits

- Spawn and origin-shift frames spike the render thread, range-slot pressure: [ledger/streaming.md](../../ledger/streaming.md).
- Entity count of near statics without precombine: [ledger/statics.md](../../ledger/statics.md).
- Frame cost: [ledger/perf.md](../../ledger/perf.md).

## Specs

`superfargoth/docs/specs/streaming.md`, `world-coordinates.md`.
