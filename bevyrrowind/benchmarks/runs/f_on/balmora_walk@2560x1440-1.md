# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5420 frames over 65.6 s · **median 11.87 ms (84 fps)** · mean 12.11 · p95 14.44 · p99 16.72 (1% low 60 fps) · worst 37.91 ms · 33 spikes over 1.5× median · 24.7 stutters/min (work 25.6/min)
- warm-up: 323 frames over 9.6 s, worst 2673.5 ms
- hitches (> 29.7 ms): **4** measured, 42 in warm-up
- timed GPU passes: 7.41 ms of a 12.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.87 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- **FAIL** worst frame 37.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.072 | 2.080 | 2.232 | 1.00 | 0.175 |
| main_opaque_pass_3d | 1.635 | 1.641 | 2.039 | 1.00 | 0.072 |
| early prepass | 0.539 | 0.541 | 0.775 | 1.00 | 0.082 |
| bloom | 0.320 | 0.321 | 0.327 | 1.00 | 0.064 |
| atmosphere_luts | 0.316 | 0.317 | 0.374 | 1.00 | 0.004 |
| water_velocity | 0.282 | 0.283 | 0.353 | 1.00 | 0.002 |
| water_sspr | 0.206 | 0.206 | 0.206 | 1.00 | 0.002 |
| ssao | 0.192 | 0.193 | 0.195 | 1.00 | 0.008 |
| per_view_shadow_pass_cascade1 | 0.152 | 0.589 | 0.663 | 0.26 | 0.044 |
| atmos_froxel_inject | 0.147 | 0.147 | 0.148 | 1.00 | 0.001 |
| uniform_allocation | 0.144 | 0.145 | 0.318 | 1.00 | 0.028 |
| clouds_trace | 0.141 | 0.142 | 0.233 | 1.00 | 0.001 |
| auxiliary_views | 0.138 | 0.138 | 0.526 | 1.00 | 0.135 |
| water_fft | 0.132 | 0.133 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.121 | 0.122 | 0.129 | 1.00 | 0.005 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 1.00 | 0.002 |
| bin_unpacking | 0.102 | 0.102 | 0.124 | 1.00 | 0.020 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.084 | 0.084 | 0.162 | 1.00 | 0.009 |
| clustering | 0.066 | 0.066 | 0.069 | 1.00 | 0.019 |
| render_sky | 0.052 | 0.052 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.420 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.045 | 0.058 | 0.058 | 0.78 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.043 | 1.00 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.228 | 0.237 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.027 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.025 | 1.00 | 0.114 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 1.00 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.013 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.043 | 0.052 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.052 | 0.078 | 0.00 | 0.016 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.238 | 16.038 | 77.11 | 60907.9 |
| render/thread/ms | 10.765 | 12.958 | 31.78 | 58346.0 |
| render/busy/ms | 10.201 | 12.371 | 31.31 | 55288.5 |
| main/busy/ms | 9.533 | 11.873 | 31.22 | 51668.4 |
| stage/render/render | 6.820 | 8.319 | 27.99 | 36966.1 |
| render/graph/ms | 6.066 | 7.485 | 27.21 | 32875.8 |
| stage/main/post_update | 6.007 | 7.945 | 27.76 | 32556.2 |
| stage/render/graph_submit | 4.873 | 5.994 | 26.05 | 26413.2 |
| encode/finish_wall | 2.584 | 3.376 | 23.67 | 14003.4 |
| encode/submit | 2.337 | 3.100 | 6.64 | 12665.1 |
| stage/main/update | 2.233 | 2.728 | 22.82 | 12105.2 |
| stage/render/camera_driver | 1.136 | 1.562 | 3.91 | 6154.6 |
| stage/render/prepare_write_phase_buffers | 0.795 | 1.051 | 2.57 | 4310.2 |
| bigspace/total/ms | 0.616 | 1.967 | 3.59 | 3340.0 |
| stage/render/prepare_assets | 0.611 | 0.786 | 10.42 | 3309.2 |
| render/present/ms | 0.473 | 0.580 | 2.11 | 2563.0 |
| stage/render/prepare_resources | 0.469 | 0.570 | 1.19 | 2541.6 |
| stage/render/prepare_meshes | 0.427 | 0.555 | 1.68 | 2315.1 |
| stage/main/first | 0.407 | 0.528 | 6.97 | 2203.6 |
| stage/main/pre_update | 0.338 | 0.479 | 3.13 | 1832.5 |
| stage/render/specialize | 0.308 | 0.446 | 2.59 | 1667.9 |
| terrain/update/ms | 0.307 | 0.401 | 2.06 | 1663.8 |
| controller/feed/ms | 0.294 | 0.380 | 0.73 | 1592.8 |
| terrain/select/ms | 0.273 | 0.357 | 0.81 | 1478.0 |
| stage/render/prepare_bind_groups | 0.272 | 0.464 | 2.61 | 1475.3 |
| bigspace/low_precision/ms | 0.263 | 0.854 | 3.12 | 1425.5 |
| stream/drive/ms | 0.252 | 0.338 | 20.70 | 1366.6 |
| stage/main/fixed_main_loop | 0.244 | 0.387 | 0.95 | 1323.0 |
| stage/render/queue | 0.235 | 0.390 | 3.11 | 1271.3 |
| bigspace/high_precision/ms | 0.197 | 0.757 | 3.25 | 1068.8 |
| anim/evaluate/ms | 0.197 | 0.300 | 2.95 | 1068.7 |
| stream/tick/ms | 0.193 | 0.263 | 0.51 | 1043.5 |
| stage/render/create_views | 0.192 | 0.251 | 2.46 | 1040.2 |
| far/select/ms | 0.192 | 0.266 | 0.57 | 1038.9 |
| stage/render/extract_commands | 0.187 | 0.221 | 0.50 | 1011.7 |
| stage/render/prepare_flush | 0.184 | 0.385 | 0.85 | 995.6 |
| stage/render/prepare_batch_phases | 0.163 | 0.229 | 0.64 | 882.4 |
| anim/evaluate_cpu/ms | 0.155 | 0.201 | 0.71 | 842.4 |
| stage/render/prepare_views | 0.134 | 0.170 | 0.45 | 726.2 |
| controller/move/ms | 0.125 | 0.227 | 0.74 | 679.8 |
| stage/main/last | 0.124 | 0.199 | 1.72 | 669.9 |
| render/acquire/ms | 0.091 | 0.118 | 0.35 | 494.4 |
| stage/render/phase_sort | 0.077 | 0.104 | 1.31 | 416.8 |
| physics/step/ms | 0.054 | 0.092 | 0.49 | 295.3 |
| pipelines/compile_ms | 0.054 | 0.000 | 120.23 | 294.7 |
| pipelines/front_ms | 0.052 | 0.000 | 115.88 | 281.5 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.076 | 0.31 | 263.8 |
| physics/insert/ms | 0.036 | 0.389 | 1.07 | 194.4 |
| encode/resolve_diagnostics/sum | 0.034 | 0.047 | 0.59 | 182.1 |
| stage/render/cleanup | 0.033 | 0.107 | 0.75 | 180.7 |
| weather/map/ms | 0.027 | 0.035 | 0.23 | 144.6 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.47 | 88.7 |
| anim/apply/ms | 0.013 | 0.024 | 0.14 | 72.2 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.28 | 66.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.22 | 58.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.08 | 57.8 |
| script/update/ms | 0.007 | 0.009 | 0.22 | 40.0 |
| stream/despawn/ms | 0.005 | 0.000 | 0.97 | 29.1 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 28.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.05 | 26.0 |
| script/ui/ms | 0.005 | 0.006 | 0.19 | 25.2 |
| controller/write/ms | 0.004 | 0.005 | 0.05 | 19.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 14.3 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.35 | 13.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.7 |
| stage/render/prepare | 0.001 | 0.002 | 0.06 | 2.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 355 | 646 | 144 |
| crowd/c1/count | count | 145 | 155 | 149 |
| crowd/c1/joints/count | count | 5539 | 5937 | 5661 |
| crowd/c1/meshes/count | count | 1172 | 1281 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4601 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.14 | 23.19 | 2.03 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.47 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 2.14 | 23.19 | 2.03 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.21 | 31.31 | 9.06 |
| gpu/timed/ms | ms | 8.59 | 10.64 | 9.06 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 824 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 824 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1294 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3446 | 5107 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1294 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3446 | 5107 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 191 | 19 |
| phases/shadow/bins | count | 4356 | 5501 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.1 k | 12.6 k | 5442 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 186 | 231 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
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
| preprocess/bind_groups | count | 13 | 539 | 37 |
| preprocess/bind_groups_created | count | 13 | 539 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 214 | 123 |
| process/cpu_usage | % | 26.34 | 37.50 | 21.63 |
| process/mem_usage | GiB | 5.39 | 5.56 | 5.56 |
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
| started_load_count |  | 4916 | 5010 | 5010 |
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
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.52 | 57.66 | 31.37 |
| system/mem_usage | % | 68.14 | 68.66 | 68.59 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 66 | 238 | 69 |
| views/all/bins | count | 7041 | 8340 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.3 k | 25.0 k | 13.1 k |
| views/all/pipelines | count | 231 | 278 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 22.2 k | 13.4 k |
| views/cascade/batch_sets | count | 10 | 33 | 19 |
| views/cascade/bins | count | 2229 | 2899 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7658 | 9676 | 5442 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 534 | 9676 | 5442 |
| views/lamp_face/batch_sets | count | 0 | 182 | 0 |
| views/lamp_face/bins | count | 2132 | 2766 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2379 | 3063 | 0 |
| views/lamp_face/pipelines | count | 162 | 203 | 0 |
| views/lamp_face/work_items | count | 0 | 2819 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8538 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8696 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 74.41 | 307 | 403.3 k |
| anim/events/count | count | 0.10 | 3 | 536 |
| assets/Image/added | count | 0.11 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.75 | 152 | 4061 |
| assets/Mesh/removed | count | 0.89 | 1242 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 4.00 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.86 | 1389 | 4676 |
| encode//count | count | 38.80 | 272 | 210.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5420 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5420 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5420 |
| physics/inserts | count | 0.42 | 109 | 2274 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 129 | 2335 |
| physics/steps | count | 0.73 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6246 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1369 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.7 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.67 | 2673.5 | main/busy/ms 2644, stage/main/post_update 2541, stage/main/update 96.14, surface/advance_cpu_ms 66.17, stage/main/pre_update 2.97, stage/main/last 1.15, surface/cell_upload_ms 0.77, stream/drive/ms 0.69, stage/main/first 0.61, stage/main/fixed_main_loop 0.40, stream/tick/ms 0.20, weather/map/ms 0.11 |
| warm-up | 3.43 | 752.7 | main/busy/ms 2783, stage/main/post_update 2565, render/thread/ms 717, render/busy/ms 553, stage/render/prepare_assets 515, stage/main/update 206, stage/render/prepare_views 160, render/acquire/ms 159, surface/advance_cpu_ms 66.17, stage/render/prepare_resources 21.82, stage/render/render 13.66, encode//sum 9.18 |
| warm-up | 3.52 | 90.2 | render/thread/ms 789, render/busy/ms 622, stage/render/prepare_assets 561, main/busy/ms 164, stage/render/prepare_views 161, render/acquire/ms 159, stage/main/update 116, pipelines/compile_ms 44.61, pipelines/front_ms 42.77, stage/main/post_update 37.25, stage/render/render 30.58, stage/render/prepare_resources 25.41 |
| warm-up | 3.56 | 40.7 | pipelines/compile_ms 264, pipelines/front_ms 239, render/thread/ms 110, render/busy/ms 106, stage/render/prepare_assets 47.28, main/busy/ms 42.18, pipelines/driver_ms 24.66, terrain/update/ms 23.80, stage/main/post_update 23.77, stage/render/render 22.96, stage/render/prepare_resources 22.14, pipelines/wait_ms 19.94 |
| warm-up | 3.65 | 94.9 | pipelines/wait_ms 1273, pipelines/compile_ms 1038, pipelines/front_ms 962, render/thread/ms 132, render/busy/ms 128, pipelines/driver_ms 75.83, main/busy/ms 55.93, stage/render/prepare_resources 44.50, stage/render/prepare_assets 27.14, stage/main/update 26.63, stage/render/prepare_write_phase_buffers 22.61, stage/main/post_update 19.87 |
| warm-up | 3.74 | 91.8 | pipelines/wait_ms 5519, pipelines/compile_ms 1511, pipelines/front_ms 1427, render/thread/ms 184, render/busy/ms 181, main/busy/ms 91.85, pipelines/driver_ms 84.04, stage/render/prepare_resources 78.98, stage/render/prepare_assets 39.11, stage/render/prepare_write_phase_buffers 37.92, stage/main/post_update 36.76, stage/main/update 25.61 |
| warm-up | 3.80 | 52.4 | pipelines/wait_ms 4453, pipelines/compile_ms 1504, pipelines/front_ms 1425, render/thread/ms 137, render/busy/ms 135, main/busy/ms 119, pipelines/driver_ms 78.59, stage/render/prepare_resources 69.62, stage/main/update 59.53, stage/main/post_update 33.51, stage/render/prepare_assets 30.47, stage/render/prepare_write_phase_buffers 15.94 |
| warm-up | 3.89 | 90.6 | pipelines/compile_ms 2164, pipelines/front_ms 2065, pipelines/wait_ms 544, render/thread/ms 115, render/busy/ms 112, pipelines/driver_ms 98.48, main/busy/ms 94.45, stage/render/prepare_assets 72.48, stage/main/update 66.82, stage/main/post_update 21.62, stage/render/prepare_resources 17.75, stage/render/render 13.33 |
| warm-up | 3.97 | 34.9 | pipelines/compile_ms 147, pipelines/front_ms 137, render/thread/ms 54.11, render/busy/ms 51.79, main/busy/ms 41.35, stage/main/update 20.01, stage/render/prepare_resources 18.99, stage/main/post_update 17.56, stage/render/prepare_assets 13.40, stage/render/render 12.63, encode//sum 11.19, render/graph/ms 9.82 |
| warm-up | 4.13 | 78.7 | pipelines/compile_ms 1352, pipelines/front_ms 1296, main/busy/ms 66.94, render/thread/ms 61.93, render/busy/ms 59.15, pipelines/driver_ms 55.88, stage/main/update 53.82, stream/drive/ms 48.49, stage/render/prepare_assets 36.40, stage/render/render 16.82, render/graph/ms 14.26, stage/render/graph_submit 9.88 |
| warm-up | 4.17 | 35.4 | pipelines/compile_ms 1467, pipelines/front_ms 1402, render/thread/ms 65.25, pipelines/driver_ms 65.22, render/busy/ms 63.29, stage/render/prepare_assets 42.57, main/busy/ms 38.04, stage/main/update 17.28, stage/render/render 15.73, stage/main/post_update 13.63, render/graph/ms 13.46, pipelines/wait_ms 11.75 |
| warm-up | 4.20 | 31.8 | pipelines/compile_ms 589, pipelines/front_ms 565, render/thread/ms 57.25, render/busy/ms 55.07, main/busy/ms 43.28, stage/render/prepare_assets 29.01, pipelines/driver_ms 23.41, stage/render/render 20.41, render/graph/ms 18.08, stage/main/update 16.60, stage/main/post_update 16.04, stream/drive/ms 10.22 |
| warm-up | 4.23 | 33.6 | pipelines/compile_ms 515, pipelines/front_ms 498, main/busy/ms 68.89, render/thread/ms 54.41, render/busy/ms 52.49, stage/main/update 41.01, stage/render/prepare_assets 30.58, stream/drive/ms 25.85, stage/main/post_update 18.01, pipelines/driver_ms 17.76, stage/render/render 16.00, render/graph/ms 13.61 |
| warm-up | 4.33 | 94.9 | pipelines/compile_ms 938, pipelines/front_ms 911, render/thread/ms 77.87, render/busy/ms 75.83, main/busy/ms 66.52, stage/render/prepare_assets 58.17, stage/main/update 43.02, stream/drive/ms 27.75, pipelines/driver_ms 27.05, stage/main/post_update 16.43, stage/render/render 12.19, render/graph/ms 9.71 |
| warm-up | 4.39 | 64.3 | pipelines/compile_ms 1517, pipelines/front_ms 1476, render/thread/ms 96.05, render/busy/ms 94.23, stage/render/prepare_assets 61.83, main/busy/ms 47.30, pipelines/driver_ms 41.09, stage/main/post_update 24.48, stage/main/update 15.34, stage/render/render 14.72, render/graph/ms 12.56, stage/render/prepare_write_phase_buffers 12.36 |
| warm-up | 4.49 | 95.2 | pipelines/compile_ms 1089, pipelines/front_ms 1057, main/busy/ms 103, render/thread/ms 101, render/busy/ms 99.29, stage/main/update 54.36, stage/render/prepare_assets 43.28, stage/main/post_update 40.59, pipelines/driver_ms 32.28, stage/render/render 16.80, stage/render/prepare_write_phase_buffers 14.95, stage/render/cleanup 14.60 |
| warm-up | 4.58 | 95.9 | pipelines/compile_ms 2094, pipelines/front_ms 2037, render/thread/ms 117, render/busy/ms 115, main/busy/ms 96.59, pipelines/wait_ms 89.66, stage/render/prepare_assets 68.35, stage/main/update 64.18, pipelines/driver_ms 57.11, stage/main/post_update 25.21, stage/render/render 15.96, stage/render/cleanup 14.93 |
| warm-up | 4.65 | 67.4 | pipelines/compile_ms 2615, pipelines/front_ms 2544, pipelines/wait_ms 191, render/thread/ms 126, render/busy/ms 124, pipelines/driver_ms 70.47, stage/render/prepare_assets 67.70, main/busy/ms 65.17, stage/main/update 30.79, stage/main/post_update 24.08, stage/render/prepare_resources 21.80, stage/render/render 14.50 |
| warm-up | 4.78 | 134.4 | pipelines/compile_ms 2680, pipelines/front_ms 2601, pipelines/wait_ms 1122, render/thread/ms 184, render/busy/ms 182, main/busy/ms 80.96, pipelines/driver_ms 79.60, stage/render/prepare_assets 55.06, stage/main/post_update 50.49, stage/render/render 29.61, stage/render/prepare_resources 29.01, render/graph/ms 26.92 |
| warm-up | 4.86 | 78.1 | pipelines/compile_ms 2932, pipelines/front_ms 2813, pipelines/wait_ms 1678, render/thread/ms 191, render/busy/ms 188, pipelines/driver_ms 119, main/busy/ms 91.93, encode//sum 79.66, stage/render/prepare_assets 47.89, stage/main/post_update 42.48, stage/render/render 41.98, render/graph/ms 38.83 |
| warm-up | 4.92 | 53.0 | pipelines/compile_ms 1990, pipelines/front_ms 1891, pipelines/wait_ms 1184, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 99.18, encode//sum 80.87, main/busy/ms 69.83, stage/render/render 32.05, stage/main/update 30.42, stage/render/prepare_assets 29.89, render/graph/ms 28.66 |
| warm-up | 5.01 | 66.6 | pipelines/compile_ms 668, pipelines/front_ms 652, pipelines/wait_ms 263, main/busy/ms 105, stage/main/update 78.49, stream/drive/ms 54.92, render/thread/ms 42.92, render/busy/ms 39.35, stage/render/render 20.41, render/graph/ms 15.95, pipelines/driver_ms 15.34, stage/main/post_update 14.81 |
| warm-up | 5.10 | 86.2 | pipelines/compile_ms 1234, pipelines/front_ms 1198, pipelines/wait_ms 235, main/busy/ms 120, stage/main/update 92.51, render/thread/ms 70.48, render/busy/ms 66.90, stream/drive/ms 48.81, pipelines/driver_ms 36.09, stage/render/prepare_assets 26.34, stage/render/render 26.02, render/graph/ms 21.67 |
| warm-up | 5.24 | 141.7 | pipelines/compile_ms 1885, pipelines/front_ms 1830, pipelines/wait_ms 135, render/thread/ms 134, render/busy/ms 132, main/busy/ms 106, stage/main/update 81.52, stage/render/prepare_assets 67.54, pipelines/driver_ms 54.79, stream/drive/ms 48.27, stage/render/prepare_resources 24.95, encode//sum 21.76 |
| warm-up | 5.35 | 94.9 | pipelines/compile_ms 422, pipelines/front_ms 408, main/busy/ms 74.70, render/thread/ms 56.72, stage/main/update 55.70, render/busy/ms 55.04, stream/drive/ms 37.51, stage/render/prepare_assets 26.79, stage/render/render 18.01, encode//sum 16.60, render/graph/ms 15.84, pipelines/driver_ms 13.82 |
| warm-up | 5.44 | 57.8 | pipelines/compile_ms 559, pipelines/front_ms 536, main/busy/ms 58.85, stage/main/update 44.80, render/thread/ms 43.70, render/busy/ms 40.95, stream/drive/ms 32.67, pipelines/driver_ms 22.84, encode//sum 15.59, stage/render/render 15.31, stage/render/prepare_assets 14.02, render/graph/ms 11.79 |
| warm-up | 5.54 | 37.5 | pipelines/compile_ms 399, pipelines/front_ms 386, render/thread/ms 53.70, render/busy/ms 51.97, main/busy/ms 44.20, stage/main/post_update 22.44, encode//sum 20.55, stage/main/update 16.14, stage/render/render 14.26, pipelines/driver_ms 13.20, render/graph/ms 12.12, stage/render/prepare_assets 8.80 |
| warm-up | 5.58 | 35.7 | pipelines/compile_ms 585, pipelines/front_ms 568, render/thread/ms 66.09, render/busy/ms 63.36, main/busy/ms 50.76, stage/main/post_update 30.98, encode//sum 20.05, pipelines/driver_ms 17.02, stage/main/update 13.88, stage/render/render 13.53, stage/render/prepare_resources 11.42, render/graph/ms 11.41 |
| warm-up | 5.62 | 41.2 | pipelines/compile_ms 340, pipelines/front_ms 331, render/thread/ms 61.09, render/busy/ms 58.21, main/busy/ms 54.11, stage/main/post_update 24.78, stage/main/update 22.75, stage/render/render 15.60, encode//sum 14.74, stage/render/prepare_assets 14.65, render/graph/ms 13.40, stream/drive/ms 11.32 |
| warm-up | 5.66 | 40.7 | pipelines/compile_ms 976, pipelines/front_ms 944, render/thread/ms 57.71, render/busy/ms 55.54, main/busy/ms 49.63, pipelines/driver_ms 31.14, stage/render/prepare_assets 29.29, stage/main/update 26.88, stage/main/post_update 15.91, stage/render/render 15.58, stream/drive/ms 13.68, encode//sum 13.23 |
| warm-up | 5.73 | 31.8 | main/busy/ms 36.73, render/thread/ms 34.89, render/busy/ms 32.71, stage/main/update 19.19, encode//sum 18.74, stage/render/render 14.73, pipelines/compile_ms 14.02, stream/drive/ms 13.44, pipelines/front_ms 13.22, render/graph/ms 12.31, stage/main/post_update 12.16, stage/render/graph_submit 9.18 |
| warm-up | 5.80 | 34.7 | pipelines/compile_ms 373, pipelines/front_ms 362, render/thread/ms 54.90, render/busy/ms 53.14, main/busy/ms 37.20, encode//sum 19.01, stage/main/post_update 17.99, stage/render/render 14.58, stage/main/update 13.63, stage/render/prepare_assets 13.44, render/graph/ms 12.38, pipelines/driver_ms 10.57 |
| warm-up | 5.83 | 30.2 | pipelines/compile_ms 218, pipelines/front_ms 212, render/thread/ms 53.70, render/busy/ms 52.05, main/busy/ms 31.86, stage/render/prepare_assets 25.93, encode//sum 18.31, stage/main/update 13.42, stage/render/render 12.46, stage/main/post_update 11.14, render/graph/ms 10.25, stage/render/graph_submit 7.55 |
| warm-up | 6.42 | 31.7 | render/thread/ms 56.48, render/busy/ms 54.66, main/busy/ms 34.53, encode//sum 25.33, stage/render/render 17.60, stage/render/prepare_assets 15.59, stage/main/post_update 15.33, render/graph/ms 15.03, stage/main/update 13.74, stage/render/graph_submit 11.29, encode/submit 6.81, stage/render/specialize 6.78 |
| warm-up | 6.46 | 35.0 | render/thread/ms 58.03, render/busy/ms 56.25, main/busy/ms 35.31, encode//sum 25.15, stage/render/prepare_assets 22.95, stage/render/render 18.40, stage/main/post_update 16.40, render/graph/ms 15.79, stage/main/update 13.50, stage/render/graph_submit 12.06, encode/submit 7.27, stream/drive/ms 5.70 |
| warm-up | 6.50 | 38.6 | render/thread/ms 56.16, render/busy/ms 55.02, main/busy/ms 35.62, stage/render/prepare_assets 33.82, encode//sum 18.08, stage/main/update 16.63, stage/main/post_update 12.29, stage/render/render 10.93, render/graph/ms 9.21, stage/render/graph_submit 6.97, stream/drive/ms 5.83, encode/finish_wall 3.73 |
| warm-up | 6.53 | 33.7 | render/thread/ms 58.56, render/busy/ms 57.37, main/busy/ms 36.30, stage/render/prepare_assets 35.98, encode//sum 17.71, stage/main/update 17.39, stage/main/post_update 12.08, stage/render/render 10.25, render/graph/ms 8.48, stage/render/graph_submit 6.20, stream/drive/ms 3.87, encode/finish_wall 3.17 |
| warm-up | 6.59 | 37.1 | render/thread/ms 55.33, render/busy/ms 53.99, main/busy/ms 29.25, stage/render/prepare_assets 26.65, encode//sum 23.87, stage/main/update 12.39, stage/main/post_update 11.98, stage/render/render 11.89, render/graph/ms 10.17, stage/render/graph_submit 7.57, encode/finish_wall 4.72, stream/drive/ms 4.13 |
| warm-up | 6.65 | 31.6 | render/thread/ms 46.59, render/busy/ms 45.36, main/busy/ms 35.50, stage/render/prepare_assets 24.62, stage/main/update 19.29, encode//sum 15.84, stage/main/post_update 10.66, stage/render/render 9.67, stream/drive/ms 9.45, render/graph/ms 8.06, stage/render/graph_submit 5.94, encode/finish_wall 3.38 |
| warm-up | 6.77 | 30.6 | pipelines/compile_ms 83.49, pipelines/front_ms 77.73, render/thread/ms 48.88, render/busy/ms 47.31, main/busy/ms 40.38, encode//sum 24.10, stage/main/update 17.73, stage/render/render 17.21, render/graph/ms 14.98, stage/render/prepare_assets 13.97, stage/main/post_update 13.88, stage/render/graph_submit 11.14 |
| warm-up | 6.85 | 39.1 | main/busy/ms 48.16, render/thread/ms 33.84, render/busy/ms 32.44, stage/main/update 26.16, encode//sum 21.06, stream/drive/ms 15.78, stage/main/post_update 14.75, stage/render/render 13.14, render/graph/ms 11.23, stage/render/graph_submit 8.22, stage/render/prepare_assets 7.90, encode/finish_wall 4.57 |
| warm-up | 6.88 | 32.5 | main/busy/ms 44.98, render/thread/ms 44.19, render/busy/ms 42.54, stage/main/update 23.93, encode//sum 21.38, stage/render/prepare_assets 14.85, stage/main/post_update 13.94, stage/render/render 12.70, render/graph/ms 10.66, stream/drive/ms 8.62, stage/render/graph_submit 7.41, encode/finish_wall 4.33 |
| measure | 0.88 | 33.1 | encode//sum 85.90, render/thread/ms 41.81, main/busy/ms 41.70, render/busy/ms 40.83, stage/main/post_update 34.51, stage/render/render 34.02, render/graph/ms 32.59, stage/render/graph_submit 30.31, encode/finish_wall 25.81, encode/submit 4.56, stage/main/update 4.48, stage/render/camera_driver 2.19 |
| measure | 18.19 | 31.5 | main/busy/ms 39.63, encode//sum 28.93, stage/main/update 24.75, render/thread/ms 23.08, render/busy/ms 21.97, stream/drive/ms 20.93, stage/render/render 14.14, render/graph/ms 12.62, stage/main/post_update 11.81, stage/render/graph_submit 9.81, encode/submit 4.98, encode/finish_wall 4.91 |
| measure | 59.08 | 37.9 | render/thread/ms 39.87, render/busy/ms 38.36, encode//sum 28.84, main/busy/ms 24.58, stage/render/render 16.95, render/graph/ms 15.10, stage/main/post_update 13.37, stage/render/prepare_assets 11.96, stage/render/graph_submit 11.64, stage/main/update 6.42, encode/finish_wall 6.02, encode/submit 5.74 |
| measure | 59.11 | 32.2 | render/thread/ms 39.25, render/busy/ms 38.11, stage/render/prepare_assets 20.36, main/busy/ms 19.20, encode//sum 17.74, stage/render/render 12.19, render/graph/ms 10.76, stage/main/post_update 9.82, stage/render/graph_submit 8.66, stage/main/update 4.96, encode/submit 4.41, encode/finish_wall 4.35 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_on\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **74%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.5 W · 70 °C (max 76) · VRAM 5955 MiB · throttled 100% of samples
