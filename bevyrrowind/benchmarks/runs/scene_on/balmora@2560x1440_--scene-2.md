# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1326 frames over 15.0 s · **median 11.23 ms (89 fps)** · mean 11.32 · p95 12.69 · p99 13.81 (1% low 72 fps) · worst 17.11 ms · 1 spikes over 1.5× median · 0.0 stutters/min (work 4.0/min)
- warm-up: 318 frames over 5.2 s, worst 625.2 ms
- hitches (> 28.1 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.73 ms of a 11.32 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.23 ms, limit 11 ms
- pass worst frame 17.11 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.396 | 3.131 | 0.049 |
| dlss_super_resolution | 2.343 | 2.997 | 0.184 |
| early prepass | 0.592 | 0.976 | 0.087 |
| lightprobe_irradiance_map | 0.469 | 0.973 | 0.001 |
| bloom | 0.374 | 0.712 | 0.069 |
| atmosphere_luts | 0.354 | 0.427 | 0.003 |
| main_transparent_pass_3d | 0.237 | 0.629 | 0.066 |
| ssao | 0.234 | 0.378 | 0.007 |
| lightprobe_radiance_map | 0.225 | 0.242 | 0.004 |
| uniform_allocation | 0.136 | 0.137 | 0.015 |
| clouds_trace | 0.134 | 0.152 | 0.001 |
| upscaling | 0.129 | 0.129 | 0.005 |
| water_fft | 0.129 | 0.129 | 0.003 |
| bin_unpacking | 0.099 | 0.104 | 0.011 |
| atmos_froxel_inject | 0.091 | 0.094 | 0.001 |
| tonemapping | 0.090 | 0.097 | 0.004 |
| gpu_instance_materialization | 0.080 | 0.399 | 0.004 |
| water_velocity | 0.074 | 0.077 | 0.002 |
| early_mesh_preprocessing | 0.067 | 0.066 | 0.004 |
| clouds_composite | 0.064 | 0.065 | 0.002 |
| clustering | 0.062 | 0.065 | 0.022 |
| render_sky | 0.054 | 0.058 | 0.002 |
| per_view_shadow_pass | 0.049 | 0.038 | 0.008 |
| clouds_resolve | 0.038 | 0.047 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.032 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| water_mips | 0.025 | 0.024 | 0.002 |
| range_unpacking | 0.017 | 0.018 | 0.003 |
| atmosphere_environment | 0.016 | 0.013 | 0.003 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.009 | 0.008 | 0.003 |
| night_sky | 0.003 | 0.004 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 23.651 | 0.000 | 8092.52 | 31361.3 |
| render/thread/ms | 8.859 | 9.980 | 20.00 | 11747.6 |
| render/busy/ms | 8.270 | 9.309 | 17.20 | 10966.5 |
| main/busy/ms | 7.715 | 8.699 | 11.62 | 10229.6 |
| render/graph/ms | 3.761 | 4.501 | 7.89 | 4987.6 |
| render/present/ms | 0.496 | 1.122 | 3.05 | 657.1 |
| pipelines/compile_ms | 0.333 | 0.000 | 130.63 | 440.9 |
| bigspace/total/ms | 0.327 | 0.538 | 1.61 | 433.4 |
| pipelines/front_ms | 0.323 | 0.000 | 127.50 | 427.9 |
| terrain/update/ms | 0.229 | 0.287 | 0.89 | 303.8 |
| controller/move/ms | 0.222 | 0.413 | 1.00 | 294.4 |
| terrain/select/ms | 0.196 | 0.245 | 0.85 | 259.7 |
| stream/drive/ms | 0.186 | 0.237 | 0.54 | 247.3 |
| far/select/ms | 0.182 | 0.319 | 0.61 | 241.4 |
| stream/tick/ms | 0.157 | 0.206 | 0.46 | 208.2 |
| controller/feed/ms | 0.128 | 0.185 | 0.50 | 169.5 |
| anim/evaluate/ms | 0.109 | 0.377 | 1.01 | 144.9 |
| render/acquire/ms | 0.094 | 0.122 | 0.42 | 124.0 |
| bigspace/high_precision/ms | 0.093 | 0.130 | 1.00 | 122.8 |
| anim/apply/ms | 0.082 | 0.216 | 1.34 | 109.0 |
| bigspace/low_precision/ms | 0.064 | 0.083 | 1.08 | 84.5 |
| anim/evaluate_cpu/ms | 0.060 | 0.076 | 0.33 | 80.1 |
| physics/step/ms | 0.032 | 0.058 | 0.56 | 42.1 |
| weather/map/ms | 0.028 | 0.036 | 0.09 | 36.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 15.2 |
| pipelines/driver_ms | 0.010 | 0.000 | 4.16 | 13.0 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 9.6 |
| controller/avoid/ms | 0.007 | 0.011 | 0.05 | 8.8 |
| script/ui/ms | 0.006 | 0.008 | 0.34 | 7.4 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 6.9 |
| controller/write/ms | 0.002 | 0.004 | 0.05 | 3.1 |
| physics/insert/ms | 0.002 | 0.000 | 0.68 | 2.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.16 | 1.1 |
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
| anim/evaluated/count | count | 7 | 7 | 7 |
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
| frame/work/ms | ms | 8.77 | 17.20 | 8.43 |
| gpu/timed/ms | ms | 8.67 | 10.74 | 8.32 |
| mesh_allocator_allocations |  meshes | 12.8 k | 12.8 k | 12.8 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 320.34 M | 320.34 M | 320.34 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 274 | 274 | 274 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 729 | 729 | 729 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 274 | 274 | 274 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 729 | 729 | 729 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d/bins | count | 973 | 974 | 973 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1891 | 1891 | 1891 |
| phases/opaque3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d/items | count | 3825 | 3825 | 3825 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 973 | 974 | 973 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1891 | 1891 | 1891 |
| phases/opaque3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d_prepass/items | count | 3825 | 3825 | 3825 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 1216 | 1229 | 1216 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 7371 | 7371 | 7371 |
| phases/shadow/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| phases/shadow/items | count | 4793 | 4820 | 4793 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 145 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 22.60 | 38.42 | 23.08 |
| process/mem_usage | GiB | 1.97 | 1.99 | 1.99 |
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
| started_load_count |  | 4547 | 4549 | 4549 |
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
| stream/models | count | 540 | 540 | 540 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.92 | 49.56 | 35.70 |
| system/mem_usage | % | 60.31 | 60.43 | 60.39 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3710 | 3723 | 3710 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 12.3 k | 12.3 k | 12.3 k |
| views/all/instance_slots | count | 81.9 k | 81.9 k | 81.9 k |
| views/all/items | count | 13.9 k | 13.9 k | 13.9 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 55 | 53 |
| views/all/work_items | count | 96.0 k | 96.0 k | 96.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 1216 | 1229 | 1216 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 7371 | 7371 | 7371 |
| views/cascade/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| views/cascade/items | count | 4793 | 4820 | 4793 |
| views/cascade/pipelines | count | 27 | 30 | 27 |
| views/cascade/work_items | count | 53.9 k | 54.0 k | 53.9 k |
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
| views/main/bins | count | 2494 | 2496 | 2494 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 4914 | 4914 | 4914 |
| views/main/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| views/main/items | count | 9108 | 9108 | 9108 |
| views/main/pipelines | count | 38 | 40 | 38 |
| views/main/work_items | count | 42.0 k | 42.0 k | 42.0 k |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 133 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.73 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.01 | 10 | 13 |
| physics/inserts | count | 0.17 | 77 | 222 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.68 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.63 | 625.2 | main/busy/ms 593, surface/advance_cpu_ms 76.39, surface/cell_upload_ms 0.69, stream/drive/ms 0.29, weather/map/ms 0.10, bigspace/total/ms 0.08, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.02, anim/evaluate/ms 0.02 |
| warm-up | 0.94 | 309.9 | main/busy/ms 634, render/thread/ms 271, render/busy/ms 192, render/acquire/ms 76.44, surface/advance_cpu_ms 76.39, render/graph/ms 5.24, render/present/ms 2.53, stream/drive/ms 0.97, surface/cell_upload_ms 0.69, stream/tick/ms 0.47, terrain/update/ms 0.39, terrain/select/ms 0.24 |
| warm-up | 1.08 | 143.4 | pipelines/wait_ms 2577, render/thread/ms 293, render/busy/ms 212, pipelines/compile_ms 141, pipelines/front_ms 130, render/acquire/ms 76.81, main/busy/ms 61.49, terrain/update/ms 23.86, render/graph/ms 11.76, pipelines/driver_ms 10.93, render/present/ms 4.17, stream/drive/ms 1.05 |
| warm-up | 1.34 | 98.4 | main/busy/ms 70.33, render/thread/ms 53.73, render/busy/ms 52.22, render/graph/ms 4.72, stream/drive/ms 1.71, stream/tick/ms 1.35, render/present/ms 1.27, terrain/update/ms 0.71, bigspace/total/ms 0.60, terrain/select/ms 0.55, render/acquire/ms 0.24, bigspace/high_precision/ms 0.15 |
| warm-up | 3.58 | 30.9 | pipelines/wait_ms 5320, pipelines/compile_ms 54.87, pipelines/front_ms 51.83, main/busy/ms 48.57, render/thread/ms 38.75, render/busy/ms 37.60, render/graph/ms 8.43, pipelines/driver_ms 3.04, stream/drive/ms 1.24, bigspace/total/ms 1.09, physics/insert/ms 0.98, anim/evaluate_cpu/ms 0.97 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on\balmora@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 13%, CPU 16%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **88%** (min 78%) · 2580 MHz (min 2580) · memory 8001 MHz · 82.7 W · 70 °C (max 72) · VRAM 4072 MiB · throttled 100% of samples
