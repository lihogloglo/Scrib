# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1300 frames over 15.0 s · **median 11.23 ms (89 fps)** · mean 11.55 · p95 12.92 · p99 18.85 (1% low 53 fps) · worst 70.26 ms · 15 spikes over 1.5× median · 64.0 stutters/min (work 67.9/min)
- warm-up: 320 frames over 4.9 s, worst 609.8 ms
- hitches (> 28.1 ms): **6** measured, 7 in warm-up
- timed GPU passes: 9.01 ms of a 11.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.23 ms, limit 11 ms
- **FAIL** worst frame 70.26 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.469 | 3.151 | 0.048 |
| dlss_super_resolution | 2.374 | 3.066 | 0.182 |
| early prepass | 0.754 | 1.082 | 0.087 |
| lightprobe_irradiance_map | 0.456 | 0.819 | 0.001 |
| bloom | 0.378 | 0.714 | 0.070 |
| atmosphere_luts | 0.348 | 0.410 | 0.003 |
| ssao | 0.242 | 0.597 | 0.007 |
| main_transparent_pass_3d | 0.233 | 0.641 | 0.068 |
| lightprobe_radiance_map | 0.223 | 0.234 | 0.004 |
| uniform_allocation | 0.139 | 0.138 | 0.015 |
| clouds_trace | 0.134 | 0.153 | 0.001 |
| water_fft | 0.128 | 0.130 | 0.003 |
| upscaling | 0.127 | 0.148 | 0.005 |
| bin_unpacking | 0.104 | 0.106 | 0.010 |
| gpu_instance_materialization | 0.096 | 0.459 | 0.004 |
| tonemapping | 0.090 | 0.099 | 0.004 |
| atmos_froxel_inject | 0.090 | 0.093 | 0.002 |
| water_velocity | 0.075 | 0.077 | 0.002 |
| clustering | 0.065 | 0.066 | 0.022 |
| clouds_composite | 0.064 | 0.065 | 0.002 |
| early_mesh_preprocessing | 0.063 | 0.065 | 0.004 |
| per_view_shadow_pass | 0.057 | 0.196 | 0.008 |
| render_sky | 0.054 | 0.062 | 0.002 |
| clouds_resolve | 0.038 | 0.047 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.032 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| water_mips | 0.025 | 0.024 | 0.002 |
| range_unpacking | 0.019 | 0.020 | 0.003 |
| atmosphere_environment | 0.015 | 0.012 | 0.003 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.012 | 0.011 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.002 |
| auxiliary_views | 0.003 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 51.229 | 0.000 | 11008.05 | 66598.0 |
| render/thread/ms | 9.083 | 10.273 | 67.13 | 11808.0 |
| render/busy/ms | 8.326 | 9.370 | 66.31 | 10823.3 |
| main/busy/ms | 7.659 | 8.647 | 12.47 | 9956.1 |
| render/graph/ms | 3.794 | 4.470 | 61.89 | 4931.7 |
| pipelines/compile_ms | 0.596 | 0.000 | 138.49 | 774.8 |
| pipelines/front_ms | 0.574 | 0.000 | 134.44 | 746.8 |
| render/present/ms | 0.493 | 1.070 | 3.48 | 640.8 |
| bigspace/total/ms | 0.328 | 0.513 | 1.35 | 426.4 |
| render/acquire/ms | 0.265 | 0.136 | 30.10 | 343.9 |
| terrain/update/ms | 0.226 | 0.279 | 0.55 | 294.4 |
| terrain/select/ms | 0.194 | 0.239 | 0.52 | 251.9 |
| stream/drive/ms | 0.187 | 0.239 | 0.51 | 243.5 |
| far/select/ms | 0.183 | 0.319 | 0.58 | 237.3 |
| controller/move/ms | 0.176 | 0.316 | 0.74 | 228.2 |
| stream/tick/ms | 0.158 | 0.204 | 0.46 | 205.0 |
| controller/feed/ms | 0.128 | 0.191 | 0.34 | 166.5 |
| anim/evaluate/ms | 0.094 | 0.252 | 1.04 | 122.5 |
| bigspace/high_precision/ms | 0.093 | 0.137 | 1.07 | 120.6 |
| anim/apply/ms | 0.082 | 0.200 | 1.14 | 107.1 |
| bigspace/low_precision/ms | 0.068 | 0.090 | 0.79 | 88.9 |
| anim/evaluate_cpu/ms | 0.065 | 0.084 | 0.25 | 84.8 |
| physics/step/ms | 0.039 | 0.065 | 0.85 | 50.3 |
| weather/map/ms | 0.027 | 0.035 | 0.18 | 35.2 |
| pipelines/driver_ms | 0.022 | 0.000 | 5.06 | 28.0 |
| weather/eval/ms | 0.012 | 0.015 | 0.19 | 15.3 |
| physics/insert/ms | 0.012 | 0.000 | 1.16 | 15.1 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 9.1 |
| controller/avoid/ms | 0.007 | 0.011 | 0.17 | 9.0 |
| script/ui/ms | 0.005 | 0.008 | 0.02 | 6.7 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 6.6 |
| controller/write/ms | 0.002 | 0.004 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.9 |
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
| anim/a2/count | count | 0 | 3 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 19 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 56.8 k | 56.8 k | 56.8 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.81 | 66.31 | 9.22 |
| gpu/timed/ms | ms | 8.71 | 42.38 | 9.22 |
| mesh_allocator_allocations |  meshes | 12.8 k | 12.8 k | 12.8 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 320.34 M | 320.34 M | 320.34 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 273 | 273 | 273 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 722 | 722 | 722 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 273 | 273 | 273 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 722 | 722 | 722 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d/bins | count | 954 | 954 | 954 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1898 | 1898 | 1898 |
| phases/opaque3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d/items | count | 3693 | 3693 | 3693 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 954 | 954 | 954 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1898 | 1898 | 1898 |
| phases/opaque3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d_prepass/items | count | 3693 | 3693 | 3693 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 1193 | 1207 | 1193 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 7392 | 7392 | 7392 |
| phases/shadow/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| phases/shadow/items | count | 4626 | 4655 | 4626 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1639 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 22.49 | 31.25 | 24.04 |
| process/mem_usage | GiB | 1.94 | 1.98 | 1.98 |
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1642 | 1642 | 1642 |
| scene/chains/live | count | 1642 | 1642 | 1642 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 10.4 k | 10.4 k | 10.4 k |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 8782 | 8782 | 8782 |
| scene/slots/batch_capacity | count | 16.4 k | 16.4 k | 16.4 k |
| scene/slots/batch_capacity_bytes | bytes | 1.05 M | 1.05 M | 1.05 M |
| scene/slots/batch_live | count | 8782 | 8782 | 8782 |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 4542 | 4544 | 4544 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 354 | 354 | 354 |
| statics/materials_bound | count | 354 | 354 | 354 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 535 | 535 | 535 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.55 | 49.37 | 32.18 |
| system/mem_usage | % | 60.33 | 60.50 | 60.50 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3647 | 3661 | 3647 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 12.3 k | 12.3 k | 12.3 k |
| views/all/instance_slots | count | 81.9 k | 81.9 k | 81.9 k |
| views/all/items | count | 13.5 k | 13.5 k | 13.5 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 55 | 53 |
| views/all/work_items | count | 95.5 k | 95.6 k | 95.5 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 1193 | 1207 | 1193 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 7392 | 7392 | 7392 |
| views/cascade/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| views/cascade/items | count | 4626 | 4655 | 4626 |
| views/cascade/pipelines | count | 27 | 30 | 27 |
| views/cascade/work_items | count | 53.8 k | 53.8 k | 53.8 k |
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
| views/main/bins | count | 2454 | 2454 | 2454 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 4928 | 4928 | 4928 |
| views/main/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| views/main/items | count | 8830 | 8830 | 8830 |
| views/main/pipelines | count | 38 | 40 | 38 |
| views/main/work_items | count | 41.8 k | 41.8 k | 41.8 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 34.5 k | 34.5 k | 34.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.05 | 66 | 66 |
| anim/events/count | count | 0.11 | 3 | 145 |
| assets/Image/added | count | 0.04 | 23 | 52 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.15 | 94 | 200 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.04 | 27 | 57 |
| assets/StandardMaterial/modified | count | 3.80 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.39 | 964 | 3108 |
| physics/inserts | count | 1.15 | 182 | 1495 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.69 | 3 | 900 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| scene/overflow/cells | count | 0 | 0 | 0 |
| scene/slots/overflow | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 13, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.61 | 609.8 | main/busy/ms 585, surface/advance_cpu_ms 68.44, surface/cell_upload_ms 0.68, stream/drive/ms 0.25, weather/map/ms 0.11, bigspace/total/ms 0.11, bigspace/high_precision/ms 0.06, stream/tick/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.97 | 358.2 | main/busy/ms 619, render/thread/ms 318, render/busy/ms 238, render/acquire/ms 76.61, surface/advance_cpu_ms 68.44, render/graph/ms 7.40, render/present/ms 2.73, stream/drive/ms 0.84, surface/cell_upload_ms 0.68, terrain/update/ms 0.67, stream/tick/ms 0.45, terrain/select/ms 0.42 |
| warm-up | 1.06 | 96.8 | pipelines/wait_ms 829, render/thread/ms 336, render/busy/ms 255, pipelines/compile_ms 84.15, pipelines/front_ms 77.45, render/acquire/ms 76.92, main/busy/ms 52.76, terrain/update/ms 18.60, render/graph/ms 12.98, pipelines/driver_ms 6.70, render/present/ms 4.29, stream/drive/ms 0.96 |
| warm-up | 1.32 | 93.6 | pipelines/compile_ms 217, pipelines/front_ms 196, main/busy/ms 70.29, render/thread/ms 54.67, render/busy/ms 52.89, pipelines/driver_ms 21.20, pipelines/wait_ms 17.02, render/graph/ms 4.88, stream/drive/ms 1.51, render/present/ms 1.49, stream/tick/ms 1.24, terrain/update/ms 0.62 |
| warm-up | 3.63 | 31.3 | pipelines/wait_ms 11.7 k, main/busy/ms 48.36, pipelines/compile_ms 38.82, pipelines/front_ms 36.89, render/thread/ms 26.84, render/busy/ms 25.12, render/graph/ms 6.63, stream/drive/ms 2.92, pipelines/driver_ms 1.93, render/present/ms 1.48, stream/spawn/ms 1.29, physics/insert/ms 1.01 |
| warm-up | 3.68 | 47.1 | pipelines/wait_ms 9705, main/busy/ms 58.39, render/thread/ms 46.51, render/busy/ms 44.75, pipelines/compile_ms 21.59, pipelines/front_ms 20.20, render/graph/ms 7.62, stream/drive/ms 1.81, render/present/ms 1.53, bigspace/total/ms 1.49, pipelines/driver_ms 1.39, physics/insert/ms 1.01 |
| warm-up | 3.75 | 46.6 | pipelines/wait_ms 4722, pipelines/compile_ms 125, pipelines/front_ms 118, main/busy/ms 64.31, render/thread/ms 42.92, render/busy/ms 41.49, pipelines/driver_ms 7.82, render/graph/ms 6.50, stream/drive/ms 2.67, stream/spawn/ms 1.55, render/present/ms 1.18, physics/insert/ms 1.07 |
| measure | 0.36 | 35.9 | pipelines/wait_ms 11.0 k, pipelines/compile_ms 138, pipelines/front_ms 134, render/thread/ms 43.89, render/acquire/ms 26.14, render/busy/ms 16.80, main/busy/ms 13.46, render/graph/ms 6.52, pipelines/driver_ms 4.05, render/present/ms 0.94, terrain/update/ms 0.53, terrain/select/ms 0.47 |
| measure | 0.42 | 31.0 | pipelines/wait_ms 13.4 k, render/thread/ms 51.64, pipelines/compile_ms 49.52, pipelines/front_ms 46.81, render/acquire/ms 31.59, render/busy/ms 18.92, main/busy/ms 18.70, render/graph/ms 7.50, pipelines/driver_ms 2.71, render/present/ms 1.14, physics/insert/ms 1.04, bigspace/total/ms 0.59 |
| measure | 0.46 | 38.5 | pipelines/wait_ms 7534, render/thread/ms 63.69, render/busy/ms 35.54, pipelines/compile_ms 29.51, pipelines/front_ms 28.24, render/acquire/ms 27.05, render/graph/ms 24.46, main/busy/ms 16.07, pipelines/driver_ms 1.27, physics/step/ms 1.12, render/present/ms 1.11, physics/insert/ms 1.10 |
| measure | 0.62 | 42.9 | pipelines/wait_ms 10.7 k, pipelines/compile_ms 107, pipelines/front_ms 102, render/thread/ms 63.61, render/acquire/ms 43.54, render/busy/ms 18.95, main/busy/ms 15.03, render/graph/ms 8.91, pipelines/driver_ms 5.06, physics/step/ms 1.13, render/present/ms 1.12, bigspace/total/ms 0.57 |
| measure | 0.68 | 41.0 | pipelines/wait_ms 8098, render/thread/ms 58.39, pipelines/compile_ms 38.69, render/acquire/ms 38.50, pipelines/front_ms 36.43, render/busy/ms 18.78, main/busy/ms 16.87, render/graph/ms 7.58, pipelines/driver_ms 2.26, physics/insert/ms 1.62, render/present/ms 1.10, bigspace/total/ms 0.63 |
| measure | 0.93 | 70.3 | render/thread/ms 74.90, render/busy/ms 73.55, render/graph/ms 64.66, main/busy/ms 16.44, render/present/ms 1.19, bigspace/total/ms 0.60, terrain/update/ms 0.48, terrain/select/ms 0.42, stream/drive/ms 0.39, far/select/ms 0.38, stream/tick/ms 0.33, physics/step/ms 0.32 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on\balmora@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 12%, CPU 20%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **88%** (min 46%) · 2581 MHz (min 2580) · memory 8001 MHz · 81.2 W · 69 °C (max 71) · VRAM 4077 MiB · throttled 100% of samples
