# `exterior_red_mountain`

- bench_ground: false · build: release · camera: [-120.0, 82.1, 80.0] -> [293.0, 250.0, -878.0] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1572 frames over 15.0 s · **median 9.46 ms (106 fps)** · mean 9.54 · p95 10.88 · p99 11.74 (1% low 85 fps) · worst 15.94 ms · 2 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 482 frames over 5.5 s, worst 578.5 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 8.20 ms of a 9.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.46 ms, limit 11 ms
- pass worst frame 15.94 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.275 | 2.903 | 0.181 |
| main_opaque_pass_3d | 1.911 | 2.635 | 0.050 |
| early prepass | 0.727 | 1.401 | 0.081 |
| lightprobe_irradiance_map | 0.452 | 0.749 | 0.001 |
| atmosphere_luts | 0.389 | 0.746 | 0.003 |
| bloom | 0.338 | 0.481 | 0.067 |
| clouds_trace | 0.271 | 0.357 | 0.001 |
| lightprobe_radiance_map | 0.236 | 0.236 | 0.004 |
| ssao | 0.218 | 0.211 | 0.007 |
| uniform_allocation | 0.177 | 0.303 | 0.019 |
| water_fft | 0.134 | 0.128 | 0.003 |
| upscaling | 0.130 | 0.153 | 0.004 |
| bin_unpacking | 0.113 | 0.113 | 0.013 |
| atmos_froxel_inject | 0.094 | 0.092 | 0.001 |
| tonemapping | 0.094 | 0.094 | 0.004 |
| main_transparent_pass_3d | 0.086 | 0.086 | 0.073 |
| water_velocity | 0.081 | 0.075 | 0.002 |
| clustering | 0.070 | 0.073 | 0.022 |
| clouds_composite | 0.068 | 0.066 | 0.002 |
| render_sky | 0.052 | 0.053 | 0.002 |
| clouds_resolve | 0.036 | 0.048 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.006 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.025 | 0.007 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| night_sky | 0.014 | 0.014 | 0.002 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.012 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| auxiliary_views | 0.001 | 0.002 | 0.003 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.209 | 9.449 | 14.56 | 12905.1 |
| render/busy/ms | 7.500 | 8.545 | 12.39 | 11789.7 |
| main/busy/ms | 6.761 | 7.534 | 9.76 | 10627.7 |
| render/graph/ms | 3.601 | 4.363 | 6.09 | 5661.3 |
| render/present/ms | 0.524 | 1.284 | 4.18 | 824.0 |
| bigspace/total/ms | 0.333 | 0.470 | 1.25 | 523.2 |
| stream/drive/ms | 0.239 | 0.423 | 1.01 | 375.5 |
| terrain/update/ms | 0.239 | 0.299 | 0.76 | 374.9 |
| far/select/ms | 0.232 | 0.401 | 0.79 | 365.4 |
| stream/tick/ms | 0.209 | 0.386 | 0.98 | 329.0 |
| terrain/select/ms | 0.206 | 0.258 | 0.72 | 323.6 |
| render/acquire/ms | 0.185 | 0.808 | 2.14 | 291.4 |
| controller/feed/ms | 0.154 | 0.214 | 0.64 | 242.5 |
| controller/move/ms | 0.117 | 0.203 | 0.38 | 184.5 |
| anim/evaluate/ms | 0.109 | 0.202 | 1.21 | 170.9 |
| bigspace/low_precision/ms | 0.103 | 0.151 | 1.03 | 162.2 |
| bigspace/high_precision/ms | 0.093 | 0.130 | 0.87 | 145.8 |
| anim/evaluate_cpu/ms | 0.087 | 0.107 | 0.67 | 136.5 |
| anim/apply/ms | 0.083 | 0.108 | 1.18 | 130.6 |
| physics/step/ms | 0.042 | 0.084 | 0.45 | 66.2 |
| weather/map/ms | 0.027 | 0.035 | 0.37 | 42.0 |
| weather/eval/ms | 0.012 | 0.015 | 0.04 | 18.1 |
| controller/avoid/ms | 0.010 | 0.016 | 0.07 | 16.1 |
| script/update/ms | 0.007 | 0.010 | 0.04 | 11.7 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 8.3 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.8 |
| controller/write/ms | 0.003 | 0.004 | 0.35 | 4.7 |
| shapes/activate/ms | 0.001 | 0.003 | 0.01 | 1.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| physics/insert/ms | 0.001 | 0.000 | 0.37 | 1.1 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 0.9 |
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
| anim/a2/count | count | 1 | 1 | 1 |
| anim/a3/count | count | 502 | 503 | 502 |
| anim/characters/count | count | 503 | 503 | 503 |
| anim/evaluated/count | count | 15 | 15 | 15 |
| controller/movers/count | count | 15 | 15 | 15 |
| ecs/entities | count | 51.4 k | 51.4 k | 51.4 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.23 | 12.39 | 9.09 |
| gpu/timed/ms | ms | 8.19 | 9.93 | 9.09 |
| mesh_allocator_allocations |  meshes | 11.7 k | 11.7 k | 11.7 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.19 M | 433.19 M | 433.19 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 331 | 331 | 331 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 462 | 462 | 462 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 331 | 331 | 331 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 462 | 462 | 462 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d/bins | count | 1018 | 1018 | 1018 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2942 | 2942 | 2942 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d_prepass/bins | count | 1018 | 1018 | 1018 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2942 | 2942 | 2942 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/shadow/batch_sets | count | 25 | 25 | 25 |
| phases/shadow/bins | count | 1713 | 1728 | 1728 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4045 | 4060 | 4060 |
| phases/shadow/pipelines | count | 25 | 25 | 25 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2229 | 2229 | 2229 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 174 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 157 | 157 | 157 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 154 | 154 | 154 |
| preprocess/bind_groups_created | count | 154 | 154 | 154 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 26.67 | 36.19 | 27.73 |
| process/mem_usage | GiB | 2.04 | 2.10 | 2.10 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2657 | 2659 | 2659 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 355 | 355 | 355 |
| statics/materials_bound | count | 355 | 355 | 355 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 214 | 214 | 214 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 23.03 M | 23.03 M | 23.03 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 1124 | 1124 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.22 | 48.52 | 32.30 |
| system/mem_usage | % | 62.30 | 62.46 | 62.46 |
| terrain/patches | count | 391 | 391 | 391 |
| views/all/batch_sets | count | 67 | 67 | 67 |
| views/all/bins | count | 4411 | 4426 | 4426 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 10.9 k | 10.9 k | 10.9 k |
| views/all/pipelines | count | 63 | 63 | 63 |
| views/all/pipelines_distinct | count | 53 | 53 | 53 |
| views/all/work_items | count | 11.1 k | 11.1 k | 11.1 k |
| views/cascade/batch_sets | count | 25 | 25 | 25 |
| views/cascade/bins | count | 1713 | 1728 | 1728 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4045 | 4060 | 4060 |
| views/cascade/pipelines | count | 25 | 25 | 25 |
| views/cascade/work_items | count | 4045 | 4060 | 4060 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 42 | 42 | 42 |
| views/main/bins | count | 2698 | 2698 | 2698 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 6808 | 6808 | 6808 |
| views/main/pipelines | count | 38 | 38 | 38 |
| views/main/work_items | count | 7026 | 7026 | 7026 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 23.9 k | 23.9 k | 23.9 k |
| world/point_lights | count | 200 | 200 | 200 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 6.38 | 34 | 10.0 k |
| anim/events/count | count | 0.17 | 3 | 272 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.15 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| physics/inserts | count | 0.11 | 105 | 175 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 578.5 | main/busy/ms 558, surface/advance_cpu_ms 68.35, surface/cell_upload_ms 0.71, stream/drive/ms 0.24, weather/map/ms 0.08, stream/tick/ms 0.06, bigspace/total/ms 0.05, weather/eval/ms 0.04, bigspace/high_precision/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.94 | 366.4 | main/busy/ms 587, render/thread/ms 333, render/busy/ms 265, surface/advance_cpu_ms 68.35, render/acquire/ms 66.72, render/graph/ms 4.82, render/present/ms 2.02, stream/drive/ms 0.78, surface/cell_upload_ms 0.71, stream/tick/ms 0.41, terrain/update/ms 0.36, terrain/select/ms 0.21 |
| warm-up | 1.20 | 90.1 | main/busy/ms 64.79, render/thread/ms 60.69, render/busy/ms 58.83, render/graph/ms 4.52, render/present/ms 1.49, stream/drive/ms 1.43, stream/tick/ms 1.21, terrain/update/ms 0.66, terrain/select/ms 0.50, render/acquire/ms 0.37, bigspace/total/ms 0.28, bigspace/high_precision/ms 0.12 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --camera -120,+70,80,293,250,-878 --bench benchmarks/exterior_red_mountain.json --resolution 2560x1440 --bench-out target/perf\ab_main\red_mountain@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 7%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **95%** (min 88%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.0 W · 74 °C (max 76) · VRAM 4536 MiB · throttled 100% of samples
