# Static scene perf audit (2026-11-11, session scene-audit)

Question (owner): why is `--scene` slower than the entity + precombine path, and is RO-18 (floating foliage) ours?
Evidence: owner A/B `bevyrrowind/benchmarks/runs/scene_{off,on}`; traced runs `target/perf/au_tr_{off,on,on32k}`
(profiling-trace exe, `balmora` standing, 2560x1440, DLSS Quality, `--trace`; `au_tr_on32k` = batch capacity 32768
through a temporary env override, reverted). Traces are ~3.5 GB each (C:), delete when done.

## Results

| run | median ms | entities | meshes | overflow inst. | sub-draws/frame | work items/frame |
|---|---:|---:|---:|---:|---:|---:|
| off (entities + precombine) | 11.94 | 47.0 k | 24.6 k | - | 0 | 14.8 k |
| `--scene`, 4096 slots (default) | 13.48 | 56.7 k | 34.5 k | 10.4 k | 12.3 k | 95.7 k |
| `--scene`, 32768 slots | 13.76 | 37.3 k | 15.0 k | 0 | 29.3 k | 664.6 k |

Fixing the overflow alone does **not** make the scene faster: it removes 19k entities (main thread -0.3 ms vs off),
but the per-frame cost of the batch path grows with it. Timed GPU ms is flat; the frame is render-thread and Extract bound.

## Root causes, in cost order (trace self time per frame, `au_tr_on32k` vs `au_tr_off`)

1. **Ranges are per block, not per chain** (`sf_scene::SceneSpace::batch_ranges`, space.rs ~631: one range per
   `(cell, chain, level)` block). ~1.6k live chains become thousands of ranges per batch, and every consumer is
   O(ranges) **every frame**:
   - `prepare_gpu_batched_mesh_draws` **1.71 ms** (vendor `bevy_pbr/src/gpu_instance_batch/mod.rs` ~617-720:
     `resolve_gpu_ranges` re-resolves every range against `MeshAllocator` + `RenderMaterialBindings` each frame);
   - `extract_gpu_batched_meshes` **0.64 ms** in Extract (~485-590: rebuilds and copies every range list each frame,
     no change detection);
   - `write_work_item_buffers` + `indexed_cpu_metadata` **0.74 ms** (one sub-draw per range per view and phase:
     29.3k per frame).
   Industry: cached mesh draw commands rebuilt on change only (UE `FMeshDrawCommand` caching, Unity SRP/BRG batcher),
   one draw per mesh+material, instances grouped per mesh. Fix: (a) change detection in the port (extract on
   `Changed<GpuBatchedMesh3d>`, re-resolve only when a slab moved or a binding changed); (b) ranges per chain level:
   one contiguous region per chain level per batch, cells sub-allocated inside it (or compact on change).
2. **`extract_chain_slabs` answers every wanted mesh and material every frame** (`sf_scene_bevy/src/slabs.rs`,
   **1.14 ms**, inside Extract so it stalls both threads). Its own docs say a slab id never changes while the mesh
   lives. Fix: answer only ids not yet resolved (and on `dirty`); skip the system when nothing is pending.
3. **Batch capacity 4096 is ~5x too small for Balmora** (21.7k static instances resident; the S0 test sized it for
   4k). 4 batches (`PerClass`): 10-12k instances overflow and spawn as entities **without precombine** (the
   interlock turns it off), i.e. both paths run. The overflow warn re-arms after each warning
   (`SceneSpace::take_overflow_warning` takes `first_overflow`, the next overflow sets it again): 883 lines.
4. **Dispatch width = reserved capacity x views** (664k work items at 32k slots): GPU waste today, and why a bigger
   capacity can't be the whole fix. This is the parked S3a3 compaction (spec "Per-view output memory").
5. Smaller: `publish_scene_gauges` 0.14 ms/frame (`Last`).

Not a cause: CPU culling, per-frame slot uploads (writes happen at stream-in only), materialization (0.08 ms GPU).

## Correctness findings

- **Lamp shadows (suspected, `--scene` only, needs an owner night check):** `sf_shadow_cache_bevy` tracks casters
  `With<Mesh3d>` only (lib.rs ~252) and filters cube faces to its tracked sets, so batch statics are likely dropped from
  cached lamp shadows (lamp-face items 1196 -> 364 in the owner A/B; `views/lamp_face/instance_batches` 0 at max), and a
  cell streaming its statics into a batch never triggers a recapture. Its system order vs
  `add_gpu_batched_meshes_to_light_visibility` is also unspecified.
- **No size culling, no LOD, no handoff end on the scene path** (S3a3 parked, spec §4): every batched static draws LOD 0
  to its cell's residency edge, past the far-field handoff, overlapping the far proxies in that band (GPU only).
- What the table holds: `STAT` statics only (`mw_bake/src/scene.rs` ~240); doors, containers, lights, items, actors stay
  entities on both paths. Far-field HLOD (proxies, impostors) is unchanged by `--scene`.

## Independent review (Fable 5.2, read-only, same day): corrections

- All nine claims hold in substance (CONFIRMED: 2, 3, 4, 7, 9; PARTLY: 1, 5, 6, 8). Corrections: ~5.86k ranges in
  total (~1.5k per batch, ~3.6 per chain, ~3.3 instances per range), not "thousands per batch"; resident statics are
  19.2k, not 21.7k; overflow is **per-class skew** (two batches full, two near empty), ~2.3-3.6x short, so capacity
  must be per class, not a uniform 32k; dispatch width costs ~+0.3 ms GPU at 32k (the real cost is ~115 MB output memory).
- Frame model: frame ≈ Render + Extract on both paths (off 10.53 + 1.44, 32k 11.26 + 2.51). The gap is Extract
  +1.07 and Render +0.73, so summed self times (~4.2 ms) overstate the gain. Traces rank costs; only untraced 3x A/Bs
  give ms (trace overhead ~2.3-2.7 ms, uneven between paths).
- `extract_chain_slabs` takes `ResMut<MainWorld>`, so it also serializes against every `Extract<>` system.
- Untraced, the old path standing is near GPU-bound (9.26 ms frame, 8.23 ms GPU): CPU fixes bring the scene path to
  about parity there, not a win. The walk (+2.78 ms, over the 11 ms target) was never traced: trace it before S3a3.
- The scene path's measured win so far is pacing: stutters 28/min (off) vs 8 (4096) vs 0 (32k).
- Lamp shadows: no recapture on stream-in confirmed; "dropped" depends on the unordered pair of systems. Owner night
  check still needed.

## Fixing plan (ordered by ms per unit of work; untraced 3x standing + walk A/B after each slice)

1. `extract_chain_slabs` answers only unresolved ids and runs only when something is pending (re-check on material
   `AssetEvent::Modified`). ~1.0 ms (Extract). No visual risk.
2. Change detection in the vendored GS port: keep `ExtractedGpuBatchedMeshes`, re-extract on `Changed<GpuBatchedMesh3d>`;
   cache resolved ranges and spans, re-resolve only when a list changed, a `MeshAllocator` generation bumped (public
   counter in vendored `bevy_render`) or bindings changed (UE cached mesh draw commands). ~0.7-1.5 ms. Guard stale
   offsets with the generation plus a debug-build recheck.
3. Per-class batch capacity from measured residency plus headroom; fix the overflow warn-once. Overflow 0, ~19k fewer
   entities. After 1-2.
4. One range per chain level (start by merging adjacent same-chain blocks): 5.86k → ~1.64k sub-draws per phase.
5. Lamp-shadow correctness: order `classify_lights` against `add_gpu_batched_meshes_to_light_visibility`; a
   game-agnostic "static casters changed (sphere)" message from scene residency to the shadow cache. Before `--scene`
   can be the default.
6. Un-park S3a3 (GPU count, allocate, scatter per view with size cull, LOD and handoff end at the spec's pixel
   thresholds): the only slice that buys GPU time and memory. Large; visuals need the owner's sign-off.

Default stays the old path until 1-3 land and an untraced A/B shows parity or better.

## RO-18 (floating foliage): not reproduced under control

Seen in both paths (owner A/B), so not the scene. BRP checks on the dev build (no `--scene`): after a vertical origin
shift, 14,649 meshes moved by -256 m as they should; camera-following meshes (precip, ocean, kickup, terrain patches)
stay at 0 by design. During the owner's own flight ~129 new meshes sat at camera height and were despawned before
inspection. A scripted climb at Balmora (40 -> 700 m, 40 m/s, `tools`: BRP `bevyrrowind/set` camera, F11 free camera)
found none: impostors, proxies and light sprites at the right heights. Ruled out by reading: wind shaders (none on
foliage), impostor/proxy node frames (bake and runtime both node-relative), streaming cells (directory cells, 3D
distances). Next: a capture script running while the owner flies the route where it shows (start, direction, height).
