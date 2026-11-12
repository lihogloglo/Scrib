# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6489 frames over 65.6 s · **median 10.02 ms (100 fps)** · mean 10.12 · p95 12.07 · p99 13.63 (1% low 73 fps) · worst 35.32 ms · 31 spikes over 1.5× median · 16.5 stutters/min (work 11.9/min)
- warm-up: 337 frames over 4.4 s, worst 580.8 ms
- hitches (> 25.0 ms): **2** measured, 7 in warm-up
- timed GPU passes: 6.32 ms of a 10.12 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.02 ms, limit 11 ms
- **FAIL** frame_ms: 18 stutters (> 1.5x moving median of 60) in 1.09 min = 16.45/min, limit 1/min
- **FAIL** worst frame 35.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.046 | 2.208 | 2.587 | 0.93 | 0.174 |
| main_opaque_pass_3d | 1.366 | 1.474 | 2.107 | 0.93 | 0.049 |
| early prepass | 0.472 | 0.509 | 0.776 | 0.93 | 0.076 |
| bloom | 0.313 | 0.338 | 0.494 | 0.93 | 0.064 |
| atmosphere_luts | 0.301 | 0.326 | 0.507 | 0.92 | 0.003 |
| ssao | 0.200 | 0.216 | 0.214 | 0.93 | 0.007 |
| uniform_allocation | 0.167 | 0.180 | 0.184 | 0.93 | 0.031 |
| auxiliary_views | 0.166 | 0.180 | 0.601 | 0.93 | 0.163 |
| water_fft | 0.123 | 0.132 | 0.129 | 0.93 | 0.003 |
| upscaling | 0.121 | 0.130 | 0.156 | 0.93 | 0.004 |
| clouds_trace | 0.118 | 0.127 | 0.217 | 0.93 | 0.001 |
| bin_unpacking | 0.118 | 0.127 | 0.139 | 0.93 | 0.023 |
| atmos_froxel_inject | 0.090 | 0.097 | 0.108 | 0.93 | 0.002 |
| tonemapping | 0.086 | 0.092 | 0.094 | 0.93 | 0.004 |
| water_velocity | 0.080 | 0.086 | 0.086 | 0.93 | 0.002 |
| clustering | 0.062 | 0.068 | 0.070 | 0.91 | 0.019 |
| clouds_composite | 0.060 | 0.065 | 0.064 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.059 | 0.063 | 0.111 | 0.93 | 0.105 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.93 | 0.002 |
| lightprobe_irradiance_map | 0.042 | 0.436 | 0.751 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.040 | 0.043 | 0.048 | 0.93 | 0.005 |
| surface_trail | 0.037 | 0.059 | 0.058 | 0.62 | 0.002 |
| clouds_resolve | 0.032 | 0.035 | 0.037 | 0.93 | 0.001 |
| clouds_shadow | 0.029 | 0.032 | 0.035 | 0.93 | 0.003 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.030 | 0.93 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.024 | 0.93 | 0.001 |
| lightprobe_radiance_map | 0.022 | 0.226 | 0.234 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.022 | 0.024 | 0.93 | 0.007 |
| clouds_fog_light | 0.012 | 0.013 | 0.014 | 0.93 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.93 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.93 | 0.002 |
| lightprobe_copy | 0.001 | 0.013 | 0.015 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 23.456 | 0.000 | 7619.25 | 152207.1 |
| render/thread/ms | 8.877 | 10.656 | 33.66 | 57600.9 |
| render/busy/ms | 8.265 | 10.088 | 32.84 | 53629.6 |
| main/busy/ms | 8.228 | 9.778 | 15.13 | 53392.2 |
| render/graph/ms | 4.430 | 5.641 | 25.04 | 28746.4 |
| bigspace/total/ms | 0.539 | 1.237 | 2.32 | 3498.9 |
| render/present/ms | 0.509 | 1.169 | 17.24 | 3302.1 |
| pipelines/compile_ms | 0.369 | 0.000 | 152.80 | 2391.5 |
| pipelines/front_ms | 0.351 | 0.000 | 147.49 | 2280.5 |
| terrain/update/ms | 0.295 | 0.377 | 2.06 | 1913.6 |
| controller/feed/ms | 0.269 | 0.359 | 0.88 | 1743.3 |
| terrain/select/ms | 0.262 | 0.335 | 0.85 | 1698.4 |
| stream/drive/ms | 0.232 | 0.326 | 2.16 | 1508.4 |
| bigspace/low_precision/ms | 0.216 | 0.657 | 1.80 | 1399.8 |
| far/select/ms | 0.196 | 0.267 | 0.83 | 1274.4 |
| stream/tick/ms | 0.189 | 0.261 | 0.69 | 1228.2 |
| anim/evaluate_cpu/ms | 0.165 | 0.212 | 0.51 | 1070.2 |
| anim/evaluate/ms | 0.160 | 0.288 | 1.63 | 1039.4 |
| bigspace/high_precision/ms | 0.157 | 0.607 | 1.77 | 1020.9 |
| controller/move/ms | 0.154 | 0.359 | 0.82 | 1000.0 |
| render/acquire/ms | 0.103 | 0.126 | 2.24 | 669.2 |
| physics/step/ms | 0.044 | 0.085 | 0.63 | 286.4 |
| physics/insert/ms | 0.037 | 0.381 | 0.99 | 241.4 |
| weather/map/ms | 0.027 | 0.036 | 0.30 | 174.3 |
| pipelines/driver_ms | 0.017 | 0.000 | 34.36 | 111.0 |
| anim/apply/ms | 0.012 | 0.022 | 0.15 | 76.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 71.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.31 | 69.3 |
| script/update/ms | 0.007 | 0.009 | 0.14 | 45.4 |
| script/input/ms | 0.005 | 0.006 | 0.30 | 32.9 |
| stream/despawn/ms | 0.004 | 0.000 | 0.98 | 29.2 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 24.6 |
| stream/spawn/ms | 0.004 | 0.000 | 1.06 | 24.5 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 21.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 14.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 537 |
| crowd/c0/meshes/count | count | 353 | 646 | 146 |
| crowd/c1/count | count | 145 | 158 | 149 |
| crowd/c1/joints/count | count | 5491 | 6045 | 5659 |
| crowd/c1/meshes/count | count | 1157 | 1313 | 1195 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4259 | 13.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4245 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.51 | 32.84 | 9.03 |
| gpu/timed/ms | ms | 7.47 | 9.97 | 9.03 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 544.17 M | 544.17 M | 544.17 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1287 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3425 | 5130 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1287 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3425 | 5130 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 41 | 235 | 29 |
| phases/shadow/bins | count | 3778 | 5394 | 2252 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9468 | 12.8 k | 5703 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 116 | 248 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2677 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 817 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 173 | 175 | 175 |
| pipelines/waiting | count | 0 | 43 | 0 |
| preprocess/bind_groups | count | 9 | 613 | 6 |
| preprocess/bind_groups_created | count | 9 | 613 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 171 | 261 | 160 |
| process/cpu_usage | % | 26.92 | 42.31 | 22.49 |
| process/mem_usage | GiB | 2.65 | 2.82 | 2.81 |
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
| started_load_count |  | 3477 | 3543 | 3543 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.21 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.84 | 56.13 | 31.27 |
| system/mem_usage | % | 54.48 | 55.10 | 55.07 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 275 | 73 |
| views/all/bins | count | 6398 | 8200 | 5732 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 156 | 286 | 69 |
| views/all/pipelines_distinct | count | 60 | 69 | 55 |
| views/all/work_items | count | 17.3 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 29 |
| views/cascade/bins | count | 2598 | 3187 | 2252 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8166 | 10.2 k | 5703 |
| views/cascade/pipelines | count | 36 | 50 | 29 |
| views/cascade/work_items | count | 8166 | 10.2 k | 5703 |
| views/lamp_face/batch_sets | count | 5 | 196 | 0 |
| views/lamp_face/bins | count | 1088 | 2438 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1231 | 2814 | 0 |
| views/lamp_face/pipelines | count | 76 | 201 | 0 |
| views/lamp_face/work_items | count | 81 | 2578 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2884 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8490 | 13.0 k | 7592 |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 8650 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 56.29 | 277 | 365.3 k |
| anim/events/count | count | 0.25 | 4 | 1621 |
| assets/Image/added | count | 0.05 | 35 | 340 |
| assets/Image/modified | count | 0.00 | 9 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.66 | 160 | 4265 |
| assets/Mesh/removed | count | 0.75 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 502 |
| assets/StandardMaterial/modified | count | 3.34 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.07 | 1494 | 6934 |
| physics/inserts | count | 0.47 | 192 | 3067 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 161 | 2335 |
| physics/steps | count | 0.61 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5217 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 62 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1144 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 9, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.8 | main/busy/ms 558, surface/advance_cpu_ms 67.99, surface/cell_upload_ms 0.69, stream/drive/ms 0.31, weather/map/ms 0.17, bigspace/total/ms 0.10, stream/tick/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.86 | 283.9 | main/busy/ms 590, render/thread/ms 246, render/busy/ms 167, render/acquire/ms 76.63, surface/advance_cpu_ms 67.99, render/graph/ms 5.23, render/present/ms 2.01, stream/drive/ms 0.94, surface/cell_upload_ms 0.69, stream/tick/ms 0.46, terrain/update/ms 0.38, weather/map/ms 0.32 |
| warm-up | 1.00 | 130.4 | pipelines/wait_ms 554, render/thread/ms 257, render/busy/ms 177, pipelines/compile_ms 137, pipelines/front_ms 125, render/acquire/ms 76.91, main/busy/ms 46.92, terrain/update/ms 17.13, pipelines/driver_ms 12.42, render/graph/ms 8.72, render/present/ms 2.92, stream/drive/ms 1.18 |
| warm-up | 1.25 | 93.5 | pipelines/compile_ms 194, pipelines/front_ms 174, main/busy/ms 61.20, render/thread/ms 49.31, render/busy/ms 47.73, pipelines/wait_ms 40.55, pipelines/driver_ms 19.12, render/graph/ms 3.83, render/present/ms 1.36, stream/drive/ms 1.34, stream/tick/ms 1.13, terrain/update/ms 0.94 |
| warm-up | 1.38 | 25.8 | render/thread/ms 27.68, render/busy/ms 26.40, main/busy/ms 15.41, stream/drive/ms 3.74, render/graph/ms 3.69, stream/spawn/ms 1.18, render/present/ms 1.11, stream/tick/ms 0.71, terrain/update/ms 0.70, terrain/select/ms 0.58, bigspace/total/ms 0.23, render/acquire/ms 0.17 |
| warm-up | 1.42 | 26.0 | render/thread/ms 35.34, render/busy/ms 34.18, main/busy/ms 17.10, render/graph/ms 3.42, stream/drive/ms 2.38, render/present/ms 0.99, stream/tick/ms 0.82, terrain/update/ms 0.66, terrain/select/ms 0.55, stream/spawn/ms 0.46, stream/activate/ms 0.35, physics/insert/ms 0.31 |
| warm-up | 2.70 | 26.3 | pipelines/wait_ms 4720, pipelines/compile_ms 72.88, pipelines/front_ms 67.83, main/busy/ms 32.31, render/thread/ms 17.11, render/busy/ms 15.90, pipelines/driver_ms 5.05, render/graph/ms 3.07, stream/drive/ms 1.46, render/present/ms 0.96, stream/tick/ms 0.69, terrain/update/ms 0.66 |
| measure | 0.39 | 35.3 | pipelines/wait_ms 3232, pipelines/compile_ms 128, pipelines/front_ms 125, render/thread/ms 41.12, render/busy/ms 39.56, render/graph/ms 28.71, main/busy/ms 16.33, pipelines/driver_ms 3.67, render/present/ms 1.31, bigspace/total/ms 1.09, physics/insert/ms 0.65, bigspace/high_precision/ms 0.57 |
| measure | 45.00 | 27.8 | render/thread/ms 35.04, render/present/ms 17.65, main/busy/ms 17.23, render/busy/ms 17.18, render/graph/ms 9.42, bigspace/total/ms 1.79, bigspace/high_precision/ms 0.82, controller/feed/ms 0.75, terrain/update/ms 0.65, terrain/select/ms 0.58, stream/drive/ms 0.45, stream/tick/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_base\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **86%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.4 W · 75 °C (max 79) · VRAM 4641 MiB · throttled 100% of samples
