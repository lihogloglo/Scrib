# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5930 frames over 65.6 s · **median 10.78 ms (93 fps)** · mean 11.07 · p95 13.57 · p99 15.32 (1% low 65 fps) · worst 24.80 ms · 32 spikes over 1.5× median · 10.1 stutters/min (work 18.3/min)
- warm-up: 445 frames over 5.4 s, worst 594.7 ms
- hitches (> 27.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 7.22 ms of a 11.07 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.78 ms, limit 11 ms
- **FAIL** frame_ms: 11 stutters (> 1.5x moving median of 60) in 1.09 min = 10.05/min, limit 1/min
- pass worst frame 24.80 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.111 | 2.367 | 0.182 |
| main_opaque_pass_3d | 1.405 | 1.850 | 0.044 |
| early prepass | 0.483 | 0.915 | 0.078 |
| lightprobe_irradiance_map | 0.432 | 0.645 | 0.001 |
| bloom | 0.323 | 0.329 | 0.068 |
| atmosphere_luts | 0.319 | 0.429 | 0.003 |
| lightprobe_radiance_map | 0.226 | 0.236 | 0.004 |
| ssao | 0.209 | 0.208 | 0.007 |
| uniform_allocation | 0.190 | 0.404 | 0.017 |
| auxiliary_views | 0.176 | 0.614 | 0.160 |
| bin_unpacking | 0.129 | 0.151 | 0.013 |
| clouds_trace | 0.127 | 0.218 | 0.001 |
| water_fft | 0.127 | 0.127 | 0.002 |
| upscaling | 0.122 | 0.156 | 0.004 |
| atmos_froxel_inject | 0.094 | 0.100 | 0.001 |
| tonemapping | 0.088 | 0.090 | 0.003 |
| water_velocity | 0.083 | 0.085 | 0.001 |
| clustering | 0.065 | 0.067 | 0.019 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.103 | 0.105 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.048 | 0.005 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.577 | 11.996 | 23.15 | 56791.8 |
| render/busy/ms | 9.043 | 11.428 | 22.37 | 53623.6 |
| main/busy/ms | 8.067 | 9.809 | 13.69 | 47837.9 |
| render/graph/ms | 4.487 | 5.900 | 13.27 | 26605.0 |
| render/present/ms | 0.440 | 0.553 | 4.29 | 2606.6 |
| bigspace/total/ms | 0.409 | 0.614 | 2.49 | 2425.3 |
| terrain/update/ms | 0.278 | 0.355 | 1.51 | 1646.8 |
| controller/feed/ms | 0.248 | 0.337 | 0.94 | 1471.2 |
| terrain/select/ms | 0.246 | 0.312 | 0.73 | 1456.7 |
| stream/drive/ms | 0.232 | 0.388 | 3.33 | 1374.6 |
| far/select/ms | 0.215 | 0.364 | 0.73 | 1276.5 |
| stream/tick/ms | 0.189 | 0.291 | 0.69 | 1117.9 |
| anim/evaluate_cpu/ms | 0.165 | 0.206 | 0.45 | 976.5 |
| controller/move/ms | 0.160 | 0.308 | 0.64 | 949.9 |
| bigspace/low_precision/ms | 0.150 | 0.232 | 1.31 | 887.3 |
| anim/evaluate/ms | 0.147 | 0.283 | 1.74 | 871.2 |
| bigspace/high_precision/ms | 0.096 | 0.152 | 1.76 | 571.6 |
| render/acquire/ms | 0.095 | 0.124 | 0.30 | 561.6 |
| anim/apply/ms | 0.088 | 0.140 | 1.41 | 521.2 |
| physics/step/ms | 0.049 | 0.090 | 0.65 | 293.1 |
| physics/insert/ms | 0.045 | 0.405 | 0.87 | 265.1 |
| weather/map/ms | 0.025 | 0.034 | 0.25 | 150.3 |
| controller/avoid/ms | 0.012 | 0.019 | 0.46 | 72.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 60.3 |
| script/update/ms | 0.007 | 0.009 | 0.12 | 39.6 |
| script/input/ms | 0.005 | 0.007 | 0.30 | 31.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.03 | 30.2 |
| script/ui/ms | 0.005 | 0.006 | 0.10 | 27.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 26.9 |
| controller/write/ms | 0.004 | 0.007 | 0.15 | 23.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 14.3 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 5.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 23 | 18 |
| ecs/entities | count | 59.1 k | 68.1 k | 59.1 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.81 | 22.37 | 8.95 |
| gpu/timed/ms | ms | 7.27 | 10.63 | 8.26 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 598 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 22 | 0 |
| process/cpu_usage | % | 23.56 | 36.54 | 29.33 |
| process/mem_usage | GiB | 2.53 | 2.59 | 2.59 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3462 | 3532 | 3532 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.15 | 60.63 | 31.06 |
| system/mem_usage | % | 45.76 | 46.04 | 46.04 |
| terrain/patches | count | 511 | 523 | 514 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.96 | 224 | 308.1 k |
| anim/events/count | count | 0.25 | 4 | 1475 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.69 | 160 | 4086 |
| assets/Mesh/removed | count | 0.82 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.65 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 0.81 | 1328 | 4815 |
| physics/inserts | count | 0.48 | 125 | 2857 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5709 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1252 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 594.7 | main/busy/ms 574, surface/advance_cpu_ms 73.84, surface/cell_upload_ms 1.18, stream/drive/ms 0.32, weather/map/ms 0.13, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.08, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, weather/apply/ms 0.01 |
| warm-up | 1.00 | 401.0 | main/busy/ms 614, render/thread/ms 366, render/busy/ms 318, surface/advance_cpu_ms 73.84, render/acquire/ms 45.93, render/graph/ms 7.61, render/present/ms 1.95, surface/cell_upload_ms 1.18, stream/drive/ms 1.11, stream/tick/ms 0.59, terrain/update/ms 0.44, terrain/select/ms 0.30 |
| warm-up | 1.04 | 44.6 | render/thread/ms 384, render/busy/ms 335, main/busy/ms 57.46, render/acquire/ms 46.19, terrain/update/ms 26.55, render/graph/ms 12.59, render/present/ms 3.54, stream/drive/ms 1.30, stream/tick/ms 0.89, terrain/select/ms 0.84, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.17 |
| warm-up | 1.26 | 88.6 | main/busy/ms 68.10, render/thread/ms 54.19, render/busy/ms 52.42, render/graph/ms 4.30, stream/drive/ms 1.89, render/present/ms 1.53, stream/tick/ms 1.30, terrain/update/ms 0.99, terrain/select/ms 0.80, bigspace/total/ms 0.35, render/acquire/ms 0.24, stream/spawn/ms 0.23 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **77%** (min 51%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.8 W · 73 °C (max 78) · VRAM 3631 MiB · throttled 100% of samples
