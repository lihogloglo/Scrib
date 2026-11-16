# `queue.submit` cost diagnosis (CW-5, ROADMAP 1.25), 2026-11-16

Read-only: no code change, no build, no run. Sources: code reading (our vendored Bevy, wgpu-core / wgpu-hal
**30.0.1**, the version in `Cargo.lock`) and the per-frame CSVs of existing runs. Backend in every run: Vulkan,
NVIDIA 617.42, `AutoNoVsync` (run `meta`). Paths `wgpu-core/...` and `wgpu-hal/...` mean
`~/.cargo/registry/src/*/wgpu-core-30.0.1/src/...` and `.../wgpu-hal-30.0.1/src/...`.

## 1. Verdict

**The premise is wrong: submit is not constant.** It is a floor of about 0.9-1.2 ms plus about **0.03 ms per
command encoder**. The census mean (1.98 ms) hid two populations: 1.36 ms on frames with ~21 encoders and ~2.6 ms on
frames with 45-65 encoders (lamp faces drawing). The audit's "does not follow content" compared main-view items,
which indeed do not matter; encoders do.

Three mechanisms explain the shape. None is sized individually yet: code reading gives the mechanism and its
scaling, not its milliseconds. One instrumentation slice (§5) splits them.

## 2. What the gauge measures

- `encode/submit` is the **sum of every timed `queue.submit` in the frame** (`sf_diag_bevy/src/encode.rs:67-68, 85,
  108`). Two are timed per frame: the main one with all pending command buffers
  (`vendor/bevy_core_pipeline/src/schedule.rs:316-324`) and a second one for the screenshot/readback encoder, sent
  even when it is empty (`vendor/bevy_render/src/renderer/mod.rs:101-114`). `FlushCommands`
  (`render_context.rs:313-322`) is timed too but nothing calls it.
- It does **not** include `encoder.finish()`: `finish_parallel` is eager and completes before the timer starts
  (`render_context.rs:114-148`; `schedule.rs:310` then `317`).
- It **does** include all of wgpu's deferred housekeeping: `Queue::submit` ends with `device.maintain(Poll)`
  (`wgpu-core/device/queue.rs:1536-1541`), and Bevy never polls the device elsewhere (grep of `bevy_render`: only
  `diagnostic/tracy_gpu.rs:74`). So everything freed or called back after a GPU frame completes is paid here.
- Untimed submits exist (slab growth `slab_allocator.rs:911`, `storage.rs:403`, `gpu_image.rs:194`,
  `gpu_instance_batch/mod.rs:1027`, Solari `blas.rs:158`, DLSS context `super_resolution.rs:96`): grep only, not
  checked for per-frame use.
- The gauge exists only under `--bench`, which also turns on `RenderDiagnosticsPlugin`
  (`mw_devtools_bevy/src/perf.rs:168`). Submit has never been measured without GPU pass timers.

## 3. New evidence from existing runs (per-frame CSV regression, no new run)

Median `encode/submit` (ms) by `encode//count` bucket, and a least-squares fit over all frames:

| run | 20 | 30 | 35 | 40 | 45 | 50 | 55 | 60 | fit |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| W `target/perf/f_on/balmora_walk@2560x1440-2` (5631 frames) | - | 2.08 | 2.20 | 2.43 | 2.65 | 2.78 | 2.97 | 3.03 | 1.24 + 0.0295 x encoders, r 0.66 |
| S `f_on/balmora@2560x1440-2` (1443) | - | 1.96 | 2.14 | - | - | - | - | - | too little spread |
| C `w4btput_warm3/census@2560x1440-1` (976) | 1.36 (n 521) | - | - | 2.57 | 2.58 | 2.55 | 2.73 | 2.76 | 0.89 + 0.0295 x encoders, r 0.74 |

- Same slope in an exterior and an interior. The extra encoders in both are lamp-face shadow encoders (best
  content predictor in both: `views/lamp_face/work_items`, r 0.67 W / 0.76 C), so the slope is that of **an encoder
  with one shadow pass that binds the material slab**. Other encoders may cost more or less.
- At the base encoder count the slope alone would give ~0.6 ms (C, 21) to ~0.9 ms (W, 31), leaving a floor of
  ~0.9-1.2 ms. The exterior floor is ~0.35 ms above the interior one.
- In S (GPU-bound) submit follows `render/present/ms` (r 0.57) and falls as `render/acquire/ms` rises (r -0.42).
  Not explained; C is also GPU-bound and has the lowest floor, so back-pressure is not the floor.
- No run records `write_buffer` calls or bytes, passes, native command buffers or live resource counts.

## 4. What `Queue::submit` does (wgpu-core 30.0.1)

Per wgpu command buffer, serial, under the device tracker lock (`device/queue.rs:1375-1523`):
1. `validate_command_buffer`: visits **every** buffer, texture, query set and bind group in that command buffer's
   tracker (`queue.rs:2053-2112`).
2. Opens one more native command buffer "(wgpu internal) Transit" (`queue.rs:1447-1452`; Vulkan
   `wgpu-hal/vulkan/command.rs:122-152`: pop or allocate, `vkBeginCommandBuffer`), runs memory-init fixups, merges
   the command buffer's tracker into the device tracker and records barriers (`command/mod.rs:1069-1091`), closes it
   (`queue.rs:1477`).
3. Deferred query-set resolves (`queue.rs:1437`).

Then once per submit: end the pending-writes encoder (`queue.rs:449-462`), one `vkQueueSubmit` with every native
command buffer (`wgpu-hal/vulkan/mod.rs:1475-1495`, plus a relay-semaphore chain `:1432-1441`), register the
submission, then `maintain`: read the fence, `triage_submissions` drops every encoder of every completed submission
(`device/life.rs:328-350`, upstream's own profiling scope there is named "drop command buffer trackers"), then
buffer-map callbacks (`queue.rs:236-262`).

Native command buffers: each render or compute pass is two (the pass and its "Pre Pass" barrier buffer:
`command/render.rs:2314, 2725, 2765`; `command/compute.rs:675, 935, 956`), plus one "Transit" per encoder.

## 5. Ranked causes

### 1. Per-encoder tracker work, multiplied by the bindless material slab (slope; ~0.6-1.2 ms of the total)
- **Mechanism.** In a render pass every `set_bind_group` merges all texture views of that bind group into the pass
  scope (`command/pass.rs:111-117`, `track/texture.rs:363-370`). Our materials sit in one slab sized at 8192
  resources (`docs/wiki/engine/textures.md:78`); the walk run shows 1 slab, 2613 materials, 2679 images. So every
  encoder with a pass that draws materials (prepass, opaque, each cascade, each lamp face) carries ~2.7k textures in
  its tracker. Submit walks them in step 1, again in the tracker merge of step 2, and a later submit dec-refs each
  when the tracker is dropped. Each encoder also pays its "Transit" buffer and, on drop, a `vkResetCommandPool` and
  one `vkDestroyFramebuffer` per pass (`wgpu-hal/vulkan/command.rs:176-193`).
- **Scales:** per encoder x textures referenced by that encoder. (The same merge also runs inside `finish()`, so it
  is a suspect for `encode/finish_wall` too.)
- **Confidence:** high that the cost is per encoder (measured, two scenes, same slope); medium that the slab is the
  main part of it (code read, not measured).
- **Standard fix.** Few command lists for many small passes: shipped engines record a shadow atlas as one pass or
  one command list per worker, not one per face (the reverse of F1, which pays for large cascades only). Put all
  lamp faces of a frame in one encoder, and stop keeping encoders for lamp-face views with no work. Upstream: the
  tracker cost of large binding arrays is a known wgpu limitation (cited from memory, not re-fetched; search
  gfx-rs/wgpu for "binding array" tracking before patching). A vendored wgpu patch is the last resort (rule 4).
- **Deciding measurement:** the Tracy split below; or an A/B with lamp faces in one encoder against one each, on
  census (expected: frames with lamp work fall from ~2.6 towards ~1.4 ms).

### 2. Deferred destruction paid inside submit (floor; estimated 0.2-0.6 ms, unmeasured)
- **Mechanism.** Every `queue.write_buffer` creates and maps its own staging buffer
  (`wgpu-core/resource.rs:1255-1266`, called from `queue.rs:631-685`); the copy command is recorded at write time
  (`queue.rs:796-840`), so bytes cost nothing at submit. The staging buffer is destroyed when its submission
  completes (`resource.rs:1370-1376`, a `TempResource`, `queue.rs:330-336`), which happens in `maintain`, inside the
  next submit on the render thread. Same for buffers and textures we drop, and for map callbacks.
- **Scales:** per `write_buffer` call (a `vkDestroyBuffer` plus an allocator free), not per byte. 95 call sites in
  vendored Bevy and our crates; the per-view phase buffers scale with views, and 42-48 lamp-face views stay alive
  with no work.
- **Confidence:** high on the mechanism, low on the size (no counter).
- **Standard fix.** One persistent mapped upload ring per frame instead of a buffer per write: wgpu ships
  `util::StagingBelt` and `write_buffer_with`; engines use a per-frame linear upload allocator. And run the
  housekeeping off the bound thread: `Device::poll` may be called from another thread, after which submit's own
  `maintain` finds nothing to do.
- **Deciding measurement:** call `render_device.poll(PollType::Poll)` just before the timed submit, behind an env
  switch, with its own gauge `encode/maintain`. What moves out of `encode/submit` is this cause plus the tracker
  drops of cause 1. Add a `write_buffer` call and byte counter in the same slice.

### 3. Native command buffer count and the driver's `vkQueueSubmit` (floor; unknown size)
- **Mechanism.** 2 native command buffers per pass + 1 per encoder go into one `vkQueueSubmit`, each with its own
  begin/end; the frame has two timed submits, each with fixed work (submission index, fence read, relay semaphores,
  `maintain`). Exterior frames have more passes than interior ones (water chain, atmosphere LUTs, clouds, froxels,
  cascades, DLSS), which fits the ~0.35 ms higher exterior floor.
- **Scales:** per pass, plus a constant per submit.
- **Confidence:** low on size. It is the only candidate that could hide a driver cost we cannot change.
- **Standard fix.** Fewer, larger passes and command lists (engines submit tens of command lists, not hundreds);
  fold the tail readback encoder into the main submit (one submit per frame); upstream wgpu has a TODO to reuse the
  last native buffer instead of a new "Transit" one per command buffer (`queue.rs:1370-1372`).
- **Deciding measurement:** the Tracy split: wgpu already has scopes `prepare`, `process command buffer`,
  `vkQueueSubmit`, `cleanup`, `triage_submissions`, `drop command buffer trackers`
  (`queue.rs:1352, 1366, 1376, 1536`; `vulkan/mod.rs:1489`; `life.rs:332, 344`). One traced census run and one walk
  run with wgpu's `profiling` scopes routed to Tracy give the share of all three causes at once. Check first that
  our `tracy` feature turns those scopes on; if not, that is the instrumentation slice.

## 6. Killed or parked candidates

| candidate | status | evidence |
|---|---|---|
| `encoder.finish()` hidden in the gauge | killed | finish is eager, before the timer (§2) |
| `write_buffer` / `write_texture` bytes flushed at submit | killed | copies recorded at write time; only the call count matters (cause 2) |
| indirect-draw validation | killed | Bevy removes `VALIDATION_INDIRECT_CALL` (`vendor/bevy_render/src/settings.rs:144`) |
| debug labels / validation layers | killed | release default flags are empty after that removal (`wgpu-types/instance.rs:268-274`) |
| draw or item count | killed | C 524 items; no item counter leads the regression |
| present / acquire wait counted as submit | killed for the floor | separate gauges (0.49 / 0.09 ms W); the S correlation stays open |
| GPU timers (`RenderDiagnosticsPlugin`) | open, likely small | adds query resolves at submit and a map callback in `maintain`; never measured off. One arm with the plugin skipped decides |
| DLSS plugin, readbacks (shelter, occlusion) | not checked | grep only; their map callbacks would land in `maintain` (cause 2's measurement covers them) |
| live resource count (bind groups, buffers, textures) | partly | tracker sets are sized by device-wide texture count per command buffer (`queue.rs:1380`); small next to cause 1 |

## 7. Proposed slice (one, instrumentation only)

"Submit split": (a) `encode/maintain` gauge with the poll-before-submit switch, (b) `write_buffer` calls and bytes
per frame, (c) passes and encoders per frame, (d) one arm without `RenderDiagnosticsPlugin`. Then one interleaved
`perf suite --arm` on census and the walk. It decides causes 2 and 3 and bounds cause 1; the lamp-face
one-encoder A/B is the follow-up if cause 1 holds. Expected ceiling if all three are fixed: about -1.0 ms render
on the walk (submit to ~1.3 ms); the poll move alone is "banked" off the render thread, not removed.

**Built (CW-5b, 2026-11-16, not measured; exe `target/profiling/cw5b.exe`).** Gauges: `encode/poll` (ms, only with the
switch), `encode/queue/{submits,command_buffers}` and `encode/queue/{write_buffer,write_buffer_with,write_texture}/
{calls,bytes}` (every call through `RenderQueue`, timed or not; counted by vendored `RenderQueue` methods shadowing
`wgpu::Queue`'s). (a) is named `encode/poll`, next to `encode/submit`. (c) encoders only: `command_buffers` is one per
encoder; passes are not counted (56+ direct `begin_*_pass` sites; wgpu's internal counters need its `counters` feature, a
new Bevy build). Switches: `BEVYRROWIND_POLL_BEFORE_SUBMIT=1`, `BEVYRROWIND_NO_GPU_TIMERS=1` (no
`RenderDiagnosticsPlugin`: every `render/*/elapsed_gpu|cpu` column and the GPU-ms summary columns are empty, meta
`gpu_timers: off`, the DLSS-timed check is skipped). A/B, one interleaved invocation, census + balmora_walk, 3+ repeats:
`--arm base: --arm "poll:--env BEVYRROWIND_POLL_BEFORE_SUBMIT=1" --arm "nogpu:--env BEVYRROWIND_NO_GPU_TIMERS=1"`, read
`encode/submit` + `encode/poll` against `encode/queue/command_buffers` and `write_*/calls`, and render busy (poll arm:
the sum `submit + poll` should hold; the split sizes cause 2). Cause 1's lamp-faces-in-one-encoder arm is not cheap
the way `BEVYRROWIND_CASCADE_ENCODERS` was: each lamp face is its own root view running the `Core3dShadows` schedule
(`camera_driver`, `bevy_core_pipeline/src/schedule.rs:249-262`), so every recording system opens one encoder per face.
Sharing one needs a render-world encoder that `shared_shadow_pass` (`bevy_pbr/src/render/light.rs:3788`) records into
across views, pushed into `PendingCommandBuffers` right after the last auxiliary view and before any camera (shadow maps
before the passes that read them): ~60-80 lines in two vendored files, and it serialises those faces' deferred
`finish()` into one task (watch `encode/finish_critical`). Not built.

## 8. Not done

Stopped at the brief's ~100k-token limit. Not read: wgpu's user-facing `Queue::submit` wrapper, the gpu-allocator
free path, Bevy's diagnostic recorder buffers, DLSS and our readback plugins. No upstream issue or PR was fetched;
every upstream reference above is from the source of our version or from memory and says which.
