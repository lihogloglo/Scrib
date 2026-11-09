# S3a handoff: bevyengine/bevy#25978 (`GpuBatchedMesh3d`) in vendored Bevy 0.20

Agent session 2026-11-09, static-scene.md slice S3a. Branch `static-scene/s3a`. Session file
`.sessions/s3a-gpu-batch.md`.

## What was done
- Port of PR #25978 commit `36e79e21` (head `917cd5fb` merges main), `bevy_pbr` + `bevy_render` hunks only, into
  `superfargoth/vendor/`. Every hunk carries `// superfargoth patch (bevyengine/bevy#25978)` (new files: first line);
  `grep -rn "bevy#25978" superfargoth/vendor` lists them. Combined diff (vendored 0.20 + earlier patches -> this port):
  `superfargoth/vendor/patches/bevyengine-bevy-25978-gpu-batched-mesh.diff`. Port shape and deviations:
  `superfargoth/vendor/README.md` "The GPU-batched mesh port"; spec note in static-scene.md §6 "Port shape".
- Files: new `bevy_pbr/src/gpu_instance_batch/{mod.rs, materialize.rs, materialize.wesl, gpu_instance.wesl}`,
  `bevy_pbr/src/render/unpack_ranges.wesl`; changed `bevy_pbr/src/{lib.rs, material.rs, prepass/mod.rs, wireframe.rs}`,
  `bevy_pbr/src/render/{gpu_preprocess.rs, light.rs, mesh.rs, mesh_preprocess.wesl}`,
  `bevy_render/src/batching/{gpu_preprocessing.rs, mod.rs}`, `bevy_render/src/occlusion_culling/mesh_preprocess_types.wesl`,
  `bevy_render/src/render_phase/mod.rs`, `bevy_render/src/render_resource/sparse_buffer_vec.rs`.
- Not vendored: `bevy_mesh`, `bevy_camera`, `bevy_light` (no `Mesh3dVisibility`; see README). Not ported: example,
  migration guides, `assets/` shader.
- Tests: `superfargoth/crates/sf_world_bevy/src/gpu_batch_tests.rs` (dev-deps `naga`, `wesl`).

## Build / test status
All through `cargo_turn.sh`, shared target dir, default features (Delirium on):
- `check --workspace --all-targets --locked`: passed (`Finished`, no warnings).
- `test -p sf_world_bevy -p sf_diag_bevy --features sf_diag_bevy/views --locked`: sf_diag_bevy 11 passed; sf_world_bevy
  18 passed (incl. the 3 `gpu_batch_tests`; the first run failed only on the WGSL struct-name lookup, fixed, rerun).
- `clippy -p sf_world_bevy --all-targets --no-deps -- -D warnings`: clean (vendored crates not linted, by rule).
- Sanity tests: (1) `materialize`, `unpack_ranges`, `unpack_bins`, `allocate_uniforms`, `reset_indirect_batch_sets`,
  `mesh_preprocess` x 4 def sets (direct, frustum, occlusion early/late), `build_indirect_params` x 5 link (wesl) and
  validate (naga); (2) `GpuMeshInstance` / `MeshCullingData`: Rust `ShaderType` size = WGSL size = `size_of`, default
  culling data active, zeroed instance inactive; (3) `MinimalPlugins` + assets + `GpuInstanceBatchPlugin` (no render
  app, no GPU), 3 updates: the batch carries the `Mesh3d` class and no `Mesh3d`, sits once in every cascade of a
  shadow-mapped sun, not in a shadows-off sun; a `NotShadowCaster` batch is excluded. Point/spot path not unit-tested.
- **Shared target dir hazard (found here):** cargo hashes path crates relative to the workspace root, so the vendored
  `bevy_render`/`bevy_pbr` of this worktree and of `repo` (main) share artifact names in `repo/target`; a build from
  the other tree can leave an artifact that this tree's mtime fingerprint treats as fresh (the first test build here
  linked an unpatched `bevy_render`). Fixed by touching the vendored sources; until this merges, sessions on main
  building into the same dir may see the reverse (touch their `superfargoth/vendor/bevy_{render,pbr}/src` files).

## For S3b (APIs, limits)
- Main world: spawn `GpuBatchedMesh3d { mesh, max_capacity }` + `MeshMaterial3d<M>` (never `Mesh3d`), add
  `GpuInstanceBatchPlugin` (not in `PbrPlugin`; needs GPU culling support, warns otherwise). Requires `NoFrustumCulling`;
  `NoCpuCulling` removes it from every view (warned).
- Render world: `GpuInstanceBatchReservations::get(main_entity)` -> `output_buffer()` (`max_capacity` x
  `GpuMeshInstance` = 64 B: `world_from_local: [Vec4; 3]` (transposed affine), `is_active`, `tag`, pad). The producer
  writes it in a `RenderGraph` system `.in_set(GpuInstanceBatchSystems::Publish)`; materialization runs right after.
  The buffer is replaced (contents copied) when `max_capacity` changes: rebind on identity change.
- Every frame materialization copies all `max_capacity` slots (dispatch capacity/64); inactive slots (`is_active == 0`)
  are skipped in preprocessing. Capacity is input-buffer space reserved every frame: size batches to need.
- No motion vectors: the template's `previous_input_index` is `u32::MAX` (spec §7 work for S4). No transmission,
  no per-instance sorting (a transparent batch sorts as one item at the entity's transform + mesh AABB center), no 2D.
- Shadows: the batch joins every shadow-mapped directional cascade and every face of every visible shadow-mapped
  point/spot light (`NoFrustumCulling` rules; GPU culling per view does the rest). The #26161 point/spot skip hash
  sees the batch as one stable entity: GPU-side content changes do NOT invalidate cached lamp faces (S5 item: feed a
  generation counter into the hash). `BinnedRenderPhase::entity_count` counts one per batch.
- Culling bounds: one `MeshCullingData` per batch from the mesh AABB, applied per instance in local space.

## Owner checks (no `test-pass-2026-11-10.md` on this branch)
- None visual for S3a by itself: nothing in the game spawns a `GpuBatchedMesh3d` yet. Regression check for the shared
  path the port touches (always on): one normal exterior + one interior run at High with DLSS, look for missing or
  flickering meshes, shadows, transparent sorting (merged sorted items now carry empty `batch_range`), wireframe debug.
