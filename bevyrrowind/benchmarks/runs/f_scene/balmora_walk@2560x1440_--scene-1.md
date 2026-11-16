# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5846 frames over 65.7 s · **median 11.01 ms (91 fps)** · mean 11.23 · p95 13.34 · p99 15.09 (1% low 66 fps) · worst 34.01 ms · 29 spikes over 1.5× median · 21.9 stutters/min (work 22.8/min)
- warm-up: 469 frames over 10.7 s, worst 2514.3 ms
- hitches (> 27.5 ms): **2** measured, 40 in warm-up
- timed GPU passes: 7.24 ms of a 11.23 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.01 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- **FAIL** worst frame 34.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.992 | 2.073 | 2.109 | 0.96 | 0.170 |
| main_opaque_pass_3d | 1.548 | 1.611 | 1.965 | 0.96 | 0.065 |
| early prepass | 0.556 | 0.579 | 0.820 | 0.96 | 0.081 |
| bloom | 0.308 | 0.320 | 0.326 | 0.96 | 0.062 |
| atmosphere_luts | 0.301 | 0.313 | 0.372 | 0.96 | 0.003 |
| water_velocity | 0.270 | 0.281 | 0.353 | 0.96 | 0.001 |
| water_sspr | 0.205 | 0.214 | 0.213 | 0.96 | 0.002 |
| ssao | 0.185 | 0.192 | 0.195 | 0.96 | 0.007 |
| atmos_froxel_inject | 0.141 | 0.147 | 0.148 | 0.96 | 0.001 |
| clouds_trace | 0.136 | 0.142 | 0.237 | 0.96 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.133 | 0.573 | 0.646 | 0.23 | 0.044 |
| water_fft | 0.127 | 0.132 | 0.138 | 0.96 | 0.003 |
| gpu_instance_materialization | 0.117 | 0.121 | 0.431 | 0.96 | 0.004 |
| uniform_allocation | 0.116 | 0.121 | 0.148 | 0.96 | 0.022 |
| upscaling | 0.113 | 0.118 | 0.126 | 0.96 | 0.004 |
| auxiliary_views | 0.104 | 0.108 | 0.444 | 0.96 | 0.102 |
| clouds_composite | 0.104 | 0.108 | 0.110 | 0.96 | 0.002 |
| bin_unpacking | 0.095 | 0.099 | 0.117 | 0.96 | 0.017 |
| tonemapping | 0.083 | 0.086 | 0.090 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.080 | 0.084 | 0.160 | 0.96 | 0.008 |
| clustering | 0.064 | 0.067 | 0.068 | 0.96 | 0.018 |
| early_mesh_preprocessing | 0.064 | 0.066 | 0.087 | 0.96 | 0.003 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.96 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.424 | 0.422 | 0.10 | 0.001 |
| surface_trail | 0.040 | 0.058 | 0.057 | 0.70 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.96 | 0.001 |
| range_unpacking | 0.026 | 0.027 | 0.029 | 0.96 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.223 | 0.233 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.96 | 0.005 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| main_transparent_pass_3d | 0.009 | 0.009 | 0.022 | 0.96 | 0.077 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_copy | 0.002 | 0.019 | 0.015 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.017 | 0.022 | 0.01 | 0.010 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.015 | 0.018 | 0.00 | 0.011 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.194 | 14.863 | 46.13 | 59595.7 |
| render/thread/ms | 10.051 | 12.020 | 23.33 | 58755.3 |
| render/busy/ms | 9.468 | 11.451 | 22.47 | 55352.6 |
| main/busy/ms | 8.770 | 10.905 | 30.94 | 51268.7 |
| stage/render/render | 6.432 | 7.743 | 15.70 | 37602.1 |
| render/graph/ms | 5.677 | 6.965 | 14.51 | 33186.0 |
| stage/main/post_update | 5.201 | 7.028 | 9.97 | 30402.6 |
| stage/render/graph_submit | 4.597 | 5.692 | 10.85 | 26874.1 |
| encode/finish_wall | 2.473 | 3.185 | 5.01 | 14458.0 |
| stage/main/update | 2.198 | 2.668 | 24.71 | 12849.3 |
| encode/submit | 2.176 | 2.904 | 6.51 | 12723.2 |
| stage/render/camera_driver | 1.020 | 1.326 | 3.59 | 5962.5 |
| bigspace/total/ms | 0.638 | 2.203 | 3.58 | 3732.6 |
| stage/render/prepare_write_phase_buffers | 0.624 | 0.882 | 1.95 | 3649.6 |
| stage/render/prepare_assets | 0.539 | 0.690 | 11.27 | 3151.4 |
| render/present/ms | 0.494 | 0.639 | 4.48 | 2886.4 |
| stage/render/prepare_resources | 0.427 | 0.514 | 0.89 | 2494.8 |
| stage/main/first | 0.400 | 0.521 | 4.08 | 2338.0 |
| stage/render/prepare_meshes | 0.388 | 0.502 | 2.68 | 2266.8 |
| stage/main/pre_update | 0.325 | 0.479 | 2.11 | 1901.4 |
| terrain/update/ms | 0.300 | 0.388 | 1.79 | 1753.3 |
| stage/render/prepare_bind_groups | 0.299 | 0.472 | 2.33 | 1746.7 |
| bigspace/low_precision/ms | 0.284 | 1.118 | 3.33 | 1660.5 |
| stage/render/prepare_flush | 0.279 | 0.481 | 1.64 | 1631.4 |
| controller/feed/ms | 0.271 | 0.350 | 0.69 | 1583.4 |
| terrain/select/ms | 0.266 | 0.344 | 0.78 | 1556.9 |
| stage/main/last | 0.252 | 0.306 | 0.86 | 1473.9 |
| stream/drive/ms | 0.249 | 0.339 | 12.15 | 1456.3 |
| stage/render/specialize | 0.246 | 0.354 | 2.37 | 1440.8 |
| stage/main/fixed_main_loop | 0.218 | 0.361 | 1.04 | 1273.6 |
| stage/render/queue | 0.212 | 0.327 | 2.04 | 1238.2 |
| bigspace/high_precision/ms | 0.211 | 0.907 | 3.24 | 1232.9 |
| anim/evaluate/ms | 0.205 | 0.357 | 3.09 | 1196.0 |
| stage/render/prepare_batch_phases | 0.203 | 0.291 | 0.57 | 1185.2 |
| stream/tick/ms | 0.194 | 0.268 | 0.83 | 1134.3 |
| far/select/ms | 0.186 | 0.252 | 0.78 | 1088.7 |
| stage/render/extract_commands | 0.173 | 0.204 | 0.51 | 1010.5 |
| stage/render/create_views | 0.168 | 0.219 | 0.96 | 981.2 |
| anim/evaluate_cpu/ms | 0.141 | 0.182 | 0.62 | 822.0 |
| controller/move/ms | 0.130 | 0.253 | 0.48 | 759.7 |
| stage/render/prepare_views | 0.129 | 0.164 | 0.41 | 756.1 |
| render/acquire/ms | 0.088 | 0.115 | 0.37 | 516.3 |
| pipelines/compile_ms | 0.063 | 0.000 | 125.73 | 368.7 |
| pipelines/front_ms | 0.061 | 0.000 | 121.28 | 353.8 |
| physics/step/ms | 0.048 | 0.085 | 0.51 | 283.4 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.26 | 273.1 |
| physics/insert/ms | 0.031 | 0.350 | 0.92 | 182.8 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.37 | 181.5 |
| stage/render/phase_sort | 0.027 | 0.048 | 0.32 | 160.2 |
| weather/map/ms | 0.026 | 0.034 | 0.22 | 154.6 |
| stage/render/cleanup | 0.024 | 0.035 | 0.80 | 140.1 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.44 | 75.6 |
| anim/apply/ms | 0.011 | 0.021 | 0.14 | 64.3 |
| weather/eval/ms | 0.011 | 0.014 | 0.12 | 61.6 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.28 | 59.8 |
| controller/avoid/ms | 0.010 | 0.013 | 0.08 | 58.8 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 40.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 29.4 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 28.3 |
| script/ui/ms | 0.005 | 0.006 | 0.22 | 27.6 |
| stream/upload/ms | 0.004 | 0.000 | 1.80 | 23.2 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 19.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.05 | 17.1 |
| pipelines/driver_ms | 0.003 | 0.000 | 4.45 | 14.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.1 |
| stage/render/prepare | 0.000 | 0.002 | 0.04 | 2.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.13 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.11 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 28 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1269 | 2262 | 583 |
| crowd/c0/meshes/count | count | 349 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5548 | 5991 | 5613 |
| crowd/c1/meshes/count | count | 1171 | 1295 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4620 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4601 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.8 k | 46.9 k |
| encode//max | ms | 2.10 | 3.75 | 2.29 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.44 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 2.10 | 3.75 | 2.29 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.37 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.51 | 30.94 | 9.41 |
| gpu/timed/ms | ms | 8.68 | 10.81 | 9.41 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 333 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 333 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 759 | 969 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2515 | 3529 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 759 | 969 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2515 | 3529 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 176 | 8 |
| phases/shadow/bins | count | 1552 | 2162 | 896 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 4976 | 6458 | 3790 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 149 | 205 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2670 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 500 | 11 |
| preprocess/bind_groups_created | count | 12 | 494 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 125 | 201 | 119 |
| process/cpu_usage | % | 25.52 | 43.41 | 25 |
| process/mem_usage | GiB | 5.42 | 5.59 | 5.59 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 6449 | 6573 | 6573 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.07 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.16 | 48.16 | 28.06 |
| system/mem_usage | % | 51.32 | 51.88 | 51.80 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2881 | 2996 | 2996 |
| textures/images | count | 2665 | 2723 | 2723 |
| views/all/batch_sets | count | 74 | 231 | 64 |
| views/all/bins | count | 3525 | 4189 | 3178 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 10.7 k | 13.3 k | 9730 |
| views/all/pipelines | count | 194 | 252 | 64 |
| views/all/pipelines_distinct | count | 68 | 73 | 59 |
| views/all/work_items | count | 154.0 k | 209.1 k | 153.8 k |
| views/cascade/batch_sets | count | 13 | 39 | 8 |
| views/cascade/bins | count | 1142 | 1551 | 896 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 4270 | 5540 | 3790 |
| views/cascade/pipelines | count | 25 | 34 | 20 |
| views/cascade/work_items | count | 49.5 k | 103.8 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 144 | 0 |
| views/lamp_face/bins | count | 399 | 667 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 641 | 1006 | 0 |
| views/lamp_face/pipelines | count | 124 | 177 | 0 |
| views/lamp_face/work_items | count | 0 | 742 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2018 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5951 | 7480 | 5940 |
| views/main/pipelines | count | 45 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 105.9 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.6 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 50.76 | 252 | 296.7 k |
| anim/events/count | count | 0.10 | 3 | 580 |
| assets/Image/added | count | 0.10 | 43 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.49 | 135 | 2874 |
| assets/Mesh/removed | count | 0.83 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.70 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.25 | 1455 | 1489 |
| encode//count | count | 38.48 | 273 | 224.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5846 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5846 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5846 |
| physics/inserts | count | 0.38 | 88 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 6 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5791 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1270 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.5 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2514.3 | main/busy/ms 2488, stage/main/post_update 2384, stage/main/update 97.80, surface/advance_cpu_ms 67.93, stage/main/pre_update 2.57, stage/main/last 1.18, surface/cell_upload_ms 0.70, stage/main/first 0.55, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.32, bigspace/total/ms 0.12, bigspace/high_precision/ms 0.09 |
| warm-up | 3.20 | 689.5 | main/busy/ms 2624, stage/main/post_update 2407, render/thread/ms 660, render/busy/ms 500, stage/render/prepare_assets 478, stage/main/update 205, stage/render/prepare_views 158, render/acquire/ms 158, surface/advance_cpu_ms 67.93, encode//sum 10.53, stage/render/render 9.70, stage/render/prepare_resources 8.29 |
| warm-up | 3.27 | 61.7 | render/thread/ms 705, render/busy/ms 544, stage/render/prepare_assets 501, main/busy/ms 159, stage/render/prepare_views 159, render/acquire/ms 158, stage/main/update 113, pipelines/compile_ms 63.22, pipelines/front_ms 59.71, stage/main/post_update 36.38, stage/render/render 22.54, encode//sum 20.58 |
| warm-up | 3.34 | 52.6 | pipelines/compile_ms 55.41, pipelines/front_ms 51.88, render/thread/ms 50.76, main/busy/ms 49.63, render/busy/ms 46.73, stage/main/post_update 37.99, terrain/update/ms 15.81, stage/render/prepare_assets 14.64, stage/render/prepare_resources 13.46, pipelines/wait_ms 10.08, stage/render/render 9.91, stage/main/update 7.00 |
| warm-up | 3.48 | 126.6 | pipelines/wait_ms 6450, pipelines/compile_ms 2187, pipelines/front_ms 1964, pipelines/driver_ms 223, render/thread/ms 126, render/busy/ms 124, main/busy/ms 83.82, stage/main/last 32.48, stage/main/update 28.27, stage/render/prepare_resources 24.89, stage/render/render 21.69, stage/render/prepare_write_phase_buffers 21.19 |
| warm-up | 3.53 | 59.2 | pipelines/wait_ms 10.2 k, pipelines/compile_ms 2971, pipelines/front_ms 2697, pipelines/driver_ms 273, render/thread/ms 158, render/busy/ms 156, main/busy/ms 133, stage/main/update 79.26, stage/render/prepare_assets 36.85, stage/render/prepare_resources 32.88, stage/main/last 32.45, stage/render/render 24.01 |
| warm-up | 3.66 | 121.4 | pipelines/wait_ms 4448, pipelines/compile_ms 1932, pipelines/front_ms 1834, render/thread/ms 134, render/busy/ms 132, pipelines/driver_ms 97.45, stage/render/prepare_assets 96.57, main/busy/ms 79.02, stage/main/update 58.38, stage/render/render 17.01, stage/main/post_update 15.36, render/graph/ms 14.05 |
| warm-up | 4.07 | 56.0 | pipelines/compile_ms 427, pipelines/front_ms 412, render/thread/ms 76.74, render/busy/ms 73.64, main/busy/ms 42.23, stage/main/post_update 33.06, stage/render/render 21.12, pipelines/driver_ms 15.71, stage/render/prepare_resources 15.01, stage/render/prepare_assets 14.73, render/graph/ms 14.52, stage/render/prepare_write_phase_buffers 12.83 |
| warm-up | 4.10 | 34.2 | pipelines/compile_ms 1078, pipelines/front_ms 1046, render/thread/ms 81.12, render/busy/ms 79.53, main/busy/ms 33.54, pipelines/driver_ms 32.14, stage/render/prepare_resources 23.63, stage/main/post_update 21.70, stage/render/prepare_assets 21.03, stage/render/prepare_write_phase_buffers 14.08, stage/render/render 11.72, bigspace/total/ms 9.72 |
| warm-up | 4.16 | 59.1 | pipelines/compile_ms 1318, pipelines/front_ms 1280, render/thread/ms 85.22, render/busy/ms 83.41, main/busy/ms 81.33, stage/main/post_update 45.35, pipelines/driver_ms 37.76, stage/main/update 28.38, stage/render/prepare_resources 20.03, stage/render/prepare_meshes 16.84, stage/render/prepare_assets 15.16, stage/render/render 14.36 |
| warm-up | 4.22 | 55.0 | pipelines/compile_ms 1830, pipelines/front_ms 1776, render/thread/ms 100, render/busy/ms 98.26, main/busy/ms 82.44, pipelines/driver_ms 53.37, stage/main/post_update 50.05, pipelines/wait_ms 38.35, stage/render/prepare_assets 29.93, stage/main/update 25.03, stage/render/prepare_flush 22.01, stage/render/prepare_meshes 16.88 |
| warm-up | 4.27 | 54.6 | pipelines/compile_ms 1457, pipelines/front_ms 1410, main/busy/ms 114, render/thread/ms 97.04, render/busy/ms 92.58, stage/main/post_update 62.58, stage/render/prepare_flush 51.55, pipelines/driver_ms 47.51, pipelines/wait_ms 38.83, stage/main/fixed_main_loop 30.13, stage/render/prepare_assets 24.82, stage/main/update 18.15 |
| warm-up | 4.33 | 57.8 | pipelines/compile_ms 1435, pipelines/front_ms 1382, pipelines/wait_ms 123, main/busy/ms 101, render/thread/ms 67.28, render/busy/ms 62.77, pipelines/driver_ms 53.60, stage/main/post_update 50.56, stage/render/prepare_flush 41.07, stage/main/fixed_main_loop 30.31, stage/main/update 18.07, stage/render/render 13.11 |
| warm-up | 4.37 | 35.2 | pipelines/compile_ms 1610, pipelines/front_ms 1555, pipelines/wait_ms 124, pipelines/driver_ms 55.12, main/busy/ms 50.55, render/thread/ms 47.62, render/busy/ms 45.31, stage/main/post_update 28.11, stage/render/render 17.01, render/graph/ms 14.58, stage/render/graph_submit 10.59, stage/render/prepare_assets 10.51 |
| warm-up | 4.39 | 27.9 | pipelines/compile_ms 539, pipelines/front_ms 519, main/busy/ms 54.95, render/thread/ms 48.91, render/busy/ms 46.51, stage/main/post_update 30.18, pipelines/driver_ms 19.23, stage/render/render 16.57, render/graph/ms 14.06, stage/render/graph_submit 10.21, stage/render/prepare_assets 9.28, stage/main/pre_update 9.05 |
| warm-up | 4.47 | 73.9 | pipelines/compile_ms 1230, pipelines/front_ms 1199, render/thread/ms 87.17, render/busy/ms 85.35, pipelines/wait_ms 77.17, pipelines/driver_ms 31.45, stage/render/prepare_resources 27.04, main/busy/ms 25.17, stage/render/prepare_flush 23.18, stage/render/prepare_write_phase_buffers 15.71, stage/render/render 11.74, stage/main/post_update 10.21 |
| warm-up | 4.53 | 40.3 | pipelines/compile_ms 419, pipelines/front_ms 408, render/thread/ms 52.71, render/busy/ms 51.30, main/busy/ms 42.07, pipelines/wait_ms 35.04, stage/main/post_update 23.18, stage/render/prepare_resources 13.94, stage/main/update 12.32, stage/render/prepare_write_phase_buffers 11.55, pipelines/driver_ms 11.54, stage/render/render 10.83 |
| warm-up | 4.57 | 31.1 | pipelines/compile_ms 232, pipelines/front_ms 227, render/thread/ms 37.84, render/busy/ms 36.10, main/busy/ms 35.09, stage/render/prepare_assets 12.41, stage/main/fixed_main_loop 12.03, stage/render/render 11.19, render/graph/ms 9.06, stage/main/update 9.02, stage/render/prepare_flush 8.18, encode//sum 7.51 |
| warm-up | 4.62 | 27.7 | pipelines/compile_ms 1065, pipelines/front_ms 1035, main/busy/ms 38.76, pipelines/wait_ms 37.27, pipelines/driver_ms 30.35, render/thread/ms 26.67, stage/main/update 26.05, render/busy/ms 24.91, stage/render/render 11.33, render/graph/ms 9.07, encode//sum 8.94, stage/main/post_update 8.02 |
| warm-up | 4.71 | 75.7 | pipelines/compile_ms 1312, pipelines/front_ms 1278, pipelines/wait_ms 51.17, main/busy/ms 43.57, render/thread/ms 39.55, render/busy/ms 37.59, pipelines/driver_ms 34.21, stage/main/update 34.18, stage/render/prepare_assets 20.56, stage/render/render 12.78, render/graph/ms 10.42, encode//sum 8.35 |
| warm-up | 4.77 | 64.2 | pipelines/compile_ms 1177, pipelines/front_ms 1144, render/thread/ms 75.61, render/busy/ms 74.04, stage/render/prepare_assets 61.18, pipelines/wait_ms 55.36, pipelines/driver_ms 33.16, main/busy/ms 14.70, stage/render/render 9.63, render/graph/ms 7.99, encode//sum 6.61, stage/render/graph_submit 6.08 |
| warm-up | 4.89 | 34.2 | pipelines/compile_ms 386, pipelines/front_ms 370, render/thread/ms 53.50, render/busy/ms 51.92, main/busy/ms 44.01, stage/main/post_update 21.93, pipelines/driver_ms 15.53, stage/main/update 15.34, stage/render/prepare_assets 13.91, stage/render/prepare_resources 10.05, stage/render/render 9.20, bigspace/total/ms 7.95 |
| warm-up | 5.03 | 30.4 | pipelines/compile_ms 720, pipelines/front_ms 691, render/thread/ms 53.66, render/busy/ms 48.94, pipelines/driver_ms 29.35, stage/render/render 22.32, main/busy/ms 20.95, render/graph/ms 17.36, encode//sum 13.99, stage/main/post_update 11.96, stage/render/graph_submit 11.53, stage/render/prepare_resources 8.12 |
| warm-up | 5.08 | 46.7 | pipelines/compile_ms 553, pipelines/front_ms 530, render/thread/ms 69.00, render/busy/ms 64.59, main/busy/ms 37.43, stage/main/post_update 28.60, pipelines/driver_ms 23.63, stage/render/prepare_resources 17.35, stage/render/render 17.21, stage/render/prepare_assets 16.80, encode//sum 13.27, render/graph/ms 12.40 |
| warm-up | 5.12 | 44.9 | pipelines/compile_ms 982, pipelines/front_ms 961, render/thread/ms 80.65, render/busy/ms 79.23, main/busy/ms 62.48, stage/render/prepare_assets 37.40, stage/main/update 28.29, stage/main/post_update 26.62, pipelines/driver_ms 21.29, stage/render/prepare_resources 19.54, pipelines/wait_ms 13.19, stage/render/render 8.00 |
| warm-up | 5.16 | 34.6 | pipelines/compile_ms 1424, pipelines/front_ms 1393, render/thread/ms 73.66, render/busy/ms 72.16, main/busy/ms 56.74, pipelines/wait_ms 37.84, stage/render/prepare_assets 34.92, stage/main/update 31.22, pipelines/driver_ms 30.70, stage/main/post_update 15.20, stage/render/render 12.87, stage/render/prepare_flush 9.96 |
| warm-up | 5.20 | 38.7 | pipelines/compile_ms 953, pipelines/front_ms 917, render/thread/ms 69.98, render/busy/ms 68.31, main/busy/ms 56.71, pipelines/driver_ms 35.74, pipelines/wait_ms 25.88, stage/main/post_update 25.17, stage/render/render 24.47, stage/main/update 21.61, render/graph/ms 19.78, stage/render/prepare_assets 14.96 |
| warm-up | 5.23 | 33.5 | pipelines/compile_ms 731, pipelines/front_ms 692, main/busy/ms 76.52, render/thread/ms 69.80, render/busy/ms 67.39, stage/main/update 41.93, pipelines/driver_ms 39.57, stage/render/render 33.98, render/graph/ms 31.21, encode//sum 21.50, stage/main/post_update 20.72, stage/render/camera_driver 19.25 |
| warm-up | 5.26 | 29.9 | pipelines/compile_ms 1190, pipelines/front_ms 1154, render/thread/ms 53.00, main/busy/ms 51.43, render/busy/ms 50.75, pipelines/driver_ms 36.45, stage/main/update 31.30, encode//sum 27.58, stage/render/render 26.20, render/graph/ms 23.12, pipelines/wait_ms 12.00, stage/render/graph_submit 11.72 |
| warm-up | 5.46 | 27.7 | render/thread/ms 32.86, render/busy/ms 31.34, main/busy/ms 17.84, stage/render/prepare_assets 14.36, encode//sum 13.33, stage/render/render 10.34, render/graph/ms 8.13, stage/main/post_update 6.91, stage/main/update 6.79, stage/render/graph_submit 5.03, stage/render/camera_driver 2.90, encode/submit 2.72 |
| warm-up | 5.52 | 38.8 | render/thread/ms 47.42, render/busy/ms 45.45, main/busy/ms 31.19, stage/render/prepare_assets 20.12, encode//sum 17.51, stage/main/update 17.50, stage/render/render 14.41, pipelines/compile_ms 12.44, render/graph/ms 11.72, pipelines/front_ms 11.17, stream/drive/ms 10.70, stage/main/post_update 7.96 |
| warm-up | 5.56 | 35.7 | main/busy/ms 117, stage/main/update 105, render/thread/ms 54.96, render/busy/ms 52.72, stage/render/prepare_assets 25.95, encode//sum 17.24, stage/render/render 15.34, render/graph/ms 12.31, stream/drive/ms 10.89, stage/render/graph_submit 8.02, stage/main/post_update 7.18, encode/submit 4.93 |
| warm-up | 5.73 | 171.9 | pipelines/compile_ms 174, pipelines/front_ms 170, main/busy/ms 112, stage/main/update 102, render/thread/ms 94.94, render/busy/ms 93.48, stage/render/prepare_assets 76.73, encode//sum 12.46, stage/render/render 10.42, stream/drive/ms 8.65, render/graph/ms 8.24, stage/main/post_update 6.42 |
| warm-up | 5.81 | 38.4 | main/busy/ms 62.77, render/thread/ms 48.98, render/busy/ms 47.23, stage/main/update 45.78, encode//sum 17.79, stage/render/render 15.01, render/graph/ms 12.56, stage/render/prepare_assets 11.68, stage/main/post_update 9.19, stage/render/graph_submit 8.06, stage/render/specialize 6.30, stream/drive/ms 5.95 |
| warm-up | 5.87 | 62.0 | pipelines/compile_ms 86.58, pipelines/front_ms 83.01, render/thread/ms 67.45, render/busy/ms 65.69, main/busy/ms 63.93, stage/main/update 45.90, stage/render/prepare_assets 27.91, encode//sum 20.13, stage/render/render 17.17, render/graph/ms 14.55, stage/render/graph_submit 9.95, stage/main/post_update 8.98 |
| warm-up | 5.90 | 28.3 | pipelines/compile_ms 86.58, pipelines/front_ms 83.01, render/thread/ms 58.32, render/busy/ms 56.85, main/busy/ms 48.35, stage/main/update 28.03, stage/render/prepare_assets 27.74, encode//sum 19.30, stage/render/render 14.19, stage/main/post_update 12.43, render/graph/ms 11.83, stream/drive/ms 10.94 |
| warm-up | 6.03 | 59.5 | pipelines/compile_ms 484, pipelines/front_ms 471, main/busy/ms 61.03, stage/main/update 48.96, stream/drive/ms 43.07, render/thread/ms 41.67, render/busy/ms 39.89, stage/render/prepare_assets 17.35, encode//sum 17.23, pipelines/driver_ms 12.76, stage/render/render 12.10, render/graph/ms 9.42 |
| warm-up | 6.08 | 30.5 | render/thread/ms 44.86, render/busy/ms 43.28, main/busy/ms 31.77, encode//sum 22.34, stage/render/render 16.10, render/graph/ms 13.99, stage/main/post_update 13.36, stage/main/update 12.76, stage/render/graph_submit 10.08, stage/render/prepare_assets 9.14, encode/submit 5.95, encode/finish_wall 4.14 |
| warm-up | 6.59 | 35.0 | render/thread/ms 45.14, render/busy/ms 43.89, main/busy/ms 21.38, encode//sum 17.89, stage/render/prepare_flush 16.40, stage/render/render 10.41, stage/main/post_update 10.29, render/graph/ms 8.74, stage/main/update 8.24, stage/render/prepare_assets 7.90, stage/render/graph_submit 6.32, encode/finish_wall 3.30 |
| warm-up | 7.13 | 27.6 | render/thread/ms 38.30, render/busy/ms 36.61, main/busy/ms 31.06, encode//sum 24.37, stage/main/update 15.98, stage/render/render 15.93, render/graph/ms 13.48, stage/render/prepare_assets 11.01, stage/render/graph_submit 10.33, stage/main/post_update 10.11, encode/finish_wall 5.42, encode/submit 4.99 |
| measure | 18.20 | 34.0 | main/busy/ms 42.37, stage/main/update 26.60, encode//sum 25.13, render/thread/ms 23.68, render/busy/ms 22.56, stage/render/render 13.81, stage/main/post_update 12.20, render/graph/ms 12.19, stage/render/graph_submit 9.62, encode/finish_wall 5.52, encode/submit 4.11, stage/render/prepare_assets 3.28 |
| measure | 59.05 | 31.1 | render/thread/ms 40.22, render/busy/ms 38.90, main/busy/ms 26.11, encode//sum 19.81, stage/render/prepare_assets 16.62, stage/main/post_update 14.54, stage/render/render 13.08, render/graph/ms 11.52, stage/render/graph_submit 8.45, stage/main/update 6.35, encode/finish_wall 4.90, encode/submit 3.60 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\f_scene\balmora_walk@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **80%** (min 49%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.9 W · 72 °C (max 78) · VRAM 5238 MiB · throttled 100% of samples
