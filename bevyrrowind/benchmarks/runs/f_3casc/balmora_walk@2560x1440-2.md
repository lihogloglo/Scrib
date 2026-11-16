# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5601 frames over 65.6 s · **median 11.43 ms (88 fps)** · mean 11.72 · p95 14.26 · p99 16.86 (1% low 59 fps) · worst 31.99 ms · 49 spikes over 1.5× median · 29.2 stutters/min (work 30.2/min)
- warm-up: 324 frames over 9.3 s, worst 2528.2 ms
- hitches (> 28.6 ms): **2** measured, 42 in warm-up
- timed GPU passes: 7.43 ms of a 11.72 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.43 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- pass worst frame 31.99 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.070 | 2.090 | 2.339 | 0.99 | 0.174 |
| main_opaque_pass_3d | 1.657 | 1.673 | 2.080 | 0.99 | 0.059 |
| early prepass | 0.543 | 0.549 | 0.847 | 0.99 | 0.083 |
| bloom | 0.320 | 0.323 | 0.327 | 0.99 | 0.064 |
| atmosphere_luts | 0.314 | 0.317 | 0.389 | 0.99 | 0.003 |
| water_velocity | 0.284 | 0.286 | 0.354 | 0.99 | 0.002 |
| water_sspr | 0.205 | 0.207 | 0.206 | 0.99 | 0.002 |
| ssao | 0.195 | 0.197 | 0.204 | 0.99 | 0.007 |
| uniform_allocation | 0.149 | 0.151 | 0.341 | 0.99 | 0.024 |
| atmos_froxel_inject | 0.148 | 0.149 | 0.150 | 0.99 | 0.001 |
| clouds_trace | 0.141 | 0.143 | 0.236 | 0.99 | 0.001 |
| water_fft | 0.134 | 0.135 | 0.140 | 0.99 | 0.003 |
| auxiliary_views | 0.122 | 0.123 | 0.528 | 0.99 | 0.111 |
| upscaling | 0.119 | 0.120 | 0.134 | 0.99 | 0.004 |
| per_view_shadow_pass_cascade2 | 0.116 | 0.460 | 0.525 | 0.25 | 0.035 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 0.99 | 0.002 |
| bin_unpacking | 0.104 | 0.105 | 0.139 | 0.99 | 0.017 |
| tonemapping | 0.086 | 0.087 | 0.090 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.084 | 0.085 | 0.165 | 0.99 | 0.008 |
| clustering | 0.065 | 0.066 | 0.068 | 0.99 | 0.019 |
| render_sky | 0.052 | 0.053 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.429 | 0.558 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.045 | 0.060 | 0.058 | 0.74 | 0.002 |
| per_view_shadow_pass_cascade1 | 0.041 | 0.164 | 0.175 | 0.25 | 0.035 |
| early_mesh_preprocessing | 0.031 | 0.032 | 0.047 | 0.99 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.230 | 0.237 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.027 | 0.99 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.025 | 0.99 | 0.115 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.012 | 0.022 | 0.99 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.017 | 0.018 | 0.00 | 0.010 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.842 | 16.622 | 46.66 | 60726.1 |
| render/thread/ms | 10.409 | 12.744 | 25.26 | 58301.5 |
| render/busy/ms | 9.838 | 12.141 | 24.64 | 55104.8 |
| main/busy/ms | 9.456 | 11.656 | 29.45 | 52960.4 |
| stage/render/render | 6.646 | 8.170 | 16.71 | 37226.4 |
| stage/main/post_update | 5.961 | 7.876 | 12.91 | 33387.5 |
| render/graph/ms | 5.880 | 7.383 | 15.80 | 32935.8 |
| stage/render/graph_submit | 4.737 | 5.924 | 11.54 | 26533.2 |
| encode/finish_wall | 2.454 | 3.217 | 5.57 | 13745.1 |
| encode/submit | 2.339 | 3.129 | 6.88 | 13102.3 |
| stage/main/update | 2.204 | 2.643 | 22.76 | 12343.0 |
| stage/render/camera_driver | 1.087 | 1.495 | 5.01 | 6088.7 |
| stage/render/prepare_write_phase_buffers | 0.749 | 1.064 | 3.14 | 4196.8 |
| bigspace/total/ms | 0.642 | 2.176 | 3.54 | 3598.6 |
| stage/render/prepare_assets | 0.566 | 0.738 | 11.20 | 3169.5 |
| render/present/ms | 0.478 | 0.588 | 2.09 | 2676.9 |
| stage/render/prepare_resources | 0.462 | 0.569 | 1.60 | 2589.5 |
| stage/main/first | 0.413 | 0.543 | 5.35 | 2315.3 |
| stage/render/prepare_meshes | 0.390 | 0.518 | 2.07 | 2186.2 |
| stage/main/pre_update | 0.338 | 0.500 | 1.57 | 1892.6 |
| terrain/update/ms | 0.311 | 0.410 | 2.27 | 1742.1 |
| stage/render/specialize | 0.290 | 0.430 | 2.67 | 1621.8 |
| stage/render/prepare_bind_groups | 0.281 | 0.499 | 2.36 | 1574.4 |
| terrain/select/ms | 0.277 | 0.363 | 0.83 | 1551.0 |
| controller/feed/ms | 0.271 | 0.351 | 0.69 | 1517.6 |
| bigspace/low_precision/ms | 0.267 | 0.909 | 3.16 | 1496.0 |
| stream/drive/ms | 0.253 | 0.351 | 20.60 | 1419.2 |
| stage/main/fixed_main_loop | 0.236 | 0.393 | 1.28 | 1322.4 |
| stage/render/queue | 0.217 | 0.379 | 2.41 | 1216.9 |
| bigspace/high_precision/ms | 0.216 | 0.959 | 3.18 | 1210.6 |
| anim/evaluate/ms | 0.216 | 0.422 | 2.72 | 1209.2 |
| stream/tick/ms | 0.195 | 0.265 | 0.77 | 1091.2 |
| stage/render/create_views | 0.189 | 0.250 | 1.26 | 1056.5 |
| far/select/ms | 0.187 | 0.253 | 0.69 | 1045.6 |
| stage/render/prepare_flush | 0.176 | 0.369 | 1.11 | 987.4 |
| stage/render/extract_commands | 0.176 | 0.214 | 0.59 | 983.3 |
| stage/render/prepare_batch_phases | 0.163 | 0.228 | 0.71 | 911.8 |
| anim/evaluate_cpu/ms | 0.145 | 0.194 | 0.65 | 811.8 |
| stage/render/prepare_views | 0.135 | 0.175 | 0.47 | 758.9 |
| controller/move/ms | 0.132 | 0.241 | 0.47 | 740.7 |
| stage/main/last | 0.123 | 0.208 | 2.31 | 688.4 |
| render/acquire/ms | 0.093 | 0.122 | 0.43 | 519.8 |
| stage/render/phase_sort | 0.082 | 0.115 | 0.35 | 459.0 |
| pipelines/compile_ms | 0.052 | 0.000 | 120.59 | 293.0 |
| physics/step/ms | 0.051 | 0.088 | 0.57 | 285.1 |
| pipelines/front_ms | 0.050 | 0.000 | 117.06 | 281.8 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.075 | 0.29 | 270.0 |
| physics/insert/ms | 0.035 | 0.378 | 0.87 | 195.1 |
| stage/render/cleanup | 0.032 | 0.091 | 1.02 | 177.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.55 | 174.2 |
| weather/map/ms | 0.026 | 0.033 | 0.26 | 144.1 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.31 | 83.6 |
| anim/apply/ms | 0.011 | 0.020 | 0.27 | 62.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.44 | 61.4 |
| weather/eval/ms | 0.011 | 0.014 | 0.17 | 59.6 |
| controller/avoid/ms | 0.010 | 0.013 | 0.18 | 57.7 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 40.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.15 | 29.6 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 28.1 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 27.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.8 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 19.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 14.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.94 | 11.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.15 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.0 |
| stage/render/prepare | 0.001 | 0.002 | 0.03 | 2.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 595 |
| crowd/c0/meshes/count | count | 353 | 646 | 158 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5601 |
| crowd/c1/meshes/count | count | 1166 | 1268 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4589 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.05 | 4.24 | 2.49 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.31 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.44 | 0.01 |
| encode/finish_critical | ms | 2.05 | 4.24 | 2.49 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.55 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.92 | 29.45 | 9.17 |
| gpu/timed/ms | ms | 8.63 | 11.46 | 9.14 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 193 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 825 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 193 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 825 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3447 | 5128 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3447 | 5128 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 198 | 4 |
| phases/shadow/bins | count | 4899 | 5814 | 2333 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.1 k | 13.4 k | 6111 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 244 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
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
| preprocess/bind_groups | count | 14 | 553 | 6 |
| preprocess/bind_groups_created | count | 14 | 553 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 227 | 123 |
| process/cpu_usage | % | 26.89 | 39.52 | 25.71 |
| process/mem_usage | GiB | 5.40 | 5.57 | 5.57 |
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
| system/cpu_usage | % | 33.82 | 53.08 | 33.69 |
| system/mem_usage | % | 57.32 | 58.96 | 56.72 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 61 | 245 | 54 |
| views/all/bins | count | 7508 | 8729 | 5831 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.1 k | 25.9 k | 13.7 k |
| views/all/pipelines | count | 248 | 287 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.6 k | 23.2 k | 8063 |
| views/cascade/batch_sets | count | 7 | 46 | 4 |
| views/cascade/bins | count | 2656 | 3224 | 2333 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8550 | 10.5 k | 6111 |
| views/cascade/pipelines | count | 33 | 46 | 29 |
| views/cascade/work_items | count | 244 | 10.5 k | 145 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2168 | 2754 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2420 | 3053 | 0 |
| views/lamp_face/pipelines | count | 169 | 200 | 0 |
| views/lamp_face/work_items | count | 0 | 2917 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2874 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8560 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8716 | 13.2 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 50.77 | 238 | 284.3 k |
| anim/events/count | count | 0.09 | 3 | 478 |
| assets/Image/added | count | 0.11 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.87 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1566 | 4675 |
| encode//count | count | 38.15 | 272 | 213.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5601 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5601 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5601 |
| physics/inserts | count | 0.40 | 95 | 2261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3938 |
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
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6044 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1325 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2528.2 | main/busy/ms 2500, stage/main/post_update 2395, stage/main/update 96.77, surface/advance_cpu_ms 67.34, stage/main/pre_update 3.56, stage/main/last 1.20, surface/cell_upload_ms 0.70, stage/main/first 0.60, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.37, weather/map/ms 0.17, stream/tick/ms 0.10 |
| warm-up | 3.31 | 777.8 | main/busy/ms 2641, stage/main/post_update 2421, render/thread/ms 749, render/busy/ms 583, stage/render/prepare_assets 535, stage/main/update 206, stage/render/prepare_views 164, render/acquire/ms 164, surface/advance_cpu_ms 67.34, stage/render/create_views 27.88, stage/render/render 12.45, encode//sum 12.37 |
| warm-up | 3.40 | 95.5 | render/thread/ms 827, render/busy/ms 660, stage/render/prepare_assets 585, main/busy/ms 168, stage/render/prepare_views 164, render/acquire/ms 164, pipelines/compile_ms 115, stage/main/update 115, pipelines/front_ms 109, stage/main/post_update 38.23, stage/render/render 31.76, stage/render/create_views 28.25 |
| warm-up | 3.44 | 42.3 | pipelines/compile_ms 294, pipelines/front_ms 272, render/thread/ms 118, render/busy/ms 107, pipelines/wait_ms 98.15, main/busy/ms 76.87, stage/render/render 51.43, stage/render/prepare_assets 50.10, stage/main/post_update 48.14, render/graph/ms 39.65, terrain/update/ms 24.06, stage/render/camera_driver 22.13 |
| warm-up | 3.55 | 106.3 | pipelines/wait_ms 3157, pipelines/compile_ms 916, pipelines/front_ms 837, render/thread/ms 135, render/busy/ms 124, main/busy/ms 107, stage/main/post_update 80.37, pipelines/driver_ms 78.81, stage/render/prepare_resources 59.78, stage/render/render 36.66, terrain/update/ms 31.99, render/graph/ms 25.22 |
| warm-up | 3.63 | 77.3 | pipelines/wait_ms 5076, pipelines/compile_ms 2324, pipelines/front_ms 2196, render/thread/ms 170, render/busy/ms 168, main/busy/ms 153, pipelines/driver_ms 128, stage/render/prepare_resources 81.52, stage/main/update 77.76, stage/main/post_update 57.10, stage/render/prepare_assets 38.62, terrain/update/ms 31.37 |
| warm-up | 3.73 | 101.8 | pipelines/wait_ms 2395, pipelines/compile_ms 2354, pipelines/front_ms 2252, render/thread/ms 148, render/busy/ms 146, main/busy/ms 139, pipelines/driver_ms 102, stage/render/prepare_assets 87.16, stage/main/update 80.72, stage/main/post_update 40.15, stage/render/prepare_resources 26.12, bigspace/total/ms 15.26 |
| warm-up | 3.80 | 38.6 | pipelines/compile_ms 430, pipelines/front_ms 410, pipelines/wait_ms 169, render/thread/ms 60.20, render/busy/ms 58.50, main/busy/ms 42.67, stage/render/prepare_assets 34.27, pipelines/driver_ms 20.18, stage/main/post_update 16.58, stage/render/render 11.98, stage/main/pre_update 11.51, stage/main/update 9.90 |
| warm-up | 3.83 | 31.9 | pipelines/compile_ms 862, pipelines/front_ms 820, pipelines/wait_ms 271, render/thread/ms 65.07, render/busy/ms 62.80, pipelines/driver_ms 42.53, main/busy/ms 36.39, encode//sum 25.95, stage/render/prepare_assets 22.17, stage/render/render 16.29, stage/main/post_update 14.02, render/graph/ms 13.63 |
| warm-up | 3.87 | 29.3 | pipelines/compile_ms 651, pipelines/front_ms 628, pipelines/wait_ms 71.45, render/thread/ms 41.57, render/busy/ms 39.51, main/busy/ms 28.81, pipelines/driver_ms 23.00, stage/render/prepare_assets 18.42, stage/main/post_update 14.99, stage/render/render 11.59, terrain/update/ms 9.25, render/graph/ms 9.02 |
| warm-up | 3.93 | 58.9 | pipelines/compile_ms 533, pipelines/front_ms 458, render/thread/ms 85.06, render/busy/ms 80.44, pipelines/driver_ms 74.74, main/busy/ms 68.10, stage/render/render 52.50, render/graph/ms 47.14, stage/main/update 43.84, stage/render/graph_submit 43.46, encode//sum 41.83, stream/drive/ms 40.40 |
| warm-up | 3.99 | 58.4 | pipelines/compile_ms 1292, pipelines/front_ms 1186, pipelines/driver_ms 106, render/thread/ms 103, render/busy/ms 98.26, main/busy/ms 65.41, stage/render/render 52.62, render/graph/ms 47.31, stage/main/update 46.22, stage/render/graph_submit 43.74, encode//sum 41.85, stream/drive/ms 39.95 |
| warm-up | 4.02 | 32.3 | pipelines/compile_ms 885, pipelines/front_ms 845, render/thread/ms 73.53, render/busy/ms 71.66, main/busy/ms 43.71, stage/render/prepare_assets 43.18, pipelines/driver_ms 39.70, stage/main/update 27.22, stage/render/render 19.70, render/graph/ms 17.42, stream/drive/ms 10.98, stage/render/graph_submit 10.66 |
| warm-up | 4.05 | 33.3 | pipelines/compile_ms 357, pipelines/front_ms 341, render/thread/ms 55.72, render/busy/ms 53.95, main/busy/ms 40.41, stage/render/prepare_assets 26.18, stage/main/update 23.99, stage/render/render 19.53, render/graph/ms 17.44, pipelines/driver_ms 16.08, stage/render/graph_submit 10.77, stream/drive/ms 10.48 |
| warm-up | 4.09 | 34.9 | pipelines/compile_ms 557, pipelines/front_ms 537, main/busy/ms 58.76, render/thread/ms 56.68, render/busy/ms 54.64, stage/render/prepare_assets 31.40, stage/main/update 31.27, stream/drive/ms 24.53, pipelines/driver_ms 19.91, stage/main/post_update 19.14, stage/render/render 11.23, encode//sum 9.20 |
| warm-up | 4.16 | 71.0 | pipelines/compile_ms 530, pipelines/front_ms 509, render/thread/ms 71.24, render/busy/ms 68.64, main/busy/ms 63.94, stage/render/prepare_assets 41.62, stage/main/update 32.82, stream/drive/ms 25.92, stage/main/post_update 23.65, pipelines/driver_ms 20.82, stage/render/render 14.03, render/graph/ms 10.52 |
| warm-up | 4.20 | 38.8 | pipelines/compile_ms 325, pipelines/front_ms 311, render/thread/ms 68.71, render/busy/ms 65.89, stage/render/prepare_assets 45.60, main/busy/ms 33.03, pipelines/driver_ms 13.99, stage/render/render 13.79, stage/main/post_update 12.56, stage/main/update 12.56, render/graph/ms 10.18, stage/render/graph_submit 7.13 |
| warm-up | 4.28 | 78.4 | pipelines/compile_ms 1291, pipelines/front_ms 1229, render/thread/ms 81.91, render/busy/ms 79.34, main/busy/ms 71.97, pipelines/driver_ms 62.07, stage/render/prepare_assets 57.66, stage/main/update 46.69, stage/main/post_update 12.64, stage/render/render 10.07, render/graph/ms 7.28, pipelines/wait_ms 6.21 |
| warm-up | 4.35 | 71.9 | pipelines/compile_ms 1490, pipelines/front_ms 1424, render/thread/ms 107, render/busy/ms 105, main/busy/ms 90.86, stage/render/prepare_assets 78.90, pipelines/driver_ms 65.44, stage/main/update 52.30, stage/main/post_update 25.86, stage/render/render 10.05, render/graph/ms 7.80, encode//sum 7.42 |
| warm-up | 4.40 | 53.9 | pipelines/compile_ms 796, pipelines/front_ms 772, render/thread/ms 94.23, render/busy/ms 92.57, main/busy/ms 61.92, stage/render/prepare_assets 54.97, stage/main/update 28.32, pipelines/wait_ms 27.56, stage/main/post_update 24.84, pipelines/driver_ms 24.07, stage/render/render 12.30, encode//sum 11.30 |
| warm-up | 4.51 | 102.8 | pipelines/compile_ms 2407, pipelines/front_ms 2345, pipelines/wait_ms 344, render/thread/ms 134, render/busy/ms 132, main/busy/ms 76.31, pipelines/driver_ms 61.58, stage/render/prepare_assets 59.53, stage/main/post_update 45.30, encode//sum 29.00, stage/main/update 23.81, stage/render/render 21.57 |
| warm-up | 4.56 | 58.3 | pipelines/compile_ms 2550, pipelines/front_ms 2471, pipelines/wait_ms 794, render/thread/ms 148, render/busy/ms 143, encode//sum 83.04, pipelines/driver_ms 79.37, main/busy/ms 74.07, stage/render/prepare_assets 46.69, stage/main/post_update 44.61, stage/render/render 39.10, render/graph/ms 36.41 |
| warm-up | 4.65 | 85.9 | pipelines/compile_ms 2247, pipelines/front_ms 2161, pipelines/wait_ms 1742, render/thread/ms 128, render/busy/ms 122, pipelines/driver_ms 85.42, encode//sum 77.64, stage/render/render 44.35, main/busy/ms 44.11, render/graph/ms 40.82, stage/render/prepare_assets 29.12, stage/render/graph_submit 26.97 |
| warm-up | 4.75 | 95.0 | pipelines/compile_ms 2421, pipelines/wait_ms 2385, pipelines/front_ms 2345, render/thread/ms 160, render/busy/ms 157, main/busy/ms 106, pipelines/driver_ms 75.44, stage/main/update 71.64, stream/drive/ms 60.93, stage/render/prepare_assets 54.59, stage/render/render 29.14, stage/render/prepare_bind_groups 25.02 |
| warm-up | 4.81 | 63.7 | pipelines/compile_ms 1486, pipelines/front_ms 1430, pipelines/wait_ms 1405, render/thread/ms 147, render/busy/ms 144, main/busy/ms 136, stage/main/update 85.84, stream/drive/ms 62.98, pipelines/driver_ms 56.24, stage/render/prepare_resources 37.27, stage/render/prepare_assets 37.26, stage/main/post_update 28.50 |
| warm-up | 4.89 | 80.4 | pipelines/compile_ms 2168, pipelines/front_ms 2092, pipelines/wait_ms 1489, main/busy/ms 137, render/thread/ms 119, render/busy/ms 116, stage/main/update 101, pipelines/driver_ms 75.74, stream/drive/ms 56.12, stage/render/prepare_assets 30.83, stage/main/post_update 25.46, stage/render/render 24.57 |
| warm-up | 5.03 | 137.8 | pipelines/compile_ms 2368, pipelines/front_ms 2300, pipelines/wait_ms 1531, render/thread/ms 145, render/busy/ms 143, main/busy/ms 114, stage/main/update 92.05, stage/render/prepare_assets 69.72, pipelines/driver_ms 67.92, stream/drive/ms 54.40, stage/render/prepare_resources 24.94, stage/render/render 23.68 |
| warm-up | 5.07 | 45.5 | pipelines/compile_ms 917, pipelines/front_ms 888, pipelines/wait_ms 330, render/thread/ms 124, render/busy/ms 122, main/busy/ms 97.24, stage/main/update 70.26, stage/render/prepare_assets 55.87, stream/drive/ms 49.48, pipelines/driver_ms 28.72, stage/render/render 28.49, render/graph/ms 26.00 |
| warm-up | 5.15 | 76.7 | pipelines/compile_ms 101, pipelines/front_ms 94.58, main/busy/ms 91.22, stage/main/update 69.49, render/thread/ms 68.89, render/busy/ms 67.06, stream/drive/ms 49.39, stage/render/render 27.73, stage/render/prepare_assets 27.72, render/graph/ms 25.18, stage/render/graph_submit 21.18, encode//sum 18.18 |
| warm-up | 5.23 | 59.8 | pipelines/compile_ms 1327, pipelines/front_ms 1293, main/busy/ms 64.09, stage/main/update 48.95, render/thread/ms 40.30, render/busy/ms 38.51, stream/drive/ms 35.09, pipelines/driver_ms 33.51, encode//sum 14.70, stage/render/render 13.15, stage/render/prepare_assets 12.65, render/graph/ms 10.76 |
| warm-up | 5.26 | 29.7 | pipelines/compile_ms 1721, pipelines/front_ms 1678, render/thread/ms 47.91, render/busy/ms 46.02, pipelines/driver_ms 43.23, main/busy/ms 29.18, encode//sum 25.99, stage/render/render 15.96, render/graph/ms 13.32, stage/main/update 13.12, stage/render/prepare_assets 12.07, stage/main/post_update 10.57 |
| warm-up | 5.41 | 55.6 | pipelines/compile_ms 125, pipelines/front_ms 121, render/thread/ms 60.53, render/busy/ms 58.28, main/busy/ms 41.70, stage/render/prepare_assets 27.91, stage/main/update 23.64, stage/render/render 22.71, render/graph/ms 19.93, encode//sum 18.29, stage/render/graph_submit 17.44, stream/drive/ms 11.77 |
| warm-up | 5.44 | 38.6 | pipelines/compile_ms 771, pipelines/front_ms 747, render/thread/ms 67.82, render/busy/ms 65.41, main/busy/ms 47.83, stage/render/prepare_assets 33.58, stage/main/update 30.33, pipelines/driver_ms 24.44, stage/render/render 23.92, render/graph/ms 20.83, stage/render/graph_submit 15.14, encode//sum 15.09 |
| warm-up | 5.49 | 47.8 | pipelines/compile_ms 1274, pipelines/front_ms 1235, render/thread/ms 64.78, render/busy/ms 62.71, main/busy/ms 47.76, pipelines/driver_ms 38.26, stage/render/prepare_assets 30.97, stage/main/update 29.41, stage/render/render 23.67, render/graph/ms 20.81, encode//sum 15.32, stage/render/graph_submit 14.98 |
| warm-up | 5.53 | 32.8 | pipelines/compile_ms 721, pipelines/front_ms 700, render/thread/ms 64.56, render/busy/ms 62.90, stage/render/prepare_assets 36.07, main/busy/ms 35.21, pipelines/wait_ms 23.32, pipelines/driver_ms 21.54, stage/main/update 17.34, stage/render/render 17.08, encode//sum 15.35, render/graph/ms 14.72 |
| warm-up | 5.56 | 31.1 | pipelines/compile_ms 316, pipelines/front_ms 305, render/thread/ms 55.08, render/busy/ms 53.55, stage/render/prepare_assets 33.66, main/busy/ms 32.28, encode//sum 16.53, stage/main/update 15.68, pipelines/wait_ms 14.77, pipelines/driver_ms 11.44, stage/render/render 10.96, stage/main/post_update 10.60 |
| warm-up | 5.66 | 34.7 | pipelines/compile_ms 570, pipelines/front_ms 554, render/thread/ms 53.77, render/busy/ms 52.23, stage/render/prepare_assets 32.65, main/busy/ms 29.56, encode//sum 17.98, pipelines/driver_ms 16.73, pipelines/wait_ms 15.28, stage/main/post_update 14.26, stage/render/render 12.74, render/graph/ms 10.55 |
| warm-up | 5.72 | 34.5 | pipelines/compile_ms 780, pipelines/front_ms 760, render/thread/ms 50.14, render/busy/ms 48.54, main/busy/ms 28.43, stage/render/prepare_assets 24.71, pipelines/driver_ms 20.10, encode//sum 14.94, stage/render/render 13.29, stage/main/post_update 12.19, render/graph/ms 11.04, stage/main/update 10.95 |
| warm-up | 6.24 | 30.6 | render/thread/ms 38.82, render/busy/ms 37.65, main/busy/ms 27.97, stage/render/prepare_assets 16.02, encode//sum 13.42, stage/main/update 11.83, stage/render/render 9.50, stage/main/post_update 9.35, render/graph/ms 7.75, stage/render/graph_submit 5.73, stage/render/specialize 4.26, stream/drive/ms 3.55 |
| warm-up | 6.50 | 31.4 | pipelines/compile_ms 80.10, pipelines/front_ms 76.43, render/thread/ms 41.62, render/busy/ms 40.49, main/busy/ms 30.00, stage/render/prepare_assets 21.93, encode//sum 16.23, stage/main/update 13.86, stage/main/post_update 9.59, stage/render/render 9.46, render/graph/ms 7.86, stage/render/graph_submit 5.88 |
| warm-up | 6.60 | 35.6 | render/thread/ms 41.09, render/busy/ms 39.85, main/busy/ms 34.85, encode//sum 23.08, stage/main/update 18.58, stage/render/prepare_assets 17.73, stage/render/render 13.29, render/graph/ms 11.28, stage/main/post_update 11.22, stage/render/graph_submit 7.89, encode/finish_wall 4.92, stream/drive/ms 3.95 |
| warm-up | 6.75 | 29.6 | render/thread/ms 50.69, render/busy/ms 49.46, main/busy/ms 32.10, stage/render/prepare_assets 29.60, encode//sum 21.62, stage/main/update 15.03, stage/render/render 11.97, stage/main/post_update 10.80, render/graph/ms 10.24, stage/render/graph_submit 7.84, stream/drive/ms 6.32, encode/finish_wall 4.60 |
| measure | 18.19 | 32.0 | main/busy/ms 40.33, stage/main/update 25.04, render/thread/ms 21.45, encode//sum 21.23, stream/drive/ms 20.84, render/busy/ms 20.35, stage/render/render 12.82, stage/main/post_update 12.41, render/graph/ms 11.31, stage/render/graph_submit 9.16, encode/submit 4.67, encode/finish_wall 4.56 |
| measure | 59.05 | 31.4 | render/thread/ms 41.63, render/busy/ms 40.19, encode//sum 31.70, main/busy/ms 28.08, stage/render/render 16.22, stage/main/post_update 15.18, render/graph/ms 14.43, stage/render/prepare_assets 13.73, stage/render/graph_submit 11.03, stage/main/update 8.23, encode/finish_wall 6.59, encode/submit 4.54 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_3casc\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 5%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **76%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.4 W · 72 °C (max 77) · VRAM 5280 MiB · throttled 100% of samples
