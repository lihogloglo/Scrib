# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9031 frames over 65.6 s · **median 7.25 ms (138 fps)** · mean 7.27 · p95 8.32 · p99 9.07 (1% low 110 fps) · worst 29.51 ms · 16 spikes over 1.5× median · 16.5 stutters/min (work 6.4/min)
- warm-up: 328 frames over 3.3 s, worst 583.6 ms
- hitches (> 25.0 ms): **2** measured, 4 in warm-up
- timed GPU passes: 6.76 ms of a 7.27 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.25 ms, limit 11 ms
- **FAIL** frame_ms: 18 stutters (> 1.5x moving median of 60) in 1.09 min = 16.45/min, limit 1/min
- pass worst frame 29.51 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.229 | 2.731 | 0.183 |
| main_opaque_pass_3d | 1.312 | 1.929 | 0.019 |
| lightprobe_irradiance_map | 0.444 | 0.753 | 0.001 |
| bloom | 0.344 | 0.650 | 0.060 |
| early prepass | 0.343 | 0.589 | 0.034 |
| atmosphere_luts | 0.306 | 0.439 | 0.003 |
| lightprobe_radiance_map | 0.240 | 0.242 | 0.003 |
| ssao | 0.221 | 0.219 | 0.007 |
| clouds_trace | 0.149 | 0.245 | 0.001 |
| water_fft | 0.132 | 0.128 | 0.002 |
| atmos_froxel_inject | 0.100 | 0.104 | 0.001 |
| upscaling | 0.092 | 0.121 | 0.004 |
| tonemapping | 0.091 | 0.091 | 0.003 |
| water_velocity | 0.088 | 0.086 | 0.001 |
| main_transparent_pass_3d | 0.083 | 0.259 | 0.013 |
| clustering | 0.068 | 0.074 | 0.020 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| surface_trail | 0.060 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| uniform_allocation | 0.042 | 0.050 | 0.007 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| bin_unpacking | 0.029 | 0.030 | 0.006 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| early_mesh_preprocessing | 0.024 | 0.025 | 0.004 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.030 | 0.006 |
| lightprobe_copy | 0.015 | 0.017 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.537 | 7.580 | 28.48 | 59034.5 |
| render/busy/ms | 4.851 | 5.799 | 13.15 | 43805.0 |
| main/busy/ms | 4.764 | 5.532 | 11.53 | 43023.1 |
| render/graph/ms | 2.760 | 3.576 | 11.05 | 24924.8 |
| render/acquire/ms | 1.238 | 2.329 | 3.57 | 11182.8 |
| render/present/ms | 0.448 | 0.553 | 21.44 | 4046.7 |
| terrain/update/ms | 0.298 | 0.378 | 1.71 | 2695.1 |
| terrain/select/ms | 0.263 | 0.333 | 0.75 | 2376.0 |
| bigspace/total/ms | 0.244 | 0.537 | 1.93 | 2206.5 |
| stream/drive/ms | 0.221 | 0.318 | 2.49 | 1992.4 |
| stream/tick/ms | 0.187 | 0.266 | 0.72 | 1685.8 |
| far/select/ms | 0.168 | 0.235 | 0.59 | 1519.2 |
| bigspace/high_precision/ms | 0.125 | 0.298 | 1.48 | 1131.2 |
| bigspace/low_precision/ms | 0.052 | 0.181 | 1.08 | 473.6 |
| weather/map/ms | 0.026 | 0.034 | 0.34 | 231.3 |
| physics/insert/ms | 0.021 | 0.221 | 0.91 | 193.0 |
| physics/step/ms | 0.012 | 0.028 | 0.68 | 104.2 |
| weather/eval/ms | 0.010 | 0.015 | 0.34 | 94.5 |
| script/update/ms | 0.006 | 0.009 | 0.22 | 56.2 |
| script/input/ms | 0.005 | 0.006 | 0.09 | 42.4 |
| script/ui/ms | 0.004 | 0.006 | 0.17 | 33.4 |
| controller/move/ms | 0.002 | 0.003 | 0.07 | 18.9 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.10 | 16.3 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.60 | 12.6 |
| anim/apply/ms | 0.001 | 0.002 | 0.03 | 12.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 8.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.09 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 4.9 |
| stream/spawn/ms | 0.000 | 0.000 | 0.17 | 3.5 |
| stream/despawn/ms | 0.000 | 0.000 | 0.17 | 2.7 |
| controller/feed/ms | 0.000 | 0.001 | 0.06 | 2.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.7 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| ecs/entities | count | 8059 | 8412 | 7944 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 6.79 | 13.15 | 6.84 |
| gpu/timed/ms | ms | 6.79 | 8.37 | 6.84 |
| mesh_allocator_allocations |  meshes | 9036 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 307.48 M | 308.53 M | 307.48 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 127 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 127 | 368 | 361 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 127 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 127 | 368 | 361 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 148 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 328 | 617 | 603 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 148 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 328 | 617 | 603 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/shadow/batch_sets | count | 5 | 7 | 6 |
| phases/shadow/bins | count | 85 | 108 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 575 | 615 | 572 |
| phases/shadow/pipelines | count | 5 | 7 | 6 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2711 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 194 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 101 | 101 | 101 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 54 | 58 | 56 |
| preprocess/bind_groups_created | count | 54 | 58 | 56 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 22.70 | 45.19 | 29.41 |
| process/mem_usage | GiB | 2.14 | 2.19 | 2.15 |
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
| started_load_count |  | 3012 | 3046 | 3046 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 755 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.13 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.35 | 58.64 | 25.09 |
| system/mem_usage | % | 48.56 | 49.03 | 48.95 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 639 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1507 | 2536 | 2500 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1610 | 2746 | 2701 |
| views/cascade/batch_sets | count | 5 | 7 | 6 |
| views/cascade/bins | count | 85 | 108 | 75 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 575 | 615 | 572 |
| views/cascade/pipelines | count | 5 | 7 | 6 |
| views/cascade/work_items | count | 575 | 615 | 572 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 12 | 12 | 12 |
| views/main/bins | count | 548 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 910 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1013 | 2172 | 2129 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 3200 | 3212 | 3203 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0.01 | 18 | 125 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.23 | 195 | 2087 |
| assets/Mesh/removed | count | 0.49 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.27 | 111 | 2436 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.26 | 161 | 2335 |
| physics/steps | count | 0.44 | 1 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.04 | 27 | 351 |
| stream/upload/bytes | bytes | 3748 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 822 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 583.6 | main/busy/ms 561, surface/advance_cpu_ms 66.28, surface/cell_upload_ms 0.86, stream/drive/ms 0.54, weather/map/ms 0.15, bigspace/total/ms 0.15, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.92 | 332.9 | main/busy/ms 596, render/thread/ms 297, render/busy/ms 226, render/acquire/ms 68.57, surface/advance_cpu_ms 66.28, render/graph/ms 4.77, render/present/ms 2.12, stream/drive/ms 1.27, surface/cell_upload_ms 0.86, terrain/update/ms 0.57, stream/tick/ms 0.48, terrain/select/ms 0.39 |
| warm-up | 0.99 | 75.5 | pipelines/wait_ms 828, render/thread/ms 316, render/busy/ms 244, pipelines/compile_ms 70.00, render/acquire/ms 68.83, pipelines/front_ms 64.20, main/busy/ms 52.25, terrain/update/ms 20.48, render/graph/ms 9.92, pipelines/driver_ms 5.80, render/present/ms 3.13, stream/drive/ms 1.14 |
| warm-up | 1.18 | 84.6 | main/busy/ms 59.12, render/thread/ms 49.55, render/busy/ms 47.97, render/graph/ms 3.53, stream/drive/ms 1.62, render/present/ms 1.21, stream/tick/ms 0.93, terrain/update/ms 0.90, terrain/select/ms 0.74, render/acquire/ms 0.38, bigspace/total/ms 0.29, stream/spawn/ms 0.28 |
| measure | 2.62 | 29.5 | render/thread/ms 35.07, render/present/ms 22.13, render/busy/ms 12.75, main/busy/ms 11.16, render/graph/ms 7.23, stream/drive/ms 0.81, bigspace/total/ms 0.79, stream/tick/ms 0.72, bigspace/high_precision/ms 0.61, terrain/update/ms 0.54, terrain/select/ms 0.48, far/select/ms 0.40 |
| measure | 52.60 | 27.6 | render/thread/ms 33.72, render/present/ms 20.76, render/busy/ms 10.53, main/busy/ms 10.03, render/graph/ms 6.09, render/acquire/ms 2.44, bigspace/total/ms 0.62, terrain/update/ms 0.61, terrain/select/ms 0.55, far/select/ms 0.43, bigspace/high_precision/ms 0.40, stream/drive/ms 0.37 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_statics_off\balmora_walk@2560x1440_off=statics-2 --bench-baseline none --off statics
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **97%** (min 84%) · 2580 MHz (min 2580) · memory 8001 MHz · 100.3 W · 78 °C (max 81) · VRAM 3436 MiB · throttled 100% of samples
