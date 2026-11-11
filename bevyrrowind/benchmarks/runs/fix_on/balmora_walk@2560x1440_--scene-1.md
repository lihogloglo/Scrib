# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5773 frames over 65.6 s · **median 11.09 ms (90 fps)** · mean 11.37 · p95 13.58 · p99 15.11 (1% low 66 fps) · worst 24.99 ms · 23 spikes over 1.5× median · 12.8 stutters/min (work 23.8/min)
- warm-up: 361 frames over 4.6 s, worst 642.5 ms
- hitches (> 27.7 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.29 ms of a 11.37 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.09 ms, limit 11 ms
- **FAIL** frame_ms: 14 stutters (> 1.5x moving median of 60) in 1.09 min = 12.80/min, limit 1/min
- pass worst frame 24.99 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.076 | 2.101 | 0.175 |
| main_opaque_pass_3d | 1.390 | 1.714 | 0.050 |
| early prepass | 0.524 | 0.765 | 0.077 |
| lightprobe_irradiance_map | 0.443 | 0.766 | 0.001 |
| bloom | 0.319 | 0.326 | 0.063 |
| atmosphere_luts | 0.310 | 0.369 | 0.003 |
| lightprobe_radiance_map | 0.224 | 0.236 | 0.004 |
| ssao | 0.208 | 0.209 | 0.007 |
| uniform_allocation | 0.154 | 0.162 | 0.018 |
| auxiliary_views | 0.143 | 0.587 | 0.132 |
| clouds_trace | 0.128 | 0.221 | 0.001 |
| water_fft | 0.125 | 0.125 | 0.002 |
| upscaling | 0.120 | 0.134 | 0.004 |
| bin_unpacking | 0.115 | 0.125 | 0.014 |
| atmos_froxel_inject | 0.092 | 0.100 | 0.002 |
| gpu_instance_materialization | 0.092 | 0.096 | 0.004 |
| early_mesh_preprocessing | 0.090 | 0.098 | 0.004 |
| tonemapping | 0.087 | 0.090 | 0.003 |
| water_velocity | 0.081 | 0.085 | 0.001 |
| clustering | 0.066 | 0.068 | 0.018 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| main_transparent_pass_3d | 0.056 | 0.101 | 0.079 |
| render_sky | 0.052 | 0.053 | 0.002 |
| per_view_shadow_pass | 0.036 | 0.038 | 0.008 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| range_unpacking | 0.028 | 0.033 | 0.004 |
| atmos_froxel_integrate | 0.025 | 0.028 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
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
| pipelines/wait_ms | 19.374 | 0.000 | 10553.53 | 111847.4 |
| render/thread/ms | 9.766 | 11.922 | 22.35 | 56380.3 |
| render/busy/ms | 9.256 | 11.381 | 21.70 | 53433.1 |
| main/busy/ms | 8.153 | 9.929 | 19.87 | 47068.7 |
| render/graph/ms | 4.582 | 5.808 | 13.89 | 26453.7 |
| render/present/ms | 0.424 | 0.510 | 2.21 | 2446.4 |
| bigspace/total/ms | 0.402 | 0.593 | 2.71 | 2319.6 |
| pipelines/compile_ms | 0.288 | 0.000 | 161.97 | 1662.7 |
| terrain/update/ms | 0.283 | 0.362 | 1.77 | 1632.8 |
| pipelines/front_ms | 0.277 | 0.000 | 155.37 | 1599.0 |
| stream/drive/ms | 0.266 | 0.438 | 2.30 | 1535.5 |
| controller/feed/ms | 0.255 | 0.334 | 0.74 | 1471.8 |
| terrain/select/ms | 0.248 | 0.317 | 0.67 | 1433.1 |
| far/select/ms | 0.215 | 0.359 | 0.58 | 1240.3 |
| stream/tick/ms | 0.212 | 0.361 | 0.70 | 1224.4 |
| anim/evaluate_cpu/ms | 0.175 | 0.222 | 0.61 | 1012.8 |
| bigspace/low_precision/ms | 0.156 | 0.242 | 1.70 | 902.8 |
| anim/evaluate/ms | 0.148 | 0.310 | 1.30 | 857.1 |
| controller/move/ms | 0.141 | 0.317 | 0.73 | 811.5 |
| bigspace/high_precision/ms | 0.096 | 0.139 | 2.27 | 556.9 |
| anim/apply/ms | 0.087 | 0.163 | 2.26 | 501.4 |
| render/acquire/ms | 0.087 | 0.114 | 0.36 | 500.8 |
| physics/step/ms | 0.050 | 0.088 | 0.93 | 289.2 |
| physics/insert/ms | 0.046 | 0.415 | 1.10 | 267.5 |
| weather/map/ms | 0.027 | 0.035 | 0.24 | 156.8 |
| controller/avoid/ms | 0.011 | 0.017 | 0.09 | 64.1 |
| pipelines/driver_ms | 0.011 | 0.000 | 7.86 | 63.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 61.8 |
| script/update/ms | 0.007 | 0.013 | 0.14 | 41.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.05 | 33.0 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 28.5 |
| script/ui/ms | 0.005 | 0.008 | 0.03 | 26.4 |
| controller/write/ms | 0.004 | 0.006 | 0.28 | 21.0 |
| stream/spawn/ms | 0.003 | 0.000 | 1.09 | 19.3 |
| stream/upload/ms | 0.003 | 0.000 | 1.02 | 19.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 16.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 17 | 0 |
| anim/a2/count | count | 0 | 16 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 64 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
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
| frame/work/ms | ms | 8.99 | 21.70 | 8.64 |
| gpu/timed/ms | ms | 7.33 | 8.99 | 7.69 |
| mesh_allocator_allocations |  meshes | 10.8 k | 11.8 k | 10.8 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 311.90 M | 311.90 M | 311.90 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d/items | count | 330 | 588 | 516 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d_prepass/items | count | 330 | 588 | 516 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 760 | 973 | 757 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 5881 | 5881 |
| phases/opaque3d/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d/items | count | 2521 | 3605 | 2450 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 760 | 973 | 757 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 5881 | 5881 |
| phases/opaque3d_prepass/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d_prepass/items | count | 2521 | 3605 | 2450 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 49 | 222 | 40 |
| phases/shadow/bins | count | 1618 | 2523 | 938 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 19.8 k | 19.8 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5138 | 6734 | 3835 |
| phases/shadow/pipelines | count | 101 | 228 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2654 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 1861 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 170 | 171 | 171 |
| pipelines/waiting | count | 0 | 30 | 0 |
| preprocess/bind_groups | count | 186 | 571 | 167 |
| preprocess/bind_groups_created | count | 179 | 564 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 23.56 | 36.06 | 18.75 |
| process/mem_usage | GiB | 2.95 | 3.00 | 3.00 |
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
| started_load_count |  | 5029 | 5181 | 5181 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 237 | 333 | 225 |
| statics/materials_bound | count | 237 | 333 | 225 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.05 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 290 | 380 | 265 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.84 | 44.73 | 29.96 |
| system/mem_usage | % | 43.46 | 43.89 | 43.84 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 99 | 268 | 90 |
| views/all/bins | count | 3489 | 4494 | 3212 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.0 k | 33.0 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 10.8 k | 14.0 k | 9767 |
| views/all/pipelines | count | 139 | 268 | 68 |
| views/all/pipelines_distinct | count | 60 | 66 | 53 |
| views/all/work_items | count | 256.3 k | 259.1 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1421 | 1839 | 938 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 19.8 k | 19.8 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4750 | 5894 | 3835 |
| views/cascade/pipelines | count | 38 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.3 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 198 | 777 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 351 | 1152 | 0 |
| views/lamp_face/pipelines | count | 60 | 185 | 0 |
| views/lamp_face/work_items | count | 0 | 966 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2024 | 2318 | 2274 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.9 k | 13.2 k | 13.2 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5950 | 7630 | 5932 |
| views/main/pipelines | count | 38 | 42 | 38 |
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
| anim/bones_written/count | count | 56.56 | 842 | 326.5 k |
| anim/events/count | count | 0.26 | 4 | 1519 |
| assets/Image/added | count | 0.08 | 45 | 472 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.63 | 240 | 3651 |
| assets/Mesh/removed | count | 0.85 | 1242 | 4910 |
| assets/StandardMaterial/added | count | 0.12 | 47 | 688 |
| assets/StandardMaterial/modified | count | 3.75 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.19 | 3570 | 18.4 k |
| physics/inserts | count | 0.69 | 132 | 3956 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 8 |
| scene/overflow/cells | count | 0.00 | 1 | 6 |
| scene/slots/overflow | count | 0.08 | 190 | 461 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5864 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1286 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.64 | 642.5 | main/busy/ms 621, surface/advance_cpu_ms 118, surface/cell_upload_ms 0.69, stream/drive/ms 0.35, weather/map/ms 0.16, stream/tick/ms 0.11, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 1.04 | 400.1 | main/busy/ms 655, render/thread/ms 366, render/busy/ms 293, surface/advance_cpu_ms 118, render/acquire/ms 69.08, render/graph/ms 6.24, render/present/ms 4.00, stream/drive/ms 0.98, surface/cell_upload_ms 0.69, terrain/update/ms 0.54, stream/tick/ms 0.49, bigspace/total/ms 0.29 |
| warm-up | 1.31 | 83.1 | pipelines/wait_ms 379, pipelines/compile_ms 205, pipelines/front_ms 190, main/busy/ms 71.42, render/thread/ms 65.52, render/busy/ms 62.21, pipelines/driver_ms 14.91, render/graph/ms 4.22, stream/drive/ms 3.37, stream/tick/ms 2.29, render/acquire/ms 1.96, render/present/ms 1.35 |
| warm-up | 1.58 | 28.3 | render/thread/ms 38.15, render/busy/ms 36.77, main/busy/ms 25.23, render/graph/ms 12.40, stream/drive/ms 5.20, stream/spawn/ms 1.26, render/present/ms 1.24, stream/upload/ms 1.07, stream/tick/ms 0.68, terrain/update/ms 0.55, terrain/select/ms 0.46, bigspace/total/ms 0.26 |
| warm-up | 1.97 | 27.9 | main/busy/ms 33.30, render/thread/ms 16.34, render/busy/ms 15.12, render/graph/ms 3.75, stream/drive/ms 3.14, render/present/ms 0.99, stream/tick/ms 0.91, stream/spawn/ms 0.74, terrain/update/ms 0.62, terrain/select/ms 0.51, bigspace/total/ms 0.42, anim/evaluate_cpu/ms 0.30 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\fix_on\balmora_walk@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **76%** (min 58%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.2 W · 69 °C (max 74) · VRAM 3713 MiB · throttled 100% of samples
