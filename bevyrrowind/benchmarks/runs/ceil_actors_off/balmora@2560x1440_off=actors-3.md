# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1656 frames over 15.0 s · **median 9.02 ms (111 fps)** · mean 9.06 · p95 9.98 · p99 10.56 (1% low 95 fps) · worst 12.84 ms · 0 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 302 frames over 3.6 s, worst 574.3 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 8.24 ms of a 9.06 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.02 ms, limit 11 ms
- pass worst frame 12.84 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.449 | 2.949 | 0.040 |
| dlss_super_resolution | 2.092 | 2.437 | 0.187 |
| early prepass | 0.551 | 0.566 | 0.068 |
| lightprobe_irradiance_map | 0.414 | 0.415 | 0.001 |
| atmosphere_luts | 0.391 | 0.708 | 0.003 |
| bloom | 0.323 | 0.335 | 0.062 |
| ssao | 0.288 | 0.588 | 0.008 |
| lightprobe_radiance_map | 0.223 | 0.235 | 0.004 |
| main_transparent_pass_3d | 0.193 | 0.202 | 0.070 |
| water_fft | 0.168 | 0.515 | 0.003 |
| clouds_trace | 0.136 | 0.160 | 0.001 |
| upscaling | 0.119 | 0.128 | 0.004 |
| uniform_allocation | 0.103 | 0.104 | 0.016 |
| atmos_froxel_inject | 0.089 | 0.091 | 0.001 |
| tonemapping | 0.089 | 0.092 | 0.003 |
| water_velocity | 0.075 | 0.076 | 0.001 |
| bin_unpacking | 0.075 | 0.081 | 0.011 |
| clustering | 0.064 | 0.076 | 0.021 |
| clouds_composite | 0.064 | 0.065 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.040 | 0.006 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| water_mips | 0.027 | 0.026 | 0.002 |
| per_view_shadow_pass | 0.027 | 0.029 | 0.007 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.022 | 0.025 | 0.001 |
| lightprobe_copy | 0.016 | 0.017 | 0.004 |
| atmosphere_environment | 0.015 | 0.014 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.003 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 30.846 | 0.000 | 5698.78 | 51080.7 |
| render/thread/ms | 8.277 | 9.223 | 11.85 | 13706.6 |
| render/busy/ms | 5.868 | 6.965 | 10.65 | 9717.5 |
| main/busy/ms | 5.214 | 6.245 | 9.00 | 8634.6 |
| render/graph/ms | 3.044 | 3.980 | 6.08 | 5041.3 |
| render/acquire/ms | 1.978 | 3.014 | 4.12 | 3276.2 |
| pipelines/compile_ms | 0.565 | 0.000 | 136.82 | 935.9 |
| pipelines/front_ms | 0.546 | 0.000 | 132.44 | 904.6 |
| render/present/ms | 0.431 | 0.543 | 1.93 | 712.9 |
| terrain/update/ms | 0.236 | 0.310 | 0.58 | 390.1 |
| bigspace/total/ms | 0.226 | 0.406 | 1.05 | 373.9 |
| terrain/select/ms | 0.202 | 0.266 | 0.54 | 335.1 |
| stream/drive/ms | 0.173 | 0.211 | 0.58 | 286.5 |
| far/select/ms | 0.152 | 0.187 | 0.68 | 251.5 |
| stream/tick/ms | 0.148 | 0.182 | 0.53 | 244.8 |
| bigspace/high_precision/ms | 0.105 | 0.236 | 0.89 | 174.4 |
| bigspace/low_precision/ms | 0.042 | 0.117 | 0.81 | 69.1 |
| weather/map/ms | 0.024 | 0.032 | 0.16 | 40.1 |
| pipelines/driver_ms | 0.019 | 0.000 | 4.48 | 31.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 18.0 |
| physics/step/ms | 0.010 | 0.016 | 0.81 | 16.2 |
| script/update/ms | 0.007 | 0.008 | 0.24 | 11.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.1 |
| script/ui/ms | 0.003 | 0.004 | 0.17 | 4.6 |
| physics/insert/ms | 0.003 | 0.000 | 0.60 | 4.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.02 | 0.5 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| ecs/entities | count | 27.7 k | 27.7 k | 27.7 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.26 | 10.65 | 8.16 |
| gpu/timed/ms | ms | 8.26 | 10.12 | 8.16 |
| mesh_allocator_allocations |  meshes | 11.4 k | 11.4 k | 11.4 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 430.57 M | 430.57 M | 430.57 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 281 | 281 | 281 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 875 | 875 | 875 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 281 | 281 | 281 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 875 | 875 | 875 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 938 | 938 | 938 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3001 | 3001 | 3001 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 938 | 938 | 938 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3001 | 3001 | 3001 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/shadow/batch_sets | count | 16 | 16 | 16 |
| phases/shadow/bins | count | 1619 | 1619 | 1619 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4449 | 4449 | 4449 |
| phases/shadow/pipelines | count | 16 | 16 | 16 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2077 | 2077 | 2077 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 326 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 136 | 136 | 136 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 120 | 120 | 120 |
| preprocess/bind_groups_created | count | 120 | 120 | 120 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 22.52 | 35.22 | 22.12 |
| process/mem_usage | GiB | 1.86 | 1.91 | 1.91 |
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
| started_load_count |  | 2132 | 2134 | 2134 |
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
| stream/models | count | 240 | 240 | 240 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.48 | 62.02 | 35.04 |
| system/mem_usage | % | 50.74 | 50.83 | 50.83 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 50 | 50 | 50 |
| views/all/bins | count | 4057 | 4057 | 4057 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 46 | 46 | 46 |
| views/all/pipelines_distinct | count | 40 | 40 | 40 |
| views/all/work_items | count | 12.4 k | 12.4 k | 12.4 k |
| views/cascade/batch_sets | count | 16 | 16 | 16 |
| views/cascade/bins | count | 1619 | 1619 | 1619 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4449 | 4449 | 4449 |
| views/cascade/pipelines | count | 16 | 16 | 16 |
| views/cascade/work_items | count | 4449 | 4449 | 4449 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 34 | 34 | 34 |
| views/main/bins | count | 2438 | 2438 | 2438 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7752 | 7752 | 7752 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7930 | 7930 | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.7 k | 21.7 k | 21.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.99 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 3 |
| physics/inserts | count | 0.22 | 82 | 358 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.54 | 1 | 900 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 574.3 | main/busy/ms 550, surface/advance_cpu_ms 64.98, surface/cell_upload_ms 0.78, stream/drive/ms 0.35, weather/map/ms 0.09, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, script/ui/ms 0.01 |
| warm-up | 0.98 | 407.1 | main/busy/ms 582, render/thread/ms 368, render/busy/ms 298, render/acquire/ms 68.29, surface/advance_cpu_ms 64.98, render/graph/ms 5.40, render/present/ms 1.91, stream/drive/ms 1.07, surface/cell_upload_ms 0.78, terrain/update/ms 0.59, stream/tick/ms 0.58, terrain/select/ms 0.40 |
| warm-up | 1.23 | 85.3 | main/busy/ms 54.04, render/thread/ms 51.91, render/busy/ms 50.36, render/graph/ms 2.98, stream/drive/ms 1.89, stream/tick/ms 1.50, render/present/ms 1.31, terrain/update/ms 0.66, terrain/select/ms 0.52, render/acquire/ms 0.24, weather/map/ms 0.16, bigspace/total/ms 0.16 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ceil_actors_off\balmora@2560x1440_off=actors-3 --bench-baseline none --off actors
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.9 W · 70 °C (max 73) · VRAM 4031 MiB · throttled 100% of samples
