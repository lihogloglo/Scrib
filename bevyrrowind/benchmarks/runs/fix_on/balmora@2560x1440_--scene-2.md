# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1577 frames over 15.0 s · **median 9.45 ms (106 fps)** · mean 9.51 · p95 10.64 · p99 11.68 (1% low 86 fps) · worst 14.77 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 315 frames over 4.3 s, worst 581.7 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.28 ms of a 9.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.45 ms, limit 11 ms
- pass worst frame 14.77 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.240 | 2.552 | 0.046 |
| dlss_super_resolution | 2.072 | 2.116 | 0.182 |
| early prepass | 0.644 | 0.972 | 0.082 |
| lightprobe_irradiance_map | 0.427 | 0.419 | 0.001 |
| atmosphere_luts | 0.418 | 0.685 | 0.003 |
| bloom | 0.321 | 0.337 | 0.063 |
| ssao | 0.238 | 0.544 | 0.007 |
| lightprobe_radiance_map | 0.224 | 0.233 | 0.004 |
| main_transparent_pass_3d | 0.182 | 0.191 | 0.043 |
| water_fft | 0.138 | 0.269 | 0.003 |
| clouds_trace | 0.136 | 0.157 | 0.001 |
| uniform_allocation | 0.130 | 0.133 | 0.014 |
| upscaling | 0.117 | 0.130 | 0.004 |
| bin_unpacking | 0.100 | 0.102 | 0.010 |
| atmos_froxel_inject | 0.088 | 0.091 | 0.001 |
| tonemapping | 0.088 | 0.095 | 0.003 |
| early_mesh_preprocessing | 0.081 | 0.089 | 0.003 |
| gpu_instance_materialization | 0.080 | 0.093 | 0.004 |
| water_velocity | 0.075 | 0.076 | 0.001 |
| clustering | 0.068 | 0.078 | 0.021 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.041 | 0.044 | 0.008 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| range_unpacking | 0.033 | 0.034 | 0.003 |
| water_mips | 0.026 | 0.024 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.027 | 0.001 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.012 | 0.014 | 0.003 |
| clouds_fog_light | 0.011 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.009 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.002 | 0.002 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 30.227 | 0.000 | 3842.57 | 47667.5 |
| render/thread/ms | 8.323 | 9.414 | 13.48 | 13125.7 |
| render/busy/ms | 7.344 | 8.435 | 10.98 | 11582.2 |
| main/busy/ms | 6.769 | 7.616 | 9.37 | 10674.4 |
| render/graph/ms | 3.777 | 4.825 | 6.92 | 5956.2 |
| pipelines/compile_ms | 0.748 | 0.000 | 130.49 | 1179.0 |
| pipelines/front_ms | 0.723 | 0.000 | 125.80 | 1140.8 |
| render/acquire/ms | 0.515 | 1.439 | 2.47 | 811.8 |
| render/present/ms | 0.464 | 0.597 | 3.63 | 731.7 |
| bigspace/total/ms | 0.314 | 0.612 | 1.42 | 495.7 |
| terrain/update/ms | 0.229 | 0.293 | 0.58 | 361.1 |
| stream/drive/ms | 0.216 | 0.389 | 0.71 | 340.2 |
| terrain/select/ms | 0.197 | 0.251 | 0.54 | 311.0 |
| stream/tick/ms | 0.185 | 0.341 | 0.62 | 291.6 |
| far/select/ms | 0.179 | 0.308 | 0.52 | 282.5 |
| controller/feed/ms | 0.153 | 0.211 | 0.44 | 240.8 |
| controller/move/ms | 0.130 | 0.274 | 0.49 | 205.1 |
| bigspace/high_precision/ms | 0.108 | 0.248 | 1.09 | 170.8 |
| anim/evaluate/ms | 0.090 | 0.234 | 1.00 | 141.8 |
| anim/apply/ms | 0.082 | 0.208 | 1.56 | 128.6 |
| bigspace/low_precision/ms | 0.079 | 0.170 | 1.19 | 124.1 |
| anim/evaluate_cpu/ms | 0.059 | 0.076 | 0.38 | 93.8 |
| physics/step/ms | 0.029 | 0.057 | 1.10 | 46.4 |
| weather/map/ms | 0.027 | 0.036 | 0.23 | 42.1 |
| pipelines/driver_ms | 0.024 | 0.000 | 4.69 | 38.2 |
| weather/eval/ms | 0.012 | 0.017 | 0.30 | 18.8 |
| script/update/ms | 0.008 | 0.018 | 0.12 | 12.9 |
| controller/avoid/ms | 0.007 | 0.010 | 0.19 | 10.5 |
| script/ui/ms | 0.005 | 0.008 | 0.15 | 8.5 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 7.4 |
| physics/insert/ms | 0.004 | 0.000 | 1.16 | 6.0 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 1.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 13 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.33 | 10.98 | 8.35 |
| gpu/timed/ms | ms | 8.31 | 9.04 | 8.35 |
| mesh_allocator_allocations |  meshes | 9463 | 9463 | 9463 |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 275.13 M | 275.13 M | 275.13 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d_prepass/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d/bins | count | 877 | 877 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 877 | 877 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 859 | 859 | 858 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2848 | 2848 | 2847 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 488 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 20 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 26.32 | 39.01 | 30.77 |
| process/mem_usage | GiB | 2.47 | 2.56 | 2.56 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 16.4 k | 16.4 k | 16.4 k |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2389 | 2389 | 2389 |
| scene/batch/masked/live | count | 2389 | 2389 | 2389 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1642 | 1642 | 1642 |
| scene/chains/live | count | 1642 | 1642 | 1642 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 4192 | 4194 | 4194 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.67 | 41.29 | 27.27 |
| system/mem_usage | % | 42.04 | 42.21 | 42.20 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 88 | 84 |
| views/all/bins | count | 3111 | 3111 | 3110 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9108 | 9108 | 9107 |
| views/all/pipelines | count | 65 | 69 | 65 |
| views/all/pipelines_distinct | count | 53 | 57 | 53 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 859 | 859 | 858 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2848 | 2848 | 2847 |
| views/cascade/pipelines | count | 27 | 30 | 27 |
| views/cascade/work_items | count | 150.3 k | 150.3 k | 150.3 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2252 | 2252 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 38 | 42 | 38 |
| views/main/work_items | count | 104.7 k | 104.7 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.0 k | 15.0 k | 15.0 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 4 | 145 |
| assets/Image/added | count | 0.01 | 16 | 17 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.07 | 92 | 109 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 16 | 17 |
| assets/StandardMaterial/modified | count | 3.14 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.60 | 500 | 940 |
| physics/inserts | count | 0.31 | 119 | 488 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 3 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.7 | main/busy/ms 559, surface/advance_cpu_ms 65.97, surface/cell_upload_ms 1.18, stream/drive/ms 0.32, weather/map/ms 0.11, bigspace/total/ms 0.11, stream/tick/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 1.06 | 473.9 | main/busy/ms 590, render/thread/ms 439, render/busy/ms 360, render/acquire/ms 76.73, surface/advance_cpu_ms 65.97, render/graph/ms 4.65, render/present/ms 2.16, stream/drive/ms 1.20, surface/cell_upload_ms 1.18, stream/tick/ms 0.73, terrain/update/ms 0.51, terrain/select/ms 0.35 |
| warm-up | 1.30 | 76.1 | pipelines/wait_ms 203, pipelines/compile_ms 113, pipelines/front_ms 105, main/busy/ms 58.13, render/thread/ms 50.81, render/busy/ms 48.69, pipelines/driver_ms 8.29, render/graph/ms 4.57, render/present/ms 1.78, stream/drive/ms 1.16, stream/tick/ms 0.95, terrain/update/ms 0.52 |
| warm-up | 1.34 | 25.4 | render/thread/ms 33.88, render/busy/ms 32.56, main/busy/ms 18.58, stream/drive/ms 3.70, render/graph/ms 3.44, stream/tick/ms 1.73, stream/spawn/ms 1.09, render/present/ms 1.05, terrain/update/ms 0.59, terrain/select/ms 0.44, render/acquire/ms 0.27, stream/upload/ms 0.23 |
| warm-up | 2.39 | 30.7 | main/busy/ms 39.54, render/thread/ms 27.92, render/busy/ms 26.72, render/graph/ms 4.30, stream/drive/ms 2.31, stream/upload/ms 1.13, render/present/ms 1.01, physics/insert/ms 0.57, bigspace/total/ms 0.57, stream/tick/ms 0.54, terrain/update/ms 0.48, terrain/select/ms 0.40 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\fix_on\balmora@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 91%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.5 W · 72 °C (max 74) · VRAM 3397 MiB · throttled 100% of samples
