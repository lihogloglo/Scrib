# Why the PBR maps cost 2.7 ms on the walk (diagnosis, 2026-11-15)

Diagnosis only, no code changed. Setup as `docs/ledger/perf.md` (RTX 4060 laptop, High, 1440p + DLSS Q). Inputs:
`benchmarks/runs/lean_{base,pom,pbr}` (this morning), `m_base` (2026-11-12), two new traced walks
(`target/perf/tcd_trbase` = maps on, `tcd_trpbr` = `--off normalmaps,orm,parallax`; profiling-trace build with
`trace,dlss`), and a same-exe water A/B (`benchmarks/runs/tc_pbr`, `tc_pbrwater`; see the caveat in section 3).

## 1. Mechanism

The walk is render-thread bound. The maps add almost no draws (`views/all/items` 17478 vs 17603) but split
the same draws into more **batch sets**, and each extra batch set is expensive to encode.

**Where the time goes (trace, self ms per frame, maps on vs off):** `finish_command_buffer` of
`per_view_shadow_pass` 3.35 vs 1.50, `main_opaque_pass_3d` 2.33 vs 0.97, `early_prepass` 2.31 vs 0.95,
`shared_shadow_pass` 1.76 vs 1.04, wgpu `finish_command_buffers` 1.65 vs 0.72. That is +6.3 thread-ms of pass
encoding, run in parallel. `RenderGraph` is 8.71 vs 6.50 ms per frame and the traced frame 16.41 vs 13.80 ms
(-2.6, the same as the untraced -2.73). `RenderExtractApp` +1.8 is the main thread waiting on the render thread.

**Three things split the batch sets** (a batch set is one `(pipeline, draw function, material bind group, mesh
slabs)`, `vendor/bevy_core_pipeline/src/core_3d/mod.rs:200-223`):

1. **Pipeline key bits.** `StandardMaterialKey::NORMAL_MAP` and `RELIEF_MAPPING` come from
   `normal_map_texture.is_some()` and the parallax method (`vendor/bevy_pbr/src/pbr_material.rs:1296-1307`).
   They become shader defs (:1484-1488) and are part of every material pipeline key, **including the
   depth-only shadow and prepass pipelines**, which never read the normal map. ORM adds no key bit (only its UV
   channel does). Static material keys go from 2 to 4.25 (`statics/material_keys`). Opaque pipelines in the main
   view go 54 to 65, distinct pipelines 82 to 97.
2. **A second bindless slab.** A slab holds at most 2048 distinct resources
   (`AUTO_BINDLESS_SLAB_RESOURCE_LIMIT`, `bevy_render-0.20.0/src/render_resource/bindless.rs:32`; full check in
   `vendor/bevy_render/src/material_bind_groups.rs:1091-1097`). Samplers are already shared (`SharedImageSamplers`,
   on by default, `superfargoth/crates/sf_world_bevy/src/lib.rs:171-173`), so the count here is textures. Base
   colours alone fit in one slab. The walk binds 523 normal, 523 ORM and 329 depth companions on top
   (`materials.live:` line). That spills StandardMaterial into 2 slabs (`statics/material_bind_groups` 1.0 to
   2.0), and every pipeline's batch set is duplicated per slab: main view 65 pipelines but 84 batch sets.
3. **Rebinding a big slab is O(slab size).** In a render pass, wgpu merges every resource of a bind group into
   the pass's usage scope on each `set_bind_group` (`wgpu-core-30.0.1/src/command/pass.rs:111-117`). Bevy skips
   a rebind only when the same group is already bound (`vendor/bevy_render/src/render_phase/draw_state.rs:188`).
   Batch sets are kept in insertion order (`IndexMap`, `render_phase/mod.rs:127`), so with 2 slabs the binding
   flips between groups of ~1000-2000 textures many times per pass. Measured: about 100 us of encode per extra
   main-view batch set (main opaque +1.37 ms for +13 opaque batch sets). A plain multi-draw costs a few us, so
   per-bind resource tracking is the only per-batch-set work that scales like this. **Not yet proved by an
   experiment:** fix A below is that experiment.

**Compiles while walking** are async (opaque relief variants 1.4-2.9 s in the driver): worker time
(`pipelines/compile_ms` 0.84 vs 0.49 per frame) and late pop-in, little frame time.
**Standing (GPU bound, -0.95):** `main_opaque` GPU 3.08 / 2.69 / 2.63 ms (all / POM off / maps off): POM ~0.4,
normal + ORM shading ~0.05, the rest is encode.

## 2. Counts (walk, mean per frame, median of 3 runs)

| gauge | m_base 11-12 | maps off | POM off | maps on |
|---|---:|---:|---:|---:|
| views/all batch_sets | 88 | 116 | 150 | 160 |
| main view pipelines / batch_sets | 41 / 44 | 54 / 58 | 58 / 78 | 65 / 84 |
| opaque3d pipelines / batch_sets | 13.7 / 14.5 | 21.0 / 21.9 | 23.0 / 31.3 | 26.4 / 34.3 |
| shadow phase pipelines / batch_sets | 106 / 44 | 129 / 58 | 130 / 72 | 133 / 79 |
| statics material keys / bind groups | 2 / 1 | 2 / 1 | 3.25 / 2 | 4.25 / 2 |
| mesh allocator slabs | 14 | 17 | 17 | 17 |
| render busy / graph ms | 8.26 / 4.43 | 9.88 / 5.45 | 11.60 / 7.06 | 12.63 / 7.66 |

## 3. The other regression (maps off: walk 9.96 to 11.42, standing 9.42 to 10.46)

Commits since `0f21d5d` were checked against the gauges in `m_base` and `lean_pbr`:

- **Tangent vertex layouts (653a9bc3 tangent rule, applied by the 2026-11-13 PBR pack bake; de0aadd1 keeps
  tangents on LOD/chain meshes):** about **+1.0 ms walk**, render thread. Pipelines are cached per final
  vertex buffer layout (`bevy_render-0.20.0/src/render_resource/pipeline_specializer.rs:202,230-240`). A stride
  48 mesh with tangents (`VERTEX_TANGENTS`, `bevy_pbr render/mesh.rs:3453`) needs a different pipeline from a
  stride 32 mesh, even in depth-only shadows. It also needs a different mesh slab (`ElementLayout` includes the
  size, `mesh/allocator.rs:181`). Measured: opaque pipelines 13.7 to 21.0, shadow batch sets 44 to 58, all batch
  sets 88 to 116, mesh slabs 14 to 17 (+51 MB), `render/graph` +1.0 ms. A bake property, so there is no runtime
  switch; attributed from the counters.
- **Water (63599adc W-SSPR, c1b15fa3 W-GERSTNER):** about **+0.45 ms walk, +0.55 standing**, GPU. Under DLSS:
  `water_sspr` +0.22-0.24 (new), `water_velocity` +0.18-0.21, water moved from the transparent to the
  transmissive pass (+0.05 walk, +0.12 standing). Same-exe A/B `--off waterssr,shorewaves`: walk 13.19 to 12.48
  (-0.71), standing 15.55 to 14.76 (-0.79), `water_velocity` 0.31 to 0.11. **Caveat:** that exe copy found no
  `dlss/` plugin and ran TAA at native 1440p, so these A/B numbers are larger than with DLSS. Also,
  `target/profiling/dlss/` currently has no `sf_dlss.dll`, so any suite run from `target/profiling` now runs
  without DLSS (tell `h1-gpucull`).
- **Fog and clouds (6e69c019 W-ORDER/FOG fixes, 70bf17fd/da6d1c60 cloud fixes):** about **+0.13 ms GPU**.
  `atmos_froxel_inject` 0.10 to 0.16 (invocations 983k to 1376k), `clouds_composite` + `clouds_resolve` +0.07.
- Rest (+0.1-0.2 GPU in `main_opaque`, `bin_unpacking`, `uniform_allocation`): extra batch sets, tangent fetch.

## 4. Fixes, ranked (expected walk ms from the counts above; measure each with a 3-run A/B)

| # | fix | industry reference | expected | size |
|---|---|---|---|---|
| A | **One StandardMaterial slab:** raise the bindless slab limit in the vendored `pbr_material.rs:25` (`#[bindless(limit(N), ...)]`), sized to the content (about 4096-8192 textures). First check `bindless_supported` (sampler arrays x N within `max_binding_array_sampler_elements_per_shader_stage`). Add a fallback that sorts multi-draw batch sets by `material_bind_group_index`, so a pass binds each slab once. | One global descriptor heap for all material textures: UE5 bindless and D3D12 heaps of up to 1M descriptors; state-sorted draw lists (UE mesh draw command sort) | **-1.2 to -1.6** (batch sets 160 to about 135; removes most of the extra encode) | S |
| B | **One vertex layout and one key per material class:** tangents on every lit static mesh, plus a shared 1x1 flat normal map for those without a companion, so `NORMAL_MAP` folds in matclass (`matclass.rs:21-23` already describes this). Mask `NORMAL_MAP`/`RELIEF_MAPPING` out of depth-only shadow pipeline keys (vendored patch). | UE stores a tangent basis on every static mesh and material instances share the parent shader | **-0.7 to -1.0**, and it undoes the tangent half of the regression (pipelines back to about m_base), +16 B/vertex | M (bake + matclass + small patch) |
| C | **Parallax only where it pays:** a POM material class or flag (ground, walls), relief only on Ultra, Occlusion on High, faded out with distance | UE POM as an opt-in material function with distance fade; OpenMW's parallax per texture | **-0.3 to -0.4 standing GPU**; its CPU share goes with A+B | S-M |
| D | **Precompile the variants at load** (the reveal gate warm-up already exists) and use a Vulkan pipeline cache, so a variant is never first compiled on the walk | UE5 PSO precaching, Vulkan pipeline cache | stutters and pop-in, ~0 median | S-M |

A recovers most of the CPU cost; A+B should put the walk with maps on near or under this morning's maps-off 11.4
ms. Water and fog are owner-confirmed features; half-rate SSPR is the standard lever if they must shrink.

## 5. Ultra out of GPU memory (note, do not fix)

Ultra goes out of memory on the walk with 4354 MiB of live textures. The three BC5 terrain layer arrays alone are
1520 MiB, because `mw_exterior_bevy/src/exterior.rs:541-544` only sets the terrain budget when the preset has a size
cap (`budget: texture_cap.and(...)`, and Ultra has none), and the global texture pool (spec textures.md:155,
S3/S4) is not built. The standard is a per-preset pool that also stays under a fraction of the device's VRAM:
UE `r.Streaming.PoolSize` per scalability group (about 1-3 GB on 8 GB cards, the rest for render targets and
meshes), Unity's mip streaming memory budget (a fixed cap, lower mips past it). For an 8 GB laptop that means
about 2.5-3 GiB of textures in total on Ultra, terrain arrays inside it. So: always apply
`DEFAULT_LAYER_BUDGET` (1 GiB, `sf_terrain_bevy/src/lib.rs:75`) to the terrain, and when S4 lands, clamp the
spec's Ultra 4 GiB to about 0.4 x VRAM. Also: process RAM went 2.6 to 5.0 GB since 11-12, so the main world
probably keeps CPU copies of the images (`RenderAssetUsages`). Check this when the pool is built.
