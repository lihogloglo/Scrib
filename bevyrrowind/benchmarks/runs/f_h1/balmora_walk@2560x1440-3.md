# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5640 frames over 65.6 s · **median 11.24 ms (89 fps)** · mean 11.64 · p95 14.46 · p99 16.35 (1% low 61 fps) · worst 31.13 ms · 45 spikes over 1.5× median · 21.9 stutters/min (work 21.0/min)
- warm-up: 323 frames over 8.9 s, worst 2522.8 ms
- hitches (> 28.1 ms): **3** measured, 38 in warm-up
- timed GPU passes: 7.40 ms of a 11.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.24 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 31.13 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.038 | 2.073 | 2.110 | 0.98 | 0.177 |
| main_opaque_pass_3d | 1.615 | 1.642 | 2.029 | 0.98 | 0.056 |
| early prepass | 0.543 | 0.553 | 0.796 | 0.98 | 0.078 |
| bloom | 0.314 | 0.320 | 0.326 | 0.98 | 0.064 |
| atmosphere_luts | 0.309 | 0.314 | 0.372 | 0.98 | 0.003 |
| water_velocity | 0.279 | 0.283 | 0.353 | 0.98 | 0.001 |
| water_sspr | 0.202 | 0.206 | 0.206 | 0.98 | 0.002 |
| auxiliary_views | 0.189 | 0.193 | 0.687 | 0.98 | 0.170 |
| ssao | 0.188 | 0.192 | 0.195 | 0.98 | 0.007 |
| uniform_allocation | 0.172 | 0.175 | 0.450 | 0.98 | 0.023 |
| per_view_shadow_pass_cascade1 | 0.148 | 0.589 | 0.660 | 0.25 | 0.039 |
| atmos_froxel_inject | 0.145 | 0.147 | 0.148 | 0.98 | 0.001 |
| clouds_trace | 0.140 | 0.142 | 0.234 | 0.98 | 0.001 |
| water_fft | 0.131 | 0.133 | 0.139 | 0.98 | 0.003 |
| upscaling | 0.116 | 0.118 | 0.128 | 0.98 | 0.004 |
| bin_unpacking | 0.108 | 0.110 | 0.133 | 0.98 | 0.017 |
| clouds_composite | 0.106 | 0.108 | 0.110 | 0.98 | 0.002 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.084 | 0.085 | 0.162 | 0.98 | 0.008 |
| clustering | 0.064 | 0.065 | 0.068 | 0.98 | 0.018 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.433 | 0.698 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| surface_trail | 0.041 | 0.057 | 0.057 | 0.73 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.053 | 0.98 | 0.004 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.225 | 0.238 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.027 | 0.98 | 0.158 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.004 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_downsampling_second_pass | 0.003 | 0.023 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.014 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.11 | 0.003 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.037 | 0.050 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.044 | 0.048 | 0.00 | 0.012 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.984 | 17.371 | 45.94 | 61952.4 |
| render/thread/ms | 10.333 | 12.977 | 25.47 | 58276.6 |
| render/busy/ms | 9.760 | 12.404 | 24.67 | 55046.1 |
| main/busy/ms | 8.784 | 10.823 | 28.52 | 49541.2 |
| stage/render/render | 6.447 | 8.219 | 16.49 | 36360.4 |
| render/graph/ms | 5.662 | 7.357 | 15.27 | 31936.4 |
| stage/main/post_update | 5.401 | 7.116 | 10.66 | 30459.3 |
| stage/render/graph_submit | 4.441 | 5.728 | 11.38 | 25048.9 |
| encode/submit | 2.279 | 3.116 | 6.16 | 12851.8 |
| encode/finish_wall | 2.225 | 2.855 | 6.40 | 12548.1 |
| stage/main/update | 2.142 | 2.552 | 22.66 | 12078.6 |
| stage/render/camera_driver | 1.166 | 1.635 | 4.15 | 6577.3 |
| stage/render/prepare_write_phase_buffers | 0.770 | 1.085 | 2.24 | 4343.9 |
| stage/render/prepare_assets | 0.554 | 0.719 | 9.25 | 3124.5 |
| bigspace/total/ms | 0.531 | 1.519 | 3.29 | 2993.6 |
| render/present/ms | 0.482 | 0.575 | 5.61 | 2716.2 |
| stage/render/prepare_meshes | 0.455 | 0.627 | 2.74 | 2566.4 |
| stage/render/prepare_resources | 0.449 | 0.541 | 1.27 | 2533.1 |
| stage/main/first | 0.394 | 0.514 | 5.51 | 2220.5 |
| stage/main/pre_update | 0.329 | 0.486 | 1.76 | 1856.7 |
| terrain/update/ms | 0.307 | 0.402 | 2.04 | 1730.8 |
| stage/render/specialize | 0.297 | 0.449 | 3.92 | 1674.8 |
| terrain/select/ms | 0.274 | 0.361 | 0.82 | 1547.1 |
| controller/feed/ms | 0.272 | 0.345 | 0.66 | 1532.6 |
| stage/render/prepare_bind_groups | 0.265 | 0.462 | 2.53 | 1493.6 |
| stream/drive/ms | 0.237 | 0.321 | 20.62 | 1337.3 |
| stage/render/prepare_batch_phases | 0.230 | 0.312 | 0.72 | 1299.1 |
| stage/render/queue | 0.228 | 0.384 | 3.11 | 1285.3 |
| stage/main/fixed_main_loop | 0.223 | 0.363 | 1.29 | 1255.4 |
| bigspace/low_precision/ms | 0.218 | 0.632 | 2.77 | 1230.3 |
| anim/evaluate/ms | 0.187 | 0.286 | 2.21 | 1056.3 |
| far/select/ms | 0.186 | 0.247 | 0.48 | 1046.7 |
| stage/render/create_views | 0.183 | 0.241 | 0.89 | 1033.6 |
| stream/tick/ms | 0.181 | 0.247 | 0.55 | 1023.2 |
| stage/render/prepare_flush | 0.175 | 0.357 | 0.84 | 989.1 |
| stage/render/extract_commands | 0.171 | 0.202 | 0.53 | 963.2 |
| bigspace/high_precision/ms | 0.163 | 0.630 | 2.95 | 918.3 |
| anim/evaluate_cpu/ms | 0.146 | 0.188 | 0.44 | 821.9 |
| stage/render/prepare_views | 0.131 | 0.163 | 0.42 | 739.0 |
| controller/move/ms | 0.126 | 0.271 | 0.56 | 711.1 |
| stage/main/last | 0.120 | 0.192 | 1.61 | 676.9 |
| stage/render/phase_sort | 0.111 | 0.146 | 0.37 | 624.1 |
| render/acquire/ms | 0.091 | 0.114 | 0.36 | 514.4 |
| stage/render/cleanup | 0.079 | 0.204 | 0.73 | 447.9 |
| pipelines/compile_ms | 0.051 | 0.000 | 120.58 | 287.4 |
| pipelines/front_ms | 0.049 | 0.000 | 116.36 | 276.2 |
| physics/step/ms | 0.049 | 0.083 | 0.62 | 276.1 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.25 | 266.9 |
| physics/insert/ms | 0.034 | 0.374 | 0.93 | 194.6 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.46 | 169.4 |
| weather/map/ms | 0.025 | 0.032 | 0.18 | 140.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.41 | 82.1 |
| anim/apply/ms | 0.011 | 0.020 | 0.17 | 61.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.17 | 59.5 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.32 | 58.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 57.8 |
| script/update/ms | 0.007 | 0.008 | 0.17 | 36.9 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 29.2 |
| stream/despawn/ms | 0.005 | 0.000 | 0.94 | 29.0 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 26.6 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 25.3 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 13.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.22 | 11.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.2 |
| stage/render/prepare | 0.000 | 0.002 | 0.03 | 2.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.13 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 42 | 15 |
| crowd/c0/joints/count | count | 1301 | 2218 | 714 |
| crowd/c0/meshes/count | count | 353 | 633 | 198 |
| crowd/c1/count | count | 146 | 156 | 146 |
| crowd/c1/joints/count | count | 5527 | 5982 | 5482 |
| crowd/c1/meshes/count | count | 1165 | 1284 | 1143 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4603 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.81 | 6.26 | 1.93 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.41 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.32 | 0.01 |
| encode/finish_critical | ms | 1.81 | 6.26 | 1.93 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.46 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.57 | 28.52 | 8.84 |
| gpu/timed/ms | ms | 8.73 | 11.98 | 8.84 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 248 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3902 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3902 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 4091 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4091 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 206 | 5 |
| phases/shadow/bins | count | 10.4 k | 12.6 k | 8218 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 29.1 k | 32.4 k | 27.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 192 | 239 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 20 | 577 | 6 |
| preprocess/bind_groups_created | count | 20 | 577 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 135 | 238 | 125 |
| process/cpu_usage | % | 25.36 | 38.94 | 28.71 |
| process/mem_usage | GiB | 5.37 | 5.53 | 5.53 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 0 | 0 | 0 |
| scene/batch/masked/count | count | 0 | 0 | 0 |
| scene/batch/masked/high_water | count | 0 | 0 | 0 |
| scene/batch/masked/live | count | 0 | 0 | 0 |
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
| scene/batch/opaque/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque/count | count | 0 | 0 | 0 |
| scene/batch/opaque/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque/live | count | 0 | 0 | 0 |
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
| scene/batches/count | count | 0 | 0 | 0 |
| scene/cells/resident | count | 0 | 0 | 0 |
| scene/chains/high_water | count | 0 | 0 | 0 |
| scene/chains/live | count | 0 | 0 | 0 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 0 | 0 | 0 |
| scene/slots/batch_capacity | count | 0 | 0 | 0 |
| scene/slots/batch_capacity_bytes | bytes | 0 | 0 | 0 |
| scene/slots/batch_live | count | 0 | 0 | 0 |
| scene/slots/scene_bump | count | 0 | 0 | 0 |
| scene/slots/scene_live | count | 0 | 0 | 0 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 4914 | 5008 | 5008 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.64 | 45.27 | 28.31 |
| system/mem_usage | % | 50.51 | 51.07 | 51.01 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 70 | 257 | 55 |
| views/all/bins | count | 19.4 k | 21.4 k | 17.8 k |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 56.9 k | 60.9 k | 57.0 k |
| views/all/pipelines | count | 241 | 288 | 66 |
| views/all/pipelines_distinct | count | 72 | 76 | 61 |
| views/all/work_items | count | 41.3 k | 58.3 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 34 | 5 |
| views/cascade/bins | count | 8064 | 8796 | 8218 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 26.8 k | 28.3 k | 27.4 k |
| views/cascade/pipelines | count | 25 | 34 | 20 |
| views/cascade/work_items | count | 11.8 k | 28.1 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 3135 | 4330 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3394 | 4634 | 0 |
| views/lamp_face/pipelines | count | 167 | 208 | 0 |
| views/lamp_face/work_items | count | 0 | 4140 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8666 | 9602 | 9602 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 27.9 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 47 | 51 | 46 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 62.94 | 301 | 355.0 k |
| anim/events/count | count | 0.09 | 3 | 517 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.84 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1312 | 4676 |
| encode//count | count | 41.33 | 272 | 233.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5640 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5640 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5640 |
| physics/inserts | count | 0.40 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 21 | 378 |
| stream/upload/bytes | bytes | 6002 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1316 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2522.8 | main/busy/ms 2494, stage/main/post_update 2392, stage/main/update 95.19, surface/advance_cpu_ms 66.66, stage/main/pre_update 2.93, stage/main/last 1.20, surface/cell_upload_ms 0.70, stage/main/first 0.63, stream/drive/ms 0.56, stage/main/fixed_main_loop 0.39, stream/tick/ms 0.15, weather/map/ms 0.09 |
| warm-up | 3.23 | 703.3 | main/busy/ms 2627, stage/main/post_update 2413, render/thread/ms 674, render/busy/ms 507, stage/render/prepare_assets 470, stage/main/update 200, stage/render/prepare_views 166, render/acquire/ms 165, surface/advance_cpu_ms 66.66, stage/render/prepare_resources 21.76, stage/render/render 10.82, encode//sum 7.97 |
| warm-up | 3.29 | 68.4 | render/thread/ms 723, render/busy/ms 554, stage/render/prepare_assets 493, stage/render/prepare_views 167, render/acquire/ms 166, main/busy/ms 150, stage/main/update 109, pipelines/compile_ms 35.59, pipelines/front_ms 33.77, stage/main/post_update 30.80, stage/render/render 28.45, stage/render/prepare_resources 24.77 |
| warm-up | 3.32 | 30.1 | pipelines/compile_ms 230, pipelines/front_ms 208, render/thread/ms 76.93, render/busy/ms 63.26, pipelines/wait_ms 55.94, main/busy/ms 41.95, stage/render/render 39.80, stage/main/post_update 28.09, render/graph/ms 25.58, stage/render/prepare_assets 23.70, pipelines/driver_ms 22.38, terrain/update/ms 20.45 |
| warm-up | 3.48 | 153.3 | pipelines/wait_ms 4198, pipelines/compile_ms 1478, pipelines/front_ms 1361, render/thread/ms 166, render/busy/ms 153, pipelines/driver_ms 117, stage/render/prepare_write_phase_buffers 66.92, main/busy/ms 47.40, stage/render/render 41.52, stage/render/prepare_resources 37.45, stage/main/post_update 30.93, render/graph/ms 27.82 |
| warm-up | 3.65 | 152.9 | pipelines/compile_ms 1991, pipelines/front_ms 1835, encode//sum 312, pipelines/wait_ms 304, pipelines/driver_ms 157, render/thread/ms 117, render/busy/ms 114, main/busy/ms 89.02, stage/render/prepare_assets 64.10, stage/main/update 58.89, stage/render/render 43.63, render/graph/ms 41.10 |
| warm-up | 3.71 | 39.0 | pipelines/compile_ms 949, pipelines/front_ms 909, pipelines/wait_ms 259, render/thread/ms 55.07, render/busy/ms 51.09, pipelines/driver_ms 40.19, main/busy/ms 37.01, stage/render/prepare_assets 35.34, stage/main/post_update 17.21, stage/main/update 14.71, stage/render/render 11.67, terrain/update/ms 9.67 |
| warm-up | 3.76 | 28.5 | pipelines/compile_ms 74.63, pipelines/front_ms 67.62, render/thread/ms 45.01, render/busy/ms 42.65, main/busy/ms 28.17, stage/render/prepare_assets 17.32, stage/render/render 15.04, stage/main/post_update 14.17, render/graph/ms 12.39, stage/render/graph_submit 9.88, encode//sum 8.86, terrain/update/ms 8.13 |
| warm-up | 3.86 | 84.4 | pipelines/compile_ms 1290, pipelines/front_ms 1244, main/busy/ms 54.22, render/thread/ms 51.56, render/busy/ms 50.02, pipelines/driver_ms 46.40, stage/main/update 43.38, stream/drive/ms 38.98, stage/render/prepare_assets 33.24, stage/render/render 10.13, render/graph/ms 8.30, encode//sum 7.29 |
| warm-up | 4.00 | 72.0 | pipelines/compile_ms 214, pipelines/front_ms 201, render/thread/ms 59.51, render/busy/ms 57.58, main/busy/ms 44.39, stage/render/prepare_assets 39.41, stage/main/update 28.57, stream/drive/ms 24.67, pipelines/driver_ms 12.59, stage/render/render 11.99, stage/main/post_update 9.88, render/graph/ms 9.78 |
| warm-up | 4.13 | 106.0 | pipelines/compile_ms 908, pipelines/front_ms 788, pipelines/driver_ms 120, render/thread/ms 99.65, render/busy/ms 63.26, main/busy/ms 55.71, stage/render/render 46.06, stage/render/prepare_assets 44.98, stage/main/update 37.35, render/present/ms 35.86, stage/main/post_update 10.75, render/graph/ms 9.31 |
| warm-up | 4.18 | 51.4 | pipelines/compile_ms 1481, pipelines/front_ms 1198, pipelines/driver_ms 283, render/thread/ms 118, render/busy/ms 81.61, main/busy/ms 57.43, stage/render/prepare_assets 56.64, stage/render/render 47.54, stage/main/update 39.12, render/present/ms 36.02, stage/main/post_update 11.73, render/graph/ms 10.91 |
| warm-up | 4.24 | 55.7 | pipelines/compile_ms 957, pipelines/front_ms 772, pipelines/driver_ms 185, pipelines/wait_ms 110, render/thread/ms 82.31, render/busy/ms 80.51, main/busy/ms 58.56, stage/render/prepare_assets 38.93, stage/main/update 18.03, stage/main/first 17.35, stage/render/render 16.87, render/graph/ms 14.74 |
| warm-up | 4.33 | 90.2 | pipelines/compile_ms 760, pipelines/front_ms 726, pipelines/wait_ms 161, render/thread/ms 120, render/busy/ms 118, main/busy/ms 95.07, stage/render/prepare_assets 70.39, stage/main/post_update 52.81, pipelines/driver_ms 34.39, encode//sum 22.53, stage/render/render 19.32, stage/main/first 17.48 |
| warm-up | 4.43 | 101.3 | pipelines/compile_ms 2265, pipelines/front_ms 2187, pipelines/wait_ms 340, render/thread/ms 172, render/busy/ms 170, main/busy/ms 82.47, pipelines/driver_ms 77.96, encode//sum 75.60, stage/render/prepare_assets 57.51, stage/main/post_update 55.04, stage/render/prepare_resources 34.81, stage/render/prepare_write_phase_buffers 32.85 |
| warm-up | 4.49 | 61.4 | pipelines/compile_ms 2828, pipelines/front_ms 2724, pipelines/wait_ms 1180, render/thread/ms 149, render/busy/ms 147, pipelines/driver_ms 105, encode//sum 75.36, main/busy/ms 53.94, stage/render/prepare_resources 33.99, stage/render/prepare_write_phase_buffers 32.68, stage/render/prepare_assets 26.11, stage/render/render 25.95 |
| warm-up | 4.59 | 99.2 | pipelines/compile_ms 2198, pipelines/front_ms 2114, pipelines/wait_ms 1528, render/thread/ms 119, render/busy/ms 102, main/busy/ms 97.71, pipelines/driver_ms 84.20, stage/main/update 74.12, stream/drive/ms 60.46, stage/render/prepare_assets 60.36, stage/render/render 29.66, encode//sum 21.20 |
| warm-up | 4.64 | 46.1 | pipelines/compile_ms 2018, pipelines/front_ms 1933, pipelines/wait_ms 783, main/busy/ms 111, render/thread/ms 104, render/busy/ms 86.67, pipelines/driver_ms 84.35, stage/main/update 81.31, stream/drive/ms 61.98, stage/render/prepare_assets 50.38, stage/render/render 32.22, stage/main/post_update 18.90 |
| warm-up | 4.69 | 51.0 | pipelines/compile_ms 1391, pipelines/front_ms 1332, pipelines/wait_ms 314, main/busy/ms 114, stage/main/update 87.92, render/thread/ms 68.97, render/busy/ms 66.85, pipelines/driver_ms 58.98, stream/drive/ms 53.16, stage/render/prepare_assets 30.56, stage/render/render 19.39, render/graph/ms 16.34 |
| warm-up | 4.78 | 90.1 | pipelines/compile_ms 1027, pipelines/front_ms 998, pipelines/wait_ms 206, main/busy/ms 99.12, stage/main/update 80.83, render/thread/ms 69.02, render/busy/ms 67.32, stream/drive/ms 51.59, stage/render/prepare_assets 36.03, pipelines/driver_ms 28.21, encode//sum 14.51, stage/render/render 13.71 |
| warm-up | 4.81 | 31.2 | pipelines/compile_ms 824, pipelines/front_ms 801, main/busy/ms 80.19, render/thread/ms 61.32, render/busy/ms 59.51, stage/main/update 58.60, pipelines/wait_ms 44.03, stream/drive/ms 39.83, stage/render/prepare_assets 23.68, pipelines/driver_ms 22.29, encode//sum 16.59, stage/render/render 14.44 |
| warm-up | 4.89 | 77.2 | pipelines/compile_ms 824, pipelines/front_ms 797, main/busy/ms 77.17, stage/main/update 57.56, render/thread/ms 54.54, render/busy/ms 52.79, stream/drive/ms 40.12, pipelines/driver_ms 26.92, stage/render/prepare_assets 26.11, encode//sum 15.84, stage/render/render 13.53, stage/main/post_update 12.48 |
| warm-up | 4.97 | 57.6 | pipelines/compile_ms 357, pipelines/front_ms 346, main/busy/ms 60.31, stage/main/update 46.42, render/thread/ms 45.99, render/busy/ms 44.40, stream/drive/ms 33.32, encode//sum 17.20, stage/render/render 13.06, stage/render/prepare_assets 13.02, render/graph/ms 10.94, pipelines/driver_ms 10.42 |
| warm-up | 5.00 | 28.4 | pipelines/compile_ms 544, pipelines/front_ms 527, render/thread/ms 50.37, render/busy/ms 48.81, main/busy/ms 32.33, encode//sum 17.24, pipelines/driver_ms 16.58, stage/main/update 13.58, stage/main/post_update 13.57, stage/render/render 11.93, stage/render/prepare_assets 10.95, render/graph/ms 10.02 |
| warm-up | 5.05 | 28.4 | pipelines/compile_ms 301, pipelines/front_ms 291, render/thread/ms 45.96, render/busy/ms 43.99, main/busy/ms 32.61, stage/main/update 16.73, encode//sum 16.17, stage/render/render 13.97, render/graph/ms 11.65, pipelines/driver_ms 10.86, stage/render/prepare_assets 10.71, stage/main/post_update 10.32 |
| warm-up | 5.15 | 44.2 | pipelines/compile_ms 202, pipelines/front_ms 192, render/thread/ms 54.04, render/busy/ms 52.00, main/busy/ms 38.54, stage/render/prepare_assets 23.53, stage/main/update 19.05, encode//sum 16.23, stage/render/render 13.86, stage/main/post_update 12.26, render/graph/ms 11.44, pipelines/driver_ms 9.94 |
| warm-up | 5.18 | 32.4 | pipelines/compile_ms 397, pipelines/front_ms 384, render/thread/ms 59.42, render/busy/ms 57.84, main/busy/ms 40.71, stage/render/prepare_assets 32.94, stage/main/update 23.27, encode//sum 16.12, stream/drive/ms 14.38, pipelines/driver_ms 13.02, stage/render/render 11.69, stage/main/post_update 10.95 |
| warm-up | 5.22 | 34.7 | pipelines/compile_ms 226, pipelines/front_ms 221, render/thread/ms 57.23, render/busy/ms 55.70, main/busy/ms 37.20, stage/render/prepare_assets 32.62, stage/main/update 20.67, encode//sum 14.74, stream/drive/ms 13.21, stage/render/render 11.27, stage/main/post_update 10.45, render/graph/ms 9.18 |
| warm-up | 5.34 | 32.2 | pipelines/compile_ms 293, pipelines/front_ms 282, render/thread/ms 49.88, render/busy/ms 48.11, main/busy/ms 34.00, stage/render/prepare_assets 22.43, encode//sum 21.54, stage/render/render 18.67, render/graph/ms 16.36, stage/main/post_update 14.14, stage/main/update 14.01, stage/render/graph_submit 13.43 |
| warm-up | 5.38 | 38.8 | pipelines/compile_ms 488, pipelines/front_ms 469, render/thread/ms 62.86, render/busy/ms 61.27, main/busy/ms 35.59, stage/render/prepare_assets 33.06, encode//sum 22.93, pipelines/wait_ms 22.35, stage/render/render 18.94, pipelines/driver_ms 18.25, render/graph/ms 16.70, stage/main/post_update 14.03 |
| warm-up | 5.41 | 30.8 | pipelines/compile_ms 503, pipelines/front_ms 486, render/thread/ms 63.41, render/busy/ms 61.71, main/busy/ms 36.42, stage/render/prepare_assets 27.27, pipelines/wait_ms 22.43, encode//sum 16.76, pipelines/driver_ms 16.46, stage/main/post_update 16.04, stage/render/render 14.56, render/graph/ms 12.26 |
| warm-up | 5.45 | 41.2 | pipelines/compile_ms 429, pipelines/front_ms 416, render/thread/ms 64.63, render/busy/ms 63.07, stage/render/prepare_assets 33.08, main/busy/ms 26.77, encode//sum 16.05, stage/render/render 13.31, pipelines/wait_ms 13.18, stage/main/post_update 12.57, pipelines/driver_ms 12.18, render/graph/ms 11.19 |
| warm-up | 5.98 | 32.8 | render/thread/ms 36.30, render/busy/ms 35.09, main/busy/ms 24.48, stage/render/prepare_assets 15.73, encode//sum 13.49, stage/main/update 10.66, stage/render/render 9.43, stage/main/post_update 8.56, render/graph/ms 7.80, stage/render/graph_submit 5.76, stream/drive/ms 3.01, encode/finish_wall 2.94 |
| warm-up | 6.20 | 35.1 | render/thread/ms 51.13, render/busy/ms 49.24, main/busy/ms 40.50, encode//sum 22.70, stage/main/update 19.59, stage/render/render 16.33, stage/main/post_update 14.64, render/graph/ms 13.88, stage/render/graph_submit 11.08, stage/render/specialize 10.23, encode/submit 6.60, stage/render/queue 6.27 |
| warm-up | 6.25 | 35.8 | pipelines/compile_ms 76.28, pipelines/front_ms 72.68, render/thread/ms 43.34, render/busy/ms 42.29, main/busy/ms 28.90, stage/render/prepare_assets 20.14, encode//sum 16.23, stage/main/update 13.39, stage/main/post_update 9.72, stage/render/render 8.97, render/graph/ms 7.43, stage/render/graph_submit 5.52 |
| warm-up | 6.33 | 33.5 | render/thread/ms 43.89, render/busy/ms 42.67, main/busy/ms 33.73, stage/main/update 17.30, stage/render/prepare_assets 15.08, encode//sum 14.80, stage/main/post_update 10.12, stage/render/render 8.66, render/graph/ms 6.93, stage/render/prepare_meshes 6.42, stage/render/graph_submit 4.99, stage/render/specialize 4.88 |
| warm-up | 6.36 | 28.4 | render/thread/ms 49.05, render/busy/ms 47.87, main/busy/ms 31.42, stage/render/prepare_assets 18.58, stage/main/update 16.76, encode//sum 16.31, stage/main/post_update 10.06, stage/render/render 9.17, render/graph/ms 7.51, stage/render/prepare_meshes 6.47, stage/render/specialize 6.20, stage/render/graph_submit 5.56 |
| warm-up | 6.51 | 41.4 | render/thread/ms 60.40, render/busy/ms 59.20, main/busy/ms 37.48, stage/render/prepare_assets 31.76, stage/main/update 23.78, encode//sum 21.72, stream/drive/ms 16.05, stage/render/render 12.26, render/graph/ms 10.42, stage/main/post_update 9.20, stage/render/graph_submit 7.71, encode/finish_wall 4.19 |
| measure | 0.84 | 29.4 | render/thread/ms 42.25, render/busy/ms 41.19, main/busy/ms 29.01, encode//sum 22.25, stage/render/render 15.59, render/graph/ms 13.98, stage/main/post_update 13.98, stage/main/update 10.61, stage/render/graph_submit 10.43, stage/render/prepare_assets 8.33, encode/finish_wall 6.75, stage/render/specialize 5.02 |
| measure | 18.18 | 31.1 | main/busy/ms 38.32, stage/main/update 25.08, encode//sum 25.08, render/thread/ms 22.49, render/busy/ms 21.43, stream/drive/ms 20.84, stage/render/render 13.46, render/graph/ms 11.87, stage/main/post_update 10.35, stage/render/graph_submit 8.97, encode/submit 4.79, encode/finish_wall 4.25 |
| measure | 59.05 | 28.4 | render/thread/ms 41.23, render/busy/ms 39.95, main/busy/ms 26.57, encode//sum 21.29, stage/render/prepare_assets 14.85, stage/main/post_update 13.85, stage/render/render 12.35, render/graph/ms 10.71, stage/render/graph_submit 8.09, stage/main/update 7.90, encode/finish_wall 4.42, encode/submit 3.75 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_h1\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.1 W · 72 °C (max 77) · VRAM 5512 MiB · throttled 100% of samples
