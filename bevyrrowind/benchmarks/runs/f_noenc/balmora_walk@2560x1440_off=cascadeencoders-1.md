# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadeencoders · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5638 frames over 65.6 s · **median 11.26 ms (89 fps)** · mean 11.64 · p95 14.57 · p99 17.01 (1% low 59 fps) · worst 41.57 ms · 60 spikes over 1.5× median · 33.8 stutters/min (work 27.4/min)
- warm-up: 341 frames over 9.5 s, worst 2644.0 ms
- hitches (> 28.2 ms): **5** measured, 43 in warm-up
- timed GPU passes: 7.13 ms of a 11.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.26 ms, limit 11 ms
- **FAIL** frame_ms: 37 stutters (> 1.5x moving median of 60) in 1.09 min = 33.82/min, limit 1/min
- **FAIL** worst frame 41.57 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.033 | 2.086 | 2.299 | 0.97 | 0.178 |
| main_opaque_pass_3d | 1.599 | 1.640 | 2.041 | 0.97 | 0.056 |
| early prepass | 0.546 | 0.560 | 1.104 | 0.97 | 0.083 |
| bloom | 0.314 | 0.322 | 0.326 | 0.97 | 0.065 |
| atmosphere_luts | 0.310 | 0.318 | 0.382 | 0.97 | 0.003 |
| water_velocity | 0.278 | 0.285 | 0.354 | 0.97 | 0.002 |
| water_sspr | 0.209 | 0.214 | 0.213 | 0.97 | 0.003 |
| ssao | 0.189 | 0.194 | 0.196 | 0.97 | 0.007 |
| uniform_allocation | 0.147 | 0.151 | 0.429 | 0.98 | 0.024 |
| atmos_froxel_inject | 0.144 | 0.148 | 0.148 | 0.97 | 0.002 |
| clouds_trace | 0.139 | 0.142 | 0.233 | 0.97 | 0.001 |
| water_fft | 0.132 | 0.135 | 0.140 | 0.97 | 0.003 |
| auxiliary_views | 0.130 | 0.133 | 0.531 | 0.98 | 0.121 |
| upscaling | 0.118 | 0.121 | 0.129 | 0.97 | 0.004 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.97 | 0.002 |
| bin_unpacking | 0.099 | 0.102 | 0.123 | 0.98 | 0.017 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.084 | 0.162 | 0.97 | 0.008 |
| clustering | 0.064 | 0.065 | 0.069 | 0.97 | 0.019 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.421 | 0.420 | 0.11 | 0.001 |
| surface_trail | 0.043 | 0.058 | 0.058 | 0.73 | 0.002 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 0.97 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.042 | 0.98 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.231 | 0.238 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.97 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.97 | 0.113 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.97 | 0.004 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.013 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.442 | 16.325 | 63.06 | 58872.6 |
| render/thread/ms | 10.293 | 13.034 | 27.35 | 58034.4 |
| render/busy/ms | 9.704 | 12.426 | 26.45 | 54709.2 |
| main/busy/ms | 9.346 | 11.950 | 37.25 | 52692.7 |
| stage/render/render | 6.621 | 8.487 | 21.78 | 37330.0 |
| stage/main/post_update | 5.851 | 7.990 | 12.21 | 32985.8 |
| render/graph/ms | 5.844 | 7.630 | 21.12 | 32950.1 |
| stage/render/graph_submit | 4.705 | 6.162 | 13.93 | 26525.1 |
| encode/finish_wall | 2.492 | 3.402 | 6.84 | 14050.1 |
| encode/submit | 2.280 | 3.188 | 7.14 | 12855.0 |
| stage/main/update | 2.226 | 2.721 | 30.53 | 12552.3 |
| stage/render/camera_driver | 1.084 | 1.498 | 17.13 | 6111.4 |
| stage/render/prepare_write_phase_buffers | 0.733 | 1.026 | 2.32 | 4134.7 |
| bigspace/total/ms | 0.673 | 2.255 | 3.74 | 3792.3 |
| stage/render/prepare_assets | 0.557 | 0.725 | 11.42 | 3140.1 |
| render/present/ms | 0.495 | 0.638 | 2.37 | 2792.2 |
| stage/render/prepare_resources | 0.449 | 0.549 | 1.10 | 2529.0 |
| stage/main/first | 0.402 | 0.537 | 4.77 | 2265.0 |
| stage/render/prepare_meshes | 0.388 | 0.524 | 1.62 | 2185.7 |
| stage/main/pre_update | 0.337 | 0.513 | 1.20 | 1897.4 |
| terrain/update/ms | 0.302 | 0.386 | 1.79 | 1704.1 |
| bigspace/low_precision/ms | 0.276 | 0.976 | 3.43 | 1555.8 |
| stage/render/specialize | 0.272 | 0.409 | 2.41 | 1535.3 |
| controller/feed/ms | 0.272 | 0.356 | 0.83 | 1534.4 |
| terrain/select/ms | 0.269 | 0.343 | 0.85 | 1514.6 |
| stream/drive/ms | 0.260 | 0.348 | 28.23 | 1467.8 |
| stage/render/prepare_bind_groups | 0.255 | 0.428 | 2.47 | 1437.4 |
| bigspace/high_precision/ms | 0.236 | 1.269 | 3.35 | 1328.7 |
| anim/evaluate/ms | 0.233 | 0.544 | 2.97 | 1315.2 |
| stage/main/fixed_main_loop | 0.228 | 0.384 | 1.12 | 1286.3 |
| stage/render/queue | 0.205 | 0.360 | 2.36 | 1156.1 |
| stream/tick/ms | 0.193 | 0.266 | 0.53 | 1089.8 |
| far/select/ms | 0.187 | 0.254 | 0.58 | 1056.7 |
| stage/render/create_views | 0.186 | 0.250 | 0.95 | 1046.9 |
| stage/render/prepare_flush | 0.179 | 0.373 | 1.04 | 1010.7 |
| stage/render/extract_commands | 0.177 | 0.218 | 0.52 | 997.7 |
| stage/render/prepare_batch_phases | 0.165 | 0.231 | 1.46 | 930.2 |
| anim/evaluate_cpu/ms | 0.148 | 0.195 | 0.49 | 836.3 |
| controller/move/ms | 0.139 | 0.281 | 0.63 | 783.3 |
| stage/render/prepare_views | 0.137 | 0.176 | 1.35 | 774.5 |
| stage/main/last | 0.124 | 0.201 | 1.64 | 697.7 |
| render/acquire/ms | 0.095 | 0.124 | 1.27 | 533.0 |
| stage/render/phase_sort | 0.079 | 0.111 | 0.36 | 445.8 |
| pipelines/compile_ms | 0.052 | 0.000 | 122.24 | 291.1 |
| physics/step/ms | 0.050 | 0.086 | 0.56 | 280.7 |
| pipelines/front_ms | 0.049 | 0.000 | 118.33 | 278.9 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.077 | 0.27 | 269.8 |
| physics/insert/ms | 0.035 | 0.376 | 1.24 | 198.3 |
| stage/render/cleanup | 0.033 | 0.104 | 1.04 | 187.6 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.46 | 178.6 |
| weather/map/ms | 0.026 | 0.034 | 0.07 | 146.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.32 | 84.5 |
| anim/apply/ms | 0.011 | 0.021 | 0.08 | 63.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.43 | 62.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.28 | 61.4 |
| controller/avoid/ms | 0.010 | 0.013 | 0.37 | 58.3 |
| script/update/ms | 0.007 | 0.010 | 0.17 | 40.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.02 | 30.2 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 29.5 |
| script/input/ms | 0.005 | 0.006 | 0.18 | 26.2 |
| stream/spawn/ms | 0.004 | 0.000 | 1.06 | 25.3 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 19.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.10 | 12.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 23 | 18 |
| crowd/c0/count | count | 26 | 43 | 15 |
| crowd/c0/joints/count | count | 1281 | 2262 | 723 |
| crowd/c0/meshes/count | count | 352 | 646 | 193 |
| crowd/c1/count | count | 145 | 156 | 145 |
| crowd/c1/joints/count | count | 5518 | 5991 | 5441 |
| crowd/c1/meshes/count | count | 1167 | 1295 | 1140 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4603 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.08 | 4.66 | 1.86 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.32 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.43 | 0.01 |
| encode/finish_critical | ms | 2.08 | 4.66 | 1.86 |
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
| frame/work/ms | ms | 9.75 | 37.25 | 9.70 |
| gpu/timed/ms | ms | 7.96 | 11.86 | 8.92 |
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
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3406 | 5148 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3406 | 5148 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 198 | 5 |
| phases/shadow/bins | count | 4385 | 5391 | 1921 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.1 k | 12.5 k | 5439 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 192 | 233 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 553 | 6 |
| preprocess/bind_groups_created | count | 13 | 553 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 208 | 125 |
| process/cpu_usage | % | 26.32 | 40.67 | 27.88 |
| process/mem_usage | GiB | 5.36 | 5.53 | 5.52 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.06 | 61.09 | 30.54 |
| system/mem_usage | % | 68.76 | 69.43 | 68.76 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 64 | 245 | 55 |
| views/all/bins | count | 6994 | 8273 | 5419 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 24.9 k | 13.0 k |
| views/all/pipelines | count | 237 | 280 | 66 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.8 k | 22.3 k | 8158 |
| views/cascade/batch_sets | count | 10 | 33 | 5 |
| views/cascade/bins | count | 2226 | 2861 | 1921 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7652 | 9647 | 5439 |
| views/cascade/pipelines | count | 24 | 33 | 20 |
| views/cascade/work_items | count | 521 | 9350 | 240 |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 2152 | 2782 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3071 | 0 |
| views/lamp_face/pipelines | count | 168 | 203 | 0 |
| views/lamp_face/work_items | count | 0 | 2830 | 0 |
| views/main/batch_sets | count | 50 | 52 | 50 |
| views/main/bins | count | 2888 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8428 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 48 | 46 |
| views/main/work_items | count | 8590 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 56.15 | 260 | 316.6 k |
| anim/events/count | count | 0.11 | 4 | 594 |
| assets/Image/added | count | 0.11 | 30 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 19 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.84 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1379 | 4676 |
| encode//count | count | 37.91 | 271 | 213.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5638 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5638 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5638 |
| physics/inserts | count | 0.40 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.70 | 3 | 3939 |
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
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6004 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1316 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.2 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2644.0 | main/busy/ms 2615, stage/main/post_update 2514, stage/main/update 94.99, surface/advance_cpu_ms 67.85, stage/main/pre_update 2.33, stage/main/last 1.27, surface/cell_upload_ms 0.83, stream/drive/ms 0.53, stage/main/first 0.52, stage/main/fixed_main_loop 0.42, weather/map/ms 0.14, bigspace/total/ms 0.12 |
| warm-up | 3.38 | 739.2 | main/busy/ms 2760, stage/main/post_update 2541, render/thread/ms 708, render/busy/ms 545, stage/render/prepare_assets 517, stage/main/update 207, stage/render/prepare_views 162, render/acquire/ms 161, surface/advance_cpu_ms 67.85, stage/render/prepare_resources 14.27, stage/render/render 9.65, encode//sum 8.11 |
| warm-up | 3.44 | 61.0 | render/thread/ms 753, render/busy/ms 588, stage/render/prepare_assets 540, main/busy/ms 164, stage/render/prepare_views 162, render/acquire/ms 161, stage/main/update 116, stage/main/post_update 37.91, stage/render/render 23.27, terrain/update/ms 20.36, render/graph/ms 17.78, stage/render/prepare_resources 17.25 |
| warm-up | 3.62 | 147.6 | pipelines/wait_ms 2355, pipelines/compile_ms 1315, pipelines/front_ms 1230, render/thread/ms 171, render/busy/ms 151, pipelines/driver_ms 84.85, stage/render/prepare_write_phase_buffers 57.42, main/busy/ms 49.48, stage/main/post_update 36.02, stage/render/render 32.77, stage/render/prepare_resources 23.07, render/present/ms 19.37 |
| warm-up | 3.68 | 65.1 | pipelines/wait_ms 6642, pipelines/compile_ms 2612, pipelines/front_ms 2454, render/thread/ms 209, render/busy/ms 195, pipelines/driver_ms 158, main/busy/ms 101, stage/render/prepare_write_phase_buffers 59.89, stage/main/update 56.06, stage/render/prepare_resources 38.61, stage/render/prepare_assets 38.07, stage/main/post_update 30.99 |
| warm-up | 3.80 | 120.2 | pipelines/wait_ms 4724, pipelines/compile_ms 2468, pipelines/front_ms 2350, render/thread/ms 162, render/busy/ms 144, pipelines/driver_ms 118, main/busy/ms 97.29, stage/render/prepare_assets 74.73, stage/main/update 60.07, stage/render/render 51.80, render/graph/ms 33.80, stage/render/graph_submit 26.95 |
| warm-up | 3.84 | 34.1 | pipelines/compile_ms 1348, pipelines/front_ms 1273, pipelines/wait_ms 476, render/thread/ms 131, render/busy/ms 114, pipelines/driver_ms 75.28, stage/render/prepare_assets 74.62, stage/render/render 47.65, main/busy/ms 37.12, render/graph/ms 30.54, encode//sum 23.57, stage/render/graph_submit 23.56 |
| warm-up | 3.88 | 43.6 | pipelines/compile_ms 603, pipelines/front_ms 552, pipelines/wait_ms 101, render/thread/ms 69.44, render/busy/ms 67.61, pipelines/driver_ms 51.07, stage/render/prepare_assets 49.74, main/busy/ms 49.57, stage/main/update 18.56, stage/main/post_update 17.98, terrain/update/ms 11.27, stage/render/render 9.92 |
| warm-up | 4.00 | 58.9 | pipelines/compile_ms 756, pipelines/front_ms 727, main/busy/ms 51.04, stage/main/update 39.98, render/thread/ms 38.34, render/busy/ms 36.68, stream/drive/ms 35.88, pipelines/driver_ms 29.36, stage/render/prepare_assets 20.06, stage/render/render 12.20, render/graph/ms 10.18, encode//sum 8.38 |
| warm-up | 4.06 | 39.4 | pipelines/compile_ms 636, pipelines/front_ms 607, render/thread/ms 48.94, render/busy/ms 47.28, main/busy/ms 30.98, stage/render/prepare_assets 30.40, pipelines/driver_ms 29.41, stage/main/update 17.54, stage/render/render 12.56, render/graph/ms 10.54, stream/drive/ms 9.70, stage/render/graph_submit 8.37 |
| warm-up | 4.17 | 84.7 | pipelines/compile_ms 461, pipelines/front_ms 445, main/busy/ms 106, stage/main/update 73.34, render/thread/ms 53.36, render/busy/ms 50.83, stream/drive/ms 30.80, stage/render/prepare_assets 24.01, stage/main/post_update 21.83, stage/render/render 20.97, pipelines/driver_ms 16.49, render/graph/ms 15.51 |
| warm-up | 4.30 | 128.3 | pipelines/compile_ms 1745, pipelines/front_ms 1701, render/thread/ms 109, render/busy/ms 107, main/busy/ms 74.74, stage/render/prepare_assets 58.37, stage/main/update 47.05, pipelines/driver_ms 43.65, stage/render/prepare_write_phase_buffers 26.01, stage/main/post_update 17.09, stage/render/render 14.04, render/graph/ms 11.38 |
| warm-up | 4.37 | 67.3 | pipelines/compile_ms 1928, pipelines/front_ms 1870, render/thread/ms 123, render/busy/ms 121, pipelines/wait_ms 72.21, main/busy/ms 64.91, pipelines/driver_ms 58.31, stage/render/prepare_assets 57.41, stage/render/prepare_write_phase_buffers 27.38, stage/main/pre_update 21.63, stage/render/render 18.86, stage/main/update 18.25 |
| warm-up | 4.46 | 94.2 | pipelines/compile_ms 710, pipelines/front_ms 676, render/thread/ms 125, render/busy/ms 123, pipelines/wait_ms 89.56, main/busy/ms 65.88, stage/render/prepare_assets 58.82, pipelines/driver_ms 33.35, stage/main/pre_update 22.10, stage/render/render 21.84, stage/render/prepare_write_phase_buffers 19.51, encode//sum 18.86 |
| warm-up | 4.52 | 62.0 | pipelines/compile_ms 1378, pipelines/front_ms 1339, render/thread/ms 133, render/busy/ms 130, pipelines/wait_ms 71.22, encode//sum 68.34, main/busy/ms 55.70, stage/render/prepare_assets 45.68, pipelines/driver_ms 39.17, stage/main/post_update 27.77, stage/render/render 25.53, render/graph/ms 22.44 |
| warm-up | 4.60 | 80.1 | pipelines/compile_ms 2586, pipelines/front_ms 2503, pipelines/wait_ms 901, render/thread/ms 128, render/busy/ms 127, pipelines/driver_ms 83.70, encode//sum 71.63, main/busy/ms 51.87, stage/render/render 30.83, stage/main/post_update 30.67, render/graph/ms 28.11, stage/render/prepare_bind_groups 26.62 |
| warm-up | 4.66 | 58.6 | pipelines/wait_ms 2004, pipelines/compile_ms 1861, pipelines/front_ms 1771, render/thread/ms 128, render/busy/ms 126, main/busy/ms 93.93, pipelines/driver_ms 89.59, stage/main/post_update 68.44, stage/render/render 40.72, render/graph/ms 37.74, stage/render/prepare_flush 30.27, stage/render/graph_submit 25.26 |
| warm-up | 4.72 | 59.0 | pipelines/compile_ms 1637, pipelines/front_ms 1565, pipelines/wait_ms 1562, main/busy/ms 93.81, render/thread/ms 81.87, render/busy/ms 79.64, pipelines/driver_ms 72.72, stage/main/post_update 65.91, stage/render/render 32.77, render/graph/ms 29.79, stage/render/graph_submit 19.19, stage/render/prepare_assets 19.17 |
| warm-up | 4.79 | 65.4 | pipelines/compile_ms 2194, pipelines/front_ms 2136, pipelines/wait_ms 947, main/busy/ms 80.05, render/thread/ms 63.50, render/busy/ms 61.50, stage/main/update 59.39, pipelines/driver_ms 58.11, stream/drive/ms 51.92, stage/render/prepare_assets 42.81, stage/main/post_update 11.83, stage/render/render 11.76 |
| warm-up | 4.85 | 60.5 | pipelines/compile_ms 1722, pipelines/front_ms 1677, pipelines/wait_ms 1010, main/busy/ms 102, stage/main/update 80.64, render/thread/ms 67.95, render/busy/ms 64.21, stream/drive/ms 52.27, pipelines/driver_ms 45.16, stage/render/prepare_assets 34.53, stage/render/render 22.37, render/graph/ms 17.15 |
| warm-up | 4.91 | 67.3 | pipelines/compile_ms 1044, pipelines/front_ms 1010, pipelines/wait_ms 484, main/busy/ms 123, stage/main/update 99.81, render/thread/ms 73.04, render/busy/ms 65.20, stream/drive/ms 45.28, pipelines/driver_ms 33.74, stage/render/render 28.31, stage/render/prepare_assets 27.26, render/graph/ms 19.17 |
| warm-up | 5.01 | 96.3 | pipelines/compile_ms 694, pipelines/front_ms 671, main/busy/ms 104, render/thread/ms 83.53, stage/main/update 82.33, render/busy/ms 77.54, stream/drive/ms 45.11, stage/render/prepare_assets 42.53, pipelines/driver_ms 22.73, stage/render/render 22.15, encode//sum 16.11, render/graph/ms 15.70 |
| warm-up | 5.04 | 30.5 | pipelines/compile_ms 1105, pipelines/front_ms 1070, main/busy/ms 91.02, stage/main/update 72.01, render/thread/ms 67.75, render/busy/ms 65.90, stream/drive/ms 55.71, pipelines/driver_ms 35.25, stage/render/prepare_assets 28.25, stage/render/render 19.28, render/graph/ms 16.93, encode//sum 15.98 |
| warm-up | 5.13 | 93.5 | pipelines/compile_ms 1100, pipelines/front_ms 1064, main/busy/ms 81.82, stage/main/update 65.87, render/thread/ms 58.52, render/busy/ms 56.81, stream/drive/ms 54.90, pipelines/driver_ms 36.18, stage/render/prepare_assets 26.12, encode//sum 14.91, stage/render/render 13.84, render/graph/ms 11.61 |
| warm-up | 5.22 | 67.0 | pipelines/compile_ms 1596, pipelines/front_ms 1552, main/busy/ms 57.25, render/thread/ms 56.89, render/busy/ms 55.22, pipelines/driver_ms 43.57, stage/main/update 41.48, stream/drive/ms 34.24, stage/render/prepare_assets 28.13, stage/render/render 14.99, encode//sum 13.22, render/graph/ms 12.81 |
| warm-up | 5.27 | 29.6 | pipelines/compile_ms 104, pipelines/front_ms 99.00, render/thread/ms 47.74, render/busy/ms 46.11, main/busy/ms 34.78, stage/main/update 16.93, stage/render/prepare_assets 15.42, encode//sum 15.24, stage/render/render 12.80, stage/main/post_update 11.92, render/graph/ms 10.58, stage/render/graph_submit 8.22 |
| warm-up | 5.39 | 43.3 | pipelines/compile_ms 119, pipelines/front_ms 113, render/thread/ms 51.07, render/busy/ms 49.22, main/busy/ms 38.55, stage/main/update 20.58, stage/render/render 19.94, encode//sum 17.55, render/graph/ms 17.44, stage/render/prepare_assets 14.92, stage/render/graph_submit 14.70, stream/drive/ms 12.52 |
| warm-up | 5.43 | 44.6 | pipelines/compile_ms 1321, pipelines/front_ms 1279, render/thread/ms 69.01, render/busy/ms 67.13, main/busy/ms 45.39, pipelines/driver_ms 41.82, stage/render/prepare_assets 31.32, stage/main/update 24.05, stage/render/render 21.07, render/graph/ms 18.37, encode//sum 16.40, stage/render/graph_submit 15.13 |
| warm-up | 5.48 | 47.5 | pipelines/compile_ms 1454, pipelines/front_ms 1408, render/thread/ms 69.59, render/busy/ms 67.86, pipelines/driver_ms 45.57, stage/render/prepare_assets 43.37, main/busy/ms 43.24, stage/main/update 23.38, stage/render/render 14.85, encode//sum 13.64, stage/main/post_update 12.82, render/graph/ms 12.51 |
| warm-up | 5.51 | 32.9 | pipelines/compile_ms 364, pipelines/front_ms 350, render/thread/ms 65.18, render/busy/ms 63.58, stage/render/prepare_assets 37.22, main/busy/ms 35.77, encode//sum 14.71, stage/main/post_update 14.66, stage/main/update 14.26, pipelines/driver_ms 13.69, stage/render/render 12.57, render/graph/ms 10.55 |
| warm-up | 5.55 | 41.8 | pipelines/compile_ms 112, pipelines/front_ms 108, render/thread/ms 68.12, render/busy/ms 66.18, main/busy/ms 37.02, encode//sum 18.96, stage/render/prepare_assets 18.72, stage/render/prepare_resources 16.39, stage/main/post_update 15.73, stage/render/render 14.13, stage/main/update 13.37, render/graph/ms 11.66 |
| warm-up | 5.58 | 29.4 | pipelines/compile_ms 608, pipelines/front_ms 591, render/thread/ms 66.67, render/busy/ms 64.78, main/busy/ms 37.41, encode//sum 20.22, stage/render/prepare_assets 20.17, stage/render/prepare_resources 17.23, pipelines/driver_ms 16.98, stage/main/post_update 16.71, stage/render/render 15.18, render/graph/ms 12.58 |
| warm-up | 5.66 | 49.7 | pipelines/compile_ms 414, pipelines/front_ms 401, render/thread/ms 53.33, render/busy/ms 51.71, main/busy/ms 49.56, stage/main/update 29.14, stage/render/prepare_assets 25.16, stream/drive/ms 19.90, encode//sum 15.74, stage/render/render 14.59, stage/main/post_update 13.37, pipelines/driver_ms 13.06 |
| warm-up | 5.69 | 30.9 | pipelines/compile_ms 1155, pipelines/front_ms 1123, render/thread/ms 59.09, render/busy/ms 57.47, pipelines/driver_ms 31.85, stage/render/prepare_assets 30.08, main/busy/ms 28.74, encode//sum 16.17, stage/render/render 13.62, stage/main/update 12.16, render/graph/ms 11.14, stage/main/post_update 10.64 |
| warm-up | 6.07 | 28.5 | main/busy/ms 37.16, render/thread/ms 25.87, render/busy/ms 24.52, stage/main/update 22.21, stream/drive/ms 18.20, encode//sum 17.10, stage/render/render 13.90, render/graph/ms 11.83, stage/main/post_update 11.39, stage/render/graph_submit 8.99, encode/submit 5.25, stage/render/prepare_assets 4.71 |
| warm-up | 6.47 | 44.0 | main/busy/ms 46.14, stage/main/update 32.92, render/thread/ms 22.61, render/busy/ms 21.13, stream/drive/ms 19.86, encode//sum 15.63, stage/render/render 10.93, render/graph/ms 8.94, stage/main/post_update 8.55, stage/render/graph_submit 6.69, stage/render/prepare_assets 4.93, encode/submit 3.39 |
| warm-up | 6.61 | 30.0 | render/thread/ms 40.06, render/busy/ms 38.93, main/busy/ms 29.17, stage/render/prepare_assets 17.16, encode//sum 15.59, stage/main/update 12.73, stage/render/render 10.29, stage/main/post_update 9.81, render/graph/ms 8.48, stage/render/graph_submit 5.49, encode/finish_wall 3.41, stream/drive/ms 3.37 |
| warm-up | 6.80 | 30.8 | main/busy/ms 41.81, render/thread/ms 39.61, render/busy/ms 37.84, encode//sum 21.14, stage/main/update 18.76, stage/render/render 17.52, stage/main/post_update 16.66, render/graph/ms 14.69, stage/render/graph_submit 10.88, encode/submit 5.89, stage/render/prepare_assets 5.52, encode/finish_wall 5.13 |
| warm-up | 6.83 | 30.4 | pipelines/compile_ms 83.67, pipelines/front_ms 78.97, render/thread/ms 46.22, render/busy/ms 44.49, main/busy/ms 44.45, encode//sum 21.80, stage/main/post_update 21.05, stage/main/update 15.92, stage/render/render 15.79, render/graph/ms 13.28, stage/render/prepare_assets 11.52, stage/render/graph_submit 9.92 |
| warm-up | 6.86 | 30.6 | pipelines/compile_ms 83.67, pipelines/front_ms 78.97, render/thread/ms 42.75, render/busy/ms 41.61, main/busy/ms 33.13, stage/render/prepare_assets 19.47, encode//sum 16.31, stage/main/post_update 14.54, stage/main/update 11.88, stage/render/render 10.30, render/graph/ms 8.64, stage/render/graph_submit 6.05 |
| warm-up | 7.02 | 29.8 | render/thread/ms 47.88, render/busy/ms 46.18, main/busy/ms 40.59, encode//sum 23.44, stage/render/prepare_assets 20.61, stage/main/update 18.15, stage/main/post_update 16.08, stage/render/render 14.94, render/graph/ms 12.61, stage/render/graph_submit 9.63, encode/finish_wall 5.70, encode/submit 4.02 |
| warm-up | 7.06 | 40.4 | render/thread/ms 57.93, render/busy/ms 56.21, main/busy/ms 56.01, stage/main/update 29.59, stage/render/prepare_assets 29.27, encode//sum 23.63, stage/main/post_update 18.95, stage/render/render 15.83, stream/drive/ms 15.17, render/graph/ms 13.52, stage/render/graph_submit 10.15, encode/finish_wall 5.76 |
| warm-up | 7.10 | 40.0 | render/thread/ms 61.30, render/busy/ms 60.11, main/busy/ms 45.48, stage/render/prepare_assets 39.01, stage/main/update 23.71, encode//sum 17.98, stream/drive/ms 15.17, stage/main/post_update 14.87, stage/render/render 12.36, render/graph/ms 10.64, stage/render/graph_submit 7.98, encode/finish_wall 4.24 |
| measure | 2.72 | 28.6 | encode//sum 78.02, render/thread/ms 40.61, render/busy/ms 39.13, stage/render/render 28.84, render/graph/ms 26.68, stage/render/graph_submit 20.64, main/busy/ms 20.50, stage/main/post_update 13.13, encode/finish_wall 11.01, encode/submit 9.70, stage/render/camera_driver 5.90, stage/main/update 4.95 |
| measure | 4.92 | 41.4 | main/busy/ms 47.39, render/thread/ms 29.77, render/busy/ms 28.42, encode//sum 28.40, stage/main/update 23.85, stage/main/post_update 19.90, stream/drive/ms 18.97, stage/render/render 16.44, render/graph/ms 14.16, stage/render/graph_submit 10.77, encode/finish_wall 6.50, encode/submit 4.35 |
| measure | 18.20 | 41.6 | main/busy/ms 48.46, stage/main/update 32.59, stream/drive/ms 28.50, encode//sum 26.24, render/thread/ms 23.84, render/busy/ms 22.51, stage/render/render 13.54, stage/main/post_update 12.97, render/graph/ms 11.57, stage/render/graph_submit 8.14, encode/finish_wall 4.81, encode/submit 3.40 |
| measure | 22.79 | 32.2 | main/busy/ms 40.80, render/thread/ms 26.07, encode//sum 25.60, render/busy/ms 24.86, stage/main/update 22.50, stream/drive/ms 18.59, stage/main/post_update 14.92, stage/render/render 13.60, render/graph/ms 11.81, stage/render/graph_submit 8.78, stage/render/prepare_assets 5.13, encode/finish_wall 4.60 |
| measure | 59.05 | 28.4 | render/thread/ms 41.97, render/busy/ms 40.67, main/busy/ms 30.17, encode//sum 22.93, stage/main/post_update 17.27, stage/render/prepare_assets 15.29, stage/render/render 14.07, render/graph/ms 12.46, stage/render/graph_submit 8.90, stage/main/update 7.16, encode/finish_wall 5.21, encode/submit 3.77 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_noenc\balmora_walk@2560x1440_off=cascadeencoders-1 --bench-baseline none --off cascadeencoders
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 47%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.0 W · 71 °C (max 77) · VRAM 6017 MiB · throttled 100% of samples
