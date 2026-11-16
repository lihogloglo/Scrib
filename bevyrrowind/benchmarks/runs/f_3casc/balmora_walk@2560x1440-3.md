# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5555 frames over 65.7 s · **median 11.44 ms (87 fps)** · mean 11.82 · p95 14.82 · p99 17.45 (1% low 57 fps) · worst 28.81 ms · 64 spikes over 1.5× median · 25.6 stutters/min (work 31.1/min)
- warm-up: 322 frames over 9.5 s, worst 2568.0 ms
- hitches (> 28.6 ms): **2** measured, 47 in warm-up
- timed GPU passes: 7.20 ms of a 11.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.44 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 28.81 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.002 | 2.082 | 2.239 | 0.96 | 0.178 |
| main_opaque_pass_3d | 1.600 | 1.663 | 2.080 | 0.96 | 0.069 |
| early prepass | 0.540 | 0.561 | 1.110 | 0.96 | 0.082 |
| bloom | 0.309 | 0.321 | 0.326 | 0.96 | 0.064 |
| atmosphere_luts | 0.306 | 0.318 | 0.383 | 0.96 | 0.003 |
| water_velocity | 0.274 | 0.285 | 0.354 | 0.96 | 0.002 |
| water_sspr | 0.198 | 0.206 | 0.206 | 0.96 | 0.002 |
| ssao | 0.189 | 0.196 | 0.204 | 0.96 | 0.007 |
| auxiliary_views | 0.155 | 0.161 | 0.619 | 0.96 | 0.154 |
| atmos_froxel_inject | 0.144 | 0.149 | 0.150 | 0.96 | 0.001 |
| uniform_allocation | 0.140 | 0.145 | 0.287 | 0.96 | 0.026 |
| clouds_trace | 0.137 | 0.142 | 0.236 | 0.96 | 0.001 |
| water_fft | 0.129 | 0.134 | 0.140 | 0.96 | 0.003 |
| upscaling | 0.115 | 0.119 | 0.128 | 0.96 | 0.005 |
| per_view_shadow_pass_cascade2 | 0.106 | 0.467 | 0.531 | 0.23 | 0.035 |
| clouds_composite | 0.104 | 0.108 | 0.110 | 0.96 | 0.002 |
| bin_unpacking | 0.099 | 0.103 | 0.139 | 0.96 | 0.019 |
| tonemapping | 0.083 | 0.087 | 0.090 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.083 | 0.086 | 0.168 | 0.96 | 0.009 |
| clustering | 0.063 | 0.065 | 0.068 | 0.96 | 0.019 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.96 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.96 | 0.001 |
| surface_trail | 0.042 | 0.058 | 0.057 | 0.73 | 0.002 |
| lightprobe_irradiance_map | 0.039 | 0.420 | 0.421 | 0.09 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.036 | 0.158 | 0.174 | 0.23 | 0.037 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.031 | 0.046 | 0.96 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.96 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.020 | 0.222 | 0.232 | 0.09 | 0.004 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.025 | 0.96 | 0.116 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.96 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.09 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.017 | 0.018 | 0.00 | 0.014 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.994 | 17.794 | 53.77 | 61069.2 |
| render/thread/ms | 10.506 | 13.342 | 26.56 | 58358.5 |
| render/busy/ms | 9.915 | 12.742 | 25.79 | 55079.9 |
| main/busy/ms | 9.515 | 11.880 | 26.27 | 52854.5 |
| stage/render/render | 6.626 | 8.616 | 16.74 | 36809.5 |
| stage/main/post_update | 6.028 | 8.003 | 14.35 | 33486.4 |
| render/graph/ms | 5.841 | 7.765 | 15.54 | 32448.2 |
| stage/render/graph_submit | 4.630 | 6.172 | 11.83 | 25722.4 |
| encode/submit | 2.423 | 3.442 | 6.12 | 13461.2 |
| encode/finish_wall | 2.279 | 3.002 | 5.98 | 12660.2 |
| stage/main/update | 2.205 | 2.701 | 20.21 | 12246.6 |
| stage/render/camera_driver | 1.155 | 1.644 | 9.42 | 6417.8 |
| stage/render/prepare_write_phase_buffers | 0.785 | 1.146 | 2.50 | 4359.4 |
| bigspace/total/ms | 0.615 | 1.911 | 4.41 | 3416.5 |
| stage/render/prepare_assets | 0.581 | 0.746 | 10.34 | 3227.8 |
| render/present/ms | 0.498 | 0.623 | 2.34 | 2764.4 |
| stage/render/prepare_resources | 0.468 | 0.574 | 4.37 | 2600.9 |
| stage/main/first | 0.406 | 0.537 | 5.49 | 2256.0 |
| stage/render/prepare_meshes | 0.402 | 0.532 | 2.16 | 2231.6 |
| stage/main/pre_update | 0.338 | 0.500 | 1.58 | 1879.0 |
| terrain/update/ms | 0.308 | 0.405 | 2.18 | 1712.3 |
| stage/render/specialize | 0.302 | 0.447 | 2.62 | 1677.7 |
| stage/render/prepare_bind_groups | 0.293 | 0.515 | 3.00 | 1626.4 |
| controller/feed/ms | 0.287 | 0.366 | 0.67 | 1591.6 |
| terrain/select/ms | 0.274 | 0.362 | 0.91 | 1523.0 |
| bigspace/low_precision/ms | 0.257 | 0.804 | 3.61 | 1429.2 |
| stream/drive/ms | 0.255 | 0.345 | 18.25 | 1418.9 |
| stage/main/fixed_main_loop | 0.235 | 0.384 | 1.57 | 1303.4 |
| stage/render/queue | 0.231 | 0.401 | 3.35 | 1281.0 |
| anim/evaluate/ms | 0.205 | 0.334 | 2.69 | 1136.1 |
| bigspace/high_precision/ms | 0.200 | 0.824 | 3.10 | 1111.1 |
| stream/tick/ms | 0.197 | 0.269 | 0.63 | 1093.0 |
| stage/render/create_views | 0.193 | 0.255 | 1.02 | 1073.3 |
| far/select/ms | 0.193 | 0.264 | 0.60 | 1072.4 |
| stage/render/extract_commands | 0.184 | 0.220 | 0.53 | 1020.0 |
| stage/render/prepare_flush | 0.179 | 0.386 | 0.90 | 995.2 |
| stage/render/prepare_batch_phases | 0.163 | 0.229 | 0.47 | 906.8 |
| anim/evaluate_cpu/ms | 0.149 | 0.192 | 0.50 | 827.2 |
| stage/render/prepare_views | 0.135 | 0.171 | 0.45 | 747.2 |
| controller/move/ms | 0.125 | 0.222 | 0.57 | 697.1 |
| stage/main/last | 0.124 | 0.203 | 3.38 | 687.7 |
| render/acquire/ms | 0.093 | 0.120 | 0.41 | 514.3 |
| stage/render/phase_sort | 0.078 | 0.105 | 0.35 | 433.1 |
| pipelines/compile_ms | 0.053 | 0.000 | 121.04 | 293.4 |
| physics/step/ms | 0.051 | 0.087 | 0.64 | 285.2 |
| pipelines/front_ms | 0.051 | 0.000 | 116.90 | 281.0 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.074 | 0.33 | 264.9 |
| physics/insert/ms | 0.037 | 0.390 | 0.81 | 204.0 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.68 | 180.0 |
| stage/render/cleanup | 0.031 | 0.090 | 1.70 | 174.4 |
| weather/map/ms | 0.026 | 0.034 | 0.23 | 146.1 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.19 | 88.0 |
| anim/apply/ms | 0.012 | 0.022 | 0.22 | 66.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.41 | 63.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 58.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.13 | 57.8 |
| script/update/ms | 0.007 | 0.009 | 0.25 | 40.0 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 28.7 |
| stream/despawn/ms | 0.005 | 0.000 | 0.90 | 28.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.6 |
| script/ui/ms | 0.005 | 0.006 | 0.08 | 25.8 |
| controller/write/ms | 0.004 | 0.005 | 0.20 | 20.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 15.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.15 | 12.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.20 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.0 |
| stage/render/prepare | 0.001 | 0.002 | 0.04 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.22 | 1.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1272 | 2218 | 595 |
| crowd/c0/meshes/count | count | 352 | 633 | 158 |
| crowd/c1/count | count | 146 | 154 | 148 |
| crowd/c1/joints/count | count | 5534 | 5882 | 5601 |
| crowd/c1/meshes/count | count | 1166 | 1254 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.85 | 3.83 | 2.11 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.19 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.41 | 0.01 |
| encode/finish_critical | ms | 1.85 | 3.83 | 2.11 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.68 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.92 | 26.27 | 9.19 |
| gpu/timed/ms | ms | 8.65 | 12.05 | 9.19 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3406 | 5146 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3406 | 5146 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 198 | 4 |
| phases/shadow/bins | count | 4850 | 6000 | 2325 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.0 k | 13.5 k | 6107 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 198 | 249 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 223 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 553 | 9 |
| preprocess/bind_groups_created | count | 14 | 553 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 126 | 236 | 120 |
| process/cpu_usage | % | 26.44 | 41.18 | 22.60 |
| process/mem_usage | GiB | 5.37 | 5.53 | 5.53 |
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
| started_load_count |  | 4915 | 5009 | 5009 |
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
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.37 | 63.63 | 29.54 |
| system/mem_usage | % | 55.19 | 56.84 | 55.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 61 | 245 | 54 |
| views/all/bins | count | 7433 | 8918 | 5823 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.1 k | 26.1 k | 13.7 k |
| views/all/pipelines | count | 243 | 296 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.6 k | 23.2 k | 8071 |
| views/cascade/batch_sets | count | 7 | 43 | 4 |
| views/cascade/bins | count | 2660 | 3230 | 2325 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8542 | 10.5 k | 6107 |
| views/cascade/pipelines | count | 33 | 44 | 29 |
| views/cascade/work_items | count | 242 | 10.4 k | 145 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2123 | 2802 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2377 | 3101 | 0 |
| views/lamp_face/pipelines | count | 164 | 207 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2874 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8428 | 13.1 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8588 | 13.3 k | 7926 |
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
| anim/bones_written/count | count | 56.41 | 293 | 313.4 k |
| anim/events/count | count | 0.08 | 3 | 470 |
| assets/Image/added | count | 0.11 | 41 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.87 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.90 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 1328 | 4676 |
| encode//count | count | 39.92 | 272 | 221.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5555 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5555 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5555 |
| physics/inserts | count | 0.41 | 92 | 2252 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.71 | 2 | 3939 |
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
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 20 | 378 |
| stream/upload/bytes | bytes | 6094 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1336 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2568.0 | main/busy/ms 2540, stage/main/post_update 2433, stage/main/update 100, surface/advance_cpu_ms 69.56, stage/main/pre_update 2.26, stage/main/last 1.04, surface/cell_upload_ms 0.77, stage/main/first 0.68, stage/main/fixed_main_loop 0.43, stream/drive/ms 0.39, weather/map/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.30 | 728.3 | main/busy/ms 2678, stage/main/post_update 2460, render/thread/ms 694, render/busy/ms 542, stage/render/prepare_assets 494, stage/main/update 206, stage/render/prepare_views 149, render/acquire/ms 149, surface/advance_cpu_ms 69.56, stage/render/create_views 26.35, stage/render/render 11.28, encode//sum 10.73 |
| warm-up | 3.36 | 66.1 | render/thread/ms 742, render/busy/ms 589, stage/render/prepare_assets 520, main/busy/ms 158, stage/render/prepare_views 150, render/acquire/ms 150, stage/main/update 110, pipelines/compile_ms 89.50, pipelines/front_ms 85.23, stage/main/post_update 37.47, stage/render/create_views 27.01, stage/render/render 23.07 |
| warm-up | 3.40 | 41.5 | pipelines/compile_ms 154, pipelines/front_ms 146, render/thread/ms 77.54, render/busy/ms 67.80, main/busy/ms 59.19, stage/main/post_update 39.12, stage/render/render 29.15, stage/render/prepare_assets 27.04, terrain/update/ms 26.14, pipelines/wait_ms 25.74, render/graph/ms 17.63, pipelines/lock_ms 10.10 |
| warm-up | 3.57 | 168.6 | pipelines/wait_ms 6631, pipelines/compile_ms 2020, pipelines/front_ms 1885, render/thread/ms 195, render/busy/ms 186, pipelines/driver_ms 135, main/busy/ms 79.22, stage/main/post_update 53.35, stage/render/prepare_resources 52.93, stage/render/prepare_write_phase_buffers 51.72, stage/render/render 36.59, encode//sum 31.37 |
| warm-up | 3.61 | 40.0 | pipelines/wait_ms 6886, pipelines/compile_ms 2353, pipelines/front_ms 2205, render/thread/ms 204, render/busy/ms 201, pipelines/driver_ms 149, main/busy/ms 124, stage/main/update 66.58, stage/render/prepare_resources 57.02, stage/render/prepare_write_phase_buffers 50.64, stage/main/post_update 42.29, encode//sum 35.47 |
| warm-up | 3.76 | 143.3 | pipelines/compile_ms 1455, pipelines/front_ms 1367, pipelines/wait_ms 515, render/thread/ms 131, render/busy/ms 103, main/busy/ms 101, pipelines/driver_ms 87.69, stage/render/prepare_assets 68.47, stage/main/update 65.42, stage/render/render 38.83, render/present/ms 27.67, stage/main/post_update 25.34 |
| warm-up | 3.79 | 33.2 | pipelines/compile_ms 1680, pipelines/front_ms 1523, pipelines/wait_ms 378, pipelines/driver_ms 157, render/thread/ms 124, render/busy/ms 95.62, stage/render/prepare_assets 71.58, stage/render/render 43.98, main/busy/ms 36.22, render/present/ms 28.39, stage/main/post_update 17.53, render/graph/ms 14.75 |
| warm-up | 3.83 | 36.6 | pipelines/compile_ms 1176, pipelines/front_ms 1067, pipelines/wait_ms 280, pipelines/driver_ms 109, render/thread/ms 65.74, render/busy/ms 62.40, main/busy/ms 49.65, stage/render/prepare_assets 26.86, stage/render/render 26.52, stage/main/post_update 26.02, render/graph/ms 22.86, stage/main/update 16.02 |
| warm-up | 3.87 | 29.5 | pipelines/compile_ms 68.05, pipelines/front_ms 64.02, render/thread/ms 44.73, render/busy/ms 42.51, main/busy/ms 36.41, encode//sum 21.18, stage/render/prepare_assets 17.20, stage/main/post_update 16.38, stage/main/update 15.51, stage/render/render 15.25, render/graph/ms 12.53, stage/render/graph_submit 8.77 |
| warm-up | 3.98 | 84.1 | pipelines/compile_ms 961, pipelines/front_ms 923, main/busy/ms 64.19, render/thread/ms 57.78, render/busy/ms 55.92, stage/main/update 51.49, stream/drive/ms 47.58, stage/render/prepare_assets 39.16, pipelines/driver_ms 37.57, stage/render/render 11.60, render/graph/ms 9.31, stage/main/post_update 8.54 |
| warm-up | 4.02 | 36.7 | pipelines/compile_ms 1223, pipelines/front_ms 1177, render/thread/ms 68.44, render/busy/ms 66.47, main/busy/ms 50.65, pipelines/driver_ms 45.98, stage/render/prepare_assets 33.70, stage/main/update 27.24, stage/render/render 23.96, render/graph/ms 21.58, stream/drive/ms 21.16, stage/main/post_update 15.73 |
| warm-up | 4.05 | 35.2 | pipelines/compile_ms 522, pipelines/front_ms 506, main/busy/ms 53.49, render/thread/ms 51.04, render/busy/ms 49.27, stage/main/update 27.14, stage/render/render 21.59, stream/drive/ms 20.95, render/graph/ms 19.37, stage/render/prepare_assets 18.53, pipelines/driver_ms 16.37, stage/main/post_update 15.38 |
| warm-up | 4.11 | 57.8 | pipelines/compile_ms 534, pipelines/front_ms 513, render/thread/ms 70.73, render/busy/ms 68.86, main/busy/ms 52.47, stage/render/prepare_assets 51.84, stage/main/update 34.41, stream/drive/ms 29.77, pipelines/driver_ms 21.17, stage/render/render 11.23, stage/main/post_update 10.17, render/graph/ms 8.83 |
| warm-up | 4.19 | 84.5 | pipelines/compile_ms 367, pipelines/front_ms 350, render/thread/ms 125, render/busy/ms 123, stage/render/prepare_assets 94.44, main/busy/ms 64.77, stage/main/update 42.70, stream/drive/ms 31.99, pipelines/driver_ms 16.58, stage/render/render 12.45, stage/main/post_update 12.42, render/graph/ms 9.82 |
| warm-up | 4.27 | 78.7 | pipelines/compile_ms 1355, pipelines/front_ms 1265, render/thread/ms 136, render/busy/ms 134, stage/render/prepare_assets 96.38, pipelines/driver_ms 90.49, main/busy/ms 74.54, stage/main/update 49.40, stage/render/render 14.74, stage/main/post_update 13.21, render/graph/ms 12.22, pipelines/wait_ms 12.04 |
| warm-up | 4.44 | 163.4 | pipelines/compile_ms 3352, pipelines/front_ms 3216, pipelines/wait_ms 415, render/thread/ms 174, render/busy/ms 172, pipelines/driver_ms 136, stage/render/prepare_assets 118, main/busy/ms 93.35, stage/main/update 48.39, stage/render/render 36.51, stage/main/post_update 35.08, render/graph/ms 33.64 |
| warm-up | 4.55 | 109.4 | pipelines/compile_ms 3609, pipelines/front_ms 3512, pipelines/wait_ms 828, render/thread/ms 211, render/busy/ms 209, stage/render/prepare_assets 106, pipelines/driver_ms 97.02, main/busy/ms 69.58, stage/render/render 36.60, stage/main/post_update 35.01, render/graph/ms 33.78, stage/render/prepare_resources 27.79 |
| warm-up | 4.63 | 82.1 | pipelines/compile_ms 2692, pipelines/front_ms 2608, pipelines/wait_ms 716, render/thread/ms 173, render/busy/ms 171, pipelines/driver_ms 84.72, stage/render/prepare_assets 57.51, main/busy/ms 53.15, encode//sum 39.73, stage/render/prepare_resources 37.23, stage/render/prepare_write_phase_buffers 31.64, stage/main/post_update 25.01 |
| warm-up | 4.71 | 78.0 | pipelines/compile_ms 2355, pipelines/front_ms 2282, pipelines/wait_ms 1056, render/thread/ms 140, render/busy/ms 138, pipelines/driver_ms 73.31, main/busy/ms 69.95, encode//sum 47.98, stage/render/prepare_assets 39.66, stage/main/post_update 36.38, stage/render/render 32.46, stage/render/prepare_resources 30.13 |
| warm-up | 4.74 | 34.9 | pipelines/compile_ms 1421, pipelines/front_ms 1377, pipelines/wait_ms 852, render/thread/ms 95.22, render/busy/ms 92.13, main/busy/ms 66.21, pipelines/driver_ms 43.41, encode//sum 43.10, stage/render/render 35.63, render/graph/ms 31.81, stage/main/post_update 28.61, stage/main/update 28.39 |
| warm-up | 4.77 | 31.2 | pipelines/compile_ms 479, pipelines/front_ms 464, pipelines/wait_ms 89.69, render/thread/ms 53.34, render/busy/ms 50.68, main/busy/ms 44.39, encode//sum 21.87, stage/main/update 21.23, stage/render/render 20.25, render/graph/ms 17.09, stage/main/post_update 16.01, pipelines/driver_ms 15.37 |
| warm-up | 4.80 | 32.8 | pipelines/compile_ms 910, pipelines/front_ms 883, pipelines/wait_ms 194, main/busy/ms 81.03, stage/main/update 61.65, stream/drive/ms 48.09, render/thread/ms 44.96, render/busy/ms 43.26, pipelines/driver_ms 27.11, stage/render/prepare_assets 19.03, stage/main/post_update 11.85, encode//sum 11.76 |
| warm-up | 4.88 | 77.6 | pipelines/compile_ms 884, pipelines/front_ms 859, pipelines/wait_ms 283, main/busy/ms 103, stage/main/update 79.81, render/thread/ms 58.50, render/busy/ms 55.25, stream/drive/ms 48.50, pipelines/driver_ms 24.83, stage/render/render 21.61, render/graph/ms 17.07, stage/render/prepare_assets 15.64 |
| warm-up | 4.98 | 101.7 | pipelines/compile_ms 952, pipelines/front_ms 923, main/busy/ms 139, pipelines/wait_ms 116, stage/main/update 113, render/thread/ms 110, render/busy/ms 105, stream/drive/ms 52.07, stage/render/prepare_assets 43.45, stage/render/render 29.77, pipelines/driver_ms 28.81, pipelines/lock_ms 25.83 |
| warm-up | 5.14 | 151.7 | pipelines/compile_ms 2862, pipelines/front_ms 2633, pipelines/driver_ms 229, render/thread/ms 172, render/busy/ms 169, stage/render/prepare_assets 123, main/busy/ms 123, stage/main/update 96.55, pipelines/wait_ms 64.29, stream/drive/ms 52.40, pipelines/lock_ms 40.58, stage/render/render 21.70 |
| warm-up | 5.17 | 37.4 | pipelines/compile_ms 2193, pipelines/front_ms 1982, pipelines/driver_ms 211, render/thread/ms 131, render/busy/ms 128, stage/render/prepare_assets 91.63, main/busy/ms 90.96, stage/main/update 63.44, stream/drive/ms 41.71, pipelines/wait_ms 40.78, stage/render/render 25.89, render/graph/ms 22.91 |
| warm-up | 5.27 | 96.5 | pipelines/compile_ms 135, pipelines/front_ms 127, main/busy/ms 87.16, render/thread/ms 80.54, render/busy/ms 78.07, stage/main/update 65.07, stream/drive/ms 41.41, stage/render/prepare_assets 40.06, stage/render/render 24.31, render/graph/ms 21.16, stage/render/graph_submit 18.16, encode//sum 14.75 |
| warm-up | 5.35 | 55.2 | pipelines/compile_ms 764, pipelines/front_ms 741, main/busy/ms 67.15, stage/main/update 48.99, render/thread/ms 47.89, render/busy/ms 45.72, stream/drive/ms 34.36, pipelines/driver_ms 22.37, stage/render/render 16.48, encode//sum 15.87, render/graph/ms 12.86, stage/render/prepare_assets 11.09 |
| warm-up | 5.38 | 29.0 | pipelines/compile_ms 1150, pipelines/front_ms 1118, render/thread/ms 49.50, render/busy/ms 47.50, main/busy/ms 40.56, pipelines/driver_ms 31.54, stage/main/post_update 19.07, encode//sum 15.64, stage/render/render 14.41, stage/main/update 14.26, render/graph/ms 12.04, stage/render/prepare_assets 10.64 |
| warm-up | 5.41 | 32.9 | pipelines/compile_ms 590, pipelines/front_ms 573, render/thread/ms 54.73, render/busy/ms 52.90, main/busy/ms 48.97, stage/main/post_update 20.90, stage/main/update 18.99, pipelines/driver_ms 17.61, stage/render/render 17.15, encode//sum 16.86, render/graph/ms 14.84, stage/render/graph_submit 11.48 |
| warm-up | 5.45 | 33.0 | pipelines/compile_ms 204, pipelines/front_ms 195, render/thread/ms 56.26, render/busy/ms 54.64, main/busy/ms 42.17, stage/main/update 19.01, stage/render/prepare_assets 18.18, encode//sum 17.31, stage/main/post_update 14.66, stage/render/render 14.28, render/graph/ms 11.99, stage/render/graph_submit 9.10 |
| warm-up | 5.48 | 30.3 | pipelines/compile_ms 221, pipelines/front_ms 213, render/thread/ms 54.73, render/busy/ms 52.62, main/busy/ms 31.19, stage/render/prepare_assets 17.34, encode//sum 14.94, stage/render/render 14.46, stage/main/update 12.51, stage/main/post_update 12.30, render/graph/ms 11.76, stage/render/graph_submit 8.64 |
| warm-up | 5.57 | 68.8 | pipelines/compile_ms 747, pipelines/front_ms 723, render/thread/ms 63.70, render/busy/ms 61.67, main/busy/ms 45.49, stage/render/prepare_assets 30.12, pipelines/driver_ms 23.95, stage/render/render 21.92, render/graph/ms 19.25, encode//sum 18.80, stage/main/update 18.47, stage/render/graph_submit 13.12 |
| warm-up | 5.60 | 33.2 | pipelines/compile_ms 798, pipelines/front_ms 772, render/thread/ms 73.32, render/busy/ms 71.51, main/busy/ms 48.93, stage/render/prepare_assets 35.14, stage/main/update 31.23, pipelines/driver_ms 26.73, stage/render/render 25.51, render/graph/ms 22.65, encode//sum 17.40, stream/drive/ms 13.80 |
| warm-up | 5.64 | 43.1 | pipelines/compile_ms 749, pipelines/front_ms 730, render/thread/ms 59.64, render/busy/ms 58.02, main/busy/ms 50.38, stage/render/prepare_assets 28.23, stage/main/update 28.23, pipelines/driver_ms 19.55, stage/render/render 18.56, encode//sum 16.38, render/graph/ms 15.84, stage/main/post_update 15.63 |
| warm-up | 5.69 | 29.0 | render/thread/ms 42.56, render/busy/ms 41.07, main/busy/ms 25.67, encode//sum 17.37, stage/render/prepare_assets 16.83, stage/render/render 14.57, render/graph/ms 12.42, stage/main/post_update 11.67, stage/render/graph_submit 9.01, stage/main/update 8.99, encode/submit 5.25, encode/finish_wall 3.87 |
| warm-up | 5.86 | 34.4 | pipelines/compile_ms 352, pipelines/front_ms 344, render/thread/ms 46.95, render/busy/ms 45.63, main/busy/ms 28.66, stage/render/prepare_assets 26.20, encode//sum 15.19, stage/main/post_update 12.65, stage/render/render 11.56, stage/main/update 10.08, render/graph/ms 9.79, pipelines/driver_ms 8.36 |
| warm-up | 6.03 | 28.6 | main/busy/ms 34.39, render/thread/ms 24.26, render/busy/ms 22.92, stage/main/update 22.06, encode//sum 18.24, stream/drive/ms 17.37, stage/render/render 11.01, render/graph/ms 9.03, stage/main/post_update 8.57, stage/render/graph_submit 6.63, stage/render/prepare_assets 5.48, encode/submit 3.54 |
| warm-up | 6.35 | 30.8 | render/thread/ms 43.17, render/busy/ms 41.18, main/busy/ms 30.49, encode//sum 27.29, stage/render/render 18.02, render/graph/ms 15.45, stage/main/post_update 14.94, stage/render/graph_submit 11.38, stage/main/update 11.22, encode/submit 6.63, stage/render/prepare_assets 5.31, encode/finish_wall 4.83 |
| warm-up | 6.40 | 32.9 | render/thread/ms 35.51, render/busy/ms 34.35, main/busy/ms 26.53, stage/render/prepare_assets 14.99, encode//sum 14.87, stage/main/update 11.12, stage/main/post_update 10.83, stage/render/render 10.64, render/graph/ms 9.00, stage/render/graph_submit 6.60, stream/drive/ms 3.51, encode/submit 3.37 |
| warm-up | 6.61 | 32.3 | main/busy/ms 44.44, render/thread/ms 42.54, render/busy/ms 40.76, encode//sum 24.37, stage/main/update 22.47, stage/render/render 17.14, stage/main/post_update 15.11, render/graph/ms 14.18, stage/render/graph_submit 10.50, stream/drive/ms 5.83, stage/render/prepare_assets 5.67, encode/finish_wall 5.33 |
| warm-up | 6.64 | 29.0 | pipelines/compile_ms 83.84, pipelines/front_ms 79.68, render/thread/ms 47.03, render/busy/ms 45.34, main/busy/ms 40.44, encode//sum 22.18, stage/main/update 17.53, stage/render/render 15.13, stage/render/prepare_assets 14.41, stage/main/post_update 14.29, render/graph/ms 12.39, stage/render/graph_submit 9.10 |
| warm-up | 6.67 | 33.0 | pipelines/compile_ms 83.84, pipelines/front_ms 79.68, render/thread/ms 45.54, render/busy/ms 44.33, main/busy/ms 32.78, stage/render/prepare_assets 23.36, encode//sum 17.24, stage/main/update 14.43, stage/main/post_update 11.49, stage/render/render 9.64, render/graph/ms 7.95, stage/render/graph_submit 5.69 |
| warm-up | 6.75 | 31.6 | render/thread/ms 41.79, main/busy/ms 41.41, render/busy/ms 40.64, stage/main/update 22.17, stage/render/prepare_assets 15.73, encode//sum 15.24, stage/main/post_update 11.53, stage/render/render 11.36, render/graph/ms 9.45, stage/render/graph_submit 6.60, stream/drive/ms 3.95, encode/finish_wall 3.82 |
| warm-up | 6.93 | 46.9 | render/thread/ms 58.98, render/busy/ms 57.42, main/busy/ms 43.53, stage/render/prepare_assets 34.95, stage/main/update 28.44, encode//sum 22.38, stream/drive/ms 19.90, stage/render/render 13.63, render/graph/ms 11.41, stage/main/post_update 10.50, stage/render/graph_submit 9.15, encode/submit 5.19 |
| warm-up | 6.98 | 30.4 | render/thread/ms 34.60, render/busy/ms 33.41, main/busy/ms 26.02, encode//sum 18.37, stage/render/prepare_assets 15.28, stage/render/render 11.33, render/graph/ms 9.67, stage/main/update 9.56, stage/main/post_update 9.29, stage/render/graph_submit 7.28, encode/finish_wall 3.70, encode/submit 3.64 |
| measure | 8.13 | 28.8 | encode//sum 63.88, render/thread/ms 36.63, render/busy/ms 35.26, stage/render/render 23.35, render/graph/ms 21.35, main/busy/ms 20.34, stage/render/graph_submit 16.32, stage/main/post_update 13.37, encode/submit 8.64, encode/finish_wall 7.78, stage/render/camera_driver 4.91, stage/main/update 4.51 |
| measure | 59.04 | 28.7 | render/thread/ms 41.38, render/busy/ms 40.08, main/busy/ms 26.22, encode//sum 21.36, stage/render/prepare_assets 16.75, stage/main/post_update 14.41, stage/render/render 12.18, render/graph/ms 10.60, stage/render/graph_submit 8.34, stage/main/update 6.59, encode/finish_wall 4.23, encode/submit 4.20 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_3casc\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 10%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **75%** (min 49%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.2 W · 73 °C (max 78) · VRAM 5537 MiB · throttled 100% of samples
