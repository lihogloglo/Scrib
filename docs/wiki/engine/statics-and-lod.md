# Statics: precombine, far field, static scene, GPU-only culling

## What it does

Three ways draw the static world, by distance. Near cells (inside the far-field handoff) draw from a baked **precombine**
(a few merged meshes per cell) or, by default since 2026-11-17 (owner; `--off scene` = precombine), from the **static scene** (retained GPU instance batches). Past the handoff the
**far field** draws HLOD proxies (L1/L2/L3), impostors and light sprites. Separately, an opt-in switch makes static parts
skip Bevy's CPU culling (`NoCpuCulling`). Streaming and the per-placement fallback path are in `streaming.md`.

## How it works

**Precombine (default near path).**
- Bake: `sf_lod::near` merges a cell's eligible statics per material and LOD level into cooked models, one per size
  bucket (`BUCKET_EDGES` 0.5, 2, 8 m radius, `sf_lod/src/near.rs:21`); nothing is simplified or re-textured. The game's
  veto (`FarSource::precombine_exclude`, `mw_bake` `far.rs`) keeps out statics a script names and breakable statics (IN38c3).
- Load: a near `Load` request starts loading the cell's `CellPrecombine` asset when `config.precombine`, a far field is
  installed and the worldspace is not ray-traced (`sf_world_bevy/src/stream.rs:807-818`). The path comes from `Precombines`
  that `FarFieldPlugin` installs (`sf_lod_bevy/src/lib.rs:186-193`, product `NEAR_STEP`, loader `NearCombineLoader`).
- The fetch waits for the asset (`stream.rs:849`). `valid_members` (`stream.rs:1773`) checks every member is present and
  untouched by the overlay; otherwise the whole precombine is dropped and the cell spawns per placement
  (`precombine_fallbacks`, `stream.rs:869, 908`). Members skip model loading and visuals but still activate.
- Spawn: `spawn_precombine` (`stream.rs:1400`) builds one template per bucket and one batch entity. Each bucket ends at its
  farthest-reaching member (the handoff for far-eligible members, else the member's end plus the bucket `reach`; `lod_live::bucket_end`) and uses
  `bucket_ranges` (`precombine.rs:60`: every LOD switch pushed out by `reach`). Parts spawn through
  `lod_live::spawn_levels` (`lod_live.rs:210`), so each bucket is still one entity per LOD level per part.

**Far field (`sf_lod_bevy`).** `FarFieldPlugin` (`lib.rs:163`) inserts `FarProxies` with handoff `levels.handoff(1)` (default
handoffs 400 / 2000 / 8000 m times the preset bias, `sf_lod/src/levels.rs:72`) and runs after streaming `Drive`
(`lib.rs:207-217`): `load_directories`, `build_impostor_arrays`, `spawn_upper_nodes`, `residency_pass`, `update_materials`.
L1 nodes ride the streaming far ring through `CellProxies` (`runtime.rs:203`, `FarHook`); L2/L3 are resident per worldspace
(`runtime.rs:129, 530`). Eligible placements are `Static | Door | Light` plus custom classes carrying `PlacementFlags::FAR_PROXY`
(`sf_world/src/placement.rs:135`; adapter sets it on scriptless and `GhostfenceScript` `ACTI`s, `mw_world/src/source.rs:278`).
Opaque proxies are `Mesh3d` + `ProxyMaterial`; `AlphaMode::Blend` submeshes (grouped by quantised emissive and double-sidedness
at L1/L2, dropped at L3, `sf_lod/src/build.rs:150`) draw with `BlendProxyMaterial` (forward `Blend`, `NotShadowCaster`,
`FarShadowExt` + `TranslucentFog`, `sf_lod_bevy/src/materials.rs:117`). Both carry `VisibilityRange` crossfades from lead bands
(`levels.rs`, `FadeRange`). A far-eligible placement hands off only if its L1 node draws it: `NodeHeader::unrepresented` (`sf_lod/src/build.rs`) lists models the node draws nothing for (policy `Drop`, no cook, boundless particle models), and `Ends::inputs` (`sf_world_bevy/src/stream.rs`) keeps those to their own ring. Blended sources made of back-to-back triangle pairs are folded first (`cluster::fold_back_faces`), else the weld collapses them. `residency_pass` (`runtime.rs:570`) forces a parent visible while any child that should draw is
not resident (no holes).

**Impostors (`ImpostorMaterial`, `sf_lod_bevy/src/materials.rs:59`).** One mesh per far node holds its impostor cards:
alpha-masked, forward-shaded, with their own prepass shader. The alpha test runs once, in the camera's depth prepass
(BW-2c, 2026-11-26): in a view with a depth prepass `Material::specialize` (`materials.rs:100`) sets the main pass to
depth-equal with no depth write (`depth_equal_main_pass`, `depth_equal.rs:46`) and the def `DEPTH_EQUAL_MAIN`, under which
`impostor.wesl` neither alpha-tests nor range-dithers. Prepass and shadow pipelines and views without a depth prepass
keep Bevy's state. It holds because both vertex stages mark the position `@invariant` and the camera prepass writes no
`frag_depth` (`shader_tests.rs`). Switch: `far.impostor.depth_equal` (`lib.rs:105`, default on, live; off = alpha test in
both passes). Counter `far/impostors/no_prepass` (`depth_equal.rs`, `count_unprepassed`): parts with a ready main-pass
pipeline and no ready prepass pipeline, which the equal test would leave undrawn; expected 0, a warn line otherwise.

**Static scene (default since owner 2026-11-17; `--scene` is a no-op).** `static_paths` (`mw_exterior_bevy/src/exterior.rs:1405`): scene needs no
`--off scene`, and no `--off near/nif`, no `--lod-tint`; it turns the precombine off (never both). `ScenePlugin`
(`sf_scene_bevy/src/lib.rs:64`) installs `SceneCells(SceneHook)` when enabled (`lib.rs:191`); the game also adds the vendored
`GpuInstanceBatchPlugin` (`exterior.rs:302`). Flow: baked per-cell tables (`sf_scene`, `SceneTableLoader`) load through the
hook's `acquire/ready/covered/upload/release`; `covered` placements get no model load and no
entity. A committed cell plan writes instance slots into retained `GpuBatchedMesh3d` batches (`residency.rs:280`) via
`publish_scene` (`upload.rs`), always in the space's own frame (`SpaceState::frame`); when the active worldspace's origin cell differs from its space's frame, `rewrite_on_origin_shift` rewrites every resident cell's slots and moves the frame (`shift.rs`, `OriginFrames`: a worldspace not drawn keeps its frame); a streamed-out cell writes every slot it held inactive (`queue_release_writes`, `CellRelease::freed_blocks`), because `batch_ranges` lends free slots to the neighbour range; batch entities are top-level and carry their worldspace's content `RenderLayers` (`sync_batch_layers` on a registry change, none while the worldspace is unregistered), so a worldspace's scene draws only in its own views; the world teardown calls `reset_scene` (`residency.rs`; `mw_exterior_bevy/src/teardown.rs`); a promoted placement writes
inactive slots and spawns a real entity (`PromoteToEntity`, `stream.rs` `accept_promotion`); a hidden one (`Streaming::hide_covered`, IN38c1) writes the same inactive slots with no entity. Culling and LOD of batch ranges is done by the
vendored Bevy GPU preprocess, not by a scene compute pass: the scene-buffer upload
is still `TODO(B4b)` (`residency.rs:228, 827`). Without GPU culling the scene turns itself off (`SceneSupport`, `lib.rs:111`).

**GPU-only culling (`NoCpuCulling`, opt-in).** `BEVYRROWIND_GPU_CULL_STATICS=1` (and no `--off cpuculltag`) sets
`StreamingConfig::gpu_cull_statics` (`exterior.rs:282`); `StaticGpuCulling` (`spawn.rs:289`) is forced off when the adapter
lacks GPU culling (`sf_world_bevy/src/lib.rs:167`). Then `Static`-class parts and precombine buckets carry `NoCpuCulling`
(`lod_live.rs:246`, `stream.rs:1432`); actors, doors, items and lights keep CPU culling. The GPU preprocess culls them by frustum
and visibility range in the camera and the sun cascades (persistent views; every LOD level stays binned there). Lamp faces
take no GPU-culled meshes (`CpuListedPointShadowCasters`, `vendor/bevy_pbr/src/render/mesh.rs:2503-2533`, inserted by
`PointShadowPlugin`): `list_gpu_culled_casters` (`sf_sky_bevy/src/gpu_culled_casters.rs`, runs only while a `NoCpuCulling`
mesh exists) adds each shadowed lamp's GPU-culled casters to its `CubemapVisibleEntities` from a per-lamp list rebuilt only
when the lamp or a GPU-culled caster changes.

## Bevy features used

`VisibilityRange`, `NoCpuCulling`, GPU preprocessing and indirect draws; vendored patches `vendor/bevy_pbr`
(`gpu_instance_batch`, bevy#25978 port) and `vendor/bevy_render` (`SharedImageSamplers`); static materials fold into classes
in `sf_world_bevy/src/matclass.rs`.

## Switches and counters

- `--off precombine` (`exterior.rs:1271`), `scene`, `matclass`, `cpuculltag`, `farfield` (`lod`), `near`/`far`/`nif`; flags
  `--off scene` (precombine), `--lod-tint`; env `BEVYRROWIND_GPU_CULL_STATICS`.
- Far-field selection gate (CW-2, on by default (owner 2026-11-17); walk -0.44 ms, ledger perf 2026-11-17): the residency pass (`far/select/ms`) is skipped
  (`sf_lod_bevy/src/runtime.rs:614`) while the camera stays within `far.select.move_m` (2 m) and the far state, the
  scheduler's revision and the settings are unchanged, plus one settle frame after any change (new parts get their
  `GlobalTransform` in PostUpdate); while on, nodes need their children 2 m early (guard band, `runtime.rs:622`) so
  the no-hole fallback never lags. Live tunable `select.gate` (alias `BEVYRROWIND_SELECT_GATE=1`), `--off selectgate`.
- `StreamStats`: `precombined`, `precombine_fallbacks`, `scene_covered`, `scene_upload_bytes` (`stream.rs:245-270`).
- Scene gauges (`sf_scene_bevy/src/gauges.rs`): `scene/cells/resident`, `scene/batches/count`, `scene/slots/batch_live`,
  `scene/slots/batch_capacity_bytes`. Far field: `far/*` (`FarFieldStats`). Per-view cost: `views/*`, see `frame-and-views.md`.
- View-distance tunables (T5a, the quality level's at the `Preset` layer, `mw_exterior_bevy/src/quality_presets.rs`): `lod.bias`,
  `lod.size_cull.min_px` (`--off sizecull` = 0), `lod.size_cull.min_distance_m` are `Live` (T5c): `Streaming::set_lod_screen`
  (`sf_world_bevy/src/stream.rs:528`) and resident LOD parts (placements and precombine buckets, each with a `LodPart` holding its
  chain's inputs) re-ranged in place by `lod_live::apply_lod_screen` (`lod_live.rs:283`), `lod.live.chains_per_frame` (4096) chains
  a frame; collapsed levels despawn, opened levels spawn; `StreamStats::lod_live_*`, counters `stream/lod_live/{chains,ms}`. Not
  live: uncooked NIF subtrees, `--scene`, and `far.handoff_scale` (d1 = baked 400 m x scale; near rings sized from it): `Restart`.

## Known limits

- Static scene: parity with the precombine on the walk, -0.22 ms standing and ~6x fewer walk hitches on the walk-once base; default since owner 2026-11-17: [ledger/statics.md](../../ledger/statics.md).
- `NoCpuCulling` is opt-in (walk stutters, lamp work items; with lamp faces fixed it still loses: more batch sets in the
  camera and cascades, H1 2026-11-15): [ledger/statics.md](../../ledger/statics.md).
- Precombine falls back to per-placement entities when a member moved or was removed: [ledger/statics.md](../../ledger/statics.md).
- Frame totals: [ledger/perf.md](../../ledger/perf.md).

## Specs

`superfargoth/docs/specs/precombine.md`, `far-field.md`, `static-scene.md`, `scene-runtime-plan.md`.
