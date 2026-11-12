# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1977 frames over 15.0 s · **median 7.52 ms (133 fps)** · mean 7.59 · p95 8.47 · p99 9.03 (1% low 111 fps) · worst 24.68 ms · 5 spikes over 1.5× median · 20.0 stutters/min (work 8.0/min)
- warm-up: 305 frames over 3.1 s, worst 577.3 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.24 ms of a 7.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.52 ms, limit 11 ms
- pass worst frame 24.68 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.187 | 2.537 | 0.178 |
| main_opaque_pass_3d | 1.716 | 2.144 | 0.019 |
| early prepass | 0.447 | 0.783 | 0.033 |
| lightprobe_irradiance_map | 0.421 | 0.419 | 0.001 |
| atmosphere_luts | 0.362 | 0.443 | 0.003 |
| bloom | 0.340 | 0.598 | 0.058 |
| lightprobe_radiance_map | 0.236 | 0.251 | 0.003 |
| ssao | 0.217 | 0.215 | 0.006 |
| main_transparent_pass_3d | 0.172 | 0.189 | 0.013 |
| water_fft | 0.136 | 0.131 | 0.002 |
| clouds_trace | 0.135 | 0.155 | 0.001 |
| upscaling | 0.093 | 0.139 | 0.004 |
| tonemapping | 0.092 | 0.098 | 0.003 |
| atmos_froxel_inject | 0.092 | 0.094 | 0.001 |
| water_velocity | 0.078 | 0.076 | 0.001 |
| clustering | 0.068 | 0.076 | 0.020 |
| clouds_composite | 0.067 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| uniform_allocation | 0.036 | 0.037 | 0.007 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| bin_unpacking | 0.029 | 0.031 | 0.006 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.023 | 0.031 | 0.001 |
| per_view_shadow_pass | 0.021 | 0.029 | 0.007 |
| early_mesh_preprocessing | 0.020 | 0.022 | 0.004 |
| lightprobe_copy | 0.015 | 0.017 | 0.003 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.005 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.004 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.855 | 7.743 | 23.71 | 13551.6 |
| render/busy/ms | 4.669 | 5.624 | 14.98 | 9231.1 |
| main/busy/ms | 4.305 | 5.116 | 7.03 | 8510.8 |
| render/graph/ms | 2.658 | 3.546 | 13.09 | 5254.2 |
| render/acquire/ms | 1.740 | 2.760 | 3.57 | 3440.5 |
| render/present/ms | 0.445 | 0.561 | 17.57 | 879.9 |
| terrain/update/ms | 0.239 | 0.308 | 0.60 | 473.4 |
| bigspace/total/ms | 0.206 | 0.419 | 1.16 | 407.1 |
| terrain/select/ms | 0.206 | 0.263 | 0.55 | 406.9 |
| stream/drive/ms | 0.174 | 0.233 | 0.51 | 343.7 |
| far/select/ms | 0.150 | 0.192 | 0.47 | 297.0 |
| stream/tick/ms | 0.146 | 0.197 | 0.47 | 288.5 |
| bigspace/high_precision/ms | 0.117 | 0.268 | 1.10 | 230.6 |
| bigspace/low_precision/ms | 0.046 | 0.178 | 0.91 | 90.2 |
| weather/map/ms | 0.025 | 0.033 | 0.37 | 50.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 20.9 |
| physics/step/ms | 0.007 | 0.016 | 0.14 | 13.3 |
| script/update/ms | 0.006 | 0.008 | 0.15 | 11.8 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 9.7 |
| script/ui/ms | 0.003 | 0.005 | 0.03 | 5.5 |
| controller/move/ms | 0.002 | 0.003 | 0.03 | 3.9 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.02 | 3.5 |
| anim/apply/ms | 0.001 | 0.002 | 0.12 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.12 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| weather/apply/ms | 0.000 | 0.000 | 0.04 | 0.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 0.3 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.1 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| ecs/entities | count | 6837 | 6838 | 6838 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 7.22 | 14.98 | 7.12 |
| gpu/timed/ms | ms | 7.22 | 8.07 | 7.12 |
| mesh_allocator_allocations |  meshes | 8081 | 8081 | 8081 |
| mesh_allocator_slabs |  slabs | 11 | 11 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 298.98 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 219 | 219 | 219 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 219 | 219 | 219 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 219 | 219 | 219 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 219 | 219 | 219 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 247 | 247 | 247 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 371 | 371 | 371 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 247 | 247 | 247 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 371 | 371 | 371 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/shadow/batch_sets | count | 5 | 5 | 5 |
| phases/shadow/bins | count | 75 | 75 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 197 | 197 | 197 |
| phases/shadow/pipelines | count | 5 | 5 | 5 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2077 | 2077 | 2077 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 106 | 106 | 106 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 54 | 54 | 54 |
| preprocess/bind_groups_created | count | 54 | 54 | 54 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 22.22 | 39.23 | 18.27 |
| process/mem_usage | GiB | 1.90 | 1.95 | 1.95 |
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
| started_load_count |  | 2372 | 2374 | 2374 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.22 | 0.22 | 0.22 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 656 | 656 | 656 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.19 | 45.41 | 23.73 |
| system/mem_usage | % | 47.76 | 47.91 | 47.91 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 17 | 17 | 17 |
| views/all/bins | count | 1007 | 1007 | 1007 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1377 | 1377 | 1377 |
| views/all/pipelines | count | 13 | 13 | 13 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1501 | 1501 | 1501 |
| views/cascade/batch_sets | count | 5 | 5 | 5 |
| views/cascade/bins | count | 75 | 75 | 75 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 197 | 197 | 197 |
| views/cascade/pipelines | count | 5 | 5 | 5 |
| views/cascade/work_items | count | 197 | 197 | 197 |
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
| views/main/bins | count | 932 | 932 | 932 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 1180 | 1180 | 1180 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1304 | 1304 | 1304 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 2697 | 2697 | 2697 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.46 | 1 | 900 |
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
| warm-up | 0.58 | 577.3 | main/busy/ms 555, surface/advance_cpu_ms 64.75, surface/cell_upload_ms 0.84, stream/drive/ms 0.31, weather/map/ms 0.12, stream/tick/ms 0.08, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.95 | 370.8 | main/busy/ms 589, render/thread/ms 337, render/busy/ms 245, render/acquire/ms 89.30, surface/advance_cpu_ms 64.75, render/graph/ms 4.52, render/present/ms 2.40, stream/drive/ms 0.97, surface/cell_upload_ms 0.84, stream/tick/ms 0.47, terrain/update/ms 0.33, weather/map/ms 0.24 |
| warm-up | 1.03 | 80.9 | pipelines/wait_ms 666, render/thread/ms 356, render/busy/ms 263, render/acquire/ms 89.68, pipelines/compile_ms 71.34, pipelines/front_ms 64.93, main/busy/ms 51.77, terrain/update/ms 18.22, render/graph/ms 10.03, pipelines/driver_ms 6.41, render/present/ms 3.58, stream/drive/ms 0.99 |
| warm-up | 1.27 | 93.1 | main/busy/ms 63.73, render/thread/ms 60.49, render/busy/ms 58.91, render/graph/ms 3.79, stream/drive/ms 2.02, stream/tick/ms 1.54, render/present/ms 1.36, terrain/update/ms 0.59, terrain/select/ms 0.46, render/acquire/ms 0.21, weather/map/ms 0.17, bigspace/total/ms 0.16 |
| warm-up | 2.78 | 30.6 | render/thread/ms 37.21, render/busy/ms 32.82, render/graph/ms 28.29, main/busy/ms 12.25, render/acquire/ms 3.09, render/present/ms 1.31, physics/insert/ms 1.12, stream/drive/ms 0.88, terrain/update/ms 0.47, stream/tick/ms 0.47, terrain/select/ms 0.41, bigspace/total/ms 0.27 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ceil_statics_off\balmora@2560x1440_off=statics-1 --bench-baseline none --off statics
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 8%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 95%) · 2580 MHz (min 2580) · memory 8001 MHz · 102.1 W · 76 °C (max 78) · VRAM 3470 MiB · throttled 100% of samples
