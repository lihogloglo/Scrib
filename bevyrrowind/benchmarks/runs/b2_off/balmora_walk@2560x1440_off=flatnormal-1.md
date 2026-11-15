# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: flatnormal · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5138 frames over 65.6 s · **median 12.47 ms (80 fps)** · mean 12.78 · p95 15.72 · p99 18.41 (1% low 54 fps) · worst 32.81 ms · 48 spikes over 1.5× median · 30.2 stutters/min (work 36.6/min)
- warm-up: 341 frames over 9.0 s, worst 2700.9 ms
- hitches (> 31.2 ms): **3** measured, 21 in warm-up
- timed GPU passes: 7.84 ms of a 12.78 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.47 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- pass worst frame 32.81 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.203 | 2.278 | 2.756 | 0.97 | 0.187 |
| main_opaque_pass_3d | 1.769 | 1.829 | 2.625 | 0.97 | 0.062 |
| early prepass | 0.545 | 0.563 | 0.996 | 0.97 | 0.089 |
| bloom | 0.339 | 0.350 | 0.593 | 0.97 | 0.070 |
| atmosphere_luts | 0.327 | 0.338 | 0.574 | 0.97 | 0.004 |
| water_velocity | 0.308 | 0.318 | 0.606 | 0.97 | 0.002 |
| water_sspr | 0.226 | 0.233 | 0.457 | 0.97 | 0.002 |
| ssao | 0.205 | 0.212 | 0.436 | 0.97 | 0.008 |
| uniform_allocation | 0.192 | 0.199 | 0.202 | 0.97 | 0.036 |
| auxiliary_views | 0.172 | 0.178 | 0.710 | 0.97 | 0.161 |
| atmos_froxel_inject | 0.165 | 0.170 | 0.397 | 0.97 | 0.002 |
| bin_unpacking | 0.154 | 0.159 | 0.360 | 0.97 | 0.026 |
| water_fft | 0.140 | 0.145 | 0.143 | 0.97 | 0.003 |
| clouds_trace | 0.137 | 0.141 | 0.232 | 0.97 | 0.001 |
| upscaling | 0.129 | 0.133 | 0.169 | 0.97 | 0.005 |
| clouds_composite | 0.110 | 0.114 | 0.112 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.093 | 0.096 | 0.182 | 0.97 | 0.009 |
| tonemapping | 0.093 | 0.096 | 0.094 | 0.97 | 0.004 |
| clustering | 0.068 | 0.070 | 0.070 | 0.97 | 0.021 |
| render_sky | 0.053 | 0.055 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.462 | 0.696 | 0.11 | 0.001 |
| surface_trail | 0.048 | 0.062 | 0.059 | 0.78 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.045 | 0.047 | 0.049 | 0.97 | 0.006 |
| water_snapshot | 0.039 | 0.040 | 0.045 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.233 | 0.240 | 0.11 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.97 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.97 | 0.119 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 19.419 | 0.000 | 6075.32 | 99775.0 |
| encode//sum | 11.729 | 16.698 | 64.31 | 60265.5 |
| render/thread/ms | 11.301 | 14.106 | 31.53 | 58064.7 |
| render/busy/ms | 10.677 | 13.420 | 30.85 | 54857.3 |
| main/busy/ms | 9.914 | 12.298 | 29.49 | 50938.2 |
| render/graph/ms | 6.447 | 8.250 | 19.42 | 33123.6 |
| stage/main/post_update | 6.203 | 8.242 | 16.14 | 31870.6 |
| stage/render/graph_submit | 5.168 | 6.663 | 17.39 | 26554.0 |
| stage/render/render | 4.463 | 8.675 | 20.24 | 22928.4 |
| stage/render/cleanup | 2.838 | 8.523 | 19.48 | 14583.6 |
| encode/finish_wall | 2.771 | 3.735 | 7.89 | 14235.8 |
| encode/submit | 2.474 | 3.441 | 14.57 | 12709.1 |
| stage/main/update | 2.333 | 2.928 | 22.98 | 11985.3 |
| stage/render/camera_driver | 1.217 | 1.648 | 4.03 | 6251.6 |
| stage/render/prepare_write_phase_buffers | 1.012 | 1.364 | 3.03 | 5199.6 |
| pipelines/compile_ms | 0.650 | 0.000 | 1363.90 | 3342.1 |
| stage/render/prepare_assets | 0.595 | 0.800 | 12.06 | 3055.6 |
| bigspace/total/ms | 0.592 | 1.742 | 3.75 | 3041.1 |
| render/present/ms | 0.520 | 0.717 | 2.46 | 2671.2 |
| stage/render/prepare_resources | 0.476 | 0.626 | 1.29 | 2444.8 |
| stage/main/first | 0.431 | 0.580 | 5.27 | 2214.5 |
| stage/render/prepare_meshes | 0.415 | 0.571 | 2.15 | 2130.8 |
| pipelines/front_ms | 0.387 | 0.000 | 155.12 | 1988.8 |
| stage/main/pre_update | 0.353 | 0.530 | 2.43 | 1813.8 |
| terrain/update/ms | 0.315 | 0.407 | 2.03 | 1616.0 |
| controller/feed/ms | 0.301 | 0.414 | 0.78 | 1544.4 |
| stage/render/specialize | 0.295 | 0.443 | 2.46 | 1515.8 |
| stage/render/queue | 0.289 | 0.470 | 2.68 | 1487.1 |
| terrain/select/ms | 0.279 | 0.361 | 1.02 | 1435.8 |
| stage/render/prepare_bind_groups | 0.276 | 0.502 | 2.76 | 1420.5 |
| stage/main/fixed_main_loop | 0.273 | 0.460 | 3.14 | 1402.2 |
| stream/drive/ms | 0.268 | 0.402 | 20.66 | 1378.8 |
| pipelines/driver_ms | 0.263 | 0.000 | 1246.28 | 1353.3 |
| bigspace/low_precision/ms | 0.250 | 0.674 | 3.21 | 1285.8 |
| far/select/ms | 0.208 | 0.304 | 0.75 | 1068.2 |
| stream/tick/ms | 0.204 | 0.297 | 0.71 | 1048.2 |
| stage/render/create_views | 0.200 | 0.273 | 8.43 | 1026.2 |
| stage/render/prepare_batch_phases | 0.191 | 0.282 | 3.10 | 982.0 |
| anim/evaluate_cpu/ms | 0.170 | 0.224 | 0.68 | 875.9 |
| stage/render/prepare_flush | 0.170 | 0.351 | 2.22 | 871.5 |
| bigspace/high_precision/ms | 0.169 | 0.588 | 3.39 | 868.9 |
| stage/render/extract_commands | 0.164 | 0.212 | 0.51 | 842.6 |
| controller/move/ms | 0.156 | 0.318 | 0.66 | 803.0 |
| stage/render/prepare_views | 0.151 | 0.212 | 6.30 | 773.9 |
| anim/evaluate/ms | 0.148 | 0.273 | 2.67 | 759.2 |
| stage/main/last | 0.126 | 0.224 | 1.72 | 649.2 |
| render/acquire/ms | 0.104 | 0.149 | 6.22 | 536.2 |
| stage/render/prepare_collect_phase_buffers | 0.070 | 0.143 | 0.40 | 358.7 |
| physics/step/ms | 0.061 | 0.103 | 1.08 | 312.2 |
| physics/insert/ms | 0.059 | 0.463 | 1.15 | 303.8 |
| stage/render/phase_sort | 0.037 | 0.065 | 0.44 | 187.7 |
| encode/resolve_diagnostics/sum | 0.035 | 0.048 | 0.60 | 178.0 |
| weather/map/ms | 0.026 | 0.035 | 0.43 | 133.4 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.025 | 0.42 | 83.5 |
| anim/apply/ms | 0.013 | 0.023 | 0.22 | 65.9 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.33 | 59.3 |
| controller/avoid/ms | 0.011 | 0.015 | 0.23 | 57.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.15 | 55.7 |
| script/update/ms | 0.008 | 0.012 | 0.10 | 41.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.14 | 31.6 |
| script/ui/ms | 0.006 | 0.008 | 0.12 | 29.8 |
| script/input/ms | 0.005 | 0.007 | 0.13 | 28.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.05 | 27.9 |
| controller/write/ms | 0.004 | 0.005 | 0.24 | 19.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.15 | 3.1 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1301 | 2262 | 480 |
| crowd/c0/meshes/count | count | 353 | 646 | 117 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5937 | 5684 |
| crowd/c1/meshes/count | count | 1180 | 1281 | 1216 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4266 | 10.0 k | 6135 |
| crowd/joints_uploaded/count | count | 4244 | 6643 | 6135 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.26 | 6.58 | 2.73 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.42 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.33 | 0.01 |
| encode/finish_critical | ms | 2.26 | 6.58 | 2.73 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.60 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.63 | 30.85 | 10.36 |
| gpu/timed/ms | ms | 8.77 | 11.48 | 9.06 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
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
| phases/alpha_mask3d/items | count | 828 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 828 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3457 | 5116 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3457 | 5116 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 19 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 227 | 28 |
| phases/shadow/bins | count | 4841 | 5821 | 2208 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.7 k | 13.2 k | 5659 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 252 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2349 |
| physics/items | count | 200 | 476 | 19 |
| physics/pending | count | 0 | 651 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 188 | 190 | 190 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 11 | 625 | 6 |
| preprocess/bind_groups_created | count | 11 | 625 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 192 | 289 | 185 |
| process/cpu_usage | % | 26.34 | 43.75 | 26.01 |
| process/mem_usage | GiB | 5.20 | 5.45 | 5.45 |
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
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.35 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.93 | 69.19 | 42.91 |
| system/mem_usage | % | 68.73 | 69.59 | 69.45 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2934 | 2934 |
| textures/images | count | 2589 | 2644 | 2644 |
| views/all/batch_sets | count | 92 | 280 | 84 |
| views/all/bins | count | 7473 | 8695 | 5706 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.7 k | 25.7 k | 13.3 k |
| views/all/pipelines | count | 254 | 303 | 80 |
| views/all/pipelines_distinct | count | 75 | 79 | 67 |
| views/all/work_items | count | 17.4 k | 23.1 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2606 | 3187 | 2208 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8185 | 10.2 k | 5659 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8185 | 10.2 k | 5659 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2160 | 2901 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2406 | 3208 | 0 |
| views/lamp_face/pipelines | count | 168 | 211 | 0 |
| views/lamp_face/work_items | count | 0 | 2906 | 0 |
| views/main/batch_sets | count | 55 | 58 | 56 |
| views/main/bins | count | 2882 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8574 | 13.0 k | 7610 |
| views/main/pipelines | count | 51 | 54 | 52 |
| views/main/work_items | count | 8729 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 67.73 | 280 | 348.0 k |
| anim/events/count | count | 0.10 | 3 | 537 |
| assets/Image/added | count | 0.12 | 43 | 604 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.80 | 152 | 4105 |
| assets/Mesh/removed | count | 0.95 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 476 |
| assets/StandardMaterial/modified | count | 4.22 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.15 | 1245 | 5906 |
| encode//count | count | 39.89 | 274 | 205.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5138 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5138 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5138 |
| physics/inserts | count | 0.56 | 130 | 2902 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 113 | 2335 |
| physics/steps | count | 0.77 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 8 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6589 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1445 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.2 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.70 | 2700.9 | main/busy/ms 2672, stage/main/post_update 2568, stage/main/update 97.88, surface/advance_cpu_ms 68.16, stage/main/pre_update 2.80, stage/main/last 1.27, surface/cell_upload_ms 0.81, stage/main/first 0.59, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.28, weather/map/ms 0.11, bigspace/total/ms 0.11 |
| warm-up | 3.50 | 797.9 | main/busy/ms 2715, stage/main/post_update 2598, render/thread/ms 752, render/busy/ms 640, stage/render/prepare_assets 495, stage/render/prepare_resources 125, stage/render/prepare_views 109, render/acquire/ms 109, stage/main/update 104, surface/advance_cpu_ms 68.16, stage/render/create_views 11.29, stage/render/render 8.09 |
| warm-up | 3.64 | 140.4 | render/thread/ms 762, render/busy/ms 649, pipelines/wait_ms 530, stage/render/prepare_assets 496, pipelines/compile_ms 141, pipelines/front_ms 134, stage/render/prepare_resources 127, stage/render/prepare_views 109, render/acquire/ms 109, main/busy/ms 62.67, stage/main/post_update 40.29, terrain/update/ms 23.68 |
| warm-up | 3.86 | 88.8 | main/busy/ms 62.92, stage/main/update 49.89, render/thread/ms 48.87, render/busy/ms 46.51, stage/render/prepare_assets 35.84, stage/main/post_update 9.61, stage/render/render 7.94, render/graph/ms 5.22, stage/render/graph_submit 3.30, encode/submit 2.73, render/present/ms 2.06, encode//sum 1.80 |
| warm-up | 4.05 | 66.5 | render/thread/ms 49.72, main/busy/ms 48.95, render/busy/ms 47.67, stage/main/update 40.43, stage/render/prepare_assets 39.20, stream/drive/ms 36.70, stage/main/post_update 6.02, stage/render/render 5.24, render/graph/ms 2.79, render/present/ms 1.83, stage/render/graph_submit 1.41, encode//sum 1.30 |
| warm-up | 4.17 | 82.8 | pipelines/wait_ms 5127, pipelines/compile_ms 254, pipelines/front_ms 230, render/thread/ms 63.29, render/busy/ms 61.09, stage/render/prepare_assets 45.29, main/busy/ms 40.23, stage/main/update 26.22, pipelines/driver_ms 24.50, stream/drive/ms 22.32, stage/render/render 11.27, stage/main/post_update 10.17 |
| warm-up | 4.20 | 33.1 | pipelines/wait_ms 4922, pipelines/compile_ms 112, pipelines/front_ms 106, render/thread/ms 78.20, render/busy/ms 75.88, stage/render/prepare_assets 58.14, main/busy/ms 54.94, stage/main/update 34.28, stream/drive/ms 29.65, stage/render/render 12.18, stage/main/post_update 11.56, render/graph/ms 9.15 |
| warm-up | 4.33 | 125.1 | pipelines/wait_ms 2525, render/thread/ms 97.78, render/busy/ms 96.08, stage/render/prepare_assets 84.90, pipelines/compile_ms 69.01, pipelines/front_ms 66.83, main/busy/ms 58.67, stage/main/update 34.78, stream/drive/ms 29.32, stage/main/post_update 13.52, stage/render/render 6.11, render/graph/ms 3.95 |
| warm-up | 4.45 | 40.4 | pipelines/wait_ms 4293, encode//sum 152, pipelines/compile_ms 77.01, render/thread/ms 60.73, render/busy/ms 57.97, pipelines/front_ms 57.90, stage/render/render 31.76, render/graph/ms 27.90, main/busy/ms 23.08, stage/render/graph_submit 22.42, pipelines/driver_ms 19.10, encode/finish_wall 16.04 |
| warm-up | 4.52 | 41.7 | main/busy/ms 70.14, stage/main/update 55.24, render/thread/ms 51.51, render/busy/ms 49.99, stream/drive/ms 49.93, stage/render/prepare_assets 24.08, stage/render/render 11.64, render/graph/ms 9.38, stage/main/post_update 9.10, encode//sum 8.56, stage/render/graph_submit 5.32, stage/render/prepare_bind_groups 4.36 |
| warm-up | 4.59 | 60.9 | pipelines/wait_ms 1488, pipelines/compile_ms 145, pipelines/front_ms 136, main/busy/ms 68.11, stage/main/update 54.37, stream/drive/ms 50.37, render/thread/ms 44.92, render/busy/ms 43.55, stage/render/prepare_assets 33.23, pipelines/driver_ms 9.05, stage/main/post_update 8.42, stage/render/render 4.64 |
| warm-up | 4.68 | 85.9 | pipelines/wait_ms 9124, pipelines/compile_ms 82.94, pipelines/front_ms 77.49, main/busy/ms 75.35, stage/main/update 54.55, stream/drive/ms 49.80, render/thread/ms 36.46, render/busy/ms 35.33, stage/render/prepare_assets 18.16, stage/main/post_update 14.53, stage/render/render 6.45, pipelines/driver_ms 5.45 |
| warm-up | 4.77 | 63.8 | main/busy/ms 61.99, stage/main/update 44.28, stream/drive/ms 38.64, render/thread/ms 33.59, render/busy/ms 32.26, stage/render/prepare_assets 16.30, stage/main/post_update 13.20, terrain/update/ms 9.07, stage/render/render 6.90, render/graph/ms 4.98, encode//sum 4.77, stage/render/graph_submit 2.53 |
| warm-up | 4.81 | 35.3 | pipelines/wait_ms 5729, pipelines/compile_ms 150, pipelines/front_ms 145, main/busy/ms 40.47, stage/main/update 30.67, stream/drive/ms 26.28, render/thread/ms 22.70, render/busy/ms 21.52, stage/render/prepare_assets 7.64, stage/render/render 6.28, stage/main/post_update 6.25, pipelines/driver_ms 5.11 |
| warm-up | 5.56 | 41.5 | pipelines/wait_ms 6700, pipelines/compile_ms 123, pipelines/front_ms 112, render/thread/ms 57.16, render/busy/ms 55.33, main/busy/ms 36.64, stage/render/prepare_assets 25.34, stage/main/update 16.13, stage/main/post_update 12.56, pipelines/driver_ms 10.88, stage/render/render 8.70, stage/render/specialize 7.97 |
| warm-up | 5.60 | 49.1 | pipelines/wait_ms 24.3 k, pipelines/compile_ms 178, pipelines/front_ms 164, render/thread/ms 69.04, render/busy/ms 67.49, main/busy/ms 52.70, stage/render/prepare_assets 43.22, stage/main/update 34.35, pipelines/driver_ms 14.74, stage/main/post_update 11.09, encode//sum 6.26, render/graph/ms 5.63 |
| warm-up | 5.67 | 64.3 | pipelines/wait_ms 17.6 k, render/thread/ms 87.98, render/busy/ms 86.24, main/busy/ms 81.27, stage/main/update 65.27, stage/render/prepare_assets 62.44, pipelines/compile_ms 55.21, pipelines/front_ms 51.36, stage/main/post_update 9.70, render/graph/ms 6.20, encode//sum 6.18, stream/drive/ms 4.57 |
| warm-up | 5.73 | 57.2 | pipelines/wait_ms 3953, render/thread/ms 96.44, render/busy/ms 94.71, stage/render/prepare_assets 71.51, main/busy/ms 71.46, pipelines/compile_ms 70.45, pipelines/front_ms 68.56, stage/main/update 52.96, stage/main/post_update 12.31, stage/render/render 8.59, render/graph/ms 6.25, encode//sum 6.24 |
| warm-up | 6.03 | 31.5 | pipelines/wait_ms 2279, pipelines/compile_ms 62.93, pipelines/front_ms 59.36, main/busy/ms 38.32, render/thread/ms 29.73, render/busy/ms 28.22, stage/main/update 22.31, stream/drive/ms 15.53, stage/main/post_update 10.19, stage/render/prepare_assets 8.72, encode//sum 8.59, render/graph/ms 7.66 |
| warm-up | 6.82 | 37.3 | pipelines/wait_ms 4106, pipelines/compile_ms 219, pipelines/front_ms 211, render/thread/ms 48.19, stage/render/prepare_views 30.48, render/acquire/ms 30.33, main/busy/ms 17.19, render/busy/ms 16.65, encode//sum 12.76, stage/render/render 10.64, stage/main/post_update 10.26, render/graph/ms 8.63 |
| warm-up | 8.08 | 35.2 | main/busy/ms 45.20, render/thread/ms 36.67, render/busy/ms 35.19, stage/main/update 31.44, encode//sum 22.34, stage/render/render 16.83, render/graph/ms 14.80, stage/render/graph_submit 11.88, stage/main/post_update 10.43, stage/render/prepare_assets 10.26, encode/finish_wall 6.24, encode/submit 5.71 |
| measure | 2.73 | 32.8 | encode//sum 52.77, render/thread/ms 41.96, render/busy/ms 40.76, stage/render/render 21.36, render/graph/ms 19.59, main/busy/ms 19.09, stage/render/graph_submit 14.36, stage/main/post_update 11.24, stage/render/create_views 8.58, encode/submit 7.36, encode/finish_wall 7.07, stage/main/update 5.26 |
| measure | 24.10 | 32.3 | render/thread/ms 43.20, render/busy/ms 41.38, encode//sum 41.25, main/busy/ms 41.19, stage/main/post_update 18.20, render/graph/ms 17.43, stage/main/update 16.62, stage/render/graph_submit 13.89, stage/render/render 11.72, stage/render/cleanup 8.58, stage/render/prepare_assets 8.19, encode/finish_wall 7.74 |
| measure | 59.07 | 32.7 | render/thread/ms 42.14, render/busy/ms 40.77, main/busy/ms 27.66, stage/render/prepare_assets 20.92, encode//sum 19.14, stage/main/post_update 14.42, render/graph/ms 10.70, stage/render/graph_submit 8.49, stage/main/update 8.32, stage/render/cleanup 6.24, stage/render/render 6.04, encode/finish_wall 4.68 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\b2_off\balmora_walk@2560x1440_off=flatnormal-1 --bench-baseline none --off flatnormal
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 16%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (250 samples): GPU load **81%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.2 W · 74 °C (max 79) · VRAM 7076 MiB · throttled 100% of samples
