# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5494 frames over 65.6 s · **median 11.75 ms (85 fps)** · mean 11.95 · p95 13.93 · p99 15.72 (1% low 64 fps) · worst 34.52 ms · 29 spikes over 1.5× median · 22.8 stutters/min (work 23.8/min)
- warm-up: 398 frames over 8.8 s, worst 2642.0 ms
- hitches (> 29.4 ms): **3** measured, 16 in warm-up
- timed GPU passes: 7.42 ms of a 11.95 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.75 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 34.52 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.040 | 2.064 | 2.097 | 0.99 | 0.171 |
| main_opaque_pass_3d | 1.588 | 1.606 | 1.940 | 0.99 | 0.062 |
| early prepass | 0.554 | 0.561 | 0.794 | 0.99 | 0.081 |
| bloom | 0.316 | 0.319 | 0.325 | 0.99 | 0.061 |
| atmosphere_luts | 0.309 | 0.313 | 0.372 | 0.99 | 0.003 |
| water_velocity | 0.282 | 0.286 | 0.354 | 0.99 | 0.002 |
| water_sspr | 0.206 | 0.209 | 0.211 | 0.99 | 0.002 |
| ssao | 0.190 | 0.192 | 0.193 | 0.99 | 0.007 |
| uniform_allocation | 0.163 | 0.164 | 0.172 | 0.99 | 0.031 |
| atmos_froxel_inject | 0.147 | 0.149 | 0.150 | 0.99 | 0.001 |
| clouds_trace | 0.139 | 0.141 | 0.234 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.138 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.128 | 0.129 | 0.440 | 0.99 | 0.004 |
| bin_unpacking | 0.126 | 0.127 | 0.134 | 0.99 | 0.024 |
| upscaling | 0.115 | 0.117 | 0.126 | 0.99 | 0.004 |
| clouds_composite | 0.106 | 0.107 | 0.110 | 0.99 | 0.002 |
| auxiliary_views | 0.101 | 0.102 | 0.479 | 0.99 | 0.099 |
| gpu_instance_materialization | 0.095 | 0.096 | 0.100 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.087 | 0.088 | 0.167 | 0.99 | 0.008 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.003 |
| clustering | 0.065 | 0.066 | 0.068 | 0.99 | 0.020 |
| lightprobe_irradiance_map | 0.053 | 0.450 | 0.733 | 0.12 | 0.001 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.043 | 0.057 | 0.057 | 0.76 | 0.002 |
| per_view_shadow_pass | 0.037 | 0.037 | 0.040 | 0.99 | 0.008 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.99 | 0.003 |
| water_snapshot | 0.030 | 0.031 | 0.034 | 0.99 | 0.002 |
| range_unpacking | 0.029 | 0.029 | 0.034 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.026 | 0.223 | 0.234 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.025 | 0.028 | 0.99 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.032 | 0.99 | 0.079 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 13.071 | 0.000 | 10213.26 | 71810.9 |
| render/thread/ms | 10.790 | 12.643 | 24.42 | 59282.6 |
| encode//sum | 10.545 | 14.408 | 54.35 | 57936.0 |
| render/busy/ms | 10.217 | 12.068 | 23.82 | 56133.0 |
| main/busy/ms | 8.910 | 11.093 | 30.95 | 48949.8 |
| stage/render/render | 6.838 | 8.130 | 16.16 | 37570.6 |
| render/graph/ms | 6.084 | 7.333 | 15.11 | 33423.7 |
| stage/main/post_update | 5.318 | 7.332 | 9.43 | 29214.9 |
| stage/render/graph_submit | 4.972 | 5.995 | 11.52 | 27314.9 |
| encode/finish_wall | 2.829 | 3.545 | 6.08 | 15543.8 |
| stage/main/update | 2.186 | 2.645 | 25.19 | 12012.6 |
| encode/submit | 2.172 | 2.833 | 6.28 | 11934.0 |
| stage/render/camera_driver | 1.050 | 1.353 | 3.89 | 5767.7 |
| stage/render/prepare_write_phase_buffers | 0.812 | 1.032 | 2.08 | 4463.0 |
| stage/render/prepare_assets | 0.553 | 0.700 | 12.12 | 3038.2 |
| bigspace/total/ms | 0.533 | 1.270 | 4.31 | 2930.8 |
| render/present/ms | 0.484 | 0.620 | 3.08 | 2660.1 |
| stage/render/prepare_resources | 0.435 | 0.523 | 1.01 | 2390.5 |
| stage/main/first | 0.409 | 0.528 | 4.77 | 2248.7 |
| stage/render/prepare_meshes | 0.396 | 0.530 | 3.30 | 2176.3 |
| stage/main/pre_update | 0.333 | 0.479 | 2.57 | 1829.3 |
| stage/render/prepare_flush | 0.314 | 0.525 | 2.69 | 1722.7 |
| stage/render/prepare_bind_groups | 0.314 | 0.527 | 3.02 | 1722.5 |
| terrain/update/ms | 0.292 | 0.377 | 2.09 | 1605.9 |
| controller/feed/ms | 0.286 | 0.372 | 0.68 | 1573.8 |
| terrain/select/ms | 0.259 | 0.335 | 0.67 | 1425.0 |
| stage/render/prepare_batch_phases | 0.259 | 0.358 | 0.72 | 1423.2 |
| stream/drive/ms | 0.251 | 0.348 | 13.12 | 1381.0 |
| stage/render/specialize | 0.247 | 0.358 | 1.95 | 1358.2 |
| pipelines/compile_ms | 0.247 | 0.000 | 162.87 | 1355.7 |
| stage/main/last | 0.246 | 0.310 | 2.91 | 1351.1 |
| pipelines/front_ms | 0.238 | 0.000 | 157.38 | 1308.4 |
| stage/main/fixed_main_loop | 0.234 | 0.376 | 0.95 | 1282.9 |
| stage/render/queue | 0.231 | 0.353 | 2.03 | 1267.7 |
| bigspace/low_precision/ms | 0.227 | 0.610 | 3.89 | 1249.3 |
| stream/tick/ms | 0.194 | 0.271 | 0.56 | 1064.1 |
| far/select/ms | 0.193 | 0.273 | 0.62 | 1062.6 |
| stage/render/extract_commands | 0.177 | 0.210 | 0.40 | 975.0 |
| stage/render/create_views | 0.162 | 0.209 | 1.25 | 892.0 |
| anim/evaluate_cpu/ms | 0.162 | 0.204 | 0.54 | 890.7 |
| bigspace/high_precision/ms | 0.152 | 0.429 | 3.44 | 833.4 |
| anim/evaluate/ms | 0.141 | 0.254 | 2.83 | 775.1 |
| stage/render/prepare_views | 0.131 | 0.166 | 0.38 | 717.7 |
| controller/move/ms | 0.130 | 0.215 | 0.42 | 714.4 |
| render/acquire/ms | 0.089 | 0.117 | 0.34 | 489.4 |
| physics/step/ms | 0.052 | 0.090 | 0.61 | 285.7 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.27 | 259.8 |
| physics/insert/ms | 0.041 | 0.401 | 0.88 | 226.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.47 | 171.7 |
| stage/render/cleanup | 0.026 | 0.039 | 0.60 | 142.0 |
| stage/render/phase_sort | 0.024 | 0.043 | 0.27 | 131.6 |
| weather/map/ms | 0.023 | 0.032 | 0.15 | 127.7 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.44 | 76.3 |
| anim/apply/ms | 0.012 | 0.021 | 0.23 | 64.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.21 | 58.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.07 | 56.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 54.3 |
| pipelines/driver_ms | 0.009 | 0.000 | 5.49 | 47.3 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 39.4 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 29.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.18 | 29.2 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 25.1 |
| stream/upload/ms | 0.004 | 0.000 | 1.01 | 20.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 18.0 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 17.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 14.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 2.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 564 | 579 | 578 |
| anim/characters/count | count | 567 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1306 | 2262 | 535 |
| crowd/c0/meshes/count | count | 362 | 646 | 144 |
| crowd/c1/count | count | 144 | 154 | 149 |
| crowd/c1/joints/count | count | 5475 | 5882 | 5661 |
| crowd/c1/meshes/count | count | 1162 | 1254 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4254 | 13.3 k | 6179 |
| crowd/joints_uploaded/count | count | 4232 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.7 k | 46.9 k |
| encode//max | ms | 2.42 | 4.39 | 2.27 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.44 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.21 | 0.01 |
| encode/finish_critical | ms | 2.42 | 4.39 | 2.27 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.12 | 30.95 | 8.99 |
| gpu/timed/ms | ms | 8.16 | 10.24 | 8.35 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2512 | 2569 | 2569 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 286.5 k | 286.5 k | 286.5 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.6 k | 11.8 k | 10.4 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 323.97 M | 323.97 M | 323.97 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 329 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 329 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 761 | 972 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2517 | 3561 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 761 | 972 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2517 | 3561 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 47 | 192 | 40 |
| phases/shadow/bins | count | 1826 | 2460 | 955 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5466 | 6849 | 3852 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 165 | 219 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2662 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 978 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 181 | 183 | 183 |
| pipelines/waiting | count | 0 | 15 | 0 |
| preprocess/bind_groups | count | 17 | 526 | 13 |
| preprocess/bind_groups_created | count | 10 | 519 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| preprocess/bind_groups_reused | count | 175 | 246 | 169 |
| process/cpu_usage | % | 25 | 37.50 | 23.66 |
| process/mem_usage | GiB | 5.09 | 5.27 | 5.27 |
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
| started_load_count |  | 6296 | 6420 | 6420 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.55 | 45.44 | 37.15 |
| system/mem_usage | % | 60.13 | 61.01 | 60.95 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2817 | 2932 | 2932 |
| textures/images | count | 2578 | 2636 | 2636 |
| views/all/batch_sets | count | 103 | 247 | 96 |
| views/all/bins | count | 3732 | 4522 | 3237 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.8 k | 33.8 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 11.2 k | 13.8 k | 9792 |
| views/all/pipelines | count | 209 | 266 | 74 |
| views/all/pipelines_distinct | count | 68 | 73 | 59 |
| views/all/work_items | count | 256.3 k | 259.0 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1434 | 1807 | 955 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4748 | 6050 | 3852 |
| views/cascade/pipelines | count | 38 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.5 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 147 | 0 |
| views/lamp_face/bins | count | 395 | 664 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 638 | 1034 | 0 |
| views/lamp_face/pipelines | count | 128 | 171 | 0 |
| views/lamp_face/work_items | count | 0 | 716 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2026 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5951 | 7542 | 5940 |
| views/main/pipelines | count | 45 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
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
| anim/bones_written/count | count | 54.07 | 281 | 297.0 k |
| anim/events/count | count | 0.09 | 3 | 471 |
| assets/Image/added | count | 0.11 | 43 | 616 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.56 | 342 | 3055 |
| assets/Mesh/removed | count | 0.89 | 1238 | 4896 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 505 |
| assets/StandardMaterial/modified | count | 3.94 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 40 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.58 | 1485 | 3201 |
| encode//count | count | 38.06 | 273 | 209.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5494 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5494 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5494 |
| physics/inserts | count | 0.59 | 164 | 3220 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 145 | 2335 |
| physics/steps | count | 0.72 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 3 | 12 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 6162 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1351 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.4 ms; slowest 19, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2642.0 | main/busy/ms 2617, stage/main/post_update 2499, stage/main/update 112, surface/advance_cpu_ms 84.09, stage/main/pre_update 3.18, stage/main/last 1.01, surface/cell_upload_ms 0.87, stage/main/first 0.53, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.31, weather/map/ms 0.27, bigspace/total/ms 0.10 |
| warm-up | 3.57 | 927.7 | main/busy/ms 2649, stage/main/post_update 2521, render/thread/ms 892, render/busy/ms 790, stage/render/prepare_assets 513, stage/render/prepare_resources 259, stage/main/update 116, stage/render/prepare_views 99.87, render/acquire/ms 99.67, surface/advance_cpu_ms 84.09, stage/render/render 8.33, stage/render/create_views 7.69 |
| warm-up | 3.83 | 79.6 | main/busy/ms 61.70, stage/main/update 51.75, render/thread/ms 49.31, render/busy/ms 47.55, stage/render/prepare_assets 38.35, stage/main/post_update 7.07, stage/render/render 5.88, render/graph/ms 3.65, stream/drive/ms 1.82, stage/render/camera_driver 1.78, stage/render/graph_submit 1.75, stream/tick/ms 1.49 |
| warm-up | 4.01 | 63.6 | main/busy/ms 48.69, render/thread/ms 43.88, render/busy/ms 41.97, stage/main/update 39.34, stage/render/prepare_assets 30.60, stage/render/render 7.38, stage/main/post_update 5.78, render/graph/ms 4.94, stage/render/graph_submit 3.36, encode/submit 2.87, render/present/ms 1.65, encode//sum 1.47 |
| warm-up | 4.14 | 36.6 | pipelines/wait_ms 7080, main/busy/ms 125, pipelines/compile_ms 109, stage/main/update 109, pipelines/front_ms 99.67, render/thread/ms 47.12, render/busy/ms 44.49, stage/render/prepare_assets 27.98, stage/render/render 11.27, pipelines/driver_ms 9.54, stage/main/post_update 9.48, render/graph/ms 7.61 |
| warm-up | 4.33 | 186.8 | pipelines/wait_ms 11.5 k, pipelines/compile_ms 170, pipelines/front_ms 156, main/busy/ms 126, stage/main/update 108, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 87.57, pipelines/driver_ms 14.07, stage/main/post_update 10.36, stage/render/render 10.27, render/graph/ms 7.20 |
| warm-up | 4.48 | 37.4 | pipelines/wait_ms 657, pipelines/compile_ms 202, pipelines/front_ms 187, render/thread/ms 46.90, encode//sum 45.59, render/busy/ms 44.57, main/busy/ms 38.61, stage/main/update 25.76, stage/render/render 18.79, stream/drive/ms 18.32, stage/render/prepare_assets 18.07, render/graph/ms 15.77 |
| warm-up | 4.51 | 33.8 | pipelines/wait_ms 657, pipelines/compile_ms 202, pipelines/front_ms 187, render/thread/ms 51.12, render/busy/ms 48.80, main/busy/ms 29.96, stage/render/prepare_assets 27.85, pipelines/driver_ms 14.31, stage/main/update 13.08, stage/render/render 12.30, stage/main/post_update 9.62, render/graph/ms 8.93 |
| warm-up | 4.56 | 45.8 | render/thread/ms 50.83, render/busy/ms 49.11, stage/render/prepare_assets 30.88, main/busy/ms 29.64, stage/main/update 13.46, stage/render/render 9.98, stage/main/post_update 8.67, render/graph/ms 7.48, stream/drive/ms 5.89, encode//sum 5.45, stage/render/graph_submit 5.09, encode/submit 3.99 |
| warm-up | 4.60 | 47.9 | pipelines/wait_ms 614, pipelines/compile_ms 147, pipelines/front_ms 143, render/thread/ms 54.90, render/busy/ms 53.75, stage/render/prepare_assets 37.73, main/busy/ms 24.88, stage/main/update 12.94, stream/drive/ms 6.56, stage/render/render 6.08, stage/main/post_update 5.60, render/graph/ms 4.54 |
| warm-up | 4.64 | 31.8 | pipelines/wait_ms 988, main/busy/ms 67.76, stage/main/update 55.28, stream/drive/ms 50.83, pipelines/compile_ms 20.10, pipelines/front_ms 18.95, render/thread/ms 18.63, render/busy/ms 17.18, stage/main/post_update 8.13, stage/render/render 6.10, stage/render/prepare_assets 4.91, render/graph/ms 4.22 |
| warm-up | 4.69 | 47.2 | main/busy/ms 52.97, stage/main/update 39.99, stream/drive/ms 35.55, render/thread/ms 24.91, render/busy/ms 23.57, stage/main/post_update 9.81, stage/render/prepare_assets 8.11, stage/render/render 4.86, render/graph/ms 3.18, encode//sum 2.66, stage/render/specialize 2.33, stage/render/prepare_write_phase_buffers 2.22 |
| warm-up | 4.84 | 46.7 | main/busy/ms 51.66, stage/main/update 44.12, stream/drive/ms 38.57, render/thread/ms 24.82, render/busy/ms 23.75, stage/render/prepare_assets 13.02, stage/main/post_update 4.74, stage/render/render 4.63, render/graph/ms 3.13, encode//sum 2.52, stage/render/graph_submit 2.03, stage/render/prepare_write_phase_buffers 1.61 |
| warm-up | 5.78 | 54.5 | pipelines/wait_ms 7048, main/busy/ms 52.42, render/thread/ms 41.10, render/busy/ms 39.28, stage/main/update 37.90, pipelines/compile_ms 23.02, pipelines/front_ms 19.93, stage/render/prepare_assets 18.00, stage/render/render 10.09, encode//sum 9.02, stage/main/post_update 8.87, render/graph/ms 7.66 |
| warm-up | 5.91 | 30.6 | pipelines/wait_ms 2219, pipelines/compile_ms 73.11, pipelines/front_ms 69.38, main/busy/ms 43.35, render/thread/ms 34.22, render/busy/ms 32.75, stage/main/update 31.30, stage/render/prepare_assets 10.02, stage/main/post_update 8.52, stage/render/render 8.51, encode//sum 7.80, render/graph/ms 6.54 |
| warm-up | 6.28 | 31.7 | main/busy/ms 45.12, stage/main/update 34.68, render/thread/ms 23.25, render/busy/ms 21.53, encode//sum 10.93, stage/render/render 9.84, stage/main/post_update 7.64, render/graph/ms 7.36, stage/render/graph_submit 4.77, stage/render/prepare_assets 3.83, stage/render/prepare_write_phase_buffers 2.75, encode/submit 2.59 |
| measure | 9.71 | 30.6 | main/busy/ms 33.00, render/thread/ms 29.70, render/busy/ms 28.58, encode//sum 26.70, stage/main/update 19.04, stage/render/render 13.80, stream/drive/ms 13.49, render/graph/ms 12.07, stage/main/post_update 10.78, stage/render/graph_submit 9.91, stage/render/prepare_assets 6.43, encode/finish_wall 6.00 |
| measure | 18.14 | 34.5 | main/busy/ms 41.84, stage/main/update 27.69, render/thread/ms 23.92, render/busy/ms 22.79, encode//sum 20.72, stage/render/render 13.29, render/graph/ms 11.77, stage/main/post_update 10.87, stage/render/graph_submit 9.69, encode/finish_wall 5.44, encode/submit 4.29, stage/render/prepare_assets 4.03 |
| measure | 59.03 | 33.5 | render/thread/ms 40.12, render/busy/ms 38.82, main/busy/ms 23.51, encode//sum 19.01, stage/render/prepare_assets 18.93, stage/render/render 12.49, stage/main/post_update 11.02, render/graph/ms 10.97, stage/render/graph_submit 8.72, stage/main/update 6.31, encode/finish_wall 4.90, encode/submit 3.86 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on15\balmora_walk@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **80%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.5 W · 73 °C (max 78) · VRAM 6083 MiB · throttled 100% of samples
