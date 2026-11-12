# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5716 frames over 65.6 s · **median 10.66 ms (94 fps)** · mean 11.48 · p95 14.96 · p99 21.63 (1% low 46 fps) · worst 101.70 ms · 174 spikes over 1.5× median · 85.9 stutters/min (work 103.3/min)
- warm-up: 346 frames over 6.5 s, worst 592.6 ms
- hitches (> 26.7 ms): **42** measured, 31 in warm-up
- timed GPU passes: 7.96 ms of a 11.48 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.66 ms, limit 11 ms
- **FAIL** frame_ms: 94 stutters (> 1.5x moving median of 60) in 1.09 min = 85.92/min, limit 1/min
- **FAIL** worst frame 101.70 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.117 | 2.480 | 0.179 |
| main_opaque_pass_3d | 1.432 | 1.865 | 0.045 |
| atmosphere_luts | 0.847 | 1.127 | 0.003 |
| early prepass | 0.557 | 1.092 | 0.075 |
| lightprobe_irradiance_map | 0.446 | 0.746 | 0.001 |
| bloom | 0.326 | 0.329 | 0.066 |
| lightprobe_radiance_map | 0.226 | 0.235 | 0.004 |
| auxiliary_views | 0.226 | 0.962 | 0.136 |
| ssao | 0.208 | 0.209 | 0.007 |
| uniform_allocation | 0.201 | 0.206 | 0.032 |
| bin_unpacking | 0.130 | 0.141 | 0.023 |
| clouds_trace | 0.127 | 0.213 | 0.001 |
| water_fft | 0.127 | 0.127 | 0.003 |
| upscaling | 0.123 | 0.140 | 0.004 |
| atmos_froxel_inject | 0.095 | 0.101 | 0.001 |
| tonemapping | 0.088 | 0.090 | 0.003 |
| water_velocity | 0.083 | 0.086 | 0.002 |
| clustering | 0.067 | 0.070 | 0.019 |
| main_transparent_pass_3d | 0.064 | 0.108 | 0.151 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| surface_trail | 0.062 | 0.057 | 0.002 |
| early_mesh_preprocessing | 0.060 | 0.060 | 0.005 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.034 | 0.036 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.027 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.026 | 0.007 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.186 | 13.347 | 99.03 | 58222.9 |
| render/busy/ms | 9.567 | 12.770 | 98.22 | 54682.8 |
| main/busy/ms | 7.874 | 9.768 | 37.17 | 45005.2 |
| pipelines/wait_ms | 4.905 | 0.000 | 8232.76 | 28034.5 |
| render/graph/ms | 4.499 | 5.914 | 18.12 | 25714.6 |
| pipelines/compile_ms | 1.082 | 0.000 | 2106.76 | 6182.5 |
| pipelines/driver_ms | 0.915 | 0.000 | 1902.20 | 5229.8 |
| render/present/ms | 0.496 | 1.168 | 4.87 | 2833.7 |
| bigspace/total/ms | 0.472 | 0.825 | 2.42 | 2697.2 |
| terrain/update/ms | 0.293 | 0.367 | 8.28 | 1676.5 |
| controller/feed/ms | 0.274 | 0.359 | 0.73 | 1563.8 |
| terrain/select/ms | 0.260 | 0.327 | 0.76 | 1484.4 |
| stream/drive/ms | 0.223 | 0.322 | 2.18 | 1272.9 |
| far/select/ms | 0.193 | 0.273 | 0.94 | 1102.8 |
| stream/tick/ms | 0.180 | 0.237 | 0.65 | 1028.2 |
| bigspace/low_precision/ms | 0.179 | 0.348 | 1.68 | 1024.9 |
| anim/evaluate_cpu/ms | 0.169 | 0.217 | 0.83 | 967.3 |
| pipelines/front_ms | 0.167 | 0.000 | 204.56 | 952.7 |
| anim/evaluate/ms | 0.155 | 0.293 | 2.06 | 884.7 |
| controller/move/ms | 0.149 | 0.308 | 0.67 | 854.0 |
| render/acquire/ms | 0.124 | 0.285 | 2.45 | 706.4 |
| bigspace/high_precision/ms | 0.115 | 0.259 | 2.04 | 659.3 |
| physics/step/ms | 0.047 | 0.086 | 0.70 | 268.1 |
| physics/insert/ms | 0.039 | 0.388 | 0.83 | 224.7 |
| weather/map/ms | 0.026 | 0.034 | 0.21 | 149.0 |
| anim/apply/ms | 0.011 | 0.021 | 0.24 | 65.2 |
| controller/avoid/ms | 0.011 | 0.015 | 0.19 | 64.2 |
| weather/eval/ms | 0.011 | 0.015 | 0.25 | 63.2 |
| script/update/ms | 0.007 | 0.009 | 0.31 | 40.5 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 28.5 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 27.5 |
| stream/despawn/ms | 0.005 | 0.000 | 0.94 | 26.9 |
| stream/spawn/ms | 0.004 | 0.000 | 1.06 | 23.7 |
| controller/write/ms | 0.003 | 0.005 | 0.26 | 19.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 13.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.09 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 20 | 0 |
| anim/a2/count | count | 0 | 10 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 102 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
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
| frame/work/ms | ms | 8.80 | 98.22 | 8.38 |
| gpu/timed/ms | ms | 7.97 | 12.09 | 8.08 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 262 | 486 | 478 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3970 | 4350 | 4314 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 262 | 486 | 478 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3970 | 4350 | 4314 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 4118 | 4334 | 4333 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 4118 | 4334 | 4333 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 39 | 352 | 31 |
| phases/shadow/bins | count | 63.1 k | 174.1 k | 12.0 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 206.5 k | 559.8 k | 39.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 146 | 362 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 304 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 173 | 174 | 174 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 10 | 847 | 6 |
| preprocess/bind_groups_created | count | 10 | 847 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 170 | 278 | 164 |
| process/cpu_usage | % | 24.16 | 42.79 | 25 |
| process/mem_usage | GiB | 2.86 | 2.93 | 2.92 |
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
| started_load_count |  | 3482 | 3535 | 3535 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 431 | 530 | 398 |
| statics/materials_bound | count | 431 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.12 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 327 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.39 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.64 | 70.56 | 32.11 |
| system/mem_usage | % | 50.16 | 52.82 | 50.19 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 83 | 392 | 75 |
| views/all/bins | count | 70.7 k | 182.4 k | 21.6 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 231.7 k | 588.1 k | 69.0 k |
| views/all/pipelines | count | 182 | 398 | 71 |
| views/all/pipelines_distinct | count | 62 | 69 | 55 |
| views/all/work_items | count | 69.2 k | 575.6 k | 69.7 k |
| views/cascade/batch_sets | count | 38 | 50 | 31 |
| views/cascade/bins | count | 11.9 k | 12.5 k | 12.0 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.5 k | 40.0 k | 39.4 k |
| views/cascade/pipelines | count | 38 | 50 | 31 |
| views/cascade/work_items | count | 38.5 k | 40.0 k | 39.4 k |
| views/lamp_face/batch_sets | count | 0 | 311 | 0 |
| views/lamp_face/bins | count | 53.0 k | 162.2 k | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 173.1 k | 521.7 k | 0 |
| views/lamp_face/pipelines | count | 108 | 317 | 0 |
| views/lamp_face/work_items | count | 0 | 510.3 k | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 8762 | 9622 | 9622 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.1 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 28.6 k | 30.5 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.68 | 847 | 272.5 k |
| anim/events/count | count | 0.29 | 9 | 1633 |
| assets/Image/added | count | 0.07 | 40 | 379 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.74 | 160 | 4257 |
| assets/Mesh/removed | count | 0.86 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.10 | 47 | 548 |
| assets/StandardMaterial/modified | count | 3.71 | 11 | 21.2 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.59 | 3410 | 9106 |
| physics/inserts | count | 0.44 | 102 | 2506 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.68 | 3 | 3888 |
| physics/steps_dropped | count | 0.01 | 3 | 51 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 18 | 378 |
| stream/upload/bytes | bytes | 5922 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1298 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 592.6 | main/busy/ms 569, surface/advance_cpu_ms 67.49, surface/cell_upload_ms 0.78, stream/drive/ms 0.45, script/ui/ms 0.14, weather/map/ms 0.10, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01 |
| warm-up | 1.00 | 412.2 | main/busy/ms 605, render/thread/ms 373, render/busy/ms 300, render/acquire/ms 70.25, surface/advance_cpu_ms 67.49, render/graph/ms 4.85, render/present/ms 1.90, stream/drive/ms 1.19, surface/cell_upload_ms 0.78, stream/tick/ms 0.54, terrain/update/ms 0.49, terrain/select/ms 0.36 |
| warm-up | 1.34 | 98.1 | pipelines/wait_ms 486, pipelines/compile_ms 134, pipelines/front_ms 126, render/thread/ms 58.29, render/busy/ms 56.47, main/busy/ms 54.39, pipelines/driver_ms 8.34, render/graph/ms 3.02, render/present/ms 1.47, stream/drive/ms 1.35, stream/tick/ms 1.10, terrain/update/ms 0.73 |
| warm-up | 1.83 | 31.2 | pipelines/wait_ms 4699, pipelines/compile_ms 143, pipelines/front_ms 134, render/thread/ms 43.52, render/busy/ms 42.29, main/busy/ms 18.09, pipelines/driver_ms 9.03, render/graph/ms 5.08, stream/drive/ms 3.41, stream/spawn/ms 1.48, render/present/ms 0.99, stream/tick/ms 0.92 |
| warm-up | 2.46 | 30.1 | render/thread/ms 37.55, render/busy/ms 36.44, main/busy/ms 14.36, render/graph/ms 5.86, stream/drive/ms 2.65, stream/spawn/ms 1.54, render/present/ms 0.90, stream/tick/ms 0.75, terrain/update/ms 0.58, terrain/select/ms 0.50, bigspace/total/ms 0.45, bigspace/high_precision/ms 0.22 |
| warm-up | 2.52 | 34.4 | pipelines/wait_ms 3648, pipelines/compile_ms 142, pipelines/front_ms 138, render/thread/ms 58.03, render/busy/ms 56.62, main/busy/ms 14.59, render/graph/ms 5.82, pipelines/driver_ms 4.51, stream/drive/ms 2.68, render/present/ms 1.18, stream/spawn/ms 1.01, stream/tick/ms 0.68 |
| warm-up | 2.61 | 44.9 | pipelines/wait_ms 1010, pipelines/compile_ms 109, pipelines/front_ms 105, render/thread/ms 56.43, render/busy/ms 55.26, main/busy/ms 16.44, render/graph/ms 5.84, pipelines/driver_ms 3.69, stream/drive/ms 2.54, stream/spawn/ms 1.39, render/present/ms 0.95, stream/tick/ms 0.67 |
| warm-up | 2.74 | 43.8 | pipelines/wait_ms 4301, pipelines/compile_ms 99.83, pipelines/front_ms 95.79, render/thread/ms 66.59, render/busy/ms 64.69, main/busy/ms 17.99, render/graph/ms 6.86, pipelines/driver_ms 4.04, stream/drive/ms 2.42, render/present/ms 1.65, stream/tick/ms 0.98, stream/spawn/ms 0.81 |
| warm-up | 2.84 | 46.9 | pipelines/wait_ms 4784, pipelines/compile_ms 132, pipelines/front_ms 128, render/thread/ms 66.33, render/busy/ms 65.05, main/busy/ms 20.44, render/graph/ms 6.40, pipelines/driver_ms 3.48, stream/drive/ms 3.14, stream/spawn/ms 2.00, physics/insert/ms 1.23, render/present/ms 1.02 |
| warm-up | 2.88 | 39.5 | pipelines/wait_ms 4784, pipelines/compile_ms 132, pipelines/front_ms 128, render/thread/ms 83.28, render/busy/ms 81.69, main/busy/ms 21.29, render/graph/ms 7.65, pipelines/driver_ms 3.48, stream/drive/ms 3.35, stream/spawn/ms 2.17, render/present/ms 1.33, physics/insert/ms 1.30 |
| warm-up | 2.92 | 41.9 | render/thread/ms 78.27, render/busy/ms 76.68, main/busy/ms 22.29, render/graph/ms 7.62, stream/drive/ms 3.88, stream/spawn/ms 2.19, render/present/ms 1.33, physics/insert/ms 1.00, stream/tick/ms 0.74, physics/step/ms 0.66, terrain/update/ms 0.62, terrain/select/ms 0.52 |
| warm-up | 3.01 | 35.6 | pipelines/wait_ms 9626, pipelines/compile_ms 170, pipelines/front_ms 163, render/thread/ms 58.60, render/busy/ms 57.44, main/busy/ms 19.98, pipelines/driver_ms 7.10, render/graph/ms 6.39, stream/drive/ms 3.63, stream/spawn/ms 2.06, render/present/ms 0.93, stream/tick/ms 0.84 |
| warm-up | 3.05 | 38.8 | pipelines/wait_ms 6610, render/thread/ms 70.81, render/busy/ms 69.54, pipelines/compile_ms 36.32, pipelines/front_ms 34.32, main/busy/ms 24.25, render/graph/ms 6.84, stream/drive/ms 2.99, pipelines/driver_ms 2.00, stream/spawn/ms 1.21, render/present/ms 0.98, bigspace/total/ms 0.90 |
| warm-up | 3.10 | 31.4 | render/thread/ms 47.50, render/busy/ms 46.37, main/busy/ms 21.90, render/graph/ms 6.75, stream/drive/ms 3.56, stream/spawn/ms 1.72, render/present/ms 0.94, stream/tick/ms 0.86, physics/insert/ms 0.65, bigspace/total/ms 0.61, terrain/update/ms 0.60, terrain/select/ms 0.50 |
| warm-up | 3.16 | 48.2 | pipelines/wait_ms 5675, pipelines/compile_ms 151, pipelines/front_ms 145, render/thread/ms 58.36, render/busy/ms 57.16, main/busy/ms 21.64, render/graph/ms 8.72, pipelines/driver_ms 5.98, stream/drive/ms 3.82, stream/spawn/ms 2.08, render/present/ms 0.98, terrain/update/ms 0.88 |
| warm-up | 3.19 | 29.8 | pipelines/wait_ms 5675, pipelines/compile_ms 151, pipelines/front_ms 145, render/thread/ms 73.73, render/busy/ms 72.38, main/busy/ms 21.36, render/graph/ms 7.66, pipelines/driver_ms 5.98, stream/drive/ms 4.00, stream/spawn/ms 2.08, render/present/ms 1.10, stream/tick/ms 0.70 |
| warm-up | 3.24 | 49.9 | pipelines/wait_ms 3497, pipelines/compile_ms 97.38, pipelines/front_ms 94.10, render/thread/ms 75.05, render/busy/ms 73.54, main/busy/ms 22.70, render/graph/ms 7.39, stream/drive/ms 4.24, pipelines/driver_ms 3.28, stream/spawn/ms 2.14, render/present/ms 1.24, stream/tick/ms 0.72 |
| warm-up | 3.30 | 56.9 | pipelines/wait_ms 13.7 k, pipelines/compile_ms 133, pipelines/front_ms 127, render/thread/ms 102, render/busy/ms 101, main/busy/ms 24.50, render/graph/ms 7.66, pipelines/driver_ms 6.10, stream/drive/ms 4.23, stream/spawn/ms 2.21, render/present/ms 1.12, physics/insert/ms 0.93 |
| warm-up | 3.33 | 31.8 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 99.48, pipelines/front_ms 93.14, render/thread/ms 84.09, render/busy/ms 82.90, main/busy/ms 23.36, render/graph/ms 7.20, pipelines/driver_ms 6.33, stream/drive/ms 3.70, stream/spawn/ms 2.16, physics/insert/ms 1.07, render/present/ms 0.94 |
| warm-up | 3.39 | 39.4 | pipelines/wait_ms 5522, pipelines/compile_ms 60.71, pipelines/front_ms 58.74, render/thread/ms 53.77, render/busy/ms 52.48, main/busy/ms 21.34, render/graph/ms 7.92, stream/drive/ms 2.27, pipelines/driver_ms 1.97, stream/spawn/ms 1.07, render/present/ms 1.03, terrain/update/ms 0.75 |
| warm-up | 3.51 | 34.6 | render/thread/ms 53.71, render/busy/ms 52.24, main/busy/ms 22.00, render/graph/ms 9.59, stream/drive/ms 2.48, render/present/ms 1.21, physics/insert/ms 1.13, stream/spawn/ms 0.92, stream/tick/ms 0.77, terrain/update/ms 0.70, bigspace/total/ms 0.61, terrain/select/ms 0.58 |
| warm-up | 3.63 | 30.6 | render/thread/ms 52.16, render/busy/ms 50.67, main/busy/ms 25.07, render/graph/ms 8.82, stream/drive/ms 3.60, stream/spawn/ms 2.31, render/present/ms 1.23, physics/insert/ms 1.22, bigspace/total/ms 0.69, stream/tick/ms 0.66, terrain/update/ms 0.62, terrain/select/ms 0.52 |
| warm-up | 3.66 | 29.3 | pipelines/wait_ms 7261, pipelines/compile_ms 91.78, pipelines/front_ms 87.97, render/thread/ms 55.03, render/busy/ms 52.85, main/busy/ms 26.63, render/graph/ms 9.23, pipelines/driver_ms 3.81, stream/drive/ms 3.70, stream/spawn/ms 2.27, render/present/ms 1.88, physics/insert/ms 1.27 |
| warm-up | 3.83 | 31.1 | render/thread/ms 50.86, render/busy/ms 48.86, main/busy/ms 42.07, render/graph/ms 13.69, stream/drive/ms 3.12, render/present/ms 1.70, physics/insert/ms 1.61, stream/spawn/ms 1.40, stream/tick/ms 0.98, terrain/update/ms 0.97, bigspace/total/ms 0.87, terrain/select/ms 0.65 |
| measure | 0.47 | 101.7 | pipelines/wait_ms 12.6 k, pipelines/compile_ms 169, pipelines/front_ms 165, render/thread/ms 111, render/busy/ms 109, render/graph/ms 24.04, main/busy/ms 19.69, pipelines/driver_ms 3.89, render/present/ms 1.19, bigspace/total/ms 0.93, terrain/update/ms 0.65, terrain/select/ms 0.58 |
| measure | 0.49 | 28.7 | pipelines/wait_ms 8233, render/thread/ms 125, render/busy/ms 122, main/busy/ms 46.72, render/graph/ms 30.33, pipelines/compile_ms 16.81, pipelines/front_ms 16.20, render/present/ms 2.37, bigspace/total/ms 1.24, anim/evaluate_cpu/ms 1.04, anim/evaluate/ms 0.82, terrain/update/ms 0.72 |
| measure | 0.53 | 40.7 | main/busy/ms 55.43, render/thread/ms 52.55, render/busy/ms 50.14, render/graph/ms 16.97, render/present/ms 2.22, bigspace/total/ms 1.61, anim/evaluate_cpu/ms 1.15, anim/evaluate/ms 1.00, stream/drive/ms 0.79, bigspace/low_precision/ms 0.77, terrain/update/ms 0.75, physics/insert/ms 0.72 |
| measure | 0.87 | 58.8 | render/thread/ms 70.63, render/busy/ms 69.17, main/busy/ms 26.71, render/graph/ms 12.46, render/present/ms 1.20, bigspace/total/ms 1.18, stream/drive/ms 0.99, physics/insert/ms 0.71, terrain/update/ms 0.60, stream/spawn/ms 0.54, controller/feed/ms 0.54, terrain/select/ms 0.53 |
| measure | 2.23 | 85.5 | render/thread/ms 95.98, render/busy/ms 94.58, render/graph/ms 19.31, main/busy/ms 16.90, render/present/ms 1.16, bigspace/total/ms 0.89, physics/insert/ms 0.80, terrain/update/ms 0.56, terrain/select/ms 0.50, stream/drive/ms 0.44, controller/feed/ms 0.42, far/select/ms 0.38 |
| measure | 2.77 | 79.6 | render/thread/ms 89.28, render/busy/ms 88.20, render/graph/ms 18.95, main/busy/ms 16.60, render/present/ms 0.87, bigspace/total/ms 0.77, physics/insert/ms 0.64, stream/drive/ms 0.60, terrain/update/ms 0.55, stream/tick/ms 0.54, terrain/select/ms 0.50, controller/feed/ms 0.45 |
| measure | 3.18 | 28.9 | render/thread/ms 42.63, render/busy/ms 41.53, main/busy/ms 21.44, render/graph/ms 12.77, stream/drive/ms 1.25, render/present/ms 0.88, bigspace/total/ms 0.85, physics/insert/ms 0.72, controller/feed/ms 0.63, stream/spawn/ms 0.60, terrain/update/ms 0.59, terrain/select/ms 0.53 |
| measure | 3.21 | 28.4 | render/thread/ms 54.71, render/busy/ms 53.55, main/busy/ms 17.80, render/graph/ms 12.17, render/present/ms 0.91, bigspace/total/ms 0.75, physics/insert/ms 0.69, stream/drive/ms 0.61, controller/feed/ms 0.59, terrain/update/ms 0.58, terrain/select/ms 0.52, stream/tick/ms 0.50 |
| measure | 6.07 | 72.4 | render/thread/ms 81.11, render/busy/ms 79.87, main/busy/ms 14.59, render/graph/ms 10.04, render/present/ms 1.03, bigspace/total/ms 0.86, terrain/update/ms 0.58, terrain/select/ms 0.52, controller/feed/ms 0.49, stream/drive/ms 0.41, bigspace/low_precision/ms 0.39, stream/tick/ms 0.36 |
| measure | 8.16 | 63.2 | render/thread/ms 70.41, render/busy/ms 69.27, main/busy/ms 14.31, render/graph/ms 8.47, bigspace/total/ms 0.97, render/present/ms 0.91, terrain/update/ms 0.54, terrain/select/ms 0.49, stream/drive/ms 0.40, controller/feed/ms 0.39, bigspace/low_precision/ms 0.37, stream/tick/ms 0.35 |
| measure | 9.35 | 92.2 | render/thread/ms 101, render/busy/ms 99.59, main/busy/ms 14.19, render/graph/ms 11.88, render/present/ms 1.08, bigspace/total/ms 0.88, terrain/update/ms 0.52, terrain/select/ms 0.47, bigspace/low_precision/ms 0.45, controller/feed/ms 0.41, stream/drive/ms 0.40, stream/tick/ms 0.36 |
| measure | 9.44 | 79.3 | render/thread/ms 89.70, render/busy/ms 88.48, main/busy/ms 20.12, render/graph/ms 11.32, bigspace/total/ms 1.01, render/present/ms 0.97, stream/drive/ms 0.69, far/select/ms 0.60, controller/feed/ms 0.60, terrain/update/ms 0.56, stream/tick/ms 0.55, terrain/select/ms 0.49 |
| measure | 9.51 | 30.8 | render/thread/ms 43.18, render/busy/ms 41.50, main/busy/ms 20.31, render/graph/ms 12.55, stream/drive/ms 1.97, render/present/ms 1.46, bigspace/total/ms 1.11, stream/despawn/ms 1.04, anim/evaluate/ms 0.70, terrain/update/ms 0.67, bigspace/low_precision/ms 0.54, anim/evaluate_cpu/ms 0.49 |
| measure | 11.56 | 65.7 | render/thread/ms 73.95, render/busy/ms 72.90, main/busy/ms 15.51, render/graph/ms 8.85, bigspace/total/ms 0.94, render/present/ms 0.81, terrain/update/ms 0.56, bigspace/low_precision/ms 0.51, terrain/select/ms 0.49, controller/feed/ms 0.46, stream/drive/ms 0.39, stream/tick/ms 0.34 |
| measure | 12.03 | 68.5 | render/thread/ms 77.88, render/busy/ms 76.78, main/busy/ms 15.80, render/graph/ms 9.54, bigspace/total/ms 0.91, render/present/ms 0.83, terrain/update/ms 0.53, stream/drive/ms 0.50, controller/feed/ms 0.50, terrain/select/ms 0.48, stream/tick/ms 0.45, controller/move/ms 0.38 |
| measure | 16.19 | 66.3 | render/thread/ms 73.88, render/busy/ms 72.79, main/busy/ms 13.17, render/graph/ms 9.53, render/present/ms 0.86, bigspace/total/ms 0.79, terrain/update/ms 0.54, terrain/select/ms 0.49, controller/feed/ms 0.45, stream/drive/ms 0.42, far/select/ms 0.40, stream/tick/ms 0.37 |
| measure | 16.41 | 65.5 | render/thread/ms 73.29, render/busy/ms 72.12, main/busy/ms 14.08, render/graph/ms 9.72, bigspace/total/ms 1.03, render/present/ms 0.93, terrain/update/ms 0.57, controller/feed/ms 0.55, terrain/select/ms 0.51, stream/drive/ms 0.51, stream/tick/ms 0.46, anim/evaluate_cpu/ms 0.34 |
| measure | 16.99 | 65.4 | render/thread/ms 73.83, render/busy/ms 72.71, main/busy/ms 15.09, render/graph/ms 9.21, render/present/ms 0.89, bigspace/total/ms 0.71, terrain/update/ms 0.54, terrain/select/ms 0.48, controller/feed/ms 0.46, anim/evaluate/ms 0.41, stream/drive/ms 0.37, anim/evaluate_cpu/ms 0.36 |
| measure | 18.12 | 66.0 | render/thread/ms 73.26, render/busy/ms 72.23, main/busy/ms 13.51, render/graph/ms 8.32, render/present/ms 0.78, bigspace/total/ms 0.74, terrain/update/ms 0.59, terrain/select/ms 0.52, controller/feed/ms 0.52, stream/drive/ms 0.37, anim/evaluate_cpu/ms 0.33, far/select/ms 0.32 |
| measure | 18.25 | 28.4 | render/thread/ms 37.87, render/busy/ms 36.80, main/busy/ms 17.40, render/graph/ms 10.88, stream/drive/ms 1.06, render/present/ms 0.85, bigspace/total/ms 0.78, stream/spawn/ms 0.63, terrain/update/ms 0.56, controller/feed/ms 0.54, terrain/select/ms 0.50, anim/evaluate_cpu/ms 0.42 |
| measure | 18.81 | 75.1 | render/thread/ms 87.76, render/busy/ms 86.50, main/busy/ms 16.42, render/graph/ms 11.95, render/present/ms 1.01, bigspace/total/ms 0.82, controller/feed/ms 0.68, terrain/update/ms 0.54, anim/evaluate_cpu/ms 0.54, terrain/select/ms 0.48, stream/drive/ms 0.40, far/select/ms 0.36 |
| measure | 19.28 | 73.3 | render/thread/ms 83.42, render/busy/ms 82.18, main/busy/ms 16.13, render/graph/ms 10.69, render/present/ms 1.00, physics/insert/ms 0.88, bigspace/total/ms 0.83, controller/feed/ms 0.60, terrain/update/ms 0.56, terrain/select/ms 0.51, anim/evaluate_cpu/ms 0.41, stream/drive/ms 0.40 |
| measure | 19.39 | 90.0 | render/thread/ms 99.89, render/busy/ms 98.72, render/graph/ms 18.82, main/busy/ms 17.42, render/present/ms 0.95, physics/insert/ms 0.80, bigspace/total/ms 0.77, controller/feed/ms 0.71, stream/drive/ms 0.55, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/tick/ms 0.44 |
| measure | 20.17 | 76.1 | render/thread/ms 86.24, render/busy/ms 84.98, main/busy/ms 17.14, render/graph/ms 11.85, render/present/ms 1.02, bigspace/total/ms 0.88, physics/insert/ms 0.77, controller/feed/ms 0.64, terrain/update/ms 0.59, terrain/select/ms 0.53, anim/evaluate_cpu/ms 0.41, stream/drive/ms 0.39 |
| measure | 20.99 | 72.0 | render/thread/ms 81.49, render/busy/ms 80.34, main/busy/ms 15.55, render/graph/ms 10.64, render/present/ms 0.95, controller/feed/ms 0.75, bigspace/total/ms 0.74, terrain/update/ms 0.57, terrain/select/ms 0.51, anim/evaluate_cpu/ms 0.42, stream/drive/ms 0.40, stream/tick/ms 0.35 |
| measure | 22.27 | 70.8 | render/thread/ms 79.35, render/busy/ms 78.29, main/busy/ms 17.67, render/graph/ms 10.72, render/present/ms 0.87, bigspace/total/ms 0.77, terrain/update/ms 0.54, controller/feed/ms 0.53, terrain/select/ms 0.49, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.38, stream/tick/ms 0.35 |
| measure | 22.48 | 70.7 | render/thread/ms 79.48, render/busy/ms 78.37, main/busy/ms 15.75, render/graph/ms 9.89, bigspace/total/ms 0.95, render/present/ms 0.89, terrain/update/ms 0.59, terrain/select/ms 0.53, controller/feed/ms 0.47, anim/evaluate_cpu/ms 0.43, stream/drive/ms 0.38, bigspace/low_precision/ms 0.35 |
| measure | 22.78 | 73.1 | render/thread/ms 82.47, render/busy/ms 81.36, main/busy/ms 14.76, render/graph/ms 10.58, render/present/ms 0.89, bigspace/total/ms 0.73, terrain/update/ms 0.64, terrain/select/ms 0.58, controller/feed/ms 0.57, stream/drive/ms 0.43, anim/evaluate_cpu/ms 0.38, stream/tick/ms 0.34 |
| measure | 23.24 | 84.9 | render/thread/ms 102, render/busy/ms 101, main/busy/ms 27.17, render/graph/ms 12.06, bigspace/total/ms 1.42, render/present/ms 1.03, stream/drive/ms 0.96, terrain/update/ms 0.77, controller/feed/ms 0.75, terrain/select/ms 0.69, physics/insert/ms 0.63, anim/evaluate_cpu/ms 0.57 |
| measure | 25.76 | 68.4 | render/thread/ms 76.47, render/busy/ms 75.34, main/busy/ms 14.64, render/graph/ms 9.30, render/present/ms 0.93, bigspace/total/ms 0.81, terrain/update/ms 0.57, controller/feed/ms 0.56, terrain/select/ms 0.51, stream/drive/ms 0.40, stream/tick/ms 0.36, anim/evaluate_cpu/ms 0.33 |
| measure | 26.14 | 75.7 | render/thread/ms 85.04, render/busy/ms 83.86, main/busy/ms 16.16, render/graph/ms 12.45, render/present/ms 0.97, bigspace/total/ms 0.84, terrain/update/ms 0.60, terrain/select/ms 0.53, controller/feed/ms 0.51, anim/evaluate_cpu/ms 0.41, stream/drive/ms 0.41, stream/tick/ms 0.36 |
| measure | 26.23 | 76.7 | render/thread/ms 87.04, render/busy/ms 86.01, main/busy/ms 17.92, render/graph/ms 12.07, bigspace/total/ms 0.93, render/present/ms 0.83, terrain/update/ms 0.66, controller/feed/ms 0.60, terrain/select/ms 0.59, bigspace/low_precision/ms 0.45, stream/drive/ms 0.45, anim/evaluate_cpu/ms 0.39 |
| measure | 26.33 | 71.3 | render/thread/ms 83.01, render/busy/ms 81.93, main/busy/ms 15.81, render/graph/ms 11.28, bigspace/total/ms 0.93, render/present/ms 0.86, terrain/update/ms 0.58, controller/feed/ms 0.55, terrain/select/ms 0.52, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.40, bigspace/low_precision/ms 0.37 |
| measure | 26.41 | 82.0 | render/thread/ms 150, render/busy/ms 149, main/busy/ms 15.56, render/graph/ms 11.49, render/present/ms 0.89, bigspace/total/ms 0.82, terrain/update/ms 0.56, controller/feed/ms 0.54, terrain/select/ms 0.51, stream/drive/ms 0.41, anim/evaluate_cpu/ms 0.38, stream/tick/ms 0.36 |
| measure | 26.52 | 75.6 | render/thread/ms 97.82, render/busy/ms 96.68, main/busy/ms 16.59, render/graph/ms 10.83, bigspace/total/ms 1.00, render/present/ms 0.92, terrain/update/ms 0.57, controller/feed/ms 0.53, terrain/select/ms 0.51, stream/drive/ms 0.49, stream/tick/ms 0.43, anim/evaluate_cpu/ms 0.39 |
| measure | 29.52 | 69.9 | render/thread/ms 78.00, render/busy/ms 76.89, main/busy/ms 16.51, render/graph/ms 8.78, bigspace/total/ms 0.91, render/present/ms 0.89, controller/feed/ms 0.58, terrain/update/ms 0.57, terrain/select/ms 0.51, stream/drive/ms 0.43, anim/evaluate_cpu/ms 0.39, stream/tick/ms 0.38 |
| measure | 30.02 | 70.3 | render/thread/ms 78.73, render/busy/ms 77.57, main/busy/ms 16.83, render/graph/ms 9.44, render/present/ms 0.90, bigspace/total/ms 0.85, terrain/update/ms 0.55, controller/feed/ms 0.54, terrain/select/ms 0.50, stream/drive/ms 0.40, far/select/ms 0.36, stream/tick/ms 0.36 |
| measure | 33.01 | 77.7 | render/thread/ms 85.97, render/busy/ms 84.84, main/busy/ms 14.88, render/graph/ms 8.74, bigspace/total/ms 1.13, render/present/ms 0.91, anim/evaluate_cpu/ms 0.59, controller/feed/ms 0.58, terrain/update/ms 0.53, terrain/select/ms 0.48, anim/evaluate/ms 0.46, stream/drive/ms 0.39 |
| measure | 33.16 | 72.2 | render/thread/ms 79.96, render/busy/ms 78.94, main/busy/ms 15.21, render/graph/ms 8.93, bigspace/total/ms 1.02, render/present/ms 0.79, controller/feed/ms 0.64, terrain/update/ms 0.57, terrain/select/ms 0.52, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.36, stream/tick/ms 0.35 |
| measure | 34.35 | 71.0 | render/thread/ms 79.07, render/busy/ms 78.00, main/busy/ms 15.68, render/graph/ms 9.39, render/present/ms 0.84, bigspace/total/ms 0.74, terrain/update/ms 0.58, terrain/select/ms 0.52, controller/feed/ms 0.49, stream/drive/ms 0.42, stream/tick/ms 0.37, far/select/ms 0.33 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\push_all\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **82%** (min 35%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.2 W · 71 °C (max 78) · VRAM 4389 MiB · throttled 100% of samples
