# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5635 frames over 65.6 s · **median 11.47 ms (87 fps)** · mean 11.65 · p95 13.64 · p99 15.58 (1% low 64 fps) · worst 32.29 ms · 26 spikes over 1.5× median · 23.8 stutters/min (work 20.1/min)
- warm-up: 394 frames over 8.8 s, worst 2624.2 ms
- hitches (> 28.7 ms): **3** measured, 28 in warm-up
- timed GPU passes: 7.20 ms of a 11.65 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.47 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- pass worst frame 32.29 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.032 | 2.080 | 2.228 | 0.98 | 0.174 |
| main_opaque_pass_3d | 1.614 | 1.652 | 2.041 | 0.98 | 0.054 |
| early prepass | 0.508 | 0.520 | 0.752 | 0.98 | 0.078 |
| bloom | 0.314 | 0.322 | 0.327 | 0.98 | 0.062 |
| atmosphere_luts | 0.305 | 0.313 | 0.372 | 0.98 | 0.003 |
| water_velocity | 0.281 | 0.287 | 0.354 | 0.98 | 0.002 |
| water_sspr | 0.206 | 0.210 | 0.211 | 0.98 | 0.002 |
| ssao | 0.189 | 0.193 | 0.195 | 0.98 | 0.007 |
| uniform_allocation | 0.179 | 0.183 | 0.189 | 0.98 | 0.031 |
| atmos_froxel_inject | 0.145 | 0.149 | 0.150 | 0.98 | 0.001 |
| clouds_trace | 0.137 | 0.140 | 0.231 | 0.98 | 0.001 |
| bin_unpacking | 0.131 | 0.134 | 0.143 | 0.98 | 0.022 |
| water_fft | 0.130 | 0.133 | 0.138 | 0.98 | 0.003 |
| auxiliary_views | 0.118 | 0.121 | 0.505 | 0.98 | 0.110 |
| upscaling | 0.114 | 0.117 | 0.126 | 0.98 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.111 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.087 | 0.089 | 0.168 | 0.98 | 0.008 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.98 | 0.003 |
| clustering | 0.066 | 0.068 | 0.070 | 0.98 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| surface_trail | 0.051 | 0.069 | 0.065 | 0.74 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.050 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.049 | 0.050 | 0.98 | 0.005 |
| lightprobe_irradiance_map | 0.047 | 0.426 | 0.423 | 0.11 | 0.001 |
| water_snapshot | 0.032 | 0.033 | 0.036 | 0.98 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| lightprobe_radiance_map | 0.025 | 0.223 | 0.234 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.025 | 0.028 | 0.98 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.023 | 0.98 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.98 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.008 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 20.162 | 0.000 | 11088.02 | 113613.1 |
| render/thread/ms | 10.377 | 12.231 | 27.83 | 58476.4 |
| encode//sum | 10.358 | 14.166 | 54.20 | 58368.9 |
| render/busy/ms | 9.805 | 11.648 | 27.25 | 55253.7 |
| main/busy/ms | 9.027 | 11.118 | 28.62 | 50869.8 |
| stage/render/render | 6.686 | 8.029 | 15.51 | 37674.4 |
| render/graph/ms | 5.924 | 7.211 | 14.39 | 33380.0 |
| stage/main/post_update | 5.656 | 7.525 | 9.32 | 31871.9 |
| stage/render/graph_submit | 4.806 | 5.882 | 10.83 | 27079.2 |
| encode/finish_wall | 2.630 | 3.440 | 6.31 | 14819.8 |
| encode/submit | 2.234 | 2.938 | 6.36 | 12586.9 |
| stage/main/update | 2.121 | 2.537 | 22.51 | 11951.7 |
| stage/render/camera_driver | 1.062 | 1.378 | 3.48 | 5981.8 |
| stage/render/prepare_write_phase_buffers | 0.877 | 1.107 | 2.21 | 4939.8 |
| bigspace/total/ms | 0.605 | 2.074 | 3.73 | 3407.3 |
| stage/render/prepare_assets | 0.540 | 0.692 | 11.76 | 3043.7 |
| render/present/ms | 0.477 | 0.587 | 2.35 | 2689.3 |
| stage/render/prepare_resources | 0.429 | 0.517 | 3.34 | 2418.3 |
| stage/main/first | 0.406 | 0.526 | 4.87 | 2288.2 |
| stage/render/prepare_meshes | 0.343 | 0.457 | 1.64 | 1933.4 |
| stage/main/pre_update | 0.325 | 0.490 | 1.18 | 1834.1 |
| pipelines/compile_ms | 0.320 | 0.000 | 163.90 | 1800.8 |
| pipelines/front_ms | 0.308 | 0.000 | 157.75 | 1737.4 |
| terrain/update/ms | 0.296 | 0.381 | 1.95 | 1666.5 |
| stage/render/specialize | 0.268 | 0.396 | 2.30 | 1509.9 |
| terrain/select/ms | 0.263 | 0.338 | 0.82 | 1479.6 |
| controller/feed/ms | 0.262 | 0.339 | 0.61 | 1473.6 |
| stage/render/queue | 0.261 | 0.417 | 3.10 | 1473.4 |
| stream/drive/ms | 0.258 | 0.348 | 20.52 | 1452.3 |
| bigspace/low_precision/ms | 0.255 | 0.729 | 3.43 | 1439.1 |
| stage/render/prepare_bind_groups | 0.242 | 0.399 | 2.94 | 1365.2 |
| stage/main/fixed_main_loop | 0.228 | 0.372 | 1.17 | 1283.4 |
| stream/tick/ms | 0.198 | 0.271 | 0.90 | 1116.5 |
| bigspace/high_precision/ms | 0.190 | 0.656 | 3.06 | 1069.7 |
| far/select/ms | 0.189 | 0.258 | 0.68 | 1063.4 |
| stage/render/create_views | 0.181 | 0.233 | 7.98 | 1021.6 |
| stage/render/extract_commands | 0.170 | 0.206 | 0.59 | 959.7 |
| stage/render/prepare_batch_phases | 0.167 | 0.236 | 0.59 | 942.7 |
| anim/evaluate_cpu/ms | 0.161 | 0.206 | 0.53 | 909.6 |
| stage/render/prepare_flush | 0.153 | 0.287 | 0.87 | 864.2 |
| stage/render/prepare_views | 0.137 | 0.180 | 0.49 | 773.3 |
| controller/move/ms | 0.134 | 0.317 | 0.79 | 756.2 |
| anim/evaluate/ms | 0.130 | 0.230 | 2.82 | 732.8 |
| stage/main/last | 0.114 | 0.199 | 1.70 | 641.3 |
| render/acquire/ms | 0.095 | 0.130 | 0.45 | 533.4 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.123 | 0.34 | 324.2 |
| physics/step/ms | 0.050 | 0.086 | 0.89 | 280.7 |
| physics/insert/ms | 0.039 | 0.399 | 0.81 | 219.3 |
| stage/render/cleanup | 0.032 | 0.104 | 0.56 | 181.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.32 | 175.5 |
| stage/render/phase_sort | 0.028 | 0.049 | 0.21 | 157.8 |
| weather/map/ms | 0.024 | 0.033 | 0.20 | 135.6 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.26 | 82.1 |
| anim/apply/ms | 0.012 | 0.022 | 0.20 | 65.7 |
| pipelines/driver_ms | 0.011 | 0.000 | 6.15 | 63.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.27 | 61.9 |
| controller/avoid/ms | 0.010 | 0.014 | 0.20 | 57.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.22 | 57.3 |
| script/update/ms | 0.007 | 0.009 | 0.22 | 39.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.18 | 29.5 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 28.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 26.1 |
| script/ui/ms | 0.005 | 0.006 | 0.22 | 26.1 |
| controller/write/ms | 0.003 | 0.004 | 0.08 | 18.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 2.8 |
| weather/apply/ms | 0.000 | 0.000 | 0.02 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1306 | 2262 | 482 |
| crowd/c0/meshes/count | count | 355 | 646 | 119 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5983 | 5682 |
| crowd/c1/meshes/count | count | 1172 | 1297 | 1214 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4254 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4235 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.20 | 4.50 | 1.96 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.26 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.27 | 0.01 |
| encode/finish_critical | ms | 2.20 | 4.50 | 1.96 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.32 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.81 | 28.62 | 9.33 |
| gpu/timed/ms | ms | 8.03 | 9.96 | 9.33 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 193 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 829 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 193 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 829 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1285 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3457 | 5115 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1285 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3457 | 5115 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 225 | 28 |
| phases/shadow/bins | count | 4810 | 5690 | 2229 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 13.1 k | 5680 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 242 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2652 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 720 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 10 | 609 | 6 |
| preprocess/bind_groups_created | count | 10 | 609 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 182 | 255 | 173 |
| process/cpu_usage | % | 25.94 | 45.76 | 26.42 |
| process/mem_usage | GiB | 5.08 | 5.36 | 5.36 |
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
| started_load_count |  | 4764 | 4859 | 4859 |
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
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.72 | 46.21 | 29.85 |
| system/mem_usage | % | 60.97 | 61.49 | 61.16 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 87 | 272 | 78 |
| views/all/bins | count | 7384 | 8522 | 5727 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 248 | 289 | 74 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 17.4 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2593 | 3113 | 2229 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8159 | 10.2 k | 5680 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8159 | 10.2 k | 5680 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2155 | 2786 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2396 | 3083 | 0 |
| views/lamp_face/pipelines | count | 168 | 203 | 0 |
| views/lamp_face/work_items | count | 0 | 2876 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2874 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8566 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8720 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 52.76 | 275 | 297.3 k |
| anim/events/count | count | 0.09 | 3 | 504 |
| assets/Image/added | count | 0.11 | 43 | 595 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.73 | 152 | 4089 |
| assets/Mesh/removed | count | 0.87 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 467 |
| assets/StandardMaterial/modified | count | 3.84 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.00 | 1386 | 5621 |
| encode//count | count | 37.74 | 274 | 212.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5635 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5635 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5635 |
| physics/inserts | count | 0.53 | 111 | 2998 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 20 | 378 |
| stream/upload/bytes | bytes | 6008 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1317 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.7 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2624.2 | main/busy/ms 2596, stage/main/post_update 2489, stage/main/update 101, surface/advance_cpu_ms 74.45, stage/main/pre_update 3.10, stage/main/last 1.05, surface/cell_upload_ms 0.73, stage/main/first 0.44, stage/main/fixed_main_loop 0.41, stream/drive/ms 0.35, weather/map/ms 0.14, stream/tick/ms 0.10 |
| warm-up | 3.37 | 743.3 | main/busy/ms 2631, stage/main/post_update 2514, render/thread/ms 704, render/busy/ms 657, stage/render/prepare_assets 462, stage/render/prepare_resources 139, stage/main/update 105, surface/advance_cpu_ms 74.45, stage/render/prepare_views 43.97, render/acquire/ms 43.83, stage/render/render 36.55, render/graph/ms 33.03 |
| warm-up | 3.49 | 125.7 | pipelines/wait_ms 1556, render/thread/ms 713, render/busy/ms 665, stage/render/prepare_assets 462, pipelines/compile_ms 144, stage/render/prepare_resources 141, pipelines/front_ms 136, main/busy/ms 50.33, stage/render/prepare_views 44.21, render/acquire/ms 44.01, stage/render/render 40.23, render/graph/ms 35.14 |
| warm-up | 3.74 | 91.2 | pipelines/compile_ms 235, pipelines/front_ms 227, main/busy/ms 65.57, render/thread/ms 58.23, render/busy/ms 56.12, stage/main/update 51.51, stage/render/prepare_assets 42.95, pipelines/driver_ms 8.21, stage/main/post_update 7.97, stage/render/render 7.42, render/graph/ms 4.71, stage/main/pre_update 3.13 |
| warm-up | 3.87 | 51.5 | render/thread/ms 43.98, render/busy/ms 42.52, main/busy/ms 40.52, stage/render/prepare_assets 32.23, stage/main/update 30.35, stream/drive/ms 27.16, stage/main/post_update 7.05, stage/render/render 5.93, render/graph/ms 4.09, stage/render/graph_submit 2.56, encode/submit 2.17, encode//sum 1.48 |
| warm-up | 3.94 | 54.5 | render/thread/ms 41.52, render/busy/ms 37.95, main/busy/ms 35.46, stage/render/prepare_assets 25.26, stage/main/update 23.25, stream/drive/ms 19.62, stage/render/render 8.01, stage/main/post_update 6.93, render/graph/ms 3.91, render/present/ms 3.20, stage/render/graph_submit 2.19, stage/main/pre_update 1.94 |
| warm-up | 4.04 | 72.7 | pipelines/wait_ms 7820, pipelines/compile_ms 288, pipelines/front_ms 268, render/thread/ms 55.26, render/busy/ms 53.81, main/busy/ms 43.88, stage/render/prepare_assets 41.79, stage/main/update 30.39, stream/drive/ms 25.49, pipelines/driver_ms 20.22, stage/main/post_update 7.88, stage/render/render 5.60 |
| warm-up | 4.07 | 32.5 | pipelines/wait_ms 7942, pipelines/compile_ms 118, pipelines/front_ms 110, render/thread/ms 55.99, render/busy/ms 54.73, main/busy/ms 50.21, stage/render/prepare_assets 45.05, stage/main/update 33.11, stream/drive/ms 27.67, stage/main/post_update 9.79, pipelines/driver_ms 8.55, stage/render/render 5.14 |
| warm-up | 4.13 | 53.9 | pipelines/wait_ms 2138, main/busy/ms 52.24, render/thread/ms 42.28, render/busy/ms 41.15, pipelines/compile_ms 36.63, pipelines/front_ms 34.59, stage/render/prepare_assets 31.81, stage/main/update 31.66, stream/drive/ms 26.46, stage/main/post_update 13.25, stage/render/render 4.97, render/graph/ms 3.52 |
| warm-up | 4.18 | 29.9 | render/thread/ms 46.09, render/busy/ms 43.76, main/busy/ms 26.85, stage/render/prepare_assets 15.29, stage/render/render 11.29, encode//sum 9.96, stage/main/update 9.65, stage/main/post_update 9.37, render/graph/ms 8.06, stage/render/specialize 4.75, stream/drive/ms 4.43, stage/render/graph_submit 4.19 |
| warm-up | 4.21 | 29.8 | render/thread/ms 48.29, render/busy/ms 46.38, main/busy/ms 20.30, stage/render/prepare_assets 16.65, encode//sum 11.07, stage/render/render 10.51, stage/main/update 9.43, render/graph/ms 7.88, stage/render/create_views 6.93, stage/main/post_update 6.16, stage/render/graph_submit 4.19, stream/drive/ms 3.83 |
| warm-up | 4.25 | 40.7 | pipelines/wait_ms 3675, pipelines/compile_ms 186, pipelines/front_ms 173, render/thread/ms 60.43, render/busy/ms 58.14, encode//sum 54.61, main/busy/ms 26.00, stage/render/render 21.64, render/graph/ms 18.60, stage/render/prepare_assets 16.87, stage/render/graph_submit 13.85, pipelines/driver_ms 13.69 |
| warm-up | 4.29 | 32.4 | pipelines/wait_ms 4940, pipelines/compile_ms 212, pipelines/front_ms 195, render/thread/ms 62.58, render/busy/ms 59.27, encode//sum 58.08, main/busy/ms 30.31, stage/render/render 27.17, render/graph/ms 22.86, pipelines/driver_ms 17.72, stage/render/graph_submit 15.84, stage/main/update 13.08 |
| warm-up | 4.33 | 47.1 | pipelines/wait_ms 3578, main/busy/ms 67.88, pipelines/compile_ms 65.14, pipelines/front_ms 58.96, render/thread/ms 55.07, stage/main/update 53.90, render/busy/ms 52.57, stream/drive/ms 49.02, stage/render/prepare_assets 24.01, stage/render/render 14.45, render/graph/ms 11.15, encode//sum 7.84 |
| warm-up | 4.38 | 48.3 | pipelines/wait_ms 2314, main/busy/ms 66.16, stage/main/update 52.87, stream/drive/ms 48.97, render/thread/ms 46.09, render/busy/ms 44.78, pipelines/compile_ms 39.08, pipelines/front_ms 36.94, stage/render/prepare_assets 30.69, stage/render/render 8.11, stage/main/post_update 6.44, render/graph/ms 6.25 |
| warm-up | 4.47 | 69.9 | pipelines/wait_ms 3312, pipelines/compile_ms 158, pipelines/front_ms 148, main/busy/ms 72.75, stage/main/update 56.05, stream/drive/ms 51.11, render/thread/ms 38.59, render/busy/ms 37.17, stage/render/prepare_assets 16.16, stage/main/post_update 11.16, pipelines/driver_ms 9.64, stage/render/render 8.08 |
| warm-up | 4.54 | 46.0 | pipelines/wait_ms 2737, pipelines/compile_ms 89.68, pipelines/front_ms 86.15, main/busy/ms 50.02, stage/main/update 39.36, stream/drive/ms 35.87, render/thread/ms 30.54, render/busy/ms 29.26, stage/render/prepare_assets 15.19, stage/render/render 6.08, stage/main/post_update 6.01, render/graph/ms 4.32 |
| warm-up | 4.58 | 34.5 | pipelines/wait_ms 2837, main/busy/ms 38.34, stage/main/update 29.56, stream/drive/ms 25.38, pipelines/compile_ms 25.13, pipelines/front_ms 23.72, render/thread/ms 20.63, render/busy/ms 19.23, stage/render/render 6.03, stage/main/post_update 6.03, stage/render/prepare_assets 5.43, render/graph/ms 4.16 |
| warm-up | 4.79 | 36.4 | render/thread/ms 50.72, render/busy/ms 49.16, main/busy/ms 29.60, stage/render/prepare_assets 26.22, stage/main/update 11.42, stage/main/post_update 11.07, stage/render/render 8.71, render/graph/ms 6.59, encode//sum 5.92, stream/drive/ms 5.22, stage/render/graph_submit 4.02, stage/render/specialize 3.20 |
| warm-up | 4.90 | 35.4 | render/thread/ms 32.57, render/busy/ms 31.39, main/busy/ms 30.63, stage/render/prepare_assets 20.55, stage/main/update 18.46, stream/drive/ms 13.95, stage/main/post_update 8.55, stage/render/render 6.23, render/graph/ms 4.59, encode//sum 3.65, stage/render/graph_submit 3.12, encode/submit 2.41 |
| warm-up | 5.36 | 29.2 | pipelines/wait_ms 1670, pipelines/compile_ms 173, pipelines/front_ms 162, render/thread/ms 34.83, render/busy/ms 33.71, main/busy/ms 23.23, stage/render/prepare_assets 13.81, pipelines/driver_ms 10.81, stage/main/update 8.66, stage/main/post_update 7.59, stage/render/render 5.78, stream/drive/ms 4.75 |
| warm-up | 5.73 | 35.7 | render/thread/ms 46.22, render/busy/ms 44.77, main/busy/ms 43.49, stage/render/prepare_assets 30.22, stage/main/update 26.32, stream/drive/ms 21.90, stage/main/post_update 9.91, stage/render/render 7.95, encode//sum 6.96, render/graph/ms 5.88, stage/render/graph_submit 4.15, stage/main/pre_update 3.84 |
| warm-up | 5.77 | 37.4 | pipelines/wait_ms 4877, pipelines/compile_ms 121, pipelines/front_ms 118, render/thread/ms 53.30, render/busy/ms 51.80, stage/render/prepare_assets 39.21, main/busy/ms 32.36, stage/main/update 14.37, stage/main/post_update 11.61, stream/drive/ms 9.10, stage/render/render 7.26, encode//sum 6.73 |
| warm-up | 5.89 | 28.9 | main/busy/ms 39.01, render/thread/ms 29.50, render/busy/ms 28.10, stage/main/update 26.02, stage/render/render 9.45, stage/main/post_update 9.29, encode//sum 8.32, render/graph/ms 7.53, stage/render/prepare_assets 6.11, stage/render/graph_submit 5.06, stream/drive/ms 3.55, encode/submit 3.25 |
| warm-up | 6.62 | 31.0 | render/thread/ms 35.70, render/busy/ms 34.00, main/busy/ms 33.93, stage/main/update 19.61, stage/render/prepare_assets 16.11, stage/main/post_update 10.48, stage/render/render 10.16, encode//sum 9.77, render/graph/ms 7.74, stage/render/graph_submit 4.97, encode/submit 3.23, stage/render/camera_driver 2.59 |
| warm-up | 6.67 | 29.6 | main/busy/ms 46.84, stage/main/update 31.57, render/thread/ms 25.95, render/busy/ms 24.42, stage/main/post_update 12.31, stage/render/render 9.00, encode//sum 8.40, render/graph/ms 6.89, stage/render/graph_submit 4.81, stage/render/prepare_assets 4.36, encode/submit 3.32, stage/render/prepare_write_phase_buffers 2.89 |
| warm-up | 7.17 | 31.2 | pipelines/wait_ms 4842, main/busy/ms 41.66, render/thread/ms 33.73, render/busy/ms 32.16, stage/main/update 27.30, encode//sum 13.21, stage/render/prepare_assets 11.76, pipelines/compile_ms 11.02, stage/render/render 10.87, stage/main/post_update 10.85, pipelines/front_ms 10.32, render/graph/ms 8.28 |
| warm-up | 7.20 | 32.0 | render/thread/ms 40.86, render/busy/ms 39.60, main/busy/ms 35.16, stage/main/update 21.38, stage/render/prepare_assets 19.36, encode//sum 10.70, stage/main/post_update 10.16, stage/render/render 8.58, render/graph/ms 6.79, stage/render/graph_submit 4.52, encode/submit 2.84, stage/render/prepare_write_phase_buffers 2.59 |
| measure | 2.72 | 28.8 | encode//sum 50.79, render/thread/ms 38.85, render/busy/ms 37.67, stage/render/render 20.38, render/graph/ms 18.69, main/busy/ms 18.05, stage/render/graph_submit 14.25, stage/main/post_update 10.81, stage/render/create_views 8.23, encode/submit 7.57, encode/finish_wall 6.74, stage/main/update 4.99 |
| measure | 18.19 | 29.7 | main/busy/ms 39.62, stage/main/update 24.67, render/thread/ms 22.53, render/busy/ms 21.50, encode//sum 21.22, stream/drive/ms 20.75, stage/render/render 13.46, stage/main/post_update 12.55, render/graph/ms 11.99, stage/render/graph_submit 9.86, encode/submit 4.96, encode/finish_wall 4.96 |
| measure | 59.03 | 32.3 | render/thread/ms 42.22, render/busy/ms 40.91, main/busy/ms 28.92, encode//sum 20.79, stage/render/prepare_assets 17.23, stage/main/post_update 14.58, stage/render/render 13.12, render/graph/ms 11.55, stage/render/graph_submit 9.11, stage/main/update 8.43, encode/finish_wall 5.31, bigspace/total/ms 4.20 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\scene_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **81%** (min 66%) · 2580 MHz (min 2580) · memory 8001 MHz · 79.1 W · 73 °C (max 79) · VRAM 6049 MiB · throttled 100% of samples
