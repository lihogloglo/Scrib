# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5713 frames over 65.6 s · **median 11.26 ms (89 fps)** · mean 11.49 · p95 13.38 · p99 15.26 (1% low 66 fps) · worst 33.63 ms · 28 spikes over 1.5× median · 17.4 stutters/min (work 24.7/min)
- warm-up: 375 frames over 5.0 s, worst 583.3 ms
- hitches (> 28.1 ms): **2** measured, 5 in warm-up
- timed GPU passes: 7.68 ms of a 11.49 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.26 ms, limit 11 ms
- **FAIL** frame_ms: 19 stutters (> 1.5x moving median of 60) in 1.09 min = 17.37/min, limit 1/min
- **FAIL** worst frame 33.63 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.234 | 2.610 | 0.180 |
| main_opaque_pass_3d | 1.478 | 2.045 | 0.047 |
| early prepass | 0.558 | 0.870 | 0.080 |
| lightprobe_irradiance_map | 0.438 | 0.753 | 0.001 |
| bloom | 0.338 | 0.492 | 0.065 |
| atmosphere_luts | 0.332 | 0.606 | 0.003 |
| lightprobe_radiance_map | 0.230 | 0.237 | 0.004 |
| ssao | 0.217 | 0.211 | 0.007 |
| uniform_allocation | 0.158 | 0.163 | 0.015 |
| water_fft | 0.132 | 0.128 | 0.002 |
| auxiliary_views | 0.131 | 0.517 | 0.119 |
| bin_unpacking | 0.130 | 0.134 | 0.012 |
| clouds_trace | 0.128 | 0.216 | 0.001 |
| upscaling | 0.127 | 0.134 | 0.004 |
| atmos_froxel_inject | 0.101 | 0.108 | 0.002 |
| gpu_instance_materialization | 0.092 | 0.096 | 0.004 |
| tonemapping | 0.091 | 0.091 | 0.003 |
| early_mesh_preprocessing | 0.088 | 0.100 | 0.004 |
| water_velocity | 0.086 | 0.085 | 0.002 |
| clustering | 0.073 | 0.070 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| surface_trail | 0.060 | 0.059 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.105 | 0.083 |
| render_sky | 0.054 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.037 | 0.038 | 0.008 |
| clouds_resolve | 0.035 | 0.039 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| range_unpacking | 0.029 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.031 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.889 | 11.736 | 32.04 | 56498.2 |
| render/busy/ms | 9.350 | 11.179 | 20.69 | 53414.8 |
| main/busy/ms | 8.270 | 9.844 | 15.87 | 47247.0 |
| pipelines/wait_ms | 6.512 | 0.000 | 6679.39 | 37205.3 |
| render/graph/ms | 4.588 | 5.638 | 13.17 | 26209.7 |
| render/present/ms | 0.446 | 0.543 | 20.63 | 2548.3 |
| bigspace/total/ms | 0.411 | 0.628 | 2.61 | 2348.1 |
| terrain/update/ms | 0.289 | 0.370 | 2.33 | 1650.6 |
| controller/feed/ms | 0.272 | 0.355 | 0.72 | 1556.5 |
| stream/drive/ms | 0.267 | 0.445 | 1.92 | 1526.1 |
| terrain/select/ms | 0.254 | 0.323 | 0.72 | 1449.6 |
| far/select/ms | 0.218 | 0.366 | 0.73 | 1246.1 |
| stream/tick/ms | 0.214 | 0.363 | 0.89 | 1221.1 |
| pipelines/compile_ms | 0.182 | 0.000 | 153.25 | 1037.7 |
| pipelines/front_ms | 0.176 | 0.000 | 148.03 | 1005.6 |
| anim/evaluate_cpu/ms | 0.171 | 0.221 | 0.64 | 975.2 |
| bigspace/low_precision/ms | 0.158 | 0.246 | 1.56 | 904.1 |
| anim/evaluate/ms | 0.147 | 0.316 | 1.44 | 839.5 |
| controller/move/ms | 0.145 | 0.299 | 0.80 | 828.9 |
| bigspace/high_precision/ms | 0.099 | 0.167 | 2.20 | 564.1 |
| render/acquire/ms | 0.094 | 0.120 | 16.87 | 535.1 |
| anim/apply/ms | 0.087 | 0.146 | 1.69 | 498.9 |
| physics/insert/ms | 0.051 | 0.425 | 0.97 | 293.9 |
| physics/step/ms | 0.050 | 0.088 | 0.47 | 287.1 |
| weather/map/ms | 0.027 | 0.036 | 0.31 | 156.0 |
| controller/avoid/ms | 0.012 | 0.018 | 0.21 | 66.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.23 | 61.3 |
| script/update/ms | 0.007 | 0.013 | 0.17 | 41.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.12 | 33.0 |
| pipelines/driver_ms | 0.006 | 0.000 | 5.22 | 32.1 |
| script/ui/ms | 0.006 | 0.009 | 0.27 | 32.0 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 28.3 |
| stream/upload/ms | 0.004 | 0.000 | 1.13 | 22.1 |
| controller/write/ms | 0.004 | 0.006 | 0.07 | 20.7 |
| stream/spawn/ms | 0.003 | 0.000 | 1.09 | 19.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.89 | 16.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.05 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
| ecs/entities | count | 48.6 k | 56.7 k | 46.9 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 9.17 | 20.69 | 8.76 |
| gpu/timed/ms | ms | 7.67 | 10.59 | 8.07 |
| mesh_allocator_allocations |  meshes | 10.6 k | 11.8 k | 10.4 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 311.90 M | 311.90 M | 311.90 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 325 | 588 | 516 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 325 | 588 | 516 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 765 | 973 | 757 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2528 | 3576 | 2451 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 765 | 973 | 757 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2528 | 3576 | 2451 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 49 | 212 | 40 |
| phases/shadow/bins | count | 1668 | 2391 | 959 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5168 | 6971 | 3856 |
| phases/shadow/pipelines | count | 104 | 219 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2669 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 494 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 166 | 167 | 167 |
| pipelines/waiting | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 186 | 551 | 167 |
| preprocess/bind_groups_created | count | 179 | 544 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 22.60 | 40.29 | 22.60 |
| process/mem_usage | GiB | 2.61 | 2.68 | 2.68 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2979 | 3169 | 3169 |
| scene/batch/masked/live | count | 2979 | 3169 | 3169 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1594 | 1624 | 1624 |
| scene/chains/live | count | 1594 | 1624 | 1624 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 4998 | 5095 | 5095 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.97 | 66.81 | 28.39 |
| system/mem_usage | % | 49.74 | 50.33 | 50.32 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 99 | 258 | 90 |
| views/all/bins | count | 3499 | 4374 | 3233 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.9 k | 33.9 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 10.9 k | 13.8 k | 9790 |
| views/all/pipelines | count | 144 | 257 | 68 |
| views/all/pipelines_distinct | count | 61 | 65 | 53 |
| views/all/work_items | count | 256.4 k | 258.8 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1434 | 1811 | 959 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4752 | 6041 | 3856 |
| views/cascade/pipelines | count | 38 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.5 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 163 | 0 |
| views/lamp_face/bins | count | 241 | 676 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 399 | 1036 | 0 |
| views/lamp_face/pipelines | count | 66 | 176 | 0 |
| views/lamp_face/work_items | count | 0 | 819 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2024 | 2318 | 2274 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.9 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5950 | 7572 | 5934 |
| views/main/pipelines | count | 40 | 42 | 38 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.7 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 52.35 | 227 | 299.1 k |
| anim/events/count | count | 0.28 | 3 | 1600 |
| assets/Image/added | count | 0.05 | 28 | 308 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.51 | 245 | 2920 |
| assets/Mesh/removed | count | 0.86 | 1228 | 4910 |
| assets/StandardMaterial/added | count | 0.08 | 29 | 475 |
| assets/StandardMaterial/modified | count | 3.79 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.34 | 1296 | 1938 |
| physics/inserts | count | 0.48 | 110 | 2751 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
| physics/steps | count | 0.69 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 8 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5925 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1299 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 583.3 | main/busy/ms 562, surface/advance_cpu_ms 66.38, surface/cell_upload_ms 0.72, stream/drive/ms 0.28, weather/map/ms 0.08, stream/tick/ms 0.07, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 1.01 | 423.0 | main/busy/ms 601, render/thread/ms 388, render/busy/ms 298, render/acquire/ms 87.15, surface/advance_cpu_ms 66.38, render/graph/ms 7.09, render/present/ms 2.08, stream/drive/ms 1.08, surface/cell_upload_ms 0.72, stream/tick/ms 0.53, terrain/update/ms 0.39, terrain/select/ms 0.25 |
| warm-up | 1.04 | 29.8 | render/thread/ms 407, render/busy/ms 316, pipelines/wait_ms 185, render/acquire/ms 87.51, main/busy/ms 55.98, terrain/update/ms 23.43, pipelines/compile_ms 12.75, render/graph/ms 12.41, pipelines/front_ms 12.16, render/present/ms 3.43, stream/drive/ms 1.26, stream/tick/ms 0.84 |
| warm-up | 1.26 | 98.8 | pipelines/compile_ms 208, pipelines/front_ms 192, main/busy/ms 64.90, render/thread/ms 59.02, render/busy/ms 57.01, pipelines/driver_ms 16.42, pipelines/wait_ms 13.69, render/graph/ms 4.40, stream/drive/ms 1.82, render/present/ms 1.71, stream/tick/ms 1.23, terrain/update/ms 0.97 |
| warm-up | 2.90 | 28.9 | main/busy/ms 39.10, render/thread/ms 21.74, render/busy/ms 20.60, render/graph/ms 7.06, stream/drive/ms 1.28, bigspace/total/ms 1.26, render/present/ms 0.96, physics/insert/ms 0.68, stream/tick/ms 0.60, terrain/update/ms 0.54, anim/evaluate_cpu/ms 0.51, terrain/select/ms 0.47 |
| measure | 38.36 | 29.1 | render/thread/ms 37.68, render/busy/ms 21.09, main/busy/ms 17.10, render/present/ms 16.42, render/graph/ms 9.96, controller/feed/ms 0.84, bigspace/total/ms 0.77, terrain/update/ms 0.56, terrain/select/ms 0.49, stream/drive/ms 0.44, anim/evaluate_cpu/ms 0.44, stream/tick/ms 0.38 |
| measure | 58.02 | 33.6 | render/thread/ms 42.85, render/present/ms 22.16, render/busy/ms 20.47, main/busy/ms 18.21, render/graph/ms 10.46, bigspace/total/ms 2.03, controller/feed/ms 0.79, terrain/update/ms 0.55, stream/drive/ms 0.52, terrain/select/ms 0.48, stream/tick/ms 0.45, far/select/ms 0.40 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\fix4_on\balmora_walk@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 6%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **79%** (min 65%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.3 W · 72 °C (max 76) · VRAM 3794 MiB · throttled 100% of samples
