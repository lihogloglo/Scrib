# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1599 frames over 15.0 s · **median 9.33 ms (107 fps)** · mean 9.38 · p95 10.39 · p99 10.87 (1% low 92 fps) · worst 13.10 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 480 frames over 5.4 s, worst 525.9 ms
- hitches (> 25.0 ms): **0** measured, 7 in warm-up
- timed GPU passes: 8.59 ms of a 9.38 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.33 ms, limit 11 ms
- pass worst frame 13.10 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.159 | 2.611 | 0.047 |
| dlss_super_resolution | 2.077 | 2.383 | 0.198 |
| atmosphere_luts | 1.080 | 1.321 | 0.003 |
| early prepass | 0.611 | 0.762 | 0.082 |
| lightprobe_irradiance_map | 0.426 | 0.559 | 0.001 |
| bloom | 0.323 | 0.338 | 0.072 |
| ssao | 0.258 | 0.606 | 0.008 |
| lightprobe_radiance_map | 0.225 | 0.234 | 0.004 |
| main_transparent_pass_3d | 0.177 | 0.186 | 0.122 |
| uniform_allocation | 0.147 | 0.154 | 0.027 |
| water_fft | 0.134 | 0.129 | 0.003 |
| upscaling | 0.114 | 0.130 | 0.005 |
| bin_unpacking | 0.099 | 0.105 | 0.020 |
| tonemapping | 0.088 | 0.091 | 0.004 |
| atmos_froxel_inject | 0.077 | 0.079 | 0.002 |
| water_velocity | 0.076 | 0.076 | 0.002 |
| clouds_trace | 0.067 | 0.078 | 0.001 |
| clustering | 0.065 | 0.078 | 0.023 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.044 | 0.006 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| water_mips | 0.034 | 0.029 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.022 | 0.026 | 0.001 |
| atmosphere_environment | 0.020 | 0.017 | 0.003 |
| per_view_shadow_pass | 0.018 | 0.022 | 0.005 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.007 | 0.008 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.006 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.342 | 9.358 | 11.99 | 13339.4 |
| pipelines/wait_ms | 6.809 | 0.000 | 4340.81 | 10887.5 |
| render/busy/ms | 6.775 | 7.963 | 10.16 | 10833.7 |
| main/busy/ms | 6.689 | 7.558 | 8.61 | 10695.4 |
| render/graph/ms | 3.727 | 4.705 | 6.38 | 5960.1 |
| render/acquire/ms | 1.112 | 2.173 | 3.43 | 1777.5 |
| render/present/ms | 0.455 | 0.565 | 2.32 | 728.2 |
| bigspace/total/ms | 0.388 | 0.731 | 1.83 | 620.6 |
| terrain/update/ms | 0.240 | 0.305 | 0.79 | 383.0 |
| terrain/select/ms | 0.208 | 0.262 | 0.75 | 332.1 |
| stream/drive/ms | 0.187 | 0.274 | 0.54 | 299.6 |
| far/select/ms | 0.170 | 0.260 | 0.57 | 272.1 |
| stream/tick/ms | 0.160 | 0.237 | 0.39 | 256.2 |
| controller/feed/ms | 0.137 | 0.196 | 0.33 | 219.5 |
| bigspace/high_precision/ms | 0.134 | 0.307 | 1.59 | 213.8 |
| controller/move/ms | 0.130 | 0.223 | 0.44 | 207.1 |
| pipelines/compile_ms | 0.092 | 0.000 | 133.55 | 147.2 |
| pipelines/front_ms | 0.089 | 0.000 | 129.46 | 141.9 |
| bigspace/low_precision/ms | 0.087 | 0.213 | 1.35 | 139.3 |
| anim/evaluate/ms | 0.078 | 0.201 | 1.17 | 124.9 |
| anim/evaluate_cpu/ms | 0.065 | 0.082 | 0.49 | 104.5 |
| physics/step/ms | 0.026 | 0.058 | 0.16 | 41.1 |
| weather/map/ms | 0.017 | 0.023 | 0.06 | 27.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 17.5 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 10.9 |
| controller/avoid/ms | 0.006 | 0.008 | 0.12 | 10.0 |
| script/ui/ms | 0.005 | 0.007 | 0.24 | 8.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.8 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 7.5 |
| pipelines/driver_ms | 0.003 | 0.000 | 4.10 | 5.3 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.54 | 10.16 | 8.52 |
| gpu/timed/ms | ms | 8.54 | 10.03 | 8.52 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 337 | 337 | 337 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3196 | 3196 | 3196 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 337 | 337 | 337 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3196 | 3196 | 3196 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d/bins | count | 3899 | 3899 | 3899 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9299 | 9299 | 9299 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d_prepass/bins | count | 3899 | 3899 | 3899 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9299 | 9299 | 9299 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 19 | 19 |
| phases/shadow/bins | count | 6714 | 6714 | 6714 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 20.0 k | 20.0 k | 20.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 19 | 19 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 145 | 145 | 145 |
| process/cpu_usage | % | 30.77 | 41.35 | 22.27 |
| process/mem_usage | GiB | 2.19 | 2.26 | 2.26 |
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
| started_load_count |  | 2595 | 2598 | 2598 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 388 | 388 | 388 |
| statics/materials_bound | count | 388 | 388 | 388 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.83 | 49.34 | 34.61 |
| system/mem_usage | % | 50.44 | 50.61 | 50.57 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 63 | 63 | 63 |
| views/all/bins | count | 15.2 k | 15.2 k | 15.2 k |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 45.0 k | 45.0 k | 45.0 k |
| views/all/pipelines | count | 59 | 59 | 59 |
| views/all/pipelines_distinct | count | 55 | 55 | 55 |
| views/all/work_items | count | 45.4 k | 45.4 k | 45.4 k |
| views/cascade/batch_sets | count | 19 | 19 | 19 |
| views/cascade/bins | count | 6714 | 6714 | 6714 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 20.0 k | 20.0 k | 20.0 k |
| views/cascade/pipelines | count | 19 | 19 | 19 |
| views/cascade/work_items | count | 20.0 k | 20.0 k | 20.0 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 44 | 44 |
| views/main/bins | count | 8472 | 8472 | 8472 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 25.0 k | 25.0 k | 25.0 k |
| views/main/pipelines | count | 40 | 40 | 40 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 3 | 140 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.10 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 1 | 900 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.53 | 525.9 | main/busy/ms 500, surface/advance_cpu_ms 0.73, stream/drive/ms 0.33, surface/cell_upload_ms 0.27, bigspace/total/ms 0.11, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.87 | 347.4 | main/busy/ms 536, render/thread/ms 308, render/busy/ms 236, render/acquire/ms 69.59, render/graph/ms 4.42, render/present/ms 1.97, stream/drive/ms 1.13, surface/advance_cpu_ms 0.73, stream/tick/ms 0.57, terrain/update/ms 0.50, bigspace/total/ms 0.35, terrain/select/ms 0.33 |
| warm-up | 0.92 | 45.1 | pipelines/wait_ms 664, render/thread/ms 318, render/busy/ms 245, render/acquire/ms 69.77, pipelines/compile_ms 58.99, pipelines/front_ms 52.49, main/busy/ms 51.15, terrain/update/ms 18.65, render/graph/ms 7.36, pipelines/driver_ms 6.49, render/present/ms 2.88, stream/drive/ms 1.27 |
| warm-up | 1.15 | 79.5 | main/busy/ms 61.19, render/thread/ms 49.75, render/busy/ms 48.41, render/graph/ms 3.67, render/present/ms 1.12, stream/drive/ms 1.00, terrain/update/ms 0.82, stream/tick/ms 0.79, terrain/select/ms 0.64, bigspace/total/ms 0.24, render/acquire/ms 0.23, bigspace/high_precision/ms 0.10 |
| warm-up | 1.32 | 33.5 | render/thread/ms 36.08, render/busy/ms 34.93, main/busy/ms 18.59, stream/drive/ms 4.57, render/graph/ms 3.69, stream/spawn/ms 2.12, render/present/ms 0.97, stream/tick/ms 0.72, terrain/update/ms 0.57, terrain/select/ms 0.44, bigspace/total/ms 0.22, physics/insert/ms 0.19 |
| warm-up | 1.67 | 25.2 | pipelines/wait_ms 2811, pipelines/compile_ms 35.30, main/busy/ms 34.71, pipelines/front_ms 33.91, render/thread/ms 32.98, render/busy/ms 31.81, stream/drive/ms 4.20, render/graph/ms 3.66, stream/spawn/ms 2.01, bigspace/total/ms 1.90, pipelines/driver_ms 1.39, render/present/ms 0.98 |
| warm-up | 2.10 | 33.5 | pipelines/wait_ms 5667, pipelines/compile_ms 55.40, pipelines/front_ms 51.98, main/busy/ms 48.59, render/thread/ms 40.88, render/busy/ms 39.38, terrain/update/ms 6.23, render/graph/ms 5.07, stream/drive/ms 3.66, pipelines/driver_ms 3.42, stream/spawn/ms 2.16, render/present/ms 1.28 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_all\balmora_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.9 W · 75 °C (max 77) · VRAM 3924 MiB · throttled 100% of samples
