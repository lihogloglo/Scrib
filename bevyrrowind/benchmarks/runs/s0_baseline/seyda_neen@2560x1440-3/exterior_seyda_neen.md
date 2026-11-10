# `exterior_seyda_neen`

- bench_ground: false · build: release · camera: [-105.0, 8.0, 1100.0] -> [-140.0, 4.5, 1035.0] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1490 frames over 15.0 s · **median 9.98 ms (100 fps)** · mean 10.07 · p95 11.53 · p99 12.69 (1% low 79 fps) · worst 15.02 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 461 frames over 5.4 s, worst 580.1 ms
- hitches (> 25.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 8.68 ms of a 10.07 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.98 ms, limit 11 ms
- pass worst frame 15.02 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.213 | 2.586 | 0.187 |
| main_opaque_pass_3d | 2.140 | 2.565 | 0.044 |
| early prepass | 0.693 | 1.023 | 0.079 |
| main_transparent_pass_3d | 0.491 | 0.842 | 0.126 |
| lightprobe_irradiance_map | 0.433 | 0.558 | 0.001 |
| bloom | 0.339 | 0.491 | 0.067 |
| atmosphere_luts | 0.332 | 0.503 | 0.003 |
| lightprobe_radiance_map | 0.231 | 0.236 | 0.004 |
| ssao | 0.220 | 0.355 | 0.007 |
| clouds_trace | 0.201 | 0.262 | 0.001 |
| uniform_allocation | 0.189 | 0.185 | 0.017 |
| upscaling | 0.134 | 0.161 | 0.004 |
| water_fft | 0.134 | 0.131 | 0.003 |
| bin_unpacking | 0.127 | 0.129 | 0.012 |
| water_velocity | 0.100 | 0.098 | 0.002 |
| atmos_froxel_inject | 0.095 | 0.103 | 0.001 |
| tonemapping | 0.094 | 0.095 | 0.003 |
| clustering | 0.077 | 0.081 | 0.021 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| render_sky | 0.055 | 0.056 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.044 | 0.005 |
| clouds_resolve | 0.035 | 0.042 | 0.001 |
| auxiliary_views | 0.033 | 0.210 | 0.035 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.033 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.025 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.025 | 0.007 |
| lightprobe_copy | 0.014 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| night_sky | 0.008 | 0.011 | 0.002 |
| sparse buffer updates | 0.006 | 0.008 | 0.003 |
| blas_build | 0.002 | 0.004 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.840 | 10.243 | 13.73 | 13171.2 |
| render/busy/ms | 7.972 | 9.287 | 13.16 | 11877.9 |
| main/busy/ms | 7.135 | 8.076 | 10.46 | 10631.3 |
| render/graph/ms | 4.081 | 5.007 | 7.86 | 6080.1 |
| render/present/ms | 0.492 | 1.126 | 1.67 | 733.5 |
| render/acquire/ms | 0.376 | 1.250 | 2.43 | 559.8 |
| bigspace/total/ms | 0.360 | 0.637 | 1.47 | 536.4 |
| terrain/update/ms | 0.255 | 0.318 | 0.76 | 379.4 |
| terrain/select/ms | 0.222 | 0.275 | 0.72 | 331.3 |
| stream/drive/ms | 0.199 | 0.344 | 0.57 | 296.8 |
| controller/feed/ms | 0.175 | 0.237 | 0.66 | 260.8 |
| stream/tick/ms | 0.172 | 0.310 | 0.55 | 256.6 |
| far/select/ms | 0.146 | 0.258 | 0.54 | 217.2 |
| anim/evaluate/ms | 0.141 | 0.270 | 1.60 | 209.5 |
| anim/evaluate_cpu/ms | 0.136 | 0.174 | 0.37 | 202.8 |
| bigspace/low_precision/ms | 0.131 | 0.236 | 0.90 | 195.3 |
| controller/move/ms | 0.121 | 0.185 | 0.39 | 180.8 |
| bigspace/high_precision/ms | 0.098 | 0.166 | 1.25 | 146.5 |
| anim/apply/ms | 0.062 | 0.096 | 1.25 | 92.0 |
| physics/step/ms | 0.046 | 0.088 | 0.60 | 69.1 |
| weather/map/ms | 0.026 | 0.035 | 0.17 | 39.3 |
| weather/eval/ms | 0.012 | 0.016 | 0.48 | 17.7 |
| controller/avoid/ms | 0.011 | 0.016 | 0.05 | 17.0 |
| script/update/ms | 0.007 | 0.012 | 0.04 | 10.4 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.3 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.5 |
| controller/write/ms | 0.004 | 0.005 | 0.02 | 5.2 |
| physics/insert/ms | 0.003 | 0.000 | 0.54 | 4.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.4 |
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
| anim/a2/count | count | 2 | 2 | 2 |
| anim/a3/count | count | 380 | 382 | 380 |
| anim/characters/count | count | 382 | 382 | 382 |
| anim/evaluated/count | count | 20 | 25 | 20 |
| controller/movers/count | count | 22 | 22 | 22 |
| ecs/entities | count | 42.6 k | 42.6 k | 42.6 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.70 | 13.16 | 9.21 |
| gpu/timed/ms | ms | 8.65 | 9.78 | 9.21 |
| mesh_allocator_allocations |  meshes | 11.6 k | 11.6 k | 11.6 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 378.52 M | 378.52 M | 378.52 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 957 | 957 | 957 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 957 | 957 | 957 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 15 | 15 |
| phases/opaque3d/bins | count | 1509 | 1509 | 1509 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3765 | 3765 | 3765 |
| phases/opaque3d/pipelines | count | 14 | 14 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 15 | 15 |
| phases/opaque3d_prepass/bins | count | 1509 | 1509 | 1509 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3765 | 3765 | 3765 |
| phases/opaque3d_prepass/pipelines | count | 14 | 14 | 14 |
| phases/shadow/batch_sets | count | 32 | 64 | 34 |
| phases/shadow/bins | count | 3609 | 3952 | 3609 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8894 | 9286 | 8892 |
| phases/shadow/pipelines | count | 101 | 104 | 101 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 1601 | 1601 | 1601 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 303 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 172 | 241 | 176 |
| preprocess/bind_groups_created | count | 172 | 241 | 176 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 28.37 | 39.42 | 39.34 |
| process/mem_usage | GiB | 2.15 | 2.22 | 2.22 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2488 | 2489 | 2489 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 372 | 372 | 372 |
| statics/materials_bound | count | 372 | 372 | 372 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 192 | 192 | 192 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 16.10 M | 16.10 M | 16.10 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 792 | 792 | 792 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.67 | 49.00 | 39.54 |
| system/mem_usage | % | 62.59 | 62.72 | 62.72 |
| terrain/patches | count | 441 | 441 | 441 |
| views/all/batch_sets | count | 76 | 108 | 78 |
| views/all/bins | count | 7287 | 7630 | 7287 |
| views/all/count | count | 16 | 18 | 16 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 18.7 k | 18.3 k |
| views/all/pipelines | count | 141 | 144 | 141 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 17.4 k | 18.0 k | 17.4 k |
| views/cascade/batch_sets | count | 32 | 39 | 34 |
| views/cascade/bins | count | 2642 | 2989 | 2644 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7686 | 8084 | 7686 |
| views/cascade/pipelines | count | 32 | 39 | 34 |
| views/cascade/work_items | count | 7686 | 8084 | 7686 |
| views/lamp_face/batch_sets | count | 0 | 32 | 0 |
| views/lamp_face/bins | count | 965 | 967 | 965 |
| views/lamp_face/count | count | 12 | 12 | 12 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1208 | 1212 | 1206 |
| views/lamp_face/pipelines | count | 67 | 69 | 67 |
| views/lamp_face/work_items | count | 0 | 542 | 0 |
| views/main/batch_sets | count | 44 | 44 | 44 |
| views/main/bins | count | 3678 | 3678 | 3678 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9444 | 9444 | 9444 |
| views/main/pipelines | count | 40 | 40 | 40 |
| views/main/work_items | count | 9727 | 9727 | 9727 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 23.0 k | 23.0 k | 23.0 k |
| world/point_lights | count | 394 | 394 | 394 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 12.27 | 42 | 18.3 k |
| anim/events/count | count | 0.08 | 2 | 121 |
| assets/Image/added | count | 0.01 | 16 | 22 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.09 | 82 | 133 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 16 | 22 |
| assets/StandardMaterial/modified | count | 1.51 | 5 | 2250 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 657 | 1257 |
| physics/inserts | count | 0.21 | 117 | 315 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.1 | main/busy/ms 559, surface/advance_cpu_ms 67.77, surface/cell_upload_ms 0.63, stream/drive/ms 0.25, weather/map/ms 0.12, stream/tick/ms 0.06, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.03, script/update/ms 0.03, weather/eval/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.88 | 295.7 | main/busy/ms 592, render/thread/ms 260, render/busy/ms 192, surface/advance_cpu_ms 67.77, render/acquire/ms 65.82, render/graph/ms 5.32, render/present/ms 2.19, stream/drive/ms 0.81, surface/cell_upload_ms 0.63, terrain/update/ms 0.57, stream/tick/ms 0.44, terrain/select/ms 0.38 |
| warm-up | 0.96 | 85.5 | render/thread/ms 278, render/busy/ms 209, render/acquire/ms 66.11, main/busy/ms 48.65, terrain/update/ms 18.29, render/graph/ms 10.29, render/present/ms 3.47, stream/drive/ms 0.91, terrain/select/ms 0.68, stream/tick/ms 0.65, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.15 |
| warm-up | 1.18 | 79.7 | main/busy/ms 55.72, render/thread/ms 48.89, render/busy/ms 47.14, render/graph/ms 3.14, render/present/ms 1.58, stream/drive/ms 1.15, stream/tick/ms 0.94, terrain/update/ms 0.92, terrain/select/ms 0.75, bigspace/total/ms 0.26, render/acquire/ms 0.18, bigspace/high_precision/ms 0.12 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --camera -105,+8,1100,-140,+3,1035 --bench benchmarks/exterior_seyda_neen.json --resolution 2560x1440 --bench-out target/perf\ab_main\seyda_neen@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 12%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 94%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.7 W · 74 °C (max 77) · VRAM 4297 MiB · throttled 100% of samples
