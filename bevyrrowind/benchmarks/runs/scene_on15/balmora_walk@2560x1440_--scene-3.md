# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5408 frames over 65.6 s · **median 11.84 ms (84 fps)** · mean 12.14 · p95 14.50 · p99 17.13 (1% low 58 fps) · worst 34.01 ms · 38 spikes over 1.5× median · 31.1 stutters/min (work 25.6/min)
- warm-up: 369 frames over 8.4 s, worst 2634.1 ms
- hitches (> 29.6 ms): **4** measured, 20 in warm-up
- timed GPU passes: 7.45 ms of a 12.14 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.84 ms, limit 11 ms
- **FAIL** frame_ms: 34 stutters (> 1.5x moving median of 60) in 1.09 min = 31.08/min, limit 1/min
- **FAIL** worst frame 34.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.034 | 2.065 | 2.099 | 0.99 | 0.173 |
| main_opaque_pass_3d | 1.591 | 1.615 | 1.957 | 0.99 | 0.059 |
| early prepass | 0.555 | 0.563 | 0.793 | 0.99 | 0.079 |
| bloom | 0.314 | 0.319 | 0.325 | 0.99 | 0.063 |
| atmosphere_luts | 0.310 | 0.315 | 0.374 | 0.99 | 0.003 |
| water_velocity | 0.281 | 0.285 | 0.353 | 0.99 | 0.001 |
| water_sspr | 0.206 | 0.209 | 0.211 | 0.99 | 0.002 |
| ssao | 0.189 | 0.192 | 0.193 | 0.99 | 0.007 |
| uniform_allocation | 0.163 | 0.165 | 0.172 | 0.99 | 0.030 |
| auxiliary_views | 0.162 | 0.165 | 0.601 | 0.99 | 0.154 |
| atmos_froxel_inject | 0.147 | 0.149 | 0.150 | 0.99 | 0.002 |
| clouds_trace | 0.138 | 0.140 | 0.232 | 0.99 | 0.001 |
| water_fft | 0.131 | 0.133 | 0.137 | 0.99 | 0.003 |
| bin_unpacking | 0.123 | 0.125 | 0.134 | 0.99 | 0.022 |
| early_mesh_preprocessing | 0.116 | 0.118 | 0.426 | 0.99 | 0.004 |
| upscaling | 0.115 | 0.117 | 0.126 | 0.99 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.111 | 0.99 | 0.002 |
| gpu_instance_materialization | 0.090 | 0.091 | 0.095 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.088 | 0.165 | 0.99 | 0.008 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.003 |
| clustering | 0.065 | 0.066 | 0.068 | 0.99 | 0.019 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.451 | 0.723 | 0.11 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.044 | 0.057 | 0.057 | 0.77 | 0.002 |
| per_view_shadow_pass | 0.037 | 0.037 | 0.039 | 0.99 | 0.008 |
| water_snapshot | 0.031 | 0.031 | 0.035 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| range_unpacking | 0.028 | 0.028 | 0.033 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.025 | 0.223 | 0.233 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.025 | 0.025 | 0.028 | 0.99 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.032 | 0.99 | 0.079 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 18.584 | 0.000 | 9997.89 | 100500.3 |
| encode//sum | 11.010 | 15.995 | 51.60 | 59541.0 |
| render/thread/ms | 10.968 | 13.231 | 32.24 | 59317.3 |
| render/busy/ms | 10.393 | 12.668 | 30.91 | 56205.4 |
| main/busy/ms | 9.017 | 11.373 | 30.68 | 48763.4 |
| stage/render/render | 6.933 | 8.580 | 22.00 | 37491.5 |
| render/graph/ms | 6.174 | 7.776 | 21.22 | 33388.7 |
| stage/main/post_update | 5.469 | 7.574 | 22.80 | 29574.5 |
| stage/render/graph_submit | 5.018 | 6.303 | 20.33 | 27134.7 |
| encode/finish_wall | 2.783 | 3.575 | 18.06 | 15051.0 |
| encode/submit | 2.270 | 3.057 | 6.21 | 12276.4 |
| stage/main/update | 2.150 | 2.658 | 24.91 | 11628.1 |
| stage/render/camera_driver | 1.093 | 1.472 | 4.67 | 5913.2 |
| stage/render/prepare_write_phase_buffers | 0.879 | 1.162 | 2.37 | 4755.1 |
| stage/render/prepare_assets | 0.571 | 0.730 | 9.81 | 3089.1 |
| bigspace/total/ms | 0.490 | 0.930 | 18.40 | 2647.8 |
| render/present/ms | 0.486 | 0.624 | 2.97 | 2627.4 |
| stage/render/prepare_resources | 0.440 | 0.527 | 1.18 | 2381.0 |
| stage/main/first | 0.408 | 0.535 | 4.83 | 2207.0 |
| stage/render/prepare_meshes | 0.394 | 0.520 | 3.12 | 2128.9 |
| pipelines/compile_ms | 0.353 | 0.000 | 153.95 | 1911.6 |
| pipelines/front_ms | 0.341 | 0.000 | 148.12 | 1844.0 |
| stage/main/pre_update | 0.325 | 0.463 | 2.66 | 1758.5 |
| stage/render/prepare_flush | 0.316 | 0.506 | 1.32 | 1710.2 |
| stage/render/prepare_bind_groups | 0.312 | 0.532 | 2.59 | 1689.6 |
| terrain/update/ms | 0.294 | 0.376 | 2.16 | 1587.6 |
| controller/feed/ms | 0.271 | 0.353 | 0.68 | 1463.9 |
| terrain/select/ms | 0.261 | 0.333 | 0.63 | 1411.5 |
| stage/render/prepare_batch_phases | 0.257 | 0.356 | 0.68 | 1389.4 |
| stream/drive/ms | 0.250 | 0.342 | 11.28 | 1350.1 |
| stage/main/last | 0.246 | 0.311 | 2.66 | 1329.2 |
| stage/render/specialize | 0.246 | 0.354 | 2.15 | 1328.6 |
| stage/main/fixed_main_loop | 0.236 | 0.376 | 1.27 | 1274.6 |
| stage/render/queue | 0.228 | 0.350 | 2.20 | 1233.0 |
| bigspace/low_precision/ms | 0.202 | 0.498 | 3.41 | 1092.1 |
| stream/tick/ms | 0.192 | 0.265 | 0.68 | 1035.8 |
| far/select/ms | 0.190 | 0.260 | 0.54 | 1024.8 |
| stage/render/extract_commands | 0.174 | 0.208 | 0.50 | 940.7 |
| stage/render/create_views | 0.165 | 0.213 | 8.14 | 890.9 |
| anim/evaluate_cpu/ms | 0.156 | 0.202 | 0.45 | 841.0 |
| anim/evaluate/ms | 0.154 | 0.290 | 2.76 | 831.6 |
| stage/render/prepare_views | 0.132 | 0.168 | 0.62 | 712.7 |
| bigspace/high_precision/ms | 0.132 | 0.256 | 18.05 | 711.5 |
| controller/move/ms | 0.118 | 0.203 | 0.44 | 638.0 |
| render/acquire/ms | 0.090 | 0.118 | 0.33 | 484.6 |
| physics/step/ms | 0.054 | 0.089 | 0.89 | 292.0 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.072 | 0.35 | 253.9 |
| physics/insert/ms | 0.044 | 0.405 | 0.92 | 235.8 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.42 | 166.0 |
| stage/render/cleanup | 0.026 | 0.042 | 1.06 | 141.1 |
| stage/render/phase_sort | 0.026 | 0.046 | 0.24 | 138.1 |
| weather/map/ms | 0.024 | 0.032 | 0.20 | 130.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.35 | 71.7 |
| pipelines/driver_ms | 0.013 | 0.000 | 5.82 | 67.6 |
| anim/apply/ms | 0.011 | 0.020 | 0.11 | 60.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.76 | 57.8 |
| controller/avoid/ms | 0.010 | 0.014 | 0.07 | 56.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.23 | 54.5 |
| script/update/ms | 0.007 | 0.009 | 0.09 | 37.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.26 | 29.4 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 27.9 |
| script/ui/ms | 0.005 | 0.008 | 0.33 | 27.7 |
| stream/upload/ms | 0.004 | 0.000 | 1.52 | 21.2 |
| controller/write/ms | 0.003 | 0.004 | 0.20 | 18.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.08 | 18.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 15.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.1 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 24 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 56 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 145 | 156 | 147 |
| crowd/c1/joints/count | count | 5514 | 5991 | 5581 |
| crowd/c1/meshes/count | count | 1172 | 1295 | 1171 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4259 | 10.3 k | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.6 k | 56.7 k | 46.9 k |
| encode//max | ms | 2.34 | 18.01 | 1.62 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.35 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.76 | 0.01 |
| encode/finish_critical | ms | 2.34 | 18.01 | 1.62 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.42 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.19 | 30.91 | 8.90 |
| gpu/timed/ms | ms | 8.27 | 9.85 | 8.64 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2512 | 2569 | 2569 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 286.5 k | 286.5 k | 286.5 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.6 k | 11.8 k | 10.4 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 323.97 M | 323.97 M | 323.97 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 337 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 337 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 758 | 970 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2508 | 3553 | 2453 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 758 | 970 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2508 | 3553 | 2453 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 47 | 192 | 40 |
| phases/shadow/bins | count | 1845 | 2426 | 961 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5434 | 6735 | 3858 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 166 | 214 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 921 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 181 | 183 | 183 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 17 | 526 | 13 |
| preprocess/bind_groups_created | count | 10 | 519 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| preprocess/bind_groups_reused | count | 174 | 270 | 169 |
| process/cpu_usage | % | 24.29 | 38.94 | 28.64 |
| process/mem_usage | GiB | 5.07 | 5.24 | 5.24 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 6296 | 6420 | 6420 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.07 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.75 | 48.87 | 28.61 |
| system/mem_usage | % | 60.34 | 61.13 | 60.94 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2817 | 2928 | 2928 |
| textures/images | count | 2578 | 2631 | 2631 |
| views/all/batch_sets | count | 103 | 247 | 96 |
| views/all/bins | count | 3765 | 4492 | 3243 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.8 k | 33.8 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 11.2 k | 13.9 k | 9796 |
| views/all/pipelines | count | 211 | 261 | 74 |
| views/all/pipelines_distinct | count | 68 | 73 | 59 |
| views/all/work_items | count | 256.3 k | 259.2 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 56 | 40 |
| views/cascade/bins | count | 1422 | 1782 | 961 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4742 | 5920 | 3858 |
| views/cascade/pipelines | count | 37 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.4 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 147 | 0 |
| views/lamp_face/bins | count | 395 | 747 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 643 | 1090 | 0 |
| views/lamp_face/pipelines | count | 128 | 170 | 0 |
| views/lamp_face/work_items | count | 0 | 712 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2020 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5943 | 7528 | 5938 |
| views/main/pipelines | count | 45 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 105.9 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.6 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.88 | 840 | 280.6 k |
| anim/events/count | count | 0.09 | 3 | 477 |
| assets/Image/added | count | 0.12 | 43 | 670 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.59 | 341 | 3191 |
| assets/Mesh/removed | count | 0.91 | 1238 | 4896 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 562 |
| assets/StandardMaterial/modified | count | 4.00 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 1.87 | 2772 | 10.1 k |
| encode//count | count | 41.01 | 273 | 221.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5408 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5408 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5408 |
| physics/inserts | count | 0.59 | 164 | 3167 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 113 | 2335 |
| physics/steps | count | 0.73 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 3 | 12 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 6260 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1372 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.6 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2634.1 | main/busy/ms 2602, stage/main/post_update 2503, stage/main/update 93.21, surface/advance_cpu_ms 66.39, stage/main/pre_update 2.37, stage/main/last 1.00, surface/cell_upload_ms 0.83, stage/main/first 0.47, stage/main/fixed_main_loop 0.35, stream/drive/ms 0.32, stream/tick/ms 0.10, weather/map/ms 0.10 |
| warm-up | 3.59 | 957.9 | main/busy/ms 2640, stage/main/post_update 2532, render/thread/ms 913, render/busy/ms 793, stage/render/prepare_assets 529, stage/render/prepare_resources 234, stage/render/prepare_views 118, render/acquire/ms 118, stage/main/update 98.04, surface/advance_cpu_ms 66.39, stage/render/create_views 18.87, stage/render/render 9.53 |
| warm-up | 3.85 | 75.3 | main/busy/ms 61.68, render/thread/ms 53.06, render/busy/ms 51.32, stage/main/update 50.60, stage/render/prepare_assets 41.16, stage/main/post_update 7.87, stage/render/render 6.91, render/graph/ms 4.72, stage/render/graph_submit 2.79, encode/submit 2.23, stage/render/camera_driver 1.77, stream/drive/ms 1.76 |
| warm-up | 3.98 | 55.1 | main/busy/ms 61.59, render/thread/ms 60.47, render/busy/ms 58.27, stage/main/update 49.41, stage/render/prepare_assets 43.40, stage/render/render 8.22, stage/main/post_update 7.92, render/graph/ms 5.55, stage/render/graph_submit 3.41, encode/submit 2.85, stream/drive/ms 2.40, stage/render/camera_driver 1.96 |
| warm-up | 4.03 | 48.9 | render/thread/ms 64.85, render/busy/ms 63.30, stage/render/prepare_assets 52.44, main/busy/ms 45.78, stage/main/update 34.15, stage/main/post_update 7.12, stage/render/render 5.79, render/graph/ms 4.08, stage/render/graph_submit 2.62, encode/submit 2.20, stream/drive/ms 2.17, stream/tick/ms 1.64 |
| warm-up | 4.09 | 61.7 | pipelines/wait_ms 1973, pipelines/compile_ms 314, pipelines/front_ms 223, pipelines/driver_ms 91.02, render/thread/ms 82.37, render/busy/ms 81.03, stage/render/prepare_assets 70.68, main/busy/ms 23.04, stage/main/update 13.33, stage/render/render 7.22, render/graph/ms 5.63, stage/main/post_update 5.55 |
| warm-up | 4.20 | 33.6 | render/thread/ms 40.49, render/busy/ms 38.78, stage/render/prepare_assets 24.38, main/busy/ms 23.90, stage/main/update 10.08, stage/main/post_update 8.16, stage/render/render 7.39, render/graph/ms 4.97, stream/drive/ms 3.76, stage/render/graph_submit 2.80, encode//sum 2.70, stage/main/pre_update 2.19 |
| warm-up | 4.23 | 35.2 | render/thread/ms 40.68, render/busy/ms 39.50, stage/render/prepare_assets 28.89, main/busy/ms 23.74, stage/main/update 9.78, stage/main/post_update 9.57, stage/render/render 5.14, stream/drive/ms 4.08, render/graph/ms 3.39, encode//sum 2.14, stage/render/graph_submit 1.77, stream/tick/ms 1.52 |
| warm-up | 4.40 | 152.6 | pipelines/wait_ms 2391, pipelines/compile_ms 275, pipelines/front_ms 257, main/busy/ms 124, stage/main/update 105, render/thread/ms 52.68, render/busy/ms 51.23, stage/render/prepare_assets 38.22, pipelines/driver_ms 17.41, stage/main/post_update 12.60, stage/render/render 6.09, render/graph/ms 4.14 |
| warm-up | 4.50 | 44.7 | pipelines/wait_ms 1570, pipelines/compile_ms 91.61, pipelines/front_ms 87.32, render/thread/ms 53.83, render/busy/ms 51.45, main/busy/ms 43.77, stage/main/update 28.53, stage/render/prepare_assets 27.37, encode//sum 22.41, stream/drive/ms 19.86, stage/render/render 14.58, render/graph/ms 11.22 |
| warm-up | 4.53 | 30.9 | pipelines/wait_ms 1570, pipelines/compile_ms 91.61, pipelines/front_ms 87.32, render/thread/ms 54.00, render/busy/ms 52.10, main/busy/ms 32.96, stage/render/prepare_assets 29.88, encode//sum 18.37, stage/main/update 15.48, stage/render/render 11.94, stage/main/post_update 9.87, render/graph/ms 9.46 |
| warm-up | 4.57 | 38.3 | render/thread/ms 52.22, render/busy/ms 50.29, main/busy/ms 38.18, stage/render/prepare_assets 25.17, stage/main/update 16.86, stage/render/render 12.43, stage/main/post_update 11.33, encode//sum 10.81, render/graph/ms 9.64, stream/drive/ms 6.42, stage/render/graph_submit 5.58, stage/render/prepare_bind_groups 4.49 |
| warm-up | 4.69 | 59.5 | pipelines/wait_ms 596, pipelines/compile_ms 148, pipelines/front_ms 144, main/busy/ms 77.14, stage/main/update 61.92, stream/drive/ms 57.15, render/thread/ms 32.63, render/busy/ms 31.39, stage/render/prepare_assets 16.36, stage/main/post_update 9.64, stage/render/render 5.59, render/graph/ms 3.94 |
| warm-up | 4.90 | 45.5 | main/busy/ms 48.48, stage/main/update 37.00, stream/drive/ms 33.44, render/thread/ms 17.80, render/busy/ms 16.64, stage/main/post_update 8.10, stage/render/prepare_assets 6.34, stage/render/render 4.69, render/graph/ms 3.21, encode//sum 2.16, stage/render/graph_submit 1.90, stage/render/prepare_write_phase_buffers 1.70 |
| warm-up | 5.62 | 31.2 | pipelines/wait_ms 13.2 k, pipelines/compile_ms 68.80, pipelines/front_ms 64.07, render/thread/ms 40.64, render/busy/ms 39.13, main/busy/ms 37.13, stage/main/update 21.72, stage/render/prepare_assets 13.54, stage/main/post_update 10.08, stage/render/render 8.83, render/graph/ms 6.34, encode//sum 5.82 |
| warm-up | 5.66 | 42.5 | pipelines/wait_ms 9322, render/thread/ms 57.20, render/busy/ms 55.31, main/busy/ms 45.12, pipelines/compile_ms 34.14, pipelines/front_ms 30.87, stage/main/update 25.30, stage/render/prepare_assets 23.12, stage/main/post_update 13.05, stage/render/render 10.91, render/graph/ms 7.87, encode//sum 7.53 |
| warm-up | 5.87 | 40.3 | pipelines/wait_ms 5124, pipelines/compile_ms 167, pipelines/front_ms 159, main/busy/ms 43.24, render/thread/ms 41.97, render/busy/ms 40.29, stage/main/update 29.24, stage/render/prepare_assets 15.76, stage/render/render 12.75, render/graph/ms 10.44, stage/main/post_update 9.03, pipelines/driver_ms 8.72 |
| warm-up | 6.33 | 32.9 | main/busy/ms 39.53, stage/main/update 26.97, render/thread/ms 24.79, render/busy/ms 23.16, encode//sum 10.21, stage/render/render 9.45, stage/main/post_update 9.35, render/graph/ms 7.30, stage/render/prepare_assets 5.73, stage/render/graph_submit 5.01, encode/submit 2.84, encode/finish_wall 2.19 |
| warm-up | 6.39 | 30.0 | pipelines/wait_ms 1533, pipelines/compile_ms 106, pipelines/front_ms 103, main/busy/ms 34.90, render/thread/ms 32.00, render/busy/ms 30.56, stage/main/update 23.00, stage/render/prepare_assets 12.11, encode//sum 10.12, stage/render/render 9.80, stage/main/post_update 8.59, render/graph/ms 7.63 |
| warm-up | 6.43 | 31.0 | main/busy/ms 42.72, stage/main/update 30.96, render/thread/ms 27.57, render/busy/ms 26.09, encode//sum 12.81, stage/render/render 11.87, render/graph/ms 9.72, stage/main/post_update 8.46, stage/render/graph_submit 6.39, stage/render/prepare_assets 6.21, encode/submit 3.76, stage/render/camera_driver 3.10 |
| measure | 2.17 | 33.2 | encode//sum 66.12, render/thread/ms 44.04, render/busy/ms 42.18, stage/render/render 25.45, render/graph/ms 23.02, stage/render/graph_submit 17.56, main/busy/ms 15.80, pipelines/compile_ms 11.73, pipelines/front_ms 11.52, encode/finish_wall 9.53, stage/main/post_update 9.24, stage/render/create_views 8.27 |
| measure | 18.15 | 33.0 | main/busy/ms 40.81, stage/main/update 26.99, render/thread/ms 26.09, encode//sum 25.76, render/busy/ms 24.92, stage/render/render 14.39, render/graph/ms 12.88, stage/main/post_update 10.49, stage/render/graph_submit 10.10, encode/finish_wall 5.60, encode/submit 4.54, stage/render/prepare_assets 4.02 |
| measure | 58.99 | 34.0 | render/thread/ms 38.86, render/busy/ms 37.58, main/busy/ms 27.29, encode//sum 22.56, stage/main/post_update 16.98, stage/render/render 16.69, render/graph/ms 15.04, stage/render/prepare_assets 13.22, stage/render/graph_submit 12.02, encode/finish_wall 6.33, encode/submit 5.74, stage/main/update 4.87 |
| measure | 59.02 | 30.2 | render/thread/ms 37.42, render/busy/ms 36.35, stage/render/prepare_assets 18.80, encode//sum 17.57, main/busy/ms 17.35, stage/render/render 11.32, render/graph/ms 10.01, stage/main/post_update 9.05, stage/render/graph_submit 8.02, encode/finish_wall 4.24, stage/main/update 3.90, encode/submit 3.81 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on15\balmora_walk@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **79%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.9 W · 73 °C (max 77) · VRAM 6080 MiB · throttled 100% of samples
