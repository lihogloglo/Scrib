# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: waterssr,shorewaves,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5319 frames over 65.6 s · **median 12.26 ms (82 fps)** · mean 12.34 · p95 14.63 · p99 17.24 (1% low 58 fps) · worst 33.11 ms · 32 spikes over 1.5× median · 24.7 stutters/min (work 19.2/min)
- warm-up: 370 frames over 8.3 s, worst 2498.1 ms
- hitches (> 30.7 ms): **2** measured, 20 in warm-up
- timed GPU passes: 9.24 ms of a 12.34 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.26 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- **FAIL** worst frame 33.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.827 | 4.015 | 5.014 | 0.95 | 0.062 |
| ssao | 0.762 | 0.800 | 1.100 | 0.95 | 0.008 |
| early prepass | 0.656 | 0.688 | 1.067 | 0.95 | 0.099 |
| clouds_composite | 0.441 | 0.463 | 0.794 | 0.95 | 0.002 |
| taa | 0.390 | 0.409 | 0.740 | 0.95 | 0.002 |
| bloom | 0.322 | 0.338 | 0.624 | 0.95 | 0.068 |
| clouds_trace | 0.320 | 0.336 | 0.597 | 0.95 | 0.001 |
| atmosphere_luts | 0.314 | 0.329 | 0.578 | 0.95 | 0.003 |
| uniform_allocation | 0.212 | 0.223 | 0.231 | 0.95 | 0.039 |
| render_sky | 0.203 | 0.213 | 0.214 | 0.95 | 0.002 |
| clouds_resolve | 0.187 | 0.196 | 0.205 | 0.95 | 0.001 |
| water_snapshot | 0.173 | 0.182 | 0.231 | 0.95 | 0.002 |
| auxiliary_views | 0.169 | 0.177 | 0.681 | 0.95 | 0.161 |
| bin_unpacking | 0.164 | 0.172 | 0.183 | 0.95 | 0.028 |
| atmos_froxel_inject | 0.154 | 0.161 | 0.159 | 0.95 | 0.002 |
| water_fft | 0.134 | 0.141 | 0.140 | 0.95 | 0.003 |
| upscaling | 0.124 | 0.130 | 0.141 | 0.95 | 0.004 |
| water_velocity | 0.105 | 0.110 | 0.113 | 0.95 | 0.002 |
| main_transmissive_pass_3d | 0.088 | 0.092 | 0.247 | 0.95 | 0.008 |
| tonemapping | 0.087 | 0.091 | 0.093 | 0.95 | 0.003 |
| clustering | 0.066 | 0.069 | 0.075 | 0.95 | 0.020 |
| lightprobe_irradiance_map | 0.050 | 0.439 | 0.758 | 0.11 | 0.001 |
| surface_trail | 0.045 | 0.060 | 0.058 | 0.75 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.95 | 0.006 |
| atmos_froxel_integrate | 0.034 | 0.035 | 0.036 | 0.95 | 0.001 |
| clouds_shadow | 0.029 | 0.030 | 0.033 | 0.95 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.236 | 0.240 | 0.11 | 0.004 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.031 | 0.95 | 0.007 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.95 | 0.002 |
| night_sky | 0.019 | 0.020 | 0.047 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.95 | 0.113 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.012 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 26.801 | 0.000 | 13490.83 | 142556.9 |
| render/thread/ms | 11.105 | 13.238 | 31.44 | 59067.1 |
| render/busy/ms | 9.767 | 12.246 | 30.54 | 51951.2 |
| main/busy/ms | 9.084 | 10.825 | 25.34 | 48320.2 |
| render/graph/ms | 5.385 | 6.991 | 18.14 | 28642.6 |
| render/acquire/ms | 0.795 | 2.554 | 5.84 | 4226.5 |
| render/present/ms | 0.543 | 0.750 | 16.16 | 2889.4 |
| bigspace/total/ms | 0.505 | 1.004 | 3.01 | 2686.1 |
| pipelines/compile_ms | 0.475 | 0.000 | 168.98 | 2525.1 |
| pipelines/front_ms | 0.459 | 0.000 | 162.67 | 2440.8 |
| terrain/update/ms | 0.305 | 0.392 | 1.87 | 1623.7 |
| controller/feed/ms | 0.295 | 0.397 | 0.88 | 1569.9 |
| terrain/select/ms | 0.272 | 0.348 | 1.03 | 1444.5 |
| stream/drive/ms | 0.246 | 0.361 | 17.46 | 1308.3 |
| bigspace/low_precision/ms | 0.194 | 0.374 | 2.79 | 1034.4 |
| stream/tick/ms | 0.190 | 0.271 | 0.73 | 1010.5 |
| far/select/ms | 0.190 | 0.277 | 0.61 | 1009.0 |
| anim/evaluate/ms | 0.179 | 0.329 | 1.67 | 949.6 |
| anim/evaluate_cpu/ms | 0.161 | 0.206 | 0.59 | 854.1 |
| bigspace/high_precision/ms | 0.132 | 0.312 | 2.13 | 703.9 |
| controller/move/ms | 0.122 | 0.257 | 0.59 | 650.2 |
| physics/insert/ms | 0.061 | 0.459 | 1.00 | 324.3 |
| physics/step/ms | 0.055 | 0.092 | 1.24 | 291.5 |
| weather/map/ms | 0.025 | 0.034 | 0.30 | 135.1 |
| pipelines/driver_ms | 0.016 | 0.000 | 7.27 | 84.3 |
| anim/apply/ms | 0.012 | 0.020 | 0.28 | 63.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.33 | 57.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.22 | 54.9 |
| script/update/ms | 0.007 | 0.009 | 0.07 | 38.0 |
| stream/despawn/ms | 0.005 | 0.000 | 0.97 | 29.2 |
| script/input/ms | 0.005 | 0.007 | 0.31 | 27.5 |
| script/ui/ms | 0.005 | 0.007 | 0.15 | 26.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.0 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 18.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 15.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.06 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1313 | 2262 | 538 |
| crowd/c0/meshes/count | count | 357 | 646 | 129 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5501 | 5928 | 5626 |
| crowd/c1/meshes/count | count | 1166 | 1270 | 1204 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6390 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4254 | 8979 | 6179 |
| crowd/joints_uploaded/count | count | 4232 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.72 | 30.54 | 11.10 |
| gpu/timed/ms | ms | 10.52 | 14.53 | 11.10 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 538.36 M | 539.54 M | 539.54 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 188 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 866 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 188 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 866 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3512 | 5132 | 3178 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3512 | 5132 | 3178 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 52 | 315 | 39 |
| phases/shadow/bins | count | 3923 | 5547 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9709 | 13.0 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 151 | 318 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2653 | 3268 | 2346 |
| physics/items | count | 200 | 476 | 19 |
| physics/pending | count | 0 | 1026 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 206 | 210 | 210 |
| pipelines/waiting | count | 0 | 31 | 0 |
| preprocess/bind_groups | count | 12 | 799 | 6 |
| preprocess/bind_groups_created | count | 12 | 799 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 222 | 362 | 215 |
| process/cpu_usage | % | 28.37 | 44.23 | 26.44 |
| process/mem_usage | GiB | 5.33 | 5.78 | 5.78 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 528 | 396 |
| statics/materials_bound | count | 435 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.41 | 56.76 | 30.80 |
| system/mem_usage | % | 76.43 | 77.96 | 77.87 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2794 | 2906 | 2906 |
| textures/images | count | 2589 | 2644 | 2644 |
| views/all/batch_sets | count | 109 | 367 | 99 |
| views/all/bins | count | 6561 | 8405 | 5700 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.7 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 206 | 370 | 95 |
| views/all/pipelines_distinct | count | 83 | 94 | 77 |
| views/all/work_items | count | 17.6 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 63 | 39 |
| views/cascade/bins | count | 2621 | 3141 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8234 | 10.2 k | 5671 |
| views/cascade/pipelines | count | 48 | 63 | 39 |
| views/cascade/work_items | count | 8234 | 10.2 k | 5671 |
| views/lamp_face/batch_sets | count | 0 | 262 | 0 |
| views/lamp_face/bins | count | 1188 | 2603 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1363 | 2896 | 0 |
| views/lamp_face/pipelines | count | 103 | 267 | 0 |
| views/lamp_face/work_items | count | 0 | 2766 | 0 |
| views/main/batch_sets | count | 58 | 64 | 60 |
| views/main/bins | count | 2886 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8774 | 13.0 k | 7590 |
| views/main/pipelines | count | 54 | 60 | 56 |
| views/main/work_items | count | 8928 | 13.3 k | 7898 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 55.06 | 238 | 292.9 k |
| anim/events/count | count | 0.09 | 3 | 503 |
| assets/Image/added | count | 0.12 | 43 | 617 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.78 | 152 | 4149 |
| assets/Mesh/removed | count | 0.92 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 492 |
| assets/StandardMaterial/modified | count | 4.10 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.07 | 1389 | 5704 |
| physics/inserts | count | 0.61 | 151 | 3219 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.74 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6364 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1395 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.7 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2498.1 | main/busy/ms 2476, surface/advance_cpu_ms 67.54, surface/cell_upload_ms 0.70, stream/drive/ms 0.31, weather/map/ms 0.15, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.06 | 562.0 | main/busy/ms 2516, render/thread/ms 528, render/busy/ms 507, surface/advance_cpu_ms 67.54, render/acquire/ms 18.45, render/graph/ms 3.75, render/present/ms 2.27, stream/drive/ms 1.24, surface/cell_upload_ms 0.70, stream/tick/ms 0.66, terrain/update/ms 0.37, bigspace/total/ms 0.28 |
| warm-up | 3.28 | 223.9 | pipelines/wait_ms 1263, render/thread/ms 567, render/busy/ms 544, pipelines/compile_ms 208, pipelines/front_ms 194, main/busy/ms 58.35, render/graph/ms 33.50, terrain/update/ms 21.96, render/acquire/ms 18.64, pipelines/driver_ms 14.13, render/present/ms 4.17, stream/drive/ms 1.61 |
| warm-up | 3.49 | 100.4 | pipelines/compile_ms 198, pipelines/front_ms 182, main/busy/ms 65.49, render/thread/ms 55.59, render/busy/ms 53.85, pipelines/wait_ms 33.57, pipelines/driver_ms 15.93, render/graph/ms 3.09, stream/drive/ms 1.77, stream/tick/ms 1.51, render/present/ms 1.26, terrain/update/ms 0.65 |
| warm-up | 3.64 | 68.3 | main/busy/ms 46.06, render/thread/ms 37.46, render/busy/ms 35.98, stream/drive/ms 31.83, render/graph/ms 1.91, render/present/ms 1.13, stream/tick/ms 0.82, terrain/update/ms 0.73, stream/spawn/ms 0.63, terrain/select/ms 0.59, render/acquire/ms 0.35, bigspace/total/ms 0.34 |
| warm-up | 3.76 | 69.5 | pipelines/wait_ms 2758, pipelines/compile_ms 87.66, pipelines/front_ms 79.08, render/thread/ms 56.35, render/busy/ms 48.90, main/busy/ms 43.43, stream/drive/ms 24.58, pipelines/driver_ms 8.58, render/present/ms 7.16, render/graph/ms 5.84, stream/spawn/ms 2.10, stream/tick/ms 2.04 |
| warm-up | 3.91 | 117.4 | pipelines/wait_ms 2263, pipelines/compile_ms 88.97, render/thread/ms 83.92, render/busy/ms 82.59, pipelines/front_ms 78.84, main/busy/ms 60.73, stream/drive/ms 33.88, pipelines/driver_ms 10.13, stream/spawn/ms 6.33, render/graph/ms 2.44, stream/tick/ms 1.05, render/present/ms 1.03 |
| warm-up | 3.95 | 36.2 | pipelines/wait_ms 1524, render/thread/ms 91.54, render/busy/ms 89.98, pipelines/compile_ms 66.38, pipelines/front_ms 61.44, main/busy/ms 33.67, stream/drive/ms 8.66, stream/spawn/ms 5.54, pipelines/driver_ms 4.95, render/graph/ms 4.27, terrain/update/ms 1.32, render/present/ms 1.24 |
| warm-up | 4.03 | 31.4 | pipelines/wait_ms 684, render/thread/ms 49.03, render/busy/ms 46.76, main/busy/ms 18.38, render/graph/ms 18.32, stream/drive/ms 4.30, pipelines/compile_ms 3.96, pipelines/front_ms 3.49, stream/spawn/ms 2.33, render/present/ms 2.05, terrain/update/ms 0.92, stream/tick/ms 0.79 |
| warm-up | 4.10 | 41.6 | main/busy/ms 70.17, stream/drive/ms 52.00, render/thread/ms 40.35, render/busy/ms 38.64, render/graph/ms 4.38, stream/spawn/ms 2.60, stream/tick/ms 1.78, render/present/ms 1.46, terrain/update/ms 0.80, terrain/select/ms 0.67, bigspace/total/ms 0.61, bigspace/low_precision/ms 0.35 |
| warm-up | 4.20 | 102.1 | pipelines/wait_ms 8901, pipelines/compile_ms 296, pipelines/front_ms 200, pipelines/driver_ms 95.92, render/thread/ms 77.72, render/busy/ms 76.49, main/busy/ms 68.21, stream/drive/ms 50.67, render/graph/ms 3.18, stream/spawn/ms 2.16, terrain/update/ms 1.14, render/present/ms 1.08 |
| warm-up | 4.29 | 71.0 | pipelines/wait_ms 904, main/busy/ms 73.13, stream/drive/ms 50.83, render/thread/ms 38.94, render/busy/ms 37.59, render/graph/ms 8.01, pipelines/compile_ms 6.22, pipelines/front_ms 5.80, stream/spawn/ms 2.08, render/present/ms 1.15, terrain/update/ms 0.79, stream/tick/ms 0.70 |
| warm-up | 4.36 | 46.0 | pipelines/wait_ms 4834, pipelines/compile_ms 156, pipelines/front_ms 151, main/busy/ms 49.43, stream/drive/ms 34.94, render/thread/ms 31.19, render/busy/ms 29.65, pipelines/driver_ms 5.27, render/graph/ms 4.30, stream/spawn/ms 1.52, render/present/ms 1.24, stream/activate/ms 1.05 |
| warm-up | 4.40 | 38.0 | main/busy/ms 41.05, stream/drive/ms 24.90, render/thread/ms 15.50, render/busy/ms 14.45, terrain/update/ms 4.26, render/graph/ms 4.03, stream/activate/ms 1.05, render/present/ms 0.87, stream/tick/ms 0.67, stream/spawn/ms 0.59, terrain/select/ms 0.53, physics/insert/ms 0.46 |
| warm-up | 4.53 | 34.1 | pipelines/wait_ms 8666, pipelines/compile_ms 186, pipelines/front_ms 168, render/thread/ms 41.87, render/busy/ms 40.32, main/busy/ms 23.74, pipelines/driver_ms 18.53, stream/drive/ms 5.25, render/graph/ms 4.92, stream/spawn/ms 1.84, stream/tick/ms 1.79, render/present/ms 1.34 |
| warm-up | 4.64 | 39.7 | pipelines/wait_ms 10.9 k, pipelines/compile_ms 62.22, pipelines/front_ms 56.30, render/thread/ms 43.93, render/busy/ms 42.46, main/busy/ms 34.11, render/graph/ms 6.86, pipelines/driver_ms 5.92, stream/drive/ms 1.93, render/present/ms 1.29, stream/tick/ms 1, physics/insert/ms 0.98 |
| warm-up | 4.69 | 42.9 | render/thread/ms 34.33, render/busy/ms 32.54, main/busy/ms 30.95, render/graph/ms 6.15, stream/drive/ms 2.50, render/present/ms 1.62, stream/tick/ms 1.19, terrain/update/ms 0.97, terrain/select/ms 0.77, bigspace/total/ms 0.69, physics/step/ms 0.38, bigspace/high_precision/ms 0.27 |
| warm-up | 4.73 | 38.7 | pipelines/wait_ms 3774, pipelines/compile_ms 91.33, pipelines/front_ms 87.04, render/thread/ms 52.08, render/busy/ms 50.55, main/busy/ms 44.27, render/graph/ms 5.20, pipelines/driver_ms 4.29, stream/drive/ms 2.45, render/present/ms 1.22, stream/tick/ms 1.17, terrain/update/ms 0.78 |
| warm-up | 4.77 | 38.0 | pipelines/wait_ms 3774, pipelines/compile_ms 91.33, pipelines/front_ms 87.04, render/thread/ms 55.52, render/busy/ms 54.08, main/busy/ms 41.58, render/graph/ms 5.46, pipelines/driver_ms 4.29, stream/drive/ms 1.87, render/present/ms 1.16, stream/tick/ms 0.85, physics/insert/ms 0.73 |
| warm-up | 5.69 | 31.4 | pipelines/wait_ms 1629, pipelines/compile_ms 137, pipelines/front_ms 133, render/thread/ms 42.11, render/busy/ms 40.67, main/busy/ms 35.38, stream/drive/ms 17.96, render/graph/ms 5.25, pipelines/driver_ms 4.28, stream/spawn/ms 1.39, render/present/ms 1.20, stream/tick/ms 0.67 |
| measure | 2.18 | 32.4 | render/thread/ms 42.80, render/busy/ms 40.02, render/graph/ms 20.73, main/busy/ms 16.97, pipelines/compile_ms 13.97, pipelines/front_ms 13.75, render/acquire/ms 1.53, render/present/ms 1.25, bigspace/total/ms 0.77, physics/insert/ms 0.67, terrain/update/ms 0.56, controller/feed/ms 0.56 |
| measure | 59.07 | 33.1 | render/thread/ms 40.89, render/busy/ms 39.82, main/busy/ms 22.94, render/graph/ms 10.56, stream/drive/ms 1.04, render/present/ms 0.91, stream/tick/ms 0.91, terrain/update/ms 0.86, bigspace/total/ms 0.85, terrain/select/ms 0.77, far/select/ms 0.66, controller/feed/ms 0.61 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\tc_pbrwater\balmora_walk@2560x1440_off=normalmaps_orm_parallax_waterssr_shorewaves-1 --bench-baseline none --off normalmaps,orm,parallax,waterssr,shorewaves
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 13%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **95%** (min 70%) · 2569 MHz (min 2565) · memory 8001 MHz · 93.2 W · 82 °C (max 86) · VRAM 5444 MiB · throttled 100% of samples
