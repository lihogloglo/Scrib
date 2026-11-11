# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5752 frames over 65.6 s · **median 11.10 ms (90 fps)** · mean 11.41 · p95 13.57 · p99 14.80 (1% low 68 fps) · worst 26.66 ms · 18 spikes over 1.5× median · 6.4 stutters/min (work 12.8/min)
- warm-up: 462 frames over 5.7 s, worst 581.3 ms
- hitches (> 27.7 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.31 ms of a 11.41 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.10 ms, limit 11 ms
- **FAIL** frame_ms: 7 stutters (> 1.5x moving median of 60) in 1.09 min = 6.40/min, limit 1/min
- pass worst frame 26.66 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.073 | 2.100 | 0.175 |
| main_opaque_pass_3d | 1.397 | 1.709 | 0.049 |
| early prepass | 0.520 | 0.747 | 0.078 |
| lightprobe_irradiance_map | 0.449 | 0.767 | 0.001 |
| bloom | 0.319 | 0.327 | 0.063 |
| atmosphere_luts | 0.310 | 0.371 | 0.003 |
| lightprobe_radiance_map | 0.223 | 0.233 | 0.004 |
| ssao | 0.207 | 0.208 | 0.007 |
| auxiliary_views | 0.162 | 0.595 | 0.147 |
| uniform_allocation | 0.152 | 0.160 | 0.017 |
| clouds_trace | 0.127 | 0.215 | 0.001 |
| water_fft | 0.126 | 0.127 | 0.002 |
| upscaling | 0.118 | 0.130 | 0.004 |
| bin_unpacking | 0.117 | 0.126 | 0.013 |
| atmos_froxel_inject | 0.093 | 0.100 | 0.002 |
| gpu_instance_materialization | 0.093 | 0.097 | 0.004 |
| early_mesh_preprocessing | 0.091 | 0.098 | 0.004 |
| tonemapping | 0.086 | 0.087 | 0.003 |
| water_velocity | 0.082 | 0.085 | 0.002 |
| clustering | 0.066 | 0.068 | 0.018 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| surface_trail | 0.057 | 0.057 | 0.002 |
| main_transparent_pass_3d | 0.054 | 0.099 | 0.080 |
| render_sky | 0.052 | 0.053 | 0.002 |
| per_view_shadow_pass | 0.037 | 0.038 | 0.008 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| range_unpacking | 0.027 | 0.032 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.028 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
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
| render/thread/ms | 9.883 | 11.958 | 24.91 | 56846.1 |
| render/busy/ms | 9.366 | 11.410 | 24.22 | 53873.2 |
| main/busy/ms | 8.110 | 9.639 | 13.87 | 46646.1 |
| render/graph/ms | 4.707 | 5.883 | 14.93 | 27073.4 |
| pipelines/wait_ms | 2.260 | 0.000 | 4452.32 | 12997.2 |
| render/present/ms | 0.430 | 0.527 | 3.07 | 2472.3 |
| bigspace/total/ms | 0.402 | 0.624 | 2.04 | 2309.8 |
| terrain/update/ms | 0.284 | 0.364 | 1.82 | 1633.6 |
| stream/drive/ms | 0.259 | 0.430 | 1.93 | 1492.2 |
| controller/feed/ms | 0.255 | 0.329 | 0.59 | 1465.5 |
| terrain/select/ms | 0.249 | 0.319 | 0.59 | 1432.6 |
| far/select/ms | 0.214 | 0.356 | 0.63 | 1229.8 |
| stream/tick/ms | 0.207 | 0.356 | 0.70 | 1190.3 |
| anim/evaluate/ms | 0.182 | 0.345 | 1.73 | 1044.0 |
| bigspace/low_precision/ms | 0.153 | 0.241 | 1.80 | 881.7 |
| anim/evaluate_cpu/ms | 0.152 | 0.205 | 0.50 | 874.1 |
| controller/move/ms | 0.142 | 0.299 | 0.58 | 816.6 |
| bigspace/high_precision/ms | 0.097 | 0.142 | 1.79 | 557.3 |
| pipelines/compile_ms | 0.093 | 0.000 | 148.26 | 535.7 |
| pipelines/front_ms | 0.090 | 0.000 | 142.78 | 517.6 |
| render/acquire/ms | 0.087 | 0.111 | 0.38 | 500.6 |
| anim/apply/ms | 0.084 | 0.148 | 1.08 | 485.7 |
| physics/step/ms | 0.047 | 0.082 | 0.51 | 272.3 |
| physics/insert/ms | 0.038 | 0.383 | 1.00 | 220.6 |
| weather/map/ms | 0.026 | 0.034 | 0.12 | 152.0 |
| controller/avoid/ms | 0.011 | 0.017 | 0.17 | 64.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 59.1 |
| script/update/ms | 0.007 | 0.012 | 0.07 | 39.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.27 | 33.7 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 26.8 |
| script/ui/ms | 0.005 | 0.008 | 0.17 | 26.3 |
| controller/write/ms | 0.004 | 0.006 | 0.08 | 20.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.10 | 19.1 |
| pipelines/driver_ms | 0.003 | 0.000 | 5.48 | 18.1 |
| stream/upload/ms | 0.003 | 0.000 | 1.13 | 18.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.69 | 16.4 |
| shapes/activate/ms | 0.001 | 0.002 | 0.38 | 6.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| ecs/entities | count | 48.8 k | 56.7 k | 47.7 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 9.10 | 24.22 | 8.62 |
| gpu/timed/ms | ms | 7.35 | 9.38 | 7.53 |
| mesh_allocator_allocations |  meshes | 10.8 k | 11.8 k | 10.8 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 311.90 M | 311.90 M | 311.90 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d_prepass/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 758 | 973 | 757 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 5881 | 5881 |
| phases/opaque3d/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d/items | count | 2514 | 3575 | 2450 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 758 | 973 | 757 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 5881 | 5881 |
| phases/opaque3d_prepass/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d_prepass/items | count | 2514 | 3575 | 2450 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 48 | 224 | 40 |
| phases/shadow/bins | count | 1648 | 2479 | 951 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 19.8 k | 19.8 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5132 | 7145 | 3848 |
| phases/shadow/pipelines | count | 106 | 226 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 243 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 170 | 171 | 171 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 185 | 575 | 167 |
| preprocess/bind_groups_created | count | 178 | 568 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 23.56 | 39.61 | 26.05 |
| process/mem_usage | GiB | 2.92 | 2.98 | 2.98 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 16.4 k | 16.4 k | 16.4 k |
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
| scene/batch/opaque/capacity | count | 32.8 k | 32.8 k | 32.8 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.4 k | 19.4 k |
| scene/batch/opaque/live | count | 18.8 k | 19.4 k | 19.4 k |
| scene/batch/opaque/overflow | count | 0 | 461 | 461 |
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
| scene/overflow/instances | count | 0 | 461 | 461 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 22.6 k | 22.6 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 22.6 k | 22.6 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5028 | 5177 | 5177 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 237 | 333 | 225 |
| statics/materials_bound | count | 237 | 333 | 225 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.05 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 290 | 380 | 265 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.41 | 44.43 | 27.75 |
| system/mem_usage | % | 43.55 | 43.77 | 43.58 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 99 | 270 | 90 |
| views/all/bins | count | 3476 | 4492 | 3225 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.0 k | 33.0 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 10.8 k | 14.0 k | 9780 |
| views/all/pipelines | count | 145 | 264 | 68 |
| views/all/pipelines_distinct | count | 60 | 66 | 53 |
| views/all/work_items | count | 256.4 k | 259.1 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 56 | 40 |
| views/cascade/bins | count | 1430 | 1776 | 951 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 19.8 k | 19.8 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4739 | 6068 | 3848 |
| views/cascade/pipelines | count | 37 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.5 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 175 | 0 |
| views/lamp_face/bins | count | 210 | 821 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 355 | 1093 | 0 |
| views/lamp_face/pipelines | count | 65 | 180 | 0 |
| views/lamp_face/work_items | count | 0 | 1021 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2026 | 2318 | 2274 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.9 k | 13.2 k | 13.2 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5954 | 7570 | 5932 |
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
| world/meshes | count | 20.5 k | 26.4 k | 19.3 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 43.73 | 216 | 251.5 k |
| anim/events/count | count | 0.26 | 4 | 1467 |
| assets/Image/added | count | 0.05 | 35 | 307 |
| assets/Image/modified | count | 0.00 | 9 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.57 | 255 | 3272 |
| assets/Mesh/removed | count | 0.85 | 1242 | 4910 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 511 |
| assets/StandardMaterial/modified | count | 3.77 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.41 | 1391 | 2331 |
| physics/inserts | count | 0.40 | 84 | 2320 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| scene/overflow/cells | count | 0.00 | 1 | 6 |
| scene/slots/overflow | count | 0.08 | 190 | 461 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5885 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1290 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.3 | main/busy/ms 559, surface/advance_cpu_ms 64.99, surface/cell_upload_ms 0.71, stream/drive/ms 0.30, weather/map/ms 0.12, stream/tick/ms 0.06, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.02, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 1.04 | 461.8 | main/busy/ms 598, render/thread/ms 427, render/busy/ms 353, render/acquire/ms 71.78, surface/advance_cpu_ms 64.99, render/graph/ms 4.82, render/present/ms 2.18, stream/drive/ms 1.15, surface/cell_upload_ms 0.71, stream/tick/ms 0.58, terrain/update/ms 0.44, terrain/select/ms 0.29 |
| warm-up | 1.07 | 30.3 | render/thread/ms 447, render/busy/ms 371, pipelines/wait_ms 361, render/acquire/ms 72.02, main/busy/ms 56.23, terrain/update/ms 22.12, pipelines/compile_ms 11.81, pipelines/front_ms 10.68, render/graph/ms 10.04, render/present/ms 3.71, stream/drive/ms 1.27, pipelines/driver_ms 1.13 |
| warm-up | 1.31 | 87.9 | render/thread/ms 55.92, main/busy/ms 55.67, render/busy/ms 54.52, render/graph/ms 4.11, stream/drive/ms 2.02, stream/tick/ms 1.68, render/present/ms 1.09, terrain/update/ms 0.76, terrain/select/ms 0.64, render/acquire/ms 0.32, bigspace/total/ms 0.20, weather/map/ms 0.14 |
| warm-up | 1.34 | 29.6 | render/thread/ms 36.45, render/busy/ms 34.88, main/busy/ms 20.44, render/graph/ms 7.89, stream/drive/ms 3.12, stream/tick/ms 1.59, render/present/ms 1.38, stream/spawn/ms 0.81, terrain/update/ms 0.65, terrain/select/ms 0.54, bigspace/total/ms 0.44, render/acquire/ms 0.18 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\fix_on\balmora_walk@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **76%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.8 W · 69 °C (max 74) · VRAM 3715 MiB · throttled 100% of samples
