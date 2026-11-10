# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1649 frames over 15.0 s · **median 8.90 ms (112 fps)** · mean 9.10 · p95 10.83 · p99 12.55 (1% low 80 fps) · worst 23.22 ms · 11 spikes over 1.5× median · 32.0 stutters/min (work 16.0/min)
- warm-up: 291 frames over 3.9 s, worst 508.4 ms
- hitches (> 25.0 ms): **0** measured, 6 in warm-up
- timed GPU passes: 8.27 ms of a 9.10 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.90 ms, limit 11 ms
- pass worst frame 23.22 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.434 | 2.905 | 0.043 |
| dlss_super_resolution | 2.146 | 2.595 | 0.187 |
| early prepass | 0.626 | 0.886 | 0.078 |
| lightprobe_irradiance_map | 0.425 | 0.421 | 0.001 |
| atmosphere_luts | 0.399 | 0.631 | 0.003 |
| bloom | 0.336 | 0.479 | 0.068 |
| ssao | 0.235 | 0.467 | 0.008 |
| lightprobe_radiance_map | 0.229 | 0.237 | 0.004 |
| main_transparent_pass_3d | 0.203 | 0.232 | 0.072 |
| water_fft | 0.135 | 0.133 | 0.003 |
| uniform_allocation | 0.134 | 0.134 | 0.015 |
| upscaling | 0.120 | 0.139 | 0.004 |
| bin_unpacking | 0.097 | 0.101 | 0.009 |
| tonemapping | 0.090 | 0.092 | 0.004 |
| atmos_froxel_inject | 0.081 | 0.083 | 0.001 |
| water_velocity | 0.077 | 0.076 | 0.002 |
| clouds_trace | 0.068 | 0.088 | 0.001 |
| clustering | 0.067 | 0.076 | 0.022 |
| clouds_composite | 0.066 | 0.065 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| clouds_resolve | 0.035 | 0.039 | 0.001 |
| early_mesh_preprocessing | 0.032 | 0.033 | 0.005 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.024 | 0.028 | 0.001 |
| lightprobe_copy | 0.016 | 0.016 | 0.004 |
| per_view_shadow_pass | 0.015 | 0.018 | 0.005 |
| lightprobe_downsampling_first_pass | 0.015 | 0.015 | 0.001 |
| lightprobe_downsampling_second_pass | 0.013 | 0.012 | 0.001 |
| atmosphere_environment | 0.013 | 0.013 | 0.003 |
| clouds_fog_light | 0.011 | 0.012 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.001 | 0.007 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.004 | 9.603 | 22.01 | 13198.2 |
| render/busy/ms | 7.033 | 8.565 | 21.02 | 11597.1 |
| main/busy/ms | 6.595 | 7.773 | 14.92 | 10874.5 |
| render/graph/ms | 3.644 | 4.721 | 17.67 | 6008.4 |
| render/present/ms | 0.491 | 0.910 | 2.90 | 810.1 |
| render/acquire/ms | 0.480 | 1.426 | 2.35 | 791.0 |
| bigspace/total/ms | 0.341 | 0.677 | 1.70 | 562.5 |
| terrain/update/ms | 0.230 | 0.299 | 0.71 | 380.0 |
| stream/drive/ms | 0.214 | 0.364 | 0.67 | 352.8 |
| terrain/select/ms | 0.200 | 0.258 | 0.68 | 329.8 |
| far/select/ms | 0.188 | 0.314 | 0.52 | 310.3 |
| stream/tick/ms | 0.184 | 0.327 | 0.63 | 303.5 |
| controller/feed/ms | 0.145 | 0.209 | 0.44 | 238.7 |
| bigspace/high_precision/ms | 0.118 | 0.248 | 1.24 | 194.9 |
| controller/move/ms | 0.113 | 0.259 | 0.41 | 186.6 |
| anim/evaluate/ms | 0.087 | 0.184 | 0.87 | 142.8 |
| bigspace/low_precision/ms | 0.080 | 0.173 | 1.17 | 131.5 |
| anim/apply/ms | 0.077 | 0.161 | 0.97 | 126.6 |
| anim/evaluate_cpu/ms | 0.053 | 0.073 | 0.30 | 87.7 |
| physics/step/ms | 0.032 | 0.065 | 1.16 | 53.4 |
| weather/map/ms | 0.017 | 0.022 | 0.18 | 27.6 |
| physics/insert/ms | 0.012 | 0.000 | 0.67 | 20.5 |
| weather/eval/ms | 0.010 | 0.013 | 0.07 | 16.6 |
| script/update/ms | 0.007 | 0.014 | 0.05 | 12.3 |
| controller/avoid/ms | 0.007 | 0.010 | 0.03 | 10.8 |
| script/input/ms | 0.005 | 0.007 | 0.01 | 8.8 |
| script/ui/ms | 0.005 | 0.008 | 0.07 | 8.4 |
| controller/write/ms | 0.002 | 0.003 | 0.30 | 3.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.10 | 1.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.04 | 1.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| anim/a2/count | count | 0 | 2 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 13 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.21 | 21.02 | 8.36 |
| gpu/timed/ms | ms | 8.18 | 10.68 | 8.36 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1979 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 31 | 0 |
| process/cpu_usage | % | 29.69 | 39.90 | 29.67 |
| process/mem_usage | GiB | 2.14 | 2.21 | 2.21 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2585 | 2587 | 2587 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.20 | 69.40 | 36.64 |
| system/mem_usage | % | 45.38 | 45.65 | 45.65 |
| terrain/patches | count | 370 | 370 | 370 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.04 | 66 | 66 |
| anim/events/count | count | 0.07 | 4 | 117 |
| assets/Image/added | count | 0.02 | 16 | 28 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.06 | 58 | 106 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 17 | 29 |
| assets/StandardMaterial/modified | count | 3.00 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.35 | 682 | 2231 |
| physics/inserts | count | 0.99 | 116 | 1630 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.55 | 2 | 900 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 508.4 | main/busy/ms 488, surface/advance_cpu_ms 0.74, surface/cell_upload_ms 0.45, stream/drive/ms 0.30, stream/tick/ms 0.08, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, weather/map/ms 0.03, script/update/ms 0.02, weather/eval/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 0.77 | 260.9 | main/busy/ms 523, render/thread/ms 225, render/busy/ms 178, render/acquire/ms 44.68, render/graph/ms 7.37, render/present/ms 1.93, stream/drive/ms 1.18, surface/advance_cpu_ms 0.74, stream/tick/ms 0.64, surface/cell_upload_ms 0.45, terrain/update/ms 0.41, terrain/select/ms 0.28 |
| warm-up | 0.88 | 108.5 | render/thread/ms 242, render/busy/ms 194, main/busy/ms 50.36, render/acquire/ms 45.04, terrain/update/ms 22.15, render/graph/ms 12.45, render/present/ms 3.12, stream/drive/ms 1.22, stream/tick/ms 0.84, terrain/select/ms 0.64, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.10 |
| warm-up | 1.10 | 80.9 | main/busy/ms 55.74, render/thread/ms 45.56, render/busy/ms 43.87, render/graph/ms 4.85, render/present/ms 1.48, stream/drive/ms 1.16, stream/tick/ms 0.96, terrain/update/ms 0.67, terrain/select/ms 0.51, bigspace/total/ms 0.28, render/acquire/ms 0.22, bigspace/high_precision/ms 0.13 |
| warm-up | 1.96 | 29.6 | main/busy/ms 36.84, render/thread/ms 32.00, render/busy/ms 30.75, render/graph/ms 5.57, stream/drive/ms 2.23, render/present/ms 1.01, stream/spawn/ms 0.89, bigspace/total/ms 0.72, stream/tick/ms 0.65, terrain/update/ms 0.54, terrain/select/ms 0.44, anim/evaluate_cpu/ms 0.37 |
| warm-up | 2.10 | 25.4 | main/busy/ms 32.52, render/thread/ms 17.56, render/busy/ms 16.38, render/graph/ms 4.32, stream/drive/ms 1.28, render/present/ms 0.94, bigspace/total/ms 0.69, stream/tick/ms 0.61, terrain/update/ms 0.58, terrain/select/ms 0.45, stream/spawn/ms 0.36, anim/evaluate/ms 0.28 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **96%** (min 84%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.5 W · 72 °C (max 76) · VRAM 3677 MiB · throttled 100% of samples
