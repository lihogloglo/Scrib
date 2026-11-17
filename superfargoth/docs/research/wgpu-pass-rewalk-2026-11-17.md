# Why a render pass walks a bind group's textures more than once (wgpu-core 30.0.1, 2026-11-17)

Read-only slice A of the wgpu-walk lane (ledger perf 2026-11-17 "Texture-walk ceiling": the town walk pays >= 1.8 ms
of render thread for the texture count of the material slab, in one encoder at `finish()`). Sources: wgpu-core 30.0.1
(crates.io) and our vendored Bevy 0.20. Counts below are per render pass and per bind group of N texture views; the
numbers a frame really pays come from the counters of slice B, not from reading.

## 1. Where the walks are

Render pass commands are recorded and replayed in `encode_render_pass` (`command/render.rs:2288`), which runs inside
`CommandEncoder::finish()`. Each pass has its own `PassState` (`command/pass.rs:44`): a usage scope, a binder, and a
borrow of the encoder's trackers and memory-init lists. Three places walk every texture view of a bind group:

| # | site | when | per texture |
|---|---|---|---|
| 1 | `pass::set_bind_group`, `command/pass.rs:111-117`: `state.scope.merge_bind_group(&bind_group.used)` -> `TextureUsageScope::merge_bind_group` (`track/texture.rs:363-372`) -> `merge_single` -> `insert_or_merge` | **every** `SetBindGroup` command of a render pass, no check for "already merged" | bounds check, metadata bit, usage union + conflict check |
| 2 | `flush_bindings_helper`, `command/pass.rs:144-189`, loop `:159-166`: `texture_memory_actions.register_init_action(action)` (`command/memory_init.rs:55`) for each of `bind_group.texture_init_actions` | at **every draw**, for each valid group at index >= the binder's rebind start (`:145-146`) | a read lock on the texture's `initialization_status` (an atomic write on a line shared by every encoder finishing in parallel), `check_action`, `discards.retain` |
| 3 | pass end, `command/render.rs:2743` `insert_barriers_from_scope` -> `set_from_usage_scope` + `drain_transitions` (`command/mod.rs:1031-1066`) | once per pass | once per **unique** texture of the scope |

Site 3 is already once per pass. Sites 1 and 2 repeat.

The rebind start (site 2) is the lowest index touched since the last draw (`command/bind.rs`, `BoundBindGroupLayouts`):
`assign` on every `SetBindGroup` (`update_rebind_start_index(index)`), `update_expectations` on a pipeline whose layout
differs (first index whose bind group layout changed), and index 0 when the immediates size changes
(`Binder::change_pipeline_layout`). `take_rebind_start_index` resets it to "nothing", so a draw with no change walks
nothing: that is the fast path the wgpu maintainers pointed at in gfx-rs/wgpu#9738. Every group from the start index
**up** is flushed again, so a group at a high index is re-walked whenever a lower one changes.

## 2. What Bevy does to trigger them

- Indices (`bevy_pbr/src/material.rs:79,703-706`): view 0, view binding arrays 1, mesh 2, **material 3**. The slab is
  the highest group, the worst place for site 2.
- `TrackedRenderPass::set_bind_group` (`bevy_render/src/render_phase/draw_state.rs:182-213`) drops a set only when the
  same bind group with the same offsets is already at that index. `set_render_pipeline` (`:165-173`) keeps bind group
  state.
- So, per pass, for the slab at index 3:
  - **site 1** runs once per time the slab is set again after another material group took index 3 (another material
    type's slab or a non-bindless material between two StandardMaterial batch sets);
  - **site 2** runs once per draw that follows any of: the slab set again; the mesh group (index 2) changing
    (`SetMeshBindGroup`, `bevy_pbr/src/render/mesh.rs:4815`: keyed by metadata slab, morph slab, skinned or not,
    lightmap slab); a pipeline whose layout differs at index <= 3 (skinned / morphed / lightmapped mesh layouts,
    another material's layout).
  - Upper bound: once per batch set of the pass for each site; lower bound: once.
- Every pass that binds the slab pays its own first walk of all three sites: main opaque, prepass, transparent, each
  shadow view (unless `render.depth_only.skip_material`).

## 3. Why skipping repeats is exact

- Site 1: a bind group is immutable and the scope only grows during a render pass (compute passes remove entries per
  dispatch; render passes never do). Merging is a per-subresource union with a validity check on the result; merging
  the same uses again leaves the state unchanged and it was valid, or the pass already failed. So a second merge of
  the same bind group into the same pass scope is a no-op.
- Site 2: `register_init_action` (a) appends the still-uninitialised ranges to the encoder's init list, where a
  duplicate is redundant (applied in order at submit; the second finds nothing left), and (b) removes matching entries
  from the encoder's discard list and returns them for an immediate clear. Discards are only added when a pass starts
  (`RenderPassInfo::add_pass_texture_init_actions`, `command/render.rs:1178-1187`), never during one, so within a pass a
  second registration of the same bind group finds nothing in (b) either. Same for its buffer init actions. The
  hardware `set_bind_group` that follows in the same loop must still run on every rebind.
- Not exact across passes of one encoder (a pass in between may discard an attachment that a bind group samples), so
  the memo is per pass, not per encoder.

## 4. Smallest patch (slice B)

Vendored wgpu-core 30.0.1, `command/pass.rs`, behind a process-wide switch that is off by default (off = stock code
path, one atomic load per pass):

1. `PassState` gains two small sets of bind group addresses (the encoder's stateless tracker keeps every bind group
   alive for the encoder's life, so an address is unique within a pass), filled only in render passes with the switch on.
2. `set_bind_group`: skip `scope.merge_bind_group` when the bind group is already in the first set.
3. `flush_bindings_helper`: skip the buffer and texture init-action loops when the bind group is already in the
   second set; keep the acceleration-structure actions and the raw `set_bind_group`.
4. Counters (relaxed atomics, per bind group, not per texture): merges done and skipped, init registrations done and
   skipped, texture entries walked by each. With the switch off they are the measured "how often".

Left after the patch, per pass that binds the slab: one merge, one init registration, one pass-end walk. If the
counters show the main pass already sits near that floor, the patch recovers little and the remaining lever is the
texture count itself (texture arrays bucketed by size and format, or fewer passes binding the slab).

## 5. Encoder names

The per-system encoder columns are blank (`encode//sum`): system names are empty without Bevy's `debug` feature, so
"the critical encoder is the main pass" was an inference. Slice B names an encoder after the first render pass begun
on it when the system name is unavailable (vendored `bevy_render` `render_context.rs`, M3 encode timers; the name
without `debug` is the placeholder `<Enable the debug feature to see the name>`, which the counter code cut to
nothing at its `<`). Result, one stock run of the town walk: `main_opaque_pass_3d` 2.05 ms is the longest, `early
prepass` 2.03 and the sun cascade encoder 1.73 follow closely and finish in parallel. So the inference was half
right: the main pass is the critical encoder, but two more encoders pay nearly the same walk.

## 6. Texture arrays: one view is one entry (verified by reading, no design yet)

A bind group records one `(view, usage)` pair and one init action per **view** it binds
(`device/resource.rs:3185-3212`, `create_texture_binding`), whatever the view's layer count. Site 1 keys the scope by
the view's parent texture (`track/texture.rs:367-368`) and stays on the cheap single-state path when the view covers
the whole texture; site 2's `check_action` loops over the view's mips, not its layers (`init_tracker/texture.rs:65-90`).
So a `D2Array` view of N layers costs what one 2D texture costs today at all three sites: bucketing the slab's
textures into arrays by size and format divides every walk by the bucket fill. Only worth designing if the counters
show the per-pass first walks (not the repeats) are what is paid.

## 7. Measured (slice B, ledger perf 2026-11-17 "walked once")

Town walk bench, per frame, slab of ~2.55 k textures, stock -> switch on:

- site 2 (init registration): 568 bind groups -> 454 (110 skipped), 78.8 k -> 12.0 k texture entries. Stock walks the
  slab ~31 times a frame here; this is where the cost was.
- site 1 (merge): 183 -> 97 (85 skipped), 17.8 k -> 11.9 k texture entries (~7 slab walks -> ~4.7).
- ~4.7 walks of each kind remain: one per pass that binds the slab.
- Longest encoder `finish()` 2.40 -> 1.09 ms, all encoders 10.70 -> 7.35 thread-ms, render thread -1.37 ms, frame
  -1.15 ms; the one-texture ceiling gives -1.55 / -1.30. Submit unchanged (2.31 -> 2.33).
