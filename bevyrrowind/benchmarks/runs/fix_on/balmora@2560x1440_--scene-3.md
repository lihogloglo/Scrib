# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1578 frames over 15.0 s · **median 9.44 ms (106 fps)** · mean 9.51 · p95 10.52 · p99 11.43 (1% low 87 fps) · worst 16.68 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 319 frames over 4.1 s, worst 584.4 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.30 ms of a 9.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.44 ms, limit 11 ms
- pass worst frame 16.68 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.257 | 2.585 | 0.047 |
| dlss_super_resolution | 2.071 | 2.102 | 0.178 |
| early prepass | 0.634 | 0.962 | 0.081 |
| lightprobe_irradiance_map | 0.434 | 0.681 | 0.001 |
| atmosphere_luts | 0.406 | 0.677 | 0.003 |
| bloom | 0.320 | 0.337 | 0.063 |
| ssao | 0.246 | 0.545 | 0.007 |
| lightprobe_radiance_map | 0.224 | 0.235 | 0.004 |
| main_transparent_pass_3d | 0.182 | 0.190 | 0.042 |
| water_fft | 0.144 | 0.282 | 0.003 |
| clouds_trace | 0.136 | 0.158 | 0.001 |
| uniform_allocation | 0.132 | 0.133 | 0.015 |
| upscaling | 0.118 | 0.126 | 0.004 |
| bin_unpacking | 0.101 | 0.102 | 0.011 |
| tonemapping | 0.088 | 0.094 | 0.003 |
| atmos_froxel_inject | 0.088 | 0.091 | 0.001 |
| early_mesh_preprocessing | 0.084 | 0.089 | 0.004 |
| gpu_instance_materialization | 0.081 | 0.093 | 0.004 |
| water_velocity | 0.076 | 0.076 | 0.002 |
| clustering | 0.073 | 0.078 | 0.021 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.041 | 0.044 | 0.008 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| range_unpacking | 0.034 | 0.036 | 0.004 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.027 | 0.001 |
| lightprobe_copy | 0.014 | 0.015 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.012 | 0.014 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 41.612 | 0.000 | 7845.59 | 65663.0 |
| render/thread/ms | 8.315 | 9.305 | 12.97 | 13121.6 |
| render/busy/ms | 7.330 | 8.374 | 12.40 | 11566.9 |
| main/busy/ms | 6.777 | 7.569 | 14.34 | 10693.5 |
| render/graph/ms | 3.783 | 4.745 | 6.31 | 5969.6 |
| pipelines/compile_ms | 0.901 | 0.000 | 148.16 | 1422.5 |
| pipelines/front_ms | 0.873 | 0.000 | 144.58 | 1377.5 |
| render/acquire/ms | 0.518 | 1.398 | 2.41 | 817.0 |
| render/present/ms | 0.468 | 0.594 | 2.88 | 737.7 |
| bigspace/total/ms | 0.306 | 0.549 | 1.72 | 482.2 |
| terrain/update/ms | 0.226 | 0.286 | 0.64 | 356.8 |
| stream/drive/ms | 0.213 | 0.370 | 0.73 | 335.4 |
| terrain/select/ms | 0.194 | 0.245 | 0.61 | 305.4 |
| far/select/ms | 0.183 | 0.318 | 0.71 | 288.0 |
| stream/tick/ms | 0.182 | 0.328 | 0.71 | 287.4 |
| controller/feed/ms | 0.129 | 0.192 | 0.43 | 203.8 |
| controller/move/ms | 0.120 | 0.270 | 0.59 | 189.5 |
| bigspace/high_precision/ms | 0.109 | 0.243 | 1.25 | 171.3 |
| anim/apply/ms | 0.079 | 0.194 | 0.83 | 124.0 |
| anim/evaluate/ms | 0.077 | 0.232 | 1.29 | 121.2 |
| bigspace/low_precision/ms | 0.069 | 0.124 | 1.52 | 108.7 |
| anim/evaluate_cpu/ms | 0.062 | 0.082 | 0.32 | 98.5 |
| physics/step/ms | 0.033 | 0.058 | 1.01 | 51.6 |
| pipelines/driver_ms | 0.029 | 0.000 | 4.42 | 45.1 |
| weather/map/ms | 0.026 | 0.036 | 0.15 | 40.7 |
| physics/insert/ms | 0.015 | 0.073 | 0.92 | 24.4 |
| weather/eval/ms | 0.011 | 0.016 | 0.05 | 18.1 |
| script/update/ms | 0.008 | 0.017 | 0.14 | 12.6 |
| controller/avoid/ms | 0.006 | 0.010 | 0.23 | 10.1 |
| script/ui/ms | 0.005 | 0.008 | 0.13 | 7.6 |
| script/input/ms | 0.004 | 0.005 | 0.02 | 7.0 |
| controller/write/ms | 0.002 | 0.003 | 0.01 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 1.8 |
| shapes/activate/ms | 0.001 | 0.002 | 0.08 | 1.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| anim/a2/count | count | 0 | 6 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 37 | 7 |
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
| frame/work/ms | ms | 8.35 | 14.34 | 8.59 |
| gpu/timed/ms | ms | 8.33 | 9.10 | 8.59 |
| mesh_allocator_allocations |  meshes | 9463 | 9463 | 9463 |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 309.15 M | 309.15 M | 309.15 M |
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
| phases/opaque3d/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d/bins | count | 877 | 878 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 877 | 878 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 859 | 859 | 859 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2848 | 2848 | 2848 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 2366 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 25.96 | 40.67 | 15.21 |
| process/mem_usage | GiB | 2.46 | 2.55 | 2.55 |
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
| system/cpu_usage | % | 32.27 | 46.59 | 27.02 |
| system/mem_usage | % | 41.94 | 42.19 | 42.19 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3111 | 3113 | 3111 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9108 | 9108 | 9108 |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 55 | 53 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 859 | 859 | 859 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2848 | 2848 | 2848 |
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
| anim/bones_written/count | count | 0.10 | 165 | 165 |
| anim/events/count | count | 0.05 | 2 | 83 |
| assets/Image/added | count | 0.02 | 18 | 24 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.06 | 52 | 91 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 18 | 24 |
| assets/StandardMaterial/modified | count | 3.14 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.77 | 1620 | 2794 |
| physics/inserts | count | 1.27 | 145 | 2006 |
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
| warm-up | 0.58 | 584.4 | main/busy/ms 563, surface/advance_cpu_ms 67.43, surface/cell_upload_ms 0.69, stream/drive/ms 0.33, weather/map/ms 0.12, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.07, stream/tick/ms 0.06, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.89 | 304.5 | main/busy/ms 594, render/thread/ms 266, render/busy/ms 188, render/acquire/ms 76.15, surface/advance_cpu_ms 67.43, render/graph/ms 5.17, render/present/ms 2.15, stream/drive/ms 0.90, surface/cell_upload_ms 0.69, stream/tick/ms 0.46, terrain/update/ms 0.33, weather/map/ms 0.24 |
| warm-up | 1.00 | 108.7 | pipelines/wait_ms 1134, render/thread/ms 283, render/busy/ms 204, pipelines/compile_ms 101, pipelines/front_ms 92.68, render/acquire/ms 76.27, main/busy/ms 46.66, terrain/update/ms 16.22, render/graph/ms 10.03, pipelines/driver_ms 8.00, render/present/ms 3.44, stream/drive/ms 1.07 |
| warm-up | 1.24 | 96.9 | pipelines/compile_ms 102, pipelines/front_ms 93.42, main/busy/ms 77.65, pipelines/wait_ms 55.64, render/thread/ms 52.59, render/busy/ms 50.70, pipelines/driver_ms 8.33, terrain/update/ms 7.00, render/graph/ms 4.58, render/present/ms 1.56, stream/drive/ms 1.49, stream/tick/ms 1.19 |
| warm-up | 2.61 | 25.4 | main/busy/ms 40.28, render/thread/ms 17.03, render/busy/ms 15.91, render/graph/ms 4.71, stream/drive/ms 1.10, render/present/ms 0.95, bigspace/total/ms 0.69, anim/evaluate_cpu/ms 0.55, stream/tick/ms 0.51, terrain/update/ms 0.45, terrain/select/ms 0.37, anim/evaluate/ms 0.33 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\fix_on\balmora@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 89%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.9 W · 72 °C (max 74) · VRAM 3653 MiB · throttled 100% of samples
