# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadeencoders · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5718 frames over 65.6 s · **median 11.21 ms (89 fps)** · mean 11.48 · p95 14.02 · p99 16.24 (1% low 62 fps) · worst 31.23 ms · 44 spikes over 1.5× median · 22.8 stutters/min (work 22.8/min)
- warm-up: 326 frames over 9.4 s, worst 2685.6 ms
- hitches (> 28.0 ms): **2** measured, 47 in warm-up
- timed GPU passes: 7.13 ms of a 11.48 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.21 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 31.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.032 | 2.083 | 2.257 | 0.98 | 0.176 |
| main_opaque_pass_3d | 1.592 | 1.632 | 2.019 | 0.98 | 0.056 |
| early prepass | 0.538 | 0.552 | 1.089 | 0.98 | 0.080 |
| bloom | 0.314 | 0.322 | 0.326 | 0.98 | 0.063 |
| atmosphere_luts | 0.311 | 0.318 | 0.381 | 0.98 | 0.003 |
| water_velocity | 0.279 | 0.286 | 0.354 | 0.98 | 0.001 |
| water_sspr | 0.201 | 0.206 | 0.205 | 0.98 | 0.002 |
| ssao | 0.189 | 0.193 | 0.196 | 0.98 | 0.007 |
| uniform_allocation | 0.151 | 0.155 | 0.436 | 0.98 | 0.024 |
| auxiliary_views | 0.145 | 0.149 | 0.589 | 0.98 | 0.130 |
| atmos_froxel_inject | 0.145 | 0.149 | 0.147 | 0.98 | 0.001 |
| clouds_trace | 0.138 | 0.142 | 0.231 | 0.98 | 0.001 |
| water_fft | 0.129 | 0.133 | 0.139 | 0.98 | 0.003 |
| upscaling | 0.118 | 0.121 | 0.129 | 0.98 | 0.004 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.98 | 0.002 |
| bin_unpacking | 0.101 | 0.104 | 0.125 | 0.98 | 0.017 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.084 | 0.160 | 0.98 | 0.008 |
| clustering | 0.064 | 0.066 | 0.069 | 0.98 | 0.018 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.98 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.423 | 0.421 | 0.11 | 0.001 |
| surface_trail | 0.042 | 0.058 | 0.057 | 0.72 | 0.002 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.043 | 0.98 | 0.004 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.231 | 0.236 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.98 | 0.112 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_downsampling_second_pass | 0.002 | 0.014 | 0.013 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.014 | 0.014 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.344 | 15.884 | 46.06 | 59149.5 |
| render/thread/ms | 10.167 | 12.504 | 23.37 | 58134.4 |
| render/busy/ms | 9.596 | 11.933 | 22.72 | 54869.6 |
| main/busy/ms | 9.151 | 11.479 | 28.65 | 52323.6 |
| stage/render/render | 6.529 | 8.180 | 14.94 | 37332.8 |
| render/graph/ms | 5.774 | 7.369 | 13.87 | 33018.2 |
| stage/main/post_update | 5.739 | 7.731 | 11.48 | 32815.3 |
| stage/render/graph_submit | 4.649 | 5.938 | 10.70 | 26583.3 |
| encode/finish_wall | 2.440 | 3.233 | 5.15 | 13951.4 |
| encode/submit | 2.267 | 3.123 | 6.51 | 12961.9 |
| stage/main/update | 2.170 | 2.610 | 22.36 | 12405.2 |
| stage/render/camera_driver | 1.071 | 1.465 | 3.71 | 6124.1 |
| stage/render/prepare_write_phase_buffers | 0.747 | 1.024 | 4.37 | 4270.0 |
| bigspace/total/ms | 0.653 | 2.182 | 3.69 | 3731.2 |
| stage/render/prepare_assets | 0.546 | 0.701 | 10.43 | 3121.5 |
| render/present/ms | 0.479 | 0.599 | 2.17 | 2737.3 |
| stage/render/prepare_resources | 0.449 | 0.549 | 1.04 | 2564.8 |
| stage/main/first | 0.398 | 0.523 | 4.51 | 2277.2 |
| stage/render/prepare_meshes | 0.372 | 0.485 | 1.51 | 2127.6 |
| stage/main/pre_update | 0.327 | 0.487 | 1.18 | 1871.2 |
| terrain/update/ms | 0.301 | 0.389 | 2.12 | 1721.6 |
| stage/render/specialize | 0.274 | 0.407 | 2.44 | 1564.5 |
| bigspace/low_precision/ms | 0.273 | 1.045 | 3.29 | 1558.7 |
| terrain/select/ms | 0.268 | 0.347 | 0.79 | 1531.3 |
| controller/feed/ms | 0.260 | 0.334 | 0.88 | 1484.8 |
| stage/render/prepare_bind_groups | 0.254 | 0.424 | 2.30 | 1450.9 |
| stream/drive/ms | 0.252 | 0.336 | 20.40 | 1441.5 |
| bigspace/high_precision/ms | 0.226 | 1.121 | 3.27 | 1293.6 |
| stage/main/fixed_main_loop | 0.220 | 0.369 | 0.89 | 1260.0 |
| anim/evaluate/ms | 0.218 | 0.413 | 3.17 | 1244.5 |
| stage/render/queue | 0.203 | 0.355 | 2.96 | 1159.8 |
| stream/tick/ms | 0.195 | 0.266 | 0.62 | 1115.7 |
| far/select/ms | 0.188 | 0.257 | 0.58 | 1074.9 |
| stage/render/create_views | 0.181 | 0.238 | 0.93 | 1032.8 |
| stage/render/prepare_flush | 0.174 | 0.367 | 0.82 | 997.4 |
| stage/render/extract_commands | 0.174 | 0.210 | 0.44 | 993.1 |
| stage/render/prepare_batch_phases | 0.162 | 0.229 | 0.56 | 927.2 |
| anim/evaluate_cpu/ms | 0.145 | 0.186 | 0.47 | 828.5 |
| stage/render/prepare_views | 0.134 | 0.175 | 1.10 | 766.4 |
| controller/move/ms | 0.132 | 0.272 | 0.52 | 753.6 |
| stage/main/last | 0.120 | 0.194 | 1.65 | 686.8 |
| render/acquire/ms | 0.092 | 0.123 | 1.03 | 527.4 |
| stage/render/phase_sort | 0.078 | 0.106 | 0.28 | 443.4 |
| pipelines/compile_ms | 0.050 | 0.000 | 119.64 | 287.1 |
| pipelines/front_ms | 0.048 | 0.000 | 115.37 | 274.8 |
| physics/step/ms | 0.048 | 0.083 | 0.56 | 274.4 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.074 | 0.28 | 267.5 |
| stage/render/cleanup | 0.033 | 0.106 | 0.66 | 190.0 |
| physics/insert/ms | 0.033 | 0.373 | 0.89 | 188.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.35 | 170.7 |
| weather/map/ms | 0.026 | 0.034 | 0.23 | 148.6 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.17 | 82.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.26 | 65.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.05 | 61.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.29 | 60.8 |
| controller/avoid/ms | 0.010 | 0.014 | 0.20 | 59.3 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 37.8 |
| stream/despawn/ms | 0.005 | 0.000 | 0.89 | 28.9 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 27.5 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 26.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 24.8 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 19.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 14.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.27 | 12.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.56 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.0 |
| stage/render/prepare | 0.001 | 0.002 | 0.07 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.07 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1291 | 2262 | 528 |
| crowd/c0/meshes/count | count | 355 | 646 | 135 |
| crowd/c1/count | count | 144 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6046 | 5668 |
| crowd/c1/meshes/count | count | 1194 | 1322 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4601 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.04 | 4.03 | 2.02 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.17 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.02 |
| encode/finish_critical | ms | 2.04 | 4.03 | 2.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.35 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.66 | 28.65 | 8.93 |
| gpu/timed/ms | ms | 7.97 | 10.12 | 8.12 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3424 | 5126 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3424 | 5126 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 199 | 19 |
| phases/shadow/bins | count | 4389 | 5540 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.1 k | 12.6 k | 5442 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 190 | 241 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 196 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 559 | 37 |
| preprocess/bind_groups_created | count | 14 | 559 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 132 | 216 | 123 |
| process/cpu_usage | % | 26.92 | 47.14 | 20.10 |
| process/mem_usage | GiB | 5.38 | 5.54 | 5.54 |
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
| started_load_count |  | 4918 | 5013 | 5013 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.57 | 47.88 | 31.82 |
| system/mem_usage | % | 68.28 | 68.88 | 68.81 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 68 | 248 | 69 |
| views/all/bins | count | 7027 | 8438 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.3 k | 25.1 k | 13.1 k |
| views/all/pipelines | count | 237 | 288 | 65 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.9 k | 22.6 k | 13.4 k |
| views/cascade/batch_sets | count | 10 | 33 | 19 |
| views/cascade/bins | count | 2228 | 2881 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7647 | 9637 | 5442 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 522 | 8953 | 5442 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2153 | 2834 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3128 | 0 |
| views/lamp_face/pipelines | count | 166 | 210 | 0 |
| views/lamp_face/work_items | count | 0 | 2860 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2870 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8488 | 13.0 k | 7610 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8648 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 48.13 | 212 | 275.2 k |
| anim/events/count | count | 0.08 | 3 | 446 |
| assets/Image/added | count | 0.10 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.71 | 152 | 4061 |
| assets/Mesh/removed | count | 0.85 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.79 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1492 | 4675 |
| encode//count | count | 38.83 | 272 | 222.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5718 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5718 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5718 |
| physics/inserts | count | 0.39 | 106 | 2245 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 2 | 3939 |
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
| stream/upload/bytes | bytes | 5920 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1298 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.0 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.69 | 2685.6 | main/busy/ms 2659, stage/main/post_update 2559, stage/main/update 93.07, surface/advance_cpu_ms 65.16, stage/main/pre_update 2.95, stage/main/last 1.05, surface/cell_upload_ms 0.73, stage/main/first 0.51, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.39, weather/map/ms 0.11, bigspace/total/ms 0.08 |
| warm-up | 3.42 | 735.1 | main/busy/ms 2793, stage/main/post_update 2582, render/thread/ms 707, render/busy/ms 544, stage/render/prepare_assets 508, stage/main/update 199, stage/render/prepare_views 159, render/acquire/ms 158, surface/advance_cpu_ms 65.16, stage/render/prepare_resources 21.83, stage/render/render 12.97, encode//sum 7.08 |
| warm-up | 3.50 | 78.5 | render/thread/ms 769, render/busy/ms 604, stage/render/prepare_assets 554, stage/render/prepare_views 160, render/acquire/ms 159, main/busy/ms 158, stage/main/update 111, stage/main/post_update 35.05, stage/render/prepare_resources 27.10, stage/render/render 19.61, terrain/update/ms 17.12, encode//sum 11.95 |
| warm-up | 3.56 | 61.2 | pipelines/wait_ms 482, pipelines/compile_ms 445, pipelines/front_ms 410, render/thread/ms 117, render/busy/ms 101, main/busy/ms 79.32, stage/main/post_update 60.75, stage/render/render 52.42, stage/render/prepare_assets 47.17, render/graph/ms 35.71, pipelines/driver_ms 34.64, pipelines/lock_ms 29.19 |
| warm-up | 3.69 | 126.4 | pipelines/wait_ms 4141, pipelines/compile_ms 1460, pipelines/front_ms 1336, render/thread/ms 171, render/busy/ms 156, pipelines/driver_ms 125, main/busy/ms 88.56, stage/render/render 70.17, stage/main/post_update 62.47, render/graph/ms 49.42, stage/render/prepare_resources 40.66, pipelines/lock_ms 28.46 |
| warm-up | 3.74 | 51.5 | pipelines/wait_ms 4589, pipelines/compile_ms 2114, pipelines/front_ms 1979, render/thread/ms 166, render/busy/ms 163, pipelines/driver_ms 135, main/busy/ms 80.93, stage/render/prepare_resources 46.01, stage/render/prepare_assets 35.47, encode//sum 35.26, stage/render/render 34.77, stage/main/pre_update 33.31 |
| warm-up | 3.80 | 61.3 | pipelines/compile_ms 1241, pipelines/front_ms 1187, pipelines/wait_ms 1063, main/busy/ms 145, render/thread/ms 106, render/busy/ms 103, stage/main/update 87.94, stage/render/prepare_assets 67.01, pipelines/driver_ms 54.21, stage/main/pre_update 26.31, stage/main/post_update 24.12, encode//sum 20.44 |
| warm-up | 3.91 | 112.1 | pipelines/compile_ms 916, pipelines/front_ms 860, pipelines/wait_ms 165, render/thread/ms 126, render/busy/ms 124, main/busy/ms 118, stage/render/prepare_assets 106, stage/main/update 94.11, pipelines/driver_ms 55.46, terrain/update/ms 16.26, stage/main/post_update 14.19, stage/render/render 10.02 |
| warm-up | 4.12 | 64.6 | pipelines/compile_ms 1138, pipelines/front_ms 1090, main/busy/ms 65.30, render/thread/ms 62.25, render/busy/ms 60.84, stage/main/update 52.79, pipelines/driver_ms 48.17, stream/drive/ms 44.92, stage/render/prepare_assets 43.67, stage/render/render 9.63, render/graph/ms 7.91, encode//sum 7.74 |
| warm-up | 4.17 | 28.3 | pipelines/compile_ms 254, pipelines/front_ms 246, render/thread/ms 43.85, render/busy/ms 39.69, main/busy/ms 33.58, stage/render/prepare_assets 21.44, stage/render/render 15.37, stage/main/update 15.11, render/graph/ms 10.70, stage/main/post_update 10.65, stream/drive/ms 9.87, pipelines/driver_ms 8.85 |
| warm-up | 4.20 | 29.0 | pipelines/compile_ms 305, pipelines/front_ms 294, main/busy/ms 57.65, render/thread/ms 47.67, render/busy/ms 43.35, stage/main/update 36.66, stream/drive/ms 27.03, stage/render/prepare_assets 24.80, stage/render/render 13.74, stage/main/post_update 12.30, pipelines/driver_ms 11.13, render/graph/ms 8.83 |
| warm-up | 4.30 | 94.3 | pipelines/compile_ms 516, pipelines/front_ms 500, render/thread/ms 81.52, render/busy/ms 79.00, main/busy/ms 66.08, stage/render/prepare_assets 55.83, stage/main/update 43.05, stream/drive/ms 28.13, pipelines/driver_ms 16.10, stage/render/render 15.97, stage/main/post_update 15.31, render/graph/ms 12.85 |
| warm-up | 4.35 | 57.1 | pipelines/compile_ms 706, pipelines/front_ms 682, render/thread/ms 102, render/busy/ms 99.26, main/busy/ms 69.96, stage/render/prepare_assets 55.26, stage/main/update 50.26, stage/render/render 28.88, render/graph/ms 25.43, pipelines/driver_ms 23.21, stage/main/post_update 11.36, stage/render/graph_submit 9.36 |
| warm-up | 4.52 | 169.7 | pipelines/compile_ms 2593, pipelines/front_ms 2522, render/thread/ms 164, render/busy/ms 162, pipelines/wait_ms 140, pipelines/driver_ms 71.65, stage/render/prepare_assets 68.35, main/busy/ms 62.13, stage/render/render 51.24, render/graph/ms 48.46, stage/main/update 44.73, stage/render/camera_driver 32.45 |
| warm-up | 4.57 | 45.4 | pipelines/compile_ms 2358, pipelines/front_ms 2295, render/thread/ms 164, pipelines/wait_ms 162, render/busy/ms 162, pipelines/driver_ms 62.61, stage/render/prepare_assets 57.60, stage/render/render 52.24, render/graph/ms 49.75, encode//sum 48.40, main/busy/ms 37.25, stage/render/camera_driver 30.56 |
| warm-up | 4.61 | 44.6 | render/thread/ms 79.24, render/busy/ms 77.34, pipelines/compile_ms 67.15, pipelines/front_ms 64.06, encode//sum 52.74, main/busy/ms 46.28, pipelines/wait_ms 42.02, stage/render/render 28.86, render/graph/ms 26.32, stage/render/graph_submit 19.93, stage/main/post_update 17.73, stage/render/prepare_assets 16.34 |
| warm-up | 4.70 | 92.4 | pipelines/compile_ms 1375, pipelines/front_ms 1338, pipelines/wait_ms 174, render/thread/ms 119, render/busy/ms 118, encode//sum 77.76, main/busy/ms 44.90, pipelines/driver_ms 36.93, stage/render/prepare_resources 34.99, stage/render/render 30.02, render/graph/ms 27.70, stage/render/prepare_assets 27.16 |
| warm-up | 4.75 | 44.2 | pipelines/compile_ms 2011, pipelines/front_ms 1958, pipelines/wait_ms 345, render/thread/ms 126, render/busy/ms 124, encode//sum 81.60, pipelines/driver_ms 52.84, main/busy/ms 48.59, stage/render/prepare_resources 47.94, stage/render/render 35.52, render/graph/ms 32.87, stage/render/graph_submit 26.71 |
| warm-up | 4.78 | 30.7 | pipelines/compile_ms 1281, pipelines/front_ms 1246, pipelines/wait_ms 202, render/thread/ms 62.81, render/busy/ms 61.18, main/busy/ms 42.32, pipelines/driver_ms 34.92, stage/main/post_update 22.11, stage/render/render 20.05, encode//sum 19.42, render/graph/ms 17.46, stage/render/prepare_resources 15.75 |
| warm-up | 4.91 | 101.2 | pipelines/compile_ms 1404, pipelines/front_ms 1358, pipelines/wait_ms 688, main/busy/ms 107, stage/main/update 75.71, render/thread/ms 61.22, render/busy/ms 57.52, stream/drive/ms 52.92, pipelines/driver_ms 46.50, stage/render/prepare_assets 26.54, stage/main/post_update 20.84, stage/render/render 18.93 |
| warm-up | 4.97 | 59.0 | pipelines/compile_ms 2043, pipelines/front_ms 1969, pipelines/wait_ms 775, main/busy/ms 118, render/thread/ms 88.31, stage/main/update 87.76, render/busy/ms 86.03, pipelines/driver_ms 73.87, stream/drive/ms 54.67, stage/render/prepare_assets 36.28, stage/render/render 27.53, render/graph/ms 24.58 |
| warm-up | 5.11 | 141.2 | pipelines/compile_ms 1845, pipelines/front_ms 1789, pipelines/wait_ms 133, render/thread/ms 128, render/busy/ms 126, main/busy/ms 110, stage/main/update 86.29, stage/render/prepare_assets 63.43, pipelines/driver_ms 55.90, stream/drive/ms 53.46, stage/render/prepare_resources 25.16, stage/render/render 18.94 |
| warm-up | 5.15 | 45.0 | pipelines/compile_ms 1240, pipelines/front_ms 1200, render/thread/ms 119, render/busy/ms 117, main/busy/ms 83.76, stage/main/update 59.95, stage/render/prepare_assets 51.84, pipelines/driver_ms 40.15, stream/drive/ms 39.13, stage/render/render 27.68, stage/render/prepare_resources 25.13, render/graph/ms 24.47 |
| warm-up | 5.21 | 59.1 | pipelines/compile_ms 275, pipelines/front_ms 257, main/busy/ms 76.71, render/thread/ms 69.15, render/busy/ms 66.59, stage/main/update 61.18, stream/drive/ms 38.90, stage/render/render 27.23, stage/render/prepare_assets 26.11, render/graph/ms 24.01, stage/render/graph_submit 20.75, pipelines/driver_ms 18.70 |
| warm-up | 5.24 | 29.5 | pipelines/compile_ms 61.50, main/busy/ms 59.91, pipelines/front_ms 58.60, render/thread/ms 53.00, render/busy/ms 51.47, stage/main/update 45.40, stream/drive/ms 28.39, stage/render/prepare_assets 25.79, encode//sum 15.88, stage/render/render 13.12, render/graph/ms 10.88, stage/render/graph_submit 8.48 |
| warm-up | 5.27 | 32.4 | pipelines/compile_ms 516, pipelines/front_ms 502, main/busy/ms 50.92, render/thread/ms 37.88, render/busy/ms 36.22, stage/main/update 34.59, stream/drive/ms 28.04, encode//sum 17.12, stage/render/render 14.53, pipelines/driver_ms 14.42, render/graph/ms 12.22, stage/render/prepare_assets 9.45 |
| warm-up | 5.35 | 34.8 | pipelines/compile_ms 140, pipelines/front_ms 133, render/thread/ms 42.31, render/busy/ms 40.79, main/busy/ms 33.62, stage/render/prepare_assets 16.96, encode//sum 16.31, stage/main/post_update 16.10, stage/main/update 12.58, stage/render/render 12.12, render/graph/ms 10.13, stage/render/graph_submit 7.39 |
| warm-up | 5.40 | 29.6 | pipelines/compile_ms 210, pipelines/front_ms 200, render/thread/ms 48.00, render/busy/ms 46.16, main/busy/ms 29.51, encode//sum 20.88, stage/render/render 16.05, render/graph/ms 13.50, stage/render/prepare_assets 12.36, stage/main/update 12.14, stage/main/post_update 11.00, stage/render/graph_submit 9.60 |
| warm-up | 5.44 | 32.5 | pipelines/compile_ms 714, pipelines/front_ms 693, render/thread/ms 52.33, render/busy/ms 50.71, main/busy/ms 33.34, stage/render/prepare_assets 24.87, pipelines/driver_ms 21.23, encode//sum 19.25, stage/main/update 17.60, stage/render/render 13.91, render/graph/ms 11.56, stage/main/post_update 9.56 |
| warm-up | 5.48 | 43.9 | pipelines/compile_ms 799, pipelines/front_ms 776, render/thread/ms 62.89, render/busy/ms 59.68, main/busy/ms 33.07, stage/render/prepare_assets 27.49, encode//sum 23.03, pipelines/driver_ms 22.78, stage/render/render 21.01, render/graph/ms 18.46, stage/main/update 16.69, stage/render/graph_submit 13.23 |
| warm-up | 5.51 | 28.6 | pipelines/compile_ms 556, pipelines/front_ms 532, render/thread/ms 64.28, render/busy/ms 60.96, main/busy/ms 32.93, stage/render/render 25.98, encode//sum 25.13, pipelines/driver_ms 23.67, render/graph/ms 23.25, stage/render/graph_submit 17.16, stage/main/post_update 13.74, stage/render/prepare_assets 13.00 |
| warm-up | 5.54 | 33.7 | pipelines/compile_ms 518, pipelines/front_ms 499, render/thread/ms 58.05, render/busy/ms 55.92, main/busy/ms 37.72, stage/render/render 23.26, render/graph/ms 20.35, pipelines/driver_ms 19.30, encode//sum 18.79, stage/main/post_update 17.06, stage/main/update 13.92, stage/render/graph_submit 12.32 |
| warm-up | 5.58 | 39.0 | pipelines/compile_ms 325, pipelines/front_ms 315, render/thread/ms 67.13, render/busy/ms 65.05, main/busy/ms 49.75, pipelines/wait_ms 31.66, stage/main/update 21.68, encode//sum 19.15, stage/render/render 18.90, stage/render/prepare_assets 18.25, stage/main/post_update 18.21, render/graph/ms 16.01 |
| warm-up | 5.64 | 56.2 | pipelines/compile_ms 480, pipelines/front_ms 468, render/thread/ms 83.03, render/busy/ms 81.35, main/busy/ms 51.94, stage/render/prepare_assets 40.97, pipelines/wait_ms 31.68, stage/main/update 23.91, encode//sum 22.34, stage/main/post_update 18.63, stage/render/render 14.18, stage/render/prepare_write_phase_buffers 13.34 |
| warm-up | 5.69 | 32.4 | pipelines/compile_ms 1283, pipelines/front_ms 1236, render/thread/ms 50.07, main/busy/ms 48.56, render/busy/ms 48.54, pipelines/driver_ms 46.82, stage/render/prepare_assets 21.64, stage/main/post_update 20.97, stage/main/update 19.79, encode//sum 15.96, stage/render/render 14.96, render/graph/ms 12.79 |
| warm-up | 5.73 | 38.3 | pipelines/compile_ms 1510, pipelines/front_ms 1456, render/thread/ms 59.17, render/busy/ms 57.21, pipelines/driver_ms 53.84, main/busy/ms 50.19, stage/render/prepare_assets 25.33, stage/main/update 25.13, encode//sum 18.62, stage/render/render 18.57, stage/main/post_update 16.69, render/graph/ms 15.84 |
| warm-up | 5.80 | 46.3 | pipelines/compile_ms 210, pipelines/front_ms 202, render/thread/ms 59.88, render/busy/ms 58.28, main/busy/ms 35.61, stage/render/prepare_assets 27.66, stage/main/post_update 20.38, encode//sum 19.99, stage/render/render 16.67, render/graph/ms 14.26, stage/render/graph_submit 10.84, stage/main/update 9.50 |
| warm-up | 5.83 | 29.7 | render/thread/ms 62.79, render/busy/ms 61.45, stage/render/prepare_assets 38.77, main/busy/ms 36.46, stage/main/post_update 22.31, encode//sum 19.84, stage/render/render 13.84, render/graph/ms 11.84, bigspace/total/ms 9.38, stage/main/update 9.00, stage/render/graph_submit 8.80, bigspace/high_precision/ms 5.74 |
| warm-up | 6.31 | 29.7 | render/thread/ms 40.42, render/busy/ms 38.55, main/busy/ms 26.70, encode//sum 24.04, stage/render/render 19.43, render/graph/ms 16.73, stage/main/post_update 12.79, stage/render/graph_submit 12.54, stage/main/update 9.31, encode/submit 7.49, encode/finish_wall 5.18, stage/render/prepare_assets 4.22 |
| warm-up | 6.36 | 32.4 | render/thread/ms 40.62, render/busy/ms 39.36, main/busy/ms 25.93, encode//sum 17.75, stage/render/prepare_assets 16.48, stage/render/render 10.88, stage/main/update 10.66, stage/main/post_update 10.20, render/graph/ms 9.31, stage/render/graph_submit 6.70, encode/finish_wall 3.73, stream/drive/ms 3.31 |
| warm-up | 6.56 | 30.6 | render/thread/ms 37.51, main/busy/ms 37.10, render/busy/ms 36.26, encode//sum 21.39, stage/main/update 18.37, stage/render/render 13.17, stage/main/post_update 12.93, render/graph/ms 11.35, stage/render/graph_submit 7.74, stage/render/prepare_assets 6.91, stream/drive/ms 5.24, encode/finish_wall 4.18 |
| warm-up | 6.59 | 30.1 | render/thread/ms 49.70, render/busy/ms 48.23, main/busy/ms 44.21, encode//sum 24.56, stage/main/post_update 19.56, stage/render/render 17.58, stage/main/update 16.42, render/graph/ms 15.52, stage/render/graph_submit 11.67, stage/render/prepare_assets 11.15, encode/submit 6.71, stream/drive/ms 5.47 |
| warm-up | 6.69 | 38.8 | main/busy/ms 46.13, render/thread/ms 34.31, render/busy/ms 33.14, stage/main/update 25.16, encode//sum 19.95, stream/drive/ms 15.21, stage/main/post_update 15.06, stage/render/render 12.12, render/graph/ms 10.43, stage/render/prepare_assets 7.91, stage/render/graph_submit 7.59, encode/finish_wall 3.96 |
| warm-up | 6.72 | 30.1 | render/thread/ms 42.16, render/busy/ms 40.90, main/busy/ms 39.89, stage/main/update 20.73, encode//sum 17.71, stage/render/prepare_assets 16.84, stage/main/post_update 12.51, stage/render/render 10.60, render/graph/ms 9.02, stage/render/graph_submit 6.18, stream/drive/ms 4.24, encode/finish_wall 3.79 |
| warm-up | 6.84 | 30.2 | render/thread/ms 44.08, render/busy/ms 42.50, main/busy/ms 30.28, encode//sum 25.94, stage/render/render 16.18, stage/main/post_update 15.95, render/graph/ms 13.94, stage/render/prepare_assets 12.90, stage/render/graph_submit 10.75, stage/main/update 10.53, encode/submit 5.54, encode/finish_wall 5.30 |
| warm-up | 6.92 | 31.3 | render/thread/ms 47.84, render/busy/ms 46.64, main/busy/ms 32.68, stage/render/prepare_assets 26.36, encode//sum 21.02, stage/main/update 14.58, stage/main/post_update 12.90, stage/render/render 12.80, render/graph/ms 11.01, stage/render/graph_submit 8.26, stream/drive/ms 6.88, encode/finish_wall 4.45 |
| warm-up | 7.10 | 33.6 | main/busy/ms 40.03, render/thread/ms 26.67, render/busy/ms 24.90, stage/main/update 21.91, encode//sum 21.79, stream/drive/ms 15.02, stage/render/render 14.21, stream/spawn/ms 13.72, stage/main/post_update 12.89, render/graph/ms 11.97, stage/render/graph_submit 9.18, encode/finish_wall 4.99 |
| measure | 18.16 | 31.2 | main/busy/ms 39.33, stage/main/update 24.13, render/thread/ms 20.76, stream/drive/ms 20.63, render/busy/ms 19.73, encode//sum 19.14, stage/main/post_update 12.52, stage/render/render 12.40, render/graph/ms 10.81, stage/render/graph_submit 8.65, encode/finish_wall 4.76, encode/submit 3.95 |
| measure | 59.03 | 29.6 | render/thread/ms 41.20, render/busy/ms 39.98, main/busy/ms 26.09, encode//sum 22.69, stage/render/prepare_assets 14.48, stage/main/post_update 14.15, stage/render/render 14.10, render/graph/ms 12.38, stage/render/graph_submit 9.15, stage/main/update 6.75, encode/finish_wall 5.33, encode/submit 3.87 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_noenc\balmora_walk@2560x1440_off=cascadeencoders-2 --bench-baseline none --off cascadeencoders
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.4 W · 71 °C (max 77) · VRAM 5992 MiB · throttled 100% of samples
