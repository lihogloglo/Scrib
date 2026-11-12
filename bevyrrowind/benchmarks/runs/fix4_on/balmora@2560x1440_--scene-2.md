# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1547 frames over 15.0 s · **median 9.60 ms (104 fps)** · mean 9.70 · p95 10.75 · p99 11.66 (1% low 86 fps) · worst 26.26 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 447 frames over 5.1 s, worst 585.6 ms
- hitches (> 25.0 ms): **1** measured, 4 in warm-up
- timed GPU passes: 8.53 ms of a 9.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.60 ms, limit 11 ms
- pass worst frame 26.26 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.314 | 2.742 | 0.046 |
| dlss_super_resolution | 2.196 | 2.574 | 0.181 |
| early prepass | 0.653 | 0.975 | 0.082 |
| lightprobe_irradiance_map | 0.444 | 0.754 | 0.001 |
| atmosphere_luts | 0.368 | 0.663 | 0.003 |
| bloom | 0.341 | 0.505 | 0.064 |
| lightprobe_radiance_map | 0.241 | 0.475 | 0.004 |
| ssao | 0.215 | 0.215 | 0.007 |
| main_transparent_pass_3d | 0.190 | 0.222 | 0.044 |
| clouds_trace | 0.137 | 0.157 | 0.001 |
| uniform_allocation | 0.135 | 0.133 | 0.013 |
| water_fft | 0.130 | 0.127 | 0.003 |
| upscaling | 0.125 | 0.132 | 0.004 |
| bin_unpacking | 0.105 | 0.103 | 0.010 |
| atmos_froxel_inject | 0.096 | 0.095 | 0.001 |
| tonemapping | 0.092 | 0.096 | 0.003 |
| early_mesh_preprocessing | 0.091 | 0.091 | 0.003 |
| gpu_instance_materialization | 0.089 | 0.094 | 0.004 |
| water_velocity | 0.084 | 0.079 | 0.001 |
| clustering | 0.069 | 0.078 | 0.021 |
| clouds_composite | 0.067 | 0.066 | 0.002 |
| render_sky | 0.056 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.039 | 0.044 | 0.008 |
| clouds_resolve | 0.036 | 0.042 | 0.001 |
| range_unpacking | 0.036 | 0.038 | 0.003 |
| clouds_shadow | 0.032 | 0.036 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| lightprobe_copy | 0.014 | 0.015 | 0.005 |
| lightprobe_downsampling_first_pass | 0.014 | 0.015 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.009 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.474 | 9.498 | 24.71 | 13110.0 |
| pipelines/wait_ms | 7.577 | 0.000 | 3656.95 | 11721.0 |
| render/busy/ms | 7.478 | 8.557 | 10.81 | 11567.7 |
| main/busy/ms | 6.791 | 7.638 | 11.16 | 10505.1 |
| render/graph/ms | 3.892 | 4.775 | 6.65 | 6020.2 |
| render/acquire/ms | 0.525 | 1.433 | 2.61 | 811.5 |
| render/present/ms | 0.472 | 0.585 | 16.84 | 730.8 |
| bigspace/total/ms | 0.298 | 0.518 | 1.62 | 461.5 |
| terrain/update/ms | 0.229 | 0.285 | 0.60 | 354.5 |
| stream/drive/ms | 0.208 | 0.364 | 0.65 | 321.1 |
| terrain/select/ms | 0.196 | 0.242 | 0.57 | 303.8 |
| far/select/ms | 0.183 | 0.317 | 0.65 | 283.0 |
| stream/tick/ms | 0.177 | 0.319 | 0.57 | 274.2 |
| pipelines/compile_ms | 0.172 | 0.000 | 139.44 | 265.8 |
| pipelines/front_ms | 0.166 | 0.000 | 135.30 | 257.0 |
| controller/feed/ms | 0.126 | 0.183 | 0.40 | 195.2 |
| bigspace/high_precision/ms | 0.101 | 0.217 | 1.41 | 157.0 |
| controller/move/ms | 0.100 | 0.177 | 0.51 | 154.8 |
| anim/evaluate/ms | 0.096 | 0.268 | 0.94 | 148.5 |
| anim/apply/ms | 0.075 | 0.204 | 0.99 | 116.0 |
| bigspace/low_precision/ms | 0.072 | 0.135 | 0.86 | 111.1 |
| anim/evaluate_cpu/ms | 0.063 | 0.080 | 0.40 | 97.1 |
| physics/step/ms | 0.030 | 0.057 | 0.59 | 45.7 |
| weather/map/ms | 0.027 | 0.041 | 0.30 | 42.4 |
| weather/eval/ms | 0.012 | 0.018 | 0.06 | 17.8 |
| script/update/ms | 0.008 | 0.017 | 0.20 | 12.4 |
| controller/avoid/ms | 0.007 | 0.010 | 0.28 | 10.5 |
| pipelines/driver_ms | 0.006 | 0.000 | 4.14 | 8.7 |
| physics/insert/ms | 0.005 | 0.000 | 0.80 | 8.5 |
| script/ui/ms | 0.005 | 0.008 | 0.02 | 7.6 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.3 |
| controller/write/ms | 0.002 | 0.003 | 0.04 | 3.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.7 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.2 |
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
| anim/a2/count | count | 0 | 5 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 26 | 7 |
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
| frame/work/ms | ms | 8.53 | 11.16 | 8.79 |
| gpu/timed/ms | ms | 8.52 | 9.76 | 8.79 |
| mesh_allocator_allocations |  meshes | 9463 | 9463 | 9463 |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 309.15 M | 309.15 M | 309.15 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d/bins | count | 877 | 878 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 877 | 878 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 858 | 872 | 858 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2847 | 2876 | 2847 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 691 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 4 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 23.08 | 37.95 | 30.99 |
| process/mem_usage | GiB | 2.11 | 2.18 | 2.18 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.29 | 53.49 | 29.26 |
| system/mem_usage | % | 48.36 | 48.59 | 48.59 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3110 | 3124 | 3110 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9107 | 9136 | 9107 |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 55 | 53 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 858 | 872 | 858 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2847 | 2876 | 2847 |
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
| views/main/batch_sets | count | 50 | 52 | 50 |
| views/main/bins | count | 2252 | 2254 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 38 | 40 | 38 |
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
| anim/bones_written/count | count | 0.02 | 33 | 33 |
| anim/events/count | count | 0.09 | 3 | 143 |
| assets/Image/added | count | 0.02 | 16 | 25 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.06 | 58 | 86 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 18 | 27 |
| assets/StandardMaterial/modified | count | 3.20 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.16 | 1308 | 1799 |
| physics/inserts | count | 0.24 | 73 | 375 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 585.6 | main/busy/ms 563, surface/advance_cpu_ms 65.41, surface/cell_upload_ms 0.72, stream/drive/ms 0.31, weather/map/ms 0.18, bigspace/total/ms 0.09, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.91 | 326.6 | main/busy/ms 596, render/thread/ms 287, render/busy/ms 217, render/acquire/ms 67.71, surface/advance_cpu_ms 65.41, render/graph/ms 6.47, render/present/ms 2.35, stream/drive/ms 0.83, surface/cell_upload_ms 0.72, terrain/update/ms 0.54, stream/tick/ms 0.40, terrain/select/ms 0.39 |
| warm-up | 1.00 | 86.6 | pipelines/wait_ms 478, render/thread/ms 305, render/busy/ms 233, pipelines/compile_ms 87.71, pipelines/front_ms 81.97, render/acquire/ms 68.00, main/busy/ms 50.25, terrain/update/ms 17.60, render/graph/ms 11.04, pipelines/driver_ms 5.74, render/present/ms 4.15, stream/drive/ms 0.88 |
| warm-up | 1.24 | 94.5 | pipelines/wait_ms 440, main/busy/ms 67.71, pipelines/compile_ms 50.77, pipelines/front_ms 46.93, render/thread/ms 46.23, render/busy/ms 44.60, pipelines/driver_ms 3.83, render/graph/ms 3.12, stream/drive/ms 1.61, stream/tick/ms 1.33, render/present/ms 1.30, terrain/update/ms 0.56 |
| measure | 0.59 | 26.3 | render/thread/ms 34.78, render/present/ms 17.31, render/busy/ms 17.30, main/busy/ms 15.84, render/graph/ms 8.98, stream/drive/ms 0.71, bigspace/total/ms 0.64, stream/tick/ms 0.63, terrain/update/ms 0.49, terrain/select/ms 0.43, far/select/ms 0.34, bigspace/high_precision/ms 0.31 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\fix4_on\balmora@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 5%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **97%** (min 95%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.8 W · 74 °C (max 76) · VRAM 3725 MiB · throttled 100% of samples
