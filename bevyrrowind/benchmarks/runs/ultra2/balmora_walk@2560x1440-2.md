# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4972 frames over 65.6 s · **median 12.59 ms (79 fps)** · mean 13.20 · p95 16.66 · p99 20.00 (1% low 50 fps) · worst 100.81 ms · 73 spikes over 1.5× median · 38.4 stutters/min (work 41.1/min)
- warm-up: 353 frames over 8.8 s, worst 2769.9 ms
- hitches (> 31.5 ms): **10** measured, 23 in warm-up
- timed GPU passes: 7.68 ms of a 13.20 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.59 ms, limit 11 ms
- **FAIL** frame_ms: 42 stutters (> 1.5x moving median of 60) in 1.09 min = 38.39/min, limit 1/min
- **FAIL** worst frame 100.81 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.025 | 2.084 | 2.245 | 0.97 | 0.186 |
| main_opaque_pass_3d | 1.698 | 1.748 | 2.117 | 0.97 | 0.056 |
| early prepass | 0.507 | 0.522 | 0.735 | 0.97 | 0.075 |
| ssao | 0.322 | 0.332 | 0.332 | 0.97 | 0.007 |
| bloom | 0.313 | 0.322 | 0.327 | 0.97 | 0.064 |
| atmosphere_luts | 0.307 | 0.315 | 0.374 | 0.97 | 0.003 |
| auxiliary_views | 0.288 | 0.296 | 0.742 | 0.97 | 0.193 |
| water_velocity | 0.277 | 0.285 | 0.354 | 0.97 | 0.002 |
| water_sspr | 0.205 | 0.211 | 0.212 | 0.97 | 0.002 |
| uniform_allocation | 0.204 | 0.209 | 0.221 | 0.97 | 0.035 |
| bin_unpacking | 0.166 | 0.171 | 0.467 | 0.97 | 0.026 |
| water_fft | 0.166 | 0.171 | 0.174 | 0.97 | 0.003 |
| atmos_froxel_inject | 0.150 | 0.154 | 0.156 | 0.97 | 0.001 |
| clouds_trace | 0.137 | 0.141 | 0.231 | 0.97 | 0.001 |
| upscaling | 0.116 | 0.119 | 0.127 | 0.97 | 0.004 |
| clouds_composite | 0.106 | 0.109 | 0.111 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.085 | 0.088 | 0.169 | 0.97 | 0.008 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.97 | 0.004 |
| clustering | 0.067 | 0.069 | 0.070 | 0.97 | 0.019 |
| early_mesh_preprocessing | 0.054 | 0.055 | 0.056 | 0.97 | 0.005 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.048 | 0.438 | 0.741 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| surface_trail | 0.046 | 0.058 | 0.058 | 0.79 | 0.002 |
| water_snapshot | 0.033 | 0.034 | 0.036 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.026 | 0.027 | 0.030 | 0.97 | 0.007 |
| atmos_froxel_integrate | 0.025 | 0.026 | 0.028 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.229 | 0.234 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.032 | 0.97 | 0.106 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.298 | 18.033 | 84.09 | 61145.2 |
| render/thread/ms | 11.763 | 14.914 | 98.82 | 58485.5 |
| render/busy/ms | 11.134 | 14.241 | 78.47 | 55358.8 |
| main/busy/ms | 9.719 | 12.772 | 28.98 | 48323.2 |
| pipelines/wait_ms | 8.480 | 0.000 | 4761.01 | 42162.0 |
| render/graph/ms | 6.884 | 8.964 | 40.19 | 34227.3 |
| stage/main/post_update | 6.271 | 8.920 | 12.20 | 31179.6 |
| stage/render/graph_submit | 5.637 | 7.351 | 17.42 | 28026.2 |
| stage/render/render | 4.604 | 9.464 | 21.36 | 22890.3 |
| encode/finish_wall | 3.289 | 4.421 | 8.93 | 16350.9 |
| stage/render/cleanup | 3.095 | 9.180 | 41.27 | 15386.3 |
| encode/submit | 2.419 | 3.282 | 9.39 | 12025.7 |
| stage/main/update | 2.125 | 2.621 | 22.50 | 10564.2 |
| stage/render/camera_driver | 1.187 | 1.660 | 34.02 | 5904.0 |
| stage/render/prepare_write_phase_buffers | 1.067 | 1.403 | 4.14 | 5303.0 |
| stage/render/prepare_assets | 0.572 | 0.734 | 12.76 | 2846.0 |
| render/present/ms | 0.496 | 0.639 | 6.77 | 2463.8 |
| bigspace/total/ms | 0.478 | 0.866 | 5.38 | 2375.9 |
| stage/render/prepare_resources | 0.443 | 0.540 | 2.69 | 2202.2 |
| stage/main/first | 0.435 | 0.595 | 4.17 | 2164.4 |
| stage/render/prepare_meshes | 0.412 | 0.550 | 1.70 | 2050.0 |
| stage/main/pre_update | 0.335 | 0.510 | 1.59 | 1665.6 |
| stage/render/specialize | 0.330 | 0.540 | 3.40 | 1638.3 |
| stage/render/queue | 0.327 | 0.589 | 4.24 | 1624.9 |
| pipelines/compile_ms | 0.319 | 0.000 | 170.40 | 1585.2 |
| pipelines/front_ms | 0.308 | 0.000 | 163.64 | 1529.7 |
| terrain/update/ms | 0.295 | 0.374 | 2.28 | 1468.4 |
| stage/render/prepare_bind_groups | 0.275 | 0.505 | 3.45 | 1368.0 |
| terrain/select/ms | 0.263 | 0.333 | 2.23 | 1305.9 |
| controller/feed/ms | 0.260 | 0.343 | 0.65 | 1294.1 |
| stage/main/fixed_main_loop | 0.257 | 0.395 | 1.79 | 1276.2 |
| stream/drive/ms | 0.253 | 0.351 | 20.33 | 1259.1 |
| stage/render/create_views | 0.214 | 0.254 | 45.93 | 1065.6 |
| bigspace/low_precision/ms | 0.196 | 0.351 | 5.15 | 973.8 |
| stream/tick/ms | 0.192 | 0.268 | 0.71 | 954.8 |
| far/select/ms | 0.191 | 0.263 | 0.51 | 951.2 |
| stage/render/prepare_views | 0.175 | 0.182 | 73.61 | 870.9 |
| stage/render/prepare_batch_phases | 0.174 | 0.246 | 1.72 | 864.3 |
| anim/evaluate_cpu/ms | 0.163 | 0.213 | 0.45 | 811.7 |
| stage/render/prepare_flush | 0.162 | 0.304 | 2.35 | 807.8 |
| stage/render/extract_commands | 0.156 | 0.190 | 1.91 | 777.8 |
| anim/evaluate/ms | 0.134 | 0.259 | 2.67 | 666.8 |
| render/acquire/ms | 0.133 | 0.132 | 73.52 | 662.9 |
| bigspace/high_precision/ms | 0.124 | 0.245 | 3.60 | 617.7 |
| stage/main/last | 0.116 | 0.206 | 1.64 | 577.2 |
| controller/move/ms | 0.112 | 0.196 | 0.86 | 557.2 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.128 | 0.27 | 293.1 |
| physics/step/ms | 0.058 | 0.091 | 0.64 | 287.0 |
| physics/insert/ms | 0.055 | 0.446 | 0.89 | 273.9 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.48 | 159.9 |
| stage/render/phase_sort | 0.029 | 0.048 | 0.28 | 142.7 |
| weather/map/ms | 0.025 | 0.033 | 0.18 | 122.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.56 | 75.3 |
| anim/apply/ms | 0.012 | 0.024 | 0.30 | 61.8 |
| pipelines/driver_ms | 0.011 | 0.000 | 6.86 | 55.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.45 | 53.9 |
| controller/avoid/ms | 0.011 | 0.014 | 0.25 | 52.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 50.1 |
| script/update/ms | 0.007 | 0.010 | 0.04 | 34.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.21 | 27.9 |
| script/input/ms | 0.005 | 0.007 | 0.08 | 26.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 25.4 |
| script/ui/ms | 0.005 | 0.007 | 0.19 | 24.5 |
| controller/write/ms | 0.004 | 0.005 | 0.08 | 18.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 14.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.08 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 2.7 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 15 | 0 |
| anim/a2/count | count | 1 | 13 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 62 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 50 | 13 |
| crowd/c0/joints/count | count | 1269 | 2218 | 586 |
| crowd/c0/meshes/count | count | 349 | 633 | 147 |
| crowd/c1/count | count | 147 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5610 |
| crowd/c1/meshes/count | count | 1165 | 1268 | 1194 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 243 | 286 | 273 |
| crowd/joints_extracted/count | count | 5306 | 9421 | 7309 |
| crowd/joints_uploaded/count | count | 5279 | 7784 | 7309 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.74 | 8.08 | 3.16 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.56 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.02 |
| encode/finish_critical | ms | 2.74 | 8.08 | 3.16 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.05 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.76 | 78.47 | 11.82 |
| gpu/timed/ms | ms | 8.52 | 26.54 | 8.61 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 552.74 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 799 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 22 | 20 |
| phases/opaque3d/bins | count | 1304 | 1397 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3458 | 5339 | 3199 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 21 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1304 | 1397 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3458 | 5339 | 3199 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 51 | 342 | 38 |
| phases/shadow/bins | count | 5868 | 8429 | 3199 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 14.0 k | 18.4 k | 8611 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 214 | 357 | 38 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 764 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 180 | 182 | 182 |
| pipelines/waiting | count | 0 | 11 | 0 |
| preprocess/bind_groups | count | 13 | 864 | 7 |
| preprocess/bind_groups_created | count | 13 | 864 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 208 | 308 | 193 |
| process/cpu_usage | % | 24.88 | 40.10 | 28.85 |
| process/mem_usage | GiB | 5.62 | 5.81 | 5.81 |
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
| started_load_count |  | 4769 | 4860 | 4860 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.37 | 54.14 | 32.00 |
| system/mem_usage | % | 66.91 | 67.73 | 67.73 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2841 | 2956 | 2956 |
| textures/images | count | 2592 | 2650 | 2650 |
| views/all/batch_sets | count | 100 | 390 | 88 |
| views/all/bins | count | 8475 | 11.3 k | 6697 |
| views/all/count | count | 47 | 79 | 5 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 23.4 k | 31.3 k | 16.2 k |
| views/all/pipelines | count | 259 | 403 | 84 |
| views/all/pipelines_distinct | count | 69 | 75 | 61 |
| views/all/work_items | count | 21.4 k | 30.3 k | 16.6 k |
| views/cascade/batch_sets | count | 47 | 57 | 38 |
| views/cascade/bins | count | 3702 | 4405 | 3199 |
| views/cascade/count | count | 4 | 6 | 4 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 11.6 k | 14.0 k | 8611 |
| views/cascade/pipelines | count | 47 | 57 | 38 |
| views/cascade/work_items | count | 11.6 k | 14.0 k | 8611 |
| views/lamp_face/batch_sets | count | 5 | 292 | 0 |
| views/lamp_face/bins | count | 2144 | 4115 | 0 |
| views/lamp_face/count | count | 42 | 72 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2391 | 4524 | 0 |
| views/lamp_face/pipelines | count | 168 | 303 | 0 |
| views/lamp_face/work_items | count | 62 | 4371 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2904 | 3564 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8516 | 13.4 k | 7632 |
| views/main/pipelines | count | 46 | 50 | 46 |
| views/main/work_items | count | 8686 | 13.6 k | 7940 |
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
| anim/bones_written/count | count | 68.78 | 643 | 342.0 k |
| anim/events/count | count | 0.11 | 2 | 562 |
| assets/Image/added | count | 0.13 | 43 | 652 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.85 | 168 | 4231 |
| assets/Mesh/removed | count | 0.98 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 528 |
| assets/StandardMaterial/modified | count | 4.35 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.87 | 3257 | 9304 |
| encode//count | count | 42.47 | 392 | 211.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 4972 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 4972 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 4972 |
| physics/inserts | count | 0.59 | 105 | 2945 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.79 | 3 | 3932 |
| physics/steps_dropped | count | 0.00 | 3 | 6 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 6809 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1493 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.5 ms; slowest 33, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.77 | 2769.9 | main/busy/ms 2742, stage/main/post_update 2644, stage/main/update 91.93, surface/advance_cpu_ms 64.79, stage/main/pre_update 2.52, stage/main/last 1.10, surface/cell_upload_ms 0.91, stage/main/fixed_main_loop 0.60, stage/main/first 0.52, stream/drive/ms 0.34, bigspace/total/ms 0.12, weather/map/ms 0.09 |
| warm-up | 3.49 | 719.2 | main/busy/ms 2779, stage/main/post_update 2672, render/thread/ms 676, render/busy/ms 627, stage/render/prepare_assets 447, stage/render/prepare_resources 124, stage/main/update 96.12, surface/advance_cpu_ms 64.79, stage/render/prepare_views 46.55, render/acquire/ms 46.37, stage/render/render 35.04, render/graph/ms 31.05 |
| warm-up | 3.61 | 124.1 | pipelines/wait_ms 913, render/thread/ms 686, render/busy/ms 636, stage/render/prepare_assets 448, pipelines/compile_ms 146, pipelines/front_ms 138, stage/render/prepare_resources 126, main/busy/ms 56.76, stage/render/prepare_views 46.89, render/acquire/ms 46.66, stage/render/render 38.41, stage/main/post_update 38.36 |
| warm-up | 3.86 | 79.8 | pipelines/compile_ms 250, pipelines/front_ms 239, main/busy/ms 66.03, stage/main/update 54.43, render/thread/ms 54.26, render/busy/ms 51.99, stage/render/prepare_assets 40.82, pipelines/driver_ms 11.15, stage/render/render 8.31, stage/main/post_update 8.15, render/graph/ms 5.39, stage/render/graph_submit 3.16 |
| warm-up | 4.04 | 77.6 | main/busy/ms 47.37, render/thread/ms 39.64, render/busy/ms 38.20, stage/main/update 35.89, stream/drive/ms 31.98, stage/render/prepare_assets 29.96, stage/main/post_update 8.32, render/graph/ms 2.98, stage/render/cleanup 2.79, stage/render/render 2.14, stage/render/graph_submit 1.65, encode//sum 1.52 |
| warm-up | 4.15 | 60.9 | pipelines/wait_ms 4687, pipelines/compile_ms 84.64, pipelines/front_ms 76.99, render/thread/ms 52.06, render/busy/ms 50.34, main/busy/ms 42.31, stage/render/prepare_assets 37.61, stage/main/update 28.96, stream/drive/ms 25.44, pipelines/driver_ms 7.65, stage/main/post_update 7.25, render/graph/ms 5.98 |
| warm-up | 4.19 | 43.9 | pipelines/wait_ms 4765, pipelines/compile_ms 78.16, render/thread/ms 73.76, render/busy/ms 72.04, pipelines/front_ms 71.25, stage/render/prepare_assets 59.64, main/busy/ms 39.29, stage/main/post_update 16.53, stage/main/update 16.44, stream/drive/ms 12.27, pipelines/driver_ms 6.90, render/graph/ms 5.80 |
| warm-up | 4.24 | 46.2 | pipelines/wait_ms 2167, pipelines/compile_ms 91.83, pipelines/front_ms 87.66, main/busy/ms 59.39, render/thread/ms 58.49, render/busy/ms 57.11, stage/render/prepare_assets 48.96, stage/main/update 31.42, stream/drive/ms 26.43, stage/main/post_update 20.91, terrain/update/ms 6.90, pipelines/driver_ms 4.17 |
| warm-up | 4.29 | 48.5 | pipelines/wait_ms 1665, pipelines/compile_ms 73.22, pipelines/front_ms 70.04, main/busy/ms 53.13, render/thread/ms 48.86, render/busy/ms 47.28, stage/main/update 32.78, stage/render/prepare_assets 32.70, stream/drive/ms 27.29, stage/main/post_update 13.24, terrain/update/ms 6.56, stage/render/render 4.75 |
| warm-up | 4.33 | 39.3 | render/thread/ms 51.39, render/busy/ms 49.84, stage/render/prepare_assets 30.48, main/busy/ms 27.29, stage/main/post_update 10.01, stage/main/update 9.82, stage/render/render 6.15, stream/drive/ms 4.85, encode//sum 4.48, render/graph/ms 4.21, stage/render/specialize 3.87, stage/render/prepare_write_phase_buffers 2.45 |
| warm-up | 4.39 | 36.2 | pipelines/wait_ms 3267, pipelines/compile_ms 182, pipelines/front_ms 167, render/thread/ms 56.49, render/busy/ms 54.42, encode//sum 49.38, main/busy/ms 21.49, stage/render/render 18.85, stage/render/prepare_assets 16.34, render/graph/ms 15.87, pipelines/driver_ms 15.32, stage/render/graph_submit 10.51 |
| warm-up | 4.44 | 46.6 | pipelines/wait_ms 6093, pipelines/compile_ms 224, pipelines/front_ms 206, render/thread/ms 73.31, encode//sum 72.86, render/busy/ms 70.92, stage/render/render 25.36, main/busy/ms 23.04, render/graph/ms 22.16, pipelines/driver_ms 18.54, stage/render/graph_submit 15.41, stage/render/create_views 14.20 |
| warm-up | 4.58 | 116.3 | pipelines/wait_ms 3804, render/thread/ms 73.71, main/busy/ms 70.63, stage/main/update 57.34, stream/drive/ms 53.41, render/busy/ms 42.27, stage/render/render 36.35, render/present/ms 31.22, pipelines/compile_ms 31.15, stage/render/prepare_assets 30.44, pipelines/front_ms 28.92, stage/main/post_update 7.35 |
| warm-up | 4.69 | 83.2 | pipelines/wait_ms 8138, pipelines/compile_ms 254, pipelines/front_ms 228, main/busy/ms 74.04, stage/main/update 57.71, stream/drive/ms 53.55, render/thread/ms 42.66, render/busy/ms 40.80, pipelines/driver_ms 26.08, stage/render/prepare_assets 17.64, stage/render/render 11.94, render/graph/ms 9.61 |
| warm-up | 4.77 | 64.6 | pipelines/wait_ms 5258, pipelines/compile_ms 79.82, pipelines/front_ms 72.23, main/busy/ms 55.77, stage/main/update 42.09, stream/drive/ms 38.33, render/thread/ms 35.76, render/busy/ms 34.39, stage/render/prepare_assets 16.97, stage/main/post_update 9.23, pipelines/driver_ms 7.59, stage/render/render 4.59 |
| warm-up | 4.81 | 32.9 | pipelines/wait_ms 6656, pipelines/compile_ms 51.86, pipelines/front_ms 48.66, main/busy/ms 36.14, stage/main/update 27.52, stream/drive/ms 24.28, render/thread/ms 16.91, render/busy/ms 15.70, stage/render/render 5.93, stage/main/post_update 4.91, render/graph/ms 4.36, pipelines/driver_ms 3.19 |
| warm-up | 4.87 | 31.6 | pipelines/wait_ms 5642, render/thread/ms 35.22, render/busy/ms 33.80, main/busy/ms 33.35, pipelines/compile_ms 31.85, pipelines/front_ms 29.62, stage/main/update 15.71, stage/main/post_update 13.36, stage/render/prepare_assets 11.69, stage/render/render 8.65, render/graph/ms 6.71, stream/drive/ms 5.49 |
| warm-up | 4.91 | 33.2 | pipelines/wait_ms 3238, main/busy/ms 52.92, render/thread/ms 50.45, render/busy/ms 48.47, stage/main/update 34.17, stage/render/prepare_assets 18.78, pipelines/compile_ms 14.31, stage/main/post_update 13.32, pipelines/front_ms 13.30, stage/render/render 11.88, render/graph/ms 9.15, stage/render/graph_submit 6.39 |
| warm-up | 4.95 | 44.8 | render/thread/ms 52.83, render/busy/ms 51.03, main/busy/ms 48.23, stage/main/update 32.29, stage/render/prepare_assets 26.70, stage/main/post_update 10.55, stage/render/render 10.11, render/graph/ms 7.49, encode//sum 6.34, stage/render/graph_submit 4.99, stream/drive/ms 4.92, stage/render/prepare_write_phase_buffers 3.90 |
| warm-up | 5.03 | 35.2 | render/thread/ms 40.77, render/busy/ms 39.55, main/busy/ms 26.26, stage/render/prepare_assets 20.89, stage/main/update 12.16, stage/main/post_update 9.68, encode//sum 4.86, render/graph/ms 4.84, stream/drive/ms 4.08, stage/render/render 3.39, stage/render/cleanup 3.21, stage/render/graph_submit 3.15 |
| warm-up | 7.01 | 33.0 | pipelines/wait_ms 8690, pipelines/compile_ms 52.97, main/busy/ms 49.87, pipelines/front_ms 49.55, render/thread/ms 41.20, render/busy/ms 39.17, stage/main/update 36.36, encode//sum 23.06, stage/render/render 18.86, render/graph/ms 15.92, stage/render/graph_submit 11.91, stage/render/prepare_assets 9.74 |
| warm-up | 7.05 | 44.7 | pipelines/wait_ms 6026, render/thread/ms 50.89, render/busy/ms 49.09, main/busy/ms 43.17, pipelines/compile_ms 38.42, pipelines/front_ms 35.84, stage/main/update 29.85, stage/render/prepare_assets 23.98, encode//sum 18.31, stage/render/render 13.99, render/graph/ms 11.40, stage/main/post_update 9.33 |
| warm-up | 7.56 | 37.9 | main/busy/ms 46.54, stage/main/update 31.44, render/thread/ms 29.42, render/busy/ms 28.16, encode//sum 16.91, stage/render/render 13.28, stage/main/post_update 11.72, render/graph/ms 11.25, stage/render/graph_submit 8.15, stage/render/prepare_assets 7.21, encode/finish_wall 4.77, encode/submit 3.42 |
| measure | 2.18 | 35.2 | encode//sum 56.31, render/thread/ms 48.73, render/busy/ms 47.55, render/graph/ms 21.51, main/busy/ms 17.24, stage/render/graph_submit 16.69, stage/render/render 15.41, pipelines/compile_ms 11.08, pipelines/front_ms 10.86, stage/main/post_update 9.69, encode/finish_wall 8.87, stage/render/create_views 8.04 |
| measure | 2.75 | 35.6 | encode//sum 54.57, render/thread/ms 48.71, render/busy/ms 46.98, main/busy/ms 25.31, render/graph/ms 22.97, stage/render/graph_submit 17.57, stage/main/post_update 15.93, stage/render/cleanup 15.11, stage/render/render 10.20, encode/submit 9.44, stage/render/create_views 8.43, encode/finish_wall 8.27 |
| measure | 8.13 | 32.3 | encode//sum 56.93, render/thread/ms 41.87, render/busy/ms 40.80, stage/render/cleanup 22.69, render/graph/ms 20.90, main/busy/ms 18.34, stage/render/graph_submit 16.04, stage/main/post_update 11.81, stage/render/create_views 8.12, encode/submit 8.08, encode/finish_wall 8.00, stage/render/camera_driver 4.73 |
| measure | 9.29 | 33.3 | encode//sum 57.15, render/thread/ms 44.27, render/busy/ms 43.02, render/graph/ms 22.97, main/busy/ms 17.99, stage/render/graph_submit 17.60, stage/render/cleanup 15.90, stage/main/post_update 11.17, encode/finish_wall 8.84, stage/render/render 8.83, encode/submit 8.82, stage/render/create_views 8.14 |
| measure | 9.39 | 36.3 | encode//sum 62.81, render/thread/ms 47.47, render/busy/ms 46.29, stage/render/cleanup 26.94, render/graph/ms 25.17, main/busy/ms 19.87, stage/render/graph_submit 19.18, stage/main/post_update 13.03, encode/finish_wall 9.96, encode/submit 9.27, stage/render/create_views 8.20, stage/render/camera_driver 5.81 |
| measure | 10.43 | 46.2 | render/thread/ms 56.67, render/busy/ms 55.41, render/graph/ms 47.39, stage/render/cleanup 41.27, stage/render/camera_driver 35.16, encode//sum 22.89, main/busy/ms 19.17, stage/main/post_update 12.72, stage/render/graph_submit 12.10, stage/render/render 7.95, encode/finish_wall 6.72, encode/submit 5.45 |
| measure | 12.03 | 65.7 | encode//sum 79.34, render/thread/ms 76.43, render/busy/ms 75.07, stage/render/create_views 33.49, render/graph/ms 27.49, stage/render/render 21.36, stage/render/graph_submit 20.91, main/busy/ms 18.65, stage/main/post_update 12.24, encode/finish_wall 10.67, encode/submit 10.32, pipelines/compile_ms 8.87 |
| measure | 18.83 | 80.9 | encode//sum 101, render/thread/ms 92.72, render/busy/ms 91.05, stage/render/create_views 46.12, render/graph/ms 30.21, stage/render/graph_submit 24.02, stage/render/cleanup 23.49, main/busy/ms 20.89, stage/main/post_update 14.11, encode/finish_wall 12.42, encode/submit 11.65, stage/render/render 9.32 |
| measure | 18.95 | 100.8 | pipelines/compile_ms 144, pipelines/front_ms 139, render/thread/ms 111, stage/render/prepare_views 73.74, render/acquire/ms 73.63, encode//sum 54.85, render/busy/ms 34.82, stage/render/render 25.31, render/graph/ms 21.93, main/busy/ms 21.60, stage/render/graph_submit 18.15, stage/main/post_update 13.29 |
| measure | 59.06 | 34.9 | render/thread/ms 39.28, render/busy/ms 38.05, main/busy/ms 24.22, encode//sum 19.59, stage/render/prepare_assets 18.23, stage/render/cleanup 12.40, stage/main/post_update 11.33, render/graph/ms 11.04, stage/render/graph_submit 8.94, stage/main/update 7.62, encode/finish_wall 5.39, encode/submit 3.61 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ultra2\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_QUALITY=ultra
- before the run: GPU 0%, CPU 3%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **78%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.9 W · 74 °C (max 80) · VRAM 7690 MiB · throttled 100% of samples
