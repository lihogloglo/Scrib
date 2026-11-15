# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5313 frames over 65.6 s · **median 12.03 ms (83 fps)** · mean 12.36 · p95 15.34 · p99 18.66 (1% low 54 fps) · worst 32.97 ms · 69 spikes over 1.5× median · 40.2 stutters/min (work 37.5/min)
- warm-up: 438 frames over 9.4 s, worst 2837.6 ms
- hitches (> 30.1 ms): **2** measured, 23 in warm-up
- timed GPU passes: 7.42 ms of a 12.36 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.03 ms, limit 11 ms
- **FAIL** frame_ms: 44 stutters (> 1.5x moving median of 60) in 1.09 min = 40.21/min, limit 1/min
- pass worst frame 32.97 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.091 | 2.130 | 2.552 | 0.98 | 0.178 |
| main_opaque_pass_3d | 1.666 | 1.697 | 2.301 | 0.98 | 0.057 |
| early prepass | 0.523 | 0.532 | 0.815 | 0.98 | 0.079 |
| bloom | 0.322 | 0.328 | 0.332 | 0.98 | 0.065 |
| atmosphere_luts | 0.315 | 0.321 | 0.433 | 0.98 | 0.003 |
| water_velocity | 0.287 | 0.293 | 0.356 | 0.98 | 0.001 |
| water_sspr | 0.212 | 0.216 | 0.213 | 0.98 | 0.002 |
| ssao | 0.195 | 0.199 | 0.199 | 0.98 | 0.007 |
| uniform_allocation | 0.180 | 0.184 | 0.189 | 0.98 | 0.033 |
| atmos_froxel_inject | 0.150 | 0.153 | 0.152 | 0.98 | 0.002 |
| clouds_trace | 0.139 | 0.141 | 0.234 | 0.98 | 0.001 |
| auxiliary_views | 0.138 | 0.140 | 0.593 | 0.98 | 0.127 |
| bin_unpacking | 0.135 | 0.137 | 0.145 | 0.98 | 0.023 |
| water_fft | 0.133 | 0.136 | 0.139 | 0.98 | 0.003 |
| upscaling | 0.117 | 0.120 | 0.129 | 0.98 | 0.004 |
| clouds_composite | 0.107 | 0.109 | 0.111 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.089 | 0.091 | 0.174 | 0.98 | 0.008 |
| tonemapping | 0.088 | 0.089 | 0.091 | 0.98 | 0.004 |
| clustering | 0.068 | 0.069 | 0.070 | 0.98 | 0.020 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.98 | 0.002 |
| surface_trail | 0.051 | 0.066 | 0.060 | 0.78 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.434 | 0.640 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.049 | 0.049 | 0.98 | 0.005 |
| water_snapshot | 0.034 | 0.034 | 0.037 | 0.98 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.223 | 0.234 | 0.12 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.98 | 0.007 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.029 | 0.98 | 0.111 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.936 | 13.748 | 25.44 | 58100.5 |
| encode//sum | 10.775 | 15.626 | 58.94 | 57249.5 |
| render/busy/ms | 10.348 | 13.123 | 24.72 | 54979.7 |
| main/busy/ms | 9.344 | 11.898 | 28.56 | 49643.3 |
| render/graph/ms | 6.465 | 8.338 | 16.81 | 34345.9 |
| stage/main/post_update | 5.825 | 7.965 | 14.88 | 30946.2 |
| stage/render/graph_submit | 5.291 | 6.854 | 14.30 | 28112.4 |
| pipelines/wait_ms | 4.787 | 0.000 | 5353.20 | 25433.3 |
| stage/render/render | 4.141 | 8.556 | 18.36 | 22003.2 |
| stage/render/cleanup | 3.121 | 8.519 | 16.08 | 16579.9 |
| encode/finish_wall | 3.046 | 4.184 | 9.62 | 16186.0 |
| encode/submit | 2.303 | 3.129 | 5.93 | 12236.9 |
| stage/main/update | 2.207 | 2.761 | 22.34 | 11728.4 |
| stage/render/camera_driver | 1.114 | 1.525 | 4.80 | 5917.3 |
| stage/render/prepare_write_phase_buffers | 0.873 | 1.175 | 2.41 | 4636.3 |
| bigspace/total/ms | 0.641 | 2.028 | 4.51 | 3404.4 |
| stage/render/prepare_assets | 0.559 | 0.730 | 11.09 | 2971.9 |
| render/present/ms | 0.489 | 0.640 | 5.35 | 2598.5 |
| stage/render/prepare_resources | 0.433 | 0.546 | 2.39 | 2301.3 |
| stage/main/first | 0.415 | 0.546 | 5.39 | 2204.5 |
| stage/render/prepare_meshes | 0.400 | 0.547 | 2.97 | 2127.8 |
| stage/main/pre_update | 0.342 | 0.521 | 1.35 | 1816.0 |
| terrain/update/ms | 0.303 | 0.391 | 2.14 | 1608.0 |
| controller/feed/ms | 0.280 | 0.383 | 0.75 | 1489.2 |
| stage/render/specialize | 0.278 | 0.417 | 3.58 | 1478.0 |
| stage/render/queue | 0.275 | 0.454 | 2.68 | 1461.8 |
| bigspace/low_precision/ms | 0.272 | 0.892 | 3.93 | 1446.0 |
| terrain/select/ms | 0.268 | 0.346 | 0.76 | 1426.5 |
| stream/drive/ms | 0.267 | 0.376 | 20.44 | 1417.2 |
| stage/render/prepare_bind_groups | 0.256 | 0.442 | 2.97 | 1359.1 |
| stage/main/fixed_main_loop | 0.249 | 0.415 | 1.49 | 1322.7 |
| pipelines/compile_ms | 0.227 | 0.000 | 159.64 | 1203.5 |
| pipelines/front_ms | 0.218 | 0.000 | 153.92 | 1160.8 |
| stream/tick/ms | 0.206 | 0.290 | 1.01 | 1096.0 |
| bigspace/high_precision/ms | 0.205 | 0.764 | 4.17 | 1091.4 |
| far/select/ms | 0.198 | 0.287 | 0.79 | 1054.3 |
| anim/evaluate/ms | 0.197 | 0.332 | 3.25 | 1046.8 |
| stage/render/create_views | 0.188 | 0.254 | 0.93 | 1000.4 |
| stage/render/prepare_batch_phases | 0.179 | 0.259 | 0.83 | 949.9 |
| stage/render/extract_commands | 0.160 | 0.200 | 0.59 | 847.5 |
| stage/render/prepare_flush | 0.154 | 0.323 | 1.27 | 818.5 |
| anim/evaluate_cpu/ms | 0.149 | 0.197 | 0.77 | 793.8 |
| stage/render/prepare_views | 0.142 | 0.192 | 0.49 | 755.0 |
| controller/move/ms | 0.141 | 0.288 | 0.54 | 747.2 |
| stage/main/last | 0.120 | 0.207 | 1.72 | 635.4 |
| render/acquire/ms | 0.098 | 0.136 | 0.41 | 522.2 |
| stage/render/prepare_collect_phase_buffers | 0.066 | 0.136 | 0.46 | 352.3 |
| physics/step/ms | 0.054 | 0.093 | 0.63 | 288.7 |
| physics/insert/ms | 0.049 | 0.425 | 1.13 | 258.2 |
| encode/resolve_diagnostics/sum | 0.033 | 0.045 | 0.39 | 173.9 |
| stage/render/phase_sort | 0.031 | 0.054 | 0.36 | 165.6 |
| weather/map/ms | 0.025 | 0.034 | 0.23 | 134.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.14 | 79.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.78 | 60.3 |
| anim/apply/ms | 0.011 | 0.019 | 0.31 | 60.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.25 | 58.3 |
| weather/eval/ms | 0.011 | 0.014 | 0.37 | 56.1 |
| pipelines/driver_ms | 0.008 | 0.000 | 6.17 | 42.7 |
| script/update/ms | 0.008 | 0.012 | 0.06 | 40.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.32 | 29.7 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 28.5 |
| script/ui/ms | 0.005 | 0.007 | 0.27 | 28.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 26.0 |
| controller/write/ms | 0.004 | 0.005 | 0.16 | 19.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 15.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 2.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.07 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 11 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 36 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 15 |
| crowd/c0/joints/count | count | 1285 | 2262 | 707 |
| crowd/c0/meshes/count | count | 353 | 646 | 189 |
| crowd/c1/count | count | 145 | 158 | 146 |
| crowd/c1/joints/count | count | 5505 | 6092 | 5489 |
| crowd/c1/meshes/count | count | 1164 | 1338 | 1152 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.54 | 7.87 | 3.02 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.14 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.78 | 0.02 |
| encode/finish_critical | ms | 2.54 | 7.87 | 3.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.39 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.22 | 28.56 | 11.32 |
| gpu/timed/ms | ms | 8.20 | 11.36 | 8.42 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 833 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 833 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3461 | 5114 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3461 | 5114 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 226 | 28 |
| phases/shadow/bins | count | 4847 | 5716 | 2221 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 12.8 k | 5672 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 247 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 215 | 476 | 19 |
| physics/pending | count | 0 | 580 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 11 | 611 | 6 |
| preprocess/bind_groups_created | count | 11 | 611 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 181 | 270 | 173 |
| process/cpu_usage | % | 24.57 | 39.90 | 24.77 |
| process/mem_usage | GiB | 5.11 | 5.29 | 5.29 |
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
| started_load_count |  | 4764 | 4858 | 4858 |
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
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.16 | 69.67 | 38.87 |
| system/mem_usage | % | 69.07 | 69.94 | 69.91 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2590 | 2650 | 2650 |
| views/all/batch_sets | count | 85 | 273 | 78 |
| views/all/bins | count | 7433 | 8634 | 5719 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.7 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 249 | 294 | 74 |
| views/all/pipelines_distinct | count | 68 | 73 | 61 |
| views/all/work_items | count | 17.2 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2592 | 3107 | 2221 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8165 | 10.0 k | 5672 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8165 | 10.0 k | 5672 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2193 | 2697 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2433 | 3017 | 0 |
| views/lamp_face/pipelines | count | 169 | 204 | 0 |
| views/lamp_face/work_items | count | 0 | 2900 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2874 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8586 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8740 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 52.64 | 292 | 279.7 k |
| anim/events/count | count | 0.09 | 3 | 491 |
| assets/Image/added | count | 0.12 | 43 | 616 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.79 | 152 | 4178 |
| assets/Mesh/removed | count | 0.92 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 490 |
| assets/StandardMaterial/modified | count | 4.08 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 60 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.32 | 1510 | 7033 |
| encode//count | count | 38.50 | 272 | 204.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5313 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5313 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5313 |
| physics/inserts | count | 0.52 | 148 | 2757 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.74 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6372 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1397 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.1 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.84 | 2837.6 | main/busy/ms 2811, stage/main/post_update 2709, stage/main/update 96.38, surface/advance_cpu_ms 67.08, stage/main/pre_update 2.66, stage/main/last 0.99, surface/cell_upload_ms 0.73, stage/main/first 0.63, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.35, weather/map/ms 0.15, stream/tick/ms 0.11 |
| warm-up | 3.58 | 740.5 | main/busy/ms 2846, stage/main/post_update 2733, render/thread/ms 699, render/busy/ms 654, stage/render/prepare_assets 475, stage/render/prepare_resources 126, stage/main/update 101, surface/advance_cpu_ms 67.08, stage/render/prepare_views 43.17, render/acquire/ms 43.04, stage/render/render 34.97, render/graph/ms 32.04 |
| warm-up | 3.68 | 98.2 | render/thread/ms 708, render/busy/ms 661, stage/render/prepare_assets 475, pipelines/wait_ms 384, stage/render/prepare_resources 127, pipelines/compile_ms 121, pipelines/front_ms 114, main/busy/ms 50.77, stage/render/prepare_views 43.39, render/acquire/ms 43.20, stage/render/render 38.37, render/graph/ms 34.11 |
| warm-up | 3.91 | 78.3 | pipelines/compile_ms 225, pipelines/front_ms 211, main/busy/ms 57.90, stage/main/update 47.60, render/thread/ms 46.59, render/busy/ms 44.78, stage/render/prepare_assets 35.86, pipelines/driver_ms 14.16, stage/main/post_update 7.29, render/graph/ms 4.34, stage/render/cleanup 3.37, stage/render/render 3.03 |
| warm-up | 4.07 | 62.8 | render/thread/ms 42.89, main/busy/ms 42.86, render/busy/ms 41.50, stage/render/prepare_assets 34.97, stage/main/update 34.51, stream/drive/ms 31.20, stage/main/post_update 5.93, stage/render/render 4.04, render/graph/ms 2.36, encode//sum 1.33, stage/render/graph_submit 1.28, render/present/ms 1.12 |
| warm-up | 4.13 | 43.5 | pipelines/wait_ms 1465, pipelines/compile_ms 188, pipelines/front_ms 165, main/busy/ms 40.05, render/thread/ms 32.49, render/busy/ms 30.43, stage/main/update 24.00, pipelines/driver_ms 23.10, stream/drive/ms 20.24, stage/render/prepare_assets 18.29, stage/main/post_update 10.90, stage/render/render 7.47 |
| warm-up | 4.21 | 50.0 | pipelines/wait_ms 1361, pipelines/compile_ms 77.79, pipelines/front_ms 74.57, render/thread/ms 52.69, render/busy/ms 50.87, stage/render/prepare_assets 35.99, main/busy/ms 35.14, stage/main/update 20.62, stream/drive/ms 16.11, stage/render/render 8.09, stage/main/post_update 7.60, render/graph/ms 5.56 |
| warm-up | 4.33 | 93.7 | render/thread/ms 76.34, render/busy/ms 48.79, main/busy/ms 45.61, stage/render/prepare_assets 38.12, stage/main/update 30.19, stage/render/cleanup 28.61, render/present/ms 27.38, stream/drive/ms 25.85, stage/main/post_update 9.52, render/graph/ms 3.04, stage/render/render 2.42, stage/main/pre_update 1.88 |
| warm-up | 4.40 | 42.6 | pipelines/wait_ms 8757, pipelines/compile_ms 318, pipelines/front_ms 216, encode//sum 113, pipelines/driver_ms 102, render/thread/ms 60.99, render/busy/ms 59.28, main/busy/ms 22.89, render/graph/ms 20.51, stage/render/render 18.30, stage/render/graph_submit 16.90, stage/render/prepare_assets 14.30 |
| warm-up | 4.46 | 30.6 | pipelines/wait_ms 4114, pipelines/compile_ms 53.64, encode//sum 53.53, pipelines/front_ms 50.14, render/thread/ms 47.35, render/busy/ms 45.09, stage/render/render 21.02, main/busy/ms 18.25, render/graph/ms 17.97, stage/render/graph_submit 12.29, stage/render/prepare_assets 9.18, stage/main/update 8.64 |
| warm-up | 4.52 | 38.9 | main/busy/ms 67.44, stage/main/update 52.82, stream/drive/ms 48.48, render/thread/ms 36.80, render/busy/ms 35.35, stage/render/prepare_assets 19.64, stage/main/post_update 7.58, stage/render/render 3.43, render/graph/ms 3.31, encode//sum 2.72, stage/render/prepare_bind_groups 2.65, stream/tick/ms 2.29 |
| warm-up | 4.60 | 77.4 | pipelines/wait_ms 3162, pipelines/compile_ms 151, pipelines/front_ms 137, main/busy/ms 71.75, stage/main/update 53.09, render/thread/ms 47.70, stream/drive/ms 47.65, render/busy/ms 46.57, stage/render/prepare_assets 35.82, pipelines/driver_ms 13.35, stage/main/post_update 12.74, stage/render/cleanup 4.15 |
| warm-up | 4.68 | 67.5 | pipelines/wait_ms 13.3 k, pipelines/compile_ms 94.02, pipelines/front_ms 86.53, main/busy/ms 64.74, stage/main/update 50.02, stream/drive/ms 45.24, render/thread/ms 34.08, render/busy/ms 32.81, stage/render/prepare_assets 18.47, stage/main/post_update 7.97, pipelines/driver_ms 7.49, stage/render/cleanup 4.96 |
| warm-up | 4.75 | 48.3 | pipelines/wait_ms 2586, main/busy/ms 49.64, stage/main/update 39.81, stream/drive/ms 36.08, render/thread/ms 30.65, render/busy/ms 29.30, pipelines/compile_ms 12.37, pipelines/front_ms 11.52, stage/render/prepare_assets 11.00, stage/render/render 8.68, render/graph/ms 6.93, stage/main/post_update 5.72 |
| warm-up | 4.79 | 33.0 | pipelines/wait_ms 5910, pipelines/compile_ms 95.48, pipelines/front_ms 90.55, main/busy/ms 34.85, stage/main/update 27.46, stream/drive/ms 24.11, render/thread/ms 15.90, render/busy/ms 14.86, stage/render/render 6.51, render/graph/ms 5.19, pipelines/driver_ms 4.93, encode//sum 4.50 |
| warm-up | 4.91 | 31.0 | render/thread/ms 37.89, render/busy/ms 36.38, main/busy/ms 22.28, stage/render/prepare_assets 21.81, stage/main/post_update 8.65, stage/main/update 7.68, render/graph/ms 5.89, stage/render/render 5.10, encode//sum 5.03, stage/render/graph_submit 3.89, stream/drive/ms 3.68, encode/submit 2.93 |
| warm-up | 5.07 | 36.9 | pipelines/wait_ms 6649, pipelines/compile_ms 106, pipelines/front_ms 100, render/thread/ms 36.96, main/busy/ms 36.37, render/busy/ms 35.78, stage/render/prepare_assets 22.30, stage/main/update 20.11, stage/main/post_update 12.62, stage/render/cleanup 6.55, pipelines/driver_ms 5.84, render/graph/ms 4.89 |
| warm-up | 5.11 | 31.3 | render/thread/ms 33.79, render/busy/ms 32.41, main/busy/ms 27.47, stage/render/prepare_assets 17.07, stage/main/post_update 12.02, stage/main/update 10.88, stage/render/render 6.29, encode//sum 5.08, render/graph/ms 4.42, stage/render/graph_submit 2.81, terrain/update/ms 2.52, stream/drive/ms 2.11 |
| warm-up | 5.61 | 30.2 | render/thread/ms 31.54, render/busy/ms 30.43, main/busy/ms 22.37, stage/render/prepare_assets 15.61, stage/main/post_update 8.56, stage/main/update 8.11, stage/render/render 6.11, encode//sum 5.58, render/graph/ms 4.66, stream/drive/ms 4.13, stage/render/graph_submit 3.34, stage/main/pre_update 2.67 |
| warm-up | 5.81 | 34.7 | pipelines/wait_ms 5674, pipelines/compile_ms 152, pipelines/front_ms 147, render/thread/ms 39.28, render/busy/ms 38.07, main/busy/ms 28.55, stage/render/prepare_assets 23.36, stage/main/update 11.80, stage/main/post_update 10.97, stage/render/render 6.10, encode//sum 5.83, stream/drive/ms 4.97 |
| warm-up | 7.55 | 32.8 | main/busy/ms 38.96, render/thread/ms 30.85, render/busy/ms 29.59, stage/main/update 26.32, encode//sum 17.92, stage/render/render 14.26, render/graph/ms 12.39, stage/render/graph_submit 9.39, stage/main/post_update 9.24, stage/render/prepare_assets 9.03, encode/finish_wall 4.84, encode/submit 4.58 |
| warm-up | 8.84 | 44.4 | pipelines/wait_ms 3977, main/busy/ms 46.47, stage/main/update 32.61, render/thread/ms 30.05, render/busy/ms 28.90, encode//sum 16.98, pipelines/compile_ms 14.27, pipelines/front_ms 13.64, render/graph/ms 11.24, stage/main/post_update 11.02, stage/render/prepare_assets 9.99, stage/render/graph_submit 8.13 |
| warm-up | 9.20 | 30.5 | pipelines/wait_ms 4794, main/busy/ms 39.15, render/thread/ms 25.60, stage/main/update 25.23, render/busy/ms 24.45, encode//sum 20.29, render/graph/ms 11.21, stage/main/post_update 10.42, stage/render/graph_submit 8.66, stage/render/cleanup 7.43, pipelines/compile_ms 7.11, pipelines/front_ms 6.94 |
| measure | 59.03 | 33.0 | render/thread/ms 46.61, render/busy/ms 44.59, encode//sum 39.57, main/busy/ms 36.89, render/graph/ms 23.87, stage/main/post_update 20.57, stage/render/graph_submit 19.60, stage/render/render 18.36, encode/finish_wall 12.92, stage/main/update 10.28, stage/render/prepare_assets 10.03, stage/render/cleanup 8.06 |
| measure | 59.06 | 31.4 | render/thread/ms 42.49, render/busy/ms 41.09, main/busy/ms 27.80, encode//sum 22.50, stage/render/prepare_assets 20.43, stage/main/post_update 13.50, render/graph/ms 12.29, stage/render/graph_submit 9.56, stage/main/update 8.10, stage/render/cleanup 8.06, stage/render/render 5.90, encode/finish_wall 5.63 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\b2_on\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **78%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.7 W · 73 °C (max 79) · VRAM 7024 MiB · throttled 100% of samples
