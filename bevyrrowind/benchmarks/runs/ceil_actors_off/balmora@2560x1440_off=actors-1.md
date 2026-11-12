# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1612 frames over 15.0 s · **median 9.17 ms (109 fps)** · mean 9.31 · p95 10.75 · p99 12.99 (1% low 77 fps) · worst 24.88 ms · 9 spikes over 1.5× median · 36.0 stutters/min (work 12.0/min)
- warm-up: 295 frames over 3.9 s, worst 637.4 ms
- hitches (> 25.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 8.35 ms of a 9.31 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.17 ms, limit 11 ms
- pass worst frame 24.88 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.514 | 2.968 | 0.050 |
| dlss_super_resolution | 2.111 | 2.475 | 0.205 |
| early prepass | 0.559 | 0.569 | 0.076 |
| atmosphere_luts | 0.428 | 0.777 | 0.004 |
| lightprobe_irradiance_map | 0.420 | 0.416 | 0.001 |
| bloom | 0.327 | 0.339 | 0.072 |
| ssao | 0.267 | 0.585 | 0.008 |
| lightprobe_radiance_map | 0.225 | 0.232 | 0.004 |
| main_transparent_pass_3d | 0.202 | 0.231 | 0.079 |
| water_fft | 0.151 | 0.505 | 0.003 |
| clouds_trace | 0.136 | 0.160 | 0.002 |
| upscaling | 0.105 | 0.124 | 0.005 |
| uniform_allocation | 0.102 | 0.108 | 0.019 |
| atmos_froxel_inject | 0.091 | 0.093 | 0.002 |
| tonemapping | 0.089 | 0.091 | 0.004 |
| bin_unpacking | 0.077 | 0.079 | 0.013 |
| water_velocity | 0.076 | 0.076 | 0.002 |
| clustering | 0.065 | 0.078 | 0.024 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.003 |
| early_mesh_preprocessing | 0.037 | 0.041 | 0.006 |
| clouds_resolve | 0.036 | 0.039 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.004 |
| per_view_shadow_pass | 0.027 | 0.030 | 0.008 |
| water_mips | 0.027 | 0.025 | 0.002 |
| atmos_froxel_integrate | 0.024 | 0.028 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.013 | 0.014 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.006 | 0.007 | 0.004 |
| night_sky | 0.004 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 18.876 | 0.000 | 2848.55 | 30427.5 |
| render/thread/ms | 8.428 | 9.802 | 23.98 | 13585.2 |
| render/busy/ms | 6.780 | 8.566 | 23.42 | 10929.9 |
| main/busy/ms | 6.091 | 7.481 | 14.90 | 9819.1 |
| render/graph/ms | 3.593 | 4.982 | 20.62 | 5792.7 |
| render/acquire/ms | 1.135 | 2.362 | 3.86 | 1829.7 |
| render/present/ms | 0.512 | 0.660 | 7.30 | 825.6 |
| pipelines/compile_ms | 0.504 | 0.000 | 147.65 | 812.5 |
| pipelines/front_ms | 0.488 | 0.000 | 142.75 | 786.3 |
| bigspace/total/ms | 0.288 | 0.552 | 2.50 | 464.6 |
| terrain/update/ms | 0.248 | 0.318 | 0.69 | 400.4 |
| terrain/select/ms | 0.213 | 0.271 | 0.52 | 342.6 |
| stream/drive/ms | 0.207 | 0.316 | 0.80 | 333.2 |
| far/select/ms | 0.186 | 0.288 | 1.00 | 299.8 |
| stream/tick/ms | 0.177 | 0.277 | 0.74 | 284.7 |
| bigspace/high_precision/ms | 0.125 | 0.285 | 1.41 | 202.2 |
| bigspace/low_precision/ms | 0.055 | 0.186 | 0.87 | 89.5 |
| weather/map/ms | 0.029 | 0.039 | 0.33 | 46.1 |
| pipelines/driver_ms | 0.016 | 0.000 | 5.21 | 26.2 |
| weather/eval/ms | 0.012 | 0.017 | 0.06 | 20.0 |
| physics/step/ms | 0.010 | 0.021 | 0.47 | 16.2 |
| script/update/ms | 0.008 | 0.011 | 0.20 | 12.7 |
| script/input/ms | 0.006 | 0.008 | 0.04 | 9.0 |
| script/ui/ms | 0.004 | 0.006 | 0.05 | 6.4 |
| physics/insert/ms | 0.001 | 0.000 | 1.03 | 2.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| frame/work/ms | ms | 8.34 | 23.42 | 8.30 |
| gpu/timed/ms | ms | 8.33 | 9.46 | 8.30 |
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
| phases/shadow/batch_sets | count | 16 | 19 | 16 |
| phases/shadow/bins | count | 1619 | 1926 | 1619 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4449 | 4775 | 4449 |
| phases/shadow/pipelines | count | 16 | 19 | 16 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2077 | 2077 | 2077 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 94 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 131 | 131 | 131 |
| pipelines/waiting | count | 0 | 12 | 0 |
| preprocess/bind_groups | count | 120 | 128 | 120 |
| preprocess/bind_groups_created | count | 120 | 128 | 120 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 25.85 | 39.90 | 24.04 |
| process/mem_usage | GiB | 1.90 | 2.03 | 2.03 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 240 | 240 | 240 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 40.25 | 79.62 | 25.59 |
| system/mem_usage | % | 50.91 | 51.35 | 51.35 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 50 | 53 | 50 |
| views/all/bins | count | 4057 | 4364 | 4057 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 12.2 k | 12.5 k | 12.2 k |
| views/all/pipelines | count | 46 | 49 | 46 |
| views/all/pipelines_distinct | count | 40 | 40 | 40 |
| views/all/work_items | count | 12.4 k | 12.7 k | 12.4 k |
| views/cascade/batch_sets | count | 16 | 19 | 16 |
| views/cascade/bins | count | 1619 | 1926 | 1619 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4449 | 4775 | 4449 |
| views/cascade/pipelines | count | 16 | 19 | 16 |
| views/cascade/work_items | count | 4449 | 4775 | 4449 |
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
| assets/StandardMaterial/modified | count | 3.07 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0.06 | 34 | 94 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.64 | 637.4 | main/busy/ms 601, surface/advance_cpu_ms 65.95, stream/drive/ms 1.93, stream/tick/ms 0.90, surface/cell_upload_ms 0.73, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.21, weather/map/ms 0.12, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 1.15 | 517.4 | main/busy/ms 827, render/thread/ms 477, render/busy/ms 393, render/acquire/ms 81.14, surface/advance_cpu_ms 65.95, render/graph/ms 6.05, render/present/ms 3.81, stream/drive/ms 2.60, terrain/update/ms 1.61, stream/tick/ms 1.38, surface/cell_upload_ms 0.73, bigspace/total/ms 0.41 |
| warm-up | 1.41 | 83.8 | pipelines/wait_ms 600, pipelines/compile_ms 227, pipelines/front_ms 202, main/busy/ms 59.26, render/thread/ms 51.63, render/busy/ms 49.86, pipelines/driver_ms 25.05, render/graph/ms 3.90, stream/drive/ms 1.82, render/present/ms 1.51, stream/tick/ms 1.45, terrain/update/ms 0.53 |
| warm-up | 1.45 | 27.3 | pipelines/wait_ms 694, pipelines/compile_ms 53.04, pipelines/front_ms 44.81, render/thread/ms 29.21, render/busy/ms 27.57, main/busy/ms 18.17, pipelines/driver_ms 8.24, render/graph/ms 2.91, stream/drive/ms 2.53, render/present/ms 1.41, stream/tick/ms 1.18, stream/spawn/ms 0.69 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ceil_actors_off\balmora@2560x1440_off=actors-1 --bench-baseline none --off actors
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 57%) · 2586 MHz (min 2580) · memory 8001 MHz · 92.0 W · 64 °C (max 69) · VRAM 4016 MiB · throttled 100% of samples
