# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5205 frames over 65.7 s · **median 12.27 ms (82 fps)** · mean 12.61 · p95 15.44 · p99 18.28 (1% low 55 fps) · worst 35.27 ms · 49 spikes over 1.5× median · 33.8 stutters/min (work 35.6/min)
- warm-up: 395 frames over 9.2 s, worst 2640.5 ms
- hitches (> 30.7 ms): **4** measured, 23 in warm-up
- timed GPU passes: 7.55 ms of a 12.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.27 ms, limit 11 ms
- **FAIL** frame_ms: 37 stutters (> 1.5x moving median of 60) in 1.09 min = 33.81/min, limit 1/min
- **FAIL** worst frame 35.27 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.121 | 2.142 | 2.531 | 0.99 | 0.180 |
| main_opaque_pass_3d | 1.719 | 1.737 | 2.392 | 0.99 | 0.067 |
| early prepass | 0.547 | 0.552 | 0.921 | 0.99 | 0.081 |
| bloom | 0.327 | 0.330 | 0.334 | 0.99 | 0.065 |
| atmosphere_luts | 0.321 | 0.324 | 0.457 | 0.99 | 0.004 |
| water_velocity | 0.292 | 0.295 | 0.357 | 0.99 | 0.002 |
| water_sspr | 0.218 | 0.221 | 0.241 | 0.99 | 0.002 |
| ssao | 0.198 | 0.200 | 0.207 | 0.99 | 0.008 |
| uniform_allocation | 0.184 | 0.185 | 0.190 | 0.99 | 0.038 |
| atmos_froxel_inject | 0.153 | 0.155 | 0.152 | 0.99 | 0.002 |
| clouds_trace | 0.140 | 0.141 | 0.232 | 0.99 | 0.001 |
| water_fft | 0.138 | 0.140 | 0.141 | 0.99 | 0.003 |
| bin_unpacking | 0.136 | 0.138 | 0.146 | 0.99 | 0.026 |
| upscaling | 0.120 | 0.122 | 0.129 | 0.99 | 0.005 |
| clouds_composite | 0.109 | 0.110 | 0.111 | 0.99 | 0.002 |
| auxiliary_views | 0.099 | 0.100 | 0.470 | 0.99 | 0.101 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.172 | 0.99 | 0.009 |
| tonemapping | 0.088 | 0.089 | 0.091 | 0.99 | 0.004 |
| clustering | 0.070 | 0.070 | 0.070 | 0.99 | 0.021 |
| surface_trail | 0.054 | 0.069 | 0.060 | 0.79 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.450 | 0.766 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.051 | 0.049 | 0.99 | 0.007 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| water_snapshot | 0.035 | 0.035 | 0.040 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.029 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.223 | 0.235 | 0.12 | 0.005 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.99 | 0.113 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 20.714 | 0.000 | 10233.05 | 107818.5 |
| encode//sum | 11.294 | 16.004 | 54.26 | 58783.1 |
| render/thread/ms | 11.181 | 13.805 | 26.52 | 58196.8 |
| render/busy/ms | 10.602 | 13.200 | 26.03 | 55183.8 |
| main/busy/ms | 9.549 | 12.152 | 29.12 | 49701.1 |
| render/graph/ms | 6.490 | 8.283 | 16.21 | 33778.8 |
| stage/main/post_update | 5.985 | 8.266 | 12.33 | 31153.2 |
| stage/render/graph_submit | 5.301 | 6.826 | 11.97 | 27592.5 |
| stage/render/render | 4.470 | 8.664 | 17.14 | 23267.1 |
| encode/finish_wall | 3.075 | 4.188 | 7.36 | 16004.3 |
| stage/render/cleanup | 2.812 | 8.443 | 15.95 | 14637.7 |
| encode/submit | 2.277 | 3.073 | 7.35 | 11850.8 |
| stage/main/update | 2.232 | 2.792 | 22.72 | 11619.7 |
| stage/render/camera_driver | 1.129 | 1.502 | 4.19 | 5874.7 |
| stage/render/prepare_write_phase_buffers | 0.942 | 1.228 | 2.61 | 4904.3 |
| stage/render/prepare_assets | 0.609 | 0.779 | 12.97 | 3167.8 |
| bigspace/total/ms | 0.600 | 1.981 | 4.83 | 3121.5 |
| render/present/ms | 0.480 | 0.605 | 4.13 | 2500.9 |
| stage/render/prepare_resources | 0.475 | 0.595 | 3.61 | 2472.6 |
| stage/main/first | 0.418 | 0.555 | 4.77 | 2174.9 |
| stage/render/prepare_meshes | 0.407 | 0.550 | 1.90 | 2119.6 |
| pipelines/compile_ms | 0.353 | 0.000 | 170.18 | 1834.8 |
| stage/main/pre_update | 0.347 | 0.514 | 2.35 | 1806.1 |
| pipelines/front_ms | 0.340 | 0.000 | 164.77 | 1772.1 |
| terrain/update/ms | 0.303 | 0.394 | 2.14 | 1577.3 |
| controller/feed/ms | 0.299 | 0.389 | 0.83 | 1554.5 |
| stage/render/queue | 0.298 | 0.478 | 3.06 | 1549.4 |
| stage/render/specialize | 0.297 | 0.442 | 3.10 | 1548.4 |
| stage/render/prepare_bind_groups | 0.269 | 0.478 | 2.94 | 1401.6 |
| terrain/select/ms | 0.269 | 0.350 | 0.66 | 1399.7 |
| stream/drive/ms | 0.268 | 0.382 | 20.88 | 1396.1 |
| stage/main/fixed_main_loop | 0.259 | 0.419 | 2.24 | 1349.0 |
| bigspace/low_precision/ms | 0.256 | 0.707 | 4.08 | 1332.0 |
| far/select/ms | 0.206 | 0.298 | 0.73 | 1071.1 |
| stream/tick/ms | 0.205 | 0.292 | 0.91 | 1065.5 |
| stage/render/create_views | 0.193 | 0.257 | 1.22 | 1003.6 |
| bigspace/high_precision/ms | 0.178 | 0.614 | 3.98 | 928.8 |
| stage/render/prepare_batch_phases | 0.176 | 0.250 | 0.49 | 915.9 |
| stage/render/extract_commands | 0.170 | 0.206 | 0.58 | 884.9 |
| stage/render/prepare_flush | 0.166 | 0.296 | 1.20 | 863.4 |
| anim/evaluate/ms | 0.166 | 0.289 | 4.38 | 862.5 |
| anim/evaluate_cpu/ms | 0.157 | 0.203 | 0.69 | 817.8 |
| stage/render/prepare_views | 0.143 | 0.189 | 3.40 | 743.2 |
| controller/move/ms | 0.127 | 0.274 | 0.61 | 663.3 |
| stage/main/last | 0.120 | 0.207 | 1.67 | 623.0 |
| render/acquire/ms | 0.098 | 0.135 | 3.32 | 512.1 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.129 | 0.38 | 303.7 |
| physics/step/ms | 0.058 | 0.097 | 0.76 | 301.2 |
| physics/insert/ms | 0.051 | 0.431 | 1.03 | 265.4 |
| encode/resolve_diagnostics/sum | 0.034 | 0.046 | 0.76 | 176.8 |
| stage/render/phase_sort | 0.027 | 0.047 | 0.29 | 142.0 |
| weather/map/ms | 0.024 | 0.033 | 0.12 | 126.4 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.53 | 85.1 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.44 | 62.8 |
| pipelines/driver_ms | 0.012 | 0.000 | 5.60 | 62.7 |
| anim/apply/ms | 0.012 | 0.023 | 0.15 | 61.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.21 | 56.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.32 | 55.6 |
| script/update/ms | 0.008 | 0.011 | 0.20 | 42.1 |
| script/input/ms | 0.006 | 0.007 | 0.04 | 29.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 28.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.18 | 27.6 |
| script/ui/ms | 0.005 | 0.007 | 0.12 | 24.6 |
| controller/write/ms | 0.003 | 0.004 | 0.02 | 17.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.69 | 14.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.14 | 2.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 28 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 62 | 20 |
| controller/movers/count | count | 20 | 21 | 20 |
| crowd/c0/count | count | 26 | 58 | 14 |
| crowd/c0/joints/count | count | 1285 | 2262 | 649 |
| crowd/c0/meshes/count | count | 353 | 646 | 177 |
| crowd/c1/count | count | 145 | 158 | 147 |
| crowd/c1/joints/count | count | 5507 | 6092 | 5547 |
| crowd/c1/meshes/count | count | 1171 | 1338 | 1164 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.56 | 5.96 | 2.16 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.53 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.44 | 0.01 |
| encode/finish_critical | ms | 2.56 | 5.96 | 2.16 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.76 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.44 | 29.12 | 12.02 |
| gpu/timed/ms | ms | 8.25 | 10.78 | 9.01 |
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
| phases/alpha_mask3d/items | count | 821 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 821 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1285 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3424 | 5114 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1285 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3424 | 5114 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 225 | 29 |
| phases/shadow/bins | count | 4808 | 5623 | 2251 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 12.8 k | 5706 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 249 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 202 | 476 | 19 |
| physics/pending | count | 0 | 829 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 11 | 609 | 37 |
| preprocess/bind_groups_created | count | 11 | 609 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 179 | 271 | 144 |
| process/cpu_usage | % | 25.82 | 39.42 | 18.27 |
| process/mem_usage | GiB | 5.08 | 5.25 | 5.25 |
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
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.18 | 64.54 | 33.86 |
| system/mem_usage | % | 69.15 | 69.50 | 69.37 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 85 | 272 | 79 |
| views/all/bins | count | 7402 | 8501 | 5749 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 249 | 296 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 17.3 k | 23.1 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 45 | 29 |
| views/cascade/bins | count | 2591 | 3153 | 2251 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8159 | 10.0 k | 5706 |
| views/cascade/pipelines | count | 35 | 45 | 29 |
| views/cascade/work_items | count | 8159 | 10.0 k | 5706 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2157 | 2696 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2408 | 3017 | 0 |
| views/lamp_face/pipelines | count | 168 | 204 | 0 |
| views/lamp_face/work_items | count | 0 | 2846 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2876 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8486 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8643 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 48.58 | 906 | 252.9 k |
| anim/events/count | count | 0.10 | 4 | 532 |
| assets/Image/added | count | 0.12 | 43 | 642 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.82 | 152 | 4247 |
| assets/Mesh/removed | count | 0.94 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 36 | 515 |
| assets/StandardMaterial/modified | count | 4.16 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.47 | 2840 | 12.9 k |
| encode//count | count | 36.79 | 272 | 191.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5205 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5205 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5205 |
| physics/inserts | count | 0.58 | 118 | 3042 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 129 | 2335 |
| physics/steps | count | 0.76 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6504 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1426 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.7 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2640.5 | main/busy/ms 2613, stage/main/post_update 2511, stage/main/update 95.95, surface/advance_cpu_ms 66.94, stage/main/pre_update 2.60, stage/main/last 0.95, surface/cell_upload_ms 0.80, stage/main/first 0.52, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.28, weather/map/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.33 | 694.3 | main/busy/ms 2648, stage/main/post_update 2537, render/thread/ms 658, render/busy/ms 605, stage/render/prepare_assets 429, stage/render/prepare_resources 114, stage/main/update 100, surface/advance_cpu_ms 66.94, stage/render/prepare_views 49.94, render/acquire/ms 49.77, stage/render/render 43.81, render/graph/ms 40.21 |
| warm-up | 3.45 | 112.0 | render/thread/ms 670, render/busy/ms 617, stage/render/prepare_assets 430, pipelines/wait_ms 390, pipelines/compile_ms 129, pipelines/front_ms 121, stage/render/prepare_resources 118, main/busy/ms 52.50, stage/render/prepare_views 50.09, render/acquire/ms 49.88, stage/render/render 43.81, render/graph/ms 42.45 |
| warm-up | 3.71 | 86.5 | pipelines/compile_ms 241, pipelines/front_ms 224, render/thread/ms 80.87, main/busy/ms 63.23, render/busy/ms 63.16, stage/render/prepare_assets 51.48, stage/main/update 47.38, pipelines/driver_ms 17.84, stage/render/prepare_views 15.73, render/acquire/ms 15.52, stage/main/post_update 12.24, stage/render/render 7.94 |
| warm-up | 3.85 | 53.3 | main/busy/ms 45.55, render/thread/ms 40.20, render/busy/ms 38.49, stage/main/update 35.77, stream/drive/ms 32.30, stage/render/prepare_assets 29.58, stage/main/post_update 6.99, stage/render/render 6.10, render/graph/ms 4.13, stage/render/graph_submit 2.40, encode/submit 1.99, encode//sum 1.67 |
| warm-up | 3.90 | 43.0 | pipelines/wait_ms 3239, pipelines/compile_ms 206, pipelines/front_ms 185, render/thread/ms 37.22, render/busy/ms 35.91, main/busy/ms 27.93, stage/render/prepare_assets 27.02, pipelines/driver_ms 20.73, stage/main/update 15.37, stream/drive/ms 11.27, stage/main/post_update 7.77, stage/render/render 4.74 |
| warm-up | 4.00 | 80.8 | pipelines/wait_ms 2692, render/thread/ms 63.84, render/busy/ms 62.53, stage/render/prepare_assets 51.62, main/busy/ms 49.85, pipelines/compile_ms 44.97, pipelines/front_ms 41.73, stage/main/update 28.35, stream/drive/ms 24.78, stage/main/post_update 15.52, terrain/update/ms 7.42, render/graph/ms 4.09 |
| warm-up | 4.15 | 134.7 | pipelines/wait_ms 2666, pipelines/compile_ms 179, pipelines/front_ms 170, render/thread/ms 92.11, main/busy/ms 59.09, render/busy/ms 56.66, stage/render/prepare_assets 47.28, stage/render/render 38.67, stage/main/update 35.64, render/present/ms 35.22, stream/drive/ms 28.89, stage/main/post_update 15.44 |
| warm-up | 4.20 | 43.7 | pipelines/wait_ms 8280, pipelines/compile_ms 345, pipelines/front_ms 231, pipelines/driver_ms 114, render/thread/ms 113, render/busy/ms 76.83, stage/render/prepare_assets 61.09, main/busy/ms 43.57, stage/render/render 36.78, render/present/ms 35.29, stage/main/post_update 17.74, stage/main/update 13.76 |
| warm-up | 4.23 | 32.1 | pipelines/wait_ms 5615, pipelines/compile_ms 167, pipelines/driver_ms 106, pipelines/front_ms 61.07, render/thread/ms 59.31, render/busy/ms 57.60, stage/render/prepare_assets 32.06, main/busy/ms 31.19, encode//sum 28.86, render/graph/ms 12.21, stage/main/post_update 11.12, stage/render/render 10.98 |
| warm-up | 4.35 | 40.8 | pipelines/wait_ms 1343, main/busy/ms 73.87, stage/main/update 58.24, stream/drive/ms 53.41, render/thread/ms 40.30, render/busy/ms 38.46, pipelines/compile_ms 37.93, pipelines/front_ms 35.65, stage/render/prepare_assets 21.29, stage/main/post_update 9.22, stage/render/render 4.67, render/graph/ms 4.56 |
| warm-up | 4.43 | 74.1 | pipelines/wait_ms 2659, main/busy/ms 76.17, pipelines/compile_ms 72.72, pipelines/front_ms 68.91, stage/main/update 56.68, stream/drive/ms 51.71, render/thread/ms 45.37, render/busy/ms 44.30, stage/render/prepare_assets 33.24, stage/main/post_update 12.78, stage/render/cleanup 4.24, pipelines/driver_ms 3.82 |
| warm-up | 4.51 | 67.3 | pipelines/wait_ms 2666, pipelines/compile_ms 145, pipelines/front_ms 133, main/busy/ms 61.90, stage/main/update 48.50, stream/drive/ms 44.64, render/thread/ms 33.77, render/busy/ms 32.67, stage/render/prepare_assets 17.56, pipelines/driver_ms 11.67, stage/main/post_update 7.49, render/graph/ms 3.73 |
| warm-up | 4.57 | 46.4 | pipelines/wait_ms 9020, pipelines/compile_ms 57.72, pipelines/front_ms 51.65, main/busy/ms 44.65, stage/main/update 35.61, stream/drive/ms 32.34, render/thread/ms 29.18, render/busy/ms 27.94, stage/render/prepare_assets 12.25, pipelines/driver_ms 6.07, render/graph/ms 5.06, stage/main/post_update 5.04 |
| warm-up | 5.07 | 31.7 | pipelines/wait_ms 4400, pipelines/compile_ms 47.87, pipelines/front_ms 45.66, render/thread/ms 42.45, render/busy/ms 41.26, main/busy/ms 40.88, stage/main/update 28.84, stage/render/prepare_assets 20.88, stage/main/post_update 7.80, stream/drive/ms 7.36, render/graph/ms 5.93, encode//sum 5.54 |
| warm-up | 5.14 | 33.8 | main/busy/ms 33.77, render/thread/ms 26.96, render/busy/ms 25.66, stage/main/update 22.53, stream/drive/ms 13.11, stage/render/prepare_assets 12.48, stage/main/post_update 7.48, render/graph/ms 5.28, encode//sum 4.82, stage/render/cleanup 3.73, stage/render/render 3.46, stage/render/graph_submit 3.44 |
| warm-up | 5.50 | 40.3 | pipelines/wait_ms 7009, pipelines/compile_ms 85.22, pipelines/front_ms 79.96, render/thread/ms 44.12, render/busy/ms 42.33, main/busy/ms 36.14, stage/render/prepare_assets 22.07, stage/main/update 15.33, stage/main/post_update 15.16, stage/render/render 8.74, render/graph/ms 6.33, encode//sum 6.11 |
| warm-up | 5.54 | 44.7 | pipelines/wait_ms 3324, render/thread/ms 64.04, main/busy/ms 63.91, render/busy/ms 60.89, pipelines/compile_ms 42.51, pipelines/front_ms 39.22, stage/main/update 35.73, stage/render/prepare_assets 24.91, stage/main/post_update 22.39, stage/render/render 14.20, render/graph/ms 11.24, stage/render/graph_submit 8.70 |
| warm-up | 5.59 | 49.1 | pipelines/wait_ms 4135, pipelines/compile_ms 89.87, pipelines/front_ms 87.71, render/thread/ms 68.18, render/busy/ms 65.12, main/busy/ms 55.71, stage/main/update 30.35, stage/render/prepare_assets 25.69, stage/main/post_update 17.96, stage/render/render 14.29, render/graph/ms 11.37, stage/render/graph_submit 8.92 |
| warm-up | 5.67 | 51.7 | pipelines/wait_ms 3583, main/busy/ms 56.75, render/thread/ms 52.83, render/busy/ms 51.45, stage/main/update 39.40, stage/render/prepare_assets 20.59, pipelines/compile_ms 19.46, pipelines/front_ms 18.55, stage/main/post_update 12.19, encode//sum 8.73, render/graph/ms 7.84, stage/render/render 6.13 |
| warm-up | 5.82 | 31.6 | render/thread/ms 38.92, render/busy/ms 37.87, main/busy/ms 29.37, stage/render/prepare_assets 21.56, stage/main/post_update 11.62, stage/main/update 10.48, encode//sum 6.68, stage/render/render 6.27, render/graph/ms 4.64, stream/drive/ms 4.03, stage/render/graph_submit 3.10, stage/render/prepare_write_phase_buffers 2.27 |
| warm-up | 6.06 | 37.3 | render/thread/ms 51.94, render/busy/ms 50.44, main/busy/ms 38.48, stage/render/prepare_assets 33.56, stage/main/update 24.37, stream/drive/ms 20.28, stage/main/post_update 9.81, stage/render/render 9.07, encode//sum 8.18, render/graph/ms 6.94, stage/render/graph_submit 5.06, encode/submit 3.40 |
| warm-up | 8.42 | 37.0 | pipelines/wait_ms 2725, main/busy/ms 42.43, render/thread/ms 38.21, render/busy/ms 36.76, pipelines/compile_ms 29.60, pipelines/front_ms 28.26, stage/main/update 25.49, encode//sum 24.86, stage/render/render 13.70, stage/main/post_update 12.87, stage/render/prepare_assets 11.46, render/graph/ms 11.31 |
| measure | 18.18 | 31.6 | main/busy/ms 38.65, stage/main/update 24.66, render/thread/ms 23.25, encode//sum 23.15, render/busy/ms 22.12, stream/drive/ms 21.12, stage/render/cleanup 14.22, render/graph/ms 12.75, stage/main/post_update 10.97, stage/render/graph_submit 10.11, encode/finish_wall 5.68, encode/submit 4.53 |
| measure | 23.10 | 31.8 | render/thread/ms 45.45, render/busy/ms 43.90, main/busy/ms 33.73, encode//sum 31.92, render/graph/ms 15.72, stage/main/post_update 15.04, stage/main/update 14.55, stage/render/graph_submit 11.24, stage/render/render 10.97, stage/render/prepare_assets 10.94, stage/render/cleanup 7.41, encode/finish_wall 6.81 |
| measure | 59.04 | 31.6 | render/thread/ms 45.38, render/busy/ms 43.23, main/busy/ms 35.88, encode//sum 33.65, stage/render/render 23.97, stage/main/post_update 21.34, render/graph/ms 21.25, stage/render/graph_submit 17.28, encode/finish_wall 10.01, stage/main/update 9.27, encode/submit 7.69, stage/render/prepare_assets 7.59 |
| measure | 59.07 | 35.3 | render/thread/ms 47.21, render/busy/ms 45.93, main/busy/ms 29.57, encode//sum 23.57, stage/render/prepare_assets 19.96, stage/main/post_update 15.04, stage/render/render 14.50, render/graph/ms 12.94, stage/render/graph_submit 10.19, stage/main/update 8.58, encode/finish_wall 6.20, encode/submit 4.07 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\b2_on\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 9%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **77%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.2 W · 73 °C (max 79) · VRAM 7082 MiB · throttled 100% of samples
