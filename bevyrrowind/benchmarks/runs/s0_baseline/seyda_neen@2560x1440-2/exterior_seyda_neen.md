# `exterior_seyda_neen`

- bench_ground: false · build: release · camera: [-105.0, 8.0, 1100.0] -> [-140.0, 4.5, 1035.0] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1437 frames over 15.0 s · **median 10.27 ms (97 fps)** · mean 10.44 · p95 12.10 · p99 13.70 (1% low 73 fps) · worst 36.27 ms · 3 spikes over 1.5× median · 48.0 stutters/min (work 52.0/min)
- warm-up: 415 frames over 4.9 s, worst 580.3 ms
- hitches (> 25.7 ms): **2** measured, 3 in warm-up
- timed GPU passes: 9.00 ms of a 10.44 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.27 ms, limit 11 ms
- **FAIL** worst frame 36.27 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.288 | 2.930 | 0.187 |
| main_opaque_pass_3d | 2.270 | 2.953 | 0.049 |
| early prepass | 0.722 | 1.369 | 0.085 |
| main_transparent_pass_3d | 0.502 | 0.859 | 0.125 |
| lightprobe_irradiance_map | 0.441 | 0.560 | 0.001 |
| bloom | 0.351 | 0.494 | 0.069 |
| atmosphere_luts | 0.341 | 0.504 | 0.003 |
| lightprobe_radiance_map | 0.237 | 0.233 | 0.004 |
| ssao | 0.227 | 0.357 | 0.007 |
| clouds_trace | 0.203 | 0.282 | 0.001 |
| uniform_allocation | 0.194 | 0.188 | 0.020 |
| upscaling | 0.139 | 0.210 | 0.005 |
| water_fft | 0.138 | 0.130 | 0.003 |
| bin_unpacking | 0.130 | 0.132 | 0.014 |
| water_velocity | 0.103 | 0.099 | 0.002 |
| tonemapping | 0.099 | 0.118 | 0.004 |
| atmos_froxel_inject | 0.097 | 0.104 | 0.002 |
| clustering | 0.076 | 0.081 | 0.022 |
| clouds_composite | 0.066 | 0.065 | 0.002 |
| render_sky | 0.055 | 0.057 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.043 | 0.005 |
| clouds_resolve | 0.035 | 0.046 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.026 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.026 | 0.032 | 0.001 |
| auxiliary_views | 0.025 | 0.125 | 0.027 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.026 | 0.007 |
| lightprobe_copy | 0.018 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.013 | 0.013 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| night_sky | 0.008 | 0.012 | 0.002 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.204 | 10.868 | 34.67 | 13226.1 |
| render/busy/ms | 8.239 | 9.658 | 24.89 | 11839.4 |
| main/busy/ms | 7.209 | 8.254 | 10.08 | 10359.5 |
| render/graph/ms | 4.232 | 5.325 | 20.45 | 6081.8 |
| render/present/ms | 0.515 | 0.719 | 25.20 | 740.4 |
| render/acquire/ms | 0.450 | 1.533 | 5.46 | 646.2 |
| bigspace/total/ms | 0.363 | 0.617 | 1.83 | 521.5 |
| terrain/update/ms | 0.254 | 0.314 | 0.58 | 365.5 |
| terrain/select/ms | 0.222 | 0.272 | 0.55 | 319.0 |
| stream/drive/ms | 0.207 | 0.341 | 0.70 | 297.8 |
| stream/tick/ms | 0.179 | 0.303 | 0.67 | 257.3 |
| controller/feed/ms | 0.173 | 0.239 | 0.58 | 248.2 |
| far/select/ms | 0.148 | 0.254 | 0.61 | 212.0 |
| anim/evaluate_cpu/ms | 0.133 | 0.168 | 0.52 | 191.8 |
| bigspace/low_precision/ms | 0.129 | 0.221 | 1.02 | 185.9 |
| controller/move/ms | 0.113 | 0.181 | 0.72 | 162.7 |
| bigspace/high_precision/ms | 0.103 | 0.220 | 1.22 | 148.4 |
| anim/evaluate/ms | 0.101 | 0.246 | 1.14 | 145.2 |
| anim/apply/ms | 0.062 | 0.095 | 0.92 | 89.3 |
| physics/step/ms | 0.047 | 0.085 | 0.91 | 67.0 |
| weather/map/ms | 0.026 | 0.035 | 0.17 | 37.6 |
| physics/insert/ms | 0.014 | 0.118 | 0.56 | 19.9 |
| weather/eval/ms | 0.012 | 0.016 | 0.16 | 16.9 |
| controller/avoid/ms | 0.011 | 0.016 | 0.37 | 16.0 |
| script/update/ms | 0.007 | 0.010 | 0.12 | 10.7 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 7.6 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.2 |
| controller/write/ms | 0.003 | 0.005 | 0.08 | 4.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 1.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
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
| anim/evaluated/count | count | 21 | 24 | 21 |
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
| frame/work/ms | ms | 8.96 | 24.89 | 8.46 |
| gpu/timed/ms | ms | 8.90 | 13.65 | 8.46 |
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
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1509 | 1510 | 1509 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3765 | 3765 | 3765 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1509 | 1510 | 1509 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3765 | 3765 | 3765 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 34 | 53 | 32 |
| phases/shadow/bins | count | 3607 | 3952 | 3607 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8890 | 9260 | 8894 |
| phases/shadow/pipelines | count | 100 | 104 | 99 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 1601 | 1601 | 1601 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 1649 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 31 | 0 |
| preprocess/bind_groups | count | 176 | 217 | 172 |
| preprocess/bind_groups_created | count | 176 | 217 | 172 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.33 | 41.83 | 16.75 |
| process/mem_usage | GiB | 2.22 | 2.26 | 2.26 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 192 | 192 | 192 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 16.10 M | 16.10 M | 16.10 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 792 | 792 | 792 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.31 | 49.69 | 28.92 |
| system/mem_usage | % | 62.81 | 62.99 | 62.96 |
| terrain/patches | count | 441 | 441 | 441 |
| views/all/batch_sets | count | 78 | 97 | 76 |
| views/all/bins | count | 7285 | 7630 | 7285 |
| views/all/count | count | 16 | 18 | 16 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 18.7 k | 18.3 k |
| views/all/pipelines | count | 140 | 144 | 139 |
| views/all/pipelines_distinct | count | 63 | 65 | 63 |
| views/all/work_items | count | 17.4 k | 17.8 k | 17.4 k |
| views/cascade/batch_sets | count | 34 | 39 | 32 |
| views/cascade/bins | count | 2644 | 2989 | 2642 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7684 | 8058 | 7684 |
| views/cascade/pipelines | count | 34 | 39 | 32 |
| views/cascade/work_items | count | 7684 | 8058 | 7684 |
| views/lamp_face/batch_sets | count | 0 | 19 | 0 |
| views/lamp_face/bins | count | 963 | 965 | 965 |
| views/lamp_face/count | count | 12 | 12 | 12 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1204 | 1210 | 1210 |
| views/lamp_face/pipelines | count | 66 | 67 | 67 |
| views/lamp_face/work_items | count | 0 | 355 | 0 |
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 3678 | 3680 | 3678 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9444 | 9444 | 9444 |
| views/main/pipelines | count | 40 | 42 | 40 |
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
| anim/bones_written/count | count | 11.50 | 21 | 16.5 k |
| anim/events/count | count | 0.05 | 2 | 76 |
| assets/Image/added | count | 0.00 | 4 | 7 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 18 | 34 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 4 | 7 |
| assets/StandardMaterial/modified | count | 1.57 | 5 | 2250 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.59 | 477 | 849 |
| physics/inserts | count | 0.65 | 146 | 935 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 4 | 6 |
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

## Hitches (> 25.7 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.3 | main/busy/ms 558, surface/advance_cpu_ms 68.34, surface/cell_upload_ms 0.64, stream/drive/ms 0.31, weather/map/ms 0.11, bigspace/total/ms 0.10, stream/tick/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.03, script/input/ms 0.01 |
| warm-up | 0.98 | 400.3 | main/busy/ms 592, render/thread/ms 366, render/busy/ms 295, render/acquire/ms 69.43, surface/advance_cpu_ms 68.34, render/graph/ms 4.90, render/present/ms 1.88, stream/drive/ms 0.78, surface/cell_upload_ms 0.64, terrain/update/ms 0.47, stream/tick/ms 0.38, terrain/select/ms 0.30 |
| warm-up | 1.25 | 92.4 | main/busy/ms 63.26, render/thread/ms 50.30, render/busy/ms 48.58, render/graph/ms 4.34, render/present/ms 1.48, stream/drive/ms 1.28, terrain/update/ms 1.05, stream/tick/ms 1.03, terrain/select/ms 0.88, render/acquire/ms 0.24, bigspace/total/ms 0.19, weather/map/ms 0.09 |
| measure | 6.11 | 26.8 | render/thread/ms 35.60, render/busy/ms 34.49, render/graph/ms 25.81, main/busy/ms 15.48, bigspace/total/ms 0.95, render/present/ms 0.93, terrain/update/ms 0.48, terrain/select/ms 0.42, controller/feed/ms 0.37, stream/drive/ms 0.36, stream/tick/ms 0.31, anim/evaluate_cpu/ms 0.29 |
| measure | 8.13 | 36.3 | render/thread/ms 45.00, render/present/ms 25.90, render/busy/ms 18.91, main/busy/ms 15.13, render/graph/ms 10.20, bigspace/total/ms 0.71, terrain/update/ms 0.50, terrain/select/ms 0.43, controller/feed/ms 0.41, stream/drive/ms 0.40, stream/tick/ms 0.34, anim/evaluate_cpu/ms 0.30 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --camera -105,+8,1100,-140,+3,1035 --bench benchmarks/exterior_seyda_neen.json --resolution 2560x1440 --bench-out target/perf\ab_main\seyda_neen@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 13%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **96%** (min 65%) · 2580 MHz (min 2580) · memory 8001 MHz · 89.5 W · 73 °C (max 76) · VRAM 4277 MiB · throttled 100% of samples
