# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5547 frames over 65.7 s · **median 11.60 ms (86 fps)** · mean 11.84 · p95 14.25 · p99 16.73 (1% low 60 fps) · worst 31.47 ms · 45 spikes over 1.5× median · 25.6 stutters/min (work 25.6/min)
- warm-up: 323 frames over 9.4 s, worst 2705.4 ms
- hitches (> 29.0 ms): **3** measured, 40 in warm-up
- timed GPU passes: 7.80 ms of a 11.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.60 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 31.47 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.037 | 2.096 | 2.367 | 0.97 | 0.173 |
| main_opaque_pass_3d | 1.621 | 1.667 | 2.182 | 0.97 | 0.055 |
| per_view_shadow_pass_cascade1 | 0.597 | 0.614 | 0.914 | 0.97 | 0.043 |
| early prepass | 0.520 | 0.535 | 0.771 | 0.97 | 0.081 |
| bloom | 0.314 | 0.323 | 0.327 | 0.97 | 0.064 |
| atmosphere_luts | 0.311 | 0.320 | 0.383 | 0.97 | 0.003 |
| water_velocity | 0.282 | 0.290 | 0.355 | 0.97 | 0.002 |
| water_sspr | 0.202 | 0.208 | 0.205 | 0.97 | 0.002 |
| ssao | 0.191 | 0.196 | 0.196 | 0.97 | 0.007 |
| uniform_allocation | 0.172 | 0.177 | 0.466 | 0.97 | 0.028 |
| atmos_froxel_inject | 0.145 | 0.150 | 0.150 | 0.97 | 0.001 |
| auxiliary_views | 0.138 | 0.142 | 0.525 | 0.97 | 0.129 |
| clouds_trace | 0.138 | 0.142 | 0.233 | 0.97 | 0.001 |
| water_fft | 0.132 | 0.135 | 0.139 | 0.97 | 0.003 |
| upscaling | 0.119 | 0.122 | 0.131 | 0.97 | 0.004 |
| bin_unpacking | 0.118 | 0.122 | 0.129 | 0.97 | 0.021 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.97 | 0.002 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.084 | 0.162 | 0.97 | 0.008 |
| clustering | 0.064 | 0.066 | 0.068 | 0.97 | 0.019 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.436 | 0.719 | 0.11 | 0.001 |
| surface_trail | 0.043 | 0.058 | 0.057 | 0.74 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.044 | 0.045 | 0.97 | 0.004 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.223 | 0.234 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.015 | 0.017 | 0.97 | 0.005 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.97 | 0.114 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.017 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.044 | 0.049 | 0.01 | 0.014 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.018 | 0.020 | 0.01 | 0.010 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.836 | 17.273 | 49.91 | 65656.4 |
| render/thread/ms | 10.542 | 12.865 | 24.87 | 58478.0 |
| render/busy/ms | 9.945 | 12.285 | 24.26 | 55165.4 |
| main/busy/ms | 9.504 | 11.716 | 28.58 | 52718.6 |
| stage/render/render | 6.715 | 8.375 | 16.33 | 37250.9 |
| stage/main/post_update | 6.029 | 7.941 | 13.41 | 33442.2 |
| render/graph/ms | 5.931 | 7.566 | 15.28 | 32900.6 |
| stage/render/graph_submit | 4.773 | 6.132 | 12.14 | 26473.8 |
| encode/submit | 2.432 | 3.239 | 6.30 | 13490.1 |
| encode/finish_wall | 2.418 | 3.222 | 5.90 | 13411.9 |
| stage/main/update | 2.197 | 2.658 | 22.00 | 12186.7 |
| stage/render/camera_driver | 1.102 | 1.470 | 4.13 | 6113.5 |
| stage/render/prepare_write_phase_buffers | 0.845 | 1.105 | 2.23 | 4688.8 |
| bigspace/total/ms | 0.653 | 2.200 | 3.75 | 3624.1 |
| stage/render/prepare_assets | 0.561 | 0.739 | 10.61 | 3112.8 |
| render/present/ms | 0.502 | 0.639 | 2.15 | 2786.6 |
| stage/render/prepare_resources | 0.458 | 0.570 | 1.04 | 2540.3 |
| stage/main/first | 0.403 | 0.532 | 5.48 | 2233.1 |
| stage/render/prepare_meshes | 0.389 | 0.511 | 2.25 | 2156.5 |
| stage/main/pre_update | 0.337 | 0.511 | 1.17 | 1869.6 |
| terrain/update/ms | 0.306 | 0.392 | 1.84 | 1696.2 |
| stage/render/specialize | 0.285 | 0.424 | 2.56 | 1578.7 |
| bigspace/low_precision/ms | 0.278 | 1.233 | 3.06 | 1542.5 |
| terrain/select/ms | 0.272 | 0.351 | 0.75 | 1510.8 |
| controller/feed/ms | 0.271 | 0.348 | 1.06 | 1501.2 |
| stream/drive/ms | 0.251 | 0.344 | 20.01 | 1394.3 |
| stage/render/prepare_bind_groups | 0.247 | 0.421 | 2.79 | 1371.8 |
| stage/main/fixed_main_loop | 0.232 | 0.384 | 1.57 | 1287.9 |
| bigspace/high_precision/ms | 0.221 | 1.166 | 3.28 | 1224.3 |
| stage/render/queue | 0.218 | 0.368 | 2.71 | 1209.2 |
| anim/evaluate/ms | 0.196 | 0.312 | 3.64 | 1088.1 |
| stream/tick/ms | 0.193 | 0.262 | 0.67 | 1068.1 |
| far/select/ms | 0.189 | 0.257 | 0.61 | 1047.3 |
| stage/render/create_views | 0.184 | 0.246 | 1.29 | 1020.7 |
| stage/render/prepare_flush | 0.181 | 0.371 | 1.01 | 1004.2 |
| stage/render/extract_commands | 0.178 | 0.219 | 0.53 | 989.6 |
| stage/render/prepare_batch_phases | 0.166 | 0.237 | 0.88 | 919.9 |
| anim/evaluate_cpu/ms | 0.142 | 0.183 | 0.41 | 789.8 |
| stage/render/prepare_views | 0.137 | 0.182 | 0.40 | 762.5 |
| controller/move/ms | 0.127 | 0.224 | 0.57 | 707.2 |
| stage/main/last | 0.125 | 0.204 | 3.09 | 695.0 |
| render/acquire/ms | 0.095 | 0.127 | 0.34 | 526.0 |
| stage/render/phase_sort | 0.084 | 0.117 | 0.39 | 464.9 |
| pipelines/compile_ms | 0.053 | 0.000 | 119.98 | 291.5 |
| physics/step/ms | 0.051 | 0.086 | 0.57 | 280.8 |
| pipelines/front_ms | 0.050 | 0.000 | 116.07 | 279.3 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.078 | 0.42 | 268.8 |
| physics/insert/ms | 0.037 | 0.375 | 0.94 | 203.5 |
| stage/render/cleanup | 0.035 | 0.113 | 0.73 | 195.5 |
| encode/resolve_diagnostics/sum | 0.030 | 0.042 | 0.53 | 166.9 |
| weather/map/ms | 0.025 | 0.033 | 0.35 | 140.5 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.15 | 80.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.32 | 60.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.15 | 60.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.28 | 59.1 |
| controller/avoid/ms | 0.010 | 0.014 | 0.17 | 57.8 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 38.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 29.6 |
| script/ui/ms | 0.005 | 0.007 | 0.06 | 28.8 |
| script/input/ms | 0.005 | 0.006 | 0.20 | 26.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 25.9 |
| controller/write/ms | 0.004 | 0.005 | 0.15 | 20.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.67 | 14.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.03 | 12.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.13 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.11 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 15 |
| crowd/c0/joints/count | count | 1277 | 2262 | 707 |
| crowd/c0/meshes/count | count | 353 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 145 |
| crowd/c1/joints/count | count | 5524 | 5982 | 5457 |
| crowd/c1/meshes/count | count | 1163 | 1284 | 1144 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4268 | 9879 | 6135 |
| crowd/joints_uploaded/count | count | 4257 | 6643 | 6135 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.97 | 3.80 | 2.46 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.15 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.02 |
| encode/finish_critical | ms | 1.97 | 3.80 | 2.46 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.02 | 28.58 | 11.45 |
| gpu/timed/ms | ms | 8.74 | 10.41 | 9.27 |
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
| phases/alpha_mask3d/items | count | 818 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 818 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1281 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3416 | 5118 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1281 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3416 | 5118 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 25 | 213 | 19 |
| phases/shadow/bins | count | 4290 | 5490 | 1780 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9515 | 12.0 k | 4876 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 194 | 241 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 10 | 584 | 11 |
| preprocess/bind_groups_created | count | 10 | 584 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 160 | 247 | 149 |
| process/cpu_usage | % | 26.34 | 41.35 | 27.88 |
| process/mem_usage | GiB | 5.40 | 5.56 | 5.56 |
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
| started_load_count |  | 4915 | 5008 | 5008 |
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
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.54 | 55.27 | 42.75 |
| system/mem_usage | % | 68.14 | 69.02 | 69.02 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 76 | 260 | 69 |
| views/all/bins | count | 6939 | 8372 | 5278 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.6 k | 24.2 k | 12.5 k |
| views/all/pipelines | count | 238 | 288 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 16.3 k | 21.9 k | 12.8 k |
| views/cascade/batch_sets | count | 24 | 33 | 19 |
| views/cascade/bins | count | 2085 | 2678 | 1780 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6995 | 8999 | 4876 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 6995 | 8999 | 4876 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2201 | 2825 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2459 | 3120 | 0 |
| views/lamp_face/pipelines | count | 170 | 214 | 0 |
| views/lamp_face/work_items | count | 0 | 2884 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2860 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8458 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8617 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 54.87 | 246 | 304.3 k |
| anim/events/count | count | 0.10 | 3 | 564 |
| assets/Image/added | count | 0.11 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.87 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.91 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 1389 | 4675 |
| encode//count | count | 39.55 | 273 | 219.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5547 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5547 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5547 |
| physics/inserts | count | 0.41 | 111 | 2259 |
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
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6103 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1338 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.0 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.71 | 2705.4 | main/busy/ms 2675, stage/main/post_update 2573, stage/main/update 95.95, surface/advance_cpu_ms 67.09, stage/main/pre_update 2.31, stage/main/last 1.14, surface/cell_upload_ms 0.78, stage/main/first 0.49, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.38, weather/map/ms 0.11, stream/tick/ms 0.08 |
| warm-up | 3.47 | 766.9 | main/busy/ms 2815, stage/main/post_update 2597, render/thread/ms 733, render/busy/ms 561, stage/render/prepare_assets 523, stage/main/update 205, stage/render/prepare_views 170, render/acquire/ms 169, surface/advance_cpu_ms 67.09, stage/render/prepare_resources 22.98, encode//sum 11.01, stage/render/render 10.84 |
| warm-up | 3.56 | 84.3 | render/thread/ms 797, render/busy/ms 623, stage/render/prepare_assets 568, stage/render/prepare_views 171, render/acquire/ms 170, main/busy/ms 168, stage/main/update 115, stage/main/post_update 39.19, stage/render/prepare_resources 26.33, stage/render/render 21.53, terrain/update/ms 18.17, encode//sum 15.92 |
| warm-up | 3.59 | 34.1 | pipelines/compile_ms 240, pipelines/front_ms 217, render/thread/ms 96.57, render/busy/ms 92.96, pipelines/wait_ms 86.35, main/busy/ms 64.56, stage/render/prepare_assets 46.16, stage/main/post_update 44.81, stage/render/render 36.14, render/graph/ms 32.16, terrain/update/ms 25.93, pipelines/driver_ms 23.23 |
| warm-up | 3.64 | 51.7 | pipelines/wait_ms 563, pipelines/compile_ms 524, pipelines/front_ms 487, render/thread/ms 78.07, render/busy/ms 75.60, main/busy/ms 64.07, stage/main/post_update 52.41, pipelines/driver_ms 37.44, stage/render/render 31.90, render/graph/ms 26.42, terrain/update/ms 15.01, pipelines/lock_ms 13.34 |
| warm-up | 3.74 | 102.5 | pipelines/wait_ms 4225, pipelines/compile_ms 1156, pipelines/front_ms 1104, render/thread/ms 146, render/busy/ms 143, main/busy/ms 94.99, stage/main/update 53.26, pipelines/driver_ms 51.25, stage/render/prepare_resources 44.12, encode//sum 37.75, stage/main/post_update 34.08, stage/render/render 32.34 |
| warm-up | 3.82 | 73.7 | pipelines/wait_ms 7213, pipelines/compile_ms 2053, pipelines/front_ms 1928, render/thread/ms 172, render/busy/ms 169, pipelines/driver_ms 125, main/busy/ms 97.01, stage/main/update 73.06, stage/render/prepare_resources 52.60, encode//sum 45.57, stage/render/prepare_assets 38.96, stage/render/render 35.57 |
| warm-up | 3.85 | 31.8 | pipelines/wait_ms 3695, pipelines/compile_ms 1842, pipelines/front_ms 1730, pipelines/driver_ms 112, render/thread/ms 98.17, render/busy/ms 95.94, main/busy/ms 92.56, stage/main/update 73.49, stage/render/prepare_assets 50.64, stage/render/prepare_resources 17.98, stage/render/render 14.98, encode//sum 14.15 |
| warm-up | 3.96 | 107.3 | pipelines/compile_ms 1620, pipelines/front_ms 1552, pipelines/wait_ms 510, render/thread/ms 93.74, render/busy/ms 91.59, main/busy/ms 83.58, stage/render/prepare_assets 71.93, pipelines/driver_ms 67.89, stage/main/update 58.68, stage/main/post_update 19.16, stage/render/render 13.47, render/graph/ms 11.00 |
| warm-up | 4.12 | 70.3 | pipelines/compile_ms 613, pipelines/front_ms 590, render/thread/ms 55.71, render/busy/ms 54.03, main/busy/ms 50.49, stage/main/update 39.71, stage/render/prepare_assets 37.99, stream/drive/ms 35.68, pipelines/driver_ms 22.99, stage/render/render 11.56, encode//sum 11.14, render/graph/ms 9.42 |
| warm-up | 4.22 | 72.2 | pipelines/compile_ms 643, pipelines/front_ms 621, main/busy/ms 53.68, render/thread/ms 53.08, render/busy/ms 50.54, stage/main/update 40.75, stream/drive/ms 34.68, stage/render/prepare_assets 28.57, pipelines/driver_ms 22.57, stage/render/render 13.87, render/graph/ms 10.84, stage/render/graph_submit 8.19 |
| warm-up | 4.25 | 36.4 | pipelines/compile_ms 497, pipelines/front_ms 476, render/thread/ms 70.65, render/busy/ms 67.59, main/busy/ms 47.33, stage/render/render 27.53, stage/main/update 27.50, stage/render/prepare_assets 27.41, render/graph/ms 23.83, pipelines/driver_ms 20.45, stage/render/graph_submit 16.15, stream/drive/ms 14.78 |
| warm-up | 4.34 | 88.7 | pipelines/compile_ms 717, pipelines/front_ms 692, render/thread/ms 93.45, render/busy/ms 90.78, main/busy/ms 56.85, stage/render/prepare_assets 51.29, stage/main/update 32.21, stage/render/render 26.41, pipelines/driver_ms 24.16, render/graph/ms 23.31, stage/render/graph_submit 16.00, stage/main/post_update 14.67 |
| warm-up | 4.39 | 45.8 | pipelines/compile_ms 1335, pipelines/front_ms 1299, render/thread/ms 93.60, render/busy/ms 91.34, stage/render/prepare_assets 68.58, main/busy/ms 46.65, pipelines/driver_ms 36.02, stage/main/update 29.68, stage/render/render 15.73, render/graph/ms 13.03, stage/render/graph_submit 10.71, stage/main/post_update 7.79 |
| warm-up | 4.48 | 88.5 | pipelines/compile_ms 806, pipelines/front_ms 785, render/thread/ms 85.85, render/busy/ms 83.62, stage/render/prepare_assets 60.98, main/busy/ms 49.57, stage/main/update 30.35, pipelines/wait_ms 22.40, pipelines/driver_ms 20.25, stage/render/render 14.31, stage/main/post_update 11.81, render/graph/ms 11.52 |
| warm-up | 4.58 | 102.0 | pipelines/compile_ms 2077, pipelines/front_ms 1982, render/thread/ms 129, render/busy/ms 127, pipelines/driver_ms 95.06, main/busy/ms 80.37, pipelines/wait_ms 61.44, stage/render/prepare_assets 49.61, stage/render/prepare_resources 39.93, stage/main/post_update 36.46, stage/main/update 33.02, stage/render/render 18.83 |
| warm-up | 4.66 | 50.7 | pipelines/compile_ms 84.04, pipelines/front_ms 80.27, render/thread/ms 58.16, render/busy/ms 56.39, main/busy/ms 39.87, stage/render/prepare_assets 28.10, pipelines/wait_ms 19.28, stage/main/update 19.23, stage/main/post_update 12.16, encode//sum 12.09, stage/render/render 11.84, render/graph/ms 9.83 |
| warm-up | 4.79 | 128.9 | pipelines/compile_ms 1912, pipelines/front_ms 1860, pipelines/wait_ms 618, render/thread/ms 158, render/busy/ms 140, main/busy/ms 77.90, encode//sum 53.02, pipelines/driver_ms 52.61, stage/main/post_update 41.02, stage/render/prepare_write_phase_buffers 37.42, stage/render/prepare_assets 31.14, stage/main/update 29.53 |
| warm-up | 4.84 | 53.4 | pipelines/compile_ms 2987, pipelines/front_ms 2910, pipelines/wait_ms 809, render/thread/ms 173, main/busy/ms 157, render/busy/ms 155, stage/main/update 109, encode//sum 84.71, pipelines/driver_ms 76.44, stream/drive/ms 61.22, stage/render/prepare_write_phase_buffers 41.32, stage/main/post_update 37.63 |
| warm-up | 4.99 | 145.6 | pipelines/compile_ms 2677, pipelines/front_ms 2609, pipelines/wait_ms 758, main/busy/ms 126, render/thread/ms 124, render/busy/ms 123, stage/main/update 103, pipelines/driver_ms 68.59, stream/drive/ms 61.88, encode//sum 55.20, stage/render/prepare_assets 45.26, stage/render/render 31.53 |
| warm-up | 5.12 | 104.6 | pipelines/compile_ms 579, pipelines/front_ms 555, main/busy/ms 92.56, stage/main/update 68.29, render/thread/ms 66.98, render/busy/ms 65.12, stream/drive/ms 51.32, stage/render/prepare_assets 39.67, pipelines/driver_ms 23.79, stage/main/post_update 14.99, stage/render/render 14.87, encode//sum 14.21 |
| warm-up | 5.19 | 68.2 | pipelines/compile_ms 1220, pipelines/front_ms 1172, render/thread/ms 104, render/busy/ms 103, main/busy/ms 87.65, encode//sum 82.38, stage/main/update 59.78, stage/render/render 48.94, pipelines/driver_ms 48.12, render/graph/ms 46.82, stage/render/prepare_assets 44.74, stage/render/graph_submit 44.34 |
| warm-up | 5.23 | 46.3 | pipelines/compile_ms 903, pipelines/front_ms 870, render/thread/ms 95.76, render/busy/ms 94.21, encode//sum 82.39, main/busy/ms 75.38, stage/main/update 55.09, stage/render/render 50.28, render/graph/ms 48.21, stage/render/graph_submit 45.41, stream/drive/ms 38.17, encode/finish_wall 36.16 |
| warm-up | 5.30 | 43.1 | pipelines/compile_ms 344, pipelines/front_ms 335, main/busy/ms 54.60, stage/main/update 40.91, render/thread/ms 39.61, render/busy/ms 38.18, stream/drive/ms 29.92, encode//sum 16.75, stage/render/render 12.48, stage/render/prepare_assets 12.43, render/graph/ms 10.63, pipelines/driver_ms 8.82 |
| warm-up | 5.42 | 36.3 | pipelines/compile_ms 595, pipelines/front_ms 577, render/thread/ms 54.70, render/busy/ms 52.95, main/busy/ms 40.92, encode//sum 19.10, stage/main/post_update 18.53, pipelines/driver_ms 18.15, stage/main/update 16.14, stage/render/prepare_assets 15.52, stage/render/render 14.88, render/graph/ms 12.51 |
| warm-up | 5.48 | 38.6 | pipelines/compile_ms 902, pipelines/front_ms 875, render/thread/ms 50.97, render/busy/ms 49.32, main/busy/ms 45.76, pipelines/driver_ms 27.09, stage/main/update 22.70, stage/render/render 21.64, render/graph/ms 19.53, encode//sum 17.74, stage/main/post_update 17.67, stage/render/prepare_assets 14.65 |
| warm-up | 5.54 | 31.4 | pipelines/compile_ms 330, pipelines/front_ms 319, render/thread/ms 45.26, render/busy/ms 43.56, main/busy/ms 38.94, stage/main/update 17.91, encode//sum 17.91, stage/render/render 17.73, stage/main/post_update 15.57, render/graph/ms 15.53, stage/render/prepare_assets 15.02, stage/render/graph_submit 12.51 |
| warm-up | 5.57 | 29.6 | pipelines/compile_ms 301, pipelines/front_ms 290, render/thread/ms 51.55, render/busy/ms 49.82, main/busy/ms 38.14, stage/main/update 18.38, encode//sum 16.58, stage/render/render 15.95, render/graph/ms 13.58, stage/main/post_update 12.68, stage/render/prepare_resources 12.29, stage/render/prepare_assets 11.36 |
| warm-up | 5.60 | 30.6 | pipelines/compile_ms 74.06, pipelines/front_ms 68.21, render/thread/ms 54.02, render/busy/ms 52.20, main/busy/ms 44.61, encode//sum 19.63, stage/main/update 19.13, stage/main/post_update 18.18, stage/render/prepare_resources 15.70, stage/render/render 15.57, render/graph/ms 13.26, stage/render/graph_submit 10.34 |
| warm-up | 5.64 | 37.6 | render/thread/ms 59.17, render/busy/ms 57.39, main/busy/ms 48.82, pipelines/compile_ms 30.94, pipelines/front_ms 29.19, encode//sum 22.51, stage/main/update 22.39, stage/main/post_update 21.19, stage/render/render 15.15, stage/render/prepare_assets 12.85, render/graph/ms 12.82, stage/render/prepare_resources 11.67 |
| warm-up | 6.29 | 30.7 | render/thread/ms 48.94, render/busy/ms 47.23, main/busy/ms 27.19, encode//sum 25.35, stage/render/render 16.18, render/graph/ms 13.85, stage/render/prepare_assets 12.86, stage/main/post_update 11.95, stage/render/graph_submit 11.01, stage/main/update 9.81, encode/submit 6.20, stage/render/specialize 6.06 |
| warm-up | 6.32 | 30.4 | render/thread/ms 55.99, render/busy/ms 54.19, main/busy/ms 33.44, encode//sum 24.09, stage/render/render 16.08, stage/render/prepare_assets 15.97, render/graph/ms 13.63, stage/main/post_update 13.29, stage/main/update 12.20, stage/render/graph_submit 10.93, stage/render/specialize 8.25, encode/submit 6.37 |
| warm-up | 6.36 | 41.7 | render/thread/ms 57.08, render/busy/ms 55.89, stage/render/prepare_assets 32.04, main/busy/ms 31.11, encode//sum 17.93, stage/main/update 12.51, stage/main/post_update 11.25, stage/render/render 10.89, render/graph/ms 9.14, stage/render/graph_submit 7.05, stream/drive/ms 4.09, encode/submit 3.86 |
| warm-up | 6.39 | 29.6 | render/thread/ms 56.46, render/busy/ms 55.26, stage/render/prepare_assets 34.14, main/busy/ms 28.05, encode//sum 17.81, stage/main/post_update 11.75, stage/main/update 10.89, stage/render/render 10.85, render/graph/ms 9.08, stage/render/graph_submit 7.03, encode/submit 3.75, stream/drive/ms 3.68 |
| warm-up | 6.47 | 30.2 | render/thread/ms 49.39, render/busy/ms 48.19, main/busy/ms 36.54, stage/render/prepare_assets 26.28, encode//sum 19.80, stage/main/update 18.65, stage/main/post_update 13.58, stage/render/render 9.85, render/graph/ms 8.30, stage/render/graph_submit 6.17, stream/drive/ms 3.56, encode/finish_wall 3.56 |
| warm-up | 6.63 | 29.6 | pipelines/compile_ms 77.04, pipelines/front_ms 70.76, render/thread/ms 44.77, render/busy/ms 43.33, main/busy/ms 37.88, encode//sum 35.02, stage/render/render 19.20, render/graph/ms 17.21, stage/main/post_update 16.69, stage/main/update 14.99, stage/render/graph_submit 12.69, stage/render/prepare_assets 8.71 |
| warm-up | 6.73 | 33.7 | main/busy/ms 44.66, render/thread/ms 34.02, render/busy/ms 32.59, stage/main/update 24.63, encode//sum 19.22, stream/drive/ms 14.13, stage/main/post_update 14.01, stage/render/render 11.99, render/graph/ms 9.80, stage/render/graph_submit 7.58, stage/render/prepare_assets 7.39, encode/finish_wall 3.82 |
| warm-up | 6.76 | 30.0 | render/thread/ms 42.47, main/busy/ms 41.74, render/busy/ms 41.21, stage/main/update 23.41, encode//sum 18.39, stage/render/prepare_assets 14.86, stage/main/post_update 12.02, stage/render/render 10.72, render/graph/ms 8.84, stream/drive/ms 7.19, stage/render/graph_submit 6.57, stage/render/prepare_write_phase_buffers 4.88 |
| warm-up | 6.97 | 29.3 | render/thread/ms 46.33, render/busy/ms 45.13, main/busy/ms 31.36, stage/render/prepare_assets 26.08, encode//sum 21.92, stage/main/post_update 13.56, stage/main/update 13.54, stage/render/render 12.39, render/graph/ms 10.83, stage/render/graph_submit 8.70, encode/finish_wall 4.51, encode/submit 4.26 |
| warm-up | 7.09 | 31.8 | main/busy/ms 38.61, render/thread/ms 22.71, encode//sum 20.85, render/busy/ms 20.21, stage/main/update 19.93, stream/drive/ms 14.13, stage/main/post_update 13.13, stream/spawn/ms 12.49, stage/render/render 11.68, render/graph/ms 8.69, stage/render/graph_submit 6.29, stage/render/prepare_assets 4.71 |
| measure | 18.18 | 31.5 | main/busy/ms 40.43, encode//sum 28.74, stage/main/update 24.61, render/thread/ms 23.03, render/busy/ms 21.88, stream/drive/ms 20.29, stage/render/render 14.24, stage/main/post_update 12.81, render/graph/ms 12.44, stage/render/graph_submit 9.89, encode/submit 5.22, encode/finish_wall 4.76 |
| measure | 59.03 | 29.5 | render/thread/ms 41.29, render/busy/ms 39.62, main/busy/ms 36.42, encode//sum 32.88, stage/main/post_update 21.84, stage/render/render 20.41, render/graph/ms 18.02, stage/render/graph_submit 14.19, stage/main/update 9.72, encode/submit 7.90, stage/render/prepare_assets 7.00, encode/finish_wall 6.54 |
| measure | 59.06 | 30.3 | render/thread/ms 42.56, render/busy/ms 41.34, main/busy/ms 27.14, encode//sum 23.17, stage/render/prepare_assets 16.90, stage/main/post_update 14.10, stage/render/render 12.75, render/graph/ms 11.06, stage/render/graph_submit 8.40, stage/main/update 7.98, encode/submit 4.31, encode/finish_wall 4.18 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_nohold\balmora_walk@2560x1440_off=cascadehold-2 --bench-baseline none --off cascadehold
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **80%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.0 W · 73 °C (max 78) · VRAM 5982 MiB · throttled 100% of samples
