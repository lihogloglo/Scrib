# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5448 frames over 65.6 s · **median 11.73 ms (85 fps)** · mean 12.05 · p95 14.67 · p99 17.17 (1% low 58 fps) · worst 37.42 ms · 41 spikes over 1.5× median · 28.3 stutters/min (work 23.8/min)
- warm-up: 1494 frames over 19.9 s, worst 3310.6 ms
- hitches (> 29.3 ms): **3** measured, 23 in warm-up
- timed GPU passes: 7.16 ms of a 12.05 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.73 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 1.09 min = 28.33/min, limit 1/min
- **FAIL** worst frame 37.42 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.007 | 2.064 | 2.095 | 0.97 | 0.175 |
| main_opaque_pass_3d | 1.593 | 1.638 | 2.040 | 0.97 | 0.053 |
| early prepass | 0.523 | 0.538 | 1.069 | 0.97 | 0.077 |
| bloom | 0.311 | 0.320 | 0.327 | 0.97 | 0.063 |
| atmosphere_luts | 0.303 | 0.312 | 0.371 | 0.97 | 0.003 |
| water_velocity | 0.281 | 0.289 | 0.365 | 0.97 | 0.002 |
| water_sspr | 0.203 | 0.209 | 0.210 | 0.97 | 0.002 |
| ssao | 0.188 | 0.193 | 0.195 | 0.97 | 0.007 |
| uniform_allocation | 0.174 | 0.179 | 0.190 | 0.97 | 0.031 |
| atmos_froxel_inject | 0.144 | 0.148 | 0.150 | 0.97 | 0.002 |
| auxiliary_views | 0.140 | 0.144 | 0.528 | 0.97 | 0.135 |
| clouds_trace | 0.137 | 0.141 | 0.237 | 0.97 | 0.001 |
| bin_unpacking | 0.131 | 0.134 | 0.147 | 0.97 | 0.023 |
| water_fft | 0.129 | 0.133 | 0.137 | 0.97 | 0.003 |
| upscaling | 0.114 | 0.117 | 0.126 | 0.97 | 0.004 |
| clouds_composite | 0.104 | 0.107 | 0.111 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.086 | 0.089 | 0.170 | 0.97 | 0.008 |
| tonemapping | 0.084 | 0.086 | 0.090 | 0.97 | 0.003 |
| clustering | 0.064 | 0.066 | 0.069 | 0.97 | 0.019 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.451 | 0.724 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.050 | 0.050 | 0.97 | 0.001 |
| surface_trail | 0.042 | 0.057 | 0.057 | 0.75 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.042 | 0.048 | 0.97 | 0.005 |
| water_snapshot | 0.031 | 0.032 | 0.035 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.033 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.026 | 0.222 | 0.233 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.024 | 0.027 | 0.97 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 0.97 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.97 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 83.471 | 0.000 | 20216.99 | 454751.0 |
| encode//sum | 10.734 | 16.240 | 58.20 | 58481.1 |
| render/thread/ms | 10.732 | 13.254 | 34.95 | 58466.7 |
| render/busy/ms | 10.145 | 12.643 | 33.86 | 55269.1 |
| main/busy/ms | 9.075 | 11.640 | 30.44 | 49438.2 |
| stage/render/render | 7.031 | 8.857 | 18.89 | 38306.3 |
| pipelines/compile_ms | 6.303 | 0.000 | 3061.30 | 34337.8 |
| render/graph/ms | 6.249 | 8.031 | 17.42 | 34044.4 |
| pipelines/driver_ms | 5.764 | 0.000 | 2913.99 | 31404.7 |
| stage/main/post_update | 5.660 | 7.925 | 10.58 | 30836.9 |
| stage/render/graph_submit | 5.108 | 6.597 | 13.70 | 27827.3 |
| encode/finish_wall | 2.919 | 3.963 | 6.31 | 15903.0 |
| encode/submit | 2.252 | 3.072 | 7.44 | 12269.4 |
| stage/main/update | 2.134 | 2.687 | 24.28 | 11628.6 |
| stage/render/camera_driver | 1.084 | 1.476 | 3.97 | 5904.8 |
| stage/render/prepare_write_phase_buffers | 0.878 | 1.135 | 2.57 | 4782.3 |
| bigspace/total/ms | 0.589 | 1.946 | 4.71 | 3206.7 |
| stage/render/prepare_assets | 0.552 | 0.696 | 8.83 | 3008.7 |
| pipelines/front_ms | 0.538 | 0.000 | 174.17 | 2933.1 |
| render/present/ms | 0.492 | 0.649 | 2.69 | 2682.3 |
| stage/render/prepare_resources | 0.426 | 0.518 | 0.98 | 2323.3 |
| stage/main/first | 0.412 | 0.539 | 5.17 | 2246.3 |
| stage/render/prepare_meshes | 0.355 | 0.472 | 1.84 | 1935.0 |
| stage/main/pre_update | 0.329 | 0.480 | 1.06 | 1792.5 |
| terrain/update/ms | 0.293 | 0.379 | 1.87 | 1597.7 |
| controller/feed/ms | 0.261 | 0.344 | 0.97 | 1423.3 |
| terrain/select/ms | 0.260 | 0.335 | 0.68 | 1417.0 |
| stage/render/specialize | 0.258 | 0.384 | 2.36 | 1406.2 |
| stream/drive/ms | 0.257 | 0.368 | 22.03 | 1401.9 |
| stage/render/prepare_bind_groups | 0.254 | 0.463 | 3.19 | 1382.3 |
| stage/render/queue | 0.252 | 0.407 | 2.49 | 1371.1 |
| bigspace/low_precision/ms | 0.244 | 0.707 | 4.01 | 1329.8 |
| stage/main/fixed_main_loop | 0.239 | 0.389 | 1.29 | 1301.5 |
| stream/tick/ms | 0.196 | 0.276 | 0.50 | 1069.5 |
| bigspace/high_precision/ms | 0.190 | 0.640 | 4.31 | 1035.7 |
| far/select/ms | 0.188 | 0.259 | 0.57 | 1023.5 |
| stage/render/create_views | 0.182 | 0.236 | 8.45 | 989.5 |
| stage/render/prepare_batch_phases | 0.172 | 0.240 | 0.58 | 937.4 |
| stage/render/extract_commands | 0.170 | 0.209 | 0.44 | 928.5 |
| stage/render/prepare_flush | 0.151 | 0.273 | 0.74 | 821.0 |
| anim/evaluate_cpu/ms | 0.146 | 0.196 | 0.68 | 797.6 |
| stage/render/prepare_views | 0.137 | 0.178 | 0.42 | 745.4 |
| anim/evaluate/ms | 0.130 | 0.236 | 3.64 | 707.1 |
| controller/move/ms | 0.123 | 0.224 | 0.53 | 667.7 |
| stage/main/last | 0.117 | 0.209 | 2.57 | 638.4 |
| render/acquire/ms | 0.095 | 0.129 | 0.36 | 515.3 |
| physics/insert/ms | 0.061 | 0.463 | 1.13 | 333.6 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.121 | 0.29 | 306.3 |
| physics/step/ms | 0.054 | 0.091 | 0.87 | 291.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.043 | 0.53 | 165.6 |
| stage/render/cleanup | 0.029 | 0.071 | 0.72 | 157.2 |
| stage/render/phase_sort | 0.025 | 0.044 | 0.22 | 138.6 |
| weather/map/ms | 0.024 | 0.035 | 0.24 | 130.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.32 | 81.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.27 | 60.0 |
| anim/apply/ms | 0.011 | 0.021 | 0.16 | 60.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 56.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.29 | 53.1 |
| script/update/ms | 0.007 | 0.010 | 0.28 | 38.1 |
| stream/despawn/ms | 0.005 | 0.000 | 0.96 | 28.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 27.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 26.1 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 25.3 |
| controller/write/ms | 0.003 | 0.004 | 0.02 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.78 | 14.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.0 |
| stage/render/prepare | 0.001 | 0.002 | 0.06 | 2.8 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 3 | 0 |
| anim/a1/count | count | 0 | 27 | 0 |
| anim/a2/count | count | 1 | 11 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 83 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 68 | 13 |
| crowd/c0/joints/count | count | 1306 | 2672 | 593 |
| crowd/c0/meshes/count | count | 355 | 655 | 156 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5507 | 5895 | 5603 |
| crowd/c1/meshes/count | count | 1160 | 1267 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4273 | 8028 | 6179 |
| crowd/joints_uploaded/count | count | 4248 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.47 | 4.99 | 2.06 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.32 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.27 | 0.01 |
| encode/finish_critical | ms | 2.47 | 4.99 | 2.06 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.99 | 33.86 | 8.86 |
| gpu/timed/ms | ms | 8.04 | 11.67 | 8.86 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 810 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 810 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3400 | 5126 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3400 | 5126 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 228 | 28 |
| phases/shadow/bins | count | 4823 | 5940 | 2221 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 13.0 k | 5672 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 249 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2630 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 185 | 187 | 187 |
| pipelines/waiting | count | 0 | 37 | 0 |
| preprocess/bind_groups | count | 10 | 615 | 6 |
| preprocess/bind_groups_created | count | 10 | 615 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 183 | 266 | 173 |
| process/cpu_usage | % | 26.15 | 54.79 | 27.75 |
| process/mem_usage | GiB | 5.35 | 5.48 | 5.48 |
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
| started_load_count |  | 4771 | 4865 | 4865 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.53 | 59.40 | 36.19 |
| system/mem_usage | % | 61.97 | 62.59 | 62.59 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 86 | 275 | 78 |
| views/all/bins | count | 7395 | 8856 | 5719 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.4 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 247 | 294 | 74 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.3 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2618 | 3153 | 2221 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8146 | 10.0 k | 5672 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8146 | 10.0 k | 5672 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2147 | 2858 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2403 | 3148 | 0 |
| views/lamp_face/pipelines | count | 168 | 206 | 0 |
| views/lamp_face/work_items | count | 0 | 2932 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2860 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8412 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8578 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 51.33 | 1021 | 279.6 k |
| anim/events/count | count | 0.09 | 3 | 510 |
| assets/Image/added | count | 0.12 | 26 | 672 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.80 | 152 | 4333 |
| assets/Mesh/removed | count | 0.90 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 557 |
| assets/StandardMaterial/modified | count | 3.98 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 99.84 | 2730 | 543.9 k |
| ecs/entities_delta | count | 2.34 | 3595 | 12.8 k |
| encode//count | count | 38.78 | 271 | 211.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5448 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5448 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5448 |
| physics/inserts | count | 0.64 | 111 | 3507 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 145 | 2335 |
| physics/steps | count | 0.72 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 3 | 4 |
| pipelines/created | count | 0.00 | 3 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6214 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 62 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1362 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.3 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.31 | 3310.6 | main/busy/ms 3284, stage/main/post_update 3180, stage/main/update 96.69, surface/advance_cpu_ms 67.17, stage/main/pre_update 3.29, stage/main/last 1.02, surface/cell_upload_ms 0.73, stage/main/first 0.64, stage/main/fixed_main_loop 0.51, stream/drive/ms 0.41, weather/map/ms 0.24, stream/tick/ms 0.14 |
| warm-up | 4.11 | 800.9 | main/busy/ms 3323, stage/main/post_update 3210, render/thread/ms 761, render/busy/ms 592, stage/render/prepare_assets 442, stage/render/prepare_views 167, render/acquire/ms 167, stage/render/prepare_resources 117, stage/main/update 100, surface/advance_cpu_ms 67.17, stage/render/create_views 19.78, stage/render/render 9.82 |
| warm-up | 4.21 | 101.1 | render/thread/ms 777, render/busy/ms 605, stage/render/prepare_assets 444, pipelines/compile_ms 353, pipelines/driver_ms 297, pipelines/wait_ms 238, stage/render/prepare_views 168, render/acquire/ms 168, stage/render/prepare_resources 122, main/busy/ms 58.09, pipelines/front_ms 56.00, stage/main/post_update 40.03 |
| warm-up | 4.43 | 43.7 | pipelines/compile_ms 237, pipelines/driver_ms 231, pipelines/wait_ms 207, render/thread/ms 65.92, render/busy/ms 36.11, main/busy/ms 33.97, stage/render/prepare_views 28.47, render/acquire/ms 28.27, stage/render/prepare_assets 26.51, stage/main/post_update 16.30, stage/main/update 14.17, stage/render/render 6.66 |
| warm-up | 4.58 | 101.5 | pipelines/wait_ms 1511, render/thread/ms 84.12, render/busy/ms 79.40, main/busy/ms 74.84, stage/render/prepare_assets 67.69, stage/main/update 64.22, pipelines/compile_ms 63.52, pipelines/driver_ms 48.66, pipelines/front_ms 14.87, stage/render/render 7.64, render/graph/ms 5.30, stage/main/post_update 4.83 |
| warm-up | 4.67 | 33.4 | pipelines/wait_ms 585, pipelines/compile_ms 104, pipelines/driver_ms 85.96, render/thread/ms 41.76, render/busy/ms 39.46, stage/render/prepare_assets 24.09, main/busy/ms 23.99, pipelines/front_ms 17.55, stage/main/update 9.70, stage/render/render 8.95, stage/main/post_update 8.08, render/graph/ms 5.80 |
| warm-up | 4.78 | 80.9 | pipelines/wait_ms 1402, pipelines/compile_ms 64.72, main/busy/ms 62.74, render/thread/ms 59.37, render/busy/ms 57.63, stage/main/update 47.30, stage/render/prepare_assets 46.38, stream/drive/ms 42.66, pipelines/front_ms 33.14, pipelines/driver_ms 31.58, stage/main/post_update 8.72, stage/render/render 6.46 |
| warm-up | 4.84 | 36.1 | pipelines/wait_ms 581, render/thread/ms 46.66, render/busy/ms 44.88, stage/render/prepare_assets 35.02, main/busy/ms 27.57, pipelines/compile_ms 18.28, pipelines/driver_ms 15.58, stage/main/post_update 12.01, stage/main/update 8.19, stage/render/render 6.01, terrain/update/ms 5.22, render/graph/ms 3.91 |
| warm-up | 4.90 | 43.5 | pipelines/wait_ms 1245, pipelines/compile_ms 71.20, pipelines/driver_ms 55.96, render/thread/ms 47.68, render/busy/ms 45.80, stage/render/prepare_assets 31.91, main/busy/ms 24.07, pipelines/front_ms 15.24, stage/main/post_update 10.88, stage/render/render 9.25, stage/main/update 8.02, render/graph/ms 6.91 |
| warm-up | 5.11 | 59.1 | pipelines/wait_ms 2485, pipelines/compile_ms 74.64, pipelines/driver_ms 54.84, main/busy/ms 53.81, stage/main/update 42.60, stream/drive/ms 37.68, render/thread/ms 24.41, render/busy/ms 23.13, pipelines/front_ms 19.80, stage/render/prepare_assets 14.48, stage/main/post_update 6.63, stage/render/render 5.03 |
| warm-up | 5.23 | 43.3 | pipelines/wait_ms 1884, pipelines/compile_ms 87.51, pipelines/driver_ms 79.48, render/thread/ms 48.41, render/busy/ms 45.24, main/busy/ms 35.84, stage/main/update 22.10, stream/drive/ms 18.38, stage/render/render 17.45, encode//sum 15.97, stage/render/prepare_assets 13.46, render/graph/ms 13.21 |
| warm-up | 5.26 | 29.6 | pipelines/wait_ms 1884, pipelines/compile_ms 87.51, pipelines/driver_ms 79.48, encode//sum 62.46, render/thread/ms 54.60, render/busy/ms 51.48, stage/render/render 22.59, main/busy/ms 21.54, render/graph/ms 18.52, stage/render/prepare_assets 14.99, stage/render/graph_submit 12.81, stage/main/post_update 9.91 |
| warm-up | 5.29 | 34.4 | encode//sum 52.30, render/thread/ms 48.93, render/busy/ms 46.87, main/busy/ms 31.24, stage/render/prepare_assets 17.45, stage/render/render 16.19, stage/main/update 14.11, render/graph/ms 13.34, stream/drive/ms 9.86, stage/main/post_update 9.67, stage/render/graph_submit 9.61, encode/finish_wall 5.86 |
| warm-up | 5.33 | 37.2 | main/busy/ms 64.11, render/thread/ms 55.76, render/busy/ms 54.00, stage/main/update 46.67, stream/drive/ms 41.26, stage/render/prepare_assets 26.29, stage/main/post_update 9.95, stage/render/render 7.16, render/graph/ms 4.80, stage/render/specialize 4.31, stage/render/prepare_write_phase_buffers 3.81, stream/spawn/ms 3.78 |
| warm-up | 5.43 | 97.7 | pipelines/wait_ms 2176, pipelines/compile_ms 204, pipelines/driver_ms 179, main/busy/ms 92.66, render/thread/ms 82.48, render/busy/ms 80.84, stage/main/update 75.88, stream/drive/ms 70.48, stage/render/prepare_assets 50.22, pipelines/front_ms 24.81, stage/main/post_update 8.72, stage/render/render 7.54 |
| warm-up | 5.48 | 57.8 | pipelines/wait_ms 2702, pipelines/compile_ms 1137, pipelines/driver_ms 1053, pipelines/front_ms 84.41, render/thread/ms 83.43, render/busy/ms 82.02, main/busy/ms 62.78, stage/render/prepare_assets 62.09, stage/main/update 44.61, stream/drive/ms 39.53, stage/main/post_update 8.19, stage/render/render 6.93 |
| warm-up | 5.53 | 47.0 | pipelines/compile_ms 933, pipelines/driver_ms 874, pipelines/wait_ms 525, main/busy/ms 94.22, render/thread/ms 77.26, stage/main/update 73.17, stream/drive/ms 68.56, pipelines/front_ms 59.60, render/busy/ms 47.36, stage/render/render 35.30, stage/render/prepare_assets 31.06, render/present/ms 29.65 |
| warm-up | 5.61 | 76.1 | main/busy/ms 105, stage/main/update 86.36, stream/drive/ms 81.59, render/thread/ms 71.45, render/busy/ms 41.30, stage/render/render 35.45, render/present/ms 29.82, stage/render/prepare_assets 22.75, stage/main/post_update 12.74, render/graph/ms 4.63, stage/render/graph_submit 3.29, stage/render/specialize 2.72 |
| warm-up | 6.28 | 36.4 | pipelines/wait_ms 4394, pipelines/compile_ms 117, pipelines/driver_ms 89.44, render/thread/ms 48.58, render/busy/ms 46.92, stage/render/prepare_assets 29.21, main/busy/ms 28.00, pipelines/front_ms 27.67, stage/main/post_update 11.03, stage/main/update 9.72, stage/render/render 6.86, stream/drive/ms 5.00 |
| warm-up | 6.62 | 32.0 | main/busy/ms 38.72, render/thread/ms 26.37, render/busy/ms 24.28, stage/main/update 21.10, stream/drive/ms 16.24, stage/main/post_update 12.20, stage/render/render 9.65, render/graph/ms 6.98, stage/render/graph_submit 4.86, stage/render/prepare_assets 4.57, encode//sum 4.27, encode/submit 3.85 |
| warm-up | 11.11 | 37.1 | main/busy/ms 52.61, stage/main/update 38.46, render/thread/ms 27.56, render/busy/ms 25.48, stage/render/render 12.97, stage/main/post_update 11.07, render/graph/ms 9.83, encode//sum 9.53, stage/render/graph_submit 6.24, stage/render/prepare_assets 4.70, encode/submit 4.05, stage/render/camera_driver 3.31 |
| warm-up | 11.15 | 40.1 | pipelines/wait_ms 2969, main/busy/ms 44.12, render/thread/ms 38.78, pipelines/compile_ms 38.30, render/busy/ms 36.85, stage/main/update 27.39, pipelines/driver_ms 21.28, stage/render/prepare_assets 17.24, pipelines/front_ms 17.01, stage/main/post_update 12.35, stage/render/render 10.78, render/graph/ms 7.85 |
| warm-up | 15.30 | 51.0 | main/busy/ms 45.48, render/thread/ms 35.14, render/busy/ms 33.74, stage/main/update 29.85, stage/render/prepare_assets 15.31, encode//sum 13.33, stage/main/post_update 12.41, stage/render/render 10.20, render/graph/ms 8.25, stage/render/graph_submit 6.44, encode/submit 3.65, encode/finish_wall 2.87 |
| measure | 2.72 | 36.2 | pipelines/wait_ms 17.4 k, encode//sum 68.14, pipelines/compile_ms 46.94, render/thread/ms 46.70, render/busy/ms 44.95, pipelines/driver_ms 27.79, stage/render/render 26.53, render/graph/ms 24.22, main/busy/ms 19.40, pipelines/front_ms 19.14, stage/render/graph_submit 19.00, stage/main/post_update 11.79 |
| measure | 18.18 | 31.9 | main/busy/ms 39.68, stage/main/update 26.66, render/thread/ms 22.54, encode//sum 22.31, stream/drive/ms 22.28, render/busy/ms 21.51, stage/render/render 14.28, render/graph/ms 12.79, stage/main/post_update 10.30, stage/render/graph_submit 10.23, encode/finish_wall 5.78, encode/submit 4.49 |
| measure | 59.05 | 37.4 | render/thread/ms 37.10, render/busy/ms 35.35, encode//sum 24.44, main/busy/ms 23.06, stage/render/render 18.09, render/graph/ms 15.85, stage/main/post_update 12.47, stage/render/graph_submit 12.23, stage/render/prepare_assets 9.58, encode/finish_wall 6.67, stage/main/update 6.07, encode/submit 5.65 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\scene_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **78%** (min 58%) · 2582 MHz (min 2580) · memory 8001 MHz · 77.2 W · 69 °C (max 76) · VRAM 6118 MiB · throttled 100% of samples
