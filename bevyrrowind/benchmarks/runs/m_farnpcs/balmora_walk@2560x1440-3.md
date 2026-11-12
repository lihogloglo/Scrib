# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: farnpcs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6620 frames over 65.6 s · **median 9.82 ms (102 fps)** · mean 9.92 · p95 12.10 · p99 13.53 (1% low 74 fps) · worst 25.30 ms · 26 spikes over 1.5× median · 16.5 stutters/min (work 11.9/min)
- warm-up: 503 frames over 5.8 s, worst 577.3 ms
- hitches (> 25.0 ms): **2** measured, 6 in warm-up
- timed GPU passes: 6.25 ms of a 9.92 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.82 ms, limit 11 ms
- **FAIL** frame_ms: 18 stutters (> 1.5x moving median of 60) in 1.09 min = 16.45/min, limit 1/min
- pass worst frame 25.30 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.056 | 2.228 | 2.821 | 0.92 | 0.176 |
| main_opaque_pass_3d | 1.324 | 1.435 | 2.104 | 0.92 | 0.040 |
| early prepass | 0.434 | 0.471 | 0.752 | 0.92 | 0.069 |
| bloom | 0.317 | 0.344 | 0.502 | 0.92 | 0.064 |
| atmosphere_luts | 0.304 | 0.329 | 0.513 | 0.92 | 0.003 |
| ssao | 0.200 | 0.217 | 0.211 | 0.92 | 0.007 |
| auxiliary_views | 0.172 | 0.187 | 0.684 | 0.92 | 0.159 |
| uniform_allocation | 0.156 | 0.169 | 0.177 | 0.92 | 0.029 |
| water_fft | 0.123 | 0.133 | 0.129 | 0.92 | 0.003 |
| upscaling | 0.121 | 0.131 | 0.162 | 0.92 | 0.004 |
| clouds_trace | 0.118 | 0.128 | 0.219 | 0.92 | 0.001 |
| bin_unpacking | 0.111 | 0.120 | 0.136 | 0.92 | 0.022 |
| atmos_froxel_inject | 0.090 | 0.097 | 0.106 | 0.92 | 0.002 |
| tonemapping | 0.085 | 0.092 | 0.093 | 0.92 | 0.003 |
| water_velocity | 0.082 | 0.089 | 0.086 | 0.92 | 0.001 |
| clustering | 0.066 | 0.071 | 0.070 | 0.92 | 0.019 |
| clouds_composite | 0.060 | 0.065 | 0.063 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.054 | 0.059 | 0.106 | 0.92 | 0.106 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.92 | 0.002 |
| lightprobe_irradiance_map | 0.043 | 0.442 | 0.752 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.041 | 0.046 | 0.92 | 0.005 |
| surface_trail | 0.036 | 0.060 | 0.057 | 0.61 | 0.002 |
| clouds_resolve | 0.032 | 0.035 | 0.038 | 0.92 | 0.001 |
| clouds_shadow | 0.029 | 0.032 | 0.035 | 0.92 | 0.003 |
| lightprobe_radiance_map | 0.024 | 0.239 | 0.237 | 0.10 | 0.004 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.030 | 0.92 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.024 | 0.92 | 0.001 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.024 | 0.92 | 0.006 |
| clouds_fog_light | 0.012 | 0.013 | 0.014 | 0.92 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.92 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.92 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.015 | 0.015 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.684 | 10.760 | 23.86 | 57485.9 |
| main/busy/ms | 8.057 | 9.596 | 16.19 | 53339.8 |
| render/busy/ms | 8.053 | 10.212 | 22.60 | 53310.5 |
| render/graph/ms | 4.343 | 5.786 | 13.73 | 28749.4 |
| pipelines/wait_ms | 2.903 | 0.000 | 3997.16 | 19216.1 |
| bigspace/total/ms | 0.531 | 1.151 | 2.77 | 3518.0 |
| render/present/ms | 0.499 | 1.129 | 14.50 | 3303.8 |
| terrain/update/ms | 0.293 | 0.375 | 1.56 | 1938.9 |
| controller/feed/ms | 0.266 | 0.348 | 0.71 | 1761.0 |
| terrain/select/ms | 0.260 | 0.334 | 0.62 | 1723.7 |
| stream/drive/ms | 0.236 | 0.323 | 2.06 | 1561.0 |
| bigspace/low_precision/ms | 0.205 | 0.645 | 2.52 | 1355.9 |
| anim/evaluate/ms | 0.203 | 0.416 | 1.58 | 1345.7 |
| stream/tick/ms | 0.194 | 0.259 | 0.62 | 1282.5 |
| far/select/ms | 0.188 | 0.250 | 0.56 | 1244.4 |
| bigspace/high_precision/ms | 0.162 | 0.664 | 1.76 | 1070.0 |
| anim/evaluate_cpu/ms | 0.150 | 0.193 | 0.62 | 995.6 |
| render/acquire/ms | 0.132 | 0.295 | 2.65 | 871.5 |
| controller/move/ms | 0.120 | 0.233 | 0.48 | 796.7 |
| pipelines/compile_ms | 0.101 | 0.000 | 163.96 | 667.9 |
| pipelines/front_ms | 0.098 | 0.000 | 157.67 | 646.5 |
| physics/step/ms | 0.041 | 0.079 | 0.76 | 270.7 |
| physics/insert/ms | 0.037 | 0.379 | 0.89 | 244.5 |
| weather/map/ms | 0.026 | 0.035 | 0.31 | 173.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.15 | 75.2 |
| weather/eval/ms | 0.011 | 0.015 | 0.28 | 72.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.16 | 66.8 |
| script/update/ms | 0.007 | 0.010 | 0.17 | 47.3 |
| script/input/ms | 0.005 | 0.006 | 0.24 | 32.5 |
| stream/despawn/ms | 0.004 | 0.000 | 1.07 | 27.7 |
| script/ui/ms | 0.004 | 0.005 | 0.15 | 24.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 24.3 |
| controller/write/ms | 0.003 | 0.004 | 0.13 | 23.1 |
| pipelines/driver_ms | 0.003 | 0.000 | 6.29 | 21.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.16 | 6.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.22 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 4.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 6 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 44 | 19 |
| controller/movers/count | count | 19 | 21 | 19 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1277 | 2218 | 583 |
| crowd/c0/meshes/count | count | 353 | 633 | 162 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5613 |
| crowd/c1/meshes/count | count | 1171 | 1281 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 532 | 569 | 569 |
| crowd/joint_blocks/count | count | 14 | 22 | 8 |
| crowd/joints_extracted/count | count | 728 | 6208 | 402 |
| crowd/joints_uploaded/count | count | 728 | 1090 | 402 |
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
| frame/work/ms | ms | 8.36 | 22.60 | 8.14 |
| gpu/timed/ms | ms | 7.38 | 9.90 | 8.09 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 820 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 820 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 12 | 16 | 10 |
| phases/opaque3d/bins | count | 975 | 1210 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2804 | 4453 | 2175 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 11 | 15 | 9 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 12 | 16 | 10 |
| phases/opaque3d_prepass/bins | count | 975 | 1210 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2804 | 4453 | 2175 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 11 | 15 | 9 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 37 | 242 | 28 |
| phases/shadow/bins | count | 3572 | 5302 | 1911 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8913 | 12.3 k | 4602 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 117 | 256 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2668 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 329 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 10 | 631 | 2 |
| preprocess/bind_groups_created | count | 10 | 631 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 158 | 264 | 142 |
| process/cpu_usage | % | 26.92 | 41.83 | 25.48 |
| process/mem_usage | GiB | 2.73 | 2.92 | 2.92 |
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
| started_load_count |  | 3467 | 3536 | 3536 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.22 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.15 | 46.01 | 37.29 |
| system/mem_usage | % | 54.28 | 54.93 | 54.93 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 76 | 284 | 62 |
| views/all/bins | count | 5618 | 7445 | 5009 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.6 k | 23.2 k | 10.2 k |
| views/all/pipelines | count | 153 | 291 | 58 |
| views/all/pipelines_distinct | count | 59 | 68 | 44 |
| views/all/work_items | count | 15.6 k | 21.9 k | 10.5 k |
| views/cascade/batch_sets | count | 35 | 50 | 28 |
| views/cascade/bins | count | 2307 | 2895 | 1911 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7449 | 9594 | 4602 |
| views/cascade/pipelines | count | 35 | 50 | 28 |
| views/cascade/work_items | count | 7449 | 9594 | 4602 |
| views/lamp_face/batch_sets | count | 0 | 203 | 0 |
| views/lamp_face/bins | count | 1082 | 2609 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1226 | 2985 | 0 |
| views/lamp_face/pipelines | count | 76 | 208 | 0 |
| views/lamp_face/work_items | count | 0 | 2707 | 0 |
| views/main/batch_sets | count | 38 | 48 | 34 |
| views/main/bins | count | 2298 | 3214 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7256 | 11.6 k | 5584 |
| views/main/pipelines | count | 36 | 44 | 30 |
| views/main/work_items | count | 7416 | 11.9 k | 5892 |
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
| anim/bones_written/count | count | 52.71 | 302 | 348.9 k |
| anim/events/count | count | 0.22 | 3 | 1470 |
| assets/Image/added | count | 0.05 | 35 | 325 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.62 | 160 | 4109 |
| assets/Mesh/removed | count | 0.74 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 483 |
| assets/StandardMaterial/modified | count | 3.27 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.85 | 1568 | 5628 |
| physics/inserts | count | 0.39 | 111 | 2579 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 145 | 2335 |
| physics/steps | count | 0.60 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 6 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5114 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1121 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.3 | main/busy/ms 558, surface/advance_cpu_ms 68.17, surface/cell_upload_ms 0.74, stream/drive/ms 0.34, weather/map/ms 0.16, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 1.07 | 489.0 | main/busy/ms 593, render/thread/ms 452, render/busy/ms 363, render/acquire/ms 86.76, surface/advance_cpu_ms 68.17, render/graph/ms 4.91, render/present/ms 2.09, stream/drive/ms 1.04, surface/cell_upload_ms 0.74, stream/tick/ms 0.52, terrain/update/ms 0.49, terrain/select/ms 0.35 |
| warm-up | 1.31 | 101.2 | pipelines/compile_ms 194, pipelines/front_ms 179, render/thread/ms 62.25, render/busy/ms 59.95, main/busy/ms 56.69, pipelines/wait_ms 39.27, pipelines/driver_ms 14.59, render/graph/ms 4.14, render/present/ms 2.10, stream/drive/ms 1.83, stream/tick/ms 1.33, terrain/update/ms 0.88 |
| warm-up | 1.35 | 26.5 | render/thread/ms 35.78, render/busy/ms 34.48, main/busy/ms 18.05, stream/drive/ms 2.78, render/graph/ms 2.57, stream/tick/ms 1.85, render/present/ms 1.13, terrain/update/ms 0.83, terrain/select/ms 0.69, stream/spawn/ms 0.54, bigspace/total/ms 0.30, render/acquire/ms 0.16 |
| warm-up | 1.49 | 28.0 | pipelines/wait_ms 287, pipelines/compile_ms 91.52, pipelines/driver_ms 75.63, render/thread/ms 39.01, render/busy/ms 37.77, main/busy/ms 20.48, pipelines/front_ms 15.89, stream/drive/ms 4.78, stream/spawn/ms 2.20, render/graph/ms 2.12, render/present/ms 1.06, stream/tick/ms 1.01 |
| warm-up | 2.05 | 43.2 | render/thread/ms 45.53, render/busy/ms 44.39, main/busy/ms 27.12, render/graph/ms 3.21, stream/drive/ms 2.19, render/present/ms 0.93, stream/tick/ms 0.84, terrain/update/ms 0.69, terrain/select/ms 0.59, stream/spawn/ms 0.56, bigspace/total/ms 0.52, bigspace/high_precision/ms 0.30 |
| measure | 19.26 | 25.3 | render/thread/ms 31.41, render/busy/ms 30.34, main/busy/ms 18.63, render/graph/ms 17.03, bigspace/total/ms 1.68, bigspace/high_precision/ms 1.08, physics/insert/ms 0.90, render/present/ms 0.89, terrain/update/ms 0.62, controller/feed/ms 0.59, terrain/select/ms 0.57, stream/drive/ms 0.51 |
| measure | 38.16 | 25.0 | render/thread/ms 32.68, render/busy/ms 17.49, main/busy/ms 17.19, render/present/ms 14.98, render/graph/ms 9.76, bigspace/total/ms 0.87, terrain/update/ms 0.72, terrain/select/ms 0.65, stream/drive/ms 0.58, controller/feed/ms 0.56, stream/tick/ms 0.49, anim/evaluate/ms 0.40 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_farnpcs\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=farnpcs
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **87%** (min 59%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.7 W · 73 °C (max 80) · VRAM 4137 MiB · throttled 100% of samples
