# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4742 frames over 65.7 s · **median 13.26 ms (75 fps)** · mean 13.84 · p95 17.77 · p99 23.32 (1% low 43 fps) · worst 80.34 ms · 108 spikes over 1.5× median · 60.3 stutters/min (work 63.1/min)
- warm-up: 403 frames over 9.3 s, worst 2584.4 ms
- hitches (> 33.1 ms): **11** measured, 18 in warm-up
- timed GPU passes: 7.89 ms of a 13.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.26 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.09 min = 60.32/min, limit 1/min
- **FAIL** worst frame 80.34 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.096 | 2.136 | 2.554 | 0.98 | 0.187 |
| main_opaque_pass_3d | 1.752 | 1.786 | 2.468 | 0.98 | 0.058 |
| early prepass | 0.526 | 0.536 | 0.763 | 0.98 | 0.080 |
| ssao | 0.332 | 0.338 | 0.340 | 0.98 | 0.008 |
| bloom | 0.324 | 0.330 | 0.331 | 0.98 | 0.068 |
| atmosphere_luts | 0.314 | 0.320 | 0.382 | 0.98 | 0.003 |
| water_velocity | 0.285 | 0.290 | 0.355 | 0.98 | 0.002 |
| auxiliary_views | 0.272 | 0.277 | 0.820 | 0.98 | 0.193 |
| water_sspr | 0.211 | 0.215 | 0.213 | 0.98 | 0.002 |
| uniform_allocation | 0.206 | 0.209 | 0.218 | 0.98 | 0.036 |
| water_fft | 0.173 | 0.177 | 0.177 | 0.98 | 0.003 |
| bin_unpacking | 0.161 | 0.164 | 0.168 | 0.98 | 0.026 |
| atmos_froxel_inject | 0.155 | 0.158 | 0.156 | 0.98 | 0.002 |
| clouds_trace | 0.139 | 0.142 | 0.232 | 0.98 | 0.001 |
| upscaling | 0.120 | 0.123 | 0.130 | 0.98 | 0.004 |
| clouds_composite | 0.107 | 0.109 | 0.111 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.089 | 0.091 | 0.172 | 0.98 | 0.009 |
| tonemapping | 0.087 | 0.089 | 0.091 | 0.98 | 0.004 |
| clustering | 0.067 | 0.069 | 0.070 | 0.98 | 0.021 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.445 | 0.697 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.052 | 0.056 | 0.98 | 0.006 |
| surface_trail | 0.049 | 0.059 | 0.059 | 0.83 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.98 | 0.001 |
| water_snapshot | 0.034 | 0.035 | 0.037 | 0.98 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.028 | 0.029 | 0.030 | 0.98 | 0.007 |
| lightprobe_radiance_map | 0.027 | 0.227 | 0.235 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.028 | 0.98 | 0.001 |
| water_mips | 0.022 | 0.023 | 0.023 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.033 | 0.98 | 0.113 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.017 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.450 | 19.051 | 68.84 | 59036.0 |
| render/thread/ms | 12.322 | 15.944 | 77.57 | 58429.3 |
| render/busy/ms | 11.673 | 15.129 | 76.97 | 55355.1 |
| main/busy/ms | 10.139 | 13.439 | 36.54 | 48079.8 |
| render/graph/ms | 7.260 | 9.593 | 20.69 | 34427.0 |
| stage/main/post_update | 6.498 | 9.306 | 27.53 | 30815.5 |
| stage/render/graph_submit | 5.963 | 7.924 | 17.72 | 28274.9 |
| pipelines/wait_ms | 4.863 | 0.000 | 4945.94 | 23058.4 |
| stage/render/render | 4.570 | 9.936 | 22.59 | 21668.8 |
| stage/render/cleanup | 3.524 | 9.727 | 36.69 | 16709.1 |
| encode/finish_wall | 3.517 | 4.844 | 12.41 | 16677.6 |
| encode/submit | 2.501 | 3.529 | 9.73 | 11859.7 |
| stage/main/update | 2.249 | 2.840 | 22.72 | 10663.8 |
| stage/render/camera_driver | 1.235 | 1.765 | 10.48 | 5856.2 |
| stage/render/prepare_write_phase_buffers | 1.080 | 1.440 | 6.65 | 5121.0 |
| stage/render/prepare_assets | 0.596 | 0.831 | 11.16 | 2826.3 |
| bigspace/total/ms | 0.518 | 1.035 | 7.55 | 2454.3 |
| render/present/ms | 0.498 | 0.661 | 2.67 | 2362.4 |
| stage/render/prepare_resources | 0.460 | 0.585 | 3.06 | 2182.0 |
| stage/main/first | 0.448 | 0.613 | 10.10 | 2123.3 |
| stage/render/prepare_meshes | 0.432 | 0.598 | 1.96 | 2047.7 |
| stage/main/pre_update | 0.352 | 0.538 | 3.42 | 1671.1 |
| stage/render/specialize | 0.349 | 0.558 | 3.88 | 1653.4 |
| stage/render/queue | 0.343 | 0.608 | 5.44 | 1625.3 |
| terrain/update/ms | 0.300 | 0.386 | 2.04 | 1420.7 |
| stage/render/prepare_bind_groups | 0.287 | 0.524 | 3.21 | 1359.0 |
| controller/feed/ms | 0.280 | 0.393 | 0.87 | 1326.4 |
| stage/main/fixed_main_loop | 0.278 | 0.447 | 1.59 | 1318.7 |
| terrain/select/ms | 0.265 | 0.339 | 1.46 | 1257.8 |
| stream/drive/ms | 0.265 | 0.388 | 20.47 | 1256.5 |
| stage/render/create_views | 0.220 | 0.270 | 45.96 | 1042.2 |
| bigspace/low_precision/ms | 0.207 | 0.434 | 3.75 | 983.8 |
| stream/tick/ms | 0.200 | 0.286 | 0.60 | 950.2 |
| pipelines/compile_ms | 0.200 | 0.000 | 172.83 | 946.3 |
| far/select/ms | 0.199 | 0.291 | 0.72 | 944.6 |
| stage/render/prepare_views | 0.195 | 0.203 | 20.82 | 926.4 |
| pipelines/front_ms | 0.186 | 0.000 | 134.76 | 881.9 |
| stage/render/prepare_batch_phases | 0.186 | 0.280 | 1.07 | 879.8 |
| stage/render/prepare_flush | 0.167 | 0.325 | 1.13 | 792.8 |
| stage/render/extract_commands | 0.162 | 0.206 | 0.52 | 769.6 |
| anim/evaluate_cpu/ms | 0.158 | 0.205 | 0.42 | 746.9 |
| render/acquire/ms | 0.150 | 0.143 | 20.76 | 711.8 |
| controller/move/ms | 0.147 | 0.287 | 0.61 | 697.9 |
| anim/evaluate/ms | 0.146 | 0.274 | 3.82 | 694.0 |
| bigspace/high_precision/ms | 0.142 | 0.277 | 4.68 | 672.1 |
| stage/main/last | 0.123 | 0.211 | 1.61 | 583.6 |
| stage/render/prepare_collect_phase_buffers | 0.067 | 0.140 | 0.37 | 317.2 |
| physics/step/ms | 0.061 | 0.098 | 0.62 | 290.5 |
| physics/insert/ms | 0.044 | 0.410 | 0.79 | 208.6 |
| encode/resolve_diagnostics/sum | 0.034 | 0.049 | 0.66 | 163.4 |
| stage/render/phase_sort | 0.031 | 0.055 | 0.31 | 145.2 |
| weather/map/ms | 0.025 | 0.034 | 0.26 | 119.6 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.39 | 73.7 |
| pipelines/driver_ms | 0.014 | 0.000 | 38.07 | 64.4 |
| anim/apply/ms | 0.012 | 0.021 | 0.14 | 55.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.25 | 54.2 |
| controller/avoid/ms | 0.011 | 0.015 | 0.09 | 52.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 51.3 |
| script/update/ms | 0.008 | 0.012 | 0.20 | 36.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 29.4 |
| stream/spawn/ms | 0.006 | 0.000 | 1.09 | 26.7 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 25.7 |
| script/ui/ms | 0.005 | 0.008 | 0.05 | 25.7 |
| controller/write/ms | 0.004 | 0.005 | 0.07 | 16.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 2.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.27 | 0.5 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 21 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1292 | 2262 | 593 |
| crowd/c0/meshes/count | count | 349 | 646 | 156 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5603 |
| crowd/c1/meshes/count | count | 1164 | 1268 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 242 | 286 | 273 |
| crowd/joints_extracted/count | count | 5294 | 11.0 k | 7309 |
| crowd/joints_uploaded/count | count | 5265 | 7784 | 7309 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.91 | 7.50 | 2.90 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.39 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.25 | 0.01 |
| encode/finish_critical | ms | 2.91 | 7.50 | 2.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.66 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 11.35 | 76.97 | 10.84 |
| gpu/timed/ms | ms | 8.54 | 24.72 | 9.91 |
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
| phases/alpha_mask3d/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 22 | 20 |
| phases/opaque3d/bins | count | 1289 | 1397 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3508 | 5294 | 3199 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 21 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1397 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3508 | 5294 | 3199 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 51 | 333 | 38 |
| phases/shadow/bins | count | 5892 | 8303 | 3199 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 14.2 k | 18.2 k | 8611 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 214 | 359 | 38 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 180 | 182 | 182 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 13 | 852 | 7 |
| preprocess/bind_groups_created | count | 13 | 852 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 207 | 294 | 193 |
| process/cpu_usage | % | 24.52 | 41.35 | 41.35 |
| process/mem_usage | GiB | 5.46 | 5.71 | 5.71 |
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
| started_load_count |  | 4767 | 4860 | 4860 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.33 | 81.62 | 45.37 |
| system/mem_usage | % | 64.75 | 66.54 | 66.31 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2840 | 2956 | 2956 |
| textures/images | count | 2590 | 2650 | 2650 |
| views/all/batch_sets | count | 100 | 381 | 88 |
| views/all/bins | count | 8617 | 11.3 k | 6697 |
| views/all/count | count | 47 | 79 | 5 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 23.6 k | 30.6 k | 16.2 k |
| views/all/pipelines | count | 260 | 403 | 84 |
| views/all/pipelines_distinct | count | 69 | 75 | 61 |
| views/all/work_items | count | 21.5 k | 30.2 k | 16.6 k |
| views/cascade/batch_sets | count | 47 | 58 | 38 |
| views/cascade/bins | count | 3710 | 4417 | 3199 |
| views/cascade/count | count | 4 | 6 | 4 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 11.7 k | 13.9 k | 8611 |
| views/cascade/pipelines | count | 47 | 58 | 38 |
| views/cascade/work_items | count | 11.7 k | 13.9 k | 8611 |
| views/lamp_face/batch_sets | count | 0 | 285 | 0 |
| views/lamp_face/bins | count | 2192 | 4004 | 0 |
| views/lamp_face/count | count | 42 | 72 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2430 | 4398 | 0 |
| views/lamp_face/pipelines | count | 167 | 301 | 0 |
| views/lamp_face/work_items | count | 0 | 4391 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2881 | 3564 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8635 | 13.3 k | 7632 |
| views/main/pipelines | count | 46 | 50 | 46 |
| views/main/work_items | count | 8796 | 13.5 k | 7940 |
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
| anim/bones_written/count | count | 57.48 | 239 | 272.6 k |
| anim/events/count | count | 0.11 | 4 | 513 |
| assets/Image/added | count | 0.13 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.86 | 152 | 4075 |
| assets/Mesh/removed | count | 1.03 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.56 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.09 | 1328 | 5172 |
| encode//count | count | 42.52 | 392 | 201.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 4742 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 4742 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 4742 |
| physics/inserts | count | 0.51 | 111 | 2410 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.49 | 129 | 2335 |
| physics/steps | count | 0.83 | 3 | 3937 |
| physics/steps_dropped | count | 0.00 | 2 | 2 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 7139 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1565 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.1 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2584.4 | main/busy/ms 2556, stage/main/post_update 2459, stage/main/update 90.75, surface/advance_cpu_ms 64.84, stage/main/pre_update 2.81, stage/main/last 1.13, surface/cell_upload_ms 0.77, stage/main/first 0.58, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.27, weather/map/ms 0.09, bigspace/total/ms 0.08 |
| warm-up | 3.32 | 738.1 | main/busy/ms 2589, stage/main/post_update 2482, render/thread/ms 693, render/busy/ms 643, stage/render/prepare_assets 445, stage/render/prepare_resources 133, stage/main/update 94.86, surface/advance_cpu_ms 64.84, stage/render/prepare_views 46.81, render/acquire/ms 46.58, stage/render/render 45.26, render/graph/ms 41.31 |
| warm-up | 3.44 | 117.5 | render/thread/ms 707, render/busy/ms 656, stage/render/prepare_assets 447, pipelines/wait_ms 429, pipelines/compile_ms 138, stage/render/prepare_resources 138, pipelines/front_ms 130, main/busy/ms 53.56, stage/render/render 49.38, stage/render/prepare_views 47.14, render/acquire/ms 46.84, render/graph/ms 43.46 |
| warm-up | 3.71 | 90.4 | pipelines/compile_ms 274, pipelines/front_ms 259, render/thread/ms 83.84, main/busy/ms 69.78, render/busy/ms 64.35, stage/main/update 57.60, stage/render/prepare_assets 52.69, stage/render/prepare_views 17.88, render/acquire/ms 17.71, pipelines/driver_ms 15.30, stage/main/post_update 7.81, render/graph/ms 5.60 |
| warm-up | 3.87 | 57.7 | render/thread/ms 54.10, render/busy/ms 52.82, main/busy/ms 48.55, stage/render/prepare_assets 42.98, stage/main/update 37.76, stream/drive/ms 33.66, stage/main/post_update 6.52, stage/render/render 5.96, render/graph/ms 4.29, stage/render/graph_submit 2.58, stage/main/pre_update 1.90, encode/submit 1.79 |
| warm-up | 3.93 | 33.9 | pipelines/wait_ms 300, pipelines/compile_ms 190, pipelines/front_ms 173, render/thread/ms 43.66, render/busy/ms 41.65, main/busy/ms 35.66, stage/render/prepare_assets 27.75, stage/main/update 24.22, stream/drive/ms 19.64, pipelines/driver_ms 17.31, stage/main/post_update 6.35, render/graph/ms 5.41 |
| warm-up | 4.04 | 80.2 | pipelines/wait_ms 2171, pipelines/compile_ms 125, pipelines/front_ms 119, render/thread/ms 75.50, render/busy/ms 73.94, stage/render/prepare_assets 62.07, main/busy/ms 43.62, stage/main/update 25.99, stream/drive/ms 20.02, stage/main/post_update 8.73, stage/render/render 6.56, pipelines/driver_ms 6.30 |
| warm-up | 4.07 | 35.9 | pipelines/wait_ms 6295, pipelines/compile_ms 169, pipelines/front_ms 157, render/thread/ms 81.50, render/busy/ms 79.96, main/busy/ms 69.66, stage/render/prepare_assets 66.49, stage/main/update 42.89, stream/drive/ms 36.94, stage/main/post_update 17.94, pipelines/driver_ms 11.77, stage/render/render 6.29 |
| warm-up | 4.21 | 132.3 | pipelines/wait_ms 4738, render/thread/ms 108, main/busy/ms 74.35, pipelines/compile_ms 73.25, render/busy/ms 70.06, pipelines/front_ms 66.00, stage/render/prepare_assets 56.21, stage/main/update 42.22, stage/render/cleanup 39.04, render/present/ms 37.96, stream/drive/ms 36.03, stage/main/post_update 20.41 |
| warm-up | 4.24 | 35.5 | pipelines/wait_ms 2530, pipelines/compile_ms 191, pipelines/front_ms 169, render/thread/ms 114, render/busy/ms 74.97, stage/render/prepare_assets 44.30, stage/render/cleanup 39.03, render/present/ms 39.00, main/busy/ms 38.98, pipelines/driver_ms 21.37, stage/main/post_update 16.99, stage/render/render 11.48 |
| warm-up | 4.33 | 47.0 | pipelines/wait_ms 4389, pipelines/compile_ms 107, pipelines/front_ms 73.69, encode//sum 71.61, render/thread/ms 67.47, render/busy/ms 64.68, pipelines/driver_ms 33.33, stage/render/render 26.43, render/graph/ms 22.58, main/busy/ms 19.11, stage/render/graph_submit 16.80, stage/render/create_views 12.53 |
| warm-up | 4.41 | 54.2 | pipelines/wait_ms 2622, main/busy/ms 72.21, stage/main/update 54.91, render/thread/ms 50.31, stream/drive/ms 49.77, render/busy/ms 48.63, stage/render/prepare_assets 29.17, pipelines/compile_ms 28.11, pipelines/front_ms 26.17, stage/main/post_update 9.04, stage/render/render 7.37, render/graph/ms 5.05 |
| warm-up | 4.49 | 79.5 | pipelines/wait_ms 996, render/thread/ms 76.77, main/busy/ms 73.46, stage/main/update 54.28, render/busy/ms 50.93, stream/drive/ms 49.19, stage/render/prepare_assets 37.43, stage/render/cleanup 27.06, pipelines/compile_ms 26.22, render/present/ms 25.64, pipelines/driver_ms 23.08, stage/main/post_update 12.67 |
| warm-up | 4.58 | 66.8 | pipelines/wait_ms 18.8 k, pipelines/compile_ms 281, pipelines/front_ms 221, main/busy/ms 64.89, pipelines/driver_ms 60.04, stage/main/update 51.24, stream/drive/ms 46.93, render/thread/ms 40.85, render/busy/ms 39.56, stage/render/prepare_assets 21.79, stage/main/post_update 6.59, render/graph/ms 4.72 |
| warm-up | 4.65 | 51.4 | pipelines/wait_ms 1961, main/busy/ms 49.69, stage/main/update 38.55, stream/drive/ms 34.60, pipelines/compile_ms 33.53, pipelines/front_ms 32.01, render/thread/ms 29.69, render/busy/ms 28.54, stage/render/prepare_assets 11.69, stage/main/post_update 6.77, render/graph/ms 4.98, encode//sum 4.15 |
| warm-up | 4.69 | 37.0 | pipelines/wait_ms 3610, pipelines/compile_ms 71.23, pipelines/front_ms 68.01, main/busy/ms 38.25, stage/main/update 28.84, stream/drive/ms 24.00, render/thread/ms 20.53, render/busy/ms 18.69, render/graph/ms 6.19, stage/main/post_update 6.05, stage/render/prepare_assets 5.07, stage/render/graph_submit 4.90 |
| warm-up | 4.93 | 34.8 | render/thread/ms 34.48, render/busy/ms 33.19, main/busy/ms 26.27, stage/render/prepare_assets 20.37, stage/main/update 14.08, stream/drive/ms 10.44, stage/main/post_update 7.97, render/graph/ms 4.50, encode//sum 4.05, stage/render/render 3.26, stage/render/graph_submit 3.18, stage/render/cleanup 3.14 |
| warm-up | 5.79 | 60.0 | main/busy/ms 63.25, render/thread/ms 59.14, render/busy/ms 57.19, stage/main/update 41.15, stage/render/prepare_assets 23.13, stage/main/post_update 13.52, stage/render/render 13.00, encode//sum 11.90, render/graph/ms 10.56, stage/render/graph_submit 8.08, stream/drive/ms 6.64, encode/submit 5.52 |
| measure | 2.73 | 36.0 | encode//sum 59.87, render/thread/ms 47.09, render/busy/ms 45.58, stage/render/cleanup 25.90, render/graph/ms 23.66, main/busy/ms 19.75, stage/render/graph_submit 17.67, stage/main/post_update 11.98, encode/submit 9.26, encode/finish_wall 8.48, stage/render/create_views 8.14, stage/render/camera_driver 5.85 |
| measure | 8.13 | 35.3 | encode//sum 65.75, render/thread/ms 45.03, render/busy/ms 43.88, stage/render/render 26.18, render/graph/ms 24.37, stage/render/graph_submit 18.38, main/busy/ms 16.72, stage/main/post_update 10.13, encode/finish_wall 9.61, encode/submit 8.81, stage/render/create_views 7.88, stage/render/camera_driver 5.85 |
| measure | 9.30 | 34.4 | encode//sum 55.60, render/thread/ms 45.13, render/busy/ms 43.96, stage/render/render 24.64, render/graph/ms 22.97, main/busy/ms 20.24, stage/render/graph_submit 17.70, stage/main/post_update 13.46, encode/submit 10.13, stage/render/create_views 8.39, encode/finish_wall 7.64, stage/render/camera_driver 5.13 |
| measure | 9.39 | 35.3 | encode//sum 66.41, render/thread/ms 47.11, render/busy/ms 46.06, render/graph/ms 25.42, main/busy/ms 19.94, stage/render/graph_submit 19.56, stage/render/cleanup 17.85, stage/main/post_update 13.33, encode/finish_wall 10.21, encode/submit 9.39, stage/render/render 9.15, stage/render/create_views 7.99 |
| measure | 12.01 | 52.6 | encode//sum 73.09, render/thread/ms 62.53, render/busy/ms 61.18, stage/render/render 27.89, render/graph/ms 25.80, stage/render/create_views 21.23, stage/render/graph_submit 20.06, main/busy/ms 18.60, stage/main/post_update 11.64, encode/finish_wall 10.15, encode/submit 9.96, stage/render/camera_driver 5.61 |
| measure | 15.23 | 33.8 | render/thread/ms 52.34, stage/render/prepare_views 29.01, render/acquire/ms 28.88, encode//sum 25.68, render/busy/ms 22.47, main/busy/ms 17.20, render/graph/ms 13.45, stage/render/graph_submit 10.73, stage/main/post_update 10.54, stage/render/cleanup 7.86, stage/render/render 7.22, encode/finish_wall 5.90 |
| measure | 18.82 | 80.3 | render/thread/ms 92.57, render/busy/ms 91.42, encode//sum 88.18, stage/render/create_views 46.18, render/graph/ms 29.06, main/busy/ms 23.81, stage/render/graph_submit 22.81, stage/render/render 20.69, stage/main/post_update 15.10, encode/submit 11.57, encode/finish_wall 11.30, stage/render/cleanup 10.25 |
| measure | 18.87 | 52.9 | render/thread/ms 92.00, render/busy/ms 90.89, encode//sum 83.64, stage/render/create_views 46.19, stage/render/cleanup 36.69, stage/render/render 28.72, render/graph/ms 26.84, main/busy/ms 24.26, stage/render/graph_submit 20.34, stage/main/post_update 14.93, encode/finish_wall 10.87, encode/submit 9.50 |
| measure | 33.53 | 38.6 | render/thread/ms 64.76, render/busy/ms 61.10, encode//sum 48.54, main/busy/ms 35.90, stage/render/render 29.03, stage/main/post_update 25.67, render/graph/ms 24.58, stage/render/graph_submit 20.26, encode/finish_wall 10.87, encode/submit 9.62, stage/render/prepare_write_phase_buffers 9.01, stage/render/queue 8.98 |
| measure | 33.57 | 38.4 | render/thread/ms 69.03, render/busy/ms 65.47, main/busy/ms 56.44, encode//sum 52.91, stage/main/post_update 42.50, stage/render/render 38.99, render/graph/ms 34.36, stage/render/graph_submit 28.98, encode/finish_wall 18.00, encode/submit 11.13, stage/render/prepare_write_phase_buffers 9.86, stage/main/update 7.74 |
| measure | 59.05 | 37.5 | render/thread/ms 46.65, render/busy/ms 44.33, main/busy/ms 38.52, encode//sum 34.85, stage/render/render 25.76, render/graph/ms 22.81, stage/main/post_update 21.94, stage/render/graph_submit 18.95, encode/finish_wall 10.71, stage/render/prepare_assets 10.59, stage/main/update 10.22, encode/submit 8.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ultra2\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_QUALITY=ultra
- before the run: GPU 0%, CPU 7%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **76%** (min 51%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.0 W · 74 °C (max 79) · VRAM 7772 MiB · throttled 100% of samples
