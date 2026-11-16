# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1494 frames over 15.0 s · **median 9.97 ms (100 fps)** · mean 10.04 · p95 11.47 · p99 12.31 (1% low 81 fps) · worst 15.86 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 309 frames over 8.7 s, worst 2539.6 ms
- hitches (> 25.0 ms): **0** measured, 40 in warm-up
- timed GPU passes: 8.70 ms of a 10.04 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.97 ms, limit 11 ms
- pass worst frame 15.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.446 | 2.473 | 2.576 | 0.99 | 0.068 |
| dlss_super_resolution | 2.033 | 2.055 | 2.074 | 0.99 | 0.181 |
| early prepass | 0.874 | 0.883 | 1.065 | 0.99 | 0.091 |
| atmosphere_luts | 0.390 | 0.394 | 0.709 | 0.99 | 0.003 |
| bloom | 0.315 | 0.318 | 0.325 | 0.99 | 0.064 |
| main_transmissive_pass_3d | 0.265 | 0.267 | 0.294 | 0.99 | 0.008 |
| water_velocity | 0.240 | 0.243 | 0.268 | 0.99 | 0.002 |
| water_sspr | 0.207 | 0.209 | 0.216 | 0.99 | 0.002 |
| ssao | 0.197 | 0.199 | 0.210 | 0.99 | 0.007 |
| per_view_shadow_pass_cascade1 | 0.174 | 0.711 | 0.977 | 0.24 | 0.048 |
| atmos_froxel_inject | 0.150 | 0.152 | 0.157 | 0.99 | 0.001 |
| clouds_trace | 0.150 | 0.151 | 0.173 | 0.99 | 0.001 |
| water_fft | 0.134 | 0.135 | 0.138 | 0.99 | 0.003 |
| uniform_allocation | 0.118 | 0.120 | 0.136 | 0.99 | 0.021 |
| clouds_composite | 0.113 | 0.114 | 0.116 | 0.99 | 0.002 |
| upscaling | 0.113 | 0.114 | 0.119 | 0.99 | 0.004 |
| bin_unpacking | 0.096 | 0.097 | 0.116 | 0.99 | 0.015 |
| gpu_instance_materialization | 0.087 | 0.088 | 0.092 | 0.99 | 0.004 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.003 |
| clustering | 0.071 | 0.071 | 0.078 | 0.99 | 0.022 |
| early_mesh_preprocessing | 0.069 | 0.070 | 0.085 | 0.99 | 0.004 |
| render_sky | 0.053 | 0.053 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.425 | 0.417 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.222 | 0.232 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.023 | 0.99 | 0.002 |
| range_unpacking | 0.022 | 0.023 | 0.034 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.025 | 0.99 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.017 | 0.99 | 0.041 |
| night_sky | 0.004 | 0.004 | 0.004 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.016 | 0.016 | 0.00 | 0.006 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.004 |
| per_view_shadow_pass_cascade3 | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.155 | 10.558 | 14.89 | 13677.4 |
| render/busy/ms | 7.491 | 8.722 | 12.78 | 11192.0 |
| main/busy/ms | 7.174 | 8.373 | 11.14 | 10717.7 |
| encode//sum | 6.975 | 9.524 | 13.31 | 10420.1 |
| stage/render/render | 5.132 | 6.249 | 9.35 | 7667.5 |
| render/graph/ms | 4.427 | 5.442 | 7.91 | 6614.2 |
| stage/main/post_update | 4.126 | 5.232 | 6.58 | 6163.9 |
| stage/render/graph_submit | 3.448 | 4.433 | 6.57 | 5151.7 |
| encode/submit | 1.901 | 2.508 | 4.72 | 2839.8 |
| stage/main/update | 1.826 | 2.221 | 3.54 | 2728.8 |
| encode/finish_wall | 1.577 | 2.021 | 2.70 | 2356.5 |
| stage/render/prepare_views | 1.222 | 2.760 | 3.87 | 1826.2 |
| render/acquire/ms | 1.179 | 2.707 | 3.82 | 1760.9 |
| stage/render/camera_driver | 0.917 | 1.086 | 1.51 | 1369.5 |
| stage/render/prepare_write_phase_buffers | 0.553 | 0.740 | 1.38 | 826.8 |
| render/present/ms | 0.485 | 0.587 | 2.25 | 724.6 |
| stage/render/prepare_assets | 0.438 | 0.562 | 1.55 | 654.7 |
| stage/render/prepare_resources | 0.393 | 0.464 | 0.71 | 586.7 |
| bigspace/total/ms | 0.385 | 0.846 | 2.56 | 574.5 |
| stage/main/pre_update | 0.316 | 0.409 | 0.67 | 472.1 |
| stage/main/first | 0.293 | 0.358 | 1.38 | 438.0 |
| stage/render/prepare_flush | 0.292 | 0.486 | 0.83 | 436.2 |
| stage/render/prepare_bind_groups | 0.263 | 0.397 | 0.73 | 393.3 |
| stage/main/last | 0.250 | 0.321 | 0.73 | 373.9 |
| terrain/update/ms | 0.234 | 0.302 | 0.52 | 349.0 |
| terrain/select/ms | 0.203 | 0.261 | 0.49 | 303.1 |
| stage/render/prepare_batch_phases | 0.202 | 0.287 | 0.46 | 301.4 |
| stage/render/prepare_meshes | 0.188 | 0.237 | 0.51 | 281.1 |
| stream/drive/ms | 0.185 | 0.266 | 0.43 | 275.9 |
| stage/main/fixed_main_loop | 0.182 | 0.335 | 1.14 | 271.3 |
| stage/render/extract_commands | 0.171 | 0.207 | 0.43 | 255.3 |
| controller/feed/ms | 0.167 | 0.224 | 0.45 | 249.6 |
| far/select/ms | 0.160 | 0.236 | 0.50 | 239.7 |
| stream/tick/ms | 0.158 | 0.233 | 0.40 | 236.6 |
| controller/move/ms | 0.147 | 0.257 | 0.47 | 219.7 |
| bigspace/high_precision/ms | 0.145 | 0.383 | 2.31 | 217.1 |
| stage/render/create_views | 0.143 | 0.183 | 0.41 | 214.2 |
| anim/evaluate/ms | 0.109 | 0.281 | 1.57 | 162.6 |
| bigspace/low_precision/ms | 0.108 | 0.258 | 2.15 | 160.9 |
| stage/render/queue | 0.101 | 0.134 | 0.34 | 150.7 |
| stage/render/specialize | 0.088 | 0.115 | 0.34 | 132.0 |
| anim/evaluate_cpu/ms | 0.060 | 0.079 | 0.38 | 90.0 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.080 | 0.35 | 72.8 |
| encode/resolve_diagnostics/sum | 0.043 | 0.073 | 0.13 | 63.8 |
| physics/step/ms | 0.029 | 0.057 | 0.25 | 43.5 |
| weather/map/ms | 0.025 | 0.033 | 0.22 | 36.7 |
| stage/render/cleanup | 0.024 | 0.032 | 0.43 | 35.2 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.034 | 0.09 | 27.5 |
| encode/camera_driver_span_end/sum | 0.016 | 0.032 | 0.27 | 23.6 |
| stage/render/phase_sort | 0.014 | 0.025 | 0.17 | 21.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 16.7 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 10.1 |
| controller/avoid/ms | 0.006 | 0.008 | 0.05 | 9.4 |
| script/input/ms | 0.005 | 0.006 | 0.22 | 7.5 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.2 |
| anim/apply/ms | 0.004 | 0.005 | 0.13 | 6.4 |
| controller/write/ms | 0.002 | 0.003 | 0.22 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| encode//max | ms | 1.30 | 2.31 | 1.39 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.27 | 0.02 |
| encode/finish_critical | ms | 1.30 | 2.31 | 1.39 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.13 | 0.02 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.91 | 12.78 | 9.76 |
| gpu/timed/ms | ms | 9.91 | 11.37 | 9.76 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1929 | 1929 | 1929 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 214.7 k | 214.7 k | 214.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.8 k | 10.8 k | 10.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 877 | 877 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/bins | count | 877 | 877 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 15 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 5 | 24 | 5 |
| phases/shadow/bins | count | 857 | 870 | 857 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 5861 | 11.7 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 3121 | 3149 | 3121 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 20 | 23 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 294 | 294 | 294 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 5 | 37 | 5 |
| preprocess/bind_groups_created | count | 0 | 31 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 123 | 123 | 123 |
| process/cpu_usage | % | 23.08 | 33.65 | 20.38 |
| process/mem_usage | GiB | 4.50 | 4.60 | 4.60 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5285 | 5287 | 5287 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.00 | 62.69 | 27.32 |
| system/mem_usage | % | 47.91 | 48.09 | 47.94 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2538 | 2538 | 2538 |
| textures/images | count | 1995 | 1995 | 1995 |
| views/all/batch_sets | count | 63 | 82 | 63 |
| views/all/bins | count | 3109 | 3122 | 3109 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 17.6 k | 23.4 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 9381 | 9409 | 9381 |
| views/all/pipelines | count | 66 | 69 | 66 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
| views/all/work_items | count | 153.8 k | 206.1 k | 153.8 k |
| views/cascade/batch_sets | count | 5 | 24 | 5 |
| views/cascade/bins | count | 857 | 870 | 857 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 5861 | 11.7 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 3121 | 3149 | 3121 |
| views/cascade/pipelines | count | 20 | 23 | 20 |
| views/cascade/work_items | count | 49.2 k | 101.4 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 2252 | 2252 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 46 | 46 | 46 |
| views/main/work_items | count | 104.7 k | 104.7 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.1 k | 15.1 k | 15.1 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 152 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.31 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 32.50 | 35 | 48.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1494 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1494 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1494 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2539.6 | main/busy/ms 2513, stage/main/post_update 2407, stage/main/update 99.01, surface/advance_cpu_ms 67.93, stage/main/pre_update 2.33, stage/main/last 2.09, surface/cell_upload_ms 0.70, stage/main/fixed_main_loop 0.46, stage/main/first 0.45, stream/drive/ms 0.40, weather/map/ms 0.12, bigspace/total/ms 0.10 |
| warm-up | 3.29 | 746.5 | main/busy/ms 2652, stage/main/post_update 2432, render/thread/ms 716, render/busy/ms 553, stage/render/prepare_assets 517, stage/main/update 207, stage/render/prepare_views 162, render/acquire/ms 161, surface/advance_cpu_ms 67.93, stage/render/prepare_resources 22.04, stage/render/render 10.45, encode//sum 10.35 |
| warm-up | 3.38 | 94.3 | render/thread/ms 793, render/busy/ms 628, stage/render/prepare_assets 569, main/busy/ms 171, stage/render/prepare_views 162, render/acquire/ms 161, stage/main/update 116, stage/main/post_update 40.74, pipelines/compile_ms 32.26, pipelines/front_ms 30.55, stage/render/prepare_resources 27.18, stage/render/render 25.39 |
| warm-up | 3.41 | 26.5 | render/thread/ms 94.08, pipelines/compile_ms 92.18, render/busy/ms 89.08, pipelines/front_ms 86.30, main/busy/ms 55.71, stage/render/prepare_assets 52.89, stage/main/post_update 31.76, terrain/update/ms 26.06, stage/render/render 25.34, render/graph/ms 17.59, pipelines/wait_ms 12.30, stage/main/update 10.97 |
| warm-up | 3.55 | 145.8 | pipelines/wait_ms 4325, pipelines/compile_ms 1391, pipelines/front_ms 1290, render/thread/ms 153, render/busy/ms 148, main/busy/ms 146, stage/main/post_update 110, pipelines/driver_ms 100, bigspace/total/ms 46.65, stage/render/prepare_write_phase_buffers 43.59, stage/render/prepare_flush 32.90, stage/render/prepare_bind_groups 26.83 |
| warm-up | 3.63 | 78.4 | pipelines/wait_ms 5789, pipelines/compile_ms 2174, pipelines/front_ms 2014, render/thread/ms 211, render/busy/ms 208, main/busy/ms 204, pipelines/driver_ms 161, stage/main/post_update 106, stage/main/update 80.57, bigspace/total/ms 45.59, stage/render/prepare_write_phase_buffers 44.25, stage/render/prepare_resources 40.82 |
| warm-up | 3.72 | 91.5 | pipelines/compile_ms 1806, pipelines/front_ms 1692, pipelines/wait_ms 1540, render/thread/ms 155, render/busy/ms 153, pipelines/driver_ms 114, main/busy/ms 105, stage/render/prepare_assets 75.51, stage/main/update 70.53, stage/render/render 33.03, render/graph/ms 30.55, stage/render/graph_submit 27.00 |
| warm-up | 3.76 | 32.6 | pipelines/compile_ms 1428, pipelines/front_ms 1359, pipelines/wait_ms 223, render/thread/ms 104, render/busy/ms 103, pipelines/driver_ms 69.59, stage/render/prepare_assets 64.19, main/busy/ms 44.37, stage/render/render 31.79, render/graph/ms 29.77, stage/render/graph_submit 26.38, stage/main/post_update 16.94 |
| warm-up | 3.82 | 67.7 | pipelines/compile_ms 1585, pipelines/front_ms 1442, pipelines/wait_ms 242, pipelines/driver_ms 143, encode//sum 125, render/thread/ms 88.71, render/busy/ms 71.48, stage/render/render 46.53, main/busy/ms 34.04, stage/render/prepare_assets 33.50, render/graph/ms 28.90, stage/render/graph_submit 26.28 |
| warm-up | 3.88 | 42.3 | pipelines/compile_ms 779, pipelines/front_ms 735, main/busy/ms 79.00, render/thread/ms 46.06, pipelines/driver_ms 44.26, stage/main/update 43.95, render/busy/ms 43.37, stage/main/post_update 24.73, stage/render/prepare_assets 19.81, stage/render/render 14.98, render/graph/ms 11.61, encode//sum 8.53 |
| warm-up | 3.98 | 93.9 | pipelines/compile_ms 1174, pipelines/front_ms 1118, main/busy/ms 103, render/thread/ms 86.78, render/busy/ms 83.51, stage/main/update 79.33, stage/render/prepare_assets 57.23, pipelines/driver_ms 56.13, pipelines/wait_ms 41.42, stage/render/render 18.07, render/graph/ms 14.21, stage/main/post_update 12.97 |
| warm-up | 4.10 | 119.3 | pipelines/compile_ms 1230, pipelines/front_ms 1122, render/thread/ms 133, pipelines/driver_ms 108, render/busy/ms 102, stage/render/prepare_assets 82.11, main/busy/ms 61.93, pipelines/wait_ms 46.05, stage/main/update 45.54, stage/render/render 41.98, render/present/ms 30.76, render/graph/ms 10.59 |
| warm-up | 4.15 | 50.5 | pipelines/compile_ms 1497, pipelines/front_ms 1382, pipelines/driver_ms 115, render/thread/ms 106, render/busy/ms 75.91, stage/render/prepare_assets 61.61, stage/render/render 39.55, render/present/ms 30.03, main/busy/ms 21.40, encode//sum 13.99, pipelines/wait_ms 12.83, pipelines/lock_ms 11.38 |
| warm-up | 4.23 | 36.3 | pipelines/compile_ms 146, pipelines/front_ms 140, render/thread/ms 54.33, render/busy/ms 49.20, main/busy/ms 46.65, stage/main/update 31.82, encode//sum 25.89, stage/render/render 23.05, stream/drive/ms 22.77, render/graph/ms 20.64, stage/render/graph_submit 17.82, stage/render/prepare_resources 14.40 |
| warm-up | 4.29 | 60.2 | pipelines/compile_ms 146, pipelines/front_ms 140, render/thread/ms 84.98, render/busy/ms 83.35, main/busy/ms 59.91, stage/main/update 35.81, stream/drive/ms 24.55, stage/render/prepare_resources 20.00, stage/main/post_update 18.04, stage/render/prepare_assets 16.58, stage/render/prepare_flush 15.31, stage/render/render 12.76 |
| warm-up | 4.38 | 91.6 | pipelines/compile_ms 1481, pipelines/front_ms 1421, pipelines/wait_ms 382, main/busy/ms 163, render/thread/ms 145, render/busy/ms 143, stage/main/update 141, pipelines/driver_ms 60.36, stage/render/prepare_write_phase_buffers 50.27, stage/render/prepare_resources 39.15, stage/render/prepare_flush 19.49, stage/main/post_update 18.12 |
| warm-up | 4.53 | 155.6 | pipelines/compile_ms 3618, pipelines/front_ms 3508, pipelines/wait_ms 1119, render/thread/ms 171, render/busy/ms 165, main/busy/ms 153, stage/main/update 138, pipelines/driver_ms 109, stage/render/prepare_resources 57.32, stage/render/prepare_assets 42.81, stage/render/prepare_write_phase_buffers 41.01, stage/render/render 18.38 |
| warm-up | 4.57 | 37.8 | pipelines/compile_ms 2288, pipelines/front_ms 2228, pipelines/wait_ms 742, render/thread/ms 115, render/busy/ms 110, pipelines/driver_ms 59.73, main/busy/ms 47.33, stage/render/prepare_assets 43.51, stage/render/prepare_resources 25.25, stage/render/render 24.78, stage/main/update 22.35, render/graph/ms 19.52 |
| warm-up | 4.62 | 48.7 | pipelines/compile_ms 151, pipelines/front_ms 141, render/thread/ms 69.70, render/busy/ms 67.50, main/busy/ms 55.66, stage/main/update 29.69, stage/render/prepare_assets 24.20, stage/render/render 20.50, render/graph/ms 18.32, stage/main/post_update 15.82, stage/render/graph_submit 14.40, pipelines/driver_ms 10.79 |
| warm-up | 4.72 | 94.4 | pipelines/compile_ms 175, pipelines/front_ms 167, pipelines/wait_ms 112, render/thread/ms 105, render/busy/ms 104, stage/render/prepare_assets 66.29, main/busy/ms 59.55, stage/main/post_update 26.00, stage/main/update 25.90, stage/render/render 14.20, render/graph/ms 11.96, encode//sum 9.76 |
| warm-up | 4.77 | 59.6 | pipelines/compile_ms 1472, pipelines/front_ms 1426, pipelines/wait_ms 186, render/thread/ms 116, render/busy/ms 115, main/busy/ms 68.30, stage/render/prepare_assets 62.43, pipelines/driver_ms 45.17, stage/main/update 24.67, stage/main/post_update 24.45, stage/render/render 13.78, stage/render/prepare_write_phase_buffers 12.72 |
| warm-up | 4.82 | 48.5 | pipelines/compile_ms 2113, pipelines/front_ms 2051, pipelines/wait_ms 159, main/busy/ms 97.76, render/thread/ms 87.96, render/busy/ms 86.20, pipelines/driver_ms 62.31, stage/main/update 61.56, stream/drive/ms 54.38, stage/render/prepare_assets 23.52, stage/render/render 19.95, encode//sum 18.94 |
| warm-up | 4.90 | 78.6 | pipelines/compile_ms 1302, pipelines/front_ms 1264, pipelines/wait_ms 165, main/busy/ms 112, render/thread/ms 89.80, render/busy/ms 88.14, stage/main/update 77.64, stream/drive/ms 48.66, pipelines/driver_ms 37.64, stage/render/render 21.99, encode//sum 20.48, render/graph/ms 19.45 |
| warm-up | 4.99 | 92.5 | pipelines/compile_ms 1866, pipelines/front_ms 1816, pipelines/wait_ms 198, render/thread/ms 125, render/busy/ms 123, main/busy/ms 97.82, stage/render/prepare_assets 67.12, stage/main/update 65.81, pipelines/driver_ms 50.19, stage/main/post_update 21.55, stage/render/render 14.25, stage/render/prepare_flush 14.01 |
| warm-up | 5.05 | 52.9 | pipelines/compile_ms 1672, pipelines/front_ms 1626, pipelines/wait_ms 119, render/thread/ms 111, render/busy/ms 109, stage/render/prepare_assets 80.75, main/busy/ms 65.87, pipelines/driver_ms 46.55, stage/main/update 42.71, stage/main/post_update 14.65, encode//sum 11.85, stage/render/render 11.70 |
| warm-up | 5.09 | 38.4 | pipelines/compile_ms 352, pipelines/front_ms 339, render/thread/ms 62.12, render/busy/ms 60.40, main/busy/ms 38.18, stage/render/prepare_assets 35.00, stage/main/update 19.65, stage/render/render 13.19, pipelines/driver_ms 13.05, encode//sum 12.99, render/graph/ms 11.05, stage/main/post_update 9.24 |
| warm-up | 5.11 | 26.0 | pipelines/compile_ms 850, pipelines/front_ms 830, main/busy/ms 54.34, render/thread/ms 51.83, render/busy/ms 49.30, stage/main/update 33.77, stage/render/render 20.61, pipelines/driver_ms 20.51, stage/render/prepare_assets 17.89, render/graph/ms 17.85, encode//sum 17.73, stage/render/graph_submit 14.52 |
| warm-up | 5.15 | 40.1 | pipelines/compile_ms 879, pipelines/front_ms 860, main/busy/ms 49.06, render/thread/ms 46.03, render/busy/ms 43.69, stage/main/update 29.66, pipelines/driver_ms 19.20, stage/render/render 18.15, encode//sum 16.90, render/graph/ms 15.58, stage/render/prepare_assets 15.31, stage/main/post_update 13.94 |
| warm-up | 5.21 | 38.2 | pipelines/compile_ms 434, pipelines/front_ms 419, main/busy/ms 41.63, render/thread/ms 40.59, render/busy/ms 38.32, stage/main/update 30.37, pipelines/driver_ms 15.63, stream/drive/ms 14.21, stage/render/render 14.02, encode//sum 13.62, render/graph/ms 11.42, stage/render/prepare_assets 10.55 |
| warm-up | 5.38 | 28.4 | pipelines/compile_ms 483, pipelines/front_ms 465, render/thread/ms 44.91, render/busy/ms 43.35, main/busy/ms 28.63, pipelines/wait_ms 20.74, pipelines/driver_ms 18.05, encode//sum 14.43, stage/render/prepare_assets 12.88, stage/render/render 12.84, stage/main/update 12.11, stage/main/post_update 11.70 |
| warm-up | 5.45 | 37.5 | pipelines/compile_ms 413, pipelines/front_ms 402, render/thread/ms 56.19, render/busy/ms 54.34, main/busy/ms 42.88, stage/main/post_update 20.14, stage/main/update 17.20, stage/render/render 16.56, encode//sum 16.24, stage/render/prepare_resources 15.21, render/graph/ms 14.01, pipelines/driver_ms 10.58 |
| warm-up | 5.48 | 35.7 | pipelines/compile_ms 190, pipelines/front_ms 185, render/thread/ms 61.50, render/busy/ms 59.52, main/busy/ms 48.09, stage/main/update 21.49, stage/main/post_update 19.00, stage/render/render 17.22, encode//sum 15.08, render/graph/ms 14.75, stage/render/prepare_resources 14.66, stage/render/prepare_assets 10.91 |
| warm-up | 5.52 | 37.2 | pipelines/compile_ms 1189, pipelines/front_ms 1162, render/thread/ms 47.47, render/busy/ms 44.93, main/busy/ms 34.03, pipelines/driver_ms 26.59, stage/render/prepare_assets 19.35, stage/main/update 18.76, encode//sum 12.92, stage/render/render 12.52, render/graph/ms 9.63, stage/main/post_update 8.15 |
| warm-up | 5.71 | 30.8 | pipelines/compile_ms 142, pipelines/front_ms 137, render/thread/ms 48.26, render/busy/ms 46.25, main/busy/ms 39.32, stage/main/update 19.52, encode//sum 19.02, stage/render/render 17.86, stage/render/prepare_assets 16.95, render/graph/ms 15.17, stage/main/post_update 13.57, stage/render/graph_submit 11.12 |
| warm-up | 5.76 | 50.1 | pipelines/compile_ms 164, pipelines/front_ms 159, render/thread/ms 57.53, render/busy/ms 56.06, main/busy/ms 50.39, stage/render/prepare_assets 31.02, stage/main/update 28.96, encode//sum 15.67, stage/main/post_update 13.49, stage/render/render 12.07, render/graph/ms 10.21, stage/render/graph_submit 7.05 |
| warm-up | 5.81 | 54.1 | pipelines/compile_ms 164, pipelines/front_ms 159, render/thread/ms 75.62, render/busy/ms 56.19, stage/render/prepare_assets 36.26, main/busy/ms 34.23, stage/render/render 26.65, stage/main/update 20.32, render/present/ms 19.14, encode//sum 14.16, stage/main/post_update 8.51, render/graph/ms 6.88 |
| warm-up | 5.95 | 26.7 | main/busy/ms 40.26, render/thread/ms 36.92, render/busy/ms 35.65, stage/main/update 28.62, encode//sum 20.23, stream/drive/ms 16.44, stage/render/render 15.83, render/graph/ms 13.79, pipelines/compile_ms 13.71, pipelines/front_ms 12.68, stage/render/graph_submit 9.66, stage/render/prepare_assets 7.74 |
| warm-up | 6.47 | 26.3 | render/thread/ms 40.72, render/busy/ms 39.19, main/busy/ms 32.66, encode//sum 18.80, stage/main/update 14.98, stage/render/render 14.25, stage/main/post_update 12.38, render/graph/ms 12.21, stage/render/prepare_assets 11.64, stage/render/graph_submit 9.07, encode/submit 5.08, encode/finish_wall 4.06 |
| warm-up | 6.50 | 30.4 | render/thread/ms 39.00, render/busy/ms 37.96, main/busy/ms 27.34, stage/render/prepare_assets 18.90, encode//sum 12.85, stage/main/update 10.69, stage/main/post_update 10.23, stage/render/render 8.08, render/graph/ms 6.62, stage/render/graph_submit 4.58, stage/render/prepare_meshes 3.34, encode/finish_wall 2.81 |
| warm-up | 6.54 | 33.5 | render/thread/ms 41.07, render/busy/ms 39.99, stage/render/render 29.22, render/graph/ms 27.73, stage/render/camera_driver 21.73, main/busy/ms 19.39, encode//sum 13.84, stage/main/post_update 8.55, stage/main/update 7.55, stage/render/graph_submit 5.86, stage/render/prepare_assets 4.80, encode/finish_wall 2.94 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\f_scene\balmora@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 89%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.4 W · 73 °C (max 76) · VRAM 4928 MiB · throttled 100% of samples
