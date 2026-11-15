# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5546 frames over 65.6 s · **median 11.60 ms (86 fps)** · mean 11.84 · p95 14.14 · p99 16.23 (1% low 62 fps) · worst 42.59 ms · 28 spikes over 1.5× median · 18.3 stutters/min (work 21.0/min)
- warm-up: 371 frames over 8.5 s, worst 2573.4 ms
- hitches (> 29.0 ms): **4** measured, 16 in warm-up
- timed GPU passes: 7.21 ms of a 11.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.60 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- **FAIL** worst frame 42.59 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.021 | 2.073 | 2.118 | 0.97 | 0.175 |
| main_opaque_pass_3d | 1.607 | 1.649 | 2.038 | 0.97 | 0.060 |
| early prepass | 0.508 | 0.521 | 0.748 | 0.97 | 0.078 |
| bloom | 0.312 | 0.321 | 0.328 | 0.97 | 0.062 |
| atmosphere_luts | 0.304 | 0.312 | 0.372 | 0.97 | 0.003 |
| water_velocity | 0.279 | 0.286 | 0.353 | 0.97 | 0.002 |
| water_sspr | 0.212 | 0.217 | 0.219 | 0.97 | 0.002 |
| ssao | 0.188 | 0.193 | 0.195 | 0.97 | 0.007 |
| uniform_allocation | 0.179 | 0.184 | 0.189 | 0.98 | 0.033 |
| auxiliary_views | 0.151 | 0.154 | 0.534 | 0.98 | 0.146 |
| atmos_froxel_inject | 0.144 | 0.148 | 0.150 | 0.97 | 0.001 |
| clouds_trace | 0.137 | 0.140 | 0.233 | 0.97 | 0.001 |
| bin_unpacking | 0.133 | 0.137 | 0.147 | 0.98 | 0.023 |
| water_fft | 0.129 | 0.132 | 0.137 | 0.97 | 0.003 |
| upscaling | 0.116 | 0.119 | 0.140 | 0.97 | 0.004 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.085 | 0.088 | 0.167 | 0.97 | 0.008 |
| tonemapping | 0.084 | 0.086 | 0.090 | 0.97 | 0.004 |
| clustering | 0.066 | 0.068 | 0.070 | 0.97 | 0.019 |
| render_sky | 0.052 | 0.054 | 0.054 | 0.97 | 0.002 |
| surface_trail | 0.052 | 0.070 | 0.207 | 0.74 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.426 | 0.424 | 0.11 | 0.001 |
| early_mesh_preprocessing | 0.046 | 0.048 | 0.049 | 0.98 | 0.005 |
| water_snapshot | 0.031 | 0.032 | 0.036 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.025 | 0.028 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.223 | 0.234 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 0.97 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.031 | 0.97 | 0.110 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 17.018 | 0.000 | 9119.46 | 94382.3 |
| encode//sum | 10.578 | 15.600 | 78.20 | 58666.4 |
| render/thread/ms | 10.545 | 12.716 | 41.44 | 58480.8 |
| render/busy/ms | 9.956 | 12.137 | 40.58 | 55216.9 |
| main/busy/ms | 9.229 | 11.355 | 28.43 | 51182.2 |
| stage/render/render | 6.728 | 8.314 | 31.74 | 37313.4 |
| render/graph/ms | 5.946 | 7.489 | 30.45 | 32979.1 |
| stage/main/post_update | 5.827 | 7.727 | 10.02 | 32316.5 |
| stage/render/graph_submit | 4.777 | 6.055 | 26.54 | 26493.6 |
| encode/finish_wall | 2.507 | 3.329 | 21.36 | 13905.4 |
| encode/submit | 2.338 | 3.100 | 6.23 | 12968.3 |
| stage/main/update | 2.133 | 2.569 | 22.41 | 11828.1 |
| stage/render/camera_driver | 1.113 | 1.480 | 4.57 | 6170.0 |
| stage/render/prepare_write_phase_buffers | 0.911 | 1.178 | 2.56 | 5051.2 |
| bigspace/total/ms | 0.567 | 1.858 | 3.60 | 3144.3 |
| stage/render/prepare_assets | 0.561 | 0.720 | 12.46 | 3114.0 |
| render/present/ms | 0.493 | 0.620 | 3.30 | 2734.1 |
| stage/render/prepare_resources | 0.446 | 0.543 | 1.01 | 2472.2 |
| stage/main/first | 0.406 | 0.530 | 4.96 | 2249.0 |
| stage/render/prepare_meshes | 0.370 | 0.487 | 1.52 | 2050.4 |
| pipelines/compile_ms | 0.334 | 0.000 | 164.05 | 1850.2 |
| stage/main/pre_update | 0.331 | 0.497 | 1.17 | 1835.2 |
| pipelines/front_ms | 0.320 | 0.000 | 158.33 | 1773.6 |
| terrain/update/ms | 0.301 | 0.392 | 1.85 | 1668.4 |
| terrain/select/ms | 0.268 | 0.349 | 0.84 | 1484.1 |
| stage/render/specialize | 0.266 | 0.393 | 2.28 | 1476.9 |
| controller/feed/ms | 0.265 | 0.344 | 0.79 | 1467.8 |
| stage/render/queue | 0.262 | 0.416 | 2.64 | 1452.9 |
| stage/render/prepare_bind_groups | 0.254 | 0.433 | 3.08 | 1407.7 |
| stream/drive/ms | 0.252 | 0.346 | 20.45 | 1399.0 |
| bigspace/low_precision/ms | 0.237 | 0.695 | 3.38 | 1313.1 |
| stage/main/fixed_main_loop | 0.235 | 0.381 | 1.24 | 1302.8 |
| far/select/ms | 0.197 | 0.272 | 0.76 | 1092.5 |
| stream/tick/ms | 0.193 | 0.267 | 0.76 | 1072.1 |
| stage/render/create_views | 0.183 | 0.238 | 8.28 | 1016.3 |
| stage/render/extract_commands | 0.174 | 0.208 | 0.40 | 967.6 |
| stage/render/prepare_batch_phases | 0.171 | 0.239 | 0.50 | 946.5 |
| bigspace/high_precision/ms | 0.171 | 0.619 | 2.73 | 946.0 |
| stage/render/prepare_flush | 0.160 | 0.320 | 0.81 | 888.2 |
| anim/evaluate/ms | 0.159 | 0.262 | 2.50 | 879.1 |
| anim/evaluate_cpu/ms | 0.150 | 0.196 | 0.47 | 834.0 |
| stage/render/prepare_views | 0.139 | 0.179 | 0.42 | 768.6 |
| controller/move/ms | 0.128 | 0.237 | 0.64 | 708.0 |
| stage/main/last | 0.116 | 0.206 | 1.61 | 645.1 |
| render/acquire/ms | 0.096 | 0.129 | 0.37 | 529.9 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.125 | 0.34 | 315.9 |
| physics/step/ms | 0.052 | 0.089 | 0.73 | 289.6 |
| physics/insert/ms | 0.045 | 0.416 | 0.85 | 248.4 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.56 | 173.9 |
| stage/render/cleanup | 0.031 | 0.085 | 0.80 | 170.3 |
| stage/render/phase_sort | 0.029 | 0.050 | 0.25 | 159.7 |
| weather/map/ms | 0.024 | 0.032 | 0.25 | 134.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.49 | 84.9 |
| pipelines/driver_ms | 0.014 | 0.000 | 13.36 | 76.6 |
| anim/apply/ms | 0.012 | 0.021 | 0.22 | 65.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.33 | 61.5 |
| controller/avoid/ms | 0.010 | 0.014 | 0.06 | 57.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 55.8 |
| script/update/ms | 0.007 | 0.009 | 0.24 | 39.8 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 29.2 |
| stream/despawn/ms | 0.005 | 0.000 | 0.99 | 29.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 26.3 |
| script/ui/ms | 0.005 | 0.006 | 0.10 | 25.5 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 18.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.57 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.16 | 3.0 |
| weather/apply/ms | 0.000 | 0.000 | 0.02 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 13 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 39 | 16 |
| controller/movers/count | count | 19 | 21 | 16 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1247 | 2262 | 480 |
| crowd/c0/meshes/count | count | 343 | 646 | 117 |
| crowd/c1/count | count | 147 | 157 | 149 |
| crowd/c1/joints/count | count | 5539 | 6050 | 5684 |
| crowd/c1/meshes/count | count | 1181 | 1324 | 1216 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.04 | 19.94 | 1.94 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.49 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.33 | 0.01 |
| encode/finish_critical | ms | 2.04 | 19.94 | 1.94 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.56 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.96 | 40.58 | 8.89 |
| gpu/timed/ms | ms | 8.09 | 11.77 | 8.56 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1294 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3409 | 5131 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1294 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3409 | 5131 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 224 | 28 |
| phases/shadow/bins | count | 4832 | 5653 | 2229 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 12.8 k | 5680 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 248 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2641 | 3268 | 2345 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 826 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 11 | 607 | 6 |
| preprocess/bind_groups_created | count | 11 | 607 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 181 | 270 | 173 |
| process/cpu_usage | % | 25.96 | 39.62 | 17.94 |
| process/mem_usage | GiB | 5.05 | 5.24 | 5.24 |
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
| started_load_count |  | 4764 | 4859 | 4859 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.38 | 49.78 | 30.55 |
| system/mem_usage | % | 60.21 | 61.05 | 61.05 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 87 | 271 | 78 |
| views/all/bins | count | 7381 | 8566 | 5727 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 248 | 295 | 74 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.4 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2605 | 3153 | 2229 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8159 | 9932 | 5680 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8159 | 9932 | 5680 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2160 | 2790 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2416 | 3083 | 0 |
| views/lamp_face/pipelines | count | 168 | 208 | 0 |
| views/lamp_face/work_items | count | 0 | 2887 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8440 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8601 | 13.3 k | 7918 |
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
| anim/bones_written/count | count | 61.60 | 429 | 341.6 k |
| anim/events/count | count | 0.09 | 3 | 507 |
| assets/Image/added | count | 0.11 | 43 | 598 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.74 | 152 | 4096 |
| assets/Mesh/removed | count | 0.88 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 472 |
| assets/StandardMaterial/modified | count | 3.91 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.17 | 1605 | 6494 |
| encode//count | count | 39.20 | 272 | 217.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5546 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5546 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5546 |
| physics/inserts | count | 0.51 | 111 | 2812 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.71 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6104 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1338 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.0 ms; slowest 20, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2573.4 | main/busy/ms 2547, stage/main/post_update 2441, stage/main/update 100, surface/advance_cpu_ms 71.09, stage/main/pre_update 2.48, stage/main/last 1, surface/cell_upload_ms 0.75, stage/main/first 0.57, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.34, weather/map/ms 0.15, stream/tick/ms 0.09 |
| warm-up | 3.28 | 705.5 | main/busy/ms 2582, stage/main/post_update 2466, render/thread/ms 665, render/busy/ms 617, stage/render/prepare_assets 433, stage/render/prepare_resources 125, stage/main/update 104, surface/advance_cpu_ms 71.09, stage/render/prepare_views 45.74, render/acquire/ms 45.56, stage/render/render 38.92, render/graph/ms 35.80 |
| warm-up | 3.37 | 93.5 | render/thread/ms 674, pipelines/wait_ms 627, render/busy/ms 625, stage/render/prepare_assets 434, stage/render/prepare_resources 126, pipelines/compile_ms 121, pipelines/front_ms 113, main/busy/ms 54.57, stage/render/prepare_views 46.04, render/acquire/ms 45.81, stage/render/render 41.96, render/graph/ms 37.68 |
| warm-up | 3.62 | 85.6 | pipelines/compile_ms 237, pipelines/front_ms 224, main/busy/ms 63.08, stage/main/update 52.72, render/thread/ms 46.29, render/busy/ms 44.05, stage/render/prepare_assets 34.23, pipelines/driver_ms 12.20, stage/render/render 7.34, stage/main/post_update 7.26, render/graph/ms 4.62, stage/render/graph_submit 2.69 |
| warm-up | 3.77 | 55.8 | render/thread/ms 38.73, main/busy/ms 37.99, render/busy/ms 37.23, stage/main/update 30.49, stage/render/prepare_assets 28.66, stream/drive/ms 26.51, stage/render/render 4.73, stage/main/post_update 4.73, render/graph/ms 2.69, encode//sum 1.55, stage/render/graph_submit 1.46, render/present/ms 1.30 |
| warm-up | 3.86 | 87.9 | pipelines/wait_ms 1077, pipelines/compile_ms 235, pipelines/front_ms 211, main/busy/ms 54.23, render/thread/ms 46.28, render/busy/ms 44.01, stage/main/update 37.21, stream/drive/ms 33.38, stage/render/prepare_assets 29.80, pipelines/driver_ms 23.49, stage/main/post_update 13.98, stage/render/render 8.80 |
| warm-up | 3.95 | 66.1 | pipelines/wait_ms 933, render/thread/ms 60.36, render/busy/ms 58.20, stage/render/prepare_assets 43.92, main/busy/ms 34.37, pipelines/compile_ms 23.30, pipelines/front_ms 21.29, stage/main/update 20.26, stream/drive/ms 15.10, stage/render/render 8.52, stage/main/post_update 7.27, render/graph/ms 5.92 |
| warm-up | 3.99 | 36.7 | render/thread/ms 69.27, render/busy/ms 67.71, stage/render/prepare_assets 54.88, main/busy/ms 51.03, stage/main/update 38.27, stream/drive/ms 33.05, stage/main/post_update 6.91, stage/render/render 6.49, stream/spawn/ms 5.10, render/graph/ms 4.60, stage/render/graph_submit 3.31, encode/submit 2.89 |
| warm-up | 4.04 | 51.8 | pipelines/wait_ms 2111, pipelines/compile_ms 176, pipelines/front_ms 168, render/thread/ms 49.17, main/busy/ms 48.87, render/busy/ms 47.97, stage/render/prepare_assets 38.69, stage/main/update 33.52, stream/drive/ms 28.95, stage/main/post_update 10.03, pipelines/driver_ms 8.54, stage/render/render 4.14 |
| warm-up | 4.23 | 84.9 | render/thread/ms 66.81, render/busy/ms 64.51, main/busy/ms 57.36, stage/main/update 48.95, stream/drive/ms 45.46, encode//sum 39.88, stage/render/prepare_assets 23.44, stage/render/render 19.68, render/graph/ms 16.51, stage/render/graph_submit 11.85, stage/render/create_views 10.83, encode/submit 6.68 |
| warm-up | 4.33 | 91.0 | pipelines/wait_ms 6202, pipelines/compile_ms 224, pipelines/front_ms 202, main/busy/ms 59.37, render/thread/ms 51.13, stage/main/update 46.26, stream/drive/ms 41.86, stage/render/render 31.71, render/present/ms 26.25, render/busy/ms 24.68, pipelines/driver_ms 22.22, stage/render/prepare_assets 14.04 |
| warm-up | 4.41 | 53.9 | pipelines/wait_ms 16.6 k, pipelines/compile_ms 141, pipelines/front_ms 115, main/busy/ms 61.29, stage/main/update 46.20, render/thread/ms 45.89, render/busy/ms 44.03, stream/drive/ms 40.55, pipelines/driver_ms 26.11, stage/render/prepare_assets 18.55, stage/render/render 10.21, stage/main/post_update 9.10 |
| warm-up | 4.45 | 30.7 | pipelines/wait_ms 3781, pipelines/compile_ms 39.35, main/busy/ms 36.12, pipelines/front_ms 36.03, stage/main/update 27.28, stream/drive/ms 24.02, render/thread/ms 17.60, render/busy/ms 16.45, stage/render/render 6.21, stage/main/post_update 5.71, render/graph/ms 4.73, stage/render/graph_submit 3.34 |
| warm-up | 4.67 | 36.9 | pipelines/wait_ms 10.8 k, pipelines/compile_ms 72.69, pipelines/front_ms 66.68, render/thread/ms 45.38, render/busy/ms 44.20, main/busy/ms 35.03, stage/render/prepare_assets 31.24, stage/main/update 17.93, stage/main/post_update 13.05, stream/drive/ms 6.69, stage/render/render 6.63, pipelines/driver_ms 6.01 |
| warm-up | 5.77 | 32.3 | pipelines/wait_ms 10.8 k, pipelines/compile_ms 85.92, pipelines/front_ms 79.41, main/busy/ms 38.70, render/thread/ms 26.93, stage/main/update 25.46, render/busy/ms 24.73, stage/render/render 10.00, stage/main/post_update 9.80, encode//sum 8.84, stream/drive/ms 8.58, stage/render/prepare_assets 7.24 |
| warm-up | 5.89 | 56.9 | main/busy/ms 63.59, stage/main/update 46.80, render/thread/ms 39.74, render/busy/ms 38.02, stage/render/prepare_assets 17.53, stage/render/render 12.60, encode//sum 11.48, stage/main/post_update 10.52, render/graph/ms 9.95, stage/render/graph_submit 6.55, stream/drive/ms 5.17, encode/submit 4.55 |
| measure | 2.18 | 42.6 | encode//sum 90.03, render/thread/ms 54.32, render/busy/ms 52.97, stage/render/render 40.83, render/graph/ms 38.82, stage/render/graph_submit 33.36, encode/finish_wall 25.63, pipelines/compile_ms 25.22, main/busy/ms 19.82, pipelines/driver_ms 13.36, stage/main/post_update 12.94, pipelines/front_ms 11.87 |
| measure | 2.72 | 32.1 | encode//sum 49.66, render/thread/ms 41.27, render/busy/ms 39.90, stage/render/render 21.29, main/busy/ms 19.84, render/graph/ms 19.47, stage/render/graph_submit 14.48, stage/main/post_update 13.06, stage/render/create_views 8.47, encode/submit 7.42, encode/finish_wall 7.13, stage/render/camera_driver 4.85 |
| measure | 18.18 | 30.9 | main/busy/ms 39.01, stage/main/update 25.06, encode//sum 23.96, render/thread/ms 23.21, render/busy/ms 22.10, stream/drive/ms 20.71, stage/render/render 14.02, render/graph/ms 12.50, stage/main/post_update 11.03, stage/render/graph_submit 9.75, encode/submit 4.99, encode/finish_wall 4.84 |
| measure | 59.05 | 33.8 | render/thread/ms 40.83, render/busy/ms 39.53, main/busy/ms 25.48, stage/render/prepare_assets 18.86, encode//sum 18.53, stage/main/post_update 13.23, stage/render/render 11.90, render/graph/ms 10.37, stage/render/graph_submit 7.87, stage/main/update 7.23, encode/finish_wall 4.17, encode/submit 3.79 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\scene_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **79%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.8 W · 74 °C (max 80) · VRAM 6031 MiB · throttled 100% of samples
