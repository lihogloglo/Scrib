# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 733 frames over 15.0 s · **median 11.43 ms (88 fps)** · mean 20.47 · p95 56.25 · p99 59.02 (1% low 17 fps) · worst 62.73 ms · 319 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 1316 frames over 28.0 s, worst 2761.1 ms
- hitches (> 28.6 ms): **95** measured, 195 in warm-up
- timed GPU passes: 15.87 ms of a 20.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.43 ms, limit 11 ms
- **FAIL** worst frame 62.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.367 | 3.486 | 7.380 | 0.97 | 0.070 |
| water_fft | 3.317 | 3.434 | 8.500 | 0.97 | 0.003 |
| dlss_super_resolution | 2.606 | 2.698 | 5.555 | 0.97 | 0.202 |
| early prepass | 1.076 | 1.114 | 3.072 | 0.97 | 0.114 |
| bin_unpacking | 0.950 | 0.983 | 3.777 | 0.97 | 0.024 |
| early_mesh_preprocessing | 0.592 | 0.613 | 1.884 | 0.97 | 0.005 |
| atmosphere_luts | 0.446 | 0.462 | 1.046 | 0.97 | 0.004 |
| bloom | 0.403 | 0.417 | 0.951 | 0.97 | 0.064 |
| uniform_allocation | 0.397 | 0.411 | 1.190 | 0.97 | 0.035 |
| water_sspr | 0.395 | 0.409 | 1.118 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.354 | 0.366 | 0.955 | 0.97 | 0.008 |
| water_snapshot | 0.353 | 0.365 | 1.208 | 0.97 | 0.002 |
| water_velocity | 0.284 | 0.294 | 0.803 | 0.97 | 0.001 |
| ssao | 0.222 | 0.229 | 0.469 | 0.97 | 0.008 |
| atmos_froxel_inject | 0.166 | 0.172 | 0.479 | 0.97 | 0.001 |
| clustering | 0.154 | 0.160 | 0.257 | 0.97 | 0.025 |
| clouds_composite | 0.111 | 0.115 | 0.110 | 0.97 | 0.002 |
| tonemapping | 0.107 | 0.111 | 0.114 | 0.97 | 0.003 |
| clouds_trace | 0.099 | 0.102 | 0.152 | 0.97 | 0.001 |
| upscaling | 0.097 | 0.101 | 0.130 | 0.97 | 0.004 |
| lightprobe_irradiance_map | 0.067 | 0.544 | 1.147 | 0.12 | 0.001 |
| render_sky | 0.054 | 0.056 | 0.057 | 0.97 | 0.002 |
| clouds_resolve | 0.049 | 0.051 | 0.052 | 0.97 | 0.001 |
| atmos_froxel_integrate | 0.046 | 0.048 | 0.034 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.037 | 0.298 | 0.780 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.027 | 0.028 | 0.035 | 0.97 | 0.076 |
| water_mips | 0.025 | 0.026 | 0.028 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.018 | 0.022 | 0.97 | 0.006 |
| atmosphere_environment | 0.012 | 0.012 | 0.013 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.009 | 0.009 | 0.007 | 0.97 | 0.003 |
| lightprobe_copy | 0.008 | 0.064 | 0.073 | 0.12 | 0.004 |
| night_sky | 0.007 | 0.007 | 0.008 | 0.97 | 0.001 |
| clouds_shadow | 0.004 | 0.004 | 0.005 | 0.97 | 0.004 |
| auxiliary_views | 0.003 | 0.003 | 0.003 | 0.97 | 0.004 |
| lightprobe_downsampling_first_pass | 0.003 | 0.022 | 0.034 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 289.959 | 0.000 | 70737.76 | 212540.2 |
| render/thread/ms | 19.476 | 55.236 | 61.85 | 14275.8 |
| render/acquire/ms | 10.054 | 44.705 | 50.40 | 7369.9 |
| pipelines/compile_ms | 9.324 | 0.000 | 2282.70 | 6834.3 |
| render/busy/ms | 8.871 | 10.420 | 13.02 | 6502.4 |
| pipelines/driver_ms | 8.357 | 0.000 | 2155.45 | 6126.0 |
| main/busy/ms | 7.073 | 8.541 | 11.12 | 5184.3 |
| render/graph/ms | 5.408 | 6.464 | 8.30 | 3964.3 |
| pipelines/front_ms | 0.966 | 0.000 | 160.09 | 708.3 |
| render/present/ms | 0.551 | 0.801 | 4.38 | 403.5 |
| bigspace/total/ms | 0.382 | 0.895 | 2.53 | 280.3 |
| terrain/update/ms | 0.245 | 0.316 | 0.85 | 179.6 |
| terrain/select/ms | 0.214 | 0.272 | 0.82 | 156.8 |
| stream/drive/ms | 0.202 | 0.291 | 0.67 | 148.1 |
| far/select/ms | 0.175 | 0.252 | 0.51 | 128.4 |
| stream/tick/ms | 0.172 | 0.257 | 0.62 | 126.3 |
| controller/move/ms | 0.160 | 0.277 | 1.54 | 117.5 |
| controller/feed/ms | 0.154 | 0.206 | 0.37 | 112.7 |
| bigspace/high_precision/ms | 0.126 | 0.167 | 2.12 | 92.5 |
| bigspace/low_precision/ms | 0.112 | 0.159 | 2.19 | 82.2 |
| anim/evaluate/ms | 0.075 | 0.149 | 0.61 | 55.2 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.21 | 44.9 |
| physics/step/ms | 0.059 | 0.112 | 0.88 | 43.0 |
| weather/map/ms | 0.016 | 0.022 | 0.03 | 12.0 |
| physics/insert/ms | 0.011 | 0.000 | 0.90 | 7.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.23 | 7.8 |
| script/update/ms | 0.008 | 0.012 | 0.25 | 5.9 |
| controller/avoid/ms | 0.006 | 0.008 | 0.02 | 4.4 |
| script/input/ms | 0.006 | 0.007 | 0.17 | 4.3 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 3.6 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 3.2 |
| controller/write/ms | 0.002 | 0.002 | 0.00 | 1.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.3 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 2958 | 2958 |
| crowd/joints_uploaded/count | count | 2958 | 2958 | 2958 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.73 | 51.41 | 9.10 |
| gpu/timed/ms | ms | 9.68 | 51.41 | 9.10 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1851 | 1851 | 1851 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 196.0 k | 196.0 k | 196.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 28 | 28 |
| phases/opaque3d/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 28 | 28 |
| phases/opaque3d_prepass/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 24 | 24 |
| phases/shadow/bins | count | 985 | 985 | 985 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2006 | 2006 | 2006 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 24 | 24 | 24 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 546 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 192 | 192 | 192 |
| pipelines/waiting | count | 0 | 9 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 210 | 210 | 210 |
| process/cpu_usage | % | 23.56 | 43.27 | 34.87 |
| process/mem_usage | GiB | 4.99 | 5.10 | 5.04 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 0 | 0 | 0 |
| scene/batch/masked/count | count | 0 | 0 | 0 |
| scene/batch/masked/high_water | count | 0 | 0 | 0 |
| scene/batch/masked/live | count | 0 | 0 | 0 |
| scene/batch/masked/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_wind/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_wind/count | count | 0 | 0 | 0 |
| scene/batch/masked_wind/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_wind/live | count | 0 | 0 | 0 |
| scene/batch/masked_wind/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque/count | count | 0 | 0 | 0 |
| scene/batch/opaque/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque/live | count | 0 | 0 | 0 |
| scene/batch/opaque/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/count | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/live | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batches/count | count | 0 | 0 | 0 |
| scene/cells/resident | count | 0 | 0 | 0 |
| scene/chains/high_water | count | 0 | 0 | 0 |
| scene/chains/live | count | 0 | 0 | 0 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 0 | 0 | 0 |
| scene/slots/batch_capacity | count | 0 | 0 | 0 |
| scene/slots/batch_capacity_bytes | bytes | 0 | 0 | 0 |
| scene/slots/batch_live | count | 0 | 0 | 0 |
| scene/slots/scene_bump | count | 0 | 0 | 0 |
| scene/slots/scene_live | count | 0 | 0 | 0 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3552 | 3552 | 3552 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.91 | 57.20 | 39.35 |
| system/mem_usage | % | 58.66 | 73.07 | 58.55 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 94 | 94 | 94 |
| views/all/bins | count | 4151 | 4151 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 90 | 90 |
| views/all/pipelines_distinct | count | 89 | 89 | 89 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 24 | 24 | 24 |
| views/cascade/bins | count | 985 | 985 | 985 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2006 | 2006 | 2006 |
| views/cascade/pipelines | count | 24 | 24 | 24 |
| views/cascade/work_items | count | 2006 | 2006 | 2006 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 70 | 70 | 70 |
| views/main/bins | count | 3166 | 3166 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 66 | 66 | 66 |
| views/main/work_items | count | 9480 | 9480 | 9480 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.18 | 3 | 135 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 5.85 | 11 | 4290 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -2 |
| physics/inserts | count | 0.90 | 124 | 659 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 1.19 | 3 | 874 |
| physics/steps_dropped | count | 0.04 | 1 | 29 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.76 | 2761.1 | main/busy/ms 2729, surface/advance_cpu_ms 0.75, stream/drive/ms 0.43, surface/cell_upload_ms 0.30, stream/tick/ms 0.13, bigspace/total/ms 0.12, bigspace/high_precision/ms 0.07, weather/map/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.03, anim/evaluate/ms 0.01 |
| warm-up | 3.42 | 661.9 | main/busy/ms 2771, render/thread/ms 617, render/busy/ms 511, render/acquire/ms 105, render/graph/ms 5.58, render/present/ms 2.31, stream/drive/ms 1.11, surface/advance_cpu_ms 0.75, terrain/update/ms 0.70, stream/tick/ms 0.57, terrain/select/ms 0.40, surface/cell_upload_ms 0.30 |
| warm-up | 3.71 | 98.7 | pipelines/wait_ms 1384, pipelines/compile_ms 617, pipelines/driver_ms 526, pipelines/front_ms 90.93, main/busy/ms 75.08, render/thread/ms 56.96, render/busy/ms 54.82, render/graph/ms 5.16, stream/drive/ms 2.15, render/present/ms 1.69, stream/tick/ms 1.69, terrain/update/ms 0.71 |
| warm-up | 3.90 | 64.6 | pipelines/wait_ms 402, main/busy/ms 43.99, render/thread/ms 40.68, render/busy/ms 39.25, stream/drive/ms 32.73, pipelines/compile_ms 10.92, pipelines/driver_ms 7.83, render/graph/ms 3.56, pipelines/front_ms 3.09, render/present/ms 1.20, stream/tick/ms 0.80, terrain/update/ms 0.55 |
| warm-up | 3.97 | 65.8 | pipelines/wait_ms 443, pipelines/compile_ms 64.40, pipelines/driver_ms 51.48, render/thread/ms 41.46, render/busy/ms 39.49, main/busy/ms 37.61, stream/drive/ms 21.35, pipelines/front_ms 12.92, render/graph/ms 4.93, render/present/ms 1.73, terrain/update/ms 1.50, stream/spawn/ms 1.36 |
| warm-up | 4.16 | 166.7 | render/thread/ms 97.70, render/busy/ms 79.10, main/busy/ms 78.02, stream/drive/ms 51.99, render/present/ms 18.28, render/graph/ms 4.96, stream/spawn/ms 2.45, stream/tick/ms 1.73, terrain/update/ms 0.82, terrain/select/ms 0.63, bigspace/total/ms 0.58, bigspace/high_precision/ms 0.36 |
| warm-up | 4.23 | 60.1 | render/thread/ms 116, render/busy/ms 97.44, main/busy/ms 57.50, stream/drive/ms 31.79, render/present/ms 18.25, render/graph/ms 7.92, stream/spawn/ms 1.49, stream/tick/ms 1.46, terrain/update/ms 0.87, terrain/select/ms 0.65, bigspace/total/ms 0.57, stream/activate/ms 0.48 |
| warm-up | 4.63 | 132.6 | pipelines/wait_ms 1579, pipelines/compile_ms 746, pipelines/driver_ms 706, main/busy/ms 117, stream/drive/ms 101, render/thread/ms 52.27, render/busy/ms 50.49, pipelines/front_ms 40.40, render/graph/ms 3.71, render/present/ms 1.51, stream/tick/ms 1.49, stream/spawn/ms 1.48 |
| warm-up | 9.77 | 62.9 | pipelines/wait_ms 4217, pipelines/compile_ms 104, main/busy/ms 82.03, render/thread/ms 76.29, render/busy/ms 73.14, pipelines/driver_ms 58.13, pipelines/front_ms 46.02, render/graph/ms 7.94, bigspace/total/ms 2.29, render/present/ms 2.12, bigspace/low_precision/ms 1.17, stream/drive/ms 1.16 |
| warm-up | 9.89 | 73.6 | pipelines/wait_ms 3396, pipelines/compile_ms 137, pipelines/driver_ms 90.18, main/busy/ms 74.84, render/thread/ms 61.28, render/busy/ms 58.81, pipelines/front_ms 46.52, render/graph/ms 8.51, render/present/ms 2.17, bigspace/total/ms 1.23, anim/evaluate_cpu/ms 0.94, stream/drive/ms 0.68 |
| warm-up | 12.82 | 79.6 | render/thread/ms 93.81, render/acquire/ms 66.26, render/busy/ms 25.69, main/busy/ms 21.03, render/graph/ms 8.72, render/present/ms 1.86, terrain/update/ms 0.96, bigspace/total/ms 0.85, terrain/select/ms 0.73, stream/drive/ms 0.60, stream/tick/ms 0.53, far/select/ms 0.48 |
| warm-up | 22.11 | 59.8 | pipelines/wait_ms 28.0 k, render/thread/ms 99.36, render/acquire/ms 74.04, render/busy/ms 23.72, main/busy/ms 14.71, render/graph/ms 13.14, pipelines/compile_ms 7.42, pipelines/driver_ms 6.75, render/present/ms 1.60, pipelines/front_ms 0.67, bigspace/total/ms 0.58, terrain/update/ms 0.57 |
| warm-up | 22.39 | 59.5 | render/thread/ms 114, render/acquire/ms 90.97, render/busy/ms 21.61, main/busy/ms 17.94, render/graph/ms 11.73, render/present/ms 1.31, controller/feed/ms 0.74, bigspace/total/ms 0.65, terrain/update/ms 0.60, far/select/ms 0.57, stream/drive/ms 0.54, terrain/select/ms 0.47 |
| warm-up | 22.72 | 60.5 | render/thread/ms 115, render/acquire/ms 90.50, render/busy/ms 22.63, main/busy/ms 17.60, render/graph/ms 13.69, render/present/ms 1.51, bigspace/total/ms 0.91, terrain/update/ms 0.62, stream/drive/ms 0.58, stream/tick/ms 0.50, terrain/select/ms 0.46, far/select/ms 0.44 |
| warm-up | 22.89 | 58.5 | render/thread/ms 105, render/acquire/ms 83.24, render/busy/ms 20.40, main/busy/ms 19.22, render/graph/ms 11.60, render/present/ms 1.26, terrain/update/ms 0.81, terrain/select/ms 0.70, bigspace/total/ms 0.64, far/select/ms 0.63, stream/drive/ms 0.63, stream/tick/ms 0.55 |
| warm-up | 23.06 | 60.4 | render/thread/ms 115, render/acquire/ms 90.78, render/busy/ms 22.62, main/busy/ms 17.19, render/graph/ms 12.67, render/present/ms 1.50, stream/drive/ms 0.61, bigspace/total/ms 0.60, terrain/update/ms 0.53, stream/tick/ms 0.52, far/select/ms 0.51, terrain/select/ms 0.41 |
| warm-up | 23.18 | 63.6 | render/thread/ms 113, render/acquire/ms 85.30, render/busy/ms 26.32, main/busy/ms 17.13, render/graph/ms 14.81, render/present/ms 1.23, bigspace/total/ms 0.66, terrain/update/ms 0.58, far/select/ms 0.49, stream/drive/ms 0.46, terrain/select/ms 0.46, stream/tick/ms 0.39 |
| warm-up | 23.50 | 59.5 | pipelines/wait_ms 20.1 k, pipelines/compile_ms 1426, pipelines/driver_ms 1265, pipelines/front_ms 161, render/thread/ms 103, render/acquire/ms 79.60, render/busy/ms 22.20, main/busy/ms 18.56, render/graph/ms 13.50, render/present/ms 1.22, far/select/ms 0.87, stream/drive/ms 0.73 |
| warm-up | 24.59 | 59.7 | render/thread/ms 108, render/acquire/ms 83.38, render/busy/ms 23.07, main/busy/ms 15.40, render/graph/ms 14.05, render/present/ms 1.49, terrain/update/ms 0.93, terrain/select/ms 0.79, bigspace/total/ms 0.59, physics/insert/ms 0.45, far/select/ms 0.44, stream/drive/ms 0.39 |
| warm-up | 24.65 | 58.5 | render/thread/ms 116, render/acquire/ms 91.20, render/busy/ms 22.73, main/busy/ms 16.77, render/graph/ms 13.86, render/present/ms 1.58, terrain/update/ms 0.99, terrain/select/ms 0.85, bigspace/total/ms 0.66, far/select/ms 0.54, physics/insert/ms 0.47, stream/drive/ms 0.45 |
| warm-up | 24.71 | 59.9 | render/thread/ms 116, render/acquire/ms 91.29, render/busy/ms 23.25, main/busy/ms 18.35, render/graph/ms 14.40, render/present/ms 1.45, bigspace/total/ms 0.69, terrain/update/ms 0.57, far/select/ms 0.54, physics/insert/ms 0.50, stream/drive/ms 0.47, terrain/select/ms 0.47 |
| warm-up | 24.83 | 61.8 | pipelines/wait_ms 18.5 k, pipelines/compile_ms 1532, pipelines/driver_ms 1400, pipelines/front_ms 132, render/thread/ms 111, render/acquire/ms 84.23, render/busy/ms 25.40, main/busy/ms 17.73, render/graph/ms 15.15, render/present/ms 1.66, bigspace/total/ms 0.71, physics/insert/ms 0.51 |
| warm-up | 25.00 | 58.3 | render/thread/ms 111, render/acquire/ms 82.21, render/busy/ms 27.52, main/busy/ms 17.47, render/graph/ms 17.05, render/present/ms 1.60, bigspace/total/ms 0.77, terrain/update/ms 0.49, physics/insert/ms 0.46, stream/drive/ms 0.41, terrain/select/ms 0.41, controller/feed/ms 0.38 |
| warm-up | 25.06 | 58.6 | render/thread/ms 114, render/acquire/ms 87.43, render/busy/ms 25.26, main/busy/ms 17.37, render/graph/ms 15.76, render/present/ms 1.55, bigspace/total/ms 0.76, stream/drive/ms 0.54, terrain/update/ms 0.51, physics/insert/ms 0.47, stream/tick/ms 0.47, terrain/select/ms 0.42 |
| warm-up | 25.28 | 59.4 | render/thread/ms 114, render/acquire/ms 89.61, render/busy/ms 23.32, main/busy/ms 19.75, render/graph/ms 13.98, render/present/ms 1.27, bigspace/total/ms 0.80, terrain/update/ms 0.60, terrain/select/ms 0.50, physics/insert/ms 0.45, stream/drive/ms 0.43, controller/feed/ms 0.41 |
| warm-up | 25.40 | 61.8 | render/thread/ms 110, render/acquire/ms 86.12, render/busy/ms 22.65, main/busy/ms 15.58, render/graph/ms 13.83, render/present/ms 1.60, bigspace/total/ms 0.58, physics/insert/ms 0.50, terrain/update/ms 0.49, stream/drive/ms 0.49, stream/tick/ms 0.42, terrain/select/ms 0.41 |
| warm-up | 25.46 | 59.7 | render/thread/ms 119, render/acquire/ms 94.25, render/busy/ms 23.28, main/busy/ms 17.49, render/graph/ms 14.53, render/present/ms 1.59, bigspace/total/ms 0.61, stream/drive/ms 0.48, terrain/update/ms 0.48, physics/insert/ms 0.45, stream/tick/ms 0.42, terrain/select/ms 0.40 |
| warm-up | 25.79 | 63.7 | render/thread/ms 114, render/acquire/ms 83.03, render/busy/ms 23.86, main/busy/ms 15.83, render/graph/ms 15.42, render/present/ms 7.24, bigspace/total/ms 0.64, terrain/update/ms 0.51, physics/insert/ms 0.47, terrain/select/ms 0.42, stream/drive/ms 0.38, far/select/ms 0.34 |
| warm-up | 26.11 | 59.0 | pipelines/wait_ms 98.4 k, pipelines/compile_ms 2131, pipelines/driver_ms 1961, pipelines/front_ms 171, render/thread/ms 107, render/acquire/ms 84.48, render/busy/ms 21.20, main/busy/ms 17.96, render/graph/ms 13.07, render/present/ms 1.43, physics/insert/ms 0.88, bigspace/total/ms 0.59 |
| warm-up | 26.17 | 59.7 | pipelines/wait_ms 120.4 k, pipelines/compile_ms 2128, pipelines/driver_ms 1949, pipelines/front_ms 180, render/thread/ms 116, render/acquire/ms 90.72, render/busy/ms 23.37, main/busy/ms 20.32, render/graph/ms 12.69, render/present/ms 1.54, physics/insert/ms 0.91, bigspace/total/ms 0.57 |
| warm-up | 26.29 | 58.6 | pipelines/wait_ms 20.0 k, pipelines/compile_ms 1443, pipelines/driver_ms 1292, pipelines/front_ms 151, render/thread/ms 113, render/acquire/ms 88.51, render/busy/ms 22.96, main/busy/ms 22.32, render/graph/ms 13.50, render/present/ms 1.05, bigspace/total/ms 0.96, stream/drive/ms 0.80 |
| warm-up | 26.35 | 62.7 | render/thread/ms 119, render/acquire/ms 91.20, main/busy/ms 31.18, render/busy/ms 26.39, render/graph/ms 15.63, terrain/update/ms 4.14, render/present/ms 1.36, physics/insert/ms 0.91, bigspace/total/ms 0.79, stream/drive/ms 0.76, stream/tick/ms 0.67, far/select/ms 0.55 |
| warm-up | 26.41 | 60.9 | render/thread/ms 121, render/acquire/ms 88.58, render/busy/ms 30.50, main/busy/ms 29.24, render/graph/ms 17.32, terrain/update/ms 10.40, render/present/ms 1.60, physics/insert/ms 0.86, bigspace/total/ms 0.74, stream/drive/ms 0.54, far/select/ms 0.51, terrain/select/ms 0.47 |
| warm-up | 26.58 | 58.4 | render/thread/ms 107, render/acquire/ms 79.95, render/busy/ms 25.12, main/busy/ms 16.94, render/graph/ms 15.78, render/present/ms 1.57, bigspace/total/ms 0.72, terrain/update/ms 0.64, terrain/select/ms 0.54, physics/insert/ms 0.52, stream/drive/ms 0.51, stream/tick/ms 0.44 |
| warm-up | 26.64 | 60.4 | pipelines/wait_ms 63.7 k, pipelines/compile_ms 2967, pipelines/driver_ms 2684, pipelines/front_ms 283, render/thread/ms 116, render/acquire/ms 90.29, render/busy/ms 24.24, main/busy/ms 18.77, render/graph/ms 15.42, render/present/ms 1.67, bigspace/total/ms 0.89, terrain/update/ms 0.59 |
| warm-up | 26.75 | 60.1 | pipelines/wait_ms 67.3 k, pipelines/compile_ms 111, render/thread/ms 108, render/acquire/ms 83.12, pipelines/driver_ms 66.95, pipelines/front_ms 43.73, main/busy/ms 23.67, render/busy/ms 23.11, render/graph/ms 14.17, render/present/ms 1.51, physics/insert/ms 1.08, bigspace/total/ms 0.90 |
| warm-up | 26.98 | 62.8 | render/thread/ms 116, render/acquire/ms 88.91, render/busy/ms 25.43, main/busy/ms 18.88, render/graph/ms 15.93, render/present/ms 1.37, physics/step/ms 1.31, physics/insert/ms 1.12, terrain/update/ms 0.60, bigspace/total/ms 0.53, terrain/select/ms 0.51, stream/drive/ms 0.48 |
| warm-up | 27.04 | 61.4 | render/thread/ms 122, render/acquire/ms 93.63, render/busy/ms 26.18, main/busy/ms 19.87, render/graph/ms 17.11, render/present/ms 1.70, physics/insert/ms 1.20, physics/step/ms 1.12, bigspace/total/ms 0.71, far/select/ms 0.58, stream/drive/ms 0.55, terrain/update/ms 0.53 |
| warm-up | 27.10 | 59.7 | render/thread/ms 118, render/acquire/ms 93.51, render/busy/ms 23.21, main/busy/ms 21.25, render/graph/ms 14.30, render/present/ms 1.55, physics/insert/ms 1.17, physics/step/ms 0.87, bigspace/total/ms 0.79, far/select/ms 0.63, stream/drive/ms 0.57, terrain/update/ms 0.53 |
| warm-up | 27.22 | 62.2 | render/thread/ms 118, render/acquire/ms 92.64, render/busy/ms 23.77, main/busy/ms 17.92, render/graph/ms 14.79, render/present/ms 1.21, physics/insert/ms 1.04, physics/step/ms 0.93, bigspace/total/ms 0.62, terrain/update/ms 0.58, terrain/select/ms 0.50, stream/drive/ms 0.49 |
| warm-up | 27.28 | 60.1 | render/thread/ms 120, render/acquire/ms 93.35, render/busy/ms 25.34, main/busy/ms 19.14, render/graph/ms 16.81, render/present/ms 1.35, physics/insert/ms 1.00, physics/step/ms 0.85, bigspace/total/ms 0.60, stream/drive/ms 0.49, terrain/update/ms 0.47, controller/feed/ms 0.46 |
| warm-up | 27.34 | 59.3 | render/thread/ms 117, render/acquire/ms 90.67, render/busy/ms 24.91, main/busy/ms 21.18, render/graph/ms 16.24, render/present/ms 1.34, physics/insert/ms 1.31, physics/step/ms 0.80, bigspace/total/ms 0.67, stream/drive/ms 0.52, terrain/update/ms 0.46, far/select/ms 0.45 |
| warm-up | 27.45 | 59.8 | render/thread/ms 109, render/acquire/ms 83.77, render/busy/ms 23.56, main/busy/ms 15.99, render/graph/ms 15.10, render/present/ms 1.48, physics/insert/ms 0.98, stream/drive/ms 0.76, stream/tick/ms 0.69, bigspace/total/ms 0.63, physics/step/ms 0.52, terrain/update/ms 0.48 |
| warm-up | 27.51 | 61.6 | render/thread/ms 119, render/acquire/ms 92.03, render/busy/ms 25.40, main/busy/ms 16.94, render/graph/ms 16.05, render/present/ms 1.55, stream/drive/ms 0.78, bigspace/total/ms 0.73, stream/tick/ms 0.71, terrain/update/ms 0.45, far/select/ms 0.43, physics/insert/ms 0.42 |
| warm-up | 27.57 | 62.8 | render/thread/ms 121, render/acquire/ms 95.39, render/busy/ms 24.54, main/busy/ms 19.29, render/graph/ms 14.60, render/present/ms 1.55, bigspace/total/ms 0.68, stream/drive/ms 0.60, far/select/ms 0.56, stream/tick/ms 0.52, terrain/update/ms 0.46, terrain/select/ms 0.40 |
| warm-up | 27.69 | 61.5 | render/thread/ms 113, render/acquire/ms 88.61, render/busy/ms 23.00, main/busy/ms 18.14, render/graph/ms 13.98, render/present/ms 1.50, bigspace/total/ms 0.71, far/select/ms 0.61, terrain/update/ms 0.54, terrain/select/ms 0.46, stream/drive/ms 0.46, controller/feed/ms 0.43 |
| warm-up | 27.75 | 59.9 | render/thread/ms 119, render/acquire/ms 94.42, render/busy/ms 23.24, main/busy/ms 17.86, render/graph/ms 14.31, render/present/ms 1.39, bigspace/total/ms 0.60, far/select/ms 0.58, stream/drive/ms 0.50, terrain/update/ms 0.48, stream/tick/ms 0.44, terrain/select/ms 0.41 |
| warm-up | 27.81 | 61.3 | render/thread/ms 119, render/acquire/ms 92.63, render/busy/ms 24.76, main/busy/ms 17.89, render/graph/ms 15.09, render/present/ms 1.40, bigspace/total/ms 0.61, stream/drive/ms 0.58, stream/tick/ms 0.50, terrain/update/ms 0.50, far/select/ms 0.45, terrain/select/ms 0.43 |
| warm-up | 27.93 | 62.0 | pipelines/wait_ms 43.4 k, pipelines/compile_ms 3394, pipelines/driver_ms 3112, pipelines/front_ms 282, render/thread/ms 115, render/acquire/ms 88.53, render/busy/ms 25.29, main/busy/ms 18.55, render/graph/ms 15.86, render/present/ms 1.33, stream/drive/ms 0.77, stream/tick/ms 0.68 |
| warm-up | 28.04 | 59.4 | render/thread/ms 113, render/acquire/ms 90.19, render/busy/ms 21.68, main/busy/ms 18.10, render/graph/ms 13.25, render/present/ms 1.30, physics/insert/ms 0.70, bigspace/total/ms 0.66, terrain/update/ms 0.51, terrain/select/ms 0.43, stream/drive/ms 0.42, controller/feed/ms 0.40 |
| measure | 0.11 | 60.1 | pipelines/wait_ms 23.8 k, render/thread/ms 111, render/acquire/ms 86.28, pipelines/compile_ms 56.53, pipelines/driver_ms 39.95, render/busy/ms 23.36, main/busy/ms 20.24, pipelines/front_ms 16.59, render/graph/ms 14.96, physics/step/ms 1.61, render/present/ms 1.29, physics/insert/ms 1.06 |
| measure | 0.17 | 58.3 | pipelines/wait_ms 47.8 k, render/thread/ms 116, render/acquire/ms 90.83, pipelines/compile_ms 76.85, pipelines/driver_ms 49.62, pipelines/front_ms 27.23, render/busy/ms 23.91, main/busy/ms 20.19, render/graph/ms 15.44, physics/step/ms 1.64, render/present/ms 1.29, physics/insert/ms 1.06 |
| measure | 0.34 | 58.8 | pipelines/wait_ms 70.7 k, pipelines/compile_ms 1455, pipelines/driver_ms 1295, pipelines/front_ms 160, render/thread/ms 107, render/acquire/ms 82.83, render/busy/ms 23.44, main/busy/ms 19.51, render/graph/ms 14.17, physics/insert/ms 1.40, physics/step/ms 1.30, render/present/ms 1.21 |
| measure | 0.40 | 59.5 | render/thread/ms 115, render/acquire/ms 89.06, render/busy/ms 24.62, main/busy/ms 20.24, render/graph/ms 15.27, render/present/ms 1.48, physics/step/ms 1.40, physics/insert/ms 0.92, bigspace/total/ms 0.79, terrain/update/ms 0.47, terrain/select/ms 0.41, controller/feed/ms 0.38 |
| measure | 0.46 | 60.4 | render/thread/ms 117, render/acquire/ms 92.55, render/busy/ms 23.41, main/busy/ms 18.32, render/graph/ms 14.69, render/present/ms 1.49, physics/step/ms 1.01, bigspace/total/ms 0.72, terrain/update/ms 0.51, stream/drive/ms 0.49, terrain/select/ms 0.45, stream/tick/ms 0.43 |
| measure | 0.57 | 60.8 | render/thread/ms 110, render/acquire/ms 85.48, render/busy/ms 23.18, main/busy/ms 16.22, render/graph/ms 13.75, render/present/ms 1.27, physics/insert/ms 0.85, bigspace/total/ms 0.75, physics/step/ms 0.52, terrain/update/ms 0.52, terrain/select/ms 0.45, controller/move/ms 0.42 |
| measure | 0.85 | 58.6 | render/thread/ms 111, render/acquire/ms 86.60, render/busy/ms 22.80, main/busy/ms 17.31, render/graph/ms 14.20, render/present/ms 1.50, physics/insert/ms 0.65, physics/step/ms 0.61, bigspace/total/ms 0.60, terrain/update/ms 0.46, controller/move/ms 0.44, controller/feed/ms 0.43 |
| measure | 1.02 | 59.5 | pipelines/wait_ms 22.4 k, pipelines/compile_ms 2283, pipelines/driver_ms 2155, pipelines/front_ms 127, render/thread/ms 108, render/acquire/ms 85.58, render/busy/ms 20.89, main/busy/ms 15.50, render/graph/ms 13.26, render/present/ms 1.16, bigspace/total/ms 0.69, physics/insert/ms 0.55 |
| measure | 1.25 | 59.1 | render/thread/ms 108, render/acquire/ms 85.44, render/busy/ms 21.39, main/busy/ms 13.49, render/graph/ms 12.51, render/present/ms 1.06, bigspace/total/ms 0.70, terrain/update/ms 0.46, terrain/select/ms 0.40, stream/drive/ms 0.34, far/select/ms 0.32, controller/feed/ms 0.32 |
| measure | 1.36 | 59.2 | render/thread/ms 113, render/acquire/ms 91.70, render/busy/ms 20.02, main/busy/ms 16.36, render/graph/ms 11.70, render/present/ms 1.48, bigspace/total/ms 0.74, terrain/update/ms 0.48, terrain/select/ms 0.41, stream/drive/ms 0.39, far/select/ms 0.35, stream/tick/ms 0.33 |
| measure | 3.53 | 58.3 | render/thread/ms 110, render/acquire/ms 91.43, render/busy/ms 17.34, main/busy/ms 15.05, render/graph/ms 10.31, render/present/ms 1.10, bigspace/total/ms 0.75, terrain/update/ms 0.44, stream/drive/ms 0.38, terrain/select/ms 0.38, far/select/ms 0.33, stream/tick/ms 0.32 |
| measure | 4.07 | 58.8 | render/thread/ms 113, render/acquire/ms 92.75, render/busy/ms 19.34, main/busy/ms 13.39, render/graph/ms 11.56, render/present/ms 1.20, bigspace/total/ms 0.59, terrain/update/ms 0.52, terrain/select/ms 0.46, stream/drive/ms 0.34, far/select/ms 0.31, controller/feed/ms 0.29 |
| measure | 4.41 | 58.2 | render/thread/ms 112, render/acquire/ms 93.93, render/busy/ms 17.17, main/busy/ms 13.69, render/graph/ms 10.07, render/present/ms 0.89, bigspace/total/ms 0.62, stream/drive/ms 0.51, controller/feed/ms 0.48, terrain/update/ms 0.45, stream/tick/ms 0.45, terrain/select/ms 0.39 |
| measure | 4.53 | 62.7 | render/thread/ms 114, render/acquire/ms 93.65, render/busy/ms 19.50, main/busy/ms 13.63, render/graph/ms 11.95, render/present/ms 1.04, bigspace/total/ms 0.52, terrain/update/ms 0.43, terrain/select/ms 0.38, stream/drive/ms 0.33, far/select/ms 0.31, stream/tick/ms 0.28 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 9%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 92%) · 2580 MHz (min 2580) · memory 8001 MHz · 65.0 W · 73 °C (max 80) · VRAM 7830 MiB · throttled 100% of samples
