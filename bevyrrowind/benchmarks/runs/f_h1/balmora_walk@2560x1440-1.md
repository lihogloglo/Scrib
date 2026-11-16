# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5768 frames over 65.7 s · **median 11.05 ms (91 fps)** · mean 11.38 · p95 13.95 · p99 16.10 (1% low 62 fps) · worst 28.09 ms · 41 spikes over 1.5× median · 27.4 stutters/min (work 25.6/min)
- warm-up: 325 frames over 9.0 s, worst 2516.3 ms
- hitches (> 27.6 ms): **2** measured, 39 in warm-up
- timed GPU passes: 7.32 ms of a 11.38 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.05 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 28.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.026 | 2.082 | 2.221 | 0.97 | 0.172 |
| main_opaque_pass_3d | 1.606 | 1.650 | 2.044 | 0.97 | 0.059 |
| early prepass | 0.542 | 0.557 | 1.027 | 0.97 | 0.077 |
| bloom | 0.312 | 0.320 | 0.325 | 0.97 | 0.063 |
| atmosphere_luts | 0.305 | 0.313 | 0.373 | 0.97 | 0.003 |
| water_velocity | 0.278 | 0.286 | 0.353 | 0.97 | 0.002 |
| water_sspr | 0.201 | 0.207 | 0.206 | 0.97 | 0.002 |
| ssao | 0.188 | 0.193 | 0.195 | 0.97 | 0.007 |
| uniform_allocation | 0.178 | 0.183 | 0.465 | 0.97 | 0.024 |
| auxiliary_views | 0.154 | 0.158 | 0.618 | 0.97 | 0.136 |
| atmos_froxel_inject | 0.144 | 0.147 | 0.147 | 0.97 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.139 | 0.585 | 0.654 | 0.24 | 0.039 |
| clouds_trace | 0.139 | 0.143 | 0.234 | 0.97 | 0.001 |
| water_fft | 0.132 | 0.136 | 0.142 | 0.97 | 0.003 |
| upscaling | 0.113 | 0.116 | 0.126 | 0.97 | 0.004 |
| bin_unpacking | 0.105 | 0.108 | 0.131 | 0.97 | 0.017 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.97 | 0.002 |
| tonemapping | 0.084 | 0.087 | 0.090 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.084 | 0.161 | 0.97 | 0.008 |
| clustering | 0.064 | 0.066 | 0.068 | 0.97 | 0.019 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.421 | 0.420 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| surface_trail | 0.041 | 0.057 | 0.057 | 0.72 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.053 | 0.97 | 0.004 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.224 | 0.234 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.97 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.024 | 0.97 | 0.156 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.97 | 0.004 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_downsampling_second_pass | 0.002 | 0.016 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.12 | 0.004 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.038 | 0.050 | 0.01 | 0.015 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.043 | 0.047 | 0.00 | 0.016 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.227 | 16.717 | 59.78 | 58987.8 |
| render/thread/ms | 10.069 | 12.502 | 26.26 | 58079.6 |
| render/busy/ms | 9.486 | 11.937 | 25.28 | 54717.9 |
| main/busy/ms | 8.656 | 10.663 | 24.73 | 49927.2 |
| stage/render/render | 6.238 | 7.951 | 17.09 | 35982.4 |
| render/graph/ms | 5.443 | 7.109 | 15.58 | 31394.8 |
| stage/main/post_update | 5.263 | 6.913 | 9.58 | 30359.2 |
| stage/render/graph_submit | 4.260 | 5.572 | 11.42 | 24569.5 |
| encode/submit | 2.268 | 3.087 | 6.78 | 13083.2 |
| stage/main/update | 2.148 | 2.578 | 19.12 | 12386.8 |
| encode/finish_wall | 2.066 | 2.724 | 6.44 | 11915.3 |
| stage/render/camera_driver | 1.128 | 1.548 | 4.09 | 6508.3 |
| stage/render/prepare_write_phase_buffers | 0.753 | 1.070 | 2.87 | 4343.6 |
| stage/render/prepare_assets | 0.550 | 0.706 | 9.22 | 3174.3 |
| bigspace/total/ms | 0.540 | 1.524 | 3.02 | 3117.0 |
| render/present/ms | 0.493 | 0.619 | 2.56 | 2843.3 |
| stage/render/prepare_meshes | 0.446 | 0.602 | 2.52 | 2572.1 |
| stage/render/prepare_resources | 0.445 | 0.535 | 1.10 | 2568.3 |
| stage/main/first | 0.396 | 0.520 | 5.33 | 2284.5 |
| stage/main/pre_update | 0.329 | 0.492 | 1.52 | 1900.5 |
| terrain/update/ms | 0.302 | 0.390 | 2.11 | 1741.9 |
| stage/render/specialize | 0.293 | 0.429 | 5.83 | 1689.0 |
| controller/feed/ms | 0.274 | 0.342 | 0.96 | 1583.1 |
| terrain/select/ms | 0.269 | 0.348 | 0.70 | 1552.4 |
| stage/render/prepare_bind_groups | 0.262 | 0.452 | 2.76 | 1508.5 |
| stream/drive/ms | 0.243 | 0.321 | 17.12 | 1402.8 |
| bigspace/low_precision/ms | 0.232 | 0.666 | 2.64 | 1336.5 |
| stage/render/prepare_batch_phases | 0.226 | 0.309 | 0.89 | 1305.0 |
| stage/render/queue | 0.223 | 0.383 | 2.82 | 1283.4 |
| stage/main/fixed_main_loop | 0.222 | 0.372 | 1.07 | 1282.8 |
| stream/tick/ms | 0.188 | 0.256 | 0.53 | 1083.9 |
| anim/evaluate/ms | 0.184 | 0.284 | 2.18 | 1059.3 |
| stage/render/create_views | 0.183 | 0.242 | 1.26 | 1057.8 |
| far/select/ms | 0.182 | 0.246 | 0.59 | 1050.7 |
| stage/render/prepare_flush | 0.176 | 0.348 | 0.83 | 1016.8 |
| stage/render/extract_commands | 0.171 | 0.204 | 0.45 | 983.7 |
| bigspace/high_precision/ms | 0.158 | 0.610 | 2.48 | 913.3 |
| anim/evaluate_cpu/ms | 0.142 | 0.178 | 0.42 | 820.6 |
| stage/render/prepare_views | 0.130 | 0.166 | 0.35 | 751.5 |
| controller/move/ms | 0.125 | 0.247 | 0.48 | 720.7 |
| stage/main/last | 0.121 | 0.197 | 2.57 | 699.0 |
| stage/render/phase_sort | 0.108 | 0.141 | 0.46 | 624.1 |
| render/acquire/ms | 0.090 | 0.115 | 0.30 | 518.4 |
| stage/render/cleanup | 0.079 | 0.207 | 0.91 | 453.4 |
| pipelines/compile_ms | 0.050 | 0.000 | 112.51 | 286.2 |
| physics/step/ms | 0.049 | 0.086 | 0.63 | 281.6 |
| pipelines/front_ms | 0.048 | 0.000 | 108.84 | 274.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.070 | 0.25 | 267.2 |
| physics/insert/ms | 0.032 | 0.375 | 0.80 | 186.4 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.46 | 171.6 |
| weather/map/ms | 0.025 | 0.033 | 0.23 | 145.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.24 | 85.0 |
| anim/apply/ms | 0.011 | 0.020 | 0.16 | 63.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.28 | 61.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.20 | 60.1 |
| controller/avoid/ms | 0.010 | 0.014 | 0.18 | 60.0 |
| script/update/ms | 0.007 | 0.009 | 0.14 | 38.6 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 28.4 |
| stream/despawn/ms | 0.005 | 0.000 | 0.94 | 28.3 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 28.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 25.2 |
| controller/write/ms | 0.003 | 0.005 | 0.04 | 20.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.22 | 11.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.9 |
| stage/render/prepare | 0.001 | 0.002 | 0.13 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1269 | 2218 | 641 |
| crowd/c0/meshes/count | count | 349 | 633 | 174 |
| crowd/c1/count | count | 146 | 154 | 147 |
| crowd/c1/joints/count | count | 5477 | 5882 | 5555 |
| crowd/c1/meshes/count | count | 1157 | 1254 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4615 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.68 | 3.44 | 1.71 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.24 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 1.68 | 3.44 | 1.71 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.46 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.41 | 25.28 | 9.20 |
| gpu/timed/ms | ms | 8.70 | 12.06 | 9.20 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 248 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3904 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3904 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 4100 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4100 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 200 | 5 |
| phases/shadow/bins | count | 10.8 k | 12.6 k | 8218 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 29.0 k | 32.5 k | 27.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 193 | 239 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 20 | 561 | 6 |
| preprocess/bind_groups_created | count | 20 | 561 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 233 | 125 |
| process/cpu_usage | % | 25.58 | 36.84 | 21.63 |
| process/mem_usage | GiB | 5.41 | 5.56 | 5.56 |
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
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.71 | 48.36 | 39.12 |
| system/mem_usage | % | 50.42 | 50.96 | 50.89 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 69 | 249 | 55 |
| views/all/bins | count | 19.4 k | 21.2 k | 17.8 k |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 56.9 k | 60.9 k | 57.0 k |
| views/all/pipelines | count | 240 | 288 | 66 |
| views/all/pipelines_distinct | count | 70 | 76 | 61 |
| views/all/work_items | count | 41.4 k | 57.9 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 34 | 5 |
| views/cascade/bins | count | 8064 | 8796 | 8218 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 26.7 k | 28.3 k | 27.4 k |
| views/cascade/pipelines | count | 25 | 34 | 20 |
| views/cascade/work_items | count | 11.8 k | 28.0 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 3163 | 4173 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3418 | 4475 | 0 |
| views/lamp_face/pipelines | count | 168 | 211 | 0 |
| views/lamp_face/work_items | count | 0 | 4066 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8674 | 9602 | 9602 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 47 | 51 | 46 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 53.58 | 289 | 309.1 k |
| anim/events/count | count | 0.10 | 3 | 552 |
| assets/Image/added | count | 0.10 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.70 | 152 | 4061 |
| assets/Mesh/removed | count | 0.84 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.76 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.81 | 1562 | 4675 |
| encode//count | count | 39.56 | 272 | 228.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5768 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5768 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5768 |
| physics/inserts | count | 0.39 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5869 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1287 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.6 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2516.3 | main/busy/ms 2490, stage/main/post_update 2388, stage/main/update 95.76, surface/advance_cpu_ms 68.40, stage/main/pre_update 2.70, stage/main/last 1.24, surface/cell_upload_ms 0.83, stream/drive/ms 0.57, stage/main/first 0.49, stage/main/fixed_main_loop 0.44, stream/tick/ms 0.12, weather/map/ms 0.11 |
| warm-up | 3.26 | 748.6 | main/busy/ms 2633, stage/main/post_update 2417, render/thread/ms 720, render/busy/ms 548, stage/render/prepare_assets 513, stage/main/update 204, stage/render/prepare_views 169, render/acquire/ms 168, surface/advance_cpu_ms 68.40, stage/render/prepare_resources 21.69, stage/render/render 11.72, encode//sum 11.29 |
| warm-up | 3.35 | 87.6 | render/thread/ms 790, render/busy/ms 617, stage/render/prepare_assets 558, stage/render/prepare_views 169, main/busy/ms 169, render/acquire/ms 168, pipelines/compile_ms 155, pipelines/front_ms 143, stage/main/update 115, stage/main/post_update 42.54, stage/render/render 26.95, stage/render/prepare_resources 26.81 |
| warm-up | 3.45 | 79.8 | pipelines/wait_ms 1338, pipelines/compile_ms 552, pipelines/front_ms 509, render/thread/ms 73.84, render/busy/ms 69.47, main/busy/ms 59.20, pipelines/driver_ms 42.89, stage/main/post_update 36.25, stage/render/prepare_assets 26.60, stage/render/render 19.10, stage/main/update 17.59, stage/render/prepare_meshes 15.99 |
| warm-up | 3.56 | 107.5 | pipelines/wait_ms 5264, pipelines/compile_ms 1749, pipelines/front_ms 1644, render/thread/ms 166, render/busy/ms 163, pipelines/driver_ms 104, main/busy/ms 75.16, stage/main/post_update 54.64, stage/render/prepare_resources 54.12, stage/render/prepare_batch_phases 27.92, stage/render/prepare_assets 26.24, stage/render/render 19.15 |
| warm-up | 3.61 | 55.1 | pipelines/wait_ms 4072, pipelines/compile_ms 1889, pipelines/front_ms 1785, render/thread/ms 160, render/busy/ms 157, main/busy/ms 143, pipelines/driver_ms 104, stage/main/update 62.44, stage/render/prepare_resources 54.23, stage/main/post_update 52.75, stage/render/prepare_assets 29.19, stage/render/prepare_batch_phases 26.57 |
| warm-up | 3.78 | 169.9 | pipelines/compile_ms 1759, pipelines/front_ms 1593, pipelines/wait_ms 441, render/thread/ms 177, render/busy/ms 175, pipelines/driver_ms 167, main/busy/ms 132, stage/render/prepare_assets 113, stage/main/update 70.55, stage/render/render 54.28, render/graph/ms 51.90, stage/render/graph_submit 38.97 |
| warm-up | 3.84 | 39.6 | pipelines/compile_ms 1215, pipelines/front_ms 1153, pipelines/wait_ms 217, pipelines/driver_ms 61.80, render/thread/ms 55.72, render/busy/ms 35.05, stage/render/render 30.87, main/busy/ms 26.91, render/present/ms 20.37, stage/render/prepare_assets 16.60, terrain/update/ms 12.99, stage/main/post_update 12.58 |
| warm-up | 3.97 | 65.4 | pipelines/compile_ms 1210, pipelines/front_ms 1157, pipelines/driver_ms 52.20, main/busy/ms 50.89, render/thread/ms 46.39, render/busy/ms 44.43, stage/main/update 39.22, stream/drive/ms 34.58, stage/render/prepare_assets 28.70, stage/render/render 10.82, render/graph/ms 8.83, encode//sum 6.89 |
| warm-up | 4.01 | 37.4 | pipelines/compile_ms 1142, pipelines/front_ms 1089, render/thread/ms 63.39, render/busy/ms 61.65, pipelines/driver_ms 53.16, stage/render/prepare_assets 45.37, main/busy/ms 41.49, stage/main/update 25.36, stream/drive/ms 20.71, stage/render/render 10.69, stage/main/post_update 8.74, render/graph/ms 8.65 |
| warm-up | 4.05 | 41.6 | pipelines/compile_ms 516, pipelines/front_ms 496, render/thread/ms 69.46, render/busy/ms 66.69, main/busy/ms 64.58, stage/render/prepare_assets 38.50, stage/main/update 30.40, stream/drive/ms 21.81, stage/main/post_update 20.76, stage/render/render 20.17, pipelines/driver_ms 19.32, render/graph/ms 17.14 |
| warm-up | 4.10 | 48.2 | pipelines/compile_ms 663, pipelines/front_ms 642, main/busy/ms 77.56, render/thread/ms 71.97, render/busy/ms 68.51, stage/main/update 44.99, stage/render/prepare_assets 40.00, stream/drive/ms 29.37, stage/render/render 22.10, pipelines/driver_ms 20.85, stage/main/post_update 19.51, render/graph/ms 18.51 |
| warm-up | 4.18 | 82.1 | pipelines/compile_ms 958, pipelines/front_ms 929, render/thread/ms 84.40, render/busy/ms 81.85, stage/render/prepare_assets 62.43, main/busy/ms 58.40, stage/main/update 41.04, stream/drive/ms 28.93, pipelines/driver_ms 28.30, stage/render/render 14.05, render/graph/ms 11.19, stage/main/post_update 9.80 |
| warm-up | 4.22 | 38.3 | pipelines/compile_ms 968, pipelines/front_ms 941, render/thread/ms 76.16, render/busy/ms 74.43, stage/render/prepare_assets 58.18, main/busy/ms 35.89, pipelines/driver_ms 26.52, stage/main/update 13.32, stage/main/post_update 11.20, stage/render/render 9.58, render/graph/ms 7.48, encode//sum 6.56 |
| warm-up | 4.29 | 66.0 | pipelines/compile_ms 502, pipelines/front_ms 480, render/thread/ms 78.89, render/busy/ms 76.94, main/busy/ms 53.28, stage/render/prepare_assets 43.91, stage/main/update 25.81, stage/render/render 22.18, pipelines/driver_ms 21.40, render/graph/ms 19.81, stage/main/post_update 14.82, encode//sum 6.46 |
| warm-up | 4.37 | 78.6 | pipelines/compile_ms 752, pipelines/front_ms 723, render/thread/ms 121, render/busy/ms 119, main/busy/ms 92.84, stage/main/update 60.41, stage/render/prepare_assets 44.00, pipelines/wait_ms 30.54, pipelines/driver_ms 29.21, stage/render/render 25.15, render/graph/ms 22.67, stage/main/post_update 14.62 |
| warm-up | 4.45 | 89.0 | pipelines/compile_ms 1210, pipelines/front_ms 1177, pipelines/wait_ms 180, render/thread/ms 133, render/busy/ms 131, main/busy/ms 97.59, stage/main/update 52.97, stage/render/prepare_assets 51.50, pipelines/driver_ms 32.98, stage/main/post_update 29.01, encode//sum 20.48, stage/render/render 16.25 |
| warm-up | 4.59 | 134.3 | pipelines/compile_ms 3586, pipelines/front_ms 3400, pipelines/wait_ms 779, render/thread/ms 186, pipelines/driver_ms 185, render/busy/ms 183, stage/render/prepare_assets 84.74, main/busy/ms 54.23, stage/render/create_views 37.31, encode//sum 34.12, stage/render/render 29.33, render/graph/ms 26.46 |
| warm-up | 4.68 | 88.0 | pipelines/compile_ms 3695, pipelines/front_ms 3491, pipelines/wait_ms 822, render/thread/ms 206, render/busy/ms 203, pipelines/driver_ms 203, main/busy/ms 63.68, stage/render/prepare_assets 60.22, stage/render/create_views 46.59, encode//sum 44.42, stage/render/render 33.05, render/graph/ms 29.91 |
| warm-up | 4.72 | 41.1 | pipelines/compile_ms 1820, pipelines/front_ms 1746, pipelines/wait_ms 533, main/busy/ms 113, render/thread/ms 113, render/busy/ms 110, pipelines/driver_ms 74.42, stage/main/update 64.91, stream/drive/ms 53.28, encode//sum 31.16, stage/render/prepare_assets 21.21, stage/render/render 20.91 |
| warm-up | 4.80 | 84.8 | pipelines/compile_ms 2024, pipelines/front_ms 1961, pipelines/wait_ms 494, main/busy/ms 82.01, pipelines/driver_ms 62.91, stage/main/update 58.67, render/thread/ms 57.90, render/busy/ms 56.22, stream/drive/ms 51.93, stage/render/prepare_assets 33.72, stage/render/render 13.57, encode//sum 12.77 |
| warm-up | 4.83 | 28.3 | pipelines/compile_ms 1110, pipelines/front_ms 1080, pipelines/wait_ms 157, main/busy/ms 108, stage/main/update 86.97, render/thread/ms 53.46, render/busy/ms 51.80, stream/drive/ms 50.27, pipelines/driver_ms 29.50, stage/render/prepare_assets 25.32, stage/render/render 20.02, render/graph/ms 17.97 |
| warm-up | 4.98 | 144.2 | pipelines/compile_ms 860, pipelines/front_ms 831, main/busy/ms 115, stage/main/update 90.49, render/thread/ms 80.62, render/busy/ms 78.82, stream/drive/ms 50.17, stage/render/prepare_assets 41.16, pipelines/driver_ms 28.78, stage/render/render 21.35, render/graph/ms 19.22, encode//sum 15.09 |
| warm-up | 5.01 | 39.4 | pipelines/compile_ms 1893, pipelines/front_ms 1836, render/thread/ms 89.84, render/busy/ms 87.86, main/busy/ms 85.72, stage/main/update 63.40, pipelines/driver_ms 57.07, stream/drive/ms 43.51, stage/render/prepare_assets 43.27, stage/render/render 20.80, render/graph/ms 18.32, encode//sum 17.29 |
| warm-up | 5.09 | 75.6 | pipelines/compile_ms 1199, pipelines/front_ms 1160, main/busy/ms 84.43, stage/main/update 67.50, render/thread/ms 65.48, render/busy/ms 63.51, stream/drive/ms 44.41, pipelines/driver_ms 38.37, stage/render/prepare_assets 25.51, stage/render/render 18.91, render/graph/ms 16.45, encode//sum 13.93 |
| warm-up | 5.12 | 33.0 | pipelines/compile_ms 388, pipelines/front_ms 372, main/busy/ms 63.33, render/thread/ms 56.92, render/busy/ms 55.28, stage/main/update 48.91, stream/drive/ms 34.28, stage/render/prepare_assets 24.59, pipelines/driver_ms 16.17, encode//sum 13.93, stage/render/render 12.17, render/graph/ms 10.10 |
| warm-up | 5.17 | 44.1 | pipelines/compile_ms 720, pipelines/front_ms 696, main/busy/ms 57.15, render/thread/ms 45.22, render/busy/ms 43.59, stage/main/update 41.69, stream/drive/ms 34.26, pipelines/driver_ms 23.31, encode//sum 15.25, stage/render/prepare_assets 13.03, stage/render/render 12.04, render/graph/ms 9.90 |
| warm-up | 5.34 | 52.2 | pipelines/compile_ms 1037, pipelines/front_ms 1009, render/thread/ms 53.10, render/busy/ms 51.17, main/busy/ms 42.68, pipelines/driver_ms 28.04, stage/render/prepare_assets 27.22, stage/main/update 23.08, encode//sum 16.49, stage/render/render 15.01, stage/main/post_update 13.29, render/graph/ms 12.48 |
| warm-up | 5.37 | 31.8 | pipelines/compile_ms 854, pipelines/front_ms 831, render/thread/ms 59.99, render/busy/ms 58.46, main/busy/ms 37.93, stage/render/prepare_assets 32.92, pipelines/driver_ms 22.92, stage/main/update 19.86, encode//sum 14.85, stage/render/render 13.08, stage/main/post_update 12.24, stream/drive/ms 12.17 |
| warm-up | 5.40 | 30.4 | pipelines/compile_ms 219, pipelines/front_ms 212, render/thread/ms 51.73, render/busy/ms 50.15, main/busy/ms 39.13, stage/render/prepare_assets 25.24, stage/main/update 18.06, stage/main/post_update 14.77, encode//sum 14.47, stage/render/render 12.75, stream/drive/ms 11.27, render/graph/ms 10.54 |
| warm-up | 5.44 | 34.8 | pipelines/compile_ms 402, pipelines/front_ms 389, render/thread/ms 56.32, render/busy/ms 54.78, stage/render/prepare_assets 35.10, main/busy/ms 32.82, encode//sum 14.75, stage/main/post_update 14.37, pipelines/driver_ms 13.56, stage/main/update 12.90, stage/render/render 11.01, render/graph/ms 8.79 |
| warm-up | 5.49 | 29.9 | pipelines/compile_ms 344, pipelines/front_ms 333, render/thread/ms 47.72, render/busy/ms 45.59, main/busy/ms 25.09, stage/render/prepare_assets 19.89, stage/render/render 19.18, render/graph/ms 16.38, encode//sum 13.48, stage/main/post_update 11.56, pipelines/driver_ms 10.68, stage/render/camera_driver 10.46 |
| warm-up | 5.72 | 29.2 | render/thread/ms 40.58, render/busy/ms 39.25, main/busy/ms 33.51, stage/main/update 18.33, encode//sum 15.57, stage/render/prepare_assets 13.69, stage/main/post_update 10.88, stage/render/render 10.70, render/graph/ms 9.06, stream/drive/ms 6.97, stage/render/graph_submit 6.53, encode/submit 4.07 |
| warm-up | 6.12 | 30.4 | render/thread/ms 36.52, render/busy/ms 35.56, main/busy/ms 24.27, stage/render/prepare_assets 15.07, encode//sum 13.75, stage/main/update 10.02, stage/render/render 8.03, stage/main/post_update 7.98, render/graph/ms 6.71, stage/render/graph_submit 4.90, stage/render/specialize 4.41, stage/main/pre_update 3.37 |
| warm-up | 6.33 | 28.5 | render/thread/ms 46.96, render/busy/ms 45.28, main/busy/ms 34.64, encode//sum 21.98, stage/render/render 16.23, stage/main/update 15.12, render/graph/ms 13.76, stage/main/post_update 13.07, stage/render/graph_submit 9.91, stage/render/specialize 8.09, encode/submit 5.60, stage/render/prepare_assets 5.43 |
| warm-up | 6.39 | 33.8 | pipelines/compile_ms 72.64, pipelines/front_ms 68.41, render/thread/ms 42.66, render/busy/ms 41.59, main/busy/ms 27.93, stage/render/prepare_assets 20.36, encode//sum 17.21, stage/main/update 12.03, stage/main/post_update 10.29, stage/render/render 8.92, render/graph/ms 7.37, stage/render/graph_submit 5.17 |
| warm-up | 6.47 | 32.3 | render/thread/ms 46.88, render/busy/ms 45.77, main/busy/ms 35.34, stage/render/prepare_assets 17.93, encode//sum 17.17, stage/main/update 16.58, stage/main/post_update 12.66, stage/render/render 9.77, render/graph/ms 8.07, stage/render/graph_submit 5.37, stream/drive/ms 4.14, stage/render/specialize 4.12 |
| warm-up | 6.62 | 35.5 | render/thread/ms 48.69, render/busy/ms 47.43, main/busy/ms 38.55, stage/main/update 24.53, stage/render/prepare_assets 17.97, encode//sum 17.06, stream/drive/ms 13.86, stage/render/render 10.17, stage/main/post_update 9.76, render/graph/ms 8.48, stage/render/specialize 7.12, stage/render/graph_submit 6.34 |
| warm-up | 6.66 | 35.7 | render/thread/ms 65.75, render/busy/ms 64.48, main/busy/ms 33.45, stage/render/prepare_assets 32.19, stage/main/update 20.16, encode//sum 16.99, stream/drive/ms 13.24, stage/render/render 9.84, stage/main/post_update 8.85, render/graph/ms 8.09, stage/render/specialize 8.02, stage/render/graph_submit 6.00 |
| measure | 18.17 | 28.1 | main/busy/ms 33.68, render/thread/ms 21.30, encode//sum 21.19, stage/main/update 21.05, render/busy/ms 20.23, stream/drive/ms 17.36, stage/render/render 12.80, render/graph/ms 11.10, stage/main/post_update 9.92, stage/render/graph_submit 8.62, encode/submit 4.48, encode/finish_wall 4.21 |
| measure | 59.04 | 28.0 | render/thread/ms 40.84, render/busy/ms 39.55, main/busy/ms 26.61, encode//sum 17.73, stage/main/post_update 14.69, stage/render/prepare_assets 13.85, stage/render/render 11.83, render/graph/ms 10.19, stage/render/graph_submit 7.42, stage/main/update 6.55, encode/finish_wall 3.77, encode/submit 3.75 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_h1\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 5%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **80%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.8 W · 72 °C (max 77) · VRAM 5260 MiB · throttled 100% of samples
