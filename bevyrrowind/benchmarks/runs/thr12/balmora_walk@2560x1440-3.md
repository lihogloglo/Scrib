# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4936 frames over 65.6 s · **median 12.81 ms (78 fps)** · mean 13.30 · p95 17.44 · p99 21.11 (1% low 47 fps) · worst 73.31 ms · 112 spikes over 1.5× median · 35.6 stutters/min (work 34.7/min)
- warm-up: 339 frames over 8.3 s, worst 2555.0 ms
- hitches (> 32.0 ms): **4** measured, 23 in warm-up
- timed GPU passes: 7.66 ms of a 13.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.81 ms, limit 11 ms
- **FAIL** frame_ms: 39 stutters (> 1.5x moving median of 60) in 1.09 min = 35.65/min, limit 1/min
- **FAIL** worst frame 73.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.156 | 2.211 | 2.845 | 0.97 | 0.187 |
| main_opaque_pass_3d | 1.729 | 1.774 | 2.672 | 0.97 | 0.059 |
| early prepass | 0.544 | 0.558 | 0.951 | 0.97 | 0.084 |
| bloom | 0.333 | 0.341 | 0.507 | 0.97 | 0.070 |
| atmosphere_luts | 0.321 | 0.329 | 0.472 | 0.97 | 0.004 |
| water_velocity | 0.298 | 0.306 | 0.418 | 0.97 | 0.002 |
| water_sspr | 0.221 | 0.227 | 0.367 | 0.97 | 0.003 |
| ssao | 0.200 | 0.206 | 0.210 | 0.97 | 0.008 |
| uniform_allocation | 0.184 | 0.188 | 0.191 | 0.98 | 0.034 |
| atmos_froxel_inject | 0.156 | 0.160 | 0.155 | 0.97 | 0.002 |
| auxiliary_views | 0.147 | 0.151 | 0.597 | 0.97 | 0.147 |
| clouds_trace | 0.139 | 0.142 | 0.237 | 0.97 | 0.001 |
| bin_unpacking | 0.136 | 0.139 | 0.145 | 0.98 | 0.025 |
| water_fft | 0.136 | 0.139 | 0.141 | 0.97 | 0.003 |
| upscaling | 0.121 | 0.124 | 0.131 | 0.97 | 0.005 |
| clouds_composite | 0.108 | 0.111 | 0.111 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.092 | 0.094 | 0.173 | 0.97 | 0.009 |
| tonemapping | 0.089 | 0.091 | 0.091 | 0.97 | 0.004 |
| clustering | 0.069 | 0.071 | 0.070 | 0.97 | 0.022 |
| surface_trail | 0.057 | 0.071 | 0.060 | 0.80 | 0.003 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.450 | 0.725 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.051 | 0.050 | 0.98 | 0.006 |
| clouds_resolve | 0.048 | 0.050 | 0.050 | 0.97 | 0.001 |
| water_snapshot | 0.038 | 0.039 | 0.042 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.026 | 0.227 | 0.237 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.026 | 0.027 | 0.030 | 0.97 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.97 | 0.008 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.97 | 0.116 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.97 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.97 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.005 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 23.448 | 0.000 | 13924.25 | 115741.2 |
| encode//sum | 13.682 | 21.057 | 71.33 | 67532.2 |
| render/thread/ms | 11.829 | 15.670 | 70.79 | 58388.9 |
| render/busy/ms | 11.192 | 14.953 | 38.28 | 55245.4 |
| main/busy/ms | 9.690 | 12.821 | 43.18 | 47829.0 |
| render/graph/ms | 7.005 | 9.540 | 18.92 | 34578.2 |
| stage/main/post_update | 5.970 | 8.292 | 20.19 | 29467.8 |
| stage/render/graph_submit | 5.733 | 7.823 | 14.96 | 28298.8 |
| stage/render/render | 4.693 | 9.733 | 63.49 | 23165.9 |
| encode/finish_wall | 3.361 | 4.911 | 10.25 | 16590.0 |
| stage/render/cleanup | 3.171 | 9.502 | 17.84 | 15652.5 |
| encode/submit | 2.442 | 3.447 | 9.18 | 12053.3 |
| stage/main/update | 2.316 | 3.019 | 23.30 | 11432.0 |
| stage/render/camera_driver | 1.205 | 1.714 | 4.00 | 5950.0 |
| stage/render/prepare_write_phase_buffers | 0.988 | 1.407 | 3.29 | 4877.2 |
| bigspace/total/ms | 0.647 | 1.874 | 4.53 | 3194.6 |
| stage/render/prepare_assets | 0.608 | 0.860 | 13.38 | 3001.8 |
| render/present/ms | 0.532 | 0.733 | 52.97 | 2627.8 |
| stage/render/prepare_resources | 0.469 | 0.613 | 2.45 | 2317.4 |
| pipelines/compile_ms | 0.448 | 0.000 | 171.23 | 2209.2 |
| stage/main/first | 0.435 | 0.598 | 5.21 | 2148.6 |
| pipelines/front_ms | 0.431 | 0.000 | 165.05 | 2127.4 |
| stage/render/prepare_meshes | 0.385 | 0.542 | 2.20 | 1901.8 |
| stage/main/pre_update | 0.364 | 0.545 | 7.36 | 1798.9 |
| terrain/update/ms | 0.324 | 0.422 | 1.94 | 1599.2 |
| stage/render/specialize | 0.294 | 0.460 | 2.68 | 1451.7 |
| stage/render/queue | 0.293 | 0.508 | 3.39 | 1446.1 |
| controller/feed/ms | 0.291 | 0.413 | 0.92 | 1437.6 |
| terrain/select/ms | 0.287 | 0.373 | 0.94 | 1417.2 |
| stage/main/fixed_main_loop | 0.279 | 0.466 | 3.22 | 1378.9 |
| stream/drive/ms | 0.278 | 0.422 | 21.19 | 1371.5 |
| stage/render/prepare_bind_groups | 0.271 | 0.471 | 2.91 | 1336.1 |
| bigspace/high_precision/ms | 0.238 | 0.655 | 4.18 | 1173.7 |
| bigspace/low_precision/ms | 0.234 | 0.562 | 3.41 | 1153.6 |
| stream/tick/ms | 0.212 | 0.310 | 0.87 | 1048.1 |
| far/select/ms | 0.212 | 0.303 | 0.63 | 1044.6 |
| stage/render/create_views | 0.200 | 0.278 | 8.13 | 986.3 |
| stage/render/prepare_batch_phases | 0.187 | 0.276 | 1.04 | 921.9 |
| stage/render/prepare_flush | 0.182 | 0.336 | 2.66 | 899.7 |
| anim/evaluate/ms | 0.175 | 0.285 | 4.99 | 864.2 |
| stage/render/extract_commands | 0.168 | 0.226 | 0.55 | 829.5 |
| anim/evaluate_cpu/ms | 0.157 | 0.216 | 0.99 | 773.9 |
| stage/render/prepare_views | 0.152 | 0.217 | 0.69 | 752.6 |
| controller/move/ms | 0.129 | 0.267 | 0.63 | 635.4 |
| stage/main/last | 0.127 | 0.217 | 4.33 | 629.1 |
| render/acquire/ms | 0.104 | 0.149 | 0.55 | 515.7 |
| stage/render/prepare_collect_phase_buffers | 0.067 | 0.142 | 0.39 | 331.8 |
| physics/insert/ms | 0.065 | 0.480 | 0.89 | 321.3 |
| physics/step/ms | 0.060 | 0.101 | 0.75 | 298.0 |
| encode/resolve_diagnostics/sum | 0.038 | 0.051 | 1.57 | 186.5 |
| stage/render/phase_sort | 0.034 | 0.060 | 0.38 | 168.1 |
| weather/map/ms | 0.027 | 0.036 | 0.30 | 133.9 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.026 | 0.94 | 89.7 |
| pipelines/driver_ms | 0.017 | 0.000 | 7.14 | 81.8 |
| encode/camera_driver_span_end/sum | 0.014 | 0.023 | 1.13 | 69.4 |
| anim/apply/ms | 0.012 | 0.023 | 0.24 | 60.6 |
| weather/eval/ms | 0.012 | 0.015 | 0.28 | 57.9 |
| controller/avoid/ms | 0.011 | 0.015 | 0.24 | 54.5 |
| script/update/ms | 0.009 | 0.013 | 0.44 | 42.1 |
| stream/despawn/ms | 0.006 | 0.000 | 1.25 | 31.1 |
| script/ui/ms | 0.006 | 0.009 | 0.21 | 30.2 |
| script/input/ms | 0.006 | 0.008 | 0.40 | 29.0 |
| stream/spawn/ms | 0.006 | 0.000 | 1.21 | 27.3 |
| controller/write/ms | 0.003 | 0.005 | 0.07 | 16.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 15.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.09 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 2.31 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 538 |
| crowd/c0/meshes/count | count | 353 | 646 | 129 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5658 |
| crowd/c1/meshes/count | count | 1180 | 1295 | 1212 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4263 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4241 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.89 | 9.83 | 2.30 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.94 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 1.13 | 0.02 |
| encode/finish_critical | ms | 2.89 | 9.83 | 2.30 |
| encode/resolve_diagnostics/max | ms | 0.03 | 1.57 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.87 | 43.18 | 8.93 |
| gpu/timed/ms | ms | 8.49 | 11.78 | 8.93 |
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
| phases/alpha_mask3d/items | count | 826 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 826 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3452 | 5131 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3452 | 5131 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 225 | 28 |
| phases/shadow/bins | count | 4801 | 5830 | 2229 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 13.2 k | 5680 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 241 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1574 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 11 | 609 | 6 |
| preprocess/bind_groups_created | count | 11 | 609 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 179 | 268 | 173 |
| process/cpu_usage | % | 27.40 | 50.89 | 21.15 |
| process/mem_usage | GiB | 5.05 | 5.22 | 5.22 |
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
| started_load_count |  | 4766 | 4860 | 4860 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 42.16 | 80.48 | 42.61 |
| system/mem_usage | % | 61.89 | 63.30 | 63.15 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2934 | 2934 |
| textures/images | count | 2590 | 2645 | 2645 |
| views/all/batch_sets | count | 85 | 272 | 78 |
| views/all/bins | count | 7370 | 8680 | 5727 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.7 k | 13.3 k |
| views/all/pipelines | count | 246 | 286 | 74 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.5 k | 23.4 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2604 | 3147 | 2229 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8151 | 10.2 k | 5680 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8151 | 10.2 k | 5680 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2145 | 2751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2399 | 3045 | 0 |
| views/lamp_face/pipelines | count | 168 | 198 | 0 |
| views/lamp_face/work_items | count | 0 | 2850 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2880 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8565 | 13.0 k | 7612 |
| views/main/pipelines | count | 45 | 49 | 46 |
| views/main/work_items | count | 8722 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 51.40 | 296 | 253.7 k |
| anim/events/count | count | 0.11 | 3 | 545 |
| assets/Image/added | count | 0.12 | 43 | 595 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.83 | 152 | 4089 |
| assets/Mesh/removed | count | 0.99 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 467 |
| assets/StandardMaterial/modified | count | 4.38 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.03 | 1492 | 5064 |
| encode//count | count | 38.80 | 272 | 191.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 4936 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 4936 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 4936 |
| physics/inserts | count | 0.76 | 139 | 3758 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.80 | 3 | 3937 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 6858 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1504 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.0 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2555.0 | main/busy/ms 2529, stage/main/post_update 2402, stage/main/update 121, surface/advance_cpu_ms 90.86, stage/main/pre_update 2.73, stage/main/last 0.96, surface/cell_upload_ms 0.93, stage/main/first 0.54, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.41, weather/map/ms 0.11, stream/tick/ms 0.09 |
| warm-up | 3.33 | 770.5 | main/busy/ms 2566, stage/main/post_update 2430, render/thread/ms 733, render/busy/ms 587, stage/render/prepare_assets 435, stage/render/prepare_views 144, render/acquire/ms 144, stage/main/update 125, stage/render/prepare_resources 123, surface/advance_cpu_ms 90.86, stage/render/create_views 20.00, stage/render/render 8.39 |
| warm-up | 3.46 | 131.1 | pipelines/wait_ms 902, render/thread/ms 743, render/busy/ms 596, stage/render/prepare_assets 436, stage/render/prepare_views 144, render/acquire/ms 144, pipelines/compile_ms 134, pipelines/front_ms 127, stage/render/prepare_resources 125, main/busy/ms 55.06, stage/main/post_update 38.25, terrain/update/ms 21.93 |
| warm-up | 3.71 | 85.1 | pipelines/compile_ms 238, pipelines/front_ms 228, main/busy/ms 62.32, render/thread/ms 57.43, render/busy/ms 54.60, stage/main/update 51.21, stage/render/prepare_assets 41.85, pipelines/driver_ms 9.78, stage/render/render 9.56, stage/main/post_update 7.35, render/graph/ms 6.27, stage/render/graph_submit 3.32 |
| warm-up | 3.84 | 65.1 | main/busy/ms 47.37, stage/main/update 36.85, render/thread/ms 36.43, render/busy/ms 34.86, stream/drive/ms 33.06, stage/render/prepare_assets 24.78, stage/main/post_update 7.45, render/graph/ms 3.80, stage/render/cleanup 3.36, stage/render/render 2.48, stage/render/graph_submit 2.30, encode/submit 1.82 |
| warm-up | 3.95 | 53.1 | pipelines/wait_ms 2591, pipelines/compile_ms 51.02, pipelines/front_ms 45.70, render/thread/ms 37.94, main/busy/ms 37.45, render/busy/ms 35.71, stage/main/update 26.73, stage/render/prepare_assets 23.79, stream/drive/ms 22.43, stage/render/render 8.71, stage/main/post_update 6.25, render/graph/ms 5.74 |
| warm-up | 4.09 | 121.0 | pipelines/wait_ms 4408, render/thread/ms 75.62, pipelines/compile_ms 75.00, render/busy/ms 74.10, pipelines/front_ms 68.73, main/busy/ms 55.41, stage/render/prepare_assets 38.35, stage/main/update 31.75, stream/drive/ms 27.32, stage/render/prepare_write_phase_buffers 25.17, stage/main/post_update 15.37, pipelines/driver_ms 6.27 |
| warm-up | 4.13 | 37.7 | pipelines/wait_ms 3446, pipelines/compile_ms 184, pipelines/front_ms 165, render/thread/ms 91.42, render/busy/ms 89.96, stage/render/prepare_assets 49.60, main/busy/ms 33.84, stage/render/prepare_write_phase_buffers 25.56, pipelines/driver_ms 19.25, stage/main/post_update 15.10, stage/main/update 10.17, terrain/update/ms 5.74 |
| warm-up | 4.16 | 32.5 | pipelines/wait_ms 5393, pipelines/compile_ms 188, pipelines/front_ms 168, render/thread/ms 54.35, render/busy/ms 52.72, encode//sum 46.26, main/busy/ms 30.03, stage/render/prepare_assets 28.73, pipelines/driver_ms 20.17, stage/main/post_update 12.32, stage/render/render 11.43, stage/main/update 9.81 |
| warm-up | 4.20 | 32.3 | pipelines/wait_ms 4467, encode//sum 73.38, render/thread/ms 53.41, render/busy/ms 51.38, pipelines/compile_ms 40.03, pipelines/front_ms 36.44, stage/render/render 17.42, main/busy/ms 17.33, stage/render/prepare_assets 15.75, render/graph/ms 14.82, stage/render/graph_submit 10.96, stage/main/update 7.68 |
| warm-up | 4.30 | 39.2 | pipelines/wait_ms 3577, pipelines/compile_ms 112, pipelines/front_ms 104, main/busy/ms 73.88, stage/main/update 57.26, stream/drive/ms 52.68, render/thread/ms 42.57, render/busy/ms 40.99, stage/render/prepare_assets 25.17, stage/main/post_update 9.71, pipelines/driver_ms 7.69, stage/render/render 6.50 |
| warm-up | 4.36 | 65.2 | pipelines/wait_ms 2886, main/busy/ms 73.42, stage/main/update 54.74, stream/drive/ms 50.19, render/thread/ms 42.31, render/busy/ms 41.22, stage/render/prepare_assets 31.22, pipelines/compile_ms 28.49, pipelines/front_ms 25.79, stage/main/post_update 12.48, render/graph/ms 3.08, pipelines/driver_ms 2.70 |
| warm-up | 4.45 | 69.7 | pipelines/wait_ms 2378, pipelines/compile_ms 128, pipelines/front_ms 124, main/busy/ms 65.03, stage/main/update 52.09, stream/drive/ms 48.65, render/thread/ms 37.99, render/busy/ms 36.76, stage/render/prepare_assets 20.30, stage/main/post_update 6.66, render/graph/ms 3.79, pipelines/driver_ms 3.46 |
| warm-up | 4.52 | 51.4 | pipelines/wait_ms 4684, pipelines/compile_ms 104, pipelines/front_ms 96.07, main/busy/ms 46.53, stage/main/update 36.82, render/thread/ms 34.78, render/busy/ms 33.46, stream/drive/ms 33.31, stage/render/prepare_assets 13.73, stage/render/cleanup 9.18, pipelines/driver_ms 8.41, render/graph/ms 6.67 |
| warm-up | 4.59 | 59.4 | render/thread/ms 41.67, main/busy/ms 40.29, render/busy/ms 38.99, stage/main/update 32.07, stream/drive/ms 29.45, stage/render/prepare_assets 27.21, stage/main/post_update 5.29, stage/render/cleanup 4.92, render/graph/ms 4.46, encode/submit 3.17, stage/render/render 3.14, encode//sum 2.60 |
| warm-up | 4.73 | 36.8 | pipelines/wait_ms 10.4 k, pipelines/compile_ms 79.42, pipelines/front_ms 68.54, render/thread/ms 44.44, render/busy/ms 42.53, main/busy/ms 27.65, stage/render/prepare_assets 26.68, pipelines/driver_ms 10.88, stage/main/post_update 10.53, stage/main/update 10.53, stream/drive/ms 5.60, render/graph/ms 5.15 |
| warm-up | 4.76 | 33.8 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 61.90, render/thread/ms 54.01, pipelines/front_ms 52.43, render/busy/ms 52.40, main/busy/ms 38.36, stage/render/prepare_assets 35.86, stage/main/update 22.97, stage/main/post_update 10.06, pipelines/driver_ms 9.47, stream/drive/ms 8.79, render/graph/ms 4.51 |
| warm-up | 4.81 | 47.9 | pipelines/wait_ms 3507, pipelines/compile_ms 74.77, pipelines/front_ms 71.56, render/thread/ms 61.64, render/busy/ms 60.25, stage/render/prepare_assets 44.79, main/busy/ms 42.28, stage/main/update 23.11, stage/main/post_update 14.43, stage/render/render 6.47, stream/drive/ms 6.23, render/graph/ms 4.66 |
| warm-up | 4.90 | 74.5 | main/busy/ms 66.40, stage/main/update 49.92, render/thread/ms 48.17, render/busy/ms 46.69, stage/render/prepare_assets 29.60, stream/drive/ms 12.17, stage/main/post_update 11.32, stage/render/render 6.58, terrain/update/ms 5.13, render/graph/ms 4.50, encode//sum 3.72, stage/render/graph_submit 2.98 |
| warm-up | 4.96 | 38.2 | pipelines/wait_ms 3951, pipelines/compile_ms 148, pipelines/front_ms 136, render/thread/ms 42.60, render/busy/ms 40.70, main/busy/ms 31.20, stage/main/update 18.35, stage/render/prepare_assets 18.28, pipelines/driver_ms 11.48, stage/render/render 9.74, stage/main/post_update 8.83, render/graph/ms 7.18 |
| warm-up | 5.81 | 40.2 | pipelines/wait_ms 1209, pipelines/compile_ms 106, pipelines/front_ms 101, render/thread/ms 49.22, render/busy/ms 47.69, main/busy/ms 45.62, stage/render/prepare_assets 30.85, stage/main/update 28.99, stream/drive/ms 21.30, stage/main/post_update 10.17, stage/render/render 9.02, encode//sum 7.62 |
| warm-up | 6.46 | 33.3 | pipelines/wait_ms 4732, pipelines/compile_ms 98.10, pipelines/front_ms 93.92, main/busy/ms 60.50, stage/main/update 44.01, render/thread/ms 37.62, render/busy/ms 35.99, stage/render/prepare_assets 14.02, stage/main/post_update 11.71, encode//sum 11.55, stage/render/render 11.48, render/graph/ms 8.94 |
| warm-up | 6.50 | 46.3 | pipelines/wait_ms 4732, pipelines/compile_ms 98.10, pipelines/front_ms 93.92, main/busy/ms 61.13, render/thread/ms 41.77, render/busy/ms 40.23, stage/main/update 39.73, stage/render/prepare_assets 18.99, stage/main/post_update 15.87, stage/render/render 11.75, encode//sum 11.24, render/graph/ms 9.26 |
| measure | 18.19 | 32.3 | main/busy/ms 39.98, encode//sum 33.40, stage/main/update 25.75, render/thread/ms 25.69, render/busy/ms 24.39, stream/drive/ms 21.44, render/graph/ms 14.76, stage/render/graph_submit 11.82, stage/main/post_update 11.42, stage/render/render 8.35, stage/render/cleanup 8.24, encode/finish_wall 6.53 |
| measure | 35.42 | 73.3 | render/thread/ms 87.37, stage/render/render 74.58, render/present/ms 53.57, encode//sum 33.82, render/busy/ms 33.57, main/busy/ms 28.94, render/graph/ms 19.98, stage/main/post_update 18.94, stage/render/graph_submit 16.37, encode/finish_wall 10.49, encode/submit 6.01, stage/main/update 5.82 |
| measure | 35.54 | 42.2 | render/thread/ms 58.36, main/busy/ms 57.09, render/busy/ms 55.63, encode//sum 47.83, render/graph/ms 29.34, stage/main/post_update 28.15, stage/render/graph_submit 23.93, stage/render/render 21.04, encode/finish_wall 15.37, stage/main/update 12.87, stage/render/cleanup 11.72, encode/submit 8.73 |
| measure | 59.07 | 35.5 | render/thread/ms 49.31, render/busy/ms 47.91, main/busy/ms 30.89, encode//sum 30.44, stage/render/cleanup 18.40, stage/render/prepare_assets 17.68, render/graph/ms 16.63, stage/main/post_update 15.77, stage/render/graph_submit 12.82, stage/main/update 8.77, encode/finish_wall 8.40, encode/submit 4.59 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\thr12\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_COMPUTE_THREADS=12
- before the run: GPU 0%, CPU 18%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **77%** (min 51%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.9 W · 74 °C (max 79) · VRAM 7232 MiB · throttled 100% of samples
