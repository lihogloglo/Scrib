# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4213 frames over 65.7 s · **median 11.39 ms (88 fps)** · mean 15.58 · p95 45.41 · p99 86.86 (1% low 12 fps) · worst 129.22 ms · 388 spikes over 1.5× median · 23.8 stutters/min (work 28.3/min)
- warm-up: 332 frames over 13.5 s, worst 2635.5 ms
- hitches (> 28.5 ms): **351** measured, 166 in warm-up
- timed GPU passes: 10.33 ms of a 15.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.39 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- **FAIL** worst frame 129.22 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.235 | 2.363 | 2.991 | 0.95 | 0.181 |
| main_opaque_pass_3d | 1.912 | 2.021 | 4.607 | 0.95 | 0.061 |
| bin_unpacking | 0.902 | 0.953 | 7.188 | 0.95 | 0.027 |
| early prepass | 0.835 | 0.882 | 3.477 | 0.95 | 0.093 |
| auxiliary_views | 0.676 | 0.715 | 4.756 | 0.95 | 0.136 |
| uniform_allocation | 0.520 | 0.550 | 2.626 | 0.95 | 0.046 |
| bloom | 0.349 | 0.369 | 0.784 | 0.95 | 0.066 |
| atmosphere_luts | 0.339 | 0.358 | 0.718 | 0.95 | 0.003 |
| water_velocity | 0.293 | 0.310 | 0.744 | 0.95 | 0.002 |
| early_mesh_preprocessing | 0.274 | 0.289 | 2.503 | 0.95 | 0.006 |
| water_sspr | 0.238 | 0.251 | 0.560 | 0.95 | 0.002 |
| ssao | 0.210 | 0.222 | 0.535 | 0.95 | 0.008 |
| atmos_froxel_inject | 0.161 | 0.170 | 0.156 | 0.95 | 0.002 |
| clouds_trace | 0.147 | 0.155 | 0.449 | 0.95 | 0.001 |
| surface_trail | 0.138 | 0.188 | 1.369 | 0.73 | 0.002 |
| water_fft | 0.133 | 0.141 | 0.137 | 0.95 | 0.003 |
| upscaling | 0.124 | 0.132 | 0.160 | 0.95 | 0.004 |
| clouds_composite | 0.106 | 0.112 | 0.112 | 0.95 | 0.002 |
| water_snapshot | 0.101 | 0.107 | 0.562 | 0.95 | 0.002 |
| clustering | 0.097 | 0.103 | 0.270 | 0.95 | 0.021 |
| main_transmissive_pass_3d | 0.097 | 0.103 | 0.314 | 0.95 | 0.008 |
| tonemapping | 0.089 | 0.094 | 0.097 | 0.95 | 0.004 |
| render_sky | 0.053 | 0.056 | 0.055 | 0.95 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.475 | 0.985 | 0.10 | 0.001 |
| clouds_resolve | 0.047 | 0.050 | 0.052 | 0.95 | 0.001 |
| clouds_shadow | 0.036 | 0.038 | 0.109 | 0.95 | 0.003 |
| per_view_shadow_pass | 0.028 | 0.030 | 0.052 | 0.95 | 0.007 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.032 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.259 | 0.563 | 0.10 | 0.004 |
| water_mips | 0.027 | 0.028 | 0.025 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.020 | 0.021 | 0.034 | 0.95 | 0.147 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.95 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.009 | 0.95 | 0.003 |
| night_sky | 0.005 | 0.006 | 0.007 | 0.95 | 0.001 |
| lightprobe_copy | 0.004 | 0.038 | 0.071 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.018 | 0.033 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 14.294 | 43.912 | 127.13 | 60222.4 |
| render/busy/ms | 10.098 | 13.832 | 70.59 | 42541.7 |
| main/busy/ms | 8.392 | 10.333 | 28.89 | 35353.6 |
| render/graph/ms | 5.470 | 7.667 | 22.05 | 23044.6 |
| render/acquire/ms | 3.643 | 29.263 | 106.47 | 15348.9 |
| render/present/ms | 0.553 | 1.382 | 6.47 | 2331.8 |
| bigspace/total/ms | 0.439 | 0.655 | 2.32 | 1850.7 |
| controller/feed/ms | 0.303 | 0.393 | 0.75 | 1275.7 |
| terrain/update/ms | 0.300 | 0.382 | 1.87 | 1264.8 |
| terrain/select/ms | 0.266 | 0.339 | 0.88 | 1120.2 |
| stream/drive/ms | 0.257 | 0.379 | 19.89 | 1081.1 |
| far/select/ms | 0.202 | 0.284 | 0.57 | 850.9 |
| stream/tick/ms | 0.191 | 0.264 | 0.62 | 805.9 |
| anim/evaluate_cpu/ms | 0.175 | 0.225 | 0.77 | 738.2 |
| bigspace/low_precision/ms | 0.167 | 0.273 | 2.02 | 703.1 |
| controller/move/ms | 0.132 | 0.235 | 0.81 | 557.8 |
| pipelines/compile_ms | 0.125 | 0.000 | 133.17 | 526.4 |
| anim/evaluate/ms | 0.122 | 0.246 | 2.52 | 515.1 |
| pipelines/front_ms | 0.121 | 0.000 | 128.59 | 509.2 |
| bigspace/high_precision/ms | 0.105 | 0.203 | 1.40 | 442.7 |
| physics/insert/ms | 0.058 | 0.461 | 1.51 | 244.2 |
| physics/step/ms | 0.058 | 0.123 | 0.66 | 243.4 |
| pipelines/wait_ms | 0.032 | 0.000 | 120.45 | 136.3 |
| weather/map/ms | 0.024 | 0.034 | 0.66 | 102.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 46.4 |
| anim/apply/ms | 0.011 | 0.021 | 0.17 | 44.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.19 | 44.6 |
| script/update/ms | 0.007 | 0.010 | 0.07 | 30.6 |
| stream/despawn/ms | 0.007 | 0.000 | 1.28 | 29.9 |
| stream/spawn/ms | 0.006 | 0.000 | 1.10 | 26.2 |
| script/input/ms | 0.006 | 0.008 | 0.08 | 23.4 |
| script/ui/ms | 0.005 | 0.007 | 0.08 | 22.4 |
| pipelines/driver_ms | 0.004 | 0.000 | 4.58 | 17.3 |
| controller/write/ms | 0.004 | 0.005 | 0.12 | 15.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 12.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.61 | 6.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.22 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.08 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 18 | 42 | 13 |
| crowd/c0/joints/count | count | 959 | 2218 | 583 |
| crowd/c0/meshes/count | count | 303 | 633 | 162 |
| crowd/c1/count | count | 149 | 156 | 148 |
| crowd/c1/joints/count | count | 5637 | 5983 | 5613 |
| crowd/c1/meshes/count | count | 1187 | 1297 | 1179 |
| crowd/c2/count | count | 401 | 417 | 417 |
| crowd/c2/joints/count | count | 6438 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1836 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 194 | 236 | 223 |
| crowd/joints_extracted/count | count | 4366 | 9861 | 6179 |
| crowd/joints_uploaded/count | count | 4323 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.56 | 99.10 | 10.48 |
| gpu/timed/ms | ms | 8.47 | 99.10 | 9.19 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.1 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 368 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 4258 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 368 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 4258 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 24 | 23 |
| phases/opaque3d/bins | count | 4170 | 4327 | 4326 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.2 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 23 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 4170 | 4327 | 4326 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.2 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 22 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 51 | 292 | 42 |
| phases/shadow/bins | count | 12.0 k | 15.6 k | 11.9 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 39.0 k | 42.3 k | 39.3 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 51 | 311 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2607 | 3268 | 2348 |
| physics/items | count | 91 | 476 | 19 |
| physics/pending | count | 0 | 424 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 200 | 203 | 203 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 11 | 757 | 6 |
| preprocess/bind_groups_created | count | 11 | 757 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 228 | 376 | 221 |
| process/cpu_usage | % | 22.12 | 38.46 | 28.12 |
| process/mem_usage | GiB | 6.35 | 6.62 | 6.61 |
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
| started_load_count |  | 4800 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 420 | 528 | 396 |
| statics/materials_bound | count | 420 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.03 | 0.32 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 320 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.52 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1125 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.36 | 47.91 | 35.39 |
| system/mem_usage | % | 67.55 | 80.73 | 68.08 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2832 | 2938 | 2938 |
| textures/images | count | 2600 | 2649 | 2649 |
| views/all/batch_sets | count | 111 | 346 | 102 |
| views/all/bins | count | 21.3 k | 24.2 k | 21.5 k |
| views/all/count | count | 4 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 68.0 k | 70.7 k | 68.9 k |
| views/all/pipelines | count | 107 | 369 | 98 |
| views/all/pipelines_distinct | count | 79 | 93 | 77 |
| views/all/work_items | count | 67.8 k | 70.2 k | 69.6 k |
| views/cascade/batch_sets | count | 51 | 65 | 42 |
| views/cascade/bins | count | 11.9 k | 12.4 k | 11.9 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.6 k | 39.8 k | 39.3 k |
| views/cascade/pipelines | count | 51 | 65 | 42 |
| views/cascade/work_items | count | 38.6 k | 39.8 k | 39.3 k |
| views/lamp_face/batch_sets | count | 0 | 235 | 0 |
| views/lamp_face/bins | count | 0 | 3848 | 0 |
| views/lamp_face/count | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 4212 | 0 |
| views/lamp_face/pipelines | count | 0 | 250 | 0 |
| views/lamp_face/work_items | count | 0 | 3840 | 0 |
| views/main/batch_sets | count | 60 | 64 | 60 |
| views/main/bins | count | 8840 | 9594 | 9594 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.5 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 56 | 60 | 56 |
| views/main/work_items | count | 29.0 k | 30.4 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.0 k | 37.8 k | 30.7 k |
| world/point_lights | count | 289 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 36.44 | 263 | 153.5 k |
| anim/events/count | count | 0.12 | 4 | 514 |
| assets/Image/added | count | 0.14 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.97 | 168 | 4075 |
| assets/Mesh/removed | count | 1.16 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.47 | 22 | 18.8 k |
| assets/StandardMaterial/removed | count | 0.10 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 6 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.12 | 1485 | 4718 |
| physics/inserts | count | 0.55 | 111 | 2324 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.52 | 145 | 2181 |
| physics/steps | count | 0.87 | 3 | 3663 |
| physics/steps_dropped | count | 0.07 | 5 | 278 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 11 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 3 | 83 |
| stream/spawn/items | count | 0.09 | 18 | 378 |
| stream/upload/bytes | bytes | 8035 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 43 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1762 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2635.5 | main/busy/ms 2609, surface/advance_cpu_ms 69.36, surface/cell_upload_ms 0.70, stream/drive/ms 0.37, bigspace/total/ms 0.11, stream/tick/ms 0.09, weather/map/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.47 | 830.6 | main/busy/ms 2646, render/thread/ms 794, render/busy/ms 623, render/acquire/ms 122, surface/advance_cpu_ms 69.36, render/present/ms 49.19, render/graph/ms 12.41, stream/drive/ms 1.05, surface/cell_upload_ms 0.70, stream/tick/ms 0.51, terrain/update/ms 0.50, bigspace/total/ms 0.32 |
| warm-up | 3.78 | 132.7 | pipelines/compile_ms 286, pipelines/front_ms 271, pipelines/wait_ms 212, render/thread/ms 86.85, main/busy/ms 68.47, render/busy/ms 56.61, render/present/ms 30.02, pipelines/driver_ms 15.15, render/graph/ms 3.41, stream/drive/ms 1.89, stream/tick/ms 1.46, terrain/update/ms 0.90 |
| warm-up | 4.03 | 89.4 | render/thread/ms 60.11, render/busy/ms 58.09, main/busy/ms 47.31, stream/drive/ms 29.76, render/graph/ms 6.93, terrain/update/ms 6.55, stream/spawn/ms 1.81, stream/tick/ms 1.76, render/present/ms 1.73, terrain/select/ms 0.73, bigspace/total/ms 0.36, render/acquire/ms 0.30 |
| warm-up | 4.52 | 143.8 | pipelines/wait_ms 4107, pipelines/compile_ms 164, pipelines/front_ms 154, main/busy/ms 99.30, render/thread/ms 98.35, render/busy/ms 94.42, stream/drive/ms 79.43, render/graph/ms 13.71, pipelines/driver_ms 10.05, render/present/ms 3.27, stream/tick/ms 2.98, stream/spawn/ms 1.74 |
| measure | 2.40 | 115.2 | render/thread/ms 133, render/acquire/ms 99.90, render/busy/ms 32.00, main/busy/ms 18.60, render/graph/ms 17.76, render/present/ms 1.32, bigspace/total/ms 1.04, controller/feed/ms 0.70, terrain/update/ms 0.63, terrain/select/ms 0.55, physics/insert/ms 0.54, stream/drive/ms 0.52 |
| measure | 2.85 | 90.6 | render/thread/ms 148, render/acquire/ms 100, render/busy/ms 46.09, main/busy/ms 29.63, render/graph/ms 26.61, stream/drive/ms 4.14, render/present/ms 1.42, terrain/update/ms 0.83, bigspace/total/ms 0.81, terrain/select/ms 0.73, physics/insert/ms 0.67, controller/feed/ms 0.61 |
| measure | 3.03 | 129.2 | render/thread/ms 177, render/acquire/ms 134, render/busy/ms 41.44, main/busy/ms 29.17, render/graph/ms 22.95, render/present/ms 1.52, stream/drive/ms 1.31, physics/insert/ms 0.89, controller/feed/ms 0.88, bigspace/total/ms 0.86, far/select/ms 0.70, terrain/update/ms 0.60 |
| measure | 4.06 | 90.8 | render/thread/ms 161, render/acquire/ms 124, render/busy/ms 35.57, main/busy/ms 21.86, render/graph/ms 20.68, bigspace/total/ms 1.51, render/present/ms 1.46, physics/insert/ms 1.11, terrain/update/ms 0.71, stream/drive/ms 0.69, terrain/select/ms 0.65, controller/feed/ms 0.52 |
| measure | 5.22 | 83.8 | render/thread/ms 145, render/acquire/ms 115, render/busy/ms 28.95, render/graph/ms 18.33, main/busy/ms 18.17, render/present/ms 1.04, physics/insert/ms 0.94, bigspace/total/ms 0.84, terrain/update/ms 0.61, terrain/select/ms 0.55, controller/feed/ms 0.55, stream/drive/ms 0.51 |
| measure | 5.83 | 83.3 | render/thread/ms 154, render/acquire/ms 122, render/busy/ms 30.38, main/busy/ms 18.72, render/graph/ms 16.90, render/present/ms 1.83, bigspace/total/ms 1.31, bigspace/high_precision/ms 0.63, terrain/update/ms 0.60, stream/drive/ms 0.59, far/select/ms 0.56, controller/feed/ms 0.56 |
| measure | 7.54 | 113.4 | render/thread/ms 185, render/acquire/ms 152, render/busy/ms 32.17, main/busy/ms 27.27, render/graph/ms 15.30, stream/drive/ms 2.53, physics/insert/ms 1.30, render/present/ms 1.15, bigspace/total/ms 0.94, stream/spawn/ms 0.78, terrain/update/ms 0.65, stream/tick/ms 0.65 |
| measure | 7.71 | 84.1 | render/thread/ms 160, render/acquire/ms 129, render/busy/ms 29.61, main/busy/ms 20.32, render/graph/ms 16.42, stream/drive/ms 1.10, render/present/ms 1.06, physics/insert/ms 1.03, bigspace/total/ms 0.80, terrain/update/ms 0.68, controller/feed/ms 0.66, terrain/select/ms 0.62 |
| measure | 7.79 | 86.3 | render/thread/ms 168, render/acquire/ms 140, render/busy/ms 26.43, main/busy/ms 21.19, render/graph/ms 15.86, stream/drive/ms 1.41, bigspace/total/ms 1.13, render/present/ms 1.08, physics/insert/ms 0.98, stream/despawn/ms 0.86, controller/feed/ms 0.71, terrain/update/ms 0.67 |
| measure | 8.35 | 88.7 | render/thread/ms 164, render/acquire/ms 133, render/busy/ms 29.87, main/busy/ms 19.30, render/graph/ms 17.36, render/present/ms 1.06, bigspace/total/ms 0.87, physics/insert/ms 0.84, stream/drive/ms 0.67, controller/feed/ms 0.65, terrain/update/ms 0.61, terrain/select/ms 0.56 |
| measure | 8.44 | 88.1 | render/thread/ms 174, render/acquire/ms 145, render/busy/ms 28.76, main/busy/ms 18.50, render/graph/ms 15.97, render/present/ms 1.05, bigspace/total/ms 1.00, physics/insert/ms 0.88, terrain/update/ms 0.65, controller/feed/ms 0.63, terrain/select/ms 0.58, anim/evaluate_cpu/ms 0.45 |
| measure | 8.68 | 84.6 | render/thread/ms 163, render/acquire/ms 136, render/busy/ms 26.29, main/busy/ms 18.08, render/graph/ms 14.70, render/present/ms 1.14, bigspace/total/ms 0.84, terrain/update/ms 0.76, terrain/select/ms 0.68, physics/insert/ms 0.63, controller/feed/ms 0.56, stream/drive/ms 0.48 |
| measure | 8.77 | 88.0 | render/thread/ms 170, render/acquire/ms 144, render/busy/ms 24.67, main/busy/ms 18.08, render/graph/ms 13.32, render/present/ms 1.17, physics/insert/ms 0.82, bigspace/total/ms 0.79, terrain/update/ms 0.67, terrain/select/ms 0.59, controller/feed/ms 0.57, stream/drive/ms 0.51 |
| measure | 8.86 | 87.2 | render/thread/ms 173, render/acquire/ms 147, render/busy/ms 24.94, main/busy/ms 20.40, render/graph/ms 14.20, physics/insert/ms 1.16, render/present/ms 1.09, bigspace/total/ms 0.99, controller/feed/ms 0.69, terrain/update/ms 0.60, terrain/select/ms 0.55, anim/evaluate_cpu/ms 0.53 |
| measure | 9.09 | 83.9 | render/thread/ms 154, render/acquire/ms 127, render/busy/ms 26.39, main/busy/ms 19.86, render/graph/ms 12.68, render/present/ms 1.04, bigspace/total/ms 0.79, stream/drive/ms 0.68, physics/insert/ms 0.65, controller/feed/ms 0.59, terrain/update/ms 0.59, terrain/select/ms 0.53 |
| measure | 9.67 | 86.9 | render/thread/ms 166, render/acquire/ms 137, render/busy/ms 28.37, main/busy/ms 21.71, render/graph/ms 15.01, stream/drive/ms 1.94, stream/despawn/ms 1.04, render/present/ms 0.89, bigspace/total/ms 0.84, terrain/update/ms 0.76, controller/feed/ms 0.58, terrain/select/ms 0.56 |
| measure | 9.76 | 87.4 | render/thread/ms 172, render/acquire/ms 141, render/busy/ms 29.31, main/busy/ms 23.21, render/graph/ms 16.05, stream/drive/ms 2.46, stream/despawn/ms 1.16, render/present/ms 1.01, bigspace/total/ms 0.85, physics/insert/ms 0.73, controller/feed/ms 0.65, terrain/update/ms 0.64 |
| measure | 9.84 | 86.2 | render/thread/ms 171, render/acquire/ms 135, render/busy/ms 34.43, main/busy/ms 26.27, render/graph/ms 17.90, stream/drive/ms 2.27, render/present/ms 1.16, physics/insert/ms 1.09, stream/despawn/ms 0.96, bigspace/total/ms 0.95, controller/feed/ms 0.80, stream/tick/ms 0.71 |
| measure | 10.01 | 88.5 | render/thread/ms 163, render/acquire/ms 131, render/busy/ms 30.59, main/busy/ms 20.23, render/graph/ms 15.27, render/present/ms 1.08, bigspace/total/ms 0.82, stream/drive/ms 0.70, controller/feed/ms 0.64, terrain/update/ms 0.63, terrain/select/ms 0.57, physics/insert/ms 0.45 |
| measure | 10.09 | 84.0 | render/thread/ms 170, render/acquire/ms 139, render/busy/ms 29.58, main/busy/ms 19.55, render/graph/ms 15.23, render/present/ms 1.05, bigspace/total/ms 0.82, controller/feed/ms 0.71, stream/drive/ms 0.68, physics/insert/ms 0.66, terrain/update/ms 0.56, terrain/select/ms 0.51 |
| measure | 10.19 | 95.4 | render/thread/ms 177, render/acquire/ms 148, render/busy/ms 27.76, main/busy/ms 20.25, render/graph/ms 14.71, render/present/ms 0.97, physics/insert/ms 0.97, stream/drive/ms 0.87, bigspace/total/ms 0.82, terrain/update/ms 0.63, controller/feed/ms 0.57, terrain/select/ms 0.56 |
| measure | 10.27 | 86.0 | render/thread/ms 179, render/acquire/ms 148, render/busy/ms 29.68, main/busy/ms 20.81, render/graph/ms 16.17, stream/drive/ms 1.43, render/present/ms 0.99, physics/insert/ms 0.97, bigspace/total/ms 0.74, terrain/update/ms 0.64, terrain/select/ms 0.58, stream/deactivate/ms 0.52 |
| measure | 10.36 | 83.9 | render/thread/ms 167, render/acquire/ms 136, render/busy/ms 30.60, main/busy/ms 26.48, render/graph/ms 15.48, stream/drive/ms 2.02, physics/insert/ms 1.45, bigspace/total/ms 1.38, render/present/ms 1.09, controller/feed/ms 0.81, stream/despawn/ms 0.72, far/select/ms 0.72 |
| measure | 10.44 | 83.7 | render/thread/ms 161, render/acquire/ms 127, render/busy/ms 32.87, main/busy/ms 30.28, render/graph/ms 13.47, stream/drive/ms 2.21, bigspace/total/ms 1.41, stream/despawn/ms 1.37, physics/insert/ms 1.26, render/present/ms 1.12, controller/feed/ms 0.95, terrain/update/ms 0.79 |
| measure | 10.53 | 88.6 | render/thread/ms 165, render/acquire/ms 132, render/busy/ms 31.99, main/busy/ms 24.20, render/graph/ms 13.44, stream/drive/ms 1.84, terrain/update/ms 1.78, stream/despawn/ms 1.15, render/present/ms 1.04, terrain/select/ms 0.75, bigspace/total/ms 0.74, controller/feed/ms 0.65 |
| measure | 10.62 | 88.2 | render/thread/ms 174, render/acquire/ms 145, render/busy/ms 27.54, main/busy/ms 20.39, render/graph/ms 14.68, terrain/update/ms 1.55, stream/drive/ms 1.18, render/present/ms 1.00, bigspace/total/ms 0.81, controller/feed/ms 0.63, stream/despawn/ms 0.59, terrain/select/ms 0.55 |
| measure | 10.70 | 84.8 | render/thread/ms 170, render/acquire/ms 144, render/busy/ms 25.72, main/busy/ms 21.70, render/graph/ms 14.22, stream/drive/ms 1.38, render/present/ms 1.01, bigspace/total/ms 0.86, terrain/update/ms 0.76, controller/feed/ms 0.73, stream/spawn/ms 0.71, terrain/select/ms 0.68 |
| measure | 10.79 | 83.6 | render/thread/ms 166, render/acquire/ms 139, render/busy/ms 26.14, main/busy/ms 22.46, render/graph/ms 13.27, stream/drive/ms 1.28, render/present/ms 1.07, physics/insert/ms 0.83, bigspace/total/ms 0.80, terrain/update/ms 0.78, stream/spawn/ms 0.71, terrain/select/ms 0.70 |
| measure | 10.95 | 84.0 | render/thread/ms 164, render/acquire/ms 136, render/busy/ms 27.21, main/busy/ms 19.15, render/graph/ms 14.31, render/present/ms 1.08, bigspace/total/ms 0.89, stream/drive/ms 0.68, controller/feed/ms 0.68, terrain/update/ms 0.66, terrain/select/ms 0.61, physics/insert/ms 0.52 |
| measure | 11.05 | 95.3 | render/thread/ms 177, render/acquire/ms 149, render/busy/ms 26.58, main/busy/ms 18.38, render/graph/ms 15.30, render/present/ms 1.02, bigspace/total/ms 0.82, terrain/update/ms 0.63, terrain/select/ms 0.57, controller/feed/ms 0.56, stream/drive/ms 0.46, physics/insert/ms 0.42 |
| measure | 11.13 | 83.4 | render/thread/ms 176, render/acquire/ms 148, render/busy/ms 27.36, main/busy/ms 18.76, render/graph/ms 15.52, render/present/ms 1.04, bigspace/total/ms 0.80, physics/insert/ms 0.62, terrain/update/ms 0.56, controller/feed/ms 0.52, terrain/select/ms 0.51, stream/drive/ms 0.48 |
| measure | 11.22 | 91.9 | render/thread/ms 173, render/acquire/ms 146, render/busy/ms 26.06, main/busy/ms 17.89, render/graph/ms 14.62, render/present/ms 1.07, bigspace/total/ms 0.79, physics/insert/ms 0.76, terrain/update/ms 0.63, terrain/select/ms 0.57, controller/feed/ms 0.55, stream/drive/ms 0.53 |
| measure | 11.32 | 93.1 | render/thread/ms 182, render/acquire/ms 153, render/busy/ms 28.65, main/busy/ms 17.52, render/graph/ms 17.32, render/present/ms 1.10, physics/insert/ms 0.87, bigspace/total/ms 0.81, terrain/update/ms 0.64, terrain/select/ms 0.58, controller/feed/ms 0.58, stream/drive/ms 0.49 |
| measure | 11.41 | 96.2 | render/thread/ms 186, render/acquire/ms 158, render/busy/ms 27.37, main/busy/ms 25.57, render/graph/ms 16.03, stream/drive/ms 1.15, physics/insert/ms 1.13, render/present/ms 1.12, bigspace/total/ms 1.08, controller/feed/ms 0.86, terrain/update/ms 0.60, anim/evaluate_cpu/ms 0.57 |
| measure | 11.51 | 91.4 | render/thread/ms 185, render/acquire/ms 158, render/busy/ms 25.40, main/busy/ms 24.98, render/graph/ms 13.07, physics/insert/ms 1.15, stream/drive/ms 1.15, render/present/ms 1.12, bigspace/total/ms 1.02, controller/feed/ms 0.82, terrain/update/ms 0.73, terrain/select/ms 0.66 |
| measure | 11.67 | 87.1 | render/thread/ms 167, render/acquire/ms 139, render/busy/ms 27.01, main/busy/ms 18.82, render/graph/ms 15.47, render/present/ms 1.05, bigspace/total/ms 0.79, controller/feed/ms 0.66, terrain/update/ms 0.58, physics/insert/ms 0.56, stream/drive/ms 0.53, terrain/select/ms 0.52 |
| measure | 11.76 | 86.5 | render/thread/ms 171, render/acquire/ms 141, render/busy/ms 29.48, main/busy/ms 19.77, render/graph/ms 16.64, render/present/ms 0.98, bigspace/total/ms 0.85, terrain/update/ms 0.79, terrain/select/ms 0.71, controller/feed/ms 0.65, physics/insert/ms 0.55, stream/drive/ms 0.55 |
| measure | 11.85 | 87.1 | render/thread/ms 171, render/acquire/ms 142, render/busy/ms 27.81, main/busy/ms 18.07, render/graph/ms 15.18, render/present/ms 1.05, terrain/update/ms 0.79, bigspace/total/ms 0.75, terrain/select/ms 0.72, controller/feed/ms 0.59, stream/drive/ms 0.48, stream/tick/ms 0.43 |
| measure | 11.95 | 99.9 | render/thread/ms 185, render/acquire/ms 156, render/busy/ms 27.80, main/busy/ms 18.54, render/graph/ms 15.89, render/present/ms 1.10, terrain/update/ms 0.94, terrain/select/ms 0.88, bigspace/total/ms 0.76, controller/feed/ms 0.69, stream/drive/ms 0.41, anim/evaluate_cpu/ms 0.37 |
| measure | 12.04 | 91.8 | render/thread/ms 189, render/acquire/ms 158, render/busy/ms 30.09, main/busy/ms 20.66, render/graph/ms 17.03, terrain/update/ms 1.15, render/present/ms 1.10, terrain/select/ms 1.08, anim/evaluate_cpu/ms 0.93, bigspace/total/ms 0.80, anim/evaluate/ms 0.74, controller/feed/ms 0.68 |
| measure | 12.15 | 105.9 | render/thread/ms 195, render/acquire/ms 164, render/busy/ms 30.13, main/busy/ms 20.27, render/graph/ms 16.52, pipelines/compile_ms 8.11, pipelines/front_ms 7.91, render/present/ms 1.23, anim/evaluate_cpu/ms 0.95, terrain/update/ms 0.82, bigspace/total/ms 0.78, terrain/select/ms 0.76 |
| measure | 12.24 | 98.7 | render/thread/ms 202, render/acquire/ms 170, render/busy/ms 30.95, main/busy/ms 20.89, render/graph/ms 17.55, pipelines/compile_ms 8.11, pipelines/front_ms 7.91, render/present/ms 1.25, bigspace/total/ms 0.91, controller/feed/ms 0.65, terrain/update/ms 0.63, terrain/select/ms 0.57 |
| measure | 12.34 | 91.0 | render/thread/ms 187, render/acquire/ms 157, render/busy/ms 28.62, main/busy/ms 20.61, render/graph/ms 16.14, terrain/update/ms 2.19, render/present/ms 1.13, bigspace/total/ms 0.90, physics/insert/ms 0.65, terrain/select/ms 0.63, controller/feed/ms 0.58, stream/drive/ms 0.47 |
| measure | 12.43 | 99.2 | render/thread/ms 188, render/acquire/ms 161, render/busy/ms 25.27, main/busy/ms 18.00, render/graph/ms 14.34, terrain/update/ms 2.22, render/present/ms 1.04, physics/insert/ms 0.94, bigspace/total/ms 0.80, terrain/select/ms 0.65, controller/feed/ms 0.59, stream/drive/ms 0.48 |
| measure | 12.53 | 94.6 | render/thread/ms 191, render/acquire/ms 161, render/busy/ms 29.16, main/busy/ms 20.76, render/graph/ms 16.45, physics/insert/ms 1.14, render/present/ms 1.14, bigspace/total/ms 0.91, terrain/update/ms 0.70, controller/feed/ms 0.67, terrain/select/ms 0.64, far/select/ms 0.52 |
| measure | 12.62 | 91.7 | render/thread/ms 183, render/acquire/ms 152, render/busy/ms 30.24, main/busy/ms 19.81, render/graph/ms 17.04, render/present/ms 1.19, bigspace/total/ms 0.88, physics/insert/ms 0.79, terrain/update/ms 0.76, controller/feed/ms 0.70, terrain/select/ms 0.69, far/select/ms 0.54 |
| measure | 12.71 | 90.5 | render/thread/ms 180, render/acquire/ms 151, render/busy/ms 27.79, main/busy/ms 17.65, render/graph/ms 16.27, render/present/ms 1.11, physics/insert/ms 0.75, bigspace/total/ms 0.74, terrain/update/ms 0.71, terrain/select/ms 0.64, controller/feed/ms 0.63, stream/drive/ms 0.46 |
| measure | 12.81 | 96.5 | render/thread/ms 184, render/acquire/ms 157, render/busy/ms 26.69, main/busy/ms 19.74, render/graph/ms 14.67, physics/insert/ms 1.03, render/present/ms 1.01, controller/feed/ms 0.81, bigspace/total/ms 0.77, terrain/update/ms 0.58, terrain/select/ms 0.53, stream/drive/ms 0.48 |
| measure | 12.91 | 103.2 | render/thread/ms 197, render/acquire/ms 170, render/busy/ms 26.56, main/busy/ms 19.30, render/graph/ms 14.52, render/present/ms 1.02, bigspace/total/ms 0.84, terrain/update/ms 0.77, controller/feed/ms 0.77, physics/insert/ms 0.74, terrain/select/ms 0.70, stream/drive/ms 0.45 |
| measure | 13.00 | 92.2 | render/thread/ms 193, render/acquire/ms 163, render/busy/ms 28.68, main/busy/ms 18.75, render/graph/ms 16.95, render/present/ms 1.05, stream/drive/ms 0.97, bigspace/total/ms 0.91, terrain/update/ms 0.77, terrain/select/ms 0.69, controller/feed/ms 0.54, stream/deactivate/ms 0.52 |
| measure | 13.10 | 94.4 | render/thread/ms 184, render/acquire/ms 154, render/busy/ms 28.97, main/busy/ms 19.55, render/graph/ms 16.58, stream/drive/ms 1.46, render/present/ms 1.09, stream/deactivate/ms 1.01, bigspace/total/ms 0.91, controller/feed/ms 0.61, terrain/update/ms 0.58, terrain/select/ms 0.53 |
| measure | 13.20 | 100.4 | render/thread/ms 192, render/acquire/ms 163, render/busy/ms 28.60, main/busy/ms 20.62, render/graph/ms 16.27, stream/drive/ms 1.67, render/present/ms 1.12, bigspace/total/ms 0.78, stream/despawn/ms 0.75, controller/feed/ms 0.69, terrain/update/ms 0.68, terrain/select/ms 0.62 |
| measure | 13.29 | 91.5 | render/thread/ms 189, render/acquire/ms 157, render/busy/ms 31.47, main/busy/ms 23.23, render/graph/ms 18.84, stream/drive/ms 1.69, stream/despawn/ms 1.27, render/present/ms 1.05, bigspace/total/ms 0.88, terrain/update/ms 0.67, controller/feed/ms 0.63, terrain/select/ms 0.61 |
| measure | 13.39 | 98.9 | render/thread/ms 188, render/acquire/ms 152, render/busy/ms 34.35, main/busy/ms 22.99, render/graph/ms 20.77, stream/drive/ms 1.25, render/present/ms 1.12, bigspace/total/ms 1.05, stream/despawn/ms 0.84, terrain/update/ms 0.72, controller/feed/ms 0.62, anim/evaluate_cpu/ms 0.56 |
| measure | 13.49 | 102.4 | render/thread/ms 199, render/acquire/ms 161, render/busy/ms 36.79, render/graph/ms 23.34, main/busy/ms 20.11, render/present/ms 1.25, bigspace/total/ms 0.90, stream/drive/ms 0.84, terrain/update/ms 0.79, physics/insert/ms 0.71, controller/feed/ms 0.61, terrain/select/ms 0.56 |
| measure | 13.59 | 100.3 | render/thread/ms 200, render/acquire/ms 164, render/busy/ms 34.31, render/graph/ms 21.64, main/busy/ms 21.63, render/present/ms 1.09, bigspace/total/ms 0.87, physics/insert/ms 0.74, anim/evaluate_cpu/ms 0.70, terrain/update/ms 0.70, terrain/select/ms 0.63, controller/feed/ms 0.58 |
| measure | 13.70 | 111.6 | render/thread/ms 209, render/acquire/ms 172, render/busy/ms 35.35, main/busy/ms 21.00, render/graph/ms 19.00, render/present/ms 1.04, bigspace/total/ms 0.90, terrain/update/ms 0.70, anim/evaluate_cpu/ms 0.70, terrain/select/ms 0.64, controller/feed/ms 0.58, physics/insert/ms 0.52 |
| measure | 13.80 | 94.3 | render/thread/ms 203, render/acquire/ms 160, render/busy/ms 41.41, render/graph/ms 19.55, main/busy/ms 19.02, render/present/ms 1.14, bigspace/total/ms 0.86, terrain/update/ms 0.67, terrain/select/ms 0.60, physics/insert/ms 0.57, controller/feed/ms 0.55, stream/drive/ms 0.45 |
| measure | 13.89 | 97.6 | render/thread/ms 189, render/acquire/ms 151, render/busy/ms 37.01, render/graph/ms 19.42, main/busy/ms 18.78, render/present/ms 1.14, bigspace/total/ms 0.84, terrain/update/ms 0.66, physics/insert/ms 0.64, terrain/select/ms 0.60, controller/feed/ms 0.53, stream/drive/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_new_nomaps\balmora_walk@2560x1440_off=normalmaps_orm_parallax-3 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 7%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **90%** (min 61%) · 2576 MHz (min 2565) · memory 8001 MHz · 71.0 W · 75 °C (max 82) · VRAM 7846 MiB · throttled 100% of samples
