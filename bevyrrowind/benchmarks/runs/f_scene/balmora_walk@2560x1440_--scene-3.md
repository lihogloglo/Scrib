# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5815 frames over 65.6 s · **median 11.08 ms (90 fps)** · mean 11.29 · p95 13.34 · p99 15.45 (1% low 65 fps) · worst 32.86 ms · 33 spikes over 1.5× median · 22.8 stutters/min (work 21.9/min)
- warm-up: 353 frames over 9.2 s, worst 2501.1 ms
- hitches (> 27.7 ms): **2** measured, 37 in warm-up
- timed GPU passes: 7.36 ms of a 11.29 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.08 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 32.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.017 | 2.075 | 2.116 | 0.97 | 0.172 |
| main_opaque_pass_3d | 1.565 | 1.610 | 1.956 | 0.97 | 0.063 |
| early prepass | 0.559 | 0.575 | 0.813 | 0.97 | 0.081 |
| bloom | 0.312 | 0.321 | 0.326 | 0.97 | 0.062 |
| atmosphere_luts | 0.305 | 0.313 | 0.372 | 0.97 | 0.003 |
| water_velocity | 0.276 | 0.284 | 0.353 | 0.97 | 0.001 |
| water_sspr | 0.200 | 0.206 | 0.206 | 0.97 | 0.002 |
| ssao | 0.187 | 0.192 | 0.195 | 0.97 | 0.007 |
| atmos_froxel_inject | 0.143 | 0.147 | 0.147 | 0.97 | 0.001 |
| auxiliary_views | 0.141 | 0.145 | 0.520 | 0.97 | 0.135 |
| clouds_trace | 0.138 | 0.142 | 0.231 | 0.97 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.136 | 0.571 | 0.645 | 0.24 | 0.044 |
| water_fft | 0.129 | 0.132 | 0.138 | 0.97 | 0.003 |
| gpu_instance_materialization | 0.118 | 0.121 | 0.425 | 0.97 | 0.004 |
| uniform_allocation | 0.117 | 0.120 | 0.146 | 0.97 | 0.022 |
| upscaling | 0.115 | 0.119 | 0.128 | 0.97 | 0.004 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.97 | 0.002 |
| bin_unpacking | 0.096 | 0.099 | 0.116 | 0.97 | 0.017 |
| tonemapping | 0.084 | 0.086 | 0.090 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.081 | 0.083 | 0.160 | 0.97 | 0.008 |
| early_mesh_preprocessing | 0.065 | 0.067 | 0.088 | 0.97 | 0.003 |
| clustering | 0.064 | 0.066 | 0.068 | 0.97 | 0.019 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.430 | 0.425 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| surface_trail | 0.041 | 0.058 | 0.057 | 0.71 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.223 | 0.234 | 0.12 | 0.004 |
| range_unpacking | 0.025 | 0.026 | 0.029 | 0.97 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.027 | 0.97 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.97 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| main_transparent_pass_3d | 0.008 | 0.009 | 0.020 | 0.97 | 0.079 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.020 | 0.016 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.017 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.017 | 0.022 | 0.01 | 0.008 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.015 | 0.016 | 0.00 | 0.011 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.752 | 15.430 | 42.40 | 62524.3 |
| render/thread/ms | 10.112 | 12.069 | 22.79 | 58801.2 |
| render/busy/ms | 9.525 | 11.476 | 22.13 | 55387.2 |
| main/busy/ms | 8.881 | 10.825 | 30.37 | 51642.6 |
| stage/render/render | 6.438 | 7.784 | 17.14 | 37436.6 |
| render/graph/ms | 5.677 | 6.986 | 16.36 | 33013.5 |
| stage/main/post_update | 5.300 | 7.009 | 10.00 | 30819.3 |
| stage/render/graph_submit | 4.563 | 5.608 | 10.97 | 26533.9 |
| encode/submit | 2.316 | 3.076 | 6.79 | 13470.1 |
| encode/finish_wall | 2.303 | 2.951 | 4.56 | 13393.5 |
| stage/main/update | 2.212 | 2.675 | 24.75 | 12863.9 |
| stage/render/camera_driver | 1.055 | 1.387 | 11.33 | 6134.7 |
| stage/render/prepare_write_phase_buffers | 0.641 | 0.911 | 1.93 | 3726.2 |
| bigspace/total/ms | 0.625 | 2.097 | 3.70 | 3635.1 |
| stage/render/prepare_assets | 0.547 | 0.698 | 11.74 | 3179.9 |
| render/present/ms | 0.498 | 0.626 | 4.58 | 2895.3 |
| stage/render/prepare_resources | 0.442 | 0.529 | 2.18 | 2570.1 |
| stage/main/first | 0.396 | 0.514 | 5.21 | 2303.7 |
| stage/render/prepare_meshes | 0.387 | 0.489 | 3.16 | 2253.0 |
| stage/main/pre_update | 0.325 | 0.474 | 2.23 | 1892.5 |
| terrain/update/ms | 0.308 | 0.412 | 2.12 | 1790.8 |
| stage/render/prepare_bind_groups | 0.304 | 0.496 | 2.19 | 1769.2 |
| stage/render/prepare_flush | 0.284 | 0.509 | 1.86 | 1650.2 |
| bigspace/low_precision/ms | 0.279 | 1.071 | 3.48 | 1621.4 |
| controller/feed/ms | 0.275 | 0.349 | 0.78 | 1600.3 |
| terrain/select/ms | 0.275 | 0.368 | 0.81 | 1597.2 |
| stream/drive/ms | 0.252 | 0.334 | 11.22 | 1466.8 |
| stage/main/last | 0.251 | 0.306 | 1.06 | 1459.0 |
| stage/render/specialize | 0.239 | 0.344 | 1.93 | 1391.4 |
| stage/main/fixed_main_loop | 0.218 | 0.361 | 1.13 | 1269.2 |
| stage/render/queue | 0.214 | 0.333 | 2.44 | 1245.4 |
| anim/evaluate/ms | 0.208 | 0.366 | 3.16 | 1211.2 |
| stage/render/prepare_batch_phases | 0.206 | 0.297 | 0.56 | 1195.7 |
| bigspace/high_precision/ms | 0.201 | 0.796 | 3.14 | 1169.7 |
| stream/tick/ms | 0.197 | 0.266 | 0.51 | 1147.8 |
| far/select/ms | 0.190 | 0.258 | 0.67 | 1107.7 |
| stage/render/extract_commands | 0.174 | 0.207 | 0.52 | 1010.9 |
| stage/render/create_views | 0.169 | 0.218 | 0.89 | 981.9 |
| anim/evaluate_cpu/ms | 0.145 | 0.186 | 0.53 | 842.6 |
| stage/render/prepare_views | 0.131 | 0.165 | 0.42 | 762.0 |
| controller/move/ms | 0.130 | 0.260 | 0.52 | 753.9 |
| render/acquire/ms | 0.089 | 0.117 | 0.39 | 518.8 |
| pipelines/compile_ms | 0.062 | 0.000 | 114.80 | 363.2 |
| pipelines/front_ms | 0.060 | 0.000 | 111.11 | 348.2 |
| physics/step/ms | 0.048 | 0.085 | 0.41 | 280.8 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.071 | 0.38 | 271.3 |
| physics/insert/ms | 0.034 | 0.370 | 1.00 | 198.8 |
| stage/render/phase_sort | 0.032 | 0.060 | 0.29 | 188.6 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.46 | 176.9 |
| weather/map/ms | 0.025 | 0.033 | 0.09 | 148.0 |
| stage/render/cleanup | 0.023 | 0.034 | 0.78 | 136.3 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.46 | 75.8 |
| anim/apply/ms | 0.011 | 0.022 | 0.27 | 66.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.17 | 61.1 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.78 | 60.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.07 | 59.2 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 40.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.31 | 29.4 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 28.7 |
| script/ui/ms | 0.005 | 0.006 | 0.04 | 27.8 |
| stream/upload/ms | 0.004 | 0.000 | 1.33 | 20.7 |
| controller/write/ms | 0.003 | 0.004 | 0.25 | 19.9 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 17.0 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.73 | 15.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 14.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| stage/render/prepare | 0.000 | 0.002 | 0.05 | 2.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1291 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 6004 | 5613 |
| crowd/c1/meshes/count | count | 1171 | 1308 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4608 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.8 k | 46.9 k |
| encode//max | ms | 1.90 | 3.76 | 2.40 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.46 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.78 | 0.01 |
| encode/finish_critical | ms | 1.90 | 3.76 | 2.40 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.46 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.57 | 30.37 | 9.64 |
| gpu/timed/ms | ms | 8.69 | 10.60 | 9.14 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 387 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 387 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 761 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2508 | 3585 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 761 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2508 | 3585 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 157 | 8 |
| phases/shadow/bins | count | 1566 | 2279 | 896 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 4999 | 6494 | 3790 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 155 | 216 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2668 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 302 | 303 | 303 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 18 | 468 | 11 |
| preprocess/bind_groups_created | count | 13 | 463 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 126 | 199 | 119 |
| process/cpu_usage | % | 25.96 | 39.32 | 24.52 |
| process/mem_usage | GiB | 5.39 | 5.56 | 5.56 |
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
| started_load_count |  | 6447 | 6571 | 6571 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.24 | 44.93 | 30.81 |
| system/mem_usage | % | 50.43 | 50.90 | 50.89 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2880 | 2996 | 2996 |
| textures/images | count | 2663 | 2723 | 2723 |
| views/all/batch_sets | count | 77 | 212 | 64 |
| views/all/bins | count | 3540 | 4253 | 3178 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 10.7 k | 13.5 k | 9730 |
| views/all/pipelines | count | 199 | 261 | 64 |
| views/all/pipelines_distinct | count | 68 | 73 | 59 |
| views/all/work_items | count | 154.0 k | 209.6 k | 153.8 k |
| views/cascade/batch_sets | count | 13 | 40 | 8 |
| views/cascade/bins | count | 1141 | 1562 | 896 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 4270 | 5503 | 3790 |
| views/cascade/pipelines | count | 25 | 34 | 20 |
| views/cascade/work_items | count | 49.5 k | 103.8 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 144 | 0 |
| views/lamp_face/bins | count | 426 | 724 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 674 | 1097 | 0 |
| views/lamp_face/pipelines | count | 130 | 184 | 0 |
| views/lamp_face/work_items | count | 0 | 714 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2002 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5950 | 7590 | 5940 |
| views/main/pipelines | count | 45 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
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
| anim/bones_written/count | count | 56.27 | 286 | 327.2 k |
| anim/events/count | count | 0.09 | 3 | 520 |
| assets/Image/added | count | 0.10 | 43 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.49 | 448 | 2874 |
| assets/Mesh/removed | count | 0.84 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.73 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.26 | 1455 | 1489 |
| encode//count | count | 40.26 | 273 | 234.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5815 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5815 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5815 |
| physics/inserts | count | 0.39 | 107 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 6 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5822 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1276 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 39, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2501.1 | main/busy/ms 2475, stage/main/post_update 2375, stage/main/update 93.22, surface/advance_cpu_ms 66.44, stage/main/pre_update 2.51, stage/main/last 1.16, surface/cell_upload_ms 0.71, stage/main/first 0.57, stream/drive/ms 0.46, stage/main/fixed_main_loop 0.40, weather/map/ms 0.17, stream/tick/ms 0.09 |
| warm-up | 3.19 | 689.9 | main/busy/ms 2607, stage/main/post_update 2397, render/thread/ms 661, render/busy/ms 497, stage/render/prepare_assets 471, stage/main/update 198, stage/render/prepare_views 162, render/acquire/ms 161, surface/advance_cpu_ms 66.44, stage/render/prepare_resources 11.35, encode//sum 10.23, stage/render/render 9.75 |
| warm-up | 3.25 | 61.7 | render/thread/ms 706, render/busy/ms 541, stage/render/prepare_assets 494, stage/render/prepare_views 163, render/acquire/ms 162, main/busy/ms 151, stage/main/update 108, stage/main/post_update 32.88, pipelines/compile_ms 31.82, pipelines/front_ms 29.94, stage/render/render 23.55, render/graph/ms 18.42 |
| warm-up | 3.29 | 38.7 | pipelines/compile_ms 132, pipelines/front_ms 123, render/thread/ms 83.17, pipelines/wait_ms 82.49, render/busy/ms 78.33, main/busy/ms 54.77, stage/render/render 45.10, stage/main/post_update 41.33, render/graph/ms 34.04, stage/render/prepare_assets 23.50, terrain/update/ms 22.20, stage/render/camera_driver 13.36 |
| warm-up | 3.42 | 130.9 | pipelines/wait_ms 3391, pipelines/compile_ms 1011, pipelines/front_ms 949, render/thread/ms 165, render/busy/ms 160, main/busy/ms 72.29, pipelines/driver_ms 61.71, stage/render/prepare_resources 53.43, stage/render/render 45.73, stage/main/post_update 40.83, render/graph/ms 35.40, stage/render/prepare_flush 22.44 |
| warm-up | 3.49 | 67.9 | pipelines/wait_ms 6961, pipelines/compile_ms 2298, pipelines/front_ms 2173, render/thread/ms 193, render/busy/ms 191, pipelines/driver_ms 125, main/busy/ms 115, stage/main/update 79.56, stage/render/prepare_resources 78.39, stage/render/prepare_assets 30.05, stage/render/render 25.77, render/graph/ms 23.35 |
| warm-up | 3.62 | 132.8 | pipelines/wait_ms 3962, pipelines/compile_ms 2508, pipelines/front_ms 2380, render/thread/ms 182, render/busy/ms 154, pipelines/driver_ms 127, main/busy/ms 99.58, stage/main/update 72.10, stage/render/prepare_assets 70.37, stage/render/render 51.11, stage/render/prepare_resources 46.08, render/present/ms 26.96 |
| warm-up | 3.68 | 32.6 | pipelines/compile_ms 639, pipelines/front_ms 551, pipelines/driver_ms 88.08, render/thread/ms 55.42, render/busy/ms 53.32, pipelines/wait_ms 43.23, main/busy/ms 28.78, stage/render/prepare_assets 26.38, stage/render/render 22.24, render/graph/ms 19.93, encode//sum 18.96, stage/render/graph_submit 17.36 |
| warm-up | 3.71 | 29.3 | pipelines/compile_ms 610, pipelines/front_ms 588, pipelines/wait_ms 172, render/thread/ms 58.15, render/busy/ms 56.21, main/busy/ms 42.19, stage/render/prepare_assets 24.99, pipelines/driver_ms 22.87, stage/main/update 21.93, stage/main/post_update 15.91, stage/render/prepare_write_phase_buffers 13.99, stage/render/render 10.15 |
| warm-up | 3.76 | 46.3 | pipelines/compile_ms 679, pipelines/front_ms 643, pipelines/wait_ms 162, main/busy/ms 94.47, stage/main/update 71.85, render/thread/ms 69.43, render/busy/ms 67.35, pipelines/driver_ms 36.00, stage/render/prepare_assets 31.80, stage/main/post_update 18.14, stage/render/prepare_write_phase_buffers 14.03, stage/render/render 12.85 |
| warm-up | 3.85 | 87.8 | pipelines/compile_ms 1304, pipelines/front_ms 1236, render/thread/ms 74.38, main/busy/ms 73.31, render/busy/ms 72.65, pipelines/driver_ms 67.55, stage/main/update 56.31, stage/render/prepare_assets 55.11, stage/render/render 11.26, stage/main/post_update 10.88, terrain/update/ms 10.25, encode//sum 10.11 |
| warm-up | 3.93 | 81.1 | pipelines/compile_ms 1339, pipelines/front_ms 1222, pipelines/driver_ms 117, render/thread/ms 97.06, render/busy/ms 59.96, stage/render/prepare_assets 46.59, stage/render/render 44.87, render/present/ms 36.92, main/busy/ms 18.01, render/graph/ms 7.51, encode//sum 6.29, stage/main/update 6.13 |
| warm-up | 4.02 | 48.9 | pipelines/compile_ms 393, pipelines/front_ms 379, main/busy/ms 52.22, render/thread/ms 47.39, render/busy/ms 45.27, stage/render/render 20.42, stage/main/post_update 19.04, stage/main/update 18.78, render/graph/ms 18.08, stage/render/prepare_assets 15.93, pipelines/driver_ms 13.92, stage/render/graph_submit 13.45 |
| warm-up | 4.10 | 73.4 | pipelines/compile_ms 1175, pipelines/front_ms 1129, render/thread/ms 70.28, render/busy/ms 68.40, main/busy/ms 56.87, pipelines/driver_ms 45.95, stage/render/prepare_assets 43.30, stage/main/update 24.36, stage/main/post_update 15.29, stage/render/render 14.28, stream/drive/ms 12.32, render/graph/ms 11.68 |
| warm-up | 4.19 | 93.6 | pipelines/compile_ms 2057, pipelines/front_ms 1980, render/thread/ms 102, render/busy/ms 88.95, pipelines/driver_ms 77.36, stage/render/prepare_assets 69.89, main/busy/ms 49.69, stage/main/update 31.17, stage/render/render 24.40, stream/drive/ms 22.90, pipelines/wait_ms 19.19, render/present/ms 12.90 |
| warm-up | 4.24 | 43.8 | pipelines/compile_ms 1460, pipelines/front_ms 1409, render/thread/ms 102, render/busy/ms 88.19, main/busy/ms 77.54, stage/main/update 56.43, pipelines/driver_ms 50.67, stage/render/prepare_assets 50.46, stage/render/render 36.92, render/graph/ms 22.62, stage/render/graph_submit 20.00, pipelines/wait_ms 16.41 |
| warm-up | 4.33 | 92.8 | pipelines/compile_ms 903, pipelines/front_ms 876, main/busy/ms 199, stage/main/update 179, pipelines/wait_ms 88.91, render/thread/ms 86.88, render/busy/ms 84.40, stage/render/prepare_assets 39.47, stage/render/render 27.16, pipelines/driver_ms 27.02, render/graph/ms 24.28, stage/render/graph_submit 21.50 |
| warm-up | 4.49 | 157.1 | pipelines/compile_ms 2976, pipelines/front_ms 2889, pipelines/wait_ms 1070, main/busy/ms 157, stage/main/update 142, render/thread/ms 88.92, pipelines/driver_ms 87.58, render/busy/ms 87.16, stage/render/prepare_assets 55.24, stage/render/render 16.97, encode//sum 15.87, render/graph/ms 14.48 |
| warm-up | 4.57 | 85.1 | pipelines/compile_ms 3380, pipelines/front_ms 3263, pipelines/wait_ms 1291, render/thread/ms 118, pipelines/driver_ms 116, render/busy/ms 116, main/busy/ms 70.77, stage/main/post_update 48.66, encode//sum 32.26, stage/render/prepare_assets 28.46, stage/render/create_views 27.90, stage/render/render 26.61 |
| warm-up | 4.61 | 44.2 | pipelines/compile_ms 2276, pipelines/front_ms 2193, pipelines/wait_ms 859, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 82.78, main/busy/ms 80.79, stage/main/post_update 48.02, encode//sum 37.77, stage/render/create_views 28.74, stage/render/render 27.39, stage/main/update 25.66 |
| warm-up | 4.68 | 60.1 | pipelines/compile_ms 1378, pipelines/front_ms 1337, pipelines/wait_ms 550, encode//sum 90.58, render/thread/ms 85.83, render/busy/ms 83.98, main/busy/ms 41.16, pipelines/driver_ms 40.75, stage/render/render 30.94, render/graph/ms 28.46, stage/main/update 25.87, stage/render/prepare_assets 21.32 |
| warm-up | 4.79 | 93.1 | pipelines/compile_ms 962, pipelines/front_ms 941, render/thread/ms 75.47, render/busy/ms 73.40, main/busy/ms 71.14, stage/main/update 52.56, pipelines/wait_ms 38.74, stage/render/prepare_assets 35.16, pipelines/driver_ms 21.61, stage/render/render 21.55, render/graph/ms 18.77, encode//sum 14.49 |
| warm-up | 4.83 | 33.6 | pipelines/compile_ms 1020, pipelines/front_ms 995, render/thread/ms 79.31, render/busy/ms 77.71, stage/render/prepare_assets 42.32, pipelines/wait_ms 39.61, main/busy/ms 38.68, pipelines/driver_ms 25.46, stage/main/update 21.33, encode//sum 16.28, stage/render/render 13.83, render/graph/ms 11.84 |
| warm-up | 4.87 | 43.5 | pipelines/compile_ms 674, pipelines/front_ms 651, main/busy/ms 83.22, stage/main/update 69.02, stream/drive/ms 57.29, render/thread/ms 55.55, render/busy/ms 53.85, stage/render/prepare_assets 27.26, pipelines/driver_ms 22.92, encode//sum 16.11, stage/render/render 13.40, render/graph/ms 11.18 |
| warm-up | 4.94 | 66.3 | pipelines/compile_ms 440, pipelines/front_ms 425, main/busy/ms 92.13, stage/main/update 73.70, stream/drive/ms 59.18, render/thread/ms 54.19, render/busy/ms 52.52, stage/render/prepare_assets 23.87, encode//sum 16.54, pipelines/driver_ms 15.07, stage/render/render 13.92, stage/main/post_update 12.06 |
| warm-up | 4.97 | 30.4 | pipelines/compile_ms 541, pipelines/front_ms 523, render/thread/ms 47.23, render/busy/ms 45.61, main/busy/ms 45.10, stage/main/update 23.55, pipelines/driver_ms 17.57, encode//sum 16.18, stage/render/render 15.70, stage/main/post_update 14.94, render/graph/ms 13.62, stage/render/prepare_assets 12.61 |
| warm-up | 5.00 | 30.3 | pipelines/compile_ms 927, pipelines/front_ms 894, render/thread/ms 50.18, render/busy/ms 48.47, main/busy/ms 46.69, pipelines/driver_ms 32.79, stage/main/update 26.23, stage/render/render 16.80, encode//sum 16.11, render/graph/ms 14.81, stage/main/post_update 13.52, stage/render/graph_submit 10.27 |
| warm-up | 5.03 | 28.9 | pipelines/compile_ms 408, pipelines/front_ms 392, render/thread/ms 53.19, render/busy/ms 51.63, main/busy/ms 50.21, stage/main/update 28.16, pipelines/driver_ms 16.53, encode//sum 16.45, stage/main/post_update 15.66, stage/render/render 15.49, render/graph/ms 13.62, stage/render/prepare_assets 12.19 |
| warm-up | 5.06 | 34.0 | render/thread/ms 51.17, render/busy/ms 49.64, main/busy/ms 40.34, pipelines/compile_ms 36.67, pipelines/front_ms 34.88, stage/main/update 20.62, stage/render/prepare_assets 18.11, encode//sum 17.33, stage/render/render 13.94, stage/main/post_update 13.17, render/graph/ms 11.94, stage/render/graph_submit 8.92 |
| warm-up | 5.23 | 28.1 | pipelines/compile_ms 477, pipelines/front_ms 461, render/thread/ms 38.76, main/busy/ms 36.98, render/busy/ms 36.91, stage/main/update 23.06, encode//sum 18.21, pipelines/driver_ms 16.04, stage/render/render 14.29, stream/drive/ms 13.92, render/graph/ms 11.78, stage/render/prepare_assets 9.01 |
| warm-up | 5.86 | 28.8 | render/thread/ms 34.96, render/busy/ms 33.82, main/busy/ms 21.62, stage/render/prepare_assets 18.68, encode//sum 14.57, stage/main/update 9.31, stage/render/render 8.81, stage/main/post_update 7.88, render/graph/ms 7.28, stage/render/graph_submit 5.26, stream/drive/ms 3.33, encode/finish_wall 2.81 |
| warm-up | 6.08 | 32.5 | render/thread/ms 44.15, render/busy/ms 42.74, main/busy/ms 39.04, encode//sum 21.10, stage/main/update 19.14, stage/main/post_update 15.05, stage/render/render 14.36, render/graph/ms 12.13, stage/render/prepare_assets 9.89, stage/render/graph_submit 8.07, stage/render/prepare_meshes 5.84, stage/render/prepare_bind_groups 4.38 |
| warm-up | 6.14 | 33.9 | render/thread/ms 46.90, render/busy/ms 45.64, main/busy/ms 39.13, stage/main/update 20.39, encode//sum 17.60, stage/render/prepare_assets 14.41, stage/main/post_update 12.33, stage/render/render 11.88, render/graph/ms 10.28, stage/render/graph_submit 7.59, stage/render/prepare_meshes 6.41, stream/drive/ms 5.12 |
| warm-up | 6.18 | 41.6 | render/thread/ms 65.51, render/busy/ms 63.94, main/busy/ms 48.80, stage/main/update 24.69, encode//sum 24.17, stage/render/prepare_assets 21.52, stage/main/post_update 18.45, stage/render/render 16.68, render/graph/ms 14.38, stage/render/graph_submit 10.38, stage/render/prepare_meshes 8.67, encode/finish_wall 5.62 |
| warm-up | 6.24 | 30.1 | pipelines/compile_ms 85.80, pipelines/front_ms 81.52, render/thread/ms 42.98, render/busy/ms 40.13, main/busy/ms 33.61, encode//sum 20.38, stage/main/update 17.75, stage/render/prepare_assets 16.45, stage/render/render 13.52, stage/main/post_update 11.56, render/graph/ms 10.22, stage/render/graph_submit 7.21 |
| warm-up | 6.38 | 51.1 | main/busy/ms 59.86, render/thread/ms 43.16, render/busy/ms 41.68, stage/main/update 39.73, encode//sum 24.15, stage/render/render 16.87, stage/main/post_update 16.50, stream/drive/ms 14.87, render/graph/ms 14.83, stage/render/prepare_assets 12.07, stage/render/graph_submit 10.90, encode/submit 6.21 |
| warm-up | 6.42 | 38.9 | render/thread/ms 60.64, render/busy/ms 59.12, main/busy/ms 47.07, stage/main/update 28.09, encode//sum 24.51, stage/render/prepare_assets 20.09, stage/render/render 15.32, stage/main/post_update 14.03, stage/render/prepare_write_phase_buffers 13.89, render/graph/ms 13.08, stage/render/graph_submit 9.11, stream/drive/ms 8.73 |
| measure | 18.15 | 32.5 | main/busy/ms 39.61, stage/main/update 26.36, render/thread/ms 24.54, render/busy/ms 23.29, encode//sum 22.13, stage/render/render 13.74, render/graph/ms 12.17, stage/main/post_update 10.07, stage/render/graph_submit 9.71, encode/submit 5.34, encode/finish_wall 4.42, stage/render/prepare_assets 4.14 |
| measure | 59.02 | 32.9 | render/thread/ms 42.12, render/busy/ms 40.79, main/busy/ms 24.64, encode//sum 20.17, stage/render/prepare_assets 18.50, stage/main/post_update 12.62, stage/render/render 11.74, render/graph/ms 10.25, stage/render/graph_submit 7.89, stage/main/update 6.64, encode/finish_wall 4.03, encode/submit 3.95 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\f_scene\balmora_walk@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **80%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.5 W · 73 °C (max 78) · VRAM 5238 MiB · throttled 100% of samples
