# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1635 frames over 15.0 s · **median 9.13 ms (110 fps)** · mean 9.18 · p95 10.52 · p99 11.55 (1% low 87 fps) · worst 22.79 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 8.0/min)
- warm-up: 293 frames over 3.6 s, worst 576.3 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.34 ms of a 9.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.13 ms, limit 11 ms
- pass worst frame 22.79 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.447 | 2.984 | 0.040 |
| dlss_super_resolution | 2.139 | 2.615 | 0.191 |
| early prepass | 0.544 | 0.865 | 0.070 |
| atmosphere_luts | 0.434 | 0.713 | 0.003 |
| lightprobe_irradiance_map | 0.426 | 0.418 | 0.001 |
| bloom | 0.330 | 0.338 | 0.067 |
| ssao | 0.267 | 0.582 | 0.007 |
| lightprobe_radiance_map | 0.232 | 0.243 | 0.005 |
| main_transparent_pass_3d | 0.199 | 0.229 | 0.072 |
| water_fft | 0.144 | 0.284 | 0.003 |
| clouds_trace | 0.138 | 0.164 | 0.001 |
| upscaling | 0.127 | 0.207 | 0.004 |
| uniform_allocation | 0.104 | 0.108 | 0.014 |
| tonemapping | 0.092 | 0.105 | 0.003 |
| atmos_froxel_inject | 0.090 | 0.093 | 0.001 |
| water_velocity | 0.079 | 0.076 | 0.001 |
| bin_unpacking | 0.078 | 0.082 | 0.010 |
| clustering | 0.066 | 0.079 | 0.023 |
| clouds_composite | 0.065 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.038 | 0.040 | 0.006 |
| clouds_resolve | 0.036 | 0.039 | 0.001 |
| clouds_shadow | 0.033 | 0.037 | 0.003 |
| water_mips | 0.026 | 0.025 | 0.002 |
| per_view_shadow_pass | 0.025 | 0.031 | 0.007 |
| atmos_froxel_integrate | 0.024 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.002 |
| atmosphere_environment | 0.016 | 0.015 | 0.003 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.009 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 44.062 | 0.000 | 9054.97 | 72041.0 |
| render/thread/ms | 8.336 | 9.620 | 22.08 | 13628.8 |
| render/busy/ms | 6.182 | 7.539 | 19.59 | 10108.3 |
| main/busy/ms | 5.563 | 6.666 | 9.19 | 9095.3 |
| render/graph/ms | 3.229 | 4.366 | 16.77 | 5278.6 |
| render/acquire/ms | 1.687 | 2.886 | 4.20 | 2758.0 |
| pipelines/compile_ms | 0.615 | 0.000 | 146.42 | 1005.3 |
| pipelines/front_ms | 0.585 | 0.000 | 140.12 | 957.2 |
| render/present/ms | 0.466 | 0.605 | 1.84 | 762.5 |
| bigspace/total/ms | 0.247 | 0.448 | 1.42 | 403.6 |
| terrain/update/ms | 0.243 | 0.313 | 0.71 | 396.8 |
| terrain/select/ms | 0.208 | 0.266 | 0.68 | 340.7 |
| stream/drive/ms | 0.184 | 0.256 | 0.58 | 300.9 |
| far/select/ms | 0.166 | 0.243 | 0.52 | 271.4 |
| stream/tick/ms | 0.157 | 0.224 | 0.54 | 256.5 |
| bigspace/high_precision/ms | 0.115 | 0.256 | 1.06 | 187.5 |
| bigspace/low_precision/ms | 0.042 | 0.093 | 0.66 | 69.4 |
| pipelines/driver_ms | 0.029 | 0.000 | 11.14 | 48.1 |
| weather/map/ms | 0.025 | 0.033 | 0.18 | 40.7 |
| physics/step/ms | 0.014 | 0.030 | 0.18 | 23.0 |
| weather/eval/ms | 0.012 | 0.015 | 0.45 | 19.2 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 11.1 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 8.6 |
| script/ui/ms | 0.003 | 0.006 | 0.10 | 5.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| frame/work/ms | ms | 8.33 | 19.59 | 8.09 |
| gpu/timed/ms | ms | 8.32 | 11.12 | 8.09 |
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
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 136 | 136 | 136 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 120 | 128 | 120 |
| preprocess/bind_groups_created | count | 120 | 128 | 120 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 25.35 | 44.02 | 24.52 |
| process/mem_usage | GiB | 1.93 | 2.01 | 2.01 |
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
| system/cpu_usage | % | 37.40 | 54.91 | 37.51 |
| system/mem_usage | % | 50.83 | 51.16 | 51.12 |
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
| assets/StandardMaterial/modified | count | 3.03 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 3 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.55 | 1 | 900 |
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
| warm-up | 0.58 | 576.3 | main/busy/ms 548, surface/advance_cpu_ms 63.85, surface/cell_upload_ms 0.69, stream/drive/ms 0.31, weather/map/ms 0.12, stream/tick/ms 0.08, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.84 | 263.9 | main/busy/ms 581, render/thread/ms 224, render/busy/ms 142, render/acquire/ms 79.29, surface/advance_cpu_ms 63.85, render/graph/ms 4.91, render/present/ms 2.38, stream/drive/ms 0.85, surface/cell_upload_ms 0.69, stream/tick/ms 0.42, terrain/update/ms 0.34, bigspace/total/ms 0.22 |
| warm-up | 1.00 | 160.7 | pipelines/wait_ms 1529, render/thread/ms 241, pipelines/compile_ms 177, pipelines/front_ms 161, render/busy/ms 158, render/acquire/ms 79.48, main/busy/ms 49.67, terrain/update/ms 17.74, pipelines/driver_ms 15.40, render/graph/ms 9.23, render/present/ms 3.80, stream/drive/ms 1.04 |
| warm-up | 1.22 | 74.0 | pipelines/compile_ms 112, pipelines/front_ms 104, pipelines/wait_ms 91.06, main/busy/ms 55.29, render/thread/ms 48.56, render/busy/ms 47.23, pipelines/driver_ms 7.67, render/graph/ms 3.95, stream/drive/ms 1.44, render/present/ms 1.12, stream/tick/ms 1.11, terrain/update/ms 0.56 |
| warm-up | 1.76 | 28.2 | pipelines/wait_ms 705, render/thread/ms 36.37, render/busy/ms 35.31, main/busy/ms 17.53, pipelines/compile_ms 7.79, pipelines/front_ms 7.25, stream/drive/ms 4.16, render/graph/ms 3.30, stream/spawn/ms 2.07, render/present/ms 0.89, stream/tick/ms 0.81, pipelines/driver_ms 0.54 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ceil_actors_off\balmora@2560x1440_off=actors-2 --bench-baseline none --off actors
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 63%) · 2582 MHz (min 2580) · memory 8001 MHz · 92.4 W · 68 °C (max 71) · VRAM 3770 MiB · throttled 100% of samples
