# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1412 frames over 15.0 s · **median 10.54 ms (95 fps)** · mean 10.62 · p95 11.76 · p99 12.80 (1% low 78 fps) · worst 21.43 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 327 frames over 7.9 s, worst 2587.4 ms
- hitches (> 26.4 ms): **0** measured, 23 in warm-up
- timed GPU passes: 8.61 ms of a 10.62 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.54 ms, limit 11 ms
- pass worst frame 21.43 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.490 | 2.533 | 2.721 | 0.98 | 0.066 |
| dlss_super_resolution | 2.015 | 2.049 | 2.071 | 0.98 | 0.178 |
| early prepass | 0.827 | 0.841 | 1.039 | 0.98 | 0.084 |
| atmosphere_luts | 0.379 | 0.386 | 0.702 | 0.98 | 0.003 |
| bloom | 0.313 | 0.318 | 0.334 | 0.98 | 0.062 |
| main_transmissive_pass_3d | 0.275 | 0.279 | 0.298 | 0.98 | 0.009 |
| water_velocity | 0.240 | 0.244 | 0.270 | 0.98 | 0.002 |
| water_sspr | 0.218 | 0.222 | 0.225 | 0.98 | 0.002 |
| ssao | 0.201 | 0.205 | 0.214 | 0.98 | 0.008 |
| clouds_trace | 0.148 | 0.150 | 0.183 | 0.98 | 0.001 |
| uniform_allocation | 0.147 | 0.150 | 0.148 | 0.98 | 0.031 |
| atmos_froxel_inject | 0.145 | 0.148 | 0.151 | 0.98 | 0.002 |
| water_fft | 0.137 | 0.139 | 0.138 | 0.98 | 0.003 |
| upscaling | 0.112 | 0.114 | 0.122 | 0.98 | 0.004 |
| clouds_composite | 0.112 | 0.114 | 0.116 | 0.98 | 0.002 |
| bin_unpacking | 0.112 | 0.114 | 0.113 | 0.98 | 0.021 |
| gpu_instance_materialization | 0.088 | 0.090 | 0.092 | 0.98 | 0.004 |
| early_mesh_preprocessing | 0.085 | 0.087 | 0.093 | 0.98 | 0.006 |
| tonemapping | 0.085 | 0.087 | 0.092 | 0.98 | 0.003 |
| clustering | 0.078 | 0.080 | 0.089 | 0.98 | 0.022 |
| render_sky | 0.053 | 0.054 | 0.055 | 0.98 | 0.002 |
| clouds_resolve | 0.051 | 0.052 | 0.054 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.419 | 0.417 | 0.12 | 0.001 |
| per_view_shadow_pass | 0.042 | 0.043 | 0.045 | 0.98 | 0.009 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.98 | 0.003 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.230 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.028 | 0.98 | 0.001 |
| range_unpacking | 0.027 | 0.027 | 0.028 | 0.98 | 0.005 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.018 | 0.98 | 0.040 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 48.794 | 0.000 | 8675.84 | 68897.1 |
| render/thread/ms | 9.728 | 10.849 | 14.52 | 13736.4 |
| encode//sum | 8.606 | 12.233 | 35.40 | 12151.7 |
| render/busy/ms | 8.181 | 9.589 | 13.52 | 11551.3 |
| main/busy/ms | 7.295 | 8.699 | 14.97 | 10299.9 |
| stage/render/render | 5.654 | 7.038 | 11.36 | 7983.4 |
| render/graph/ms | 4.936 | 6.190 | 8.48 | 6970.2 |
| stage/main/post_update | 4.273 | 5.612 | 6.83 | 6033.0 |
| stage/render/graph_submit | 3.940 | 5.153 | 7.31 | 5562.8 |
| encode/finish_wall | 2.132 | 2.758 | 4.86 | 3009.8 |
| encode/submit | 1.845 | 2.601 | 4.35 | 2604.6 |
| stage/main/update | 1.786 | 2.167 | 9.70 | 2522.3 |
| stage/render/prepare_views | 1.095 | 2.068 | 2.96 | 1545.7 |
| render/acquire/ms | 1.052 | 2.014 | 2.92 | 1484.7 |
| stage/render/camera_driver | 0.933 | 1.102 | 1.35 | 1316.8 |
| pipelines/compile_ms | 0.900 | 0.000 | 131.40 | 1270.5 |
| pipelines/front_ms | 0.869 | 0.000 | 127.08 | 1227.5 |
| stage/render/prepare_write_phase_buffers | 0.662 | 0.811 | 1.15 | 935.3 |
| render/present/ms | 0.496 | 0.610 | 4.52 | 700.3 |
| stage/render/prepare_assets | 0.452 | 0.588 | 5.71 | 638.5 |
| stage/render/prepare_resources | 0.389 | 0.462 | 0.72 | 549.7 |
| bigspace/total/ms | 0.374 | 0.737 | 2.87 | 528.3 |
| stage/main/pre_update | 0.322 | 0.412 | 0.97 | 454.1 |
| stage/render/prepare_flush | 0.312 | 0.468 | 0.92 | 440.6 |
| stage/main/first | 0.302 | 0.371 | 1.37 | 426.6 |
| stage/render/prepare_bind_groups | 0.261 | 0.343 | 0.79 | 368.0 |
| stage/render/prepare_batch_phases | 0.251 | 0.342 | 0.53 | 353.7 |
| terrain/update/ms | 0.231 | 0.302 | 0.61 | 326.2 |
| stage/main/last | 0.227 | 0.289 | 0.61 | 321.0 |
| terrain/select/ms | 0.200 | 0.261 | 0.59 | 282.5 |
| stage/main/fixed_main_loop | 0.198 | 0.355 | 1.37 | 279.1 |
| stream/drive/ms | 0.188 | 0.278 | 0.60 | 265.6 |
| stage/render/prepare_meshes | 0.185 | 0.238 | 0.42 | 260.9 |
| stage/render/extract_commands | 0.176 | 0.217 | 0.38 | 248.6 |
| far/select/ms | 0.162 | 0.240 | 0.43 | 228.4 |
| controller/feed/ms | 0.160 | 0.221 | 0.56 | 225.9 |
| stream/tick/ms | 0.160 | 0.242 | 0.57 | 225.6 |
| stage/render/create_views | 0.142 | 0.183 | 0.47 | 200.0 |
| bigspace/high_precision/ms | 0.126 | 0.319 | 2.69 | 178.3 |
| controller/move/ms | 0.105 | 0.214 | 0.36 | 147.8 |
| stage/render/queue | 0.103 | 0.134 | 0.22 | 145.1 |
| bigspace/low_precision/ms | 0.094 | 0.231 | 2.48 | 132.4 |
| stage/render/specialize | 0.087 | 0.118 | 0.34 | 123.2 |
| anim/evaluate/ms | 0.087 | 0.282 | 1.39 | 123.1 |
| anim/evaluate_cpu/ms | 0.059 | 0.079 | 0.35 | 83.5 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.074 | 0.17 | 66.5 |
| encode/resolve_diagnostics/sum | 0.043 | 0.077 | 0.36 | 60.7 |
| physics/step/ms | 0.033 | 0.060 | 1.07 | 46.3 |
| pipelines/driver_ms | 0.030 | 0.000 | 4.62 | 42.9 |
| stage/render/cleanup | 0.026 | 0.039 | 0.51 | 36.8 |
| weather/map/ms | 0.023 | 0.032 | 0.20 | 32.2 |
| encode/camera_driver_span_begin/sum | 0.019 | 0.035 | 0.28 | 27.2 |
| encode/camera_driver_span_end/sum | 0.016 | 0.032 | 0.28 | 22.4 |
| stage/render/phase_sort | 0.012 | 0.023 | 0.35 | 16.9 |
| weather/eval/ms | 0.010 | 0.015 | 0.05 | 14.7 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 10.4 |
| physics/insert/ms | 0.007 | 0.044 | 0.95 | 10.4 |
| script/input/ms | 0.005 | 0.007 | 0.06 | 7.6 |
| controller/avoid/ms | 0.005 | 0.007 | 0.18 | 7.6 |
| anim/apply/ms | 0.005 | 0.006 | 0.10 | 6.4 |
| script/ui/ms | 0.004 | 0.007 | 0.02 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.04 | 2.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.8 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 0.4 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 17 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 41 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 25 | 8 |
| crowd/c0/joints/count | count | 272 | 850 | 272 |
| crowd/c0/meshes/count | count | 48 | 150 | 48 |
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1093 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4436 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| encode//max | ms | 1.79 | 4.16 | 1.80 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.28 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.02 |
| encode/finish_critical | ms | 1.79 | 4.16 | 1.80 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.36 | 0.02 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.41 | 14.97 | 9.62 |
| gpu/timed/ms | ms | 9.40 | 10.35 | 9.62 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1837 | 1837 | 1837 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 200.8 k | 200.8 k | 200.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 9431 | 9431 | 9431 |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 277.85 M | 277.85 M | 277.85 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 877 | 877 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/bins | count | 877 | 877 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 15 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 859 | 872 | 859 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2848 | 2876 | 2848 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1050 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 165 | 165 | 165 |
| pipelines/waiting | count | 0 | 18 | 0 |
| preprocess/bind_groups | count | 7 | 25 | 7 |
| preprocess/bind_groups_created | count | 0 | 18 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| preprocess/bind_groups_reused | count | 167 | 167 | 167 |
| process/cpu_usage | % | 22.84 | 39.42 | 21.63 |
| process/mem_usage | GiB | 4.31 | 4.36 | 4.34 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5125 | 5127 | 5127 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 27.95 | 45.07 | 27.64 |
| system/mem_usage | % | 57.86 | 58.03 | 57.99 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2465 | 2465 | 2465 |
| textures/images | count | 1903 | 1903 | 1903 |
| views/all/batch_sets | count | 92 | 95 | 92 |
| views/all/bins | count | 3111 | 3124 | 3111 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9108 | 9136 | 9108 |
| views/all/pipelines | count | 73 | 76 | 73 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 859 | 872 | 859 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2848 | 2876 | 2848 |
| views/cascade/pipelines | count | 27 | 30 | 27 |
| views/cascade/work_items | count | 150.3 k | 150.3 k | 150.3 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 2252 | 2252 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 46 | 46 | 46 |
| views/main/work_items | count | 104.7 k | 104.7 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.1 k | 15.1 k | 15.1 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.21 | 297 | 297 |
| anim/events/count | count | 0.09 | 3 | 123 |
| assets/Image/added | count | 0.01 | 19 | 19 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.04 | 53 | 53 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 19 | 19 |
| assets/StandardMaterial/modified | count | 3.51 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.63 | 2191 | 2303 |
| encode//count | count | 32.25 | 34 | 45.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1412 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1412 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1412 |
| physics/inserts | count | 0.77 | 148 | 1087 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2587.4 | main/busy/ms 2562, stage/main/post_update 2463, stage/main/update 93.33, surface/advance_cpu_ms 67.43, stage/main/pre_update 2.34, stage/main/last 0.96, surface/cell_upload_ms 0.70, stage/main/first 0.66, stage/main/fixed_main_loop 0.49, stream/drive/ms 0.32, weather/map/ms 0.10, stream/tick/ms 0.08 |
| warm-up | 3.51 | 926.1 | main/busy/ms 2596, stage/main/post_update 2488, render/thread/ms 884, render/busy/ms 716, stage/render/prepare_assets 431, stage/render/prepare_resources 257, stage/render/prepare_views 166, render/acquire/ms 165, stage/main/update 96.82, surface/advance_cpu_ms 67.43, stage/render/create_views 18.72, stage/render/render 7.93 |
| warm-up | 3.75 | 81.7 | main/busy/ms 57.01, stage/main/update 46.27, render/thread/ms 42.54, render/busy/ms 40.70, stage/render/prepare_assets 30.88, stage/main/post_update 7.46, stage/render/render 6.45, render/graph/ms 3.89, stage/render/graph_submit 2.10, encode/submit 1.87, encode//sum 1.66, stage/render/camera_driver 1.65 |
| warm-up | 4.02 | 159.0 | pipelines/compile_ms 243, pipelines/front_ms 232, pipelines/wait_ms 220, render/thread/ms 95.29, main/busy/ms 73.51, render/busy/ms 67.07, stage/main/update 62.25, stage/render/prepare_assets 55.49, stage/render/render 33.99, render/present/ms 27.88, pipelines/driver_ms 11.15, stage/main/post_update 7.13 |
| warm-up | 4.06 | 40.7 | pipelines/wait_ms 2518, pipelines/compile_ms 326, pipelines/front_ms 309, render/thread/ms 97.82, render/busy/ms 70.26, stage/render/prepare_assets 62.69, stage/render/render 31.12, render/present/ms 27.31, main/busy/ms 20.32, pipelines/driver_ms 17.00, stage/main/post_update 9.65, stage/main/update 4.81 |
| warm-up | 4.15 | 32.5 | main/busy/ms 101, stage/main/update 93.27, render/thread/ms 21.15, render/busy/ms 19.50, stream/drive/ms 13.78, stage/render/prepare_assets 8.48, stage/render/render 6.62, stage/main/post_update 5.84, render/graph/ms 4.44, stage/render/graph_submit 2.49, encode//sum 2.06, encode/submit 1.85 |
| warm-up | 4.26 | 116.1 | pipelines/wait_ms 6880, pipelines/compile_ms 176, pipelines/front_ms 168, main/busy/ms 93.19, stage/main/update 82.31, render/thread/ms 53.06, render/busy/ms 50.69, stage/render/prepare_assets 32.05, stage/render/render 11.70, pipelines/driver_ms 8.76, render/graph/ms 8.53, stage/main/post_update 7.16 |
| warm-up | 4.32 | 36.8 | pipelines/wait_ms 694, pipelines/compile_ms 77.68, pipelines/front_ms 74.40, main/busy/ms 57.36, stage/main/update 43.95, stream/drive/ms 36.82, render/thread/ms 31.69, render/busy/ms 30.20, stage/render/prepare_assets 20.63, stage/main/post_update 7.06, stage/render/render 5.39, render/graph/ms 3.55 |
| warm-up | 4.37 | 54.0 | pipelines/wait_ms 3616, pipelines/compile_ms 101, pipelines/front_ms 95.93, main/busy/ms 58.54, stage/main/update 43.93, render/thread/ms 37.35, stream/drive/ms 36.77, render/busy/ms 35.85, stage/render/prepare_assets 26.68, stage/main/post_update 7.36, stage/render/render 4.88, pipelines/driver_ms 4.59 |
| warm-up | 4.54 | 26.9 | pipelines/wait_ms 1691, render/thread/ms 34.16, render/busy/ms 32.30, main/busy/ms 29.74, pipelines/compile_ms 14.21, stage/main/update 13.70, stage/render/prepare_assets 13.66, pipelines/front_ms 12.68, stage/main/post_update 10.07, stage/render/render 9.02, render/graph/ms 6.45, stream/drive/ms 5.79 |
| warm-up | 4.57 | 28.9 | render/thread/ms 36.25, render/busy/ms 34.89, main/busy/ms 31.99, stage/render/prepare_assets 19.83, stage/main/update 16.42, stream/drive/ms 10.63, stage/main/post_update 8.09, stage/render/render 6.02, render/graph/ms 4.24, encode//sum 3.51, stage/main/pre_update 2.30, stage/render/graph_submit 2.29 |
| warm-up | 4.87 | 26.6 | render/thread/ms 31.51, render/busy/ms 29.90, main/busy/ms 25.78, stage/render/prepare_assets 14.44, stage/main/update 13.20, stage/render/render 8.81, render/graph/ms 6.46, stage/main/post_update 6.35, stage/main/pre_update 4.41, stage/render/graph_submit 4.07, encode//sum 3.83, encode/submit 3.16 |
| warm-up | 4.89 | 26.5 | pipelines/wait_ms 1349, render/thread/ms 40.64, render/busy/ms 38.58, main/busy/ms 26.44, stage/render/prepare_assets 19.42, stage/main/update 13.75, stage/render/render 10.18, stage/main/post_update 9.05, render/graph/ms 7.12, encode//sum 5.58, stage/render/graph_submit 4.17, stage/render/prepare_bind_groups 2.97 |
| warm-up | 4.93 | 37.4 | pipelines/wait_ms 5044, pipelines/compile_ms 219, pipelines/front_ms 195, render/thread/ms 39.22, render/busy/ms 37.45, main/busy/ms 25.32, pipelines/driver_ms 23.21, stage/render/prepare_assets 22.93, stage/main/update 10.88, stage/main/post_update 10.06, stage/render/render 7.31, render/graph/ms 4.88 |
| warm-up | 4.96 | 27.6 | pipelines/wait_ms 7873, pipelines/compile_ms 232, pipelines/front_ms 208, render/thread/ms 37.38, render/busy/ms 36.24, stage/render/prepare_assets 27.39, pipelines/driver_ms 23.86, main/busy/ms 21.94, stage/main/update 10.27, stage/main/post_update 7.61, stage/render/render 4.54, render/graph/ms 3.11 |
| warm-up | 5.38 | 33.1 | pipelines/wait_ms 7181, main/busy/ms 55.06, pipelines/compile_ms 48.32, pipelines/front_ms 44.19, stage/main/update 42.70, render/thread/ms 38.32, render/busy/ms 36.59, stage/render/prepare_assets 10.99, stage/render/render 10.86, encode//sum 8.31, render/graph/ms 8.14, stage/main/post_update 6.97 |
| warm-up | 5.43 | 46.7 | pipelines/wait_ms 5415, main/busy/ms 71.09, stage/main/update 58.47, render/thread/ms 50.07, render/busy/ms 48.02, pipelines/compile_ms 18.06, stage/render/prepare_assets 15.37, pipelines/front_ms 15.10, stage/render/render 11.10, render/graph/ms 8.27, encode//sum 7.87, stage/main/post_update 7.27 |
| warm-up | 5.49 | 60.3 | pipelines/wait_ms 7525, pipelines/compile_ms 136, pipelines/front_ms 130, main/busy/ms 81.87, stage/main/update 69.29, render/thread/ms 63.03, render/busy/ms 61.14, stage/render/prepare_assets 30.33, stage/render/render 9.86, stage/main/post_update 7.69, render/graph/ms 7.35, stage/render/prepare_meshes 7.26 |
| warm-up | 5.53 | 41.9 | pipelines/wait_ms 7525, pipelines/compile_ms 136, pipelines/front_ms 130, render/thread/ms 57.88, main/busy/ms 57.73, render/busy/ms 56.55, stage/main/update 45.29, stage/render/prepare_assets 34.31, stage/main/post_update 7.11, stage/render/render 6.88, pipelines/driver_ms 6.83, stage/render/prepare_meshes 6.21 |
| warm-up | 5.68 | 30.7 | render/thread/ms 39.13, render/busy/ms 37.29, main/busy/ms 32.80, stage/main/update 15.46, stage/main/post_update 12.63, stage/render/render 11.61, stage/render/prepare_assets 9.74, render/graph/ms 9.16, encode//sum 8.94, stage/render/graph_submit 6.30, stream/drive/ms 6.15, encode/submit 4.65 |
| warm-up | 5.70 | 27.8 | render/thread/ms 41.09, render/busy/ms 39.78, main/busy/ms 27.11, stage/render/prepare_assets 17.43, stage/main/update 11.20, stage/main/post_update 9.79, encode//sum 7.42, stage/render/render 6.64, stage/render/prepare_meshes 6.01, stream/drive/ms 4.92, render/graph/ms 4.84, stage/render/graph_submit 2.98 |
| warm-up | 5.96 | 32.8 | main/busy/ms 49.59, render/thread/ms 42.11, render/busy/ms 40.66, stage/main/update 36.20, stage/render/prepare_assets 15.74, encode//sum 12.78, stage/render/render 10.93, render/graph/ms 8.86, stage/main/post_update 8.64, stage/render/graph_submit 6.07, encode/submit 3.31, stage/render/prepare_write_phase_buffers 3.27 |
| warm-up | 6.00 | 34.4 | pipelines/wait_ms 5353, pipelines/compile_ms 103, pipelines/front_ms 97.30, render/thread/ms 42.40, render/busy/ms 40.93, main/busy/ms 36.64, stage/main/update 23.11, stage/render/prepare_assets 19.07, encode//sum 10.54, stage/render/render 9.73, stage/main/post_update 8.03, render/graph/ms 7.66 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on15\balmora@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 12%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 89%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.3 W · 75 °C (max 77) · VRAM 5777 MiB · throttled 100% of samples
