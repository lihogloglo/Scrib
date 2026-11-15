# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5593 frames over 65.6 s · **median 11.43 ms (87 fps)** · mean 11.74 · p95 14.18 · p99 16.06 (1% low 62 fps) · worst 34.02 ms · 36 spikes over 1.5× median · 20.1 stutters/min (work 18.3/min)
- warm-up: 571 frames over 10.4 s, worst 2575.2 ms
- hitches (> 28.6 ms): **2** measured, 21 in warm-up
- timed GPU passes: 7.35 ms of a 11.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.43 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 34.02 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.005 | 2.064 | 2.097 | 0.97 | 0.170 |
| main_opaque_pass_3d | 1.565 | 1.611 | 1.968 | 0.97 | 0.056 |
| early prepass | 0.561 | 0.577 | 0.984 | 0.97 | 0.077 |
| bloom | 0.310 | 0.319 | 0.326 | 0.97 | 0.062 |
| atmosphere_luts | 0.306 | 0.315 | 0.375 | 0.97 | 0.003 |
| water_velocity | 0.274 | 0.282 | 0.353 | 0.97 | 0.001 |
| water_sspr | 0.210 | 0.217 | 0.218 | 0.97 | 0.002 |
| ssao | 0.187 | 0.193 | 0.193 | 0.97 | 0.007 |
| uniform_allocation | 0.163 | 0.167 | 0.174 | 0.97 | 0.028 |
| atmos_froxel_inject | 0.144 | 0.148 | 0.150 | 0.97 | 0.001 |
| clouds_trace | 0.137 | 0.141 | 0.234 | 0.97 | 0.001 |
| water_fft | 0.128 | 0.132 | 0.136 | 0.97 | 0.003 |
| auxiliary_views | 0.127 | 0.131 | 0.518 | 0.97 | 0.120 |
| bin_unpacking | 0.127 | 0.130 | 0.135 | 0.97 | 0.022 |
| early_mesh_preprocessing | 0.123 | 0.127 | 0.445 | 0.97 | 0.004 |
| upscaling | 0.114 | 0.118 | 0.126 | 0.97 | 0.004 |
| clouds_composite | 0.105 | 0.108 | 0.111 | 0.97 | 0.002 |
| gpu_instance_materialization | 0.089 | 0.091 | 0.095 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.089 | 0.167 | 0.97 | 0.008 |
| tonemapping | 0.084 | 0.086 | 0.090 | 0.97 | 0.003 |
| clustering | 0.066 | 0.068 | 0.069 | 0.97 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.471 | 0.746 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.050 | 0.050 | 0.97 | 0.001 |
| surface_trail | 0.042 | 0.057 | 0.057 | 0.73 | 0.002 |
| per_view_shadow_pass | 0.036 | 0.037 | 0.039 | 0.97 | 0.007 |
| water_snapshot | 0.030 | 0.031 | 0.034 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.033 | 0.97 | 0.003 |
| range_unpacking | 0.029 | 0.030 | 0.035 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.024 | 0.223 | 0.233 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.025 | 0.028 | 0.97 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.032 | 0.97 | 0.077 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.592 | 12.951 | 23.12 | 59239.4 |
| encode//sum | 10.271 | 15.174 | 42.10 | 57444.0 |
| render/busy/ms | 10.005 | 12.387 | 22.57 | 55956.4 |
| main/busy/ms | 8.794 | 11.227 | 30.41 | 49186.1 |
| stage/render/render | 6.721 | 8.388 | 15.67 | 37592.9 |
| render/graph/ms | 5.952 | 7.589 | 14.58 | 33292.0 |
| stage/main/post_update | 5.267 | 7.399 | 10.66 | 29457.6 |
| stage/render/graph_submit | 4.850 | 6.226 | 10.74 | 27125.9 |
| encode/finish_wall | 2.712 | 3.555 | 5.26 | 15166.6 |
| encode/submit | 2.188 | 2.978 | 6.05 | 12239.8 |
| stage/main/update | 2.141 | 2.602 | 24.48 | 11973.3 |
| stage/render/camera_driver | 1.041 | 1.373 | 3.76 | 5825.0 |
| stage/render/prepare_write_phase_buffers | 0.815 | 1.079 | 2.46 | 4558.4 |
| stage/render/prepare_assets | 0.542 | 0.704 | 11.70 | 3029.1 |
| bigspace/total/ms | 0.519 | 1.200 | 3.81 | 2903.4 |
| render/present/ms | 0.499 | 0.690 | 2.69 | 2789.7 |
| stage/render/prepare_resources | 0.424 | 0.507 | 1.18 | 2373.1 |
| stage/main/first | 0.413 | 0.537 | 4.90 | 2312.4 |
| stage/render/prepare_meshes | 0.374 | 0.505 | 2.95 | 2091.2 |
| stage/main/pre_update | 0.322 | 0.467 | 2.56 | 1801.1 |
| stage/render/prepare_flush | 0.313 | 0.501 | 1.32 | 1749.0 |
| stage/render/prepare_bind_groups | 0.301 | 0.495 | 2.63 | 1685.8 |
| terrain/update/ms | 0.296 | 0.377 | 1.58 | 1653.2 |
| terrain/select/ms | 0.263 | 0.336 | 0.62 | 1472.3 |
| controller/feed/ms | 0.262 | 0.342 | 0.75 | 1466.1 |
| stage/render/prepare_batch_phases | 0.256 | 0.354 | 0.54 | 1433.8 |
| stream/drive/ms | 0.249 | 0.345 | 12.18 | 1392.1 |
| stage/main/last | 0.246 | 0.308 | 2.61 | 1377.2 |
| stage/render/specialize | 0.239 | 0.349 | 2.11 | 1336.6 |
| bigspace/low_precision/ms | 0.230 | 0.616 | 3.18 | 1289.1 |
| stage/main/fixed_main_loop | 0.225 | 0.363 | 1.12 | 1257.4 |
| stage/render/queue | 0.221 | 0.341 | 1.95 | 1234.5 |
| stream/tick/ms | 0.193 | 0.263 | 0.81 | 1078.0 |
| far/select/ms | 0.188 | 0.255 | 0.54 | 1053.0 |
| stage/render/extract_commands | 0.171 | 0.205 | 0.60 | 955.1 |
| stage/render/create_views | 0.161 | 0.210 | 0.82 | 901.7 |
| anim/evaluate_cpu/ms | 0.158 | 0.197 | 0.41 | 884.7 |
| anim/evaluate/ms | 0.140 | 0.255 | 2.57 | 780.3 |
| bigspace/high_precision/ms | 0.136 | 0.287 | 3.46 | 761.2 |
| controller/move/ms | 0.133 | 0.244 | 0.63 | 743.6 |
| stage/render/prepare_views | 0.129 | 0.166 | 0.53 | 723.7 |
| pipelines/compile_ms | 0.102 | 0.000 | 148.87 | 572.6 |
| pipelines/front_ms | 0.099 | 0.000 | 143.65 | 553.1 |
| render/acquire/ms | 0.088 | 0.117 | 0.46 | 493.3 |
| physics/step/ms | 0.050 | 0.084 | 0.50 | 277.7 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.070 | 0.43 | 259.9 |
| physics/insert/ms | 0.033 | 0.370 | 0.84 | 183.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.44 | 169.3 |
| stage/render/cleanup | 0.028 | 0.050 | 0.83 | 154.8 |
| stage/render/phase_sort | 0.026 | 0.049 | 0.22 | 146.1 |
| weather/map/ms | 0.023 | 0.031 | 0.17 | 128.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.08 | 70.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.19 | 61.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.06 | 58.5 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.55 | 56.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.10 | 54.8 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 37.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.02 | 29.3 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 27.1 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 26.7 |
| stream/upload/ms | 0.004 | 0.000 | 1.18 | 20.5 |
| pipelines/driver_ms | 0.003 | 0.000 | 5.22 | 19.5 |
| controller/write/ms | 0.003 | 0.004 | 0.06 | 18.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.07 | 18.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.7 |
| pipelines/wait_ms | 0.001 | 0.000 | 7.37 | 7.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.0 |
| stage/render/prepare | 0.000 | 0.003 | 0.03 | 2.6 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.17 | 0.4 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1306 | 2262 | 538 |
| crowd/c0/meshes/count | count | 353 | 646 | 129 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6066 | 5658 |
| crowd/c1/meshes/count | count | 1173 | 1324 | 1212 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4266 | 9951 | 6135 |
| crowd/joints_uploaded/count | count | 4250 | 6643 | 6135 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.5 k | 56.7 k | 46.9 k |
| encode//max | ms | 2.27 | 4.45 | 2.02 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.08 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.55 | 0.01 |
| encode/finish_critical | ms | 2.27 | 4.45 | 2.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.44 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.79 | 30.41 | 8.78 |
| gpu/timed/ms | ms | 8.23 | 9.90 | 8.74 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2512 | 2569 | 2569 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 286.5 k | 286.5 k | 286.5 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.6 k | 11.8 k | 10.4 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 374.99 M | 374.99 M | 374.99 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 387 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 334 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 387 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 334 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 762 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2520 | 3562 | 2455 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 762 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2520 | 3562 | 2455 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 47 | 186 | 40 |
| phases/shadow/bins | count | 1847 | 2383 | 966 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5431 | 6790 | 3863 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 162 | 223 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 180 | 182 | 182 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 17 | 514 | 13 |
| preprocess/bind_groups_created | count | 10 | 507 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| preprocess/bind_groups_reused | count | 174 | 265 | 169 |
| process/cpu_usage | % | 25 | 36.54 | 28.37 |
| process/mem_usage | GiB | 5.11 | 5.35 | 5.35 |
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
| started_load_count |  | 6297 | 6421 | 6421 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.07 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.65 | 50.47 | 30.80 |
| system/mem_usage | % | 60.44 | 61.09 | 61.09 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2817 | 2932 | 2932 |
| textures/images | count | 2578 | 2636 | 2636 |
| views/all/batch_sets | count | 103 | 241 | 96 |
| views/all/bins | count | 3733 | 4481 | 3248 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.8 k | 33.8 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 11.2 k | 13.9 k | 9805 |
| views/all/pipelines | count | 207 | 270 | 74 |
| views/all/pipelines_distinct | count | 68 | 73 | 59 |
| views/all/work_items | count | 256.4 k | 258.8 k | 255.9 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1423 | 1781 | 966 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4733 | 5855 | 3863 |
| views/cascade/pipelines | count | 37 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.3 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 141 | 0 |
| views/lamp_face/bins | count | 413 | 719 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 651 | 1063 | 0 |
| views/lamp_face/pipelines | count | 125 | 180 | 0 |
| views/lamp_face/work_items | count | 0 | 715 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2026 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5962 | 7544 | 5942 |
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
| world/meshes | count | 20.5 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 52.65 | 235 | 294.5 k |
| anim/events/count | count | 0.09 | 3 | 512 |
| assets/Image/added | count | 0.11 | 43 | 588 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.52 | 135 | 2888 |
| assets/Mesh/removed | count | 0.88 | 1238 | 4896 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 473 |
| assets/StandardMaterial/modified | count | 3.87 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.27 | 1330 | 1489 |
| encode//count | count | 39.34 | 273 | 220.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5593 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5593 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5593 |
| physics/inserts | count | 0.40 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 9 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 6053 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1327 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2575.2 | main/busy/ms 2550, stage/main/post_update 2448, stage/main/update 95.29, surface/advance_cpu_ms 67.00, stage/main/pre_update 2.82, stage/main/last 1.04, surface/cell_upload_ms 0.81, stage/main/fixed_main_loop 0.58, stream/drive/ms 0.50, stage/main/first 0.48, weather/map/ms 0.15, stream/tick/ms 0.15 |
| warm-up | 3.48 | 908.6 | main/busy/ms 2587, stage/main/post_update 2477, render/thread/ms 869, render/busy/ms 702, stage/render/prepare_assets 435, stage/render/prepare_resources 237, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 98.97, surface/advance_cpu_ms 67.00, stage/render/create_views 19.47, stage/render/render 7.99 |
| warm-up | 3.73 | 78.4 | pipelines/compile_ms 203, pipelines/front_ms 183, main/busy/ms 60.21, render/thread/ms 52.91, render/busy/ms 51.09, stage/main/update 50.83, stage/render/prepare_assets 39.83, pipelines/wait_ms 34.99, pipelines/driver_ms 20.31, stage/render/render 7.31, stage/main/post_update 5.63, render/graph/ms 5.02 |
| warm-up | 4.09 | 45.6 | main/busy/ms 44.98, stage/main/update 36.86, render/thread/ms 14.49, render/busy/ms 13.16, stage/main/post_update 5.55, stage/render/prepare_assets 5.46, stage/render/render 4.89, render/graph/ms 3.19, stage/render/graph_submit 1.69, encode//sum 1.61, stage/render/camera_driver 1.36, stream/drive/ms 1.28 |
| warm-up | 4.76 | 91.3 | pipelines/wait_ms 10.2 k, pipelines/compile_ms 107, pipelines/front_ms 100, main/busy/ms 81.38, stage/main/update 73.29, render/thread/ms 22.03, render/busy/ms 20.59, stage/render/prepare_assets 11.88, pipelines/driver_ms 6.54, stage/render/render 5.97, stage/main/post_update 5.59, render/graph/ms 4.11 |
| warm-up | 4.92 | 35.1 | main/busy/ms 40.93, render/thread/ms 37.21, render/busy/ms 32.26, stage/main/update 28.13, stage/render/prepare_assets 17.82, stream/drive/ms 14.94, stage/render/render 10.44, stage/main/post_update 7.82, render/graph/ms 4.78, render/present/ms 4.72, encode//sum 3.73, stage/render/graph_submit 2.55 |
| warm-up | 4.95 | 29.2 | pipelines/wait_ms 6428, pipelines/compile_ms 233, pipelines/front_ms 214, render/thread/ms 36.37, render/busy/ms 31.61, main/busy/ms 23.11, stage/render/prepare_assets 19.05, pipelines/driver_ms 18.93, stage/main/update 11.33, stage/render/render 9.88, stage/main/post_update 6.56, render/graph/ms 4.63 |
| warm-up | 5.30 | 39.9 | pipelines/wait_ms 8416, render/thread/ms 39.83, render/busy/ms 37.88, main/busy/ms 35.74, pipelines/compile_ms 35.04, pipelines/front_ms 28.48, stage/main/update 24.86, stage/render/prepare_assets 14.99, stage/render/render 11.81, render/graph/ms 8.94, encode//sum 8.00, stage/main/post_update 6.62 |
| warm-up | 5.38 | 29.9 | encode//sum 86.65, render/thread/ms 48.50, render/busy/ms 46.66, stage/render/render 22.59, render/graph/ms 20.09, main/busy/ms 16.23, stage/render/graph_submit 14.69, encode/finish_wall 9.47, stage/render/create_views 8.33, stage/main/update 7.62, stage/render/prepare_assets 6.84, encode/submit 5.21 |
| warm-up | 5.50 | 70.7 | pipelines/wait_ms 16.4 k, pipelines/compile_ms 240, pipelines/front_ms 223, main/busy/ms 74.23, stage/main/update 56.64, stream/drive/ms 51.85, render/thread/ms 23.13, render/busy/ms 21.77, pipelines/driver_ms 17.07, stage/main/post_update 10.33, stage/render/prepare_assets 9.42, stage/render/render 6.43 |
| warm-up | 5.81 | 56.3 | pipelines/wait_ms 12.2 k, main/busy/ms 65.02, stage/main/update 55.75, pipelines/compile_ms 29.02, render/thread/ms 28.85, stream/drive/ms 28.02, render/busy/ms 27.53, pipelines/front_ms 26.59, stage/render/prepare_assets 10.74, stage/render/render 8.65, encode//sum 6.87, render/graph/ms 6.66 |
| warm-up | 6.22 | 47.3 | pipelines/wait_ms 9763, pipelines/compile_ms 52.38, pipelines/front_ms 48.61, main/busy/ms 42.67, render/thread/ms 33.03, render/busy/ms 31.71, stage/main/update 30.84, stage/render/prepare_assets 13.73, stream/drive/ms 13.35, encode//sum 9.99, stage/render/render 8.83, stage/main/post_update 8.69 |
| warm-up | 6.25 | 30.9 | pipelines/wait_ms 14.3 k, pipelines/compile_ms 81.42, pipelines/front_ms 76.11, render/thread/ms 45.00, render/busy/ms 43.40, main/busy/ms 42.42, stage/main/update 30.34, stage/render/prepare_assets 22.97, encode//sum 10.40, stage/render/render 8.81, stage/main/post_update 7.16, render/graph/ms 6.75 |
| warm-up | 6.28 | 30.2 | pipelines/wait_ms 17.1 k, pipelines/compile_ms 72.07, pipelines/front_ms 66.70, main/busy/ms 46.58, render/thread/ms 42.84, render/busy/ms 41.03, stage/main/update 27.81, stage/render/prepare_assets 16.46, encode//sum 13.12, stage/main/post_update 12.93, stage/render/render 10.27, render/graph/ms 7.78 |
| warm-up | 6.34 | 32.1 | pipelines/wait_ms 5900, pipelines/compile_ms 48.74, render/thread/ms 46.88, pipelines/front_ms 46.52, render/busy/ms 45.66, main/busy/ms 37.54, stage/main/update 22.35, encode//sum 21.07, stage/render/prepare_assets 16.30, stage/render/render 14.41, render/graph/ms 12.59, stage/main/post_update 10.28 |
| warm-up | 6.64 | 44.2 | pipelines/wait_ms 1537, pipelines/compile_ms 151, pipelines/front_ms 147, render/thread/ms 45.40, render/busy/ms 43.80, main/busy/ms 41.45, stage/main/update 25.84, stage/render/prepare_assets 17.01, encode//sum 16.04, stage/render/render 11.55, stage/main/post_update 9.44, render/graph/ms 9.23 |
| warm-up | 6.67 | 28.7 | render/thread/ms 44.17, render/busy/ms 42.99, main/busy/ms 27.49, stage/render/prepare_assets 24.10, encode//sum 12.58, stage/main/update 12.08, stage/main/post_update 9.59, stage/render/render 8.46, render/graph/ms 6.90, stage/render/graph_submit 5.13, encode/finish_wall 3.16, stage/render/prepare_meshes 2.43 |
| warm-up | 7.46 | 30.0 | render/thread/ms 41.16, render/busy/ms 39.81, main/busy/ms 29.61, encode//sum 19.96, stage/render/prepare_assets 14.55, stage/render/render 14.18, stage/main/update 12.78, render/graph/ms 12.23, stage/main/post_update 11.95, stage/render/graph_submit 9.05, encode/finish_wall 5.11, encode/submit 3.94 |
| warm-up | 7.55 | 43.5 | main/busy/ms 39.66, render/thread/ms 37.19, render/busy/ms 35.69, stage/main/update 25.36, encode//sum 18.28, stage/render/render 15.71, render/graph/ms 13.73, stage/render/prepare_assets 11.78, stage/render/graph_submit 11.39, stage/main/post_update 10.71, encode/finish_wall 6.53, encode/submit 4.76 |
| warm-up | 8.14 | 34.7 | main/busy/ms 52.51, render/thread/ms 43.60, render/busy/ms 42.12, stage/main/update 39.04, encode//sum 23.91, stage/render/render 15.68, render/graph/ms 13.56, stage/render/prepare_assets 12.53, stage/render/graph_submit 10.25, stage/main/post_update 8.97, encode/finish_wall 5.53, encode/submit 4.79 |
| warm-up | 8.41 | 30.7 | render/thread/ms 33.03, render/busy/ms 31.74, main/busy/ms 28.85, encode//sum 24.27, stage/main/update 16.99, stage/render/render 15.92, render/graph/ms 14.11, stage/render/graph_submit 11.38, stage/main/post_update 8.72, stage/render/prepare_assets 7.60, encode/finish_wall 6.82, encode/submit 4.61 |
| measure | 18.19 | 34.0 | main/busy/ms 40.15, stage/main/update 26.57, render/thread/ms 23.31, render/busy/ms 22.24, encode//sum 21.03, stage/render/render 13.18, render/graph/ms 11.62, stage/main/post_update 10.73, stage/render/graph_submit 9.31, encode/finish_wall 4.73, encode/submit 4.60, stage/render/prepare_assets 3.20 |
| measure | 59.06 | 30.9 | render/thread/ms 43.00, render/busy/ms 41.79, main/busy/ms 27.08, encode//sum 21.10, stage/render/prepare_assets 18.75, stage/main/post_update 13.99, stage/render/render 12.89, render/graph/ms 11.08, stage/render/graph_submit 8.09, stage/main/update 7.11, encode/finish_wall 5.27, stage/render/prepare_write_phase_buffers 3.28 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on15\balmora_walk@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **82%** (min 64%) · 2580 MHz (min 2580) · memory 8001 MHz · 79.6 W · 73 °C (max 79) · VRAM 6344 MiB · throttled 100% of samples
