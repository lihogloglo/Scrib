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
On a shift frame `sync_origin` (`exterior.rs` ~:942) puts `O_new - O_old` on the camera's `OriginRebase` (vendored
`bevy_render`, WGSL `View::origin_rebase`); "previous = current" motion-vector writers (`mesh_preprocess.wesl` for GPU
batches, terrain and impostor prepasses) add it, so TAA/DLSS keep their history (FLICKER-1; before, every shift reset it:
the ~256 m pop of ST-2). A grid change still resets; `--off shiftrebase` restores the reset. Log line `origin shift`.

**Rings.** Defaults (`rings.rs:81`): active 192/256 m, near 768/896, far 3000/3250 (load/unload). With a far field,
`far_streaming_config` (`sf_lod_bevy/src/lib.rs:136`) derives near/far from the handoffs (`exterior.rs:269`);
`--near-ring/--far-ring` override (`exterior.rs:1276`).

**One placement into entities.** `spawn_item` (`stream.rs:1353`) spawns the far proxy, then the precombine buckets (see
`statics-and-lod.md`), then 16 placements per item into a hidden batch under a hidden group under the cell. A
`PlacementSpawner` is picked by class (`spawn.rs:123-186`): the game registers light, uncooked-NIF and actor spawners
(`exterior.rs:727-745`); statics use `StaticSpawner` (`spawn.rs:303`). `lod_live::spawn_levels` (`lod_live.rs:210`) makes **one
entity per LOD level per mesh part**: `Mesh3d`, `MeshMaterial3d`, local `Transform`, `VisibilityRange`, worldspace
`RenderLayers`, `Aabb` + `NoAutoAabb`, `StaticPart`, `LodPart`, `ChildOf(batch)` (`lod_live.rs:222-234`); `NoCpuCulling` when GPU-culled
(`lod_live.rs:246`); ray-tracing variants are not used in exteriors (`exterior.rs:1207`).

**LOD ranges.** `chain_ranges` (`spawn.rs:412`) maps per-level projected error to switch distances plus an end band;
`visibility_range` (`spawn.rs:431`) makes dithered `VisibilityRange` fades; `shared_range` (`spawn.rs:468`) rounds bounds up
to a log grid (32 steps per octave) because Bevy 0.20 gives each distinct range a never-freed GPU slot (65535 max). A
placement ends at the far handoff if far-eligible, else its ring's unload radius, shortened by size culling
(`Ends::of` `stream.rs:1756`, `size_culled_end` `spawn.rs:396`).

**Reveal.** A cell group stays `Visibility::Hidden` until `reveal` in the `Upload` stage (`stream.rs:1575`); `RevealGate`
(`reveal.rs`) is what loading and door fades wait on.

**Worldspace left behind (CW-9, on by default (owner 2026-11-17)).** Indoors the exterior stays resident: the residency-cache ghost
(`scheduler.rs` `linger`) keeps its full rings while the player stands within 32 m of the way back (a small interior:
the whole visit), and far-field L2/L3 nodes never leave. Without parking it all keeps ticking (visibility, big_space,
character layers, walkers, far-field pass). With `stream.interior.disable_exterior` on, `park::park_worldspaces`
(`sf_world_bevy/src/park.rs`, `Update` after the fade) parks a worldspace the player left once it had no streaming
request for `stream.interior.park_settle_s`: root `Visibility::Hidden` (extraction drops its instances and lights;
Bevy 0.20's render sync ignores `Disabled`), next frame `Disabled` on root + subtree. Unparked (same ids and state) when
it becomes active (the door cut, before the hold), gets a request, the switch goes off, or an entity is parented under
it (`wake_on_parent` observer). Never-entered prefetch targets don't park. `Allow<Disabled>` in subtree layer/range
hooks, `lod_live`, `persist_actor_stats` (saves), teardown.

## Bevy features used

`big_space` + `FloatingOrigin`; `VisibilityRange`; `RenderLayers` (world plus shadow-only layer per worldspace,
`worldspace.rs:45`); `NoCpuCulling`. Bevy's `RenderAssetBytesPerFrame` limiter is off (`lib.rs:157`: a deferred upload left
stale instance offsets in 0.20).

## Switches and counters

- `--off near` (`statics`, `r1`), `--off far` (`r2`): spawners wrap and skip that level (`mw_devtools_bevy/src/perf.rs:259`).
  `--off nif`, `sizecull` (`exterior.rs:266`), `farfield` (`exterior.rs:217`), `actors`, `walkers`, `terrain`. Others:
  `statics-and-lod.md`. Env `BEVYRROWIND_LOADING_MESHES` (`exterior.rs:1278`). `--off shiftrebase`: origin shifts reset
  temporal history instead of rebasing motion vectors (FLICKER-1 A/B).
- Selection gate (CW-2, on by default (owner 2026-11-17), walk -0.44 ms; `sf_world/src/select_gate.rs`): `Scheduler::update`
  skips gather + decide (`scheduler.rs:762`) while no effective anchor moved `stream.select.move_m` (4 m, UE5
  `LocationQuantization`) or turned `stream.select.turn_deg` (10 deg) and `Scheduler::revision` (cell moves finished,
  worldspaces, links, reload) is unchanged; linger, door prefetch, async starts and burst end stay per tick, a reload
  always runs. `set select.gate on` (live; alias `BEVYRROWIND_SELECT_GATE=1`) turns on all three gates (terrain, far field), `--off selectgate` forces off.
- Parking (CW-9, on by default (owner 2026-11-17), not measurable on the census bench): `set stream.interior.disable_exterior on` (live both ways,
  bound in `mw_exterior_bevy/src/quality_presets.rs` `add_gates`), `stream.interior.park_settle_s` (0.25 s). Counters
  `stream/parked/{worldspaces,entities}`, `stream/park/ms`; log `sf_world_bevy: parked|unparked <ws>: N entities`.
- Counters: `stream/drive/ms`, `stream/tick/ms`, `stream/<stage>/{items,ms,queue}` (+`bytes` for upload; `stream.rs:972-1010`),
  `stream/{requests,resident_cells,models,model_queue,page_cache/bytes,oldest_request/s,latency/s,shift_guard/frames}`,
  `bigspace/*/ms` (`lib.rs:188`), `world/meshes`, `world/point_lights` (`exterior.rs:1185`). `StreamStats` (`stream.rs:245`).
- Unbudgeted work gauges (2026-11-22, ms a frame): `stream/{poll_fetches,poll_models,promote,assets_ready}/ms` (`Streaming::drive` outside
  the five stages, `stream.rs` `drive`), `scene/load/{chains,plan,wanted}/ms` (`sf_scene_bevy` `poll_cell_loads`),
  `character/finish/{ms,uploads,count}` (`sf_character_bevy` `finish_builds`), `assets/cutout_decode/{count,ms}`
  (`mw_assets::texture_is_cutout`; seeded from the bake's `texture.alpha_class.index` at start, so it counts only
  references the bake never saw).
- Budgets of that work (2026-11-22, live tunables bound in `exterior.rs`, **off by default**: A/B inconclusive):
  `stream.scene.load_ms` (default 50 = no limit, candidate 1 ms; `poll_cell_loads` steps cells until it is spent, at
  least one a frame, x burst factor; `sf_scene_bevy::SceneLoadBudget`, gauge `scene/load/deferred`),
  `character.finish.per_frame` (default 256 = no limit, candidate 2; `finish_builds`, `CharacterFinishBudget`).
  Model textures load `RenderAssetUsages::RENDER_WORLD` (`mw_assets::image_settings_clamped`): extraction moves their
  pixels instead of cloning them, and the main world keeps no copy; anything that `get_mut`s one fails to re-extract.
- Live view distance (T5c, `sf_world_bevy/src/lod_live.rs`): a `LodScreen` change (`lod.bias`, `lod.size_cull.*`, `Live`)
  re-ranges resident LOD chains in place, `lod.live.chains_per_frame` a frame, after `drive`; no restream, no cell, actor or
  item touched. Details: `statics-and-lod.md` "Switches and counters".

## Known limits

- Spawn and origin-shift frames spike the render thread, range-slot pressure: [ledger/streaming.md](../../ledger/streaming.md).
- Walk stutters on the Balmora night walk (2026-11-22): no hitch > 25 ms left, still ~17-20 stutters/min (render image
  upload bursts, ~3 ms per cell reveal in `prepare_meshes`, main mesh bursts); the 2026-11-22 lines of the same ledger.
- Entity count of near statics without precombine: [ledger/statics.md](../../ledger/statics.md).
- Frame cost: [ledger/perf.md](../../ledger/perf.md).
- Parking (CW-9): a looping voice under a parked root keeps playing at its last gain (`bevy_audio` sinks are not paused
  yet); parked actors' AI, animation and timed effects pause (their stats still save: `persist_actor_stats`).
- **Worldspace lifetime (fixed 2026-11-15, door-crash):** a worldspace that is fully streamed out (not active, nothing resident or pending, no cell) is released at the end of each streaming update (`Streaming::release_unloaded`, `stream.rs`): `WorldspaceRegistry::release_in` (`worldspace.rs`) despawns its root and returns its render-layer pair to a FIFO free list (refused while the root holds more than its proxy); its index directory stays. Runtime layers use the growing constructor (`dynamic_layer`), so > 31 worldspaces loaded at once only allocate (one-time warning "render layers past 63 spill"). Before: pairs were never freed and `RenderLayers::layer(64)` panicked after ~31 interiors passed (ROADMAP 1.1). Open: per-worldspace physics worlds may never be freed (`sf_physics world.rs:979`); never-entered prefetched roots still tick (left ones park with CW-9).

## Specs

`superfargoth/docs/specs/streaming.md`, `world-coordinates.md`.
