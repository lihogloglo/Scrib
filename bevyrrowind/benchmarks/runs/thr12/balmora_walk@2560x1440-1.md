# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4970 frames over 65.6 s · **median 12.59 ms (79 fps)** · mean 13.21 · p95 17.39 · p99 21.01 (1% low 48 fps) · worst 36.12 ms · 130 spikes over 1.5× median · 39.3 stutters/min (work 43.0/min)
- warm-up: 338 frames over 8.8 s, worst 2537.8 ms
- hitches (> 31.5 ms): **4** measured, 19 in warm-up
- timed GPU passes: 7.80 ms of a 13.21 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.59 ms, limit 11 ms
- **FAIL** frame_ms: 43 stutters (> 1.5x moving median of 60) in 1.09 min = 39.30/min, limit 1/min
- **FAIL** worst frame 36.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.232 | 2.296 | 2.743 | 0.97 | 0.188 |
| main_opaque_pass_3d | 1.771 | 1.821 | 2.553 | 0.97 | 0.066 |
| early prepass | 0.552 | 0.567 | 0.944 | 0.97 | 0.087 |
| bloom | 0.342 | 0.351 | 0.593 | 0.97 | 0.072 |
| atmosphere_luts | 0.330 | 0.339 | 0.591 | 0.97 | 0.004 |
| water_velocity | 0.310 | 0.319 | 0.598 | 0.97 | 0.002 |
| water_sspr | 0.227 | 0.234 | 0.461 | 0.97 | 0.003 |
| ssao | 0.205 | 0.210 | 0.431 | 0.97 | 0.009 |
| uniform_allocation | 0.184 | 0.189 | 0.193 | 0.97 | 0.036 |
| atmos_froxel_inject | 0.160 | 0.165 | 0.381 | 0.97 | 0.002 |
| bin_unpacking | 0.138 | 0.141 | 0.150 | 0.97 | 0.026 |
| clouds_trace | 0.137 | 0.141 | 0.229 | 0.97 | 0.001 |
| water_fft | 0.137 | 0.140 | 0.141 | 0.97 | 0.003 |
| upscaling | 0.127 | 0.130 | 0.159 | 0.97 | 0.005 |
| clouds_composite | 0.110 | 0.113 | 0.111 | 0.97 | 0.003 |
| auxiliary_views | 0.107 | 0.110 | 0.483 | 0.97 | 0.115 |
| main_transmissive_pass_3d | 0.095 | 0.098 | 0.185 | 0.97 | 0.010 |
| tonemapping | 0.093 | 0.096 | 0.094 | 0.97 | 0.004 |
| clustering | 0.072 | 0.074 | 0.072 | 0.97 | 0.022 |
| surface_trail | 0.055 | 0.069 | 0.062 | 0.80 | 0.003 |
| render_sky | 0.053 | 0.055 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.460 | 0.708 | 0.11 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.049 | 0.049 | 0.97 | 0.006 |
| water_snapshot | 0.037 | 0.038 | 0.040 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.035 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.97 | 0.001 |
| water_mips | 0.025 | 0.026 | 0.024 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.024 | 0.225 | 0.233 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.97 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.030 | 0.97 | 0.118 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.97 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.008 | 0.008 | 0.008 | 0.97 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.005 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 20.579 | 0.000 | 10875.51 | 102275.2 |
| encode//sum | 13.130 | 19.296 | 75.27 | 65256.5 |
| render/thread/ms | 11.745 | 15.595 | 34.77 | 58370.8 |
| render/busy/ms | 11.117 | 14.903 | 33.96 | 55251.1 |
| main/busy/ms | 9.580 | 12.961 | 28.04 | 47610.2 |
| render/graph/ms | 6.916 | 9.365 | 21.18 | 34370.8 |
| stage/main/post_update | 5.840 | 8.439 | 21.69 | 29024.8 |
| stage/render/graph_submit | 5.656 | 7.715 | 20.05 | 28109.5 |
| stage/render/render | 4.789 | 9.645 | 21.92 | 23799.9 |
| encode/finish_wall | 3.360 | 4.814 | 17.37 | 16697.1 |
| stage/render/cleanup | 2.974 | 9.508 | 17.96 | 14781.2 |
| encode/submit | 2.368 | 3.340 | 6.51 | 11768.0 |
| stage/main/update | 2.343 | 3.112 | 21.05 | 11645.0 |
| stage/render/camera_driver | 1.193 | 1.697 | 4.77 | 5929.9 |
| stage/render/prepare_write_phase_buffers | 0.983 | 1.395 | 5.21 | 4887.5 |
| bigspace/total/ms | 0.630 | 1.582 | 4.32 | 3131.1 |
| stage/render/prepare_assets | 0.620 | 0.843 | 15.86 | 3079.1 |
| render/present/ms | 0.524 | 0.754 | 3.39 | 2603.2 |
| stage/render/prepare_resources | 0.478 | 0.633 | 3.13 | 2374.7 |
| stage/main/first | 0.433 | 0.613 | 4.31 | 2153.1 |
| pipelines/compile_ms | 0.421 | 0.000 | 181.73 | 2094.5 |
| pipelines/front_ms | 0.407 | 0.000 | 174.05 | 2022.0 |
| stage/render/prepare_meshes | 0.383 | 0.561 | 1.66 | 1903.9 |
| stage/main/pre_update | 0.366 | 0.554 | 1.50 | 1818.0 |
| terrain/update/ms | 0.323 | 0.434 | 1.97 | 1604.3 |
| controller/feed/ms | 0.299 | 0.413 | 0.81 | 1486.0 |
| stage/render/queue | 0.298 | 0.519 | 3.92 | 1480.1 |
| stage/render/specialize | 0.294 | 0.463 | 3.31 | 1460.2 |
| terrain/select/ms | 0.286 | 0.378 | 0.93 | 1419.5 |
| stage/main/fixed_main_loop | 0.278 | 0.467 | 1.44 | 1384.1 |
| stage/render/prepare_bind_groups | 0.277 | 0.504 | 2.93 | 1378.7 |
| stream/drive/ms | 0.277 | 0.432 | 18.43 | 1376.9 |
| bigspace/low_precision/ms | 0.229 | 0.585 | 3.82 | 1138.8 |
| bigspace/high_precision/ms | 0.223 | 0.635 | 3.91 | 1108.6 |
| far/select/ms | 0.218 | 0.316 | 0.77 | 1081.0 |
| stream/tick/ms | 0.210 | 0.317 | 0.85 | 1042.8 |
| stage/render/create_views | 0.200 | 0.286 | 9.20 | 995.8 |
| stage/render/prepare_batch_phases | 0.184 | 0.261 | 2.83 | 916.0 |
| anim/evaluate_cpu/ms | 0.182 | 0.251 | 0.72 | 906.4 |
| stage/render/prepare_flush | 0.182 | 0.358 | 2.04 | 905.3 |
| stage/render/extract_commands | 0.166 | 0.218 | 0.96 | 823.9 |
| stage/render/prepare_views | 0.151 | 0.218 | 0.67 | 752.8 |
| controller/move/ms | 0.140 | 0.277 | 0.75 | 696.3 |
| anim/evaluate/ms | 0.135 | 0.260 | 3.64 | 671.9 |
| stage/main/last | 0.125 | 0.227 | 1.74 | 622.1 |
| render/acquire/ms | 0.104 | 0.154 | 0.54 | 516.5 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.130 | 0.48 | 312.9 |
| physics/insert/ms | 0.062 | 0.470 | 1.39 | 308.4 |
| physics/step/ms | 0.060 | 0.100 | 0.90 | 299.7 |
| encode/resolve_diagnostics/sum | 0.040 | 0.054 | 0.87 | 197.1 |
| stage/render/phase_sort | 0.031 | 0.055 | 0.34 | 155.4 |
| weather/map/ms | 0.027 | 0.037 | 0.25 | 135.1 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.026 | 0.60 | 91.8 |
| pipelines/driver_ms | 0.015 | 0.000 | 7.68 | 72.5 |
| encode/camera_driver_span_end/sum | 0.014 | 0.023 | 0.30 | 69.2 |
| anim/apply/ms | 0.013 | 0.023 | 0.21 | 62.6 |
| weather/eval/ms | 0.012 | 0.015 | 0.34 | 58.0 |
| controller/avoid/ms | 0.011 | 0.015 | 0.21 | 56.4 |
| script/update/ms | 0.008 | 0.013 | 0.21 | 41.7 |
| stream/despawn/ms | 0.007 | 0.000 | 1.34 | 32.8 |
| script/input/ms | 0.006 | 0.008 | 0.24 | 28.4 |
| stream/spawn/ms | 0.006 | 0.000 | 1.16 | 27.8 |
| script/ui/ms | 0.005 | 0.008 | 0.26 | 27.1 |
| controller/write/ms | 0.004 | 0.005 | 0.09 | 17.9 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.95 | 17.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 14 | 0 |
| anim/a2/count | count | 1 | 15 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 51 | 19 |
| controller/movers/count | count | 19 | 21 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 590 |
| crowd/c0/meshes/count | count | 353 | 646 | 171 |
| crowd/c1/count | count | 146 | 157 | 148 |
| crowd/c1/joints/count | count | 5531 | 6050 | 5606 |
| crowd/c1/meshes/count | count | 1165 | 1324 | 1170 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4277 | 14.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4250 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.91 | 17.15 | 2.85 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.30 | 0.01 |
| encode/finish_critical | ms | 2.91 | 17.15 | 2.85 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.87 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.70 | 33.96 | 9.33 |
| gpu/timed/ms | ms | 8.71 | 14.43 | 9.29 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 552.74 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1295 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3372 | 5110 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1295 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3372 | 5110 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 219 | 28 |
| phases/shadow/bins | count | 4758 | 5733 | 2221 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 12.8 k | 5672 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 197 | 243 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1449 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 184 | 186 | 186 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 10 | 597 | 6 |
| preprocess/bind_groups_created | count | 10 | 597 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 179 | 268 | 173 |
| process/cpu_usage | % | 25.48 | 62.98 | 23.08 |
| process/mem_usage | GiB | 5.10 | 5.28 | 5.27 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.68 | 75.09 | 34.17 |
| system/mem_usage | % | 62.59 | 63.08 | 62.90 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 84 | 266 | 78 |
| views/all/bins | count | 7285 | 8579 | 5719 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.3 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 242 | 290 | 74 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 17.0 k | 22.9 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2574 | 3140 | 2221 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8119 | 9997 | 5672 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8119 | 9997 | 5672 |
| views/lamp_face/batch_sets | count | 0 | 184 | 0 |
| views/lamp_face/bins | count | 2113 | 2715 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2371 | 3006 | 0 |
| views/lamp_face/pipelines | count | 162 | 201 | 0 |
| views/lamp_face/work_items | count | 0 | 2802 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8343 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8512 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 57.37 | 502 | 285.1 k |
| anim/events/count | count | 0.11 | 3 | 537 |
| assets/Image/added | count | 0.14 | 43 | 682 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.88 | 152 | 4379 |
| assets/Mesh/removed | count | 0.98 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 39 | 563 |
| assets/StandardMaterial/modified | count | 4.36 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.59 | 2125 | 12.9 k |
| encode//count | count | 36.84 | 274 | 183.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 4970 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 4970 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 4970 |
| physics/inserts | count | 0.66 | 158 | 3296 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.79 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 24 | 378 |
| stream/upload/bytes | bytes | 6811 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1493 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.5 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2537.8 | main/busy/ms 2512, stage/main/post_update 2411, stage/main/update 95.31, surface/advance_cpu_ms 65.25, stage/main/pre_update 2.52, stage/main/last 1.21, surface/cell_upload_ms 0.71, stage/main/first 0.49, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.33, weather/map/ms 0.22, stream/tick/ms 0.11 |
| warm-up | 3.23 | 691.8 | main/busy/ms 2548, stage/main/post_update 2437, render/thread/ms 643, render/busy/ms 594, stage/render/prepare_assets 408, stage/render/prepare_resources 135, stage/main/update 99.55, surface/advance_cpu_ms 65.25, stage/render/prepare_views 47.07, render/acquire/ms 46.90, stage/render/render 37.62, render/graph/ms 34.57 |
| warm-up | 3.33 | 100.3 | pipelines/wait_ms 666, render/thread/ms 652, render/busy/ms 602, stage/render/prepare_assets 409, stage/render/prepare_resources 136, pipelines/compile_ms 125, pipelines/front_ms 118, main/busy/ms 52.24, stage/render/prepare_views 47.53, render/acquire/ms 47.21, stage/render/render 37.62, render/graph/ms 36.62 |
| warm-up | 3.60 | 97.2 | pipelines/compile_ms 264, pipelines/front_ms 250, main/busy/ms 66.71, stage/main/update 53.52, render/thread/ms 52.56, render/busy/ms 49.99, stage/render/prepare_assets 39.52, pipelines/driver_ms 14.77, stage/main/post_update 9.30, stage/render/render 8.29, render/graph/ms 5.29, stage/render/graph_submit 3.50 |
| warm-up | 3.74 | 48.4 | render/thread/ms 39.51, main/busy/ms 38.95, render/busy/ms 37.97, stage/main/update 30.10, stage/render/prepare_assets 28.79, stream/drive/ms 26.76, stage/main/post_update 6.01, render/graph/ms 3.59, stage/render/cleanup 2.99, stage/render/render 2.72, stage/render/graph_submit 2.27, encode/submit 1.89 |
| warm-up | 3.81 | 55.2 | pipelines/wait_ms 903, main/busy/ms 42.71, render/thread/ms 42.43, render/busy/ms 40.61, stage/main/update 30.88, stage/render/prepare_assets 30.75, stream/drive/ms 25.93, pipelines/compile_ms 11.76, pipelines/front_ms 10.97, stage/main/post_update 7.87, render/graph/ms 4.13, stage/render/render 3.59 |
| warm-up | 3.90 | 82.8 | pipelines/wait_ms 4743, pipelines/compile_ms 275, pipelines/front_ms 251, render/thread/ms 66.37, render/busy/ms 64.10, stage/render/prepare_assets 49.33, main/busy/ms 34.46, pipelines/driver_ms 23.77, stage/main/update 19.61, stream/drive/ms 14.47, stage/main/post_update 8.55, render/graph/ms 6.12 |
| warm-up | 4.04 | 108.5 | pipelines/wait_ms 1679, render/thread/ms 78.29, render/busy/ms 77.09, main/busy/ms 54.41, pipelines/compile_ms 53.49, pipelines/front_ms 51.45, stage/render/prepare_assets 43.48, stage/main/update 34.97, stream/drive/ms 30.70, stage/render/prepare_write_phase_buffers 24.73, stage/main/post_update 13.48, terrain/update/ms 4.61 |
| warm-up | 4.14 | 32.1 | pipelines/wait_ms 3557, pipelines/compile_ms 52.28, pipelines/front_ms 48.03, render/thread/ms 43.09, render/busy/ms 40.47, main/busy/ms 19.88, stage/render/prepare_assets 14.44, encode//sum 13.73, stage/render/render 11.64, stage/main/post_update 8.80, render/graph/ms 8.28, stage/main/update 7.75 |
| warm-up | 4.18 | 33.9 | pipelines/wait_ms 3642, encode//sum 74.51, render/thread/ms 54.46, render/busy/ms 51.92, pipelines/compile_ms 41.65, pipelines/front_ms 37.55, main/busy/ms 24.28, stage/render/render 21.53, render/graph/ms 17.74, stage/render/graph_submit 12.05, stage/render/prepare_assets 11.40, stage/main/post_update 9.71 |
| warm-up | 4.23 | 49.1 | pipelines/wait_ms 2385, encode//sum 93.20, pipelines/compile_ms 80.00, pipelines/front_ms 76.84, main/busy/ms 67.72, render/thread/ms 60.63, render/busy/ms 58.70, stage/main/update 52.64, stream/drive/ms 47.66, stage/render/render 22.30, stage/render/prepare_assets 21.04, render/graph/ms 19.32 |
| warm-up | 4.29 | 62.3 | pipelines/wait_ms 803, pipelines/compile_ms 70.94, main/busy/ms 70.42, pipelines/front_ms 68.62, render/thread/ms 62.23, render/busy/ms 60.92, stage/main/update 52.94, stream/drive/ms 47.34, stage/render/prepare_assets 36.83, encode//sum 24.56, stage/render/render 9.85, stage/main/post_update 8.59 |
| warm-up | 4.37 | 64.4 | pipelines/wait_ms 4304, pipelines/compile_ms 158, pipelines/front_ms 151, main/busy/ms 67.30, stage/main/update 51.80, stream/drive/ms 47.11, render/thread/ms 32.56, render/busy/ms 31.54, stage/render/prepare_assets 18.20, stage/main/post_update 11.01, pipelines/driver_ms 6.83, render/graph/ms 2.96 |
| warm-up | 4.43 | 47.6 | pipelines/wait_ms 2053, main/busy/ms 46.70, stage/main/update 37.29, stream/drive/ms 34.25, render/thread/ms 26.03, pipelines/compile_ms 25.17, render/busy/ms 24.58, pipelines/front_ms 23.79, stage/render/prepare_assets 10.65, stage/main/post_update 5.10, render/graph/ms 4.27, stage/render/render 3.98 |
| warm-up | 4.48 | 34.7 | pipelines/wait_ms 1047, pipelines/compile_ms 75.19, pipelines/front_ms 70.37, main/busy/ms 36.47, stage/main/update 28.01, stream/drive/ms 24.32, render/thread/ms 18.13, render/busy/ms 16.94, stage/render/prepare_assets 6.40, stage/main/post_update 5.72, pipelines/driver_ms 4.82, render/graph/ms 3.68 |
| warm-up | 4.62 | 33.1 | pipelines/wait_ms 3321, pipelines/compile_ms 155, pipelines/front_ms 148, render/thread/ms 47.45, render/busy/ms 46.19, main/busy/ms 31.93, stage/render/prepare_assets 31.31, stage/main/update 17.07, stream/drive/ms 11.73, stage/main/post_update 9.08, pipelines/driver_ms 7.01, render/graph/ms 3.98 |
| warm-up | 5.47 | 35.2 | pipelines/wait_ms 13.1 k, pipelines/compile_ms 66.63, pipelines/front_ms 62.16, main/busy/ms 44.96, render/thread/ms 37.47, render/busy/ms 35.50, stage/main/update 25.60, stage/render/render 12.08, stage/main/post_update 11.60, stage/render/prepare_assets 10.37, encode//sum 9.92, render/graph/ms 8.95 |
| warm-up | 6.22 | 38.8 | main/busy/ms 53.26, stage/main/update 40.35, render/thread/ms 31.88, render/busy/ms 27.31, stage/render/render 15.61, encode//sum 11.53, render/graph/ms 10.01, stage/main/post_update 9.91, stage/render/prepare_assets 7.22, stage/render/graph_submit 7.00, encode/submit 4.79, render/present/ms 4.21 |
| warm-up | 6.26 | 42.5 | render/thread/ms 44.52, main/busy/ms 43.52, render/busy/ms 40.02, stage/main/update 27.16, stage/render/prepare_assets 20.57, stage/render/render 13.28, stage/main/post_update 12.10, encode//sum 9.51, render/graph/ms 7.72, stage/render/graph_submit 5.43, render/present/ms 4.20, encode/submit 3.67 |
| measure | 0.79 | 34.0 | main/busy/ms 37.17, render/thread/ms 33.89, render/busy/ms 32.59, encode//sum 31.56, stage/main/update 21.97, stage/render/render 17.12, render/graph/ms 15.34, stream/drive/ms 14.73, stage/main/post_update 11.72, stage/render/graph_submit 11.30, stage/render/prepare_assets 6.57, encode/finish_wall 6.15 |
| measure | 2.73 | 36.1 | encode//sum 85.18, render/thread/ms 47.97, render/busy/ms 46.46, render/graph/ms 25.12, main/busy/ms 23.33, stage/render/graph_submit 18.36, stage/render/cleanup 17.96, stage/main/post_update 13.78, encode/submit 9.65, stage/render/create_views 9.39, stage/render/render 9.19, encode/finish_wall 8.78 |
| measure | 18.20 | 31.8 | main/busy/ms 39.52, encode//sum 32.52, render/thread/ms 28.69, render/busy/ms 27.23, stage/main/update 23.83, stream/drive/ms 18.70, render/graph/ms 16.95, stage/render/graph_submit 13.91, stage/main/post_update 12.53, stage/render/cleanup 10.27, stage/render/render 8.78, encode/finish_wall 7.70 |
| measure | 59.04 | 34.3 | render/thread/ms 40.80, render/busy/ms 39.52, main/busy/ms 25.50, encode//sum 20.88, stage/render/prepare_assets 19.06, stage/main/post_update 13.15, render/graph/ms 10.80, stage/render/graph_submit 8.55, stage/main/update 7.37, stage/render/cleanup 6.36, stage/render/render 5.96, encode/finish_wall 4.72 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\thr12\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_COMPUTE_THREADS=12
- before the run: GPU 0%, CPU 14%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **78%** (min 54%) · 2581 MHz (min 2580) · memory 8001 MHz · 73.6 W · 71 °C (max 77) · VRAM 7267 MiB · throttled 100% of samples
