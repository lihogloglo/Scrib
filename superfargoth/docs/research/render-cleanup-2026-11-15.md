# Render `Cleanup` 3.3 ms: a gauge artifact (2026-11-15)

Diagnosis only (session `render-cleanup`): no code, no runs. Data: `bevyrrowind/benchmarks/runs/b2_on/
balmora_walk@2560x1440-{1,2,3}.md`. The build uses the vendored `superfargoth/vendor/bevy_render` (root
`Cargo.toml:324`), wgpu-core 30.0.1.

## 1. What the 3.3 ms is

**Verdict: not Cleanup work. About 2.8-3.0 ms of it is `render_system` (graph encode, submit, present) booked
to the wrong stage; real Cleanup is at most ~0.3 ms.**

Facts:

- `render_system` is one exclusive system in `RenderSystems::Render`: it runs the `RenderGraph` schedule, the
  final submit and `present` (`vendor/bevy_render/src/renderer/mod.rs:84-132`, added `lib.rs:468-471`).
- Its wall is measured directly by `phases.rs:104-105` (two `ResMut<Clock>` systems, serialized around it):
  `render/graph` + `render/present`. Run 2: 6.465 + 0.489 = **6.95 ms**. The stage gauges give
  render 4.14 + cleanup 3.12 = **7.26 ms** for the span that contains it. So everything else in that span
  (`process_pipeline_queue_system`, every Cleanup system, the markers) is **≤ 0.31 ms** (runs 1 and 3, from
  `camera_driver + graph_submit + present`: ≤ 0.38, ≤ 0.37).
- `stage/render/render` (4.14-4.47) is smaller than what runs inside it (`camera_driver` 1.1 + `graph_submit`
  5.3): impossible unless the Render end stamp is missing on part of the frames.
- Render and cleanup have the same p95 in every run (9.08 / 9.04, 8.56 / 8.52, 8.66 / 8.44): one quantity
  landing in either bucket. Run 1's worst frame: `graph_submit` 66.8 ms (`encode/submit` 63.2) shows up as
  cleanup max 69.2 while render max is 25.5.
- What Cleanup holds: Bevy's `reset_render_asset_bytes_per_frame` (`lib.rs:472`), `expire_*_specializations_
  for_views` (`camera.rs:111-112`), `map_buffers` (`gpu_readback.rs:72`), `clear_changed_shader_buffers`
  (`storage.rs:46`), `update_texture_cache_system` (`texture/mod.rs:42`), one in `bevy_pbr/render/mesh.rs:231`,
  `reset_mip_generation_jobs`, UI `clear_batches`; ours are counters only (`sf_diag_bevy` `phases.rs:106`,
  `churn.rs:100`, `pipeline_watch.rs:79,104,129`, `textures.rs:294`, `sf_character_bevy/crowd.rs:82`,
  `mw_devtools_bevy/pipeline_watch.rs:46`). `despawn_temporary_entities` is in `PostCleanup`, outside the gauge.
- `tcd_trbase.trace.txt`: no Cleanup system, drop or despawn span in the top self-time list (cut-off ~0.17 ms);
  `present_frames` 0.34 ms.

Inference (mechanism, from the code; rate not measured):

- The sets are chained with `chain_weak` (`lib.rs:329-347`): order is kept only between conflicting systems
  (`bevy_ecs-0.20.0/src/schedule/config.rs:449-457`). The Render marker (`stages.rs:176`) is strictly after the
  whole Render set, so it waits for `submit_end`; `report` (`stages.rs:181`) waits only for Cleanup systems,
  and nothing orders it after the markers. After `submit_end`, the marker and `frame_end` are spawned
  together as pool tasks; `report` follows `frame_end` and often runs first. It then reads the Render stamp as
  unset (delta 0, `stages.rs:81-83`) and Cleanup absorbs the whole Render set. Loss rate ≈ 1 − 4.14 / 7.0 ≈ 40%.
- The same race can drop any earlier stamp (its time moves to the next stage). The stage **sum** is right;
  the per-stage split is soft.
- "Markers block nothing" is not strictly true: an exclusive system cannot start while any task is pending
  (`bevy_ecs-0.20.0/src/schedule/executor/multi_threaded.rs:533`), and the render thread does not tick the
  pool itself (`:274-275`, `false`). Each marker ahead of `render_system` or a sync point adds a pool hop.

## 2. The standard, and where real cleanup cost would show

- Industry: deletion is deferred and batched off the hot path. UE queues RHI resource deletes and flushes them
  on the RHI thread; transient targets come from pools (RDG); per-frame data lives in frame-in-flight ring
  buffers. Decima and RAGE do the same with N-frame delayed frees.
- Bevy/wgpu already has this shape. `TextureCache` pools transient textures (`update_texture_cache_system`).
  wgpu frees a dropped resource only once the submission that used it is done, in `Device::maintain`, which
  `Queue::submit` calls (`wgpu-core-30.0.1/src/device/queue.rs:1348`, `:1541`). So per-frame resource churn is
  paid inside `queue.submit` on the render thread, i.e. in **`encode/submit` (2.3-2.5 ms)**, not in Cleanup.
  The standard answer there is persistent bind groups and buffers (we do it for preprocess, P4) and fewer
  submits.
- Upstream: `gh search` on bevyengine/bevy (three queries on Cleanup / submit perf) returned nothing in this
  session; no issue number to cite. Not settled.

## 3. Ranked fixes

| # | Fix | Expected untraced ms | Size | Risk |
|---|-----|----------------------|------|------|
| 1 | Gauge: order `report` after every marker (markers in one set, `report.after_ignore_deferred(set)`), or take the Render end from `phases.rs`' serialized clock | 0 frame gain; cleanup reads ≤ 0.3, render ≈ 7.0 | ~5 lines in `stages.rs` + the `completion_deltas` test | none |
| 2 | Drop Cleanup as a perf target. The render thread's cost is `render_system` 6.95 = `camera_driver` 1.1 + `graph_submit` 5.3 (`finish_wall` 3.0, `submit` 2.3) + present 0.5 | the 2.6 ms over target is here | separate slice | - |
| 3 | A/B the 17 markers' own cost (pool hops before exclusive systems) | unknown, guess ≤ 0.3 | a switch that skips `stages::render::add` | none |
| 4 | Only if cleanup still reads > 0.5 after fix 1: trace and sum the Cleanup systems listed above | - | - | - |

## 4. Measurement to confirm

After fix 1, the same walk, 3 runs untraced. Pass: `stage/render/cleanup` ≤ ~0.3 ms; `stage/render/render` ≈
`render/graph` + `render/present` + ~0.1; the two p95s no longer match; cleanup max no longer follows
`graph_submit` max. No `bevy/debug` or traced run is needed for this question. For fix 3: `render/thread/ms`
with and without the stage markers, 3 runs each (under ~0.5 ms is noise).
