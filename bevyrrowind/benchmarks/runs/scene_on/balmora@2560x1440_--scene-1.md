# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1257 frames over 15.0 s · **median 11.81 ms (85 fps)** · mean 11.94 · p95 13.14 · p99 14.57 (1% low 69 fps) · worst 20.12 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 12.0/min)
- warm-up: 317 frames over 5.2 s, worst 597.6 ms
- hitches (> 29.5 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.73 ms of a 11.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.81 ms, limit 11 ms
- pass worst frame 20.12 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.388 | 3.109 | 0.060 |
| dlss_super_resolution | 2.365 | 3.036 | 0.189 |
| early prepass | 0.598 | 1.006 | 0.090 |
| lightprobe_irradiance_map | 0.475 | 0.906 | 0.001 |
| bloom | 0.368 | 0.689 | 0.072 |
| atmosphere_luts | 0.357 | 0.490 | 0.003 |
| main_transparent_pass_3d | 0.241 | 0.639 | 0.070 |
| lightprobe_radiance_map | 0.223 | 0.235 | 0.004 |
| ssao | 0.221 | 0.214 | 0.008 |
| uniform_allocation | 0.136 | 0.137 | 0.020 |
| clouds_trace | 0.134 | 0.153 | 0.001 |
| water_fft | 0.127 | 0.129 | 0.003 |
| upscaling | 0.123 | 0.126 | 0.005 |
| bin_unpacking | 0.103 | 0.104 | 0.014 |
| tonemapping | 0.090 | 0.092 | 0.004 |
| atmos_froxel_inject | 0.090 | 0.093 | 0.002 |
| gpu_instance_materialization | 0.075 | 0.335 | 0.004 |
| water_velocity | 0.074 | 0.077 | 0.002 |
| early_mesh_preprocessing | 0.068 | 0.067 | 0.004 |
| clouds_composite | 0.063 | 0.064 | 0.002 |
| clustering | 0.063 | 0.066 | 0.022 |
| render_sky | 0.054 | 0.061 | 0.002 |
| per_view_shadow_pass | 0.043 | 0.038 | 0.009 |
| clouds_resolve | 0.038 | 0.048 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.032 | 0.001 |
| water_mips | 0.026 | 0.024 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.002 |
| atmosphere_environment | 0.018 | 0.013 | 0.003 |
| range_unpacking | 0.016 | 0.018 | 0.004 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.003 |
| sparse buffer updates | 0.011 | 0.009 | 0.003 |
| lightprobe_downsampling_second_pass | 0.010 | 0.011 | 0.001 |
| night_sky | 0.004 | 0.004 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.012 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 10.202 | 0.000 | 3836.31 | 12823.4 |
| render/thread/ms | 9.266 | 10.317 | 18.99 | 11647.8 |
| render/busy/ms | 8.698 | 9.676 | 16.14 | 10933.4 |
| main/busy/ms | 7.899 | 8.872 | 14.36 | 9928.5 |
| render/graph/ms | 3.929 | 4.586 | 7.33 | 4938.2 |
| render/present/ms | 0.477 | 0.590 | 4.98 | 600.0 |
| bigspace/total/ms | 0.346 | 0.597 | 1.82 | 435.0 |
| terrain/update/ms | 0.229 | 0.285 | 0.64 | 287.5 |
| controller/move/ms | 0.197 | 0.389 | 1.06 | 247.1 |
| terrain/select/ms | 0.196 | 0.244 | 0.59 | 246.0 |
| stream/drive/ms | 0.188 | 0.235 | 0.83 | 236.2 |
| far/select/ms | 0.184 | 0.296 | 0.77 | 230.7 |
| controller/feed/ms | 0.164 | 0.218 | 0.46 | 205.5 |
| stream/tick/ms | 0.157 | 0.201 | 0.77 | 197.4 |
| pipelines/compile_ms | 0.148 | 0.000 | 129.64 | 186.4 |
| pipelines/front_ms | 0.143 | 0.000 | 126.21 | 180.1 |
| anim/evaluate/ms | 0.096 | 0.267 | 1.23 | 120.6 |
| bigspace/high_precision/ms | 0.096 | 0.172 | 0.80 | 120.4 |
| render/acquire/ms | 0.091 | 0.119 | 0.27 | 114.4 |
| anim/apply/ms | 0.086 | 0.219 | 1.18 | 108.6 |
| bigspace/low_precision/ms | 0.074 | 0.095 | 0.69 | 92.4 |
| anim/evaluate_cpu/ms | 0.067 | 0.081 | 0.33 | 83.7 |
| physics/step/ms | 0.037 | 0.063 | 0.58 | 47.1 |
| weather/map/ms | 0.028 | 0.036 | 0.16 | 34.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.05 | 14.9 |
| script/update/ms | 0.008 | 0.010 | 0.14 | 10.0 |
| controller/avoid/ms | 0.007 | 0.011 | 0.18 | 8.8 |
| script/input/ms | 0.006 | 0.007 | 0.26 | 7.6 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 6.5 |
| pipelines/driver_ms | 0.005 | 0.000 | 3.43 | 6.2 |
| physics/insert/ms | 0.005 | 0.000 | 0.80 | 6.1 |
| controller/write/ms | 0.002 | 0.004 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| frame/work/ms | ms | 8.91 | 16.14 | 8.51 |
| gpu/timed/ms | ms | 8.66 | 11.20 | 8.51 |
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
| phases/opaque3d/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d/bins | count | 954 | 954 | 954 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1903 | 1903 | 1903 |
| phases/opaque3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d/items | count | 3696 | 3696 | 3696 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 954 | 954 | 954 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1903 | 1903 | 1903 |
| phases/opaque3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d_prepass/items | count | 3696 | 3696 | 3696 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 1199 | 1212 | 1199 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 7407 | 7407 | 7407 |
| phases/shadow/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| phases/shadow/items | count | 4649 | 4677 | 4649 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 167 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 150 | 150 | 150 |
| pipelines/waiting | count | 0 | 4 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 21.63 | 33.65 | 22.17 |
| process/mem_usage | GiB | 1.94 | 1.96 | 1.96 |
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
| started_load_count |  | 4543 | 4545 | 4545 |
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
| stream/models | count | 536 | 536 | 536 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.12 | 49.02 | 34.77 |
| system/mem_usage | % | 61.46 | 61.60 | 61.48 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3653 | 3666 | 3653 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 12.3 k | 12.3 k | 12.3 k |
| views/all/instance_slots | count | 81.9 k | 81.9 k | 81.9 k |
| views/all/items | count | 13.5 k | 13.5 k | 13.5 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 53 | 53 |
| views/all/work_items | count | 95.6 k | 95.6 k | 95.6 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 1199 | 1212 | 1199 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 7407 | 7407 | 7407 |
| views/cascade/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| views/cascade/items | count | 4649 | 4677 | 4649 |
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
| views/main/batch_sets | count | 50 | 50 | 50 |
| views/main/bins | count | 2454 | 2454 | 2454 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 4938 | 4938 | 4938 |
| views/main/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| views/main/items | count | 8836 | 8836 | 8836 |
| views/main/pipelines | count | 38 | 38 | 38 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.11 | 3 | 135 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.94 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| physics/inserts | count | 0.13 | 128 | 167 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.72 | 1 | 900 |
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

## Hitches (> 29.5 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.60 | 597.6 | main/busy/ms 574, surface/advance_cpu_ms 71.49, surface/cell_upload_ms 0.69, stream/drive/ms 0.33, weather/map/ms 0.11, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.94 | 343.8 | main/busy/ms 610, render/thread/ms 301, render/busy/ms 225, render/acquire/ms 73.28, surface/advance_cpu_ms 71.49, render/graph/ms 5.54, render/present/ms 2.71, stream/drive/ms 0.93, surface/cell_upload_ms 0.69, stream/tick/ms 0.51, terrain/update/ms 0.35, terrain/select/ms 0.22 |
| warm-up | 1.08 | 136.8 | pipelines/wait_ms 3491, render/thread/ms 321, render/busy/ms 243, pipelines/compile_ms 147, pipelines/front_ms 136, render/acquire/ms 73.74, main/busy/ms 56.96, terrain/update/ms 20.91, render/graph/ms 12.17, pipelines/driver_ms 11.12, render/present/ms 4.49, stream/drive/ms 0.96 |
| warm-up | 1.32 | 93.6 | pipelines/compile_ms 223, pipelines/front_ms 198, main/busy/ms 65.47, render/thread/ms 58.41, render/busy/ms 56.83, pipelines/driver_ms 25.69, pipelines/wait_ms 15.93, render/graph/ms 4.93, stream/drive/ms 2.52, stream/tick/ms 1.94, render/present/ms 1.34, terrain/update/ms 0.69 |
| warm-up | 3.71 | 36.3 | main/busy/ms 42.14, render/thread/ms 35.22, render/busy/ms 33.86, render/graph/ms 8.74, stream/drive/ms 1.14, render/present/ms 1.11, bigspace/total/ms 0.81, stream/tick/ms 0.71, terrain/update/ms 0.47, physics/insert/ms 0.40, terrain/select/ms 0.39, anim/evaluate_cpu/ms 0.36 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on\balmora@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **84%** (min 77%) · 2589 MHz (min 2580) · memory 8001 MHz · 79.0 W · 66 °C (max 69) · VRAM 4072 MiB · throttled 100% of samples
