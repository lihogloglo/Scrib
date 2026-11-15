# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1404 frames over 15.0 s · **median 10.50 ms (95 fps)** · mean 10.69 · p95 12.34 · p99 14.14 (1% low 71 fps) · worst 39.23 ms · 11 spikes over 1.5× median · 44.0 stutters/min (work 24.0/min)
- warm-up: 297 frames over 9.6 s, worst 3855.1 ms
- hitches (> 26.2 ms): **3** measured, 37 in warm-up
- timed GPU passes: 8.22 ms of a 10.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.50 ms, limit 11 ms
- **FAIL** worst frame 39.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.549 | 2.719 | 3.123 | 0.94 | 0.088 |
| dlss_super_resolution | 2.018 | 2.153 | 2.473 | 0.94 | 0.186 |
| early prepass | 0.593 | 0.633 | 0.958 | 0.94 | 0.118 |
| atmosphere_luts | 0.340 | 0.362 | 0.557 | 0.94 | 0.004 |
| bloom | 0.319 | 0.340 | 0.499 | 0.94 | 0.070 |
| main_transmissive_pass_3d | 0.273 | 0.291 | 0.447 | 0.94 | 0.009 |
| water_velocity | 0.265 | 0.283 | 0.431 | 0.94 | 0.002 |
| water_sspr | 0.220 | 0.235 | 0.375 | 0.94 | 0.003 |
| uniform_allocation | 0.199 | 0.212 | 0.220 | 0.94 | 0.040 |
| ssao | 0.197 | 0.210 | 0.354 | 0.94 | 0.009 |
| bin_unpacking | 0.158 | 0.168 | 0.176 | 0.94 | 0.028 |
| water_fft | 0.134 | 0.143 | 0.138 | 0.94 | 0.003 |
| atmos_froxel_inject | 0.131 | 0.139 | 0.136 | 0.94 | 0.002 |
| upscaling | 0.127 | 0.135 | 0.164 | 0.94 | 0.005 |
| clouds_composite | 0.106 | 0.113 | 0.110 | 0.94 | 0.003 |
| tonemapping | 0.094 | 0.101 | 0.109 | 0.94 | 0.004 |
| clouds_trace | 0.066 | 0.071 | 0.092 | 0.94 | 0.001 |
| clustering | 0.063 | 0.067 | 0.073 | 0.94 | 0.024 |
| render_sky | 0.052 | 0.056 | 0.056 | 0.94 | 0.002 |
| clouds_resolve | 0.048 | 0.051 | 0.053 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.438 | 0.758 | 0.11 | 0.001 |
| water_snapshot | 0.036 | 0.039 | 0.053 | 0.94 | 0.002 |
| early_mesh_preprocessing | 0.031 | 0.033 | 0.034 | 0.94 | 0.007 |
| atmos_froxel_integrate | 0.029 | 0.031 | 0.033 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.236 | 0.237 | 0.11 | 0.005 |
| main_transparent_pass_3d | 0.023 | 0.024 | 0.027 | 0.94 | 0.072 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.94 | 0.003 |
| per_view_shadow_pass | 0.014 | 0.015 | 0.016 | 0.94 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.003 |
| night_sky | 0.007 | 0.007 | 0.010 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.004 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.94 | 0.004 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.014 | 0.014 | 0.11 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.94 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 148.641 | 0.000 | 11391.85 | 208692.6 |
| render/thread/ms | 9.681 | 11.290 | 35.12 | 13592.0 |
| render/busy/ms | 8.824 | 10.068 | 34.47 | 12388.6 |
| main/busy/ms | 7.688 | 9.207 | 20.42 | 10793.5 |
| render/graph/ms | 5.218 | 6.203 | 7.46 | 7326.1 |
| pipelines/compile_ms | 2.388 | 0.000 | 150.32 | 3352.7 |
| pipelines/front_ms | 2.289 | 0.000 | 144.67 | 3213.7 |
| render/present/ms | 0.580 | 1.390 | 18.78 | 813.9 |
| bigspace/total/ms | 0.372 | 0.716 | 2.95 | 522.2 |
| render/acquire/ms | 0.277 | 1.190 | 2.24 | 389.5 |
| terrain/update/ms | 0.238 | 0.309 | 0.70 | 334.3 |
| stream/drive/ms | 0.234 | 0.319 | 0.65 | 328.9 |
| terrain/select/ms | 0.205 | 0.264 | 0.66 | 287.8 |
| stream/tick/ms | 0.202 | 0.283 | 0.48 | 283.8 |
| far/select/ms | 0.190 | 0.275 | 0.61 | 266.3 |
| controller/feed/ms | 0.165 | 0.214 | 0.50 | 232.3 |
| bigspace/high_precision/ms | 0.132 | 0.267 | 2.55 | 184.6 |
| controller/move/ms | 0.101 | 0.196 | 1.08 | 142.5 |
| pipelines/driver_ms | 0.099 | 0.000 | 10.04 | 139.0 |
| bigspace/low_precision/ms | 0.094 | 0.195 | 2.20 | 132.3 |
| anim/evaluate/ms | 0.068 | 0.163 | 2.18 | 95.3 |
| anim/evaluate_cpu/ms | 0.059 | 0.083 | 0.50 | 82.7 |
| physics/step/ms | 0.046 | 0.073 | 3.82 | 64.8 |
| physics/insert/ms | 0.036 | 0.192 | 1.84 | 49.9 |
| weather/map/ms | 0.018 | 0.023 | 0.17 | 24.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 15.8 |
| script/update/ms | 0.009 | 0.014 | 0.05 | 12.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.63 | 8.0 |
| script/input/ms | 0.006 | 0.007 | 0.13 | 7.8 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.0 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.11 | 2.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.18 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.7 |
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
| anim/a2/count | count | 0 | 23 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 81 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 30 | 8 |
| crowd/c0/joints/count | count | 272 | 1020 | 272 |
| crowd/c0/meshes/count | count | 48 | 180 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 10.3 k | 2958 |
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
| frame/work/ms | ms | 9.46 | 34.47 | 10.69 |
| gpu/timed/ms | ms | 9.36 | 10.33 | 9.83 |
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
| phases/opaque3d/batch_sets | count | 33 | 33 | 33 |
| phases/opaque3d/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 33 | 33 |
| phases/opaque3d_prepass/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 29 | 29 | 29 |
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
| physics/pending | count | 0 | 2135 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 193 | 193 | 193 |
| pipelines/waiting | count | 0 | 39 | 0 |
| preprocess/bind_groups | count | 0 | 232 | 0 |
| preprocess/bind_groups_created | count | 0 | 232 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 240 | 240 | 240 |
| process/cpu_usage | % | 30.29 | 42.79 | 33.17 |
| process/mem_usage | GiB | 4.25 | 4.29 | 4.29 |
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
| started_load_count |  | 3545 | 3547 | 3547 |
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
| system/cpu_usage | % | 43.80 | 64.63 | 49.02 |
| system/mem_usage | % | 62.97 | 63.13 | 57.78 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 109 | 109 | 109 |
| views/all/bins | count | 4151 | 4151 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 90 | 90 |
| views/all/pipelines_distinct | count | 89 | 89 | 89 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 29 | 29 | 29 |
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
| views/main/batch_sets | count | 80 | 80 | 80 |
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
| anim/bones_written/count | count | 0.28 | 363 | 396 |
| anim/events/count | count | 0.09 | 4 | 122 |
| assets/Image/added | count | 0.13 | 36 | 178 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.36 | 78 | 499 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.14 | 36 | 193 |
| assets/StandardMaterial/modified | count | 3.53 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 6.73 | 3783 | 9443 |
| physics/inserts | count | 1.38 | 211 | 1944 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 6 | 16 |
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

## Hitches (> 26.2 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.86 | 3855.1 | main/busy/ms 3768, stream/drive/ms 1.82, stream/tick/ms 1.03, bigspace/total/ms 0.82, surface/advance_cpu_ms 0.77, surface/cell_upload_ms 0.40, weather/map/ms 0.28, bigspace/high_precision/ms 0.27, weather/eval/ms 0.25, far/select/ms 0.10, script/update/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 4.87 | 1010.6 | main/busy/ms 3831, render/thread/ms 968, render/busy/ms 787, render/acquire/ms 178, render/graph/ms 9.52, stream/drive/ms 4.44, render/present/ms 2.60, stream/tick/ms 2.50, bigspace/total/ms 0.93, surface/advance_cpu_ms 0.77, terrain/update/ms 0.68, surface/cell_upload_ms 0.40 |
| warm-up | 5.16 | 103.8 | pipelines/compile_ms 238, pipelines/front_ms 216, main/busy/ms 71.19, render/thread/ms 68.74, render/busy/ms 66.92, pipelines/wait_ms 44.74, pipelines/driver_ms 22.10, render/graph/ms 4.10, render/present/ms 1.52, stream/drive/ms 1.35, stream/tick/ms 1.05, terrain/update/ms 0.77 |
| warm-up | 5.34 | 68.5 | render/thread/ms 54.87, main/busy/ms 54.30, render/busy/ms 53.05, stream/drive/ms 40.88, render/graph/ms 5.29, render/present/ms 1.61, stream/tick/ms 0.82, terrain/update/ms 0.73, terrain/select/ms 0.58, stream/spawn/ms 0.32, bigspace/total/ms 0.25, render/acquire/ms 0.21 |
| warm-up | 5.41 | 59.3 | pipelines/wait_ms 751, pipelines/compile_ms 247, pipelines/front_ms 231, main/busy/ms 43.57, render/thread/ms 38.31, render/busy/ms 36.44, stream/drive/ms 23.43, pipelines/driver_ms 15.85, render/graph/ms 5.25, stream/spawn/ms 2.41, render/present/ms 1.59, stream/tick/ms 1.57 |
| warm-up | 5.44 | 30.1 | pipelines/wait_ms 751, pipelines/compile_ms 247, pipelines/front_ms 231, render/thread/ms 48.77, render/busy/ms 46.93, main/busy/ms 34.86, pipelines/driver_ms 15.85, render/graph/ms 6.32, stream/drive/ms 6.29, stream/spawn/ms 3.15, stream/tick/ms 2.09, render/present/ms 1.58 |
| warm-up | 5.49 | 48.8 | render/thread/ms 48.50, render/busy/ms 46.72, main/busy/ms 39.82, terrain/update/ms 6.76, render/graph/ms 5.27, stream/drive/ms 4.58, stream/spawn/ms 2.02, stream/tick/ms 1.57, render/present/ms 1.42, terrain/select/ms 0.67, bigspace/total/ms 0.44, render/acquire/ms 0.36 |
| warm-up | 5.53 | 39.1 | main/busy/ms 73.75, render/thread/ms 63.14, render/busy/ms 61.21, stream/drive/ms 38.77, terrain/update/ms 6.55, render/graph/ms 4.82, stream/spawn/ms 2.55, render/present/ms 1.54, stream/tick/ms 1.08, terrain/select/ms 0.56, bigspace/total/ms 0.42, render/acquire/ms 0.40 |
| warm-up | 5.59 | 61.8 | main/busy/ms 81.48, render/thread/ms 59.39, render/busy/ms 57.16, stream/drive/ms 45.06, stream/spawn/ms 8.24, render/graph/ms 5.15, render/present/ms 1.82, stream/tick/ms 0.93, stream/activate/ms 0.82, physics/insert/ms 0.79, terrain/update/ms 0.68, terrain/select/ms 0.51 |
| warm-up | 5.64 | 51.0 | pipelines/wait_ms 1902, pipelines/compile_ms 284, pipelines/front_ms 256, main/busy/ms 89.40, render/thread/ms 60.83, render/busy/ms 58.32, stream/drive/ms 57.06, pipelines/driver_ms 27.89, stream/spawn/ms 7.31, render/graph/ms 5.30, render/present/ms 2.07, stream/activate/ms 1.23 |
| warm-up | 5.74 | 94.7 | pipelines/wait_ms 6090, pipelines/compile_ms 401, pipelines/front_ms 365, main/busy/ms 77.46, render/thread/ms 69.27, render/busy/ms 67.23, stream/drive/ms 51.43, pipelines/driver_ms 36.60, render/graph/ms 4.69, stream/spawn/ms 2.26, render/present/ms 1.72, stream/tick/ms 1.07 |
| warm-up | 5.83 | 94.0 | pipelines/wait_ms 5873, pipelines/compile_ms 149, pipelines/front_ms 138, render/thread/ms 121, render/busy/ms 61.44, render/present/ms 59.39, main/busy/ms 38.70, pipelines/driver_ms 10.79, stream/drive/ms 9.21, render/graph/ms 4.00, stream/tick/ms 0.79, stream/spawn/ms 0.78 |
| warm-up | 5.91 | 62.4 | main/busy/ms 56.79, render/thread/ms 46.01, render/busy/ms 44.28, stream/drive/ms 38.35, render/graph/ms 4.42, render/present/ms 1.54, stream/spawn/ms 1.38, stream/tick/ms 1.07, terrain/update/ms 0.65, terrain/select/ms 0.51, stream/activate/ms 0.44, bigspace/total/ms 0.31 |
| warm-up | 5.94 | 30.3 | pipelines/wait_ms 3658, pipelines/compile_ms 167, pipelines/front_ms 160, main/busy/ms 53.81, render/thread/ms 44.29, render/busy/ms 42.99, stream/drive/ms 37.51, pipelines/driver_ms 6.76, render/graph/ms 3.25, stream/tick/ms 1.19, render/present/ms 1.10, stream/spawn/ms 0.62 |
| warm-up | 6.07 | 61.5 | main/busy/ms 108, stream/drive/ms 95.28, render/thread/ms 26.61, render/busy/ms 25.17, render/graph/ms 3.34, stream/tick/ms 1.35, render/present/ms 1.21, terrain/update/ms 0.80, terrain/select/ms 0.64, bigspace/total/ms 0.28, stream/activate/ms 0.26, render/acquire/ms 0.22 |
| warm-up | 6.12 | 57.6 | pipelines/wait_ms 784, pipelines/compile_ms 163, pipelines/front_ms 152, main/busy/ms 73.91, stream/drive/ms 60.33, render/thread/ms 24.03, render/busy/ms 22.49, pipelines/driver_ms 11.14, render/graph/ms 2.79, render/present/ms 1.34, stream/spawn/ms 1.19, stream/tick/ms 0.95 |
| warm-up | 6.36 | 34.3 | pipelines/wait_ms 3919, pipelines/compile_ms 173, pipelines/front_ms 157, render/thread/ms 37.71, render/busy/ms 35.91, main/busy/ms 31.11, pipelines/driver_ms 16.21, stream/drive/ms 13.12, render/graph/ms 4.74, stream/spawn/ms 3.02, render/present/ms 1.52, stream/tick/ms 0.86 |
| warm-up | 6.41 | 29.4 | render/thread/ms 33.68, render/busy/ms 31.69, main/busy/ms 24.43, stream/drive/ms 6.11, render/graph/ms 5.76, stream/spawn/ms 2.90, stream/tick/ms 1.75, render/present/ms 1.68, terrain/update/ms 0.74, terrain/select/ms 0.59, bigspace/total/ms 0.45, render/acquire/ms 0.30 |
| warm-up | 6.46 | 26.5 | pipelines/wait_ms 4505, pipelines/compile_ms 97.15, pipelines/front_ms 92.62, render/thread/ms 33.03, render/busy/ms 31.57, main/busy/ms 22.51, pipelines/driver_ms 4.53, stream/drive/ms 4.21, render/graph/ms 3.42, stream/spawn/ms 2.08, render/present/ms 1.21, stream/tick/ms 1.07 |
| warm-up | 6.48 | 28.1 | pipelines/wait_ms 9018, pipelines/compile_ms 51.21, pipelines/front_ms 44.50, render/thread/ms 43.34, render/busy/ms 41.98, main/busy/ms 18.56, pipelines/driver_ms 6.71, render/graph/ms 3.74, stream/drive/ms 3.43, stream/spawn/ms 1.55, render/present/ms 1.12, stream/tick/ms 0.89 |
| warm-up | 6.59 | 29.1 | pipelines/wait_ms 9602, pipelines/compile_ms 75.25, pipelines/front_ms 70.43, render/thread/ms 47.10, render/busy/ms 45.81, main/busy/ms 25.85, stream/drive/ms 5.75, pipelines/driver_ms 4.82, render/graph/ms 4.61, stream/spawn/ms 2.35, render/present/ms 1.04, bigspace/total/ms 0.75 |
| warm-up | 6.64 | 27.6 | pipelines/wait_ms 8200, pipelines/compile_ms 54.04, pipelines/front_ms 47.90, render/thread/ms 43.64, render/busy/ms 41.81, main/busy/ms 23.89, render/graph/ms 6.25, pipelines/driver_ms 6.14, stream/drive/ms 4.41, stream/spawn/ms 2.05, render/present/ms 1.53, stream/tick/ms 1.13 |
| warm-up | 6.67 | 33.4 | pipelines/wait_ms 4864, pipelines/compile_ms 53.19, render/thread/ms 46.18, pipelines/front_ms 44.46, render/busy/ms 44.25, main/busy/ms 27.53, pipelines/driver_ms 8.73, render/graph/ms 6.37, stream/drive/ms 4.80, stream/spawn/ms 2.06, render/present/ms 1.67, stream/tick/ms 1.33 |
| warm-up | 6.70 | 33.1 | pipelines/wait_ms 8496, pipelines/compile_ms 53.89, render/thread/ms 47.44, pipelines/front_ms 47.24, render/busy/ms 45.64, main/busy/ms 26.86, render/graph/ms 6.85, pipelines/driver_ms 6.64, stream/drive/ms 4.71, stream/spawn/ms 2.13, render/present/ms 1.55, stream/tick/ms 0.94 |
| warm-up | 6.75 | 30.1 | render/thread/ms 43.42, render/busy/ms 42.29, main/busy/ms 18.96, render/graph/ms 4.69, stream/drive/ms 3.89, stream/spawn/ms 2.10, render/present/ms 0.88, stream/tick/ms 0.71, terrain/update/ms 0.55, bigspace/total/ms 0.54, terrain/select/ms 0.43, bigspace/high_precision/ms 0.29 |
| warm-up | 6.80 | 30.1 | render/thread/ms 35.46, render/busy/ms 33.25, main/busy/ms 28.40, render/graph/ms 8.87, stream/drive/ms 7.47, stream/spawn/ms 3.28, render/present/ms 1.99, stream/tick/ms 1.02, bigspace/total/ms 0.96, terrain/update/ms 0.63, terrain/select/ms 0.49, bigspace/high_precision/ms 0.36 |
| warm-up | 6.84 | 38.7 | render/thread/ms 47.62, render/busy/ms 41.98, main/busy/ms 27.30, render/graph/ms 5.89, render/present/ms 5.30, stream/drive/ms 4.54, stream/spawn/ms 2.45, stream/tick/ms 1.41, bigspace/total/ms 0.79, terrain/update/ms 0.58, terrain/select/ms 0.45, bigspace/high_precision/ms 0.34 |
| warm-up | 6.87 | 31.8 | pipelines/wait_ms 7296, pipelines/compile_ms 175, pipelines/front_ms 164, render/thread/ms 48.01, main/busy/ms 42.74, render/busy/ms 42.43, stream/drive/ms 16.90, pipelines/driver_ms 10.18, render/graph/ms 6.05, render/present/ms 5.25, stream/spawn/ms 2.08, stream/tick/ms 1.18 |
| warm-up | 6.91 | 40.3 | pipelines/wait_ms 19.1 k, pipelines/compile_ms 236, pipelines/front_ms 222, main/busy/ms 44.68, render/thread/ms 39.73, render/busy/ms 37.98, stream/drive/ms 16.68, pipelines/driver_ms 14.05, render/graph/ms 5.29, terrain/update/ms 4.10, stream/spawn/ms 2.12, render/present/ms 1.55 |
| warm-up | 6.97 | 37.4 | pipelines/wait_ms 4067, pipelines/compile_ms 56.94, pipelines/front_ms 53.59, main/busy/ms 49.08, render/thread/ms 32.66, render/busy/ms 31.26, stream/drive/ms 8.24, render/graph/ms 5.07, physics/insert/ms 5.06, pipelines/driver_ms 3.35, terrain/update/ms 2.98, physics/step/ms 2.65 |
| warm-up | 7.10 | 55.8 | main/busy/ms 57.74, render/thread/ms 29.61, render/busy/ms 28.03, stream/drive/ms 20.19, render/graph/ms 4.74, stream/spawn/ms 2.17, render/present/ms 1.24, stream/tick/ms 1.07, bigspace/total/ms 0.56, terrain/update/ms 0.55, terrain/select/ms 0.45, physics/insert/ms 0.35 |
| warm-up | 7.25 | 32.7 | pipelines/wait_ms 4365, pipelines/compile_ms 272, pipelines/front_ms 256, render/thread/ms 36.32, render/busy/ms 34.89, main/busy/ms 30.14, pipelines/driver_ms 16.15, stream/drive/ms 12.81, render/graph/ms 6.56, stream/spawn/ms 2.06, render/present/ms 1.17, stream/tick/ms 0.77 |
| warm-up | 7.40 | 30.4 | pipelines/wait_ms 943, pipelines/compile_ms 169, pipelines/front_ms 162, render/thread/ms 32.18, render/busy/ms 30.11, main/busy/ms 26.46, render/graph/ms 12.68, pipelines/driver_ms 7.68, stream/drive/ms 3.42, render/present/ms 1.79, stream/spawn/ms 1.51, stream/tick/ms 1.20 |
| warm-up | 7.43 | 35.7 | pipelines/wait_ms 943, pipelines/compile_ms 169, pipelines/front_ms 162, render/thread/ms 39.37, render/busy/ms 37.67, main/busy/ms 33.11, stream/drive/ms 11.37, render/graph/ms 10.71, pipelines/driver_ms 7.68, stream/tick/ms 1.58, render/present/ms 1.45, stream/spawn/ms 1.08 |
| warm-up | 7.48 | 41.8 | render/thread/ms 46.25, render/busy/ms 44.88, main/busy/ms 32.44, stream/drive/ms 12.01, render/graph/ms 4.68, stream/spawn/ms 2.31, render/present/ms 1.13, stream/tick/ms 1.13, bigspace/total/ms 0.69, terrain/update/ms 0.67, terrain/select/ms 0.57, bigspace/high_precision/ms 0.33 |
| warm-up | 8.03 | 28.8 | main/busy/ms 40.11, render/thread/ms 37.05, render/busy/ms 33.69, render/graph/ms 9.76, render/present/ms 3.08, bigspace/total/ms 1.64, stream/drive/ms 1.40, terrain/update/ms 0.72, stream/tick/ms 0.65, bigspace/low_precision/ms 0.64, terrain/select/ms 0.60, bigspace/high_precision/ms 0.50 |
| warm-up | 8.10 | 38.3 | main/busy/ms 44.17, render/thread/ms 39.77, render/busy/ms 38.15, render/graph/ms 7.76, stream/drive/ms 1.70, render/present/ms 1.34, bigspace/total/ms 1.10, anim/evaluate_cpu/ms 0.84, stream/tick/ms 0.75, terrain/update/ms 0.56, bigspace/low_precision/ms 0.54, terrain/select/ms 0.45 |
| measure | 0.11 | 29.6 | render/thread/ms 37.00, render/present/ms 19.30, render/busy/ms 17.05, main/busy/ms 15.21, render/graph/ms 8.60, render/acquire/ms 0.66, stream/drive/ms 0.62, bigspace/total/ms 0.58, stream/tick/ms 0.55, terrain/update/ms 0.49, far/select/ms 0.45, terrain/select/ms 0.42 |
| measure | 0.53 | 33.3 | main/busy/ms 40.42, render/thread/ms 34.39, render/busy/ms 32.89, render/graph/ms 11.37, bigspace/total/ms 1.94, render/present/ms 1.25, anim/evaluate_cpu/ms 0.73, terrain/update/ms 0.73, terrain/select/ms 0.65, anim/evaluate/ms 0.56, bigspace/low_precision/ms 0.39, stream/drive/ms 0.38 |
| measure | 0.63 | 39.2 | pipelines/wait_ms 18.4 k, pipelines/compile_ms 65.05, pipelines/front_ms 53.94, render/thread/ms 46.24, render/busy/ms 44.96, main/busy/ms 27.16, pipelines/driver_ms 11.11, render/graph/ms 10.59, render/present/ms 1.11, bigspace/total/ms 1.07, physics/insert/ms 0.97, physics/step/ms 0.50 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\h1_old\balmora_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **95%** (min 76%) · 2576 MHz (min 2565) · memory 8001 MHz · 90.7 W · 77 °C (max 80) · VRAM 5067 MiB · throttled 100% of samples
