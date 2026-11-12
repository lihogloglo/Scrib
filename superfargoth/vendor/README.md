# Vendored third-party crates

Patched copies of crates.io crates, wired with `[patch.crates-io]` in **both**
`superfargoth/Cargo.toml` and `bevyrrowind/Cargo.toml`, excluded from the superfargoth
workspace (not ours to lint or test). Every changed hunk carries a `superfargoth patch`
comment; `grep -rn "superfargoth patch" vendor/<crate>` lists them.

| Crate(s) | Base | Patch | Drop when |
|---|---|---|---|
| `big_space` | crates.io release | dirty-tree pruning in low-precision propagation (`docs/research/perf/README.md` item 2) | upstream has it |
| `bevy_core_pipeline`, `bevy_pbr`, `bevy_render` | 0.20.0 (crates.io) | bevyengine/bevy [#26149](https://github.com/bevyengine/bevy/pull/26149) + [#26161](https://github.com/bevyengine/bevy/pull/26161) (`patches/bevy-26149-26161.diff`, both PRs' stacked diff at head `4e718df0`). **Own fork since 2026-10-30**: both PRs were closed unmerged (below) | when upstream solves per-face shadow scheduling and per-view skipping, by any means |
| `dlss_wgpu` | 6.0.0 (crates.io) | NVIDIA-free facade: same API, NGX calls moved to the runtime-loaded `plugins/sf_dlss` DLL through `src/abi.rs` (GPL game exe holds no NVIDIA code) | never while the game is GPL; re-port on each dlss_wgpu bump that Bevy needs |
| `bevy_pbr` (own, on top of the row above) | same | `superfargoth patch (own: shadow early-out)` + `(own: lamp shadow filter)` (`patches/superfargoth-shadow-early-out-lamp-filter.diff`, 44+/9- lines, shadow reset B2) | early-out: Bevy skips the point/spot shadow fetch where the light contributes nothing; filter: Bevy gets a per-light-type (or per-light) shadow filter |
| `bevy_core_pipeline`, `bevy_pbr` | same | bevyengine/bevy [#26235](https://github.com/bevyengine/bevy/pull/26235), [#26236](https://github.com/bevyengine/bevy/pull/26236), [#26237](https://github.com/bevyengine/bevy/pull/26237) (LT-4 DLSS cluster lookup, transmission UV, and OIT indexing with `MainPassResolutionOverride`) | upstream Bevy release |
| `bevy_render`, `bevy_pbr` | same | bevyengine/bevy [#26321](https://github.com/bevyengine/bevy/pull/26321) (`ExtendedMaterial` bindless fallback when extension does not support bindless) | upstream Bevy release |
| `bevy_render`, `bevy_pbr` | same | bevyengine/bevy [#26095](https://github.com/bevyengine/bevy/pull/26095) (clear out stale bin unpacking & uniform allocation jobs and bind groups; since P4 its bind-group clear in `prepare_preprocess_bind_groups` is replaced by #26418's cache, the job clear stays) | upstream Bevy release |
| `bevy_pbr` | same | bevyengine/bevy [#26234](https://github.com/bevyengine/bevy/pull/26234) (`patches/bevy-26234.diff`: SSAO textures, dispatch and UVs at `main_pass_viewport`, i.e. the DLSS internal resolution; RO-11 lead) | upstream Bevy release |
| `bevy_core_pipeline`, `bevy_pbr` | same | bevyengine/bevy [#26170](https://github.com/bevyengine/bevy/pull/26170) (`patches/bevy-26170.diff`: occlusion culling clamps depth-pyramid texel loads at the right/bottom edges) | upstream Bevy release |
| `bevy_pbr` (own) | same | `superfargoth patch (own, SH-20)`: `ShadowMoverRefresh` (redraw close-up mover/animated shadow faces per frame within 16 m of camera, `--off movershadows` A/B; SH-20b: per-light refresh, moved lights redraw whole) | upstream gets dynamic caster refresh |
| `bevy_pbr` (own) | same | `superfargoth patch (own, H2)`: World-Unit Directional PCSS (`shadow_sampling.wesl`, `shadows.wesl`: world-meter penumbra scale, analytic receiver slope, 4-tap sunlit early-out, and far-cascade rotated PCF fast path; filter taps fixed by RO-11 below) | upstream gets cascade-invariant world-unit PCSS |
| `bevy_pbr` (own) | same | `superfargoth patch (own, RO-11)`: DLSS-correct contact shadows/SSR (`ssr/raymarch.wesl`, `ssr.wesl`, `render/pbr_functions.wesl`: depth march in `main_pass_viewport` pixel space, viewport UV mapped into the target-sized depth/color textures and clamped to the rendered rect) and sun PCF receiver-plane fix (`render/shadow_sampling.wesl`: full slope + half-texel margin on every filter tap, slope exact to N·L 0.125). No diff file; grep the marker | upstream contact shadows/SSR honour `MainPassResolutionOverride` (raymarch half); with H2 (shadow half) |
| `bevy_solari` (own) | 0.20.0 (crates.io) | `superfargoth patch`: `prepare_raytracing_scene_bind_group` builds the scene bind group whenever the TLAS has instances, lights or not (probe GI, `sf_gi_bevy`, traces lightless interiors: emissive + bounce only); `realtime/node.rs` and `pathtracer/node.rs` read `lit_bind_group()` so Solari's own lighting still skips lightless scenes as before; `RaytracingSceneBindings::{instance_count, light_count}` (read-only, GI status log). 3 files, ~25 lines | upstream builds the scene bind group without lights and exposes scene counts |
| `bevy_pbr` (own) | same | `superfargoth patch (own)`: `AtmosphereStaticLutCache` (`atmosphere/{mod.rs,resources.rs,node.rs}`: caches static `transmittance_lut` and `multiscattering_lut` per GPU texture pair when planet/medium are unchanged) | upstream caches static atmosphere LUTs |
| `bevy_pbr` (own) | same | `superfargoth patch (own, P1: live medium scales)`: `ScatteringMediumScales` component (per-term scattering/absorption multipliers, `MAX_MEDIUM_TERMS` = 8; `medium.rs`), extracted into `ExtractedAtmosphere` (`atmosphere/mod.rs`); medium LUTs hold each term's unit falloff (4 terms per texel) and phase (a row per term, `MediumLutData::bake`), coefficients x scales go to `GpuAtmosphere` / WGSL `Atmosphere` (`resources.rs`, `types.wesl`), summed per term in `functions.wesl` `sample_density_lut` / `sample_scattering_lut`; scales join the `AtmosphereStaticLutCache` key. P1b: the per-term sum runs once per scale change on the GPU (`medium_lut.wesl`) into stock combined LUTs, sampled with one fetch. Weather no longer re-bakes the 256² medium LUT on the render thread (4.3 ms, ~51 times a minute) | upstream makes medium coefficients live (UE-style scattering/absorption scales) |
| `bevy_pbr` (own) | same | `superfargoth patch (own, GI)`: DDGI probe sampling (`render/gi_probes.{rs,wesl}` new; `MeshPipelineKey::GI_PROBES` 1<<26 + `MeshPipelineViewLayoutKey::GI_PROBES` 1<<17, mesh-view bindings 39-42 in `mesh_view_bindings.{rs,wesl}` + `GiProbeVolume` in `mesh_view_types.wesl`, view key/def in `mesh.rs` and `deferred/mod.rs`, shader lib in `lib.rs`, indirect diffuse in `pbr_functions.wesl`). A view with a `GiProbeVolumeView` render-world component (filled by `sf_gi_bevy`) samples its irradiance/visibility atlases in place of the irradiance-volume/env-map diffuse inside the volume. No diff file; grep the marker | upstream Bevy gets a pluggable diffuse-GI provider (or DDGI) on the raster path |
| `bevy_pbr` (own) | same | `superfargoth patch (own, SW)`: surface weather (surface-weather.md W3). `render/surface_weather.rs` new (`SurfaceWeatherView(SurfaceWeatherBindings)` render-world view component, binding consts 43-53; layout and bind-group entry helpers shared with the prepass); `MeshPipelineKey::SURFACE_WEATHER` 1<<27 + `MeshPipelineViewLayoutKey::SURFACE_WEATHER` 1<<18, view key/def in `mesh.rs`, layout + bind group entries in `mesh_view_bindings.rs` (uniform of unknown layout, cells, shelter as unfilterable float, noise, filtering + non-filtering samplers, canopy as unfilterable float: W2b, cover layer array as filterable 2D array + filtering sampler at 50-51: W5a, trail ring as filterable 2D at 52: W6a; last frame's ring at 53 and every SW binding visible to the vertex stage too: W6b), `render/pbr.wesl` imports `sf_surface_bevy::view` under `@if(SURFACE_WEATHER)` and calls `apply_surface_weather_standard` before lighting. W6b, `prepass/mod.rs`: views with the component get `MeshPipelineKey::SURFACE_WEATHER` in their prepass key, prepass pipelines then take two extra view layouts (motion vectors or not, + bindings 43-53) and the `SURFACE_WEATHER_PREPASS` def, `prepare_prepass_view_bind_group` puts a `SurfaceWeatherPrepassBindGroups` on those views and `SetPrepassViewBindGroup` binds it (raised snow displaced identically in depth, motion vectors and color; shadow views never carry it). The provider (`sf_surface_bevy`) declares the WGSL bindings and owns the resources; bevy_pbr knows nothing of their layout. No diff file; grep the marker. ~200 lines in 6 files | upstream Bevy gets a view-level material hook (global shader bindings + a fragment extension point for every `StandardMaterial`) |
| `bevy_pbr` (own) | same | `superfargoth patch (own, N1: ambient specular toggle)`: `AmbientLightSpecular(bool)` resource (`render/light.rs`, extracted; registered in `lib.rs`; default `true` = stock) sets `AmbientLightFlags::DIFFUSE_ONLY` (bit 1 of the view's `ambient_light_flags`) in `prepare_lights`; `render/pbr_ambient.wesl` drops the flat ambient's specular under `AMBIENT_LIGHT_FLAGS_DIFFUSE_ONLY_BIT` (`mesh_view_types.wesl`). Env maps, probes and lights keep theirs. No diff file; grep the marker. ~40 lines in 4 files | upstream: ambient specular toggle (e.g. an `AmbientLight` field) |
| `bevy_pbr` (own) | same | `superfargoth patch (own, P5: shared skin joint blocks)`: `render/skin.rs` `SkinJointBlocks` (pub, in `SkinUniforms::blocks`; exported from `render/mod.rs`) gives skinned meshes with the same binding (inverse-bindposes `AssetId` + joint entities in order) ONE refcounted joint block, so `extract_skins` walks joints and `prepare_skins` uploads them once per block, not per mesh (a modular character = one mesh per material on one palette). Matrices are identical by key equality; previous-frame (motion-vector) matrices sit at the same offset. Upstream main (#23283) still keys per mesh entity; no prior PR/issue. Read by `sf_character_bevy` test `meshes_on_one_skeleton_binding_share_one_joint_block`. No diff file; grep the marker. ~250 lines in 2 files | upstream shares joint buffers between skins with the same binding |
| `bevy_pbr` (own) | same | `superfargoth patch (own, N2: sky glow)`: `AtmosphereSettings::{airglow, airglow_horizon}` (zenith radiance cd/m², horizon/zenith ratio; default zero = stock) copied into `GpuAtmosphereSettings` (`atmosphere/mod.rs`) and the WGSL `AtmosphereSettings` (`types.wesl`); `functions.wesl` `airglow(r, mu)` (van Rhijn profile; ground hits see albedo x the sky's irradiance) added in `sky_view_lut.wesl` and the ray-marched branch of `render_sky.wesl`, so the sky, the atmosphere environment map, fog and clouds (all read the LUT) see it. No diff file; grep the marker. ~35 lines in 5 files | upstream: a sky emission / airglow term in `Atmosphere` |
| `bevy_pbr` (own) | same | `superfargoth patch (own: CPU-listed shadow views)`, statics-cpu-cost slice 1 (`docs/research/statics-cpu-cost-2026-11-12.md`): `collect_gpu_culled_meshes` (`render/mesh.rs`, helpers `takes_gpu_culled_meshes` / `drain_gpu_culled_meshes` above it) puts `NoCpuCulling` meshes only into the shadow subviews Bevy makes (point faces 0..6, the spot view, cascades below the light's cascade count); subviews a plugin injects with its own caster list (`sf_sky_bevy::sun_view` depth views, `sf_shadow_cache_bevy` capture faces) were full-flushed with every GPU-culled mesh on each (fresh) redraw. Opt-in render-world resource `CpuListedPointShadowCasters` (inserted by `sf_shadow_cache_bevy`): point faces take none either, the app lists them in `CubemapVisibleEntities`. No diff file; grep the marker. ~70 lines in 1 file | upstream lets a shadow view opt out of GPU-culled meshes (or lists injected subviews), and point-light caster lists become app-overridable |
| `bevy_render` (own) | same | `superfargoth patch (own)`: incremental `extract_visibility_ranges` (`view/visibility/range.rs`: `O(changed + removed)` instead of clearing and re-hashing all ~28k `VisibilityRange` entities on `ExtractSchedule`) | upstream makes `extract_visibility_ranges` incremental |
| `bevy_render` (own) | same | `superfargoth patch (own: shared image samplers)`, static-scene S2: opt-in render-world resource `SharedImageSamplers` (`texture/gpu_image.rs`: struct + `get_or_create` at the top, `Option<SRes<SharedImageSamplers>>` in `GpuImage::Param`, used in `prepare_asset` for `ImageSampler::Descriptor`): images with equal `ImageSamplerDescriptor`s share one `Sampler` (linear scan, never evicted), so a bindless material slab counts one sampler, not one per texture. Absent = stock (one sampler per image). Inserted by `sf_world_bevy` unless `material_classes` is off (`--off matclass`). No diff file; grep the marker. ~55 lines in 1 file | upstream dedups image samplers (a sampler cache in `GpuImage` preparation) |
| `bevy_render`, `bevy_pbr` | same | bevyengine/bevy [#25978](https://github.com/bevyengine/bevy/pull/25978) (`GpuBatchedMesh3d`: GPU-authored mesh instances drawn through the mesh-material pipeline; upstream PR **open**, ported from commit `36e79e21` against Bevy main; `patches/bevyengine-bevy-25978-gpu-batched-mesh.diff`; marker `superfargoth patch (bevyengine/bevy#25978)`; static-scene.md S3a). Port shape in "The GPU-batched mesh port" below. Our additions on top ("patch GS") are planned as an **own fork**: offered upstream, never waited on | the Bevy release that ships GPU-authored instance batching with per-batch geometry ranges (then drop the light-visibility shim too) |
| `bevy_render`, `bevy_pbr` (own) | same | `superfargoth patch (own: phase counts)` + `(own: preprocess bind group count)`, static-scene S0: read-only `BinnedRenderPhase::{bin_count, batch_set_count}` (`render_phase/mod.rs` ~1444-1480) and the `PreprocessBindGroupCount` render-world resource (`render/gpu_preprocess.rs`: struct and `bind_group_count` helpers ~326-358, `init_resource` ~550, set in `prepare_preprocess_bind_groups`; since P4 it counts cache misses in `created` and hits in `reused`, the `bind_group_count` helpers are gone). Counters only, no behaviour change; read by `sf_diag_bevy::RenderViewsPlugin`. No diff file; grep the markers. ~95 lines in 2 files | upstream exposes phase bin/batch-set counts and a preprocess bind-group count (or stops rebuilding them every frame, bevy #26257) |
| `bevy_render` (own) | same | `superfargoth patch (own: pipeline compile timing)`, pipeline-warmup.md W0: `PipelineCompileTiming` + opt-in `PipelineCache::{record_compile_timings, take_compile_timings}` (`render_resource/pipeline_cache.rs`: four `Instant`s per pipeline task in `start_create_{render,compute,mesh}_pipeline`: spawned, body started, shader/layout locks taken, locks dropped; then after `create_*_pipeline`), and read-only `PrepareNextFrameAssets::{len, is_empty}` (`render_asset.rs`; W1: the same on the erased one in `erased_render_asset.rs`, one material type's retry queue). Not recording = stock. Read by `sf_diag_bevy::PipelineWatchPlugin` and `PendingDrawsPlugin`. No diff file; grep the marker. ~100 lines in 3 files | upstream exposes per-pipeline creation timings (or a pipeline-cache stats API) |
| `bevy_pbr` | same | bevyengine/bevy [#26418](https://github.com/bevyengine/bevy/pull/26418) (fixes issue [#26257](https://github.com/bevyengine/bevy/issues/26257); PR **open**, ported from head `6d46e7df`; marker `superfargoth patch (bevyengine/bevy#26418)`), perf P4: `SceneUnpackingBindGroupCache` keeps the `unpack_bins` / `allocate_uniforms` bind groups across frames keyed on the `BufferId`s they bind, and their metadata binding becomes a dynamic uniform offset passed at dispatch (`render/gpu_preprocess.rs`). Hand-merged into `prepare_preprocess_bind_groups` over #26095's clear; the PR's `wgpu/noop` test module is not carried (vendored crates are never tested here), its 256-byte alignment check is a `const` assert. Own addition: a `created` miss counter on the cache. No diff file; grep the marker | upstream Bevy release |
| `bevy_pbr` (own) | same | `superfargoth patch (own: preprocess bind group memo)`, perf P4 (`docs/research/statics-cpu-cost-2026-11-12.md` slice 2): render-world `PreprocessBindGroupMemo` (`render/gpu_preprocess.rs`): the mesh-preprocessing (direct, frustum, early/late occlusion) and indirect-parameter-building bind groups go through `create_bind_group` memoized on their entire descriptor (layout id + every binding's wgpu buffer/texture view/sampler handle, offset and size; handles held by clone, so no id reuse), entries unused for 2 frames evicted (current/previous input buffers swap each frame). Rebuilt only when a bound buffer is reallocated or a bound size/offset changes (work-item counts). No upstream issue or PR covers these families (searched 2026-11-12). No diff file; grep the marker. ~150 lines in 1 file | upstream caches the preprocess bind groups |
| `navmesh-rs` (own) | 0.2.2 (crates.io; recastnavigation `9f4ce644`) | `superfargoth patch (own: public detached query)`: `DetachedQuery` (`src/detour/query.rs`: was `pub(crate)` and `crowd`-only, for `PathQueue`) public and ungated, with `new` / `attach` / `NavMeshQuery::detach`; re-exported from `detour` (`src/detour/mod.rs`). `sf_nav_recast` keeps each job's sliced A* in its own detached query and attaches it to the mesh per step (nav.md NV2). `patches/superfargoth-navmesh-rs-detached-query.diff`, ~30 lines in 2 files, own code (no GPL-derived lines): to offer upstream under MIT OR Apache-2.0 | navmesh-rs exposes detached (or self-owned, resumable) sliced A* state |

## The Bevy shadow patch (#26149, #26161) — own fork

**Both PRs were closed unmerged upstream on 2026-10-30**: self-closed by their author, branches deleted, zero reviews
between them (the underlying bevy issue #25576 is still open). This row is therefore not "a vendored patch carrying an
upstream PR" (CONTRIBUTING rule 4) but an **own fork we maintain**, and its "Drop when" can only be met by upstream
solving the same problem by some other means. Nothing in `docs/specs/static-scene.md` plans around these PRs merging
(`docs/research/static-scene-architecture-review-2026-11-09.md` B4).


- **#26149** (`Core3dShadows`): point and spot shadow views run a slim schedule (GPU mesh
  preprocessing + the two shadow passes) instead of the whole `Core3d` camera schedule
  per cube face (bevy issue #25576).
- **#26161** (`SkipRootViewRendering`): per point/spot light, a hash of its visible casters,
  its view/projection and the queued shadow-phase entity count; a shadow view whose inputs
  are unchanged (no caster moved, deforms, or changed mesh; no pipeline still compiling)
  is skipped by the camera driver and keeps last frame's map. Directional lights are not
  covered. `bevy_render` only gains `BinnedRenderPhase::entity_count`.

## The GPU-batched mesh port (#25978, static-scene S3a)

- **Source**: commit `36e79e21` (PR head `917cd5fb` is a merge of main), +2,314/-291 over 28 files against Bevy
  main. Only the `bevy_pbr`/`bevy_render` hunks are carried; no example, migration guides or `assets/` shader.
- **Mapping**: our 0.20 files had not diverged in these areas: every hunk applied by context (offsets only)
  except two. `prepass/mod.rs` imports (our SW patch rewrote the list) were redone by hand; one
  `render_phase/mod.rs` hunk (`is_empty` gains `instance_batches`) fuzzed into our S0 `batch_set_count` and was
  moved back. Our S0 counters, #26095 and #26149/#26161 code are kept; `BinnedRenderPhase::{bin_count, entity_count}`
  (S0, #26161) also count instance batches (one bin / one entity per batch).
- **Deviation, no `Mesh3dVisibility`**: upstream adds a `Mesh3dVisibility` class in `bevy_mesh` (required by `Mesh3d`)
  and switches `bevy_camera`/`bevy_light`/`bevy_pbr` lookups to it. We keep `Mesh3d` as the class key so nothing
  else is vendored and our crates that key lists by `TypeId::of::<Mesh3d>()` (`sf_shadow_cache_bevy`, `sf_sky_bevy`
  sun view, `mw_devtools_bevy`) keep working: the 14 hunks that only renamed the key are dropped, `GpuBatchedMesh3d`
  requires `Visibility` + `VisibilityClass` and pushes the `Mesh3d` class in an `on_add` hook,
  `late_sweep_material_instances` also sweeps removed `GpuBatchedMesh3d`, and
  `gpu_instance_batch::add_gpu_batched_meshes_to_light_visibility` (PostUpdate, after `CheckLightVisibility`) adds
  batches to directional cascades and point/spot shadow lists by the rules `bevy_light` applies to a
  `NoFrustumCulling` mesh (shadow maps on, render layers, `NotShadowCaster`, `VisibilityRange`).
- **Opt-in**: `GpuInstanceBatchPlugin` is not part of `PbrPlugin`; the shared-path changes (`MeshCullingData::is_active`,
  `RenderMeshDraws`, merged sorted-phase items get an empty `batch_range`, range unpacking) are always on.
- Tests: `sf_world_bevy` `src/gpu_batch_tests.rs` (shaders link + validate, Rust/WGSL layouts, headless app).
- **"Patch GS" (our additions) is planned as an own fork.** #25978 itself has been `mergeable_state: dirty` /
  `S-Merge-Conflicts` since 2026-10-17, has no milestone, has had one cosmetic review ever, and is tested only on
  macOS/Metal — no occlusion-culling example, no shadow scene, no Vulkan or DX12 run. Our additions (N geometry ranges
  per batch sharing one batch set, `unpack_ranges` in the shadow-view schedules, instance batches in the phase
  counters, per-view level selection) stay in new files under `bevy_pbr/src/gpu_instance_batch/` and separate WGSL
  modules so they remain offerable upstream, but no slice waits on a merge (`docs/specs/static-scene.md` §6,
  `docs/specs/gs-patch-plan.md`).
- **Patch GS as built**, marker `// superfargoth patch (own, GS: <tag>)`, `grep -rn "own, GS:" vendor` lists them.
  Tag `range unpack in preprocess` (slice A, `docs/specs/gs-patch-plan.md` step 1): the range-unpacking dispatch
  moved out of `GpuInstanceBatchPlugin`'s `Core3d` system registration and into `early_gpu_preprocess`
  (`bevy_pbr/src/render/gpu_preprocess.rs`, private `dispatch_range_unpacking`, its own compute pass before
  `early_mesh_preprocessing`), so **every** schedule that preprocesses also unpacks: `Core3d`, `Core3dShadows`
  (#26149) and `sf_shadow_cache_bevy`'s `LampShadowView`. #25978 registered it in `Core3d` only, so the two shadow
  schedules read `push_range`'s `push_multiple_uninit` reservations as uninitialised GPU memory. Offerable to
  #25978 as a review fix. Still a no-op without the plugin (the pipeline id stays `None`).
- Tag `counters` (slice A, plan step 3), **Scrib-specific**, kept separable from the `own: phase counts` /
  `own: preprocess bind group count` bodies of work: `BinnedRenderPhase::batch_set_count` adds
  `instance_batch_draws.len()` (it read only `self.batch_sets`, while instance batches draw from
  `instance_batch_draws` and register their batch sets straight on the indirect buffers, so the one number
  static-scene.md's "batch sets per view" budget gates on reported **zero** extra draws for every static), plus
  read-only `instance_batch_draw_count`, `instance_batch_indirect_draw_count` and `instance_batch_slot_count`
  (reserved, pre-cull: the surviving count is GPU-only). `PreprocessBindGroupCount` gains `ranges` and `materialize`,
  set by `prepare_range_unpacking_bind_groups` and `gpu_instance_batch::materialize::prepare_bind_groups` - both were
  uncounted. Counters only, no behaviour change; read by `sf_diag_bevy::RenderViewsPlugin`, summed in Bevy-free
  `sf_diag::views` (`PhaseCounts::{instance_batches, instance_draws, instance_slots}`, `BindGroups`).
- Tag `guards` (slice A, plan step 4), three of them, **no behaviour change in any view**:
  (a) `mesh_preprocess.wesl`'s `is_active` gate gets its own `@if(FRUSTUM_CULLING)` statement instead of sitting
  inside the frustum-cull block, so the two independent tests stop reading as one. **Defence in depth, not a bug
  fix:** `mesh_culling_data` is bound only under that def, so the gate is unreachable - not bypassed - in a view
  without it, and such a view (`direct_preprocess` ⇔ `NoIndirectDrawing`) gets no instance-batch work items and no
  draw record at all. The real hardware floor below is what statics hit there; the gate becomes def-independent only
  when `is_active` moves into `MeshInput.flags`.
  (b) `GpuInstanceBatchPlugin` `.chain()`s `prepare_gpu_batched_mesh_reservations` before
  `prepare_gpu_batched_mesh_draws` (#25978 left them unordered in a tuple; harmless today, and step 2's per-range
  validation will read the reservation).
  (c) **Hardware floor, documented** on `GpuBatchedMesh3d` and here: GPU instance batches require
  `GpuPreprocessingMode::Culling`. An adapter limited to `PreprocessingOnly`, and a camera or inherited view with
  `NoIndirectDrawing`, draw **none** - `batch_and_prepare_binned_render_phase` and its sorted twin `continue` on
  `no_indirect_drawing` and `push_range` returns `None` on a `Direct` work-item buffer, so the statics are absent
  there, never drawn un-culled. A producer needing something on such an adapter falls back to `Mesh3d` entities
  itself. Our own inherited shadow views (`sf_sky_bevy::sun_view`, `sf_shadow_cache_bevy::render`) only take
  `NoIndirectDrawing` from the camera or from the adapter clamp, so they never individually lose indirect drawing.
  (a) and (c) are offerable to #25978 as review suggestions.
- Tag `ranges` (slice B1, plan step 2a-2d), the data path only: `GpuBatchedMesh3d::ranges:
  Vec<GpuBatchedMeshRange>` (mesh + index sub-range + slot span + optional material per range) with pure
  `validate_ranges` / `range_of` / `batch_spans` next to it; `ExtractedRange` validated once per change on
  extraction (`warn_once` + clear ⇒ upstream behaviour, never a dropped batch); per-frame resolution in
  `prepare_gpu_batched_mesh_draws` into `GpuBatchGeometryRange` (64 B, mirrored by `GeometryRange` in
  `materialize.wesl`) against this frame's slabs and material bindings, rejecting a range that leaves the
  batch's vertex slab, index slab or material bind group (`warn_once`, batch draws nothing that frame);
  a 5th binding on the materialize bind group (per-job `RawBufferVec`, its `BufferId` in `bound_buffers`)
  and a binary search in `materialize.wesl` writing per-instance geometry and culling bounds;
  `RenderMeshInstanceBatch::spans` + `GetFullBatchData::get_instance_batch -> Option<&[InstanceBatchSpan]>`
  in place of #25978's single `Range<u32>`. **`ranges` empty is exactly #25978**: no range table is read
  (`range_count == 0`), and the batch reserves one span over its whole capacity. The drawing half is the
  next entry.
- Tag `ranges` (slice B2, plan steps 2e-2f), the drawing half: `instance_batch_sub_draws(base, output_base,
  spans)` in `bevy_render/src/batching/mod.rs` is the single place a batch's sub-draw layout is decided -
  span *i* takes indirect parameters slot `base + i` and the output slots the first *i* spans do not - and
  the metadata writes, the binned work items and the sorted work items all read it, so they cannot
  disagree about which slot a range draws from. `prepare_instance_batch_indirect` now `allocate`s
  `spans.len()` contiguous slots (still in the first view loop, still through
  `UntypedPhaseIndirectParametersBuffers::allocate`, so the shared `metadata` buffer keeps its
  init-before-uninit order), writes one `IndirectParametersMetadata` per span, and registers the whole run
  as **one** `IndirectBatchSet`: a batch of N ranges is a single `multi_draw_indirect_count` of N
  sub-draws, and a range-less batch is one command in one batch set, byte for byte #25978.
  `get_next_batch_set_index` is read after `allocate` and immediately before `add_batch_set`, with nothing
  pushing a batch set in between. **Sorted phases pass `use_batch_set: false`** and so write no batch set
  index into their metadata: `SortedRenderBatchSet::flush` never draws through a batch set, so their N
  commands are one plain `multi_draw_indirect` and `build_indirect_params.wesl` must keep its 1:1
  batch-to-slot mapping for them (a batch-set index there would make the shader compact the commands into
  slots the sorted draw does not read). The hardware gate is
  `render_phase::multi_draw_indirect_count_supported(features, backend)` - `MULTI_DRAW_INDIRECT_COUNT` and
  **not** DX12 (the wgpu [#7974](https://github.com/gfx-rs/wgpu/issues/7974) workaround) - one function
  now shared by `render_batchable_meshes` (its inline copy removed, same expression) and
  `render_instance_batches`, which drops `batch_set_index` and keeps the indirect parameters range when it
  is false, so the draw falls back to `multi_draw_indirect` over all N commands. **DX12 is a backend we
  ship on, so that is a normal path**; it inherits upstream's #7974 compromise exactly (metadata still
  carries the batch set index, so the shader compacts while the draw issues all N commands - see
  `docs/test-pass-handoffs/gs-slice-b2.md`). `bevy_render::settings` re-exports `Backend` so the gate is
  testable without a GPU. Also `own, GS: guards`: both batching callers now `warn_once` instead of
  silently dropping every instance batch in a `NoIndirectDrawing` view (plan step 4.2's per-view warning).
- Tag `change detection` (perf audit 2026-11-11 fix 2; UE cached mesh draw commands): extraction persists and
  re-extracts a batch only on change or a mesh asset event; `gpu_instance_batch/range_cache.rs` caches resolved
  ranges until a new extraction, a binding move or a `MeshAllocator::changes()` round (`bevy_render`, new
  generation counter) names one of its meshes; debug builds assert hits. Tests: `sf_world_bevy` `gpu_batch_cache_tests.rs`.
- Tag `light visibility set` (perf audit 2026-11-11 fix 5): `add_gpu_batched_meshes_to_light_visibility` runs in the
  public set `gpu_instance_batch::GpuBatchedLightVisibility`, so `sf_shadow_cache_bevy` filters the light face lists
  after the batches joined them (the pair was unordered).

## Upstream PR fixes

- **#26235, #26236, #26237** (`LT-4` DLSS viewport indexing): when `MainPassResolutionOverride` (DLSS)
  is active, fragment shading passes rasterize at `main_pass_viewport` rather than full `viewport`.
  Without these patches, clustered forward light assignment (`clustered_forward.wesl`), transmissive
  refraction sampling (`transmission.wesl`), and order-independent transparency (`oit/draw.wesl`,
  `oit_resolve.wesl`) sample clusters and buffers scaled to the output resolution, producing blocky light
  dropout and misaligned screen-space lookups.
- **#26321** (`ExtendedMaterial` bindless fallback): combines `AsBindGroup::bindless_supported` across base
  and extension materials. If either material does not support bindless, the combined material forces non-bindless
  layout entries and bind group construction.
- **#26234** (SSAO at the main-pass resolution, merged upstream 2026-10-26): with `MainPassResolutionOverride` the GTAO
  textures, compute dispatch, depth/normal UVs and the AO lookup in `pbr_fragment.wesl` / `deferred_lighting.wesl` use
  `main_pass_viewport`; 0.20 ran them at the output size (view-space reconstruction mis-scaled, AO sampled off its pixel).
- **#26170** (occlusion culling at the right/bottom screen edges): `occlusion_culling.wesl` and the meshlet/MSAA
  `downsample_depth.wesl` loads clamp texel coordinates instead of reading out of bounds (never-culled edge objects).
- **#26095** (GPU preprocessing unpacking job clear): clears `SceneUnpackingBuffers` unpacking jobs and
  `BinUnpackingBindGroups` / `UniformAllocationBindGroups` every frame so that stale metadata from prior frames
  or dead views does not cause dangling compute dispatches.

## Our own Bevy patches

### Shadow reset B2 (early-out & lamp filter)
Diffed against the PR-patched sources in `patches/superfargoth-shadow-early-out-lamp-filter.diff`:
- **Shadow early-out** (`pbr_functions.wesl`): point and spot loops evaluate the light first and fetch the
  shadow (and lamp contact shadows) only if `any(light_contrib != 0)`. Range falloff, N·L and the spot cone
  saturate to exactly 0, so skipped fragments are the ones the shadow would have been multiplied by zero: exact.
  Since 2026-11-09 the directional loop does the same (`// superfargoth patch (own: sun shadow early-out)`): the
  cascade fetch and the sun/moon contact-shadow march run only if the light's contribution is non-zero (N·L, and
  the atmosphere transmittance with the sun below the horizon); the diffuse-transmission lobe (`-N`) has its own
  gate, and `lit` is taken after the `DIRECTIONAL_LIGHT_SHADOW_MAP_DEBUG_CASCADES` overlay (never 0: unchanged).
- **Lamp shadow filter**: camera marker `PointSpotShadowsHardware2x2` (`render/mesh.rs`, extracted like other
  camera settings) sets `MeshPipelineKey::POINT_SPOT_SHADOWS_HARDWARE_2X2` → shader def
  `POINT_SPOT_SHADOW_FILTER_HARDWARE_2X2`: point cubes (`sample_shadow_cubemap`) and spot maps
  (`fetch_spot_shadow`) take Bevy's 1-tap 2x2 hardware PCF; directional keeps the camera's
  `ShadowFilteringMethod`. PCSS lights (`soft_shadow_size`) unchanged. Forward/prepass only: the deferred
  lighting pipeline and meshlets don't read the key. Volumetric fog's point/spot lookups share the functions.

### SH-20 Mover shadow refresh (`render/light.rs`)
- Animated characters moving under lamp lights stepped under the B3 rule (which quantized skinned roots on a
  ≥ 20 cm grid and capped refreshes to 2 dirty faces per frame).
- `ShadowMoverRefresh`: split face hashing into `(members, motion)`. Close-up animated casters (within 16 m of an
  active `Camera3d`) redraw their shadow faces the same frame (capped at 4 unbudgeted mover faces/frame), while
  distant street walkers and set changes (streaming, LOD swaps) remain on the coarse grid + LRU budget. Main-world
  resource `ShadowMoverRefresh { per_frame }` (default true; false = B3 rule, toggled via `--off movershadows`).
- SH-20b (2026-11-01, carried torches): refresh decisions are per light, never per face: a light drawn this frame
  draws all its dirty faces (no shadow cut at a cube seam). `ShadowCasterChanges::moved_lights` = transform changed
  and rigid casters' light-relative placement changed (a floating-origin shift changes only the first): such a
  light redraws whole every frame, uncapped, outside every budget. Still lights: `MAX_MOVER_LIGHT_REFRESHES_PER_FRAME`
  = 2 (casters moved, oldest first, overflow joins LRU) and `MAX_LRU_LIGHT_REFRESHES_PER_FRAME` = 1 (set changes,
  minus first captures only). `RUST_LOG=bevy_pbr::shadow_refresh=debug` logs the per-second light counts.

### H2 World-Unit Directional PCSS (`render/shadow_sampling.wesl`, `shadows.wesl`)
- Stock Bevy directional PCSS computes blocker depth and penumbra filter radius in orthographic cascade NDC,
  making penumbra width depend on each cascade's bounding box and receiver NDC depth (jumping visibly across
  cascade splits).
- Adapted from OpenMW Clockwork `files/shaders/lib/light/sunsoft.glsl` @ `c5e5a9a6` (GPL-3.0, Thomas Portal),
  plus rotated blocker kernel from `bevyengine/bevy#22272`.
- Converts blocker height into world meters via `clip_from_world`'s Z row (`per_z = length(row_z)`), computes
  analytic receiver-plane depth slope to avoid derivatives across non-uniform cascade branches, applies an angular
  penumbra spread in world meters across all cascade splits, and adds a 4-tap sunlit precheck + far-cascade
  (`texel_size >= 0.15 m`) 4-tap rotated PCF fast path.

### RO-11 DLSS contact shadows/SSR & sun PCF receiver plane (`ssr/raymarch.wesl`, `ssr.wesl`, `pbr_functions.wesl`, `shadow_sampling.wesl`)
- Owner symptom RO-11 (dark bands on the ground that slide with the camera under DLSS); audit
  `docs/research/shadows/2026-11-01-audit.md` §3. Two defects:
- Depth ray march (contact shadows, SSR) used `textureDimensions(depth)` while the prepass fills only the
  `main_pass_viewport` rect: steps now use `main_pass_viewport.zw`, samples map viewport UV to texels like #26235
  and clamp to the rendered rect, SSR maps `hit_uv` the same way. Identity with no override and a full viewport.
- H2 filter taps (far fast path and PCSS filter) used half the receiver slope, no margin: now full slope (exact to
  N·L 0.125, ±8 clamp, no 0.05 gate) + half-texel margin. The 0.15 m switch is per cascade, so the overlap blend hides it.
- `patches/superfargoth-shadow-early-out-lamp-filter.diff` predates the RO-11 hunk in `pbr_functions.wesl` (stale
  for that file; B2 hunks unchanged).

### Atmosphere static LUT cache (`atmosphere/{mod.rs,resources.rs,node.rs}`) & incremental `VisibilityRange` extraction (`view/visibility/range.rs`)
- `AtmosphereStaticLutCache` tracks computed `(ExtractedAtmosphere, GpuImage)` pairs and skips dispatching
  `transmittance_lut` and `multiscattering_lut` compute passes once populated unless planet radii or medium inputs
  change (`sky_view_lut` and `aerial_view_lut` still update per frame with sun direction and camera view).
- `extract_visibility_ranges` updates `RenderVisibilityRanges::entities` incrementally from `Changed<VisibilityRange>`
  and `RemovedComponents<VisibilityRange>` instead of clearing and re-hashing every `VisibilityRange` entity in the
  world on `ExtractSchedule` whenever streaming spawns or despawns a cell.

### P1 Live medium scales (`medium.rs`, `atmosphere/{mod.rs,resources.rs,functions.wesl,types.wesl}`)
- Stock `GpuScatteringMedium::prepare_asset` bakes coefficient x falloff x phase summed over terms into a
  `falloff x phase` LUT on the render thread, so any coefficient change (weather haze, dust) re-bakes it on the CPU.
- The LUTs now hold only what never changes with weather: each term's unit falloff (`density_lut`, 4 terms per
  RGBA texel) and phase (`scattering_lut`, one row per term); coefficients times `ScatteringMediumScales` are
  uniform data, as UE `SkyAtmosphere` scattering/absorption scales are live. Shader cost: the density fetch is
  unchanged (one texel per 4 terms), the scattering fetch adds one phase fetch per term with scattering (skipped
  at zero, e.g. ozone, dust off). A scale change changes the static LUT key: transmittance and multiscattering
  recompute on the GPU once. Checks: `sf_sky_bevy` `shader_tests` (passes link, Rust/WGSL uniform sizes agree).
- **P1b (2026-11-12, combined LUTs on the GPU, `atmosphere/{medium_lut.wesl,node.rs,resources.rs,functions.wesl}`).**
  Summing the terms per sample (2 density + per light 1 falloff + 1 phase per scattering term; sun and moon are
  both lights) raised GPU `atmosphere_luts` 0.36 -> 0.84 ms. Now `medium_lut.wesl` writes the stock combined
  `falloff x 2` density and `falloff x phase` scattering LUTs (`AtmosphereTextures::medium_density_lut`,
  `medium_scattering_lut`, Rgba32Float) inside the cached static pass, keyed like it (medium ids + scales; the
  cache key texture list now holds all four static textures), and every pass samples them stock-style, one fetch
  per lookup. Other users of bindings 5/6 (`sf_atmos_bevy` clouds) bind the combined textures too.

## Rebase or drop on a Bevy upgrade

1. Check both PRs: merged and released in the new version? Delete the three
   `vendor/bevy_*` folders and their `[patch.crates-io]` lines in both workspaces. Done.
2. Otherwise copy the new version's crates from the cargo registry
   (`~/.cargo/registry/src/index.crates.io-*/bevy_{core_pipeline,pbr,render}-<ver>`) over
   the folders (remove `Cargo.lock`, `.cargo-ok`), fetch the PRs' current diff
   (`gh pr diff 26161 -R bevyengine/bevy`, it includes #26149), strip the `crates/` prefix
   (`sed 's#\(a\|b\)/crates/#\1/#g'`), `patch -p1` it from `vendor/`, re-add the
   `superfargoth patch (#…)` comments, refresh `patches/`, and bump the versions above.
3. Every patched crate must match the version the rest of Bevy resolves to, or cargo
   silently ignores the patch (`cargo tree -i bevy_pbr` shows the path source when it took).
