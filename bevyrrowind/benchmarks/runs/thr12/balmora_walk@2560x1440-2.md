# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5133 frames over 65.7 s · **median 12.47 ms (80 fps)** · mean 12.79 · p95 15.59 · p99 18.62 (1% low 54 fps) · worst 41.39 ms · 48 spikes over 1.5× median · 21.9 stutters/min (work 24.7/min)
- warm-up: 412 frames over 9.4 s, worst 2561.2 ms
- hitches (> 31.2 ms): **5** measured, 21 in warm-up
- timed GPU passes: 7.91 ms of a 12.79 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.47 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- **FAIL** worst frame 41.39 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.264 | 2.321 | 2.827 | 0.98 | 0.185 |
| main_opaque_pass_3d | 1.793 | 1.838 | 2.575 | 0.98 | 0.062 |
| early prepass | 0.554 | 0.568 | 0.960 | 0.98 | 0.083 |
| bloom | 0.346 | 0.355 | 0.599 | 0.98 | 0.070 |
| atmosphere_luts | 0.335 | 0.344 | 0.600 | 0.98 | 0.004 |
| water_velocity | 0.318 | 0.326 | 0.663 | 0.98 | 0.002 |
| water_sspr | 0.232 | 0.238 | 0.472 | 0.98 | 0.003 |
| ssao | 0.209 | 0.214 | 0.438 | 0.98 | 0.008 |
| uniform_allocation | 0.188 | 0.192 | 0.194 | 0.98 | 0.036 |
| atmos_froxel_inject | 0.165 | 0.169 | 0.396 | 0.98 | 0.002 |
| bin_unpacking | 0.144 | 0.148 | 0.157 | 0.98 | 0.026 |
| clouds_trace | 0.138 | 0.141 | 0.236 | 0.98 | 0.001 |
| water_fft | 0.137 | 0.141 | 0.140 | 0.98 | 0.003 |
| upscaling | 0.130 | 0.133 | 0.167 | 0.98 | 0.005 |
| clouds_composite | 0.113 | 0.116 | 0.112 | 0.98 | 0.002 |
| auxiliary_views | 0.106 | 0.108 | 0.451 | 0.98 | 0.111 |
| main_transmissive_pass_3d | 0.096 | 0.099 | 0.184 | 0.98 | 0.009 |
| tonemapping | 0.093 | 0.095 | 0.096 | 0.98 | 0.004 |
| clustering | 0.070 | 0.072 | 0.071 | 0.98 | 0.022 |
| surface_trail | 0.054 | 0.069 | 0.061 | 0.78 | 0.003 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.052 | 0.053 | 0.051 | 0.98 | 0.006 |
| lightprobe_irradiance_map | 0.051 | 0.467 | 0.752 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.98 | 0.001 |
| water_snapshot | 0.039 | 0.040 | 0.043 | 0.98 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.98 | 0.001 |
| water_mips | 0.025 | 0.026 | 0.024 | 0.98 | 0.003 |
| lightprobe_radiance_map | 0.025 | 0.227 | 0.234 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.98 | 0.008 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.031 | 0.98 | 0.117 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.98 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.008 | 0.008 | 0.008 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.005 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.630 | 17.586 | 77.95 | 64827.4 |
| render/thread/ms | 11.328 | 13.872 | 32.70 | 58146.5 |
| render/busy/ms | 10.720 | 13.263 | 31.72 | 55025.4 |
| main/busy/ms | 9.436 | 12.080 | 34.90 | 48437.3 |
| pipelines/wait_ms | 7.805 | 0.000 | 5927.33 | 40061.9 |
| render/graph/ms | 6.580 | 8.191 | 18.77 | 33774.7 |
| stage/main/post_update | 5.724 | 7.897 | 11.45 | 29379.8 |
| stage/render/graph_submit | 5.342 | 6.689 | 13.67 | 27419.2 |
| stage/render/render | 4.344 | 8.671 | 18.77 | 22296.1 |
| encode/finish_wall | 3.073 | 3.996 | 7.44 | 15775.9 |
| stage/render/cleanup | 3.061 | 8.455 | 19.95 | 15712.6 |
| encode/submit | 2.334 | 3.206 | 6.74 | 11979.3 |
| stage/main/update | 2.330 | 2.966 | 26.07 | 11962.2 |
| stage/render/camera_driver | 1.174 | 1.581 | 5.01 | 6024.2 |
| stage/render/prepare_write_phase_buffers | 0.967 | 1.300 | 3.09 | 4963.4 |
| stage/render/prepare_assets | 0.605 | 0.807 | 12.60 | 3104.9 |
| bigspace/total/ms | 0.596 | 1.431 | 3.51 | 3058.8 |
| render/present/ms | 0.505 | 0.659 | 2.03 | 2591.7 |
| stage/render/prepare_resources | 0.468 | 0.603 | 1.63 | 2404.7 |
| stage/main/first | 0.431 | 0.582 | 4.89 | 2211.7 |
| stage/render/prepare_meshes | 0.383 | 0.518 | 1.66 | 1967.8 |
| stage/main/pre_update | 0.362 | 0.541 | 8.01 | 1856.7 |
| pipelines/compile_ms | 0.348 | 0.000 | 185.72 | 1785.2 |
| pipelines/front_ms | 0.336 | 0.000 | 179.04 | 1723.9 |
| terrain/update/ms | 0.319 | 0.427 | 1.83 | 1638.9 |
| stage/render/specialize | 0.296 | 0.454 | 2.34 | 1519.9 |
| controller/feed/ms | 0.294 | 0.389 | 0.82 | 1511.7 |
| stage/render/queue | 0.294 | 0.490 | 2.67 | 1510.3 |
| terrain/select/ms | 0.283 | 0.374 | 0.82 | 1453.2 |
| stream/drive/ms | 0.275 | 0.423 | 21.43 | 1413.4 |
| stage/render/prepare_bind_groups | 0.270 | 0.469 | 3.36 | 1384.4 |
| stage/main/fixed_main_loop | 0.269 | 0.442 | 1.87 | 1379.9 |
| bigspace/high_precision/ms | 0.219 | 0.626 | 3.12 | 1121.9 |
| far/select/ms | 0.215 | 0.303 | 1.07 | 1103.6 |
| stream/tick/ms | 0.208 | 0.307 | 0.83 | 1066.4 |
| bigspace/low_precision/ms | 0.202 | 0.539 | 3.11 | 1035.8 |
| stage/render/create_views | 0.201 | 0.281 | 7.69 | 1029.4 |
| stage/render/prepare_batch_phases | 0.181 | 0.259 | 0.52 | 930.0 |
| anim/evaluate_cpu/ms | 0.177 | 0.234 | 0.65 | 906.7 |
| stage/render/prepare_flush | 0.174 | 0.332 | 0.92 | 892.6 |
| stage/render/extract_commands | 0.166 | 0.211 | 0.88 | 850.1 |
| controller/move/ms | 0.155 | 0.338 | 0.81 | 795.0 |
| anim/evaluate/ms | 0.151 | 0.287 | 3.47 | 773.8 |
| stage/render/prepare_views | 0.150 | 0.213 | 0.68 | 770.9 |
| stage/main/last | 0.126 | 0.224 | 1.56 | 645.2 |
| render/acquire/ms | 0.103 | 0.149 | 0.60 | 529.4 |
| stage/render/prepare_collect_phase_buffers | 0.067 | 0.135 | 0.28 | 345.1 |
| physics/step/ms | 0.058 | 0.095 | 0.95 | 299.1 |
| physics/insert/ms | 0.058 | 0.468 | 0.91 | 297.4 |
| encode/resolve_diagnostics/sum | 0.039 | 0.052 | 4.03 | 201.9 |
| stage/render/phase_sort | 0.030 | 0.051 | 0.34 | 152.7 |
| weather/map/ms | 0.026 | 0.035 | 0.33 | 134.7 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.025 | 0.27 | 91.8 |
| encode/camera_driver_span_end/sum | 0.014 | 0.023 | 0.41 | 72.6 |
| anim/apply/ms | 0.012 | 0.023 | 0.14 | 63.3 |
| pipelines/driver_ms | 0.012 | 0.000 | 7.72 | 61.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.15 | 57.9 |
| controller/avoid/ms | 0.011 | 0.015 | 0.06 | 57.0 |
| script/update/ms | 0.008 | 0.012 | 0.18 | 41.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.18 | 31.7 |
| stream/spawn/ms | 0.006 | 0.000 | 1.18 | 30.7 |
| script/ui/ms | 0.006 | 0.008 | 0.15 | 29.2 |
| script/input/ms | 0.006 | 0.007 | 0.16 | 29.0 |
| controller/write/ms | 0.003 | 0.004 | 0.15 | 17.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 16.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.14 | 4.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 2.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 20 |
| controller/movers/count | count | 20 | 21 | 20 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 590 |
| crowd/c0/meshes/count | count | 353 | 646 | 171 |
| crowd/c1/count | count | 146 | 157 | 148 |
| crowd/c1/joints/count | count | 5531 | 6050 | 5606 |
| crowd/c1/meshes/count | count | 1165 | 1324 | 1170 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4253 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.72 | 6.58 | 3.22 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.27 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.41 | 0.02 |
| encode/finish_critical | ms | 2.72 | 6.58 | 3.22 |
| encode/resolve_diagnostics/max | ms | 0.04 | 4.03 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.55 | 34.90 | 9.82 |
| gpu/timed/ms | ms | 8.79 | 13.64 | 8.96 |
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
| phases/alpha_mask3d/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5112 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5112 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 219 | 28 |
| phases/shadow/bins | count | 4764 | 5743 | 2221 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 13.2 k | 5676 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 198 | 246 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 752 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 12 | 0 |
| preprocess/bind_groups | count | 10 | 597 | 11 |
| preprocess/bind_groups_created | count | 10 | 597 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 180 | 273 | 168 |
| process/cpu_usage | % | 26.44 | 49.52 | 14.29 |
| process/mem_usage | GiB | 5.11 | 5.28 | 5.28 |
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
| started_load_count |  | 4768 | 4861 | 4861 |
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
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 40.09 | 72.32 | 34.98 |
| system/mem_usage | % | 61.79 | 62.60 | 62.50 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2592 | 2650 | 2650 |
| views/all/batch_sets | count | 85 | 266 | 78 |
| views/all/bins | count | 7328 | 8583 | 5719 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.5 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 242 | 292 | 74 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 17.2 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2598 | 3140 | 2221 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8144 | 10.2 k | 5676 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8144 | 10.2 k | 5676 |
| views/lamp_face/batch_sets | count | 0 | 184 | 0 |
| views/lamp_face/bins | count | 2114 | 2734 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2372 | 3023 | 0 |
| views/lamp_face/pipelines | count | 162 | 203 | 0 |
| views/lamp_face/work_items | count | 0 | 2802 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2878 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8406 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8572 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 58.49 | 245 | 300.2 k |
| anim/events/count | count | 0.11 | 3 | 583 |
| assets/Image/added | count | 0.12 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.79 | 152 | 4075 |
| assets/Mesh/removed | count | 0.95 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.22 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.91 | 1245 | 4676 |
| encode//count | count | 36.83 | 272 | 189.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5133 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5133 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5133 |
| physics/inserts | count | 0.58 | 164 | 2978 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 145 | 2335 |
| physics/steps | count | 0.77 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 24 | 378 |
| stream/upload/bytes | bytes | 6595 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1446 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.2 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2561.2 | main/busy/ms 2532, stage/main/post_update 2431, stage/main/update 95.07, surface/advance_cpu_ms 67.20, stage/main/pre_update 2.65, stage/main/last 0.94, surface/cell_upload_ms 0.77, stage/main/first 0.57, stage/main/fixed_main_loop 0.41, stream/drive/ms 0.34, weather/map/ms 0.14, bigspace/total/ms 0.10 |
| warm-up | 3.34 | 781.7 | main/busy/ms 2570, stage/main/post_update 2459, render/thread/ms 740, render/busy/ms 617, stage/render/prepare_assets 463, stage/render/prepare_resources 124, stage/render/prepare_views 120, render/acquire/ms 120, stage/main/update 99.16, surface/advance_cpu_ms 67.20, stage/render/create_views 20.07, stage/render/render 8.92 |
| warm-up | 3.49 | 145.0 | pipelines/wait_ms 803, render/thread/ms 750, render/busy/ms 626, stage/render/prepare_assets 464, pipelines/compile_ms 145, pipelines/front_ms 138, stage/render/prepare_resources 126, stage/render/prepare_views 121, render/acquire/ms 121, main/busy/ms 52.56, stage/main/post_update 35.35, terrain/update/ms 21.17 |
| warm-up | 3.74 | 96.8 | pipelines/compile_ms 232, pipelines/front_ms 216, main/busy/ms 71.57, stage/main/update 59.27, render/thread/ms 58.55, render/busy/ms 56.51, stage/render/prepare_assets 46.04, pipelines/driver_ms 15.80, stage/main/post_update 9.23, stage/render/render 6.64, render/graph/ms 4.23, stage/render/graph_submit 2.62 |
| warm-up | 3.89 | 66.3 | main/busy/ms 41.17, render/thread/ms 34.62, render/busy/ms 33.19, stage/main/update 29.54, stage/render/prepare_assets 25.75, stream/drive/ms 25.59, stage/main/post_update 9.00, stage/render/render 4.15, render/graph/ms 2.55, encode//sum 1.45, stage/render/graph_submit 1.28, stage/render/camera_driver 1.17 |
| warm-up | 3.98 | 76.2 | pipelines/wait_ms 6204, pipelines/compile_ms 234, pipelines/front_ms 210, main/busy/ms 54.36, render/thread/ms 45.94, render/busy/ms 43.30, stage/main/update 34.80, stream/drive/ms 30.44, stage/render/prepare_assets 27.85, pipelines/driver_ms 24.11, stage/main/post_update 15.31, stage/render/render 10.46 |
| warm-up | 4.07 | 70.9 | pipelines/wait_ms 3608, pipelines/compile_ms 112, pipelines/front_ms 104, render/thread/ms 53.75, render/busy/ms 52.02, stage/render/prepare_assets 36.56, main/busy/ms 32.29, stage/main/update 13.58, stage/main/post_update 10.75, stream/drive/ms 9.50, pipelines/driver_ms 7.92, render/graph/ms 5.92 |
| warm-up | 4.11 | 39.2 | pipelines/wait_ms 2903, pipelines/compile_ms 77.08, pipelines/front_ms 71.10, render/thread/ms 70.46, render/busy/ms 68.79, stage/render/prepare_assets 54.63, main/busy/ms 48.52, stage/main/update 31.51, stream/drive/ms 27.26, stage/main/post_update 10.61, stage/render/render 7.80, pipelines/driver_ms 5.98 |
| warm-up | 4.15 | 43.7 | render/thread/ms 55.00, render/busy/ms 53.10, main/busy/ms 48.59, stage/render/prepare_assets 43.22, stage/main/update 35.28, stream/drive/ms 30.94, stage/main/post_update 6.68, stage/render/render 6.21, render/graph/ms 3.73, stream/spawn/ms 3.47, stage/render/graph_submit 2.54, stage/main/fixed_main_loop 2.17 |
| warm-up | 4.28 | 35.8 | pipelines/wait_ms 719, render/thread/ms 53.90, render/busy/ms 50.87, encode//sum 38.68, pipelines/compile_ms 31.12, pipelines/front_ms 30.35, main/busy/ms 26.09, stage/render/render 20.71, render/graph/ms 16.48, stage/render/prepare_assets 13.62, stage/main/update 11.94, stage/render/graph_submit 10.58 |
| warm-up | 4.32 | 40.9 | pipelines/wait_ms 719, main/busy/ms 71.14, stage/main/update 56.39, render/thread/ms 54.16, stream/drive/ms 52.74, render/busy/ms 51.67, encode//sum 31.23, pipelines/compile_ms 31.12, pipelines/front_ms 30.35, stage/render/prepare_assets 19.48, stage/render/render 18.33, render/graph/ms 14.91 |
| warm-up | 4.43 | 107.1 | pipelines/wait_ms 3795, encode//sum 448, pipelines/compile_ms 286, pipelines/front_ms 157, pipelines/driver_ms 128, render/thread/ms 89.25, render/busy/ms 87.36, main/busy/ms 69.76, stage/main/update 55.72, stream/drive/ms 51.55, stage/render/render 44.82, render/graph/ms 42.45 |
| warm-up | 4.52 | 65.6 | pipelines/wait_ms 17.0 k, pipelines/compile_ms 103, pipelines/front_ms 82.55, main/busy/ms 72.52, stage/main/update 54.93, stream/drive/ms 49.25, render/thread/ms 40.78, render/busy/ms 39.27, stage/render/prepare_assets 23.49, pipelines/driver_ms 20.10, stage/main/post_update 12.35, render/graph/ms 6.15 |
| warm-up | 4.60 | 61.6 | pipelines/wait_ms 1011, main/busy/ms 68.40, stage/main/update 57.29, stream/drive/ms 36.75, render/thread/ms 34.76, render/busy/ms 33.47, stage/render/prepare_assets 18.28, pipelines/compile_ms 13.19, pipelines/front_ms 12.43, stage/render/render 5.99, stage/main/post_update 5.80, encode//sum 4.76 |
| warm-up | 4.65 | 37.1 | main/busy/ms 41.25, stage/main/update 28.54, stream/drive/ms 24.17, render/thread/ms 22.11, render/busy/ms 20.99, stage/main/post_update 8.82, stage/render/prepare_assets 7.11, stage/render/render 6.28, render/graph/ms 4.82, encode//sum 3.89, stage/render/graph_submit 3.47, encode/submit 2.71 |
| warm-up | 7.38 | 38.0 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 154, pipelines/front_ms 148, main/busy/ms 57.00, render/thread/ms 45.17, render/busy/ms 43.34, stage/main/update 43.03, encode//sum 24.49, stage/render/render 19.28, render/graph/ms 16.75, stage/render/graph_submit 12.82, stage/render/prepare_assets 12.20 |
| warm-up | 7.43 | 57.8 | pipelines/wait_ms 9603, main/busy/ms 75.56, render/thread/ms 67.84, render/busy/ms 65.97, pipelines/compile_ms 56.18, stage/main/update 55.26, pipelines/front_ms 53.82, stage/render/prepare_assets 31.15, encode//sum 21.99, stage/render/render 16.07, stage/main/post_update 15.06, render/graph/ms 13.58 |
| warm-up | 7.48 | 45.1 | pipelines/wait_ms 3133, render/thread/ms 69.75, render/busy/ms 68.26, main/busy/ms 61.47, stage/main/update 38.91, stage/render/prepare_assets 37.86, encode//sum 17.76, pipelines/compile_ms 17.30, stage/main/post_update 17.04, pipelines/front_ms 16.96, stage/render/render 11.99, render/graph/ms 10.00 |
| warm-up | 7.52 | 44.6 | render/thread/ms 66.69, render/busy/ms 65.28, main/busy/ms 43.43, stage/render/prepare_assets 36.58, stage/main/update 22.36, encode//sum 17.67, stage/main/post_update 14.57, stage/render/render 11.93, render/graph/ms 9.76, stage/render/graph_submit 7.68, stage/render/specialize 4.22, encode/submit 3.88 |
| warm-up | 7.68 | 35.3 | main/busy/ms 54.51, render/thread/ms 49.00, render/busy/ms 47.62, stage/main/update 35.27, encode//sum 22.17, stage/render/render 17.71, render/graph/ms 15.79, stage/main/post_update 15.08, stage/render/prepare_assets 13.45, stage/render/graph_submit 11.81, encode/finish_wall 7.19, stage/render/prepare_write_phase_buffers 5.33 |
| warm-up | 7.72 | 45.0 | main/busy/ms 68.41, render/thread/ms 56.45, render/busy/ms 54.99, stage/main/update 50.76, stage/render/prepare_assets 24.58, encode//sum 20.05, stage/render/render 13.60, stage/main/post_update 12.76, render/graph/ms 11.61, stage/render/graph_submit 8.54, encode/finish_wall 5.13, stage/render/prepare_write_phase_buffers 4.50 |
| measure | 0.82 | 41.4 | pipelines/wait_ms 2542, pipelines/compile_ms 163, pipelines/front_ms 158, main/busy/ms 52.79, render/thread/ms 39.99, render/busy/ms 38.41, encode//sum 31.21, stage/main/update 30.75, stream/drive/ms 21.71, stage/render/render 21.03, render/graph/ms 18.94, stage/main/post_update 16.87 |
| measure | 2.17 | 32.0 | encode//sum 95.87, render/thread/ms 43.25, render/busy/ms 41.73, stage/render/cleanup 28.77, render/graph/ms 26.68, main/busy/ms 21.17, stage/render/graph_submit 20.02, pipelines/compile_ms 15.28, pipelines/front_ms 15.01, stage/main/post_update 12.33, encode/finish_wall 11.37, encode/submit 8.71 |
| measure | 2.72 | 33.8 | encode//sum 69.25, render/thread/ms 41.56, render/busy/ms 40.02, main/busy/ms 25.73, stage/render/render 22.48, render/graph/ms 20.45, stage/render/graph_submit 14.97, stage/main/post_update 10.54, stage/main/pre_update 8.35, stage/render/create_views 7.87, encode/finish_wall 7.58, encode/submit 7.48 |
| measure | 18.19 | 31.8 | main/busy/ms 39.52, encode//sum 27.89, stage/main/update 25.37, render/thread/ms 23.42, render/busy/ms 22.32, stream/drive/ms 21.35, stage/render/cleanup 14.41, render/graph/ms 12.88, stage/main/post_update 11.07, stage/render/graph_submit 9.95, encode/submit 5.03, encode/finish_wall 4.99 |
| measure | 59.07 | 33.6 | render/thread/ms 44.02, render/busy/ms 42.74, main/busy/ms 28.75, encode//sum 23.25, stage/render/prepare_assets 17.40, stage/main/post_update 14.75, stage/render/render 14.43, render/graph/ms 12.95, stage/render/graph_submit 10.02, stage/main/update 8.15, encode/finish_wall 5.59, encode/submit 4.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\thr12\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_COMPUTE_THREADS=12
- before the run: GPU 0%, CPU 14%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **82%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.0 W · 74 °C (max 78) · VRAM 7132 MiB · throttled 100% of samples
